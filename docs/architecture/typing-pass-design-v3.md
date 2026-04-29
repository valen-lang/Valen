# Typing Pass Design — v3

Architecture and design decisions for the Scala-to-Rust typing-pass migration. This is the authoritative design reference; operational handoff instructions are in `tl-handoff.md` at the repo root.

For historical slab-by-slab progress (Slabs 0–14b), see `docs/historical/slab-chronicle.md`. Per-slab handoff docs with translation tables and gotchas are in `FrontendRust/docs/migration/handoff-slab-*.md`. The historical design docs (`docs/historical/typing-pass-design-v1.md`, `docs/historical/typing-pass-design-v2.md`, `docs/historical/typing-pass-migration-setup.md`) are obsolete — they each carry "DO NOT FOLLOW" banners.

---

## Part 1: Arena and Lifetime Model

The approach is **pragmatic arena retention**: scout data lives past the typing pass, so typing output can reference it directly. Output types carry `<'s, 't>` — they can hold `&'s` refs into the scout arena. Heavy templatas hold `&'s FunctionA` / `&'s StructA` directly. No re-interning, no side tables for origin data. Envs allocate into the typing arena.

**The trade.** Cost: scout arena memory (FunctionA/StructA/etc.) retained through instantiation; typing arena memory (envs, typing-pass output) retained through instantiation. Rough estimate: hundreds of MB to a few GB for large programs. Fine for batch compilation; reconsider for long-running LSP-style use (see Risks / Open Questions). Benefit: the port maps to Scala line-for-line in most places. No side-table plumbing. Faster to implement, easier to review for Scala parity.

### 1.1 Three Arenas

- **`'p` — Parser arena (`ParseArena<'p>`)**: parser AST, `StrI<'p>`, coords.
- **`'s` — Scout arena (`ScoutArena<'s>`)**: postparser + higher-typing output (`FunctionA<'s>`, `StructA<'s>`, etc.), interned postparser names (`INameS<'s>`, `IRuneS<'s>`, `IImpreciseNameS<'s>`). Unchanged for the typing pass.
- **`'t` — Typing arena (`TypingInterner<'t>`)**: interned typing-pass types (names, kinds, coords, templatas), typing-pass output AST (function defs, expressions, `HinputsT`), **and typing-pass environments** (`IEnvironmentT` and its 9 concrete variants). Created before the typing pass; outlives the typing pass.

All three arenas use `bumpalo`. Arena-allocated structs follow AASSNCMCX: no `Vec`/`HashMap`/`String` inside arena types.

### 1.2 Lifetime Invariants

1. **`'s` outlives `'t`**. Expressed as `where 's: 't` on every output type that transitively holds `&'s` data. Rust does not enforce outlives via drop order — we must declare the bound. Representative structs that carry this bound include `HinputsT<'s, 't>`, `IEnvironmentT<'s, 't>`, `KindT<'s, 't>`, and every interned typing-pass type.
2. **`'t` outlives the typing pass.** Created by the caller before the pass; dropped by the caller after the instantiator is done.
3. **`'s` outlives the instantiator**, because `HinputsT<'s, 't>` contains `&'s` refs. Scout arena drops after the instantiator completes (or later).
4. **Only two arena lifetimes beyond `'p`:** `'s` and `'t`. Envs live in `'t` (with `&'s`-lifetimed content like `FunctionA` refs and `INameS` references flowing through via field types).
5. **Arena borrow convention.** Arena parameters use a short borrow lifetime (elided or named `'ctx`), never the arena's own lifetime. Write `&ScoutArena<'s>` or `&'ctx ScoutArena<'s>`, never `&'s ScoutArena<'s>`. Same for the typing arena. The decoupling works because `arena.alloc(&self, val: T) -> &'t mut T` returns arena-lifetimed data from a short `&self` borrow — callers don't need to hold the arena for all of `'t`/`'s` just to allocate into it.

### 1.3 Arena Construction Order

```
fn top_level_driver() {
    let parse_arena = ParseArena::new();
    // ... parser produces parser AST into 'p ...

    let scout_arena = ScoutArena::new();
    // ... postparser/higher-typing produce into 's ...

    let typing_bump = Bump::new();
    let typing_interner = TypingInterner::new(&typing_bump);
    let hinputs = run_typing_pass(&scout_arena, &typing_interner, ... );

    // ... instantiator runs over HinputsT<'s, 't> ...

    // typing_interner / typing_bump drop (after instantiator): 't dies
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
| Heavy templatas (`FunctionTemplataT`, `StructDefinitionTemplataT`, etc.) | `<'s, 't>` | `'t`; hold `&'t` env refs and `&'s FunctionA`/`&'s StructA` directly |
| `IEnvironmentT` and sub-types | `<'s, 't>` | `'t` (allocated in typing arena) |
| `GlobalEnvironmentT` | `<'s, 't>` | `'t`; one per typing pass |
| `CompilerOutputs` | `<'s, 't>` | stack-owned; heap-backed HashMaps; dies at pass end |
| `Compiler` (god struct) | `<'s, 'ctx, 't>` | stack; dies at pass end |

### 1.5 Full Type Inventory — Which Arena Each Type Lives In

A complete per-type checklist.

**`'s` scout arena — existing, unchanged by the typing pass:**
- `StrI<'s>`, `PackageCoordinate<'s>`, `FileCoordinate<'s>`, `RangeS<'s>`, `CodeLocationS<'s>` — postparser-interned, reused as-is
- `INameS<'s>`, `IRuneS<'s>`, `IImpreciseNameS<'s>` — postparser names, reused as-is
- `FunctionA<'s>`, `StructA<'s>`, `InterfaceA<'s>`, `ImplA<'s>` — higher-typing output, referenced directly by heavy templatas and envs

**`'t` typing arena — interned (dedup via `TypingInterner`):**
- Concrete name structs (`FunctionNameT`, `StructNameT`, etc. — ~60 of them)
- `IdT<'s, 't>` — monomorphic (always widest form with `local_name: INameT<'s, 't>`)
- Concrete Kind payloads: `StructTT<'s, 't>`, `InterfaceTT<'s, 't>`, `StaticSizedArrayTT<'s, 't>`, `RuntimeSizedArrayTT<'s, 't>`, `KindPlaceholderT<'s, 't>`, `OverloadSetT<'s, 't>`
- Interned templata payloads: `CoordTemplataT<'s, 't>`, `KindTemplataT<'s, 't>`, `PlaceholderTemplataT<'s, 't>`, `PrototypeTemplataT<'s, 't>`, `IsaTemplataT<'s, 't>`, `CoordListTemplataT<'s, 't>`
- `PrototypeT<'s, 't>`, `SignatureT<'s, 't>` — monomorphic

**`'t` typing arena — allocated but NOT interned:**
- `FunctionDefinitionT<'s, 't>`, `FunctionHeaderT<'s, 't>`
- `StructDefinitionT<'s, 't>`, `InterfaceDefinitionT<'s, 't>`, `ImplT<'s, 't>`
- `EdgeT<'s, 't>`, `OverrideT<'s, 't>`
- `ParameterT<'s, 't>`
- `ReferenceExpressionTE<'s, 't>` (~48 variants), `AddressExpressionTE<'s, 't>` (~5 variants)
- `InstantiationBoundArgumentsT<'s, 't>`
- Heavy templata payloads: `FunctionTemplataT`, `StructDefinitionTemplataT`, `InterfaceDefinitionTemplataT`, `ImplDefinitionTemplataT`, `ExternFunctionTemplataT`
- `HinputsT<'s, 't>` (pass output)
- **Environments:** 9 concrete variants, plus `GlobalEnvironmentT<'s, 't>` (one per pass). `TemplatasStoreT<'s, 't>` is held inline inside envs (arena-allocated, non-Copy — uses `ArenaIndexMap`).

**Inline Copy, NOT interned (Scala-verbatim structural equality):**
- Name sub-enum families (22 of them): each is a 16-byte inline Copy value (tag + 8-byte concrete ref).
- Kind wrapper enums: `KindT<'s, 't>`, `ICitizenTT<'s, 't>`, `ISubKindTT<'s, 't>`, `ISuperKindTT<'s, 't>` — same pattern. Non-primitive variants hold `&'t StructTT` etc.; primitive variants (`Never`, `Void`, `Int`, `Bool`, `Str`, `Float`) hold tiny Copy payloads inline.
- `ITemplataT<'s, 't>` — also inline wrapper. Variants mix `&'t` refs to interned templata payloads with inline Copy-value variants (`Integer(i64)`, `Boolean(bool)`, `Mutability(MutabilityTemplataT)`, etc.).
- `CoordT<'s, 't>` — passed by value, `kind: KindT<'s, 't>` inline.
- `OwnershipT`, `MutabilityT`, `VariabilityT`, `LocationT`, `RegionT` — pure Copy enums.
- Small templata value variants: `MutabilityTemplataT`, `VariabilityTemplataT`, `OwnershipTemplataT`, `RuntimeSizedArrayTemplateTemplataT`, `StaticSizedArrayTemplateTemplataT`.
- Env wrapper enums: `IEnvironmentT<'s, 't>` (9 variants, each holding `&'t FooEnvironmentT`), `IInDenizenEnvironmentT<'s, 't>` (6-variant subset), `IEnvEntryT<'s, 't>` (5 variants), `IVariableT<'s, 't>` (4 concrete-by-value variants), `ILocalVariableT<'s, 't>` (2-variant subset).

**Casting identity rule.** Wrapper enums compare structurally on their 16 bytes (tag + inner ref). Concrete payloads and interned templata payloads compare via `ptr::eq` on the `&'t` ref. Casting up a sub-enum hierarchy (concrete → sub-enum → super-sub-enum → widest) is a stack-only rewrap via `From`/`TryFrom` impls; no interner involvement.

**Neither arena (stack / heap-Vec / HashMap):**
- `CompilerOutputs<'s, 't>` — stack-owned accumulator, dies at pass end
- `Compiler<'s, 'ctx, 't>` — stack god struct
- Env builders — stack-local with heap `Vec`s / `HashMap`s until `build_in(&TypingInterner<'t>)` freezes into `'t`.
- `DeferredActionT` entries in `VecDeque` — owned structs, not `Box<dyn>`

### 1.6 Mutual-Recursion Shape

The interned type graph is mutually recursive but every edge is an `&'s` or `&'t` ref — pointer-sized, finite. No `Box`/`Vec` needed in arena types. `CoordT → KindT → StructTT → IdT → INameT → ITemplataT → CoordT`. Same-type self-recursion (e.g. `ForwarderFunctionNameT.inner: IFunctionNameT<'s, 't>`) is resolved by inline 16-byte sub-enum values — no `Box`.

---

## Part 2: The God Struct

### 2.1 Architecture

All Scala sub-compilers (FunctionCompiler, ExpressionCompiler, StructCompiler, ImplCompiler, OverloadResolver, InferCompiler, TemplataCompiler, ConvertHelper, DestructorCompiler, VirtualCompiler, SequenceCompiler, ArrayCompiler, EdgeCompiler, BlockCompiler, PatternCompiler, CallCompiler, LocalHelper, …) collapse into a single `Compiler` struct:

```
pub struct Compiler<'s, 'ctx, 't> {
    pub scout_arena: &'ctx ScoutArena<'s>,
    pub typing_interner: &'ctx TypingInterner<'s, 't>,
    pub keywords: &'ctx Keywords<'s>,  // scout-interned keywords are fine
    pub opts: &'ctx TypingPassOptions<'s>,
}
```

Immutable configuration only. Mutable state threads through `&mut CompilerOutputs<'s, 't>` on every call.

**Not on the god struct:**
- `global_env` — Scala builds `globalEnv` as a local inside `compile()`, not as a constructor arg. We do the same: the top-level env is a method parameter on `compile_program` (and anything downstream that needs it), not a field.
- `name_translator` — Scala's `NameTranslator` is a pure helper class with no state; its methods translate postparser names through the interner. In Rust, every `Compiler` method already has `self.scout_arena` and `self.typing_interner`, so the helper adds nothing. Its ~6 translate methods move directly onto `impl Compiler` and the struct is deleted.

### 2.2 `&self` + `&mut coutputs` Enables Re-entrancy

Deep mutual recursion (e.g. `evaluate_expression → evaluate_prefix_call → find_function → evaluate_function_body → evaluate_expression`) works because `&self` allows shared borrow, and `&mut coutputs` is re-borrowed at each call level. With `&mut self`, this would require two simultaneous mutable borrows of the god struct.

### 2.3 Macros

Macros (AsSubtypeMacro, LockWeakMacro, StructDropMacro, etc.) were stateful classes in Scala (holding `keywords`, `expressionCompiler`, etc.). In the god-struct refactor, all that state is already on `Compiler`. So macro structs disappear entirely and their methods become ordinary methods on `Compiler`.

Dispatch is via a small Copy unit-variant enum per Scala macro trait — `FunctionBodyMacro`, `OnStructDefinedMacro`, `OnInterfaceDefinedMacro`, `OnImplDefinedMacro`. Each variant tags which `Compiler` method to call. A given macro may appear in multiple enums iff Scala had it extending multiple traits (e.g. `StructDropMacro extends IFunctionBodyMacro with IOnStructDefinedMacro`).

### 2.4 Delegate Traits Eliminated

In Scala, sub-compilers were wired via anonymous delegate traits (IExpressionCompilerDelegate, IFunctionCompilerDelegate, IInfererDelegate, etc.). With the god struct, these are unnecessary — every method calls `self.method(...)` directly.

### 2.5 Method vs Free Function Under The Collapse

When porting a Scala `def`, decide method-on-`Compiler` vs free function by inspecting two things: what the **Scala body uses**, and whether the Rust port has to be **stored in a closure**.

- **Scala body uses no `this`** (no field reads, no method calls on `this` — pure on its arguments) → may become a Rust free function. *Must* become one when the function is stored in a `Box<dyn Fn>` whose `'static` default would otherwise force a lifetime workaround. Mirror the function name and file location; only the receiver drops. SPDMX exception Q codifies this. Example: `getPuzzles` in the typing-pass solver, `get_puzzles_rune_type` and `get_runes_rune_type` in the postparser solvers.
- **Scala body uses `this`** (reads fields, calls methods on the same class) → must stay a method on `Compiler`, even if the Scala class itself disappears (collapsed by §2.1) or had its methods forwarded through a `delegate: ISomethingDelegate` parameter. The delegate parameter disappears in Rust because §2.4 puts everything on `Compiler`. Example: `solveRule` (was `private def` on `object CompilerRuleSolver`, takes `delegate`; in Rust it's `Compiler::solve_rule(&self, ...)`).

**Common mistake.** Modeling a typing-pass piece after a postparser solver without checking the body. The postparser solvers (`identifiability_solver.rs`, `rune_type_solver.rs`) have free-function `solve_rule_impl` because they have no `Compiler` and no delegate. Typing-pass code touches the delegate methods constantly — those have to be `&self` methods on `Compiler`. Don't generalize the postparser's free-function shape to the typing pass.

---

## Part 3: Environments

### 3.1 Arena-Allocated In `'t`

Environments are allocated directly into the typing arena `'t`. They reference scout data (FunctionA, StructA, INameS) freely via `&'s` fields and reference interned typing-pass data (IdT, ITemplataT, KindT payloads) via the inline wrappers + `&'t` refs.

**Why `'t`, not `'s`**: envs hold TemplatasStoreT which stores `IEnvEntryT::Templata(ITemplataT<'s, 't>)`, which transitively holds `&'t` refs to interned typing-pass payloads. A struct with lifetime `'s` can only hold `&'x` references where `'x: 's`. Since `'s: 't`, `'t: 's` is false — so `'s`-allocated envs cannot hold `&'t` refs. Envs must live in `'t`. The lifetime ordering is still fine: `'t` outlives the typing pass; envs die together with all other typing-pass output when the typing arena drops.

Two parallel wrapper enums. Both hold `&'t` refs to the same concrete payloads, so casting between them is stack-only rewraps (no interner involvement). `IInDenizenEnvironmentT` is a 6-variant subset of `IEnvironmentT`'s 9 — the envs that represent "a denizen currently being compiled."

```
pub enum IEnvironmentT<'s, 't> {       // 9 variants — Package/Citizen/Function/Node/
    Package(&'t PackageEnvironmentT<'s, 't>),                  // BuildingWithClosureds/
    Citizen(&'t CitizenEnvironmentT<'s, 't>),                  // BuildingWithClosuredsAndTemplateArgs/
    Function(&'t FunctionEnvironmentT<'s, 't>),                // General/Export/Extern.
    Node(&'t NodeEnvironmentT<'s, 't>),
    BuildingWithClosureds(&'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>),
    BuildingWithClosuredsAndTemplateArgs(&'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>),
    General(&'t GeneralEnvironmentT<'s, 't>),
    Export(&'t ExportEnvironmentT<'s, 't>),
    Extern(&'t ExternEnvironmentT<'s, 't>),
}

pub enum IInDenizenEnvironmentT<'s, 't> {  // 6-variant subset (no Package/Export/Extern).
    Citizen, Function, Node, BuildingWithClosureds, BuildingWithClosuredsAndTemplateArgs, General
}
```

Every env variant carries `global_env: &'t GlobalEnvironmentT<'s, 't>` as a back-ref (Scala parity; Scala's `IEnvironmentT` has `def globalEnv`). `NodeEnvironmentT` omits it because it delegates via `parent_function_env.global_env` (matching Scala's `override def globalEnv = parentFunctionEnv.globalEnv`).

`IEnvEntryT<'s, 't>` is a 5-variant inline Copy enum: `Function(&'s FunctionA<'s>)`, `Struct(&'s StructA<'s>)`, `Interface(&'s InterfaceA<'s>)`, `Impl(&'s ImplA<'s>)`, `Templata(ITemplataT<'s, 't>)`. Not interned. Lives inline in `TemplatasStoreT.name_to_entry`.

Env Hash/PartialEq/Eq are **manual impls**, not derived — they match Scala's `id`-based identity (or `(id, life)` for `NodeEnvironmentT`, `panic!("vcurious")` for `GeneralEnvironmentT`). Deriving would walk into `TemplatasStoreT`'s maps and diverge from Scala.

### 3.2 `TemplatasStoreT` Uses `ArenaIndexMap`

For 1:1 Scala parity, `TemplatasStoreT` uses `ArenaIndexMap` (the AASSNCMCX-blessed arena-backed hash map) to mirror Scala's `Map[INameT, IEnvEntry]` and `Map[IImpreciseNameS, Vector[IEnvEntry]]`:

```
pub struct TemplatasStoreT<'s, 't> {
    pub templatas_store_name: &'t IdT<'s, 't>,
    pub name_to_entry: ArenaIndexMap<'t, INameT<'s, 't>, IEnvEntryT<'s, 't>>,
    pub imprecise_to_entries: ArenaIndexMap<'t, &'s IImpreciseNameS<'s>, &'t [IEnvEntryT<'s, 't>]>,
}
```

- **`name_to_entry`** — `ArenaIndexMap` keyed by `INameT`. Each Scala `entriesByNameT.get(name)` translates to `name_to_entry.get(&name)`.
- **`imprecise_to_entries`** — `ArenaIndexMap` keyed by `&'s IImpreciseNameS`. Values are `&'t [IEnvEntryT]` slices (the per-imprecise-name overload buckets), matching Scala's `Map[IImpreciseNameS, Vector[IEnvEntry]]`.

`ArenaIndexMap` is not `Copy`/`Clone`, so `TemplatasStoreT` is `/// Arena-allocated` (not value-type). Lives inline in env structs (~80 bytes: two `ArenaIndexMap`s + the `&'t IdT` back-ref).

> **Future exploration:** Once body migration is complete and benchmarks exist, profile lookup-heavy paths (overload resolution, name-imprecise lookups during scout-name-to-typing-pass-name resolution). For env kinds where scope sizes are consistently small (block-local, function-local, node) and lookups are frequent, evaluate switching back to unsorted slice-of-pairs with linear scan on a per-env-kind basis.

### 3.3 Mutable Building Phase

During construction, an env is mutable. Builders live on the stack with heap `Vec`s (and heap `HashMap`s for the imprecise-name index); freeze into the typing arena when done. No `&mut FooEnvironmentT` over arena-allocated envs — once a `&'t FooEnvironmentT` is created, it's immutable. Child scopes and mutations produce a fresh builder → fresh arena allocation → fresh `&'t` ref (matches Scala's `NodeEnvironmentBox`, which also allocates a new env per mutation).

`TemplatasStoreBuilder<'s, 't>` is its own stack builder with `Vec<(INameT, IEnvEntryT)>` for the name-to-entry mapping and `HashMap<&'s IImpreciseNameS<'s>, Vec<IEnvEntryT<'s, 't>>>` for the imprecise index (heap during construction, frozen to `ArenaIndexMap` on `build_in`).

Child-scope API: each env has a `make_child(...)` returning a fresh builder (not a `&mut` over arena-allocated data). `NodeEnvironmentBox`, `FunctionEnvironmentBoxT`, `IDenizenEnvironmentBoxT` (Scala's mutable wrappers and trait) are deleted in Rust — the builder-freeze pattern subsumes them.

### 3.4 Transient Reads Use `&IEnvironmentT`

Scala's `NodeEnvironmentBox.snapshot` is called ~36 times in ExpressionCompiler.scala, mostly for transient reads. In Rust:

- **`&IEnvironmentT<'_, 's, 't>`** (elided lifetime) — read-only borrow of the inline wrapper enum. Zero cost, for helpers that only inspect.
- **`&'t IEnvironmentT<'s, 't>`** or **`IEnvironmentT<'s, 't>` by value** — arena-pinned, needed when storing into a parent pointer, output, or a heavy templata.

Promotion to `&'t` happens only at explicit "store this env" points (via `build_in(interner)` on a builder). Transient reads just use `&`. Since `IEnvironmentT` is itself a Copy wrapper (16 bytes), callers can also pass it by value cheaply.

### 3.5 `FunctionTemplata` Is A Plain Computed Value

Matches Scala directly: a method on `FunctionEnvironmentT` allocates a fresh `FunctionTemplataT` into `'t` per call. No caching, no `OnceCell`, no ID minting, no side-table insertion — `FunctionTemplataT` is a small struct holding two pointers, trivially cheap to construct on every call. It's arena-allocated into `'t` but not interned.

### 3.6 Env-ID Uniqueness Not Required

Environments don't need unique `id` fields per instance. No HashMap keys on env's own id anywhere in the design:
- `CompilerOutputs.function_name_to_outer_env` etc. are keyed by **function/type template ids** (genuinely unique per Scala's `vassert`).
- OverloadSet holds the env by direct reference, not by id.
- Heavy templatas hold refs, not ids.

Scala's env-id collisions (NodeEnvironment delegating to parent, GeneralEnvironment reusing caller ids, Box wrappers sharing ids) translate as-is.

### 3.7 `GlobalEnvironmentT`

One instance per typing pass, allocated in `'t` early. Every env carries `global_env: &'t GlobalEnvironmentT<'s, 't>` as a back-ref. Holds `name_to_top_level_environment` (slice of `(IdT, TemplatasStoreT)` pairs) and `builtins: TemplatasStoreT`. Scala's macro fields (`nameToFunctionBodyMacro`, etc.) are **dropped** — macros are methods on `Compiler` now (Part 2.3). The struct carries only data.

### 3.8 Why Not Two-Tier Per-Denizen Arenas (Yet)

The migration-phase design uses one `'t` typing arena. A post-migration redesign splits into a program-wide `'out` outputs arena and per-top-level-denizen `'scratch` arenas. Full write-up: `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md`. Not in scope for this migration.

---

## Part 4: CompilerOutputs

### 4.1 Structure

Mutable accumulator threaded through the pass. Carries `<'s, 't>`. 23 fields:

- **Function registries.** `return_types_by_signature: HashMap<PtrKey<'t, SignatureT>, CoordT>`; `signature_to_function: HashMap<PtrKey<'t, SignatureT>, &'t FunctionDefinitionT>`.
- **Declaration tracking.** `function_declared_names: HashMap<PtrKey<'t, IdT>, RangeS>`; `type_declared_names: HashSet<PtrKey<'t, IdT>>`.
- **Env back-refs (keyed by function/type template id).** `function_name_to_outer_env`, `function_name_to_inner_env`, `type_name_to_outer_env`, `type_name_to_inner_env` — all `HashMap<PtrKey<'t, IdT>, &'t IInDenizenEnvironmentT<'s, 't>>`. Narrower in-denizen wrapper, matching Scala.
- **Type metadata.** `type_name_to_mutability: HashMap<PtrKey<'t, IdT>, ITemplataT>`; `interface_name_to_sealed: HashMap<PtrKey<'t, IdT>, bool>`.
- **Definitions.** `struct_template_name_to_definition`, `interface_template_name_to_definition` — `HashMap<PtrKey<'t, IdT>, &'t {Struct|Interface}DefinitionT>`.
- **Impls + reverse indexes.** `all_impls: HashMap<PtrKey<'t, IdT>, &'t ImplT>`; `sub_citizen_template_to_impls`, `super_interface_template_to_impls` — `HashMap<PtrKey<'t, IdT>, Vec<&'t ImplT>>` (the two `Vec`-valued exceptions to §4.3, mutated as new impls are discovered; OK because `CompilerOutputs` is stack-owned, not arena-allocated).
- **Exports/externs.** `kind_exports`, `function_exports`, `kind_externs`, `function_externs` — `Vec<&'t {Kind|Function}{Export|Extern}T>`.
- **Instantiation bounds.** `instantiation_name_to_bounds: HashMap<PtrKey<'t, IdT>, &'t InstantiationBoundArgumentsT>`.
- **Deferred queues.** `deferred_actions: VecDeque<DeferredActionT>`; `finished_deferred_function_body_compiles: HashSet<PtrKey<'t, PrototypeT>>`; `finished_deferred_function_compiles: HashSet<PtrKey<'t, IdT>>`.

No origin side tables — heavy templatas and OverloadSets hold scout refs directly.

### 4.2 `PtrKey<'t, T>` Newtype For HashMap Keys

Interned `&'t T` refs hash by pointer identity, not structural equality. Newtype wrapper gives the custom Hash/Eq. Lives in `src/typing/ptr_key.rs`:

- `PartialEq` uses `std::ptr::eq`.
- `Hash` casts the inner `&'t T` to `*const ()` and hashes that.
- Manual `Copy`/`Clone` (not `#[derive]`) so they work for any `T: ?Sized` without requiring `T: Copy`. The `?Sized` bound is forward-compat for `dyn` / slice keys.

### 4.3 Side-Table Access Pattern — Copy Out Before &mut

The env back-refs need the copy-out pattern to satisfy the borrow checker:

```
// Does not compile:
let env = coutputs.function_name_to_outer_env.get(&PtrKey(id)).unwrap();
self.do_stuff(coutputs, env, ...);  // &mut coutputs rejected; env borrows from it

// Works — copy out first:
let env_t: &'t IInDenizenEnvironmentT<'s, 't> =
    *coutputs.function_name_to_outer_env.get(&PtrKey(id)).unwrap();
self.do_stuff(coutputs, env_t, ...);  // OK; env_t is Copy'd out
```

**Invariant: most HashMap values in `CompilerOutputs` are pointer-sized Copy refs** (`&'s T`, `&'t T`) — enables the copy-out-then-mutate pattern.

**Two exceptions:** `sub_citizen_template_to_impls` and `super_interface_template_to_impls` hold `Vec<&'t ImplT>` because the reverse-index lookups return multiple impls per key. Access these with `mem::take` / `drain` + reinsert, not by-reference read.

### 4.4 Deferred Actions

Avoid `Box<dyn FnOnce>` via structured enum:

```
pub enum DeferredActionT<'s, 't> {
    EvaluateFunctionBody {
        prototype: &'t PrototypeT<'s, 't>,
        function_env: &'t FunctionEnvironmentT<'s, 't>,
        origin: &'s FunctionA<'s>,
    },
    EvaluateFunction {
        name: &'t IdT<'s, 't>,
        calling_env: &'t IInDenizenEnvironmentT<'s, 't>,
        origin: &'s FunctionA<'s>,
        template_args: &'t [ITemplataT<'s, 't>],
    },
    // Add variants as needed.
}
```

`function_env` on `EvaluateFunctionBody` is `&'t FunctionEnvironmentT` (the concrete env type, matches Scala). `calling_env` on `EvaluateFunction` is `&'t IInDenizenEnvironmentT` (the narrow wrapper, also matches Scala).

Drain loop: `pop_front()` → match → call `Compiler` method with `&mut coutputs`. No self-borrow because once the action is popped, it doesn't alias anything inside `coutputs`. No `Box`, no `dyn`, no hidden captures.

**Invariant:** `DeferredActionT` variants hold only owned or `Copy` context (`&'s` and `&'t` refs, small values). They never reference `CompilerOutputs` itself.

### 4.5 Speculative Writes Are Idempotent

During overload resolution, `attempt_candidate_banner` writes to `coutputs` even for candidates that may later be rejected. Safe because writes are idempotent — re-writing asserts equality. No rollback machinery.

---

## Part 5: OverloadSet

### 5.1 Transient Kind, But Holds Env Directly

- Overload resolution always completes during the typing pass; nothing unresolved reaches the instantiator.
- But an `OverloadSet`-kinded `ReinterpretTE` survives in output as an inert type tag (the instantiator elides it via `InstantiationBoundArgumentsT.runeToFunctionBoundArg`).

Since `'s` and `'t` live through instantiation, we hold the env directly — no `ctx_id` indirection needed:

```
pub struct OverloadSetT<'s, 't> {
    pub env: &'t IInDenizenEnvironmentT<'s, 't>,
    pub name: &'s IImpreciseNameS<'s>,  // scout-interned, fine to hold
}

pub enum KindT<'s, 't> {
    // ... other variants ...
    OverloadSet(&'t OverloadSetT<'s, 't>),
}
```

### 5.2 Construction

A method on Compiler interns an `OverloadSetT { env, name }` into a `KindT::OverloadSet`. No side-table insertion, no `ctx_id`, no `&mut coutputs` required.

### 5.3 Overload Resolution Uses The Env Directly

The resolver reads `overload_set.env` directly, walks its parent chain, looks up candidates, recurses into nested OverloadSets via their own `env` fields. Matches Scala's `OverloadResolver.scala:210` pattern verbatim.

### 5.4 Post-Pass

`OverloadSetT<'s, 't>` survives in `HinputsT<'s, 't>` inside inert `ReinterpretTE` args. Its env and name refs remain valid for as long as `'t` and `'s` live. The instantiator elides these on sight.

---

## Part 6: Type System Types

Every lifetime-parameterized struct in this section has an implicit `where 's: 't` bound (§1.2).

### 6.0 Phantom-`+T`-Erasure Principle

**Any Scala `+T <: SomeTrait` type parameter is erased to monomorphic widest-form in Rust.** The Rust port carries no leaf-type generic parameter; callers pattern-match at the use site to narrow, just as Scala does at runtime (`+T` is JVM-erased anyway). This applies uniformly to:

- `IdT[+T <: INameT]` → `IdT<'s, 't>` with `local_name: INameT<'s, 't>` (§6.3)
- `PrototypeT` (no generic in Rust; monomorphic)
- `SignatureT` (no generic in Rust; monomorphic)
- `ITemplataT[+T <: ITemplataType]` → `ITemplataT<'s, 't>` (§6.6)
- `PlaceholderTemplataT[+T <: ITemplataType]` → `PlaceholderTemplataT<'s, 't>` with `tyype: ITemplataType<'s>` (the widest existing postparser enum, *not* `ITemplataT` — the field holds a type *descriptor*, not a templata value)
- `PrototypeTemplataT[T <: IFunctionNameT]` → `PrototypeTemplataT<'s, 't>` (no inner generic; just holds `&'t PrototypeT`)

Rationale + alternatives are recorded in `FrontendRust/docs/reasoning/idt-typed-view-alternatives.md`.

**Erasure rule: widen the field to match the bound, not the enclosing trait.** `IdT[+T <: INameT]` has `local_name: T` — the bound is `INameT`, so the erased field is `INameT<'s, 't>`. `PlaceholderTemplataT[+T <: ITemplataType]` has `tyype: T` — the bound is `ITemplataType`, so the erased field is `ITemplataType<'s>`. A common mistake is widening to `ITemplataT<'s, 't>` (the *value* enum), but `ITemplataType` and `ITemplataT` are different type families: `ITemplataType<'s>` is a postparser-defined type *descriptor* ("what kind of templata?" — e.g. `IntegerTemplataType`, `MutabilityTemplataType`), while `ITemplataT<'s, 't>` is an actual templata *value* ("what does this templata hold?" — e.g. `Integer(42)`, `Kind(&'t StructTT)`). `ITemplataType` is `'s`-only because it never holds typing-pass data.

**No `'t`-lifetime re-interned versions of `StrI`, `IImpreciseNameS`, `RangeS`, `CodeLocationS`, `PackageCoordinate`, `FileCoordinate`.** Use the scout-arena versions directly. Same for `ITemplataType<'s>` — the existing postparsing enum is reused unchanged from typing-pass code.

### 6.1 IDEPFL Dual-Enum Pattern

All interned typing types use the dual-enum pattern: a **reference enum** (canonical, `&'t` refs) and a **value enum** (transient, HashMap lookup). Scout-lifetimed values (StrI, IImpreciseNameS, RangeS, etc.) are used directly wherever they appear — no re-interning boundary.

Interned type families (each gets its own HashMap dedup + optional `*ValT` lookup key per IDEPFL):
- Concrete name structs (~60). 15 have `&'t [...]` slices and get a transient `*ValT<'s, 't, 'tmp>`; the rest reuse the struct as its own Val.
- IdT / IdValT — monomorphic (see §6.3).
- Concrete Kind payloads — all simple/shallow, reuse struct as Val.
- Interned templata payloads — simple/shallow, reuse struct as Val. CoordListTemplataT has an arena slice so it gets `CoordListTemplataValT<'s, 't, 'tmp>`.
- PrototypeT/PrototypeValT, SignatureT/SignatureValT — monomorphic; both contain IdT so Val needs `'tmp` for the nested IdValT's slice.

**Wrapper enums are NOT interned.** INameT + 21 name sub-enums, KindT + 3 Kind sub-enums (ICitizenTT/ISubKindTT/ISuperKindTT), and ITemplataT are all 16-byte inline Copy values. Sub-enum casts between narrow and wide forms are stack-only rewraps via From/TryFrom.

**Val types use content-based Hash, not ptr-based.** `*ValT` types (the interner lookup keys) use derived Hash/PartialEq/Eq (content-based) so query-Val (`'tmp`) and stored-Val (`'t`) hash consistently. The `ptr::eq` treatment stays for the *canonical `&'t` refs*, where pointer identity is the intent.

### 6.2 INameT Hierarchy

~60 concrete name types, ~21 sub-trait enums. Every name type carries `<'s, 't>`.

**DAG rule (critical).** Scala's sealed-trait hierarchy is a DAG — some concrete names extend multiple sub-traits (`ExternFunctionNameT extends IFunctionNameT with IFunctionTemplateNameT`, etc.). In Rust this becomes: **a shared concrete name gets a variant in each sub-enum it belongs to.** The same `&'t ExternFunctionNameT` can appear as `IFunctionNameT::ExternFunction(...)` and `IFunctionTemplateNameT::ExternFunction(...)`.

**Sub-enums are inline-owned Copy values, NOT arena-interned.** Only concrete name types and INameT itself live in the typing arena. The intermediate sub-enums are 16-byte derived-Copy values built on the stack when needed. Casting is a pure stack-only rewrap via `.into()` — no interner round-trip.

Identity: two concrete names compare via `ptr::eq`. Two owned sub-enum values compare structurally — but since the tag + inner pointer is 16 bytes, this is a 2-word compare and still cheap.

Bridging via `From<X> for SubEnum` (narrow → wide, infallible) and `TryFrom<INameT<'s, 't>> for SubEnum` (wide → narrow, fallible, match-and-rewrap on the stack). Real implementations in `names.rs`, no interner involvement.

**Self-referential variants** (e.g. `ForwarderFunctionNameT.inner: IFunctionNameT<'s, 't>`) are inline-owned; since sub-enums are 16 bytes and Copy they can live directly as struct fields without `Box` or `&'t`.

### 6.3 `IdT<'s, 't>` — Monomorphic, Interned

```
pub struct IdT<'s, 't>
where 's: 't,
{
    pub package_coord: &'s PackageCoordinate<'s>,  // scout-lifetimed
    pub init_steps: &'t [INameT<'s, 't>],          // inline INameT values
    pub local_name: INameT<'s, 't>,                // always the widest form
}
```

Per §6.0, Scala's `IdT[+T <: INameT]` phantom outer parameter is **erased**. Callers that need a specific leaf-name variant pattern-match on `local_name` at the point of use.

IdT defines custom PartialEq/Eq/Hash (not derive):
- `package_coord`: pointer-eq (scout arena canonicalizes PackageCoordinate).
- `init_steps`: slice data pointer + length compare — the typing interner canonicalizes `&'t [INameT]` slices as a whole per IDEPFL, so equal content ⇒ equal slice pointer.
- `local_name`: structural compare on the inline INameT (16 bytes).

Interning uses one HashMap per IDEPFL keyed by `IdValT<'s, 't, 'tmp>` (transient, `'tmp`-borrowed init_steps slice). No widen/try_narrow methods — pattern-match instead.

### 6.4 `CoordT<'s, 't>` — Inline Copy

`CoordT` holds `ownership: OwnershipT`, `region: RegionT`, `kind: KindT<'s, 't>`. Small (~24 bytes), Copy, passed by value. Not interned — structural eq.

### 6.5 `KindT<'s, 't>` — Inline Wrapper

```
pub enum KindT<'s, 't> {
    // Primitives inline (small)
    Never(NeverT), Void(VoidT), Int(IntT), Bool(BoolT), Str(StrT), Float(FloatT),
    // Non-primitives — &'t refs to interned payloads
    Struct(&'t StructTT<'s, 't>),
    Interface(&'t InterfaceTT<'s, 't>),
    StaticSizedArray(&'t StaticSizedArrayTT<'s, 't>),
    RuntimeSizedArray(&'t RuntimeSizedArrayTT<'s, 't>),
    KindPlaceholder(&'t KindPlaceholderT<'s, 't>),
    OverloadSet(&'t OverloadSetT<'s, 't>),
}
```

KindT is **inline-owned** (not arena-interned): 16 bytes tag + ref, Copy. Concrete payloads (StructTT etc.) are arena-interned; the wrapper just tags them.

Same philosophy for the three Kind sub-enum families (ICitizenTT, ISubKindTT, ISuperKindTT). Casts between them are stack-only rewraps via From/TryFrom.

### 6.6 `ITemplataT<'s, 't>` — Inline Wrapper, Phantom Parameters Erased

Per §6.0, Scala's `ITemplataT[+T <: ITemplataType]` outer phantom type parameter is **erased** — the wrapper is a monomorphic `ITemplataT<'s, 't>` enum, and callers that need a specific kind pattern-match on the variant.

Variants split into three groups:
- **Interned-payload variants** (`&'t` ref to arena-allocated payload): Coord, Kind, Placeholder, Prototype, Isa, CoordList.
- **Inline Copy-value variants**: Mutability, Variability, Ownership, Integer(i64), Boolean(bool), String(StrI<'s>) — *scout-lifetimed, not re-interned*, RuntimeSizedArrayTemplate, StaticSizedArrayTemplate.
- **Heavy templatas** (`&'t` to env-ref-holding payloads, not interned): Function, StructDefinition, InterfaceDefinition, ImplDefinition, ExternFunction.

ITemplataT itself is **inline-owned**, not arena-interned.

Heavy templata payloads hold `&'t` env refs and `&'s` scout refs:

```
pub struct FunctionTemplataT<'s, 't> {
    pub outer_env: &'t IInDenizenEnvironmentT<'s, 't>,
    pub function: &'s FunctionA<'s>,
}
// StructDefinitionTemplataT / InterfaceDefinitionTemplataT / ImplDefinitionTemplataT
// each hold `&'t IInDenizenEnvironmentT` and one of `&'s StructA` / `&'s InterfaceA` / `&'s ImplA`.
pub struct ExternFunctionTemplataT<'s, 't> {
    pub header: &'t FunctionHeaderT<'s, 't>,
}
```

Heavy-templata Eq/Hash is via `std::ptr::eq` on the scout refs (the scout arena canonicalizes those).

`PlaceholderTemplataT.tyype: ITemplataType<'s>` (the widest postparser enum, *not* ITemplataT). The Scala field is `tyype: T` where `T <: ITemplataType` — a *type descriptor*, not a templata value.

`PrototypeTemplataT`'s Scala inner type parameter is also erased (§6.0) — just holds `&'t PrototypeT<'s, 't>`.

### 6.7 Mutual Recursion

Types form a mutually recursive graph (`CoordT → KindT → StructTT → IdT → INameT → ITemplataT → CoordT`). Most links are `&'s` or `&'t` refs — pointer-sized, finite. The name sub-enums are the exception — inline 16-byte tagged pointers, one word larger than a raw ref but still finite and Copy. No `Box`/`Vec` needed.

---

## Part 7: Expression AST

### 7.1 Three Enums

- `ReferenceExpressionTE<'s, 't>` — 48 variants (LetNormal, If, While, FunctionCall, Consecutor, Construct, Reinterpret, …).
- `AddressExpressionTE<'s, 't>` — 5 variants (LocalLookup, ReferenceMemberLookup, …).
- `ExpressionTE<'s, 't>` — 2-variant wrapper: `Reference(&'t ReferenceExpressionTE)` / `Address(&'t AddressExpressionTE)`.

**Narrow-use rule.** `ConstructTE` is the **only** expression node that holds the broad ExpressionTE wrapper. Every other slot uses the specific `&'t ReferenceExpressionTE` or `&'t AddressExpressionTE`.

### 7.2 Arena-Allocated, Not Interned

Expression nodes are allocated into `'t` but not deduped. Sub-expressions are `&'t` refs; collections are arena slices. No `*ValT` companions, no intern methods, no From/TryFrom bridges across the wrapper enums.

`IExpressionResultT<'s, 't>` is a 2-variant inline Copy enum (`Reference(ReferenceResultT) | Address(AddressResultT)`), both result structs hold a single CoordT.

### 7.3 Derives — `#[derive(PartialEq, Debug)]` Only

`ConstantFloatTE` holds `f64`, which can't derive Eq/Hash. The whole expression-AST family drops Eq/Hash uniformly. `IExpressionResultT` / `ReferenceResultT` / `AddressResultT` keep the full `Copy, Clone, PartialEq, Eq, Hash, Debug` — they only hold CoordT, no f64 transitively.

### 7.4 Visitor/Collector Pattern

Same as parsing/postparsing: `NodeRefT<'s, 't>` enum, `visit_*` functions, entry points, `collect_where_tnodes!`/`collect_only_tnodes!` macros.

---

## Part 8: Error Handling

### 8.1 `Result<T, CompileErrorT>` With `?`

Scala's `throw CompileErrorExceptionT(...)` → `return Err(CompileErrorT::...)` with `?` propagation. Single catch boundary at top-level `Compiler::compile()`.

`ICompileErrorT<'s, 't>` has 55 real variants (filled in Slab 14).

### 8.2 Panic For Unimplemented

`panic!()` with unique identifying messages for unmigrated branches. Fully-stub functions acceptable during incremental migration.

---

## Part 9: Keywords

`Keywords<'s>` — re-uses scout arena for interned strings. Same Keywords instance can serve the typing pass and subsequent passes as long as `'s` lives. No typing-specific `Keywords<'t>` needed.

---

## Part 10: Driving The Pass

### 10.1 Top-Level Entry

```
pub fn run_typing_pass<'s, 'ctx, 't>(
    scout_arena: &'ctx ScoutArena<'s>,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    keywords: &'ctx Keywords<'s>,
    opts: &'ctx TypingPassOptions<'s>,
    program_a: &'s ProgramA<'s>,
) -> Result<HinputsT<'s, 't>, ICompileErrorT<'s, 't>>
where 's: 't,
{
    let compiler = Compiler::new(scout_arena, typing_interner, keywords, opts);
    let mut coutputs = CompilerOutputs::new();

    compiler.compile_program(&mut coutputs, program_a)?;
    compiler.drain_all_deferred(&mut coutputs);

    let hinputs = HinputsT { /* materialized from coutputs */ };
    Ok(hinputs)
    // coutputs drops; its HashMaps (storing 's/'t refs) die cheaply
    // scout_arena and typing_interner survive; instantiator can use them
}
```

No `finalize()`. Natural scope exit.

### 10.2 Instantiator Handoff

The instantiator receives `HinputsT<'s, 't>` plus both arena references. When it's done, the caller lets local bindings drop in scope order (§1.3): typing interner first, scout arena second, parse arena last.

---

## Part 11: Invariants Summary

For quick review during implementation:

1. **`'s` outlives `'t`.** Declared via `where 's: 't` on every type that transitively holds `&'s` data. Rust does not enforce outlives via drop order; the bound must be written.
2. **Arena types never contain Vec, HashMap, String, Rc, Box.** AASSNCMCX applies. Use arena slices. (One pre-existing exception — `LocationInFunctionEnvironmentT.path: Vec<i32>` — is known debt.)
3. **All HashMap keys on interned refs use `PtrKey<'t, T>`** for pointer-based hash/eq.
4. **Most `CompilerOutputs` HashMap values are pointer-sized Copy refs** (`&'s T`, `&'t T`) — enables the copy-out-then-mutate pattern. Exceptions: `sub_citizen_template_to_impls` and `super_interface_template_to_impls` hold `Vec<&'t ImplT>`; access via `mem::take` / `drain` + reinsert.
5. **Speculative writes are idempotent.** No rollback machinery.
6. **Overload resolution always completes during the typing pass.** No unresolved overloads reach the instantiator.
7. **Env equality/hashing never used.** Envs keyed in CompilerOutputs by external (function/type template) ids, not their own id.
8. **Envs live in the typing arena `'t`.** They transitively hold `&'t` refs (via ITemplataT in IEnvEntryT), so they can't live in `'s`. They drop when `'t` drops.
9. **`DeferredActionT` variants never reference `CompilerOutputs`.** All captured context is owned or Copy. Preserves the drain pattern without self-borrow hazards.
10. **Arena parameters use a short borrow lifetime.** `&ScoutArena<'s>` or `&'ctx ScoutArena<'s>`, never `&'s ScoutArena<'s>`.
11. **Scala `+T <: SomeTrait` parameters erase to monomorphic widest-form.** No leaf-type generic parameter on the Rust port; pattern-match at use sites.
12. **Val types use content-based Hash/Eq.** Canonical `&'t` refs use `ptr::eq`. Don't mix the two.
13. **Heavy-templata env refs are `&'t`, payload (FunctionA / StructA / etc.) refs are `&'s`.** Don't put env refs in `&'s`.
14. **Never use `'static`.** All data must live in `'p`, `'s`, or `'t` (or on the stack). `'static` bypasses the arena system: a `&'static T` has a different pointer than a structurally identical `&'s T`, breaking pointer equality for interned types and creating a latent bug for any type that gets interned later. See `Luz/shields/NeverUseStaticLifetime-NUSLX.md`.
15. **Don't try to fix a `Box<dyn Fn> + &self` deferred-borrow with a lifetime parameter.** A boxed closure that captures `&self` keeps that shared borrow live for the full lifetime of the box, not just for closure construction. Holding the box across other `&self` calls on the same struct deadlocks the borrow checker — and threading a closure-lifetime parameter through the storage type (e.g. `'fn_lt` on `SimpleSolverState`) only sidesteps the `'static` default; it does not resolve the underlying conflict. The fix is always to drop the receiver: convert the captured method to a free function (per §2.5), or pass the data the closure needs by value into the closure body. Don't propose lifetime-threading as a fix without first checking whether the closure's call sites hold the box across other `&self` calls.

---

## Risks / Open Questions

- **LSP / long-running use**: scout arena retention through instantiation is memory-heavy; single-arena typing makes batch-only the safe mode. See `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md`.
- **Post-migration design revisits**: the inline-owned-wrapper philosophy makes casts free but gives up compile-time "this IdT's local_name is a FunctionName" assertions. See `FrontendRust/docs/reasoning/idt-typed-view-alternatives.md`.
- **TypingInterner perf**: six hashbrown HashMap maps with heterogeneous `'tmp`→`'t` lookup via Equivalent. Works today but hasn't been measured. Profile before optimizing.
- **Body migration ordering**: the 141 panic stubs have implicit dependency ordering — some bodies call other stubbed methods. The test-driven approach naturally discovers this, but expect some batches to require implementing a chain of 3-5 bodies before a test passes end-to-end.
- **Incremental compilation**: serializing HinputsT to disk requires a serialization boundary that breaks `'s` refs. Batch compilation only for now.
- **Parallelization**: single-threaded design (`!Sync` arenas, stack CompilerOutputs). Per-function parallelization is a later topic.
- **Typing storage → two-tier per-denizen arenas**: scheduled as a post-body-migration redesign. See `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md`.

---

## Key Files / Directories

| Path | Purpose |
|---|---|
| `tl-handoff.md` | operational handoff — process, principles, current work |
| `docs/architecture/typing-pass-design-v3.md` | this doc — architecture + design decisions |
| `docs/historical/slab-chronicle.md` | slab-by-slab history (Slabs 0–14b) |
| `FrontendRust/docs/migration/handoff-slab-*.md` | per-slab handoff docs (translation tables, gotchas) |
| `FrontendRust/docs/architecture/typing-pass-arenas.md` | typing-pass arena architecture |
| `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md` | two-tier per-denizen target + LSP direction |
| `FrontendRust/docs/reasoning/idt-typed-view-alternatives.md` | IdT monomorphic / typed-view decision |
| `FrontendRust/docs/reasoning/` | other design-decision docs |
| `FrontendRust/src/typing/names/names.rs` | ~3100 lines: all name types, IdT, From/TryFrom bridges, ValT companions |
| `FrontendRust/src/typing/types/types.rs` | KindT + sub-enums + concrete Kind payloads |
| `FrontendRust/src/typing/templata/templata.rs` | ITemplataT + payload structs |
| `FrontendRust/src/typing/ast/ast.rs` | PrototypeT, SignatureT, IdT-holding structs |
| `FrontendRust/src/typing/ast/expressions.rs` | ~2100 lines: 3 enums + 53 payload structs |
| `FrontendRust/src/typing/compiler_outputs.rs` | CompilerOutputs 23-field struct + 54 methods |
| `FrontendRust/src/typing/templata_compiler.rs` | all 49 method sigs; IBoundArgumentsSource enum + IPlaceholderSubstituter trait |
| `FrontendRust/src/typing/compiler_error_reporter.rs` | ICompileErrorT 55-variant enum |
| `FrontendRust/src/typing/typing_interner.rs` | 560 lines: 6-family HashMap design, ~84 wrappers |
| `FrontendRust/src/typing/compiler.rs` | god struct (Compiler, 4 fields) |
| `FrontendRust/src/typing/hinputs_t.rs` | HinputsT real-fielded with new() constructor |
| `FrontendRust/src/typing/compilation.rs` | TypingPassOptions, run_typing_pass entry point |
| `FrontendRust/src/typing/env/environment.rs` | 5 of 9 env types + wrapper enums + GlobalEnvironmentT + TemplatasStoreT |
| `FrontendRust/src/typing/env/function_environment_t.rs` | 4 of 9 env types + builders + variables |
| `FrontendRust/src/typing/env/i_env_entry.rs` | IEnvEntryT 5-variant enum |
| `FrontendRust/src/typing/test/` | 14 test files; 173 test bodies ready to drive body migration |
| `.claude/hooks/check-scala-comments` | pre-commit hook guarding `/* scala */` blocks |
