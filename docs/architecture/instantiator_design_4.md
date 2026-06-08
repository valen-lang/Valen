# Instantiator Pass Design

Architecture and design decisions for the Scala-to-Rust instantiator-pass migration. This is the authoritative design reference; per-test handoff lives in the bucket `migration-drive-todo.md` files; meta-guidance for change-makers lives in `docs/architecture/instantiator-ai-guide.md` (sister doc to typing-pass-ai-guide.md).

The instantiator is **pass 5** in the Vale frontend pipeline (parse → postparse → higher-typing → typing → **instantiate** → simplify → emit). It consumes `HinputsT<'s, 't>` (the typed denizen graph) and produces `HinputsI<'s, 'i>` (the monomorphic instantiated denizen graph). Its primary job is **monomorphization**: walk the program from each `exported` entry point, substitute concrete templatas for `KindPlaceholderT` parameters, dedupe instantiations by identity, and emit one concrete denizen per reachable instantiation. Tree-shaking falls out of reachability — uncalled overloads never appear in `HinputsI`.

For comparison: where the typing pass produces *templated* function/struct/interface definitions (one per source-level declaration, parameterized by `KindPlaceholderT`), the instantiator produces *fully monomorphic* ones (one per reachable substitution, with all placeholders replaced by concrete kinds/coords).

---

## Part 1: Arena and Lifetime Model

The instantiator adopts the same **pragmatic arena retention** stance as the typing pass: scout and typing data live past the instantiator, so instantiator output can reference them directly. Output types carry `<'s, 'i>` — they can hold `&'s` refs into scout and reach back into `'t` typing data via the bound-arg substitutions, but most output references stay `&'i` to the freshly-interned instantiator types.

### 1.1 Four Arenas (Plus `'p`)

By the time the instantiator runs, all four input arenas are live:

- **`'p`** — Parser arena. Read-only.
- **`'s`** — Scout arena. Read-only.
- **`'t`** — Typing arena (`TypingInterner<'s, 't>`). The instantiator reads `HinputsT<'s, 't>` and walks `IdT`/`KindT`/`ITemplataT` payloads from here as input.
- **`'i`** — **Instantiating arena (`InstantiatingInterner<'s, 'i>`)**. Output goes here: every interned instantiator type (names, kinds, coords, prototypes, signatures, function/struct/interface definitions) and the top-level `HinputsI<'s, 'i>` container.

All four use `bumpalo`. The instantiating arena is created *after* `HinputsT` is built; the typing arena outlives it through the entire instantiator pass. The simplifying pass reads `HinputsI<'s, 'i>` directly — `'s` and `'i` both stay live through simplification.

### 1.2 Lifetime Invariants

1. **`'s: 't` and `'s: 'i`** — the scout arena outlives both typing and instantiating outputs. Stated as explicit `where 's: 't, 's: 'i` bounds on every output type that holds `&'s` data.
2. **`'t` and `'i` are *independent*** — neither outlives the other in a fixed direction. The typing arena holds typing-pass output; the instantiating arena holds instantiator output. They share `'s` lineage but their own arenas drop in caller-controlled order.
3. **`'i` outlives the instantiator and the simplifying pass.** Hammer (the simplifying god struct) reads `HinputsI<'s, 'i>` and continues holding `&'i` refs throughout its own pass.
4. **No `'static`.** Same NUSLX discipline as the typing pass — `&'static T` has a different pointer than a structurally identical `&'s T` / `&'i T`, silently breaking pointer-equality for interned types.
5. **AASSNCMCX continues to apply.** No `Vec`/`HashMap`/`String` inside arena-allocated types. The instantiator's `ArenaIndexMap` (insertion-ordered, arena-backed) covers the cases where Scala used `Map[K, V]`.

### 1.3 Arena Construction Order

```rust
fn top_level_driver() {
    let parse_arena = ParseArena::new();
    // ... parser produces parser AST into 'p ...
    let scout_arena = ScoutArena::new();
    // ... postparser/higher-typing produce into 's ...

    let typing_bump = Bump::new();
    let typing_interner = TypingInterner::new(&typing_bump);
    let hinputs_t = run_typing_pass(&scout_arena, &typing_interner, ...);

    let instantiating_bump = Bump::new();
    let instantiating_interner = InstantiatingInterner::new(&instantiating_bump);
    let hinputs_i = instantiator::translate(
        &opts, &instantiating_interner, &typing_interner, &keywords, &hinputs_t);

    let hammer_bump = Bump::new();
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let program_h = simplify(&hammer_interner, &hinputs_i, ...);

    // hammer_interner drops: 'h dies
    // instantiating_interner drops: 'i dies
    // typing_interner drops: 't dies
    // scout_arena drops: 's dies
    // parse_arena drops: 'p dies
}
```

Drop order (reverse of declaration) naturally enforces `'h < 'i < 't < 's < 'p`. *Note:* `'t` and `'i` could in principle be dropped in either order — the design treats them as separable but the call sites construct `'t` first by convention.

### 1.4 Two Region Modes: `cI` vs `sI` — The Defining Phantom

Vale's region system splits a denizen into **collapsed** (region-erased) and **split** (region-retained) views. The instantiator carries this distinction as a phantom-typed parameter `R` on every region-sensitive interned type.

- **`sI`** (split — sites I) — region info preserved per-site. Types parameterized as `IdI<'s, 'i, sI>`, `PrototypeI<'s, 'i, sI>`, `CoordI<'s, 'i, sI>`, etc. carry one of the per-call-site region collapse trees.
- **`cI`** (collapsed — composite I) — region info collapsed to defaults. Types parameterized as `IdI<'s, 'i, cI>`, `CoordI<'s, 'i, cI>`, etc. are the version that flows into `HinputsI.{structs, interfaces, functions}` (the canonical output).

```rust
#[allow(non_camel_case_types)] pub struct sI;
#[allow(non_camel_case_types)] pub type cI = sI;  // sealed alias today; distinct concept
impl region_mode_sealed::Sealed for sI {}
impl IRegionsModeIT for sI {}
```

Mechanically `cI` is currently a type alias for `sI`, but semantically they are different roles. The phantom-typed seal (`IRegionsModeIT`) prevents accidental cross-region-mode mixing. Every interned instantiator type carries `<'s, 'i, R>` where `R: IRegionsModeIT`. The three **region collapser** modules (`region_collapser_consistent.rs`, `region_collapser_individual.rs`, `region_counter.rs`) translate `sI` → `cI` at well-defined points.

This is the single biggest divergence from the typing pass design. In the typing pass, every templated type has one canonical shape; in the instantiator, *every* shape has two — pre-collapse (sI, region-aware) and post-collapse (cI, region-erased). Output types living in `HinputsI.functions: &'i [&'i FunctionDefinitionI<'s, 'i>]` are `cI`-flavored; intermediate per-call-site computations work in `sI`-flavored types and collapse on emit.

### 1.5 Where Each Type Lives

| Type | Lifetimes | Arena |
|---|---|---|
| `HinputsI` | `<'s, 'i>` | `'i` for the struct itself; holds `&'i [&'i T]` slices of definitions |
| `FunctionDefinitionI<'s, 'i>` | `<'s, 'i>` | `'i`, allocated; holds `cI`-flavored ids and prototypes |
| `StructDefinitionI<'s, 'i, cI>` | `<'s, 'i, cI>` | `'i`, allocated |
| `InterfaceDefinitionI<'s, 'i, cI>` | `<'s, 'i, cI>` | `'i`, allocated |
| `KindIT`, `IdI`, `INameI`, `ITemplataI` (interned) | `<'s, 'i, R>` | `'i`, interned via `InstantiatingInterner` |
| `CoordI` | `<'s, 'i, R>` | inline Copy struct (~24 bytes — ownership + kind ref); not interned |
| `PrototypeI`, `SignatureI` | `<'s, 'i, R>` | `'i`, value-type with sealed-canonical interning at opt-in sites |
| `EdgeI`, `InterfaceEdgeBlueprintI` | `<'s, 'i>` | `'i`, allocated |
| `KindExportI`, `FunctionExportI`, `KindExternI`, `FunctionExternI` | `<'s, 'i>` | `'i`, allocated |
| `InstantiationBoundArgumentsI` | `<'s, 'i>` | `'i`, value type holding `ArenaIndexMap`s |
| `DenizenBoundToDenizenCallerBoundArgI` | `<'s, 't, 'i>` | Stack-owned temporary (uses heap `IndexMap`; not arena-allocated) |
| `InstantiatedOutputsI` | `<'s, 't, 'i>` | Stack-owned accumulator (mutable; uses heap `IndexMap`) |
| `InstantiatorI` (god struct) | `<'s, 'ctx, 't, 'i>` | Stack |

### 1.6 Polyvalue Pattern Continues

The same @TFITCX / @PVECFPZ design applies to instantiator wrapper enums. `INameI<'s, 'i, R>` (analog of typing's `INameT`), `KindIT<'s, 'i, R>` (analog of `KindT`), `ITemplataI<'s, 'i, R>` (analog of `ITemplataT`) are 16-byte Polyvalues that derive `PartialEq`/`Eq`/`Hash` — never hand-roll `ptr::eq(self, other)` on the outer reference. The interned variant payloads compare via `ptr::eq` on the inner `&'i T`; the wrapper-derive delegates correctly.

### 1.7 Why a Separate `InstantiatingInterner`

Scala used one `Interner` for the entire pipeline. The Rust port splits into per-pass interners (`HammerInterner` for parsing strings, `TypingInterner` for typing types, `InstantiatingInterner` for instantiator types) so each arena drops independently. This means the instantiator holds **both** `typing_interner` (for reading T-side data and helper calls like `TemplataCompiler::get_super_template`) **and** `instantiating_interner` (for emitting I-side data). The god struct's two-interner field set is a Rust-specific adaptation; Scala had a single shared `interner` field.

---

## Part 2: The God Struct

### 2.1 Architecture

The instantiator collapses Scala's `Instantiator` class into a `InstantiatorI<'s, 'ctx, 't, 'i>` struct. Unlike the typing-pass `Compiler` (which collapsed ~17 sub-compilers), the Scala instantiator already lived mostly in one file (`Instantiator.scala`) — the collapse is structurally lighter but the pattern is identical.

```rust
pub struct InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub opts: &'ctx GlobalOptions,
    pub interner: &'ctx InstantiatingInterner<'s, 'i>,
    pub typing_interner: &'ctx TypingInterner<'s, 't>,
    pub keywords: &'ctx Keywords<'s>,
    pub hinputs: &'ctx HinputsT<'s, 't>,
}
```

Immutable configuration only. Mutable state threads through `&mut InstantiatedOutputsI<'s, 't, 'i>` on every call — same `&self + &mut state` re-entrancy pattern as `Compiler` + `CompilerOutputs`.

### 2.2 Sub-Module Methods Become `impl InstantiatorI`

Scala had helper objects (`TemplataCompiler` for templata translation, plus some module-level functions in `Instantiator.scala`). In Rust, **TemplataCompiler stays in the typing pass** because instantiator-side helpers don't need its full surface; the few methods the instantiator uses are accessed via `typing_interner` or are reimplemented as `impl InstantiatorI` methods.

Test-side helpers (`Collector.scala`'s `Collector.all`/`only` for instantiator AST inspection) port to `collector.rs` — the parallel of `typing/test/traverse.rs` from the typing pass. Macros: `NodeRefI` (instantiator-AST node ref) + `collect_only_inode!` / `collect_where_inode!` macros analogous to the T-side.

### 2.3 Re-entrancy Via `&self + &mut monouts`

Deep recursion (e.g. `translate_struct_definition → translate_member → translate_coord → translate_struct_id → translate_struct_definition`) works because `&self` is shared-borrowed and `&mut monouts: &mut InstantiatedOutputsI` is re-borrowed at each call level. Same idiom as the typing pass.

### 2.4 The `translate` Entry Point

```rust
pub fn translate<'s, 'ctx, 't, 'i>(
    opts: &'ctx GlobalOptions,
    interner: &'ctx InstantiatingInterner<'s, 'i>,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    keywords: &'ctx Keywords<'s>,
    hinputs: &'ctx HinputsT<'s, 't>,
) -> HinputsI<'s, 'i>
```

Top-level entry. Constructs the `InstantiatorI`, initializes `InstantiatedOutputsI`, walks all exported entry points, drains the deferred queue until convergence, then finalizes `HinputsI`. Mirrors Scala's `Instantiator.translate(opts, interner, keywords, hinputs)`.

---

## Part 3: Monomorphization State — `InstantiatedOutputsI`

The mutable accumulator. Parallels `CompilerOutputs` in the typing pass but with simpler structure (no deferred-action queue per se — monomorphization is driven by walking from entry points outward, with reachability tracked via "already-instantiated" sets).

### 3.1 Structure

```rust
pub struct InstantiatedOutputsI<'s, 't, 'i> where 's: 't, 's: 'i {
    pub structs: IndexMap<IdI<'s, 'i, cI>, &'i StructDefinitionI<'s, 'i, cI>>,
    pub interfaces: IndexMap<IdI<'s, 'i, cI>, &'i InterfaceDefinitionI<'s, 'i, cI>>,
    pub functions: IndexMap<IdI<'s, 'i, cI>, &'i FunctionDefinitionI<'s, 'i>>,
    pub struct_to_mutability: IndexMap<IdI<'s, 'i, cI>, MutabilityI>,
    pub struct_to_bounds: IndexMap<IdI<'s, 'i, sI>, DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>,
    pub interface_to_bounds: IndexMap<IdI<'s, 'i, sI>, DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>,
    pub function_to_bounds: IndexMap<IdI<'s, 'i, sI>, DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>,
    pub kind_externs: Vec<KindExternI<'s, 'i>>,
    pub function_externs: Vec<FunctionExternI<'s, 'i>>,
    pub kind_exports: Vec<KindExportI<'s, 'i>>,
    pub function_exports: Vec<FunctionExportI<'s, 'i>>,
    pub abstract_func_to_v_table_index:
        IndexMap<IdI<'s, 'i, cI>, IndexMap<IdI<'s, 'i, cI>, PrototypeI<'s, 'i, cI>>>,
    pub deferred_evaluating_function:
        Vec<(IdT<'s, 't>, IndexMap<IRuneS<'s>, ITemplataI<'s, 'i, sI>>)>,
    // ... additional reverse-index maps and tracker sets ...
}
```

Note the two-flavor keys: structural identity uses `cI` (collapsed view); bound-arg substitutions and intermediate work use `sI`. This is intentional. The collapse step happens as the denizen is finalized into `structs`/`interfaces`/`functions`.

### 3.2 `IndexMap`, Not `HashMap`

All instantiator maps use `indexmap::IndexMap` rather than `std::HashMap`. **Insertion-ordered iteration is load-bearing** for determinism — `HinputsI.functions` etc. must come out in a stable order so downstream wire-format output (`VonHammer`) produces byte-stable goldens. The typing pass dodges this via `ArenaIndexMap` for the arena-allocated maps; the instantiator's stack-owned `InstantiatedOutputsI` uses heap `IndexMap` directly.

### 3.3 `cI`-Identity vs `sI`-Identity

Keying the **same denizen** by `IdI<sI>` (one entry per call-site region-set) vs `IdI<cI>` (one entry per region-collapsed identity) is the instantiator's biggest source of subtle bugs. Rule of thumb:

- **`cI`-keyed**: any map whose values are *final* output (`structs`, `interfaces`, `functions`, `struct_to_mutability`, `abstract_func_to_v_table_index`).
- **`sI`-keyed**: any map whose values are *per-instantiation substitution data* (`struct_to_bounds`, `interface_to_bounds`, `function_to_bounds`).

If you find yourself looking up a `cI` id in an `sI`-keyed map (or vice versa), something upstream went wrong — don't paper over it with a flavor cast; trace back to find the collapse-site boundary.

### 3.4 The Deferred Function Queue

`deferred_evaluating_function: Vec<(IdT, IndexMap<IRuneS, ITemplataI>)>` carries call sites that the instantiator discovered but hasn't yet processed (typically because the callee was reached via a virtual dispatch or generic body that hasn't been instantiated for these template args yet). The driver loop pops from this vector, calls `translate_function_definition` for that `(name, template_args)` pair, and repeats until empty. Single-pass; no fixpoint iteration.

`deferred_evaluating_function` is `Vec`-not-`VecDeque` (Scala parity). FIFO/LIFO ordering doesn't matter for output determinism since the work-list is keyed by interned `IdT` (already-instantiated dedup).

### 3.5 Speculative Writes Don't Apply

Unlike the typing pass's `attempt_candidate_banner` (where speculative writes are idempotent during overload resolution), the instantiator never speculates. Every write to `InstantiatedOutputsI` is the result of a confirmed reach from an exported entry point. No rollback machinery, no candidate-attempt patterns.

---

## Part 4: The Bound-Arg Substitution Story

`DenizenBoundToDenizenCallerBoundArgI` is the **most important conceptual type** in the instantiator. It records how a denizen's *parameter bound* names (`FunctionBoundNameT`, `ImplBoundNameT`) get resolved at a specific call site.

```rust
pub struct DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i> where 's: 't, 's: 'i {
    pub func_id_to_bound_arg_prototype: IndexMap<IdT<'s, 't>, &'i PrototypeI<'s, 'i, sI>>,
    pub bound_param_impl_id_to_bound_arg_impl_id: IndexMap<IdT<'s, 't>, IdI<'s, 'i, sI>>,
}
```

### 4.1 What It Models

In the typing pass, generic denizens take "function bounds" and "impl bounds" as implicit parameters — a generic `func foo<T>(t T)` that internally calls `drop(t)` carries a `FunctionBoundNameT` for `drop[T]`. The caller supplies the actual `drop` prototype at instantiation time. `DenizenBoundToDenizenCallerBoundArgI` records this mapping for one specific instantiation: for *this* call site, which concrete function (`func_id_to_bound_arg_prototype`) and which concrete impl (`bound_param_impl_id_to_bound_arg_impl_id`) gets substituted for each bound parameter.

### 4.2 Stack-Owned, Cloned Liberally

`DenizenBoundToDenizenCallerBoundArgI` is **not arena-allocated**. It's a stack value carried by reference or cloned. Scala used `Map[IdT, PrototypeI]` with structural-sharing semantics; Rust uses `IndexMap` and clones. Performance is not a concern — each instantiation builds a small map.

### 4.3 `.plus(...)` Composition

The `plus` method merges two bound-arg sets (used when a denizen passes its bounds through to a callee). Scala's `union` semantics: if both maps have the same key, the values must agree (else `vfail`). Rust port uses `union_maps_expect_no_conflict` helper.

### 4.4 Keyed by `'t`-IdT, Values are `'i`

Notable lifetime asymmetry. The *keys* (`IdT<'s, 't>`) reference the typing arena because they identify a denizen's bound *parameter* — a typing-pass-defined entity. The *values* (`PrototypeI<'s, 'i, sI>`, `IdI<'s, 'i, sI>`) are instantiator output. So `DenizenBoundToDenizenCallerBoundArgI` carries `<'s, 't, 'i>` — three lifetime parameters. Don't try to collapse this to `<'s, 'i>`; you'd lose the precise typing-pass identity on the parameter side.

---

## Part 5: Region Collapsing

Three modules handle the `sI` → `cI` translation:

### 5.1 `region_collapser_consistent.rs`

The **type-collapse** pass. Given an `sI`-flavored type (e.g. `CoordI<'s, 'i, sI>`), produces the `cI`-equivalent by erasing per-site region information. Handles the recursive case of `KindIT::StructIT(&StructIT { id: IdI{...}, .. })` etc.: collapse the inner `IdI`, re-intern under `cI`, return the new `StructIT`.

Mirrors Scala's `RegionCollapserConsistent`. Most arms are straightforward 1:1 ports — `IntIT`/`BoolIT`/`FloatIT` etc. just rewrap with `cI` phantom; `StructIT`/`InterfaceIT`/`StaticSizedArrayIT`/`RuntimeSizedArrayIT` recurse.

### 5.2 `region_collapser_individual.rs`

The **name-collapse** pass. Given an `sI`-flavored name (e.g. `INameI::FunctionNameIX(&FunctionNameIX { template_args: &[ITemplataI<sI>], .. })`), produces the `cI`-equivalent. Mostly mechanical, but each name variant needs its own arm — the `IVarNameI` cluster (Iterable/Iterator/IterationOption) was wired during the CL bucket's foreach work.

Mirrors Scala's `RegionCollapserIndividual`. Recursive over name structure; uses `intern_*_ci` macros to allocate the result into `'i` under the `cI` phantom.

### 5.3 `region_counter.rs`

Counts and assigns region identifiers when constructing fresh `sI`-flavored types. Used during call-site instantiation to give each region a unique identifier within the call scope. Not heavily invoked from the body migrations done so far; defers to Scala when the instantiator path doesn't reach it.

### 5.4 Why Three Modules

The three correspond to Scala's three collapser objects. Splitting (consistent / individual / counter) reflects different roles in the pipeline:
- **Counter** runs *first* — assigns fresh region IDs during sI construction.
- **Consistent** runs *during* sI → cI collapse on types.
- **Individual** runs alongside Consistent on names.

In practice, body migrations call `collapse_coord` / `collapse_id` / `collapse_kind` helpers that thread the right collapser internally; the three-module split is invisible at most call sites.

---

## Part 6: Output AST — `HinputsI`

```rust
pub struct HinputsI<'s, 'i> where 's: 'i {
    pub interfaces: &'i [InterfaceDefinitionI<'s, 'i, cI>],
    pub structs: &'i [&'i StructDefinitionI<'s, 'i, cI>],
    pub functions: &'i [&'i FunctionDefinitionI<'s, 'i>],
    pub interface_to_edge_blueprints:
        ArenaIndexMap<'i, IdI<'s, 'i, cI>, InterfaceEdgeBlueprintI<'s, 'i>>,
    pub interface_to_sub_citizen_to_edge:
        ArenaIndexMap<'i, IdI<'s, 'i, cI>, ArenaIndexMap<'i, IdI<'s, 'i, cI>, EdgeI<'s, 'i>>>,
    pub kind_exports: &'i [KindExportI<'s, 'i>],
    pub function_exports: &'i [FunctionExportI<'s, 'i>],
    pub kind_externs: ArenaIndexMap<'i, &'i StructIT<'s, 'i, cI>, KindExternI<'s, 'i>>,
    pub function_externs: &'i [FunctionExternI<'s, 'i>],
}
```

### 6.1 What Survives Through

- **All denizen definitions** monomorphized to `cI`. `FunctionDefinitionI` carries no `R` parameter — functions are always `cI`-flavored at output (their bodies reference `cI`-typed coords).
- **Edge blueprints + v-tables** for interface dispatch. Each interface gets a blueprint listing its abstract functions; each (interface, sub-citizen) pair gets an edge with the concrete override prototypes.
- **Externs + exports** verbatim with full type info.
- **No env data.** Envs were typing-pass-only — by the time the instantiator emits a denizen, all overload resolution has happened, so there are no `OverloadSet`-kinded coords to carry envs through. (Inert `ReinterpretTE` survives in expression bodies but elides on emit.)

### 6.2 What's Tree-Shaken

Functions never reached from an exported entry never get instantiated, never get into `InstantiatedOutputsI.functions`, never make it into `HinputsI.functions`. This is what `test_shaking` confirms — `bork` (uncalled) and the unused `helperFunc(str)` overload don't appear.

Tree-shaking is **a consequence of monomorphization-by-walking**, not a separate pass. There's no "dead code elimination" step.

### 6.3 `lookup_struct_by_name` and Other Inspection Methods

Test-side inspection (currently filled during the MI bucket's stolen-from-SI cluster — see `hinputs.rs::lookup_struct_by_name`) walks `HinputsI.structs` linearly. Scala-parity bodies: filter by `INameI::StructName` matching, assert size, return first. Not hot — only test code calls these.

---

## Part 7: Recurring Traps

### 7.1 `cI` vs `sI` Phantom Confusion

The single biggest trap. Symptoms:
- Compile error "expected `cI`, found `sI`" on a `IdI<'s, 'i, X>` field.
- Runtime panic in `region_collapser_consistent` because the input was already collapsed.
- HashMap lookup miss because the lookup key was the wrong flavor.

**Diagnosis:** trace the type from where it was created. `sI` comes out of per-call-site instantiation work; `cI` comes out of collapsers. If a final denizen field demands `cI` and you have `sI`, you need to collapse. If an intermediate computation demands `sI` and you have `cI`, something already collapsed it incorrectly.

### 7.2 Stale Bound-Arg Snapshot

`DenizenBoundToDenizenCallerBoundArgI` is captured at the *call site* where instantiation is decided. If you reuse one across multiple call sites (e.g. iterating over methods of an interface and using the interface's bound-arg map for each method), you may apply the wrong substitution. Mirror Scala's pattern: re-compute the bound-arg map per call site, even when it looks redundant.

### 7.3 `IndexMap` Cloning Cost

`IndexMap` clones aren't free at scale. The `DenizenBoundToDenizenCallerBoundArgI::plus` call clones two maps and unions them; called transitively in deeply-nested instantiations, the per-call O(n) cost compounds. Not currently a measured bottleneck, but profile before scaling.

### 7.4 Two-Interner Field Confusion

`InstantiatorI` holds both `interner: &InstantiatingInterner` (for emitting I-side data) and `typing_interner: &TypingInterner` (for reading T-side data). Pattern-matching `coord_t.kind` (typing-pass type) needs `typing_interner`-side variants; constructing the corresponding `CoordI` needs `instantiating_interner`-side helpers. Mixing them produces confusing borrow-checker errors. **Convention:** if your local variable is `*_t`, use `typing_interner`; if it's `*_i` or `*_ci`, use `interner`.

### 7.5 Expression-Side Translation Surface

The expression-translation arms (`translate_ref_expr`, `translate_address_expr`, `translate_expression`) carry the same width as the typing pass's expression evaluators. ~48 ReferenceExpressionTE arms × the work each one does. This is a frequent source of "the next panic" during body migrations; expect 1-3 panic-replacements per integration test.

### 7.6 `vassert_one` and `vassertSome`

Scala's `vassertOne(coll.find(...))` is the idiomatic "expect exactly one match" pattern. Ported as `vassert_one(coll.iter().find(...))` returning `&T`. **Don't substitute** `coll.iter().find(...).unwrap()` — that's `vassertSome`, which only checks `Option::Some`. The `vassert_one` variant asserts uniqueness too (the helper at `crate::utils::vassert::vassert_one` enforces both).

### 7.7 Region Collapse Position-Correctness

When `region_collapser_individual.rs` ports a Scala arm, the **interleave order** of arms matters for SCPX (Scala Comment Parity). Scala's source ordering must be preserved at the `intern_*_ci` call ordering — see the CL bucket's `migrate_rsa` and the `Iterable / ConstructingMember / Iterator / IterationOption` block as the canonical example.

---

## Part 8: Design Decisions That Are Easy To Get Wrong

### 8.1 `'t` Is Read-Only From The Instantiator

The instantiator never mutates the typing-pass output. `hinputs: &'ctx HinputsT<'s, 't>` is held by shared ref; all reads are `&` traversals. If you find yourself wanting to mutate something on the T-side, you're solving the wrong problem — the answer is to record the mutation on the I-side accumulator instead.

### 8.2 No `IInDenizenEnvironmentT` Equivalent

The typing pass has `IInDenizenEnvironmentT` as the narrow wrapper (8-variant subset of `IEnvironmentT` for "denizen-being-compiled" contexts). The instantiator has no equivalent because there's no concept of "currently compiling" — instantiation is per-call-site and doesn't accumulate scopes the way typing does. Don't introduce one; if a Scala arm uses `IInDenizenEnvironmentT`, it's typing-pass code being elided at the instantiator boundary.

### 8.3 Heavy Templatas Don't Have Bound-Arg Storage

Heavy templatas (`FunctionTemplataI`, `StructDefinitionTemplataI`, etc., the I-side analogs of the typing pass's heavy templatas) **do not carry their own bound-arg map**. The substitution context lives in `InstantiatedOutputsI` keyed by the templata's id. This is why translating a templata to a concrete denizen requires a lookup into `outputs.{struct,interface,function}_to_bounds` — the bound info isn't on the templata itself.

### 8.4 Externs Don't Get `cI` Collapse

`FunctionExternI` and `KindExternI` carry `sI`-flavored types directly into `HinputsI` (well, more precisely: extern data is emitted untouched by the region collapsers because there's no region info to collapse — externs are by definition fixed-shape). Don't run them through `collapse_coord` etc.; pass through as-is.

### 8.5 The Phantom-`+T` Erasure Rule Continues

Same as typing pass §6.0: Scala `+T <: SomeTrait` parameters are erased to monomorphic widest-form in Rust. `IdI[+T <: INameI]` → `IdI<'s, 'i, R>` with `local_name: INameI<'s, 'i, R>`. Pattern-matching at use sites narrows.

### 8.6 No Speculative Recursion

The instantiator can recurse deeply (e.g. instantiating `func foo<T>` triggers instantiating its body, which calls `bar<List<T>>`, which triggers instantiating `bar`, …). The driver loop bounds recursion by tracking "already-instantiated" via the `InstantiatedOutputsI.functions` keyed lookup. **Don't disable** the already-instantiated check — non-termination on cyclic generics will surface immediately.

### 8.7 `ArenaIndexMap` In Output But `IndexMap` In State

The output container (`HinputsI`) uses arena-backed `ArenaIndexMap` because the output is arena-allocated. The mutable state (`InstantiatedOutputsI`) uses heap `IndexMap` because it lives on the stack and gets mutated freely. Don't try to make `InstantiatedOutputsI` use arena maps — the mutation surface doesn't fit.

---

## Part 9: Cross-Pass Boundaries

### 9.1 Input — `HinputsT<'s, 't>`

The instantiator reads, never writes. Walked from exported entry points outward.

### 9.2 Output — `HinputsI<'s, 'i>`

The instantiator produces, the simplifying pass reads. After the instantiator's `translate(...)` returns, `InstantiatedOutputsI` is finalized into `HinputsI` (arena-allocate the slices and arena-index-maps, copy the final state). The simplifying pass then holds `&hinputs_i` for its full duration.

### 9.3 Per-Bucket Migration Status

As of the four-way Phase 2 sprint closeout, the instantiator's body fills are mostly complete: every Phase-2 integration test exercises some instantiator arm. Remaining `panic!()`s are mostly in cold paths (unusual templata combinations, error-channel branches). Ongoing work tracked in `FrontendRust/src/typing/typing-pass-todo.md` and the per-bucket `migration-drive-todo.md` files.

---

## Part 10: Risks / Open Questions

### 10.1 Long-Running Process Memory

Same caveat as the typing pass: arena retention means the instantiator's output is held for the simplifying pass's duration. For batch compilation this is fine; for an LSP-style long-running daemon, the cumulative arena would grow unboundedly. Two-tier per-denizen arenas would fix this — see the typing pass's `environments-per-denizen-long-term.md` reasoning doc.

### 10.2 Region Collapse As The Last Region Pass

Today's design treats region collapse as a single forward sweep from `sI` to `cI` at the instantiator boundary. A more region-aware backend (one that wants to preserve region info further into codegen) would need to skip collapsing. Not in scope for this migration; flagged in case the architecture choice is revisited.

### 10.3 Determinism via `IndexMap` Order

`InstantiatedOutputsI` uses `IndexMap` for insertion-ordered iteration. Determinism of `HinputsI` output depends on the insertion order being a function of the entry-point order. If parallelized (multiple entry points processed concurrently), ordering becomes nondeterministic. Not currently a problem; would need attention if the instantiator parallelizes.

### 10.4 Bound-Arg Map Scale

`DenizenBoundToDenizenCallerBoundArgI` clones happen at every transitive call boundary. For programs with many small generic functions and deep call chains, the cumulative clone cost grows. Profile before pessimizing; current evidence is that real-world test programs are well within tolerances.

---

## Part 11: SEE ALSO

- **Typing pass design** (input to this pass): `docs/architecture/typing-pass-design-v3.md`.
- **Simplifying pass design** (consumer of this pass's output): `docs/architecture/simplifying_pass_design.md`.
- **Operational/change-maker guidance** for instantiator edits: live in the per-bucket migration-drive-todo files and the migrate-tl.md root playbook.
- **Frontend Scala source for instantiator**: `Frontend/InstantiatingPass/src/dev/vale/instantiating/Instantiator.scala` + neighbors.
- **MI bucket's contributed instantiator infra** (Range/Map/add_entry/etc.): commits `09ae12b17`, `c3885f7d6` on `experimental`.
