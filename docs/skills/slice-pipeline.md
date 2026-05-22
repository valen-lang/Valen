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

## Step -1: Verify the target files are slice-pipeline-ready

Before doing anything else, for each `.rs` file you're about to process:
 * It must be **registered** in its parent `mod.rs` (or its parent's `pub mod` chain reaches it). If you can't find `pub mod <file_stem>;` (or `#[cfg(test)] pub mod <file_stem>;`) somewhere on the path from `lib.rs` down, the file is unregistered.
 * It must already be in **wrapped-Scala** shape: first non-blank line should be `// From Frontend/…/<File>.scala` (or similar header comment), second non-blank line `/*`, and the file ends with `*/`. Any Rust definitions live between the header and the opening `/*`, or interleaved with later `*/`...`/*` pairs.

If either check fails, **STOP and escalate to TL** via `for-tl.md`. The wrap-and-register prep is TL/architect-only work — JR should not attempt it. Symptoms of an unprepped file: raw `package dev.vale.…`, `import dev.vale.…`, or `class …` lines appearing outside `/* */` blocks; these will not compile if registered, and slice-start will produce nonsense if run on them.

The fix is mechanical (TL wraps each file in `/* */` + adds `pub mod` entries) and takes ~5 minutes per directory; just don't try to do it yourself.

## Step 0: Verify the universal migration policy has a row for this pass

The pipeline is policy-driven by a single universal file: `FrontendRust/docs/migration/migration-policy.md`. Open it and locate the row in the **Per-pass values** table whose **Path prefix** matches the target file's path (e.g. `src/typing/` for typing, `src/simplifying/` for simplifying).

If no row matches, STOP and tell the user — running without one produces the cleanup burden documented in TL.md §"Cleaning Up After The Slice Pipeline." The architect needs to add a row before the pipeline can run.

If the matching row has any `(TBD — defer to architect when first file gets migrated)` cells in columns this pipeline would need, STOP and escalate to the architect for those values.

The subsequent slice agents (`slice-rustify`, `slice-placehold`) read the policy themselves at the same fixed path; no need to pass it into their spawn prompts.

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

This wraps each module-scope `pub fn` stub in its own dedicated `impl<'s, 't> Foo<'s, 't> { fn ... }` block (one impl per method, matching the style in `FrontendRust/src/typing/`), using the `// mig: impl Foo` markers as receiver-type anchors. Per-fn `<'s, 't>` generics are stripped (the impl provides them — see TL.md §143).

This step runs **after** reconcile (Steps 4–6) so that any old Rust definitions copied into stubs by reconcile-copy get wrapped together with the fresh stubs.

## Step 8: SCPX verification

After all prior steps, verify the Scala-comment audit trail is structurally intact. Run:

```bash
cargo run --manifest-path Luz/shields/ScalaCommentParity-SCPX/Cargo.toml --release -- --check-all > ./tmp/slice-pipeline-scpx.txt 2>&1
```

Then check the output: it must report `All N files OK` for some N. If any file fails SCPX, STOP and report the failing file + reason — the pipeline output is not safe to commit until SCPX is green. Common causes:
 * A Rust definition landed between a struct and its `/* */` block (the user's known issue #2).
 * A `// mig: fn` was placed at module scope where its impl wrap should sit between the Rust fn and the Scala `/* */` block.
 * An emitted impl block has its closing `}` in the wrong position.

**Cargo green is NOT a goal of this pipeline.** Placeholder stubs naturally reference types that don't exist yet (upstream-pass output AST, not-yet-migrated cross-pass types, etc.). `cargo check` is expected to be red after the pipeline runs, and that's fine — the next phase (Guardian-enabled real migration) is where stubs get filled in and references resolve as test paths drive the work. Do NOT chase cargo errors during the pipeline; do NOT add `use` statements / scaffold upstream types / add lifetime decoration to make things compile. The pipeline's "done" condition is **SCPX green**, full stop.

# When done

Say "done" and give a brief summary of what was done at each step, including the SCPX result. Don't report cargo state — it isn't load-bearing.
