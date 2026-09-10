# Accuracy report: Need Separate IDrop and Drop Names (NSIDN)

Audited against the working tree on 2026-09-06. Source: docs/old/Compiler/Templar/Templar.md:297-344

## Verdict
This is a historical design note from the Scala Templar era: it identifies the "diamond drop" problem (a struct implementing two interfaces, each with its own implicit `drop`, needs a name that doesn't collide) and proposes a fix — split every type's destructor into a `drop` name (own type's destructor) and a `vdrop` name (interface/impl virtual-dispatch entry point). The problem it identifies is still real and still tested today (`implementing_two_interfaces_causes_no_vdrop_conflict` in src/typing/test/compiler_virtual_tests.rs:143), but the current Rust typing pass solves it without the two-name split the doc proposes: `vdrop` does not exist anywhere in src/ or Backend/ outside that one test's name, and `destructor_compiler.rs` resolves everything through a single `drop` name, dispatching to "abstract drop" per-interface via the interpreted struct/interface kind rather than via a separate vdrop identifier (src/typing/function/destructor_compiler.rs:79-121). So the specific mechanism this note proposes was superseded, though the problem statement remains accurate and the current test suite implicitly documents that it's solved. No code cites NSIDN. Recommendation: delete — the concern it raises is real but resolved differently, has zero live citations, and the proposed drop/vdrop naming scheme was never adopted, so migrating this text as-is would misdescribe the current mechanism.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "We made it automatically generate drop functions for everything." | TRUE | src/typing/function/destructor_compiler.rs:79 `pub fn drop(...)` is called generically for every value going out of scope | — |
| 2 | A struct implementing two interfaces (e.g. IShip and IExplosive) each with an implicit drop causes a naming conflict for the struct's own drop override | DRIFTED | src/typing/test/compiler_virtual_tests.rs:143-172 `implementing_two_interfaces_causes_no_vdrop_conflict` compiles a struct implementing two independent interfaces successfully today | The scenario is exercised and passes; the doc's "problem" is resolved, but not by the mechanism the doc proposes (see claim 3) |
| 3 | Proposed fix: "every type has a 'drop' method" + "every interface and impl have a 'vdrop' method"; interface drop calls abstract vdrop, impl's vdrop calls concrete drop | FALSE (as description of current code) | grep for "vdrop" across src/ and Backend/ finds it only in the test function's *name* (compiler_virtual_tests.rs:143), never as an actual generated function/identifier; destructor_compiler.rs uses a single `drop` name throughout (lines 19-121) | Current dispatch does not use a separate vdrop name; the two-interface conflict is avoided by a different mechanism (single `drop`, resolved per receiving interface/kind context) |

## Stale citation sites
None — record shows zero code citations (`sites: []`), confirmed by `grep -rn -w "NSIDN"` across src/, Backend/, docs/ returning no hits outside the doc itself.

## Uncited sites that embody the arcana
- src/typing/test/compiler_virtual_tests.rs:143 — `implementing_two_interfaces_causes_no_vdrop_conflict`, the live regression test for exactly the diamond-drop scenario this note describes (test name still says "vdrop" even though the mechanism doesn't use that name — mildly misleading test name, worth a note but not a citation site to add).
- src/typing/function/destructor_compiler.rs:79-121 — current single-`drop`-name resolution that replaced the proposed drop/vdrop split.

## Suggested text
Not included — verdict is obsolete/delete, and the doc is a D3 (old file, no citations) rather than a G or F kind, so the template only requires suggested text for D3 when major-inaccuracies or obsolete. Given the recommendation is deletion (the proposed mechanism isn't what's implemented), no replacement arcana text is offered; if the maintainer instead wants to preserve the underlying diamond-drop problem as documentation, it should be rewritten from scratch describing the *current* single-name resolution in destructor_compiler.rs rather than adapted from this text.
