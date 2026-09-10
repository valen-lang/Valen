# Accuracy report: Each Call-Site Is Its Own Solve (ECSIIOSZ)

Audited against the working tree on 2026-09-06. Arcana doc: docs/arcana/EachCallSiteIsItsOwnSolve-ECSIIOSZ.md

## Verdict

The core mechanism the doc describes — a fresh `InferCompiler`/`SimpleSolverState` per call-site, self-contained rule vectors, the MKRFA/SROACSD/DRSINI/CSSNCE setup contract — is still true and confirmed live at all three code citation sites (src/typing/infer_compiler.rs:257, src/typing/function/function_compiler_solving_layer.rs:267, src/typing/rust_interop/declarations.rs:14). But the doc already knows part of itself is stale: it carries its own trailing `// VCOORD: update this` block (added by a prior session) admitting the ⚠ MKRFA block's central claim — that `RuneParentEnvLookupSR` is "a silent no-op that conceals violations" — is false; that handler is now a `panic!`. That VCOORD note itself has drifted further since it was written: it cites `compiler_solver.rs:1053` for the panic, but the panic is now at src/typing/infer/compiler_solver.rs:1100. The doc's main prose (the ⚠ block) still reads as if live and has not been corrected in place, only flagged. This is minor-inaccuracies: the doc has not gone obsolete or wrong on its central claim, but it is shipping a known-stale warning plus a self-correction that has itself drifted one line-number generation further.

## Claims

| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | Every call-site is lowered into its own self-contained vector of solver rules, and typing spins up a fresh `InferCompiler` solver instance per call-site | TRUE | src/typing/function/function_compiler_solving_layer.rs:267-283 builds `all_rules`/`call_site_rules` per call; src/typing/infer_compiler.rs:257-268 (`make_solver_state`) builds a fresh `SimpleSolverState` per invocation | |
| 2 | Call-site solves don't share state (own rule vector, rune-to-type map, initial-knowns, conclusion map) | TRUE | src/typing/infer_compiler.rs:262-289: `make_solver_state` takes `rules`, `rune_to_type`, `initial_knowns` as fresh per-call params and builds a new `already_known` map each call | |
| 3 | `RuneParentEnvLookupSR` must be preprocessed out into initial-knowns (MKRFA), and the solver's own handler is a no-op that conceals violations if you don't | FALSE (as stated) | src/typing/infer/compiler_solver.rs:1100 — the handler is `panic!("vwat: RuneParentEnvLookupSR should have been MKRFA-preprocessed before reaching the solver: {:?}", r.rune)`, not a silent no-op | The doc's own trailing VCOORD block already says this; the main-body ⚠ text should be rewritten to state the panic exists, not warn about a no-op |
| 4 | Citation `CompilerSolver.scala:852` for the no-op handler | FALSE / stale | No `.scala` files exist anywhere in the repo (grep -rn "RuneParentEnvLookupSR" turns up only .rs files); the doc's own trailing note flags this too | Replace with src/typing/infer/compiler_solver.rs:1100 |
| 5 | Setup contract applies to `ArrayCompiler`, `OverloadResolver`, `ImplCompiler`, `StructCompilerGenericArgsLayer`, `FunctionCompilerSolvingLayer` | UNVERIFIABLE (partial) | Confirmed `FunctionCompilerSolvingLayer` exists and matches (function_compiler_solving_layer.rs). Did not exhaustively verify `ArrayCompiler`/`OverloadResolver`/`ImplCompiler`/`StructCompilerGenericArgsLayer` still exist under those exact names — not in the known citation list and out of budget to chase further | |
| 6 | Cross-referenced codes DBDAR, SROACSD, MKRFA, CSSNCE have no file anywhere | TRUE (per doc's own trailing note, corroborated) | Doc's own appended block states this explicitly; multiple convo logs (docs/convos/convo-8-phased-callsite-solving.md:207, convo-23 ...md:208) independently confirm the same five(!) codes (also NBIFP) have no backing file | The doc's prose still cites SROACSD/MKRFA/DBDAR/CSSNCE as if resolvable references |
| 7 | `where implements(..)` is now a bound rather than a rule, absent from every rule vector this doc describes | UNVERIFIABLE | Did not find `ImplSR`/`implements(` handling in src/typing/infer within budget; doc's own trailing note asserts this as a correction, not independently re-verified here | |
| 8 | Declaration-scoped callers are safe "by accident" because their rule sources never emit `RuneParentEnvLookupSR` | UNVERIFIABLE | Not independently traced in this pass | |

## Stale citation sites

- docs/arcana/EachCallSiteIsItsOwnSolve-ECSIIOSZ.md's own trailing VCOORD block, line citing `compiler_solver.rs:1053` — the panic it refers to is now at src/typing/infer/compiler_solver.rs:1100. The self-correction has drifted one generation further than the correction itself acknowledges.
- The ⚠ block in the main doc body (not yet edited to match the trailing VCOORD note) still asserts the no-op/silent-failure characterization and the `CompilerSolver.scala:852` citation as current fact.

All three .rs code citation sites (src/typing/infer_compiler.rs:257, src/typing/function/function_compiler_solving_layer.rs:267, src/typing/rust_interop/declarations.rs:14) are accurate and not stale with respect to the ECSIIOSZ claim itself — each site's surrounding code still does what the arcana says. (function_compiler_solving_layer.rs:267 carries its own separate VCOORD note about a missing param-rule fold, unrelated to ECSIIOSZ's content.)

## Uncited sites that embody the arcana

- src/typing/infer/compiler_solver.rs:1100 — the `panic!` that is the actual current MKRFA enforcement point; not cited by @ECSIIOSZ or @MKRFA anywhere, despite being exactly the code the doc's ⚠ block is trying to describe.
- Other named setup-contract callers (`ArrayCompiler`, `OverloadResolver`, `ImplCompiler`, `StructCompilerGenericArgsLayer`) were not located/verified for @ECSIIOSZ citations in this pass — worth a follow-up grep for their current file locations to confirm each still carries (or should carry) the citation.

## Suggested rewrite

Not included — verdict is minor-inaccuracies, not major/obsolete.
