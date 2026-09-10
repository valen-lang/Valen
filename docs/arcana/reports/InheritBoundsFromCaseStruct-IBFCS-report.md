# Accuracy report: Inherit Bounds From Case Struct (IBFCS)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:1253 ("### Inherit Bounds From Case Struct (IBFCS)")

## Verdict
The core mechanism the section describes is real and currently implemented: while resolving an override, the compiler solves for the sub-citizen ("case struct") via `partial_resolve_impl`, then calls `resolve_citizen_bounds` on that sub-citizen and imports its reachable bounds as function bounds into the dispatcher's environment before doing the final `resolve_impl`. This exactly matches the section's claim ("we do a solve to get the case struct... we also grab the reachable bounds from that struct"). No claim in the section is FALSE. The only problems are (1) IBFCS has zero citations anywhere in `src/`, even though `edge_compiler.rs` is precisely the code that embodies it and other nearby arcana IDs (FOSFC, NBIFPR) are cross-referenced from that same doc region; and (2) an uncommitted self-note sits directly above the section header ("NOTE TO SELF: we're not bringing in any impl bounds! this might be where we used to do that") which is stale/misleading now that the bound-importing code (edge_compiler.rs:555-590) demonstrably exists — a future reader could mistake this note as saying the feature isn't implemented.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "In FOSFC, we do a solve to get the case struct." | TRUE | src/typing/edge_compiler.rs:529-551 (`partial_resolve_impl` call, comment "Step 4: Figure Out Struct For Case, see FOSFC." at edge_compiler.rs:650) | — |
| 2 | "When doing that, we also grab the reachable bounds from that struct." | TRUE | src/typing/edge_compiler.rs:555-590 — filters `partial_resolve_conclusions` for the sub-citizen, calls `self.resolve_citizen_bounds(...)`, and imports each bound via `Compiler::import_function_bound` into the dispatcher's env | — |
| 3 | Conceptual example: `launch<T>(self &Ship<T>)` requires `Lam: drop`, which is not guaranteed by `ILaunchable`, but is guaranteed because an *existing* `Ship<T>` value already satisfies it | UNVERIFIABLE (illustrative, not literal code) | — | Plausible and consistent with current Vale generics syntax (`where func drop(Lam)void`, `#!DeriveInterfaceDrop`/`#!DeriveStructDrop` still used elsewhere in the doc) |
| 4 | "We do this for NBIFPR for parameters and returns and one day for cases inside matches." | TRUE (as a forward-looking statement) | docs/arcana/Generics.md:491-654 (NBIFPR section exists and describes the parameter/return case); "cases inside matches" still phrased as future work — no evidence found it's implemented | — |

## Stale citation sites
None — IBFCS has no citation sites in code (`sites: []` in the record), so there is nothing to check for staleness.

## Uncited sites that embody the arcana
- src/typing/edge_compiler.rs:529-551 — `partial_resolve_impl` call that solves for the dispatcher case's sub-citizen (the "case struct" solve from FOSFC, feeding into IBFCS).
- src/typing/edge_compiler.rs:555-590 — the actual bound-gathering/import loop: `resolve_citizen_bounds` + `Compiler::import_function_bound`, which is the concrete implementation of "grab the reachable bounds from that struct." This is the single best candidate for an `@IBFCS` (or `see IBFCS`) citation.
- src/typing/edge_compiler.rs:592-616 — construction of `dispatcher_inner_env_with_bounds_for_sub_citizen`, the environment that carries the inherited bounds into the subsequent `resolve_impl` call.

## Suggested rewrite
Not required (verdict is minor-inaccuracies, not major/obsolete), but recommend two small doc fixes rather than a rewrite:
1. Add a citation `@IBFCS` / `see IBFCS` at src/typing/edge_compiler.rs:555 (start of the bound-gathering loop), since this is the arcana's home in code and currently has none.
2. Either remove the "NOTE TO SELF: we're not bringing in any impl bounds! this might be where we used to do that" line above the IBFCS header, or update it to note that this gap has since been closed by the code at edge_compiler.rs:555-590 — as written it reads as still-open when the following section (and the code) show it is resolved.
