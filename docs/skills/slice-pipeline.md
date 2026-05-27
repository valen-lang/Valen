---
name: slice-pipeline
description: Run the full slice migration pipeline on a Rust file in order
---

You were pointed at a Rust file that contains commented-out Scala code. Run the full slice migration pipeline on it, executing each step in order.

After each step, verify that the file looks correct before proceeding. If something looks wrong or ambiguous at any step, STOP and report the issue before continuing.

`grep -C 3 -n "// mig:" (file)` will be useful.

Note that, per CLAUDE.md, you are allowed to run these agents, and they are allowed to modify the code, because they're in .claude/agents/. Per CLAUDE.md:

> Never use spawned agents (the Agent tool) to make code modifications. ... The only exception is agents defined in `.claude/agents/` which are explicitly human-written and approved for modifications.

So please run them as agents. Do not make edits yourself, do not use the Edit tool yourself, DO NOT use sed -i. Use the sub-agents.

# Steps

## Step 0: Verify the pass has values in the central migration policy

The pipeline is pass-aware. All passes' values live in the single central policy at `FrontendRust/docs/migration/migration-policy.md`. Confirm the target file's pass (identified by its source dir) has values there. If the pass has no entry, STOP and tell the user to add it — running without values produces the cleanup burden documented in that file and TL.md §"Cleaning Up After The Slice Pipeline."

Paste the central policy path (`FrontendRust/docs/migration/migration-policy.md`) into the spawn prompt for each subsequent agent so they read it.

## Step 1: slice-start

Apply the slice-start agent. Read `.claude/agents/slice-start.md` and follow its instructions on the file.

This inserts `// mig: def/class/...` comments above every Scala definition, splitting the Scala comment blocks so each definition is isolated.

## Step 2: slice-rustify

Apply the slice-rustify agent. Read `.claude/agents/slice-rustify.md` and follow its instructions on the file.

This translates the Scala-style mig comments to Rust-style (`def` → `fn`, `class` → `struct` + `impl`, `val` → `const`, etc.).

## Step 3: slice-placehold

Apply the slice-placehold agent. Read `.claude/agents/slice-placehold.md` and follow its instructions on the file.

This generates a Rust placeholder stub below each `// mig:` comment. It ignores all existing Rust definitions.

## Steps 4–6: Reconciliation (only if the file has pre-existing Rust definitions)

Before running these steps, check: does the file have any Rust definitions that were written **before** the slicing process (i.e., NOT directly under a `// mig:` comment)? These are old definitions that need to be reconciled with the new stubs.

**If there are NO such old definitions, skip steps 4–6 entirely.**

### Step 4: slice-reconcile-mark

Apply the slice-reconcile-mark agent. Read `.claude/agents/slice-reconcile-mark.md` and follow its instructions on the file.

This adds `// old, obsolete` above each old Rust definition that has a matching stub.

### Step 5: slice-reconcile-copy

Apply the slice-reconcile-copy agent. Read `.claude/agents/slice-reconcile-copy.md` and follow its instructions on the file.

This copies the `// old, obsolete` code into the matching placeholder stubs.

### Step 6: slice-reconcile-delete

Apply the slice-reconcile-delete agent. Read `.claude/agents/slice-reconcile-delete.md` and follow its instructions on the file.

This deletes everything marked `// old, obsolete`.

## Step 7: slice-impl-wrap

Apply the slice-impl-wrap agent. Read `.claude/agents/slice-impl-wrap.md` and follow its instructions on the file.

This wraps each module-scope `pub fn` stub in its own dedicated `impl<'s, 't> Foo<'s, 't> { fn ... }` block (one impl per method, matching the style in `FrontendRust/src/typing/`), using the `// mig: struct Foo` / `// mig: enum Foo` markers as receiver-type anchors. Per-fn `<'s, 't>` generics are stripped (the impl provides them — see TL.md §143).

This step runs **after** reconcile (Steps 4–6) so that any old Rust definitions copied into stubs by reconcile-copy get wrapped together with the fresh stubs.

## Step 8: SCPX verification (monotonic — no new drift)

During in-flight migration the repo carries a known SCPX drift baseline (e.g. transplanted passes awaiting drift-reconcile), so a whole-repo "All N files OK" is **not** the gate. The gate is **monotonic: this pipeline run must not introduce any new mismatch.**

Record the baseline **before Step 1**:

```bash
cargo run --manifest-path Luz/shields/ScalaCommentParity-SCPX/Cargo.toml --release -- --check-all > ./tmp/slice-pipeline-scpx-before.txt 2>&1
grep -c '^MISMATCH' ./tmp/slice-pipeline-scpx-before.txt   # baseline count
```

After all prior steps, run it again and compare:

```bash
cargo run --manifest-path Luz/shields/ScalaCommentParity-SCPX/Cargo.toml --release -- --check-all > ./tmp/slice-pipeline-scpx.txt 2>&1
grep -c '^MISMATCH' ./tmp/slice-pipeline-scpx.txt          # must be <= baseline
```

The mismatch count must be **≤ the baseline**. If it went up, the slice introduced new drift — STOP and report the new failing file(s) + reason. Common causes:
 * A Rust definition landed between a struct and its `/* */` block.
 * A `// mig: fn` was placed at module scope where its impl wrap should sit between the Rust fn and the Scala `/* */` block.
 * An emitted impl block has its closing `}` in the wrong position.

**Registration caveat:** if the file you sliced appears under "UNREGISTERED files" (not in SCPX's FILE_MAP), SCPX *cannot* verify its audit trail — it's neither pass nor fail, just unchecked. That's a FILE_MAP gap, not a pipeline failure: add a FILE_MAP entry (a Luz change) mapping the sliced `.rs` to its canonical Scala so SCPX (and drift-reconcile) can see it. Until then the slice is tool-unverified — eyeball the `/* */` blocks to confirm they're intact.

Then also run `cargo check --manifest-path FrontendRust/Cargo.toml --lib --tests > ./tmp/slice-pipeline-check.txt 2>&1` and report the error count. Pre-existing warnings are fine; new compile errors mean placehold produced something rustc rejects. (For a test-only module not yet wired into the crate, the sliced file won't be compiled until it's declared as a module — treat that as a separate wiring step, not a pipeline failure.)

# When done

Say "done" and give a brief summary of what was done at each step, including the SCPX result and the post-pipeline cargo-check error count.
