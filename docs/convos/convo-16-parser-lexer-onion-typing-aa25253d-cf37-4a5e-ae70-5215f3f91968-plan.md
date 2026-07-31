# Plan document

Source: `/Users/verdagon/.claude/plans/please-plan-out-these-transient-balloon.md`
Session: aa25253d-cf37-4a5e-ae70-5215f3f91968

---

# Onion typing — parser layer refactor plan

## Context

This is the **parser + lexer slice** of the larger onion-typing refactor (see `/Volumes/V/Vale2/vcoord-handoff.md`, `/Volumes/V/Vale2/onion-typing-scouting.md`, `/Volumes/V/Vale2/onion-typing-plan.md` for the full arc). Onion typing dissolves the flat `OwnershipT` axis into four structural ref variants (`BorrowRef` / `HeapOwnRef` / `ShareRef` / `WeakRef`) on `KindT`. This parser slice mirrors the same collapse at the parser AST level so surface syntax lowers to structurally-distinct variants from the top of the pipeline.

**Intended outcome**: after this slice lands,
- `cargo test --lib parsing::` and `cargo test --lib lexing::` both pass with the new parser AST shape.
- Downstream stages (postparsing, higher_typing, typing, hammer, backend) do not build — expected. They'll be fixed by later slices. Where the parser must still cross into downstream types (`OwnershipP` consumers in postparsing), a one-shot stub commit inserts `panic!("STUB: onion typing")` to isolate breakage to that boundary.

**Trigger**: locked-in design decisions from an extended architectural discussion (2026-07-03/04). All rule-fate, AST-shape, and surface-syntax decisions are captured in the referenced onion-typing docs.

## Scope

**In scope:**
- `FrontendRust/src/parsing/**` — parser AST, templex + expression parsers, tests.
- `FrontendRust/src/lexing/**` — audit only; no code changes expected (`weak` and `heap` are already interned keywords).
- `FrontendRust/src/parsing/keywords.rs` — no additions (weak/heap already there); possibly deletions in a later slice.
- `FrontendRust/src/postparsing/post_parser_error_humanizer.rs` — `humanize_ownership` deletion.
- `.vale` fixture files that use `&&` for weak.

**Out of scope (expected to break):**
- All rule scouts, higher typer, typing pass, hammer, backend, testvm, docs.
- Every downstream consumer of `OwnershipP`, `InterpretedPT`, `AugmentPE`.

## Design summary

### Templex AST (`ITemplexPT`) — 4-way split

Delete `ITemplexPT::Interpreted(InterpretedPT)`. Add:

```rust
ITemplexPT::BorrowRef  { range, inner: &'p ITemplexPT<'p>, region: Option<&'p RegionRunePT<'p>> }
ITemplexPT::HeapOwnRef { range, inner: &'p ITemplexPT<'p> }
ITemplexPT::ShareRef   { range, inner: &'p ITemplexPT<'p> }
ITemplexPT::WeakRef    { range, inner: &'p ITemplexPT<'p> }
```

Also delete atom variants `ITemplexPT::Ownership(OwnershipPT)` and `ITemplexPT::Share(SharePT)` — these were the `own`/`borrow`/`weak`/`share` literal atoms used inside `where M Ownership = borrow`. The whole Ownership rune-type axis is dying; no replacement.

Delete `InterpretedPT` struct and `OwnershipPT` tuple type. `SharePT` deletes with the atom `Share` variant.

### Expression AST (`IExpressionPE`) — 4-way split

Delete `IExpressionPE::Augment(AugmentPE)`. Add:

```rust
IExpressionPE::Move   { range, inner: &'p IExpressionPE<'p> }   // ^x
IExpressionPE::Borrow { range, inner: &'p IExpressionPE<'p> }   // &x
IExpressionPE::Weak   { range, inner: &'p IExpressionPE<'p> }   // weak x (new surface)
IExpressionPE::Share  { range, inner: &'p IExpressionPE<'p> }   // @x (new surface)
```

Delete `AugmentPE` struct.

### `OwnershipP` deletion

Delete the entire enum (`ast.rs:322-329`, variants Own/Borrow/Live/Weak/Share). Everything that referenced it in the parser is now gone via the two splits above.

### Surface prefix table after refactor

| Surface | Templex → | Expression → |
|---|---|---|
| `&T` / `&x` | `BorrowRef { inner: T, region: None }` | `Borrow { inner: x }` |
| `&r'T` | `BorrowRef { inner: T, region: Some(r) }` | (n/a) |
| `&&T` / `&&x` | `BorrowRef { inner: BorrowRef { .. }, region: None }` (**flip**) | `Borrow { inner: Borrow { inner: x } }` (**flip**) |
| `heap T` | `HeapOwnRef { inner: T }` (**new**) | (n/a) |
| `@T` / `@x` | `ShareRef { inner: T }` | `Share { inner: x }` (**new**) |
| `weak T` / `weak x` | `WeakRef { inner: T }` (**new**) | `Weak { inner: x }` (**new**) |
| `^T` | **parse error** (removed) | (n/a) |
| `^x` | (n/a) | `Move { inner: x }` |
| `inl x` | (n/a) | `Move { inner: x }` (remapped from Own) |
| Region-only `r'T` alone | **parse error** (removed) | (n/a) |

`heap` at templex level: existing atom use at `templex_parser.rs:374` (`LocationP::Yonder` for `Location` rune-type values inside `where M Location = heap`) **survives**. New `heap T` prefix parsing dispatches at prefix-position only — atom-position `heap` untouched. Same disambiguation the parser already uses for other position-sensitive tokens.

### Where-clause builtins dying at this slice

Delete tests (parser has no source change; generic builtin dispatch is unaffected):
- `Prot[...]` — `PrototypeComponentsSR` dies downstream.
- `any(...)` — `OneOfSR` dies.
- `isInterface(...)` — `IsInterfaceSR` dies.
- `isConcrete(...)` — `IsConcreteSR` dies.
- `refListCompoundMutability(...)` — `RefListCompoundMutabilitySR` dies.

## Sequencing — RFIGA slices

Additive-first: every new variant / prefix lands before the old dies. Between slices `cargo test --lib parsing::` stays green. The final cleanup slice (C3) intentionally cascades into downstream breakage; land it after all parser-side work is done.

Each slice follows the RFIGA convention from `docs/skills/tdd.md`: **R** (write red test), **F** (verify it fails), **I** (implement), **G** (verify green), **A** (audit full suite still green).

### Templex slices

- **T1. Add `BorrowRef`; migrate `&T`.**
  R: rewrite `patterns/capture_and_type_tests.rs::capture_with_borrow_tame` (~54-68), `patterns/type_tests.rs::static_sized_array_with_borrow`, `functions/function_tests.rs::func_with_func_bound`, `kind_rule_tests.rs::rwkilc` sub-case to assert `ITemplexPT::BorrowRef { inner, region: None }`.
  I: `parse_interpreted` line 278 emits `BorrowRef` instead of `Interpreted{Borrow, ..}`.

- **T2. `&r'T` region-carrying borrow.**
  R: new test `borrow_with_region` in `patterns/type_tests.rs` — source `&i'MyStruct`, assert `BorrowRef { inner, region: Some(_) }`.
  I: extend the `BorrowRef` construction to consume the trailing region rune from `parse_region`.

- **T3. Delete region-only Interpreted surface.**
  R: rewrite or delete `functions/function_tests.rs::return_pure_immutable` (~215, uses `'int`) and `return_isolate` (~236, uses `i'int`). If the region-alone form is genuinely gone, both tests get deleted with a comment referencing this slice. If they're rewritten to include an explicit ownership prefix, they assert the corresponding `BorrowRef` shape.
  I: delete the region-only branch in `parse_interpreted:288-291` (the `(None, Some(_))` case). Region-only `r'T` becomes a parse error.

- **T4. Add `WeakRef` + `weak T` prefix + `&&T` flip.**
  R: (a) rewrite `struct_tests.rs::struct_with_weak` (~136-166), `patterns/type_tests.rs::static_sized_array_with_weak` (~124-146), `patterns/capture_and_type_tests.rs::capture_with_self_in_front` (~70-91) — each currently asserts `OwnershipP::Weak` on `&&T`; flip to assert `BorrowRef { inner: BorrowRef { .. }, .. }` (double-borrow). (b) new test `weak_prefix_type` for source `weak T` → `WeakRef { inner: NameOrRune("T") }`.
  I: in `parse_interpreted:276`, don't collapse `&&` to Weak — parse one `&`, recurse to consume another `&` as an inner `BorrowRef`. Add `try_skip_word(self.keywords.weak)` branch that emits `WeakRef`.

- **T5. Add `ShareRef`; migrate `@T`.**
  R: update `kind_rule_tests.rs` any `Moo<@int>` sub-case to assert `ShareRef { inner: NameOrRune("int") }`.
  I: `parse_interpreted:274` emits `ShareRef` instead of `Interpreted{Share, ..}`.

- **T6. Add `HeapOwnRef` + `heap T` prefix (position-sensitive).**
  R: new test `heap_prefix_type` — source `heap T`, assert `HeapOwnRef { inner: NameOrRune("T") }`. Keep existing atom-position `location`-atom test at `rules_enums_tests.rs:143` untouched.
  I: in `parse_interpreted`, add a `try_skip_word(self.keywords.heap)` branch before falling through to `parse_templex_atom`. Add a comment at `parse_templex_atom:374` noting that atom-position `heap` still means `LocationP::Yonder` and only prefix-position `heap` means `HeapOwnRef`.

- **T7. Reject `^T` at templex.**
  R: new test `caret_type_is_error` using `compile_templex_for_error` (or the moral equivalent — audit `utils.rs` for existing error-compile helpers; use the same pattern) — asserts source `^T` errors. Delete `struct_tests.rs::struct_with_heap` (~168-198) which currently tests `x ^Marine` — this surface is gone; if a "field type is heap-own" test is desired, add it under T6's `heap T` shape instead.
  I: remove the `^` branch from `parse_interpreted:272`.

### Expression slices

- **E1. Add `IExpressionPE::Borrow`.**
  R: rewrite `expression_tests.rs::borrowing_result_of_function_call` (~236-259), any tests in `if_tests.rs` / `statement_tests.rs` / `expression_tests.rs` (~415-430 template-call args) that currently assert `Augment{Borrow, ..}` → assert `Borrow { inner }`.
  I: `expression_parser.rs:1939-1946` `&` branch emits `Borrow` instead of `Augment{Borrow, ..}`.

- **E2. Add `IExpressionPE::Move`; migrate `^x` and `inl x`.**
  R: rewrite `expression_tests.rs::specifying_heap` (~261-275) and `inline_call_ignored` (~277-302) → assert `Move { inner }`. Note both current test names are misnamed; keep the names for grep-ability during transition, or rename during E2's A step.
  I: `expression_parser.rs:1935-1938` `^` branch and `:1949-1952` `inl` branch both emit `Move` instead of `Augment{Own, ..}`.

- **E3. Add `IExpressionPE::Weak`; add `weak x` prefix; flip `&&x` → double-Borrow.**
  R: (a) **new gap-filling test** `weak_expression_via_double_borrow` — source `&&x` → `Borrow { inner: Borrow { inner: Lookup("x") } }`. This test does not exist today; adding it plugs a real coverage gap. (b) new test `weak_expression_via_keyword` — source `weak x` → `Weak { inner: Lookup("x") }`.
  I: `expression_parser.rs:1942-1945` — don't collapse `&&` to Weak; treat as nested Borrow. Add `try_skip_word(self.keywords.weak)` branch emitting `Weak`.

- **E4. Add `IExpressionPE::Share` (`@x`).**
  R: new test `at_share_expression` — source `@x` → `Share { inner: Lookup("x") }`.
  I: `expression_parser.rs:1934` prefix match — add `@` symbol branch emitting `Share`.

- **E5. Trailing-`&` at `parse_spree_step`.**
  R: audit `expression_tests.rs` for a test on trailing `&` (e.g., `foo()&`); add `trailing_ampersand_is_borrow` if missing.
  I: `expression_parser.rs:1391-1398` — construct `Borrow { inner }` instead of `Augment{Borrow, ..}`.

### Cleanup slices

- **C1. Delete tests for dying `where`-clause builtins.**
  R: delete `parsing/tests/rules/rule_tests.rs::destructure_prototype` (~88-101), `prototype_with_coords` (~115-134), delete `rules/rules_enums_tests.rs::ownership` (~22-96 — uses `any(own, borrow, weak)`), delete the `any(...)`-shaped sub-case of `rules_enums_tests.rs::location` if present, delete matching tests in `rules/coord_rule_tests.rs`.
  I: none — parser generic builtin dispatch at `templex_parser.rs::parse_rule_call:606+` is untouched.

- **C2. Delete atom `own`/`borrow`/`weak`/`share` keywords at templex atom position.**
  R: none new (any test that would fail was already deleted in C1).
  I: remove the `try_skip_word(self.keywords.own/borrow/weak/share)` blocks at `templex_parser.rs:356-388` — both the first block AND the duplicate dead block at 380-388.

- **C3. Delete `InterpretedPT`, `OwnershipPT`, `AugmentPE`, `SharePT`, atom `ITemplexPT::{Ownership, Share}`, `IExpressionPE::Augment`, `OwnershipP`, `humanize_ownership`.**
  R: none — after T- and E-series, no parser code or test references them.
  I: enum-variant deletions in `parsing/ast/templex.rs` (lines 20, 21, 26 for variants; 147-160, 164 for structs), `parsing/ast/expressions.rs` (line 37; 410-415), `parsing/ast/ast.rs:322-329` (OwnershipP). Delete `humanize_ownership` at `postparsing/post_parser_error_humanizer.rs:369-379`. Any downstream `OwnershipP::*` matches: stub with `panic!("STUB: onion typing — parser refactor removed OwnershipP")`. `IRulexSR::Augment` humanizer at post_parser_error_humanizer.rs:341 also stubs. `AugmentSR` struct at `postparsing/rules/rules.rs:318-324` may need its `ownership: Option<OwnershipP>` field stubbed — expect that whole struct to die in a later downstream slice.
  A: parser tests all pass. `cargo build --lib` may fail — that's expected and captured in the slice notes.

- **C4. Fixture rewrites.**
  R: none — parser tests already covered.
  I: mechanical `&&` → `weak ` substitution across:
    - `src/builtins/resources/weak.vale`
    - `src/builtins/resources/clone.vale`
    - `src/tests/programs/weaks/callWeakSelfMethodAfterDrop.vale`
    - `src/tests/programs/weaks/callWeakSelfMethodWhileLive.vale`
    - `src/tests/programs/weaks/dropThenLockInterface.vale`
    - `src/tests/programs/weaks/dropThenLockStruct.vale`
    - `src/tests/programs/weaks/dropWhileLockedInterface.vale`
    - `src/tests/programs/weaks/dropWhileLockedStruct.vale`
    - `src/tests/programs/weaks/lockWhileLiveInterface.vale`
    - `src/tests/programs/weaks/lockWhileLiveStruct.vale`
    - `src/tests/programs/weaks/weakFromCRefInterface.vale`
    - `src/tests/programs/weaks/weakFromCRefStruct.vale`
    - `src/tests/programs/weaks/weakFromLocalCRefInterface.vale`
    - `src/tests/programs/weaks/weakFromLocalCRefStruct.vale`
  Both templex-level (`&&T` → `weak T`) and expression-level (`&&x` → `weak x`) sites. Each file's occurrences audited by context.
  A: `parse_sample_test!` samples (`parse_samples_tests.rs:137-149`) still parse-succeed under the new surface.

## Traverser + humanizer boilerplate

Piggyback each traverser/humanizer update onto its corresponding slice's **I** step, so no slice leaves the traverser out of sync with the AST.

**`parsing/tests/traverse.rs`:**
- `visit_templex` (line 620-651) — `Interpreted` arm (637-651) → replace with 4 new arms (`BorrowRef` visits `inner` + optional `region`; `HeapOwnRef` / `ShareRef` / `WeakRef` each visit `inner`).
- Atom `Ownership` and `Share` arms in `visit_templex` — delete (in C2 / C3).
- `visit_expression` `Augment` arm (line 983-990) → replace with `Move` / `Borrow` / `Weak` / `Share` arms, each a `visit_expression(inner)` one-liner.
- `visit_ownership` helper (line 1127-1133) → delete in C3.
- `NodeRefP::Ownership` variant (line 1164) → delete in C3.

**Humanizer** — `humanize_ownership` at `postparsing/post_parser_error_humanizer.rs:369-379` deletes in C3. `IRulexSR::Augment` humanizer at line 341 stubs in C3 (whole IRulexSR::Augment humanization goes away with AugmentSR downstream; stub for now).

## Coverage gaps filled (new tests required)

Beyond the update-existing-tests work, these are net-new tests to write:

| Test | Location | Slice |
|---|---|---|
| `borrow_with_region` | `patterns/type_tests.rs` | T2 |
| `weak_prefix_type` | `patterns/type_tests.rs` or new `templex_prefix_tests.rs` | T4 |
| `heap_prefix_type` | same | T6 |
| `caret_type_is_error` | `struct_tests.rs` or new `templex_error_tests.rs` | T7 |
| `weak_expression_via_double_borrow` | `expression_tests.rs` (plugs an existing coverage gap) | E3 |
| `weak_expression_via_keyword` | `expression_tests.rs` | E3 |
| `at_share_expression` | `expression_tests.rs` | E4 |
| `trailing_ampersand_is_borrow` (if missing) | `expression_tests.rs` | E5 |

## Critical files

Modify:
- `FrontendRust/src/parsing/ast/templex.rs` — AST changes (T1/T4/T5/T6, deletions in C3).
- `FrontendRust/src/parsing/ast/expressions.rs` — AST changes (E1-E4, deletions in C3).
- `FrontendRust/src/parsing/ast/ast.rs` — `OwnershipP` deletion (C3).
- `FrontendRust/src/parsing/templex_parser.rs` — `parse_interpreted` (T1-T7), `parse_templex_atom` (C2).
- `FrontendRust/src/parsing/expression_parser.rs` — main augment path (E1-E4), `parse_spree_step` (E5).
- `FrontendRust/src/parsing/tests/traverse.rs` — traverser arms updated per slice, deletions in C3.
- `FrontendRust/src/postparsing/post_parser_error_humanizer.rs` — `humanize_ownership` deletion, `IRulexSR::Augment` stub (C3).
- Existing parser test files at the paths listed in each slice.
- 14 `.vale` fixture files listed under C4.

**Reuse (do not reimplement):**
- `FrontendRust/src/parsing/tests/utils.rs` — helpers `compile_templex_expect` (275-302), `compile_expression_expect`, `compile_struct_expect`, `compile_rulex_expect`. `cast!` macro (483-491) for enum unwrapping.
- `FrontendRust/src/parsing/tests/utils.rs::compile_*_for_error` — for negative tests (T7).
- `FrontendRust/src/parsing/keywords.rs` — `weak` (line 16) and `heap` (line 21) already interned; no additions needed.

## Verification

After each slice (RFIGA **A** step):
```
cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing:: > tmp/onion-parser-refactor.txt 2>&1
grep "test result" tmp/onion-parser-refactor.txt
```

After the whole slice-series (post-C4):
```
cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing:: --no-fail-fast > tmp/onion-parser-refactor.txt 2>&1
cargo test --manifest-path FrontendRust/Cargo.toml --lib lexing:: --no-fail-fast >> tmp/onion-parser-refactor.txt 2>&1
grep "test result" tmp/onion-parser-refactor.txt
```

**Success criteria:**
- All parsing + lexing tests pass. No `#[ignore]` additions.
- `cargo build --lib` may or may not build depending on downstream stubbing coverage — this is by design.
- No test failures anywhere in `parsing::` or `lexing::`.
- All 14 `.vale` fixtures parse without error (though their downstream compilation is expected to break).

**Fixture parse validation** (spot-check):
```
cargo test --manifest-path FrontendRust/Cargo.toml --lib parse_sample -- --no-fail-fast >> tmp/onion-parser-refactor.txt 2>&1
```
The `parse_sample_test!` macro samples 070-082 (`weaks/*.vale`) should still parse-succeed post-C4 (they now produce `weak T` / `weak x` structurally, not `&&`).

**Manual spot check** — verify the new humanizer output isn't needed; parse-error surface is the only humanizer touch in the parser layer (`parse_error_humanizer.rs` at 151 lines doesn't render AST).

## Rollback

If the slice-series stalls mid-flight, roll back to the pre-T1 commit. All slices in the parser layer are grouped by AST-additive design, so the tree is either "all old shape" or "all new shape" at commit boundaries. No half-migrated state should ship.
