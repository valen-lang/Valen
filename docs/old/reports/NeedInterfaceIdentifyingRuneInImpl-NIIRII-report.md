# Accuracy report: Need Interface Identifying Rune In Impl (NIIRII)

Audited against the working tree on 2026-09-06. Source: docs/old/Compiler/Templar/Templar.md:535-543

## Verdict
The bug this note describes — a struct implementing two interfaces generating two `vdrop`s with a name/signature collision — is real and still guarded today by a live regression test (`implementing_two_interfaces_causes_no_vdrop_conflict` and `#[test] fn` right above it that asserts exactly `drop_func_names.len() == 2`), so the underlying concern is accurate and still relevant. But the fix as described no longer matches how the current Rust typing pass disambiguates these functions: there is no "interface rune" plumbed into drop's identifying runes anywhere in the current name types. Instead, each interface's synthesized abstract `drop` is a *separate* `FunctionS`/`FunctionTemplateNameT` with its own `code_location` (the interface's own declaration site), and `FunctionNameT`/`FunctionTemplateNameT` equality/hashing is keyed off that `code_location` plus `template_args`/`parameters` — so the two synthesized drops are distinct purely because they're two distinct declarations, not because a rune was added to carry the interface's identity. This is a historical Scala-Templar note whose stated mechanism has drifted from the current architecture even though its motivating bug is still correctly prevented. Recommendation: leave it in docs/old/ (it's dead history, uncited, and the current fix is structural rather than rune-based) — do not migrate it into docs/arcana; a one-line pointer from the test to the concept would be more useful than promoting this note.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "We had a weird bug where if a struct implemented two interfaces, two vdrops would be generated" | TRUE | src/typing/test/compiler_virtual_tests.rs:143 (`implementing_two_interfaces_causes_no_vdrop_conflict`) and the test above it asserting `drop_func_names.len() == 2` for a struct with `#!DeriveAnonymousSubstruct` — confirms two drop functions is the expected/correct outcome, not a bug by itself | The bug was specifically the two drops *colliding* (same signature), not the existence of two drops, which is intentional (one abstract per interface) |
| 2 | "...but they collided because they had the same signatures" | UNVERIFIABLE (historical) | no Scala Templar code remains to check; current code has no such collision (see claim 3) | This described the old Scala implementation's bug, not something checkable in the current Rust tree |
| 3 | "To solve it, we made vdrop have the interface rune as another identifying rune" | DRIFTED | src/typing/names/names.rs:1343-1392 (`FunctionNameT`/`FunctionTemplateNameT` carry `code_location`, not any interface rune); src/typing/macros/citizen/interface_drop_macro.rs:88-99 (each interface's synthesized `drop` FunctionS gets `code_location: interface_a.name.range`, i.e. the interface's own declaration site) | Current disambiguation is structural: each interface macro-generates its own separate `FunctionS` declaration (distinct `code_location`), so identity falls out of normal per-declaration function naming — there is no explicit "interface rune" field added to drop's rune set to carry over |

## Stale citation sites
None — `sites` is empty and grep for `NIIRII` across src/, Backend/, docs/ found zero references outside docs/old/.

## Uncited sites that embody the arcana
- src/typing/test/compiler_virtual_tests.rs:143 — `implementing_two_interfaces_causes_no_vdrop_conflict`, the regression test guarding exactly this scenario.
- src/typing/macros/citizen/interface_drop_macro.rs:88-99 — where each interface's abstract drop gets its own declaration/code_location, which is the actual current disambiguation mechanism.
- src/typing/names/names.rs:1343-1392 — `FunctionNameT`/`FunctionTemplateNameT`, whose `code_location`-keyed identity is why no explicit "interface rune" is needed anymore.

## Suggested text
(Not applicable — verdict is minor-inaccuracies/historical, not major-inaccuracies or obsolete; per instructions, Suggested text is only required for D3 kind on major-inaccuracies/obsolete verdicts.)
