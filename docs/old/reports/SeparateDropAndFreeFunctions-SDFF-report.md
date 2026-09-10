# Accuracy report: Separate Drop and Free Functions (SDFF)

Audited against the working tree on 2026-09-06. Source: docs/old/Compiler/Templar/Templar.md:247-281

## Verdict
This is a design musing from the old Templar-era docs, not a description of implemented behavior: it proposes splitting `drop()` into a user-overridable `drop()` and a compiler-only `free()` (destroy/discard-refs/call-other-frees) so that `free` calls can be elided under managed backends, Vivem, or bump-allocated regions, and it explicitly defers the harder question ("Though, there is another distinction we need...") to a follow-up section (ADSDF). There are zero citations of "SDFF" anywhere in current code or docs, and the current compiler has no separate `free()` concept — `destructor_compiler.rs` implements only a single `drop`/`get_drop_function` path (src/typing/function/destructor_compiler.rs:19-121), with no elision-under-managed-backend or free-vs-drop split anywhere in src/ or Backend/. The proposal was never adopted. Recommendation: delete (no arcana to migrate — it's an abandoned design alternative, not a description of the codebase, and nothing cites it).

## Claims
This section is a design proposal, not a set of claims about current code. It floats a hypothetical split between `drop()` (user-visible semantics) and `free()` (pure deallocation, elidable under GC/Vivem/bump regions) and says explicitly this "doesn't necessarily need to be exposed to the user" — i.e., it's an open design question, never asserted as implemented. No checkable "the code does X" claims are made.

| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "free will only: Destroy instances / Discard references / Call other free() functions" — proposed distinct `free` function exists or is planned as a real construct | UNVERIFIABLE (proposal, not implemented) | src/typing/function/destructor_compiler.rs:19-121 (only `drop`/`get_drop_function`, no `free`) | No separate `free` function exists in the current compiler; this was never built. |
| 2 | "we can elide any free calls when: Compiled to JS/Java/Swift/C#... In Vivem... In a bump-allocated type-stable region" | UNVERIFIABLE (speculative, never implemented) | grep for "elide" / managed-backend drop elision in src/, Backend/ — none found | No such elision logic exists; those backends (JS/Java/Swift/C#) don't exist in this codebase at all. |
| 3 | "an immutable's drop function is only called when its refcount reaches zero... the array drop method also manually calls drop on its elements" | UNVERIFIABLE / plausibly stale | src/typing/function/destructor_compiler.rs (drop is resolved by name per-kind, no refcount-zero gating visible in this pass) | Refcounting/RC semantics live in a different layer (borrow checker / backend); this file doesn't show the described refcount-triggered call, so the claim can't be confirmed against current typing-pass code as written. |

## Stale citation sites
None — zero code or doc sites cite "SDFF" (confirmed via `grep -rn -w "SDFF"` across src/, Backend/, docs/, excluding docs/old itself).

## Uncited sites that embody the arcana
None found. The proposed drop/free split was never implemented, so there is no code that embodies it to cite from.

## Suggested text
Not applicable (D3 kind, verdict is not major-inaccuracies/obsolete-with-implemented-code — this is an abandoned design alternative with no corresponding code at all, so no corrected arcana paragraph is warranted; recommend straight deletion).
