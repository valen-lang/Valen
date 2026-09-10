# Accuracy report: Must Tether Inline Varyings (MTIV)

Audited against the working tree on 2026-09-06. Source: `docs/old/HGM V16, V17, V18.md:379-398`

## Verdict
This is a design musing from the old Scala-era "HGM" generational-references memory-model notes: a prescriptive rule the authors were considering ("we must tether the inner generation if we want to read or write it...") plus a follow-on open question (CITBD) about how to implement the check ("This might require recursive functions, but maybe not... fancy table magic"). It makes no factual claim about code as it exists today — it proposes a rule for a design that was never built this way. `sites` is empty (no live code citations), and grepping the tree finds zero occurrences of "MTIV" outside this same doc file (one self-reference at line 407) and zero occurrences of "tether" as a real mechanism anywhere in `src/` or `Backend/` — the only hit is `Backend/test/tethercrash.vale`, an old fixture whose `__tether` syntax is unrelated vale-source flavor text, not a citation of this arcana or evidence the mechanism was implemented. The current compiler's memory-model story (region-based borrow checker in `src/typing/borrow_checker/`, GenRC in `Backend/src/region/rcimm/`) does not implement "tethering" of inline varyings as described. Recommendation: leave it where it is (docs/old) — it is a design musing/open question, not an arcana with checkable claims, and there is nothing here worth migrating or actively deleting.

## Claims
Not applicable — this is a design musing/open question about a proposed rule, not an assertion about existing code behavior. No numbered claims are checkable against the current codebase.

## Stale citation sites
None — `sites` is empty; the only other occurrence of "MTIV" in the tree is the same document's own line 407 (`docs/old/HGM V16, V17, V18.md:407`), a self-reference, not a code citation.

## Uncited sites that embody the arcana
None found — "tether" as a runtime/typing concept does not appear in `src/` or `Backend/src/`; the region/generational-reference machinery that does exist (`src/typing/borrow_checker/`, `Backend/src/region/rcimm/rcimm.cpp`, `Backend/src/region/common/common.cpp`) uses a different mechanism (borrow-checker-proven aliasing / RC generations) and does not implement inline-varying tethering as this note describes.
