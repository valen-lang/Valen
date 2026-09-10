# Accuracy report: Exact/Non-Exact Call Candidate Lookup (ENECCLZ)

Audited against the working tree on 2026-09-06. Arcana doc: docs/arcana/ExactNonExactCallCandidateLookup-ENECCLZ.md

## Verdict
The core mechanism is accurate and every citation site still does what the doc describes: `exact: bool` is threaded through `get_param_environments` (src/typing/overload_resolver.rs:643) and `params_match` (src/typing/overload_resolver.rs:97), with the same peel-vs-keep-whole and equality-vs-convertibility split the doc describes. The one factual error is line 3's claim that overload lookup runs through two entry points, `find_function` and `find_potential_function` — only `find_function` exists in the codebase. This is a small, easily-fixed inaccuracy, not a change to the underlying idea, so the verdict is minor-inaccuracies.

## Claims
| # | Claim (quoted or closely paraphrased from the doc) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "Overload lookup (`find_function`, `find_potential_function`) runs in one of two modes, chosen by an `exact: bool`" | DRIFTED | src/typing/overload_resolver.rs:730 (find_function exists); no `find_potential_function` anywhere in src/ | Only `find_function` exists; drop `find_potential_function` from the claim. |
| 2 | `exact: bool` "reaches both `get_param_environments` and `params_match`" | TRUE | src/typing/overload_resolver.rs:643 (`get_param_environments(..., exact: bool)`), src/typing/overload_resolver.rs:97 (`params_match(..., exact: bool)`) | |
| 3 | Non-exact: "peels the argument's references to pick which namespace to search... a `&Ship` argument means searching `Ship`'s environment" | TRUE | src/typing/overload_resolver.rs:659-661 (`peel_all_references(*tyype)` when `!exact`) | |
| 4 | Non-exact "also considers subtypes/supertypes, so calling something on a &Ship should also look in IFlying's namespaces" | TRUE (via separate helper) | src/typing/overload_resolver.rs:679-724 `get_placeholder_extra_call_envs`, tagged `@BDPFWDZ` not `@ENECCLZ`, but invoked alongside non-exact lookup to add supertype/interface envs | Minor: doc doesn't mention this lives in a differently-cited helper; not a correctness issue. |
| 5 | Exact: "keeps the type whole, so a `&Ship` searches the borrow reference's environment (`borrow.vale`)... matches parameters by equality, with no coercion" | TRUE | src/typing/overload_resolver.rs:650-657 (keeps `*tyype` whole when `exact`), src/typing/overload_resolver.rs:116-124 (`desired_param != candidate_param` exact equality check) | |
| 6 | Bound example: `T=Ship` searches `Ship`'s env and finds hand-written `clone(&Ship)Ship`; `T=&Ship` searches `&Ship`'s env / `borrow.vale` and finds blanket `clone<T>(&&T)&T` | TRUE (as described conceptually) | src/typing/infer_compiler.rs:33-35 (doc comment restates this exact example, citing @ENECCLZ) | `ship.vale`/`borrow.vale` are illustrative file names used consistently by both the arcana and the code comment, not literal files in the repo — fine as an example, not literally verifiable as a file. |
| 7 | "Virtual dispatch also needs it, because vtables can't do any casting, they need... the *exact* function" | TRUE (consistent with comment) | src/typing/overload_resolver.rs:116 ("exact is used for looking for functions to satisfy bounds, looking for things in vtables, etc.") | |
| 8 | "Both of these are handled by `exact: bool`... unsure [if it will be split into two booleans]" | TRUE, still current | src/typing/overload_resolver.rs:97, :643, :730 all take a single `exact: bool` | Doc's own "VCOORD: is this true still?" self-flag is answered yes — still one bool. |

## Stale citation sites
None. All 7 code citation sites (src/typing/infer/compiler_solver.rs:598, src/typing/infer_compiler.rs:33, :418, :1011, :1018, src/typing/overload_resolver.rs:116, :650) match current surrounding code and correctly restate the doc's exact/non-exact split and the "search the rune's resolved-value env, not the raw param coord" refinement documented in infer_compiler.rs:33 and infer/compiler_solver.rs:598 (labeled there as "decision 3", not otherwise mentioned in the arcana doc itself — a gap, not a contradiction).

## Uncited sites that embody the arcana
None found. `get_param_environments` and `params_match` are only defined and called from src/typing/overload_resolver.rs and src/typing/infer_compiler.rs, and every call site there already carries an `@ENECCLZ`/`ENECCLZ` comment.
