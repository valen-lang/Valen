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
signature, calls a method on it, and drops it — all against a real rustc, with an **empty core
diff**. Nothing downstream of typing exists: no instantiator, no codegen, no linking. The interop
build deliberately does not even link the C++ backend yet.

**The near-term goal** is a typing-pass surface broad enough to trust — the 40-case corpus in §5.1,
plus generic Rust *types* (§9) — and then the LLVM 16 → ~21 port, which is what unblocks everything
after typing. §3 has the full alternating order.

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

### 0.1 The core/interop split is a protocol, not just a layout

Work inside `typing/rust_interop/` and the interop test subtree proceeds freely. **A change to the
core compiler stops and is brought to the architect verbatim — the exact hunks, before landing.**
Precedents: the two `compiler.rs` hunks for the import kickoff, and the `get_imprecise_name` arm.

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

### 0.3 Deferrals are trigger-gated, never vague

Scope pruning is the architect's and it is aggressive — *"lets focus on only the things that block us
from that goal."* But a deferral names its trigger: *"if we run into a collision, we should work on
qualified names."* Where a trigger exists, there should be a case pinning the current behaviour so
the trigger is observable rather than theoretical.

### 0.4 Who is authoritative on what

- **Valen (`valen-design-1.md` / `-2.md`) is the language specification.** One architect owns Valen
  and Vale; a contradiction means the doc is behind a ruling, not that two authorities disagree.
- **Vale2 owns the core compiler and its semantics.** `dot_borrow`, `is_type_convertible`, the
  overload/dispatch redesign, `convert()` unification are theirs; we sequence behind them and route
  findings over. Their handoff is `/Volumes/V/Vale2/vcoord-handoff.md`.
- **Harmonious/Sky is evidence, not authority.** *"we'll be using their prototype as a signal for
  **what works**, but not necessarily **whats best**. keep an eye out for things we can do better
  than they did."* Their operational scars have repeatedly been worth more than their conclusions —
  and several times what they "taught" us turned out to be in our own architecture doc already,
  because they helped write it.

### 0.5 A change must not cost other branches anything

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

### 0.6 Moves that keep finding things

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
- **"Does this help Vale outside interop?"** Applied to qualified names, it produced §10.8 — Vale's
  own name story (make `import` bind a name; turn the multiplicity panic into an ambiguity error) is
  worth more than the interop half and needs no resolver. A mechanism that only interop wants should
  be suspected; one the language wants anyway is usually the right shape.
- **"What does rustc do?"** Four findings that changed decisions: `visible_parent_map` is a lossy
  BFS kept *only* for diagnostics (which killed the full-path key map); `NameResolution`'s two
  `Option`s make precedence a struct field rather than a comparison; there is no `Res::Module`, so a
  namespace value type isn't needed; and foreign modules populate **on first touch**. `~/rust` is a
  full checkout and an agent sweep of it is cheap.
- **Check the architecture doc before proposing an architecture change.** A proposal to gate the C++
  backend away was made and was simply wrong — §1.7 and §5 already covered it, in detail. The doc is
  authoritative; read it before contradicting it.

### 0.7 The sibling implementations, and the trap in reading them

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

### 0.8 Doc discipline: a wind-down must not thin the reasoning

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

All green. Nothing committed since `8d40eff9d`.

| | |
|---|---|
| default suite | **577** passed / 170 failed / 8 ignored |
| interop suite | **586** passed / 170 / 8 |
| driver (`valec-rs`) | exit 0 |
| warnings | 8, all pre-existing |
| **core diff** | **empty** — everything lives in `rust_interop/` and the interop test tree |

The 175 failures are the onion arc's known state, ratified twice as the commit bar ("typing pass
builds, some typing tests pass"). **Treat 577/170/8 and 586/170/8 as the fixed baseline; movement in
either direction is a stop, not a footnote.**

Five of the old 175 cleared on 2026-07-26 when `experimental` brought in the `where implements(T,
IShip)` postparse restoration and the parse/solver error-discarding fixes — upstream's work, not
ours. That rebase also added an `impl_bounds` parameter to `FunctionS::new`; a synthesized Rust
declaration passes `&[]`, which is the truth rather than a placeholder, since rustc discharges a Rust
function's trait obligations and we read no predicates at all.

The interop delta is the 9-case corpus in `typing/test/rust_interop/cases.rs`, all running against a
real `TyCtxt` inside `cargo test --lib`.

### Uncommitted work

- `declarations.rs` — `synthesize_extern_function`: unique `DefId`-derived `CodeLocationS`, generic
  parameters declared and referenced directly (no rule needed — that is what the postparser emits
  for a hand-written generic function), `LookupSR` per concrete type, `ExternBody`.
- `importer.rs` — `import_rust_types` declares the type, its sharedness, a real `StructDefinitionT`,
  empty outer **and inner** envs; `rust_package_stores` emits the type as a nameable `Kind` entry
  plus one declaration per free function, method, and drop.
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
intended corpus: **9 implemented (✅), 31 planned**. Each line says what breaks if the case fails,
because a case whose failure mode nobody can state is a case nobody will fix.

Every case that compiles also declares the value `main` returns, so tier 2 can run the identical
case and check the output. Cases marked **fail** are tier-1-only by nature.

**A. Signatures and lowering** — what a Rust signature may contain and what happens when it can't
be expressed.

| # | case | pins |
|---|---|---|
| 1 | `calls_a_rust_free_function` ✅ | a synthesized declaration resolves an ordinary call |
| 2 | `calls_a_zero_arg_rust_function` | empty parameter list is the degenerate case, not a special one |
| 3 | `calls_a_rust_function_returning_unit` | `()` → `VoidT`, and a call in statement position |
| 4 | `passes_and_returns_a_bool` | a non-integer primitive round-trips |
| 5 | `takes_a_rust_type_as_a_parameter` | a Rust citizen in *argument* position, not just return |
| 6 | `takes_and_returns_a_rust_type` | the same citizen identity on both sides of one signature |
| 7 | `reads_a_generic_signature_structurally` ✅ | generics stay parameters instead of collapsing to one instantiation |
| 8 | `binds_the_second_generic_parameter` | the mirror canary — `pick_second<A,B> -> B` at `<int,bool>` catches an index swap the first canary cannot |
| 9 | `instantiates_a_generic_at_one_parameter` | `id<T>(T)->T` — substitution happens at all; passes under any mapping, so it is a floor not a canary |
| 10 | `instantiates_a_generic_at_a_rust_type` | a citizen as a *generic argument*, not just a parameter type |
| 11 | `declines_an_unrepresentable_signature` ✅ | an un-normalizable alias in return position is dropped, not imported with a hole |
| 12 | `declines_an_unrepresentable_parameter` | the same, in argument position — a different code path |
| 13 | `declines_an_unsigned_integer` | the `IntT`-has-no-signedness gap; **currently panics**, see §6 |
| 14 | `declines_a_float` | the `FloatT`-has-no-width gap; same |
| 15 | `declines_a_signature_naming_an_unimported_type` | @RTMEIZ — reaching a type only through another item's signature does not import it |

**B. Item kinds** — that free functions, methods, drop and associated functions are one path.

| # | case | pins |
|---|---|---|
| 16 | `calls_a_method_on_a_rust_type` ✅ | a method is a top-level function whose first parameter is the receiver |
| 17 | `calls_an_associated_function_with_no_receiver` | `Counter::new()` — an inherent fn without `self` still imports |
| 18 | `calls_two_methods_on_one_type` | method discovery is a list, not a lucky single |
| 19 | `calls_methods_on_two_different_rust_types` | per-type method sets do not bleed into each other |
| 20 | `a_rust_value_bound_to_a_local_gets_a_scope_end_drop` ✅ | the synthesized `drop` exists and resolves |
| 21 | `a_rust_value_returned_and_discarded_gets_dropped` | drop on the temporary path, not just the bound-local path |
| 22 | `calls_a_generic_method` | a method carrying its *own* type params, on top of the container's |

**C. Multiplicity and crates** — that nothing depends on there being exactly one of anything.

| # | case | pins |
|---|---|---|
| 23 | `imports_two_rust_types_at_once` | the importer is a loop, not a single-item path |
| 24 | `imports_from_two_crates` | one store per package coordinate, keyed correctly |
| 25 | `two_crates_exporting_the_same_short_name_stay_distinct` | the @ATAFLBZ identity hazard. **Expected to fail until the `DefId` fix lands** — write it red, fix §6, watch it go green |
| 26 | `a_rust_type_flows_through_two_calls` | citizen identity survives being produced by one call and consumed by another |

**D. Scoping** — that the allowlist is load-bearing and is the only thing that is.

| # | case | pins |
|---|---|---|
| 27 | `an_empty_allowlist_makes_nothing_importable` ✅ **fail** | the positive cases are not vacuous |
| 28 | `an_item_not_in_the_allowlist_is_not_importable` **fail** | the positive control's mirror: the crate exports it, we still can't see it |
| 29 | `an_allowlist_entry_the_crate_does_not_export_is_ignored` | a stale allowlist entry is inert, not fatal |
| 30 | `a_module_named_in_the_allowlist_is_filtered_by_defkind` | `mycrate`'s children include `std`; a name match must not hand back a module where a function was asked for |

**E. Failure modes** — that wrong programs fail, and fail legibly.

| # | case | pins |
|---|---|---|
| 31 | `wrong_argument_types_do_not_resolve` **fail** | a Rust callee competes on `params_match` like any other |
| 32 | `wrong_generic_arity_does_not_resolve` **fail** | arity is checked rather than silently truncated (@ETASTZ) |
| 33 | `a_vale_function_and_a_rust_function_with_the_same_name` | that same-named functions **do not** collide. Candidate collection is `lookup_all_with_imprecise_name` — *plural* — so the outcome is a clean resolution or `CouldntNarrowDownCandidates`, never a panic. See §10.10; the type-name case is what needs pinning, not this one |
| 34 | `a_fatal_rustc_error_costs_one_case` ✅ | a broken fixture cannot take the suite down |

**F. Provenance and vacuity** — that our machinery ran, and only where it should.

| # | case | pins |
|---|---|---|
| 35 | ~~`no_oracle_query_happens_per_call_site`~~ | **subsumed, not written.** The per-call-site queries were deleted from the trait (§2), so the property is unrepresentable rather than tested — a stronger guarantee than a case |
| 36 | `a_program_using_no_rust_items_compiles_with_an_oracle_present` | an oracle in scope costs an ordinary Vale program nothing |
| 37 | `no_extern_function_name_reaches_an_environment_store` | whether `get_imprecise_name`'s `INameT::ExternFunction` arm (`environment.rs:488`) is still reachable — see §6. If entries are only ever `IEnvEntryT::Function`, that core arm is dead and can go |

**G. Vale source naming Rust items** — the half of the naming story that is *not* about synthesized
declarations.

| # | case | pins |
|---|---|---|
| 38 | `vale_source_can_name_a_rust_type` ✅ | hand-written Vale naming a Rust type by bare name, with no import statement. **Verified 2026-07-26** — it works, via the citizen's `Kind` entry in the reserved `rust` package store plus `PackageEnvironmentT`'s flat union. Easy to assume otherwise |
| 39 | `vale_source_calls_a_method_on_a_named_rust_parameter` | the same, with a body that *uses* the parameter. **Blocked**: reading a parameter yields `BorrowRef(Counter)` where `get(self Counter)` wants it owned, and `is_type_convertible` panics on the borrow read-out (`templata_compiler.rs:1209`). Vale2's, per §7 — write it when they land the fix |
| 40 | `a_generic_rust_type_loses_its_arguments` ✅ | a **generic** Rust type imports with its arguments silently dropped — `Holder<i32>` and `Holder<bool>` intern to the same bare `Holder`. The case asserts the *defect*, so it is pinned rather than merely known; invert it when §9's step 2 lands |

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
to be wrong the moment someone ran the code, and each was cheap to check. The corpus in §5.1 is the
RFIGA list going forward — case 25 in particular is specified to be *written red* against the
@ATAFLBZ fix, and case 40 is already written against a live defect.

### 5.3 Next, in order

1. **Hoist each case's Vale program to a shared `const`**, alongside its expected return value, so
   one corpus genuinely feeds both tiers (arch §26b.1). Today the programs are inline string
   literals inside `#[test]` functions, so only the Rust fixture crates are shared. **Cheap at nine
   cases, expensive at forty — do it before growing, not after.**
2. **The `@ATAFLBZ` fix** (§6) — key on `DefId`, add the provenance filter, then the grep fence.
   Early because case 25 is written red against it, and because every case added meanwhile is built
   on top of string matching.
3. **Grow the corpus** to the 40 above. Cheapest first: group A needs only fixture functions.
4. **A fixture compile-check**, so a fixture that type-errors cannot rot unnoticed (§26b.2) — it
   must skip `fixtures_broken_rust/`, which is unparseable on purpose.
5. **Tier 2**, when the LLVM port and the onion relink land: a second runner over the same cases,
   asserting only on what `main` returns.

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

- **The oracle matches by name — one site left, down from three.** `resolve_method` and
  `resolve_function` were two of the three, and both were **deleted 2026-07-26** having lost their
  last callers to the pivot (see §2). What remains is the up-front crate walk
  (`TyCtxtOracle::new`), which decides by `child.ident` string equality against the allowlist.
  Fix: key on `DefId`, add a provenance filter (arch §6.3's `__VALE_STUBS_MARKER` plus the
  DefId-parentage check). Harmonious's advice stands: also write a **grep fence** with an
  allow-marker — the value is not that one site, it is the next one in eight months.
- **The up-front crate walk is insufficient, not merely slow.** `module_children` on a crate root
  yields only direct children, so `std::vec::Vec` would never be found. Recursing is what would make
  it expensive *and* widen collisions. End state: resolve the one path an `import` names.
  **Where we are: recorded, nothing built.** The reasoning is written up in full as a `VCOORD` on
  `TyCtxtOracle::new` — both halves, why recursing is the wrong fix, and what the end state
  enumerates instead (nothing). It is step 1 of §9, so `Vec` is what will force it.
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

  Poisoning needs a small **core** hook — a field on the declaration or a new `ICompileErrorT`
  variant — so it is not this arc's to land unilaterally.

  **Where we are: analysed, not changed, and waiting on the architect.** The panics are still in
  place; `lower_sig_ty` still declines aliases and inherited parameters; the inconsistency stands.
  Nothing should flip until the poison hook is designed, because flipping to a silent decline
  reintroduces exactly the lie that "for now, panic" was chosen to avoid. Cases 11–15 pin whichever
  behaviour is chosen — write them once it is.
- **`get_imprecise_name`'s `INameT::ExternFunction` arm** (`environment.rs:488`) was added to core
  for the prototype-store design. Under synthesized declarations a store holds
  `IEnvEntryT::Function`, so the arm is probably unreachable — but that is unverified, and a dead
  arm in a core file is exactly the "dead but constructible" shape that restores an abandoned design
  by accident. Case 37 is the test that decides it; the cheap manual check is to make the arm panic
  and see whether both suites stay green.
- **Eagerness.** Four layers: the oracle tables every allowed item at construction; a declaration is
  synthesized per item; and the function-compile phase compiles **every** declaration whether called
  or not. Fine at a five-name allowlist; `import rust.std.vec.Vec` brings ~100 inherent methods.
  Harmonious's counsel: keep the wrapper (it is what lets the ordinary solver do the work — which is
  why generics needed zero core changes), attack the eagerness. Synthesize on first reference, as
  rustc's own `populate_on_access` does.
- **`resolve_function -> Option<RustItemId>` is the wrong shape.** clippy returns `Vec<DefId>`
  because `memchr::memchr` resolves to two major versions of the crate at once. Harmonious conceded
  their own resolver has the same defect.
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

**1. Path resolution into nested modules.** `Vec` is `std::vec::Vec`, not a child of the `std` crate
root, and today's walk is one level deep — so `Vec` is not merely unimported, it is unreachable.
This is §6's walk item, and `Vec` is what forces it. Note `std::vec::Vec` is itself a *re-export*
(`pub use alloc_crate::vec;`), whose def path is `alloc::vec::Vec`, so naming it the way a user
would means traversing the re-export rather than matching a key.

This step is **Problem B** in §10.0's split, and it is `Vec`-specific: a generic type from a crate
whose items sit at the root — `Holder<T>` in our own fixture — needs none of it. That is what makes
`Holder<int>` the right first target and `Vec` a later one.

**2. Generic types carrying their arguments.** `TyCtxtOracle::type_kind` builds its `StructNameValT`
with `template_args: &[]` and never reads the ADT's `GenericArgsRef`. Today that is a *silent wrong
answer*, not a gap: case 40 shows `Holder<i32>` and `Holder<bool>` interning to the same kind.
Generic *functions* already work — their parameters live on the signature and Vale's solver
substitutes them — but a generic *citizen* needs the name itself to carry args, which nothing has
built. This is the largest of the four.

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
| ✅ | (partly) keying on identity rather than strings — still open in the oracle, see §6 |
| ❌ | **qualified interned name in `LookupSR`** — `declarations.rs:115` still emits a bare `IImpreciseNameValS::CodeName(CodeNameS { .. })` |

The third is a genuine divergence from the stated plan. It has not bitten because nothing collides
yet, and it is **still cheap** — and per §10.0 it is cheaper than this doc previously implied, since
the qualified name's contents come from `tcx.def_path` rather than needing a resolver to exist
first. The whole of Problem A is:

1. `TyCtxtOracle` stamps each item's `package_coord` from `tcx.def_path` instead of from one coord
   handed to the constructor;
2. add `IImpreciseNameValS::QualifiedCodeName(&[StrI])` plus its interner and humanizer arms;
3. `declarations.rs:115` emits it instead of a bare `CodeName`;
4. `get_imprecise_name` derives the same key for a registered Rust citizen, so the two ends match.

None of that needs a walker, a precedence struct, or the `get_only_nearest` fix — those are
Problem B.

### 10.10 Vale2's dispatch model shrinks Problem B further

Source: `/Volumes/V/Vale2/vcoord-handoff.md`, *"Mission — Overload resolution & dispatch model
redesign"* (lines 898-968). Not started over there, and **not ratified upstream** — the same doc
records (line 317) that design-1 says nothing about how a candidate set is assembled and that Valen's
module/import syntax is an open question, with a flag to compare when it lands. Build toward it;
don't treat it as final.

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
stop being by-value), and the redesign **deletes the exact-vs-coercion tiebreaker**, at which point
`is_type_convertible` collapses to a boolean driven off a dry-run `convert()` — the cluster §7's
borrow path is blocked behind.

**Net effect on Problem B.** It does not disappear, but it stops being a *language-wide* precedence
problem and becomes a *type-name* one: the only path that can still panic is a Vale type and a Rust
type sharing a bare name, reached from hand-written Vale source. Which is exactly what `import`
scopes, and exactly where Rust raises `E0252`.
