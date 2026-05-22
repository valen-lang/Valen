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

# Deletion bounds: where deletion STOPS

This is the most common bug — agents over-extend the deletion range past the definition's true end. Be precise:

 * **For a `struct` / `enum` / `const` / `fn`:** deletion ends at the closing `}` (struct/enum) or `;` (const) or the closing `}` of the fn body. The line containing the closing `}`/`;` is the LAST line you delete.
 * **For an `impl` block:** deletion ends at the closing `}` of the `impl` itself — the brace at indent level 0 that matches the opening `impl X {`. NOT at the closing brace of any inner method, NOT at the next `// mig:` marker, NOT at the next blank line. **Count brace depth** as you scan downward; you stop only when the depth returns to where it was at the `impl` line (typically 0).
 * **For a `trait` block:** same as `impl` — match the trait's outer `{` and `}`.

After you delete the last line of the definition (`}` or `;`), **STOP**. Anything below that — blank lines, `/* … */` Scala blocks, other `// mig:` markers, other definitions — is NOT yours to touch.

**Common failure modes to actively avoid:**

 * Treating the next `// mig:` marker as the deletion boundary. ❌ Wrong — the marker's content is unrelated to the obsolete definition.
 * Treating "the next blank line" as the boundary. ❌ Wrong — blank lines exist inside impl blocks.
 * Treating "the next `/* … */`" as the boundary. ❌ Wrong — Scala comment blocks below an obsolete impl are stay-content (they're the audit-trail for the migrated definition that lives elsewhere in the file).
 * Confusing inner-method braces with the outer-impl brace. ❌ Wrong — track brace depth explicitly.

If you cannot unambiguously identify where the marked-obsolete definition ends (e.g. mismatched braces, unclear nesting, multiple plausible closing `}` candidates), STOP and report. Do NOT guess.

# Steps

 1. Find every `// old, obsolete` comment in the file.
 2. For each one, identify the full extent of the definition below it using the brace-depth-counting rule above. Mentally mark the first line (the `// old, obsolete` itself) and the last line (the closing `}`/`;` of the definition).
 3. Delete exactly that range — first line through last line, inclusive. Nothing before, nothing after.
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
