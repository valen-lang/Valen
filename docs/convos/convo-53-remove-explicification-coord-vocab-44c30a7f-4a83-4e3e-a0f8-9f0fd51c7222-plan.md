# Plan document

Source: `/Users/verdagon/.claude/plans/ethereal-watching-matsumoto.md`
Session: 44c30a7f-4a83-4e3e-a0f8-9f0fd51c7222

---

# Remove explicification (the Kind→Coord coercion vocabulary)

## Context

Under the pre-onion model a type rune could be a bare `Kind` (`Ship`) or a `Coord` (ownership+kind, `&Ship`). "Explicification" made an implicit Kind→Coord lift explicit by inserting a `CoerceToCoordSR` rule after a `Lookup`/`Call` that produced a kind. The `explicify_lookups` **pass** did this automatically; the drop/constructor **macros** do it by hand.

Onion dissolves the Kind/Coord axis — a reference is a wrapped `Kind`, an owned value is a bare `Kind` — so explicification has no job. The pass is already gone (function deleted with higher_typing; its 6 dead calls removed earlier this session). What remains is the coord vocabulary the macros still emit.

State of that vocabulary (verified):
- **`CoerceToCoordSR`** — the rule variant is already deleted from the enum, so every construction is a live `E0422`.
- **The coord runes** `SelfCoordRune` / `MacroSelfCoordRune` / `MacroVoidCoordRune` — **already deleted** from `names.rs`; the drop-macro constructions of them are *already* `E0422`s. So there is **no rune teardown** for these — the rewrite just stops constructing them and repoints to the surviving **kind** runes (`SelfKindRune`, `MacroSelfKindRune`, `MacroVoidKindRune`, all still defined).
- **`ImplicitCoercionKindRune`** — still fully defined and constructed once (struct constructor). Needs a real teardown.

**Goal:** rewrite the macros' rule-building to its final onion shape — every `CoerceToCoord` site becomes a **bare kind** — and tear down `ImplicitCoercionKindRune`. Per architect decision the constructor return and drop self-param are **bare (inline owned value)** even for share citizens, so there is **no `ShareRef` branch and no sharedness reading** in the rules. Nothing is expected to *solve* afterward (see Out of scope); the suite stays red. This removes the explicification concept and clears its compile errors.

## Files touched & top-level items

**Macros — rule-building rewrites (bare kind, delete `CoerceToCoord`):**

1. `src/typing/macros/struct_constructor_macro.rs`
   - `use crate::postparsing::names::{…}` (line 16) — drop `ImplicitCoercionKindRuneValS`
   - fn `get_struct_sibling_entries_struct_constructor` — point the `Call.result_rune` at `ret_rune`; delete the `struct_kind_rune` intermediate (the `ImplicitCoercionKindRune`) and the `CoerceToCoord`
2. `src/typing/macros/citizen/struct_drop_macro.rs`
   - fn `get_struct_sibling_entries_struct_drop` — delete the `void_coord_rune_s` / `self_coord_rune_s` interns and both `CoerceToCoord`; repoint the return usage to `void_kind_rune_s` (`MacroVoidKindRune`) and the `this` param to `self_kind_rune_s` (`SelfKindRune`)
   - fn `make_implicit_drop_function_struct_drop` — delete the `drop_v_rune` / `drop_p1_rune` interns and both `CoerceToCoord`; repoint the return to `drop_vk_rune` and the `x` param to `drop_p1k_rune`
3. `src/typing/macros/citizen/interface_drop_macro.rs`
   - fn `get_interface_sibling_entries_interface_drop` — delete the `void_coord_rune_s` / `self_coord_rune_s` interns and both `CoerceToCoord`; repoint return to `MacroVoidKindRune`, self param to `MacroSelfKindRune`
4. `src/typing/macros/anonymous_interface_macro.rs`
   - fn `map_runes_anonymous_interface` — delete the commented-out `CoerceToCoord` block (~284–288)

**`ImplicitCoercionKindRune` teardown (exhaustive matches — arms MUST be removed or they won't compile):**

5. `src/postparsing/names.rs`
   - enum `IRuneS` — remove the `ImplicitCoercionKindRune` variant (~736)
   - impl `IRuneS` fn `canonical_ptr` — remove its arm (~799)
   - enum `IRuneValS` — remove the `ImplicitCoercionKindRune` variant (~942)
   - the `RuneValQuery` equality impl — remove its arm (~1023; has a `_ => false` fallback, but the dead arm still goes)
   - struct `ImplicitCoercionKindRuneValS` — delete (~853)
   - struct `ImplicitCoercionKindRuneS` — delete (~1147)
6. `src/scout_arena.rs`
   - `use …names::{…}` (line 10) — drop `ImplicitCoercionKindRuneS`
   - fn `alloc_rune_canonical` — remove the `ImplicitCoercionKindRune` arm (~396–401; no wildcard — required)
7. `src/postparsing/post_parser_error_humanizer.rs`
   - fn `humanize_rune` — remove the `ImplicitCoercionKindRune` arm (~191; no wildcard — required)

**Cosmetic rename `rules_with_implicitly_coercing_lookups_s` → `rules_s`** (leftover from the explicify-call removal; nothing coerces now):

8. `src/typing/array_compiler.rs` — param in fns `evaluate_static_sized_array_from_callable`, `evaluate_runtime_sized_array_from_callable`, `evaluate_static_sized_array_from_values`
9. `src/typing/expression/pattern_compiler.rs` — param in fn `infer_and_translate_pattern`
10. `src/typing/expression/expression_compiler.rs` — local in fn `astronomize_lambda`

## Rewrite recipe (rule sites)

Uniform: **delete the `CoerceToCoord` rule and the (already-deleted) coord-rune intern that fed it; repoint the consumer** — the param/return `RuneUsage` that named the coord rune — **at the kind rune** the `Lookup`/`Call` already produces. No sharedness branch, no wrap rule. For the constructor, the `Call` writes straight into `ret_rune` (`ReturnRune`, kept), which the `Some(ret_rune)` return already references.

## Keep

`IRuneTypingLookupFailedError` (`rune_type_solver.rs` + consumers) — the rune-type solver's lookup-failure error (`CouldntFindType`/`TooManyMatchingTypes`), the validation that moved *into* the solver, not explicify residue.

## Out of scope (noted, not done here — files stay partly red, which is fine mid-slice)

- **Solver arms** — the rewritten rules won't solve yet: `Call` panics in the rune-type solver (`rune_type_solver.rs`), and the onion wrap rules panic in the value solver (`compiler_solver.rs`). That's the rune-type-solver rewrite / value-solver-shrink work, needed for all `Call`/wrap rules regardless of explicification.
- **The macro body-generators** (`generate_function_body_struct_constructor`, `generate_function_body_struct_drop`) — separate residue: `OwnershipT`, `KindT::new(ownership,…)`, `.kind` peels, deleted `IStructMemberT`/`IMemberTypeT`, and real share-drop semantics. The OwnershipT-dissolution cascade.
- **`CoordComponentsSR`** in `anonymous_interface_macro.rs` (~741, ~768; references an undefined `self_ownership_rune`) — a different retired coord rule.

## Verification

Build is red and stays red (typing slice); success = the explicification errors clear and no *new* errors appear in touched files.

```bash
cargo check --manifest-path FrontendRust/Cargo.toml --lib > tmp/onion-arc.txt 2>&1
```

- `grep -rn "CoerceToCoord\|ImplicitCoercionKindRune\|SelfCoordRune\|MacroVoidCoordRune\|MacroSelfCoordRune" FrontendRust/src/` → **zero** hits (comments included).
- The `E0422: cannot find … CoerceToCoordSR` / `… MacroVoidCoordRuneS` / `… SelfCoordRuneS` / `… MacroSelfCoordRuneS` class is gone; error total drops accordingly (was 546).
- **No new** primary errors anchor in the touched files. Critically, `names.rs`, `scout_arena.rs`, and `post_parser_error_humanizer.rs` are in the compiling (non-red) set — they must stay at **0 primary errors** (i.e. every exhaustive-match arm was removed and no dangling reference remains). Watch for a non-exhaustive-match error there as the signal an arm was missed.
- Tests can't run while typing is red — no test step. (The postparse `UserFunction` tests added earlier are unaffected.)
