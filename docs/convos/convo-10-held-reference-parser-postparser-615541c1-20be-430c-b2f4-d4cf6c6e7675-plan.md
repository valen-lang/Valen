# Plan document

Source: `/Users/verdagon/.claude/plans/wiggly-meandering-thacker.md`
Session: 615541c1-20be-430c-b2f4-d4cf6c6e7675

---

# Plan: `held` reference at the parser + postparser (region-flavored borrow)

## Context

We're adding a new reference flavor, `held`, to Vale. Semantically `held` is a **borrow into an
anonymous region that the callee treats as undestroyable** — the caller proves, at the call site,
that nothing in the callee or the callee's callees can destroy the referent. It's the reference you
get from merely mentioning a variable, as opposed to an explicit `&` borrow. In the typing pass (a
*later* slice, not this one) `held` will be represented as a `BorrowRef` whose region is
`RegionT::Held`, so there is deliberately **no separate `HeldRef` type**; held is a region flavor of
an ordinary borrow.

This plan covers only the **parser + postparser**, which the architect wants to carry `held` at the
type level as its own surface form while lowering it into the existing borrow machinery. It builds
on a clean parsing+postparsing baseline: the `typing` and `solver` modules are currently unlinked
in `src/lib.rs`, and the suite is **504 passed / 0 failed / 1 ignored**.

The chosen representation (decided with the architect): the borrow's region becomes a three-way
enum, so `held`, an explicit region annotation, and "no annotation" are sibling values in one slot —
which also sets up the future syntax that lets a borrow name its region.

```rust
// parser: src/parsing/ast/templex.rs
pub enum RegionP<'p> {
  Unspecified,                  // &Ship          (no region written; today's `region: None`)
  Held,                         // held Ship      (new)
  Rune(&'p RegionRunePT<'p>),   // &'Ship / &i'Ship (today's `region: Some(rune)`; anon or named)
}
// postparse: src/postparsing/rules/rules.rs
pub enum RegionSR<'s> {
  Unspecified,
  Held,
  Rune(RuneUsage<'s>),
}
```

`held` becomes a reserved keyword and a type prefix (modeled on `weak`/`heap`). It does **not** take
an explicit region (held *is* a region), so `held Ship` only — `held i'Ship` is out of scope here.

## Out of scope — deferred to the typing slice (do NOT touch now)

Typing is unlinked; these are the downstream consumers of the region field and will be updated when
typing re-links (they will not compile against the new enum until then — that is expected):
- `src/typing/types/types.rs:16` — add `RegionT::Held` beside `Iso`/`Default`.
- The lvalue-lookup family (`src/typing/ast/expressions.rs:167` and siblings) — flip the stamped
  region from `RegionT::Default` to `RegionT::Held`.
- `src/typing/rune_typing/rune_type_solver.rs:464-471` — region consumer; match `Rune(r)` to stamp
  `RegionTemplataType`, `Unspecified`/`Held` stamp nothing (later, `Held` → `RegionT::Held`).
- `src/typing/macros/anonymous_interface_macro.rs:420` — `region_rune: None` → `RegionSR::Unspecified`.

## RFIGA

### Slice 1 — Foundation: generalize the borrow region field to the enum (behavior-preserving refactor)

Introduces `RegionP` / `RegionSR` (with an as-yet-unproduced `Held` variant), swaps the
two region fields to them, and rewires every reader/bridge. No new behavior: `&Ship`, `&'Ship`,
`&i'Ship`, `&&R` must parse and scout exactly as before. The `Held` variant is wired end-to-end
through the bridge (`Held → Held`) but unreachable until Slice 2 adds the keyword.

- **R**: Rewrite the existing region-asserting tests to the enum, preserving the same asserted
  behavior (a `match` with a single success arm + `other => panic!(…)`, per the no-conditionals rule):
  - `src/parsing/tests/patterns/type_tests.rs` `borrow_with_region` (`&i'MyStruct`): `borrow_ref.region`
    → match `RegionP::Rune(r)`, assert `r.name…as_str() == "i"`.
  - `src/parsing/tests/patterns/capture_and_type_tests.rs` `capture_with_borrow_tame` (`&R`) and
    `capture_with_self_in_front` (`&&R`): `.region.is_none()` → match `RegionP::Unspecified`.
  - `src/parsing/tests/functions/function_tests.rs:534-535` and `src/parsing/tests/struct_tests.rs:150-153,215-216`:
    `region: None` / `.is_none()` → `RegionP::Unspecified`.
- **F**: `cargo test … --lib` — the rewritten tests fail to compile (`RegionP` doesn't exist yet).
  Report: "Tests are correctly failing, proceeding with implementation."
- **I** (minimum to green):
  - Add `RegionP` enum and change `BorrowRefPT.region` to it (`src/parsing/ast/templex.rs:117-121`).
  - In `parse_ref_prefix` (`src/parsing/templex_parser.rs:289-297`), map `parse_region`: `None →
    Unspecified`, `Some(r) → Rune(r)` (no `held` arm yet).
  - Add `RegionSR` enum and change `BorrowRefSR.region_rune: Option<RuneUsage>` → `region:
    RegionSR` (`src/postparsing/rules/rules.rs:154-160`).
  - Bridge: update `translate_borrow_ref_templex` (`src/postparsing/rules/templex_scout.rs:113-129`)
    to take/emit `RegionSR`, and the two feeder sites (`templex_scout.rs:304-309` normal walk,
    `542-547` signature walk) to translate `RegionP → RegionSR` (`Unspecified→Unspecified`,
    `Rune(r)→Rune(translate r)`, `Held→Held`).
  - Update the one existing non-empty producer, the closure self-param
    (`src/postparsing/function_scout.rs:933-936`): `Some(RuneUsage{…})` → `RegionSR::Rune(RuneUsage{…})`.
  - Update readers: `runes()` collector (`src/postparsing/rules/rules.rs:74-80`) → `Rune(r)` pushes,
    `Unspecified`/`Held` push nothing; parser traverse visitor (`src/parsing/tests/traverse.rs:620-628`)
    and postparse test traverse visitor (`src/postparsing/test/traverse.rs:757-762`) → visit only `Rune(r)`.
- **G**: re-run the rewritten tests; they pass (same behavior, new spelling).
- **A**: full `--lib` suite green (still 504/0/1).

### Slice 2 — Feature: the `held` keyword parses and scouts to a held-flavored borrow

- **R**: add three new tests:
  - Parser `held_ref_type` in `type_tests.rs`: `compile_templex_expect(…, "held MyStruct")` →
    `cast!(…, ITemplexPT::BorrowRef)` → match region `RegionP::Held` and inner name `MyStruct`.
  - Parser `held_and_borrow_ref_type` in `type_tests.rs`: `"held &MyStruct"` → outer BorrowRef region
    `Held`, inner `cast!` BorrowRef region `Unspecified`, inner name `MyStruct`.
  - Postparse `test_param_held_ref_wrap_routing` in `post_parser_tests.rs` (MLVFX multi-line raw
    string, mirroring `test_param_single_ref_wrap_routing`): `func foo(x held int) int { return 0; }`
    → `match foo.params` arm `[ParameterS { type_outer_ref_rules: [IRulexSR::BorrowRef(BorrowRefSR {
    region: RegionSR::Held, result_rune, inner_rune, .. })], value_type_rules:
    [Lookup(CodeName("int"))], full_type_rune, value_type_rune, .. }]` → assert `result_rune ==
    full_type_rune`, `inner_rune == value_type_rune`; `other => panic!(…)`.
- **F**: `cargo test … --lib` — all three fail (`held` is not a keyword, so it never becomes a
  `BorrowRef` with `Held`). Report: "Tests are correctly failing, proceeding with implementation."
- **I** (minimum to green):
  - Add `pub held: StrI<'a>` to `Keywords` (`src/keywords.rs`) and `held: …intern_str("held")` to
    **both** `new_for_parse` and `new_for_scout`.
  - Add a `held` arm in `parse_ref_prefix` (`src/parsing/templex_parser.rs`, modeled on the `weak`/`heap`
    arms at `261`/`270`): `iter.try_skip_word(self.keywords.held)` → parse inner → return
    `ITemplexPT::BorrowRef(BorrowRefPT { range, inner, region: RegionP::Held })`. No `parse_region`
    call (held takes no explicit region). The Slice-1 bridge already carries `Held` into postparse.
- **G**: re-run the three new tests; they pass.
- **A**: full `--lib` suite green (~507/0/1).

## Files to modify

Slice 1 (all in linked parsing/postparsing): `src/parsing/ast/templex.rs`,
`src/parsing/templex_parser.rs`, `src/parsing/tests/traverse.rs`,
`src/parsing/tests/patterns/{type_tests.rs,capture_and_type_tests.rs}`,
`src/parsing/tests/functions/function_tests.rs`, `src/parsing/tests/struct_tests.rs`,
`src/postparsing/rules/rules.rs`, `src/postparsing/rules/templex_scout.rs`,
`src/postparsing/function_scout.rs`, `src/postparsing/test/traverse.rs`.

Slice 2: `src/keywords.rs`, `src/parsing/templex_parser.rs`,
`src/parsing/tests/patterns/type_tests.rs`, `src/postparsing/test/post_parser_tests.rs`.

## Reuse notes (don't reinvent)

- Keyword-prefix parse pattern: the `weak`/`heap` arms in `parse_ref_prefix`
  (`src/parsing/templex_parser.rs:261,270`) — copy their shape for `held`.
- Single BorrowRefSR construction point: `translate_borrow_ref_templex` (`templex_scout.rs:113`) —
  change it once, both walks route through it.
- Test helpers: `cast!` (parsing/tests/utils.rs:483), `compile_templex_expect` (utils.rs:275-302),
  `expect_1/2` (utils.rs:433-479); postparse `compile` + `program.lookup_function` +
  `match foo.params { … => {}, other => panic!(…) }` (see `test_param_single_ref_wrap_routing`,
  `post_parser_tests.rs:1388`).

## Testing rules to honor (good-testing / TDD)

- Vertical slices: one slice's tests written and seen failing before its implementation.
- No conditionals in tests: fold the region check into the `match`/`cast!` pattern (a single
  success arm; all other arms `panic!`), never `if matches!(...)`.
- Assert hand-chosen concrete values (`"i"`, `"MyStruct"`, `StrI("int")`); use `unwrap()` not
  `expect("msg")`; use `expect_N` not assert-len-then-index.
- MLVFX: body-block Vale source (the postparse fixture) goes in a multi-line raw string; the
  parser templex fragments (`"held MyStruct"`) may stay single-line.
- Slice 1 adds **no** new behavior tests — it only rewrites existing tests to the new spelling; the
  new `held` behavior tests all live in Slice 2.

## Verification

- Per-slice F/G: run the touched modules, e.g.
  `cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing::tests::patterns > tmp/held-region-work.txt 2>&1`
  and `… --lib postparsing::test::post_parser_tests > tmp/held-region-work.txt 2>&1`, then
  `grep "test result" tmp/held-region-work.txt`.
- Full regression (A step): `cargo test --manifest-path FrontendRust/Cargo.toml --lib >
  tmp/held-region-work.txt 2>&1`; expect green, ~507 passed / 0 failed / 1 ignored, and **no new
  warnings** (`grep -E "warning|test result" tmp/held-region-work.txt`).
- Do not re-link `typing`/`solver`; they stay unlinked for this slice.
