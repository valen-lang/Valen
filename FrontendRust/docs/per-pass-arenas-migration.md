# Plan: Eliminate `'a` Interner Arena — Per-Pass Arenas With Copy-At-Boundary

## Context

The current codebase uses a long-lived `'a` interner arena that outlives all pass-specific arenas (`'p`, `'s`). This creates lifetime constraints (`'a: 'p`, `'a: 's`), forces multi-lifetime type parameters on nearly every struct (`<'a, 's>`, `<'a, 'p>`), and prevents freeing earlier arenas after their data has been consumed.

The new model: **each pass owns its own arena with its own interning maps. When a later pass needs data from an earlier pass, it copies (re-interns) it into its own arena.** This eliminates `'a`, gives every struct a single lifetime parameter, and allows arenas to be dropped as soon as the next pass has consumed their output.

This plan covers migrating the existing codebase (lexing through higher_typing). The typing pass design doc will be updated separately.

## The New Lifetime Model

```
BEFORE:  'a (interner, lives forever)
         'p (parser arena, 'a: 'p)
         's (postparser + higher_typing arena, 'a: 's)

AFTER:   'l (lexer arena — OR lexer stays heap-allocated, no arena)
         'p (parser arena, self-contained)
         's (postparser + higher_typing arena, self-contained)
         't (typing pass arena, self-contained — future work)
```

Each arena has **interning maps** on top of it (HashMap-based deduplication). When the postparser needs a `StrI` or `PackageCoordinate` from the parser, it copies the data into `'s` and gets back an `&'s`-lifetime reference. The interning maps ensure deduplication within each arena.

---

## Key Design Decisions

### 1. Per-pass arena structs (not a generic Interning<'x>)

Each pass gets its own arena struct with a Bump arena and the interning HashMaps it needs:

```rust
// AFTERM: figure out how to deduplicate all the common code across these interners
struct ParseArena<'p> {
    bump: &'p Bump,
    strings: RefCell<HashMap<String, StrI<'p>>>,        // pre-size for ~64 entries (keywords)
    package_coords: RefCell<HashMap<...>>,
    file_coords: RefCell<HashMap<...>>,
}

// AFTERM: figure out how to deduplicate all the common code across these interners
struct ScoutArena<'s> {
    bump: &'s Bump,
    strings: RefCell<HashMap<String, StrI<'s>>>,        // pre-size for ~64 entries (keywords)
    package_coords: RefCell<HashMap<...>>,
    file_coords: RefCell<HashMap<...>>,
    names: RefCell<HashMap<INameValS<'s>, INameS<'s>>>,
    runes: RefCell<HashMap<IRuneValS<'s>, IRuneS<'s>>>,
    imprecise_names: RefCell<HashMap<...>>,
}

// AFTERM: figure out how to deduplicate all the common code across these interners
struct TypingArena<'t> { ... }  // future work
```

String interning HashMaps should be pre-sized with `HashMap::with_capacity(64)` (or similar) to avoid rehashing during the ~40 keyword interns at pass startup.

No translation methods on the arena structs — translation is application logic in the pass compilation code.

### 2. Cross-pass data translation

At each pass boundary, the consuming pass's compilation logic re-interns data from the previous arena using the arena struct's intern methods:

```rust
// In postparser compilation logic (NOT on ScoutArena):
let s_str: StrI<'s> = scout_arena.intern_str(p_str.as_str());
let s_pkg: &'s PackageCoordinate<'s> = scout_arena.intern_package_coord(...);
let s_file: &'s FileCoordinate<'s> = scout_arena.intern_file_coord(...);
let s_range: RangeS<'s> = RangeS::new(
    CodeLocationS { file: s_file, offset: p_range.begin.offset },
    CodeLocationS { file: s_file, offset: p_range.end.offset },
);
```

This is mechanical and happens at pass boundaries.

### 3. What crosses the parser→postparser boundary

- `StrI` (string content) — re-interned into `'s`
- `PackageCoordinate`, `FileCoordinate` — re-interned into `'s`
- `RangeS`/`CodeLocationS` — reconstructed with `'s` FileCoordinate refs
- Parser AST nodes (`FileP`, `FunctionP`, etc.) — consumed by postparser, referenced as `'p` (read-only input, NOT copied)

The postparser reads `'p` data as input and produces `'s` data. `PostParser` keeps `'p` as an input lifetime: `PostParser<'p, 'ctx, 's>`. Postparser output types drop to just `<'s>`.

### 4. Source code map is lifetime-free

The `code_map_cache` (file contents) and `vpst_map_cache` (serialized JSON AST) currently live in `FileCoordinateMap<'a, String>`. Source code needs to outlive all passes for error humanizers.

**Resolution**: Source code map becomes `HashMap<String, String>` (filepath → contents) or similar, owned by the compilation driver with no arena lifetime. Error humanizers receive `&str` when they need to show a line. This is separated from `parseds_cache` which stays in `'p`.

### 5. Lexer uses `'p` arena

Lexer types (`FileL`, `StructL`, etc.) are heap-allocated and use `StrI` for interned strings. The lexer interns strings into the `'p` arena. The `'p` Bump arena and `ParseArena<'p>` are created before the lexer runs. The caller creates both and passes the `ParseArena` to the lexer and parser.

### 6. `IPackageResolver` uses `'p`

```rust
trait IPackageResolver<'p, T> {
    fn resolve(&self, package_coord: &'p PackageCoordinate<'p>) -> Option<T>;
}
```

The caller creates the `'p` arena, interns `PackageCoordinate`s into it, then passes them to the compilation pipeline. Straightforward rename from `'a` to `'p`.

### 7. `CodeLocationS`/`RangeS` — kill the `Arc`

`CodeLocationS<'a>` currently holds `Arc<FileCoordinate<'a>>`. Since `FileCoordinate` is interned (deduplicated) into the arena, we replace `Arc` with a plain arena reference:

```rust
pub struct CodeLocationS<'x> {
    pub file: &'x FileCoordinate<'x>,  // was Arc<FileCoordinate<'a>>
    pub offset: i32,
}
```

`CodeLocationS` and `RangeS` become fully `Copy` (12 bytes and 24 bytes respectively), eliminating heap allocation on every `eval_pos` call.

### 8. Keywords — fresh per pass

Each pass creates its own `Keywords<'x>` by interning ~40 string constants into its arena. Trivially cheap (40 hash lookups). No cross-pass dependency.

### 9. `LocationInDenizen<'x>` — already arena-parameterized

This type already uses a generic `'x` lifetime and works in multiple arenas. No change needed — it's the model for everything else.

---

## Struct-by-Struct Migration

### Legend
- **Before → After**: lifetime parameter change
- **Fields affected**: which fields change and how
- **Implications**: what breaks or needs updating

---

### CORE INFRASTRUCTURE

#### `StrI<'a>` → `StrI<'x>` (interner.rs:30)
- Already generic in principle (it's just `&'x str`)
- Before: always `'a`. After: `'p`, `'s`, or `'t` depending on which arena
- **Implication**: Every type containing `StrI<'a>` changes to `StrI<'x>` where `'x` is its arena

#### `InternedSlice<'a, T>` → `InternedSlice<'x, T>` (interner.rs:71)
- Just `&'x [T]` wrapper. Already generic.
- **Implication**: Same as StrI

#### `Interner<'a>` → eliminated (interner.rs:128)
- Currently owns the `'a` Bump and all interning maps
- **After**: Replaced by `ParseArena<'p>` and `ScoutArena<'s>` (and later `TypingArena<'t>`). Each has its own bump + interning maps.
- **Implication**: Major refactor. All call sites change from `interner.intern_str(...)` to `parse_arena.intern_str(...)` or `scout_arena.intern_str(...)`.

#### `InternerInner<'a>` → eliminated (interner.rs:155)
- Maps: `string_to_interned`, `package_coord_to_ref`, `file_coord_to_ref`, `imprecise_name_val_to_ref`, `name_val_to_ref`, `rune_val_to_ref`
- **After**: These maps are fields on the per-pass arena structs. String/coord maps on all arenas. Name/rune maps only on `ScoutArena`.

#### `Keywords<'a>` → `Keywords<'x>` (keywords.rs:7)
- ~40 `StrI<'a>` fields (func, impoort, export, truue, etc.)
- **After**: `Keywords<'x>` where `'x` is whichever arena. Created fresh per pass by interning keyword strings into that pass's arena.
- **Implication**: Each pass creates its own `Keywords`. Pre-size string HashMap to avoid rehashing.

#### `PackageCoordinate<'a>` → `PackageCoordinate<'x>` (utils/code_hierarchy.rs:106)
- Fields: `module: StrI<'a>`, `packages: InternedSlice<'a, StrI<'a>>`
- **After**: `PackageCoordinate<'x>` with `StrI<'x>`, `InternedSlice<'x, StrI<'x>>`
- **Implication**: Arena-allocated. Cross-pass translation copies module string + package strings.

#### `FileCoordinate<'a>` → `FileCoordinate<'x>` (utils/code_hierarchy.rs:51)
- Fields: `package_coord: &'a PackageCoordinate<'a>`, `filepath: StrI<'a>`
- **After**: `&'x PackageCoordinate<'x>`, `StrI<'x>`
- **Implication**: Arena-allocated. Translation copies package coord + filepath string.

#### `CodeLocationS<'a>` → `CodeLocationS<'x>` (utils/range.rs:43)
- Fields: `file: Arc<FileCoordinate<'a>>`, `offset: i32`
- **After**: `file: &'x FileCoordinate<'x>`, `offset: i32` — Arc eliminated, becomes fully `Copy` (12 bytes)
- **Implication**: Every `RangeS` changes. `eval_pos` becomes a pointer copy. `RangeS` becomes 24 bytes, fully `Copy`.

#### `RangeS<'a>` → `RangeS<'x>` (utils/range.rs:87)
- Fields: `begin: CodeLocationS<'a>`, `end: CodeLocationS<'a>`
- **After**: `CodeLocationS<'x>`
- **Implication**: Used in ~100+ structs. Purely mechanical rename.

#### `FileCoordinateMap<'a, Contents>` → `FileCoordinateMap<'x, Contents>` (utils/code_hierarchy.rs:295)
- Fields: `HashMap<&'a PackageCoordinate<'a>, Vec<&'a FileCoordinate<'a>>>`, `HashMap<&'a FileCoordinate<'a>, Contents>`
- **After**: All `'a` → `'x`. Stays in `'p` inside `ParserCompilation`. Postparser borrows with `'p` keys.
- **Implication**: `code_map_cache` and `vpst_map_cache` extracted to lifetime-free `HashMap<String, String>` for error humanizer access.

#### `PackageCoordinateMap<'a, Contents>` → `PackageCoordinateMap<'x, Contents>` (utils/code_hierarchy.rs:632)
- Fields: `HashMap<&'a PackageCoordinate<'a>, Contents>`
- **After**: `'a` → `'x`

#### `ArenaIndexMap<'bump, K, V>` → no change (utils/arena_index_map.rs:36)
- Already parameterized by `'bump`. No change needed.

#### `LocationInDenizen<'x>` → no change (postparsing/ast.rs:1148)
- Already uses generic `'x`. No change needed.

---

### LEXING (all currently `<'a>`, become `<'p>`)

Lexer types are heap-allocated and use `StrI<'a>` for interned strings. Under the new model, the lexer interns into the parser arena, so these become `<'p>`.

| Struct/Enum | File:Line | Change |
|---|---|---|
| `FailedParse<'a>` | lexing/errors.rs:5 | → `<'p>` |
| `FileL<'a>` | lexing/ast.rs:43 | → `<'p>` |
| `ImplL<'a>` | lexing/ast.rs:80 | → `<'p>` |
| `ExportAsL<'a>` | lexing/ast.rs:103 | → `<'p>` |
| `ImportL<'a>` | lexing/ast.rs:116 | → `<'p>` |
| `StructL<'a>` | lexing/ast.rs:133 | → `<'p>` |
| `InterfaceL<'a>` | lexing/ast.rs:156 | → `<'p>` |
| `FunctionL<'a>` | lexing/ast.rs:228 | → `<'p>` |
| `FunctionBodyL<'a>` | lexing/ast.rs:243 | → `<'p>` |
| `FunctionHeaderL<'a>` | lexing/ast.rs:255 | → `<'p>` |
| `ScrambleLE<'a>` | lexing/ast.rs:301 | → `<'p>` |
| `ParendLE<'a>` | lexing/ast.rs:361 | → `<'p>` |
| `AngledLE<'a>` | lexing/ast.rs:379 | → `<'p>` |
| `SquaredLE<'a>` | lexing/ast.rs:397 | → `<'p>` |
| `CurliedLE<'a>` | lexing/ast.rs:416 | → `<'p>` |
| `WordLE<'a>` | lexing/ast.rs:435 | → `<'p>` |
| `StringLE<'a>` | lexing/ast.rs:479 | → `<'p>` |
| `IDenizenL<'a>` | lexing/ast.rs:59 | → `<'p>` |
| `IAttributeL<'a>` | lexing/ast.rs:181 | → `<'p>` |
| `INodeLEEnum<'a>` | lexing/ast.rs:329 | → `<'p>` |
| `StringPart<'a>` | lexing/ast.rs:498 | → `<'p>` |
| `Lexer<'a, 'ctx>` | lexing/lexer.rs:18 | → `Lexer<'p, 'ctx>` |

**Implication**: The `Lexer` needs access to the `'p` arena's interning context to intern strings. Currently it receives `&Interner<'a>`. After: it receives the parser's interning context.

---

### PARSING (currently `<'a, 'p>`, become `<'p>`)

Parser types have two lifetimes: `'a` for interned strings/coords, `'p` for arena-allocated AST nodes. Since strings will now be interned into `'p`, everything collapses to one lifetime.

| Struct/Enum | File:Line | Change |
|---|---|---|
| `FileP<'a, 'p>` | parsing/ast/ast.rs:53 | → `<'p>` |
| `StructP<'a, 'p>` | parsing/ast/ast.rs:309 | → `<'p>` |
| `InterfaceP<'a, 'p>` | parsing/ast/ast.rs:383 | → `<'p>` |
| `FunctionP<'a, 'p>` | parsing/ast/ast.rs:409 | → `<'p>` |
| `FunctionHeaderP<'a, 'p>` | parsing/ast/ast.rs:423 | → `<'p>` |
| `FunctionReturnP<'a, 'p>` | parsing/ast/ast.rs:451 | → `<'p>` |
| `GenericParameterP<'a, 'p>` | parsing/ast/ast.rs:464 | → `<'p>` |
| `GenericParametersP<'a, 'p>` | parsing/ast/ast.rs:498 | → `<'p>` |
| `TemplateRulesP<'a, 'p>` | parsing/ast/ast.rs:508 | → `<'p>` |
| `ParamsP<'a, 'p>` | parsing/ast/ast.rs:518 | → `<'p>` |
| `ImplP<'a, 'p>` | parsing/ast/ast.rs:97 | → `<'p>` |
| `ExportAsP<'a, 'p>` | parsing/ast/ast.rs:120 | → `<'p>` |
| `ImportP<'a, 'p>` | parsing/ast/ast.rs:134 | → `<'p>` |
| `StructMembersP<'a, 'p>` | parsing/ast/ast.rs:335 | → `<'p>` |
| `NormalStructMemberP<'a, 'p>` | parsing/ast/ast.rs:353 | → `<'p>` |
| `VariadicStructMemberP<'a, 'p>` | parsing/ast/ast.rs:360 | → `<'p>` |
| `IDenizenP<'a, 'p>` | parsing/ast/ast.rs:77 | → `<'p>` |
| `IAttributeP<'a>` | parsing/ast/ast.rs:234 | → `<'p>` |
| `IStructContent<'a, 'p>` | parsing/ast/ast.rs:347 | → `<'p>` |
| `NameP<'a>` | parsing/ast/ast.rs:30 | → `<'p>` |
| `MacroCallP<'a>` | parsing/ast/ast.rs:166 | → `<'p>` |
| `BuiltinAttributeP<'a>` | parsing/ast/ast.rs:192 | → `<'p>` |
| `IExpressionPE<'a, 'p>` | parsing/ast/expressions.rs:16 | → `<'p>` |
| `PackPE<'a, 'p>` | parsing/ast/expressions.rs:215 | → `<'p>` |
| `SubExpressionPE<'a, 'p>` | parsing/ast/expressions.rs:233 | → `<'p>` |
| `AndPE<'a, 'p>` | parsing/ast/expressions.rs:248 | → `<'p>` |
| `OrPE<'a, 'p>` | parsing/ast/expressions.rs:263 | → `<'p>` |
| `IfPE<'a, 'p>` | parsing/ast/expressions.rs:278 | → `<'p>` |
| `WhilePE<'a, 'p>` | parsing/ast/expressions.rs:306 | → `<'p>` |
| `EachPE<'a, 'p>` | parsing/ast/expressions.rs:324 | → `<'p>` |
| `RangePE<'a, 'p>` | parsing/ast/expressions.rs:342 | → `<'p>` |
| `DestructPE<'a, 'p>` | parsing/ast/expressions.rs:357 | → `<'p>` |
| `UnletPE<'a>` | parsing/ast/expressions.rs:371 | → `<'p>` |
| `MutatePE<'a, 'p>` | parsing/ast/expressions.rs:385 | → `<'p>` |
| `ReturnPE<'a, 'p>` | parsing/ast/expressions.rs:404 | → `<'p>` |
| `LetPE<'a, 'p>` | parsing/ast/expressions.rs:431 | → `<'p>` |
| `TuplePE<'a, 'p>` | parsing/ast/expressions.rs:451 | → `<'p>` |
| `StaticSizedArraySizeP<'a, 'p>` | parsing/ast/expressions.rs:465 | → `<'p>` |
| `ConstructArrayPE<'a, 'p>` | parsing/ast/expressions.rs:482 | → `<'p>` |
| `ConstantStrPE<'a>` | parsing/ast/expressions.rs:541 | → `<'p>` |
| `StrInterpolatePE<'a, 'p>` | parsing/ast/expressions.rs:570 | → `<'p>` |
| `DotPE<'a, 'p>` | parsing/ast/expressions.rs:584 | → `<'p>` |
| `IndexPE<'a, 'p>` | parsing/ast/expressions.rs:604 | → `<'p>` |
| `FunctionCallPE<'a, 'p>` | parsing/ast/expressions.rs:619 | → `<'p>` |
| `BraceCallPE<'a, 'p>` | parsing/ast/expressions.rs:640 | → `<'p>` |
| `NotPE<'a, 'p>` | parsing/ast/expressions.rs:663 | → `<'p>` |
| `AugmentPE<'a, 'p>` | parsing/ast/expressions.rs:678 | → `<'p>` |
| `TransmigratePE<'a, 'p>` | parsing/ast/expressions.rs:699 | → `<'p>` |
| `BinaryCallPE<'a, 'p>` | parsing/ast/expressions.rs:720 | → `<'p>` |
| `MethodCallPE<'a, 'p>` | parsing/ast/expressions.rs:741 | → `<'p>` |
| `LookupPE<'a, 'p>` | parsing/ast/expressions.rs:793 | → `<'p>` |
| `TemplateArgsP<'a, 'p>` | parsing/ast/expressions.rs:812 | → `<'p>` |
| `LambdaPE<'a, 'p>` | parsing/ast/expressions.rs:837 | → `<'p>` |
| `BlockPE<'a, 'p>` | parsing/ast/expressions.rs:856 | → `<'p>` |
| `ConsecutorPE<'a, 'p>` | parsing/ast/expressions.rs:873 | → `<'p>` |
| `ShortcallPE<'a, 'p>` | parsing/ast/expressions.rs:894 | → `<'p>` |
| `IImpreciseNameP<'a>` | parsing/ast/expressions.rs:765 | → `<'p>` |
| `IArraySizeP<'a, 'p>` | parsing/ast/expressions.rs:470 | → `<'p>` |
| `ITemplexPT<'a, 'p>` | parsing/ast/templex.rs:14 | → `<'p>` |
| `PointPT<'a, 'p>` | parsing/ast/templex.rs:97 | → `<'p>` |
| `CallPT<'a, 'p>` | parsing/ast/templex.rs:109 | → `<'p>` |
| `FunctionPT<'a, 'p>` | parsing/ast/templex.rs:120 | → `<'p>` |
| `InlinePT<'a, 'p>` | parsing/ast/templex.rs:133 | → `<'p>` |
| `TuplePT<'a, 'p>` | parsing/ast/templex.rs:163 | → `<'p>` |
| `NameOrRunePT<'a>` | parsing/ast/templex.rs:180 | → `<'p>` |
| `InterpretedPT<'a, 'p>` | parsing/ast/templex.rs:191 | → `<'p>` |
| `FuncPT<'a, 'p>` | parsing/ast/templex.rs:209 | → `<'p>` |
| `StaticSizedArrayPT<'a, 'p>` | parsing/ast/templex.rs:222 | → `<'p>` |
| `RuntimeSizedArrayPT<'a, 'p>` | parsing/ast/templex.rs:241 | → `<'p>` |
| `SharePT<'a, 'p>` | parsing/ast/templex.rs:256 | → `<'p>` |
| `TypedRunePT<'a>` | parsing/ast/templex.rs:276 | → `<'p>` |
| `RegionRunePT<'a>` | parsing/ast/templex.rs:294 | → `<'p>` |
| `PackPT<'a, 'p>` | parsing/ast/templex.rs:311 | → `<'p>` |
| `IRulexPR<'a, 'p>` | parsing/ast/rules.rs:12 | → `<'p>` |
| `EqualsPR<'a, 'p>` | parsing/ast/rules.rs:24 | → `<'p>` |
| `OrPR<'a, 'p>` | parsing/ast/rules.rs:31 | → `<'p>` |
| `DotPR<'a, 'p>` | parsing/ast/rules.rs:37 | → `<'p>` |
| `ComponentsPR<'a, 'p>` | parsing/ast/rules.rs:44 | → `<'p>` |
| `TypedPR<'a>` | parsing/ast/rules.rs:51 | → `<'p>` |
| `BuiltinCallPR<'a, 'p>` | parsing/ast/rules.rs:58 | → `<'p>` |
| `PackPR<'a, 'p>` | parsing/ast/rules.rs:65 | → `<'p>` |
| `ParameterP<'a, 'p>` | parsing/ast/pattern.rs:23 | → `<'p>` |
| `DestinationLocalP<'a>` | parsing/ast/pattern.rs:44 | → `<'p>` |
| `PatternPP<'a, 'p>` | parsing/ast/pattern.rs:54 | → `<'p>` |
| `DestructureP<'a, 'p>` | parsing/ast/pattern.rs:80 | → `<'p>` |
| `INameDeclarationP<'a>` | parsing/ast/pattern.rs:95 | → `<'p>` |
| `NodeRefP<'a, 'p>` | parsing/tests/traverse.rs | → `<'p>` |
| `ExpressionElement<'a, 'p>` | parsing/expression_parser.rs:34 | → `<'p>` |

**Pass infrastructure:**

| Struct | File:Line | Before → After |
|---|---|---|
| `Parser<'a, 'ctx, 'p>` | parsing/parser.rs:41 | → `Parser<'p, 'ctx>` |
| `ExpressionParser<'a, 'ctx, 'p>` | parsing/expression_parser.rs:40 | → `ExpressionParser<'p, 'ctx>` |
| `TemplexParser<'a, 'ctx, 'p>` | parsing/templex_parser.rs:34 | → `TemplexParser<'p, 'ctx>` |
| `PatternParser<'a, 'ctx, 'p>` | parsing/pattern_parser.rs:25 | → `PatternParser<'p, 'ctx>` |
| `ParserCompilation<'a, 'ctx, 'p>` | parsing/parser.rs:1717 | → `ParserCompilation<'p, 'ctx>` |
| `ParserVonifier<'a>` | parsing/vonifier.rs:10 | → `ParserVonifier<'p>` |
| `ScrambleIterator<'a, 's>` | parsing/scramble_iterator.rs:7 | → `ScrambleIterator<'p, 's>` (NOTE: 's here is a local iteration lifetime, not scout) |

**Implication**: Parser infrastructure drops `'a`, uses `'p` for everything. The `interner: &'ctx Interner<'a>` field becomes something like `interning: &'ctx Interning<'p>`.

---

### POSTPARSING — Names/Runes (currently `<'a>`, become `<'s>`)

These are interned into the `'a` arena currently. Under the new model, they're interned into `'s`.

| Struct/Enum | File:Line | Change |
|---|---|---|
| `INameS<'a>` | postparsing/names.rs:13 | → `<'s>` |
| `INameValS<'a>` | postparsing/names.rs:80 | → `<'s>` |
| `IImpreciseNameS<'a>` | postparsing/names.rs:116 | → `<'s>` |
| `IImpreciseNameValS<'a>` | postparsing/names.rs:230 | → `<'s>` |
| `IVarNameS<'a>` | postparsing/names.rs:255 | → `<'s>` |
| `IVarNameValS<'a>` | postparsing/names.rs:283 | → `<'s>` |
| `IFunctionDeclarationNameS<'a>` | postparsing/names.rs:298 | → `<'s>` |
| `IFunctionDeclarationNameValS<'a>` | postparsing/names.rs:317 | → `<'s>` |
| `IImplDeclarationNameS<'a>` | postparsing/names.rs:384 | → `<'s>` |
| `TopLevelCitizenDeclarationNameS<'a>` | postparsing/names.rs:496 | → `<'s>` |
| `IStructDeclarationNameS<'a>` | postparsing/names.rs:521 | → `<'s>` |
| `IRuneS<'a>` | postparsing/names.rs:782 | → `<'s>` |
| `IRuneValS<'a>` | postparsing/names.rs:1008 | → `<'s>` |
| All ~30 concrete name/rune payload structs | postparsing/names.rs | → `<'s>` |

**Implication**: These are the most widely-referenced types. Every `IRuneS<'a>` becomes `IRuneS<'s>`, every `INameS<'a>` becomes `INameS<'s>`. This cascades through EVERY struct that contains a name or rune.

The interning maps for names/runes move from `Interner<'a>` to the scout-pass interning context.

---

### POSTPARSING — AST (currently `<'a, 's>`, become `<'s>`)

| Struct/Enum | File:Line | Change |
|---|---|---|
| `ProgramS<'a, 's>` | postparsing/ast.rs:42 | → `<'s>` |
| `StructS<'a, 's>` | postparsing/ast.rs:273 | → `<'s>` |
| `InterfaceS<'a, 's>` | postparsing/ast.rs:452 | → `<'s>` |
| `ImplS<'a, 's>` | postparsing/ast.rs:555 | → `<'s>` |
| `FunctionS<'a, 's>` | postparsing/ast.rs:934 | → `<'s>` |
| `ExportAsS<'a, 's>` | postparsing/ast.rs:585 | → `<'s>` |
| `ImportS<'a, 's>` | postparsing/ast.rs:604 | → `<'s>` |
| `GenericParameterS<'a, 's>` | postparsing/ast.rs:899 | → `<'s>` |
| `GenericParameterDefaultS<'a, 's>` | postparsing/ast.rs:921 | → `<'s>` |
| `SimpleParameterS<'a, 's>` | postparsing/ast.rs:699 | → `<'s>` |
| `CodeBodyS<'a, 's>` | postparsing/ast.rs:755 | → `<'s>` |
| `TopLevelFunctionS<'a, 's>` | postparsing/ast.rs:1225 | → `<'s>` |
| `TopLevelImplS<'a, 's>` | postparsing/ast.rs:1234 | → `<'s>` |
| `TopLevelExportAsS<'a, 's>` | postparsing/ast.rs:1243 | → `<'s>` |
| `TopLevelImportS<'a, 's>` | postparsing/ast.rs:1251 | → `<'s>` |
| `TopLevelStructS<'a, 's>` | postparsing/ast.rs:1300 | → `<'s>` |
| `TopLevelInterfaceS<'a, 's>` | postparsing/ast.rs:1311 | → `<'s>` |
| `FileS<'a, 's>` | postparsing/ast.rs:1322 | → `<'s>` |
| `IDenizenS<'a, 's>` | postparsing/ast.rs:1212 | → `<'s>` |
| `ICitizenDenizenS<'a, 's>` | postparsing/ast.rs:1271 | → `<'s>` |
| `ICitizenS<'a, 's>` | postparsing/ast.rs:233 | → `<'s>` |
| `IBodyS<'a, 's>` | postparsing/ast.rs:717 | → `<'s>` |
| `ParameterS<'a>` | postparsing/ast.rs:659 | → `<'s>` |
| `AbstractSP<'a>` | postparsing/ast.rs:684 | → `<'s>` |
| `ExternS<'a>` | postparsing/ast.rs:157 | → `<'s>` |
| `BuiltinS<'a>` | postparsing/ast.rs:189 | → `<'s>` |
| `MacroCallS<'a>` | postparsing/ast.rs:202 | → `<'s>` |
| `ExportS<'a>` | postparsing/ast.rs:215 | → `<'s>` |
| `ICitizenAttributeS<'a>` | postparsing/ast.rs:129 | → `<'s>` |
| `IFunctionAttributeS<'a>` | postparsing/ast.rs:143 | → `<'s>` |
| `IStructMemberS<'a>` | postparsing/ast.rs:377 | → `<'s>` |
| `NormalStructMemberS<'a>` | postparsing/ast.rs:418 | → `<'s>` |
| `VariadicStructMemberS<'a>` | postparsing/ast.rs:436 | → `<'s>` |
| `IGenericParameterTypeS<'a>` | postparsing/ast.rs:781 | → `<'s>` |
| `CoordGenericParameterTypeS<'a>` | postparsing/ast.rs:848 | → `<'s>` |
| `GeneratedBodyS<'a>` | postparsing/ast.rs:744 | → `<'s>` |

**Implication**: All `'a` references in these structs (to `RangeS`, `StrI`, `IRuneS`, `INameS`, etc.) become `'s` since those types are now in the `'s` arena. The second lifetime parameter `'s` was already for arena slices — now it's the only lifetime.

---

### POSTPARSING — Expressions (currently `<'a, 's>`, become `<'s>`)

| Struct/Enum | File:Line | Change |
|---|---|---|
| `IExpressionSE<'a, 's>` | postparsing/expressions.rs:257 | → `<'s>` |
| `LetSE<'a, 's>` | postparsing/expressions.rs:32 | → `<'s>` |
| `IfSE<'a, 's>` | postparsing/expressions.rs:39 | → `<'s>` |
| `LoopSE<'a, 's>` | postparsing/expressions.rs:58 | → `<'s>` |
| `WhileSE<'a, 's>` | postparsing/expressions.rs:78 | → `<'s>` |
| `MapSE<'a, 's>` | postparsing/expressions.rs:89 | → `<'s>` |
| `ExprMutateSE<'a, 's>` | postparsing/expressions.rs:100 | → `<'s>` |
| `GlobalMutateSE<'a, 's>` | postparsing/expressions.rs:111 | → `<'s>` |
| `LocalMutateSE<'a, 's>` | postparsing/expressions.rs:122 | → `<'s>` |
| `OwnershippedSE<'a, 's>` | postparsing/expressions.rs:133 | → `<'s>` |
| `BodySE<'a, 's>` | postparsing/expressions.rs:194 | → `<'s>` |
| `PureSE<'a, 's>` | postparsing/expressions.rs:215 | → `<'s>` |
| `BlockSE<'a, 's>` | postparsing/expressions.rs:234 | → `<'s>` |
| `ConsecutorSE<'a, 's>` | postparsing/expressions.rs:343 | → `<'s>` |
| `RepeaterBlockSE<'a, 's>` | postparsing/expressions.rs:400 | → `<'s>` |
| `RepeaterBlockIteratorSE<'a, 's>` | postparsing/expressions.rs:411 | → `<'s>` |
| `ReturnSE<'a, 's>` | postparsing/expressions.rs:422 | → `<'s>` |
| `TupleSE<'a, 's>` | postparsing/expressions.rs:445 | → `<'s>` |
| `StaticArrayFromValuesSE<'a, 's>` | postparsing/expressions.rs:455 | → `<'s>` |
| `StaticArrayFromCallableSE<'a, 's>` | postparsing/expressions.rs:478 | → `<'s>` |
| `NewRuntimeSizedArraySE<'a, 's>` | postparsing/expressions.rs:501 | → `<'s>` |
| `RepeaterPackSE<'a, 's>` | postparsing/expressions.rs:522 | → `<'s>` |
| `RepeaterPackIteratorSE<'a, 's>` | postparsing/expressions.rs:533 | → `<'s>` |
| `FunctionSE<'a, 's>` | postparsing/expressions.rs:606 | → `<'s>` |
| `DotSE<'a, 's>` | postparsing/expressions.rs:615 | → `<'s>` |
| `IndexSE<'a, 's>` | postparsing/expressions.rs:627 | → `<'s>` |
| `FunctionCallSE<'a, 's>` | postparsing/expressions.rs:643 | → `<'s>` |
| `OutsideLoadSE<'a, 's>` | postparsing/expressions.rs:662 | → `<'s>` |
| `DestructSE<'a, 's>` | postparsing/expressions.rs:586 | → `<'s>` |
| `BreakSE<'a>` | postparsing/expressions.rs:69 | → `<'s>` |
| `VoidSE<'a>` | postparsing/expressions.rs:436 | → `<'s>` |
| `ArgLookupSE<'a>` | postparsing/expressions.rs:389 | → `<'s>` |
| `ConstantIntSE<'a>` | postparsing/expressions.rs:549 | → `<'s>` |
| `ConstantBoolSE<'a>` | postparsing/expressions.rs:555 | → `<'s>` |
| `ConstantStrSE<'a>` | postparsing/expressions.rs:565 | → `<'s>` |
| `ConstantFloatSE<'a>` | postparsing/expressions.rs:576 | → `<'s>` |
| `UnletSE<'a>` | postparsing/expressions.rs:596 | → `<'s>` |
| `LocalLoadSE<'a>` | postparsing/expressions.rs:656 | → `<'s>` |
| `RuneLookupSE<'a>` | postparsing/expressions.rs:684 | → `<'s>` |
| `LocalS<'a>` | postparsing/expressions.rs:170 | → `<'s>` |

---

### POSTPARSING — Rules (currently `<'a>` or `<'a, 's>`, become `<'s>`)

| Struct/Enum | File:Line | Change |
|---|---|---|
| `RuneUsage<'a>` | postparsing/rules/rules.rs:23 | → `<'s>` |
| `IRulexSR<'a, 's>` | postparsing/rules/rules.rs:36 | → `<'s>` |
| `EqualsSR<'a, 's>` + all other rule structs | postparsing/rules/rules.rs:165-712 | → `<'s>` |
| `ILiteralSL<'a>` | postparsing/rules/rules.rs:660 | → `<'s>` |

---

### POSTPARSING — Errors (currently `<'a>` or `<'a, 's>`, become `<'s>`)

| Struct/Enum | File:Line | Change |
|---|---|---|
| `ICompileErrorS<'a, 's>` | postparsing/post_parser.rs:87 | → `<'s>` |
| `CompileErrorExceptionS<'a, 's>` | postparsing/post_parser.rs:81 | → `<'s>` |
| All error structs (`CouldntFindVarToMutateS<'a>`, etc.) | postparsing/post_parser.rs | → `<'s>` |
| `RuneTypeSolveError<'a, 's>` | postparsing/rune_type_solver.rs:16 | → `<'s>` |
| `IdentifiabilitySolveError<'a, 's>` | postparsing/identifiability_solver.rs:22 | → `<'s>` |
| All rune type solver error structs | postparsing/rune_type_solver.rs | → `<'s>` |

---

### POSTPARSING — Environments & Infrastructure

| Struct/Enum | File:Line | Before → After |
|---|---|---|
| `EnvironmentS<'a>` | postparsing/post_parser.rs:337 | → `<'s>` |
| `FunctionEnvironmentS<'a>` | postparsing/post_parser.rs:388 | → `<'s>` |
| `IEnvironmentS<'a>` | postparsing/post_parser.rs:287 | → `<'s>` |
| `StackFrame<'a>` | postparsing/post_parser.rs:460 | → `<'s>` |
| `VariableUseS<'a>` | postparsing/variable_uses.rs:13 | → `<'s>` |
| `VariableDeclarationS<'a>` | postparsing/variable_uses.rs:31 | → `<'s>` |
| `VariableDeclarations<'a>` | postparsing/variable_uses.rs:42 | → `<'s>` |
| `VariableUses<'a>` | postparsing/variable_uses.rs:129 | → `<'s>` |
| `CaptureS<'a>` | postparsing/patterns/patterns.rs:16 | → `<'s>` |
| `AtomSP<'a>` | postparsing/patterns/patterns.rs:30 | → `<'s>` |
| `PostParser<'a, 'p, 'ctx, 's>` | postparsing/post_parser.rs:987 | → `PostParser<'p, 'ctx, 's>` |
| `ScoutCompilation<'a, 'ctx, 'p, 's>` | postparsing/post_parser.rs:2620 | → `ScoutCompilation<'p, 'ctx, 's>` |
| `RuneTypeSolver<'a, 'ctx>` | postparsing/rune_type_solver.rs:366 | → `RuneTypeSolver<'s, 'ctx>` |

**Implication for PostParser**: Currently holds `interner: &'ctx Interner<'a>`. After: holds `scout_arena: &'ctx ScoutArena<'s>`. The `'p` lifetime stays as read-only input.

---

### POSTPARSING — Other

| Struct/Enum | File:Line | Change |
|---|---|---|
| `LocalLookupResultS<'a>` | postparsing/expression_scout.rs:84 | → `<'s>` |
| `OutsideLookupResultS<'a, 'p>` | postparsing/expression_scout.rs:96 | → `<'p, 's>` (still needs `'p` for parser template args) |
| `NormalResultS<'a, 's>` | postparsing/expression_scout.rs:115 | → `<'s>` |
| `IScoutResult<'a, 'p, 's>` | postparsing/expression_scout.rs:72 | → `<'p, 's>` |
| `IFunctionParent<'a, 's>` | postparsing/function_scout.rs:69 | → `<'s>` |
| `CitizenRuneTypeSolverLookupResult<'a, 's>` | postparsing/rune_type_solver.rs:255 | → `<'s>` |
| `NodeRefS<'a, 's>` | postparsing/test/traverse.rs:27 | → `<'s>` |

**Note on `OutsideLookupResultS` and `IScoutResult`**: These hold `'p` references to parser template args (`&'p [ITemplexPT]`). They still need `'p` as an input lifetime during postparsing. But `'a` is gone.

---

### HIGHER TYPING (currently `<'a, 's>`, become `<'s>`)

| Struct/Enum | File:Line | Change |
|---|---|---|
| `ProgramA<'a, 's>` | higher_typing/ast.rs:31 | → `<'s>` |
| `StructA<'a, 's>` | higher_typing/ast.rs:153 | → `<'s>` |
| `InterfaceA<'a, 's>` | higher_typing/ast.rs:445 | → `<'s>` |
| `FunctionA<'a, 's>` | higher_typing/ast.rs:634 | → `<'s>` |
| `ImplA<'a, 's>` | higher_typing/ast.rs:303 | → `<'s>` |
| `ExportAsA<'a, 's>` | higher_typing/ast.rs:382 | → `<'s>` |
| `Astrouts<'a, 's>` | higher_typing/higher_typing_pass.rs:62 | → `<'s>` |
| `EnvironmentA<'a, 's>` | higher_typing/higher_typing_pass.rs:79 | → `<'s>` |
| `HigherTypingPass<'a, 'ctx, 's>` | higher_typing/higher_typing_pass.rs:443 | → `HigherTypingPass<'ctx, 's>` |
| `HigherTypingCompilation<'a, 'ctx, 'p, 's>` | higher_typing/higher_typing_pass.rs:1725 | → `HigherTypingCompilation<'ctx, 'p, 's>` |
| `HigherTypingRuneTypeSolverEnv<'a, 'ctx, 's, 'env>` | higher_typing/higher_typing_pass.rs:1877 | → `HigherTypingRuneTypeSolverEnv<'ctx, 's, 'env>` |
| `ICompileErrorA<'a, 's>` | higher_typing/astronomer_error_reporter.rs:39 | → `<'s>` |
| `CompileErrorExceptionA<'a, 's>` | higher_typing/astronomer_error_reporter.rs:16 | → `<'s>` |
| All error structs (CouldntFindTypeA, etc.) | higher_typing/astronomer_error_reporter.rs | → `<'s>` |

---

### SOLVER

| Struct | File:Line | Change |
|---|---|---|
| `Solver<'a, Rule, Rune, ...>` | solver/solver.rs:348 | `'a` is a local callback lifetime, NOT the interner — no change needed |
| `TestRuleSolver<'a>` | solver/test/test_rule_solver.rs:15 | Same — local test lifetime |

---

### COMPILATION PIPELINE

| Struct | File:Line | Before → After |
|---|---|---|
| `FullCompilation<'a, 'ctx, 'p, 's>` | pass_manager/full_compilation.rs:47 | → `FullCompilation<'ctx, 'p, 's>` |
| `HammerCompilation<'a, 'ctx, 'p, 's>` | simplifying/hammer_compilation.rs:24 | → `HammerCompilation<'ctx, 'p, 's>` |
| `TypingPassCompilation<'a, 'ctx, 'p, 's>` | typing/compilation.rs:25 | → `TypingPassCompilation<'ctx, 'p, 's>` |
| `InstantiatedCompilation<'a, 'ctx, 'p, 's>` | instantiating/instantiated_compilation.rs:22 | → `InstantiatedCompilation<'ctx, 'p, 's>` |
| `Options<'a>` | pass_manager/pass_manager.rs:613 | → `Options<'p>` (PackageCoordinate now in `'p`) |
| `FileSystemResolver<'a>` | pass_manager/pass_manager.rs:51 | → `FileSystemResolver<'p>` |
| `IFrontendInput<'a>` | pass_manager/pass_manager.rs:255 | → `IFrontendInput<'p>` |

**Implication**: All `where 'a: 'ctx, 'a: 'p, 'a: 's` constraints are eliminated.

---

## All Complications — Resolved

### 1. Where does string interning happen first?
The `'p` Bump arena and `ParseArena<'p>` are created by the caller (or `ParserCompilation`) before the lexer runs. Both the lexer and parser receive `&ParseArena<'p>` and intern into it. ✅

### 2. `Arc<FileCoordinate>` in CodeLocationS
Kill the `Arc`. Replace with `&'x FileCoordinate<'x>` (arena reference). `CodeLocationS` and `RangeS` become fully `Copy`. `eval_pos` becomes a pointer copy instead of `Arc::new`. ✅

### 3. `FileCoordinateMap`/`PackageCoordinateMap` cross pass boundaries
They don't need to cross. `parseds_cache` stays in `'p` inside `ParserCompilation`. The postparser borrows it with `'p` keys. Source code map (`code_map_cache`) becomes a lifetime-free `HashMap<String, String>` owned by the compilation driver (needed by error humanizers that outlive all passes). `vpst_map_cache` (serialized JSON AST) same treatment. ✅

### 4. `IPackageResolver` trait uses `'a`
Becomes `IPackageResolver<'p, T>`. The caller creates the `'p` arena, interns `PackageCoordinate`s into it, then passes them to the compilation pipeline. ✅

### 5. Parser still needs `'p` as input to postparser
Expected and fine. `PostParser<'p, 'ctx, 's>` — two arena lifetimes (input + output), no `'a`. `HigherTypingCompilation<'ctx, 'p, 's>` keeps `'p` transitively. Output types are `<'s>` only. ✅

### 6. `Keywords` duplication across passes
Each pass creates its own `Keywords<'x>` fresh by interning ~40 string constants into its arena. Pre-size the string interning HashMap to avoid rehashing. ✅

### 7. Test files
Mechanical updates. Tests become simpler — one arena struct per test instead of separate `Interner` + arena. ✅

### 8. Cross-pass translation
Translation logic lives in normal pass compilation code, NOT on the arena structs. The postparser calls `scout_arena.intern_str(...)`, `scout_arena.intern_file_coord(...)` etc. when it needs to copy data from `'p` into `'s`. `FileCoordinate` translation happens once at postparse start per file. ✅

### 9. `StrI<'a>` flows from parser types into postparser names — can't split phases
Changing parser types to `<'p>` without also changing postparser names to `<'s>` creates a lifetime mismatch: postparser gets `StrI<'p>` from parser nodes but needs `StrI<'a>` for name building. Since `'a: 'p` (not reverse), this doesn't coerce. **Resolution**: Phases 1+2 are combined into a single change. ✅

### 10. PostParser needs `&ParseArena<'p>`, not just `&'p Bump` (see @PPSPASTNZ)
The postparser synthesizes parser AST nodes (`IExpressionPE<'p>`, `LookupPE<'p>`, etc.) during expression scouting and loop desugaring. These nodes need proper string interning into `'p`, so `PostParser` holds `parse_arena: &'ctx ParseArena<'p>` (not a raw Bump). ✅

### 11. PostParser needs `Keywords<'p>` for synthetic parser nodes
Synthetic parser nodes use keyword strings (`self_`, `begin`, `next`, `isEmpty`, `get`). Those are `StrI<'s>` in the scout `Keywords`, but need to be `StrI<'p>` inside parser-typed nodes. **Resolution**: PostParser holds a second `Keywords<'p>` constructed from `ParseArena<'p>`. ✅

### 12. loop_post_parser.rs has ~20+ synthetic parser AST constructions
The plan originally only noted expression_scout.rs for @PPSPASTNZ. The loop desugaring (`scout_each`, `scout_each_body`, `scout_while`, `scout_while_body`) builds extensive chains of `LetPE`, `LookupPE`, `AugmentPE`, `FunctionCallPE`. All need the same `parse_arena` + `keywords_p` treatment. ✅

### 13. Slice lifetime relaxation pattern — **FLAG IF THIS GOES AWRY**
Functions like `get_ordered_rune_declarations_from_templexes_with_duplicates(&'p [ITemplexPT<'p>])` need the slice reference lifetime relaxed to `&[ITemplexPT<'p>]` because callers construct local `Vec`s and pass slices. Previously `'a` outlived everything so `&'a [T]` worked even for locals. Now callers can't produce `&'p [T]` from a local Vec. The fix (remove `'p` from the slice ref) is usually correct — the function borrows the data temporarily and extracts `'p`-lived content from inside. **But**: if any function actually needs the slice to persist for `'p` (e.g., stores it in a struct), this fix would be wrong. Alert the user if relaxing a slice lifetime causes downstream issues.

### 14. Solver parameter lifetimes may need tightening
`identifiability_solver::solve_identifiability` needed `rules: &'s [IRulexSR<'s>]` (was `&[IRulexSR<'s>]`) because it extracts `IRuneS<'s>` values from the rules and returns them. The rules slice must live at least as long as `'s` for this to work. `check_identifiability` callers need to arena-allocate rules before passing them. ✅

### 15. `VariableUses` methods had `'b` generics that broke without `'a: 'b`
`then_merge<'b>`, `mark_borrowed<'b>`, etc. used a generic `'b` for mixing `VariableUses<'a>` with new data. Without the universal `'a`, the `'b` became ambiguous. **Resolution**: removed `'b`, all methods use `'s` directly. All call sites operate on `VariableUses<'s>` anyway. ✅

### 16. `'static` returns caused cascading E0282 inference failures
`VariableUses::empty()` returned `VariableUses<'static>` and `PostParser::no_declarations()` returned `VariableDeclarations<'static>`. Without `'a` to unify with, the compiler couldn't infer lifetimes at call sites. **Resolution**: changed return types to `'s`, qualified call sites with turbofish: `VariableUses::<'s>::empty()`, `PostParser::<'s, 'p, '_>::no_declarations()`. ✅

### 17. ~15 tuple destructurings needed explicit type annotations
Every `let (stack_frame, expr, self_uses, child_uses) = self.scout_expression(...)` needed a full type annotation. Previously `'a` provided enough constraint for inference. **Resolution**: added explicit `: (StackFrame<'s>, &'s IExpressionSE<'s>, VariableUses<'s>, VariableUses<'s>)` at each site. ✅

### 18. Parser-typed function parameters needed `&'p` not `&`
Functions like `scout_impure_block(block_pe: &BlockPE<'p>)` needed `&'p BlockPE<'p>` because `scout_block` expects `&'p`. The reference lifetime itself matters, not just the data inside. ✅

### 19. Cross-pass `StrI` re-interning is pervasive (~30 sites)
The plan said translation happens at pass boundaries. In reality, re-interning `StrI<'p>` → `StrI<'s>` via `interner.intern(name_p.str().as_str())` happens at ~30 individual sites scattered throughout postparser code — anywhere a parser node's `.str()` is used to build a scout name/rune. Not a single boundary step. ✅

### 20. Pipeline chain needed `'p` infrastructure threaded 6 layers deep
`FullCompilation` → `HammerCompilation` → `InstantiatedCompilation` → `TypingPassCompilation` → `HigherTypingCompilation` → `ScoutCompilation` — all needed `parser_interner`, `parser_keywords`, `parse_arena` added to their `new()` signatures. Not separable from core work. ✅

---

## Standing instruction

**Notify the user** when encountering lifetime conundrums during implementation, especially:
- Slice lifetime relaxation (#13) causing downstream issues
- Solver parameter lifetime tightening (#14) propagating unexpectedly
- Any case where two independent arena lifetimes need to interact in a way that wasn't anticipated
- Any new cross-arena data flow (like @PPSPASTNZ) not already documented

---

## Why Phases 1+2 must be combined

`StrI<'a>` flows from parser types into postparser names: the postparser extracts `StrI<'a>` from `FileP<'a,'p>` and uses it to build `CodeNameS { name: StrI<'a> }`. If we change parser types to `<'p>` (Phase 1) without also changing postparser names to `<'s>` (Phase 2), the postparser would get `StrI<'p>` from parser nodes but need `StrI<'a>` for names. Since `'a: 'p` (not the reverse), `StrI<'p>` can't be used as `StrI<'a>`.

Doing them separately would require temporary re-interning code that gets thrown away. Combining them is cleaner.

---

## Migration Strategy

### Phase 1+2 (combined): Rename all lifetimes, create both arena structs

**Phase 1+2 is COMPLETE.** All steps below are done:
- `ParseArena<'p>` created at `src/parse_arena.rs` ✅
- Mechanical lifetime renames for lexing/, parsing/, postparsing/, higher_typing/, pipeline ✅
- `Arc` killed in `CodeLocationS` — now `&'x FileCoordinate<'x>`, `Copy` derived ✅
- PostParser holds `&ParseArena<'p>` and `Keywords<'p>` for @PPSPASTNZ ✅
- Synthetic parser AST nodes in expression_scout.rs and loop_post_parser.rs use `parse_arena` ✅
- Slice lifetimes relaxed where needed ✅
- ~30 cross-pass `StrI` re-interning sites (`interner.intern(name_p.str().as_str())`) ✅
- `VariableUses` methods: `'b` generic removed, using `'s` directly ✅
- `'static` returns replaced with `'s`, turbofish at call sites ✅
- ~15 tuple destructurings annotated with explicit types ✅
- Parser-typed parameters: `&BlockPE<'p>` → `&'p BlockPE<'p>` etc. ✅
- Pipeline chain: `parser_interner`, `parser_keywords`, `parse_arena` threaded through 6 layers ✅
- `get_code_map`/`get_parseds`/`get_vpst_map` return `'p` types throughout chain ✅
- Core postparser: **0 errors** ✅
- Pipeline wiring: **6 errors** (Phase 3+4 work)

### Phase 3+4 (consolidated): Eliminate `Interner<'a>` and wire pipeline

Phase 3 (higher_typing renames) is already done. The remaining work is creating per-pass interners and restructuring the pipeline.

**Step 1 — Create `ScoutArena<'s>` ✅**
- Created at `src/scout_arena.rs` with full name/rune/imprecise-name interning (no delegation to Interner)

**Step 2 — Restructure `pass_manager::build()` ✅**
- `build()` now creates local `scout_bump`, `ParseArena`, `scout_interner`, `scout_keywords`
- Passes both `'p` and `'s` infrastructure into `FullCompilation::new`
- `build()` signature changed from `<'a, 'ctx>` to `<'p, 'ctx>`

**Step 3 — Switch parser to `ParseArena<'p>` ✅ COMPLETE**
- `Lexer`: `parse_arena: &ParseArena<'p>` instead of `interner: &Interner<'p>` ✅
- `Parser`: `parse_arena: &ParseArena<'p>` ✅
- `ExpressionParser`: `parse_arena: &ParseArena<'p>` ✅
- `TemplexParser`: `parse_arena: &ParseArena<'p>` ✅
- `PatternParser`: `parse_arena: &ParseArena<'p>` ✅
- `ParserCompilation`: `parse_arena: &ParseArena<'p>` ✅
- `parse_and_explore`: `parse_arena: &ParseArena<'p>` ✅
- `lex_and_explore`: `parse_arena: &ParseArena<'p>` ✅
- `parsed_loader.rs`: `&ParseArena<'p>` ✅
- `Keywords::new`: takes `&ParseArena<'p>` or `&ScoutArena<'s>` ✅
- Test file `parsing/tests/utils.rs`: updated ✅

**Step 4 — Switch postparser to `ScoutArena<'s>` (IN PROGRESS)**

Done:
- `postparsing/names.rs`: `ScoutArena` instead of `Interner` ✅
- `postparsing/identifiability_solver.rs`: `_scout_arena` parameter ✅
- `postparsing/rune_type_solver.rs`: `RuneTypeSolver.scout_arena` field ✅
- `postparsing/rules/rule_scout.rs`: all `interner` refs → `scout_arena` ✅
- `postparsing/rules/templex_scout.rs`: all 8 functions updated ✅
- `postparsing/patterns/pattern_scout.rs`: `translate_pattern` updated ✅
- `postparsing/post_parser.rs`: `PostParser` + `ScoutCompilation` fields updated, all call sites ✅
- `postparsing/function_scout.rs`: all call sites updated ✅
- `postparsing/expression_scout.rs`: all call sites updated ✅
- `higher_typing/higher_typing_pass.rs`: `HigherTypingPass` + `HigherTypingCompilation` fields, `RuneTypeSolver` init, `explicify_lookups`/`coerce_*` functions ✅

Remaining: None — all done ✅

**Step 5 — Delete `Interner<'a>` ✅**
- Removed `Interner` struct, `InternerInner`, `FileCoordLookupKey`, all `impl Interner` blocks, and interner tests from `src/interner.rs` (kept `StrI`, `InternedSlice`)
- Removed `pub use interner::Interner` from `lib.rs`
- Removed `parser_interner` from all 6 pipeline compilation struct constructors
- Removed all `use crate::Interner` / `use crate::interner::Interner` from all files
- Removed stale `Interner` imports from `lexer.rs`, `expression_parser.rs`, `pattern_parser.rs`, `templex_parser.rs`
- Deleted `Keywords::new(interner)` (kept `new_for_parse` and `new_for_scout`)
- Switched `builtins.rs` from `&Interner` to `&ParseArena`
- Switched `pass_manager.rs` from `Interner::with_arena` to `ParseArena::new`
- Switched test utilities (`range.rs`, `code_hierarchy.rs`) from `&Interner` to `&ScoutArena`/`&ParseArena`
- `PackageCoordinate::parent()` takes `&'a Bump` directly (no interning needed)
- 578 tests pass, zero errors

**Step 6 — Fix tests** ✅
- Parser tests: all fixed — `compile_file`/`compile` take `&ParseArena<'p>` instead of `&Interner<'p>`
- Postparser tests: all fixed — create `ParseArena` + `ScoutArena`, use proper cross-pass re-interning for `FileCoordinate`
- Higher typing tests: already updated with `parser_interner`/`parser_keywords`/`parse_arena` — will need updating again when Interner is removed

**Step 7 — Update docs ✅**
- `docs/arena-lifetimes.md` — rewritten: "Three Arenas" → "Two Arenas", removed all `'a`/`Interner` references ✅
- `docs/arena-two-phase-lifecycle.md` — updated `StrI` interning row and dangling pointer note ✅
- `docs/arena-allocated-structs.md` — split "interner arena" section into scout/parse arena sections ✅
- `zen/typing-pass-design.md` — no changes needed (already describes future `'t` arena) ✅
- `docs/per-pass-arenas-migration.md` — synced status sections ✅

---

## Verification

After Phase 1+2 (done):
- Core lexing/parsing/postparsing/higher_typing: 0 errors ✅
- Pipeline wiring: 0 errors ✅
- Lib builds: `cargo build --lib` succeeds ✅
- All 589 tests pass ✅

Current state (ALL STEPS COMPLETE):
- Steps 1–6 complete; `Interner<'a>` fully deleted
- `cargo build --lib` succeeds with zero errors
- `cargo test` passes: 578 tests, 0 failures
- All live Rust code uses `ParseArena<'p>` or `ScoutArena<'s>` — zero remaining `Interner<` references
- `src/interner.rs` contains only `StrI<'a>` and `InternedSlice<'a, T>`

After Phase 3+4:
- `cargo check` compiles with zero errors
- `cargo build --lib` succeeds
- `cargo test` passes
- Grep for `Interner<` in src/ — should be zero (except in commented Scala code)
- Grep for `'a` in non-test, non-comment Rust code — should be zero in lexing/parsing/postparsing/higher_typing

---

## Files to modify

**Phase 1+2 (done):**
- `src/parse_arena.rs` ✅
- `src/lib.rs` ✅
- `src/utils/range.rs` ✅
- `src/lexing/*.rs` ✅
- `src/parsing/**/*.rs` ✅
- `src/postparsing/**/*.rs` ✅
- `src/higher_typing/**/*.rs` ✅
- `src/pass_manager/full_compilation.rs` ✅
- `src/typing/compilation.rs` ✅
- `src/simplifying/hammer_compilation.rs` ✅
- `src/instantiating/instantiated_compilation.rs` ✅

**Phase 3+4 (remaining):**
- NEW: `src/scout_arena.rs`
- `src/interner.rs` (strip name/rune interning → ScoutArena, then delete Interner)
- `src/keywords.rs` (take ParseArena or ScoutArena instead of Interner)
- `src/pass_manager/pass_manager.rs` (restructure `build()`)
- `src/lexing/lexer.rs` (take ParseArena)
- `src/parsing/parser.rs` (take ParseArena)
- `src/postparsing/post_parser.rs` (take ScoutArena)
- `src/utils/range.rs` (CodeLocationS helpers take arena instead of Interner)
- All test files
- `docs/*.md`, `zen/*.md`
