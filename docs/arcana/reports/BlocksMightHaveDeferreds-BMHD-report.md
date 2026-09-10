# Accuracy report: Blocks Might Have Deferreds (BMHD)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/ret-vs-panic-locals.md:151 ("# Blocks Might Have Deferreds (BMHD)")

## Verdict
The core idea — that block translation must schedule a trailing expression's deferred (temporary) destruction before/around the implicit discarding of earlier statements, because all but the last statement in a block get discarded — still holds in the current typing pass. But the section is an unedited copy of an old Scala-era comment block: it never names any current (or even old) type/function, has zero citation sites anywhere in src/ or docs/, and its example uses syntax (`println(Marine(4).hp);` then a bare `= [4, 5].0;` line) that doesn't match any current test fixture I could find. The mechanism it describes is implemented today in src/typing/expression/block_compiler.rs's `evaluate_block_statements_block`, via `ConsecutorTE` (for multi-statement discarding, per the neighboring CWNWMT section) and `Compiler::drop_since` (for deferred temp/local destruction) — but nothing in the doc or the code cross-references this, so a reader can't verify the claim without independently finding block_compiler.rs.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Naively one would translate deferreds after each particular expression | UNVERIFIABLE | — | Design musing, not a checkable code claim. |
| 2 | The last statement in a block can be a returning expression with a pending defer (e.g. an array literal whose temp must be destructed after its `.0` is read) | TRUE | src/typing/expression/block_compiler.rs:32-59 | `pending_drops_from_exprs` returned from `evaluate_expression` are threaded into `drop_since` after the whole block expr is evaluated — same shape as the claim. |
| 3 | "We do deferreds before any discards, since all lines except the last in a block will be discarded" | DRIFTED | src/typing/expression/block_compiler.rs:22-70 | Current code evaluates the entire block as one expression (which internally handles per-statement discarding via `ConsecutorTE`), then calls `drop_since` twice afterward (temp drops, then live-local drops) — i.e. deferred destruction is layered on *after* the whole discard-bearing expression is built, not literally "before" it in the sequence the doc implies. The end effect (correct destruction ordering relative to discards) is preserved, but "before any discards" no longer describes the code structure precisely. |
| 4 | Example: `fn main() { println(Marine(4).hp); = [4, 5].0; }` | UNVERIFIABLE | — | Could not find this fixture or an equivalent leading-`=`-statement pattern in src/typing/test; syntax may be stale Scala-era Vale syntax. Not confirmed broken, just unconfirmed as still-valid. |

## Stale citation sites
None — the section has zero citation sites (`grep -rn -w "BMHD"` finds only the two doc definitions, in docs/arcana/ret-vs-panic-locals.md and the superseded docs/old/... copy).

## Uncited sites that embody the arcana
- src/typing/expression/block_compiler.rs:22 — `evaluate_block_statements_block`, the actual current implementation of "deferreds vs. discards" ordering for blocks.
- src/typing/expression/block_compiler.rs:44-70 — the two `drop_since` calls (pending temp drops, then live-variable drops) that realize the deferred-destruction behavior described.
- src/typing/expression/expression_compiler.rs:2651 — `evaluate_block_statements`, the entry point that delegates to block_compiler.rs.

## Suggested rewrite
Not written — verdict is minor-inaccuracies (core idea intact), not major/obsolete.
