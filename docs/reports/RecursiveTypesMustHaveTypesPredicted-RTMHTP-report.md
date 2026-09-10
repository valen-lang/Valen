# Accuracy report: Recursive Types Must Have Types Predicted (RTMHTP)

Audited against the working tree on 2026-09-06. Arcana section: docs/HigherTypingPass.md:105 ("# Recursive Types Must Have Types Predicted (RTMHTP)")

## Verdict
Obsolete. The section describes a Scala-era mechanism — a "Scout" compilation phase with a "predictor" that forward-declares rune types for recursive structs, and a "Templar" phase that consumes those predicted types to disambiguate template-argument templatas during calls. None of Scout, Templar, Astronomer, or "predictor" exist anywhere in the current Rust codebase (`src/`); the pipeline today is postparsing → typing (with rust_interop) → instantiating → backend, with no separate scout/predictor pass. The doc itself frames this as a historical note ("Previously: ... In Scout") documenting a bug fix from the Scala compiler, not a claim about current code, and it has zero citations anywhere in src/, Backend/, or docs/. It reads as an abandoned/superseded design note rather than a live invariant.

## Claims
| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | There is a compiler phase called "Scout" containing a "predictor" that types recursive struct runes ahead of time | FALSE (obsolete) | `grep -rn "Scout\|Astronomer\|predictor" src` returns nothing | No Scout phase exists in the current Rust compiler; typing/rune-solving lives in src/typing/ (see e.g. src/typing/solver or rules-typing code) with no separate scout pre-pass. |
| 2 | "Templar will use those types to disambiguate template arg templatas during calls" | FALSE (obsolete) | `grep -rn "Templar" src` returns nothing | The Scala "Templar" was renamed/replaced by the Rust `src/typing/` pass; the specific disambiguation mechanism described here (consuming Scout-predicted rune types) has no current counterpart under that name. |
| 3 | The `MyList<Ref#T>` / `MyOption<MyList<#T>>` infinite-loop example is a live compiler concern | UNVERIFIABLE / stale example | No `MyList`/`MyOption` recursive-rune-typing test found under src/typing/test | Cannot confirm the underlying infinite-loop bug still applies to the current typing pass's cycle handling; example syntax (`Ref#T`) is Scala-era rune syntax not matching current Vale surface syntax patterns used elsewhere in the repo's current tests. |

## Stale citation sites
None — there are zero citations to RTMHTP anywhere in src/, Backend/, or docs/ (`grep -rn -w "RTMHTP"` and `grep -rn "@RTMHTP"` both empty).

## Uncited sites that embody the arcana
None found — no Scout/predictor/Templar concept exists in current code to embody this arcana.

## Suggested rewrite
This section should either be deleted or explicitly re-labeled as historical/archival, since it documents a Scala-Templar-era bugfix with no bearing on the current Rust typing pass. Suggested replacement text:

> # Recursive Types Must Have Types Predicted (RTMHTP) — HISTORICAL
>
> This section documents a bug and fix from the old Scala compiler (Scout/Templar/Astronomer phases), which no longer exist in the current Rust frontend (src/typing/, src/instantiating/). It is kept for historical context only; if recursive-type rune-solving in the current typing pass hits an analogous infinite loop, that would need a fresh investigation and a new arcana entry, not a revival of this one.
