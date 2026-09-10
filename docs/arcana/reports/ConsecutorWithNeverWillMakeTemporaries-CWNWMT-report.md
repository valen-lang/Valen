# Accuracy report: Consecutor With Never Will Make Temporaries (CWNWMT)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/ret-vs-panic-locals.md:143 ("Consecutor With Never Will Make Temporaries (CWNWMT)")

## Verdict
Obsolete. The section is one sentence: "A never is the destroyer of worlds in hammer; it will stop evaluating anything after a never, and skip any instruction that depends on a never." This describes behavior of the old Scala-era "Hammer" simplifying/lowering stage. That stage has been removed from the current pipeline entirely — `src/pass_manager/full_compilation.rs:29-31` says explicitly "the simplifying/hammer stage and its ProgramH ('h) output are gone; HinputsI (in the 'i instantiating arena) is the sole backend contract." The only surviving trace of `ExpressionH`/`ConsecutorH` in the repo is in a leftover integration test file (`src/integration_tests/tests/hammer_tests.rs`), not in any live compiler pass. Never-handling for consecutors today lives in the typing pass (e.g. `src/typing/compiler.rs`, `src/typing/expression/expression_compiler.rs`), and the arcana doesn't describe that.

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | A "hammer" stage exists and, upon seeing a Never, stops evaluating anything after it and skips instructions depending on it. | FALSE/OBSOLETE | src/pass_manager/full_compilation.rs:29-31 ("the simplifying/hammer stage ... are gone"); only remnant is src/integration_tests/tests/hammer_tests.rs | The Hammer stage no longer exists; if this behavior is preserved, it now happens in the typing pass (consecutor/Never handling in src/typing/compiler.rs and src/typing/expression/expression_compiler.rs), not in a separate lowering stage. |
| 2 | Title implies consecutors containing a Never get temporaries made for them ("Will Make Temporaries") — no elaboration given in the visible text. | UNVERIFIABLE | n/a | The section body doesn't actually explain the "temporaries" half of its own title; whatever mechanism it referred to isn't described, and no hammer-stage code remains to check it against. |

## Stale citation sites
None — `grep -rn -w "CWNWMT"` across src/, Backend/, docs/ finds no code or doc citations at all, only the two markdown definitions (docs/arcana/ret-vs-panic-locals.md and the old pre-migration doc). Nothing to be stale in code, but that also means the concept was apparently never actually wired into the ported (Rust) code via a citation.

## Uncited sites that embody the arcana
None found — since the Hammer stage is gone, there is no current code that performs "stop evaluating after a Never" at a Consecutor-lowering step for this concern to attach to. Never-short-circuiting in the typing pass (e.g. consecutor result-type-is-Never-if-any-inner-is-Never, per src/typing/compiler.rs) is a different, already-typing-level concern and isn't a natural site for this specific arcana.

## Suggested rewrite
This section documented a Hammer-stage optimization/invariant that has no analog since Hammer was deleted. Recommend either deleting the section outright, or replacing it with something like:

> Consecutor With Never Will Make Temporaries (CWNWMT)
>
> (Historical/obsolete.) In the old Scala-era Hammer lowering stage, a Never anywhere inside a Consecutor caused the stage to stop evaluating subsequent instructions and skip anything depending on the Never. The Hammer stage has since been removed from the pipeline (see src/pass_manager/full_compilation.rs) — HinputsI, produced by the typing/instantiating passes, is now the sole backend contract. Never-propagation through consecutors (a consecutor's result type is Never if any inner expression is Never) is handled directly in the typing pass; see src/typing/compiler.rs for the current logic. This section is kept for historical context only.
