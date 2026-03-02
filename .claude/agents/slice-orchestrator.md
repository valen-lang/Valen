---
name: slice-orchestrator
description: Run the full slice migration pipeline on a Rust file in order (project)
tools: [Read, Task, Grep]
model: sonnet
---

You were pointed at a Rust file that contains commented-out Scala code. Run the full slice migration pipeline on it by orchestrating the individual slice subagents in order.

After each step, verify that the file looks correct before proceeding. If something looks wrong or ambiguous at any step, STOP and report the issue before continuing.

# Steps

## Step 1: slice-start

Invoke the `slice-start` subagent using the Task tool. Pass the file path.

This inserts `// mig: def/class/...` comments above every Scala definition, splitting the Scala comment blocks so each definition is isolated.

Wait for the agent to complete and verify the results look correct.

## Step 2: slice-rustify

Invoke the `slice-rustify` subagent using the Task tool. Pass the file path.

This translates the Scala-style mig comments to Rust-style (`def` → `fn`, `class` → `struct` + `impl`, `val` → `const`, etc.).

Wait for the agent to complete and verify the results look correct.

## Step 3: slice-placehold

Invoke the `slice-placehold` subagent using the Task tool. Pass the file path.

This generates a Rust placeholder stub below each `// mig:` comment. It ignores all existing Rust definitions.

Wait for the agent to complete and verify the results look correct.

## Steps 4–6: Reconciliation (only if the file has pre-existing Rust definitions)

Before running these steps, check: does the file have any Rust definitions that were written **before** the slicing process (i.e., NOT directly under a `// mig:` comment)? These are old definitions that need to be reconciled with the new stubs.

**If there are NO such old definitions, skip steps 4–6 entirely.**

### Step 4: slice-reconcile-mark

Invoke the `slice-reconcile-mark` subagent using the Task tool. Pass the file path.

This adds `// old, obsolete` above each old Rust definition that has a matching stub.

Wait for the agent to complete and verify the results look correct.

### Step 5: slice-reconcile-copy

Invoke the `slice-reconcile-copy` subagent using the Task tool. Pass the file path.

This copies the `// old, obsolete` code into the matching placeholder stubs.

Wait for the agent to complete and verify the results look correct.

### Step 6: slice-reconcile-delete

Invoke the `slice-reconcile-delete` subagent using the Task tool. Pass the file path.

This deletes everything marked `// old, obsolete`.

Wait for the agent to complete and verify the results look correct.

# When done

Say "done" and give a brief summary of what was done at each step.
