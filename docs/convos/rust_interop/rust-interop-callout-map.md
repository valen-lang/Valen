# Rust interop — where the compiler calls out to the Rust-handling code

Companion to `rust-interop-frontend-plan.md` and `vale-rust-interop-architecture.md` §8.10.

This document is the **exhaustive map of call-out points**: every place the existing Vale compiler
asks a question about a type that, for a Rust-backed type, must be answered by the `RustOracle`
instead of by Vale's own tables. It was produced by twelve independent read-only surveys of
`FrontendRust/src` (2026-07-24), each covering one class of question.

Its purpose is to replace the frontend plan's "five edits, plus more as encountered" with a
counted, cited list, so the integration surface is known before code is written rather than
discovered one panic at a time.

**Line numbers are as of `experimental-4` @ `af3a3c17a`** and will drift. Treat them as anchors,
not addresses.

---

## 0. How to read this document

### 0.1 Live vs unlinked

`FrontendRust/src/lib.rs:7-33` comments the following out of the crate:

```
backend_ffi, clang, file_coordinate_map, final_ast, instantiating,
simplifying, von, testvm, end_to_end_tests, integration_tests
```

`pass_manager/mod.rs:1-2` additionally comments out both of its submodules, so
`bin/valec/frontend.rs:92` calls a `pass_manager::pass_manager::build` that no longer exists —
**the `valec` binary does not compile in this tree.** Compilation today is driven only by the
typing-pass test harness.

This is deliberate (`lib.rs:5-6`: *"Onion typing arc: parser + postparsing linked; typing and
downstream stay unlinked pending their own slices"*), but it means roughly half the call-out
points below are **in code that does not currently run**. Every entry is tagged:

- **[LIVE]** — in the compiled crate today. Real, testable, blocking.
- **[DARK]** — ported-from-Scala but unlinked. Real future work; cannot be validated until the
  onion arc relinks that pass. Design now, land later.

Stub density, as a calibration on how finished the dark code is: `typing/` has 242
`panic!("Unimplemented…")`, `instantiating/` 105 (38 in `instantiator.rs` alone), `simplifying/`
26, `final_ast/` 11. `typing/reachability.rs` is 100% stub with zero callers.

### 0.2 The two kinds of finding

Throughout, distinguish:

- **A call-out point** — a place that must route to the oracle. Mechanical once designed.
- **A model mismatch** — a place where Vale's design and Rust's design disagree, so there is no
  answer the oracle could give. These need decisions, not code. Collected in §5.

---

## 1. Headline

The frontend plan lists **5 live edits** (`typing/mod.rs`, the `CompilerOutputs` oracle field and
`new()` param, `find_function`'s `Err` arm, `compiler.rs:742`) plus one pinned-next (the `Dot`
handler) and an open-ended "as-encountered" bucket.

The surveys found **~30 distinct call-out points in the live typing pass**, plus ~20 more in the
dark passes. More consequentially, they found that **one of the five live edits is unreachable as
specified** (§3.1) and **one of the plan's type specifications no longer exists** (§3.2).

The good news is genuinely good and is not diminished by the count:

- The core representational bet holds. `KindT::Struct` carries **only** an `IdT`; every type
  property is a side-table lookup keyed by that id. So the property side-tables *are* the oracle's
  surface, and reusing the existing kinds requires **no new `KindT` arm and no new name type**.
  `PackageCoordinate` has no validation, no enum, no registry; `IdT` treats it as an opaque
  canonical pointer; the interner never inspects it. A `rust`-packaged name flows through the
  72-variant `INameT` and every match on it unchanged.
- Backend mangling **already** prepends module + package steps, so `rust.std.vec.Vec` mangles
  correctly today (`simplifying/name_hammer.rs:126-144`).
- Someone laid groundwork on purpose. `keywords.rs:148` declares `pub rust: StrI` (initialized in
  both arenas at `:305`/`:462`, currently unread). `simplifying/hammer.rs:332` has a **live**
  `panic!("translate functionExterns: rust-package empty-name branch")`. `hammer.rs:226` has the
  Scala `if module == "rust" { "" }` commented in. The `@PRIIROZ`/`@SMLRZ` generic-arg reshuffle
  (`function_compiler_core.rs:398-409`, `hammer.rs:344-370`) exists specifically so a path emits as
  `Vec<i32>::new` rather than `Vec::new<i32>` — a Rust concern, not a C one.
- Several surfaces are *less* built than feared, which means less to fight: **variance does not
  exist anywhere** in the frontend (zero grep hits); arity checking is barely wired
  (`check_generic_call`'s citizen call site is commented out); conformance is not yet a solver
  constraint at all.

---

## 2. The live call-out points

Grouped by the question being asked. Each is a place where, today, a Rust-backed id either panics
or silently produces a wrong answer.

### 2.1 Getting a Rust name into the compiler at all

| # | Site | Question | Notes |
|---|---|---|---|
| 1 | `code_source.rs:69` `CodeSource::resolve` | "what files are in package `rust.std.vec`?" | **[LIVE]** Returning `Some(HashMap::default())` for `module == "rust"` makes `import rust.std.vec.Vec` stop panicking with **zero other code**. Precedent: `integration_tests/tests/import_tests.rs:229-265` already imports a package with no `.vale` files and passes. Prevents `panic!("Couldn't find: {:?}")` at `lexing/lex_and_explore.rs:41`. |
| 2 | `typing/compiler.rs:716` | "what top-level namespaces exist?" | **[LIVE]** `name_to_top_level_environment` is built from a vec at `:653-659`. Appending one `(rust_package_id, rust_templatas_store)` pair makes Rust names visible to **all ~40 env-lookup call sites with no changes to any of them**. Copy the builtin-store pattern at `:661-670`. |

**Caveat on #2 — see §5.1.** This is the cheapest possible injection *and* it walks straight into
the name-collision hazard, because Vale's env lookup is package-blind.

### 2.2 Definition lookup — "give me the `StructDefinitionT`"

All six public accessors delegate to **two map reads**: `compiler_outputs.rs:561` and `:579`.

| # | Site | Notes |
|---|---|---|
| 3 | `compiler_outputs.rs:560` `lookup_struct_template` | **[LIVE]** `.expect("Struct template not found")`. Covers `lookup_struct`, `lookup_citizen_by_template_name`, `lookup_citizen_by_tt` by delegation. |
| 4 | `compiler_outputs.rs:577` `lookup_interface_by_template_name` | **[LIVE]** `panic!("vfail: … templateName not found")`. Covers `lookup_interface`. |

Consumers that reach these with a citizen id (all **[LIVE]**): the `Dot` handler
(`expression_compiler.rs:796`), positional member load (`pattern_compiler.rs:700`), destructure
(`pattern_compiler.rs:575`), `destruct` (`expression_compiler.rs:1475`), constructor synthesis
(`struct_constructor_macro.rs:147`), drop synthesis (`struct_drop_macro.rs:238`), the export
walker (`compiler.rs:1542`), sharedness (`struct_compiler.rs:300`), weakability
(`expression_compiler.rs:1932/1938`, `impl_compiler.rs:330/334`), the `Extern` attribute check
(`compiler.rs:1517`), and generic-param arity for extern methods
(`function_compiler_core.rs:405`).

**The architectural decision the plan already made — and should hold to — is that there is no
`StructDefinitionT` for a Rust type.** These two accessors should therefore *not* synthesize one.
They should be reachable only in error, and the per-question seams below are the real surface.
What they need is to stop being `panic!` and start being a diagnosable error (§4.3).

### 2.3 Environment-for-type — the method-resolution surface

**This is the surface the frontend plan does not mention, and it is where `my_vec.push(x)`
actually resolves.**

| # | Site | Question | Notes |
|---|---|---|---|
| 5 | `compiler_outputs.rs:633` `get_outer_env_for_type` | "what env should I search for methods on this type?" | **[LIVE]** `panic!("No outer env for type: {:?}")` at `:640`. |
| 6 | `compiler_outputs.rs:646` `get_inner_env_for_type` | "what function-bound runes does this citizen carry?" | **[LIVE]** bare `.unwrap()` at `:647`. Correct answer for a Rust type is "empty", but it panics first. |
| 7 | `compiler_outputs.rs:534` `lookup_sealed` | "is this interface sealed?" | **[LIVE]** `panic!("Still figuring out sealed")`. Exactly one caller (`function_compiler_middle_layer.rs:49`). **Not an oracle call** — a policy default to pick, since Rust's sealed-trait pattern is a convention (a private supertrait), not a queryable property. Conservative = sealed, whose consequence is that Vale-side abstract methods on an imported Rust trait are blocked. Whatever the default, *every* imported Rust trait needs an entry, because absence panics here rather than defaulting. |

Callers of #5/#6 (all **[LIVE]**): `overload_resolver.rs:504-505` (`get_param_environments` — the
hot one), `:540` (`get_placeholder_extra_call_envs`), `impl_compiler.rs:517/589-590/491`,
`edge_compiler.rs:642-643/:516`, `struct_compiler_generic_args_layer.rs:364/:483`,
`templata_compiler.rs:1052`, `infer_compiler.rs:494`.

### 2.4 Method / function resolution

| # | Site | Notes |
|---|---|---|
| 8 | `overload_resolver.rs:496-510` `get_param_environments` | **[LIVE] MANDATORY, and the plan omits it.** Must return `vec![]` for a `rust`-package id *before* calling `get_struct_template` + `get_outer_env_for_type`. Without this, resolution panics at `compiler_outputs.rs:640` on line `:571`, long before any fallback can run. |
| 9 | `overload_resolver.rs:553` `find_potential_function`, ~`:591` | **[LIVE]** The corrected method-fallback hook (§3.1). Single point where "zero candidates" and "all candidates rejected" converge; still holds `args`, `function_name`, template-arg runes; returns `AttemptedCandidate { prototype }`. Hooking here covers all 9 `find_function` callers and all 4 `resolve_function` callers with no per-site edits. |

Paths that **bypass** #9 and need separate treatment (all **[LIVE]**):
`expression_compiler.rs:1726/1753/1847/1889` (`Some`/`None`/`Ok`/`Err` resolve directly via
`evaluate_generic_light_function_from_call_for_prototype`); `call_compiler.rs:244 check_types`
(re-checks after resolution, `panic!` at `:272`); `convert_helper.rs:60` (§2.7).

### 2.5 Field access

| # | Site | Notes |
|---|---|---|
| 10 | `expression_compiler.rs:783` (`Dot` handler), `KindT::Struct` arm at `:795-796` | **[LIVE]** The plan's pinned-next seam; confirmed. Note `:799` is `panic!("CouldntFindMemberT")` — it panics rather than raising the error variant that already exists. That must become a real error before a "private Rust field" diagnostic can live there. |

### 2.6 Solver rules

Of the **12 live `IRulexSR` variants**, only four interrogate a type. The solver core
(`solver/solver.rs`) is fully rule-agnostic — nothing to inject there.

| # | Site | Rule | Notes |
|---|---|---|---|
| 11 | `compiler.rs:296` `lookup_templata_imprecise` → `templata_compiler.rs:1331` | `Lookup` | **[LIVE]** Sole path by which `Lookup` produces a template templata. Returns `StructDefinitionTemplataT` holding `&'s StructS` — see §5.2. |
| 12 | `compiler_solver.rs:1231` `solve_call_rule` | `Call` | **[LIVE]** Both directions. Forward branch ends in `panic!("vimpl: solve_call_rule None")` at `:1452` — a Rust template lands there. Reverse branch zips `template_args()` positionally against arg runes at `:1258`/`:1291`. |
| 13 | `rune_type_solver.rs:165` `IRuneTypeSolverEnv::lookup` (impl at `templata_compiler.rs:1117`) | rune-typing `Lookup` | **[LIVE]** A genuinely clean trait seam — two implementors, one method. Returns `Citizen { tyype, generic_params }` read off `origin_struct`, which a Rust item does not have. For a `rust`-packaged name, synthesize a `TemplateTemplataType` from rustc's `generics_of`, mapping each param kind: **type → `Kind`, const → `Integer`, lifetime → `Region`** (and see §5.3 — the lifetime row is exactly the `ITemplataT::Region` gap). |
| 14 | `rune_type_solver.rs:457` | rune-typing `Call` | **[LIVE]** Needs `generics_of` mapped to a `TemplateTemplataType`. `other => panic!` at `:480`. |
| 15 | `compiler_solver.rs:551` | `Resolve` | **[LIVE]** Falls through to overload resolution, so #8/#9 cover it. |

The other eight variants (`Equals`, `Literal`, `RuneParentEnvLookup`, `KindList`, `CallSiteFunc`,
`DefinitionFunc`, `BorrowRef`, `WeakRef`, `OwnRef`) are structural — no rustc query.

### 2.7 Conformance, conversion, dispatch

| # | Site | Notes |
|---|---|---|
| 16 | `impl_compiler.rs:568` `is_parent` | **[LIVE]** THE conformance oracle. **Not a predicate** — returns `IsParent { templata, conclusions, impl_id }` *and* mutates `coutputs` via `add_instantiation_bounds` at `:610`/`:654`. Largest API-shape risk in the seam. Dispatch must happen before the `get_outer_env_for_type` calls at `:589-590`. |
| 17 | `impl_compiler.rs:507` `get_parents` | **[LIVE]** "what does X implement". Feeds `is_descendant`, `get_ancestors`, and the if/else LUB (§5.4). Must return early for a rust-backed `sub_kind` — the `get_outer_env_for_type` on `:517` panics otherwise. **Honest warning: this question may not be answerable for Rust at all**, because Vale wants the complete set and Rust's is unbounded under blanket impls. Where a caller only needs "does it implement *this* one", route it to `is_parent` (#16) instead of trying to enumerate. Note `:547` silently drops its error. |
| 18 | `convert_helper.rs:139` `convert_via_upcast` | **[LIVE]** Single place an upcast is materialized. `assert!(get_instantiation_bounds(impl_id).is_some())` at `:176` must be satisfied or relaxed. |
| 19 | `templata_compiler.rs:1184` `is_type_convertible` | **[LIVE]** Needs its own arms regardless of #16: `:1231` hardcodes `(_, KindT::Struct) => false`, and `:1245` is `_ => panic!`. Two more live panics at `:1209`/`:1215`. |
| 20 | `edge_compiler.rs:66` `compile_i_tables` | **[LIVE]** Single entry to vtable construction, one caller (`compiler.rs:1201`). Rust-backed interfaces must be excluded *here*, not deeper (§5.5). |

### 2.8 Lifecycle

| # | Site | Notes |
|---|---|---|
| 21 | `destructor_compiler.rs:41` `Compiler::drop` | **[LIVE]** Single decision point for discard-vs-destructor-call. `__vale_drop<T>` slots in here. A Rust-backed arm asks the oracle **two** things: `needs_drop(ty)` — if false, emit a bare `Discard` exactly like the primitive arms above — and otherwise the **drop-glue symbol** (rustc's `InstanceKind::DropGlue` + `symbol_name`, one per monomorphization). Without the first query every Rust value pays a destructor call it doesn't need; without the second there's nothing to call. Scope-end synthesis (`drop_since`, `expression_compiler.rs:2104`) is already type-agnostic — it asks the *environment* which names are live and asks the type nothing — so it needs no change, which also means it will happily synthesize `drop(x)` for a Rust value and the failure lands entirely here. |
| 22 | `destructor_compiler.rs:18` `get_drop_function` | **[LIVE]** Single name-lookup point for drop resolution. |
| 23 | `struct_drop_macro.rs:225-235` | **[LIVE]** Already `panic!("auto-generated drop for extern struct is unsupported…")`. This is the exact line Rust-backed types die on today. Existing design doc: `todo/opaque-extern-drop.md`, plus `todo/ffi-drop-followups.md`. |

### 2.9 Generic machinery

| # | Site | Notes |
|---|---|---|
| 24 | `templata_compiler.rs:1033` `get_reachable_bounds` | **[LIVE]** Calls `get_inner_env_for_type(...).unwrap()`. Correct answer for a Rust kind is empty bounds; must short-circuit. |
| 25 | `templata_compiler.rs:446` `substitute_templatas_in_kind` | **[LIVE]** Rebuilds a `StructTT` by rewriting its `IdT`'s template args. For a Rust type this is rustc's `EarlyBinder::instantiate` — Vale cannot do it. Hits `get_instantiation_bounds(...).unwrap()` at `:591`. Reachable whenever a Vale generic mentions `Vec<T>` with a placeholder `T`. |
| 26 | `compiler.rs:171` `get_placeholders_in_kind` (+ `:140` `get_placeholders_in_templata`) | **[LIVE]** Mutually recursive; walks template args and all four onion layers. Needs `generics_of` + a recursive walk. |
| 27 | `templata_compiler.rs:207-402` — ~9 name-projection fns (`get_citizen_template`, `get_struct_template`, `get_interface_template`, `get_impl_template`, `get_placeholder_template`, …) | **[LIVE]** Name-level only, no storage. Each `panic!`s on an unexpected variant. A `rust`-packaged name must be a genuine `INameT::Struct`/`Interface`, which the plan's design guarantees — so these are safe *as long as the plan is followed*. Listed because they're the tripwire if it isn't. |

### 2.10 Export / extern boundary

| # | Site | Notes |
|---|---|---|
| 28 | `compiler.rs:1416-1611` `ensure_deep_exports` | **[LIVE]** The single "type crosses a boundary" check in the compiler, and the direct template for a Rust-boundary universe check. Recursively walks members (`:1548-1566`) and array element types (`:1567-1597`). Three error variants fire spuriously on Rust-backed kinds (§4.2). The four ref arms at `:1603-1606` are `unimplemented!()` — and Rust interop is fundamentally about references. |
| 29 | `compiler.rs:1517` | **[LIVE]** The `ICitizenAttributeT::Extern` check inside #28. The existing extern escape hatch at `:1509-1520` (placeholders waved through because "the concrete kind per monomorphization is what matters for ABI") is exactly the right precedent to generalize. |
| 30 | `compiler.rs:1683` `Compiler::is_primitive` / `types.rs:97` `KindT::is_primitive` | **[LIVE]** Two divergent definitions — see §4.1. Both are ABI gates. |

### 2.11 Things that need a decision rather than a hook

| # | Site | Notes |
|---|---|---|
| 31 | `struct_constructor_macro.rs:36/:85` | **[LIVE]** Reads the **postparsing `StructS`**, not the typing definition. A `lookup_struct` oracle hook never reaches it, so a Rust-backed struct silently gets **no constructor**. Probably correct ("call `Vec::new()` through the oracle"), but it must be a decision, not an accident. |
| 32 | `typing/compiler.rs:1299-1310` `translate_function_attributes` | **[LIVE]** `panic!` at `:1304` on any attribute other than `UserFunction`/`Extern`. Any new `exported(rust)`-style attribute **must** be added here or it panics at runtime. |

---

## 3. Corrections to the checked-in frontend plan

### 3.1 Edit 6 is unreachable as specified

The plan hooks the `Err(e)` arm of `find_function` (`overload_resolver.rs:96`). That code is
**not reached** for a Rust-backed receiver. The path is:

```
find_function:67
  → find_potential_function:553
    → get_candidate_banners:571
      → get_param_environments:504
        → get_outer_env_for_type
          → panic!("No outer env for type")   ← compiler_outputs.rs:640
```

A Rust-backed struct will never have an entry in `type_name_to_outer_env`.

**Fix, as first drafted (also wrong — recorded so it isn't re-proposed):** move the hook to
`find_potential_function`'s `successes.is_empty()` branch (~`:591`). That *is* reachable once
call-out #8's guard exists, but hooking any failure branch makes a Rust callee **invisible whenever
any Vale function of the same name matches loosely** — an overload-semantics decision made by
accident rather than chosen.

**Fix as landed (2026-07-25): a fourth candidate source, not a fallback.**
`push_rust_call_candidates` runs inside `get_candidate_banners`, alongside the calling env, the
param envs, and the placeholder extra-call envs, and contributes an
`ICalleeCandidate::PrototypeTemplata`. The synthesized prototype then flows through
`attempt_candidate_banner`'s **existing** `PrototypeTemplata` arm (`:475`) — which already does
`IFunctionNameT::try_from(..).parameters()` → `params_match` → scoring →
`narrow_down_callable_overloads` — so a Rust callee competes on equal footing with same-named Vale
functions. The machinery a Rust callee needs was already there for function bounds.

Two triggers, because a Rust callee arrives two ways:

- **receiver-keyed** — `my_vec.push(x)`: `param_filters[0]` is Rust-backed → `oracle.resolve_method`.
- **name-keyed** — `add_two_numbers(3, 4)`: *no* argument is Rust-backed, so the name is the only
  signal → `oracle.resolve_function`, with scoping delegated to the oracle. The receiver-keyed
  trigger structurally cannot fire for a free function; this was found by working the example, not
  by inspection.

Call-out #8's guard in `get_param_environments` is still **mandatory** — it is what makes the
candidate source reachable at all.

**Why this cannot live in an environment instead**, which is the more obvious design and is
foreclosed: `get_candidate_banners_inner` asserts `get_instantiation_bounds(..).is_some()` on every
Prototype candidate it accepts, but `get_outer_env_for_type` takes `&self` and env lookup has no
`&mut CompilerOutputs`, so an env could never register the bounds the very next line asserts. The
candidate source runs where `coutputs: &mut` is in scope, which is the only place that assert can be
satisfied.

### 3.2 `CoordT` no longer exists

The plan's §2 and §7 specify `ValeSig { params: &'t [CoordT], ret: CoordT }` and instruct
"reuse the real `CoordT` + effect types." **`CoordT` has zero occurrences in
`FrontendRust/src`.** The onion refactor dissolved it into reference wraps inside `KindT`
(`BorrowRefT`/`OwnRefT`/`ShareRefT`/`WeakRefT`, `types.rs:52-69`). `ValeSig` must be respecified
against `KindT` before any code is written against it.

### 3.3 Three constraints the plan doesn't account for

1. **`PrototypeT::param_types()` is name-derived.** `ast.rs:416` reconstructs params from
   `IdT.local_name` via `IFunctionNameT::try_from(...).parameters()`, panicking on a non-function
   name. So every Rust method signature must round-trip into an interned `IFunctionNameT` carrying
   `parameters: &'t [KindT]`. Arity-overloaded Rust items, `impl Trait` params, and
   where-clause-dependent signatures have no representation there.
2. **~10 `assert!(get_instantiation_bounds(...).is_some())` guard returned prototypes** —
   `overload_resolver.rs:218/435/457/488`, `call_compiler.rs:134/235`,
   `destructor_compiler.rs:63`, `array_compiler.rs:297`, `edge_compiler.rs:663`,
   `convert_helper.rs:213`. A fabricated Rust prototype must call
   `add_instantiation_bounds(..., empty)`. `compiler.rs:414-435 assemble_prototype` is the
   existing template for exactly this shape, and `is_parent:610`'s `IsaTemplataT` fast path
   already registers empty bounds — usable precedent.
3. **Drop routes through overload resolution.** `destructor_compiler.rs:52` sends `KindT::Struct`
   to `find_function("drop")`. Any Rust-backed value going out of scope reaches the method
   fallback asking rustc for a Vale-named `drop`. Needs an explicit pre-empt at call-out #21, not
   a fallback.

### 3.4 The oracle carrier: `Compiler`, not `CompilerOutputs`

The plan puts the oracle on `CompilerOutputs` (a constructor arg, `compiler_outputs.rs:52`/`:114`,
one call site at `compiler.rs:742`). The survey recommends **`Compiler` (`compiler.rs:110`)**
instead, and the argument is sound:

- `Compiler` is the *immutable-context* struct (arena, interner, keywords, options). An oracle is
  a query service — context. `CompilerOutputs` is the *output accumulator* drained into `HinputsT`
  at `compiler.rs:1291`; parking an input service on the output bag invites exactly the "is the
  oracle in `HinputsT`?" confusion the design exists to prevent.
- `Compiler` is available as `&self` at *every* seam, including the ones `CompilerOutputs` reaches
  only because `lookup_struct` already takes `compiler: &Compiler`.
- Cost is identical: 1 field + 1 `new` param + 1 call site either way, and `Compiler::evaluate`
  needs a param under the plan's version regardless, so both converge on the same edit count.

**Measured blast radius for the `Compiler` route:** 5 signature/struct edits + 9 `TypingPassCompilation::new` call sites = **14 mechanical edits**. Of 224 typing-test construction
sites, **7 change** (all in `compiler_project_tests.rs:248/305/361/420/473/528/581`); the other
**217 are absorbed by `compiler_test_compilation`** (`typing/test/compiler_test_compilation.rs:13`)
given a `NullOracle` default. Zero of the 184 `coutputs`-carrying signatures change. Zero of the
567 `pub fn`s in `src/typing/` change signature.

**Do not** put it on `TypingPassOptions` — it is currently lifetime-free, and a
`RustOracle<'s,'t>` would infect it and all 9 literal sites plus three downstream compilations.

**Do not** thread it as a literal per-call parameter — that is 184 signatures across 45 files. The
brief's "stack parameter" requirement is satisfied by the `Compiler` route: the oracle lives on a
stack-local `Compiler` created at `typing/compilation.rs:110` and dropped when
`get_compiler_outputs` returns. Verified: `HinputsT` (`hinputs_t.rs:54`) is built field-by-field at
`compiler.rs:1291` from `coutputs` getters, so nothing can leak in by accident.

### 3.5 "~zero `#[cfg]`" is achievable — and was deliberately not taken

> **Outcome (2026-07-25): the plan's §4 won, on a requirement this section didn't weigh.** Zero
> `#[cfg]` is achievable exactly as argued below, but it means the *core calls into the interop
> module* in the standalone build. The chosen requirement is stronger: under `valec`, nothing may
> reference `rust_interop` at all. So the module tree, the `Compiler` field and its threading, and
> every seam hook are gated, and `NullOracle` became `StubOracle` supplied by the test harness
> rather than a default. Enforcement is the green cfg-off build. The argument below is preserved
> because it is correct on its own terms and worth re-reading if that requirement is ever relaxed.
>
> One thing this section got right and is worth acting on: **a real Cargo feature is friction-free
> by comparison** (last paragraph). That was adopted 2026-07-25 — the switch is now a cargo feature
> rather than a bare `--cfg`, because `build.rs` cannot see a `RUSTFLAGS` cfg and needs to know the
> mode (to skip the LLVM-16 backend link, and to emit the rustc sysroot search path). Two switches
> would let the wrong combination compile cleanly and fail silently.

The plan's §4 mandates `#[cfg(rust_interop)]` on **every** seam hook, the `mod` declaration, and
the oracle field, and concedes an "honest residual" of gated one-liners in the core files.

That is not necessary. With `RustOracle` + `NullOracle` in an **unconditionally compiled** module
and every seam calling `oracle.foo(...) -> Option<...>` without a cfg, the typing pass needs
**zero** `#[cfg]`. The only cfg is on the rustc-backed *impl* — one line in
`typing/rust_interop/mod.rs`. Cost: the seam call is physically present in the core files (inert),
and the non-interop binary pays a vtable call that returns `None`.

The load-bearing constraint that makes this work is the plan's own rule: **no `'tcx`, `DefId`, or
`Ty` in any `RustOracle` signature.** If a rustc type appears in the trait, the cfg leaks straight
back into the typing pass.

Note also: "zero interop *text* in the core" is not achievable — `@NGSAX` forbids the global hook
table that would be the only alternative. Zero `#[cfg]` is the right goal; zero text is not.

**Build-system detail:** `FrontendRust/Cargo.toml` has **no `[features]` section**, one `[[bin]]`,
and zero `#[cfg(feature = ...)]` anywhere in `src/`. A bare `--cfg rust_interop` needs
`println!("cargo::rustc-check-cfg=cfg(rust_interop)");` added to `FrontendRust/build.rs` (currently
absent) or the `unexpected_cfgs` lint fires on `nightly-2025-12-09`, and every `cargo`/`cargo
nextest` invocation in `fire-commit-config.md` needs matching `RUSTFLAGS`. A real Cargo feature is
friction-free by comparison.

### 3.6 Don't plumb past `TypingPassCompilation` yet

`pass_manager/`, `instantiating/`, `simplifying/`, `final_ast/`, `backend_ffi/` are all unlinked
and the `valec` bin doesn't build. Adding oracle plumbing to `pass_manager::build` or
`FullCompilation` today means writing against dead code that will be re-derived when the onion arc
relinks those layers. **Add the oracle at `TypingPassCompilation::new` (`typing/compilation.rs:53`)
and stop.**

---

## 4. Prerequisites — fix before or alongside, not during

These are pre-existing defects that Rust interop will trip over. Each was found independently by
two or more surveys.

### 4.1 `is_primitive` has two divergent definitions

`types.rs:97` says `Str` is **not** primitive. `compiler.rs:1683` says it **is**. The export/extern
ABI gate at `compiler.rs:1553` uses the latter. Both are called "is this a value?" and they
disagree. Additionally `compiler.rs:1683` is `unimplemented!()` on all four ref arms, and
`instantiating/ast/types.rs:106` `KindIT::is_primitive` is `panic!("Unimplemented")` with zero
callers.

### 4.2 Three export errors will fire spuriously

`ExportedFunctionDependedOnNonExportedKind` (`compiler_error_reporter.rs:77`),
`ExternFunctionDependedOnNonExportedKind` (`:83`), `ExportedKindDependedOnNonExportedKind` (`:89`).
A Rust-backed kind is neither Vale-exported nor Vale-extern, so all three reject it. Needs a
target-partitioned universe check at call-out #28 and a `CantExportRustBackedKindT` replacement.

**And one fires *silently wrong*:** `compiler.rs:1550` iterates **all** members of an exported
shared struct to prove transitive exportability. Under a `pub`-fields-only view it
under-approximates and **passes when it should fail**. No diagnostic. This is the single worst
failure mode found.

**The fix is two distinct field queries, not one.** Field *read* is fine with a partial view; field
*enumeration* is not, and conflating them is what produces the silent pass above:

| query | answers | when answerable |
|---|---|---|
| `field_by_name(id, name)` | one field's type + index + visibility | **always** — `pub` fields are genuinely reachable to an external consumer; private gets a clear error |
| `all_fields(id)` | the complete member list | **only when the Rust type has no private fields** — otherwise it must refuse |

Every *enumeration* site must call the second and be able to fail loudly: the export walk here,
plus construction, destructuring, drop synthesis, and layout. `field_by_name` is what the `Dot`
handler (#10) wants. Giving both sites one query is how the truncated-list bug gets written.

Also at this site: three of `ensure_deep_exports`'s errors fire spuriously on a Rust-backed kind
because such a kind is neither Vale-exported nor Vale-extern. The `ICitizenAttributeT::Extern`
check (#29) is the existing escape hatch to generalize.

### 4.3 Panic-first culture at the seam sites

Several seams currently `panic!` where they must return `Err`:

- `expression_compiler.rs:799` — `panic!("CouldntFindMemberT")`. A private-Rust-field access would
  abort the compiler. The error variant already exists; it just isn't raised.
- `overload_resolver.rs:774`, `call_compiler.rs:217`, `abstract_body_macro.rs:74` —
  `panic!("CouldntFindFunctionToCallT")`.
- `overload_resolver.rs:721` — `panic!("No candidate is a clear winner!")`.
- `get_candidate_banners_inner` panics on 5 of 7 templata shapes (`:200/204/209/214/225`).
- `compiler_outputs.rs:534/565/582/640/650` — the five lookup accessors.

**60+ sites assume "if I have a `StructTT`, a definition exists."** Converting the lookup family
from panicking to `Option`-returning is the prerequisite refactor. Per `valec-reviewer`: never
discard `Err` payloads.

### 4.4 `TookWeakRefOfNonWeakableError` is never raised

Its test (`after_regions_error_tests.rs:495`) is `#[ignore]`d with "typing pass produces Ok where
the error is expected." So today `&&rustThing` would be **accepted** and produce garbage rather
than erroring. Fix before Rust types can be non-weakable.

### 4.5 The IR cannot represent Rust's primitives

**At the typing level [LIVE]** — `typing/types/types.rs`, and this is where a `Ty<'tcx>` → `KindT`
lowering hits it *first*, so it is not a dark-pass concern:

- `IntT { bits: i32 }` has **no signedness**, so `u32` lowers to the same `KindT` as `i32` — a
  silent conflation, not a missing case.
- `FloatT` is a **unit struct** with no width at all, so `f32` has no representation.
- There is **no unsized concept** (zero occurrences of `Sized`), so `str` / `[T]` / `dyn Trait`
  cannot be value types.
- There is **no Send/Sync/Unpin property** anywhere in the frontend (also zero occurrences), so
  thread-safety and address-stability are new concepts to add, not facts to look up.

Vale's `int` is 32-bit — the interop test asserts `KindT::Int(IntT { bits: 32 })` — so a fixture
Rust function must be written `i32`, not `i64`.

**At the H level [DARK]** — the same gaps, restated where the backend sees them:

- `FloatHT` (`final_ast/types.rs:163`) has **no width field** — f64 assumed everywhere.
- `IntHT` has **no signedness bit**.
- Array length is hard-coded to `IntHT { bits: 32 }` (`final_ast/instructions.rs:158-159`) while
  Rust's `len()` returns `usize`.

Rust `f32`, `u64`, `usize` have no encoding at either level. **No oracle papers over this — the IR
must grow fields.** A representational gap, not a porting gap.

**Interim behavior (decided 2026-07-25): panic, with a message that names the type and the gap** —
e.g. `"cannot lower Rust u64: IntT has no signedness"` — so the panic doubles as the spec of what's
missing. Long-term the answer is to grow the IR rather than to diagnose. Note arch §8.10 accepts a
*permanent* residual regardless ("complete for identity, not for full Rust type expressiveness" —
HRTBs and complex bounds fall back to annotation files, §24), and that residual will eventually want
a real diagnostic rather than a panic.

### 4.6 `member_index` is a declaration ordinal, not a memory index

Five open-coded copies of `members.iter().position(|m| m.name == *name)`
(`simplifying/load_hammer.rs:173/227/352`, `mutate_hammer.rs:144/190`) produce the `member_index`
that crosses to the backend. Under `#[repr(Rust)]` **rustc reorders fields**, so declaration index
≠ `memory_index`. This is not a missing-definition problem; it is a **silent wrong-offset
correctness bug**. Wants a single `member_index_of(kind, name)` helper returning a memory index.
[DARK]

---

## 5. Model mismatches — decisions, not code

### 5.1 Name collision (the biggest live hazard)

`StructTemplateNameT` is `{ human_name }` and `InterfaceTemplateNameT` is `{ human_namee }` —
**no package, no location** (`names.rs:1408-1416`). And `PackageEnvironmentT::lookup_*`
(`environment.rs:882/905`) walks **every** global namespace concatenating results, **ignoring
`get_only_nearest`** (the parameter is literally `_get_only_nearest` at `:876`).

So a Rust `Vec` plus a Vale stdlib `Vec` yields two results →
`panic!("Too many with name")` at `environment.rs:164`.

There is no escape hatch, because **`import X.Y.Z` grants no visibility today.** It means exactly
one thing: "also load package `X.Y` from disk." The `importee_name` is *discarded* at
`lex_and_explore.rs:95-112`; `grep "\.imports"` finds one hit, in a test traversal.

Options: gate the rust namespace behind real import scoping (new mechanism); add disambiguation to
the template names; or adopt a distinct human-name convention for Rust items. **This needs deciding
before call-out #2 lands.**

### 5.2 A Rust env entry needs a synthesized `StructS`

`IEnvEntryT` (`typing/env/i_env_entry.rs:12-20`) has no arm for a definition-less type.
`entry_to_templata` (`environment.rs:400-432`) builds
`StructDefinitionTemplataT { declaring_env, origin_struct: &'s StructS }`, and its `eq`/`hash`
(`templata.rs:202-211`) compare `origin_struct.range` and `.name`. So a Rust struct env entry
requires a **synthesized postparsing `StructS`** in the scout arena with a stable synthetic
`RangeS`. That is the real work item behind call-out #2, and it is orthogonal to naming.

Related: `ExternFunctionTemplataT` (`templata.rs:414`) is the existing precedent for "a template
with no AST behind it" — and its `tyype()` is `panic!("Unimplemented")` (`templata.rs:124`), and it
isn't handled in `solve_call_rule`. Nobody has walked this path.

### 5.3 The lossy-args problem, and why the arch doc's fix may be backwards

`vale-rust-interop-architecture.md` §8.10 names this as Option A's sharpest weakness: Vale stores
`[Kind(i64)]` where rustc's real args are `[i64, Global]` plus lifetime slots, so feeding rustc back
requires reconstructing `GenericArgs` via `generics_of` + `mk_args` + `re_erased`.

The arg list has a narrow waist — one write family (`make_struct_name`/`make_interface_name`,
`names.rs:594`/`:629`, fed from five sites in `struct_compiler_generic_args_layer.rs`) and one read
accessor (`IInstantiationNameT::template_args()`, `names.rs:421`). That is good.

But **`ITemplataT` has no `Region` variant** (`templata.rs:67`). Regions are hardcoded
`RegionT::Default` in six solver sites. **A Vale arg list literally cannot carry a lifetime today.**

The alternative worth weighing: **store the full rustc arg list losslessly and do the elision at
scout time**, reusing Vale's existing default-generic-argument machinery, which already handles
"fewer args supplied than params" (see the comment at `struct_compiler_generic_args_layer.rs:55-57`
and `assemble_predict_rules`, `templata_compiler.rs:162`). Then the Vale name is lossless and no
boundary reconstruction is needed. Cost: adding `ITemplataT::Region`, touching `tyype()`,
`sanity_check_conclusion`, the humanizer, the instantiator, and the interner.

Reconstruction is also ambiguous whenever a Rust type has more than one defaulted param, or a
defaulted param before a non-defaulted one — it needs a stable Vale-slot ↔ rustc-param projection
map that nothing currently stores.

### 5.4 Vale computes a subtyping least-upper-bound; Rust has none

`expression_compiler.rs:955-976` reconciles `if`/`else` branch types by intersecting both operands'
parent sets, then `panic!`s on zero or more than one common ancestor. Rust has no subtyping
lattice — `if { Dog } else { Cat }` requires an explicit `Box<dyn Animal>`. Any Rust-typed branch
pair hits one of those panics.

### 5.5 Vale materializes vtables eagerly; Rust resolves impls on demand

- `compile_i_tables` enumerates every impl of every interface via
  `get_child_impls_for_super_interface_template`; `instantiator.rs:844` redoes it at mono. There is
  no way to answer "all impls of a Rust trait" — blanket impls make the answer unbounded.
- **Vtable slot order is a Vale-computed global.** `make_interface_edge_blueprints` derives slot
  indices; `expression_hammer.rs:836` resolves a call to a slot by signature *position*. Rust's
  vtable layout is unspecified. Calls into `dyn Trait` must go through a Rust-side shim, never
  `InterfaceCallH`.
- `assert!(oks.len() <= 1)` at `impl_compiler.rs:639` — at most one impl may relate a (sub, super)
  pair. Rust has blanket + generic impls.
- `look_for_override` (`edge_compiler.rs:253`, ~440 lines) synthesizes and compiles a *dispatcher
  function* per (impl, abstract method), with dependent/independent rune splitting and a nested
  `find_function`. No Rust counterpart; must be bypassed entirely, not fed a fallback.
- Weakability parity (`impl_compiler.rs:330-341`) requires sub and super to agree on `weakable`.
  Rust types have no such attribute.

**Corollary for Rust-enum-as-`InterfaceTT`:** the mapping works in the typing pass's
variant-to-sum upcast direction, but everything downstream of `compile_i_tables` assumes an
interface is a vtable dispatch target. Every interface must get a blueprint — a fiction for an
enum — and `struct_hammer.rs:62/:332` plus `expression_hammer.rs:833` all
`.expect("vassertSome interface_to_edge_blueprints")`.

### 5.6 Conformance is not yet a solver constraint — an opportunity

`CallSiteCoordIsaSR`, `DefinitionCoordIsaSR`, and `CoordSendSR` are entirely commented out
(`compiler_solver.rs:662-860`, `infer_compiler.rs:752-790`). `ITypingPassSolverError::IsaFailed` is
declared and matched but **never constructed**, so its two consumers
(`call_compiler.rs:73/:84`) are dead. `where implements(...)` parses but
`postparsing/rules/rule_scout.rs:152` has no arm for it → `panic!`.

Rust interop *needs* a conformance constraint (`T: Trait` is pervasive). Writing that rule
rustc-aware from the start is the cleanest injection point available — far cleaner than
retrofitting one.

### 5.7 Properties with no Rust answer

| Vale property | Where | Decision needed |
|---|---|---|
| `SharednessT::Shared` (RC'd immutable citizens) | `types.rs:9`, `citizens.rs:68/110`, `struct_compiler.rs:290` | **Not an oracle call** — Rust has no type-level notion of "refcounted by the language" (`Rc`/`Arc` are wrapper types, not a property of `T`). Declare it at import time via `CompilerOutputs::declare_type_sharedness`, so `struct_compiler_get_sharedness`'s `lookup_struct` never runs for a Rust id. `struct_compiler_core.rs:84-92` already hard-panics on `extern` + `share` ("must be Own+Inline"), which forces the answer to `Single` and **blocks `Rc`/`Arc`-shaped imports outright** — and in turn means a Vale `share` struct cannot contain a Rust type without an explicit Rust-side `Arc`. Note `get_sharedness` already special-cases `Str → Shared`, so the mapping is already non-uniform. |
| `weakable` | `citizens.rs:67/109` | No rustc query exists. Answering `false` silently forbids Rust types from implementing any `weakable` Vale interface (`WeakableImplingMismatch`). |
| `sealed` | `compiler_outputs.rs:534` | rustc exposes no `is_sealed`; the Rust sealed-trait pattern is a convention. And `lookup_sealed` **panics** on a missing entry, so every imported Rust trait must get one. |
| `Send` / `Sync` | — | **Zero occurrences** in `typing/`, `instantiating/`, `simplifying/`. A *new* concept to add, not one to map. |
| `Sized` / `?Sized` | — | **Zero occurrences.** No way to represent `str`/`[T]`/`dyn Trait` as a value type. `type_hammer.rs:75` hardcoding extern structs to `InlineH` is almost certainly wrong for `Box<dyn Trait>`. |
| `Unpin` / address stability | — | Vale moves locals freely (`resultify_expressions`, `make_temporary_local`) and has no pinning concept. Any `!Unpin` Rust type cannot be a bare Vale local, and there is nowhere to express the constraint. |
| Drop cannot unwind | `destructor_compiler.rs:107-112` | Vale requires drop to return `Void`/`Never`. Rust `Drop::drop` can panic. Needs an abort-shim decision. |
| Move-out-of-member | `expression_compiler.rs:684` | Vale forbids it; Rust permits it for non-`Drop` types. |
| Branch-symmetric move sets | `expression_compiler.rs:1009-1016` | Both `if` branches must move *exactly* the same variables, else `panic!`. Rust unions the sets and uses drop flags. **Vale has no drop-flag concept.** |
| `dyn` drop | `destructor_compiler.rs:71` | `unimplemented!()`. Rust puts the drop pointer in the vtable; Vale synthesizes an abstract method. |
| Generic `T` drop | `destructor_compiler.rs:74` | `unimplemented!()`. Generic drop goes through the `where func drop(T)void` **bound-prototype** machinery, so `__vale_drop<T>` must be resolvable as a `PrototypeT` the instantiator carries through `InstantiationBoundArgumentsT` — threading `templata_compiler.rs:623-740`, `infer_compiler.rs:419`, `compiler_outputs.rs:234-264`, `impl_compiler.rs:390`. **This is the largest single piece of plumbing the plan implies, and it is not a chokepoint.** |

### 5.8 Do not revive `get_compound_type_mutability`

`struct_compiler.rs:279` and `compiler.rs:1702` are both
`panic!("Unimplemented: Slab 15")`, with a commented body deriving sharedness transitively from all
members. Asking rustc to walk private fields of a foreign `#[non_exhaustive]` type to answer "are
all your fields Share?" isn't just hard — it's semantically wrong. **Sharedness must be declared at
the boundary, not inferred.**

---

## 6. The dark passes

Design now, land when the onion arc relinks them. All **[DARK]**.

### 6.1 Instantiator

`translate_prototype` (`instantiator.rs:970`) already has a **three-way fork** — `FunctionBound`
(substitute), `ExternFunction` (pass through opaquely, no queue), Vale (enqueue). **A Rust callee
is a fourth arm**, and `:1018` is where `monouts.rust_deps.push(...)` goes instead of
`new_functions.push(...)`. The shape is right; it needs one case.

Eleven guard sites: `:970` (the fork), `:797` `translate_function_callsite` (`vassert_one`),
`:953` (unconditional `get_instantiation_bound_args`, must move inside the match), `:2362`/`:2367`
`translate_kind`, `:657`/`:665` `find_struct`/`find_interface` (`assert_eq!(matches.len(), 1)`),
`:641`/`:581` the callsite translators, `:2232` `get_sharedness`, `:2124` `translate_impl_id`,
`:1769`/`:1965` Construct arms, `:873` `assemble_placeholder_map_inner`.

Note `hinputs.get_instantiation_bound_args` (`hinputs_t.rs:136`, a bare `.unwrap()`) is called
unconditionally at **12 sites**.

### 6.2 `reachability.rs` — greenfield, write it Rust-aware

100% stub (8 `panic!("Unimplemented: Slab 15")`), **zero callers**. The signatures are already
fixed: `find_reachables` + six `visit_*`. Each `visit_*` wants an early
`if is_rust_backed(x) { reachables.rust_deps.insert(path_of(x)); return; }` before descending, and
`Reachables` (`:8`) gains a 7th field. **Do not write it Vale-only and patch it.**

Two mismatches to note: it walks the **pre-instantiation `CompilerOutputs`** keyed on placeholdered
`SignatureT`/`StructTT`, i.e. definition-level reachability, whereas "which Rust items do we report
to rustc" is per-Instance. And `TypingPassOptions.tree_shaking_enabled`
(`typing/compilation.rs:34`) is set `true` by every test but **read by nothing**.

### 6.3 Hammer / final AST — mostly already shaped

`type_hammer.rs:49` already forks on `hinputs.kind_externs.contains_key(...)` →
`KindHT::OpaqueHT`, and `:75` forces extern structs to `(OwnH, InlineH)`. That is the existing hook
a Rust type rides. Needed: generalize "extern" to "extern or rust", and take
size/align/`backend_repr` from rustc's `layout_of` rather than **assuming `InlineH`, which is wrong
for anything unsized behind a pointer** (`Box<dyn Trait>`); drop the `hinputs.lookup_struct` call in
`translate_opaque_i` (`struct_hammer.rs:176`) since a Rust type has no `StructDefinitionI` shell;
implement `hammer.rs:332`'s live rust-package panic; and `hammer.rs:226`'s `mangle_func` (currently
`panic!("Unimplemented")`, with the Scala rule commented in).

**`translate_members` (`struct_hammer.rs:197`) likely needs a third path, not a relaxed assert.**
It is the only place a struct's field list becomes backend layout. The existing extern path
(`translate_opaque_i`) asserts `members.is_empty()` and emits an `OpaqueHT` with **no layout at
all**; the normal path translates a member list a Rust type doesn't have. A Rust type with visible
`pub` fields fits neither — it has real layout (rustc's) *and* real fields, so it wants a path that
carries layout without a Vale member list.

`backend_ffi/metal_lowerer.rs:262` is `panic!("KindHT::OpaqueHT not yet implemented")` and looks
reachable the moment any extern kind is emitted. The comment at `:28-34` explains the cost: the C++
Backend's per-region `translateType`/`getControlBlock` exhaustive `dynamic_cast` switches mean
**adding a Rust-backed Kind touches every region implementation** in `Backend/src/region/`. That is
the largest single item on the backend side.

### 6.4 The cache is entirely unbuilt

**No `serde`, no `bincode` anywhere in `FrontendRust`** — not in `Cargo.toml`, not in any `.rs`.
The frontend→backend wire is FFI handle-passing (`backend_ffi/metal_cache.rs`), not serialization.
`src/von/` is the retired JSON path and is unlinked.

`SimpleId` (`final_ast/types.rs:321`) — `{steps: [{name, template_args}]}` — is exactly the right
stable, session-independent path type, and the Backend already consumes it. But it exists **only at
the H level**; `IdT`/`IdI` have no equivalent, and `simplify_name` is partial (panics on
`InterfaceName`/`InterfaceTemplate`, `name_hammer.rs:159-162`). Hoisting a `SimpleId`-shaped path
down to the typing level would make one value serve as cache identity, rustc report, and backend
path.

Everything is arena-allocated `&'t` with `MustIntern` witnesses and pointer-identity equality, so
serialization is a graph→index-table lowering plus re-interning on load. **Budget this as its own
project, not a task.**

---

## 7. Recommended order

**Landed 2026-07-25** (revising steps 3, 4, 6 of the original order below):

1. ~~**Respecify `ValeSig` against `KindT`**~~ (§3.2) — done.
2. ~~**Land the oracle carrier**~~ — done, on `Compiler` (§3.4), but **fully `#[cfg]`-gated rather
   than cfg-free** (§3.5). The requirement chosen was stronger than the map recommended: under
   `valec`, nothing may reference the interop module *at all*.
3. ~~**The call seam**~~ — done, as a **candidate source** rather than either fallback (§3.1), with
   both receiver-keyed and name-keyed triggers. The free-function trigger is tested end-to-end
   against a fixture oracle; the method trigger is written but untested, because it needs a
   Rust-backed `StructTT` and therefore `resolve_path` + `kind` first.

**Remaining order:**

1. **Milestone 2 — replace the fixture oracle with a real `TyCtxt`.** rustc hosts and the typing
   pass runs inside `Callbacks::after_expansion`; see the frontend plan §9. This is what makes
   @EarlyBinder testable at all, and it is where `Ty<'tcx>` → `KindT` lowering starts hitting §4.5.
2. **Prerequisites** (§4) — unify `is_primitive`; convert the `compiler_outputs` lookup family from
   `panic!` to `Option` (this is what retires call-out #8's guard and the six-plus sibling guards it
   implies); raise `CouldntFindMemberT` instead of panicking at `expression_compiler.rs:799`. Pure
   cleanups with independent value.
3. **Call-outs #1, #2, #11** — get a Rust *type* interned, which is the gate on everything below.
   Requires §5.2 (synthesized `StructS`) and forces §5.1 (name collision), deliberately deferred
   until now because no Rust type existed to collide.
4. **The method trigger, tested** — `my_vec.push(x)`, now that a receiver kind exists.
5. **Call-out #10** — `pub` field access, with the `field_by_name` / `all_fields` split (§4.2).
6. **Decide §5.3** (lossless args vs boundary reconstruction) before generics land — it changes
   `ITemplataT`, so it wants deciding before there are many arg-list producers, not after.
7. Everything else as the capabilities land, one guard at a time — which is the plan's per-question
   model, and it is the right model. The correction is only to the *count* and to which specific
   sites are load-bearing.

---

## 8. Provenance

Twelve read-only surveys, 2026-07-24, each independently reading `FrontendRust/src`:
definition lookup; field/member queries; call and overload resolution; subtyping/impls/dispatch;
layout/size/ABI; drop/lifecycle; solver/generics/templata; names/env/imports; post-typing
lowering; existing extern/export FFI; type-property predicates; plumbing/diagnostics/tests.

Findings confirmed by two or more independent surveys are marked as such in the text. The
`is_primitive` divergence was found by three; the `get_outer_env_for_type` panic path by four.

---

## 9. Site index — the former `// ZRI` markers

For a period these 21 sites carried `// ZRI:` comments in the source. **They were removed
deliberately (2026-07-25):** Rust interop is meant to stay contained to the interop directories so
the main compiler reads as a Vale compiler, and 21 interop annotations scattered across `typing/`,
`instantiating/`, and `simplifying/` worked against that. This table is their replacement — it is
what `grep ZRI` used to answer, so **this document is now the only record** and every marker's
content was folded into the sections cited here before deletion.

Line numbers are anchors as of `experimental-4` @ `af3a3c17a` and will drift; the function names
won't.

| # | Site | Question | Covered in |
|---|---|---|---|
| 1 | `typing/compiler.rs:309` `lookup_templata_imprecise` | resolve a Rust path to a templata | #11, §5.2 |
| 2 | `typing/compiler.rs:1436` `ensure_deep_exports` | may this type cross a boundary | #28, #29, §4.2 |
| 3 | `typing/compiler_outputs.rs:534` `lookup_sealed` | is this trait/enum sealed | #7, §5.7 |
| 4 | `typing/compiler_outputs.rs:639` `get_outer_env_for_type` | what env holds this type's methods | #5, §3.1 |
| 5 | `typing/templata_compiler.rs:446` `substitute_templatas_in_kind` | substitute generics into a Rust kind | #25 |
| 6 | `typing/templata_compiler.rs:1123` `IRuneTypeSolverEnv::lookup` | rune-typing's view of a Rust item | #13 |
| 7 | `typing/convert_helper.rs:139` `convert_via_upcast` | upcast a Rust enum variant / to a trait | #18 |
| 8 | `typing/types/types.rs:47` `KindT` | signedness, float width, unsized, Send/Sync/Unpin | §4.5 |
| 9 | `typing/reachability.rs:8` | which Rust items does a body reach | §6.2 |
| 10 | `typing/citizen/struct_compiler.rs:290` `struct_compiler_get_sharedness` | sharedness of a Rust type | §5.7, §5.8 |
| 11 | `typing/citizen/impl_compiler.rs:507` `get_parents` | what does this type implement | #17 |
| 12 | `typing/citizen/impl_compiler.rs:575` `is_parent` | does sub implement super | #16, §5.5 |
| 13 | `typing/function/destructor_compiler.rs:41` `Compiler::drop` | needs_drop + drop-glue symbol | #21, §5.7 |
| 14 | `typing/expression/expression_compiler.rs:796` `Dot` handler | `pub` field type + index | #10, §4.3 |
| 15 | `typing/expression/expression_compiler.rs:1940` | weakability of a Rust kind | §4.4, §5.7 |
| 16 | `typing/templata/templata.rs:65` `ITemplataT` | no `Region` variant → arg-list storage | §5.3 |
| 17 | `typing/infer/compiler_solver.rs:1224` `solve_call_rule` | apply generic args to a Rust template | #12 |
| 18 | `instantiating/instantiator.rs:970` `translate_prototype` | the fourth arm for a Rust callee | §6.1 |
| 19 | `simplifying/load_hammer.rs:172` | member **memory** index (5 open-coded copies) | §4.6 |
| 20 | `simplifying/struct_hammer.rs:197` `translate_members` | struct layout for the backend | §6.3 |
| 21 | `simplifying/type_hammer.rs:65` `translate_coord` | size / align / inline-vs-boxed | §6.3 |

Three of these are **not** oracle calls but policy defaults, and the map says so at each: sharedness
(#10), weakability (#15), sealedness (#3). Two carry **silent-wrongness** warnings rather than
panics, which are the ones least safe to lose: the export member walk (#2) passes when it should
fail, and the member index (#19) reads the wrong field under `#[repr(Rust)]`.
