# Accuracy report: Incrementally Reluctantly Add Generic Placeholders (IRAGP)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:310 ("## Incrementally Reluctantly Add Generic Placeholders (IRAGP)")

## Verdict
The core claim — that placeholders are solved-for incrementally, one at a time with a solve in between each, to avoid premature conflicts — is TRUE and correctly, currently cited at src/typing/infer_compiler.rs:1201 in `incrementally_solve`'s loop. The `bork<T, Y>(a T) Y where T = Y` example is verified live in src/typing/test/compiler_solver_tests.rs:1786. However, the section's illustrative `Array<M, E>` example (`where M Mutability = mut, E Ref`, distinguishing mutable/immutable array constructors) no longer matches the actual builtin: src/builtins/resources/arrays.vale defines a single `func Array<E>(size int) []E` extern'd to `vale_runtime_sized_array_new`, with no `Mutability` parameter, no `= mut`/`= imm` defaults, and no separate mut/imm variants. `grep -rn "Mutability = mut"` across src/ returns nothing. This is a stale illustrative example, not a false claim about current mechanism.

## Claims
| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | Two array-making functions exist, one mut one imm, distinguished by `= mut`/`= imm` defaults on an `M Mutability` param | DRIFTED | src/builtins/resources/arrays.vale:16 (`func Array<E>(size int) []E`, single function, no Mutability param) | Current builtin has one `Array<E>` function with no region/mutability parameter; the mut/imm-distinguished-by-default-region example is stale. |
| 2 | When compiling functions, placeholders handed in immediately conflict with rules like the default-region ones, so we solve before filling in placeholders | UNVERIFIABLE (mechanism plausible but the specific triggering rule from claim 1 no longer exists) | — | — |
| 3 | We solve *between* adding placeholders too, else `bork<T,Y>(a T) Y where T = Y` fails because T's placeholder ≠ Y's placeholder | TRUE | src/typing/test/compiler_solver_tests.rs:1786 (identical source `func bork<T, Y>(a T) Y where T = Y { return ^a; }` present as a live test) | — |
| 4 | Placeholders are populated one at a time with a solve in between each | TRUE | src/typing/infer_compiler.rs:1201, `incrementally_solve` loop calling `self.r#continue(...)` and checking `solver_state.is_complete()` each iteration, explicitly commented "See IRAGP for why we have this incremental solving/placeholdering." | — |

## Stale citation sites
None — src/typing/infer_compiler.rs:1201 still correctly implements and cites the incremental solve/placeholder loop described by the section.

## Uncited sites that embody the arcana
- src/typing/macros/struct_constructor_macro.rs:69 — already references "an IRAGP test" in a comment, effectively citing it.

## Suggested rewrite
(minor-inaccuracies — rewrite of the example only, core paragraphs kept)

Replace the `Array<M, E>` code example with one reflecting the current builtin surface, e.g.:

> We used to have two functions for making arrays, one for mutable and one for immutable, distinguished by a `Mutability` generic parameter defaulting to `mut` or `imm`. That specific example has since been simplified away (today's `Array<E>` builtin has no such parameter), but the underlying problem it illustrated is still live: when compiling a function, the placeholders we hand in can immediately conflict with `where`-clause defaults or equalities. For that reason, we do some solving before filling in any placeholders.
