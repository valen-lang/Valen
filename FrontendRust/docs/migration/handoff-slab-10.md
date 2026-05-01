# Handoff: Typing Pass Slab 10 — `compiler.rs` Residuals + `local_helper.rs` / `struct_compiler.rs` Orphans + `class TemplataCompiler` Tail

## Who this is for

You're picking up an in-progress Rust port of a Scala compiler frontend. A senior engineer has been moving through the typing pass one "slab" at a time per `quest.md` §12. Slabs 0-9 are done:

- **Slabs 0-7** — arena substrate, names, types, templatas, envs, AST, `CompilerOutputs` data shape, `HinputsT`/`Compiler` scaffolding. All tagged `slab-N-complete`.
- **Slab 8** — lifted all 54 `CompilerOutputs` method stubs in `compiler_outputs.rs` into `impl<'s, 't> CompilerOutputs<'s, 't>` blocks with real sigs. Bodies stay `panic!()`.
- **Slab 9** — lifted all 35 `object TemplataCompiler` free-fn stubs at the head of `templata_compiler.rs` into `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>` blocks with real sigs. Bodies stay `panic!()`.

Slabs 8 and 9 established the **signature-rewrite pattern** for this phase. Slab 10 is the third signature-rewrite slab and concentrates on cleanup of the **top-level scaffolding files** that don't fit into any one sub-compiler family:

- Residual `()`-placeholder methods and object orphan static fns in **`src/typing/compiler.rs`** (the god-struct host file).
- 2 object orphan static fns in **`src/typing/expression/local_helper.rs`**.
- 1 object orphan static fn in **`src/typing/citizen/struct_compiler.rs`** (+ one already-typed nearby fn that gets lifted for consistency).
- The **14 bare-`(&self)` tail impls** in **`src/typing/templata_compiler.rs`** at lines 1439–1945, which Slab 9 deliberately left alone — these are the Scala `class TemplataCompiler` instance methods.

Total: **~30 signatures** across 4 files. Budget ~3 hours focused. After Slab 10, Slabs 11-13 each target one sub-compiler layer (expression, solver, function/macros) on the same pattern.

**Read these first in this order**, then come back:

1. `FrontendRust/docs/migration/handoff-slab-9.md` — the prior signature-rewrite slab on the same file (`templata_compiler.rs`). The translation-rules table, receiver classification, Gotcha set (drop-`interner`/`keywords`, trait-param handling, `&'t IInDenizenEnvironmentT`, `&mut CompilerOutputs` conservative default, closure params) apply verbatim. Slab 10 reuses 100% of the Slab-9 pattern with no new design decisions.
2. `FrontendRust/docs/migration/handoff-slab-8.md` — the foundational signature-rewrite slab. The signature translation table is the canonical reference.
3. `TL-HANDOFF.md` at repo root — file-layout standards ("one fn per impl block, multi-line body") + the current slab roadmap.
4. `FrontendRust/src/typing/templata_compiler.rs` lines 101–1437 — the **49 already-lifted `impl Compiler` blocks** (14 class-methods from prior work + 35 object-methods from Slab 9). These are your style template.
5. This doc.

You shouldn't need to read the Scala source externally — every Scala `def` sig is already embedded inline in the `/* def ... */` blocks beneath each Rust stub. Those blocks are authoritative.

---

## The big picture: why Slab 10 exists

After Slab 9, `templata_compiler.rs` is mostly migrated — but the 14 bare `pub fn foo(&self) { panic!(...) }` stubs at the tail (the `class TemplataCompiler` instance methods) remain. Their param lists are lost entirely; each is just `(&self)` with no other params, no return type.

Similarly, `compiler.rs` has 8 class methods with `()`-placeholder params and 5 object orphan static fns with `()`-placeholders (`print`, `consecutive`, `is_primitive`, `get_mutabilities`, `get_mutability`). These are the `object Compiler { ... }` utilities.

Finally, `local_helper.rs` and `struct_compiler.rs` each have a tail `object Foo { ... }` block with 2 and 2 free-fn stubs respectively. Some of these already have typed params (e.g. `determine_if_local_is_addressible`'s params are already `&ITemplataT<'s, 't>` and `&LocalS<'s>`); they just haven't been lifted onto `impl Compiler` yet. Slab 10 lifts them uniformly.

The work is all the same shape as Slab 9:

1. Read the Scala `/* def ... */` block beneath each stub.
2. Translate each parameter/return per the Slab 8/9 rules table.
3. Drop `interner` / `keywords` params wherever Scala passes them (Rust accesses via `self.typing_interner` / `self.keywords`).
4. Wrap in a one-fn `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't { pub fn ... (&self, ...) -> ... { panic!("..."); } }` block.
5. Leave the Scala `/* */` block unchanged.

No new design decisions. No new lifetime tricks. No trait definitions needed beyond what Slab 9 already encountered. If you hit something surprising, reread the Slab 9 Gotcha list first — the answer is almost always there.

By the end of Slab 10:

- `cargo check --lib` passes with 0 errors, 0 warnings.
- All 30 stubs described below have real `(&self, ...)` signatures inside one-fn `impl` blocks. Bodies stay `panic!("Unimplemented: Slab 14 — body migration");`. (Note: Slab 8/9 used the message `"Unimplemented: Slab 10 — body migration"` — that text is stale per TL-HANDOFF; new panic messages use `Slab 14`. Don't re-touch the ~89 stale messages already in the codebase; only new ones.)
- Scala `/* */` blocks unchanged byte-for-byte.
- Only the 4 files listed below touched. No imports added except what the new signatures require.

**What Slab 10 is NOT:**

- No body migration. Bodies stay `panic!()`. Slab 14+.
- No `IBoundArgumentsSource` refactor. Keep `&'t dyn IBoundArgumentsSource<'s, 't>` where Scala's `IBoundArgumentsSource` appears (Slab 9 Gotcha 3).
- No `IPlaceholderSubstituter` trait definition. None of the 30 Slab-10 stubs need that return type (the three `get_placeholder_substituter*` methods that do were already lifted in Slab 9). If a Slab-10 stub's Scala body *calls* `TemplataCompiler.getPlaceholderSubstituter`, that's a body concern — Slab 14+ territory.
- No filling `UseBoundsFromContainer` unit struct fields.
- No `ICompileErrorT` variant filling.
- No touching of the 35 Slab-9-lifted blocks at `templata_compiler.rs` lines 101–1037 or the 14 prior-work-lifted blocks at lines 1034–1437.
- No touching of Slab 8's 54 `CompilerOutputs` impl blocks.
- No `expression_compiler.rs` / `infer_compiler.rs` / `overload_resolver.rs` / etc. — Slabs 11–13.
- No touching the `IFunctionGenerator::generate` trait method at `compiler.rs:57` — it's a trait method, not a `Compiler` method. Out of scope.
- No touching the `DefaultPrintyThing` struct at `compiler.rs:98` — it's a placeholder struct, not a stub to lift. Leave `DefaultPrintyThing<'s, 't>(pub std::marker::PhantomData<...>)` alone.

---

## Stub inventory: the 30 targets

### File 1: `src/typing/compiler.rs` — 13 stubs

**Class methods (8 — all use `&self`, all have `()` placeholders):**

| Line | Stub | Scala signature (summary) |
|---|---|---|
| 826 | `evaluate(&self, code_map: (), package_to_program_a: ()) -> ()` | `evaluate(codeMap: FileCoordinateMap[String], packageToProgramA: PackageCoordinateMap[ProgramA]): Result[HinputsT, ICompileErrorT]` |
| 1522 | `preprocess_struct(&self, name_to_struct_defined_macro: (), struct_name_t: (), struct_a: ()) -> Vec<()>` | `preprocessStruct(nameToStructDefinedMacro: Map[StrI, IOnStructDefinedMacro], structNameT: IdT[INameT], structA: StructA): Vector[(IdT[INameT], IEnvEntry)]` |
| 1541 | `preprocess_interface(&self, ...) -> Vec<()>` | `preprocessInterface(...): Vector[(IdT[INameT], IEnvEntry)]` |
| 1565 | `determine_macros_to_call<T>(&self, name_to_macro: (), ...) -> Vec<T>` | `determineMacrosToCall[T](nameToMacro: Map[StrI, T], defaultCalledMacros: Vector[MacroCallS], parentRanges: List[RangeS], attributes: Vector[ICitizenAttributeS]): Vector[T]` |
| 1595 | `ensure_deep_exports(&self, coutputs: ())` | `ensureDeepExports(coutputs: CompilerOutputs): Unit` |
| 1696 | `is_root_function(&self, function_a: ()) -> bool` | `isRootFunction(functionA: FunctionA): Boolean` |
| 1714 | `is_root_struct(&self, struct_a: ()) -> bool` | `isRootStruct(structA: StructA): Boolean` |
| 1724 | `is_root_interface(&self, interface_a: ()) -> bool` | `isRootInterface(interfaceA: InterfaceA): Boolean` |

**Object orphan static fns (5 — currently free fns at module scope, lift to `impl Compiler` with `&self`):**

| Line | Stub | Scala signature (summary) |
|---|---|---|
| 102 | `print(x: ())` | `object DefaultPrintyThing.print(x: => Object)` — debug printer |
| 1740 | `consecutive(exprs: Vec<()>) -> ()` | `object Compiler.consecutive(exprs: Vector[ReferenceExpressionTE]): ReferenceExpressionTE` |
| 1771 | `is_primitive(kind: ()) -> bool` | `object Compiler.isPrimitive(kind: KindT): Boolean` |
| 1788 | `get_mutabilities(coutputs: (), concrete_values2: Vec<()>) -> Vec<()>` | `object Compiler.getMutabilities(coutputs: CompilerOutputs, concreteValues2: Vector[KindT]): Vector[ITemplataT[MutabilityTemplataType]]` |
| 1798 | `get_mutability(coutputs: (), concrete_value2: ()) -> ()` | `object Compiler.getMutability(coutputs: CompilerOutputs, concreteValue2: KindT): ITemplataT[MutabilityTemplataType]` |

Notes:
- `print` comes from `object DefaultPrintyThing`, not `object Compiler` — but the Slab 9 convention of lifting every companion-object method onto `Compiler` applies. Pattern: `pub fn print(&self, x: ()) { ... }` with a TODO comment on the `x: => Object` by-name parameter (Scala has no direct Rust analog for call-by-name; `x: ()` is fine as a placeholder since the body stays `panic!()`).
- `consecutive` takes a `Vector[ReferenceExpressionTE]` — translate per Slab 8 rules: `&[&'t ReferenceExpressionTE<'s, 't>]` (arena refs per AASSNCMCX).
- `get_mutability` / `get_mutabilities` / `is_primitive` take `coutputs: CompilerOutputs` and `kind: KindT` — per Slab 8: `coutputs: &CompilerOutputs<'s, 't>` (read-only here — the Scala body only calls `coutputs.lookupMutability`), `kind: KindT<'s, 't>` by value (Copy).

### File 2: `src/typing/expression/local_helper.rs` — 2 stubs

Both currently at module scope with typed params, not yet lifted onto `impl Compiler`.

| Line | Stub | Scala signature |
|---|---|---|
| 381 | `fn determine_if_local_is_addressible<'s, 't>(mutability: &ITemplataT<'s, 't>, local_a: &LocalS<'s>) -> bool` | `determineIfLocalIsAddressible(mutability: ITemplataT[MutabilityTemplataType], localA: LocalS): Boolean` |
| 398 | `fn determine_local_variability<'s>(local_a: &LocalS<'s>) -> VariabilityT` | `determineLocalVariability(localA: LocalS): VariabilityT` |

Lift both onto `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't { pub fn ... (&self, ...) { panic!(...) } }`. Change param types to match Slab 8/9 conventions:
- `mutability: &ITemplataT<'s, 't>` → `mutability: ITemplataT<'s, 't>` by value (ITemplataT is Copy per Slab 3).
- `local_a: &LocalS<'s>` → keep as `local_a: &'s LocalS<'s>` (scout-side, arena-referenced).

### File 3: `src/typing/citizen/struct_compiler.rs` — 2 stubs

These live in `pub mod struct_compiler_module { ... }` at the file tail (lines 527–578) mirroring Scala's `object StructCompiler { ... }`.

| Line | Stub | Scala signature |
|---|---|---|
| 532 | `pub fn get_compound_type_mutability(member_types: &[CoordT<'_, '_>]) -> MutabilityT` | `getCompoundTypeMutability(memberTypes2: Vector[CoordT]): MutabilityT` |
| 543 | `pub fn get_mutability<'s, 't>(sanity_check: bool, interner: &Interner<'s>, keywords: &Keywords<'s>, coutputs: &CompilerOutputs<'s, 't>, ...) -> ITemplataT<'s, 't>` | `getMutability(sanityCheck: Boolean, interner: Interner, keywords: Keywords, coutputs: CompilerOutputs, originalCallingDenizenId: IdT[ITemplateNameT], region: RegionT, structTT: StructTT, boundArgumentsSource: IBoundArgumentsSource): ITemplataT[MutabilityTemplataType]` |

Lift both onto `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't`. On `get_mutability`, **drop `interner: &Interner<'s>` and `keywords: &Keywords<'s>` per Slab 9 Gotcha 2** — access via `self.typing_interner` / `self.keywords` in the body (body stays panic-stubbed).

Also remove the `pub mod struct_compiler_module { ... }` wrapper once both fns are lifted — the module existed solely to host the two object fns. The Scala `/* object StructCompiler { */` block marker stays in place (frozen).

### File 4: `src/typing/templata_compiler.rs` — 14 stubs

The `class TemplataCompiler` instance methods at lines 1439–1945. Each is currently `pub fn foo(&self) { panic!("Unimplemented: foo"); }` with no other params — your job is to fill in params and return type from the Scala block directly below. `impl` headers and `where 's: 't` bounds already exist; you just replace the method signature inside.

| Line | Stub | Scala signature (summary) |
|---|---|---|
| 1442 | `is_type_convertible` | `isTypeConvertible(coutputs, callingEnv, parentRanges, callLocation, sourcePointerType, targetPointerType): Boolean` |
| 1521 | `pointify_kind` | `pointifyKind(coutputs, kind, region, ownershipIfMutable): CoordT` |
| 1619 | `lookup_templata_by_name` | `lookupTemplata(env: IEnvironmentT, coutputs, range, name: INameT): ITemplataT[ITemplataType]` |
| 1638 | `lookup_templata_by_rune` | `lookupTemplata(env: IEnvironmentT, coutputs, range, name: IImpreciseNameS): Option[ITemplataT[ITemplataType]]` |
| 1661 | `coerce_kind_to_coord` | `coerceKindToCoord(coutputs, kind, region): CoordT` |
| 1681 | `coerce_to_coord` | `coerceToCoord(coutputs, env, range, templata, region): ITemplataT[ITemplataType]` |
| 1748 | `resolve_struct_template` | `resolveStructTemplate(structTemplata: StructDefinitionTemplataT): IdT[IStructTemplateNameT]` |
| 1760 | `resolve_interface_template` | `resolveInterfaceTemplate(interfaceTemplata: InterfaceDefinitionTemplataT): IdT[IInterfaceTemplateNameT]` |
| 1772 | `resolve_citizen_template` | `resolveCitizenTemplate(citizenTemplata: CitizenDefinitionTemplataT): IdT[ICitizenTemplateNameT]` |
| 1786 | `citizen_is_from_template` | `citizenIsFromTemplate(actualCitizenRef: ICitizenTT, expectedCitizenTemplata: ITemplataT[ITemplataType]): Boolean` |
| 1805 | `create_placeholder` | `createPlaceholder(coutputs, env, namePrefix, genericParam, index, runeToType, currentHeight, registerWithCompilerOutputs): ITemplataT[ITemplataType]` |
| 1863 | `create_coord_placeholder_inner` | `createCoordPlaceholderInner(coutputs, env, namePrefix, index, rune, currentHeight, regionMutability, kindOwnership, registerWithCompilerOutputs): CoordTemplataT` |
| 1890 | `create_kind_placeholder_inner` | `createKindPlaceholderInner(coutputs, env, namePrefix, index, rune, kindOwnership, registerWithCompilerOutputs): KindTemplataT` |
| 1932 | `create_non_kind_non_region_placeholder_inner` | `createNonKindNonRegionPlaceholderInner[T <: ITemplataType](namePrefix, index, rune, tyype: T): PlaceholderTemplataT[T]` |

The last one, `create_non_kind_non_region_placeholder_inner[T <: ITemplataType]`, has a Scala type parameter. **Erase it** per Slab 3 — `PlaceholderTemplataT` is monomorphic on the Rust side (`ITemplataT<'s, 't>` is monomorphic; the phantom `[T]` is gone). Return `ITemplataT<'s, 't>` (or more specifically, whatever `PlaceholderTemplataT` maps to — check Slab 3 output). Drop the `tyype: T` param if it's unused in the Rust return type, or keep as `tyype: ITemplataType<'s, 't>` if `ITemplataType` exists as a standalone type on the Rust side; grep for `ITemplataType` to check.

---

## Signature translation rules

**Same table as Slab 8 and 9 — no new rules in Slab 10.** Reread Slab 9 §"Signature translation rules" if rusty. Quick recap for the types you'll encounter most in Slab 10:

| Scala | Rust |
|---|---|
| `CompilerOutputs` param (read-only per Scala body) | `&CompilerOutputs<'s, 't>` |
| `CompilerOutputs` param (mutates in Scala body) | `&mut CompilerOutputs<'s, 't>` — **conservative default** for the substitution/placeholder-register methods (Slab 9 Gotcha 8) |
| `IInDenizenEnvironmentT` / `IEnvironmentT` | `&'t IInDenizenEnvironmentT<'s, 't>` (**Slab-4 override**, never `&'s`) |
| `List[RangeS]` / `Vector[RangeS]` | `&[RangeS<'s>]` |
| `IdT[X]` (any phantom) | `IdT<'s, 't>` by value |
| `StructDefinitionTemplataT` / `InterfaceDefinitionTemplataT` / `CitizenDefinitionTemplataT` | `&'t StructDefinitionTemplataT<'s, 't>` etc. (heavy templatas — see Slab 3) |
| `KindT` / `CoordT` / `ITemplataT[X]` | by value (all Copy) |
| `RegionT` | by value (small enum; verify Copy) |
| `OwnershipT` | by value |
| `ICitizenTT` | by value (Copy per Slab 3) |
| `StructA` / `InterfaceA` / `FunctionA` | `&'s StructA<'s>` etc. (scout-side, arena-referenced) |
| `GenericParameterS` | `&'s GenericParameterS<'s>` (scout-side) |
| `LocalS` | `&'s LocalS<'s>` (scout-side) |
| `MacroCallS` / `ICitizenAttributeS` | `&'s MacroCallS<'s>` / `&[&'s ICitizenAttributeS<'s>]` (scout-side) |
| `Map[IRuneS, ITemplataType]` | `&HashMap<IRuneS<'s>, ITemplataType<'s, 't>>` (or just `&HashMap<IRuneS<'s>, ITemplataType>` if `ITemplataType` exists as a scout-side enum) — grep for prior usage |
| `Map[StrI, IOnStructDefinedMacro]` | `&HashMap<StrI<'s>, OnStructDefinedMacro>` (Copy dispatch-tag enum at `src/typing/macros/macros.rs:59`; the Scala trait is replaced by a 2-variant Copy enum — pass by value, not ref) |
| `Map[StrI, IOnInterfaceDefinedMacro]` | `&HashMap<StrI<'s>, OnInterfaceDefinedMacro>` (Copy enum at `macros.rs:72`, 2 variants) |
| `Map[StrI, IOnImplDefinedMacro]` | `&HashMap<StrI<'s>, OnImplDefinedMacro>` (Copy enum at `macros.rs:86`, **0 variants** — empty enum, the Scala map is initialized empty) |
| `Map[StrI, IFunctionBodyMacro]` | `&HashMap<StrI<'s>, FunctionBodyMacro>` (Copy enum at `macros.rs:23`, 15 variants) |
| `FileCoordinateMap[String]` | `&FileCoordinateMap<'s, &'s str>` (`src/utils/code_hierarchy.rs:304`, generic on `Contents`) |
| `PackageCoordinateMap[ProgramA]` | `&PackageCoordinateMap<'s, &'s ProgramA<'s>>` (`src/utils/code_hierarchy.rs:640`) |
| `ITemplataType` (the Scala trait, not `ITemplataT[X]`) | `ITemplataType<'s>` by value (enum at `src/postparsing/itemplatatype.rs:61`, scout-side) |
| `Option[Int]` | `Option<i32>` |
| `Interner`, `Keywords` params | **dropped** (Slab 9 Gotcha 2). Body uses `self.typing_interner` / `self.keywords`. |
| `Vector[(IdT[INameT], IEnvEntry)]` return | `Vec<(IdT<'s, 't>, &'t IEnvEntryT<'s, 't>)>` or `&[...]` — Scala returns a fresh vector, so `Vec` is fine; `&'t [...]` if the body would arena-allocate. Conservative: `Vec<...>` for now, Slab 14+ may optimize. |
| `Result[HinputsT, ICompileErrorT]` | `Result<HinputsT<'s, 't>, ICompileErrorT<'s, 't>>` (matches `compilation.rs::run_typing_pass` precedent from Slab 7) |
| Scala type parameter `[T]` / `[T <: ITemplataType]` | **Erase.** `determine_macros_to_call`'s `[T]` is generic over the macro-trait family; in Rust this becomes a concrete trait-object or an erased return. Default: keep the `<T>` on `determine_macros_to_call` because the Scala body just stores/filters values — the Rust version can stay generic. For `create_non_kind_non_region_placeholder_inner[T]`, erase. |

### Receiver classification

All 30 methods on `&self`. None need `&mut self` — `Compiler<'s, 'ctx, 't>` holds only immutable refs (`scout_arena`, `typing_interner`, `keywords`, `opts`). Mutation happens through `coutputs: &mut CompilerOutputs<'s, 't>`.

### Derive / lifetime bounds

- Every `impl` block: `<'s, 'ctx, 't>` with `where 's: 't`.
- `pub fn` (not `fn`).
- No new derives.

---

## Shape of a lifted stub

### Example 1: `compiler.rs` class method

Before (line 1522):

```rust
fn preprocess_struct(&self, name_to_struct_defined_macro: (), struct_name_t: (), struct_a: ()) -> Vec<()> {
    panic!("Unimplemented: preprocess_struct");
}
/*
  private def preprocessStruct(
    nameToStructDefinedMacro: Map[StrI, IOnStructDefinedMacro],
    structNameT: IdT[INameT],
    structA: StructA): Vector[(IdT[INameT], IEnvEntry)] = { ... }
*/
```

After:

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn preprocess_struct(
        &self,
        name_to_struct_defined_macro: &HashMap<StrI<'s>, OnStructDefinedMacro>,
        struct_name_t: IdT<'s, 't>,
        struct_a: &'s StructA<'s>,
    ) -> Vec<(IdT<'s, 't>, &'t IEnvEntryT<'s, 't>)> {
        panic!("Unimplemented: Slab 14 — body migration");
    }
}
/*
  private def preprocessStruct(
    nameToStructDefinedMacro: Map[StrI, IOnStructDefinedMacro],
    structNameT: IdT[INameT],
    structA: StructA): Vector[(IdT[INameT], IEnvEntry)] = { ... }
*/
```

Note: `fn preprocess_struct` was originally inside a `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> { ... }` block (starting at line 166). Lifting it means **closing the enclosing impl at the Scala block above `preprocess_struct`** and opening a new one-fn impl for the new method. Follow the existing lift pattern — re-read Slab 9's actual diff for a file-structure reference.

**Important**: `preprocess_struct` is currently inside the giant `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {` opened at line 166 and closed at line 1727. That impl block contains the already-lifted `evaluate` panic-stub and the 8 methods you're lifting. Slab 9 convention is one-fn impl blocks — so **split** the giant impl into 9 per-method impls. Each gets its own `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't { ... }` wrapper placed directly above its Scala `/* */` anchor. (The giant impl at `compiler.rs:166` is a hold-over from prior work; splitting is a style fix, not a semantic change.)

### Example 2: `templata_compiler.rs` tail method

Before (line 1442):

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn is_type_convertible(&self) { panic!("Unimplemented: is_type_convertible"); }
/*
  def isTypeConvertible(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    sourcePointerType: CoordT,
    targetPointerType: CoordT):
  Boolean = { ... }
*/
}
```

After:

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn is_type_convertible(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: &'t IInDenizenEnvironmentT<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        source_pointer_type: CoordT<'s, 't>,
        target_pointer_type: CoordT<'s, 't>,
    ) -> bool {
        panic!("Unimplemented: Slab 14 — body migration");
    }
/*
  def isTypeConvertible(... same as before ...): Boolean = { ... }
*/
}
```

The outer impl header and Scala block stay as-is. Only the bare `pub fn is_type_convertible(&self)` line gets replaced with the multi-line parameter list and real return type.

### Example 3: `local_helper.rs` orphan lift

Before (line 381):

```rust
fn determine_if_local_is_addressible<'s, 't>(mutability: &ITemplataT<'s, 't>, local_a: &LocalS<'s>) -> bool {
  panic!("Unimplemented: determine_if_local_is_addressible");
}
/*
  def determineIfLocalIsAddressible(mutability: ITemplataT[MutabilityTemplataType], localA: LocalS): Boolean = { ... }
*/
```

After:

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn determine_if_local_is_addressible(
        &self,
        mutability: ITemplataT<'s, 't>,
        local_a: &'s LocalS<'s>,
    ) -> bool {
        panic!("Unimplemented: Slab 14 — body migration");
    }
}
/*
  def determineIfLocalIsAddressible(mutability: ITemplataT[MutabilityTemplataType], localA: LocalS): Boolean = { ... }
*/
```

Param type changes: `&ITemplataT<'s, 't>` → `ITemplataT<'s, 't>` by value (Copy per Slab 3); `&LocalS<'s>` → `&'s LocalS<'s>` (arena-referenced).

---

## Gotchas

### Gotcha 1: `compiler.rs` has a giant multi-method `impl` block — split it

The `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {` at `compiler.rs:166` wraps all 8 class methods (`evaluate` through `is_root_interface`). TL-HANDOFF's "one fn per impl block, multi-line body" convention wants each method in its own impl. **Split** the giant impl into 8 one-fn impls as part of Slab 10.

Don't do this as a separate commit-style cleanup — just do it inline with the signature lifts. Each of the 8 methods gets:

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn <name>(&self, ... params ...) -> <ret> {
        panic!("Unimplemented: Slab 14 — body migration");
    }
}
```

... placed directly above its Scala `/* def ... */` anchor. The Scala block moves down relative to the impl (the Scala block currently sits *between* the `fn foo { ... }` body and the next stub).

### Gotcha 2: `print` is on `object DefaultPrintyThing`, not `object Compiler`

`DefaultPrintyThing` is a 2-line Scala helper object — Rust currently has a `pub struct DefaultPrintyThing<'s, 't>(pub std::marker::PhantomData<...>)` placeholder at line 98 plus a free `pub fn print(x: ()) { panic!() }` at line 102.

Two options for the lift:

A) Leave `DefaultPrintyThing` as a placeholder struct; add `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> { pub fn print(&self, x: ()) { panic!() } }` on `Compiler` (consistent with lifting every object method to `Compiler`).

B) Keep `print` on a `DefaultPrintyThing` impl block.

**Pick A** for consistency with the Slab 9 convention. Option B would diverge — we're unifying every object-method onto `Compiler` across the pass.

The `x: => Object` by-name parameter in Scala has no direct Rust analog. Use `x: ()` with a TODO comment: `// TODO: Slab 14 — Scala uses a by-name parameter here; pick impl Display or &str as the Rust equivalent when porting the body.` Body stays panic-stubbed.

### Gotcha 3: macros are Copy dispatch-tag enums, not traits

Scala's `IOnStructDefinedMacro`, `IOnInterfaceDefinedMacro`, `IOnImplDefinedMacro`, `IFunctionBodyMacro` were traits with concrete implementors. The Rust codebase has already replaced them with Copy dispatch-tag enums at `src/typing/macros/macros.rs`:

- `OnStructDefinedMacro` (line 59, 2 variants: `StructConstructor`, `StructDrop`)
- `OnInterfaceDefinedMacro` (line 72, 2 variants: `AnonymousInterface`, `InterfaceDrop`)
- `OnImplDefinedMacro` (line 86, **0 variants** — the Scala map is initialized empty)
- `FunctionBodyMacro` (line 23, 15 variants)

All four derive `Copy, Clone, Debug, PartialEq, Eq`. Use them **by value** in signatures (no `&'t`, no `dyn`):

```rust
name_to_struct_defined_macro: &HashMap<StrI<'s>, OnStructDefinedMacro>,
```

Per the Scala `/* */` comments at each enum, the trait bodies (`getStructSiblingEntries`, `getInterfaceSiblingEntries`, `generateFunctionBody`) are now methods on `Compiler` dispatched by matching on the enum variant. Slab 14+ body migration fills that dispatch in; for Slab 10 you just need the enum type in parameter slots.

### Gotcha 4: `FileCoordinateMap` / `PackageCoordinateMap` / `ITemplataType` exist — use them

All three types the `evaluate` and `create_non_kind_non_region_placeholder_inner` signatures reference are already defined:

- `FileCoordinateMap<'s, Contents>` — `src/utils/code_hierarchy.rs:304` (generic on `Contents`)
- `PackageCoordinateMap<'s, Contents>` — `src/utils/code_hierarchy.rs:640`
- `ITemplataType<'s>` — `src/postparsing/itemplatatype.rs:61` (the scout-side enum behind Scala's `sealed trait ITemplataType`; don't confuse with `ITemplataT<'s, 't>`, which is the value)

Use them directly:

```rust
code_map: &FileCoordinateMap<'s, &'s str>,
package_to_program_a: &PackageCoordinateMap<'s, &'s ProgramA<'s>>,
// ...
tyype: ITemplataType<'s>,
```

No `()` placeholders needed for these three.

### Gotcha 5: `determine_macros_to_call<T>` — keep the generic

Scala: `def determineMacrosToCall[T](nameToMacro: Map[StrI, T], ...): Vector[T]`. The `[T]` is a free type parameter — the body stores/filters `T` values but doesn't introspect their type. Keep as:

```rust
pub fn determine_macros_to_call<T>(
    &self,
    name_to_macro: &HashMap<StrI<'s>, T>,
    default_called_macros: &[&'s MacroCallS<'s>],
    parent_ranges: &[RangeS<'s>],
    attributes: &[&'s ICitizenAttributeS<'s>],
) -> Vec<T> {
    panic!("Unimplemented: Slab 14 — body migration");
}
```

Where `T: Clone` may be needed once the body lands (the Scala body uses `T` values once — probably won't need `Clone`; decide in Slab 14). For now omit bounds.

### Gotcha 6: `create_non_kind_non_region_placeholder_inner[T <: ITemplataType]` — erase the type param

Scala has `createNonKindNonRegionPlaceholderInner[T <: ITemplataType](..., tyype: T): PlaceholderTemplataT[T]`. Rust's `ITemplataT<'s, 't>` is monomorphic (Slab 3) — the `PlaceholderTemplataT[T]` phantom was erased.

Check if `ITemplataType` exists as a standalone Rust type (it's the Scala `sealed trait ITemplataType` — not to be confused with `ITemplataT[X]`, which is the value):

```
Grep pattern: "pub (enum|struct) ITemplataType"
```

If it exists (likely), use `tyype: ITemplataType<'s>` or similar. Return `ITemplataT<'s, 't>` (not `PlaceholderTemplataT<'s, 't, T>`).

### Gotcha 7: `struct_compiler.rs` — remove the `pub mod struct_compiler_module` wrapper after lifting

Once both object-fns (`get_compound_type_mutability`, `get_mutability`) are lifted onto `impl Compiler`, the `pub mod struct_compiler_module { ... }` block is empty except for the Scala `/* object StructCompiler { */` comment. Delete the `pub mod` wrapper; leave the Scala block in place at file scope.

**Don't delete the Scala block.** Pre-commit hook enforces it.

### Gotcha 8: bodies stay `panic!()` — resist the temptation (restatement of Slab 9 Gotcha 10)

Several Slab-10 methods have trivial Scala bodies:

- `is_root_function` — 8 lines, mostly pattern match.
- `is_root_struct` — 2 lines.
- `is_root_interface` — 2 lines.
- `resolve_struct_template` / `resolve_interface_template` — 3 lines each.
- `coerce_kind_to_coord` — 8 lines.
- `get_compound_type_mutability` — 5 lines.

**Don't port them.** Slab 14+ migrates bodies together. Consistency wins.

### Gotcha 9: panic message uses **Slab 14**, not Slab 10

Slab 8 and 9 used `panic!("Unimplemented: Slab 10 — body migration")`. That text is now stale — body migration is Slab 14+. New panic messages in Slab 10 use **`"Unimplemented: Slab 14 — body migration"`**.

Don't re-touch the ~89 stale Slab-10 panic messages already in the codebase. Those get bulk-updated when bodies land.

### Gotcha 10: one-fn impl blocks per TL-HANDOFF convention

Same as Slab 8/9 Gotcha. Each lifted method gets its own `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't { pub fn foo(...) { panic!(...) } }` block, adjacent to its Scala `/* def ... */` anchor.

Do NOT consolidate methods into a single impl block. Slab 9 established this — follow it.

### Gotcha 11: Scala `/* */` blocks are frozen

Same as every prior slab. Pre-commit hook `.claude/hooks/check-scala-comments` enforces byte-for-byte.

### Gotcha 12: no downstream breakage expected

All 30 stubs are currently `panic!()`. No caller exercises them at runtime. Lifting changes the signature but preserves panic bodies — compilation of downstream callers (which don't yet exist for most) is unaffected.

If you see a compile error after a lift, it's a signature mistranslation (wrong lifetime, wrong by-value-vs-by-ref, missing `&'t`), not a downstream issue.

### Gotcha 13: `Result` / error types — use `ICompileErrorT<'s, 't>` not `ICompileError[S]`

`evaluate` returns `Result[HinputsT, ICompileErrorT]`. Rust side: `Result<HinputsT<'s, 't>, ICompileErrorT<'s, 't>>`. Note: the Rust `ICompileErrorT` is still a `_Phantom`-only enum per TL-HANDOFF — that's fine, nothing constructs variants yet.

---

## Step-by-step plan

### Step 1: Confirm starting point

```bash
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-10.txt 2>&1
```

Then:

```bash
grep -c "^error" /tmp/sylvan-slab-10.txt
```

Must print `0`.

### Step 2: Pre-flight type location refresher

The types Slab-10 signatures reference all exist already (verified during handoff authoring). Confirm their locations if you want to double-check:

- Macro dispatch enums: `src/typing/macros/macros.rs:23` (`FunctionBodyMacro`), `:59` (`OnStructDefinedMacro`), `:72` (`OnInterfaceDefinedMacro`), `:86` (`OnImplDefinedMacro`)
- `FileCoordinateMap<'s, Contents>`: `src/utils/code_hierarchy.rs:304`
- `PackageCoordinateMap<'s, Contents>`: `src/utils/code_hierarchy.rs:640`
- `ITemplataType<'s>`: `src/postparsing/itemplatatype.rs:61`
- `LocationInDenizen<'s>`: `src/postparsing/ast.rs:1168`
- `LocalS<'s>`: `src/postparsing/expressions.rs:179`
- `MacroCallS<'s>`: `src/postparsing/ast.rs:197`
- `ICitizenAttributeS<'s>`: `src/postparsing/ast.rs:129`

If a grep turns up an unexpected absence, stop and flag it before continuing — the handoff doc was authored on the assumption these all exist.

### Step 3: Lift file-by-file

Recommended order (smallest → largest file, building momentum):

1. **`local_helper.rs` (2 stubs)** — warm-up. 30-45 min.
2. **`struct_compiler.rs` (2 stubs + module-wrapper cleanup)** — 30-45 min.
3. **`compiler.rs` class methods (8) + split the giant impl** — 60-90 min.
4. **`compiler.rs` object orphan fns (5)** — 30-45 min.
5. **`templata_compiler.rs` tail (14)** — 60-75 min. This is the largest chunk; the impl headers are already in place, so it's the most mechanical.

### Step 4: Incremental verification

After each file:

```bash
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-10.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-10.txt
```

Fix errors before moving on. Common mistakes (see Slab 9 for the canonical list):
- `Interner` or `Keywords` param left in (Gotcha 2 of Slab 9).
- `IEnvironmentT` instead of `IInDenizenEnvironmentT`.
- `&'s IInDenizenEnvironmentT` instead of `&'t IInDenizenEnvironmentT`.
- Forgetting `&mut` on `coutputs` when the Scala body mutates.
- Missing `&'s` on scout-side types (`LocalS`, `StructA`, `FunctionA`, `GenericParameterS`).
- Treating `IBoundArgumentsSource` as a concrete enum rather than `&'t dyn IBoundArgumentsSource<'s, 't>`.

### Step 5: Final verification

```bash
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-10.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-10.txt        # must be 0
grep -c "^warning" /tmp/sylvan-slab-10.txt      # must be 0
tail -3 /tmp/sylvan-slab-10.txt                  # must show "Finished"
```

Sanity greps:

```
Grep pattern: "fn .*\(.*: \(\)" path: FrontendRust/src/typing/compiler.rs
# ^ should match 0 lines (no more () placeholders)

Grep pattern: "pub fn .*\(&self\) \{" path: FrontendRust/src/typing/templata_compiler.rs
# ^ should match 0 lines (no more bare (&self) stubs)

Grep pattern: "pub mod struct_compiler_module" path: FrontendRust/src/typing/citizen/struct_compiler.rs
# ^ should match 0 lines (module wrapper deleted)

Grep pattern: "Slab 14 — body migration" path: FrontendRust/src/typing
# ^ should be exactly 30 (one per Slab-10 lift)
```

### Step 6: Diff self-review

```bash
git diff FrontendRust/src/typing/compiler.rs | head -300
git diff FrontendRust/src/typing/templata_compiler.rs | head -300
git diff FrontendRust/src/typing/expression/local_helper.rs
git diff FrontendRust/src/typing/citizen/struct_compiler.rs
git diff --stat
```

Confirm:
- Only stub rewrites and new `impl` wrappers.
- No edits inside Scala `/* */` blocks.
- No other files touched.
- `struct_compiler_module` wrapper deleted; Scala `/* object StructCompiler { */` anchor preserved.
- Every new panic message uses "Slab 14 — body migration"; no stale messages re-touched.

### Step 7: Hand off

**Never commit.** Hand back uncommitted; the human tags `slab-10-complete`.

Work-order checkpoints:
- Step 2 — pre-flight greps recorded.
- Step 3 (1/5) — `local_helper.rs` done.
- Step 3 (2/5) — `struct_compiler.rs` done.
- Step 3 (3/5) — `compiler.rs` class methods done + impl block split.
- Step 3 (4/5) — `compiler.rs` object orphans done.
- Step 3 (5/5) — `templata_compiler.rs` tail done.
- Step 5 — verification greps pass.
- Step 6 — diff self-review.
- Step 7 — hand back.

---

## What "done" looks like

- `cargo check --lib` passes with 0 errors, 0 warnings.
- All 30 target stubs have real signatures inside one-fn `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't { pub fn ... (&self, ...) { panic!(...) } }` blocks.
- `interner` and `keywords` parameters dropped across every lifted method.
- `coutputs` params are `&CompilerOutputs<'s, 't>` (read-only) or `&mut CompilerOutputs<'s, 't>` (mutating) per Scala body.
- Every env param is `&'t IInDenizenEnvironmentT<'s, 't>`.
- Every definition/heavy-templata param is `&'t X<'s, 't>`.
- Every scout-side type (`StructA`, `LocalS`, `MacroCallS`, `GenericParameterS`, `ICitizenAttributeS`) is `&'s X<'s>`.
- `IdT<'s, 't>` / `KindT<'s, 't>` / `CoordT<'s, 't>` / `ITemplataT<'s, 't>` / `ICitizenTT<'s, 't>` passed by value.
- Scala type parameters erased (`IdT[X]` → `IdT<'s, 't>`; `[T <: ITemplataType]` → concrete type).
- Macro dispatch enums (`OnStructDefinedMacro` etc.) passed by value, not `dyn` trait objects.
- Panic messages on lifted methods read `"Unimplemented: Slab 14 — body migration"`.
- `compiler.rs`'s giant multi-method `impl` block at line 166 split into 8 one-fn impls.
- `struct_compiler.rs`'s `pub mod struct_compiler_module` wrapper deleted.
- Scala `/* */` blocks unchanged byte-for-byte.
- No file outside the 4 Slab-10 targets modified.
- Handed back uncommitted.

---

## When you're stuck

- **"cannot find type `IOnStructDefinedMacro` / `IFunctionBodyMacro`"**: these don't exist as traits — they've been replaced by Copy dispatch-tag enums (`OnStructDefinedMacro`, `FunctionBodyMacro`, etc.) at `src/typing/macros/macros.rs`. Gotcha 3.
- **"cannot find type `FileCoordinateMap` / `ITemplataType`"**: they exist — `src/utils/code_hierarchy.rs` and `src/postparsing/itemplatatype.rs`. Gotcha 4.
- **"the trait `Sized` is not implemented for `dyn IBoundArgumentsSource`"**: always `&'t dyn`, not by value.
- **"`'s` may not live long enough"**: missing `where 's: 't` on the `impl` header.
- **"lifetime mismatch: expected `&'t X<'s, 't>`, found `&'s X<'s, 't>`"**: env/definition borrowed from wrong lifetime. Check Slab 4 override (envs in `'t`, not `'s`).
- **"too many arguments to function"**: left `interner` or `keywords` in the param list.
- **"I want to port `is_root_struct` because it's 2 lines"**: don't. Gotcha 8. Slab 14 ports all bodies.
- **"`IResolveOutcome` is `_Phantom`-only"**: correct. Use it as-is in signatures; Slab 14 fills variants.
- **"the `DefaultPrintyThing` struct at line 98 is weird"**: it's a placeholder. Leave the struct alone; only lift the `print` free fn onto `Compiler`.
- **"`determine_macros_to_call<T>` — do I need `T: Clone`?"**: add bounds only if `cargo check` demands them. With a panic body, none are needed.
- **"I want to touch `expression_compiler.rs` because it also has () placeholders"**: don't. Slab 11.
- **"I want to define `IPlaceholderSubstituter` / `IRuneTypeSolverEnv` because `get_placeholder_substituter` already returns `()`"**: don't. That's Slab 14+.

## Where to file questions

- **Design**: `FrontendRust/docs/migration/handoff-slab-9.md` is the spec for the pattern this slab continues. If Slab 9 and this doc disagree on a general rule, Slab 9 wins. File-specific details here (compiler.rs impl-block split, struct_compiler_module deletion, 4-file scope) are authoritative.
- **Scala semantics**: each Scala `/* def ... */` block beneath a stub is the spec.
- **Hook rejections**: read the hook's diff output. Usually accidental whitespace inside `/* */`.

## Final advice

This is the third signature-rewrite slab. The pattern from Slab 8 and 9 carries directly — just apply it to four new files. No new design decisions required; every tricky pattern (drop-`interner`/`keywords`, trait-param handling, closure params, erased type parameters, unknown-trait `()` placeholders) was already resolved in Slab 9.

**Key differences from Slab 9:**

1. **Four files instead of one** — lift stubs file-by-file for localized review gates.
2. **`compiler.rs` needs an impl-block split** — the giant multi-method impl at line 166 becomes 8 one-fn impls.
3. **`struct_compiler.rs` needs a module-wrapper deletion** — `pub mod struct_compiler_module` goes away once empty.
4. **Panic message uses "Slab 14"**, not "Slab 10" (post-audit slab-roadmap correction).
5. **Macro params use Copy dispatch-tag enums**, not `dyn` trait objects — `OnStructDefinedMacro` / `OnInterfaceDefinedMacro` / `OnImplDefinedMacro` / `FunctionBodyMacro` at `src/typing/macros/macros.rs`. Pass by value, no lifetime params.

After Slab 10, the typing pass has:
- All `CompilerOutputs` methods with real sigs (Slab 8).
- All `TemplataCompiler::object` methods with real sigs (Slab 9).
- All `compiler.rs` residual class methods and `object Compiler` fns with real sigs (Slab 10).
- All `TemplataCompiler::class` tail methods with real sigs (Slab 10).
- `local_helper.rs` and `struct_compiler.rs` object orphans lifted onto `Compiler` (Slab 10).
- All data-def structs real (Slabs 0–7).
- ~95 remaining sub-compiler methods still partial (Slabs 11–13).

**Slab 11** = expression-layer sigs (~39 across `expression_compiler.rs` / `pattern_compiler.rs` / `block_compiler.rs` / `call_compiler.rs`).
**Slab 12** = solver + resolver sigs (~35 across `infer_compiler.rs` / `overload_resolver.rs` / `impl_compiler.rs` / `reachability.rs`).
**Slab 13** = function/citizen/macros sigs (~23 across `function_compiler.rs` / `function_body_compiler.rs` / `destructor_compiler.rs` / `struct_compiler_core.rs` / `anonymous_interface_macro.rs`).

After Slab 13, the signature-rewrite phase is complete and the typing pass compiles with all methods having real parameter/return types. Slab 14+ is pure body migration, driven by tests.

Good luck.
