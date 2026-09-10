# Accuracy report: Whether To Merge Equal Rules Beforehand (WTMERB)

Audited against the working tree on 2026-09-06. Source: `docs/old/Compiler/Templar/Infer Templar.md:467-489`

## Verdict
This is a design-decision rationale from the old Scala Templar (a "we decided not to do X, because Y/Z/W" note), not a claim about present behavior of specific code — it has no citing sites anywhere in the tree (`grep -w WTMERB` across src/, Backend/, docs/ returns nothing). Its underlying mechanism did survive the port: `IRulexSR::Equals` is still solved directly (not pre-canonicalized/merged) in `src/typing/infer/compiler_solver.rs:200,802-806`, and the sibling concept it references, SRCAMP (late-added Equals rules during solving), is still live in the same file. The one claim that no longer clearly holds is the "Or rules" reason — no `Or` rule variant was found in the current `IRulexSR` enum, so that specific justification may be stale, though the other two reasons (no real time savings, late-added Equals rules) are independently sufficient and still true. Recommendation: leave it in docs/old — it's an internal rationale note with zero citations, the core outcome it argued for is still the code's behavior, and it doesn't carry enough independent weight or unique framing to warrant a new arcana doc; not worth deleting since it correctly explains a design choice that's still in effect, but no action needed.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Runes are "canonicalized" into integers for solving speed, and Equals rules could in principle assign equal runes the same integer. | TRUE (background) | src/typing/infer/compiler_solver.rs — rune-keyed solver state | — |
| 2 | The team chose NOT to merge/dedupe equal runes ahead of time. | TRUE | No merge/dedup logic found in src/typing/infer/compiler_solver.rs; `IRulexSR::Equals` is solved by direct lookup at compiler_solver.rs:802-806 rather than through a pre-merged canonical rune. | — |
| 3 | Reason: dedup ahead of time wouldn't save much time over propagating through the solver. | UNVERIFIABLE | Performance claim, not checkable from static code. | — |
| 4 | Reason: Equals rules are sometimes added late, during solving itself (see SRCAMP). | TRUE | SRCAMP is a live concept, cited in src/typing/infer/compiler_solver.rs. | — |
| 5 | Reason: some Equals rules run conditionally inside Or rules and can't be merged ahead of time. | DRIFTED | No `Or` variant found in the current `IRulexSR` rule enum (grep for `Or(` / Or-rule handling in src/typing/ast, src/typing/compiler.rs, overload_resolver.rs, infer_compiler.rs turned up nothing beyond unrelated `Or` text matches). | Or-conditional rules appear to have been dropped or renamed in the port; this specific justification may no longer apply, though the other two independently support the same conclusion. |

## Stale citation sites
None — the ID has zero citations in src/, Backend/, or docs/ (excluding docs/old itself).

## Uncited sites that embody the arcana
- src/typing/infer/compiler_solver.rs:200,802-806 — `IRulexSR::Equals` solved via direct rune lookup, i.e., the "don't pre-merge, propagate through the solver" decision in practice.

## Suggested text
Not applicable (D3 kind, verdict is not major-inaccuracies/obsolete).
