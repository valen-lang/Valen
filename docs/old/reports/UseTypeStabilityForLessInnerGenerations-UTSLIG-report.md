# Accuracy report: Use Type Stability for Less Inner Generations (UTSLIG)

Audited against the working tree on 2026-09-06. Source: docs/old/HGM V20.md:195-262

## Verdict
This is a design musing, not a documented fact about existing code: it proposes that deeply type-stable struct members (e.g. plain `int` fields, fixed-size arrays of them) don't need an inner generation number in a generational-references (HGM) memory model, and walks through a hypothetical `Spaceship` example plus one caveat about scope-tethering on inline borrows. It has zero code citations (`sites: []`), a grep for `UTSLIG` across src/, Backend/, and docs/ finds no hits outside this file, and there is no "type-stable"/"deeply type-stable" concept anywhere in the current Rust codebase — the current borrow checker (src/typing/borrow_checker/) implements a completely different mechanism (group-based aliasing/noalias analysis, per recent commits), not generational references with per-field generation counters. The HGM V-doc series describes an alternate memory-model design (generational references) that was never built this way in the current compiler. No claims here are checkable against real code. Recommendation: leave it in docs/old/ (not a candidate for docs/arcana migration) — it documents an abandoned/superseded design direction, not the current system.

## Claims
Not-an-arcana: the text contains no verifiable claims about the current codebase. It's self-contained speculative design reasoning (a memory-model optimization idea with a worked example), with no reference implementation, no citing code, and no counterpart concept in the current Rust compiler.

## Stale citation sites
None — sites list is empty and grep confirms no citations exist anywhere in src/, Backend/, or docs/ (outside this file itself).

## Uncited sites that embody the arcana
None found. The current borrow checker uses group-based noalias/aliasing analysis (src/typing/borrow_checker/), not generational references or per-field "type stability" elision, so there is no code this concept could be migrated onto.

## Suggested text
Not applicable (D3 kind, verdict is obsolete, not major-inaccuracies) — no suggested arcana text is warranted since the underlying memory model (generational references with per-field generation counters) isn't part of the current design.
