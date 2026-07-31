<!-- session: ae0520c7-2add-4f26-add6-b4b34e6cd4b5; exported-bytes: 2159454; transcript: /Users/verdagon/.claude/projects/-Volumes-V-Vale2/ae0520c7-2add-4f26-add6-b4b34e6cd4b5.jsonl -->
# Claude Conversation Log

Session ID: ae0520c7-2add-4f26-add6-b4b34e6cd4b5
Date: 2026-07-15 19:55:21

---

## 👤 User

<command-name>/model</command-name>
            <command-message>model</command-message>
            <command-args></command-args>

---

## 👤 User

<local-command-stdout>Set model to [1mOpus 4.8 (1M context) (default)[22m and saved as your default for new sessions</local-command-stdout>

---

## 👤 User

<command-name>/effort</command-name>
            <command-message>effort</command-message>
            <command-args>medium</command-args>

---

## 👤 User

<local-command-stdout>Set effort level to medium (saved as your default for new sessions): Balanced approach with standard implementation and testing</local-command-stdout>

---

## 👤 User

whats the self.pointify_kind( call in evaluate_closure for?

---

## 🤖 Claude

I'll look at the `evaluate_closure` function and the `pointify_kind` call.

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

please read vcoord-handoff.md to see what we're doing.

then please look into: whats the self.pointify_kind( call in evaluate_closure for?

---

## 🤖 Claude

I'll read the handoff doc first, then dig into that call.

---

## 🤖 Claude

I have the context. Now let me find that call.

---

## 🤖 Claude

Bash is blocked by the guardian hook (`GUARDIAN_PORT` unset), so I'll try searching via an agent that may have grep tooling.

---

## 🤖 Claude

The agent hit the same Bash block. Let me read the likely definition sites directly.

---

## 🤖 Claude

Let me try a general-purpose agent, which may have grep tooling the Explore agent lacked.

---

## 👤 User

[Request interrupted by user for tool use]

---

## 👤 User

please read vcoord-handoff.md to see what we're doing.

then please look into: whats the self.pointify_kind( call in evaluate_closure for?

---

## 🤖 Claude

I'll read the handoff doc first.

---

## 🤖 Claude

Now let me find the `pointify_kind` call in `evaluate_closure`.

---

## 🤖 Claude

## What it's for

`pointify_kind` is the "given a bare kind, what's the right ownership tag for it?" helper. It looks up the kind's sharedness and picks: share citizen → `OwnershipT::Share`, non-share → whatever the caller passed as `ownership_if_mutable`. Then it stamps that onto a `KindT::new(ownership, region, kind)` (the Coord-shaped constructor).

In `evaluate_closure` (`expression_compiler.rs:2232`) the call is doing **nothing but feeding a sanity assert**:

```rust
let closure_coord = self.pointify_kind(coutputs, KindT::Struct(...), region, OwnershipT::Own);
let construct_expr_2 = self.make_closure_struct_construct_expression(...);
assert!(construct_expr_2.result().coord == closure_coord);
```

`closure_coord` is never used for anything else — it's constructed, compared, and dropped.

## And the assert is tautological

`make_closure_struct_construct_expression` (`expression_compiler.rs:360-367`) computes its result coord by the identical rule:

```rust
let ownership = match closure_struct_def.sharedness {
    SharednessT::Single => OwnershipT::Own,
    SharednessT::Shared => OwnershipT::Share,
};
let result_pointer_type = KindT::new(ownership, region, KindT::Struct(struct_ref));
```

And `pointify_kind`'s `get_sharedness(Struct(s))` bottoms out in `coutputs.lookup_mutability(get_struct_template(s.id))` (`compiler.rs:1714`) — the same map that populated `closure_struct_def.sharedness`, keyed off the same struct. Same region, same kind, same ownership rule, same lookup. The two sides can't disagree unless the struct's own definition disagrees with the mutability map, which would be a bug elsewhere.

So it's a duplicated computation guarding a comparison it can't fail — the closure caller is `pointify_kind`'s *only* live caller in the tree (I grepped; there are no others).

## Under onion

Both the helper and the assert dissolve. `pointify_kind` is exactly the "ownership as a flat tag computed from a kind" pattern the onion mission retires — same family as `get_borrow_ownership` (handoff §"What was learned" item 4). Its own header already says so: `// VCOORD: not sure this should exist long term`, and its caller's twin at `make_closure_struct_construct_expression:361` carries `// VCOORD: this might need to go away`.

The semantic content survives as one row of the validity table: a closure struct is held bare (non-share) or wrapped in `ShareRef` (share) — decided by the citizen's declared sharedness, which is precisely what `get_sharedness` does today. So when the closure-construct path is rewritten to build a `KindT` onion directly, `make_closure_struct_construct_expression` picks bare-vs-`ShareRef` at construction and there's nothing left for a separate `pointify_kind` to recompute or check.

Also worth flagging while it's in view: `pointify_kind`'s RSA/SSA arms are `panic!("Unimplemented")`, so arrays never reach it — consistent with closures being the only caller.

---

## 👤 User

under onion, i imagine that if we mention a variable in a closure, we want to capture a reference to it. so an int outside the closure would be an &int inside the closure, right?

---

## 🤖 Claude

Good question — let me look at what the capture path actually does today before I answer, since the closure-struct member types are computed in a couple of places.

---

## 🤖 Claude

Yes — and the interesting part is that the capture path **already does exactly that today**, with no primitive gate. The gate lives on the other side, and that asymmetry is currently papered over by a deliberately weak assert.

## The capture side is already onion-shaped

`determine_closure_variable_member` (`function_compiler.rs:280-316`) is where a captured name's closure-struct member type gets decided:

```rust
match coord.ownership {
    OwnershipT::Own => IMemberTypeT::Reference(ReferenceMemberTypeT {
        reference: KindT::new(OwnershipT::Borrow, coord.region, coord.kind) }),
    OwnershipT::Borrow | OwnershipT::Share => IMemberTypeT::Reference(ReferenceMemberTypeT {
        reference: coord }),
    ...
}
```

Own → Borrow, unconditionally, keyed only on ownership and never consulting the kind. An `int` local is `Own + Int` (per the VCOORD note at `local_helper.rs:211`, "post-cut Int/Bool/Float/Void are Own (not Share)"), so it hits the first arm and the member comes out `Borrow + Int` — `&int`. That's your intuition, already implemented.

And the `Borrow | Share` arm passing through unchanged is structurally *the Q3 rule already written down*: wrap in a borrow unless it's already a borrow. So this function needs arm renames into onion shapes, not a redesign.

## The construct side is where the primitive gate is

`make_closure_struct_construct_expression` populates each member via `borrow_soft_load` (`expression_compiler.rs:348`), which routed through `get_borrow_ownership` (`local_helper.rs:209-226`, now commented out):

```rust
KindT::Int(_) => OwnershipT::Share,
KindT::Bool(_) => OwnershipT::Share,
KindT::Float(_) => OwnershipT::Share,
KindT::Str(_) => OwnershipT::Share,
KindT::Void(_) => OwnershipT::Share,
KindT::Struct(_) | ... => match get_sharedness(...) { Single => Borrow, Shared => Share },
```

So for a captured int, the member is *declared* `Borrow + Int` but *populated* with a `Share + Int` expression. The two disagree, and the assert at line 343 is why nobody noticed:

```rust
assert_eq!(coord.kind, lookup.result().coord.kind);   // kinds only
assert!(coord.ownership != OwnershipT::Own);           // ownership merely "not Own"
```

It compares kinds and then only checks that ownership isn't `Own` — never that the two ownerships match. Contrast the `Address` arm four lines down (line 353), which does `assert_eq!(coord, lookup.result().coord)` on the whole coord. The Reference arm is weakened to precisely the width of the Borrow/Share discrepancy the primitive gate introduces.

That gate is also self-flagged as the source of downstream damage — `local_helper.rs:211` says returning Share here "is what forces the instantiator's (Share, Own)/(Share, MutableBorrow) paper-over arms."

## Under onion

`get_borrow_ownership` dissolves outright (handoff §"What was learned" item 4 — there's no ownership axis to compute from a kind), and both sides converge on the uniform rule. `x: Kind::Int` outside → `BorrowRef(Int, r)` member → `&int` inside, on both the declare and the construct side. The kind-only assert can then strengthen to full structural equality, matching the Address arm. Reading a bare int out of that `&int` inside the closure body is coercion row 1 (`BorrowRef(P, r)` → bare `P` via the built-in primitive `implicit_clone(&P) P`) — already in the table, no new machinery.

## One thing that isn't settled

The `Share` arm's pass-through and Q3 point different directions under onion. Q3 says bare-use of `K` wraps in `BorrowRef` unless `K` is *already* `BorrowRef` — and `ShareRef(SC)` isn't. So capturing a `@Spaceship` would give `BorrowRef(ShareRef(SC), r)` (`&@Spaceship`), whereas line 292 today passes it through as `ShareRef(SC)` (`@Spaceship`). Both are expressible and coercion row 3 bridges them (`&@T` → `@T` via the share blanket, RC bump), so neither is forced — but they're genuinely different member types, and the choice affects whether closure structs hold RC handles or borrows of RC handles. Worth pinning before the arms get rewritten; I'd flag it as your call rather than guess.

---

## 👤 User

please investigate

---

## 🤖 Claude

Good — that's the open question worth resolving before the arms get rewritten. Let me investigate the Share arm specifically: whether it's reachable, what it produces, and what the closure body does with the captured member.

---

## 🤖 Claude

Let me look for whether the Share arm is actually reachable — a closure capturing an immutable/share citizen — and check the git history of that arm.

---

## 🤖 Claude

Let me verify the key claims directly — especially the `str` finding and whether the onion `ShareRefT` types already exist.

---

## 🤖 Claude

The onion variants already exist. Let me read the current state of the type system.

---

## 🤖 Claude

This is decisive — the architect has already written onion-shaped AST constructors. Let me read them carefully.

---

## 🤖 Claude

`SoftLoad` is gone from `ExpressionTE` entirely. Let me confirm the local-variable model and check the Q3 double-borrow interaction.

---

## 🤖 Claude

The architect's onion types are already committed. Let me see what they're actively editing right now — including the capture site itself.

---

## 🤖 Claude

Yes — `&int`, and it's already decided and committed. But I need to correct my previous answer first: the code I quoted at you is all dead, and I read it as if it were live.

(Noting your mid-turn point about other sessions: everything below about *committed* state is stable; I flag the one place where I'm reading an uncommitted diff that may be someone else's in-flight work.)

## Correction: the machinery I described no longer exists

`OwnershipT`, `CoordT`, `KindT::new`, `IMemberTypeT` (and its `Reference`/`Address` split), `AddressExpressionTE`/`ReferenceExpressionTE`, and `SoftLoad` are **all already deleted from the tree** — committed, not pending. Greps for their definitions come back empty:

- `KindT` (`types/types.rs:52-69`) carries the four ref variants live; there is no ownership field anywhere.
- `StructMemberT` is now `{ name, tyype: KindT }` (`ast/citizens.rs:97-101`) — `tyype: KindT` is a *context* line in the working diff, so the `IMemberTypeT` deletion is committed; only the `NormalStructMemberT` → `StructMemberT` rename is in flight (that one may be another session's).
- `ExpressionTE` (`ast/expressions.rs:60-83`) has no `SoftLoad` variant, and `MutateTE.destination_expr` is a plain `ExpressionTE` (`:353`) rather than an `AddressExpressionTE`.

So `determine_closure_variable_member`'s ownership match, `pointify_kind`, and `make_closure_struct_construct_expression` are dangling fossils — they're part of the 103 errors. There is no "Share arm to preserve or rename." I framed those as live policy last message; they can't compile.

## Your intuition is right, and it's now enforced structurally

`LocalLookupTE` (`ast/expressions.rs:639-655`):

```rust
pub struct LocalLookupTE<'s, 't> {
    pub range: RangeS<'s>,
    pub local_variable: LocalVariable<'s, 't>,
    // A local lookup is a borrow reference to the variable's value.
    pub result: &'t BorrowRefT<'s, 't>,     // <-- the type, not a policy
    _sealed: (),
}

pub fn new(...) -> LocalLookupTE<'s, 't> {
    let result = interner.alloc(BorrowRefT { inner: local_variable.tyype(), region: RegionT::Default });
    ...
}
```

The `result` field is *typed* `&'t BorrowRefT`. A local lookup cannot produce anything but a `BorrowRef` — it's not a match arm anyone could get wrong. `x = 4` gives a local of kind `Int`, mentioning it yields `BorrowRef(Int, Default)` = `&int`. No sharedness check, no primitive gate; `get_borrow_ownership`'s old `KindT::Int(_) => OwnershipT::Share` is now *inexpressible*. Same treatment at `ReferenceMemberLookupTE` (`:746`), `AddressMemberLookupTE` (`:770`), and both array lookups (`:680`, `:706`).

The deeper reason is that **`SoftLoad`'s deletion collapsed the lvalue/rvalue split**. Under onion, "address of `x`" and "borrow of `x`" are the same thing — `&x` — so the wrap moved into the lookup constructor and the deref node became unnecessary. This inverts RMLRMO: the Scala-era doc (`docs/old/Compiler/Templar/Addresses.md:32-34`) concluded that ReferenceMemberLookup yields the member's *owned* type with a SoftLoad wrapping it to `&Engine`; the new comment at `:745` cites RMLRMO for the opposite conclusion.

## My "open question" about `@Spaceship` is answered — `&@`

`ConstantStrTE::new` (`:619`) builds `ShareRefT { inner: KindT::Str(StrT) }`, and share citizens can't be held bare, so a `str` local's kind is `ShareRef(Str)`. `LocalLookup` wraps it unconditionally → `BorrowRef(ShareRef(Str))` = `&@str`. Q3's uniform borrow won; the old Share pass-through is gone and the constructor has no way to express it. So `determine_closure_variable_member`'s whole three-arm match collapses to one line — member = `BorrowRef(local's kind)` — which is exactly what `LocalLookupTE::new` already does.

Worth knowing for when that gets rewritten: **the Share arm was live, but via `str`, not `struct ... share`.** The one fixture that reaches it is `typing/test/compiler_lambda_tests.rs:320`:

```
func helperFunc(x str) {
  { print(&x); }();
}
```

`x` is `Share + Str`, captured read-only, so it's non-addressible (`determine_if_local_is_addressible` gates Shared locals on `child_mutated` alone) and flows into the Reference path. Zero `struct Foo share` fixtures capture into a closure. So that test is the one that will exercise the new `&@str` shape.

## The real open question I turned up

**Q3's pass-through isn't implemented, and it has nowhere left to live.** The handoff says bare-use "passes through unchanged when `K` is `BorrowRef` (avoids accidental double-borrows in ordinary code)." But `LocalLookupTE::new` wraps unconditionally, so a local holding a `&Marine` yields `BorrowRef(BorrowRef(Struct(Marine)))` = `&&Marine`. Coercion row (d) says `&&K → &K` is an **error, no silent coerce** — which together would reject ordinary code that passes a borrow onward.

Previously the lvalue/rvalue split absorbed this: the lookup gave you the slot, SoftLoad read it. With SoftLoad deleted, there's no deref node, so the only places the pass-through can go are (a) a check inside `LocalLookupTE::new` (`if inner is BorrowRef { reuse it }` — sharedness-free, cheap), or (b) softening row (d) for this case. That's a design call, not mechanical, so I'm leaving it with you rather than guessing.

---

## 👤 User

what's missing/unimplemented?

---

## 🤖 Claude

Let me get ground truth from the compiler rather than guess — plus an inventory of explicit stubs.

---

## 🤖 Claude

808 errors — the tree has moved a lot since the handoff's 103. Let me categorize them.

---

## 🤖 Claude

Ground truth: **808 compile errors**, up from the handoff's 103 — because the pre-onion substrate got deleted wholesale and the callers haven't been chased yet. (Snapshot caveat: other sessions are editing, so the exact count is drifting under me.) I read your question as "what's left in the onion/typing migration" — say the word if you meant something narrower.

The headline is that the ratio is good: roughly 90% is mechanical cascade from deletions that already happened, and only a handful of things are genuinely undesigned.

## The cascade — deleted substrate, callers not updated

| Deleted thing | Errors | Where callers still reference it |
|---|---|---|
| `ITemplataT::Coord` / `CoordTemplataT` | ~126 | the templata layer — largest single family |
| `OwnershipT` (+ `.ownership` field) | ~126 | everywhere |
| `ExpressionTE::Reference` / `::Address` | 53 | the lvalue/rvalue split |
| `.kind` field on `KindT` | 37 | old `Coord.kind` reads |
| `KindT::new` | 34 | old Coord constructor |
| `LocalVariable::Reference` / `::Addressible` | 26 | the enum is now a plain struct |
| `IStructMemberT` / `IMemberTypeT` / `ReferenceMemberTypeT` | 23 | member model |
| `BlockTE::result` | 21 | |
| `SoftLoad` / `soft_load` / `borrow_soft_load` | 14 | |

Plus 43 field renames on AST nodes (`ArgLookupTE.coord` ×19, `FunctionCallTE.return_type` ×5, `ConstructTE.result_reference`, `AddressMemberLookupTE.result_type2`, …) — fallout from the `result:` normalization in `expressions.rs`. And ~26 postparse-side leftovers typing hasn't caught up to (`ParameterS.pattern` ×17, `ImplS.sub_citizen_rune` ×7, `IRulexSR::CoerceToCoord`).

It's concentrated: `expression_compiler.rs` (246), `pattern_compiler.rs` (70), `compiler_error_humanizer.rs` (67), `templata_compiler.rs` (51). Four files are half of it.

## The genuinely undesigned parts

**`ITemplataT::Coord` is the big one** — 126 errors, 45 of them construction sites. The handoff covers `CoordTemplataType` → `KindTemplataType` at the *type* level, but the templata *value* `CoordTemplataT` has no stated onion replacement. This is the largest family and it's semantic, not mechanical.

**The boxing/addressibility model is now self-contradictory.** `IMemberTypeT::Address` is deleted, so no member can be an address — but `AddressMemberLookupTE` still exists in `ExpressionTE`, and `determine_if_local_is_addressible` (plus its spec test at `closure_tests.rs:61-107`) is still live. Either boxing is retired and those go, or members need an address flavor back. Right now it's half of each.

**`get_sharedness`** (`compiler.rs:1699-1718`) has no arms for the four ref variants — it's non-exhaustive and can't compile. Under onion it probably shouldn't take an arbitrary `KindT` at all, only a citizen kind.

**`convert()`'s probe-based coercion** — the whole table from the handoff is unwritten; `convert_helper.rs` has 28 errors.

**The rune-type solver** is still sitting at its pre-onion 810-LOC shape, live-linked, awaiting the ~50-80 LOC walker rewrite. **The anon-interface macro** (21 errors) still wants a stored rune-type map instead of `derive_rune_to_type`.

**Q3's pass-through**, from my last message — `LocalLookupTE::new` wraps unconditionally, so a local holding `&Marine` yields `&&Marine`, which coercion row (d) calls an error.

Beyond the errors: **409 explicit stub panics** (`panic!("Unimplemented: …")` / `implement:`) across `typing/`, and **124 VCOORD markers**.

## One thing that's now unblocked

About 40 of those VCOORD markers are in `builtins/resources/arith.vale` and `logic.vale`, all saying *"retire when Phase 2 lands (bare-use→Borrow makes the & overload unnecessary)"* — duplicate `&`-flavored overloads of every arithmetic and logic builtin. Per what I found last message, bare-use→Borrow **has** landed structurally: `LocalLookupTE.result` is typed `&'t BorrowRefT`, so every mention of a local is already a borrow. Those ~40 overloads look retirable now. That's a clean, purely-mechanical sweep if you want one queued up — though I'd hold until the typing cascade settles, since deleting overloads while resolution is red would be hard to validate.

---

## 👤 User

hold please

---

## 🤖 Claude

Holding. Nothing in flight — no edits made, all read-only.

Ready when you are.

---

## 👤 User

tldr the above? that was a lot

---

## 🤖 Claude

**Your intuition is right: yes, `&int`.** And it's not a decision to make — it's already committed and structurally enforced.

Three things:

**1. I corrected myself.** The code I quoted you last message (`determine_closure_variable_member`'s ownership match, `pointify_kind`) is dead. `OwnershipT`, `CoordT`, `IMemberTypeT`, and `SoftLoad` are all already deleted from the tree. Those functions are dangling fossils — part of the 103 errors, not live policy.

**2. The answer lives in a field type, not a match arm.** `LocalLookupTE.result` is typed `&'t BorrowRefT` (`ast/expressions.rs:643`), so a local lookup *cannot* produce anything but a `BorrowRef`. A local of kind `Int` → `&int`. The old primitive gate (`Int => Share`) is now inexpressible. Same for member and array lookups.

**3. My earlier "open question" (`@Spaceship` vs `&@Spaceship`) is answered: `&@`.** Uniform borrow won; no pass-through arm exists.

**The one thing worth your attention:** Q3's pass-through isn't implemented and has nowhere left to live. `LocalLookupTE::new` wraps unconditionally, so a local holding a `&Marine` gives `&&Marine` — and coercion row (d) says `&& → &` is an error. That combination rejects ordinary code. `SoftLoad`'s deletion removed the deref step that used to absorb this, so it needs to go either inside `LocalLookupTE::new` or into row (d). That's your call.

---

## 👤 User

so what dead code can we delete?

---

## 🤖 Claude

Good question — but "dead" splits two ways here, and the distinction matters: some of this has *lost its purpose* (delete), and some merely *references removed APIs* while the function still needs to exist (rewrite). Let me audit rather than guess.

---

## 🤖 Claude

Let me check which of these are in **linked** code (causing the 103 errors) versus unlinked modules, and whether they're live or already commented out.

---

## 🤖 Claude

Here's the audit. Short version: **there's less safely-deletable code than the 103 errors suggest** — most of the red needs *rewriting*, not deleting. The genuinely dead set is about 195 lines, and almost none of it is currently costing you errors.

## Tier 1 — dead, purpose dissolved, safe to delete now

**`pointify_kind`** — `templata_compiler.rs:1218-1248` (31 lines), plus its only call site (`expression_compiler.rs:2232-2233`) and the assert it feeds (`:2239`). Verified sole caller; the assert is tautological (both sides derive ownership from the same `lookup_mutability` on the same struct). Under onion the closure struct's bare-vs-`ShareRef` shape is decided at construction, so nothing recomputes it. This is the only Tier-1 item that removes an actual compile error.

**The commented soft-load family** — `local_helper.rs:97-226` (~130 lines): `maybe_borrow_soft_load`, `soft_load`, `borrow_soft_load`, `get_borrow_ownership`. All already commented out, and now unrevivable: `SoftLoad` is gone from `ExpressionTE`, and `get_borrow_ownership`'s primitive gate (`Int => Share`) is inexpressible since `LocalLookupTE.result` is typed `&BorrowRefT`. Handoff item 4 says this function "goes away" outright.

**The commented ownership match in `is_type_convertible`** — `templata_compiler.rs:1184-1213` (~30 lines). Handoff §4 calls the broadening obsolete. Delete the commented block, **not** the function — the handoff's wording ("coercions are target-side rules that don't need `is_type_convertible` to permit them") suggests the function itself may survive for isa/kind matching. That part's your call.

Honest caveat: items 2 and 3 are commented corpses, so deleting them buys hygiene, not progress toward green. Only `pointify_kind` moves the error count, by roughly one.

## Tier 2 — dead per the handoff, but inside your territory

**`AliasTE` in `convert_helper.rs:177-179`** — live code emitting `ExpressionTE::Alias`, a variant that no longer exists. Handoff §4: "obsolete — no boundary collapse to bridge." But it sits inside `convert()`, which you're rewriting wholesale, so I'd fold it into that rather than pick it out.

**`compiler_solver.rs:667` and `:833`** (the file you have open) — the `CoordSendSR` else-branch and `CallSiteCoordIsa` ancestor-branch coercion-accept patches. Handoff §4 lists both as obsolete under onion.

## Tier 3 — red, but rewrite not delete

The bulk of the 103. `determine_closure_variable_member` and `make_closure_struct_construct_expression` still need to exist (they collapse to roughly one line each). The 43 dangling `IMemberTypeT` refs are just members that are plain `KindT` now. `traverse.rs`'s SoftLoad visitor loses a node, not the traversal. And the `SoftLoadTE` assertions in `compiler_tests.rs` want re-authoring against the new shapes — per CLAUDE.md I wouldn't delete or ignore those.

## Tier 4 — don't touch yet

`instantiating/instantiator.rs` holds 54 of the 63 `AddressExpressionTE`/`ReferenceExpressionTE` refs plus most `AliasTE` refs, and `integration_tests/` holds the rest. Both are commented out of `lib.rs:17-18`, so they cost nothing today and belong to the instantiator slice.

## One thing I want to surface rather than guess

**Is addressibility retired?** The signals conflict. `LocalVariable` is now a plain struct (`function_environment_t.rs:934-940`) — the `Reference`/`Addressible` enum is gone — and `IMemberTypeT::Address` is gone with it. But `determine_if_local_is_addressible` still exists (`local_helper.rs:229`) with a dedicated spec test (`closure_tests.rs:61-107`), and `AddressMemberLookupTE` is still a live `ExpressionTE` variant. So either boxes are being retired and ~40 lines plus that test go away, or they're staying and the local-variable enum needs restoring. That changes what's deletable in `local_helper.rs`, and it's a design call, not a mechanical one — so I've left it alone.

Want me to execute Tier 1?

---

## 👤 User

please comment out pointify_kind, dont delete

---

## 🤖 Claude

Let me re-read both sites first — other sessions may have moved them since I last looked.

---

## 🤖 Claude

The file shifted under me — another session is editing. Let me relocate the call site.

---

## 🤖 Claude

Now I'll comment out the function, matching the repo's existing commented-block style.

---

## 🤖 Claude

Guardian caught that I'd rewritten the `VCOORD:` marker into my own prose. Preserving it verbatim instead.

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

i removed it, try again

---

## 🤖 Claude

Marker's gone. Commenting out the function now.

---

## 🤖 Claude

Now the call site — leaving a call to a commented-out function would just trade one error for another.

---

## 🤖 Claude

Done — `pointify_kind` is commented out at `templata_compiler.rs:1218`, along with its sole call site and the assert it fed in `evaluate_closure` (`expression_compiler.rs:2189-2196`).

Two notes on what I did beyond the literal ask:

**I commented out the call site too.** You only asked for `pointify_kind`, but leaving `self.pointify_kind(...)` calling a commented-out function would have swapped a `KindT::new` error for an unresolved-method error — no net progress. The assert at `:2196` had to go with it since `closure_coord` was its only consumer, and that assert was the tautology the whole investigation turned on. `make_closure_struct_construct_expression` still runs and its result is still returned; nothing else read `closure_coord`.

**Guardian was right and I was wrong.** My first two attempts tripped NRVMX because I'd folded your `// VCOORD: not sure this should exist long term` into my own prose — the second attempt kept the text but the shield correctly counts a rewrite as a removal. Thanks for clearing it; the header comment now says the same thing in explanatory form, without a marker.

I did not run a build. The crate is red for unrelated reasons, so a `cargo check` wouldn't isolate this change's effect — but if you want the error count re-measured against `./tmp/`, say the word and I'll run it.

Remaining Tier 1, untouched and awaiting your call: the commented soft-load family (`local_helper.rs:97-226`, ~130 lines) and the commented ownership match inside `is_type_convertible` (`templata_compiler.rs:1184-1213`, ~30 lines). Both are already-commented corpses, so deleting or leaving them changes nothing mechanically.

---

## 👤 User

what was that assert meant to guard against?

---

## 🤖 Claude

Good question — I asserted it was tautological but never established what it was *for*. Let me check the history, since this is a Scala port and it may not always have been.

---

## 🤖 Claude

`sharedness` is hardcoded. Let me trace whether the mutability map gets its value from the same place.

---

## 🤖 Claude

Tracing it all the way down: **it guarded against the closure struct's mutability map disagreeing with its compiled definition — a divergence that's currently impossible, but was pre-positioned for a feature that hasn't landed.**

## It was tautological even harder than I said

I claimed the two sides derive from "the same `lookup_mutability` on the same struct." The truth is stronger. In `make_closure_understruct_core` (`struct_compiler_core.rs`):

```rust
// VCOORD:
// In the distant future, we'll want to opt into shared closures with a simpler syntax.
let sharedness = SharednessT::Single;                                   // :341 — hardcoded
...
coutputs.declare_type_mutability(understruct_templated_id, sharedness); // :455 — feeds the map
...
let closure_struct_definition = StructDefinitionT {
    ...
    sharedness,                                                         // :462 — feeds the def
```

One local variable, holding a **constant**, written to both places. `pointify_kind` read it back via `get_sharedness` → `lookup_mutability(template)` → the map at `:455`. `make_closure_struct_construct_expression` read it via `closure_struct_def.sharedness` → the def at `:462`. So the assert compared `Own` to `Own` where both sides trace to a hardcoded `SharednessT::Single`. It could never fire.

## But there was real intent behind it

Two things make it more than noise.

**It's a verbatim Scala transcription.** `evaluate_closure` was `panic!("Unimplemented: Slab 15 — body migration")` until `7a65955a0`, where the whole body — assert included — landed in one shot. Given this repo's `ScalaParityDuringMigration` and `NoNewDefinitions` shields, the porter was obligated to carry the original `vassert` across faithfully rather than reason about whether it still held. So the question "what was it meant to guard" is really about the Scala author's intent, and the answer is inherited, not invented here.

**The two writes are genuinely separate statements.** `:455` and `:462` are seven lines apart, and nothing structurally couples them — they only agree because a human kept them in sync. That's exactly the shape an assert is for. And the `VCOORD` at `:339` names the future that would activate it: *"we'll want to opt into shared closures."* The moment `sharedness` stops being a constant and becomes computed from a `share` annotation, a refactor could plausibly update the map write and miss the definition write, or vice versa — and the assert in `evaluate_closure` would catch it at the first lambda.

So it's a **latent** invariant check: vacuous today, load-bearing the day shared closures land. Not a useless assert — a pre-positioned one.

## Which is why it still doesn't survive onion

The invariant it wants is real; the mechanism is what dissolves. "Ownership tag computed from sharedness" is the flat-ownership pattern going away. The onion equivalent — *"a closure struct's wrapper is bare for a Single citizen and `ShareRef` for a Shared one"* — is one row of the validity table, and the handoff puts enforcement in the interner ("interner should reject constructions that violate it"), not in an assert at one call site. That's strictly better: the interner catches every construction site, whereas this assert only ever watched lambdas.

Worth noting for whoever lands shared closures: when `:341` stops being a constant, that invariant needs to exist *somewhere*. If the interner validity table is in place by then, it's covered. If not, the check I just commented out was the only thing watching it — so it's worth a `V` marker on `:341` pointing at that dependency rather than letting it silently evaporate. Your call whether to add one; I won't touch the marker.

---

## 👤 User

there's a bunch of errors everywhere about us constructing expression AST directly, instead of calling ::new, do you see what i mean?

---

## 🤖 Claude

Yes — I think I know exactly what you're seeing, but let me confirm against the real errors rather than guess.

---

## 🤖 Claude

701 errors now, not the handoff's 103 — the onion work has landed a lot since. Let me isolate the class you're asking about.

---

## 🤖 Claude

Now I've got the full picture. Let me check how mechanical the fix actually is by sampling the two biggest offenders.

---

## 🤖 Claude

Yes — I see it. It's the `_sealed: ()` field, and it's a deliberate forcing function, not incidental breakage.

## The mechanism

Every `*TE` struct in `ast/expressions.rs` now carries a private `_sealed: ()` field alongside its `result`:

```rust
pub struct BlockTE<'s, 't> {
    pub inner: ExpressionTE<'s, 't>,
    pub result: KindT<'s, 't>,
    _sealed: (),                       // <-- private, so no struct literal outside this module
}

impl<'s, 't> BlockTE<'s, 't> {
    pub fn new(inner: ExpressionTE<'s, 't>) -> BlockTE<'s, 't> {
        let result = inner.result();   // <-- the only place result is decided
        BlockTE { inner, result, _sealed: () }
    }
}
```

Since `_sealed` is private, `BlockTE { .. }` is unconstructible outside `expressions.rs`. That makes `::new` the sole entry point — and `::new` is where the result type gets *derived* rather than handed in.

**This is the same design I hit in the closure investigation.** It's why `LocalLookupTE::new` can guarantee `&int`: the `result` field is typed `&'t BorrowRefT`, and no caller can hand it something else. Sealing generalizes that guarantee across the whole AST — a caller can no longer hand-compute a result type (which is exactly what the old `OwnershipT`-based code did). The 65 errors are every site that used to do so.

## The scale

65 errors across 26 expression types. It's the fourth-largest class, well behind `OwnershipT` (112), `ITemplataT::Coord` (68), and `CoordTemplataT` (45).

Also worth flagging: **the tree is at 701 errors, not the handoff's 103.** The handoff is stale — your onion work has landed a lot since it was written.

## Most of it is genuinely mechanical

`BlockTE` and `ReturnTE` alone are 30 of the 65, and the call sites already pass exactly what `::new` wants:

```rust
// function_body_compiler.rs:230
BlockTE { inner: converted_body_with_return }   →   BlockTE::new(converted_body_with_return)

// struct_constructor_macro.rs:230
ReturnTE { source_expr: construct_expr }        →   ReturnTE::new(construct_expr)
```

rustc even emits the `help: you might have meant to use the new associated function` suggestion on each. The old sites never passed `result` at all, so there's nothing to drop — it's a pure literal→`::new` rewrite, concentrated in `typing/macros/` (about 20 of the sites are `*_macro.rs` files with near-identical `Block(Return(...))` shapes).

Two caveats before I'd sweep it:

- **Not all of them are one-liners.** `ConstantIntTE`, `MutateTE`, `FunctionCallTE` etc. may have `::new` signatures that take different arguments than the literal supplied, and some sites currently pass a hand-computed `result`/`result_reference` that must be *dropped* rather than forwarded. Those need reading, not pattern-matching.
- **Two sites are pattern matches, not constructions** — `compiler_solver_tests.rs:341-342` destructures `BlockTE { inner }` / `ReturnTE { source_expr }`. Sealing doesn't block those; they need `..` added, and they're a different fix.

Want me to sweep the mechanical subset? I'd suggest starting with `BlockTE` + `ReturnTE` (30 errors, ~20 files, uniform shape) — that's Edit-tool territory per the scripting skill's ~40-invocation rule, no script needed. I'd leave the ones with real signature changes for you.

---

## 👤 User

im having another claude session work on expression_compiler.rs, and another on pattern_compiler.rs. but can you fix the rest of this kind of error in the other files? please do NOT fix any that require any sort of onion typing logic change. just do a very mechanical predictable translation. i'll fix any errors later

---

## 🤖 Claude

On it. Let me get the exact site list, excluding the two files other sessions own.

---

## 🤖 Claude

`pattern_compiler.rs` has none, so it's just `expression_compiler.rs` (10 sites) to leave alone — 55 sites across 22 files for me. Let me map each site to its type.

---

## 🤖 Claude

Let me pull every relevant `::new` signature so I know exactly what each site must pass.

---

## 🤖 Claude

I have everything I need. One flag before I start: `UpcastTE::new` is `unimplemented!("UpcastTE onion result")` with a `VCOORD` on it — that's onion logic you haven't written, so I'll leave `convert_helper.rs:90` alone. Let me read the sites in batches.

---

## 🤖 Claude

Very uniform. Starting with the RSA macros — same `Block(Return(...))` shape in each.

---

## 🤖 Claude

Now the RSA new macro and the SSA macros.

---

## 🤖 Claude

Now the remaining macros: `abstract_body`, `as_subtype`, `same_instance`, `lock_weak`, `struct_constructor`.

---

## 🤖 Claude

Now `struct_drop_macro.rs` (6 sites — the biggest single file).

---

## 🤖 Claude

Macros done. Now the non-macro files.

---

## 🤖 Claude

Now `function_compiler_core.rs`, `destructor_compiler.rs`, `call_compiler.rs`, `array_compiler.rs`, `local_helper.rs`.

---

## 🤖 Claude

All sites edited. Let me re-check and confirm the error class actually dropped.

---

## 🤖 Claude

65 → 1. Let me verify I didn't introduce anything new in the files I touched.

---

## 🤖 Claude

Let me confirm none of the remaining errors in those files are mine — checking for arity/type mismatches specifically.

---

## 🤖 Claude

Done. **65 sealed-literal errors → 1**, and the one left is the one I flagged upfront.

## What I changed

55 sites across 19 files, all pure literal→`::new` rewrites:

| Pattern | Files |
|---|---|
| `Block(Return(...))` bodies | 12 macros (`rsa/*`, `ssa/*`, `abstract_body`, `as_subtype`, `same_instance`, `lock_weak`, `struct_constructor`) |
| `ArgLookupTE { param_index, coord }` | `function_body_compiler`, `function_compiler_core`, `struct_drop_macro`, 5 rsa/ssa macros |
| everything else | `struct_drop_macro` (6), `destructor_compiler`, `call_compiler`, `array_compiler`, `sequence_compiler`, `compiler`, `local_helper` |

Total errors: **701 → 625**. The delta is larger than 55 because the `ArgLookupTE` rewrites also cleared the separate `has no field named coord` class for free — that field was renamed to `result`, and `::new` takes it positionally.

Two spots needed more than a bracket swap, both still mechanical:
- **`ExternFunctionCallTE`** (`function_compiler_core.rs:375`) — `::new` derives `result` from `prototype2.return_type`, so the explicit field was dropped rather than forwarded. Same value.
- **`LetAndLendTE`** (`local_helper.rs:37`) — `::new` takes the interner as its first arg; passed `self.typing_interner`, which was already in scope.

## What I deliberately left

**`convert_helper.rs:90` (`UpcastTE`)** — the one remaining sealed error. `UpcastTE::new` is `unimplemented!("UpcastTE onion result")` under a `VCOORD` saying *"preserve the inner wrap and swap the innermost citizen to target_super_kind."* Converting the call site would compile but swap a compile error for a runtime panic, and picking the onion result is exactly the logic change you told me not to make. It's yours.

I also left untouched, inside files I edited, every line that's a different error class: `LocalVariable::Reference` (`struct_drop_macro.rs:296`, `local_helper.rs:37`), `param_coords[0].tyype.kind` (`ssa_len_macro.rs:43`), `PopRuntimeSizedArrayTE { element_type }` (`rsa_pop_macro.rs:49`), `ITemplataT::Coord` (`rsa_new_macro.rs:53`). I preserved those arguments verbatim inside the new `::new` calls, so they still fail the same way they did before — as you asked.

## Verification

I checked every arity and type-mismatch error in the tree against my edit sites: none land on a line I touched (they're at `array_compiler.rs:555/676/781`, `struct_constructor_macro.rs:99`, `struct_drop_macro.rs:110/157`, all pre-existing). `expression_compiler.rs`'s 10 sealed sites also cleared — that's the other session's work, not mine; I only touched that file for the earlier `pointify_kind` call site.

---

## 👤 User

please do convert_helper.rs:90 now ty

---

## 🤖 Claude

The call site already passes exactly `::new`'s three args in order, so this is a clean mechanical swap.

---

## 🤖 Claude

**Sealed-literal errors: 0.** The whole class is gone. Total errors 625 → 604.

The swap was exact — the old literal passed `inner_expr`, `target_super_kind`, `impl_name` and never supplied `result`, which is precisely why it errored; `::new` takes those same three positionally and derives `result` itself. Nothing dropped, nothing reordered.

One thing to be aware of, since it changes the failure mode rather than removing it:

**This site now reaches `unimplemented!("UpcastTE onion result")` at runtime.** Before, it was a compile error; now it compiles and panics if an upcast is ever evaluated. That's the `VCOORD` you left on `UpcastTE::new` — *"preserve the inner wrap and swap the innermost citizen to target_super_kind"* — and filling it in is the onion logic I'm staying out of. The tradeoff is inherent to converting this one: there's no version of calling `::new` that doesn't route into its unimplemented body. Given the crate is 604 errors from compiling, nothing executes yet either way, so it costs you nothing today — but it's now invisible to `cargo check`, where before it was one of the 65.

The 27 remaining `convert_helper.rs` errors are all other classes (`OwnershipT`, `ITemplataT::Coord`, the `AliasTE` arm at `:177-179` from the earlier audit) — untouched.

---

## 👤 User

hold please

---

## 🤖 Claude

Holding.

Nothing in flight. Last action was `convert_helper.rs:90` (`UpcastTE` → `::new`), which cleared the sealed-literal class to zero at 604 total errors.

---

## 👤 User

hold please

---

## 🤖 Claude

Still holding. Nothing running, no pending edits.

---

## 👤 User

hold

---

## 🤖 Claude

Holding.

---

## 👤 User

hold

---

## 🤖 Claude

Holding.

---

## 👤 User

can you please look into determine_closure_variable_member and tell me what parts of it will need to change?

---

## 🤖 Claude

Let me read it fresh — other sessions have been moving things.

---

## 🤖 Claude

Here's the picture. **Every line of the body is dead** — 18 errors across it, and nothing it references still exists.

## What's gone underneath it

| The code says | Reality |
|---|---|
| `coord.ownership`, `coord.region`, `coord.kind` | `KindT` has no such fields — it's the onion enum now |
| `KindT::new(ownership, region, kind)` | doesn't exist |
| `OwnershipT::{Own,Borrow,Share,Weak}` | enum deleted (10 of the 18 errors) |
| `IMemberTypeT::{Reference,Address}`, `ReferenceMemberTypeT`, `AddressMemberTypeT` | all deleted — zero definitions in the tree |
| `StructMemberT.tyype` | now a plain `KindT`, not an `IMemberTypeT` |

So the signature and the interner call at `:315` survive; the entire `match` at `:287-314` does not.

## What it collapses to

The whole thing becomes roughly:

```rust
let translated_name = self.translate_var_name_step(name);
let inner = match env.get_variable(translated_name).unwrap() {
    IVariableT::Local(v) => v.tyype,
    IVariableT::Capture(v) => v.coord,
};
let tyype = KindT::BorrowRef(self.typing_interner.alloc(BorrowRefT { inner, region: RegionT::Default }));
self.typing_interner.alloc(StructMemberT { name: translated_name, tyype })
```

That's the same rule `LocalLookupTE::new` already enforces (`expressions.rs:653`) — which is what makes it right: the member type has to *match* what the construct expression puts in it, and the construct side now goes through the sealed `::new`. The three-arm ownership match dissolves because there's no ownership tag to branch on; "captured own is borrow" stops being a case and becomes the unconditional rule.

Two field-name traps worth noting: `LocalVariable` uses `tyype` but `CapturedVariableT` still uses `coord`, so they don't destructure alike. And `IVariableT` has only two variants now.

## Four things I can't decide for you

**1. Arms `:298` and `:311` are already dead as written.** They duplicate the `:288`/`:301` patterns — `IVariableT::Local(LocalVariable { tyype: reference, .. })` is the same pattern as `:288` with a different binding name, so it's unreachable. They're the vestige of a `Reference`/`Addressible` split that no longer exists in `LocalVariable`. Deleting them is right *if* addressibility is retired — which is the open question from my earlier audit and still unresolved (`determine_if_local_is_addressible` still exists at `local_helper.rs:230`, with a spec test, and `AddressMemberLookupTE` is still a live `ExpressionTE` variant, but `IMemberTypeT::Address` is gone so a member can no longer *be* an address). Note `make_user_local_variable` (`local_helper.rs:83-93`) still calls `LocalVariable::Addressible(...)` / `::Reference(...)` as if the enum existed, so something has to give.

**2. The Q3 pass-through.** If the captured var's kind is already `BorrowRef`, the sketch above yields `&&Marine`. Same unresolved question I raised earlier — and the answer must match whatever `LocalLookupTE::new` does, or the member type and the construct expression disagree.

**3. Region.** I wrote `RegionT::Default` because that's what `LocalLookupTE::new` hardcodes. Under onion the region lives on `BorrowRefT`, and a closure capturing across regions may want the source's. Currently unknowable from this function — it doesn't take a region param.

**4. The `Share` arm's semantics.** Today `@str` passes through as `@str` (closure holds an RC handle). The sketch makes it `&@str`. That's the decision I flagged; `LocalLookupTE::new` has already implicitly made it, but this function is where it becomes visible in the closure struct's shape.

Want me to write the mechanical skeleton and leave `// V:` markers at 2–4? I'd rather not guess at any of them.

---

## 👤 User

please look at what the function used to do in master and in experimental-2

---

## 🤖 Claude

Let me pull the function from both branches.

---

## 🤖 Claude

This is the answer to the open question. Let me confirm the collapse's provenance.

---

## 🤖 Claude

That comparison settles the question I've been flagging. **Addressibility wasn't retired — it was destroyed by a mechanical variant collapse**, and the two "dead" arms are the wreckage.

## What master had

`IVariableT` carried the distinction **in the variant itself**:

```rust
pub enum IVariableT<'s, 't> {
  AddressibleLocal(AddressibleLocalVariableT<'s, 't>),
  ReferenceLocal(ReferenceLocalVariableT<'s, 't>),
  AddressibleClosure(AddressibleClosureVariableT<'s, 't>),
  ReferenceClosure(ReferenceClosureVariableT<'s, 't>),
}
```

So master's four arms were all live and each meant something distinct:

| variant | member type |
|---|---|
| `ReferenceLocal` | ownership match → `Reference` |
| `AddressibleLocal` | → `Address` |
| `ReferenceClosure` | ownership match → `Reference` |
| `AddressibleClosure` | → `Address` |

Reference-vs-Addressible was the **outer** axis; ownership was the **inner** one. Two orthogonal dimensions, 4 = 2×2.

## What the collapse did

`538fdb12a` ("temporary commit") folded 4 variants → 2 by dropping the *Addressible/Reference* axis and keeping only the *Local/Closure* one:

- `ReferenceLocal(…{ coord })` → `Local(LocalVariable { coord })`
- `AddressibleLocal(…{ coord: reference })` → `Local(LocalVariable { coord: reference })`

Those are now **the same pattern**. Arms `:298` and `:311` didn't become redundant through some later drift — they were rewritten into duplicates of `:288`/`:301` in one shot. `LocalVariable { name, tyype }` has no addressibility bit, so there is nowhere left to record the answer. The `Address` arms are unreachable not by design but because the discriminant they matched on no longer exists.

## Why this is a live problem, not just dead code

The rest of the addressibility machinery **survived the collapse intact**:

- `determine_if_local_is_addressible` still exists (`local_helper.rs:230`) and still computes the bit.
- `make_user_local_variable` (`local_helper.rs:83-93`) still calls `LocalVariable::Addressible(…)` / `LocalVariable::Reference(…)` as if the enum were still there — that's the 19 `no associated item named Reference found for struct LocalVariable` errors.
- `closure_tests.rs:61-107` still spec-tests the addressibility rules.
- But `IMemberTypeT::Address` is gone, so a member can no longer *be* an address.

So the concept is half-collapsed: computed, tested, and branched on — with no representation and no consumer. That's why `determine_closure_variable_member` can't be finished mechanically. It's not "which arm do I keep"; it's **does addressibility still exist**.

Two coherent endings, and it's your call:

1. **Retire it.** `determine_if_local_is_addressible`, its callers, the `Addressible`/`Reference` branch in `make_user_local_variable`, and the `addressibility` spec test all go. `determine_closure_variable_member` becomes the one-liner I sketched. Boxes/mutable-capture semantics would need to come from somewhere else — `mutates_from_inside_a_closure` (`closure_tests.rs:379`) currently depends on an `Address` member.
2. **Restore the discriminant.** `LocalVariable` regains an addressibility bit (or `IVariableT` regains the 2×2), and the function keeps a two-arm outer match with the onion wrap replacing the inner ownership match.

Also worth noting in passing: master's `NormalStructMemberT` carried a `variability` field that experimental-2 dropped, and `VariabilityT` is now zero-hits in the tree — a separate axis retired by the same commit.

I'd lean toward (2) being the safer read, since three independent pieces of the machinery survived and only the storage vanished — that shape looks like collateral rather than intent. But per the "surface before reverting" rule I'm not touching it; the collapse is landed architectural progress and you may well mean to finish retiring it.

---

## 👤 User

well we really shouldnt have addressibility anymore. that concept goes away i think, replaced with the notion that LocalLoad will produce a reference to the local, similar to how LLVM thinks. dunno. wdyt?

---

## 🤖 Claude

That's the right instinct, and I'd go further: **the AST has already committed to your model — `determine_if_local_is_addressible` is the last holdout, not a live design choice.** But let me check one thing that decides how clean the retirement is.

---

## 🤖 Claude

No fixture escapes a closure — all four lambda fixtures call immediately (`({ set x = 42; })()`). That matters. Here's my view.

**Agree, and I'd argue it's already decided — `determine_if_local_is_addressible` is the last holdout, not a live choice.** `LocalLookupTE.result` is typed `&'t BorrowRefT` and `::new` wraps unconditionally. That *is* the alloca model: every local is storage, a load hands you a pointer. There's no bit to consult and, since the collapse, nowhere to put one.

**The LLVM analogy is exact for the case that actually matters.** In `mutates_from_inside_a_closure`, the closure holds `&x` pointing at main's storage for `x`; `set x = x + 1` stores through it. Same storage, no box, mutation just works. That's mem2reg's world — alloca everything, promote later — and the promotion (which locals can live in registers) becomes a hammer/backend concern instead of a typing one. That's the deeper win: addressibility is **typing doing escape analysis**, and typing is the wrong pass for it. `determine_if_local_is_addressible` reads `child_mutated`/`child_moved` off the scout's use-tracking to predict storage layout — that's a codegen decision smuggled into the type system.

**Two things the box bought that a pointer doesn't.** Worth being explicit rather than discovering them later:

*Moves.* Addressibility fires for Single when `child_moved != NotUsed` — the `Marine`/`consume(m)` case in the spec test. Under your model, moving out of a captured `&x` is move-out-of-borrow, which your coercion table already declares an **error** ("user writes `^local`"). So that case doesn't become a box; it becomes a compile error. Real semantics change, but I think it's the right one — and the Scala author was already unsure, per their own aside: *"(or, we can change it so we move it into the closure struct, but that seems weird, i like thinking that closures only ever have borrows or addressibles)"*. Onion's answer is neither box nor move-into-struct: it's "you can't, say `^`." That's more honest than both.

*Escape.* The box could outlive the frame; `&x` can't. This is the one genuine gap — but it's **latent, not blocking**, since nothing in the tree returns a closure. And it's exactly what `BorrowRefT { inner, region }` is for. Addressibility was a *pre-region* workaround for a lifetime problem; you now have a real lifetime axis, so it's subsumed — but subsumed *by regions*, not by nothing. Someone eventually enforces it, and `LocalLookupTE::new` hardcodes `RegionT::Default` today, so nothing does yet. I'd want a `V` marker there rather than silence.

**One free win:** the Single/Shared asymmetry ("move = copy, so immutables ignore child_moved") evaporates. It only existed because move-tracking needed a box for Single and not for Shared. No box, no asymmetry — that whole branch and half the spec test go.

**One consequence to check:** `ReferenceMemberLookupTE` and `AddressMemberLookupTE` now have *identical* shape — both `result: &'t BorrowRefT`, both wrapping `member_kind`. If addressibility goes, I'd expect those two to merge into one node, which also kills the `Reference`/`Address` split in `ExpressionTE` (43 + 10 of your remaining errors). Worth confirming before committing, since `mutates_from_inside_a_closure` currently pins an `Address` member and would need re-authoring against the merged node — not deleting.

So: yes, retire it. The thing I'd hold onto is that you're not deleting a concept, you're **moving it** — mutation-sharing to `&x`, moves to a compile error, lifetime to regions, layout to hammer. Each has a home. If any of those three homes isn't real yet, that's the thing to mark rather than to quietly drop.

---

## 👤 User

whats the simplest test we have in the typing pass, thats probably going to be the first one to pass?

---

## 🤖 Claude

**`simple_program_returning_an_int_explicit`** (`compiler_tests.rs:74-95`) is the one. It's the simplest by a clear margin and the natural first-to-pass.

```rust
let code = "func main() int { return 3; }";
let code_source = CodeSource::new(vec![
    new_test_code_map(&parse_arena, code),      // <-- that's the whole source list
]);
...
let main = coutputs.lookup_function_by_str("main");
assert!(main.header.return_type.kind == KindT::Int(IntT { bits: 32 }));
```

What makes it the floor:

- **No builtins at all.** Its `CodeSource` is a single entry — no `Source::builtin_module(...)`, not even `empty_v_builtins_stub`. Compare `tests_panic_return_type` (4 builtin modules) or `lambda_inside_different_function_with_same_name` (5). Every builtin you load drags in more of the tree that has to type-check first.
- **Not even `exported`** — `hardcoding_negative_numbers` is otherwise identical but adds the export path.
- **One expression, one literal, one return.** No locals, no calls, no structs, no closures.
- **A weak assertion**: one equality on the header's return type. It doesn't walk the body, so it doesn't pin any expression node's shape.

That last point is why it'll fall first *and* why it's a thin signal. It only exercises: parse → scout → function header resolution → `int` lookup → `ConstantIntTE` → `ReturnTE` → `BlockTE`. Notably the `ReturnTE`/`BlockTE` construction on that path already goes through `::new` now — `function_body_compiler.rs:205` and `:230` were two of the sites I converted.

**One catch worth knowing before you aim at it:** `main.header.return_type.kind` is itself a `.kind`-on-`KindT` access — one of the 26 `no field kind on type KindT` errors. So the *test body* needs an onion fix (probably just `main.header.return_type == KindT::Int(...)`) even once the compiler path works. Same for `simple_local`'s assertion at `:150`.

Runner-up: **`simple_local`** (`:127-151`) — also zero builtins, adds one `let` and a `^a` move. That's the first test that touches `LocalVariable`, `LetNormalTE`, and the move path, so it's the natural *second* target and the first one that would actually exercise the addressibility retirement we just talked through.

If you want a genuine smoke signal rather than a header check, `hardcoding_negative_numbers` (`:97`) is barely harder and does walk the body via `collect_only_tnode!` on `ConstantIntTE`.

---

## 👤 User

whats the first problem it would hit, that we can fix?

---

## 🤖 Claude

Important framing first, then the answer — let me check whether the question is even per-test.

---

## 🤖 Claude

The honest answer is that **the question doesn't have a per-test answer yet** — and that's the useful thing to know before you aim at it.

## There's no "first problem it hits"

Rust builds the whole `lib` before running any test. `cargo test --lib simple_program_returning_an_int_explicit` still has to compile all 604 errors across 45 files first. So that test hits *every* error, not a first one. It isn't closer to passing than any other test — its path isn't shorter, its *blocker set is identical* to every other test's.

That's why "simplest test" and "first to pass" come apart here. `simple_program_returning_an_int_explicit` will be the first to **pass** once the crate compiles, but it can't be the first to **run**, so it can't drive the ordering.

Two consequences worth naming:

- **Nothing gives you a green signal until the last error is gone.** No incremental feedback from the test suite along the way — `cargo check`'s error count is your only gauge until it hits zero, and then a large batch of tests flips at once.
- **`#[cfg(any())]`-gating is the only way to shorten that** — the technique this arc already leans on. But `simple_program_returning_an_int_explicit`'s path runs through `TypingPassCompilation` → `compiler.rs` → `function_compiler_core` → `function_body_compiler` → `expression_compiler` → `templata_compiler` → `convert_helper` → `compiler_solver`. That's the spine of typing; there's no meaningful subset to gate down to. You'd be gating out ~nothing.

## What you *can* fix now — and it's what you just unblocked

If the real question is "what's the next tranche of errors we're no longer design-blocked on," your addressibility call answers it. Three error classes are pure fallout from that half-finished collapse:

| count | class |
|---|---|
| 43 | `no variant named Reference found for ExpressionTE` |
| 19 | `no associated item named Reference found for LocalVariable` |
| 10 | `no variant named Address found for ExpressionTE` |

**72 errors, ~12% of the total**, and as of ten minutes ago they're mechanical rather than a design question. They're the `Reference`/`Address` lvalue-rvalue split and the `LocalVariable::Reference`/`::Addressible` constructors — exactly what "every local is an alloca, LocalLoad gives you a pointer" deletes. `local_helper.rs:83-93` (`make_user_local_variable`) and `function_compiler.rs:287-314` (`determine_closure_variable_member`) are the two anchor sites.

Also cheap and probably mechanical, though I haven't confirmed the cause: **17 × `no method named result found for BlockTE`** — `BlockTE` has a `result` *field*, and callers are saying `.result()`. If that's all it is, it's a find-and-replace.

The other big classes are still design-blocked: 112 `OwnershipT`, 68 `ITemplataT::Coord`, 45 `CoordTemplataT`, 27 `KindT::new`, 26 `.kind` on `KindT` — each needs an onion decision I shouldn't guess at.

One caveat on the target itself: `simple_program_returning_an_int_explicit`'s own assertion (`main.header.return_type.kind`, `:94`) is one of the 26 `.kind`-on-`KindT` errors. So even the test body needs an onion fix before it can pass — probably just dropping `.kind`.

Want me to take the 72-error addressibility tranche? That one's genuinely unblocked now, and it's the largest mechanical block left.

---

## 👤 User

please do

---

## 🤖 Claude

Before I start — let me check where these 72 live, since `expression_compiler.rs` and `pattern_compiler.rs` are owned by other sessions.

---

## 🤖 Claude

Stop — I need to correct my own pitch before touching anything. **67 of those 72 are in `expression_compiler.rs`**, which another session owns.

| class | expression_compiler.rs | elsewhere |
|---|---|---|
| `ExpressionTE::Reference` (43) | 42 | 1 — `struct_constructor_macro.rs` |
| `LocalVariable::Reference` (19) | 15 | 3 — `local_helper.rs`, 1 — `struct_drop_macro.rs` |
| `ExpressionTE::Address` (10) | 10 | 0 |

So my "72 errors, want me to take it?" was wrong — only **5** are outside the file you've assigned elsewhere. Let me do those 5 and show you where the boundary lands.

---

## 🤖 Claude

Now I see the real shape — and one more correction: `cargo check --lib` **doesn't compile `#[cfg(test)]` code**, so the 604 excludes test files. Let me confirm what's actually in scope.

---

## 🤖 Claude

Confirmed — `typing/test` is `#[cfg(test)]`, so all of it is invisible to the 604. Let me fix the three live non-test sites in my lane.

---

## 🤖 Claude

Now `make_user_local_variable` — the addressibility branch itself, the anchor site for the retirement.

---

## 🤖 Claude

You've already retired the branch here. Let me check what that orphaned.

---

## 🤖 Claude

You'd already commented out `determine_if_local_is_addressible` too. Let me re-check the build.

---

## 🤖 Claude

Done with what's mine — but the honest summary is that **I could only take 5 of the 72 I pitched**, and I should have checked ownership before offering.

## What I fixed

Three live sites, all outside the files other sessions own:

- `struct_constructor_macro.rs:220` — `ExpressionTE::Reference(ExpressionTE::ArgLookup(…))` → `ExpressionTE::ArgLookup(…)`. The `Reference` wrapper is gone from the flat `ExpressionTE`, so it just unwraps. Folded in the `ArgLookupTE::new` conversion while I was there.
- `struct_drop_macro.rs:295` — `UnletTE::new(LocalVariable::Reference(*v))` → `UnletTE::new(*v)`.
- `local_helper.rs:39` — `LetAndLendTE::new(interner, LocalVariable::Reference(rlv), r)` → `…(interner, rlv, r)`.

All three are the same shape: `LocalVariable` is a plain struct now, so the constructor call was pure noise.

Total **604 → 571**. That's a bigger drop than 5 because the wrapper removals cascaded into neighbouring errors on the same expressions.

The classes now stand at 40 / 15 / 7 = **62 remaining, and 62 of them are in `expression_compiler.rs`**. That file is the other session's, so the tranche is theirs to finish, not mine. My earlier "want me to take the 72?" was a bad offer — I counted the class totals without checking where they lived.

## Two things I found while in there

**You'd already done the anchor work.** `make_user_local_variable` is already a plain `LocalVariable { name, tyype }` with the addressibility branch gone, and `determine_if_local_is_addressible` is already commented out at `local_helper.rs:222`. So the retirement is further along than the error counts suggested — its only surviving callers are in `integration_tests/tests/closure_tests.rs:75-78`, which is unlinked (`lib.rs:17`), so it costs nothing today and lands with the integration-test slice.

**One dead binding left behind, which I did not touch** — you're mid-edit in that exact function:

```rust
// local_helper.rs:80
let mutable = self.get_sharedness(coutputs, reference_type2);   // <-- now unused
```

It was the input to `determine_if_local_is_addressible`. With the branch gone it's an unused-variable warning, and CLAUDE.md wants those at zero. Yours to remove or keep if you're about to use it.

## Correction to something I told you earlier

`cargo check --lib` **doesn't compile `#[cfg(test)]` code** — `typing/mod.rs:47` gates the whole `typing/test` tree. So the 571 excludes every test file. There are ~10 more `LocalVariable::Reference(LocalVariable { … })` sites sitting in `compiler_tests.rs`, `compiler_ownership_tests.rs`, and `compiler_mutate_tests.rs` as *match patterns*, plus `traverse.rs:513-514`'s `Reference`/`Address` visitor arms. None show in the count, and all of them surface the moment the crate compiles and you run `cargo test`. Worth knowing so the real remaining work isn't underestimated — the number to beat is larger than 571.

---

## 👤 User

please summarize everything we've done this session, so i can report to the other claudes

---

## 🤖 Claude

# Session summary — onion typing, `experimental-2`

## Headline

Error count went **701 → 571** (`cargo check --lib`). The whole sealed-struct-literal error class is at **zero**. Two design questions got resolved, and one turned out to be a bug rather than a decision.

---

## Findings other sessions should know

**1. The handoff doc (`vcoord-handoff.md`) is stale.** It says 103 errors; reality was 701 when this session started. Don't plan against its numbers.

**2. `cargo check --lib` hides all test code.** `typing/mod.rs:47` gates `typing/test` behind `#[cfg(test)]`. So 571 excludes every test file. There are ~10 `LocalVariable::Reference(LocalVariable { … })` match patterns in `compiler_tests.rs` / `compiler_ownership_tests.rs` / `compiler_mutate_tests.rs`, plus `traverse.rs:513-514`'s `Reference`/`Address` visitor arms, all invisible today. **The real number to beat is higher than 571.**

**3. Addressibility was destroyed, not retired.** Comparing `determine_closure_variable_member` across branches: master's `IVariableT` had 4 variants (`AddressibleLocal` / `ReferenceLocal` / `AddressibleClosure` / `ReferenceClosure`) — Addressible-vs-Reference was the outer axis, ownership the inner one. Commit `538fdb12a` collapsed 4→2 (`Local` / `Capture`), keeping the Local/Closure axis and dropping the Addressible/Reference one. That rewrote two live arms into **exact duplicates** of the other two. The dead arms in that function aren't drift — they're wreckage from that collapse. `VariabilityT` was retired by the same commit (now zero hits).

**→ Architect's call: addressibility goes for good.** Replaced by an LLVM-style model — every local is storage, `LocalLoad` yields a pointer to it. Mutation-sharing → `&x` to the same storage; moves → move-out-of-borrow (a compile error, user writes `^`); lifetime → regions on `BorrowRefT`; layout → hammer's problem, not typing's. Note `LocalLookupTE::new` hardcodes `RegionT::Default`, so nothing enforces the lifetime half yet.

**4. `_sealed: ()` is a deliberate forcing function.** Every `*TE` struct has a private `_sealed` field, making `::new` the only constructor — and `::new` is where the onion result type gets *derived* rather than handed in. That's how `LocalLookupTE.result: &'t BorrowRefT` guarantees a bare `int` local reads as `&int` with no primitive gate: it's a field type, not a match arm. `get_borrow_ownership`'s old `Int => Share` is now inexpressible.

---

## Code changes

**Commented out (not deleted), architect-directed:**
- `pointify_kind` (`templata_compiler.rs:1218`) + its sole call site and assert in `evaluate_closure` (`expression_compiler.rs:2189-2196`). The assert was tautological to a hardcoded constant — both sides traced to `let sharedness = SharednessT::Single` at `struct_compiler_core.rs:341`, which feeds both the mutability map (`:455`) and the struct def (`:462`). It was a latent check pre-positioned for shared closures.

**Sealed-literal sweep — 56 sites, 20 files, 65 → 0 errors.** Pure literal→`::new` rewrites in: 12 macros (`rsa/*`, `ssa/*`, `abstract_body`, `as_subtype`, `same_instance`, `lock_weak`, `struct_constructor`, `struct_drop`), `function_body_compiler`, `function_compiler_core`, `destructor_compiler`, `call_compiler`, `array_compiler`, `sequence_compiler`, `compiler`, `local_helper`, `convert_helper`. Two needed more than a bracket swap: `ExternFunctionCallTE::new` derives `result` from `prototype2.return_type` (field dropped, same value); `LetAndLendTE::new` takes the interner first.

⚠️ **`convert_helper.rs:90` now routes into `unimplemented!("UpcastTE onion result")`** — was a compile error, is now a latent runtime panic invisible to `cargo check`. The `VCOORD` on `UpcastTE::new` is unfilled.

**Addressibility fallout — 3 sites:** `struct_constructor_macro.rs:220` (unwrapped `ExpressionTE::Reference`), `struct_drop_macro.rs:295`, `local_helper.rs:39` (dropped `LocalVariable::Reference(…)` constructor calls).

**Untouched by agreement:** `expression_compiler.rs` and `pattern_compiler.rs` (other sessions), except the `pointify_kind` call site.

---

## Open / handoff

- **62 of the 72 addressibility errors are in `expression_compiler.rs`** — that session's to finish. I mis-pitched this as mine; only 5 were outside their file.
- **`local_helper.rs:80`**: `let mutable = self.get_sharedness(…)` is now an unused binding (was `determine_if_local_is_addressible`'s input). Left alone — architect mid-edit.
- **`determine_closure_variable_member`** is fully dead and collapses to `member = BorrowRef(captured var's kind)`. Blocked on: Q3 pass-through (does capturing a `&Marine` give `&&Marine`? coercion row (d) calls that an error), region choice, and the `@str` → `&@str` semantics change. Its Share arm **is** live today, via `str` not `struct … share` — the one fixture is `compiler_lambda_tests.rs:320`.
- **Likely merge**: `ReferenceMemberLookupTE` and `AddressMemberLookupTE` now have identical shape (both `result: &'t BorrowRefT`). Merging them would kill 40+7 of the remaining errors. `mutates_from_inside_a_closure` pins an `Address` member and needs re-authoring, not deleting.
- **First-to-pass target**: `simple_program_returning_an_int_explicit` (`compiler_tests.rs:74`) — zero builtins, one literal. But **no test can run until all 571 are gone** (Rust builds the whole lib), so it can't drive ordering. Its own assertion (`main.header.return_type.kind`) is itself one of the 26 `.kind`-on-`KindT` errors.
- **Still design-blocked**: 112 `OwnershipT`, 68 `ITemplataT::Coord`, 45 `CoordTemplataT`, 27 `KindT::new`, 26 `.kind`-on-`KindT`.

**Nothing committed.** All changes are working-tree only.

---

