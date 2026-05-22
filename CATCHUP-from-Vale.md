# Catchup Guide: Syncing `/Volumes/V/Vale` to the `rustmigrate-z` Branch State

This guide brings a working copy at `/Volumes/V/Vale` (or any other directory name) up to the state currently checked into the `rustmigrate-z` branch of `https://github.com/Verdagon/Vale`. Run these steps in order; they assume macOS / a shell with `git` ≥ 2.30.

The repo uses **nested submodules** (Guardian carries its own submodules: Luz, ContextifiedDiff, ContextifiedShield, Rabble, ShieldFile, opencode). All steps below recurse through them.

---

## Step 0: Save your local work before touching anything

A `git status` from your `/Volumes/V/Vale` working copy may show uncommitted edits — including inside the submodules. Decide what to do with each:

```bash
cd /Volumes/V/Vale
git status --short
git submodule foreach --recursive 'echo "=== $name ==="; git status --short'
```

For anything you want to keep:

- **Parent repo changes** — commit to your branch, or `git stash push -m "pre-catchup"`.
- **Submodule changes** — `cd` into the submodule and either commit/push to its remote or `git stash push -m "pre-catchup"` there. (`git stash` at the parent does NOT save submodule working trees.)

If your local commits live on a different branch than `rustmigrate-z`, push that branch up first so it's recoverable:

```bash
git push -u origin <your-branch-name>
```

Anything you don't save before the next steps is at risk.

---

## Step 1: Confirm the remote URL

The parent repo's `origin` must point at `https://github.com/Verdagon/Vale`:

```bash
git -C /Volumes/V/Vale remote -v
```

If `origin` is wrong, fix it:

```bash
git -C /Volumes/V/Vale remote set-url origin https://github.com/Verdagon/Vale
```

---

## Step 2: Fetch and check out `rustmigrate-z`

```bash
cd /Volumes/V/Vale
git fetch origin --prune
git checkout rustmigrate-z
git pull --ff-only origin rustmigrate-z
```

If `git checkout rustmigrate-z` fails with "pathspec did not match," your remote is stale — re-run `git fetch origin` and retry. If `pull --ff-only` fails because your local `rustmigrate-z` has diverged from origin, you have local commits that need to be saved (Step 0) or rebased; do not force-pull.

Expected HEAD after this step: `e03cac7c` (or whatever's current on origin/rustmigrate-z; check at https://github.com/Verdagon/Vale/commits/rustmigrate-z).

---

## Step 3: Initialize and update all submodules recursively

This is the critical step — most catchup failures come from skipping `--recursive` or `--init`.

```bash
git submodule update --init --recursive --progress
```

This will:
- Clone `Guardian` and `Luz` at the top level.
- Inside `Guardian`, clone `Luz`, `ContextifiedDiff`, `ContextifiedShield`, `Rabble`, `ShieldFile`, `opencode`.
- Inside each of those that has its own submodules (e.g. `ContextifiedDiff/Luz`, `ContextifiedShield/Luz`, `Rabble/Luz`, `ShieldFile/Luz`), clone those too.

For first-time setup you'll see ~11 sub-checkouts happen. It can take a few minutes on a fresh clone.

---

## Step 4: Verify submodule branches

Most submodules sit on `main`, but **`Guardian/opencode` is on a non-`main` branch** — `verdagon-lsp-context-defs`. The recursive update should put it there automatically (the branch is recorded in `Guardian/.gitmodules`), but verify:

```bash
git submodule foreach --recursive 'echo "$name: $(git rev-parse --abbrev-ref HEAD)"'
```

Expected output:

```
Guardian: main
ContextifiedDiff: main
Luz: main
ContextifiedShield: main
Luz: main
Luz: main
Rabble: main
Luz: main
ShieldFile: main
Luz: main
opencode: verdagon-lsp-context-defs
Luz: main
```

Note the multiple `Luz:` lines — that's because Guardian, ContextifiedDiff, ContextifiedShield, Rabble, ShieldFile, and the top-level repo each include `Luz` as a submodule independently. They may sit on different SHAs of the Luz `main` branch; that's by design.

If any submodule shows a different branch (e.g. `(HEAD detached at <sha>)`), force it to its tracked branch:

```bash
git -C <submodule-path> checkout <expected-branch>
```

---

## Step 5: Verify the `.cargo/config.toml` is in place

The repo ships a `.cargo/config.toml` that sets `RUST_MIN_STACK=16777216` (16 MB stack for `#[test]` threads — the default 2 MB is too small for debug-mode runs on real Vale programs). After the pull this file should exist:

```bash
cat /Volumes/V/Vale/.cargo/config.toml
```

If it's empty or missing, something went wrong with the pull — re-run Step 2.

---

## Step 6: Build and test

```bash
cd /Volumes/V/Vale
cargo build --manifest-path FrontendRust/Cargo.toml --lib > tmp/catchup-build.txt 2>&1
tail -20 tmp/catchup-build.txt
```

Expected: clean build, two pre-existing warnings in `expression_compiler.rs` (`unreachable expression` + `unreachable pattern`). No errors.

Then run the test suite:

```bash
cargo nextest run --manifest-path FrontendRust/Cargo.toml --lib > tmp/catchup-tests.txt 2>&1
grep "Summary" tmp/catchup-tests.txt
```

Expected: `Summary [..s] 760 tests run: 760 passed, 0 skipped`.

If a test fails, save the full output (`tmp/catchup-tests.txt`) and check the failure against the current state of `TL.md` ("Where We Are" section) — the suite is supposed to be fully green on `rustmigrate-z`.

---

## Step 7: Sanity-check the workflow files

These are the files the migration workflow lives in. Read them before touching code:

- **`TL.md`** — TL handoff / migration state. Read top-to-bottom. Re-read after every compaction.
- **`for-tl.md`** — JR escalation queue. Check at the start of every turn.
- **`for-jr.md`** — TL's responses to JR. JR picks up from here between turns.
- **`CLAUDE.md`** — project-level Claude Code rules (no `cd && cargo`, no temp programs, etc.).
- **`docs/architecture/typing-pass-design-v3.md`** — typing-pass migration design doc.

The `/Volumes/V/Sylvan` checkout uses these files canonically; `/Volumes/V/Vale` should treat the same files as canonical too — don't fork them.

---

## Step 8: Optional — rename the working directory

The repo expects no specific working-directory name (`/Volumes/V/Vale`, `/Volumes/V/Sylvan`, and other names all work). If you want to match the convention I've been using on my side:

```bash
mv /Volumes/V/Vale /Volumes/V/Sylvan
```

…but **this is purely cosmetic**. The Cargo manifests use relative paths; no script depends on the parent directory's name. Skip this if it's disruptive.

---

## Troubleshooting

**"fatal: remote error: upload-pack: not our ref"** during `submodule update`:
A submodule pointer in the parent references a SHA that hasn't been pushed to that submodule's origin. Run `git submodule sync --recursive` then retry, or `cd` into the specific submodule and `git fetch origin` manually.

**`git checkout rustmigrate-z` says "would overwrite local changes"**:
Step 0 caught something. Stash or commit before retrying.

**`cargo nextest` fails on `typing_pass_on_roguelike` with a stack-overflow**:
The `.cargo/config.toml` from Step 5 isn't being picked up. Cargo only reads `.cargo/config.toml` from the directory it's invoked from (and parents). If you're running from a deeper subdirectory like `FrontendRust/`, that's fine — Cargo walks up. But if Cargo is being invoked through a tool that overrides `CARGO_HOME` or similar, the config may be ignored. Confirm with `cargo run --manifest-path FrontendRust/Cargo.toml --bin <something> -- --print-env | grep STACK` or set `RUST_MIN_STACK=16777216` manually in the shell.

**Guardian shields fire on every edit**:
Guardian is the active code-review system for this migration. If a shield is wrong, the fix is to amend the shield (`Luz/shields/<name>.md`) or to escalate the case (`Guardian/<shield>/cases/need-…/`), not to disable Guardian globally. Read `Luz/skills/guardian-*.md` for the workflow.

**The submodules are *huge***:
Yes. Guardian alone carries 5 nested submodules. A fresh checkout is ~600 MB. If you don't need Guardian (e.g. read-only browsing), you can skip `--recurse-submodules` and the parent repo alone is small. But for code review or running tests with Guardian active, you need them all.

---

## Quick-reference: one-line catchup

If your `/Volumes/V/Vale` checkout has nothing local to save, the entire catchup collapses to:

```bash
cd /Volumes/V/Vale && \
  git fetch origin --prune && \
  git checkout rustmigrate-z && \
  git pull --ff-only origin rustmigrate-z && \
  git submodule update --init --recursive --progress && \
  cargo nextest run --manifest-path FrontendRust/Cargo.toml --lib
```

If that prints `Summary [..s] 760 tests run: 760 passed, 0 skipped`, you're in sync.
