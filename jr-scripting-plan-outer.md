# Plan: JR Bulk-Edit Scripting — Repo-Level Changes

## Context

JR needs a shield-gated way to do bulk transformations across many definitions (the kind of work that currently routes through TL ordination via sed, which is wrong on multiple axes — TL skipping shields and TL doing JR's mechanical work). The full design is:

1. JR writes a Python script in `./tmp/scripts/edit.py` that reads a file via stdin, transforms it, writes to stdout.
2. For each target file (one at a time, no loops): JR invokes `python3 ./tmp/scripts/edit.py < $file > ./tmp/working/$relpath`, then reviews `diff -u $file ./tmp/working/$relpath` end-to-end.
3. JR posts to a new Guardian endpoint `POST /apply-file` which runs the normal shield pipeline against the diff, applies the changes for defs whose shields passed (or rejects the whole file in `all_or_nothing` mode), backs up the original to `./tmp/backup/{ts}/`, and returns per-def shield verdicts.

The Guardian endpoint itself — request/response types, handler logic, merge construction, backup/recovery, TDD slices — is fully specified in **`./Guardian/jr-scripting-plan.md`**. That is the implementation plan for the endpoint and its tests; whoever picks up this work should read that plan in full before starting.

This **outer** plan covers the repo-level (non-Guardian) changes needed to make the whole system work end-to-end:

1. **Two companion Luz shields** that enforce script structure and invocation shape, so the working-file-isolation property holds.
2. **VRBX whitelist additions** so JR's commands run without permission prompts.
3. **`/Volumes/V/Vale/CLAUDE.md` documentation update** so future Claude sessions know the protocol.

These three changes are independent of the Guardian endpoint and can be developed in parallel (or after the endpoint lands). The endpoint, however, depends on the shields existing if you want the full safety property — without them, JR could (a) write a Python script that opens source files directly, bypassing the whole working-file model, or (b) `python3 script.py > /some/source/path` and overwrite a file without any shield check.

---

## Repo-Level Change 1: Script-Content Shield (Luz)

A new Luz shield that fires on writes to `./tmp/scripts/*.py` and rejects scripts that do filesystem I/O, subprocess calls, or any other side effect outside stdin/stdout.

**Shield mode:** LLM-primary (SimpleSmart tier) with a Rust trainee. The trainee can fast-reject the obvious static patterns; the LLM catches the subtler cases.

**File:** `Luz/shields/BulkEditScriptIsStdinStdoutOnly-BESISSO.md` (or similar; pick the suffix per existing Luz conventions).

**What the shield should reject** (LLM prompt content):

The shield receives a contextified diff of a file in `./tmp/scripts/`. Reject the change if the resulting script:

- Calls `open()` for any mode (read, write, append, exclusive). The script's input is stdin; its output is stdout.
- Imports or uses any of: `os.rename`, `os.replace`, `os.remove`, `os.unlink`, `os.symlink`, `os.link`, `os.mkdir`, `os.makedirs`, `os.rmdir`.
- Imports or uses any of: `shutil.move`, `shutil.copy`, `shutil.copy2`, `shutil.copyfile`, `shutil.rmtree`, `shutil.copytree`.
- Uses `pathlib.Path.write_text`, `.write_bytes`, `.read_text`, `.read_bytes`, `.unlink`, `.rename`, `.replace`, `.mkdir`, `.rmdir`, `.touch`, `.symlink_to`, `.hardlink_to`.
- Imports `subprocess` for any purpose. Side door for invoking sed/cp/mv/python and bypassing the sandbox.
- Reassigns `sys.stdin` or `sys.stdout` (or `sys.stderr` if it's used to silently swallow errors).
- Uses bare `except:` or `except Exception:` without an immediate `raise`. Silent error swallow is forbidden.
- Uses `try: ... except ...: pass` anywhere.
- Accesses a regex match result (`m.group()`, `m.start()`, etc.) without first checking the match is not None (either via `if m:` guard, `is None` check, or an `if not m: raise ...` pattern).
- Calls `assert` for runtime validation that user-input could trip (asserts are stripped under `-O`).
- Doesn't exit non-zero on errors with `file:line` context in the error message.
- Uses `# noqa`, `# type: ignore`, or other lint suppressions without a justification comment.
- Defines functions with mutable default arguments (e.g. `def foo(x=[]):`).
- Uses globals to carry state across input lines (the script must be stateless except for accumulating stdin → stdout — no module-level mutable state that affects output).

The shield's allow path: pure stdin → transform → stdout, with explicit `if match is None: raise ValueError(...)` style error handling. Example of an ALLOWED minimal script the shield should pass cleanly:

```python
import sys
import re

content = sys.stdin.read()
PATTERN = re.compile(r'(\w+)\.unwrap\(\)')
def replace(m):
    return f"{m.group(1)}.unwrap_or_else(|e| panic!(\"{m.group(1)}: {{e}}\"))"
result = PATTERN.sub(replace, content)
sys.stdout.write(result)
```

**Shield context:** `diff` (whole-file) — the script is a single file, no def-level granularity matters.

**Trainee (Rust mode):** A static check that fast-rejects scripts containing literal substrings `open(`, `subprocess`, `shutil`, `os.rename`, `os.replace`, `os.symlink`, `os.unlink`, `os.remove`, `os.mkdir`, `pathlib.Path` followed by `.write_`/`.read_`/`.unlink`/`.rename`/`.replace`/`.mkdir`/`.rmdir`. These can't appear in a legitimate stdin→stdout script. The trainee catches the obvious cases without paying for an LLM call.

**Where to author:** Use the `guardian-add` skill to bootstrap the shield file, then `guardian-rustify` once the LLM shield is calibrated to add the trainee.

**Test cases:** Write tests under `Luz/shields/BulkEditScriptIsStdinStdoutOnly-BESISSO/tests/` covering at least: a clean stdin→stdout script (allow), a script with `open()` (deny), a script with `subprocess` (deny), a script with bare `except:` (deny), a script with `m.group()` after no None-check (deny), a script with a silent `try/except: pass` (deny), and a script that uses `pathlib.Path.write_text` (deny).

---

## Repo-Level Change 2: Invocation Shield (Luz)

A new Luz shield, this one in **command-context** mode, that fires on bash invocations of `python3` and rejects any invocation that doesn't redirect stdout to a path under `./tmp/working/`.

**Shield file:** `Luz/shields/BulkEditScriptOutputMustGoToWorking-BESOMGW.md` (or similar).

**Shield mode:** Rust-primary (deterministic command matcher).

**Behavior:** When the Bash tool is about to run a command that invokes a script under `./tmp/scripts/*.py`, require that the command match the shape:

```
python3 ./tmp/scripts/<NAME>.py < <PATH> > ./tmp/working/<PATH>
```

Reject if:
- The output redirect target is NOT under `./tmp/working/`.
- There's no output redirect at all (e.g. `python3 ./tmp/scripts/edit.py < foo.rs` — the transformed output would go to stdout, possibly captured into something else).
- The command is wrapped in a `for`/`while`/`xargs`/`find -exec`/`parallel` invocation (batching forbidden per CLAUDE.md).
- The command pipes `python3 ./tmp/scripts/edit.py` output into another command (e.g. `python3 ... | tee somewhere` — defeats the working-file isolation).

Allow path: exactly the canonical single-file invocation shape. Multiple invocations across files = multiple separate bash commands, each individually shielded.

**Implementation note:** This is a Rust-mode command shield. Look at `Luz/shields/ValidateReadonlyBash-VRBX.md` for the pattern of command-matching shields and the deterministic-companion-program structure. The companion program parses the bash command and returns allow/deny based on the redirect target.

**Test cases:** Under `Luz/shields/BulkEditScriptOutputMustGoToWorking-BESOMGW/tests/`: canonical invocation (allow), missing redirect (deny), redirect to source path (deny), loop wrapper (deny), pipe chain (deny).

---

## Repo-Level Change 3: VRBX Whitelist Additions

Add the new auto-allow shapes to `/Volumes/V/Vale/.claude/settings.json` (or wherever VRBX's allowlist lives in this repo — check `Luz/shields/ValidateReadonlyBash-VRBX.md` for the format). These let JR's standard bulk-edit commands run without permission prompts.

The shapes to whitelist (exact regex patterns depend on VRBX's matcher syntax — adapt these as needed):

- `^mkdir -p ./tmp/working/.*$`
- `^mkdir -p ./tmp/scripts/.*$` and `^mkdir -p ./tmp/scripts$`
- `^diff -u [^ ]+ ./tmp/working/[^ ]+$`
- `^rm ./tmp/working/\*$` and `^rm -rf ./tmp/working/.*$` (cleanup between batches)
- `^curl -s -X POST http://localhost:[0-9]+/apply-file -H 'Content-Type: application/json' -d '\{.*\}'$`

Do **NOT** whitelist `python3 ./tmp/scripts/*.py < ... > ./tmp/working/...` here — the **invocation shield from Change 2 above** is what auto-allows that, with its own deterministic check. Whitelisting it twice (once in VRBX, once in the new shield) would create overlap; the shield is the authoritative gate because it carries the redirect-target check.

`./tmp/backup/...` paths don't appear in JR commands (Guardian handles backups server-side from inside the `/apply-file` handler), so no whitelist needed for them.

**Verification after landing:** JR's standard bulk-edit batch should run end-to-end with no permission prompts. Validate by issuing a small test batch (one file) and confirming each command in the sequence auto-allows.

---

## Repo-Level Change 4: CLAUDE.md "Bulk Edits" Section

Replace the existing **"Bulk Sed Safety Protocol"** section in `/Volumes/V/Vale/CLAUDE.md` with a new **"Bulk Edits"** section. Wording (settled with the architect):

> ## Bulk Edits
>
> `sed` and `perl -pi` are outlawed for editing this repo. They produce silent collateral damage, can't be reviewed pre-application, and have no error handling.
>
> For bulk transforms, **prefer the Edit tool** — repetitive Edit calls are explicit, per-site reviewable, and don't need any script machinery. The Edit-tool threshold is ~40 *distinct invocations*, not total sites touched: 7 Edit calls × 6 sites each = 42 sites and stays in Edit territory; 50 one-off Edits crosses into Python.
>
> When Edit truly isn't enough, the alternative is a Python script in `./tmp/scripts/`, invoked stdin → stdout with output redirected to `./tmp/working/`. The script's structure (no file I/O, no subprocess, full error handling) and the invocation shape are enforced by Guardian shields and VRBX rules — follow the verdicts; they explain the constraints reactively.
>
> The apply step is `POST /apply-file` to the local Guardian server (not bare `cp && mv`). It runs the normal shield pipeline against the diff per-def, applies whatever passes, rejects the rest, and backs up the original to `./tmp/backup/{ts}/`. See Guardian's `/apply-file` documentation for the request/response shape.
>
> **No loops or batching, ever, without explicit "fire batch" approval from the architect.** Even when applying the script to 100 files, run the full per-file sequence — script invocation, `diff -u` review, `POST /apply-file` — serially, one command set per file. Bash `for`/`while`/`xargs` over the file list is forbidden by Guardian's invocation shield. The serial discipline is the load-bearing safety: it catches script bugs on file 1 before they propagate to files 2–N, and keeps every change individually visible. If a large batch genuinely warrants loop-form invocation, surface it to the architect and wait for explicit "fire batch" go-ahead — never batch unilaterally.

Two points the implementor must double-check as they land this:
- The endpoint is **JR's** tool, not TL's. TL doesn't do bulk edits via script — TL's job is rulings and review.
- The apply step is `POST /apply-file`, not bare filesystem `cp && mv`.

---

## Ordering and Dependencies

The four repo-level changes are mostly independent. Recommended order:

1. **Land the Guardian `/apply-file` endpoint first** (per `./Guardian/jr-scripting-plan.md`). The endpoint is the load-bearing piece; without it, the rest is unusable.
2. **Then the two companion shields** (Changes 1 and 2). These can be developed in parallel — both use the `guardian-add` skill to bootstrap.
3. **Then the VRBX whitelist additions** (Change 3). Trivial; one settings.json edit.
4. **Finally the CLAUDE.md update** (Change 4). Last because the doc should describe the system as it exists, not the system as it will exist.

Alternative ordering: land 3 and 4 in parallel with the Guardian work — neither depends on the endpoint existing.

---

## Verification

After all four changes plus the Guardian endpoint are in place, do an end-to-end smoke test:

1. **Author a trivial test transform.** Create `./tmp/scripts/test-edit.py` that does a no-op text replacement (e.g. replace `foo` with `bar` in stdin).
2. **Confirm the script-shield blocks an unsafe script.** Try authoring a version with `import os; os.unlink('/tmp/x')` — confirm the shield denies.
3. **Confirm the invocation shield blocks a bad invocation.** Try `python3 ./tmp/scripts/test-edit.py < foo.rs > foo.rs` (overwriting source directly) — confirm the shield denies. Try the canonical `... > ./tmp/working/foo.rs` — confirm it allows.
4. **Run the canonical sequence** on one test file:
   ```
   mkdir -p ./tmp/working/path/to/somewhere
   python3 ./tmp/scripts/test-edit.py < path/to/somewhere/foo.rs > ./tmp/working/path/to/somewhere/foo.rs
   diff -u path/to/somewhere/foo.rs ./tmp/working/path/to/somewhere/foo.rs
   curl -s -X POST http://localhost:7879/apply-file \
     -H 'Content-Type: application/json' \
     -d '{"session_id":"smoke","path":"'$PWD/path/to/somewhere/foo.rs'","mode":"partial"}'
   ```
   Confirm `foo.rs` was updated, backup exists under `./tmp/backup/{ts}/`, response JSON parses cleanly.
5. **VRBX prompt check:** the four commands above should run with zero permission prompts.
6. **Read the new CLAUDE.md "Bulk Edits" section** — confirm wording is accurate to the actual system behavior.

---

## Summary

Four small repo-level changes that complete the JR bulk-edit scripting system:
- One LLM-primary shield (script content).
- One Rust-mode shield (invocation shape).
- A few VRBX allowlist regex shapes.
- One CLAUDE.md section rewrite.

The Guardian endpoint that does the actual apply-with-shielding is the heavy lift; see `./Guardian/jr-scripting-plan.md`. Together, the outer changes plus the endpoint give JR scripted bulk-edit capability while preserving every shield check that would normally fire on an Edit-tool call.
