# Accuracy report: We must execute deferreds before a return (MEDBR)

Audited against the working tree on 2026-09-06. Source: docs/old/Compiler/Templar/Templar.md:121-180

## Verdict

This section documents implementation decisions of the Scala-era "Templar" (destructuring/expression-lowering pass) and "FunctionHammer", including a TODO referencing a `__result` variable and an assert in `FunctionHammer`. Neither class exists in the current Rust codebase — the typing pass and instantiator/simplifier have been rewritten from scratch with different architecture, and there is no evidence the described "deferred execution ordering" mechanism (a list of expression kinds explicitly included/excluded from pre-return deferred execution, keyed on borrow-liveness of a temporary) survives in any recognizable form. `grep -rn -w "MEDBR"` finds zero citations anywhere in src/, Backend/, or docs/ outside this file itself — the concern was never carried into the port. The content is an implementation note tied to data structures (`PackE2`, `TupleE2`, `Construct2`, `CheckRefCount2`, `Discard2`, `Block2`) that are Scala AST node names, not current Rust types (confirmed absent via grep below). This is obsolete: the code it describes is gone and the concern has no live descendant to migrate. Recommendation: delete (nothing to migrate; no citing comments exist to clean up).

## Claims

| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | A pass computes "deferreds" (destructors of locals) and executes them before an explicit `return`, to avoid the return skipping cleanup. | UNVERIFIABLE / OBSOLETE | grep below | Describes Scala Templar mechanism; no Scala Templar or matching structure exists in current src/typing. |
| 2 | The scheduling list explicitly excludes "load" expressions (from ref local, addressible local, ref member, addressible member, unknown/known size array) to avoid destroying a temporary while a borrow into it is still live. | UNVERIFIABLE / OBSOLETE | — | Ties to Scala AST node names (`PackE2`, `TupleE2`, `Construct2`, `Discard2`, `Block2`, `CheckRefCount2`) which do not appear in src/ (see grep). |
| 3 | Example: `Wizard(Wand(10)).wand.charges` demonstrates the ordering concern, with `Wand`/`Wizard`/`charges` as sample identifiers. | UNVERIFIABLE | n/a | Syntax (`= Wizard(...)...;` as a bare top-level expr statement, `\>` blockquote artifacts from doc conversion) doesn't match current Vale example style seen elsewhere in docs/arcana; not checked against current parser since concern is unrelated to current pipeline. |
| 4 | "There's an assert for it in FunctionHammer" (guarding against active deferreds at function return). | FALSE (mechanism absent) | `grep -rn "FunctionHammer" src/ Backend/` → no matches | `FunctionHammer` doesn't exist in the current tree; the assert this refers to cannot be located. |

Given zero citations and named types/classes (`Templar`, `FunctionHammer`, `PackE2`, etc.) entirely absent from the current codebase, verdict per the D3 kind's own scale is effectively "obsolete" — the code it describes is gone.

## Stale citation sites
None — `sites: []`, and `grep -rn -w "MEDBR"` across src/, Backend/, docs/ returns only the definition line itself.

## Uncited sites that embody the arcana
None found. The current Rust typing pass has its own drop/destructor-ordering logic (e.g. `src/typing/expression/expression_compiler.rs`), but nothing there cites MEDBR, and confirming whether its ordering logic matches this old Scala-era scheme would require a separate investigation outside this doc's scope — the described mechanism (explicit expression-kind allow/exclude list keyed to borrow safety) is not obviously present as a discrete, nameable unit in the current code, so no confident migration site is proposed.

## Suggested text
Not applicable (D3 kind; only required for major-inaccuracies or obsolete when a corrected replacement is warranted). This note's actual content — "don't destroy a temporary while a live borrow points into it, and destructors must run before an explicit `return` shoots past them" — may still be a real constraint in the current typing pass, but describing it accurately would require investigating current drop/expression-compiler code from scratch rather than translating this Scala-specific text, which is out of scope here. No suggested text is offered; recommend deletion of this section as dead documentation.
