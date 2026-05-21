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

## Step 0: Verify the pass migration policy exists

The pipeline is pass-aware. Each pass needs a `migration-policy.md` next to its source root (walking up from the target file). If none exists, STOP and tell the user — running without one produces the cleanup burden documented in `FrontendRust/docs/migration/migration-policy.md` and TL.md §"Cleaning Up After The Slice Pipeline."

Check by walking up from the target file's directory. Do NOT accept `FrontendRust/docs/migration/migration-policy.md` as a match — that is the template/canonical-example, not a real per-pass policy.

If found, paste the full policy path into the spawn prompt for each subsequent agent so they can read it.

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

If SCPX is green: also do a `cargo check --manifest-path FrontendRust/Cargo.toml --lib > ./tmp/slice-pipeline-check.txt 2>&1` and report the error count. Pre-existing warnings are fine; new compile errors mean placehold produced something rustc rejects.

# When done

Say "done" and give a brief summary of what was done at each step, including the SCPX result and the post-pipeline cargo-check error count.
