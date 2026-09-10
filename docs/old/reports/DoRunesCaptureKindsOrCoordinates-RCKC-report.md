# Accuracy report: Do Runes Capture Kinds or Coordinates? (RCKC)

Audited against the working tree on 2026-09-06. Source: docs/old/Parser_Scout.md:280-316

## Verdict
This is a closed design deliberation from the Scala-era parser about whether pattern runes like `#T` (in `a: #T`) should capture Kinds or Coordinates — it ends with a decision ("go with the second, and capture the coords... make an exception for `a: &T`, aka BCTE") rather than a standing claim about present code. The `#T` rune-capture sugar and the `BCTE` exception it names do not appear anywhere in current src/ or Backend/, and RCKC has zero citations from code (`sites: []` in the record, confirmed by grep). It is a settled historical decision record, not a cross-cutting concern that current code embodies or needs to reference. Recommendation: leave it in docs/old/ (or delete) — not-an-arcana, nothing to migrate.

## Claims
Not-an-arcana: the section is a design deliberation ("what do those runes capture? Kinds or coords?") that concludes with a decision, not an assertion about how the code currently behaves. No numbered claims to check — the `#T`/`&T` syntax it discusses is not present in the current Rust compiler (grep for the syntax and for BCTE turned up nothing outside this doc file).

## Stale citation sites
None — RCKC has no citations in src/, Backend/, or docs/ outside docs/old/Parser_Scout.md itself (the ID appears only in the doc's own table of contents entry).

## Uncited sites that embody the arcana
None found. The specific `#T`-capturing rune sugar this section debates was never carried into (or was later removed from) the current parser/typing pass; there is no analogous "kind vs. coordinate capture" decision point findable in src/typing today to attach this ID to.

## Suggested text
N/A (not-an-arcana; no migration needed).
