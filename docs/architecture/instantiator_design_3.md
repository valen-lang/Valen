# Instantiating Pass Design

Architecture and design decisions for the Scala-to-Rust instantiating-pass migration. Companion to `docs/architecture/typing-pass-design-v3.md` and `docs/architecture/simplifying_pass_design.md`. Operational handoff in `migrate-tl.md` at the repo root.

This pass takes typing-pass output (`HinputsT<'s, 't>`) and produces fully-monomorphized output (`HinputsI<'s, 'i>`) for the simplifying pass. It is the pass that resolves generics into concrete instantiations — every templated function/struct/interface gets a concrete monomorphized clone per use-site. Generic placeholders become concrete types; bound arguments get threaded into v-tables; region modes get collapsed from `sI` (still-placeholdered) through `nI` (per-denizen renumbered) to `cI` (concrete output-ready).

For migration policy values (Val/Ref pairs, lifetime conventions, region-mode handling), see `FrontendRust/docs/migration/migration-policy.md` instantiating rows.

**Current migration status (mid-Slab-16).** Most type-level scaffolding is in place; ~109 `translate_*` methods on `InstantiatorI` are mostly `panic!` stubs awaiting body migration. The architectural design described here is fully landed at the type level; body migration is ongoing.

---

## Part 1: Arena and Lifetime Model

The instantiating pass introduces a single new arena lifetime (`'i`) on top of the lifetimes inherited from upstream (`'p`, `'s`, `'t`). It also introduces a **type-level region-mode generic parameter `R: IRegionsModeI`** that flows through every I-suffix type — the headline architectural concept (see Part 5).

### 1.1 Four Arenas

- **`'p` — Parser arena (`ParseArena<'p>`)**: parser AST, parser-side `StrI<'p>`. Read-only by this point.
- **`'s` — Scout arena (`ScoutArena<'s>`)**: interned scout names (`INameS<'s>`, `IRuneS<'s>`, `IImpreciseNameS<'s>`), source strings (`StrI<'s>`), code locations. Read freely.
- **`'t` — Typing arena (`TypingInterner<'s, 't>`)**: input data for the instantiator (`HinputsT<'s, 't>` + everything it reaches: `IdT`, `INameT`, `KindT`, `ITemplataT`, `PrototypeT`, env structs, expression AST). Read-only here.
- **`'i` — Instantiating arena (`InstantiatingInterner<'s, 'i>`)**: instantiating-pass output. Holds all I-suffix interned types and the allocated-but-not-interned I-side AST.

`InstantiatingArena<'i>` (`instantiating_arena.rs:10-20`) is a thin newtype around `&'i Bump`. `InstantiatingInterner<'s, 'i>` (`instantiating_interner.rs:43-69`) wraps the bump plus a `RefCell<Inner>` holding **12 HashMap families** (see §4).

### 1.2 Lifetime Invariants

1. **`'s: 'i`** — interner-level invariant. Carried into every I-suffix type that holds `&'s` refs to scout-arena data (which is most of them).
2. **`'t` is intentionally NOT carried on the `InstantiatingInterner`** (`instantiating_interner.rs:14-16` doc comment). `HinputsI<'s, 'i>` needs to outlive `'t` so the simplifying pass can drop the typing arena before consuming I-side data. The instantiator itself carries `<'s, 'ctx, 't, 'i>` because it bridges the T→I boundary, but its *output* only carries `<'s, 'i>`.
3. **`'s: 't`** — inherited from the typing arena. The combined where-clause on `InstantiatorI` is `where 's: 't, 's: 'i` (`instantiator.rs:235`).
4. **`'i` outlives the simplifying pass** because `HinputsI<'s, 'i>` is the input to Hammer.
5. **Arena borrow convention.** Same pattern as typing — write `&InstantiatingInterner<'s, 'i>` or `&'ctx InstantiatingInterner<'s, 'i>`, never `&'i InstantiatingInterner<'s, 'i>`. `InstantiatingInterner::alloc(&self, T) -> &'i T` returns arena-lifetimed data from a short `&self` borrow.

### 1.3 Arena Construction Order

```
fn top_level_driver() {
    let parse_arena = ParseArena::new();
    let scout_arena = ScoutArena::new();
    let typing_interner = TypingInterner::new(...);
    let hinputs_t = typing::run_typing_pass(...);              // HinputsT<'s, 't>

    let instantiating_bump = Bump::new();
    let instantiating_interner = InstantiatingInterner::new(&instantiating_bump);  // 'i
    let hinputs_i = instantiating::translate(
        &opts, &instantiating_interner, &typing_interner,
        &keywords, &hinputs_t);                                // HinputsI<'s, 'i>

    // typing_interner can drop here — instantiating output is in 'i
    drop(typing_interner);

    // ... simplifying pass consumes hinputs_i ...

    // instantiating_interner drops: 'i dies
    // scout_arena drops: 's dies
    // parse_arena drops: 'p dies
}
```

The intentional `'t`-absence on `InstantiatingInterner` lets the typing arena drop early — the alternative (keeping `'t` alive through simplifying) would inflate peak memory.

### 1.4 Where Each Type Lives

| Type | Lifetimes | Location |
|---|---|---|
| `HinputsI` | `<'s, 'i>` | `'i`, allocated; holds `&'i` and `&'s` refs throughout |
| `InstantiationBoundArgumentsI` | `<'s, 'i>` | `'i`, allocated, not interned |
| `IdI` | `<'s, 'i, R>` | `'i`, allocated (NOT yet sealed; see §4.6) |
| `PrototypeI` | `<'s, 'i, R>` | `'i`, interned, `MustIntern` seal |
| `SignatureI` | `<'s, 'i, R>` | `'i`, interned, `MustIntern` seal |
| `StructIT`, `InterfaceIT`, `StaticSizedArrayIT`, `RuntimeSizedArrayIT` | `<'s, 'i, R>` | `'i`, interned, `MustIntern` seal |
| ~72 concrete `*NameI` structs | `<'s, 'i, R>` | `'i`, allocated; interned via `intern_name_*` |
| `INameI` | `<'s, 'i, R>` | inline Copy Polyvalue wrapper |
| `KindIT` | `<'s, 'i, R>` | inline Copy Polyvalue wrapper |
| `ITemplataI` | `<'s, 'i, R>` | inline Copy Polyvalue wrapper |
| `CoordI` | `<'s, 'i, R>` | inline Copy Value-type |
| `OwnershipI`, `MutabilityI`, `VariabilityI`, `LocationI` | none | inline Copy unit-variant enums |
| `ExpressionIE`, `ReferenceExpressionIE`, `AddressExpressionIE` | `<'s, 'i, R>` | inline Copy wrapper; variant payload `&'i XIE` |
| ~50 expression payload structs | `<'s, 'i, R>` | `'i`, allocated, not interned |
| `InstantiatorI` (god struct) | `<'s, 'ctx, 't, 'i>` | stack; dies at pass end |
| `InstantiatedOutputsI` (Monouts) | `<'s, 't, 'i>` | stack-owned; heap-backed `IndexMap`s + `Vec`s |
| `InstantiatedCompilation` (driver) | `<'s, 'ctx, 't, 'i, 'p>` | stack |

### 1.5 Type Inventory By Arena

- **`'t` typing arena (read-only input):** `HinputsT<'s, 't>` plus everything it transitively reaches.
- **`'i` instantiating arena, interned:** the 4 sealed kind payloads (`StructIT`, `InterfaceIT`, `StaticSizedArrayIT`, `RuntimeSizedArrayIT`), `PrototypeI`, `SignatureI`, ~72 concrete `*NameI` structs. Each interned type carries `_must_intern: MustIntern` and has a transient `*ValI` mirror.
- **`'i` instantiating arena, allocated but not interned:** `HinputsI`, `InstantiationBoundArgumentsI`, `IdI` (not sealed yet), `InterfaceEdgeBlueprintI`, `FunctionDefinitionI`, `FunctionHeaderI`, `StructDefinitionI`, `InterfaceDefinitionI`, `EdgeI`, `KindExportI`/`FunctionExportI`/`KindExternI`/`FunctionExternI`, all `ParameterI` / `IVariableI` / `ILocalVariableI` / `RegionI`, plus the ~50 expression payload structs and the 20 `ITemplataI` payload structs.
- **Polyvalue inline wrappers (16 bytes, not arena-stored, all carry `R`):** `KindIT` (10 variants), `INameI` (72 variants), `ITemplataI` (20 variants), `ICitizenIT`, `ISubKindIT`, the three expression wrappers (`ExpressionIE`/`ReferenceExpressionIE`/`AddressExpressionIE`).
- **Value-type primitives:** `CoordI`, `OwnershipI`, `MutabilityI`, `VariabilityI`, `LocationI`, primitive kind payloads.
- **Neither arena (stack/heap):** `InstantiatorI`, `InstantiatedOutputsI`, `InstantiatedCompilation`.

### 1.6 Mutual-Recursion Shape

Same as typing — the I-type graph is mutually recursive but every edge is `&'s`, `&'i`, or `<'s, 'i, R>`-parameterized inline. `CoordI<R> → KindIT<R> → StructIT<R> → IdI<R> → INameI<R> → ITemplataI<R> → CoordI<R>`. Same-type self-recursion via 16-byte inline sub-enum values — no `Box`.

The `R` parameter threads through this graph: an `sI`-mode CoordI's nested IdI is also `sI`, etc. This is type-level enforcement of "consistent region mode in a sub-tree" — see Part 5.

---

## Part 2: The God Struct (`InstantiatorI`)

### 2.1 Architecture

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

`instantiator.rs:235-243`. Immutable-only context. Mutable state threads through `&mut InstantiatedOutputsI<'s, 't, 'i>` (the "monouts" — see Part 3) on every call.

**Bridges T → I worlds.** The god struct carries both the typing-pass interner and the instantiating interner. Methods read T-side data (via `self.typing_interner`, `self.hinputs`) and write I-side data (via `self.interner`). The two arenas coexist throughout the pass; only after `translate()` returns does the caller drop the typing arena.

**Scala collapse note.** Unlike typing's `Compiler` (which collapses ~16 Scala sub-compilers) or simplifying's `Hammer` (which collapses ~10 sub-hammers), the Scala `Instantiator` is **already one big class** — no sub-instantiators to collapse. The "god struct" framing applies but the collapse story is degenerate. The Scala constructor signature is `(opts, interner, keywords, hinputs, monouts)`; the Rust port lifts everything except `monouts` to the struct (`monouts` stays per-call `&mut` per the typing-pass re-entrancy pattern).

### 2.2 `hinputs` on the God Struct (Different from Typing)

The typing-pass `Compiler` takes the typing-pass input as a method parameter on `compile_program`, not as a field. Instantiating differs: `hinputs: &'ctx HinputsT<'s, 't>` lives on the god struct. This matches Scala's `Instantiator` ctor signature directly.

Pragmatic reason: every `translate_*` method needs to look up T-side definitions (e.g. `hinputs.functions.find(...)` inside `translate_override` at `:1110-1115`), so threading `hinputs` as a parameter would add it to ~109 method signatures. Lifting to the struct is cleaner.

### 2.3 `&self` + `&mut monouts` Enables Re-entrancy

Same pattern as typing-pass-design-v3.md §2.2 / simplifying-pass-design.md §2.2. Deep mutual recursion (e.g. `translate_ref_expr → translate_call → translate_function → translate_ref_expr`) works because `&self` allows shared borrow and `&mut monouts` is re-borrowed at each call level.

### 2.4 Top-Level `translate` Free Function

`instantiator.rs:214-219`:

```rust
pub fn translate<'s, 'ctx, 't, 'i>(
    opts: &'ctx GlobalOptions,
    interner: &'ctx InstantiatingInterner<'s, 'i>,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    keywords: &'ctx Keywords<'s>,
    hinputs: &'ctx HinputsT<'s, 't>,
) -> HinputsI<'s, 'i>
where 's: 't, 's: 'i
```

Free function (not method) — constructs both `InstantiatedOutputsI::new()` and the `InstantiatorI` value, then calls `translate_method` (Scala's `translate()` renamed to avoid shadowing the free fn).

---

## Part 3: `InstantiatedOutputsI` (Monouts)

### 3.1 Structure

`instantiator.rs:86-108`. Mutable accumulator threaded through every method. The "Monouts" name comes from Scala's `monouts: InstantiatedOutputs` parameter — Rust collapses it onto a per-call `&mut`. 19 fields, all `IndexMap` or `Vec`, carries `<'s, 't, 'i>`:

- **Output collectors keyed by `cI`-mode ids**: `functions`, `structs`, `interfaces_without_methods`, `*_to_mutability` (3 of them), `interface_to_impls`, `interface_to_abstract_func_to_virtual_index`, `impls`, `abstract_func_to_bounds`, `interface_to_impl_to_abstract_prototype_to_override` (the v-table assembly cache).
- **Bounds storage keyed by `sI`-mode ids**: `struct_to_bounds`, `interface_to_bounds`, `impl_to_bounds`. The asymmetry — outputs in `cI`, bounds in `sI` — reflects that bound resolution happens *before* region collapse.
- **Work queues** (`Vec`, drained front-to-back): `new_functions`, `new_impls`, `new_abstract_funcs`. These drive the discovery loop (§3.2). As `translate_*` methods encounter previously-unseen instantiations, they push onto these queues; the main loop drains them.
- **Final outputs**: `kind_externs`, `function_externs`.

### 3.2 Worklist-Driven Discovery Loop

Inside `translate_method` at `instantiator.rs:406-436`:

```rust
loop {
    if let Some(func_to_translate) = monouts.new_functions.pop_front() {
        self.translate_function(monouts, ...);
    } else if let Some(impl_to_translate) = monouts.new_impls.pop_front() {
        self.translate_impl(monouts, ...);
    } else if let Some(abstract_func) = monouts.new_abstract_funcs.pop_front() {
        self.translate_abstract_func(monouts, ...);
    } else {
        break;
    }
}
```

This drives the entire pass after a single seed insertion (the exported functions). Each translate may push more work; eventually all queues empty.

Scala has additional `newStructs`/`newInterfaces` queues (preserved in the Rust file's Scala-comment audit trail); the Rust comment notes "We make structs and interfaces eagerly as we come across them" — those types are translated inline rather than queued. Functions, impls, and abstract funcs are deferred because they may depend on bounds discovered later in the same iteration.

### 3.3 No `PtrKey`

Unlike typing-pass `CompilerOutputs` (which uses `PtrKey<'t, T>` for HashMap keys on interned `&'t` refs), `InstantiatedOutputsI` uses `IndexMap` keyed by value-type `IdI`/`PrototypeI`. Reason: I-side interned types carry pointer-based identity through their `MustIntern` seal directly, so the keys can be Value-type and `IndexMap` works without a newtype wrapper.

`IndexMap` (not `HashMap`) is chosen for insertion-order iteration — important for output determinism when emitting `HinputsI`.

### 3.4 The One Mutation API

`add_method_to_v_table(impl_id, super_interface_id, abstract_prototype_c, override_prototype_c)` at `instantiator.rs:187-194` is the one named mutation method beyond field assignment. Called from `translate_override` to register a concrete v-table entry. Other monouts mutations are direct field manipulation (`monouts.functions.insert(...)`, `monouts.new_functions.push_back(...)`).

---

## Part 4: Interning Surface — `InstantiatingInterner`

### 4.1 The 12-HashMap Structure

`instantiating_interner.rs:51-69`. **12 HashMaps = 4 logical types × 3 region modes**:

| | `sI` | `nI` | `cI` |
|---|---|---|---|
| Kind payloads | `kind_payload_val_to_ref_si` | `..._ni` | `..._ci` |
| Prototype | `prototype_val_to_ref_si` | `..._ni` | `..._ci` |
| Signature | `signature_val_to_ref_si` | `..._ni` | `..._ci` |
| Name | `name_val_to_ref_si` | `..._ni` | `..._ci` |

This is the architectural symmetry that pays for the region-mode-as-generic-parameter design: every interned type exists in three pointer-distinct flavors so cross-mode references don't accidentally alias.

### 4.2 Family-Level Methods (12)

One `intern_*` per (logical type, region mode) pair:

- `intern_kind_payload_si` / `intern_kind_payload_ni` / `intern_kind_payload_ci` (`:261, :325, :389`) — dispatch the 4 sealed kind variants.
- `intern_prototype_si` / `intern_prototype_ni` / `intern_prototype_ci` (`:453, :470, :487`).
- `intern_signature_si` / `intern_signature_ni` / `intern_signature_ci` (`:508, :524, :540`).
- `intern_name_si` / `intern_name_ni` / `intern_name_ci` (`:569, :582, :595`).

### 4.3 Per-Concrete Wrapper Methods (~216)

Each interned variant gets its own typed convenience wrapper that dispatches through the family method. For the 4 sealed kind payloads × 3 modes = 12 methods (`:296-447`):
- `intern_struct_it_si` / `intern_struct_it_ni` / `intern_struct_it_ci`
- `intern_interface_it_si` / `intern_interface_it_ni` / `intern_interface_it_ci`
- `intern_static_sized_array_it_{si,ni,ci}`
- `intern_runtime_sized_array_it_{si,ni,ci}`

For names: **72 concrete `*NameI` types × 3 modes = 216 wrapper methods**. Hand-writing 216 methods would be miserable; they're generated by macros `impl_intern_name_wrappers!` and `impl_intern_name_wrappers_5lt!` at `:611-683`. The 5-lt variant exists for the 14 name structs that need `'p` and `'t` (parser+typing arenas) on top of `'s,'i,R` — those names hold `ITemplataI` references that transitively touch both.

### 4.4 Val/Ref Pair Pattern

Same as typing and simplifying. Each interned type `XI<'s,'i,R>` has a sibling `XValI<'s,'i,R>` (Value-type, derives Hash/Eq) used as the HashMap query key. The canonical `&'i XI` has `_must_intern: MustIntern(())` (`:26-27`) seal field.

```rust
pub fn intern_struct_it_ci(&self, val: StructITValI<'s, 'i, cI>) -> &'i StructIT<'s, 'i, cI> {
    let mut inner = self.inner.borrow_mut();
    if let Some(canonical) = inner.kind_payload_val_to_ref_ci.get(&val.into()) {
        // dispatch through family and downcast
        ...
    }
    // allocate, seal, insert, return
    ...
}
```

### 4.5 Arena Utilities

`:239-255` exposes raw arena allocation:
- `alloc<T>(T) -> &'i T`
- `alloc_slice_copy<T: Copy>(&[T]) -> &'i [T]`
- `alloc_slice_from_vec<T>(Vec<T>) -> &'i [T]`
- `alloc_index_map<K, V>() -> ArenaIndexMap<'i, K, V>`
- `alloc_index_map_from_iter<K, V, I>(I) -> ArenaIndexMap<'i, K, V>`

### 4.6 Deferred / Future Intern Families

`:685-687` notes:
- `intern_id_*` is not yet wired — `IdI` is allocated but not interned (no `MustIntern` seal yet). Adding it requires settling `IdI` shape stability, which is mid-Slab-16 work.
- `intern_templata_*` is intentionally not planned — `ITemplataI` is Value-type per Scala parity (`ITemplataT` likewise — typing pass agrees).

Tests are gated off (`:693`: `#[cfg(all(test, any()))]`) pending `IdI` maturation.

---

## Part 5: The Region-Mode Generic `R: IRegionsModeI`

The headline architectural concept. Every I-suffix type carries `R: IRegionsModeI` as a type parameter; `R` ranges over three closed-set modes.

### 5.1 Semantics

`ast/types.rs:152-199`. Three region modes, each a zero-member ZST struct:

- **`sI`** (`:175`) — "**s**ubjective" / "**s**till has placeholders" / "**s**ource". Per `reintern.rs:10-11`, **instantiation starts in `sI`** — the T → I boundary always produces `sI`. Placeholders for unresolved regions live here.
- **`nI`** (`:189`) — "**n**ew". Per the Scala comment at line 192: "Serves as a starting point for a new instantiation." Used as the queue-time mode for `new_impls`/`new_functions`/`new_abstract_funcs` after per-denizen region renumbering.
- **`cI`** (`:196`) — "**c**oncrete" / "**c**ollapsed". Fully-instantiated, region-modes fully resolved. The final `HinputsI` output is all `cI` (`hinputs.rs:47-58` — `interfaces`, `structs`, `functions` are all `cI`-typed).

**Closed-set sealing** via private `region_mode_sealed::Sealed` trait (`:157-160`). `IRegionsModeI` is the bound trait (`:160`). No external code can introduce a fourth region mode.

### 5.2 CCFCTS — `nI` and `cI` Are Type Aliases for `sI`

`ast/types.rs:179-185`:

```rust
pub type nI = sI;
pub type cI = sI;
```

This is **CCFCTS** ("Casting Collapsed From Compile-Time Source", documented at `:179-185`). The three modes are *inert* at runtime — there's nothing to compare or convert; they're purely compile-time markers. The type aliases let ASTs cast freely across modes without runtime cost.

**Why type aliases instead of newtypes?** Two reasons:
1. Erases Scala's `nI <: sI` covariance (Scala's mode hierarchy with subtyping). Rust doesn't have subtyping; type aliases sidestep the issue.
2. Enables the `unsafe std::mem::transmute(templata_c)` at `instantiator.rs:1131` — a documented exploitation of the aliasing to reuse a `cI` value as `sI` in one tight spot in `translate_override`. The transmute is sound because the types are layout-identical.

**The cost of the aliasing.** If anyone adds a member to `sI`/`nI`/`cI`, the aliasing breaks and the transmute becomes UB. The zero-member invariant is load-bearing. Sealing keeps external code from violating it; internal discipline keeps the file from breaking it.

### 5.3 Which Types Carry R

**Almost everything I-suffix**:
- `CoordI<'s, 'i, R>` (`ast/types.rs:215-219`)
- `KindIT<'s, 'i, R>` (`:258-269`) — every variant carries R; concrete payload structs (`StructIT<'s,'i,R>` etc.) also carry R
- `INameI<'s, 'i, R>` (`ast/names.rs:135-209`) — 72 variants, all carry R
- `INameValI<'s, 'i, R>` — the Val mirror also carries R
- `IdI<'s, 'i, R>` (`:25-29`)
- All ~72 concrete `*NameI` structs
- `ITemplataI<'s, 'i, R>` (`ast/templata.rs:133-154`) — 20 variants, payloads each carry R
- `PrototypeI<'s, 'i, R>` (`ast/ast.rs:667`), `SignatureI<'s, 'i, R>` (`:376`)
- Every expression payload struct in `ast/expressions.rs`
- The three expression wrappers (`ExpressionIE`/`ReferenceExpressionIE`/`AddressExpressionIE`)

**Not carrying R**:
- `IRegionsModeI` trait itself (it's the trait that defines R).
- Simple atoms: `OwnershipI`, `MutabilityI`, `VariabilityI`, `LocationI`.
- `InstantiationBoundArgumentsI` — its components reference `sI` explicitly via field types.
- `HinputsI<'s, 'i>` (final output) — each field hard-codes `cI` rather than being R-parameterized.
- Some I-side primitives that need R for type-system bookkeeping only carry `PhantomData<R>`.

### 5.4 The Collapse Pipeline

The instantiator does its work in `sI`, then runs two collapse passes to produce `nI` (per-denizen renumbered) and `cI` (concrete output).

**`region_counter.rs` (970 lines)** — walks an `sI` AST and produces a `HashMap<i32, i32>` region renumbering map. Entry points: `count_id`, `count_prototype`, `count_function_id`, `count_kind`, etc. (`:108-680`). Pure computation; no mutation, no interning.

**`region_collapser_consistent.rs` (830 lines)** — `sI → nI` collapse. Takes the external renumbering map produced by `region_counter`. Used at queue-time to canonicalize the region naming per denizen (so two distinct call-sites of the same templated function with different placeholder ids get the same nI form, enabling work-queue deduplication). API examples:
- `collapse_prototype(interner, map, &PrototypeI<sI>) -> PrototypeI<nI>` (`:20`)
- `collapse_id`, `collapse_function_id`, `collapse_kind`, `collapse_coord`, `collapse_templata`, `collapse_struct_id`, `collapse_interface_id`, `collapse_impl_id`, `collapse_struct_name`, `collapse_interface_name`, `collapse_impl_name`, `collapse_export_id`, `collapse_extern_id`, `collapse_citizen`, `collapse_runtime_sized_array`, `collapse_static_sized_array`

**`region_collapser_individual.rs` (824 lines)** — `sI → cI` collapse. Generates a fresh renumbering map per node (per-individual-collapse, hence the name). Used during final output emission. Parallel API surface to `region_collapser_consistent`:
- `collapse_prototype(interner, &PrototypeI<sI>) -> PrototypeI<cI>` (`:25-32`)

Both files have ~30 functions each, mirrored 1:1 — the only difference is whether they take an external map (`consistent`) or build their own (`individual`). Both call into `region_counter.rs` to count regions before collapsing.

**`@ICRHRC`** (`region_collapser_consistent.rs:7`, `region_collapser_individual.rs:9`) — the doc-acronym for "why/when we count the regions". Definition not currently in `docs/` — opportunity for a future writeup.

### 5.5 The `IdT → IdI<sI> → IdI<nI> → IdI<cI>` Chain

Concrete pipeline for a single id:

1. **`IdT<'s, 't>` (input from typing pass).**
2. **`InstantiatorI::translate_id`** (`instantiator.rs:735-747`) → produces `IdI<'s, 'i, sI>`. Uses `reintern::name` per-step to translate each step's `INameT` into `INameI<sI>`. Region mode at the boundary is always `sI` (`reintern.rs:10-11`).
3. **`region_collapser_consistent::collapse_id`** (when queuing a new instantiation) → `IdI<'s, 'i, nI>`. Uses the per-denizen renumbering map.
4. **`region_collapser_individual::collapse_id`** (at final output emission) → `IdI<'s, 'i, cI>`. Uses a per-node fresh map.

This four-stage chain is the architectural reason the interner has 12 HashMaps (3 modes × 4 logical types): each stage's output needs to be deduplicated against other outputs at the same stage.

### 5.6 `reintern` — The T → I Bridge

`reintern.rs` is the documented T→I memoization layer (`:1-13`). All 11 functions are currently `panic!` stubs awaiting body migration. Architectural choice: **free functions**, not methods on `InstantiatingInterner` (which stays "pure-`'i`-side" and shouldn't know about `'t`) or `InstantiatorI` (which is stateful and would entangle the bridge with translation state).

The reintern bridge does the type-level swap: `INameT<'s, 't> → INameI<'s, 'i, sI>`, `KindT<'s, 't> → KindIT<'s, 'i, sI>`, etc. — preserving the data but switching the arena+R. Region mode at the boundary is always `sI`.

---

## Part 6: The `translate_*` Method Family

`InstantiatorI` exposes **109 `translate_*` methods** (counted via grep). Most are `panic!` stubs awaiting body migration; the architectural shape is fully designed.

### 6.1 Typical Method Signature

```rust
pub fn translate_prototype(
    &self,
    monouts: &mut InstantiatedOutputsI<'s, 't, 'i>,
    denizen_name: &IdT<'s, 't>,
    denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>,
    substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>,
    perspective_region_t: &RegionT,
    prototype_t: &PrototypeT<'s, 't>,
) -> (PrototypeI<'s, 'i, sI>, PrototypeI<'s, 'i, cI>)
```

(`instantiator.rs:2159`.) Common parameter pattern:
- `&self` — re-entrant access to `self.hinputs`, `self.interner`, `self.typing_interner`.
- `monouts: &mut InstantiatedOutputsI` — accumulator.
- `denizen_name: &IdT` — which denizen we're inside (for bound-arg resolution).
- `denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI` — caller-supplied bound args; see Part 7.
- `substitutions: &IndexMap<IdT, ITemplataI<sI>>` — placeholder-id → resolved-templata map. Threaded explicitly.
- `perspective_region_t: &RegionT` — which region's perspective we're translating from (used for region collapse).
- `<thing>_t: &<Thing>T` — the T-side input.

Returns the I-side equivalent, often a `(sI, cI)` pair: `translate_prototype` returns `(PrototypeI<sI>, PrototypeI<cI>)`; `translate_ref_expr` returns `(CoordI<sI>, ReferenceExpressionIE<cI>)` (`:3336`).

### 6.2 Key Entry Points

- `translate_method` (`:255`) — the renamed Scala `translate`, top-level worklist driver.
- `translate_id` (`:735`) — generic id walker; takes a closure for the local_name conversion.
- `translate_name` (`:788`) — panic stub.
- **The four denizens dispatched from the worklist**: `translate_function` (`:1481`), `translate_impl` (`:1420`), `translate_abstract_func` (`:1605`), `translate_override` (`:1092`).
- **Id-family**: `translate_function_id` (`:5017`), `translate_struct_id` (`:5068`), `translate_interface_id` (`:5108`), `translate_impl_id` (`:5147`).
- **Prototype/header/collapsed-definition family**: `translate_prototype` (`:2159`), `translate_function_header` (`:2768`), `translate_collapsed_function` (`:2863`), `translate_collapsed_struct_definition` (`:2592`), `translate_collapsed_interface_definition` (`:2675`).
- **Expression family**: `translate_ref_expr` (`:3336`), `translate_addr_expr` (`:3090`), `translate_expr` (`:3299`).
- **Variable family**: `translate_local_variable` (`:2970`), `translate_reference_local_variable` (`:3008`), `translate_addressible_local_variable` (`:3050`).
- **Atom translation**: `translate_struct_member` (`:2044`), `translate_function_attribute` (`:2824`), `translate_citizen_attribute` (`:2845`), `translate_variability` (`:2142`), `translate_mutability` (`:2142`), `translate_ownership` (`:4836`).
- **Bound threading**: `translate_bound_args_for_callee` (`:2416`).

### 6.3 `translate_override` — The Heavyweight Example

`instantiator.rs:1092-1182`. 90 lines. The most architecturally interesting `translate_*` method because it does the v-table assembly for one (impl, abstract function) pair.

Walks the dispatcher's placeholder ids; builds case substitutions; threads `dispatcher_instantiation_bound_params_to_args` plus `extra_bounds` via `.plus()`; calls `translate_prototype` with the combined map; finally calls `monouts.add_method_to_v_table(impl_id_c, super_interface_id, abstract_func_prototype_c, override_prototype_c)`.

The CCFCTS transmute at `:1131` lives in this function: `unsafe std::mem::transmute(templata_c)` reuses a `cI` value as `sI` to avoid a roundtrip through region collapse for one specific case.

The `@TIBANFC` ("Translate Impl Bound Argument Names For Case", defined in `docs/Generics.md:1404`) reference applies here — case-bound naming is the conceptual model for what `translate_override` does.

### 6.4 Substitutions and Bounds Threading

Every body-bearing `translate_*` takes:
- `substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>` — placeholder-id → resolved-templata. Keys are typing-arena `IdT`s; values are `sI`-mode templatas.
- `denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>` — see Part 7.

These flow through every recursive call. When a `translate_*` needs to resolve a placeholder, it looks up `substitutions[&placeholder_id]`. When it needs a bound arg (e.g. for a virtual call), it walks `denizen_bound_to_denizen_caller_supplied_thing`.

---

## Part 7: Bounds — `InstantiationBoundArgumentsI` and `DenizenBoundToDenizenCallerBoundArgI`

### 7.1 `InstantiationBoundArgumentsI` — Per-Site Bound Storage

`ast/hinputs.rs:30-35`:

```rust
pub struct InstantiationBoundArgumentsI<'s, 'i>
where 's: 'i,
{
    pub rune_to_function_bound_arg:
        ArenaIndexMap<'i, IRuneS<'s>, &'i PrototypeI<'s, 'i, sI>>,
    pub caller_rune_to_callee_rune_to_reachable_func:
        ArenaIndexMap<'i, IRuneS<'s>,
            ArenaIndexMap<'i, IRuneS<'s>, &'i PrototypeI<'s, 'i, sI>>>,
    pub rune_to_impl_bound_arg:
        ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i, sI>>,
}
```

Not interned; `/// Temporary state`. Uses `ArenaIndexMap` for insertion-ordered iteration (deterministic output). **All bound args stored in `sI` mode** — they get collapsed only at final output emission.

Three maps:
- `rune_to_function_bound_arg` — for each function bound the call site needs, which concrete function satisfies it.
- `caller_rune_to_callee_rune_to_reachable_func` — for nested bounds (caller-side rune → callee-side rune → satisfying function). This nested-map shape is what's needed for `translate_override`'s `_bound_param_prototype_t_to_bound_arg_prototype_i_from_impl` builder (the closure JR-fill that needed the TL-recipe for the `&lw.some_constructor` lifetime fix in the weak-tests cluster).
- `rune_to_impl_bound_arg` — for each impl bound, the satisfying impl id.

### 7.2 `DenizenBoundToDenizenCallerBoundArgI` — Per-Call Bound Threading

`instantiator.rs:53-56`:

```rust
pub struct DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i> {
    pub func_id_to_bound_arg_prototype:
        IndexMap<IdT<'s, 't>, &'i PrototypeI<'s, 'i, sI>>,
    pub bound_param_impl_id_to_bound_arg_impl_id:
        IndexMap<IdT<'s, 't>, IdI<'s, 'i, sI>>,
}
```

Carries the bound args "supplied by the caller" — passed down to every `translate_*` method as `denizen_bound_to_denizen_caller_supplied_thing`.

**Merging via `.plus()`** (`:64-71`): `union_maps_expect_no_conflict` — combines two `DenizenBoundToDenizenCallerBoundArgI` values, panicking if any key collides. Used in `translate_override` to merge dispatcher-supplied bounds with case-specific extra bounds.

All bounds in this struct are in `sI` mode (the typed maps carry `&'i PrototypeI<sI>` and `IdI<sI>`).

### 7.3 Bound Storage in `InstantiatedOutputsI`

The `InstantiatedOutputsI` accumulator stores bounds via four fields:
- `struct_to_bounds: IndexMap<&'i StructIT<sI>, DenizenBoundToDenizenCallerBoundArgI>`
- `interface_to_bounds: IndexMap<&'i InterfaceIT<sI>, DenizenBoundToDenizenCallerBoundArgI>`
- `impl_to_bounds: IndexMap<&'i IdI<sI>, DenizenBoundToDenizenCallerBoundArgI>`
- `abstract_func_to_bounds: IndexMap<&'i PrototypeI<sI>, DenizenBoundToDenizenCallerBoundArgI>`

All keyed by `sI`-mode ids — bound resolution happens before region collapse.

---

## Part 8: I-Suffix AST Type Families

### 8.1 `CoordI` — Inline Copy

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CoordI<'s, 'i, R> where 's: 'i {
    pub ownership: OwnershipI,
    pub kind: KindIT<'s, 'i, R>,
}
```

`ast/types.rs:215-219`. Value-type, Copy, derives Eq/Hash, carries `R`.

**Differs from typing's `CoordT` and simplifying's `CoordH`:**
- vs `CoordT`: drops the `region: RegionT` field. Regions are encoded in `R` and in nested templatas.
- vs `CoordH`: lacks the `location: LocationH` field (computed in Hammer, not the instantiator).

### 8.2 `KindIT` — Inline Polyvalue Wrapper, Carries R

`ast/types.rs:258-269`. 10 variants. Primitives held inline; the 4 compound payloads (`StructIT`/`InterfaceIT`/`StaticSizedArrayIT`/`RuntimeSizedArrayIT`) by `&'i` ref. Sub-enum families: `ISubKindIT` (`:545`), `ICitizenIT` (`:559`). All carry `R`.

```rust
pub enum KindIT<'s, 'i, R> where 's: 'i {
    Never(NeverIT), Void(VoidIT), Int(IntIT), Bool(BoolIT), Str(StrIT), Float(FloatIT),
    Struct(&'i StructIT<'s, 'i, R>),
    Interface(&'i InterfaceIT<'s, 'i, R>),
    StaticSizedArray(&'i StaticSizedArrayIT<'s, 'i, R>),
    RuntimeSizedArray(&'i RuntimeSizedArrayIT<'s, 'i, R>),
}
```

Inline-owned (16 bytes), Copy, Polyvalue.

### 8.3 `IdI` — Allocated, Not Sealed (Yet)

```rust
pub struct IdI<'s, 'i, R> where 's: 'i {
    pub package_coord: &'s PackageCoordinate<'s>,
    pub init_steps: &'i [INameI<'s, 'i, R>],
    pub local_name: INameI<'s, 'i, R>,
}
```

`ast/names.rs:25-29`. **Currently no `_must_intern` seal** — `IdI` is allocated but not yet sealed, unlike its peers `IdT` (typing) and `IdH` (simplifying). Reason: `intern_id_*` methods are in the TODO list (`instantiating_interner.rs:685-687`); the seal happens once `IdI` shape stabilizes.

Carries `R`. Pattern-match callers on `local_name` to narrow per the typing pass's +T-erasure principle (typing-pass-design-v3.md §6.0).

### 8.4 `INameI` and `INameValI` — 72 Variants Each

`ast/names.rs:135-209` (INameI), `:214+` (INameValI). Polyvalue enums; both carry `R`. 72 variants in each — every Scala `INameI` subclass gets a Rust variant. The Val mirror is used as the interner key (Value-type, derives Hash/Eq).

The 72 concrete `*NameI` structs (`StructNameI`, `FunctionNameI`, `ImplNameI`, etc.) all live in `ast/names.rs` (2539 lines) and all carry `R`. Interned via the macro-generated `intern_name_*_{si,ni,ci}` wrappers (216 methods total — see §4.3).

### 8.5 `ITemplataI` — Inline Polyvalue Wrapper, Carries R

`ast/templata.rs:133-154`. **20 variants**:

`Coord`, `Kind`, `RuntimeSizedArrayTemplate`, `StaticSizedArrayTemplate`, `Function`, `StructDefinition`, `InterfaceDefinition`, `ImplDefinition`, `Ownership`, `Variability`, `Mutability`, `Location`, `Boolean`, `Integer`, `String`, `Prototype`, `Isa`, `CoordList`, **`Region`**, `ExternFunction`.

All variants are value-type (no `&'i` refs in the variant tags themselves) — payloads each carry `<'s, 'i, R>`.

**`Region` is an instantiating-specific addition** — typing-pass `ITemplataT` doesn't have it. Region templatas track the region values that flow through generic instantiation; they get collapsed away by region_collapser_individual at final emission.

### 8.6 `PrototypeI` and `SignatureI` — Interned, Sealed, Carry R

```rust
pub struct PrototypeI<'s, 'i, R> where 's: 'i {
    pub id: &'i IdI<'s, 'i, R>,
    pub params: &'i [CoordI<'s, 'i, R>],
    pub return_type: CoordI<'s, 'i, R>,
    pub _must_intern: MustIntern,
}
```

`ast/ast.rs:667`. Sibling `PrototypeIValI` mirror. Sealed with `_must_intern`. Interned via `intern_prototype_si/ni/ci`. `SignatureI` (`:376`) follows the same pattern.

### 8.7 Expression AST — `ExpressionIE` / `ReferenceExpressionIE` / `AddressExpressionIE`

`ast/expressions.rs`. Three-enum wrapper layered like typing:
- `ExpressionIE<'s, 'i, R>` (`:26-29`) — 2-variant dispatcher (`Reference` / `Address`).
- `ReferenceExpressionIE<'s, 'i, R>` (`:103-153`) — 49 variants, all `&'i Payload`.
- `AddressExpressionIE<'s, 'i, R>` (`:159-165`) — 5 variants.

**Derives: `Copy, Clone, Debug` only** (`:21-24` comment: Scala `vcurious`). Drops Eq/Hash uniformly per the typing-pass precedent (typing-pass-design-v3.md §7.3).

`.result()` method dispatcher on `ReferenceExpressionIE` (`:42-96`) — many branches still `panic!`.

All ~50 payload structs carry `<'s, 'i, R>`.

### 8.8 `HinputsI` — Final Output

`ast/hinputs.rs:47-58`. Top-level container produced by `translate`. All fields hard-code `cI` mode:

```rust
pub struct HinputsI<'s, 'i> where 's: 'i {
    pub interfaces: &'i [InterfaceDefinitionI<'s, 'i, cI>],
    pub structs: &'i [StructDefinitionI<'s, 'i, cI>],
    pub functions: &'i [FunctionDefinitionI<'s, 'i, cI>],
    pub static_sized_arrays: &'i [StaticSizedArrayDefinitionIT<'s, 'i, cI>],
    pub runtime_sized_arrays: &'i [RuntimeSizedArrayDefinitionIT<'s, 'i, cI>],
    // ... more cI-typed fields ...
}
```

`HinputsI` itself doesn't carry `R` — its R-collapse is fixed at `cI` at the type level. This is the contract with the simplifying pass: Hammer only ever sees `cI`-mode data.

---

## Part 9: Error Handling

**No `Result`, no `ICompileErrorI`.** The instantiator uses `panic!` exclusively for unimplemented branches and `vassert*`-style assertions for invariants (e.g. `vassert_one` at `instantiator.rs:1094`). `translate()` (`:214`) returns `HinputsI<'s, 'i>` directly, not a `Result`.

This matches Scala: Scala's typing pass throws `CompileErrorExceptionT` but the instantiating pass is expected not to fail — all user errors should have been caught in typing, all internal failures are bugs. The Rust port preserves this: any panic in the instantiator is a migration bug, not a recoverable user error.

(The simplifying pass agrees — see simplifying-pass-design.md §7. The testvm Result-bubble refactor produces `VmRuntimeErrorV<'s>` and lives entirely inside testvm; neither instantiating nor simplifying produces it.)

---

## Part 10: Driving the Pass

### 10.1 Top-Level Entry — `instantiating::translate`

Free function at `instantiator.rs:214-219`. Constructs `InstantiatedOutputsI::new()`, builds the `InstantiatorI` value, calls `translate_method`. No `Result`.

### 10.2 Worklist Driver

`translate_method` runs the worklist loop (§3.2) until all three queues (`new_functions`, `new_impls`, `new_abstract_funcs`) drain. Each translate-callout may push more work; eventually all queues empty. Then builds final `HinputsI` with `cI`-collapsed contents.

### 10.3 `InstantiatedCompilation` Coordination Wrapper

`instantiated_compilation.rs:71`. 5-lifetime struct (`<'s, 'ctx, 't, 'i, 'p>`) that owns:
- `TypingPassCompilation` (which transitively owns the typing arena and upstream passes).
- The `InstantiatingInterner` reference.
- A cached `HinputsI` (lazy compute-on-first-access).

Constructor: `new(...)` at `:104`. Consumer entry: `get_monouts()` returns `&HinputsI`.

### 10.4 Handoff to Simplifying

`InstantiatedCompilation`'s `HinputsI` is consumed by `HammerCompilation` (see simplifying-pass-design.md §8.2). The typing arena can drop between instantiating-done and simplifying-start because `HinputsI` no longer holds any `'t` refs (the reintern bridge translated everything across the T → I boundary).

**Note from current state**: `instantiated_compilation.rs:10-11` documents that the simplifying pass was unlinked during instantiating bring-up (Slabs 16a–16j). The current simplifying pass only reads I-suffix output enums from `ast/types.rs` directly (per `ast/mod.rs:5-8`); the full handoff wiring is restored later in body migration.

---

## Part 11: Invariants Summary

Invariants that aren't fully stated elsewhere in this doc:

1. **Region mode at the T → I boundary is always `sI`.** The reintern bridge (`reintern.rs:10-11`) produces only `sI`-mode I-side data. Anything else (`nI`/`cI`) comes from a region-collapse pass downstream.
2. **`cI` is reachable only via collapse**, never via direct construction at the T → I boundary. The collapse always passes through `region_counter` first to build the renumbering map.
3. **`InstantiatingInterner` does not carry `'t`** (`instantiating_interner.rs:14-16`) — by design, so the typing arena can drop after instantiation.
4. **`nI` and `cI` are type aliases for `sI`** (CCFCTS, `ast/types.rs:179-185`). Zero-member ZSTs. The `unsafe transmute` at `instantiator.rs:1131` relies on this invariant.
5. **All bounds stored in `sI` mode.** Both `InstantiationBoundArgumentsI` and `DenizenBoundToDenizenCallerBoundArgI` carry `sI`-typed prototype/id refs. Bound resolution happens before region collapse.
6. **Worklist contains queued work in `nI` mode.** Per-denizen region renumbering canonicalizes the queue keys.
7. **Final output (`HinputsI`) is all `cI`.** Hammer (simplifying) only ever sees `cI`-mode data. The simplifying pass has no region-mode concept at all.
8. **Never use `'static`** (NUSLX). All data lives in `'p`, `'s`, `'t`, `'i` (or on the stack).
9. **`MustIntern` seal preserves canonical pointer identity** on `&'i StructIT`, `&'i InterfaceIT`, `&'i StaticSizedArrayIT`, `&'i RuntimeSizedArrayIT`, `&'i PrototypeI`, `&'i SignatureI`, and the ~72 concrete `*NameI` types. `IdI` is the unsealed exception (pending Slab 17+).

---

## Part 12: Risks / Open Questions

- **`IdI` not yet sealed.** Pending Slab 17 body migration. Until then, `IdI`-keyed maps fall back to structural comparison rather than `ptr::eq` — slightly slower but correct.
- **`reintern.rs` is panic-stubbed.** The T → I bridge has 11 functions all currently `panic!()`. Body migration concern. Without it, `translate_method` can't actually run end-to-end; the architectural design is laid down but not yet exercised.
- **`unsafe transmute` at `instantiator.rs:1131`** is a load-bearing usage of the CCFCTS aliasing. If anyone adds a member to `sI`/`nI`/`cI` it becomes UB. Sealed; documented; relies on internal discipline.
- **216 macro-generated name intern wrappers**: hand-counted from `impl_intern_name_wrappers!` invocations. If new name types are added without macro coverage, the intern surface will be inconsistent. Worth a code-gen audit pre-Slab-completion.
- **Dual region-collapser modules** (`region_collapser_individual.rs`, `region_collapser_consistent.rs`) implement parallel ~30-function surfaces — adding a new I-side type requires updating both. The parallelism is enforced only by convention. A code-gen-via-macro pass over both files might be warranted later.
- **12-HashMap interner footprint**: 3 modes × 4 logical types = 12 dedup maps. Each map's hot path is checked on every intern. Profile before optimizing; the design's correctness pays for the multiplication.
- **109 `translate_*` methods**, mostly `panic!` stubs. The architectural shape (parameter list, return type, dispatcher organization) is uniform; body migration is the hard part. Test-driven JR body fills (per migrate-tl.md) is the current driver.
- **`InstantiatedCompilation` 5-lifetime soup**: structural consequence of cross-pass orchestration. Working but novel.
- **No code-gen / `paste`-crate usage**: the macros at `instantiating_interner.rs:103-126` are hand-written declarative macros. If the name set grows substantially, switching to `paste`-generated code (as typing pass does in some places) might be cleaner.
- **Acronym docs gap**: `@PASDZ` (Placeholder Authoritative-Source = Denizen, cited at `instantiator.rs:1275,1308,1327,1736,5378,5635,5940`), `@CCFCTS` (cited but the canonical-definition link is the source comment itself), `@HRALII` (ownership-splitting at `types.rs:41,48,68,75`), `@ICRHRC` (region-counting timing at `region_collapser_*.rs`) — none have dedicated `docs/` files. Opportunity for a `docs/architecture/instantiator-acronyms.md` writeup.
- **TIBANFC, NMORFI, SCCTT** are typing-pass-defined acronyms (`docs/Generics.md:1404`, `docs/virtuals/Impls.md:3,182,186,192`) that the instantiator inherits via the typing-pass data it consumes. The instantiator implements them but doesn't define them.

---

## Part 13: Key Files / Directories

| Path | Purpose |
|---|---|
| `docs/architecture/instantiator_design.md` | this doc |
| `docs/architecture/typing-pass-design-v3.md` | upstream pass design |
| `docs/architecture/simplifying_pass_design.md` | downstream pass design |
| `FrontendRust/docs/migration/migration-policy.md` | per-pass policy values; instantiating rows at lines 19, 25, 37, 46, 49 |
| `docs/Generics.md` | TIBANFC definition (line 1404) |
| `docs/virtuals/Impls.md` | NMORFI / SCCTT definitions (lines 3, 182, 186, 192) |
| `FrontendRust/src/instantiating/mod.rs` | Module exports |
| `FrontendRust/src/instantiating/instantiating_arena.rs` | `InstantiatingArena<'i>` newtype |
| `FrontendRust/src/instantiating/instantiating_interner.rs` | 12-HashMap interner, ~216 macro-generated wrapper methods |
| `FrontendRust/src/instantiating/instantiator.rs` | 6538 lines: `InstantiatorI` god struct, `InstantiatedOutputsI`, 109 `translate_*` methods |
| `FrontendRust/src/instantiating/reintern.rs` | T → I memoization bridge (panic stubs pending Slab 17) |
| `FrontendRust/src/instantiating/region_counter.rs` | 970 lines: walk AST, build region renumbering maps |
| `FrontendRust/src/instantiating/region_collapser_individual.rs` | 824 lines: sI → cI per-node collapse |
| `FrontendRust/src/instantiating/region_collapser_consistent.rs` | 830 lines: sI → nI per-denizen collapse |
| `FrontendRust/src/instantiating/collector.rs` | Visitor pattern over I-side AST |
| `FrontendRust/src/instantiating/instantiated_compilation.rs` | Driver wrapper, 5-lifetime, lazy-cached HinputsI |
| `FrontendRust/src/instantiating/instantiated_humanizer.rs` | Debug/print |
| `FrontendRust/src/instantiating/ast/types.rs` | `CoordI`, `KindIT`, `IRegionsModeI` + `sI`/`nI`/`cI` markers |
| `FrontendRust/src/instantiating/ast/names.rs` | 2539 lines: `IdI`, `INameI` (72 variants), all `*NameI` structs |
| `FrontendRust/src/instantiating/ast/ast.rs` | `PrototypeI`, `SignatureI`, `FunctionDefinitionI`, exports/externs |
| `FrontendRust/src/instantiating/ast/templata.rs` | `ITemplataI` + 20 payload structs |
| `FrontendRust/src/instantiating/ast/citizens.rs` | `StructDefinitionI`, `InterfaceDefinitionI`, `ICitizenDefinitionI` |
| `FrontendRust/src/instantiating/ast/expressions.rs` | `ExpressionIE`/`ReferenceExpressionIE`/`AddressExpressionIE` + ~50 payloads |
| `FrontendRust/src/instantiating/ast/hinputs.rs` | `HinputsI`, `InstantiationBoundArgumentsI` |
| `Frontend/InstantiatingPass/src/dev/vale/instantiating/Instantiator.scala` | Scala source — line-for-line port target |
