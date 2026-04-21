# Handoff: Typing Pass Slab 14b — Second-Wave Placeholder Type Flesh-Out

## Who this is for

You're picking up an in-progress Rust port of a Scala compiler frontend. Slabs 0-14 are done:

- **Slabs 0-13** — arena substrate + all ~210 method signatures across the typing pass.
- **Slab 14** — first-wave placeholder type flesh-out (`ICompileErrorT` with 55 variants, `IBoundArgumentsSource` → enum, `IFunctionGenerator` → dispatch enum, `IPlaceholderSubstituter` defined, `HinputsT::new()`, and ~12 other error/result/solver-state types filled).

`cargo check --lib` is clean (0 errors, 0 warnings). 141 `panic!("Unimplemented: Slab 15 — body migration")` bodies await.

**Slab 14a** deliberately scoped itself to the specific placeholders the senior's audit named. But a **second wave of `_Phantom` types** remained — types that Slab 14a chose not to touch but that body migration (Slab 15+) will hit very quickly. Rather than let those types get filled in a scattered way across dozens of body-migration commits, **Slab 14b** clears the decks in one focused pass.

**Slab 14b scope: 15 placeholder types** across AST, function-result, and solver-error families. Budget **~2.5-3 hours focused** — smaller than Slab 14a because no trait-to-enum conversions (the novel hard work is already done) and no mass variant list like `ICompileErrorT`'s 55.

After Slab 14b, every typing-pass placeholder type is filled. Slab 15+ body migration starts from a complete type skeleton.

**Read these first in this order**, then come back:

1. `FrontendRust/docs/migration/handoff-slab-14.md` — the precedent pass. Translation rules (`Vector[X]` → `&'t [X<'s, 't>]`, `Map` → `HashMap`, scout-side single-lifetime, etc.), Gotcha 2 (let compiler drive lifetime propagation), Gotcha 5 (variant batching), Gotcha 7 (`String` → `&'s str`), Gotcha 10 (bulk-rename panic messages — N/A this slab, no new stubs).
2. `FrontendRust/docs/migration/handoff-slab-9.md` — canonical translation-rules table.
3. `TL-HANDOFF.md` at repo root — file-layout standards.
4. This doc.

Each type's Scala source is anchored in `/* */` blocks next to the Rust placeholder — no external source reading needed.

---

## The big picture: why Slab 14b exists

Slab 14a's audit listed ~15 placeholders to fill. The junior completing Slab 14a flagged a handback observation: **more `_Phantom` placeholders exist** that weren't in the audit list. Re-audit found ~15 additional types — mostly AST attribute/member types, function-compilation result types, and solver-error types.

These second-wave types weren't blocking the signature-rewrite phase (Slabs 8-13), which is why the initial audit missed them. But body migration (Slab 15+) will construct and pattern-match on these types heavily:

- `CitizenDefinitionT` — every citizen-compilation body touches it (`CompilerOutputs::add_struct/add_interface` stores it; Slab 15+ bodies read it everywhere).
- `ICalleeCandidate` — overload resolution returns candidate lists; every body in `overload_resolver.rs` touches it.
- `IDefineFunctionResult` / `IResolveFunctionResult` / `IStampFunctionResult` — returned from 5+ methods in `function_compiler.rs`.
- `ITypingPassSolverError` — 16 variants used as the error type in `infer/compiler_solver.rs` — every solver body constructs these.
- `IFunctionAttributeT` / `ICitizenAttributeT` — matched in attribute-translation bodies.
- `IStructMemberT` / `IMemberTypeT` — every struct-compilation body touches members.

Fill them now → body migration goes faster and stays focused on logic (not interleaved with type churn).

By the end of Slab 14b:

- Every `_Phantom` placeholder in the inventory below is replaced with real Scala-parity variants/fields.
- `cargo check --lib` clean (0 errors, 0 warnings).
- Scala `/* */` blocks unchanged byte-for-byte.
- Only the necessary files touched (AST, function_compiler.rs, compiler_solver.rs, names.rs, and a few downstream cascades).

**What Slab 14b is NOT:**

- No body migration. Bodies stay `panic!()`. Slab 15+.
- No signature rewrites (Slabs 8-13 already done).
- No touching of types already fleshed out by Slab 14a.
- No `HinputsT` lookup method body migrations — those are still Scala `/* */` blocks (no Rust stubs exist yet); Slab 15+ writes them from scratch when the first consumer demands them.
- No re-touching of the 89 stale `Slab 10 — body migration` panic messages (they get bulk-updated as their owning methods migrate in Slab 15+).
- No renumbering the 141 `Slab 15 — body migration` panic messages (they're correctly labeled — this slab is 14b, not 15).

---

## Inventory: 15 placeholders

### Group A: AST foundational types (7 items, 60-75 min)

#### A1. `CitizenDefinitionT<'s, 't>` — fold in 2 existing concrete structs

**Location**: `src/typing/ast/citizens.rs:23`. Currently:
```rust
pub enum CitizenDefinitionT<'s, 't> {
    _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
```

**Scala**:
```scala
sealed trait CitizenDefinitionT
```

The concrete implementors `StructDefinitionT` and `InterfaceDefinitionT` already exist with real fields at `citizens.rs:54` and `:225`. Their Scala blocks end with `extends CitizenDefinitionT`.

**Work**: fold into enum variants.
```rust
pub enum CitizenDefinitionT<'s, 't> {
    Struct(StructDefinitionT<'s, 't>),
    Interface(InterfaceDefinitionT<'s, 't>),
}
```

Follow the `IsParentResult` precedent from Slab 14a Gotcha 1 — keep the structs standalone; wrap them in enum variants. Delete `_Phantom`.

**Downstream audit**: `CompilerOutputs`'s `get_all_citizens` (or similar) and `add_struct`/`add_interface` methods may take/return `CitizenDefinitionT` by value — check if their signatures need derive adjustments. If the structs aren't Copy, pass by value (move).

#### A2. `IStructMemberT<'s, 't>` — fold in 2 concrete structs

**Location**: `src/typing/ast/citizens.rs:138`. Currently `_Phantom`.

**Scala variants** (concrete structs already exist at lines 151, 166):
- `NormalStructMemberT(name, variability, tyype)` → `NormalStructMemberT<'s, 't>` struct exists, already filled.
- `VariadicStructMemberT(name, tyype)` → `VariadicStructMemberT<'s, 't>` struct exists, already filled.

```rust
pub enum IStructMemberT<'s, 't> {
    Normal(NormalStructMemberT<'s, 't>),
    Variadic(VariadicStructMemberT<'s, 't>),
}
```

#### A3. `IMemberTypeT<'s, 't>` — fold in 2 concrete structs

**Location**: `src/typing/ast/citizens.rs:178`. Currently `_Phantom`.

**Scala variants** (concrete structs at lines 213, 219):
- `AddressMemberTypeT(reference: CoordT)`
- `ReferenceMemberTypeT(reference: CoordT)`

```rust
pub enum IMemberTypeT<'s, 't> {
    Address(AddressMemberTypeT<'s, 't>),
    Reference(ReferenceMemberTypeT<'s, 't>),
}
```

#### A4. `IFunctionAttributeT<'s, 't>` — 4 variants

**Location**: `src/typing/ast/ast.rs:650`. Currently `_Phantom`.

**Scala variants** (inline `case class` / `case object` blocks at lines 664-687):
- `case class ExternT(packageCoord: PackageCoordinate) extends IFunctionAttributeT with ICitizenAttributeT`
- `case object PureT extends IFunctionAttributeT`
- `case object AdditiveT extends IFunctionAttributeT`
- `case object UserFunctionT extends IFunctionAttributeT`

**`ExternT` caveat**: it extends BOTH `IFunctionAttributeT` and `ICitizenAttributeT`. Rust can't do multiple-inheritance. The existing placeholder `pub struct ExternT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>)` at `ast.rs:662` should be promoted to:
```rust
pub struct ExternT<'s> {
    pub package_coord: PackageCoordinate<'s>,
}
```

Then reference it from both enum variants:

```rust
pub enum IFunctionAttributeT<'s, 't> {
    Extern(ExternT<'s>),
    Pure,
    Additive,
    UserFunction,
    // 't may become unused after this — if so, drop the 't param and make the enum <'s> only.
}

pub enum ICitizenAttributeT<'s, 't> {  // see A5
    Extern(ExternT<'s>),
    Sealed,
}
```

**Lifetime decision**: if `IFunctionAttributeT` ends up only using `'s` (because Extern is `<'s>` and the other variants are unit), drop the `<'s, 't>` in favor of `<'s>`. Let `cargo check` guide — add an `_Phantom(PhantomData<&'t ()>)` variant temporarily if needed, remove once the downstream consumer types have their `<'s, 't>` settled.

#### A5. `ICitizenAttributeT<'s, 't>` — 2 variants

**Location**: `src/typing/ast/ast.rs:656`. Currently `_Phantom`.

**Scala variants** (lines 664, 684):
- `ExternT(packageCoord: PackageCoordinate)` (shared with A4)
- `case object SealedT extends ICitizenAttributeT`

```rust
pub enum ICitizenAttributeT<'s, 't> {
    Extern(ExternT<'s>),
    Sealed,
}
```

Same lifetime deliberation as A4.

#### A6. `ICalleeCandidate<'s, 't>` — fold in 3 concrete structs

**Location**: `src/typing/ast/ast.rs:433`. Currently `_Phantom`.

**Scala variants** (concrete structs at lines 439, 453, 467 with `extends ICalleeCandidate` in their Scala blocks):
- `FunctionCalleeCandidate(ft: FunctionTemplataT)`
- `HeaderCalleeCandidate(header: FunctionHeaderT)`
- `PrototypeTemplataCalleeCandidate(prototypeT: PrototypeT[IFunctionNameT])`

```rust
pub enum ICalleeCandidate<'s, 't> {
    Function(FunctionCalleeCandidate<'s, 't>),
    Header(HeaderCalleeCandidate<'s, 't>),
    PrototypeTemplata(PrototypeTemplataCalleeCandidate<'s, 't>),
}
```

Matches `IsParentResult` pattern. Delete `_Phantom`.

#### A7. `AbstractT<'s, 't>` — promote to real unit or field-bearing struct

**Location**: `src/typing/ast/ast.rs:390`. Currently `_Phantom` struct.

**Scala** (grep for the Scala block nearby):
```scala
case class AbstractT()  // if a case class, or case object AbstractT
```

If it's `case class AbstractT()` with no fields (most likely based on the single-marker role), **convert to a unit struct**:
```rust
pub struct AbstractT;
```

Drop `<'s, 't>` — no lifetimes needed. If cargo flags consumers, add lifetimes back as demanded.

If Scala has fields, follow them. Grep the `/* */` block.

### Group B: Function-compilation result types (3 enum-triple sets, 45-60 min)

#### B1. `IDefineFunctionResult<'s, 't>` + `DefineFunctionSuccess` + `DefineFunctionFailure`

**Location**: `src/typing/function/function_compiler.rs:120-140`. Currently all `_Phantom`.

**Scala**:
```scala
trait IDefineFunctionResult

case class DefineFunctionSuccess(
    prototype: PrototypeTemplataT[IFunctionNameT],
    inferences: Map[IRuneS, ITemplataT[ITemplataType]],
    instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT]
) extends IDefineFunctionResult

case class DefineFunctionFailure(reason: IDefiningError) extends IDefineFunctionResult
```

Rust:
```rust
pub struct DefineFunctionSuccess<'s, 't> {
    pub prototype: &'t PrototypeTemplataT<'s, 't>,
    pub inferences: HashMap<IRuneS<'s>, ITemplataT<'s, 't>>,
    pub instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
}

pub struct DefineFunctionFailure<'s, 't> {
    pub reason: IDefiningError<'s, 't>,
}

pub enum IDefineFunctionResult<'s, 't> {
    Success(DefineFunctionSuccess<'s, 't>),
    Failure(DefineFunctionFailure<'s, 't>),
}
```

Matches the `IEvaluateFunctionResult` pattern from Slab 14a Group C. `PrototypeTemplataT` is monomorphic (Slab 3). Phantoms erased.

#### B2. `IResolveFunctionResult<'s, 't>` + `ResolveFunctionSuccess` + `ResolveFunctionFailure`

Same shape:
```scala
case class ResolveFunctionSuccess(
    prototype: PrototypeTemplataT[IFunctionNameT],
    inferences: Map[IRuneS, ITemplataT[ITemplataType]]
) extends IResolveFunctionResult

case class ResolveFunctionFailure(reason: IResolvingError) extends IResolveFunctionResult
```

Rust:
```rust
pub struct ResolveFunctionSuccess<'s, 't> {
    pub prototype: &'t PrototypeTemplataT<'s, 't>,
    pub inferences: HashMap<IRuneS<'s>, ITemplataT<'s, 't>>,
}

pub struct ResolveFunctionFailure<'s, 't> {
    pub reason: IResolvingError<'s, 't>,
}

pub enum IResolveFunctionResult<'s, 't> {
    Success(ResolveFunctionSuccess<'s, 't>),
    Failure(ResolveFunctionFailure<'s, 't>),
}
```

#### B3. `IStampFunctionResult<'s, 't>` + `StampFunctionFailure` (Success already filled by Slab 14a)

`StampFunctionSuccess` already has real fields (Slab 14a C1). Just the enum and Failure need fleshing:

```scala
case class StampFunctionFailure(reason: IFindFunctionFailureReason) extends IStampFunctionResult
```

Rust:
```rust
pub struct StampFunctionFailure<'s, 't> {
    pub reason: IFindFunctionFailureReason<'s, 't>,
}

pub enum IStampFunctionResult<'s, 't> {
    Success(StampFunctionSuccess<'s, 't>),
    Failure(StampFunctionFailure<'s, 't>),
}
```

### Group C: Solver error enum (1 big enum, 45-60 min)

#### C1. `ITypingPassSolverError<'s, 't>` — 16 variants

**Location**: `src/typing/infer/compiler_solver.rs:54`. Currently `_Phantom`.

**Scala variants** (inline `case class ... extends ITypingPassSolverError` at lines 56-~130):

| Variant | Scala fields |
|---|---|
| `KindIsNotConcrete` | `kind: KindT` |
| `KindIsNotInterface` | `kind: KindT` |
| `KindIsNotStruct` | `kind: KindT` |
| `CouldntFindFunction` | `range: List[RangeS]`, `fff: FindFunctionFailure` |
| `CouldntFindImpl` | `range: List[RangeS]`, `fail: IsntParent` |
| *(more at line ~76-83)* | *grep the file* |
| `CantShareMutable` | `kind: KindT` |
| `CantSharePlaceholder` | `kind: KindT` |
| `BadIsaSubKind` | `kind: KindT` |
| `BadIsaSuperKind` | `kind: KindT` |
| `SendingNonCitizen` | `kind: KindT` |
| `CantCheckPlaceholder` | `range: List[RangeS]` |
| `ReceivingDifferentOwnerships` | `params: Vector[(IRuneS, CoordT)]` |
| `SendingNonIdenticalKinds` | `sendCoord: CoordT, receiveCoord: CoordT` |
| `NoCommonAncestors` | `params: Vector[(IRuneS, CoordT)]` |
| `LookupFailed` | `name: IImpreciseNameS` |
| `NoAncestorsSatisfyCall` | `params: Vector[(IRuneS, CoordT)]` |
| `CantDetermineNarrowestKind` | `kinds: Set[KindT]` |
| *(more below line 122)* | grep for `extends ITypingPassSolverError` |

**Check the full file** by greping `extends ITypingPassSolverError` in `compiler_solver.rs`; the audit found ~16 but the real count might be 18-20. Add all of them.

Rust:
```rust
pub enum ITypingPassSolverError<'s, 't> {
    KindIsNotConcrete { kind: KindT<'s, 't> },
    KindIsNotInterface { kind: KindT<'s, 't> },
    KindIsNotStruct { kind: KindT<'s, 't> },
    CouldntFindFunction { range: &'t [RangeS<'s>], fff: FindFunctionFailure<'s, 't> },
    CouldntFindImpl { range: &'t [RangeS<'s>], fail: IsntParent<'s, 't> },
    CantShareMutable { kind: KindT<'s, 't> },
    CantSharePlaceholder { kind: KindT<'s, 't> },
    BadIsaSubKind { kind: KindT<'s, 't> },
    BadIsaSuperKind { kind: KindT<'s, 't> },
    SendingNonCitizen { kind: KindT<'s, 't> },
    CantCheckPlaceholder { range: &'t [RangeS<'s>] },
    ReceivingDifferentOwnerships { params: &'t [(IRuneS<'s>, CoordT<'s, 't>)] },
    SendingNonIdenticalKinds { send_coord: CoordT<'s, 't>, receive_coord: CoordT<'s, 't> },
    NoCommonAncestors { params: &'t [(IRuneS<'s>, CoordT<'s, 't>)] },
    LookupFailed { name: IImpreciseNameS<'s> },
    NoAncestorsSatisfyCall { params: &'t [(IRuneS<'s>, CoordT<'s, 't>)] },
    CantDetermineNarrowestKind { kinds: &'t [KindT<'s, 't>] },
    // ... any remaining from the grep
}
```

`Set[KindT]` → `&'t [KindT<'s, 't>]` (AASSNCMCX: no HashSet inside an arena type if avoidable; use slice).

### Group D: Region name enum (1 enum, 15-30 min)

#### D1. `IRegionNameT<'s, 't>` — grep for variants

**Location**: `src/typing/names/names.rs:471`. Currently `_Phantom`.

**Scala**:
```scala
sealed trait IRegionNameT extends INameT
```

Grep for `extends IRegionNameT` in `src/typing/names/names.rs` to find the concrete variants. Likely 1-3 variants (e.g. `RegionPlaceholderNameT`, `RegionNameT`). Follow the IDEPFL precedent from Slab 2 — the enum wraps `&'t` refs to concrete interned structs:

```rust
pub enum IRegionNameT<'s, 't> {
    // Grep-discovered variants, e.g.:
    // Placeholder(&'t RegionPlaceholderNameT<'s, 't>),
    // Concrete(&'t RegionNameT<'s, 't>),
}
```

**If you find 0 variants** (i.e. `IRegionNameT` is a trait with no current implementors — possible if Scala's hierarchy has it as pure abstract), keep `_Phantom` and note in the handback. The enum exists for type-system consistency but has no variants yet; Slab 15+ fills implementors as they're introduced.

Also verify the `INameValT` tagged-union at Slab 4 references `IRegionNameT` — if so, add a `Region(IRegionNameT<'s, 't>)` variant there too. If not, nothing downstream breaks.

### Group E: Misc (4 items, 15-30 min)

#### E1. `TookWeakRefOfNonWeakableError<'s, 't>` — demote to unit struct

**Location**: `src/typing/expression/expression_compiler.rs:55`. Currently `_Phantom` struct with `<'s, 't>`.

**Scala**:
```scala
case class TookWeakRefOfNonWeakableError() extends Throwable
```

Rust:
```rust
pub struct TookWeakRefOfNonWeakableError;
```

Drop `<'s, 't>` — no lifetimes needed. If cargo flags downstream, re-add as demanded. **Note**: this is a Throwable in Scala (an exception type). Rust doesn't do exceptions; it'll be used as an error variant inside `Result<..., TookWeakRefOfNonWeakableError>` or folded into `ICompileErrorT` eventually. For Slab 14b, just make it a real unit struct; Slab 15+ decides how it's thrown/returned.

#### E2. `DefaultPrintyThing<'s, 't>` — demote to unit struct

**Location**: `src/typing/compiler.rs:111`. Currently `_Phantom` struct.

**Scala**:
```scala
object DefaultPrintyThing {
  def print(x: => Object) = { ... }
}
```

Scala's `object` corresponds to a singleton. Rust doesn't have singletons; the existing struct+impl pattern (Slab 10 lifted `print` onto `Compiler`) means `DefaultPrintyThing` itself is now unused-as-a-type. Two options:

A. **Delete** `DefaultPrintyThing` entirely (its `print` method was already lifted to `Compiler::print` in Slab 10; the struct was a placeholder).
B. **Keep as unit struct** `pub struct DefaultPrintyThing;` — harmless placeholder, matches Scala's `object` name as a type marker.

**Recommend**: Option A (delete). Verify with a grep: `DefaultPrintyThing` should have 0 uses in non-Scala-comment Rust code. If grep confirms 0 uses, delete the struct and its Scala anchor stays in place (frozen).

#### E3. `IContainer::_Phantom` variant — remove straggler

**Location**: `src/typing/templata/templata.rs:421`. Currently:
```rust
pub enum IContainer<'s, 't> {
  Interface(ContainerInterface<'s>),
  Struct(ContainerStruct<'s>),
  Function(ContainerFunction<'s>),
  Impl(ContainerImpl<'s>),
  _Phantom(std::marker::PhantomData<&'t ()>),
}
```

The `_Phantom` was added to "use" the `'t` generic, but all 4 real variants are `<'s>` only — `'t` isn't actually needed. Two fixes:

A. **Drop `'t` from `IContainer`**: `pub enum IContainer<'s>` with the 4 variants. Remove `_Phantom`. Downstream: any `IContainer<'s, 't>` reference becomes `IContainer<'s>`. Simplest.

B. **Keep `<'s, 't>` and the `_Phantom`**: body migration may add a `'t`-using variant later (e.g. if a container variant needs to reference a `'t` arena type). Preserves flexibility at the cost of one phantom variant.

**Recommend**: Option A (drop `'t`). If Slab 15+ finds it needs `'t`, re-add then. Downstream audit: let cargo drive.

#### E4. `AbstractT` / `ExternT` — handled in A4/A7

Done as part of Group A. Flagging here for completeness of the inventory.

### Group F (optional): `HinputsT` lookup bodies

The `HinputsT` struct has ~10 lookup methods that are still **Scala `/* */` blocks only** — no Rust stubs exist yet. Slab 14a Gotcha 13 deferred these to Slab 15+ because writing new stubs is body-migration territory.

**Slab 14b recommendation**: skip this. Body migration writes the Rust stubs from scratch when the first caller needs them. Including here would expand scope beyond the "fill placeholders" charter.

If you want to include anyway (e.g. `HinputsT::lookup_function` is a one-liner `self.functions.iter().find(...)`), do it under the same "3-line cap" rule as Slab 14a F2. Flag in the handback which lookups landed.

---

## Signature translation rules

Same as Slab 14a. Quick reference:

| Scala | Rust |
|---|---|
| `RangeS` / `List[RangeS]` | `RangeS<'s>` / `&'t [RangeS<'s>]` |
| `KindT` / `CoordT` / `ITemplataT[X]` | by value (Copy) |
| `IRuneS` / `IImpreciseNameS` | by value (Copy) |
| `INameS` / `IVarNameS` | by value (Copy, scout) |
| `IdT[X]` / `KindT` / `ICitizenTT` | by value |
| `INameT` / `IVarNameT` | by value (Copy) |
| `PackageCoordinate` | `PackageCoordinate<'s>` by value (Copy) |
| `PrototypeTemplataT[X]` / `PrototypeT[X]` | `&'t PrototypeTemplataT<'s, 't>` / `&'t PrototypeT<'s, 't>` (phantom erased) |
| `InstantiationBoundArgumentsT[X, Y]` | `&'t InstantiationBoundArgumentsT<'s, 't>` |
| `FunctionTemplataT` | `FunctionTemplataT<'s, 't>` by value (Copy heavy templata per Slab 3) |
| `FunctionHeaderT` | `&'t FunctionHeaderT<'s, 't>` |
| `Vector[X]` / `Set[X]` / `Iterable[X]` in arena-owned fields | `&'t [X<'s, 't>]` per AASSNCMCX |
| `Map[K, V]` | `HashMap<K, V>` (owned; `HinputsT`/`CompilerOutputs` AASSNCMCX deviation applies here too) |
| `Option[X]` | `Option<X>` |
| `String` | `&'s str` (scout-arena) |
| `Int` / `Boolean` | `i32` / `bool` |
| `IDefiningError` / `IResolvingError` / `IFindFunctionFailureReason` / etc. | by value (move) with `<'s, 't>` lifetimes from Slab 14a |

Named-variant syntax preferred for enum variants with multiple fields. Tuple variants OK for single-field wrappers (e.g. `CitizenDefinitionT::Struct(StructDefinitionT<'s, 't>)`).

---

## Gotchas

### Gotcha 1: `ExternT` multi-inheritance — one struct, two enum references

Scala's `ExternT(packageCoord: PackageCoordinate)` extends both `IFunctionAttributeT` and `ICitizenAttributeT`. Rust can't do multiple-inheritance, but you can reference the same struct from two different enums. Convert the existing `_Phantom` `ExternT<'s, 't>` to a real single-field struct (`<'s>` only, since `PackageCoordinate<'s>` is scout):

```rust
pub struct ExternT<'s> {
    pub package_coord: PackageCoordinate<'s>,
}
```

Then reference in both:
```rust
pub enum IFunctionAttributeT<'s, 't> { Extern(ExternT<'s>), Pure, Additive, UserFunction }
pub enum ICitizenAttributeT<'s, 't> { Extern(ExternT<'s>), Sealed }
```

Hash/Eq concerns: `ExternT` gets derived Copy+Clone+PartialEq+Eq+Hash. Two `ExternT` values from different enum variants are `==` if their package coords are `==`. That's fine for pattern matching.

### Gotcha 2: lifetime minimization on attribute/unit-heavy enums

`IFunctionAttributeT` has `Extern(<'s>)` + 3 unit variants. `ICitizenAttributeT` has `Extern(<'s>)` + 1 unit variant. Neither touches `'t`.

**Decide**: drop `'t` from these enums?

- Option A: `pub enum IFunctionAttributeT<'s>` (simpler).
- Option B: `pub enum IFunctionAttributeT<'s, 't>` (keep for consumer-type consistency).

**Recommend**: Option A unless a consumer type forces `'t`. Let cargo drive: start with `<'s>`, add `<'s, 't>` back only if the compiler demands it.

Same analysis for `IRegionNameT` — if all variants are `<'s>`-only, use `<'s>`.

### Gotcha 3: `CitizenDefinitionT` enum replacement — check `CompilerOutputs` fields

Slab 6's `CompilerOutputs` struct has fields like `struct_name_to_definition: HashMap<PtrKey<'t, IdT<'s, 't>>, &'t StructDefinitionT<'s, 't>>` and `interface_name_to_definition: HashMap<..., &'t InterfaceDefinitionT<'s, 't>>` — stored by concrete type. That's fine; `CitizenDefinitionT` is used in **return** positions for "give me the citizen definition" queries.

After A1, if a signature returns `CitizenDefinitionT<'s, 't>` (by value) vs returns `&'t CitizenDefinitionT<'s, 't>` (by ref), check the existing sig. Most return-by-value patterns since `CitizenDefinitionT` wraps concrete arena refs (likely `&'t StructDefinitionT`-level is already in the concrete struct).

Wait — `StructDefinitionT<'s, 't>` is the direct struct, not a ref. So `CitizenDefinitionT::Struct(StructDefinitionT<'s, 't>)` contains a big struct by value. That's expensive to move. **Alternative**: wrap with `&'t`:

```rust
pub enum CitizenDefinitionT<'s, 't> {
    Struct(&'t StructDefinitionT<'s, 't>),
    Interface(&'t InterfaceDefinitionT<'s, 't>),
}
```

This matches `ICitizenTT`'s precedent from Slab 3 (wraps `&'t StructTT` / `&'t InterfaceTT`). **Recommend**: use `&'t` wrapping. `CitizenDefinitionT` becomes Copy.

**Audit downstream**: `CompilerOutputs` / `HinputsT` methods that construct/accept `CitizenDefinitionT` may need their signatures adjusted. Let cargo drive.

### Gotcha 4: `IStructMemberT` / `IMemberTypeT` — ref or value?

Same question as Gotcha 3. The concrete structs (`NormalStructMemberT`, `VariadicStructMemberT`, `AddressMemberTypeT`, `ReferenceMemberTypeT`) are stored in `StructDefinitionT.members: &'t [IStructMemberT<'s, 't>]` (verify in `citizens.rs`).

If stored as `&'t [IStructMemberT<'s, 't>]`, the enum itself must be **sized** (can't be `&'t [&'t IStructMemberT<'s, 't>]` — too many indirections). So the enum variants likely hold the concrete structs **by value**:

```rust
pub enum IStructMemberT<'s, 't> {
    Normal(NormalStructMemberT<'s, 't>),
    Variadic(VariadicStructMemberT<'s, 't>),
}
```

Verify `NormalStructMemberT`'s size (check fields at `citizens.rs:151`) — if it's <= 5 words, by-value is cheap. If it's huge, switch to `&'t NormalStructMemberT<'s, 't>` and audit the slice layout.

**Default**: by-value. Measure and fix if performance matters later.

Same for `IMemberTypeT` (smaller — just a `CoordT`).

### Gotcha 5: `ICalleeCandidate` — same ref-or-value question

Concrete structs hold single fields (`ft: FunctionTemplataT`, `header: FunctionHeaderT`, `prototypeT: PrototypeT`). `FunctionTemplataT` is Copy (Slab 3); `FunctionHeaderT` likely large (not Copy); `PrototypeT` is `&'t`-wrapped arena.

**Recommend**:
```rust
pub enum ICalleeCandidate<'s, 't> {
    Function(FunctionCalleeCandidate<'s, 't>),           // holds FunctionTemplataT (Copy)
    Header(&'t HeaderCalleeCandidate<'s, 't>),           // &'t because FunctionHeaderT is big
    PrototypeTemplata(PrototypeTemplataCalleeCandidate<'s, 't>),  // holds &'t PrototypeT
}
```

Let cargo / shields guide: if `#[derive(Clone, Debug)]` fails on the enum because `HeaderCalleeCandidate` isn't Clone, switch Header to `&'t` (which is always Copy).

### Gotcha 6: `ITypingPassSolverError` — grep for the FULL variant list before coding

The audit says 16 variants; the file has more. **Before touching the enum**, do:

```
Grep pattern: "extends ITypingPassSolverError" path: FrontendRust/src/typing/infer/compiler_solver.rs
```

Count matches. Add all of them, in file-top-to-bottom order. Don't skip any (even the ones with multi-line Scala blocks).

### Gotcha 7: lifetime propagation audits after each flip

Just like Slab 14a: flipping `IFunctionAttributeT<'s, 't>` to `IFunctionAttributeT<'s>` triggers a cascade. Every downstream `IFunctionAttributeT<'s, 't>` reference (in sub-compiler signatures, `FunctionHeaderT` field types, etc.) must be updated.

Run `cargo check --lib` after each enum flip; let the compiler identify the propagation set. Don't preemptively edit call sites.

### Gotcha 8: `_Phantom` deletion — remove after the first real variant lands

Same as Slab 14a Gotcha 6. For each enum being fleshed out, add real variants first, then delete `_Phantom` once the variants use `'s` and `'t` (or only `'s` if lifetime was minimized per Gotcha 2).

### Gotcha 9: `TookWeakRefOfNonWeakableError` is a Scala Throwable

Scala throws it as an exception. Rust doesn't do exceptions. Slab 14b just defines it as a unit struct. Slab 15+ decides whether to:
- Fold into `ICompileErrorT` as a variant (`ICompileErrorT::TookWeakRefOfNonWeakable { range: ... }`).
- Leave as a standalone error type used inside `Result<..., TookWeakRefOfNonWeakableError>`.

**Don't decide now.** Defer to Slab 15.

### Gotcha 10: `DefaultPrintyThing` deletion vs retention

If you delete (Option A per E2), verify via grep:
```
Grep pattern: "DefaultPrintyThing" path: FrontendRust/src
```

Should match only Scala `/* */` blocks and the `pub struct DefaultPrintyThing` definition itself. If any non-comment Rust code references it, don't delete — keep as unit struct (Option B).

### Gotcha 11: `IRegionNameT` grep may find no variants

Run:
```
Grep pattern: "extends IRegionNameT" path: FrontendRust/src/typing/names/names.rs
```

If 0 matches, keep `_Phantom`. Flag in handback. This is the one exception to the "delete `_Phantom` after variants land" rule — an enum with no variants yet is a legitimate forward-declared type.

If you find matches, add them following the IDEPFL pattern: each variant wraps `&'t Concrete<'s, 't>`.

### Gotcha 12: HinputsT lookup bodies — skip per §F

Do not write new stubs for `HinputsT::lookup_function` etc. Those are new-code, not placeholder-filling. Slab 15+.

### Gotcha 13: Scala `/* */` blocks are still frozen

Pre-commit hook enforces. Same as every prior slab.

### Gotcha 14: bodies stay `panic!()`

No body migration in Slab 14b. Every method keeps `panic!("Unimplemented: Slab 15 — body migration")`.

### Gotcha 15: no panic-message renames this slab

Slab 14a did the bulk `Slab 14 → Slab 15` rename. Slab 14b doesn't touch panic messages — they're already correctly labeled as `Slab 15`.

The 89 stale `Slab 10 — body migration` messages stay stale. Leave them. Gotcha 14 from Slab 14a.

### Gotcha 16: `_Phantom` preservation for types NOT in this inventory

If you encounter a `_Phantom` placeholder in a file you're editing that's NOT in the Slab 14b inventory, leave it alone. Out of scope. Flag in handback if notable.

---

## Step-by-step plan

### Step 1: Confirm starting point

```bash
cargo check --lib --manifest-path /Volumes/V/Sylvan/FrontendRust/Cargo.toml > ./tmp/sylvan-slab-14b.txt 2>&1
grep -c "^error" ./tmp/sylvan-slab-14b.txt
```

Must print `0`.

### Step 2: Pre-flight

Record current `_Phantom` count across `src/typing/`:

```
Grep pattern: "_Phantom\(std::marker::PhantomData" path: FrontendRust/src/typing
```

Note the count; Slab 14b should reduce it by ~15.

Also grep for the key variant sources:
```
Grep pattern: "extends ITypingPassSolverError" path: FrontendRust/src/typing/infer/compiler_solver.rs  # expect ~16-20
Grep pattern: "extends IRegionNameT" path: FrontendRust/src/typing/names/names.rs  # may be 0
Grep pattern: "extends IFunctionAttributeT" path: FrontendRust/src/typing/ast/ast.rs  # expect 4
Grep pattern: "extends ICitizenAttributeT" path: FrontendRust/src/typing/ast/ast.rs  # expect 2 (one shared with above)
```

### Step 3: Order of operations

Recommended order (smallest blast radius → largest):

1. **Group E misc (E1-E3, 20 min)** — unit-struct demotions + `_Phantom` straggler removal. Easy wins.
2. **Group D `IRegionNameT` (20 min)** — small enum, may be no-op if grep finds 0 variants.
3. **Group A1 `CitizenDefinitionT` (20 min)** — 2 variants, simple fold-in. Downstream audit.
4. **Group A2/A3 `IStructMemberT` + `IMemberTypeT` (30 min)** — paired work on citizens.rs.
5. **Group A6 `ICalleeCandidate` (20 min)** — 3-variant fold-in.
6. **Group A4/A5 `IFunctionAttributeT` + `ICitizenAttributeT` + `ExternT` promotion + `AbstractT` promotion (45-60 min)** — the shared-Extern multi-enum work. Biggest downstream audit in Group A.
7. **Group B function-result types (B1-B3, 45 min)** — 3 enum-triples, mechanical.
8. **Group C `ITypingPassSolverError` (45-60 min)** — largest single enum in this slab; grep full variant list first.

Run `cargo check --lib` after each group. Fix propagation errors before moving on.

### Step 4: Incremental verification

After each group:

```bash
cargo check --lib --manifest-path /Volumes/V/Sylvan/FrontendRust/Cargo.toml > ./tmp/sylvan-slab-14b.txt 2>&1
grep -c "^error" ./tmp/sylvan-slab-14b.txt
```

Common Slab-14b mistakes:
- `ExternT` inlined into both enums (duplicate struct definition — only one `pub struct ExternT<'s>` should exist).
- `<'s, 't>` kept on enums that only use `'s` (Gotcha 2 — let cargo drive).
- `CitizenDefinitionT::Struct(StructDefinitionT<'s, 't>)` by value when `&'t StructDefinitionT<'s, 't>` is cheaper (Gotcha 3 — recommend `&'t`).
- Missing variants in `ITypingPassSolverError` (didn't grep the full list).
- `_Phantom` left in place after real variants added (Gotcha 8).
- Renaming `Slab 15` panic messages (Gotcha 15 — don't).

### Step 5: Final verification

```bash
cargo check --lib --manifest-path /Volumes/V/Sylvan/FrontendRust/Cargo.toml > ./tmp/sylvan-slab-14b.txt 2>&1
grep -c "^error" ./tmp/sylvan-slab-14b.txt       # must be 0
grep -c "^warning" ./tmp/sylvan-slab-14b.txt     # must be 0
tail -3 ./tmp/sylvan-slab-14b.txt                 # must show "Finished"
```

Sanity greps:

```
Grep pattern: "_Phantom\(std::marker::PhantomData" path: FrontendRust/src/typing
# Should be drastically reduced from the Step 2 baseline. Acceptable remainders: `IRegionNameT` if grep found 0 Scala variants (Gotcha 11), and any types not in this slab's inventory.
```

### Step 6: Diff self-review

```bash
git diff FrontendRust/src/typing/ | head -500
git diff --stat
```

Confirm:
- ~6-10 typing files modified.
- No Scala `/* */` block edits.
- No panic-message changes (Gotcha 15).
- No new methods / bodies (not body migration).

### Step 7: Hand off

**Never commit.** Hand back uncommitted; the human tags `slab-14b-complete`.

Work-order checkpoints:
- Step 2 — pre-flight greps recorded (include full `ITypingPassSolverError` variant count).
- Step 3 (1/8) — Group E misc done.
- Step 3 (2/8) — Group D `IRegionNameT` done.
- Step 3 (3/8) — Group A1 `CitizenDefinitionT` done.
- Step 3 (4/8) — Group A2/A3 members done.
- Step 3 (5/8) — Group A6 `ICalleeCandidate` done.
- Step 3 (6/8) — Group A4/A5 attributes done.
- Step 3 (7/8) — Group B function-results done.
- Step 3 (8/8) — Group C `ITypingPassSolverError` done.
- Step 5 — verification greps pass.
- Step 6 — diff self-review.
- Step 7 — hand back.

---

## What "done" looks like

- `cargo check --lib` passes with 0 errors, 0 warnings.
- Every placeholder in the inventory has real Scala-parity fields/variants (or is documented-deferred per Gotcha 11 for `IRegionNameT` if no Scala variants found).
- `ExternT` is a real single-field struct referenced from `IFunctionAttributeT` and `ICitizenAttributeT`.
- `DefaultPrintyThing` deleted (or demoted to unit struct per Option B).
- `IContainer` `_Phantom` straggler removed (via Option A: drop `'t`).
- No panic-message edits.
- Scala `/* */` blocks unchanged byte-for-byte.
- Only necessary files touched.
- Handed back uncommitted.

---

## When you're stuck

- **"cannot find type `ExternT`"** — you defined it inside one enum but referenced from the other. Move the struct definition outside both enums (Gotcha 1).
- **"`IFunctionAttributeT<'s, 't>` still wants `'t` after dropping"** — some downstream consumer type expects `<'s, 't>`. Either keep `<'s, 't>` here (Option B) or propagate `<'s>` through the consumer.
- **"`_Phantom` won't let me delete — compile error says `'t` is unused"** — add a `PhantomData<&'t ()>` field to a variant OR drop `'t` from the enum signature. Drop `'t` if possible (Gotcha 2).
- **"grepping `extends IRegionNameT` finds 0 matches"** — correct; keep `_Phantom` for now. Gotcha 11.
- **"`ITypingPassSolverError` has more variants than I expected"** — correct; the audit said ~16 but the file may have 18-20. Add all of them. Gotcha 6.
- **"can I inline `NormalStructMemberT`'s fields into `IStructMemberT::Normal { name, variability, tyype }`?"** — no. Keep the structs standalone per `IsParentResult` precedent (Slab 14a Gotcha 1).
- **"`FunctionHeaderT` is huge and `ICalleeCandidate::Header(HeaderCalleeCandidate<'s, 't>)` by value bloats the enum"** — switch to `&'t HeaderCalleeCandidate<'s, 't>` (Gotcha 5).
- **"should I also fill `HinputsT::lookup_function`?"** — no. Gotcha 12.
- **"can I touch the 89 `Slab 10` panic messages?"** — no. Gotcha 15.

## Where to file questions

- **Design** (lifetime minimization, ref-vs-value on enum variants): senior. Default to the recommendations above; flag if unsure.
- **Scala semantics**: each `/* */` block anchors the spec.
- **Hook rejections**: usually accidental whitespace inside `/* */`.

## Final advice

This is a short, focused slab compared to Slabs 8-14. Almost every task is "replace `_Phantom` with a real enum or struct per the Scala block directly below" — pure translation. No novel design decisions.

Key Slab-14b wrinkles:

1. **`ExternT` multi-inheritance** — one struct referenced from two enums (Gotcha 1).
2. **Lifetime minimization** — several attribute/region enums want `<'s>`, not `<'s, 't>` (Gotcha 2).
3. **Ref-vs-value on enum variants** — `IStructMemberT`/`IMemberTypeT` by value, `CitizenDefinitionT`/`ICalleeCandidate::Header` by `&'t` (Gotchas 3/4/5).
4. **`ITypingPassSolverError` has more variants than the audit suggested** — grep the full list (Gotcha 6).
5. **`IRegionNameT` may have no Scala variants** — keep `_Phantom` if so (Gotcha 11).

After Slab 14b:
- Every typing-pass placeholder type is filled.
- ~210 method signatures ready.
- 141 `panic!("Slab 15 — body migration")` bodies await migration.
- Slab 15+ body migration starts clean: no type churn, just logic.

Good luck.
