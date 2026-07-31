# Plan document

Source: `/Users/verdagon/.claude/plans/quirky-soaring-summit.md`
Session: 7716c494-7681-4615-89e2-83d60e6ffff4

---

# Retire the `higher_typing` pass

## Context

The onion typing arc dissolves the higher_typing intermediate pass. Its raison d'être — `explicify_lookups` inserting `CoerceToCoordSR` to bridge Kind-vs-Coord ambiguity — evaporates once Kind=Coord. The `*A` intermediate AST layer's "no `MaybeCoercing*` survives" invariant becomes trivially satisfied by `*S` under onion, so `*A` is dead weight.

Currently `higher_typing/` on disk holds ~2,385 LOC across 8 files + a `tests/` subdirectory (4 files, 651 LOC combined), all `#[cfg(any())]`-gated / unlinked via a commented-out `// pub mod higher_typing;` at `FrontendRust/src/lib.rs:13`. The prior TEMP CHECKPOINT (postparse slice) left it as unlinked-not-deleted; this slice completes the retirement by physically deleting the directory and its `mod` reference.

Zero currently-linked src code references `higher_typing`, so deletion is safe at build time (verified). Downstream cleanup — moving the rune-type solver to `typing/`, deleting the `*A` layer references from `typing/`, wiring per-denizen `rune_to_type` maps onto `coutputs` — is deferred to the eventual typing re-link slice per the vcoord-handoff plan.

## Scope

**In scope:**
1. Preserve regression Vale fixtures from `higher_typing/tests/` before deletion (per architect directive — some tests there are bug repros placed at the earliest pass that triggered them; docs/skills/bug-repro.md:32).
2. Delete the entire `FrontendRust/src/higher_typing/` directory.
3. Delete the commented-out `// pub mod higher_typing;` line in `FrontendRust/src/lib.rs`.
4. Update stale doc references to reflect the retirement.

**Out of scope (deferred to typing re-link):**
- `FrontendRust/src/pass_manager/pass_manager.rs` and `pass_manager/full_compilation.rs` stay gated as-is (architect chose "Leave them gated"). They hold the only remaining `use crate::higher_typing::*` imports in `src/`; they'll become unbuildable-if-un-gated but that's harmless while `pass_manager/mod.rs` keeps them out of the build.
- Moving `rune_type_solver.rs` to a `typing/rune_typing/` subfolder.
- Deleting the `*A` layer's typing-side references (77 lines across 43 files in `typing/`).
- Wiring per-denizen `rune_to_type` maps onto `coutputs`.
- Cleaning up 1-line mentions in `simplifying/`, `instantiating/`, `integration_tests/`.

## Steps

### Step 1 — Preserve regression Vale fixtures

Scan `FrontendRust/src/higher_typing/tests/higher_typing_pass_tests.rs` (373 LOC) and `error_tests.rs` (240 LOC) for tests that look like bug repros — typically identifiable by:
- Test name containing `regression`, `repro`, `bug_`, or an issue number.
- An inline `// bug X` / `// regression for Y` / `// repro for Z` comment.
- A raw-string Vale fixture (`r#"..."#`) attached to the test.

Extract each identified repro's Vale fixture text + a one-line description into a new preservation doc:

**New file:** `FrontendRust/docs/regression-fixtures-from-retired-higher-typing.md`

Structure: one section per repro, each with:
- Original test fn name (as a `## <name>` heading).
- One-line description (from the test's comment or code-inferred intent).
- The `r#"..."#` Vale fixture(s) as fenced code blocks.
- A note that the original test lived in `higher_typing/tests/` and was retired with the pass; any resurfacing of the underlying bug will now surface at postparse or typing.

If a test has no fixture and no repro marker, skip it — that's a positive-behavior test of a dead pass, not a repro. `higher_typing_pass_tests.rs` is likely mostly positive tests; `error_tests.rs` is likely mostly error-emission tests. Expect to preserve maybe 5-15 fixtures, not all 651 LOC.

### Step 2 — Delete `higher_typing/` directory

`git rm -r FrontendRust/src/higher_typing/`

Everything below vanishes: `mod.rs`, `ast.rs`, `higher_typing_pass.rs`, `astronomer_error_reporter.rs`, `higher_typing_error_humanizer.rs`, `patterns.rs`, `textifier.rs`, and the entire `tests/` subdirectory.

### Step 3 — Delete the commented mod line

In `FrontendRust/src/lib.rs`:
- Line 13: delete the `// pub mod higher_typing;` line.
- Line 5: update the `// VCOORD:` comment above it to drop the "higher_typing" name from the list of unlinked passes (current phrasing: "higher_typing, typing, and ..." — becomes "typing, and ...").

### Step 4 — Update doc references

**Doc updates that reflect completed retirement:**
- `vcoord-handoff.md` — update the "immediate next step" section: higher_typing deletion is DONE, not "next work". Advance the current state to "postparse + higher_typing retired; typing next".
- `postparse-slice-plan.md:3,272` — drop mentions of `higher_typing/` as an unlinked slice; note it's now deleted.
- `onion-typing-plan.md:132,138,139` — mark the higher_typing/* line items as done.
- `onion-typing-scouting.md:7,179,428` — update density counts + `explicify_lookups` reference to past tense.

**Doc updates that fix stale paths:**
- `docs/skills/bug-repro.md:32` — the "place regression repros in higher_typing/tests/" instruction becomes wrong. Retarget to `postparsing/test/` or `typing/test/` (whichever is closer to the earliest currently-linked pass that triggers the bug). Add a one-line note that repros for retired higher_typing behavior are preserved in `FrontendRust/docs/regression-fixtures-from-retired-higher-typing.md`.
- `docs/meta.md:35` — drop the `higher_typing/docs/` entry from the docs directory list.
- `FrontendRust/docs/background/arenas.md:41` — arena diagram naming HigherTyping: remove the HigherTyping stage.
- `docs/architecture/typing-pass-design-v3.md:16,37,73` and `typing-pass-ai-guide.md:50` — the `'s` arena is described as "postparser + higher-typing output". Reword to just "postparser output" (or "postparser + typing" if typing owns the same arena post-onion — check the source of truth in `scout_arena.rs`).
- `docs/architecture/simplifier-design.md:5` — pipeline diagram: drop the higher-typing stage box.
- `docs/refactor-thoughts/mkrfa-protocol-leak.md:226` — passing mention; update to past tense.

**Keep as-is:**
- `docs/HigherTypingPass.md` — Scala-era historical doc, per `onion-typing-scouting.md:428` flagged as "preserve".

## Files to modify

Concrete list:

- **Delete outright:** `FrontendRust/src/higher_typing/` (entire directory, 8 files + tests dir).
- **Create:** `FrontendRust/docs/regression-fixtures-from-retired-higher-typing.md` (preservation doc for extracted repros).
- **Modify (1 line each, mechanical):**
  - `FrontendRust/src/lib.rs` (lines 5 and 13).
- **Modify (planning docs, updated to reflect completed state):**
  - `vcoord-handoff.md` (immediate-next-step section).
  - `postparse-slice-plan.md` (lines 3, 272).
  - `onion-typing-plan.md` (lines 132, 138, 139).
  - `onion-typing-scouting.md` (lines 7, 179, 428).
- **Modify (arch docs, remove/reword stale references):**
  - `docs/skills/bug-repro.md` (line 32 + add pointer to preservation doc).
  - `docs/meta.md` (line 35).
  - `FrontendRust/docs/background/arenas.md` (line 41).
  - `docs/architecture/typing-pass-design-v3.md` (lines 16, 37, 73).
  - `docs/architecture/typing-pass-ai-guide.md` (line 50).
  - `docs/architecture/simplifier-design.md` (line 5).
  - `docs/refactor-thoughts/mkrfa-protocol-leak.md` (line 226).

## Verification

1. **Build clean:** `cargo build --manifest-path FrontendRust/Cargo.toml --lib` — zero errors, zero new warnings.
2. **Test suite green at baseline:** `cargo test --manifest-path FrontendRust/Cargo.toml --lib` — expected `489 passed; 0 failed; 1 ignored`. No regressions.
3. **Grep verification** — zero hits from any currently-linked src location:
   ```
   grep -rn "higher_typing\|HigherTyping" FrontendRust/src/lexing/ FrontendRust/src/parsing/ FrontendRust/src/postparsing/ FrontendRust/src/utils/ FrontendRust/src/lib.rs
   ```
4. **Git status sanity check** — expected `git status` shows: `higher_typing/` directory tree marked deleted; `lib.rs`, planning docs, and arch docs modified; preservation doc created. No accidental modifications elsewhere.
5. **Deferred cleanup markers preserved** — 77 `higher_typing` references in `typing/` and the 6 hits in `pass_manager/{pass_manager,full_compilation,code_source,mod}.rs` should still be present (they're inside gated code, will be handled at typing re-link).

## Expected outcome

- ~2,385 LOC of unlinked source code deleted.
- ~10 stale doc references updated to reflect the retirement.
- Regression fixtures preserved in a discoverable location.
- Suite stays green at 489/0/1.
- No behavioral change (higher_typing was already unlinked; deletion just makes it official).

Ready for a `fire commit temporary to experimental` or (once you also want CI/integration) a full `fire commit to experimental` afterwards.
