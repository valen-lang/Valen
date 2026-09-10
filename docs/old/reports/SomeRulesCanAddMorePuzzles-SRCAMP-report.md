# Accuracy report: Some Rules Can Add More Puzzles (SRCAMP)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Compiler/Templar/Infer Templar.md:424 ("# Some Rules Can Add More Puzzles (SRCAMP)")

## Verdict
Obsolete. The section documents "Approach A": when the impl rule (CoordSendSR) can't yet decide whether its result is the sub or super side, the solver adds a brand-new rule/puzzle (CallSiteCoordIsa / DefinitionCoordIsa) mid-solve rather than pre-planning rule order. In the current Rust solver every rule variant this mechanism depends on — `CoordSend`, `CallSiteCoordIsa`, `DefinitionCoordIsa` — is commented out dead code in src/typing/infer/compiler_solver.rs, and the sole live citation sits inside that commented-out block. Per docs/convos/convo-84-verify-127-expected-test-failures-700ac0d1-b9cf-4264-a7ae-b2528e50bafc.md:3177, the whole reason for this dynamic mid-solve rule generation was removed: upcast resolution was moved to a "§2A" phase before solving even starts, so the solver no longer needs to self-replace rules during solving. The core mechanism the section describes no longer exists in the live compiler.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | The impl rule (CoordSendSR) doesn't know if its receiver rune alone is enough to solve. | DRIFTED/OBSOLETE | src/typing/infer/compiler_solver.rs:836 (`// IRulexSR::CoordSend(coord_send) => {`) | `CoordSend` is entirely commented out; impl-rule resolution has moved out of the solver (upcast handled pre-solve, per convo-84). |
| 2 | Approach A: when the puzzle solves, add a new rule with a new puzzle (e.g. CallSiteCoordIsa/DefinitionCoordIsa), rather than conditionally solving multiple runes in one rule. | FALSE (as description of current code) | src/typing/infer/compiler_solver.rs:704, 765, 846-878 (`// IRulexSR::CallSiteCoordIsa`, `// IRulexSR::DefinitionCoordIsa`, `// let new_rule = IRulexSR::CallSiteCoordIsa(...)`) | All three rule variants and the rule-adding call `commit_step(..., vec![new_rule], ...)` are dead, commented-out code — not exercised by the live solver at all. |
| 3 | "Both of these kind of mean we won't be planning any rule execution order beforehand." | FALSE (as a statement about the live compiler) | docs/convos/convo-84-verify-127-expected-test-failures-700ac0d1-b9cf-4264-a7ae-b2528e50bafc.md:3177 | Confirmed obsolete in-repo: upcasts were moved to a pre-solve phase ("§2A") specifically so rule execution order *can* be statically planned — the opposite of what this claim says is necessary. |

## Stale citation sites
src/typing/infer/compiler_solver.rs:837 — `// See IRFU and SRCAMP for what's going on here.` sits inside the fully commented-out `CoordSend` match arm (lines 836-951 are all `//`-prefixed). The code this comment annotates does not run; the citation points at dead code, and the doc it cites describes a mechanism (self-replacing rules for impl resolution) that convo-84 confirms was deliberately eliminated. Not load-bearing for a reader of live code — there is no live code left to read at this site — but it is actively misleading if re-enabled without noticing it documents an abandoned design.

## Uncited sites that embody the arcana
None found. The mechanism (mid-solve rule generation for impl/upcast resolution) is not present anywhere live in src/typing/infer/compiler_solver.rs — searched all `IRulexSR::` match arms (lines 158-1324) and the only variants that add new rules via `commit_step` with a non-empty new-rules vec are inside the commented-out `CoordSend` block itself.

## Suggested rewrite
This section (and its sibling IRFU/SAIRFU) should be marked as historical/superseded rather than corrected in place, since the mechanism it describes is gone, not renamed. Suggested note to prepend to the section:

> **Superseded.** This "add a new rule/puzzle mid-solve" approach (Approach A) applied to the Scala-era impl/upcast rule (`CoordSendSR`/`CallSiteCoordIsaSR`/`DefinitionCoordIsaSR`). In the current Rust solver (src/typing/infer/compiler_solver.rs) these rule variants are dead code — upcast resolution now happens in a phase before constraint solving begins, so the solver no longer needs to dynamically inject new rules to decide sub/super direction, and rule execution order can be planned statically. Kept for history; do not cite this ID from live code.
