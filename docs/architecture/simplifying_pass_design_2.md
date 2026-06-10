# Simplifying Pass (Hammer) Design

Architecture and design decisions for the Scala-to-Rust **simplifying pass** migration. The Scala code calls this pass "Hammer" — that's the verb (and most type-suffix `H`) you'll see throughout. Sister doc to `typing-pass-design-v3.md` and `instantiator_design.md`. This pass was transplanted from Sylvan and drift-reconciled against Vale's canonical `Frontend/SimplifyingPass/`; **some design decisions here are recovered-from-code rather than architect-deliberated** — flagged inline.

For body-migration TL/JR process, see `Luz/skills/guardian-tl.md`. For operational status, see `migrate-tl.md`. The Scala source of truth is `Frontend/SimplifyingPass/src/dev/vale/simplifying/`.

---

## Part 1: Arena and Lifetime Model

The simplifying pass takes instantiator output (`HinputsI<'s, 'i>`) and lowers it to a **wire-format-ready** AST (the `H`-suffix types: `ProgramH`, `ExpressionH`, `CoordH`, etc.). The output is consumed by `VonHammer` (see Part 5) to emit VON wire format that the backend reads.

**The trade.** The hammer pass is the last stop before serialization. Output lives just long enough for VON emission, but the pass still uses an arena (`'h`) because:
- The H-side AST is mutually recursive (`KindHT → CoordH → KindHT`); arena allocation avoids `Box` chains.
- Interning H-side ids (`IdH`) and prototypes (`PrototypeH`) is required for VON wire-format determinism — the same logical id must round-trip to the same wire identifier.

**Critical: VON parity is the documented acceptance criterion.** Per `migrate-tl.md` ("Roadmap To Completion") and `docs/migration/migration-policy.md`, the test of correctness for the full Rust frontend migration is whether `von_hammer` output matches what Scala's backend reads. Every architectural decision here defers to that.

### 1.1 Four Arenas Threaded

- **`'s` — Scout arena**. Inherited; read-only. Holds `PackageCoordinate`, `StrI`, scout-side names.
- **`'i` — Instantiating arena**. Inherited input. Holds `StructIT[cI]`, `PrototypeI[cI]`, `KindIT[cI]` keys used as `Hamuts` HashMap keys.
- **`'h` — Hammer arena** (`HammerArena<'h>`, `hammer_arena.rs:8`). Owned by this pass.
- **`'ctx`** — Borrow lifetime of the `Hammer` god struct's references.

### 1.2 Lifetime Invariants

1. **`'s` outlives `'i` and `'h`.** Both `where 's: 'i` and `where 's: 'h` declared on every output type. Bumpalo doesn't infer.
2. **`'i` outlives `'h`.** `where 'i: 'h` declared on `Hamuts` and downstream. The hammer pass borrows instantiator-arena data (`&'i StructIT` as keys into the I→H translation maps), so `'i` must outlive the hammer pass.
3. **Arena borrow convention.** Short borrow (`&HammerArena<'h>` or `&'ctx HammerArena<'h>`), never the arena's own lifetime.
4. **Never `'static`** — same NUSLX rule.
5. **AASSNCMCX.** No `Vec`/`HashMap`/`String` inside arena-allocated H-side types. Use `&'h [T]` and `ArenaIndexMap<'h, K, V>`. Driver structs (`Hamuts`, `Hammer`, `Locals`) live on the stack and use plain `HashMap`/`Vec`.

### 1.3 Arena Construction Order

```rust
fn run_pipeline() {
    // ... typing + instantiating passes ...
    let hammer_bump = Bump::new();
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let hamuts = simplifying::translate(
        &hammer_interner, keywords, scout_arena,
        instantiating_interner, &hinputs_i,
    );
    // hammer_bump drops: 'h dies
    // instantiating_bump drops: 'i dies
    // typing_bump drops: 't dies
    // scout_arena drops: 's dies
}
```

### 1.4 Where Each Type Lives

| Type | Lifetimes | Arena |
|---|---|---|
| `ProgramH` (top-level output) | `<'s, 'h>` | `'h`, holds `&'h` interior refs and `ArenaIndexMap` |
| `PackageH` | `<'s, 'h>` | `'h` |
| `StructDefinitionH`, `InterfaceDefinitionH`, `FunctionH` | `<'s, 'h>` | `'h` |
| `IdH`, `PrototypeH`, `InternedKindPayloadH` | `<'s, 'h>` | `'h`, interned |
| `CoordH`, `KindHT`, `OwnershipH`, `LocationH` | inline Copy values | Stack-passed |
| `ExpressionH`, `IExpressionH`, payload structs (`ConstantIntH`, `CallH`, …) | `<'s, 'h>` | `'h`, not interned |
| `Hamuts` (mutable accumulator) | `<'s, 'i, 'h>` | Stack, heap-backed HashMaps |
| `Locals` | `<'s, 'h>` | Stack |
| `Hammer` (god struct) | `<'s, 'i, 'h, 'ctx>` | Stack |

### 1.5 Single Region Mode

**The hammer pass has no `R` region-mode generic.** By the time data reaches H-side, the instantiator's region-collapse has already run — every `KindIT`/`CoordI`/etc. arriving at the hammer is in `cI` mode. The hammer simply drops the `R` parameter entirely.

This is **load-bearing for the interner**: `HammerInterner` has one HashMap per logical type (not three), per `hammer_interner.rs:6-8`: "by H-side, region modes already collapsed." If the instantiator's `cI`-collapse ever becomes lazy or partial, the hammer interner would need region-aware keys.

---

## Part 2: The God Struct

### 2.1 Architecture

All Scala sub-hammers (NameHammer, StructHammer, TypeHammer, FunctionHammer, VonHammer, etc.) collapse into a single `Hammer` struct in `hammer.rs:452`:

```rust
pub struct Hammer<'s, 'i, 'h, 'ctx> {
    pub interner: &'ctx HammerInterner<'s, 'h>,
    pub keywords: &'ctx Keywords<'s>,
    pub scout_arena: &'ctx ScoutArena<'s>,
    pub instantiating_interner: &'ctx InstantiatingInterner<'s, 'i>,
}
```

Comment in source (`hammer.rs:445-451`): "Central god struct (typing-pass `Compiler` precedent). Sub-hammer fields from Scala (`nameHammer`, `structHammer`, `typeHammer`, `functionHammer`, `vonHammer`) NOT held as Rust fields — their methods become `impl Hammer { ... }` blocks colocated in per-area files."

Methods spread across nine per-area files, each contributing `impl Hammer` blocks:

| File | Concern |
|---|---|
| `hammer.rs` | Top-level `translate`, mangling helpers, consecutor flattening |
| `type_hammer.rs` | Kind/Coord/Prototype lowering |
| `struct_hammer.rs` | Struct/interface/edge translation |
| `function_hammer.rs` | Function-shell translation |
| `block_hammer.rs` | Block translation |
| `expression_hammer.rs` | The mega-match: every `RE::*` → `ExpressionH` |
| `let_hammer.rs` | Let, restackify, destructuring lets |
| `load_hammer.rs` | Local/member loads, address↔reference conversion |
| `mutate_hammer.rs` | Local/member stores |
| `von_hammer.rs` | VON wire-format emit (2,501 LOC — largest) |

Plus `name_hammer.rs` uses **free functions** (`simplify_id`, `simplify_kind`, `simplify_name`) — they're pure on their args, take the interner explicitly, don't touch `Hammer` state.

### 2.2 `&self` + `&mut Hamuts` + `&mut Locals` Threading

Same re-entrancy pattern as typing/instantiating. The hammer god struct is `&self` everywhere; mutable state is in `Hamuts` and (for function bodies) `Locals`. Deep recursion through `translate_function → translate_block → translate_expression → translate_call → ...` works because every borrow is short-lived.

### 2.3 Collapsed Mutable Wrappers: `HamutsBox`, `LocalsBox`, `VonHammer`

Scala has a recurring pattern: an immutable struct (`Hamuts`) wrapped in a mutable wrapper (`HamutsBox`) that holds it as `var`. The Rust port **collapses both onto a single struct mutated via `&mut self`** (architect directive, per `hamuts.rs:430-436` and `mod.rs:7-9`, typing-pass `CompilerOutputs` precedent).

Likewise:
- `LocalsBox` collapsed into `Locals` (`hammer.rs:52-58`).
- `VonHammer` collapsed onto `Hammer` (impl blocks in `von_hammer.rs`; per `mod.rs` and source comment).

**Don't reintroduce the Box wrappers** even when the borrow checker complains. The complaint is almost always solvable by re-borrowing at a higher scope (copy-out-then-mutate per typing §4.3).

---

## Part 3: Locals

`Locals` (`hammer.rs:278`) is the per-function-body local-variable state:

```rust
pub struct Locals<'s, 'h> {
    next_local_id_number: i32,
    // ... maps from variable-id to slot, etc.
}
```

Threaded as `&mut self.locals` (or `&mut locals`) through every expression-translation method that introduces or references a local. Increments `next_local_id_number` on stackify (`hammer.rs:228-232`).

**Inferred:** `Locals` is per-function-scope (not per-pass). Each `translate_function` constructs a fresh `Locals`, drains it as expressions are translated, and discards on function exit. No cross-function state lives here.

---

## Part 4: `Hamuts` — The Mutable Accumulator

### 4.1 Structure

`Hamuts<'s, 'i, 'h>` (`hamuts.rs:437`) — 16 fields. Same role as typing's `CompilerOutputs` and instantiator's `InstantiatedOutputsI`, threaded as `&mut hamuts` through every `translate_*` method.

Field groups:

- **Naming dedup.** `human_name_to_full_name_to_id: HashMap<String, HashMap<String, i32>>` — the canonical disambiguating id minter for human-readable mangled names.
- **Struct lookups.** `struct_t_to_opaque_h`, `struct_t_to_struct_h`, `struct_t_to_struct_def_h` (3 maps keyed by `&'i StructIT`), plus a `Vec<StructDefinitionH>` accumulator.
- **Array lookups.** `static_sized_arrays`, `runtime_sized_arrays` (keyed by `&'i StaticSizedArrayIT`/`RuntimeSizedArrayIT`).
- **Interface lookups.** `interface_t_to_interface_h`, `interface_t_to_interface_def_h`.
- **Function lookups.** `function_refs`, `function_defs` (keyed by `&'i PrototypeI`).
- **Exports.** `package_coord_to_export_name_to_function`, `package_coord_to_export_name_to_kind`.
- **Externs.** `package_coord_to_prototype_to_extern`, `package_coord_to_kind_to_extern`.

All `HashMap`/`Vec` on the stack — no arena allocation for `Hamuts` itself. Values are either `&'h T` refs (interned arena allocations) or owned `*DefinitionH` structs (which themselves hold `&'h` refs internally).

### 4.2 Why HashMap Keys Are I-Side Refs

The I→H translation maps (`struct_t_to_struct_h` etc.) are keyed by `&'i StructIT` (instantiator-side canonical ref). This is the **canonical identity** for "have I already translated this struct?" — pointer-equality on the I-side ref is the cheapest correct dedup key.

Don't switch these to H-side keys; the H-side is what's being produced, not what's already known.

### 4.3 Per-Package Grouping At The End

Most of `Hamuts` is flat (one big map per concern). `Hammer::translate`'s final step (`hammer.rs:612-735`) **regroups** everything by `PackageCoordinate` into a `PackageCoordinateMap<PackageH>` for the `ProgramH` output. The flat accumulator → per-package output split mirrors Scala.

---

## Part 5: VonHammer — The VON Emit Layer

### 5.1 Role

`von_hammer.rs` (2,501 LOC, **largest file in the pass**) lowers the H-side AST to **VON wire format** — the documented contract boundary with the backend.

Per `mod.rs` and `von_hammer.rs:3`: methods are colocated as additional `impl Hammer` blocks. There's no `VonHammer` struct; everything is `impl Hammer { fn translate_*_von(...) -> IVonData }`.

### 5.2 Output Type — `IVonData`

Lives in `src/von/ast.rs`:

```rust
pub enum IVonData {
    Int(VonInt),
    Bool(VonBool),
    Float(VonFloat),
    Str(VonStr),
    Object(VonObject),
    Array(VonArray),
}

pub struct VonObject {
    pub members: Vec<VonMember>,
}
// etc.
```

This is a tree of self-describing tagged data — the wire format the C-backend expects.

### 5.3 Pipeline Position

`VonHammer` runs **after** `Hammer::translate` produces `ProgramH`. The driver in `HammerCompilation::get_hamuts` (`hammer_compilation.rs:284-296`) caches `ProgramH`; downstream code (the production CLI's VAST emission) walks the cached `ProgramH` and feeds each piece through `Hammer`'s `translate_*_von` methods to produce `IVonData`.

### 5.4 VON Parity As Acceptance Criterion

Per `migrate-tl.md` Roadmap To Completion: **"VON wire-format parity — `von_hammer` output must match what the backend reads (the real acceptance criterion); the ignored tests must assert the *same* behavior as Scala, not merely compile."**

When body-migrating a `*_von` method, the test of correctness is **byte-for-byte equality** with Scala's output on the same input. Other parity drift may be tolerable; VON drift is not.

---

## Part 6: The Output AST (`final_ast/`)

The H-suffix types live in `src/final_ast/` (not in `simplifying/`). This mirrors Scala's `dev.vale.finalast` package — the wire-format-AST is a separate concern from the pass that produces it.

### 6.1 `final_ast/types.rs` — Type Lattice

- **`CoordH`** — inline Copy. `{ ownership, location, kind: KindHT }`.
- **`KindHT`** — 11-variant enum. Primitives (`NeverHT, IntHT, BoolHT, FloatHT, StrHT, VoidHT, OpaqueHT`) inline; compounds (`InterfaceHT, StructHT, StaticSizedArrayHT, RuntimeSizedArrayHT`) as `&'h` refs to interned payloads.
- **`OwnershipH`** — `OwnH, ImmutableBorrowH, MutableBorrowH, WeakH, ImmutableShareH, MutableShareH`.
- **`LocationH`** — `InlineH, YonderH`.
- **`Mutability`, `Variability`** — small Copy enums.
- **`RegionH`** — region descriptor (post-collapse).
- **`SimpleId`/`SimpleIdStep`, `CodeLocation`** — used by VON-side and mangling.
- **`InternedKindPayloadH`**/`InternedKindPayloadValH` — IDEPFL dual-enum pattern for interning the 4 compound kind payloads.

### 6.2 `final_ast/ast.rs` — Top-Level Program Shape

- **`ProgramH<'s, 'h>`** — `{ packages: PackageCoordinateMap<PackageH> }`. The root output.
- **`PackageH<'s, 'h>`** — per-package container: interfaces, structs, functions, arrays, exports, externs. All arena-allocated slices / `ArenaIndexMap`s.
- **`StructDefinitionH`, `StructMemberH`, `InterfaceDefinitionH`, `InterfaceMethodH`, `EdgeH`, `FunctionH`** — the canonical lowered shapes.
- **`PrototypeH`, `IdH`** — interned (have `*ValH` companions, `MustIntern` seal).
- **`FunctionRefH`, `Export`, `IFunctionAttributeH`** — auxiliary.

### 6.3 `final_ast/instructions.rs` — Expression Layer

- **`ExpressionH<'s, 'h>`** — **48-variant enum** of every wire-format instruction (`ConstantIntH, StackifyH, LocalLoadH, CallH, IfH, WhileH, ConsecutorH, BlockH, NewStructH, ReturnH, AsSubtypeH, ...`).
- **`IExpressionH<'s, 'h>`** — 2-variant wrapper splitting `ReferenceExpressionH | AddressExpressionH`. **This is the equivalent of typing's `ReferenceExpressionTE`/`AddressExpressionTE` split.** Note the inversion: typing has the split *as* the top-level type and a 2-variant wrapper for the union; the hammer has one 48-variant enum and a 2-variant wrapper *over* it. Scala parity wins; both designs are correct for their pass.
- **~50 payload structs** — one per `ExpressionH` variant, holding the variant's specific data.
- **`Local`, `VariableIdH`** — local-variable metadata.

Every `ExpressionH` variant has a `result_type(&self) -> CoordH` impl. Adding a new arm requires implementing this — and as of the CL Phase 2 NCWSRX investigation, dispatcher impls over wide enums require an empty `/* */` block after the impl close to satisfy the parity shield.

---

## Part 7: Interner Model

### 7.1 `HammerInterner<'s, 'h>`

`hammer_interner.rs:36`. `RefCell<Inner>` wrapping `&'h Bump`. Three intern maps (which serve five logical interned types via dispatch):

```rust
kind_payload_val_to_ref: HashMap<InternedKindPayloadValH, InternedKindPayloadH>
prototype_val_to_ref:    HashMap<PrototypeHValH, &'h PrototypeH>
id_h_val_to_ref:         HashMap<IdHValH, &'h IdH>
```

`InternedKindPayloadValH` dispatches to four logical types (`StructHT`, `InterfaceHT`, `StaticSizedArrayHT`, `RuntimeSizedArrayHT`), so the interner covers six logical types via three HashMaps + one dispatch enum.

### 7.2 Dual-Enum Pattern

Same `*ValH` (key, transient-borrowed) + `*H` (canonical arena ref) pattern as typing/instantiating. `MustIntern(())` seal (`hammer_interner.rs:23`) per @SICZ.

### 7.3 No Region Modes

Unlike `InstantiatingInterner` (12 families), `HammerInterner` has one family per logical type. See §1.5.

### 7.4 Arena Allocation Helpers

`HammerInterner` also exposes raw-allocation helpers (since it owns `&'h Bump`):
- `alloc<T>(&self, T) -> &'h mut T`
- `alloc_slice_copy<T: Copy>(&self, &[T]) -> &'h [T]`
- `alloc_slice_from_vec<T>(&self, Vec<T>) -> &'h [T]`
- `alloc_index_map<K, V>(&self) -> ArenaIndexMap<'h, K, V>`
- `alloc_index_map_from_iter<K, V>(&self, iter) -> ArenaIndexMap<'h, K, V>`

So `HammerInterner` plays double duty: interning maps + raw arena allocator. Same as `TypingInterner`/`InstantiatingInterner`.

---

## Part 8: Driving The Pass

### 8.1 Top-Level Entry

`Hammer::translate` at `hammer.rs:612-735`. Real code, not a stub. Roughly:

1. Destructure `HinputsI` into 9 fields (interfaces, structs, functions, edge tables, exports, externs).
2. Process exports/externs first (they seed naming).
3. `translate_interfaces(&mut hamuts, interfaces)`.
4. `translate_structs(&mut hamuts, structs)`.
5. `translate_functions(&mut hamuts, functions)` — splits user vs non-user batches.
6. Regroup everything by `PackageCoordinate` into a `PackageCoordinateMap<PackageH>`.
7. Allocate `ProgramH { packages }` into `'h` and return `&'h ProgramH`.

### 8.2 Driver Caching: `HammerCompilation`

`hammer_compilation.rs:92`. Owns an `InstantiatedCompilation`, the hammer arena and interner. `get_hamuts(&mut self) -> &'h ProgramH<'s, 'h>` runs the whole `translate` once and caches in `hamuts_cache: Option<&'h ProgramH<'s, 'h>>`. Test harness and production CLI both go through this.

### 8.3 No `Result` Return

Like the instantiator, the hammer doesn't fail on well-typed/well-monomorphized input. Stubbed paths panic; production paths return owned/arena-allocated values.

---

## Part 9: Mangling

`Hammer::mangle_*` (`hammer.rs`, ~6 free fns / methods: `mangle_func`, `mangle_name`, `mangle_struct`, `mangle_kind`, `mangle_coord`, `mangle_templata`). These produce backend-readable string ids.

**Most are `panic!()` stubs today.** Scala's mangling is mostly commented out in the original (the backend evolved to not need most of it); the Rust port carries the structure but few live paths. Mangling is one of the lower-priority body-migration targets.

The naming-dedup state lives on `Hamuts.human_name_to_full_name_to_id` (§4.1) — when two structs have the same human-readable name, they get distinct numeric suffixes via this counter.

---

## Part 10: Invariants Summary

1. **All HashMap keys on interned refs use `PtrKey<'h, T>`** (same as typing §11.1). Caveat: `IdH`'s own `==` is ptr-eq on inner fields; don't wrap in `PtrKey`.
2. **Never `'static`** — NUSLX.
3. **`Hamuts` is stack-owned**; its HashMaps are heap-backed and die when `translate` returns. AASSNCMCX doesn't apply to it.
4. **VON parity is the acceptance criterion.** All other tolerances narrow here.
5. **Single region mode in interner.** Don't add region-aware keys — by H-side it's settled.
6. **No `R` generic on H-side types.** If the instantiator's region collapse stops being eager, the hammer must add `R` back.
7. **`HamutsBox`/`LocalsBox`/`VonHammer` are deleted.** Don't reintroduce the Box-wrapper-around-immutable Scala pattern.

For the rest (`'s: 'h`, `'i: 'h`, copy-out-before-`&mut`, sub-enum stack-only rewrap) see §1.2, Part 4, §6.1, §7.1.

---

## Part 11: Recurring Traps

- **`HashMap<&'i StructIT, _>` borrow conflicts.** Reading the map and then calling a `&mut hamuts` method holds the borrow across the call — copy out the value first (typing §4.3 pattern).
- **Reintroducing `HamutsBox`/`LocalsBox`.** The borrow-checker pain when first migrating a method is real, but the fix is short-borrow-and-copy-out, not reinstating the Box. See `hamuts.rs:430-436`.
- **`@PRIIROZ` template-args reshuffling** (`hammer.rs:796-801`). Extern translation moves trailing `numInherited` template args from the leaf step to the parent step. Forgetting this breaks extern wire format on the inheritance path.
- **Mangling stubs.** `mangle_*` methods mostly panic; many call sites use shortcuts that work for the current test corpus. Don't fill mangling in just because a stub panicked — verify Scala actually produces the mangled name on that path.
- **`@TFITCX` classification on `Hamuts`.** Temporary state, not arena-allocated. Don't add `/// Arena-allocated` annotations to H-side accumulator types.
- **VON output drift.** When body-migrating a `*_von` method, run the typing-pass + instantiator tests, then diff `IVonData` output against a known-Scala-correct baseline. Drift here is the only kind of drift that breaks downstream.
- **NCWSRX on dispatcher arms.** Adding an `ExpressionH` variant arm to a `result_type` (or similar) impl requires an empty `/* */` block after the impl close — Guardian's NCWSRX rust-mode companion explicitly skips `/* Guardian: disable-all */` directives. See `final_ast/instructions.rs:1316-1389` for the established pattern (8 impls there).
- **`consecrash` vs `consecutive`.** Two consecutor-building helpers in `hammer.rs` (`flatten_and_filter_voids:899`, `consecrash:987`). Different semantics — `consecrash` panics on any non-void result inside a void context; `consecutive` is the user-callable builder. Pick by which Scala helper the source uses.

---

## Part 12: Migration State

As of session at experimental tip `18e3e4bda`:

- **AST/scaffolding: ~95%.** `final_ast/` is mostly complete; some `result_type` arms still panic.
- **Interner: 100%.** All 3 maps wired.
- **`Hammer::translate` top-level: real code.**
- **`translate_kind`, `translate_coord`, `translate_interfaces`, `translate_structs`, `translate_functions` shells: real.**
- **`expression_hammer.rs` mega-match: 50-60% migrated by body** (1,983 LOC). Common branches work end-to-end on simple Vale; exotic branches (`LockWeak`, `BorrowToWeak`, `InterfaceToInterfaceUpcast`, `ReinterpretH`, imm RSA destruction, some array branches) still panic.
- **`von_hammer.rs`: ~70%.** Most happy-path emits work; some exotic AST shapes panic.
- **`mangle_*`: ~10%.** Most stubs; matches Scala's mostly-commented-out state.

**Total `panic!()` count: ~98 across the pass.** Top files: `expression_hammer.rs` (21), `hammer.rs` (20, mostly mangling), `von_hammer.rs` (15), `name_hammer.rs` (11). Plus 28 in `final_ast/` (mostly `result_type` for not-yet-needed variants).

Body migration drives off the same JR loop as typing/instantiating (`docs/skills/migration-drive.md`). The hammer-bucket tests landed during CL Phase 2 (`migrate_rsa`, `migrate_ssa`, `array_capacity`, etc.) typically filled 4-6 hammer methods end-to-end per test.

---

## Risks / Open Questions

- **VON parity not yet verified.** Body migration tests check Rust output goes green; they don't check the IVonData matches Scala byte-for-byte. The roadmap "VON wire-format parity" milestone needs a dedicated diff harness before cutover.
- **Mangling underspecification.** The Scala mangling is mostly commented out, so there's no clear spec for what the Rust port should produce. Defer until backend integration forces the question.
- **`PrototypeH` interning soundness.** Two distinct Scala prototypes that lower to structurally-equal `PrototypeH` should dedupe to the same `&'h PrototypeH`. Verify with VON parity tests once that harness exists.
- **No `Result` channel for "I expected this to be valid but it isn't"** conditions. The hammer pass treats well-typed/well-monomorphized input as a precondition. If a future I→H bug produces invalid input, the panic message has to be the diagnostic — invest in good panic-message hygiene.
- **`final_ast/metal_printer.rs` is a 4-line stub.** The actual serializer is `von_hammer.rs` → `IVonData` → consumed by `src/von/`. The metal_printer name is misleading; consider renaming or deleting.

---

## Key Files / Directories

| Path | Purpose |
|---|---|
| `docs/architecture/typing-pass-design-v3.md` | Foundational sister doc |
| `docs/architecture/instantiator_design.md` | Direct upstream pass sister doc |
| `docs/architecture/simplifying_pass_design.md` | This doc |
| `FrontendRust/src/simplifying/hammer.rs` | 1,055 LOC — `Hammer` struct, `Locals`, top-level `translate`, mangling helpers, `flatten_and_filter_voids`, `consecutive`, `consecrash` |
| `FrontendRust/src/simplifying/hammer_arena.rs` | `HammerArena<'h>` newtype |
| `FrontendRust/src/simplifying/hammer_interner.rs` | 3 intern maps, `MustIntern` seal, arena-allocation helpers |
| `FrontendRust/src/simplifying/hammer_compilation.rs` | `HammerCompilation` driver, options, `get_hamuts` cache |
| `FrontendRust/src/simplifying/hamuts.rs` | 940 LOC — `Hamuts` 16-field accumulator + `add_*` methods |
| `FrontendRust/src/simplifying/type_hammer.rs` | Kind/Coord/Prototype lowering |
| `FrontendRust/src/simplifying/conversions.rs` | Mutability/Ownership/Variability/Location atomic conversions |
| `FrontendRust/src/simplifying/name_hammer.rs` | `simplify_*` free fns; VON CodeLocation/PackageCoord translation |
| `FrontendRust/src/simplifying/struct_hammer.rs` | Struct/interface/edge translation (717 LOC) |
| `FrontendRust/src/simplifying/function_hammer.rs` | `translate_functions`, function-shell translation |
| `FrontendRust/src/simplifying/expression_hammer.rs` | 1,983 LOC — the mega-match over `RE::*` |
| `FrontendRust/src/simplifying/block_hammer.rs` | Block translation |
| `FrontendRust/src/simplifying/let_hammer.rs` | Let, restackify, destructuring lets (955 LOC) |
| `FrontendRust/src/simplifying/load_hammer.rs` | Local/member loads, address↔reference (915 LOC) |
| `FrontendRust/src/simplifying/mutate_hammer.rs` | Local/member stores |
| `FrontendRust/src/simplifying/von_hammer.rs` | 2,501 LOC — VON wire-format emit (largest file in the pass) |
| `FrontendRust/src/final_ast/ast.rs` | `ProgramH`, `PackageH`, definitions, exports, attributes |
| `FrontendRust/src/final_ast/types.rs` | `CoordH`, `KindHT`, ownership/location enums, interned payload dual-enums |
| `FrontendRust/src/final_ast/instructions.rs` | 1,869 LOC — `ExpressionH` 48-variant enum + 50 payload structs, `Local`, `VariableIdH` |
| `FrontendRust/src/von/ast.rs` | `IVonData` wire-format value type |
| `Frontend/SimplifyingPass/src/dev/vale/simplifying/` | Scala source of truth |
