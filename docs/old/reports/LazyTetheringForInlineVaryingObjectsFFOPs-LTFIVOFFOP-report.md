# Accuracy report: Lazy Tethering For Inline Varying Objects' FFOPs (LTFIVOFFOP)

Audited against the working tree on 2026-09-06. Source: docs/old/HGM V16, V17, V18.md:462-556

## Verdict
This is a design musing from the old "HGM" generational-reference memory-model notes, proposing a scheme ("lazy tethering") for taking borrow references into inline type-stable/varying struct chains without eagerly tethering, deferring the tether to the first heap-owned object encountered. It makes no claims about code as it currently exists — it's future-tense design speculation ("If we want to pass to an impure function...", "we'd tether the heap-owned object right then") for a `tether`/generational-reference-based borrowing model. Grepping the tree shows the "tether" and "generation"/"varying inline struct" concepts it depends on are absent from the current typing pass entirely (`src/typing/borrow_checker/` uses an unrelated group-based aliasing/noalias scheme — see recent commits 5ae6a825, 04bf7c70), and "tether" survives only as a Backend C++/region-runtime term (`Backend/src/region/rcimm/rcimm.cpp`, `Backend/test/tethercrash.vale`) unconnected to this specific inline-object proposal. There are zero code citations (sites: []) and no code anywhere implements "lazy tethering" for inline varying objects. Recommendation: delete — this is superseded design speculation for an abandoned generational-tethering borrowing scheme, with no current code to migrate a doc for.

## Claims
Not a set of verifiable claims about current code — the whole section is a hypothetical design proposal ("This approach says that... we don't necessarily need to tether anything right away", worked example with Spaceship/Engine/FuelCell/Thing, rules for what to do "if we want to pass to an impure function"). No claim asserts what the compiler does today; every sentence is proposing what a future generational-reference implementation *should* do.

| # | Claim | Status | Evidence | Correction |
|---|---|---|---|---|
| 1 | "we don't necessarily need to tether anything right away" (design proposal, not a code claim) | UNVERIFIABLE | n/a | Not a claim about existing code |
| 2 | Depends on "type stability" / "varying inline struct" / generational-reference tethering machinery existing | FALSE (as a claim about current code) | `grep -rn tether src/typing` and `grep -rn generational src/typing` return nothing | This machinery does not exist in the current Rust typing pass; the current borrow checker (src/typing/borrow_checker/) implements a different group/noalias-based scheme, not generational tethering |

## Stale citation sites
None — sites list is empty; no code cites LTFIVOFFOP.

## Uncited sites that embody the arcana
None found — no code implements tethering, inline-varying-object borrow deferral, or anything resembling this scheme. The concept was never realized in the Rust port.

## Suggested text
N/A — not applicable for D3 unless major-inaccuracies/obsolete and worth migrating. This section describes a design that was never implemented and has no current embodiment; there is nothing to migrate into docs/arcana. Delete the section (or leave it archived in docs/old/ as historical record, per maintainer preference) rather than create a new arcana doc.
