# Slab 14b — Complete

Second-wave placeholder type flesh-out. Typing-pass `_Phantom` types
reduced from **12** to **1** (the one remaining, `IRegionNameT`, is
deferred per the handoff plan because no Scala implementors exist yet).

`cargo check --lib` clean — **0 errors, 0 warnings**.

## What landed

### Group E — misc demotions

- **E1 `TookWeakRefOfNonWeakableError`** (`expression/expression_compiler.rs:55`)
  demoted from `pub struct Foo<'s, 't>(pub PhantomData<...>)` to
  `pub struct Foo;`. Scala anchor is `case class TookWeakRefOfNonWeakableError()
  extends Throwable`; no fields.
- **E2 `DefaultPrintyThing`** (`compiler.rs`) deleted. Only non-comment
  reference was the struct definition itself (the Scala `object
  DefaultPrintyThing.print` was already lifted to `Compiler::print` in a
  prior slab). Scala `/* */` anchor preserved.
- **E3 `IContainer`** (`templata/templata.rs:415`) dropped the unused
  `'t` lifetime parameter; no consumer needed it. `_Phantom` straggler
  removed.

### Group D — IRegionNameT

No change. Grep for `extends IRegionNameT` in `names/names.rs` returned
**0 matches**, so per Gotcha 11 the enum keeps its `_Phantom` variant as
a forward declaration.

### Group A — AST foundational types

- **A1 `CitizenDefinitionT`** (`ast/citizens.rs:23`): `_Phantom` →
  `Struct(&'t StructDefinitionT<'s, 't>)` + `Interface(&'t
  InterfaceDefinitionT<'s, 't>)`. Chose `&'t` wrapping per Gotcha 3 —
  the concrete structs are big (Vec-bearing), and downstream consumers
  in `compiler_outputs.rs` already return `&'t CitizenDefinitionT<'s,
  't>` so an outer ref is idiomatic.
- **A2 `IStructMemberT`** (`ast/citizens.rs:138`): by-value folding —
  `Normal(NormalStructMemberT<'s, 't>)` + `Variadic(VariadicStructMemberT<'s, 't>)`.
  The concrete structs are small (`IVarNameT + VariabilityT +
  IMemberTypeT` / `IVarNameT + PlaceholderTemplataT`).
- **A3 `IMemberTypeT`** (`ast/citizens.rs:178`): by-value —
  `Address(AddressMemberTypeT<'s, 't>)` + `Reference(ReferenceMemberTypeT<'s, 't>)`.
  Each variant carries a single `CoordT` — cheap.
- **A4/A5/A7 attributes + `ExternT` + `AbstractT`** (`ast/ast.rs`):
  - `ExternT<'s, 't>` → `ExternT<'s>` with real field `pub package_coord: PackageCoordinate<'s>`.
  - `IFunctionAttributeT<'s, 't>` → `IFunctionAttributeT<'s>` with
    `Extern(ExternT<'s>)` + `Pure` + `Additive` + `UserFunction`.
  - `ICitizenAttributeT<'s, 't>` → `ICitizenAttributeT<'s>` with
    `Extern(ExternT<'s>)` + `Sealed`.
  - `AbstractT<'s, 't>(PhantomData)` → `AbstractT;` (Scala is `case class
    AbstractT()` with no fields).
  - **Lifetime minimization (Gotcha 2)**: dropped `'t` from all three —
    none of their fields carry `'t`. Propagation touched five
    consumer sites: `FunctionHeaderT.attributes`,
    `FunctionHeaderT::new(attributes)`, `translate_attributes`,
    `translate_function_attributes`, `StructDefinitionT.attributes`,
    `InterfaceDefinitionT.attributes`, `translate_citizen_attributes`,
    and `evaluate_maybe_virtuality`'s return type (`AbstractT`). All
    compile.
  - `ExternT` is referenced from both attribute enums per Gotcha 1 —
    one struct definition, two enum references.
- **A6 `ICalleeCandidate`** (`ast/ast.rs:433`): three variants
  with mixed ref/value per Gotcha 5:
  - `Function(FunctionCalleeCandidate<'s, 't>)` — holds `FunctionTemplataT` (Copy).
  - `Header(&'t HeaderCalleeCandidate<'s, 't>)` — `&'t` because
    `FunctionHeaderT` is Vec-heavy (not Copy).
  - `PrototypeTemplata(PrototypeTemplataCalleeCandidate<'s, 't>)` —
    holds `PrototypeT` (Copy).

### Group B — function-compilation result types

All three enum/Success/Failure triples in `function/function_compiler.rs`
folded following the existing `IEvaluateFunctionResult` precedent (variant
names match wrapped struct names):

- **B1 `IDefineFunctionResult`** + `DefineFunctionSuccess` (fields:
  `&'t PrototypeTemplataT`, `HashMap<IRuneS, ITemplataT>`, `&'t
  InstantiationBoundArgumentsT`) + `DefineFunctionFailure` (field: `IDefiningError`).
- **B2 `IResolveFunctionResult`** + `ResolveFunctionSuccess` (`&'t
  PrototypeTemplataT`, `HashMap<IRuneS, ITemplataT>`) +
  `ResolveFunctionFailure` (`IResolvingError`).
- **B3 `IStampFunctionResult`**: `StampFunctionSuccess` was already
  filled by Slab 14a. Added `StampFunctionFailure` (field:
  `IFindFunctionFailureReason`, fully qualified
  `crate::typing::overload_resolver::IFindFunctionFailureReason`) and the
  enum variants.

### Group C — ITypingPassSolverError (28 variants)

Pre-flight grep for `extends ITypingPassSolverError` in
`infer/compiler_solver.rs` found **28 variants** — significantly more
than the ~16 the audit suggested (Gotcha 6). All 28 added in
file-top-to-bottom order, named-field syntax:

`KindIsNotConcrete`, `KindIsNotInterface`, `KindIsNotStruct`,
`CouldntFindFunction`, `CouldntFindImpl`, `CouldntResolveKind`,
`CantShareMutable`, `CantSharePlaceholder`, `BadIsaSubKind`,
`BadIsaSuperKind`, `SendingNonCitizen`, `CantCheckPlaceholder`,
`ReceivingDifferentOwnerships`, `SendingNonIdenticalKinds`,
`NoCommonAncestors`, `LookupFailed`, `NoAncestorsSatisfyCall`,
`CantDetermineNarrowestKind`, `OwnershipDidntMatch`,
`CallResultWasntExpectedType`, `CallResultIsntCallable`, `OneOfFailed`,
`IsaFailed`, `WrongNumberOfTemplateArgs`, `FunctionDoesntHaveName`,
`CantGetComponentsOfPlaceholderPrototype`, `ReturnTypeConflict`,
`InternalSolverError`.

**Recursion fix**: three variants required `&'t` wrapping to break an
infinite-size cycle (`ITypingPassSolverError → ResolveFailure →
IResolvingError → FailedSolve → ISolverError → RuleError →
ITypingPassSolverError`):

- `CouldntFindImpl { fail: &'t IsntParent<'s, 't> }`
- `CouldntResolveKind { rf: &'t ResolveFailure<'s, 't, KindT<'s, 't>> }`
- `InternalSolverError { err: &'t ISolverError<IRuneS<'s>,
  ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>> }`

Other large-but-non-recursive payloads (`FindFunctionFailure`) stay
by-value — the enum itself isn't Copy, which matches the
`IsParentResult` precedent. Three imports added:
`FindFunctionFailure`, `IsntParent`, `ResolveFailure`.

## What I did NOT touch

- **HinputsT lookup bodies** — Group F was marked optional; skipped per
  Gotcha 12.
- **Panic messages** — no `Slab 15 → Slab 16` renames, no touching of
  stale `Slab 10` panics. (Gotcha 15).
- **Scala `/* */` blocks** — unchanged byte-for-byte.
- **Types outside the Slab 14b inventory** — untouched per Gotcha 16.

## File touch summary (Slab 14b edits only)

10 files:
1. `src/typing/ast/ast.rs` — Group A4/A5/A6/A7 + lifetime propagation.
2. `src/typing/ast/citizens.rs` — Group A1/A2/A3 + attribute propagation.
3. `src/typing/citizen/struct_compiler_core.rs` — attribute propagation.
4. `src/typing/compiler.rs` — E2 `DefaultPrintyThing` removal.
5. `src/typing/expression/expression_compiler.rs` — E1 demotion.
6. `src/typing/function/function_compiler.rs` — Group B.
7. `src/typing/function/function_compiler_core.rs` — attribute propagation.
8. `src/typing/function/function_compiler_middle_layer.rs` — AbstractT propagation.
9. `src/typing/infer/compiler_solver.rs` — Group C (enum + 3 imports).
10. `src/typing/templata/templata.rs` — E3 `IContainer` `'t` drop.

## Verification

```
cargo check --lib
0 errors
0 warnings
Finished in 1.10s
```

`_Phantom` count in `src/typing/`: baseline 12 → final 1 (IRegionNameT,
deferred per Gotcha 11).

## Handed back uncommitted.
