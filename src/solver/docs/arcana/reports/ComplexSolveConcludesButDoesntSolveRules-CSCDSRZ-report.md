# Accuracy report: Complex Solve Concludes But Doesn't Solve Rules (CSCDSRZ)

Audited against the working tree on 2026-09-06. Arcana doc: src/solver/docs/arcana/ComplexSolveConcludesButDoesntSolveRules-CSCDSRZ.md

## Verdict

The doc already contains a self-appended `// VCOORD: update this` correction note that is itself accurate: the "Where" list's `CompilerRuleSolver.complexSolve` no longer exists — `src/typing/infer/compiler_solver.rs` has no complex solve at all (grepping the file for `Complex`/`complex_solve` finds nothing), and typing's only remaining complex-solve function, `complex_solve()` in `src/typing/rune_typing/rune_type_solver.rs:956`, is an unconditional `panic!("Unimplemented complex_solve")`. The mechanism described (claims 1-4, the "Why it exists" section, the interface names) is still live and correctly described at the solver-framework level (`src/solver/`), verified by three passing tests in `src/solver/test/solver_tests.rs`. So the body of the doc is accurate for the framework but false as a description of the typing pass, and the doc knows it — it just hasn't been rewritten to remove the stale framing or promote the correction to the main text. This is a "minor-inaccuracies" situation only in the narrow sense that the doc author already diagnosed the drift; as a reference doc it is confusing (a reader following "Where" straight to `compiler_solver.rs` finds nothing) and should be rewritten, not left with an unresolved TODO note bolted onto the bottom.

## Claims

| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Complex solve infers conclusions from multiple unsolved rules but does not mark them solved; rules stay unsolved until the next simple solve pass | TRUE (framework level) | src/solver/test/test_rule_solver.rs:14 `complex_solve_impl`; src/solver/test/solver_tests.rs:310-345 (3 passing tests) | |
| 2 | `CompilerRuleSolver.complexSolve` exists (in "Where" list) | FALSE | `grep -n "Complex\|complex_solve" src/typing/infer/compiler_solver.rs` → no matches | This name should be removed from "Where"; typing pass has no complex solve today. |
| 3 | `SimpleSolverState.commitStep` — called with empty `solvedRuleIndices` from complex solve | DRIFTED (naming) | The actual site is `compiler_solver.rs:317` `solver_state.rule_is_solved(solving_rule_index)` under a `// Per @CSCDSRZ, only true after simple solve.` comment — real names in Rust are `SolverState`/`commit_step`, not the Scala-cased names quoted | Doc's own note (line 38-39) already flags "Scala spellings throughout" as stale wording. |
| 4 | `advanceInfer` / `advance` loop cited as stage1→stage2→stage1 cycle | UNVERIFIABLE for typing pass (no complex solve stage exists to cycle to) | src/typing/infer/compiler_solver.rs has no complex-solve stage | Should be scoped explicitly to `src/solver/` framework loop, not typing's `advance`. |
| 5 | `TestRuleSolver.complexSolveInner` in solver tests — same pattern | DRIFTED (naming) | Actual name is `complex_solve_impl`, src/solver/test/test_rule_solver.rs:14 | Rename in doc. |
| 6 | Complex solve step's only observable effect is new entries in `getConclusions()`; rule solved only via simple solve `commitStep` | TRUE (framework) | src/solver/test/solver_tests.rs:310-345 tests assert conclusions appear without rules marked solved | |
| 7 | The citation at compiler_solver.rs:316 (doc's known site) documents "only true after simple solve" invariant | TRUE / current line shifted to 316 (assert at 317) | src/typing/infer/compiler_solver.rs:316-317 | Line number in the task's "known citation" (316) is the comment line; the assert is 317 — close enough, not stale. |
| 8 | (VCOORD note) typing's only remaining complex-solve is `rune_type_solver.rs:826`, an unimplemented panic | DRIFTED (line number) | Actual panic is at src/typing/rune_typing/rune_type_solver.rs:957 (`fn complex_solve()` declared 956, panics at 957); file also moved to `rune_typing/` subdirectory | Note's line number (826) and bare filename are stale; correct path is `src/typing/rune_typing/rune_type_solver.rs:956-957`. |
| 9 | (VCOORD note) `where implements(..)` now comes back as a bound rather than a rule, because complex solve's Isa-walking is gone | UNVERIFIABLE (plausible, not independently confirmed) | No direct grep evidence found tying this causally; compiler_solver.rs:78-118 shows `BadIsaSubKind`/`IsaFailed` error variants still present, but no complex-solve Isa-walk exists to compare against | Left as asserted by prior session; not falsified but not independently reproduced either. |

## Stale citation sites

- `src/typing/infer/compiler_solver.rs:316` — the comment `// Per @CSCDSRZ, only true after simple solve.` still matches its assertion on the next line (`rule_is_solved` only true post simple-solve), so this citation itself is not stale. The staleness is one level up: the doc's "Where" section this citation implicitly points readers to (`CompilerRuleSolver.complexSolve`) doesn't exist in this file — there is no complex-solve stage anywhere in `compiler_solver.rs` for the comment to relate to structurally, only the residual assertion about solved-timing.
- The doc's own embedded note names `rune_type_solver.rs:826` — current location is `src/typing/rune_typing/rune_type_solver.rs:956` (`fn complex_solve()`) / `:957` (the panic).

## Uncited sites that embody the arcana

- src/solver/test/solver_tests.rs:310, 323, 340 — three tests explicitly labeled "Tests @CSCDSRZ" (these do cite it, so not uncited — noting for completeness that citation coverage on the framework side is good).
- src/typing/rune_typing/rune_type_solver.rs:956-957 — `fn complex_solve()` panicking as unimplemented embodies the doc's "typing pass consumer is gone" claim but carries no `@CSCDSRZ` citation; worth tagging so a future reader lands here from the arcana.

## Suggested rewrite

Replace the "Where" section and fold the VCOORD note into the main body:

```
## Where

- `complex_solve_impl` in `src/solver/` (framework-level implementation, e.g. `src/solver/test/test_rule_solver.rs`)
- `SolverState::commit_step` — called with empty `solved_rule_indices` from complex solve
- The solver framework's advance loop — simple-solve / complex-solve alternation
- Three passing tests in `src/solver/test/solver_tests.rs` exercise this end to end

**Not currently used by the typing pass.** `src/typing/infer/compiler_solver.rs` has no
complex-solve stage; its only citation (line 316) is a residual assertion about solved-timing,
not an active complex-solve call. Typing's only remaining `complex_solve` function is
`src/typing/rune_typing/rune_type_solver.rs:956`, an unconditional
`panic!("Unimplemented complex_solve")`. The mechanism is alive and tested at the solver-framework
level only.

Complex solve's disappearance from the typing pass is load-bearing: it's why `where implements(..)`
now comes back as a *bound* rather than a rule — complex solve walking sender→receiver edges for
common ancestors was the only thing that read an `Isa` mid-solve, so with it gone the relation is
minted after the solve instead.
```
