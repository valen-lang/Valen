# Accuracy report: Complex Solve As Last Resort (CSALR)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Compiler/Templar/Infer Templar.md:570 ("# Complex Solve As Last Resort (CSALR)")

## Verdict
The section proposes that the Scala Templar's type solver, when it can't finish a "simple solve," fall back to a "complex solve" that picks the narrowest possible type among candidate senders/subtypes ("SMCMST" — the preceding section, "Should Match Chosen Most Specific Type"). The one citing site, docs/handoffs/call-site-dispatch-handoff.md:169-172, states outright that this strategy has been retired: Rust's call-argument type checking rejects the ambiguous-variance programs CSALR was meant to resolve via LUB/most-specific-guessing, and the strategy is "consistent with three decided things: Valen refuses variance outright, the overload redesign says *no specificity, no fallback, no tiebreakers*, and `complex_solve` is already dead." Current code confirms it: `src/typing/rune_typing/rune_type_solver.rs:956-957` has a `complex_solve()` function that unconditionally panics with "Unimplemented complex_solve," and `src/typing/infer/compiler_solver.rs` has no complex-solve stage at all — the mechanism the doc describes was never built in the Rust typing pass and the design direction it depended on (implicit variance / most-specific fallback) was explicitly rejected. Recommendation: leave in docs/old (it's dead history, already correctly superseded and cited as such by the handoff) — do not migrate.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "To accomplish SMCMST, the solver will ask its delegate to do a 'complex solve', as a last resort if it can't figure out everything the normal way." | OBSOLETE | docs/handoffs/call-site-dispatch-handoff.md:169-172 ("This retires SMCMST/CSALR..."); src/typing/infer/compiler_solver.rs has no complex-solve stage | The strategy was abandoned; typing pass never implements this fallback. |
| 2 | "The previous, normal way to solve is a 'simple' solve." | TRUE (framework-level, still-live terminology) | src/solver/docs/arcana/ComplexSolveConcludesButDoesntSolveRules-CSCDSRZ.md; src/solver/test/solver_tests.rs:30,53 (simple/complex solve staging still exists in the generic `src/solver/` framework) | Applies to the generic solver framework, not to the typing pass's call-argument inference this section is actually about. |
| 3 | "Hopefully, we can find a way to optimize these and maybe even fold them back into the main simple solver. Currently, they do some looping, which causes a O(n^2)." | NOT-AN-ARCANA-CLAIM (open question / musing) | — | No checkable code claim; superseded by the retirement noted in claim 1. |

## Stale citation sites
docs/handoffs/call-site-dispatch-handoff.md:169 — this site already correctly treats CSALR as retired/dead ("This retires SMCMST/CSALR... `complex_solve` is already dead"); it is not stale, it is the evidence for the obsolete verdict. No correction needed there.

## Uncited sites that embody the arcana
None found — the concept this section describes (a last-resort complex/most-specific-guessing solve for call-site type inference) was deliberately not built; the only `complex_solve` in the typing pass (src/typing/rune_typing/rune_type_solver.rs:956-957) is an explicit unimplemented panic, not an implementation of CSALR's idea. The separate, unrelated "complex solve" staging mechanism in src/solver/ (see CSCDSRZ arcana) belongs to a different arcana entry and should not be cited from here.

## Suggested rewrite
N/A — omitted per instructions (only required for major-inaccuracies/obsolete verdicts needing a rewrite; since this is confirmed dead history with an already-accurate citation explaining its retirement, no rewrite of the old section is warranted — it should simply stay in docs/old as historical record).
