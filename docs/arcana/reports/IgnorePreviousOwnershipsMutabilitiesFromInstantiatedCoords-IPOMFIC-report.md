# Accuracy report: Ignore Previous Ownerships Mutabilities From Instantiated Coords (IPOMFIC)

Audited against the working tree on 2026-09-06. Source: git d097d488^:docs/InstantiatorRegions.md lines 283-290 (file deleted at commit d097d488)

## Verdict

The concept described no longer applies to the current instantiator. IPOMFIC assumes a multi-region model where coords carry a region "height" (e.g. `-1'`) and the instantiator must recompute ownership/mutability when substituting a coord from one perspective region into a container at another. The current instantiating pass has no such model: `RegionT` (src/typing/types/types.rs:16-20) is a two-variant stub (`Iso`, `Default`) explicitly marked `// TODO: Get rid of this when we have an actual default region`, and every call site in `src/instantiating/instantiator.rs` hardcodes `RegionT::Default` (e.g. lines 375, 529, 1160, 1261) rather than deriving a real perspective. Worse, where IPOMFIC's rule would apply — substituting a `KindPlaceholder` into a containing type — `translate_kind` (src/instantiating/instantiator.rs:2173) does the opposite of what the text describes: it takes the substituted kind verbatim (`ITemplataI::Kind(k) => k.kind`, instantiator.rs:2184) with no recomputation of ownership/mutability at all. Ownership today is represented structurally as "onion wrap" kind variants (`OwnRefIT`/`BorrowRefIT`/`ShareRefIT`) recursed into and rebuilt directly (instantiator.rs:2200-2213), not as a per-region mutability value that could go stale. This is not a drifted detail — the entire mechanism IPOMFIC hinges on (region-height-relative ownership recompute) has been replaced. The only surviving trace is a bare, undefined name-drop in an acronym index (docs/architecture/instantiator-design.md:663). Recommendation: delete — the mechanism is gone, nothing cites it as a live rule, and there is no code today this could be migrated to describe.

## Claims

| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "When we have a substitutions array that contains ... `E = -1'*#Thing` (an immutable share reference to something in region height -1), that substitution was calculated from the perspective of the containing function." | OBSOLETE/UNVERIFIABLE | src/typing/types/types.rs:16-20 | Region-height notation (`-1'`) does not exist in the current type system; `RegionT` has no height/relative-perspective concept, just `Iso`/`Default`. |
| 2 | "If we have an array of them, like `-1'[]E`, then when we substitute it, it really needs to be mutable again." | FALSE for current code | src/instantiating/instantiator.rs:2173-2216 | `translate_kind`'s `KindT::StaticSizedArray`/`RuntimeSizedArray` arms (instantiator.rs:2197-2198) just recurse via `translate_static_sized_array`/`translate_runtime_sized_array`; there is no special-case recompute of ownership when a placeholder lands inside an array. |
| 3 | "So, when we're substituting that `E` into that `-1'[]E`, we ignore whatever ownership mutability it had and just recalculate it (in this case, it will be a mutable share, `*MyThing`)." | FALSE for current code | src/instantiating/instantiator.rs:2181-2186 | The `KindPlaceholder` substitution arm does the literal opposite: `let sub = substitutions.get(&p.id)...; match sub { ITemplataI::Kind(k) => k.kind, ... }` — it reuses the substituted kind as-is, with no ignore-and-recompute step. |

## Stale citation sites

docs/architecture/instantiator-design.md:663 — this is a bare name-drop inside a long acronym list ("IPOMFIC, TTTDRM, CSHROOR, ... — various spot citations. Most defined in `docs/`; see grep."). It asserts IPOMFIC is "defined in docs/" but the defining doc (docs/InstantiatorRegions.md) was deleted at d097d488 and never replaced; the index entry is itself stale and should be dropped from that list.

## Uncited sites that embody the arcana

None found. The current instantiator's coord/ownership substitution (`translate_kind`, instantiator.rs:2173) does not implement region-perspective-based recompute at all — there is no live code that embodies this specific concern to cite it from.

## Suggested text

No corrected paragraph is offered: the rule IPOMFIC states is not a lingering-but-mislabeled version of something true today, it describes a mechanism (region-height-relative ownership recompute) that has been removed wholesale in favor of the current single-default-region model. Nothing in `src/instantiating/` needs this warning any more, so there is nothing to migrate into `docs/arcana/`.
