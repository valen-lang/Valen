# Accuracy report: User Must Specify Enough Identifying Runes (UMSEIR)

Audited against the working tree on 2026-09-06. Source: docs/old/Parser_Scout.md:579-697

## Verdict
This is a Scala-era design-decision log (five candidate approaches A–E for resolving ambiguity when a function's declared "identifying runes" don't cover every rune appearing in its parameter patterns), plus three sub-decisions (LDNEIR, PTUBR, NIPFO, DCSIR). It ends with a real decision — "Will go with approach B2" (don't allow anonymous runes in parameters; identifying runes are exactly what the user writes, nothing auto-added) — and that decision is still exactly what the Rust code does: `user_specified_identifying_runes` in src/postparsing/function_scout.rs:108 is taken verbatim from the user's `generic_parameters_p`, with no compiler-side augmentation, and the one explicitly-named exception in the text (LDNEIR — lambdas don't need explicit identifying runes) is implemented and even cross-referenced by name in a code comment at src/postparsing/function_scout.rs:846-847. The claims are mostly design musing (rejected approaches A, C, D, E), which are not checkable claims about current code, but the adopted approach B2 and its LDNEIR exception are checkable and true. Recommendation: migrate the "decision" portion (B2 + LDNEIR) into docs/arcana as a proper Z-suffix doc describing why identifying runes are exactly the user-declared set (no compiler augmentation, no anonymous runes) — the rejected approaches (A, C, D, E) and the sub-musings PTUBR/NIPFO/DCSIR can be dropped since they're either superseded reasoning or already implicitly covered by docs/arcana/Generics.md.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | A function like `fn moo:(#K,#V)(a: Map:(#K,#V,#H))` creates ambiguity if H isn't identifying | UNVERIFIABLE (design musing, not a claim about current code) | — | — |
| 2 | Decision: Approach B2 adopted — "Don't allow any anonymous runes in the parameters," identifying runes = exactly what user declares | TRUE | src/postparsing/function_scout.rs:108-133 (`user_specified_identifying_runes` copied directly from `generic_parameters_p`, no synthesis of extra identifying runes from parameter patterns) | — |
| 3 | Exception (LDNEIR): lambdas don't need explicit identifying runes; inferred from magic params/generics of callee | TRUE | src/postparsing/function_scout.rs:844-852, explicitly cites "Lambdas Dont Need Explicit Identifying Runes (LDNEIR)" in a comment | — |
| 4 | Sub-decision PTUBR: pattern-templex underscores get promoted to real runes, subject to the same B2 rule | UNVERIFIABLE (no PTUBR citation in current code found; underscore/placeholder handling exists in function_scout.rs but not traceably tied to this specific claim without deeper trace) | src/postparsing/function_scout.rs:406,434 (unrelated placeholder handling found, not conclusively the same mechanism) | — |
| 5 | Sub-decision NIPFO: override functions can't supply identifying params | UNVERIFIABLE (no NIPFO citation found in current code) | — | — |
| 6 | Sub-decision DCSIR: destructured fields can't be identifying runes | UNVERIFIABLE (no DCSIR citation found in current code) | — | — |

## Stale citation sites
None (record's `sites` field is empty; this ID has no live code citations).

## Uncited sites that embody the arcana
- src/postparsing/function_scout.rs:108-133 — `user_specified_identifying_runes` built directly from user's declared generic parameters, implementing decided approach B2.
- src/postparsing/function_scout.rs:844-852 — lambda magic-params become identifying runes automatically, implementing the LDNEIR exception named right in this doc section.

## Suggested text
Not required for D3 kind unless verdict is major-inaccuracies or obsolete; verdict here is closer to accurate-but-historical (a decided design question whose outcome is still correctly implemented). No suggested-text section produced, but see the Verdict paragraph for what a migrated arcana entry should focus on (B2 + LDNEIR only).
