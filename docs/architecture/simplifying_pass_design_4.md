# Simplifying Pass Design

Architecture and design decisions for the Scala-to-Rust simplifying-pass (Hammer) migration. This is the authoritative design reference; per-test handoff lives in the bucket `migration-drive-todo.md` files; meta-guidance for change-makers lives in `docs/architecture/simplifying-pass-ai-guide.md` (sister doc to typing-pass-ai-guide.md).

The simplifying pass — colloquially "Hammer" after its Scala namesake — is **pass 6** in the Vale frontend pipeline (parse → postparse → higher-typing → typing → instantiate → **simplify** → emit). It consumes `HinputsI<'s, 'i>` (the monomorphic instantiated denizen graph from the instantiator) and produces `ProgramH<'s, 'h>` (the raw "final AST" or "H-AST" that the backend consumes).

Its primary job is **lowering**: walk the monomorphic denizen graph and translate every `cI`-flavored type, name, expression, and definition into a backend-shaped equivalent. Many of these translations are mechanical (e.g. `IntIT { bits: 32, .. }` → `IntHT { bits: 32 }`), but the lowering also does:
- **Identifier mangling**: produces backend-stable string names (`mangleFunc`, `mangleName`, `mangleTemplata`).
- **Local-variable layout**: `LocalsBox`-equivalent state assigns numeric `VariableIdH` to each typing-pass local, tracks scope and unstackify state.
- **Wire-format vonification**: emits `IVonData`-formatted output via `von_hammer.rs` (used by tests for golden comparison and by the backend for serialized AST consumption).

The output ProgramH is what gets written to disk (or fed directly to the LLVM backend in an integrated build).

---

## Part 1: Arena and Lifetime Model

The Hammer pass follows the same **pragmatic arena retention** stance as the typing pass and instantiator. Output types carry `<'s, 'h>` — they can hold `&'s` refs into scout (for `PackageCoordinate`/`StrI` parts that don't need lowering) and reach back into `'i` for intermediate Hamuts state. Most output references stay `&'h` to freshly-interned hammer types.

### 1.1 Five Arenas (Counting `'p`)

By the time Hammer runs, all five input arenas are live:

- **`'p`** — Parser arena. Read-only.
- **`'s`** — Scout arena. Read-only.
- **`'t`** — Typing arena. **Logically still live, but Hammer doesn't touch it.** All typing-pass data was already projected through the instantiator into `'i`-flavored shapes; Hammer's view of the program is `HinputsI<'s, 'i>` only.
- **`'i`** — Instantiating arena. Read-only input to Hammer.
- **`'h`** — **Hammer arena (`HammerInterner<'s, 'h>`)**. Output goes here: every interned hammer type (names, kinds, coords, prototypes), the H-AST (`ExpressionH` variants, definitions, `ProgramH`).

The "five arenas" framing is a bit misleading — `'t` is alive but unused. From Hammer's perspective there are effectively four arenas (`'p`, `'s`, `'i`, `'h`) and its job is `'i` → `'h` translation.

### 1.2 Lifetime Invariants

1. **`'s: 'i: 'h`** — clean nesting. Each arena outlives the next. Stated as `where 's: 'h, 's: 'i, 'i: 'h` on the Hammer god struct and every output type that crosses these boundaries.
2. **`'h` outlives the emit/backend phase.** ProgramH is consumed by `metal_printer.rs` (the wire-format writer) and downstream backends; those must complete before `hammer_interner` drops.
3. **No `'static`.** Same NUSLX discipline — `&'static T` has a different pointer than a structurally identical `&'h T`, breaking pointer-equality for interned types.
4. **AASSNCMCX continues to apply.** No `Vec`/`HashMap`/`String` inside arena-allocated types. Hammer uses `&'h [T]` slices and `ArenaIndexMap` for output collections; `Hamuts` (stack-owned mutable state) uses heap `HashMap` freely.

### 1.3 Arena Construction Order

```rust
fn top_level_driver() {
    // ... typing pass + instantiator produce HinputsI<'s, 'i> ...

    let hammer_bump = Bump::new();
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let mut hamuts = Hamuts::new();
    let program_h = Hammer::translate_program(
        &hammer_interner, &keywords, &instantiating_interner,
        &mut hamuts, hinputs_i);

    // ... emit ProgramH via metal_printer.rs or downstream backend ...
    // hammer_interner drops: 'h dies
    // (then 'i, 't, 's, 'p drop)
}
```

Drop order naturally enforces `'h < 'i < 't < 's < 'p`.

### 1.4 Where Each Type Lives

| Type | Lifetimes | Arena |
|---|---|---|
| `ProgramH` | `<'s, 'h>` | `'h`, allocated; holds `PackageCoordinateMap<'s, PackageH>` |
| `PackageH`, `FunctionH`, `StructDefinitionH`, `InterfaceDefinitionH` | `<'s, 'h>` | `'h`, allocated |
| `KindHT`, `CoordH`, `IdH`, `INameH`, `PrototypeH` | `<'s, 'h>` | `'h`, interned via `HammerInterner` |
| `ExpressionH` variants (~60) | `<'s, 'h>` | `'h`, allocated |
| `Local`, `VariableIdH` | `<'s, 'h>` | `'h`, allocated |
| `IVonData` (wire-format output) | (no arena) | Owned `String` / `Vec<VonMember>` etc. — pure runtime data, not arena-backed |
| `Hammer` (god struct) | `<'s, 'i, 'h, 'ctx>` | Stack |
| `Hamuts` (mutable state) | `<'s, 'i, 'h>` | Stack-owned; heap `HashMap` fields |
| `Locals` (mutable local-var tracker) | `<'s, 'i, 'h>` | Stack-owned per function |
| `FunctionRefH` | `<'s, 'h>` | Inline Copy struct (`{ prototype: &'h PrototypeH }`); not interned |

### 1.5 Why `IVonData` Is Heap, Not Arena

`IVonData` is the wire-format representation — emitted to disk as a string or fed directly to the backend. It's a **runtime data type**, not a program-AST type. Its variants own their data (`String`, `Vec<VonMember>`) because:
1. The consumer (file writer or backend) doesn't care about arena lifetimes.
2. The output is serialized — pointer-identity doesn't carry meaning across the boundary.
3. Wire-format generation is a one-shot walk; no benefit to interning.

This is the one Hammer-side data type that **doesn't** follow the AASSNCMCX rule. It's the explicit boundary where in-memory arena layout exits and runtime serialization takes over.

### 1.6 Polyvalue Pattern: Less Dense Here

Unlike the typing pass and instantiator (with deep Polyvalue hierarchies), Hammer's H-AST mostly uses flat enums. `KindHT` has ~10 variants (`IntHT`, `BoolHT`, `FloatHT`, `StrHT`, `VoidHT`, `NeverHT`, `OpaqueHT`, `StructHT`, `InterfaceHT`, `StaticSizedArrayHT`, `RuntimeSizedArrayHT`) — most are unit-like or hold a `bits: i32`. `ExpressionH` has ~60 variants (one per H-side expression form). The Polyvalue pattern still applies to multi-variant wrapper enums (derive `PartialEq`/`Eq`/`Hash`, never hand-roll `ptr::eq` on the outer), but the depth is shallower than the typing pass.

### 1.7 Why a Separate `HammerInterner`

Same rationale as the typing/instantiating split: per-pass interners let each arena drop independently. `HammerInterner<'s, 'h>` interns H-side types into `'h` and uses scout-arena `StrI<'s>` for string interning that doesn't need an H-flavored copy.

---

## Part 2: The God Struct

### 2.1 Architecture

Hammer collapses Scala's sub-hammers (`NameHammer`, `StructHammer`, `TypeHammer`, `FunctionHammer`, `VonHammer`) into a single `Hammer<'s, 'i, 'h, 'ctx>` struct. **The sub-hammer state in Scala was just `interner` + `keywords` + cross-references between sub-hammers**, so the collapse leaves only the genuinely immutable fields:

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

Immutable configuration only. Mutable state threads through `&mut Hamuts<'s, 'i, 'h>` and (per-function) `&mut Locals<'s, 'i, 'h>` on every method call.

### 2.2 Sub-Hammer Methods Become `impl Hammer` In Per-Area Files

Following the typing-pass `Compiler` precedent: Scala's `NameHammer.scala`, `StructHammer.scala`, etc. don't become struct fields. Their methods become `impl Hammer { ... }` blocks colocated in per-area files for organization:

- `name_hammer.rs` — `mangle_func`, `mangle_name`, `mangle_templata`
- `struct_hammer.rs` — struct/interface lowering
- `type_hammer.rs` — coord/kind lowering
- `function_hammer.rs` — function-shape lowering
- `expression_hammer.rs` — per-`ExpressionTE`-variant lowering (the biggest file)
- `block_hammer.rs` — block lowering + scope/locals tracking
- `let_hammer.rs` — let/stackify lowering
- `mutate_hammer.rs` — mutate-arm lowering
- `load_hammer.rs` — load/borrow/upcast lowering
- `von_hammer.rs` — H-AST → IVonData wire-format conversion

Every file declares `impl Hammer { ... }` with the relevant methods. Same single struct.

### 2.3 Re-entrancy Via `&self + &mut hamuts + &mut locals`

Deep recursion (e.g. `translate_function_body → translate_expression → translate_block → translate_let → translate_expression`) works because `&self` is shared and the mutable accumulators thread through. **Two mutable references**: `&mut hamuts` for global state (functions, structs, interfaces, externs) and `&mut locals` for per-function local-variable state.

The two-mutable-ref pattern is what `LocalsBox` modeled in Scala. The Rust port collapses `LocalsBox` into a single `Locals` struct mutated through `&mut self` methods — architect-blessed (matches the `HamutsBox → Hamuts` collapse).

### 2.4 Translation Entry Points

- `Hammer::translate_program(hinputs_i) -> ProgramH` — top-level entry.
- `Hammer::translate_function(...) -> FunctionH` — per-function lowering.
- `Hammer::translate_expression(...) -> ExpressionH` — per-expression dispatch.
- `Hammer::vonify_program(program_h) -> IVonData` — wire-format generation.

The split mirrors Scala's per-sub-hammer entry points; the methods just live on `Hammer` now.

---

## Part 3: Lowering State — `Hamuts`

`Hamuts` ("hammer outputs") is the mutable accumulator. Parallels `CompilerOutputs` (typing) and `InstantiatedOutputsI` (instantiator). Single struct, stack-owned, heap-backed `HashMap`s.

### 3.1 Structure

```rust
pub struct Hamuts<'s, 'i, 'h>
where 's: 'i, 'i: 'h,
{
    pub human_name_to_full_name_to_id: HashMap<String, HashMap<String, i32>>,
    pub struct_t_to_opaque_h: HashMap<&'i StructIT<'s, 'i, cI>, &'h OpaqueHT<'s, 'h>>,
    pub struct_t_to_struct_h: HashMap<&'i StructIT<'s, 'i, cI>, &'h StructHT<'s, 'h>>,
    pub struct_t_to_struct_def_h: HashMap<&'i StructIT<'s, 'i, cI>, StructDefinitionH<'s, 'h>>,
    pub struct_defs: Vec<StructDefinitionH<'s, 'h>>,
    pub static_sized_arrays:
        HashMap<&'i StaticSizedArrayIT<'s, 'i, cI>, StaticSizedArrayDefinitionHT<'s, 'h>>,
    pub runtime_sized_arrays:
        HashMap<&'i RuntimeSizedArrayIT<'s, 'i, cI>, RuntimeSizedArrayDefinitionHT<'s, 'h>>,
    pub interface_t_to_interface_h:
        HashMap<&'i InterfaceIT<'s, 'i, cI>, &'h InterfaceHT<'s, 'h>>,
    pub interface_t_to_interface_def_h:
        HashMap<&'i InterfaceIT<'s, 'i, cI>, InterfaceDefinitionH<'s, 'h>>,
    pub function_refs: HashMap<&'i PrototypeI<'s, 'i, cI>, FunctionRefH<'s, 'h>>,
    pub function_defs: HashMap<&'i PrototypeI<'s, 'i, cI>, FunctionH<'s, 'h>>,
    pub package_coord_to_export_name_to_function:
        HashMap<PackageCoordinate<'s>, HashMap<StrI<'s>, &'h PrototypeH<'s, 'h>>>,
    pub package_coord_to_export_name_to_kind:
        HashMap<PackageCoordinate<'s>, HashMap<StrI<'s>, KindHT<'s, 'h>>>,
    pub package_coord_to_prototype_to_extern:
        HashMap<PackageCoordinate<'s>, HashMap<&'h PrototypeH<'s, 'h>, HamutsFunctionExtern<'s, 'h>>>,
    pub package_coord_to_kind_to_extern:
        HashMap<PackageCoordinate<'s>, HashMap<&'h OpaqueHT<'s, 'h>, HamutsKindExtern<'s, 'h>>>,
}
```

### 3.2 Bidirectional Maps: I-Side Key, H-Side Value

The defining shape of Hamuts is `Map<I-side-key, H-side-value>`: for each I-side denizen (`StructIT[cI]`, `PrototypeI[cI]`), record the lowered H-side equivalent (`StructHT`, `FunctionH`). This avoids re-lowering when the same I-side type appears multiple times in the program (e.g. a struct used by 50 callers).

The "translate or lookup" pattern is the lowering-loop's main idiom:

```rust
if let Some(existing) = hamuts.struct_t_to_struct_h.get(&struct_i) {
    return *existing;
}
let struct_h = self.intern_struct_h(...); // build new
hamuts.struct_t_to_struct_h.insert(struct_i, struct_h);
struct_h
```

Mirrors Scala's identical `getOrElseUpdate` pattern.

### 3.3 `HashMap`, Not `IndexMap`

Hamuts uses `HashMap` (not `IndexMap`). Insertion order *doesn't* matter for output determinism in this pass — final ProgramH ordering is controlled separately (`struct_defs: Vec<StructDefinitionH>` carries the deterministic ordering; the maps are pure lookup tables). This is a divergence from the instantiator's `IndexMap` choice; the rationale is that Hamuts's maps are pure caches, not iteration sources.

### 3.4 No Speculative Writes

Same as the instantiator — every write is the result of a confirmed lowering. No rollback machinery.

---

## Part 4: Locals — The Per-Function Companion

`Locals<'s, 'i, 'h>` is a **per-function** mutable accumulator, distinct from `Hamuts` (which is program-wide). It tracks local-variable layout and scope state during expression lowering.

### 4.1 Structure (Sketch)

```rust
pub struct Locals<'s, 'i, 'h> {
    pub var_id_to_local: HashMap<VariableIdH, Local<'s, 'h>>,
    pub var_name_to_var_id: HashMap<IVarNameI<'s, 'i, cI>, VariableIdH>,
    pub unstackified: HashSet<VariableIdH>,
    pub next_var_id: i32,
    // ... scope-tracking fields ...
}
```

### 4.2 Why Separate From Hamuts

Local-variable state is meaningless outside the function being lowered. Threading it through `Hamuts` would muddle the global/per-function distinction. Each call to `translate_function` constructs a fresh `Locals` and discards it after the function's H-form is built.

### 4.3 Scala's `LocalsBox` Collapsed

In Scala, `LocalsBox` was a mutable wrapper around an immutable `Locals` (functional-update style). Per architect directive (matching `HamutsBox → Hamuts` precedent and the typing-pass `CompilerOutputs` model), the Rust port has a single mutable `Locals` accumulator. `LocalsBox`'s methods become `&mut self` / `&self` methods on `Locals` directly. Audit-trail comments preserve the Scala `var inner: Locals` form under `(collapsed)` markers.

### 4.4 `VariableIdH` Allocation

Each call to `add_local` mints a fresh `VariableIdH` (monotonically increasing). The numeric IDs are stable within a function — they're what backends use to address locals in stack frames. Within a function the IDs are deterministic by source-order; across functions they're independently allocated (no global registry).

---

## Part 5: Output AST — `ProgramH`

```rust
pub struct ProgramH<'s, 'h> where 's: 'h {
    pub packages: PackageCoordinateMap<'s, PackageH<'s, 'h>>,
}
```

### 5.1 PackageH Bundles Per-Package Definitions

Each `PackageH` holds the lowered per-package state: function defs, struct defs, interface defs, exports, externs. Within a package, ordering is deterministic and matches the source-walk order.

### 5.2 Identifier Identity Via `IdH`

Backend stability requires that every named entity have a backend-stable string form. `IdH` carries `(package_coord, init_steps, local_name, shortened_name, fully_qualified_name)` — multiple representations for different consumers. `mangle_func` / `mangle_name` / `mangle_templata` produce the mangled form used in the wire format.

The `IdH` `Display` impl (added during MI bucket's Phase 1 work — see commit history around `print_float`) emits the Scala case-class auto-toString shape for debug parity.

### 5.3 ExpressionH Is Very Wide

`ExpressionH<'s, 'h>` has ~60 variants — one per H-side expression form. Most are 1:1 lowerings of typing-pass/instantiator expression forms (`ConstantIntH`, `ArgumentH`, `LocalLoadH`, `LocalStoreH`, `StackifyH`, `RestackifyH`, `UnstackifyH`, `DestroyH`, `IfH`, `WhileH`, `BreakH`, `BlockH`, `ConsecutorH`, `CallH`, `ExternCallH`, `InterfaceCallH`, `ConstructH`, `StaticArrayFromCallableH`, `MemberLoadH`, `IsSameInstanceH`, `MapH`, …).

Each variant lowers to backend-consumable bytes via `von_hammer.rs`. The vonification process is mostly mechanical but carries one critical invariant: **wire-format field names must match Scala exactly**. `VonMember("sourceExpr", ...)` not `VonMember("source_expr", ...)`. Backend goldens depend on this.

### 5.4 `result_type()` On Every ExpressionH

Every `ExpressionH` variant carries a `result_type(&self) -> CoordH` method on the dispatcher. This is the backend's source-of-truth for stack frame layout — `CoordH` says ownership + location (Inline/Yonder) + kind. Bugs in `result_type` produce subtle backend miscompiles, so this method is a frequent panic-stub target during body migrations.

---

## Part 6: VonHammer — The Wire-Format Boundary

`von_hammer.rs` walks `ProgramH` and emits `IVonData` (the structured wire format consumed by the backend or written to disk). Every H-AST node has a corresponding `vonify_*` method.

### 6.1 Type Tags Match Scala String-For-String

The `tyype` field of `VonObject` is a Scala-faithful string: `"ConstantInt"`, `"LocalLoad"`, `"Restackify"`, `"ExternCall"`, etc. Tests assert on these via golden comparison; any divergence breaks the round-trip.

### 6.2 Field Names Match Scala Camel-Case

Wire-format field names use Scala's camelCase: `"sourceExpr"`, `"resultType"`, `"argumentIndex"`, `"knownLive"`, `"localName"`. **Not** Rust's snake_case. The Rust `VonMember` struct holds a `String` for the field name; vonify_* methods supply the camelCase literal.

### 6.3 No Behavior, Only Translation

`von_hammer.rs` is pure translation — it doesn't compute, doesn't error, doesn't allocate into the H arena. The only state it reads is the input `ProgramH`; the output `IVonData` is owned (string/vec). This makes it the cleanest part of the pass to migrate — every method is a 1:1 Scala port with no interesting design decisions.

---

## Part 7: Recurring Traps

### 7.1 Wire-Format String Mismatch

The single biggest trap is camelCase-vs-snake_case in wire format. Rust idiom wants `"source_expr"`; Scala parity demands `"sourceExpr"`. **Always check the Scala VonHammer.scala source for the exact string** when porting a `vonify_*` arm. Verifying this through-the-pipeline panic-by-panic is what the MI bucket spent most of its Phase 1 on.

### 7.2 IdH Display Divergence — `StrI` Renders Bare

Scala's `StrI` is a case class whose auto-toString is `StrI(<value>)`. Rust's `impl Display for StrI` (`interner.rs`) prints the bare underlying string for usability. This means `IdH.Display` output differs between Scala and Rust by the `StrI(...)` wrapping — flagged at `Display` impl sites for any future stdout-golden test that compares. Not currently a regression risk because no test uses stdout goldens at the H level.

### 7.3 Variable ID Reuse Across Functions

`Locals.next_var_id` starts at 0 (or 1) per function. Don't try to share `Locals` across functions — IDs would collide, the backend would produce garbage stack-frame layouts.

### 7.4 `ExpressionH::result_type()` Arm Coverage

The `result_type` dispatcher has ~60 arms, one per variant. Missing an arm panics at runtime when that variant is encountered. During body migrations, "next panic in result_type" is one of the most common surfaces. Land them sibling-arm-by-sibling-arm; mirror the relevant Scala `def resultType: CoordH` override on the matching case class.

### 7.5 The Guardian Disable-All Block Pattern

The `result_type` dispatcher lives under `/* Guardian: disable-all */` because it's a polymorphic-trait dispatcher with no per-arm Scala body to anchor each `CoordH {...}` arm to. New arms land directly inside the existing disable-all block; no per-arm Scala comment is needed (the variant's case-class `def resultType` lives next to the variant declaration, not in the dispatcher).

### 7.6 `StructIT` / `InterfaceIT` Identity For Hamuts Keys

Hamuts maps are keyed by `&'i StructIT<'s, 'i, cI>` (pointer identity on the interned I-side payload). The Rust `HashMap` uses `Hash` + `Eq` — for these `&'i` refs, that's pointer-Hash + pointer-Eq via the Polyvalue derive. Don't try to wrap in `PtrKey<'i, StructIT>` here — the `&'i` ref already hashes by pointer (per IEOIBZ on `&IT` types in the instantiator).

### 7.7 `cI` Phantom Carries Through

Hammer reads `cI`-flavored types but doesn't construct any `sI`-flavored ones. If you find yourself wanting to read an `sI`-flavored type in Hammer, something is wrong — the instantiator should have collapsed it before the boundary. Surface to the architect; don't paper over with a phantom cast.

---

## Part 8: Design Decisions That Are Easy To Get Wrong

### 8.1 `'i` Is Read-Only From Hammer

Hammer never mutates the instantiator's output. `hinputs_i: &'ctx HinputsI<'s, 'i>` is held by shared ref; all reads are `&` traversals. Side effects of lowering go to `Hamuts` and `Locals`.

### 8.2 No Type-Level Genericity (Like `R` In The Instantiator)

The H-side has no `R` phantom — wire-format consumers don't know about region modes. Types are concrete: `CoordH { ownership: OwnershipH, location: LocationH, kind: KindHT }`. The H-side `OwnershipH` enum has `MutableShareH`, `MutableBorrowH`, `OwnH`, `WeakH`, `ImmutableShareH`, `ImmutableBorrowH` — flat. No phantom typing.

### 8.3 Wire-Format Output Is Not Arena

`IVonData` is heap-owned `String`/`Vec` — explicitly NOT arena-backed. This is the only point in the entire pipeline where the AASSNCMCX rule is relaxed: at the serialization boundary. Don't try to "fix" this by arena-allocating the `String`s — the output is consumed across an FFI/disk boundary where arena lifetimes don't carry.

### 8.4 Sub-Hammers Are Just Files, Not Structs

When porting a Scala sub-hammer method, **add it as `impl Hammer { ... }` in the matching per-area file** — not as a struct field, not as a free function. Scala had `nameHammer.mangleFunc(id)`; Rust has `self.mangle_func(id)` from any `Hammer` method. The cross-references between sub-hammers (Scala's `structHammer` calling `typeHammer.translateCoord`) just become `self.translate_coord(...)` in Rust.

### 8.5 `FunctionRefH` Is A Tiny Inline Struct

`FunctionRefH { prototype: &'h PrototypeH }` is small Copy and lives inline in containing types. Not interned, not arena-allocated separately. Pass by value freely.

### 8.6 `Hamuts.struct_defs: Vec<StructDefinitionH>` Is The Order Source

The `struct_defs` field (and parallel `interface_defs` etc. in the final output) defines the iteration order for output. Hamuts's maps are caches; the `Vec`s control determinism. When finalizing into `ProgramH`, iterate the `Vec` rather than the map.

### 8.7 Externs Don't Get Lowered Like Functions

`HamutsFunctionExtern` and `HamutsKindExtern` carry through with minimal translation — they're already "external" (the backend supplies their implementation). Don't run them through the same translate-or-lookup pipeline as regular functions; they have their own per-package map keyed by `(package_coord, prototype_h)` / `(package_coord, opaque_h)`.

---

## Part 9: Cross-Pass Boundaries

### 9.1 Input — `HinputsI<'s, 'i>`

The instantiator's output. Hammer reads, never writes.

### 9.2 Output — `ProgramH<'s, 'h>`

The simplifying pass's output. Two consumers downstream:
1. **`metal_printer.rs` / `von_hammer.rs`** — produces `IVonData` wire format for disk emission.
2. **Direct backend integration** — newer integrated backend reads `ProgramH` directly without round-tripping through IVonData.

### 9.3 Test Surface — Test Helpers Use `compile.get_hamuts()`

Integration tests inspect post-Hammer state via `RunCompilation::get_hamuts()` which returns `&'h ProgramH<'s, 'h>`. Tests walk `program_h.packages` and inspect function lists, prototypes, etc. The MI bucket landed several tests using this surface (`test_export_functions`, `test_extern_functions`, `simple_program_with_print`).

### 9.4 Per-Bucket Migration Status

Hammer body fills landed steadily through all four buckets' Phase 1 (extensive expression-lowering arms across `expression_hammer.rs`/`block_hammer.rs`/`let_hammer.rs`/etc.). Phase 2 added the harder cases (Restackify pipeline, Map arm via parallel_foreach work, IfH/WhileH/BreakH). As of sprint closeout, most arms are filled; remaining `panic!()`s cluster around exotic templata combinations.

---

## Part 10: Risks / Open Questions

### 10.1 Long-Running Process Memory

Same caveat: `'h` arena holds the entire ProgramH for the backend's duration. For batch compilation this is fine; for an LSP-style daemon you'd want streaming emit.

### 10.2 Wire-Format Stability

`IVonData` field names and ordering define the wire-format contract. Changes here cascade to any external consumer (golden files, backend, on-disk format). Treat as a versioned API — additions are safe; renames/reorders break consumers.

### 10.3 Mangling Determinism

`mangleFunc` / `mangleName` / `mangleTemplata` produce backend identifiers. Their output must be deterministic across runs and stable across migrations. Any change to mangling format is a breaking ABI change for downstream artifacts. The Scala mangler hasn't been re-examined recently — the Rust port mirrors it line-for-line and inherits whatever quirks Scala had.

### 10.4 Per-Function Locals Allocation

`Locals` clones nothing arena-side, but its `HashMap` allocations add up across many functions. Per-function alloc cost hasn't been measured; the structure is small enough that this is unlikely to be a bottleneck.

---

## Part 11: SEE ALSO

- **Instantiator design** (input to this pass): `docs/architecture/instantiator_design.md`.
- **Typing pass design** (two passes upstream): `docs/architecture/typing-pass-design-v3.md`.
- **Operational/change-maker guidance** for Hammer edits: live in the per-bucket migration-drive-todo files and the migrate-tl.md root playbook.
- **Frontend Scala source for Hammer**: `Frontend/SimplifyingPass/src/dev/vale/simplifying/Hammer.scala` + neighbors.
- **MI bucket's contributed Hammer infra**: the bulk of expression_hammer / let_hammer / mutate_hammer / von_hammer body fills (Phase 1 + Restackify pipeline in Phase 2 round-3).
