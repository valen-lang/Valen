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

## Transient vs Permanent Data

Arena data is **permanent** — once allocated, it lives for the arena's lifetime (`'s` or `'p`) and is referenced by everything downstream.

**Transient** data exists only to build or look up permanent data, then is discarded:

- **Val types** (`IRuneValS`, `INameValS`, etc.) — transient lookup keys for interning. Built on the stack, checked against the intern map, then either discarded (hit) or promoted to permanent (miss). Val types with slices use a `'tmp` lifetime to borrow from stack temporaries, deferring arena allocation until a miss (see @DSAUIMZ).
- **Working accumulators** — hold `HashMap`/`Vec` fields on the stack or heap, build data that eventually freezes into arenas:
  - `Astrouts` — higher typing accumulator, stack-local `&mut`
  - `EnvironmentA` — higher typing scope context, created functionally per scope
  - `EnvironmentS`, `FunctionEnvironmentS` — postparser scope contexts, cloned and boxed
  - `StackFrame` — expression scouting context
  - `LocationInDenizenBuilder` — mutable builder for `LocationInDenizen`. Produces transient `LocationInDenizenVal` via `borrow_val()` or permanent `LocationInDenizen` via `consume_in()`
  - `VariableDeclarations`, `VariableUses` — transient accumulators
  - Error types, solver state — returned via `Result` or mutated during solving

## Arena-Allocated Structs

**Scout arena (`'s`):** `StructS`, `InterfaceS`, `ImplS`, `FunctionS`, `GenericParameterS`, `ParameterS`, `ExportAsS`, `ImportS`, all `IExpressionSE` variants, all `IRulexSR` variant structs, `StructA`, `InterfaceA`, `ImplA`, `FunctionA`, `ExportAsA`, `ProgramA`, all `IRuneS`/`INameS`/`IImpreciseNameS` variant payloads. For interned rune payloads containing `LocationInDenizen` (e.g., `ImplicitRuneS`), the inner `&'s [i32]` path slice is only arena-allocated on intern miss — per @DSAUIMZ, the transient Val borrows from the stack until promotion.

**Parse arena (`'p`):** `PackageCoordinate<'p>`, `FileCoordinate<'p>`.

**Inline (not arena-allocated, not heap):** `CodeLocationS`, `RangeS` — fully `Copy`, stored directly in parent structs.

## The `'x` Generic Lifetime Pattern

Types that live in multiple arenas use a generic `'x` instead of a specific arena lifetime. `LocationInDenizen<'x>` is the model: it holds `&'x [i32]` and `'x` unifies with the owner's arena at each use site. This avoids duplicating types per arena.
