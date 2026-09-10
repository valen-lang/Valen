# Accuracy report: Pointers in Registers / Serialize Buffers Can Be Offsets (PRCBO)

Audited against the working tree on 2026-09-06. Arcana section: docs/todo/metaprogrammed-record-replay.md:671 ("## Appendix: the prior scheme this replaces (PSBCBO / PRCBO)")

## Verdict
Obsolete. This is the doc's own explicitly-labeled historical appendix — "the prior scheme this replaces" — describing the Linear region's offset-pointer marshaling (`linear.cpp` and its adjuster machinery), which the appendix itself says "is deleted." That deletion is confirmed: `linear.cpp` no longer exists in the repo, and `docs/arcana/IRegion.md:25-27` records "LinearRegion: RETIRED (2026-07)." Every claim in the section is TRUE as a historical description, but the section describes code that no longer exists — it is preserved deliberately as design-history context (per its own framing), not as a claim about current code, so it does not fit "accurate" (which implies checkable current-code claims) and instead is correctly obsolete/retired material. No corrections are needed to the text — it accurately describes what used to exist — but it should not be mistaken for a live invariant.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "the implementing code (linear.cpp and its adjuster machinery) is deleted" | TRUE | `find . -iname linear.cpp` returns nothing; docs/arcana/IRegion.md:25 "LinearRegion: RETIRED (2026-07)" | none |
| 2 | PSBCBO: Linear wrote pointers-as-offsets into buffers destined for C (regular addressing) vs. recording files (file-relative addressing), via a "Serialized Address Adjuster" | UNVERIFIABLE (historical, code gone) | no `getSerializedAddressAdjuster` / linear.cpp in tree | describes deleted C++ code faithfully per docs/todo/ffi-drop-followups.md:53-81, which narrates the same convention while fixing bugs in it |
| 3 | "Address Mode" boolean (mode 0 = pointers, mode 1 = offsets with file begin) on the Linear region object | UNVERIFIABLE (historical, code gone) | same as above | consistent with docs/todo/remove-record-replay-on-experimental.md:207 "Linear's PSBCBO/offsets mode is dead code" |
| 4 | PRCBO: offsets are not translated to real pointers until dereference (load/store through a field), not immediately upon load into a register | TRUE (as historical design) | docs/notes/LinearRegionNotes.md:1-25 gives the full rationale (Option A vs B) matching this description | none |

Claims 2-4 are design-history statements about deleted code; they are internally consistent with every other doc that references them (docs/notes/LinearRegionNotes.md, docs/todo/ffi-drop-followups.md, docs/todo/remove-record-replay-on-experimental.md, docs/arcana/IRegion.md) and none of those citing docs treat PSBCBO/PRCBO as currently active — all explicitly call it retired/prior/dead.

## Stale citation sites
None. All three known sites (docs/arcana/IRegion.md:25, docs/notes/LinearRegionNotes.md:1, docs/todo/ffi-drop-followups.md:62) correctly frame PSBCBO/PRCBO as retired/historical, not as current behavior:
- docs/arcana/IRegion.md:25-28 — "LinearRegion: RETIRED (2026-07)... FFI moved to opaque handles and record/replay was removed; see the PSBCBO/PRCBO appendix ... for the retired scheme and its successor design." Correct.
- docs/notes/LinearRegionNotes.md:1 — companion reasoning note (PRCBOR), consistent with the appendix's PRCBO claim.
- docs/todo/ffi-drop-followups.md:46-81 — narrates in-arc bug fixes made *while PSBCBO/Linear was still live*, correctly cited as historical narrative of that era, not a claim about current code.

## Uncited sites that embody the arcana
None found. No current Rust code under src/typing/rust_interop/ or src/instantiating/rust_interop/ (or anywhere else) implements offset-pointer/adjuster semantics — the mechanism has no live successor in the compiler; record/replay and the opaque-handle FFI replaced it, per docs/arcana/IRegion.md:25-28 and docs/todo/metaprogrammed-record-replay.md itself (whose main body, not this appendix, describes the current design).

## Suggested rewrite
Not applicable — the text is accurate as history and already labeled as an appendix describing deleted code; no rewrite needed beyond what exists. (Report written because the audit protocol classifies "describes code no longer in the codebase" as obsolete rather than accurate, even though nothing here is wrong.)
