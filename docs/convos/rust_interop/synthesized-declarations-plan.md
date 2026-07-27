# Rust interop — state, plan, and handoff

**Start here.** This is the working document for the Rust-interop arc: where the design landed, what
is in the tree right now, what is next, and what is blocked on whom.

Written 2026-07-25 when the oracle-answers-per-call design was abandoned; rewritten 2026-07-26 after
generics, the method/function seam collapse, and the testing experiments.

Companions: `vale-rust-interop-architecture.md` is **authoritative** on architecture (see §8.10's
revision block and §26b for the parts this arc changed). `rust-interop-frontend-plan.md` and
`rust-interop-callout-map.md` are largely superseded and carry banners saying which parts still hold.

## Where this is going

Read this before the sections below, because everything in them is a step toward it and none of them
says it.

**The endeavour is that Vale source uses Rust crates directly, and Rust source uses Vale items —
both directions, as first-class types, not `extern "C"` FFI.** `import rust.std.vec.Vec`, then
`Vec<int>` is an ordinary Vale type. Vale's typechecker reads Rust signatures from a **live
`TyCtxt`** rather than from generated bindings; later, Vale-defined items appear in **rustc's own
monomorphization graph** so that generics crossing the boundary in either direction resolve
correctly. Arch §1 and §2 are the full framing; §2 in particular argues why a pre-pass cannot work
and interleaving is required.

**Two things are non-negotiable and shape everything.** Vale's **C++ backend owns every byte of
Vale-emitted LLVM IR** — we are not using rustc's codegen or its MIR (arch §1.7, §5). And
**Vale-private items stay invisible to rustc** — the surface we publish is proportional to what the
user explicitly exported, never to Vale's whole type universe (arch §1.7, §9.4).

**Where that stands today.** The typing pass can typecheck a Vale program that calls a Rust free
function, calls a generic Rust function at concrete types, holds a Rust type obtained from a
signature, calls a method or associated function on it, drops it, imports from two crates at once,
and uses a **generic** Rust type at two different arguments — all against a real rustc, with a core
diff that is a net **deletion**. 33 corpus cases. Nothing downstream of typing exists: no
instantiator, no codegen, no linking. The interop build deliberately does not even link the C++
backend yet.

**What is still missing at the typing layer**, in rough order of how much it costs: a Rust type
whose name collides with another (§10, and nobody else has solved it either — §10.9b); scope-end
drop of a *generic* citizen (Vale2's, and pure Vale has it too); unrepresentable types panicking
where they should decline (§6, now unblocked); and reaching a type in a nested module at all, which
is what `Vec` needs (§9 step 1).

**The near-term goal** is a typing-pass surface broad enough to trust — the corpus in §5.1, and the
remaining steps toward `Vec<int>()` in §9 — and then the LLVM 16 → ~21 port, which is what unblocks
everything after typing. §3 has the full alternating order.

---

**Which doc holds which horizon.** This one is **short and medium term**: §5.3 is what to do next,
§6 is what is broken now, §9 and §10 are the two medium-term arcs (generic types up to `Vec<int>()`,
and name resolution). The architecture doc is the **long term and the destination**: §28 is the
phase plan out to 1.0, §29 the open questions and v2 deferrals, and the other 28 chapters are the
end state rather than a schedule. §3's phase order is the bridge between them, and where the two
disagree about *ordering*, this doc is current — arch §28's phase list predates it and carries a
banner saying so.

---

## 0. How this arc is run

Not process for its own sake — each of these was stated or enforced during the arc, and each has
changed an outcome at least once.

### 0.0 What "core Vale" means

§0.1's protocol is scoped to a specific body of code, so the scope has to be stated. **Core Vale
is:**

- **the typing pass** (`FrontendRust/src/typing/`), except:
  - anything under a `rust_interop/` folder — this arc's own territory, worked in freely;
  - **patterns** — `typing/expression/pattern_compiler.rs` and `typing/rune_typing/patterns.rs`. Out
    because it is *surface*: the architect is content for edits to land there directly, since what a
    change there can reach is small and legible.
- **the instantiating pass** (`FrontendRust/src/instantiating/`);
- **the backend** — `FrontendRust/src/backend_ffi/` and the C++ `Backend/` behind it.

**Both edges are easy to get wrong, in opposite directions.** The instantiator and the backend are
**in**, even though every worked example of the protocol so far has been a typing-pass file — a
session generalising from the examples will edit them freely and be wrong. Patterns is **out**, even
though it sits inside the typing pass — a session applying "typing pass = core" will stop when it
need not.

Everything else in the tree — parsing, postparsing, the solver, simplifying, the final AST — is
neither core in this sense nor ours. Nothing in this arc has needed to touch it; if something does,
that is itself worth surfacing.

**"Core Vale" is not "core IR."** Arch §8.10 uses the latter for `names`/`types`/`interner`
specifically, in the Option A argument, where it carries a different claim — that no rustc type
reaches those three files. It is a strictly narrower set and it keeps its own name. Same word, two
scopes; don't collapse them.

### 0.1 The core/interop split is a protocol, not just a layout

Work inside `typing/rust_interop/` and the interop test subtree proceeds freely. **A change to core
Vale — §0.0 defines exactly what that covers — stops and is brought to the architect verbatim, the
exact hunks, before landing.** Precedents: the two `compiler.rs` hunks for the import kickoff, and
the `get_imprecise_name` arm.

The corollary matters more than the rule: **when a core change is needed, ask for it rather than
routing around it in interop.** A workaround built to avoid asking is exactly the debt arch §1.5.6
exists to prevent. Twice the answer to "what core change does this need?" turned out to be *none* —
`StructDefinitionT` needs no `StructS`, and the `extern` attribute removed the last guarded arm —
but that was found by asking, not by assuming.

### 0.2 Probes before claims

Three statements this document carried were refuted the moment someone ran the code, each in under
ten minutes:

- *"A bare `&Moo` param synthesizes an implicit region rune"* — false.
- *"Vale source cannot name a Rust type"* — it can (case 38).
- *"A generic Rust type isn't supported"* — worse; it compiles and **silently drops its arguments**
  (case 40).

A fourth was found the same way: rustc's fatal path was assumed to `process::exit`; it **unwinds**.

The rule: if a claim about compiler behaviour is cheap to check, check it before writing it down.
Cases that came out of probes have consistently found something *different* from what they went
looking for.

**Probe past the first satisfying answer — especially when that answer is "not blocked."** The
generic-types question was answered three times in one day, confidently and differently each time,
and the failure was identical each time: probe one layer, find something clean, stop. *"The rule
shape is `LookupSR` + `CallSR`, which is ours, so we are not blocked"* was true and useless — what
the solver did with the **registration** was one layer further down and inverted the conclusion. An
answer that lets you stop looking is the one to distrust; "we're blocked" at least keeps you honest
about not knowing.

**And expect the failure to be silent.** Every defect this arc found in its own work failed
quietly rather than loudly:

| what | how it failed |
|---|---|
| a citizen registered as a finished kind | `solve_call_rule`'s `Kind` arm binds the result and **ignores the arguments** |
| a new oracle method not forwarded by `LoggingOracle` | the decorator inherits the default body and answers *empty* |
| lowering a citizen eagerly | the type argument is dropped; two instantiations intern alike |
| a test compilation with no builtins | zero candidates, reported as `rejected_callee_to_reason: []` |

None of these errored. Each produced a plausible wrong answer or a no-op. **So when something is
subtly wrong, suspect a path that *succeeded* rather than one that failed** — and per arch §1.5.9,
when you find a dispatch that accepts a malformed input and quietly does nothing, make it loud in
the same change or your fix is one mis-registration from being undone.

### 0.3 No fakes or mocks, ever — dark-box and end-to-end only

**Two kinds of test exist here: dark-box (source in, structured outcome out, at a *pass* boundary)
and end-to-end (run the program, check its output).** Nothing else — no fakes, no mocks, no stubs,
no test doubles, and no test that reaches inside a pass to call one function with hand-built
arguments. A test drives the real component or it does not exist.

This is a standing rule, not a cost/benefit call to re-run per case. **A fake encodes what you
currently believe the real thing does**, so it passes exactly when your belief is self-consistent —
which is when it teaches you nothing — and keeps passing after the real behaviour moves, which is
when you needed telling. It also freezes the interface it doubles, which is the architectural
inertia the architect objects to in unit tests, one layer down. Arch §26b.3 has the full statement
and §26b.5 the reasoning.

Live proof from this arc: **every probe that found something real found it by running the actual
compiler** — that rustc unwinds rather than exits, that Vale source can already name a Rust type,
that generic types compile and silently drop their arguments, that a Vale/Rust name clash is a
designed error while a type clash is a panic. Not one of those was reachable from a canned table.

The arc's own fake, the `FixtureOracle`, is deleted. Read its obituary in §26b.3 carefully: its
*specific* weaknesses are why it was easy to remove, not why it was wrong. "Then write a cheaper
fake" is the wrong lesson.

Two clarifications worth having, because both look like violations and are not:

- **A decorator over the real thing is fine.** `LoggingOracle` wraps the genuine `TyCtxtOracle` and
  records what was asked; the answers still come from rustc. Recording and tracing wrappers change
  nothing about what is computed.
- **Absence is spelled as absence, not as an object that answers nothing.** `Oracles::none()` is an
  `Option` that is `None`. A no-op implementation is a mock with a sadder face.

### 0.3b An inherited conclusion is not a ratified one

**This document and the architecture doc were drafted by filling a skeleton from Harmonious/Sky's.
Some of what came across was never put to the architect, and at least two things arrived without the
condition that justified them.** Both were found on 2026-07-27, both by asking "why do we believe
this?" rather than by anything failing.

- **Arch §1.7's no-inference rule was never a Vale rule.** A near-verbatim transcription of Sky's
  `rust-interop-architecture.md:201`, whose closing sentence — *"Sky inherits this discipline from
  toylang's experience with inference-related complexity"* — was **dropped in the copy**. It has no
  Q-ref while every neighbour in that list cites one, is absent from convo-9's inventory of
  inherited §1.7 items, appears in no Vale design conversation, and has no Valen backing. It is
  **struck**. It had been cited in this document to justify *not* fixing a real defect.
- **The `__vale_drop` by-pointer rationale may not transfer.** It is properly recorded and real —
  Sky's `Vec<Vec<Widget>>` double-free — but the argument is *"by-value + unconditional per-`let`
  emission + **the compiler doesn't track moves** = double-free."* Vale has linear types and move
  semantics. Whether we inherit that premise is addressed nowhere.

The shape to watch for: **a conclusion imported without its condition.** It reads as settled because
it is phrased like the settled things around it, and the phrasing survives the copy while the
argument does not. When a rule blocks work, check its provenance before obeying it — the ones with
no Q-ref are the candidates.

### 0.3c Uphold properties with the type system first; a lint is the last resort

**The ordering is arch §1.5.6 rule 4** and it is an order, not a menu: the type system and API shape
(the violation cannot be written) beats a loud runtime failure (it cannot survive a run) beats an
ordinary test (it cannot survive the suite) beats a lint over our own source. Prefer sealed
constructors, obligation tokens, private fields, and signatures that cannot name the wrong thing —
`docs/skills/type-enforced-apis.md` is the how.

**A lint is the weakest of the four**, dodgeable by construction — `args.len() == 0` slips past a
matcher keyed on `is_empty`, and the allow-marker is an escape hatch anyone can write — and its
failure mode is the bad one: when the code moves, it stops matching and goes *quiet*, so everyone
keeps believing the property holds. A weak check that looks load-bearing is worse than none.

**The discriminator is loud-versus-silent.** A property whose violation fails loudly is already
guarded by the suite; a lint there is redundant machinery. Only a *silent* failure needs a
mechanism, and even then rule 4's order applies.

**Worked both ways, this arc:**

- **@NNGZ — no lint, and the instinct to write one was wrong.** Violating it (*"why would I apply
  zero arguments?"*) cost **twelve corpus cases at once**, immediately, with a legible message. Loud.
  The plan carried an item to mechanize it, on Harmonious's counsel that a rule is only what you
  believe while a mechanical check is what actually stops you. That was quoted approvingly here
  without noticing it offers two options and omits the one that beats both. **Item dropped 2026-07-27.**
- **@ATAFLBZ — silent, and the real fix was deletion, not detection.** Two crates' same-named types
  interning to one Vale kind produced no error anywhere. What fixed it was **deleting
  `resolve_method` and `resolve_function`**, the two functions that turned a human-name string into
  identity; once no such function exists there is nothing to catch. The lint added afterwards guards
  against reintroducing that shape, which is a real but much thinner benefit than it was first
  written up as.

**Two rules that survive, about any check:**

- **Validate it by making it fail.** After building the @ATAFLBZ lint we injected a violating line
  to confirm it fires. The *first* injection was invalid Rust, so the build died before the test ran
  — which reads as a pass. A check validated only by "the suite is green" is not validated, and one
  observed to fail *for the wrong reason* is worse, because now it is trusted.
- **Check the mechanism of a failure, not the fact of it.** The general form of §26b.4's vacuity
  rule: a negative control degenerates whenever the pass and fail branches become
  indistinguishable.

### 0.4 Deferrals are trigger-gated, never vague

Scope pruning is the architect's and it is aggressive — *"lets focus on only the things that block us
from that goal."* But a deferral names its trigger: *"if we run into a collision, we should work on
qualified names."* Where a trigger exists, there should be a case pinning the current behaviour so
the trigger is observable rather than theoretical.

### 0.5 Who is authoritative on what

- **Valen (`valen-design-1.md` / `-2.md`) is the language specification.** One architect owns Valen
  and Vale; a contradiction means the doc is behind a ruling, not that two authorities disagree.
- **Vale2 owns core Vale (§0.0) and its semantics.** `dot_borrow`, `is_type_convertible`, the
  overload/dispatch redesign, `convert()` unification are theirs; we sequence behind them and route
  findings over. Those examples are all typing-pass because that is where we have met them — the
  ownership covers the instantiator and backend too. Their handoff is
  `/Volumes/V/Vale2/vcoord-handoff.md`.
- **Harmonious/Sky is evidence, not authority.** *"we'll be using their prototype as a signal for
  **what works**, but not necessarily **whats best**. keep an eye out for things we can do better
  than they did."* Their operational scars have repeatedly been worth more than their conclusions —
  and several times what they "taught" us turned out to be in our own architecture doc already,
  because they helped write it.

**Correspondence is two-way, and the reporting half is the half that pays.** Writing to the sibling
sessions produced an unusual share of this arc's findings, and not mainly from the questions:

- We reported the decorator hazard we had just walked into. Harmonious went and checked their own
  tree and **found it live there** — the one method with a default body in their consumer trait is
  the hook their entire codegen contribution flows through, so a decorator that forgot to forward it
  would contribute zero modules and fail at link with no diagnostic. They would not have looked.
- We reported `+` resolving zero candidates as a possible Vale defect. Vale2 **pushed back rather
  than accepting it**, and were right: our harness supplies no builtins at all. Withdrawn, and the
  correction taught us we can have arithmetic in a corpus program whenever we want it.
- We asked how they handle a generic foreign type's destructor. The answer inverted our framing —
  *the synthesizer is the caller with the **most** information* — which is what turned an open
  question into the specific divergence we could hand to Vale2.

So: **report findings, not just questions, and make them refutable.** A report with the file, the
line and the mechanism can be checked and contradicted; a report with a conclusion can only be
believed. Two of the three above landed because the recipient could verify them, and one of those
verifications went against us.

### 0.6 A change must not cost other branches anything

Several branches track `experimental` and build the typing pass. **Interop work must leave their
build and test exactly where it found them.** The concrete bar, set by the architect: *"typing-pass
should build, and some typing pass tests should pass"* — they run `--lib`, because plain
`cargo build` already fails on `src/bin/valec` for onion-arc reasons that predate us.

The case that made this a rule: **`rust-toolchain.toml` is deliberately not pinned to `rustc-dev`.**
Adding it to `components` makes *every* `cargo` invocation in the repo, on every branch, for every
developer and CI job, ask rustup to ensure a hundreds-of-MB component nobody but interop needs — and
if it were ever unavailable for the pinned nightly on some target, every cargo command in the repo
breaks rather than just ours. Interop developers run one `rustup component add` instead. **Do not
"tidy this up" by pinning it.**

The general test before landing: for each changed file, why can this not affect a build with the
feature off? Feature-gated items and `build.rs` reads of `CARGO_FEATURE_RUST_INTEROP` are inert;
anything in the toolchain pin, or unconditionally compiled, is not.

### 0.7 Moves that keep finding things

Not general advice — each of these is a question the architect asked that changed an outcome, and
each is repeatable.

- **"Why does this need that capability?"** The single highest-value question of the arc. Asking why
  `rust_package_stores` took `&mut CompilerOutputs` — then what exactly it did with it, then why it
  called `add_instantiation_bounds` — is what exposed that prototypes were being minted at
  environment-build time, which is what made generic externs unrepresentable. **A component asking
  for a surprising capability is usually doing something at the wrong time.**
- **"This special case hints we're not seeing something."** The `Vec::new()` guard in
  `get_param_environments` returned the same value for "no methods exist" and "methods exist
  elsewhere". Pulling that thread produced the import-materialization design, showed the guard was
  speculative dead code, and led to the shape the arc now has.
- **"Are these actually one problem?"** Suspect conflation whenever a problem resists a clean
  answer. This fired **twice in one afternoon**, and both times took something that had been argued
  as hard down to nearly free. *Name resolution* was one problem until it was two — a synthesized
  declaration naming a type (we mint both ends; a def-path key matches by construction) versus user
  source naming one (the only place re-exports and precedence bite), and every argument against a
  key map applied only to the second. *The collision problem* was one problem until it was three —
  type-name lookup, which panics; function candidate collection, which is plural and cannot; and
  namespace scope, which the dispatch redesign keys on argument type. **The tell: you are defending
  a position rather than answering a question.**
- **"Is that a bug in Vale itself, or just with the Rust stuff?"** Ask before assuming, every time
  something breaks near the boundary. It correctly routed `dot_borrow` and the
  `get_param_environments` ref-peel gap to Vale2 (§7) instead of us building workarounds for
  language-level holes — and the second was found only because a probe hit it while testing
  something else. Getting this backwards is expensive in both directions: a workaround for someone
  else's bug becomes debt we own, and a bug reported as theirs that is ours wastes their time.

  **The same question in its other form: "is this a Vale object or a Rust object?"** Ask which side
  of the boundary a *thing* sits on before reasoning about its semantics, exactly as you ask which
  side a *defect* sits on. One question ended several turns of circling on drop: Vale has no
  drop-in-place — a Vale value is **moved into** `drop(self T)` — while a Rust-backed value is
  moved into an extern drop whose *body* destructs in place, because Vale does not own that
  destructor. Arch §15.7 had been applying one mechanism to both, and no amount of reasoning about
  the mechanism was going to surface that; only asking what was being dropped did. When something
  at the seam looks wrong, check whose object it is before checking whether the machinery is right.
- **"Does this help Vale outside interop?"** Applied to qualified names, it produced §10.8 — Vale's
  own name story (make `import` bind a name; turn the multiplicity panic into an ambiguity error) is
  worth more than the interop half and needs no resolver. A mechanism that only interop wants should
  be suspected; one the language wants anyway is usually the right shape.
- **"I don't recognize this rule — find out why we added it."** Not *is this rule right*, but
  *where did it come from*. Asked of arch §1.7's no-inference bullet, the answer was: nowhere. It
  was a near-verbatim transcription of Sky's, its rationale sentence dropped in the copy, with no
  Q-ref, absent from convo-9's inventory of inherited items, and no Valen backing — and it had been
  cited in this document to justify *not* fixing a real defect. One background agent, and a rule
  that should never have existed was struck.

  **The trigger is specific and worth recognizing: a rule you do not remember adopting, phrased
  with the same confidence as the ones around it.** That confidence is what makes an inherited
  conclusion read as a ratified one (§0.3b). The cheap check is provenance — a Q-ref, a convo, a
  commit — before obedience. Rules with none are the candidates.
- **"What does rustc do?"** Four findings that changed decisions: `visible_parent_map` is a lossy
  BFS kept *only* for diagnostics (which killed the full-path key map); `NameResolution`'s two
  `Option`s make precedence a struct field rather than a comparison; there is no `Res::Module`, so a
  namespace value type isn't needed; and foreign modules populate **on first touch**. `~/rust` is a
  full checkout and an agent sweep of it is cheap.
- **Check the architecture doc before proposing an architecture change.** A proposal to gate the C++
  backend away was made and was simply wrong — §1.7 and §5 already covered it, in detail. The doc is
  authoritative; read it before contradicting it.

### 0.8 The sibling implementations, and the trap in reading them

Two other Rust-interop implementations exist on this machine. Both are useful, and one is dangerous
to read carelessly.

- **`/Volumes/V/RustInteropReiImpl`** — the more recent (last activity 2026-06-09), and **a worktree
  of a branch of *this* repo** (`rust-interop-reimpl`; also `origin/master-with-rust-interop-reimpl`).
  Its `FrontendRust/` is the same language and largely the same code as ours, so its findings
  transfer directly. It is where the `extern`-as-body-kind design and working extern generics live.
- **`/Volumes/V/ValeRustInterop`** — the older Scala ancestor (2026-05-04, no git history). Its value
  is *archaeology*: it preserves abandoned experiments as commented-out corpses — the
  `OpaqueStructMemberT` blob-member design, `isRustOpaqueType()` gated on a package coordinate, and
  the `lift` flag that baked Rust's name shape into the typing pass and was rolled back (§8).

**►► THE TRAP: an agent surveying ReiImpl will report *its* `file:line` as if it were ours. ◄◄**
This has cost real time twice. A whole plan section once recommended synthesizing `FunctionA`/
`StructA` on the strength of `higher_typing_pass.rs` line numbers — a pass **retired outright** in
our tree. **Require every survey of a sibling tree to state which tree each citation belongs to, and
re-verify any load-bearing claim against our own source before acting on it.**

### 0.9 Doc discipline: a wind-down must not thin the reasoning

The name-resolution design (§10) was worked out over a day, including an agent sweep of `~/rust`, and
was then compressed to a single bullet in a wind-down rewrite. It had to be reconstructed from a
transcript. **A section that is long because the reasoning was expensive is correct as-is** — the
compression target is stale state, never argument. What earns permanent space: what was ruled out and
*why*, facts that cost real time to establish (§4), and the failure mode a decision was avoiding.

---

## 1. The design, in one page

**`extern` is a body kind, not a denizen kind.** A Rust item becomes an ordinary synthesized Vale
declaration — a `FunctionS` whose body is `IBodyS::ExternBody`, carrying the same `Extern` attribute
the postparser attaches for a hand-written `extern func`. From there nothing downstream knows rustc
was involved: the function-compile phase picks it out of a top-level store like any Vale function,
the solver resolves its rules, and `make_extern_function` mints the concrete `PrototypeT` and
registers its bounds **per instantiation**, after types are known.

Four consequences worth stating as principles, because each replaced something that looked
reasonable and wasn't:

1. **One code path for free functions, methods, and drop.** All three are top-level declarations
   whose first parameter is the receiver if they have one. Vale erases method syntax in the
   postparser — `v.get()` becomes an overload call with the subject spliced in as argument zero —
   so a method-shaped declaration buys nothing. No prototypes in environments, no citizen-env
   entries for methods, no asymmetry.

   **This is a direction, not a convenience.** The architect: *"i dont like it when methods are
   treated any differently than functions, i think that's one of rust's biggest mistakes"* — and
   separately, that **Rust got drops wrong in general**. Vale's own design already agrees on both:
   `IEnvEntryT` has no method variant, `ITemplataT` has none, `overload_resolver.rs` contains the
   string "internal" zero times, and the monomorphization path never mentions drop. Vale2's dispatch
   redesign states the first outright: *"`x.foo()` and `foo(x)` search the exact same candidate
   set."*

   **Both are instances of one posture — arch §1.5.7, "Refuse special cases."** It is the same
   principle as *non-generic is the degenerate case of generic*, as *synthesized is the degenerate
   case of parsed*, and as *`extern` is a body kind rather than a denizen kind* — which is this
   arc's entire design. §1.5.7 lists what Rust made special about drop, in five compounding ways,
   and why Rust instinct actively misleads here. **Read it before adding any construct that seems to
   need its own machinery**; the answer is almost always that it is a case of something ordinary.

   Concretely for us: a Rust type's `drop` is one more top-level declaration in the same store as
   its other functions, produced by the same code path, resolved by the same overload lookup. There
   is no drop-shaped seam in `rust_interop/` and there should never be one.
2. **A Rust type is a real, defined citizen with zero members.** `StructDefinitionT` + `Extern`
   attribute + `sharedness: Single` + `weakable: false`. Zero members is the truth, not a stub.
   Single/non-weakable are permanent: Rust will never support either.
3. **Signatures are read structurally, once.** `fn_sig` returns generic parameters *as* parameters
   (`ValeSigType::Generic(i)`), not one instantiation. Vale's solver substitutes. There is no
   per-call-site query back to rustc.
4. **The oracle is a binding generator, not a query service.** Consulted once per item to produce
   declarations.
5. **Arguments are data on the node, not something applied away during construction.**
   `ValeSigType::Citizen { name, args }` keeps a citizen *unapplied*, with its arguments as
   signature positions of their own, recursively — so `Holder<int>`, `Holder<Holder<int>>` and
   `Holder<T>` are one case at three depths. The alternative, lowering to a settled `KindT` at read
   time, fails two ways and **both fail silently**: it drops the argument (`Holder<i32>` and
   `Holder<bool>` interning alike) and it cannot express a parameter at all (`T` has no `KindT`).

   **This is the same principle at three layers, discovered independently three times.** Harmonious
   carries the arguments on the *type reference*; we now carry them on the *signature position*; and
   arch §15.7's synthesized drop is supposed to carry them on the *call node*. Harmonious's framing,
   which is the general one: *arguments must be data on the node, not something applied away during
   construction* — and in every instance the eager version fails quietly rather than loudly.

   The tell that you are about to get this wrong is reaching for "resolve it now while I have the
   information." §1.5.8 is the same instinct one level up.

6. **Synthesized is the degenerate case of parsed, for types as well as functions.** A Rust type is
   a `StructS` handed to the ordinary machinery, not a `StructDefinitionT` we build ourselves. That
   turned out to be a *simplification* — `import_rust_types`' six `coutputs` calls all had ordinary
   owners, and deleting it shrank core's interop footprint rather than growing it. The two derive
   macros are suppressed with the language's own `DontCallMacro` attribute rather than any
   Rust-specific special case.

### Why the previous design failed

It built a finished `PrototypeT` at environment-build time from `fn_sig(item, &[])` — empty args —
which cannot represent a generic Rust function: `fn pick<A, B>(a: A, b: B) -> A` has no single
signature, only one per instantiation. Two arcana already forbade it (**@ECSIIOSZ**: every call site
gets its own fresh solver; **@BDPFWDZ**: each solve reaches into the calling env at solve time
rather than depending on something pre-pushed into a store). Both sibling implementations had
already tried and abandoned the same shape — one still contains the corpse: an
`ExternFunctionTemplataT` holding a finished header, zero producers, `tyype` is `vfail()`. **Do not
port it**; it is dead but *constructible*, so producing one anywhere silently restores the old
behaviour with no compile error.

---

## 2. What is in the tree

**Committed and clean** through `acd47597c` (on `experimental-4`, ratcheted to `experimental`,
2026-07-27). Nothing uncommitted.

| | |
|---|---|
| default suite | **577** passed / 170 failed / 8 ignored |
| interop suite | **622** passed / 170 / 8 — 45 corpus cases (610/33 at `acd47597c`; +12 uncommitted — three panic-vs-decline, four multiplicity/scoping, two nested-module, two re-export, one composition) |
| driver (`valec-rs`) | exit 0 — `valec-rs <fixture-dir> <out-dir>` |
| warnings | 8, all pre-existing |
| **core diff** | a **deletion** — the `import_rust_types` call site went away and nothing replaced it |

The 170 failures are the onion arc's known state, ratified repeatedly as the commit bar ("typing
pass builds, some typing tests pass"). **Treat 577/170/8 and 610/170/8 as the fixed baseline;
movement in either direction is a stop, not a footnote.**

**The full gate still cannot run.** `cargo build` exits 101 on `src/bin/valec/`, which references
`backend_ffi`/`pass_manager` — intentionally commented out of `lib.rs` by the onion arc — so neither
nextest backend can build its targets. This has blocked the config's gate on four consecutive
`fire commit`s and is unchanged by any of them. `--lib` is the ratified substitute.

### What landed in `acd47597c` (the generic-types commit)

- **`rust_interop/corpus.rs` — new.** Every case as data: `(fixture, name, Vale program, allowlist,
  expectation)`, the expectation being `Returns(n)`, `FailsToCompile(variant)` or `RustcFails`. It
  lives in the interop module rather than the test tree **because tier 2's likely home,
  `end_to_end_tests`, is an ordinary `pub mod`** and cannot see anything gated on `cfg(test)` — a
  corpus in the test tree would be invisible to it and the tiers would drift back into two copies of
  each program. Data only: no assertions, no AST walking.
- **A Rust type is a synthesized `StructS`**, registered as `IEnvEntryT::Struct`. `import_rust_types`
  is **deleted**: `precompile_struct`/`compile_struct` do its six `coutputs` calls, so keeping it
  would double-declare. `importer.rs` lost 193 lines net.
- **`ValeSigType::Citizen { name, args }`**, recursive — a signature position can be a citizen
  applied to arguments that are themselves positions.
- **Per-item package coordinates** from `tcx.def_path`, retiring the last @ATAFLBZ site.
- **`harness.rs` builds N dependency crates**, discovered from the fixture directory and sorted for
  determinism; plus `compile_check_fixture`.
- **`fixtures_two_crates/` — new**, with a colliding `Widget`/`Widget` pair and a non-colliding
  `Gadget`/`Doohickey` pair, so one directory serves both the multiplicity and the collision case.
  Which question a case asks is decided purely by its allowlist.

### Previously uncommitted, now in `26791765e`

- `declarations.rs` — `synthesize_extern_function`: unique `DefId`-derived `CodeLocationS`, generic
  parameters declared and referenced directly (no rule needed — that is what the postparser emits
  for a hand-written generic function), `LookupSR` per concrete type, `ExternBody`.
- `importer.rs` — `import_rust_types` declares the type, its sharedness, a real `StructDefinitionT`,
  empty outer **and inner** envs; `rust_package_stores` emits the type as a nameable `Kind` entry
  plus one declaration per free function, method, and drop. *(Superseded by the uncommitted work
  above: `import_rust_types` is deleted and the type is registered as a declaration instead. This
  entry describes `26791765e` as it stands, not the working tree.)*
- `oracle.rs` / `tyctxt_oracle.rs` — `ValeSigType`, structural `fn_sig`, name-keyed generic
  resolution, `TyKind::Alias` declined.
- **Five dead oracle methods deleted** (2026-07-26): `resolve_path`, `kind`, `resolve_method`,
  `resolve_function`, `field`, plus the `RustKind` and `RustFieldInfo` types that existed only to
  serve them. All had lost their last caller when the per-call-site seam was retired. Deleted
  rather than parked because `resolve_method` and `resolve_function` matched Rust items by **human
  name string** — two of the three @ATAFLBZ sites (§6) — and a dead-but-callable name matcher is
  how that hazard comes back. Also makes "nothing queries the oracle per call site"
  unrepresentable rather than merely tested, which retires case 35.
- `seam.rs` **deleted** — both exports lost their last caller when the pivot landed.
- `logging_oracle.rs` — every entry now carries a structured `OracleQuery` **and** the rendered
  line. Tests key on the former; the latter is what a person reads when one fails. This is what
  stops assertions coupling to `Debug` output, which broke twice in one day.
- `fixtures/mycrate.rs` — gained `pick<A, B>` and `first<I: Iterator> -> I::Item`.
- `fixtures_broken_rust/` — a deliberately unparseable stub crate, now the input to a *passing*
  regression test rather than a deliberate red.
- `test/rust_interop/harness.rs` — `run_case` / `try_run_case` plus one `Callbacks` impl. The
  extractor is higher-ranked (`for<'s, 't>`) with `R` fixed outside the quantifier, which makes
  "only owned data escapes the callback" a compile error to violate.
- `test/rust_interop/cases.rs` — the 7-case corpus (§5).
- **`code_source.rs`: `Source::rust()` and `resolve_rust_package` deleted.** They had zero callers —
  added for an `import rust.X.Y` path the synthesized-declaration design never took, since a Rust
  type arrives by inference from a signature and no `.vale`-source package needs resolving. That
  file now carries **no interop cfgs at all**. A comment records what was there and when it comes
  back (when `import` populates the allowlist).
- **`fixture.rs` deleted** — `FixtureOracle` lost its last consumer when
  `calls_a_rust_free_function` moved onto real rustc. `fixtures_missing/` went with it; the
  property it demonstrated was about the driver's removed `check()`.
- `driver/main.rs` — **no longer carries assertions.** It compiles, reports, and exits; the
  assertions it used to hold are the corpus. It stays as the seed of the real `valec-rs`
  (arch §3.2), which is the only reason it ever needed to be a binary.

---

## 3. Decisions locked this arc

- **Global `panic = "abort"`** — ratified; it was already arch §1.7/§16, so this confirmed rather
  than changed the architecture. Dissolves the `Void`/`Never` destructor-return constraint instead
  of engineering around it. Known cost: `catch_unwind` does not work, including inside Rust
  libraries that sandbox with it.
- **A Vale value is moved into its destructor; a Rust-backed value is not ours to destruct.**
  Ratified by the architect 2026-07-27, and it corrects arch §15.7, which applied one mechanism to
  both. **Vale has no drop-in-place** — `drop(self T)` takes the value by move. A *Rust-backed*
  value is likewise moved into an `extern` drop, but its **body** destructs in place through
  rustc's own glue, because Vale is an external consumer and does not own that destructor. Our
  synthesized receiver is by-value with no reference wrap; only the extern body bridges to a
  pointer. See the correction block on arch §15.7 for the two inconsistencies this exposes there.
- **Extern drop uses `__vale_drop<T>`** (arch §1.7). One generic wrapper doing `drop_in_place::<T>`;
  rustc resolves its own drop glue inside it. No symbol to name, no per-monomorphization user shim,
  and non-`needs_drop` types cost nothing for free. **By-pointer, never by-value** — Sky tried
  `mem::drop`-shaped and reverted within a day (`Vec<Vec<Widget>>` double-frees if the compiler does
  not track moves). Do **not** write a `needs_drop` predicate; it cannot answer for a bare type
  parameter. `todo/opaque-extern-drop.md`'s `extern(rust)` rows contradict this and carry a
  superseded banner.
- **Inherited generic params go last** (@PRIIROZ) — settled by our own comment at
  `function_compiler_core.rs:398-402` plus four sources in the sibling tree. The @SMLRZ re-split
  projector already exists at the Hammer boundary.
- **Two test tiers, no fixture oracle** — arch §26b. **Tier 2 asserts on program output only**, the
  way Vale's existing end-to-end tests already do; it does not re-assert tier 1's structure. So a
  case is `(Rust fixture, Vale program, expectation)` where the expectation is either "compiles and
  returns N" or "fails with error E", and the two tiers read the same case — tier 1 checking the
  compile half plus the typed AST, tier 2 running it and checking N. Cases that must *not* compile
  are tier-1-only. The Vale program lives in a shared Rust `const` so both tiers read one text; no
  on-disk schema is needed for that, and none should be invented until something needs it.
- **Phase order** (stated 2026-07-25, still governing): *a lot of things working in the typing pass
  → the LLVM 16 → ~21 port → codegen/instantiator → more typing pass → more codegen*, alternating.
  We are in the first phase. The port is what unblocks tier 2, the instantiator, symbol naming, and
  the @SMLRZ wire-format re-split — so "blocked on the LLVM port" throughout this doc means
  "scheduled, not stalled."
- **Serialization deferred.** Tier 1 needs none (§5), so typing-pass serialization can be designed
  into core on its own merits later. Note `von/` is only the 103-line value model and is commented
  out of `lib.rs:53`; VonHammer was never ported. Use `serde_json` if and when it happens.

---

## 4. Verified facts worth not rediscovering

- **`StructDefinitionT` has no `origin_struct` field** and `add_struct` asserts only two things
  (sharedness declared; not already added). So a definition can be built with no `StructS` at all —
  which is what made the core diff empty.
- **Methods are already just functions in Vale.** `IEnvEntryT` has no method variant, `ITemplataT`
  has none, `grep -n "internal" overload_resolver.rs` returns zero hits, and method syntax is erased
  in the postparser. The ~15 places that treat methods differently are all "declared inside braces,
  so it inherits the citizen's generic params / env / vtable slot."
- **The inner env is *not* only placeholders.** It holds every rune the definition solve concluded,
  and `infer_compiler.rs:491` / `edge_compiler.rs:516` depend on it holding **prototypes** (function
  bounds). The placeholder intuition is right about the *ids*, not the store.
- **`sibling_entries` are the macro-derived drop/constructor** nested under that struct — *not* the
  declaring package's contents. A Vale struct's own name is reachable from its methods because
  `PackageEnvironmentT` is flat and global (every package env unions *all* top-level stores), not
  because of siblings.
- **A citizen env checks its own store first and honours `get_only_nearest`**, short-circuiting
  before the ambient namespace — unlike `PackageEnvironmentT`, which ignores it and concatenates.
- **Prototypes in environments are a real Vale shape — but only for bounds**, keyed by rune or
  reachable-index. Nothing in either tree stores a prototype as "what function does this name refer
  to."
- **A lambda understruct puts its own kind in its own outer store**, under both its name and
  `Self_` — the precedent for a self-entry, if one is ever needed. Not what extern structs do.
- **`compile_struct_core:144` panics on any non-`Function` entry** in a citizen outer store. Inert
  today (no `StructS` ⇒ it never runs), live the moment one is synthesized.
- **`get_imprecise_name` takes an `INameT`, not an `IdT`** (`environment.rs:435`) — so it sees the
  *local* name and **never the package coordinate**. `add_entries` keys every store entry through
  it (`:567`). This is why §10.9's step 4 cannot be a new match arm: there is nothing in scope to
  build a qualified key *from*. Either the coordinate gets threaded in, or the interop side
  registers under an explicitly-supplied imprecise key. Worth knowing before designing the fix.
- **`CouldntNarrowDownCandidates` has no trailing `T`**, unlike most `ICompileErrorT` arms. Cost a
  red test on first write.
- **A citizen's name resolves to a *template*, so a `CallSR` is needed even at zero arguments.**
  `solve_call_rule`'s forward branch dispatches on what the template rune resolved to:
  `StructDefinition` produces a real instantiation, while **`ITemplataT::Kind` binds the result and
  ignores the arguments entirely** (`compiler_solver.rs`, the `None` branch's last arm). So the
  wrong registration fails *silently* rather than erroring. Skipping the call for a non-generic
  citizen fails loudly the other way — the parameter rune resolves to a `StructDefinition` where a
  `Kind` is wanted and `evaluate_function_param_types` panics.
- **`StructDefinitionTemplataT` holds `origin_struct: &StructS`** — a *parsed* declaration. That is
  why registering a type as `IEnvEntryT::Struct` is the whole mechanism, and why there is no need
  for a Rust-specific `ITemplataT` arm. Neither sibling tree has one.
- **`#!DeriveStructConstructor` / `#!DeriveStructDrop` are `DontCallMacro` attributes**, filtered by
  `determine_macros_to_call`. Suppressing a derive on a synthesized declaration is an existing
  language feature, not a special case.
- **The interop test compilation supplies no builtins at all.** `CodeSource::new(vec![one entry])`,
  so `+` resolves *zero candidates* — not a scoring failure, and not a Vale defect. Vale's own tests
  pass `builtin_source_for_arith(..)` plus `import v.builtins.arith.*`. We can have arithmetic in a
  corpus program whenever one wants it; we had been shaping programs around its absence without
  knowing why. (Reported to Vale2 as a possible defect and **withdrawn** — check your own harness
  before reporting.)
- **`integration_tests` is commented out of `lib.rs:37`.** Anything under it does not compile or
  run, including `infer_template_tests.rs`, which asserts generic-argument inference directly. Do
  not cite those tests as evidence of current behaviour without checking they execute.
- **Every method added to `RustOracle` must be forwarded in `LoggingOracle`.** A default trait body
  means the decorator silently answers the *default* rather than delegating — adding
  `type_generic_params` without forwarding made every generic Rust type arrive with zero generic
  parameters, with no error anywhere. The file carries its own warning (*"a decorator that inherits
  a default is a decorator that lies"*) and it was written into this arc **and then walked into
  anyway**, which is why it belongs here too: a default body is a silent answer, and a silent answer
  at a seam is indistinguishable from data. Harmonious checked their own tree after we reported it
  and found the same hazard live — their single defaulted method is the hook their whole codegen
  contribution flows through.
- **`assemble_initial_sends_from_args` builds `InitialSend`s that nothing consumes.** All four call
  sites bind the result and drop it; `InitialSend` is used nowhere in `typing/`. The commented-out
  Scala original passed it to `solveForDefining`. Routed to Vale2 as the most likely explanation for
  argument types not reaching parameter runes — **confirmed, and it is now their design**: their
  ratified phase 0 names this producer as its output. Two corrections to how we recorded it: the
  sends go against `full_type_rune` **unpeeled** (harmless only because discarded, so the producer
  needs reshaping too), and wiring up the existing call sites is *not* the fix, because what a send
  means when the rune is already determined is explicitly unruled. See §7.
- **`+` resolves no candidate at all** in the interop test compilation, and **reading a local**
  yields `BorrowRef(int)` where `int` is wanted (`NoImplicitCloneDefinedT`). Both are Vale-side, not
  interop's — the second is the same borrow read-out gap that blocks case 39. Corpus programs
  therefore avoid arithmetic and return call results directly rather than through a local.
- **A synthesized declaration has to stay *statically filterable*, and today's shape already is.**
  Vale2's ratified candidate filter (§10.10) reads three things off a parameter with no solving:
  arity, the **wrap chain** from `type_outer_ref_rules`, and the **value-type template name** from
  `value_type_rules`' outermost `Call` templated on a `Lookup`. That is exactly the `LookupSR` +
  `CallSR` pair `declarations.rs` emits for every citizen position — so we satisfy the filter by
  construction rather than by intent. **Consequence: the `CallSR` is load-bearing for a second
  reason.** @NNGZ says emit it at zero arguments because non-generic is the degenerate case; this
  says emit it because a citizen position with no `Call` presents no readable template name and the
  filter cannot see the parameter at all. Anyone "simplifying" the emission breaks overload
  resolution in a way today's suite would not catch, since the filter does not exist yet.

---

## 5. Testing — where this is going

Full strategy in arch §26b. **The harness is built and the corpus has replaced the driver's
assertions**; what remains is growing it.

Coverage was one automated test plus a driver binary `cargo test` never ran — nine assertions
sitting behind a manually-invoked `[[bin]]`. It is now 7 cases in `cargo test --lib`, each against a
real `TyCtxt`, each pinning one behaviour so a failure localizes rather than arriving as a lump:

| case | what fails if it breaks |
|---|---|
| `calls_a_rust_free_function` | a synthesized declaration no longer resolves a call |
| `an_empty_allowlist_makes_nothing_importable` | the positive case above has gone vacuous |
| `reads_a_generic_signature_structurally` | generics collapsed to one instantiation, or the parameter index mapping slipped |
| `calls_a_method_on_a_rust_type` | a Rust type stopped reaching Vale from a signature, or a method stopped being an ordinary function |
| `a_rust_value_bound_to_a_local_gets_a_scope_end_drop` | the synthesized `drop` went missing |
| `declines_an_unrepresentable_signature` | an un-normalizable alias got imported with a hole in it |
| `a_fatal_rustc_error_costs_one_case` | a broken fixture can take the suite down |

The shape a case takes:

```rust
let outcome = run_case("fixtures", "case-name", vale_source, allowed, callees_in_main);
assert_eq!(&vec![/* owned facts */], outcome.expect_compiled());
assert!(outcome.asked(|q| q.offered("add_two_numbers").is_some()));   // vacuity
```

**A correction to the measurement.** The fatal-rustc-error hazard was framed as a `process::exit`
that would take the whole suite down. It is not: rustc emits its diagnostic and then **unwinds**,
with a `FatalErrorMarker` payload rather than a string — which is why, uncaught, it produced a test
failure with *no message at all*. `catch_with_exit_code` is rustc's own way to turn that back into a
value, and the harness uses it. The conclusion is unchanged and slightly stronger: a broken fixture
costs one case, legibly.

### 5.1 The corpus — 40 cases, named

"A lot of them" needs a number, and the sibling tree's 32 is the bar to pass. Below is the whole
intended corpus: **39 implemented (✅), 2 remaining, 2 answered without a case**. Each line says what
breaks if the case fails, because a case whose failure mode nobody can state is a case nobody will
fix.

**Both remaining cases are blocked, and neither is ours**: case 39 needs Vale2's borrow read-out fix
and case 41 needs their phase 0 (§7). Case 25 is written but pins a panic until the naming change
lands. Cases 35 and 37 were **answered by making the property unrepresentable or by probe** rather
than by writing a case — see their rows; that is §1.5.6 rule 4 working, not coverage missing.

Every case that compiles also declares the value `main` returns, so tier 2 can run the identical
case and check the output. Cases marked **fail** are tier-1-only by nature.

**A. Signatures and lowering** — what a Rust signature may contain and what happens when it can't
be expressed.

| # | case | pins |
|---|---|---|
| 1 | `calls_a_rust_free_function` ✅ | a synthesized declaration resolves an ordinary call |
| 2 | `calls_a_zero_arg_rust_function` ✅ | empty parameter list is the degenerate case, not a special one |
| 3 | `calls_a_rust_function_returning_unit` ✅ | `()` → `VoidT`, and a call in statement position |
| 4 | `passes_and_returns_a_bool` ✅ | a non-integer primitive round-trips |
| 5 | `takes_a_rust_type_as_a_parameter` ✅ | a Rust citizen in *argument* position, not just return |
| 6 | `takes_and_returns_a_rust_type` ✅ | the same citizen identity on both sides of one signature |
| 7 | `reads_a_generic_signature_structurally` ✅ | generics stay parameters instead of collapsing to one instantiation |
| 8 | `binds_the_second_generic_parameter` ✅ | the mirror canary — `pick_second<A,B> -> B` at `<int,bool>` catches an index swap the first canary cannot |
| 9 | `instantiates_a_generic_at_one_parameter` ✅ | `id<T>(T)->T` — substitution happens at all; passes under any mapping, so it is a floor not a canary |
| 10 | `instantiates_a_generic_at_a_rust_type` ✅ | a citizen as a *generic argument*, not just a parameter type |
| 11 | `declines_an_unrepresentable_signature` ✅ | an un-normalizable alias in return position is dropped, not imported with a hole |
| 12 | `declines_an_unrepresentable_parameter` ✅ | the same, in argument position — a different code path |
| 13 | `declines_an_unsigned_integer` ✅ | the `IntT`-has-no-signedness gap. Panicked until 2026-07-27; now declines by the same exit as an alias |
| 14 | `declines_a_float` ✅ | the `FloatT`-has-no-width gap; same exit. Takes *and* returns `f32`, so the decline is reachable from either the parameter loop or the return lowering |
| 15 | `declines_a_signature_naming_an_unimported_type` ✅ | @RTMEIZ — reaching a type only through another item's signature does not import it. **Written to go red and passed immediately**: `lower_sig_ty`'s `Adt` arm already declined via `?`, so only `lower_ty`'s un-`lower_sig_ty`'d path (through `TyKind::Ref`) ever panicked here |

**B. Item kinds** — that free functions, methods, drop and associated functions are one path.

| # | case | pins |
|---|---|---|
| 16 | `calls_a_method_on_a_rust_type` ✅ | a method is a top-level function whose first parameter is the receiver |
| 17 | `calls_an_associated_function_with_no_receiver` ✅ | `Counter::new()` — an inherent fn without `self` still imports |
| 18 | `calls_two_methods_on_one_type` ✅ | method discovery is a list, not a lucky single |
| 19 | `calls_methods_on_two_different_rust_types` ✅ | two types' methods coexist, each resolving to its own receiver. `Counter::get` and `Gauge::get` share a name deliberately — there is no per-type method table to "bleed", so what this actually catches is the importer pairing a method with the **wrong receiver**, which surfaces as a resolution failure rather than a wrong answer |
| 20 | `a_rust_value_bound_to_a_local_gets_a_scope_end_drop` ✅ | the synthesized `drop` exists and resolves |
| 21 | `a_rust_value_returned_and_discarded_gets_dropped` ✅ | drop on the temporary path, not just the bound-local path. Silent if broken — the program compiles and returns the right number either way — so the assertion is on the callee list, never on the outcome |
| 22 | `calls_a_generic_method` ✅ | a method carrying its *own* type params, on top of the container's |

**C. Multiplicity and crates** — that nothing depends on there being exactly one of anything.

| # | case | pins |
|---|---|---|
| 23 | `imports_two_rust_types_at_once` ✅ | the importer is a loop, not a single-item path. Free-function-only on purpose, so it does not also depend on method discovery — case 19 covers that half |
| 24 | `imports_from_two_crates` ✅ | one store per package coordinate, keyed correctly |
| 25 | `two_crates_exporting_the_same_short_name_stay_distinct` ✅ | the @ATAFLBZ identity hazard, **half fixed and half core-blocked**. Written red; per-item `def_path` coordinates made the two `Widget`s two Vale types, which cleared the `declare_type` assertion. What remains is *naming* them: a bare `CodeNameS` finds both and `lookup_nearest_with_imprecise_name` panics. The case now pins that panic via `should_panic` so the trigger is observable; the corpus still declares `Returns(5)`, which is where it lands once §10.9's core steps land |
| 26 | `a_rust_type_flows_through_two_calls` ✅ | citizen identity survives being produced by one call and consumed by another. A lowering minting a fresh kind per signature would typecheck each call in isolation and fail only here |

**D. Scoping** — that the allowlist is load-bearing and is the only thing that is.

| # | case | pins |
|---|---|---|
| 27 | `an_empty_allowlist_makes_nothing_importable` ✅ **fail** | the positive cases are not vacuous |
| 28 | `an_item_not_in_the_allowlist_is_not_importable` ✅ **fail** | the positive control's mirror: the crate exports it, we still can't see it |
| 29 | `an_allowlist_entry_the_crate_does_not_export_is_ignored` ✅ | a stale allowlist entry is inert, not fatal |
| 30 | `a_module_named_in_the_allowlist_is_filtered_by_defkind` ✅ | `mycrate`'s children include `std`; a name match must not hand back a module where a function was asked for |

**E. Failure modes** — that wrong programs fail, and fail legibly.

| # | case | pins |
|---|---|---|
| 31 | `wrong_argument_types_do_not_resolve` ✅ **fail** | a Rust callee competes on `params_match` like any other |
| 32 | `wrong_generic_arity_does_not_resolve` ✅ **fail** | arity is checked rather than silently truncated (@ETASTZ) |
| 33 | `a_vale_function_and_a_rust_function_with_the_same_name` ✅ | that same-named functions **do not** collide — **now measured, not predicted.** Both reach overload resolution as candidates (one `package_coord: test`, one `rust.["mycrate"]`) and the outcome is the designed `CouldntNarrowDownCandidates` error, never a panic. Exactly §10.10's split, and the deliberate contrast with case 25's type-name panic. Note the variant has **no trailing `T`**, unlike most `ICompileErrorT` arms |
| 34 | `a_fatal_rustc_error_costs_one_case` ✅ | a broken fixture cannot take the suite down |

**F. Provenance and vacuity** — that our machinery ran, and only where it should.

| # | case | pins |
|---|---|---|
| 35 | ~~`no_oracle_query_happens_per_call_site`~~ | **subsumed, not written.** The per-call-site queries were deleted from the trait (§2), so the property is unrepresentable rather than tested — a stronger guarantee than a case |
| 36 | `a_program_using_no_rust_items_compiles_with_an_oracle_present` ✅ | an oracle in scope costs an ordinary Vale program nothing |
| 37 | ~~`no_extern_function_name_reaches_an_environment_store`~~ | **answered by probe, not written — 2026-07-27.** `get_imprecise_name`'s `INameT::ExternFunction` arm (`environment.rs:488`) was replaced with a `panic!` and both suites re-run: **617/170/8 and 577/170/8, byte-identical, zero hits.** The arm is dead. Like case 35, the right resolution is deleting it so the shape is unrepresentable rather than writing a test asserting its absence (§1.5.6 rule 4) — but that is **core**, so it is the architect's (§7). **Evidence limit worth stating:** "not reached by this suite" is weaker than "unreachable", because 170 tests stop at a first blocker and cannot exercise what lies behind them |

**G. Vale source naming Rust items** — the half of the naming story that is *not* about synthesized
declarations.

| # | case | pins |
|---|---|---|
| 38 | `vale_source_can_name_a_rust_type` ✅ | hand-written Vale naming a Rust type by bare name, with no import statement. **Verified 2026-07-26** — it works, via the citizen's `Kind` entry in the reserved `rust` package store plus `PackageEnvironmentT`'s flat union. Easy to assume otherwise |
| 39 | `vale_source_calls_a_method_on_a_named_rust_parameter` | the same, with a body that *uses* the parameter. **Blocked**: reading a parameter yields `BorrowRef(Counter)` where `get(self Counter)` wants it owned, and `is_type_convertible` panics on the borrow read-out (`templata_compiler.rs:1209`). Vale2's, per §7 — write it when they land the fix |
| 40 | `a_generic_rust_type_carries_its_arguments` ✅ | a **generic** Rust type imports with its arguments intact — `Holder<i32>` and `Holder<bool>` are two distinct kinds, asserted as `("rust-citizen<int32>", "rust-citizen<bool>")`. Asserted the *defect* until 2026-07-26; inverted when §9 step 2 landed |
| 41 | `a_generic_rust_type_gets_a_scope_end_drop` | **not written, and Vale2's.** A compiler-generated drop call supplies no explicit type argument, and inference does not happen. **Pure Vale has the same gap** — `compiler_ownership_tests::opt_with_undroppable_contents`, a hand-written top-level `func drop<T>(opt Some<T>)`, is among the 170. Not an interop defect; see §9 step 2 |
| 42 | `calls_a_generic_function_taking_a_generic_type` ✅ | `holder_ignore<T>(Holder<T>)` at `<int>` — a citizen *applied to its own parameter* in argument position. The shape `pick<A, B>` does not reach, and what `ValeSigType::Citizen` exists for |
| 43 | `every_fixture_stub_is_valid_rust` ✅ | a fixture cannot rot into invalid Rust unnoticed. Tier 1 sees only *parse* errors, so nothing else covers a stub that type-errors. Skips `fixtures_broken_rust`, which is unparseable on purpose |
| 44 | `imports_an_item_from_a_nested_module` ✅ | an item below the crate root, named by a dotted path — the shape `Vec` needs. **The first case a root-only walk fails**: every other case sits at a root, which is the degenerate path, so all of them passed while nested items were structurally invisible |
| 45 | `imports_a_type_from_a_nested_module` ✅ | the same at a different `DefKind`, plus a method on the nested type. A walk could plausibly descend for functions and not for types; the method came for free, since discovery runs off the owner's `inherent_impls` and never asks how the owner was reached |
| 46 | `imports_through_a_re_exported_item` ✅ | a re-exported name, which is the shape `std::vec::Vec` actually has. **Written expecting red, passed immediately** — `module_children` reports a re-export with its `Res` naming the definition, so taking the `DefId` from the `Res` follows it invisibly. Intra-crate only; cross-crate is untested |
| 47 | `imports_through_a_re_exported_module` ✅ | descending *through* a re-exported module rather than landing on a re-exported item — a walk could handle the destination and not the intermediate hop. It handles both |
| 48 | `a_program_using_everything_at_once` ✅ | **the composition case.** Sixteen mechanisms in one program and one import list, plus three declined items that must not disturb them. Every other case is narrow so failures localize, which is right for diagnosis and useless for *"do these coexist?"* — interference is its own failure class (a shared name resolving to the wrong item, an import-order dependency, a drop that works only when it is the only drop) and no narrow case can see it. Asserts on the **callee list**, not the return value: a program this size could return 31 while resolving half its calls wrongly |

### 5.2 Discipline — where we are

**RFIGA / TDD: partial, and honestly so.** The generics slice ran the full loop. The harness work
did not, because migrating existing assertions has no RED to see — with two exceptions that both
paid for themselves:

- `a_fatal_rustc_error_costs_one_case` went red first and corrected the mechanism: rustc **unwinds**
  rather than exiting.
- `vale_source_can_name_a_rust_type` was written as a probe and immediately refuted a claim this
  doc had carried twice — that Vale source cannot name a Rust type. It can.
- `a_generic_rust_type_loses_its_arguments` came out of a probe too, and found something worse than
  the gap it went looking for: not "generic types are unsupported" but "generic types compile and
  give the same answer for different arguments."

The lesson to keep: **probes before claims.** Three separate statements in this document turned out
to be wrong the moment someone ran the code, and each was cheap to check.

**2026-07-27 — the loop ran properly, and every red taught something.** Updating this section
because it had been describing an older, weaker state:

- **Case 25 was written red against the @ATAFLBZ fix, as §5.1 specified**, and went red for the
  *predicted* reason — the second `declare_type` tripping its own assertion. Then the fix moved it
  to a *different* red (`Too many with name`), which is what revealed that the identity half and the
  naming half are separate problems and only the first was ours.
- **Case 40 was inverted to assert the fix before the fix existed**, watched fail, then made to
  pass. Its first failure message was a bare boolean, which is what prompted `describe_kind` to
  render arguments — a case that can only assert inequality is a case whose vocabulary is too
  coarse (§26b.4).
- **The `holder_ignore` probe failed differently than predicted and redirected the design.** It was
  written to distinguish "drop is special" from "citizen-applied parameters are broken", and instead
  panicked in the oracle on `ty::Param` — which is what produced `ValeSigType::Citizen`, and with it
  both the generic-type fix and the deletion of a hand-written drop synthesizer. The most valuable
  red of the session was the one that failed for an unexpected reason.

The counter-example, recorded because the pattern is worth seeing: **`type_kind`'s `template_args`
fix was written without a red first**, on the reasoning that the cause was obvious. It changed
nothing observable — the declaration never carries that kind — and had to be reverted. Writing the
failing observation first would have shown that in a minute.

The corpus in §5.1 remains the RFIGA list going forward.

### 5.3 Next, in order

Everything from the previous list is **done**: the corpus hoist, the `@ATAFLBZ` identity fix, the
corpus growth (9 → 33), the fixture compile-check, and the `@ATAFLBZ` source lint. What follows is
the new list, ordered.

1. **Panic-vs-decline, which is no longer core-blocked.** Harmonious's answer removed the need for
   the poison hook. Two steps, both ours:
   1. **Unify the exits.** `lower_ty` *panics* on unsigned ints, floats, unsized types and
      un-imported ADTs while `lower_sig_ty` *declines* aliases and inherited parameters — one cause,
      two behaviours, and the panic is the wrong one. Make both decline. This is a correctness
      improvement independent of how the decline is surfaced, and Harmonious was explicit that it
      comes first.
   2. **A side table of declined items and reasons**, populated during enumeration, consulted from
      the **existing** lookup-failure path so the error reads *"found `first`, but its return type
      has no Vale form"* rather than "couldn't find function." No field on the declaration, no new
      `ICompileErrorT` variant. Poisoning earns its cost only if a poisoned item must *participate*
      in later phases; if it only has to explain its own absence, a side table is strictly less
      machinery.

      > **►► CORRECTION 2026-07-27: this step is core-blocked after all. ◄◄** The line above said
      > *"no core change — which is what the poison hook needed."* Wrong, and found by tracing the
      > consumer rather than by anything failing. The **producer** is ours (the oracle, during
      > enumeration), but every **consumer** is core: `CouldntFindFunctionToCallT` is minted in
      > `overload_resolver.rs:751`, `array_compiler.rs:280` and `destructor_compiler.rs:33`, and
      > rendered in `compiler_error_humanizer.rs:227`. A declined item is not a *candidate*, so it
      > cannot ride `FindFunctionFailure.rejected_callee_to_reason` — it never became a callee. The
      > three shapes, all core, for the architect to pick between:
      >
      > 1. **A field on `FindFunctionFailure`** (`overload_resolver.rs:55`) carrying declined items
      >    and reasons, populated where the failure is minted. Smallest data change; every
      >    construction site of that struct gains one field.
      > 2. **A new `IFindFunctionFailureReason` variant** plus a synthetic `ICalleeCandidate` for a
      >    thing that is not a callee. Reuses the existing channel at the cost of a lie in the
      >    vocabulary — a "rejected callee" that was never a candidate.
      > 3. **The humanizer consults the oracle** (`compiler_error_humanizer.rs:227`). No data
      >    change, but it needs the oracle in scope where only an error and an interner are today,
      >    which is the widest of the three.
      >
      > What Harmonious's side table *did* remove is real and still holds — the declaration field
      > and the poisoned value flowing through later phases. It shrank the core touch from a hook to
      > a consult; it did not remove it. **So the poison-hook item did not become unblocked, it
      > became cheaper**, and §6's "unblocked" framing is corrected there too.

   **Step 1 landed 2026-07-27** and retires cases 13, 14 and 15 on its own; step 2 waits on the
   ruling above.
2. ~~**A lint for @NNGZ**, matching the @ATAFLBZ one.~~ **DROPPED 2026-07-27.** The violation fails
   *loudly* — twelve corpus cases at once — so the suite already guards it, and per §0.3c a lint is
   the weakest available mechanism. If the property wants defending at a site, the answer is making
   the citizen-position emission atomic (one call emitting both `LookupSR` and `CallSR`, with no way
   to emit half), not watching for people not doing it.
3. ~~**The remaining corpus cases** — 19, 21, 23, 26, 37.~~ **DONE 2026-07-27.** Cases 19, 21, 23
   and 26 are written and green; case 37 was **answered by probe instead of by a case** — the arm it
   was to test is dead, and removing it beats testing for it (§6). The corpus is now 40 cases, with
   only the two Vale2-blocked ones outstanding.
4. **Eagerness** (§6) — **partly core, so not startable in full.** Probed 2026-07-27: the expensive
   half is the compile-everything loop at `compiler.rs:766`, which walks every top-level store and
   compiles every entry. Lazy population would need that loop and the lookup that drives it, both
   core. The ours-half is the per-type method fan-out (`Vec` declares ~100 methods whether called or
   not), and the allowlist already bounds it. Needs a ruling before anything is built.
5. ~~**Re-export traversal.**~~ **ALREADY WORKS — measured 2026-07-27, cases 46 and 47.** Written
   expecting red; both passed on the first run. `module_children` reports a re-export with its `Res`
   naming the **definition**, and the segment walk takes its `DefId` from that `Res`, so it follows
   a re-export without knowing it did — for a re-exported *item* and for descending *through* a
   re-exported module alike. Nothing to build.

   **Two things this does not yet prove.** Our fixture re-exports are **intra-crate**
   (`pub use crate::instruments::…`); `std::vec` is a **cross-crate** one (`pub use alloc_crate::vec`),
   which is a different `module_children` path and untested here. And the *diagnostic* half of §10.0
   still stands: a def-path-derived coordinate will say `rust.alloc.vec.Vec` where the user wrote
   `std.vec.Vec`, which is exactly what rustc keeps `visible_parent_map` — a lossy BFS — to invert.
   Identity follows the def path; messages will need the written one.
6. **Tier 2**, when the LLVM port and the onion relink land: a second runner over the same cases,
   asserting only on what `main` returns. The corpus is shaped for it — that was the point of
   `corpus.rs`. **Distance, measured on their side rather than guessed on ours:** `instantiating/`
   and `simplifying/` are not merely gated, they are *"stale and would not compile"* — they match on
   `ReferenceExpressionTE::While/Return/Break`, an enum with zero hits under `typing/` — and Vale2
   sizes the relink at **~3 weeks**. So tier 2 is scheduled behind a known quantity, not an open
   one, and nothing in the corpus's shape should be traded away to reach it sooner.

**Prefer the Vale program to carry the assertion.** `pick<int, bool>` returning `A` means a swapped
index yields `bool` where `int` belongs and `main() int` will not typecheck — nothing to grep, and it
survives any refactor of how anything renders. The log's remaining job is **vacuity** — proving the
oracle was consulted — which no source program can express, and which caught an empty
`packages_to_build` on its first run.

Where a case does need to look, it looks at *structure*. Substring assertions against `Debug` output
broke twice in one day, neither time from a behaviour change, so nothing keys on rendering any more:
the log carries a typed `OracleQuery` beside its rendered line, a compile failure carries the
`ICompileErrorT` variant name beside its detail, and AST assertions go through a test-owned
`describe_kind` that names a type the way source does.

---

## 6. Known defects and open questions

- **The oracle no longer takes identity from a name.** `resolve_method` and `resolve_function`
  were deleted 2026-07-26 having lost their callers to the pivot; the remaining site — the up-front
  crate walk in `TyCtxtOracle::new` — was fixed by **deriving each item's `package_coord` from its
  own `tcx.def_path`** instead of stamping every item with one coordinate handed to the
  constructor. Two crates each exporting a `Widget` are now two Vale types in two packages
  (`imports_from_two_crates` is the green proof; case 25 is the collision).

  What is still name-shaped is *selection* — which items the allowlist admits — and that is the
  allowlist's own semantics rather than an identity claim.

  **The deletion is what fixed this, not the lint that followed it.** Worth stating plainly because
  the write-up originally had it the other way round: once no function turns a name string into
  identity, the hazard has no site to occur at. The `@ATAFLBZ` source lint guards against
  reintroducing that shape — a real but much thinner benefit, and the weakest mechanism available
  (§0.3c). It carries an allow-marker for the legitimate case, since allowlist *selection* is
  name-shaped by its own semantics while *identity* must not be.

- **A type-name collision is a panic, and the trigger has fired.** With two same-named types
  imported, a synthesized declaration's `LookupSR` carries a bare `CodeNameS`; `PackageEnvironmentT`
  unions every top-level store, so the lookup finds both and `lookup_nearest_with_imprecise_name`
  panics (`environment.rs:164`). Case 25 pins it. The fix is §10.9's Problem A, **two of whose four
  steps are core**, so it is the architect's — and §10.9 now records a correction to how step 4
  has to work.
- ~~**The up-front crate walk is insufficient, not merely slow.**~~ **The insufficiency is fixed
  (2026-07-27); the eagerness is not.** An allowlist entry is now a dotted path resolved segment by
  segment, so a nested item is reachable — see §9 step 1. Recursing the whole graph was correctly
  ruled out: the walk resolves *the paths it was given*, which is O(imports × crates) rather than
  O(crate graph), and no wider collision surface than the allowlist already had.

  **Still eager — and probed 2026-07-27, with a result that splits the item in two.**

  The expensive half is **core**. Declarations are compiled by the loop at `compiler.rs:766`, which
  walks `global_env.name_to_top_level_environment` and compiles *every* entry in *every* top-level
  store, ours included. Running the solver over ~100 declarations is the cost; synthesizing them is
  not. So "synthesize on first reference, as rustc's `populate_on_access` does" cannot be built
  entirely on our side — the lookup that would trigger population is core too.

  The half that **is** ours is narrower than it looked: how many declarations one allowlist entry
  fans out to. Importing a type walks all of its `inherent_impls` and declares every method, so
  `Vec` costs ~100 whether or not the program calls one. And there is a real mitigation already in
  the design — **the allowlist bounds the blast radius**, since a user imports what they name. The
  fan-out is per-type, not per-crate.

  **Do not build a name-scan of the Vale source as a reachability filter.** It was considered and is
  the wrong shape: a Rust item can be reached without its name appearing (a drop we synthesize, a
  method reached through a generic instantiation), so the filter would be approximate in the
  direction that silently drops declarations — §0.2's failure mode exactly.

  **Re-exports turned out to traverse for free** (cases 46/47, measured — the `Res` names the
  definition). What survives is the *diagnostic* half: a def-path coordinate says
  `rust.alloc.vec.Vec` where the user wrote `std.vec.Vec`, and inverting that is what rustc keeps
  `visible_parent_map` for. Identity by def path, messages by the written path — later, and after
  §10's naming change.
- **`lower_ty` panics where `lower_sig_ty` declines, for the same class of reason.** Six panics
  (`tyctxt_oracle.rs:278/283/287/290/302/324`) cover unsigned ints, floats, unsized types, and
  un-imported ADTs; meanwhile aliases and inherited parameters return `None` and the declaration is
  simply dropped. Same cause — "Vale cannot express this Rust type" — two behaviours.

  The resolution is **not** simply panic → decline, and the earlier framing of it as such was too
  quick. Three constraints have to hold at once:

  1. *"for now, panic"* was chosen (2026-07-25) over returning `None`, because `None` produced
     "couldn't find function `foo`" for a function that plainly exists — a lie, which is worse than
     a crash.
  2. But these panics fire during **enumeration**, not at a use site. One `u64` in a crate's export
     surface would make the whole crate unimportable. Declining is right *for enumeration*.
  3. So (1) and (2) only reconcile through Harmonious's **poison, don't drop**: register the
     declaration with the reason attached, and let the use site say *"found `first`, but it can't be
     imported yet: its return type `<I as Iterator>::Item` has no Vale form."*

  ~~Poisoning needs a small **core** hook — a field on the declaration or a new `ICompileErrorT`
  variant — so it is not this arc's to land unilaterally.~~

  **No longer core-blocked, as of 2026-07-27.** Harmonious — who recommended poisoning and then
  disclosed they never built it — offered a cheaper shape that satisfies (3) without the hook: a
  **side table** of declined items and reasons, populated during enumeration and consulted from the
  *existing* lookup-failure path to enrich its message. The lie in (1) was that the failure said
  "couldn't find function `foo`"; improving that message is the whole requirement. A poisoned
  declaration only earns its keep if it must *participate* in later phases, which it need not here.

  **Where we are: half landed, half core-blocked** *(2026-07-27)*.

  **The exits are unified.** `lower_ty` returns `Result<KindT, DeclineReason>` and every one of its
  six panics is now a carried reason; `lower_sig_ty` returns `Result` too, so both halves of the
  same cause take the same exit. Cases 13, 14 and 15 are green, and the suite went 610 → 613 with
  the 170 unchanged. The reason travels as **structure** — a `DeclineReason` enum with no rendering
  on it, because the wording belongs where diagnostics are built and where the arenas to hold it
  live (§26b.4, and the `'static` shield fired on the first attempt to put it on the enum).

  **One case was already satisfied, which the probe found rather than reasoning.** Case 15
  (`declines_a_signature_naming_an_unimported_type`) was written to go red and **passed on the first
  run**: `lower_sig_ty`'s `Adt` arm already used `?` on the item lookup, so an un-imported ADT
  declined at signature level all along. The `lower_ty` panic for that case is reachable only
  through `TyKind::Ref`'s recursion, which does not go through `lower_sig_ty`. So this doc's claim
  that un-imported ADTs panic was true of one path and false of the one that mattered.

  **The table is core-blocked** — see the correction under §5.3 step 1 for the three shapes and why
  the producer being ours does not make the consumer ours. Until it lands, the reason is computed
  and dropped at `fn_sig`, with a `VCOORD` naming the attachment point.
- **`get_imprecise_name`'s `INameT::ExternFunction` arm** (`environment.rs:488`) was added to core
  for the prototype-store design. Under synthesized declarations a store holds
  `IEnvEntryT::Function`, so the arm should be unreachable — and a dead arm in a core file is
  exactly the "dead but constructible" shape that restores an abandoned design by accident.

  **Measured 2026-07-27: it is dead.** The arm was temporarily replaced with a `panic!` and both
  suites re-run — **617/170/8 and 577/170/8, unchanged, with zero hits.** Probe reverted; the tree
  carries no trace.

  **The deletion is core, so it is the architect's** (§0.1). Worth doing rather than testing around:
  per §1.5.6 rule 4 the strongest move is removing the arm so the shape cannot be produced, which is
  what retired case 35 and is strictly better than a case asserting its absence.

  **One honest limit on the evidence.** "Not reached by this suite" is weaker than "unreachable" —
  170 tests stop at their first blocker, so nothing behind those blockers was exercised. The
  conclusion is well-supported for the paths we can currently run and should be re-checked if the
  onion arc greens a large block of them.
- **Eagerness.** Four layers: the oracle tables every allowed item at construction; a declaration is
  synthesized per item; and the function-compile phase compiles **every** declaration whether called
  or not. Fine at a five-name allowlist; `import rust.std.vec.Vec` brings ~100 inherent methods.
  Harmonious's counsel: keep the wrapper (it is what lets the ordinary solver do the work — which is
  why generics needed zero core changes), attack the eagerness. Synthesize on first reference, as
  rustc's own `populate_on_access` does.
- ~~**`resolve_function -> Option<RustItemId>` is the wrong shape.**~~ **Stale, and then honoured.**
  The function it named was deleted on 2026-07-26 with the other four dead oracle methods, so the
  bullet described something that no longer existed. The principle was live, though, and landed
  where it mattered: `resolve_allowlist_path` returns a `Vec` of every match rather than an
  `Option` of the first (§9 step 1). clippy returns `Vec<DefId>` because `memchr::memchr` resolves
  to two major versions at once, and Harmonious conceded their own resolver has the same defect —
  two trees confirming the shape, honoured before it could bite rather than after.
- **Qualified names / collision precedence.** Deferred by decision: *if we hit a collision, do
  qualified names.* Today ambient lookup concatenates every namespace and hard-panics on two hits
  (`environment.rs:164`), with `_get_only_nearest` ignored at the package level (`:880`).
  **The full design is §10** — what was ruled out and why, the representation-vs-resolution split,
  rustc's precedence struct, dual registration, and the ~1,500–2,500-line kernel estimate. **§10.10
  narrows it considerably**: only *type* names can panic, functions never collide, and Vale2's
  dispatch redesign scopes candidates by argument type rather than ambiently.

---

## 7. Blocked elsewhere

**The borrow path.** Two independent holes, both Vale's, neither ours:

- `dot_borrow` is `unimplemented` (`expression_compiler.rs:1963`). Demonstrated by a pure-Vale test:
  `compiler_ownership_tests::calling_a_method_on_a_local_will_supply_borrow_ref`, currently failing.
- `get_param_environments` (`overload_resolver.rs:502-509`) matches only `Struct`, `Interface` and
  `KindPlaceholder`. Since the onion refactor put ref-ness *inside* `KindT`, a `&Foo` argument is
  `BorrowRef(Struct(Foo))` and matches none — so a borrowed receiver contributes no param
  environment, and a citizen's internal methods would be invisible to it.

Flagged to Vale2 (2026-07-26) for their medium-term radar. **We route around both**: the fixture
uses by-value `self`, and the top-level-declaration design means resolution comes from the calling
env, so `get_param_environments` is not on our path at all.

**Confirmed live by Vale2, 2026-07-27.** `dot_borrow` is their **largest single cluster at 30
tests**; the design is worked out (six arms, wrap arms peeling to the base kind) and waits on one
shape decision from the architect rather than on discovery. The ref-peel gap is recorded and
unfixed, and they read our `NoImplicitCloneDefinedT` finding — reading a local yields
`BorrowRef(Int)` where `Int` is wanted — as the same family, sitting behind `dot_borrow` in
practice. Their instruction: **keep case 39 parked and expect to write it**, rather than routing
around it permanently. Our corpus programs currently return call results directly instead of
through a local, which sidesteps the same family and should be unwound when it lands.

**Also blocked on Vale2: scope-end drop of a generic citizen** (§9 step 2). Not an interop defect —
pure Vale has it too, with a failing test.

**The lead we routed them has become their design, and the wait is now precise** *(their handoff,
2026-07-27)*. They have ratified a **six-phase call-site pipeline** — prepare, rune-typing,
value-solve, resolve, convert, borrow-check — whose **phase 0 "prepare"** owns *"shape-adjust each
argument to the parameter's wrap chain … then emit sends for runes nothing else determines."* And
`assemble_initial_sends_from_args`, the producer we found consumed nowhere, is named as phase 0's
output outright: *"That producer becomes phase 0's output."* So the mechanism has a home. Two things
gate it, both theirs:

- **What a send *is*, mechanically, is unruled.** A hard seed conflicts exactly when phase 4
  (convert) has work to do; *"conclude if unknown, no-op if known"* was proposed and **rejected** —
  in one direction it is `Equals` and in the other it is nothing. Their instruction is verbatim:
  ***"Do not build until ruled."***
- **Their defect 11 blocks the mechanism.** `compiler_solver.rs:1193` concludes into `result_rune`
  where it means `inner_rune`, so the wrap rule cannot fire in its peel direction — and they assess
  it as *"load-bearing rather than incidental"* for phase 0 specifically.

One detail we did not have: those sends currently go against `full_type_rune` **unpeeled**, which is
*"harmless only because the output is discarded."* So the producer needs a shape change as well as a
consumer, and a fix that only wires up the existing call sites would be wrong.

**One thing they fixed that has not reached us.** `@TNLTZACZ`: a bare type name lowers to `Lookup` +
a zero-arg `Call`, and `ITemplexPT::Call` was routing its own *template* position through that same
lowering, collapsing an applied generic to its return type before its arguments could apply. It hit
40 tests. Their fix is not on `experimental` as of `acd47597c` — expect
`opt_with_undroppable_contents` to move from `rune_type_solver.rs:477` to
`templata_compiler.rs:507` once it lands, and re-measure before diagnosing anything nearby.

**Treat it as an unconfirmed claim rather than a landed fix.** It reached us by mailbox only:
`vcoord-handoff.md` — their durable doc, current to 2026-07-27 — contains **zero** occurrences of
`TNLTZACZ` or `opt_with_undroppable_contents`, and their own capability ladder still lists 29 tests
blocked at `rune_type_solver.rs:477`. So either it has not landed, or it landed and their handoff has
not absorbed it. Measure before relying on either state.

**Also blocked:** tier 2 (needs the LLVM port and the onion relink); the 7 extern-struct tests are
`#[ignore]`d *and* `integration_tests` is commented out of `lib.rs:37`, so un-ignoring is not a
shortcut.

---

## 8. The @SMLRZ trap

ValeRustInterop once baked Rust's *name shape* into the typing pass. It broke three escalating ways
and took a full rollback. The architect's conclusion: Rust's `Vec<i32>::push` form has **no internal
justification in Vale** — it is a foreign rendering concern that belongs at the Simplifying→Backend
boundary, where the projector already exists.

**We are at higher risk than they were.** They read Vale source text, already in Vale's shape, and
had to work to convert it to Rust's. We read `TyCtxt`, where the Rust shape is what we are handed
natively — *preserving* it is our path of least resistance and it is wrong. It already happened once:
the old seam minted `rust.mycrate :: [Struct(Counter)] :: ExternFunction(get)`.

**Self-check for every synthesized declaration:** it should be structurally indistinguishable from
what the postparser produces for the equivalent hand-written Vale source. If the oracle's knowledge
of *which args came from the impl* is visible anywhere in the `FunctionS`, @SMLRZ is being rebuilt.

---

## 9. What `Vec<int>()` needs

The question from 2026-07-25 — *"is `x = Vec<int>();` legal, or must it be
`rust.std.vec.Vec<int>()`?"* — was about **naming**, and the answer given was "bare names are
legal." That half is **done and verified** (case 38): hand-written Vale names a Rust type by bare
name, with no import statement, because the citizen is a `Kind` entry in the reserved `rust`
package's top-level store and `PackageEnvironmentT` unions every top-level store.

`Vec<int>()` *as written* needs four more things, in dependency order.

**1. Path resolution into nested modules — ✅ DONE 2026-07-27.** `Vec` is `std::vec::Vec`, not a
child of the `std` crate root, and the walk was one level deep — so a nested item was not merely
unimported, it was **unreachable**.

An allowlist entry is now a **dotted path** (`instruments.depth_reading`), resolved segment by
segment against `module_children`, which is what clippy and rustdoc both do because neither can
build a key map (§10.2). Three properties worth keeping:

- **Plural by construction.** `resolve_allowlist_path` returns *every* match across every loaded
  crate rather than the first. Rust has no uniqueness rule for names at any depth, and both sibling
  trees shipped the `Option` shape and regretted it — this is the one place §6's "build it plural
  from the start" was cheap to honour rather than retrofit.
- **A single-segment entry is the degenerate case**, descending through zero modules to match at the
  crate root. No "is this a path?" branch anywhere (@NNGZ).
- **Intermediate segments must be modules.** Matching them on name alone would let a struct named
  `vec` swallow the `vec` in `std::vec::Vec` — the same `DefKind` filter the final segment already
  needed, one level up.

Cases 44 and 45 cover the function and type paths; the nested type's *method* came for free, because
discovery runs off the owner's `inherent_impls` and knows nothing about how the owner was reached.

**What this does not do**, and neither is needed for `Vec` to be reachable: the short name is still
the final segment, so registering under the whole path — §10.0's Problem A, which two same-named
imports would need — is untouched and still core. And **re-exports are not traversed**:
`std::vec::Vec` is `pub use alloc_crate::vec`, so its def path is `alloc::vec::Vec` and naming it
the way a user would means following the re-export rather than matching a key. That is the next
piece of Problem B.

This step was **Problem B** in §10.0's split. The `Vec`-specific remainder is the re-export hop plus
the eagerness (§6), which stops being cosmetic the moment a crate the size of `std` is walked.

**2. Generic types carrying their arguments — ✅ DONE 2026-07-26.** `Holder<i32>` and `Holder<bool>`
are now two distinct Vale kinds; case 40 was inverted from asserting the defect to asserting
`("rust-citizen<int32>", "rust-citizen<bool>")`. Four changes, all inside `rust_interop/` bar one
core deletion:

1. `type_kind` reads the ADT's `GenericArgsRef` onto the interned name.
2. A Rust type is now a synthesized **`StructS`** (`synthesize_extern_struct`), registered as
   `IEnvEntryT::Struct` rather than a finished `ITemplataT::Kind`. That is what makes its name
   resolve to a `StructDefinition` templata, the one arm `solve_call_rule` can apply arguments to.
3. `declarations.rs` emits `LookupSR` + `CallSR` for **every** citizen position, generic or not.
4. `import_rust_types` is **deleted** — `precompile_struct`/`compile_struct` do its six
   `coutputs` calls. The core diff is a 7-line deletion of the gated call site, so interop's core
   footprint shrank.

Both derive macros are suppressed with the language's own `DontCallMacro` attribute. The
constructor because a field constructor over zero members claims a layout Vale does not have; the
**drop** because its `GeneratedBody` destructures *members*, so for a zero-member Rust citizen it is
an empty destructor that never reaches rustc — indistinguishable from correct for a type with no
`Drop` impl, and a silently skipped destructor for one that has it. We synthesize our own
`ExternBody` drop instead.

**A fifth change, and the one that made the other four general: `ValeSigType::Citizen`.** A
signature position can now be *a citizen applied to arguments that are themselves positions*, so
`Holder<T>` is expressible where before only a settled `KindT` was. Two things were impossible
without it, both discovered by probe rather than by reasoning:

- `holder_ignore<T>(h: Holder<T>)` **panicked in the oracle** — lowering the ADT's arguments went
  through `lower_ty`, and a `ty::Param` has no `KindT` at all.
- a generic type's `drop` receiver could only be named as an argument-less `Holder`, so
  `predict_struct` zipped one generic parameter against zero arguments.

It also let `synthesize_extern_drop` be deleted: the drop is now an ordinary `ValeSig` with the
receiver as `Citizen { Holder, [Generic(0)] }`, built by the same `synthesize_extern_function` as
everything else. One code path again.

> **►► Remaining gap, now diagnosed properly: a compiler-generated drop call cannot infer its type
> argument. ◄◄** `drop<T>(Holder<T>)` resolves fine when the argument is *written* — case
> `calls_a_generic_function_taking_a_generic_type` proves the exact shape works via
> `holder_ignore<int>(make_holder())`. What fails is the **implicit** case: `get_drop_function`
> calls `find_function` with **no explicit template arguments**, so `T` would have to be inferred
> backwards through the `CallSR` from the concrete argument, and the solve ends `SolveIncomplete`
> with `T` and the receiver rune unsolved.
>
> ~~**That is arch §1.7 behaving as specified**, not obviously a bug: *"Vale does not infer generic
> type arguments at call sites."*~~ **WRONG, and struck 2026-07-27.** That rule was a transcription
> of Sky's, never ratified for Vale, and the architect has struck it from §1.7. So a drop call
> supplying no explicit type argument is **not** behaving as specified — there is nothing to
> specify. This paragraph is left standing rather than deleted because citing an unratified
> inherited rule as authority to *not* fix a defect is the failure mode worth remembering: the rule
> read as locked because it was phrased like the locked ones around it.
>
> ~~Vale's own generic structs must hit this too, and the difference worth chasing is
> **placement**: a derived drop is registered *nested under the citizen's id* and reaches the
> citizen's outer env, while ours is a flat top-level declaration.~~ ~~**Do not "fix" it by turning
> on call-site inference**; that contradicts a locked decision.~~
>
> **Both struck. Placement is not the mechanism, and there was no locked decision.** The inference
> rule was never a Vale rule (§0.3b, arch §1.7). And Vale2's 2026-07-27 pipeline answers the
> placement question from the other side: **phase 0 owns emitting the sends that carry an argument's
> type into a parameter rune**, for every call site, with no reference to where the callee's
> declaration is registered. Harmonious reached the same conclusion independently — their drop calls
> are flat top-level calls and work, *"because the argument rides the call node rather than being
> recovered from the surrounding environment."* Two trees agree that nesting is not what supplies
> the argument. **Relying on argument-driven determination is the direction; it is phase 0, it is
> theirs, and it is unbuilt** — see §7 for the two things gating it.
>
> Non-generic drop is unaffected. Case 40's program therefore consumes both `Holder`s rather than
> letting them fall out of scope, so it pins generic arguments alone; **a case for the drop gap is
> owed** as case 41.
>
> **►► And it is not an interop gap at all — pure Vale has it too, with a failing test. ◄◄**
> `compiler_ownership_tests::opt_with_undroppable_contents` is a hand-written top-level
> `func drop<T>(opt Some<T>)` over a generic struct, and it fails with **`Bad template call`**
> (`rune_type_solver.rs:477`). It is among the 170 the default suite carries unchanged; everything
> we touch is feature-gated and absent from a default build, so it is not ours. Different error from
> ours, same capability: a generic citizen going out of scope.
>
> **►► The architecture doc already specifies the design that would work, and the typing pass
> implements a different one. ◄◄** Arch §15.7 step 4 says `insert_scope_end_drops` appends
> `FnCall { name: "__vale_drop", type_args: [T], args: [Ref(Var(local))] }` — **the type argument
> written onto the call node**, read off the binding, with one generic wrapper doing all the work
> and *"no predicate, no `local_needs_scope_drop` decision."* That is exactly the shape Harmonious
> independently described as why the question never arises for them.
>
> But `insert_scope_end_drops` **does not exist in the tree** (zero hits), and `Compiler::drop`
> instead resolves a *per-type* destructor by overload lookup on the name `drop` with **no explicit
> type arguments** (`destructor_compiler.rs:29`). That divergence is a complete explanation of both
> failures: a per-type destructor for a generic citizen has a type argument nobody supplies, whereas
> one generic wrapper called with an explicit argument has nothing to infer.
>
> **Consequence for us:** we are synthesizing one `drop` declaration *per imported type*, matching
> the implementation rather than the architecture. If the `__vale_drop<T>` wrapper shape is what
> lands, our per-type drops should retire into it — which would dissolve our half of this gap
> without any solver work. Routed to Vale2 2026-07-27; do not build around it until they rule.
>
> **Status of that routing, read off their handoff the same day: not ruled, and the wrapper is not
> on their board.** `insert_scope_end_drops` and `__vale_drop` appear nowhere in it. What they *have*
> ratified is phase 0, which fixes the general mechanism (arguments reach parameter runes) rather
> than drop specifically — and under phase 0 a per-type `drop<T>(Holder<T>)` would resolve from its
> argument with no wrapper needed. So the wrapper may turn out to be unnecessary rather than
> pending. Do not retire our per-type drops in anticipation of either outcome.

> **►► Measured 2026-07-26, and the obvious fix is not where it looks. ◄◄** Filling `template_args`
> in `type_kind` from the ADT's `GenericArgsRef` — which `lower_ty` already has in hand and
> discards — **changes nothing observable**. Case 40 still passes with the two kinds identical.
>
> The reason is one layer down: **a synthesized declaration never carries the lowered kind.**
> `synthesize_extern_function` reduces a `ValeSigType::Kind(kind)` to
> `LookupSR { name: CodeName(vale_type_name(kind)) }` — the type's bare *human name* — and lets the
> solver re-resolve it against whatever the importer registered, which is the argument-less
> template. So the arguments are dropped by the *declaration*, not by the oracle, and any fix that
> stops at `type_kind` is dead weight.
>
> **The rule shape that would work is `LookupSR` + `CallSR`** — bind a rune to the *template*, then
> apply the argument runes to it. That is what `declarations.rs`'s own comment calls *"the extra
> rule the citizen-shaped macros have to emit"*, and what `struct_constructor_macro`,
> `struct_drop_macro` and `anonymous_interface_macro` already do. Each argument is itself a
> `ValeSigType`, so the existing `bind` closure recurses and a generic argument that is itself a
> generic parameter falls out for free.
>
> **But emitting those rules is not sufficient, and this is the part that decides who owns the
> work.** `solve_call_rule`'s forward branch (`compiler_solver.rs:1359`) dispatches on what the
> template rune resolved to, and it accepts `StructDefinition` / `InterfaceDefinition` / the two
> array templates — each of which it hands to `predict_struct` and friends. **What the importer
> registers today is `ITemplataT::Kind`**, and that arm exists too:
>
> ```rust
> ITemplataT::Kind(kt) => { /* binds result_rune to kt, ignoring args entirely */ }
> ```
>
> So a `CallSR` over the current registration **silently passes the argument-less kind through**.
> Not an error — the same silent wrong answer as case 40, moved one layer up. `CallSR` is
> necessary and nowhere near sufficient.
>
> **`ITemplataT::StructDefinition` is the arm that would work, and it is gated on a `StructS`.**
> `StructDefinitionTemplataT { declaring_env, origin_struct: &'s StructS }` holds a *parsed* struct
> declaration, and a Rust type deliberately has none — that absence is what made the core diff
> empty (§4). Two ways forward, and **choosing between them is the architect's**:
>
> 1. **Synthesize a `StructS` per imported Rust type**, symmetric with the `FunctionS` we already
>    synthesize per imported function — and explicitly sanctioned: *"i dont think we should generate
>    .vale source literally. if anything, we'd want to generate FunctionS/StructS ... ones that are
>    wrappers."* Entirely interop-side. **The cost is that it turns on struct-compile machinery over
>    Rust types**, including `get_struct_sibling_entries`' macro-derived **field constructor** — which
>    §9 step 4 argues must *not* exist for a Rust type, since fabricating one claims knowledge of a
>    layout and invariants Vale does not have. It also makes `compile_struct_core:144` live, which
>    §4 flags as inert only while no `StructS` exists.
> 2. **Add an `ITemplataT` arm for a Rust-backed citizen template**, plus its arms in
>    `solve_call_rule` and `resolve_template_call_conclusion`. **Core**, and a new variant on a core
>    enum — but it keeps Rust types out of the struct-compile path entirely, which is the property
>    the whole design has been protecting.
>
> **Correction owed:** an earlier revision of this block said generic types were *"not blocked on the
> architect."* That was wrong, and wrong in the specific way §0.2 warns about — it was written after
> probing the *rule* shape but before probing what the solver does with the registration. The `Kind`
> arm above is the fact that changes the answer.
>
> ### What the two sibling trees said (2026-07-26)
>
> **ReiImpl has generic Rust types working, with a two-instantiation test.**
> `[ReiImpl] tests/rust-interop/ri_23_two_vec_diff_generics.vale` builds `Vec<int>` and `Vec<bool>`
> in one program and returns `15 + 8`. So this is achieved, not theoretical.
>
> **Their route is one the architect ruled out, but the mechanism findings transfer.** They get a
> `StructS` by **generating `.vale` source text** out of process (ValeRuster emits
> `#!DeriveStructConstructor extern struct Vec<T> imm { extern func ... }` to a file, picked up by an
> extra package resolver), which the ordinary lexer/parser/postparser turns into a completely
> ordinary `StructS`. That is *"generate .vale source literally"*, which the architect declined in
> favour of *"generate FunctionS/StructS ... ones that are wrappers."* So the route is out; three
> findings underneath it are not:
>
> 1. **The field-constructor objection dissolves — the language already has the opt-out.**
>    `#!DeriveStructConstructor` is a lexer token producing a `DontCallMacro` macro-call attribute,
>    and `determine_macros_to_call` filters the default macro list by exactly that
>    (`[ReiImpl] FrontendRust/src/typing/compiler.rs:2834`). The constructor macro is never invoked
>    for their extern structs. **In-compiler, the equivalent is to seed the synthesized `StructS`'s
>    attributes with `MacroCallS { include: DontCallMacro, macro_name: DeriveStructConstructor }`** —
>    an existing language feature, not a special case. Note `DeriveStructDrop` is *not* suppressed in
>    their tree; we synthesize our own drop, so we would want to suppress that one too or drop ours.
> 2. **The missing link is the env entry kind, not a new templata arm.**
>    `IEnvEntryT::Struct(struct_a)` converts to
>    `ITemplataT::StructDefinition(StructDefinitionTemplataT { declaring_env, origin_struct })`
>    (`[ReiImpl] FrontendRust/src/typing/env/environment.rs:617`) — precisely the arm `solve_call_rule`
>    needs. We register `IEnvEntryT::Templata(ITemplataT::Kind(..))` instead, which is what deprives
>    us of it.
> 3. **Neither sibling tree has a Rust-specific `ITemplataT` arm.** The agent read ReiImpl's whole
>    enum; there is no `RustCitizenTemplata` or equivalent. Nobody needed one.
>
> **Harmonious keeps foreign types out of the declaration path entirely**, carrying the arguments on
> the *type reference* — their ADT lowering yields `RustType { name, type_args }`, so two
> instantiations are structurally different data and no interning step can discard the args. Their
> constructor/derive problem never arises because struct-compile is never reached. They offered this
> as a third option and correctly scoped it: *"whether that's available to you depends on whether
> `LookupSR`/`CallSR` is the only way to name a parameterized type."*
>
> **It is not available to us without a core change.** §10.1 is the answer: `LookupSR` is the only
> rule variant that names a type and it resolves *by name*; `LiteralSR` carries int/string/bool only;
> **no rule carries a pre-resolved templata.** So Harmonious's shape needs a new rule variant, which
> is core — a bigger core change than the templata arm, not a smaller one.
>
> **Two warnings worth acting on regardless of route.** Harmonious flags that the silent
> `ITemplataT::Kind` arm is the same hazard as the corpse — reachable, silent, wrong — and would
> make it loud or unreachable *as part of the same change*, so the fix cannot be one
> mis-registration away from re-introducing today's bug. And they flag that **`DefId`/`CrateNum` are
> session-local**: fine while identity lives in one compile, wrong the moment it crosses a session
> boundary (a cache, a symbol name, another rustc run). Their answer was content-addressed identity —
> hash the qualified path. §10.0 currently presents `def_path` as *the* durable name without that
> caveat; see §10.11.
>
> **Where that leaves the choice.** Option 1 — synthesize a `StructS` in-compiler, register it as
> `IEnvEntryT::Struct`, and suppress the constructor macro with the language's own attribute — is now
> the strongest: it is what the architect specified (*"FunctionS/StructS ... wrappers"*), it needs no
> core change, its main objection has a precedented answer, and ReiImpl proves everything downstream
> of it works. **It remains the architect's call**, because it reverses the lean recorded above and
> because it makes `compile_struct_core` live over Rust types for the first time (§4's note; our
> citizen outer env is empty, so the specific panic at `:144` should not fire, but that is reasoning
> rather than measurement).

**3. Outbound `GenericArgs` reconstruction.** rustc's real args for `Vec<i64>` are `[i64, Global]` —
type **plus allocator** — while the Vale name would carry `[Kind(i64)]`. Feeding rustc back
(`Instance::resolve`, `fn_sig`) means rebuilding the full list via `generics_of` + `mk_args` +
`re_erased`. Arch §8.10 records this as Option A's sharpest genuine weakness; it is bounded and
memoizable, but it is real bug surface and it arrives exactly here.

**4. Deciding what `()` means for a Rust type.** `Vec<int>(...)` is a *call*, and for a Vale struct
the callee is the macro-derived field constructor from `get_struct_sibling_entries` — which runs
only over parsed `StructS` denizens, so a Rust-backed type has none. That is correct and should stay
correct: Vale is an external consumer, `Vec`'s fields are private, and synthesizing a field
constructor would claim knowledge of a layout and invariants we do not have. So construction has to
route to a Rust **associated function** (`Vec::new`), which needs cases 17 and 22.

**This is the open question for the architect**: should `Vec<int>()` construct at all, or should
Vale source say `Vec<int>::new()` (or an equivalent), leaving the bare-call form to mean "Vale
struct literal" only? The naming answer does not settle it, and nothing should be built for step 4
until it is.

**Two things that arrive with `Vec` regardless.** Eagerness stops being cosmetic — `Vec` alone
brings ~100 inherent methods, and today every importable item gets a declaration *compiled* whether
called or not (§6). And `Vec` has a real `Drop` impl, so tier 2 will exercise `__vale_drop<T>`
reaching rustc's own drop glue for the first time; the typing side already treats drop as an
ordinary function, so nothing changes there.

**Distance, honestly:** steps 1 and 2 are each a solid slice of work with a clear shape; step 3 is
smaller but fiddly; step 4 is a decision before it is code. None of it is blocked on the LLVM port
or on Vale2. A generic Rust type from our *own* fixture — `Holder<int>` — is reachable well before
`Vec`, needs only step 2, and is the right first target.

---

## 10. Name resolution — the design

Restored 2026-07-26. This was worked out in convo-9 and then **thinned to a single §6 bullet** in the
wind-down rewrite, which is precisely the loss this document exists to prevent. The reasoning below
cost a day, an agent sweep of `~/rust`, and two reversals.

**Nothing here is built.** It is trigger-gated on the decision *"if we run into a collision, we
should work on qualified names"*, and case 33 is what pins the current behaviour until then.

### 10.0 Two problems, and the first one is easy

An earlier draft of this section treated name resolution as one problem. It is two, and conflating
them made the easy half look as hard as the hard half.

- **Problem A — a *synthesized declaration* naming a type.** We mint both ends: the `LookupSR` in
  the generated `FunctionS`, and the store entry the importer registers. No user-written name is
  involved anywhere.
- **Problem B — *user source* naming a Rust item.** `import rust.std.vec.Vec`, or a bare `Vec` in a
  `.vale` file.

**Problem A is solved by asking rustc for the canonical name.** `tcx.def_path(def_id)` gives the
definition path; use it as the key on both ends and they agree by construction. Concretely, the Vale
name already has somewhere to put it — `IdT.package_coord` is `{ module, packages: &[StrI] }`, and
arch §8.10 already specifies that *"the module path rides `IdT.package_coord`"*. So `Vec` becomes
`rust.["alloc","vec"] :: Struct(Vec)`, carrying its whole canonical path with no new mechanism.
Today `TyCtxtOracle::new` takes **one** coord and stamps every item with it, which is why everything
lands in `rust.["mycrate"]` regardless of nesting; populating it per item from `def_path` is the
change.

What that buys for A: **no collisions** (def paths are unique, so `alloc::vec::Vec` and a Vale `Vec`
are simply different keys and `panic!("Too many with name")` is unreachable), **no walker**, **no
precedence struct**, **no `get_only_nearest` fix**. The `QualifiedCodeName` variant of §10.3 is still
the vehicle, but its contents come from rustc rather than being reconstructed — the real name rather
than our guess at one.

**Everything in §10.2's second half — re-exports, `visible_parent_map`, clippy and rustdoc walking —
is an argument about Problem B only.** It is about matching a *user's* path against a definition,
and none of it applies when we generate both sides.

**B survives, at the right size:** resolving the handful of paths in `import` statements, once. That
is where a segment walk is genuinely needed and the only place. Everything downstream of an import
is def-path-keyed. Dual registration (§10.5) then reads more cleanly: the def-path key always, plus
the bare user-facing name when the program imported it.

**One consequence to price.** The def path is the *definition* path, so a diagnostic would say
`rust.alloc.vec.Vec` where the user wrote `std.vec.Vec`. That inversion is exactly what rustc runs
`visible_parent_map` for — a lossy BFS it maintains **purely for diagnostics**. So the split is: def
path for identity, a `visible_parent_map`-shaped inversion for error messages, later. Which is how
rustc itself divides it. No @SMLRZ risk: that trap is about Rust's *name shape*
(`Vec<i32>::push` vs `Vec::push<i32>`), not about using Rust's module path as the package path, which
is what `package_coord` exists for.

### 10.1 The problem, in its general form

A synthesized `FunctionS` carries **runes and rules**, not types. Of the rule variants, the only one
that names a type is `LookupSR { rune, name: IImpreciseNameS }` — and it resolves **by name**.
`LiteralSR` carries int/string/bool only; there is no rule that carries a pre-resolved templata.

So rustc hands us a precise `DefId`, and to write it into a declaration we downgrade it to a
source-level string and ask Vale to find it again. The downgrade is unrecoverable, because the
imprecise lookup has no tiebreak:

- `PackageEnvironmentT::lookup_with_name_inner` takes `_get_only_nearest` and **ignores it**
  (`environment.rs:880`), walking builtins plus *every* global namespace and concatenating.
- `lookup_nearest_with_imprecise_name` then does `_ => panic!("Too many with name")` (`:164`).

Not "ambiguity resolved badly" — there is no resolution step, and two hits is a compiler crash.
This is also why the *old* oracle design could defer the question forever: a type arrived by identity
from `fn_sig` and never went through a name lookup at all. Synthesizing declarations is what put us
on the name path, and that is the previously-unpriced cost of the pivot.

### 10.2 Two things ruled out, with reasons

**`RuneParentEnvLookupSR` (@MKRFA) is not an escape hatch.** Three independent reasons: it is
stripped on exactly three paths (call-site overload attempt, array rules, patterns) and the
**function-definition** solve is not among them; if one reaches the solver it now panics outright
(`compiler_solver.rs:1045`); and even where stripped it resolves against the *calling* env by rune
name, and a Vale program calling `make_counter()` has no `Counter` rune bound.

**A full-path key map cannot work *for user-written paths* (Problem B only — see §10.0).**
`library/std/src/lib.rs:575` is `pub use alloc_crate::vec;`,
so the key `["rust","std","vec","Vec"]` names *no definition* — the def path is `alloc::vec::Vec`.
Populate from def paths and users cannot write what they would write; populate from use paths and
you must know every re-export in advance. There is no canonical answer to pick: rustc runs an entire
query, `visible_parent_map`, doing a **BFS over the whole crate forest** purely to invert def-path →
writable-path for diagnostics, and the result is explicitly many-to-one and lossy. Both real-world
analogues of our oracle — clippy's `lookup_with_base` and rustdoc — **walk segments** against
`tcx.module_children`, because neither can build a key map. (Clippy's header also says *"this
function is expensive, use sparingly"*, and it returns `Vec<DefId>` rather than `Option`, because
`memchr::memchr` can resolve to two major versions at once — see §6.)

### 10.3 Representation and resolution are layers, not alternatives

This was the reversal worth keeping. The architect's qualified name is the **source-level
representation**; walking is the **resolution strategy**; they compose.

**Representation — a sibling variant, not a reshape.** Add
`IImpreciseNameValS::QualifiedCodeName(&[StrI])` alongside `CodeName`. Do *not* widen `CodeNameS`
itself: it has ~102 references and is the representation of every source identifier in the language,
so widening makes 99% of names pay an indirection and turns equality/hash from interned-symbol
comparison into slice comparison on the hottest name type in the compiler. Nor does putting a vec on
`LookupSR` help — `r.name` goes straight into `lookup_templata_imprecise`, which reads
`imprecise_to_entries: IndexMap<IImpreciseNameS, Vec<IEnvEntryT>>`. **The key type is the deciding
axis**, so the variant is needed either way and the `LookupSR` change would be redundant with it.

**Resolution — walk, with a per-step primitive.** `children_of(item) -> [(name, ns, item)]`, backed
by our own store for Vale modules and by `tcx.module_children` for Rust, exactly as clippy does.

**And the objection to walking was wrong.** It was claimed that walking needs a namespace *value*
type, which `ITemplataT` has no arm for, making it a new concept in the type system. **rustc has no
such type either**: there is no `Res::Module`; a module is `Res::Def(DefKind::Mod, def_id)`, and
`PathSource::is_expected` rejects `DefKind::Mod` in every position, so it never reaches `Ty`. What
rustc has is a **resolver-result type strictly larger than the typechecker's value universe** —
modules are legal *intermediates*, illegal *finals* — plus a side graph of module nodes keyed by
`DefId` that the typechecker never sees. So we need a resolver-result enum that is not `ITemplataT`,
which is far cheaper than a new templata kind.

### 10.4 Precedence: steal rustc's struct outright

`imports.rs:243-266` is five lines and is the entire three-tier model:

```rust
pub(crate) struct NameResolution<'ra> {
    pub single_imports: FxIndexSet<Import<'ra>>,
    pub non_glob_decl: Option<Decl<'ra>>,
    pub glob_decl: Option<Decl<'ra>>,
}
pub(crate) fn best_decl(&self) -> Option<Decl<'ra>> {
    self.non_glob_decl.or(self.glob_decl)
}
```

**Precedence is a struct field, not a comparison.** "Explicit `use` silently shadows a glob" is
literally `non_glob_decl.or(glob_decl)` — there is no ambiguity to detect, because the two live in
different slots. `E0252` (two explicit uses) is a collision *in the data structure*; `E0659` (two
globs) is a loser stapled to the winner and reported lazily at the use site. That is exactly the
model this doc once called "something Vale cannot express," and it costs one struct with two
`Option`s.

Adopt on day one from `ident.rs:66`: **user-defined names outrank built-in/stdlib names**, so adding
to the stdlib is not a breaking change.

### 10.5 Dual registration makes `import` mean something

Register each Rust item under **both** keys: always the qualified one, and *additionally* the bare
one when the program actually imported it. Then:

- `rust.mycrate.Counter` always resolves;
- bare `Counter` resolves **iff you imported it** — which is import semantics, for the first time;
- the multiplicity panic can only fire when two *imported* things share a bare name, which is
  precisely where Rust raises `E0252`.

Not a hack: `add_entries` already registers each prototype under several imprecise keys
(`environment.rs:568-573` — a template key, a local key, and a `PrototypeName` key). Multi-key
registration is established practice in the exact function this extends.

### 10.6 Scale, and the one thing that makes it hard

`rustc_resolve` is 26,099 lines, but the irreducible kernel — walk a path against a module tree
honouring imports and shadowing — is **~1,500–2,500**. The rest is diagnostics (~40%), macros,
hygiene, rustdoc, lints, and Rust's own features. Editions cost ~120 lines.

**Globs are what force the fixed-point iteration.** Without them, imports form a DAG and a
topological sort suffices. rustc's own fixed point still fails to converge in four open issues from
2024, all of the shape "explicit import shadows a glob whose resolution depends on that glob." That
is a strong argument for Vale not having globs, or having them late.

**Populate lazily.** rustc fills a foreign module's children **on first touch**
(`populate_on_access`), never up front — which is also the fix for our own eager walk (§6).

### 10.7 The cheap escape hatch, if the surface stays small

rustc's own answer for naming library items *from compiler code* is not paths at all — it is
`#[rustc_diagnostic_item = "Vec"]`, 397 of them, a flat `Symbol → DefId` map declared at the
**definition** site. If Vale's prelude only ever needs to name a handful of Rust items, that is a
registry rather than a resolver, and it sidesteps §10.1 entirely for those items.

### 10.8 Vale's own name story, which is separate and more valuable

Independent of interop, and in value order:

1. **Make `import X.Y.Z` bind `Z` in the importing scope.** Registration-time mapping, no resolver
   needed — and the data already exists. A correction worth keeping: `ImportS { range, module_name,
   package_names, importee_name }` carries the full path *and* the imported name intact into
   postparsing. The earlier claim that `importee_name` was discarded was **wrong**; it survives, and
   nothing reads it but a test traversal.
2. **Turn `panic!("Too many with name")` into a real ambiguity error.**
3. **Qualified paths as an escape hatch.**

(1) and (2) are the high-value pieces and neither needs path-walking. Today Vale has *no* way to
disambiguate two same-named items from different packages — no shadowing, no qualification, no
escape hatch; the program is simply uncompilable with a panic. That is a language gap, not just an
interop one.

### 10.9 What was recommended to carry early, and what actually happened

Three things were named as cheap now and expensive to retrofit. Two landed; one did not:

| carried? | item |
|---|---|
| ✅ | unique `DefId`-derived `CodeLocationS` per synthesized denizen |
| ✅ | keying on identity rather than strings — **done 2026-07-26**, via per-item `package_coord` from `tcx.def_path` (§6) |
| ❌ | **qualified interned name in `LookupSR`** — `declarations.rs:115` still emits a bare `IImpreciseNameValS::CodeName(CodeNameS { .. })` |

The third is a genuine divergence from the stated plan. It has not bitten because nothing collides
yet, and it is **still cheap** — and per §10.0 it is cheaper than this doc previously implied, since
the qualified name's contents come from `tcx.def_path` rather than needing a resolver to exist
first. The whole of Problem A is:

1. ~~`TyCtxtOracle` stamps each item's `package_coord` from `tcx.def_path`~~ — **done 2026-07-26**,
   and it is the whole zero-core half. Two crates' same-named types are now two Vale types.
2. add `IImpreciseNameValS::QualifiedCodeName(&[StrI])` plus its interner and humanizer arms
   — **core**, `postparsing/names.rs`;
3. `declarations.rs:115` emits it instead of a bare `CodeName` — ours;
4. registration derives the same key, so the two ends match — **core**, and **not the one-arm change
   this list previously implied.**

> **►► Correction to step 4, found while writing case 25. ◄◄** `get_imprecise_name`
> (`environment.rs:435`) takes an **`INameT`**, not an `IdT` — it sees the local name and never the
> package coordinate — and `add_entries` keys every store entry through it (`:567`). So there is
> nothing in scope from which to build a qualified key, and adding a match arm cannot work. The two
> shapes that can: thread the coordinate into `get_imprecise_name`/`add_entries`, or let a caller
> supply the imprecise key explicitly and have the interop side pass a qualified one. The second
> keeps the change off every existing call site, at the cost of a wider `add_entries` signature.
> Deciding between them is the architect's, since both touch core.

None of that needs a walker, a precedence struct, or the `get_only_nearest` fix — those are
Problem B.

**The trigger for this work has now fired**, rather than being hypothetical: case 25 is a real
program that panics, and §5.1 records it. Until step 2–4 land, two imported Rust types sharing a
short name is a compiler crash.

### 10.9b Nobody has solved the collision — and `DefId` is session-local

Two findings from the sibling trees, 2026-07-26, both bearing on the design above.

**Neither sibling tree solved same-short-name collisions.** An agent read ReiImpl end to end for
this: no qualified-name surface syntax, no per-crate mangling at the Vale-source level, no
disambiguation in their generator, no fixture, and **the identical panic text at
`[ReiImpl] FrontendRust/src/typing/env/environment.rs:213`**. Their docs' own limitations and
troubleshooting sections never mention it. Harmonious hit the hazard and fixed it with a
**provenance filter** at six-plus sites rather than with naming — a narrower problem than ours.

They do get per-crate package coordinates for free (`rust.std.vec`), because a user writes
`import rust.std.vec.Vec` and the coordinate falls out of the path. But **lookup is still by bare
imprecise name unioned across every namespace**, so the collision is latent there exactly as here.
Qualification exists only *downstream of typing*, for ABI purposes (`rustifySimpleId` →
`std::vec::Vec<i32>` for the backend pragmas), derived after the name has already resolved — so it
does not help.

**Conclusion: there is nothing to steal here.** §10's design is ours to get right, and its value is
correspondingly higher.

**And `DefId`/`CrateNum` are session-local.** `tcx.def_path`'s crate component is a `CrateNum`,
assigned per compilation session and not stable across invocations. That is fine while identity
lives inside one compile — which is all §10.0's Problem A needs today — and **wrong the moment
identity crosses a session boundary**: persisted to a `.vale-cache`, embedded in a symbol name, or
compared against something a different rustc run built. Harmonious hit exactly this and their answer
was **content-addressed identity**: hash the qualified path so independent compilations compute the
same id with no shared session state, which is also what arch §10.8's typeids already do for Vale
types.

So §10.0's *"ask rustc for the canonical name"* is right, with the refinement that what gets
**stored** past the session boundary must be the path or its hash, never the `DefId`. Arch §8.10
already says the name serializes as a path and re-resolves at universe-load, so the architecture
agrees; this is a caveat on the interop-side implementation, not a change of direction.

**One more from Harmonious, worth carrying into the design:** their resolver returns
`Option<DefId>` and takes the first match, which cannot represent the two-major-versions case
(`memchr::memchr` resolving to two crate versions at once) that our own clippy survey found. Their
words: build it plural from the start. §6 already records the shape; this is a second tree
confirming it bit.

### 10.10 Vale2's dispatch model shrinks Problem B further

Source: `/Volumes/V/Vale2/vcoord-handoff.md`, *"Mission — Overload resolution & dispatch model
redesign"*. Still **unstarted** as code, but **no longer unratified**: a batch of rulings landed
2026-07-27 and the section is now marked *"do not re-open."* What was ruled — and what stayed open —
matters to us differently, so both are recorded below rather than the old blanket caveat.

> **►► RULED 2026-07-27, and one of them changes what we can rely on. ◄◄**
>
> - **The candidate filter is final and purely static.** *"Params match the args"* is decided
>   **before any value-solving**, from three things available with no solving at all: **arity**, each
>   parameter's **wrap chain** (`type_outer_ref_rules` is a list of `BorrowRef`/`WeakRef`/`OwnRef`
>   rules, so the variants read directly), and each parameter's **value-type template name** (or "it
>   is a bare rune, which accepts anything"). Solving never eliminates a candidate; exactly one is
>   ever solved.
> - **Overlapping overloads are outlawed**, the same coherence rule they applied to impls — two
>   functions whose parameter shapes could both accept one argument tuple is an error, not a
>   resolution question. Checked at the call site before solving, with a declaration-time check only
>   where it is local to one file.
> - **No most-specific-common-ancestor**, and **a generic argument needing an upcast must be written
>   explicitly** (`launch<int>(&Firefly<int>())`). Consequence they state: **no impl walking anywhere
>   in phases 0–2.**
>
> **Still open, and it is the one our narrowing rests on:** whether *"mentions T in a parameter"*
> counts `&Ship` as mentioning `Ship`. They have promoted it from a detail to
> **"OPEN, AND NOW LOAD-BEARING"** — with the tiebreaker deleted, it decides whether an ordinary
> `clone(&myShip)` is ambiguous, and *"nothing else covers for a wrong answer here."* So §10.10's
> shrinkage of Problem B is real but **contingent**: it assumes a namespace model whose membership
> rule is not settled.

**Three concerns this doc had been conflating.** Verified against our tree 2026-07-26:

| concern | mechanism | behaviour on >1 |
|---|---|---|
| naming a **type** (`LookupSR`, a Vale type annotation) | `lookup_nearest_with_imprecise_name` | **`panic!("Too many with name")`** |
| collecting **function** candidates | `lookup_all_with_imprecise_name` — *plural* (`overload_resolver.rs:187`) | normal; overload resolution scores them |
| which **namespaces** are searched | union of the *argument types'* namespaces + explicit imports | a foreign function is not a candidate at all |

**Only the first panics.** Every `lookup_nearest_with_imprecise_name` call site is a type or rune
lookup — `struct_compiler_core.rs:287`, `expression_compiler.rs:550`, the array element types,
`templata_compiler.rs:1343`. **A correction this doc was carrying: two same-named *functions* do not
collide.** They resolve cleanly or produce `CouldntNarrowDownCandidates`. Case 33 is rewritten
accordingly.

**Two rules of the redesign do the work:**

- *"No specificity, no phases, no fallback, no tiebreakers. Two equally-matching candidates is always
  an ambiguity error."* This retires the convo-8 worry about whether a Rust callee could outrank a
  same-named Vale one. It cannot — the outcome is a designed error, never a silent ranking.
- *"A function lives in type T's namespace iff (a) it's defined in T's file AND (b) it mentions T in
  a parameter"*, and candidates come from *"the union of namespaces of every arg type at the call
  site"* plus explicit imports. **Scope is determined by argument type, not by a shared ambient
  namespace** — so a Vale call with no Rust-typed argument never sees a Rust function, and most
  collisions cannot form.

**It ratifies the seam collapse.** *"`x.foo()` and `foo(x)` search the exact same candidate set. No
Self-based namespace, no separate dispatch path for dot-syntax."* A Rust method as a top-level
function whose first parameter is the receiver **is** a member of the receiver type's namespace under
this rule.

**And it names one thing we built against the model.** `rust_package_stores` puts Rust functions in
the reserved `rust` package's top-level store, and `PackageEnvironmentT` unions *all* top-level
stores — so they are **ambient**, findable from every call site in the program. That is precisely
what the namespace model replaces. Harmless at a hand-written allowlist; the fix, when the dispatch
redesign lands, is to place a Rust function in the namespace of the types its parameters mention
rather than in a global store. **Do not deepen the dependence on ambient visibility meanwhile.**

Two smaller carries: **`Ship` and `&Ship` are different namespaces** (relevant once our receivers
stop being by-value), and **`is_type_convertible` loses both of its overload jobs, not just the
tiebreak** — corrected from an earlier reading of this section. It had been recorded as "the
redesign deletes the exact-vs-coercion tiebreaker, at which point `is_type_convertible` collapses to
a boolean." Under filter-is-final it does not collapse, it **stops being part of overload resolution
at all**: the tiebreaker is deleted, *and* membership no longer routes through it either, because a
purely static filter never asks "does this convert?". What survives is phase 4's real conversion and
the gate-checks, both of which want `convert()` rather than a predicate. The cluster §7's borrow path
is blocked behind is downstream of that.

**Net effect on Problem B.** It does not disappear, but it stops being a *language-wide* precedence
problem and becomes a *type-name* one: the only path that can still panic is a Vale type and a Rust
type sharing a bare name, reached from hand-written Vale source. Which is exactly what `import`
scopes, and exactly where Rust raises `E0252`.
