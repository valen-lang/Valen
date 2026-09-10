# Accuracy report: Matching Doesnt Evaluate Struct Or Interface (MDESOI)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Impls.md:1-115 ("Need Match-Only Rule For Impls (NMORFI) / Also: Matching Doesnt Evaluate Struct Or Interface (MDESOI)")

## Verdict

This is a Scala-era design log: it narrates a stack-overflow problem in the old rule-engine impl resolver and walks through four candidate fixes (A-D), concluding "we'll go with D for now, but long term we'll want to do C." None of that machinery exists in the current Rust typing pass. `src/typing/citizen/impl_compiler.rs` resolves impls (`resolve_impl`, `is_parent`, `get_impl_parent_given_sub_citizen`) by seeding the struct-kind and/or interface-kind runes directly as `InitialKnown` values passed into a general fixed-point solver (`src/solver/solver.rs`, `src/typing/infer_compiler.rs`), rather than emitting two rule-orderings per impl depending on match direction as "Solution D" describes. There is no ordering trick, no `match(...)` pseudo-rule, and no "two sets of rules for every impl." The doc has zero citations anywhere in the codebase (`grep -w MDESOI` and `grep NMORFI` both come up empty in src/, only the doc itself and one comment in an integration test reference NMORFI by name, not the mechanism). Since the concrete mechanism the section describes and recommends is gone — replaced by a different, more general design — this is obsolete, though the underlying goal it names (don't fully evaluate the struct/interface kind when matching an impl) is still honored, just by a different means.

## Claims

| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | Evaluating a struct evaluates all its parent interfaces, which evaluates all impls in the program, causing stack overflows | UNVERIFIABLE (historical) | no trace of this old evaluation path remains | Describes long-gone Scala-era Templar behavior; not checkable against current code |
| 2 | We need matching against an impl's struct/interface to not fully evaluate it (MDESOI) | DRIFTED | src/typing/citizen/impl_compiler.rs:591-602 (`get_impl_parent_given_sub_citizen`), :769-800 (`is_parent`) | Goal still holds, but achieved via `InitialKnown` seeding, not via a match-only rule or rule ordering |
| 3 | Solution A ("special match rule") doesn't work due to deadlock | UNVERIFIABLE (historical) | — | No `match(...)` rule construct exists in current solver code (`grep -rn "fn match" src/solver` finds nothing relevant) |
| 4 | Solution B (plain ordering) doesn't work | UNVERIFIABLE (historical) | — | n/a |
| 5 | Solution C: two rule-sets per impl (struct-first vs interface-first), with the non-primary rune's kind rule wrapped in `match(...)` | FALSE for current code | impl_compiler.rs:583-620, :769-820 | Current code builds ONE set of rules per impl and instead supplies `InitialKnown{rune, templata}` entries for whichever of struct-kind/interface-kind rune(s) are already known before solving — no duplicated rule sets, no `match()` wrapper |
| 6 | Solution D: order the struct rule first when matching from a struct, interface rule first when matching from an interface, so it "fails early" | FALSE for current code | impl_compiler.rs:591-596 (only struct_kind_rune seeded via InitialKnown when going struct→interface) vs :769-782 (both struct_kind_rune and super_kind seeded when checking `is_parent` for a specific pair) | Current resolver doesn't reorder rules by direction at all; it hands the solver pre-known values for the runes it already has, and lets the fixed-point solver (`src/solver/solver.rs`) proceed in whatever order it likes |
| 7 | "We'll go with D for now, but long term we'll want to do C" — implies D is the current implementation | FALSE | see #5, #6 | Neither D nor C, as literally described, is what ships; the actual solution is a generalized InitialKnown-seeding approach not enumerated in the doc |

## Stale citation sites

None — MDESOI has zero citations in src/, Backend/, or docs/ outside the defining file itself (confirmed via `grep -rn -w "MDESOI"`). NMORFI (the sibling ID in the same doc, describing the same underlying fix) is cited once, at src/integration_tests/tests/integration_tests_c.rs:449, as historical color for a test — it does not describe present-day machinery either, but that comment doesn't make any mechanism claim, so it isn't "stale" in the same sense.

## Uncited sites that embody the arcana

- src/typing/citizen/impl_compiler.rs:591-602 (`get_impl_parent_given_sub_citizen`) — seeds `struct_kind_rune` as an `InitialKnown` before solving, the modern analog of "match the struct without fully evaluating it."
- src/typing/citizen/impl_compiler.rs:769-800 (`is_parent`) — seeds both `struct_kind_rune` and `interface_kind_rune` as `InitialKnown` when checking a specific sub/super pair.
- src/typing/citizen/impl_compiler.rs:624-706 (`get_parents`) — narrows candidate impls by imprecise name lookup before ever calling into the solver, which is the actual current answer to "impls all have the same name, so we can't narrow it down beforehand" (the doc's stated blocker).
- src/typing/infer_compiler.rs:118-135 (`InitialKnown` struct and its threading through `resolve_impl`/solve entry points) — the core mechanism this arcana's problem is now solved by.

## Suggested rewrite

The historical problem narrative (paragraphs before "Solution A") is fine to keep as-is for context. The solution section should be replaced or clearly marked as superseded, e.g.:

> **Update (current implementation):** The Rust typing pass didn't end up implementing Solution C or D above. Instead, impl resolution (`src/typing/citizen/impl_compiler.rs`: `resolve_impl`, `is_parent`, `get_impl_parent_given_sub_citizen`) narrows candidate impls by imprecise name first (addressing "impls all have the same name"), then seeds whichever of the struct-kind / interface-kind runes are already known as `InitialKnown` values (`src/typing/infer_compiler.rs`) before running the general fixed-point solver. This sidesteps re-evaluating the struct or interface kind entirely for the already-known side, without needing per-direction rule sets or a special `match(...)` rule. MDESOI itself (matching shouldn't force full evaluation) still holds; it's just no longer implemented via rule ordering.
