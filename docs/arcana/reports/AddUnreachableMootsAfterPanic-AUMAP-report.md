# Accuracy report: Add Unreachable Moots After Panic (AUMAP)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/ret-vs-panic-locals.md:1 ("# Add Unreachable Moots After Panic (AUMAP)")

## Verdict
Obsolete. The section is a verbatim carry-over of a Scala-era design note (a near-duplicate of docs/old/"Ret vs Panic, Locals (already migrated).md") built entirely around backend/type-system machinery that no longer exists under these names: `UnreachableMoot`, the `Hammer` codegen pass, `Midas`, `Stackify`, and "Templar". None of these identifiers appear anywhere in `src/` (grep across the whole tree returns zero hits for `UnreachableMoot`, and `Hammer`/`Midas` survive only as unrelated file/test names — `midas.rs` is just the current backend-driver binary, not the old pass). The current typing pass represents unreachability via `KindT::Never`/`NeverT` (src/typing/expression/expression_compiler.rs) with no analog of an explicit "UnreachableMoot(x)" instruction that "prints it out von-style and halts the program." The doc's own citing site (docs/architecture/instantiator-design.md:663) is not a real usage — it's a bare name-drop in a long list of IDs ("various spot citations... see grep") with no discussion of the mechanism. The doc is effectively dead design history that was never rewritten for the Rust port.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "UnreachableMoot is a wrapper instruction to signal to Hammer that something won't actually ever be run." | FALSE/OBSOLETE | `grep -rn "UnreachableMoot" src/` → no results | No such instruction exists in the current pipeline; unreachability is expressed via `KindT::Never` in the typing AST, not a dedicated moot instruction. |
| 2 | "We have UnreachableMoots after return, break, and panic." | UNVERIFIABLE/OBSOLETE | same as above | Can't verify a mechanism that isn't present. |
| 3 | "It won't make a difference to Midas." (Midas = old backend/codegen pass) | OBSOLETE | `grep -rln "Midas" src/` → only `src/bin/valec/midas.rs` (current backend-driver binary name, unrelated concept) | Terminology doesn't map to current architecture (typing → instantiating → LLVM backend). |
| 4 | Decision: "We'll have a special UnreachableMoot(x) which takes an argument by move, prints it out von-style, and halts the program." | FALSE/OBSOLETE | `grep -rn "UnreachableMoot\|von-style" src/` → none | Not implemented under this name; how (or whether) panics currently thread moved locals through codegen is undocumented by this section. |
| 5 | "In the case of if-statement where one branch returns Never(), it will take all the variables that were moved by the other branch, and implicitly call that panic() on them..." | UNVERIFIABLE | src/typing/expression/expression_compiler.rs:1282-1287 shows `KindT::Never` handling in if-expression result-type unification, but no code implementing "implicitly call panic() on the other branch's moved variables" was found | The current `Never` handling appears to be about result-type unification only, not the moot-injection scheme described. |
| 6 | Example programs (`fn main() { m = Marine(7); ret 6; }` etc.) | UNVERIFIABLE (syntax plausible but not confirmed against current parser) | — | Illustrative only; not load-bearing for the arcana's core claims. |

## Stale citation sites
docs/architecture/instantiator-design.md:663 — AUMAP is listed only as one of ~20 IDs in a catch-all "various spot citations" sentence with no surrounding discussion of unreachable-moot machinery; the line doesn't actually invoke or describe the mechanism, so there's nothing there to check against current behavior — the citation itself is vestigial.

## Uncited sites that embody the arcana
None found — no code implements `UnreachableMoot`, so there is nothing in the current tree that embodies this specific design.

## Suggested rewrite
The whole section describes now-defunct Scala-era machinery (Hammer/Midas/Stackify/UnreachableMoot) that was never carried into the Rust port under these names. Rather than patch individual claims, this arcana should either be:
- Retired/archived (moved fully into docs/old/, since a near-duplicate, "Ret vs Panic, Locals (already migrated).md", already lives there), with the AUMAP ID and its citation removed from docs/architecture/instantiator-design.md:663; or
- Replaced with a fresh section describing how the current typing pass (`KindT::Never`/`NeverT` in src/typing/expression/expression_compiler.rs) and the instantiating/backend passes actually handle locals moved before an unreachable path (panic/return/break), if that behavior is still considered worth documenting under the AUMAP id.
