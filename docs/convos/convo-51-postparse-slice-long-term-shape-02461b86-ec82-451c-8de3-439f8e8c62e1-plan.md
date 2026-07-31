# Plan document

Source: `/Users/verdagon/.claude/plans/this-session-lets-just-zippy-star.md`
Session: 02461b86-ec82-451c-8de3-439f8e8c62e1

---

# Postparse Slice — Long-Term Shape

## Context

Onion typing arc. Parser + lexer already landed at long-term shape (`b5bde70e6`). Everything from postparsing onward is commented out in `lib.rs`. This session brings **postparsing** back online at its long-term end-state — no gates, no preserved-but-dead code, no half-measures.

**Long-term direction (settled in this session):**
- Postparse does **zero solving**. Only naming, scope tables, expression lowering, non-typing errors.
- Higher_typing dies entirely (its work moves to typing entry + rune-type walker in typing/).
- Rune-type solver library moves to `typing/` (not postparse).
- Postparse doesn't populate `predicted_rune_to_type` — that field dies.

**For this session:** land postparse's long-term shape. Higher_typing and downstream stay unlinked (their slices come later). Solver files that don't belong in postparse long-term get **deleted** (not gated).

**Reference docs:**
- `vcoord-handoff.md` — architectural direction (updated this session).
- `postparse-slice-plan.md` — high-level plan for enum-level type changes; treat as canonical for what deletes/renames/adds.

## Scope decisions locked

- **`predict_rune_types` in `post_parser.rs:1409`:** deleted. `rune_to_predicted_type` / `predicted_rune_to_type` / `header_predicted_rune_to_type` / `members_predicted_rune_to_type` fields on function/citizen AST nodes: deleted.
- **`identifiability_solver.rs` (~270 LOC):** deleted. `check_identifiability` in post_parser.rs: deleted. `IdentifyingRunesIncompleteS` error variant: deleted (typing will re-add its own version).
- **`rune_type_solver.rs` (~810 LOC):** deleted from postparse. Later slices rebuild as walker in typing/rune_typing/. `RuneExplicitTypeConflictS`: deleted (typing will re-add).
- **Higher_typing + all later modules:** stay unlinked (they already are). Their slices come later.

## Sub-slices

### P1. IRulexSR + ILiteralSL + ITemplataType enum surface

**Files:** `postparsing/rules/rules.rs`, `postparsing/itemplatatype.rs`.

- **`IRulexSR` (26 → 14 variants).** Delete: `CoordSend`, `DefinitionCoordIsa`, `CallSiteCoordIsa`, `CoordComponents`, `CoerceToCoord`, `KindComponents`, `PrototypeComponents`, `Augment`, `OneOf`, `IsInterface`, `IsConcrete`, `IsStruct`, `RefListCompoundMutability`, `IndexList`, `MaybeCoercingLookup`, `MaybeCoercingCall`. Rename `Pack` → `KindList`. Add `BorrowRefSR { range, result_rune, inner_rune, region_rune: Option<RuneUsage> }`, `HeapOwnRefSR { range, result_rune, inner_rune }`, `ShareRefSR { range, result_rune, inner_rune }`, `WeakRefSR { range, result_rune, inner_rune }`.
- **`ILiteralSL` (5 → 3).** Delete `LocationLiteral`, `OwnershipLiteral`.
- **`ITemplataType` (13 → 9).** Delete `CoordTemplataType`, `OwnershipTemplataType`, `LocationTemplataType`, `PrototypeTemplataType`.
- Update `range()` / `rune_usages()` match arms accordingly.

Postparse still unlinked; cargo compiles nothing at this point. Move on when `rules.rs` + `itemplatatype.rs` are internally consistent.

### P2. Names, ASTs, patterns

**Files:** `postparsing/names.rs`, `postparsing/ast.rs`, `postparsing/patterns/patterns.rs`.

- **`IRuneS` + `INameValS` — rename 8 `*CoordRune` variants to `*KindRune`:** `ImplDropCoordRune`, `SelfCoordRune`, `MacroVoidCoordRune`, `MacroSelfCoordRune`, `AnonymousSubstructParentInterfaceCoordRune`, `AnonymousSubstructCoordRune`, `AnonymousSubstructVoidCoordRune`, `AnonymousSubstructMethodSelfBorrowCoordRune`. Rename companion `*S` structs and `ValS` companions in the same edit (interner-key drift landmine).
- **`IRuneS` + `INameValS` — delete 3 `*OwnershipRune` variants:** `ImplicitCoercionOwnershipRune`, `SelfOwnershipRune`, `AnonymousSubstructMethodSelfOwnCoordRune` (with `ValS` companions).
- **`AtomSP.coord_rune` → `kind_rune`** (`patterns/patterns.rs:17`). ~193 call sites across postparse rename.
- **`ast.rs`:**
  - Delete `enum IRegionMutabilityS` (only `ReadWriteRegion` constructible after parser slice); delete `RegionGenericParameterTypeS.mutability` field.
  - Delete `CoordGenericParameterTypeS`'s three dead fields (`coord_region`, `kind_mutable`, `region_mutable`). Rename struct to `KindGenericParameterTypeS` and variant `IGenericParameterTypeS::CoordGenericParameterType` → `KindGenericParameterType`. Its `tyype()` returns `KindTemplataType`.
  - Delete `predicted_rune_to_type` / `header_predicted_rune_to_type` / `members_predicted_rune_to_type` fields on `StructS` / `InterfaceS`. Delete `rune_to_predicted_type` on `FunctionS`.

### P3. Delete solver files + callers

**Files:** `postparsing/rune_type_solver.rs`, `postparsing/identifiability_solver.rs`, `postparsing/post_parser.rs`, `postparsing/mod.rs`.

- Delete `rune_type_solver.rs` outright.
- Delete `identifiability_solver.rs` outright.
- In `post_parser.rs`:
  - Delete `PostParser::check_identifiability` (~line 1455).
  - Delete `PostParser::predict_rune_types` (~line 1409).
  - Delete callers of both (function_scout.rs:782,794; post_parser.rs:1271,1615).
  - Delete `enum ICompileErrorS`'s `IdentifyingRunesIncompleteS` variant + `IdentifyingRunesIncompleteS` struct.
  - Delete `RuneExplicitTypeConflictS` variant + struct.
- In `postparsing/mod.rs`: delete `pub mod rune_type_solver;` + `pub mod identifiability_solver;`.
- Remove now-unused imports throughout postparse.

### P4. Scout stage rewrites

**Files:** `postparsing/rules/templex_scout.rs`, `postparsing/rules/rule_scout.rs`, `postparsing/expression_scout.rs`, `postparsing/patterns/pattern_scout.rs`, `postparsing/function_scout.rs`.

- **`templex_scout.rs`:** dispatch on 4 new `ITemplexPT` variants → 4 new `*RefSR` emissions. Result-rune stamp flip: line 565 `CoordTemplataType` → `KindTemplataType`.
- **`expression_scout.rs`:** dispatch on the 4 new `IExpressionPE` variants (`Move`/`Borrow`/`Weak`/`Share`) → `OwnershippedSE { target_ownership: LoadAsP::{Move, LoadAsBorrow, LoadAsWeak, LoadAsShare} }`. Delete now-unreachable Augment dispatch.
- **`rule_scout.rs`:**
  - Delete branches for retired where-clause builtins: `any(...)`, `isInterface(...)`, `isConcrete(...)`, `refListCompoundMutability(...)`, `Prot[...]`.
  - `refs(...)` → emit `KindListSR` (renamed).
  - Delete `ITypePR::CoordType` and `ITypePR::LocationType` arms; keep `KindType` → `KindTemplataType`.
  - Flip `CoordTemplataType` stamps → `KindTemplataType` at lines 172, 174, 220, 366.
- **`pattern_scout.rs:67`:** `CoordTemplataType` → `KindTemplataType`. Field rename `coord_rune` → `kind_rune` cascades.
- **`function_scout.rs`:** ~6 stamp sites (381, 434, 534, 565, 883, 948) `CoordTemplataType` → `KindTemplataType`. Drop the dead `coord_region: None, kind_mutable: true, region_mutable: false` payload at lines 472-474, 713-715 (`CoordGenericParameterTypeS` restructure).

### P5. Traverser + humanizer + tests

**Files:** `postparsing/test/traverse.rs`, `postparsing/post_parser_error_humanizer.rs`, all files under `postparsing/test/*.rs`.

- **`test/traverse.rs`:** delete 16 arms for retired SR variants; add 4 arms for `*RefSR` variants; rename `Pack` → `KindList`. Delete `NodeRefS` variants for retired `Ownership`, `AugmentRule`, `OneOf`, `IsInterface`, etc. Update `ILiteralSL` match to drop `LocationLiteral` / `OwnershipLiteral`.
- **`post_parser_error_humanizer.rs`:** delete `humanize_ownership` function (`OwnershipP` already gone at parser). Delete arms for retired SR variants (many are already `panic!("implement:")` stubs). Add 4 humanizer arms:
  - `BorrowRefSR` → `&<inner>` (with `<region>' <inner>` when `region_rune.is_some()`)
  - `HeapOwnRefSR` → `heap <inner>`
  - `ShareRefSR` → `@<inner>`
  - `WeakRefSR` → `weak <inner>`
- **Test deletions:**
  - `post_parsing_rule_tests.rs` (7 tests): delete `predict_for_is_interface`, `predict_knows_type_from_or_rule` (uses retired `any(...)`), and the 5 remaining `predict_*` tests (they all assert on `main.rune_to_predicted_type` which no longer exists). Whole file may become deletable if all 7 tests go.
  - Any test asserting `CoordTemplataType` / `OwnershipTemplataType` / `LocationTemplataType` / `PrototypeTemplataType`: update to `KindTemplataType` or delete if the assertion tested a retired feature. Concentrated in `after_regions_error_tests.rs:13,193`.
- **Test updates for MaybeCoercing rename** — `post_parsing_parameters_tests.rs:197,226`, `post_parser_tests.rs:139,154,296,307,318`: change `MaybeCoercingLookup` → `Lookup` (post-merge). Similarly for MaybeCoercingCall if any tests use it.
- **Test updates for AugmentSR** — `post_parsing_parameters_tests.rs:150,182`: rewrite `borrowed_rune` to assert on `BorrowRefSR` emission instead of `AugmentSR`.

### P6. Re-link + un-gate + verify

**Files:** `FrontendRust/src/lib.rs`, `FrontendRust/src/keywords.rs`, `FrontendRust/src/scout_arena.rs`, `FrontendRust/src/utils/range.rs`, `FrontendRust/src/utils/code_hierarchy.rs`.

- **Uncomment `pub mod postparsing;`** in `lib.rs` (remove the `// TEMP:` marker).
- **Un-gate the scout_arena stubs:** remove `#[cfg(any())]` from:
  - `scout_arena.rs` (file-level in mod.rs / lib.rs)
  - `Keywords::new_for_scout` in `keywords.rs`
  - `RangeS::internal`, `RangeS::test_zero`, `CodeLocationS::internal`, `CodeLocationS::test_zero` in `utils/range.rs`
  - `FileCoordinate::test`, `PackageCoordinate::internal`, top-level `test<>` fn, `FileCoordinateMap::test` in `utils/code_hierarchy.rs`
- **Higher_typing + all later modules stay unlinked.** They'll turn on in their own slices.

**Build target:** `cargo build --lib` clean.

**Test target:**
- `parsing::tests`: still 397/0/1 (unchanged)
- `lexing`: still 3/0/0 (unchanged)
- `postparsing::tests`: green (count will drop from today's number because we deleted the `predict_*` tests + any tests using retired features)

## Landmines

- **Interner-key drift on `IRuneS` renames.** Every `*CoordRune` variant AND its `*CoordRuneValS` companion must rename together in the same commit. Miss one, silent lookup failures.
- **`predict_rune_types` deletion cascades to citizen ASTs.** `StructS::new` / `InterfaceS::new` / `FunctionS::new` constructor signatures change. All construction sites in `post_parser.rs` update in the same edit.
- **`ICompileErrorS::IdentifyingRunesIncompleteS` deletion** shifts variant numbering. Its downstream error consumers (humanizer, tests) must delete matching arms simultaneously.
- **`OwnershipP` at parser is already deleted.** Any remaining reference in postparse (`expression_scout.rs`, `loop_post_parser.rs`, `post_parser_error_humanizer.rs`) needs the 4-variant `IExpressionPE` dispatch replacement.
- **Test file deletion.** If `post_parsing_rule_tests.rs` becomes fully empty after deletions, remove its `mod` entry in `postparsing/test/mod.rs`.
- **`humanize_ownership` may already be dead-linked.** With `OwnershipP` gone at parser, this function has no callers. Verify + delete cleanly.
- **Anon-interface macro** (`typing/macros/anonymous_interface_macro.rs`) references retired SR variants in `panic!("implement:")` stubs. Since `typing/` stays unlinked, these are inert this session. Cleaned up in typing slice.

## Verification

Ordered from cheapest to most thorough:

1. `cargo check --manifest-path FrontendRust/Cargo.toml --lib > tmp/postparse-slice.txt 2>&1` — expect clean build after P6.
2. `cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing:: > tmp/postparse-slice.txt 2>&1` then `grep "test result" tmp/postparse-slice.txt` — expect 397/0/1 unchanged.
3. `cargo test --manifest-path FrontendRust/Cargo.toml --lib lexing:: > tmp/postparse-slice.txt 2>&1` — expect 3/0/0 unchanged.
4. `cargo test --manifest-path FrontendRust/Cargo.toml --lib postparsing:: --no-fail-fast > tmp/postparse-slice.txt 2>&1` — new green baseline.
5. Full suite `cargo test --manifest-path FrontendRust/Cargo.toml --lib --no-fail-fast > tmp/postparse-slice.txt 2>&1` — confirm nothing else re-linked accidentally.

All output goes to the single fixed file `tmp/postparse-slice.txt` per CLAUDE.md convention; inspect via separate `grep`/`tail` commands.

**Success criterion:** postparse builds clean at its long-term end-state shape, its tests are green, parser + lexer suites unchanged, higher_typing + all downstream still unlinked and untouched.

## Non-goals for this session

- Moving `rune_type_solver.rs` to `typing/rune_typing/` — happens in higher_typing collapse slice.
- Re-adding `RuneExplicitTypeConflictS` / `IdentifyingRunesIncompleteS` at typing entry — happens in higher_typing collapse slice.
- Populating `coutputs.type_name_to_rune_types` — happens when typing links.
- Deleting `higher_typing_pass.rs` file itself — happens in higher_typing collapse slice (this session leaves the source untouched but unlinked).
- Anon-interface macro rewrites — happens in typing slice.
- Value solver shrink in `compiler_solver.rs` — happens in typing slice.
