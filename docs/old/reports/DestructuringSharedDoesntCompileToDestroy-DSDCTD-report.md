# Accuracy report: Destructuring Shared Doesnt Compile To Destroy (DSDCTD)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Compiler/Templar/Patterns.md:1 ("**Destructuring Shared Doesnt Compile To Destroy** (DSDCTD)")

## Verdict
The core idea — destructuring a shared/immutable value compiles to aliasing, not a destroy — is still true and directly visible in the Rust typing pass (pattern_compiler.rs dispatches `ShareRef`/`BorrowRef` to `destructure_non_owning_and_maybe_continue`, and only bare owned values to `destructure_owning`). But the doc's supporting detail is stale: there is no `Destroy2` instruction in the current Backend (only a single unified `Destroy`/`DestroyRuntimeSizedArray`/`DestroyStaticSizedArrayInto*` family in Backend/src/metal/instructions.h), and the "asserts no more references, then deallocates" behavior it describes is currently an unimplemented stub (`assert(false); throw 1337;`) in the RCImm region backend rather than working code. Recommendation: migrate the core claim (retitled/reworded without "Destroy2") into docs/arcana/, but flag the imm-destructor refcount-assert detail as presently unimplemented rather than restating it as fact.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "We also destroy shared things." / destructuring `(x,y,z) = Vec3(3,4,5)` on a shared value does not evaluate to a Destroy, but to aliasing | TRUE | src/typing/expression/pattern_compiler.rs:327-337 (KindT::BorrowRef/ShareRef → destructure_non_owning_and_maybe_continue) vs :348-360 (bare owned value → destructure_owning) | none needed |
| 2 | "we *do* have a Destroy2 instruction in every imm struct's destructor" | FALSE (name) | Backend/src/metal/instructions.h:135 defines only `class Destroy` — no `Destroy2` class anywhere in Backend/ or src/ | There is one `Destroy` instruction, not a separate `Destroy2`; the Midas-era split apparently didn't survive the C++/current backend |
| 3 | "That asserts that there are no more references, and then deallocates it" | DRIFTED/currently unimplemented | Backend/src/region/rcimm/rcimm.cpp:345-352 `RCImm::discardOwningRef` body is `{ assert(false); throw 1337; }` — a stub, not a refcount-check-then-deallocate | The behavior described is aspirational/not-yet-implemented in the active RCImm region; note this instead of stating it as fact |

## Stale citation sites
None (no known `sites` were recorded, and grep for `DSDCTD` across src/, Backend/, docs/ found zero hits — the ID is not cited anywhere in code).

## Uncited sites that embody the arcana
- src/typing/expression/pattern_compiler.rs:327-337 — dispatches ShareRef/BorrowRef destructuring to the non-owning/aliasing path (the exact mechanism the arcana names).
- src/typing/expression/pattern_compiler.rs:526+ — `destructure_non_owning_and_maybe_continue`, the function that performs the "aliasing" path.
- src/typing/expression/pattern_compiler.rs:374 — `destructure_owning`, the owned-value counterpart that actually destroys.
- Backend/src/metal/instructions.h:135 — the current unified `Destroy` instruction class.
- Backend/src/region/rcimm/rcimm.cpp:345 — `RCImm::discardOwningRef`, the (currently stubbed) home of the "assert no more refs, then deallocate" logic for imm structs.

## Suggested rewrite
We also destroy shared things — but not via destructuring. Destructuring a shared/immutable value, e.g. `(x, y, z) = Vec3(3, 4, 5)`, does not evaluate to a Destroy; it evaluates to aliasing of the members (see `destructure_non_owning_and_maybe_continue` in src/typing/expression/pattern_compiler.rs). Actually destroying an immutable struct happens elsewhere, when its owning reference is discarded: the current Backend has a single `Destroy` instruction family (Backend/src/metal/instructions.h) used for both owned and immutable structs; for the immutable/RC region the region-specific "assert refcount is now zero, then deallocate" logic belongs in `RCImm::discardOwningRef` (Backend/src/region/rcimm/rcimm.cpp), which as of this writing is an unimplemented stub rather than working code.
