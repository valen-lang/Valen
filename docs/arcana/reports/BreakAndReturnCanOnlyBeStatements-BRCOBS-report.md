# Accuracy report: Break and Return Can Only Be Statements (BRCOBS)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/ret-vs-panic-locals.md:67 ("# Break and Return Can Only Be Statements (BRCOBS)")

## Verdict
Mostly a Scala-era design musing (open questions about Midas/Hammer/Templar, most of which are historical/exploratory and unverifiable against current Rust code), but its one hard, checkable claim — "break and return can only be statements" because of the `a(b(), break)` limbo-drop problem — is still true and actively enforced by the parser today (`ParseError::CantUseBreakInExpression` / `CantUseReturnInExpression`), with tests confirming it. The only problem is that one of the two cited sites is dead, fully commented-out code, not live enforcement, so it no longer supports the claim as evidence.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "Break and return can only be statements" (can't appear inside another expression, e.g. `a(b(), break)`) | TRUE | src/parsing/tests/expression_tests.rs:1091-1099 (`detect_break_in_expr`/`detect_return_in_expr` assert `ParseError::CantUseBreakInExpression`/`CantUseReturnInExpression` for `a(b, break)` / `a(b, return)`) | — |
| 2 | This is checked "occasionally" to make sure no breaks/returns appear inside other expressions (except consecutors owned by blocks) | TRUE (still accurate at a coarse level) | src/parsing/tests/expression_tests.rs:1090-1100; src/testvm/expression_vivem.rs:510 (vivem still panics if an ExternFunctionCall arg evaluates to Break/Return) | — |
| 3 (implicit, via cited site) | expression_compiler.rs:1621 shows live typing-pass logic relying on "we can't have a BreakTE inside a FunctionCallTE, see BRCOBS" | FALSE as a live-code citation | src/typing/expression/expression_compiler.rs:1560-1630 — the entire surrounding block (including line 1621) is commented-out dead code (a never-finished While-loop desugaring), not compiled or executed | The comment is real but the code it's attached to is inert; it documents historical intent, not current enforcement. |
| 4 | Everything else in the section (UnreachableMoot, Midas/Hammer/Templar/Stackify discussion, "Consecutor With Never Will Make Temporaries" musing at the end) | UNVERIFIABLE / not-checkable | — | These are open questions and Scala-era terminology (Midas, Hammer, Templar, Stackify) with no 1:1 current-code claim to verify; treat as historical musing, not asserted fact. |

## Stale citation sites
- src/typing/expression/expression_compiler.rs:1621 — the section (via the code comment) implies this is a live typing-pass site enforcing "no BreakTE inside a FunctionCallTE"; in fact this whole function body is commented out (dead code for a never-completed While-loop lowering), so it enforces nothing today. The real, live enforcement lives in the parser (src/parsing, `ParseError::CantUseBreakInExpression`/`CantUseReturnInExpression`), which is uncited by the arcana doc.

## Uncited sites that embody the arcana
- src/parsing/tests/expression_tests.rs:1091-1099 — `detect_break_in_expr`/`detect_return_in_expr`, the actual regression tests for the rule (do cite BRCOBS in a comment, but the arcana doc's own citation list doesn't point back at the parser as the enforcement site).
- src/testvm/expression_vivem.rs:510 — vivem-level assertion that Break/Return can't leak out of an ExternFunctionCall arg (cites BRCOBS in the panic message).

## Suggested rewrite
Not written — verdict is minor-inaccuracies, not major-inaccuracies/obsolete. Recommend only updating the arcana's implicit pointer: note that the enforcement now lives in the parser (`ParseError::CantUseBreakInExpression`/`CantUseReturnInExpression`, src/parsing/tests/expression_tests.rs) rather than in the (now dead/commented) typing-pass code at expression_compiler.rs:1621.
