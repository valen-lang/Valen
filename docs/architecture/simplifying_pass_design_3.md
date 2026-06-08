# Simplifying + final_ast Pass Design

Architecture and design decisions for the Scala-to-Rust simplifying-pass migration. Companion to `docs/architecture/typing-pass-design-v3.md` and `docs/architecture/instantiator_design.md`. Operational handoff in `migrate-tl.md` at the repo root.

This pass takes fully-instantiated I-side AST (`HinputsI<'s, 'i>`) and lowers it to the final wire-format AST (`ProgramH<'s, 'h>`) consumed by the backend. It is the last pass before VON emission and the first that operates without any user-error-channel — by the time data reaches Hammer, all errors have been caught upstream.

For migration policy values (Val/Ref pairs, lifetime conventions, sealed-trait policy), see `FrontendRust/docs/migration/migration-policy.md` simplifying + final_ast rows.

---

## Part 1: Arena and Lifetime Model

The simplifying pass introduces a single new arena lifetime (`'h`) on top of the lifetimes inherited from upstream (`'p`, `'s`, `'i`). The pass reads I-side input through `&'i` refs and emits H-side output into `'h`. No re-interning of scout-arena data (`StrI<'s>`, `PackageCoordinate<'s>`); strings flow through unchanged.

### 1.1 Four Arenas

- **`'p` — Parser arena (`ParseArena<'p>`)**: parser AST, parser-side `StrI<'p>` literals. Read-only at this point; instantiating already finished using it.
- **`'s` — Scout arena (`ScoutArena<'s>`)**: interned scout names (`INameS<'s>`, `IRuneS<'s>`, `IImpreciseNameS<'s>`), source strings (`StrI<'s>`), code locations (`PackageCoordinate<'s>`, `RangeS<'s>`). Hammer reads scout-arena data freely via `&'s` fields.
- **`'i` — Instantiating arena (`InstantiatingInterner<'s, 'i>`)**: holds the input `HinputsI<'s, 'i>` and every I-suffix type it transitively reaches. Read-only here.
- **`'h` — Hammer arena (`HammerInterner<'s, 'h>`)**: simplifying-pass output. Holds all H-suffix interned types (`StructHT`, `InterfaceHT`, `PrototypeH`, `IdH`, `*ArrayHT`), the allocated-but-not-interned AST (`ExpressionH` payloads, `FunctionH`, `StructDefinitionH`, `PackageH`, `ProgramH`), and the cross-reference `ArenaIndexMap`s inside `PackageH`. Created before the simplifying pass starts; outlives the pass.

All four arenas use `bumpalo`. `HammerArena<'h>` (`hammer_arena.rs:10-20`) is a thin newtype around `&'h Bump`; `HammerInterner<'s, 'h>` (`hammer_interner.rs:36-50`) wraps the bump plus a `RefCell<Inner>` holding three `StdHashMap`s, one per intern family.

Arena-allocated structs follow AASSNCMCX (no `Vec`/`HashMap`/`String` inside arena types) — the `PackageH` cross-reference tables use `ArenaIndexMap` instead of `HashMap` for that reason.

### 1.2 Lifetime Invariants

1. **`'s: 'i`** — interner-level invariant from instantiating; carried into Hammer because every H type that holds I-side refs needs `&'i I<…, R>` and those I types are `<'s, 'i, R>`.
2. **`'i: 'h`** — H-side output references I-side input directly (e.g. `Hamuts.struct_t_to_struct_h: HashMap<&'i StructIT<'s,'i,cI>, &'h StructHT<'s,'h>>`); the I-arena must outlive the H-arena.
3. **`'s: 'h`** — composing 1 and 2; explicit in the god struct's where-clause (`hammer.rs:453`: `where 's: 'i, 's: 'h, 'i: 'h`).
4. **Region-mode resolved before Hammer.** Hammer reads only `cI`-mode I-side input. The instantiator's worklist drives all `sI → nI → cI` collapses; nothing in `'sI` survives across the I → H boundary.
5. **Arena borrow convention.** Hammer parameters take a short borrow lifetime (elided or named `'ctx`), never the arena's own lifetime. Write `&HammerInterner<'s, 'h>` or `&'ctx HammerInterner<'s, 'h>`, never `&'h HammerInterner<'s, 'h>`. `HammerInterner::alloc(&self, T) -> &'h T` returns arena-lifetimed data from a short `&self` borrow per the same pattern as `TypingInterner::alloc`.
6. **Only one arena lifetime added beyond instantiating.** No per-package or per-function sub-arena. Single `'h` for the whole simplifying pass.

### 1.3 Arena Construction Order

```
fn top_level_driver() {
    let parse_arena = ParseArena::new();                   // 'p
    let scout_arena = ScoutArena::new();                   // 's
    let typing_interner = TypingInterner::new(...);        // 't (drops after instantiating)
    let instantiating_interner = InstantiatingInterner::new(...);  // 'i
    let hamuts_i = instantiating::translate(...);          // produces HinputsI<'s, 'i>

    let hammer_bump = Bump::new();
    let hammer_interner = HammerInterner::new(&hammer_bump);  // 'h
    let program_h = hammer.translate(&hamuts_i);           // returns &'h ProgramH<'s, 'h>

    // ... VON emission, backend handoff ...

    // hammer_interner / hammer_bump drop: 'h dies
    // instantiating_interner drops: 'i dies
    // scout_arena drops: 's dies
    // parse_arena drops: 'p dies
}
```

Rust's drop order (reverse of declaration) enforces `'h < 'i < 's < 'p` lifetime nesting. The typing arena (`'t`) is conventionally dropped between `'t` instantiating-finished and `'i` build because instantiating fully consumes typing-pass data into I-side equivalents.

### 1.4 Where Each Type Lives

| Type | Lifetimes | Location |
|---|---|---|
| `ProgramH`, `PackageH` | `<'s, 'h>` | `'h`, allocated, holds `&'h [...]` slices and `&'h ArenaIndexMap`s |
| `FunctionH`, `StructDefinitionH`, `InterfaceDefinitionH`, `EdgeH` | `<'s, 'h>` | `'h`, allocated, not interned |
| `IdH` | `<'s, 'h>` | `'h`, interned, `MustIntern` seal |
| `PrototypeH` | `<'s, 'h>` | `'h`, interned, `MustIntern` seal |
| `StructHT`, `InterfaceHT`, `StaticSizedArrayHT`, `RuntimeSizedArrayHT` | `<'s, 'h>` | `'h`, interned, `MustIntern` seal |
| `KindHT` | `<'s, 'h>` | inline Copy Polyvalue wrapper, 16 bytes |
| `ExpressionH`, `ReferenceExpressionH`, `AddressExpressionH`, `IExpressionH` | `<'s, 'h>` | inline Copy wrapper (16 bytes), variant payload is `&'h XH` |
| `CoordH` | none | inline Copy Value-type (~6 bytes) |
| `OwnershipH`, `LocationH`, `Mutability`, `Variability` | none | inline Copy unit-variant enums |
| ~50 expression payload structs (`StackifyH`, `CallH`, `IfH`, …) | `<'s, 'h>` | `'h`, allocated, not interned |
| `Hamuts` | `<'s, 'i, 'h>` | stack-owned; heap-backed `HashMap`s; dies at pass end |
| `Hammer` (god struct) | `<'s, 'i, 'h, 'ctx>` | stack; dies at pass end |
| `Locals` | `<'s, 'h>` | stack; collapsed Scala `LocalsBox` |
| `HammerCompilation` | 6 lifetimes | stack driver; owns `InstantiatedCompilation` + the H-arena |

### 1.5 Type Inventory By Arena

Bucket-level summary. Per-type detail lives in §5 (H-types) and §3 (Hamuts).

- **`'i` instantiating arena (read-only inputs):** `HinputsI<'s, 'i>` plus everything it transitively reaches — `StructIT<'s,'i,cI>`, `InterfaceIT`, `FunctionDefinitionI`, `PrototypeI`, `IdI`, `ITemplataI`, and the I-side expression AST. The simplifying pass never mutates this data.
- **`'h` hammer arena, interned:** `IdH`, `PrototypeH`, plus the 4 sealed kind payloads (`StructHT`, `InterfaceHT`, `StaticSizedArrayHT`, `RuntimeSizedArrayHT`). Each interned type carries `_must_intern: MustIntern` and has a transient `*ValH` mirror (`StructHTValH`, etc.) used as the HashMap query key.
- **`'h` hammer arena, allocated but not interned:** `ProgramH`, `PackageH`, definitions (`FunctionH`, `StructDefinitionH`, `InterfaceDefinitionH`, `EdgeH`, `StructMemberH`, `InterfaceMethodH`, `FunctionRefH`), the expression hierarchy (~50 `ExpressionH` payload structs + the three wrapper enums + payload-bearing helpers like `Local` and `VariableIdH`), `StaticSizedArrayDefinitionHT`, `RuntimeSizedArrayDefinitionHT`, `HamutsFunctionExtern`, `HamutsKindExtern`.
- **Polyvalue inline wrappers (16 bytes, not arena-stored)** — closed-set fat pointers, see @TFITCX / @PVECFPZ: `KindHT` (11 variants), `ExpressionH` (50 variants), `IExpressionH` (2 variants — `ReferenceExpressionH`/`AddressExpressionH`).
- **Value-type primitives (Copy, no interning):** `CoordH`, `OwnershipH`, `LocationH`, `Mutability`, `Variability`, `IntHT`, `VoidHT`, `BoolHT`, `StrHT`, `FloatHT`, `NeverHT`, `OpaqueHT`, `CodeLocation`.
- **Neither arena (stack/heap):** `Hamuts` (stack accumulator), `Hammer` (stack god struct), `Locals` (stack), `HammerCompilation` (stack driver), `Inner` of HammerInterner (heap HashMaps).

**Casting identity rule.** Polyvalue enums compare structurally on their 16-byte (tag + ref/value) layout — derive `PartialEq`/`Eq`/`Hash`, which delegates to each variant's inner identity. Interned canonical refs compare via `ptr::eq` on the `&'h T` (the `MustIntern` seal guarantees canonical pointer). Casting up the sub-enum hierarchy (e.g. concrete `&'h StructHT` → `KindHT::StructHT(...)`) is a stack-only rewrap; no interner involvement.

**Equality opt-out for the expression hierarchy** (`ExpressionH`, `ReferenceExpressionH`, `AddressExpressionH`, ~50 per-variant payload structs, plus `FunctionH` and `PackageH` which embed expressions transitively) — see @IEOIBZ. Driving reason: `ConstantF64H` holds `f64`, which can't derive `Eq`/`Hash`; the whole expression-AST family drops them uniformly. `CoordH`/`KindHT`/interned payloads keep the full `Copy, Clone, PartialEq, Eq, Hash, Debug`.

### 1.6 Mutual-Recursion Shape

The H-type graph is mutually recursive but every edge is an `&'s`, `&'i`, or `&'h` ref — pointer-sized, finite. `CoordH → KindHT → StructHT → IdH → SimpleIdStep`. `ExpressionH → CallH → PrototypeH → IdH`. Self-recursion via `ExpressionH` variants (`BlockH.inner: ExpressionH`) is resolved by inline 16-byte sub-enum values — no `Box`.

The `PackageH` cross-reference tables (`ArenaIndexMap` keyed on `&'h StructHT`, `&'h InterfaceHT`, etc.) close the graph back up: e.g. `&'h StructHT` resolves to its `&'h StructDefinitionH` via `PackageH.struct_id_to_struct_def`.

---

## Part 2: The God Struct (`Hammer`)

### 2.1 Architecture

All Scala sub-hammers (`ExpressionHammer`, `BlockHammer`, `LetHammer`, `LoadHammer`, `MutateHammer`, `StructHammer`, `TypeHammer`, `FunctionHammer`, `NameHammer`, `VonHammer`) collapse into a single `Hammer` struct:

```rust
pub struct Hammer<'s, 'i, 'h, 'ctx>
where 's: 'i, 's: 'h, 'i: 'h,
{
    pub interner: &'ctx HammerInterner<'s, 'h>,
    pub keywords: &'ctx Keywords<'s>,
    pub scout_arena: &'ctx ScoutArena<'s>,
    pub instantiating_interner: &'ctx InstantiatingInterner<'s, 'i>,
}
```

Immutable configuration only (`hammer.rs:452-459`). Mutable state threads through `&mut Hamuts<'s, 'i, 'h>` on every call. The collapse mirrors the typing-pass `Compiler` precedent — see typing-pass-design-v3.md §2.1.

**File-level organization.** Each per-area file (`expression_hammer.rs`, `block_hammer.rs`, `let_hammer.rs`, `load_hammer.rs`, `mutate_hammer.rs`, `struct_hammer.rs`, `type_hammer.rs`, `function_hammer.rs`, `name_hammer.rs`, `von_hammer.rs`) contains `impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx> where … { … }` blocks. No state lives in any of these files — they exist purely for code organization, the same way typing-pass body files are organized by topic.

**Not on the god struct:**
- **No Hamuts field** — `Hamuts` is threaded as `&mut hamuts: &mut Hamuts` per call (matches Scala's `mutable.Map` field on `HamutsBox` which the Rust port collapses; see §3).
- **No VonHammer field** — VonHammer's methods are `impl Hammer` blocks in `von_hammer.rs`; the Scala `VonHammer` class collapses entirely (see §6).
- **No Locals field** — `Locals` is constructed per-function inside `translate_function` and threaded as `&mut Locals` to expression-level methods.

### 2.2 `&self` + `&mut hamuts` Enables Re-entrancy

Deep mutual recursion (`translate_expression → translate_call → translate_function → translate_expression`) works because `&self` allows shared borrow, and `&mut hamuts` is re-borrowed at each call level. Same pattern as the typing pass — see typing-pass-design-v3.md §2.2.

### 2.3 VonHammer Collapsed Onto Hammer

Scala's `VonHammer` was a stateful sub-compiler (holding `keywords`, `interner`, package coords) emitting VON wire-format from the H-side AST. In the god-struct collapse, all that state is already on `Hammer`. The Rust port (`von_hammer.rs:1-4`) makes the collapse explicit:

> Per typing-pass `Compiler` precedent, `VonHammer` is not a Rust struct. Methods become `impl Hammer { ... }` blocks colocated here.

`HammerCompilation.get_von_hammer()` (`hammer_compilation.rs:169-178`) — Scala's `vassertSome(vonHammerCache)` lookup — now constructs a fresh `Hammer` value with the appropriate field borrows. The Scala cache field is deleted (`hammer_compilation.rs:94-98`).

### 2.4 HamutsBox and LocalsBox Collapsed

Scala had wrapper classes around mutable state:
- `HamutsBox` (mutable wrapper) + `Hamuts` (immutable value)
- `LocalsBox` (mutable wrapper) + `Locals` (immutable value)

Both collapse in Rust because Rust expresses mutation directly via `&mut`. The architect-directive collapse is documented at `hamuts.rs:1-12` and `hammer.rs:52-63`:

> No HamutsBox/Hamuts split — Scala's `HamutsBox` (the mutable wrapper) and `Hamuts` (the immutable value) collapse into one `Hamuts` struct.

Mutation is via `&mut Hamuts` / `&mut Locals` parameters. Snapshot semantics preserved via `Locals::snapshot(&self) -> Locals` clone (`hammer.rs:69-77`) where Scala's `LocalsBox.snapshot()` would have done it — this mirrors the typing-pass `NodeEnvironmentBox.snapshot` pattern. Same SPDMX exception U / Q justification as the typing-pass collapse.

### 2.5 Method vs Free Function Under The Collapse

When porting a Scala `def`, decide method-on-`Hammer` vs free function by inspecting two things: what the **Scala body uses**, and whether the Rust port has to be **stored in a closure**.

- **Scala body uses no `this`** → may become a Rust free function. Mirror the function name and file location; only the receiver drops. SPDMX exception Q codifies this.
- **Scala body uses `this`** → must stay a method on `Hammer`, even if the Scala class itself disappears under §2.1.

Concrete example: `name_hammer.rs:182, 211, 252, 270, 296` carry free functions `simplify_id` / `simplify_name` / `simplify_templata` / `simplify_kind` / `simplify_coord`. These were Scala's `object NameHammer` static methods — pure on their arguments, no `this` reads — so they translate as free fns (see `name_hammer.rs:1-5`). Body-bearing methods like `translate_full_name` stay on `impl Hammer` because they need `self.interner` / `self.keywords`.

---

## Part 3: Hamuts — Cross-Pass Shared State

### 3.1 Structure

Mutable accumulator threaded through the pass. Carries `<'s, 'i, 'h>`. 15 fields, all heap `HashMap` or `Vec`, lives on the stack (`hamuts.rs:437-455`):

- **Human-readable name index.** `human_name_to_full_name_to_id: HashMap<String, HashMap<String, i32>>` — the Scala `humanNameToFullNameToId` registry used for shortened-name disambiguation.
- **Kind translation tables** (cross-arena keys — `&'i KeyI → &'h ValH`): `struct_t_to_opaque_h`, `struct_t_to_struct_h`, `struct_t_to_struct_def_h`, `static_sized_arrays`, `runtime_sized_arrays`, `interface_t_to_interface_h`, `interface_t_to_interface_def_h`.
- **Function translation tables**: `function_refs: HashMap<&'i PrototypeI, FunctionRefH>`, `function_defs: HashMap<&'i PrototypeI, FunctionH>`.
- **Definition collectors**: `struct_defs: Vec<StructDefinitionH>` (the in-emission-order list eventually allocated into `PackageH.structs`).
- **Package-grouped exports / externs (nested HashMaps)**: `package_coord_to_export_name_to_function`, `package_coord_to_export_name_to_kind`, `package_coord_to_prototype_to_extern`, `package_coord_to_kind_to_extern`.

No origin side tables — H-side data references I-side data directly via `&'i` refs.

### 3.2 Cross-Arena HashMap Pattern

The translation tables use `HashMap<&'i KeyI, &'h ValH>` — keys borrow from the input arena, values from the output arena. This is structurally the same as the typing pass's `HashMap<PtrKey<'t, IdT>, &'t T>` pattern but the key side carries `&'i` instead of being wrapped in `PtrKey`. Reason: I-side interned types (`StructIT`, `PrototypeI`) already implement pointer-based `PartialEq`/`Hash` via their `MustIntern` seal — no `PtrKey` newtype needed.

By-value values (`StructDefinitionH` in `struct_t_to_struct_def_h`) work because `StructDefinitionH` is small Copy data; the `Vec<StructDefinitionH>` collector keeps the actual storage and the map indexes into it.

### 3.3 Side-Table Access Pattern — Copy Out Before &mut

Same pattern as typing-pass-design-v3.md §4.3. Lookups return `Copy` pointer-sized values (`&'i K`, `&'h V`), enabling copy-out-then-mutate without aliasing issues:

```rust
// Copy out first:
let interface_h: &'h InterfaceHT<'s, 'h> = *hamuts.interface_t_to_interface_h
    .get(&interface_i)
    .expect("vassertSome");
self.translate_method(hamuts, ..., interface_h);  // OK; interface_h is Copy'd out
```

### 3.4 Mutator API

15 mutator methods (`hamuts.rs:193-396`): `forward_declare_struct`, `add_struct_originating_from_typing_pass`, `add_struct_originating_from_hammer`, `add_opaque`, `add_function`, `add_kind_export`, `add_function_export`, `add_function_extern`, `add_kind_extern`, plus the corresponding interface methods.

Every mutator asserts the Scala `vassert` preconditions (e.g. `hamuts.rs:207, 295, 308`). Re-insertion of an already-registered entry panics — the simplifying pass treats double-registration as a bug, not a recoverable condition.

### 3.5 Hamuts Constructed Inline

`Hamuts` is constructed inline in `Hammer::translate` at `hammer.rs:625-641` (all 15 fields seeded to empty `HashMap::new()` / `Vec::new()`). No `Hamuts::new()` constructor — the inline construction is one-shot per simplifying-pass run.

---

## Part 4: Interning Surface — `HammerInterner`

### 4.1 Structure

`HammerInterner<'s, 'h>` (`hammer_interner.rs:36-50`) wraps a `&'h Bump` plus `RefCell<Inner>` holding **three `StdHashMap`s**, one per intern family. Three families, not 12 like the instantiator — Hammer's interning surface is much narrower because the H-side has no region-mode multiplication (everything is already collapsed to `cI` by the time it arrives).

### 4.2 Intern Families

**Family-dispatcher pattern (kind payloads):**
- `intern_kind_payload(InternedKindPayloadValH<'s,'h>) -> InternedKindPayloadH<'s,'h>` (`hammer_interner.rs:90`) — dispatches the 4 sealed kind variants.
- Variant convenience wrappers (`hammer_interner.rs:125-148`):
  - `intern_struct_ht(StructHTValH) -> &'h StructHT`
  - `intern_interface_ht(InterfaceHTValH) -> &'h InterfaceHT`
  - `intern_static_sized_array_ht(StaticSizedArrayHTValH) -> &'h StaticSizedArrayHT`
  - `intern_runtime_sized_array_ht(RuntimeSizedArrayHTValH) -> &'h RuntimeSizedArrayHT`

**Direct Val-to-Ref pattern:**
- `intern_prototype(PrototypeHValH) -> &'h PrototypeH` (`:154`)
- `intern_id_h(IdHValH) -> &'h IdH` (`:173`)

### 4.3 `MustIntern` Seal

Per the IDEPFL pattern. `pub struct MustIntern(())` at `hammer_interner.rs:22-23` — private inner unit means only `hammer_interner.rs` itself can construct one. Every interned struct definition has `pub _must_intern: MustIntern` as a private-module-only field (e.g. `StructHT { id, _must_intern: MustIntern }` at `types.rs:357-360`; `IdH` at `ast.rs:585-592`).

The seal guarantees canonical pointer identity: two `&'h StructHT` with structurally-equal contents always have the same pointer because the only way to construct one is through `intern_*_ht`, which deduplicates via the `HashMap`. This enables the `ptr::eq`-based `PartialEq` and `Hash` on `&'h StructHT` to be sound; see @SICZ for the full argument.

### 4.4 Val/Ref Pair Pattern

Each interned type has a sibling `*ValH` mirror used as the `HashMap` query key. The Val type is Value-type (derives Eq/Hash structurally), the canonical Ref is Interning-permanent (custom Hash/Eq via `ptr::eq` on the seal-witnessed sealed pointer). Pattern:

```rust
pub fn intern_struct_ht(&self, val: StructHTValH<'s, 'h>) -> &'h StructHT<'s, 'h> {
    let mut inner = self.inner.borrow_mut();
    if let Some(canonical) = inner.struct_ht_val_to_ref.get(&val) {
        return canonical;
    }
    let canonical = self.bump.alloc(StructHT { id: val.id, _must_intern: MustIntern(()) });
    inner.struct_ht_val_to_ref.insert(val, canonical);
    canonical
}
```

(Schematic; real code at `hammer_interner.rs:125-148`.)

### 4.5 Arena Utilities

`hammer_interner.rs:67-84` exposes raw arena allocation alongside interning:
- `bump() -> &'h Bump`
- `alloc<T>(T) -> &'h T`
- `alloc_slice_copy<T: Copy>(&[T]) -> &'h [T]`
- `alloc_slice_from_vec<T>(Vec<T>) -> &'h [T]`
- `alloc_index_map<K, V>() -> ArenaIndexMap<'h, K, V>`
- `alloc_index_map_from_iter<K, V, I>(I) -> ArenaIndexMap<'h, K, V>`

Used for emitting non-interned data: expression payloads, definitions, the `PackageH` cross-reference `ArenaIndexMap`s.

### 4.6 Future Intern Families

Currently 5 wired (`hammer_interner.rs:11-13`). More may be deferred until body migration restores their fields — `IdH` for example only got its real fields late and its dedicated interner test module is gated off (`hammer_interner.rs:200`).

---

## Part 5: Final-AST Type Families

The final AST (`src/final_ast/`) is ~3766 LOC across three files. Every lifetime-parameterized struct has an implicit `where 's: 'h` bound.

### 5.1 The Three Files

- **`instructions.rs` (2374 lines):** `ExpressionH` + 50 payload structs + `IExpressionH`/`ReferenceExpressionH`/`AddressExpressionH` + `Local`/`VariableIdH`.
- **`ast.rs` (664 lines):** `ProgramH`, `PackageH`, `FunctionH`, `StructDefinitionH`, `InterfaceDefinitionH`, `EdgeH`, `PrototypeH`, `IdH`, `FunctionRefH`, plus their `*ValH` mirrors.
- **`types.rs` (713 lines):** `CoordH`, `KindHT` + sub-enum families, `OwnershipH`, `LocationH`, `Mutability`, `Variability`, the 4 sealed kind payloads (`StructHT`, `InterfaceHT`, `StaticSizedArrayHT`, `RuntimeSizedArrayHT`) + their `*ValH` mirrors, `SimpleId`, `SimpleIdStep`.

### 5.2 `CoordH` — Inline Copy

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CoordH<'s, 'h> where 's: 'h {
    pub ownership: OwnershipH,
    pub location: LocationH,
    pub kind: KindHT<'s, 'h>,
}
```

Differs from typing's `CoordT` and instantiating's `CoordI`:
- vs `CoordT`: drops the `region: RegionT` field — regions are resolved away by the time we reach H.
- vs `CoordI`: adds `location: LocationH` — Hammer's job includes computing `InlineH`/`YonderH` (the H-only data-layout distinction the backend needs); see @HRALII for the related ownership-splitting work that produces this.

Small (~6 bytes), Copy, passed by value. Not interned — structural eq.

### 5.3 `KindHT` — Inline Wrapper, Polyvalue

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum KindHT<'s, 'h> where 's: 'h {
    // Primitives inline (small Copy)
    Int(IntHT), Void(VoidHT), Bool(BoolHT), Str(StrHT),
    Float(FloatHT), Never(NeverHT), Opaque(OpaqueHT),
    // Non-primitives — &'h refs to interned payloads
    Struct(&'h StructHT<'s, 'h>),
    Interface(&'h InterfaceHT<'s, 'h>),
    StaticSizedArray(&'h StaticSizedArrayHT<'s, 'h>),
    RuntimeSizedArray(&'h RuntimeSizedArrayHT<'s, 'h>),
}
```

**KindHT is inline-owned, not arena-interned**: 16 bytes tag + ref, Copy. Concrete payloads (`StructHT` etc.) are arena-interned; the wrapper just tags them. Same philosophy as typing's `KindT` (typing-pass-design-v3.md §6.5).

`KindHT::OverloadSet` does **not** exist in the H-side AST — overload sets are resolved before Hammer; the I-side `OverloadSetIE`-as-`ReinterpretH`-arg-elision happens in the instantiator (see instantiator_design.md §5.4).

### 5.4 `IdH` — Interned, Sealed

```rust
#[derive(Copy, Clone, Debug)]
pub struct IdH<'s, 'h> where 's: 'h {
    pub local_name: StrI<'s>,
    pub package_coordinate: PackageCoordinate<'s>,
    pub shortened_name: StrI<'s>,
    pub fully_qualified_name: StrI<'s>,
    pub _must_intern: MustIntern,
    pub _phantom_h: PhantomData<&'h ()>,
}
```

Custom Display impl at `ast.rs:597-601`. Per the typing-pass `IdT` precedent (typing-pass-design-v3.md §6.3), `IdH` defines custom `PartialEq`/`Eq`/`Hash` (not derive): ptr-eq on the sealed `&'h IdH` ref. Soundness: the `_must_intern` seal forces construction through `intern_id_h`, which deduplicates structurally so equal content ⇒ equal pointer.

**`IdH` is structurally different from `IdT`/`IdI`.** Typing's `IdT` and instantiating's `IdI` carry `init_steps: &[INameT/INameI]` (multi-step hierarchical name path); H's `IdH` collapses to four pre-computed `StrI` fields (`local_name`, `package_coordinate`, `shortened_name`, `fully_qualified_name`). This is the H-side's job: produce a flat string-keyed name the backend can consume directly without walking a step list.

The `_phantom_h: PhantomData<&'h ()>` keeps `'h` live in the type even though no field actually borrows from `'h` — required so future fields holding `&'h` refs can be added without lifetime changes. Same trick as the typing pass's pre-Slab-13 `IdT`.

### 5.5 `PrototypeH` — Interned, Sealed

```rust
#[derive(Copy, Clone, Debug)]
pub struct PrototypeH<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s, 'h>,
    pub params: &'h [CoordH<'s, 'h>],
    pub return_type: CoordH<'s, 'h>,
    pub _must_intern: MustIntern,
}
```

Mirror of typing's `PrototypeT` and instantiating's `PrototypeI`, but `IdH`-flavored. The slice `params: &'h [CoordH]` is arena-allocated; equality via `ptr::eq` on the sealed `&'h PrototypeH` (the slice pointer is canonicalized through the intern's structural-equal lookup).

### 5.6 `ExpressionH` and the 50 Payload Structs

```rust
#[derive(Copy, Clone, Debug)]
pub enum ExpressionH<'s, 'h> where 's: 'h {
    ConstantVoidH(&'h ConstantVoidH),
    ConstantIntH(&'h ConstantIntH),
    ConstantBoolH(&'h ConstantBoolH),
    ConstantStrH(&'h ConstantStrH<'s, 'h>),
    ConstantF64H(&'h ConstantF64H),
    // ... 45 more variants ...
    BorrowToWeakH(&'h BorrowToWeakH<'s, 'h>),
    IsSameInstanceH(&'h IsSameInstanceH<'s, 'h>),
    LockWeakH(&'h LockWeakH<'s, 'h>),
    DiscardH(&'h DiscardH<'s, 'h>),
    PreCheckBorrowH(&'h PreCheckBorrowH<'s, 'h>),
}
```

50 variants total (`instructions.rs:26-79`). All payloads are arena-allocated `&'h XH` refs, not interned — expression nodes are unique by construction.

**Derives: `Copy, Clone, Debug` only.** Drops `PartialEq`/`Eq`/`Hash` because `ConstantF64H` holds `f64`. The whole expression-AST family drops them uniformly. This mirrors typing-pass-design-v3.md §7.3.

**The dispatcher.** `ExpressionH::result_type(&self) -> CoordH<'s, 'h>` (`instructions.rs:84-`) is the type-of dispatcher — one big match arming all 50 variants. Per-arm bodies are 1-line field reads or simple computations (`b.ref_expression.result_type()`, `CoordH { ownership: OwnershipH::WeakH, location: LocationH::YonderH, kind: b.ref_expression.result_type().kind }` for `BorrowToWeakH`). The Scala equivalent is per-case-class `override def resultType` overrides; the Rust collapses them into one dispatcher per the "abstract-def dispatcher: dispatcher-on-enum" policy (migration-policy.md sealed-trait policy for simplifying).

**Three-enum wrapper layered above:** `IExpressionH` (2 variants), `ReferenceExpressionH` (variant for expressions returning a value), `AddressExpressionH` (variant for expressions returning a memory address). The shape mirrors typing's `ExpressionTE`/`ReferenceExpressionTE`/`AddressExpressionTE` family.

### 5.7 `StructHT` / `InterfaceHT` / Array Types

The 4 sealed kind payloads. Each is Interning-permanent with a `*ValH` mirror. Shape (concrete example):

```rust
#[derive(Copy, Clone, Debug)]
pub struct StructHT<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s, 'h>,
    pub _must_intern: MustIntern,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructHTValH<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s, 'h>,
}
```

The `*ValH` Val mirror lacks `_must_intern` (it's the construction-time value used to query the interner). `StaticSizedArrayHT`/`RuntimeSizedArrayHT` follow the same pattern with array-shape fields (size, mutability, element coord, etc.).

The corresponding **definition** types (`StructDefinitionH`, `InterfaceDefinitionH`, `StaticSizedArrayDefinitionHT`, `RuntimeSizedArrayDefinitionHT`) are `/// Temporary state` — arena-allocated by-value, held in `Hamuts`'s translation tables, and ultimately end up in `PackageH.struct_defs` / `PackageH.interface_defs` etc.

### 5.8 `FunctionH`, `StructDefinitionH`, `InterfaceDefinitionH`

Definition types. All `/// Temporary state`, arena-allocated but not interned. Drop `Eq`/`Hash` because they embed `ExpressionH` transitively (`FunctionH.body: ExpressionH`) or hold `ArenaIndexMap`s (`PackageH`).

```rust
#[derive(Copy, Clone, Debug)]
pub struct FunctionH<'s, 'h> where 's: 'h {
    pub prototype: &'h PrototypeH<'s, 'h>,
    pub locals: &'h [Local<'s, 'h>],
    pub export_name: Option<StrI<'s>>,
    pub is_user_function: bool,
    pub attributes: &'h [IFunctionAttributeH],
    pub body: ExpressionH<'s, 'h>,
}
```

### 5.9 `ProgramH` and `PackageH`

Top-level container produced by `Hammer::translate`:

```rust
#[derive(Copy, Clone, Debug)]
pub struct ProgramH<'s, 'h> where 's: 'h {
    pub packages: &'h [PackageH<'s, 'h>],
}

pub struct PackageH<'s, 'h> where 's: 'h {
    pub package_coordinate: PackageCoordinate<'s>,
    pub interfaces: &'h [InterfaceDefinitionH<'s, 'h>],
    pub structs: &'h [StructDefinitionH<'s, 'h>],
    pub functions: &'h [FunctionH<'s, 'h>],
    pub static_sized_arrays: &'h [StaticSizedArrayDefinitionHT<'s, 'h>],
    pub runtime_sized_arrays: &'h [RuntimeSizedArrayDefinitionHT<'s, 'h>],
    // Cross-reference tables for backend consumption:
    pub struct_id_to_struct_def: &'h ArenaIndexMap<'h, &'h IdH<'s, 'h>, &'h StructDefinitionH<'s, 'h>>,
    pub interface_id_to_interface_def: &'h ArenaIndexMap<'h, ...>,
    pub function_id_to_function: &'h ArenaIndexMap<'h, ...>,
    pub edge_blueprints: &'h ArenaIndexMap<'h, ...>,
    // Externs and exports:
    pub kind_externs: &'h [HamutsKindExtern<'s, 'h>],
    pub function_externs: &'h [HamutsFunctionExtern<'s, 'h>],
    pub kind_exports: &'h [Export<'s, 'h>],
    pub function_exports: &'h [Export<'s, 'h>],
}
```

`PackageH` opts out of `Eq`/`Hash` because `ArenaIndexMap` does not impl them (`ast.rs:72-73`).

---

## Part 6: VonHammer (VON Wire-Format Emission)

`VonHammer` is the JSON-ish wire-format emission stage for the H-side AST. It produces `IVonData` values (`src/von/ast.rs`: `VonObject`, `VonArray`, `VonMember`, `VonString`, `VonInt`, `VonBool`, `VonFloat`) that the backend consumes verbatim.

**Collapsed onto Hammer** per §2.3. `von_hammer.rs` is the largest simplifying file (2534 LOC) and contains zero state — all methods take `&self` on `Hammer` and read from arguments.

**Entry point**: `vonify_program(&self, program: &ProgramH) -> IVonData` at `von_hammer.rs:39-` walks the entire `ProgramH` tree and emits a top-level `VonObject` with packages, structs, interfaces, functions as nested `VonArray`s. Per-AST-node `vonify_*` methods handle each H-type — e.g. `vonify_expression` is a 50-arm dispatcher mirroring `ExpressionH::result_type`'s shape (one arm per ExpressionH variant).

**Side note: the test VM (testvm)** reads `ProgramH` directly, not VON. VonHammer is purely for downstream wire-format-consuming tools (the actual backend, external introspection); it doesn't affect the test pipeline.

---

## Part 7: Error Handling

**The simplifying pass has no equivalent to `ICompileErrorT`.** By the time the AST reaches `Hammer::translate`, all user errors have been caught upstream. `translate` (`hammer.rs:609-735`) returns `&'h ProgramH<'s, 'h>` directly — no `Result`.

The only `Result`-returning fns on `HammerCompilation` are pass-through delegates to earlier passes (`get_parseds`, `get_scoutput`, `get_compiler_outputs`), and they return the upstream error types (`FailedParse`, `ICompileErrorS`, `ICompileErrorT`) — `hammer_compilation.rs:196-241`.

Internal failures use `panic!` (e.g. `hamuts.rs:326, 371, 388, 310` for duplicate-export detection; `hammer.rs:480, 515, 553` for unimplemented branches). This matches Scala's `vfail`/`vimpl`/`vwat` pattern. No testvm-specific error propagation here — the testvm Result-bubble refactor (`VmRuntimeErrorV<'s>`) is testvm-internal and doesn't reach Hammer.

---

## Part 8: Driving The Pass

### 8.1 Top-Level Entry — `Hammer::translate`

```rust
pub fn translate(&self, hamuts_i: &HinputsI<'s, 'i>) -> &'h ProgramH<'s, 'h>
```

`hammer.rs:609-735`. Public method on `Hammer`. Pseudocode:

1. Destructure `HinputsI` (`:613-623`).
2. Construct fresh stack `Hamuts` with 15 empty maps (`:625-641`).
3. Iterate `kind_exports` → `translate_kind` + `hamuts.add_kind_export` (`:643-646`).
4. Iterate `function_exports`, `kind_externs`, `function_externs` (`:648-678`).
5. `translate_interfaces` → `translate_structs` (`:680-681`).
6. **Two-phase function translate** (user first, then non-user) for ID stability (`:682-685`).
7. Group accumulated definitions by `PackageCoordinate` (`:687-716`).
8. Build `PackageH` per package, `interner.alloc_slice_from_vec` for each list (`:718-733`).
9. `interner.alloc(ProgramH { packages })` and return (`:734`).

No deferred-action drain (unlike typing's `drain_all_deferred`) — simplifying is fully synchronous because by Hammer-time the program is fully monomorphized, no further discovery happens.

### 8.2 `HammerCompilation` Coordination Wrapper

`HammerCompilation<'s, 'h, 'ctx, 't, 'i, 'p>` at `hammer_compilation.rs:83-98`. Six lifetimes — the most of any single struct in the codebase, because it threads through every prior pass's lifetime to orchestrate the full pipeline.

Owns:
- `InstantiatedCompilation` (which transitively owns `TypingPassCompilation`, the typing arena, and all upstream passes).
- The `HammerInterner` reference.
- A cached `Option<&'h ProgramH>` (`hamuts_cache`) — lazy translate-on-first-access pattern.

**Constructor** (`hammer_compilation.rs:125-161`) takes 10 args: `scout_arena`, `interner: &HammerInterner`, `typing_interner`, `keywords`, `parser_keywords`, `parse_arena`, `packages_to_build`, `package_to_contents_resolver`, `options: HammerCompilationOptions`, `instantiating_bump`. `HammerCompilationOptions` (`:39-42`) carries `debug_out: Arc<dyn Fn(&str) + Send + Sync>` and `global_options: GlobalOptions`.

**Entry for callers**: `get_hamuts(&mut self) -> &'h ProgramH<'s, 'h>` at `hammer_compilation.rs:272-285` — lazily runs the pipeline through instantiation and then calls `Hammer::translate`. Idempotent via the cache.

### 8.3 Test Entry

`simplifying/test/test_compilation.rs::test` (lines 23-58) builds the standard test `HammerCompilation` with `BUILTIN` + `test` package coordinates. The integration tests use this same path via `integration_tests/tests/run_compilation::test`.

### 8.4 Backend Handoff

Once `HammerCompilation::get_hamuts` returns `&'h ProgramH`, the caller has two consumers:

1. **Test VM (`testvm/`):** reads `ProgramH` directly via `vivem::execute_with_primitive_args(program_h, ...)` / `execute_with_heap(...)`. Used by integration tests. Returns `Result<IVonData, VmRuntimeErrorV<'s>>` per the architect-approved Result-bubble refactor.
2. **VON emission (`von_hammer::vonify_program`):** for the real backend's wire-format consumer. Returns `IVonData` directly.

The caller can run either or both. `'h` stays alive as long as either consumer holds output references.

---

## Part 9: Invariants Summary

Invariants that aren't fully stated elsewhere in this doc:

1. **All HashMap keys on interned refs use direct `&'i K` or `&'h K`** (no `PtrKey` newtype) — because the I-side and H-side interner seals already guarantee canonical pointers, so `ptr::eq`-based identity is sound directly on the raw refs.
2. **Never use `'static`** (NUSLX). All data must live in `'p`, `'s`, `'i`, or `'h` (or on the stack). Same reason as typing — `&'static T` has a different pointer than a structurally identical `&'h T`, breaking pointer equality for interned types.
3. **No Hamuts side-table read keeps the H-arena borrow live across a mutation.** Copy out before `&mut hamuts` (§3.3).
4. **VonHammer reads but does not mutate Hamuts** — it operates on already-built `&'h ProgramH`. Hamuts has died by then.
5. **The expression-AST family drops `Eq`/`Hash` uniformly** (driven by `ConstantF64H`'s `f64` field, see §5.6).
6. **`MustIntern` seal preserves canonical pointer identity** on `&'h StructHT`, `&'h InterfaceHT`, `&'h StaticSizedArrayHT`, `&'h RuntimeSizedArrayHT`, `&'h PrototypeH`, `&'h IdH` — never construct these except through the `intern_*` methods.

---

## Part 10: Risks / Open Questions

- **`IdH` interner test gating**: the dedicated interner test module is gated off (`hammer_interner.rs:200`) because it predates the post-PhantomData `IdH` field set. Tests should be revived once `IdH` shape stabilizes.
- **VON wire-format parity**: the simplifying-pass output is the **real acceptance criterion** for the migration — `von_hammer`'s emission must match what the Scala backend reads byte-for-byte. The 2 ignored `hammer_tests.rs` integration tests (`extern_method_in_generic_extern_struct_puts_container_args_on_citizen_step_in_wire_format_simple_id` and `mixed_own_inherited_template_args_split_correctly_in_wire_format_simple_id`) assert wire-format shape; un-ignoring them needs the VON-format-comparison harness brought across from Scala.
- **`Hammer` is `!Sync`** — `HammerInterner.inner: RefCell<Inner>` precludes thread-safe sharing. Per-package or per-function parallel translation would require a different interner shape.
- **`HammerCompilation` 6-lifetime soup**: structural consequence of the multi-pass orchestration. Working but novel even in this codebase. If a future pass needs to be inserted between instantiating and simplifying, lifetimes would need to be re-numbered.
- **Body migration status**: most expression-payload structs in `instructions.rs` are bare-placeholders with `PhantomData` per `instructions.rs:9-10`. The methods on `Hammer` in `expression_hammer.rs`/`let_hammer.rs`/`load_hammer.rs`/`mutate_hammer.rs`/`block_hammer.rs` are mostly `panic!()` stubs awaiting body migration (per `simplifying/mod.rs:6-9`).
- **Future intern families**: only 5 wired today. As body migration touches more H-types, additional intern wrappers may be added.

---

## Part 11: Key Files / Directories

| Path | Purpose |
|---|---|
| `docs/architecture/simplifying_pass_design.md` | this doc |
| `docs/architecture/typing-pass-design-v3.md` | upstream pass design — most patterns mirrored here |
| `docs/architecture/instantiator_design.md` | upstream-of-simplifying pass design |
| `FrontendRust/docs/migration/migration-policy.md` | per-pass policy values; simplifying rows at lines 20, 26, 130-136, 154 |
| `FrontendRust/src/simplifying/hammer.rs` | `Hammer` god struct (4 fields, `<'s,'i,'h,'ctx>`); top-level `translate`; `Locals` |
| `FrontendRust/src/simplifying/hamuts.rs` | `Hamuts` 15-field accumulator + mutator API |
| `FrontendRust/src/simplifying/hammer_interner.rs` | 5 intern families (kind-payload, struct, interface, ssa, rsa, prototype, id_h); `MustIntern` seal; arena utilities |
| `FrontendRust/src/simplifying/hammer_arena.rs` | `HammerArena<'h>` newtype |
| `FrontendRust/src/simplifying/hammer_compilation.rs` | 6-lifetime driver wrapper; cached `Option<&'h ProgramH>`; entry point `get_hamuts` |
| `FrontendRust/src/simplifying/expression_hammer.rs` | body-migration: `impl Hammer` blocks for expression translation |
| `FrontendRust/src/simplifying/block_hammer.rs`, `let_hammer.rs`, `load_hammer.rs`, `mutate_hammer.rs` | body-migration: more `impl Hammer` per-area files |
| `FrontendRust/src/simplifying/struct_hammer.rs`, `type_hammer.rs`, `function_hammer.rs` | citizen/type/function translation |
| `FrontendRust/src/simplifying/name_hammer.rs` | name translation (incl. free fns `simplify_id`/`simplify_name`/...) |
| `FrontendRust/src/simplifying/von_hammer.rs` | 2534 LOC: VON wire-format emission; `impl Hammer` blocks |
| `FrontendRust/src/simplifying/conversions.rs` | small I→H enum conversion table (Mutability/Variability/Ownership/Location) |
| `FrontendRust/src/simplifying/test/test_compilation.rs` | standard test `HammerCompilation` builder |
| `FrontendRust/src/final_ast/instructions.rs` | 2374 LOC: `ExpressionH` + 50 payloads + `IExpressionH`/`ReferenceExpressionH`/`AddressExpressionH` + `Local`/`VariableIdH` + `ExpressionH::result_type` dispatcher |
| `FrontendRust/src/final_ast/ast.rs` | 664 LOC: `ProgramH`, `PackageH`, `FunctionH`, definitions, `PrototypeH`, `IdH`, exports/externs |
| `FrontendRust/src/final_ast/types.rs` | 713 LOC: `CoordH`, `KindHT`, the 4 sealed kind payloads + `*ValH` mirrors, `OwnershipH`/`LocationH`/`Mutability`/`Variability`, `SimpleId`/`SimpleIdStep`, `CodeLocation` |
| `FrontendRust/src/final_ast/test/` | test-only AST visitor scaffolding (mirrors typing/test/traverse.rs) |
| `FrontendRust/src/testvm/vivem.rs` | `execute_with_primitive_args` / `execute_with_heap` — testvm entry that reads `&'h ProgramH` |
| `FrontendRust/src/von/ast.rs` | `IVonData` + `VonObject`/`VonArray`/`VonMember`/`VonString`/`VonInt`/`VonBool`/`VonFloat` |
