<!-- session: 69f83eee-2c90-402d-a8d7-dbd79fe0517c; exported-bytes: 3294573; transcript: /Users/verdagon/.claude/projects/-Volumes-V-Vale2/69f83eee-2c90-402d-a8d7-dbd79fe0517c.jsonl -->
# Claude Conversation Log

Session ID: 69f83eee-2c90-402d-a8d7-dbd79fe0517c
Date: 2026-07-10 03:40:30

---

## 👤 User

please read vcoord-handoff.md and /tmp/claude-conversation-2026-07-09-7c2d3839.md in full, no skipping any parts please.

---

## 🤖 Claude

I'll read both files in full.

---

## 🤖 Claude

Both files were truncated. Continuing to read the remaining portions of each, as requested.

---

## 🤖 Claude

I've now read both files in full — `vcoord-handoff.md` (702 lines) and the prior session log (2011 lines). I'm caught up. Here's my consolidated understanding.

## The arc: onion typing on `experimental-2`

`Kind` gains four nestable ref variants (`BorrowRef`/`HeapOwnRef`/`ShareRef`/`WeakRef`); `Coord` dissolves; ownership becomes structural rather than a flat tag. Share-flavoredness is intrinsic to the citizen. The payoff: `&&T` becomes distinct from `&T`, so the three-blanket typeclass design (clone/drop/eq/hash — borrow blanket, share blanket, user-owned) resolves unambiguously. Landed slices: parser → postparse → higher_typing retirement → typing re-link + mechanical prep. The handoff's baseline is typing/ **RED** with the architect driving the semantic cascade.

## What the prior session (2026-07-09) actually did

It started by finishing the **reviewer-comment cleanup** (stripping `// Slice N:` / plan-name / "will be added later" timeline references; inline-into-match test cleanups), then turned into a **surface-syntax + de-duplication pass**:

1. **Retired `[#N]T` static-array *type* syntax → `StaticArray<N, T>`** (size first, element second) — since `StaticArray<N,T>` is an ordinary generic that already lowered to identical postparse output, this was net deletion. Then **removed** the interim `RetiredStaticArrayTypeSyntax` error (no back-compat needed — `[#2]int` now falls through to generic `BadTypeExpression`). Fixtures migrated (`arrays.vale`, `migrate.vale`, test programs).
2. **Unified `translate_templex` into the single split-capable walker** — deleted the ~100-line `translate_pattern_type_into_runes` fork; added `emit_ref_layer` as the one ref-emission site; `translate_templex` gained an optional `outer_shape_builder`. The `ParameterS` outer/named split you asked to keep is preserved and now feeds through one path (consumers: solver treats named type as opaque, anon-interface macro, eventually return position).
3. **Tests converted to match-style** (shape-in-pattern over assertion-piles); **audited and deleted 6 redundant StaticArray tests** (zero coverage loss — they were re-testing generic-call machinery); added a valec-reviewer rule: **no `|` in a test's match pattern**.
4. **Reverted the destructure-first path** (`[a, b] T`) — that order isn't Vale (type comes *before* the destructure: `T[a, b]`). `pattern_parser.rs` is now back to HEAD (no functional change); param-destructure tests rewritten to type-first form. The desugar feature itself is untouched.
5. **Cleaned up the clear-and-re-derive double-walk** (#1) in `function_scout.rs`'s param loop.

## Where it stopped

- **Suite green at 501/0/1, zero warnings**, but with **`typing`/`tests`/`solver` gated out in `lib.rs`** so parsing+postparse compile alone. This gating is a session-local step, not the handoff's intended end state (the handoff has typing/ linked-but-red).
- **Everything uncommitted** — nothing committed since `978bc0736`. Awaits `fire commit` / `fire commit temporary`.
- **One known-awkward spot left deliberately:** `inner_kind_rune` reconstructed by peeking `outer_shape_rules.first()` in `function_scout.rs` (#2) — a tradeoff from not rippling `translate_templex`'s return type across ~15 callers.
- Session ended on **"Holding."**

The working-tree state you gave me at session start matches this exactly (parser AST files, `function_scout.rs`, `ast.rs`, `names.rs`, the two `.vale` fixtures, `valec-reviewer.md`, `lib.rs` gating, plus the untracked `fire-commit-config.md`).

I'm caught up and holding. Ready for your direction.

---

## 👤 User

1. can you please add a rule to valec-reviewer.md that we should avoid early-returns as much as possible? equivalent branches should be next to each other, at the same indentation. for a good
  example of what im talking about, see the added chunk in templex_scout.rs; we took the code *out* of the main match statement and hacked them to be special up above. early-returns are bad because
  they lose the signal about what are equivalent operations. previously, these were in the same match statement, so we could assume that the purpose of these cases were analogous to the purpose of
  the other case statements (they were), and now we've signaled the exact opposite (that these have a different purpose).
  2. can you please add a rule to valec-reviewer.md that if at all possible, we should arrange our diff chunks so that its as easy as possible to review. if we rewrite someting, rewrite it in roughly
  the same spot in the file as it used to be. it's easier for reviewers.

---

## 🤖 Claude

I'll read the current `valec-reviewer.md` to match its house style, and look at the `templex_scout.rs` chunk you're referencing so rule #1's example is accurate.

---

## 🤖 Claude

The `templex_scout.rs` diff confirms exactly your point #1: the four ref-layer cases (`BorrowRef`/`HeapOwnRef`/`ShareRef`/`WeakRef`) used to be arms *inside* the main `match` — siblings of `Call`, `RuntimeSizedArray`, etc. — and the unification hoisted them into an `if let … { return … }` block above the match, signaling "these are special/different" when they aren't.

I'll append both rules in the doc's house format (≤30 words prose + concise BEFORE/AFTER).

---

## 🤖 Claude

Both rules added to `valec-reviewer.md`, in the doc's house format (≤30-word prose + concise BEFORE/AFTER):

1. **"Avoid early-returns; keep equivalent branches together"** — captures your point exactly: sibling match arms read as analogous operations, so hoisting some into an early-return above the match falsely signals they have a different purpose. The BEFORE/AFTER is the distilled `templex_scout.rs` situation — ref layers pulled out into an `if let … return` vs. sitting as siblings inside the match at the same indentation.

2. **"Arrange the diff to be easy to review"** — rewrite in roughly the same spot the original lived, so the reviewer sees one in-place old-against-new hunk rather than an unrelated deletion + addition across the file. Uses a `diff`-block BEFORE/AFTER since the rule is inherently about diff layout.

I kept both generic rather than naming `templex_scout.rs` directly, consistent with the doc's own "no timeline/historical references" style — file-specific citations rot. Say the word if you'd rather one of them cite the concrete site.

Note: rule #1's principle ("keep equivalent branches together") arguably calls for un-hoisting that ref-layer block in `templex_scout.rs` back into the match — but that's the very unification we just landed to kill the fork, so I've left the code alone. Flagging it in case you want to revisit that structure now that the rule's on paper.

---

## 👤 User

can we not put that back in the match?

---

## 👤 User

would it be better to have two layers of functions? one that does special things for its outer layers, and a normal one. and the former calls the latter. and people have the choice of which to call

---

## 🤖 Claude

Yes — I think it's better, and it's the clean way to un-hoist that ref block without walking back into the fork we just deleted. Two named functions:

```rust
// normal walker — every rule goes into rule_builder
pub fn translate_templex(.., rule_builder, context_region, templex) -> RuneUsage

// signature-position walker — outer ref layers → outer_shape_builder, named type → rule_builder
pub fn translate_signature_templex(.., rule_builder, outer_shape_builder, context_region, templex) -> ..
```

`translate_signature_templex` peels the outermost run of ref layers into `outer_shape_builder` (recursing into itself), and the moment it hits a non-ref node it delegates the named-type root to plain `translate_templex`. Both keep calling the shared `emit_ref_layer`, so rule *construction* is still written once — the two functions differ only in *routing*, which is exactly their reason to be two functions.

**What it dissolves:**

1. **The early-return hoist** — the four ref cases go back inside `translate_templex`'s match as siblings of `Call`/`RuntimeSizedArray`/etc., at the same indentation. This is precisely the rule we just wrote, so the code would exemplify it instead of contradicting it.

2. **The `Option<&mut Vec>` param + its ~15 `None` args** — gone from `translate_templex`'s signature and every non-split callsite. The prior session's diff had to sprinkle `None` across ~15 sites; this removes all of them.

3. **Issue #2 from last session for free** — the awkward `inner_kind_rune` reconstruction by peeking `outer_shape_rules.first()`. We avoided returning `(full, inner)` from `translate_templex` only because it would ripple the return type across ~15 callers. But `translate_signature_templex` has exactly **one** caller (the param loop), so it can just return `(full_rune, inner_kind_rune)` directly. Both awkward spots the prior session flagged disappear together.

**The one honest caveat:** the four ref arms in `translate_templex` each echo a ~4-line scaffold (eval range → recurse inner → `emit_ref_layer`), and `translate_signature_templex`'s peel echoes that shape once more. But that's not the fork's duplication — construction is shared in `emit_ref_layer`; this is just the legible "these are four analogous operations shown as four analogous arms" parallelism that rule #1 is asking for. Much smaller cost than either the fork (fully duplicated) or the hoist (violates the rule).

Net: it's the better factoring on both the rule we just codified and the two loose ends from last session.

This is a refactor of code that's currently green and uncommitted, so I don't want to start unilaterally — want me to do it? It's low-risk; the three split-routing tests (`no_outer_wrap`/`single_ref_wrap`/`nested_ref_wrap`) plus the full parse/postparse suite will confirm behavior is unchanged.

---

## 👤 User

proceed

---

## 🤖 Claude

I'll implement it. Let me read the current state of the relevant code precisely before editing.

---

## 🤖 Claude

Now let me find every call site of `translate_templex` across the codebase, so I know which `None` args to drop.

---

## 🤖 Claude

Let me read the param-loop caller in `function_scout.rs` to see exactly what it needs from the split (including the `inner_kind_rune` derivation).

---

## 🤖 Claude

Now I have the full picture. Let me read the external callers to see the exact `None`-argument placement I'll be removing.

---

## 🤖 Claude

All callers mapped. Now I'll make the edits. Starting with `templex_scout.rs` — replace the doc comment + signature + hoisted block with the plain normal-walker head.

---

## 🤖 Claude

Now restore the four ref arms inside the match — placing them right where they used to live (before the `Call` arm), and drop the two `None` args in the `Call` arm at the same time.

---

## 🤖 Claude

Now drop the remaining internal `None` args in the `Func`, `RuntimeSizedArray`, `Tuple` arms, and `translate_type_into_rune`.

---

## 🤖 Claude

Now add the new `translate_signature_templex` function, placed right after `translate_templex` (it delegates to both `translate_templex` and `translate_type_into_rune`).

---

## 🤖 Claude

Guardian's **NNDX** shield (no new definitions without a Scala counterpart) blocked inserting the new `translate_signature_templex` function — the same Scala-parity shield that fired in the prior session. The file was **not** modified by that call.

Important: the *rest* of the refactor already landed before this block — `translate_templex` has lost its `outer_shape_builder` param, the four ref arms are back in its match, and the internal/`None`-arg cleanups are done. So the tree is temporarily **non-compiling**: `function_scout.rs` still calls the old 8-arg signature, and `translate_signature_templex` doesn't exist yet. That's a transient half-applied state — the only thing standing between here and a clean compile is adding that one new function (blocked) and repointing the callers.

Ordination is per-session, so this fresh session isn't ordained. Two ways forward:

- **Ordain this session** — you re-run `/guardian-ordain "thunder road" <port>` (the port may differ from last session's `7880`), then I insert the function and finish the caller repointing + verify.
- **I hold** and you tell me how you'd rather handle it.

How do you want to proceed?

---

## 👤 User

<command-message>guardian-ordain</command-message>
<command-name>/guardian-ordain</command-name>
<command-args>"thunder road" 7880</command-args>

---

## 🤖 Claude

I'll register this session with Guardian using the password and port you provided.

---

## 🤖 Claude

Ordained (`{"success":true}`). Now inserting the `translate_signature_templex` function.

---

## 🤖 Claude

Now update the external callers to drop the `None` arg. Doing `post_parser.rs` (3 edits — the struct-member block is byte-identical in both the normal and variadic arms, so `replace_all` handles both).

---

## 🤖 Claude

Now `expression_scout.rs` (3 edits) and `rule_scout.rs` (1 edit).

---

## 🤖 Claude

Now the main caller — `function_scout.rs`. Update the import, the bucket comment, and rewrite the param-loop call to use `translate_signature_templex` (getting `(full, inner)` directly, dropping the `.first()` peek).

---

## 🤖 Claude

All edits applied. Now build the lib (typing/tests/solver are gated out per the prior session, so this compiles parsing+postparse).

---

## 🤖 Claude

Clean build, zero warnings. Now the full lib test suite.

---

## 🤖 Claude

Green — **501 passed, 0 failed, 1 ignored, zero warnings** (same count as before the refactor). Let me explicitly confirm the three split-routing tests ran and passed, since they directly exercise this path.

---

## 🤖 Claude

`cargo test` takes only one positional filter. Let me find the actual test names, then run them by a shared substring.

---

## 🤖 Claude

The shared substring is `test_param_` and `_wrap_routing`. Running by `wrap_routing`:

---

## 🤖 Claude

All three split-routing tests pass — the split output (no-wrap / single-ref / nested-ref chaining) is byte-identical through the refactor. Final sanity sweep for any lingering old-signature references:

---

## 🤖 Claude

Done — the two-function refactor is complete and verified.

## What landed

**`translate_templex` is back to a plain walker; `translate_signature_templex` is the named split.**

- **`translate_templex`** lost its `outer_shape_builder: Option<…>` param. The four ref cases (`BorrowRef`/`HeapOwnRef`/`ShareRef`/`WeakRef`) are **restored as sibling arms inside the match**, at their original spot (right before the `Call` arm) — no early-return hoist. Every rule goes into `rule_builder`. This is exactly the shape the rule we just wrote is asking for: equivalent branches at the same indentation, reading as analogous operations.
- **`translate_signature_templex`** (new) peels the outermost run of ref layers into `outer_shape_builder`, then delegates the named-type root to `translate_type_into_rune` — and returns **`(full_rune, named_type_rune)` directly**. Both walkers still call the shared `emit_ref_layer`, so rule construction is written once; the two functions differ only in *routing*, which is their reason to exist.
- **~15 non-split callers** (`post_parser.rs`, `expression_scout.rs`, `rule_scout.rs`, and the internal recursions) dropped their `None` argument — cleaner signatures across the board.
- **The param loop** (`function_scout.rs`) now calls `translate_signature_templex` and takes `(full, inner)` straight from the return value. **Issue #2 from last session is dissolved** — the awkward `param_outer_shape_rules_vec.first()` peek (and its `panic!("outer_shape_rules only contains ref-layer rules")`) is gone, because the single-caller split function can return the inner rune without rippling `translate_templex`'s return type across the other callers.

## Verification

- `cargo build --lib` — clean, zero warnings.
- Full lib suite — **501 passed, 0 failed, 1 ignored, zero warnings** (identical to pre-refactor: behavior preserved, not just compiling).
- The three split-routing tests (`test_param_no_outer_wrap_routing` / `_single_ref_wrap_routing` / `_nested_ref_wrap_routing`) pass — confirming the outer-shape/named-type split and the ref-chain ordering are byte-identical to before.
- Grep confirms no stale 8-arg call sites remain; `translate_signature_templex` has exactly one external caller (the param loop), as intended.

Both awkward spots the prior session flagged (the early-return hoist and the `.first()` reconstruction) are now gone together. Everything remains **uncommitted** on the working tree, pending your `fire commit`. One process note: this session needed Guardian ordination (NNDX fired on the new function) — you re-ordained on port 7880, `{"success":true}`.

---

## 👤 User

whats emit_ref_layer?

---

## 🤖 Claude

`emit_ref_layer` is a small helper in `templex_scout.rs` (`FrontendRust/src/postparsing/rules/templex_scout.rs:114`) — it's the single place that constructs any of the four ref-layer rules.

```rust
fn emit_ref_layer<'s>(
  scout_arena, lidb,
  builder: &mut Vec<IRulexSR<'s>>,   // where to push the rule
  kind: RefKind,                     // Borrow | HeapOwn | Share | Weak
  range_s,
  inner_rune: RuneUsage<'s>,         // rune of the type this layer wraps
  region_rune: Option<RuneUsage<'s>>,// only Borrow carries one
) -> RuneUsage<'s>
```

It does three things:
1. **Mints a fresh result rune** for this layer (an implicit rune).
2. **Builds the matching `IRulexSR` variant** off `kind` — `BorrowRefSR` / `HeapOwnRefSR` / `ShareRefSR` / `WeakRefSR`. Borrow gets the `region_rune`; the other three don't have a region field.
3. **Pushes that rule into `builder`** and returns the new result rune.

So given "I have a rune for the inner type, and this is a `&` layer wrapping it," it produces the rune for the wrapped type and records the rule that says "this rune = a borrow of that inner rune."

**Why it exists:** it's the shared construction site that both walkers call — `translate_templex`'s four ref arms (which pass `rule_builder`) and `translate_signature_templex`'s peel loop (which passes `outer_shape_builder`). The *only* difference between the two walkers is which vec they hand to `emit_ref_layer`; the actual rule-building is written once here. That's what let the two-function split avoid reintroducing the duplicated fork we deleted last session — the routing differs, but the construction doesn't.

It was introduced in the prior session's `translate_templex` unification (not this session), and this session's refactor leaned on it precisely because it already centralized the construction.

---

## 👤 User

inline it please

---

## 🤖 Claude

Quick flag before I do it, because this bumps into what we just agreed:

`emit_ref_layer` is called from **5 sites across both walkers** — the four ref arms in `translate_templex` plus the peel in `translate_signature_templex`. Inlining copies the "mint a result rune, build the `*RefSR`, push it" logic into all of them. And because `translate_signature_templex` dispatches on a *dynamic* `kind`, its inlined copy needs its own 4-way match on `RefKind`. So the ref-rule construction lands back in **~8 places** (4 static arms + a 4-branch match) across the two functions — which is the fork-level duplication we deleted last session and kept `emit_ref_layer` specifically to centralize.

I can't inline it without reintroducing that, because the two functions route the same construction into two *different* vecs — that's the exact thing the shared helper factors out.

So — two honest readings, your call:

- **You still want it inlined** (you're fine with the construction living in both walkers) — say so and I'll do it.
- **The thing bugging you is the `RefKind` round-trip** — matching the templex to produce a `RefKind` tag, then matching the tag back to a rule. That's a real smell, but the fix is different (e.g. drop `RefKind` and have `emit_ref_layer` take the built rule, or collapse the dispatch another way) and it *keeps* one construction site. If that's the target, tell me and I'll go that direction instead.

Which one?

---

## 👤 User

wouldnt translate_signature_templex call itself recursively until it gets to a non-ref, and then just call translate_templex?

---

## 🤖 Claude

Good instinct — and it already works exactly like that. `translate_signature_templex` recurses on itself through the ref layers and, at the first non-ref node, delegates. So the *peeling* structure is precisely what you describe.

But the recursion handles **depth** (nested `&&`, `&heap`), not **construction**. On the way back up, each peeled layer still has to *build its `*RefSR` rule into `outer_shape_builder`* — and that build is the whole of `emit_ref_layer`. The recursion sequences the layers; it doesn't remove the per-layer rule construction. Concretely, the ref case is:

```rust
Some((kind, inner, region_pt)) => {
  let (inner_rune, named) = translate_signature_templex(.., inner);   // ← recurse-until-non-ref (your point)
  let region_rune = region_pt.map(..);
  let full = emit_ref_layer(.., outer_shape_builder, kind, ..);       // ← still has to build THIS layer's rule
  (full, named)
}
None => translate_type_into_rune(.., templex)                         // ← delegate at the bottom
```

That `emit_ref_layer` call is the thing at issue. `translate_templex`'s four ref arms build the *same* `BorrowRefSR`/`HeapOwnRefSR`/… rules, just into `rule_builder` instead of `outer_shape_builder`. So the two functions share one construction, differing only in the target vec. Inline it and that construction gets spelled out in both — the 8 copies. The recursion doesn't change that, because it's orthogonal: peeling ≠ building.

So your restructuring is real and it's already in place; it just doesn't dissolve `emit_ref_layer`.

Two footnotes:
- The bottom currently calls `translate_type_into_rune`, not `translate_templex` directly. `translate_type_into_rune` is a thin wrapper that returns a bare declared-rune at the root *without* a lookup rule, then delegates everything else to `translate_templex`. That shortcut is the bare-rune-at-root divergence the prior session flagged and deliberately preserved — "just call `translate_templex`" would change that one behavior.
- If the real irritant is the `RefKind` round-trip (match the templex → tag → match the tag → rule), that's separable and I can kill it while keeping one construction site. That's a cleaner target than inlining, which reintroduces the duplication.

My lean is to keep `emit_ref_layer`. But if you'd still rather inline and accept the two construction sites, say so and I'll do it.

---

## 👤 User

i still feel like translate_templex can inline it, its even hardcoding the kind: RefKind

---

## 👤 User

i still feel like translate_templex can emit_ref_layer it, its even hardcoding the kind: RefKind

---

## 👤 User

i still feel like translate_templex can inline emit_ref_layer, its even hardcoding the kind: RefKind

---

## 🤖 Claude

You've actually put your finger on the exact crux, and it cuts both ways: **`RefKind` exists *precisely* to avoid the duplication I keep flagging.** They're the same tension from two ends.

Here's why. In `translate_templex` you're right — each arm hardcodes its `RefKind`, so it's threading a constant into a switch that immediately switches back. Pure ceremony there. So yes, `translate_templex` alone would inline cleanly into four direct arms.

But `translate_signature_templex` builds those *same four rules* into a different vec (`outer_shape_builder`), and its kind is **dynamic** (it computes it from the templex it's peeling). So:

- To fully kill `RefKind`, `translate_signature_templex` also has to match the templex directly with four arms.
- Now the `BorrowRefSR`/`HeapOwnRefSR`/… construction is spelled out in **both** functions' four arms — which is exactly the shape of the fork we deleted last session (that fork *was* the four ref arms).

`RefKind` + `emit_ref_layer` is the thing that collapses those two sets of four arms into one construction site. Remove it and the four arms come back in both places. So "inline it / drop `RefKind`" and "don't re-fork" are genuinely mutually exclusive — you can have one.

The honest mitigation on the inline side: these are **compiler-checked struct literals**, not behavioral logic. Add a field to `BorrowRefSR` and all sites fail to compile — you can't silently drift. So the "single source of truth" argument is weaker than for normal duplicated logic; the real cost is just eight short arms instead of four thin ones + a helper.

So it's a straight taste call:

- **Keep `emit_ref_layer`** — one construction site, at the price of the `RefKind` ceremony you're objecting to.
- **Inline it** — four direct, self-contained arms in each function, no `RefKind`, at the price of the four ref arms reappearing in both (the fork shape, but lighter and compiler-guarded).

You've pushed toward inline three times and the `RefKind` smell is real — I'm happy to do it. Since it re-forks the thing we deliberately un-forked, I just want the explicit nod: **inline it, four arms in both, `RefKind` and `emit_ref_layer` gone?**

---

## 👤 User

inline it now please, i want to see it

---

## 🤖 Claude

Doing it now — inlining the construction into direct arms in both functions and deleting `RefKind` + `emit_ref_layer`. First, delete the helper and enum.

---

## 🤖 Claude

Now inline the four ref arms in `translate_templex`.

---

## 🤖 Claude

Now rewrite `translate_signature_templex` with four direct ref arms (dropping `maybe_ref` + `RefKind`).

---

## 🤖 Claude

Now build and test.

---

## 🤖 Claude

Clean build, zero warnings, **501/0/1**. Confirming the split-routing tests and that no `RefKind`/`emit_ref_layer` references remain.

---

## 🤖 Claude

Done — inlined, `RefKind` and `emit_ref_layer` deleted, suite green (501/0/1, zero warnings, three split-routing tests pass). Here's what it looks like now.

**`translate_templex`** — the four ref arms are direct and self-contained, sitting as siblings alongside `Call`/`RuntimeSizedArray`/etc. Each builds its own rule; no constant threaded through a switch:

---

## 🤖 Claude

And **`translate_signature_templex`** — now four direct ref arms too (peels into `outer_shape_builder`, recurses via itself, propagates the named-type root up), with a `_ =>` delegating the root:

---

## 🤖 Claude

There it is. The shape now:

- **`RefKind` enum and `emit_ref_layer` are gone.** No more matching a templex into a tag and matching the tag back to a rule.
- **`translate_templex`** — four direct ref arms in its match, each minting its rune and pushing its own `BorrowRefSR`/`HeapOwnRefSR`/`ShareRefSR`/`WeakRefSR` into `rule_builder`, then returning the rune. Reads straight top-to-bottom, no indirection.
- **`translate_signature_templex`** — the same four ref arms, but pushing into `outer_shape_builder`, recursing via itself to keep peeling, threading the named-type root up, and a `_ =>` that delegates the root to `translate_type_into_rune`. The `maybe_ref` normalization is gone.

**The tradeoff, now realized as you wanted to see it:** the `*RefSR` construction is spelled out in both functions — eight arms total. That's the fork shape we deleted last session, but much lighter than the old fork: each arm is ~7 lines (the verbose old arms were ~20), the rune-minting matches the file's existing idiom (the `Call`/`Tuple`/`RSA` arms mint the same way inline), and it's all compiler-checked struct literals — add a field to `BorrowRefSR` and all eight sites fail to compile, so there's no silent-drift risk. The `RefKind` ceremony you flagged is gone in exchange.

Verified: `cargo build --lib` clean, zero warnings; full suite **501/0/1**; the three split-routing tests (`no_outer_wrap`/`single_ref_wrap`/`nested_ref_wrap`) pass, so the split behavior is byte-identical; grep confirms zero remaining `RefKind`/`emit_ref_layer` references. Still uncommitted, pending your `fire commit`.

---

## 👤 User

can you make separate translate_borrow_ref_templex, and one for share, own, weak?

---

## 🤖 Claude

On it — extracting a `translate_*_ref_templex` per kind, shared by both walkers (each takes the already-recursed inner and the target builder, so `translate_templex` and `translate_signature_templex` both delegate to them). This kills `RefKind` *and* the fork-level construction duplication in one move. First, add the four functions where `emit_ref_layer` used to live.

---

## 🤖 Claude

Now update `translate_templex`'s four ref arms to delegate to the new functions.

---

## 🤖 Claude

Now `translate_signature_templex`'s four ref arms.

---

## 🤖 Claude

Build and test.

---

## 🤖 Claude

Green — build clean, zero warnings, **501/0/1**. Confirming the split-routing tests:

---

## 🤖 Claude

All three split-routing tests pass. Here's the result — the four per-kind functions:

---

## 🤖 Claude

Done — this actually resolves the whole thread cleanly. Each of the four ref layers now has its own named function (`translate_borrow_ref_templex`, `translate_heap_own_ref_templex`, `translate_share_ref_templex`, `translate_weak_ref_templex`), and **both walkers share them** — each takes the already-recursed inner rune (plus the region for borrow) and the target builder, then mints the rune and pushes its one `*RefSR`.

So the caller arms are now thin — recurse in their own strategy, then delegate:

```rust
// translate_templex (normal walk → rule_builder):
ITemplexPT::HeapOwnRef(heap_own_ref) => {
  let range_s = PostParser::eval_range(file, heap_own_ref.range);
  let inner_rune = translate_templex(.., rule_builder, .., heap_own_ref.inner);
  translate_heap_own_ref_templex(scout_arena, lidb, rule_builder, range_s, inner_rune)
}

// translate_signature_templex (split → outer_shape_builder, threads named root up):
ITemplexPT::HeapOwnRef(heap_own_ref) => {
  let range_s = PostParser::eval_range(file, heap_own_ref.range);
  let (inner_rune, named_type_rune) = translate_signature_templex(.., outer_shape_builder, .., heap_own_ref.inner);
  let full = translate_heap_own_ref_templex(scout_arena, lidb, outer_shape_builder, range_s, inner_rune);
  (full, named_type_rune)
}
```

**This gets both things at once:**
- **No `RefKind`** — the kind lives in the function name, not a constant threaded into a switch. That's the smell you were after, gone.
- **No fork duplication** — each `*RefSR` is constructed in exactly one place (its function), shared by both walkers. So we're *not* back to the eight-arm fork; the construction is deduped again, just without `RefKind`.

The one honest residue: the four functions each carry the identical rune-minting line, and `heap_own`/`share`/`weak` differ only by which `*RefSR` variant they push — so they read as near-twins. That's trivial, compiler-checked boilerplate (this file mints runes the same way in a dozen other arms), and it's the price of "one function per kind" versus the single `RefKind`-dispatching helper. Net, I think it reads better than either predecessor.

Verified: `cargo build --lib` clean, zero warnings; full suite **501/0/1**; the three split-routing tests pass, so behavior is byte-identical. Still uncommitted, pending your `fire commit`.

---

## 👤 User

please look at all the added lines containing `// V` in this diff, including unstaged

---

## 🤖 Claude

These are architect review markers. Let me get their exact file locations so I can read each in context.

---

## 🤖 Claude

These are architect review markers you've left in the working tree. Let me confirm precisely which are **added** (vs pre-existing) and read each in context.

---

## 🤖 Claude

Now let me read each in context.

---

## 🤖 Claude

I've read all 18 added `// V`-family markers in context. They're your review annotations on this session's work. Here's the organized catalog, grouped by what each is actually asking.

## 1. Field renames in `ParameterS` (`ast.rs`) — clear directives
A consistent terminology shift: **"full type"** (with wraps) vs **"value type"** (the bare named root).
- `full_kind_rune` → `full_type_rune` (373)
- `inner_kind_rune` → `value_type_rune` (376)
- `outer_shape_rules` → `full_type_rules` (381)
- `named_type_rules` → `value_type_rules` (385)
- Then "make the rest consistent with that terminology" (387) — the local vars in `function_scout.rs` (`param_outer_shape_rules_vec` etc.), the `translate_signature_templex` doc, and the test field accesses.
- Plus write **arcana justifying the split** (388, and 1447 "need arcana outer shape etc.").

## 2. Reviewer-doc rules to codify (the `VADD` "add a rule" ones)
- **`ast.rs:407`** — add a rule: impossible data combinations should be outlawed via the type system (ask before changing types) *or* guarded with asserts like the `debug_assert!` below it.
- **`post_parser_tests.rs:1419`** — don't use `.any` in test asserts; use the collect-where macro.
- **`post_parser_tests.rs:1420`** — only allow asserts that aren't expressible via match statements (and those matches can live in collect macros).
- **`function_scout.rs:447`** — add a rule: if it's not immediately obvious what leads into a match arm, that arm needs a very concise comment saying what gets you there.
- **`function_scout.rs:769`** — add a rule: **no em-dashes in comments.** (Heads up: I use em-dashes heavily — this rule flags many of my added comments this session, e.g. the `translate_signature_templex` doc, not just line 771.)

## 3. Comment/prose fixes (on comments I wrote)
- **`function_scout.rs:447–448`** — the param-split comment (444–446) is too abstract and lost the old comment's clarity about *which case* leads into the branch; needs a concise example.
- **`function_scout.rs:769`** — remove the em-dash from the LetSE comment.
- **`post_parser_tests.rs:1380`** — prose pass on the split-routing doc comment.

## 4. Design questions — need your direction, not unilateral edits
- **`function_scout.rs:377`** — "get rid of `rune_to_explicit_type`." A real removal, threads through several sites.
- **`function_scout.rs:432`** — "why do we calculate pattern rules ever, and why is it OK to throw them away for params?" (This is the `discarded_pattern_rules` throwaway we set up earlier — a genuine question about whether `translate_pattern` should emit rules at all in this path.)
- **`ast.rs:367`** — "if there's no destructure, don't generate it as a pattern — simpler AST." A `ParameterS.pattern` / `AtomSP` shape change.

## 5. Status marker — no action
- **`lib.rs:28`** VCOORD: records that typing is gated so parsing/postparse compile alone; un-gate when the typing cascade compiles. Just a state note, consistent with the arc.

---

**Proposed plan:** I can knock out categories **1 and 3** (renames + consistency + the three comment fixes) and **codify category 2** into `valec-reviewer.md` / `test-review.md` right now — those are all clear directives. Categories **4** (three design questions) I'd rather talk through before touching, since two are genuine "why does this exist" questions and one changes AST shape. And the arcana (part of 1) I'll draft once the renames land.

Want me to start on 1 + 2 + 3, and hold 4 for discussion? Or a different order?

---

## 👤 User

tell me what you understand of the VADD ones, we'll be adding rules to valec-reviewer.md about them

---

## 🤖 Claude

Good — let me ground my understanding of #4/#5 before explaining, since they reference a project-specific "collect-where macro." Let me check the test-review conventions and the full assert block for #1.

---

## 🤖 Claude

Grounded now. Here's my understanding of each `VADD`, with the principle I'd codify and the nuances I see. (I checked `test-review.md` and the `collect_` macros because #4/#5 lean on them.)

## VADD #1 — `ast.rs:407`: make illegal states unrepresentable, else assert them
**Trigger:** the two `debug_assert!`s in `ParameterS::new` — one pins that `outer_shape_rules` holds only the four ref-wrap variants, the other that `full_kind_rune == inner_kind_rune` when there are no outer wraps.

**Principle:** when a combination of data is *expected never to happen*, don't leave it silently representable. Two tiers, in order:
1. **Outlaw it in the type system** — make it unrepresentable by construction. But changing a type requires asking you first.
2. **If you can't (or didn't) narrow the type, assert it** — a `debug_assert!` with a message stating the invariant, like these.

**Nuance worth capturing:** the first assert is a live example of tier-1 being *available but not taken* — `outer_shape_rules: &[IRulexSR]` could be narrowed to a type that only holds the four ref wraps, which would delete the assert. So the rule should make clear the assert is the *fallback*, and the reviewer's job is to ask "could a tighter type have made this assert unnecessary?" before accepting it.

## VADD #2 — `function_scout.rs:447`: non-obvious match arms need a "what gets us here" comment
**Trigger:** my rewritten comment above `match (&pattern.templex, pattern_s.kind_rune.clone())` described *what the code does* but dropped the old comment's signal for *which case each arm is*. (The `_` arm still says "Untyped param (lambda `(a) => a`)"; the `(Some, Some)` arm lost its label.)

**Principle:** if it isn't immediately obvious what input/condition lands you in a given match arm, that arm needs a **very concise** comment naming that case. Paired with the `V:` on the next line (448): that comment should be **concrete — carry a tiny example** (like `(a) => a`), not abstract prose about the machinery.

So the rule is really two-in-one: (a) comment the *entry condition* of a non-obvious arm, and (b) make it concrete with an example rather than abstract.

## VADD #3 — `function_scout.rs:769`: no em-dashes in comments
**Trigger:** the LetSE comment using `—`.

**Principle:** plainly, no em-dashes in comments; restructure with commas, parens, colons, or separate sentences. Mechanical and absolute.

**Heads-up:** this is pervasive in my writing — the comments I added this session (e.g. the `translate_signature_templex` doc, the per-kind function header) are full of them. Codifying it means a sweep of my session's comments, not a one-line fix.

## VADD #4 — `post_parser_tests.rs:1419`: no `.any` in test asserts, use the collect-where macro
**Trigger:** `assert!(param.named_type_rules.iter().any(|r| is_lookup_of(r, "int")), ...)`.

**Principle:** don't hand-roll `.iter().any(|r| predicate)` in a test. Use the pattern-specific `collect_where_*!` / `collect_only_*!` macro (the `_snode` family for postparse) so the **pattern** does the matching and the **macro** does the counting/existence check. This is exactly `test-review.md` §3's first listed anti-pattern (`.any(|x| ...)` over a collected `Vec`).

## VADD #5 — `post_parser_tests.rs:1420`: asserts only for what a match can't express
**Trigger:** same assert block. This is the umbrella principle behind #4.

**Principle:** an `assert!` is only allowed when its condition **can't** be written as a match. If it *can* be a match (pinning an exact shape), it must be — and that match may live inside a collect macro. Asserts are reserved for the genuine residue: numeric comparisons, rune-equality (`full_kind_rune.rune == inner_kind_rune.rune`), etc.

**Nuance / how far it reaches:** applied to this very test, `assert_eq!(outer_shape_rules.len(), 1)` + `assert!(matches!(outer_shape_rules[0], IRulexSR::BorrowRef(_)))` are both match-expressible — they collapse to one slice pattern, `matches!(param.outer_shape_rules, [IRulexSR::BorrowRef(_)])`. So #5 isn't just "prefer matches"; it actively merges count+element asserts into a single pattern. The `.rune ==` asserts legitimately stay as asserts (not match-expressible).

---

Two things to decide before we write them into `valec-reviewer.md`:

- **#4 and #5 overlap `test-review.md` §3** ("One `collect_` per specific pattern — no post-hoc filtering," which already names `.any(|x| ...)` and "`assert_eq!(len, N)` + per-element checks → split into `collect_` calls"). So we either restate a tight version in `valec-reviewer.md` or cross-reference. #5's "assert only when not match-expressible" is a sharper framing than §3 currently has, so it may be worth stating even if it partly repeats.
- **#2 and #3 are comment-style rules** — they'd sit alongside the existing `valec-reviewer.md` comment rules (jargon/historical/timeline), though `prose-reviewer.md` is the other natural home. Your call which doc.

Want me to draft the five rule entries in the doc's house format (≤30-word prose + BEFORE/AFTER) for your review?

---

## 👤 User

2 and 3 into prose-reviewer.md, yeah.

put the rest into valec-reviewer.md, collapsing as you see fit

---

## 🤖 Claude

Let me read `prose-reviewer.md` to match its house format before drafting.

---

## 🤖 Claude

Both docs use the same house format (≤30 words + concise BEFORE/AFTER). Let me verify the exact `snode` collect-macro name so the valec example is faithful.

---

## 🤖 Claude

Let me confirm the `NodeRefS` variant for a rule so the collect-macro example is faithful (not fabricated).

---

## 🤖 Claude

Confirmed — `NodeRefS::LookupRule` and `NodeRefS::Parameter` are real variants, so the collect-macro example will be faithful. Now writing the rules.

**`prose-reviewer.md`** — #2 (match-arm case comments) and #3 (no em-dashes):

---

## 🤖 Claude

**`valec-reviewer.md`** — #1 (illegal states → types or assert) and #4+#5 collapsed (assert only what a match can't express):

---

## 🤖 Claude

Done. Four V-marker directives codified across the two docs; #4 and #5 collapsed into one rule.

**`prose-reviewer.md`** (2 new rules):
- **"Comment a non-obvious match arm with what leads into it"** (#2) — name the entry case in one concise line with a concrete example, don't describe the machinery. BEFORE/AFTER uses the actual typed-param vs untyped-lambda arms.
- **"No em-dashes in comments"** (#3) — comma/colon/parens/new-sentence instead. BEFORE/AFTER is the LetSE comment that triggered it.

**`valec-reviewer.md`** (2 new rules):
- **"Outlaw an impossible state, or assert it"** (#1) — unrepresentable-via-types first (ask before changing a type), `debug_assert!` as fallback. AFTER is the real `outer_shape_rules` assert.
- **"Assert only what a match can't express"** (#4 + #5 collapsed) — #5's umbrella as the prose, #4's `.any` ban folded in. The BEFORE/AFTER carries both conversions: `.any(...)` → `collect_only_snode!`, and the `len()==1` + `[0]` element pair → a single slice pattern `matches!(…, [IRulexSR::BorrowRef(_)])`.

Two notes:
- I verified the `collect_only_snode!` / `NodeRefS::Parameter` / `NodeRefS::LookupRule` names against the source so the example is real, not fabricated. One caveat I couldn't fully settle without a rewrite: whether the `Parameter` traversal actually descends into the `outer_shape_rules`/`named_type_rules` buckets. That only matters when we *apply* the rule to those tests (a category-4 task), not for the rule statement itself.
- The collapsed rule **overlaps `test-review.md` §3** ("no post-hoc filtering," which already flags `.any` over a collected `Vec`). I stated it anyway because #5's "assert only for what a match can't express" is a sharper framing than §3 has, and `valec-reviewer.md` is where you wanted it. Say the word if you'd rather it cross-reference §3 instead of restate.

Doc-only changes, nothing to build. The five markers (the `// V:`/`// VADD:` lines) are still in the code — want me to strip the ones we've now codified, leave them for you, or hold until we tackle the category-4 design questions?

---

## 👤 User

<command-message>prose-reviewer</command-message>
<command-name>/prose-reviewer</command-name>
<command-args>on all of our changes</command-args>

---

## 👤 User

<command-message>valec-reviewer</command-message>
<command-name>/valec-reviewer</command-name>
<command-args>on all of our changes</command-args>

---

## 🤖 Claude

Running both reviewers over the code I changed this session (primarily `templex_scout.rs` and `function_scout.rs`). The highest-yield rule against my own writing is **no em-dashes in comments** — I use them heavily. Let me find every one in the comments I added.

---

## 🤖 Claude

Found 6 em-dashes in comments I wrote, plus the abstract match-arm comment (rule #5). Let me read the exact current text of each site to fix precisely.

---

## 🤖 Claude

Now fixing the em-dashes in `templex_scout.rs` (comma/colon/period per the rule):

---

## 🤖 Claude

Now `function_scout.rs`. Let me read the two regions exactly.

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

review all uncommitted changes, not just this session

---

## 🤖 Claude

Understood — broadening to the entire uncommitted diff vs HEAD. Let me size it up and run the mechanically-detectable rules (em-dashes, timeline labels, discarded `Err` payloads, historical context) across all added lines first.

---

## 🤖 Claude

Scope is ~1,150 insertions across ~15 substantive Rust files. No timeline labels or historical-context comments slipped in (those rules are clean). Em-dashes and the judgment-based rules remain. Given the breadth, I'll fan out read-only review agents across file groups — they return findings, I apply every fix in the main conversation (agents don't edit).

---

## 🤖 Claude

All five reviewers reported. Let me verify a couple of claims before applying (the "unused imports" one contradicts my zero-warning build), and read the exact edit sites.

---

## 🤖 Claude

Verified. Now let me read the remaining exact edit sites (`pattern.rs`, and the `post_parser_tests` comment regions).

---

## 🤖 Claude

All sites confirmed. Applying the safe prose/comment fixes. Starting with `function_scout.rs` — the P5 match-arm comment restructure (dropping the abstract comment and its two now-satisfied markers), plus the two em-dashes.

---

## 🤖 Claude

Now `templex_scout.rs` (P5 concrete example), `pattern.rs` (invariant first), and the two `post_parser_tests.rs` comment fixes.

---

## 🤖 Claude

Now build + test to confirm no breakage and settle the unused-import question (compiler is the judge). Also re-sweep the whole diff for any remaining comment em-dashes.

---

## 🤖 Claude

Review complete. Suite green (**501/0/1, zero warnings**), and the em-dash sweep is clean except the one intentional em-dash in the `prose-reviewer.md` BEFORE example (that's the rule demonstrating the violation).

## Applied (safe prose/comment fixes — 5 reviewers, consolidated)
- **Em-dashes (P6): 8 total** — `templex_scout.rs` ×4, `function_scout.rs` ×2, `post_parser_tests.rs` ×2. All rewritten to comma/colon/period.
- **Non-obvious match-arm comments (P5)** — `function_scout.rs`: dropped the abstract machinery comment above the typed/untyped param match, put a concise case+example on each arm (`// A typed param, e.g. \`foo(x &int)\`.` / `// An untyped param, e.g. a lambda \`(a) => a\`.`). `templex_scout.rs`: added a concrete `&&Ship` example to the split-walk comment.
- **Invariant-first (P4/P3)** — `pattern.rs` `self_borrow` comment now states what the field holds, then the `&self` example.
- **Timeline wording (V3/V4)** — `post_parser_tests.rs:1510` "…live on their params now" → "The int and bool Lookups live on their params, not on FunctionS.rules."
- **Removed 3 markers** that a fix in this pass fully satisfied: the two `// V:`/`// VADD:` on the function_scout comment I rewrote, and `// VADD: no em-dashes` above the em-dash I fixed.

## Surfaced — your call (I did NOT apply these)

1. **`function_scout.rs` param match [V14 + V5]** — the reviewer found `match (&pattern.templex, pattern_s.kind_rune)` makes `(Some,None)`/`(None,Some)` representable even though `translate_pattern` guarantees `templex ⟺ kind_rune`, and the `_` arm silently absorbs those impossible crosses (a stray `(Some,None)` would be mistreated as untyped, dropping the type). `kind_rune` is also just a proxy for `templex` here. **Fix:** match on `&pattern.templex` alone (`Some(type_p)` → typed, `None` → untyped). Clean and verified, but it's a control-flow change, so I'm asking first.

2. **V15 test-assertion rewrites in `post_parser_tests.rs`** — this is the VADD #4/#5 cleanup we'd bucketed as category-4. Two are clean, ready to apply: `len()==1` + `[0]` `matches!` → `matches!(outer_shape_rules, [IRulexSR::BorrowRef(_)])` (1448/1450), and the two-borrow version → a slice-let (1482/1486–1490). The `.iter().any(is_lookup_of …)` existence/absence checks (1421/1426/1453/1511/1513) need me to confirm the exact rule slice a bare `int`/`bool` param emits before collapsing. Want these now, or as the deliberate test-cleanup pass?

3. **Test rename (V3/V4)** — `test_function_rules_no_longer_contains_param_rules` still has "no_longer" (timeline) in the *name*. I left it since renaming a test definition is more than a comment fix and you may want the name. Suggest `test_function_rules_excludes_param_rules`.

4. **Remaining `// V:` / `// VADD:` markers** — still in `ast.rs` (renames/arcana/407), `function_scout.rs` (377, 432), `post_parser_tests.rs` (1380/1419/1420/1447), plus the allowed `lib.rs` VCOORD. These are your design/category-4 TODOs; I only stripped the three this pass resolved. Say the word if you want the codified/addressed ones cleared too.

Two things I checked and cleared: the agent's "unused imports" flag on `post_parser_tests.rs:1387/1388` produces **no warning** (build is clean), so no action. And in `templex_scout.rs`'s BorrowRef arm, region rules going to `rule_builder` while the `BorrowRefSR` goes to `outer_shape_builder` is correct-by-design (region belongs with the named type), not a violation.

---

## 👤 User

please do "1. Field renames in ParameterS", and /good-doc for the arcana please

---

## 🤖 Claude

I'll do the `ParameterS` field renames (`full_kind_rune`→`full_type_rune`, `inner_kind_rune`→`value_type_rune`, `outer_shape_rules`→`full_type_rules`, `named_type_rules`→`value_type_rules`) and align the consumers, then write the arcana via good-doc. First, let me scope every occurrence.

---

## 🤖 Claude

Occurrences are confined to `ast.rs`, `function_scout.rs`, `post_parser_tests.rs` (plus the `outer_shape_builder` param in `templex_scout.rs`), and the four names are unique tokens — safe for scoped `replace_all`. Starting with the `ParameterS` struct block in `ast.rs` (new field docs, and the rename markers come out since I'm completing them).

---

## 🤖 Claude

Now the `replace_all` sweeps for the remaining occurrences in `ast.rs` (`::new` params, asserts, construction):

---

## 🤖 Claude

Now `function_scout.rs` (locals, destructure, `::new` args, comment):

---

## 🤖 Claude

Now `post_parser_tests.rs` (field accesses, comments, assert-message strings):

---

## 🤖 Claude

Now handle the stragglers not caught by the exact-token sweep (`outer_shape_builder` in `templex_scout.rs`, singular/hyphenated mentions, and terminology in comments). Let me find them all.

---

## 🤖 Claude

Now `templex_scout.rs` — rename the `outer_shape_builder` param and the local `named_type_rune` to match the new terminology:

---

## 🤖 Claude

Now the terminology in `templex_scout.rs`'s doc + internal comments ("named type" → "value type"), and the two `function_scout.rs` comments. Let me read the current doc block to rewrite it exactly.

---

## 🤖 Claude

Now the two `function_scout.rs` comments and any singular straggler in the tests. Let me read the exact lines.

---

## 🤖 Claude

Fixing the remaining terminology comments. First `function_scout.rs`:

---

## 🤖 Claude

Now the `post_parser_tests.rs` doc comment and the singular straggler. Let me read the doc header.

---

## 🤖 Claude

Renames applied across all four sites. Let me verify no old names remain, then build + test.

---

## 🤖 Claude

Only remaining "named-type" is the intentional definition in `ast.rs:374` ("the value type: the named-type root, past the outer wraps") — that's a helpful bridge, keeping it. Now build + test.

---

## 🤖 Claude

Green — **501/0/1, zero warnings**. Renames complete and clean. Now the arcana via good-doc.

---

## 🤖 Claude

I'll follow the good-doc protocol. First, read the documentation strategy.

---

## 🤖 Claude

Read the strategy. This is squarely **arcana** (a local structure — the `ParameterS` split — with non-obvious effects elsewhere: the solver, the anon-interface macro, return position). No new shield: its two invariants (`full_type_rules` holds only ref wraps; `full == value` when there are no wraps) are already `debug_assert`-guarded in `ParameterS::new` — compiler/runtime enforced, not a Guardian-checkable pattern. Let me confirm the arcana directory and existing naming convention.

---

## 🤖 Claude

There's one existing postparsing arcana. Let me read it as the style/format reference.

---

## 🤖 Claude

This is one arcana. Per the current good-doc format (single section, why-focused, no file/line refs), here's my proposal — **I'll write it to disk and annotate the code only after you approve.**

**Title:** Parameter Full-Type / Value-Type Split
**ID:** `PFVSZ`
**Location:** `FrontendRust/src/postparsing/docs/arcana/ParameterFullTypeValueTypeSplit-PFVSZ.md`
**Shield:** none — its two invariants are already `debug_assert`-guarded in `ParameterS::new` (compiler/runtime enforced, not a Guardian pattern).

**Drafted body:**

---
# Parameter Full-Type / Value-Type Split (PFVSZ)

A function parameter's type is stored on `ParameterS` as two halves, not one. The **full type** is what the caller passes: the value plus its outer reference wraps, e.g. `&&Ship`. The **value type** is the citizen the wraps enclose, e.g. `Ship`. So `func foo(x &Ship)` has full type `&Ship` and value type `Ship`; a bare `func foo(x Ship)` has a full type and value type that coincide.

Each half is recorded twice: a rune (`full_type_rune` / `value_type_rune`) and the rules that produce it (`full_type_rules` / `value_type_rules`). The rule buckets are named after the rune each produces, not after their contents. `full_type_rules` holds the chain of `BorrowRef` / `HeapOwnRef` / `ShareRef` / `WeakRef` wraps whose outermost result is `full_type_rune`; `value_type_rules` holds the `Lookup` / `Call` / etc. that build `value_type_rune`. Naming a bucket after its result rune keeps the two halves symmetric and lets a reader follow "which rules make this rune" from the name alone.

`translate_signature_templex` is the sole producer of the split: it peels the outermost run of ref wraps into the full-type bucket and drops the value type, and anything nested inside it (including inner wraps in template args), into the main bucket.

**Why split at all.** Signature position is the one place that needs the value type held apart from its wrapping. The solver treats the value type as opaque when resolving a call, matching and substituting on the outer shape without re-walking the citizen underneath. The anonymous-interface macro mirrors a parameter's outer wraps onto a synthesized forwarder parameter while swapping the value type. Return position will take the same split when it lands. Every consumer wants "the wraps" and "the thing wrapped" as separable pieces, so the split lives on `ParameterS` (a signature-only structure) instead of being re-derived at each use.

**Invariants.** `full_type_rules` may hold only the four ref-wrap rules, and when it is empty `full_type_rune` and `value_type_rune` are the same rune. Both are `debug_assert`ed in `ParameterS::new` rather than made unrepresentable, so an illegal bucket or a mismatched pair fails loudly at construction.

---

**Code sites I'll annotate with `@PFVSZ`** (each in a sentence): the `ParameterS` split fields (`ast.rs`), `translate_signature_templex` (`templex_scout.rs`), and the per-param bucket comment in the param loop (`function_scout.rs`).

Approve as-is, or want changes to the title/ID or wording before I write it?

---

## 👤 User

is the anonymous-interface macro really worth mentioning here? it's not special, others need this machinery too right?

---

## 👤 User

is the anonymous-interface macro really worth mentioning here? it's not special, others need this machinery too right?

---

## 🤖 Claude

You're right, and it's the same thing you caught me on earlier in the plan discussion — the split isn't *for* the anon-interface macro; the macro is just one consumer of a general signature/value boundary. Naming it elevates it into looking like the reason, when the real reason is "at signature position you need the wraps and the value separable," and the load-bearing consumer is actually the solver's opacity. Anything that reshapes wrapping needs the same machinery.

So I'll generalize the "Why" paragraph: lead with the solver (the always-active, load-bearing case), describe the wrap-reshaping use as a *category* rather than naming the macro, and keep the brief forward-pointer to return position to show it's a general pattern.

Revised paragraph:

> **Why split at all.** Signature position is where the outer wraps and the value they enclose have to be reasoned about separately. The load-bearing case is the solver: resolving a call matches and substitutes on the outer shape while treating the value type as opaque, so it never re-walks the citizen underneath. Anything that reshapes a parameter's wrapping — keeping the wraps while swapping the value type, say — likewise works on the wrap chain as a unit, and return position will take the same split when it lands. The split lives on `ParameterS` (a signature-only structure) because every such consumer wants the wraps and the wrapped value as separable pieces, not re-derived at each use.

The rest of the draft stands. Good to write it with this paragraph, or want to adjust further?

---

## 👤 User

"resolving a call matches and
substitutes on the outer shape while treating the value type as opaque, so it never re-walks the citizen underneath."

what are you saying here?

---

## 🤖 Claude

Honestly — I overreached there, and you caught it. I took your earlier "we can ignore the second list when we're doing solving" and inflated it into specific solver mechanism ("matches and substitutes on the outer shape," "never re-walks the citizen underneath") that I can't actually back. Two problems:

1. **It states solver behavior as present fact, but the solver is in `typing/`, which is gated out.** Nothing live reads the split today — its consumers are all in not-yet-linked passes. So I'm narrating intended behavior in the present tense as if it's running.

2. **The mechanism is invented.** What you actually told me was that solving can *skip the value-type rules* (the second list). I turned "skip the second list" into claims about how unification walks the wrap vs. the citizen — which I have no grounding for. That's exactly the jargon-soup / unverified-claim trap.

So that sentence should either lose the mechanistic detail or be replaced with what the solver actually does — and you know that, I don't. Two ways forward:

- **You tell me the real intent** and I write it accurately. My guess is "the value type is an already-resolved unit, so a consumer working on the wrapping only needs `full_type_rules`, not `value_type_rules`" — but that's a guess.
- **We keep it modest** and I don't pin the mechanism, e.g.:

  > The split lets a consumer take the value type as a single opaque piece and reason about the wraps on their own — so, for example, work that only concerns a parameter's wrapping doesn't have to re-derive the value type's rules.

Related: since every consumer is currently gated, the arcana probably shouldn't read as describing live behavior at all. I'd add a line making clear the split is produced now but consumed by passes that aren't linked yet. Want me to go with the modest version + that caveat, or do you want to give me the actual solver intent to write down?

---

## 👤 User

please look at /Volumes/V/Vale1's prose-reviewer.md and bring its changes into ours please

---

## 🤖 Claude

Let me find and read Vale1's prose-reviewer.md, then diff it against ours.

---

## 🤖 Claude

Comparing: Vale1's rules 1–4 are byte-identical to ours. Vale1 has **two rules we don't**:
- "Lead with the takeaway, then an example, then the details"
- "Minimize reader effort, not word count"

(And ours has two Vale1 lacks — "Comment a non-obvious match arm…" and "No em-dashes…" — those are our session's additions; I'll leave them.)

I'll insert Vale1's two rules where Vale1 has them — right after "Describe the general invariant" and before our match-arm rule — so the prose-quality progression stays contiguous and our mechanical rules stay at the end.

---

## 🤖 Claude

Brought over. Let me confirm the result — diffing Vale1 against ours should now show only our two session-added rules as the remaining difference.

---

## 🤖 Claude

Done. Vale1's two rules are now in our `prose-reviewer.md`, placed exactly where Vale1 has them (after "Describe the general invariant," before our session's mechanical rules):

- **Lead with the takeaway, then an example, then the details** (TLDR → example → details, with "show, don't describe")
- **Minimize reader effort, not word count**

The heading diff confirms the only remaining difference is our two session-added rules (`Comment a non-obvious match arm…`, `No em-dashes in comments`), which stay at the end — so our doc is now a strict superset of Vale1's.

One thing worth flagging, since we just added the em-dash rule: the two rules I imported use em-dashes in their *doc prose* (e.g. "in that order — including this rule", the `BEFORE —`/`AFTER —` labels). I ported them verbatim to stay faithful to Vale1's canonical text, and it's consistent with the rest of this doc — the whole file (preamble and every existing rule) uses em-dashes in its rationale prose. Our "No em-dashes" rule is scoped to *code comments*, not the reviewer doc's own prose, so there's no contradiction. If you'd rather de-em-dash the doc's own prose too, that's a separate sweep — say the word.

Want me to circle back to the arcana now (you were about to give me the real solver intent for the `PFVSZ` "why" paragraph)?

---

## 👤 User

please rewrite your arcana draft to be more clear, per prose-reviewer

---

## 🤖 Claude

Here's the rewritten draft, applying prose-reviewer: leads with the takeaway then a concrete example then details, one idea per sentence, active voice, no em-dashes, and a concrete rule example ("show, don't describe"). I also fixed the "why" paragraph you flagged: dropped the invented solver mechanism, kept only what's grounded (value type taken as opaque, per your "ignore the second list"), didn't elevate the anon-interface macro, and added the caveat that every consumer is gated so nothing live reads it yet.

---
# Parameter Full-Type / Value-Type Split (PFVSZ)

A function parameter's type is stored on `ParameterS` in two halves: the outer reference wraps, and the value those wraps enclose.

Take `func foo(x &Ship)`. The **value type** is `Ship`, the citizen being referred to. The **full type** is `&Ship`, that citizen inside one borrow wrap. `&&Ship` has the same value type `Ship` and a full type of two wraps. A bare `func foo(x Ship)` has no wraps at all, so its full type and value type are the same.

Each half is stored as a rune plus the rules that build that rune, which is four fields: `full_type_rune` with `full_type_rules`, and `value_type_rune` with `value_type_rules`. For `&Ship`, `value_type_rules` is a `Lookup(Ship)` producing `value_type_rune`, and `full_type_rules` is a `BorrowRef` around that rune producing `full_type_rune`. The wrap rules (`BorrowRef`, `HeapOwnRef`, `ShareRef`, `WeakRef`) always live in `full_type_rules`; the value's own rules (`Lookup`, `Call`, and so on) always live in `value_type_rules`. Each bucket is named after the rune it produces, not after what it contains, so you can read "which rules make this rune" straight from the field name.

`translate_signature_templex` produces the split. It peels the outermost run of wraps into the full-type bucket, and puts the value type, plus anything nested inside it (including wraps buried in template args), into the main bucket.

**Why store two halves.** A parameter's wraps and the value they enclose get used separately, so `ParameterS` keeps them apart instead of making every reader re-derive the split. The solver, for one, will take the value type as a single opaque piece and reason about the wraps on their own. Other machinery copies the wrap chain onto a different parameter while swapping the value underneath. All of this lives in passes that aren't linked yet, so today the split is produced but nothing live consumes it.

**Invariants.** `full_type_rules` may hold only the four wrap rules. When it is empty (no wraps), `full_type_rune` and `value_type_rune` are the same rune. `ParameterS::new` checks both with `debug_assert!` rather than making them unrepresentable, so an illegal bucket or a mismatched pair fails loudly at construction.

---

Approve this and I'll write it to `ParameterFullTypeValueTypeSplit-PFVSZ.md` and add the `@PFVSZ` references (the `ParameterS` fields, `translate_signature_templex`, and the per-param bucket comment)? Or refine further first.

---

## 👤 User

use this instead, in there:

The ParameterS holds:
- `full_type_rune` holds the rune for the full type.
- `value_type_rune` holds the rune for the value type.
- `value_type_rules` holds the Lookup / Call / etc. that build value_type_rune.
- `full_type_rules` holds the chain of BorrowRef / HeapOwnRef / ShareRef / WeakRef wraps whose outermost result is full_type_rune.

---

## 🤖 Claude

Integrated. Here's the full draft with your bullet list in place of my dense fields paragraph:

---
# Parameter Full-Type / Value-Type Split (PFVSZ)

A function parameter's type is stored on `ParameterS` in two halves: the outer reference wraps, and the value those wraps enclose.

Take `func foo(x &Ship)`. The **value type** is `Ship`, the citizen being referred to. The **full type** is `&Ship`, that citizen inside one borrow wrap. `&&Ship` has the same value type `Ship` and a full type of two wraps. A bare `func foo(x Ship)` has no wraps at all, so its full type and value type are the same.

The `ParameterS` holds:
- `full_type_rune` holds the rune for the full type.
- `value_type_rune` holds the rune for the value type.
- `value_type_rules` holds the `Lookup` / `Call` / etc. that build `value_type_rune`.
- `full_type_rules` holds the chain of `BorrowRef` / `HeapOwnRef` / `ShareRef` / `WeakRef` wraps whose outermost result is `full_type_rune`.

Each bucket is named after the rune it produces, not after what it contains, so you can read "which rules make this rune" straight from the field name.

`translate_signature_templex` produces the split. It peels the outermost run of wraps into the full-type bucket, and puts the value type, plus anything nested inside it (including wraps buried in template args), into the main bucket.

**Why store two halves.** A parameter's wraps and the value they enclose get used separately, so `ParameterS` keeps them apart instead of making every reader re-derive the split. The solver, for one, will take the value type as a single opaque piece and reason about the wraps on their own. Other machinery copies the wrap chain onto a different parameter while swapping the value underneath. All of this lives in passes that aren't linked yet, so today the split is produced but nothing live consumes it.

**Invariants.** `full_type_rules` may hold only the four wrap rules. When it is empty (no wraps), `full_type_rune` and `value_type_rune` are the same rune. `ParameterS::new` checks both with `debug_assert!` rather than making them unrepresentable, so an illegal bucket or a mismatched pair fails loudly at construction.

---

I kept the concrete `func foo(x &Ship)` example above the bullets (it anchors "value type" vs "full type" before the field list) and the naming-convention note below them. Approve and I'll write the file + add the `@PFVSZ` references?

---

## 👤 User

"Each bucket is named after the rune it produces, not after what it contains" youre right, and im not sure thats a good thing.

should we go with type_outer_ref_rules and type_inner_rules?

---

## 🤖 Claude

You're right, and it's worth pinning down *which* name is actually the problem, because they're not equally guilty:

- **`full_type_rules` is the misleading one.** Building the full type `&Ship` needs *both* buckets: the `Lookup(Ship)` lives in one, the `BorrowRef` in the other. So `full_type_rules` does not hold "the full type's rules" — it holds only the outer wrap that sits on top. Named after its output rune, not its contents. That's the smell.
- **`value_type_rules` is actually already honest.** It holds the rules that build the value type, and the value type *is* fully built there. So it's named after its contents too, it just happens to coincide with its output rune.

So by your own principle ("name by contents"), only `full_type_rules` strictly has to change. `type_outer_ref_rules` fixes it cleanly — that's exactly what's in the bucket.

That leaves a small choice on the value side:

- **Keep `value_type_rules`** — it's already contents-honest and more meaningful than "inner" (it tells you it's the value/citizen). Cost: the two rule fields read as a mismatched pair (`type_outer_ref_rules` + `value_type_rules`).
- **Rename to `type_inner_rules`** (your proposal) — the two buckets read as a clean `outer`/`inner` pair. Cost: you rename an already-accurate name, and the value rune/rules stop sharing a stem (`value_type_rune` alongside `type_inner_rules`).

The runes stay `full_type_rune` / `value_type_rune` either way — a rune denotes a type, so naming it by the type is right; the "named after output" problem only applies to the rule buckets.

My lean: `type_outer_ref_rules` + keep `value_type_rules`. It fixes the one genuinely misleading name and leaves the one that's already fine, minimizing churn, and "outer ref rules" vs "value type rules" still reads clearly (the wraps vs the value). But if the consistent `outer`/`inner` pair matters more to you, `type_outer_ref_rules` + `type_inner_rules` is also clean.

Whichever you pick, I'll do the rename pass (same three files + tests) and update the arcana — dropping that "named after the rune it produces" note, since the whole point is we're fixing it. Which way?

---

## 👤 User

type_outer_ref_rules + keep value_type_rules sg, please make ith appen

---

## 🤖 Claude

On it. First the field rename `full_type_rules` → `type_outer_ref_rules` across the three files (`value_type_rules` stays):

---

## 🤖 Claude

Now the builder param in `templex_scout.rs` to match, then the prose stragglers:

---

## 🤖 Claude

Let me verify what `full_type` references remain (should only be the kept `full_type_rune`), then build + test.

---

## 🤖 Claude

Green (**501/0/1**). Almost everything remaining is `full_type_rune` (kept, correct). Two prose stragglers still say "full-type" for the renamed rules — fixing:

---

## 🤖 Claude

Field rename complete and green. Now writing the arcana (with the corrected `type_outer_ref_rules` naming and the old "named after the rune" note replaced by a single sentence explaining the *current* naming), then annotating the code sites.

---

## 🤖 Claude

Now the `@PFVSZ` references. Let me read the current field region in `ast.rs` to place one precisely.

---

## 🤖 Claude

I'll add the `@PFVSZ` reference above the four split fields in `ast.rs`:

---

## 🤖 Claude

Now the `translate_signature_templex` doc in `templex_scout.rs` and the per-param bucket comment in `function_scout.rs`:

---

## 🤖 Claude

All three sites annotated. Final verification — build/test green and references consistent:

---

## 🤖 Claude

Done. Both the rename and the arcana landed; suite green throughout (**501/0/1, zero warnings**).

## Rename
- **`full_type_rules` → `type_outer_ref_rules`** across `ast.rs`, `function_scout.rs`, `post_parser_tests.rs` (field, `::new` param, local `param_…_vec`, `debug_assert!` messages, test accesses).
- **`full_type_builder` → `type_outer_ref_builder`** in `translate_signature_templex`, plus its prose comments.
- **Kept** `value_type_rules`, `full_type_rune`, `value_type_rune` (all already contents-honest, per your call).
- Swept the prose stragglers (`full-type wraps` / `full-type builder` / `full-type rules` in comments) over to "outer ref".

## Arcana
Written to `FrontendRust/src/postparsing/docs/arcana/ParameterFullTypeValueTypeSplit-PFVSZ.md`, with `@PFVSZ` references at three sites: the `ParameterS` split fields (`ast.rs`), `translate_signature_templex` (`templex_scout.rs`), and the per-param bucket comment (`function_scout.rs`). It's a proper arcana (discoverable via those `@ID` backlinks), no manifest wiring needed.

**One thing to eyeball:** you said drop the "named after the rune it produces" note. I dropped that flaw-note, but replaced it with a single *positive* sentence explaining the current naming — why `type_outer_ref_rules` isn't `full_type_rules`:

> `type_outer_ref_rules` is named for its contents, not for `full_type_rune`, because the wraps don't build the full type alone; the full type also needs the value rune that `value_type_rules` produces.

That captures the exact insight behind the rename, so I thought it earned a line — but if you meant strip it entirely, say so and I'll cut it.

---

## 👤 User

remember when i had you put in the "The `ParameterS` holds:" list instead of what you had? do you understand why i had you do that?

---

## 🤖 Claude

Yes, I think so — and it's more than just "bullets read easier."

Your version states, for each field, **exactly what it holds**, one field per line: "`full_type_rules` holds the chain of BorrowRef/… wraps." Mine described how the pieces *compose* (rune-plus-rules, the `&Ship` walk of how one produces the other) and paired them by name, which glossed over what each bucket literally contains.

Two things came out of that:

1. **It's far easier to read** — one idea per line, nothing crammed. That's the "minimize reader effort, not word count" rule we just pulled in from Vale1. My paragraph made the reader parse a compound sentence to extract each field's role.

2. **The real payoff: it exposed the misnaming.** Once each field had to plainly declare its contents, the line "`full_type_rules` holds the chain of BorrowRef/… wraps" made it obvious the bucket holds *wraps*, not "the full type" — the mismatch between the name and the contents was sitting right there. My prose hid it by describing relationships instead of contents. That's what let you immediately catch it and rename it to `type_outer_ref_rules`.

So the lesson I take: stating a struct's fields as flat "X holds Y" lines isn't just formatting — it forces honesty about what each field actually contains, and that surfaces naming and design problems that flowing prose papers over.

Is that the reason you were after, or were you pointing at something more specific?

---

## 👤 User

mainly #1. lists are easier to read. it gives the reader's mind hints on how to mentally organize the content, before they get into it. but basically yes, it's clearer. can you add a section to prose-reviewer.md that we should use short lists instead of dense paragraphs wherever it makes sense? like, whenever presenting multiple equal sub-parts of a concept.

---

## 🤖 Claude

That's the cleaner framing — a list hands the reader the shape (how many parts, that they're parallel) *before* they start reading, so their mind has a scaffold. I'll add it right after "Minimize reader effort, not word count," since it's the same family.

---

## 🤖 Claude

Added — **"Use a list for equal sub-parts, not a dense paragraph"**, placed right after "Minimize reader effort, not word count" (same reader-effort family, keeping the doc's build-up order).

The rule text:
> List a concept's equal sub-parts instead of cramming them into a paragraph. The list shows the reader how many pieces there are and that they're parallel before they dive in.

And the BEFORE/AFTER uses the exact `ParameterS` case that prompted it — a dense four-fact sentence vs. the four-bullet field list — so the doc example is the real thing that motivated the rule.

---

## 👤 User

you had:

**Why store two halves.** A parameter's wraps and the value they enclose get used separately, so `ParameterS` keeps them apart instead of making every reader re-derive the split. The solver, for one, will take the value type as a single opaque piece and reason about the wraps on their own. Other machinery copies the wrap chain onto a different parameter while swapping the value underneath.

but i think this is better:

**Why store two halves.** The typing pass ignores the outermost references when looking for functions to call. For example, if `my_ship_ref &Ship` and we call `my_ship_ref.launch()`, the typing pass ignores the `&` and looks in the namespace of `Ship` for the `launch` method. Having a separate `value_type_rules` (without the outer references) makes it easier for the typing pass to do this.

do you understand why?

---

## 🤖 Claude

Yes — and it's the sharpest version yet of the exact thing you've been correcting me on all session.

**My version described the benefit in the abstract** — "take the value type as a single opaque piece and reason about the wraps on their own." That's hand-waving. It never says what actually *happens*, so a reader can't picture it. It's the same "what are you saying here?" trap from the solver sentence, just dressed a little differently.

**Your version shows the real mechanism, concretely.** "The typing pass ignores the `&` and looks in `Ship`'s namespace for `launch`." Now the reason is self-evident: if lookup is going to strip the outer refs and search the value type's namespace, then storing `value_type_rules` already stripped is exactly what makes that easy. The example does the explaining — that's the "lead with a takeaway, then a real example" and "show, don't describe" rules, applied to the one paragraph where I'd ignored them.

It's also **grounded** where mine was invented. "The typing pass looks in `Ship`'s namespace for `launch`" is a real, checkable claim about how dispatch works. "reason about the wraps on their own" was me papering over a gap.

And it **drops the noise**: my "other machinery copies the wrap chain" (the anon-interface thing you told me not to elevate, still lurking) and the gating hedge, both gone.

Swapping it in now:

---

## 🤖 Claude

Swapped in. I dropped the "nothing consumes it yet" gating caveat to match your version — it was clutter on the *why*, and the gating status is already recorded at the `lib.rs` VCOORD. Say the word if you want it back as a closing line.

The arcana now reads clean end to end: concept → concrete example → the four fields as a list → the naming note → your grounded "why" → invariants. Nothing else in it references the solver or the gating, so there's no leftover conflict.

That closes out the `PFVSZ` arcana. Everything from this stretch (the `type_outer_ref_rules` rename, the arcana + its three `@PFVSZ` annotations, and the two prose-reviewer additions) is on the working tree, green at 501/0/1, still uncommitted pending your `fire commit`.

---

## 👤 User

lets add another section to prose-reviewer.md about this

---

## 🤖 Claude

The lesson: when explaining *why* something exists, I gave an abstract benefit ("treat it as opaque, reason about the wraps") that sounds like an explanation but lets the reader picture nothing. Your version named the concrete operation with a real example. That's its own rule, distinct from "show, don't describe" (which is about example *values*) — this one's about justifying a design. Adding it after the list rule, before the mechanical ones:

---

## 🤖 Claude

Added — **"Explain a design by the concrete thing it enables"**:

> When you say why something exists, describe the concrete operation it enables, with a real example. An abstract benefit reads as explanation but lets the reader picture nothing.

The BEFORE/AFTER is the exact pair from the arcana — my "treat it as a single opaque piece and reason about the wraps" versus your "strips the `&` and looks in `Ship`'s namespace for `launch`." So the doc example is the real case that produced the rule.

That's two prose-reviewer rules born from this arcana (the list rule and this one), both grounded in what actually happened rather than invented examples. `prose-reviewer.md` now runs: invariant → active voice → front-load → general-invariant → takeaway/example/details → minimize-effort → use-a-list → explain-by-what-it-enables → match-arm-comments → no-em-dashes.

---

## 👤 User

ty. whats next

---

## 🤖 Claude

Here's what's still open, grouped. Everything's uncommitted and green (**501/0/1**, `typing`/`tests`/`solver` still gated in `lib.rs` per the arc).

**Ready to apply on your nod — leftovers from the reviewer pass:**
- **`function_scout.rs` param match [V14/V5].** Match on `&pattern.templex` alone instead of the `(templex, kind_rune)` tuple, so the impossible `(Some,None)`/`(None,Some)` crosses stop being silently absorbed by the `_` arm. Small, clean, and it's exactly the "outlaw an impossible state" rule we just codified.
- **V15 test-assertion cleanups** in `post_parser_tests.rs.** The two clean collapses (`len()==N` + element `matches!` → one slice pattern) are ready now; the `.iter().any(...)` ones need me to confirm the exact rule slice a bare `int`/`bool` param emits first.
- **Test rename**: `test_function_rules_no_longer_contains_param_rules` → drop the "no_longer" timeline word.

**Parked design questions — need your direction (the `// V:` markers still in the code):**
- Get rid of `rune_to_explicit_type` (`function_scout:377`).
- Why do we compute pattern rules at all, and why is it OK to throw them away for params? (`function_scout:432` — the `discarded_pattern_rules` throwaway.)
- Don't generate a pattern when there's no destructure, for a simpler AST (`ast.rs:367`).

**Housekeeping:**
- Strip the now-addressed `// V:` / `// VADD:` markers (you'd parked that decision; several are codified-or-fixed now).

**Commit:** whenever you want to checkpoint this stretch (StaticArray retirement follow-ups, the two-function split + per-kind extraction, the `type_outer_ref_rules` rename, the arcana, and the reviewer-doc rules), it's ready for `fire commit` / `fire commit temporary`.

My suggestion: knock out the three quick cleanups first (they close the review cleanly), then dig into the design questions — the `rune_to_explicit_type` and pattern-rules-throwaway ones are related and probably the most interesting. Where do you want to start?

---

## 👤 User

what's the next // V: ?

---

## 🤖 Claude

Let me get the current, accurate list of remaining markers.

---

## 🤖 Claude

The next open one is **`function_scout.rs:377` — "lets get rid of `rune_to_explicit_type`."** That's a directive (remove the mechanism), and it's tightly coupled with **`432`** ("why compute pattern rules at all, and why is it OK to throw them away for params?") — both are about what the param path builds and discards, so they'll likely resolve together.

Full remaining set, for accuracy (a couple I hadn't cataloged before turned up):

**Open design questions (`// V:`, from this arc):**
- `function_scout.rs:377` — get rid of `rune_to_explicit_type` ← **next**
- `function_scout.rs:432` — why compute pattern rules, why OK to discard for params
- `ast.rs:367` — don't generate a pattern when there's no destructure (simpler AST)
- `expression_scout.rs:1606` — open question: why don't we coerce to an overload set here?

**Codified `// VADD:` (work done, just marker-removal left):**
- `ast.rs:402` (outlaw-impossible-state → valec-reviewer), `post_parser_tests.rs:1417/1418` (asserts → valec-reviewer), `post_parser_tests.rs:1445` ("need arcana outer shape" → the `PFVSZ` arcana now covers exactly this).

**Pre-existing / separate (not this arc):**
- `ast.rs:647` ("this `LocationInDenizen` feels weird"), `parsing/tests/.../after_regions_function_tests.rs:10` ("unignore this" — an ignored test).

Want me to start scoping `rune_to_explicit_type` — read-only, map what it is and everywhere it threads through — so we can see what removing it actually entails? Since `377` and `432` are coupled, I'd look at both together.

---

## 👤 User

defer getting rid of rune_to_explicit_type to the end please. ok next

---

## 🤖 Claude

Deferring `rune_to_explicit_type` to the end. Next is **`function_scout.rs:432`** — "why do we calculate pattern rules ever, and why is it OK to throw them away for params?" That's a genuine "why" question about the `discarded_pattern_rules` throwaway. Let me investigate it (read-only) — starting with what `translate_pattern` actually emits into that rule builder and who normally consumes it.

---

## 🤖 Claude

`translate_pattern` builds the `AtomSP` and, when the pattern has a type annotation, emits that type's rules into `rule_builder` (and records the `kind_rune`). Let me confirm the two callers differ as I suspect — the `let` path (expression_scout) keeps those rules, the param path discards them.

---

## 🤖 Claude

Confirmed. The two callers differ exactly as suspected. Here's the answer to `432`.

## Why do we calculate pattern rules ever?

Because `translate_pattern` is shared with the **`let`-binding path**, which genuinely consumes them. In `expression_scout` (line 1116), the let caller passes its real `rule_builder`. So for `let x int = ...`, `translate_pattern` emits the `int` `Lookup` into the LetSE's rules and points the pattern's `kind_rune` at it. Those rules are the type annotation, and the LetSE needs them. That's their consumer.

## Why is it OK to throw them away for params?

Because the param path recomputes everything `translate_pattern` produces, in a shape it actually wants:

- **The type rules** get re-derived by `translate_signature_templex`, split into `type_outer_ref_rules` / `value_type_rules`. `translate_pattern` would emit them *un-split* (all in one bucket), which the param path doesn't want.
- **The `kind_rune`** gets overwritten right after, with the split's `full` rune (`pattern_s.kind_rune = Some(full.clone())`).
- **Any destructure** moves to a body-head `LetSE`, which recomputes its own rules.

So for a param, `translate_pattern` is called only to build the `AtomSP` skeleton (the capture name and destructure structure); its rule output and `kind_rune` are both redundant. That's why the rules go to `discarded_pattern_rules`.

## The smell the marker is pointing at

For a **typed** param, `discarded_pattern_rules` is not empty — `translate_pattern` walks `&Ship`, emits `[Lookup(Ship), BorrowRef(...)]`, and we immediately throw them away and recompute the split version. That's real, redundant work, and calling a rule-emitting function purely for its skeleton (then discarding the rules) is confusing to read.

The clean fix would be to separate "build the `AtomSP` skeleton" from "emit the type rules," so the param path calls only the skeleton part and never computes rules it discards. But that's a change — per your "understand first" style, I've stopped at the why.

Want me to scope that split (skeleton vs rule-emission), or is answering the question enough for now and we move to the next marker?

---

## 👤 User

im confused about what kind of rules we generate when translating a pattern

---

## 👤 User

im confused about what kind of rules we generate when translating a pattern, and why

---

## 🤖 Claude

Good question — this is the crux, and the name "pattern rules" is what's confusing. Let me clear it up.

## The only rules `translate_pattern` generates are for the pattern's *type annotation*

Look at where `rule_builder.push` actually happens inside `translate_pattern`: nowhere directly. The one place rules get emitted is `translate_maybe_type_into_rune` (line 55), which only runs when the pattern **has a templex** (a type annotation). Everything else the function does — the capture name, the destructure nesting — becomes `AtomSP` *fields*, not rules.

So a pattern's *shape* (name, destructure) is structural, and a pattern's *type* is rules. Concretely:

- `let x = 5` — no type annotation → **zero rules**. `kind_rune` is `None`.
- `let x int = 5` → **one rule**, `Lookup(int)`, and `kind_rune` points at it.
- `let [a, b] = tup` — destructure, no types → **zero rules** (just nested `AtomSP`s for `a` and `b`).
- `let [a A, b B] = tup` → **two rules**, `Lookup(A)` and `Lookup(B)`, one per sub-pattern's annotation.
- `func foo(x &Ship)` → `Lookup(Ship)` then `BorrowRef(...)` — the same rules any `&Ship` type expression produces anywhere.

## Why a type annotation becomes rules at all

A type annotation isn't a fixed thing — it's a type *expression* that has to be resolved: look up the name (`int`, `Ship`), apply reference wraps (`&`), instantiate generics (`MyList<&Ship>`). That resolution is a little constraint program, and the compiler expresses it the same way everywhere: a **rune** is the unknown standing for "the resolved type," and **rules** (`Lookup`, `Call`, `BorrowRef`, …) are how you compute it. The pattern stores the top rune as `kind_rune` so downstream code can say "the type this binding was declared as."

This is the exact same machinery as `translate_templex` for a struct member or a return type — `translate_maybe_type_into_rune` delegates straight to it. There's nothing pattern-specific about the *rules*; what's pattern-specific is the `AtomSP` structure around them.

So "pattern rules" really means "the type-annotation rules emitted while walking a pattern." A pattern with no type annotations (`let [a, b] = tup`) generates no rules at all — which is also why, for an untyped lambda param, `discarded_pattern_rules` comes back empty.

Does that clear up the confusion, or do you want me to trace a specific pattern (say `foo(x MyList<&Ship>)`) rule-by-rule?

---

## 👤 User

when we're parsing a param pattern, why dont we save those pattern rules for the later let statement?

---

## 🤖 Claude

Let me look at how the body-head `LetSE` is actually synthesized, because the answer depends on whether that `LetSE` re-states the type or just the name/destructure.

---

## 🤖 Claude

Good instinct, but it turns out the `LetSE` already gets the type for free — by sharing, not by copying rules. Look at the synthesis (lines 776–782):

```rust
LetSE {
  rules: Vec::new(),               // empty
  pattern: param.pattern.clone(),  // the param's own AtomSP
  expr: load_expr,                 // load of the param slot
}
```

The body-head `LetSE` is built with **empty rules** and **reuses the param's `AtomSP`**. That `AtomSP`'s `kind_rune` already points at the param's resolved type (`full_type_rune`, defined by the param's `type_outer_ref_rules` / `value_type_rules`). So the type is resolved **once, on the param**, and the `LetSE` just references that same rune. It doesn't recompute the type, so it has nothing to "save rules" for.

Two reasons saving `translate_pattern`'s discarded rules onto the `LetSE` would be wrong, not helpful:

- **They're the wrong shape.** `translate_pattern` emits the type *un-split* (all in one builder). The param needs it *split* (so the typing pass can ignore the outer refs and look up in the value type's namespace, per `@PFVSZ`). Only `translate_signature_templex` produces the split. The un-split copy isn't what anyone downstream wants.
- **It would create a second source of truth.** If the `LetSE` carried its own resolution of `&Ship`, the binding `x` would have a type defined by the `LetSE`'s rules *and* the param would have its own — two independent resolutions of one type that could drift. Today there's exactly one (on the param), shared through the cloned `kind_rune`.

So the redundancy the marker flags is narrower than "we throw away rules the `let` needs." It's just: for a *typed* param, `translate_pattern` computes un-split type rules that are immediately discarded because the param recomputes them split and the `LetSE` reuses the param's rune. The clean fix, if we want one, is to stop emitting those rules for params (call only the skeleton-building part of `translate_pattern`) — not to reroute them to the `LetSE`.

Does that resolve the confusion?

---

