# Accuracy report: Matching Imprecise Names Against Absolute Names (MINAAN)

Audited against the working tree on 2026-09-06. Source: docs/old/Names.md:1-77 (unchanged since commit 6d3cd979, "Adding compiler docs", 2022-07-03; zero live citations)

## Verdict

This is a worked-out algorithm sketch (a scratch design note walking through examples of matching an "imprecise" name like `[moo, bork]` against an "absolute" env path `[foo, bar]` by slicing prefixes and filtering by length/equality), not a description of code that exists. It has no `@MINAAN`/`see MINAAN` citations anywhere in src/, Backend/, or docs/, and the actual current mechanism for imprecise-name lookup (`TemplatasStoreT` in src/typing/env/environment.rs, `get_imprecise_name`/`lookup_with_imprecise_name_inner` in src/typing/env/function_environment_t.rs) works completely differently: entries are indexed by their own imprecise (last-step) name in a map (`imprecise_to_entries` / `new_entries_by_name_s`, src/typing/env/environment.rs:663,908-990) and lookup walks the *lexical scope chain* (`parent_env.lookup_with_imprecise_name_inner`, src/typing/env/function_environment_t.rs:1104-1128), not by slicing the current absolute path into prefix candidates and comparing them against candidate name endings as the doc describes. The doc's algorithm was apparently never implemented this way, or was implemented and later replaced by the map+scope-chain approach with no trace left. Since it's an unreferenced, never-realized design musing with no checkable claims that hold against current code as an accurate description, it doesn't qualify for migration into docs/arcana. Recommendation: delete (docs/old/Names.md, no citing comments exist to clean up).

## Claims

| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | Imprecise-name matching works by slicing the absolute env path into all prefix "first halves," filtering candidate struct names whose end matches the imprecise needle, then comparing surviving prefixes to the current env path (missing entries in the "hay" are fine) | FALSE (as a description of current code) | src/typing/env/environment.rs:908-990 (entries indexed by imprecise name at insertion time); src/typing/env/function_environment_t.rs:1104-1128 (`lookup_with_imprecise_name_inner` walks `parent_env` scope chain, no prefix-slicing/filtering step) | Current lookup is a direct map keyed by imprecise name, populated when entries are added to a scope, combined with parent-env chain traversal — not the prefix-slice-and-filter algorithm described. |
| 2 | (implicit) There exists a "current env" represented as an absolute name path, and a notion of "imprecise name" distinct from absolute name, used during name resolution | TRUE | `IImpreciseNameS` / `INameT` types throughout src/typing/env/environment.rs and src/typing/names/names.rs | This part of the vocabulary survived; only the matching *algorithm* described is not what's implemented. |

This is fundamentally a design-musing document (worked examples toward an algorithm) rather than a set of assertions about existing code, so most of its content falls outside the TRUE/FALSE claim framework — see verdict.

## Stale citation sites

None — zero citations of MINAAN exist in the tree (confirmed via `grep -rn -w "MINAAN"` across src/, Backend/, docs/).

## Uncited sites that embody the arcana

None found in the sense the doc describes (prefix-slicing algorithm). The *general concept* of imprecise-vs-absolute name resolution is embodied at:
- src/typing/env/environment.rs:908-990 — `TemplatasStoreT::add_entries`, indexes entries by imprecise name
- src/typing/env/function_environment_t.rs:1104-1128 — `lookup_with_imprecise_name_inner`, scope-chain lookup

But since these implement a different mechanism than what MINAAN describes, citing MINAAN from them would misdescribe the code, not document it.

## Suggested text

Not applicable (D3 kind; verdict is obsolete/delete, not major-inaccuracies requiring a corrected replacement — the mechanism this doc envisioned isn't what shipped, so there is nothing accurate to preserve).
