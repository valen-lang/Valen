<!-- session: 4dcef9fa-7b32-412f-893c-a57290e52051; exported-bytes: 5226966; transcript: /Users/verdagon/.claude/projects/-Volumes-V-Vale2/4dcef9fa-7b32-412f-893c-a57290e52051.jsonl -->
# Claude Conversation Log

Session ID: 4dcef9fa-7b32-412f-893c-a57290e52051
Date: 2026-07-11 17:51:48

---

## 👤 User

please read /tmp/claude-conversation-2026-07-10-69f83eee.md in full

---

## 🤖 Claude

I'll read the conversation file in full.

---

## 🤖 Claude

I'll read the whole file.

---

## 🤖 Claude

I'll continue reading the rest.

---

## 🤖 Claude

I've read the entire conversation log in full (all 1,604 lines). Here's my consolidated understanding of where things stand.

## The arc
Work on `experimental-2`: the "onion typing" refactor where `Coord` dissolves and `Kind` gains nestable ref variants (`BorrowRef`/`HeapOwnRef`/`ShareRef`/`WeakRef`). The suite is green at **501/0/1, zero warnings**, but with `typing`/`tests`/`solver` still gated out in `lib.rs` (VCOORD marker) so parsing+postparse compile alone. Everything is **uncommitted**, pending `fire commit`.

## What this session (2026-07-10) did
1. **Un-forked the ref-layer walk** through several iterations you drove:
   - Split into `translate_templex` (plain walker) + `translate_signature_templex` (signature-position walker that peels outer ref wraps into a separate builder). This dissolved the early-return hoist, the `Option<&mut Vec>` param + ~15 `None` args, and the `.first()` reconstruction.
   - Then inlined `emit_ref_layer` + deleted `RefKind` at your request, then landed the final shape: **four per-kind functions** (`translate_borrow_ref_templex`, `_heap_own_`, `_share_`, `_weak_`) shared by both walkers — no `RefKind`, no fork duplication.
2. **Reviewer-doc rules** added: 2 to `valec-reviewer.md` (avoid early-returns/keep equivalent branches together; arrange diffs for easy review; outlaw-impossible-state-or-assert; assert-only-what-a-match-can't-express), and to `prose-reviewer.md` (match-arm case comments, no em-dashes, plus imported Vale1's rules, plus "use a list for equal sub-parts" and "explain a design by the concrete thing it enables").
3. **`ParameterS` field renames** → `full_type_rune`/`value_type_rune`/`value_type_rules`/**`type_outer_ref_rules`** (renamed from `full_type_rules` because that name was misleading — the bucket holds only wraps).
4. **Wrote the PFVSZ arcana** (`ParameterFullTypeValueTypeSplit-PFVSZ.md`) with `@PFVSZ` annotations at three sites, including your grounded "why" (typing pass ignores outer `&` and looks in `Ship`'s namespace for `.launch()`).
5. Applied em-dash/comment fixes across the uncommitted diff.

## Where it stopped
Walking the remaining `// V:` design-question markers, deferring `rune_to_explicit_type` (`function_scout:377`) to the end. The last exchange answered **`function_scout:432`** — why param pattern rules are discarded rather than saved for the body-head `LetSE`: the `LetSE` reuses the param's `AtomSP`/`kind_rune` (one source of truth, shared), and `translate_pattern` emits *un-split* rules the param path doesn't want. The clean fix would be to stop emitting rules for params (skeleton-only), not reroute them.

## Still open
- **Ready to apply:** `function_scout` param match on `&pattern.templex` alone [V14/V5]; V15 test-assertion cleanups; test rename dropping "no_longer".
- **Design questions:** `ast.rs:367` (don't generate a pattern with no destructure), `expression_scout.rs:1606` (coerce to overload set?), and the deferred `rune_to_explicit_type`.
- **Housekeeping:** strip codified `// VADD:` markers; `fire commit` when you want to checkpoint.

I'm caught up and holding. Ready for your direction.

---

## 👤 User

where were we?

---

## 🤖 Claude

Based on the log, here's where we left off. Let me confirm against the live working tree.

---

## 🤖 Claude

Confirmed — working tree matches the log exactly. Green suite (501/0/1), all uncommitted on `experimental-2`, `typing`/`tests`/`solver` still gated in `lib.rs`.

**Where we left off:** walking the remaining `// V:` design-question markers. We'd just finished answering `function_scout.rs:432` (why param pattern rules are discarded rather than saved for the body-head `LetSE` — the `LetSE` reuses the param's `AtomSP`/`kind_rune` as one shared source of truth, and `translate_pattern` emits *un-split* rules the param path doesn't want).

**The next `// V:` markers still open** (per our agreed order — `rune_to_explicit_type` deferred to the end):

Design questions:
- `ast.rs:367` — don't generate a pattern when there's no destructure (simpler AST)
- `expression_scout.rs:1606` — why don't we coerce to an overload set here?
- `function_scout.rs:377` — get rid of `rune_to_explicit_type` *(deferred to end, per your call)*
- Plus a few pre-existing/separate ones: `ast.rs:647` (`LocationInDenizen` feels weird), `tests/tests.rs:1`, `instantiating_interner.rs:757`, `after_regions_function_tests.rs:10` (unignore this)

Codified-but-marker-still-present: `post_parser_tests.rs:1445` ("need arcana outer shape" — the PFVSZ arcana now covers this).

**Ready to apply on your nod (from the reviewer pass):** the `function_scout` param match on `&pattern.templex` alone [V14/V5], the V15 test-assertion cleanups, and the `test_function_rules_no_longer_...` rename.

Want to pick up at the next marker — `ast.rs:367` (don't generate a pattern when there's no destructure)?

---

## 👤 User

i think we need to rethink how we're doing patterns and parameters a little bit. the fact that we're throwing away anything is a big design smell to me. it means that we havent found the actual simple building blocks here

---

## 👤 User

i think we need to rethink how we're doing patterns and parameters a little bit. the fact that we're throwing away anything is a big design smell to me. it means that we havent found the actual simple building blocks here.
can you look around and answer me these questions:
- why do patterns need rules, actually? is it because any mention of types needs rules?
- why cant each AtomSP have its own rule list?
- when processing the parameter's pattern, why cant we save its rules for later, why are we throwing them away? i dont think we actually had the right answer last session.
And please investigate any other questions that arise when youre looking into those

---

## 🤖 Claude

Good — this is a "understand the real building blocks first" investigation. Let me dig into the actual code. I'll read the core structures directly and fan out read-only agents for breadth on consumers.

---

## 🤖 Claude

Now the `ParameterS` struct and the param loop in `function_scout.rs`:

---

## 🤖 Claude

Now the param loop in `function_scout.rs`:

---

## 🤖 Claude

Now let me find the body-head `LetSE` synthesis and the `let` path in expression_scout:

---

## 🤖 Claude

Now the `LetSE` struct and the `let` path in expression_scout:

---

## 🤖 Claude

Now let me understand how the typing pass (the consumer) reads these — `LetSE.rules`, `ParameterS` rules, `AtomSP.kind_rune`, and whether it walks the AtomSP tree or a flat rule bucket. Let me fan out read-only agents for breadth while I check the solver's rule model directly.

---

## 🤖 Claude

The split fields have **zero consumers** outside postparse — the typing pass hasn't been relinked to them yet. Let me look at how the typing pass historically read a parameter's type, and how function rules get assembled, to understand the intended model. Let me spawn focused read-only agents in parallel.

---

## 🤖 Claude

While those run, let me check how `FunctionS.rules` is assembled and whether param rules merge into it:

---

## 🤖 Claude

Let me check the pre-onion (master) design of `ParameterS` — that reveals what the "original simple building blocks" were:

---

## 🤖 Claude

Master `ParameterS` is dead simple — no runes, no rules, just `pattern`. Let me see how master fed param type rules to the solver:

---

## 🤖 Claude

That's the key finding. In **master**, `translate_pattern` for a param was called with `&mut rules` — the function's single flat rule bucket (line 404) — exactly like return types. No split, no per-param buckets, no throwaway. Let me confirm the let path and wait for the solver-model agents.

---

## 🤖 Claude

The let path (master and current) writes `translate_pattern`'s rules straight into `LetSE.rules` — no throwaway. Only the current **param** path discards. Let me confirm the solver's rule-input model directly while the agents finish:

---

## 🤖 Claude

Solver model confirmed: every rule carries its own `result_rune` and references inputs by rune (`BorrowRefSR { result_rune, inner_rune, .. }`). Rules are **rune-keyed constraints** — the flat list is just a container; the actual graph lives in the rune references, so ordering is irrelevant to the solver.

I have the core picture. Let me wait for the two agents to confirm the typing-pass consumption side (central to whether each-AtomSP-owns-rules is feasible) before I synthesize — I don't want to speculate about the consumer.

---

## 🤖 Claude

The master model is now fully clear: `assemble_call_site_rules(function.rules)` — the solver solves the **one flat bucket**, and param types were IN it. The `ParameterS.pattern.coord_rune` merely *names* which solved rune is the param's type; ParameterS carried no rules at all.

I have the producer side and solver model solid. I'm waiting on the two agents to nail down the consumer side (how the future typing pass wants to read the split) before I synthesize — I don't want to speculate about the consumer, which is exactly the trap to avoid here. Let me hold for their reports.

---

## 🤖 Claude

I'll wait for the two investigation agents to complete before synthesizing the final answer.

---

## 🤖 Claude

I don't need Monitor — the agents notify on completion automatically. I'll wait for both reports, then synthesize the answer to your three questions. Holding for the agent completions.

---

## 🤖 Claude

Agent 1 confirms the picture precisely. Now waiting on agent 2 (solver model / feasibility of per-AtomSP rules) before synthesizing.

Key confirmation from agent 1: the typing pass solves **only** the flat `FunctionS.rules`; the receiver rune for each param is read off `p.pattern.kind_rune` (which is now set to `full_type_rune`); the `let` pipeline is self-consistent (one `LetSE.rules` bucket + AtomSP kind_runes read as solved variables); and `get_rune_types_from_pattern` recursively walks the AtomSP destructure tree collecting each `kind_rune`. Holding for agent 2.

---

## 🤖 Claude

Both agents reported. I now have the producer side, the solver model, and the consumer side all grounded. Here's what I found, answering your three questions — and I think you're right that there's a missing simple building block.

## The one-sentence takeaway

The throwing-away is a *symptom*. The root smell is that a parameter's type is now stored **twice** in two different shapes: as flat rules-in-a-bucket (how every other type mention works) and as a pre-split pair of slices on `ParameterS`. The split slices carry no information the flat rules don't already carry, so one of the two walks is always redundant. Last session's answer ("we recompute split, the LetSE reuses the rune") described the symptom, not this.

The proof it's not inherent: **master threw nothing away.**

## What master did (the simple building block)

- `master`'s `ParameterS` is just `{ range, virtuality, pre_checked, pattern }`. No runes, no rule slices.
- In the param loop, `translate_pattern` was called with `&mut rules` — the function's **one flat rule bucket**, the same bucket that holds the return type's rules, the template rules, everything.
- Nothing was discarded. `AtomSP.coord_rune` just *named* which rune in that bucket was the param's type.

So master had exactly one building block, used uniformly: **any mention of a type, anywhere, emits its rules into the denizen's single bucket and hands back a rune that names the result.** Param, return type, let-binding, struct member — all identical.

The `let` path *still* works this way today, and it's self-consistent: `translate_pattern` writes into the real `rule_builder` that becomes `LetSE.rules`, no throwaway. Only the **param** path was changed to be different.

## Your three questions

**1. Why do patterns need rules? Is it because any mention of types needs rules?**

Yes, exactly that, and "pattern rules" is a misnomer worth dropping. A pattern contributes *structure* (a capture name, a destructure shape) — that becomes `AtomSP` fields, never rules. The **only** thing in a pattern that emits rules is a **type annotation**, and a type annotation is just a type expression that has to be resolved (look up `Ship`, apply `&`, instantiate `MyList<&T>`). That resolution is the same little rune+rules constraint program used for a struct member or a return type; `translate_maybe_type_into_rune` delegates straight to the shared `translate_templex`. Confirm: an untyped pattern (`let x = 5`, or a lambda param `(a) => a`) emits **zero** rules. So the rules aren't "pattern rules," they're "the type-annotation rules that happen to sit inside a pattern."

**2. Why can't each AtomSP have its own rule list?**

It can. Nothing in the solver stops it. The solver core (`SimpleSolverState`) is **rune-keyed and order-independent** — it treats rules as an unordered bag indexed by the runes they mention, and fires each rule when its input runes are known, regardless of list position. Rules only ever reference each other by rune identity (`BorrowRefSR { result_rune, inner_rune }`), never by pointer or position.

So the single-flat-bucket-per-denizen shape is a **postparse feeding convention, not a solver requirement.** You could hang each `AtomSP`'s type-annotation rules on that `AtomSP` and concatenate them at the `&[IRulexSR]` boundary; the engine would index and solve them identically. Per-AtomSP rules is arguably *more* honest — `let [a A, b B] = tup` would have `a` own `[Lookup(A)]` and `b` own `[Lookup(B)]`, instead of both dumping into one shared `LetSE.rules`. The only cost is a gather-walk at solve time, and you lose the single-slice convenience. There's already precedent for sub-structuring within a denizen (`StructS` header/member buckets, `GenericParameterDefaultS`'s sub-solve) — all just re-flattened before the solve.

**3. Why are we throwing the param's pattern rules away?**

Because the onion work chose to store the param's type as a **pre-computed split** (`type_outer_ref_rules` + `value_type_rules`) on `ParameterS`, and to build that split it walks the type a **second time** via `translate_signature_templex`. That second walk makes the first walk's output (from `translate_pattern`) redundant, so it goes to `discarded_pattern_rules`. And note `pattern_s.kind_rune` is *also* overwritten right after with the split's `full` rune — so even the rune from the first walk is discarded.

We absolutely *can* save them; master used them directly. The fix is not to reroute the discarded rules onto the `LetSE` (last session's framing). It's to **not have two walks**. The rules from `translate_pattern` are perfectly good — they already contain the full ref chain, linked `result_rune → inner_rune`.

## The deeper finding you should weigh in on

Three things I turned up that make the split look like premature materialization:

1. **The split is derivable, not primary.** Given `full_type_rune`, the value type is "follow the `BorrowRef`/`HeapOwn`/`Share`/`Weak` chain down `inner_rune` until you hit a non-ref rule." Both `value_type_rune` *and* the outer/value rule partition are a *view* over the flat rules, not new information. The only genuinely-primary data is: the flat rules + `full_type_rune`.

2. **The split has zero consumers.** Nothing in `src/typing/` reads any of the four split fields. The typing pass still solves only `FunctionS.rules` and reads `p.pattern.kind_rune`. But the param's rules no longer live in `FunctionS.rules` (they're stranded on `ParameterS`), so the current split is not just unused — it would leave the future solver *unable to see param types at all* until it's rewired. So today the split is pure cost with no realized benefit.

3. **There are now three inconsistent treatments of the same thing:**
   - return type → rules into the flat function bucket
   - explicit param → pre-split slices on `ParameterS` + a discarded first walk
   - closure param → rules into the flat function bucket (split slices left empty)

That inconsistency is the concrete shape of the smell you're sensing.

## The decision that's actually yours

The `@PFVSZ` rationale for the split was "so the typing pass can ignore outer refs for method lookup without re-deriving." Given findings 1–3, the real question is: **is the split worth materializing at all, or should a parameter store the same building block as everything else (flat rules + a naming rune) and let the typing pass derive the value type by walking the ref chain when it needs it?**

Three coherent directions, none of which I want to take unilaterally since this reshapes `ParameterS` and touches the arcana we just wrote:

- **A — collapse to the master building block.** `translate_pattern` writes param rules into one bucket (function-level, or onto the AtomSP per Q2); drop the split slices; derive value-type-vs-wraps in the typing pass on demand. Kills the second walk, the throwaway, and the inconsistency in one move.
- **B — keep the split but make it the only walk.** Split `translate_pattern` into "build skeleton" and "emit type rules"; the param path calls skeleton-only, and `translate_signature_templex` is the sole rule emitter. Removes the throwaway but keeps `ParameterS` special.
- **C — keep as-is** and accept the materialization, on the bet that the future typing pass will want it pre-split often enough to justify the duplication.

My read is that **A** is the one that matches your "find the actual simple building blocks" instinct: a parameter isn't special, it's just a place a type is mentioned, same as a struct member or a return type. But this depends on how the not-yet-linked typing pass will actually want to consume it, and there's real design intent behind the split I don't want to overwrite. Which direction do you want to explore?

---

## 👤 User

what about this... what if in postparsing a function signature, we just look at the outer type of the pattern (for example the Ship in `a Ship[_, _, Engine[_, fuel]]`) instead of looking at the pattern as a whole. then once we get to the body, we do a full pattern postparsing. theres no reason for the signature postparsing to fully process the entire pattern

---

## 🤖 Claude

That's a cleaner decomposition than anything in the A/B/C menu I gave you, and I think it's correct. Let me state why, then verify it against the code.

## Why the proposal is right

The signature and the body want genuinely different things out of a parameter:

- **The signature wants the type.** `func foo([a, b, Engine[_, fuel]] Ship)` has exactly the same signature as `func foo(x Ship)`. The destructure changes nothing about the function's type, overload resolution, or solve. Only the type annotation `Ship` matters.
- **The body wants the pattern.** The destructure, the captures, the nested `Engine[_, fuel]` — all of that is about binding names *inside* the body, which is precisely what a `let` already does.

Today `translate_pattern` is called in the signature and forced to do *both* jobs at once — walk the whole destructure *and* emit the type rules — and then the type rules get thrown away and re-derived. Your proposal splits it along the natural seam: **the signature translates only the type annotation (`Ship`); the body does the full pattern, once, exactly like a normal `let`.** No walk is ever discarded, because the two walks now compute different things for different consumers.

It also isn't really "two solves of `Ship`." The body-head `LetSE` destructures a *load of the already-typed param slot*, so its top level needs no type annotation at all — the type flows in from the load. The body only resolves the genuinely-body-level inner annotations (the `Engine`). So the redundancy doesn't just move, it disappears.

And it directly closes two of your open `// V:` markers: `ast.rs:367` ("don't generate a pattern when there's no destructure — simpler AST") becomes automatic, because the signature stops carrying a pattern; and the `function_scout.rs:432` throwaway is gone.

Let me verify the two things that could sink it: what else reads `ParameterS.pattern`, and whether the parser pattern is available at body-synthesis time.

---

## 🤖 Claude

The greps confirm it's feasible, and better than I expected. Here's the blast radius.

## What actually reads `ParameterS.pattern`

**Live postparse — only three real consumers, all replaceable:**
- `get_parameter_captures(&param.pattern)` (`:543`) — builds the body's variable decls.
- `pattern: param.pattern.clone()` (`:779`) — the body-head `LetSE`.
- magic/closure param handling (`:751`, synthesized params).

The first two are exactly the two jobs your proposal moves to the body. And critically: the signature **already** computes the type independently of `translate_pattern`. Look at `:444–459` — the param type comes from `translate_signature_templex(type_p)` reading `parser_param.pattern.templex` directly. The `translate_pattern` call at `:418` produces *only* the throwaway rules plus a skeleton that gets cloned to the body. So in the signature, `translate_pattern` is already almost pure waste — your proposal just deletes it.

**Typing pass — ~10 reads, all `param.pattern.kind_rune` (the type), plus a few `.name`/`.destructure`.** These migrate to reading a type field directly off `ParameterS`, which is churn we already owe (the pass is stale and being rewritten for onion). Reading a param's type out of a pattern's `kind_rune` is itself a symptom of the conflation you're removing.

## The restructure

- **Signature (param loop):** translate only `parser_param.pattern.templex` → the param's type rune + rules. Drop the `translate_pattern` call, `discarded_pattern_rules`, and the whole throwaway. `ParameterS` stops carrying the destructure.
- **Body synthesis:** for each param with a body, call `translate_pattern` on the **parser** pattern (`params_p`, already in scope at `:634`) into the `LetSE`'s real rule bucket — the once, like a normal `let`. Derive captures from that same pattern.
- **`ParameterS` shape:** loses `pattern`; keeps `{ range, virtuality, pre_checked, name (ABI slot), type… }`. That *is* the `ast.rs:367` marker resolved — no pattern when there's no destructure, because the signature never holds a pattern at all.

The body-head `LetSE` also gets simpler: it destructures a load of the already-typed param slot, so its top level needs no type annotation. The type flows from the load; the body only resolves genuinely-inner annotations (the `Engine` in your example). The redundancy doesn't move, it's gone.

## Gotchas to decide on (none look fatal)

- **Blockless bodies** (abstract/extern/generated) have no body to hold a `LetSE`. They already reject param destructures (`ParamDestructureRequiresBody`), so they just need the param *type* from the signature — fine, and the existing check stays.
- **Synthesized params** (`&self`, closure, magic `_`) have no parser pattern to re-translate. They stay special-cased: synthesize their body-binding `LetSE` directly, as today. Simple, but it means "build the body pattern" has two sources (parser pattern for user params, synthesized for these).
- **Typing-pass migration** (`pattern.kind_rune` → a type field, and the anon-interface macro's `pattern.destructure`/`.name` reads) is the biggest single chunk, but it's all in gated code being rewritten anyway.

## How it relates to the ref-split question

It's **orthogonal but complementary.** Your proposal removes the destructure-vs-type conflation; the earlier A/B/C question (store the `type_outer_ref_rules`/`value_type_rules` split, or flat rules + derive) is about the *type* translation alone. With the throwaway gone, that decision gets cleaner because it's no longer entangled with the pattern walk — the signature just has "translate this type annotation," and you can choose split-or-flat there in isolation.

My read: this is the decomposition that finds the real building block — **signature = types, body = patterns, overlapping only at the type annotation the signature reads directly.** I'd want to do it before (or together with) settling the ref-split, since it simplifies the ground the ref-split decision stands on.

Want me to scope it concretely — the exact edits to the param loop and body-synthesis block, plus how the synthesized params (`&self`/closure/magic) fold in — as a plan for your approval before any code changes?

---

## 👤 User

tell me about the Synthesized params quirk

---

## 🤖 Claude

Let me read the three synthesized-param paths and how the body-head `LetSE` currently treats them uniformly.

---

## 🤖 Claude

Now the closure param builder:

---

## 🤖 Claude

I've now read all three synthesized-param builders. Here's the quirk in full.

## What the synthesized params are

Three of a function's `ParameterS`es are never written by the user — postparse fabricates them:

- **`&self`** — the `(Some(_), None)` arm (`:371–405`). Name is the interned `self_`; no parser `PatternPP` exists for it.
- **The closure param** — `create_closure_param` (`:900–979`). Binds the lambda's closure struct so the body can load captured variables from it. Name and type are wholly synthesized.
- **Magic params** (`_`, `_2`, … in lambdas) — `create_magic_parameters` (`:982–1029`). Built from just a `MagicParamName`; no parser pattern.

## Why they're a quirk for your proposal

Today, *every* param carries an `AtomSP` on `ParameterS.pattern`, and that pattern does double duty. It holds the param's **type rune** (a signature thing) *and* the user's **binding name + destructure** (a body thing). The body-head loop (`:770–782`) then treats all params identically: for each one it emits `<param.pattern> = load(param.name)`, which is what binds the user-facing name to the ABI slot.

Your proposal says "rebuild the pattern from the parser pattern at body time." That works for user params — they *have* a `PatternPP` to re-translate. But the three synthesized params **have no parser pattern to rebuild from.** Their binding name is generated during signature processing (`self`, the closure name, the magic name) and currently lives nowhere but `ParameterS.pattern`. Strip `.pattern` from `ParameterS` and that name has no home.

So the underlying tension your proposal exposes: `ParameterS.pattern` is the single carrier that unifies "user-written binding" and "fabricated binding." Splitting parameter (signature) from binding (body) means the **binding list now has two sources** — parser syntax for user params, fabrication for synthesized ones.

The saving grace: **a synthesized binding is always trivial.** None of the three ever destructures; each is just "bind one generated name to one slot." So the fabricated side is the simplest possible `LetSE` — `self = load(slot)`, never a nested pattern.

## Two ways to resolve it

- **Option A — synthesized params keep a tiny binding, user params don't.** `ParameterS` drops the destructure but keeps an `Option<binding-name>` (or a name-only `AtomSP` that's guaranteed destructure-free). User params leave it `None` (their binding is rebuilt from the parser pattern); synthesized params fill it with the fabricated name. Cheapest change, but asymmetric, and `ParameterS` still carries a body-ish field.
- **Option B — bindings leave `ParameterS` entirely.** `ParameterS` becomes pure signature (`range, virtuality, pre_checked, abi_name, type`). The body-synthesis step builds *all* bindings as `LetSE`s: user ones from the parser pattern, synthesized ones fabricated at the point of synthesis (each of `create_closure_param` / `create_magic_parameters` / the `&self` arm returns its binding alongside its `ParameterS`, or the body loop special-cases them). Cleanest separation — it's the full realization of "parameter is signature, binding is body" — but it touches all three synthesizers.

## The one constraint either way

The synthesized params are **interleaved with user params in a fixed order** — closure param first, then user params, then magic params (`:731–763`) — and the body-head `LetSE`s must be emitted in that same order to line up with the ABI slots. So whichever option, the binding list has to be built preserving param order, not as two separate appended groups unless that order is honored.

My lean is **B**: it's the version that actually finds the building block you're after (`ParameterS` stops being a body/signature hybrid), and the synthesized bindings are so trivial that fabricating them at body-synthesis is a couple of lines each. **A** is the smaller diff if you'd rather keep the blast radius down and accept a residual name field on `ParameterS`.

Want me to look at exactly how much each synthesizer would have to change under B (what info each would need to hand back for its body binding), so you can see the real size of it before deciding?

---

## 👤 User

for &self, there is no pattern, so im not sure what the problem is.
same with the closure param.
same with magic params. none of these specify patterns. so i dont see the problem. what problem are you seeing?
also, im not sure we *should* have DesugaredParamName. is that what's causing this problem?

---

## 🤖 Claude

You're right, and I was inventing the problem. Let me correct myself, then investigate the `DesugaredParamName` question properly rather than guess.

**My error:** I conflated "name" with "pattern." A parameter intrinsically *has a name and a type* — `self`, `x`, `_`, the closure name. That's not a body concern; that's just what a parameter is. The **destructure** is the only body concern. So the synthesized params aren't a quirk at all: `&self` is a param named `self`, the closure param has the closure name, magic params are named `_`. Each has a name and a type, full stop. There's nothing to "rebuild from a parser pattern" because there's nothing to destructure. I manufactured a difficulty by assuming `ParameterS` had to drop its name and reconstruct everything — which is wrong.

And you've put your finger on *why* I got confused: `DesugaredParamName`. Let me check whether it's actually load-bearing or just imposed uniformity.

---

## 🤖 Claude

`DesugaredParamName` and the per-param body-head `LetSE` are **both new to experimental-2** — master had neither. Master's `ParameterS` just carried the whole pattern (name + destructure) directly, and the typing pass destructured params in place. Let me find why experimental-2 introduced the desugar.

---

## 🤖 Claude

The `-S` search found nothing because it's all **uncommitted** working-tree work. Let me check the diff and any handoff notes for the rationale.

---

## 🤖 Claude

HEAD has zero `DesugaredParamName` — it's entirely uncommitted working-tree work, which is why the history search came up empty. Let me read the handoff's rationale for the desugar.

---

## 🤖 Claude

The handoff doesn't cover it. Let me check the planning docs directly.

---

## 🤖 Claude

No documented rationale anywhere — not in the handoff, `onion-typing-plan.md`, `postparse-slice-plan.md`, or `architectural-direction.md`. So here's the honest picture.

## Answering your questions directly

**"What problem am I seeing?"** — None real. I invented it. `&self`, the closure param, and magic params don't specify patterns, and they don't *need* to, because a parameter is fundamentally **a name and a type**, and all three have exactly that: `self`, the closure name, `_`. There's nothing to rebuild and nothing to defer. My "quirk" came entirely from assuming `ParameterS` had to surrender its name and reconstruct bindings from parser patterns — which is only true if you've already committed to desugaring *everything*.

**"Is `DesugaredParamName` causing this?"** — Yes. That's the root of it. What I found:

- `DesugaredParamName` and the per-param body-head `LetSE` are **both new to experimental-2**. HEAD has zero occurrences; it's uncommitted working-tree work.
- **Master did none of it.** Master's `ParameterS` was `{ range, virtuality, pre_checked, pattern }` — it carried the real name and destructure on the param, and postparse never synthesized body-head lets.
- The current design desugars **every** param: even a plain `x &Ship` gets a synthetic `DesugaredParamName` slot plus a body `let x = load(slot)`. That blanket desugar is the artificial uniformity that made the synthesized params look like they needed special handling. They only looked special because *everything* was being forced through "synthetic slot + body let."

## The simpler model this points to

Desugar to a body let **only when there's a destructure** — the one case that actually needs it (one ABI slot exploding into several names). Otherwise a param keeps its real name:

- `foo(x &Ship)` → param `x : &Ship`. Body refers to `x`. **No let, no synthetic name.**
- `&self` → param `self : &Self`. **No let.**
- closure / magic → params with their own names. **No let.**
- `foo([a, b] Pair)` → param `<syn> : Pair` **+** body `let [a, b] = load(<syn>)`. **Desugar only here**, and this is the *only* place `DesugaredParamName` is needed.

Under this, the signature peeks at the pattern only enough to ask "is there a destructure?" — if no, take the capture name as the param's name; if yes, synthesize a name and defer the destructure to the body. That's fully consistent with your "signature looks at the outer type" idea, just extended to the outer *name* too. `ParameterS` becomes `{ range, virtuality, pre_checked, name, type }`, the synthesized params are ordinary name+type entries, and the throwaway is gone — all at once.

## The one thing I can't answer, and won't guess

I don't know **why** the blanket desugar was introduced, because it's undocumented and the code that would justify it (the typing pass) is gated. I can imagine a real reason — e.g. wanting a single destructure code path, or treating params as ordinary locals so borrow/capture/`set` analysis is uniform — but that would justify desugaring *destructure* params, not *all* params. If one of those was the actual intent, that changes whether we restrict it or keep it.

Since this is your architectural decision and undoing the blanket desugar is a real change (not a cleanup): **do you know why every param was desugared to a body let, rather than only the destructuring ones?** If it was deliberate, what were you protecting? If it was just "seemed uniform," then restricting the desugar to destructures is the building block you're after.

---

## 👤 User

it wasnt introduced for any particular reason. i think the thought was that we would have a let statement for *every* parameter, and the name would be introduced by the let statement. but that wasnt actually my intent, even back then i only wanted a let statement for parameters that actually had destructures, though the claude from that session misunderstood

---

## 🤖 Claude

That lands it — and it matches everything the code shows. The blanket desugar was never the intent; only destructures should get a body let. So the target model is:

## Confirmed target

- **Simple param** `x &Ship` → `ParameterS` named `x`, with its type. Body refers to `x` directly. **No let, no `DesugaredParamName`.**
- **`&self` / closure / magic** → `ParameterS` named `self` / the closure name / `_`, with its type. **No let.** (These stop being special — they're just name+type like any simple param.)
- **Destructure param** `[a, b] Pair` → `ParameterS` with a synthetic `DesugaredParamName` slot + type `Pair`, **plus** a body-head `let [a, b] = load(slot)`. This is the *only* case that desugars, and the only place `DesugaredParamName` survives.

Consequences that fall out for free:
- `ParameterS` loses `pattern: AtomSP` — it holds a plain `name` (real, or synthetic only for destructures) + the type. That's `ast.rs:367` resolved.
- The destructure `AtomSP` is built from the parser pattern at body-synthesis time and lives in the body let, exactly like a normal `let`. `translate_pattern` runs once, into the real bucket. The throwaway (`function_scout.rs:432`) is gone.
- `get_parameter_captures` reduces: a simple/synthesized param declares its own name; a destructure param's inner names come from the body let. No signature-side pattern walk.

This is essentially **master's simplicity for the common case, keeping experimental-2's body-let desugar but correctly scoped to destructures only.**

## One axis still genuinely open

How the **type** sits on `ParameterS`. That's the earlier A/B/C question, now cleanly isolated because it no longer tangles with the pattern:
- **flat** — `type_rune` + a single `type_rules` (master-style: rules could even go into the function's one bucket), typing pass derives the value-type-past-the-wraps by walking the ref chain when it needs it; or
- **split** — keep `full_type_rune` / `value_type_rune` / `type_outer_ref_rules` / `value_type_rules` materialized, per the current `@PFVSZ` design.

My read remains that flat is the simpler building block (the split is derivable and currently has zero consumers), but that's the one real decision left, and it rewrites the `@PFVSZ` arcana either way.

Want me to settle the flat-vs-split question with you now, then write this up as a proper plan (param-loop changes, body-synthesis changes, the three synthesizers, `ParameterS` shape, and the typing-pass field migration) for your approval before any code? I don't want to start editing a green, uncommitted tree unilaterally.

---

## 👤 User

are you sure `[a, b] Pair` is valid syntax? pretty sure it would be `Pair[a, b]`.
also, what about if the user says `p Pair[a, b]`?

---

## 🤖 Claude

You're right, and I even had that in the prior session log (we reverted the destructure-first path because type comes before the destructure). Let me check the actual grammar rather than keep guessing at syntax.

---

## 🤖 Claude

Confirmed from the AST — and my syntax was wrong. `PatternPP` has **three independent optional pieces**:

- `destination` — the name (`p`)
- `templex` — the type (`Pair`)
- `destructure` — the destructure (`[a, b]`)

So the type comes first and the destructure follows it: `Pair[a, b]`, and `p Pair[a, b]` is all three at once — name `p`, type `Pair`, destructure `[a, b]`. Thanks for the correction.

This actually sharpens the model, because it shows the two things I'd been fusing are **orthogonal conditions**:

- **Synthetic name (`DesugaredParamName`) is needed ⟺ there's no user name.**
- **A body-head let is needed ⟺ there's a destructure.**

Full truth table over (has-name?, has-destructure?):

| Param | Param name | Body let? |
|---|---|---|
| `p Pair` | `p` | no |
| `Pair[a, b]` | synthetic | `let Pair[a, b] = load(slot)` |
| **`p Pair[a, b]`** | **`p`** | **`let [a, b] = load(p)`** |
| `_ Pair` | synthetic (or anon slot) | no |

So your `p Pair[a, b]` case: the param keeps its real name `p` (no synthetic needed), of type `Pair`, **and** a body let destructures `p` into `a` and `b`. `p`, `a`, and `b` are all in scope in the body. The let's pattern is just the destructure part — the outer name `p` is already the param, and the type `Pair` already rides in on `load(p)`, so the let doesn't restate either.

This means `DesugaredParamName` isn't really "the destructure name" — it's "the fallback name when the user didn't write one" (anonymous destructure, or an ignored param). And the body let is driven purely by the presence of a destructure, independent of naming.

That's cleaner than what I said before: two small, orthogonal rules instead of one fused "destructure ⇒ desugar everything" rule. Does this match your intent for the named-destructure case — param keeps `p`, body let peels `a`/`b` off it?

---

## 👤 User

also, what do you mean that ParameterS no longer needs to do the split? i think it still needs the split

---

## 🤖 Claude

You're right, and I need to walk back what I said — I was sloppy, and "flat vs split" was a bad framing. The split stays. Let me separate the two things I'd carelessly fused.

**What the desugar/pattern refactor actually removes:** `ParameterS.pattern` (the `AtomSP`) and the `translate_pattern` throwaway. Nothing else.

**What it does *not* touch: the type split.** The split is computed by `translate_signature_templex` from the param's *type annotation* — that call (`function_scout.rs:447–456`) is already completely independent of the `translate_pattern` walk. Removing the pattern from the signature leaves it untouched. So after the refactor, `ParameterS` is:

```
{ range, virtuality, pre_checked, name,
  full_type_rune, value_type_rune, type_outer_ref_rules, value_type_rules }
```

It loses only `pattern`. The four type fields — the split — remain.

**And you're right that the split is genuinely needed, not just a derivable convenience.** The reason is in your own overload model in the handoff: a function is findable through a parameter by the parameter's *underlying value kind*. `foo(x &Ship)` gets indexed in `Ship`'s namespace, ignoring the `&` ("the underlying kind is what matters"). So each parameter has to expose its **value type separated from its outer refs**, at the signature level — which is exactly `ParameterS`. That's a load-bearing requirement, and `ParameterS` is where it's consumed.

I also have to retract my "zero consumers, so maybe drop it" point — that was weak. The consumer is the overload indexer, which lives in the gated typing pass. "Zero consumers today" just reflects the gating, not that the split is unneeded. My mistake for leaning on it.

So, corrected: the split is not on the table. `ParameterS` keeps its value/full type separation; the refactor's whole job is to stop the signature from processing the *pattern* (destructure/binding), which is the body's concern, and to kill the throwaway. Two orthogonal things, and I conflated them.

Does that square with how you're seeing it — split stays, pattern goes?

---

## 👤 User

sounds good. please plan out this new direction

---

## 🤖 Claude

I've already explored this code deeply over the last several exchanges — I have the param loop, body-synthesis block, `translate_pattern`, the three synthesizers, the split, and the typing-pass consumers mapped. Let me close the two remaining gaps before writing the plan: the exact test/traverse migration surface, and the precise body-synthesis edit points.

---

## 🤖 Claude

While that maps the test surface, let me read the exact body-synthesis block, the `&self` arm, and the capture flow precisely.

---

## 🤖 Claude

Now let me read the capture-declaration flow precisely, since that's the subtle part (how a param's name gets declared as a body variable):

---

## 🤖 Claude

Now I have the capture flow. This refactor has real intricacy in three places — the param-loop's return shape, how synthesized params interleave with body-lets, and the capture-declaration flow. Let me launch a Plan agent with full context to design the detailed data flow while the test-surface agent finishes.

---

## 🤖 Claude

Both agents are running (test-surface inventory + detailed data-flow design). I'll wait for both, then write the plan incorporating their findings. Holding for the completions.

---

## 🤖 Claude

Important discrepancy to reconcile: the agent reported the repo is "RED with ~172 compile errors." That's reading the `TEMP CHECKPOINT` commit *messages*, not the current tree. The session baseline is **green at 501/0/1 with `typing`/`tests`/`solver` gated out in `lib.rs`**. Let me confirm the gating so I scope the plan correctly.

---

## 🤖 Claude

Confirmed — `typing` (30-31), `tests` (32-33), and `solver` (38-40) are all gated with `#[cfg(any())]`. The tree is green at 501/0/1 building parser+postparse only. So the agent overstated scope: the ~20 typing read-sites and the 4-arg `ParameterS::new` calls it flagged are **gated and already stale** (they don't match today's 9-arg `new`), so they're part of the separate typing-un-gating work, not this refactor. This refactor is postparse-only.

That's an important scoping correction. Now waiting on the design-data-flow Plan agent before writing the plan.

---

## 🤖 Claude

Both agents independently claimed "RED per git log" — that's the same misread of the `TEMP CHECKPOINT` commit messages. Let me settle it definitively with a read-only check of whether the typing-gating is committed or working-tree-only, so the plan's baseline is accurate.

---

## 🤖 Claude

Settled: at HEAD, typing is **not** gated (`pub mod typing;`), so HEAD is genuinely RED — that's what both agents saw. The **working tree** adds the uncommitted `#[cfg(any())]` gating, which excludes typing/tests/solver and builds **green at 501/0/1**. So this refactor operates on the green working tree; the ~20 typing sites are gated and out of scope. I now have everything to write the plan.

---

## 🤖 Claude

The plan is written. Let me load and call ExitPlanMode to request approval.

---

## 🤖 Claude

Plan approved. Let me set up a task list and start implementing.

---

## 🤖 Claude

Starting with task #1. Let me verify the `IVarNameS` variants before writing the whitelist assert.

---

## 🤖 Claude

Let me mark task 1 in progress and check the `IVarNameS` variants.

---

## 🤖 Claude

Variants confirmed: `CodeVarName`, `ConstructingMemberName`, `ClosureParamName`, `MagicParamName`, `DesugaredParamName`. Also note the `DesugaredParamName` doc in `names.rs:256-261` says "All params get one" — now inaccurate, I'll fix it too. Now the `ast.rs` edits:

---

## 🤖 Claude

Now the `new` signature and asserts:

---

## 🤖 Claude

Now fix the `names.rs` doc comment for `DesugaredParamName` (both the value and interned enums have it):

---

## 🤖 Claude

Task 1 done. Now the big one — task 2 (param loop). Let me re-read the exact current state of the param loop region.

---

## 🤖 Claude

Now let me look at the top of the file to place the helper struct and confirm imports:

---

## 🤖 Claude

I need to place the `ExplicitParamExtras` struct and import `INameDeclarationP`. Let me add both:

---

## 🤖 Claude

Guardian's **NNDX** shield (no new definitions without a Scala counterpart) blocked the new `ExplicitParamExtras` struct — the same Scala-parity shield that fired in the prior session (on `translate_signature_templex`). The file was **not** modified by that call; everything else so far (the `ParameterS` shape change, `names.rs` doc) is in place.

Ordination is per-session, so this fresh session isn't ordained. Three ways forward:

- **Ordain this session** — re-run `/guardian-ordain "thunder road" <port>` (the port may differ from last session's `7880`), then I add the struct and continue.
- **Skip the struct** — carry the extras as a plain tuple `(RangeS, IVarNameS, Option<(AtomSP, &[IRulexSR])>)` instead. No new definition, so NNDX won't fire and no ordination is needed — at the cost of a less-readable 3-tuple threaded through the param loop and body-synthesis.
- **Hold** and you tell me how you'd rather handle it.

How do you want to proceed?

---

## 👤 User

ordain "thunder road" 7880 please

---

## 🤖 Claude

Ordained (`{"success":true}`). Now inserting the `ExplicitParamExtras` struct:

---

## 🤖 Claude

Now the param loop rewrite. I'll do it in three edits. First, the declaration line + the `&self` arm:

---

## 🤖 Claude

Now the user-param arm:

---

## 🤖 Claude

Now the split loop that unpacks the tuples:

---

## 🤖 Claude

Task 2 done. Now let me find the current line numbers for captures and body-head lets (shifted after edits):

---

## 🤖 Claude

Now task 4 — the capture-declaration loop:

---

## 🤖 Claude

Now task 3 — the synthesized params. Let me read the magic→GenericParameterS region and the closure/magic builders:

---

## 🤖 Claude

Now retarget the magic→GenericParameterS reads (task 3, part):

---

## 🤖 Claude

Now task 5 — the body-head let loop. It must iterate `explicit_param_extras` and emit lets only for destructures:

---

## 🤖 Claude

Now the remaining part of task 3 — the closure and magic param builders. Let me read their current state:

---

## 🤖 Claude

Now update the closure param builder — remove the `AtomSP`, use the real name:

---

## 🤖 Claude

Now the magic param builder:

---

## 🤖 Claude

Let me verify `RuneUsage` is `Copy` (I used it twice without clone in the closure edit):

---

## 🤖 Claude

`RuneUsage` is `Copy` — the closure edit is fine. Now the magic param:

---

## 🤖 Claude

Tasks 3-5 done. Let me build to catch compile errors before touching tests.

---

## 🤖 Claude

Build succeeded. Let me check for warnings:

---

## 🤖 Claude

Clean build, zero warnings. Let me check whether `CaptureS` is now an unused import (the file allows `unused_imports` globally, so it won't warn, but I should keep it clean):

---

## 🤖 Claude

`CaptureS` is now unused — remove it:

---

## 🤖 Claude

Now let me run the test suite to see what needs updating in the tests:

---

## 🤖 Claude

Test-compile errors on `param.pattern`. Let me get exact locations:

---

## 🤖 Claude

Let me start with `traverse.rs` and read the two test regions:

---

## 🤖 Claude

`visit_parameter` previously reached only the param's (full) type rune via `pattern.kind_rune`; the destructure now lives in the body `LetSE` (traversed separately). I'll mirror the old coverage — visit `full_type_rune`:

---

## 🤖 Claude

Now the two lambda-param test assertions. Let me read both regions:

---

## 🤖 Claude

Now rewrite both lambda-param matches to check `name` + `full_type_rune` directly (no pattern):

---

## 🤖 Claude

Let me recompile the tests to clear remaining compile errors:

---

## 🤖 Claude

Tests compile. Now run them to find behavioral failures:

---

## 🤖 Claude

3 failures, exactly the blanket-desugar tests the plan flagged (498+3 = 501, no other regressions). Let me read and rewrite each. First `test_param_gets_desugared_name_at_postparse`:

---

## 🤖 Claude

Rewrite `test_param_gets_desugared_name_at_postparse` — a named param now keeps its real name:

---

## 🤖 Claude

Now the bare-param test. Under the new behavior it gets **no** body-let. Let me rewrite it (and the shared comment above):

---

## 🤖 Claude

Now the third — `test_extern_bare_param_ok`. Let me read it:

---

## 🤖 Claude

Now run the full suite to confirm the three are fixed:

---

## 🤖 Claude

Green at 501/0/1. Now let me add the planned test for the named-destructure case (`p Pair[a, b]` → param name `p` **plus** the `[a,b]` let). Let me read the existing anonymous-destructure test to model it:

---

## 🤖 Claude

Before adding the `p Pair[a, b]` test, let me confirm the parser actually supports the combined name+type+destructure form (I don't want a test failing for parser reasons):

---

## 🤖 Claude

The parser reads destination → templex → destructure, all optional in sequence — so `p Pair[a, b]` parses. Let me confirm there's an existing parser test of the combined form:

---

## 🤖 Claude

Confirmed — the combined form parses (`_ (int, bool)[a, b]` gives destination + templex + destructure). Now let me add the named-destructure postparse test after the anonymous one:

---

## 🤖 Claude

Now run the full suite with the new test:

---

## 🤖 Claude

502 passed (501 + the new test), 0 failed. Now the verification sanity checks:

---

## 🤖 Claude

All four remaining `.pattern` reads are legitimate (parser params at 358/380, `LocalS` at variable_tests:139, `LetSE` at traverse:488) — no stale `ParameterS.pattern`. Zero warnings, no `discarded_pattern_rules`, all 4 `ParameterS::new` callers on the 8-arg form. Let me confirm the renamed/new tests actually ran:

---

## 🤖 Claude

All key tests pass, including the three split-routing tests — confirming the type split is untouched. Let me close the tasks.

---

## 🤖 Claude

Done. The refactor is complete and green.

## What landed

A parameter is now **a name + a type**, not a pattern. `translate_pattern` runs at most once per param, only when there's a destructure, and nothing is thrown away.

**`ParameterS` (`ast.rs`)** — dropped the `pattern: AtomSP` field (`::new` is now 8 args). Deleted the `pattern.kind_rune` assert; relaxed the name assert to a whitelist of legal param-name variants (real names now allowed); kept the two split-bucket asserts. The type split (`full_type_rune` / `value_type_rune` / `type_outer_ref_rules` / `value_type_rules`) stays.

**Param loop (`function_scout.rs`)** — new `ExplicitParamExtras` carry struct (needed ordination — NNDX fired, you re-ordained on 7880). Each param derives its **real name** from the parser destination (`p` → `CodeVarName`, ignored/anonymous → synthetic `DesugaredParamName`); the split is still computed by `translate_signature_templex`; the throwaway `translate_pattern` block is gone. A destructure builds its body-let pattern **once** here (typeless at top), rules destined for the let's real bucket.

**Synthesized params** — `&self`, closure, and magic params now keep their real names (`self` / closure name / `_`) instead of `DesugaredParamName`; their inline `AtomSP`s are gone; magic→`GenericParameterS` reads retargeted to `full_type_rune`/`range`.

**Captures & body-lets** — capture declarations gather off the extras (real name + destructure inner names); a body-head `LetSE` is emitted **only** for destructuring params, wrapped in a `ConsecutorSE` only when at least one exists.

**Tests** — rewrote the three blanket-desugar tests to the new behavior (bare param keeps its name + no let; extern bare param keeps its name), renamed them accordingly, updated the two lambda-param matches to read `name`/`full_type_rune`, fixed `traverse.rs`, and **added** `test_named_destructure_param_keeps_name_and_gets_let` for the `p Pair[a, b]` case.

## Verification
- `cargo build --lib` — clean, **zero warnings**.
- `cargo test --lib` — **502 passed / 0 failed / 1 ignored** (501 base + the new test). The three split-routing tests pass, so the type split is byte-identical.
- No stale `ParameterS.pattern` reads or `discarded_pattern_rules` in postparse; all 4 `ParameterS::new` callers on the 8-arg form.

## Still open (deferred, as planned)
- **`p Pair[a, b]` load semantics** — the head-let uses `LoadAsP::Use` (status quo); whether `p` stays usable after the destructure is a typing/runtime decision to confirm when typing is un-gated.
- The `// V: lets get rid of rune_to_explicit_type` marker in the `&self` arm is untouched (separate deferred TODO).
- The ~20 gated `typing/` sites reading `param.pattern.*` remain part of the typing-un-gating slice.

Everything's uncommitted on `experimental-2`, pending your `fire commit`.

---

## 👤 User

<command-message>valec-reviewer</command-message>
<command-name>/valec-reviewer</command-name>
<command-args>please fix the tests to have less assertions and more collect/match</command-args>

---

## 👤 User

does valec-reviewer.md say to not have if statements in tests?

---

## 🤖 Claude

That line 212 is inside the "Avoid early-returns" rule's BEFORE example (production code), not a rule about tests. Let me search the whole doc for any actual rule about `if` in tests:

---

## 🤖 Claude

Let me read the most relevant rule — "Assert only what a match can't express":

---

## 🤖 Claude

No — there's no rule literally titled "no `if` statements in tests." The line you selected (212) is inside the **"Avoid early-returns"** rule's BEFORE example, which is about production code, not tests.

But two rules do govern this, and their **spirit** says a test shouldn't use `if let … panic!` to check a shape:

- **"No `|` in a test's match pattern"** (185) — the house form for pinning a shape is `match { ExactShape => {}, other => panic!(...) }`.
- **"Assert only what a match can't express"** (268) — "Anything that pins a shape is a match… never a `.any`."

So the doc's position is: **shape-checks in tests are `match` statements, not `if`.**

And you're right to point at it — my new `test_bare_param_keeps_name_and_gets_no_body_let` violates that spirit. It uses:

```rust
if let IExpressionSE::Consecutor(cons) = &code_body.body.block.expr {
    if let Some(IExpressionSE::Let(_)) = cons.exprs.first() {
      panic!("bare param should not produce a body-head LetSE");
    }
}
```

That's a nested `if let … panic!` doing a shape-check — exactly the ad-hoc form those rules push you away from. It should be a `match` that pins the body head's actual shape, with `other => panic!`.

The catch: it's asserting an *absence* (no leading param-let), so the clean fix is to pin what the empty body head positively **is**. Want me to rewrite it that way — check the real empty-body expr shape and match on it — so the test conforms? And, if it's worth codifying, we could add a rule making the "shape-checks are matches, not `if let`" point explicit, since it's currently only implied.

---

## 👤 User

yeah, that one in test_bare_param_keeps_name_and_gets_no_body_let is particularly bad. tests should check for an exact, expected result, not be wishy-washy and allow invalid results.
please add to valec-reviewer.md about this

---

## 🤖 Claude

You're right — a test that lets invalid results pass is worse than useless. Let me first find the exact shape of an empty body head so I can pin it precisely. Let me check how existing tests inspect a simple body expr:

---

## 🤖 Claude

There's a `Void(VoidSE)` variant — likely the empty-body head. Let me rewrite the test to the exact-match form (guessing `Void`, with a catch-all that reports the real shape) and run to confirm:

---

## 🤖 Claude

The empty-body head is `Void` — the test now pins the exact shape and fails on any Consecutor/param-let. Now let me add the rule to `valec-reviewer.md`. Let me check the end of the file:

---

## 🤖 Claude

Now let me run the full suite to confirm everything's still green after the rewrite:

---

## 🤖 Claude

Done. Both parts landed.

**Fixed the test.** `test_bare_param_keeps_name_and_gets_no_body_let` now pins the exact expected shape instead of the wishy-washy `if let … panic!`:

```rust
match &code_body.body.block.expr {
    IExpressionSE::Void(_) => {}
    other => panic!("expected an untouched Void body head (no param let), got {:?}", other),
}
```

I confirmed empirically that a bare param's empty body head is `Void` (the old `if let` would have silently passed for *any* non-Consecutor shape, including a wrong one; this fails on anything but `Void`).

**Added the rule** to `valec-reviewer.md` — **"Pin the exact result; don't `if let` a shape check"** — in the doc's house format, with the exact BEFORE/AFTER from this case. It states the principle you named: a test asserts the one correct shape, and an `if let … panic!` lets every other invalid result pass silently.

Suite still green: **502 passed / 0 failed / 1 ignored, zero warnings.**

Everything remains uncommitted on `experimental-2`, pending your `fire commit`.

---

## 👤 User

<command-message>valec-reviewer</command-message>
<command-name>/valec-reviewer</command-name>
<command-args>everything uncommitted please</command-args>

---

## 👤 User

<command-message>prose-reviewer</command-message>
<command-name>/prose-reviewer</command-name>
<command-args>everything uncommitted please</command-args>

---

## 🤖 Claude

I'll review all uncommitted changes against both rule sets, focusing on this session's additions (the param-desugar refactor). Let me pull the diff of what I touched.

---

## 🤖 Claude

Let me check for em-dashes directly and review the test-file additions:

---

## 🤖 Claude

The em-dashes found are all in the reviewer **docs' own prose** (BEFORE/AFTER labels and rationale) — allowed, since the rule is scoped to code comments. No em-dashes in my code comments. Let me review the substance.

Findings, applied and surfaced:

**Fix 1 — `ParameterS.name` doc (prose: list rule + a redundancy).** "A user-written name keeps its real name" is circular, and the three cases read better as a list:

---

## 🤖 Claude

**Fix 2 — the normal-param match arm has no "what leads here" comment** (the `&self` and typed/untyped arms do). Add one for consistency with the "comment a non-obvious match arm" rule:

---

## 🤖 Claude

Let me confirm the doc/comment edits still build clean:

---

## 🤖 Claude

Clean. Here's the review outcome across the uncommitted diff.

## Applied (my session's additions)

- **`ParameterS.name` doc → list + de-circularized** (`ast.rs`). "A user-written *name* keeps its real *name*" was circular; the cases now read as a list (prose-reviewer: "use a list for equal sub-parts").
- **Normal-param match arm now names its case** (`function_scout.rs`) — `// A normal param, e.g. \`foo(x int)\`, \`foo(&Ship)\`, or \`foo(Pair[a, b])\`.`. The `&self` and typed/untyped arms already had case comments; this one didn't (prose-reviewer: "comment a non-obvious match arm").
- (Earlier this turn) **bare-param test rewritten** to pin the exact `Void` head instead of `if let … panic!`, and the **"Pin the exact result"** rule added to `valec-reviewer.md`.

## Clean

The rest of my new code passes both rule sets: no em-dashes in code comments (the em-dashes the scan found are all in the reviewer docs' own prose, which is out of scope); every gate routes on the honest name variant (`matches!(abi_name, DesugaredParamName(_))`), not a count; new tests use the endorsed `assert!(matches!(…))` / `match { … other => panic! }` forms; no `|` in any test pattern; match arms are siblings with no early-return hoists.

## Surfaced — pre-existing, not fixed (your call)

Three routing tests carry `.any(|r| is_lookup_of(r, "int"))` assertions that violate **"Assert only what a match can't express"** — `test_param_no_outer_wrap_routing` (`post_parser_tests.rs:118`, `:125`), `test_param_single_ref_wrap_routing` (`:155`), and the two-borrow test (`:210`). They're already flagged by the `// VADD:` markers sitting right above them and were parked as the **category-4 test-cleanup** work (convert `.any` → `collect_only_snode!`). I left them because they're a known deferred task, not this session's code — say the word and I'll do that conversion.

Everything still builds clean, zero warnings; suite green at 502/0/1. All uncommitted, pending `fire commit`.

---

## 👤 User

please do that conversion now

---

## 🤖 Claude

Let me read the three routing tests in full to convert them precisely:

---

## 🤖 Claude

This conversion is broader than just `.any` — per the rule it also collapses `len()==N` + `[i]` element checks into single slice patterns, and the `if let` at 1433 silently skips its asserts if unmatched. Let me convert all three tests. First, the two self-contained param-slice checks in the no-outer and single tests:

---

## 🤖 Claude

Now the single-ref test — collapse `len==1` + `[0]` + the silent `if let` into one exact match:

---

## 🤖 Claude

Now the nested-ref test — collapse `len==2` + index extraction into one slice match:

---

## 🤖 Claude

Now the `test_function_rules_no_longer_contains_param_rules` absence checks — pin `foo.rules` positively as `[Lookup(void)]`, which inherently excludes the param lookups:

---

## 🤖 Claude

I removed the now-resolved `// VADD:` and `// V: need arcana` markers along with their `.any` code. Let me run the tests to verify the pins (especially the `foo.rules == [Lookup(void)]` assumption):

---

## 🤖 Claude

All green — 502 passed. The `foo.rules == [Lookup(void)]` pins held, and the tests are now stronger (exact shapes, not "contains"). Let me confirm no `.any` remains in these tests:

---

## 🤖 Claude

Done. The `.any` conversion is complete and the tests are now stronger — they pin **exact** shapes instead of "contains."

## Converted (3 routing tests + the no-leak test)

- **Presence `.any` → exact slice match.** `value_type_rules.iter().any(|r| is_lookup_of(r, "int"))` → `matches!(param.value_type_rules, [r] if is_lookup_of(r, "int"))` — now asserts *exactly* `[Lookup(int)]`, not just "contains one."
- **`len()==N` + `[i]` element checks → one slice pattern.** The single-ref test's `assert_eq!(len, 1)` + `matches!([0], BorrowRef)` + a silently-skipping `if let` all collapsed into `match param.type_outer_ref_rules { [IRulexSR::BorrowRef(br)] => {…} other => panic! }`. Same for the nested case → `[BorrowRef(inner_br), BorrowRef(outer_br)]`.
- **The silent `if let`** (which skipped its rune asserts if the shape didn't match) is gone — the rune-equality checks now live inside the matched arm, with an `other => panic!` catch-all.
- **Absence `.any` → positive exact pin.** `!foo.rules.iter().any(is_lookup_of "int")` became `matches!(foo.rules, [r] if is_lookup_of(r, "void"))` — pinning `FunctionS.rules` as exactly `[Lookup(void)]` inherently proves the param's int/bool Lookups didn't leak in, without a negative `.any`.
- Kept the `assert_eq!`/`assert_ne!` on **runes** — equality isn't match-expressible, which is exactly what asserts are for.

Removed the now-resolved `// VADD:` (`.any`) and `// V: need arcana` markers along with the code they flagged.

## Verified
- `cargo test --lib` — **502 passed / 0 failed / 1 ignored**, zero warnings. The `foo.rules == [Lookup(void)]` and `value_type_rules == [Lookup(int)]` pins all held.
- No `.any`, `.len()`, `.is_empty()`, or VADD markers remain in those tests; `is_lookup_of` still used in the match guards.

Everything's uncommitted on `experimental-2`, pending your `fire commit`.

---

## 👤 User

is there anything in valec-reviewer.md saying no if guards in match statements?

---

## 🤖 Claude

No — there's no rule against `if` guards in match statements. The two related rules are about different things:

- **"No `|` in a test's match pattern"** (185) — bans or-patterns (accepting several shapes), not guards.
- **"Pin the exact result; don't `if let` a shape check"** (286) — bans using an `if let … panic!` *as the whole assertion*, not guards inside a match.

In fact the doc **endorses** guards — the "Assert only what a match can't express" AFTER example (line 282) uses one:

```rust
collect_only_snode!(NodeRefS::Parameter(param),
  NodeRefS::LookupRule(l) if l.name.as_str() == "int" => ());
```

So the `matches!(param.value_type_rules, [r] if is_lookup_of(r, "int"))` form I used in the conversion is consistent with the doc. If you'd *like* a rule discouraging guards (e.g. preferring the predicate be pushed into the pattern where possible), that'd be a new rule — say the word and I'll draft it, but nothing currently says it.

---

## 👤 User

that example is wrong, because we dont use as_str. look at the real LookupRule please

---

## 🤖 Claude

Let me look at the real `NodeRefS::LookupRule` and `IImpreciseNameS` structure:

---

## 🤖 Claude

Let me see how a real `collect_only_snode!` matches a `LookupRule` name in the tests:

---

## 🤖 Claude

You're right — `l.name` is `IImpreciseNameS`, which has no `.as_str()`. The real form (from the actual test at `post_parser_tests.rs:277`) pins the name in the **pattern**: `IImpreciseNameS::CodeName(CodeNameS { name: StrI("int"), .. })`. Let me fix the doc example to the real structure:

---

## 🤖 Claude

The field names are stale too (`named_type_rules`/`outer_shape_rules` were renamed). Let me fix the whole example — current field names + the real `LookupSR`/`IImpreciseNameS`/`CodeNameS`/`StrI` structure, pinning the name in the pattern (no `.as_str()`):

---

## 🤖 Claude

Fixed. Three things were wrong in that example, now all corrected:

1. **`.as_str()` on the name** — `l.name` is `IImpreciseNameS`, not a string. The real form pins the name in the pattern: `IImpreciseNameS::CodeName(CodeNameS { name: StrI("int"), .. })`, exactly as the actual `LookupRule` test does (`post_parser_tests.rs:277`).
2. **Stale field names** — `named_type_rules`/`outer_shape_rules` are the old names; updated to `value_type_rules`/`type_outer_ref_rules`.
3. **`collect_only_snode!(NodeRefS::Parameter(param), …)` wouldn't actually work** — `visit_parameter` only visits `full_type_rune`, not the rule slices, so a collect starting from a `Parameter` never reaches its `LookupRule`s (it'd find zero and the `_only_` assert would fail). So I switched the AFTER to the slice-match form — which is real, works, and is exactly what I used in the converted tests.

The prose ("may live in a collect_ macro") still stands: the collect form genuinely works for node types whose traversal *does* descend into rules — e.g. `NodeRefS::Impl`, per that same real test. It just doesn't for `Parameter` today.

One decision for you: if you'd rather the doc example (and param-rule tests) use the `collect_only_snode!` form, I'd make `visit_parameter` descend into `value_type_rules`/`type_outer_ref_rules` — but that adds those rule nodes to every traversal from a param, which could shift counts in other collect-based tests, so I kept it minimal. Want me to make the parameter traverse descend, or leave the slice-match form as the house pattern for param rules?

Doc-only change; nothing to build.

---

## 👤 User

i think we need to be stricter/stronger on this: i dont like assert!s and i dont like matches!s. and for example, look at function_with_param_destructure, i dont like how it's using expect_1. can you please rewrite it to be just one match on the function AST?

---

## 🤖 Claude

Let me find the `function_with_param_destructure` test:

---

## 🤖 Claude

The user wants `expect_1` and the `.unwrap()`s folded into a single match on the function AST. Let me find the exact type of `function.header.params`:

---

## 🤖 Claude

`function.header.params` is `Option<ParamsP>`, `ParamsP { params: &[ParameterP] }`. Let me check the test file's imports before rewriting:

---

## 🤖 Claude

`use crate::parsing::ast::*` covers `ParamsP`/`ParameterP`. Now the rewrite — one match on the function AST, folding `expect_1` (→ single-element slice) and both `.unwrap()`s (→ `Some(...)`) into the pattern:

---

## 🤖 Claude

Let me verify it compiles and passes:

---

## 🤖 Claude

The test rewrite passes. Now let me strengthen the rule in `valec-reviewer.md` per your direction — the "Assert only what a match can't express" rule should demand one full match, no `assert!`/`matches!`/`expect_1` helpers:

---

## 🤖 Claude

Now let me confirm the full suite is still green after the test rewrite:

---

## 🤖 Claude

Done — and I need to flag an important consequence, because this new standard reverses what I did two turns ago.

## Applied

- **`function_with_param_destructure` → one match** on `function.header.params`. `expect_1` became a single-element slice pattern `[ParameterP { … }]`, and both `.unwrap()`s folded into `Some(…)` patterns. No helpers, no `assert!`, no `matches!` — just the pattern and an `other => panic!("…{:?}", other)`.
- **Rule strengthened** in `valec-reviewer.md`: renamed to **"Pin a shape with one full match, not asserts"** — "one `match { expected => {}, other => panic! }` over the whole value; fold `expect_1`/`.unwrap()` in; never `.any` or `assert!(matches!)`; asserts only for numbers and equality." Its BEFORE/AFTER is the `expect_1` → folded-match transformation.

Suite green: 502/0/1.

## Surface — this standard now conflicts with recent work

The stronger rule means `assert!(matches!(…))` is out, but I just **added** a batch of it:

1. **The `.any` conversions from two turns ago** (`test_param_no_outer_wrap_routing`, `_single_ref_`, `_nested_ref_`, `test_function_rules_no_longer_…`) — I converted `.any` → `assert!(matches!(…))`. Under the new rule those should be full matches (and pin the Lookup name in the pattern instead of the `is_lookup_of` guard).
2. **My new param tests** (`test_named_param_keeps_its_name`, `test_bare_param_keeps_name…`, `test_named_destructure…`, `test_extern_bare_param_ok`) use `assert!(matches!(param.name, …))`.
3. **The sibling parser destructure tests** you gestured at — `function_with_nested_param_destructure`, `_typed_sub_atom_`, `_ignore_in_` — still use `expect_1` + inner match, exactly like the one I just fixed.
4. Beyond that, `expect_1` / `assert!(matches!)` are used across **many** existing tests codebase-wide.

How far do you want the sweep? My suggestion: do (1)+(2)+(3) now — they're this refactor's own tests plus the direct siblings you pointed at — and leave the codebase-wide sweep (4) as a separate pass. Or narrower/wider if you prefer.

---

## 👤 User

sweep just the tests we've added or changed in our uncommitted work please

---

## 👤 User

sweep just the tests we've added or changed in our uncommitted work please.
but before that, i remember seeing an if-guard in a match that called a helper, can you find that for me please

---

## 🤖 Claude

Found it. The if-guard calling a helper is **`[r] if is_lookup_of(r, "…")`** — 4 sites, all in the routing tests I converted two turns ago:

- `post_parser_tests.rs:1392` — `matches!(param.value_type_rules, [r] if is_lookup_of(r, "int"))`
- `:1398` — `matches!(foo.rules, [r] if is_lookup_of(r, "void"))`
- `:1417` — `matches!(param.value_type_rules, [r] if is_lookup_of(r, "int"))`
- `:1476` — `matches!(foo.rules, [r] if is_lookup_of(r, "void"))`

The sweep eliminates these — pinning the Lookup name *in the pattern* (`IImpreciseNameS::CodeName(CodeNameS { name: StrI("int"), .. })`) drops both the `assert!(matches!)` and the `is_lookup_of` guard. Now let me sweep. Let me re-read the current routing tests to convert precisely:

---

## 🤖 Claude

Now the sweep. Converting each routing test to one full match (folding `expect_1`, pinning the Lookup name in-pattern, equality asserts inside the arm). First, `test_param_no_outer_wrap_routing`:

---

## 🤖 Claude

Routing tests done. Now the param tests. Let me read them:

---

## 🤖 Claude

Now the bare-param and named-destructure tests. Let me read them:

---

## 🤖 Claude

`test_bare_param` — fold the name `assert!(matches!)` into a match on `foo.params` (the Void match already conforms):

---

## 🤖 Claude

Let me read the rest of the named-destructure test to convert it into one match:

---

## 🤖 Claude

Now convert the named-destructure test — one match on `foo.params` for the name, one match folding the whole `let [a,b] = load(p)` head:

---

## 🤖 Claude

Now `test_extern_bare_param_ok` and the two lambda-param matches. Let me find them:

---

## 🤖 Claude

Now the two lambda-param matches — fold `expect_2` into the match:

---

## 🤖 Claude

Now let me compile the tests to catch import/pattern issues:

---

## 🤖 Claude

Compiles clean. Now run the tests to confirm the folded patterns match:

---

## 🤖 Claude

All 502 pass. Now clean up the now-unused `is_lookup_of` helper (all 4 call sites are gone):

---

## 🤖 Claude

Only the definition remains — it's dead. Let me read and remove it:

---

## 🤖 Claude

`BorrowRefSR` and `IImpreciseNameValS` are now only in the `use` line (unused); `IRulexSR` and the pattern types are still used. Let me remove the dead helper and trim the unused imports:

---

## 🤖 Claude

All green — 502 passed, zero warnings. Let me confirm the sweep is complete across the tests I touched:

---

## 🤖 Claude

The remaining `assert!(matches!)`/`.any` are all in tests I didn't touch (generic-param tests at 209-210/261-262, the error test at 1698) — correctly out of scope. `is_lookup_of` fully removed. Let me confirm no `expect_1`/`expect_2` linger in my swept tests:

---

## 🤖 Claude

The sweep is complete. All remaining `expect_1`/`expect_2`/`assert!(matches!)`/`.any` are in tests I didn't touch (generic-param tests, the sibling destructure tests, the error test) — correctly out of scope. My changed/added tests are all clean.

## Swept to one-match form (this turn)

All in `post_parser_tests.rs` unless noted, each now a single `match … { expected => {}, other => panic!("…{:?}", other) }` with `expect_1`/`expect_2`/`.unwrap()` folded into the pattern and equality checks as `assert_eq!`/`assert_ne!` inside the matched arm:

- **4 routing tests** (`test_param_no_outer_wrap_routing`, `_single_ref_`, `_nested_ref_`, `test_function_rules_no_longer_…`) — the `assert!(matches!(…, [r] if is_lookup_of(r, "int")))` **if-guards you flagged are gone**; the Lookup name is now pinned in the pattern (`IImpreciseNameS::CodeName(CodeNameS { name: StrI("int"), .. })`). `expect_1` folded into `[ParameterS { … }]`.
- **`test_named_param_keeps_its_name`**, **`test_bare_param_keeps_name…`**, **`test_extern_bare_param_ok`** — `expect_1` + `assert!(matches!(param.name, …))` → one match on `foo.params`.
- **`test_named_destructure_param_keeps_name…`** — the `expect_1` + `.unwrap()`s + `assert!(matches!)` + `is_none()` all folded into one match pinning the whole `let [a, b] = load(p)` head.
- **The two lambda-param matches** — `expect_2` folded into `[_, ParameterS { … }]`, and the bare `_ => panic!()` upgraded to `other => panic!("…{:?}", other)` so failures show the actual value.
- **`function_with_param_destructure`** (parser test) — the one-match rewrite from before.
- **Removed the now-dead `is_lookup_of` helper** and its two unused imports.

## Verified
- `cargo test --lib` → **502 passed / 0 failed / 1 ignored**, zero warnings.
- No `assert!(matches!)`, `.any`, `is_lookup_of`, or `expect_1`/`expect_2` remain in any test I added or changed.

**Neighbors I left alone** (out of your scope — I didn't change them): the sibling parser destructure tests `function_with_nested_param_destructure`, `_typed_sub_atom_`, `_ignore_in_` still use `expect_1` + inner match, and the error test at `:1698` uses `assert!(matches!(err, …))`. Same pattern, easy to fold if you want them in a follow-up.

All uncommitted on `experimental-2`, pending your `fire commit`.

---

## 👤 User

can you add to valec-reviewer.md that we should never have if-guards in tests?

---

## 🤖 Claude

Rule added. Now let me find and fix `function_with_empty_param_destructure`:

---

## 🤖 Claude

Both done. Let me run the full suite to confirm:

---

## 🤖 Claude

Both changes are done and green (502/0/1, zero warnings).

## Added rule
**"Never an `if` guard in a test's match"** in `valec-reviewer.md`, placed with the other test-match rules — "Pin a literal in the pattern, don't test it in a guard. For a comparison between two bound values, `assert_eq!` in the arm body." BEFORE/AFTER shows `IVarNameS::CodeVarName(s) if s.as_str() == "a"` → `IVarNameS::CodeVarName(StrI("a"))`.

## Fixed `function_with_empty_param_destructure`
Folded `expect_1` + `.unwrap()` into one match on `function.header.params`, same as `function_with_param_destructure`:

```rust
match &function.header.params {
    Some(ParamsP { params: [
        ParameterP { pattern: Some(PatternPP { destructure: Some(DestructureP { patterns: [], .. }), .. }), .. },
    ], .. }) => {}
    other => panic!("expected one `T[]` param, got {:?}", other),
}
```
(Also corrected the panic message from the old destructure-first `[] T` to `T[]`.)

Note: this is one of the sibling parser destructure tests I'd flagged as out-of-scope earlier — the other two (`function_with_nested_param_destructure`, `function_with_typed_sub_atom_param_destructure`) and the `function_with_ignore_in_param_destructure` test still use the `expect_1` + inner-match form (and the last one uses an `if s.as_str()` guard, which the new rule now forbids). Want me to fix those too while we're here?

---

## 👤 User

<command-message>prose-reviewer</command-message>
<command-name>/prose-reviewer</command-name>
<command-args>on translate_signature_templex</command-args>

---

## 🤖 Claude

I'll review the prose/comments on `translate_signature_templex` against the prose-reviewer rules. Let me read it in full.

---

## 🤖 Claude

Reviewing the comments on `translate_signature_templex` against the rules, I see three issues:

1. **Abstract-benefit violation** (513-517): "This lets a caller mirror the wrapping or leave the value type opaque while solving" — this is the exact abstract hand-wave you corrected in the `@PFVSZ` arcana. The concrete "why" already lives in `@PFVSZ`, so this sentence is both abstract *and* redundant.
2. **Not TLDR→example→details / not a list**: the two builders are equal sub-parts crammed into a semicolon'd sentence; the `&&Ship` example is buried in the body comment.
3. **The `_` match arm has no case comment** — it's the non-obvious one (everything-that-isn't-a-ref = the value root).

Let me fix the doc comment first — lead with takeaway + example, list the two builders, drop the abstract sentence (the why is in `@PFVSZ`):

---

## 🤖 Claude

Now trim the body comment — the `&&Ship` example and split explanation now live in the doc comment, so this should focus on the recursion, one idea per sentence:

---

## 🤖 Claude

Now add the case comment to the non-obvious `_` arm:

---

## 🤖 Claude

Comment-only changes. Let me confirm the build is still clean:

---

## 🤖 Claude

Clean, zero warnings. Prose review of `translate_signature_templex` applied — three fixes:

1. **Dropped the abstract-benefit sentence** ("lets a caller mirror the wrapping or leave the value type opaque while solving"). That's the same hand-wave you rewrote in the `@PFVSZ` arcana; since the doc already says "per @PFVSZ," the concrete "why" is one hop away and doesn't need an abstract restatement here. *(Rule: explain a design by the concrete thing it enables — or defer to where it's already concrete.)*

2. **Restructured the doc comment to TLDR → example → list.** Leads with what it does, then the concrete `&&Ship` example (both borrows are wraps, `Ship` is the value type), then the two builders as a **list** instead of a semicolon'd cram. *(Rules: lead with takeaway then example; use a list for equal sub-parts.)*

3. **Commented the `_` match arm** — `// A non-ref node, e.g. \`Ship\` or \`List<T>\`: the value-type root, with no wrapping.` The four ref arms are self-evident from their variant names; the `_` was the non-obvious one. And I trimmed the body comment to just the recursion mechanism (one idea per sentence), since the example and split explanation now live in the doc comment. *(Rule: comment a non-obvious match arm with what leads into it.)*

No em-dashes were present. Build clean, suite still green at 502/0/1. All uncommitted, pending `fire commit`.

---

## 👤 User

please fix test_destructure_param_desugars_to_let_with_destructure

---

## 🤖 Claude

Let me read the current state of `test_destructure_param_desugars_to_let_with_destructure`:

---

## 🤖 Claude

Fixed. `test_destructure_param_desugars_to_let_with_destructure` is now one match on the body head, folding away the chained `.first().expect()` / `.expect()` / `expect_2` / `.unwrap()` extractors and the two `if s.as_str() == "…"` guards.

```rust
match &code_body.body.block.expr {
    IExpressionSE::Consecutor(ConsecutorSE { exprs: [
        IExpressionSE::Let(LetSE {
            pattern: AtomSP {
                name: None,
                destructure: Some([
                    AtomSP { name: Some(CaptureS { name: IVarNameS::CodeVarName(StrI("a")), .. }), .. },
                    AtomSP { name: Some(CaptureS { name: IVarNameS::CodeVarName(StrI("b")), .. }), .. },
                ]), .. },
            expr: IExpressionSE::LocalLoad(LocalLoadSE { name: IVarNameS::DesugaredParamName(_), .. }),
            .. }),
        ..
    ], .. }) => {}
    other => panic!("expected body head `let [a, b] = load(<param>)`, got {:?}", other),
}
```

It also now pins more than the original did — that the head loads the anonymous param's synthetic `DesugaredParamName` slot, and that the top pattern is nameless — while the `a`/`b` names are pinned in-pattern (`StrI("a")`/`StrI("b")`) rather than via guards.

Suite green: **502 passed / 0 failed / 1 ignored**, zero warnings. Still uncommitted on `experimental-2`.

Remaining same-family neighbors not yet swept (still using `expect_1` + inner match, and `_typed_sub_atom_`/`_ignore_in_`/`_nested_` using the `if s.as_str()` guard): `function_with_nested_param_destructure`, `function_with_typed_sub_atom_param_destructure`, `function_with_ignore_in_param_destructure`, and the postparse `test_nested_destructure_preserved`. Want those too?

---

## 👤 User

add to valec-reviewer.md that we shouldnt have cast! in tests

---

## 🤖 Claude

I'll add the `cast!` rule right after the "Never an `if` guard" rule, keeping the test-match rules together:

---

## 🤖 Claude

Added the **"No `cast!` in a test; match the variant"** rule to `valec-reviewer.md`, in the doc's house format and grouped with the other test-match rules (after the `if`-guard rule). It captures the same principle as folding `expect_1`/`.unwrap()`: don't extract a variant with `cast!` and then work on the result — match the variant in the pattern so a wrong variant falls through `other => panic!` with the actual value, instead of an opaque cast failure.

Note this now flags the `cast!(&foo.body, IBodyS::CodeBody)` calls in the tests I just swept (the destructure/bare-param tests still `cast!` the body before matching its head). Want me to fold those into their matches too, or leave that for a later pass?

---

## 👤 User

dont nest matches like that

---

## 🤖 Claude

You're right — the AFTER should be one flat match with a deep pattern, not a match nested in a match arm. Let me fix the `cast!` rule's example:

---

## 🤖 Claude

Fixed. The AFTER is now one flat match — the `LetSE.pattern.destructure` path folded into the variant pattern (`IExpressionSE::Let(LetSE { pattern: AtomSP { destructure: Some(_), .. }, .. })`) — instead of a match nested inside a match arm. The prose now says "match the variant in **one flat pattern**" to make the no-nesting point explicit.

Doc-only change. This also settles how I'd fold the `cast!(&foo.body, …)` calls in the body-head tests if you want them swept: one flat match with the full `CodeBody → BodySE → BlockSE → expr` path pinned, not a nested match.

---

## 👤 User

sweep please

---

## 🤖 Claude

The user wants the `cast!` swept in the tests we changed (fold into one flat match). That's 3 tests: `test_bare_param` (1494), `test_destructure_param_desugars` (1517), `test_named_destructure` (1558). The other `cast!`s are in tests I didn't touch — out of scope. Let me read and fold each. First `test_bare_param`:

---

## 🤖 Claude

Now `test_destructure_param_desugars` — fold the `cast!` into the same match. Let me read its current form:

---

## 🤖 Claude

Now `test_named_destructure` — same fold. Let me read its current form:

---

## 🤖 Claude

Now let me compile to check brace-counting and imports (`CodeBodyS`/`BodySE`/`BlockSE`):

---

## 🤖 Claude

Brace-counting is correct — just missing imports. Let me find the current imports and add `CodeBodyS`/`BodySE`/`BlockSE`:

---

## 🤖 Claude

All 502 pass, zero warnings. Let me confirm `cast!` is gone from the tests I changed:

---

## 🤖 Claude

Swept. The `cast!(&foo.body, IBodyS::CodeBody)` in all three tests we changed is gone — each now matches the body variant in one flat pattern (`IBodyS::CodeBody(CodeBodyS { body: BodySE { block: BlockSE { expr: … } } })`), no `cast!` and no nested matches:

- `test_bare_param_keeps_name_and_gets_no_body_let` — the Void head
- `test_destructure_param_desugars_to_let_with_destructure` — the `let [a,b] = load(<param>)` head
- `test_named_destructure_param_keeps_name_and_gets_let` — the `let [a,b] = load(p)` head

Added `CodeBodyS`/`BodySE`/`BlockSE` to the imports.

Verified: **502 passed / 0 failed / 1 ignored**, zero warnings. The remaining `cast!` calls (~25) are all in tests I didn't touch — out of scope per "tests we've added or changed." Everything uncommitted on `experimental-2`, pending your `fire commit`.

---

## 👤 User

by my rules, should we even have expect_1 etc?

---

## 🤖 Claude

Yes — by your rules, they shouldn't be in tests.

The "Pin a shape with one full match" rule already names `expect_1`/`.unwrap()` as things to **fold into the pattern**, and the reasoning is identical to the `cast!` rule: `expect_1(xs)` is just `match xs { [x] => x, other => panic! }`, and `expect_2` is `[a, b]`. The helper pins "exactly one/two" and panics otherwise — which a slice pattern with `other => panic!` does natively, and *better*, because the failure shows the actual slice instead of a generic "expected 1 got N."

So every `expect_N` in a test is redundant with a slice pattern. There's no case where it buys you something the pattern doesn't — even "extract then pass to a helper" folds to `match xs { [x] => helper(x), other => panic! }`.

Two honest caveats on going all the way to **deleting** them:

- They're used widely — a quick count:

---

## 🤖 Claude

124 uses across the tree, helpers defined in `parsing/tests/utils.rs`.

So the honest position:

- **In tests: no, we shouldn't use them** — it's the same conclusion as `cast!`, and the "Pin a shape with one full match" rule already names `expect_1` as something to fold. The helper is strictly redundant with a slice pattern.
- **Deleting the helpers** is a real sweep: 124 call sites. It's mechanical, but some tests that do `let x = expect_1(…)` then a run of independent checks would fold into one deep arm — occasionally more awkward than the flat form, though still what the "one match" philosophy asks for.

My recommendation, and where I'd want your call:

1. **The rule already covers it** — I don't think a new rule is needed, but I can make `expect_N` as explicit as `cast!` got (a dedicated "No `expect_N` in a test; use a slice pattern" entry) so it's not buried inside the "Pin a shape" rule. Cheap, worth doing if you want it prominent.
2. **The 124-site removal + deleting the helpers** is the big question. Options: (a) keep sweeping only tests we touch, as we've been doing, and let the helpers wither; (b) do a dedicated codebase-wide sweep now and delete the three helpers once unused. I'd lean (a) — the same incremental scope we've held all along — unless you want the mass edit.

Want me to add the explicit `expect_N` rule, and which sweep scope — incremental (touched tests only) or the full 124-site removal?

---

## 👤 User

nah, lets not do that sweep yet. i am however open to the idea of doing a `fn expect_code_body_expr` that will do this part for us:

match &foo.body {
    IBodyS::CodeBody(CodeBodyS { body: BodySE { 

wdyt?

---

## 🤖 Claude

I like it — and I don't think it contradicts the `cast!`/`expect_N` rules; it's on the right side of the line those rules draw.

Here's the distinction. The `cast!` rule is about extractions that pin **the shape under test**. In these body tests, `IBodyS::CodeBody → CodeBodyS → BodySE → BlockSE → expr` is **not** what the test is asserting — it's the same for every normal function. The thing under test is the *head expr*. So spelling the wrapper out in ~25 tests is noise, not signal. Naming that ubiquitous navigation is the same kind of abstraction as `find_func_named` / `lookup_function`, which we use freely.

Two things keep it honest:
- **It preserves the diagnostic that `cast!` throws away.** `cast!` fails opaquely; a helper with `other => panic!("expected a code body, got {:?}", body)` reports the actual body. So it's strictly better than the `cast!` it replaces.
- **The real assertion stays a match.** The helper returns the expr; the test still does `match expect_code_body_expr(…) { ExpectedHead => {} other => panic! }`. It removes the wrapper, not the assertion.

The one caveat: **don't use it where the body variant *is* the assertion.** `test_extern_bare_param_ok` asserts `IBodyS::ExternBody` — that stays a direct match on `&foo.body`. The helper is only for "I want the head, and a code body is assumed."

Concretely I'd write (in `tests/utils.rs`, next to `expect_1`), itself a match, not a `cast!`:

```rust
// The head expression of a normal (code) function body. Panics with the actual body if it isn't a
// code body, since a wrong body variant (extern/abstract) means the test's setup is off.
pub fn expect_code_body_expr<'s>(body: &'s IBodyS<'s>) -> &'s IExpressionSE<'s> {
  match body {
    IBodyS::CodeBody(code_body) => code_body.body.block.expr,
    other => panic!("expected a code body, got {:?}", other),
  }
}
```

Then our three touched tests collapse the wrapper back out:

```rust
match expect_code_body_expr(&foo.body) {
  IExpressionSE::Void(_) => {}
  other => panic!("expected an untouched Void body head, got {:?}", other),
}
```

Want me to add it and apply it to the three tests we changed (leaving the ~22 untouched ones for later, per our scope)?

---

## 👤 User

please do

---

## 🤖 Claude

I'll add the helper and apply it to the three tests. First, let me read the `expect_1` region in `tests/utils.rs` and its imports:

---

## 🤖 Claude

`IBodyS`/`CodeBodyS`/`IExpressionSE` are postparsing types — putting them in `parsing/tests/utils.rs` would make the parsing layer's test utils import postparsing (a layering smell). Let me check what that file imports:

---

## 🤖 Claude

`parsing/tests/utils.rs` imports no postparsing types, so I'll put the helper in `post_parser_tests.rs` (its only consumer) to keep the layering clean — a parsing test-util importing postparsing types would be backwards. Let me find a spot after the imports:

---

## 🤖 Claude

Now apply it to the three tests. `test_bare_param`:

---

## 🤖 Claude

`test_destructure_param_desugars` — unwrap the `CodeBody` wrapper, match the expr directly:

---

## 🤖 Claude

Let me finish the third test, then move the helper to a central location:

---

## 🤖 Claude

Now move the helper to a central postparsing test-utils location. Let me check the test module structure:

---

## 🤖 Claude

No shared postparsing test-utils file exists. Let me create `postparsing/test/utils.rs` as the central home:

---

## 🤖 Claude

Now remove the local helper from `post_parser_tests.rs` and import it from the central module:

---

## 🤖 Claude

The tests no longer spell out `CodeBodyS`/`BodySE`/`BlockSE` (the helper handles them), so those imports I added are now unused. Let me revert them:

---

## 🤖 Claude

All green, 502/0/1, zero warnings. Done.

## What landed

- **New helper `expect_code_body_expr`** in a new central module `postparsing/test/utils.rs` (added `pub mod utils;` to `postparsing/test/mod.rs`). It's itself a match, not a `cast!`, and panics with the actual body variant if it isn't a code body:
  ```rust
  pub fn expect_code_body_expr<'s>(body: &'s IBodyS<'s>) -> &'s IExpressionSE<'s> {
    match body {
      IBodyS::CodeBody(code_body) => code_body.body.block.expr,
      other => panic!("expected a code body, got {:?}", other),
    }
  }
  ```
  Placed there rather than `parsing/tests/utils.rs` — that file imports no postparsing types, and a parsing test-util reaching into postparsing would invert the layering.

- **Applied to the three tests we changed** — `test_bare_param…`, `test_destructure_param_desugars…`, `test_named_destructure…` now open with `match expect_code_body_expr(&foo.body) { … }`, dropping the `IBodyS::CodeBody(CodeBodyS { body: BodySE { block: BlockSE { … } } })` wrapper. The real assertion (the head expr shape) stays a match; only the ubiquitous navigation is named.

- **Reverted** the now-unused `CodeBodyS`/`BodySE`/`BlockSE` imports.

Now `expect_code_body_expr` is available for the ~22 other body tests whenever you want to sweep them. Everything uncommitted on `experimental-2`, pending your `fire commit`.

---

## 👤 User

please fix test_nested_destructure_preserved and all the tests below it

---

## 🤖 Claude

Let me read `test_nested_destructure_preserved` and everything below it to the end of the file:

---

## 🤖 Claude

Now I'll fix each. `test_nested_destructure_preserved` — fold the `cast!` + chained matches + `expect_2` + `.unwrap()` + if-guards into one match:

---

## 🤖 Claude

`test_destructure_ignore`:

---

## 🤖 Claude

`test_empty_destructure`:

---

## 🤖 Claude

`test_extern_param_destructure_rejected` — the `assert!(matches!(err, …))` on an error variant:

---

## 🤖 Claude

All green — 502/0/1, zero warnings. Let me confirm the tail of the file is fully clean:

---

## 🤖 Claude

Clean. Fixed `test_nested_destructure_preserved` and all four tests below it, each now one match against the standards we've set:

- **`test_nested_destructure_preserved`** — folded `cast!` + two chained matches + two `expect_2` + `.unwrap()`s + three `if s.as_str()` guards into a single match on `expect_code_body_expr(&foo.body)` pinning `let [a, [b, c]]` (including the nameless nested sub-destructure).
- **`test_destructure_ignore`** — same fold; the ignore slot is pinned as `AtomSP { name: None, .. }` in-pattern (replacing `assert!(ignore.name.is_none())`), and `b` pinned by `StrI("b")` (replacing the `if s.as_str()` guard).
- **`test_empty_destructure`** — `destructure: Some([])` pins the empty destructure in-pattern (replacing `assert_eq!(destructure.len(), 0)`).
- **`test_extern_param_destructure_rejected`** — `assert!(matches!(err, …))` → a match with `other => panic!` that reports the actual error.
- **`test_extern_bare_param_ok`** — already conformed from the earlier sweep.

No `cast!`, `expect_N`, `assert!(matches!)`, or `if`-guards remain from `test_nested_destructure_preserved` to end of file. Suite green at **502/0/1**, zero warnings. Uncommitted on `experimental-2`, pending your `fire commit`.

---

## 👤 User

add a valec-reviewer.md rule that says that tests must have comments, and it must be inside the test as the first line of the test body

---

## 🤖 Claude

Added the **"Every test opens with a comment saying what it protects"** rule to `valec-reviewer.md`, at the head of the test-rules cluster (before "No `|` in a test's match pattern"). The prose: "A test carries a comment stating the invariant it protects, placed as the first line inside the test body, not above `#[test]`." The BEFORE/AFTER shows a comment moving from above `#[test]` to the first body line.

Doc-only change — nothing to build.

One thing worth flagging: several tests we just touched have their explanatory comment **above** `#[test]` (or above the `fn`), not as the first body line — e.g. `test_named_param_keeps_its_name_at_postparse`, `test_bare_param_keeps_name…`, `test_named_destructure…` all have a `//` block sitting above the attribute. By this new rule those should move inside. Want me to sweep the tests we changed to conform (move the comment to the first body line, and add one where a touched test has none)?

---

## 👤 User

sweep them ty

---

## 🤖 Claude

I'll sweep the tests we changed to put a first-body-line comment in each. Let me read the headers of the tests I touched to see current comment placement. First the param/routing cluster:

---

## 🤖 Claude

Let me see the full comment block above `test_named_param`:

---

## 🤖 Claude

This is a sweep across the ~15 tests I touched — each needs its invariant comment as the first body line. Let me work through them. First, `test_named_param` (comment above `#[test]` → move inside):

---

## 🤖 Claude

Now the routing tests. Let me read the current state to move their comments to the first body line:

---

## 🤖 Claude

This is a mechanical sweep — I'll move each test's invariant comment to the first body line. First, remove the routing section-comment (the `use` stays), then move each routing test's comment up. Starting with the section comment + `test_param_no_outer_wrap_routing`:

---

## 🤖 Claude

Now remove the now-redundant mid-body comment in `test_param_no_outer_wrap_routing`:

---

## 🤖 Claude

`test_param_single_ref_wrap_routing` — add first-line comment, remove mid-body one:

---

## 🤖 Claude

`test_param_nested_ref_wrap_routing`:

---

## 🤖 Claude

`test_function_rules_no_longer_contains_param_rules`:

---

## 🤖 Claude

Now the destructure tests. Let me read the current state from `test_bare_param` onward:

---

## 🤖 Claude

`test_bare_param` — move the comment above `#[test]` inside as the first body line:

---

## 🤖 Claude

`test_destructure_param_desugars` — add first-line comment, remove the mid-body one:

---

## 🤖 Claude

`test_named_destructure` — move the comment above `#[test]` inside as first body line:

---

## 🤖 Claude

Remove the now-redundant mid-body comment in `test_named_destructure`:

---

## 🤖 Claude

Now the last group. Let me read from `test_nested_destructure_preserved` to the end:

---

## 🤖 Claude

Now the final five. `test_nested_destructure_preserved`:

---

## 🤖 Claude

`test_destructure_ignore`:

---

## 🤖 Claude

`test_empty_destructure`:

---

## 🤖 Claude

`test_extern_param_destructure_rejected` (comment above → inside):

---

## 🤖 Claude

`test_extern_bare_param_ok` (mid comment → first line):

---

## 🤖 Claude

Now the lambda test (I changed its two param matches) and the two `function_tests.rs` tests. Let me find the lambda test's name and header:

---

## 🤖 Claude

Now the two `function_tests.rs` tests. Let me check their headers:

---

## 🤖 Claude

That's all the tests I touched. Let me build and run to confirm nothing broke:

---

## 🤖 Claude

All green — 502/0/1, zero warnings. Sweep complete.

Every test we touched now opens with an invariant comment as the first line inside the body:

- **Comments that sat above `#[test]` moved inside** — `test_named_param_keeps_its_name`, `test_bare_param_keeps_name`, `test_named_destructure`, `test_extern_param_destructure_rejected` (and the routing section-comment folded into `test_param_no_outer_wrap_routing`, `use` kept).
- **Mid-body comments hoisted to the first line** — the four routing tests, `test_destructure_param_desugars`, `test_nested_destructure_preserved`, `test_destructure_ignore`, `test_empty_destructure`, `test_extern_bare_param_ok` (their now-redundant mid-body copies removed).
- **First-line comments added where a touched test had none** — `function_with_magic_lambda_and_regular_lambda`, and the two parser tests `function_with_param_destructure` / `function_with_empty_param_destructure`.

Each comment states the invariant the test protects, per the new rule. Scope held to the tests we added or changed — the untouched tests still have their comments wherever they were.

Everything uncommitted on `experimental-2`, pending your `fire commit`.

---

## 👤 User

remember when we were debating on whether or not it was possible to have an outer recursive function call an inner recursive function, and you resisted because of the synthetic param thing?

---

## 🤖 Claude

Honestly — I recall the two pieces you might be stitching together, but not a single debate that matches that exact framing, and I don't want to confabulate a merged memory.

What I do clearly recall:

1. **The outer-recursive-calls-inner-recursive structure** is real and we have it: `translate_signature_templex` recurses on itself peeling ref layers, and at the first non-ref node delegates to `translate_templex` / `translate_type_into_rune` (which recurse in their own right). That was discussed at length — originally in the session whose log I read at the start ("wouldn't `translate_signature_templex` call itself recursively until it hits a non-ref, then just call `translate_templex`?"). I didn't resist *that*; the pushback there was narrower — that the recursion sequences the layers but doesn't by itself dissolve `emit_ref_layer`.

2. **The synthetic-param resistance** is also real, but it was in a *different* discussion — the parameter-desugar refactor. I raised a "synthesized params quirk" (that `&self`/closure/magic have no parser pattern to rebuild from), and you pushed back with "there is no pattern, so what's the problem?" — and I conceded I'd invented the difficulty.

But I don't have a memory of those being the *same* conversation — i.e. of me resisting the outer→inner recursion *because of* synthesized params. If they were one debate, I've lost the thread connecting them.

So rather than nod along: which one are you pointing at — the `translate_signature_templex` recursion, or the synthesized-param objection? Or were they genuinely tied together in a way I'm not recalling? If you tell me the context (or roughly when), I can go back and reconstruct it accurately instead of guessing.

---

## 👤 User

temporary commit to this branch please

---

## 🤖 Claude

I can't act on that yet — the project rule is explicit: I may only commit when you say the literal phrase **"fire commit"** or **"fire commit temporary"**, and "no other phrasing authorizes a commit." "temporary commit to this branch please" is unambiguous in intent, but it's not the literal trigger, so I'm holding rather than commit off a paraphrase.

If you want the temporary-checkpoint flow (commit everything on `experimental-2` as a `TEMP CHECKPOINT:`, sync Luz/Guardian, rebase onto integration but stop short of fast-forwarding), just say **fire commit temporary** and I'll run it.

---

## 👤 User

<command-message>fire-commit</command-message>
<command-name>/fire-commit</command-name>
<command-args>temporary with experimental</command-args>

---

## 🤖 Claude

The literal trigger is given: **`fire commit temporary with experimental`**. Running the checkpoint sequence — Luz/Guardian sync fully, Vale gets a `TEMP CHECKPOINT:` commit rebased onto `experimental`, no fast-forward. Starting with pre-flight (step 0).

---

## 🤖 Claude

Luz curate queue is empty (good). Vale is my session's work. But **Luz and Guardian have dirty state that isn't from this session** (`ValidateReadonlyBash-VRBX` shield + lib.rs in Luz; an `opencode` submodule bump in Guardian). Let me run the remaining sweeps and get a read on those external changes before touching anything.

---

## 🤖 Claude

Sweeps are clean: no Guardian temp-disables, no real `DO NOT SUBMIT` (the Luz hits are just the fire-commit skill doc quoting the sweep command), no absolute paths, no broken symlinks. Vale suite is green (502/0/1 from the last run — the full runnable lib suite, since typing/tests/solver are gated). Let me get the Vale test-delta for the report:

---

## 🤖 Claude

Pre-flight summary for the checkpoint:

**Clean:** no Guardian temp-disables, no real `DO NOT SUBMIT` (the Luz hits are just the fire-commit skill doc quoting the sweep), no absolute paths, no broken symlinks.

**Vale test-delta** (informational for a temporary checkpoint): 2 renames (`test_param_gets_desugared_name…`→`test_named_param_keeps_its_name…`, `test_bare_param_desugars…`→`test_bare_param_keeps_name…`), 1 net-new test (`test_named_destructure_param_keeps_name_and_gets_let`), and many modified (assertions rewritten to the one-match style, comments moved to first body line). No deletions. All part of the param-desugar refactor + the test-style sweep.

**Vale suite:** green — 502 passed / 0 failed / 1 ignored (the full runnable lib suite; typing/tests/solver are gated).

**One thing I need your call on before I commit anything:** Luz and Guardian have dirty state that is **not** from this session — I never touched either:

- **Luz:** `ValidateReadonlyBash-VRBX.md` + its `src/lib.rs` (75 insertions) — a shield edit.
- **Guardian:** an `opencode` submodule gitlink bump (pointer only).

The `fire commit temporary` protocol says to commit + **push both to `main`**. But I can't account for these changes, and pushing unfamiliar work to `main` publishes it to every other worktree irreversibly. So I'm not going to blind-commit them.

How do you want to handle it?
- **(a)** Checkpoint Vale only; leave Luz/Guardian dirty for you to handle (they'd show as not-clean in the final tree check, deliberately).
- **(b)** Commit + push Luz/Guardian too — if so, confirm those VRBX/opencode changes are yours/intended and I'll sync them per protocol.

The Vale side (my session's work) is ready to checkpoint either way.

---

## 👤 User

yeah those are intentional, commit those too

---

## 🤖 Claude

The architect confirmed. Proceeding with the full sequence — Luz and Guardian first. Let me check their branch state and read the Luz diff so I can write an accurate message:

---

## 🤖 Claude

Both on `main`. The Luz change adds a "shell comment lines as no-op segments" rule to the VRBX shield (`.md` doc + `lib.rs` implementation). Let me commit and push Luz:

---

## 🤖 Claude

Remote `main` advanced (another worktree pushed). Let me fetch, rebase my commit onto `origin/main`, and re-push:

---

## 🤖 Claude

Luz pushed and clean. Now Guardian's `opencode` submodule change — let me see exactly what it is:

---

## 🤖 Claude

The Guardian change isn't a gitlink bump — the `opencode` submodule points at the **same** commit (`28793953`), it's just `-dirty` (uncommitted changes inside opencode's own working tree). So there's nothing to commit at the Guardian repo level. Let me see what's actually dirty inside opencode:

---

## 🤖 Claude

Correction on Guardian: the only dirty thing is `bun.lock` inside the `opencode` submodule (a Bun dependency lockfile), on a detached HEAD — that's build-artifact churn, not intentional work, and the gitlink commit is unchanged. So there's **nothing to commit at the Guardian level**, and I'm not going to commit a lockfile artifact inside a third-party submodule. My earlier "gitlink bump" description was wrong — apologies. Leaving Guardian's opencode as-is.

Luz is done (pushed to `main`). Now the Vale checkpoint. Let me stage the `FrontendRust/` tree and review before committing:

---

## 🤖 Claude

Staged set is exactly the `FrontendRust/` tree (my param-desugar work + the accumulated onion/postparse-slice changes + the new PFVSZ arcana and `test/utils.rs`), no cruft. Only `fire-commit-config.md` is left unstaged (pre-existing root untracked — correctly excluded). Committing with the mandatory `TEMP CHECKPOINT:` prefix:

---

## 🤖 Claude

Vale checkpoint committed on `experimental-2` (37 files, +1417/-571). Now step 4 — rebase onto `experimental`. Let me check the target's state first:

---

## 🤖 Claude

`experimental` is fully contained in `experimental-2` (16 ahead, 0 behind) — so the rebase onto local `experimental` is a no-op, meaning the checkpoint is already aligned with integration. Let me run it to confirm, then do the clean-tree check (step 5 fast-forward is skipped for temporary):

---

## 🤖 Claude

The checkpoint is complete. Here's the final state.

## Done — `fire commit temporary with experimental`

**Luz** — committed and pushed to `main`. The VRBX shield change (shell comment lines treated as no-op segments) landed; I had to fetch + rebase onto `origin/main` because another worktree pushed concurrently, then pushed clean.

**Vale** — `TEMP CHECKPOINT: parameter desugar refactor + @PFVSZ type split…` committed on `experimental-2` (`e08b31357`, 37 files, +1417/−571). Rebase onto `experimental` was a no-op — `experimental-2` already contains all of local `experimental` (16 ahead, 0 behind), so it's aligned with integration. Fast-forward skipped and the branch not pushed, per the temporary variant.

**Guardian** — nothing committed. The only dirty thing is `bun.lock` churn inside the `opencode` submodule (an artifact on a detached HEAD; gitlink commit unchanged), which isn't intentional work to commit.

## Two deliberate not-clean items (flagged, not touched)
- **Vale:** `fire-commit-config.md` (untracked at the repo root) — pre-existing from before this session, not part of the FrontendRust work, so left unstaged.
- **Guardian:** the `opencode` `bun.lock` artifact churn, left as-is.

## Pre-flight recap
Sweeps clean (no Guardian temp-disables, no real `DO NOT SUBMIT`, no absolute paths, no broken symlinks). Runnable Vale lib suite green at **502/0/1** (typing/tests/solver gated). Test-delta was informational: 2 renames, 1 net-new test, several rewritten to one-match style, no deletions.

The checkpoint is on the branch for you to accumulate onto and squash later. Nothing landed on `experimental` itself.

---

## 👤 User

now, can you look into whether we can excise from the postparser anything that things about a rune type map? i saw we were building lots of rune type maps that shouldnt be in the postparser anymore

---

## 🤖 Claude

This connects to the `// V: lets get rid of rune_to_explicit_type` marker we deferred. Let me investigate what rune-type maps the postparser builds and whether they can be excised. Let me map the surface first.

---

## 🤖 Claude

Two maps: `rune_to_explicit_type` (threaded through 7 files, 71 uses, and **commented out** on a struct at ast.rs:299) and `rune_to_type` stored on a struct at ast.rs:543. Let me read those struct regions:

---

## 🤖 Claude

`ImplS` already has `rune_to_explicit_type` commented out — the excision is partly done. Let me check the `rune_to_type` struct and whether the other citizen/function structs still carry the map:

---

## 🤖 Claude

`FunctionS` has no `rune_to_explicit_type` at all — already removed. So the map is threaded through the postparse but no longer stored on functions or impls. Let me fan out read-only agents to trace this thoroughly across the 7 files while I confirm the remaining structs.

---

## 🤖 Claude

While the agents trace this, let me confirm the remaining citizen structs and read the `default_rune_to_type` builder:

---

## 🤖 Claude

The picture is already fairly clear from the structs:

- **`rune_to_explicit_type`** is gone from every live AST struct — `FunctionS`, `StructS`, `InterfaceS`, `ExportAsS` have no such field, and `ImplS` has it commented out. So the map is still *threaded and accumulated* through 7 postparse files but no longer *stored* anywhere. That's the vestige.
- **`GenericParameterDefaultS.rune_to_type`** is different — a purpose-built single-entry map (`result_rune → its type`), and the comment at `post_parser.rs:498` says the typing pass reads it at default-fire time (`registerRunes(default.runeToType.keys)`). That one looks deliberate, not vestigial.

I've got two agents tracing the full dataflow (every surviving read of the threaded map) and confirming the typing pass derives rune types itself. Let me wait for them before proposing the exact excision.

---

## 🤖 Claude

One agent done. The key nuance it surfaced: the persisted maps are already gone from AST structs (typing's `rune_typing` solver derives rune types from rules itself), but the threaded `rune_to_explicit_type` is still **consumed within postparse** via `ParentCitizen.citizen_rune_to_explicit_type` (function_scout.rs:70) for nested denizen/lambda scoping. So it's not obviously dead — the question is whether that within-postparse use is itself still needed. The other agent is tracing exactly that (every surviving read). Let me wait for it before proposing the excision.

---

## 🤖 Claude

Both agents agree, and the verdict is unambiguous: **yes — `rune_to_explicit_type` is fully dead and can be excised entirely.**

## What it is

A self-referential accumulator threaded through ~7 postparse files (71 uses). Every rune — user-declared or minted implicit — gets a `(IRuneS, ITemplataType)` pair pushed as postparse walks generic params, patterns, templexes, and rules. But:

- **It's stored nowhere live.** `FunctionS`, `StructS`, `InterfaceS`, `ExportAsS` have no such field; the Vecs built in `scout_function`/`scout_impl`/`scout_export_as`/the LetSE path are just **dropped**. `ImplS` has it commented out.
- **Its one apparent "storage" is also dead** — `ParentCitizen.citizen_rune_to_explicit_type` (function_scout.rs:70) has exactly 3 references in the whole tree: the field def + 2 writes, **zero reads**.
- **It's only ever read by its own maintenance** — the dedup/merge loops in `scout_function` clone it into sub-scopes and merge keys back, feeding nothing downstream.

## Why it's redundant now

This is a vestige of the retired design. On `master`, these maps were stored on the AST structs and consumed by the old `higher_typing` pass. That pass is gone; the rune-type solver moved into `typing/rune_typing/`, and it now **derives** rune→type from a denizen's rules plus generic-param seeds — which are still on the AST (`GenericParameterS.rune` + `.tyype`). So the postparse precomputation buys nothing. This is exactly the `// V: lets get rid of rune_to_explicit_type` marker (function_scout.rs:388) we deferred earlier.

No test references it (the test traverser doesn't even walk it), so excision breaks nothing.

## Keep (do NOT touch)

**`GenericParameterDefaultS.rune_to_type`** is a *different* thing — separately built as a single-entry `[(result_rune, its type)]`, and genuinely consumed by the (gated) typing pass, which registers default-only runes from its keys. Leave it and its `default_rune_to_type` builder alone.

## Excision scope

- Drop the `rune_to_explicit_type` param from `translate_pattern`, `translate_rulexes`, `translate_templex`/`translate_type_into_rune`, and the scout functions, plus all ~15 push sites and the dedup/merge loops.
- Delete `ParentCitizen.citizen_rune_to_explicit_type` (field + 2 writes) and the commented-out `ImplS` remnants (ast.rs:299/314/323).
- Verify green (502/0/1) — no functional change, pure dead-code removal.

The one thing that's *not* our concern here: when typing is un-gated it'll seed the solver from generic-param types itself (and there's a commented-out `MaybeCoercingLookup` coercion path to reconcile then) — but that's typing-side and doesn't block removing the postparse accumulator.

Want me to do the excision? It's wide but purely mechanical, and it's the confirmed-safe version of that deferred `// V:` marker.

---

## 👤 User

proceed

---

## 🤖 Claude

Proceeding. Let me get the complete inventory of every `rune_to_explicit_type` site so I can remove them systematically.

---

## 🤖 Claude

This is a wide, interdependent excision. Let me set up tasks and work through it methodically, compiling frequently. Let me start with the leaf helper `translate_pattern` — re-read it precisely:

---

## 🤖 Claude

`translate_maybe_type_into_rune` (line 55) doesn't take the map — `translate_pattern`'s param is used only for the insert (65-68) and the recursion (85). Let me remove all three:

---

## 🤖 Claude

Now `templex_scout.rs` — let me read the function at 660-690:

---

## 🤖 Claude

This is `translate_maybe_type_into_maybe_rune` — remove the param and the insert:

---

## 🤖 Claude

Now `rule_scout.rs` — two functions take the map. Let me read it:

---

## 🤖 Claude

`rule_scout.rs` has both `translate_rulexes` and `translate_rulex`. Let me remove the params, the pushes, and the threaded args:

---

## 🤖 Claude

Now `post_parser.rs:428` — the function taking the map. Let me read it:

---

## 🤖 Claude

`scout_generic_parameter` — but `type_s` is used later (for the default's `rune_to_type` at :508). Let me check how `type_s` flows before editing:

---

## 🤖 Claude

`type_s` flows into `generic_param_type_s` — I'll keep it and just remove the param + push:

---

## 🤖 Claude

Now the two `function_scout.rs` helpers (`create_closure_param`, `create_magic_parameters`). Let me read them:

---

## 🤖 Claude

`create_closure_param` — remove the param and the push:

---

## 🤖 Claude

Now `create_magic_parameters`. Let me read its current state:

---

## 🤖 Claude

Task 8 (leaf helpers) done. Now the local builders. Let me re-inventory the remaining sites:

---

## 🤖 Claude

Now the intricate part — the builders in `scout_function`. Let me read the regions to edit precisely:

---

## 🤖 Claude

Now the three call-arg sites (304/323/338) and the param-loop sites. Let me read those regions:

---

## 🤖 Claude

The line numbers shifted. Both `translate_rulexes` calls share an identical block — let me remove the arg from both (replace_all), then the `scout_generic_parameter` arg:

---

## 🤖 Claude

Now the param-loop sites (the `&self` push, the destructure map+dedup, the ret map+dedup, the untyped push). Let me read that region:

---

## 🤖 Claude

Now the return-type region and the last two args. Let me re-grep function_scout:

---

## 🤖 Claude

Now the last two function_scout args (751, 761). Let me read them:

---

## 🤖 Claude

One arg left at 730 (likely the closure-param call). Let me read it:

---

## 🤖 Claude

function_scout done (except the `ParentCitizen` field). Now `expression_scout.rs` LetSE path. Let me read it:

---

## 🤖 Claude

Now `post_parser.rs` — the citizen scouts (impl/struct/export/interface). Let me re-grep for current positions:

---

## 🤖 Claude

Let me read `scout_impl` (690-750):

---

## 🤖 Claude

`scout_impl` — `ImplS` doesn't store the map, so remove the decl + 2 args:

---

## 🤖 Claude

Now `scout_export_as` (around 916). Let me read it:

---

## 🤖 Claude

`scout_export_as` — `ExportAsS` doesn't store it; remove decl + push:

---

## 🤖 Claude

Now `scout_struct` — the biggest, with header/members/all maps + the citizen write. Let me read it:

---

## 🤖 Claude

`scout_struct` — remove the header/members/all maps, member inserts, and the `citizen_rune_to_explicit_type` write. Working through them:

---

## 🤖 Claude

Now remove the `citizen_rune_to_explicit_type` write in `scout_struct`'s `ParentCitizen`:

---

## 🤖 Claude

Now `scout_interface`. Let me re-grep post_parser for remaining sites:

---

## 🤖 Claude

`scout_interface` — remove decl, both args, and the citizen write:

---

## 🤖 Claude

Task 9 done. Now task 10 — the `ParentCitizen` field def and the `ImplS` commented remnants:

---

## 🤖 Claude

All sites edited. Now let me build to catch any missed sites or newly-unused imports:

---

## 🤖 Claude

No `rune_to_explicit_type` remains, and the build passed. Let me check warnings (newly-unused imports):

---

## 🤖 Claude

Zero warnings, no sites remain. Let me run the full suite to confirm the excision didn't change behavior:

---

## 🤖 Claude

502 passed — behavior-preserving. Now let me clean up imports that became unused (the `allow(unused_imports)` hides them, but I want it clean). Let me check the heavily-edited files:

---

## 🤖 Claude

Several look import-only now. Let me verify precisely which are dead per file:

---

## 🤖 Claude

Let me check `IRuneS` in pattern_scout (was in the removed map type) before trimming imports:

---

## 🤖 Claude

All import-only in pattern_scout. Let me trim the dead imports across the four files:

---

## 🤖 Claude

Let me remove each `HashMap` import with adjacent context for uniqueness:

---

## 🤖 Claude

Now rebuild and run the full suite to confirm the import trims didn't break anything:

---

## 🤖 Claude

All green. Final sanity sweep:

---

## 🤖 Claude

Excision complete. Let me close the task.

---

## 🤖 Claude

Done — `rune_to_explicit_type` is fully excised from the postparser, and the suite is green (**502/0/1, zero warnings**) with no behavior change.

## What was removed

- **The threaded parameter** dropped from every helper that took it: `translate_pattern`, `translate_maybe_type_into_maybe_rune`, `translate_rulexes`, `translate_rulex`, `scout_generic_parameter`, `create_closure_param`, `create_magic_parameters` — plus the argument at every call site.
- **Every producer** — the ~15 `push`/`insert` sites (generic params, params, return runes, struct members, region runes, closure/magic runes).
- **The self-referential maintenance** — the local `Vec`s in `scout_function`/`scout_impl`/`scout_struct`/`scout_export_as`/`scout_interface`/the LetSE path, and the dedup/merge loops (`rune_to_explicit_type_for_destructure`, `_for_ret`) that only existed to keep the map coherent across sub-scopes.
- **`ParentCitizen.citizen_rune_to_explicit_type`** — the field (function_scout) and its two writes (scout_struct, scout_interface). It had zero reads.
- **The commented-out `ImplS` remnants** (ast.rs) and the `// V: lets get rid of rune_to_explicit_type` markers.
- **Now-dead imports** — `HashMap` in four files, `ITemplataType`/`KindTemplataType`/`IRuneS` in `pattern_scout`.

## What was kept

`GenericParameterDefaultS.rune_to_type` — a different, live thing (single-entry `[(result_rune, its type)]`, read by the gated typing pass to register default-only runes). Untouched.

## Why it's safe

This is exactly the dead vestige the two agents traced: the rune-type maps used to be stored on the AST structs and consumed by the retired `higher_typing`; now the `typing/rune_typing/` solver derives rune→type from a denizen's rules plus generic-param seeds (still on the AST). So the postparse precomputation fed nothing. No test referenced it, and the suite is unchanged at 502/0/1.

This closes out the `// V:` marker we deferred earlier. Everything's uncommitted on `experimental-2`, on top of the `TEMP CHECKPOINT` — ready for your next `fire commit temporary` (or squash) when you want it.

---

## 👤 User

can we kill GenericParameterDefaultS.rune_to_type

---

## 🤖 Claude

Good question — the agents flagged this as the "weakest" survivor. Let me verify precisely: what `rune_to_type` holds, how it's built, and exactly what the (gated) typing consumers extract from it. Let me read the builder and the consumers.

---

## 🤖 Claude

The builder makes `rune_to_type` a **single entry**: `(result_rune, parent_generic_param.tyype())`. The key is already the separate `result_rune` field; the value is the owning `GenericParameterS.tyype`. Let me check what the (gated) consumers actually pull out — keys only, or values too:

---

## 🤖 Claude

Yes — it's killable, and it's fully derivable. But it's a slightly different case from `rune_to_explicit_type`, so let me be precise.

## What it actually is

The builder makes `rune_to_type` a **single entry**: `[(result_rune, generic_param_type_s.tyype())]`. Both halves are already available elsewhere:
- **The key** is `result_rune` — which is *already a separate field* on `GenericParameterDefaultS`.
- **The value** is `generic_param_type_s.tyype()` — the `tyype` of the **owning** `GenericParameterS` (the generic param this default belongs to).

## How the (gated) typing consumes it

Two consumption shapes:
- **Keys only** — `function_compiler_solving_layer.rs:447` and `infer_compiler.rs:158` do `.rune_to_type.iter().map(|(k, _)| *k)` to register the default's runes. That's just `result_rune`.
- **Keys + values** — `struct_compiler_generic_args_layer.rs:141/217` builds `defaults_rune_to_type` from `gp.default…flat_map(|d| d.rune_to_type…(*k, *v))` and uses the **value** as a prediction fallback.

So unlike `rune_to_explicit_type` (which had zero real consumers), this one *does* have an intended value-consumer. The saving grace: at every one of those sites the code already iterates `gp.default`, so **`gp` is in scope** — meaning `defaults_rune_to_type` can be rebuilt as `(d.result_rune, gp.tyype.tyype())` with no stored map. The reconstruction is exact (`gp.tyype` *is* the `generic_param_type_s` the builder stored).

## Verdict

Killable. `rune_to_type` carries nothing that isn't recoverable from `result_rune` + the owning generic param's `tyype`.

The one honest difference from the last excision: those ~4 reads are **intended** (gated) consumption, not dead code. So killing the field means:
1. Postparse drops the `default_rune_to_type` builder + the struct field (trivial, keeps the green build green).
2. The gated typing sites reading `d.rune_to_type` become 4 more stale reads for the eventual un-gate — but they join a large existing pile (they already read removed fields like `header_rune_to_type`/`members_rune_to_type`), and the reconstruction recipe (`(d.result_rune, gp.tyype())`) is simple.

My recommendation: kill it, and leave a one-line note at the postparse builder site (or in the `traverse.rs` visitor, which already skips `rune_to_type`) recording that recipe so the un-gate work doesn't have to rediscover it.

Want me to proceed — remove the field + builder, and drop the `visit_generic_parameter_default` handling if needed?

---

## 👤 User

kill it. leave no comment

---

## 🤖 Claude

Killing it. Let me find the exact builder location and the field:

---

## 🤖 Claude

Only one construction site, and traverse doesn't touch it. Let me read the builder region and remove:

---

## 🤖 Claude

Killing it. First the post_parser builder + field:

---

## 🤖 Claude

Killed. `GenericParameterDefaultS.rune_to_type` and its `default_rune_to_type` builder are gone, no comment left. `GenericParameterDefaultS` now carries just `result_rune` + `rules`.

- No `rune_to_type` remains anywhere in postparsing.
- Build clean, **502 passed / 0 failed / 1 ignored**, zero warnings.

The value the field held is fully reconstructable when the typing pass is un-gated: at each of the ~4 gated read sites the code already iterates `gp.default`, so `(d.result_rune, gp.tyype())` reproduces exactly what was stored. The gated typing's `d.rune_to_type` reads join the existing pile of stale reads to be fixed at un-gate time.

Combined with the previous excision, the postparser no longer builds or stores **any** rune-type map. Everything's uncommitted on `experimental-2`, on top of the `TEMP CHECKPOINT`.

---

