<!-- session: 78c8a72e-277d-4a37-9fcb-494ea6ae7c04; exported-bytes: 5676912; transcript: /Users/verdagon/.claude/projects/-Volumes-V-Vale4/78c8a72e-277d-4a37-9fcb-494ea6ae7c04.jsonl; opened: convo-9-generics-seam-collapse-and-test-tiers.md -->
# Claude Conversation Log

Session ID: 78c8a72e-277d-4a37-9fcb-494ea6ae7c04
Date: 2026-07-26 19:06:43

---

## 👤 User

<command-message>use-mailbox</command-message>
<command-name>/use-mailbox</command-name>
<command-args>then please /full-read /tmp/claude-conversation-2026-07-25-43fdb10f.md and docs/convos/rust_interop/convo-9-generics-seam-collapse-and-test-tiers.md and then the architecture doc and then the handoff doc please</command-args>

---

## 🤖 Claude

I'll arm a mailbox watcher first, then do the full reads.

---

## 🤖 Claude

The handoff doc is `synthesized-declarations-plan.md`. Let me load the full-read skill.

---

## 🤖 Claude

File 1 complete (4,642 lines). Now file 2.

---

## 🤖 Claude

File 2 complete (4,430 lines). Now the architecture doc.

---

## 🤖 Claude

Architecture doc complete (3,649 lines). Now the handoff doc.

---

## 🤖 Claude

Watcher armed as **Vale4-delta**. All four read in full — 4,642 + 4,430 + 3,649 + 261 lines.

## Where the arc stands

**The design landed, twice over.** Convo-8 built the per-call-site oracle to a real `TyCtxt`, then discovered mid-session that it was structurally wrong — a finished `PrototypeT` minted at environment-build time from `fn_sig(item, &[])` cannot represent `fn pick<A, B>(a: A, b: B) -> A`. Two arcana already forbade it (@ECSIIOSZ, @BDPFWDZ), and both sibling repos had tried and abandoned the same shape; one still carries the corpse (`ExternFunctionTemplataT`, zero producers, `vfail()`).

**Convo-9 rebuilt it and it works.** `extern` is a *body kind*, not a denizen kind. A Rust item is an ordinary synthesized `FunctionS` + `IBodyS::ExternBody` in the reserved `rust` package's top-level store; the function-compile phase picks it up, the solver resolves its rules, `make_extern_function` mints the prototype and registers bounds per instantiation. Generics needed **zero** changes outside `rust_interop/` — `pick<int, bool>` typechecks against real rustc, and `first<I: Iterator> -> I::Item` is declined rather than imported.

**The seam collapsed to nothing.** Free functions, methods, and drop are one code path producing one entry kind — because Vale erases method syntax in the postparser anyway. `overload_resolver.rs` has zero interop content, and after your `add_struct` suggestion the **core diff is empty**: the Rust type gets a real `StructDefinitionT` with zero members and an `Extern` attribute, so `ensure_deep_exports` reads the attribute like any hand-written extern struct rather than needing a `is_rust_backed` guard.

## Four things I'd flag as live

1. **Tier 1 testing is settled but not built.** The experiment proved `run_compiler` works from a `#[test]`, twice, in parallel, with `collect_*` on the typed AST — so no serialization, no serde, no core changes. But it's still `in_process_rustc_experiment.rs`, and `experiment_three_a_fatal_rustc_error` is a *deliberate* red (576/**176**/8). That plus `fixtures_broken_rust/` should be deleted or inverted before anything commits.
2. **The oracle violates our own @ATAFLBZ** in three places — matching by human-name string, including `resolve_method` matching a method's *owner* by name. The arch doc already states the rule with the `Box` example.
3. **Eagerness.** Four layers, and the function-compile phase compiles every declaration whether called or not. Fine at five names; `import rust.std.vec.Vec` brings ~100 inherent methods.
4. **Nothing committed since `8d40eff9d`.**

Blocked elsewhere and routed around: `dot_borrow` and `get_param_environments`' missing ref-peel (both pure-Vale, with Vale2 since 2026-07-26); tier 2 needs the LLVM 16→~21 port and the onion relink.

---

## 👤 User

yep lets continue where we left off

---

## 🤖 Claude

Picking up at plan §5 step 1 — promote the experiment into the real harness. Let me read the current state.

---

## 🤖 Claude

Baseline confirmed: **576 / 176 / 8**, with `experiment_three_a_fatal_rustc_error` as the one deliberate red. Now let me see the pieces the harness needs.

---

## 🤖 Claude

Good picture. Let me check the test-harness entry point before writing.

---

## 🤖 Claude

Now the structured oracle log — the piece that makes moved assertions non-brittle.

---

## 🤖 Claude

Now the driver — its `check()` is test logic living in a production binary; the assertions move to the corpus.

---

## 🤖 Claude

Now the harness. Let me check the compilation entry points precisely.

---

## 🤖 Claude

Now the corpus itself.

---

## 🤖 Claude

Now the deletions — step 3 of the plan, the fixture oracle loses its last consumer.

---

## 🤖 Claude

Lib builds. Now the suite.

---

## 🤖 Claude

The mechanism is sharper than assumed — rustc *unwinds* with a `FatalErrorMarker` rather than exiting. Let me handle it the way rustc's own API intends.

---

## 🤖 Claude

**580 / 175 / 8** — the deliberate red is gone and the failure count is back to the 175 baseline. Let me verify the default config and the driver.

---

## 🤖 Claude

8 warnings, all pre-existing, none in the new files. Now the driver.

---

## 🤖 Claude

Driver green, exit 0, and the log reads cleanly. Now the docs.

---

## 🤖 Claude

Final verification across both configs.

---

## 🤖 Claude

One pre-existing warning sits in the arc's own uncommitted work — one word, worth clearing.

---

## 🤖 Claude

Plan §5 steps 1–3 are done, and the tree is green in both configs.

## What landed

**The harness** (`test/rust_interop/harness.rs`) — `run_case` / `try_run_case` plus one `Callbacks` impl. The extractor is higher-ranked with `R` fixed outside the `for<'s, 't>`, so "only owned data escapes the callback" is a compile error to violate rather than a rule to remember.

**The corpus** (`test/rust_interop/cases.rs`) — 7 cases replacing 3 experiments + 1 fixture test + the driver's `check()`. One behaviour each, so a failure localizes instead of arriving as a lump: free function, empty-allowlist negative control, structural generic reading, method on a Rust type, scope-end drop, declined projection, and the fatal-rustc-error regression.

**`FixtureOracle` deleted** — `calls_a_rust_free_function` moved onto real rustc, which was its last consumer. `fixtures_missing/` went with it.

**The driver carries no assertions.** It compiles, reports, exits. It stays as the seed of the real `valec-rs`; the reason it *had* to be a binary turned out not to hold.

## Two things worth flagging

**The fatal-error mechanism isn't what we assumed.** It was recorded as a `process::exit` that would take the suite down. rustc actually **unwinds**, with a `FatalErrorMarker` payload rather than a string — which is exactly why the deliberate-red experiment failed with *no message at all*. `catch_with_exit_code` is rustc's own way to convert that, and the harness uses it. Same conclusion, better mechanism, and the case now passes instead of being a standing red. Corrected in both docs.

**Assertions no longer key on `Debug` rendering.** The plan noted substring matches broke twice in one day without any behaviour changing, so: the oracle log carries a typed `OracleQuery` beside its rendered line, a compile failure carries the `ICompileErrorT` variant name beside its detail, and AST assertions go through a test-owned `describe_kind`. Recorded as arch §26b.4.

## State

| | |
|---|---|
| default | **573** / 175 / 8 — unchanged |
| interop | **580** / 175 / 8 — no deliberate red; was 576/**176** |
| driver | exit 0 |
| warnings | 8, all pre-existing (cleared one `unused_mut` in `importer.rs` that predated today) |
| core diff | still **empty** |

Nothing committed — still on `8d40eff9d`.

Next in the plan is growing the corpus along the axes that multiply (parameter-position Rust types, several types, two crates, a generic Rust *type*), then the fixture compile-check, then the `@ATAFLBZ` name-matching fixes — which now have a corpus to catch regressions in.

---

## 👤 User

can you remind me all the preferences i've expressed throughout these conversations (verbatim please), and for each one, how far away we are from all of them?

---

## 🤖 Claude

Pulled from both transcripts. Grouped by theme; status is how far the tree is from each, not how well-recorded it is.

---

## Containment and tech debt

> "its very important that rust interop doesnt add tech debt to the main compiler."

**Met.** `git diff` on core is empty. `overload_resolver.rs` has zero interop content. Permanent footprint is the module declaration, the `Oracles` field and its threading, `Source::rust()`, and the test-subtree gate.

> "i see youre adding rust interop code outside of a rust_interop folder. whats this src/bin thing about?"

**Met.** Everything lives under `rust_interop/` and `test/rust_interop/`. The floor is two files Rust forces to the crate root: `lib.rs`'s `cfg_attr` + `extern crate rustc_*`, and `Cargo.toml`.

> "our docs should be in docs/convos/rust_interop. reasoning: rust interop is separate from the main compiler, so we can keep the main compiler sane, and let claude loose on just the rust interop part while keeping it contained to the rust interop dirs."

**Met.** Also "all of our convos should be in ./docs/convos" — met, with a rename table for the stale paths inside verbatim transcripts.

> "i want us to remove all of the ZRI comments before we commit. though we should capture somewhere the things we think we'll need to modify in the core compiler"

**Met.** All 21 removed; callout map §9 is the index that replaced `grep ZRI`.

---

## Architecture

> "we are going to keep the c++ backend. we're not using rust's MIR, and we're using LLVM with our own backend to produce things that we'll eventually link in with the rust stuff."

**Met and unchanged.** The interop build's backend link is gated *off* — explicitly temporary scaffolding expiring at the LLVM port, with the reason in `build.rs`'s comment so nobody reads it as the backend becoming optional.

> "we'll get a lot of things working in the typing pass, then we'll do the llvm 16->21 port ourselves, and then we'll get things working with codegen/instantiator, and then we'll do more of the typing pass, and then more of the codegen/instantaitor, etc."

**Still in phase one, and it's the long one.** The LLVM 16 → ~21 port hasn't started. Everything downstream — tier-2 tests, the instantiator, symbol names, `@SMLRZ` — waits on it.

> "confirmed, dont split now" (single crate, not `frontend_rust_rustc`)

**Met.** Split trigger recorded: when `per_instance_mir`/`layout_of` land.

> "i dont think we should generate .vale source literally. if anything, we'd want to generate FunctionS/StructS... ones that are wrappers."

**Met.** `FunctionS` + `IBodyS::ExternBody`, and a real `StructDefinitionT` with zero members.

> "we should be register *templates* this early. later on, when we try to call them, is when ITemplataT::Prototype things should be made. thats important too because we want to support extern generics, without making a PrototypeT up front"

**Met, and it was the pivot.** `pick<A, B>` at `<int, bool>` typechecks; `make_extern_function` mints the prototype per instantiation.

> "could we instead have added an extern attribute to the generated struct?"

**Met.** That's what took the core diff from one guarded arm to zero.

> "sharedness: Single and weakable: false is correct. rust will never support those two things."

**Met**, asserted in the definition rather than left absent.

> "yes, global panic=abort."

**Ratified, no work yet.** It was already arch §1.7/§16, and nothing exercises it until codegen.

> "valen-private things (not explicitly declared by user as exported) should be *invisible* and unknown to rustc. please make sure thats somewhere near the top of our architecture doc."

**Met.** §1.7 bullet, naming the misreading explicitly. It was already in the doc at §9.4 — but at line 998 of 3,649.

> "we'll want to expand Vale to handle those types, long-term. but for now, panic."

**Partly, and now inconsistent.** `IntT` still has no signedness, `FloatT` no width. Worse, `lower_ty` *panics* on an un-imported ADT where the structural path *declines* — the two paths disagree, and per Harmonious's rule (decline while user input can still be at fault) the panic is on the wrong side.

---

## Naming and collisions

> "\"x = Vec<int>();\" is legal"

> "long term we should copy what rust does here."

> "top-level functions is the way forward. if we run into a collision, we should work on qualified names."

**Not started, deliberately.** Bare Rust type names are *not* resolvable today — a Rust type reaches Vale only by inference from a signature. `lookup_nearest_with_imprecise_name` still `panic!("Too many with name")` rather than erroring, and `_get_only_nearest` is still ignored. Your last instruction defers this until a collision forces it, and we haven't hit one.

> "please note somewhere that we want to get rid of this linear walking and up-front iterating"

**Noted, not fixed.** `VCOORD` on `TyCtxtOracle::new` plus plan §6. The walk is also *insufficient*, not merely slow — `module_children` on a crate root can't find `std::vec::Vec`.

---

## Testing

> "the collect_ macros shouldnt be reachable from production code. all walks must be manually written. otherwise it's way too easy to introduce a very expensive walk."

**Met.** `collect_*` appears only under `test/`. The driver doesn't walk at all now, because it asserts nothing.

> "we shouldnt have a fixture oracle, and we should strive to get rid of it."

**Done today.** `fixture.rs` deleted; its last consumer moved onto real rustc.

> "tier 1 also should test the compiler outputs, the typing pass output AST."

**Met.** Cases walk `main`'s body with `collect_where_tnode!` and assert on callee name, rust-backedness, params and return.

> "i still very much like the spirit of dark-box. because unit tests are bad, theyre brittle, and they cause inertia keeping us to a certain architecture"

> "also, please read DBAPIZ"

**Met for the boundary.** `run_case` is source-in, structured-outcome-out; test logic no longer ships inside `valec-rs`. Assertions key on typed values, not on `Debug` rendering.

> "how many tests for the typing pass do we want, invoking the actual rustc typing pass and our typing pass together? i think a lot of them."

**7. Not a lot.** The sibling tree's 32 is the order of magnitude to pass.

> "how many tests do we want to be truly end-to-end, invoking the actual rustc entirely (typing, mono, codegen), and our entire compiler (typing, instantiator, backend) together? i think a lot of them there as well."

**Zero, and blocked.** Needs the LLVM port *and* the onion arc's relink of `backend_ffi`/`pass_manager`.

> "i think every test should be run in the former, and every test should be run in the latter"

**Diverged, and you should know it.** I built tier-1 cases as `#[test]` functions, not as on-disk fixtures. The Rust crates are shared and will feed tier 2 unchanged — but the *Vale programs* live in Rust string literals, so a tier-2 runner can't read them. Making one corpus feed both means moving the Vale source and expected outcome onto disk. Cheap now at 7 cases; not cheap at 40. I'd want your call on doing it before growing the corpus.

> "we should be doing this with good /tdd"

**Partly, honestly.** Today was migrating existing assertions rather than adding behaviour, so there was no RED to see for most cases. The one that did go red-then-green — `a_fatal_rustc_error_costs_one_case` — is exactly where I learned something: rustc *unwinds* rather than exits, which is why the old experiment failed with no message at all.

> "can you make this happen with only changes in rust_interop? i do eventually want to serialize typing pass outputs, but im not ready to design that into vale core properly quite yet."

**Met.** Tier 1 needs no serialization at all, so the question is fully deferred to whenever you want to design it on its own merits.

---

## Process

> "let me know if any core changes are needed, so i can do them manually"

**Followed.** None needed today; the one time it was, I brought you the verbatim two hunks first.

> "please dont edit with scripts" / "please edit normally"

**Followed.** Edit/Write only.

> "typing-pass should build, and some typing pass tests should pass, thats the bar for today."

**Met.** 573/175/8 default, 580/175/8 interop, 8 pre-existing warnings, `bin/valec` untouched so nobody else's build changes.

> "we'll be using their prototype as a signal for *what works*, but not necessarily *whats best*. keep an eye out for things we can do better than they did"

**Ongoing.** Today's instance: their `CallbackLog` is text they grep; ours carries a typed query beside the rendered line, because they told us grepping cost them.

---

## The three I'd rank as furthest away

1. **"a lot of them" in both tiers** — 7 and 0 against a stated bar of "a lot", with tier 2 hard-blocked on work that hasn't started.
2. **One corpus feeding both tiers** — currently structurally impossible, and the cost of fixing it grows with every case added.
3. **The name/collision story** — deferred by your own instruction, but it's what stands between "a Rust type arrives from a signature" and `x = Vec<int>()` being legal, which you said it is.

---

## 👤 User

tell me again, all the ones that werent clearly met please

---

## 🤖 Claude

Let me verify two residues before answering — both are core code from earlier commits that the pivot may have orphaned.

---

## 🤖 Claude

Two concrete findings from that check, which change one of the answers I gave you. Here are only the ones not clearly met.

---

**"how many tests for the typing pass do we want... i think a lot of them."**
7. Not a lot. The sibling tree's 32 is the bar to pass.

**"how many tests do we want to be truly end-to-end... i think a lot of them there as well."**
Zero, and hard-blocked on the LLVM port *and* the onion arc relinking `backend_ffi`/`pass_manager`.

**"i think every test should be run in the former, and every test should be run in the latter"**
Diverged. Tier-1 cases are `#[test]` functions with the Vale source in Rust string literals, so a tier-2 runner can't read them — only the Rust fixture crates are shared. Fixing it means moving the Vale program and expected outcome onto disk. Cheap at 7 cases, not at 40, so this wants deciding before the corpus grows.

---

**"we'll get a lot of things working in the typing pass, then we'll do the llvm 16->21 port ourselves, and then we'll get things working with codegen/instantiator, and then we'll do more of the typing pass..."**
Still in the first phase. The port hasn't started, and it gates tier 2, the instantiator, symbol naming, and the `@SMLRZ` wire-format work.

**"\"x = Vec<int>();\" is legal"**
Not true today. A Rust type reaches Vale only by inference from a signature; bare Rust type names don't resolve.

**"long term we should copy what rust does here."** (name collisions)
Not started. rustc's `NameResolution { single_imports, non_glob_decl, glob_decl }` is recorded as the design to copy; nothing built.

**"we'd see two Vec templates? that should be an error in some way."**
Still `panic!("Too many with name")` at `environment.rs:164`, and `_get_only_nearest` is still ignored at the package level. Untouched.

**"top-level functions is the way forward. if we run into a collision, we should work on qualified names."**
First half met. Second half untested — we haven't hit a collision, so the trigger hasn't fired.

---

**"we'll want to expand Vale to handle those types, long-term. but for now, panic."**
Partly, and now internally inconsistent. `IntT` still has no signedness, `FloatT` no width. Worse: `lower_ty` *panics* on an un-imported ADT while the structural path *declines* the same class of problem. Per the rule we adopted — decline while user input can still be at fault — the panic is on the wrong side, and an un-imported type is user error.

**"please note somewhere that we want to get rid of this linear walking and up-front iterating"**
Noted (`VCOORD` on `TyCtxtOracle::new`, plan §6), not fixed. And it's *insufficient*, not just slow — `module_children` on a crate root can't reach `std::vec::Vec` at all.

**"we should be doing this with good /tdd"**
Partly. Today was migrating existing assertions, so most cases had no RED to see. The one that did — `a_fatal_rustc_error_costs_one_case` — is where the real finding came from.

---

**"its very important that rust interop doesnt add tech debt to the main compiler."**

I said Met. That's true against HEAD, but not against the whole arc — two things interop put in core in `699241ffb` are now orphaned:

- **`Source::rust()` + `resolve_rust_package` in `code_source.rs` have zero callers.** ~44 lines of dead core code, added for an import path that the synthesized-declaration design never used.
- **The `INameT::ExternFunction` arm in `get_imprecise_name`** (`environment.rs:488`) was added for the prototype-store design. Under the pivot, stores hold `IEnvEntryT::Function`, so it's likely unreachable — but I have not verified that, and I won't instrument a core file without asking. The cheap check is to make it panic, run both suites, and revert; say the word.

Both are the "dead but constructible" shape Harmonious warned about, which is how an abandoned design gets restored by accident.

---

**"yes, global panic=abort."**
Ratified, unexercised. Nothing touches it until codegen exists.

**"we'll be using their prototype as a signal for *what works*, but not necessarily *whats best*. keep an eye out for things we can do better than they did"**
Ongoing by nature, no end state. Today's instance: their `CallbackLog` is text they grep; ours carries a typed query beside the rendered line, precisely because grepping cost them.

---

## 👤 User

1. lets make sure that at least 32 specific tests are implemented or at least in the plan. we should say specifically what each one is / will be.
2. understood. lets capture our intent to have those into the plan doc.
3. yeah, every tier 2 test can just look at the output, thats fine. thats how it currently works in vale compiler tests too.
4. make sure the plan has that please.
5. remind me how its not legal?
6. ackn.
7. ackn.
8. ackn.
9. ackn.
10. should we fix those panics to be in the right place?
11. where did we end up on this?
12. how are we doing on the RFIGA/tdd/testing?
13. lets get rid of that dead code please.
14. add to the plan that we should have a test for this.
15. ackn

---

## 🤖 Claude

Taking these in order. First the dead code (13) — let me see exactly what's there.

---

## 🤖 Claude

`Source::Fn` itself is used everywhere — only the two rust-specific pieces are orphaned. Deleting them.

---

## 🤖 Claude

Now the panics. Let me see what's actually there.

---

## 🤖 Claude

Both suites green, `code_source.rs` now carries zero interop cfgs. Now the plan doc.

---

## 🤖 Claude

Now the plan doc — the corpus catalogue is the main piece.

---

## 🤖 Claude

Let me verify the constructor claim before answering that one.

---

## 🤖 Claude

One claim in that answer I'd rather know than believe. Quick experiment.

---

## 🤖 Claude

The type resolved — the failure is downstream, in the known borrow gap. Let me isolate naming from use.

---

## 🤖 Claude

It passes. **Vale source can already name a Rust type** — that corrects what I told you. Let me record it.

---

## 🤖 Claude

Done. Both configs green: **573/175/8** default, **581/175/8** interop, 8 pre-existing warnings.

## Actions taken

**1 & 2 & 3 & 4 — plan doc.** New §5.1 names the whole intended corpus: **39 cases, 8 implemented, 31 planned**, grouped by axis (signatures/lowering, item kinds, multiplicity/crates, scoping, failure modes, provenance, Vale-source naming). Each row says what breaks if it fails. §3 now locks that **tier 2 asserts on program output only**, that a case is `(Rust fixture, Vale program, expectation)` read by both tiers via a shared Rust `const` — no on-disk schema — and records the alternating phase order with the LLVM port as the tier-2 gate.

**13 — dead code gone.** `Source::rust()` and `resolve_rust_package` deleted. `code_source.rs` now carries **zero interop cfgs**; `Source::Fn` itself stays, it's used everywhere else. A comment records what was there and when it comes back.

**14 — planned as case 37**, `no_extern_function_name_reaches_an_environment_store`. I didn't instrument the core arm unasked; the cheap manual check (make it panic, see if both suites stay green) is noted.

---

## 5 — how `x = Vec<int>()` isn't legal

**I was wrong, and the experiment says so.** Bare Rust type names *do* resolve in hand-written Vale today. This compiles, no import statement:

```vale
exported func main() int { return value_of(make_counter()); }
func value_of(c Counter) int { return 7; }
```

Pinned as case 38. The citizen is a `Kind` entry in the reserved `rust` package's store and `PackageEnvironmentT` unions every top-level store, so the name is ambient — the same path a synthesized declaration's `LookupSR` takes.

What's actually missing for `Vec<int>()`:

- **No constructor.** `Vec<int>(...)` is a *call*. For a Vale struct the callee comes from `derive_struct_constructor`, run by `get_struct_sibling_entries` — which iterates parsed `StructS` denizens. A Rust type has none, so it never runs. Deliberately: the fields are opaque and fabricating a constructor claims knowledge of layout and invariants we don't have.
- **No generic Rust *types*.** Generic functions work; `importable_types` handles non-generic ADTs only.
- **Collision precedence**, latent as before.

The experiment also confirmed the *use* half is blocked exactly where we thought: `return (c).get()` panics in `is_type_convertible` on `BorrowRef(Counter) -> Counter` — Vale2's, already with them. That's case 39, written when they land it.

## 11 — the linear walk

Where we ended up: **noted, not fixed, and now with a named endpoint.** `VCOORD` on `TyCtxtOracle::new` plus plan §6. The finding that matters is that it's *insufficient* rather than slow — `module_children` on a crate root is one level deep, so `std::vec::Vec` could never be found, and recursing would make it both expensive and collision-prone. The endpoint is resolving the one path an `import` names (clippy and rustdoc both walk segment by segment; rustc populates foreign modules on first touch). Nothing built.

## 12 — RFIGA / TDD / testing

Honest: **the harness is good, the discipline was partial.** Today was migrating existing assertions, so most cases had no RED to see first. Two places it did work properly and paid: `a_fatal_rustc_error_costs_one_case` went red and taught me rustc *unwinds* rather than exits; and case 38 above was written as a probe and immediately corrected a claim I'd made to you twice. Generics earlier in the arc followed the full RFIGA loop. Going forward the corpus is the RFIGA list — case 25 in particular is specified to be **written red** against the `@ATAFLBZ` fix.

## 10 — the panics

**Yes, but not by flipping them to declines**, and I've recorded why rather than acting. Three constraints collide:

1. "for now, panic" was chosen over `None` because `None` gave "couldn't find function `foo`" for a function that exists — a lie, worse than a crash.
2. But these fire during *enumeration*, not at a use site. One `u64` in a crate's exports would make the whole crate unimportable. Declining is right there.
3. Which means (1) and (2) only reconcile through **poison, don't drop** — register the declaration with its reason so the use site says *"found `first`, but its return type has no Vale form."*

Poisoning needs a small core hook (a field on the declaration, or an `ICompileErrorT` variant), so it's yours rather than mine to land. Interim: leave the panics, keep the reason in the oracle log. Cases 11–15 pin whichever way it goes. Say the word if you want the core hook designed.

---

## 👤 User

how far are we from getting Vec<int>() to work?
please note in the plan where we are on 11, 12, 10

---

## 🤖 Claude

Good question — let me look at what the type path actually does today rather than estimate.

---

## 🤖 Claude

Let me measure it rather than estimate — adding a generic type to the fixture and seeing what happens.

---

## 🤖 Claude

The probe found something worse than a gap: it **compiles**, and `Holder<i32>` silently interns with `template_args: []`. Let me pin that.

---

## 🤖 Claude

Now the plan updates.

---

## 🤖 Claude

Green: **573/175/8** default, **582/175/8** interop, 8 pre-existing warnings.

## How far from `Vec<int>()`

I measured instead of estimating, and found something worth knowing.

**The naming half is done** — that was the actual question back in July ("must Rust types be named by qualified path?"), and case 38 verifies bare names work with no import statement.

**Then I probed a generic Rust type, and it isn't a gap — it's a silent wrong answer.** `Holder<i32>` and `Holder<bool>` both compile, and both intern as a bare `Holder` with `template_args: []`. Same Vale kind for different Rust types. `type_kind` never reads the ADT's `GenericArgsRef`. That's now pinned as case 40, asserting the defect so it's caught rather than merely known — invert it when the fix lands.

Four things stand between here and `Vec<int>()`, in dependency order (plan §9):

1. **Path resolution into nested modules.** `Vec` is `std::vec::Vec`, and the walk is one level deep — so `Vec` is unreachable, not just unimported. Sharper: `std::vec::Vec` is a *re-export* of `alloc::vec::Vec`, so there's no canonical path and a key map can't work; the resolver must walk segments.
2. **Generic types carrying their arguments.** The largest piece, and the one case 40 pins. Generic *functions* already work — their parameters ride the signature and Vale's solver substitutes them — but a generic *citizen* needs the name itself to carry args, which nothing has built.
3. **Outbound `GenericArgs` reconstruction.** rustc's real args for `Vec<i64>` are `[i64, Global]`; the Vale name carries `[Kind(i64)]`. Arch §8.10 already calls this Option A's sharpest weakness.
4. **A decision, not code: what does `()` mean for a Rust type?** A Rust-backed type gets no field constructor, correctly — Vale is an external consumer and `Vec`'s fields are private. So construction must route to `Vec::new`. Whether `Vec<int>()` should construct at all, or Vale source should say `Vec<int>::new()`, is yours to settle; nothing should be built for step 4 until it is.

**Honest distance:** steps 1 and 2 are each a real slice, 3 is small but fiddly, 4 is a decision. None of it waits on the LLVM port or Vale2. **`Holder<int>` from our own fixture needs only step 2** and is the right first target — `Vec` specifically drags in the walker, the allocator arg, and ~100 eagerly-compiled inherent methods all at once.

## Noted in the plan

- **11 (linear walk)** — §6: *recorded, nothing built*. The full reasoning lives as a `VCOORD` on `TyCtxtOracle::new`; it's step 1 of §9, so `Vec` is what forces it.
- **12 (RFIGA/TDD)** — new §5.3: *partial, and honestly so*, with the lesson stated as **probes before claims**. Three statements in that document were refuted the moment someone ran the code — including two of mine this session.
- **10 (panics)** — §6: *analysed, not changed, waiting on you*. Nothing flips until the poison hook is designed, because a silent decline reintroduces exactly the lie "for now, panic" was chosen to avoid.

Corpus is now **40 named cases, 9 implemented**. Nothing committed.

---

## 👤 User

what was our plan for name resolution again?

---

## 🤖 Claude

Let me check what survived into the docs, since §5 was rewritten twice.

---

## 🤖 Claude

Confirmed — the naming design got thinned to one bullet in the wind-down rewrite; the substance is only in the convo-9 transcript. Let me check one thing before I restore it.

---

## 🤖 Claude

Found a real divergence. Let me restore the design to the plan.

---

## 🤖 Claude

Restored as plan §10 — it had been thinned to a single bullet in the wind-down rewrite, which is exactly the loss the handoff doc exists to prevent. Here's the recall.

## The problem

A synthesized `FunctionS` carries **runes and rules**, not types. The only rule that names a type is `LookupSR { rune, name: IImpreciseNameS }`, and it resolves **by name**. So rustc hands us a precise `DefId` and we downgrade it to a string and ask Vale to find it again. The downgrade is unrecoverable: `PackageEnvironmentT::lookup_with_name_inner` ignores `_get_only_nearest` and concatenates every namespace, then `lookup_nearest_with_imprecise_name` does `panic!("Too many with name")`. Not bad resolution — *no* resolution, and two hits is a crash.

Worth remembering *why* this appeared: the old oracle design could defer name resolution forever, because a type arrived by identity from `fn_sig` and never went through a lookup. Synthesizing declarations is what put us on the name path. That was the pivot's unpriced cost.

## Two things ruled out

**`RuneParentEnvLookupSR` (@MKRFA)** — not an escape hatch. It's stripped on three paths and the function-*definition* solve isn't one; the solver now panics outright if one reaches it; and where it *is* stripped it resolves against the **calling** env by rune, and a program calling `make_counter()` has no `Counter` rune.

**A full-path key map** — can't work. `std::vec::Vec` is `pub use alloc_crate::vec;`, so the key `["rust","std","vec","Vec"]` names no definition; the def path is `alloc::vec::Vec`. There's no canonical path to pick — rustc runs `visible_parent_map`, a BFS over the whole crate forest, purely to invert def-path → writable-path for diagnostics, and it's explicitly lossy. Both real analogues of our oracle, clippy and rustdoc, **walk segments**.

## The design

**Representation and resolution are layers, not alternatives** — that was the reversal worth keeping. Your qualified name is the source-level representation; walking is the resolution strategy.

- **Representation:** `IImpreciseNameValS::QualifiedCodeName(&[StrI])` as a *sibling* variant. Not widening `CodeNameS` (~102 refs, the hottest name type — equality would go from interned-symbol to slice comparison). And not a vec on `LookupSR`, because `r.name` goes straight into a map keyed on the interned `IImpreciseNameS` — **the key type is the deciding axis**.
- **Resolution:** walk, via `children_of(item) -> [(name, ns, item)]` — our store for Vale, `tcx.module_children` for Rust.
- **The objection to walking was wrong.** I'd said it needs a namespace *value* type that `ITemplataT` lacks. rustc has none either — there's no `Res::Module`; a module is `Res::Def(DefKind::Mod, ..)`, rejected in every position. Modules are legal *intermediates*, illegal *finals*. So we need a resolver-result enum that isn't `ITemplataT`, which is far cheaper than a new templata kind.

**Precedence: steal rustc's struct.** `NameResolution { single_imports, non_glob_decl, glob_decl }` with `best_decl() = non_glob_decl.or(glob_decl)`. Precedence is a *field*, not a comparison — E0252 is a collision in the data structure, E0659 is a loser stapled to the winner and reported lazily. The three-tier model we called inexpressible costs one struct with two `Option`s. Plus: user-defined names outrank stdlib, so growing the stdlib isn't breaking.

**Dual registration makes `import` mean something.** Register under the qualified key always, and the bare key *iff* imported. Then bare `Counter` resolves only if you imported it, and the panic can only fire for two *imported* things sharing a name — exactly where Rust raises E0252. Precedented: `add_entries` already registers prototypes under three imprecise keys.

**Scale:** `rustc_resolve` is 26,099 lines; the kernel is ~1,500–2,500. **Globs are what force the fixed-point iteration** — without them imports are a DAG. rustc still has four open 2024 issues where its fixed point doesn't converge. Strong argument for Vale not having globs, or having them late. And populate lazily, as `populate_on_access` does — which is also the fix for our eager walk.

**The cheap out, if the surface stays small:** rustc's own way of naming library items from compiler code isn't paths at all — `#[rustc_diagnostic_item = "Vec"]`, 397 of them, a flat `Symbol → DefId` declared at the definition site. A registry, not a resolver.

**Vale's own name story is separate and worth more** — in value order: make `import X.Y.Z` bind `Z` (registration-time, and the data already survives intact in `ImportS`; my earlier claim that `importee_name` was discarded was wrong); turn the panic into an ambiguity error; qualified paths as escape hatch. The first two need no walker.

## One divergence I found while checking

Three things were named "cheap now, expensive to retrofit." Two landed. The third didn't: **`declarations.rs:115` still emits a bare `CodeNameS`**, not a qualified name. It hasn't bitten because nothing collides yet, and it's still cheap — one call site plus the variant — so it's worth doing *with* the variant rather than carrying as debt. Recorded in §10.9.

---

## 👤 User

i was imagining that we'd ask rust what the thing's real canonical name is, and then use that in the generated FunctionS, would that not have worked?

---

## 🤖 Claude

It would have worked — and better than what I wrote. My §10 conflated two different problems.

## Two problems, not one

**A. A synthesized declaration naming a type.** We mint *both* ends: the `LookupSR` in the generated `FunctionS`, and the store entry the importer registers. Nothing user-written is involved.

**B. User source naming a Rust item** — `import rust.std.vec.Vec`, or bare `Vec` in a `.vale` file.

Everything I said about re-exports killing a key map — `std::vec::Vec` is `pub use alloc_crate::vec;`, def path is `alloc::vec::Vec`, no canonical path to pick — is **true only of B**. It's an argument about matching a *user's* path against a definition. For A there's no user path to match: ask rustc for `def_path`, use it as the key on both sides, and they agree by construction.

## Why it works for A

The Vale name already has somewhere to put it. `IdT.package_coord` is `{ module, packages: &[StrI] }`, and arch §8.10 explicitly says *"the module path rides `IdT.package_coord`"*. So `Vec` becomes `rust.["alloc","vec"] :: Struct(Vec)` — the whole canonical path, carried in the id, no new mechanism.

Today `TyCtxtOracle::new` takes **one** `package_coord` and stamps every item with it, which is why everything lands in `rust.["mycrate"]` regardless of nesting. Populating it per-item from `tcx.def_path` is the change.

Then:
- **No collisions.** Def paths are unique by construction, so `alloc::vec::Vec` and a Vale `Vec` are different keys. The `panic!("Too many with name")` path is never reached.
- **No walker, no precedence struct, no fixed point** for A.
- **No `get_only_nearest` fix** needed for A.

The `QualifiedCodeName` variant is still the vehicle — `imprecise_to_entries` is keyed on an interned `IImpreciseNameS` — but its contents come from rustc rather than being reconstructed. That's strictly better: it's the *real* name, not our guess at one.

## What it doesn't solve

B survives, but it shrinks to the right size: **resolving the handful of paths in `import` statements**, once. An `import rust.std.vec.Vec` still has to traverse the re-export to reach the DefId — that's where the segment walk is genuinely needed, and only there. After that everything is def-path-keyed. Which is what §10.6 already said the end state was: *"the end state enumerates nothing — an import resolves that one path segment by segment, keyed by `DefId` thereafter."* I just failed to notice that it makes A independent of B.

Dual registration (§10.5) still earns its place, now with a cleaner story: the def-path key always, plus the bare user-facing name when imported.

## One consequence worth pricing

Def path is the *definition* path, so a diagnostic would say `rust.alloc.vec.Vec` where the user wrote `std.vec.Vec`. That's exactly the inversion rustc runs `visible_parent_map` for — a lossy BFS it maintains **purely for diagnostics**. So the split is: def path for identity, a `visible_parent_map`-shaped inversion for error messages, eventually. Which is how rustc itself divides it, and it's a diagnostics problem rather than a resolution one.

No @SMLRZ risk here — that trap is about Rust's *name shape* (`Vec<i32>::push` vs `Vec::push<i32>`), not about using Rust's module path as the package path, which is what `package_coord` is for.

---

Let me fix §10, since it's misleading as written.

---

## 🤖 Claude

Section numbering got out of order. Fixing.

---

## 🤖 Claude

Suite still **582/175/8**; docs only.

## Short answer

**Yes, and it's the better design.** I split name resolution into two problems in §10.0:

- **A — a synthesized declaration naming a type.** We mint both ends, so ask rustc for `def_path`, key on it, and the two agree by construction. No collisions (def paths are unique), no walker, no precedence struct, no `get_only_nearest` fix.
- **B — user source naming a Rust item.** *This* is where re-exports bite, because you're matching a path a human wrote against a definition.

Everything I cited about `visible_parent_map` and clippy/rustdoc walking is an argument about B. I'd applied it to both.

The nice part is the Vale name already has the slot: `IdT.package_coord` is `{ module, packages: &[StrI] }`, and arch §8.10 already says *"the module path rides `IdT.package_coord`."* So `Vec` becomes `rust.["alloc","vec"] :: Struct(Vec)`. Today `TyCtxtOracle::new` takes **one** coord and stamps every item with it — that's why everything lands in `rust.["mycrate"]` regardless of nesting. Per-item stamping from `def_path` is the change.

Problem A end to end is four steps, now written into §10.9:

1. stamp `package_coord` from `tcx.def_path` per item;
2. add `IImpreciseNameValS::QualifiedCodeName(&[StrI])` + interner/humanizer arms;
3. `declarations.rs:115` emits it instead of a bare `CodeName`;
4. `get_imprecise_name` derives the same key for a registered Rust citizen.

B doesn't disappear but shrinks to the right size — resolving the paths in `import` statements, once, which is the only place a segment walk is genuinely needed. That's what §10.6 already called the end state; I just hadn't noticed it makes A independent of B.

Two things I'd price before building: the def path is the *definition* path, so a diagnostic says `rust.alloc.vec.Vec` where the user wrote `std.vec.Vec` — that inversion is what rustc keeps `visible_parent_map` for, and it's diagnostics-only, so it can wait. And this is not @SMLRZ: that trap is about Rust's *name shape* (`Vec<i32>::push`), not about using Rust's module path as a package path, which is what `package_coord` is for.

One knock-on for §9: step 1 (the walker) is Problem B and `Vec`-specific. `Holder<int>` from our own fixture — items at the crate root — needs only the generic-args work, which is why it stays the right first target.

---

## 👤 User

sorry, im catching up. explain what we're settling on?

---

## 🤖 Claude

No problem — here's the shape.

## The thing we just settled: how a Rust item gets a name in Vale

**Ask rustc for the item's real definition path, and use that as its Vale name.**

So `Vec` becomes `rust.["alloc","vec"] :: Struct(Vec)` rather than just `Vec`. The generated declaration refers to it by that name, and the importer registers it under that same name. Both sides are ours, so they match automatically — and since definition paths are unique, two different types can never collide.

That was your suggestion, and it's simpler than what I'd been proposing.

## Why it matters

I'd been describing a fairly large piece of work: a name resolver with shadowing rules, precedence, glob handling — modelled on rustc's. Your question made me notice I'd merged two different problems:

- **Our generated code referring to Rust items.** We write both ends, so there's nothing to resolve. Def paths just work. **This is the one that was blocking us, and it's small.**
- **A human writing a Rust name in Vale source** — `import rust.std.vec.Vec`, or bare `Vec`. Here someone types a path that has to be matched against a definition, and that's genuinely messy, because `std::vec::Vec` is a re-export of `alloc::vec::Vec` — the path people write isn't the path the thing lives at.

All my arguments about re-exports and needing a walker apply only to the second one. Applying them to the first made an easy problem look hard.

## What that changes

The big resolver is **deferred, and smaller than feared** — it's only ever needed for import lines, once each, not for every type mention. And it's gated behind your call anyway: qualified names when we actually hit a collision.

What's on the table to build for the first problem is four small steps: stamp each item's package coordinate from `tcx.def_path` instead of one hardcoded coordinate; add a qualified-name variant to the name enum; have the declaration generator emit it; make the lookup side derive the same key.

**Nothing is built.** This is written into the plan (§10) — it had been thinned to a single bullet during the wind-down, which is why I couldn't just recall it and had to go check.

## Where the arc is overall

Working today: a Rust function, a generic Rust function, a Rust type reaching Vale from a signature, a method, and drop — all through real rustc, with an empty core diff. 9 automated cases, 40 named in the plan.

Not working: generic Rust *types* (they compile but silently lose their arguments — newly found and pinned), anything needing the LLVM port, and the borrow path, which is Vale2's.

---

## 👤 User

"gated behind your call anyway: qualified names when we actually hit a collision."
what call?

---

## 🤖 Claude

That one — verbatim, from convo-9, when you chose between Rust methods living inside the citizen's environment versus being ordinary top-level functions:

> "top-level functions is the way forward. if we run into a collision, we should work on qualified names."

It set the pattern for the whole naming thread: take the simple route now, and let an actual collision be the thing that triggers the bigger work.

It follows an earlier one from convo-8, after you asked whether two `Vec` templates should be an error:

> "long term we should copy what rust does here. also, i dont think this is a problem yet for our main goal of just making vale able to call into a simple extern rust function that adds two ints. lets focus on only the things that block us from that goal"

## What "hit a collision" would actually mean

Concretely: any Vale program where a Vale item shares a bare name with an imported Rust item. `PackageEnvironmentT` unions *all* top-level stores, so a Vale `Counter` and a Rust `Counter` both answer to the imprecise name `Counter`, and `lookup_nearest_with_imprecise_name` does `panic!("Too many with name")` (`environment.rs:164`).

One honest caveat: **that's predicted, not observed.** Case 33 in the plan (`a_vale_function_and_a_rust_function_with_the_same_name`) is the test that would prove it, and it isn't written yet. So we don't currently have a way to notice the trigger firing other than someone hitting it.

If you want that trigger to be real rather than theoretical, case 33 is cheap and I'd write it next — it either confirms the panic, or tells us something we don't know.

---

## 👤 User

hmmm. please /full-read look at /Volumes/V/Vale2/vcoord-handoff.md
especially the part on overload resolution, and how we should be having no conflicts.
i think that will help us here. i think there are some concerns that we're conflating, that actually can be orthogonal, which let us avoid the collision problem, if we do things right

---

## 🤖 Claude

Lines are dense — smaller chunks.

---

## 🤖 Claude

Read it — the relevant section is **"Mission — Overload resolution & dispatch model redesign"** (lines 898–968). You're right, and it dissolves most of what I've been calling the collision problem. I also verified one claim against our tree and found I'd told you something wrong.

## The three concerns I'd been conflating

| concern | mechanism | on >1 |
|---|---|---|
| **Naming a type** (`LookupSR`, a Vale type annotation) | `lookup_nearest_with_imprecise_name` | **`panic!("Too many with name")`** |
| **Collecting function candidates** | `lookup_all_with_imprecise_name` — *plural* | normal; overload resolution scores them |
| **Which namespaces are searched** | union of the **argument types'** namespaces + explicit imports | n/a — a foreign function isn't even a candidate |

I'd been treating these as one thing called "the collision problem." Only the first one panics.

**Correction owed:** I told you case 33 (`a_vale_function_and_a_rust_function_with_the_same_name`) would `panic!("Too many with name")`. It won't. `overload_resolver.rs:187` uses the plural lookup; the panic sites are all type lookups (`struct_compiler_core.rs:287`, `expression_compiler.rs:550`, the array element types, the rune lookups). Two same-named functions either resolve cleanly or produce `CouldntNarrowDownCandidates` — a designed error. So the collision trigger is much narrower than I said: **a Vale *type* and a Rust *type* sharing a name.**

## Why the dispatch model means "no conflicts"

Two rules do the work:

> **"No specificity, no phases, no fallback, no tiebreakers. Two equally-matching candidates is always an ambiguity error."**

That kills the worry from convo-8 about whether a Rust callee could outrank a same-named Vale one. It can't — the answer is a clean error the user disambiguates, never a silent ranking.

> **"A function lives in type T's namespace iff (a) it's defined in T's file AND (b) it mentions T in a parameter."** Candidates come from *"the union of namespaces of every arg type at the call site"* plus explicit imports.

So a Rust `add_two_numbers` isn't in some global soup — it's in the namespace of the types it mentions. A Vale call with no Rust-typed argument never even *sees* it. That's the orthogonality: **scope is determined by argument type, not by a shared ambient namespace**, so most collisions can't form in the first place.

## What this says about what we built

**It ratifies the seam collapse.** *"`x.foo()` and `foo(x)` search the exact same candidate set. No Self-based namespace, no separate dispatch path for dot-syntax."* A Rust method as a top-level function whose first parameter is the receiver **is** a member of the receiver type's namespace under this rule. We built the right thing for the right reason.

**And it names one thing we got wrong.** `rust_package_stores` puts Rust functions in the `rust` package's top-level store, and `PackageEnvironmentT` unions *all* top-level stores — so they're **ambient**, findable from every call site in the program. That's precisely the model this design replaces. Harmless at a 3-name allowlist; it's the thing to change when the dispatch redesign lands, and the fix is to put a Rust function in the namespace of the types its parameters mention rather than in a global store.

Two smaller things worth carrying: **`Ship` and `&Ship` are different namespaces** (relevant once our receivers stop being by-value), and the redesign **deletes the exact-vs-coercion tiebreaker**, at which point `is_type_convertible` collapses to a boolean driven off a dry-run `convert()` — which is the cluster our borrow path is blocked behind.

## One caveat from the doc itself

Line 317: *"Our namespace/dispatch model is NOT a divergence — it's unsettled upstream."* design-1 says nothing about how a candidate set is assembled, and module/import syntax is an open Valen question. So this is Vale2's design, not ratified — with an explicit flag to compare when Valen decides modules. Worth building toward, not worth treating as final.

Let me record this.

---

## 🤖 Claude

Suite unchanged at **582/175/8** — docs only.

## Recorded as §10.10

The headline is that Problem B doesn't disappear but stops being a *language-wide precedence* problem and becomes a *type-name* one. The only path that can still panic is **a Vale type and a Rust type sharing a bare name, reached from hand-written Vale source** — which is exactly what `import` scopes, and exactly where Rust raises `E0252`.

Three things changed in the plan:

- **Case 33 rewritten.** It now pins that same-named *functions* **don't** collide, rather than pinning a panic that can't happen.
- **§10.10 added** — the three-concern table with the verification, the two rules that do the work, and the note that this ratifies the seam collapse.
- **§6's collision bullet** now points at §10.10 rather than implying a broad precedence problem.

The one action item that fell out: `rust_package_stores` makes Rust functions **ambient** by putting them in a top-level store that `PackageEnvironmentT` unions globally. That's the thing the namespace model replaces. It's harmless now and I haven't changed it — but it's written down as "don't deepen the dependence on ambient visibility meanwhile," because every case added between now and the dispatch redesign is one more thing relying on it.

---

## 👤 User

whats next?

---

## 🤖 Claude

**First: there's a lot uncommitted.** 24 files, ~1,500 lines vs `8d40eff9d` — the generics work from the end of last session plus everything today. Both configs are green at baseline. That's the largest exposure on the board, and it needs the literal phrase from you.

## What I'd do next: one slice, "identity, not strings"

The two structural defects we've named are the same defect on opposite sides of the seam, and both live in `tyctxt_oracle.rs` — **zero core changes**:

- **@ATAFLBZ (plan §6):** three sites match by human-name string, including `resolve_method` matching a method's *owner* by name. Key on `DefId`, add a provenance filter.
- **Def-path package coordinates (§10.0 step 1):** stamp each item's `package_coord` from `tcx.def_path` instead of the single coordinate handed to the constructor. This is your canonical-name idea, and step 1 is the whole zero-core half of it.

Doing them together because they're one idea. And doing them *before* growing the corpus, because:

- **Case 25 is specified to be written red against the @ATAFLBZ fix** — two crates exporting the same short name. That only works if the fix hasn't landed yet.
- Every case added meanwhile is one more thing built on string matching and on ambient visibility, which §10.10 explicitly warns against deepening.

Then **grow the corpus** on a correct foundation. Group A is cheap — most of it needs only fixture functions.

## The alternative, if you'd rather have capability than correctness

**Generic Rust types** (`Holder<int>`, §9 step 2). Higher user-visible value: it's the largest remaining piece of the `Vec<int>()` road, it has a live defect already pinned by case 40, and it's also zero-core. The argument against going first is that it builds on the string matching we know is wrong.

I'd take identity-first, but it's close, and if the generic-type capability is what you want to see move, say so.

## Waiting on you, not on work

- **The poison hook** — the panic/decline decision needs a small core field or error variant (§6). Nothing should flip until it's designed.
- **`IImpreciseNameValS::QualifiedCodeName`** — steps 2–4 of Problem A add an enum variant to a core name type plus its interner and humanizer arms. Small, but core.
- **What `()` means for a Rust type** (§9 step 4) — whether `Vec<int>()` constructs at all, or Vale source says `Vec<int>::new()`.
- **The LLVM 16 → ~21 port** — gates tier 2 entirely.

## Smaller loose ends, whenever

Case 37 (does `get_imprecise_name`'s `ExternFunction` arm still get reached — it may be dead core code), and the fixture compile-check so a fixture can't rot into invalid Rust unnoticed.

---

## 👤 User

whats next? is there anything thats not blocked on me or core vale or Vale2?

---

## 🤖 Claude

Let me check what's actually reachable before answering.

---

## 🤖 Claude

Confirmed something worth acting on. Let me widen the check.

---

## 🤖 Claude

Those are Vale's own `Compiler::resolve_function` — unrelated. So five oracle methods are genuinely dead. Let me clear them.

---

## 🤖 Claude

Both suites green, driver exit 0, and the oracle log now contains zero per-call-site queries because the methods don't exist. Updating the plan.

---

## 🤖 Claude

Done and verified: **573/175/8** default, **582/175/8** interop, driver exit 0.

## Yes — a fair amount isn't blocked

I started with the one that turned out to be free. Five oracle methods had **zero callers** — `resolve_path`, `kind`, `resolve_method`, `resolve_function`, `field` — leftovers from the per-call-site design. Deleting them:

- **removed two of the three @ATAFLBZ sites outright.** `resolve_method` matched a method's *owner* by human-name string; `resolve_function` matched by name. Only the crate walk is left.
- **made "nothing queries the oracle per call site" unrepresentable** rather than tested, which retires planned case 35.

That's the shape I'd keep looking for: the pivot orphaned things, and deleting them is often better than testing around them.

## Unblocked, in the order I'd take them

1. **Generic Rust types** (§9 step 2) — `Holder<int>`. Biggest capability available, and it's the one with a *live silent defect* behind it: `Holder<i32>` and `Holder<bool>` currently intern to the same kind. `type_kind` is our file, and the declare APIs we'd need are the public core ones we already call. I'd want to probe before promising it's entirely ours — the template-vs-instantiation question for a generic citizen is the unknown.
2. **The last @ATAFLBZ site + the grep fence** — key the walk on `DefId`, filter by provenance. Now a one-site change.
3. **Grow the corpus** — groups A, B, D, E, F are fixture functions plus cases. ~25 of the 31 planned need nothing from anyone.
4. **Fixture compile-check** so a fixture that type-errors can't rot unnoticed.
5. **Driver takes Vale source from argv** — small, and makes it an actual `valec-rs` seed instead of a hardcoded program.
6. **Warm-cache observability** — Harmonious's standing prediction from convo-8, still unaddressed: an absent log entry can't distinguish "not consulted" from "served from rustc's incremental cache."

## Blocked, and on whom

**You:** the poison hook for panic-vs-decline (a field or an `ICompileErrorT` variant); the `QualifiedCodeName` variant for §10.0; deleting the `get_imprecise_name` arm once case 37 says it's dead.

**Vale2:** the borrow path — `dot_borrow` and `get_param_environments`' missing ref-peel — which is case 39. Also the dispatch redesign, which is what eventually un-ambients our Rust functions.

**The LLVM port + onion relink:** tier 2, entirely.

My recommendation is **1 then 2** — the generic-types work is where the real capability is, and it's better done on `DefId` identity than on string matching we'd have to unpick afterward. Want me to probe the generic-citizen question first, or go straight at it?

---

## 👤 User

lets wind down.
the last two conversations plus this one have captured a lot of direction and knowledge that im not sure is really adequately framed and captured in the handoff or architecture doc. can you make sure they both accurately capture where i want to take things, and capture the subtleties of how i want things done?

---

## 🤖 Claude

Good place to stop. Let me make both docs carry the *direction* and the *how*, not just the state.

---

## 🤖 Claude

Now the handoff doc — the working protocol has never been written down anywhere.

---

## 🤖 Claude

Wound down. **573/175/8** default, **582/175/8** interop, 8 pre-existing warnings, driver green, core diff empty.

## What I added, and why each was missing

The docs captured *state* well and *direction* unevenly. The gaps were mostly the "how," which had only ever existed as things you said in conversation.

**Handoff — new §0, "How this arc is run"** (it goes before the design, because a fresh session needs it first):

- **The core/interop split is a protocol, not a layout.** Core changes stop and come to you verbatim as hunks. The corollary is the part worth having in writing: *ask for the core change rather than routing around it in interop*. Twice the answer was "none needed" — `StructDefinitionT` needs no `StructS`, and the `extern` attribute removed the last guarded arm — and both were found by asking.
- **Probes before claims**, with the four refutations listed by name. This one earned its place today: every probe found something *different* from what it went looking for.
- **Deferrals are trigger-gated**, with a case pinning current behaviour so the trigger is observable rather than theoretical.
- **Who's authoritative on what** — Valen is the spec, Vale2 owns core semantics, Harmonious is *evidence not authority*, quoted directly. That relationship shaped a lot of decisions and appeared nowhere.
- **Doc discipline** — a wind-down must not thin reasoning. §10 was the casualty; recording the lesson at the top is the only thing that stops it recurring, including today.

**Handoff §1** — methods-aren't-special is now marked as **direction, not convenience**, with your framing ("one of rust's biggest mistakes") and the three independent things that agree with it. A future change that reintroduces a method-shaped path is now visibly going the wrong way.

**Architecture — new §1.5.6, "Interop must not add tech debt to the main compiler."** Your constraint was driving the whole arc and was recorded only as an outcome (core diff empty), never as a principle with rules. Four rules, each stated against the alternative it beat: containment and its irreducible floor; *a gated delegation is not debt, a lie is*; prefer honest data over a special case; make the wrong thing unrepresentable rather than tested. Plus delete corpses, with the sibling tree's `ExternFunctionTemplataT` as the cautionary example.

**Architecture — new §26b.5**, the *why* behind the testing shape. Your reasoning about brittleness and architectural inertia was the thing generating the strategy and wasn't written down, so the strategy read as arbitrary. Also records that per-pass suites are a concession to scale rather than a preference, that the corpus should be large in **both** tiers, and the real reason `collect_*` is test-only — a generic collecting walker makes an expensive traversal too easy to write.

**Architecture — §8.10 revision block** gained two items: the durable name is `tcx.def_path` (your suggestion, with the Problem A/B split), and function names cannot collide — only type names can.

## One thing I did rather than wrote

Five oracle methods had zero callers and are deleted — including the two that matched Rust items by **human-name string**, which were two of the three @ATAFLBZ sites. Only the crate walk is left.

Still nothing committed; still on `8d40eff9d`. `CLAUDE.md` and `.github/ci-guide.md` remain other people's uncommitted work, untouched.

---

## 👤 User

anything more we should capture in the doc? please list out all the things ive said (yes i know its a lot), and then tell me your leaning on whether we should capture it.

---

## 👤 User

anything more we should capture in the doc? please list out all the things ive said (yes i know its a lot), and for each, tell me your leaning on whether we should capture it.

---

## 👤 User

anything more we should capture in the doc? please list out all the things ive said in these three sessions (yes i know its a lot), and for each, tell me your leaning on whether we should capture it.

---

## 🤖 Claude

Let me verify four I'm unsure about before listing.

---

## 🤖 Claude

I went back through all three transcripts. Roughly 130 of your messages; most are session mechanics. Here's the full accounting.

## Already captured — direction and design

| what you said | where it lives |
|---|---|
| Keep the C++ backend; not using Rust's MIR; our LLVM links with the Rust stuff | arch §1.7, §5 |
| Typing pass → LLVM 16→21 port → codegen/instantiator → alternate | plan §3 |
| Stay a cfg'd submodule; don't split the crate | arch §8.10 |
| Expand Vale for unrepresentable types long-term; panic for now | plan §6 |
| Allowlist now, `import`-driven later | plan §6, §10.5 |
| Global `panic = "abort"` | plan §3, arch §1.7/§16 |
| Valen-private items invisible to rustc; put it near the top | arch §1.7 |
| `x = Vec<int>()` is legal (naming); copy Rust long-term; two `Vec`s should be an error | plan §9, §10 |
| Top-level functions; qualified names when a collision forces it | plan §0.3, §10 |
| `sharedness: Single`, `weakable: false` — Rust will never support either | plan §1 |
| Don't generate `.vale` text; generate `FunctionS`/`StructS` wrappers; the opaque-struct and extern-C-instruction precedents | plan §1 |
| Register *templates* early, not prototypes — that's what extern generics need | plan §1 |
| An `extern` attribute rather than a cfg'd guard arm | arch §1.5.6 |
| Methods must not be treated differently from functions — "one of Rust's biggest mistakes" | plan §1 |
| No fixture oracle | arch §26b.3 |
| A *lot* of tests in both tiers; every case in both | arch §26b.5, plan §5.1 |
| Tier 1 asserts on the typing pass's output AST | arch §26b, plan §5.1 |
| Tier 2 asserts on output only, like Vale's existing end-to-end tests | plan §3 |
| Unit tests are brittle and cause architectural inertia | arch §26b.5 |
| `collect_*` test-only; production walks hand-written | arch §26b.5 |
| No von/VonHammer; `serde_json` if ever; serialization stays out of core for now | plan §3 |
| Ask rustc for the canonical name | plan §10.0, arch §8.10 |
| Overload resolution shouldn't have conflicts; the concerns are orthogonal | plan §10.10 |
| Get rid of the linear walk and up-front iteration | plan §6, `VCOORD` |
| Rust interop must not add tech debt to the main compiler | arch §1.5.6 |
| Docs in `docs/convos/rust_interop/`; convos in `docs/convos/`; containment rationale | README, arch §1.5.6 |
| Interop code stays in a `rust_interop` folder; `src/bin` was wrong | arch §1.5.6 |
| Harmonious = what works, not what's best; they wrote half our arch doc | plan §0.4 |
| Verbatim core changes for you to apply; tell you what's needed | plan §0.1 |
| TDD/RFIGA | plan §5.2 |
| Typing pass builds and some typing tests pass = the bar | plan §2 |

## Not captured — I'd capture these

**1. The two sibling implementation trees.** Where they are, what each is, which is newer, and the trap. `/Volumes/V/RustInteropReiImpl` is a **branch of this repo** (`rust-interop-reimpl`), more recent, same language as ours — so its findings transfer directly. `/Volumes/V/ValeRustInterop` is the older Scala ancestor that holds the abandoned experiments as commented-out corpses. **The trap cost us real time twice:** an agent surveyed ReiImpl and reported *its* `file:line` numbers as if they were ours, which sent a whole plan section (`FunctionA`/`StructA`) in the wrong direction. **Strong yes** — this is a standing research resource with a standing hazard.

**2. Don't break other branches.** The `rust-toolchain.toml` case: adding `rustc-dev` to `components` would make every `cargo` invocation on every branch, for every dev and CI job, fetch hundreds of MB nobody but us needs. Deliberately not pinned; interop devs run one `rustup component add`. **Strong yes** — it's a live constraint that's trivially easy to undo by someone tidying up.

**3. The two same-named `rust_interop` identifiers.** `#[cfg(feature = "rust_interop")]` gates **compiler** code; `#[cfg(rust_interop)]` in arch §3.3 gates **user Vale source**. Same word, two levels, and renaming the wrong one breaks a language feature. **Strong yes.** While there: §8.10's body still spells the compiler-side gate the old way and still mentions `StubOracle`, both stale.

**4. C7's missing clause.** Harmonious's rule is *"an integration test must run the artifact and check its output, not assert the build returned 0"* — their build exited 0 while the linker resolved against `unreachable!()` bodies, and it took hours. Our §25.3 C7 inherited only the wrapper half. I said I'd restore it in convo-8 and never did. **Yes** — it's a one-line restoration of a lesson someone paid for.

**5. The crate-split trigger, and two stale sections.** §8.10 says single-crate submodule; §3.2 and §28 Phase 3 still list `frontend_rust_rustc` as a distinct crate. The contradiction is noted but never fixed, and the **trigger** — split when `per_instance_mir`/`layout_of` land — was never written down at all, so it'll get re-litigated from scratch. **Yes.**

**6. "When something needs a surprising capability, ask why."** Your `&mut coutputs` questioning — *"why does `rust_package_stores` take that?"*, then *"TLDR all the ways it modifies coutputs"*, then *"why is it calling `add_instantiation_bounds`?"* — is what caught the entire wrong design. **Strong yes.** Arguably the highest-value question of the arc, and it's a repeatable move, not a one-off.

**7. "A special case hints at something we're not seeing."** Your question about the `Vec::new()` guard — *"the fact that this special case is here hints to me that we're not seeing something, something beautiful"* — produced the import-materialization design, showed the guard was speculative dead code, and started the line that ends in today's shape. **Yes.**

**8. "Does this help Vale outside interop?"** Applied to qualified names and path walking; it's what produced §10.8's finding that Vale's own name story (make `import` bind, turn the panic into an error) is more valuable than the interop half. **Yes**, as a standing evaluation criterion.

**9. "Check what rustc does."** Four high-value findings came from it: `visible_parent_map` being lossy-by-design, the `NameResolution` precedence struct, no `Res::Module`, `populate_on_access`. **Yes**, brief.

**10. Concentrate cfgs rather than threading them.** The `Oracles` struct took `compiler.rs` and `compilation.rs` to zero interop cfgs by moving them into one purpose-built file. **Yes** — one line in arch §1.5.6, since it generalizes to the next service.

**Lower confidence, would still lean yes:** *verify against the arch doc before proposing an architecture change* — I proposed gating the C++ backend without checking, and the doc already covered it thoroughly.

## Not captured — I'd deliberately skip

Session mechanics (`ok next`, `eli5`, `tldr`, `show me the concrete artifact`, `dont update the plan doc yet`, `hold for a while`); option picks whose outcome is already visible in the code (the cargo feature, the crate-boundary fixture, the assert-after-return restructure); investigation requests; and things now in CLAUDE.md or a skill (no scripts, no `cd && cargo`, commit phrasing).

---

Ten items, and 1–5 are the ones I'd regret losing. Want me to add them?

---

## 👤 User

yep please add all 10, thanks

---

## 🤖 Claude

Adding all ten. Plan doc first.

---

## 🤖 Claude

All ten in. **573/175/8** default, **582/175/8** interop — docs only, nothing touched code.

## Handoff doc

**§0.5 — "A change must not cost other branches anything."** The bar you set (*typing pass builds, some typing tests pass*), why they run `--lib`, and the `rust-toolchain.toml` case in full: pinning `rustc-dev` would make every cargo invocation on every branch fetch hundreds of MB nobody but interop needs, and would break *all* cargo commands if it were ever unavailable for the pinned nightly. Marked **do not "tidy this up."** Plus the general pre-landing test: for each changed file, why can this not affect a feature-off build?

**§0.6 — "Moves that keep finding things."** The four questions plus the arch-doc one, each with the outcome it produced rather than as advice: *why does this need that capability* (caught the whole wrong design, via the `&mut coutputs` thread); *this special case hints we're not seeing something* (the `Vec::new()` guard → import materialization); *does this help Vale outside interop* (→ §10.8); *what does rustc do* (four decision-changing findings); and check the arch doc before contradicting it.

**§0.7 — "The sibling implementations, and the trap in reading them."** Where both live, which is newer, that **ReiImpl is a branch of this repo** so its findings transfer directly, and that ValeRustInterop's value is archaeology. The trap is called out in a banner: an agent surveying ReiImpl reports *its* `file:line` as ours, and it once sent a whole plan section toward `FunctionA`/`StructA` on the strength of a pass we've **retired**. Standing requirement: every sibling-tree survey states which tree each citation belongs to.

## Architecture doc

**§1.5.6 rule 5** — concentrate cfgs rather than threading them, with the `Oracles` result (two central files to zero) and why it generalizes to the inbound seam.

**§3.3 trap banner** — `rust_interop` names two unrelated switches. `#[cfg(rust_interop)]` gates *Vale user source*; `#[cfg(feature = "rust_interop")]` gates *compiler code*, and is a cargo feature specifically so `build.rs` can see the mode. Says which one to rename if the collision ever proves too costly — never the language feature.

**§25.3 C7** — the lost clause restored, with Sky's failure in full: hook never installed, zero modules contributed, linker resolved against `unreachable!()` bodies, **build exited 0**, surfaced hours later as an unrelated-looking runtime panic. Linked forward to §26b.4, since it's the same principle one layer up.

**§3.2 correction block** — `frontend_rust_rustc` is not a crate and deliberately so, with the **split trigger recorded**: split when `per_instance_mir`/`layout_of` land. Also why deferral is cheap (the oracle is already `'tcx`-free in every signature, so the boundary needs no new abstraction) and why it was declined now (the interop tests need `frontend_rust`'s own test tree). §28 Phase 3's bullet struck through and pointed at it.

**§8.10 revision block** — two stale spellings flagged: the compiler-side gate, and `StubOracle`, which no longer exists.

---

That's everything from the list. The docs now carry the *why* behind the constraints, not just the constraints — which is the part that was only ever in the transcripts. Nothing committed; still on `8d40eff9d`, with `CLAUDE.md` and `.github/ci-guide.md` left as other people's work.

---

## 👤 User

which of these two docs capture the short term, medium term, or long term plans?

---

## 🤖 Claude

Good question — let me check one thing that bears on the answer.

---

## 🤖 Claude

Confirmed two gaps at the seam. Let me answer, and fix them since they're exactly what you're asking about.

---

## 🤖 Claude

Unchanged at 582/175/8 — docs only.

## The split

**Handoff (`synthesized-declarations-plan.md`) — short and medium term.**

- **Short:** §5.3 "Next, in order" (what to do next); §6 known defects; §2 current state and baseline.
- **Medium:** §9 (generic types → `Vec<int>()`, four steps in dependency order) and §10 (name resolution, designed and unbuilt); §5.1's 40-case corpus spans both.
- **Timeless:** §0 (how the work is run), §4 (verified facts), §8 (the @SMLRZ trap).

**Architecture doc — long term and the destination.**

- §28 is the phase plan to 1.0 (Phases 0–10, 3–5 years); §28.2/§28.3 are the v2 and 1.0 deferrals; §29 the open questions. The other 28 chapters describe the *end state* rather than a schedule.

**§3's phase order is the bridge** — your alternating sequence, which is the only thing tying "what we do next" to "where this ends up."

## Two gaps at that seam, now fixed

Asking the question surfaced them.

**Arch §28 didn't know about the alternating order.** It read as a linear Phase 0→10 list, so anyone planning from it alone would conclude Phase 0 completes first. It now carries a banner: the order is deliberately alternating, and **where the work actually is** — a Phase-3-shaped driver exists while almost none of Phase 0 does (no LLVM port, no `GlobalState` refactor, no arena migration, no symbol audit). That's the order working, not drift.

**A stated prerequisite is outstanding, and the reason it's fine had been lost.** §28 Phase 0 says the arena-ownership migration *"must land before Phase 3's LangCallbacks integration."* It hasn't, and we did that integration anyway. The justification was worked out in convo-8 — every arena is a **function local created inside `after_expansion`**, so `'tcx` outlives them and the nesting is sound — and it was going to be recorded in the frontend plan's §9, which then got superseded. So it existed nowhere. It's now in §28 along with the real trigger: **it becomes hard-blocking the moment work spans more than one callback** (a cache write at `after_rust_analysis`, or providers at codegen time), which is what to watch for rather than the phase number.

Both docs now say which horizon they hold, and that the handoff wins on ordering while the architecture doc wins on destination.

---

## 👤 User

so, if the next session doesnt read these conversations, they'll remember where we're going, and all of the mindset/principles ive told you?

---

## 👤 User

can you make sure that we've written down things such that if the next session doesnt read these conversations, they'll remember where we're going, and all of the mindset/principles ive told you?

---

## 🤖 Claude

Let me check what a fresh reader actually gets, rather than assume.

---

## 🤖 Claude

Found the gap. A fresh reader learns the arc, the horizons, and the process — but never what the whole thing is *for*.

---

## 🤖 Claude

Suite unchanged at 582/175/8.

## What I checked

I went through the principle list item by item and asked, for each: would a fresh session reading only these two docs find it, without knowing it exists? Everything from the earlier catalogue has a home a reader arrives at naturally — plan §0 for the working rules, arch §1.5.6 for the containment principles, arch §26b.5 for the testing philosophy and its reasoning, plan §1 for methods-aren't-special, plan §3 and the arch §28 banner for direction.

## The one real gap, now closed

**The handoff never said what the endeavour is for.** It opened with *"the working document for the Rust-interop arc"* and went straight into process. §1 is "The design, in one page" — but that's the *current mechanism*, not the destination. A fresh session would have known exactly how to work and roughly nothing about where it was heading; they'd grow the corpus competently without understanding why any of it matters.

New **"Where this is going"** at the very top, before the process section, covering: the endeavour (both directions, first-class types, live `TyCtxt`, and eventually Vale items in rustc's own monomorphization graph); the two non-negotiables that shape everything (our C++ backend owns every byte of emitted IR — not rustc's codegen, not its MIR; and Vale-private items stay invisible to rustc); an honest statement of where that stands (typing pass works against real rustc with an empty core diff — nothing downstream of typing exists at all); and the near-term goal with the pointer to §3's alternating order.

It's deliberately short and points at arch §1 and §2 for the full framing, including why a pre-pass can't work.

## What a fresh session now gets, in reading order

1. **Where this is going** — the destination, in a page.
2. **§0 How this arc is run** — the eight working rules, each with the outcome it changed.
3. **§1 The design** — the current mechanism and why the previous one failed.
4. **§2–§4** — state, locked decisions, facts worth not rediscovering.
5. **§5** — testing, the 40-case corpus, and what's next.
6. **§6–§10** — defects, blockers, the two medium-term arcs.

With the arch doc as the destination and the authority, carrying its own banner about where the phase list is stale.

The thing I can't verify from inside this session is whether the framing actually lands for someone without the context — the honest test is a fresh session reading it cold. If you want, that's a cheap thing to check at the start of the next one before doing anything else.

---

## 👤 User

youve captured my thoughts on testing, and my other principles?

---

## 🤖 Claude

Let me actually check rather than tell you yes.

---

## 🤖 Claude

Two gaps in §26b.1, and one thing it claims that isn't true. Fixing.

---

## 🤖 Claude

Suite unchanged at 582/175/8.

## Answer: mostly yes, but checking found three things

I went through what you actually said about testing rather than trusting my memory of it. Present and quoted: dark-box spirit and *why* (brittleness, architectural inertia); per-pass boundaries as a concession to scale, not a preference; `collect_*` test-only with your reason (a generic walker makes expensive traversals too easy to write); no fixture oracle; "a lot of them" in both tiers; every case in both; tier 1 asserts on the typed AST; the oracle log's job is vacuity; 32 as the bar with 40 named; TDD; serialization stays out of core, `serde_json` not von.

Three things weren't right:

**1. Tier 2's rule was implied, not stated.** §26b.1 said "the program runs and produces 42" — but not that tier 2 asserts on output **and nothing else**, nor your reason: *"thats how it currently works in vale compiler tests too."* That precedent is what makes it sufficient rather than a compromise, and it was missing.

**2. "One corpus" had no mechanism.** The arch doc asserted the tiers share a corpus without saying what a case *is*. Now: `(Rust fixture, Vale program, expectation)`, expectation being "compiles and returns N" or "fails with E", program in a shared constant both runners read — and explicitly **no on-disk schema until something needs one**.

**3. The arch doc claimed something untrue.** It said tier 1's *"fixtures are shaped so tier 2 consumes them unchanged."* They aren't — the Vale programs are inline string literals inside `#[test]` functions, so only the Rust crates are shared. I'd flagged this to you as a divergence but never fixed the doc that asserts otherwise. It now carries a warning, and **"hoist each program to a shared `const`" is step 1 of §5.3**, ahead of growing the corpus — cheap at nine cases, expensive at forty.

## On the other principles

Those I'd already verified when you asked me to add the ten: containment and its four rules, make-it-unrepresentable, delete corpses, honest data over special cases, concentrate cfgs, the core-change protocol, probes before claims, trigger-gated deferrals, the authority map, the cross-branch constraint, the four investigative moves, the sibling-tree trap, doc discipline, methods-aren't-special.

The one I'd treat as unproven is whether the framing *lands* for someone without our context. I can confirm the content is present and findable; I can't confirm it reads correctly cold. A fresh session reading both docs before touching anything is the actual test.

---

## 👤 User

methods-arent-special is a particularly important one that rust got wrong. also, that it got drops wrong in general

---

## 🤖 Claude

That's a bigger principle than either half — and it's the spine of this arc's design. Let me write it as one.

---

## 🤖 Claude

Suite unchanged at 582/175/8.

## Written as one principle, because it is one

New **arch §1.5.7, "Refuse special cases — the recurring shape."** These weren't two preferences; they're the same posture in six costumes, and the doc had four of them scattered without ever connecting them:

| special-cased | Vale's answer |
|---|---|
| non-generic vs generic | degenerate case of generic (§1.5.5, already there) |
| **methods vs functions** | a method **is** a function; the receiver is parameter zero |
| **drop vs functions** | drop **is** a function the language sometimes auto-calls |
| synthesized vs parsed declarations | synthesized is the degenerate case of parsed |
| `extern` as a denizen kind | `extern` is a **body kind** |
| absence as an object that answers nothing | absence spelled as absence |

Naming it matters because it **predicts** decisions rather than describing past ones. It's also this arc's entire design — `extern`-as-body-kind is the row that made everything else fall out. Harmonious actually spotted the family in convo-9 ("third costume, same principle") and it was never written down.

## On drop specifically

I laid out the five compounding ways Rust made destruction special, because "Rust got drops wrong" needs the *how* to be usable:

- **It's a trait**, so droppability becomes a bound question — and the bound doesn't answer it. The trap is concrete: Valen has two spellings of `drop`, only one satisfies `T: Drop`, but **both** make a type linear, so a `#[derive(Copy)]` gate written the natural way passes a file handle through. Rust has no analogue, which is exactly why Rust instinct misleads.
- **`Drop::drop` takes `&mut self`** — a workaround forced by drop being special, since you can't move out of a value being destroyed.
- **You can't call it.** `x.drop()` is an error; `drop(x)` is a different function that merely moves.
- **Drop glue is compiler machinery with its own rules**, not ordinary calls the rest of the compiler can see.
- **"Must be explicitly consumed" is inexpressible**, because drop always runs. `#[must_use]` is a lint; `mem::forget` is safe. The obligation Vale wants is precisely the one Rust's model can't hold.
- Plus unwinding, a hazard class of its own.

And Vale's inversion: ordinary function, ordinary overload resolution, one generic wrapper synthesized at scope end so **the monomorphization path never mentions drop** — and **drop *absence* creates the linear obligation**, not its presence. Also recorded that a `needs_drop` predicate isn't just unnecessary but *unanswerable* for a bare type parameter; Sky wrote one, found that out, deleted it.

Closed with the interop-specific consequence: a Rust type's `drop` is one more top-level declaration in the same store, same code path, same lookup — **there is no drop-shaped seam in `rust_interop/` and there should never be one.**

Plan §1 now carries both quotes and points at §1.5.7 with a standing instruction: read it before adding any construct that seems to need its own machinery.

---

## 👤 User

methods arent special, my drop thinking, and testing... what other wisdom did i give?

---

## 🤖 Claude

Going back through with "wisdom" as the filter rather than "decision" — here's what I find that isn't yet captured *as a principle*, only as an outcome.

## The four I'd add

**A. Don't resolve before you can.**
> *"we should be register **templates** this early. later on, when we try to call them, is when ITemplataT::Prototype things should be made."*

This is recorded as a design fact (§1 principle 3) but never as a principle. It generalizes hard: a resolved form built before the information that resolves it exists **cannot represent the general case** — a prototype must pick one instantiation, and a generic function has none. And the connection nobody has drawn: **this is the same idea as arch §2's interleaved monomorphization.** The whole architecture exists because a pre-pass can't know what it needs to know yet; minting prototypes at environment-build time was a pre-pass in miniature. Harmonious arrived at the same shape independently and calls it *pre-pass versus interleaving* — *"don't commit to an answer before the context that determines it exists."* Strongest of the four.

**B. Suspect conflation.**
> *"i think there are some concerns that we're conflating, that actually can be orthogonal, which let us avoid the collision problem, if we do things right"*

This fired **twice in one session** and both times took a problem from hard to nearly-free: the Problem A/B split (synthesized names vs user-written names), and the type-vs-function-vs-namespace split. In both cases I'd been treating a bundle as one thing and defending it as hard. The move is *"is this actually one problem?"* — and I'd note the tell: when a problem resists a clean answer, check whether it's two problems sharing a name.

**C. Ask the authority rather than reconstructing.**
> *"i was imagining that we'd ask rust what the thing's real canonical name is"*

Recorded as an outcome in §10.0, not as a principle. It's the same instinct that put us on `TyCtxt` instead of parsing rustdoc JSON the way ValeRuster did — and the same reason that repo could sidestep re-export canonicalization entirely: rustc had already walked the chain. General form: **when another system already holds the answer, taking it beats deriving it — and the derived version is where the bugs are.**

**D. Don't let a subsystem force a core design before you're ready to design it properly.**
> *"can you make this happen with only changes in rust_interop? i do eventually want to serialize typing pass outputs, but im not ready to design that into vale core properly quite yet."*

Distinct from "don't add tech debt." That's about *quantity*; this is about *timing and authorship* — a core design should be made on its own merits, when you're ready, not extracted under pressure from whatever needs it first. It's what saved typing-pass serialization from being designed around a test harness's convenience.

## Three sharpenings of things already present

**E. Containment enables delegation, not just tidiness.** Your stated reason for `docs/convos/rust_interop/` was *"so we can keep the main compiler sane, and let claude loose on just the rust interop part while keeping it contained."* §1.5.6 gives containment's cost/debt rationale but not this one — that a properly contained subsystem is one you can hand off and stop supervising.

**F. "Whose bug is this?"** — *"is that a bug in vale itself, or just with the rust stuff?"* asked before assuming. It correctly routed `dot_borrow` and the ref-peel gap to Vale2 rather than us building workarounds. §7 has the outcome, not the habit.

**G. Make the thing true rather than the checker tolerant.** §1.5.6 rule 3 currently says "prefer honest data over a special case," which is close but softer than what you actually did — you didn't ask for a *nicer* special case, you asked whether the struct could just *have* the attribute. The sharper form: when a check is failing, ask whether the data is wrong before making the check lenient.

---

A and B are the two I'd most regret losing — A because it's the arc's whole failure mode and it rhymes with the architecture, B because it's a repeatable move that saved a large piece of work an hour ago. Want all seven, or just A–D?

---

## 👤 User

capture A, B, E, F please

---

## 🤖 Claude

All four in. **573/175/8** default, **582/175/8** interop — docs only.

## A — arch §1.5.8, "Don't resolve before you can"

Its own section rather than a note inside §1.5.7, because the *tell* is different: §1.5.7 is about a construct that looks like it needs machinery, this is about a phase that looks like it can answer.

States it as **cannot represent**, not "is less efficient" — `fn_sig(item, &[])` at environment-build time works for `add(i32, i32)` and is structurally incapable of `pick<A, B>`. Names the two arcana that already forbade it (@ECSIIOSZ, @BDPFWDZ) and that both sibling trees tried it and abandoned it.

The tell is recorded as its own paragraph, because it's what actually caught this: **a component asking for a capability it shouldn't need at that phase.** Wanting `&mut CompilerOutputs` during environment construction — to register bounds belonging to instantiations that don't exist yet — is what your questioning pulled on.

And the rhyme with §2: **interleaved monomorphization is this same argument at whole-compiler scale.** A pre-pass can't enumerate what it needs because the concrete arguments come from downstream; minting prototypes early was that mistake in miniature. Closes with Sky's independent one-liner — *don't commit to an answer before the context that determines it exists.*

## B — plan §0.6, "Are these actually one problem?"

With both instances and the tell. Name resolution was one problem until it was two (we mint both ends vs. a user wrote it — and every argument against a key map applied only to the second). The collision problem was one until it was three (type lookup panics; function collection is plural and can't; namespace scope keys on argument type). The tell I'd want a future session to recognize: **you are defending a position rather than answering a question.** That was true of me both times.

## E — arch §1.5.6

Containment's second payoff, in your words: *"let claude loose on just the rust interop part while keeping it contained."* Framed as the reason the *docs* are consolidated and not just the code, with a usable test — judge a change by whether it keeps the property, since one that makes an interop question answerable only by reading the typing pass has cost something real even at zero core lines.

## F — plan §0.6, "Is that a bug in Vale itself, or just with the Rust stuff?"

With both routings it got right, the note that the ref-peel gap surfaced only because a probe hit it sideways, and why it's expensive in *both* directions — a workaround for someone else's bug becomes debt we own; a bug misreported as theirs wastes their time.

---

Skipped C, D, and G as you said. Nothing committed; still `8d40eff9d`, with `CLAUDE.md` and `.github/ci-guide.md` left alone.

---

## 👤 User

<command-message>fire-commit</command-message>
<command-name>/fire-commit</command-name>
<command-args>with experimental</command-args>

---

## 🤖 Claude

Reading the config first, as the skill requires.

---

## 🤖 Claude

Config: rebase-and-fast-forward on the `experimental` family, target `experimental`, gate is `cargo build` + both nextest backends. Starting pre-flight. **0a — external repos.**

---

## 🤖 Claude

Externals clean and current (0/0), no curate queue, submodules at pinned refs. Staging precise paths so the sweeps see the real diff.

---

## 🤖 Claude

24 files staged. Now the sweeps — **0b, 0c, 0d, 0e**.

---

## 🤖 Claude

0b clean (all hits are docs describing the sweep). Now **0d, 0e, 0f**.

---

## 🤖 Claude

0d clean, 0e no real hits, no new `#[ignore]`. Now the blocking gate — **0g**, the config's test command, unfiltered.

---

## 🤖 Claude

All 8 errors are in `src/bin/valec/`, and **none of those files are staged**. Let me confirm nextest can't start either, then measure the bar you ratified.

---

## 🤖 Claude

Pre-flight complete. **Stopping at 0g** — the gate fails, for the third time on this branch and for the same structural reason.

## The blocker

The config's test command starts with `cargo build --manifest-path FrontendRust/Cargo.toml`, which exits 101:

```
error[E0432]: unresolved import `frontend_rust::backend_ffi`   src/bin/valec/midas.rs:7
error[E0433]: could not find `pass_manager` in `pass_manager`  src/bin/valec/build.rs:252
                                                               src/bin/valec/frontend.rs ×6
```

All 8 errors are in `src/bin/valec/`, and **none of those files are staged** — `backend_ffi` and `pass_manager` are intentionally commented out of `lib.rs` by the onion arc. `cargo nextest run` can't build its targets either, so neither backend gate can start. This isn't a red suite; it's a suite that can't run, and it's identical to the two previous `fire commit` runs.

Against the bar you set for exactly this condition — *"typing-pass should build, and some typing pass tests should pass"*:

| | result |
|---|---|
| `cargo build --lib` | **exit 0**, 7 warnings, all pre-existing |
| `cargo test --lib` | **573 passed** / 175 / 8 — unchanged all session |
| `cargo test --lib --features rust_interop` | **582 passed** / 175 / 8 |

## Everything else is clean

| check | result |
|---|---|
| 0a Luz | clean, **0 ahead / 0 behind**, curate queue empty, only untracked `tmp/` scratch from June |
| 0a Guardian | clean, **0/0**, all 5 submodules at pinned refs |
| 0b `DO NOT SUBMIT` | no real hits — all matches are docs *describing* the sweep |
| 0c absolute paths | see below |
| 0d broken symlinks | none |
| 0e Guardian temp-disable | no real hits (docs only) |
| new `#[ignore]` | none |

**0f test delta — 9 added, 1 deleted, 0 modified.**

- **Added (8 new):** `an_empty_allowlist_makes_nothing_importable`, `reads_a_generic_signature_structurally`, `calls_a_method_on_a_rust_type`, `a_rust_value_bound_to_a_local_gets_a_scope_end_drop`, `declines_an_unrepresentable_signature`, `vale_source_can_name_a_rust_type`, `a_generic_rust_type_loses_its_arguments`, `a_fatal_rustc_error_costs_one_case`.
- **Deleted + re-added: `calls_a_rust_free_function`.** Not a coverage loss — it *moved* from `rust_interop_tests.rs` to `cases.rs` and was rewritten to run against a real `TyCtxt` instead of the fixture oracle. Strictly stronger: same assertions, real rustc. The fixture oracle it depended on is deleted in this commit.

**0c — absolute paths, all judgment calls I'd rather flag than silently "fix":** hits are inside the `convo-9` verbatim transcript (rewriting would falsify the record), and cross-repo references in the plan doc to `/Volumes/V/RustInteropReiImpl`, `/Volumes/V/ValeRustInterop`, and `/Volumes/V/Vale2/vcoord-handoff.md`. Those are *other checkouts on this machine* with no relative form — and they're deliberate: §0.7 exists to tell a future session where to look. Same category as the README cross-repo refs accepted last time.

## What's staged, and what isn't

**24 files.** The prior session's uncommitted generics work plus this session's harness, 9-case corpus, five dead-oracle-method deletion, `Source::rust()` removal, and the doc work (§0 working agreements, §1.5.6–1.5.8, §26b.4–26b.5, §9, §10).

**Deliberately not staged, both yours to place:** `CLAUDE.md` (manifest-sync regen, predates this session) and `.github/ci-guide.md` (untracked, from a prior session).

---

You've ratified this bar twice for this exact condition. Say the word and I'll run steps 1–5 — commit, rebase onto `experimental`, re-verify after the rebase, and ratchet. Or `fire commit temporary with experimental` if you'd rather it not move integration.

---

## 👤 User

proceed, fire the commit

---

## 🤖 Claude

Go. **Step 1 — external repos:** both clean and current (0/0), nothing to commit or push. **Step 2 — host commit.**

---

## 🤖 Claude

Committed `a96cea442`. **Step 4 — rebase onto `experimental`.**

---

## 🤖 Claude

`experimental` advanced by 2 while I was composing — expected. Parking the unstaged `CLAUDE.md` (not mine) across the rebase.

---

## 🤖 Claude

Rebase clean. Two commits came in, so **re-verifying after the rebase as a separate step**.

---

## 🤖 Claude

The rebase broke the interop config — an incoming commit changed a signature we call. Let me look.

---

## 🤖 Claude

`FunctionS::new` gained an `impl_bounds` parameter from the incoming `where implements(T, IShip)` work. Let me look at the call site.

---

## 🤖 Claude

`impl_bounds` slots between `rules` and `body`. Empty is the truth here, not a placeholder — worth saying why.

---

## 🤖 Claude

All 9 cases green. New numbers: **577/170/8** default, **586/170/8** interop — the 5-failure improvement is upstream's. The plan doc's baseline is now stale, and it says movement is "a stop, not a footnote," so let me correct it.

---

## 🤖 Claude

Both commits in. Checking state before the ratchet.

---

## 🤖 Claude

Tree clean but for the untracked `ci-guide.md`. **Ratcheting `experimental`.**

---

## 🤖 Claude

`fire commit with experimental` is complete.

## Landed

**`26791765e`** on `experimental-4`, ratcheted to `experimental` (`88599abe7..26791765e`). Divergence **0/0** both ways. Per the config's experimental-family model the local ratchet *is* the sync — nothing pushed.

Two commits:

- **`60e6c5e94`** — 24 files. Structural generic signatures, the tier-1 harness and 9-case corpus, five dead oracle methods plus `FixtureOracle`, `seam.rs`, `Source::rust()` and the driver's `check()` deleted, structured assertions, and the doc work.
- **`26791765e`** — rebase fallout, see below.

## The rebase earned its keep

`experimental` had advanced by two commits — the `where implements(T, IShip)` postparse restoration and the parse/solver error-discarding fixes. Re-verifying afterward as a separate step, which the skill insists on, caught a real break:

**`FunctionS::new` gained an `impl_bounds` parameter**, so the interop config stopped compiling. Fixed by passing `&[]` — and worth stating that this is the *truth* rather than a placeholder: rustc discharges a Rust function's own trait obligations, and we deliberately read no predicates, which is exactly why a signature needing one (`first<I: Iterator> -> I::Item`) is declined outright instead of imported with an unsatisfiable bound. Had I chained rebase-and-ratchet in one line, a non-compiling interop config would have landed on `experimental` for other TLs.

## Verified, twice

| | pre-rebase | post-rebase |
|---|---|---|
| `cargo build --lib` | exit 0 | exit 0, 7 warnings, all pre-existing |
| `cargo test --lib` | 573 / 175 / 8 | **577 / 170 / 8** |
| `cargo test --lib --features rust_interop` | 582 / 175 / 8 | **586 / 170 / 8**, all 9 cases green |

**Five failures cleared — upstream's, not ours.** Since the handoff calls movement in either direction "a stop, not a footnote," I re-baselined it in the second commit with the attribution, so the next session doesn't read 577 as a regression.

`cargo build` and both nextest backends still can't run — `src/bin/valec` references the intentionally-unlinked `backend_ffi`/`pass_manager`. Unchanged by this commit; none of those files were touched.

## Externals

Both **clean and current, 0 ahead / 0 behind** throughout — nothing to commit or push. Luz's curate queue was empty; all five Guardian submodules sit at their pinned refs. **Nothing moved on disk in Luz or Guardian, so no Guardian restart is needed.**

## Two leftovers, deliberately not mine

```
 M CLAUDE.md              ← manifest-sync regen, predates this session
?? .github/ci-guide.md    ← CI doc from a prior session
```

`CLAUDE.md` blocked the rebase, so I parked it in a stash and restored it afterward rather than folding someone else's in-flight work into an interop commit. `Luz/tmp/` is untracked scratch from June, as it's been all along.

So the tree isn't fully clean — three items, all pre-existing and none mine. That's the one respect in which this falls short of the skill's clean-tree handoff, and I'd rather flag it than sweep other people's changes into my commit.

---

## 👤 User

time to wind down, any advice for the next session?

---

## 🤖 Claude

A few things, mostly about traps rather than tasks.

## Start by reading both docs cold — and notice where they fail

The plan doc's "Where this is going" then §0, then the architecture doc's §1.5.6–1.5.8. That's the direction and the principles.

But here's the part worth acting on: **that framing has never been tested on someone without context.** I wrote it, so I can confirm the content is present and can't confirm it lands. If something reads as obvious-in-hindsight, or as a rule without a reason, that's data — fix it while you're the only person who'll ever have fresh eyes on it.

## Do §5.3 step 1 first, because it's decaying

Hoist each case's Vale program to a shared `const` with its expected return value. Two reasons it's first:

- **It's cheap at nine cases and expensive at forty.** Every case added before it is another one to migrate.
- **Arch §26b.1 currently claims a property the code doesn't have** — it says tier-1 fixtures are shaped so tier 2 consumes them unchanged, and they aren't. There's a warning banner on it, but a doc that lies about code is the worst kind of stale, and this is the cheapest window to make it true.

## Three traps that will actually bite

**The numbers just moved, legitimately.** Baseline is **577/170/8** default, **586/170/8** interop — five failures cleared by upstream's `where implements` work, not by us. The handoff says movement in either direction is a stop, so if you measure something different, *find out whose it was* before treating it as a regression. `experimental` is active; this will recur.

**`a_generic_rust_type_loses_its_arguments` is a green test asserting a defect.** `Holder<i32>` and `Holder<bool>` intern to the same kind. When generic types land, **invert that assertion** — don't delete it. It's the regression test for the fix, and the comment says so.

**`fixtures_broken_rust/` contains Rust that doesn't parse on purpose.** Anyone adding a "check the fixtures compile" step must skip it, or they'll break the case that proves a fatal rustc error costs one test rather than the run.

## Expect the ground to move under you

Today's rebase added an `impl_bounds` parameter to `FunctionS::new` and broke the interop config — caught only because the skill makes you re-verify after a rebase as a separate step. **That is exactly the surface Vale2 is working on**: our synthesized declarations sit directly on postparse structures they're actively changing. Rebase often, re-verify always, and read `/Volumes/V/Vale2/vcoord-handoff.md` before touching anything near the solver or postparse.

## A calibration note about the docs themselves

Treat them as **authoritative on decisions and unreliable on behaviour.** Three claims I'd written were refuted the moment someone ran the code — *"a bare `&Moo` synthesizes a region rune"*, *"Vale source can't name a Rust type"*, *"generic types aren't supported"* (they compile and silently drop arguments, which is worse). A fourth: rustc's fatal path doesn't `exit`, it unwinds.

So: what was *decided* is trustworthy. What the compiler *does* is worth a ten-minute probe before you build on it. Every probe this session found something different from what it went looking for.

## What not to do

**Don't route around a needed core change.** Twice the answer to "what does this need from core?" turned out to be *nothing* — but only because someone asked. The poison hook for panic-vs-decline and the `QualifiedCodeName` variant are both waiting on the architect; building interop-side workarounds for either is how the debt starts.

**Don't deepen the dependence on ambient visibility.** Rust functions currently live in a global top-level store that `PackageEnvironmentT` unions everywhere. Vale2's dispatch redesign replaces exactly that with argument-type-scoped namespaces. It's harmless at a hand-written allowlist and shouldn't be built on.

## The obvious next work

Generic Rust types (`Holder<int>` before `Vec`, §9 step 2) is the biggest unblocked capability, and the `@ATAFLBZ` fix is in the same file — do them together so the generic work is built on `DefId` identity rather than string matching you'd unpick later.

---

One loose end: this session has no transcript export, unlike convo-8 and convo-9. Less costly than usual — the substance went into the docs deliberately — but the reasoning behind the def-path split and the Vale2 dispatch reading exists only here.

---

## 👤 User

<command-message>close-session</command-message>
<command-name>/close-session</command-name>

---

## 🤖 Claude

Wrapping up. Docs are current and committed in `26791765e`. Exporting the conversation.

---

## 🤖 Claude

Belongs in the `rust_interop/` subtree with its siblings. Renaming and updating the index.

---

## 🤖 Claude

Wrapped up. Target confirmed as `65054 claude`.

## Session close state

**Landed and synced:** `26791765e`, with `experimental` ratcheted to match, 0/0 both ways. Suites at the new baseline — **577/170/8** default, **586/170/8** interop, all 9 interop cases green.

**Docs are current.** The plan and architecture docs were part of the commit, and the baseline was re-measured after the rebase so nothing claims a state the tree doesn't match.

**Conversation exported** as `docs/convos/rust_interop/convo-10-test-corpus-and-arc-principles.md`, moved into the `rust_interop/` subtree with its siblings and added to the README index.

## Left uncommitted, deliberately

```
 M CLAUDE.md                                                    ← not mine, manifest-sync regen
 M docs/convos/rust_interop/README.md                           ← convo-10 index row
?? docs/convos/rust_interop/convo-10-test-corpus-and-arc-principles.md
?? .github/ci-guide.md                                          ← not mine, prior session
```

The export and its README row are mine and ready to land — but `fire commit` was authorized for the previous turn's work and completed. A fresh commit now isn't covered by that phrase, and the repo rule is absolute about it, so I've left them staged-ready rather than assuming. They'll fold into the next `fire commit` cleanly.

Closing the session.

---

