# Handoff: Typing Pass Slab 4 — Environments + Interner Bodies

**⚠️ HISTORICAL — SLAB 4 COMPLETE.** This doc was the prescriptive handoff written before Slab 4 shipped; it stays as the authoritative record of what Slab 4 was and isn't live spec anymore. Two corrections happened during execution and are now reflected in the committed code: (1) the interner uses **6 family-level HashMaps, not ~75 per-concrete maps** (Gotcha 2's sizing was wrong; see the family-dispatch skeleton for the right shape); (2) the `*Box*T` family of stubs to delete is broader than Gotcha 4 originally listed — it covers `NodeEnvironmentBox`, `FunctionEnvironmentBoxT`, and `IDenizenEnvironmentBoxT` (and any sibling that extends `IDenizenEnvironmentBoxT` in Scala). Tagged `slab-4-complete`. The Gotchas themselves (16 of them) are still a good template for future slab handoffs. Current arena architecture lives in `FrontendRust/docs/architecture/typing-pass-arenas.md`; long-term target in `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md`.

## Who this is for

You're picking up an in-progress Rust port of a Scala compiler frontend. A senior engineer has been moving through the typing pass one "slab" at a time per `quest.md` §12. Slabs 0–3 are done:

- **Slab 0** (arena substrate, `TypingInterner` stub): merged.
- **Slab 1** (leaf types: `OwnershipT`/`MutabilityT`/`VariabilityT`/`LocationT` enums, primitive `KindT` payloads): merged.
- **Slab 2** (name hierarchy: ~60 concrete names, 21 sub-enums, monomorphic `IdT<'s, 't>`, `From`/`TryFrom` bridges, IDEPFL `*ValT` companions). Tagged `slab-2-complete`.
- **Slab 3** (Kind / Coord / Templata trio: `KindT` inline wrapper, interned concrete Kind payloads, `ITemplataT` inline wrapper with mixed-mode equality, `PrototypeValT`/`SignatureValT` with `'tmp`). Tagged `slab-3-complete`.

You're doing **Slab 4** — the environment types (`src/typing/env/*.rs`), the first real `TypingInterner` implementation, `GlobalEnvironmentT`, and the variable types (`IVariableT` / `ILocalVariableT` / etc.) that envs hold. This is the biggest slab so far — structural work, not just data translation. Budget a full workday, possibly 1.5.

**Read these first in this order**, then come back:

1. `quest.md` — at least §§1.2, 1.5, 3 (all of Part 3), 6.4, 12.1-Slab-4 paragraph. **Note:** §3.1 places envs in the `'s` scout arena — override: envs go in the `'t` typing arena. See Gotcha 1 below for why.
2. `TL-HANDOFF.md` at the repo root — overrides section captures the `'t`-not-`'s` correction and Slab-3 state you need to respect.
3. `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md` — records the arena design you're building **and** the deferred post-migration design. Explains why Slab 4 is arena-based, what that costs, what future work is scheduled. The "Chosen (during migration)" section is the spec for this slab.
4. `.claude/rules/postparser/IDEPFL-postparser-interning.md` — the dual-enum value/reference pattern. The interner bodies you're writing use it (see Gotcha 2 for the live reference impl in `scout_arena.rs`).
5. `FrontendRust/docs/migration/handoff-slab-3.md` — sets conventions you'll reuse (inline-wrapper philosophy, `From`/`TryFrom` bridges, `/* scala */` block rules).
6. `FrontendRust/docs/reasoning/idt-typed-view-alternatives.md` — already-established: `IdT`, `PrototypeT`, `SignatureT` are monomorphic. Several Scala env types (`PackageEnvironmentT[+T]`, `CitizenEnvironmentT[+T,+Y]`, `GeneralEnvironmentT[+T]`, `BuildingFunctionEnvironmentWithClosuredsT.id: IdT[IFunctionTemplateNameT]`) are phantom-generic in Scala; erase those generics in Rust. Every Scala `IdT[…]` becomes the monomorphic `IdT<'s, 't>` on the Rust side; call sites pattern-match `id.local_name` to narrow.
7. This doc.

**Two senior-already-decided items to know up front** (don't re-litigate):

- **Envs live in `'t` arena, not `'s`.** See Gotcha 1. Quest.md §3.1 is wrong about this.
- **No `Rc`, no side table, no interior mutability.** Builder+freeze pattern per quest.md §3.3 applies. The deferred side-tables-of-`Rc` design lives in the reasoning doc above and is post-Slab-8 work. Slab 4 is intentionally arena-based.

You shouldn't need to read the Scala source externally — every Scala `case class` / sealed trait is already embedded inline in the `/* ... */` blocks in the env/ files. If the block is ambiguous, ask; don't guess.

## The big picture: why Slab 4 exists

Environments are the runtime store for name-and-type resolution during typing. When you write `let x = foo(5)` inside a function body, the compiler walks up the env chain looking for what `foo` resolves to — is it a function template? a local variable? an imported templata? The env chain has structure: a function body's `NodeEnvironmentT` inherits from a `FunctionEnvironmentT`, which inherits from a `CitizenEnvironmentT` or `PackageEnvironmentT`, etc.

In Scala, envs are plain case classes and a `NodeEnvironmentBox` mutable wrapper. In Rust we're using arena-allocated envs (`&'t FooEnvironmentT<'s, 't>`) with a builder-freeze pattern for mutation. This matches the storage philosophy of Slabs 1-3 (names, kinds, templatas) — everything lives in the typing arena, everything is referenced by `&'t`.

Slab 4 is bigger than Slab 2/3 because:

1. **Nine env variants**, each a real struct with 4–10 fields. Scala env code has more logic mixed with data than the name/kind/templata types did.
2. **`TemplatasStoreT`** is the first "map-like" arena structure — slice-of-pairs pattern per AASSNCMCX. You're working through the arena-slice idiom in earnest for the first time.
3. **Builder types** — each env kind that mutates during construction gets a matching `*Builder` stack struct with heap `Vec`s. You're introducing a second type family per env variant.
4. **`TypingInterner` gains real bodies.** Slabs 2-3 left `intern_id` / `intern_prototype` / `intern_signature` as `panic!()`. Slab 4 implements them (plus ~60 concrete-name intern methods and ~12 concrete payload intern methods for StructTT/InterfaceTT/KindTemplataT/etc.). Envs can't be constructed without real interning, so this is the first slab that lights up the interner.
5. **`GlobalEnvironmentT`** is a new type introduced here (not just an expand-stub-into-fields job). Every env holds `&'t GlobalEnvironmentT<'s, 't>` as a back-ref.
6. **Two parallel enums** — `IEnvironmentT` (9 variants) and `IInDenizenEnvironmentT` (6-variant subset). Both wrap the same `&'t FooEnvironmentT` payloads. You need `From`/`TryFrom` bridges between them.
7. **Variable types** (`IVariableT`, `ILocalVariableT`, `AddressibleLocalVariableT`, `ReferenceLocalVariableT`, `AddressibleClosureVariableT`, `ReferenceClosureVariableT`) need real definitions. Stubs exist in `env/function_environment_t.rs`; fill them.

By the end of Slab 4:

- `cargo check --lib` passes with 0 errors.
- `src/typing/env/environment.rs`: `IEnvironmentT`, `IInDenizenEnvironmentT`, `GlobalEnvironmentT`, `TemplatasStoreT`, `ILookupContext`, `PackageEnvironmentT`, `CitizenEnvironmentT`, `ExportEnvironmentT`, `ExternEnvironmentT`, `GeneralEnvironmentT` are all real structs/enums with Scala-parity fields + `From`/`TryFrom` bridges.
- `src/typing/env/function_environment_t.rs`: `BuildingFunctionEnvironmentWithClosuredsT`, `BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT`, `NodeEnvironmentT`, `FunctionEnvironmentT`, `IVariableT`, `ILocalVariableT`, `AddressibleLocalVariableT`, `ReferenceLocalVariableT`, `AddressibleClosureVariableT`, `ReferenceClosureVariableT` — real definitions. `NodeEnvironmentBox` and `FunctionEnvironmentBoxT` are deleted outright; see Gotcha 4.
- `src/typing/env/i_env_entry.rs`: `IEnvEntryT` is a real 5-variant inline Copy enum; the individual `FunctionEnvEntry` / `StructEnvEntry` / … stubs are deleted (they roll into enum variants).
- `src/typing/typing_interner.rs`: real bodies using `bumpalo::Bump` + `hashbrown::HashMap` + `RefCell<Inner>`. One intern method per interned family; the per-concrete-Kind and per-interned-templata-payload methods are added here. `intern_id` / `intern_prototype` / `intern_signature` work end-to-end.
- Per-env-variant **`*Builder`** stack struct with `build_in(self, interner: &TypingInterner<'s, 't>) -> &'t FooEnvironmentT<'s, 't>` freezing method. (`TypingInterner` exposes `alloc` / `alloc_slice_copy` wrappers around its internal `Bump` — same pattern as `ScoutArena` in `src/scout_arena.rs`.)

## What's already in place (don't duplicate; don't delete)

### `env/environment.rs` (704 lines, mostly Scala in `/* */` blocks)

Current Rust stubs:
- `IEnvironmentT<'s, 't>` — `_Phantom`-only enum with `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]`. Slab 3 added the derives. **Keep them.** Replace variants.
- `IInDenizenEnvironmentT<'s, 't>` — same `_Phantom`-only enum.
- `IDenizenEnvironmentBoxT<'s, 't>` trait (empty). **Delete** — the builder-freeze pattern subsumes it (see Gotcha 4).
- `ILookupContext` — already real 2-variant enum. Verify derives: add `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]`.
- `GlobalEnvironment<'s, 't>` — `PhantomData` struct. **Rename to `GlobalEnvironmentT<'s, 't>`** to match the `T` suffix convention of this slab. Fill with fields per Gotcha 5.
- `TemplatasStore<'s, 't>` — `PhantomData` struct. **Rename to `TemplatasStoreT<'s, 't>`** and fill per Gotcha 3.
- `PackageEnvironmentT<'s, 't>`, `CitizenEnvironmentT<'s, 't>`, `GeneralEnvironmentT<'s, 't>`, `ExportEnvironmentT<'s, 't>`, `ExternEnvironmentT<'s, 't>` — all `PhantomData` structs. Fill.
- Numerous free `fn entry_matches_filter() { panic!() }` / `fn entry_to_templata() { panic!() }` / `fn code_locations_match() { panic!() }` / `fn make_top_level_environment() { panic!() }` / `fn child_of() { panic!() }`. **Leave these alone** — they're slice-pipeline artifacts. Slab 8 body work will either wire them to `Compiler` methods or delete them. Don't touch now.

### `env/function_environment_t.rs` (895 lines)

Current stubs:
- `BuildingFunctionEnvironmentWithClosuredsT<'s, 't>`, `BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>`, `NodeEnvironmentT<'s, 't>`, `FunctionEnvironmentT<'s, 't>` — all `PhantomData`. Fill per Scala blocks.
- `NodeEnvironmentBox<'s, 't>` — `PhantomData`. **Delete** — see Gotcha 4.
- `FunctionEnvironmentBoxT<'s, 't>` — `PhantomData`. **Delete** — same rationale as `NodeEnvironmentBox`; Gotcha 4 covers the full `*Box*T` family.
- `IVariableT<'s, 't>`, `ILocalVariableT<'s, 't>` — `_Phantom`-only enums. Fill with DAG-appropriate variants.
- `AddressibleLocalVariableT<'s, 't>`, `ReferenceLocalVariableT<'s, 't>`, `AddressibleClosureVariableT<'s, 't>`, `ReferenceClosureVariableT<'s, 't>` — `PhantomData` structs. Fill.
- `lookup_with_name_inner` / `lookup_with_imprecise_name_inner` / `filter_regular_entries` / similar free fn stubs at the bottom. **Leave alone** — Slab 8.

### `env/i_env_entry.rs` (49 lines)

Current stubs:
- `IEnvEntry` — empty enum, **not prefixed `T`**. Rename to `IEnvEntryT<'s, 't>` and fill per Gotcha 6.
- Five individual structs (`FunctionEnvEntry<'s, 't>`, `ImplEnvEntry`, `StructEnvEntry`, `InterfaceEnvEntry`, `TemplataEnvEntry`) — **delete all five.** They become enum variants of `IEnvEntryT`, not separate types. The Scala `/* */` blocks stay alongside the new enum's variants (not alongside each deleted stub — see Gotcha 7 for how to move Scala blocks safely).

### `typing_interner.rs` (43 lines)

Current: `TypingInterner<'t>(PhantomData)`, with three stub `intern_id` / `intern_prototype` / `intern_signature` methods that panic. No per-concrete-name methods yet. **Replace the whole file.** Substrate + dual-lifetime flip in Step 2; real intern bodies in Step 6.

### Things NOT in Slab 4 scope (don't touch)

- `src/typing/names/names.rs` (Slab 2 frozen).
- `src/typing/types/types.rs` (Slab 3 frozen — **except** `OverloadSetT.env`'s `&'s` → `&'t` flip per Gotcha 11).
- `src/typing/templata/templata.rs` (Slab 3 frozen — **except** the 4 heavy-templata `&'s IEnvironmentT` fields per Gotcha 11).
- `src/typing/ast/ast.rs` (`PrototypeT`, `SignatureT`, `IdT`, etc. — Slab 2/3 touched; leave data defs alone. The `LocationInFunctionEnvironmentT` there has a `Vec<i32>` that's AASSNCMCX-non-compliant and will leak if the typing arena is dropped without running destructors — known issue, not yours to fix in Slab 4).
- `src/typing/compiler.rs` (god struct — Slab 4 may add an env-related helper method or two; don't restructure).
- `src/typing/ast/expressions.rs` (Slab 5 territory; it uses `ILocalVariableT<'s, 't>` already and will continue to compile once you give that enum real variants).
- Sub-compiler files (`expression/*.rs`, `function/*.rs`, `citizen/*.rs`, etc.). They reference env types via existing signatures; those signatures use the stubs you're replacing. **After** filling in the envs, run `cargo check --lib` and resolve any new compile errors there. Most fixes are "the stub didn't have a real field, now it does — update the use site to either access the field or stub-panic". Keep body work to a minimum; the goal is compile-green, not correct behavior.

## Rules for each field translation

Same rules as Slabs 2/3. Quick reference:

| Scala | Rust |
|---|---|
| `StrI` | `StrI<'s>` |
| `PackageCoordinate` | `&'s PackageCoordinate<'s>` |
| `IImpreciseNameS` | `&'s IImpreciseNameS<'s>` |
| `INameT` | `INameT<'s, 't>` (inline wrapper from Slab 2, 16 bytes) |
| `IdT[T]` | `IdT<'s, 't>` — **monomorphic**, no generic T. Narrow via `id.local_name` pattern-match. |
| `IdT[IFunctionNameT]` / `IdT[IFunctionTemplateNameT]` / etc. | Same: `IdT<'s, 't>`. Scala's outer generic is erased; narrowing happens at pattern-match. |
| `KindT`, `CoordT`, `ITemplataT` | `KindT<'s, 't>`, `CoordT<'s, 't>`, `ITemplataT<'s, 't>` — inline Copy wrappers from Slab 3. |
| `FunctionA`, `StructA`, `InterfaceA`, `ImplA` | `&'s FunctionA<'s>` etc. — scout-lifetime refs |
| `GlobalEnvironment` (Scala name) | `&'t GlobalEnvironmentT<'s, 't>` — arena-allocated, one per pass |
| `IEnvironmentT` (field in Scala) | `IEnvironmentT<'s, 't>` — the inline wrapper you're defining this slab. Variants hold `&'t FooEnvironmentT<'s, 't>`. |
| `IInDenizenEnvironmentT` (field in Scala) | `IInDenizenEnvironmentT<'s, 't>` — narrower inline wrapper |
| `TemplatasStore` | `TemplatasStoreT<'s, 't>` (by value, small — 3 slice-and-ref fields ≈ 40 bytes) |
| `Vector[IVariableT]` | `&'t [IVariableT<'s, 't>]` if fixed-post-build; otherwise `Vec<IVariableT<'s, 't>>` in a builder that freezes later. |
| `Vector[(INameT, IEnvEntry)]` | `&'t [(INameT<'s, 't>, IEnvEntryT<'s, 't>)]` inside `TemplatasStoreT`; `Vec<(…)>` in a builder. |
| `Set[IVarNameT]` | `&'t [IVarNameT<'s, 't>]` (arena slice, sorted-unique OR unsorted-unique; see Gotcha 3). |
| `Map[IImpreciseNameS, Vector[IEnvEntry]]` | `&'t [(&'s IImpreciseNameS<'s>, &'t [IEnvEntryT<'s, 't>])]` per the nested-slice option (see Gotcha 3). |
| `Option[X]` | `Option<X>` (Copy if X is Copy) |
| `Boolean` | `bool` |

### Derives

All concrete `*EnvironmentT` payload structs: `#[derive(PartialEq, Eq, Hash, Debug)]`. **Not `Copy` or `Clone`** — these are arena-allocated; AASSNCMCX + ATDCX apply. Slab 3 applied the same rule to concrete Kind payloads.

The wrapper enums (`IEnvironmentT`, `IInDenizenEnvironmentT`, `IEnvEntryT`, `IVariableT`, `ILocalVariableT`): `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]`. These are inline 16-24 byte values (tag + 8-byte ref / variant payload). Same philosophy as Slab 2 name wrappers and Slab 3 `KindT` wrapper.

Concrete variable payload structs (`AddressibleLocalVariableT`, `ReferenceLocalVariableT`, etc.) — these aren't arena-allocated; they're small enough to live inline in `ILocalVariableT` variants. `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]`. Note: Scala's `coord: CoordT` is inline `CoordT<'s, 't>` (~24 bytes), which makes `ReferenceLocalVariableT` about 40 bytes. Still acceptable inline; don't `&'t`-box them unless Scala semantics require it.

### Fieldless structs

Use a named `_phantom` field instead of a tuple struct, per Slab 2 convention:

```rust
pub struct EmptyT<'s, 't> {
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
```

None of the Slab 4 structs are fieldless — all have real Scala fields — so this is defensive.

## The 9 IEnvironmentT variants

Per quest.md §3.1 (with the `'t`-not-`'s` override) and the inline-wrapper philosophy:

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IEnvironmentT<'s, 't> {
    Package(&'t PackageEnvironmentT<'s, 't>),
    Citizen(&'t CitizenEnvironmentT<'s, 't>),
    Function(&'t FunctionEnvironmentT<'s, 't>),
    Node(&'t NodeEnvironmentT<'s, 't>),
    BuildingWithClosureds(&'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>),
    BuildingWithClosuredsAndTemplateArgs(&'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>),
    General(&'t GeneralEnvironmentT<'s, 't>),
    Export(&'t ExportEnvironmentT<'s, 't>),
    Extern(&'t ExternEnvironmentT<'s, 't>),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IInDenizenEnvironmentT<'s, 't> {
    Citizen(&'t CitizenEnvironmentT<'s, 't>),
    Function(&'t FunctionEnvironmentT<'s, 't>),
    Node(&'t NodeEnvironmentT<'s, 't>),
    BuildingWithClosureds(&'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>),
    BuildingWithClosuredsAndTemplateArgs(&'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>),
    General(&'t GeneralEnvironmentT<'s, 't>),
    // NO Package/Export/Extern — those are not InDenizen in Scala.
}
```

`IInDenizenEnvironmentT` holds a subset of `IEnvironmentT`'s variants. The two enums hold the same `&'t FooEnvironmentT` payloads, so casting between them is stack-only rewraps.

### `From`/`TryFrom` bridges

Write these for Slab-4 parity with Slab 2/3:

- `From<&'t FooEnvironmentT<'s, 't>> for IEnvironmentT<'s, 't>` for each of the 9 concrete envs.
- `From<&'t FooEnvironmentT<'s, 't>> for IInDenizenEnvironmentT<'s, 't>` for each of the 6 in-denizen envs.
- `From<IInDenizenEnvironmentT<'s, 't>> for IEnvironmentT<'s, 't>` (widening — always succeeds).
- `TryFrom<IEnvironmentT<'s, 't>> for IInDenizenEnvironmentT<'s, 't>` (narrowing — errors on Package/Export/Extern). Error type: `IEnvironmentT<'s, 't>` (return-the-input-on-error, like Slab 2's `TryFrom` patterns).
- `TryFrom<IEnvironmentT<'s, 't>> for &'t FooEnvironmentT<'s, 't>` for each variant (as needed downstream).

Use `From<&'t T>` (borrowed) not `From<T>` (owned) for the concrete→wrapper direction — matches Slab 2 names and Slab 3 kinds.

## Per-variant shapes

Read each Scala `/* */` block and translate. Below is a starting outline; the blocks are the source of truth.

**`PackageEnvironmentT<'s, 't>`** — Scala `case class PackageEnvironmentT[+T <: INameT]`. Erase `[+T]`. Fields: `global_env: &'t GlobalEnvironmentT<'s, 't>`, `id: IdT<'s, 't>`, `global_namespaces: &'t [TemplatasStoreT<'s, 't>]`.

**`CitizenEnvironmentT<'s, 't>`** — Scala `case class CitizenEnvironmentT[+T, +Y]`. Erase both generics. Fields: `global_env: &'t GlobalEnvironmentT<'s, 't>`, `parent_env: IEnvironmentT<'s, 't>`, `template_id: IdT<'s, 't>`, `id: IdT<'s, 't>`, `templatas: TemplatasStoreT<'s, 't>`. Note `parent_env` is by-value `IEnvironmentT` (a 16-byte inline wrapper), not `&'t IEnvironmentT` — the wrapper already contains a `&'t` ref internally. Same pattern everywhere you see a Scala `parentEnv: IEnvironmentT` field.

**`FunctionEnvironmentT<'s, 't>`** — see Scala block ~L554. Fields: `global_env`, `parent_env: IEnvironmentT<'s, 't>`, `template_id: IdT<'s, 't>`, `id: IdT<'s, 't>`, `templatas: TemplatasStoreT<'s, 't>`, `function: &'s FunctionA<'s>`, `maybe_return_type: Option<CoordT<'s, 't>>`, `closured_locals: &'t [IVariableT<'s, 't>]`, `is_root_compiling_denizen: bool`, `default_region: RegionT`.

**`NodeEnvironmentT<'s, 't>`** — see Scala block ~L161. Fields: `parent_function_env: &'t FunctionEnvironmentT<'s, 't>`, `parent_node_env: Option<&'t NodeEnvironmentT<'s, 't>>`, `node: &'s IExpressionSE<'s>` (scout-lifetime ref — `IExpressionSE` is defined in `src/postparsing/expressions.rs`, already arena-allocated in `'s`), `life: LocationInFunctionEnvironmentT<'s>`, `templatas: TemplatasStoreT<'s, 't>`, `declared_locals: &'t [IVariableT<'s, 't>]`, `unstackified_locals: &'t [IVarNameT<'s, 't>]`, `restackified_locals: &'t [IVarNameT<'s, 't>]`, `default_region: RegionT`. NodeEnvironmentT does NOT have `global_env` — it delegates to `parent_function_env.global_env`. Same for `id`, `function`, etc. This matches Scala's `override def id = parentFunctionEnv.id`. Don't store what you can derive.

Note: `&'t [IVarNameT<'s,'t>]` and `&'t [IVariableT<'s,'t>]` are dense slices of inline 16-byte wrappers (Slab 2/this-slab) — not slices of `&'t` refs. The slice element is the wrapper value itself.

**`BuildingFunctionEnvironmentWithClosuredsT<'s, 't>`** — see Scala block ~L24. Fields: `global_env`, `parent_env: IEnvironmentT<'s, 't>`, `id: IdT<'s, 't>`, `templatas: TemplatasStoreT<'s, 't>`, `function: &'s FunctionA<'s>`, `variables: &'t [IVariableT<'s, 't>]`, `is_root_compiling_denizen: bool`.

**`BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't>`** — see Scala block ~L92. Fields: `global_env`, `parent_env: IEnvironmentT<'s, 't>`, `id: IdT<'s, 't>`, `template_args: &'t [ITemplataT<'s, 't>]`, `templatas: TemplatasStoreT<'s, 't>`, `function: &'s FunctionA<'s>`, `variables: &'t [IVariableT<'s, 't>]`, `is_root_compiling_denizen: bool`, `default_region: RegionT`.

**`GeneralEnvironmentT<'s, 't>`** — see Scala block ~L665. Fields: `global_env`, `parent_env: IInDenizenEnvironmentT<'s, 't>`, `template_id: IdT<'s, 't>`, `id: IdT<'s, 't>`, `templatas: TemplatasStoreT<'s, 't>`. (Note `parent_env` is the narrower in-denizen enum here, per Scala.)

**`ExportEnvironmentT<'s, 't>`** — see Scala block ~L595. Fields: `global_env`, `parent_env: &'t PackageEnvironmentT<'s, 't>`, `template_id: IdT<'s, 't>`, `id: IdT<'s, 't>`, `templatas: TemplatasStoreT<'s, 't>`. (Note `parent_env` is a direct `&'t PackageEnvironmentT` here, not an `IEnvironmentT` wrapper — Scala constrains it.)

**`ExternEnvironmentT<'s, 't>`** — see Scala block ~L630. Same shape as `ExportEnvironmentT`: `global_env`, `parent_env: &'t PackageEnvironmentT<'s, 't>`, `template_id`, `id`, `templatas`.

## Gotchas

### Gotcha 1 (resolved): envs go in `'t`, not `'s`

`quest.md` §3.1 places envs in the scout arena and gives them `scout_arena.alloc(...)`. This is **infeasible and must be overridden.**

Reason: envs hold `TemplatasStoreT` which stores `IEnvEntryT::Templata(ITemplataT<'s, 't>)`, which transitively holds `&'t` refs (Slab 3 made `ITemplataT` an inline wrapper whose variants are `&'t` refs to interned payloads). A struct with lifetime `'s` can only hold `&'x T` where `'x: 's`. Since `'s: 't` (scout outlives typing), `'t: 's` is **false**, so a `'s`-allocated env cannot hold a `&'t` ref. The borrow checker will reject this.

**Fix:** envs go in the typing arena `'t`. Every `&'s FooEnvironmentT<'s, 't>` in quest.md Part 3 becomes `&'t FooEnvironmentT<'s, 't>`. Builder `build_in` methods take `&TypingBump<'t>` (or whatever the typing-arena handle is, following Slab 0's substrate), not `&ScoutArena<'s>`.

Full write-up: `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md` § "Arena choice". The TL-HANDOFF.md override section flags it too.

### Gotcha 2: TypingInterner real-body implementation

Slabs 2-3 left all the `intern_*` methods as `panic!()`. Slab 4 implements them. Model the implementation on `src/scout_arena.rs` — it has a working `intern_rune()` / `intern_name()` / `intern_imprecise_name()` with `bumpalo::Bump` + `hashbrown::HashMap<ValT, CanonicalT>` + `RefCell<Inner>`.

**Reference implementation to copy the shape from:** `src/scout_arena.rs` `intern_rune` (~L329) and its `alloc_rune_canonical` helper (~L347). Read them before writing the typing version.

Scope to implement in Slab 4 (all as real bodies):

**6 family-level intern methods** — the real work happens here. Each takes the family's union Val, does one `hashbrown::HashMap` lookup, allocates+inserts on miss, returns the canonical form:

1. `intern_name(val: INameValT<'s,'t,'tmp>) -> INameT<'s,'t>` — dispatches across all ~60 concrete names based on the Val variant. See `scout_arena.rs::intern_name` (L245) for the shape.
2. `intern_id(val: IdValT<'s,'t,'tmp>) -> &'t IdT<'s,'t>` — `'tmp`-borrowed `init_steps` slice (@DSAUIMZ); promote on miss.
3. `intern_prototype(val: PrototypeValT<'s,'t,'tmp>) -> &'t PrototypeT<'s,'t>`.
4. `intern_signature(val: SignatureValT<'s,'t,'tmp>) -> &'t SignatureT<'s,'t>`.
5. `intern_kind_payload(val: InternedKindPayloadValT<'s,'t>) -> InternedKindPayloadT<'s,'t>` — dispatches across 6 Kind payload concretes.
6. `intern_templata_payload(val: InternedTemplataPayloadValT<'s,'t,'tmp>) -> InternedTemplataPayloadT<'s,'t>` — dispatches across 6 interned templata payload concretes.

**~75 per-concrete convenience wrappers** — mechanical, each ~5 lines: wrap input into the family Val, dispatch through `intern_<family>`, unwrap to the specific concrete. Same shape as scout's `intern_struct_declaration_name` (L231-236). Examples:

```rust
pub fn intern_function_name<'tmp>(&self, val: FunctionNameValT<'s,'t,'tmp>) -> &'t FunctionNameT<'s,'t> {
    match self.intern_name(INameValT::FunctionName(val)) {
        INameT::Function(r) => r,
        _ => unreachable!(),
    }
}
pub fn intern_struct_tt(&self, val: StructTT<'s,'t>) -> &'t StructTT<'s,'t> {
    match self.intern_kind_payload(InternedKindPayloadValT::StructTT(val)) {
        InternedKindPayloadT::StructTT(r) => r,
        _ => unreachable!(),
    }
}
// ... one per concrete: ~60 name + 6 Kind payload + 6 templata payload + 3 top-level (Id/Prototype/Signature) = ~75 wrappers.
```

These wrappers are what downstream sub-compiler code calls. Consider a `macro_rules!` helper (`impl_intern_wrapper!(method_name, family, variant, ValT, CanonicalT)`) to reduce boilerplate — the `intern_function_name` shape is identical across ~60 name wrappers. Slab 2 may have similar macros for `From`/`TryFrom` bridges worth reusing as a pattern.

**`TypingInterner` needs a second lifetime parameter.** The current stub is `TypingInterner<'t>(PhantomData)` — *that's insufficient once you add real bodies.* Interned values carry both lifetimes in their types (e.g. `FunctionNameT<'s, 't>` has `human_name: StrI<'s>`; `IdT<'s, 't>` has `init_steps: &'t [INameT<'s, 't>]`). The interner's `Inner<'s, 't>` holds HashMaps keyed on those types, so the outer type becomes `TypingInterner<'s, 't>`. This ripples through:

- `Compiler<'s, 'ctx, 't>.typing_interner: &'ctx TypingInterner<'s, 't>` (currently `&'ctx TypingInterner<'t>`).
- Every sub-compiler method that takes `&TypingInterner<'t>` or holds one.
- Every external `TypingInterner::new` call site.

Budget it: probably a mechanical sed across ~10-30 sites.

**One HashMap per sealed-trait family, not per concrete.** Scout's `ScoutArena` (see `src/scout_arena.rs` L62-L65) has three HashMaps — `name_val_to_ref`, `rune_val_to_ref`, `imprecise_name_val_to_ref` — each keyed on a tagged-union `*ValS` enum that holds one variant per concrete in that sealed-trait family. Per-concrete intern methods (`intern_struct_declaration_name`, etc.) wrap into the union, dispatch through the single map, and unwrap on return. Typing uses the same pattern. This is a large savings over a per-concrete design: **~6 maps total, not ~75.**

The 6 maps cover the six interned sealed-trait families:

| Map | Key | Value | Map type | Scope |
|---|---|---|---|---|
| `name_val_to_ref` | `INameValT<'s,'t,'tmp>` | `INameT<'s,'t>` | `hashbrown` (some names have slices) | all ~60 concrete names |
| `id_val_to_ref` | `IdValT<'s,'t,'tmp>` | `&'t IdT<'s,'t>` | `hashbrown` (init_steps slice) | IdT |
| `prototype_val_to_ref` | `PrototypeValT<'s,'t,'tmp>` | `&'t PrototypeT<'s,'t>` | `hashbrown` (transitive via IdT) | PrototypeT |
| `signature_val_to_ref` | `SignatureValT<'s,'t,'tmp>` | `&'t SignatureT<'s,'t>` | `hashbrown` (transitive via IdT) | SignatureT |
| `kind_payload_val_to_ref` | `InternedKindPayloadValT<'s,'t>` | `InternedKindPayloadT<'s,'t>` | `std::HashMap` (all simple/shallow) | 6 concrete Kind payloads |
| `templata_payload_val_to_ref` | `InternedTemplataPayloadValT<'s,'t,'tmp>` | `InternedTemplataPayloadT<'s,'t>` | `hashbrown` (CoordListTemplataT has a slice) | 6 interned templata payloads |

**New unifying Val enums you need to create in Slab 4:**

Slab 2/3 created per-concrete `*ValT` structs but did not add a top-level union enum. Slab 4 adds three:

```rust
// In src/typing/names/names.rs — next to the existing per-concrete ValTs:
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum INameValT<'s, 't, 'tmp> {
    FunctionName(FunctionNameValT<'s, 't, 'tmp>),
    StructName(StructNameValT<'s, 't, 'tmp>),
    InterfaceName(InterfaceNameValT<'s, 't, 'tmp>),
    // ... one variant per concrete name (~60)
}

// In src/typing/types/types.rs:
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum InternedKindPayloadValT<'s, 't> {
    StructTT(StructTT<'s, 't>),
    InterfaceTT(InterfaceTT<'s, 't>),
    StaticSizedArrayTT(StaticSizedArrayTT<'s, 't>),
    RuntimeSizedArrayTT(RuntimeSizedArrayTT<'s, 't>),
    KindPlaceholder(KindPlaceholderT<'s, 't>),
    OverloadSet(OverloadSetT<'s, 't>),
}

// And its &'t-ref union (map value type):
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum InternedKindPayloadT<'s, 't> {
    StructTT(&'t StructTT<'s, 't>),
    InterfaceTT(&'t InterfaceTT<'s, 't>),
    // ...
}

// In src/typing/templata/templata.rs — analogous:
pub enum InternedTemplataPayloadValT<'s, 't, 'tmp> { /* 6 variants */ }
pub enum InternedTemplataPayloadT<'s, 't>          { /* 6 variants of &'t refs */ }
```

These are the same shape as `INameValS` in `src/postparsing/names.rs` L81-L97. Each is ~100 lines including per-variant `From`/`TryFrom` bridges and manual `Hash`/`PartialEq` where the Val has slices (`hashbrown`'s heterogeneous-lookup requires consistent hashing between `'tmp` query and `'t` stored forms — see IDEPFL / @DSAUIMZ).

These Val union enums (`INameValT` / `InternedKindPayloadValT` / `InternedTemplataPayloadValT`) don't exist yet in the codebase. **Slab 4 creates them as part of Step 2.** They live next to the per-concrete `*ValT` structs from Slab 2/3.

Implementation skeleton:

```rust
pub struct TypingInterner<'s, 't> {
    bump: &'t Bump,
    inner: RefCell<Inner<'s, 't>>,
}

struct Inner<'s, 't> {
    name_val_to_ref:
        hashbrown::HashMap<INameValT<'s, 't, 't>, INameT<'s, 't>>,
    id_val_to_ref:
        hashbrown::HashMap<IdValT<'s, 't, 't>, &'t IdT<'s, 't>>,
    prototype_val_to_ref:
        hashbrown::HashMap<PrototypeValT<'s, 't, 't>, &'t PrototypeT<'s, 't>>,
    signature_val_to_ref:
        hashbrown::HashMap<SignatureValT<'s, 't, 't>, &'t SignatureT<'s, 't>>,
    kind_payload_val_to_ref:
        HashMap<InternedKindPayloadValT<'s, 't>, InternedKindPayloadT<'s, 't>>,
    templata_payload_val_to_ref:
        hashbrown::HashMap<InternedTemplataPayloadValT<'s, 't, 't>, InternedTemplataPayloadT<'s, 't>>,
}

impl<'s, 't> TypingInterner<'s, 't> {
    pub fn new(bump: &'t Bump) -> Self { /* ... */ }

    // Pass-throughs to self.bump — mirrors ScoutArena's shape.
    pub fn alloc<T>(&self, val: T) -> &'t mut T { self.bump.alloc(val) }
    pub fn alloc_slice_copy<T: Copy>(&self, src: &[T]) -> &'t [T] {
        self.bump.alloc_slice_copy(src)
    }

    // Top-level family interners do all the real work.
    pub fn intern_name<'tmp>(&self, val: INameValT<'s, 't, 'tmp>) -> INameT<'s, 't> { /* ... */ }
    // ... 5 more intern_<family> methods.

    // Per-concrete API (caller convenience) — wraps, dispatches, unwraps.
    pub fn intern_function_name<'tmp>(&self, val: FunctionNameValT<'s, 't, 'tmp>) -> &'t FunctionNameT<'s, 't> {
        match self.intern_name(INameValT::FunctionName(val)) {
            INameT::Function(r) => r,
            _ => unreachable!(),
        }
    }
    // ~75 of these — all mechanical wrappers. Mirror scout_arena's
    // `intern_struct_declaration_name` / `intern_interface_declaration_name` shape.
}
```

When stored, `'tmp` has collapsed to `'t` (any slice in the Val was arena-copied on miss). `hashbrown::HashMap` supports heterogeneous lookup via its `raw_entry` API — see `scout_arena.rs` line ~332 (`intern_rune`) for the live pattern.

**Sizing expectation:** after Slab 4, `typing_interner.rs` is probably 800-1500 lines (6 family-level `intern_<family>` methods with `alloc_<family>_canonical` helpers + ~75 per-concrete wrapper methods + `Inner` struct with 6 HashMap fields). You can split into one file per family if it helps (`typing_interner/mod.rs`, `typing_interner/names.rs`, `typing_interner/kinds.rs`, `typing_interner/templatas.rs`, `typing_interner/ids.rs`) — match whatever scout_arena.rs does. If scout_arena.rs keeps everything in one file, follow suit.

**Test gate:** after writing interner bodies, the existing `src/typing/tests/` (if any) may want at least a smoke test: "build an `IdT` for `myapp::Main`, intern it twice, check `std::ptr::eq`." Skip if there are no typing tests yet — Slab 9 test work picks that up.

### Gotcha 3: `TemplatasStoreT` layout (slice-of-pairs, nested-slice variant)

Per quest.md §3.2 with the `'t` correction:

```rust
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct TemplatasStoreT<'s, 't> {
    pub templatas_store_name: &'t IdT<'s, 't>,  // Scala: templatasStoreName
    pub name_to_entry: &'t [(INameT<'s, 't>, IEnvEntryT<'s, 't>)],
    pub imprecise_to_entries: &'t [(&'s IImpreciseNameS<'s>, &'t [IEnvEntryT<'s, 't>])],
}
```

- **`name_to_entry`** — unsorted slice. Linear scan in lookup. Scala `Map[INameT, IEnvEntry]`.
- **`imprecise_to_entries`** — nested-slice layout per the senior's answer (#4: Option A). Each `IImpreciseNameS` key gets its own inner arena slice of entries sharing that imprecise name. At builder freeze time: for each imprecise name, allocate a slice of its entries via `bump.alloc_slice_copy(&entries_for_name)`, collect `(&'s IImpreciseNameS, &'t [IEnvEntryT])` tuples into an outer `Vec`, freeze outer as `&'t [(…)]`. Two-level allocation.
- **`templatas_store_name: &'t IdT`** — this is Scala's `templatasStoreName: IdT[INameT]`. It identifies which env the store belongs to.

`TemplatasStoreT` is `Clone`-free (arena-pinned, never clone) and not `Copy` (too big — 3 slice-pointers plus the id ref = 48-56 bytes). Pass by `&` or inline-by-value into env structs.

**Lookup strategy:** linear scan everywhere. See quest.md §3.2 + TL-HANDOFF "Notable state" for the profile-later guidance. If you're tempted to sort and binary-search, don't — senior decided linear scan uniformly (answer #7).

### Gotcha 4: Delete all `*Box*T` / `IDenizenEnvironmentBoxT` stubs

Scala has a family of mutable wrappers over immutable inner envs — `NodeEnvironmentBox`, `FunctionEnvironmentBoxT`, and the trait `IDenizenEnvironmentBoxT` they share. In Rust, the builder-freeze pattern replaces this entire family: mutation happens in a `*Builder` stack struct, then you freeze into a `&'t FooEnvironmentT<'s, 't>` that's immutable from then on.

**Delete every `*Box*T` / box-trait stub** during Slab 4 — they all map to the builder pattern, none survive:
- `NodeEnvironmentBox<'s, 't>` in `function_environment_t.rs`.
- `FunctionEnvironmentBoxT<'s, 't>` in `function_environment_t.rs`.
- `IDenizenEnvironmentBoxT<'s, 't>` trait in `environment.rs`.
- Any other `*Box*T` stub you find that extends `IDenizenEnvironmentBoxT` in Scala — same treatment by analogy, no need to re-ask.

Remove each Rust stub. The Scala `/* */` block stays as part of the audit trail but with no Rust definition above it (this is legal per the pre-commit hook — see Gotcha 7).

**Fix call sites** at whatever minimal level makes `cargo check --lib` pass. Most `*Box*T` references currently live inside `/* */` Scala blocks (zero live Rust call sites for `FunctionEnvironmentBoxT`; a handful for `NodeEnvironmentBox` in `local_helper.rs` etc.). For the live ones: swap the param type to `&mut NodeEnvironmentBuilder<'s, 't>` / `&mut FunctionEnvironmentBuilder<'s, 't>` where the method actually uses the box's mutation API; otherwise make the whole body `panic!("Unimplemented: Slab 8 signature rewrite")` and let Slab 8 figure out the exact shape.

**Hot tip:** `rg -n "NodeEnvironmentBox|FunctionEnvironmentBoxT|IDenizenEnvironmentBoxT" src/typing/ --type rust` before you start, to see the full call-site list.

### Gotcha 5: `GlobalEnvironmentT` definition

Scala `GlobalEnvironment` has macro fields that in Rust live on `Compiler` instead (per the god-struct refactor — see `FrontendRust/docs/migration/handoff-god-struct-progress.md`). Rust `GlobalEnvironmentT<'s, 't>` drops the macro fields and keeps only the data:

```rust
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct GlobalEnvironmentT<'s, 't> {
    // Top-level envs keyed by their package-top-level-name id
    pub name_to_top_level_environment:
        &'t [(&'t IdT<'s, 't>, TemplatasStoreT<'s, 't>)],
    // Primitives and other builtins
    pub builtins: TemplatasStoreT<'s, 't>,
}
```

Scala fields to **drop** (now on `Compiler` via the god-struct refactor):
- `functorHelper`, `structConstructorMacro`, `structDropMacro`, `interfaceDropMacro`, `anonymousInterfaceMacro` — these were macro-class instance fields; macros are methods on `Compiler` now.
- `nameToStructDefinedMacro`, `nameToInterfaceDefinedMacro`, `nameToImplDefinedMacro`, `nameToFunctionBodyMacro` — these were `Map[StrI, IFunctionBodyMacro]`-style dispatch tables; in the Rust design, dispatch uses unit-variant enums on `Compiler` (see god-struct doc's "macros pattern" section). Not on `GlobalEnvironmentT`.

The Scala `/* */` block stays verbatim (pre-commit hook requires it). Just don't translate those specific Scala fields to Rust fields. Add a one-line `// (macro fields omitted — see god-struct refactor)` comment above the Rust struct if it helps the next reader.

**Instantiation:** `GlobalEnvironmentT` is allocated in `'t` once per typing pass, near the entry point (`run_typing_pass` or equivalent). Every env's `global_env` field is a `&'t GlobalEnvironmentT<'s, 't>` back-ref. No `Rc`, no cycles (see the reasoning doc's "cycles" section).

### Gotcha 6: `IEnvEntryT` is an inline 5-variant enum

Per senior's answer (#3):

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IEnvEntryT<'s, 't> {
    Function(&'s FunctionA<'s>),
    Struct(&'s StructA<'s>),
    Interface(&'s InterfaceA<'s>),
    Impl(&'s ImplA<'s>),
    Templata(ITemplataT<'s, 't>),
}
```

No separate `FunctionEnvEntry` / `StructEnvEntry` / etc. structs — delete those five stubs in `i_env_entry.rs`. The scout refs (`&'s FunctionA` etc.) are the entire variant payload; `ITemplataT` is already the inline Slab-3 wrapper.

Size: 1 byte tag + 16-byte `ITemplataT` variant (the wide case) → ~24 bytes. The `&'s _` variants are 1 tag + 8-byte ref = ~16 bytes, but the enum sizes to its biggest variant.

Not interned. Not `&'t`-boxed. Lives inline in `TemplatasStoreT.name_to_entry: &'t [(INameT, IEnvEntryT)]`.

The Scala `/* */` blocks for `FunctionEnvEntry`, `ImplEnvEntry`, `StructEnvEntry`, `InterfaceEnvEntry`, `TemplataEnvEntry` stay in the file next to the new enum, not next to individual Rust stubs (since the stubs are deleted). The pre-commit hook only cares that `/* */` blocks aren't edited, not that they have a Rust sibling — see Gotcha 7.

### Gotcha 7: Pre-commit hook on `/* */` blocks

`.claude/hooks/check-scala-comments` does **exact-match** comparison on every `/* ... */` Scala block. Any edit inside rejects the commit. Rules for this slab:

- **Don't edit inside a `/* */` block** — the text between `/*` and `*/` is frozen.
- **You can delete a Rust stub above a `/* */` block** as long as the `/* */` stays byte-for-byte unchanged. The hook only checks the block content. (This is how you'll handle `NodeEnvironmentBox` deletion, `FunctionEnvEntry` deletion, etc.)
- **You can reorder `/* */` blocks** within a file if needed, as long as each block's content stays identical. Rarely needed, but possible.
- **You can add a new Rust definition between two existing `/* */` blocks** (e.g. a builder struct that has no direct Scala counterpart — put it before the `/* */` block of its frozen-form companion, with a `// (no scala counterpart — builder for NodeEnvironmentT)` note).

Same rule as Slabs 2/3. You won't be running `git commit` yourself (the human does that after review), but the hook still defines what edits are legal — if you accidentally change whitespace inside a `/* */` block, it'll show up at the human's commit-time and bounce back to you. Treat the hook as an always-on invariant, not a commit-time check.

### Gotcha 8: Don't write `impl EnvironmentT { fn … }` method bodies

Data definitions only. Scala `case class` bodies with `override def equals`, `def addEntry(interner, name, entry): FooEnvironmentT = …`, `def rootCompilingDenizenEnv`, `def lookupWithNameInner` etc. are Slab 8+ work. Leave them in the `/* */` and don't port them in Slab 4.

Stubs that already exist in sub-compiler files (like `impl Compiler { fn lookup_nearest_with_name(...) { panic!() } }`) stay `panic!()` until the signature-rewrite slab.

**Exception:** you may need `impl` blocks for a few mechanical things:
1. **Builder `build_in` methods** — real implementations, because the builders don't exist in Scala and Slab 4 has to provide them.
2. **`From`/`TryFrom` bridges** — real implementations, per Slab 2/3 convention.
3. **Trivial projection methods** (like `pub fn id(&self) -> IdT<'s, 't> { self.id }`) — OK if directly needed to make another Slab 4 definition type-check. Otherwise wait for Slab 8.

If in doubt, prefer `panic!()` bodies and let Slab 8 fill them.

### Gotcha 9: `FunctionEnvironmentT.parent_env` is inclusive of Package

Scala `FunctionEnvironmentT.parentEnv: IEnvironmentT` (wide). `FunctionEnvironmentT.rootCompilingDenizenEnv` matches on `parentEnv` and `vwat()`s on `PackageEnvironmentT` — so `parentEnv` can be `PackageEnvironmentT` at runtime even though the logic assumes it won't be. Keep `parent_env: IEnvironmentT<'s, 't>` as the Scala type says; the runtime guarantees aren't your concern during data translation.

### Gotcha 10: Slab 3 patched env stubs with derives

Slab 3 patched `IEnvironmentT` / `IInDenizenEnvironmentT` stubs with `#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]` so `OverloadSetT` could derive `Hash`/`Eq` (it contains an env ref). When you replace those stubs with real enums, **keep the derives identical** — `Copy, Clone, PartialEq, Eq, Hash, Debug`. If you drop any, `OverloadSetT` stops compiling and Slab 3 breaks retroactively.

Spot check: after replacing, `cargo check --lib` should not complain about `OverloadSetT` — if it does, you lost a derive.

### Gotcha 11: Slab-3 heavy templatas reference envs as `&'s`; flip them to `&'t`

When Slab 3 shipped, the env stubs still implied `'s` arena allocation (quest.md §3.1's original spec). The code in `src/typing/templata/templata.rs` and `src/typing/types/types.rs` thus landed with `&'s IEnvironmentT<'s, 't>` / `&'s IInDenizenEnvironmentT<'s, 't>` refs in five places. Slab 4's `'t` arena correction (Gotcha 1) makes these stale.

Five sites to flip `&'s` → `&'t`:
- `FunctionTemplataT.outer_env: &'s IEnvironmentT<'s, 't>` → `&'t`
- `StructDefinitionTemplataT.declaring_env: &'s IEnvironmentT<'s, 't>` → `&'t`
- `InterfaceDefinitionTemplataT.declaring_env: &'s IEnvironmentT<'s, 't>` → `&'t`
- `ImplDefinitionTemplataT.env: &'s IEnvironmentT<'s, 't>` → `&'t`
- `OverloadSetT.env: &'s IInDenizenEnvironmentT<'s, 't>` → `&'t`

Do this in **Step 5.5** — after the 9 env variants exist (Step 5) and before the interner bodies (Step 6). The step plan has a dedicated slot for it. Verify with `cargo check --lib`.

Other field-type references to env types in sub-compiler method signatures (e.g. `&IInDenizenEnvironmentT<'s, 't>` in `edge_compiler.rs`) are elided-lifetime borrows — those stay as-is since they don't pin to an arena.

### Gotcha 12: `LocationInFunctionEnvironmentT.path: Vec<i32>` is pre-existing debt

`LocationInFunctionEnvironmentT<'s>` in `ast/ast.rs` has `path: Vec<i32>`. That's an AASSNCMCX-nonconforming `Vec` inside a type that lives conceptually in the `'t` arena. Two problems: (a) it violates the shield; (b) if someone later arena-allocates a struct containing this, bumpalo won't run the `Vec`'s destructor → leak. **Not Slab 4's problem.** You'll use `LocationInFunctionEnvironmentT` as-is (`NodeEnvironmentT` has `life: LocationInFunctionEnvironmentT<'s>`). If it triggers shield complaints during review, point at this gotcha and move on. Fix is probably "change to `&'t [i32]`" in a future cleanup slab.

### Gotcha 13: env `Hash`/`PartialEq` must be id-based, not derived

Scala env case classes override `equals` to compare *only the id* (and override `hashCode` to cache `id.hashCode`). Example (Scala `FunctionEnvironmentT`):

```scala
val hash = runtime.ScalaRunTime._hashCode(id);
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = {
    if (!obj.isInstanceOf[IInDenizenEnvironmentT]) return false
    id.equals(obj.asInstanceOf[IInDenizenEnvironmentT].id)
}
```

`#[derive(PartialEq, Eq, Hash)]` on the Rust struct would walk every field — including `TemplatasStoreT`'s two slices — which (a) is slow, and (b) diverges from Scala's id-based identity. Two envs with the same id but temporarily different templatas should compare equal; the derived version wouldn't.

**Do this for every env that has a Scala `override def equals`:** `PackageEnvironmentT`, `CitizenEnvironmentT`, `ExportEnvironmentT`, `ExternEnvironmentT`, `FunctionEnvironmentT`, `BuildingFunctionEnvironmentWithClosuredsT`, `BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT`. Manual impl:

```rust
impl<'s, 't> PartialEq for FunctionEnvironmentT<'s, 't> {
    fn eq(&self, other: &Self) -> bool { self.id == other.id }
}
impl<'s, 't> Eq for FunctionEnvironmentT<'s, 't> {}
impl<'s, 't> std::hash::Hash for FunctionEnvironmentT<'s, 't> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.id.hash(state); }
}
```

**`NodeEnvironmentT` is different** — Scala hashes `id.hashCode ^ life.hashCode` and compares `(id, life)`. So:
```rust
impl<'s, 't> PartialEq for NodeEnvironmentT<'s, 't> {
    fn eq(&self, other: &Self) -> bool {
        self.parent_function_env.id == other.parent_function_env.id
            && self.life == other.life
    }
}
// Hash analogously.
```

**`GeneralEnvironmentT`** has `override def equals = vcurious()` / `override def hashCode = vcurious()` — in Rust that's `panic!("vcurious")` in a manual impl. Match Scala; don't derive.

Keep `Debug` derived. Keep the struct-level `#[derive]` block but remove `PartialEq, Eq, Hash` from it, then add the manual impls separately.

### Gotcha 14: `where 's: 't` on every env struct

The current `PhantomData` stubs implicitly anchor `'s` and `'t` via `PhantomData<(&'s (), &'t ())>`. When you replace them with real fields that hold `&'t` data, you need the explicit lifetime bound. Otherwise the compiler errors with "`'s` may not live long enough" on any field of the form `&'t SomethingWith_s<'s>`:

```rust
pub struct FunctionEnvironmentT<'s, 't>
where 's: 't,
{
    pub global_env: &'t GlobalEnvironmentT<'s, 't>,
    // ... etc
}
```

Put `where 's: 't` on every env struct, every builder, `GlobalEnvironmentT`, `TemplatasStoreT`, `TemplatasStoreBuilder`, `IEnvEntryT`, `IVariableT`, `ILocalVariableT`, and the 4 concrete variable structs. Slab 2/3 did the same for their arena-allocated types.

`IEnvironmentT` / `IInDenizenEnvironmentT` (the wrapper enums) already work because their variants hold `&'t` refs without needing an explicit bound — Rust infers it from the usage. But adding `where 's: 't` to them too doesn't hurt and keeps the file uniform.

### Gotcha 15: `i_env_entry.rs` stays; only the type gets a `T` suffix

Scala's type is `IEnvEntry` (no `T`); its source file is `IEnvEntry.scala`; the Rust stub file is `src/typing/env/i_env_entry.rs` (matching the Scala filename). Slab 4 adds the `T` suffix to the Rust *type* — `IEnvEntry` → `IEnvEntryT` — for consistency with the rest of the typing pass (every other typing-pass type in Rust carries `T`). **Don't rename the file.** Rust filenames track the Scala-source filename, not the Rust-type name; that's why `function_environment_t.rs` does have `_t` (Scala source is `FunctionEnvironmentT.scala`) but `i_env_entry.rs` doesn't.

### Gotcha 16: `IVariableT` / `ILocalVariableT` DAG and variants

Scala has a small DAG:

```scala
sealed trait IVariableT {
    def name: IVarNameT
    def variability: VariabilityT
    def coord: CoordT
}
sealed trait ILocalVariableT extends IVariableT {
    // inherits the same fields
}

case class AddressibleLocalVariableT(name, variability, coord) extends ILocalVariableT
case class ReferenceLocalVariableT(name, variability, coord) extends ILocalVariableT
case class AddressibleClosureVariableT(name, closuredVarsStructType: StructTT, variability, coord) extends IVariableT  // NOT ILocal
case class ReferenceClosureVariableT(name, closuredVarsStructType: StructTT, variability, coord) extends IVariableT   // NOT ILocal
```

Apply the Slab 2 DAG rule: each concrete variant appears in each sub-enum it transitively belongs to.

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IVariableT<'s, 't> {
    AddressibleLocal(AddressibleLocalVariableT<'s, 't>),
    ReferenceLocal(ReferenceLocalVariableT<'s, 't>),
    AddressibleClosure(AddressibleClosureVariableT<'s, 't>),
    ReferenceClosure(ReferenceClosureVariableT<'s, 't>),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ILocalVariableT<'s, 't> {
    Addressible(AddressibleLocalVariableT<'s, 't>),
    Reference(ReferenceLocalVariableT<'s, 't>),
}
```

Variants hold the concrete structs **by value**, not `&'t`. These structs are small (~40 bytes with `coord: CoordT` inline) and not arena-interned — they're analogous to Scala `ILocalVariableT` being a plain case class, not a pointer-indirection. If sizing becomes a problem later, revisit; for Slab 4, inline.

Concrete shapes to match Scala:

```rust
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AddressibleLocalVariableT<'s, 't> {
    pub name: IVarNameT<'s, 't>,
    pub variability: VariabilityT,
    pub coord: CoordT<'s, 't>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ReferenceLocalVariableT<'s, 't> {
    pub name: IVarNameT<'s, 't>,
    pub variability: VariabilityT,
    pub coord: CoordT<'s, 't>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AddressibleClosureVariableT<'s, 't> {
    pub name: IVarNameT<'s, 't>,
    pub closured_vars_struct_type: &'t StructTT<'s, 't>,  // Slab 3 interned payload
    pub variability: VariabilityT,
    pub coord: CoordT<'s, 't>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ReferenceClosureVariableT<'s, 't> {
    pub name: IVarNameT<'s, 't>,
    pub closured_vars_struct_type: &'t StructTT<'s, 't>,
    pub variability: VariabilityT,
    pub coord: CoordT<'s, 't>,
}
```

Note `closured_vars_struct_type: &'t StructTT<'s, 't>` — Scala's `closuredVarsStructType: StructTT` maps to a `&'t` ref because `StructTT` is an interned Slab-3 Kind payload (arena-allocated, accessed by pointer identity), not a by-value Copy type.

Write `From`/`TryFrom` bridges both directions: concrete → `ILocalVariableT`, concrete → `IVariableT`, `ILocalVariableT` → `IVariableT`, `TryFrom<IVariableT>` for each concrete. Slab 2 style.

## Builder types

One `*Builder` per env variant. Naming: `NodeEnvironmentBuilder<'s, 't>`, `FunctionEnvironmentBuilder<'s, 't>`, `CitizenEnvironmentBuilder<'s, 't>`, etc. Lives next to the env struct in the same file.

**All 9 envs need builders** because all 9 contain a `TemplatasStoreT`, and `TemplatasStoreT` is itself built via `TemplatasStoreBuilder` (heap `Vec` + heap `HashMap`, frozen into `'t` slices on `build_in`). Envs that also carry other incrementally-built collections (`NodeEnvironmentT.declared_locals`, `FunctionEnvironmentT.closured_locals`, etc.) use the builder for those too.

`TemplatasStoreBuilder<'s, 't>` is the base builder; env-specific builders compose it.

Shape per quest.md §3.3:

```rust
pub struct NodeEnvironmentBuilder<'s, 't> {
    pub parent_function_env: &'t FunctionEnvironmentT<'s, 't>,
    pub parent_node_env: Option<&'t NodeEnvironmentT<'s, 't>>,
    pub node: &'s IExpressionSE<'s>,
    pub life: LocationInFunctionEnvironmentT<'s>,
    pub templatas_builder: TemplatasStoreBuilder<'s, 't>,
    pub declared_locals: Vec<IVariableT<'s, 't>>,
    pub unstackified_locals: Vec<IVarNameT<'s, 't>>,
    pub restackified_locals: Vec<IVarNameT<'s, 't>>,
    pub default_region: RegionT,
}

impl<'s, 't> NodeEnvironmentBuilder<'s, 't> {
    pub fn build_in(self, interner: &TypingInterner<'t>) -> &'t NodeEnvironmentT<'s, 't> {
        let templatas = self.templatas_builder.build_in(interner);
        let declared_locals = interner.bump().alloc_slice_copy(&self.declared_locals);
        let unstackified_locals = interner.bump().alloc_slice_copy(&self.unstackified_locals);
        let restackified_locals = interner.bump().alloc_slice_copy(&self.restackified_locals);
        interner.bump().alloc(NodeEnvironmentT {
            parent_function_env: self.parent_function_env,
            parent_node_env: self.parent_node_env,
            node: self.node,
            life: self.life,
            templatas,
            declared_locals,
            unstackified_locals,
            restackified_locals,
            default_region: self.default_region,
        })
    }
}
```

Note: `interner.bump()` — add a getter on `TypingInterner` returning `&'t Bump` (or the arena handle you use). Call sites freezing non-interned data need it. Alternatively, pass `&'t Bump` as a separate `build_in` param; pick what minimizes noise.

`TemplatasStoreBuilder<'s, 't>` is its own builder type with `name_to_entry: Vec<(INameT, IEnvEntryT)>` and `imprecise_to_entries: HashMap<&'s IImpreciseNameS<'s>, Vec<IEnvEntryT>>` (heap HashMap during construction, frozen to nested slices on `build_in`).

**Child-scope API (per senior's answer #6):** `NodeEnvironmentT::make_child` on the frozen env returns a fresh `NodeEnvironmentBuilder` (not `&mut NodeEnvironmentT` — that's infeasible on `&'t`-allocated envs, as the reasoning doc notes). Scala's `makeChild` returns a new case class; Rust returns a builder.

## Step-by-step plan

### Step 1: Confirm starting point

```bash
cd /Volumes/V/Sylvan
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-4.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-4.txt
```

Must print `0`. If it doesn't, stop and ask the senior. The project sets `#![allow(unused_variables, unused_imports)]`, so don't rely on warning counts.

### Step 2: Stub real-interner Inner skeleton + flip to `TypingInterner<'s, 't>`

Before writing any env data, stand up the `TypingInterner` internal skeleton (empty HashMaps, bump, `RefCell`) and add the second lifetime parameter.

Concrete changes:

1. **Rewrite `typing_interner.rs`** to have:
   ```rust
   pub struct TypingInterner<'s, 't> {
       bump: &'t Bump,
       inner: RefCell<Inner<'s, 't>>,
   }
   struct Inner<'s, 't> {
       // Will be filled in Step 6 — for now, just one or two HashMap fields as placeholders
       // so the struct actually carries 's. Use _phantom if you can't think of one.
       _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
   }
   impl<'s, 't> TypingInterner<'s, 't> {
       pub fn new(bump: &'t Bump) -> Self { /* ... */ }
       pub fn alloc<T>(&self, val: T) -> &'t mut T { self.bump.alloc(val) }
       pub fn alloc_slice_copy<T: Copy>(&self, src: &[T]) -> &'t [T] {
           self.bump.alloc_slice_copy(src)
       }
       // intern_id / intern_prototype / intern_signature stay as panic!() stubs;
       // signatures updated to use <'s, 't> instead of <'t>.
   }
   ```
2. **Flip `Compiler<'s, 'ctx, 't>.typing_interner`** from `&'ctx TypingInterner<'t>` to `&'ctx TypingInterner<'s, 't>`. Update `Compiler::new`'s parameter type accordingly. Find the driver/top-level callers: `rg -n "TypingInterner<'t>|TypingInterner::new" src/` — flip every hit to `TypingInterner<'s, 't>`. The driver pattern will look like:
   ```rust
   let typing_bump = Bump::new();
   let typing_interner = TypingInterner::new(&typing_bump);
   let compiler = Compiler::new(&scout_arena, &typing_interner, &keywords, &opts);
   ```
3. **Every sub-compiler method that references `&TypingInterner<'t>`** gets flipped to `&TypingInterner<'s, 't>`. There are ~10-30 sites; most are in `src/typing/typing_interner.rs` references across the sub-compiler files. Pure mechanical sed; `cargo check --lib` catches missed ones.
4. `cargo check --lib` — should pass. You've only changed signatures; the method bodies still panic.

Don't wire intern method bodies yet (Step 6). Don't touch env data yet (Steps 3-5).

### Step 3: Variable types first (smallest unit of work)

Fill `IVariableT`, `ILocalVariableT`, and the four concrete variable structs in `env/function_environment_t.rs` per Gotcha 16. Write `From`/`TryFrom` bridges. This unblocks expression AST downstream uses.

`cargo check --lib`. Fix downstream compile errors at reference sites — most will be "match arms over `ILocalVariableT` now have two real variants." Wherever a match is incomplete and the body is a sub-compiler method, add a `_ => panic!("Unimplemented: Slab 8")` arm for Slab 8 to fill.

### Step 4: `IEnvEntryT`, `TemplatasStoreT`, `GlobalEnvironmentT`

Fill these three in dependency order:
1. `IEnvEntryT` (Gotcha 6) — inline 5-variant enum in `env/i_env_entry.rs`. Delete the five individual struct stubs. Keep their Scala `/* */` blocks.
2. `TemplatasStoreT` + `TemplatasStoreBuilder` (Gotcha 3) — in `env/environment.rs`. Note the builder's heap `HashMap<&'s IImpreciseNameS, Vec<IEnvEntryT>>` for imprecise entries, frozen to nested slices at `build_in`.
3. `GlobalEnvironmentT` (Gotcha 5) — in `env/environment.rs`. No builder needed at this slab level; it's constructed once at the start of the typing pass (actual construction is Slab 7 territory when `run_typing_pass` is written).

`cargo check --lib`.

### Step 5: Fill the 9 env variants + their builders

Order for smallest blast radius:
1. `PackageEnvironmentT` (no parent, simplest).
2. `CitizenEnvironmentT`, `ExportEnvironmentT`, `ExternEnvironmentT` (parent is `&'t PackageEnvironmentT` for Export/Extern, `IEnvironmentT` for Citizen — all simple).
3. `GeneralEnvironmentT`.
4. `BuildingFunctionEnvironmentWithClosuredsT`, `BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT`.
5. `FunctionEnvironmentT` + `FunctionEnvironmentBuilder`.
6. `NodeEnvironmentT` + `NodeEnvironmentBuilder`.

After each, `cargo check --lib`. Fix downstream errors conservatively with `panic!("Unimplemented: Slab 8")` where body logic would be needed.

Write `From`/`TryFrom` bridges for `IEnvironmentT` ↔ `IInDenizenEnvironmentT` and concrete → wrapper after all 9 env structs exist.

**Delete** `NodeEnvironmentBox`, `FunctionEnvironmentBoxT`, and the `IDenizenEnvironmentBoxT` trait stub per Gotcha 4. Fix reference sites.

### Step 5.5: Flip Slab-3 heavy-templata env refs from `&'s` to `&'t`

Per Gotcha 11: change the 5 heavy-templata/OverloadSet env-ref field types. Carve-out from the "don't touch" list.

1. `FunctionTemplataT.outer_env: &'s IEnvironmentT<'s, 't>` → `&'t`
2. `StructDefinitionTemplataT.declaring_env: &'s IEnvironmentT<'s, 't>` → `&'t`
3. `InterfaceDefinitionTemplataT.declaring_env: &'s IEnvironmentT<'s, 't>` → `&'t`
4. `ImplDefinitionTemplataT.env: &'s IEnvironmentT<'s, 't>` → `&'t`
5. `OverloadSetT.env: &'s IInDenizenEnvironmentT<'s, 't>` → `&'t`

Each flip will cascade through construction sites. Use `panic!("Unimplemented: Slab 8")` to short-circuit any sub-compiler body that breaks; goal is `cargo check --lib` clean.

Also confirm the Slab-3 custom `PartialEq`/`Eq`/`Hash` impls on heavy templatas (`ptr::eq` on the env ref) still type-check with the lifetime flipped — they should, because `ptr::eq` doesn't care which arena the ref points into. Sanity-check with `cargo check --lib`.

### Step 6: Fill `TypingInterner` bodies

Largest chunk of Slab 4 — probably ~800-1500 lines. Order matters (family-level before per-concrete wrappers, because the wrappers dispatch through the family-level methods):

1. **Create the 3 new union Val enums** per Gotcha 2: `INameValT<'s,'t,'tmp>` in `names/names.rs`, `InternedKindPayloadValT<'s,'t>` + `InternedKindPayloadT<'s,'t>` in `types/types.rs`, `InternedTemplataPayloadValT<'s,'t,'tmp>` + `InternedTemplataPayloadT<'s,'t>` in `templata/templata.rs`. Derives: `Copy, Clone, Hash, PartialEq, Eq, Debug`. For the `'tmp`-bearing ones, write manual `Hash`/`PartialEq` impls if derive misbehaves across `'tmp` variants (should work given the per-variant Vals already derive cleanly; derive first, manual only if it fails).

2. **Add the 6 HashMap fields to `Inner<'s, 't>`**, replacing the Step 2 placeholder `_phantom`. Use `hashbrown::HashMap` for the `'tmp`-bearing families, `std::HashMap` for `InternedKindPayloadValT` (no slices).

3. **Write the 6 `intern_<family>` methods** — the real work. Each does `inner.borrow().map.get(&val)` → if present, return cloned canonical; if absent, `alloc_<family>_canonical` (match on Val variant, arena-alloc the concrete, wrap in the canonical-enum variant), then `inner.borrow_mut().map.insert(val, canonical)`. Mirror `scout_arena.rs::intern_rune` (L329) and `intern_name` (L245).

4. **Write `intern_id` / `intern_prototype` / `intern_signature`** — these are top-level singletons (not family-dispatched), same shape as the family methods but simpler since there's only one Val type per map.

5. **Write the ~75 per-concrete wrapper methods** — all mechanical wrap/dispatch/unwrap. Consider a `macro_rules!` helper here; Slab 2 may have similar macros for `From`/`TryFrom` patterns worth reusing.

`cargo check --lib` after each sub-step. Errors at this point are typically query-wrapper type mismatches — re-read the scout_arena.rs `RuneValQuery` pattern until the query/canonical shapes line up.

`cargo check --lib` after each family. Compile errors at this point are typically "query wrapper type mismatch" — re-read the scout_arena.rs pattern until the query/canonical shapes line up.

### Step 7: Wire builder `build_in` methods

With the interner lit up, finish each builder's `build_in` impl so they actually allocate into the arena. Before Step 6 they could `panic!()`; now they should work.

`cargo check --lib`. Compile errors downstream here are typically "this sub-compiler call site used to construct a `PhantomData` env stub; now needs a real builder." Replace with `panic!("Unimplemented: Slab 8 — construct real builder")` and move on.

### Step 8: Verify and hand off

```bash
cd /Volumes/V/Sylvan
cargo check --lib --manifest-path FrontendRust/Cargo.toml > /tmp/sylvan-slab-4.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-4.txt   # must be 0
grep -rn "_Phantom\|PhantomData<(&'s (), &'t ())>" FrontendRust/src/typing/env/
#   ^ should show few or zero hits — only the handful of genuinely lifetime-anchoring
#   PhantomData fields (e.g. on `LocationInFunctionEnvironmentT`), not placeholder stubs.
grep -rn "PhantomData" FrontendRust/src/typing/typing_interner.rs
#   ^ should show zero hits — TypingInterner is now a real struct with Bump + HashMap.
```

**Never commit. The human handles all commits and tags.** When `cargo check --lib` is clean, skim your own diff one more time, then hand the slab back for review with uncommitted changes in the working tree. The human reviews, directs any changes, and is the one who runs `git commit` / `git tag`. If you need a local savepoint mid-slab (end of day, experimenting with an approach you might throw away), use `git stash` or a WIP branch — don't commit on `rustmigrate-z`.

The step-order above is a **work-organization guide** — treat each step as a natural self-review savepoint where you run `cargo check --lib`, skim your own diff, and decide whether it's tight enough to move on to the next step.

Work-order checkpoints:
- Step 2 — interner `<'s, 't>` flip + substrate lands (empty HashMaps, `alloc`/`alloc_slice_copy` wrappers, `Compiler::new` update).
- Step 3 — variable types + their `From`/`TryFrom` bridges.
- Step 4 — `IEnvEntryT` + `TemplatasStoreT` + `GlobalEnvironmentT`.
- Step 5 — 9 env variants + id-based `Hash`/`PartialEq` impls + builders + `IEnvironmentT` ↔ `IInDenizenEnvironmentT` bridges + `NodeEnvironmentBox` / `FunctionEnvironmentBoxT` / `IDenizenEnvironmentBoxT` deletion.
- Step 5.5 — 5 `&'s` → `&'t` env-ref flips in `templata.rs` / `types.rs`.
- Step 6 — interner bodies (the big one; split mentally by family if it helps stay organized).
- Step 7 — builder `build_in` implementations wired to interner.
- Step 8 — final `cargo check --lib` green, diff self-review, hand back to the human for review.

## What "done" looks like

- `cargo check --lib` passes with 0 errors.
- Every `*EnvironmentT` type in `env/environment.rs` and `env/function_environment_t.rs` has real Scala-parity fields (no `PhantomData`-only stubs).
- `IEnvironmentT<'s, 't>` has 9 `&'t _`-holding variants; `IInDenizenEnvironmentT<'s, 't>` has 6.
- `From`/`TryFrom` bridges: concrete ↔ `IEnvironmentT`, concrete ↔ `IInDenizenEnvironmentT`, `IInDenizenEnvironmentT` ↔ `IEnvironmentT`.
- `IEnvEntryT<'s, 't>` is a 5-variant inline enum. Old 5 struct stubs deleted; their Scala `/* */` blocks stay.
- `NodeEnvironmentBox`, `FunctionEnvironmentBoxT`, and `IDenizenEnvironmentBoxT` Rust stubs deleted.
- `IVariableT<'s, 't>` (4 variants), `ILocalVariableT<'s, 't>` (2 variants), and 4 concrete variable structs have real fields.
- `GlobalEnvironmentT<'s, 't>` exists as a real struct with `name_to_top_level_environment` and `builtins` fields.
- `TemplatasStoreT<'s, 't>` has slice-of-pairs layout per Gotcha 3. `TemplatasStoreBuilder<'s, 't>` exists with a `build_in` method.
- `NodeEnvironmentBuilder`, `FunctionEnvironmentBuilder`, etc. exist for each env kind that mutates during construction. Each has a `build_in` returning `&'t FooEnvironmentT`.
- `TypingInterner<'t>` has real bodies (`Bump` + per-family HashMaps + `RefCell<Inner>`). All three Slab 2/3 intern methods (`intern_id`, `intern_prototype`, `intern_signature`) plus ~60 concrete-name + 6 Kind-payload + 6 templata-payload methods actually work.
- All Scala `/* */` blocks unchanged byte-for-byte.
- Tagged `slab-4-complete`.

## When you're stuck

- **"Cannot construct `IEnvironmentT` — it has private fields"**: the fields are `&'t _`, not private. Double-check you're using the right constructor (`IEnvironmentT::Function(&'t FunctionEnvironmentT)`, not `IEnvironmentT { function: … }`).
- **"`'t` does not outlive `'s`"**: you wrote `&'s FooEnvironmentT`. Change to `&'t FooEnvironmentT`. See Gotcha 1.
- **"HashMap lookup fails — query wrapper mismatch"**: the IDEPFL pattern has a `Query` wrapper type that wraps `&'tmp ValT<'s, 't, 'tmp>` and implements `Hash`/`Eq` compatibly with the stored `ValT<'s, 't, 't>`. Read `src/postparsing/names.rs`'s `RuneValQuery` for the live example. You need one per interned family that uses `'tmp`.
- **"Sub-compiler file X won't compile because NodeEnvironmentBox / FunctionEnvironmentBoxT doesn't exist"**: replace param types at those call sites. `&NodeEnvironmentBox<'s, 't>` → `&mut NodeEnvironmentBuilder<'s, 't>`; `&FunctionEnvironmentBoxT<'s, 't>` → `&mut FunctionEnvironmentBuilder<'s, 't>`. Body panics until Slab 8. If unsure, ask the senior; don't guess the right type.
- **"`OverloadSetT` stops compiling after I change `IInDenizenEnvironmentT`"**: two possible causes. (a) You lost the `Hash`/`Eq`/`PartialEq`/`Debug` derive on the wrapper enum itself — see Gotcha 10. (b) You haven't done the Step 5.5 `&'s` → `&'t` flip on `OverloadSetT.env`, so the field type still references the old scout-lifetime env — see Gotcha 11. The Slab-3 custom `ptr::eq`-based `PartialEq` impl on `OverloadSetT` stays the same; lifetime doesn't affect `ptr::eq`.
- **"I need `TypingBump` or `&'t Bump` and don't know where to get it"**: add a getter method on `TypingInterner`, or pass it as a separate constructor/build_in param. Scout_arena.rs does the former.
- **"Gotcha about a Scala method I need to port"** (`rootCompilingDenizenEnv`, `addEntry`, `makeChild`, etc.): don't. That's body work. Scala block stays in `/* */`; Rust body is `panic!()` until Slab 8.
- **"I want to modify `names.rs` / `types.rs` / `templata.rs` / `ast.rs`"**: don't. Slabs 2 and 3 are frozen. If you think a data definition there needs to change, the shape of Slab 4 probably shifted — ask the senior.

## Where to file questions

- **Design** (field layout, enum shape, arena lifetime): the reasoning doc + this handoff + `quest.md` Part 3 should cover it. If they don't, ask the senior.
- **Scala semantics**: the `/* */` block is the spec. If ambiguous, the senior has the Scala repo.
- **Interner implementation**: scout_arena.rs is the live reference. If you hit a wall on `'tmp` lifetime mechanics, read IDEPFL shield + scout_arena's `intern_rune` + the `@DSAUIMZ` reasoning doc on slice interning.
- **Hook rejections**: read the hook's diff output. Usually it's accidental whitespace change inside `/* */`.

## Final advice

This is the biggest slab so far. The data-definition parts (envs, variables, IEnvEntryT, GlobalEnvironmentT, TemplatasStoreT) follow the Slab 2/3 patterns you've already seen — mechanical once you know the rules. The fresh work is the interner implementation and the builder-freeze pattern.

**Get the interner substrate (Step 2) working before you touch env structs.** If interner signatures change mid-slab, env `build_in` methods have to change too. One-way-door ordering: interner first, then data.

**When a sub-compiler reference site breaks, default to `panic!("Unimplemented: Slab 8 signature rewrite")`.** Resist the urge to write correct bodies — that's not Slab 4's job. The goal here is shape, not behavior.

**The reasoning doc (`environments-per-denizen-long-term.md`) captures why this slab is arena-based even though `Rc` would fit Scala better.** If you find yourself thinking "shouldn't this just be an `Rc`?" — yes, eventually, after Slab 8. Not now. Read the reasoning doc's "Why not do this during migration" section if you need the argument.

Good luck.
