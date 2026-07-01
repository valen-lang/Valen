# Instantiating Pass Design

Architecture and design decisions for the instantiating pass. Companion to `docs/architecture/typing-pass-design-v3.md` and `docs/architecture/simplifier-design.md`.

This pass takes typing-pass output (`HinputsT<'s, 't>`) and produces monomorphized output (`HinputsI<'s, 'i>`) for the simplifying pass. It is the pass that resolves generics into concrete instantiations: every templated function/struct/interface gets one concrete instance per reachable use-site, generic placeholders become concrete types, and bound arguments get threaded into v-tables. Tree-shaking falls out of reachability — uncalled overloads never appear in `HinputsI`.

The Scala source of truth is `Frontend/InstantiatingPass/src/dev/vale/instantiating/`. The Rust implementation lives in `FrontendRust/src/instantiating/`.

> **Historical note.** An earlier version of this pass carried an `R: IRegionsModeIT` type-parameter on every I-side type, plus a pair of `region_collapser_*.rs` modules and a `region_counter.rs`. All of that was removed. The three "region modes" (`sI`/`nI`/`cI`) do not appear as type generics anymore. The **interner still exposes three suffixed intern families** (`_si`/`_ni`/`_ci`) — see §4 — because the three-way canonical-identity separation is still load-bearing at runtime even without the types to enforce mode-crossings. If you're reading this doc against a codebase that has an `R` generic anywhere, you're on an older branch.

---

## Part 1: Arena and Lifetime Model

The instantiator runs at the boundary of two arenas — it reads from `'t` (typing) and writes into `'i` (instantiating). Both arenas survive past the pass; instantiator output is consumed by the simplifying pass without any re-interning at the I→H boundary.

### 1.1 Four Arenas

By the time the instantiator runs, all four input arenas are live:

- **`'p` — Parser arena** (`ParseArena<'p>`). Read-only.
- **`'s` — Scout arena** (`ScoutArena<'s>`). Read-only; the instantiator only ever reads scout-interned names, source strings, and code locations.
- **`'t` — Typing arena** (`TypingInterner<'s, 't>`). Read-only input: `HinputsT<'s, 't>` and everything it transitively reaches (`IdT`, `KindT`, `ITemplataT`, `PrototypeT`, env structs, expression AST).
- **`'i` — Instantiating arena** (`InstantiatingInterner<'s, 'i>`). The instantiator's allocator. Holds interned monomorphized types (kind payloads, prototypes, signatures, names), the monomorphized definitions (`StructDefinitionI`, `InterfaceDefinitionI`, `FunctionDefinitionI`), and the instantiated expression AST.

`InstantiatingArena<'i>` (`instantiating_arena.rs`) is a thin newtype around `&'i Bump`. `InstantiatingInterner<'s, 'i>` (`instantiating_interner.rs:33-38`) wraps the bump plus a `RefCell<Inner>` holding 12 HashMap families (see §4).

### 1.2 Lifetime Invariants

1. **`'s: 'i`** — interner-level invariant, carried by every output type that holds `&'s` refs into scout-arena data.
2. **`'s: 't`** — inherited from the typing arena. The combined where-clause on `InstantiatorI` is `where 's: 't, 's: 'i`.
3. **`'t` and `'i` are siblings** — neither outlives the other; the design carries no `'t: 'i` or `'i: 't` bound. `InstantiatingInterner<'s, 'i>` deliberately does NOT carry `'t` — this means a future variant of the pipeline *could* drop the typing arena before consuming `HinputsI`, even though the current canonical driver doesn't take advantage of that.
4. **Never `'static`.** All data lives in `'p`/`'s`/`'t`/`'i` or on the stack. `&'static T` would break pointer-equality for interned types (NUSLX).
5. **Arena borrow convention.** Arena parameters take a short borrow lifetime — `&InstantiatingInterner<'s, 'i>` or `&'ctx InstantiatingInterner<'s, 'i>`, never `&'i InstantiatingInterner<'s, 'i>`. `interner.intern_*` methods return `&'i T` from a `&self` borrow.
6. **AASSNCMCX.** No `Vec`/`HashMap`/`String` inside arena-allocated types. Use `&'i [T]` slices and `ArenaIndexMap<'i, K, V>`. Driver structs (`InstantiatedOutputsI`, `InstantiatorI`) live on the stack and may use plain `IndexMap`/`Vec` — same exception as typing's `CompilerOutputs`.

### 1.3 Arena Construction & Drop Order

The canonical driver in `FrontendRust/src/pass_manager/pass_manager.rs` constructs bumps in this order:

```rust
let scout_bump         = bumpalo::Bump::new();
let typing_bump        = bumpalo::Bump::new();
let hammer_bump        = bumpalo::Bump::new();
let instantiating_bump = bumpalo::Bump::new();
```

Rust drops locals in reverse declaration order, so at end-of-scope: `'i` dies first, then `'h`, then `'t`, then `'s`. Hammer and instantiating are siblings; both drop before typing, which drops before scout. There are no explicit `drop()` calls — the order is purely lexical.

### 1.4 Where Each Type Lives

| Type | Lifetimes | Arena |
|---|---|---|
| `HinputsI` | `<'s, 'i>` | `'i` for the struct itself; holds `&'i` slices + `ArenaIndexMap` |
| `InstantiationBoundArgumentsI` | `<'s, 'i>` | `'i`, value-type holding `ArenaIndexMap`s |
| `FunctionDefinitionI` | `<'s, 'i>` | `'i`, allocated |
| `StructDefinitionI`, `InterfaceDefinitionI` | `<'s, 'i>` | `'i`, allocated |
| `IdI`, `INameI`, `ITemplataI` | `<'s, 'i>` | `'i`; names interned, IdI/templata not yet sealed |
| `KindIT` | `<'s, 'i>` | inline Copy Polyvalue wrapper; compound payloads `&'i`-interned |
| `CoordI` | `<'s, 'i>` | inline Copy struct (ownership + kind) |
| `PrototypeI`, `SignatureI` | `<'s, 'i>` | `'i`, interned, `MustIntern` seal |
| `StructIT`, `InterfaceIT`, `StaticSizedArrayIT`, `RuntimeSizedArrayIT` | `<'s, 'i>` | `'i`, interned, `MustIntern` seal |
| Expression AST (`ExpressionIE`/`ReferenceExpressionIE`/`AddressExpressionIE`) | `<'s, 'i>` | `'i`, allocated, not interned |
| `EdgeI`, `InterfaceEdgeBlueprintI` | `<'s, 'i>` | `'i`, allocated |
| `KindExportI`, `FunctionExportI`, `KindExternI`, `FunctionExternI` | `<'s, 'i>` | `'i`, allocated |
| `DenizenBoundToDenizenCallerBoundArgI` | `<'s, 't, 'i>` | Stack-owned; uses heap `IndexMap` |
| `InstantiatedOutputsI` | `<'s, 't, 'i>` | Stack-owned accumulator (mutable; heap `IndexMap`) |
| `InstantiatorI` (god struct) | `<'s, 'ctx, 't, 'i>` | Stack |
| `InstantiatedCompilation` (driver) | `<'s, 'ctx, 't, 'i, 'p>` | Stack |

### 1.5 Mutual-Recursion Shape

The I-type graph is mutually recursive but every edge is `&'s`, `&'i`, or a `<'s, 'i>`-parameterized inline value: `CoordI → KindIT → StructIT → IdI → INameI → ITemplataI → CoordI`. Same-type self-recursion (e.g. `ForwarderFunctionNameI.inner: IFunctionNameI`) resolves via the inline 16-byte sub-enum. No `Box`/`Vec` needed in arena types.

### 1.6 Why a Separate `InstantiatingInterner`

Scala used one `Interner` for the entire pipeline. The Rust port splits into per-pass interners (`ScoutArena`, `TypingInterner`, `InstantiatingInterner`, `HammerInterner`) so each arena drops independently. The instantiator holds **both** the typing-pass interner (for reading T-side data via helpers like `Compiler::get_super_template`) **and** the instantiating-pass interner (for emitting I-side data). The dual-interner pattern is a Rust-specific adaptation, not a design choice.

---

## Part 2: The God Struct (`InstantiatorI`)

### 2.1 Architecture

Scala's `Instantiator` class (already a single big class in Scala — no sub-instantiators to collapse) ports to `InstantiatorI<'s, 'ctx, 't, 'i>` at `instantiator.rs:277-283`:

```rust
pub struct InstantiatorI<'s, 'ctx, 't, 'i>
where 's: 't, 's: 'i,
{
    pub opts: &'ctx GlobalOptions,
    pub interner: &'ctx InstantiatingInterner<'s, 'i>,
    pub typing_interner: &'ctx TypingInterner<'s, 't>,
    pub keywords: &'ctx Keywords<'s>,
    pub hinputs: &'ctx HinputsT<'s, 't>,
}
```

Immutable-only configuration plus the input `HinputsT` (read-only). Mutable state threads through `&mut InstantiatedOutputsI<'s, 't, 'i>` on every translation method call.

`hinputs` lives on the god struct (not threaded through every call) because every `translate_*` method needs T-side lookups (e.g. `self.hinputs.functions.iter().find(...)` inside `find_function`/`find_struct`/`find_impl`); lifting it to the struct removes it from ~50 method signatures.

### 2.2 `&self` + `&mut monouts` Enables Re-entrancy

Deep mutual recursion (e.g. `translate_ref_expr → translate_function_call → translate_prototype → translate_function → translate_ref_expr`) works because `&self` is shared-borrowed and `&mut monouts` is re-borrowed at each call level. Same idiom as the typing pass.

### 2.3 The 57 `translate_*` Methods

`InstantiatorI` exposes 57 `pub fn translate_*` methods (raw count on the current file) on its `impl` block spanning `instantiator.rs:287-2717`. This matches the Scala side's `translate*` methods 1:1, with a small number of Scala helpers not yet ported.

Major clusters (`file:line` starting positions):

- **Entry** — `translate_method(&self, monouts: &mut InstantiatedOutputsI) -> HinputsI` (`:288`).
- **ID / name** — `translate_id` (`:543`), `translate_export_name` (`:557`), `translate_export_template_name` (`:566`), `translate_name` (`:573`), `translate_function_id` (`:2056`), `translate_struct_id` (`:2069`), `translate_interface_id` (`:2084`), `translate_impl_id` (`:2102`), `translate_citizen_name` (`:2121`), `translate_id_from_substitutions` (`:2130`), `translate_citizen_id` (`:2138`).
- **Callsite dispatch** (new instantiation) — `translate_interface_callsite` (`:579`), `translate_struct_callsite` (`:639`), `translate_override` (`:683`), `translate_impl_callsite` (`:780`), `translate_function_callsite` (`:792`), `translate_abstract_func` (`:820`).
- **Definitions** — `translate_struct_member` (`:897`), `translate_prototype` (`:941`), `translate_struct_definition` (`:1106`), `translate_interface_definition` (`:1138`), `translate_function_header` (`:1169`), `translate_function_attribute` (`:1189`), `translate_citizen_attribute` (`:1202`), `translate_function_definition` (`:1210`), `translate_impl_definition` (`:2677`).
- **Locals** — `translate_local_variable` (`:1251`), `translate_reference_local_variable` (`:1267`), `translate_addressible_local_variable` (`:1281`).
- **Expressions** — `translate_addr_expr` (`:1293`), `translate_expr` (`:1363`), `translate_ref_expr` (`:1377`).
- **Types** — `translate_ownership` (`:1996`), `translate_kind` (`:2331`), `translate_coord` (`:2148`), `translate_citizen` (`:2224`), `translate_struct` (`:2238`), `translate_interface` (`:2246`).
- **Bound-arg / templata** — `translate_bound_args_for_callee` (`:1028`).

The **module-level free function** `translate` at `:268-273` is the top-level entry point (see §10).

### 2.4 Almost Everything Is a Method

Everything except the top-level `translate` free function lives as a method on `impl InstantiatorI` and takes `&self`. In particular, `translate_id`, `translate_name`, `translate_export_name`, `translate_export_template_name`, `translate_id_from_substitutions` are all **methods**, not free functions — they all need the interner via `self.interner.intern_*`.

There are no free-function "helper" modules parallel to `instantiator.rs` — the `reintern.rs` and `region_*.rs` modules of the old architecture are gone. Small helpers like `assemble_placeholder_map` (`:849`), `assemble_placeholder_map_inner` (`:871`), `assemble_callee_denizen_function_bounds` (`:623`), and `assemble_callee_denizen_impl_bounds` (`:631`) all live as methods on `impl InstantiatorI`.

### 2.5 `DenizenBoundToDenizenCallerBoundArgI`

A companion struct (`instantiator.rs:191-205`) threaded as `&` arg through ~50 method signatures:

```rust
pub struct DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>
where 's: 't, 's: 'i
{
    pub func_id_to_bound_arg_prototype:
        IndexMap<IdT<'s, 't>, &'i PrototypeI<'s, 'i>>,
    pub bound_param_impl_id_to_bound_arg_impl_id:
        IndexMap<IdT<'s, 't>, IdI<'s, 'i>>,
}
```

Two `IndexMap`s keyed by typing-pass id (`IdT`, the bound-parameter side) and valued by instantiator id/prototype (the resolved caller-side). The `.plus()` method unions two of these via `union_maps_expect_no_conflict`. Carries `<'s, 't, 'i>` — three lifetimes — because keys are `'t`-side and values are `'i`-side.

### 2.6 No `IEnvironmentI`

The instantiating pass has **no environment type**, deliberately. After monomorphization, scope resolution is already settled — every templata reference was pinned to a concrete instantiated value during typing. The instantiator's analogue is the **substitution map** `IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>` mapping typing-pass placeholders to concrete instantiated templatas, threaded explicitly through every method that needs it alongside `DenizenBoundToDenizenCallerBoundArgI`.

Don't introduce an `IEnvironmentI` even when it would clean up signatures — Scala parity wins.

---

## Part 3: `InstantiatedOutputsI` (the Monouts Accumulator)

### 3.1 Structure

Stack-owned mutable accumulator threaded through every method. Carries `<'s, 't, 'i>`. From `instantiator.rs:209-229`, 20 fields:

```rust
pub struct InstantiatedOutputsI<'s, 't, 'i> where 's: 't, 's: 'i {
    // Definition outputs
    pub functions: IndexMap<IdI<'s, 'i>, &'i FunctionDefinitionI<'s, 'i>>,
    pub structs: IndexMap<IdI<'s, 'i>, &'i StructDefinitionI<'s, 'i>>,
    pub interfaces_without_methods: IndexMap<IdI<'s, 'i>, &'i InterfaceDefinitionI<'s, 'i>>,

    // Sharedness metadata (SharednessI is the Single/Shared enum — replaces old MutabilityI)
    pub struct_to_sharedness: IndexMap<IdI<'s, 'i>, SharednessI>,
    pub interface_to_sharedness: IndexMap<IdI<'s, 'i>, SharednessI>,
    pub impl_to_sharedness: IndexMap<IdI<'s, 'i>, SharednessI>,

    // Bound storage
    pub struct_to_bounds: IndexMap<IdI<'s, 'i>, DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>,
    pub interface_to_bounds: IndexMap<IdI<'s, 'i>, DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>,
    pub impl_to_bounds: IndexMap<IdI<'s, 'i>, DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>,
    pub abstract_func_to_bounds:
        IndexMap<IdI<'s, 'i>, (DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>,
                                &'i InstantiationBoundArgumentsI<'s, 'i>)>,

    // Edge/dispatch tables
    pub interface_to_impls: IndexMap<IdI<'s, 'i>, Vec<(IdT<'s, 't>, IdI<'s, 'i>)>>,
    pub interface_to_abstract_func_to_virtual_index:
        IndexMap<IdI<'s, 'i>, IndexMap<PrototypeI<'s, 'i>, usize>>,
    pub impls: IndexMap<IdI<'s, 'i>,
                        (ICitizenIT<'s, 'i>,
                         IdI<'s, 'i>,
                         DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>,
                         InstantiationBoundArgumentsI<'s, 'i>)>,
    pub interface_to_impl_to_abstract_prototype_to_override:
        IndexMap<IdI<'s, 'i>,
                 IndexMap<IdI<'s, 'i>,
                          IndexMap<PrototypeI<'s, 'i>, PrototypeI<'s, 'i>>>>,

    // Worklists (drained FIFO via Vec::remove(0))
    pub new_impls: Vec<(IdT<'s, 't>, IdI<'s, 'i>, InstantiationBoundArgumentsI<'s, 'i>)>,
    pub new_abstract_funcs: Vec<(PrototypeT<'s, 't>, PrototypeI<'s, 'i>, usize, IdI<'s, 'i>,
                                  InstantiationBoundArgumentsI<'s, 'i>)>,
    pub new_functions: Vec<(PrototypeT<'s, 't>, PrototypeI<'s, 'i>, InstantiationBoundArgumentsI<'s, 'i>,
                             Option<DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>)>,

    // Externs (final outputs)
    pub kind_externs: Vec<KindExternI<'s, 'i>>,
    pub function_externs: Vec<FunctionExternI<'s, 'i>>,
}
```

### 3.2 Worklist-Driven Discovery Loop

`translate_method` runs a single while-loop drainage after the initial export/extern seeding:

```rust
loop {
    if let Some(item) = monouts.new_functions.first() { ... self.translate_function(...); }
    else if let Some(item) = monouts.new_impls.first() { ... self.translate_impl(...); }
    else if let Some(item) = monouts.new_abstract_funcs.first() { ... self.translate_abstract_func(...); }
    else { break; }
}
```

Each translate may push more work onto the three queues. Worklist completion = fixpoint = monomorphization complete.

The queues use `Vec<(...)>` not `VecDeque`, drained via `Vec::remove(0)` (FIFO). Scala uses `mutable.Queue` with `dequeue()`; the Rust port chose `Vec`+`remove(0)` for simplicity, accepting O(n) drain on small queues. Semantic FIFO behavior is preserved.

Scala had additional `newStructs`/`newInterfaces` queues; these are preserved as commented-out Scala in the Rust audit trail but not used — the Rust port translates structs and interfaces eagerly as they're encountered.

### 3.3 Why `IndexMap`, Not `HashMap`, Not `ArenaIndexMap`

- `IndexMap` (not `HashMap`): insertion-ordered iteration is **load-bearing** for output determinism. `HinputsI.functions` etc. must come out in a stable order so wire-format goldens stay byte-stable.
- Heap `IndexMap` (not `ArenaIndexMap`): `InstantiatedOutputsI` is mutated by the pass — values are inserted, queues are drained, v-tables are populated. Arena-backed maps are append-only once frozen. The accumulator is stack-owned so heap allocation is fine (AASSNCMCX exception, same as `CompilerOutputs`).

The per-definition payloads stored *inside* the IndexMaps are arena-allocated (`&'i FunctionDefinitionI`, etc.) and survive past the accumulator's drop.

### 3.4 Keys Are `IdI` By Value, Not `PtrKey`

Every key in `InstantiatedOutputsI` is an `IdI` or `PrototypeI` by value, not wrapped in `PtrKey<'i, T>`. I-side interned types have derived `PartialEq`/`Eq`/`Hash` that delegate to interned `&'i` ref fields (slice and inner-enum compare → effectively ptr-eq on canonical payloads) and `&'s` ref fields (ptr-eq), so canonical-identity hashing falls out without a `PtrKey` wrapper.

This differs from typing-pass `CompilerOutputs`, which does use `PtrKey<'t, T>` in several places.

### 3.5 Sharedness, Not Mutability

The three `*_to_sharedness` fields hold values of `SharednessI` (`ast/types.rs:36-40`), a two-variant `Single`/`Shared` enum. Every `StructDefinitionI` and `InterfaceDefinitionI` also carries an inline `sharedness: SharednessI` field. This replaces the older `MutabilityI` model that used `Mutable`/`Immutable`. The rename tracks the sharedness arc that landed at the experimental-2 squash — a struct's "shared" status is now about whether its instances are aliased across owners, not about interior mutation. `MutabilityI` no longer exists in the I-side AST.

At the translation site: `translate_mutability(_m: &SharednessT) -> SharednessI` (`instantiator.rs:933`) is the T→I bridge — the Scala method name is preserved but the value it returns is a sharedness now.

### 3.6 Speculative Writes Don't Apply

Unlike typing-pass `CompilerOutputs` (which speculates during overload resolution), the instantiator never speculates. Every write is the result of a confirmed reach from an exported entry point. No rollback machinery.

### 3.7 The One Named Mutation API

`add_method_to_v_table(impl_id, super_interface_id, abstract_prototype, override_prototype)` at `instantiator.rs:258` is the one named mutation method. All other mutations are direct field manipulation (`monouts.functions.insert(...)`, `monouts.new_functions.push(...)`).

---

## Part 4: Interning Surface — `InstantiatingInterner`

### 4.1 The 12-HashMap Structure

`instantiating_interner.rs:40-58`. **12 HashMaps = 4 logical types × 3 intern namespaces (`_si` / `_ni` / `_ci`)**:

| | `_si` | `_ni` | `_ci` |
|---|---|---|---|
| Kind payloads (Struct / Interface / SSA / RSA) | `kind_payload_val_to_ref_si` | `..._ni` | `..._ci` |
| Prototype | `prototype_val_to_ref_si` | `..._ni` | `..._ci` |
| Signature | `signature_val_to_ref_si` | `..._ni` | `..._ci` |
| Name (72 variants) | `name_val_to_ref_si` | `..._ni` | `..._ci` |

**The three suffixed namespaces exist despite the `R` type-generic being gone.** After `R: IRegionsModeIT` was stripped, the values stored in `_si` / `_ni` / `_ci` are identically typed (all `&'i PrototypeI<'s, 'i>` etc. — no phantom `R`). What differs is which hashmap they live in, so canonical-pointer identity is scoped to a namespace. Callers must still pick the right suffix at the callsite: `_si` at the T→I boundary, `_ni` for worklist-facing queue payloads, `_ci` for final-emit output.

This is transitional. Once the pipeline collapses to a single stage of interning (the R generic being gone means there's no static distinction to enforce), the three families should merge to one — see §17.

### 4.2 Family Methods (24) + Per-Concrete Wrappers (~216)

Each (kind × namespace) pair has one family-level dispatch method — 4 kind payload families × 3 namespaces = 12 kind wrappers, plus `intern_prototype_{si,ni,ci}`, `intern_signature_{si,ni,ci}`, `intern_name_{si,ni,ci}` = 24 family entrypoints total (`instantiating_interner.rs:331-676`).

On top of those, each concrete variant gets typed convenience wrappers — 4 kind payloads × 3 modes = 12 kind wrappers; 72 concrete `*NameI` types × 3 modes = 216 name wrappers.

Hand-writing 216 wrappers would be miserable; they're generated by four macros (`instantiating_interner.rs:65-196`):

- `impl_intern_name_wrappers!` — standard variant.
- `impl_intern_name_wrappers_with_s!` — variants with an extra `'s` lifetime in their payload.
- `impl_intern_name_wrappers_with_i!` — variants with an extra `'i`.
- `impl_intern_name_wrappers_r_only!` — variants whose payload is nothing but a `RegionT`-flavored field (preserved for source parity with Scala even though `R` is gone).

All four use **`paste!`** (`::paste::paste!`) to construct the per-namespace method names (`[<$method _si>]`, `[<$method _ni>]`, `[<$method _ci>]`). The `paste` crate is a real build dependency (`FrontendRust/Cargo.toml: paste = "1"`).

### 4.3 Val/Ref Pair Pattern

Same as typing and simplifying. Each interned type `XI<'s,'i>` has a sibling `XValI<'s,'i>` (Value-type, derives Hash/Eq) used as the HashMap query key. The canonical `&'i XI` has `_must_intern: MustIntern(())` seal field. Construction of `MustIntern` is private to the interner module, so only `intern_*` methods can produce sealed canonicals (per @SICZ).

### 4.4 Arena Utility API

`instantiating_interner.rs:287-325` exposes raw arena access:

- `new(bump: &'i Bump) -> Self` (`:287`)
- `bump(&self) -> &'i Bump` (`:308`)
- `alloc<T>(T) -> &'i mut T` (`:309`)
- `alloc_slice_copy<T: Copy>(&[T]) -> &'i [T]` (`:310`)
- `alloc_slice_from_vec<T>(Vec<T>) -> &'i [T]` (`:313`)
- `alloc_index_map<K, V>() -> ArenaIndexMap<'i, K, V>` (`:317`)
- `alloc_index_map_from_iter<K, V, I>(I) -> ArenaIndexMap<'i, K, V>` (`:321`)

### 4.5 Deferred Intern Families

Two intern families remain unwired:

- **`intern_id_*`** — `IdI` is allocated but not sealed (no `_must_intern` field, no `IdIValI` mirror, no `intern_id_*` methods). The contrast with the sibling types is sharp: `IdT` (typing-side) is sealed, as are all `*IT`/`PrototypeI`/`SignatureI`/`*NameI` types on the I-side. Until `IdI` is sealed, `IdI`-keyed maps fall back to structural comparison — slightly slower but correct because the slice and inner-name fields propagate ptr-eq through the derived impl.
- **`intern_templata_*`** — intentionally not planned. `ITemplataI` is Value-type per Scala parity (`ITemplataT` likewise — typing pass agrees).

---

## Part 5: I-Suffix AST Type Families

Every lifetime-parameterized struct has an implicit `where 's: 'i` bound (§1.2). The `R: IRegionsModeIT` generic parameter no longer exists on any I-side type.

### 5.1 T-Erasure Principle Inherited From Typing

Same as typing-pass §6.0: Scala's `+T` covariant type parameters are erased to monomorphic widest-form in Rust:

- `IdI[+T <: INameI]` → `IdI<'s, 'i>` with `local_name: INameI<'s, 'i>`.
- `PrototypeI[+T]` → `PrototypeI<'s, 'i>` (no inner generic).
- `ITemplataI[+T <: ITemplataType]` → `ITemplataI<'s, 'i>` monomorphic.

Leaf-name discrimination is done by pattern-matching on `local_name`. Use-site narrowing via `TryFrom`/`try_into().unwrap()`.

### 5.2 `IdI` — Allocated, Not Sealed

`ast/names.rs:16-20`:

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IdI<'s, 'i> {
    pub package_coord: &'s PackageCoordinate<'s>,
    pub init_steps: &'i [INameI<'s, 'i>],
    pub local_name: INameI<'s, 'i>,
}
```

**Currently no `_must_intern` seal** — `IdI` is allocated but not yet sealed, unlike its peers `IdT` (typing) and `IdH` (simplifying). Sealing is on the cleanup list (§17). Until then, `IdI`-keyed maps fall back to structural comparison — correct because the slice/local-name fields propagate ptr-eq through the derived impl.

### 5.3 `CoordI` — Two Fields, Inline Copy

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CoordI<'s, 'i> where 's: 'i {
    pub ownership: OwnershipI,
    pub kind: KindIT<'s, 'i>,
    _sealed: (),
}
```

`ast/types.rs:67-88`. **Two logical fields: ownership and kind.** No region field. This differs from both the typing-side `CoordT` (which has `region: RegionT`) and the simplifying-side `CoordH` (which has `location: LocationH`). Small, Copy, derives Eq/Hash, passed by value. Not interned — structural equality.

The `_sealed: ()` field keeps construction private to the module; use `CoordI::new(ownership, kind)` or `CoordI::void()`.

### 5.4 `KindIT` — 10 Variants, Inline Wrapper

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum KindIT<'s, 'i> where 's: 'i {
    NeverIT(NeverIT),
    VoidIT(VoidIT),
    IntIT(IntIT),
    BoolIT(BoolIT),
    StrIT(StrIT),
    FloatIT(FloatIT),
    StaticSizedArrayIT(&'i StaticSizedArrayIT<'s, 'i>),
    RuntimeSizedArrayIT(&'i RuntimeSizedArrayIT<'s, 'i>),
    StructIT(&'i StructIT<'s, 'i>),
    InterfaceIT(&'i InterfaceIT<'s, 'i>),
}
```

`ast/types.rs:92-103`. **No `KindPlaceholder` variant** — the instantiator's job is to eliminate placeholders by substitution. By the time you have a `KindIT`, every placeholder has been resolved. Placeholder handling lives on the input side: `translate_kind` (`instantiator.rs:2331`) matches `KindT::KindPlaceholder` and looks the id up in the substitutions map.

Primitives (Never/Void/Int/Bool/Str/Float) are inline `TFITCX` Value-types. The 4 compound payloads (`StructIT`, `InterfaceIT`, `StaticSizedArrayIT`, `RuntimeSizedArrayIT`) are arena-interned and held by `&'i` ref (per @WVSBIZ).

Sub-enum families: `ISubKindIT<'s, 'i>` and `ICitizenIT<'s, 'i>` — each with `StructIT` and `InterfaceIT` arms.

### 5.5 `INameI` Hierarchy — 72 Variants

`ast/names.rs:73-147`. The wide enum `INameI<'s, 'i>` has 72 variants spanning every name kind. Each variant holds a `&'i ConcreteNameI<'s, 'i>`.

Sub-enums (each a subset of `INameI`):

- `ITemplateNameI`, `IFunctionTemplateNameI`, `IInstantiationNameI`, `IFunctionNameI`, `IVarNameI`, `ICitizenNameI`, `IStructNameI`, `IInterfaceNameI`, `ISubKindNameI`, `ISuperKindNameI`, `IImplNameI`, `IImplTemplateNameI`, `IStructTemplateNameI`, `IInterfaceTemplateNameI`, `ICitizenTemplateNameI`, `ISubKindTemplateNameI`, `ISuperKindTemplateNameI`, `IRegionNameI`.

DAG-parity rule (typing §6.2): a concrete name extending multiple Scala sub-traits gets a variant in each Rust sub-enum. Widening (`From<Narrow> for Wide`) and narrowing (`TryFrom<Wide> for Narrow`) impls are hand-written.

The `INameValI<'s, 'i>` mirror has the same 72 variants but holds payloads by value (not `&'i` ref) — it's the interner's HashMap lookup key.

### 5.6 `ITemplataI` — 18 Variants

`ast/templata.rs:53-72`:

```rust
pub enum ITemplataI<'s, 'i> {
    Coord(CoordTemplataI),
    Kind(KindTemplataI),
    RuntimeSizedArrayTemplate(RuntimeSizedArrayTemplateTemplataI),
    StaticSizedArrayTemplate(StaticSizedArrayTemplateTemplataI),
    Function(FunctionTemplataI),                       // heavy
    StructDefinition(StructDefinitionTemplataI),       // heavy
    InterfaceDefinition(InterfaceDefinitionTemplataI), // heavy
    ImplDefinition(ImplDefinitionTemplataI),           // heavy
    Ownership(OwnershipTemplataI),
    Location(LocationTemplataI),
    Boolean(BooleanTemplataI),
    Integer(IntegerTemplataI),
    String(StringTemplataI),
    Prototype(PrototypeTemplataI),
    Isa(IsaTemplataI),
    CoordList(CoordListTemplataI),
    Region(RegionTemplataI),                           // instantiating-specific
    ExternFunction(ExternFunctionTemplataI),           // heavy
}
```

18 variants. The "heavy" templatas (Function/StructDefinition/InterfaceDefinition/ImplDefinition/ExternFunction) **do** survive into the instantiator — they were carried over by direct port from Scala's `templata.scala`. Heavy templatas don't carry their own bound-arg map; the bound info lives in `InstantiatedOutputsI` keyed by the templata's id.

`Region` is an instantiator-specific variant (Scala parity but the typing-pass `ITemplataT` doesn't have it because regions are inferred during typing without dedicated templatas; the instantiator materializes them).

### 5.7 `PrototypeI` and `SignatureI` — Interned, Sealed

`ast/ast.rs:302-306` and `:168-171`:

```rust
pub struct PrototypeI<'s, 'i> {
    pub id: IdI<'s, 'i>,
    pub return_type: CoordI<'s, 'i>,
    pub _must_intern: MustIntern,
}

pub struct SignatureI<'s, 'i> {
    pub id: IdI<'s, 'i>,
    pub _must_intern: MustIntern,
}
```

Both have `*ValI` mirrors and are interned via `intern_prototype_{si,ni,ci}` and `intern_signature_{si,ni,ci}`. Sealed with `_must_intern`.

### 5.8 `HinputsI` — Final Output

`ast/hinputs.rs:29-41`:

```rust
pub struct HinputsI<'s, 'i> where 's: 'i {
    pub interfaces: &'i [InterfaceDefinitionI<'s, 'i>],
    pub structs: &'i [&'i StructDefinitionI<'s, 'i>],
    pub functions: &'i [&'i FunctionDefinitionI<'s, 'i>],
    pub interface_to_edge_blueprints:
        ArenaIndexMap<'i, IdI<'s, 'i>, InterfaceEdgeBlueprintI<'s, 'i>>,
    pub interface_to_sub_citizen_to_edge:
        ArenaIndexMap<'i, IdI<'s, 'i>, ArenaIndexMap<'i, IdI<'s, 'i>, EdgeI<'s, 'i>>>,
    pub kind_exports: &'i [KindExportI<'s, 'i>],
    pub function_exports: &'i [FunctionExportI<'s, 'i>],
    pub kind_externs: ArenaIndexMap<'i, &'i StructIT<'s, 'i>, KindExternI<'s, 'i>>,
    pub function_externs: &'i [FunctionExternI<'s, 'i>],
}
```

**Hybrid shape**: definition lists as `&'i` slices, lookup tables as `ArenaIndexMap`. Notable subtlety: `interfaces` holds `InterfaceDefinitionI` **by value** (inline), while `structs` and `functions` hold `&'i T` refs. This asymmetry is because `interfaces` is assembled inline in `translate_method` from accumulator data, whereas `structs` and `functions` are already-arena-allocated refs pulled from `monouts.structs.values()` / `monouts.functions.values()`. See §17 for cleanup.

### 5.9 Definition Types

`FunctionDefinitionI<'s, 'i>` (`ast/ast.rs:100-105`), `StructDefinitionI<'s, 'i>` (`ast/citizens.rs:19-28`), and `InterfaceDefinitionI<'s, 'i>` (`ast/citizens.rs:95-103`) all take the same two lifetimes and no other generics.

```rust
pub struct FunctionDefinitionI<'s, 'i> where 's: 'i {
    pub header: FunctionHeaderI<'s, 'i>,
    pub rune_to_func_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i>>,
    pub rune_to_impl_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i>>,
    pub body: ReferenceExpressionIE<'s, 'i>,
}
```

`StructDefinitionI` (`ast/citizens.rs:19-28`) carries `sharedness: SharednessI`, `weakable: bool`, `is_closure: bool`, `members: &'i [StructMemberI<'s, 'i>]`, plus the two rune-to-bound maps. `InterfaceDefinitionI` (`ast/citizens.rs:95-103`) has the same shape minus members, plus `internal_methods: &'i [(&'i PrototypeI<'s, 'i>, i32)]`.

**There is no `ImplDefinitionI`.** Impls are stored in `InstantiatedOutputsI::impls` as a tuple: `(ICitizenIT, IdI, DenizenBoundToDenizenCallerBoundArgI, InstantiationBoundArgumentsI)` (see §3.1).

---

## Part 6: Expression AST

### 6.1 Three Enums

`ast/expressions.rs`:

- `ExpressionIE<'s, 'i>` (`:22-25`) — 2-variant dispatcher (`Reference` / `Address`).
- `ReferenceExpressionIE<'s, 'i>` (`:93-142`) — 49 variants, each holding `&'i Payload`.
- `AddressExpressionIE<'s, 'i>` (`:146+`) — 5 variants (`LocalLookup`, `StaticSizedArrayLookup`, `RuntimeSizedArrayLookup`, `ReferenceMemberLookup`, `AddressMemberLookup`).

Every expression type and all payload structs carry `<'s, 'i>` only. Return types:

- `InstantiatorI::translate_ref_expr` returns `(CoordI<'s, 'i>, ReferenceExpressionIE<'s, 'i>)` (`instantiator.rs:1377`).
- `InstantiatorI::translate_addr_expr` returns something similar (`:1293`).

### 6.2 Arena-Allocated, Not Interned

Expression nodes allocate into `'i` without deduping. Sub-expressions are `&'i Payload` refs; collections are `&'i [T]` slices. No `*ValI` companions on expressions.

### 6.3 Derives

`#[derive(Copy, Clone, Debug)]` only — Scala's `vcurious` (panic on equals) on every expression case class becomes Rust's "no `PartialEq`/`Hash`/`Eq` impl," which gives a strictly stronger compile-time error. `ConstantFloatIE` holds `f64`, which couldn't derive `Eq`/`Hash` anyway. Inline result types (like `IExpressionResultI`) keep full Copy/Eq/Hash.

### 6.4 Visitor/Collector

`collector.rs` (400 lines) — Rust-only port of Scala's reflective `Collector`, specialized for the I-side value AST. `NodeRefI` enum with 14 variants (`Prototype`, `Id`, `Name`, `Coord`, `Kind`, `Templata`, `FunctionDefinition`, `ReferenceExpression`, `LetNormal`, `FunctionCall`, plus a few more) + `all_in_*` / `only_in_*` helpers + `collect_*_inode!` macros. The expression-hierarchy walkers are partial — extend as use sites require more coverage.

---

## Part 7: Bound-Arg Threading

### 7.1 `InstantiationBoundArgumentsI` — Per-Site Storage

`ast/hinputs.rs:18-23`:

```rust
pub struct InstantiationBoundArgumentsI<'s, 'i> where 's: 'i {
    pub rune_to_function_bound_arg:
        ArenaIndexMap<'i, IRuneS<'s>, &'i PrototypeI<'s, 'i>>,
    pub caller_rune_to_callee_rune_to_reachable_func:
        ArenaIndexMap<'i, IRuneS<'s>,
            ArenaIndexMap<'i, IRuneS<'s>, &'i PrototypeI<'s, 'i>>>,
    pub rune_to_impl_bound_arg:
        ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i>>,
}
```

Three maps:

- `rune_to_function_bound_arg` — for each function bound the call site needs, which concrete function satisfies it.
- `caller_rune_to_callee_rune_to_reachable_func` — for nested bounds (caller-side rune → callee-side rune → satisfying function).
- `rune_to_impl_bound_arg` — for each impl bound, the satisfying impl id.

### 7.2 `DenizenBoundToDenizenCallerBoundArgI` — Per-Call Threading

See §2.5. The two `IndexMap`s map typing-side bound *parameter* ids to instantiating-side bound *argument* prototypes/ids. Threaded as `&` parameter through ~50 method signatures. Combined via `.plus()` (`union_maps_expect_no_conflict`).

Stack-owned, cloned liberally (Scala used GC'd `Map[IdT, PrototypeI]`; Rust uses `IndexMap` with `.clone()`). Not arena-allocated.

### 7.3 `assemble_instantiation_bound_param_to_arg`

`instantiator.rs:591-622` materializes a `DenizenBoundToDenizenCallerBoundArgI` from a `(callee's instantiation_bound_params, caller's supplied instantiation_bound_args)` pair by zipping them on rune. Used at the start of `translate_impl`, `translate_function`, `translate_abstract_func`, and the citizen-definition translators.

### 7.4 Stale-Snapshot Trap

`DenizenBoundToDenizenCallerBoundArgI` is captured at the *call site* where instantiation is decided. If you reuse one across multiple call sites (e.g. iterating over methods of an interface and using the interface's bound-arg map for each method), you may apply the wrong substitution. Mirror Scala's pattern: re-compute the bound-arg map per call site, even when it looks redundant.

---

## Part 8: Error Handling

**No `Result`, no `ICompileErrorI`.** The instantiator uses `panic!` exclusively for unimplemented branches and `vassert*`-style assertions for invariants.

- `pub fn translate(...) -> HinputsI<'s, 'i>` at `instantiator.rs:268` — direct return, no `Result`.
- Scala equivalent `def translate(...): HinputsI` at `Instantiator.scala` — also direct return.
- `grep -n "ICompileErrorI"` over both `FrontendRust/src/instantiating/` and `Frontend/InstantiatingPass/` returns zero hits. The type doesn't exist on either side.

The contract: typing-pass output is already valid; all user-level errors were caught earlier. Anything the instantiator hits internally is a programmer-error invariant violation → `panic!`. A separate `Result<HinputsT, ICompileErrorT>` lives on `InstantiatedCompilation::get_compiler_outputs` (it propagates the typing-side error type unchanged from earlier passes), but instantiator-side operations all return values directly.

For unmigrated branches: `panic!("Unimplemented: <branch_name>")` with a unique identifying message.

---

## Part 9: Driving the Pass

### 9.1 Top-Level Entry

Free function at `instantiator.rs:268-273`:

```rust
pub fn translate<'s, 'ctx, 't, 'i>(
    opts: &'ctx GlobalOptions,
    interner: &'ctx InstantiatingInterner<'s, 'i>,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    keywords: &'ctx Keywords<'s>,
    hinputs: &'ctx HinputsT<'s, 't>,
) -> HinputsI<'s, 'i>
where 's: 't, 's: 'i,
{
    let mut monouts = InstantiatedOutputsI::new();
    let instantiator = InstantiatorI { opts, interner, typing_interner, keywords, hinputs };
    instantiator.translate_method(&mut monouts)
}
```

Constructs both `InstantiatedOutputsI::new()` (`:234`) and the `InstantiatorI` value, then calls `translate_method` (Scala's `translate()` renamed to disambiguate from the free fn).

### 9.2 `translate_method` Top-Level Body

`instantiator.rs:288-542`. Real code (not a stub), ~250 LOC:

1. Destructure `HinputsT`.
2. Translate `kind_exports_t` and `function_exports_t` (these seed the worklists by triggering inner `translate_prototype` / `translate_kind` calls).
3. Translate non-generic function externs (generic externs are deferred to callsite-time).
4. Worklist drain loop (§3.2): while any of `new_functions` / `new_impls` / `new_abstract_funcs` is non-empty, pop one and process.
5. Assemble final `interface_edge_blueprints` and `interface_to_sub_citizen_to_edge` tables from accumulator data.
6. Pack `HinputsI` and return.

Each `translate_*_callsite` method receives `monouts`, the T-side definition, the pre-computed I-side id, and the caller-supplied `InstantiationBoundArgumentsI`. Its first check is idempotency: if the id already lives in `monouts.functions` / `structs` / `interfaces_without_methods` / `impls`, early-return. Otherwise translate and push newly-discovered instantiations onto the appropriate `new_*` worklist.

### 9.3 `InstantiatedCompilation` Coordination Wrapper

`instantiated_compilation.rs:41-52`. 5-lifetime struct (`<'s, 'ctx, 't, 'i, 'p>`) that owns:

- A `TypingPassCompilation` (which transitively owns typing arena and upstream passes).
- The `InstantiatingInterner` (built from the externally-owned `'i` Bump passed to `new`).
- A cached `monouts_cache: Option<HinputsI<'s, 'i>>` (lazy compute-on-first-access).

Consumer entry: `get_monouts(&mut self) -> &HinputsI<'s, 'i>` (`:145`) calls `instantiator::translate` once and caches.

### 9.4 Hammer Handoff

The hammer pass receives `&HinputsI<'s, 'i>` plus the instantiating interner reference. Caller drop order at end-of-scope (`pass_manager.rs` declaration order, reversed for drop): `'i` dies first, then `'h`, `'t`, `'s`. Since `HinputsI<'s, 'i>` doesn't structurally borrow `'t` data, the typing arena could theoretically drop after the instantiator finishes — but the canonical driver doesn't take advantage of this.

---

## Part 10: Invariants Summary

1. **`InstantiatingInterner` does not carry `'t`** — by design, so the typing arena could drop after instantiation.
2. **`MustIntern` seal preserves canonical pointer identity** on `&'i StructIT`, `&'i InterfaceIT`, `&'i StaticSizedArrayIT`, `&'i RuntimeSizedArrayIT`, `&'i PrototypeI`, `&'i SignatureI`, and the ~72 concrete `*NameI` types. `IdI` is the unsealed exception (pending future sealing).
3. **Worklists are `Vec`, drained FIFO via `remove(0)`.** Not `VecDeque`. Performance-acceptable because queue lengths are small.
4. **Keys in `InstantiatedOutputsI` are by-value, never `PtrKey`.** I-side interned types have derived `PartialEq`/`Hash` that delegate to interned-ref fields.
5. **Stamp-once idempotency required** — the call graph has cycles. Every top-level `translate_*_callsite` checks the relevant `_to_sharedness` or accumulator IndexMap and returns early if already processed.
6. **`InstantiatedOutputsI` is stack-owned**, but its arena-allocated values survive past its drop. AASSNCMCX exception is for the accumulator itself; payloads it holds follow the rule.
7. **Interner namespaces are still tri-modal** (`_si` / `_ni` / `_ci`) even though the R generic is gone. Callers pick the suffix at the callsite. Cross-namespace pointer identity is NOT preserved — a `_si`-interned prototype and a `_ci`-interned prototype with structurally-identical val payloads have distinct `&'i` pointers.

---

## Part 11: Recurring Traps

1. **Forgetting `where 's: 'i`** on new output types. Rust won't infer the outlives; the rebase will fail at the first call site that tries to thread `&'s` data through. Declare it on the struct.
2. **Introducing an environment type.** Tempting when the substitution map gets threaded through 10 method signatures. Scala parity wins — don't.
3. **Mutating arena-allocated values.** `InstantiatedOutputsI` is stack — fine to mutate. `HinputsI` is arena — immutable after construction; produce a fresh allocation if you need a changed version.
4. **Skipping a worklist push.** A `translate_function` body that omits the "push newly-discovered call onto `new_functions`" step silently fails to monomorphize the callee. Mirror the Scala recursion exactly.
5. **Two-interner field confusion.** `InstantiatorI` holds both `interner` (I-side) and `typing_interner` (T-side). Pattern-matching `coord_t.kind` (T-side) needs `typing_interner`-side variants; constructing the corresponding `CoordI` needs `interner`-side helpers. Convention: if your local is named `*_t`, use `typing_interner`; otherwise use `interner`.
6. **`vassert_one` vs `vassert_some`.** `vassert_one(coll.iter().filter(...))` is the idiomatic "expect exactly one match" pattern that asserts uniqueness *and* presence. Don't substitute `coll.iter().find(...).unwrap()` — that's `vassert_some` and only checks presence.
7. **`IdI` not sealed.** `IdI`-keyed maps fall back to structural eq/hash. Fine in practice (interned slice/inner-enum compare gives canonical identity) but if you see `IdI` equality behavior diverge from your expectation, check whether `init_steps` slices were arena-interned vs heap-allocated.
8. **Sharedness vs "mutability".** The Scala side uses `MutabilityI` (Mutable/Immutable). The Rust side uses `SharednessI` (Single/Shared). When porting a Scala method that names its return `mutability`, keep the Rust field name `sharedness` and adapt the value construction accordingly. `translate_mutability` (`:933`) is the Rust bridge and returns a `SharednessI` — the name is preserved for Scala parity but the value has migrated.
9. **Picking the wrong interner suffix.** `intern_prototype_si` and `intern_prototype_ci` produce identically-typed `&'i PrototypeI<'s, 'i>` values but from disjoint hashmaps. If you `intern_prototype_ci` a value that a downstream site expects to find via `intern_prototype_si`, canonical pointer equality won't hold across the two. Follow the convention: `_si` at T→I boundary, `_ni` at worklist enqueue, `_ci` at final emit.

---

## Part 12: Acronyms

Cross-references for acronyms used in this doc:

- **TFITCX** — "Types Fit Into These Categories". Tagged on Value-type variants (primitives inline, compound refs arena-interned).
- **WVSBIZ** — phantom-ref discipline in `KindIT`. Primitives inline; compounds `&'i`-interned.
- **PVECFPZ** — Polyvalue Equality / Casting discipline. Wrapper enums derive Eq/Hash; never hand-roll `ptr::eq` on the outer ref.
- **SICZ** — Sealed-Interned-Construction. `MustIntern(())` sealed by module-private unit.
- **PASDZ** — Placeholder Authoritative-Source = Denizen. Placeholder ids are path-encoded; the flat single-level substitutions map suffices.
- **NUSLX** — Never Use Static Lifetime. Breaks ptr-eq for interned types.
- **AASSNCMCX** — Arena Allocated Structs Should Not Contain Maps / Containers. Exceptions: stack-owned accumulators (`InstantiatedOutputsI`, `CompilerOutputs`) — heap maps are fine because they die with the stack frame.
- **HRALII** — ownership-splitting (BorrowI → MutableBorrow/ImmutableBorrow, ShareI → MutableShare/ImmutableShare). The instantiator turns ambiguous borrow/share into a specific Mutable/Immutable form based on region mutability.
- **LCCSL** — Lambdas Can Call Sibling Lambdas. Edge case handled in `translate_prototype`.
- **LCCPGB**, **LCNBAFA** — lambda-to-lambda bound passing.
- **LHPCTLD** — Last Has Placeholders, Containing Top Level Denizen. Only top-level denizens have placeholders; the substitution map is shared with containing lambdas.
- **NBIFP** — bound hoisting through parameters (commented-out code preserved as audit trail).
- **TIBANFC** — Translate Impl Bound Argument Names For Case. Definition in `docs/Generics.md`.
- **IPOMFIC**, **TTTDRM**, **CSHROOR**, **DUDEWCD**, **DINSIE**, **BRCOBS**, **RMLRMO**, **AUMAP**, **HCCSCS**, **NPFCASTN**, **DDSOT**, **NNSPAFOC**, **EHCFBD**, **IMRFDI**, **UINIT**, **PRIIROZ**, **SMLRZ**, **LAGTNGZ**, **TNAD**, **MFBFDP**, **OMCNAGP**, **CDFGI** — various spot citations. Most defined in `docs/`; see grep.

---

## Part 13: Key Files / Directories

| Path | Lines | Purpose |
|---|---|---|
| `docs/architecture/instantiator-design.md` | | this doc |
| `docs/architecture/typing-pass-design-v3.md` | | upstream pass design (produces `HinputsT`) |
| `docs/architecture/simplifier-design.md` | | downstream pass design (consumes `HinputsI`) |
| `docs/Generics.md` | | TIBANFC, MKRFA, etc. |
| `FrontendRust/src/instantiating/mod.rs` | 8 | module declarations |
| `FrontendRust/src/instantiating/instantiating_arena.rs` | 20 | `InstantiatingArena<'i>` newtype |
| `FrontendRust/src/instantiating/instantiating_interner.rs` | 836 | 12-family interner (4 kinds × 3 namespaces), ~216 macro-generated wrappers, `paste!`-based codegen |
| `FrontendRust/src/instantiating/instantiator.rs` | 2717 | `InstantiatorI` god struct, `InstantiatedOutputsI`, 57 `translate_*` methods, `translate_method` driver |
| `FrontendRust/src/instantiating/collector.rs` | 400 | `NodeRefI` visitor + `collect_*_inode!` macros |
| `FrontendRust/src/instantiating/instantiated_compilation.rs` | 163 | 5-lifetime driver wrapper, lazy `monouts_cache` |
| `FrontendRust/src/instantiating/instantiated_humanizer.rs` | 189 | I-side pretty-printer (sparse) |
| `FrontendRust/src/instantiating/ast/mod.rs` | 11 | AST module declarations |
| `FrontendRust/src/instantiating/ast/types.rs` | 304 | `CoordI`, `KindIT`, `SharednessI`, kind payloads, atom enums |
| `FrontendRust/src/instantiating/ast/names.rs` | 1799 | `IdI`, `INameI` (72 variants), all `*NameI` structs, sub-enum families, TryFrom/From conversions |
| `FrontendRust/src/instantiating/ast/ast.rs` | 420 | `PrototypeI`, `SignatureI`, `FunctionDefinitionI`, `FunctionHeaderI`, exports/externs, edges, attributes, local variables |
| `FrontendRust/src/instantiating/ast/templata.rs` | 242 | `ITemplataI` (18 variants) + payload structs |
| `FrontendRust/src/instantiating/ast/citizens.rs` | 113 | `StructDefinitionI`, `InterfaceDefinitionI`, `IMemberTypeI` |
| `FrontendRust/src/instantiating/ast/expressions.rs` | 834 | `ExpressionIE`/`ReferenceExpressionIE`/`AddressExpressionIE` + ~50 payloads |
| `FrontendRust/src/instantiating/ast/hinputs.rs` | 223 | `HinputsI`, `InstantiationBoundArgumentsI`, lookup methods |
| `FrontendRust/src/pass_manager/pass_manager.rs` | | top-level driver; arena construction order |
| `Frontend/InstantiatingPass/src/dev/vale/instantiating/Instantiator.scala` | | Scala source of truth — line-for-line port target |
| `Frontend/InstantiatingPass/src/dev/vale/instantiating/ast/*.scala` | | Scala AST source |

Removed files (no longer exist, from a prior version of this pass): `region_counter.rs`, `region_collapser_individual.rs`, `region_collapser_consistent.rs`, `reintern.rs`, `docs/InstantiatorRegions.md`.

---

## Part 14: Long-Term Cleanup TODO

Items below diverge from line-for-line Scala parity; check with the architect before starting. They are ordered by combined "value × independence."

### Priority 1

**1.1 Collapse the interner's three-namespace suffix scheme.**
The `R: IRegionsModeIT` generic is gone from every I-side type, but the interner still exposes `_si`/`_ni`/`_ci` suffixed families backed by 12 hashmaps (4 kinds × 3 namespaces). Post-R, the three families produce identically-typed output; the namespace separation is now purely convention. Scope of work:

- Merge the three per-namespace HashMap families into one per kind (4 total).
- Collapse `intern_prototype_{si,ni,ci}` into `intern_prototype`; same for signature, name, kind payloads.
- Drop the four `impl_intern_name_wrappers*` macros — replace with a single simpler macro (or hand-written wrappers, since 72 × 1 = 72 is more tractable than 72 × 3 = 216).
- Delete the `paste = "1"` dependency from `FrontendRust/Cargo.toml` — no per-suffix identifier construction needed anymore.
- Update all callsites (currently the code picks a suffix by convention; every callsite becomes a bare `intern_*` call).
- Estimated savings: ~500 lines from the interner, plus the paste dependency.

**Verification before starting:** grep for any callsite that intentionally *relies* on cross-namespace pointer inequality. If a prototype is `_si`-interned and its `_ci`-interned twin is meant to be a distinct arena value for some downstream reason, that reason needs to hold up under merged families.

**1.2 Seal `IdI`.**
Bring `IdI` in line with its peers (`IdT`, `IdH`, all `*IT`/`PrototypeI`/`SignatureI`/`*NameI` types). Pattern to copy verbatim from `FrontendRust/src/typing/names/names.rs` and `FrontendRust/src/typing/typing_interner.rs`:

- Add `pub _must_intern: MustIntern` field to `IdI`.
- Define `IdIValI<'s, 'i, 'tmp>` mirror with a third `'tmp` lifetime so callers can build transient queries from stack-allocated slices.
- Add `intern_id(&self, val: IdIValI<...>) -> &'i IdI<...>` method to `InstantiatingInterner`.
- Convert all current `IdI { ... }` construction sites to `interner.intern_id(IdIValI { ... })`.
- Update `IdI`'s derived `Hash`/`Eq` to use canonical pointer identity via the seal (typing pass shows the pattern).

Eliminates the "is `IdI` interned or just allocated?" caveat that the design has had to keep explaining.

### Priority 2

**2.1 Delete `humanize_signature` from `instantiated_humanizer.rs`.**
Dead code in both Scala and Rust — no call site invokes it. Removes an outdated `panic!` from `instantiated_humanizer.rs`.

**2.2 Delete `ICitizenDefinitionI` if it remains unused.**
The enum dispatcher in `ast/citizens.rs` is declared per architect-directed NEDCX but most call sites pattern-match directly on `StructDefinitionI` / `InterfaceDefinitionI`. Audit usage; if no live code depends on the wrapper, delete it. If something does depend on it, leave alone — the architect's call was deliberate.

**2.3 Switch worklists from `Vec` + `remove(0)` to `VecDeque` + `pop_front()`.**
Three-field change in `InstantiatedOutputsI` plus drain-loop in `translate_method`. Brings drain from O(n) to O(1) and matches Scala's `mutable.Queue` + `dequeue()` semantics directly.

### Priority 3

**3.1 Fix `HinputsI` slice-vs-ref asymmetry.**
Currently `interfaces: &'i [InterfaceDefinitionI<...>]` (inline) versus `structs: &'i [&'i StructDefinitionI<...>]` (by ref) — artifact of `translate_method`'s assembly, not intentional. Pick one shape (`&'i [&'i T]` for all three is the natural target since structs/functions already match it) and make the interface assembly arena-allocate the `InterfaceDefinitionI` values up front.

**3.2 Address `interfaces_without_methods` naming.**
The field holds transient state (interface definitions built with `internal_methods: &[]`, later discarded and rebuilt with methods attached via v-table assembly). The current name matches Scala (`interfacesWithoutMethods`) but actively misleads about the relationship between map contents and final emitted `HinputsI`. Options:

- Keep the name (Scala parity), add a comment explaining the discard-and-rebuild.
- Rename to `pending_interface_definitions` or similar.
- Restructure to use a dedicated `InterfaceDefinitionStubI` type for the transient state, distinct from final `InterfaceDefinitionI`. Heaviest but cleanest — the type system enforces the distinction.

Lean toward option 2 unless Scala parity is hard.
