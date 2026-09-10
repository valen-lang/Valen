# Accuracy report: Elide Checks For Known Live On/Off (ECFKLOO)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Compiler/Midas.md:104 ("# Elide Checks For Known Live On/Off")

## Verdict
The section describes a `--ecfkl` compiler flag that, when on, trusts a "Catalyst" liveness-analysis pass and skips generation-check codegen where it reports `knownLive=true`, and when off double-checks Catalyst's claim and exits with error code 116 on mismatch. No trace of `--ecfkl`, `Catalyst`, `knownLive`, or error code 116 exists anywhere in the current src/ or Backend/ trees — this was a Scala-era (Midas) design idea that either was never carried into the Rust/C++ Backend or was implemented and later removed without a replacement mechanism under these names. There's no live code to cite, and nothing embodies the idea today. Recommendation: leave in docs/old (do not migrate to docs/arcana/) — it documents a dead/unported optimization, not a current mechanism; delete only if the maintainer confirms this optimization was never adopted and isn't planned.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | A `--ecfkl` flag exists/existed to toggle trusting Catalyst's liveness analysis | FALSE (as current) | grep for "ecfkl" across src/ and Backend/ returns nothing | No such flag exists in the current compiler; not found in Backend/src or src/ |
| 2 | A "Catalyst" pass reports `knownLive=true` per generation check | FALSE (as current) | grep for "Catalyst" and "knownLive" across src/ and Backend/ returns nothing | No pass named Catalyst, nor a `knownLive` field, exists in the current codebase |
| 3 | With the flag off, a doublecheck exits with error code 116 on failure | UNVERIFIABLE/FALSE (as current) | grep for `116` and generation-check code in Backend/src turned up no matching error-code convention | Not present in current Backend error-code scheme as far as could be found |

## Stale citation sites
None — no code or doc cites ECFKLOO.

## Uncited sites that embody the arcana
None found. Backend/src/region/common/common.cpp and Backend/src/region/rcimm/rcimm.cpp implement generational-reference liveness checks in the current Backend, but neither has any knownLive/Catalyst/ecfkl-style elision mechanism — they are unrelated to this specific optimization idea, not an uncited implementation of it.

## Suggested rewrite
Not applicable (verdict is obsolete, not major-inaccuracies) — no corrected paragraph is offered; the doc should simply stay in docs/old as a record of an abandoned optimization idea from the Scala Midas backend, or be deleted if confirmed dead.
