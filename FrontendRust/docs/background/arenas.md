# Arena Allocation

The compiler frontend uses two bump arenas (`bumpalo::Bump`), each self-contained with its own lifetime and interning maps.

## `'p` — Parser Arena

Owned by `ParseArena<'p>`. Contains interned strings (`StrI<'p>`), package/file coordinates, and all parser AST nodes (`FileP`, `FunctionP`, `IExpressionPE`, `ITemplexPT`, etc.).

Access: `parse_arena.intern_str(...)`, `parse_arena.intern_package_coordinate(...)`, `parse_arena.bump()`.

The postparser reads `'p` data as input but allocates into `'s` (except synthetic parser nodes — see @PPSPASTNZ).

## `'s` — Scout Arena

Owned by `ScoutArena<'s>`. Contains all postparser output (`StructS`, `FunctionS`, `IExpressionSE`, `IRulexSR`, etc.), all higher typing output (`StructA`, `FunctionA`, `InterfaceA`, etc.), and interned names/runes (`INameS<'s>`, `IRuneS<'s>`, `IImpreciseNameS<'s>`).

Access: `scout_arena.intern_str(...)`, `scout_arena.intern_rune(...)`, `scout_arena.intern_name(...)`, `scout_arena.bump()`.

## Data Flow

```
Source code
    |
    v
Parser --- allocates into 'p arena ---> FileP, FunctionP, IExpressionPE, ...
    |                                    (StrI<'p>, PackageCoordinate<'p>)
    v
PostParser --- allocates into 's arena -> StructS, FunctionS, IExpressionSE, ...
    |                                      (re-interns StrI<'p> -> StrI<'s>)
    v
HigherTyping --- allocates into 's arena -> StructA, FunctionA, InterfaceA, ...
                                             (same 's arena as postparser)
```

At the parser->postparser boundary, `StrI<'p>` values are re-interned into `'s` via `scout_arena.intern_str(name_p.as_str())`. Coordinates are similarly re-interned.

## Key Invariant

Arena-allocated structs are **immutable after construction**. Data is built using mutable heap collections, then frozen into the arena. See `docs/usage/arenas.md` for the pattern.

## Transient vs Permanent

Interned types (runes, names, imprecise names) have two forms: a **transient Val** used to check "does this already exist?" and a **permanent** arena-allocated canonical form. Transient Vals live on the stack and are discarded on an intern hit, or promoted to permanent on a miss. A Val struct defines an **internable unit** — the boundary around everything that gets interned together. Slices and collections inside a Val are parts of that unit, not independently interned. When a transient Val contains a slice, the slice borrows from a stack temporary (lifetime `'tmp`) so that no arena space is wasted on hits. See @DSAUIMZ for the full pattern.
