# Accuracy report: Instantiator Accesses Parts of Coord Generic Args (IAPCGA)

Audited against the working tree on 2026-09-06. Arcana section: docs/arcana/Generics.md:1688 ("# Instantiator Accesses Parts of Coord Generic Args (IAPCGA)")

## Verdict

The section is an unfinished stub, not a completed arcana entry. It contains exactly two sentences: (1) a claim that "a coord has no placeholder itself, it's instead a collection of a placeholder region and a placeholder kind," and (2) the start of a sentence — "This led to a complication in the instantiator, which accesses things like" — that is cut off mid-clause with no continuation, no example, and no description of what the complication actually is or how it's handled. There is nothing here to verify: the load-bearing claim (what the instantiator does, and how) was never written down. Additionally, there is no code anywhere in the repo that cites IAPCGA (confirmed by `grep -rn -w "IAPCGA"` across src/, Backend/, docs/ — the only hit is the header itself), and the "instantiator" concept the section refers to does not exist as a named pass in the current Rust codebase (the two stray uses of the word "Instantiator" in src/typing/edge_compiler.rs:430 and src/postparsing/rules/rules.rs:23 are casual references inside comments about dispatcher/impl-bound machinery, not citations of this arcana or evidence of a distinct "Instantiator" module). This entry documents nothing that can be checked against code and cites nothing; it reads as an abandoned draft.

## Claims

| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "a coord has no placeholder itself, it's instead a collection of a placeholder region and a placeholder kind" | UNVERIFIABLE | No `CoordT`/`Coord` struct found with `region`/`kind` fields matching this description in src/typing/ast/ast.rs or elsewhere; `grep -rn "struct Coord"` under src/typing/ returns nothing definitional | Would need the current Coord representation identified before this claim can be checked |
| 2 | "This led to a complication in the instantiator, which accesses things like ..." | FALSE / incomplete | Sentence is truncated in the doc itself (docs/arcana/Generics.md:1692-1693); no "instantiator" pass exists in src/ as a named module | The sentence needs to be finished, and "instantiator" needs to be mapped to whatever current Rust code performs this monomorphization/lowering step (or the section removed if it was superseded) |

## Stale citation sites

None — no code cites IAPCGA at all.

## Uncited sites that embody the arcana

None found — since the claim itself is incomplete, it's not possible to identify which current code (if any) is the intended referent.

## Suggested rewrite

Not attempted: the source material is too incomplete (a cut-off sentence with no captured detail) to reconstruct what the original author meant to document. This entry should either be finished by whoever wrote it (checking old Scala-era notes/history for the intended continuation) or deleted as dead/abandoned content, since in its current form it makes no checkable claim and is cited nowhere.
