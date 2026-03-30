---
model: AgenticSmall
description: Output data must be Copy or behind &'s — Clone-without-Copy on output data is a smell.
---

# ArenaTypesDontClone (ATDCX)

Output data (AST nodes, rules, names, attributes) must be either **Copy** (stored inline, trivially cheap to duplicate) or **behind `&'s`/`&'p`** (arena-allocated, accessed by reference, never duplicated). The smell to watch for is **Clone-without-Copy** — that means either the type should be Copy (and something is blocking it), or it's working state that belongs on the heap, not in the output.

Note: Rust requires `Clone` as a supertrait of `Copy`, so every `Copy` type will also have `Clone` in its derive list. That's fine — the compiler generates a trivial memcpy. The concern is only with types that have `Clone` *without* `Copy`.

## Output data categories

### Behind `&'s` — arena-allocated, accessed by reference

These must NOT derive Clone. They are allocated into the arena once and shared by reference:

- Postparsing AST output nodes: `StructS`, `InterfaceS`, `ImplS`, `FunctionS`, `ParameterS`, `GenericParameterS`, `FileS`, etc.
- Parser AST output nodes: `FileP`, `FunctionP`, `StructP`, `InterfaceP`, `IExpressionPE`, `ITemplexPT`, etc.
- Higher typing output nodes: `StructA`, `FunctionA`, `InterfaceA`, etc.
- Generic parameter type variants: `RegionGenericParameterTypeS`, `CoordGenericParameterTypeS`, `OtherGenericParameterTypeS`

### Copy — small value types stored inline

These derive `Copy` (and `Clone` as required supertrait). They're stored by value in slices or struct fields and are trivially cheap to duplicate:

- Interned handles: `StrI`, `IRuneS`, `INameS`, `IImpreciseNameS`, `IVarNameS`, `IFunctionDeclarationNameS` (all tagged pointers to arena data)
- Coordinates: `RangeS`, `CodeLocationS`, `RangeL`, `FileCoordinate`, `PackageCoordinate`
- Small structs: `RuneUsage`, `FunctionNameS`, `LambdaDeclarationNameS`, `LocationInDenizen`
- Attribute enums and their variants: `ICitizenAttributeS`, `IFunctionAttributeS`, `ExternS`, `PureS`, `SealedS`, `BuiltinS`, `MacroCallS`, `ExportS`, `UserFunctionS`, `AdditiveS`
- Member enums and their variants: `IStructMemberS`, `NormalStructMemberS`, `VariadicStructMemberS`
- Body enums: `IBodyS`, `CodeBodyS`, `ExternBodyS`, `AbstractBodyS`, `GeneratedBodyS`

## Known exceptions

- **`ProgramS`**: Currently Clone-without-Copy because `EnvironmentA` stores `PackageCoordinateMap<ProgramS>` by value and clones it on scope entry. All fields are `&'s` slices (Copy-eligible). Fix: change `EnvironmentA.code_map` to a `&'s` reference.

## Working state (not covered by this shield)

Working state types (`StackFrame`, `EnvironmentS`, `FunctionEnvironmentS`, `VariableDeclarations`, solver types) live on the heap and may legitimately need Clone for scope forking. These are not output data and are not covered by this shield.

## Why

Clone-without-Copy on output data:
1. Creates a false affordance — it suggests deep-copying is a valid operation when it never should be
2. Wastes memory if accidentally called — the clone goes on the heap, not in the arena
3. May hide expensive heap allocations (Vec, HashMap inside cloned structs)

// V: we should have something that checks that weve put every shield in either include_shields or exclude_shields
// V: lets have a filter on this. lets maybe run it only during review.
// V: actually maybe lets nuke this shield