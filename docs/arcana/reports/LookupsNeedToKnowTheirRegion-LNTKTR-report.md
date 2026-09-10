# Accuracy report: Lookups Need To Know Their Region (LNTKTR)

Audited against the working tree on 2026-09-06. Source: git 24695a17^:docs/regions/Regions.md lines 596-618 (file deleted wholesale in the docs reorg commit 24695a17)

## Verdict
LNTKTR argued that `CallSR`/`LookupSR` (weakly-typed post-parsing rule AST nodes) must carry their target region explicitly rather than have the higher-typing/typing pass infer it from an ambient/default region — this was a design decision for the old generational-references "regions" scheme. That scheme, and the entire `docs/regions/Regions.md` file it lived in, was deleted in commit 24695a17 as part of a docs reorg, and the compiler went a different direction: the region concept that survived (`RegionSR`: Unspecified/Held/Rune, in src/postparsing/rules/types.rs:139-145) is attached only to `BorrowRefSR` (a borrow reference), not to `CallSR` or `LookupSR`, which today (src/postparsing/rules/rules.rs:112-124) carry no region field at all. The concern LNTKTR raised never got implemented the way it proposed, and the surrounding design it was justifying (ambient vs. explicit region on generic-parameter substitution) isn't how the current group-borrowing/aliasing-groups system (src/typing/borrow_checker/{aliasing_info,groupify}.rs) works. Recommendation: delete — the sole citing comment at docs/HigherTypingPass.md:168 references a superseded design and should be removed along with it, not migrated.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | `foo` doesn't know at post-parsing time whether `T` will be coerced to a coord (Ref); that's resolved later in higher-typing/typing pass | UNVERIFIABLE (design musing about the old regions scheme, not a checkable code claim) | — | — |
| 2 | The chosen approach was: include region info explicitly in `CallSR` (and `LookupSR`) rather than have the pass consult an ambient/default region | FALSE | src/postparsing/rules/rules.rs:112-124 | `CallSR` and `LookupSR` have no `region` field today; region info instead lives only on `BorrowRefSR` (rules.rs:141-146) via `RegionSR` |
| 3 (ICLRTP) | This makes `CallSR`/`LookupSR` awkward because they keep carrying region info that's dead weight after higher-typing | FALSE (premise no longer holds — there's no region field on these types to be dead weight) | src/postparsing/rules/rules.rs:112-124 | N/A |

## Stale citation sites
docs/HigherTypingPass.md:168 — says "there's information in some of these classes [LookupSR etc.] that's only used up until the higher typing phase," citing LNTKTR as the reason. Current `LookupSR`/`CallSR` don't carry the region field LNTKTR is about, so the citation no longer supports the point being made there (STRAST's broader argument about weakly-typed rule ASTs may still stand on other grounds, but not on LNTKTR's specific claim).

## Uncited sites that embody the arcana
None found — the current codebase does not implement the ambient-vs-explicit-region choice LNTKTR describes; regions are handled by an unrelated design (group-based aliasing/borrow-checker regions), so there is no code to cite this arcana from.

## Suggested text
Not applicable in the usual sense (this is a G-kind whose described mechanism was abandoned, not merely renamed) — no corrected paragraph is suggested since the underlying "region" system (regions-as-generic-parameter, threaded through CallSR/LookupSR) no longer exists in the compiler. If the maintainer wants a record of it, the honest framing is historical: "Vale's earlier generational-references region design considered attaching regions explicitly to CallSR/LookupSR rule nodes rather than relying on an ambient region during higher-typing; this was abandoned when docs/regions/Regions.md was retired and replaced by the current group-borrowing/aliasing-groups model, where region-like information exists only as RegionSR on BorrowRefSR."
