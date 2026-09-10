# Accuracy report: Representing Overload Sets (ROS)

Audited against the working tree on 2026-09-06. Source: `docs/old/Environments, Closures, Overload Sets.md:449-534`

## Verdict
This is a Scala-era design musing that poses two problems (what templata does an overload-set name-load produce; what does passing an overload set into a generic rune mean) and proposes a solution: represent an overload set as a kind carrying `(env, name)`, plus an optimization where a direct function call short-circuits through the overload set rather than materializing it generically. The current Rust code implements exactly this proposal, unchanged in shape: `OverloadSetT { env: IInDenizenEnvironmentT, name: &IImpreciseNameS }` in src/typing/types/types.rs:387-390, and the "look for GlobalLoadSE"-style optimization is implemented as the `IExpressionSE::OverloadSet` match arm inside `FunctionCall` handling in src/typing/expression/expression_compiler.rs:661-662. The rejected `voidStructRef` alternative was correctly never adopted (no such field exists). Every claim in the doc is TRUE against current code, just phrased as a forward-looking proposal rather than a description. Recommendation: migrate into docs/arcana as a proper Z-suffix doc — the mechanism is core, unchanged, uncited from code, and worth a short "why" arcana entry (e.g. attached to `OverloadSetT` and to the `FunctionCall`/`OverloadSet` match arm).

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Calling an overload set (e.g. `print(true)`) requires the name-load to produce a templata that "simultaneously represents all the print functions in scope." | TRUE | src/postparsing/expressions.rs:328 (`OverloadSetSE`), src/typing/types/types.rs:387-390 (`OverloadSetT`) | — |
| 2 | Passing an overload-set name into a generic rune (e.g. `forEach(seq, print)`) poses the question of what the rune binds to. | TRUE | src/typing/overload_resolver.rs:217-218 — `KindT::OverloadSet(_)` case exists but is still `panic!("implement: get_candidate_banners_inner OverloadSet")`, i.e. this exact scenario (an overload set flowing into a generic-rune resolution slot) remains unimplemented today, same as the doc's open question. | Solution direction (kind carrying env+name) exists; full resolution-into-rune path is still a stub. |
| 3 | Solution: represent an overload set as a kind holding `env: LocalEnvironment, name: String`. | TRUE | src/typing/types/types.rs:387-390: `OverloadSetT { env: IInDenizenEnvironmentT<'s,'t>, name: &'s IImpreciseNameS<'s>, .. }` | field names differ (env/name renamed to Rust types) but shape identical |
| 4 | Alternative: also carry a `voidStructRef` to give the kind "something tangible underneath." | TRUE (as an unadopted alternative) | grep for `voidStructRef`/`void_struct` in src/typing/types/types.rs returns nothing | Correctly not adopted; doc itself frames it as optional |
| 5 | Optimization: `FunctionCallSE`'s handling can specifically look for the overload-set case and evaluate arguments first, then use them to resolve the overload. | TRUE | src/typing/expression/expression_compiler.rs:660-662 (`IExpressionSE::FunctionCall(fc) => match fc.callable_expr { IExpressionSE::OverloadSet(overload_set) => { ... evaluate args ... }`); also src/typing/expression/call_compiler.rs:51 (`KindT::OverloadSet(overload_set) =>`) for the already-evaluated-kind path | — |

## Stale citation sites
None — `sites` is empty; the doc has no live code citations to check.

## Uncited sites that embody the arcana
- src/typing/types/types.rs:387-390 — `OverloadSetT { env, name }`, the literal `[Name, Environment] Kind` from the doc.
- src/typing/expression/expression_compiler.rs:661-662 — `FunctionCall` matching `IExpressionSE::OverloadSet(overload_set)` directly, the "Optimization for Calling Ordinary Overloads."
- src/typing/expression/call_compiler.rs:51-58 — resolves a call whose callable's kind is `KindT::OverloadSet`, using `overload_set.env` and `overload_set.name` to look up the function.
- src/postparsing/expressions.rs:328 — `OverloadSetSE<'s>`, the postparsing-stage precursor that carries the imprecise-name lookup into typing.
- src/typing/overload_resolver.rs:217-218 — the still-unresolved "pass overload set into a rune" case (`panic!("implement: ...")`), directly matching POFIR's open question.

## Suggested text
Not applicable per instructions (Suggested text is required for D3 kind only when verdict is major-inaccuracies or obsolete; this is minor drift at most / effectively accurate).
