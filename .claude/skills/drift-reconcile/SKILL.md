---
name: drift-reconcile
description: Iteratively reconcile a drifted FrontendRust/ file with the canonical Frontend/ Scala source. For each divergent line, update the FrontendRust/ /* scala */ audit-trail block to match Frontend/, then update the adjacent Rust code to reflect the change. Uses SCPX --check-all as the driving signal.
---

> **Every time you compact, re-read this file** (`docs/skills/drift-reconcile.md`). It changes often as the TL adds notes about new gotchas and escalation patterns learned during the session. Compaction drops the prior conversation but not the file — if you re-read it, you pick up everything the previous instance learned.

## What This Is

The Vale repo's `Frontend/` Scala has moved forward of the `FrontendRust/` migration in many places. Each `.rs` file under `FrontendRust/src/` carries `/* scala */` audit-trail block comments mirroring the original Scala — but those comments have drifted out of sync with the actual `Frontend/...File.scala`. Same goes for the Rust code itself: if Scala added a parameter, the Rust never got it; if Scala removed a method, the Rust still has the placeholder; etc.

This skill drives the per-file reconciliation: walk one drifted file, fix one divergence at a time, mirror each Scala change in the adjacent Rust definition, confirm the build still passes, repeat. The goal is **monotonic drift reduction** — every edit makes things less drifted, never more.

The companion skill `migration-drive` is the test-driven migration loop (panic → real logic). This skill is different: it presumes the Rust *body* exists (even if stub-shaped) and the work is bringing it back into shape with whatever Scala has done since the migration started.

## Driving Signal

Run SCPX in audit mode to see what's drifted:

```bash
cargo run --manifest-path Luz/shields/ScalaCommentParity-SCPX/Cargo.toml --release -- --check-all > ./tmp/drift-reconcile.txt 2>&1
```

Each `MISMATCH:` block lists a Rust file and the unified diff between its `/* scala */` blocks (normalized) and the canonical Scala source. The bottom line summarizes total drift (e.g. `84 mismatches`).

This is your driving target. As you reconcile files, total drift goes down. SCPX's per-edit gate (monotonic-improvement) ensures no edit can make it go up.

## Required Reading

Read these in full before driving any reconciliation:

1. **This file.**
2. **`TL.md`** — current migration state, what's expected of you.
3. **`docs/architecture/typing-pass-design-v3.md`** — typing-pass arena and design decisions (if reconciling typing-pass files).
4. **`FrontendRust/docs/migration/migration-policy.md`** — DCCR, RCSBASC, architect-level escape hatch.
5. **`Luz/shields/ScalaCommentParity-SCPX.md`** — the spec for the shield that gates your edits.
6. **`Luz/shields/ScalaParityDuringMigration-SPDMX.md`** — body-translation rules; Rust must mirror Scala's shape.
7. **`Luz/shields/MigrateAllCommentsToo-MACTX.md`** — explanatory comments in Scala must reappear in Rust.

## The Loop

```
1. Pick a drifted file from --check-all output (start with the one with the
   smallest diff — easiest review).
2. Open the Rust file and the canonical Scala file side by side.
3. For each divergence in the diff:
   a. Locate the corresponding /* scala */ block in the Rust file.
   b. Edit the /* scala */ block ONLY to match the canonical Scala line(s).
   c. Locate the Rust definition adjacent to that /* scala */ block.
   d. Edit the Rust definition to mirror the Scala change.
   e. Run `cargo check --manifest-path ./FrontendRust/Cargo.toml --lib` to
      confirm the Rust still compiles. If it doesn't, fix or escalate (see
      "Escalation Triggers").
4. After the whole file is reconciled, run the full test suite.
5. Re-run --check-all to confirm the file is no longer in the MISMATCH list
   (or, if not fully clean, has strictly fewer mismatches).
6. Pick the next file. Repeat.
```

## Edit Discipline

### One divergence per edit

Don't bundle multiple unrelated changes. Each edit should:
- Touch one `/* scala */` block and one adjacent Rust definition.
- Have a one-line description: "Sync `<Method>` signature to add `<param>`," "Remove deleted-from-Scala helper `<X>`," etc.

This keeps SCPX's per-edit verification meaningful and the diff reviewable.

### Edit the `/* scala */` block first, then the Rust

The audit-trail block is the spec; the Rust is the implementation. Bring the spec up to date first, then update the implementation to match the new spec. Two separate edits is fine (and often safer): the first edit reduces SCPX drift; the second changes Rust without touching SCPX at all.

Doing it in the other order — edit Rust first, then audit-trail — is also acceptable when the Rust change is mechanical and the audit-trail update is a single-line fix. Pick whichever is least confusing for the divergence at hand.

### Don't touch unrelated parts of the file

If you see other drift in the file that isn't your current target, leave it. Make a separate edit for it (or note it as a follow-up). Bundling makes the review harder and risks SCPX flagging a partial fix as introducing new drift.

### Don't "improve" the Scala-comment block

The `/* scala */` block must match `Frontend/.../File.scala` byte-for-byte (modulo the documented normalization: whitespace, blank lines, `MIGALLOW:`/`Guardian:`/`AFTERM:` lines stripped). Don't rephrase, don't fix Scala bugs you spot, don't add explanatory commentary in the block. If you think the Scala is wrong, escalate — don't unilaterally edit either the Scala or the audit-trail to be "better."

### Translate every line literally (SPDMX)

When the Scala change implies a Rust change, mirror Scala's structure exactly. No simplifications, no idiomatic Rust improvements, no "obviously equivalent" rewrites. The Rust diff should look as much like the Scala diff as the language allows.

## Patterns

### Pattern A: Scala added a parameter

Scala `def foo(env: Env, name: Name)` became `def foo(env: Env, name: Name, opts: Opts)`.

- Update the `/* */` block: add the param.
- Update the Rust `fn foo(...)`: add the param with the corresponding type.
- Update every call site in this file (Guardian will fire NNDX on a new helper, but adding a param to an existing fn doesn't trip NNDX).
- Pass through `panic!("opts not used yet")` to the body if it's not actually consumed — don't invent new logic.

### Pattern B: Scala renamed a method

Scala `def lookupFoo` became `def lookupFooByName`.

- Update the `/* */` block.
- Rename the Rust method to `lookup_foo_by_name`.
- Update call sites in this file.
- Guardian NRDX may fire on the rename — that's expected; the TL/architect can issue a temp-disable per the documented in-file precedent for "Scala-driven rename."

### Pattern C: Scala added a new case-class field

Scala `case class FooT(a: Int)` became `case class FooT(a: Int, b: String)`.

- Update the `/* */` block.
- Add the field to the Rust struct.
- Update constructor call sites (in this file) to pass the new field.
- If the field is `Option[X]`, pass `None` initially.
- If the field is a slice/collection, pass `&[]` or arena-allocate an empty slice.
- The actual usage of the field is a separate work item — leave a `// TODO via the architect` note? No — escalate the deferred-usage decision to the TL. Don't write `// TODO:` (TL.md prohibits it).

### Pattern D: Scala deleted a method/class/field

Scala had `def helper()`, now removed.

- Update the `/* */` block: remove the deleted Scala lines.
- Remove the corresponding Rust definition.
- If the Rust definition is still called from elsewhere in this file, the call sites also need to go (or be rewritten to use whatever the Scala uses instead — check `Frontend/...File.scala` to see what replaced it).
- If call sites outside this file reference the removed definition, that's a cross-file change — escalate to the TL.

### Pattern E: Scala refactored a sealed-trait hierarchy

Scala added or removed a variant from a sealed trait (e.g. a new `case object NewKindS`).

- Update the `/* */` block(s).
- Update the corresponding Rust enum.
- New variant → add it to the enum with `panic!()` in any match arms that would now be non-exhaustive. Escalate the variant's actual data + behavior to the TL.
- Removed variant → delete it from the enum and remove any match arms that exclusively handled it.
- Renamed variant → rename the Rust variant; expect NRDX, expect TL temp-disable.

### Pattern F: Scala's comment-only change

Scala has a new explanatory comment, or a removed one.

- Update the `/* */` block to mirror it (per MACTX — explanatory comments belong in Rust too).
- If the Scala comment is now embedded near a Rust definition that has its own copy of the same comment, no Rust action — the comment already lives there.
- If the Scala comment is new and explanatory, mirror it as a `//` comment above the corresponding Rust definition.

## Escalation Triggers

Stop and write to `for-tl.md` if any of these:

- **Cross-file change.** The Scala-side delta touches a definition referenced from another `.rs` file. Don't unilaterally propagate; the TL coordinates.
- **Lifetime puzzle.** `cargo check` returns a lifetime error you can't resolve in one obvious fix. Lifetime errors in this codebase fool everyone; escalate.
- **NRDX, NMDX, NNDX, SPDMX firing on the obvious Rust change.** Don't temp-disable yourself; the TL handles shield interactions.
- **Scala adds a variant whose Rust translation isn't mechanical.** Escalate the design before adding it.
- **Scala adds/changes a method body in a way that requires real Rust logic.** Mirror the signature change, leave a `panic!()` placeholder for the body, and escalate the body-translation to the TL (or to `migration-drive` once the gate is open for it).
- **You can't find the corresponding Rust definition.** Maybe the slice pipeline skipped it; maybe it was deleted accidentally; maybe it lives in a different file now. Don't invent — escalate.

Format an escalation in `for-tl.md` as:

```
## drift-reconcile: <filename>

**Scala:** Frontend/<path>:<lineno>  (canonical, ahead)
**Rust audit trail:** FrontendRust/src/<path>:<lineno>
**Rust definition:** FrontendRust/src/<path>:<lineno>  (if applicable)

**Divergence (from --check-all):**
<paste the relevant +/- lines from the unified diff>

**Proposed action:** <what you want to do>

**Blocker:** <what's stopping you — lifetime error, NRDX, cross-file refs, etc.>
```

Always lead with the `## drift-reconcile:` heading — `for-tl.md` is shared
with the `migration-drive` workflow, so the prefix tells the TL which
workflow this escalation belongs to.

## The `z` Protocol

When the architect types just `z` in chat, that's the signal to **check for
`for-jr.md` at the repo root**; if it exists, read it (it's the TL's response
to a previous escalation), apply the instructions, and then delete the file.
Same shape as `migration-drive` — the two workflows share both files.

After applying:
- If the response unblocks a specific drift-reconcile escalation, continue
  the loop on that file.
- If the response includes a new instruction (e.g. "skip this file for
  now," "use approach X for this pattern going forward"), update your
  in-progress notes (e.g. `tmp/drift-reconcile-progress.md`) so future
  sessions inherit the guidance.

**When you escalate, stop driving the current file** — don't open another
file in parallel, don't skip to a different divergence. Wait for the TL
response in `for-jr.md` before continuing. Never defer or skip a divergence
to move on; if you can't solve it, escalate and wait.

## Verification Before Moving On

After reconciling a file:

1. `cargo check --manifest-path ./FrontendRust/Cargo.toml --lib > ./tmp/drift-reconcile.txt 2>&1`
   — must be 0 errors. Pre-existing warnings are fine; new ones aren't.
2. `cargo nextest run --manifest-path ./FrontendRust/Cargo.toml > ./tmp/drift-reconcile.txt 2>&1`
   — currently-passing tests must still pass. Run the full non-ignored set; don't filter to just "your" test.
3. `cargo run --manifest-path Luz/shields/ScalaCommentParity-SCPX/Cargo.toml --release -- --check-all > ./tmp/drift-reconcile.txt 2>&1`
   — confirm total `mismatches` count is strictly lower than before. If your file dropped out of the MISMATCH list entirely, that's the gold standard.

If any of these regress, STOP and escalate — don't paper over with a temp-disable or "obvious" fix.

## Use the Same `./tmp/` File

Per CLAUDE.md's build-and-run convention: pipe every `cargo` / `sbt` invocation into the same `./tmp/drift-reconcile.txt` for the session. Inspect with separate follow-up `grep`/`tail`/`head` commands. Never chain a heavy build with `| tail` — you lose the ability to re-analyze.

## Notes

- **The audit-trail block is the spec.** You're never allowed to make the audit-trail block diverge *more* from the canonical Scala — SCPX's per-edit gate (now monotonic) enforces this. Your only allowed direction is "audit-trail moves toward Scala."
- **Comments matter (MACTX).** When Scala has explanatory comments inside or above a `def`, mirror them as `//` comments above the corresponding Rust `fn`. The `/* */` audit-trail block holds the original Scala verbatim; explanatory `//` comments above the Rust definition are the runtime version.
- **Don't add `// TODO:` or `// AFTERM:` comments.** TL.md prohibits them. If the Rust translation has to wait, escalate; don't park it inline.
- **Don't add `// NOVEL CODE` to anything you're reconciling.** That marker is for code with no Scala counterpart. Reconciled definitions have a Scala counterpart by construction.
- **`/* Guardian: disable-all */` blocks** are a separate concern from drift. If you see one, leave it alone unless the SCPX diff specifically targets it.
- **When you finish a file, mark it done in `tmp/drift-reconcile-progress.md`** (create the file if absent) with: filename, date, before-mismatch-count, after-mismatch-count, brief notes. Future sessions can pick up where you left off.

## See Also

- `migration-drive` — the test-driven body-migration loop (the other workflow that's active on this branch).
- `Luz/shields/ScalaCommentParity-SCPX.md` — the shield that gates your edits.
- `Luz/shields/ScalaParityDuringMigration-SPDMX.md` — body-translation rules.
- `Luz/shields/MigrateAllCommentsToo-MACTX.md` — comment-mirroring rule.
- `TL.md` — operational handoff.
