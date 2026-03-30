---
model: AgenticSmall
description: Arena-allocated output types must not derive Clone — they are shared by reference, never duplicated.
---

# ArenaTypesDontClone (ATDCX)

Arena-allocated types must not `#[derive(Clone)]`. Types that live in arenas (`ParseArena<'p>`, `ScoutArena<'s>`) are accessed via `&'p` or `&'s` references and should never be duplicated.

## What counts as an arena type

Any struct or enum that is allocated into a `bumpalo::Bump` arena and accessed by reference thereafter. This includes:

- Postparsing AST output nodes: `StructS`, `InterfaceS`, `ImplS`, `FunctionS`, `ParameterS`, `GenericParameterS`, `FileS`, `ProgramS`, etc.
- Their inner types: attribute variants (`SealedS`, `BuiltinS`, `ExportS`, `PureS`, `AdditiveS`, etc.), body variants (`CodeBodyS`, `ExternBodyS`, `AbstractBodyS`, `GeneratedBodyS`), member variants (`NormalStructMemberS`, `VariadicStructMemberS`), generic parameter type variants (`RegionGenericParameterTypeS`, `CoordGenericParameterTypeS`, `OtherGenericParameterTypeS`)
- Parser AST output nodes: `FileP`, `FunctionP`, `StructP`, `InterfaceP`, etc.
- Higher typing output nodes: `StructA`, `FunctionA`, `InterfaceA`, etc.

## Exceptions

- **Copy types** (`StrI`, `RangeS`, `CodeLocationS`, `FileCoordinate`, `PackageCoordinate`, interned handles): These are trivially copyable. Clone is harmless and auto-derived from Copy.
- **Value types used as HashMap keys or stored in collections by value** (`IRuneS`, `INameS`, `IImpreciseNameS`, `RuneUsage`, `IVarNameS`): These are not arena output nodes. They are small tagged-pointer enums or structs that are intentionally passed by value and need Clone.
- **Transitive Clone on enum variant payloads**: If an enum like `IRuneS` legitimately needs Clone, its variant payload structs (`CodeRuneS`, `ImplicitRuneS`, etc.) must also derive Clone. This is acceptable.

## Why

Arena data is shared by reference, never duplicated. Deriving Clone on arena types:
1. Creates a false affordance — it suggests cloning is a valid operation when it never should be
2. Wastes memory if accidentally called — the clone goes on the heap, not in the arena
3. Blocks future enforcement — if Clone exists, nothing prevents someone from calling it

// V: we should have something that checks that weve put every shield in either include_shields or exclude_shields
// V: lets have a filter on this. lets maybe run it only during review.