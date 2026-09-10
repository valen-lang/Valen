# Accuracy report: Parameter Full-Type / Value-Type Split (PFVSZ)

Audited against the working tree on 2026-09-06. Arcana doc: src/postparsing/docs/arcana/ParameterFullTypeValueTypeSplit-PFVSZ.md

## Verdict
The core idea (a `ParameterS` splits its type into `full_type_rune`/`value_type_rune` plus `type_outer_ref_rules`/`value_type_rules`, with a debug_assert-enforced invariant when there are no wraps) is intact and matches src/postparsing/ast.rs:378-450. But two claims are wrong: the doc names the producing function `translate_signature_templex`, which does not exist anywhere in the codebase as a `fn` — the real function is `translate_signature_type_st` (src/postparsing/rules/templex_scout.rs:702); and the invariants section claims `type_outer_ref_rules` "may hold only the four wrap rules" when the code (and the doc's own earlier sentence) lists exactly three (BorrowRef/WeakRef/OwnRef). This is minor-inaccuracies: the mechanism, field names, and invariant enforcement are all otherwise accurate, but a reader following the doc to find the split's entry point will look for a function that isn't there, and the "four wrap rules" line is simply false.

## Claims
| # | Claim (quoted or closely paraphrased from the doc) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | A parameter's type is stored on `ParameterS` in two halves: outer wraps and enclosed value. | TRUE | src/postparsing/ast.rs:378-403 | |
| 2 | For `func foo(x &Ship)`, value type is `Ship`, full type is `&Ship`; `&&Ship` has value type `Ship`, full type two wraps; unwrapped `Ship` has full == value type. | TRUE | src/postparsing/ast.rs:392-393, 435-437 (invariant enforces the no-wrap case) | |
| 3 | `ParameterS` holds `full_type_rune`, `value_type_rune`, `value_type_rules`, `type_outer_ref_rules` with the stated meanings. | TRUE | src/postparsing/ast.rs:394-403 | |
| 4 | `translate_signature_templex` produces the split, peeling outer wraps into `type_outer_ref_rules` and putting the value type (with nested wraps in template args) into `value_type_rules`. | DRIFTED | No `fn translate_signature_templex` exists anywhere in src/. The real function is `translate_signature_type_st` (src/postparsing/rules/templex_scout.rs:702-709), which does exactly the described job. The old name survives only in stale comments: src/postparsing/rules/templex_scout.rs:205, src/postparsing/rules/templex_scout.rs:696, src/postparsing/function_scout.rs:469. | Replace `translate_signature_templex` with `translate_signature_type_st` throughout the doc. |
| 5 | Why store two halves: the typing pass ignores outer references when resolving method calls, e.g. `my_ship_ref.launch()` looks in `Ship`'s namespace. | UNVERIFIABLE (not traced end-to-end in typing pass method resolution, but consistent with `value_type_rune`'s doc comment "the named-type root, past the outer wraps" at ast.rs:396) | src/postparsing/ast.rs:396 | |
| 6 | Invariants: `type_outer_ref_rules` may hold only the four wrap rules. | FALSE | src/postparsing/ast.rs:429-433 `debug_assert!(... matches!(r, IRulexSR::BorrowRef(_) \| IRulexSR::WeakRef(_) \| IRulexSR::OwnRef(_)))` — exactly three variants, not four. Contradicts the doc's own line 11, which correctly names three (`BorrowRef` / `WeakRef` / `OwnRef`). | Change "four wrap rules" to "three wrap rules" (or "the three onion-ref wraps"). |
| 7 | When `type_outer_ref_rules` is empty, `full_type_rune` and `value_type_rune` are the same rune. | TRUE | src/postparsing/ast.rs:435-438 | |
| 8 | `ParameterS::new` checks both with `debug_assert!` rather than making them unrepresentable, so illegal states fail loudly at construction. | TRUE | src/postparsing/ast.rs:406-438 (two debug_assert!s, private `_sealed: ()` field forcing all construction through `new`) | |

## Stale citation sites
- src/postparsing/rules/templex_scout.rs:205 — comment says "call `translate_signature_templex` instead" but that function doesn't exist; should say `translate_signature_type_st`.
- src/postparsing/rules/templex_scout.rs:696 — comment says "The ITypeST twin of `translate_signature_templex`" — same stale name; this is itself the definition site of the real function (`translate_signature_type_st`, line 702), so the comment is self-referential and wrong.
- src/postparsing/function_scout.rs:469 — comment says "`translate_signature_templex` fills both buckets," but the code two lines below (function_scout.rs:485) calls `translate_signature_type_st`.

All other listed citation sites (src/postparsing/ast.rs:231, ast.rs:391; src/postparsing/post_parser.rs:1070; src/postparsing/function_scout.rs:468; src/postparsing/names.rs:1432; src/postparsing/rules/templex_scout.rs:693,1014,1061; src/typing/rust_interop/declarations.rs:107,139,142,149,220,411,651; src/typing/rust_interop/oracle.rs:75; src/typing/function/function_compiler_solving_layer.rs:272,522,915; src/typing/function/function_body_compiler.rs:316; src/typing/macros/anonymous_interface_macro.rs:732; src/typing/macros/struct_constructor_macro.rs:34; src/typing/test/compiler_tests.rs:2935) still describe code that matches the doc's core claims (full/value rune split, outer-ref-rules chain) and were not individually flagged as stale.

## Uncited sites that embody the arcana
- src/postparsing/rules/templex_scout.rs:702 (`translate_signature_type_st` definition) — carries a nearby comment citing @PFVSZ at line 693, so effectively covered; listed for completeness since the doc names the wrong function.
- src/postparsing/post_parser.rs:447, 726, 743, 898, 1099 — five more call sites of `translate_signature_type_st` performing the same split (for generic params, return types, variadic members) with no @PFVSZ comment nearby.
- src/postparsing/rules/templex_scout.rs:1026, 1052 — two more `translate_signature_type_st` call sites (bound-function synthesis, return type) without an @PFVSZ citation.

## Suggested rewrite
Line 13, replace:

> `translate_signature_templex` produces the split. It peels the outermost run of wraps into `type_outer_ref_rules`, and puts the value type, plus anything nested inside it (including wraps buried in template args), into `value_type_rules`.

with:

> `translate_signature_type_st` produces the split. It peels the outermost run of wraps into `type_outer_ref_rules`, and puts the value type, plus anything nested inside it (including wraps buried in template args), into `value_type_rules`.

Line 17, replace "may hold only the four wrap rules" with "may hold only the three wrap rules" (or "the three onion-ref wraps — `BorrowRef` / `WeakRef` / `OwnRef`").
