# Accuracy report: Suffix Adds Size Parameter (SASP)

Audited against the working tree on 2026-09-06. Source: docs/old/Compiler/Midas.md:381-392

## Verdict
This is not a description of existing behavior — it's a speculative proposal ("When vale sees an extern name ending in _sasp, it will add extra parameters...") for a never-built feature, and the doc itself hedges the follow-on idea with "Maybe." There are zero code citations of SASP, and grepping for `_sasp`/`sasp` anywhere in src/ or Backend/ finds nothing implementing this suffix convention — the only other hit in the whole repo is docs/todo/ffi-drop-followups.md:368, which explicitly calls it "the retired _vasp/SASP" mechanism, confirming it was abandoned rather than ported to the Rust compiler. Recommendation: delete (not-an-arcana — a design musing for dead/retired functionality with no code to migrate).

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "When vale sees an extern name ending in _sasp, it will add extra parameters to the C function, containing the sizes of everything that was serialized during transit to C." | FALSE / obsolete | grep for `_sasp`/`sasp` across src/, Backend/ returns no matches; docs/todo/ffi-drop-followups.md:368 refers to this as "the retired _vasp/SASP" | Feature does not exist in the current compiler and is described elsewhere in-tree as retired. |
| 2 | "In the future, we could replace this behavior with some sort of configuration file... Maybe." | not-an-arcana (open question) | n/a | Speculative follow-up to claim 1; moot since the base feature was never built. |

## Stale citation sites
None — sites list is empty and no code references SASP.

## Uncited sites that embody the arcana
None found — no extant code implements a `_sasp`-suffix / extra-size-parameter convention for extern C functions.

## Suggested text
Not applicable (D3 kind, verdict is obsolete/not-an-arcana, not major-inaccuracies) — no migration text needed.
