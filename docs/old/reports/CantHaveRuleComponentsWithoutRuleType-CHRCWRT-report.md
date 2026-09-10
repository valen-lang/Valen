# Accuracy report: Can't Have Rule Components Without Rule Type (CHRCWRT)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Parser_Scout.md:558 ("Can't Have Rule Components Without Rule Type")

## Verdict
This is a Scala-era design Q&A about template-rule syntax: whether `Kind#K[#M]` / `Ref[#O, #P, #L, #K]` bracket "components" require an explicit type name (`Kind`, `Ref`, or `Callable`) before them, concluding yes, require it. The Rust-ported rule grammar (src/parsing/ast/rules.rs) no longer has this bracket-component syntax at all — no `IRulexPR` variant models `Name[components...]`, there is no `Callable[...]` construct, and `ITypePR` is a small fixed enum (IntType, BoolType, CoordListType, RegionType, CitizenTemplateType) unrelated to `Kind`/`Ref`/`Callable`. Rule types are instead written as concrete templex forms (BorrowRef, WeakRef, OwnRef, etc. in src/parsing/ast/templex.rs) rather than as a bracketed-component mini-language. The syntax question this section answers no longer exists in the current grammar, so its resolution is moot. Recommendation: leave in docs/old (obsolete, not worth migrating).

## Claims
| # | Claim | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Rule syntax lets you write `Kind#K[#M]` or `#R: Ref[#O, #P, #L, #K]` with bracketed "components" | FALSE (obsolete) | src/parsing/ast/rules.rs:6-52 (no bracket-component variant); src/parsing/ast/templex.rs (concrete Ref/BorrowRef/OwnRef nodes, not bracket-component syntax) | This bracket-component grammar was not carried into the Rust port. |
| 2 | A subtype of `Kind` called `Callable` exists, taking arg types and a return type: `#K: Callable[#M, [#A,#B,#C], [#R]]` | FALSE (obsolete) | grep for `Callable[` and `Callable` type across src/ finds no such construct; only unrelated `is_callable` keyword and test names | No `Callable` rule-type subtype exists in the current typing/parsing code. |
| 3 | (Resolution) full type name (`Kind`/`Ref`/etc.) must be required before bracketed components, rather than inferring `Kind` | UNVERIFIABLE / moot | n/a | Since the bracket-component syntax itself doesn't exist, this resolved design decision has no corresponding code to check. |

## Stale citation sites
None — `sites` list was empty and grep for CHRCWRT across src/, Backend/, docs/ (excluding docs/old) found no citations.

## Uncited sites that embody the arcana
None found — the described bracket-component rule syntax has no analog in the current grammar to cite from.

## Suggested rewrite
Not applicable in useful form; the mechanism this section discusses (bracketed rule components under a type name like `Kind`/`Ref`/`Callable`) doesn't exist in the current rule grammar, so there's nothing accurate to rewrite. Leave as historical record in docs/old.
