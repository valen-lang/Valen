# Rust interop — frontend implementation plan (typing pass + oracle seam)

Companion to `vale-rust-interop-architecture.md` §8.10 (the ratified Option A representation). This
plan covers **only** the Vale compiler frontend slice: representing a Rust type in the typing pass
and resolving its facts through an oracle. Codegen / `per_instance_mir` / stub-gen, the parser
`import rust.X` wiring, the real `TyCtxt`-backed oracle, and any physical crate split are **out of
scope** here (later slices).

## 0. The design in one paragraph (the name-property refinement)

Rust-backed-ness is a property of the **name's reserved `rust` `package_coord`**, over the
**existing** kinds — *not* a new kind or name type:

- Rust `struct` → `KindT::Struct(StructTT)` holding an ordinary `StructNameT`, `package_coord == rust`.
- Rust `enum` → `KindT::Interface(InterfaceTT)` — a closed sum type *is* a Vale closed trait
  (and Vale's inline closed traits lower to enums), so a Rust enum shares `InterfaceTT` with Vale's
  own closed traits.
- Rust `trait` → `KindT::Interface(InterfaceTT)` (open interface).
- Rust `union` → deferred (opaque/struct-like edge case).

Because a Rust struct genuinely *is* a struct-kind and a Rust enum genuinely *is* a closed-interface
kind, this is **not a masquerade** — it is the correct mapping, and it *unifies* Rust enums with
Vale closed traits. Verified: `StructNameT`/`InterfaceNameT` are pure-identity (`{ template,
template_args }`, templates just `{ human_name }`), so they carry Rust identity with only a reserved
`rust` `package_coord`. **No new `KindT` arm** (the 51-file blast radius disappears) and **no new
name types**. All rustc *facts* (kind, fields, methods, layout) come from a `RustOracle`, routed at
the **definition-lookup seam**. Single crate; decoupling via a new `typing/rust_interop/` module.

**Milestone 1 — DONE (2026-07-25).** With a fixture oracle, the typing pass typechecks a call to a
Rust function with no rustc and no parser involvement. Test:
`typing/test/rust_interop/rust_interop_tests.rs::calls_a_rust_free_function`.

**Milestone 2 — NEXT: the same test, hosted by rustc.** `add_two_numbers(3, 4)` typechecks against a
signature read from a *real* `TyCtxt`, and the fixture oracle is deleted. See §9.

## 1. Ownership legend

- **🟩 CLAUDE** — brand-new files/modules only. All logic that *can* live in new files does.
- **🟦 HUMAN** — every edit to an already-existing file, however small (per the ownership policy).
  Each 🟦 item below is a precise spec you apply; I never touch these files.

## 2. New code — 🟩 CLAUDE (all under `src/typing/rust_interop/`)

A single new module tree houses every interop-specific piece. The whole tree is
`#[cfg(rust_interop)]`-gated (§4), so under the standalone binary it does not exist. **As built
(2026-07-25):**

- **`mod.rs`** — module root; re-exports.
- **`reserved.rs`** — the reserved `rust` package: `const RUST_MODULE: &str = "rust"`,
  `fn is_rust_backed(id: &IdT) -> bool`, plus `peel_refs` / `citizen_id` for looking through the
  reference onion to the citizen behind a kind. The single source of truth for "is this Rust-backed?".
- **`oracle.rs`** — the seam contract, Vale-owned in/out, **no `'tcx` in any signature**:
  - `enum RustKind { Struct, Enum, Trait, Union }`
  - `struct RustItemId(u32)` — opaque handle, valid only within one invocation.
  - `struct ValeSig<'s,'t> { params: &'t [KindT<'s,'t>], ret: KindT<'s,'t> }`. **Over `KindT`, not
    `CoordT`** — `CoordT` no longer exists; the onion refactor dissolved it into the reference wraps
    inside `KindT`, so a Rust `&self` receiver arrives already wrapped as a `BorrowRef`.
  - `trait RustOracle<'s,'t>` — **per-question**, no definition object: `resolve_path`, `kind`,
    `resolve_method`, `resolve_function`, `item_package`, `fn_sig(item, args, interner)`, `field`.
    Each returns Vale-owned data; there is **no** `struct_def`/`interface_def` query.
  - `struct StubOracle` — every query returns `None`, so every seam falls through to ordinary Vale
    behavior. What the typing pass holds until the `TyCtxt` oracle lands.
- **`seam.rs`** — what the 🟦 hooks delegate to. `push_rust_call_candidates(...)` contributes a Rust
  callee as an ordinary overload candidate and synthesizes its prototype;
  `maybe_rust_field(...)` answers a `pub` field query. **No** definition synthesis (§5).
- **`fixture.rs`** — `FixtureOracle`, a table-driven `RustOracle` for tests. **Temporary: to be
  deleted in Milestone 2 (§9)**, when a real `TyCtxt` oracle replaces it.

Dropped from the original plan: `lower.rs` (nothing needs to construct a Rust-backed *kind* yet — the
free-function path never builds one) and `memo.rs` (no measured cold-path cost to amortize; add it
when a profile asks for it, not before).

## 3. Existing-file edits — 🟦 HUMAN (precise specs)

Two truths shape this: **(i) no fabricated definitions** — Rust types have no Vale
`StructDefinitionT`/`InterfaceDefinitionT`; the oracle answers *specific questions* (§5); and
**(ii) every edit is `#[cfg(rust_interop)]`-gated (§4)**, so under `valec` these files compile
byte-identical to no-interop (the module, the hooks, and the oracle field all vanish).

### 3a. The live edit set (infrastructure + method seam — enough for the `push` milestone)

**LANDED 2026-07-25.** What follows is what was actually applied, which differs from the original
draft of this section in two ways — both corrections, recorded here so the doc matches the tree.

**Correction 1 — the carrier is `Compiler`, not `CompilerOutputs`.** `CompilerOutputs` exists to be
drained into `HinputsT` (`compiler.rs:1291`); an oracle is an *input*, a query service, and belongs
with the other borrowed services on `Compiler` (`&'ctx ScoutArena`, `&'ctx TypingInterner`, …). Three
concrete consequences settled it: the seam already takes `&Compiler` anyway (it needs
`typing_interner` and `opts.global_options.sanity_check`); putting the oracle on the `&mut` thing
forced a copy-the-handle-out-first dance in the seam; and `&'ctx` is the accurate lifetime for a
borrowed service, where `CompilerOutputs` would have given it `&'t`. Moving it also *removed* edits —
`compiler_outputs.rs` is untouched, and `evaluate` needs no param.

**Correction 2 — the call seam is a candidate source, not a fallback anywhere.** The original draft
put it in `find_function`'s `Err` arm; a first revision moved it to `find_potential_function`'s
failure branch; both are wrong. See §3a.4.

1. **`src/typing/mod.rs`** — after the sub-compilers block:
   ```rust
   #[cfg(rust_interop)]
   pub mod rust_interop;
   ```
2. **`src/typing/compiler.rs`, `struct Compiler`** — add the field alongside the other borrowed
   services (plus a `#[cfg(rust_interop)] use` at file top; fully-qualified `crate::` paths inline
   are forbidden by the `UUSNNCBX` shield):
   ```rust
   #[cfg(rust_interop)]
   pub oracle: &'ctx dyn RustOracle<'s, 't>,
   ```
3. **`src/typing/compiler.rs`, `Compiler::new`** — a cfg'd param + cfg'd field-init. One call site
   (`compilation.rs:117`), so no ripple.
4. **`src/typing/overload_resolver.rs`** — the **call** seam, in **two** parts:
   - **`get_param_environments` — MANDATORY.** Yield `Vec::new()` for a Rust-backed
     `KindT::Struct`/`KindT::Interface`, *before* the `get_outer_env_for_type` calls. Without it,
     resolution panics with `"No outer env for type"` the moment a Rust-backed receiver appears —
     such a citizen has no Vale definition and therefore no registered env. This guard is the weaker
     part of the design: the real fix is `get_outer_env_for_type` returning `Option`, since six-plus
     other callers will each need the same guard until it does.
   - **`get_candidate_banners`** — the seam itself: a fourth candidate source, after the calling env,
     the param envs, and the placeholder extra-call envs. One gated line calling
     `push_rust_call_candidates(...)`, which contributes an `ICalleeCandidate::PrototypeTemplata`.

     **Why a candidate source rather than a fallback on failure.** Two reasons. (i) A failure-branch
     hook is *unreachable* — the panic above fires first. (ii) A fallback would make a Rust callee
     invisible whenever any Vale function of the same name matched loosely, which is an
     overload-semantics decision made by accident. As a candidate it flows through
     `attempt_candidate_banner`'s existing `PrototypeTemplata` arm, `params_match`, and
     `narrow_down_callable_overloads` like anything else.

     **Why the bounds registration lives here and not in an environment.** An env cannot produce a
     valid Prototype candidate: `get_candidate_banners_inner` asserts
     `get_instantiation_bounds(..).is_some()` on every one it accepts, but env lookup has no
     `&mut CompilerOutputs` and `get_outer_env_for_type` takes `&self`. The candidate source runs
     where `coutputs: &mut` is in scope, which is the only place that assert can be satisfied.
5. **`src/typing/compilation.rs:115-119`** — the pass entry constructs the oracle and hands it to
   `Compiler::new`. The single `#[cfg(rust_interop)]` line holding `StubOracle` is where the real
   `TyCtxt`-backed oracle plugs in later.
6. **`FrontendRust/build.rs`** — `println!("cargo::rustc-check-cfg=cfg(rust_interop)");`, or
   `--cfg rust_interop` trips the `unexpected_cfgs` lint on the pinned nightly.

### 3b. The per-question seam pattern (one guard at a time, as capabilities land)

There is deliberately **no `lookup_struct`/`lookup_interface` guard** — those return a Vale definition
Rust types don't have (a Rust id should never reach them; if one does, a clear panic is the nicety,
not a fabricated definition). Instead each Rust capability is a small cfg'd guard at *its* site,
routing the *specific* question to the oracle:

| capability | site | oracle query | status |
|---|---|---|---|
| free function `f(a, b)` | `overload_resolver.rs::get_candidate_banners` — candidate source, name-keyed | `resolve_function` + `item_package` + `fn_sig` | **live, tested** |
| method call `x.m()` | same candidate source, receiver-keyed on `param_filters[0]` | `resolve_method` + `fn_sig` | **live, untested** (needs a Rust-backed `StructTT`, so `resolve_path` + `kind` first) |
| `pub` field `x.f` | `expression_compiler.rs` — the `KindT::Struct(struct_tt)` arm of the `Dot` handler. Guard at the top: `#[cfg(rust_interop)] if is_rust_backed(&struct_tt.id)` → build a member-lookup node from `oracle.field(...)`'s type instead of `coutputs.lookup_struct(...).get_member_and_index(...)`. `pub` only; private field → clear "private Rust field" error. | `field(id, name) -> Option<RustFieldInfo>` | **pinned, next** |
| match a Rust enum's variants | the pattern/match compiler (site not yet pinned) | `variants(id)` | future |
| definition-facts (e.g. the `Extern` check in `ensure_deep_exports`) | that specific site | targeted (often a constant, e.g. "a Rust type is not a Vale `extern` citizen → `false`") | as-encountered |

The remaining ~30 call-out sites — layout, drop, conformance, generics, sharedness, weakability,
sealedness — are enumerated with their exact functions in `rust-interop-callout-map.md` and marked
with `// ZRI` comments in the source.

Each is a few cfg'd lines routing to an oracle query; **none reintroduces a fabricated definition**.
**No `KindT` arm, no name-type arm, no interner-wrapper edit, no family-enum arm** — all eliminated by
the name-property design.

## 4. cfg / scope boundaries — **nothing references the new files when rustc is off**

Requirement (2026-07-24): under `valec` (cfg `rust_interop` off) **nothing may call into or reference
`rust_interop`** — the core must compile as if interop were never written. Mechanism:

- **Everything interop is `#[cfg(rust_interop)]`-gated:** the `pub mod rust_interop;` declaration, the
  `oracle` field on `Compiler` and its threading, and **every seam hook**. Under valec the module
  doesn't exist, the hooks compile out, and the core files are byte-identical to no-interop.
- **Each seam hook is a single gated, delegating line** into the module, so *all* logic lives in the
  new files and the core just has "ask the module, then carry on":
  ```rust
  // candidate source — get_candidate_banners, after the three env-derived sources:
  #[cfg(rust_interop)]
  push_rust_call_candidates(self, coutputs, env, function_name, param_filters, results);
  // pub-field seam — Dot handler, KindT::Struct arm:
  #[cfg(rust_interop)]
  if let Some(field) = maybe_rust_field(self, &struct_tt.id, name) { … }
  ```
- **Where the `use` goes.** Fully-qualified `crate::…` paths inline are forbidden by the `UUSNNCBX`
  shield, so each hook's imports are a `#[cfg(rust_interop)] use` at the file top rather than an
  inline path.
- **Enforcement = the green `cfg(rust_interop)`-off `valec` build.** A non-gated reference to
  `rust_interop` fails that build; `grep rust_interop` on the core shows only the gated one-liners,
  all compiled out.
- **Honest residual:** the core *files* still physically contain those gated one-liners (inert under
  valec). Literally-zero interop text in the core is not achievable for control-flow interception
  without a global hook table (forbidden by @NGSAX / NoGlobalStateAnywhere) or editing every caller
  (worse), so a handful of `#[cfg]`-gated one-liners is the minimal footprint.
- **Tests** that exercise the stub oracle run with `--cfg rust_interop` (or are themselves
  `#[cfg(rust_interop)]`); CI builds/tests *both* configs (the valec/off build is the forcing
  function).
- The real `TyCtxt`-backed `RustOracle` impl (which additionally needs `#![cfg_attr(rust_interop,
  feature(rustc_private))]`) is a **later slice**, inside `rust_interop/` under a further cfg — per
  the single-crate decision, not a separate crate.

## 5. Rust types have no Vale definition — the oracle answers per-question

Vale does **not** build a `StructDefinitionT`/`InterfaceDefinitionT` for a Rust type. A Rust item is
opaque *as a Vale definition*; only its identity lives in Vale's IR (§8.10). Everything else is a
*question* answered by the oracle at the site that asks it (§3b):

- **method call** → `resolve_method` + `fn_sig` → a call **prototype** (not a definition).
- **`pub` field access** → `field(id, name)` → the field's Vale-lowered type + visibility. Vale *is*
  an external consumer, so **`pub` fields are reachable** (`pub` only; private → clear error). The
  earlier "Rust types are fully opaque to Vale" framing was too strong — only *private* internals are.
- **matching a Rust enum** → `variants(id)` (future).
- **definition-facts** (attributes, is-sealed, variance) → targeted queries, often a constant (e.g. a
  Rust type is not a Vale `extern` citizen → `false`).

There is intentionally **no `lookup_struct(rust_id)` path** — nothing asks a Rust type for a Vale
definition body, so Rust ids never reach it; a clear panic if one does is a diagnostic, not a reason
to fabricate one.

## 6. Sequencing

**Landed (2026-07-25).** The `rust_interop/` module (`reserved`, `oracle`, `seam`, `fixture`); the
oracle carried on `Compiler` as a `#[cfg(rust_interop)]` field, threaded from
`TypingPassCompilation::new`; the Rust candidate source in `get_candidate_banners` with both a
receiver-keyed and a name-keyed trigger; prototype synthesis; `Source::rust()`; and a green
fixture-driven test. Both configs build; the 573-test default suite is unchanged.

**Next: Milestone 2 (§9).** Everything else stays deferred until it lands: parser `import rust.X`
name-resolution wiring, codegen / `per_instance_mir` / stub-gen, the cargo orchestrator, and the
physical crate split (we chose single-crate cfg-only).

## 7. Open items to resolve during implementation

- ~~Oracle threading shape~~ — **resolved:** a `#[cfg(rust_interop)]` field on `Compiler` (not
  `CompilerOutputs` — that is the output accumulator, drained into `HinputsT`), supplied as a cfg'd
  param on `TypingPassCompilation::new`. Tests use `typing_pass_compilation_for_test`, which fills in
  a `StubOracle`, so no test about Vale semantics mentions the build mode.
- ~~Exact method-resolution site~~ — **resolved, and the original spec was wrong.** Not
  `find_function`'s `Err` arm: that is unreachable for a Rust receiver, because
  `get_param_environments` panics in `get_outer_env_for_type` first. Rust callees now enter as a
  fourth **candidate source** in `get_candidate_banners`, so they are scored alongside Vale
  candidates rather than caught after failure.
- ~~Free functions~~ — **resolved:** the receiver-keyed trigger cannot fire for
  `add_two_numbers(3, 4)`, whose args are both plain ints. A second, name-keyed trigger handles it,
  with scoping delegated to the oracle.
- ~~`ValeSig` field types~~ — **resolved:** `KindT`, not `CoordT`. `CoordT` no longer exists; the
  onion refactor dissolved it into the reference wraps inside `KindT`.
- **@EarlyBinder discipline** — still open, and the fixture *cannot* close it: `FixtureOracle::fn_sig`
  ignores its `args`, so instantiate-then-lower and lower-then-instantiate are indistinguishable.
  Needs a generic Rust function and a real oracle — scheduled in §9.
- **`get_outer_env_for_type` panics on absence.** `get_param_environments` carries a guard for it;
  six-plus other callers will each need one until that function returns `Option`. See the `ZRI`
  marker at its definition.

## 8. Doc follow-up (mine, docs only)

**Done (2026-07-24):** `vale-rust-interop-architecture.md` §8.10 has been revised to the name-property
design — Enum/Trait → interface family (a Rust enum is a closed `InterfaceTT`, unified with Vale
closed traits), reuse of the existing Struct/Interface kinds + `StructNameT`/`InterfaceNameT`, no new
`KindT` arm, the definition-lookup seam, and the single-crate-cfg framing.

**Done (2026-07-25):** `rust-interop-callout-map.md` records the ~30 call-out points across the
compiler where a Rust-backed type would need the oracle, with the `ZRI` markers in the source
pointing at each. That map is currently **one-directional** — it covers Vale asking rustc, not rustc
calling into Vale (`Callbacks`, `per_instance_mir`, `layout_of`, `fill_extra_modules`). The inbound
half is specified in the architecture doc §4/§5/§19/§20 but is not yet mapped against the code.

## 9. Milestone 2 — the typing pass, invoked by rustc

**Goal.** This program typechecks, with `add_two_numbers`'s signature read from a real `TyCtxt`:

```vale
exported func main() int {
  return add_two_numbers(3, 4);
}
```

and `FixtureOracle` is deleted in the same change. The point is not new seam surface — the seam is
built and green — it is to replace canned data with rustc and prove the two agree.

**Why this shape, and why now.** Milestone 1 tests the Vale half of the seam with the rustc half
faked. A fake we keep is a fake that becomes the specification, and it already diverges in one known
way: `FixtureOracle::fn_sig` ignores its `args`, so it cannot exercise the @EarlyBinder ordering
rule. This milestone is the smallest step that removes it.

### 9.1 The inversion

Vale is invoked as a CLI today. rustc cannot be called as a library that hands back a `TyCtxt` —
`TyCtxt` exists only inside `rustc_driver::run_compiler`'s callback. So the control flow inverts:
**rustc hosts, and the Vale typing pass runs inside `Callbacks::after_expansion(tcx)`.**

That is the architecture §20.3 already specifies, so this milestone is a miniature of the real
thing, not a detour. The arena nesting works out: Vale's `'t` is created inside the `'tcx` frame, so
`'tcx: 't` and a `TyCtxtOracle<'tcx, 's, 't>` can hold a `TyCtxt<'tcx>` while implementing the
`'tcx`-free `RustOracle<'s, 't>`.

Deliberately **not** in this milestone: cargo, the `.vale-build/` orchestrator,
`RUSTC_WORKSPACE_WRAPPER`, stub rlibs, `__VALE_STUBS_MARKER`, any query override, codegen, the fork,
or a running binary. Only the read path, and it needs no fork — `fn_sig`, `module_children`, and
`def_path` are stock queries. The four fork patches are all mono/codegen.

### 9.2 Steps

1. **Gate the C++ backend link.** `build.rs` unconditionally builds and statically links the backend
   against LLVM 16, while `rustc_driver`'s dylibs carry rustc's own (~21). Two LLVMs in one process
   is the duplicate-symbol UB §5.7 of the architecture doc names. `backend_ffi` is already out of
   `lib.rs`, so a typing-pass host needs no backend — put the linking behind a feature and leave it
   off for interop builds. **Do this first**; otherwise the failure mode is mysterious link errors.
2. **`rustc-dev`** added to `rust-toolchain.toml` components (available for this target, not
   currently installed).
3. **`#![cfg_attr(rust_interop, feature(rustc_private))]`** plus the `extern crate` declarations for
   `rustc_driver`, `rustc_middle`, `rustc_hir`, `rustc_span`, `rustc_interface`.
4. **A driver host** (new module, `#[cfg(rust_interop)]`): `run_compiler` over an in-memory Rust
   source defining `add_two_numbers`, an `impl Callbacks` whose `after_expansion` builds the oracle
   and runs the Vale typing pass in that closure.
5. **`TyCtxtOracle`** implementing `RustOracle`. Only three methods matter for this program:
   - `resolve_function(name)` — walk `module_children` from the crate root, match by name.
   - `item_package(item)` — `def_path` → intern a `PackageCoordinate` into the scout arena.
   - `fn_sig(item, args, interner)` — `tcx.fn_sig(did)`, **instantiate at `args` first**, then
     `skip_binder`, then lower each `Ty<'tcx>` to a `KindT`. For primitives that lowering is a small
     match on `TyKind`; it is the piece that will keep growing.
6. **Port the test** to run against the real oracle, then **delete `fixture.rs`** and its export.

### 9.3 What this milestone will expose

Worth expecting rather than being surprised by:

- **Lowering is where the work moves.** `Ty<'tcx>` → `KindT` is trivial for `i32` and immediately
  hard for anything else. The IR gaps recorded at the `ZRI` markers in `typing/types/types.rs` bite
  here first: no signedness on `IntT`, no width on `FloatT`, no unsized concept.
- **@EarlyBinder becomes testable.** Only once a *generic* Rust function is in the fixture crate does
  instantiate-then-lower differ from lower-then-instantiate. Add that case in this milestone — it is
  the regression fixture §7 asks for, and it cannot exist without a real oracle.
- **`resolve_function` scoping.** With real rustc, "which names are in scope" stops being a fixture's
  table and becomes a real question. The oracle should answer only for explicitly-imported paths, or
  a Rust function will start competing at call sites that never asked for it.

### 9.4 Done when

`calls_a_rust_free_function` passes against a real `TyCtxt`, `fixture.rs` is gone, and the default
(non-interop) suite is unchanged.
