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
- Env builders (`NodeEnvironmentBox` etc.) — stack-local with heap `Vec`s, freeze into `'t`.

### Env mutation pattern

`NodeEnvironmentT` mutation during expression scouting uses a builder → freeze pattern. Each new local or scope change produces a fresh builder, frozen into the arena as a new `&'t NodeEnvironmentT`. Matches Scala's `NodeEnvironmentBox` semantics (which also allocates a new case class per mutation). Allocation churn is equivalent to Scala's; bumpalo makes it visibly fast rather than GC-hidden.

### Lookup pattern

`TemplatasStoreT` uses arena-allocated slice pairs, not `HashMap`. Per AASSNCMCX arena-allocated structs cannot hold heap collections (because `bumpalo::Bump` doesn't run destructors). Lookup is linear scan. Acceptable for typical small scopes; switching hot scopes to binary search or moving envs off the arena entirely are both future options.

## Where this is heading

This is the migration-phase arena shape. The target is a per-denizen two-tier design — a program-wide outputs arena for resolved definitions + skeleton envs, and per-top-level-denizen scratchpad arenas for working envs + transient templatas — plus an eventual LSP path built on top of that. Full design, cross-denizen edge audit, required refactors, migration path, and LSP direction: `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md`.

## See also

- `FrontendRust/docs/reasoning/environments-per-denizen-long-term.md` — full long-term target design + cross-denizen audit findings.
- `FrontendRust/docs/architecture/arenas.md` — parser and scout arena architecture (`'p`, `'s`) — predecessors / inputs to this pass's arenas.
- `FrontendRust/docs/background/arenas.md` — high-level three-arena overview.
- `quest.md` — typing-pass migration design doc. Part 3 covers env architecture; see TL-HANDOFF overrides for corrections.
- `FrontendRust/docs/reasoning/idt-typed-view-alternatives.md` — `IdT` monomorphic / typed-view decision; related "chose X for migration, alternatives documented for later" pattern.
- `Luz/shields/ArenaAllocatedStructsShouldNotContainMallocdCollections-AASSNCMCX.md` — the shield that forces current slice-based `TemplatasStoreT` and would lift when envs move off arena.
