# Plan: Group-generic closures (borrow-checking closures that capture references)

**Status:** deferred. This is a handoff spec for a future implementor (junior-friendly — it explains
the concepts, not just the edits). Until it lands, the compiler **panics** on the cases this fixes (see
"Interim state" at the end), so if you hit that panic, this is your plan.

## Context — the problem this solves

The borrow checker derives a *group* for every borrow reference: an expression describing which
storage a `&T` points into (a local's storage, an array's elements, a struct member, etc.). See
`src/typing/docs/architecture/borrowing-design.md` for the group model, and the "Region borrow
checker" section of `docs/handoffs/exp-2-handoff.md`.

It cannot currently derive a group for a **reference captured by a closure**. Example — the stdlib
`src/builtins/resources/migrate.vale`:

```
func migrate<E>(from []E, to &[]E) {
  intermediate = Array<E>((&from).capacity());
  drop_into(^from, &{ (&intermediate).push(^_); });   // <-- captures `intermediate` by reference
  ...
}
```

The closure `{ (&intermediate).push(^_); }` captures `intermediate` by reference. The closure becomes
a struct with a field `intermediate: &Array<E>`, and inside the closure body `(&intermediate)` compiles
to `*(self.intermediate)` — a `Deref` of a `MemberLookup` reading that captured field. The borrow
checker checks the closure body as its own function, where `self.intermediate` is just `&Array<E>` with
no group, so its group is **underivable**.

This blocks every closure that captures a reference — which includes any code that reaches
`migrate.vale` (all array-using code) and lambdas generally.

## Required reading before you start

- `src/typing/docs/architecture/borrowing-design.md` — the group model and the two phases.
- `docs/handoffs/exp-2-handoff.md`, "Region borrow checker" section — built-vs-remaining, and the
  "Investigate before group-generic closures" note about `function_scout.rs` stripping group params.
- This plan depends on the borrow-checker cleanup in `~/.claude/plans/how-wide-every-node-tranquil-sparrow.md`
  (no Options / no fallbacks / ranges on every node / total derivation) being done first — that work
  makes derivation total and turns "no group" into an explicit result, which this plan then satisfies
  for closures.

## Background — how closures are represented today

(All paths repo-root-relative. Verified 2026-09-02; re-confirm symbols before trusting line numbers.)

**Construction (`&{...}`)** lowers to a `Construct` of the closure `StructTT`:
- `evaluate_closure` — `src/typing/expression/expression_compiler.rs` (~2633).
- `evaluate_closure_struct` — `src/typing/function/function_compiler.rs` (~305) builds the closure `StructTT`.
- `make_closure_struct_construct_expression` — `expression_compiler.rs` (~276) emits the `ConstructTE`;
  its per-member initializers (~293-313) are the **capture lookups in the enclosing frame**. For a
  by-reference capture the initializer is already a `BorrowRef` of the enclosing local — **this is the
  one site where the captured local's group is statically known** (it's `Local(intermediate)` there).
- `determine_closure_variable_member` — `function_compiler.rs` (~342): reads the captured variable's
  kind and, if not already a ref, wraps it `KindT::BorrowRef { inner }`. So `intermediate`'s field kind
  becomes `&Array<E>`.
- The closure struct is built by `make_closure_understruct_core` — `src/typing/citizen/struct_compiler_core.rs`
  (~355). It instantiates the closure struct name with **empty generic args** (`&[]`, ~386, ~521-529)
  — **the closure struct is non-generic today.**

**The closure body** is compiled as the closure's `__call` function (template registered at
`struct_compiler_core.rs` ~416).
- Its `self`/receiver param is scouted by `create_closure_param` — `src/postparsing/function_scout.rs`
  (~983). The param's **written type is a bare `ITypeST::Rune`** (~1039), not a `BorrowRef` — a separate
  rule constrains that rune to `BorrowRef(closure_struct) in closure_region` (~1025), but the borrow
  checker reads `ParameterS.tyype`, which is the bare rune.
- A capture is read via the `IVariableT::Capture` arm — `expression_compiler.rs` (~137): it builds
  `LocalLookup(self)` → `MemberLookup(self, capture_name)` → `Deref` (to decay `&&Array` to `&Array`).
  So `(&intermediate)` in the body is `Deref(MemberLookup(LocalLookup(self), intermediate))`.

**The closure struct definition** — `struct_compiler_core.rs` (~515). Members are
`StructMemberT { name, tyype }` (`src/typing/ast/citizens.rs` ~101) — **only a name and a `KindT`**,
no `ITypeST`, no region/group, no record of which enclosing local was captured. `BorrowRefT`
(`src/typing/types/types.rs` ~24) has **no group field** (by design — "BCHATZ": groups never live on
the value type, or they'd split monomorphizations).

**Why the group can't reach the body today:** groups are borrow-checker-only and live **only on the
declaration side** (`src/typing/borrow_checker/borrow_types.rs` ~10) — re-derived from written
`ITypeST` region annotations, never carried on `KindT`. The closure `StructTT` has empty template args,
its members are group-less `KindT`, and the `self` param's written type is a bare rune. So
`make_kind_g` (`borrow_types.rs` ~159) has nothing to read a capture field's group from, and
`follow_ref` (`src/typing/borrow_checker/groupify.rs` ~447) has no `MemberLookup` arm — the captured
reference's deref comes out group-less → underivable.

## The design — group-generic closures

Make the closure struct **generic over the groups its captures need**, bind those groups at the
construction site (where they're known), and let the closure body read them off `self`'s type. This is
the ordinary group-generic model — the closure is checked once, generically, over its group params,
exactly like a normal `func foo<g'>(x &T in g) mut(g)`; the construction site binds the group args like
a normal call. **No cross-function body peek and no pass-ordering dependency** (see "Why this is
ordering-independent").

### Which group params the closure struct needs

For each captured value, collect the groups by **walking the captured value's type at every depth**
(this is what `make_kind_g` already does when it builds a `KindGT`), *not* by reading the type's
definition's group-param list. This distinction is load-bearing:

- Capture a local `x: Opt<&Ship in sg>`. Here `sg` is **not** a group param of `Opt` — `Opt<T>` has a
  *kind* param `T = &Ship in sg`, and `sg` lives *inside* the kind argument. Ask `Opt`'s definition for
  its group params and you get zero; `sg` is lost. You must walk `x`'s type to find `sg`.

The closure struct's group params are the union of:
1. **Every free group in each capture's type** (walk the type; e.g. `sg`), and
2. **A fresh outer group per by-reference capture** (e.g. `xg`) — capturing by reference adds a borrow
   layer the local's own type didn't have.

De-dup shared groups (if two captures both mention `sg`, one param).

The worked example: capturing local `x: Opt<&Ship in sg>` by reference gives closure field
`x: &Opt<&Ship in sg> in xg`, and the closure struct is generic over `xg` and `sg`.

### Binding at construction

At `&{...}` (in the enclosing frame), bind each closure group param to the actual enclosing group:
- the fresh outer group `xg` → the captured local's group, `Local(x)`;
- each collected inner group (`sg`) → whatever it is in the enclosing frame (an enclosing group-param
  rune, or an enclosing local's `Local(y)`, etc.).

### Why this is ordering-independent (do NOT introduce a pass-ordering dependency)

The closure body is borrow-checked **once, generically**, treating each capture group as an abstract
param (a rune). Inside the body `self.intermediate` has group `xg` (a rune) — a real, derivable group.
Separately, the construction site binds the group args and (for a closure that *churns* a captured
group) substitutes the closure's declared effects into the caller's groups — the same
`substitute_groups` machinery a normal group-generic **call** uses (`borrow_types.rs` ~383).

The **wrong** model — checking the closure body with the *concrete* capture groups (`Local(x)`) — would
force "check the closure after, and once per, each construction site," i.e. a cross-function body peek,
which the architect has ruled out. Don't do that. Keep it generic.

Note this also matches the existing structure: `check_function` (`src/typing/borrow_checker/check.rs`)
already runs per-function at each body's tail, closures and their enclosing function independently. No
reordering of the pass is needed.

## The edits

Data-flow order. Steps 1-5 are **core** (need the architect's literal "fire core edits"); step 6 is
AI-editable (`borrow_checker/`).

1. **Stop stripping group generic params — `src/postparsing/function_scout.rs` (~909-914), CORE.**
   The `generic_params` assembly does `generic_params.extend(extra_generic_params_from_parent)` (~908)
   — which already inherits the enclosing function's params into the closure — and then `.filter`s out
   every `IGenericParameterTypeS::RegionGenericParameterType`. That filter drops the very group params
   the closure needs (both inherited ones and the minted ones from step 2). It must stop, or be
   narrowed. **CAUTION: this filter is global — it affects every function, not just closures.** It is
   almost certainly load-bearing (the solver/instantiator likely can't yet handle group generic params
   flowing through — which is why groups were kept off the type domain to begin with). **Investigate why
   it exists before removing it wholesale** (`git log -S RegionGenericParameterType`, and check the
   solver/instantiator). Likely outcome: narrow it to "strip except a closure's minted capture-group
   params," or teach the solver to pass region generic params through inertly. This is the riskiest step.

2. **Mint the closure's capture group params — `struct_compiler_core.rs` `make_closure_understruct_core`
   (~355) + `function_compiler.rs` `determine_closure_variable_member` (~342), CORE.**
   For each capture, walk the captured value's type to collect its free groups, and mint a fresh outer
   group for a by-ref capture; add these as `RegionGenericParameterType` generic params on the closure
   struct, and instantiate the struct with them instead of `&[]` (~386, ~521-529).

3. **Type the capture field with its group — `function_compiler.rs` `determine_closure_variable_member`
   (~342) + `src/typing/ast/citizens.rs` `StructMemberT` (~101), CORE.**
   Record each capture field's group. `StructMemberT` currently carries only `name` + `tyype: KindT`;
   extend it to carry the field's region/group (or express the field's type as an `ITypeST` referencing
   the closure's group params). Whatever shape you choose, the borrow checker (step 6) must be able to
   read a capture field's group off the closure struct's type.

4. **Bind the group args at construction — `expression_compiler.rs`
   `make_closure_struct_construct_expression` (~276), CORE.**
   For each capture, bind its closure group param(s) to the actual enclosing group(s). The by-ref
   capture's outer group → `Local(captured)`; inner groups → their enclosing values. The captured
   local's borrow is already in hand there (the member initializer), so the group is available.

5. **Convey the group args on the self param's type — `function_scout.rs` `create_closure_param`
   (~983), CORE.**
   Today the closure `self` param's written type is a bare `ITypeST::Rune`. Make it resolve to a borrow
   of the closure citizen **with the group args**, so the borrow checker's `make_kind_g` can read a
   capture field's group from `self`'s type.

6. **Read the capture group in the borrow checker — `borrow_checker/` (AI-editable).**
   - `borrow_types.rs` `make_kind_g` / `make_citizen_args` (~159-267): surface a struct member's group
     from the closure type's group args, so `self`'s `KindGT` carries the capture-field groups.
   - `groupify.rs` `follow_ref` (~447): add a `MemberLookup` arm that reads the field's referent group
     from the closure struct's group args and substitutes it (mirror `call_result_group` /
     `substitute_groups`). Today `follow_ref` has no `MemberLookup` arm, which is the immediate cause of
     the underivable result; `location_group` (~435) *does* handle `MemberLookup`, but it yields the
     group of the field's *storage* (`Member{ Local(self), name }`), not where the stored reference
     *points* — a `Deref` needs the latter, which is what steps 1-5 make available.

## Invariants you must not break

- **Groups never go on the value type / `KindT`.** They stay declaration-side (scout `FunctionS`,
  written `ITypeST`, and the borrow checker's `KindGT` mirror). A group on `BorrowRefT` would split
  `Vec<int> in a` from `Vec<int> in b` into two monomorphizations. The closure's group params are
  `RegionGenericParameterType` (declaration-side), not ordinary type params.
- **No cross-function body peek; no pass-ordering dependency** (see above). Closure body checked
  generically; construction binds like a call.
- **No `Option`, no fallbacks, no `GroupExprG::Empty`** (per the cleanup plan and the architect's
  standing rules — a capture group that genuinely can't be derived is a compile error or a panic, never
  a placeholder).

## Testing

- The tests this unblocks are currently red on the underivable-closure case: everything that pulls in
  `migrate.vale` (the array/end-to-end suite) and the lambda tests (`compiler_lambda_tests::*`,
  `end_to_end_tests::tests::lambdas::lambda`, `after_regions_tests::brrz_nested_bound_return_inference_through_a_lambda_body`).
  When this lands, they go green. Measure with `cargo test --manifest-path Cargo.toml --lib`.
- Add a focused red test first (TDD): a closure captures a reference into a group, then that group is
  churned, then the closure is called — expect a use-after-churn. NOTE: catching an in-closure churn of
  a captured group additionally needs **effect checking** (the closure declaring `mut(g_cap)` in terms
  of its group params so the call site substitutes) — that machinery is separately deferred. So the
  first milestone is "closures capturing references compile (group derivable)"; the use-after-churn-
  through-a-capture milestone follows once effect checking exists.
- Run the interop lane too (`cargo +rustc-fork test --manifest-path ./Cargo.toml --lib --features rust_interop`).

## Interim state (what you're replacing)

Until this lands, the borrow checker **panics** when it meets a closure-captured reference whose group
it can't derive (a `Deref` of a `MemberLookup` on a closure `self` that yields a group-less borrow),
rather than emitting a soft error or a placeholder group. The panic message points here. Your job is to
remove that panic by implementing the above; do not merely silence it.
