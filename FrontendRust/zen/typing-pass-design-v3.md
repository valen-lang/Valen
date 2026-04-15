# Typing Pass Migration Design (v3.1)

This document describes the architectural decisions for migrating `src/typing/` from Scala to Rust. It supersedes `typing-pass-design-v2.md`, which pursued strict arena containment (typing output pure `'t`). v3 chooses **pragmatic arena retention** — scout data lives past the typing pass, so typing output can reference it directly. Simpler, closer to Scala, faster to port.

v3.1 is a precision-polish pass over v3: explicit `'s: 't` outlives bounds, consistent arena-borrow convention, uniform `KindT` variant payloads, relaxed CompilerOutputs HashMap-value invariant, and minor clarifications. No architectural changes from v3.

---

## Part 0: What Changed From v2

| v2 (strict containment) | v3 (pragmatic retention) |
|---|---|
| `'s` dies at typing pass end | `'s` lives until instantiator completes (or later) |
| Output types pure `<'t>` | Output types `<'s, 't>` — can reference scout data |
| Heavy templatas hold `&'t IdT` + side tables | Heavy templatas hold `&'s FunctionA` / `&'s StructA` directly |
| `OverloadSetT<'t>` with `ctx_id` indirection | `OverloadSetT<'s, 't>` holds env directly |
| Re-intern boundary (`StrI<'t>`, `RangeT<'t>`, etc.) | No re-interning; `StrI<'s>`, `RangeS<'s>` used as-is |
| `'e` env arena separate from `'s` | Envs allocated into `'s` arena (no `'e`) |
| `FunctionEnvironmentT.templata_id: OnceCell<&'t IdT>` | `FunctionEnvironmentT.templata()` returns a fresh value, matching Scala |
| CompilerOutputs side tables for origin data | No origin side tables — heavy templatas carry refs directly |

### The Trade

- **Cost:** scout arena memory (FunctionA/StructA/etc. plus envs) retained through instantiation. Rough estimate: hundreds of MB to a few GB for large programs. Fine for batch compilation; reconsider for long-running LSP-style use.
- **Benefit:** ~half the arena/lifetime machinery gone. Port maps to Scala line-for-line in most places. No side-table plumbing. Faster to implement, easier to review for Scala parity.

---

## Part 1: Arena and Lifetime Model

### 1.1 Three Arenas

- **`'p` — Parser arena (`ParseArena<'p>`)**: parser AST, `StrI<'p>`, coords. Unchanged.
- **`'s` — Scout arena (`ScoutArena<'s>`)**: postparser + higher-typing output (`FunctionA<'s>`, `StructA<'s>`, etc.), interned postparser names (`INameS<'s>`, `IRuneS<'s>`, `IImpreciseNameS<'s>`), **and typing-pass environments**. Unchanged for postparser data; extended to hold envs.
- **`'t` — Typing arena (`TypingInterner<'t>`)**: interned typing-pass types (names, kinds, coords, templatas) and output AST (`FunctionDefinitionT`, expressions, `HinputsT`). Created before the typing pass; outlives the typing pass.

All three arenas use `bumpalo`. Arena-allocated structs follow AASSNCMCX: no `Vec`/`HashMap`/`String` inside arena types.

### 1.2 Lifetime Invariants

1. **`'s` outlives `'t`**. Expressed as `where 's: 't` on every output type that transitively holds `&'s` data. Rust does **not** enforce outlives via drop order — we must declare the bound. Representative structs that carry this bound include `HinputsT<'s, 't>`, `IEnvironmentT<'s, 't>`, `KindT<'s, 't>`, and every interned typing-pass type.
2. **`'t` outlives the typing pass.** Created by the caller before `run_typing_pass`; dropped by the caller after the instantiator is done.
3. **`'s` outlives the instantiator**, because `HinputsT<'s, 't>` contains `&'s` refs. Scout arena drops after the instantiator completes (or later).
4. **No new arenas introduced** beyond existing `'p` / `'s`. The v2 `'e` arena is eliminated; envs live in `'s`.
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

Everywhere that v2 needed three or four lifetimes, v3 uses two.

---

## Part 2: The God Struct

### 2.1 Architecture

All Scala sub-compilers collapse into a single `Compiler` struct:

```rust
pub struct Compiler<'s, 'ctx, 't> {
    pub typing_interner: &'ctx TypingInterner<'t>,
    pub scout_arena: &'ctx ScoutArena<'s>,
    pub keywords: &'ctx Keywords<'s>,  // scout-interned keywords are fine
    pub opts: &'ctx TypingPassOptions,
    pub global_env: &'s IEnvironmentT<'s, 't>,  // top-level env, arena-allocated in 's
    pub name_translator: NameTranslator,
}
```

Immutable configuration only. Mutable state threads through `&mut CompilerOutputs<'s, 't>` on every call.

### 2.2 `&self` + `&mut coutputs` Enables Re-entrancy

Deep mutual recursion works because `&self` allows shared borrow, and `&mut coutputs` is re-borrowed at each call level.

### 2.3 Macros

Macros (`AsSubtypeMacro`, `LockWeakMacro`, `StructDropMacro`, etc.) are stored in the global environment and receive the god struct at call time:

```rust
pub enum IFunctionGenerator<'s, 't> {
    AsSubtype(AsSubtypeMacro<'s, 't>),
    LockWeak(LockWeakMacro<'s, 't>),
    StructDrop(StructDropMacro<'s, 't>),
    // ... enum rather than trait-object hierarchy
}

impl<'s, 't> IFunctionGenerator<'s, 't> {
    pub fn generate(
        &self,
        compiler: &Compiler<'s, '_, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &'s IEnvironmentT<'s, 't>,
        // ...
    ) -> &'t FunctionHeaderT<'s, 't> { ... }
}
```

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

### 3.7 No `'e` Arena

Eliminated. Envs allocate via `scout_arena.alloc(...)`. Memory lives as long as `'s` (through instantiation). Some memory cost compared to dropping envs at pass end, but far simpler — no additional arena lifetime on every env type.

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

Notice: **no `origin_function_a`, `origin_struct_a`, `origin_env`, `overload_resolution_ctx`, or any other side tables for origin data.** Heavy templatas and OverloadSets hold refs directly.

### 4.2 `PtrKey<'t, T>` Newtype For HashMap Keys

Same as v2: interned `&'t T` refs hash by pointer identity, not structural equality. Newtype wrapper gives the custom `Hash`/`Eq`.

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

Per earlier investigations (Agents F, G):
- Overload resolution always completes during the typing pass; nothing unresolved reaches the instantiator.
- But an `OverloadSet`-kinded `ReinterpretTE` survives in output as an inert type tag (the instantiator elides it via `InstantiationBoundArgumentsT.runeToFunctionBoundArg`).

Since `'s` lives through instantiation, we can just hold the env directly — no `ctx_id` indirection needed:

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

No side-table insertion, no `ctx_id`, no `&mut coutputs` required for the construction itself. Simple.

### 5.3 Overload Resolution Uses The Env Directly

The resolver reads `overload_set.env` directly, walks its parent chain, looks up candidates, recurses into nested OverloadSets via their own `env` fields. Matches Scala's `OverloadResolver.scala:210` pattern verbatim. No signature changes to `getCandidateBannersInner` etc. — `&CompilerOutputs` is still threaded through the god-struct methods, but not specifically for env lookup.

### 5.4 Post-Pass

`OverloadSetT<'s, 't>` survives in `HinputsT<'s, 't>` inside inert `ReinterpretTE` args. Its `env` and `name` refs remain valid for as long as `'s` lives. The instantiator elides these on sight via `InstantiationBoundArgumentsT.runeToFunctionBoundArg` and never dereferences them — they're type-tag placeholders.

After the instantiator completes, the caller drops the typing interner (`'t` dies), then the scout arena (`'s` dies), in that order (§1.3). The OverloadSet's refs are never dangling during any live use; after both arenas drop, the entire `HinputsT` is unreachable.

---

## Part 6: Type System Types

Every lifetime-parameterized struct in this section has an implicit `where 's: 't` bound (§1.2). Representative examples are spelled out on `IdT`, `CoordT`, `KindT`, and `ITemplataT` below; others follow the same convention.

### 6.1 IDEPFL Dual-Enum Pattern

All interned typing types use the dual-enum pattern: a **reference enum** (canonical, `&'t` refs) and a **value enum** (transient, HashMap lookup). Unlike v2, there's no re-interning boundary — scout-lifetimed values (`StrI<'s>`, `IImpreciseNameS<'s>`, `RangeS<'s>`, etc.) are used directly wherever they appear.

Interned type families:
- `INameT<'s, 't>` / `INameValT<'s, 't>` (~60 variants)
- `IdT<'s, 't, T>` / `IdValT<'s, 't, T>`
- `KindT<'s, 't>` / `KindValT<'s, 't>`
- `ITemplataT<'s, 't>` / `ITemplataValT<'s, 't>`
- `PrototypeT<'s, 't>` / `PrototypeValT<'s, 't>`
- `SignatureT<'s, 't>` / `SignatureValT<'s, 't>`

- **No `'t`-lifetime re-interned versions of `StrI`, `IImpreciseNameS`, `RangeS`, `CodeLocationS`, `PackageCoordinate`, `FileCoordinate`.** Use the scout-arena versions directly. This is the central v3 simplification vs. v2 and is worth stating explicitly: anywhere you'd be tempted to create a `StrI<'t>` or `RangeT<'t>`, use the existing `StrI<'s>` or `RangeS<'s>` instead.

### 6.2 INameT Hierarchy

~60 concrete name types, ~14 sub-trait enums. Shared DAG variants get variants in each relevant sub-enum. `From`/`TryFrom` for narrow↔wide conversion. Matches v2's structure but every name type carries `<'s, 't>` instead of `<'t>`.

No new name variants needed for env handling (env-id uniqueness isn't required).

### 6.3 `IdT<'s, 't, T>` — Generic, Interned

```rust
pub struct IdT<'s, 't, T: Copy>
where 's: 't,
{
    pub package_coord: &'s PackageCoordinate<'s>,  // scout-lifetimed
    pub init_steps: &'t [&'t INameT<'s, 't>],
    pub local_name: T,
}
```

Only `T: Copy` on the struct itself — keep bounds minimal so `IdT` is cheap to mention in signatures elsewhere. Conversion bounds (`Into<&'t INameT>`, `TryInto<...>`) live on the impl blocks for the conversion methods, not on the struct definition:

```rust
impl<'s, 't, T: Copy + Into<&'t INameT<'s, 't>>> IdT<'s, 't, T>
where 's: 't,
{
    pub fn widen(self) -> IdT<'s, 't, &'t INameT<'s, 't>> { ... }
}

impl<'s, 't, T: Copy> IdT<'s, 't, T>
where 's: 't,
{
    pub fn widen_to<U: Copy>(self) -> IdT<'s, 't, U>
    where T: Into<U> { ... }
}

impl<'s, 't> IdT<'s, 't, &'t INameT<'s, 't>>
where 's: 't,
{
    pub fn try_narrow<U: Copy>(self) -> Option<IdT<'s, 't, U>>
    where &'t INameT<'s, 't>: TryInto<U> { ... }
}
```

Generic parameter preserves leaf-kind info at the type level (e.g. `IdT<'s, 't, &'t IFunctionNameT<'s, 't>>` vs `IdT<'s, 't, &'t IStructTemplateNameT<'s, 't>>`).

Interning uses one HashMap keyed by the widest form (`IdValT<'s, 't, &'t INameT>`).

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

Unlike v2, there's no leaf/heavy distinction. All variants are equally free to hold `&'s` refs:

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

No ID indirection, no side tables, no slot-constraint enforcement. `FunctionHeaderT.maybe_origin_function_templata` and `ImplT.templata` and `FunctionBannerT.origin_function_templata` all just hold the heavy templata directly (v2 would have replaced them with IDs).

**Mixed-mode equality.** `ITemplataT` deliberately mixes Copy-value variants (`Mutability`, `Variability`, `Ownership`, `Integer`, `Boolean`) with `&'t`-ref variants (`Coord`, `Kind`, `Prototype`, heavy templatas, …). Value variants compare by value; ref variants compare by pointer identity (the typing interner guarantees uniqueness). `ITemplataValT` mirrors this split for HashMap lookup. When an `ITemplataT` itself lives behind `&'t ITemplataT` and is used as a key, wrap in `PtrKey<'t, ITemplataT>` (pointer-eq on the whole value); when it's used by value, the derived `PartialEq`/`Hash` does the right thing per-variant.

### 6.7 Mutual Recursion

Types form a mutually recursive graph (`CoordT → KindT → StructTT → IdT → INameT → ITemplataT → CoordT`). Every link is an `&'s` or `&'t` ref — pointer-sized, finite. No `Box`/`Vec` needed; arena references provide the indirection.

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

### 7.2 Arena-Allocated, Not Interned

Expression nodes are allocated into `'t` but not deduped. Sub-expressions are `&'t` refs; collections are arena slices.

### 7.3 Can Reference Scout Data

Unlike v2, TE nodes can freely hold `&'s` refs — e.g., if an expression's type metadata includes a `RangeS<'s>` source location, that's fine.

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
    opts: &'ctx TypingPassOptions,
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
8. **Envs live in the scout arena**, not a separate arena. Survive through instantiator.
9. **`DeferredActionT` variants never reference `CompilerOutputs`.** All captured context is owned or `Copy` (`&'s`/`&'t` refs, small values). Preserves the `pop_front → match → handler(&mut coutputs)` drain pattern without self-borrow hazards.
10. **Arena parameters use a short borrow lifetime.** `&ScoutArena<'s>` or `&'ctx ScoutArena<'s>`, never `&'s ScoutArena<'s>`. Matches existing-pass convention (§1.2).

---

## Part 12: Slab-Ordered Migration Plan

1. **Slab 0**: Arena substrate — `TypingInterner<'t>`, lifetime conventions docs.
2. **Slab 1**: Leaf types — `OwnershipT`, `MutabilityT`, `VariabilityT`, `LocationT`, `RegionT`, primitive `KindT` payloads, leaf-value templatas.
3. **Slab 2**: Name hierarchy — all ~60 `INameT` variants, `IdT<'s, 't, T>` with casts, sub-enums, `INameValT` companions. All `<'s, 't>`.
4. **Slab 3**: Kind/Coord/Templata trio — non-primitive `KindT` variants, `CoordT<'s, 't>` inline, `ITemplataT<'s, 't>` (all variants including heavy), `PrototypeT`, `SignatureT`, `OverloadSetT`.
5. **Slab 4**: Environments — `IEnvironmentT<'s, 't>` with 9 variants allocated into scout arena, `TemplatasStoreT<'s, 't>`, builder types.
6. **Slab 5**: Expression AST — 3 enums, arena-allocated, `NodeRefT` visitor.
7. **Slab 6**: `CompilerOutputs<'s, 't>` — all fields, deferred-action enum. Fewer fields than v2 (no origin side tables).
8. **Slab 7**: Compiler god struct shell — fields, constructor, top-level `run_typing_pass`.
9. **Slab 8**: Function signatures across sub-compilers — file-by-file stubs matching Scala.
10. **Slab 9+**: Method implementations.

After Slab 8, build should be clean with `panic!()` bodies awaiting implementation. Subsequent slabs fill them.

**v3 has 9 slabs vs v2's 10** (no re-intern boundary slab).

---

## Part 13: Open Questions / Future Work

- **Long-running processes (LSP):** scout arena retention through instantiation might be prohibitive for an always-on process. If this becomes the primary use case, migrate to v2's strict-containment design. v2 stays on file as that target.
- **Incremental compilation:** serializing `HinputsT` to disk requires a serialization boundary that breaks `'s` refs. v2 design would be required. For now, batch compilation only.
- **Parallelization:** single-threaded design (`!Sync` arenas, stack `CompilerOutputs`). Per-function parallelization is a later topic.
- **Arena-backed HashMap (`ArenaIndexMap`):** `CompilerOutputs`'s heap HashMaps could move to arena-backed maps at scale. See `docs/reasoning/arena-deterministic-maps.md`.
- **Variance of `IdT<'s, 't, T>`.** Scala's `+T` gave covariance for free. Rust auto-derives variance from field positions; since `T` appears only in `local_name: T`, `IdT` should be covariant in `T`, which is what the widening casts rely on. If a future addition places `T` in an invariant position (`fn(T) -> ()`, `Cell<T>`, `PhantomData<fn(T)>`), variance flips to invariant and the `From`/`Into`-based widening stops composing. Verify empirically in Slab 2 with a trait-object test; if it ever breaks, fall back to explicit per-target `fn upcast_to<U>() -> IdT<'s, 't, U>` methods.
- **Reverse-destructor arena.** At one point there was a plan for a `'t` arena that runs `Drop` in reverse-allocation order, enabling `Rc`/`Vec` inside typing types. No doc found during v3 review. Candidate crate: `bump-scope` (runs Drop via `BumpBox<T>`). Not required by v3 as written — current design sticks to AASSNCMCX. Revisit if `Rc<IEnvironmentT>` comes back into fashion or if environment storage needs change.

---

## Part 14: Why Not V2?

v3 was chosen over v2 for migration velocity. Key reasons:

1. **Closer to Scala** — heavy templatas hold refs (matches Scala); no ID-indirection layer to port through.
2. **Fewer moving parts** — no side tables for origin data, no `ctx_id` for OverloadSet, no re-intern boundary.
3. **Simpler lifetimes** — 2 lifetimes (`'s, 't`) across typing types vs v2's 3 (`'t` only for output, but envs had `'s, 'e, 't`).
4. **Incremental path** — if we ever need v2's strict containment, the work is mostly mechanical: replace direct refs with IDs, add side tables, re-intern at boundaries. Starting from v3 doesn't preclude v2.

v2 is superior for: LSP scenarios, serialized incremental builds, and situations where scout memory retention is unacceptable. For the typical "one-shot batch compilation" use case that's the current target, v3 is strictly simpler and faster to build.
