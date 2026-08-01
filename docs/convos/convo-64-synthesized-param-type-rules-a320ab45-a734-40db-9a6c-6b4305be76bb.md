<!-- session: a320ab45-a734-40db-9a6c-6b4305be76bb; exported-bytes: 5303823; transcript: /Users/verdagon/.claude/projects/-Volumes-V-Vale4/a320ab45-a734-40db-9a6c-6b4305be76bb.jsonl; opened: convo-12-lookup-paths-and-mechanism-ordering.md -->
# Claude Conversation Log

Session ID: a320ab45-a734-40db-9a6c-6b4305be76bb
Date: 2026-07-31 19:52:13

---

## 👤 User

<command-message>use-mailbox</command-message>
<command-name>/use-mailbox</command-name>
<command-args>and /full-read convo-12-lookup-paths-and-mechanism-ordering.md and the plan doc it references, and any handoff doc it mentions</command-args>

---

## 🤖 Claude

Convo-12 read in full (3,849 lines). Now the plan doc.

---

## 🤖 Claude

Plan doc read in full (1,443 lines). Now the handoff doc it cites — `/Volumes/V/Vale2/vcoord-handoff.md`.

---

## 🤖 Claude

Watcher armed as **Vale4-teal**, resident.

Read in full, in order:

| doc | lines |
|---|---|
| `convo-12-lookup-paths-and-mechanism-ordering.md` | 3,849 |
| `synthesized-declarations-plan.md` (the plan/handoff it references) | 1,443 |
| `README.md` (the index) | 73 |
| `/Volumes/V/Vale2/vcoord-handoff.md` (the handoff it cites, §0.5 and §10.10) | 1,971 |

## What convo-12 was

Two threads. **Build:** panic-vs-decline (`lower_ty`/`lower_sig_ty` return `Result<_, DeclineReason>`), corpus cases 19/21/23/26, nested-module dotted paths, re-exports (already worked), the sixteen-mechanism composition case, then `LookupSR.name` → `parts: &[IImpreciseNameS]` with `lookup_nearest_with_path`, which killed case 25's panic. Corpus 33 → 46.

**Method:** the "fence" retirement — the rule that a property is upheld by the type system first and a source lint last, the @NNGZ lint withdrawn for failing loudly already, and five rounds of you catching the same error at smaller scale (parallel path instead of the existing one). Landed as `875ba1331`.

## Four places our docs are now behind Vale2's

Their handoff has moved past what our plan §7 and §10.10 record.

**1. Send semantics are ruled — §7 says they aren't.** Our §7 says *"what a send does when the rune is already determined is unruled — do not build until ruled."* Their handoff now carries a banner: **a send is `Equals`, and every parameter gets one.** No guard, no eligibility predicate — phase 0's preview ordering is what makes that safe, with a five-row case table. And the root cause is sharper than we recorded: master's `make_solver_state` *took* an `initial_sends` parameter and lowered each send three ways; the onion arc's `Coord*SR` sweep deleted the rule and the parameter with it. A deliberate mid-migration removal awaiting its `Equals`-shaped replacement, not an unfinished port.

They also name our case explicitly: *"Vale4's `opt_with_undroppable_contents` is the same generic-drop gap seen from the interop side."*

**2. "Does `&Ship` mention `Ship`" is closed — §10.10 has it open and load-bearing.** Answer: yes, and a strong ref mentions its payload as well, as an *ordered* union. Our §10.10's shrinkage of Problem B is marked "contingent"; it isn't any more.

**3. Overlapping overloads is superseded by something stricter.** Not "two functions whose parameter shapes could both accept one tuple is an error" — **a name is declared at most once per namespace, whether or not the shapes overlap.** They state the consequence outright: *"this makes Vale a language with no overloading."* `drop.vale`/`arith.vale`/`clone.vale`/`logic.vale` dissolve into per-type files. There's an unresolved gap flagged in their own text — a user cannot write `println(int)` and `println(str)`, since they own neither file, and *"no replacement has been named."*

**4. Defect 11 is confirmed live**, still the `BorrowRef` peel concluding into `result_rune` where it means `inner_rune`, and still called load-bearing. §7 is right about that one.

## Smaller

- Their capability ladder still names **export/extern (12 tests)** as *"Vale4's front line"* — `is_primitive` rename plus `peel_all_references` at both the check and the map lookup. Borrow read-out (case 39) is 6 tests.
- Warnings are **7 real, not 8** — `grep -c "^warning"` counts rustc's own summary line. Our docs quote 8 throughout.
- The §6.6 interface-vocabulary fix we made is recorded on their side as *"now ours"* — done, matching.
- Their `ZLOOK` on `get_drop_function` carries the Harmonious reframe our §9 quotes: the synthesizer is the one caller that never has to infer, so it should write `drop::<T>(…)` itself — with the open condition *"whether this runs after types are resolved."*

Nothing in the tree changed; this was reading only. The uncommitted files are the three doc edits and the untracked convo-12 that were there when we started.

---

## 👤 User

to be clear, vcoord-handoff.md isnt our endeavor, it's Vale2's endeavor (we're Vale4). but synthesized-declarations-plan.md is what we're doing i believe. with that in mind, where are we?

---

## 🤖 Claude

Right — vcoord-handoff is their board; it matters to us only where it gates us. Let me measure rather than quote the plan doc's numbers.

---

## 🤖 Claude

Measured at `875ba1331`: **628 / 166 / 8** interop, **582 / 166 / 8** default. Matches the recorded baseline — nothing drifted.

## Where we are

**The typing pass understands Rust.** A Vale program typechecks against a live `TyCtxt` using real Rust free functions, generic functions at concrete types, types by inference from signatures, methods and associated functions, generic types carrying their arguments (`Holder<i32>` and `Holder<bool>` are distinct Vale kinds), nested modules by dotted path, re-exports, and two crates at once. 46 corpus cases, including one program exercising sixteen mechanisms together. Unrepresentable signatures decline with a carried reason instead of panicking. Core diff is a net deletion.

**Nothing has ever run.** No instantiator, no codegen, no linking. Every assertion is on the typed AST and on which questions the oracle was asked.

## Unblocked and ours — three things

1. **Cross-crate re-exports** (§5.3 step 3). Traversal works, but every case covering it is intra-crate; `std::vec` is `pub use alloc_crate::vec`, a different `module_children` path. `fixtures_two_crates` already exists to host it. Small.
2. **The `RUST_MODULE` check.** `rust.mycrate.Widget` is unambiguous only while no Vale module is named `rust` — reserved by comment, enforced nowhere. One check.
3. **Outbound `GenericArgs` reconstruction** (§9 step 3) — rebuilding `[i64, Global]` from `[Kind(i64)]` via `generics_of` + `mk_args`. Ours and unblocked, but I'd flag it as premature: nothing feeds rustc back yet, so it would land untested against its actual consumer.

That's a thin board, and item 3 is the only substantial one.

## Yours — four, and one is much bigger than its size

**The `TooManyTypesWithNameT` conversion** is the one I'd put first. A bare ambiguous name written in hand-written Vale still panics at `lookup_nearest_with_imprecise_name`. The error variant already exists, is produced by nothing, and its humanizer is a `panic!`. Turning the panic into that error fixes strictly more programs than the whole naming change did, for less code, and `expression_compiler.rs` already returns `CouldntFindTypeT` in the analogous spot. Core, so it's yours — but it's the smallest core change on the board with the largest reach.

The other three: **the side-table shape** for a declined item's reason (three options, I lean a field on `FindFunctionFailure`); **the eagerness ruling** (our half is the per-type method fan-out, the expensive half is the compile-everything loop in `Compiler::evaluate`); and the design question **should `Vec<int>()` construct at all**, or must Vale source say `Vec<int>::new()`.

Deliberately deferred by you, not blocked: the flat-list → tree change behind `lookup_nearest_with_path`'s one `if prefix.is_empty()`.

## Vale2's — two, and one just got its ruling

**Case 41, generic-citizen drop.** Our §7 says the send semantics are unruled and not to build against. **That is now stale** — their handoff carries the ruling: a send is `Equals`, every parameter gets one, no guard, with phase 0's preview ordering making it safe. So the design gate is gone; what remains is their code. Their own PICK-UP names our `opt_with_undroppable_contents` as the same gap seen from the interop side, so it moves when they move.

**Case 39, borrow read-out.** Their `dot_borrow` cluster, 31 tests, waiting on a shape call from you rather than on discovery.

**Tier 2** is scheduled, not stalled: LLVM 16 → ~21 plus their ~3-week relink of `instantiating/`/`simplifying/`, which are stale rather than merely gated.

## Loose ends

The plan doc and README are modified and convo-12 is untracked — unbanked since `875ba1331`. And §7's "do not build until ruled" needs correcting now that the send ruling landed; that's a doc edit I can make whenever you want it.

---

## 👤 User

<command-message>fire-rebase</command-message>
<command-name>/fire-rebase</command-name>
<command-args>with main, and let me know if we're bringing in anything that unblocks us</command-args>

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

<command-message>fire-rebase</command-message>
<command-name>/fire-rebase</command-name>
<command-args>with experimental, and let me know if we're bringing in anything that unblocks us</command-args>

---

## 🤖 Claude

Config: rebase-and-fast-forward, `experimental` family. We're on `experimental-4`, a side-branch, so the target is the **local** `experimental` ref.

---

## 🤖 Claude

One incoming commit — and the message is directly relevant. Stashing first (with a distinctive label; the stash list is shared and has 9 entries from other worktrees).

---

## 🤖 Claude

Rebase clean. Re-verifying on the rebased tip — the config's gate first.

---

## 🤖 Claude

Same structural blocker as the last five runs — 9 errors, all in `src/bin/valec/`, from the onion arc commenting `backend_ffi`/`pass_manager` out of `lib.rs`. Measuring against the ratified substitute.

---

## 🤖 Claude

Clean apart from the two known untracked files. Popping the stash by explicit index.

---

## 🤖 Claude

Clean pop. Now the external repos.

---

## 🤖 Claude

Both clean fast-forwards, nothing ahead. Pulling.

---

## 🤖 Claude

Externals current, all five Guardian submodules pinned. Now the question you actually asked — what came in.

---

## 🤖 Claude

The commit is explicit that it greens nothing — measured, not assumed. But it improved the diagnostic on exactly the failure our §7 tracks. Let me read what our blocker now says.

---

## 🤖 Claude

`fire rebase with experimental` is complete.

## What happened

`experimental-4` is now at **`e656fea7e`**, one commit in, rebase clean with no conflicts. Divergence against `experimental` is **0/0**. Our stash popped cleanly and dropped; the remaining stash entries are other worktrees' and untouched.

| | before | after |
|---|---|---|
| interop | 628 / 166 / 8 | **628 / 166 / 8** |
| default | 582 / 166 / 8 | **582 / 166 / 8** |
| `cargo build --lib` | exit 0 | exit 0, 7 warnings |

The config's gate still can't run — `cargo build` exits 101 with 9 errors, all in `src/bin/valec/` against the `backend_ffi`/`pass_manager` modules the onion arc commented out of `lib.rs`. Sixth time, same structural reason, unchanged by this rebase. Measured against the ratified `--lib` substitute above.

Worth noting the warning count: **7, not 8**. Vale2's handoff is right that `grep -c "^warning"` counts rustc's own summary line, so our docs have been quoting 8 for a while.

## Does it unblock us — no, and the commit says so itself

The incoming commit is **defect 11**: the `BorrowRef` peel arm concluded into `result_rune` where it meant `inner_rune`. That's one of the two gates our §7 names on case 41.

But its own message is explicit and measured: *"The peel fix greens nothing... a panic in that arm fires zero times across the whole suite, and the totals are byte-identical."* The reason is the one our §7 already gives — nothing concludes a parameter's `full_type_rune` ahead of its `value_type_rune`, because argument types never enter the call-site solve. The arm goes live *the moment sends are consumed*, not before.

So our board is unchanged. What did improve is diagnosis. The second half of the commit unwraps `IResolvingError` in the abstract-body macro, and our blocker now reads:

```
no override found for "drop", 3 candidate(s) rejected:
  [SpecificParamDoesntMatchExactly (param 0),
   ResolveFailure (unsolved: [CodeRune("T"), ...]),
   ResolveFailure (unsolved: [CodeRune("T"), ...])]
```

`T` named as unsolved, rather than a bare "resolving failed." That's confirmation of the mechanism rather than a change to it — but it's the first time the failure states the cause in its own words.

The commit also flags two neighbours waiting on the same event: the `_ => unimplemented!()` beside the fixed arm fires when a seeded full type isn't a `BorrowRef` (an owned value sent at a `&T` parameter — what phase 0's auto-ref exists to adjust), and the wrap direction still hardcodes `RegionT::Default` instead of reading `r.region`.

## Externals

Both clean fast-forwards, nothing ahead, now current:

- **Luz** pulled 3 commits — including `update-handoff.md` being shared upstream, which is the skill we used on the plan doc last session.
- **Guardian** pulled 5 commits — setup/hooks moved out of shell into tested binary subcommands, plus an emergency brake. All five submodules at pinned refs.

⚠️ **Guardian and Luz both changed on disk — restart this directory's Guardian to pick it up.**

The doc pile is unchanged and still unbanked: the plan doc and README modified, convo-12 untracked, plus `.github/ci-guide.md` which remains someone else's.

---

## 👤 User

whats next for us?

---

## 🤖 Claude

Short version: **the biggest item on the board is yours to authorize, and my side is thin.** Here's what I'd do.

## My pick, in order

**1. Fix §7 — it's now actively wrong, not just stale.** It says the send semantics are *"explicitly unruled — do not build until ruled."* Vale2 has ruled them: a send is `Equals`, every parameter gets one, no guard, with phase 0's preview ordering making it safe. And defect 11, the other gate §7 names, landed in the commit we just pulled. So §7 currently tells a reader that case 41 is blocked on two things that are both resolved, when what actually remains is Vale2 writing the send-consumption code. That's an hour, and I have the material in context now.

Same pass picks up two small rots: the warning count is 7 rather than 8, and §10.10's *"does `&Ship` mention `Ship`"* is closed rather than open-and-load-bearing.

**2. Cross-crate re-exports.** Ordinary work, entirely ours, nothing blocked. `fixtures_two_crates` already exists to host it, and it's the last unverified piece of the path story — `std::vec` is `pub use alloc_crate::vec`, a different `module_children` path from the intra-crate re-exports cases 46 and 47 cover.

**3. Then I'd want your word before the next substantial thing**, because everything left that's worth doing is core.

## One thing I'd offer that isn't on the list

**Write case 41 now, pinned to the current failure.** §0.4 says a deferral names its trigger, and *"where a trigger exists, there should be a case pinning the current behaviour so the trigger is observable rather than theoretical."* Case 25 is the worked precedent — it sat as a `should_panic` pinning the collision crash, and flipped to green the moment the fix landed, with the corpus already declaring the right answer.

Case 41 is in exactly that shape now, and better than case 25 was: the failure is legible (`ResolveFailure (unsolved: [CodeRune("T")])`), it's the same failure pure Vale has, and when Vale2 consumes the sends we'd find out from our own suite rather than by asking. The risk is honest — it's a case that pins someone else's bug, so it needs re-checking whenever they move.

## The ruling I'd most like

**The `TooManyTypesWithNameT` conversion.** A bare ambiguous name written in hand-written Vale still panics; the error variant exists, is produced by nothing, and its humanizer is a `panic!`. All three agents last session landed on it independently, and `expression_compiler.rs` already returns `CouldntFindTypeT` in the analogous spot, so there's a working precedent for the shape. It's core, so it's yours — but it's the smallest core change available for the largest reach, and it's independent of every naming decision.

Say the word and I'll start on 1 and 2 without it.

---

## 👤 User

can you look at Vale2's uncommitted stuff and tell me if it unblocks us?

---

## 🤖 Claude

I'm blocked — that's the restart I flagged at the end of the rebase.

Guardian's source moved on disk when we pulled its 5 commits, and this directory's instance isn't answering on port 7882. Every Bash call is refused by the hook, including `echo`, so I can't re

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

sorry go on

---

## 🤖 Claude

Back up.

---

## 🤖 Claude

They're on `experimental-2` at our exact tip, with substantial uncommitted work — including `function_compiler_solving_layer.rs`, where `assemble_initial_sends_from_args` lives. Let me size it.

---

## 🤖 Claude

This is the send-consumption work, in flight. Let me check the two things that could reach us.

---

## 🤖 Claude

Field rename `FunctionS.rules` → `header_rules`, with the constructor unchanged. The question is whether our synthesized declarations use the per-param rule buckets or a flat list.

---

## 🤖 Claude

Looked at it. Answer: **not yet — but it surfaced a defect in our own declarations that's ours to fix and unblocked.**

## What they're building

`experimental-2`, sitting on our exact tip, ~258 lines uncommitted. It is **the send consumption** — the thing case 41 waits on. At two solve sites each `InitialSend` now becomes three things:

```rust
initial_knowns.push(InitialKnown { rune: s.sender_rune, templata: s.send_templata });
call_site_rules.push(IRulexSR::Equals(EqualsSR { left: s.sender_rune, right: s.receiver_rune }));
rune_to_type.insert(s.sender_rune.rune, KindTemplataType);
```

That's the ruled shape — a send is `Equals` — and it's the same three-part lowering their handoff says master's `make_solver_state` used to do before the onion sweep deleted it.

**It does not build.** Two deliberate `unimplemented!("header_rules alone: fold in the per-param type-binding rules, see @PFVSZ")` sit in the middle of two call-site paths, planted as landmines rather than left as TODOs. So there's nothing to measure yet, and nothing to pick up. It unblocks us when it lands, not now.

One thing that will reach us mechanically: they're renaming `FunctionS.rules` → `header_rules`. The constructor still takes `rules` and assigns it, and nothing in `rust_interop/` reads the field, so **we don't break**.

## The finding — and this one is ours

Chasing that rename is what exposed it. For a **source-written** Vale param, `translate_signature_templex` fills two per-param buckets; nothing about a parameter's type goes into the function's flat rule list. For **our synthesized** declarations, `declarations.rs:147-148` passes:

```rust
scout_arena.alloc_slice_from_vec::<IRulexSR<'s>>(Vec::new()),   // type_outer_ref_rules
scout_arena.alloc_slice_from_vec::<IRulexSR<'s>>(Vec::new()),   // value_type_rules
```

Both empty. Every parameter's `LookupSR` + `CallSR` goes into the flat list instead.

**Plan §4 claims the opposite, and I wrote it.** It says the static candidate filter reads a parameter's wrap chain and value-type template name, *"which is exactly the `LookupSR` + `CallSR` pair `declarations.rs` emits for every citizen position — so we satisfy the filter by construction rather than by intent."* We emit the right rules; we emit them in the **wrong place**. The filter reads `value_type_rules`, and ours is empty.

Under their spec an empty `value_type_rules` doesn't read as "invisible" — it reads as **"a bare rune, which accepts anything."** So when the filter lands, every imported Rust function becomes a candidate for every call of matching arity, and with filter-is-final and `>1 → ambiguity`, ordinary Vale calls start colliding with Rust ones. That's worse than not matching.

It's also a straight **§8/@SMLRZ violation** by our own self-check: *"a synthesized declaration should be structurally indistinguishable from what the postparser produces for the equivalent hand-written Vale source."* Ours isn't.

**Why nothing has failed:** the @PFVSZ fold is `header_rules ++ per-param buckets`, so our empty buckets contribute nothing and our flat rules bind the param runes anyway. Same silent shape as everything else this arc — a path that *succeeds* while being wrong. And today's suite can't catch it, because the filter doesn't exist yet.

Their in-flight sends should still reach us regardless: sends target `p.full_type_rune`, which we do set.

## So, revised recommendation

Move this to the front. It's ours, unblocked, small — put each parameter's value-type rules in `value_type_rules` instead of the flat list — and it's the difference between our declarations being filterable and being maximally ambiguous. The doc fix to §4 comes with it, since the claim is currently false.

Two caveats I'd rather state than bury: their tree is mid-edit, so anything about *their* code is a snapshot; and the placement fact is measured in our tree today and doesn't depend on their WIP at all.

Want me to take it? And I'd send them a note — this is exactly the refutable kind §0.5 says pays, and if they're touching @PFVSZ right now it's the moment they'd want it.

---

## 👤 User

plan it, good /tdd please

---

## 🤖 Claude

Baseline first, per the skill: the suite is **628/166/8** interop and **582/166/8** default — not green. That's the onion arc's ratified bar (plan §2), and the operative check is that it doesn't move. Flagging it rather than waving it through.

Now the thing that decides whether this is ours or core:

---

## 🤖 Claude

I have what I need. Writing the plan.

---

## 🤖 Claude

Baseline is already measured on the rebased tip: **628/166/8** interop, **582/166/8** default. Reading the exact region before editing.

---

## 🤖 Claude

Now the change.

---

## 🤖 Claude

Builds. Running **G** — both configs.

---

## 🤖 Claude

Unchanged, exactly as predicted. Now the probe that replaces the missing RED — blank the new bucket and confirm the suite breaks.

---

## 🤖 Claude

The probe fires — **628 → 590**, 38 interop cases broken. Checking *which* failure, per §0.3c.

---

## 🤖 Claude

The mechanism is right: a parameter rune with no conclusion (`check_defining_conclusions_and_resolve` unwrapping `None`), because with the bucket blanked the rules exist nowhere. That proves a parameter's binding now flows through `value_type_rules`. Reverting the probe.

---

## 🤖 Claude

Now **A** — full verification, both configs plus build and driver.

---

## 🤖 Claude

`MultiLineValeFixtures-MLVFX.md` is modified and I didn't touch it. Checking before I go further.

---

## 🤖 Claude

Guardian wrote that itself on restart — a newer schema field (`g_filter_file`) added to its own shield frontmatter. Not mine, benign, and I'll leave it alone rather than sweep it into our change. Flagging it rather than reverting.

Finishing **A**.

---

## 🤖 Claude

7 warnings, all pre-existing, none new. Now the driver.

---

## 🤖 Claude

Driver exit 0. Now the two doc edits.

---

## 🤖 Claude

Small cleanup — `ret_usage` is now a pointless alias.

---

## 🤖 Claude

9 warning lines in the interop test build vs 8 in the lib build — checking whether any are mine.

---

## 🤖 Claude

The extra one is in `compiler_solver_tests.rs` — a core test file my diff doesn't touch, and only compiled in the test target. Nothing from `rust_interop/`. Final default-config run:

---

## 🤖 Claude

Now the note to Vale2, per the plan's last section.

---

## 🤖 Claude

Guardian rejects the multi-line heredoc form. Writing the body to a file first.

---

## 🤖 Claude

Done. The slice is complete and the numbers held.

| | before | after |
|---|---|---|
| interop | 628 / 166 / 8 | **628 / 166 / 8** |
| default | 582 / 166 / 8 | **582 / 166 / 8** |
| `cargo build --lib` | exit 0, 7 warnings | exit 0, 7 warnings |
| driver `valec-rs` | exit 0 | exit 0 |
| core diff | empty | **still empty** |

## What landed

A parameter's value-type rules now live in that parameter's `value_type_rules`. The shared `rules` vector is gone: each parameter's bucket is created inside the loop that creates the parameter and handed straight to `ParameterS::new`, and only the return type's rules ride in the header. **There is no shared list for a parameter's rules to leak into** — that's the rule-1 half of the change, and it came out net simpler, deleting the `bind` closure rather than adding machinery.

One trap worth naming since it's invisible from the diff: `next_synthetic` had to stay function-scoped. It names synthetic runes, so resetting it per parameter would let two parameters mint the same rune name — a silent collision. There's a comment on it now.

## The probe did the work the RED couldn't

The plan said no red was available and the fix would be validated by making it fail afterwards. It fired, and for the right reason:

```
628 → 590 passed, 166 → 204 failed
thread 'rustc' panicked at src/typing/infer_compiler.rs:528
  called `Option::unwrap()` on a `None` value
  ... check_defining_conclusions_and_resolve
```

38 corpus cases, failing on an unconcluded parameter rune. That's what proves a parameter's binding now flows through `value_type_rules` rather than through the header list — if the move hadn't landed, blanking the bucket would have changed nothing. Probe reverted, numbers back to baseline.

## Docs

**§4** rewritten forward: placement is half of static filterability, both buckets are per-parameter, and an empty bucket reads as *"a bare rune, which accepts anything"* — a candidate for every call of matching arity rather than none. **§8** gains the concrete instance: check *where* a rule lives, not only which rules exist, because the @PFVSZ fold unions both and a misplacement typechecks.

## Sent to Vale2

Delivered to `Vale2-cedar`. It's the same invariant their two uncommitted `unimplemented!("header_rules alone…")` landmines are about, seen from the producing side, with the file/line/mechanism so they can check it. I flagged one thing they may have a better angle on: `ParameterS::new` asserts two @PFVSZ invariants but nothing asserts that a parameter's value type is described by its own bucket, which is why this was silent.

## Two things to flag

**`FrontendRust/docs/shields/MultiLineValeFixtures-MLVFX.md` is modified and it isn't mine.** The restarted Guardian added a `g_filter_file: "*"` line to its own shield frontmatter — a newer schema field. Benign, but I left it alone rather than sweeping someone else's write into our change.

**Two things I could not do the documented way.** Guardian's mailbox shield now rejects the heredoc form the `use-mailbox` skill itself prescribes (`cat <<'EOF' | mailbox send`), and `AllowedFileExtensionsOnly-AFEOX` rejects writing a `.txt` scratchpad. I worked around both (a `.md` file plus a single-line `cat | send`), but the skill's own example is currently blocked by the shield, which is worth reconciling.

The pile is now one code file and two doc files, plus the pre-existing untracked convo-12 and `.github/ci-guide.md`.

---

## 👤 User

whats next for us?

---

## 🤖 Claude

Board's changed a little since last time — one item came off, and today's bug made the case for another.

## My recommendation: bank it, then cross-crate re-exports

**Bank it.** The pile is now one code file and two doc files on top of a rebase, all green and self-contained. This change is easy to describe and easy to verify today; it gets harder as it ages under other work.

**Then cross-crate re-exports** (§5.3 step 3). Ordinary, entirely ours, nothing blocked. `fixtures_two_crates` already exists to host it, and it's the last unverified piece of the path story — cases 46 and 47 cover intra-crate re-exports, while `std::vec` is `pub use alloc_crate::vec`, a different `module_children` path.

After that, our unblocked list is down to the `RUST_MODULE` reserved-name check (one check, currently enforced nowhere) and §9 step 3's outbound `GenericArgs` reconstruction — which I'd still hold, since nothing feeds rustc back yet and it would land untested against its real consumer.

## The one I'd now argue for promoting

**Declaration observability** — the item I put in the plan's out-of-scope list. Today is the argument for it: we shipped a structural defect that was invisible at every boundary a test can reach, and it survived a full session of doc-writing that asserted the opposite. It's not a one-off. At least two more properties sit in the same blind spot:

- **@NNGZ's zero-arg `CallSR`** — the rule whose violation cost twelve cases last session has *no test at all*; the corpus catches it only because the failure happens to be loud today.
- **The package path on a citizen position** — the thing `875ba1331` built, asserted nowhere directly.

Cost is one accessor on `TypingPassCompilation` (`typing/compilation.rs`), which is core, so it's yours. That's the whole ask — the harness and tests would be ours.

## Unchanged

**Yours:** the `TooManyTypesWithNameT` conversion (still the smallest core change with the largest reach — a bare ambiguous name written in Vale still panics), the side-table shape for declined reasons, the eagerness ruling, and whether `Vec<int>()` should construct at all.

**Vale2's:** case 39 (`dot_borrow`). Case 41 is worth watching rather than waiting on — the send consumption is in their working tree right now, half-built, and when it lands our generic-drop gap closes with it.

If you only want one thing moving: fire the commit, and I'll take cross-crate re-exports straight after.

---

## 👤 User

<command-message>fire-commit</command-message>
<command-name>/fire-commit</command-name>
<command-args>with experimental</command-args>

---

## 🤖 Claude

Config already read this session: rebase-and-fast-forward, `experimental` family, and we're on side-branch `experimental-4` so the target is the local `experimental`. Starting pre-flight — **0a, external repos**.

---

## 🤖 Claude

Both externals clean and current at **0/0**, no curate queue, all five submodules pinned — step 1 will be a no-op. Checking what the README change is before staging.

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

<command-message>fire-commit</command-message>
<command-name>/fire-commit</command-name>
<command-args>with experimental</command-args>

---

## 🤖 Claude

Config already read this turn: rebase-and-fast-forward, `experimental` family, externals Luz + Guardian. Starting pre-flight — **0a**.

---

## 🤖 Claude

Both clean fast-forwards. Notably Luz's incoming commit is *"MBXX accepts the quoted-heredoc form its own skill documents"* — the exact shield problem I hit sending that note. Pulling.

---

## 🤖 Claude

Both externals at 0/0, nothing to push. One note: `Guardian/Rabble` has untracked `Luz/` content inside it — not mine, not staged.

Staging the host changes. Checking the README diff first.

---

## 🤖 Claude

Four staged. Now the sweeps — **0b, 0c, 0d, 0e**.

---

## 🤖 Claude

0b: no real hits — host's are convo transcripts describing the sweep, Luz's is its own `fire-commit.md`, Guardian clean. 0c is the accepted category (transcript content plus deliberate cross-repo citations). Now **0d** and **0e**.

---

## 🤖 Claude

0d clean — zero broken symlinks. 0e: hits are the curate skill, the config, and a convo log all *documenting* the sweep; none real. No new `#[ignore]`. Now **0f — test delta**.

---

## 🤖 Claude

Test delta: **0 added, 0 deleted, 0 modified** — this change adds no tests, which is the plan's stated position, not an oversight. Now **0g**, the config's gate, unfiltered.

---

## 🤖 Claude

Pre-flight complete. **Stopping at 0g** — the config's gate can't run, for the seventh time and the same structural reason.

## The blocker

`cargo build --manifest-path FrontendRust/Cargo.toml` exits 101 with 8 errors, all in `src/bin/valec/` (`build.rs`, `frontend.rs`, `midas.rs`) — unresolved `backend_ffi` / `pass_manager`, which the onion arc intentionally commented out of `lib.rs`. Neither nextest backend can build its targets either. **None of those files are staged**, and this is unchanged by this commit.

Against the bar you've ratified for exactly this condition:

| | result |
|---|---|
| `cargo build --lib` | **exit 0**, 7 real warnings, all pre-existing |
| `cargo test --lib` | **582** / 166 / 8 |
| `cargo test --lib --features rust_interop` | **628** / 166 / 8 |
| driver `valec-rs` | exit 0 |

Both suites are exactly the pre-change baseline, which is the whole claim this change makes.

## Everything else

| check | result |
|---|---|
| 0a Luz | clean, **pulled 1 commit** to reach 0/0; no curate queue; untracked `tmp/` as always |
| 0a Guardian | clean, **pulled 1 commit + 3 submodule bumps** (ContextifiedShield, Rabble, ShieldFile) to reach 0/0 |
| 0b `DO NOT SUBMIT` | no real hits — host's are convo transcripts describing the sweep, Luz's is its own `fire-commit.md`, Guardian clean |
| 0c absolute paths | ~12 hits, **all the accepted category** — inside the convo-12 verbatim transcript, plus the plan doc's deliberate cross-repo citations of `/Volumes/V/Vale2/vcoord-handoff.md` and the two sibling trees |
| 0d broken symlinks | **none** — a first; earlier runs carried 7 under `Guardian/` |
| 0e temp-disable | no real hits (curate skill, the config, one convo log, all documenting the sweep) |
| 0e new `#[ignore]` | none |

**0f test delta — 0 added, 0 deleted, 0 modified.** Deliberate, not an oversight: the plan established there is no red available for this property, and the validation was a probe (blank the bucket → 38 cases fail at `check_defining_conclusions_and_resolve`) rather than a committed test. I'd rather say that in the commit body than have a reviewer read zero tests as zero thought.

## Staged: 4 files

`declarations.rs` (a parameter's value-type rules move into its own `value_type_rules`); the plan doc's §4 and §8; the README's convo-12 row; and convo-12 itself as a new file.

## Two things needing your call

**`FrontendRust/docs/shields/MultiLineValeFixtures-MLVFX.md` is modified and unstaged.** Guardian wrote it itself on restart — a one-line `g_filter_file: "*"` frontmatter field from the newer schema. It's tracked, so leaving it means the tree is never clean by step 5's letter. I'd stage it as part of this commit with a note in the trailing bullets, but it isn't mine and it isn't interop, so I'm not folding someone else's write in without your word.

**`.github/ci-guide.md`** — untracked, prior session, unrelated, not requested. Deliberately not staged, as the last two commits.

---

You've ratified the 0g substitute six times for this exact condition. Say the word — and tell me whether to include the MLVFX line — and I'll run steps 1–5: commit, rebase onto `experimental`, re-verify after the rebase as a separate step, then ratchet.

⚠️ **Luz and Guardian both moved on disk** during the 0a inbound sync — you'll want to restart this directory's Guardian once we're done.

---

## 👤 User

yep include it, override, proceed

---

## 🤖 Claude

**Step 1 — external repos:** both clean and current at 0/0 after the inbound sync, nothing to commit or push. **Step 2 — host commit.**

---

## 🤖 Claude

Committed `a61dc5f83`. **Step 4 — rebase onto `experimental`.**

---

## 🤖 Claude

Three commits came in — and one is the big one: *"Argument types reach the call-site solve for the first time in the Rust frontend... suite 582 → 608."* That's the send consumption landing. Re-verifying as a separate step, which matters more than usual here since their change and mine touch the same seam.

---

## 🤖 Claude

Interop **653/142/8**, default **608/141/8** — one more failure in interop than default, where before both were 166. That means a corpus case broke. Chasing it before anything else.

---

## 🤖 Claude

Case 32 `wrong_generic_arity_does_not_resolve` now **typechecks when it should fail** — a program calling `pick<A,B>` at the wrong generic arity compiles. That's a real behaviour regression, and their commit touches exactly that area (*"the four rules they meet along the way stop faulting on candidates that simply don't fit"*). Attributing it before assuming, by measuring `experimental` alone.

---

## 🤖 Claude

Confirmed — it fails on `experimental` **without my commit**. Getting the full baseline for attribution.

---

## 🤖 Claude

`experimental` alone measures **653 / 142 / 8** — identical to my rebased branch. My commit moves both suites by exactly zero, and case 32's regression is entirely upstream's. Reading the case so my report to you is precise.

---

## 🤖 Claude

**Stopping before the ratchet.** The re-verify caught something, and it isn't mine.

## The regression is upstream's, measured not assumed

`experimental` advanced by 3 commits, including the big one — *"Argument types reach the call-site solve for the first time in the Rust frontend"*, their suite 582 → 608. The rebase was clean. But one corpus case broke:

**Case 32 `wrong_generic_arity_does_not_resolve` now typechecks when it must fail.**

| | pre-rebase | `experimental` alone | rebased + mine |
|---|---|---|---|
| interop | 628 / 166 / 8 | **653 / 142 / 8** | **653 / 142 / 8** |
| default | 582 / 166 / 8 | — | **608 / 141 / 8** |

I checked out `experimental` and ran it: **653/142/8, identical**, and case 32 fails there without my commit. So my commit moves both suites by exactly zero — as designed — and the interop/default failure counts differing by one (142 vs 141) is entirely that case.

## And I think our case is the thing that's wrong, not their change

The case is:

```vale
return pick<int>(3, true);      // against  pick<A, B>
```

That's **one explicit type arg for two slots — under-supply.** But @ETASTZ, which the case cites, is about the opposite: *"`build_generic_args_for_item` silently discards user-supplied type args **exceeding** the item's Type slot count."*

So case 32 has never actually exercised @ETASTZ's hazard. It passed because argument-driven inference didn't exist — nothing could deduce `B`. Now that argument types reach the call-site solve, `B` is inferred from `true` and the program resolves, which is what Rust does with `pick::<int>(3, true)` and what their phase 0 is *for*.

My read: **their behaviour is right, our expectation is stale, and the case was mis-aimed from the start.** The fix is to re-point it at the real @ETASTZ hazard — excess args, `pick<int, bool, str>` — and confirm that still fails. But that changes what a corpus case asserts, which is exactly what 0f says you confirm, and it isn't part of this commit.

## Your call

The commit is made (`1379a3c7d`) and rebased onto the new tip, **not ratcheted**. Two options:

1. **Ratchet anyway.** My commit changes neither suite, and the regression is already sitting on `experimental` — holding my commit back doesn't unbreak anything. Then case 32 gets its own slice. This is what I'd recommend.
2. **Hold the ratchet** until case 32 is reconciled, so integration never carries a knowingly-red corpus case.

I lean 1, but §2 says movement is a stop and the skill says surface rather than fix, so I'm not choosing it for you.

Externals are done regardless: Luz pulled 1 commit (including *"MBXX accepts the quoted-heredoc form its own skill documents"* — the shield that blocked my mailbox send earlier), Guardian 1 commit plus 3 submodule bumps, both clean at 0/0.

⚠️ **Luz and Guardian both moved on disk — restart this directory's Guardian to pick it up.**

---

## 👤 User

> build_generic_args_for_item silently discards user-supplied type args exceeding the item's Type slot count.

shouldnt that be a compiler error?

---

## 🤖 Claude

Yes. Silently discarding them turns a user's mistake into a plausible wrong answer — the exact failure class §0.2 says to suspect. Let me find where it actually lives before saying whose it is.

---

## 🤖 Claude

`build_generic_args_for_item` doesn't exist in our source at all — the only hit is the corpus comment citing it. Checking whether it's a sibling tree's function.

---

## 🤖 Claude

Yes — and asking that has turned up something better than the answer.

**@ETASTZ is not about that, and it isn't ours.** It's Sky's arcanum, about Sky's `oracle::build_generic_args_for_item` — and their own doc is unsure where it lives (*"probably in Sky's frontend codegen-prep code"*). What it describes is building a `ty::GenericArgs` to hand **back to rustc**: filling Type slots from user args, lifetime slots with `re_erased`, const slots with comptime values. It's the outbound direction — our plan §9 step 3, "Outbound `GenericArgs` reconstruction" — which we have not built.

`build_generic_args_for_item` **does not exist anywhere in our tree.** The only hit is case 32's own comment citing it. So the hazard it names cannot fire here yet.

That makes case 32 mis-aimed twice over:

- **Wrong direction.** @ETASTZ is about *excess* args being truncated; the case supplies `pick<int>` for `pick<A, B>`, which is *under*-supply.
- **Wrong layer.** @ETASTZ is a codegen-prep concern about what we hand rustc; the case tests typing-pass call resolution.

It passed for neither reason — it passed because argument-driven inference didn't exist, so nothing could deduce `B`. Vale2's commit made that inference work, and the case fell over.

This is the third time this arc: an inherited conclusion cited without the condition that justified it (§0.3b), and a sibling tree's symbol read as ours (§0.7). Same shape as the struck no-inference rule.

## So where does your actual question land?

**Should excess type args be a compile error?** Yes, in both places, and they're separate:

- **At typing** — `pick<int, bool, str>` for `pick<A, B>`. That's ordinary arity checking and it's core, so Vale2's. Their model already agrees: the static candidate filter reads **arity first**, before the wrap chain and the template name. Under filter-is-final, wrong arity means zero candidates, which is an error by construction rather than a check someone has to remember.
- **At codegen-prep** — the genuine @ETASTZ, when §9 step 3 builds a `GenericArgs` to feed rustc. That one is ours, it's unbuilt, and the arcanum is worth keeping *there*, pointed at the function we'll write.

**What I don't know is whether excess arity fails today**, because no case covers it — case 32 only ever tested under-supply. That's a one-case probe and I'd want your nod before adding it, since it changes what the corpus asserts.

## What I'd do with case 32

Retarget it to the question it always meant to ask — excess args, `pick<int, bool, str>`, expecting `CouldntFindFunctionToCallT` — and drop the @ETASTZ citation from it, moving that arcanum's reference to §9 step 3 where it actually applies. If the probe shows excess arity *also* resolves silently, that's a real finding for Vale2 and a much better report than "your commit broke our test."

That's a corpus change plus two doc edits, entirely ours, and it's the honest resolution of the red rather than deleting an inconvenient assertion.

Meanwhile the commit is still sitting unratcheted at `1379a3c7d`, and the ratchet decision from my last message is unchanged — my commit moves both suites by zero either way.

---

## 👤 User

yep retarget it please

---

## 🤖 Claude

The shield file now has `g_filter_file: "*"` on **both** line 7 and line 9 — a duplicate YAML key. Checking whether I committed that or it landed after.

---

## 🤖 Claude

That duplicate is in the commit — I staged the file while Guardian was still writing to it, after reviewing an earlier diff. My error. Fixing it, then the case.

---

## 🤖 Claude

Guardian blocks shield-file edits — correctly, since it owns them. So the duplicate key is yours to remove; I'll carry it in the wrap-up. On to the case.

---

## 👤 User

[Request interrupted by user]

---

## 👤 User

i removed it

---

## 🤖 Claude

Thanks. Retargeting the case now.

---

## 🤖 Claude

Now the probe — does excess arity actually fail today?

---

## 🤖 Claude

Passes first try — so excess arity *is* rejected today. Per §0.3c that's exactly when to check it isn't vacuous, so let me confirm the harness verifies the declared error variant rather than just "something failed."

---

## 🤖 Claude

`check` asserts `failure.is(variant)` — the declared `CouldntFindFunctionToCallT` specifically, not merely "something failed." And the case is non-vacuous by construction against case 7, which compiles `pick<int, bool>` from the same fixture and allowlist; the only difference is the extra type argument. Full suite:

---

## 🤖 Claude

**654 / 141 / 8**, and zero failing interop cases. Interop and default failure counts match again (141 = 141), which is the invariant that broke. Confirming default:

---

## 🤖 Claude

Green. Now the two doc edits — the corpus row and moving @ETASTZ where it belongs.

---

## 🤖 Claude

Case 32 is retargeted and the corpus is fully green.

## What the probe answered

**Excess type arguments already fail.** `pick<int, bool, int>(3, true)` against `pick<A, B>` produces `CouldntFindFunctionToCallT` — so the hazard the case always *meant* to guard is guarded at the typing layer, and there's nothing to report to Vale2 after all. The case passed on first write, which is the third time this arc a case has found something other than what it went looking for.

I checked it isn't vacuous rather than trusting the green: `check` asserts the specific declared variant (`failure.is(variant)`), not merely that something failed. And it's an A/B against case 7, which compiles `pick<int, bool>` from the same fixture and the same allowlist — the only difference between the two programs is the extra type argument.

| | before | after |
|---|---|---|
| interop | 653 / 142 / 8 | **654 / 141 / 8** |
| default | 608 / 141 / 8 | **608 / 141 / 8** |
| failing interop cases | 1 | **0** |

Interop and default failure counts match again at 141 — that equality is the invariant that broke, and it's restored.

## What changed

**The case now tests arity.** `pick<int>` → `pick<int, bool, int>`. The doc comment says outright that under-supply is *deliberately not* pinned, and why: `pick<int>(3, true)` is legal because argument types reach the call-site solve and deduce `B`, so a case written against that form tests inference's absence rather than arity — and stops meaning anything the moment inference works. Which is exactly what happened.

**@ETASTZ moved to where it applies** — §9 step 3, outbound `GenericArgs` reconstruction. It describes Sky's `build_generic_args_for_item` discarding args that exceed an item's slot count, which is a hazard of *populating rustc's `GenericArgs`*. We have no such helper yet. The note now says to validate the truncation at the helper site when the helper exists, and that checking a Vale call's arity is a different thing that happens at typing and is core.

**The corpus row in §5.1** carries the same distinction plus the non-vacuity argument.

## Still open

**The ratchet.** `1379a3c7d` is committed and rebased but not ratcheted, and this retarget is uncommitted on top of it. The original question stands, now with a better answer available: with case 32 fixed, ratcheting would put integration in a strictly better state than it's in now — `experimental` currently carries the red case, and this fixes it. I'd fold the retarget into the same commit (amend, since nobody else is on that hash) or land it as a follow-up — your preference.

**The duplicate `g_filter_file` line is committed** in `1379a3c7d`. You've removed it from the working tree, so it'll clear with the next commit. My error for staging a file Guardian was actively writing, after reviewing an earlier version of its diff — I'll re-diff at stage time rather than trusting an earlier read.

⚠️ **Guardian and Luz moved on disk earlier — still wants a restart.**

---

## 👤 User

proceed

---

## 🤖 Claude

Folding the retarget into the commit — nobody else is on that hash, so an amend is clean. Checking `experimental` hasn't moved again first.

---

## 🤖 Claude

0c clean. **0f: 1 test modified** — `wrong_generic_arity_does_not_resolve`, retargeted from under-supply to excess args, which you approved and I'll state in the body. Amending.

---

## 🤖 Claude

Amended to `90109322f`. Verifying on the exact committed tree before ratcheting.

---

## 🤖 Claude

Both hold. **Ratcheting to `experimental`.**

---

## 🤖 Claude

`fire commit with experimental` is complete.

## Landed

**`90109322f`** on `experimental-4`, ratcheted to `experimental` (`ad5564aed..90109322f`). Divergence **0/0**. Per the config's experimental-family model the local ratchet *is* the sync — nothing pushed.

6 files, +4,333 / −826.

## The re-verify earned its place again

The rebase applied cleanly and still surfaced a red case — third consecutive `fire commit` where that's true. What it caught this time wasn't a semantic conflict but a **stale assertion of ours**: upstream's argument-driven inference made case 32's under-supplied `pick<int>(3, true)` correctly compile, and the case had been passing for a reason that no longer existed.

| | pre-rebase | `experimental` alone | landed |
|---|---|---|---|
| interop | 628 / 166 / 8 | 653 / 142 / 8 | **654 / 141 / 8** |
| default | 582 / 166 / 8 | — | **608 / 141 / 8** |
| failing interop cases | 0 | 1 | **0** |

I attributed the movement before accepting it: `experimental` on its own measures 653/142/8, identical to my rebased branch, so **my parameter-rules change moves both suites by exactly zero** — which is the claim it makes. The +25/−25 is upstream's. The +1/−1 on top is case 32 going green.

Interop and default failure counts match again at 141. That equality is the real invariant — it's what says every corpus case passes — and it was the only thing that broke.

## What's in it

A synthesized parameter's type rules now live in that parameter's `value_type_rules` rather than the function's header, so an imported Rust function stays visible to the static candidate filter instead of presenting as a bare rune that accepts anything. Case 32 retargeted at excess arity, which turns out to be rejected already. @ETASTZ moved to §9 step 3 where the helper it describes will actually be written. Plus §4, §8, the corpus row, and the convo-12 transcript.

**Core diff is empty.**

## Externals

Both **clean and current at 0/0**. Luz pulled 1 commit, Guardian 1 plus 3 submodule bumps; nothing needed committing in either.

## Leftovers

```
?? .github/ci-guide.md    ← untracked, prior session, not mine and not requested
?? Luz/tmp/               ← untracked scratch, as always
 ? Guardian/Rabble        ← untracked Luz/ dir inside the submodule, not mine
```

Not clean by step 5's letter — three untracked items, none mine, all pre-existing. Flagging rather than sweeping them into an interop commit.

One correction carried from earlier: the duplicate `g_filter_file` line you removed is in this commit's parent, so `90109322f` has the single-line version. My error was staging that file while Guardian was still writing it, after reviewing an earlier diff — I'll re-diff at stage time rather than trusting an earlier read.

⚠️ **Luz and Guardian both moved on disk during pre-flight — restart this directory's Guardian to pick it up.**

---

## 👤 User

whats next? anything not blocked?

---

## 🤖 Claude

Before I answer — Vale2 just landed the sends, which is exactly what case 41 waited on. Worth measuring rather than assuming.

---

## 🤖 Claude

The blocker **moved again** — out of the abstract-body macro and into a bare `unimplemented!()` in `Compiler::drop`, now reached from `unlet_and_drop_all`. That means the solve gets far enough to actually attempt a scope-end drop. Let me see which arm.

---

## 🤖 Claude

Useful answer: it's line 86, `KindT::KindPlaceholder(_) => unimplemented!()`. So the sends landing discharged the abstract-body cluster, and what's left is a genuinely unfilled arm in `Compiler::drop` — which is precisely the *"Generic drop — 8 tests — `drop`'s `KindPlaceholder` arm"* row on Vale2's own capability ladder. Their handoff calls that row and the 26 *"one cause, not two capabilities"*; after the sends, that's no longer true — the 26 cleared and this one is real work.

## What I'd do next, in order

**1. Probe case 41 — it may already work for our shape.** This is the one I'd take first, because the doc says it's blocked and I don't believe that any more. The pure-Vale twin dies on `KindPlaceholder`, i.e. dropping a value whose type *is* a generic parameter. Our case is `Holder<int>` falling out of scope — a concrete `Struct(Holder<int>)`, which takes the `KindT::Struct` arm that works. Different arm entirely. Writing the case is cheap and either outcome is worth having: green closes the last corpus gap that isn't Vale2's, red gives them a second, sharper data point.

That's the same shape as re-export traversal and case 15 — cases written expecting red that turned out to already work.

**2. Cross-crate re-exports** (§5.3 step 3). Unchanged from last time: ordinary, entirely ours, `fixtures_two_crates` already hosts it, and it's the last unverified piece of the path story.

**3. A doc refresh that just came due.** §7 now names a stale site for the third time — it describes the blocker at the abstract-body macro, and it's moved to `destructor_compiler.rs`'s `KindPlaceholder` arm. §2 and §5.1's counts are also against the old baseline (654/141/8 interop, 608/141/8 default now). Small, and it's the half that rots.

Still ours and unblocked after that: the `RUST_MODULE` reserved-name check. Still premature: §9 step 3's outbound `GenericArgs`.

## Worth telling Vale2

Their ladder predicts the 8 generic-drop tests share a cause with the 26 abstract-body ones. Post-sends that's measurably not the case — the 26's cause is discharged and this is a bare `unimplemented!()`. That's exactly the "panics hide panics" rule they wrote, firing on their own estimate, and it's a one-line report worth sending since it changes what that ladder row costs.

## Unchanged and yours

The `TooManyTypesWithNameT` conversion (still the smallest core change with the largest reach), the side-table shape, the eagerness ruling, and whether `Vec<int>()` should construct.

If you want one thing moving: **the case-41 probe**, then re-exports. Say go and I'll take both.

---

## 👤 User

what currently-failing test would, if became passing, unblock us?

---

## 🤖 Claude

Good question — let me measure it rather than recite the docs, since the blockers have moved twice this session.

---

## 🤖 Claude

That test **passes now** — §7 cites it as the live demonstration of the `dot_borrow` hole, and it's green. The borrow path has moved too. Let me get the actual failing set rather than trust any of it.

---

## 🤖 Claude

`templata_compiler.rs:1227` is exactly case 39's blocker — the borrow read-out tripwire. Getting the specific tests sitting on it.

---

## 🤖 Claude

**`templata_compiler.rs:1227` — the borrow read-out tripwire.** Seven tests sit on it; the most representative is `compiler_tests::test_taking_a_callable_param`. Any of them going green means that panic got filled, and that's the single biggest unblock for us.

```rust
if let KindT::BorrowRef(sb) = source_type {
    if sb.inner == target_type {
        if sb.inner.is_primitive() { return true; }
        else { panic!("is_type_convertible: unhandled borrow read-out ..."); }
```

## Why that one and not a bigger cluster

It unblocks **two** things, and the second is worth more than the case:

- **Case 39** — §5.1 names this exact panic as its blocker: reading a parameter yields `BorrowRef(Counter)` where `get(self Counter)` wants it owned.
- **The return-directly workaround, which shapes every corpus program we write.** Reading a local into a value position yields `BorrowRef(int)` where `int` is wanted, so all 46 cases return call results directly rather than binding them. §7 says to unwind that when the fix lands. That's not one case unblocked — it's a constraint lifted off everything future.

Here's the full failure board, clustered by panic site, so you can see where it sits:

| site | tests | what it is for us |
|---|---|---|
| `destructor_compiler.rs:86` | **17** | `KindPlaceholder` drop arm — case 41 |
| `compiler_outputs.rs:282` | 12 | return-type equality assert (see caveat) |
| `compilation.rs:140` | 11 | real compile errors, triage individually |
| `rune_type_solver.rs:652` | 10 | — |
| `templata_compiler.rs:1233` | 9 | bare-to-borrow, sibling of :1227 |
| **`templata_compiler.rs:1227`** | **7** | **borrow read-out — case 39 + the workaround** |
| `destructor_compiler.rs:{83,70,85}` | 15 | Interface / Str / RSA drop arms |

## Two things that changed under the docs

**`calling_a_method_on_a_local_will_supply_borrow_ref` now passes.** §7 cites it as the live demonstration of the `dot_borrow` hole. It's green, so that half of §7 is describing something that no longer fails — which makes the borrow-path section stale in a way worth fixing when I do the §7 refresh.

**I'd hold off on repeating "export/extern is Vale4's front line."** Their ladder pins that capability to `declare_function_return_type` in `compiler_outputs.rs`, and the 12 failures are there — but the code at :282 is `assert!(*existing == return_type_2)`, a signature declared twice with disagreeing return types. The fix their ladder describes (`is_primitive` rename plus `peel_all_references`) is a different check in a different file. Either the label is stale or the assert is a downstream symptom; I don't want to route work on that description without checking it. Nothing in our corpus is blocked on it today.

## If you want the ranked answer

1. **`templata_compiler.rs:1227`** — case 39 plus the workaround. Smallest cluster of the three, largest effect on us.
2. **`destructor_compiler.rs:86`** — case 41, 17 tests, biggest cluster on the board. But our `Holder<int>` takes the `KindT::Struct` arm, not the placeholder one, so this may not block our case at all — that's the probe I'd still like to run.
3. `compiler_outputs.rs:282` — unclear, per above.

Both 1 and 2 are core, so they're Vale2's. Neither is something I can take.

---

## 👤 User

i asked for which tests

---

## 🤖 Claude

Fair — you asked for tests, I gave you clusters. Here are the names.

---

## 🤖 Claude

Here are the tests, by what they'd unblock for us.

## The one that unblocks the most — `templata_compiler.rs:1227`, borrow read-out

Case 39, **plus** the return-directly workaround that shapes every corpus program we write.

```
after_regions_tests::bound_driven_return_rune_cannot_be_inferred_from_lambda_msae_general
after_regions_tests::brrz_nested_bound_return_inference_through_a_lambda_body
after_regions_tests::brrz_two_bound_return_inferences_in_the_same_call
compiler_lambda_tests::tests_lambda_and_concept_function
compiler_tests::test_taking_a_callable_param
compiler_virtual_tests::basic_interface_forwarder
compiler_virtual_tests::use_bound_from_struct
```

## Its sibling — `templata_compiler.rs:1233`, bare-to-borrow

Same `is_type_convertible` block, other direction. Not a blocker we've recorded, but it's the other half of the same fix, so it'd likely land together:

```
compiler_mutate_tests::can_mutate_an_element_in_a_runtime_sized_array
compiler_mutate_tests::if_branches_moving_same_vars_different_order_compiles
compiler_mutate_tests::if_branches_must_move_same_variables
compiler_mutate_tests::test_mutable_member_permission
compiler_mutate_tests::test_mutating_a_local_var
compiler_ownership_tests::loop_restackify
compiler_ownership_tests::restackify
compiler_tests::reports_when_mutating_after_moving
compiler_tests::reports_when_reading_after_moving
```

Worth noting `if_branches_must_move_same_variables` and its sibling are here — Vale2's handoff says those two *"never reach the join"* because `dot_borrow` panics first. They now reach this instead, so that note has moved on too.

## `destructor_compiler.rs:86` — generic drop, case 41

```
after_regions_error_tests::detects_sending_non_citizen_to_citizen
after_regions_error_tests::hash_map_style_return_type_inference_must_not_skip_caller_bound_args
compiler_mutate_tests::expr_set_upcasts
compiler_mutate_tests::local_set_upcasts
compiler_ownership_tests::opt_with_undroppable_contents
compiler_solver_tests::assume_most_specific_common_ancestor
compiler_solver_tests::assume_most_specific_generic_param
compiler_solver_tests::stamps_an_interface_template_via_a_function_return
compiler_solver_tests::test_calling_a_generic_function_with_a_drop_concept_function
compiler_solver_tests::test_having_drop_function_concept_function
compiler_solver_tests::test_lacking_drop_function
compiler_tests::stamps_an_interface_template_via_a_function_return
compiler_tests::tests_a_foreach_for_a_linked_list
compiler_tests::tests_calling_a_templated_struct_s_constructor
compiler_tests::tests_stamping_a_struct_and_its_implemented_interface_from_a_function_param
compiler_virtual_tests::custom_drop_with_concept_function
compiler_virtual_tests::templated_interface_and_struct
```

`opt_with_undroppable_contents` is the one our §7 tracks by name.

---

The direct answer to your question is the first block — **7 tests, one panic**, and it's the only cluster whose fix changes how we write cases rather than unblocking a single one. `compiler_tests::test_taking_a_callable_param` looks like the cheapest of the seven to drive.

All three sites are core, so none is ours to fill.

---

## 👤 User

what currently-failing test would, if became passing, unblock us? please do NOT tell me about where things in the implementation are failing. just give me a list of *test* names please

---

## 🤖 Claude

`calling_a_method_on_a_local_will_supply_borrow_ref` **passes now** — so §7's cited demonstration of that gap has cleared. Let me find what actually still gates case 39 rather than quote a stale name.

---

## 🤖 Claude

Short list — and it's shorter than I expected.

## Confirmed gating us

```
typing::test::compiler_ownership_tests::opt_with_undroppable_contents
typing::test::compiler_ownership_tests::opt_with_undroppable_mutable_ref_contents
```

Both are the pure-Vale twin of our **case 41** (scope-end drop of a generic citizen). If they go green, case 41 is writable and the last corpus row that isn't a decision closes.

## The honest gap in that answer

For **case 39** (Vale source calling a method on a named Rust parameter) I can't name one. The test our plan §7 has cited all along —

```
typing::test::compiler_ownership_tests::calling_a_method_on_a_local_will_supply_borrow_ref
```

— **passes now.** So either that gap cleared, or it never demonstrated what §7 claims it did. I tried to identify the replacement by clustering the current failures and the result was too noisy to hand you as fact, so I'd rather say I don't know than give you a list I can't stand behind.

The reliable way to find out is to write case 39 and see what it does. That's cheap, it's ours, and it converts "some unknown test gates this" into a name.

## Worth saying plainly

Almost nothing else on our board is waiting on a test. The remaining items are your decisions — the `TooManyTypesWithNameT` conversion, the declined-item side table, the eagerness ruling, and whether `Vec<int>()` constructs — plus ordinary work of ours that nothing blocks. So the answer to "what failing test unblocks us" is genuinely two tests, and one open question I'd like to turn into a third name.

---

## 👤 User

im surprised opt tests gate us, are we really that far into rust interop that something as niche as option is gating us?

---

## 🤖 Claude

You're right to push, and my phrasing was wrong. Those tests don't *gate* us — I overstated it.

**Nothing about `Opt` is in our path.** `opt_with_undroppable_contents` is a hand-written pure-Vale program with a top-level `drop<T>` over a generic container. It appears in our §7 for one reason: it's the cheapest existing proof that "dropping a generic citizen" is a **Vale-wide gap rather than an interop defect**. That's an *attribution* argument — it's why the item sits under "Blocked elsewhere" and is Vale2's rather than ours. It is a witness, not a dependency.

So the accurate version of my list: those are the tests I'd *watch* to know the capability landed. Calling them "confirmed gating us" was a category error, and it's the same mistake case 32 encoded — treating a proxy as the thing.

## And your instinct points somewhere sharper

**We don't actually know that case 41 is blocked.** I inferred it from the doc, and the doc predates the sends landing. Our shape and the `Opt` shape aren't obviously the same problem:

- The pure-Vale test drops a container whose *member type is the type parameter itself*.
- Our case 41 drops `Holder<int>` at scope end — a fully concrete citizen by the time the drop is emitted.

Those may take entirely different paths, and the second one may well work today. That's exactly why I put "probe case 41" first on the last list — not because it's likely blocked, but because the doc *claims* it is and the claim is untested since Vale2's work landed.

## So the honest board

**Zero** currently-failing tests are confirmed to gate us. What I have is:

- one capability we're waiting on (generic-citizen drop) that may or may not still affect our shape — **unmeasured**;
- one case (39) whose cited blocker now passes, so its real status is **unknown**;
- and everything else waiting on your decisions rather than on any test.

Both unknowns resolve by writing the two corpus cases and looking, which is ours and unblocked. Want me to take that? It's the cheapest way to turn "the doc says blocked" into a measurement — and given three of this arc's last four "blocked" items turned out to already work, I'd bet on at least one of them being green.

---

## 👤 User

if there are capabilities that block us, and theyre the compiler broken, then we should probably tell them via a bug report containing a minimal repro failing test, what do you think?

---

## 🤖 Claude

Strongly agree — and there's already a skill for exactly this shape. Let me check what it prescribes before I answer from memory.

---

## 🤖 Claude

Yes — and the skill already prescribes it, including the rule that decides the interesting part: *"place the minimal repro at the earliest pass that triggers it."*

## The refinement that makes it actually useful to them

**Reduce to a pure-Vale program, not an interop corpus case.** An interop case needs the `rust_interop` feature and a live rustc; they can't run it in their default config, so it's a report they have to take on faith. A pure-Vale repro runs in their normal suite, which is the difference between "here's a bug" and "here's a bug you can watch go green."

That's also *self-protecting*, and we've already paid for the lesson. §0.5 records the one report Vale2 pushed back on: we reported `+` resolving zero candidates as a Vale defect, and they were right that our harness supplies no builtins at all. Reducing to pure Vale forces us to prove the gap isn't our own harness before we send it. `opt_with_undroppable_contents` is valuable to them for exactly that reason — it's why §7 cites it at all.

## Two cautions

**A new red test moves their baseline.** Their handoff treats suite movement as a stop and their capability ladder is first-blocker counts, so a test we add lands as an unexplained regression in someone else's measurement. I'd send the reduced *program* plus the assertion we'd want, and let them land the test — or add it only with their nod. Adding it silently is the kind of thing that costs an afternoon of "why did the count move."

**Check an existing test doesn't already show it.** If `opt_with_undroppable_contents` already witnesses the capability, a second repro is noise. The value is strictly when nothing existing covers it.

## Where that leaves us right now

We have **no confirmed compiler bug to report** — both candidates are unmeasured, and the skill's step 1 is *probe, observe failure*. So the sequence is unchanged, it just has a better ending:

1. Write case 41 and case 39. Either they're green — which closes them and there's nothing to report — or they fail.
2. For whichever fails, reduce to the smallest pure-Vale program that shows the same thing.
3. Send that, with the assertion we'd want and the observation that it isn't interop-specific.

And per the skill's "don't trust the bug-report's framing" rule pointed at ourselves: our own §7 has now cited a stale blocker three times, and the test it named for case 39 currently passes. So step 1 isn't optional — writing those two cases is what replaces a doc claim with a measurement.

Want me to start there? It's the same work I proposed last turn, now with a defined output if it goes red.

---

## 👤 User

1 2 3 sound good, proceed

---

## 🤖 Claude

Starting with the two cases. Reading the closest existing siblings so they match the corpus's shape.

---

## 🤖 Claude

Now both cases.

---

## 🤖 Claude

**Case 41 passes.** Now case 39.

---

## 🤖 Claude

Case 41 green, case 39 red — but with `CouldntFindFunctionToCallT`, a designed error rather than the borrow-read-out panic §7 predicted. Reading the detail.

---

## 🤖 Claude

Case 41 is green; case 39 fails on `get` with a designed `CouldntFindFunctionToCallT` — not the panic §7 predicted. Full suite, and checking whether a pure-Vale test already witnesses this.

---

## 🤖 Claude

`passing_bare_local_to_borrow_param_does_not_need_ampersand` is the *borrow* direction (`&SomeStruct`), not ours. But that comparison suggests a hypothesis worth testing before I call this a compiler bug: `get(self Counter)` **consumes**, and this codebase's rule is that consuming a local needs an explicit move. Testing whether the program is simply wrong Vale.

---

## 🤖 Claude

**Case 39 passes with the move spelling.** There is no compiler bug — our program was wrong Vale. `get(self Counter)` consumes its receiver, and consuming a local requires an explicit `^` here. Full suites:

---

## 🤖 Claude

Corpus at **48 cases, all passing**. 656/141/8 interop, 608/141/8 default, failure counts matched. Now the docs — starting with the trap this uncovered.

---

## 🤖 Claude

Now the plan doc — three sections claim these are blocked.

---

## 🤖 Claude

Now §7 and §9's gap block, both of which describe these as blocked.

---

## 🤖 Claude

§7 is now entirely about things that aren't blocked. Rewriting it — the borrow-path half first.

---

## 🤖 Claude

Done — and the answer is that **no bug report is owed.** Both cases pass.

## What the probes found

**Case 41 — green on first run.** Scope-end drop of a generic Rust citizen works. It needed argument types to reach the call-site solve, which is exactly what Vale2 landed, so `drop<T>(Holder<T>)` now deduces `T` from the value being dropped with nothing written at the call site.

**Case 39 — red, then green, and the compiler was never at fault.** It first failed with `CouldntFindFunctionToCallT` — a *designed error*, not the `is_type_convertible` panic §7 predicted for three sessions. That mismatch was the tell. The actual cause: `get(self Counter)` takes its receiver **by value**, so calling it consumes, and consuming a local here requires an explicit move. `(c).get()` → `(^c).get()` and it compiles.

That's the same rule the codebase already states as *"`drop(bare_local)` is a compile error, only `drop(^local)` is valid"* — we just had never written a program that consumed a Rust value from a local.

So step 3 of the plan — reduce and send — has nothing to send. Which is the right outcome: we'd have filed a bug against a spelling error in our own test program.

| | before | after |
|---|---|---|
| interop | 654 / 141 / 8 | **656 / 141 / 8** |
| default | 608 / 141 / 8 | **608 / 141 / 8** |
| corpus | 46 cases | **48, all passing** |

Interop and default failure counts still match at 141 — every corpus case green.

## The trap worth keeping

The failure names the **callee**, not the mention. `CouldntFindFunctionToCallT: get` reads like a missing import or a bad allowlist, while the fix is a `^` three tokens away in the caller. That's recorded on case 39 and in §4, because it will bite again the moment anyone writes a corpus program that consumes a Rust value.

## Docs

§7 is now down to one entry — **tier 2** — since both things it described are corpus cases that pass. The §5.1 preamble that said "both remaining cases are blocked" is replaced with the rule this earned:

> **Write the case before believing it is blocked.** Four rows were recorded as blocked and needed nothing: re-export traversal, case 15's decline, and cases 39 and 41. Two waited on another team who had already shipped the capability, and one waited on a compiler defect that was really a call-site spelling.

Also corrected: §9 step 2's generic-drop gap block, §4's stale borrow-read-out bullet, and case 38's comment, which asserted that `(c).get()` "does not compile" — true when written, wrong now, and wrong about the reason.

## One thing to flag

A pile of untracked `docs/convos/convo-59…71-*.md` just appeared, including one for *this* session. That's Guardian's new exporter from the pull — it's writing convo docs into the repo automatically now. Not mine and not staged; you'll want to decide whether those belong in git before the next commit, because they'll show up in every `git status` from here.

---

## 👤 User

that was a lot, TLDR?

---

## 🤖 Claude

**Nothing was blocked. Both cases pass, and there's no bug to report.**

- **Case 41** (generic scope-end drop) — green first try. Vale2's send work unblocked it.
- **Case 39** (method on a Rust-typed parameter) — failed, but the compiler was right and our program was wrong. `get(self Counter)` consumes, so the caller must move: `(c).get()` → `(^c).get()`. Then green.

Corpus **46 → 48, all passing.** 656/141/8 interop, 608/141/8 default.

**The one thing worth remembering:** the error names the *callee* (`CouldntFindFunctionToCallT: get`), so it reads like a bad allowlist when the fix is a `^` in the caller. That'll bite again.

**§7 is down to one entry — tier 2.** Everything else it listed is now a passing case.

And the rule this bought, now in the doc: *write the case before believing it's blocked.* Four rows said blocked and needed nothing.

**Loose end:** Guardian's new exporter is auto-writing `docs/convos/convo-59…71-*.md` into the repo, including this session. Untracked, not mine — worth deciding if they belong in git before the next commit.

---

## 👤 User

does that mean we arent blocked on anything?

---

## 🤖 Claude

Not blocked by anyone else — correct. But not dependency-free either.

**Nothing is waiting on Vale2 or on a failing test.** That's genuinely new; it was two cases and a capability this morning.

**Four things are waiting on you**, all core so I can't take them:
- the `TooManyTypesWithNameT` conversion (still the smallest core change with the largest reach)
- the declined-item side table's shape
- the eagerness ruling
- whether `Vec<int>()` should construct at all

**Tier 2 is scheduled, not blocked** — LLVM port plus their ~3-week relink.

**And there's real work I can do right now with none of the above:** cross-crate re-exports, and the `RUST_MODULE` reserved-name check.

One caveat, given the day I've had: "not blocked" is precisely the claim I got wrong four times in these docs. What I've *measured* is the two cases. The four items above are genuinely decisions rather than unknowns — but if any of them turns out to be another stale "blocked," the way to find out is the same as today: write the case and look.

---

## 👤 User

TooManyTypesWithNameT sounds like some sort of error, lets defer the error handling until after the other things. tell me about the other 3 items?

---

## 🤖 Claude

One correction first: **the side table is the same category you just deferred.** It's about surfacing *why* an item declined at the lookup that fails to find it — `CouldntFindFunctionToCallT` saying "found `first`, but its return type has no Vale form" instead of "couldn't find function." That's diagnostics, so I'd park it with `TooManyTypesWithNameT` rather than treat it as a separate call.

That leaves two, and they turn out to be the same agenda.

## They're both `Vec` gates

§9 lists four things `Vec<int>()` needs. Steps 1 and 2 are done. These are steps 3 and 4 — nothing else stands between us and walking real `std`.

### Eagerness — half ours, half yours

Today every allowed item is resolved, declared **and compiled**, whether the program mentions it or not. That's invisible at a five-name allowlist and it stops being invisible the moment `Vec` arrives with ~100 inherent methods, each getting a full declaration compiled for a program that calls two of them.

The split matters for who decides:

- **Ours** is the per-type method fan-out — importing a type declares every method on it. Already bounded by the allowlist, so it's the cheaper half.
- **Yours** is the expensive half: declarations get compiled by the loop in `Compiler::evaluate` that walks every top-level store. Lazy population means changing that loop and the lookup driving it, which is core.

rustc solves this with `populate_on_access` — fill a foreign module's children on first touch, never up front. Harmonious's counsel was the same: keep the wrapper, attack the eagerness.

One thing already ruled out, worth not re-proposing: **don't filter by scanning the Vale source for names.** An item can be reached without its name appearing — a drop we synthesize, a method reached through a generic instantiation — so it would silently drop declarations, in the direction that fails quietly.

### `Vec<int>()` — a decision before it's work

`Vec<int>(...)` is a **call**. For a Vale struct the callee is the macro-derived field constructor, which only exists for parsed `StructS` denizens — a Rust-backed type has none.

**That's correct and should stay correct.** Vale is an external consumer, `Vec`'s fields are private, and synthesizing a field constructor would claim knowledge of a layout and invariants we don't have. So construction has to route to a Rust associated function.

The question is what the surface says:

- `Vec<int>()` constructs, and the compiler routes the bare call to `Vec::new` — convenient, but it means a bare call means two different things depending on whether the type is Vale-backed.
- Or Vale source writes `Vec<int>::new()` (or equivalent), and the bare-call form keeps meaning "Vale struct literal" only.

I lean the second — it's the @NNGZ-shaped answer, in that it refuses a special case rather than adding one, and it keeps "bare call = field constructor" a single rule. But it's a language surface decision and genuinely yours; nothing should be built for step 4 until it lands.

## What I'd suggest

Neither is urgent **unless we're going for `Vec`**. If we are, the sequence is: your `()` ruling, your eagerness ruling, and §9 step 3's outbound `GenericArgs` reconstruction — which is ours and needs no ruling at all.

If we're not going for `Vec` yet, both of these park cleanly and I'd take cross-crate re-exports and the `RUST_MODULE` check instead.

---

## 👤 User

please start on the cross-crate re-exports, and the RUST_MODULE reserved-name check, proceed, ty

---

## 🤖 Claude

Starting with cross-crate re-exports. Reading the two-crate fixture and the reserved-name module.

---

## 🤖 Claude

The stub already does `pub use mycrate::make_widget;` — a cross-crate re-export. Let me see how the intra-crate cases are shaped before adding to it.

---

## 🤖 Claude

Now I have the shape. `std::vec` is `pub use alloc_crate::vec` — a cross-crate re-export of a *module* — so I'll mirror cases 46/47's item/module split. Fixture first.

---

## 🤖 Claude

Now the two cases.

---

## 🤖 Claude

Running both — this is the F step, and I genuinely don't know which way it goes.

---

## 🤖 Claude

Both **RED** — a real one. Diagnosing before touching anything.

---

## 🤖 Claude

`importable_functions -> []` — nothing imported at all, the signature of the allowlist resolving nothing. Checking which crates the oracle actually walks before I call this a compiler bug.

---

## 🤖 Claude

Found it, and it's my fixture that's wrong, not the compiler: the walk iterates `tcx.crates(())`, which is the *external* crates — the stub is the local crate, so modules I put there are invisible by construction. The real `std::vec` shape is dep→dep. Checking whether the harness can build one dep against another.

---

## 🤖 Claude

`crates.sort()` — deterministic, and `mycrate` sorts before `othercrate`, so othercrate can build against mycrate. That's the dep→dep shape `std::vec` actually has. Moving the re-exports there.

---

## 🤖 Claude

Both pass — **cross-crate re-exports already work.** The walk follows them regardless of crate, same as intra-crate. Full suite to lock that in:

---

## 🤖 Claude

Corpus at 50, zero failures. Now the `RUST_MODULE` check — first, where the collision would actually be visible.

---

## 🤖 Claude

The `Compiler` holds no package list, so this can't be done from `rust_interop/` alone — it needs the namespace list, which lives in `Compiler::evaluate`. Let me confirm the exact shape before bringing you the hunk.

---

## 🤖 Claude

11 warning lines where there were 8. Checking — my changes are all feature-gated so they shouldn't touch the default build.

---

## 🤖 Claude

Both slices are done, and the second one stops short of core deliberately. Summary:

## Cross-crate re-exports — **already work**, once the fixture posed the right thing

Corpus **48 → 50**, both green. 658/141/8 interop, 608/141/8 default, zero failing cases.

My first attempt went red, and it was my fixture at fault, not the compiler: I put the re-exports in `stub.rs`, but the walk iterates `tcx.crates(())` — the *external* crates — so anything in the crate being compiled is invisible by construction. That's correct behaviour, and it's now a recorded fact rather than a thing to rediscover.

The real shape is dep→dep, which is what `std::vec` is (`std` re-exports from `alloc`). Moved the re-exports into `othercrate` re-exporting from `mycrate` — the harness sorts dependency crates, so `mycrate` is built first and that direction works deterministically. Both forms pass: a re-exported **item**, and descending **through** a re-exported module.

## `RUST_MODULE` check — stopping at the core boundary

It can't be done from `rust_interop/` alone. `Compiler` holds no package list, so the collision is only visible in `Compiler::evaluate`, where the user namespaces are built. The good news is the predicate already exists in our `reserved.rs`, so the core hunk is four lines inside the *existing* `#[cfg(feature = "rust_interop")]` block — no new footprint:

```rust
#[cfg(feature = "rust_interop")]
{
    if let Some((id, _)) =
        namespace_name_to_templatas_vec.iter().find(|(id, _)| is_rust_backed(id)) {
        panic!("a Vale package claims the reserved `rust` module: {id:?}");
    }
    for (package_id, store) in rust_package_stores(self) { ... }
}
```

Two reasons I didn't land it: it's core, so §0.1 says bring the hunk rather than land it; and **it has no test** — our harness fixes the package coordinate to `test`, so a corpus case can't name a package `rust`. Also, `panic!` is the wrong rendering; a real diagnostic needs an error variant, which is the diagnostics bucket you just parked. Your call whether the loud-but-ugly version is worth having now.

## One thing that changed underneath us

**Warnings went 7 → 10**, and §2 says that's a stop rather than a footnote. All three new ones are upstream's, in files I haven't touched — two of them are the `unimplemented!("header_lines alone: fold in the per-param type-binding rules, see @PFVSZ")` landmines, which **landed on `experimental`** rather than staying in Vale2's working tree.

Correction to my earlier report: those two are now committed upstream, not uncommitted. They sit on the pre-generic templated call paths, which we established our declarations don't travel — so inert for us, but they'll panic for anyone who does reach them, and my commit message's "7 warnings" is now stale.

---

## 👤 User

can we add a test?

---

## 🤖 Claude

Fair challenge — "can't test it" is exactly the claim I've gotten wrong repeatedly today. Let me check rather than repeat it.

---

## 🤖 Claude

The package coordinate is built right there and hardcoded. But `TypingPassCompilation::new` is public and our harness already constructs everything else — so we can build it ourselves without touching core. Checking the constructor.

---

## 🤖 Claude

`test_typing_pass_options` is a plain struct literal over public types. So **yes — we can test it, entirely in our own harness, no core change.** My "can't" was wrong again. Building it.

---

## 🤖 Claude

Now the entry points — `run_case` becomes the degenerate case of a package-aware one.

---

## 🤖 Claude

That's the known `src/bin/valec` blocker from `--tests` pulling in bin targets. Using the lib test target instead.

---

## 🤖 Claude

Compiles. Now the case that names the reserved module.

---

## 🤖 Claude

Now the RED run — this is the measurement that tells us what actually happens today.

---

## 🤖 Claude

**Genuine RED, for the right reason.** A Vale package compiled as `rust` sits alongside the imported crate's `rust.["mycrate"]` store and the compilation proceeds — silently, exactly as §6 predicted. Tests are correctly failing; here's the core hunk, which is now the only thing left.

---

## 🤖 Claude

The test is red and the fix is a core change, so I'm stopping at the boundary per §0.1. Here is the exact hunk.

**`FrontendRust/src/typing/compiler.rs`**, replacing lines 706–712 — inside the existing `#[cfg(feature = "rust_interop")]` block, so our core footprint doesn't widen:

```rust
        // The reserved `rust` package's top-level namespaces, one per imported Rust crate.
        // Adding them here is what lets a Rust free function be found by ordinary ambient
        // name lookup, the same way any Vale function is.
        //
        // A Vale package claiming `rust` would be indistinguishable from an imported crate at
        // `lookup_nearest_with_path`, which selects a store by matching a coordinate whole and
        // takes a match rather than reporting two — so the collision is silent. Refuse it here,
        // where both sets of namespaces are in hand for the only time.
        #[cfg(feature = "rust_interop")]
        {
            if let Some((id, _)) =
                namespace_name_to_templatas_vec.iter().find(|(id, _)| is_rust_backed(id))
            {
                panic!("a Vale package claims the reserved `rust` module: {id:?}");
            }
            for (package_id, store) in rust_package_stores(self) {
                namespace_name_to_templatas_vec.push((package_id, store));
            }
        }
```

plus one import line alongside the existing gated one at `compiler.rs:37`:

```rust
#[cfg(feature = "rust_interop")]
use crate::typing::rust_interop::{is_rust_backed, rust_package_stores};
```

**Why it's this shape:** the predicate already exists in our `reserved.rs` and is already exported, so the logic stays ours and core gains a call rather than a rule. The check runs before the rust stores are pushed, so it compares against the user's namespaces only.

**The one thing I don't love, and it's yours to weigh:** `panic!` is the wrong rendering for a user-caused condition. The right form is an `ICompileErrorT` variant, which needs `compiler_error_reporter.rs` plus a humanizer arm — the diagnostics bucket you just parked. So this is the loud-but-ugly version, tier 2 in §0.3c's ordering, and it upgrades cleanly to a real error later without the test changing: the test asserts the compilation is *refused*, not how it's refused.

## Where things stand

**Cross-crate re-exports: done and green.** Corpus 48 → 50.

**Reserved-module check: test written and red, fix not landed.** So the tree is currently 658 passing with **one deliberate failure** — I've not committed anything, and I won't until you take or decline the hunk.

I was wrong twice about this being untestable. The harness now builds its own `TypingPassCompilation` instead of calling the core test helper, which is what made a caller-chosen package possible — `run_case` is just `run_case_in_package(case, "test", ..)`, so the ordinary value is an argument rather than a second path. That change is entirely in our own `harness.rs`.

Side effect worth flagging: `compiler_test_compilation_with_rust_oracle` in core now has **no callers**. It exists only for us, so it's a candidate for deletion — but that's core too, and I'd rather name it than quietly leave a dead function behind.

---

## 👤 User

whats next?

---

## 🤖 Claude

Three things, and the first two are quick.

## 1. The core hunk — one decision

The reserved-module test is red and stays red until you take or decline that four-line check in `compiler.rs`. Everything else is green. If you'd rather not carry a `panic!` for a user-caused condition, declining is reasonable — but then I'd delete the test rather than leave a known-red case in the corpus, and record the gap in §6 as still-unenforced.

## 2. Bank the pile

It's grown a lot since `90109322f` and it's all verified: cases 39 and 41, the two cross-crate re-export cases, the corpus at 50, the harness's package-aware refactor, and a substantial doc pass (§5.1, §7 down to one entry, §9 step 2, §4). Plus the reserved-module case pending item 1.

Two things to fold in when we do: the warning count moved 7 → 10 (upstream's, from the landed `unimplemented!()`s), and `compiler_test_compilation_with_rust_oracle` is now callerless.

## 3. Then the board is genuinely thin, and that's the news

After this, what's left that's **ours and unblocked** is §9 step 3's outbound `GenericArgs` reconstruction — which I'd still hold, since nothing feeds rustc back yet and it would land untested against its real consumer.

Everything else is one of:
- **Deferred by you** — the diagnostics bucket (`TooManyTypesWithNameT`, the declined-item side table).
- **A decision** — eagerness, and whether `Vec<int>()` constructs.
- **Scheduled** — tier 2, behind the LLVM port.

So the real question after banking is **whether we're going for `Vec`.** If yes, those two decisions are exactly the gates and step 3 stops being premature — it's the whole remaining §9. If not, I'd suggest the diagnostics bucket next despite the deferral, because it's the only substantial work left that doesn't need a language ruling, and "found `first`, but its return type has no Vale form" is the difference between a usable compiler and one that lies about what exists.

My pick: decline or take the hunk, fire the commit, then go for `Vec`.

---

## 👤 User

whats the four-line check?

---

## 🤖 Claude

Just this, placed immediately before the existing `for (package_id, store) in rust_package_stores(self)` loop in `Compiler::evaluate`:

```rust
if let Some((id, _)) =
    namespace_name_to_templatas_vec.iter().find(|(id, _)| is_rust_backed(id))
{
    panic!("a Vale package claims the reserved `rust` module: {id:?}");
}
```

At that point in `evaluate`, `namespace_name_to_templatas_vec` holds **only the user's Vale packages** — the Rust stores get pushed on the very next line, which is why this is the one moment both sets are distinguishable.

`is_rust_backed` is our existing predicate in `reserved.rs`, already exported: it's just `id.package_coord.module == "rust"`. So core gains a call, not a rule.

Two supporting bits:
- it goes inside the `#[cfg(feature = "rust_interop")]` that's already there, so the block needs braces around the check plus the existing loop;
- the existing gated import at `compiler.rs:37` grows one name: `use crate::typing::rust_interop::{is_rust_backed, rust_package_stores};`

The `panic!` is the part I'd flag. It's the correct *tier* — a loud failure, since the type system can't express "no user package may be named `rust`" — but the wrong *rendering* for something a user can cause. Upgrading it to a real `ICompileErrorT` later won't touch the test, which asserts the compilation is refused rather than how.

---

## 👤 User

add it please

---

## 🤖 Claude

This is the **G** step — the same test, now expected to pass.

---

## 🤖 Claude

The check fires, but my test asserts the wrong shape — a `panic!` unwinds rather than arriving as a `CompileFailure`. Let me see how it actually surfaces.

---

## 🤖 Claude

The check works and the message names the offending package. Fixing the test to expect a panic, with a control that proves it's about the package name and not the program.

---

## 🤖 Claude

Both green. Now **A** — the full suite in both configs, since this touched core.

---

## 🤖 Claude

Landed and green.

| | before | after |
|---|---|---|
| interop | 658 / 141 / 8 | **660 / 141 / 8** |
| default | 608 / 141 / 8 | **608 / 141 / 8** |
| `cargo build --lib` | exit 0, 10 warnings | exit 0, 10 warnings |
| failing interop cases | 0 | **0** |

Default is untouched, which matters more than usual here — the check sits behind `#[cfg(feature = "rust_interop")]`, so a build with the feature off can't reach it, and the number confirms it rather than the reasoning alone.

## What went in

**The check itself**, exactly as quoted, plus one name on the existing gated import. Core diff is +16/−3 in `compiler.rs` — still the one gated block, not a second one.

**Two tests, and the second one is the point.** The refusal is a panic, so the case is `#[should_panic(expected = "claims the reserved `rust` module")]`. On its own that would pass for *any* reason the compilation blew up — so the control compiles the identical program under an ordinary package name and asserts the Rust call resolves. Same program, two package names, opposite outcomes; that's what makes the assertion about the reservation rather than about the program.

It asserts the **message**, not the mechanism, so converting the panic to a real `ICompileErrorT` later won't touch the test.

## One correction to what I told you earlier

I said the harness change made the test possible without any core touch — true of the *test*, but the fix was always going to be core, and my first version of the test asserted the wrong shape. It expected a `CompileFailure` when a `panic!` unwinds straight past that. The RED I reported was real but for a slightly different reason than the one I gave; the check firing is what corrected it.

## Where we are

Corpus **51 cases**, all passing. §6's *"`RUST_MODULE` is reserved by comment and enforced nowhere"* is now false and wants updating with the rest of the doc pass.

The pile is substantial and entirely verified: cases 39 and 41, two cross-crate re-export cases, the reserved-module pair, the harness's package-aware refactor, this core check, and the §5.1/§7/§9/§4 doc rewrites. That's the next thing — bank it, then the `Vec` question.

---

## 👤 User

what does bank it mean

---

## 🤖 Claude

**"Bank it" just means commit** — I've been using it loosely and it isn't standard. Sorry.

Concretely: run `fire commit with experimental`, which commits the working tree, rebases onto `experimental`, re-verifies, and ratchets. Nothing here needs a new word — I should have just said "commit it."

It only takes your literal "fire commit" to authorize, so nothing happens until you say it.

Right now there are 8 changed files sitting uncommitted since `90109322f`:

- `corpus.rs` / `cases.rs` — cases 39, 41, two cross-crate re-exports, the reserved-module pair
- `harness.rs` — the package-aware refactor
- `compiler.rs` — the core check
- the two `fixtures_two_crates` files
- the plan doc — §5.1, §7, §9 step 2, §4

All green at 660/141/8 interop and 608/141/8 default, with §6's `RUST_MODULE` line still to update in the same pass.

---

## 👤 User

no commit. anything else unblocked?

---

## 🤖 Claude

Yes — three, and one of them is more valuable than it sounds.

## 1. The composition case has gone stale (my pick)

Case 48 exercises sixteen mechanisms in one program, and it exists for one reason no narrow case can serve: **interference**. A shared name resolving to the wrong item, an import-order dependency, a drop that only works when it's the only drop.

It was written when the corpus was 45 cases. We're at 51. Everything added since — generic scope-end drop, a moved receiver on a Rust-typed parameter, cross-crate re-exports through two hops — has only ever been exercised *alone*. Generic drop is the one I'd most want composed: it's the newest capability, it depends on argument-driven inference that landed days ago, and "works alone, breaks alongside four other drops" is exactly its plausible failure.

Ours, unblocked, and it's the case most likely to find something.

## 2. A scoping case for what the walk deliberately excludes

The oracle iterates `tcx.crates(())`, so items defined in the *compiled crate itself* are never importable. That's correct — the stub exists to force deps to load, not to export — but it's undocumented, untested, and it cost me a full cycle today when I put a fixture in the wrong crate.

It belongs in family D beside the other scoping cases, and it's about four lines.

## 3. The bare-generic parameter's rune shape

I put this out of scope in the parameter-rules plan and it's still open. For `func foo<T>(x T)` the postparser uses the declared rune directly; we mint an `ArgumentRune` and add an `Equals` to tie them. Filter-correct either way, but it's a second structural divergence from hand-written Vale — the same §8 self-check that caught the rule-placement bug.

Small, ours, and it's the last known instance of that pattern.

## Still not recommending

**§9 step 3** (outbound `GenericArgs`) — ours and unblocked, but nothing feeds rustc back yet, so it lands untested against its real consumer. I'd rather it wait for the `Vec` decision that gives it a consumer.

**The doc pass** — §6's *"`RUST_MODULE` is reserved by comment and enforced nowhere"* is now false, and §2's counts are stale. That's small and rides along with whatever lands next rather than being work of its own.

If you want one: **case 48**. It's the only thing on the list that can find a defect rather than just record one.

---

## 🤖 Claude

Watcher re-armed.

Nothing else has changed — the tree is still uncommitted at 660/141/8 interop and 608/141/8 default, and the three unblocked items from my last message stand, with case 48's composition refresh as the one I'd take.

---

## 👤 User

do 1-3 please, ty

---

## 🤖 Claude

Starting with case 48. Reading it and what it currently covers.

---

## 🤖 Claude

Now the Callee shape, so I can assert the new mechanisms rather than just add lines.

---

## 🤖 Claude

`params` carries rendered types, so a generic drop is distinguishable from the others. Updating the program.

---

## 🤖 Claude

That allowlist entry isn't needed — `make_holder` and `Holder` are already allowed, and the drop comes from binding rather than from a new item. Removing it.

---

## 🤖 Claude

Now the assertions — the additions need to be observable, not just present.

---

## 🤖 Claude

Passes — but case 48's own history says validate the assertion by making it fail. Probing whether the generic-drop assertion actually discriminates.

---

## 🤖 Claude

Exactly right — 4 non-generic drops and no generic one, matching the callee dump from when case 48 was written. The assertion discriminates. Restoring.

---

## 🤖 Claude

Now item 2 — the scoping case for what the walk deliberately excludes.

---

## 🤖 Claude

Item 3 — making a bare-generic parameter use the declared rune directly, as the postparser does.

---

## 🤖 Claude

Now `bind_sig_type` returns the rune that denotes the position, so the `Equals` disappears.

---

## 🤖 Claude

All three done, everything green.

| | start of this batch | now |
|---|---|---|
| interop | 660 / 141 / 8 | **661 / 141 / 8** |
| default | 608 / 141 / 8 | **608 / 141 / 8** |
| failing interop cases | 0 | **0** |
| corpus | 51 | **52** |

## 1. Case 48 refreshed — and the new assertion was validated by making it fail

Added a generic citizen left to fall out of scope, and a Vale function taking a Rust-typed parameter with a moved receiver.

The drop assertion is on **shape, not presence**: it requires both a generic drop (a param rendering with `<`) and a non-generic one in the same program. I checked it discriminates by removing the binding — it failed with exactly the right message and dumped `[["rust-citizen"] × 4]`, matching the callee list from when case 48 was written. So the composite genuinely exercises five drops of two shapes now, where before it was four of one.

**Deliberately not composed:** cross-crate re-exports. They need two dependency crates and case 48 lives in the single-dep `fixtures/`; adding a second dep there would perturb 40-odd existing cases for a mechanism whose interference risk is low. Noted rather than done.

## 2. The scoping case

`stub_only` now exists in `fixtures/stub.rs` and is unimportable — the walk resolves against `tcx.crates(())`, the loaded *dependency* crates, so the crate being compiled is out of scope. The case asserts both that the program fails and that `stub_only` was never *offered*, which is the assertion that actually discriminates. Case 1 is the control: the same mechanism does reach an item in a dependency.

This is the thing that cost me a cycle earlier today when I put the cross-crate fixture in the stub.

## 3. The bare-generic parameter rune — and it came out simpler

`bind_sig_type` now **returns the rune that denotes the position** instead of taking one and sometimes equating to it. A bare generic comes back as the declared rune with **no rule at all**, matching what the postparser emits for `func foo<T>(x T) T`.

Three things fell out: the `EqualsSR` for generic positions is gone entirely, the argument loop lost its special case for generic arguments (an argument is now just another position through the same call), and two parameters of the same type share one rune — which is what `f<T>(a T, b T)` actually means, rather than two runes tied together.

That closes the last known instance of the §8 self-check divergence I found this morning.

**One thing to flag:** `EqualsSR` may now be unused in `declarations.rs`. Warnings held at 10 real so the import is presumably still live via another path, but it's worth a look in the deadweight pass rather than assumed.

---

## 👤 User

anything unlocked we can do?

---

## 🤖 Claude

Honestly: **no new capability opened up.** Those three were coverage and shape, not features. What's left splits cleanly.

## Real and owed: the doc pass

This is the one thing I'd actually do, and it's overdue. Since the last commit I've added six corpus cases and a core check, and `§5.1`'s table doesn't mention any of them. Specifically stale right now:

- **§6 still says `RUST_MODULE` is "reserved by comment and enforced nowhere."** That's false as of an hour ago.
- **§5.1 says the corpus is 46.** It's 52, with rows missing for both cross-crate re-exports, the compiled-crate scoping case, and the reserved-module pair.
- **Case 48's row** describes sixteen mechanisms and one drop shape; it's now eighteen and two.
- **Nothing records that the walk excludes the compiled crate** — the fact that cost me a cycle today.
- **§5.3 step 3** (cross-crate re-exports) is done and still listed as next.

I've spent this whole session finding places where the docs claimed something that stopped being true. Adding six cases and not writing them down is how the next one of those gets made.

## Not worth starting

**§9 step 3** — ours and unblocked, but nothing feeds rustc back, so it lands untested against its real consumer. Same answer as last time.

**Two deadweight items** — whether `EqualsSR` is still used in `declarations.rs` after today's change, and the now-callerless `compiler_test_compilation_with_rust_oracle`. Both are small and belong in a cleanup pass, and the second is core.

## The thing I'd actually raise

**The pile is large and unbanked** — 10 changed files now, including a core change, six cases, a harness refactor, and the declaration-shape change. It's all verified green, but it's a lot of independent work resting on one uncommitted state, and today already showed what happens when upstream moves underneath us.

So: doc pass, then I'd want to commit before starting anything else. After that the board really is the three decisions — eagerness, `Vec<int>()`, and whether the diagnostics bucket stays deferred.

---

## 👤 User

do doc pass

---

## 🤖 Claude

Measuring the corpus rather than counting from memory.

---

## 🤖 Claude

50 cases: 46 previously implemented plus the 4 new, so rows 49–52. Reading the table's tail to place them.

---

## 🤖 Claude

Now the edits. Row 46's "cross-crate is untested" is stale, 48's description is stale, and rows 49–52 are missing.

---

## 🤖 Claude

Now §6's `RUST_MODULE` bullet, which the check just made false.

---

## 🤖 Claude

Now §4 gains the fact that cost me a cycle today, and the parameter-rune shape it no longer needs.

---

## 🤖 Claude

Now sweeping for anything else the day made stale.

---

## 🤖 Claude

Adding the sharper invariant to §2 — the one that actually caught the regression today.

---

## 🤖 Claude

Doc pass done. **661/141/8** unchanged — docs only, so unchanged by construction and by measurement.

## What was stale, and is now not

**The headline claim.** §0 said "33 corpus cases" and described a capability list that predates nested modules, re-exports and generic drop. It now describes what the compiler actually does, including that case 48 composes it rather than only case-by-case coverage.

**The corpus table.** Said 46 implemented; it's 50. Rows 49–52 added for the two cross-crate re-exports, the compiled-crate exclusion, and the reserved-module case. Row 46's "cross-crate is untested" and row 48's "sixteen mechanisms" were both false.

**§6's `RUST_MODULE` bullet** said "enforced nowhere." It's now enforced, and the bullet says where, why there specifically, and that the `panic!` is the right tier but the wrong rendering.

**§5.3's step 3** was cross-crate re-exports, which are done — deleted, leaving tier 2 as the only remaining step.

**§2's core-footprint claim** said the only gated core code was the store-pushing loop. There are two things in that block now.

## Two additions that earn their place

**The walk sees dependency crates, never the compiled one** (§4). This is the fact that cost me a cycle today: a fixture placed in the stub fails *identically* to a broken walk — zero candidates, `importable_*` empty. Now recorded with the consequence, that posing a real cross-crate question takes two dependency crates.

**The failure-count invariant** (§2). The docs said to compare suite counts against a remembered number. The sharper check is that the two configurations' *failure* counts match — one extra failure under `rust_interop` means exactly one corpus case is red, with no memory required. That's what surfaced case 32 today, and it wasn't written down.

Swept for annotation residue — no strikethroughs, date stamps, or correction blocks introduced.

## Where that leaves us

The pile is now 11 files: a core check, six cases, the harness refactor, the declaration-shape change, two fixture files, and this doc pass. All green, all unbanked.

Nothing else is unblocked that I'd recommend starting. The next move is committing, and after that it's your three decisions — eagerness, `Vec<int>()`, and whether the diagnostics bucket stays deferred.

---

## 👤 User

what decisions?

---

## 🤖 Claude

Three, and they're the whole remaining board.

## 1. Eagerness

Every allowed item is resolved, declared **and compiled**, whether the program mentions it or not. Invisible at a five-name allowlist; `Vec` alone brings ~100 inherent methods, each fully compiled for a program that calls two.

The split decides who does it: the per-type method fan-out is ours, but the expensive half is the loop in `Compiler::evaluate` that compiles every top-level store — lazy population means changing that loop, which is core. rustc's own answer is `populate_on_access`: fill on first touch.

One thing already ruled out: don't filter by scanning Vale source for names — an item can be reached without its name appearing (a synthesized drop, a method through a generic instantiation), so it would drop declarations silently.

**Needed from you:** whether to make the compile-everything loop lazy.

## 2. What `Vec<int>()` means

A bare call is a *call*, and for a Vale struct the callee is the macro-derived field constructor — which only exists for parsed denizens, so a Rust-backed type has none. That's correct and should stay correct: `Vec`'s fields are private, and synthesizing a field constructor would claim a layout we don't have.

So either `Vec<int>()` constructs and the compiler routes it to `Vec::new`, or Vale source writes `Vec<int>::new()` and the bare-call form keeps meaning "Vale struct literal" only.

**My lean:** the second. It refuses a special case rather than adding one, and keeps "bare call = field constructor" a single rule.

## 3. Whether diagnostics stay deferred

You parked this earlier. Two items, both core, both about the compiler telling the truth about what it knows:

- **`TooManyTypesWithNameT`** — a bare ambiguous name written in Vale still panics. The error variant exists, is produced by nothing, and its humanizer is a `panic!`.
- **The declined-item side table** — `lower_ty` already computes *why* a signature declined, and drops it. The failure says "couldn't find function `first`" about a function that plainly exists.

**Needed from you:** whether these stay parked. They're the only substantial work left that needs no language ruling.

---

**1 and 2 are the `Vec` agenda** — take them together and §9 step 3 stops being premature, since it finally has a consumer. **3 is independent** and is what stands between a compiler that works and one that explains itself.

---

## 🤖 Claude

Watcher re-armed.

Nothing else has changed. The tree is still uncommitted at 661/141/8 interop and 608/141/8 default, and the three decisions from my last message are open.

---

## 🤖 Claude

Watcher re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 👤 User

im thinking we aren't that far from being able to lazily compile things. i'm thinking maybe:
- in the environment / global env, a hierarchy of names, named globalEnv, according to whats imported. or however the global envs are structured today.
- in the coutputs, scoutputCache, a lazily populated hash map of name to scoutput? is a cache. map of name to option<definition templata scoutput>. 
- when we want something vale-defined, we look in globalEnv to see if it exists and to assemble its name, then we lazily turn its parseds into scoutput and cache it and return it.
- when we want to see something rust-defined, we look in globalEnv to see if it exists and to assemble its name, then we call into the rust plugin to make some scoutput for it.

---

## 🤖 Claude

Let me ground a couple of things before responding — especially what already exists for this.

---

## 🤖 Claude

I think the shape is right, and there's a stronger argument for it than the sketch makes. Also: **`tree_shaking_enabled` already exists on `TypingPassOptions`, is set `true` at all seven construction sites, and is read nowhere.** The flag was anticipated and the mechanism never built — this design is what fills it, or it should go.

## Why the symmetry is the good part

Today Vale is *"parse everything eagerly"* and Rust is *"synthesize everything eagerly."* Two different eagernesses that happen to meet in the same store. Your sketch makes both **"a name that can produce a declaration on demand"** — which is §1's *synthesized is the degenerate case of parsed*, moved up a layer. The interop path stops being a special case at the level where it's still special.

That's the thing I'd hold onto if the design gets pressure later.

## Three refinements

**The index stays eager; only the payload goes lazy.** Lookups are by imprecise name and return possibly-many entries, so you can't lazily populate the index — you wouldn't know whether a name exists. Fortunately that's exactly where the cheap/expensive split falls on the Rust side: `importable_functions()`/`importable_types()` are cheap enumeration, `fn_sig` is the expensive part. So "name → thunk" costs no oracle work beyond what we already do.

**`Option<...>` gives us the declined-item side table for free.** You wrote `Option<definition templata scoutput>`. If the `None` carries the `DeclineReason` that `lower_ty` already computes and currently drops, then the cache *is* the side table §5.3 step 1 wants — the lookup that fails to find `first` consults the same map that would have produced it, and says *"found `first`, but its return type has no Vale form."* One of the three open decisions dissolves into this one rather than needing its own core change.

**The static filter bounds the win, and it's worth knowing the shape.** Vale2's candidate filter reads arity, wrap chain and value-type template name *off the declaration*. So resolving a call to `get` forces synthesis of every `get` in scope, not just the winner. Laziness is keyed on *name mentioned*, and colliders come along. Still a large win — a program mentions two or three of `Vec`'s hundred methods — but the granularity is per-name, not per-item.

## The hard part: the driver, and here Vale and Rust genuinely differ

An unreferenced **Vale** function must still be typechecked — a developer expects errors in code they wrote. An unreferenced **Rust** method must not be. So the *cache* is shared but the *driver* can't be: Vale denizens stay force-compiled by the existing loop over `name_to_top_level_environment`; Rust denizens are pulled by reference.

I don't think that's a wart. "Typecheck everything I wrote, nothing I imported" is the correct rule and it's rustc's rule too. But it means the honest description is **a shared memo table with two drivers**, not "lazy compilation" — worth being precise about, because the sketch reads as though laziness applies to both.

## The risk

**Determinism, and it's the P0 shape.** Lazy compilation makes compile order depend on *reference* order rather than declaration order. This tree already has scars there: the compile loop has an explicit sort for `_`-prefixed names with an `AFTERM` asking whether everything should be ordered, and `FileCoordinateMap` is an `IndexMap` rather than a `HashMap` specifically to keep that loop @IIIOZ-compliant. A memo table is fine if it's insertion-ordered and nothing depends on *when* something was compiled — but "which error is reported first" would change, and some solver tests assert humanized text.

## What I'd check first, before any of it

Whether `precompile_struct`/`compile_struct` tolerate running out of declaration order — a citizen's methods reference the citizen. It's probably fine, since that's what the solver is for, but it's the thing that would bite, and it's a cheap probe rather than an argument.

Want me to write this up as a plan? It's core-heavy, so it'd need your sign-off on the hunks either way — but the two-driver split and the `Option`-carries-the-reason unification are the two decisions I'd want settled before anything gets built.

---

## 👤 User

agreed, dont lazily populate index. sg None carries DeclineReason. also, we dont need to compile every get in scope, vcoord-handoff.md describes how that works. i dont see the problem that you see wrt compiling all vale things and not rust things. not sure what you mean about two drivers. yes it needs to be absolutely deterministic.

please send out at least five agents to investigate this approach

---

## 🤖 Claude

Two corrections taken:

**You're right about `get`.** I was reasoning from today's ambient union, not the ratified model — candidates come from the namespaces of the argument types, and name-uniqueness is per namespace, so a call with a `Counter` argument never sees `Gauge`'s `get`. That collapses my "colliders come along" concern to almost nothing.

**And "two drivers" was me inventing a distinction.** It's one loop over `name_to_top_level_environment` that simply doesn't force rust-backed stores. A loop with a scope, not a second mechanism. Withdrawn.

Sending six investigations.

---

## 🤖 Claude

Six agents out, all read-only and instructed not to touch the tree:

| # | chasing |
|---|---|
| 1 | **What forces compilation today** — the precompile/compile loops, whether an entry-point set exists to pull from instead, and what `tree_shaking_enabled` was meant to do (it's read nowhere, so git history should say) |
| 2 | **Determinism audit** — @IIIOZ, every HashMap-vs-IndexMap choice that can reach output, error ordering, the write-once instantiation-bounds assert, and what the `_`-prefix sort is compensating for |
| 3 | **The cache key and the forcing seam** — which name vocabulary keys what, what `add_entries` builds, and the blast radius of making an `IEnvEntryT` payload lazy |
| 4 | **Oracle cost profile** — proving enumeration is actually cheap and synthesis is the cost, and where a *decline* is discovered, since that decides whether a name can enter the index before we know it's representable |
| 5 | **What rustc does** — `populate_on_access`, how the query system stays order-independent, how diagnostics stay deterministic under lazy evaluation, and whether it has an "exists but unrepresentable, here's why" shape |
| 6 | **What your dispatch model requires** — reading vcoord-handoff in full for what must exist at the moment a call resolves, so the lazy scheme doesn't break the static filter or bounds resolution |

Agent 4's question about **where a decline is discovered** is the one I think could reshape the design. If `lower_ty` only declines during synthesis, then a name enters the index before we know whether it's representable — which is fine for the `Option<...>` cache, but it means "in the index" and "importable" stop being the same thing, and the empty-allowlist and not-in-allowlist cases (27 and 28) are the ones that would notice.

I'll report when they land.

---

## 🤖 Claude

First agent back. Let me verify its sharpest claim before relaying it — it reports a silent decline path with no reason attached.

---

## 🤖 Claude

Two back, four still running. Both found things worth acting on.

## A verified defect, found in passing

`vale_type_name` names only `Int` at 32 bits, `Bool`, and `Void` — I checked it directly. But `lower_ty` accepts `i64` and lowers it to `KindT::Int(IntT::I64)`, and **`i64` is a Vale builtin primitive** (it's in `Compiler::evaluate`'s primitives list alongside `int`).

So a Rust function taking or returning `i64` passes the decline check, then gets **silently dropped** at synthesis when `bind_sig_type` returns `None` — with no `DeclineReason` recorded anywhere. That's a second decline gate that carries no reason, which is exactly the shape the `Option<...>`-carries-the-reason design is meant to eliminate. It's also a live gap: `i64` is representable and we drop it.

## The design's core assumption holds

**Only `fn_sig` drives a rustc query at trait-call time.** Everything else — `importable_types`, `importable_functions`, `methods`, `item_package`, `type_generic_params` — reads a table built once in `TyCtxtOracle::new`. So a complete name index costs *zero* rustc queries beyond what we already pay.

And the split is where we hoped: for a `Vec`-sized type, today's cost is ~100 `generics_of` at enumeration **plus ~100 `fn_sig` reads and ~100 `FunctionS` syntheses** in the importer loop. Laziness moves the second hundred out entirely.

Confirmed too: **declining is discovered strictly at synthesis**, so the index is necessarily a *superset* — "exists and is allowlisted", not "is importable".

## The obstacle, and it's narrower than it first looks

`add_entries` computes a store's imprecise keys **by inspecting the entry's payload** — for `Impl` it reads the `ImplS`'s sub/super names, for `Isa` it digs `KindT`s out of the templata, for `Prototype` it reads the prototype's own `IdT`. You cannot build that index over unforced declarations.

**But that only bites Vale denizens.** Rust items are `Function` and `Struct` entries, and those take the fallthrough arm where the imprecise key comes from the *name*. So the interop half of this is unobstructed; the Vale half is where it's real.

## The seam is clean, with one catch

Forcing funnels through exactly **two functions** — `lookup_with_name_inner` and `lookup_with_imprecise_name_inner` on `TemplatasStoreT`. No public lookup returns an `IEnvEntryT` at all; they all return templatas.

The catch: `entry_matches_filter` runs *before* conversion and matches on the entry's **variant**. So filtering forces unless the thunk exposes its kind unforced. That argues for the entry carrying its discriminant eagerly and only its payload lazily — which also happens to be all `entry_matches_filter` needs.

Two other constraints worth having now: `templatas()` hands out the raw store and **13 sites walk it end-to-end** (each would force everything), and stores escape into `HinputsT` and four `CompilerOutputs` maps via environments — so an unforced entry could be observed arbitrarily late, after typing is nominally done.

**Blast radius if `IEnvEntryT` itself becomes lazy: 17 destructure sites, all in core, none in `rust_interop/`.** Plus 38 construction sites.

Still out: the compile-loop/forcing investigation, the determinism audit, rustc's approach, and what your dispatch model requires.

---

## 🤖 Claude

Third back — the dispatch one, and it sharpens two things.

## The filter forces a *second* declaration

This is the finding that most changes the design. The filter isn't purely local to a candidate:

> "The chain a candidate is filtered on is *written wraps ++ the position rule applied given the citizen's kind*, and the second half is a **lookup, not a solve**… from the name you have the definition, and the definition carries sharedness."

So filtering candidate `foo(x MyClass)` requires looking up **`MyClass`'s definition** to learn its sharedness. Forcing one declaration can force another. Not a cycle, but a cascade, and it means the reachable set is "names mentioned, plus the citizens their parameters name."

For Rust items that's harmless — a Rust citizen is always `Single`. For Vale it's real, and it's the mechanism by which laziness could quietly force more than you'd expect.

## Rune identity must be a pure function of the name

> "The instantiator asserts the params and args maps have equal length and then zips them **by rune**, so definition and call site **must agree on rune identity across two separate solves**."

Under laziness the same declaration must be produced *identically* every time, regardless of what was produced before it. Memoization gives that — but only if the producer doesn't depend on hidden state.

We already satisfy this by accident that became deliberate: `next_synthetic` in `synthesize_extern_function` is **function-scoped**, so each declaration numbers its synthetic runes from zero. I made it function-scoped this morning to avoid two parameters minting the same name; it turns out to also be what makes the declaration order-independent. Worth writing down as a real invariant rather than a comment about parameters.

## The shape fits what's already there

> "**No cache is needed because rune-typing does not recurse across denizens: a denizen referencing another reads that one's *declared signature* off the postparse AST rather than solving it.**"

Reading another denizen's *declaration* is already the established cross-denizen dependency — which is exactly what the cache would produce. The design isn't introducing a new kind of edge, it's memoizing one that exists.

## Two constraints to respect rather than discover

**Namespace search order is load-bearing semantics**, not an implementation detail — it's the only mechanism keeping a compiler-synthesized claim clone apart from a user's `clone`. A lazy scheme must not perturb the order namespaces are searched in.

**There's a hard timing window** on the `implements` mint: "conjure after `interpret_results` and before whichever statement seeds the environment. Miss it and the entry is never indexed… and **nothing reports it**." That's the failure mode to fear — silent, not loud.

## And the handoff is silent on the thing itself

It says nothing about lazy or memoized declaration production, and nothing about what happens if a candidate's declaration isn't built at filter time. The only gesture toward it is: *"Layers are the incrementality mechanism… a pass boundary is where a hash can be compared and downstream work skipped."*

So this design isn't contradicted upstream — it's unconsidered there, which means it's ours to get right and worth telling them about.

Still out: the compile-loop/forcing investigation, the determinism audit, and rustc's approach.

---

## 🤖 Claude

Fourth back, and it reframes the whole thing.

## The instantiator is *already* pull-based

Its worklist is seeded only from `kind_exports`, `function_exports`, and non-generic `function_externs`. `hinputs.functions/structs/interfaces` are bound to `_`-prefixed unused names at the seeding point and serve purely as lookup tables afterward.

**So a typed-but-unreferenced denizen is already discarded between typing and instantiation.** Typing compiles everything and the next pass throws most of it away. Making typing pull-based doesn't introduce a new model — it removes a mismatch between two passes that already disagree.

## Your precompile/compile split is the seam, and it answers the cascade

The "indexing phase" exists *precisely* so compile order doesn't matter — `declare_type`, `declare_type_sharedness`, `declare_type_sealed`, `declare_type_outer_env`, each enforced by assertions in the compile phase. Its own comment: *"we might compile a virtual function somewhere before we compile the interface. The virtual function will need to know if the type is sealed."*

That resolves the sharedness cascade I flagged an hour ago. The filter needs a citizen's sharedness — but sharedness is declared in **precompile**, which is cheap and eager. So forcing a candidate doesn't force another denizen's *compilation*, only its *declaration*. **Precompile stays eager; compile goes lazy.** The seam already exists.

## `tree_shaking_enabled` has a history, and this design is better than what it was

It was live in Scala at one point: `Reachability.findReachables`, roots = `main` + exported functions + exported kinds, DFS over calls. Disabled during the templates→generics transition; the Rust port inherited a flag that was already dead upstream, and `reachability.rs` is a signature-only skeleton of `panic!("Unimplemented: Slab 15")` called from nowhere.

Crucially it was a **post-hoc filter on already-compiled output** — compile everything, then discard. Your design never compiles the unreachable thing at all. Same roots, strictly less work.

## Two real obstacles

**Vtable construction scans everything.** `make_interface_edge_blueprints` finds abstract functions by scanning `get_all_functions()` and *asserts* every interface's declared internal methods are present. Interface dispatch layout is derived from "all compiled functions," so anything interface-related can't go lazy without changing that.

**173 test call sites** — `lookup_function_by_str`, `lookup_struct_by_str`, `get_all_user_functions` — panic if a named denizen isn't in `HinputsT`. That's the largest consumer of compile-everything, and it's entirely test-side. A pull-based typing pass would break tests that look up denizens their program never calls.

That's not a design flaw; it's the migration cost, and it's the number I'd want you to see before committing to this.

## Smaller, but worth knowing

The `_`-prefix sort in the compile loop is a **test hook**, not a correctness mechanism — one integration test scrambles three struct names to permute compile order and assert every permutation compiles. And `use_overload_index` is in the identical dead state as `tree_shaking_enabled`: set true everywhere, read nowhere.

Two still out: the determinism audit and rustc's approach.

---

## 🤖 Claude

Fifth back — the determinism audit, and it's the sobering one.

## The headline: compile order *is* the error you see

`Compiler::evaluate` returns `Result<HinputsT, ICompileErrorT>` — **one** error, propagated with `?` per denizen. There is no accumulator anywhere in the pass; the first denizen that fails aborts it, and every later denizen's errors never come into existence.

So reference-order compilation doesn't just reorder diagnostics, it **reselects** them. 51 byte-exact `assert_humanized_eq` sites sit downstream of that. The agent was careful to say it can't tell which of the 51 would flip without actually running each program in a reversed order — that's an experiment, not a grep, and I'd want it before anyone writes code.

## Two more that are behaviour, not bytes

**vtable slots are assigned in compile order.** `edge_compiler` derives each interface's abstract-method list by scanning `get_all_functions()`, and appends any abstract function *not* already in `internal_methods` in that order. Its own comment says vivem depends on method order. Different compile order → different vtable indices → different runtime behaviour.

**`add_instantiation_bounds`' assert is order-sensitive, and its input is laundered through a `HashMap`.** The equality is zip-based — *"same entries in same insertion order"* — but the map it compares is built from `rune_to_prototype`, which is a `HashMap`. So two paths reaching the same instantiation with the same bounds in a different rune order **panic**, with a message naming neither the instantiation nor the diff. That's precisely the lazy-compilation scenario: same instantiation, different first-referencer.

## And the current state is weaker than I'd assumed

`IdT::hash` hashes **addresses** (`ptr::hash`), so every `HashMap<IdT, _>` in the typing pass iterates in ASLR-dependent order, varying per run. That's fine only while nothing iterates them into output — and there are already three live @IIIOZ violations doing exactly that, independent of any of this: the overload resolver's candidate list, the final AST's per-package definition lists, and two self-declared `HashMap`s in `infer_compiler`.

## What I take from it

The design is sound and the seam is real. But **the determinism prerequisite is larger than the design**. In rough order:

1. The single-error model has to become an accumulator, or we accept that error selection moves — and that's a user-facing change, not just a test one.
2. `rune_to_prototype` needs to stop being a `HashMap` before anything reaches `add_instantiation_bounds` from two directions.
3. Vtable construction needs a canonical order that isn't "whatever got compiled first."

Item 1 is the one I'd put in front of you, because it's a language-behaviour question rather than an implementation detail: *should the compiler report the first error in declaration order, or accumulate?* Today's answer is an accident of `?`.

Last agent — rustc's approach — is still out, and it's the one most likely to have an answer for 1 and 3, since rustc emits diagnostics lazily and deterministically.

---

## 🤖 Claude

All six back. Here's the consolidated read.

## rustc does exactly this, and the Vale/Rust split you didn't see a problem with is *one field*

`populate_on_access` is still called that. It's a `bool` on each module, initialized as:

```rust
populate_on_access: CacheCell::new(is_foreign)
```

**Local modules are never lazily populated; foreign ones always are.** That's the whole distinction — not two mechanisms, one flag whose value is "is this mine." You were right and my "two drivers" framing was noise; this is the shape it actually takes.

The trigger is three lines behind a single funnel — every read path goes through one `resolutions()` accessor. And it's a **two-level memo**: the query cache holds the decoded child list keyed by `DefId`, and the per-module bool holds "I already turned that list into bindings." The second level is hand-rolled, outside the query system entirely.

## The single most transferable idea

> "This path is unique since we're in a query so we'll only be called once with `parent_def_id`" — and the call passes a **fresh** `DisambiguatorState::new()` rather than a shared counter.

**A lazily-created name must be derivable from its key alone.** rustc does this at every lazy def-creation site. We already satisfy it: `next_synthetic` is function-scoped, so a declaration's synthetic runes are numbered from zero within itself regardless of what was built before. That was a fix for parameter naming this morning; it's now the load-bearing invariant that makes memoization safe.

And the counterexample is right there too — `rustc_public`'s `create_or_fetch` mints public ids from **insertion index into a lazily populated map**. Touch items in a different order and every id changes. rustc gets away with it only because those ids never reach a build artifact. That's the exact trap if our cache ever mints an identifier on first reference.

## It answers the error-ordering problem, and not the way I expected

rustc has **no global diagnostic sort**. What it has is a **fixed eager driver walking an eagerly-built, source-ordered index, with the queries lazy underneath**. `check_crate` is a literal script; the index it walks is built by an in-order HIR visitor. Laziness is in the *work*, not in the *traversal*.

That maps straight onto us: our index is already eager and source-ordered, and the compile loop already walks it. Keep that loop as the driver for local denizens and the error order doesn't move — only imported declarations become demand-driven, and their errors surface where the user's attention already is. My "we need an error accumulator" was premature; we need the *driver* to stay index-ordered.

Where a pass genuinely can't emit in source order, rustc buffers and sorts on a first-class per-diagnostic `sort_span` field — one line at the flush point. Worth stealing if we ever need it.

## Our design beats rustc's in one place

`Res::Err` children are **filtered out of `module_children`** before reaching metadata, so a downstream crate cannot distinguish "never existed" from "existed but failed to resolve." rustc pays for that in worse cross-crate diagnostics. That's precisely the collapse `Option<...>`-carries-`DeclineReason` avoids.

And where rustc *does* carry a reason — `rustc_public`'s `Error(String)` and `Opaque(String)` — it stops at a string. A structured enum is strictly better; there's nothing to imitate verbatim, only confirmation the slot should exist.

## The one thing I'd take independent of all of this

`UnordMap`/`UnordSet` plus the `POTENTIAL_QUERY_INSTABILITY` lint: collection types that **don't expose iteration**, so "iterated a hash map" becomes a compile-time-visible event needing an `#[allow]` with justification.

Given the determinism audit found live @IIIOZ violations and that `IdT::hash` hashes *addresses* — so every `HashMap<IdT, _>` iterates in ASLR order — this would find them mechanically. And note it's **rule 1, not rule 4**: an unordered type makes the violation unrepresentable rather than detected, which is the ordering you've been pushing for all session.

Two smaller ones: prefer an explicit `NotStarted`/`InProgress`/`Done` over rustc's clear-the-flag-then-fill, which silently serves a half-filled module on re-entry. And don't build cycle recovery — rustc's own conclusion is that cycles are an irrecoverable error, and its recovery machinery reconstructs plausible signatures with `transmute`.

---

## What's actually left to decide

The design survives contact. What it needs from you, in order:

1. **Vtable construction** derives abstract-method lists — and slot indices — from `get_all_functions()`, i.e. compile order, with an assert on completeness. That needs a canonical order before anything interface-related goes lazy.
2. **`add_instantiation_bounds`' order-sensitive assert**, whose input is currently laundered through a `HashMap`. Two paths reaching one instantiation panic rather than differ.
3. **173 test call sites** that look up denizens their program never calls. That's the migration cost, and it's the number I'd want you to see before committing.

None of those is a reason not to do it. All three are cheaper than they'd be after the fact.

---

## 🤖 Claude

Re-armed.

The six investigations are all in and summarized in my previous message. Nothing else has changed — the tree is still uncommitted at 661/141/8 interop and 608/141/8 default.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 🤖 Claude

Re-armed.

---

## 👤 User

you are over complicating this i think. we only need the ingredients i said. just lazily populate scoutput into a scoutput-by-name cache, and the rest falls out of that design

---

## 🤖 Claude

You're right, and I can name the specific mistake: **I conflated scoutput with compiled output.**

Your cache is `name → Option<scoutput>` — the *declaration*. I spent the last several messages reasoning about lazily *compiling* denizens, which is a different and much larger change. Once it's only the declaration that's lazy, the compile loop keeps walking the eager index in order and most of what I raised evaporates: error ordering unchanged, vtable slot assignment unchanged, the 173 test lookups unchanged, `add_instantiation_bounds` unchanged.

What actually survives from the investigations is small and all confirming:

- **The cost split is where it needs to be.** Only `fn_sig` drives a rustc query at call time; enumeration is a scan of a table already built. So a name index is free, and a cache miss is exactly one `fn_sig` plus one `FunctionS` — which is the thing that currently runs ~100 times for a `Vec`-sized type and would run three.
- **The producer must be a pure function of the name.** rustc hands a fresh disambiguator at every lazy def-creation site for exactly this reason. We already satisfy it — `next_synthetic` is scoped per declaration — but it's now load-bearing rather than incidental.
- **`None` carries the reason**, as you said. And that subsumes the declined-item side table, so one of the three open decisions folds into this.

Plus the one real defect the sweep turned up, independent of any of it: `vale_type_name` names only `int`/`bool`/`void`, so a Rust `i64` passes the decline check and is then dropped silently with no reason recorded — even though `i64` is a Vale primitive.

Want me to write it up as a plan, or just build it?

---

## 👤 User

what are all the changes needed in the core compiler? i would assume just changing how the scoutput is stored/cached, and changing the globalEnv a bit

---

## 🤖 Claude

Roughly yes, and it's smaller than my last few messages implied. Concretely, in core:

## `compiler_outputs.rs` — the cache

A field plus an accessor. Insertion-ordered rather than a `HashMap`, since `IdT`'s `Hash` hashes *addresses* and every existing `HashMap<IdT, _>` iterates in ASLR order. Producing on miss needs a handle to both producers — the oracle (already on `Compiler`) and the parsed AST.

## `compiler.rs` — the parsed AST has to outlive the loop

Today `file_to_program_s` is a local in `evaluate`, consumed to build `id_and_env_entry` and then dropped. The Vale-side producer needs it at cache-miss time, so it has to live somewhere reachable. That's the one change I'd call structural rather than mechanical.

## `environment.rs` — the store's value type, and the two funnels

`TemplatasStoreT.name_to_entry` holds the payload today. It becomes kind-plus-key, and the payload arrives from the cache at lookup. Two functions force — `lookup_with_name_inner` and `lookup_with_imprecise_name_inner` — and everything funnels through them; no public lookup hands out an entry.

One catch worth designing around rather than discovering: `entry_matches_filter` runs *before* conversion and matches on the entry's variant. So the store entry must carry its discriminant unforced. That's satisfied by keeping the kind in the entry and only the declaration behind the key.

## `compiler.rs` — splitting the two uses of `name_to_top_level_environment`

It serves as both the **lookup index** and the **compile driver** today. Rust stores need to be in the first and not the second. That's your "changing the globalEnv a bit", and it's the whole of it.

---

## The scope decision that keeps this small

**Only `Function`, `Struct` and `Interface` go lazy. `Impl` and `Templata` stay eager.**

That matters because `add_entries` computes a store's imprecise keys by *reading the payload* for exactly three shapes — `Impl` reads the `ImplS`'s sub/super names, `Isa` digs `KindT`s out of the templata, `Prototype` reads the prototype's own `IdT`. Function, struct and interface entries take the fallthrough arm where the key comes from the name.

So if those three stay eager, **`add_entries` is untouched** — which removes the one part of this I'd otherwise call risky. And it costs nothing: there are no Rust impls, and templata entries are already values rather than scoutput.

## What's ours, not core

`rust_package_stores` stops synthesizing and builds a name index instead; the synthesis moves behind the cache's producer; `DeclineReason` lands in the `None`.

## Two things I'd check before promising a size

Whether `IEnvEntryT` itself needs to change or only the store's value type — I think the latter, which leaves 38 construction sites alone, but I haven't walked all 17 destructure sites to confirm. And the 13 places that iterate a whole store end-to-end: for Vale stores forcing everything is correct, but I'd want to confirm none of them run over Rust stores.

Both are reading, not design. Want me to settle them and write the plan?

---

## 👤 User

you sure impl needs to be eager? pretty sure we don't need to do a full scout on it

---

## 🤖 Claude

Let me check what it actually reads rather than assume.

---

## 🤖 Claude

You're right, and the distinction I missed is worth stating.

Of the three payload-reading arms in `add_entries`, **two aren't declarations at all.** `Isa` reads `isa.sub_kind`/`super_kind` — `KindT`s, i.e. already-typed values — and `Prototype` reads a resolved prototype's `IdT`. Both are `ITemplataT` entries added *during* compilation, not parsed denizens. They were never candidates for laziness; I lumped them in because they sat in the same match.

The only arm that reads a *declaration* is `Impl`, and it reads exactly two fields:

```rust
let sub = impl_a.sub_citizen_imprecise_name;
let sup = impl_a.super_interface_imprecise_name;
```

Those are imprecise names — what the source text names as sub and super. No scouting required to know them; they come off the parsed impl. So the index carries those two names per impl and keys the three `Impl*ImpreciseName` entries from them, exactly the same shape as every other entry keying off its name. The `ImplS` itself stays behind the cache.

So: **all four declaration kinds go lazy** — function, struct, interface, impl — and `add_entries` needs the impl's two names threaded into the index rather than read off a payload. That's a smaller change than the "leave impls eager" carve-out I proposed, and it doesn't leave a special case behind.

Which collapses the core list to four things:

- `compiler_outputs.rs` — the cache and its accessor
- `compiler.rs` — the parsed AST outliving the loop, so the Vale-side producer can reach it
- `environment.rs` — the store's entry becomes kind-plus-key (plus the impl's two names), and the two lookup funnels force
- `compiler.rs` — splitting `name_to_top_level_environment`'s two jobs, lookup index versus compile driver

That's the whole of it, and `add_entries` keeps its shape rather than gaining a carve-out.

---

## 👤 User

<command-message>close-session</command-message>
<command-name>/close-session</command-name>

---

## 🤖 Claude

Wrapping up docs before closing. Two things from this session aren't recorded yet — the lazy-declaration design and the `i64` defect.

---

## 🤖 Claude

Docs current, suite unchanged at **661/141/8** interop and **608/141/8** default. Not committing — you said no commit, and only "fire commit" authorizes it.

## Session state

Landed and committed: **`90109322f`**, ratcheted to `experimental` — a synthesized parameter's type rules moved into its own `value_type_rules`, plus case 32 retargeted at excess arity.

**Uncommitted, all verified green** — 9 files. Six new corpus cases (39, 41, and 49–52), the harness's package-aware refactor, the `bind_sig_type` return-the-rune change, the reserved-`rust`-module check in `Compiler::evaluate`, two fixture files, and the plan doc.

Corpus went 46 → 52. Four rows that said *blocked* turned out to need nothing, which is now written into §5.1 as a rule rather than four separate corrections.

**Just recorded before closing:** the lazy-declaration design in §5.3 step 2 — the cache shape, the four core changes, and that `add_entries` keeps its shape because only `Impl` reads a declaration and its two names come off the parsed impl. Plus the `i64` defect in §6: it lowers fine, is a Vale primitive, and gets dropped silently because `vale_type_name` has no arm for it.

## Two things waiting for you

The **`Vec` decisions** — eagerness and whether `Vec<int>()` constructs — and whether the diagnostics bucket stays parked.

And the pile is large. It's green in both configs and the core diff is four lines plus an import, but it's a lot of independent work resting on one uncommitted state, and `experimental` moved under us twice today.

⚠️ Guardian and Luz both moved on disk earlier — this directory's Guardian still wants a restart.

---

## 🤖 Claude

Target confirmed. Closing.

---

