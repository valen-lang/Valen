# Accuracy report: Messages Are Pre-Order, With Metadata (MAPOWM)

Audited against the working tree on 2026-09-06. Source: docs/old/Compiler/Midas.md:116-186

## Verdict
Not an arcana: this is a design musing working out message-serialization byte layout (WingPointerArray, region metadata, MAP_GROWSDOWN) for the old C++ Midas backend, explicitly flagged by its own author as stale ("REVISIT THIS, since MAP_GROWSDOWN is obsolete. perhaps we can do a better order"), and none of the identifiers it discusses (WingPointerArray, Catalyst, MAP_GROWSDOWN, rootMetadataBytesNeeded) appear anywhere in the current tree, including the still-present Backend/ C++ source. It has zero citations. Recommendation: delete — nothing to migrate, no citing code to clean up.

## Claims
Not a claim-checkable document — it's an unresolved design exploration ending in "later note: in the future, we could..." with no assertion that this layout was implemented. Spot-checked its central artifacts anyway:

| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Message layout uses `WingPointerArray`, region metadata, and this ordering scheme | UNVERIFIABLE / FALSE as current | grep for `WingPointerArray`, `Catalyst`, `MAP_GROWSDOWN` across repo returns only this doc | No such types or scheme exist in Backend/src (C++) or anywhere in the Rust tree |
| 2 | "MAP_GROWSDOWN is obsolete" (self-flagged) | TRUE (per its own text) | docs/old/Compiler/Midas.md:120-121 | Author already marked it stale |

## Stale citation sites
None — sites list is empty and grep confirms no code or doc cites MAPOWM.

## Uncited sites that embody the arcana
None found — the message-serialization scheme described (WingPointerArray-based interface dispatch buffers) has no counterpart in current Backend/src or src/.

## Suggested text
N/A (not-an-arcana / delete; no suggested text needed per instructions for D3 kind unless major-inaccuracies or obsolete — this is obsolete, but the content is a raw unresolved musing with no reusable design, so nothing worth preserving).
