---
name: slice-reconcile-delete
description: Delete old Rust definitions marked with "// old, obsolete"
tools: [Read, Edit, Grep]
model: haiku
---

You were pointed at a Rust file that has old Rust definitions marked with `// old, obsolete` comments. These have already been copied to their correct locations (under `// mig:` comments) by a previous step.

Your ONLY job: delete every `// old, obsolete` comment and the definition immediately following it.

**Do NOT touch anything under a `// mig:` comment. Do NOT touch Scala comments. ONLY delete lines marked `// old, obsolete` and the code block directly below them.**

# What to delete

For each `// old, obsolete` comment in the file:
 1. Delete the `// old, obsolete` comment line itself.
 2. Delete the entire definition immediately following it — the `struct`, `impl` block (including all its methods and closing `}`), `fn`, `trait`, or `const`.

# What NOT to delete

 * **NEVER delete `// mig:` comments** or anything under them.
 * **NEVER delete Scala comment blocks** (`/* ... */`).
 * **NEVER delete code that is NOT preceded by `// old, obsolete`.**

# Steps

 1. Find every `// old, obsolete` comment in the file.
 2. For each one, identify the full extent of the definition below it (e.g., for a struct: from `pub struct` to the closing `}`; for an `impl`: from `impl` to the closing `}`).
 3. Delete the `// old, obsolete` comment and the entire definition.
 4. Clean up any extra blank lines left behind (collapse to at most one blank line).

# Rules

 * **ONLY delete things marked `// old, obsolete`.**
 * If you see a definition that looks like it should be deleted but does NOT have `// old, obsolete` above it, do NOT delete it. STOP and report it.
 * If you are unsure about the extent of a definition (where it ends), STOP and report it.

# Restrictions

 * Do not build or test. Do not run `cargo build`, `cargo run`, or `cargo test`.
 * DO NOT use sed -i or other scripts to do this for you. Do it manually.

# When done

Say "done" when you're done deleting old obsolete definitions.
