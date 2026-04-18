# Typing Pass Migration Design

This document describes the architectural decisions for migrating `src/typing/` from Scala to Rust.

The approach is **pragmatic arena retention**: scout data lives past the typing pass, so typing output can reference it directly. Output types carry `<'s, 't>` — they can hold `&'s` refs into the scout arena. Heavy templatas hold `&'s FunctionA` / `&'s StructA` directly. No re-interning, no side tables for origin data. Envs allocate into the scout arena.

## Status (2026-04-18)

**Phase 1 — lifetime-parameter correction across `src/typing/`** — complete. Every `pub struct`, `pub enum`, and `pub trait` in the typing pass carries the generics specified in §1.5. Empty placeholder types use `PhantomData<(&'s (), ...)>` with a `// TODO: placeholder PhantomData — replace with real fields during body migration` line.

**Phase 2 — god-struct refactor** — complete. All ~20 sub-compilers and all 15 macros are now methods on a single `Compiler<'s, 'ctx, 't>`. Macro dispatch runs through four Copy unit-variant enums (`FunctionBodyMacro`, `OnStructDefinedMacro`, `OnInterfaceDefinedMacro`, `OnImplDefinedMacro`) whose variants tag the target `Compiler::<method>_<suffix>` method. Vestigial `*Compiler` PhantomData holders (`ExpressionCompiler`, `ImplCompiler`, `StructCompiler`, `ArrayCompiler`, `DestructorCompiler`, `InferCompiler`, `TemplataCompiler`, `NameTranslator`, `ConvertHelper`, `OverloadResolver`, `CompilerSolver`) were deleted once the macros stopped holding them as fields. Only `Compiler` itself remains. `IInfererDelegate` stays vestigial because fn signatures in `compiler_solver.rs` still take `&dyn IInfererDelegate`; rewriting those is a later item. Progress tracked at `FrontendRust/docs/migration/handoff-god-struct-progress.md`; master plan at `FrontendRust/docs/migration/handoff-god-struct-refactor.md`.

**Phase 3 — body migration (Slabs 1–9+)** — in progress. Replace `PhantomData` stubs with real Scala-parity fields, one type family per slab per §12.
- ✅ **Slab 1** (leaf types): `OwnershipT`/`MutabilityT`/`VariabilityT`/`LocationT` real Copy enums; primitive `KindT` payloads (`NeverT`/`VoidT`/`IntT`/`BoolT`/`StrT`/`FloatT`) got full derives; `KindT<'s, 't>` gained the six primitive variants (`_Phantom` stays to anchor lifetimes until non-primitive variants land in Slab 3); leaf-value templatas (`OwnershipTemplataT`/`VariabilityTemplataT`/`MutabilityTemplataT`/`LocationTemplataT`/`BooleanTemplataT`/`IntegerTemplataT`/`StringTemplataT`) wrap their inner values. Commit `9fd7641c`.
- ⏳ **Slab 2** (name hierarchy): `IdT<'s, 't, T: Copy>` generic with `widen`/`try_narrow`; ~60 concrete name types `<'s, 't>`; ~14 sub-enums with DAG variant duplication per §6.2; `INameValT` and per-sub-enum `*ValT` companions for IDEPFL interning; `From`/`TryFrom` bridges. Handoff at `FrontendRust/docs/migration/handoff-slab-2.md`.
- Slab 3+: Kind/Coord/Templata trio, envs, expression AST, `CompilerOutputs`, `HinputsT`, sub-compiler method signatures, method bodies.

Build is clean throughout: `cargo check --lib` passes with 0 errors and 0 warnings (the crate sets `#![allow(unused_variables, unused_imports)]` in `src/lib.rs`, so incremental drift from adding/removing imports doesn't surface as noise).

Known deferred items (separate from the slab plan):
- `HinputsT` is still just a `// mig:` marker + Scala comment — no Rust stub at all.
- Several sub-compiler methods have free `fn equals` / `fn hash_code` stubs dangling from the slice pipeline; they'll be wrapped or deleted during Slab 8.
- Macro dispatcher methods on `Compiler` (`dispatch_function_body_macro` etc.) are not wired yet — will be added when env lookup is implemented (Slab 4 / later).

### The Trade

- **Cost:** scout arena memory (FunctionA/StructA/etc. plus envs) retained through instantiation. Rough estimate: hundreds of MB to a few GB for large programs. Fine for batch compilation; reconsider for long-running LSP-style use.
- **Benefit:** the port maps to Scala line-for-line in most places. No side-table plumbing. Faster to implement, easier to review for Scala parity.

---

## Part 1: Arena and Lifetime Model

### 1.1 Three Arenas

- **`'p` — Parser arena (`ParseArena<'p>`)**: parser AST, `StrI<'p>`, coords.
- **`'s` — Scout arena (`ScoutArena<'s>`)**: postparser + higher-typing output (`FunctionA<'s>`, `StructA<'s>`, etc.), interned postparser names (`INameS<'s>`, `IRuneS<'s>`, `IImpreciseNameS<'s>`), **and typing-pass environments**.
- **`'t` — Typing arena (`TypingInterner<'t>`)**: interned typing-pass types (names, kinds, coords, templatas) and output AST (`FunctionDefinitionT`, expressions, `HinputsT`). Created before the typing pass; outlives the typing pass.

All three arenas use `bumpalo`. Arena-allocated structs follow AASSNCMCX: no `Vec`/`HashMap`/`String` inside arena types.

### 1.2 Lifetime Invariants

1. **`'s` outlives `'t`**. Expressed as `where 's: 't` on every output type that transitively holds `&'s` data. Rust does **not** enforce outlives via drop order — we must declare the bound. Representative structs that carry this bound include `HinputsT<'s, 't>`, `IEnvironmentT<'s, 't>`, `KindT<'s, 't>`, and every interned typing-pass type.
2. **`'t` outlives the typing pass.** Created by the caller before `run_typing_pass`; dropped by the caller after the instantiator is done.
3. **`'s` outlives the instantiator**, because `HinputsT<'s, 't>` contains `&'s` refs. Scout arena drops after the instantiator completes (or later).
4. **Only two arena lifetimes beyond `'p`:** `'s` and `'t`. Envs live in `'s`.
5. **Arena borrow convention.** Arena parameters use a short borrow lifetime (elided or named `'ctx`), never the arena's own lifetime. Write `&ScoutArena<'s>` or `&'ctx ScoutArena<'s>`, never `&'s ScoutArena<'s>`. This matches `ParseArena` and `ScoutArena` usage in the parser, postparser, and higher-typing passes. The decoupling works because `ScoutArena::alloc(&self, val: T) -> &'s mut T` returns `'s`-lifetimed data from a short `&self` borrow — callers don't need to hold the arena for all of `'s` just to allocate into it.

### 1.3 Arena Construction Order

```rust
fn top_level_driver() {
    let parse_arena = ParseArena::new();
    // ... parser produces parser AST into 'p ...
    
    let scout_arena = ScoutArena::new();
    // ... postparser/higher-typing produce into 's ...
    
    let typing_interner = TypingInterner::new();
    let hinputs = run_typing_pass(&scout_arena, &typing_interner, /* program_a */);
    
    // ... instantiator runs over HinputsT<'s, 't> ...
    
    // typing_interner drops (after instantiator): 't dies
    // scout_arena drops: 's dies
    // parse_arena drops: 'p dies
}
```

Rust's drop order (reverse of declaration) naturally enforces `'t < 's < 'p` lifetime nesting.

### 1.4 Where Each Type Lives

| Type | Lifetimes | Arena |
|---|---|---|
| `HinputsT` | `<'s, 't>` | `'t` for the struct itself, holds `&'s` and `&'t` refs |
| `FunctionDefinitionT`, `StructDefinitionT`, `InterfaceDefinitionT`, `ImplT`, `EdgeT` | `<'s, 't>` | `'t` |
| `KindT`, `IdT`, `INameT`, `ITemplataT` (all variants) | `<'s, 't>` | `'t`, interned |
| `CoordT` | `<'s, 't>` | inline Copy, not interned |
| `ReferenceExpressionTE`, `AddressExpressionTE` | `<'s, 't>` | `'t`, not interned |
| `OverloadSetT` | `<'s, 't>` | `'t`, interned |
| `FunctionTemplataT`, `StructDefinitionTemplataT`, etc. | `<'s, 't>` | `'t`; hold `&'s FunctionA`/`&'s StructA` directly |
| `IEnvironmentT` and sub-types | `<'s, 't>` | `'s` (allocated in scout arena) |
| `CompilerOutputs` | `<'s, 't>` | stack-owned; heap-backed HashMaps; dies at pass end |
| `Compiler` (god struct) | `<'s, 'ctx, 't>` | stack; dies at pass end |

### 1.5 Full Type Inventory — Which Arena Each Type Lives In

A complete per-type checklist for Slabs 1–6. For each `// mig:` stub, the correct lifetime signature is determined here. Use this as the authoritative reference when filling definitions.

**`'s` scout arena — existing, unchanged beyond env additions:**
- `StrI<'s>`, `PackageCoordinate<'s>`, `FileCoordinate<'s>`, `RangeS<'s>`, `CodeLocationS<'s>` — postparser-interned, reused as-is
- `INameS<'s>`, `IRuneS<'s>`, `IImpreciseNameS<'s>` — postparser names, reused as-is
- `FunctionA<'s>`, `StructA<'s>`, `InterfaceA<'s>`, `ImplA<'s>` — higher-typing output, referenced directly by heavy templatas and envs
- **NEW in typing pass:** `IEnvironmentT<'s, 't>` and all 9 variants (allocated via `scout_arena.alloc(...)`)

**`'t` typing arena — interned (dedup via `TypingInterner`):**
- Concrete name structs (`FunctionNameT`, `StructNameT`, etc. — ~60 of them)
- `IdT<'s, 't, T>` (generic)
- `KindT<'s, 't>` (all variants including primitives, for uniformity)
- `ITemplataT<'s, 't>` (all variants)
- `PrototypeT<'s, 't>`, `SignatureT<'s, 't>`
- `OverloadSetT<'s, 't>`

**`'t` typing arena — allocated but NOT interned:**
- `FunctionDefinitionT<'s, 't>`, `FunctionHeaderT<'s, 't>`
- `StructDefinitionT<'s, 't>`, `InterfaceDefinitionT<'s, 't>`, `ImplT<'s, 't>`
- `EdgeT<'s, 't>`, `OverrideT<'s, 't>`
- `ParameterT<'s, 't>`, `ILocalVariableT<'s, 't>`
- `ReferenceExpressionTE<'s, 't>` (~38 variants), `AddressExpressionTE<'s, 't>` (~6 variants)
- `InstantiationBoundArgumentsT<'s, 't>`
- Heavy templata payloads: `FunctionTemplataT`, `StructDefinitionTemplataT`, `InterfaceDefinitionTemplataT`, `ImplDefinitionTemplataT`, `ExternFunctionTemplataT`
- `HinputsT<'s, 't>` (pass output)

**Inline Copy, NOT interned (Scala-verbatim structural equality):**
- Name sub-enum families (`INameT`, `IFunctionNameT`, `IFunctionTemplateNameT`, `IInstantiationNameT`, `IStructNameT`, `IInterfaceNameT`, `ICitizenNameT`, `ISubKindNameT`, `ISuperKindNameT`, `ICitizenTemplateNameT`, `IStructTemplateNameT`, `IInterfaceTemplateNameT`, `ISubKindTemplateNameT`, `ISuperKindTemplateNameT`, `ITemplateNameT`, `IImplNameT`, `IImplTemplateNameT`, `IPlaceholderNameT`, `IVarNameT`, `IRegionNameT`, `CitizenNameT`, `CitizenTemplateNameT`) — 16-byte inline Copy values (tag + 8-byte concrete ref). `INameT` (the widest union) lives inline too — in `IdT.local_name`, in `IdT.init_steps: &'t [INameT<'s, 't>]` elements, and anywhere a typed name is passed by value. Casting a concrete or narrow sub-enum into a wider sub-enum (or into `INameT`) is a stack-only rewrap: no interner, no arena allocation. Identity between two sub-enum values (including `INameT`) is structural compare on the 16 bytes; identity between concrete refs stays pointer-eq because concrete name structs remain interned.
- `CoordT<'s, 't>` — passed by value, pointer-eq on `kind` field
- `OwnershipT`, `MutabilityT`, `VariabilityT`, `LocationT`, `RegionT` — pure Copy enums
- Small templata value variants: `MutabilityTemplataT`, `VariabilityTemplataT`, `OwnershipTemplataT`, `RuntimeSizedArrayTemplateTemplataT`, `StaticSizedArrayTemplateTemplataT`
- `Integer(i64)`, `Boolean(bool)` variants of `ITemplataT`

**Neither arena (stack / heap-Vec / HashMap):**
- `CompilerOutputs<'s, 't>` — stack-owned accumulator, dies at pass end
- `Compiler<'s, 'ctx, 't>` — stack god struct
- `TemplatasStoreT` builders — `NodeEnvironmentBuilder`, etc., stack-local with heap `Vec`s until `build_in(scout_arena)` freezes into `'s`
- `DeferredActionT` entries in `VecDeque` — owned structs, not `Box<dyn>`

### 1.6 Mutual-Recursion Shape

The interned type graph is mutually recursive but every edge is an `&'s` or `&'t` ref — pointer-sized, finite. No `Box`/`Vec` needed in arena types.

```
CoordT → KindT → StructTT → IdT → INameT → ITemplataT → CoordT
                   ↓
              (also CoordT → KindT directly via CoordT.kind field)
```

Same-type self-recursion (e.g. `ForwarderFunctionNameT.inner: &'t IFunctionNameT<'s, 't>`, `ForwarderFunctionTemplateNameT.inner: &'t IFunctionTemplateNameT<'s, 't>`) is resolved by the `&'t` indirection — no `Box`.

**Ordering implication for Slab filling:** since every edge is a reference, you can declare types in any order within a slab — forward declaration is free. But for human review and staged compilability, order from leaves upward: primitives → `RegionT`/`CoordT` inline → `IdT` generic → `INameT` hierarchy → non-primitive `KindT` variants → `ITemplataT` → prototype/signature → envs → expressions.

---

## Part 2: The God Struct

### 2.1 Architecture

All Scala sub-compilers collapse into a single `Compiler` struct:

```rust
pub struct Compiler<'s, 'ctx, 't> {
    pub scout_arena: &'ctx ScoutArena<'s>,
    pub typing_interner: &'ctx TypingInterner<'t>,
    pub keywords: &'ctx Keywords<'s>,  // scout-interned keywords are fine
    pub opts: &'ctx TypingPassOptions<'s>,
}
```

Immutable configuration only. Mutable state threads through `&mut CompilerOutputs<'s, 't>` on every call.

**Not on the god struct:**
- `global_env` — Scala builds `globalEnv` as a local inside `compile()`, not as a constructor arg. We do the same: the top-level env is a method parameter on `compile_program` (and anything downstream that needs it), not a field.
- `name_translator` — Scala's `NameTranslator` is a pure helper class with no state; its methods just translate postparser names through the interner. In Rust, every `Compiler` method already has `self.scout_arena` and `self.typing_interner`, so the helper adds nothing. Its ~6 translate methods move directly onto `impl Compiler` and the struct is deleted.

### 2.2 `&self` + `&mut coutputs` Enables Re-entrancy

Deep mutual recursion works because `&self` allows shared borrow, and `&mut coutputs` is re-borrowed at each call level.

### 2.3 Macros

Macros (`AsSubtypeMacro`, `LockWeakMacro`, `StructDropMacro`, etc.) were stateful classes in Scala (holding `keywords`, `expressionCompiler`, etc.). In the god-struct refactor, all that state is already on `Compiler`. So macro structs disappear entirely and their methods become ordinary methods on `Compiler`.

Dispatch is via a small Copy unit-variant enum per Scala macro trait — `FunctionBodyMacro`, `OnStructDefinedMacro`, `OnInterfaceDefinedMacro`, `OnImplDefinedMacro`. Each variant tags which `Compiler` method to call.

```rust
pub enum FunctionBodyMacro {
    LockWeak,
    AsSubtype,
    StructDrop,
    StructConstructor,
    // ... one variant per Scala class extending IFunctionBodyMacro
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't {
    // Individual method per Scala class, name-suffixed to avoid collisions.
    pub fn generate_function_body_lock_weak(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &FunctionEnvironmentT<'s, 't>,
        // ...
    ) -> (FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>) { ... }

    // Central dispatcher for dynamic lookups out of the env.
    pub fn dispatch_function_body_macro(
        &self,
        which: FunctionBodyMacro,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &FunctionEnvironmentT<'s, 't>,
        // ...
    ) -> (FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>) {
        match which {
            FunctionBodyMacro::LockWeak => self.generate_function_body_lock_weak(coutputs, env, /* ... */),
            FunctionBodyMacro::AsSubtype => self.generate_function_body_as_subtype(coutputs, env, /* ... */),
            // ...
        }
    }
}
```

A given macro may appear in multiple enums iff Scala had it extending multiple traits (e.g. `StructDropMacro extends IFunctionBodyMacro with IOnStructDefinedMacro` → appears in both `FunctionBodyMacro` and `OnStructDefinedMacro`). Globally the shared implementation is a single `Compiler` method per Scala method, reused across the dispatcher matches.

### 2.4 Delegate Traits Eliminated

In Scala, sub-compilers wired via delegate traits. With the god struct, every method calls `self.method(...)` directly. No `IExpressionCompilerDelegate`, `IInfererDelegate`, etc.

---

## Part 3: Environments

### 3.1 Arena-Allocated In `'s`

Environments are allocated directly into the scout arena. They reference scout data (`FunctionA`, `StructA`) freely because they live in the same arena.

```rust
pub enum IEnvironmentT<'s, 't> {
    Package(PackageEnvironmentT<'s, 't>),
    Citizen(CitizenEnvironmentT<'s, 't>),
    Function(FunctionEnvironmentT<'s, 't>),
    Node(NodeEnvironmentT<'s, 't>),
    BuildingWithClosureds(BuildingFunctionEnvironmentWithClosuredsT<'s, 't>),
    BuildingWithClosuredsAndTemplateArgs(BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>),
    General(GeneralEnvironmentT<'s, 't>),
    Export(ExportEnvironmentT<'s, 't>),
    Extern(ExternEnvironmentT<'s, 't>),
}

pub struct NodeEnvironmentT<'s, 't> {
    pub parent: &'s IEnvironmentT<'s, 't>,
    pub parent_function_env: &'s FunctionEnvironmentT<'s, 't>,
    pub life: LocationInFunctionEnvT<'s>,  // scout-lifetimed
    pub templatas: TemplatasStoreT<'s, 't>,
    pub declared_locals: &'s [&'s ILocalVariableT<'s, 't>],
    pub function: &'s FunctionA<'s>,  // direct ref; fine because we live in 's
    // ...
}
```

### 3.2 `TemplatasStoreT` Uses Arena-Backed Slice Pairs

Following AASSNCMCX, we avoid heap HashMap inside arena types:

```rust
pub struct TemplatasStoreT<'s, 't> {
    pub name_to_entry: &'s [(INameT<'s, 't>, IEnvEntryT<'s, 't>)],
    pub imprecise_to_entries: &'s [(&'s IImpreciseNameS<'s>, &'s [IEnvEntryT<'s, 't>])],
    pub id: &'t IdT<'s, 't>,
}
```

Unsorted slices allocated in the scout arena. Lookup via linear scan (`.iter().find(...)`). Common-case scope size is small (~5–10 entries), where linear scan beats binary search on cache behavior and avoids the sort cost at construction. If profiling later identifies a hot slow scope (a package-level env with hundreds of entries, e.g.), switch that specific env kind to sorted-binary — but not as a default.

### 3.3 Mutable Building Phase

During construction, an env is mutable. Builders live on the stack with heap `Vec`s; freeze into the scout arena when done:

```rust
pub struct NodeEnvironmentBuilder<'s, 't> {
    pub parent: &'s IEnvironmentT<'s, 't>,
    pub parent_function_env: &'s FunctionEnvironmentT<'s, 't>,
    pub life: LocationInFunctionEnvT<'s>,
    pub templatas_pending: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)>,
    pub declared_locals_pending: Vec<&'s ILocalVariableT<'s, 't>>,
}

impl<'s, 't> NodeEnvironmentBuilder<'s, 't> {
    pub fn build_in(self, scout_arena: &ScoutArena<'s>) -> &'s NodeEnvironmentT<'s, 't> {
        // Copy pending entries into arena slices, construct final NodeEnvironmentT,
        // allocate into scout arena. Note: `&ScoutArena<'s>` (elided borrow lifetime),
        // not `&'s ScoutArena<'s>` — see §1.2 invariant 5.
        let templatas = TemplatasStoreT {
            name_to_entry: scout_arena.alloc_slice_from_iter(...),
            imprecise_to_entries: scout_arena.alloc_slice_from_iter(...),
            id: ...,
        };
        scout_arena.alloc(NodeEnvironmentT {
            parent: self.parent,
            parent_function_env: self.parent_function_env,
            life: self.life,
            templatas,
            declared_locals: scout_arena.alloc_slice_copy(&self.declared_locals_pending),
            function: self.parent_function_env.function,
        })
    }
}
```

### 3.4 Transient Reads Use `&IEnvironmentT`

Scala's `NodeEnvironmentBox.snapshot` is called ~36 times in ExpressionCompiler.scala, mostly for transient reads (passing an `IInDenizenEnvironmentT` to helpers like `isTypeConvertible`).

In Rust, we distinguish:
- **`&IEnvironmentT<'_, 's, 't>`** (elided lifetime) — read-only borrow, zero cost, for helpers that only inspect.
- **`&'s IEnvironmentT<'s, 't>`** — arena-pinned, needed when storing into a parent pointer, side table, or output.

Promotion to `&'s` happens only at explicit "store this env" points (via `build_in(scout_arena)` on a builder). Transient reads just use `&`.

### 3.5 `FunctionTemplata` Is A Plain Computed Value

Matches Scala directly:

```rust
impl<'s, 't> FunctionEnvironmentT<'s, 't> {
    pub fn templata(&self) -> FunctionTemplataT<'s, 't> {
        FunctionTemplataT {
            outer_env: self.parent,
            function: self.function,
        }
    }
}
```

No caching, no `OnceCell`, no ID minting, no side-table insertion. `FunctionTemplataT` is a small struct holding two pointers — trivially cheap to construct on every call.

### 3.6 Env-ID Uniqueness Not Required

Environments don't need unique `id` fields per instance. No HashMap keys on env's own id anywhere in the design:
- `CompilerOutputs.function_name_to_outer_env` etc. are keyed by **function/type template ids** (genuinely unique per Scala's `vassert`).
- OverloadSet holds the env by direct reference, not by id.
- Heavy templatas hold refs, not ids.

Scala's env-id collisions (NodeEnvironment delegating to parent, GeneralEnvironment reusing caller ids, Box wrappers sharing ids) translate as-is.

---

## Part 4: CompilerOutputs

### 4.1 Structure

Mutable accumulator threaded through the pass. Carries `<'s, 't>`:

```rust
pub struct CompilerOutputs<'s, 't> {
    // Function registries
    pub return_types_by_signature: HashMap<PtrKey<'t, SignatureT<'s, 't>>, CoordT<'s, 't>>,
    pub signature_to_function: HashMap<PtrKey<'t, SignatureT<'s, 't>>, &'t FunctionDefinitionT<'s, 't>>,
    
    // Declaration tracking
    pub function_declared_names: HashMap<PtrKey<'t, IdT<'s, 't>>, RangeS<'s>>,
    pub type_declared_names: HashSet<PtrKey<'t, IdT<'s, 't>>>,
    
    // Env storage (keyed by function/type template id, per Scala parity)
    pub function_name_to_outer_env: HashMap<PtrKey<'t, IdT<'s, 't>>, &'s IEnvironmentT<'s, 't>>,
    pub function_name_to_inner_env: HashMap<PtrKey<'t, IdT<'s, 't>>, &'s IEnvironmentT<'s, 't>>,
    pub type_name_to_outer_env: HashMap<PtrKey<'t, IdT<'s, 't>>, &'s IEnvironmentT<'s, 't>>,
    pub type_name_to_inner_env: HashMap<PtrKey<'t, IdT<'s, 't>>, &'s IEnvironmentT<'s, 't>>,
    
    // Type metadata
    pub type_name_to_mutability: HashMap<PtrKey<'t, IdT<'s, 't>>, ITemplataT<'s, 't>>,
    pub interface_name_to_sealed: HashMap<PtrKey<'t, IdT<'s, 't>>, bool>,
    
    // Definitions
    pub struct_template_name_to_definition: HashMap<PtrKey<'t, IdT<'s, 't>>, &'t StructDefinitionT<'s, 't>>,
    pub interface_template_name_to_definition: HashMap<PtrKey<'t, IdT<'s, 't>>, &'t InterfaceDefinitionT<'s, 't>>,
    
    // Impls + reverse indexes
    pub all_impls: HashMap<PtrKey<'t, IdT<'s, 't>>, &'t ImplT<'s, 't>>,
    // Exceptions to §4.3 copy-out invariant. Access via mem::take / drain + reinsert.
    pub sub_citizen_template_to_impls: HashMap<PtrKey<'t, IdT<'s, 't>>, Vec<&'t ImplT<'s, 't>>>,
    pub super_interface_template_to_impls: HashMap<PtrKey<'t, IdT<'s, 't>>, Vec<&'t ImplT<'s, 't>>>,
    
    // Exports/externs
    pub kind_exports: Vec<&'t KindExportT<'s, 't>>,
    pub function_exports: Vec<&'t FunctionExportT<'s, 't>>,
    pub kind_externs: Vec<&'t KindExternT<'s, 't>>,
    pub function_externs: Vec<&'t FunctionExternT<'s, 't>>,
    
    // Instantiation bounds
    pub instantiation_name_to_bounds: HashMap<PtrKey<'t, IdT<'s, 't>>, &'t InstantiationBoundArgumentsT<'s, 't>>,
    
    // Deferred evaluation queues
    pub deferred_actions: VecDeque<DeferredActionT<'s, 't>>,
    pub finished_deferred_function_body_compiles: HashSet<PtrKey<'t, PrototypeT<'s, 't>>>,
    pub finished_deferred_function_compiles: HashSet<PtrKey<'t, IdT<'s, 't>>>,
}
```

No origin side tables — heavy templatas and OverloadSets hold refs directly.

### 4.2 `PtrKey<'t, T>` Newtype For HashMap Keys

Interned `&'t T` refs hash by pointer identity, not structural equality. Newtype wrapper gives the custom `Hash`/`Eq`.

```rust
pub struct PtrKey<'t, T: ?Sized>(pub &'t T);

impl<'t, T: ?Sized> PartialEq for PtrKey<'t, T> {
    fn eq(&self, other: &Self) -> bool { std::ptr::eq(self.0, other.0) }
}
impl<'t, T: ?Sized> Eq for PtrKey<'t, T> {}
impl<'t, T: ?Sized> Hash for PtrKey<'t, T> {
    fn hash<H: Hasher>(&self, state: &mut H) { (self.0 as *const T as *const ()).hash(state) }
}
impl<'t, T: ?Sized> Copy for PtrKey<'t, T> {}
impl<'t, T: ?Sized> Clone for PtrKey<'t, T> { fn clone(&self) -> Self { *self } }
```

### 4.3 Side-Table Access Pattern — Copy Out Before &mut

The remaining side tables (env maps like `function_name_to_outer_env`) still need the copy-out pattern:

```rust
// Does not compile:
let env = coutputs.function_name_to_outer_env.get(&PtrKey(id)).unwrap();
self.do_stuff(coutputs, env, ...);  // &mut coutputs rejected; env borrows from it

// Works — copy out first:
let env: &'s IEnvironmentT<'s, 't> = *coutputs.function_name_to_outer_env.get(&PtrKey(id)).unwrap();
self.do_stuff(coutputs, env, ...);  // OK; env is Copy'd out
```

**Invariant: most HashMap values in `CompilerOutputs` are pointer-sized `Copy` types** (`&'s T`, `&'t T`), enabling the copy-out-then-mutate pattern.

**Two exceptions:** `sub_citizen_template_to_impls` and `super_interface_template_to_impls` hold `Vec<&'t ImplT<'s, 't>>` because the reverse-index lookups return multiple impls per key. Access these with `mem::take` / `drain` + reinsert, not by-reference read. The `Vec` is cheaply movable (pointer-sized entries) so take-and-restore is fine.

### 4.4 Deferred Actions

Avoid `Box<dyn FnOnce>` via structured enum:

```rust
pub enum DeferredActionT<'s, 't> {
    EvaluateFunctionBody {
        prototype: &'t PrototypeT<'s, 't>,
        function_env: &'s FunctionEnvironmentT<'s, 't>,
        origin: &'s FunctionA<'s>,
    },
    EvaluateFunction {
        name: &'t IdT<'s, 't>,
        calling_env: &'s IEnvironmentT<'s, 't>,
        origin: &'s FunctionA<'s>,
        template_args: &'t [ITemplataT<'s, 't>],
    },
    // Add variants as needed.
}
```

Drain loop matches on the variant. No `Box`, no `dyn`, no hidden captures.

**Invariant:** `DeferredActionT` variants hold only owned or `Copy` context (`&'s` and `&'t` refs, small values). They never reference `CompilerOutputs` itself. This preserves the drain pattern (`pop_front()` → match → call method with `&mut coutputs`) without self-borrow hazards — once the action is popped out, it no longer aliases anything inside `coutputs`, so passing `&mut coutputs` into the handler is safe.

### 4.5 Speculative Writes Are Idempotent

During overload resolution, `attempt_candidate_banner` writes to `coutputs` even for candidates that may be rejected. Safe because writes are idempotent — re-writing asserts equality.

---

## Part 5: OverloadSet

### 5.1 Transient Kind, But Holds Env Directly

- Overload resolution always completes during the typing pass; nothing unresolved reaches the instantiator.
- But an `OverloadSet`-kinded `ReinterpretTE` survives in output as an inert type tag (the instantiator elides it via `InstantiationBoundArgumentsT.runeToFunctionBoundArg`).

Since `'s` lives through instantiation, we hold the env directly — no `ctx_id` indirection needed:

```rust
/// Reachable from HinputsT only inside inert ReinterpretTE args that
/// the instantiator drops on sight. Never dereference env/name during
/// instantiation — they are type-tag placeholders only.
pub struct OverloadSetT<'s, 't> {
    pub env: &'s IEnvironmentT<'s, 't>,
    pub name: &'s IImpreciseNameS<'s>,  // scout-interned, fine to hold
}

pub enum KindT<'s, 't> {
    // ... other variants ...
    OverloadSet(&'t OverloadSetT<'s, 't>),
}
```

### 5.2 Construction

Matches Scala directly:

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
    pub fn make_overload_set_kind(
        &self,
        env: &'s IEnvironmentT<'s, 't>,
        name: &'s IImpreciseNameS<'s>,
    ) -> &'t KindT<'s, 't> {
        let overload_set = OverloadSetT { env, name };
        self.typing_interner.intern_kind(KindValT::OverloadSet(overload_set))
    }
}
```

No side-table insertion, no `ctx_id`, no `&mut coutputs` required for the construction itself.

### 5.3 Overload Resolution Uses The Env Directly

The resolver reads `overload_set.env` directly, walks its parent chain, looks up candidates, recurses into nested OverloadSets via their own `env` fields. Matches Scala's `OverloadResolver.scala:210` pattern verbatim. No signature changes to `getCandidateBannersInner` etc. — `&CompilerOutputs` is still threaded through the god-struct methods, but not specifically for env lookup.

### 5.4 Post-Pass

`OverloadSetT<'s, 't>` survives in `HinputsT<'s, 't>` inside inert `ReinterpretTE` args. Its `env` and `name` refs remain valid for as long as `'s` lives. The instantiator elides these on sight via `InstantiationBoundArgumentsT.runeToFunctionBoundArg` and never dereferences them — they're type-tag placeholders.

After the instantiator completes, the caller drops the typing interner (`'t` dies), then the scout arena (`'s` dies), in that order (§1.3). The OverloadSet's refs are never dangling during any live use; after both arenas drop, the entire `HinputsT` is unreachable.

---

## Part 6: Type System Types

Every lifetime-parameterized struct in this section has an implicit `where 's: 't` bound (§1.2). Representative examples are spelled out on `IdT`, `CoordT`, `KindT`, and `ITemplataT` below; others follow the same convention.

### 6.1 IDEPFL Dual-Enum Pattern

All interned typing types use the dual-enum pattern: a **reference enum** (canonical, `&'t` refs) and a **value enum** (transient, HashMap lookup). Scout-lifetimed values (`StrI<'s>`, `IImpreciseNameS<'s>`, `RangeS<'s>`, etc.) are used directly wherever they appear — no re-interning boundary.

Interned type families:
- Concrete name structs (`FunctionNameT`, `StructNameT`, `ImplNameT`, … ~60 of them) / their respective `*ValT` lookup keys
- `IdT<'s, 't, T>` / `IdValT<'s, 't, T>`
- `KindT<'s, 't>` / `KindValT<'s, 't>`
- `ITemplataT<'s, 't>` / `ITemplataValT<'s, 't>`
- `PrototypeT<'s, 't>` / `PrototypeValT<'s, 't>`
- `SignatureT<'s, 't>` / `SignatureValT<'s, 't>`

**All name enums (`INameT` + its 21 sub-enum families like `IFunctionNameT`, `ICitizenNameT`, etc.) are NOT interned.** They're inline-owned 16-byte `Copy` values — tag + inner concrete ref — built on the stack. See §6.2 for the rationale (sub-enum casts are free, no per-wrapper arena allocation). Only the ~60 concrete name structs get arena storage on the name side. `IdT.init_steps` is a `&'t [INameT<'s, 't>]` arena slice of inline `INameT` values (not a slice of refs).

**No `'t`-lifetime re-interned versions of `StrI`, `IImpreciseNameS`, `RangeS`, `CodeLocationS`, `PackageCoordinate`, `FileCoordinate`.** Use the scout-arena versions directly. Anywhere you'd be tempted to create a `StrI<'t>` or `RangeT<'t>`, use the existing `StrI<'s>` or `RangeS<'s>` instead.

### 6.2 INameT Hierarchy

~60 concrete name types, ~21 sub-trait enums. Every name type carries `<'s, 't>`.

**DAG rule (critical).** Scala's sealed-trait hierarchy is a DAG — some concrete names extend multiple sub-traits (`ExternFunctionNameT extends IFunctionNameT with IFunctionTemplateNameT`, etc.). In Rust this becomes: **a shared concrete name gets a variant in each sub-enum it belongs to.** Example:

```rust
pub enum IFunctionNameT<'s, 't> {
    Function(&'t FunctionNameT<'s, 't>),
    ForwarderFunction(&'t ForwarderFunctionNameT<'s, 't>),
    ExternFunction(&'t ExternFunctionNameT<'s, 't>),  // also appears below
    // ...
}

pub enum IFunctionTemplateNameT<'s, 't> {
    FunctionTemplate(&'t FunctionTemplateNameT<'s, 't>),
    ExternFunction(&'t ExternFunctionNameT<'s, 't>),  // shared with IFunctionNameT
    ForwarderFunctionTemplate(&'t ForwarderFunctionTemplateNameT<'s, 't>),
    // ...
}
```

**Sub-enums are inline-owned Copy values, NOT arena-interned.** Only the concrete name types (wrapped in the variants above) and `INameT` itself (the widest union of all ~60 concretes) live in the typing arena. The intermediate sub-enums (`IFunctionNameT`, `IFunctionTemplateNameT`, `ICitizenNameT`, etc. — 21 families) are 16-byte `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]` values built on the stack when needed. Casting a concrete into a sub-enum, or a narrow sub-enum into a wider one, is a pure stack-only rewrap via `.into()` — no interner round-trip.

The consequence for identity: two concrete names (e.g. two `&'t FunctionNameT`) are compared via `ptr::eq` since concretes stay interned. Two owned sub-enum values (e.g. two `IFunctionNameT<'s, 't>`, or two `INameT<'s, 't>`) are compared structurally — but since the tag + inner pointer is 16 bytes, this is a 2-word compare and still cheap.

Bridging via `From<X> for SubEnum` (narrow → wide, infallible — concrete into sub-enum and narrow sub-enum into wider sub-enum) and `TryFrom<INameT<'s, 't>> for SubEnum` (wide → narrow, fallible, match-and-rewrap on the stack). All of these are real implementations in `names.rs`, no interner involvement.

**Self-referential variants** (e.g. `ForwarderFunctionNameT.inner: IFunctionNameT<'s, 't>`, `ForwarderFunctionTemplateNameT.inner: IFunctionTemplateNameT<'s, 't>`) are inline-owned; since sub-enums are 16 bytes and Copy they can live directly as struct fields without `Box` or `&'t`.

No new name variants needed for env handling (env-id uniqueness isn't required).

### 6.3 `IdT<'s, 't, T>` — Generic, Interned

```rust
pub struct IdT<'s, 't, T: Copy>
where 's: 't,
{
    pub package_coord: &'s PackageCoordinate<'s>,  // scout-lifetimed
    pub init_steps: &'t [INameT<'s, 't>],  // inline INameT values
    pub local_name: T,
}
```

Both `init_steps` elements and `local_name` hold their name representation **inline by value**, not behind `&'t`. For a function's id, `T = IFunctionNameT<'s, 't>`; for a struct's id, `T = IStructNameT<'s, 't>`; for the widest form, `T = INameT<'s, 't>`. Each `INameT` / sub-enum value is 16 bytes (tag + 8-byte inner concrete ref). `init_steps` is an arena slice that the typing interner canonicalizes as a whole: two IdTs built with structurally-identical init_steps sequences share the same `&'t [INameT]` allocation.

Only `T: Copy` on the struct itself — keep bounds minimal so `IdT` is cheap to mention in signatures elsewhere. Conversion bounds (`Into<INameT<'s, 't>>`, `TryInto<...>`) live on the impl blocks for the conversion methods:

```rust
impl<'s, 't, T: Copy + Into<INameT<'s, 't>>> IdT<'s, 't, T>
where 's: 't,
{
    pub fn widen(self) -> IdT<'s, 't, INameT<'s, 't>> { ... }
}

impl<'s, 't, T: Copy> IdT<'s, 't, T>
where 's: 't,
{
    pub fn widen_to<U: Copy>(self) -> IdT<'s, 't, U>
    where T: Into<U> { ... }
}

impl<'s, 't> IdT<'s, 't, INameT<'s, 't>>
where 's: 't,
{
    pub fn try_narrow<U: Copy>(self) -> Option<IdT<'s, 't, U>>
    where INameT<'s, 't>: TryInto<U> { ... }
}
```

`widen`, `widen_to`, and `try_narrow` are all real, no interner dependency — they just rewrap the inline `local_name` via its `From`/`TryFrom` impl and reuse the same `package_coord` and `init_steps` pointers.

Generic parameter preserves leaf-kind info at the type level (e.g. `IdT<'s, 't, IFunctionNameT<'s, 't>>` vs `IdT<'s, 't, IStructTemplateNameT<'s, 't>>`).

Since sub-enum identity is structural (not pointer-eq), `IdT` defines a **custom** `PartialEq`/`Eq`/`Hash` rather than using derive:

- `package_coord`: pointer-eq (scout arena canonicalizes `PackageCoordinate`).
- `init_steps`: slice data pointer + length compare (the typing interner canonicalizes `&'t [INameT]` slices per IDEPFL — equal content ⇒ equal slice pointer).
- `local_name`: structural compare on the inline `T` (16-byte sub-enum, `INameT`, or concrete ref).

Interning uses one HashMap keyed by the widest form (`IdValT<'s, 't, INameT<'s, 't>>`); specialized `IdT<'s, 't, IXxxNameT<'s, 't>>` views are type-cast from the widest-form arena storage (unsafe at the interner boundary since `IdT`'s layout is T-independent at the byte level for any T that is `Copy` + same size as the widened-form's inline enum).

### 6.4 `CoordT<'s, 't>` — Inline Copy

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CoordT<'s, 't> {
    pub ownership: OwnershipT,
    pub region: RegionT,
    pub kind: &'t KindT<'s, 't>,
}
```

Small (3 fields, Copy). Passed by value. Not interned — structural eq, pointer-eq on kind.

### 6.5 `KindT<'s, 't>` — Interned

Every variant carries a payload struct, even the primitives — uniformity makes pattern matching and visitor-pattern code simpler to write mechanically:

```rust
pub enum KindT<'s, 't> {
    Never(NeverT),                                  // NeverT { from_break: bool } per Scala
    Void(VoidT),                                    // zero-sized unit struct
    Int(IntT),                                      // IntT { bits: u8 } per Scala
    Bool(BoolT),                                    // zero-sized unit struct
    Str(StrT),                                      // zero-sized unit struct
    Float(FloatT),                                  // zero-sized unit struct
    Struct(StructTT<'s, 't>),
    Interface(InterfaceTT<'s, 't>),
    StaticSizedArray(StaticSizedArrayTT<'s, 't>),
    RuntimeSizedArray(RuntimeSizedArrayTT<'s, 't>),
    KindPlaceholder(KindPlaceholderT<'s, 't>),
    OverloadSet(&'t OverloadSetT<'s, 't>),
}

pub struct NeverT { pub from_break: bool }
pub struct VoidT;
pub struct IntT { pub bits: u8 }
pub struct BoolT;
pub struct StrT;
pub struct FloatT;
```

All variants interned. Pointer-eq on `&'t KindT` is identity.

### 6.6 `ITemplataT<'s, 't>` — Type Parameter Erased, No Split Needed

All variants are equally free to hold `&'s` refs:

```rust
pub enum ITemplataT<'s, 't> {
    // Leaf-like
    Coord(&'t CoordTemplataT<'s, 't>),
    Kind(&'t KindTemplataT<'s, 't>),
    Placeholder(&'t PlaceholderTemplataT<'s, 't>),
    Mutability(MutabilityTemplataT),
    Variability(VariabilityTemplataT),
    Ownership(OwnershipTemplataT),
    Integer(i64),
    Boolean(bool),
    String(&'s StrI<'s>),  // scout-lifetimed StrI is fine
    Prototype(&'t PrototypeTemplataT<'s, 't>),
    Isa(&'t IsaTemplataT<'s, 't>),
    CoordList(&'t CoordListTemplataT<'s, 't>),
    RuntimeSizedArrayTemplate(RuntimeSizedArrayTemplateTemplataT),
    StaticSizedArrayTemplate(StaticSizedArrayTemplateTemplataT),
    
    // Heavy — carry direct &'s refs, like Scala
    Function(&'t FunctionTemplataT<'s, 't>),
    StructDefinition(&'t StructDefinitionTemplataT<'s, 't>),
    InterfaceDefinition(&'t InterfaceDefinitionTemplataT<'s, 't>),
    ImplDefinition(&'t ImplDefinitionTemplataT<'s, 't>),
    ExternFunction(&'t ExternFunctionTemplataT<'s, 't>),
}

pub struct FunctionTemplataT<'s, 't> {
    pub outer_env: &'s IEnvironmentT<'s, 't>,
    pub function: &'s FunctionA<'s>,
}

pub struct StructDefinitionTemplataT<'s, 't> {
    pub declaring_env: &'s IEnvironmentT<'s, 't>,
    pub origin_struct: &'s StructA<'s>,
}

pub struct InterfaceDefinitionTemplataT<'s, 't> {
    pub declaring_env: &'s IEnvironmentT<'s, 't>,
    pub origin_interface: &'s InterfaceA<'s>,
}

pub struct ImplDefinitionTemplataT<'s, 't> {
    pub declaring_env: &'s IEnvironmentT<'s, 't>,
    pub impl_a: &'s ImplA<'s>,
}

pub struct ExternFunctionTemplataT<'s, 't> {
    pub header: &'t FunctionHeaderT<'s, 't>,
}
```

No ID indirection, no side tables, no slot-constraint enforcement. `FunctionHeaderT.maybe_origin_function_templata` and `ImplT.templata` and `FunctionBannerT.origin_function_templata` all just hold the heavy templata directly.

**`PrototypeTemplataT`'s own `T` parameter is orthogonal.** Scala's `ITemplataT[+T <: ITemplataType]` outer parameter is erased (handled by variant tag + runtime `tyype` field on `PlaceholderTemplataT`). But `PrototypeTemplataT[T <: IFunctionNameT]` has a *separate* inner type parameter tracking which kind of function-name the prototype points at. Decide that parameter independently — most likely: keep it as `PrototypeTemplataT<'s, 't>` with `prototype: &'t PrototypeT<'s, 't>` where `PrototypeT` is generic in its leaf name (`PrototypeT<'s, 't, T: Copy = ()>` with `id: IdT<'s, 't, T>`, T holding the leaf sub-enum inline). Don't confuse the two erasures.

**Mixed-mode equality.** `ITemplataT` deliberately mixes Copy-value variants (`Mutability`, `Variability`, `Ownership`, `Integer`, `Boolean`) with `&'t`-ref variants (`Coord`, `Kind`, `Prototype`, heavy templatas, …). Value variants compare by value; ref variants compare by pointer identity (the typing interner guarantees uniqueness). `ITemplataValT` mirrors this split for HashMap lookup. When an `ITemplataT` itself lives behind `&'t ITemplataT` and is used as a key, wrap in `PtrKey<'t, ITemplataT>` (pointer-eq on the whole value); when it's used by value, the derived `PartialEq`/`Hash` does the right thing per-variant.

### 6.7 Mutual Recursion

Types form a mutually recursive graph (`CoordT → KindT → StructTT → IdT → INameT → ITemplataT → CoordT`). Most links are `&'s` or `&'t` refs — pointer-sized, finite. The name sub-enums (`IFunctionNameT`, `IStructNameT`, etc.) are the exception — they're inline 16-byte tagged pointers, one word larger than a raw ref but still finite and `Copy`. No `Box`/`Vec` needed; arena references + inline sub-enums provide the indirection.

---

## Part 7: Expression AST

### 7.1 Three Enums

```rust
pub enum ReferenceExpressionTE<'s, 't> {
    LetNormal(LetNormalTE<'s, 't>),
    If(IfTE<'s, 't>),
    While(WhileTE<'s, 't>),
    FunctionCall(FunctionCallTE<'s, 't>),
    Consecutor(ConsecutorTE<'s, 't>),
    Construct(ConstructTE<'s, 't>),
    Reinterpret(ReinterpretTE<'s, 't>),
    // ... ~30 more variants
}

pub enum AddressExpressionTE<'s, 't> {
    LocalLookup(LocalLookupTE<'s, 't>),
    ReferenceMemberLookup(ReferenceMemberLookupTE<'s, 't>),
    // ... ~4 more
}

pub enum ExpressionTE<'s, 't> {
    Reference(&'t ReferenceExpressionTE<'s, 't>),
    Address(&'t AddressExpressionTE<'s, 't>),
}
```

Used wherever a slot can hold either (e.g. `ConstructTE.args`). All other slots are specifically `&'t ReferenceExpressionTE` or `&'t AddressExpressionTE`.

**Narrow-use rule.** `ConstructTE` is the **only** expression node that holds the broad `ExpressionTE` wrapper (because closure structs can have addressible members mixed with reference-valued ones). Every other slot in every other node uses the specific `&'t ReferenceExpressionTE` or `&'t AddressExpressionTE` — no `ExpressionTE` wrapping. When filling expression definitions, default to the narrow type; only reach for `ExpressionTE` when the Scala field was typed as the broader `ExpressionT` AND the node mixes both in a collection.

### 7.2 Arena-Allocated, Not Interned

Expression nodes are allocated into `'t` but not deduped. Sub-expressions are `&'t` refs; collections are arena slices.

### 7.3 Can Reference Scout Data

TE nodes can freely hold `&'s` refs — e.g., if an expression's type metadata includes a `RangeS<'s>` source location, that's fine.

### 7.4 Visitor/Collector Pattern

Same as parsing/postparsing: `NodeRefT<'s, 't>` enum, `visit_*` functions, entry points, `collect_where_tnodes!`/`collect_only_tnodes!` macros.

---

## Part 8: Error Handling

### 8.1 `Result<T, CompileErrorT>` With `?`

Scala's `throw CompileErrorExceptionT(...)` → `return Err(CompileErrorT::...)` with `?` propagation. Single catch boundary at top-level `Compiler::compile()`.

### 8.2 Panic For Unimplemented

`panic!()` with unique identifying messages for unmigrated branches. Fully-stub functions acceptable during incremental migration.

---

## Part 9: Keywords

`Keywords<'s>` — re-uses scout arena for interned strings. Same Keywords instance can serve the typing pass and (if needed) subsequent passes as long as `'s` lives. No typing-specific `Keywords<'t>` needed.

---

## Part 10: Driving The Pass

### 10.1 Top-Level Entry

```rust
pub fn run_typing_pass<'s, 'ctx, 't>(
    scout_arena: &'ctx ScoutArena<'s>,
    typing_interner: &'ctx TypingInterner<'t>,
    keywords: &'ctx Keywords<'s>,
    opts: &'ctx TypingPassOptions<'s>,
    program_a: &'s ProgramA<'s>,
) -> Result<HinputsT<'s, 't>, CompileErrorT<'s, 't>>
where 's: 't,
{
    let compiler = Compiler::new(scout_arena, typing_interner, keywords, opts);
    let mut coutputs = CompilerOutputs::new();
    
    compiler.compile_program(&mut coutputs, program_a)?;
    compiler.drain_all_deferred(&mut coutputs);
    
    let hinputs = HinputsT {
        function_definitions: typing_interner.alloc_slice_iter(
            coutputs.signature_to_function.into_values()
        ),
        struct_definitions: typing_interner.alloc_slice_iter(
            coutputs.struct_template_name_to_definition.into_values()
        ),
        // ... etc
    };
    
    Ok(hinputs)
    // coutputs drops; its HashMaps (storing 's/'t refs) die cheaply
    // scout_arena and typing_interner survive; instantiator can use them
}
```

No `finalize()`. Natural scope exit.

### 10.2 Instantiator Handoff

The instantiator receives `HinputsT<'s, 't>` plus both arena references. When it's done, the caller lets local bindings drop in scope order (see §1.3): typing interner first, scout arena second, parse arena last.

---

## Part 11: Invariants Summary

1. **`'s` outlives `'t`.** Declared via `where 's: 't` on every type that transitively holds `&'s` data. Rust does not enforce outlives via drop order; the bound must be written.
2. **Arena types never contain `Vec`, `HashMap`, `String`, `Rc`, `Box`.** AASSNCMCX applies. Use arena slices.
3. **All HashMap keys on interned refs use `PtrKey<'t, T>`** for pointer-based hash/eq.
4. **Most `CompilerOutputs` HashMap values are pointer-sized `Copy` refs** (`&'s T`, `&'t T`) — enables the copy-out-then-mutate pattern. Exceptions: `sub_citizen_template_to_impls` and `super_interface_template_to_impls` hold `Vec<&'t ImplT>`; access via `mem::take` / `drain` + reinsert.
5. **Speculative writes are idempotent.** No rollback machinery.
6. **Overload resolution always completes during the typing pass.** No unresolved overloads reach the instantiator.
7. **Env equality/hashing never used.** Envs keyed in CompilerOutputs by external (function/type template) ids, not their own id.
8. **Envs live in the scout arena.** Survive through instantiator.
9. **`DeferredActionT` variants never reference `CompilerOutputs`.** All captured context is owned or `Copy` (`&'s`/`&'t` refs, small values). Preserves the `pop_front → match → handler(&mut coutputs)` drain pattern without self-borrow hazards.
10. **Arena parameters use a short borrow lifetime.** `&ScoutArena<'s>` or `&'ctx ScoutArena<'s>`, never `&'s ScoutArena<'s>`. Matches existing-pass convention (§1.2).

---

## Part 12: Slab-Ordered Migration Plan

### 12.0 Ground Rule: Preserve The `// mig:` Audit Trail

The typing/ skeleton is already generated: every Scala definition in a `/* ... */` block has a `// mig: ...` marker and an empty stub (`pub enum KindT {}`, `panic!()` bodies, etc.) **directly above** its Scala counterpart. This structure is the migration audit trail and is non-negotiable.

**When filling definitions:**
- Replace the empty stub in-place with the real definition. Keep the `// mig: <name>` line. Keep the `/* ... scala ... */` block immediately below unchanged.
- Never move a Rust definition away from its Scala block.
- Never delete a `/* ... scala ... */` block during filling — it gets removed only at the slice-reconcile-delete phase, long after the definition is complete and proven.
- A Rust definition grown to need helper structs (e.g. an `INameValT` companion for an `INameT`) gets its companion block **adjacent to** the main definition, with its own `// mig:` marker if it corresponds to a Scala construct, or a `// (no scala counterpart)` note if it's Rust-only scaffolding (interning Val enums, `PtrKey` newtypes, builders).

### 12.1 Slabs

1. **Slab 0**: Arena substrate — `TypingInterner<'t>`, `PtrKey<'t, T>`, lifetime conventions docs. Non-migration Rust scaffolding; lives in files without `// mig:` markers.
2. **Slab 1**: Leaf types (`types/types.rs` primitives portion) — convert unit-struct stubs into real enums: `OwnershipT { Share, Own, Borrow, Weak }`, `MutabilityT { Mutable, Immutable }`, `VariabilityT { Final, Varying }`, `LocationT { Inline, Yonder }`, `RegionT`. Primitive `KindT` payloads (`NeverT`, `VoidT`, `IntT`, `BoolT`, `StrT`, `FloatT`) with fields per Scala. Leaf-value templatas (`MutabilityTemplataT`, `VariabilityTemplataT`, `OwnershipTemplataT`, `IntegerTemplataT`, `BooleanTemplataT`, `StringTemplataT`). All Copy where possible.
3. **Slab 2**: Name hierarchy (`names/names.rs`) — `IdT<'s, 't, T: Copy>` generic struct with `widen`/`try_narrow` conversion impls; all ~60 `INameT` concrete structs with `<'s, 't>`; ~14 sub-enums (`IFunctionNameT`, `IStructNameT`, `IInterfaceNameT`, `IFunctionTemplateNameT`, `IStructTemplateNameT`, `IInterfaceTemplateNameT`, `ITemplateNameT`, `IInstantiationNameT`, `ISubKindNameT`, `ISuperKindNameT`, `ICitizenNameT`, `ICitizenTemplateNameT`, `IVarNameT`, `IRegionNameT`) with **DAG variant duplication** per §6.2. `From`/`TryFrom` bridges. `INameValT<'s, 't>` + per-sub-enum `*ValT` companions for IDEPFL interning. Self-referential `ForwarderFunctionNameT.inner: &'t IFunctionNameT<'s, 't>` resolved by `&'t`.
4. **Slab 3**: Kind/Coord/Templata trio (`types/types.rs` remainder, `templata/templata.rs`) — non-primitive `KindT` variants (`StructTT`, `InterfaceTT`, `StaticSizedArrayTT`, `RuntimeSizedArrayTT`, `KindPlaceholderT`, `OverloadSetT`); `CoordT<'s, 't>` as inline Copy struct; `ITemplataT<'s, 't>` with all 19 variants including heavy (`FunctionTemplataT`, `StructDefinitionTemplataT`, `InterfaceDefinitionTemplataT`, `ImplDefinitionTemplataT`, `ExternFunctionTemplataT`) holding direct `&'s FunctionA`/`&'s StructA` refs; `PrototypeT` (generic in leaf-name `T`), `SignatureT`; `KindValT`, `ITemplataValT`, `PrototypeValT`, `SignatureValT` companions.
5. **Slab 4**: Environments (`env/*.rs`) — **convert the current `trait IEnvironmentT` stub into an enum** per §3.1. 9 variants, each as its own struct. `TemplatasStoreT<'s, 't>` with arena-slice pairs (no heap HashMap). Stack builder types (`NodeEnvironmentBuilder`, etc.) with `build_in(&ScoutArena<'s>) -> &'s NodeEnvironmentT<'s, 't>`.
6. **Slab 5**: Expression AST (`ast/expressions.rs`) — 3 enums (`ReferenceExpressionTE` ~38 variants, `AddressExpressionTE` ~6 variants, narrow-use `ExpressionTE` wrapper). Arena-allocated, not interned. Per-variant payload structs with `<'s, 't>`. `NodeRefT<'s, 't>` visitor enum + `visit_*` + `collect_*` macros.
7. **Slab 6**: `CompilerOutputs<'s, 't>` (`compiler_outputs.rs`) — all fields per §4.1, `PtrKey<'t, T>`-keyed HashMaps, `DeferredActionT<'s, 't>` enum (no `Box<dyn>`).
8. **Slab 7**: `HinputsT<'s, 't>` + Compiler god struct shell + top-level `run_typing_pass` entry point.
9. **Slab 8**: Function signatures across sub-compiler methods — file-by-file, each method's signature matches Scala's, body is `panic!("unimplemented: <id>")`. Goal: clean `cargo build --lib`.
10. **Slab 9+**: Method implementations, driven by failing tests.

**Per-slab completion criterion:** the files in-scope compile in isolation against the previously-completed slabs (using `panic!` bodies for anything downstream). The whole crate may still fail to build until Slab 8.

After Slab 8, build should be clean with `panic!()` bodies awaiting implementation. Subsequent slabs fill them.

### 12.2 Immediate Next Action

Slab 1 (`types/types.rs` primitives portion). Concrete work: replace each `pub struct ShareT;` / `pub enum OwnershipT {}` pair with a single `pub enum OwnershipT { Share, Own, Borrow, Weak }`, then mark the four individual unit-struct stubs as merged into the enum by leaving a `// mig: merged into OwnershipT above` line in place of their stub. Repeat for `MutabilityT`, `VariabilityT`, `LocationT`. Fill primitive `KindT` payload structs. Stops at `StructTT`/`InterfaceTT`/etc. which depend on `IdT` (Slab 2).

---

## Part 13: Open Questions / Future Work

- **Long-running processes (LSP):** scout arena retention through instantiation might be prohibitive for an always-on process. If this becomes the primary use case, migrate to a strict-containment design where `'s` dies at typing pass end.
- **Incremental compilation:** serializing `HinputsT` to disk requires a serialization boundary that breaks `'s` refs. Batch compilation only for now.
- **Parallelization:** single-threaded design (`!Sync` arenas, stack `CompilerOutputs`). Per-function parallelization is a later topic.
- **Arena-backed HashMap (`ArenaIndexMap`):** `CompilerOutputs`'s heap HashMaps could move to arena-backed maps at scale. See `docs/reasoning/arena-deterministic-maps.md`.
- **Variance of `IdT<'s, 't, T>`.** Scala's `+T` gave covariance for free. Rust auto-derives variance from field positions; since `T` appears only in `local_name: T`, `IdT` should be covariant in `T`, which is what the widening casts rely on. If a future addition places `T` in an invariant position (`fn(T) -> ()`, `Cell<T>`, `PhantomData<fn(T)>`), variance flips to invariant and the `From`/`Into`-based widening stops composing. Verify empirically in Slab 2 with a trait-object test; if it ever breaks, fall back to explicit per-target `fn upcast_to<U>() -> IdT<'s, 't, U>` methods.
- **Reverse-destructor arena.** Candidate crate: `bump-scope` (runs Drop via `BumpBox<T>`). Not required by the current design — it sticks to AASSNCMCX. Revisit if `Rc<IEnvironmentT>` comes back into fashion or if environment storage needs change.
