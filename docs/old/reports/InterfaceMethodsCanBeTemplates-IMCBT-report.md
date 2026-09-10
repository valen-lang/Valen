# Accuracy report: Interface Methods Can Be Templates (IMCBT)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Compiler/Templates.md:86 ("# Interface Methods Can Be Templates (IMCBT)")

## Verdict
The core claim — an interface's virtual method can be generic over runes (e.g. `IFunction1<M, P1, R>`'s `__call`), with the runes solved from the call-site type when calling from outside, and resolved from a known instantiation when eagerly evaluating a specific override — is still true of the Rust compiler, and `IFunction1` and generic-virtual patterns still exist in `src/tests/ifunction/` and `src/tests/programs/genericvirtuals/`. But the doc's mechanism description ("look up M, P1, and R in the environment") is Scala-era phrasing for what the Rust typing pass now does via impl-based generic-parameter substitution in `src/typing/edge_compiler.rs` (e.g. line 428's handling of an abstract function's generic parameters not pinned by the impl's self-type) — not a literal environment lookup. The section is uncited from any live code or doc. One-line recommendation: migrate (into docs/arcana/), with the mechanism paragraph reworded to reference the current substitution-based implementation.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | `IFunction1<M, P1, R>`'s `__call` is templated over runes M, P1, R | TRUE | src/tests/ifunction/ifunction1/ifunction1.vale:2-3 | Current syntax drops `M` (mutability rune) but P1, R remain generic params on `__call`. |
| 2 | Calling `__call` from outside with a concrete `IFunction1<mut,int,int>` solves for M, P1, R | TRUE | src/tests/programs/genericvirtuals/templatedinterface.vale:6-19 (analogous MyIFunction1 pattern) | none |
| 3 | Eager evaluation of `__call` for an already-instantiated interface, looking up M, P1, R "in the environment" | DRIFTED | src/typing/edge_compiler.rs:428-433 | Rust pass resolves the abstract method's generic parameters via impl/edge substitution against the concrete self-type, not a generic environment-rune lookup; phrasing should say "substituted from the impl" rather than "looked up in the environment". |

## Stale citation sites
None — no code or doc cites IMCBT (grep -rn -w "IMCBT" across src/, Backend/, docs/ found only the doc's own header and the batch-run convo log).

## Uncited sites that embody the arcana
- src/typing/edge_compiler.rs:428 — comment on generic parameters of an abstract function not pinned by the impl's self-type, the modern analog of resolving M/P1/R.
- src/tests/ifunction/ifunction1/ifunction1.vale:2-3 — current `IFunction1<P1, R>` interface with generic virtual `__call`.
- src/tests/programs/genericvirtuals/templatedinterface.vale:6-19 — end-to-end test exercising a templated interface method (`MyIFunction1<P1,R>.go`).
- src/tests/programs/genericvirtuals/specializeinterface.vale:7-23 — related test, specializing a templated interface's generic method.

## Suggested rewrite
This interface is templated:

> interface IFunction1\<P1, R\> {
>
> fn \_\_call(virtual self &!IFunction1\<P1, R\>, p1 P1) R;
>
> }

P1 and R are generic parameters (runes).

This is because we might want to call `__call` from the outside with e.g. an `IFunction1<int, int>` and have the compiler solve for P1 and R from that concrete type.

So `__call` is a generic (templated) method.

Sometimes we eagerly evaluate a specific override outside of a virtual call — for example, right after instantiating an `IFunction1<int, int>`. In that case P1 and R are already known from the impl's self-type, and the typing pass substitutes them in directly (see `src/typing/edge_compiler.rs`'s handling of an abstract function's generic parameters that aren't pinned by the impl's self-type) rather than looking them up in a runtime environment.
