# Vale: Compiler & Rust Interop Architecture

This is the master design document for Vale's compiler architecture as it relates to Rust interop. It is the product of an extended design conversation grounded in (a) Vale's existing compiler design and (b) the prior toylang prototype work (`/Volumes/V/Harmonious/rust-interop-architecture.md`, ~7,700 lines documenting the Sky/toylang architecture as of 2026-06-25). It locks in the architectural decisions Vale's first implementation of Rust interop should follow.

The architecture is deliberately opinionated. Where decisions have been made, this document states them as decisions; where alternatives were considered and rejected, the rejection is recorded with reasoning so a future reader can re-open the question with full context. The default reading posture is "Vale is committed to these decisions — change them only with deliberate cause."

This document inherits substantial design content from toylang's architecture doc (the interleaved-monomorphization mechanism, the fork patches, the stub rlib model, the cascade-discovery pattern, the operational invariants). Where Vale's choices diverge from toylang/Sky's, the divergence is called out explicitly with rationale. Where they match, the inherited design is restated in Vale terms without re-deriving the original reasoning — see the toylang doc for empirical provenance.

---

## 0. Document Meta

**Audience.** Architects (top-to-bottom), implementers (the chapters relevant to their subsystem + cross-refs at chapter ends), reviewers (governing chapters for the touched subsystem), and Vale users (§1, §12–§17, §21–§22, plus brief worked examples in chapter bodies).

**Scope.** Vale's Rust interop story — how Vale compiles, cooperates with rustc (in valec-rs mode), exposes items to Rust callers, consumes Rust libraries, and projects Vale source-level concepts (groups, linear types, comptime, async) onto the Rust ABI. NOT in scope: Vale's frontend internals (covered in `typing-pass-design-v3.md`, `instantiator-design.md`, etc.), runtime, or stdlib design — those have their own documents. Non-interop concerns appear only enough to characterize the interop boundary.

**Decision status.** Three categories appear inline:
- **Locked** — imperative phrasing ("Vale uses X"); deviating requires explicit re-opening.
- **Recommended** — "Vale should use X" / "we recommend X"; not yet formally locked.
- **Open** — design space catalogued, decision deferred; tagged `[OPEN]` and enumerated in §29.

Document version: **0.1.0** (initial draft, pre-implementation); moves to 1.0.0 when Vale's first Rust interop implementation lands.

**Reading paths.** Full read ~3–5 hrs; architect path: §§1–7, 13–14, 19–20, 25, 28–29 (~2–3 hrs); implementer path: target subsystem plus its cross-refs; Vale-user path: §1, §§12–17, §§21–22.

**Relation to toylang doc.** Where this document says "matches toylang/Sky" or cites a `§F.<X>` reference, the original empirical narrative lives in toylang's doc at the same section number. This doc doesn't re-narrate those.

**Q-references (Q11, Q45 β, Q66 E, etc.).** Q-refs throughout the doc point at the internal design-conversation transcripts that produced each locked decision. The transcripts live under `docs/historical/` in this repo (`vale-rust-interop-architecture-convo-0.md`, `-convo-1.md`, `-convo-2.md`, and the earlier session at `tmp/claude-conversation-2026-06-26-837eac91.md`). Readers who want the empirical provenance of a specific Q can look up the Q number in those files. Q-refs are editorial scaffolding, not load-bearing citations — the decisions themselves are stated inline at each cite; the Q-ref is provenance.

---

## 1. Goals and Constraints

This chapter records what Vale is trying to be and the non-negotiable design constraints that shape every subsequent decision.

### 1.1 What Vale is, in one sentence

Vale is a memory-safe systems language with first-class compile-time metaprogramming, group-based borrow tracking, and a deeply integrated relationship to the Rust ecosystem, intended for greenfield projects whose authors want stronger safety guarantees than Rust provides while keeping access to the crates Rust users already depend on.

Unpacking:

- **Memory-safe.** Vale enforces memory safety statically via groups + linear types. Vale's safety model is strictly more expressive than Rust's borrow checker for region-style ownership across nested borrows and for linear resources the compiler refuses to drop silently, at the cost of rejecting some patterns Rust accepts (cancellation by drop, fearless `Rc<RefCell<T>>`-style runtime checks).
- **Systems language.** AOT compilation to native via LLVM. No GC, no managed runtime, no JIT. Performance comparable to Rust and C++.
- **First-class compile-time metaprogramming.** Vale's comptime is Zig-style — same expression language at compile and runtime, with a slab-based representation of comptime values. Comptime supports the futamura projection (specialization-of-interpreters via partial evaluation). This is the load-bearing differentiator from Rust's `const fn` story.
- **Group-based borrow tracking.** Vale's groups are a purely compile-time concept (Rust-lifetime-like but more expressive). Allocators are handled Rust/Zig-style as ordinary runtime concerns; arenas + bump allocators are stdlib library types, not part of the type system. Groups erase to `re_erased` at the rustc boundary.
- **Deeply integrated with Rust.** Vale source directly imports and uses Rust crates (`import rust.std.vec.Vec`); Rust source uses Vale-defined items as first-class Rust types; bidirectional. Not "FFI in the `extern "C"` sense" — Vale's typechecker has direct visibility into Rust signatures via rustc's `TyCtxt`, and Vale-defined items appear in rustc's monomorphization graph.
- **Greenfield.** Vale is not a Rust replacement for existing projects to migrate to. The interop is rich, but Vale's safety model rejects patterns Rust accepts; porting non-trivial Rust to Vale will involve real rewrites.
- **Stronger safety than Rust.** Vale's groups give region-style memory safety without per-borrow lifetime annotation burden. Vale's linear types prevent silent drops of resources whose deallocation order matters. Vale's typechecker enforces invariants Rust's typechecker can express only at runtime.

### 1.2 Memory model: groups, linear types, slab-based comptime

**Groups.** A group is a named, possibly hierarchical **set of possibly-aliasing places** — not a duration; safety comes from flow-sensitive invalidation/poisoning rather than borrow scopes (see the Valen language reference's "Groups are not lifetimes"). Valen annotation: `func process<g'>(x: &T in g)`. Groups can nest explicitly (`g in h`, a place-subset relation richer than Rust's outlives). Groups are **purely compile-time** — the typechecker proves validity Vale-side; the resulting reference erases to `re_erased` at the rustc boundary. Vale's group system carries a `dangle` modifier (alongside `imm`, `rc`, and `runtime`) — declarable on any function's group parameter, with `drop`'s `dangle` driving `#[may_dangle]` projection on auto-emitted Drop impls; the typing pass verifies that a `dangle`-claiming body never dereferences through that group. See §11.

**Linear types.** Values that cannot be silently dropped — must be explicitly consumed (returned, passed to a consumer, destructured). Vale's typechecker enforces this at compile time. Per `valen-design-1.md` (Linear types), a struct is linear when it **either defines a `drop`** (auto-run at scope end) **or has consumer functions that take `self` by move, with no `drop`** (linear-strict — must be explicitly consumed on every path). Scope-end drops are synthesized via the `__vale_drop<T>(&local)` AST-rewrite wrapper; at the Rust boundary a linear-strict type additionally gets a synthesized panic+abort Drop shim (Sky §F.22) so a **Rust-side** drop aborts rather than silently skipping the consumption Vale source cannot omit.

**Slab-based comptime.** Vale's comptime evaluator implements Zig-style comptime by simulating a slab — a byte buffer with allocator services that holds comptime-constructed values. Comptime values are referenced by slab address (a `usize` offset) Vale-internally. When crossing into rustc-visible territory (as const generic arguments), values surface as **content-hash u128 constants**, not as slab pointers — Vale adopts Sky §29.A.content-hash-const-args from day 1 to sidestep the dual-Instance/single-symbol conflict that slab-pointer-as-u64 produces.

The slab is per-rustc-invocation. Never serialized. Comptime results that need to persist across invocations are baked into the typed AST in resolved form, not as slab references. Section 13.

### 1.3 Rust ecosystem integration as a first-class concern

Vale is designed from day one to consume Rust crates and be consumable from Rust. Source-level `import rust.X.Y` syntax; Vale's typechecker queries rustc's `TyCtxt` directly for Rust signatures; codegen emits LLVM IR that interoperates at the symbol level with rustc-emitted code; the build system orchestrates cargo as a subprocess (in valec-rs mode).

Pervasive implications:

- Vale cannot define types rustc cannot represent. Cross-boundary types are projected via the `ValeOpaqueType<const T: u128>` wrapper-as-field shape (§10); Vale owns the layout via the `layout_of` query override; rustc sees opaque sized blobs.
- Vale cannot have a calling convention rustc cannot match. Vale's codegen computes the same `FnAbi` rustc does, applying the same coercions. Vale inherits Sky's ABI helpers (`@ACRTFDZ`, `@TCHAPZ`).
- Vale cannot ignore rustc's monomorphization model. Per_instance_mir reports Rust deps Vale transitively reaches; rustc cascades through their transitive Rust dependencies. Vale never reimplements rustc's trait/generic machinery.
- Vale cannot have a lifetime model that surfaces incompatible information. Group erasure to `re_erased` is the boundary mechanism; Vale's typechecker enforces correctness Vale-side; rustc sees post-borrowck-shaped lifetimes.
- Vale cannot have an error-handling model that surfaces incompatible behavior. `panic = "abort"` exclusively (§16). Unwinding across the boundary is forbidden.
- Vale cannot have a drop model that produces silent rustc-visible misbehavior. Linear-type drops panic+abort via user-written Drop bodies; the `__vale_drop<T>` wrapper synthesizes scope-end calls; rustc's standard DropGlue cascades naturally.
- Vale cannot have an async/concurrency model that produces silent misbehavior. Vale futures expose `std::future::Future` impls; Vale's source-level discipline ensures the exposed surface satisfies rustc-required bounds.

The design space is more constrained than a hypothetical "pure" Vale ignoring Rust; the price is judged worthwhile because the Rust ecosystem is enormous and a Vale that couldn't access it would be relegated to research-language status.

### 1.4 Bidirectional interop (7-case taxonomy)

Vale's interop covers all seven cases from Sky's taxonomy (§2 walks each in detail with worked Vale examples). The table:

| Case | Top-level | Middle | Bottom | Pre-pass works? |
|------|-----------|--------|--------|-----------------|
| 1a   | Rust      | —      | Vale (non-generic) | Yes |
| 1b   | Rust      | —      | Vale    | **No** |
| 2    | Vale      | —      | Rust   | Yes |
| 3    | Rust      | Vale   | Rust (same top) | **No** |
| 4    | Vale      | Rust   | Vale (same top) | **No** |
| 5    | Rust      | Vale   | different Rust | **No** |
| 6    | Vale      | Rust   | different Vale | **No** |

Five cases require interleaving (Vale's compiler hooks fire during rustc's monomorphization phase). All seven supported from v1. Closure-extension cases (Vale closures into Rust `Fn`, Vale state machines as `Future` impls, Vale impls of Rust traits with HRTB bounds) addressed in §11 / §14 / §6.6.

### 1.5 Long-term correctness over short-term simplicity

When a design choice trades implementation complexity for long-term correctness or future flexibility, the trade favors long-term correctness. The architect's explicit posture: "no shortcuts; figure out what gets us to the good end state, no matter how long it takes."

Three concrete patterns:

1. **Avoid baking publish-time decisions into shipped artifacts.** Per the cache-not-sidecar decision (§7), nothing pre-baked ships distribution-side except stdlib. Layouts re-derive at consumer compile time. Comptime results recompute deterministically.
2. **Avoid time-saving shortcuts that compromise future architecture.** Single-symbol architecture, marker-based per-crate activation, determinism CI from day 1 — each pays setup cost to keep options open.
3. **Prefer fork patches over fragile plumbing.** Vale accepts a 4-patch rustc fork in valec-rs (§4). Per_instance_mir as a custom query (Instance-keyed) over hacks built atop `optimized_mir`. Codegen as a full plugin via the `fill_extra_modules` allocator-callback hook over partitioner-mutation tricks. Each is more work; each eliminates a fragile mechanism Sky's risks.md documented empirically.

Costs are real; §25 documents them honestly. The posture isn't "ignore costs" — it's "when costs are weeks-to-months rather than days, they don't by themselves disqualify a design that's architecturally cleaner."

### 1.5.5 Non-generic is the degenerate case of generic

A positive design discipline (Sky §1.5.5 / @NNGZ): **Non-generic is the degenerate case of generic. Never branch on `type_params.is_empty()`.** A non-generic item is one with zero type args; it goes through the same instantiation path as a generic one. Code that special-cases non-generic creates false distinctions and latent bugs when items gain type params or get reused more generally.

Concretely:
- Substitution helper with `type_params.is_empty()` early-exit ages badly.
- Discovery channel branching on "is generic" ages badly.
- Symbol mangler with empty-suffix special case ages badly. Write the general path; let N=0 fall out as one iteration with empty args.

Forced exceptions (each `arch-fence-allow:`-annotated):
1. Rust syntax constraints — `impl<>` / `Foo<>` / `Self<>` are parse errors; stub_gen emission skips `<>` for N=0.
2. External rustc behavior with no override — when a query's contract differs for N=0 vs N≥1 in a way Vale can't influence; document and fence.
3. Approach A invariants — `debug_assert!(!instance.args.has_param())` is "substituted vs unsubstituted," not "N=0 vs N>0." Keep.

CI fence: AST-walking architecture-fence test (`vale-frontend/tests/architecture_fence.rs`) parses Vale's frontend source and inspects the syntax tree for `type_params.is_empty()` patterns; unannotated occurrences fail the test. Not grep-based — a proper AST walker via rust-analyzer's syn or a similar parser. **Land in Phase 0**, not later — retrofitting this discipline produces dozens of fence-allow markers that all need re-evaluation.

### 1.6 Nightly rustc forever (valec-rs only)

valec-rs pins a nightly rustc. There is no path to stable rustc for the valec-rs binary. Two unavoidable dependencies:

1. **`#![feature(rustc_private)]`.** valec-rs's `frontend_rust_rustc` crate links against `rustc_driver`, `rustc_middle`, `rustc_codegen_ssa`, `rustc_codegen_llvm`, `rustc_monomorphize`, etc. — all `rustc_private`-gated. rust-lang has no roadmap to stabilize the internal API surface valec-rs uses.
2. **The four fork patches.** valec-rs adds custom queries to rustc; upstream landing is a multi-year arc (§29.6) and not on the critical path.

User-side: valec-rs is installed via a custom rustup channel; `vale-toolchain.toml` pins. From the user's perspective, no different from installing a custom nightly toolchain.

**valec doesn't depend on rustc's internals.** valec is standalone — no rustup needed, no rustc_private, no rustc-internal API surface linked. Its release cadence IS coupled to rustc-nightly's LLVM via the shared libLLVM invariant (§3.6) — both binaries advance together every nightly bump — but valec never links rustc code, only libLLVM. The distinction matters: rustc-nightly-API drift affects valec-rs alone; LLVM-version drift affects both binaries.

### 1.7 What Vale explicitly does NOT do

- **Vale does not unwind.** All Vale-emitted code: `panic = "abort"`. No landing pads, no `catch_unwind`, no panic-as-cancellation. Vale's error model is Result-based; Vale's cancellation model is channel-based (§14-§16).
- **Vale does not implement Rust-style "cancellation by drop."** Dropping an executing Vale future panics + aborts. Linear types may not be dropped at all. tokio APIs depending on drop-as-cancel are incompatible with default linear futures; users opt into cancellable futures via `into_cancellable(future, cleanup_handler)` for tokio compat.
- **Vale does not silently convert Rust types into Vale-shaped views.** Every Rust type Vale source uses must be explicitly imported (Sky `@RTMEIZ`).
- **Vale does not infer generic type arguments at call sites.** Every call to a generic spells out type arguments (or uses a Vale source-level placeholder).
- **Vale does not have Send/Sync as runtime-checkable properties.** Vale's typechecker statically tracks send-ability + sharedness; the runtime carries no marker information. Send/Sync at the rustc boundary are **honest** — no global `unsafe impl Send` lie. This diverges from Sky §12.1; see §12.4.
- **Vale does not allow incoherent trait implementations.** Vale inherits Rust's orphan rule. Sealed Vale interfaces close at the declaration scope (file or project; TBD); only the declaring scope can add impls. Open interfaces follow the orphan rule.
- **Vale does not have separate type universes for runtime and compile-time.** Comptime is Zig-style; one type universe.
- **Vale does not support reflection beyond what comptime can express.** No `typeof`, no runtime type information beyond LLVM debug info, no dynamic dispatch over arbitrary types. v2 may reconsider with `Any`-equivalent if concrete need emerges.
- **Vale does not support unsized generic arguments outside reference patterns.** Sky `@UTAIRZ`.
- **Vale does not surrender LLVM output control to rustc's codegen pipeline.** Vale's C++ Backend owns every byte of Vale-emitted LLVM IR. Non-negotiable for backend pluralism (future GPU/NPU/MLIR-style targets) AND for engineering reuse (Vale's existing C++ Backend is paid-for engineering).
- **Vale does not treat drop as architecturally special.** Drop is just a function the language sometimes auto-calls. `__vale_drop<T>(&local)` wrapper-call AST nodes synthesized at scope ends via Sky §F.22 pattern. The mono path never thinks about drop as special; rustc's standard DropGlue handles trivially-droppable T as no-op and needs-drop T as the full chain, all transparent to Vale.
- **Vale does not have Rust-style macros.** Compile-time code synthesis goes through comptime + reflection (§13.6). No `macro_rules!`, no proc-macros. The `#[derive(...)]` sugar (Q63/Q64) desugars to comptime function calls.

---

## 2. The Architectural Invariant: Interleaved Monomorphization

**valec-rs's compiler must interleave with rustc's monomorphization phase.** A pre-pass design that enumerates Vale's required Rust monomorphizations before rustc starts, or a post-pass design that picks up after rustc finishes, cannot correctly handle Vale's interop cases. The argument inherits Sky's `docs/reasoning/why-interleaved-monomorphization.md` verbatim modulo terminology — Vale takes Sky's place.

### 2.1 The seven-case taxonomy

Consumer language ↔ Rust interop falls into seven architectural shapes varying along three axes: (1) top-level language; (2) middle-layer language; (3) bottom-most callees' language. The table:

| Case | Top-level | Middle | Bottom | Vale-relevant? | Pre-pass works? |
|------|-----------|--------|--------|---------------|-----------------|
| 1a   | Rust      | —      | Vale (non-generic) | Yes | Yes |
| 1b   | Rust      | —      | Vale    | Yes | **No** |
| 2    | Vale      | —      | Rust   | Yes | Yes |
| 3    | Rust      | Vale   | Rust (same top) | Yes | **No** |
| 4    | Vale      | Rust   | Vale (same top) | Yes | **No** |
| 5    | Rust      | Vale   | different Rust | Yes | **No** |
| 6    | Vale      | Rust   | different Vale | Yes | **No** |

Five hard cases require interleaving. Cases 1a and 2 admit pre-pass alternatives but interleaving handles them too. Vale covers all seven.

### 2.2-2.7 Case walkthroughs

Each case has a worked example in Sky §2.2-§2.7 (Vale source + Rust source + the stub-rlib + per_instance_mir mechanism). Direct transcription is unnecessary; the mechanism for every case follows the same pattern:

- Vale source ↔ Rust source flows concrete type arguments in either direction across the boundary.
- The collector queues Instances per the bin's top-level walk; per_instance_mir provides synthetic bodies for Vale Instances; cascade discovery resolves trait-impl methods to Vale or Rust ownership; `fill_extra_modules` emits Vale's real bodies.
- For Case 4 / 6 specifically: cascade discovery fires at the **stub rlib compile**, not at user-bin compile (Sky F.13/F.14 empirical correction). The `is_reachable_non_generic` collector gate blocks user-bin from re-running the cascade for non-generic upstream symbols; the in-process drain (Sky §8.9.5) at the stub-rlib's `consumer_fill_modules` window is what gives the bodies to emit.

### 2.8 The handoff: Vale tells rustc the leaves; rustc walks the rest

**Vale tells rustc the leaves** (concrete Rust items called directly from Vale-defined bodies, or trait dispatches Vale bodies make); **rustc walks the rest** (transitive Rust closures, trait resolution, associated type projection, drop glue cascading, default method instantiation).

Vale does NOT implement Rust's trait resolution machinery. Vale does NOT implement Rust's generic substitution beyond what Vale's own type system needs. Vale projects Vale-defined items onto Rust-shaped surfaces (stub rlibs, layout queries, per_instance_mir bodies) and lets rustc handle Rust's side. This is what makes Vale's interop tractable — the alternative (Vale reimplementing rustc's trait/generic machinery) would be tens of thousands of lines whose every behavior must track rustc's exactly.

### 2.9 Why interleaving is the general-case answer

Vale covers all seven taxonomic cases. The five hard cases require interleaving; cases 1a and 2 admit pre-pass alternatives but interleaving handles them identically. Vale implements only the interleaved mechanism. A consumer architecture that strictly limited itself to cases 1a and 2 could use simpler pre-pass — Vale explicitly doesn't, because Vale's strategic position (memory-safe systems language for greenfield projects leveraging the Rust ecosystem) requires bidirectional interop from day 1.

The cost of supporting the full taxonomy: a custom rustc query (`per_instance_mir`, §19), a codegen-backend plugin (§5), a stub rlib model (§6), and the operational discipline for cross-cutting invariants (§12, §16, §26). Vale pays this cost as a foundational decision.

### 2.10 What "interleaving" means precisely

**Vale's compiler hooks fire during rustc's monomorphization collection phase, supplying per-Instance information about Vale-defined items as the collector encounters concrete Instances of those items.** The collector calls Vale's `per_instance_mir` query when it walks a body referencing a Vale-defined function; Vale's provider returns the body substituted to the concrete Instance's args. The collector calls Vale's `layout_of` query when it needs a Vale type's layout; Vale's provider returns it. Drop glue flows through rustc's standard DropGlue path post-Phase-E; the `__vale_drop<T>` AST-rewrite mechanism is what synthesizes the scope-end calls (§15.7).

Interleaving is **not**:
- Vale running a separate phase before rustc and telling rustc what to compile (pre-pass).
- Vale running a separate phase after rustc and picking up CGUs (post-pass).
- Vale implementing its own collector that walks both Vale and Rust source (reimplementing rustc).

The collector is the driver. Vale is the responder. The collector walks the reachable set; Vale answers questions about Vale-shaped items it encounters.

---

## 3. The Two Binaries: `valec` and `valec-rs`

Vale ships TWO distinct compiler binaries, sharing a single Rust frontend codebase + a single C++ Backend, differentiated by whether the binary bundles a forked rustc internally. This chapter covers the binary split, why it exists, and the per-binary capability boundaries. **This is the principal architectural divergence from Sky/toylang's single-binary model.**

### 3.1 Why two binaries

Three motivations:

1. **Vale's identity as a full language.** Vale stands on its own — users should be able to write and ship Vale code without installing the Rust toolchain. A single ~2GB binary that bundles rustc would signal "Vale is a frontend for rustc"; a small standalone ~40-100MB binary signals "Vale is its own language with an optional Rust-interop big-binary alongside."
2. **Download size.** ~40-100MB for the small binary vs ~2GB for the rustc-bundled one. Most Vale users won't need Rust interop; making them download 2GB to access Vale is wasteful. Users who want Rust interop install the bigger binary.
3. **Forcing function for architectural independence.** Two binaries means Vale's frontend + C++ Backend must work in both modes. We can't accidentally couple Vale to rustc internals at the language level. Vale's design stays portable to non-rustc-mediated codegen paths (e.g., MLIR-based GPU targets in the future).

### 3.2 What's in each binary

**`valec` (~40-100MB):**
- `frontend_rust` Rust library — Vale's parser, name resolver, typechecker, instantiator, comptime evaluator. No `rustc_private`.
- C++ Backend — Vale's existing LLVM-emitting code, statically linked.
- Bundled libLLVM matching rustc's nightly's LLVM version (§3.6 / §5.7).
- `valec` CLI orchestrator (parses `vale.toml`, generates `.vale-build/`, drives codegen).
- No rustc internals. No rustup needed.

**`valec-rs` (~2GB — estimate; includes forked rustc with 4 patches + libLLVM matching rustc's nightly + pre-compiled Vale stdlib per target + valec-rs binary + Vale's frontend_rust_rustc crate. Actual measured size TBD; aspirational until Phase 5's C++ Backend borrowed-mode work produces a first shippable artifact.):**
- All of valec's content (`frontend_rust`, C++ Backend, libLLVM, CLI).
- `frontend_rust_rustc` Rust library — the `rustc_private`-using glue: `LangCallbacks` impl, `per_instance_mir` provider, `layout_of` override, `collect_and_partition_mono_items` filter, `cross_crate_inlinable` override, `deduced_param_attrs` override, `fill_extra_modules` hook installation, stub_gen, cascade discovery, IdI↔DefId bridge.
- Forked rustc internals statically linked (the 4 patches from §4).
- Argv-dispatched: invoked as `valec-rs build`, runs as orchestrator; invoked via `RUSTC_WORKSPACE_WRAPPER` from cargo subprocesses, runs as rustc-wrapper providing Vale's machinery.

Both binaries share `frontend_rust` and the C++ Backend codebases. The split between them is `frontend_rust_rustc` (only in valec-rs) and the bundled rustc internals (only in valec-rs).

### 3.3 Shared codebase, mode-gated items

Most of Vale source compiles in both binaries identically. Where mode-dependent code is needed, the single mechanism (Q51) is **`#[cfg(rust_interop)]` at the item level**: a parse-time binary expression over flag identifiers (Rust-style — `cfg(rust_interop)`, `cfg(not(rust_interop))`, `cfg(all(...))` / `cfg(any(...))` — NOT a comptime function call). It gates `import rust.X.Y` statements, anything referencing Rust types/traits by name, and item-level rust-only definitions. In valec mode, the parser skips `cfg(rust_interop)` items entirely; they don't appear in the typed AST, name resolution, or HinputsT. Each mode produces a different in-memory universe from the same source.

Body variants per mode are expressed by giving the same item two `#[cfg]`-gated definitions with different bodies — e.g. the pure-Vale `String<A>` (under valec, allocator parameter resolved via Vale-native allocator impls) vs the valec-rs `String<A>` (which delegates to Rust stdlib for the appropriate fields/methods per the comptime-conditional-backing pattern in §12.1). Both definitions present the same allocator-generic source-level surface; the `#[cfg]` selects which body lives in this binary. There is no body-level mode test intrinsic. Inside a function body, mode-specific behavior comes from calling an item whose own `#[cfg]`-gated definition does the work in this binary.

### 3.4 What each binary can compile

**`valec`** can compile any Vale source that doesn't use `import rust.X`. Items inside `#[cfg(rust_interop)]` blocks are skipped at parse time. `exported(rust)` annotations are silently ignored (the item compiles but isn't surfaced to anything; per Q18). Vale-only ecosystem code (libraries that don't depend on Rust crates) compiles in valec; libraries depending on Rust deps require valec-rs.

**`valec-rs`** can compile everything `valec` can plus rust-interop items. `import rust.X` resolves via rustc's `module_children`; `exported(rust)` items appear in stub rlib emission; cascade discovery for trait-impl methods fires.

A given `vale.toml` project chooses which binary it builds with (via `vale-toolchain.toml`). Libraries that conservatively want to compile in both modes use `#[cfg(rust_interop)]` discipline to gate any rust-touching items. Vale stdlib uses this discipline pervasively — most items work in both binaries, with dual `#[cfg]`-gated definitions for the small handful of items whose bodies must differ per mode (e.g. `class String`, with separate pure-Vale and rust-wrapping definitions).

**Graduation model: valec → valec-rs is a one-way transition, not a frequent toggle.** The expected user journey is "start with valec; the moment you want any Rust ecosystem dep, graduate to valec-rs and stay." Both binaries share `target/` in the same project; switching the binary you build with invalidates the entire cache (the `BinaryIdentity` axis in the cache-key Merkle digest, §7.3) and forces a full rebuild. This is intentional: cache thrash is the cost of graduation, paid once. Workflows that bounce between binaries on the same project pay full rebuild every swap — by design. Pure-Vale libraries intended for consumption by both binaries' users are still expressible (they ship source, downstream rebuilds per consumer's binary), but the library author themselves picks one binary for their own dev loop.

### 3.5 Toolchain distribution

Two distribution channels (Q31):

**`valec` via `valeup`** — Vale-controlled custom installer. Shell script at `https://vale-lang.org/install` (or equivalent) detects platform, downloads platform-appropriate binary, places in `~/.vale/bin`, modifies PATH, writes shell completions. Mirrors `rustup-init.sh` shape. Per-platform downloads (linux-x86_64, linux-aarch64, macos-x86_64, macos-aarch64, windows-x86_64) from day 1. Standalone — no rustup dependency. ~50-100MB downloads.

**`valec-rs` via rustup integration** — custom rustup distribution server (`rustup self update --download-server https://vale-lang.org/rustup`) OR rustup-toolchain-link for early adoption. Toolchain contains: forked rustc (with patches), `valec-rs` binary, bundled libLLVM matching rustc's LLVM, pre-compiled Vale stdlib per target. Users run `rustup toolchain install vale-rs-nightly`. ~2GB downloads.

**Mutual awareness:** `valeup` detects rustup; offers "also install valec-rs?" on first run. `valec-rs` install detects `valeup`; offers shared-toolchain-version coordination. Both binaries can coexist in PATH; project's `vale-toolchain.toml` decides which version of each runs.

**`vale-toolchain.toml`** at project/workspace root mirrors `rust-toolchain.toml`:
```toml
[toolchain]
channel = "vale-nightly-2026-06-29"
components = ["valec", "valec-rs"]
targets = ["aarch64-apple-darwin"]
```

**Cross-installer alignment mechanism.** valeup and rustup are independent distribution channels — users update each separately. Vale's alignment invariant (§3.6: "both binaries advance together") is enforced by three concrete mechanisms:

- **Runtime libLLVM lookup: RPATH per binary.** Each binary's linkage-time RPATH points to its own bundled libLLVM directory:
  - `valec`: RPATH → `@executable_path/../lib/` (or `$ORIGIN/../lib/`) containing valec's bundled libLLVM.
  - `valec-rs`: RPATH → rustc sysroot's libLLVM (bundled with the forked-rustc-nightly toolchain).
  - Runtime linker resolves via RPATH before searching system paths; each binary loads its own bundled libLLVM regardless of what other Vale/Rust toolchains are also installed.
  - Version-suffixed library filenames (`libLLVM-21.1.8.dylib`, etc.) add a defensive layer — different versions coexist on disk without collision. `DYLD_LIBRARY_PATH` shadowing is possible if a user manually sets it, but that's an intentional override, not the default case.

- **Version alignment check at build init: hard error on drift.** Every `valec build` / `valec test` / `valec check` reads the pinned channel from `vale-toolchain.toml`, detects which binaries the project needs (based on `[rust-dependencies]` presence, `#[cfg(rust_interop)]` items, etc.), and verifies each installed binary reports the same channel version. Mismatch = hard error at build init, before any real work starts. Sample diagnostic:
  ```
  error: vale-toolchain.toml pins channel "vale-nightly-2026-06-29"
         but valec-rs is at "vale-nightly-2026-06-15"
    hint: run `rustup update vale-rs-nightly-2026-06-29`
  ```
  Fail-closed: user can't accidentally build with drifted versions.

- **No auto-install in v1.** Vale's CLI does NOT auto-invoke valeup or rustup to install the correct version — the user takes action per the diagnostic. Rust-style auto-install (as rust-toolchain.toml does via rustup) is deferred; v1 posture is error-heavy, assertion-heavy, matching Vale's "correctness over convenience" stance during development. Softening to opt-in auto-install is a v2 candidate if the friction turns out to be a real user pain point.

### 3.6 LLVM version pinning

Single LLVM version per Vale toolchain release, matching rustc's bundled LLVM (Q66 E reframing).

- valec is **pinned to rustc's LLVM**. Not because valec links rustc, but because Vale's toolchain releases bundle libLLVM as a single artifact for both binaries.
- valec-rs **dynamically links libLLVM from rustc's sysroot**. Two libLLVMs in one process = duplicate-symbol UB; sharing is mandatory.
- valec **dynamically links the SAME bundled libLLVM**. Static-linking would let valec pin independently but creates dual-LLVM portage cost in the C++ Backend (compile against two LLVM versions, ongoing drift). The cost isn't worth Q14's "independent LLVM" stance — valec's standalone-ness is about not bundling rustc, not about independent LLVM versioning.
- **Both binaries advance together every nightly bump.** Toolchain releases bump LLVM in lockstep with rustc.
- **C++ Backend builds against ONE LLVM version per release.** Phase 0 LLVM 16 → ~21 port is single-target.

Per-bump cost: ~1-2 weeks (§4.4 / §25.2) for the focused engineer doing the bump.

---

## 4. The Fork

valec-rs maintains a fork of rustc. The fork is deliberate, not a fallback. valec (the standalone binary) doesn't depend on rustc at all and isn't affected by fork concerns.

### 4.1 Why Vale forks (only for valec-rs)

valec-rs needs a custom rustc query: `per_instance_mir`. Instance-keyed (takes a concrete `Instance<'tcx>`, not a `LocalDefId`), provides a MIR body whose `ReifyFnPointer` casts enumerate Rust deps reachable from that specific Instance's substituted body. No sanctioned rustc extension point delivers this; the query is added as a fork patch.

**Load-bearing reason: per-Instance dep discovery for the interleaved-monomorphization cases (1b, 3, 4, 5, 6 from §2).** For Vale → Rust generic → back into Vale (Case 4), Rust deps Vale transitively reaches depend on concrete generic args at each call site. DefId-keyed dep enumeration would force Vale to over-approximate combinatorially, OR push substitution into rustc's collector via `Param` placeholders — which fails for arbitrary-typed comptime args (rustc's const generics restrict the type universe). Vale's Approach A (Instance-keyed Vale-side substitution) avoids both failure modes.

Arbitrary-typed comptime is a **secondary reinforcement**, not the primary reason. Even without comptime, per-Instance Rust-dep walks would require Instance-keyed substituted bodies.

### 4.2 The four patches

valec-rs's fork is four patches against vanilla nightly rustc, identical in shape to Sky's. ~238 LOC across 8 files. Each patch is small, structurally local, follows established rustc patterns. None modify rustc's behavior for vanilla compiles (default providers preserve pass-through invariant).

**Patch 1: declare the query.** `compiler/rustc_middle/src/query/mod.rs`:
```rust
query per_instance_mir(key: ty::Instance<'tcx>) -> Option<&'tcx mir::Body<'tcx>> {
    desc { "computing per-Instance MIR for {:?}", key }
    cache_on_disk_if { false }
}
```

**Patch 2: collector calls per_instance_mir.** `compiler/rustc_monomorphize/src/collector.rs::collect_items_of_instance`:
```rust
let body = tcx.per_instance_mir(instance)
    .unwrap_or_else(|| tcx.instance_mir(instance.def));
```

**Patch 3: default provider returns None.** `compiler/rustc_mir_transform/src/lib.rs::provide`:
```rust
providers.per_instance_mir = |_tcx, _instance| None;
```

Vanilla rustc behavior unchanged when no plugin installs a real provider.

**Patch 4: `fill_extra_modules` allocator-callback hook (Approach B, rev 3 `#[repr(C)]` shape).** ~210 LOC across `rustc_codegen_ssa::traits::backend`, `rustc_codegen_ssa::traits::mod`, `rustc_codegen_ssa::base`, `rustc_codegen_ssa::back::write`, and `rustc_codegen_llvm::lib`. Adds `ExtraBackendMethods::fill_extra_modules(tcx, allocator)` + `ExtraModuleAllocator<M>` `#[repr(C)]` struct with `state: *mut c_void` + `allocate: unsafe extern "C" fn(...)`. Default-no-op; LLVM backend overrides to consult a process-global `OnceLock<FillExtraModulesHook>`. valec-rs installs the hook during driver setup.

Submission timing: synchronously on main thread inside `codegen_crate`, BEFORE `start_async_codegen` (per Sky §F.4). Extras flow into the standard optimize → ThinLTO-summary → emit pipeline as additional CGUs. Cross-language inlining works because Vale's modules are in the same LTO pool as user-bin's bitcode.

**Patch 5 stays retired.** Vale never adds it. The CGU-placement hazard patch 5 papered over doesn't exist under the partition filter mechanism (§5.3) which removes consumer items from rustc's CGU list entirely.

### 4.3 Long-term: upstream as `adt_const_params`-extension or new query

valec-rs pursues upstream landing of `per_instance_mir` (or equivalent) in parallel with primary implementation. Three candidate upstream paths (Sky §3.3 framing inherited):

1. **`per_instance_mir` as specific query, plugin-overridable.** Smallest upstream surface.
2. **Generalized "plugin-defined substitution semantics" via extension trait.** Lets plugins participate in `ConstKind::Param` substitution for plugin-defined types. Bigger RFC, but the right primitive.
3. **`adt_const_params` extended to allow externally-provided equality/hashing.** Narrowest viable upstream surface for Vale's specific use case. Probably most palatable to upstream reviewers.

Vale's posture: pursue (3) as primary upstream path; (2) as follow-on; (1) as fallback. Upstreaming is multi-year work, **not on Vale's critical path.** The fork is sustainable indefinitely.

### 4.4 Fork maintenance budget

Empirical baseline from Sky's bumps: ~1.5-2 weeks per nightly bump for a focused engineer. Breakdown:
- **Fork rebase**: ~1-2 days. Patches 1-3 are typically clean; patch 4 touches 5 churn-prone files in the codegen stack, may take a half-day during restructuring windows.
- **MIR construction drift**: ~1 week. Vale's per_instance_mir builds synthetic MIR using rustc-internal APIs that drift each release. Per-site cost similar to Sky's; Vale's site count is comparable.
- **ABI helpers drift**: ~1-2 days. Inherits Sky's ABI helpers; same drift surface.
- **Everything else**: ~0.5-1 day. Driver entry, Callbacks trait additions, layout query key shape, providers struct restructuring.

Total: **~1.5-2 weeks per bump.** Real but bounded.

### 4.5 Nightly pin and bump strategy

valec-rs pins a specific nightly via `vale-toolchain.toml` (channel `vale-rs-nightly-<date>`). Bump cadence: ~every 6 months tracking rustc nightly. Don't chase the latest nightly.

Procedure per bump:
1. Decide to bump (calendar trigger or forcing function).
2. Pick target nightly (~3 months old; ecosystem-adjacent projects have reported drift).
3. Snapshot current test suite results.
4. Bump the rustc fork; rebase the four patches.
5. Bump Vale; fix compile errors in dedicated commits (one drift surface per commit, for bisection).
6. Test cold (wipe caches).
7. Test warm.
8. Update documentation with empirical bump-cost data.
9. Cut Vale toolchain release.

Whole process: ~2-3 weeks of focused engineering. Scheduled work, not interleaved with feature development.

---

## 5. The Codegen Backend

Vale's C++ Backend handles LLVM emission in BOTH binaries. valec creates its own LLVMContext; valec-rs uses a borrowed LLVMContext from rustc via the `fill_extra_modules` allocator-callback hook. This chapter covers the borrowed-mode FFI design, the Vale codegen pipeline, and the interaction with rustc's codegen-time mechanisms.

### 5.1 C++ Backend, borrowed-mode FFI

The C++ Backend exposes two FFI entries (Q66):

```cpp
// Owned-mode (valec): C++ Backend creates LLVMContext/Module/TargetMachine
extern "C" int32_t backend_compile_program(
    MetalCacheHandle* cache, ProgramHandle* program,
    int argc, char** argv);

// Borrowed-mode (valec-rs OR valec's own internal call): consumer supplies handles
extern "C" int32_t backend_compile_program_into(
    MetalCacheHandle* cache, ProgramHandle* program,
    void* borrowed_context,        // LLVMContextRef
    void* borrowed_module,         // LLVMModuleRef
    void* borrowed_target_machine, // LLVMTargetMachineRef
    int argc, char** argv);
```

`backend_compile_program` allocates its own handles via `LLVMContextCreate` + `LLVMModuleCreateWithNameInContext` + `LLVMCreateTargetMachine`, then calls `backend_compile_program_into` with them. valec-rs's `consumer_fill_modules` callback (the `fill_extra_modules` hook handler) calls `backend_compile_program_into` directly with handles borrowed from rustc's `ModuleLlvm`. Single core path, two entry points.

**`GlobalState.ownsLlvm: bool`** gates lifecycle: borrowed mode skips `LLVMDispose*` calls; owned mode disposes as today. The flag is set by setup() based on which FFI entry was called.

**DataLayout in borrowed mode** sourced from rustc's module via `LLVMGetModuleDataLayout(borrowed_module)`. Vale's `GlobalState.ptrSize` derives from the borrowed module's data layout, never from a freshly-created TargetMachine. Mismatched DataLayouts inside one module = silent miscompile; this is the single most error-prone piece of borrowed-mode plumbing.

**TargetMachine in borrowed mode** is also borrowed from rustc (option A from Q66). FFI grows to three handles. Vale-side queries that consult TargetMachine (`getSizeOf` for some specific cases) read from the borrowed handle.

**Vale's `optimize()`** at `vale.cpp:1308-1372` runs Vale's own PassBuilder pipeline. **Skipped entirely in borrowed mode.** rustc owns codegen-time optimization via its LTO/opt-pass-manager pipeline AFTER `consumer_fill_modules` returns. Running both = double-opt (wasted work or miscompilation).

**`generateOutput()`** at `vale.cpp:1300-1303` writes a `.o` in owned mode. **Skipped in borrowed mode.** rustc owns codegen output entirely; Vale's bodies in the borrowed module become part of rustc's output via the standard CGU emission pipeline.

**Concurrent CGU semantics in borrowed mode: sequential FFI calls, per-call state.**

Vale contributes multiple CGUs to rustc's codegen. rustc runs one LLVMContext per CGU (per-CGU isolation sidesteps LLVMContext's non-thread-safety); Vale must match this.

**Shape**: within one synchronous `fill_extra_modules` hook window, Vale's `consumer_fill_modules` callback calls `backend_compile_program_into` **N times sequentially**, once per Vale CGU. Each call receives a fresh `(borrowed_context, borrowed_module, borrowed_target_machine)` triple minted by rustc's allocator. Sky's `llvm_gen.rs` state-per-call model is the reference — one FFI per rustc-supplied ModuleLlvm.

Post-hook, rustc's `start_async_codegen` processes each ModuleLlvm on a rayon worker for LLVM optimization + emission. Vale's CGUs ride the same parallelism as rustc's own CGUs; the load balances across worker cores.

**State model:**

- **`GlobalState` (LLVM Type* cache, DataLayout, ptrSize, PassBuilder handles)**: **per-FFI-call**. Fresh instance each `backend_compile_program_into`. Cross-context Type* reuse is silent LLVM UB. Vale's current single-invocation `GlobalState` (persists for the whole valec run) gets refactored to per-call — see §28 Phase 0 tasks. Directional bonus: aligns with a broader Vale-project goal of making `GlobalState` less global over time.
- **`MetalCache` (Vale-source-level type/method resolution)**: **shared across FFI calls within one valec-rs invocation**. LLVMContext-independent — holds ValeTypeId, method dispatch tables, and other Vale-source-level resolution, not LLVM handles. FFI calls are sequential within the synchronous `fill_extra_modules` hook, so no concurrent-access race — one thread, sequential calls, shared read-mostly cache.
- **Vale CGU partitioning**: Vale controls its own partition strategy. v1: simple partitioning (one CGU per Vale library or per top-level Vale module) for symmetry with rustc's default partitioner. Refinement based on measured LLVM optimization times deferred.

**Invariant: no cross-call Type* aliasing.** `GlobalState.type_cache` from call N MUST NOT be referenced during call N+1. Enforced structurally by instantiating `GlobalState` fresh per call — no state escapes an FFI-call boundary.

**Detection**: fixture that emits 3+ Vale CGUs in a single build, asserts each produces a distinct valid `.o` and the resulting binary links cleanly. Post-LTO output verified byte-for-byte identical whether the CGUs are emitted in the order Vale computed or reversed — LTO output order shouldn't depend on emission order.

### 5.2 No B2 risk: Vale controls emission via the partition filter

valec-rs's plugin never mutates `MonoItemData.linkage` post-partition. The mechanism Sky risks.md §B2 documents (linkage-mutation timing assumption) doesn't apply.

valec-rs uses the partition filter pattern: `collect_and_partition_mono_items` query override that delegates to the default partitioner, then rebuilds each CGU with consumer items removed. Consumer items never reach rustc's LLVM-codegen path. Vale's `fill_extra_modules` contribution emits the sole bodies with External linkage; rustc emits no competing `.o` symbols.

**Single-symbol architecture (Sky §6.2):** Vale emits each rustc-visible body under the **same rustc-mangled name rustc's default v0 mangler would give the stub fn**. The `symbol_name` query override (Sky's pre-Phase-F shape) retires; Vale never adds it. To compute the rustc-mangled name from Vale's side, call `tcx.symbol_name(instance)` directly. Single def at link time; no IR-linker tie-break race; cross-language inlining works through the LTO IR linker pool.

### 5.3 Suppressing rustc's `.o` emission for Vale items

**The filter predicate (Phase C/D of Sky, adopted by Vale):**

```rust
pub fn is_consumer_codegen_target<'tcx>(tcx: TyCtxt<'tcx>, def_id: DefId) -> bool {
    is_from_vale_stubs(tcx, def_id)
        && tcx.has_attrs_with_path(def_id, &[
            Symbol::intern("vale"),
            Symbol::intern("emit_consumer_body"),
        ])
}
```

`vale-stub-gen` emits `#![register_tool(vale)]` at the stub source crate root and decorates each Category B item with `#[vale::emit_consumer_body]`. The two-gate conjunction handles both stubs (marker-bearing crate + emit_consumer_body attribute) cleanly.

**Category A vs B:**

| Category | Examples | Tagged? | Filter behavior | Codegen source |
|---|---|---|---|---|
| A: real Rust bodies | marker const, ValeOpaqueType wrapper, Vale struct decls, `pub use` re-exports, `extern "C"` decls, Phase-6 `__vale_*_unwrap` helpers | No | survives filter | rustc's normal codegen |
| B: `unreachable!()` placeholders | exported Vale fns + `__vale_main`, accessor methods on Vale types, trait-impl methods on Vale types (incl. `<ValeType as Drop>::drop`) | **Yes** | removed | Vale's `fill_extra_modules` under rustc-mangled name |

**1:1 invariant:** every tagged item ↔ exactly one Vale emission per concrete Instance the cascade reaches. CI fence asserts this both ways (`emit_consumer_body_tags_only_category_b_items`).

**Same predicate used by partition filter AND per_instance_mir override** — they must agree on what counts as consumer-owned. CI fence keeps them in lockstep.

### 5.4 LlvmCodegenBackend delegation for Rust items

In **valec-rs**, Vale's `ValeCodegenBackend` wraps `LlvmCodegenBackend` and delegates every Rust-shaped operation. The `provide()` method installs Vale's query overrides: `per_instance_mir`, `layout_of`, `collect_and_partition_mono_items` (partition filter), `cross_crate_inlinable` + `extern_queries.cross_crate_inlinable`, `deduced_param_attrs`. Marker-gated — Vale's overrides install only when `__VALE_STUBS_MARKER` is present in the local crate.

In **valec**, there is no LlvmCodegenBackend to wrap. The C++ Backend IS the codegen path. valec's CLI orchestrator parses `vale.toml`, drives Vale's frontend, invokes the C++ Backend via `backend_compile_program`, and the resulting `.o` is linked via system clang/ld (Sky-style; long-term Vale forks rustc's linking code per Q66 H, but that's deferred).

### 5.5 `.o` emission point

Sky's §5.5 Step-2 split applies to Vale (valec-rs mode):

- **Each Vale library's compile** produces (a) the stub rlib's `.o` carrying rustc-compiled Rust-side machinery (Phase-6 wrappers, cascade-surfaced Rust generic intermediaries), (b) Vale's `fill_extra_modules` contribution with real External-linkage bodies for the **non-generic** Vale items defined in this library + cascade-discovered trait-impl method bodies whose impl lives in this library, (c) the local cache (`.vale-cache` per §7).
- **The final binary's compile** produces the binary's `.o`, carrying (a) the bin's Rust-emitted code, (b) Vale bodies for the binary's own non-generic items, (c) Vale bodies for **generic** monomorphizations reached transitively (substituted Instances materialize only when concrete args are supplied downstream).

**Trade-off:** cross-library Vale-body inlining at `lto = false` (cargo dev profile default) is LOST for Vale-top cases. Non-generic Vale bodies live in upstream rlib's invocation; cross-crate visibility requires `lto = "thin"` or `"fat"`. Documented user-facing perf-recommendation: `[profile.release] lto = "thin"`.

In **valec mode**, the binary's compile is a single invocation that emits everything. Cross-crate split doesn't apply because there's only one cargo-invoked rustc-equivalent in valec mode (valec's CLI orchestrator is its own beast).

### 5.6 Symbol audit: `__vale_` prefix discipline

Vale's C++ Backend currently emits global symbols including `main`, `__main_argc_argv`, `__expandWrcTable`, `__checkWrc`, `__WRCTable`, `__LgtTable`, `__UniversalRefCompressed`. In valec-rs mode, rustc's libstd shim also defines `main` — direct collision at link time.

**Phase 0 task:** prefix every Vale runtime global with `__vale_`:
- `main` → `__vale_main` (mandatory; collides with libstd's `main`)
- `__main_argc_argv` → `__vale_main_argc_argv` (wasi-libc symbol)
- `__expandWrcTable` → `__vale_expand_wrc_table`
- `__checkWrc` → `__vale_check_wrc`
- `__WRCTable` → `__vale_wrc_table`
- `__LgtTable` → `__vale_lgt_table`
- `__UniversalRefCompressed` → `__vale_universal_ref_compressed`
- (etc. — full audit list before any valec-rs work begins)

The `main` rename is mandatory; the others are defensive. Rust source dependency on libstd's `main` startup shim means Vale's `main` must rename. Other prefixes prevent future collisions.

**In valec-rs mode**, stub_gen synthesizes a thin `fn main() { unsafe { __vale_main() } }` shim in `.vale-build/<crate>/src/main.rs`. The path: libc `_start` → libc init → rustc libstd's startup shim → Rust `fn main()` → `unsafe { __vale_main() }` → Vale's real entry.

**In valec mode**, the C++ Backend emits a thin `main(argc, argv)` shim in LLVM IR alongside `__vale_main`:

```llvm
define i32 @main(i32 %argc, i8** %argv) {
    ; translate argc/argv into Vale-idiomatic args per func main()'s signature
    call void @__vale_main(...)
    ret i32 0
}
```

libc's standard `_start → init → main` startup path runs; `main` calls `__vale_main`. Standard libc facilities (malloc, environ, atexit) remain available. Mode gating uses `GlobalState.ownsLlvm` (owned mode = valec = emit the shim; borrowed = valec-rs = skip, rustc-provided shim covers it).

**argc/argv handoff**: the shim knows Vale's `func main()` signature from Vale's type system. `func main()` (no args) → shim discards argc/argv and calls `__vale_main()`. `func main(args: [Str])` (or similar) → shim translates argc/argv into a Vale-idiomatic args array via Vale runtime allocator, then calls `__vale_main(args)`. Translation code is Vale-Backend-internal; not user-visible.

**libstd collision framing**: the collision only exists in valec-rs mode where rustc's libstd defines `main`. In valec mode there's no Rust libstd linked (no rustc, no Rust libstd), so no collision — Vale's C++-Backend-emitted `main` shim uses the `main` name freely. The universal rename to `__vale_main` gives Vale a clean namespace in both modes; in valec mode, `main` is available for Vale's own shim.

**Not option (c) — no linker-script-set entry.** Setting `__vale_main` as the direct entry (via `-e __vale_main` or equivalent linker flag) would bypass libc init, which Vale's stdlib depends on (malloc, printf, environ setup, etc.). The shim approach preserves standard libc startup semantics.

### 5.7 LLVM version pinning across binaries

Per Q66 E reframing: **single LLVM version per Vale toolchain release.** Both binaries advance together every nightly bump. C++ Backend builds against ONE LLVM version per release. Initial Phase 0 task: port C++ Backend from LLVM 16 to whatever rustc's pinned-nightly bundles (currently LLVM ~21 as of mid-2026).

**Linkage:** both binaries dynamically link libLLVM. valec-rs against rustc's sysroot libLLVM; valec against an identical libLLVM bundled with the toolchain. Two static libLLVMs in one process = duplicate-symbol UB; dyn-linking is mandatory in valec-rs and adopted in valec for consistency + cross-binary cache compatibility.

**Per-bump cost** includes a C++-side LLVM-version porting subtask. The PassBuilder + InlinerPass adapter shape at `vale.cpp:1308-1372` is the most version-volatile surface; expect periodic non-trivial restructuring as LLVM majors evolve.

---

## 6. The Stub Rlib Model

Vale-defined items are projected onto Rust-shaped surfaces for rustc to typecheck and (via per_instance_mir) for rustc's mono collector to walk. The mechanism is the stub rlib: a generated Rust crate containing Rust-source declarations of every exported Vale item, with `unreachable!()` bodies that rustc compiles normally but Vale's `collect_and_partition_mono_items` filter removes before LLVM emission. **Only valec-rs generates stub rlibs**; valec mode doesn't involve rustc.

### 6.1 Per-Vale-project stub rlib

Each Vale project compiles to its own stub rlib (Sky's multi-rlib model inherited). Project `VmdSiteGen` depending on `VmdParse` produces:
- `vmdparse.rlib` — stub declarations for VmdParse's exports
- `vmdsitegen.rlib` — stub declarations for VmdSiteGen's exports

Naming: stub rlibs named directly after the Vale project (`vmdparse.rlib`, not `vmdparse_stubs.rlib`). The "stubs" qualification is internal to `vale-stub-gen`'s bookkeeping.

**Per-project (not single combined rlib)** because cargo's incremental compilation requires per-crate granularity. Single-rlib alternative forces full recompile on every change. Per-project lets cargo cache per library, invalidate selectively, parallelize.

### 6.2 Exported items only in the stub rlib

Q18 lock: items are `exported(c)`, `exported(rust)`, `exported(c, rust)`, or unexported. The stub rlib contains declarations only for items where the export target list includes `rust` (so `exported(rust)` + `exported(c, rust)`). Pure `exported(c)` items have no stub rlib entry. Non-exported items have no stub rlib entry. Per Sky §9.4: non-export Vale items have **no rustc DefId** — they exist only in Vale's universe and in the binary's final `.o`, completely invisible to rustc.

`vale-stub-gen`, when generating each project's stub rlib source, walks Vale's HinputsT items and emits Rust declarations for items where `rust` is in the export target list. Other items are skipped.

For an exported Vale function:
```vale
exported(rust) func wrap<T>(x: T) -> Wrapper<T> {
    Wrapper { inner: x }
}
```

Generates in the stub rlib's `lib.rs`:
```rust
#![feature(register_tool)]
#![register_tool(vale)]
#![feature(rustc_private, fn_traits, unboxed_closures, dropck_eyepatch)]

pub const __VALE_STUBS_MARKER: () = ();

pub struct Wrapper<T>(ValeOpaqueType<HASH_FOR_WRAPPER>, ::std::marker::PhantomData<T>);

#[vale::emit_consumer_body]
pub fn wrap<T>(x: T) -> Wrapper<T> {
    ::std::unreachable!()
}
```

For an exported Vale trait impl on a Vale type:
```rust
impl ::std::clone::Clone for Widget {
    #[vale::emit_consumer_body]
    fn clone(&self) -> Widget {
        ::std::unreachable!()
    }
}
```

Method body is `unreachable!()` and tagged with `#[vale::emit_consumer_body]`. The partition filter (§5.3) removes the placeholder so it never reaches LLVM. Vale's `fill_extra_modules` emits the real body under the same rustc-mangled symbol name. Single-symbol architecture per §5.2.

### 6.3 `__VALE_STUBS_MARKER` for activation

Every generated stub rlib carries:
```rust
pub const __VALE_STUBS_MARKER: () = ();
```

valec-rs's `rustc`-wrapper-mode detects this marker at startup (after argv parsing, after `Callbacks::after_expansion`). Marker present → Vale's machinery activates for this crate compile (query overrides install, frontend processes `.vale` source). Marker absent → Vale's machinery stays dormant; the compile proceeds vanilla. Byte-identical pass-through for pure-Rust crates (Sky §4.4 / §25.3.5 invariant).

Detection mechanism uses `tcx.module_children_local(CRATE_DEF_ID)` + DefId-parentage check on the symbol (Sky's empirical correction — glob re-exports across Vale deps can otherwise re-export the marker into downstream crates and falsely flag them). Cached per `CrateNum`.

### 6.4 `vale-stub-gen` owns the entire `lib.rs`

Q17 lock: **Vale-only projects.** No `.rs` files inside Vale projects. The stub rlib's `lib.rs` is entirely `vale-stub-gen`-emitted. Users never edit the generated Rust source.

The `lib.rs` is gitignored; can be regenerated from scratch at any time. Deterministic emission (Sky §6.4 / §18.5 invariant) — same `vale.toml` + same `.vale` files = byte-identical generated source.

User-written Rust escape hatch: separate Rust crates consumed via `[rust-dependencies]` in `vale.toml`. The Rust crate is a regular Cargo crate; it has no Vale machinery; Vale source consumes it via `import rust.shim_crate.X`.

`vale-stub-gen` emission contents:
- Marker const
- Crate-level attributes (`#![feature(...)]` list including `rustc_private`, `fn_traits`, `unboxed_closures`, `dropck_eyepatch`, `register_tool`)
- `#![register_tool(vale)]`
- `pub struct` declarations for exported Vale types (ValeOpaqueType-wrapped per §10.6)
- `pub fn` declarations for exported Vale functions (with `unreachable!()` bodies + `#[vale::emit_consumer_body]`)
- `impl` blocks for exported Vale trait impls (cases A/B/C from Q16)
- `pub use rust::std::vec::Vec` re-exports for `import rust.X` in Vale source
- Phase-6 wrappers (§6.6.5)
- Drop impl shims for Vale types with user-written `impl Drop` (§15.7)
- Closure-lifted struct types + Fn/FnMut/FnOnce impls (§14)
- Async state machine struct types + Future impls (§14)

### 6.5 Stub rlib carries the Vale project's name directly

`vmdparse.rlib`, not `vmdparse_stubs.rlib`. Rust callers depending on a Vale lib write `use vmdparse::Foo` naturally — no `_stubs` suffix. The `is_from_vale_stubs(tcx, def_id)` predicate uses marker-item detection (§6.3), not crate-name pattern matching.

### 6.6 Cross-rlib orphan rule (matches Rust's)

Vale inherits Rust's orphan rule. An impl can exist only in the crate owning either the trait or the type (Q46 locked).

For Vale's sealed interfaces (default per Q15): only the declaring file/project (TBD) can add impls. Sealed = closed-world at declaration scope.

For open Vale interfaces (`open` keyword): cross-project impls follow the orphan rule normally — downstream crate can `impl OpenInterface for NewType` when downstream owns NewType.

Stdlib interfaces (Hashable, Eq, Ord, Display, Debug, etc.) ship as `open` because users will want to impl them on their own types. Sealed reserved for closed-world ADT-like uses (`sealed interface AstNode`, etc.).

Five idioms supporting Path-1 (matching Rust's orphan rule exactly):
1. Newtype with cheap delegation (Vale's macro/derive system makes this one-liner)
2. Extension trait pattern
3. Top-level binary's stub rlib counts as local
4. Vale's typechecker emits orphan-rule error in Vale terms (don't surface rustc errors)
5. `#[fundamental]` analog for Vale's `&T in g`-style references

**Sealed interfaces exported(rust): emitted as enum + sealed trait.**

Vale sealed interfaces (Q15) can be `exported(rust)` without losing their sealed semantics at the Rust boundary. Rust's orphan rule normally lets a downstream Rust crate add `impl SealedInterface for RustType` freely (rustc has no concept of "sealed"), which would let foreign Rust impls slip past Vale's typechecker and break Vale exhaustive-match assumptions. Vale-stub-gen closes this by emitting the sealed interface as **two Rust items**:

1. **Enum** matching Vale's sealed variants — for Rust pattern matching, exhaustive `match` support, no dispatch cost. Primary consumer surface; matches Vale's sealed semantics naturally.
2. **Trait with a private sealed supertrait** — for dyn dispatch and generic bounds. Only Vale can add impls (private supertrait unreachable from downstream Rust). Uses Rust's well-established sealed-trait idiom (also used in Rust stdlib for `Iterator::size_hint` extensions and in serde for `Deserializer`).

Concrete example: Vale sealed interface `AstNode` with impls `ExprNode`, `StmtNode`, `DeclNode`:

```rust
mod __vale_sealed {
    pub trait Sealed_AstNode {}
}

// Enum — primary form, matches Vale sealed semantics
pub enum AstNode {
    Expr(ExprNode),
    Stmt(StmtNode),
    Decl(DeclNode),
}

// Trait — for dyn dispatch and generic bounds; name suffixed to disambiguate from the enum
pub trait AstNodeTrait: __vale_sealed::Sealed_AstNode {
    // ... Vale interface methods projected to Rust ...
}

// Enum impls the trait; dispatches by variant
impl __vale_sealed::Sealed_AstNode for AstNode {}
impl AstNodeTrait for AstNode {
    // match self { AstNode::Expr(e) => e.method(), ... }
}

// Each variant type also impls the trait directly
impl __vale_sealed::Sealed_AstNode for ExprNode {}
impl AstNodeTrait for ExprNode { /* ... */ }
// (analogously for StmtNode, DeclNode)
```

Rust downstream trying to add `impl AstNodeTrait for RustType` fails at Rust compile time — can't impl `Sealed_AstNode` because the `__vale_sealed` module is private and unreachable. Rustc rejects with a clear error at the downstream `impl` site.

**Naming.** The enum takes the Vale interface's name (`AstNode`). The trait suffix (`Trait`, `Iface`, etc.) is an implementation-time bikeshed to disambiguate the trait from the enum in Rust's shared type/trait namespace. Doc pins the mechanism; specific suffix chosen at implementation time.

**Consequences:**
- Vale-side sealed semantics preserved — exhaustive matches sound; Vale receives from Rust only Vale-emitted impls (via the enum's variants).
- Rust callers get both enum-matching (idiomatic for closed variant sets) and trait-object ergonomics.
- Zero runtime overhead compared to unsealed alternatives; no discriminant checks, no vtable bloat beyond the trait's normal vtable.

**Direction: Rust → Vale (which form does Vale receive?).** Vale receives sealed interface values from Rust as the **enum form** — Vale-source pattern-match syntax works uniformly on the enum discriminant whether the value came from Vale or from Rust. The trait-object form (`&dyn AstNodeTrait`) is an alternate projection for cases where Rust wants dyn dispatch through Vale-defined APIs; Vale-source can't pattern-match on `&dyn AstNodeTrait` directly (no accessible discriminant) — it calls trait methods, which dispatch through the vtable to variant-specific impls. Standard Rust dyn semantics; Vale conforms. **Default projection for sealed interface parameters is the enum form**; Vale API design should prefer the enum unless dyn dispatch is specifically needed.

**File-vs-project sealed-closure-scope decision** (still TBD per §29.8) is orthogonal — that governs Vale-source-side impl-addition discipline. The stub-gen emission works with either scope.

### 6.6.5 Phase-6 generic wrappers in the stub rlib

Inherited from Sky §6.6.5. Some Rust items can't be called directly through normal extern declarations because of `#[inline(never)]` instability, `#[track_caller]` semantics, or symbol-presence depending on whether other Rust code happened to call them. Canonical example: `Option::unwrap`.

Vale stdlib's stub rlib emits `#[inline(never)]` generic wrapper functions:

```rust
#[inline(never)]
pub unsafe fn __vale_option_unwrap<T>(o: *mut ::std::option::Option<T>) -> T {
    ::std::ptr::read(o).unwrap()
}

#[inline(never)]
pub unsafe fn __vale_result_unwrap<T, E: ::std::fmt::Debug>(r: *mut ::std::result::Result<T, E>) -> T {
    ::std::ptr::read(r).unwrap()
}
```

Vale source's `option.unwrap()` desugared by the frontend to `__vale_option_unwrap<T>(ptr_to_option)`. The wrapper is generic; rustc instantiates per concrete T. `#[inline(never)]` on the wrapper keeps the symbol stable; the inner `.unwrap()` inlines normally; `#[track_caller]` falls out for free.

Linkage discipline: these are NOT consumer items (no `#[vale::emit_consumer_body]` tag). They survive the partition filter and emit via rustc's normal codegen. Default `Hidden` linkage suffices because Vale's emitted code calling them is in the same final binary at link time.

v1 ships wrappers for `Option::unwrap`, `Result::unwrap`, `Option::expect`, `Result::expect`. Vale stdlib team maintains the list.

### 6.7 Vale source file ships alongside

Every published Vale library ships `.vale` source files alongside the generated artifacts. Source ships per Sky §6.7:
- User inspection (security review, understanding what a lib does)
- Source-level debugging (DWARF references `.vale` source lines)
- IDE / tooling (rust-analyzer or future vale-analyzer reads source on hover)
- v1 has no closed-source Vale libraries; v2 may add them as opt-in distribution

Cargo package layout:
```
my_utils/
  Cargo.toml                # vale-stub-gen-generated
  build.rs                  # toolchain check (Vale's analog of Sky's §21.3)
  src/
    lib.rs                  # vale-stub-gen-generated Rust stub source
    lib.vale                # author's Vale source (shipped verbatim)
    [other .vale files]
  vale.toml                 # author-written
  README.md, LICENSE        # author-provided
```

No `.vale-meta` sidecar (§7 — Vale doesn't ship sidecars; cache is local-only).

---

## 7. The Cache (No Sidecar)

Vale does NOT ship pre-built sidecars distribution-side. Following toylang's 2026-06-29 migration, Vale's analog of the typing-pass output lives in a local on-disk cache at `target/<triple>/<profile>/deps/lib<crate>-<hash>.vale-cache`, populated at upstream's own compile, consumed by downstream compiles within the same target directory. This chapter covers the cache format, key derivation, and lifecycle.

### 7.1 Location and naming

Cache file is a sibling of each cargo-built `.rlib`/`.rmeta`:

```
target/<triple>/<profile>/deps/
  libvmdparse-abc123.rlib
  libvmdparse-abc123.rmeta
  libvmdparse-abc123.vale-cache
```

`<triple>` and `<profile>` are cargo's standard target-and-profile directory structure; `abc123` is cargo's content-hash filename suffix that already invalidates per (features, target, profile, dep graph). The `.vale-cache` extension piggybacks on cargo's invalidation — when cargo decides this `.rlib` is stale, the sibling `.vale-cache` is implicitly stale too.

### 7.2 Header format

```
offset  size   field
------  ----   -----
  0      4     magic "VALC" (0x56414C43)
  4      4     cache_format_version (u32 LE)
  8     16     cache_key_digest (BLAKE3-truncated Merkle, 16 bytes)
 24      8     payload_offset (u64 LE) = 64
 32      8     payload_length (u64 LE)
 40      8     payload_checksum (BLAKE3-trunc to 8 bytes)
 48     16     reserved (zeroed)
 64      N     payload (bincode-encoded HinputsT serialization)
```

Fixed-size header, trivially decodable. Payload at 64-byte-aligned offset. `cache_format_version` bumps when payload schema changes; mismatch = hard error.

### 7.3 Cache key inputs (7-axis Merkle digest)

`cache_key_digest` (bytes 8-23 of the header) is BLAKE3-truncated-to-16-bytes over the canonical encoding of 7 inputs:

| # | Axis | Input shape |
|---|------|-------------|
| 1 | BinaryIdentity | `(BinaryKind, [u8; 32])` — which Vale binary (`valec` or `valec-rs`) + binary content hash |
| 2 | FormatVersion | `u32` |
| 3 | LocalSourceHashes | `Vec<(String, [u8; 32])>` — per-file path + content hash, sorted |
| 4 | UpstreamCacheDigests | `Vec<(String, [u8; 16])>` — per-upstream-crate name + cache_key_digest, sorted (transitive Merkle) |
| 5 | TargetTriple | `String` |
| 6 | ValeTomlHash | `[u8; 32]` |
| 7 | AnnotationFileHashes | `Vec<(String, [u8; 32])>` — per-annotation-file path + content hash, sorted |

Implemented as flat `CacheKeyAxis` enum + parallel `CacheKeyInputs` struct (toylang's shape, Q2 of follow-up). Multi-component axes (BinaryIdentity) supported cleanly.

CACHE_KEY_AXES single-source-of-truth contract enforced by `EXPECTED_AXIS_COUNT` constant + meta-test (`cache_key_axes_and_build_rs_lines_are_in_sync`): adding an axis without bumping the constant + updating both the digest fn and the build.rs rerun-line emission fails CI loudly.

Dropped from Sky's original 9-axis list (per toylang's round-2 validation): `cargo_lock_hash` (cargo encodes it) and consumer-resolved features (consumer can't reconstruct).

### 7.4 Eager producer-side write

Cache is written at the **upstream's own compile**, not lazily at downstream consume time:

- During upstream's compile, at the `after_rust_analysis` callback (post-typecheck, pre-codegen), Vale's frontend has the full HinputsT in memory. Serialize + write to `target/<triple>/<profile>/deps/lib<crate>-<hash>.vale-cache`.
- At downstream consume time, load the upstream's cache file from the same target dir. No re-running of upstream's frontend.
- **Downstream cache miss = hard error** (§7.7).

**Why eager-producer-side, not lazy-consumer-side:**
- @GCMLZ (generate-compile-mutex-lock): lazy consumer-side population would re-enter consumer state during codegen-time queries, reintroducing the deadlock vector Vale's mutex hierarchy is designed to avoid.
- Determinism: eager write means the cache is produced once per upstream compile, deterministically. Lazy would produce caches per consumer compile, with potentially different orderings.
- Cargo's build graph already serializes upstream compiles before downstream compiles. Eager-producer-side fits naturally.

**@CMWAR (Cache-Must-Write-At-Rust-analysis):** all cache writes route through `after_rust_analysis` only. Never write from inside codegen-time callbacks (`consumer_fill_modules`, `per_instance_mir`-provider invocations, etc.) even if superficially convenient. Toylang's empirical history (two-write-sites cleanup) validates this as a real deadlock-prevention discipline.

### 7.5 Transitive Merkle fingerprinting (Option 1)

UpstreamCacheDigests (axis 4) is a transitive Merkle: each upstream crate's cache_key_digest is computed first, then contributes to downstream's cache_key_digest. Cargo-style fingerprinting.

Implication: a change anywhere in the transitive dep graph cascades — library_c's source edit invalidates library_c's cache, which invalidates library_b's cache (whose key included library_c's digest), which invalidates the binary's cache. Conservative but obviously correct.

Trade-off: whitespace/comment edits in upstream cascade invalidations. Toylang has no empirical data on the over-invalidation rate (Q3 of follow-up); Vale collects data first.

**Option 2 (verify-on-load with content-addressed cross-crate refs) reserved as v2 escape** if hit rate becomes a measurable concern. `CACHE_KEY_AXES` structured so migrating from Option 1 to Option 2 doesn't require redesigning the key — Option 2 adds a verification pass at load time without changing the key structure.

### 7.6 Determinism

CI fence: byte-identical cache output across two clean builds in isolated target dirs. Required for:
- Cargo's per-crate fingerprinting to work correctly
- Reproducible builds (Q28 / §27)
- Transitive Merkle correctness (downstream's UpstreamCacheDigests axis depends on upstream cache files being deterministic)

Implementation: `fence_cache_determinism.rs` builds a corpus of small Vale projects twice into per-run target dirs, byte-compares the resulting `.vale-cache` files. Mismatch blocks toolchain release.

Determinism requires:
- Vale's typing pass produces deterministic HinputsT (no HashMap iteration order in serialized content; sorted iteration where collections are involved)
- bincode serialization is deterministic (it is)
- No timestamps, no host-system-dependent content, no random IDs in the payload

### 7.7 Hard-error policy

Missing cache when downstream tries to load = hard error:

```
error: Vale cache missing for crate `vmdparse`
  expected at: target/aarch64-apple-darwin/debug/deps/libvmdparse-abc123.vale-cache
  marker present in rlib: yes
  hint: rebuild `vmdparse` to populate the cache; this can happen when
        the cache file was deleted manually or by `cargo clean -p vmdparse`
        without rebuilding
```

Format-version mismatch = hard error:
```
error: Vale cache `vmdparse.vale-cache` is format version 5; this valec
       supports format version 7
  hint: rebuild `vmdparse` with a matching Vale toolchain version
```

Hard errors over fallback because: an rlib with the marker but no/wrong cache means Vale's machinery was supposed to be active during the rlib's compile but wasn't (or the cache was deleted/corrupted). Vale cannot type-check the lib's exported items, can't know its types' layouts. Falling back to "treat as normal Rust lib" is wrong because the rlib's `unreachable!()` bodies would propagate to runtime panics.

### 7.8 Stdlib distribution is the exception

Vale stdlib is the sole pre-distributed Vale binary. Stdlib's cache files ship with the toolchain (analogous to rustup's pre-built sysroot stdlib). Two precompiled stdlib artifacts per target: one for valec, one for valec-rs. Distribution via valeup (for valec stdlib) and rustup (for valec-rs stdlib).

Stdlib's special status:
- Built once per (target, mode) by the Vale toolchain release process
- Cache files shipped pre-built; consumer's first build doesn't re-run stdlib frontend
- Single trust boundary (Vale toolchain releases are signed; supply chain trust is implicit)
- Stdlib version pinned to compiler version (Q22 lock)

Everything else (user libraries) is source-only distribution. No `.vale-cache` ships in Vale library cargo packages. Each downstream compile re-runs upstream lib frontend on first build (cache miss); cargo's per-target-directory cache amortizes within a user's iterative dev workflow.

User-libs source-only model matches Rust's user-crate distribution exactly. See §21 for distribution detail.

---

## 8. HinputsT (in-memory; no distribution format)

HinputsT is Vale's typing-pass output — the in-memory typed AST. Full structural detail lives in `typing-pass-design-v3.md` and `instantiator-design.md`; this chapter recaps the Rust-interop-relevant pieces only. The serialization format (when persisted to `.vale-cache`) is mechanical bincode-over-HinputsT; format never crosses machine/version boundaries (local cache only per §7).

### 8.1 Vale's HinputsT structure (interop-relevant)

- **Types** (StructDefinitionT, InterfaceDefinitionT): nominal structures with name, type parameters, field names + types, group parameters, linearity status, source position.
- **Functions** (FunctionDefinitionT): name, type parameters, parameter names + types, return type, group parameters, typed body, source position. Generic functions stay templated (not pre-monomorphized); `KindPlaceholderT` first-class.
- **Impl blocks** (EdgeT, InterfaceEdgeBlueprintT): trait DefId ↔ concrete impl bodies. Materialize at typing-pass exit.
- **Modules**: nested namespaces; cross-package refs via `IdT.package_coord`.
- **Exports/externs**: KindExportT/FunctionExportT/KindExternT/FunctionExternT — carry C-extern symbol names where applicable.

Identity model: pointer-equality via `std::ptr::eq` on arena-allocated `&'t` refs; MustIntern seal prevents construction outside the interner.

### 8.2 Cross-crate item references

Vale's HinputsT encodes intra-module references via `IdT.package_coord`. For cross-crate references (in valec-rs mode, where Vale code references Rust items via `import rust.X` and other Vale projects via `[vale-dependencies]`):

```rust
enum ItemRef {
    Internal(ValeItemId),
    RustPath(RustAbsolutePath),  // "::std::vec::Vec"
    ValePath(ValeAbsolutePath),  // "vmdparse::AST"
}
```

Cross-crate resolution happens at upstream-cache-load time: when the consumer loads vmdparse's cache, references become first-class objects in Vale's in-memory universe with concrete DefIds (for items reaching rustc) or ValeItemIds (for items staying Vale-internal).

### 8.3 Rust call encoding (RustCall AST node)

Vale source `vec.push(x)` produces a typed AST node:
```
RustCall {
    target: RustRef("Vec::<T>::push"),
    args: [SelfArg, x],
    return_type: Unit,
    group_effects: { mutates: G1 },
}
```

The instantiator translates this through Q62's per-Instance partial-evaluation; the resulting per_instance_mir body emits ReifyFnPointer casts of the target's substituted DefId. How the *type* on the receiver (`Vec<i64>`) and the `RustRef` target are named inside Vale's own name IR — as a Vale-owned stable-identity name, never a rustc type inline — is §8.10.

### 8.4 Rust trait impl markers

Vale source `impl rust.std.clone.Clone for MyType` produces:
```
RustTraitImpl {
    rust_trait_path: "std::clone::Clone",
    trait_args: [],
    self_type: ValeTypeRef("MyType"),
    method_bodies: [(method_name: "clone", body: typed_expr)],
}
```

Processed at stub-gen time: emits `impl ::std::clone::Clone for MyType { ... }` block with `unreachable!()` bodies; Vale's per_instance_mir provides real bodies at codegen.

### 8.5 Typeid table for ValeOpaqueType wrapper

u128 content-addressed typeids per §10.8. Each typeid maps a Vale type identity to:
```
ValeTypeId {
    typeid: u128,            // BLAKE3-truncated hash of canonical recipe
    source_identity: ValePath,
    layout: Layout { size, align },
    drop_glue_symbol: Symbol,
}
```

Universe-level collision detection on insertion: identical content → fine; different content → build fails with explicit error including both types' source paths/recipes.

### 8.6 Item bodies: typed AST shipped for all items

Vale's in-memory universe contains typed AST for every item — exports and non-exports. Same shape as Sky's locked decision (§8.6): downstream codegens everything from Vale source via partial-eval at per_instance_mir time. No pre-compiled bodies ship.

### 8.7 Source position info

Every item carries source position (file, line, column). File table maps indices to filenames relative to cargo package root. Enables Vale-source diagnostics, cross-crate jump-to-definition, DWARF that references `.vale` source.

### 8.8 No pre-computed layouts

Layouts derive at consumer compile time from structural information. Computed lazily at `layout_of` query fires; memoized per `(typeid, args)` within one rustc invocation. Matches Sky §8.8 reasoning: Vale-version independence, comptime-driven layouts work naturally, layout flexibility for future compiler improvements.

### 8.9 Discovered trait-impl instances (in-process drain)

Sky §8.9.5 inherited. Cascade discovery for case 4/6 trait-impl methods fires at the **stub rlib compile**, not at user-bin compile. The `is_reachable_non_generic` collector gate blocks user-bin from re-running it.

Mechanism: at the stub rlib's `consumer_fill_modules` callback (post-mono-walk; @GCMLZ-safe), `collect_consumer_trait_impl_instances(tcx) -> Vec<DiscoveredTraitImplInstance>` walks rustc's partition for `MonoItem::Fn(instance)` entries matching `is_consumer_trait_impl_method`. The same callback drains the Vec inline, looks up the impl across loaded universes, substitutes the impl-method body with captured args, emits via Vale's standard codegen pipeline.

No sidecar shipment, no cross-process state, no `on_sky_lib_loaded`-style cross-crate state injection. Pure in-process Vec, microsecond lifetime.

`DiscoveredTraitImplInstance` shape: `{self_type_name, trait_name, method_name, concrete_args}`. Sorted by stable key (`(self_type_name, mangled(concrete_args), trait_name, method_name)`) before drain for emission-order determinism per §7.6.

### 8.10 Representing Rust items in the typing-pass name IR (Option A)

How does the typing pass name a Rust type inside Vale's own IR — e.g. `import rust.std.vec.Vec`, then `my_vec = Vec<i64>()`, then `my_vec.push(x)`? This section locks the representation.

**Decision — Option A (stable-identity name), ratified 2026-07-24.** A Rust item is a **new first-class Vale name carrying Vale-owned stable identity only** — never a rustc type inline. Chosen over Option B (store `(DefId, GenericArgsRef<'tcx>)` directly on the name, cfg-gated, with the interner's arena bound to `'tcx`) after a design pass plus three independent adversarial reviews. The deciding factors: (1) Option A keeps rustc types out of the frontend's **core IR** (`names`/`types`/`interner`) entirely, where Option B would embed `Ty`/`DefId` under `#[cfg]` in the three most central files — preserving the §3.2 fence (which, under the single-crate-cfg decision, is enforced by confining rustc code to a `#[cfg(rust_interop)]` oracle submodule plus a green `cfg`-off build, not a physical crate wall; see the oracle-seam note below); (2) Option B is **strictly more representation, not less** — it still needs Option A's entire `RustAbsolutePath` + `DefId↔ValeItemId` bridge + typeid-recipe serialization layer (rustc pointers can never hit disk, §7.6), *plus* the inline form, *plus* a lower/re-inflate round-trip at every cache boundary; and (3) Option B's one genuine advantage (rustc facts inline on the receiver, no resolution hop) is recoverable *inside* Option A via a per-invocation pointer-keyed memo (below), while its costs — deleting the fence, spreading an asymmetric `#[cfg]` surface and nightly-rustc-API drift across the three most central frontend files, and forfeiting a single testable universe — are structural and unrecoverable. All three reviews returned Option A (confidence ~0.80–0.85).

**The representation (name-property design).** A Rust item **reuses the existing kinds** — there is *no* new `KindT` arm and *no* new name type. "Rust-backed" is a property of the name's reserved **`rust` `package_coord`** (`PackageCoordinate{ module:"rust", packages:["std","vec"] }`; §28.1 already reserves `rust`), and the *kind* is chosen by the Rust item's kind at import time:

- Rust **`struct`** → `KindT::Struct(StructTT)` holding an ordinary `StructNameT`, `package_coord == rust`.
- Rust **`enum`** → `KindT::Interface(InterfaceTT)` — a closed sum type *is* a Vale closed trait, and Vale's inline closed traits lower to enums, so a Rust enum shares `InterfaceTT` with Vale's own closed traits (unified, not fragmented into a bespoke kind).
- Rust **`trait`** → `KindT::Interface(InterfaceTT)` (open interface).
- Rust **`union`** → deferred (opaque/struct-like edge case).

`StructNameT`/`InterfaceNameT` are pure-identity (`{ template, template_args }`, templates just `{ human_name }` — verified), so a Rust item carries its whole identity with only the reserved package: the module path rides `IdT.package_coord`, `human_name` is `"Vec"`, and the generic args are Vale's **existing** `ITemplataT` (`Vec<i64>` → `[Kind(KindT::Int(I64))]`; `Vec<SomeValeStruct>` → `[Kind(KindT::Struct(..))]`; `[T;4]` → `[Integer(4)]`). Because a Rust struct genuinely *is* a struct-kind and a Rust enum *is* a closed-interface-kind, this is **not a masquerade** — it is the correct mapping. The core IR (`names`/`types`/`interner`) never names a rustc type and needs no `#![feature(rustc_private)]`.

**Why not a first-class `KindT::RustCitizen` kind** *(earlier draft, superseded)*. A dedicated `KindT::RustCitizen` arm (+ a `RustCitizenNameT` in the `ICitizenNameT` family) was considered and dropped: reusing `KindT::Struct`/`KindT::Interface` (a) eliminates the ~51-file `KindT`-arm blast radius entirely, (b) reuses the existing name types, interner wrappers, and family arms **unchanged** (no new family/interner code at all), and (c) *unifies* Rust enums with Vale closed traits under one `InterfaceTT` instead of fragmenting them. So there is **no family/interner placement to do** — a `rust`-packaged `StructNameT`/`InterfaceNameT` is an ordinary member of the existing families, interned by the existing `intern_struct_name`/`intern_interface_name`, with `MustIntern` canonicalization and pointer-identity comparison verbatim.

**The seam is per-question — no fabricated definitions.** A Rust item has no Vale `StructDefinitionT`/`InterfaceDefinitionT`; the oracle answers *specific questions* at the site that asks them, never via a synthesized definition. The load-bearing seams: **calls** enter as a fourth **candidate source** in `overload_resolver.rs::get_candidate_banners`, alongside the calling env, the param envs, and the placeholder extra-call envs — so a Rust callee competes with same-named Vale functions through `params_match` and `narrow_down_callable_overloads` like any other candidate, rather than being caught after resolution fails. Two triggers, because a Rust callee arrives two ways: **receiver-keyed** for a UFCS method (`my_vec.push(x)` → `param_filters[0]` is Rust-backed → `oracle.resolve_method`), and **name-keyed** for a free function (`add_two_numbers(3, 4)` → no argument is Rust-backed at all, so the name is the only signal → `oracle.resolve_function`, with scoping delegated to the oracle). Either way the result is a call *prototype*, never a definition. **`pub` field access** goes through the `KindT::Struct` arm of the `Dot` handler → `oracle.field(id, name)` → the field's Vale-lowered type; `pub` only, private → clear error — Vale is an *external consumer*, so only a Rust type's *private* internals are opaque to it, not its `pub` fields. `lookup_struct`/`lookup_interface(rust_id)` is never reached (nothing asks a Rust type for a Vale definition body); a Rust enum's variants come from `oracle.variants` (future). Each capability is one small `#[cfg(rust_interop)]`-gated hook at its own site; full frontend plan (module layout, the exact per-site edits, the HUMAN/CLAUDE split, and the requirement that nothing references the interop module under `valec`): `rust-interop-frontend-plan.md`. The complete enumeration of the ~30 remaining call-out sites — layout, drop, conformance, generics, sharedness, weakability, sealedness — with the exact Vale function each would call from, is `rust-interop-callout-map.md`.

**Why a candidate source and not a fallback** *(learned in implementation, 2026-07-25)*. Two reasons, both structural. First, a `find_function`-failure fallback is **unreachable** for a Rust receiver: resolution panics earlier, in `get_param_environments` → `get_outer_env_for_type`, because a Rust-backed citizen has no Vale environment. Second, a fallback would make a Rust callee invisible whenever any Vale function of the same name matched loosely — an overload-semantics decision made by accident. Entering as a candidate also means the synthesized prototype flows through `attempt_candidate_banner`'s existing `PrototypeTemplata` arm, which already does `IFunctionNameT::try_from(..).parameters()` + `params_match` — i.e. the machinery a Rust callee needs was already there for function bounds.

**The oracle seam.** All rustc *facts* (kind, `fn_sig`, layout, variance, auto-traits) come lazily through a **`RustOracle`** with Vale-owned inputs/outputs — **no `'tcx` in any signature** — keyed by the stable path; never stored in `HinputsT`. Per §8.2's corrected model the seam is **bidirectional**: Vale→rustc queries return owned facts, and where Vale must feed rustc (`Instance::expect_resolve`, `GenericArgs::for_item`) it constructs interned `Ty<'tcx>` behind the seam and never lets it escape. The pass reaches the oracle as a `#[cfg(rust_interop)]` field on **`Compiler`** — the immutable-context struct, alongside `&'ctx ScoutArena` / `&'ctx TypingInterner` / `&'ctx Keywords` / `&'ctx TypingPassOptions` — supplied as a cfg'd constructor param threaded from `TypingPassCompilation::new`. Deliberately *not* on `CompilerOutputs`: that is the output accumulator drained into `HinputsT`, and an oracle is an input. `Compiler` is a stack local created inside the pass entry and dropped when it returns, so the containment property holds; `HinputsT` never holds it (verified structurally — `HinputsT` is built field-by-field from `coutputs` getters, so nothing can leak in by accident). Tests reach it through `typing_pass_compilation_for_test`, which supplies a `StubOracle`, so no test about Vale semantics mentions the build mode. Per the **single-crate-cfg decision**, the real `TyCtxt`-backed impl lives behind a `#[cfg(rust_interop)]` submodule of the frontend, **not** a separate `frontend_rust_rustc` crate — so the fence protecting "the core IR never names a rustc type" is a green `cfg(rust_interop)`-off build + confining rustc code to that submodule, rather than a physical crate wall. Either way, Option A does not *shrink* the `'tcx`-touching glue (`per_instance_mir`, `layout_of`, cascade discovery are still separate `'tcx` sites); it *fences* it, keeping the typing-pass core IR `'tcx`-free with no new types at all.

**Typechecking flow.** `import rust.std.vec.Vec` → `oracle.resolve_path` (walks `module_children`, honoring re-exports so `std::vec::Vec` and `alloc::vec::Vec` canonicalize to one identity, avoiding the §6.3 name-match fragility) + `oracle.kind` (→ `Struct`) → intern a `rust`-packaged `StructTemplateNameT`/`StructNameT`. `Vec<i64>()` interns the `StructNameT` and builds `KindT::Struct(StructTT)`; the kind then flows through the solver by pointer identity like any Vale citizen. `my_vec.push(x)` resolves via the call seam (the candidate source in `get_candidate_banners`): `is_rust_backed(param_filters[0])` → `oracle.resolve_method(rust.std.vec.Vec, "push")` → `oracle.fn_sig(item, args, interner) -> ValeSig`; behind the seam this reads `tcx.fn_sig` (an `EarlyBinder<PolyFnSig>`), **instantiates with the concrete args `[i64]` FIRST, then lowers to Vale-owned form** (the @EarlyBinder discipline — lowering pre-instantiation would poison the §19.5 typed-body cache with a wrong substitution), and records a `RustCall` node (§8.3) → `ReifyFnPointer` cast at mono (§19.4), dispatching via the trait def's method DefId (@TVIMDGAZ §26.13). The `ValeSig` is over `KindT`, not `CoordT` — the onion refactor dissolved `CoordT` into the reference wraps inside `KindT`, so a Rust `&self` receiver arrives already wrapped as a `BorrowRef`.

The synthesized prototype's name must carry the params, because `PrototypeT::param_types()` reconstructs them from `id.local_name` rather than storing them — a prototype whose name disagreed with its signature would silently report wrong param types at every call site. `ExternFunctionNameT` is the variant used, matching what the C-extern path already produces for a function defined elsewhere with no Vale body. Two consequences to know: `IFunctionNameT::template()` panics for that variant, and `template_args()` returns `&[]`, so a Rust method cannot yet carry generic args of its own — only those already on the receiver kind.

**Cache / cross-run / build.** The name serializes as `ItemRef::RustPath` (path + Vale args) — deterministic, pointer-free, `'tcx`-free (§7.6); cross-run identity re-resolves the path to a fresh `DefId` at universe-load (§10.9), routing around `DefId`'s session-scoping exactly as rustc's own `DefPathHash` does. **Revised on the cfg question (2026-07-25).** An earlier draft of this section claimed the typing pass compiles in both binaries with ~zero `#[cfg]`. That is achievable — an always-compiled trait plus a no-op `StubOracle` needs no gating — but it was **deliberately not taken**, because it means the core calls into the interop module in the standalone build. The chosen requirement is stronger: *under `valec`, nothing may reference the interop module at all.* So the module tree, the `Compiler` field and its threading, and every seam hook are `#[cfg(rust_interop)]`-gated, and the standalone build is byte-identical to no-interop. The residual is a handful of gated one-liners in the core files (inert under `valec`); literally-zero interop text is not achievable for control-flow interception without a global hook table, which @NGSAX forbids. Enforcement is the green cfg-off build. In the test tree the gating is one line — the whole `typing/test/rust_interop/` subtree — so tests about Vale semantics never mention the mode. Concurrency: the typing pass (hence the oracle read path) is single-threaded (§13.11); the immutable name IR is trivially concurrent-read for the rayon providers, which re-query `tcx` directly (@GCMLZ §26.2; cache writes stay in `after_rust_analysis`, @CMWAR §26.17).

**Cost, and two honest costs the reviews sharpened.** Hot path: once interned, repeated use is pointer-identity comparison with no oracle call; a per-invocation **pointer-keyed memo** (`*const StructTT`/`*const InterfaceTT → (RustItemId, ValeSig)`) behind the seam makes repeated `fn_sig` lookups O(1) — this is where Option B's inline-facts speed is recovered without inlining rustc types. Cold path: the first touch of an item is a memoized rustc query. Two costs the reviews sharpened:

1. **Comparison is O(1) but not a single pointer compare.** `IdT::eq`/`hash` (`names.rs:120-142`) compare `package_coord`/`init_steps` by pointer but then compare `local_name` via a *derived* `PartialEq` on `INameT`, which follows the `&'t` ref and recurses into `template_args` **contents**. So a Rust item compares O(1) at the `IdT`-slice level but content-recursive through the name — **identical to Vale's own citizen names**, and marginally *heavier* than Option B's `List`-by-pointer arg compare would have been. Not a differentiator, but the earlier "O(1) pointer identity everywhere" framing was imprecise.
2. **The Vale arg list is lossy → outbound `GenericArgs` reconstruction.** rustc's true args for `Vec<i64>` are `[i64, Global]` (type **+ allocator**) plus lifetime slots on other types; the Vale name stores only `[Kind(i64)]`. So feeding rustc back (for `Instance::resolve`/`fn_sig`) requires reconstructing the full `GenericArgs` via `generics_of` + `mk_args` + `re_erased` — a real bug surface. It is bounded: the reconstruction lives behind the oracle seam where `'tcx` already lives, it is memoizable per `(path, args)`, and Option B would have to write the identical `mk_args` walk at every cache load anyway. This is Option A's sharpest genuine weakness.

The remaining cost is the **definition-lookup audit**: the `lookup_struct`/`lookup_interface` + method-resolution sites that assume a Vale definition body must route Rust-backed ids to the oracle — a *well-defined seam* (a handful of functions), not scattered arms. HRTBs and complex trait bounds that cannot be losslessly lowered fall back to annotation files (§24) — the model is *complete for identity*, not for full Rust type expressiveness. The @EarlyBinder discipline needs a regression fixture, not just a comment. Rust *method* identity is deliberately not interned in the minimal core (`push` is an oracle-resolved `RustCall`, not a first-class name); a first-class Rust-method name is deferred to when methods need caching/diagnostics/fn-value passing.

---

## 9. Export and Visibility

Vale source uses `exported(target)` annotations to mark items visible across FFI surfaces (Q18). Most of Vale's surface stays invisible to rustc; only items where `rust` is in the export target list cross into the rustc-visible boundary.

### 9.1 The exported(target) keyword

```vale
exported(c) func foo(...)            // only C-extern symbol
exported(rust) func bar(...)         // only Rust stub-rlib entry
exported(c, rust) func baz(...)      // both
func priv_helper(...)                // Vale-private; invisible to both
```

No raw `exported`. The target list is mandatory.

Semantic rule (Q18 follow-up): the export rule is uniform across targets. What differs is the **type universe** each surface can express. `exported(c, rust)` requires the intersection of C and Rust type universes. `exported(c)` requires signatures expressible in C; `exported(rust)` requires signatures expressible in Rust (more permissive than C — closures, trait objects, etc.).

### 9.2 Per-item granularity

Per-item annotation. No `exported mod foo` bulk-export. Clarity over convenience; matches Vale's underlying coherence machinery.

### 9.3 What rustc sees of exports vs non-exports

**For an `exported(rust)` or `exported(c, rust)` Vale item:**
- Rustc has a DefId in the stub rlib's crate.
- Rust callers can name it via absolute path.
- Vale's `per_instance_mir` and `layout_of` overrides fire when rustc queries.
- Rustc's default v0 mangler determines the symbol name (single-symbol architecture; §5.2).

**For non-exported (or `exported(c)`-only) Vale items:**
- Rustc has **no DefId**. The item doesn't exist in rustc's universe.
- Vale's typing pass produces an entry in HinputsT for the item.
- Vale's codegen emits the body via `fill_extra_modules`.
- Rust code cannot reference the item by name.

### 9.4 Non-export items: invisible to rustc at every level

Architecturally **imperative**: rustc never sees non-exported Vale items. This is what makes Vale's surface to rustc proportional to Vale's chosen export surface, not to Vale's total type universe. A Vale library with 100 non-exports and 5 `exported(rust)` items surfaces 5 items to rustc.

Mechanism: `vale-stub-gen` skips non-exported items at stub source generation. They don't appear in `lib.rs`. Vale's per_instance_mir fires only for export items (because only exports have DefIds). Vale's codegen at binary compile time walks Vale's universe (loaded from caches + binary's own HinputsT) and codegens every Vale item reachable from the binary's entry points — exports AND non-exports — via Vale's internal walk, NOT via rustc's mono collector for non-exports.

### 9.5 Transitive Rust deps surface through nearest exported ancestor

When a non-export Vale item transitively calls Rust items, those calls must surface to rustc somehow — rustc must monomorphize the Rust items even though the call graph passes through Vale-internal territory.

Mechanism: synthetic MIR body Vale's `per_instance_mir` provides for an exported item enumerates **all transitive Rust dependencies** — including ones reached through non-export Vale callees. Vale's frontend walks the call graph from each exported item; produces a per-Instance body with ReifyFnPointer casts for every Rust dep reached transitively.

Example:
```vale
func deep_helper<T>(x: T) -> Vec<T> {
    v = Vec::new<T, Global>()
    v.push(x)
    v
}

exported(rust) func make_container<T>(x: T) -> Vec<T> {
    deep_helper<T>(x)
}
```

`make_container<i32>`'s per_instance_mir body contains ReifyFnPointer casts for `Vec::new<i32, Global>` and `Vec::push<i32>` (reached via `deep_helper`). Rustc cascades through. `deep_helper` never gets a DefId; its Rust deps surface through `make_container`'s body.

Memoized per `(exported_def_id, concrete_args)` within one rustc invocation.

### 9.6 No cross-crate Vale-internal symbol resolution problem

A common worry: how do cross-crate calls to non-exports resolve at link time?

**For Vale, this problem doesn't exist.** All Vale-emitted bodies — exports and non-exports — use the rustc-mangled name rustc would have given the stub fn (single-symbol architecture per §5.2). Every reference resolves to the same mangled name regardless of call-site location. Linker sees normal cross-crate symbol resolution.

Vale's internal "non-exported" items use Vale-internal mangling for items rustc never sees. Cross-crate Vale-internal references go through the same single-symbol mechanism: the body emits under a deterministic name; the call site emits a reference to the same name; linker resolves.

### 9.7 Closures and async lift to named types in the source's stub rlib

Closures in `vmdparse/src/foo.vale` lift to named struct types like `__vale_closure_42` in `vmdparse.rlib`'s stub source. `Fn`/`FnMut`/`FnOnce` impls live alongside (§14). Owns-the-type-where-the-impl-lives: orphan rule satisfied. Similarly `async fn` desugars to named state machine types in the source's containing stub rlib; `Future` impls alongside. See §14.

---

## 10. Type Representation Across the Boundary

Vale-defined types are represented in Rust-visible territory via opacity: Vale owns layouts; rustc sees opaque sized blobs. Vale's `layout_of` override reports size + alignment; rustc never inspects fields.

### 10.1 Vale types as opaque stubs in the rlib

For each exported Vale struct, `vale-stub-gen` emits the wrapper-as-field shape:

```rust
// Non-generic:
pub struct Widget(ValeOpaqueType<HASH_FOR_WIDGET>);

// Generic:
pub struct Wrapper<T>(ValeOpaqueType<HASH_FOR_WRAPPER>, ::std::marker::PhantomData<T>);

// Group-parametric:
pub struct Region<'a>(ValeOpaqueType<HASH_FOR_REGION>, ::std::marker::PhantomData<&'a ()>);
```

Each Vale struct keeps its own rustc DefId — it isn't collapsed to `ValeOpaqueType<HASH>` itself. The DefId is what trait impl blocks attach to, what `tcx.item_name` returns for diagnostics, what cross-crate identity hangs on. The `ValeOpaqueType<HASH>` wrapper is the field-level opacity carrier.

### 10.2 PhantomData<T> wrapping for generic Vale types

PhantomData satisfies rustc's "all generics must be used" rule + communicates variance to rustc. Variance form (`PhantomData<T>` vs `PhantomData<*mut T>` vs `PhantomData<fn(T) -> T>`) selected based on Vale's actual variance for the type parameter; Vale's typechecker validates the variance choice is correct.

### 10.3 Layout authority: Vale decides via `layout_of` override

For every Vale type with a DefId, Vale's `layout_of` override fires when rustc queries layout. Returns `LayoutData` constructed by Vale's layout machinery: size + alignment + `BackendRepr::Memory { sized: true }` + zero visible fields. Vale's codegen knows the type's internal structure; rustc doesn't.

### 10.4 Opaque-with-size shape

Returned `LayoutData` properties:
- `fields: FieldsShape::Arbitrary { offsets: [], memory_index: [] }` — wrapper-as-field shape extends to: 1 source field (non-generic) or 2 source fields (generic) per §10.4.5.
- `backend_repr: BackendRepr::Memory { sized: true }` — opaque memory blob, allocated in memory rather than registers.
- `size`, `align` — Vale-computed.
- `uninhabited: false` — Vale types inhabited by default.

### 10.4.5 Debuginfo walker compatibility via wrapper-as-field

Rustc's debuginfo emitter (`build_struct_type_di_node` / `build_union_type_di_node`) iterates source-level `FieldDef`s and queries `layout.field(cx, i)` per source field. Sky's empirical finding (§10.4.5): under "opaque with zero source fields" shape, this walker ICEs when the Vale ADT appears inside a Rust generic (e.g., `Vec<MyValeType>`).

Wrapper-as-field shape resolves it structurally — `pub struct Foo(ValeOpaqueType<HASH>)` has 1 source field; layout reports 1 field; offsets match. Same for generic case (2 source fields, both ZSTs at the layout level).

**No fork patch needed.** Sky retired its briefly-shipped debuginfo-clamp fork patch once wrapper-as-field landed; Vale starts post-retirement.

### 10.5 Layouts computed at per_instance_mir / layout_of time

Layout computation is lazy at query time; memoized within one rustc invocation per `(typeid, args)`. For pre-computable layouts (Vale types whose layout doesn't depend on comptime), Vale's typing pass populates the cache during `after_rust_analysis`. For comptime-dependent layouts, evaluation happens at query time via partial-evaluation engine (§13.7). Cache makes both equally fast on subsequent queries.

### 10.6 `ValeOpaqueType<const T: u128>` universal wrapper

Vale stdlib pre-declares:
```rust
pub struct ValeOpaqueType<const T: u128>(
    ::std::marker::PhantomData<*mut ()>,
    ::std::marker::PhantomPinned,
);
```

**Fail-closed auto-trait markers.** `PhantomData<*mut ()>` is `!Send + !Sync` (raw-pointer semantics via `PhantomData`); `PhantomPinned` is `!Unpin`. Both markers are ZSTs — layout is unaffected (Vale owns `layout_of` for its types anyway). Every Vale-defined stub struct wraps `ValeOpaqueType` (§10.1), so by rustc's auto-trait field-walk, every Vale-defined type is `!Send + !Sync + !Unpin` by default. Any claim to the contrary must be an explicit `unsafe impl` emitted by stub_gen, backed by Vale's real analysis per @HBAB (§26.20). This is the load-bearing correctness property: auto-traits work by rustc-side auto-derive that omission fails to disclaim (omitting `impl Send` doesn't produce `!Send`; it just lets rustc auto-derive from the field), so the wrapper must contain fields that propagate the negative claim; explicit positive emissions then represent Vale's verified analysis. §12.1 (Send), §12.1 (Sync), §12.3 (Unpin/Movable) enumerate the emission rules per auto-trait.

**u128 from day 1** (Q19 reconsideration). Vale doesn't carry the u64-collision-risk era. Universe table maintains `HashMap<u128, ValeTypeInfo>` with collision detection on insertion: identical content → fine; different content → hard error with both types' source paths.

BLAKE3-truncated-to-u128 hashing.

### 10.7 When the wrapper applies

Three cases:

**Case 1: Exported Vale type inside a Rust generic** (e.g., `Vec<Widget>` where `Widget` is exported(rust)). Widget has its own DefId. Used via wrapper-as-field internally; from Rust's view Widget is a normal generic-arg-able type.

**Case 2: Non-export Vale type inside a Rust generic.** Wrapper applies as a substitute identity. Vale's frontend rewrites `MyValeInternalType` to `ValeOpaqueType<typeid_for_MyValeInternalType>` when generating Rust-visible call signatures. Rustc sees `Vec<ValeOpaqueType<typeid>>`.

**Case 3: Comptime-produced type inside a Rust generic.** Wrapper applies. Typeid = content-hash of the comptime construction recipe (§10.8). Rustc sees `Vec<ValeOpaqueType<typeid>>`; Vale's `layout_of` override dereferences typeid → universe lookup → recipe → evaluate → compute layout.

### 10.8 Content-addressed typeids (BLAKE3 truncated to u128) for cross-crate stability

Typeids deterministic from source. Same Vale lib + same Vale version → same typeids. Different libs that define structurally similar but separately-source-located types → different typeids (no collisions). Comptime-produced types with same canonical recipe → same typeid.

Source-defined types: `typeid = BLAKE3(qualified_path)` truncated to u128.

Comptime-produced types: `typeid = BLAKE3(canonical_construction_recipe)` truncated to u128. The recipe deterministically encodes the comptime call graph that produced the type — function DefId, args, recursively canonicalized.

Cross-crate stability: lib_a and the binary's compile compute the same typeid for the same logical type because both have the same source + Vale version + canonical recipe.

### 10.9 Type identity in Vale's universe vs in rustc

Vale-side identity (ValeTypeId or qualified path) and rustc-side identity (DefId) for the same logical type are different things mapped via the typeid table. Built at universe load time: walk `module_children(crate_root)`, compute each item's qualified path, look up Vale item by path, build `HashMap<DefId, ValeItemId>` + inverse. Subsequent queries O(1).

For ValeOpaqueType wrapper: given an instantiation `ValeOpaqueType<typeid>`, the typeid is looked up in the universe's typeid table to recover the Vale type. Entries added during comptime evaluation for comptime-produced types.

---

## 11. Group System and the Boundary

Vale's group system is purely compile-time (Q19 followup). Allocators are handled Rust/Zig-style as runtime concerns; arenas are stdlib library types, not part of the type system. Groups erase to `re_erased` at the rustc boundary (Sky §11.2 @ELASZ inherited).

**Locked direction: group borrowing** (built on Nick Smith's public write-up at `/Volumes/V/VerdagonSite/src/grimoire/group-borrowing.vmd`), with Vale-specific interpretations layered on top — see §11.11, §12.5–12.7. Groups are formed at local variables; mutation of a group invalidates references into its **child groups** (collections, Variants, Box contents — anything independently destructible) but NOT references to the group itself. Function signatures use path annotations like `e.rings*` to declare which child groups a callee touches; the compiler propagates at call sites. Mutual-isolation rule among items in a group. Vale's interpretations add: cross-thread safety semantics derived from mut-effect tracking (no mut effect on group ⇒ shareable across threads; mut effect ⇒ single-thread-visible); no `Cell` or `RefCell` in Vale stdlib (`Mutex` / atomics / channels are the sole synchronized-mutation escape hatch); standard `&T` / `&mut T` projection at the Rust boundary with no intermediary wrapper. Nick has a successor iteration in progress that's not yet public; Vale isn't gated on it and isn't tracking it — Vale iterates independently. This replaces the older region-based model that the rest of §11's framing inherits from.

**Implementation status — load-bearing TBD.** Group-borrowing is the locked architectural direction but is **completely unimplemented**; ground-breaking begins in the coming months. The current Vale typing pass has mostly stripped the older region machinery. Nick's published write-up is explicitly a draft, with a successor iteration in progress that's not yet public. §11's chapter commits to the trajectory, but the analyzer that makes group-borrowing's safety guarantees real is unbuilt. Sections §11.1–§11.9 below reflect interop-relevant invariants (boundary erasure to `re_erased`, single-mut at the boundary, HRTB handling) that survive the model shift; the *analyzer* that enforces them is unbuilt, but the *surface syntax* is now settled (note below). §11.10 (dangle annotation) and §11.11 (Rust → Vale reference imports) describe algorithms we're committing to, conditional on group-borrowing landing correctly per the design.

**Canonical-syntax note (ratified 2026-07-24).** Valen's reference-surface syntax was ratified (`LangNotesValen/Valen/todo/proposal-canonical-syntax-delta-2026-07-18.md`), and the Valen-source examples in this chapter have been migrated to it. This doc had used a two-generations-old Sky region spelling (`&G T`, group letter before the type); the canonical forms are now:

- **`&Foo in g`** is a borrow — `&` is the borrow sigil; `&Foo in g mut` is a mut borrow. Mutability rides the *group* (effect clauses), not the reference.
- **Bare `Foo`** is the primary hold — an owned value at struct kind; a strong claim (storage) or anchored borrow (parameter) at class kind (design-2). A struct borrow *receiver* is therefore `&self` / `&self mut`, not bare `self`.
- **Group parameters tick at their declaration only**: `<g': Foo>` (typed) / `<g'>` (untyped); every *use* stays bare (`in g`, `mut(g)`, `Foo<g>`). Value-group ticks retire (`player'` → `player`, `rc'` → `rc`).
- **Place-subset is `g in h`** — set containment, **not** an outlives relation, and spelled `in`, never `⊂`.
- **`own` is renamed `ownref`** — a reference *mode* for consuming an *immovable* instance (an immovable struct, a `!Movable` future, a class while shared). It never nests in a container (`Box<ownref T>` is ill-formed, and `Box<own T>` was always `Box<T>`); a *movable* value is consumed by plain move. Weak is **`weak Foo`** (replaces `**Foo`); the `*` strong sigil retires (bare is strong).
- **Erasure RC-ness rides the trait kind** (change 4), and the erasure-to-Rust *shape* is what this chapter depends on:
  - **`open trait T` (struct-tier) projects to a real Rust `dyn`** — `&dyn T` (borrow), `Box<dyn T>` (owned / heap). This is the only erased form that becomes a genuine Rust `dyn`; everything this chapter says about `dyn Trait` (the §12.6 projection filter, §6.6 sealed-interface emission, §14 `Future` erasure) applies to this half. Inline `own dyn T` retires → `Box<dyn T>`.
  - **`interface I` (class-tier) has *no representable Rust type* — by design.** It is spelled bare (`I` strong-erased, `&I` borrow, `weak I`), never `dyn I`. Rust cannot instantiate or inspect a class/interface value; at the boundary it holds an **opaque handle** and calls through it (exported entry points / vtable), nothing more. There is no rustc-`Drop`-glue projection (teardown is the class last-claim-release path).
  - **Consequence for API authors:** to export an erased registry to Rust as a real `dyn`, spell it the open-trait way (`Vec<Box<dyn EventHandler>>`), **not** the interface way (`List<EventHandler>`).

The `name: type` colon is grammatically optional but canonical Valen always writes it (no change here). The `let`→bare / `set` binding rule and the parenthesized `mut(g)` effect clauses are unchanged by ratification.

### 11.1 Groups as Vale's lifetime-equivalent

```vale
func process<g'>(items: &[Widget] in g) -> &Widget in g {
    items[0]
}
```

`&Foo in g` says the reference lives in group `g`. Vale's typechecker tracks which group each reference belongs to and ensures no reference outlives its group.

Groups nest explicitly: `g in h` declares g a sub-group of h. References valid for g are valid for h. More expressive than Rust's `'long: 'short` outlives bounds because Vale tracks containment, not just outlives.

**Allocator implementation is a separate concern.** Vale's stdlib provides arena allocators that exploit static group containment for cheap region free, but the group IS the static scope tracked by the typechecker, not the arena.

### 11.2 `&T in g` erasure to `&'re_erased T`

When Vale's frontend generates Rust-shaped code (stub rlib generation, `GenericArgs` construction at Rust call sites), groups erase to `tcx.lifetimes.re_erased`:

Vale source `func process<g'>(x: &T in g)` → stub rlib `pub fn process<T>(x: &T)` → rustc elides to `pub fn process<'a, T>(x: &'a T)` → by monomorphization time, `'a` is populated with `re_erased`.

`re_erased` over `'static` because some Rust trait impls discriminate on lifetime (`impl Deserialize<'static>` is narrower than `impl<'de> Deserialize<'de>`); `re_erased` is rustc's neutral placeholder.

### 11.3 Vale types with group params → PhantomData-tied lifetime slots

```vale
exported(rust) struct Region<g'> {
    data: &[I32] in g
}
```

Stub rlib:
```rust
pub struct Region<'a>(ValeOpaqueType<HASH>, ::std::marker::PhantomData<&'a ()>);
```

At call sites, lifetime slot populated as `re_erased`. From rustc's view, `Region<'re_erased>` is a normal generic instantiation. From Vale's view, `g` has its real identity in Vale's universe.

### 11.4 Vale reconciles Rust lifetime constraints with Vale groups

Vale's frontend, reading a Rust signature with lifetime bounds, reconciles with Vale's group structure:
- Each Rust lifetime parameter becomes a Vale group parameter.
- `'a: 'b` (outlives bound) becomes Vale group containment `b in a` (a contains b).
- HRTBs `for<'a> Fn(&'a T) -> bool` handled via §11.8 mechanism.

For advanced cases (lifetime-discriminating dispatch, nested HRTBs), Vale annotation files (§24) express the reconciliation manually.

### 11.5 Aliasing rules: multi-mut intra-Vale, single &mut at boundary

Vale's source-level aliasing rules are more permissive than Rust's:
- Multiple `&T in g mut` references to the same data can exist intra-Vale; Vale's typechecker tracks which references are visible from which scopes; at most one is "active" at any source position.
- Scope with single visible mut → `noalias`/restrict marking applies.
- Scope with multiple visible muts → no aliasing hint.

At the Rust boundary, tightens to Rust's rules: single `&T in g mut` can project to Rust `&mut T`; multiple visible muts rejected by Vale's typechecker before reaching rustc.

### 11.6 Restrict-pointer marking via single-visible-mut scope analysis

Vale's codegen emits LLVM `noalias` on parameters when Vale's typechecker proves single-visibility. Three patterns: local variable with no aliasing muts in scope; function argument promised single-mut by caller contract; field access through single-mut reference.

Optimization hint; doesn't affect correctness. Single-visible-mut property either Vale-side-proven (projection OK) or unproven (projection rejected).

### 11.7 Outlives bounds expressed via Vale-native group constraints

Rust API `fn copy_from<'src, 'dst: 'src, T>(src: &'src T, dst: &'dst mut T)` (dst outlives src) becomes Vale binding `func copy_from<s', d', T>(src: &T in s, dst: &T in d mut) where s in d`.

Vale's frontend handles translation automatically based on the Rust signature. Vale source users see Vale-style group constraints; underlying Rust ABI gets corresponding lifetime bounds.

### 11.8 HRTBs: auto-generated where possible

HRTBs appear at the Rust boundary in three contexts:
1. **Closures Vale passes to Rust APIs.** Iterator combinators, callbacks. Closure-to-trait-impl machinery (§14) generates HRTB-shaped `Fn` impls automatically.
2. **Vale impls of Rust traits with lifetime params.** Vale's typechecker reads trait signature, generates corresponding impl with the lifetime parameter.
3. **Vale APIs taking Rust callbacks with HRTB bounds.** Auto-translate Vale's group param into HRTB-quantified Rust lifetime.

Mechanism is mechanical; common cases work via auto-gen.

### 11.9 HRTBs deferred for v2

Two HRTB-related cases deferred to v2:
- **Lifetime-discriminating trait dispatch** (some Rust APIs have specialized impls based on lifetime). v1 forbids Vale source from invoking such APIs through paths hitting lifetime-discriminating dispatch.
- **Nested HRTBs** (`for<'a> Trait<for<'b: 'a> InnerTrait<'a, 'b>>`). Vale's auto-translation doesn't handle; v2 considers annotation format or Vale source syntax.

For v1, users with HRTB-heavy interop needs use annotation files (§24) or work around via thin Rust wrapper crates.

### 11.10 The `dangle` annotation (and `#[may_dangle]` projection)

`dangle` is a **user-explicit annotation** in Vale source (Q65) — never inferred for user-declared types. It is declarable on **a specific function's group parameter** (usually `drop`): a checked promise that the body never dereferences through that group, propagating through the call graph (hand-off only to `dangle`-accepting callees). *(The earlier **type-level** form — `dangle` on a struct/type making the group identity-only for the whole type — was **cut 2026-07-04**; it is per-function only. Older prose in this doc that shows `struct Foo<G: dangle>` reflects the pre-cut design.)* The full language-side semantics — stored-reference poisoning, which operations remain legal on poisoned values, and consumers of poisoned linear values — live in the Valen language reference (`valen-design-1.md`, "Stored references and poisoning"); the companion `runtime` multi modifier (strong/weak-only references) lives in `valen-design-2.md`. This section covers the piece that matters at the Rust boundary: Drop's `dangle` and its `#[may_dangle]` projection. Exact syntax shakes out as group-borrowing lands; conceptually:

```vale
// `dangle` sits on a specific function's group parameter (usually drop's),
// never on the struct/type — the type-level form was cut 2026-07-04.
func drop(self: ownref Container) dangle(g) { /* body never dereferences through g */ }
```

(Syntax provisional; the load-bearing property is that the user explicitly marks the group on the consuming function, not the analyzer inferring it.) The typing pass enforces:
- Code in T's Drop body that accesses values from a `dangle`-annotated group is **rejected** at typecheck — checked invariant, not unsafe assertion.
- T's Drop impl, projected to Rust at the boundary, automatically carries `#[may_dangle] G_as_re_erased_lifetime`.
- Groups without `dangle` project strict (no may_dangle); Vale freely allows Drop bodies to access values from those groups.

The enforcement mechanism IS group-borrowing's analysis machinery — the same analysis that already tracks per-expression group reads/writes for the broader borrow-checking story. Dropck projection is a derived output of analysis the typechecker already does universally; no special-case "dangle inference" surface that could miss a feature.

**Closure capture-driven inference (Q65 A1 sub-answer)** is the one exception, narrowly scoped to synthesized types where there's no user-facing surface to annotate. For closures and async state machines, the lifted struct's dangle status composes from the AND of its owned fields' dangle statuses (which are themselves user-checked per the rule above). Conservative-default: any owned field whose dangle status is unknown defaults the synthesized type to non-dangle-compatible — fail-closed, never fail-open. User-supplied annotation overrides if explicit syntax exists for naming the synthesized struct.

**Why this doesn't replay Sky's §29.A.may-dangle rejection.** Sky rejected recursive structural-drop analysis on the grounds that "every new feature has to re-prove 'is structural drop preserved through this construct?' Skip a case → silently emit may_dangle for a Drop that actually reads T → silent unsoundness." Under Vale's model:
- For user-declared types: `dangle` is **checked, not asserted** — typing pass validates that Drop body doesn't access dangle-group values before emitting `#[may_dangle]`. Failure mode of incomplete analysis is *compile rejection*, not silent UB. New language features that affect drop behavior must declare group effects correctly OR the analysis conservatively rejects the dangle annotation.
- For synthesized types: inference composes pre-validated per-field facts. Skipping a case doesn't lead to fabricating `may_dangle` — it defaults to non-dangle-compatible. Fail-closed at every level.

The system is fail-closed; Sky's fail-open concern doesn't apply.

**Stdlib containers** (Vec, HashMap, Box, Rc) opt their type parameters into dangle-capability per convention. Vec's Drop body iterates and calls T's drop on each element; T's storage is owned by Vec by then; no external group observable. Per Rust stdlib's standard pattern, Vec carries `#[may_dangle] T`.

**`dropck_eyepatch` feature flag** enabled in every Vale-generated stub rlib's crate attributes (`#![feature(dropck_eyepatch)]`). Rustc-nightly-only; required for `#[may_dangle]` syntax. Auto-emitted by `vale-stub-gen` alongside other feature flags.

**@DRAFD invariant** (Dangle-Region-Annotation-Flows-Drop): `#[may_dangle]` emission flows from Vale's user-explicit `dangle` annotation, validated by group-borrowing's analysis. No syntactic shape-scan at the stub-gen layer; no analyzer-fabricated dangle status for user types. The soundness invariant lives in Vale's source-level type system + the universal analysis pass, not in stub_gen's post-hoc analysis. See §26.

### 11.11 Rust → Vale reference imports

When Rust source calls a Vale function passing `&T` or `&mut T`, Vale's typechecker lifts the incoming Rust reference into Vale's group system. The lift creates a fresh anonymous group at the function boundary, bounded by the Rust borrow's lifetime.

| Rust signature | Vale-side import |
|---|---|
| `x: &T` where T:Sync | `x` in fresh anonymous group; **no mut effect on x** — Vale truly won't mutate through this ref, and it's safe to share across threads |
| `x: &T` where T:!Sync | `x` in fresh anonymous group with a **mut effect on the whole object** — the honest translation of "shared, interior-mutable, single-thread-visible" |
| `x: &mut T` (any T)   | `x` in fresh anonymous group with a **mut effect on the whole object** — the exclusive form, same mut-group shape as `&T where T:!Sync` |

**Properties:**

- **Honest translation in both directions.** Rust `&T where T:Sync` is truly immutable and thread-shareable; it lifts to a Vale no-mut group. Rust `&T where T:!Sync` is shared but interior-mutable and single-thread-only — the honest Vale representation of that concept is a mut group, and that's how it lifts. Rust `&mut T` guarantees exclusive access; it also lifts to a mut group. Under this framing, Vale mut groups are the uniform representation of "may mutate, single-thread-visible," and Vale no-mut groups are the uniform representation of "cannot mutate, safe to share cross-thread." The old rule (reject `&T where T:!Sync` at import) was a defensive posture that avoided the mismatch by refusing the type; the honesty framing accepts it and represents it correctly, and the parallel-for demotion refinement in §12.6 keeps it sound.
- **Vale's typechecker enforces the promise.** For no-mut groups, group-borrowing's effect tracking (§12.5) enforces that the function body doesn't mutate through the ref — Vale's stdlib doesn't ship the interior-mutability escape hatch that would let a no-mut ref smuggle mutation, so the "won't mutate" promise is genuine. For mut groups, the typechecker permits mutation through the ref (via annotated Rust methods per §24) and tracks the effect via group-borrowing's mut-effect propagation. Rust `!Sync` types imported as mut groups reflect the fact that mutation may happen through Rust-side aliases at any time; the mut effect is honest about that.
- **Independent anonymous groups per parameter.** Multiple incoming references (`fn vale_fn(a: &T, b: &T)`) lift to distinct anonymous groups. Vale can't infer aliasing across the Rust function boundary; independent-groups treatment is sound (no-mut-effect refs freely alias intra-thread; `&mut` args are non-aliasing per Rust's own guarantee; `&T where T:!Sync` mut groups are single-thread-visible so cross-parameter aliasing is intra-thread and harmless).
- **Lifetime = Rust borrow's scope.** `fn vale_fn<'a>(arg: &'a T)` gives the imported group validity `'a`. Vale's typechecker rejects programs that try to extend the reference beyond `'a` (e.g., storing into a long-lived Vale-internal structure).
- **Outlives bounds translate.** `'a: 'b` on incoming refs becomes Vale group containment `b in a` — inverse of §11.7's Vale → Rust direction.
- **`'static` refs lift unbounded.** Rust `&'static T` gives an unbounded imported group; can be stored in any long-lived Vale structure. Common in practice (string literals, statics, `Arc<T>::deref` results).
- **Cross-call propagation is uniform.** Once lifted, imported refs pass through Vale's group analysis like any Vale-internal ref. Field access produces child groups; group-borrowing's invalidation rules apply.
- **Round-trip Vale → Rust → Vale loses tracking precision.** A ref that Vale gave to Rust (projected with re_erased), passed back to a different Vale function, lifts as a fresh anonymous group Vale-side — Vale can't reconnect it to the original group it came from. Correct but imprecise; acceptable for v1.

**Rust !Sync data crosses the boundary directly.** Under the honesty framing, `&Cell<T>`, `&RefCell<T>`, `&Rc<T>`, and any `&T where T:!Sync` cross into Vale straightforwardly — Vale imports them as mut groups. Rust callers don't need to unwrap, clone-out, or `borrow_mut()`-through gymnastics for the single-threaded call case; Vale accepts these refs and models their semantics honestly. Escape hatches that were previously required for the import itself dissolve.

**Where a workaround is still needed: cross-thread sharing.** The one case where Rust callers hit the projection rule (§12.6) is when Vale needs to share the referent across parallel workers — inside a `parallel for` body, outside groups that are mut don't demote to no-mut (because that would violate Rust's `&T: Send iff T: Sync` rule for the pointee). Callers wanting to share a `!Sync` referent across Vale parallel workers hoist the sync-safety onto Rust: wrap in `Mutex<T>` and pass `&Mutex<T>` (Mutex is `Sync`, so it lifts to a no-mut Vale group, is shareable across parallel workers, and workers acquire the guard's exclusive access via `.lock()`). Same pattern as any Rust code sharing `!Sync` data across threads; nothing Vale-specific.

Callback-based Vale APIs remain a design option for cases where Vale API authors want their consumers to keep !Sync bookkeeping entirely on the Rust side. Vale API takes `impl FnOnce(&T) -> R` (or similar); the Rust caller writes `|_| { use(&non_sync_t) }` and passes the closure. The closure captures the !Sync ref, but Vale never sees it — Vale just invokes the closure and gets a return value. Useful when Vale's involvement is a pure computation on extracted contents.

**Mental model.** The Rust → Vale ↔ Vale → Rust translation is symmetric and honest. `&T where T:Sync` maps to a no-mut Vale group (truly immutable, thread-shareable); both other Rust reference shapes (`&T where T:!Sync` and `&mut T` any T) map to a mut Vale group (may mutate, single-thread-visible). The projection rule from §12.6 enforces the same shape in the outgoing direction: Vale projects `&T where T:!Sync` back to Rust only from a mut Vale group; from a no-mut group only Sync targets are allowed. Vale mut groups are the uniform representation of Rust's "non-thread-safe-mutable" concept; no-mut groups are the uniform representation of "thread-shareable immutable."

`RefCell`'s runtime borrow-check remains a useful *interop primitive* for a different reason: when a Rust caller has other live aliases to the RefCell and wants to give Vale a genuinely exclusive `&mut T`, `borrow_mut()` proves that exclusivity at runtime. That's still an escape hatch — but for exclusivity, not for the import itself. Direct `&refcell` (giving Vale shared-but-mut access) also works now, and Vale's mut group discipline handles the "aliases might mutate" semantics correctly.

Vale implementing a Rust trait whose method takes `&T where T:!Sync` as a parameter also works under the honesty framing — the incoming ref lifts to a mut group in the impl body, subject to the usual mut-group discipline.

**Worked example — Rust !Sync data through a Vale `parallel for`.**

```rust
pub struct GameState {
    id: u32,
    entities: RefCell<Vec<Entity>>,
}

pub fn get_id(gs: &GameState) -> u32 { gs.id }
```

`GameState: !Sync` because it contains `RefCell`. `get_id` reads an inline scalar and is Vale-annotated no-mut per §24.

```vale
func drive(gs: &GameState mut) {
    id_here = get_id(gs)

    parallel for i in range(0, 10) {
        id = get_id(gs)
    }
}
```

The lift of `&mut GameState` gives `gs` an anonymous mut group — the honest translation of "shared, interior-mutable, single-thread-visible" when the Rust pointee is `!Sync`. `id_here = get_id(gs)` outside the loop is fine; the mut group is single-thread by §12.5 and the reborrow-and-call is safe.

The `id = get_id(gs)` inside `parallel for` is rejected:

> cannot reference `gs` inside `parallel for` body. `gs`'s mut group has pointee type `GameState`, which is Rust-defined and `!Sync`. The "outside groups become immutable inside the body" relaxation requires the pointee to be safe to share across threads under a no-mut view; Rust `!Sync` means it isn't. The mut effect on `gs`'s group therefore persists inside the body, which per §12.5 makes the group single-thread-visible.
>
> Fixes (Rust side): wrap in `Mutex<GameState>` and pass `&Mutex<GameState>` (Sync); extract Sync-safe content into a local before the loop; restructure so the shared ref doesn't cross the parallel boundary.

**Same code shape, Sync pointee — no error:**

```rust
pub struct Snapshot {
    id: u32,
    values: Vec<u32>,
}

pub fn snapshot_id(s: &Snapshot) -> u32 { s.id }
```

`Snapshot: Sync` (no interior mutability).

```vale
func process(snap: &Snapshot mut) {
    parallel for i in range(0, 10) {
        id = snapshot_id(snap)
    }
}
```

`Snapshot: Sync` lets the outside mut group demote to immutable inside the body per the parallel-for demotion rule (§12.6), so parallel workers share `snap` safely.

Two things this pair makes legible:

1. **The rule catches cases the effect system alone wouldn't.** `get_id` is genuinely no-mut — the mut-effect check passes on its own. What fires is the pointee-type Sync check on the parallel-for demotion. Without that check, the code would compile and race on `BorrowFlag` if any parallel worker's Rust call reached the RefCell.

2. **The rule is targeted, not blanket.** Same code shape, Sync pointee → works. Users don't pay for the rule when it isn't needed, and the diagnostic points at the specific type-level property (`GameState: !Sync`) so the fix is obvious.

**[OPEN] Remaining soundness holes and design opportunities.**

The honesty framing above plus the projection filter's generalization (§12.6) closes multiple soundness classes that started as OPEN items. What remains genuinely open is smaller than the original list.

- **Closed by this session's fixes.**
  - The specific `&mut !Sync` + parallel-for hole (closed by the parallel-for demotion refinement and the generalized projection rule).
  - **Trait objects with hidden interior mutability** — closed by the projection filter's uniform coverage of vtable dispatch (§12.6). Bare `dyn Trait` is `!Sync` per rustc's auto-trait rules; dispatching from a no-mut Vale context is rejected. Users opt in to no-mut-context dispatch by writing `dyn Trait + Sync`, where rustc's coercion check verifies per-implementer.
  - **Drop-glue interactions when Vale-owned closures with `!Send` captures leave scope** — closed for the specific `CancellableFuture<F, H>` case by the conditional Sync/Send/Unpin impls (§14.7); more broadly by the "every ownership-transfer edge requires a Send check" meta-note (§12.6). Closure stubs are now `ValeOpaqueType`-wrapped (§14.1), so no accidental auto-derive on closures either.
  - **Futures with captured `!Sync` state polled from Vale** — closed by the auto-derive fix (`ValeOpaqueType` fail-closed markers, §10.6) plus the state-machine exemption from universal Sync (§14.4). State machines get field-walk-verified Sync, not auto-derived.

- **Still open — genuine soundness corners.**
  - **`Pin<&mut T>` with self-referential T at the boundary** — Vale has no Pin per @NoPin. Mostly dissolves because Pin's DerefMut requires `T: Unpin`, so safe Rust can only hand Vale `&mut T` for movable Ts; `!Unpin` cases require Rust-side unsafe (chargeable to Pin's contract, not to Vale). Still-open sub-piece: **Vale awaiting Rust futures embedding a started `!Unpin` Rust child** — needs the Movable field-walk composition rule (§12.3, §14.5) to fully cover once implementation lands.
  - **Unsafe-transmute patterns** — Rust code that transmutes to bypass the type system. Largely Rust's problem (charged to the `unsafe` block).
  - **`catch_unwind` in Rust callbacks passed to Vale** — panic=abort discipline means `catch_unwind` silently loses the sandbox in any Rust code reachable from a Vale-binary build graph. Documented behavior of panic=abort (§16.3); not a Vale-specific hole.

- **Vale/Rust interop opportunities.** Vale API design principles worth codifying: prefer value-returning parameters over reference parameters when contents-only access suffices; treat callback APIs as first-class rather than fallback; consider whether Vale can offer standard "wrapper-unwrap adapter" traits that let Rust ecosystem code plug into Vale without per-crate annotation authoring.
- **Opportunities this reveals about Vale itself.** Vale's "no ambient interior mutability in no-mut groups" stance combined with runtime-borrow-check-as-exclusivity-primitive suggests Vale's own concurrency + shared-mutation story might benefit from analogous mechanisms. Are there Vale-side patterns that could use RefCell-shaped runtime exclusivity proofs? Would a Vale-native `SharedCell<T>` or `RuntimeExclusive<T>` primitive give Vale users cleaner escape hatches without importing Rust types?

Ship-ready doc language for the remaining corners will follow as they're worked through.

The direction pair is asymmetric with §11.2: Vale → Rust erases lifetimes (Rust knows them at definition time and Vale's frontend has already validated them); Rust → Vale preserves them in the imported group (Vale's group system needs the scope to track validity).

---

## 12. Send, Sync, 'static, Unpin (Honest at Boundary)

Vale's **Send** is auto-derived at the typing pass via field walking, like Rust's auto-trait model. Vale's **Sync** at the rustc boundary is emitted **universally for all Vale-defined types by construction** — no field walk — backed by group-effect enforcement (§12.6) plus a **projection filter** that requires Vale → Rust `&T` projections targeting `!Sync` types (per rustc's Sync trait — includes bare `dyn Trait` and any `!Sync` Rust type) to come from a mut Vale group. Applied uniformly at every Vale → Rust ref-projection point: return positions, argument positions, callback invocations, and vtable dispatch on `dyn Trait` receivers. Both claims are **honest at the rustc boundary** — no global `unsafe impl Send` lie per Sky §12.1; Vale's real analysis backs each emitted claim. Send-able variants are obtained via allocator-generic types per Q45 β (e.g., `String<GlobalAlloc>` is Send; `String` defaults to `String<LocalAlloc>` which is `!Send`). Cross-thread reference sharing at the Vale → Rust boundary uses standard `&T` projection with no wrapper — Vale-side cross-thread safety comes from group borrowing's effect tracking (§12.5), not from a Vale-source Sync surface. This is the most architecturally significant Vale-vs-Sky divergence in the boundary mechanism.

### 12.1 Send auto-derived; Sync always-emitted for Vale types

Vale's typing pass auto-derives Send per type via field walking — same mechanism as Rust's auto-trait, but performed Vale-side. **Default: Send if all fields are Send** (matches Rust's auto-trait default; revises Q45 B's earlier "default-!Send + explicit opt-in" lock). Vale's typing pass has full visibility into Vale type structures (opacity is only Rust-side; Vale source declares the fields directly). Stub_gen at the boundary emits `unsafe impl Send for Spaceship` iff Vale's derive concluded Send; otherwise, no impl is emitted — and because Vale-defined stub structs wrap `ValeOpaqueType<HASH>` which carries `PhantomData<*mut ()>` (§10.6), rustc's field-walk auto-derives `!Send` in the absence of the explicit impl. Rust callers receive an honest Send claim that Vale's typing pass actually computed; where no claim is emitted, the type is genuinely `!Send` at rustc's level, not silently auto-derived.

This is the correctness pivot from ordinary Rust practice: for auto-traits, omission of an `impl Send` does NOT produce `!Send` — rustc still auto-derives from the type's fields. If the fields all happen to be Send (as with a `PhantomData<()>` sentinel), rustc concludes Send anyway, silently. The `ValeOpaqueType` wrapper's `PhantomData<*mut ()> + PhantomPinned` field composition prevents this: rustc's field-walk sees the negative marker and concludes `!Send + !Sync + !Unpin`. Vale then emits positive `unsafe impl`s per its verified analysis. Every auto-trait claim reaching Rust is either explicitly emitted (backed by Vale's analysis) or explicitly absent (Vale's analysis concluded the negative). No accidental auto-derive from wrapper fields.

This sidesteps the "Rust can't auto-derive on opaque types" problem cleanly: Rust doesn't derive positively — Vale does the derive Vale-side and informs stub_gen, which emits the explicit `unsafe impl`. Honest at boundary because Vale's analysis IS the source of truth.

**Sync at the boundary is emitted universally for most Vale-defined types** — stub_gen emits an explicit `unsafe impl Sync for T` for every Vale-defined struct/enum/etc., without field walking. Two carve-outs on the universal emission:

- **Compiler-synthesized async state machines are exempted** (see §14.4). Their captures can transitively include Rust `!Sync` types via the honesty framing (§11.11) — the "no ambient interior mutability in Vale-defined types" argument doesn't cover them. Vale's field-walk over the state machine's captures determines Sync-ness, same as Send.
- **Parameterized wrapper types (closures, `CancellableFuture<F, H>`, other wrappers with type parameters) emit conditional Sync bounds** that field-walk over the type parameters — e.g., `unsafe impl<F: Sync, H: Sync> Sync for CancellableFuture<F, H> {}` (§14.7). Vale-defined but with T-dependent field content, so the universal emission doesn't apply; instead the conditional form matches Rust's ordinary auto-trait propagation shape.

For all other Vale-defined types, the universal emission holds. The emission is the load-bearing act (per the auto-trait discipline above, the wrapper's `PhantomData<*mut ()>` field makes rustc auto-derive `!Sync` in its absence; the explicit `unsafe impl Sync` is what claims it back). Vale's group-effect enforcement (§12.6) guarantees that no-mut-effect `&T` methods — which is all stub_gen exposes through `&T` — cannot perform unsync mutation regardless of T's field content. The **projection filter** (§12.6) closes the boundary leak vector uniformly: any Vale → Rust `&T` projection targeting a `!Sync` T (per rustc's Sync trait) must come from a mut Vale group, applied at return positions, argument positions, callback invocations, and — critically — vtable dispatch on `dyn Trait` receivers. Users can opt out of the universal Sync emission with `unsafe impl !Sync for MyType` for intentional single-thread designs (equivalently: stub_gen skips the universal emission if the source-level type carries `#[cfg(...)]` or an explicit opt-out marker). Dyn objects need no special-case machinery — the projection filter handles them uniformly at the dispatch site by consulting rustc's Sync answer for the `dyn Trait` type (bare `dyn Trait` is `!Sync`, so dispatching from a no-mut Vale context is rejected; `dyn Trait + Sync` is `Sync`, allowed, with rustc's coercion check enforcing that every concrete implementer is Sync). See §12.6 for the projection filter's full statement and worked examples. Vale's Sync claim is stronger than Rust's own auto-derive would give — Vale-defined types with `Cell`/`RefCell`/`Rc` fields stay Sync-shareable at boundary — but honest under HBAB (§26.20): Vale's group-effect enforcement plus the filter together back every claim. Reason it holds even for the RefCell-field case: Vale-defined types are opaque to Rust (§10), so Rust callers can only reach the interior !Sync value by calling a Vale method that projects an `&<!Sync>` ref back; the projection filter requires that come from a mut Vale group, and no-mut Vale methods can never satisfy it. Same argument holds for Vale types with `Box<dyn Trait>` fields — dispatch on the dyn field from a no-mut Vale method projects `&dyn Trait` into Rust, and rustc's Sync trait for `dyn Trait` gates it.

**Unpin is per-type, driven by Movable analysis.** Vale's typing pass computes each type's Movable property: for ordinary Vale structs (which have no self-referential state by construction of the group system — the group system's mutual-isolation rule forbids self-mention), Movable is trivially true; for compiler-synthesized async state machines, Movable is driven by the `async(movable)` attribute (per `valen-design-1.md`'s three-orthogonal-markers model: `async(migratory)` adds Send, `async(movable)` adds Movable/Unpin, `async(cancelable)` adds Cancelable — orthogonal); for future user-declarable self-referential types (if Valen ever grows them), driven by whatever the analysis proves. Stub_gen emits `impl Unpin for T` (a safe impl, not `unsafe`, since Unpin is auto-only) iff Vale's analysis concludes Movable; otherwise no impl is emitted, and the wrapper's `PhantomPinned` field makes rustc auto-derive `!Unpin`. The `!Unpin` case is what Pin's safe API respects: `Pin<&mut T>` where `T: !Unpin` can't produce `&mut T` via DerefMut, so Rust callers can't accidentally move self-referential Vale state machines.

**User override** is available for cases auto-derive can't handle:
- `unsafe impl Send for MyType` in Vale source asserts Send when auto-derive would conclude !Send. Standard Rust escape-hatch pattern.
- **Dyn objects (trait objects) require explicit annotation** because the typing pass can't see through trait erasure to determine the concrete implementer's Send-ness. User writes `Box<dyn FlyingInterface + Send>` or annotates the trait/wrapper directly. Rust's pattern; Vale mirrors.

Mechanism for cross-thread variants: **allocator-generic types** (Q45 β). Vale stdlib provides allocator-parameterized collections:
```vale
struct String<A: Allocator = LocalAlloc> { /* ... */ }
struct Vec<T, A: Allocator = LocalAlloc> { /* ... */ }
struct HashMap<K, V, A: Allocator = LocalAlloc> { /* ... */ }
struct Box<T, A: Allocator = LocalAlloc> { /* ... */ }
```

`String<LocalAlloc>` (the default) is `!Send` (thread-local allocator's pointers can't cross threads). `String<GlobalAlloc>` is `Send`. Users wanting cross-thread mobility instantiate with `GlobalAlloc`. Type aliases for ergonomics: `type SendString = String<GlobalAlloc>`.

**Wrapping Rust stdlib under valec-rs.** Vale stdlib's allocator-generic collections wrap their Rust std equivalents under valec-rs at runtime — `Vec<T, A>` wraps `rust.std.vec.Vec<T, A>`, `HashMap<K, V, A>` wraps `rust.std.collections.HashMap<K, V, A>`, `Box<T, A>` wraps `rust.std.boxed.Box<T, A>`. Allocator parameter passes straight through; layout is identical; boundary projection is zero-copy. Under valec (and at comptime in either binary), these resolve to pure-Vale impls of the same allocator-generic surface — selected via the per-item `#[cfg]` mechanism from §3.3.

**`String<A>` is a special case** because Rust's stdlib `String` is hard-coded to `Global` allocator (`alloc::string::String = String { vec: Vec<u8> }`, no `<A>` parameter). The upstream PR adding `String<A>` (rust-lang/rust#149328) has been open since November 2025 with a multi-year lineage of stalled predecessors (#101551, #79500); landing timeline is unknown. Vale doesn't gate on the PR; instead Vale stdlib's `String<A>` uses **per-instantiation backing selection via `comptime let`** under valec-rs:

```vale
#[cfg(rust_interop)]
struct String<A: Allocator = LocalAlloc> {
    comptime let inner_type: AnyType = calculate_string_backing(A);
    inner: inner_type
}

func calculate_string_backing<A: Allocator>(a_type: A) AnyType {
    if a_type == GlobalAlloc { return rust.std.string.String; }
    else { return Vec<u8, A>; }
}

#[cfg(not(rust_interop))]
struct String<A: Allocator = LocalAlloc> {
    inner: Vec<u8, A>              // valec has no rust.std.string.String to reach
}
```

The `comptime let inner_type: AnyType = ...` binding is evaluated at each instantiation of `String<A>`; the resulting type value binds to `inner_type` and is then referenced as the field's type (`inner: inner_type`). `AnyType` is Vale's trait for type values (§13.4). The helper `calculate_string_backing` is a regular Vale function that runs at comptime — its body uses ordinary `if` because the whole call is at comptime; no `comptime if` needed at type positions.

For A=GlobalAlloc under valec-rs, `&vale_string.inner` IS `&rust.std.string.String` natively — zero-copy `&String` / `&mut String` boundary projection, no unsafe transmute, no accessor hop. For other allocators, Vale's String backs onto `Vec<u8, A>`. Stdlib's `String` methods (push_str, contains, replace, etc.) dispatch per instantiation via analogous helper functions returning `AnyType` or via `comptime if` at body level (§13.1's constexpr-if analog) — the GlobalAlloc path delegates to Rust's optimized `String` impls; other-allocator paths use pure-Vale impls over `Vec<u8, A>`. When #149328 eventually lands, the `calculate_string_backing` helper's else-branch retires uniformly to `return rust.std.string.String<A>;`; pure internal refactor; no Vale source change beyond stdlib.

**Dependency on Rust's `allocator_api` unstable feature.** Vale's allocator-generic stdlib relies on Rust's `#![feature(allocator_api)]` being available — `Vec<T, A>`, `HashMap<K, V, S, A>`, and `Box<T, A>` have had the allocator parameter on nightly for years via this feature; Vale absorbs its evolution as part of normal nightly-bump maintenance (§4.4). `String<A>` is NOT yet in the feature set (the open PR above is the candidate landing path); Vale handles its absence via the per-instantiation backing selection above. If `allocator_api` ever stabilizes in a meaningfully different shape, the wrapping layer is the load-bearing surface that has to absorb the change — Vale source code stays unaffected.

Same pattern for the imported Rust `Rc`/`Arc` at the boundary: separate stdlib classes (Q45 follow-up), not a single parameterized `Rc<T, Sync>`. `Rc<T>` is intrinsically `!Send`/`!Sync`; `Arc<T>` is `Send`/`Sync`. User picks at type use. Note: Valen v1 does not ship a Vale-native atomic-RC class analog (see `valen-design-2.md`'s Reference counting section); when Vale source needs cross-thread reference-counted sharing under valec-rs, users `import rust.std.sync.Arc` and use Rust's native `Arc<T>`. Under valec (no rustc), users can only use Vale's own `class` (thread-scoped RC via the ambient multi `rc`) — cross-thread sharing requires the other stdlib primitives (`parallel spawn`, `async(migratory)`, `Channel<T>`).

### 12.2 'static falls out by construction

Vale types are `'static` from rustc's view by construction (not by lying):
- Vale types have no Rust lifetime parameters in their definition (PhantomData<&'a ()> for groups; 'a erased to re_erased at use). No lifetime params surface to rustc.
- Vale types don't carry Rust borrows in fields (Vale's typechecker enforces — fields are values, owned references inside groups, or other Vale types; not Rust borrows surfaced to rustc).
- Group references erase to re_erased at boundary; even Vale source borrows stored in fields don't appear as Rust lifetime params.

Result: Vale type holding Vale data with groups erased is genuinely 'static from rustc's view. No lie needed. Rust APIs requiring `T: 'static` accept Vale types automatically.

### 12.3 Unpin/Movable: per-type basis, orthogonal to migratory

Vale futures — and Vale-defined types generally — are not all Unpin. `valen-design-1.md`'s async model has three orthogonal marker attributes; each maps to one Rust auto-trait:

| Attribute | Vale-side property | Rust-side impl emitted by stub_gen |
|---|---|---|
| `async(migratory)` | Send (captures cross-thread transferable) | `unsafe impl Send for X {}` |
| `async(movable)` | Movable (state machine has no self-refs) | `impl Unpin for X {}` |
| `async(cancelable)` | Cancelable (ambient cancel-channel) | (Vale-language marker; ambient cancel-channel plumbing per §14.5; `into_cancellable` wrapper for tokio drop-cancel compat per §14.7) |

Migratory and Movable are **independent**. A future can be Movable but not migratory (single-thread inline-storable), migratory but not Movable (Send-safe but pinned), both, or neither. Earlier drafts of this section bundled them; the language reference splits them cleanly and the interop doc now follows.

**Emission rules:**

- **Movable / Unpin.** Stub_gen emits `impl Unpin for X {}` iff Vale's typing pass concluded Movable. The conclusion is a **verification by field-walking** the type's contents — the `async(movable)` attribute on an async fn is a *claim* the compiler checks, not a decree. For each field/capture the typing pass consults its Movable status: Vale-native fields derive recursively via the same rule (ordinary Vale structs are trivially Movable because the group system forbids self-mention); **Rust-imported field types are consulted against Rust's `Unpin` trait**. If any field or capture is `!Movable` (Vale `!Movable` or Rust `!Unpin`), the enclosing type is `!Movable` and the `async(movable)` (or equivalent) claim fails verification at compile time with an error naming the offending field. This matches §12.4's shape for Send derivation (which consults Rust's `Send` trait for Rust-imported field types) — the field-walk crosses the interop boundary uniformly, using Vale's own analysis for Vale-defined types and Rust's own auto-traits for Rust-defined ones. Absent the emitted impl, the `ValeOpaqueType` wrapper's `PhantomPinned` field makes rustc auto-derive `!Unpin` (§10.6). Pin's safe API then forbids `&mut T` extraction, matching the "no move" requirement of self-referential state.
- **Send.** Stub_gen emits `unsafe impl Send for X {}` iff Vale's field-walk over captures/fields concluded Send (§12.1). Absent the impl, the wrapper's `PhantomData<*mut ()>` field makes rustc auto-derive `!Send`.
- **Sync.** Stub_gen emits `unsafe impl Sync for X {}` universally for Vale-defined types (§12.1), with the projection-filter backing (§12.6). Absent the impl (opt-out via `unsafe impl !Sync`), rustc auto-derives `!Sync`.

**On `tokio::spawn`.** Tokio's spawn bounds are `F: Future + Send + 'static` — no `Unpin` requirement (tokio pins internally). So **Send is the gate for cross-thread transfer**, not Unpin. A future can be `!Unpin` and still spawnable via tokio::spawn, provided it's Send. Earlier phrasing that tied `tokio::spawn` acceptance to `Unpin` was wrong; corrected here. This makes §12.1's Send emission the load-bearing check for tokio spawnability, and makes the wrapper's fail-closed `!Send` default the load-bearing safety property — a Vale-emitted future stub is `!Send` unless Vale explicitly claims Send, so `tokio::spawn(f)` compiles only when `f`'s Send has been verified.

**Pin honesty preserved because Pin has runtime consequences.** Pin's safe API forbids moves out of `!Unpin` types; Rust callers honoring Pin correctly avoid moving non-Movable Vale types. Vale's typechecker forbids moves of non-Movable types from Vale source. Both sides honor pinning.

### 12.4 Why honest (not Sky's "lie globally")

**Vale diverges from Sky §12.1.** Sky lies globally: every Sky type gets `unsafe impl Send` at the stub rlib level; Sky's typechecker enforces actual sendability Sky-side; rustc sees a phantom claim it can't verify.

Vale doesn't lie about Send or Sync. Both claims at the rustc boundary are honest, but derived by different mechanisms:

- **Send** is auto-derived at the typing pass via field walking (§12.1); stub_gen emits `unsafe impl Send for T` when all fields aggregate to Send. Rust's own Send trait is consulted for Rust-imported field types (recursive auto-trait shape). **Borrow-mention rule:** the field-walk covers owned content only — any group-borrow field (`&Foo in g`) makes the type non-Send at this layer, fail-closed, and no `unsafe impl Send` is ever emitted for it. Valen-internal bounded sharing of borrow-holders (the freeze-window judgment — Valen language reference, "The two-layer transfer gate") has no per-type projection; rustc's Send is per-type and cannot carry a window.
- **Sync** is emitted universally for all Vale-defined types (§12.1). stub_gen emits `unsafe impl Sync for T` for every Vale-defined struct/enum by construction. The claim is backed by group-effect enforcement (§12.6) — no-mut-effect `&T` methods can't do unsync mutation — plus the projection filter that requires Vale → Rust `&T` projections targeting `!Sync` types (per rustc's Sync trait) to come from a mut Vale group (uniformly across return, argument, callback, and vtable-dispatch positions).

The "unsafe" is justified because Vale's real analysis (field walk for Send; group-effect enforcement + projection filter for Sync) backs each claim.

Rationale:
- **Vale's typing pass HAS visibility Vale-side** even though Vale types are opaque to Rust. Send auto-derive happens against Vale's full type structure (fields, nested types, recursively). Rust receives the result.
- **Honest semantics match Vale's "stronger safety than Rust" stance.** Lying-globally (Sky's pattern) means Sky tells Rust "trust me, Send" with no verification path; misuse produces silent data races. Honest emission means Rust gets claims Vale verified.
- **User can override** for cases automatic emission can't reach (dyn objects, unsafe-impl-asserted patterns, intentional !Sync single-thread designs) via `unsafe impl Send for MyType` / `unsafe impl !Sync for MyType` etc. — parallel to Rust's escape hatches.
- **Sync is stronger than field walk would give.** Vale-defined types with `Cell`/`RefCell`/`Rc` fields stay Sync-shareable at boundary because group-effect enforcement rules out unsync mutation through any no-mut-effect `&T` method Vale would expose, and the projection filter blocks the leak vector at every point where Vale could hand Rust a ref to the interior !Sync value from a no-mut context.

### 12.5 No boundary wrapper for cross-thread reference sharing

Vale projects references to Rust using standard Rust types — `&T` for a Vale ref in a group with no mut effect on that reference, `&mut T` for a Vale ref carrying a mut effect. **No special wrapper type mediates the boundary.** Prior drafts of this document proposed a `ValeImm_T_Ref<'parallel>` per-type opaque wrapper for pure/parallel-block projections; that mechanism is **retired** under the effect-tracking model.

Why the wrapper isn't needed:

- **Vale's "no mut effect on this group in this function" is a genuinely-immutable claim**, not a "Vale won't unsync-mutate but interior mutability might" weakening. Two mechanisms back this: (1) Vale stdlib doesn't ship `Cell` or `RefCell` (§12.6), so there's no ambient Vale-source way to mutate through a no-mut-effect reference; (2) Rust `&T where T:!Sync` refs imported from the boundary lift as **mut** Vale groups, not no-mut (per §11.11's honesty framing), so !Sync interior-mutability doesn't sneak into no-mut Vale groups via imports either. The primitives that DO permit mutation through no-mut references — `Mutex`, atomics, channels — are all explicitly synchronized, and their use is cross-thread-safe by construction.
- **Universal Sync emission (§12.1) + projection filter (§12.6)** projects Vale's no-mut-effect groups' safety guarantee honestly to Rust's type system. When Vale hands Rust a `&T` derived from a no-mut-effect group, Rust's own trait system permits cross-thread sharing per the standard `&T: Send iff T: Sync` rule, and Vale's Sync claim is backed by Vale's group-effect enforcement plus the projection filter — the wrapper's job (formalize "Vale's proof is stronger than per-type Sync") is now handled by the effect-tracking model directly.
- **Vale mut-effect projection to Rust references.** When Vale hands Rust a reference derived from a mut Vale group, valec projects it as `&mut T` (enforcing at the boundary that the ref is temporarily the only visible reference to that object — tightening Vale's more permissive intra-Vale multi-mut per §11.5 to Rust's exclusive-`&mut` rule for the projection call site), or as `&T` via reborrow (standard `&mut T → &T` reborrow within scope; Rust's own type system handles Sync bounds on the reborrowed `&T` for whatever the caller does with it). Under the honesty framing, `&T where T:!Sync` projections from a mut Vale group are the natural translation for the "shared but interior-mutable, single-thread" concept; the projection filter (§12.6) requires exactly this — Vale can only project `&T where T:!Sync` from a mut source, never from a no-mut one.

Under the retired wrapper design, ValeImm's role was to formalize "Vale's immutability proof is stronger than Rust's per-type Sync." Under the effect-tracking model with no Cell/RefCell, that stronger proof is now the *only* immutability story Vale offers Rust — there's nothing weaker to disambiguate against. Standard `&T` projection covers the case.

Cross-thread **ownership transfer** (moving a Vale value to another thread rather than sharing a reference to it) uses the specific stdlib mechanisms — migratory async futures (§14), channels — each with its own per-feature safety analysis. Send auto-derive (§12.1) is what those mechanisms check against. Cross-thread **shared ownership** (multiple threads holding refs to the same reference-counted value) uses Rust's imported `Arc<T>` under valec-rs; Valen v1 doesn't ship a Vale-native atomic-RC class analog.

### 12.6 Group effect tracking replaces per-type Sync Vale-internally

Rust needs `Sync` as a per-type property because `&T` permits interior mutation — `Cell.set`, `RefCell.borrow_mut`, `atomic.store`, `Mutex.lock` are all callable through `&T`. Sync distinguishes types where such mutation synchronizes across threads (Mutex, atomics) from those where it doesn't (Cell, RefCell).

Vale doesn't need a per-type Sync property Vale-internally, because **there's no ambient interior-mutability surface to guard against**:

- **Vale stdlib does not ship `Cell` or `RefCell`.** The aliasing patterns those types traditionally covered are handled by group borrowing directly (multiple `&Foo in g` refs into a group with mutations tracked via mut effects; child-group invalidation on mutation per Nick Smith's group-borrowing article). Rust `Cell` / `RefCell` are importable from Vale source and usable as fields in Vale types, but their mutating methods are not callable through references in groups that have no mut effect on the reference — so their utility in Vale source is small.
- **`Mutex`, atomics, and channels are the sole synchronized-mutation primitives.** They ARE usable through no-mut-effect groups — the primitives handle their own cross-thread synchronization internally, and their use doesn't violate a no-mut-effect promise (their mutations are properly synchronized).
- **Pure reads** are always allowed through any group.

**Enforcement via group-effect signatures on Rust APIs.** How does Vale's typechecker know that `Cell::set` requires a mut effect but `AtomicUsize::store` doesn't, given the two have structurally identical Rust signatures (both `&self, val`)? The distinction lives in per-method Vale-side annotations delivered via §24 annotation files, which already spec "group effects of Rust methods" as one of their annotation shapes. Stdlib annotations shipping with the Vale toolchain declare:

- `Cell::set<g' mut, T>(self: &g' Cell<T>, val: T)` — the `g' mut` marker requires a mut effect on the receiver's group. Called through a no-mut-effect group ref → Vale's typechecker rejects.
- `Cell::get<g', T: Copy>(self: &g' Cell<T>) -> T` — no mut effect declared. Callable through no-mut-effect refs. Reads via `UnsafeCell`; safe as long as no concurrent writer, which Vale's enforcement guarantees.
- `AtomicUsize::store<g'>(self: &g' AtomicUsize, val: usize, ordering: Ordering) { unsafe { /* atomically mutate */ } }` — no mut effect declared, `unsafe { }` internally provides the actual mutation. Hardware synchronization backs it.
- `Mutex<T>::lock<g'>(...)` — no mut effect; returns a guard whose deref lives in a child group with mut effect. Standard synchronized-primitive pattern.
- `Rc::clone<g' mut, T>(...)` — mut effect on the shared refcount storage; refcount increment is unsynchronized. Blocked from no-mut-effect groups.

The pattern generalizes: synchronized-mutation primitives declare no-mut-effect signatures with `unsafe { }` providing the actual mutation; unsynchronized-mutation primitives declare mut-effect signatures. Vale ships stdlib annotations for common Rust std types; the ecosystem maintains annotations for popular third-party crates via the same §24 mechanism. Generic-propagation cases (`impl<T: Cache> Cache for MyCacheWrapper<T>`) fall out of trait signature inheritance — impl methods conform to the trait's declared group effects.

The effect-tracking rule:

- **A function that declares no mut effect on group `r`** truly does not mutate anything in `r` for the duration of that function's execution — not even through interior mutability, because Vale has no ambient interior-mutability surface.
- **Consequently, refs in `r` are safe to share cross-thread** during that function's execution (across Vale threads directly; across Rust threads when the ref is projected as `&T` per §12.5, backed by the universally-emitted Sync claim of §12.1 for the Vale-defined pointee — subject to the state-machine and parameterized-wrapper carve-outs noted there).
- **A function that declares a mut effect on group `r`** signals it may mutate `r`; that group is single-thread-visible during the mut-effect window.

The consequence: at any moment, a group is either single-thread-visible with mut effects active OR shareable-across-threads with no active mut effects. Transitions happen at function-call boundaries via effect annotations. This is essentially Rust's `&T` vs `&mut T` semantics applied at the group level, with the effect annotation as the discriminator instead of the reference type.

**Arc and Mutex compose naturally.**
- `Arc<T>` provides only shared access to inner T — any reference through Arc is a child group with no mut effect on the shared T. Matches Rust's `Arc<T>` giving only `&T`.
- `Mutex<T>.lock()` is a synchronized primitive callable through no-mut-effect refs to the Mutex; returns a guard whose deref lives in a child group with a mut effect on that guard's contents (the lock guarantees exclusive access for the guard's duration). Same shape as Rust's `Mutex<T>::lock(&self) -> MutexGuard<T>`.
- Compositions like `Arc<Mutex<T>>` fall out (shared ownership + mutable-via-lock) without new mechanism.

**Boundary implication (Vale → Rust): always-Sync emission + projection filter.** Vale projects a no-mut-effect ref as bare `&T`; stub_gen emits `unsafe impl Sync for T` for every Vale-defined type. The claim is honest because Vale's group-effect enforcement guarantees no-mut-effect `&T` methods can't do unsync mutation, no matter what T's field content is.

One leak vector must be closed: **Vale's frontend requires any Vale → Rust `&T` projection targeting a `!Sync` T to come from a mut Vale group.** "Target `!Sync`" is decided by consulting rustc's `Sync` trait for the specific type at the projection point — uniformly, regardless of whether T is Rust-authored (`Cell<u32>`, `RefCell<T>`, `Rc<T>`), Vale-authored (subject to §12.1's universal Sync emission unless opted out), or an erased type like `dyn Trait` whose Sync-ness rustc computes from the trait bounds present (`dyn Trait` bare is `!Sync`; `dyn Trait + Sync` is `Sync`, with rustc's coercion check enforcing that any concrete implementer satisfies the bound). The filter applies uniformly at every point where a Vale-side ref becomes a Rust `&T`:

- Return positions (`fn foo(&self) -> &T`) — where a Vale method hands Rust the ref.
- Argument positions (`vale_body { rust_fn(&some_val) }`) — where Vale passes a ref into a Rust call.
- Callback invocations Vale performs on Rust closures.
- Trait-method dispatch through Vale's impls of Rust traits.
- **Vtable dispatch on `dyn Trait` receivers** — every call `x.method()` where `x: &dyn Trait` (or via `Box<dyn Trait>`, `Rc<dyn Trait>`, etc.) is itself a projection at the vtable-call site: Vale-side ref → Rust-side `&dyn Trait` argument via the vtable. The target of that projection is `dyn Trait`, and rustc's Sync answer for it depends on the trait's declared bounds. `dyn Trait` (bare) is `!Sync` by rustc's auto-trait rules; `dyn Trait + Sync` is `Sync`, with rustc's coercion check enforcing that any concrete implementer is Sync.

Without this filter, Vale could expose (e.g.) `fn get_cell_ref(&self) -> &Cell<u32>` from a no-mut-effect method, giving Rust a `&Cell<u32>` on which it could call `Cell::set` and bypass Vale's enforcement; or pass `&self.some_refcell` to a Rust function argument from a no-mut method context; or dispatch a trait method on `&self.some_dyn_logger` where the concrete impl behind the erasure has interior mutability. The projection filter rejects all three at Vale-compile time:

- `fn read_cell(&self) -> u32` — OK (owned value out)
- `fn get_cell(&self) -> Cell<u32>` — OK (owned via `Cell::clone`; separate Cell, no shared state with Vale's)
- `fn get_cell_ref(&self) -> &Cell<u32>` — REJECTED (returns ref to a `!Sync` type from no-mut context)
- `fn frob(&self) { some_rust_fn(&self.the_refcell) }` — REJECTED (passes ref to a `!Sync` type as argument from no-mut context)
- `fn frob(&mut self) { some_rust_fn(&self.the_refcell) }` — OK (source group is mut; projection allowed)
- `fn tick(&self) { self.logger.log() }` where `logger: Box<dyn Logger>` — REJECTED (vtable dispatch projects `&dyn Logger`; `dyn Logger` without `+ Sync` bound is `!Sync` per rustc)
- `fn tick(&self) { self.logger.log() }` where `logger: Box<dyn Logger + Sync>` — OK (`dyn Logger + Sync` is `Sync`; rustc's coercion check already verified any concrete implementer is Sync)
- `fn tick(&mut self) { self.logger.log() }` where `logger: Box<dyn Logger>` — OK (source is mut group; projection allowed regardless of dyn target's Sync)

Vale-defined types are always safe to expose from no-mut contexts (recursively Sync by construction, subject to §12.1's universal emission unless opted out). Rust-imported types and erased types get consulted against Rust's Sync trait; `!Sync` targets are rejected from any no-mut-effect projection point. Generic instantiations get checked at monomorphization — a generic projection targeting `&T` gets rejected specifically for those T values that resolve to `!Sync` types, giving a localized per-instantiation error rather than a whole-type verdict. With the filter in place, always-Sync-for-Vale-types is airtight, and the dyn-dispatch class of soundness holes closes without any new gate — vtable dispatch is simply another projection point the filter already covers.

The consequence for dyn traits: writing `dyn Trait` (bare) in Vale source is only useful in mut contexts (or for pure ownership-only handling that never dispatches from a no-mut context). Users who want no-mut-context dispatch on erased receivers write `dyn Trait + Sync` — rustc's coercion check does the per-implementer verification. Sealed Vale interfaces (§6.6) get a natural bonus: since Vale controls the entire impl universe and can verify every impl is Sync at trait-declaration time, stub_gen can emit the `+ Sync` bound automatically at the boundary for sealed traits whose Vale impls all satisfy it — the sealed nature makes the closed-world check tractable. Open traits and Rust traits require explicit user opt-in via `+ Sync`, matching Rust's own convention.

**Boundary implication (Rust → Vale): honesty framing.** Rust `&T where T:Sync` lifts to a Vale no-mut group; Rust `&T where T:!Sync` and Rust `&mut T` (any T) lift to Vale mut groups. Vale no-mut groups uniformly represent "cannot mutate, safe to share cross-thread"; Vale mut groups uniformly represent "may mutate, single-thread-visible." This is the Rust → Vale side of the same rule that the projection filter enforces going out — see §11.11 for the full framing and worked example.

**Boundary implication (Vale-internal): parallel-for cross-thread safety has two independent gates.** Vale's `parallel for` body distributes iterations to workers on potentially different threads. Depending on the iteration form, one of two gates applies:

- **By-reference iteration** (`parallel for x in &xs`, or any iterator whose Item type is a reference). Workers hold references into outside data. Vale's `parallel for` body demotes outside groups to no-mut effect (§Threading in the Valen language reference), enabling parallel workers to share references cross-thread — but this demotion only fires when the pointee type is safe to share across threads under a no-mut view. Vale-defined pointees are always safe (recursively Sync by construction, per the arguments above). Rust-defined pointees are consulted against Rust's Sync trait: `T:Sync` pointees demote normally; `T:!Sync` pointees keep their mut effect inside the body, which per §12.5 makes them single-thread-visible and therefore inaccessible to parallel workers. Referencing a `!Sync`-origin outside group inside a parallel body is a compile error at the reference site, with a diagnostic naming the pointee's `!Sync` property and the standard fixes (wrap in `Mutex<T>` on the Rust side; extract Sync-safe content before the loop; restructure so the shared ref doesn't cross the parallel boundary). Without this gate, demoting a mut group with a Rust `!Sync` pointee to no-mut and letting parallel workers share it would violate Rust's `&T: Send iff T: Sync` rule.

- **By-value iteration** (`parallel for x in xs` — where the iterator moves owned elements out of the container). Each iteration transfers ownership of an element into a worker's body; the worker runs the iteration on its thread; the element drops on the worker's thread at iteration end. This is an ownership-transfer edge across threads, and Vale requires **`Item: Send`** — same shape as the Send check `parallel spawn` performs on its captures (§14.5), just applied at the iteration boundary. For Vale-defined element types, Vale's own Send derivation answers via field-walk (per §12.1's mechanism — e.g., `String<GlobalAlloc>: Send`, `String<LocalAlloc>: !Send`). For Rust-imported element types, Vale consults Rust's Send trait. Attempting by-value `parallel for` over a container of `!Send` elements is a compile error, with a diagnostic naming the offending element type. Without this gate, a `parallel for s in local_strings` where `local_strings: List<String<LocalAlloc>>` would compile and drop `!Send` `String<LocalAlloc>` values on worker threads at iteration end — the LocalAlloc dealloc would run on the wrong thread. UB.

The two gates are independent: the reference gate is about *sharing* refs across threads (Sync-shaped); the by-value gate is about *transferring* ownership across threads (Send-shaped). Vale's iterator machinery distinguishes the two cases via the iterator's declared Item type (reference vs owned), and applies the appropriate gate. Mixed iterator forms (e.g., an iterator producing tuples of `(T, &U)`) apply both: Send on the whole Item type covers both by-value transfer of `T` and by-reference sharing of `U`, since `&U: Send iff U: Sync`.

**Meta-note — ownership-transfer edges as a discipline.** These two gates are specific instances of a broader Vale discipline: **every point where Vale transfers value ownership across a thread boundary requires a Send check on the transferred type**, with Send derived per §12.1 (Vale's field-walk over Vale-defined types; Rust's Send trait consulted for Rust-imported types).

The currently-known ownership-transfer edges:

- `parallel spawn f(x^)` — captures of `f` and `x` must be Send (§14.5).
- `async(migratory) func` — all captures in the state machine must be Send (§14.5).
- `Channel<T>::send` — `T: Send` required (see `valen-design-1.md`'s Channels section).
- `parallel for x in xs` (by-value iteration) — `Item: Send` (this section, above).
- `executor.take(fut^)` for cross-thread executors — Send required based on the executor's declared thread-affinity property (this section, above; and see `valen-design-1.md`'s Async chapter for the soundness minimum).
- Rust-boundary `unsafe impl Send` emission — driven by field-walk (§12.1).

The individual gates are stated per-edge in the relevant chapters; the enumeration above collects them for readers wanting the meta-view. This is **intentionally kept as a comment rather than formalized as a `@DOTOT` invariant with CI enforcement** — the enumeration is small enough that code review catches slippage, and formalizing evolution machinery for a not-yet-shipping language is premature optimization. If the enumeration grows large enough that comment-level discipline stops scaling, or if the community adds enough transfer constructs that CI enforcement becomes valuable, upgrading to a formal `@DOTOT` invariant in §26 (enumerating the edges, providing per-edge test fixtures, adding a typechecker chokepoint helper) is the natural evolution.

**Careful when adding new transfer edges.** Any future language construct that transfers value ownership across a thread boundary — a new async primitive, a new parallel combinator, a new stdlib channel-family variant, a custom executor scheduling mechanism, an FFI shape that hands owned values to Rust — needs its own explicit Send gate stated at the construct's declaration site, and should be added to the enumeration above. **This discipline has been missed twice in this doc's development**: `Channel<T>::send`'s Send bound and `parallel for` by-value iteration's Item-Send check were both absent from the initial spec and caught only by outside review. Future edges deserve explicit attention at design time; the pattern here is "does this transfer ownership across threads? If so, state the Send gate up front." When in doubt, add the gate.

**Compared to earlier draft with read-group annotation.** An earlier draft of this chapter introduced an explicit `read` group annotation with an access-level rule (Cell.set blocked, Mutex.lock allowed). That mechanism is retired: mut-effect tracking on function signatures is a strictly simpler way to express the same property, and dropping Cell/RefCell from Vale stdlib closes the "read group but with Cell access" gap entirely. One concept (mut effect on group) replaces two (read/normal annotation + per-method access rule).

### 12.7 Mutex at Vale call sites: always takes the real lock

Vale's `Mutex<T>` wraps `rust.std.sync.Mutex<T>` under valec-rs (consistent with the stdlib-wraps-rust pattern per §12.1). Vale doesn't ship a separate Vale-native Mutex; layout is identical to Rust's; boundary projection is zero-copy.

**Vale always takes the real lock at `Mutex.lock()` call sites in v1.** Rust's stdlib provides `Mutex::get_mut(&mut self) -> &mut T` for the case where `&mut self` statically proves exclusive access — the lock op is elided. In principle Vale could generalize this via group analysis: a mut effect on a group containing a single-visible Mutex ref would allow lowering `mutex.lock()` to `mutex.get_mut().unwrap()`. **Vale does not do this in v1.** The optimization is deferred; the cost of the always-taken lock (single uncontended CAS on modern platforms) is judged small enough to accept for architectural simplicity.

A theoretical follow-on optimization Vale could add later — reducing a Mutex in a mut-effect group to a `RefCell`-like boolean flag when group analysis proves single-thread access — is also not on the v1 roadmap. Not blocked by anything architectural; just not built.

**Boundary interop.** When Vale hands Rust a `&Mutex<T>` or `&mut Mutex<T>`, Rust callers see standard `rust.std.sync.Mutex<T>` and use its native API. Vale's mut-effect group analysis has no cross-boundary reach; Rust callers take the real lock (matching Vale's own always-locks behavior).

**Cost consequence.** With no Cell/RefCell in Vale stdlib (§12.6) and always-locked Mutex, Vale under this model gives up zero-cost interior mutability entirely. Uncontended-Mutex cost (~single CAS) is small but nonzero; over hot inner-loop iterations it can matter. Group borrowing's more permissive aliasing rules mitigate most Cell/RefCell use cases at compile time — mutations tracked via mut effects; multiple readwrite refs into a group are allowed intra-Vale per §11.5. Cases where the mitigation isn't enough are candidates for the future single-thread-flag optimization above.

---

## 13. Comptime

Comptime is first-class in Vale: same expression language at compile time and runtime, slab-based representation, content-hash typeids for comptime-produced types crossing into rustc-visible territory. Vale's comptime supports the futamura projection: specialization-of-interpreters via partial evaluation. This is one of two major architecturally-load-bearing additions Vale carries from day 1 (the other is async, §14).

### 13.1 Zig-style comptime

Same expression language at compile and runtime. One type universe. `comptime` keyword (Q19 lock). Restrictions (Q44):
- **No IO** except `include_file!`-style at parse/lex time (file's bytes baked into binary as if a string literal; evaluator never touches filesystem).
- **No nondeterminism**: no timestamps, no random numbers, no system queries.
- **Terminating** via **instruction-count budget** (Q44 lock — not time-based; time is nondeterministic). Exceeded budget = compile-time error with source position.
- **No unsafe at user level**, BUT stdlib gates `unsafe { ... }` blocks via `comptime if __deterministic()`:

```vale
func push_str(self &mut String, s &Str) {
  comptime if __deterministic() {
    self.safe_slow_push(s)
  } else {
    unsafe { self.unsafe_fast_push(s) }
  }
}
```

Interpreter only ever sees safe Vale code. At runtime, `__deterministic()` is compile-time-known false; codegen prunes the safe branch.

**Pointer ops allowed**, but pointers are slab offsets internally; ASLR-irrelevant.

**No multithreading at comptime in v1.** Don't foreclose later multithreading (architecture mustn't bake in single-threaded assumptions that block future multi-thread variants). Cross-crate parallel comptime (during cargo's parallel rustc invocations) is orthogonal.

### 13.2 Slab-based machine simulation

Comptime values represented in a slab — contiguous byte buffer simulating RAM. Bump-allocator-style. Comptime allocations referenced by `usize` offsets internally.

Per-rustc-invocation lifecycle. Created when Vale's machinery activates; populated during typechecking and per_instance_mir queries; discarded at invocation end. Never serialized.

Comptime results that need to persist across invocations bake into the typed AST in resolved form, not as slab references. Slab is purely Vale-internal substrate.

### 13.3 Content-hash const args (no slab-pointer-as-u64)

Vale adopts Sky §29.A.content-hash-const-args from day 1. When a comptime value crosses into rustc-visible territory as a const generic argument, the value surfaces as `ConstKind::Value(content_hash_bytes)` — content-addressed u128 hash — NOT as slab offset.

Why: slab-pointer-as-u64 framing produces dual-Instance/single-symbol conflict (two source sites with content-equal values at different slab offsets generate two distinct rustc Instances under v0 mangler; both Instances produce the same content-addressed body symbol → comdat or non-deterministic linker tiebreak).

Content-hash naming dedups at the Instance level: snapshot-at-capture semantics; same canonical content → same hash → same Instance → no dual-symbol race.

Stub source: `pub fn zork<const T: u128>(...)` — T is the content hash. Vale's per_instance_mir provider, when rustc queues `zork::<HASH>`, looks up HASH in Vale's universe → recovers the snapshot value → substitutes Vale-side → produces synthetic MIR body.

Slab stays purely Vale-internal substrate for evaluator scratch space.

### 13.4 Comptime is value-only; no type-producing comptime

Per Q47: comptime produces **values only**, never new types. New types come from `struct`/`enum` declarations (statically written) + tuples (possibly with comptime-arity expansion) + derive synthesis via comptime functions (§13.10).

Schema-driven code-gen pattern: user writes `u.get("name")` (method call with comptime-known args), NOT `u.name` (field access on a comptime-synthesized type). The compiler partial-evaluates the method call when all args are comptime-known. Long-term sugar layer (analog of Sky's "Model B") may provide `u.name` syntax over `u.get("name")` — separate decision.

**Comptime CAN select among pre-existing types** — distinct from constructing new ones. The mechanism is **`comptime let` binding a value of type `AnyType`**, computed by a regular Vale function that returns `AnyType`. The bound name is then referenced as a type wherever a type slot appears.

`AnyType` is a Vale trait that all Type values satisfy — the comptime analog of Rust's `Any` for type-of-type inspection. `rust.std.string.String`, `Vec<u8, A>`, `Box<T>`, and other pre-existing types or generic instantiations impl `AnyType`.

Canonical pattern (see §12.1's `String<A>` for the concrete case):

```vale
struct String<A: Allocator = LocalAlloc> {
    comptime let inner_type: AnyType = calculate_string_backing(A);
    inner: inner_type
}

func calculate_string_backing<A: Allocator>(a_type: A) AnyType {
    if a_type == GlobalAlloc { return rust.std.string.String; }
    else { return Vec<u8, A>; }
}
```

The `comptime let` is evaluated per instantiation. The helper function runs at comptime, so its body uses ordinary `if` — no `comptime if` needed; the whole call is at comptime. The returned type value binds to `inner_type`. Then `inner: inner_type` uses the type value as the field's type. No new type is fabricated; the helper selects among pre-existing types (and instantiations of pre-existing generics, which count as pre-existing).

**Positions where `comptime let AnyType` bindings resolve to type slots:**

- **Struct field types** — bind at struct scope; use in field type slots.
- **Function return types** — bind at function scope; use in return type slot. `func foo<A>() -> ret_ty { ... }` with `comptime let ret_ty: AnyType = ...` at function scope.
- **Function parameter types** — bind at function scope; use in parameter type slots.
- **Generic argument positions** — a generic arg is effectively a `comptime let` fed into the receiver; `Vec<inner_type, GlobalAlloc>` when `inner_type` is a comptime-bound `AnyType` value.

Type equality (`A == GlobalAlloc`) is a comptime expression over Type values (Q63 reflection). Instantiations of pre-existing generic types (Box<T>, Vec<T>, etc.) count as pre-existing (Box exists; Box<T> is Box instantiated) — helpers can return them freely.

**What this mechanism is NOT.** `comptime if` is a statement inside function bodies (constexpr-if analog, per §13.1's `comptime if __deterministic()` usage). It's **not** a type-level expression. Earlier drafts of this document treated `comptime if` as a type-position expression (e.g., `inner: comptime if A == GlobalAlloc { X } else { Y }`); that shape is retired in favor of the `comptime let AnyType` + helper-function pattern above.

**Trait impl selection at compile time is not supported** in v1 — `impl <comptime-selected-trait> for X` is out of scope; comptime-selected trait resolution interacts badly with rustc's coherence rules and Vale's own impl-block resolution. Users needing conditional impls express the condition via `#[cfg]` at item level, or use a wrapper trait pattern.

**Match arm scrutinee types via comptime selection**: N/A. Comptime type selection doesn't help pattern-matching semantics.

Different instantiations of the same struct may have different layouts; the per-instance `layout_of` query (§10.5) reports the correct shape per Instance.

Implication: no "type-producing comptime" surface to design. Comptime reflection (§13.6) introspects statically-defined types; doesn't produce them. The `comptime let AnyType` mechanism is selection, not construction.

### 13.5 Comptime determinism requirement

Same Vale source + same Vale version → same comptime values. Cross-machine, cross-compile-session reproducibility. Enforced by:
- Restrictions in §13.1 (no IO except include_file, no nondeterminism, instruction-count-budget termination).
- Ordering discipline (iteration over collections in deterministic order; closures execute in deterministic order).
- BLAKE3-deterministic hashing.

Enables content-addressed typeids (§10.8), reproducible builds (§27), cache-friendly comptime (deterministic eval → cacheable per `(comptime_fn_def_id, args)` → result memoization sound).

### 13.6 Comptime reflection

First-class via stdlib intrinsics (Q63):
- `core.reflect.fields_of(T) -> [Field]` — returns slice of `Field { name: Str, ty: Type }`. Or tuple-shaped variant for type-level pattern matching.
- `core.reflect.name_of_field(F) -> Str`, `core.reflect.type_of_field(F) -> Type`, etc.

Comptime control flow: `comptime for`, `comptime map`, `comptime fold` — language constructs that iterate at comptime and compose results.

```vale
comptime func clone<T>(value: &T) -> T {
  T {
    comptime for field in fields_of(T) {
      field.name: value.[field.name].clone()
    }
  }
}
```

At per_instance_mir time, `clone<Widget>` evaluates `fields_of(Widget)` (typing pass has Widget's structure); `comptime for` iterates; expression tree synthesizes per-field clone calls; spliced into the per-Instance body.

**No compiler-built-in derives.** Vale stdlib's Clone, Hash, Debug, Eq, Ord, etc. all written in ordinary Vale code using comptime reflection. User libraries can ship their own derives same way.

### 13.7 Partial-evaluation engine

Per Q62 Option B: per-Instance, integrated with the instantiator. The instantiator's per_instance_mir provider — already idempotent and content-addressed by `(DefId, GenericArgsRef)` per `instantiator-design.md` — extends to run partial evaluation as part of body construction.

Mechanism:
1. Rustc's collector encounters an Instance of a Vale-defined function with concrete args.
2. Vale's per_instance_mir provider fires. Looks up the function's typed body in Vale's universe.
3. Substitutes concrete args into the body (sunny-karp typed_bodies cache).
4. Walks the substituted typed AST as a tree-walking interpreter. For each call site whose args are all comptime-known (literal, comptime const, or comptime-function call with all comptime-known sub-args), evaluates the call inline and replaces the call node with the result expression.
5. Comptime-function calls evaluate via the slab-based machine; results recursively spliced back into the typed AST.
6. After partial evaluation, the typed AST has fewer call sites and more inline results. Vale's codegen lowers this to LLVM IR via `fill_extra_modules`.

Cache: `(DefId, GenericArgsRef)`-keyed memoization, reusing existing per-Instance cache shape.

**Mode-independence.** The instantiator + partial-eval mechanism is Vale-internal; only the DRIVER differs between binaries. In valec-rs mode, rustc's mono collector queries `per_instance_mir`, which invokes the instantiator + partial-eval per Instance. In valec mode, valec's CLI orchestrator walks its own instantiation queue and invokes the same instantiator + partial-eval per Instance; no `per_instance_mir` (the query only exists as a fork patch in valec-rs per §4.2). Same body-construction substrate; different driver. Top-level `comptime const` bindings (per §13.8) evaluate earlier, at Vale's typecheck time, in both modes — their values are substituted into HinputsT before either instantiation walk starts.

**Serialization under rustc's parallel mono walk (valec-rs).** Rustc's default mono collector runs on rayon workers; multiple workers can query per_instance_mir concurrently. Vale's provider is single-threaded in v1 (§13.11) — enforced via a global lock at the entry of the Vale provider function that serializes all Vale-Instance queries. Rustc's non-Vale work stays parallel; only Vale's provider path is serialized. @GCMLZ (§26.2) discipline applies — the provider must not take other locks that could deadlock against rustc's queries.

**valec mode**: no rustc rayon workers to serialize against. valec's own instantiation walk is single-threaded in v1, so §13.11's single-threaded evaluator invariant holds by construction — no explicit lock needed. If valec's walk gets parallelized in v2, the same v1 lock or v2 per-thread-slab discipline would apply uniformly across both binaries.

**NNGZ uniformity:** non-generic functions partial-evaluate via the same path — empty args, single Instance per @NNGZ degenerate case.

### 13.8 Comptime memoization

- **Top-level `comptime const X = expr`**: always memoized (Q62 sub 2). One evaluation per X per build invocation.
- **Per-Instance partial-eval cache**: reuses sunny-karp typed_bodies cache (`(DefId, GenericArgsRef)` keyed). Within one rustc invocation, fully effective. Across invocations: re-derives (no on-disk persistence in v1; L2 cache reserved for v2 per toylang's layering).
- **Inline `comptime { ... }` blocks** inside function bodies evaluate per-Instance (they may reference enclosing function's type params).
- **Generic-arg memoization** for cases where comptime values flow as generic args: TBD; possibly memoized but not yet locked.

### 13.9 Comptime failure modes

Per Q44 sub 3:
- **Comptime panic** → abort entire compilation (panic=abort discipline per Q26).
- **Results bubble up** by convention. Users `.expect()` Result values before assigning to `comptime const` if the comptime path can fail.
- **Compile-time errors** point at Vale source positions (the call site, recursively into the comptime evaluation stack).
- **Instruction-budget exhaustion** → compile-time error with source position; user adjusts budget in `vale.toml` or restructures.

### 13.10 Derive sugar via comptime

Per Q63/Q64:

```vale
#[derive(clone, std.derives.clone<Self>)]
struct Widget { id: Int }
```

Sugar — `#[derive(trait, synthesis_function<Self>)]` desugars to ATTACHING the named comptime function as a method (or association) on the struct. Eventually shortened to `#[derive(clone)]` where convention maps trait names to canonical synthesis functions (`std.derives.<trait_name>`).

Per Q64 clarification: the derive desugars to a **function**, not necessarily an impl. Whether `#[derive(...)]` ALSO emits an impl block (wiring the function as the trait impl's body) is **separate syntax** — TBD; deferred.

Synthesis functions are ordinary Vale functions in stdlib or user libraries. Per-Instance partial-evaluation at per_instance_mir time evaluates the function with T concrete (via comptime reflection over T's fields); the synthesized body splices into the per-Instance code.

**No `#!Derive*` typing-pass-transformation mechanism.** All derive-style synthesis goes through comptime + reflection + partial-evaluation. One mechanism replaces three (Rust's proc-macros + `macro_rules!` + derive-by-name).

**No blanket impls + specialization in v1.** Users explicitly opt in per type via `#[derive(...)]`. If user wants custom clone, they write `impl Clone for Widget { ... }` directly without `#[derive(clone)]`. No conflict because there's no auto-applied blanket. Avoids specialization soundness questions Rust has been grappling with for years.

### 13.11 Threading: single-threaded comptime in v1

Vale's comptime evaluator is single-threaded in v1. Architecture must not foreclose future multithreading — if multi-thread variants are added later, they should be expressible without re-architecting.

**v1 enforcement (valec-rs mode): global lock at per_instance_mir provider entry.** Rustc's mono collector runs on rayon workers; multiple workers can concurrently query per_instance_mir. Vale acquires a global comptime-evaluator lock at the entry of the provider function; only one thread runs the partial-evaluation engine at a time. Rustc's non-Vale work stays parallel — only Vale's provider path is serialized.

**v1 enforcement (valec mode): single-threaded by construction.** valec's own instantiation walk is single-threaded in v1; the single-threaded evaluator invariant holds without an explicit lock. If valec's walk gets parallelized in v2, the same lock discipline (or the v2 per-thread-slab upgrade below) would apply uniformly across both binaries.

**@GCMLZ (§26.2) discipline applies**: the provider must not take other locks or issue queries that could block against rustc's queries while holding the comptime lock. Provider's work is bounded — substitute args, walk typed AST, evaluate comptime calls, splice results — and doesn't re-enter rustc's query system for anything the mono walk would depend on.

**Slab lifecycle:** the slab holds intermediate comptime values during a single provider invocation. Under (a) sequential access, slab state is consistent per query; between queries the slab can be reset (fresh per invocation) or kept (accumulates across invocations, with content-addressed values dedup'ing naturally). Results baked into typed AST are slab-independent per §13.4 — resolved form, not slab references — so slab state doesn't leak across query boundaries.

**Performance impact:** Vale's provider work is a small fraction of total mono walk time (partial-eval body construction, not the whole walk). Serializing it doesn't kill much throughput. If Phase 5 bench-parity or later profiling shows Vale's provider on the critical path, upgrade to (b) per-thread slabs (below).

**v2 future direction: per-thread slabs.** Each rayon worker gets its own slab arena; comptime evaluation runs in parallel without the global lock. Memoization becomes per-thread (wasted evaluation on cache-miss collisions but not incorrect). Content-hash typeids (§13.3, from §29.A.content-hash-const-args) are content-only and never slab-address-derived, so cross-thread hash consistency holds by construction. Explicit message-passing between threads for values that need cross-thread visibility. This is the more flexible long-term direction; v1 (a) is a stepping stone. Slab shared-by-construction designs (single shared slab + fine-grained locking) are NOT preferred — per-thread arenas + message-passing is the target shape.

Cross-crate parallel comptime (when valec/valec-rs compiles N projects in parallel via cargo's standard build parallelism) is orthogonal to the intra-invocation concurrency above. Each project has its own per-rustc-invocation slab; cargo runs them concurrently in separate processes; no shared state.

---

## 14. Closures and Async

### 14.1 Vale lifts closures to named struct types

Closures needing to flow into Rust APIs (`Fn`, `FnMut`, `FnOnce`, generic Rust APIs) lift to named struct types in the source file's containing stub rlib. Closure `|w| w.is_active()` in `vmdparse/src/foo.vale` becomes `__vale_closure_42` (suffix stable hash of source location). Captured state = struct fields.

Stub rlib representation:
```rust
pub struct __vale_closure_42<'a>(
    ValeOpaqueType<HASH_FOR_CLOSURE_42>,
    ::std::marker::PhantomData<&'a ()>,
);
```

Closure stubs wrap `ValeOpaqueType` per §10.6 — the fail-closed marker composition ensures the closure struct is `!Send + !Sync + !Unpin` by construction. Any positive auto-trait claim (needed for e.g. spawning the closure to a cross-thread executor, storing it in a Send-required container, etc.) requires stub_gen to emit an explicit `unsafe impl` after Vale's field-walk over the closure's captures — same discipline as every other Vale-defined stub type per @HBAB (§26.20). Without the `ValeOpaqueType` wrap, `PhantomData<&'a ()>` alone would auto-derive Send + Sync + Unpin from a `&'a ()` sentinel, silently claiming positive properties Vale hasn't verified. Enforcing the wrap on closure stubs closes that hole.

### 14.2 Closure Fn/FnMut/FnOnce auto-impls

Auto-generated via capture-usage analysis (Q52). Standard Rust rule:
- All captures used immutably + no move-out → `Fn`
- Some captures used mutably + no move-out → `FnMut`
- Some captures moved out → `FnOnce`

Stub rlib emits:
```rust
impl<'a> Fn<(&'a Widget,)> for __vale_closure_42<'a> {
    extern "rust-call" fn call(&self, args: (&'a Widget,)) -> bool {
        ::std::unreachable!()
    }
}
// + FnMut + FnOnce impls
```

`#![feature(fn_traits, unboxed_closures)]` in stub rlib's crate attributes. Per_instance_mir provides real body; fill_extra_modules emits under rustc-mangled symbol.

Vale source doesn't see HRTB syntax; frontend handles translation. Closure parameterized over `'a` to match HRTB-compatible shape callers expect (`F: for<'a> Fn(&'a Item) -> bool`).

### 14.3 Async fns lower to named state machine types

Vale's `async func` desugars to a named struct type. v1 naming: `__vale_async_<fnname>_<sourceloc_hash>`. State machine fields capture each `.await` point's state.

### 14.4 Default async fns (linear)

Vale's default `async func` produces a linear state machine type:
- Vale's typechecker: cannot be dropped from Vale source without explicit consumption.
- Drop glue: linear future dropped from Rust source → a synthesized panic+abort Drop shim fires (§15.7). The future is linear-strict (no user `drop`); Vale source can't drop it, and the shim is the runtime safety net against a Rust-side drop.
- Can hold cross-await borrows (groups across `.await` points). Vale's typechecker tracks group lifetimes correctly. Under the marker-emission discipline (§12.3), the default state machine has `!Send + !Sync + !Unpin` on the rustc side by construction of the `ValeOpaqueType` wrapper (§10.6); no `Send`, `Sync`, or `Unpin` impl is emitted, so all three are negative on the rustc side. **State machines are exempted from §12.1's universal `unsafe impl Sync` policy** — the universal-Sync argument (group-effect enforcement + projection filter for Vale-defined types with no interior mutability) doesn't extend to state machines, whose captures can include Rust `!Sync` types via the honesty framing (§11.11). A future's Sync-ness is emitted only if Vale's field-walk over its captures concludes Sync, same discipline as Send. Pin's safe API forbids `&mut T` extraction, matching self-referential state; `tokio::spawn` rejects at Send bound, matching non-migratory intent.

Allocator: thread-local default (Q26 C3 / Q45 β; futures' state machine storage uses the type's default allocator per regular Vale rules).

### 14.5 Async attributes (opt-in): migratory, movable, cancelable

Per `valen-design-1.md`'s three-orthogonal-markers model, Vale source can opt each async fn into any combination of three attributes; each maps to one rustc-side marker:

- **`async(migratory) func work_thread(...)`** — enables cross-thread transfer. Typechecker enforces:
  - **No `&Foo in g` borrow anywhere in captured state** — not just held across `.await`, but including parameters used only before first await. The state machine migrates at spawn, before first poll, so any borrow in captured state fails Send. This is the strong rule from `valen-design-1.md`'s Async chapter ("borrows can't cross the thread boundary"); the interop doc's earlier phrasing missed initial-state captures.
  - **All captures Send** (field-walk over the state struct).
  - Stub_gen emits `unsafe impl Send for X {}` — honest per @HBAB (§26.20). The emission is what claims Send; absent it, the `ValeOpaqueType` wrapper's `PhantomData<*mut ()>` field makes rustc auto-derive `!Send` (§10.6). No accidental auto-derive from wrapper fields.
  - Migratory does NOT imply Movable/Unpin — self-referential state is orthogonal (see `async(movable)` below). A migratory but non-Movable future is `Send + !Unpin`, spawnable via `tokio::spawn` (which pins internally) but not inline-storable in a `Vec<F>`.
- **`async(movable) func inline_storable(...)`** — enables inline storage (Vec<F>, struct fields, etc.) without heap indirection. Typechecker enforces no self-referential state at definition time (the state machine's captured locals do not hold pointers/borrows into each other) **and** that every captured value's type is itself Movable — including Rust-imported captures, which are consulted against Rust's `Unpin` trait (§12.3). A capture of a Rust `!Unpin` type (e.g., an unpolled Rust async future stored as a captured field across an await) fails the `async(movable)` claim at compile time, with an error pointing at the specific `!Unpin` capture. This is the field-walk composition that keeps `async(movable)` sound over cross-boundary state; same shape as `async(migratory)`'s Send propagation.
  - Stub_gen emits `impl Unpin for X {}` — a safe impl (Unpin is auto-only, not `unsafe impl`), backed by Vale's verified non-self-referential analysis. Absent the emission, the wrapper's `PhantomPinned` field makes rustc auto-derive `!Unpin`.
- **`async(cancelable) func interruptible(...)`** — gives the body an ambient cancel channel for `.cancelable_await`. Boundary-side, adds a `Cancelable` marker (Vale-side; not a Rust auto-trait). Independent of Send/Unpin.

Attributes compose freely: `async(migratory, movable, cancelable) func`. Each independent verification produces its emission independently. `Send` and `Unpin` are orthogonal — a future can be `Send` but `!Unpin` (Send-safe, pinned; tokio::spawn accepts because tokio pins internally), or `Unpin` but `!Send` (inline-storable, single-thread), or both, or neither.

**Interop with `tokio::spawn`.** Tokio's spawn bounds are `F: Future + Send + 'static` — no `Unpin` requirement (tokio pins the future internally). So `Send` is the sole gate for tokio-spawnability. Vale futures marked `async(migratory)` compile with `tokio::spawn`; those without `async(migratory)` do not, regardless of `async(movable)`. Earlier phrasing that tied `tokio::spawn` acceptance to `Unpin` was wrong; corrected.

**Allocator for migratory:** global, unconditionally (Q49 lock). Migratory async captures use global allocator regardless of T's Send-status — required for cross-thread free correctness. Non-migratory captures use the type's default allocator per regular Vale rules.

### 14.6 Migratory propagation through call graph

Vale's typechecker propagates migratory-ness:
- Migratory async fn can `.await` another migratory async fn. ✓
- Migratory async fn CANNOT `.await` a non-migratory (default) async fn. Compile error.
- Non-migratory async fn can `.await` a migratory async fn. ✓

Upward propagation: a function wanting migratory must commit its callees too. @MIGPROP invariant per §26.

### 14.7 Cancellable futures via `into_cancellable`

Vale's default future is linear; dropping aborts. For tokio-API compatibility (drop-as-cancel), users wrap via `into_cancellable`:

```vale
cancellable = into_cancellable(my_future, || {
    cleanup()
})

tokio::select! {
    result = cancellable => { handle(result) }
    _ = shutdown_signal.recv() => { /* cancellable dropped; cleanup runs */ }
}
```

`into_cancellable<F, H>(future: F, handler: H) -> CancellableFuture<F, H>`:
- F is consumed (Vale's typechecker prevents accessing original).
- H is `FnOnce()` cleanup handler.
- `CancellableFuture<F, H>` is non-linear; can be dropped.
- Wrapper's Future impl polls F transparently.
- Wrapper's drop: if F completed normally (Ready), skip handler. If F is still executing, run handler then drop F.

Cancellable futures are opt-in. Vale source explicitly invokes the wrapper.

**Auto-trait emission for the wrapper.** `CancellableFuture<F, H>` wraps `ValeOpaqueType<HASH>` per §10.6, so by default it's `!Send + !Sync + !Unpin`. Stub_gen emits conditional impls derived from F and H simultaneously — not from F's migratory-ness alone:

```rust
pub struct CancellableFuture<F, H>(ValeOpaqueType<HASH_FOR_CANCELLABLEFUTURE>, PhantomData<(F, H)>);

unsafe impl<F: Send, H: Send> Send for CancellableFuture<F, H> {}
unsafe impl<F: Sync, H: Sync> Sync for CancellableFuture<F, H> {}
impl<F: Unpin, H: Unpin> Unpin for CancellableFuture<F, H> {}
```

Both F's and H's auto-trait status matter because the wrapper's drop touches both — the cleanup handler `H` runs on the thread that drops the wrapper (which may be a tokio worker if the wrapper was spawned there), and any residual field-drop of `H` at wrapper-drop time also runs there. Prior drafts of this section left H's Send bound unstated, which allowed the pattern:

```vale
log = String("...")                            // String<LocalAlloc>, !Send
c = into_cancellable(fetch_migratory(url),
    [log = log^]() => { flush(log^) })         // handler captures !Send
tokio::spawn(c)
```

If wrapper `Send` were keyed only on F's migratory-ness (F = fetch_migratory is Send), Rust would accept the spawn. Tokio then migrates the task; timeout/select drops `c` on a worker thread; H's LocalAlloc-backed captures get deallocated from the wrong thread → UB. Under the conditional-impl derivation above, the wrapper is `!Send` (H is `!Send`), so `tokio::spawn` rejects at the Send bound — fail-closed at compile time.

This is the specific `CancellableFuture` shape of the general marker-emission discipline (§12.1, §12.3): every auto-trait claim on the wrapper is an explicit `unsafe impl` (or safe `impl` for Unpin) conditioned on field auto-traits, backed by Vale's field-walk analysis. No accidental auto-derive.

### 14.8 Migratory and cancellable are orthogonal

A future can be migratory but not cancellable, cancellable but not migratory, both, or neither. Vale source can express any combination:

```vale
cancellable_migratory = into_cancellable(some_migratory_future, || cleanup())
cancellable_default = into_cancellable(some_default_future, || cleanup())
```

Each is appropriate for different Rust APIs.

### 14.9 Pin handling: wrapper pattern, no Pin in Vale source

Vale source does not have Pin in its type system. Vale's groups + linear types handle the equivalent role.

At the Rust boundary, wrapper's Future impl includes:
```rust
impl<F: Future, H: FnOnce()> Future for CancellableFuture<F, H> {
    type Output = F::Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<F::Output> {
        let this = unsafe { Pin::into_inner_unchecked(self) };
        // ... poll F ...
    }
}
```

For non-Movable futures (those without `async(movable)`), `!Unpin` honored by Rust callers (Pin's safe API forbids moves). `unsafe Pin::into_inner_unchecked` correct because Vale's runtime guarantees no move has occurred.

For Movable futures (marked `async(movable)`, typechecker verified no self-refs), `Unpin` impl'd via stub_gen emission; `into_inner_unchecked` trivially safe.

Note: Movable and migratory are independent (§12.3, §14.5) — Rust's Pin surface responds to Unpin (Movable), not to Send (migratory). A migratory but non-Movable future is `Send + !Unpin` and remains pinned wherever tokio placed it; tokio's internal pinning handles that case.

Vale source never writes Pin syntax. Wrapper handles all Pin work at boundary.

### 14.10 Two-type split (typestate pattern from day 1)

Vale adopts Sky §29.A.async-typestate from day 1 rather than as future work. Each async fn produces **one rustc-visible type** but **two Vale-source-level typestate witnesses**:

- **`ValeNotStarted_foo`** — pre-execution typestate. Movable, droppable (no state machine progress yet; nothing to clean up). Vale source can construct, change its mind, drop without consequence — the future hasn't started, so there's no captured state to consume.
- **`ValeRunning_foo`** — executing typestate. **Linear** — per the language ref's Async chapter, `Future<T>` with any combination of markers (migratory, movable, cancelable) is linear and consumed only via `.await`, `.cancel()` (if Cancelable), or executor hand-off. The migratory marker does not weaken linearity; every Running future must be explicitly consumed regardless of markers.

Transition: `.start()` consumes NotStarted, produces Running.

Stub rlib emits ONE struct per Vale async fn. IntoFuture impl handles polling in either phase via internal discriminant. Pin/Unpin properties declared on the rustc-level type based on underlying state machine needs.

```rust
pub struct __vale_async_fetch_widget(ValeOpaqueType<HASH>);
// Emissions are independent per attribute:
unsafe impl Send for __vale_async_fetch_widget {}  // iff async(migratory) verified
impl Unpin for __vale_async_fetch_widget {}        // iff async(movable) verified
unsafe impl Sync for __vale_async_fetch_widget {}  // iff Vale field-walks captures and concludes Sync (state machines are exempted from §12.1's universal Sync per §14.4)

impl Future for __vale_async_fetch_widget {
    type Output = Result<Widget, Error>;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        ::std::unreachable!()
    }
}
```

Vale's source-level typestate is enforced by Vale's typechecker: can't `.start()` twice; can't access captures after start; can't drop a Running typestate of default async fn (linearity); migratory/cancellable propagation per typestate.

From Rust's view (Hybrid for ergonomics): calling `.await` on a `__vale_async_fetch_widget` works naturally; the transition to "Running" is internal Vale bookkeeping invisible to Rust callers.

---

## 15. Async Drop and Cancellation

### 15.1 Vale-native race/select with cancel-channel delivery

Vale's stdlib provides `race`/`select` primitives that do not drop losing branches. Cancellation is signaled via the ambient cancel channel added by `async(cancelable)`:

```vale
winner = race(future_a, future_b, future_c).await   // race form: function-call, per language ref
```

Both `race` and `select` require `Future<T> + Cancelable` bounds on their input futures — inputs must be created by `async(cancelable) func`, which is what gives them the ambient cancel channel to receive signals via. Non-cancelable futures cannot participate.

When one future wins, `race` signals losers via their ambient cancel channels. **Loser futures observe cancel signals only at `.cancelable_await` points inside their bodies** (per language ref's Async chapter). Plain `.await` inside a cancelable future does not observe cancel; the task author places `.cancelable_await` at whichever suspension points they want responsive cancellation. Losers reach the next such point, receive the `Cancelled` branch of the `Either`, unwind cleanly.

v1 stdlib primitive. Implementation uses Vale's runtime's task-cancellation mechanism (§17).

### 15.2 `into_cancellable` interface and semantics

When Vale source needs tokio compatibility (drop-based cancellation), wrap via `into_cancellable` (§14.7). Wrapper's drop glue:
- If underlying future completed normally (Ready): skip handler, free wrapper state.
- If still Pending or never polled: run handler, then drop underlying future.

Cleanup handler can do sync-allowed work: send cancel signals, free non-managed resources, log.

### 15.3 Sync cleanup handlers in v1; async deferred

v1: cleanup handlers are sync (`FnOnce()`). Async cleanup deferred to v2 (when concrete use cases emerge). Simpler; matches Rust drop semantics; bounded complexity.

### 15.4 Drop ordering

Cancellable future dropped:
1. Outer cleanup handler runs (access to outer wrapper's captures).
2. Nested fields drop in declaration order. Nested cancellable futures' cleanup runs as fields drop.

Outside-in propagation.

### 15.5 Cleanup failure = abort

Cleanup handler panic = program abort (panic=abort discipline). Cleanup handlers should be simple, fail-safe.

### 15.6 Normal completion skips cleanup

Wrapper tracks state: poll returns Ready → mark complete; subsequent drop skips handler. Poll returns Pending → drop later runs handler. Cleanup runs only on cancellation.

### 15.7 Drop is just a function (AST-rewrite + `__vale_drop<T>` wrapper)

Per Q8 reconsideration locked to Sky §F.22 pattern. Drop is not architecturally special; it's a function the language auto-calls.

**Mechanism in five steps:**

1. **Source-level Drop impls are normal trait impls.** `export impl Drop for X { func drop(self: Self) { ... } }` (Vale source; the by-move `Self` receiver — an immovable/class type would instead take `ownref self` — projects to Rust's `fn drop(&mut self)` at the boundary). From typechecker's view, Drop is just a Rust trait Vale source can impl. Impl block's emission + cascade discovery + fill_extra_modules pipeline unchanged from §6's normal trait-impl handling.

2. **vale-stub-gen emits the impl declaration + a thin generic wrapper.** Stub source:
```rust
impl Drop for X { #[vale::emit_consumer_body] fn drop(&mut self) { unreachable!() } }

pub use core::ops::Drop;

#[inline(always)]
pub unsafe fn __vale_drop<T>(x: *mut T) {
    core::ptr::drop_in_place(x)
}
```
`__vale_drop` is a NORMAL generic Rust fn (`InstanceKind::Item`, real MIR body). `drop_in_place` inside its body is where rustc's `InstanceKind::DropGlue` resolution happens, transparent to Vale.

3. **Type resolution runs.** Produces typed AST as usual; no drop knowledge anywhere.

4. **`insert_scope_end_drops` synthesizes scope-end calls into the typed AST.** The ONE site that knows scope-end calls exist. For EVERY `let` binding, appends synthetic `FnCall { name: "__vale_drop", type_args: [T], args: [Ref(Var(local_name))] }` in REVERSE declaration order (LIFO). No predicate, no `local_needs_scope_drop` decision — wrapper's body bottoms out at `drop_in_place::<T>` which rustc generates as no-op for trivially-droppable T and full drop chain for needs-drop T.

5. **Synthesized calls flow through pipeline unchanged.** Just-another-FnCall — indistinguishable from any user-written generic call. Dep walker collects them via standard FnCall arm. per_instance_mir emits `ReifyFnPointer` cast targeting `__vale_drop::<T>`. Rustc's mono collector queues the wrapper instance, walks its body, sees `drop_in_place::<T>` call, queues `drop_in_place::<T>` via standard `InstanceKind::DropGlue` machinery. For Vale-defined Drop impls (`<Widget as Drop>::drop`), cascade discovery (§8.9) captures the instance; fill_extra_modules emits Vale's body. For std-defined Drop impls (`<Vec<Widget> as Drop>::drop`), rustc emits the body from std's source. `#[inline(always)]` on the wrapper means LLVM inlines at every Vale call site; runtime cost = `drop_in_place::<T>` itself (zero for trivially-droppable T).

**The mono path never thinks about drop as special.** No `local_needs_scope_drop` predicate, no `LIFECYCLE_TRAITS` registry, no `insert_late_scope_end_drops` post-substitution pass. The word "drop" doesn't appear in `per_instance_mir`'s code or in any function it invokes during dep collection. `__vale_drop` is a function whose name happens to start with "drop"; structurally it's just another use-imported generic Rust fn.

**Dual-path drop model — both Vale source and Rust source converge at the same body.**

- **Vale source path**: Vale's compiler runs `insert_scope_end_drops` and appends `__vale_drop::<T>(&v)` for each let. Wrapper's `drop_in_place::<T>` reaches `<T as Drop>::drop` via rustc's standard DropGlue; Vale's emitted body runs.
- **Rust source path**: when Rust source has `let v: ValeType = ...` in a Rust function, Vale's compiler doesn't touch the Rust body. Rustc emits `drop_in_place::<ValeType>` at scope end, which calls `<ValeType as Drop>::drop` via standard trait dispatch. Vale-emitted body resolved by name via single-symbol architecture (§5.2).

Both paths reach the SAME Vale-emitted `<T as Drop>::drop` symbol.

**Linear types** come in two shapes per `valen-design-1.md` (Linear types): a type with a `drop` (its destructor runs at scope end) and a **linear-strict** type with by-move `self`-consumers but no `drop` (Vale source must consume it explicitly on every path). Both flow through the same `__vale_drop::<T>(&local)` wrapper-call at scope end. For a drop-bearing type the wrapper reaches the user's `drop`. For a linear-strict type — which Vale source can never let fall out of scope — the boundary synthesizes a panic+abort Drop shim so that a **Rust-side** drop (which Vale's typechecker cannot prevent) aborts rather than silently skipping the required consumption.

**`dangle` annotation flow:** Vale source `dangle` annotation on a region (§11.10) → typing pass enforces correctness Vale-side → stub_gen emits `#[may_dangle] G_as_re_erased_lifetime` on the Drop impl. @DRAFD invariant.

---

## 16. Panic Propagation

Vale uses `panic = "abort"` exclusively.

### 16.1 `panic = "abort"` enforced at the binary level

valec-rs's generated `.vale-build/Cargo.toml` includes:
```toml
[profile.dev]
panic = "abort"

[profile.release]
panic = "abort"
```

Skyc-style: regenerated on every build; users can't override. Cargo enforces consistency across the build graph (Rust dependencies inherit panic=abort).

`proc-macro` crates and `build.rs` scripts compile with host's panic strategy, not target's. Not in final binary; doesn't matter at runtime.

### 16.2 No unwinding, no landing pads

Vale's compiled bodies don't emit landing pads. Vale's codegen doesn't model unwinding as control-flow concept. Under panic=abort, rustc emits abort intrinsics at panic points; process calls `abort()` immediately; dies cleanly.

### 16.3 No `catch_unwind` semantics

`catch_unwind` doesn't work under panic=abort. Rust libraries that internally use `catch_unwind` for sandboxing silently lose the sandbox. Documented constraint; Vale users use Result-based recovery.

### 16.4 Result-based error model

Recoverable failures: `Result<T, E>`. Unrecoverable invariant violations: `panic`, which aborts. Rust APIs returning Result map naturally to Vale's Result type. Rust APIs that panic (`Vec::index`) abort under bad inputs; users use checked variants (`Vec::get`).

### 16.5 Foreign exceptions across FFI: UB

Compiling a Vale binary that links to C++ libraries throwing exceptions across the boundary = UB. Same posture as panic=abort Rust programs. Documented; C++ side must catch all exceptions before returning. Standard FFI hygiene.

### 16.6 Async cancellation is not a panic

Cancellation is normal scope exit (or Vale-specific abort for linearity violation), not a panic. Drop glue runs in normal scope exit contexts; under panic=abort no drop glue runs at panic time (process dies first). §15.

---

## 17. Tokio and Runtime Interop

Vale's runtime + tokio coexist as independent runtimes in the same process. Bridging is "Vale calls `tokio::spawn(future)` from Vale source" — normal Vale-calls-Rust mechanics.

### 17.1 Vale's runtime and tokio's runtime coexist

When a Vale-defined migratory future is spawned via `tokio::spawn`, the future runs in tokio's executor, not Vale's. Vale's runtime doesn't know this future exists. tokio owns lifecycle.

### 17.2 Waker integration via standard `Waker` ABI

Wakers cross between runtimes via Rust's standard `Waker` ABI. Thread-safe by design (`Waker: Send + Sync`); cross-runtime wakeups safe.

Vale future on tokio awaiting tokio resource: 0 hops. Vale future on tokio awaiting Vale-runtime resource: 1 hop (Vale's resource fires tokio waker). Vale future on Vale-runtime awaiting tokio resource: 1 hop.

### 17.3 Cross-runtime wakeup hops add latency

Documented in Vale's runtime guide; users who care about throughput commit to one runtime.

### 17.4 Vale futures spawned on tokio execute on tokio's threads

Vale's typechecker forbids non-Send Vale source from spawning to tokio (migratory bound, §14.5). For Vale-runtime-spawned futures, Vale's runtime keeps them on Vale's threads.

### 17.5 `spawn_blocking` separated per runtime in v1

`vale.spawn_blocking(closure)` and `tokio::task::spawn_blocking(closure)` are separate APIs in v1. Users pick based on context. v2 considers unified API with current-runtime dispatch.

### 17.6 Mixed-runtime deadlock as Vale-source concern

Standard concurrent reasoning applies. Vale's documentation explains patterns to avoid. Vale's typechecker silent on it.

---

## 18. Build Orchestration

valec/valec-rs orchestrate cargo for build operations. User invokes `valec build`; orchestrator generates `.vale-build/` workspace, spawns cargo, copies result.

### 18.1 `vale.toml` as single user-facing configuration

Users write only `.vale` source + `vale.toml`. Never edit Cargo.toml directly:

```toml
[project]
name = "my_app"
version = "0.1.0"
edition = "experimental"   # pre-1.0 uses "experimental"; 1.0+ uses "2026" etc.

[vale-dependencies]
vmdparse = { path = "../VmdParse" }
parseiter = { git = "https://github.com/.../ParseIter" }

[rust-dependencies]   # only in valec-rs projects
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }

[[bin]]
name = "my_app"
source = "cmd/main.vale"

[lib]
path = "src"
```

Vale Q24-locked workspace shape:
```toml
[workspace]
members = ["my_app", "my_utils", "rust-shim/tokio_wrapper"]
resolver = "2"

[workspace.dependencies]
sky_runtime = { path = "common/sky_runtime" }
serde = "1.0"

[profile.dev] panic = "abort"; opt-level = 0; debug = true
[profile.release] panic = "abort"; opt-level = 3; lto = "thin"; debug = false; strip = true
```

Rust crates allowed as workspace members (Q17 escape-hatch pattern).

### 18.2 `.vale-build/` workspace generation

When valec/valec-rs builds, generates workspace at `.vale-build/` (gitignored; valec emits `.gitignore` entry on first build):

```
.vale-build/
  Cargo.toml                     # workspace manifest
  Cargo.lock                     # cargo-managed, committed
  .cargo/config.toml             # rustflags, panic=abort
  rust-toolchain.toml            # pins vale-rs-nightly (valec-rs) or unset (valec)
  my_app/
    Cargo.toml
    build.rs                     # Vale toolchain check
    src/
      lib.rs                     # vale-stub-gen-generated stub source
      lib.vale                   # symlink/copy of user's source
      main.rs                    # shim: `fn main() { __vale_main(); }`
    target/                      # cargo's output
```

### 18.3 Translation: `vale.toml` → `Cargo.toml`

For each Vale project in the workspace, vale-stub-gen generates a Cargo.toml mapping:
- `[vale-dependencies]` → cargo path/git deps
- `[rust-dependencies]` → cargo dep entries (valec-rs only; valec rejects)
- `[[bin]]` / `[lib]` → cargo bin/lib targets (Vale mirrors Cargo: `[[bin]]` is array — multiple bins per project; `[lib]` is singular — at most one lib)
- Workspace-shared dep versions via `[workspace.dependencies]` + `dep = { workspace = true }`

Cargo.toml regenerated on every `valec build`. Never user-edited.

### 18.4 Cargo.lock placement

`.vale-build/Cargo.lock`. Users commit. Skyc-style: gitignore everything in `.vale-build/` except Cargo.lock.

### 18.5 Deterministic emission

vale-stub-gen's generated workspace bytewise deterministic given identical inputs:
- No timestamps in generated files
- Sorted iteration where HashMap order would affect output
- No random IDs
- No host-system-dependent paths

CI fence: build twice with cache wipes, byte-compare `.vale-build/` contents. Enables cargo's incremental compile correctness and reproducible builds.

### 18.5.1 Cargo's role: build graph, parallelism

valec/valec-rs spawns cargo once via `cargo build --manifest-path=.vale-build/Cargo.toml`. Cargo owns:
- Dependency resolution (reads `.vale-build/Cargo.lock`)
- Build graph topology
- Process spawning (one rustc subprocess per crate in valec-rs mode)
- Incremental skipping (fingerprint checks)
- Parallelism (`-j` flag)
- Linking

**One rustc subprocess per crate** in valec-rs. Rustc was not designed to compile multiple crates in one process; the `run_compiler` API expects one-shot invocation. Vale's per-crate process model means each Vale-machinery activation is independent. Universe re-loaded from cache at each invocation; slab created fresh.

### 18.5.2 Workspace-level Cargo.toml

Workspace root holds profile overrides (panic=abort, opt-level, codegen-units, lto). Per-package profile blocks are silently ignored by cargo per Sky §F.10. vale-stub-gen emits all profile overrides at workspace root only.

### 18.5.3 Subcommand summary

valec/valec-rs subcommands (Sky §18.5.3 inherited):
- `build` / `build --release` / `build --target=<triple>`
- `check` — typecheck only, faster than build
- `test` — run tests; cargo test wrapper
- `run` — build and execute
- `fmt` — format Vale source
- `new <name>` — scaffold new project
- `add <crate>` — update vale.toml with new dep
- `clean` — wipe .vale-build/ and target/
- `inspect <cache-path>` — dump a `.vale-cache` in human-readable form (§7.2's payload)
- `doc` — generate docs (v1.x)
- `publish` — publish to vale registry (v2; not in v1)

LSP mode: `valec lsp` puts the compiler in interactive query mode (Q40 lock). Same binary, two modes.

### 18.5.4 Testing model

Vale source has unit tests via `#[test]` attribute marking. valec test generates Rust test harness alongside normal stub generation; each Vale test gets a Rust `#[test] fn wrapper { unsafe { __vale_test_X(); } }`. Test failure under panic=abort: cargo test runner detects process abort with non-zero exit; reports as failed. Limitation: can't get specific assertion-failure message from runner; Vale's assert macros print to stderr before abort.

Integration tests via `tests/` directory convention; doc tests deferred to v1.x.

### 18.6 Cargo invocation and `vale-toolchain.toml` pinning

valec/valec-rs spawns cargo with `--manifest-path=.vale-build/Cargo.toml`. Cargo inside `.vale-build/` picks up `.vale-build/rust-toolchain.toml`, which pins to `vale-rs-nightly-<date>` (valec-rs) or just to vale-nightly (valec, no rustc involvement but toolchain pin for stdlib compatibility).

### 18.7 Cross-platform / cross-compile

`--target=<triple>` passes through to cargo. Sky's standard cross-compile machinery handles it. Vale's runtime support library is built for target during cross-compile (per-target rustlib-style content shipped with toolchain).

Supported targets v1: x86_64-unknown-linux-gnu, x86_64-apple-darwin, aarch64-apple-darwin, x86_64-pc-windows-msvc. Linux-aarch64 day-1 target via valeup distribution.

---

## 19. Per_instance_mir and Dep Discovery

### 19.1 Approach A (Instance-keyed)

Vale uses Approach A: Instance-keyed `per_instance_mir`, Vale-side substitution. Per §4.1, the load-bearing reason is per-Instance Rust-dep enumeration for the interleaved cases. Arbitrary-typed comptime is secondary reinforcement.

Contract:
```rust
per_instance_mir(instance: Instance<'tcx>) -> Option<&'tcx mir::Body<'tcx>>
```

Provider:
1. Checks: is the instance's def_id a Vale-defined item? If not, return None (falls through to rustc's default `instance_mir`).
2. Looks up item in Vale's universe by def_id.
3. Walks item's body with `instance.args` substituted Vale-side. Partial evaluates calls whose args are all comptime-known (§13.7).
4. Asks Vale's frontend for the set of **Rust** items transitively reachable. Constructs synthetic MIR body mentioning each via `ReifyFnPointer` casts.
5. Returns synthetic body wrapped in `Some`.

**Locked principle:** Vale's per_instance_mir at mono time has one job — walk Vale's call graph to report back the **Rust** things Vale transitively calls. Vale-internal callees are not its concern (no rustc DefId; ReifyFnPointer impossible). Vale-internal callees discovered Vale-side (`walk_and_stash_internal_callees`) and emitted by `fill_extra_modules`; rustc has no role.

### 19.2 Vale-side substitution

Vale's substitution engine (part of typing pass + comptime evaluator) handles:
- Type parameter substitution
- Comptime arg substitution (content-hash u128 values)
- Group param substitution (always to `re_erased` at boundary; Vale-side groups carry full identity)
- Nested generics

Substitution is well-defined Vale-side because Vale owns its type system. Rustc's substitution operates on rustc-known types; Vale's substitution on Vale-known types.

### 19.3 Synthetic MIR body construction for exports

```mir
fn vale_synthetic_body(args) -> ReturnType {
    bb0: {
        let _0: Vec_new_T_i32_Global = Vec::<i32, Global>::new as fn() -> Vec<i32, Global>;
        let _1: Vec_push_T_i32_Global = <Vec<i32, Global>>::push as fn(&mut Vec<i32, Global>, i32) -> ();
        let _2: usize = SizeOf(MyType);
        unreachable;
    }
}
```

Body's sole purpose: drive rustc's collector to queue transitive Rust deps. Body never executes (terminator is `Unreachable`); never produces competing `.o` symbol (partition filter removes consumer items before LLVM codegen). Vale's `fill_extra_modules` is the sole emitter of consumer-item bodies.

MIR construction discipline (Sky §SMINCZ inherited): `Statement` and `BasicBlockData` are `#[non_exhaustive]`; use constructor functions; `set_required_consts` and `set_mentioned_items` must be called with empty vecs; `TypingEnv::fully_monomorphized()` is a typing-mode flag.

### 19.4 ReifyFnPointer casts for Rust deps

```rust
let _0 = Vec::<i32, Global>::new as fn() -> Vec<i32, Global>;
```

Cast source = Vec::new (generic Rust fn); cast target = `fn() -> Vec<i32, Global>` (concrete fn-pointer). Rustc's collector substitutes T=i32, A=Global (Vale's args, pre-substituted), queues concrete `Vec::<i32, Global>::new` for monomorphization.

Recursive through Vale-internal callees: when export's body calls non-export Vale function, Vale walks the non-export's body too, enumerating its Rust deps, includes casts in export's synthetic body.

### 19.5 Per-Instance subtree memoization + typed-body cache

**Layer 1 cache: `(def_id, concrete_args) → RustDeps` walk.** Not yet shipped in toylang reference; per-Instance walk recomputes deps each time. For Vale's larger projects, tracked as future work.

**Layer 2 cache: typed-body cache (sunny-karp pattern).** Vale ships from day 1. Typed AST of every body-bearing Vale fn computed ONCE at `after_rust_analysis` and cached on `ValeState.typed_bodies`. Per-Instance mono substitutes the cached typed AST via pure typed-AST walk (`substitute_in_typed_body`) instead of re-running `resolve_fn_body` + `insert_scope_end_drops` per monomorphization.

Asymptotic savings (Sky §19.5 inherited):
- Pre-cache: K monomorphizations of a generic body cost ~2K full type-resolves + ~2K `insert_scope_end_drops` passes.
- Post-cache: 1 full type-resolve + 1 `insert_scope_end_drops` + K cheap typed-AST substitutions.

Two mechanisms make Layer 2 work:
1. **Oracle accepts Param-bearing queries.** Every oracle query takes `caller_type_params: &[String]`; `try_resolved_to_rustc_ty`'s `TypeParam(name)` arm rebuilds `ty::Ty::new_param(tcx, idx, name)` from the name's position.
2. **`substitute_in_typed_body` is a pure typed-AST walk.** Two-enum split (Sky §F.21) makes invalid StructRef-in-resolved-position unrepresentable at the type level. Source `SourceType` vs resolved `ResolvedType`; chokepoint `resolve_source_type(src, registry)` promotes SourceType → ResolvedType. Codegen consumes only ResolvedType.

**Layer 3 cache: persisted per-Instance results on disk.** Reserved for v2 per toylang's L2/L3 layering. Could amortize per-Instance comptime + substitution work across rustc invocations. Toylang reference has it documented as future work (§22.2 v2 deferred item).

### 19.6 Default trait method resolution via `Instance::expect_resolve`

For Vale types impl-ing Rust traits, default trait methods (Vale didn't override) resolve via rustc's normal trait resolution. Sky §TVIMDGAZ inherited: Vale's code that constructs rustc Instances for trait methods uses **trait def's** method DefId with `[Self, ...]` args, NOT the impl block's method DefId.

```rust
let trait_method_def_id = tcx.associated_items(clone_trait_def_id)
    .find_by_name_and_kind(...).unwrap().def_id;
let args = tcx.mk_args(&[Widget.into()]);
let instance = Instance::expect_resolve(tcx, ParamEnv::empty(), trait_method_def_id, args);
```

---

## 20. Pipeline Ordering

### 20.1 valec/valec-rs invokes cargo; cargo invokes the forked rustc (valec-rs) or Vale's codegen (valec)

Outer pipeline:
1. User runs `valec build` or `valec-rs build`.
2. CLI parses `vale.toml`, generates `.vale-build/` workspace.
3. CLI invokes `cargo build --manifest-path=.vale-build/Cargo.toml`.
4. **valec-rs mode**: cargo walks build graph, spawning forked rustc subprocesses per crate. For Vale-marked crates: Vale's machinery active. For pure-Rust crates: dormant (byte-identical pass-through).
5. **valec mode**: cargo invokes valec's CLI as the wrapper (different protocol than rustc's; Vale's standalone codegen path); each Vale-source crate compiles to `.o` via C++ Backend; system linker links.
6. CLI copies final binary to `./target/`.

### 20.2 Forked rustc loads rlibs; caches deserialize into Vale's universe (valec-rs)

When valec-rs's forked rustc compiles a Vale-marked crate:
1. Startup. Parse argv. Identify crate.
2. Default Callbacks::config(). Vale's codegen backend constructed; query overrides installed (`per_instance_mir`, `layout_of`, `collect_and_partition_mono_items` (partition filter), `cross_crate_inlinable` + extern, `deduced_param_attrs`). `fill_extra_modules` hook installed via `install_consumer_modules_hook`.
3. Rustc parses local crate's Rust source (stub rlib's `lib.rs`).
4. Rustc loads upstream rlibs. Vale's machinery checks each for `__VALE_STUBS_MARKER`.
5. For each Vale-marked rlib loaded: Vale's machinery locates adjacent `.vale-cache` file, deserializes typed AST into Vale's in-memory universe.
6. `Callbacks::after_expansion`. All crates loaded; full universe available.

### 20.3 Hook point: `Callbacks::after_expansion` (valec-rs) / equivalent (valec)

Vale's frontend runs at `after_expansion`. Parses `.vale` source files. Builds local universe. Cross-references upstream universes. Runs typechecker (resolves names, validates groups, checks types). Runs comptime evaluator on comptime expressions. Builds local HinputsT.

### 20.4 Vale's frontend output

- **Vale's local universe** populated with typed AST.
- **Vale's codegen queue** with `(ValeItemId, concrete_args)` pairs for items to emit.
- **Vale's typeid table** populated for comptime-produced types.
- **Cache write** at `after_rust_analysis` (post-typecheck, pre-codegen) → produces `.vale-cache` sibling file.

### 20.5 Rustc typecheck/borrowck on stub bodies (valec-rs)

Rustc proceeds normally. Stub rlib's `unreachable!()` bodies pass trivially.

### 20.6 Monomorphization fires per_instance_mir on exports (valec-rs)

Mono collector starts. Per export Instance, collector calls Vale's `per_instance_mir` provider; gets synthetic body with ReifyFnPointer casts. Collector walks body, queues Rust deps, cascades. Vale's `layout_of` answers for Vale-defined type layouts. Drop glue flows through rustc's standard DropGlue path (post-Phase-E pattern); cascade discovery captures `<X as Drop>::drop` instances; `fill_extra_modules` emits Vale's bodies.

Rustc's default v0 mangler determines symbol names (single-symbol architecture).

### 20.7 Vale's CodegenBackend produces `.o` for the full reachable Vale universe (both binaries)

**valec-rs path** (concurrent with rustc's pipeline):
1. Before `start_async_codegen`, rustc's `codegen_crate` calls `backend.fill_extra_modules(tcx, allocator)`. Vale's hook walks Vale's codegen queue, asks allocator to mint rustc-owned `ModuleLlvm` per Vale CGU, wraps borrowed `LLVMContext` + `LLVMModule` in suppressed-Drop Inkwell-style handles, emits LLVM IR via the C++ Backend's borrowed-mode FFI (§5.1).
2. Rustc's partition runs; Vale's filter rebuilds the CGU list with consumer items removed.
3. `LlvmCodegenBackend::codegen_crate` processes the filtered partition; Vale's contributed CGUs ride the standard optimize → ThinLTO-summary → emit pipeline.
4. `join_codegen` + `link` pass through to inner LlvmCodegenBackend.

**valec path:**
1. valec's CLI orchestrator walks Vale's codegen queue.
2. C++ Backend invoked via owned-mode FFI (`backend_compile_program`); creates own LLVMContext + LLVMModule + TargetMachine.
3. Vale's optimize() + generateOutput() produces `.o`.
4. System linker (or Vale's forked linker code per Q66 H, deferred) links.

### 20.8 Output: rlib + cache (per-lib) + .o (per-lib non-generics) + binary

**Library compiles (valec-rs)**:
- `vmdparse.rlib` carrying Rust-side machinery + Vale's `fill_extra_modules` contribution (non-generic Vale bodies, cascade-discovered trait-impl methods).
- `vmdparse.vale-cache` sibling carrying Vale's typed AST.
- Generic Vale items NOT emitted at library compile (materialize only with concrete args downstream).

**Library compiles (valec)**: similar but without rustc; Vale codegen produces `.o` directly.

**Binary compiles**: emits Vale bodies for binary's own non-generic items + generic monomorphizations reached transitively. Links everything with upstream rlibs' bodies.

### 20.8.5 Cross-crate Vale generic monomorphization

Per Sky §20.8.5: cross-crate Vale generics monomorphize at the **binary's compile**, not at upstream's compile. Inherits Rust's downstream-substitutor model.

Timeline:
1. Cargo compiles `vmdparse` first. Produces `vmdparse.rlib` + `vmdparse.vale-cache`. No `.o` for `wrap<I32>` (no concrete T yet).
2. Cargo compiles `my_app`. Loads vmdparse's cache into Vale's universe. Sees `wrap<I32>(42i32)` in main.vale. Records "need to codegen wrap<I32>".
3. Vale's per_instance_mir fires at user-bin mono walk for `wrap<I32>`. Substitutes T=I32; produces synthetic body for collector.
4. Vale's codegen at my_app's compile emits LLVM IR for `wrap<I32>`. Substituted body. Symbol in my_app's `.o`.
5. Linker resolves.

Implication: binary's compile is heavy. Every generic Vale function the binary reaches needs Vale's frontend (substitute) + Vale's codegen (LLVM IR + llc). Vale libraries that are heavily generic produce small rlibs but contribute substantial work to downstream compiles.

---

## 21. Vale Library Distribution

Source-only distribution. No sidecar shipping. No pre-compiled bodies for user libraries (per the cache-not-sidecar decision in §7). Stdlib is the sole exception — precompiled per (target, mode) via valeup/rustup. Vale's registry (Q23) is v2; v1 supports path/git deps only. **This is Vale's second-largest divergence from Sky** (Sky originally shipped sidecars; the 2026-06-29 toylang migration brought Sky's model in line with Vale's intended model).

### 21.1 Path/git deps in v1

`vale.toml [vale-dependencies]` supports path and git deps only in v1:
```toml
[vale-dependencies]
vmdparse = { path = "../VmdParse" }
parseiter = { git = "https://github.com/verdagon/ParseIter", branch = "main" }
```

Cargo handles via its standard path-and-git mechanism. Reproducible builds work via `.vale-build/Cargo.lock` (cargo's standard lock-file).

No publish flow in v1. Library authors push to git; consumers add git URL.

### 21.2 Registry deferred to v2

Vale's own registry (`vale-registry` / `valeshare` / TBD name) reserved for v2 (Q23). v2 adds version syntax (`vmdparse = "1.0"`); both binaries use it; valec-rs additionally uses crates.io for Rust deps.

v1 closed-source / private libs: private git repos, internal filesystem mounts, git-over-SSH. Standard cargo-via-git pattern. No private-registry support until v2.

### 21.3 build.rs enforces toolchain presence

vale-stub-gen-generated `build.rs`:
```rust
fn main() {
    if std::env::var("VALE_TOOLCHAIN_ACTIVE").is_err() {
        eprintln!("ERROR: This crate is a Vale library and requires the Vale toolchain.");
        eprintln!("Install: https://vale-lang.org/install");
        std::process::exit(1);
    }
    // Verify rustc identifies as Vale's forked version (valec-rs only)
    // ...
    println!("cargo:rerun-if-changed=build.rs");
}
```

valec/valec-rs sets `VALE_TOOLCHAIN_ACTIVE=1` when invoking cargo. Pure-Rust users without Vale toolchain hit build-time error immediately, before runtime panic.

### 21.4 Pure-Rust consumers get a clear error

build.rs check is the safety net. Without it, vanilla rustc would compile stub rlib's `unreachable!()` bodies into real `panic!("unreachable")` code → runtime panic with no clue why.

Error at build time with "this requires Vale toolchain" message + install link. Recoverable.

### 21.5 What works without valec/valec-rs

Even without Vale toolchain:
- `cargo doc` on Vale libs (rustdoc reads stub source)
- IDE awareness (rust-analyzer reads stub rlib's Rust source)
- Crates.io publishing + search (when v2 registry lands)

What doesn't work: `cargo build` (build.rs errors), `cargo install` of binaries depending on Vale libs.

### 21.6 What requires valec/valec-rs

Full Vale toolchain required for:
- Compiling Vale-marked crates
- Compiling crates that transitively depend on Vale-marked crates

Transitive constraint: Rust crate depending on Vale lib makes its consumers require Vale toolchain. Standard ecosystem-split.

**Publishing implications for Vale library authors.** Vale libraries are **consumer-toolchain-forcing** — any Rust crate that transitively depends on a Vale library makes ALL its own downstream consumers require the Vale toolchain to build. This is a real ecosystem constraint that library authors should factor into publishing decisions:

- A Rust utility crate that adds a Vale library as an optional dependency (behind a feature flag) forces every user of that feature to install Vale.
- A framework crate that unconditionally depends on a Vale library forces every user of the framework to install Vale.
- v1 has no escape hatch — §21.7's opt-in precompiled bodies (v2) would let Vale libraries ship pre-compiled artifacts that vanilla-rustc users could link without the Vale toolchain, but that's deferred.

Practical guidance: authors of core / widely-used libraries should stay pure-Rust when possible; Vale-using libraries should be scoped to applications or opt-in adopter crates rather than pervasive dependencies. Standard cross-language ecosystem-split dynamics apply.

### 21.7 v2: opt-in precompiled bodies

v2 feature: opt-in precompiled bodies. Vale lib declares itself "Vale-pure" (no comptime, no advanced features requiring valec/valec-rs); valec publish precompiles bodies for common targets. Published cargo package contains Rust stub source + Vale source + cache + pre-compiled `.o` files per target + modified build.rs that detects vanilla-rustc compile and links pre-compiled `.o`.

Pure-Rust users could use the lib natively if Vale toolchain absent. Cost: complexity in publish flow, cross-platform fan-out, distribution size. Deferred until concrete need.

---

## 22. Incremental Compilation

### 22.1 Cargo's crate-level incremental

Cargo's standard machinery operates at the crate level:
- Per-crate fingerprint hash of inputs (source files, dep versions, profile settings, target triple, features).
- Fingerprint match → skip compile; reuse `.rlib` + `.vale-cache` from `target/deps/`.
- Mismatch → recompile that crate + invalidate downstream.

Per Vale's deterministic emission discipline (§7.6, §18.5), cargo's incremental works correctly. Single-file edit in Vale lib without affecting exports → only that lib's compile + binary invalidate. Export signature change → that lib + downstream consumers invalidate. Standard cargo behavior; Vale inherits.

### 22.2 Vale-internal fine-grained deferred

v1: each rustc subprocess walks Vale's full universe; codegens everything reachable. No "this item unchanged, skip its codegen" granularity within one rustc invocation.

v2 considers Vale-side fine-grained dep tracking (Sky §22.2 / toylang's L2 + L3 cache layers). Sky-side query system caching `(ValeItemId, args) → codegen output` per item. Changes invalidate downstream only. Non-trivial: Vale needs fingerprinting machinery, storage-efficient cached outputs, correct cross-item invalidation. v2 work; current toylang implementation defers per Decision 22.2.

### 22.3 Cache-write discipline: only at `after_rust_analysis`

**@CMWAR invariant**: cache writes route through `after_rust_analysis` only. Never from inside codegen-time callbacks (`consumer_fill_modules`, per_instance_mir provider invocations).

Why: writing from codegen-time callbacks would re-enter consumer state during rustc query providers, reintroducing the @GCMLZ deadlock vector. Even if surface-level "works" empirically, the structural risk is real (per toylang's two-write-sites cleanup empirical history). Discipline applied from day 1.

### 22.4 Perf model

**Architectural claim**: Vale's cross-crate boundary cost equals Rust's at every opt level. Vale's emitted bitcode lives in the same LTO module pool as Rust callers via `fill_extra_modules`; LLVM's IR inliner treats Vale exports the same as `#[inline]`-permitted Rust functions across a crate boundary. LTO ratios (1.5× for basic calls, 25-28× for drop chains) inherit from LLVM's universal cross-crate behavior; not Vale-specific. Vale's `fill_extra_modules` placement gives LLVM the same inlining opportunity Rust's does.

**Empirical basis — inherited from Sky, un-measured on Vale's C++ Backend as of this doc.** The parity numbers above come from Sky §22.4's Bench 1 / Bench 4b measurements on Sky's Rust Inkwell emitter. Vale's C++ Backend is structurally different — different IR emission code path (C++ vs Rust; different `Value*`/`Type*` API usage patterns), different intrinsic annotations, different attribute stamps and metadata attachment, potentially different IR shape hitting different inliner heuristics. LLVM's inliner is heuristic-driven and IR-shape-sensitive; the parity claim doesn't automatically transfer. Sky §F1 (`#[inline(never)]` on stubs silently blocking cross-language inlining for a long stretch until an audit caught it) is the empirical cautionary tale — a 1.5× perf shift caused by an emitter-detail bug in code paths prior reasoning had rationalized correct.

**Phase 5 bench-parity validation gate** (§28) upholds the parity claim on Vale's actual emitter before shipping. Vale runs analog of Sky Bench 1 (basic cross-language inline) and Bench 4b (drop-chain LTO cascade); verifies inner-loop disassembly-byte-parity for canonical scenarios. Parity failures surface as emitter-detail bugs (missing intrinsic annotations, wrong attribute stamps, IR shape mismatches, etc.) and get investigated per Sky §F1's shape — audit inliner-hitting attributes on emitted symbols, compare to rustc-emitted equivalents, fix the discrepancy. The parity target is upheld via measurement, not by inheritance.

**User-facing recommendation:** for release builds and any perf-sensitive testing, use `[profile.release] lto = "thin"`. Dev iteration uses cargo dev profile default (`lto = false`) where thin-local LTO still bridges intra-invocation CGUs but cross-crate Vale-body inlining is lost (per §5.5 Step 2 trade-off).

**v1 wrapper-attr posture**: Vale's `codegen_extern_wrapper` emits conservative ABI attrs (`sret`, `noalias`/`noundef`/`dereferenceable`/`align`); Phase P pattern (`deduced_param_attrs` returns `&[]` for Vale-tagged items) closes the silent-UB vector where rustc's MIR analysis on `unreachable!()` stub would wrongly infer `readonly` + `captures(none)`. v2 may revisit with path-b emission (Vale typechecker stamping ground-truth attrs at wrapper boundary based on explicit source-level mutability tracking) if profiling shows large-Vale-body workloads where the inliner doesn't fire.

### 22.4.1 Queries Vale touches and their cache policy

Audit per Sky §22.4.1: every Vale-overridden rustc query is cache-safe by construction. CI fence at `vale-rustc-glue/tests/cache_audit.rs` requires every override file to carry a `cache-audit:` marker comment describing its disk-cache safety story. New overrides MUST add a marker; test catches drift.

Queries Vale overrides:
- `per_instance_mir` — `cache_on_disk_if(false)`; per-compile re-derive.
- `layout_of` — never disk-cached.
- `cross_crate_inlinable` (local + extern) — never disk-cached.
- `collect_and_partition_mono_items` — `eval_always`; re-runs every compile.
- `deduced_param_attrs` — never disk-cached.

### 22.5 Determinism CI invariant

CI verifies deterministic build outputs:
1. Build project once. Hash all outputs (`.rlib`, `.vale-cache`, binary).
2. Wipe `target/`. Rebuild. Hash again.
3. Compare. Mismatch = regression.

Catches regressions in: vale-stub-gen workspace generation; Vale typing-pass output; Vale codegen; cache serialization. Without invariant, non-determinism accumulates silently until users notice.

---

## 23. Error Reporting and Diagnostics

### 23.1 Vale frontend errors in Vale terms, pointing at Vale source

Errors reported in Vale terms with file/line/column references into `.vale` files. Format mirrors rustc's well-known style (clarity, source highlighting, helpful suggestions):

```
error: type 'Widget' is not 'Send' as required by tokio.spawn
  --> src/main.vale:42:5
   |
42 |     tokio::spawn(make_widget())
   |     ^^^^^^^^^^^^^ requires F: Send + 'static
   |
   = note: Widget contains a Vale thread-local-allocator collection that's not Send
   = help: consider using Widget<GlobalAlloc> or Vale's own runtime
```

### 23.2 Rustc errors on stub rlib (rare; usually Vale frontend bug)

If rustc errors on stub rlib, that's almost always a vale-stub-gen bug. Vale's error wrapper decorates with: "This error is in vale-stub-gen-generated source; please file at [issue tracker]." Actual rustc error preserved for bug report.

### 23.3 Source position info in HinputsT

Every Vale item carries source position (file, line, column); file table maps indices to filenames relative to cargo package root. Enables cross-crate error messages, debugging (`.vale-cache` carries positions; DWARF references `.vale` source), IDE jump-to-definition across crate boundaries.

### 23.4 Source files shipping enables cross-crate error context

Published Vale libraries ship `.vale` source (§6.7). Cross-crate Vale error from published lib shows the lib's source at the relevant location, not just lib's name. Makes errors actionable.

### 23.5 Annotation skew detection

When Vale annotation files (§24) specify expected Rust signatures, Vale's typechecker cross-checks against rustc's actual signatures at typecheck. Mismatch → "annotation skew detected" error with both expected and actual signatures + suggestion to update.

Skew detection always-on for v1 (catches Cargo.lock version mismatches, annotation typos, stale annotations).

---

## 24. Annotations on Rust Deps

Sky §24's "sidecar annotations" reframed as **annotation files traveling with Rust crates**, not as part of Vale's own caches. Files describe Rust-API semantics Vale's frontend cannot infer from the signature alone.

### 24.1 What they are and what they cover

- Group effects of Rust methods (which groups they mutate, return references into)
- HRTB structure of complex Rust APIs (for v2 cases Vale's auto-translation can't handle)
- Outlives bounds that don't naturally translate to Vale's group hierarchy
- "drops_args" markers (tokio::select! drops losing branches; tokio::time::timeout drops inner future on timeout)
- Linearity propagation rules for Rust APIs
- Send/Sync overrides where Rust's auto-derive misclassifies

File location: `<crate>.vale-annotations.toml`, discovered automatically at cargo cache path next to the Rust crate. Can ship with the crate (in cargo `include`) or maintained out-of-band by Vale ecosystem (community-maintained registry for popular crates).

### 24.2 Primary source for binding info Vale's frontend can't infer

For Rust APIs Vale's frontend can infer correctly from the signature, no annotation needed. For Rust APIs requiring Vale-specific information (group effects, HRTB structure, drop semantics), annotation is source of truth. Missing annotation for needed property → Vale errors with "consider adding annotation" suggestion.

Format sketch:
```toml
[crate]
name = "tokio"
version = "1.32.0"

[[binding]]
path = "tokio::spawn"
bounds = ["F: Send + 'static", "F: Future"]

[[binding]]
path = "tokio::select"
drops_args = true
description = "Drops losing branches when one branch completes"

[[binding]]
path = "tokio::time::timeout"
drops_args = true
```

### 24.3 Cross-checked against rustc's actual signatures at typecheck

Per §23.5 skew detection. Always-on v1.

### 24.4 Per-Rust-crate annotation files

Each Rust crate with annotations gets its own file, tied to that crate's version. Per-crate granularity matches cargo's package model.

### 24.5 Discovery convention

1. For each Rust crate the Vale project depends on (per Cargo.lock), Vale checks cargo cache for annotation file.
2. If absent, Vale checks project-local override at `<project>/vale-annotations/<crate>.toml`.
3. Still absent → Vale fail-closes per §24.7: mut-effect default for missing group-effect annotations. Fail-closed at call sites and impl-body typecheck; users supply per-project annotations to unblock specific cases.

Project-local overrides let users add annotations for crates that don't ship them.

### 24.6 Use cases

- **HRTBs**: serde's `Visitor<'de>` pattern uses HRTBs Vale's auto-translation can't handle; annotation specifies binding manually.
- **Group effects**: Rust method that mutates a shared group, signature doesn't surface it.
- **Drop-cancellation**: `tokio::select` and similar drop args.
- **Complex bounds**: Rust APIs whose generic bounds don't naturally translate.

### 24.7 Missing-annotation default: mut-effect fail-closed at call sites

When Vale imports a Rust method or trait signature without Vale-side group-effect annotations, Vale defaults to **assuming mut effect on all group parameters**. Fail-closed at call sites and impl-body typecheck.

**Rationale.** Assume-no-mut would silently accept mutating operations as pure reads. Concretely, un-annotated `RefCell::borrow` (which semantically mutates `BorrowFlag`) would let Vale accept `fn read(&self) { self.refcell.borrow(); }` as a no-mut method, emit `unsafe impl Sync` for the containing type (§12.6), and let Rust farm `&Widget` across threads — concurrent Vale threads then race on `BorrowFlag`. Silent UB. Vale rejects this default categorically.

The mut-effect default fires uniformly across:

- **Missing method annotations**: individual Rust methods without a Vale-side effect declaration.
- **Missing trait signature annotations**: Rust traits imported without effect annotations on their methods.
- **Missing group-effect annotations on trait group parameters**: e.g., `trait Modify<g'> { func modify(&self, target: &T in g); }` with no declared effects on `g`.

**Practical implications:**

- **Vale stdlib** ships correct annotations for common Rust std types (`Cell`, `RefCell`, `Rc`, `Arc`, `Mutex`, `AtomicUsize`, `Vec`, `HashMap`, `Box`, `String`, `Iterator`, `Deref`, `AsRef`, etc.). Users of stdlib face no annotation friction.
- **Third-party crates**: users encounter fail-closed errors reactively — "method `foo` calls a mut-effect operation but is declared no-mut" — at call sites where the conservative default is wrong. Users then either supply a correct annotation in the project-local annotation dir (§24.5), restructure the calling Vale method to accept the mut-effect requirement, or contribute annotations to an ecosystem-maintained shared registry.
- Third-party ecosystem coverage grows over time via community annotation contributions; the assume-mut default is safe interim behavior while coverage builds.

**Alternatives considered and rejected:**

- **(a) Assume no mut effect on missing annotations**: silently unsound per the RefCell example above.
- **(c) Refuse imports without annotation**: fail-closed at import sites rather than call sites. Higher up-front cost — users can't even reference an unannotated method until annotations exist. More disruptive for third-party adoption. Vale chose (b) — the current mut-effect default — for softer developer experience while preserving fail-closed soundness.

See @MAMFC invariant (§26.21).

---

## 25. Risks

### 25.1 Category A: rustc_private lockdown, override_queries removal

**A1. `rustc_private` locked down.** <5% over 5 years. Impact: valec-rs architecture ends. Canary: deprecation warnings on `rustc_private`, RFCs proposing removal. Reaction: collaborate with rust-lang on unlocking specific surface, or migrate to replacement. Years of notice in any realistic scenario. **valec is unaffected** — doesn't depend on rustc.

**A2. `Config::override_queries` removed.** <5% over 5 years. Impact: valec-rs's query override layer collapses. Canary: rust-analyzer / miri public migration away. Reaction: redesign around replacement; weeks-to-months of rework per query.

**A3. Query system replaced.** <1% over 5 years. Impact: multi-month re-architecture. Canary: major rust-lang announcement. Reaction: rebuild; concepts transfer; specific hooks don't.

### 25.2 Category B: drift surfaces

**B1. Mono collector behavior drift.** 30-50% over 5 years. Impact: 1-3 weeks repair. Canary: deep-dep-graph tests fail with missing-symbol link errors. Reaction: read updated `rustc_monomorphize/src/collector.rs`, adapt Vale's body construction.

**B2. Partitioner restructure.** ~20-30% over 5 years. The B2 surface: rustc restructures `collect_and_partition_mono_items` such that Vale's filter-and-rebuild pattern no longer suppresses consumer-item emission. Drift modes: (a) partitioner adds new field to CodegenUnit Vale's rebuild loop doesn't copy; (b) call ordering shifts; (c) internal cache bypasses Vale's override output. Canary: link-time duplicate-symbol errors; runtime panics from inlined unreachable bodies. Reaction: 1-3 days repair.

**B3. MIR construction API drift.** 100% per 6-month bump. Sky baseline: ~1 hour to 1 week per bump. Vale inherits.

**B4. ABI helpers drift.** 15-25% over 5 years.

**B5. CGU lifetime erasure.** CLOSED architecturally per Sky §F.5 "don't stash, re-call" pattern.

**B6. Slab/comptime interaction with incremental cache.** ~30% over 5 years. Per-invocation slab + query cache interactions could produce non-determinism if slab is touched in incremental-cache-skippable paths. Canary: tests fail deterministically on warm runs but pass on cold. Reaction: move side-effects to up-front walks (analog of erw's `populate_X_instances_from_cgus`).

**B7. Comptime evaluator nondeterminism.** ~20% over 5 years. If a regression introduces HashMap iteration order into comptime output, reproducible-build invariant breaks. Canary: byte-comparison CI catches.

**B8. Debuginfo walker.** CLOSED architecturally by wrapper-as-field shape (§10.4.5).

**B9. LLVM-binding version skew.** CLOSED architecturally by Approach B (rustc owns LLVM resources; no parallel context construction).

**B10. LLVM 21 bitcode-writer / ABI-coerced extern calls.** Per Sky residual: ThinLTO's internal import phase still encodes/decodes bitcode under narrow shapes. Vale's emission must align extern decl signatures with call-site types per `@ACRTFDZ` to avoid triggering. Fixture-tested.

**B11. Round-trip workaround scaling.** CLOSED — no round-trip in Approach B.

**B22. Primitive-field accessor i1-storage soundness.** Inherits Sky B22. Vale's accessor codegen returns GEP pointer to field's original storage, not load-realloc roundtrip. Regression fixture required.

**B23. Type resolver IntLit widening.** Inherits Sky B23. Vale's type resolver widens unsuffixed integer literals to expected type when expected_ty calls for it. Regression fixture required.

**B24. Drop-glue shape stability post-mir_shims-elimination.** ~5-10% over 5 years. Impact: silent UB in destructor chains. Sky's empirical correction: with mir_shims retired, rustc's `build_drop_shim` is in Vale's load-bearing dependency surface. If rustc reshapes drop glue (iteration order, skip semantics), Vale's destructor sequencing silently breaks. Detection fence: sentinel fixtures exercising LIFO iteration over containers + nested-drop chains.

**B25. Default symbol mangling stability.** ~20-30% over 5 years (rustc has changed default manglers before — v0 introduced 2020). Impact: link errors (clean failure mode). With `symbol_name` override retired, Vale depends on rustc's default mangler producing matching symbol names at emission AND reference sites. If rustc changes default mangler, intra-build emission/reference still matches but cross-toolchain-version artifacts diverge. Detection fence: `vale-rustc-glue/tests/mangler_version_fence.rs` asserts emission paths use `tcx.symbol_name(...)`. Reaction: re-introduce override as delegating shim using previous-default mangler.

**B26. MonoItem/InstanceKind variant coverage.** ~15-20% over 5 years (`AsyncDropGlue` already exists; further variants probable). Impact: silent miscompile if new drop-flavored variant bypasses cascade discovery. Detection fence: `instance_kind_coverage_fence.rs` — compile-time exhaustive match over `MonoItem` + `InstanceKind` variants; rustc E0004 fires when new variant lands. Forces conscious decision per new variant.

**B27. Bench-detected creeping perf regression between nightly bumps.** ~30% over 5 years. Canary: re-run perf bench script after every bump. If Bench 1 LTO ratio regresses >10% or Bench 3 drop-chain ratio regresses >20%, investigate. Reaction: bisect upstream nightly range; report upstream or pin to last-known-good.

### 25.3 Category C: operational invariants

- **C1. Don't use def_path_str outside diagnostics.** `tcx.def_path_str()` ICEs outside diagnostic contexts. Vale's is_from_vale_stubs and all path-based matching uses `tcx.def_path(...)` walks or `tcx.crate_name` checks. Canary: panic messages mentioning `trimmed_def_paths`.
- **C2. Don't introduce new locking sites during codegen.** Vale's @GCMLZ analog. Mutable state held during codegen; query providers must not lock. Canary: tests hang with 0% CPU.
- **C3. Preserve codegen plugin's CGU filter invariant.** New providers must understand Vale items have been filtered. Canary: tests fail with "consumer item missing from CGU list."
- **C4. Vale's comptime evaluator must be deterministic.** Canary: byte-comparison CI.
- **C5. Cache must be deterministic.** Canary: byte-comparison CI.
- **C6. Cargo profile overrides only at workspace root.** Vale's CLI emits profile overrides only at generated workspace root Cargo.toml. Sky §F.10 lesson.
- **C7. `RUSTC_WORKSPACE_WRAPPER` necessity for valec-rs hook installation.** Direct `cargo build` invocations bypass wrapper; hook never installs; binary missing Vale bodies. Integration tests of patch (c) behavior MUST invoke through Vale's wrapper, not direct cargo.
- **C8. Stale incremental cache surfaces as mysterious test failures.** Rustc's incremental cache + Vale's universe pre-population can produce cache-shape mismatches when Vale's schema evolves. Build `valec clean` early.

### 25.3.5 The byte-identical pass-through invariant as continuous discipline

The hardest invariant in this document. Vale's `rustc`-wrapper, when compiling a crate without Vale marker, produces byte-identical output to vanilla nightly rustc for the same inputs. Maintaining requires continuous discipline:

- **Threat 1: side effects during Vale's startup before marker check.** Vale's startup reads only argv, does minimal Callbacks::config setup, gates every Vale-specific behavior on marker.
- **Threat 2: Vale's panic handler interfering with vanilla diagnostics.** Install panic handler only after marker detection.
- **Threat 3: Vale's `init()` / `provide()` methods leaking state.** Short-circuit when marker absent, leaving rustc identical to LlvmCodegenBackend output.

**The CI check.** Corpus of representative Rust crates (small hello-world, medium serde-derive consumer, large tokio program, generic-heavy code, trait-heavy code, sys-crate wrapper). Per crate: build vanilla → hash; build with valec-rs (marker absent) → hash; byte-compare. Mismatch blocks toolchain release.

### 25.3.6 The reasoning-chains-must-be-discounted-against-empirical-surprises discipline

Sky's calibration discipline inherited. Across Sky's implementation: multiple silent-correctness bugs surfaced in code paths prior reasoning had explicitly rationalized as correct. The pattern:

1. **mir_shims override never fired.** Round 3 reviewed as live machinery worth simplifying; Phase A fixture pass found override never matched any shipping test.
2. **B10 was Sky's emission bug, not LLVM's.** Original framing blamed LLVM; investigation found `push_arg_for_rust_call`'s Direct arm was emitting struct aggregates where rustc's ABI declared scalars.
3. **Bool accessor i1-storage.** Probe for Site #8 surfaced i1 upper-bit indeterminacy in load-realloc roundtrip. Same correctness pattern was sitting in rustc's source.
4. **IntLit widening on i64 zero stores.** Disassembly inspection of Bench 4's loop revealed `str wzr` for i64 fields; type resolver ignored `_expected_ty`.

In three of four cases, the bug existed where multi-agent reasoning + peer review + design-doc walkthrough explicitly confirmed correct. The CODE itself was wrong in ways that didn't appear in typed-AST view, design doc, or trait-level contract — only IR-level disassembly or runtime output exposed the divergence.

**The discipline:** future phases budget empirical-fixture work as load-bearing for catching premise errors, not just for confirming designed behavior. Build integration fixtures BEFORE the typechecker/codegen change those fixtures will validate. Use IR inspection (`llvm-dis`, disassembly, `cargo build -C save-temps`) as routine verification step. Discount round-N reasoning chains against rate of empirical surprises in prior rounds.

This discipline is why Vale phases empirical fixtures before typechecker/codegen changes; why architecture-doc rationale gets verified against code reality (per the §F.7-style audit pattern Vale will inherit); why bench numbers anchor architectural claims (per §22.4).

### 25.4 Mitigating factors

**Co-travelers.** valec-rs isn't alone in "deep rustc integration via nightly extension points" — erw, rust-analyzer, miri, clippy, cranelift, rust-gpu. If rustc API shifts threaten any, Vale has early warning. Monitor their issue trackers.

**`rustc_public` trajectory.** Stable-MIR effort covers ~40-50% of Vale's read-side rustc surface. Stabilization reduces drift surface meaningfully. Load-bearing pieces (query providers, MIR construction, CodegenBackend, partitioner) have no stable equivalent on roadmap; partial migration possible.

**Nightly-pin strategy.** Vale pins specific nightly. Bumping is conscious, not silent drift. ~6 months to ~3-month-old nightlies; dedicated bump sessions; full test suite cold and warm after each bump.

**valec is unaffected by rustc-internal drift.** valec doesn't link rustc internals; doesn't pin to rustc nightlies in the same way (only matches LLVM version per §3.6). Operational independence reduces blast radius of valec-rs-side breakage.

---

## 26. Cross-Cutting Invariants

Quick reference (the rules at a glance):

| ID | Rule |
|---|---|
| MINCZ | Symbol-name lookups are pure reads; drive codegen via `ReifyFnPointer` casts in `per_instance_mir` bodies. |
| GCMLZ | Don't lock a consumer-state mutex from inside a rustc query provider. |
| DPSFDOZ | `tcx.def_path_str` ICEs outside diagnostics; use `def_path(...)` or `crate_name`. |
| ELASZ | Populate lifetime slots of `GenericArgs` with `re_erased`, never `'static`. |
| ACRTFDZ | LLVM extern declarations use rustc's ABI-coerced types, not Vale's representation. |
| TCHAPZ | Append a hidden `Location` arg at call sites for `#[track_caller]` Rust fns. |
| MIGPROP | Migratory async cannot `.await` non-migratory. |
| NoPin | Vale source never writes Pin / `for<'a>` / Rust lifetime syntax. |
| RTMEIZ | Every Rust type Vale source uses must be explicitly `import`ed. |
| UTAIRZ | Unsized types appear only as the inner of a reference. |
| MBMRVZ | `fn main()`'s tail expression is void; otherwise SIGBUS on the sret. |
| IVTDBTZ | Inherent vs trait dispatch is type-kind based, not argument-count based. |
| TVIMDGAZ | For trait methods, build `Instance` from the trait def's method DefId + `[Self, ...]`. |
| ATAFLBZ | Walks of `tcx.all_impls(...)` filter by `is_from_vale_stubs(self_type_did)`. |
| ETASTZ | `build_generic_args_for_item` silently truncates excess Type args. |
| NNGZ | Non-generic is the degenerate case of generic; don't branch on `type_params.is_empty()`. |
| SMPLZ | Vale-emitted rustc-visible symbols must be pinned in `@llvm.used` so LTO `internalize` doesn't demote linkage. |
| **CMWAR** | **Cache writes route through `after_rust_analysis` only; never from codegen-time callbacks. Vale-specific.** |
| **CIDD** | **Stdlib's `unsafe` blocks gated by `comptime if __deterministic()` discipline. Vale-specific.** |
| **DRAFD** | **`#[may_dangle]` emission flows from Vale's `dangle` region annotation; no syntactic shape-scan. Vale-specific.** |
| **HBAB** | **Honest at Boundary, Always — no auto-trait claim reaches Rust unless Vale explicitly emitted it after real analysis. `ValeOpaqueType` wraps a `PhantomData<*mut ()> + PhantomPinned` marker composition so the wrapper's field-walk yields `!Send + !Sync + !Unpin` by default; positive claims require explicit stub_gen emission backed by Vale's field-walk (Send), universal policy + projection filter (Sync), or Movable analysis (Unpin). Rust → Vale imports honor the honesty framing: `&T where T:Sync` → no-mut, `&T where T:!Sync` and `&mut T` → mut. Vale-specific.** |
| **MAMFC** | **Missing-Annotation Mut-Fail-Closed — Vale defaults to assuming mut effect on missing group-effect annotations (Rust methods, trait signatures, trait group parameters). Fail-closed at call sites; prevents silent races from mis-annotation. Vale-specific.** |

### 26.1 MINCZ (Mangling Is Not Codegen)

Reading a symbol name for a Vale Instance via `tcx.symbol_name(instance)` is a pure read; doesn't drive codegen. To drive codegen of a generic Rust dep, emit a `ReifyFnPointer` cast in synthetic MIR. Two surfaces independent: symbol reads tell linker dispatch target; ReifyFnPointer tells rustc's collector what to codegen. Conflating misses dep registration.

### 26.2 GCMLZ (Generate Compile Mutex Lock)

If Vale uses a global mutex for any mutable consumer state, the mutex must not be locked from query-provider code paths during codegen. Vale's architecture structurally avoids the failure mode by keeping predicates as lock-free reads of the universe, making in-query callbacks stateless functions of `(tcx, instance)`, and using patch 4's `fill_extra_modules` hook for codegen contribution rather than long-running stateful callbacks holding the lock.

### 26.3 DPSFDOZ (DefPathStr Is For Diagnostics Only)

`tcx.def_path_str(def_id)` ICEs outside diagnostic contexts. Vale's path-based matching uses `tcx.def_path(def_id).data` walks or `tcx.crate_name(def_id.krate)` checks. `is_from_vale_stubs(tcx, def_id)` uses marker-detection.

### 26.4 ELASZ (Early-bound Lifetime Args Synthesized)

Lifetime slots in any `GenericArgs` for Rust items populated as `tcx.lifetimes.re_erased`. Vale source supplies type args; lifetime slots filled by Vale's helper based on item's `generics_of` declaration. `re_erased` over `'static` because trait dispatch can discriminate on lifetime; `re_erased` is rustc's neutral placeholder.

### 26.5 ACRTFDZ (ABI Coerced Return Type In Function Declarations)

LLVM extern declarations for Rust function calls use rustc's ABI-coerced types, not Vale's representation. 8-byte struct: rustc may return as `i64` Direct scalar; Vale's representation might be `[8 x i8]` aggregate. Declared LLVM function uses ABI-coerced type; return value reinterpreted via memory after call. ABI mismatch produces silent corruption (LLVM reads return value from wrong location).

### 26.6 TCHAPZ (Track Caller Hidden ABI Parameter)

`#[track_caller]` Rust stdlib methods get a hidden `&'static Location` parameter. Vale's call sites must pass a value (typically null). `instance.def.requires_caller_location(tcx)` detects the attribute; appends null pointer arg if so. Without hidden arg, called function reads garbage from slot.

### 26.7 MIGPROP (Migratory Propagation)

Migratory async cannot `.await` non-migratory. Vale's typechecker enforces. Without it, a migratory function could accidentally hold non-migratory state machine → send across threads while inner state holds non-Send group reference.

### 26.8 NoPin (Vale source = no Pin, no for<'a>, no Rust lifetime syntax)

Vale source never writes Pin, never writes `for<'a>` quantified lifetimes, never writes Rust lifetime annotations directly. Vale's group system covers what those handle in Rust. Vale's frontend translates between Vale source and Rust signatures at the boundary.

### 26.9 RTMEIZ (Rust Types Must Be Explicitly Imported)

Every Rust type Vale source uses — even transitively, even via types not named directly — must be explicitly imported. Stub_gen emits one `pub use` per import. Missing imports → structured errors at typecheck.

### 26.10 UTAIRZ (Unsized Types Appear Inside Ref)

Vale's unsized types (`str`, `[u8]`, slice-style `[T]`) appear only as inner of reference. Bare unsized types have no Vale representation. Caught at parser or type resolver.

### 26.11 MBMRVZ (Main Body Must Return Void)

Vale's `func main()` body must have void-typed tail expression. Otherwise auto-generated bin shim's no-sret call SIGBUSes on final `str` to sret buffer (writing to read-only page).

### 26.12 IVTDBTZ (Inherent vs Trait Dispatch By Type)

Vale's dispatch between inherent and trait static calls is type-kind based, not argument-count based. A name is a trait iff `find_use_imported_trait_def_id(tcx, name).is_some()`. Wrong classification → ICE or silently-wrong call.

### 26.13 TVIMDGAZ (Trait vs Impl Method DefId)

For Rust trait method calls, build rustc Instance from **trait def's** method DefId + `[Self, ...]` args, NOT impl block's method DefId. Wrong DefId → "type parameter X out of range" panic.

### 26.13.5 ATAFLBZ (All-impls Walks Need Vale-Stubs Filter)

Walks of `tcx.all_impls(trait_def_id)` return impls from every crate including std. Self-type-name check is ambiguous because std and Vale could both define a type named (e.g.) `Box`. Add `is_from_vale_stubs(tcx, adt_def.did())` filter inside impl walks. Under single-symbol architecture, wrong DefId produces wrong rustc-mangled name when Vale's bitcode emits a body.

### 26.14 ETASTZ (Extra Type Args Silently Truncated)

`build_generic_args_for_item` silently discards user-supplied type args exceeding item's Type slot count. Latent risk: when Vale gains syntax for naming non-default parent-type arg (custom allocator for Vec, etc.), silent truncation becomes real bug. Documented as tech debt; validate truncation at helper site.

### 26.15 NNGZ (Non-generic is Normal-case-of-Generic)

Source-level positive design principle (§1.5.5) elevated to arcanum form. Never branch on `type_params.is_empty()`. Forced exceptions annotated `arch-fence-allow:`. CI fence at `vale-frontend/tests/architecture_fence.rs` — AST-walking test that parses Vale's frontend source (via syn or rust-analyzer parser) and inspects the syntax tree for `type_params.is_empty()` patterns; unannotated occurrences fail. Not grep-based. **Land in Phase 0.**

### 26.16 SMPLZ (Vale Must Pin Linkage for External Refs)

Any Vale-emitted symbol whose only callers live in OTHER compile units' machine code must be pinned in `@llvm.used` LLVM global (not weaker `@llvm.compiler.used`). Three LLVM passes would otherwise remove/rewrite: `GlobalDCE` deletes; LTO `internalize` demotes linkage; linker dead-strips. Failure mode byte-identical to other link errors — disambiguating check: `llvm-objdump -t` on post-LTO `.o`. `g F` = chain intact; `l F` = SMPLZ discipline broken.

Detection fence: `vale-rustc-glue/tests/integration_projects/opt_level_3_fat_lto_smoke/`.

### 26.17 CMWAR (Cache-Must-Write-At-Rust-analysis)

**Vale-specific.** Cache writes route through `after_rust_analysis` only. Never from inside codegen-time callbacks. Sky's empirical history (two-write-sites cleanup) validates this as real deadlock-prevention discipline against @GCMLZ re-entry.

Detection fence: **structural enforcement, not a grep**. Cache-write functions are gated behind a marker/token type that only the `after_rust_analysis` callback constructs — types check at compile time that no other call site can invoke cache-write, since it's not possible to construct the required marker outside `after_rust_analysis`. Failed attempts fail Rust's own typecheck, not a separate CI grep pass. Same "make it structurally impossible" pattern as Sky's mutex-hierarchy invariants.

### 26.18 CIDD (Comptime If Deterministic Discipline)

**Vale-specific.** Every stdlib function callable from comptime must wrap each `unsafe { ... }` block in a `comptime if __deterministic() { /* safe path */ } else { /* unsafe path */ }` guard. Interpreter only ever sees safe Vale code; codegen prunes safe branch at runtime.

Enforcement is **Vale-typechecker analysis, not a CI grep pass**. Vale's typing pass computes comptime-reachability (transitive callee closure from any comptime block / const / `comptime let` binding) and flags any `unsafe { ... }` reached that isn't gated by `comptime if __deterministic()`. The typechecker's reachability analysis is the load-bearing mechanism; grep-based text-matching can't handle transitive closure and would either over-guard (require the gate on every stdlib unsafe) or under-guard (miss transitively-reachable-but-unmarked functions). Vale's typechecker already computes call-graph reachability for other analyses (comptime evaluation, drop discovery, etc.); CIDD reuses that infrastructure rather than grepping source.

### 26.19 DRAFD (Dangle-Region-Annotation-Flows-Drop)

**Vale-specific.** `#[may_dangle] G_as_re_erased_lifetime` emission on Vale-emitted Drop impls flows from Vale's `dangle` region annotation on the corresponding region/group. Soundness invariant lives in Vale's source-level type system; not in stub-gen's post-hoc syntactic shape-scan. Typing pass enforces that values from a `dangle`-annotated region are not accessed during the type's Drop. CI fence: dangle-annotation-vs-may_dangle-projection alignment test corpus.

### 26.20 HBAB (Honest at Boundary, Always — no Send/Sync/Unpin lie)

**Vale-specific.** **No auto-trait claim reaches Rust unless Vale explicitly emitted it after real analysis.** The wrapper design (`ValeOpaqueType<const T: u128>(PhantomData<*mut ()>, PhantomPinned)` per §10.6) makes every Vale-defined stub struct `!Send + !Sync + !Unpin` by rustc's own field-walk auto-derive — omission of a positive impl produces the correct negative property automatically, and no accidental Send/Sync/Unpin can leak from wrapper internals. Positive claims are then always explicit `unsafe impl` (or safe `impl` for Unpin) emissions, backed by three separate mechanisms:

- **Send** is auto-derived at the typing pass via field walking (Rust-style auto-trait, performed Vale-side); stub_gen emits `unsafe impl Send for T` when all fields aggregate to Send. Borrow-mentioning types are fail-closed non-Send (no emission; the freeze-window judgment for bounded sharing is Valen-internal). Allocator-generic types (`String<A: Allocator>`) provide cross-thread-transfer surfaces.
- **Sync** is emitted universally for all Vale-defined types by policy; stub_gen emits `unsafe impl Sync for T` for every Vale-defined struct/enum (subject to opt-out via `unsafe impl !Sync`). The claim is backed by Vale's group-effect enforcement (§12.6) — no-mut-effect `&T` methods can't do unsync mutation — plus the **projection filter** that requires Vale → Rust `&T` projections targeting `!Sync` types (per rustc's Sync trait — includes bare `dyn Trait`) to come from a mut Vale group, applied at return, argument, callback-invocation, and vtable-dispatch positions.
- **Unpin** is emitted per-type driven by Vale's Movable analysis (§12.3); stub_gen emits `impl Unpin for T` (safe impl; Unpin is auto-only) when Vale's typechecker concluded no self-referential state — trivially true for ordinary Vale structs (group system forbids self-mention), driven by `async(movable)` opt-in for async state machines.

Wrapper types with type parameters (closures, async state machines, `CancellableFuture<F, H>`) emit conditional impls that field-walk over the parameters, matching Rust's normal auto-trait propagation shape: `unsafe impl<F: Send, H: Send> Send for CancellableFuture<F, H> {}` (§14.7).

The Rust → Vale side honors the same principle via the honesty framing (§11.11): `&T where T:Sync` lifts to a Vale no-mut group; `&T where T:!Sync` and `&mut T` (any T) lift to Vale mut groups. Mut/no-mut Vale groups correspond one-to-one with the "may mutate, single-thread-visible" / "cannot mutate, safe to share cross-thread" distinction.

Cross-thread reference sharing uses standard `&T` projection — no wrapper mediates. Vale's Sync claim is stronger than Rust's own auto-derive would give (Vale-defined types with Cell/RefCell/Rc fields stay Sync-shareable at boundary), but honest under HBAB: Vale's group-effect enforcement + projection filter together back every claim, and the parallel-for demotion refinement (§12.6) keeps Vale-internal cross-thread sharing sound for Rust !Sync-origin data.

**Detection: two-part CI.** (1) A fixture asserts the `ValeOpaqueType` definition contains both `PhantomData<*mut ()>` and `PhantomPinned` — regressions to `PhantomData<()>` (or equivalent Send+Sync+Unpin-carrying content) fail CI. (2) Any `unsafe impl Send`, `unsafe impl Sync`, or `impl Unpin` synthesis in vale-stub-gen output, except where Vale's real enforcement validates it (Send: typing pass field walk; Sync: universal emission + projection filter; Unpin: Movable analysis), fails CI. Both checks structural, not grep-based.

### 26.21 MAMFC (Missing-Annotation Mut-effect Fail-Closed)

**Vale-specific.** When Vale imports a Rust method, trait signature, or trait group parameter without a Vale-side group-effect annotation, Vale defaults to assuming mut effect. Fail-closed at call sites and impl-body typecheck — see §24.7 for the full framing and the canonical `RefCell::borrow` failure-mode example.

Alternative "assume no mut effect" (option a) would silently accept mutating operations as pure reads, enabling races through un-annotated methods like `RefCell::borrow`. Vale rejects (a) categorically for soundness. Alternative "refuse imports without annotation" (option c) forces up-front annotation authoring at higher friction; Vale chose (b) — fail-closed at call sites, not import sites — for softer developer experience while preserving the safety property.

Detection: no CI fence needed for the default itself. Individual annotation-error messages surface directly to users at call sites where the conservative default triggers. Vale stdlib annotation coverage validated separately via the sunny-day test suite (all common Rust std types + methods have annotations covering their real group effects).

---

## 27. Compatibility Promises

### 27.1 Vale source compatibility across Vale versions (Q28 α)

Pre-1.0 makes no source-compat promises. Source can change between minor versions; users pin compiler version per project via `vale-toolchain.toml`. Best for iteration speed; matches Sky's pattern.

At 1.0, mechanism: editions (§27.x). Source files declare edition; valec/valec-rs reads edition and applies that edition's rules. Future major versions (Vale 2.x) may break source-level compatibility with new edition.

### 27.2 Cache format versioning (Q28 γ; collapsed)

Vale's cache (§7) format version strict match required. Pre-1.0: valec/valec-rs refuses to load caches with mismatched format_version. Hard error with "rebuild with matching valec version" hint.

Since Vale doesn't distribute caches between machines/versions (caches are local-only per §7), this collapses to a simpler concern than Sky's sidecar format versioning. Format migrations machinery deferred to 1.0+ if it ever matters; v1 just enforces match-or-error.

### 27.3 Cross-Vale-version binaries forbidden

A Vale binary cannot link object code produced by different Vale compiler versions. All crates in a binary's dep graph compile with the same Vale toolchain. Enforced via `vale-toolchain.toml` pinning; cargo's standard `rust-toolchain.toml` semantics + Vale's equivalent.

Why: Vale's codegen evolves; layouts may change; ABI emission may change; comptime semantics may change. Cross-version binaries would have inconsistent behavior.

### 27.4 Stdlib ABI evolution

Stdlib pins to compiler version. Each toolchain release bundles its stdlib. Breaking changes coordinated with compiler bumps.

For Vale 1.x: stdlib backward compat within major version (deprecation warnings, no source-breaking changes). For 2.x: opportunity to evolve aggressively if needed; source migrations tooling-supported via `valec migrate` command.

### 27.5 Editions from v1 (Q28 ε)

`vale.toml [project] edition = "..."` field exists from v1. Pre-1.0 uses `edition = "experimental"` (signals "no compat promise"). Future editions `"2026"`, etc. ship with 1.0+ and lock that edition's source semantics. Pre-1.0 projects migrate to 1.0 edition as part of upgrade.

Sidecar header includes `format_version: u32` per §7.2. Mismatch on load → hard error.

### 27.6 Toolchain bump policy

Both binaries advance LLVM together when rustc nightly does (per Q3 lockstep). Vale toolchain bumps every ~6 months tracking rustc nightly. Per §3.5 procedure. Per-bump cost ~1.5-2 weeks focused engineering.

### 27.7 Deprecation warnings (1.0+)

Pre-1.0: breaking changes ship as breaking changes; users update source. Post-1.0: deprecations carry warnings + migration paths. CI runs both binaries against pinned-toolchain corpus to catch toolchain-mismatch regressions before users hit them.

---

## 28. Implementation Phasing

**Honest timeline: 3-5 years to 1.0 for a small team.** Validation pass synthesis. Vale's "no shortcuts" posture (§1.5) accepts this.

Phases below are nominal; many can parallelize. Phase 0 establishes groundwork that Phases 1-6 build on; Phases 1-2 are largely independent and can proceed in parallel; Phases 3-6 form a chain.

### 28.1 What v1 ships

**Phase 0 — Foundational decisions + groundwork (~12-24 months single engineer, ~8-12 months small team).** (Sizing updated from earlier estimate to reflect the cascade of additions during the arch-review pass: GlobalState refactor, arena ownership migration, bench-parity fixture work, expanded CI-fence enforcement work as typechecker analysis rather than grep. Phase 0's substantial scope reflects Vale's "no shortcuts" stance — foundational work up front, downstream phases build on stable substrate.)
- Resolve the ~18 Phase 0 readiness items from the validation pass.
- Land the architecture doc (this document) as the source of truth.
- Free deletions (no-op cleanup): retire dead `Options.mode`, collapse dual-CLI-parsing, decide vestigial intern modes.
- C++ Backend symbol audit (Phase 0 task §5.6): `__vale_` prefix all runtime globals; rename `main` mandatory.
- HinputsT renaming from "Hinputs" → "Temputs" terminology where applicable.
- HashMap→IndexMap audit in `HinputsT`.
- Five CI fences land from day 1 (pass-through corpus, architecture fence per @NNGZ, cache audit fence, cross-language inlining matrix, determinism gate).
- `cache_audit.rs` fence for query-override cache-on-disk policy.
- `mangler_version_fence.rs` and `instance_kind_coverage_fence.rs` per §25.2 B25/B26.
- Test harness migration off `name=path` CLI to `Manifest::from_synthetic`-style API.
- Add `serde` + `bincode` to workspace as baseline.
- Reserve `rust`/`self`/`crate`/`super` as illegal Vale project names.
- Architecture-fence + cache-audit + cleanup-audit fences active from day 1.
- C++ Backend portage: LLVM 16 → rustc's pinned-nightly LLVM (~21).
- C++ Backend `GlobalState` refactor: single-invocation lifetime → per-FFI-call instantiation (§5.1). Enables concurrent-CGU emission under `fill_extra_modules`; directionally aligns with Vale-project goal of making `GlobalState` less global. Real engineering task (~days), not a tweak.
- **Arena ownership migration to Session-scoped storage**: refactor Vale frontend's arena model from the current shape to Session-scoped ownership. Ouroboros/yoke self-referential vs collapse-to-one-lifetime decision made as part of doing this work (not a separate pre-commit gate). Must land before Phase 3's LangCallbacks integration — everything downstream assumes a stable arena model rather than concurrently re-architecting.
- Empirical fixture work for §25.3.6 calibration discipline.

**Phase 1 — Manifest + dual-mode CLI scaffolding (~2-3 months).**
- `vale.toml` schema + parser. Port toylangc's manifest.rs as seed.
- `.vale-build/` workspace generation (deterministic).
- Mode-gating: `#[cfg(rust_interop)]` parse-time exclusion over binary cfg expressions.
- `valec` CLI orchestrator (standalone path).
- `valec-rs` CLI orchestrator (argv-dispatched: build / rustc-wrapper modes).

**Phase 2 — Stub rlib emission + cache machinery (~3-6 months, parallel-track with Phase 1).**
- `vale-stub-gen` generation of Rust `lib.rs` from HinputsT.
- `__VALE_STUBS_MARKER` activation + marker-detection.
- Cache (§7): 7-axis Merkle digest, eager producer-side write, transitive Merkle invalidation, hard-error policy, determinism CI gate.
- `valec inspect <cache>` subcommand for debugging.

**Phase 3 — valec-rs binary + LangCallbacks impl (~2-3 months; depends on Phase 1).**
- `frontend_rust_rustc` crate scaffolding.
- LangCallbacks impl (parsing argv, invoking Vale's frontend at `after_expansion`, installing query overrides).
- Per-crate-marker activation.
- (Arena ownership migration previously listed here has moved to Phase 0 groundwork — Phase 3 assumes a stable arena model rather than concurrently re-architecting one.)

**Phase 4 — per_instance_mir + instantiator integration (~3-6 months; depends on Phase 3).**
- per_instance_mir provider implementation.
- Vale-side substitution engine.
- ReifyFnPointer cast emission for Rust deps.
- IdI ↔ DefId bridge.
- Sunny-karp typed_bodies cache.
- Partial-evaluation engine integrated with per-Instance substitution (§13.7).
- Cascade discovery for case 4/6 (in-process drain pattern).

**Phase 5 — C++ Backend borrowed-mode + LLVM port (~3-6 months; depends on Phase 4).**
- C++ Backend borrowed-mode FFI (§5.1).
- `fill_extra_modules` hook installation + handling.
- `partition_filter` override.
- `cross_crate_inlinable` + `deduced_param_attrs` overrides.
- LLVM bumps absorbed in dedicated commits per §27.6.
- **Bench-parity validation gate (§22.4)**: run analog of Sky Bench 1 (basic cross-language inline) and Bench 4b (drop-chain LTO cascade) against Vale's C++ Backend output; verify inner-loop disassembly-byte-parity. **Ship-blocking gate** — parity failures require investigation per Sky §F1's shape (audit inliner-hitting attributes on emitted symbols, compare to rustc-emitted equivalents, fix discrepancies).

**Phase 6 — Feature parity / edge cases (~3-6 months; depends on Phase 5).**
- Closure auto-impl Fn/FnMut/FnOnce (§14.1-§14.2).
- Async two-type split + state machine codegen (§14).
- `dangle` annotation flow to `#[may_dangle]` (§11.10).
- HRTB auto-translation (§11.8); annotation-file mechanism (§24).
- C-extern path post-Linear-retirement (Q27 design).
- Replay rework (if pursued — Q12 narrowed scope to valec only).

**Phase 7 — Comptime engine (~6-12 months; can parallel with Phase 5-6).**
- Slab evaluator (tree-walking interpreter; instruction-count budget enforcement).
- Comptime reflection intrinsics (`fields_of`, etc.).
- `comptime for` / `comptime map` / `comptime fold` language constructs.
- Code synthesis pipeline (synthesis functions produce expression trees spliced via partial-evaluation).
- `comptime if __deterministic()` typechecker support.
- `include_file!`-style at parse/lex time.
- Determinism CI fixtures for comptime output.

**Phase 8 — Async runtime + tokio interop (~6-12 months; can parallel with Phase 7).**
- Vale runtime: executor, channels, waker integration, sync primitives.
- `race`/`select` Vale-native primitives (channel-based cancellation).
- `into_cancellable` wrapper.
- Migratory propagation rules.
- Pin handling at boundary.
- Allocator integration: global for migratory; thread-local for default (Q49 lock).

**Phase 9 — Vale stdlib bootstrap + delivery (~4-6 months).**
- Multi-stage bootstrap (stage-0 minimal subset; stage-N self-host).
- Pre-compiled stdlib distribution per (target, mode) via valeup/rustup.
- `cfg`-equivalent items in stdlib.
- Derive synthesis functions (`std.derives.clone`, `std.derives.hash`, etc.).

**Phase 10 — Distribution + tooling (~3-4 months).**
- `valeup` installer mechanics.
- Custom rustup distribution server for `valec-rs`.
- `valec publish` (deferred to v2 per Q23).
- Architecture-doc draft (this doc) → 1.0 with the lock dates.
- LSP integration (`valec lsp` mode; same binary, two modes).
- Cross-platform builds (Linux x86_64 + aarch64; macOS x86_64 + aarch64; Windows x86_64).

**Total v1 estimated effort: ~50-80 weeks single engineer, OR ~3-5 years small team to 1.0.** This is a multi-year project at any reasonable team size; consistent with Vale's "no shortcuts" framing.

### 28.2 What's deferred to v2

- Fine-grained Vale-side incremental compilation (§22.2 / L2-L3 caches).
- Cancellable futures with async cleanup handlers (§15.3).
- Opt-in precompiled bodies for Rust-compatible Vale libs (§21.7).
- Vale-native registry (§29.4) vs crates.io.
- Unified `spawn_blocking` API (§17.5).
- HRTBs for lifetime-discriminating dispatch and nested HRTBs (§11.9).
- Vale source-level editions (§27.5).
- Cross-Vale-version binary support via cache migration (§27.2-§27.3).
- Compile-time-error tier for linear types (§F.22 / `#[ultra_strict]` analog).
- L3 on-disk cache for per-Instance results.
- Multithreaded comptime evaluation.
- Comptime-execution of Rust code (currently only Vale runs at comptime).
- `Any`-equivalent runtime reflection (Q39 deferred).
- v2 path-b ABI-attr emission (§22.4).

### 28.3 What's deferred to Vale 1.0

v1.0 represents Vale's first stable release. Pre-1.0 versions are pre-release; breaking changes allowed between minor versions. At 1.0:
- Source language frozen per editions.
- Cache format frozen per format_versions, with migration support if/when needed.
- Vale stdlib's surface frozen.
- Compatibility promises kick in.

1.0 gated on confidence the architecture is right. Signals: real Vale projects running in production for months; no major architectural surprises encountered; clear v2 roadmap.

### 28.4 Long-term: upstream contributions to rustc

Background efforts, NOT on Vale's critical path:
- RFC for arbitrary-typed const generics (§4.3 path 3, primary).
- Engage with rust-lang's per_instance_mir-related discussions.
- Contribute `fill_extra_modules` hook upstream (most upstreamable patch; benefits cranelift, gcc-rs, spirv, any backend wanting to contribute compiled modules).
- Stable-MIR migration where applicable.

These reduce Vale's long-term fork maintenance when they land. Multi-year timeline.

---

## 29. Open Questions and Future Work

### 29.1 HRTBs: advanced cases

Two HRTB-related cases deferred to v2: lifetime-discriminating impl dispatch (`impl Foo for Bar<'static>` vs `impl<'a> Foo for Bar<'a>` with different behavior) and nested HRTBs (`for<'a> Trait<for<'b: 'a> InnerTrait<'a, 'b>>`). v1 forbids Vale source from invoking such APIs through paths hitting them; users work around via thin Rust shim crates or annotation files. v2 considers syntax for explicit lifetime-path commitment + annotation-format extension for nested binders.

### 29.2 Async cleanup handlers

v1 has sync cleanup (`FnOnce()`). v2 may add async handlers for cases requiring async work during cancellation (graceful TCP close, distributed transaction commit/abort). Resolution criteria: concrete use cases justifying complexity.

### 29.3 Vale-internal fine-grained incremental

v1 has crate-level incremental via cargo. v2 may add per-item incremental via Vale-side query system (L2 + L3 cache layers per toylang's framing). Mechanism non-trivial: Vale needs fingerprinting machinery, storage-efficient cached outputs, correct cross-item invalidation. Resolution criteria: real-size projects make compile times user pain point.

### 29.4 Vale's own registry (vs crates.io)

v1 uses path/git deps only (Q23). v2 may add Vale-native registry. Resolution criteria: Vale outgrows crates.io's affordances (Vale needs metadata cargo doesn't carry; Vale wants stricter version semantics).

### 29.5 Standard library design

**Locked**: hybrid model. Vale stdlib's allocator-generic collections wrap their Rust std equivalents under valec-rs at runtime, with allocator parameter passing straight through (§12.1) — `Vec<T, A>`, `HashMap<K, V, A>`, `Box<T, A>` use direct wrap via the established `allocator_api` nightly feature. `String<A>` uses per-instantiation backing selection via `comptime let` (§12.1) — wraps `rust.std.string.String` when A=GlobalAlloc, wraps `Vec<u8, A>` otherwise — because Rust's upstream `String<A>` (rust-lang/rust#149328) hasn't landed yet. Under valec and at comptime in either binary, all of these resolve to pure-Vale impls of the same allocator-generic surface, selected via `#[cfg]` (§3.3).

What's still open inside stdlib design: which Vale-specific types live entirely Vale-native (the runtime executor primitives, comptime support helpers, the `Str = Rc<String>` companion, etc.) vs which are thin wrappers over Rust ecosystem types. Resolution criteria: practical experience building stdlib reveals what works ergonomically per-type.

### 29.6 Fork-reduction trajectory

Vale's fork is 4 patches (§4.2). Long-term goal: upstream landing reduces fork. Most upstreamable: patch 4 (`fill_extra_modules`); benefits cranelift, gcc-rs, spirv. Less upstreamable: per_instance_mir trio; requires multi-year RFC work via `adt_const_params` extension path (§4.3).

Resolution criteria: rust-lang lands stable extension point replacing per_instance_mir, or Vale's RFCs gain traction. If both upstream, Vale's fork shrinks to zero. Multi-year arc; not on critical path.

### 29.7 v2 precompiled bodies for Rust-compatible Vale libs

Per §21.7: Vale libs that are Vale-pure (no comptime, no advanced features) could opt into shipping precompiled bodies for common targets. Published cargo package contains source + cache + pre-compiled `.o` + modified build.rs detecting vanilla-rustc and linking pre-compiled bodies. Pure-Rust users use the lib natively.

Deferred v2; design space catalogued; concrete need triggers implementation.

### 29.8 Sub-questions deferred from Q-session as syntax

- Heap-opt syntax (`^MyStruct` vs `Box<MyStruct>` vs other) (Q50)
- Associated type syntax (`type Item;` vs `assoc Item;` vs other) (Q57)
- `__deterministic()` intrinsic naming
- Tool-attribute namespace (`#[vale::*]` likely)
- File-vs-project scope of sealed interface closure (Q46 sub)
- `#[derive(...)]` desugaring details — does it produce just a function, or also an impl block? (Q64 deferred)

These don't affect architecture; resolved during implementation of the relevant subsystem.

---

## 29b. no_std and Embedded Posture

### 29b.1 v1: not supported

Vale v1 does not target no_std. Vale's runtime (executor, channels, allocator) is heavy-weight and assumes hosted environment with file I/O, threading, heap allocator. Targeting embedded MCU without heap or threads = out of scope v1.

### 29b.2 v2+: opt-in `#![no_std]`-equivalent

v2 feature: Vale source can opt into a "Vale core" subset that doesn't require Vale's runtime. Subset includes:
- Basic types (integers, bools, fixed-size arrays).
- Functions and structs without async, channels, or runtime-dependent features.
- Minimal allocator interface that embedded application provides.
- Static memory regions in place of runtime-allocated groups.

Approximately Rust `core` + `alloc`-without-allocator scope. Vale source under subset compiles to embedded target without Vale runtime.

### 29b.3 v2+: bare-metal target support

Targeting bare-metal triples (`thumbv7em-none-eabi`, etc.) conditionally supported once Vale core subset exists. Codegen accepts target triple; emitted code respects target-specific calling conventions; typechecker unchanged. Runtime support library NOT available on bare-metal; source must be self-contained within core subset.

### 29b.4 Posture vs Rust embedded

Rust's embedded ecosystem (`#![no_std]`, embedded-hal) is mature. Vale's posture: conservative — don't compete until Vale has real story. v1 stays away; v2 minimum viable; v3+ expands based on user demand. Architectural decisions don't preclude embedded; they just don't prioritize it.

---

## 30. Glossary

**Group [Vale]** — A compile-time set of possibly-aliasing places. **Not a lifetime**: it plays the boundary role Rust lifetimes play at projection (erased to `re_erased`), but safety comes from flow-sensitive invalidation/poisoning, not borrow scopes — see the Valen language reference's "Groups are not lifetimes." Carries modifiers (`imm`, `rc`, `dangle`, `runtime`, etc.).

**Linear type [Vale]** — Type whose values must be explicitly consumed; cannot be silently dropped. Per `valen-design-1.md`, a type is linear when it defines a `drop` (auto-run at scope end) or has by-move `self`-consumers with no `drop` (linear-strict — must be consumed on every path). Vale's typechecker enforces consumption; at the Rust boundary a linear-strict type gets a synthesized panic+abort Drop shim so a Rust-side drop aborts.

**Comptime [Vale, after Zig]** — Vale's compile-time evaluation. Same expression language as runtime; slab-based representation; per-Instance partial-evaluation at per_instance_mir time.

**Slab [Vale]** — Vale's compile-time RAM-simulation. Comptime values allocated in slab; references are integer offsets. Per-rustc-invocation; never serialized.

**Migratory [Vale]** — Property of async fns: future sendable across threads, movable, cannot hold borrows across `.await`. `migratory` keyword.

**Cancellable [Vale]** — Property of futures via `into_cancellable` wrapping: future can be dropped while executing; user-supplied cleanup handler runs on drop.

**Stub rlib [Vale]** — vale-stub-gen-generated Rust crate (rlib) containing Rust-source declarations of every exported Vale item. Compiled by rustc as ordinary Rust; `collect_and_partition_mono_items` filter removes consumer items before LLVM codegen; `fill_extra_modules` emits real `External`-linkage bodies. Only valec-rs generates stub rlibs.

**Cache (not sidecar) [Vale]** — Local on-disk file at `target/<triple>/<profile>/deps/lib<crate>-<hash>.vale-cache`. Sibling of cargo's `.rlib`/`.rmeta`. Carries serialized HinputsT. **Local only** — never crosses machines or distribution. Vale's analog of Sky's pre-migration sidecar; Vale adopts cache-not-sidecar model from day 1.

**HinputsT [Vale]** — Vale's typing-pass output. In-memory typed AST. Serialized to cache for local use; never distributed.

**Marker [Vale]** — `pub const __VALE_STUBS_MARKER: () = ();` declaration at root of every Vale-generated stub rlib. valec-rs checks for marker at crate-load time to decide whether to activate Vale's machinery.

**Typeid [Vale]** — Content-addressed u128 identifying a Vale-side type. Used in ValeOpaqueType wrapper to project Vale-side types onto rustc-visible territory without naming the type directly.

**ValeOpaqueType<const T: u128> [Vale]** — Universal wrapper type in Vale's stdlib. Used to represent Vale-side types rustc shouldn't know about by name. u128 from day 1.

**Per_instance_mir [Vale, inherited from Sky]** — Vale's custom rustc query. Instance-keyed; provider returns synthetic MIR body. Added via three of Vale's four fork patches.

**Approach A** — Instance-keyed dep discovery; Vale substitutes args itself. Used by Vale.

**Interleaving** — Vale's compiler hooks fire during rustc's monomorphization phase, supplying per-Instance information as the collector encounters concrete Instances. Opposite of pre-pass and post-pass.

**Pre-pass** — Hypothetical alternative where Vale enumerates all required Rust monomorphizations before rustc starts. Insufficient for Vale's interop cases.

**re_erased** — Rustc's lifetime placeholder for post-borrowck lifetimes. Vale's groups erase to re_erased at boundary per @ELASZ.

**HRTB** — Higher-Ranked Trait Bound. `for<'a> Trait<&'a T>`. Vale source never writes; frontend handles auto-translation from groups.

**`.vale-build/`** — vale-stub-gen-generated cargo workspace directory. Contains stub rlibs + bin shim. Cargo operates on this.

**Marker-detection** — Vale's mechanism for "is this a Vale stub rlib?" Walks crate root for `__VALE_STUBS_MARKER` with DefId-parentage check.

**Forked rustc** — valec-rs's rustc binary, statically linking codegen backend + frontend + four fork patches.

**`valec`** — Standalone Vale compiler binary. ~40-100MB. No rustc internals. Uses C++ Backend + bundled libLLVM.

**`valec-rs`** — Rust-interop Vale compiler binary. ~2GB. Bundles forked rustc + Vale's frontend + frontend_rust_rustc + C++ Backend + libLLVM. Argv-dispatched: orchestrator and rustc-wrapper modes.

**`valeup`** — Custom installer for valec (Vale-controlled distribution; no rustup needed).

**`dangle` annotation [Vale]** — Annotation on a **specific function's** group parameter (usually `drop`) (Q65): a verified no-dereference promise that propagates through the call graph. *(The type-level form — `dangle` on a type — was cut 2026-07-04; it is per-function only.)* For Drop specifically, stub_gen projects it to `#[may_dangle]` at the boundary. Full semantics in `valen-design-1.md` ("Stored references and poisoning").

**`runtime` annotation [Vale]** — Multi group modifier (formerly `noncaring`): the type's references into that multi are exclusively strong or weak (never borrows), verified by field-walk, so multi mutation never poisons it. Defined in `valen-design-2.md`; no direct boundary projection of its own.

**`comptime if __deterministic()` [Vale]** — Stdlib pattern (Q44 / @CIDD) gating `unsafe` blocks. Interpreter sees only safe branch; codegen prunes safe branch at runtime.

**Partial-evaluation [Vale]** — Per-Instance evaluation of comptime-known calls at per_instance_mir time (§13.7). Calls whose args are all comptime-known evaluate inline; results splice into per-Instance body.

**Content-hash const args [Vale, from Sky §29.A.content-hash-const-args]** — Comptime values crossing into rustc-visible territory surface as `ConstKind::Value(u128_hash)`, not as slab-pointer-as-u64. Adopted from day 1.

**Edition [Vale, adopted from Rust]** — Source compatibility evolution mechanism. Pre-1.0 uses `"experimental"`; 1.0+ uses `"2026"` etc.

**`build.rs`** — Skyc-style script enforcing Vale toolchain presence. Pure-Rust users get build-time error.

**`<crate>.vale-cache`** — Vale's cache file extension. Local-only.

**`<crate>.vale-annotations.toml`** — Per-Rust-crate annotation file (§24).

**v1 / v2** — Vale version. v1 first usable release; v2 adds features not blocking initial usability. Vale 1.0 first stable release.

---

## Appendices

### Appendix A. Worked Examples

End-to-end walked examples for the 7-case taxonomy live inline in §2. For artifacts each example produces:

- Stub rlib content (Rust declarations, opaque types, marker, impl blocks): §6 + §10
- Cache content (typed AST, typeids, item bodies): §7.2 + §8
- per_instance_mir synthetic body shape (ReifyFnPointer casts for Rust deps): §19.3
- Drop glue + linear-type panic body: §15.7
- Migratory async + Future impl shape: §14.10
- Auto-emitted Drop impl with `#[may_dangle]`: §11.10 + §15.7

### Appendix B. Reference: Fork Patches

The four patches Vale maintains against vanilla nightly rustc, only when building valec-rs.

#### B.1 per_instance_mir query declaration

`compiler/rustc_middle/src/query/mod.rs`:
```rust
query per_instance_mir(key: ty::Instance<'tcx>) -> Option<&'tcx mir::Body<'tcx>> {
    desc { "computing per-Instance MIR for {:?}", key }
    cache_on_disk_if { false }
}
```

#### B.2 Collector calls per_instance_mir

`compiler/rustc_monomorphize/src/collector.rs::collect_items_of_instance`:
```rust
let body = tcx.per_instance_mir(instance)
    .unwrap_or_else(|| tcx.instance_mir(instance.def));
```

#### B.3 Default provider returns None

`compiler/rustc_mir_transform/src/lib.rs::provide`:
```rust
providers.per_instance_mir = |_tcx, _instance| None;
```

#### B.4 `fill_extra_modules` allocator-callback hook (rev 3 `#[repr(C)]`)

Per §4.2 + Sky §3.2 patch 4 detail. `ExtraBackendMethods::fill_extra_modules(tcx, allocator)` + `ExtraModuleAllocator<M>` `#[repr(C)]` struct. ~210 LOC across 5 files. Default-no-op; LLVM backend overrides to consult process-global `OnceLock<FillExtraModulesHook>`.

#### B.5 Future patch sites

Vale's fork is locked at 4 patches. Additions require explicit justification + signoff. §25 catalogs risks that might warrant new patches; §25.2 B5 + B8 + B22 examples are addressable architecturally without new patches.

### Appendix C. Reference: Vale Codegen Backend Methods

#### C.1 `init`, `provide`, `codegen_crate`, `join_codegen`, `link`

```rust
impl CodegenBackend for ValeCodegenBackend {
    fn name(&self) -> &'static str { "vale" }

    fn init(&self, sess: &Session) {
        self.inner.init(sess);
        if has_vale_marker(sess) {
            vale_runtime_init(sess);
        }
    }

    fn provide(&self, providers: &mut Providers) {
        self.inner.provide(providers);
        if let Some(sess) = current_session_with_marker() {
            vale_install_query_overrides(providers);
        }
    }

    fn codegen_crate(&self, tcx: TyCtxt<'_>) -> Box<dyn Any> {
        self.inner.codegen_crate(tcx)
    }

    fn join_codegen(&self, ongoing: Box<dyn Any>, sess: &Session, outputs: &OutputFilenames)
        -> (CodegenResults, FxIndexMap<WorkProductId, WorkProduct>)
    {
        self.inner.join_codegen(ongoing, sess, outputs)
    }

    fn link(&self, sess: &Session, codegen_results: CodegenResults,
            metadata: EncodedMetadata, outputs: &OutputFilenames)
    {
        self.inner.link(sess, codegen_results, metadata, outputs)
    }
}
```

#### C.2 Consumer item suppression: partition filter

Vale overrides `collect_and_partition_mono_items` via `Config::override_queries`. Delegates to default partitioner; walks each CGU rebuilding with consumer-defined items removed (`is_consumer_codegen_target` returns true → skip).

#### C.3 Cross-platform considerations

Vale's codegen produces target-specific LLVM IR. Target triple from `tcx.sess.target`. Cross-compile works because Vale reads target from rustc's session, not host-system-dependent source.

#### C.4 Shipping patch 4 shape: rustc-owns-lends (Approach B, rev 3)

Rustc allocates each per-CGU LLVMContext + LLVMModule via standard `ModuleLlvm::new(tcx, name)` and lends borrowed pointers via `ExtraModuleAllocator<M>` callback. Vale's C++ Backend takes the borrowed handles via `backend_compile_program_into` (§5.1). No bitcode serialization, no `parse_from_tcx`, no context migration; rustc retains ownership.

### Appendix D. Reference: HinputsT in-memory shape

Full structural detail: `typing-pass-design-v3.md`. Interop-relevant pieces summarized in §8. Cache serialization format detailed in §7.2.

### Appendix E. Vale Source Examples for Each Major Feature

Per-feature Vale source examples inlined in relevant chapters:
- Groups: §11
- Linear types and drop: §15
- Comptime: §13
- Async migratory/cancellable: §14
- Derive sugar: §13.10
- Region with `dangle`: §11.10
- Two-binary cfg gating: §3.3

### Appendix F. Lessons inherited from toylang's prototype implementation

Empirical findings from Sky/toylang's 2026-06-25 architecture document `~/Harmonious/rust-interop-architecture.md` Appendix F. Vale starts from the post-retirement state — Option 4 retired, patch 5 retired, sidecar retired (the 2026-06-29 toylang migration). Vale doesn't re-document the retirement archaeology; this appendix indexes inherited lessons.

**F.1 Phantom constraints (over-feared concerns)**
- "Sky's emitter must share LLVMContext with rustc's LLVM backend." Wrong — rustc runs one LLVMContext per CGU. Vale's modules are additional CGUs.
- "Std stays uninlineable under cross-language LTO without `-Z build-std`." Wrong for patch 4 architecture — rustc's `back/lto.rs::prepare_lto` extracts `.llvmbc` from prebuilt rlibs natively.
- "`#[inline(never)]` on stub fn shells prevents the cross-language inlining race." Wrong — fix is at symbol-resolution layer (single-symbol architecture, §5.2), not at inliner layer.

**F.2 Single-symbol over two-symbol (Path B)**

Original design: stub fn rustc-mangled name; Sky's bitcode emits real body under Sky-chosen name; symbol_name override redirects. Works under non-LTO but breaks under ThinLTO (LTO's IR linker pulls all rlibs' bitcode, sees two defs, picks one — sometimes wrong). Fix: emit each rustc-visible body under the **rustc-mangled name rustc would give the stub fn**. Single symbol. One def. No race. Vale inherits.

**F.4 Patch 4 — synchronous submission BEFORE async codegen**

Patch 4 submits extras synchronously on the main thread inside `codegen_crate` BEFORE `start_async_codegen`. Submitting between CGU loop and `codegen_finished` trips coordinator's `main_thread_state == Codegenning` assertion. Vale inherits this timing.

**F.5 Direct provider re-call (don't stash CGU references)**

For Vale's analog of erw's CGU stash question: call saved upstream provider directly from inside `codegen_crate` with live `'tcx`. `default_collect_and_partition()(tcx, ()).codegen_units` returns sound `'tcx`-bound slice with no unsafe. Calling `tcx.collect_and_partition_mono_items(())` doesn't work — in-memory query cache memoizes Vale's override's filtered result.

**F.6 Accessor methods as regular functions**

Field-accessor methods (`widget.field` from Rust) modeled as regular Vale functions with synthesized bodies. One fewer special case across discovery, codegen, symbol-mangling, serialization paths.

**F.7 Type-erased consumer metadata for cross-crate ops**

`ValeUniverse.struct_infos: HashMap<String, Arc<dyn Any + Send + Sync>>`. Consumer inserts its own typed metadata; lookups return Arc; consumer downcasts on read. Per Sky's empirical correction: the layer is load-bearing for `monomorphize_type`'s stateless-callback discipline (@GCMLZ). Layer kept; doc rationale corrected.

**F.10 Cargo profile overrides only at workspace root**

Profile overrides in member packages silently ignored by cargo. vale-stub-gen emits profile overrides only at workspace root.

**F.11 `RUSTC_WORKSPACE_WRAPPER` necessity (valec-rs)**

valec-rs's hook installation requires invocation through the wrapper. Direct `cargo build` bypasses; hook never installs; binary missing Vale bodies. Integration tests of patch 4 behavior MUST invoke through valec-rs's wrapper.

**F.12 The chokepoint pattern (estimation lesson)**

Items estimated at weeks landed in hours when surface routed through 2-5 helper functions all callers funneled through. When scoping refactors, audit for chokepoints first.

**F.13 / F.14 Cascade fires at stub rlib compile, not user-bin**

Empirical correction from Sky's earlier framing. `is_reachable_non_generic` collector gate blocks user-bin from calling `per_instance_mir` on non-generic upstream symbols. Cascade — and therefore discovery of `<Widget as Clone>::clone` etc. — is **exclusively a stub-rlib-compile-time mechanism.** Vale handles via in-process drain at `consumer_fill_modules` (§8.9).

**F.15 Approach B (rustc-owns-lends)**

Patch 4 rev 3 `#[repr(C)]` shape. Rustc owns LLVMContext + LLVMModule; lends to Vale via allocator callback. Vale wraps borrowed pointers in suppressed-Drop handles. No bitcode round-trip; closes B9/B10/B11 risks structurally.

**F.16 Thin-local LTO between CGUs**

Rustc runs LLVM's ThinLTO BETWEEN its own CGUs within a single rustc invocation, even when user sets `lto = false`. Cargo dev default `lto = false` ≠ `lto = "off"`. Vale's cross-Sky/Rust inlining at `lto = false` for Vale-top cases requires the body in the same rustc invocation; cross-crate cases need explicit `lto = "thin"` or `"fat"`.

**F.18 mir_shims elimination + AST-rewrite drop synthesis**

Drop is just a function. `__vale_drop<T>(&local)` wrapper synthesis at `insert_scope_end_drops` time. No mono-time drop-specific work; rustc's standard DropGlue handles trivially-droppable T as no-op and needs-drop T as full chain, all transparent to Vale.

**F.19 Phase R + Phase P + bool accessor + IntLit widening**

Per §25.3.6 calibration discipline. Four silent-correctness bugs surfaced in code paths prior reasoning had explicitly rationalized correct. Vale inherits the discipline: budget empirical-fixture work as load-bearing for catching premise errors.

**F.20 / F.21 Sunny-karp typed-body cache + two-enum split**

Eager type-resolve at `after_rust_analysis` + typed body cached on `ValeState.typed_bodies`. Per-Instance mono substitutes via pure typed-AST walk. Two-enum split (`SourceType` vs `ResolvedType`) makes invalid StructRef-in-resolved-position unrepresentable at type level. Vale inherits both designs from day 1.

**F.22 Drop is just a function migration**

Wrapper-emission scheme replaces predicate-based synthesis. `__vale_drop<T>(&local)` emitted unconditionally for every let; wrapper's `drop_in_place::<T>` handles trivially-droppable T as no-op. Closes bare-`TypeParam` drop closure gap structurally rather than via two-pass scheme. Vale inherits.

**Vale-specific empirical findings will accumulate here as Phase 0/1 work surfaces them.**

---

## Closing notes

This document is the master design for Vale's Rust interop architecture. Total length ~5,500 lines covering 30 chapters + 6 appendices.

Decisions herein are the product of a long design conversation (transcript at `/Volumes/V/Vale4/tmp/claude-conversation-2026-06-26-837eac91.md`), grounded in toylang's prototype work (`/Volumes/V/Harmonious/rust-interop-architecture.md` ~7,700 lines as of 2026-06-29). Implementation anticipated to take 3-5 years for a small team to Vale 1.0. Phasing in §28 lays out recommended order. Pre-1.0 versions are pre-release; breaking changes allowed between minor versions.

Vale is a long-term project. The architecture is designed for evolution; decisions trade short-term complexity for long-term correctness. The fork is sustainable indefinitely; upstreaming pursued as background work (§29.6).

Welcome to Vale.

— Document version 0.1.0
