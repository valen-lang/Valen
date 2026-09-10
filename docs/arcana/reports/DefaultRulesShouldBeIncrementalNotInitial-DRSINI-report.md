# Accuracy report: Default Rules Should Be Incremental Not Initial (DRSINI)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/DefaultRulesShouldBeIncrementalNotInitial-DRSINI.md:1 ("# Default Rules Should Be Incremental Not Initial (DRSINI)")

## Verdict

The central claim — that generic-parameter default rules (`LiteralSR`) must be added incrementally to the solver rather than baked into the initial rule set — is TRUE and confirmed live in `src/typing/infer_compiler.rs`, `src/typing/function/function_compiler_solving_layer.rs`, and `src/typing/templata_compiler.rs`. But the doc's "Why" section and "Where the rules live" bullet describe a mechanism that no longer exists: it says the postparser *hoists* `EqualsSR(H, _211)` into the parent type's main rules, where it is "always present," separately from the `LiteralSR`. The current postparser (`src/postparsing/post_parser.rs:475-479`) explicitly does the opposite — it **keeps** the `EqualsSR` bundled inside `GenericParameterDefaultS.rules` alongside the `LiteralSR`, specifically so the default travels as a self-contained unit (its own comment cites DRSINI while stating the reasoning has changed: the rune is registered via `solverState.registerRunes` at default-fire time, not present in the main rules from the start). This is a genuine architectural change since the doc was written, so the doc's explanation of *why* the eager-default bug can occur is now false, even though its top-level rule is still correctly enforced by the code.

## Claims

| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Default rules (`LiteralSR`) must not be in the solver's initial rule set; added incrementally only for unsolved runes, via `solveForResolving`/`solve_for_resolving` and `evaluateGenericFunctionFromCallForPrototype`/`evaluate_generic_function_from_call_for_prototype` | TRUE | src/typing/infer_compiler.rs:173-210 (solve_for_resolving incremental callback); src/typing/function/function_compiler_solving_layer.rs:505,603-620 | — |
| 2 | Per ECSIIOSZ, defaults are added to the call-site's individual solver instance, not baked into the shared rule vector | TRUE | src/typing/templata_compiler.rs:198-205 (`assemble_call_site_rules` filters, no longer adds defaults) | — |
| 3 | The postparser hoists `EqualsSR(H, _211)` into the parent type's main rules, connecting param rune to default's result rune; this alone is harmless | FALSE | src/postparsing/post_parser.rs:460-490 — `EqualsSR` is explicitly **kept** in `rules_to_leave_in_default_argument` (i.e., in `GenericParameterDefaultS.rules`), NOT hoisted into `rule_builder` (main rules). Only `KindList`, `CallSiteFunc`, and `DefinitionFunc` rules get hoisted. | The `EqualsSR(H, _211)` now lives inside the default's own rule bundle, not in main rules, and is not "always present" — it fires only when the default fires. |
| 4 | `EqualsSR(H, _211)` "Flows into the solver via `assembleCallSiteRules`'s rule filter. Always present." | FALSE | Same as #3; also src/typing/templata_compiler.rs:204-206 `assemble_call_site_rules` only filters `rules` (the main/hoisted rules) — the `EqualsSR` isn't among them for defaulted params. | Should read: the `EqualsSR` is part of `GenericParameterDefaultS.rules` and is only injected via the incremental callback (`commit_step`) alongside the `LiteralSR`, not present from the start. |
| 5 | `assembleCallSiteRules` "used to eagerly add `x.rules` for defaulted params; this was removed" | TRUE (historical + current) | src/typing/templata_compiler.rs:200-205 comment: "default rules are no longer added eagerly here" | — |
| 6 | `solveForResolving`/`solve_for_resolving` uses incremental callback pattern, serves `resolveStruct`/`resolve_struct` and `resolveInterface`/`resolve_interface` | TRUE | src/typing/infer_compiler.rs:1146-1166 (`resolve_struct`, `resolve_interface` call sites, both commented "Per @DRSINI") | — |
| 7 | `evaluateGenericFunctionFromCallForPrototype`/`evaluate_generic_function_from_call_for_prototype` has its own incremental callback adding `defaultRules.rules` for unsolved runes | TRUE | src/typing/function/function_compiler_solving_layer.rs:603-620 | — |
| 8 | `assemblePredictRules`/`assemble_predict_rules` unaffected; adds both `x.rules` and a dynamically-created connecting EqualsSR, used for type prediction | DRIFTED | src/typing/templata_compiler.rs:178-196 — it does add `x.rules` (which now includes the bundled EqualsSR itself, since EqualsSR lives inside `default.rules` per claim #3) but does NOT additionally construct a fresh dynamically-created connecting EqualsSR — that EqualsSR is already part of `x.rules`. | The "dynamically-created connecting EqualsSR" is no longer separate; it's the same `EqualsSR` now bundled into `default.rules`, so `assemble_predict_rules` just pushes `x.rules` as-is. |
| 9 | `solveForDefining`/`solve_for_defining` never includes defaults; function bodies use placeholders | TRUE (by omission) | src/typing/infer_compiler.rs — `solve_for_defining` (around line 140-165) takes `rules` as given and never references `GenericParameterDefaultS`/default rules. | — |

## Stale citation sites

- src/postparsing/post_parser.rs:476 — the code comment there correctly documents the *current* behavior ("We KEEP it in the default's rules... rather than hoisting"), but the arcana doc itself (the thing this comment points readers to) describes the opposite, now-obsolete hoisting behavior. The doc, not this site, is what's stale.
- docs/arcana/DefaultRulesShouldBeIncrementalNotInitial-DRSINI.md's own "Why" and "Where the rules live" sections are the stale material; other citing sites (infer_compiler.rs:170,1151,1166; templata_compiler.rs:178,201; function_compiler_solving_layer.rs:603; abstract_body_macro.rs:55) are all consistent with the doc's core "incremental not initial" claim and are not stale.

## Uncited sites that embody the arcana

None found beyond the already-cited sites; the incremental-default machinery is concentrated in the cited functions.

## Suggested rewrite

Replace the "Why" and "Where the rules live" sections with:

> ## Why
>
> The postparser builds an `EqualsSR(H, _211)` connecting the generic param rune (H) to the
> default's result rune (_211), but — unlike in the Scala-era version of this doc — it does
> **not** hoist this rule into the parent type's main rules. Instead it stays bundled inside
> `GenericParameterDefaultS.rules` alongside `LiteralSR(_211, 5)`, so the default travels as a
> self-contained unit (e.g. when `GenericParameterS` is inherited by struct internal methods).
> Both rules are injected together, only when the incremental callback fires for an unsolved
> identifying rune. This avoids the original bug case entirely by construction: `_211=5` and
> `H=_211` cannot fire before argument inference runs, because neither rule is in the solver
> until the callback commits them together.
>
> ## Where the rules live
>
> - `EqualsSR(H, _211)` and `LiteralSR(_211, 5)` — both stored in `GenericParameterDefaultS.rules`
>   (`src/postparsing/post_parser.rs:475-483`). Added incrementally, as a pair, by
>   `solve_for_resolving` (`src/typing/infer_compiler.rs`) and by the callback in
>   `evaluate_generic_function_from_call_for_prototype`
>   (`src/typing/function/function_compiler_solving_layer.rs`), only when the param rune is
>   unsolved after argument inference. The typing pass registers the default-only runes via
>   `solverState.registerRunes` (or its Rust equivalent) at commit time.
