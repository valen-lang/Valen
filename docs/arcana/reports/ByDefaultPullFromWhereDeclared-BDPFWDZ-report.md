# Accuracy report: By Default Pull From Where Declared (BDPFWDZ)

Audited against the working tree on 2026-09-06. Arcana doc: docs/arcana/ByDefaultPullFromWhereDeclared-BDPFWDZ.md

## Verdict

The core mechanism (pull-style env resolution, the single cataloged push exception, the interaction with ECSIIOSZ/BRRZ) is accurate and every code citation site still does what the arcana says. The doc's own trailing "VCOORD: update this" section already correctly diagnoses two staleness issues — `addRunedDataToNearEnv` should read `add_runed_data_to_near_env`, and `OverloadResolver.getPlaceholderImplBoundEnvs` doesn't exist in the Rust tree — but the fixes described there were never applied to the doc's main body, which still uses the stale camelCase name and the nonexistent method name. This is a minor, already-self-diagnosed inaccuracy; nothing else in the doc is wrong.

## Claims

| # | Claim (quoted or closely paraphrased from the doc) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Vale's environments are pull-style; declarations live where introduced, consumers walk parent/impl chains to reach them. | TRUE | src/typing/overload_resolver.rs:690, src/typing/templata_compiler.rs:1928 | |
| 2 | "Introduced together" pillar: a thing and its callable surface are introduced in the same scope (struct→package, placeholder→function near-env, `where` clause→function near-env). | TRUE | src/typing/function/function_compiler_solving_layer.rs:1083 (`conjure_impl_bounds_for_defining` writes the `Isa` into the declaring function's own near-env) | |
| 3 | "Stay-in-place" pillar: declarations don't propagate downward/sideways; lookups walk to find them. | TRUE | src/typing/templata_compiler.rs:1928 (placeholder env stays empty; bounds live in introducing near-env) | |
| 4 | Cataloged push exception: `addRunedDataToNearEnv` harvests `FunctionBoundNameT` prototypes from citizen-typed parameters' inner envs into the calling function's near-env. | DRIFTED | src/typing/function/function_compiler_solving_layer.rs:448 | Function is now named `add_runed_data_to_near_env` (snake_case); the doc's main body still spells it `addRunedDataToNearEnv`. The doc's own trailing note already flags this but the body text was never updated. |
| 5 | Pull replacement for the exception: `OverloadResolver` walks the calling function's parameter envs at lookup time. | UNVERIFIABLE (as literally stated) | grep found no `get_placeholder_impl_bound_envs` / `GetPlaceholderImplBoundEnvs` in src/ | No such method exists in the Rust tree; the doc's own note confirms this and says the only trace is a comment in the gated `integration_tests/`. The refactor described is aspirational/not-yet-done, not a currently-real pull path. |
| 6 | Interaction with ECSIIOSZ: impl bounds are pulled from the calling env at solve time. | TRUE | src/typing/infer_compiler.rs:658 (comment: "Counter to @BDPFWDZ: this harvests... Pull-aligned replacement is to walk the citizen's env at lookup time instead.") | |
| 7 | Interaction with BRRZ: mid-solve real lookup reaches across to callee's definition env (pull). | TRUE | src/typing/rust_interop/declarations.rs:14 (contextual corroboration: instantiation-time resolution, not push) | |
| 8 | "Solution C" (`OverloadResolver.getPlaceholderImplBoundEnvs`) is cited as a positive pull instance. | DRIFTED | src/typing/overload_resolver.rs:680 (`get_placeholder_extra_call_envs`, not `getPlaceholderImplBoundEnvs`) | The actual current function performing this walk is `get_placeholder_extra_call_envs` (overload_resolver.rs:680), which walks placeholder param filters to collect interface envs — same idea, different name than what the doc cites. The doc's trailing note already says the cited name "exists nowhere in the Rust tree." |
| 9 | `conjure_impl_bounds_for_defining` writing a `where implements(..)` `Isa` into the declaring function's own near-env is NOT a push exception (per pillar 1). | TRUE | src/typing/function/function_compiler_solving_layer.rs:1083, called from function_compiler_solving_layer.rs:1023, infer_compiler.rs:153, struct_compiler_generic_args_layer.rs:569,706 | |

## Stale citation sites

None — all 12 known citation sites (re-verified via `grep -rn "@BDPFWDZ"`) point at code/comments that still match the arcana's description. The two file-name/line-number entries that moved slightly from the "known" list (`ECSIIOSZ.md:25/29`, `Generics.md:493`, `after_regions_integration_tests.rs:112`→108) are doc/comment sites, not load-bearing code drift.

## Uncited sites that embody the arcana

None found beyond what's already covered by the doc's own trailing VCOORD note (which itself functions as an uncited-but-self-aware follow-up). No additional push-style harvesting sites were found lacking an @BDPFWDZ comment.

## Suggested rewrite

Not required for a `minor-inaccuracies` verdict, but the fix is small and mechanical — replace the "Currently-cataloged push exceptions" bullet's `addRunedDataToNearEnv` with `add_runed_data_to_near_env`, and replace `OverloadResolver.getPlaceholderImplBoundEnvs` (in both the push-exception bullet and the "Interactions" paragraph) with `OverloadResolver::get_placeholder_extra_call_envs` — then delete the trailing "VCOORD: update this" section since its content would be folded into the body.
