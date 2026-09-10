# Accuracy report: Each Step Needs Template Args (ESNTA)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Compiler/Namespaces.md:1 ("## Each Step Needs Template Args")

## Verdict
The core claim — that a name (`FullName`/now `IdT`) is a chain of steps, and each step must carry its own template args (not just the last step) to disambiguate monomorphized nested definitions such as lambda-backed structs — is still true and load-bearing in the current Rust typing pass (`src/typing/names/names.rs`). Only the literal code shown (Scala `case class NamePart(...)`, `FullName(List(NamePart(...)))`) is stale syntax that no longer exists verbatim; the mechanism it illustrates is real. One-line recommendation: migrate (rewrite the Scala snippets as Rust `IdT`/`INameT` shapes, keep the myFunc<T> collision example).

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | A `FullName` is a list of `NamePart`, each carrying an optional `templateArgs` | DRIFTED | src/typing/names/names.rs:20-27 (`IdT` has `init_steps: &[INameT]`, `local_name: INameT`); src/typing/names/names.rs:456-480 (`template_args()` match over `IInstantiationNameT` variants) | Renamed: `FullName`→`IdT`, `NamePart`→`INameT` (per-variant structs like `FunctionNameT`, `LambdaCallFunctionNameT` each carry their own `template_args` field rather than a single `Option[List[ITemplata]]` on a generic `NamePart`). |
| 2 | Without per-step template args, two instantiations of a generic function's inner lambda (e.g. `myFunc<Int>`'s lambda vs `myFunc<Bool>`'s lambda) would both mangle to the same struct name (`main:lam1`), causing an LLVM name collision | TRUE (still the real problem the mechanism solves) | src/typing/names/names.rs:1404 (`LambdaCallFunctionNameT`), src/typing/names/names.rs:284-345 (`make_function_name` threading `template_args` through nested name construction) | None — mechanism intact, just under new type names. |
| 3 | The fix is to put `templateArgs` on every step of the full name, not just the end, e.g. `FullName(List(NamePart("main", Some(List(CoordTemplata(Int)))), NamePart("lam1", None)))` | DRIFTED (concept true, literal syntax obsolete) | src/typing/names/names.rs:20-27, :456-480 | No Rust code constructs an `IdT` via this literal Scala call shape; the equivalent today is building an `IdT` whose `init_steps` includes a `FunctionNameT` (or similar) carrying `template_args: &[ITemplataT]` for the instantiated `T=Int`, followed by the `LambdaCallFunctionNameT` step. |

## Stale citation sites
None — no doc or code cites ESNTA (see below).

## Uncited sites that embody the arcana
- src/typing/names/names.rs:20-27 — `IdT` struct: `init_steps: &[INameT]` plus `local_name: INameT`, the direct successor of `FullName`/`NamePart`.
- src/typing/names/names.rs:456-480 — `template_args()` accessor showing every instantiation-name variant (Function, Struct, Interface, Impl, LambdaCallFunction, ExternFunction, etc.) carries its own `template_args`.
- src/typing/names/names.rs:1404 — `LambdaCallFunctionNameT` struct, the Rust analog of the `lam1` step in the doc's example.
- src/typing/names/names.rs:284-345 — `make_function_name`, which threads `template_args` down through nested name construction (the mechanism that prevents the collision described in the doc).

## Suggested rewrite
Keep the doc's structure and `myFunc<T>` example, but replace the Scala code blocks:

> An `IdT` is made of `init_steps: &[INameT]` plus a final `local_name: INameT`. Many of the concrete `INameT` variants (e.g. `FunctionNameT`, `LambdaCallFunctionNameT`, `StructNameT`) carry their own `template_args: &[ITemplataT]`.
>
> ...
>
> So, we must disambiguate them. Instead of the full name being just `"main:lam1"`, every step that was itself generic must carry its own template args. That's why `template_args` lives on the per-step name structs (`FunctionNameT`, etc.) and not only on the final step — e.g. the `main` step carries `template_args: [Int]` when instantiated for `myFunc<Int>`, giving distinct `IdT`s for the two lambdas even though their `local_name` (`lam1`) is identical.
