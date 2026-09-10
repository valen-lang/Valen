# Accuracy report: Lambdas Have Readwrite Self Parameters (LHRSP)

Audited against the working tree on 2026-09-06. Source: docs/old/Compiler/Templar/Environments, Templatas, and Parent Entries.md:347-360

## Verdict
This section is a forward-looking design musing, not a claim about present-day code: it describes the state at the time ("lambdas' __call take &!self, not &self"), floats a future inference scheme ("we could automatically infer... by looking at how it's used"), and notes "later we'll remove permissions anyway." That prediction has since come true — the read/write permission system (`&self`/`&!self`) has been fully removed from the compiler; there is no `Permission` type, no readwrite-vs-readonly self-parameter distinction anywhere in `src/`. The section has zero live citation sites (`grep -w LHRSP` across src/, Backend/, docs/ outside docs/old/ returns nothing), so nothing points to it and nothing needs updating in code comments. Recommendation: delete — the concern it describes (permissions on lambda self-params) no longer exists in the language, and it was never migrated into an active arcana.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "Right now, lambdas' __call take &!self, not &self" (historical, Scala-era) | UNVERIFIABLE (not a claim about current code — describes the old permissions system at the time of writing) | — | N/A, historical |
| 2 | Future idea: infer readonly vs readwrite self by analyzing lambda body | UNVERIFIABLE / moot | grep for `Permission`, `ReadWrite` types in src/typing returns nothing | The whole permissions mechanism this idea would apply to has been removed |
| 3 | "later we'll remove permissions anyway" | TRUE (borne out) | `grep -rln "enum Permission" src` → no results; no readonly/readwrite self-param concept exists in src/typing | Permissions were indeed removed; the prediction is now history, not a live design question |

## Stale citation sites
None — `grep -rn -w "LHRSP"` across src/, Backend/, docs/ (excluding docs/old/) finds no citations.

## Uncited sites that embody the arcana
None found — the concern (permission-typed self parameters on lambda `__call`) has no counterpart in the current codebase to cite from.

## Suggested text
N/A (kind D3, verdict obsolete — not major-inaccuracies, but obsolete per the alternate condition since the described mechanism and even the question itself no longer apply).
