# Accuracy report: Instantiation Bound Args Match Instantiation Bound Params (IBAMIBP)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:1799 ("# Instantiation Bound Args Match Instantiation Bound Params (IBAMIBP)")

## Verdict
The core idea holds: a function-definition type carries a permanent `instantiation_bound_params` field of type `InstantiationBoundArgumentsT`, distinct from the per-callsite instantiation bound arguments stored in the compiler outputs, and runes at each callsite are expected to line up with the runes in the function definition. The one factual claim in the section names a Scala-era type, `FunctionT`, that no longer exists under that name — the field now lives on `FunctionDefinitionT`. No code anywhere cites IBAMIBP.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "FunctionT has an InstantiationBoundArgumentsT instance named instantiationBoundParams." | DRIFTED | src/typing/ast/ast.rs:105-108 (`FunctionDefinitionT` has `pub instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>`) | Rename to `FunctionDefinitionT` / `instantiation_bound_params` (snake_case). |
| 2 | "This is similar to the callsite InstantiationBoundArgumentsT's that are inside the coutputs." | TRUE | src/typing/compiler_outputs.rs:41,101 (`CompilerOutputs` holds per-callsite `instantiation_bound_params` and `instantiation_name_to_bounds: HashMap<IdT, &InstantiationBoundArgumentsT>`); "coutputs" is still the live variable name for `CompilerOutputs` (e.g. src/typing/compiler.rs:277) | none |
| 3 | "The runes for each callsite will match up with the runes in the function definition." | TRUE (mechanism enforced) | src/typing/compiler_outputs.rs:207-270 `add_instantiation_bounds` asserts existing vs. new bounds match ("addInstantiationBounds: existing bounds != new bounds" at line 270) | none |

## Stale citation sites
None — no code cites IBAMIBP at all (see below).

## Uncited sites that embody the arcana
- src/typing/ast/ast.rs:105-108 — `FunctionDefinitionT.instantiation_bound_params`, the exact field the section describes.
- src/typing/compiler_outputs.rs:207-270 — `add_instantiation_bounds`, which asserts the match between callsite and definition-side instantiation bound args.
- src/typing/templata_compiler.rs:705 — call site invoking `coutputs.add_instantiation_bounds`.

## Suggested rewrite
FunctionDefinitionT has an InstantiationBoundArgumentsT instance named `instantiation_bound_params`. This is similar to the callsite InstantiationBoundArgumentsT's that are inside the coutputs (CompilerOutputs).

The runes for each callsite will match up with the runes in the function definition.
