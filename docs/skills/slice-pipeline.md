---
name: slice-pipeline
description: Run the full slice migration pipeline on a Rust file in order
---

You were pointed at a Rust file that contains commented-out Scala code. Run the full slice migration pipeline on it, executing each step in order.

After each step, verify that the file looks correct before proceeding. If something looks wrong or ambiguous at any step, STOP and report the issue before continuing.

Note that, per CLAUDE.md, you are allowed to run these agents, and they are allowed to modify the code, because they're in .claude/agents/. Per CLAUDE.md:

> Never use spawned agents (the Agent tool) to make code modifications. ... The only exception is agents defined in `.claude/agents/` which are explicitly human-written and approved for modifications.

So please run them as agents. Do not make edits yourself.

DO NOT use sed -i.

# Steps

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

# When done

Say "done" and give a brief summary of what was done at each step.
