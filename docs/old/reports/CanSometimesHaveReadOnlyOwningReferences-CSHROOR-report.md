# Accuracy report: Can Sometimes Have Read-Only Owning References (CSHROOR)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Compiler/Templar/Addresses.md:63 ("## Can Sometimes Have Read-Only Owning References")

## Verdict
The section's premise is gone. It says that when RMLHTP/ReferenceMemberLookup loads `ship.engine` from a `&Ship`, the result is "a read-only reference, but one that is still an owning one" — described as "the only known case of an owning read-only reference." In the current Rust typing pass, `MemberLookupTE::new` (src/typing/ast/expressions.rs:873-891) unconditionally wraps the member's kind in a `BorrowRefT` — the result is *always* a borrow reference, never an owning one, in every context. There is also no readonly/mutable "permission" field on lookup results any more (the doc's own earlier paragraph mentions a `targetPermission` field feeding this case) — permissions of that Scala-era shape do not appear on `MemberLookupTE`/`BorrowRefT` in src/typing at all. The scenario this section exists to name (an owning reference that happens to be read-only) cannot occur in the current representation, so both the mechanism and the example are obsolete. This section's sibling, RMLRMO (cited by name in the section body), was independently audited and found to have flipped: the doc RMLRMO cites for justification argues member-lookup should yield the owned type; current code makes it a borrow unconditionally — the opposite. Recommendation: delete (fold into the RMLRMO rewrite's explanation if any note about "borrow, never owning" is wanted).

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | Loading `.engine` off a `&Ship` via ReferenceMemberLookup can produce a reference that is read-only but still "owning" | FALSE | src/typing/ast/expressions.rs:873-891 (`MemberLookupTE::new` always sets `result = interner.alloc(BorrowRefT { inner: member_kind })`) | The result is always a `&'t BorrowRefT`, i.e. always a borrow, never an owning reference, regardless of the containing struct's permission. |
| 2 | "This is the only known case of an owning read-only reference" | OBSOLETE | src/typing/ast/expressions.rs:823, :873 (both member/array lookups use `BorrowRefT` unconditionally) | No such case exists in the current representation; there is nothing to be "the only known case" of. |
| 3 | (implicit, from preceding paragraph) a `targetPermission` field on `ReferenceMemberLookup` drives whether the result is read-only | UNVERIFIABLE / likely FALSE for current code | grep for `permission`/`targetPermission` in src/typing/ast/expressions.rs returns nothing near `MemberLookupTE` | Current `MemberLookupTE` and `BorrowRefT` carry no permission field; whatever mutability/sharedness tracking exists lives elsewhere (see instantiator-design.md's `SharednessI`), not as a per-lookup read-only flag. |

## Stale citation sites
None — CSHROOR has no live citations in src/, Backend/, or docs/ outside docs/old/ (grep -rn -w "CSHROOR" across src/ Backend/ docs/, excluding target/tmp/guardian-logs/Guardian/Luz/docs/convos, returns nothing).

## Uncited sites that embody the arcana
None found — because the scenario no longer occurs, there is no current code that embodies "an owning read-only reference"; `MemberLookupTE::new` (src/typing/ast/expressions.rs:873-891) is the relevant site but it embodies the *opposite* fact (always-borrow), so it is better read as evidence against this section than as an uncited instance of it.
