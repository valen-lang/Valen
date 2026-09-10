# Accuracy report: Giving Argument Ownership For Param Subtypes (GAOFPS)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Impls.md:114-183 ("Giving Argument Ownership For Param Subtypes (GAOFPS)")

## Verdict
Obsolete. The section is a Scala-era design musing about `InfererEvaluator`, an "astronomer" rune-solving pass, and rune names (`Param1Coord`, `Param1Kind`, `Param1Ownership`, `anonRune1/2`) plus a proposed `toRef` auto-insertion fix. None of this machinery exists in the current Rust codebase: there is no `InfererEvaluator`, no astronomer/rune-typing pass under that name, no `toRef`, and no `Param1*` runes anywhere in `src/`. No code cites `GAOFPS` (only the doc's own self-reference at Impls.md:120). The section is entirely an unresolved "what should we do" musing (ends mid-thought proposing Astronomer plug dummy values and equate them) with no claim that current code implements any of it.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "In InfererEvaluator (search for GAOFPS) we're trying a bunch of possibilities" | FALSE/OBSOLETE | grep for `InfererEvaluator` across repo returns only docs/arcana/Impls.md itself; no matches in src/ | No such type/module exists; the current solver lives in src/typing/infer/compiler_solver.rs with no GAOFPS marker |
| 2 | "we're feeding in the given argument ownership... into the rule solver" via `Param1Coord` rune | UNVERIFIABLE/OBSOLETE | grep for `Param1Coord`, `Param1Kind`, `Param1Ownership` in src/ — no matches | These rune names don't exist in the Rust solver's rune/type-var naming scheme |
| 3 | Proposed fix: "astronomer automatically inserts a toRef call" | UNVERIFIABLE/OBSOLETE | grep for `toRef` in src/ — no matches; no "astronomer" pass in Rust tree | Whatever coord-from-kind coercion exists today (if any) has a different name/mechanism not covered here |
| 4 | Section is itself a musing ending "Perhaps Astronomer can do this..." with no resolution | TRUE (as a musing) | docs/arcana/Impls.md:180-183 | This part is accurately described as unresolved speculation, but it's about defunct machinery |

## Stale citation sites
None — there are no code citations of GAOFPS to be stale (grep confirms zero hits outside the doc itself).

## Uncited sites that embody the arcana
None found — the ownership-inference problem may still exist in some form in src/typing/infer/compiler_solver.rs, but nothing there uses the vocabulary (Param1Coord, toRef, InfererEvaluator) this section describes, so there's nothing to point to as "this is GAOFPS in current code."

## Suggested rewrite
This section describes long-superseded Scala-era solver internals (InfererEvaluator, astronomer, Param1* runes, toRef) that have no counterpart in the current Rust typing pass. Recommend either deleting the section outright or replacing it with a note that the underlying problem (owning-vs-shared ambiguity when matching an argument's kind against a template parameter) may still apply to src/typing/infer/compiler_solver.rs, but the described mechanism and terminology no longer exist and would need to be re-derived from current code rather than salvaged from this text.
