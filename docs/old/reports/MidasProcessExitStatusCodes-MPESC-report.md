# Accuracy report: Midas Process Exit Status Codes (MPESC)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Compiler/Midas.md:188 ("# Midas Process Exit Status Codes")

## Verdict
The core idea holds: Backend/ (the still-C++, un-ported Midas/codegen layer) uses distinct process exit codes to distinguish failure classes, and codes 1 (general error/panic) and 14 (dereferenced an invalid/dangling reference) are exactly as used in the current code at every live citation site. The doc itself is honestly incomplete (code 2 is left blank, and it says code 116 "was known live but our doublecheck showed its not actually alive" — i.e. the doc already flags its own uncertainty). The only inaccuracy found is external to the doc's content: the citation site list handed to this audit has drifted line numbers (shared.cpp calls have moved by ~70 lines from adjacent code growth), though the "See MPESC" comments themselves are all still present and still correct.

## Claims
| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | Midas uses distinct process return codes for different error conditions, for testing purposes | TRUE | Backend/src/function/expressions/externs.cpp:657, shared.cpp:271/308/333/411/428, common.cpp:1386, wrcweaks.cpp:309 all call `externs->exit` with a specific code | — |
| 2 | Code 0 = success in user-land | UNVERIFIABLE (not directly grepped, but consistent with normal process exit convention) | — | — |
| 3 | Code 1 = general error in user-land, such as a panic call | TRUE | externs.cpp:655-657 (`__vbi_panic` exits with code 1); shared.cpp:271 (assert failure exits 1); flags.cpp:65 and elements.cpp:29/57/83 also exit with 1 | — |
| 4 | Code 2 = (blank/unspecified) | TRUE (doc accurately leaves it blank; no code found using exit code 2) | grep of all `externs->exit` call sites shows only codes 1 and 14 in use | — |
| 5 | Code 14 = dereferenced an invalid reference | TRUE | common.cpp:1384-1386 (`fastPanic`, "Tried dereferencing dangling reference!"), wrcweaks.cpp:307-309 (same message), shared.cpp:409-411 and 426-428 (census-registration checks, "not registered with census, exiting!") all exit with 14 | — |
| 6 | Code 42 = general "good result" code for tests / Code 73 = general "bad result" code for tests | UNVERIFIABLE | not grepped against test harness in this pass; plausible, no contradicting code found | — |
| 7 | Code 116 = "our own internal thing... doublecheck showed its not actually alive" | TRUE (doc's own caveat holds) | no occurrence of exit code 116 found anywhere in Backend/src | — |

## Stale citation sites
- Backend/src/function/expressions/shared/shared.cpp:264 → actual comment is now at shared.cpp:331 (assert-failure exit-code-1 site)
- Backend/src/function/expressions/shared/shared.cpp:342 → actual comment is now at shared.cpp:409 (null-object census check, exit code 14)
- Backend/src/function/expressions/shared/shared.cpp:359 → actual comment is now at shared.cpp:426 (unregistered-object census check, exit code 14)

These are load-bearing: each comment is the only place in Backend/ that tells a reader why the seemingly-arbitrary exit code (1 or 14) was chosen, and the doc is the only place the full code table lives — a reader following the site would need the corrected line number to actually land on the `// See MPESC for status codes` comment.

Backend/src/function/expressions/externs.cpp:655, Backend/src/region/common/common.cpp:1384, Backend/src/region/common/wrcweaks/wrcweaks.cpp:307 — all still accurate, no drift.

## Uncited sites that embody the arcana
- Backend/src/utils/flags.cpp:65 — exits with code 1 but has no "See MPESC" comment.
- Backend/src/function/expressions/shared/elements.cpp:29 — exits with code 1, uncited.
- Backend/src/function/expressions/shared/elements.cpp:57 — exits with code 1, uncited.
- Backend/src/function/expressions/shared/elements.cpp:83 — exits with code 1, uncited.
- Backend/src/function/expressions/shared/shared.cpp:271 — exits with code 1 (assert failure), uncited (only the two 14-code neighbors and one 1-code site at 333 in this file carry the comment; this one at 271 does not).
