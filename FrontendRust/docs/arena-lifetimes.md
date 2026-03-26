# The Three Arenas

The compiler frontend uses three bump arenas (`bumpalo::Bump`), each with its own lifetime. Data flows from parser to postparser to higher typing, with each pass allocating into its own arena.
// V: we might be moving toward an immutable arena model, need to incorporate that into this doc probably
// V: should we think of interning as not its own arena, but instead in any particular arena? after all interning is just a map on top of an arena. this might also make it cleaner because right now we have typing-only names in the postparser which is weird.

## `'a` — Interner arena (longest-lived)

**Owned by:** The `Interner` struct (created at the start of compilation).

**Contains:** All interned/canonicalized data — strings (`StrI<'a>`), names (`INameS<'a>`, `IRuneS<'a>`, `IImpreciseNameS<'a>`), package coordinates, file coordinates. Also the rune variant payloads like `ImplicitRuneS<'a>`, `CodeRuneS<'a>`.

**Lifetime relationship:** `'a` outlives everything else. All other arenas and data reference `'a` data freely.

**Access:** `interner.arena()` returns `&'a Bump`. Most code accesses the interner via `self.interner` on `PostParser` or `HigherTypingPass`.

// V: we might want to rename 'a to 'i

## `'p` — Parser arena

**Owned by:** The `ParserCompilation` (or a local `Bump` in tests).

**Contains:** Parser AST nodes — `FileP`, `FunctionP`, `StructP`, `IExpressionPE`, `ITemplexPT`, etc. These are the raw parse tree from source code.

**Lifetime relationship:** `'a: 'p` (interner outlives parser). Parser nodes reference interned strings but not scout data.

**Note:** The postparser reads `'p` data as input but doesn't write to the parser arena.

// V: the goal is that we should be able to drop this after the postparser runs. possible?

## `'s` — Scout (postparser + higher typing) arena

**Owned by:** Created as a local `Bump` by the compilation entry point, passed to `PostParser::new()` and `HigherTypingPass::new()`.

**Contains:** All postparser output (`StructS`, `FunctionS`, `IExpressionSE`, `IRulexSR`, etc.) and all higher typing output (`StructA`, `FunctionA`, `InterfaceA`, etc.). Also `ArenaIndexMap` instances and arena slices (`&'s [T]`).

**Lifetime relationship:** `'a: 's` (interner outlives scout). Scout data references interned names/runes from `'a` and other scout data from `'s`.

**Access:** `self.scout_arena` on `PostParser` and `HigherTypingPass`.

// V: the goal is that we should be able to drop this after the typing pass runs. possible?

## Data flow

```
Source code
    │
    ▼
Parser ──── allocates into 'p arena ────► FileP, FunctionP, IExpressionPE, ...
    │                                      (references 'a for interned strings)
    ▼
PostParser ── allocates into 's arena ──► StructS, FunctionS, IExpressionSE, ...
    │                                      (references 'a for runes/names,
    │                                       references 's for rules/exprs)
    ▼
HigherTyping ── allocates into 's arena ─► StructA, FunctionA, InterfaceA, ...
                                            (same 's arena as postparser)
```
