# The Two Arenas

The compiler frontend uses two bump arenas (`bumpalo::Bump`), each with its own lifetime and interning maps. Data flows from parser to postparser to higher typing, with each pass allocating into its own arena.
// V: we might be moving toward an immutable arena model, need to incorporate that into this doc probably

## `'p` — Parser arena

**Owned by:** A local `Bump` created by `pass_manager::build()` (or a local `Bump` in tests). Wrapped in `ParseArena<'p>` which provides interning maps on top.

**Contains:** All interned strings (`StrI<'p>`), package coordinates (`PackageCoordinate<'p>`), file coordinates (`FileCoordinate<'p>`), parser AST nodes (`FileP`, `FunctionP`, `StructP`, `IExpressionPE`, `ITemplexPT`, etc.).

**Lifetime relationship:** Self-contained. No dependency on other arenas.

**Access:** `parse_arena.intern_str(...)`, `parse_arena.intern_package_coordinate(...)`, `parse_arena.intern_file_coordinate(...)`, `parse_arena.bump()` returns `&'p Bump`.

**Note:** The postparser reads `'p` data as input but doesn't write to the parser arena (except for synthetic parser AST nodes via `parse_arena` — see @PPSPASTNZ).

// V: the goal is that we should be able to drop this after the postparser runs. possible?

## `'s` — Scout (postparser + higher typing) arena

**Owned by:** A local `Bump` created by `pass_manager::build()` (or a local `Bump` in tests). Wrapped in `ScoutArena<'s>` which provides interning maps for strings, coordinates, names, runes, and imprecise names.

**Contains:** All postparser output (`StructS`, `FunctionS`, `IExpressionSE`, `IRulexSR`, etc.) and all higher typing output (`StructA`, `FunctionA`, `InterfaceA`, etc.). Also interned names (`INameS<'s>`, `IRuneS<'s>`, `IImpreciseNameS<'s>`), `ArenaIndexMap` instances, and arena slices (`&'s [T]`).

**Lifetime relationship:** Self-contained. Data from `'p` is re-interned (copied) into `'s` at the pass boundary.

**Access:** `scout_arena.intern_str(...)`, `scout_arena.intern_rune(...)`, `scout_arena.intern_name(...)`, `scout_arena.intern_imprecise_name(...)`, `scout_arena.bump()` returns `&'s Bump`.

// V: the goal is that we should be able to drop this after the typing pass runs. possible?

## Data flow

```
Source code
    │
    ▼
Parser ──── allocates into 'p arena ────► FileP, FunctionP, IExpressionPE, ...
    │                                      (StrI<'p>, PackageCoordinate<'p>)
    ▼
PostParser ── allocates into 's arena ──► StructS, FunctionS, IExpressionSE, ...
    │                                      (re-interns StrI<'p> → StrI<'s>,
    │                                       references 's for runes/names/rules/exprs)
    ▼
HigherTyping ── allocates into 's arena ─► StructA, FunctionA, InterfaceA, ...
                                            (same 's arena as postparser)
```

## Cross-pass data translation

At the parser→postparser boundary, `StrI<'p>` values are re-interned into `'s` via `scout_arena.intern_str(name_p.as_str())`. Package and file coordinates are similarly re-interned. This happens at ~30 individual sites throughout the postparser, wherever parser node data is used to build scout names/runes.
