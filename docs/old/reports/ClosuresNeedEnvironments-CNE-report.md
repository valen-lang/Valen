# Accuracy report: Closures Need Environments (CNE)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Environments, Closures, Overload Sets.md:60 ("# Closures Need Environments")

## Verdict
The core argument — that a closure's compiled form must carry (at least indirectly) the environment it was created in, because a lambda can be handed out of its lexical scope and later called from somewhere that has no access to the names it uses (e.g. `doublePrint`) — is still true of the Rust typing pass: closures are compiled with `__call` bound methods and carry environment/capture data (src/typing/function/function_compiler_closure_or_light_layer.rs, src/typing/env/function_environment_t.rs, src/typing/env/environment.rs), and `infer_compiler.rs:419` still talks about "the closure's env for `func __call(&Lam)T`". However every code example in the section is Scala-era syntax that no longer parses: `fn huzzah(f: #F)`, `#X` generic runes, `\_` placeholder lambdas, `= { ... };` return syntax, and the literal name `__Closure:main:lam1` are all gone from the current Vale syntax (current syntax uses `func`, ordinary generic parameter lists, and different closure-struct naming). The reasoning survives; the illustration doesn't. Recommendation: migrate the core argument into docs/arcana/ but rewrite the examples in current Vale syntax before doing so — as written it would mislead a reader into thinking this is current syntax.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | A closure body evaluating to just a `__Closure` struct plus a `FunctionS`, or sticking a `__call` into the enclosing environment and returning a plain struct kind, both lose the definition-site environment and must be rejected. | TRUE (mechanism) | src/typing/function/function_compiler_closure_or_light_layer.rs; src/typing/env/function_environment_t.rs | — |
| 2 | Therefore a closure must return something that carries its environment (at least indirectly), demonstrated via `namespace FlamingJustice { fn doublePrint... }` example. | TRUE (core idea), example DRIFTED | src/typing/infer_compiler.rs:419 references "the closure's env for `func __call(&Lam)T`" | Example uses obsolete `fn`/`#X`/`\_` syntax; would need rewriting to `func`/current generics/lambda syntax. |
| 3 | Example syntax: `fn huzzah(f: #F)`, `huzzah({ print(\_); })`, `fn doublePrint:#X(x: #X)`, `= { doublePrint(\_); };` | FALSE as current syntax | Current tests use `func name<T>(...)`, e.g. src/typing/test/compiler_solver_tests.rs:329 `func __call<F Prot = func(P1)R>(self &Functor1<F>, param1 P1) R` | Rewrite examples in current `func`/generic-parameter syntax and current lambda syntax. |
| 4 | The compiled closure struct is named `__Closure:main:lam1` | DRIFTED | grep shows no `__Closure:` naming pattern in src/typing; only `__call` survives (src/typing/test/compiler_lambda_tests.rs:339, src/typing/test/after_regions_tests.rs:173) | Naming convention for closure structs/functions has changed; update or drop the specific name in the example. |

## Stale citation sites
None — no live code cites CNE.

## Uncited sites that embody the arcana
- src/typing/function/function_compiler_closure_or_light_layer.rs — compiles closures, deciding light vs. environment-carrying (closure) form; this is the modern implementation of the "must carry environment" conclusion.
- src/typing/env/function_environment_t.rs — the function environment type threaded through so closures/templated functions can resolve names from their defining scope.
- src/typing/env/environment.rs — general environment machinery referenced by closures and generic-context name resolution (the "Structs Also Need Environments" half of the doc).
- src/typing/infer_compiler.rs:419 — comment referencing "the closure's env for `func __call(&Lam)T`", i.e. the exact CNE mechanism, uncited.
- src/typing/test/compiler_lambda_tests.rs — tests exercising lambda/`__call` compilation, the current-syntax analogue of the doc's examples.

## Suggested rewrite
Not written — this is minor-inaccuracies (core mechanism intact), and full rewrite of every example is a larger authoring task better done during actual migration to docs/arcana/, not inline here.
