# Accuracy report: ReferenceMemberLookup Results In Member's Ownership (RMLRMO)

Audited against the working tree on 2026-09-06. Arcana section: docs/old/Compiler/Templar/Addresses.md:2 ("## ReferenceMemberLookup Results In Member's Ownership (RMLRMO)")

## Verdict
The core mechanism has flipped. The doc argues that `ship.engine` (a member lookup) must produce the member's *own* type (`Engine`, not `&Engine`), because `set ship.engine = Engine(15)` needs to assign an `Engine` into the destination, and a separate `SoftLoad` node is added around the lookup for read contexts to turn it into `&Engine`. In the current Rust typing pass, `MemberLookupTE::new` (src/typing/ast/expressions.rs:873) always constructs `result` as `&'t BorrowRefT` — i.e. the lookup itself *is* a borrow reference to the member, in every context, read or write. There is no `SoftLoad` node any more (every `SoftLoad`/`SoftLoadTE` reference in src/typing is inside commented-out dead code in expression_compiler.rs and local_helper.rs). The assignment problem the doc worried about is instead solved on the *mutate* side: `MutateTE::new` (src/typing/ast/expressions.rs:391-411) unwraps the destination's `BorrowRefT` itself (`destination_inner_type`) and asserts the source's type equals that inner type. So the "assigning an Engine into an &Engine destination" problem is real and still handled, but by the opposite mechanism from what the doc prescribes. The two live citations at expressions.rs:823 and :873 ("See RMLRMO why the result is a borrow reference to the member") describe the current, correct fact (the result *is* a borrow reference) but point at a doc whose actual argument is that the result should *not* directly be a borrow reference — a reader who opens RMLRMO to understand why `result` is `&'t BorrowRefT` will read an argument for the opposite design and come away confused. This is a major inaccuracy: the core claim ("ReferenceMemberLookup should result in the member's type... an Engine in both cases") is false against current code, and the citing comments' pointer to it is actively misleading rather than merely stale wording.

## Claims
| # | Claim (quoted or closely paraphrased) | Status | Evidence (file:line) | Correction |
|---|---|---|---|---|
| 1 | `ship.engine` results in a `&Engine` in a read context | TRUE (as an observed type) | src/typing/ast/expressions.rs:865-885 (`MemberLookupTE.result: &'t BorrowRefT`) | — |
| 2 | Naively, one might think ReferenceMemberLookup always results in `&Engine` | UNVERIFIABLE (rhetorical framing, not a code claim) | — | — |
| 3 | `set ship.engine = Engine(15)` requires assigning an `Engine` into a `&Engine`-typed destination — a problem to solve | TRUE, still a real constraint | src/typing/ast/expressions.rs:399-410 | — |
| 4 | **"So instead, ReferenceMemberLookup should result in the member's type, an Engine in both cases."** | FALSE | src/typing/ast/expressions.rs:865-885 (`MemberLookupTE::new` always builds `result = interner.alloc(BorrowRefT { inner: member_kind })`, never the bare `member_kind`) | Current code does the opposite: the lookup's result is always the borrow-wrapped type; the destination-vs-source problem is resolved by `MutateTE::new` unwrapping the destination's `BorrowRefT` (src/typing/ast/expressions.rs:399-405), not by having the lookup itself carry the unwrapped type. |
| 5 | "We add a **SoftLoad**... around the ReferenceMemberLookup. It will turn the Engine into a &Engine" | FALSE / obsolete mechanism | No live `SoftLoadTE` construction site in src/typing/ast or src/typing/expression; every hit is commented out (src/typing/expression/expression_compiler.rs:343-344, src/typing/expression/local_helper.rs:152-246) | `SoftLoad` is dead code in this branch of the port; there is no node that promotes an owned member-lookup result to a borrow — the borrow is baked into `MemberLookupTE` from construction. |

## Stale citation sites
- src/typing/ast/expressions.rs:823 — comment on `RuntimeSizedArrayLookupTE.result`: "See RMLRMO why the result is a borrow reference to the element type." The field is indeed `&'t BorrowRefT` (true fact), but RMLRMO's actual argument concludes the *opposite* (that lookups should yield the owned type, with a `SoftLoad` producing the borrow separately). A reader following the citation is told to read a justification for a design the code does not use.
- src/typing/ast/expressions.rs:873 — same issue, on `MemberLookupTE.result`, the exact struct/example the doc uses (`ship.engine` / `Engine`/`Spaceship`). This is the more load-bearing of the two: without a correct doc, a reader has no source explaining why the field is unconditionally a borrow ref *and* how mutation destinations are unwrapped instead.
- docs/architecture/instantiator-design.md:663 — RMLRMO appears only in a bare list of "various spot citations" acronyms with no elaboration; not load-bearing, but inherits the same staleness if a reader chases it down.

## Uncited sites that embody the arcana
- src/typing/ast/expressions.rs:391-411 (`MutateTE::new`) — the actual current mechanism for the "assigning owned into borrow destination" problem the doc raises; doesn't cite RMLRMO at all.
- src/typing/ast/expressions.rs:849-860 (`StaticSizedArrayLookupTE::new`) — same borrow-wrapping pattern as `MemberLookupTE`, uncited.
- src/typing/test/compiler_mutate_tests.rs:78-135 (`test_mutating_a_local_var`-adjacent test using the doc's exact `Engine`/`Spaceship`/`set ship.engine = Engine(15)` example) — asserts `lookup.result.inner` is `KindT::Struct(_)`, i.e. confirms current behavior directly contradicts the doc's prescribed fix, with a comment "The lookup is a borrow of the member, so the struct is what it points at."

## Suggested rewrite
The example and the underlying constraint (assigning an owned value into a member whose lookup is typed as a borrow) are still real and worth keeping; the prescribed fix is what changed.

> ## ReferenceMemberLookup Results In A Borrow Of The Member's Type (RMLRMO)
>
> When we do:
> ```
> struct Engine { fuel int; }
> struct Spaceship { engine Engine; }
> fn main() {
>   ship = Spaceship(Engine(10));
>   println(ship.engine);
> }
> ```
> `ship.engine` results in a `&Engine`. `MemberLookupTE`'s `result` field is unconditionally a `BorrowRefT` wrapping the member's kind (built in `MemberLookupTE::new`) — there is no separate node that promotes an owned member type into a borrow; the borrow is baked in at construction.
>
> This raises a question for:
> ```
> struct Engine { fuel int; }
> struct Spaceship { engine Engine; }
> fn main() {
>   ship = Spaceship(Engine(10));
>   set ship.engine = Engine(15);
> }
> ```
> Here we're assigning an `Engine` into a destination expression whose type is `&Engine`. Rather than having the lookup itself vary its result type by context (owned for writes, borrowed for reads, historically bridged by a `SoftLoad` node — since removed), the current typing pass resolves this on the mutate side: `MutateTE::new` requires its `destination_expr`'s result to be a `BorrowRefT`, unwraps that to get the inner type, and asserts the `source_expr`'s type equals the unwrapped inner type. So `MemberLookupTE`, `StaticSizedArrayLookupTE`, and `RuntimeSizedArrayLookupTE` all uniformly produce borrow-typed results, and `MutateTE` is the one place that peels the borrow off to type-check the assignment.
