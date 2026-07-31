# Rust interop — state, plan, and handoff

**Start here.** This is the working document for the Rust-interop arc: where the design landed, what
is in the tree right now, what is next, and what is blocked on whom.

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
condition that justified them.** Both were found by asking "why do we believe this?" rather than by
anything failing.

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
  The counsel to mechanize it — a rule is only what you believe, a check is what stops you — offers
  two options and omits the one that beats both. No lint is planned.
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

### 0.7 The sibling implementations, and the trap in reading them

- **`/Volumes/V/RustInteropReiImpl`** is a worktree of a branch of *this* repo, so its `FrontendRust/`
  is the same language and largely the same code — findings transfer directly. It is where the
  `extern`-as-body-kind design and working extern generics live.
- **`/Volumes/V/ValeRustInterop`** is the older Scala ancestor. Its value is archaeology: it preserves
  abandoned experiments as commented-out corpses, including the `lift` flag that baked Rust's name
  shape into the typing pass and was rolled back (§8).

**An agent surveying a sibling tree will report *its* `file:line` as if it were ours.** This has cost
real time twice; one plan section recommended a whole design on the strength of line numbers from a
pass that had been retired outright here. Require every survey to state which tree each citation
belongs to, and re-verify any load-bearing claim against our own source before acting on it.

### 0.8 Lessons learned

Wisdom, not events. Prune an entry when nobody could act on it.

**Traps**

- A rule you do not remember adopting, phrased with the same confidence as the ones around it, may
  have no provenance at all. Check for a Q-ref, a convo, or a commit **before** obeying it — one such
  rule was cited here to justify not fixing a real defect, and turned out to be a transcription whose
  rationale sentence had been dropped in the copy.
- A component asking for a surprising capability is usually doing something at the wrong time. Asking
  why the importer needed `&mut CompilerOutputs` is what exposed prototypes being minted before
  instantiations existed.
- A special case that returns the same answer for two different situations is hiding something. The
  `Vec::new()` guard meaning both "no methods exist" and "methods exist elsewhere" is what unravelled
  into the current design.
- When a problem resists a clean answer, suspect that it is two problems. Name resolution was one
  until it was two, and the collision problem was one until it was three; both then went from hard to
  nearly free. The tell is that you are defending a position rather than answering a question.
- `~/rust` is a full checkout and reading it is cheap. Four decisions here turned on what rustc
  actually does, including that `visible_parent_map` is lossy and kept only for diagnostics, and that
  there is no `Res::Module`.

**Architect preferences, generalized**

- Don't treat non-generics as special cases — the degenerate case should fall out of the general
  path, never branch around it (@NNGZ).
- Methods are not special: a method is a function whose first parameter is the receiver. A design
  that reintroduces a method-shaped path is going the wrong way even when it is locally convenient.
- Uphold a property with the type system and API shape first; a check over our own source is the
  weakest available mechanism and goes quiet when the code moves (§0.3c).
- Ask whether a mechanism helps Vale *outside* interop. One only interop wants is suspect; one the
  language wants anyway is usually the right shape.
- Read the architecture doc before proposing an architecture change. It is authoritative, and a
  proposal to gate the C++ backend away was simply wrong on ground §1.7 and §5 already covered.

**Recurring agent mistakes**

- I probe one layer, find a clean answer, and stop. The generic-types question got three confident
  and different answers this way. An answer that lets you stop looking — especially *"not blocked"* —
  is the one to distrust.
- I reason about a seam without first asking **whose object it is**. "Is this a Vale object or a Rust
  object?" ended several turns of circling on drop that no amount of reasoning about the mechanism
  would have resolved.
- I relay a subagent's numbers without checking whether a different framing dodges them.
- I trust a tool's report that an edit was rejected. Verify the file.
- I compress a doc at wind-down and thin the reasoning with it. The §10 design was once cut to a
  single bullet and had to be rebuilt from a transcript. **A section that is long because the
  reasoning was expensive is correct as-is** — the compression target is stale state, never argument.

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

Measure before quoting anything here:

```
cargo test --manifest-path ./FrontendRust/Cargo.toml --lib --features rust_interop
cargo test --manifest-path ./FrontendRust/Cargo.toml --lib
cargo run --manifest-path ./FrontendRust/Cargo.toml --features rust_interop \
    --bin valec-rs -- <fixture-dir> <out-dir>
```

**Both suites carry the onion arc's failures, and that number is the bar.** It is the same in the
default and interop configurations, because everything interop adds is behind the feature gate. The
architect has ratified this as the commit bar — *"typing pass builds, some typing tests pass"* — so
what matters is not the absolute count but that it **does not move**: a change in either direction is
a stop, not a footnote. Take a reading before starting and compare after.

Interop's own count is the default count plus the corpus (§5.1). The driver exits 0. Warnings are
all pre-existing and unrelated — a change there is also a stop.

**The full gate cannot run at all.** `cargo build` exits 101 on `src/bin/valec/`, which references
`backend_ffi` and `pass_manager` — commented out of `lib.rs` by the onion arc — so neither nextest
backend can build its targets. This is not a red suite; it is a suite that cannot start, and it is
unchanged by every commit so far. `--lib` is the ratified substitute. Do not spend time diagnosing
it.

**Interop's footprint on core is a net deletion.** Nothing is gated into the core files except the
one loop in `Compiler::evaluate` that pushes Rust stores into the global environment; the naming work
(§10) added a path to `LookupSR` and a free function beside it, and removed more than it added.

### Where things live, and why there

- **`corpus.rs` sits in the interop module, not the test tree.** Tier 2's likely home,
  `end_to_end_tests`, is an ordinary `pub mod` and cannot see anything behind `cfg(test)` — a corpus
  in the test tree would be invisible to it, and the two tiers would drift into two copies of each
  program. It holds data only: no assertions, no AST walking.
- **`fixtures_two_crates/` serves two questions from one directory** — a colliding `Widget`/`Widget`
  pair and a non-colliding `Gadget`/`Doohickey` pair. Which question a case asks is decided purely by
  its allowlist, which is what "scoping is membership in the allowlist" means in practice.
- **`fixtures_broken_rust/` does not parse, on purpose.** It is the input to a passing regression
  test proving a broken fixture costs one case rather than the run. Anything extending the
  fixture-validity check must keep skipping it.
- **The test harness's extractor is higher-ranked** — `for<'s, 't>` with `R` fixed outside the
  quantifier — so its result cannot mention the arena lifetimes. "Only owned data escapes the
  callback" is therefore a compile error to violate rather than a rule to remember.
- **The driver binary carries no assertions.** It compiles, reports and exits; the assertions are the
  corpus. It exists as the seed of the real `valec-rs` (arch §3.2), which is the only reason it needs
  to be a binary at all.

### Deletions worth not undoing

Each of these is gone because keeping it was the hazard, not because it was merely unused.

- **The oracle's name-keyed lookups** — `resolve_method` and `resolve_function` matched Rust items by
  human-name string, which is the @ATAFLBZ hazard in its purest form. A dead-but-callable name
  matcher is how that comes back. Deleting them also made *"nothing queries the oracle per call
  site"* unrepresentable rather than merely tested, which is why case 35 is not written.
- **`FixtureOracle`** — the arc's only fake, deleted once tier 1 could host a real `TyCtxt`. §0.3 has
  the standing rule and arch §26b.3 the reasoning; read its obituary carefully, because its *specific*
  weaknesses are why it was easy to remove, not why it was wrong.
- **`Source::rust` and `resolve_rust_package` in `code_source.rs`** — added for an `import rust.X.Y`
  path this design never took, since a Rust type arrives by inference from a signature. That file
  carries no interop cfgs at all as a result. A comment there records what would bring them back.
- **`vale_type_name`'s citizen arm** in `declarations.rs` — it turned a citizen into a bare human
  name, which is exactly what the package path exists to prevent. Only builtins reach that function
  now.

---

## 3. Decisions locked this arc

- **Global `panic = "abort"`** — ratified; it was already arch §1.7/§16, so this confirmed rather
  than changed the architecture. Dissolves the `Void`/`Never` destructor-return constraint instead
  of engineering around it. Known cost: `catch_unwind` does not work, including inside Rust
  libraries that sandbox with it.
- **A Vale value is moved into its destructor; a Rust-backed value is not ours to destruct.**
  Arch §15.7 applies one mechanism to both and is wrong to. **Vale has no drop-in-place** —
  `drop(self T)` takes the value by move. A *Rust-backed*
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
  the @PRIIROZ comment in `function_compiler_core.rs` plus four sources in the sibling tree. The @SMLRZ re-split
  projector already exists at the Hammer boundary.
- **Two test tiers, no fixture oracle** — arch §26b. **Tier 2 asserts on program output only**, the
  way Vale's existing end-to-end tests already do; it does not re-assert tier 1's structure. So a
  case is `(Rust fixture, Vale program, expectation)` where the expectation is either "compiles and
  returns N" or "fails with error E", and the two tiers read the same case — tier 1 checking the
  compile half plus the typed AST, tier 2 running it and checking N. Cases that must *not* compile
  are tier-1-only. The Vale program lives in a shared Rust `const` so both tiers read one text; no
  on-disk schema is needed for that, and none should be invented until something needs it.
- **Phase order**: *a lot of things working in the typing pass
  → the LLVM 16 → ~21 port → codegen/instantiator → more typing pass → more codegen*, alternating.
  We are in the first phase. The port is what unblocks tier 2, the instantiator, symbol naming, and
  the @SMLRZ wire-format re-split — so "blocked on the LLVM port" throughout this doc means
  "scheduled, not stalled."
- **Serialization deferred.** Tier 1 needs none (§5), so typing-pass serialization can be designed
  into core on its own merits later. Note `von/` is only the 103-line value model and is commented
  out of `lib.rs`; VonHammer was never ported. Use `serde_json` if and when it happens.

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
  and both `infer_compiler.rs` and `edge_compiler.rs` depend on it holding **prototypes** (function
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
- **`get_imprecise_name` in `environment.rs` takes an `INameT`, not an `IdT`** — so it sees the
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
- **`integration_tests` is commented out of `lib.rs`.** Anything under it does not compile or
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
- **A synthesized declaration has to stay *statically filterable*, and a rule's *placement* is half
  of that.** Vale2's ratified candidate filter (§10.10) reads three things off a parameter with no
  solving: arity, the **wrap chain** from `type_outer_ref_rules`, and the **value-type template
  name** from `value_type_rules`' outermost `Call` templated on a `Lookup`. Both are per-parameter
  buckets, so a parameter's type rules belong to the parameter and never to the function:
  `synthesize_extern_function` builds each parameter's vector inside the loop that creates the
  parameter and hands it straight to `ParameterS::new`, leaving only the return type's rules in the
  header. There is no shared list for a parameter's rules to leak into, which is what keeps
  @PFVSZ's split true by construction rather than by remembering.

  **Consequence: the `CallSR` is load-bearing for a second reason.** @NNGZ says emit it at zero
  arguments because non-generic is the degenerate case; this says emit it because a citizen
  position with no `Call` presents no readable template name. And an empty bucket does not read as
  *absent* — it reads as *"a bare rune, which accepts anything"*, so such a parameter becomes a
  candidate for **every** call of matching arity rather than none, which under filter-is-final and
  `>1 → ambiguity` makes ordinary Vale calls collide with Rust ones. Anyone "simplifying" either
  the emission or its placement breaks overload resolution in a way today's suite cannot catch,
  since the filter does not exist yet. What today's suite *does* catch is the rules going missing
  altogether: blanking a parameter's bucket leaves its rune unconcluded and 38 corpus cases fail in
  `check_defining_conclusions_and_resolve`.

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
| 13 | `declines_an_unsigned_integer` ✅ | the `IntT`-has-no-signedness gap — importing `u32` would hand back a plausible `i32`. Declines by the same exit as an alias |
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
| 32 | `wrong_generic_arity_does_not_resolve` ✅ **fail** | **excess** type arguments do not resolve — three named against `pick<A, B>`'s two slots. Under-supply is *legal* and deliberately not pinned here: `pick<int>(3, true)` deduces `B` from the argument, so a case written against that form tests inference's absence rather than arity. Non-vacuous against case 7, which compiles `pick<int, bool>` from the same fixture and allowlist |
| 33 | `a_vale_function_and_a_rust_function_with_the_same_name` ✅ | that same-named functions **do not** collide — **now measured, not predicted.** Both reach overload resolution as candidates (one `package_coord: test`, one `rust.["mycrate"]`) and the outcome is the designed `CouldntNarrowDownCandidates` error, never a panic. Exactly §10.10's split, and the deliberate contrast with case 25's type-name panic. Note the variant has **no trailing `T`**, unlike most `ICompileErrorT` arms |
| 34 | `a_fatal_rustc_error_costs_one_case` ✅ | a broken fixture cannot take the suite down |

**F. Provenance and vacuity** — that our machinery ran, and only where it should.

| # | case | pins |
|---|---|---|
| 35 | `no_oracle_query_happens_per_call_site` — **not written, and should not be.** The per-call-site queries are gone from the trait, so the property is unrepresentable rather than tested. A case asserting an absence would be strictly weaker |
| 36 | `a_program_using_no_rust_items_compiles_with_an_oracle_present` ✅ | an oracle in scope costs an ordinary Vale program nothing |
| 37 | `no_extern_function_name_reaches_an_environment_store` — **not written, same reason as 35.** `get_imprecise_name`'s `ExternFunction` arm is deleted, so an extern name reaching a store fails loudly at the catch-all instead of quietly working. Note the limit on how that was established: a probe shows an arm is unreached *by this suite*, which is weaker than unreachable while so many tests stop at a first blocker |

**G. Vale source naming Rust items** — the half of the naming story that is *not* about synthesized
declarations.

| # | case | pins |
|---|---|---|
| 38 | `vale_source_can_name_a_rust_type` ✅ | hand-written Vale naming a Rust type by bare name, with no import statement. It works, via the citizen's entry in the reserved `rust` package store plus `PackageEnvironmentT`'s flat union — easy to assume otherwise, which is why the case exists |
| 39 | `vale_source_calls_a_method_on_a_named_rust_parameter` | the same, with a body that *uses* the parameter. **Blocked**: reading a parameter yields `BorrowRef(Counter)` where `get(self Counter)` wants it owned, and `is_type_convertible` in `templata_compiler.rs` panics on the borrow read-out. Vale2's, per §7 — write it when they land the fix |
| 40 | `a_generic_rust_type_carries_its_arguments` ✅ | a **generic** Rust type imports with its arguments intact — `Holder<i32>` and `Holder<bool>` are two distinct kinds, asserted as `("rust-citizen<int32>", "rust-citizen<bool>")`. The program consumes both `Holder`s rather than letting them fall out of scope, so it pins arguments alone and not the drop gap |
| 41 | `a_generic_rust_type_gets_a_scope_end_drop` | **not written, and Vale2's.** A compiler-generated drop call supplies no explicit type argument, and inference does not happen. **Pure Vale has the same gap** — `compiler_ownership_tests::opt_with_undroppable_contents`, a hand-written top-level `func drop<T>(opt Some<T>)`, is among the 170. Not an interop defect; see §9 step 2 |
| 42 | `calls_a_generic_function_taking_a_generic_type` ✅ | `holder_ignore<T>(Holder<T>)` at `<int>` — a citizen *applied to its own parameter* in argument position. The shape `pick<A, B>` does not reach, and what `ValeSigType::Citizen` exists for |
| 43 | `every_fixture_stub_is_valid_rust` ✅ | a fixture cannot rot into invalid Rust unnoticed. Tier 1 sees only *parse* errors, so nothing else covers a stub that type-errors. Skips `fixtures_broken_rust`, which is unparseable on purpose |
| 44 | `imports_an_item_from_a_nested_module` ✅ | an item below the crate root, named by a dotted path — the shape `Vec` needs. **The first case a root-only walk fails**: every other case sits at a root, which is the degenerate path, so all of them passed while nested items were structurally invisible |
| 45 | `imports_a_type_from_a_nested_module` ✅ | the same at a different `DefKind`, plus a method on the nested type. A walk could plausibly descend for functions and not for types; the method came for free, since discovery runs off the owner's `inherent_impls` and never asks how the owner was reached |
| 46 | `imports_through_a_re_exported_item` ✅ | a re-exported name, which is the shape `std::vec::Vec` actually has. **Written expecting red, passed immediately** — `module_children` reports a re-export with its `Res` naming the definition, so taking the `DefId` from the `Res` follows it invisibly. Intra-crate only; cross-crate is untested |
| 47 | `imports_through_a_re_exported_module` ✅ | descending *through* a re-exported module rather than landing on a re-exported item — a walk could handle the destination and not the intermediate hop. It handles both |
| 48 | `a_program_using_everything_at_once` ✅ | **the composition case.** Sixteen mechanisms in one program and one import list, plus three declined items that must not disturb them. Every other case is narrow so failures localize, which is right for diagnosis and useless for *"do these coexist?"* — interference is its own failure class (a shared name resolving to the wrong item, an import-order dependency, a drop that works only when it is the only drop) and no narrow case can see it. Asserts on the **callee list**, not the return value: a program this size could return 31 while resolving half its calls wrongly |

### 5.2 How to run a slice here

**Write the failing observation first, even when the cause looks obvious.** A fix written without a
red has twice changed nothing observable and had to be reverted — the failure a minute of writing
would have shown.

**Read the mechanism of a red, not just its presence.** A case going red for the *predicted* reason
and then red again for a *different* one is how the identity half of the collision problem got
separated from the naming half. And the most useful reds have been the ones that failed unexpectedly:
a probe written to distinguish "drop is special" from "citizen-applied parameters are broken"
panicked somewhere neither hypothesis predicted, which is what produced `ValeSigType::Citizen`.

**A case that can only assert inequality has a vocabulary too coarse for its subject** (§26b.4). If
the strongest thing a case can say is "these two differ," fix what it can observe before trusting
the result — two wrong-but-distinct answers satisfy it too.

The corpus in §5.1 remains the RFIGA list going forward.

### 5.3 Next, in order

1. **Surface a declined item's reason at the lookup that fails to find it.** `lower_ty` and
   `lower_sig_ty` both return `Result<_, DeclineReason>`, so the reason exists at enumeration and is
   dropped at `fn_sig` — a `VCOORD` there marks the attachment point. The consumer makes the error
   read *"found `first`, but its return type has no Vale form"* instead of "couldn't find function,"
   which is the whole point: a bare decline produces a lie about a function that plainly exists.

   **The consumer is core, and the shape is the architect's.** `CouldntFindFunctionToCallT` is minted
   in `find_function` in `overload_resolver.rs`, in `array_compiler.rs` and `destructor_compiler.rs`,
   and rendered by `humanize_compile_error` in `compiler_error_humanizer.rs`. A declined item is not a
   *candidate* — it never became a callee — so it cannot ride
   `FindFunctionFailure.rejected_callee_to_reason`. Three shapes:

   1. **A field on `FindFunctionFailure`** carrying declined items and reasons. Smallest data change;
      every construction site of that struct gains one field.
   2. **A new `IFindFunctionFailureReason` variant** plus a synthetic `ICalleeCandidate`. Reuses the
      existing channel at the cost of a lie in the vocabulary — a "rejected callee" that never was
      one.
   3. **The humanizer consults the oracle.** No data change, but it needs the oracle in scope where
      only an error and an interner are today. Widest of the three.

   A side table is the right producer regardless: poisoning earns its cost only if a poisoned item
   must *participate* in later phases, and this one only has to explain its own absence.

2. **Eagerness** (§6). The half that is ours is the per-type method fan-out — importing a type
   declares every method on it whether called or not, so `Vec` costs ~100. The expensive half is
   core: declarations are compiled by the loop in `Compiler::evaluate` that walks every top-level
   store, so lazy population needs that loop and the lookup driving it. Needs a ruling before
   anything is built.

3. **Cross-crate re-exports.** Re-export traversal works — `module_children` reports a re-export with
   its `Res` naming the definition, so taking the `DefId` from the `Res` follows it — but the cases
   covering it are **intra-crate**. `std::vec` is `pub use alloc_crate::vec`, a different
   `module_children` path, and untested. `fixtures_two_crates` already exists to host it.

4. **Tier 2**, when the LLVM port and the onion relink land: a second runner over the same cases,
   asserting only on what `main` returns. The corpus is shaped for it — that was the point of
   `corpus.rs`. `instantiating/` and `simplifying/` are not merely gated — they are stale and would
   not compile, matching on `ReferenceExpressionTE` variants with zero hits under `typing/`. Vale2
   sizes the relink at roughly three weeks, so this is scheduled rather than open, and nothing in the
   corpus's shape should be traded away to reach it sooner.

**Prefer the Vale program to carry the assertion.** `pick<int, bool>` returning `A` means a swapped
index yields `bool` where `int` belongs and `main() int` will not typecheck — nothing to grep, and it
survives any refactor of how anything renders. The log's remaining job is **vacuity** — proving the
oracle was consulted — which no source program can express.

Where a case does need to look, it looks at *structure*, never at rendering: the log carries a typed
`OracleQuery` beside its rendered line, a compile failure carries the `ICompileErrorT` variant name
beside its detail, and AST assertions go through a test-owned `describe_kind` that names a type the
way source does.

---

## 6. Known defects and open questions

- **A declined signature loses its reason before anyone can read it.** `lower_ty` and `lower_sig_ty`
  both return `Result<_, DeclineReason>`, so the reason is known at enumeration, but `fn_sig` drops
  it on the way out and the eventual failure says "couldn't find function `foo`" about a function
  that plainly exists. That lie is why declining was once rejected in favour of panicking; carrying
  the reason is what makes the panic unnecessary rather than merely relocated. §5.3 step 1 has the
  three shapes, all core.

- **Eagerness.** Every allowed item is resolved, declared and *compiled* whether or not the program
  mentions it. Harmless at a five-name allowlist; `Vec` alone brings roughly a hundred inherent
  methods. Keep the wrapper — it is what lets the ordinary solver do the work, and why generics
  needed no core changes — and attack the eagerness instead, as rustc's own `populate_on_access`
  does. §5.3 step 2 for which half is ours.

  **Do not build a name-scan of the Vale source as a reachability filter.** A Rust item can be
  reached without its name appearing — a drop we synthesize, a method reached through a generic
  instantiation — so the filter would be approximate in the direction that silently drops
  declarations.

- **Diagnostics name a path nobody can write.** A citizen's package coordinate comes from
  `tcx.def_path`, so an error says `rust.alloc.vec.Vec` where the user wrote `std.vec.Vec`.
  Inverting that is exactly what rustc keeps `visible_parent_map` — a lossy whole-forest BFS — to do.
  Identity follows the def path; messages need the written one. §10.0 has the split.

- **Two versions of one crate collide, and the path cannot separate them.** `package_coord_for` in
  `tyctxt_oracle.rs` builds the first segment from `tcx.crate_name`, so two majors of `memchr` yield
  the same coordinate and therefore the same path. A path of human-readable names can never
  distinguish them; it needs the crate disambiguator, or content-addressed identity (§10.9b). This
  is a ceiling of the current representation rather than unfinished work.

- **A bare ambiguous name still panics.** The package path routes *synthesized* declarations around
  the collision, but hand-written Vale naming a type that two packages export reaches
  `lookup_nearest_with_imprecise_name` in `environment.rs` and panics there, with a second `vfail`
  behind it in `lookup_templata_by_rune`. `ICompileErrorT::TooManyTypesWithNameT` already exists, is
  produced by nothing, and its humanizer arm is a `panic!`. Turning the panic into that error fixes
  strictly more programs than any naming change, for less code, and is independent of all of it —
  `expression_compiler.rs` already returns `CouldntFindTypeT` rather than panicking in the analogous
  spot.

- **`RUST_MODULE` is reserved by comment and enforced nowhere.** `rust.mycrate.Widget` is
  unambiguous only while no Vale module is named `rust`. One check, not yet written.

- **Name resolution beyond this is §10**, which carries what was ruled out and why. Its headline: a
  *type* name is the only thing that can panic — function candidates are collected plurally and
  produce a designed ambiguity error — and Vale2's dispatch redesign narrows even that by scoping
  candidates to the namespaces of a call's argument types.

---

## 7. Blocked elsewhere

**The borrow path.** Two holes, both Vale's, neither ours.

`dot_borrow` in `expression_compiler.rs` is unimplemented for the wrap arms, demonstrated by a
pure-Vale test — `compiler_ownership_tests::calling_a_method_on_a_local_will_supply_borrow_ref`. It
is Vale2's largest single cluster; the design is settled (six arms, wraps peeling to the base kind)
and waits on a shape decision rather than on discovery.

`get_param_environments` in `overload_resolver.rs` matches only `Struct`, `Interface` and
`KindPlaceholder`. Since the onion refactor put ref-ness *inside* `KindT`, a `&Foo` argument is
`BorrowRef(Struct(Foo))` and matches none, so a borrowed receiver contributes no param environment.

**We route around both, and that is temporary.** The fixture uses by-value `self`, and the
top-level-declaration design takes resolution from the calling env, so `get_param_environments` is
not on our path. Corpus programs return call results directly rather than through a local, which
sidesteps the same family — reading a local yields `BorrowRef(int)` where `int` is wanted. Vale2's
instruction is to **keep case 39 parked and expect to write it** rather than routing around it
permanently, and to unwind the return-directly workaround when the fix lands.

**Scope-end drop of a generic citizen.** Not an interop defect — pure Vale has it, in
`compiler_ownership_tests::opt_with_undroppable_contents`. Its blocker has moved twice, so read the
current failure before planning against any description of it, including this one.

As of the scout fix (@TNLTZACZ, which made a name in template position keep its `Lookup` instead of
collapsing to a zero-arg `Call`), it fails in `abstract_body_macro.rs`, reporting that no `drop`
override was found and naming why each candidate was rejected. Two reason kinds only —
`SpecificParamDoesntMatchExactly` always at param 0, and `FindFunctionResolveFailure`. Vale2 reads
the whole abstract-body cluster as one capability rather than many problems.

**Underneath it is a general gap, not a drop-specific one.** `get_drop_function` in
`destructor_compiler.rs` passes empty slices for all three explicit-template-arg parameters, so
dropping a generic needs `T` deduced from the argument — and that deduction is dead everywhere.
`assemble_initial_sends_from_args` in `function_compiler_solving_layer.rs` builds exactly the
argument-to-parameter sends that would carry it, and all four callers bind the result and never read
it. Vale2 has ratified a call-site pipeline whose first phase owns emitting those sends, so the
producer has a designed home. Two things gate it, both theirs: **what a send does when the rune is
already determined is unruled** — "conclude if unknown, no-op if known" was proposed and rejected —
and the `BorrowRef` arm of `solve_rule` in `compiler_solver.rs` inserts into `result_rune` in its
peel branch where it means `inner_rune`, so the wrap rule cannot fire in that direction at all.

One detail worth carrying: those sends go against `full_type_rune` **unpeeled**, harmless only
because the output is discarded. Wiring up the existing call sites without reshaping the producer
would be wrong.

**Tier 2** needs the LLVM port and the onion relink. `integration_tests` is commented out of
`lib.rs`, so un-ignoring anything is not a shortcut around it.

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

**Check where a rule lives, not only which rules exist.** The same set of rules in the wrong bucket
is still a divergence, and a silent one: `function_scout` puts a parameter's type rules in that
parameter's `value_type_rules` and never in the function's, so a synthesized declaration does the
same (§4). The @PFVSZ fold unions both, which is why a misplacement typechecks — the reader that
notices is the static candidate filter, and it does not exist yet.

---

## 9. What `Vec<int>()` needs

**Bare names are legal** — `x = Vec<int>()` rather than `rust.std.vec.Vec<int>()`. That half is
done, and case 38 pins it: hand-written Vale names a Rust type by bare
name, with no import statement, because the citizen is a `Kind` entry in the reserved `rust`
package's top-level store and `PackageEnvironmentT` unions every top-level store.

`Vec<int>()` *as written* needs four more things, in dependency order.

**1. Path resolution into nested modules — ✅ done.** `Vec` is `std::vec::Vec`, not a child of the
`std` crate root, so a walk one level deep makes a nested item not merely unimported but
**unreachable**.

An allowlist entry is a **dotted path** (`instruments.depth_reading`), resolved segment by segment
against `module_children` by `resolve_allowlist_path` in `tyctxt_oracle.rs` — which is what clippy
and rustdoc both do, because neither can build a key map (§10.2). Three properties worth keeping:

- **Plural by construction.** It returns *every* match across every loaded crate rather than the
  first. Rust has no uniqueness rule for names at any depth, and both sibling trees shipped the
  `Option` shape and regretted it.
- **A single-segment entry is the degenerate case**, descending through zero modules to match at the
  crate root. No "is this a path?" branch anywhere (@NNGZ).
- **Intermediate segments must be modules.** Matching them on name alone would let a struct named
  `vec` swallow the `vec` in `std::vec::Vec` — the same `DefKind` filter the final segment already
  needs, one level up.

Cases 44 and 45 cover the function and type paths. The nested type's *method* comes for free:
discovery runs off the owner's `inherent_impls` and knows nothing about how the owner was reached.

Re-exports traverse for free too — `module_children` reports one with its `Res` naming the
definition, so taking the `DefId` from the `Res` follows it without knowing. Only the intra-crate
case is covered; §5.3 step 3 has the gap.

This step was **Problem B** in §10.0's split. The `Vec`-specific remainder is the eagerness (§6),
which stops being cosmetic the moment a crate the size of `std` is walked.

**2. Generic types carrying their arguments — ✅ done.** `Holder<i32>` and `Holder<bool>` are two
distinct Vale kinds, pinned by case 40. Four things make that work:

1. `type_kind` reads the ADT's `GenericArgsRef` onto the interned name.
2. A Rust type is a synthesized **`StructS`** (`synthesize_extern_struct`), registered as
   `IEnvEntryT::Struct` rather than a finished `ITemplataT::Kind`. That is what makes its name
   resolve to a `StructDefinition` templata — the one arm `solve_call_rule` can apply arguments to.
   Registering a `Kind` instead hits an arm that binds the result and **ignores the arguments**, so
   the wrong registration fails silently rather than erroring.
3. `declarations.rs` emits `LookupSR` + `CallSR` for **every** citizen position, generic or not.
4. There is no hand-built definition: `precompile_struct`/`compile_struct` do the six `coutputs`
   calls that an importer would otherwise make, so keeping one would double-declare.

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

> **The remaining gap: a compiler-generated drop call supplies no type argument.**
> `drop<T>(Holder<T>)` resolves when the argument is *written* — `calls_a_generic_function_taking_a_generic_type`
> proves the shape works via `holder_ignore<int>(make_holder())`. What fails is the implicit case,
> where `get_drop_function` passes no explicit template arguments and `T` would have to come from
> the concrete argument. §7 has the mechanism and what gates it; it is Vale2's, and pure Vale fails
> the same way.
>
> **Do not chase *placement*.** Vale's own derived drop is registered nested under the citizen's id
> while ours is a flat top-level declaration, and that difference looks like the cause. It is not —
> Harmonious's drop calls are flat and work, *"because the argument rides the call node rather than
> being recovered from the surrounding environment."* Two trees agree nesting is not what supplies
> the argument.
>
> **Do not cite a no-inference rule to justify leaving this unfixed.** Arch §1.7 once carried one; it
> had no provenance here and is struck. Vale infers generic arguments from argument types at call
> sites.
>
> **Do not retire our per-type drops in anticipation of `__vale_drop<T>`.** Arch §15.7 specifies one
> generic wrapper with the type argument written onto the call node, and `insert_scope_end_drops`
> does not exist in the tree — `Compiler::drop` resolves a per-type destructor by overload lookup
> instead. That divergence is real, but the wrapper is on nobody's board, and under the call-site
> phase that emits argument sends a per-type `drop<T>(Holder<T>)` resolves without it. **This is a
> place where the design and the code disagree and neither has been ruled the winner.**
>
> Non-generic drop is unaffected. Case 40's program therefore consumes both `Holder`s rather than
> letting them fall out of scope, so it pins generic arguments alone; case 41 is owed for the gap.

> **What the sibling trees do here, since two of their findings shaped this.** ReiImpl has generic
> Rust types working, reached by generating `.vale` source text out of process — a route the
> architect ruled out in favour of synthesizing `FunctionS`/`StructS` in-compiler. Underneath it,
> two things transferred: the field-constructor objection dissolves because `DontCallMacro` is an
> existing language feature rather than a special case, and the missing link was the **env entry
> kind** (`IEnvEntryT::Struct` converts to the `StructDefinition` templata) rather than a new
> templata arm. Neither tree has a Rust-specific `ITemplataT` arm; nobody needed one.
>
> Harmonious instead keeps foreign types out of the declaration path entirely, carrying arguments on
> the *type reference*. That shape is unavailable here: `LookupSR` is the only rule that names a
> type and it resolves by name, and no rule carries a pre-resolved templata — so it would need a new
> rule variant, which is a larger core change rather than a smaller one.

**3. Outbound `GenericArgs` reconstruction.** rustc's real args for `Vec<i64>` are `[i64, Global]` —
type **plus allocator** — while the Vale name would carry `[Kind(i64)]`. Feeding rustc back
(`Instance::resolve`, `fn_sig`) means rebuilding the full list via `generics_of` + `mk_args` +
`re_erased`. Arch §8.10 records this as Option A's sharpest genuine weakness; it is bounded and
memoizable, but it is real bug surface and it arrives exactly here.

**@ETASTZ belongs to this step and to nothing before it.** It describes Sky's
`build_generic_args_for_item` silently discarding type args that exceed an item's slot count — a
hazard of *populating rustc's `GenericArgs`*, which is what this step builds and which has no
counterpart in our tree today. It is not a statement about checking a Vale call's arity; that
happens at typing, is core, and Vale2's static candidate filter reads arity first. Validate the
truncation at the helper site when the helper exists.

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

The reasoning here cost a day, an agent sweep of `~/rust`, and two reversals. It is long because
what it rules *out* is the expensive part.

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

- `PackageEnvironmentT::lookup_with_name_inner` in `environment.rs` takes `_get_only_nearest` and
  **ignores it**, walking builtins plus *every* global namespace and concatenating.
- `lookup_nearest_with_imprecise_name`, in the same file, then does
  `_ => panic!("Too many with name")`.

Not "ambiguity resolved badly" — there is no resolution step, and two hits is a compiler crash.
This is also why the *old* oracle design could defer the question forever: a type arrived by identity
from `fn_sig` and never went through a name lookup at all. Synthesizing declarations is what put us
on the name path, and that is the previously-unpriced cost of the pivot.

### 10.2 Two things ruled out, with reasons

**`RuneParentEnvLookupSR` (@MKRFA) is not an escape hatch.** Three independent reasons: it is
stripped on exactly three paths (call-site overload attempt, array rules, patterns) and the
**function-definition** solve is not among them; if one reaches the solver it now panics outright
in `compiler_solver.rs`; and even where stripped it resolves against the *calling* env by rune
name, and a Vale program calling `make_counter()` has no `Counter` rune bound.

**A full-path key map cannot work *for user-written paths* (Problem B only — see §10.0).**
`std`'s own `lib.rs` says `pub use alloc_crate::vec;`,
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

`NameResolution` in rustc's `imports.rs` is five lines and is the entire three-tier model:

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

Adopt on day one, from rustc's `ident.rs`: **user-defined names outrank built-in/stdlib names**, so adding
to the stdlib is not a breaking change.

### 10.5 Dual registration makes `import` mean something

Register each Rust item under **both** keys: always the qualified one, and *additionally* the bare
one when the program actually imported it. Then:

- `rust.mycrate.Counter` always resolves;
- bare `Counter` resolves **iff you imported it** — which is import semantics, for the first time;
- the multiplicity panic can only fire when two *imported* things share a bare name, which is
  precisely where Rust raises `E0252`.

Not a hack: `add_entries` already registers each prototype under several imprecise keys
(`add_entries` in `environment.rs` — a template key, a local key, and a `PrototypeName` key). Multi-key
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

### 10.9 How Problem A is solved

Two pieces, both landed, and the shape is worth understanding because it is not the obvious one.

**Identity comes from `tcx.def_path`.** `TyCtxtOracle::new` derives each item's `package_coord` from
its own def path rather than stamping every item with one coordinate, so two crates' `Widget`s are
two Vale types in two packages before any naming question arises.

**A `LookupSR` carries a path, not a name.** `LookupSR.parts` is a segment list, and
`synthesize_extern_struct`'s citizen positions emit the package coordinate followed by the short
name. `lookup_nearest_with_path` in `environment.rs` resolves it: the prefix selects one store, the
last segment is looked up there. A one-segment path narrows nothing and is an ordinary ambient
lookup, so nothing else in the compiler changed shape.

**Why there is no qualified *name* type, and no matching key.** The obvious design — a
`QualifiedCodeName` variant on `IImpreciseNameValS`, emitted by declarations and derived again at
registration — requires both ends to *compute the same key*, and `get_imprecise_name` cannot: it
takes an `INameT`, which is the local name and carries no package coordinate, and `add_entries` keys
every store entry through it. Walking sidesteps that entirely. The walk selects the store the
importer already registered, then asks it for the bare name it is already keyed under, so the two
ends agree by construction rather than by computing a shared key. **If you find yourself needing a
qualified key, you have taken a wrong turn.**

The path is also the wrong home for a *chain* of rules, one per segment: every rune has an
`ITemplataType`, so an intermediate rune would need a templata-type for "a package", and no
vocabulary has a namespace kind. One rule carrying the whole path materializes no intermediate —
which is rustc's split, where a module is a legal intermediate and an illegal final.

**What Problem A does not cover:** a bare ambiguous name written in Vale source still panics (§6),
and the prefix match is a linear scan over a flat table rather than a descent (§10.9c).

### 10.9b Nobody has solved the collision — and `DefId` is session-local

**Neither sibling tree solved same-short-name collisions**, so there is nothing to copy here. ReiImpl
carries the identical panic and no qualified-name surface anywhere; its limitations docs never
mention it. Harmonious hit the hazard and fixed a narrower version with a provenance filter at
several sites rather than with naming.

ReiImpl does get per-crate coordinates for free, because a user writes `import rust.std.vec.Vec` and
the coordinate falls out of the path — but their lookup is still by bare imprecise name unioned
across every namespace, so the collision is latent there exactly as it was here. Their qualification
exists only downstream of typing, for backend pragmas, derived after the name has already resolved.

**`DefId`/`CrateNum` are session-local.** `tcx.def_path`'s crate component is a `CrateNum`,
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
redesign"*. **Unstarted as code, ruled as design** — the section is marked do-not-re-open. What was
ruled and what stayed open matter to us differently.

> **Ruled, and one of these changes what we can rely on:**
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

**Three separate concerns, easily conflated.** Verified against our tree:

| concern | mechanism | behaviour on >1 |
|---|---|---|
| naming a **type** (`LookupSR`, a Vale type annotation) | `lookup_nearest_with_imprecise_name` | **`panic!("Too many with name")`** |
| collecting **function** candidates | `lookup_all_with_imprecise_name` — *plural*, called from `overload_resolver.rs` | normal; overload resolution scores them |
| which **namespaces** are searched | union of the *argument types'* namespaces + explicit imports | a foreign function is not a candidate at all |

**Only the first panics.** Every `lookup_nearest_with_imprecise_name` call site is a type or rune
lookup, spread across `struct_compiler_core.rs`, `expression_compiler.rs`, the array element types,
and `templata_compiler.rs`. **Two same-named *functions* do not
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
