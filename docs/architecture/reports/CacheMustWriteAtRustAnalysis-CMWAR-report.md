# Accuracy report: Cache-Must-Write-At-Rust-analysis (CMWAR)

Audited against the working tree on 2026-09-06. Arcana section: docs/architecture/vale-rust-interop-architecture.md:3430 ("### 26.17 CMWAR (Cache-Must-Write-At-Rust-analysis)")

## Verdict

The section states, in present tense, that "Cache writes route through `after_rust_analysis` only" and that this is enforced structurally by "a marker/token type that only the `after_rust_analysis` callback constructs." None of this exists in the code: there is no `after_rust_analysis` (or `after_analysis`) callback implemented anywhere, no `.vale-cache` file, and no cache-write token/marker type. The repo's own handoff doc — the only citation site — already says as much: "`after_analysis` is unbuilt. No callbacks struct implements it and there is no `.vale-cache`; the design reserves it as the cache-write point (@CMWAR)" (docs/handoffs/rust-interop-handoff.md:43). So CMWAR is written as a description of an existing enforced discipline but is actually a forward design note for a mechanism that has not been built at all. This is a factually false claim about current code, not merely stale wording.

## Claims

| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "Cache writes route through `after_rust_analysis` only." | FALSE | No `after_rust_analysis`/`after_analysis` callback exists in src/typing/rust_interop or src/instantiating/rust_interop; confirmed unbuilt at docs/handoffs/rust-interop-handoff.md:43 | Should read as a design intention ("will route"), not a present-tense fact |
| 2 | "Never from inside codegen-time callbacks." | UNVERIFIABLE (moot) | No cache-write code exists anywhere to check this against | N/A until cache is built |
| 3 | "Sky's empirical history (two-write-sites cleanup) validates this as real deadlock-prevention discipline against @GCMLZ re-entry." | UNVERIFIABLE | External/historical claim about the Sky project, not checkable in this repo | Leave as rationale, but flag it supports a design choice for unbuilt code |
| 4 | "Detection: structural enforcement... Cache-write functions are gated behind a marker/token type that only the `after_rust_analysis` callback constructs." | FALSE | `grep -rn "CacheWriteToken\|cache_write\|\.vale-cache" .` over src/ returns nothing; no such type exists | The token/gating mechanism described does not exist; nothing to detect a violation of yet |

## Stale citation sites

- docs/handoffs/rust-interop-handoff.md:43 — this site itself is accurate (it correctly flags CMWAR's mechanism as unbuilt); it is the arcana section (docs/architecture/vale-rust-interop-architecture.md:3430) that is stale relative to this handoff, phrasing an unbuilt design as an already-enforced current discipline.
- docs/architecture/vale-rust-interop-architecture.md:174, 1108, 1400, 3066, 3348 — all internal restatements of the same present-tense claim ("cache writes route through `after_rust_analysis` only") within the same document; none flag that the mechanism is unbuilt. These are consistent with each other but collectively stale against the actual code state documented in the handoff.

## Uncited sites that embody the arcana

None found — there is no code implementing CMWAR to cite it.

## Suggested rewrite

> ### 26.17 CMWAR (Cache-Must-Write-At-Rust-analysis)
>
> **Vale-specific, unbuilt.** Design intent: cache writes will route through an `after_rust_analysis` callback only, never from inside codegen-time callbacks. Sky's empirical history (two-write-sites cleanup) motivates this as deadlock-prevention discipline against @GCMLZ re-entry.
>
> As of 2026-09, no `after_rust_analysis` callback, `.vale-cache` file, or cache-write token type exists in the tree (see docs/handoffs/rust-interop-handoff.md, "Design doc vs as-built"). When built, detection should be **structural enforcement, not a source scan**: gate cache-write functions behind a marker/token type that only the `after_rust_analysis` callback can construct, so no other call site can invoke cache-write and misuse fails rustc's own typecheck rather than a CI pass — the same "make it structurally impossible" pattern as Sky's mutex-hierarchy invariants.
