# Accuracy report: PlaceholderTemplata, PlaceholderKind, Coords, Kinds (PTPKCK)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:264 ("# PlaceholderTemplata, PlaceholderKind, Coords, Kinds (PTPKCK)")

## Verdict
The core mechanism described is still real and accurate: `PlaceholderTemplataT` (src/typing/templata/templata.rs:154) is a genuine type used to stand in for generic parameters during a denizen's definition, and later resolved by the instantiator's substitution logic (src/instantiating/instantiator.rs:2238-2240, `translate_templata`/`translate_placeholder`). The one drifted detail is the doc's name for the special kind used for coord-placeholders: it says `PlaceholderKindT`, but the actual type is `KindPlaceholderT` (variant `KindT::KindPlaceholder`, src/typing/types/types.rs:64,402) — word order swapped. No code cites PTPKCK at all, so there are no stale citation sites, only the naming drift inside the doc text itself.

## Claims
| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | For a generic denizen's definition, the compiler conjures up placeholders, e.g. `PlaceholderTemplata(repeat$0, IntegerTemplataType)` | TRUE | src/typing/templata/templata.rs:154 `PlaceholderTemplataT { id, tyype }`; `ITemplataType::IntegerTemplataType` at src/postparsing/itemplatatype.rs (used in src/typing/array_compiler.rs:62) | — |
| 2 | The instantiator later sees callers' actual arguments and does the substitution | TRUE | src/instantiating/instantiator.rs:2238-2240 substitutes `ITemplataT::Placeholder(p)` via a `_substitutions` map keyed by `IdT` | — |
| 3 | "Any kind of templata works like this... except for kinds" — locals/members can't hold `ITemplata[CoordTemplataType]`, only `CoordT`s, so a special kind `PlaceholderKindT` is used, and "if a CoordT contains a PlaceholderKindT, that's a coord placeholder" | DRIFTED | Real type is `KindPlaceholderT` (struct, src/typing/types/types.rs:402) as the `KindT::KindPlaceholder` variant (types.rs:64); `CoordT` is a real struct with `ownership`/`kind` fields (confirmed via src/integration_tests/tests/if_tests.rs:153,163) | Rename `PlaceholderKindT` → `KindPlaceholderT` in the doc text |
| 4 | "At some point, we'll make our locals, members, etc. all contain ITemplatas so we don't need this special kind" / "Perhaps there's even a way to have everything be just a rune..." | UNVERIFIABLE (forward-looking musing) | N/A — aspirational, not a claim about current code | — |

## Stale citation sites
None — no site in src/, Backend/, or docs/ cites PTPKCK except the arcana file's own header.

## Uncited sites that embody the arcana
- src/typing/templata/templata.rs:154 — `PlaceholderTemplataT` definition
- src/typing/types/types.rs:402 — `KindPlaceholderT` definition (the section's "PlaceholderKindT")
- src/instantiating/instantiator.rs:2238-2240 — the substitution step the section describes as happening "later on in the instantiator"
- src/typing/array_compiler.rs:594,721 — `create_kind_placeholder_inner`, concrete placeholder creation for a generic case

## Suggested rewrite
(Minor drift only — full rewrite not required per rules, but the one-line fix:)

Replace: "we make a special kind, `PlaceholderKindT`. If a CoordT contains a `PlaceholderKindT`, then that's the same thing as a 'coord placeholder' so to speak."

With: "we make a special kind, `KindPlaceholderT` (the `KindT::KindPlaceholder` variant). If a CoordT contains a `KindPlaceholderT`, then that's the same thing as a 'coord placeholder' so to speak."
