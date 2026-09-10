# Accuracy report: Modules Must Export Dependencies Themselves (MMEDT)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Compiler/Midas.md:287 ("# Modules Must Export Dependencies Themselves")

## Verdict
The section documents a specific Midas-era bug — when two Vale modules each export the same underlying type (e.g. `Vec3`) under different C names, Midas's `.h` generator "arbitrarily picks the first export name found" (GitHub issue #182) — and proposes a workaround/solution (module export isolation, transitive-export requirement, one-export-per-type-per-module). Midas itself no longer exists as a component (`grep -iname "*midas*"` finds only a vestigial `src/bin/valec/midas.rs` filename); the current C++ backend (`Backend/src/vale.cpp`) generates exported headers keyed per `packageCoord` (`packageCoordToHeaderNameToC`, Backend/src/vale.cpp:358-411), which structurally routes each package's exports into its own header rather than doing the kind of global "look up the export name for this type" resolution the bug depended on. The commented-out `exportedName`/`fullNameToExportName` machinery still visible in Backend/src/metal/ast.h:268-345 is dead code, not evidence the described lookup path is live. No code or doc cites MMEDT today (grep across src/, Backend/, docs/ finds zero hits) and the one listed site, docs/old/Vale Steps.md:550, is a struck-through TODO ("~~add test for MMEDT~~"), not a live reference. The plan's proposed solutions were never implemented as such in the new architecture — the new per-package header keying sidesteps the problem a different way. Recommendation: leave in docs/old (do not migrate to arcana; nothing here is checkable against or actionable in current code).

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Midas generates `.h` files from module `export` statements, resolving a type to "the export name" | OBSOLETE | Midas doesn't exist (`find -iname "*midas*"` → only src/bin/valec/midas.rs, a filename); current generator is Backend/src/vale.cpp:355-411 | Rewrite to describe the current Backend/src/vale.cpp export pipeline if migrated |
| 2 | Midas "arbitrarily picks the first export name found" when two modules export the same type, per issue #182 | UNVERIFIABLE (historic, Midas gone) | no Midas source remains in-tree | N/A — describes deleted code |
| 3 | Solution: require one module cannot see another's exports, exports keyed/generated per-module | DRIFTED (superseded by a different mechanism) | Backend/src/vale.cpp:358 `packageCoordToHeaderNameToC` keys headers per packageCoord, not by a cross-module export-name lookup | The actual current design isn't "module can't see another's exports" as a rule enforced on lookup; it's structural — headers are built per package coordinate, so the ambiguous global lookup this section worries about doesn't occur in the same form |
| 4 | Solution: an export's dependencies must also be exported in that module | UNVERIFIABLE | no equivalent validation found in Backend/src or src/typing for this specific rule | Not confirmed either implemented or needed under the new per-package keying |

## Stale citation sites
docs/old/Vale Steps.md:550 — "~~add test for MMEDT~~" is a struck-through TODO list item, already marked done/abandoned, not an active reference to this design.

## Uncited sites that embody the arcana
None found — the current export-header generation (Backend/src/vale.cpp:355-411, keyed by `packageCoord`) solves a related problem differently rather than implementing this section's proposed rules, so it isn't fairly described as "embodying" this arcana.

