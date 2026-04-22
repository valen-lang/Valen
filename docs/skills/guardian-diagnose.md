---
name: guardian-diagnose
description: "Diagnose and resolve Guardian shield failures or unwanted prompts from hook output. Reads logs, classifies issues (violations, false positives, pipeline bugs, missing auto-allows), creates test cases, fixes shields/companion programs, and validates — all in one session."
argument-hint: "[paste Guardian hook stdout, or provide log dir path]"
allowed-tools: Bash(guardian post-hook-allow *), Bash(guardian post-hook-deny *), Bash(guardian check-direct *), Bash(guardian check *), Bash(cargo build *), Bash(cargo nextest run *), Bash(ls *), Read, Grep, Glob, Edit, Write
read-when: Read when a Guardian shield just fired or failed at hook time and you need to diagnose it.
mention-in:
  - CLAUDE.md
---

# Diagnose and Resolve Guardian Failures

When Guardian hook output shows shield failures, systematically investigate each failure, classify it, and resolve it — creating test cases, fixing shield prompts, and fixing Rust companion programs as needed.

## Input

The user will provide one of:
- Guardian hook stdout (the `[hook]` log lines) — may be a denial OR an unwanted user prompt
- A log directory path (e.g. `FrontendRust/guardian-logs/request-XXXX`)

If neither is provided, look in the project's `guardian-logs/` directory for the most recent `request-*` subdirectory. If it doesn't look like it matches what the user is asking about (different file, different shields), ask the user to confirm.

Note: not all diagnose requests involve shield denials. If the hook output shows `? Bash asking user` with no FAILED shields, the issue is likely a **missing auto-allow** (Category D) — the shield didn't fire at all, but the user wants the command auto-approved. This is a feature request for the shield's rules, not a failure.

---

## Phase 1: Diagnose

### Step 1: Parse the Hook Output

Extract from the `[hook]` lines:
- **Which shields fired** (denied) vs which were skipped and why
- **The log directory path** (appears in `[log dir]` line or in `(see ...)` references)
- **The definition name and line** (e.g. `new--204` means fn `new` at line 204)
- **The violation summaries** (the text after each shield name in the FAILED line)

Note the skip reasons — they tell you about the pipeline's decision-making:
- `DefMismatch` — shield's `defs:` field doesn't match the definition kind
- `WhenMentionedNotMatched` — shield's `when_mentioned:` regex didn't match the diff
- `DisabledByDirective` — a `// guardian-disable` comment suppressed it

### Step 2: Read the Artifacts

For each denied shield, read these files from the log directory:

1. **`<def>.contextified_diff.txt`** — The exact diff + context the LLM saw. Check:
   - Does it contain only the changed lines, or a large surrounding context?
   - Are unchanged lines shown that the LLM might mistakenly audit?

2. **`<def>.<ShieldName>.<ShieldName>.verdict.md`** — The structured verdict with violation list.

3. **`<def>.<ShieldName>.<ShieldName>.log`** — The full LLM interaction log (if it exists).

4. **`log.<def>.<ShieldName>.log`** — The pipeline log. Shows whether program or LLM path was taken, and whether exception matching ran.

5. **`<def>.data_substituted.txt`** — The full prompt after template substitution.

6. **`hook.request.json`** — The raw hook input (tool name, file path, content).

### Step 3: Read the Shield Definition

Read the actual shield markdown file. Pay attention to:

- **Frontmatter fields:**
  - `primary: rust` — Rust companion is authoritative, LLM doesn't run
  - `primary: llm` (or absent) — LLM is authoritative
  - `program:` — path to a Rust companion binary
  - `model:` — which LLM tier (SimpleSmall, SimpleLarge, etc.)
  - `defs:` — which definition kinds this shield applies to
  - `when_mentioned:` — regex that must match the diff

- **Exceptions section** (`## Exceptions`) — After LLM denies, a second LLM call checks exception categories. BUT: this only happens in the LLM path, not the Rust program path (known gap).

- **Examples section** — ALLOW and DENY examples that guide the LLM judge.

### Step 4: Classify Each Failure

**Category A: True Violation** — Shield correctly identified a real problem.
- Violation describes something in the changed lines (not context)
- No applicable exception exists

**Category B: LLM Misjudgment (False Positive)** — LLM made a mistake.
- Violation describes unchanged context, not the diff
- LLM ignored an explicit ALLOW example or Exception
- Common modes: context bleed, exception blindness, hallucinated discrepancy

**Category C: Pipeline Bug** — Guardian system bug.
- Rust-mode shield denied, but shield has Exception that should apply (exception gap)
- Contextified diff over-scoped (entire parent block as context)
- Pipeline log shows exception matching skipped

**Category D: Missing Auto-Allow (Feature Request)** — Shield worked correctly but the user wants it to handle a new pattern.
- The hook asked the user to confirm (no denial, no violation — just no auto-allow)
- The user wants the shield's rules expanded so this case is auto-approved
- Common with `g_context: command` shields like VRBX where new command patterns emerge
- Symptoms: `[hook] ? Bash asking user` in the log, empty verdict logs, no shield fired

---

## Phase 2: Triage with Human

Present each classification to the human. Human confirms or overrides:
- **True violation** → skip (human fixes code)
- **LLM false positive** → proceed with post-hook-allow
- **Pipeline bug** → report bug location, still create test case if shield prompt can be improved
- **Missing denial** (human spotted something the hook missed) → proceed with post-hook-deny
- **Missing auto-allow** → proceed to Phase 5 (update shield rules and companion program)

---

## Phase 3: Create Cases

For each false positive:
```bash
guardian post-hook-allow --log-dir <def-level-dir> --shield <CODE>
```

For each missing denial:
```bash
guardian post-hook-deny --log-dir <hook-dir> --shield <CODE> [--def <name>]
```

These create:
- `post-hook-allow` → `NNN.diff` + `NNN.expected.json` (empty violations) in `cases/need-shield-amendment/`
- `post-hook-deny` → `NNN.diff` + `NNN.expected.json` (with violations) in `tests/`

---

## Phase 4: Fix Shields (Inline Curate)

For shields with new `cases/need-shield-amendment/` cases (false positives):
1. Read the shield markdown and all human cases
2. Propose prompt changes (clarifications, examples, exceptions)
3. Get human approval, edit the shield
4. Re-run shield against the cases:
   ```bash
   guardian check-direct --input <NNN.diff> --referenced-defs <NNN.referenced_defs.txt> \
     --file-path <file> --check <shield> --cache-dir /tmp/cache --backend opencode \
     --log-dir /tmp/logs --format human --log-level overview
   ```
5. Iterate until cases pass

For shields with new `tests/` cases (false negatives):
1. Run shield against the new case to confirm it currently fails (doesn't catch it)
2. Propose prompt changes to catch the violation
3. Get human approval, edit the shield
4. Re-run to verify

---

## Phase 5: Fix Rust Companion Programs

For shields with `primary: rust`:
1. Run the Rust program against all `tests/cases/` test cases
2. If failures: propose a fix to the Rust program
3. Add a **unit test in the program's `main.rs`** targeting the specific logic bug (e.g., wrong regex, missed pattern) — not a full-diff integration test
4. Run `cargo test` to verify
5. Ask the human whether to also promote to `tests/cases/` as an integration test
6. **Update the shield markdown** to reflect any new rules the program now enforces (see below)

For Category D (missing auto-allow on `primary: rust` shields):
1. Discuss the desired behavior with the human — what should be auto-allowed and what shouldn't
2. Write failing unit tests in `main.rs` first (TDD)
3. Implement the logic change in the companion program
4. Run `cargo test` to verify all tests pass (old and new)
5. **Update the shield markdown** to document the new behavior (see below)

### Shield Markdown ↔ Companion Program Sync

The shield `.md` file is the **requirements document** for its companion program. When a companion program's behavior changes, the shield markdown must be updated to describe the new rules. A program that enforces rules not documented in the markdown (or vice versa) is a drift bug.

The markdown serves dual duty: it is both the specification for the Rust program AND the actual LLM prompt used for crash fallback and doublecheck appeals. So it must remain phrased as shield instructions to an LLM judge — not as developer documentation for the Rust code. Write new rules the way you'd write any shield rule: describe what to ALLOW and DENY, give examples, explain the reasoning. An LLM reading only the markdown should reach the same conclusions as the Rust program.

When updating a companion program:
- Add or update the relevant rule description in the shield markdown body, phrased as LLM instructions
- If the change adds new ALLOW/DENY patterns, add corresponding examples
- The markdown should be readable both as a spec someone could re-implement from AND as a prompt an LLM could enforce from

---

## Phase 6: Validate & Promote

1. Run all test cases and `cargo nextest run` for each affected shield
2. Promote resolved cases to `tests/` (ask human)
3. Report summary: which shields were fixed, what changed
4. Tell the user that Guardian shields are now fixed

---

## Reference: Log Directory Structure

```
guardian-logs/request-XXXX/
  hook.request.json                       # raw hook input (tool name, file path, content)
  hook/
    file-scope.contextified_diff.txt      # file-level contextified diff
    file-scope.diff.patch                 # raw git diff
    file-scope.modified.txt               # full modified file
    file-scope.original.txt               # full original file
    file-scope.referenced_defs.txt        # referenced definitions context
    file-scope.<Shield>.<Shield>.verdict.md  # file-scope shield verdicts

    <def>.contextified_diff.txt           # per-definition contextified diff
    <def>.data_substituted.txt            # full prompt after substitution
    <def>.diff.patch                      # per-definition raw diff
    <def>.modified.txt                    # modified definition text
    <def>.original.txt                    # original definition text
    <def>.referenced_defs.txt             # referenced defs for this definition
    <def>.<Shield>.<Shield>.verdict.md    # per-definition shield verdicts
    <def>.<Shield>.<Shield>.log           # LLM interaction log (if exists)

    log.file-scope.<Shield>.log           # pipeline log for file-scope check
    log.<def>.<Shield>.log                # pipeline log for per-def check
    log.<def>.log                         # pipeline log for definition processing
```

Where `<def>` is `<name>--<line>.<index>` (e.g. `new--204.0`).
