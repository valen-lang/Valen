# Accuracy report: Defer Slice Allocation Until Intern Miss (DSAUIMZ)

Audited against the working tree on 2026-09-06. Arcana doc: docs/arcana/DeferSliceAllocationUntilInternMiss-DSAUIMZ.md

## Verdict
The core mechanism the arcana describes — transient `*ValS<'tmp>` structs borrowing slices from a stack builder, promoted to arena storage only on an intern miss via `promote_in()`/`intern_rune()` — is accurate and every cited code site still does what the doc says. The one inaccuracy is a stale count: the doc says "`ImplicitRuneValS<'tmp>` and 6 other `*ValS<'tmp>` structs" but `src/postparsing/names.rs` currently defines only 5 other such structs (6 total, not 7). This is a minor, non-load-bearing drift.

## Claims
| # | Claim (quoted or closely paraphrased from the doc) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Val types are transient, checked against the arena, then discarded (hit) or promoted (miss) | TRUE | src/scout_arena.rs:568-580 | |
| 2 | Transient Val slices must be transient (borrowed, not pre-allocated) to avoid wasting arena space on a hit | TRUE | src/scout_arena.rs:566-567, 585 | |
| 3 | `'tmp` lifetime ties to a local builder's `Vec` on the stack | TRUE | src/postparsing/ast.rs:649-651, 693-701 (`LocationInDenizenBuilder.path: Vec<i32>`, `borrow_val` returns `&self.path`) | |
| 4 | Val struct = internable unit; naming convention `Val` marks the unit boundary | TRUE | src/postparsing/names.rs:1067, 1148 (`ImplicitRuneValS`, `IRuneValS`); src/postparsing/ast.rs:726 (`LocationInDenizenVal`) | |
| 5 | Types without `Val` (e.g. `LocationInDenizen`, bare slices) are parts, arena-allocated only when the unit is promoted | TRUE | src/postparsing/ast.rs:710-716 (`LocationInDenizen<'x>` plain struct), promoted via `promote_in` at ast.rs:737 | |
| 6 | Already-permanent refs (`IRuneS<'s>`, `StrI<'s>`) are free to copy, part of a different, already-promoted unit | TRUE | consistent with `IRuneValS` variants like `DenizenDefaultRegionRuneS<'s>` at names.rs:1163 | |
| 7 | Privacy enforces the boundary: transient Val slice fields are private, constructible only via builder methods | TRUE | src/postparsing/ast.rs:726-729 (`path: &'tmp [i32]` private field) | |
| 8 | `promote_in()` is `pub(crate)`, called only inside `intern_*` methods on a miss | TRUE | src/postparsing/ast.rs:737 (`pub(crate) fn promote_in`); called at src/scout_arena.rs:588-589 inside `alloc_rune_canonical`, itself called only from `intern_rune` (scout_arena.rs:575) | |
| 9 | `LocationInDenizenVal<'tmp>` — transient form of `LocationInDenizen<'s>`, private `path` field, constructed only via `LocationInDenizenBuilder::borrow_val()` | TRUE | src/postparsing/ast.rs:693-701, 726-729 | |
| 10 | "`ImplicitRuneValS<'tmp>` and 6 other `*ValS<'tmp>` structs — transient forms with private `lid` field" | DRIFTED | src/postparsing/names.rs:1067,1080,1093,1106,1119,1132 — only 5 *other* `*ValS<'tmp>` structs exist besides `ImplicitRuneValS` (6 total: `ImplicitRuneValS`, `CallRegionRuneValS`, `CallPureMergeRegionRuneValS`, `LetImplicitRuneValS`, `MagicParamRuneValS`, `LocalDefaultRegionRuneValS`) | Change "and 6 other" to "and 5 other" (6 `*ValS<'tmp>` structs total). |
| 11 | `IRuneValS<'s, 'tmp>` — the `'tmp` lifetime carries the transient borrow | TRUE | src/postparsing/names.rs:1148 | |
| 12 | `ScoutArena::intern_rune()` — the gateway: accepts transient `IRuneValS<'s, 'tmp>`, promotes to permanent only on miss | TRUE | src/scout_arena.rs:568-579 | |
| 13 | `RuneValQuery` — wrapper enabling heterogeneous lookup via `hashbrown::Equivalent` | TRUE | src/postparsing/names.rs:1207-1231 | |
| 14 | All `borrow_val()` call sites in templex_scout, rule_scout, function_scout | TRUE | src/postparsing/rules/templex_scout.rs (24 calls), rule_scout.rs (4 calls), function_scout.rs (7 calls), each headed by a `@DSAUIMZ` comment at line 1 | |
| 15 | Future: typing pass internable types with `Vector[ITemplataT]` fields will follow the same pattern | UNVERIFIABLE | No such typing-pass internable type exists yet (forward-looking statement, not checkable against current code) | |
| 16 | Rust's type system cannot express `'tmp != 's`, so privacy is the enforcement mechanism | TRUE | matches private-field pattern throughout ast.rs/names.rs | |

## Stale citation sites
None. All code citation sites (src/postparsing/ast.rs:669,679,693,723,736,741; src/postparsing/names.rs:74,219,712,1063,1144,1207; src/postparsing/function_scout.rs:3; src/postparsing/rules/rule_scout.rs:1; src/postparsing/rules/templex_scout.rs:1) still carry code matching the arcana's description. Note the "Where" section's claim about `ScoutArena::intern_rune()` points to src/scout_arena.rs:568 (not in the doc's own file-list, but src/scout_arena.rs:567,585,93 already carry `@DSAUIMZ` citations and are accurate — this is not a stale site, just outside the "known citation sites" list given for this audit).

## Uncited sites that embody the arcana
None found. Every `promote_in`, `borrow_val`, and `*ValS<'tmp>` construction site checked carries an `@DSAUIMZ` comment either directly or via a file-header comment (function_scout.rs:3, rule_scout.rs:1, templex_scout.rs:1).
