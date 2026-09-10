# Accuracy report: ReferenceMemberLookup Has TargetPermission (RMLHTP)

Audited against the working tree on 2026-09-06. Source: docs/old/Compiler/Templar/Addresses.md:39-62

## Verdict
The section describes a Scala-era `ReferenceMemberLookup` AST node carrying a `targetPermission` field so that accessing a member through a readonly reference yields a readonly member reference. The current Rust typing pass has no permission system at all (`grep -rln "Permission" src/` is empty) and no `targetPermission` field on `MemberLookupTE` (src/typing/ast/expressions.rs:866-876), which instead always wraps the member's kind in a `BorrowRefT` unconditionally, independent of any readonly/readwrite distinction — mutability today is governed by the group-borrowing/aliasing checker (src/typing/borrow_checker/), an entirely different mechanism. `ro`/`rw` were retired at the parser level per docs/convos/convo-28-...md:1212, confirming the mechanism this section describes no longer exists in any form. This finding matches the sibling audit already on file for CSHROOR (docs/old/reports/CanSometimesHaveReadOnlyOwningReferences-CSHROOR-report.md), which independently reached the same conclusion about this same neighboring section and explicitly calls RMLHTP's `targetPermission` field out as absent from the current representation. No code cites RMLHTP (sites: none) and no code embodies the concern (there is nothing analogous to migrate). Recommendation: delete (no live code or doc depends on it; nothing to migrate into docs/arcana).

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | "When we have a readonly reference to a Ship, and we access its engine, it too should be readonly." (a readonly/readwrite permission propagates through member access) | FALSE | src/typing/ast/expressions.rs:866-876; grep -rln "Permission" src/ → no results | There is no readonly/readwrite permission concept in the current typing pass. Mutability safety is enforced by the group-borrowing/aliasing checker (src/typing/borrow_checker/aliasing_info.rs, groupify.rs), not by a permission tag threaded through member lookups. |
| 2 | "we make it so ReferenceMemberLookup has a targetPermission field" | FALSE | src/typing/ast/expressions.rs:866-876 (struct `MemberLookupTE` fields: `range`, `struct_expr`, `member_name`, `result: &BorrowRefT`, `_sealed`) | No `target_permission` (or any permission) field exists on `MemberLookupTE`. The struct's `result` is unconditionally a `BorrowRefT`, decided by RMLRMO's (now-flipped, per docs/old/reports/CanSometimesHaveReadOnlyOwningReferences-CSHROOR-report.md) design, not by a permission computation. |
| 3 | "This would seem inconsistent with RMLRMO, but its concerns don't apply here, since we can't mutate anything readonly." | UNVERIFIABLE / moot | — | Premised entirely on the now-nonexistent permission mechanism from claims 1–2; RMLRMO itself has been independently found to be reversed by the current design (result is always a borrow, never the owned/place type it argued for), so the reconciliation this claim makes has no current referent either way. |

## Stale citation sites
None — `sites` is empty in the record, and `grep -rn -w "RMLHTP" src/ Backend/ docs/` (excluding docs/old/ and docs/convos/) returns nothing. The only remaining occurrences are the definition itself (docs/old/Compiler/Templar/Addresses.md:41,69) and discussion of its obsolescence in docs/convos/convo-28-....md and docs/old/reports/CanSometimesHaveReadOnlyOwningReferences-CSHROOR-report.md.

## Uncited sites that embody the arcana
None found. The permission-propagation mechanism this section describes does not exist in any form in the current code — group-based borrow checking (src/typing/borrow_checker/) is a structurally different design with no single "target permission" concept to attribute to a citation site.

## Suggested text
Not applicable (kind D3, verdict is not major-inaccuracies from a still-relevant idea but full obsolescence of a retired mechanism) — no migration text is warranted; recommend straight deletion of this section, consistent with the existing CSHROOR report's recommendation for its sibling section.
