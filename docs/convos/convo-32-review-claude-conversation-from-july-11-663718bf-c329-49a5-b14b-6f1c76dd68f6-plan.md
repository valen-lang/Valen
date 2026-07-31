# Plan document

Source: `/Users/verdagon/.claude/plans/spicy-waddling-quasar.md`
Session: 663718bf-c329-49a5-b14b-6f1c76dd68f6

---

# Plan: Three-phase call-checking (candidate → pure inference → subtyping/coercion)

## Context

Argument-type inference for function calls is currently **absent** in the Vale typing
pass. The old mechanism — `InitialSend` (an arg→param coercion relationship) lowered into
a `CoordSendSR` solver rule — is fully dead: the solver signatures dropped `initial_sends`,
`assemble_initial_sends_from_args` builds a `Vec<InitialSend>` that nothing consumes, and
`CoordSendSR`/`CallSiteCoordIsaSR`/`complex_solve` were retired by the onion refactor. So a
call like `foo<T>(x: &T)` with a `Dog` argument has **no path** to conclude `T=Dog`.

We are replacing it with a **strict three-phase pipeline** (validated this session against
the Vale code and against how rustc does it):

1. **Candidate matching** — pick which function(s) the call resolves to, by namespace.
2. **Pure inference solve** — infer the callee's generic parameters from the arguments,
   with **no** subtyping/coercion in the loop. Mechanism: feed each argument's
   *reference-stripped value kind* as an exact `InitialKnown` for the callee parameter's
   `value_type_rune`.
3. **Subtyping + coercion** — after the generics are solved and the expected parameter
   types are concrete, check each actual argument against the now-concrete expected
   parameter (nominal upcast + ref coercion) and insert the coercion nodes.

Why this fits Vale (and why rustc *can't* do it): rustc interleaves inference and coercion
and is bidirectional, because Rust closures need their expected type to be typed and Rust
has no nominal subtyping. **Both blockers are absent in Vale**: Vale lambdas are *templates*
(the closure-struct value kind is knowable from code-location + captures alone; the lambda's
params defer to invocation — LDNEIR), so passing a lambda never needs the expected type; and
Vale *does* have nominal interface subtyping, which genuinely wants its own phase 3.

**Design decisions already locked** (this session): reference-stripping needs **no peeling**
(the `value_type_rune` split pre-computes it); multi-arg-same-rune **rejects on conflict**
(no LUB, no cross-arg coercion) — the clean, order-independent behavior; lambda args are
deferred, not bidirectional.

## Prerequisite (named, not detailed here)

This pipeline runs on a **compiling** typing pass. Today the tree is RED (~114 errors). Two
things must land first, tracked separately:
- **Typing compiles** — the value-model dissolution (`CoordT`→`KindT` with `BorrowRef`/etc
  wrap variants; delete `CoordT`/`OwnershipT`/`LocationT`) plus the remaining onion buckets
  (macros, `ParameterS.pattern` reads in body-synthesis, `Eq`/`Hash` derives). Phase 3's
  coercion rewrite is entangled with this.
- **Closure-param split fix** — `create_closure_param` (`postparsing/function_scout.rs`,
  ~:935) currently strands the closure param's outer `BorrowRef` in the function-level rule
  bucket, so its `value_type_rune` isn't actually ref-stripped. Fix it to split like a
  normal/struct param: `value_type_rune` = the closure-struct kind, the `BorrowRef` in
  `type_outer_ref_rules`. Required so phase-2 seeding is uniform (no special-casing).

## Phase 1 — Candidate matching (small)

`get_candidate_banners` (`typing/overload_resolver.rs:149`) already collects candidates from
the arg types' namespaces (`get_param_environments`). The one addition: to find a
supertype-param candidate for a subtype arg (`foo(&Animal)` called with a `Dog`), candidate
gathering must also look in the arg kind's **ancestor** namespaces — walk the impl/parent
graph (reuse `is_parent`/the impl lookup in `citizen/impl_compiler.rs`) to include parent
kinds' namespaces in the candidate search. This is namespace-level subtyping (where to look),
NOT solve-level — it does not violate "no subtyping in phase 2."

## Phase 2 — Pure inference via arg-as-`InitialKnown` (the centerpiece)

**Replace the dead send machinery with value-kind knowns.** Rewrite
`assemble_initial_sends_from_args` (`typing/function/function_compiler_solving_layer.rs:769`)
into an `assemble_known_value_kinds_from_args` returning `Vec<InitialKnown>`:
- Zip `function.params` with the call `args: &[Option<CoordT>]` (preserve the `Option`
  tolerance for positional misalignment with synthesized self/closure/magic params).
- For each `(param, Some(arg))`, if the param is **seedable** (below), emit
  `InitialKnown { rune: param.value_type_rune, templata: Kind(<ref-stripped arg value kind>) }`.
  Today ref-stripping = `arg.kind` (the `CoordT.kind` field, dropping ownership); post-value-
  model it becomes a `value_kind_of`/`peel_wraps` helper — centralize it in one place.
- Drop the `ArgumentRuneS` sender machinery entirely.

**The selectivity rule (the linchpin — seed only generic value runes).** A param's
`value_type_rune` is seedable iff its value type *references a generic parameter* of the
callee; skip it if it's concrete-`Lookup`-determined:
- `&T` (T identifying) → `value_type_rune` **is** T's rune, `value_type_rules` empty → seed
  directly. (Verified: `postparsing/rules/templex_scout.rs` declared-rune early-return.)
- `&List<T>` → `value_type_rune` = the `CallSR` result; seed `List<Dog>` and the **backward
  `CallSR`** concludes `T=Dog` (this bidirectional decomposition already works —
  `typing/infer/compiler_solver.rs::solve_call_rule` `Some(result)` branch, ~:1185-1295).
- `&Animal` (concrete) → `value_type_rune` is `Lookup(Animal)`-pinned → **skip**; seeding
  would `SolverConflict`. These are pure phase-3 territory.
- `&List<int>` (compound, all-concrete args) → forward-determined → **skip**.

Operationally: seed iff `value_type_rune` (transitively through `value_type_rules`) mentions
one of `function.generic_params`' runes. This is a static property of the callee signature.

**Feed the knowns at the 4 solve sites.** Each currently binds a dead `initial_sends`; instead
extend the `initial_knowns` handed to the solver:
`function_compiler_solving_layer.rs` — `evaluate_templated_function_from_call_for_banner`
(~:107), `evaluate_templated_light_banner_from_call` (~:215),
`evaluate_generic_function_from_call_for_prototype` (~:397, knowns built ~:412),
`evaluate_generic_virtual_dispatcher_function_for_prototype_solving` (~:530).

**Reject-on-conflict is free.** A duplicate/mismatched seed produces the engine's
`SolverConflict` (`solver/simple_solver_state.rs` commit-step), which the candidate treats as
"doesn't apply." No LUB, no cross-arg coercion — matches the locked decision. No extra code.

**Lambda args need nothing special.** The arg's value kind is the closure-struct kind
(`LambdaCitizenNameT`, no template args — `typing/citizen/struct_compiler_core.rs:343-354`),
self-determined from code-location + captures. Seed it like any other value kind; the
lambda's own params resolve later at invocation. No expected-type feedback.

**Delete the dead types:** `InitialSend` (`typing/infer_compiler.rs:88`) and `ArgumentRuneS`,
plus their now-unused imports (`compiler_outputs.rs`, `function_compiler_solving_layer.rs`,
`citizen/struct_compiler_generic_args_layer.rs`, `expression/pattern_compiler.rs`).

## Phase 3 — Subtyping + coercion (sketched; downstream of the value model)

After phase 2 resolves the candidate to a **concrete** prototype, check each actual arg
against its concrete expected param. This already runs post-solve today:
`params_match` (`overload_resolver.rs:108`) → `is_type_convertible`
(`templata_compiler.rs:1142`), whose kind branch calls `is_parent`
(`citizen/impl_compiler.rs:568`) — the **nominal subtyping** check, which survives the onion
change untouched (it's about the impl graph, not refs). Work here:
- **Onion-rewrite `is_type_convertible`** — its `{ownership, region, kind}` decomposition
  becomes wrap-matching once `CoordT` dissolves; the `is_parent` kind branch stays.
- **`convert()`** (`typing/convert_helper.rs:50-188`) — the coercion-node emitter (implicit
  clone, auto-alias, upcast). Rewrite its `(source_ownership, target_ownership)` dispatch to
  `(source_wrap, target_wrap)` over the onion wraps, per the handoff coercion table; insert
  `UpcastTE` for nominal upcast. This is the piece entangled with the value model, so it lands
  with / after that work.
- **The unimplemented lambda-receiving-rune path** at `overload_resolver.rs:433`
  (`assert!(... is_empty(), "implement: lambda receiving rune templatas")`) — wire when
  exercised.

## Suggested execution order
1. Closure-param split fix (prerequisite, small, independent).
2. Phase 2 — the `assemble_known_value_kinds_from_args` rewrite + selectivity + 4 sites +
   delete `InitialSend`/`ArgumentRuneS`. (Depends only on typing compiling; the backward
   `CallSR` it relies on already exists.)
3. Phase 1 — ancestor-namespace candidate gathering.
4. Phase 3 — `is_type_convertible`/`convert()` onion rewrite (with/after the value model).

## Verification

Can't run until typing compiles (prerequisite). Once green, exercise end-to-end via the
FrontendRust test suite (`cargo test --manifest-path FrontendRust/Cargo.toml --lib`), driving:
- **Generic inference from args** — `foo<T>(x &T)` called with a concrete arg infers `T`; a
  nested `foo<T>(x &List<T>)` with `List<Dog>` infers `T=Dog` (the backward-`CallSR` path).
- **Reject-on-conflict** — `foo<T>(a T, b T)` with `(Dog, Cat)` is rejected (no `T`), and with
  `(Dog, Animal)` is also rejected (locked decision), order-independently.
- **Subtype arg** — `foo(x &Animal)` accepts a `Dog` (phase 3 `is_parent` + `UpcastTE`), and
  phase 2 did not seed the concrete `Animal` value rune.
- **Lambda arg** — passing `x => x+1` / `_ + 1` type-checks without needing the expected param
  type; the lambda's params resolve at invocation.
- Re-author the 6 preserved rune-type-inference fixtures
  (`FrontendRust/docs/regression-fixtures-from-retired-higher-typing.md`) that exercise
  param-position / template-call rune inference.

## Notes for execution
- No file edits until the architect gives the go (standing instruction this session).
- Phase 2 is the piece with the clearest path and least value-model entanglement — start there
  (after the closure-param fix) once typing compiles.
- Keep ref-stripping behind one `value_kind_of` helper so the CoordT-today → wrap-peel-later
  change is a one-line swap, never an ad-hoc walk in call-checking logic.
