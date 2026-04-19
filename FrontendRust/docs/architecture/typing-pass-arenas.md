# Typing-Pass Arena Architecture

Typing-pass data lives across two arenas during migration: the `'s` scout arena (shared with earlier passes) for referenced higher-typing output, and the `'t` typing arena for interned and allocated typing-pass data. This doc describes the current shape and points at the reasoning doc that captures where the typing pass is headed long-term.

## Current model (migration-phase)

One `TypingInterner<'t>` for the whole pass. Everything typing-pass produces — interned names/kinds/coords/templatas, environments, `FunctionDefinitionT`, `HinputsT` — allocates into or through this single interner. Arena drop happens once at pass end.

### What lives in `'s` (scout arena, read-only from typing's perspective)

- `FunctionA<'s>`, `StructA<'s>`, `InterfaceA<'s>`, `ImplA<'s>` — higher-typing output, referenced directly by typing-pass types.
- Scout-interned names: `INameS<'s>`, `IRuneS<'s>`, `IImpreciseNameS<'s>`.
- `StrI<'s>`, `PackageCoordinate<'s>`, `FileCoordinate<'s>`, `RangeS<'s>`, `CodeLocationS<'s>`.

The typing pass treats `'s` as stable input data. Scout arena drops after typing finishes (actually after the instantiator finishes, since `HinputsT` still holds `&'s` refs).

### What lives in `'t` (typing arena)

Interned (deduplicated via `TypingInterner`):
- Concrete name structs (~60): `FunctionNameT`, `StructNameT`, etc.
- `IdT<'s, 't>` — monomorphic, always carries the widest local-name form.
- Concrete Kind payloads: `StructTT`, `InterfaceTT`, `StaticSizedArrayTT`, `RuntimeSizedArrayTT`, `KindPlaceholderT`, `OverloadSetT`.
- Interned templata payloads: `CoordTemplataT`, `KindTemplataT`, `PlaceholderTemplataT`, `PrototypeTemplataT`, `IsaTemplataT`, `CoordListTemplataT`.
- `PrototypeT<'s, 't>`, `SignatureT<'s, 't>`.

Allocated but not interned:
- `FunctionDefinitionT`, `FunctionHeaderT`.
- `StructDefinitionT`, `InterfaceDefinitionT`, `ImplT`, `EdgeT`, `OverrideT`.
- `ParameterT`, `ILocalVariableT`.
- `ReferenceExpressionTE` (~38 variants), `AddressExpressionTE` (~6 variants).
- Heavy templata payloads: `FunctionTemplataT`, `StructDefinitionTemplataT`, `InterfaceDefinitionTemplataT`, `ImplDefinitionTemplataT`, `ExternFunctionTemplataT`.
- **Environments** — `IEnvironmentT<'s, 't>` and all 9 concrete variants, allocated via `typing_arena.alloc(...)`. (`quest.md` §3.1 said `'s`; that was wrong — see the reasoning doc.)
- `HinputsT<'s, 't>`.

Inline `Copy`, not arena-allocated:
- Wrapper enums: `INameT` + 21 name sub-enums, `KindT` + 3 Kind sub-enums, `ITemplataT`.
- `CoordT<'s, 't>`, `OwnershipT`, `MutabilityT`, `VariabilityT`, `LocationT`, `RegionT`.
- Small templata value variants (Integer, Boolean, Mutability, etc.).

Neither arena (stack / heap-Vec / HashMap):
- `CompilerOutputs<'s, 't>` — accumulator with heap-backed HashMaps, dies at pass end.
- `Compiler<'s, 'ctx, 't>` — stack god struct, four `&'ctx` fields.
- Env builders (`NodeEnvironmentBuilder` etc.) — stack-local with heap `Vec`s, freeze into `'t`.

### Env mutation pattern

`NodeEnvironmentT` mutation during expression scouting uses a builder → freeze pattern. Each new local or scope change produces a fresh builder, frozen into the arena as a new `&'t NodeEnvironmentT`. Matches Scala's `NodeEnvironmentBox` semantics (which also allocates a new case class per mutation). Allocation churn is equivalent to Scala's; bumpalo makes it visibly fast rather than GC-hidden.

### Lookup pattern

`TemplatasStoreT` uses arena-allocated slice pairs, not `HashMap`. Per AASSNCMCX arena-allocated structs cannot hold heap collections (because `bumpalo::Bump` doesn't run destructors). Lookup is linear scan. Acceptable for typical small scopes; switching hot scopes to binary search or moving envs off the arena entirely are both future options.

## Where this is heading (long-term)

**See `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md`** for the post-migration target and the empirical cross-denizen edge audit that justifies the shape.

Summary of the long-term direction:

- **Split `'t` into two arenas.** A program-wide `'out` outputs arena (for resolved definitions, interned types, skeleton envs, `HinputsT`) and a per-top-level-denizen `'scratch` arena (for working envs, transient templatas, solver state). Each Phase-3 worklist item gets its own scratchpad, dropped when the item finishes.
- **Envs bifurcate by role.** `PackageEnvironmentT`, `CitizenEnvironmentT`, `BuildingFunctionEnvironmentWithClosuredsT` live in `'out` as declaration skeletons that other denizens read. `NodeEnvironmentT`, `FunctionEnvironmentT`, `BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT`, `GeneralEnvironmentT`, `ExportEnvironmentT`, `ExternEnvironmentT` live in `'scratch` — pure working state that dies with the denizen.
- **Side table + `EnvIdx`.** Arena-allocated types that reference a scratchpad env (`FunctionTemplataT.outer_env`, `OverloadSetT.env`) hold a `u32` index into a per-denizen `Vec<&'scratch IEnvironmentT>`. This crosses the scratchpad-env boundary without the `Drop`-in-arena leak hazard and without `Rc` overhead.
- **Single interner stays (for now).** `TypingInterner` remains globally scoped — per-denizen interning would force structural `Hash`/`Eq` on many types for modest gain in the batch-typing case.

### Even further out: LSP support

The per-denizen arena design is load-bearing for eventual LSP (language-server) support. Individual denizens become invalidation units — source change to denizen A drops A's `'out` slab + `'scratch` arena, recompile A, everyone else's outputs stay valid unless A's public surface cascaded.

Beyond per-denizen arenas, an LSP build needs:

- **`DefId`-indexed cross-denizen references** (rustc-style). Cross-denizen refs can't stay as `&'out` borrows once individual slabs drop independently; they become `(DenizenId, u32)` indices resolved via a top-level `CompilerState`. DefIds are fully-qualified names — stable across compiles, rename = delete+create.
- **Shatter `GlobalEnvironment` into DefId-keyed registries** on `CompilerState`. No single struct; individual package entries replaceable per-package.
- **Two-tier interner.** A global interner (session-persistent, shared by all denizens) plus a per-denizen local interner (transient, dies with the denizen compile). During compilation, local is the default target with global as a read-only fallback. At denizen compile end, a single-writer cleanup copy-walks outputs and promotes only externally-visible entries into global — the sole path for global writes.
- **Periodic global-interner rebuild** (rather than entry-level refcounting) when the interner bloats; can run as background work with atomic swap.

Full discussion, design mechanics, and open questions: `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md` §"Further future direction: LSP support".

Prerequisite refactors (planned as part of the post-Slab-8 slab):
1. `FunctionHeaderT.maybeOriginFunctionTemplata` → `maybeOriginFunctionA: Option<&'s FunctionA>` — severs the last `FunctionTemplataT` escape path, letting it move to scratchpad.
2. Delete `envByFunctionSignature` from `CompilerOutputs` (dead code).
3. Implement the `TypingInterner` bodies that are currently `panic!()` stubs (Slab 4 prerequisite anyway).

## Why bother with the long-term shape?

Three reasons, in increasing abstraction:

- **Concrete pain point:** `bumpalo`'s no-destructor semantics mean any arena-allocated struct with a heap-owning field (`HashMap`, `Vec`, `Rc`) leaks. Today this forces slice-based `TemplatasStoreT` with linear-scan lookup; long-term we want `HashMap`-in-env for O(1) method lookup in hot scopes.
- **Architectural pressure:** `NodeEnvironmentT` mutation is the highest-churn data in the pass. Keeping it in one program-wide arena means peak memory grows with the codebase. Per-denizen scratchpad bounds peak to the largest single function's working set.
- **Scala-parity framing:** Scala's typing has a natural per-compile-unit scope (each top-level denizen's `compile()` call). The migration-phase single-arena model pays attention to Rust storage needs but loses some of Scala's implicit scoping. Per-denizen arenas restore that scoping as an explicit mechanism.

## See also

- `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md` — full long-term target design + cross-denizen audit findings.
- `FrontendRust/docs/architecture/arenas.md` — parser and scout arena architecture (`'p`, `'s`) — predecessors / inputs to this pass's arenas.
- `FrontendRust/docs/background/arenas.md` — high-level three-arena overview.
- `quest.md` — typing-pass migration design doc. Part 3 covers env architecture; see TL-HANDOFF overrides for corrections.
- `FrontendRust/docs/reasoning/idt-typed-view-alternatives.md` — `IdT` monomorphic / typed-view decision; related "chose X for migration, alternatives documented for later" pattern.
- `Luz/shields/ArenaAllocatedStructsShouldNotContainMallocdCollections-AASSNCMCX.md` — the shield that forces current slice-based `TemplatasStoreT` and would lift when envs move off arena.
