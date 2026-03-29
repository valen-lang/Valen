# Arena Architecture

## ParseArena and ScoutArena

Each arena struct wraps a `&Bump` and owns `RefCell<HashMap<...>>` interning maps:

- **`ParseArena<'p>`**: strings, package coordinates, file coordinates
- **`ScoutArena<'s>`**: strings, package coordinates, file coordinates, names (`INameS`), runes (`IRuneS`), imprecise names (`IImpreciseNameS`)

Interning maps use `HashMap::with_capacity(64)` to avoid rehashing during keyword interns at pass startup.

## Why Immutability Matters

Arena data is never mutated after construction:
- **No resizing.** `ArenaIndexMap` hash tables are allocated once at correct size.
- **No dangling pointers.** Nothing points to data that might move.
- **Bulk deallocation.** Dropping the arena frees everything. No individual destructors.

This is enforced by convention (fields are `pub`), not the type system.

## Working Accumulators (NOT Arena-Allocated)

These hold `HashMap`/`Vec` fields but live on the stack or heap — they build data that eventually freezes into arenas:

- `Astrouts` — higher typing accumulator, stack-local `&mut`
- `EnvironmentA` — higher typing scope context, created functionally per scope
- `EnvironmentS`, `FunctionEnvironmentS` — postparser scope contexts, cloned and boxed
- `StackFrame` — expression scouting context
- `LocationInDenizenBuilder` — mutable builder for `LocationInDenizen`
- `VariableDeclarations`, `VariableUses` — transient accumulators
- Error types, solver state — returned via `Result` or mutated during solving

## Arena-Allocated Structs

**Scout arena (`'s`):** `StructS`, `InterfaceS`, `ImplS`, `FunctionS`, `GenericParameterS`, `ParameterS`, `ExportAsS`, `ImportS`, all `IExpressionSE` variants, all `IRulexSR` variant structs, `StructA`, `InterfaceA`, `ImplA`, `FunctionA`, `ExportAsA`, `ProgramA`, all `IRuneS`/`INameS`/`IImpreciseNameS` variant payloads.

**Parse arena (`'p`):** `PackageCoordinate<'p>`, `FileCoordinate<'p>`.

**Inline (not arena-allocated, not heap):** `CodeLocationS`, `RangeS` — fully `Copy`, stored directly in parent structs.

## The `'x` Generic Lifetime Pattern

Types that live in multiple arenas use a generic `'x` instead of a specific arena lifetime. `LocationInDenizen<'x>` is the model: it holds `&'x [i32]` and `'x` unifies with the owner's arena at each use site. This avoids duplicating types per arena.
