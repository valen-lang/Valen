# Handoff: Typing Pass Slab 14 — Placeholder Types & Traits Flesh-Out (pre-body-migration prep)

## Who this is for

You're picking up an in-progress Rust port of a Scala compiler frontend. A senior engineer has been moving through the typing pass one "slab" at a time per `quest.md` §12. Slabs 0-13 are done:

- **Slabs 0-7** — arena substrate, names, types, templatas, envs, AST, `CompilerOutputs` data shape, `HinputsT`/`Compiler` scaffolding.
- **Slabs 8-13** — every method signature across the typing pass (~210 sigs total) lifted into one-fn `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>` blocks with real parameter/return types and `panic!()` bodies.

**The signature-rewrite phase is complete.** `cargo check --lib` is clean (0 errors, 0 warnings). 0 bare `pub fn foo(&self) { panic!(...) }` stubs remain in `src/typing/`. 136 `panic!("Unimplemented: Slab 14 — body migration")` bodies await migration.

But before body migration can begin in earnest, **a cluster of ~15 placeholder types and 2 marker traits must be fleshed out** — body code that returns `ICompileErrorT` variants can't be written until those variants exist; body code that pattern-matches `IBoundArgumentsSource` can't compile until it's an enum; and so on.

**Slab 14 is a prep slab** that fills these placeholders so Slab 15+ body migration has solid types to stand on. The shape is different from Slabs 8-13:
- Not a signature lift — no `(&self)` stubs to convert.
- Per-type, not per-file. You're filling enum variants and struct fields from the Scala `/* */` blocks that already sit beside each placeholder.
- Two trait-to-enum conversions (the most novel work).
- One new trait/struct definition (`IPlaceholderSubstituter`).
- `HinputsT::new()` constructor + ~10 lookup method body migrations (the only "real bodies" in this slab — kept minimal because lookups are one-liners).

Budget **~5-6 hours focused** — this is the largest slab in the migration so far because of `ICompileErrorT`'s 55 variants alone.

After Slab 14, body migration begins as **Slab 15+** (test-driven, per-method, different shape per quest.md §12.1).

**Read these first in this order**, then come back:

1. `FrontendRust/docs/migration/handoff-slab-9.md` — for the IDEPFL "Val/Ref" interning pattern and the `&'t` arena conventions; you'll be writing dozens of new variants that must follow these conventions.
2. `FrontendRust/docs/migration/handoff-slab-12.md` — Gotchas 14-16 cover the placeholder types you'll be fleshing out (`IsParentResult`, `IDefiningError`, `IResolvingError`).
3. `TL-HANDOFF.md` at repo root — file-layout standards + the Slabs 8-13 panic-message slab-number drift note (the existing 136 `Slab 14` panic messages get bulk-updated to `Slab 15` at the end of this slab — see Step 8).
4. `Luz/shields/RustShouldMirrorScalaAsCloseAsPossible-RSMSCPX.md` and `ArenaAllocatedStructsShouldNotContainMallocdCollections-AASSNCMCX.md` — shield reminders. Variant fields with `Vector[X]` become `&'t [X<'s, 't>]` per AASSNCMCX, not `Vec<X>`.
5. This doc.

You shouldn't need to read the Scala source externally — every type's Scala source is already embedded in `/* */` blocks beside its Rust placeholder.

---

## The big picture: why Slab 14 exists

The signature-rewrite slabs (8-13) deliberately punted on type definitions. When `solve_for_defining` returns `Result<CompleteDefineSolve, IDefiningError>`, those types existed only as empty placeholders (`pub struct CompleteDefineSolve;` and `pub enum IDefiningError {}`). That was fine for signatures because the bodies were `panic!()` and never constructed instances.

Body migration changes that. The first body that says `return Err(DefiningSolveFailedOrIncomplete(failed))` requires `IDefiningError` to actually have a `DefiningSolveFailedOrIncomplete(FailedSolve<...>)` variant. The first body that pattern-matches `IBoundArgumentsSource` requires it to be an enum, not an empty marker trait.

Rather than scatter type-flesh-out across hundreds of body-migration commits — each "I needed `ICompileErrorT::CouldntNarrowDownCandidates` so I added it" commit interleaved with body work — Slab 14 does it all upfront. After Slab 14, every type in the typing pass is shape-complete. Body migration becomes purely about porting the body bodies.

By the end of Slab 14:

- All ~15 placeholder types/structs have real Scala-parity fields/variants.
- `IBoundArgumentsSource` is a 2-variant Copy enum (replacing the marker trait).
- `IFunctionGenerator` is a Copy dispatch-tag enum (mirroring `FunctionBodyMacro` precedent).
- `IPlaceholderSubstituter` is defined (as a struct/enum — see Gotcha 9).
- `HinputsT::new()` exists; ~10 `HinputsT` lookup methods have real one-liner bodies.
- The 3 sub-compiler methods (`get_placeholder_substituter`, `get_placeholder_substituter_ext`, `create_rune_type_solver_env`) that returned `()` with TODO get their real return types flipped in.
- The 136 stale `Slab 14 — body migration` panic messages are bulk-updated to `Slab 15 — body migration`.
- `cargo check --lib` clean (0 errors, 0 warnings).
- Scala `/* */` blocks unchanged byte-for-byte.

**What Slab 14 is NOT:**

- No body migration of sub-compiler methods (other than the `HinputsT` lookup one-liners). The 136 `panic!()` bodies stay `panic!()` — Slab 15+ migrates them.
- No new sub-compiler design decisions.
- No touching of name/type/templata/AST types that are already real.
- No `_phantom: PhantomData<...>` field additions to types that don't already use them.
- No optimization, refactoring, or "while we're here" cleanups.

---

## Inventory: 15 placeholder types + 3 traits + HinputsT plumbing

### Group A: Error-and-result enums (5 types — biggest chunk by line count)

#### A1. `ICompileErrorT<'s, 't>` — 55 variants

**Location**: `src/typing/compiler_error_reporter.rs:27`. Currently:

```rust
pub enum ICompileErrorT<'s, 't> {
    _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
```

**Scala source**: 55 `case class ... extends ICompileErrorT` blocks throughout `compiler_error_reporter.rs` (lines 34-339). Each is in its own `/* */` anchor.

**Work**: convert to a real enum with 55 variants. **Drop the `_Phantom` placeholder** once the first real variant lands (variants ensure the lifetimes are used).

**Translation rules** (apply per variant):

| Scala field type | Rust field type |
|---|---|
| `range: List[RangeS]` | `range: &[RangeS<'s>]` |
| `RangeS` | `RangeS<'s>` (Copy) |
| `INameT` / `IRuneS` / `IVarNameT` | by value (Copy per Slabs 2/4) |
| `INameS` / `IImpreciseNameS` | `INameS<'s>` / `IImpreciseNameS<'s>` (Copy) |
| `IdT[X]` | `IdT<'s, 't>` by value |
| `CoordT` / `KindT` / `ICitizenTT` | by value (Copy per Slab 3) |
| `ITemplataT[X]` | `ITemplataT<'s, 't>` by value (Copy) |
| `String` | `&'s str` (scout-arena str) — verify this is the convention; check existing Rust scout types for precedent |
| `StructTT` / `InterfaceTT` | by value (Copy) |
| `RuneTypeSolveError` | `RuneTypeSolveError<'s>` by value (already exists at `src/postparsing/rune_type_solver.rs:27`) |
| `Vector[X]` | `&'t [X<'s, 't>]` per AASSNCMCX |
| `Iterable[(A, B)]` | `&'t [(A<'s, 't>, B<'s, 't>)]` |

Variant name translation: Scala `CamelCase(field: Type, ...)` → Rust `CamelCase { field: type, ... }`. **Keep the variant names verbatim** (no snake_case conversion — variants are Scala's case-class names; matches `ICompileErrorT::CouldntNarrowDownCandidates { ... }`).

**Example variant lift**:

Scala (line 34):
```scala
case class CouldntNarrowDownCandidates(range: List[RangeS], candidates: Vector[RangeS]) extends ICompileErrorT
```

Rust:
```rust
pub enum ICompileErrorT<'s, 't> {
    CouldntNarrowDownCandidates {
        range: &'t [RangeS<'s>],
        candidates: &'t [RangeS<'s>],
    },
    // ... 54 more variants
}
```

**Don't add helper constructors or impl blocks**. The variants stand alone; body migration constructs them inline.

#### A2. `IDefiningError<'s, 't>` — 2 variants

**Location**: `src/typing/infer_compiler.rs:92`. Currently `pub enum IDefiningError {}`.

**Scala variants** (anchored at the same file):
- `DefiningSolveFailedOrIncomplete(inner: FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError])`
- `DefiningResolveConclusionError(inner: IConclusionResolveError)`

**Work**: add `<'s, 't>` lifetime to the enum (current declaration has none — needed because the variants reference `'s, 't` types). Add 2 variants:

```rust
pub enum IDefiningError<'s, 't> {
    DefiningSolveFailedOrIncomplete(FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>),
    DefiningResolveConclusionError(IConclusionResolveError<'s, 't>),
}
```

**Note**: Slab 12 sigs that returned `Result<CompleteDefineSolve, IDefiningError>` (no lifetime) will now need to use `Result<CompleteDefineSolve, IDefiningError<'s, 't>>`. Do an audit after the change — `cargo check` will flag the call sites.

#### A3. `IResolvingError<'s, 't>` — 2 variants (placeholder variants exist; flesh them out)

**Location**: `src/typing/infer_compiler.rs:62`. Currently has 2 placeholder variants — replace with real ones from Scala:
- `ResolvingSolveFailedOrIncomplete(inner: FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError])`
- `ResolvingResolveConclusionError(inner: IConclusionResolveError)`

#### A4. `IConclusionResolveError<'s, 't>` — 1 variant (also `_Phantom` — user's audit missed this)

**Location**: `src/typing/infer_compiler.rs:60`. Currently `_Phantom`.

**Scala variant**: `CouldntFindImplForConclusionResolve(range: List[RangeS], fail: IsntParent)`

```rust
pub enum IConclusionResolveError<'s, 't> {
    CouldntFindImplForConclusionResolve {
        range: &'t [RangeS<'s>],
        fail: IsntParent<'s, 't>,  // see A5
    },
}
```

#### A5. `IsParentResult<'s, 't>` + `IsParent<'s, 't>` + `IsntParent<'s, 't>`

**Location**: `src/typing/citizen/impl_compiler.rs:46-66`. The two concrete structs (`IsParent`, `IsntParent`) already exist at lines 52 and 64 with real fields. Fold them into the enum:

Currently:
```rust
pub enum IsParentResult { _Phantom }
pub struct IsParent<'s, 't> { templata: ..., conclusions: ..., impl_id: ... }
pub struct IsntParent<'s, 't> { candidates: ... }
```

After:
```rust
pub enum IsParentResult<'s, 't> {
    IsParent(IsParent<'s, 't>),     // or &'t IsParent<'s, 't> — see Gotcha 1
    IsntParent(IsntParent<'s, 't>), // or &'t IsntParent<'s, 't>
}
```

**Gotcha 1 decision**: by-value vs by-ref. The structs hold `HashMap` and `Vec` which aren't Copy — pass by value (move). Keep both `IsParent` and `IsntParent` structs as separate types so `IConclusionResolveError::CouldntFindImplForConclusionResolve.fail: IsntParent<'s, 't>` can reference one of them directly. Don't inline the fields into the enum variants — keep the named structs.

Add `<'s, 't>` lifetimes to `IsParentResult`. Audit Slab-12 `is_parent` return type — was `IsParentResult` (no lifetimes); needs `IsParentResult<'s, 't>` now.

#### A6. `IFindFunctionFailureReason<'s, 't>` — 11 variants (also `_Phantom`)

**Location**: `src/typing/overload_resolver.rs:59`. Currently `_Phantom`.

**Scala variants** (lines 65-117 of the same file):
1. `WrongNumberOfArguments(supplied: Int, expected: Int)`
2. `WrongNumberOfTemplateArguments(supplied: Int, expected: Int)`
3. `SpecificParamDoesntSend(index: Int, argument: CoordT, parameter: CoordT)`
4. `SpecificParamDoesntMatchExactly(index: Int, argument: CoordT, parameter: CoordT)`
5. `SpecificParamRegionDoesntMatch(rune: IRuneS, suppliedMutability: IRegionMutabilityS, calleeMutability: IRegionMutabilityS)`
6. `SpecificParamVirtualityDoesntMatch(index: Int)`
7. `Outscored()`
8. `RuleTypeSolveFailure(reason: RuneTypeSolveError)`
9. `InferFailure(reason: FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError])`
10. `FindFunctionResolveFailure(reason: IResolvingError)`
11. `CouldntEvaluateTemplateError(reason: IDefiningError)`

Standard rules apply. `IRegionMutabilityS` lives in scout-side — grep `pub enum IRegionMutabilityS` if unsure of lifetime parameterization.

#### A7. `IResolveOutcome<'s, 't, T>` — 2 variants (currently `_Phantom`)

**Location**: `src/typing/citizen/struct_compiler.rs:119`. Currently `_Phantom`.

**Scala**: 
```scala
sealed trait IResolveOutcome[+T <: KindT] {
  def expect(): ResolveSuccess[T]
}
case class ResolveSuccess[+T <: KindT](kind: T) extends IResolveOutcome[T]
case class ResolveFailure[+T <: KindT](range: List[RangeS], x: IResolvingError) extends IResolveOutcome[T]
```

Rust:
```rust
pub enum IResolveOutcome<'s, 't, T> {
    Success(ResolveSuccess<'s, 't, T>),
    Failure(ResolveFailure<'s, 't, T>),
}
```

`ResolveSuccess<'s, 't, T>` and `ResolveFailure<'s, 't, T>` exist at lines 131 and 147 — verify they have real fields (`kind: T` and `range`/`x` respectively). The `ResolveFailure.x: IResolvingError` becomes `IResolvingError<'s, 't>`.

The Scala `+T <: KindT` covariance constraint is dropped in Rust (no variance keyword); just `<T>`.

### Group B: Solver-state placeholder structs (4 types)

#### B1. `InferEnv<'s>` — needs `<'s, 't>` lifetimes + 5 fields

**Location**: `src/typing/infer_compiler.rs:103`. Currently `pub struct InferEnv<'s>(pub std::marker::PhantomData<&'s ()>)`.

**Scala**:
```scala
case class InferEnv(
  originalCallingEnv: IInDenizenEnvironmentT,
  parentRanges: List[RangeS],
  callLocation: LocationInDenizen,
  selfEnv: IInDenizenEnvironmentT,
  region: RegionT
)
```

Rust:
```rust
pub struct InferEnv<'s, 't> {
    pub original_calling_env: &'t IInDenizenEnvironmentT<'s, 't>,
    pub parent_ranges: &'t [RangeS<'s>],
    pub call_location: LocationInDenizen<'s>,
    pub self_env: &'t IInDenizenEnvironmentT<'s, 't>,
    pub region: RegionT,
}
```

**This needs `<'s, 't>` (not just `<'s>`)**. Slab 12 used `InferEnv<'s>` per the placeholder; every Slab-12 sig that took `envs: InferEnv<'s>` will need to flip to `envs: InferEnv<'s, 't>`. Bulk audit after the change.

**Should `InferEnv` be Copy?** No — it's not Copy (`&'t [RangeS<'s>]` slice ref is Copy, but consistency with prior practice for envs prefers passing structs like this by value or `&InferEnv`). Default: pass by value (move). Slab 12 sigs use `envs: InferEnv<'s>` by value; that pattern continues with the bigger struct.

#### B2. `InitialKnown` and `InitialSend` — fields per Scala

**Location**: `src/typing/infer_compiler.rs:131` (`InitialKnown`), `:123` (`InitialSend`). Currently empty unit structs.

**Scala** (find them in the InferCompiler.scala source — likely near the top of `infer_compiler.rs`'s Scala block):
```scala
case class InitialKnown(rune: RuneUsage, templata: ITemplataT[ITemplataType])
case class InitialSend(senderRune: RuneUsage, receiverRune: RuneUsage, sendTemplata: ITemplataT[ITemplataType])
```

Rust:
```rust
pub struct InitialKnown<'s, 't> {
    pub rune: RuneUsage<'s>,
    pub templata: ITemplataT<'s, 't>,
}

pub struct InitialSend<'s, 't> {
    pub sender_rune: RuneUsage<'s>,
    pub receiver_rune: RuneUsage<'s>,
    pub send_templata: ITemplataT<'s, 't>,
}
```

**`RuneUsage<'s>`** is scout-side, Copy — verify. Adds `<'s, 't>` lifetimes; bulk-audit Slab 12 call sites.

#### B3. `CompleteDefineSolve<'s, 't>` and `CompleteResolveSolve<'s, 't>`

**Location**: `src/typing/infer_compiler.rs:53` and `:45`. Currently empty unit structs.

**Scala**:
```scala
case class CompleteDefineSolve(
    conclusions: Map[IRuneS, ITemplataT[ITemplataType]],
    runeToBound: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT])

case class CompleteResolveSolve(
    conclusions: Map[IRuneS, ITemplataT[ITemplataType]],
    runeToFunctionBound: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT])
```

Rust:
```rust
pub struct CompleteDefineSolve<'s, 't> {
    pub conclusions: HashMap<IRuneS<'s>, ITemplataT<'s, 't>>,
    pub rune_to_bound: &'t InstantiationBoundArgumentsT<'s, 't>,
}

pub struct CompleteResolveSolve<'s, 't> {
    pub conclusions: HashMap<IRuneS<'s>, ITemplataT<'s, 't>>,
    pub rune_to_function_bound: &'t InstantiationBoundArgumentsT<'s, 't>,
}
```

`InstantiationBoundArgumentsT` is monomorphic (Slab 7 flip). Bulk-audit Slab 12 call sites for `<'s, 't>` propagation.

### Group C: Result-payload placeholder structs (3 types)

#### C1. `StampFunctionSuccess<'s, 't>`

**Location**: `src/typing/function/function_compiler.rs:167`. Currently `_Phantom`.

**Scala**:
```scala
case class StampFunctionSuccess(
  prototype: PrototypeT[IFunctionNameT],
  inferences: Map[IRuneS, ITemplataT[ITemplataType]]
)
```

Rust:
```rust
pub struct StampFunctionSuccess<'s, 't> {
    pub prototype: &'t PrototypeT<'s, 't>,
    pub inferences: HashMap<IRuneS<'s>, ITemplataT<'s, 't>>,
}
```

`PrototypeT<'s, 't>` is monomorphic (Slab 3); the `[IFunctionNameT]` phantom is erased.

#### C2. `EvaluateFunctionSuccess<'s, 't>` and `EvaluateFunctionFailure<'s, 't>`

**Location**: `src/typing/function/function_compiler.rs:97` and `:106`. Currently `_Phantom` structs.

**Scala**:
```scala
case class EvaluateFunctionSuccess(
    prototype: PrototypeTemplataT[IFunctionNameT],
    inferences: Map[IRuneS, ITemplataT[ITemplataType]],
    instantiationBoundArgs: InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]
) extends IEvaluateFunctionResult

case class EvaluateFunctionFailure(reason: IDefiningError) extends IEvaluateFunctionResult
```

Rust:
```rust
pub struct EvaluateFunctionSuccess<'s, 't> {
    pub prototype: &'t PrototypeTemplataT<'s, 't>,
    pub inferences: HashMap<IRuneS<'s>, ITemplataT<'s, 't>>,
    pub instantiation_bound_args: &'t InstantiationBoundArgumentsT<'s, 't>,
}

pub struct EvaluateFunctionFailure<'s, 't> {
    pub reason: IDefiningError<'s, 't>,
}
```

#### C3. `IEvaluateFunctionResult<'s, 't>` — 2 variants (fold in C2)

**Location**: `src/typing/function/function_compiler.rs:79`. Currently `_Phantom`.

```rust
pub enum IEvaluateFunctionResult<'s, 't> {
    Success(EvaluateFunctionSuccess<'s, 't>),
    Failure(EvaluateFunctionFailure<'s, 't>),
}
```

Likely also exists `EvaluateFunctionFailure2` somewhere as a separate Scala class — check `overload_resolver.rs:135` (`pub struct EvaluateFunctionFailure2;`). If it has a Scala block, fill its fields too; otherwise leave as `_Phantom` and flag.

#### C4. `FindFunctionFailure<'s, 't>`

**Location**: `src/typing/overload_resolver.rs:121`. Currently `_Phantom` struct.

**Scala**:
```scala
case class FindFunctionFailure(
    name: IImpreciseNameS,
    args: Vector[CoordT],
    rejectedCalleeToReason: Iterable[(ICalleeCandidate, IFindFunctionFailureReason)]
)
```

Rust:
```rust
pub struct FindFunctionFailure<'s, 't> {
    pub name: IImpreciseNameS<'s>,
    pub args: &'t [CoordT<'s, 't>],
    pub rejected_callee_to_reason: &'t [(ICalleeCandidate<'s, 't>, IFindFunctionFailureReason<'s, 't>)],
}
```

**`ICalleeCandidate<'s, 't>`** — grep first; if it doesn't exist, define a stub (`pub enum ICalleeCandidate<'s, 't> { _Phantom(...) }` with a `// TODO: Slab 15 — flesh out variants` comment). Don't go down a rabbit hole adding new transitive types unless absolutely required for compilation.

### Group D: Trait-to-enum conversions (2 types — most novel work)

#### D1. `IBoundArgumentsSource` — convert trait to 2-variant enum

**Location**: `src/typing/templata_compiler.rs:44-60`. Currently:
```rust
pub trait IBoundArgumentsSource<'s, 't> {}
pub struct InheritBoundsFromTypeItself;
pub struct UseBoundsFromContainer;
impl<'s, 't> IBoundArgumentsSource<'s, 't> for InheritBoundsFromTypeItself {}
impl<'s, 't> IBoundArgumentsSource<'s, 't> for UseBoundsFromContainer {}
```

**Scala** (in `/* */` blocks):
```scala
sealed trait IBoundArgumentsSource
case object InheritBoundsFromTypeItself extends IBoundArgumentsSource
case class UseBoundsFromContainer(
  instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT],
  instantiationBoundArguments: InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]
) extends IBoundArgumentsSource
```

**Convert to enum**:
```rust
pub enum IBoundArgumentsSource<'s, 't> {
    InheritBoundsFromTypeItself,
    UseBoundsFromContainer {
        instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
        instantiation_bound_arguments: &'t InstantiationBoundArgumentsT<'s, 't>,
    },
}
```

**Delete** the empty `pub struct InheritBoundsFromTypeItself;`, `pub struct UseBoundsFromContainer;`, and the two `impl IBoundArgumentsSource for ...` blocks.

**Audit downstream**: every Slab-9 `bound_arguments_source: &'t dyn IBoundArgumentsSource<'s, 't>` parameter (the 11 substitute_* methods + a couple of struct_compiler/templata_compiler helpers) must flip to `bound_arguments_source: IBoundArgumentsSource<'s, 't>` (by value — it's Copy if both variants are Copy; check) or `&IBoundArgumentsSource<'s, 't>` (by ref if not Copy because of the slice references).

The variant payload `&'t InstantiationBoundArgumentsT<'s, 't>` is Copy (it's a ref); `IBoundArgumentsSource<'s, 't>` should be `#[derive(Copy, Clone)]`-able. Use Copy + by-value.

**This is the second-largest blast radius in Slab 14** (after `ICompileErrorT`). Expect to update ~13 sub-compiler signatures.

#### D2. `IFunctionGenerator` — convert trait to dispatch-tag enum

**Location**: `src/typing/compiler.rs:65`. Currently a trait with a `generate` method whose params are all `()` placeholders.

**Scala**:
```scala
trait IFunctionGenerator {
  def generate(
    functionCompilerCore: FunctionCompilerCore,
    structCompiler: StructCompiler,
    destructorCompiler: DestructorCompiler,
    arrayCompiler: ArrayCompiler,
    env: FunctionEnvironmentT,
    coutputs: CompilerOutputs,
    life: LocationInFunctionEnvironmentT,
    callRange: List[RangeS],
    originFunction: Option[FunctionA],
    paramCoords: Vector[ParameterT],
    maybeRetCoord: Option[CoordT]):
  FunctionHeaderT
}
```

**Implementors in Scala** (grep `extends IFunctionGenerator` in `Frontend/`):
- `StructConstructorMacro`
- `StructDropMacro`
- `InterfaceDropMacro`
- `RSADropIntoMacro`
- `RSAImmutableNewMacro`
- `RSALenMacro`
- `RSAMutableCapacityMacro`
- `RSAMutableNewMacro`
- `RSAMutablePopMacro`
- `RSAMutablePushMacro`
- `SSADropIntoMacro`
- `SSALenMacro`
- `LockWeakMacro`
- `SameInstanceMacro`
- `AsSubtypeMacro`
- `AbstractBodyMacro`
- `AnonymousInterfaceMacro` (maybe)

**These overlap with `FunctionBodyMacro`'s 15 variants** at `src/typing/macros/macros.rs:23` — most are the same. Verify by grepping the Scala source for `extends IFunctionGenerator`.

**Convert to dispatch-tag enum** following the `FunctionBodyMacro` precedent:

```rust
// Dispatch-tag enum replacing Scala's IFunctionGenerator trait;
// bodies live as Compiler::generate_function_<suffix> methods (Slab 15+ defines them).
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IFunctionGenerator {
    StructConstructor,
    StructDrop,
    InterfaceDrop,
    RsaDropInto,
    RsaImmutableNew,
    RsaLen,
    RsaMutableCapacity,
    RsaMutableNew,
    RsaMutablePop,
    RsaMutablePush,
    SsaDropInto,
    SsaLen,
    LockWeak,
    SameInstance,
    AsSubtype,
    AbstractBody,
}
```

**Delete** the trait and its `generate` method declaration. Audit any Rust references to `IFunctionGenerator` (Slab 13's `evaluate_*` family used `&()` placeholders per Slab 13 Gotcha 3 — flip those to `IFunctionGenerator` by value).

**Question for the TL**: does `IFunctionGenerator` have a different implementor set from `FunctionBodyMacro`? If so, consider whether the two enums should merge. Default decision per `FunctionBodyMacro`'s existing Scala-anchor comment: keep them separate (`IFunctionGenerator` for the high-level "produce a function header" hook; `FunctionBodyMacro` for the lower-level "produce a function body" hook). Verify against Scala.

### Group E: Trait/struct definition (1 type)

#### E1. `IPlaceholderSubstituter` — currently doesn't exist

**Location**: should be added to `src/typing/templata_compiler.rs` near the existing `IBoundArgumentsSource` (after that one is converted to enum).

**Scala**:
```scala
trait IPlaceholderSubstituter {
  def substituteForCoord(coutputs: CompilerOutputs, coord: CoordT): CoordT
  def substituteForKind(coutputs: CompilerOutputs, kind: KindT): KindT
  def substituteForTemplata(coutputs: CompilerOutputs, templata: ITemplataT[ITemplataType]): ITemplataT[ITemplataType]
  // ... possibly more
}
```

**Decision**: trait or struct?

- **Option A: trait with `&dyn` returns** — matches Scala 1:1, but body migration must define implementors.
- **Option B: struct with substitution context fields + methods** — closer to how Rust typically handles "configured substitution". Body migration calls methods.

**Recommendation**: define as a **struct** with the substitution-context fields and inherent `substitute_for_coord` / `substitute_for_kind` / `substitute_for_templata` methods. The Scala trait has only one implementor (`PlaceholderSubstituter` — the concrete instantiation returned from `getPlaceholderSubstituter`). Single-implementor traits in Rust are usually struct-with-methods.

```rust
pub struct IPlaceholderSubstituter<'s, 't> {
    // Fields per the Scala impl (grep `class PlaceholderSubstituter`):
    // sanity_check, original_calling_denizen_id, needle_template_name,
    // new_substituting_templatas, bound_arguments_source, etc.
    // Or just hold these as fields of the substituter context.
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>, // until fields land
}

impl<'s, 't> IPlaceholderSubstituter<'s, 't> {
    pub fn substitute_for_coord(...) -> CoordT<'s, 't> { panic!("Slab 15 — body migration"); }
    pub fn substitute_for_kind(...) -> KindT<'s, 't> { panic!("Slab 15 — body migration"); }
    pub fn substitute_for_templata(...) -> ITemplataT<'s, 't> { panic!("Slab 15 — body migration"); }
}
```

**Then flip the 3 sub-compiler return types** (currently `-> ()` with TODO):
- `Compiler::get_placeholder_substituter` → `IPlaceholderSubstituter<'s, 't>`
- `Compiler::get_placeholder_substituter_ext` → `IPlaceholderSubstituter<'s, 't>`
- (Keep `create_rune_type_solver_env` returning `()` for now if `IRuneTypeSolverEnv` flesh-out is out of scope; it's not in the user's audit.)

**If the senior wants Option A (trait)**, flag and skip Step E1; drive a design conversation. Default to Option B (struct).

### Group F: `HinputsT` plumbing

#### F1. `HinputsT::new()` constructor

**Location**: `src/typing/hinputs_t.rs`. The `HinputsT<'s, 't>` struct has 11 fields (`structs`, `interfaces`, `functions`, etc., all `Vec`/`HashMap`). Add:

```rust
impl<'s, 't> HinputsT<'s, 't> {
    pub fn new() -> Self {
        HinputsT {
            structs: Vec::new(),
            interfaces: Vec::new(),
            functions: Vec::new(),
            interface_to_edge_blueprints: HashMap::new(),
            interface_to_sub_citizen_to_edge: HashMap::new(),
            instantiation_name_to_instantiation_bounds: HashMap::new(),
            kind_exports: Vec::new(),
            function_exports: Vec::new(),
            kind_externs: Vec::new(),
            function_externs: Vec::new(),
            sub_citizen_to_interface_to_edge: HashMap::new(),
        }
    }
}
```

Use the actual field names from `hinputs_t.rs:91`. Some may have HashMap key types that need `PtrKey<'t, ...>` wrapping per Slab 6 conventions — match the existing field declarations.

#### F2. ~10 `HinputsT` lookup methods — real one-liner bodies

**Location**: `src/typing/hinputs_t.rs`. Read the file's existing `panic!()` stubs. Each lookup is a one-line `self.field.get(&key)` or `self.field.iter().find(...)`. Match Scala bodies inline.

**Body-migration scope clarification**: Slab 14 includes these because they're trivial one-liners and unblock Slab 15 from immediately needing `HinputsT` materialization. If a method's body is more than ~3 lines, leave it as `panic!("Unimplemented: Slab 15 — body migration")` and flag.

Specific methods (verify against the file):
- `lookup_struct(struct_tt)` — `self.structs.iter().find(|s| s.ref_t == struct_tt)`
- `lookup_interface(interface_tt)`
- `lookup_function(signature)`
- `lookup_edge(struct_tt, interface_tt)`
- `find_imm_destructor(kind)`
- `get_all_structs()` / `get_all_interfaces()` / `get_all_functions()` — return slice refs
- `find_function_by_signature(...)`
- ... read the file to enumerate

---

## Translation rules (cross-cutting)

| Scala | Rust |
|---|---|
| `case class Foo(field: Type, ...)` | `Foo { field: type, ... }` enum-variant-style (or named-field struct if standalone) |
| `case object Foo extends Trait` | unit variant `Foo,` in the converted enum |
| `Vector[X]` field in a variant | `&'t [X<'s, 't>]` per AASSNCMCX |
| `Map[K, V]` field | `HashMap<K, V>` (owned; AASSNCMCX deviation already documented for HinputsT/CompilerOutputs) |
| `Iterable[(K, V)]` field | `&'t [(K, V)]` per AASSNCMCX |
| `Option[X]` | `Option<X>` |
| `String` | `&'s str` (scout-arena) — verify by checking how existing scout error variants handle strings |
| `Int` / `Boolean` | `i32` / `bool` |
| `RangeS` | `RangeS<'s>` by value |
| `INameS` / `IRuneS` / `IImpreciseNameS` / `IVarNameS` | by value (Copy, scout) |
| `INameT` / `IVarNameT` / `IdT[X]` / `KindT` / `CoordT` / `ICitizenTT` / `ITemplataT[X]` | by value (Copy, typing) |
| `StructTT` / `InterfaceTT` | by value (Copy) |
| `IInDenizenEnvironmentT` / `FunctionEnvironmentT` / `NodeEnvironmentT` | `&'t X<'s, 't>` |
| `StructDefinitionT` / `InterfaceDefinitionT` / `FunctionDefinitionT` / `ImplT` / `EdgeT` | `&'t X<'s, 't>` |
| `PrototypeT` / `SignatureT` | `&'t X<'s, 't>` |
| `RuneTypeSolveError` | `RuneTypeSolveError<'s>` by value |
| `FailedSolve[Rule, Rune, Conclusion, ErrType]` | `FailedSolve<Rule, Rune, Conclusion, ErrType>` no lifetime params |
| `IConclusionResolveError` / `IDefiningError` / `IResolvingError` / `IFindFunctionFailureReason` / `IsParentResult` / `ICompileErrorT` | enum types — pass by value (move) |
| `InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT]` | `&'t InstantiationBoundArgumentsT<'s, 't>` (phantoms erased per Slab 7) |
| `case class Foo(...) extends Iface` (sealed-trait variant) | enum variant inside `Iface`, NOT a separate struct — use named-field variant syntax |
| Variant field that references self-recursively (e.g. `IRecur(IRecur)`) | `Box<IRecur<'s, 't>>` to break the cycle (rare; check first) |

**Variant naming**: keep PascalCase Scala names verbatim. `CouldntNarrowDownCandidates` not `couldnt_narrow_down_candidates`. (Field names within variants get snake_case as usual: `range`, `candidates`, etc.)

---

## Gotchas

### Gotcha 1: `IsParent` / `IsntParent` stay as standalone structs; fold via enum variants holding the structs

Don't inline `IsParent`'s 3 fields into the enum variant:

```rust
// WRONG — inlining loses the struct identity
pub enum IsParentResult<'s, 't> {
    IsParent { templata: ..., conclusions: ..., impl_id: ... },
    IsntParent { candidates: ... },
}

// RIGHT — keep structs; wrap in variants
pub enum IsParentResult<'s, 't> {
    IsParent(IsParent<'s, 't>),
    IsntParent(IsntParent<'s, 't>),
}
```

This matters because `IConclusionResolveError::CouldntFindImplForConclusionResolve.fail: IsntParent<'s, 't>` directly references `IsntParent` as a struct.

### Gotcha 2: `<'s, 't>` lifetime additions trigger downstream signature audits

These types currently have `<>` or `<'s>` and need `<'s, 't>` after Slab 14:
- `IDefiningError` (was `<>`)
- `InferEnv` (was `<'s>`)
- `InitialKnown` / `InitialSend` (were `<>`)
- `CompleteDefineSolve` / `CompleteResolveSolve` (were `<>`)
- `IsParentResult` (was `<>`)

After flipping each, run `cargo check --lib` and let the compiler tell you which Slab-12/Slab-13 signatures need their `<'s, 't>` propagation. Each fix is mechanical — add `<'s, 't>` to the call site or propagate through the parent function's lifetime params.

Don't preemptively change call sites. Let the compiler drive.

### Gotcha 3: `IBoundArgumentsSource` flip — change `&'t dyn IBoundArgumentsSource<'s, 't>` to `IBoundArgumentsSource<'s, 't>` everywhere

Slab 9 used `&'t dyn IBoundArgumentsSource<'s, 't>` in 11 `substitute_*` method sigs (templata_compiler.rs) plus a couple of struct_compiler.rs sigs. After D1, these all become `IBoundArgumentsSource<'s, 't>` by value (Copy enum). Drop the `&'t dyn`.

Audit:
```
Grep pattern: "dyn IBoundArgumentsSource"
```

Should match 0 lines after Slab 14.

### Gotcha 4: `IFunctionGenerator` flip — replace `&()` placeholders with `IFunctionGenerator` by value

Slab 13's `evaluate_*` family in function_compiler.rs used `&()` placeholders for the `generator` param (per Slab 13 Gotcha 3). Flip to `IFunctionGenerator` by value:

```
Grep pattern: "generator: &\(\)"
```

Should match 0 lines after Slab 14.

### Gotcha 5: `ICompileErrorT`'s 55 variants — sweep file-top-to-bottom, one Scala block at a time

Don't try to do all 55 at once. The file is 339 lines with each `case class` Scala block clearly anchored. Lift in pass:
1. Read the next `case class ... extends ICompileErrorT` block.
2. Add the Rust variant inside the `ICompileErrorT` enum.
3. Move on.

Keep the `_Phantom` placeholder until you've added the first real variant, then delete it.

If a variant's Scala references a type that doesn't exist on the Rust side yet (e.g. `LocationInDenizen` field referencing some unported Scala type), grep first; if missing, add a `// TODO: Slab 15 — Foo type missing` comment and use `()` placeholder for that field, OR define a minimal placeholder for the missing type. Default: use `()` placeholder + TODO; defining transitive types isn't Slab 14 scope.

### Gotcha 6: `_Phantom` deletion — keep until variants land, then remove

For any enum being fleshed out, keep `_Phantom(std::marker::PhantomData<(&'s (), &'t ())>)` until you've added at least one real variant that uses both `'s` and `'t`. Then delete the `_Phantom` variant. Compiler errors will guide you.

For structs being fleshed out (`InferEnv`, `StampFunctionSuccess`, etc.), the `_phantom` field is replaced wholesale by the new fields. No transition needed.

### Gotcha 7: `String` fields in error variants — use `&'s str`, not `&str` or `String`

Scala's `String` in error case classes (e.g. `ImmStructCantHaveVaryingMember.memberName: String`) must become `&'s str` (scout-arena allocated). Don't use `String` — that allocates on the heap and breaks AASSNCMCX. Don't use bare `&str` — that introduces an unnamed lifetime that fights with `'s`.

If you find no Rust precedent for arena-allocated strings, grep `&'s str` in `src/postparsing/` or `src/typing/`. The convention should already exist.

### Gotcha 8: `HinputsT::new()` vs `Default::default()` — pick `new()`

Scala uses `new HinputsT(...)` constructors. Rust convention is `pub fn new() -> Self`. **Don't** add `impl Default for HinputsT` — that's a Rust idiom that diverges from the Scala-parity goal.

### Gotcha 9: `IPlaceholderSubstituter` design decision — confirm with senior

**This is the only Slab 14 decision that truly needs senior input** (everything else is mechanical translation). Two reasonable approaches:

- Option A: trait + dyn dispatch
- Option B: struct + inherent methods (recommended in §E1)

**Default to Option B** unless flagged otherwise. The Scala trait has one implementor (`PlaceholderSubstituter` from `getPlaceholderSubstituter`); single-implementor traits in Rust idiomatically become structs.

**If you're unsure**, code Option B and flag in the hand-back; the senior can convert to Option A in a follow-up if they prefer.

### Gotcha 10: bulk-update the 136 `Slab 14 — body migration` panic messages to `Slab 15`

After all type/trait flesh-out is done (and `cargo check --lib` is clean), do a bulk find-and-replace:

```
Find: panic!("Unimplemented: Slab 14 — body migration");
Replace: panic!("Unimplemented: Slab 15 — body migration");
```

Across `src/typing/`. Verify count matches (~136 occurrences). Recheck `cargo check --lib` afterward.

This is a one-shot mechanical update — don't do it incrementally.

### Gotcha 11: Scala `/* */` blocks are still frozen

Same as every prior slab. Hook enforces. Don't edit Scala source.

### Gotcha 12: don't add `#[derive(Copy, Clone)]` to enums with non-Copy variant payloads

`ICompileErrorT` has variants holding `&'t [...]` slices (which are Copy). But it also has variants potentially holding `RuneTypeSolveError<'s>` (verify Copy), or `FailedSolve<...>` (likely NOT Copy because of internal allocations).

**Don't blanket-derive Copy/Clone on the big error enums.** Check field types per variant; if any field is non-Copy, don't add Copy. Default: `#[derive(Debug)]` only on the big error enums; Slab 15+ adds Clone or PartialEq if/when needed.

Same for `FindFunctionFailure`, `IFindFunctionFailureReason` — likely Debug only.

For trait-replacement enums (`IBoundArgumentsSource`, `IFunctionGenerator`), they're small and pure data: `#[derive(Copy, Clone, Debug, PartialEq, Eq)]` like the existing `FunctionBodyMacro` precedent.

### Gotcha 13: `HinputsT` lookup method bodies — keep to one-liners

Slab 14 includes `HinputsT` lookup body migrations because they're trivial. If a method requires a non-trivial body (e.g. `find_imm_destructor` may scan multiple collections), leave it as `panic!("Unimplemented: Slab 15 — body migration")` and flag in the handback. **Don't expand scope.**

Acceptable Slab-14 body shapes:
- `self.field.iter().find(|x| x.id == query)` (one-line scan)
- `self.field.get(&query).copied()` (one-line HashMap lookup)
- `&self.field[..]` (slice ref)

Anything more complex → defer to Slab 15.

### Gotcha 14: don't fix the 17 files of stale `Slab 10` panic messages

Slab 8/9 left ~89 panic messages saying `"Unimplemented: Slab 10 — body migration"` (the audit-stale text per quest.md §12.1 item 9). Slab 14 only updates the `Slab 14` messages (added in Slabs 10-13). Leave the `Slab 10` ones alone — they're a separate cleanup passes through with the body migration of those specific methods.

If you accidentally bulk-update `Slab 10` → `Slab 15` too, that's wrong. Be precise:

```
Find: "Unimplemented: Slab 14 — body migration"
Replace: "Unimplemented: Slab 15 — body migration"
```

Not `Slab \d+`. Exact `Slab 14` only.

---

## Step-by-step plan

### Step 1: Confirm starting point

```bash
cargo check --lib --manifest-path /Volumes/V/Sylvan/FrontendRust/Cargo.toml > /tmp/sylvan-slab-14.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-14.txt
```

Must print `0`.

### Step 2: Pre-flight inventory

Re-grep to confirm counts:

```
Grep pattern: "_Phantom\(std::marker::PhantomData" path: FrontendRust/src/typing
Grep pattern: "pub enum \w+ \{\}" path: FrontendRust/src/typing
Grep pattern: "pub struct \w+;" path: FrontendRust/src/typing  (only the placeholder ones from the inventory, ignore others)
Grep pattern: "pub trait IBoundArgumentsSource" path: FrontendRust/src/typing
Grep pattern: "pub trait IFunctionGenerator" path: FrontendRust/src/typing
Grep pattern: "extends ICompileErrorT" path: FrontendRust/src/typing  (Scala variants — should be 55)
Grep pattern: "extends IFindFunctionFailureReason" path: FrontendRust/src/typing  (Scala variants — should be 11)
```

Record numbers; you'll re-verify at the end.

### Step 3: Order of operations

Recommended sequence (smallest blast radius → largest):

1. **Group A small enums (A2-A7, ~20 variants total, 60-90 min)**
   - `IDefiningError` (2 variants)
   - `IResolvingError` (2 variants)
   - `IConclusionResolveError` (1 variant)
   - `IsParentResult` (2 variants — fold in existing structs)
   - `IFindFunctionFailureReason` (11 variants)
   - `IResolveOutcome` (2 variants)

2. **Group B solver-state structs (B1-B3, 60 min)**
   - `InferEnv<'s, 't>`
   - `InitialKnown` / `InitialSend`
   - `CompleteDefineSolve` / `CompleteResolveSolve`

3. **Group C result-payload structs (C1-C4, 60 min)**
   - `StampFunctionSuccess`
   - `EvaluateFunctionSuccess` / `EvaluateFunctionFailure` / `IEvaluateFunctionResult`
   - `FindFunctionFailure`

4. **Group D trait-to-enum conversions (D1-D2, 60-90 min)**
   - `IBoundArgumentsSource` → enum + `&'t dyn` audit + flip
   - `IFunctionGenerator` → enum + `&()` placeholder audit + flip

5. **Group E `IPlaceholderSubstituter` definition (30 min)**
   - Define struct with PhantomData
   - Add 3 inherent panic-stub methods
   - Flip 2 `Compiler::get_placeholder_substituter*` returns

6. **Group F `HinputsT` plumbing (45 min)**
   - `new()` constructor
   - One-line lookup method bodies (whatever is trivially possible)

7. **Group A1: `ICompileErrorT` 55 variants (90-120 min — the biggest single chunk)**
   - Lift file-top-to-bottom one Scala block at a time
   - Run `cargo check --lib` after every ~10 variants

8. **Step 8 mechanical: bulk-update `Slab 14` → `Slab 15` panic messages**

After each group, run `cargo check --lib` and fix any downstream signature lifetime propagation.

### Step 4: Incremental verification

After each group:

```bash
cargo check --lib --manifest-path /Volumes/V/Sylvan/FrontendRust/Cargo.toml > /tmp/sylvan-slab-14.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-14.txt
```

Common errors:
- "expected 1 type argument, found 0" — call site needs `<'s, 't>` after a placeholder type gained lifetimes (Gotcha 2).
- "the trait `IBoundArgumentsSource` is not object-safe" — you forgot to delete the trait after creating the enum (Gotcha 3).
- "cannot find type `EvaluateFunctionSuccess` (with 0 fields) in this scope" — leftover `_Phantom` PhantomData syntax in struct decl. Replace whole struct.
- "field `prototype` of struct `StampFunctionSuccess` is private" — you wrote `prototype:` instead of `pub prototype:`. Slab convention: `pub` on every field.
- "use of moved value" — you treated an enum as Copy when it wasn't. Drop `#[derive(Copy)]` (Gotcha 12).

### Step 5: Final verification

```bash
cargo check --lib --manifest-path /Volumes/V/Sylvan/FrontendRust/Cargo.toml > /tmp/sylvan-slab-14.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-14.txt       # must be 0
grep -c "^warning" /tmp/sylvan-slab-14.txt     # must be 0
tail -3 /tmp/sylvan-slab-14.txt                 # must show "Finished"
```

Sanity greps:

```
Grep pattern: "_Phantom\(std::marker::PhantomData" path: FrontendRust/src/typing
# Should be drastically reduced. Acceptable remainders: types not in the Slab 14 inventory.

Grep pattern: "pub trait IBoundArgumentsSource" path: FrontendRust/src/typing
# Must match 0.

Grep pattern: "pub trait IFunctionGenerator" path: FrontendRust/src/typing
# Must match 0.

Grep pattern: "dyn IBoundArgumentsSource" path: FrontendRust/src/typing
# Must match 0.

Grep pattern: "generator: &\(\)" path: FrontendRust/src/typing
# Must match 0.

Grep pattern: "Slab 14 — body migration" path: FrontendRust/src/typing
# Must match 0.

Grep pattern: "Slab 15 — body migration" path: FrontendRust/src/typing
# Should match ~136 (the bulk-renamed panics).
```

### Step 6: Diff self-review

```bash
git diff FrontendRust/src/typing/ | head -500
git diff --stat
```

Confirm:
- ~10-15 typing files changed (the placeholder hosts + downstream sig audits).
- No Scala `/* */` block edits.
- `compiler_error_reporter.rs` got the bulk of variant additions.
- `IBoundArgumentsSource` / `IFunctionGenerator` traits gone; replaced by enums.
- `IPlaceholderSubstituter` new struct exists.
- `HinputsT::new()` + lookup method bodies present.
- ~136 `Slab 14` → `Slab 15` panic-message updates.

### Step 7: Hand off

**Never commit.** Hand back uncommitted; the human tags `slab-14-complete` and starts the body-migration phase.

Work-order checkpoints:
- Step 2 — pre-flight inventory recorded.
- Step 3 (1/7) — Group A small enums done.
- Step 3 (2/7) — Group B solver structs done.
- Step 3 (3/7) — Group C result structs done.
- Step 3 (4/7) — Group D trait-to-enum conversions done.
- Step 3 (5/7) — Group E `IPlaceholderSubstituter` defined.
- Step 3 (6/7) — Group F `HinputsT` plumbing done.
- Step 3 (7/7) — `ICompileErrorT` 55 variants done.
- Step 8 — `Slab 14` → `Slab 15` bulk rename done.
- Step 5 — verification greps pass.
- Step 6 — diff self-review.
- Step 7 — hand back.

---

## What "done" looks like

- `cargo check --lib` passes with 0 errors, 0 warnings.
- Every type in the Slab 14 inventory has real Scala-parity fields/variants.
- `IBoundArgumentsSource` and `IFunctionGenerator` are Copy dispatch-tag enums; their traits deleted; downstream signatures flipped.
- `IPlaceholderSubstituter` exists as a struct (Option B per Gotcha 9) with 3 panic-stub inherent methods. The 2 `Compiler::get_placeholder_substituter*` returns flipped from `()` to `IPlaceholderSubstituter<'s, 't>`.
- `HinputsT::new()` constructor exists. Trivial one-liner lookup methods on `HinputsT` have real bodies; non-trivial ones stay `panic!("Slab 15 — body migration")` and are flagged.
- All 136 `Slab 14 — body migration` panic messages bulk-renamed to `Slab 15 — body migration`. Stale `Slab 10` messages (89 of them) untouched.
- Scala `/* */` blocks unchanged byte-for-byte.
- Only the necessary files touched (Slab 14 typing-pass files; no postparsing/scout/parsing changes).
- Handed back uncommitted.

---

## When you're stuck

- **"`ICompileErrorT` is huge, I'm losing momentum"** — split into 5-variant batches. Do 11 batches. After each batch, `cargo check --lib` and self-rest.
- **"a variant references a type that doesn't exist"** — use `()` placeholder + `// TODO: Slab 15 — Foo type missing` comment. Don't go down the rabbit hole.
- **"`String` field — should I use `String` or `&'s str`?"** — `&'s str`. Gotcha 7.
- **"`IPlaceholderSubstituter` — trait or struct?"** — struct (Option B). Gotcha 9. Flag if you go with Option A.
- **"the bulk `Slab 14` → `Slab 15` rename is scary"** — it's purely cosmetic; bodies stay `panic!()`. Test with a single-file rename first; verify `cargo check --lib` is still clean; then expand.
- **"`IsParent` / `IsntParent` structs — re-derive Clone/Hash on them?"** — only if `IConclusionResolveError`'s variant or some other downstream needs it. Default: keep existing derives. Add derives lazily.
- **"can I skip the `HinputsT` plumbing? It's body migration"** — it's allowed because the bodies are one-liners and Slab 15 needs them as a baseline. If a body is more than 3 lines, defer it. Gotcha 13.
- **"can I touch `infer/compiler_solver.rs`?"** — only if a Slab-14 type-flesh-out forces it. Otherwise no.
- **"can I clean up `_Phantom` from types not in the Slab 14 inventory?"** — no. Out of scope.
- **"`IFunctionGenerator` enum variant list — same as `FunctionBodyMacro`?"** — likely overlapping but not identical. Verify by grepping Scala `extends IFunctionGenerator` (not in this repo — check Scala source if available, otherwise infer from the audit-noted 16 implementors above).

## Where to file questions

- **Design (especially `IPlaceholderSubstituter` trait-vs-struct decision)**: senior. Default Option B; flag if going Option A.
- **Scala semantics**: each Scala `/* case class ... */` block is the spec.
- **Hook rejections**: usually accidental whitespace inside `/* */`.
- **"Should this go in Slab 14 or defer to Slab 15?"**: if the work is "fill an existing placeholder type/trait", Slab 14. If it's "migrate a method body", Slab 15+. The `HinputsT` one-liners are an exception (Gotcha 13).

## Final advice

This is the **only non-signature-rewrite slab** in the typing-pass migration's pre-body-migration phase. The shape is different — you're filling enum variants and struct fields, not lifting `(&self)` stubs. Read the Scala blocks in each file and translate variant-by-variant.

Slab-14 wrinkles:

1. **`ICompileErrorT` is the biggest single chunk** (55 variants, 90-120 min) — sweep the file top-to-bottom, batch in groups of 5-10.
2. **Trait-to-enum conversions have downstream blast radius** (`IBoundArgumentsSource` touches ~13 sub-compiler sigs; `IFunctionGenerator` touches Slab-13 lifts). Audit after each.
3. **`<'s, 't>` lifetime additions** (to `IDefiningError`, `InferEnv`, `InitialKnown`, etc.) trigger downstream sig audits — let the compiler drive.
4. **`IPlaceholderSubstituter` is the only design decision** — default to struct (Option B); flag if going Option A.
5. **Bulk-rename `Slab 14` → `Slab 15` panics at the end** — purely cosmetic, but visible in the diff; matches the existing slab-number drift convention.

After Slab 14, the typing pass has:
- Every type real with Scala-parity fields/variants.
- Every trait either eliminated (god-struct dispatch) or replaced by enum/struct.
- `HinputsT` constructable.
- All ~210 method signatures correct.
- All bodies `panic!("Slab 15 — body migration")` awaiting.

**Slab 15+** = body migration, test-driven, per-method. Begin with trivial `CompilerOutputs` one-liners (already enabled by Slab 8 sigs + Slab 14 type flesh-out), then `TemplataCompiler` id-transforms, then substitution engine, then sub-compiler bodies.

The hard part of the migration is now the bodies. Slab 14 is the last "fill out the skeleton" pass.

Good luck — and congratulations on closing the type-completion phase.
