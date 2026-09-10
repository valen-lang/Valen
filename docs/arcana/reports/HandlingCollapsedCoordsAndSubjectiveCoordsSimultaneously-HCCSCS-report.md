# Accuracy report: Handling Collapsed Coords and Subjective Coords Simultaneously (HCCSCS)

Audited against the working tree on 2026-09-06. Source: git d097d488^:docs/InstantiatorRegions.md (deleted in commit d097d488), lines 293-311.

## Verdict
This is a Scala-era ("Templar"/"Hammer" pipeline) design note about how the instantiator should eagerly collapse "regions" (the generational-references / immutable-region system) into concrete AST types while still tracking "subjective" per-function region numbers, to avoid backend casts. None of the region machinery it discusses exists in the current Rust port: `RegionTemplataI`/`RegionPlaceholderNameT`, "collapsed region" vs "subjective region" numbering, `Mutabilify` nodes, and region-aware pure-block casting are all absent from `src/`; the only near-hits (`NonKindNonRegionPlaceholderNameT` in `src/typing/names/names.rs`, `src/typing/templata_compiler.rs`) are unrelated placeholder-naming plumbing that merely has "Region" in the identifier to say a placeholder is *not* a region, and the two region-related lines in `src/instantiating/instantiator.rs` (624, 1263) are commented-out Scala-reference code, not live logic. `src/typing/rune_typing/rune_type_solver.rs:625` has a commented-out `RegionTemplataType` stub too. The regions/generational-references feature was never ported; this doc records an abandoned design plan for a feature that doesn't exist in the current compiler. Recommendation: delete (it documents a design for a feature that was never implemented and has no current citations to migrate).

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "If we want to avoid any casting in the backend... we have to do the region collapsing eagerly." | UNVERIFIABLE / moot | n/a | Premise presupposes a regions feature that doesn't exist in the Rust codebase; there is no "region collapsing" concept to verify. |
| 2 | Instantiator translates FunctionCallTE and looks at result types with subjective regions 0-5, collapsed nodes could refer to nonsense collapsed regions -2,-1,0 | FALSE (as current-code claim) | `src/instantiating/instantiator.rs` has no `RegionTemplataI`/subjective-region translation logic; only commented-out remnants at line 624, 1263 | No such translation path exists; this describes Scala Hammer, not `src/instantiating/instantiator.rs`. |
| 3 | Typing phase "often contain None and defer knowledge of the actual region to the instantiator (which combines the typing phase's RegionPlaceholderNameT with the actual function argument RegionTemplataI...)" | FALSE (as current-code claim) | `grep -rn "RegionPlaceholderNameT\|RegionTemplataI" src --include=*.rs` returns zero live hits (only a commented reference) | These types don't exist in `src/typing` or `src/instantiating` today. |
| 4 | The final chosen approach: "make it so the resulting AST only thinks in terms of collapsed regions, and takes things in via constructor... The instantiator will have to produce its own correct temporary information (such as an extra return from translateRefExpr)..." | UNVERIFIABLE / moot | `grep -rn "translateRefExpr\|translate_ref_expr" src` — no matches | Describes a Scala method name (`translateRefExpr`) that has no Rust counterpart (`src/typing/expression/expression_compiler.rs` has no such function); moot since regions aren't implemented. |

Claims 2-4 aren't independently falsifiable against a *feature*, since the feature (regions/generational references) simply isn't in this codebase; graded FALSE/UNVERIFIABLE as current-code claims because the mechanisms and types named don't exist.

## Stale citation sites
None — the only remaining textual occurrence of "HCCSCS" in the tree is an index list entry at docs/architecture/instantiator-design.md:663, which is a bare enumeration ("various spot citations... see grep") with no surrounding logic to be stale against.

## Uncited sites that embody the arcana
None found. The regions/collapsed-coord feature this doc plans for was never implemented in `src/instantiating` or `src/typing`, so there is no current code that embodies this concern.

## Suggested text
No suggested arcana text — the concern describes a feature (subjective/collapsed regions in the instantiator) that was abandoned before or during the Rust port and has no current implementation to document. Nothing to migrate.
