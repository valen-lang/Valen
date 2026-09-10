# Accuracy report: Need Bound Information From Parameters (NBIFP)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:491 ("Need Bound Information From Parameters (NBIFP née NBIFPR)")

## Verdict

The core idea is intact and the mechanism is real: `add_runed_data_to_near_env` (src/typing/function/function_compiler_solving_layer.rs:448) does harvest `FunctionBoundNameT` prototypes from citizen-typed parameters into the calling function's near-env, exactly as described, and this is the currently-cataloged push exception under BDPFWDZ. But the doc's own "Note on direction" paragraph carries two stale details it introduced itself: the Scala-cased function name `addRunedDataToNearEnv` (actual Rust name is `add_runed_data_to_near_env`) and a line-pinned citation `InferCompiler.checkResolvingConclusionsAndResolve:295` that no longer matches (the function is `check_resolving_conclusions_and_resolve` at src/typing/infer_compiler.rs:329, not line 295). Additionally, the Monomorphizer subsection's worked example points at a test (`hash_map_tests.rs`) whose body is entirely commented out behind `unimplemented!()` and `#[ignore]`, so "search NBIFP for test case" no longer finds a live, executing test — only dead code kept as an audit trail (matching docs/architecture/instantiator-design.md:661's own characterization: "commented-out code preserved as audit trail"). These are drifted/stale details, not a wrong core mechanism.

## Claims

| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | `add_runed_data_to_near_env` harvests bound prototypes from a citizen-typed parameter's inner env into the calling function's near-env | TRUE | src/typing/function/function_compiler_solving_layer.rs:448 | — |
| 2 | Doc spells the function `addRunedDataToNearEnv` (Scala casing) | DRIFTED | src/typing/function/function_compiler_solving_layer.rs:448 (`add_runed_data_to_near_env`) | Update to snake_case name |
| 3 | This is the currently-cataloged push exception under @BDPFWDZ | TRUE | docs/arcana/ByDefaultPullFromWhereDeclared-BDPFWDZ.md:23 | — |
| 4 | The load-bearing safety invariant is enforced by post-solve bound-arg verification at `InferCompiler.checkResolvingConclusionsAndResolve:295` | DRIFTED | src/typing/infer_compiler.rs:329 (`check_resolving_conclusions_and_resolve`) | Function exists but at line 329, not 295, and Rust name is snake_case |
| 5 | `FunctionBoundNameT` names appear for requirement prototypes like `BorkForwarder<LamT>`'s `__call` bound | TRUE | grep hits in src/typing/templata_compiler.rs, src/typing/names/names.rs, src/typing/citizen/struct_compiler.rs | — |
| 6 | See ONBIFS for abstract-function override bound incorporation | TRUE | docs/arcana/Generics.md:618 ("Overrides Need Bound Information From Structs (ONBIFS)") | — |
| 7 | The Monomorphizer/instantiator also needs to harvest parameter bounds; "search NBIFP for test case" demonstrates the bug | DRIFTED | src/integration_tests/tests/hash_map_tests.rs:12-17 (test `monomorphize_problem` is `#[ignore]`, body is `unimplemented!()` with the actual test commented out) | Note that the cited test is disabled/dead code, not an executing repro; the real mechanism lives in src/instantiating/instantiator.rs's `DenizenBoundToDenizenCallerBoundArgI`/`func_id_to_bound_arg_prototype` machinery instead |

## Stale citation sites

- `docs/arcana/Generics.md:552` ("search NBIFP for test case") → src/integration_tests/tests/hash_map_tests.rs:17 — the comment citing NBIFP is inside commented-out, `unimplemented!()`-gated, `#[ignore]`d test code, not a live test exercising the claim.

## Uncited sites that embody the arcana

- src/typing/function/function_compiler_solving_layer.rs:448 (`add_runed_data_to_near_env`) — the exact mechanism described, has no explicit NBIFP citation in its own doc comment (only referenced indirectly from other docs).
- src/instantiating/instantiator.rs (`DenizenBoundToDenizenCallerBoundArgI`, `func_id_to_bound_arg_prototype`, `struct_to_bounds`) — this is the live instantiator-side bound-hoisting-through-parameters machinery; uncited by NBIFP even though it's the actual current answer to the "Monomorphizer" subsection's question, superseding the dead test.

## Suggested rewrite

Not required — verdict is minor-inaccuracies (drifted names/line numbers, one stale example citation), core mechanism confirmed correct. Recommended mechanical fixes: replace `addRunedDataToNearEnv` with `add_runed_data_to_near_env`; replace `InferCompiler.checkResolvingConclusionsAndResolve:295` with `InferCompiler::check_resolving_conclusions_and_resolve` (src/typing/infer_compiler.rs:329); and note in the Monomorphizer subsection that the cited test case is currently disabled, pointing instead to src/instantiating/instantiator.rs's bound-hoisting maps as the live implementation.
