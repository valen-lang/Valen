# Accuracy report: Unknown Needs DeeplySatisfied (IEUNDS)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Compiler/Templar/Infer Templar.md:276 ("# Unknown Needs DeeplySatisfied")

## Verdict
This is a Scala-Templar-era design musing about a `deeplySatisfied` boolean threaded through the old rune solver, distinguishing "unknown" from "deeply satisfied" results when constraint refs like `Ref[_, _, _, #K like IMoo:#T]` only partially resolve. It makes no checkable claims about current code — it's worked example plus a half-finished thought ("unless... we put the ors into their own little subfunction. hmm..."). More importantly, the mechanism it describes was explicitly identified and rejected during the Rust port: docs/convos/convo-84-verify-127-expected-test-failures-*.md:6223 states the old matcher's top-level error was literally `"Not deeply satisfied!"` with a `deeplySatisfied` boolean threaded through thirty-plus call sites, and that the new solver's step-local `FailedSolve` design was invented specifically to replace it with named-rune, reason-specific failures. So the `deeplySatisfied` concept isn't ported — it's a named anti-pattern the new design deliberately avoids. Recommendation: leave in docs/old (do not migrate — the concept was superseded, not carried forward).

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | The Templar rune solver distinguishes "unknown" from "deeply satisfied" via a `deeplySatisfied` flag | OBSOLETE | docs/convos/convo-84-verify-127-expected-test-failures-700ac0d1-b9cf-4264-a7ae-b2528e50bafc.md:6223 | The Rust rune/type solver does not use this flag; grep of src/ and Backend/ finds no `deeply_satisf*` symbol anywhere in code |
| 2 | (worked example) `Ref[_, _, _, #K like IMoo:#T]` is valid syntax with unresolved runes producing "unknown" until #T resolves | UNVERIFIABLE / not a code claim | — | This is Scala Templar-era pattern syntax illustrating the concept; not present verbatim in current Vale surface syntax as far as grep shows |

## Stale citation sites
None — no code or doc outside docs/old/ and docs/convos/ (a Session log, not authoritative) cites IEUNDS.

## Uncited sites that embody the arcana
None found. The current rune/type solver deliberately does NOT implement this mechanism (see convo-84 above); no code embodies it because it was replaced with named-rune, step-local failure reporting instead.

## Suggested rewrite
Not applicable — this section is Scala-era design history, superseded by a different design in the ported solver, not something to correct into a live arcana doc.
