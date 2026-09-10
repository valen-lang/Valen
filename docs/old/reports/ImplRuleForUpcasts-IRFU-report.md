# Accuracy report: Impl Rule For Upcasts (IRFU)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Compiler/Templar/Infer Templar.md:356 ("# Send and Impl Rules For Upcasts (SAIRFU)")

## Verdict
Obsolete. The section describes a self-replacing "Send" rule (`CoordSendSR`) that short-circuits to an "Impl" rule (`CallSiteCoordIsaSR`) mid-solve when the receiver turns out to be an interface. That entire mechanism is commented out in src/typing/infer/compiler_solver.rs (the `IRulexSR::CoordSend` match arm and its `CallSiteCoordIsa`/`DefinitionCoordIsa` siblings are all dead, commented-out code), and project history in docs/convos/convo-84-... explicitly records it as "Already retired in-tree" — replaced by a pre-solve upcast step ("§2A's pre-solve upcast *is* the modern answer"). Both live citations (pattern_compiler.rs:57 and compiler_solver.rs:837) point at this retired mechanism; the compiler_solver.rs citation sits directly inside the commented-out block itself.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | The solver has a "Send" rule (`CoordSendSR`) that short-circuits to equality when sender/receiver kinds aren't in a hierarchy, or replaces itself with an "Impl" rule (`CallSiteCoordIsaSR`) when they are | FALSE (as live code) | src/typing/infer/compiler_solver.rs:835-948 (entire arm commented out) | Rewrite to describe the current pre-solve upcast mechanism, or mark as historical-only |
| 2 | This rule-replacement approach is needed because sender/receiver types aren't both known at rule-construction time | UNVERIFIABLE against current code | — | The design rationale may still be historically true, but is moot since the approach was abandoned |
| 3 | Generic receiver case defers to call-site "most specific type" guess (see SMCMST) | UNVERIFIABLE | not checked (SMCMST not audited here) | — |

## Stale citation sites
- src/typing/infer/compiler_solver.rs:837 — comment "See IRFU and SRCAMP for what's going on here" sits inside a `// IRulexSR::CoordSend(coord_send) => {` block that is entirely commented out (dead code), not executing logic. Load-bearing: no — the code doesn't run, so the citation documents nothing live; it's a leftover on retired code.
- src/typing/expression/pattern_compiler.rs:57 — comment "The rules are different depending on the incoming type. See Impl Rule For Upcasts (IRFU)" precedes pattern-conversion logic that calls `solve_for_defining`, but the actual Send/Impl self-replacement rule this comment describes does not exist in the active rule set (`CoordSendSR` is never constructed anywhere in src/, only referenced in comments/panics in src/typing/rune_typing/rune_type_solver.rs:274,544). Load-bearing: no — reading IRFU here would send a reader to a mechanism that no longer runs.

## Uncited sites that embody the arcana
- src/typing/convert_helper.rs:193-242 (`upcast` / `CouldntUpcastT` / `UpcastTE`) — the actual current upcast-resolution logic (impl resolution), which convo-84 identifies as the "§2A pre-solve upcast" that replaced SAIRFU. Not cited to IRFU/SAIRFU, but this is the real modern answer to the same problem.
- src/typing/typing-pass-todo.md:19 — records `CoordSendSR` as "Designed and verified... then reverted pending a coordinated landing," i.e. an explicit project note that the SAIRFU-successor work is unlanded/in-flux, not that the old SAIRFU rule is live.

## Suggested rewrite
Retitle/reframe as historical: prepend a note that this rule-replacement scheme (`CoordSendSR` → `CallSiteCoordIsaSR`/`DefinitionCoordIsaSR`) was implemented in the Rust port's solver but has since been fully commented out and retired, superseded by a pre-solve upcast step that resolves impls before the constraint solver runs (see src/typing/convert_helper.rs `upcast`). Update src/typing/infer/compiler_solver.rs:837 and src/typing/expression/pattern_compiler.rs:57 to either remove the IRFU/SAIRFU citation or point instead at the current pre-solve upcast mechanism, since the cited rule no longer executes.
