# Accuracy report: Concept Functions With Generics (CFWG)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:201 ("# Concept Functions With Generics (CFWG)")

## Verdict
The section itself is a design musing (two alternative unimplemented schemes for concept functions plus generics, "the architectural choice is not yet made") with no checkable claims about existing code — none of `ResolveSR`, `CallSiteFuncSR`, `DefinitionSiteFuncSR`, `Functor1`, etc. exist anywhere in src/. The one concrete, checkable factual claim in the section is the pointer to an implementation handoff doc, and that file does not exist in the repo, making the claim FALSE/stale.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "Our current concept functions don't really work with default generic parameters that well." | UNVERIFIABLE | — | Musing about current behavior, no pointer to code; not independently checkable without a repro. |
| 2 | `investigations/cfwg_handoff.md` is the active handoff doc covering failing tests 1.5/1.6 and should be read for current state | FALSE | `find . -iname "*cfwg*"` from the repo root returns nothing; no `investigations/` directory exists in the repo | Remove the handoff pointer or update it to wherever (if anywhere) this work is actually tracked. |
| 3 | Prototype-Based Concept Functions scheme (`ResolveSR`, `CallSiteFuncSR`, `DefinitionSiteFuncSR`) | UNVERIFIABLE (design proposal, not implemented) | `grep -rn "ResolveSR\|CallSiteFuncSR\|DefinitionSiteFuncSR" src/` → no matches | This is proposed, not existing, machinery — no claim to verify against code. |
| 4 | Placeholder-Based Concept Functions scheme (`ResolveSR`, `DefinitionFuncSR`, `CallsiteFuncSR`) | UNVERIFIABLE (design proposal, not implemented) | same grep, no matches | Same as above. |

## Stale citation sites
None — `grep -rn -w "CFWG"` across src/, Backend/, docs/ (excluding docs/convos) finds only the section header itself; nothing in code cites CFWG.

## Uncited sites that embody the arcana
None found — the schemes described are not implemented in the codebase.

## Suggested rewrite
Not applicable (verdict is minor-inaccuracies, not major/obsolete) — but recommend either deleting the "Implementation handoff" callout at docs/arcana/Generics.md:207 or repointing it to a real, current doc, since `investigations/cfwg_handoff.md` does not exist.
