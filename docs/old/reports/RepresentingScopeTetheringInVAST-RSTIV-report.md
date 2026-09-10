# Accuracy report: Representing Scope Tethering In VAST (RSTIV)

Audited against the working tree on 2026-09-06. Source: docs/old/Compiler/Midas.md:218-263

## Verdict
This is a design musing, not a settled description of code — it poses open questions ("should we also have BORROW and TETHER?", "Should we even have a BORROW reference?") and ends with a tentative plan ("So yeah, we'll make it a property of Stackify... Stackify will also have a keepAlive boolean"). The plan it settles on was never implemented as written: current `Backend/src/metal/instructions.h`'s `Stackify` class has no `keepAlive` field, and the entire "tether"/"knownLive"/HGM-generational-references vocabulary is absent from the Rust typing pass (`src/`) — the only surviving trace is the `__tether` naming convention in one legacy C++-backend test (`Backend/test/tethercrash.vale:13,15`). It has zero live citations. Recommendation: delete (or leave untouched in docs/old/ as historical record) — do not migrate; it documents an abandoned Midas/VAST-era plan with no current code to anchor it.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "In HGM, constraint ref parameters will be borrow references, and a lot of constraint ref locals will be tethers." | UNVERIFIABLE / obsolete | n/a | HGM (generational-reference memory model) planning; no such distinction exists in current code. |
| 2 | Tether is a property of Local; "we'll make it a property of Stackify" | FALSE (as implemented) | Backend/src/metal/instructions.h:93-105 | `Stackify` has only `variable`, `expr`, `result` — no tether/keepAlive field. |
| 3 | "Stackify will also have a keepAlive boolean, which only applies to constraint refs." | FALSE | Backend/src/metal/instructions.h:93-105 | No `keepAlive` field anywhere in `Stackify`, `instructions.h`, or `expression.cpp` (grep for `keepAlive`/`knownLive` returns nothing). |
| 4 | "To test, we'll have valestrom force keepAlive=true for locals ending in `__tether`." | DRIFTED | Backend/test/tethercrash.vale:13,15 | The `__tether` local-naming convention survives in one legacy test, but there is no `valestrom` (pre-Rust frontend) in the tree and no `keepAlive` field for it to set. |
| 5 | "BORROW exists, informally inside of Catalyst." | UNVERIFIABLE | n/a | "Catalyst" is not present anywhere in current src/ or Backend/; unfindable/renamed-away concept. |

## Stale citation sites
None — `sites: []` in the record; no code currently cites RSTIV.

## Uncited sites that embody the arcana
None found. The current typing pass (`src/typing/borrow_checker/`) implements a completely different borrow/region model (aliasing groups, noalias) unrelated to this doc's tether/keepAlive proposal, so there is no code to attach this arcana to.

## Suggested text
Not applicable — verdict is obsolete/not-an-arcana; no corrected paragraph is warranted since no current code implements this design.
