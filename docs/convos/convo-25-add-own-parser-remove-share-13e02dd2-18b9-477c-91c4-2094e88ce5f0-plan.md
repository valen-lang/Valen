# Plan document

Source: `/Users/verdagon/.claude/plans/partitioned-kindling-origami.md`
Session: 13e02dd2-18b9-477c-91c4-2094e88ce5f0

---

# Plan: Add `own` (parser + postparse → new `OwnRef` wrap); remove the `share` keyword

## Context

Following the Valen `own` correction (this session): `own` does **not** fully retire. At **struct kind**
it's redundant with bare (owned), but at **class kind** `own T` is the *exclusive state* — the sole
reference, with no weak/strong/shared refs allowed to form while `own` — and it's the receiver of a class
destructor (`func drop(own self)`). So `own` must be a real surface prefix, parsed and postparsed.

Per the architect, `own` is **not** a respelling of the removed `heap`: it gets its **own new `OwnRef`
wrap** at the surface (`OwnRefPT` / `OwnRefSR`), which typing will map to `HeapOwnRefT` (or a future
`OwnT`) when it re-links.

This also finishes the ownership-keyword cleanup:
- **`share` keyword** — a dead registered field (unused; the `@` sigil + `ShareRef` surface path already
  went in the removal bundle). Remove it.
- **`borrow` keyword** — already fully gone (removal bundle, Slice 1). Nothing to do; verified in the sweep.

**Scope: parser + postparse only.** `own` is type-position (`own T`, and `own self` = the receiver's
type); there is no expression-position `own x` (move stays `^x`). Typing is unlinked, so the
`OwnRefSR → HeapOwnRefT` seam is deferred and flagged.

## Baseline

Live suite = lexing + parsing + postparsing (typing/solver unlinked), green at **504/0/1, 0 warnings**
(post removal-bundle). Each slice keeps it green.

## Template to mirror — `WeakRef`

`own` is a region-less outer wrap, **structurally identical to `WeakRef`** (and to the `HeapOwnRef` path
just removed). Mirror `WeakRef` at every site:

- **Parser:** `WeakRefPT` (`parsing/ast/templex.rs`), `ITemplexPT::WeakRef` + its `range()` arm,
  `ast/mod.rs` re-export, `ast/rules.rs` rune-declaration arm, the `weak` arm in `parse_ref_prefix`
  (`parsing/templex_parser.rs`), and the parser traverse arm (`parsing/tests/traverse.rs`).
- **Postparse:** `WeakRefSR` (`postparsing/rules/rules.rs`), `IRulexSR::WeakRef` + its `range()` /
  `rune_usages()` arms, `translate_weak_ref_templex` + its two dispatch arms (in `translate_templex` and
  `translate_signature_templex`, `postparsing/rules/templex_scout.rs`), the humanizer arm
  (`post_parser_error_humanizer.rs`), the onion-wrap permitted-list `matches!` in `postparsing/ast.rs`,
  and the postparse traverse arm (`postparsing/test/traverse.rs`).
- **Tests:** the (deleted) `heap_prefix_type` shape for the parser test; `test_param_held_ref_wrap_routing`
  (`postparsing/test/post_parser_tests.rs`) for the postparse routing test.

The `own` keyword already exists (`keywords.rs:14,161,319`) — no keyword to add.

## RFIGA slices

### Slice 1 — `own T` parses to `ITemplexPT::OwnRef` (parser)
- **R:** add `own_prefix_type` (mirror the old `heap_prefix_type`): `compile("_ own T")`,
  `cast!(…, ITemplexPT::OwnRef)`, assert inner name `T`, `destructure.is_none()`.
- **F:** run it; fails (`OwnRef` doesn't exist yet). Report "Tests are correctly failing, proceeding."
- **I:** add `OwnRefPT { range, inner }` + `ITemplexPT::OwnRef` variant + its `range()` arm
  (`templex.rs`); `OwnRefPT` re-export (`ast/mod.rs`); the `own` rune-decl arm (`ast/rules.rs`); the
  `own` arm in `parse_ref_prefix` (`templex_parser.rs`, mirror `weak`); the parser traverse arm
  (`parsing/tests/traverse.rs`).
- **G:** re-run `own_prefix_type`; passes.
- **A:** full suite green, zero warnings.

### Slice 2 — `own T` routes to `IRulexSR::OwnRef` (postparse)
- **R:** add `test_param_own_ref_wrap_routing` (mirror `test_param_held_ref_wrap_routing`): a param typed
  `own T` has an `IRulexSR::OwnRef` in its `type_outer_ref_rules`.
- **F:** run it; fails. Report "Tests are correctly failing, proceeding."
- **I:** add `OwnRefSR { range, result_rune, inner_rune }` + `IRulexSR::OwnRef` variant + its `range()` /
  `rune_usages()` arms (`rules.rs`); `translate_own_ref_templex` + the two `ITemplexPT::OwnRef` dispatch
  arms (`templex_scout.rs`, mirror weak); the humanizer `"own "` arm; add `IRulexSR::OwnRef(_)` to the
  onion-wrap permitted-list (`postparsing/ast.rs`); the postparse traverse arm
  (`postparsing/test/traverse.rs`).
- **G:** re-run the routing test; passes.
- **A:** full suite green, zero warnings.

### Slice 3 — Remove the `share` keyword (dead field)
No observable change (unused; not a `parse_ref_prefix` arm; no consumers), so no red test.
- **I:** delete the `share` field + both interners (`keywords.rs:17,164,322`).
- **A:** full suite green, zero warnings (no "field never read").

## Flag for the eventual typing re-link (not this bundle)
`IRulexSR::OwnRef` needs a typing consumer when typing re-links — the rune-type solver + the wrap→`KindT`
lowering must map `OwnRefSR` → `HeapOwnRefT` (or a new `OwnT`). Mirrors the `ShareRef` / `HeapOwnRef`
seams flagged in the removal bundle.

## Verification

```bash
cargo test --manifest-path FrontendRust/Cargo.toml --lib --no-fail-fast > tmp/surface-removals.txt 2>&1
grep "test result" tmp/surface-removals.txt
```

- After each slice: green, no failures. Count rises +1 (Slice 1) and +1 (Slice 2) for the new tests;
  Slice 3 deletes none → final **506/0/1**.
- Zero warnings: `cargo build --manifest-path FrontendRust/Cargo.toml --lib` clean.
- Sweep: grep the parser for the `own` arm (present) and the `share` keyword (gone); confirm the `borrow`
  keyword is still absent (already removed).
