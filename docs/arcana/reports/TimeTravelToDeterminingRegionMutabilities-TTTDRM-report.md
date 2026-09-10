# Accuracy report: Time Travel To Determining Region Mutabilities (TTTDRM)

Audited against the working tree on 2026-09-06. Source: `git:d097d488^:docs/InstantiatorRegions.md` (deleted doc), section starting at its own line 183, "# Time Travel To Determining Region Mutabilities (TTTDRM)".

## Verdict

The recovered section is a design musing from the Scala-era regions/generational-references design, proposing an algorithm ("perspective region": have `RegionPlaceholderNameT` remember the latest pure block relative to its perspective, so mutability of one region can be computed relative to another) to solve the CTOTFIPB problem (translating types from outside a `pure` block). The doc's own inline annotation already flags it as possibly obsolete ("this might be obsolete, we have heights now"), pointing at the sibling idea RTHPSH (`RegionTemplata(Option[Int])` pure-height). In the current Rust tree neither idea is implemented: `RegionT` (src/typing/types/types.rs:15-19) is a two-variant stub enum (`Iso`, `Default`) with a `// TODO: Get rid of this when we have an actual default region` comment — there is no per-region pure-height integer, no multiit-region mutability system, and no "perspective region" tracked on placeholders. The one echo in code is a field literally named `pure_height` on region placeholders (src/typing/citizen/struct_compiler_generic_args_layer.rs:519,665, src/typing/function/function_compiler_solving_layer.rs:974) — but every site hardcodes it to `None`, so it is unused scaffolding, not RTHPSH's live "count of pure blocks between here and there". A `perspective_region_t` parameter does exist in src/instantiating/instantiator.rs (e.g. lines 375, 529, 619) but it is always `RegionT::Default` — a plumbing placeholder, not the mutability-calculation-from-a-region's-perspective algorithm TTTDRM describes. `pure` blocks themselves are still only parsing-level syntax: src/parsing/tests/statement_tests.rs:954 says outright "The pure block feature doesn't actually exist yet." So the whole problem TTTDRM is trying to solve (translating live types across a `pure` block boundary with per-region mutability) has no implementation to check against — the feature was never built in the Rust port. This is a pre-port design note about an unimplemented, currently-dormant part of the region system. No code cites TTTDRM. Recommendation: delete (do not migrate) — it documents an approach to a feature (multi-mutability regions / pure blocks) that doesn't exist in the current compiler and has no citing code to clean up.

## Claims

This is a design musing, not a set of claims about existing code — flagging per the "no claims" instruction. It proposes an algorithm for a feature (region-relative mutability across `pure` blocks) that is unimplemented today, so there is nothing in the current tree to verify claim-by-claim. The one implicit factual claim worth checking:

| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "(Note from later: this might be obsolete, we have heights now)" — implies RTHPSH's `RegionTemplata(Option[Int])` pure-height scheme superseded TTTDRM and is itself implemented | FALSE (as a claim about current code) | src/typing/types/types.rs:15-19; the `pure_height` field is always `None` at src/typing/citizen/struct_compiler_generic_args_layer.rs:519,665 and src/typing/function/function_compiler_solving_layer.rs:974 | Neither TTTDRM's perspective-region idea nor RTHPSH's pure-height counter is implemented; `RegionT` remains a 2-variant stub and `pure_height` is dead-valued `None` everywhere it's set. |
| 2 | Underlying premise: `pure` blocks exist and need type-translation logic across their boundary | FALSE for current code | src/parsing/tests/statement_tests.rs:952-960 ("The pure block feature doesn't actually exist yet.") | `pure block` is parsed as syntax only; no typing/instantiator logic processes it, so the CTOTFIPB/TTTDRM problem doesn't currently arise. |

## Stale citation sites

None — grep for `TTTDRM` across src/, Backend/, docs/ finds no code citations (only the docs/architecture/instantiator-design.md glossary-style listing that just names it alongside ~20 other IDs as "various spot citations... Most defined in docs/", with no code site given).

## Uncited sites that embody the arcana

None found. The concept (multi-region relative mutability, pure-block type translation) has no implementation to attach a citation to — `RegionT::Default`/`RegionT::Iso` and the always-`None` `pure_height` fields are inert scaffolding for a future feature, not an embodiment of TTTDRM's algorithm.

## Suggested text

Not included — the doc's own claims are FALSE against current code (no implementation exists to accurately re-describe), so there is no accurate paragraph to salvage. If/when multi-region mutability and `pure` blocks are actually implemented in FrontendRust, the underlying idea (compute a type's region-argument mutabilities relative to the region the type was constructed in, rather than relative to the observing site) may be worth re-deriving fresh against whatever mechanism gets built, but reusing this Scala-era text would misdescribe the eventual Rust design.
