# Plan: Strip per-parameter `RuneParentEnvLookup` rules in the postparser

Audience: a junior engineer new to this compiler. **Read "Context" and "Mental model" fully before
touching code.** All paths are under `/Volumes/V/Vale/exp-2-wipbx/FrontendRust/`.

---

## Context — why we're doing this

A cluster of **6 failing tests** panics inside the generics solver on a leftover
"look-this-up-in-the-parent-environment" rule. They are:

- `typing::test::compiler_generics_tests::upcasting_with_generic_bounds`
- `typing::test::compiler_tests::tests_calling_an_abstract_function`
- `typing::test::compiler_virtual_tests::generic_interface_forwarder`
- `typing::test::compiler_virtual_tests::generic_interface_forwarder_with_bound`
- `typing::test::compiler_virtual_tests::test_specializing_interface`
- `typing::test::compiler_virtual_tests::test_complex_interface`

**Root cause, one sentence:** when a **citizen method** has a parameter whose type mentions a
generic rune *inherited from the enclosing struct/interface* (e.g. `self &Bork<T>` where `T` is the
parent citizen's type parameter), the postparser leaves a `RuneParentEnvLookup` rule inside that
parameter's rule lists; that rule survives into the solver and panics, because the solver requires
such rules to have been stripped ("MKRFA-preprocessed") before it runs.

The postparser **already strips these rules** — but only from the function's **top-level** rule
list, not from the **per-parameter** lists. **This fix extends the exact same strip to the
per-parameter lists.** That's the whole change.

**Why it's safe and small:** the inherited rune's *type* and *value* are established elsewhere (the
parent's runes are folded into the function's own `generic_params`, which seed the rune's type; its
value comes from argument-matching / placeholder creation during the solve). The
`RuneParentEnvLookup` rule was only doing a redundant validation lookup. So we can delete it without
re-seeding anything. This was checked — see "Why plain strip is enough."

**Where this sits in the bigger picture (so you don't over-build):** this is a *coexistence-era*
fix. The phased-callsite redesign (`docs/plans/plan-phased-calls.md`) will eventually stop the
postparser from emitting rules at all and retire the rune-type solver, at which point this strip —
and the whole `RuneParentEnvLookup` machinery — gets deleted. But the redesign's env-supplied-rune
handling (its §1G/§1L/§32) does exactly what this strip does: resolve env-derived runes *before* the
solve. So this is a correct down-payment, not something the redesign has to unwind. **Keep it
minimal; do not build anything speculative.**

**Intended outcome:** the 6 tests advance past the `RuneParentEnvLookup` panic with **no regression**
to the 622 currently-passing tests. Some of the 6 may pass outright; some are heavyweight
virtual/interface tests and may *relocate* to a further downstream stub. Both are acceptable — see
"Scope."

**A trap that already cost time:** an earlier attempt patched the *solver's* error arm (filled in the
`unimplemented!` at `rune_type_solver.rs:478`) and greened **zero** tests — because the same
un-stripped rule *also* reaches a second solver (`compiler_solver.rs:1065`), so the crash just moved.
Stripping the rule in the postparser removes it before *either* solver sees it. **Strip at the source;
do not patch a solver arm.**

---

## Mental model — the rule, the strip, and the two solvers

1. **What a `RuneParentEnvLookup` rule is.** A postparse rule (`IRulexSR::RuneParentEnvLookup`) meaning
   "this rune's value/type belongs to an enclosing scope; look it up there." For a citizen method,
   the enclosing scope is the parent struct/interface, and the inherited rune is a parent type
   parameter like `T`.

2. **The parent runes are already folded in.** In `src/postparsing/function_scout.rs`
   (`scout_function`), a citizen method gets its parent's generic params appended to its own
   `generic_params` at **`:802`** (`generic_params.extend(extra_generic_params_from_parent)`; the
   source list is built at `:203-209`). So after folding, `T` is a normal generic param of the method.

3. **The top-level strip that already exists.** Right after folding, at
   **`function_scout.rs:813-820`**, the *top-level* rule list is filtered for `ParentCitizen`:
   ```rust
   let rules_array = match &maybe_parent {
     IFunctionParent::ParentCitizen(_) => unfiltered_rules_array
       .into_iter()
       .filter(|rule| !matches!(rule, IRulexSR::RuneParentEnvLookup(_)))
       .collect::<Vec<_>>(),
     _ => unfiltered_rules_array,
   };
   ```

4. **The gap.** Each parameter's own rule lists are built separately, at
   **`function_scout.rs:418-445`**, into `param_value_type_rules_vec` and
   `param_type_outer_ref_rules_vec` (plain `Vec<IRulexSR>`), then arena-allocated at **`:444-445`** and
   handed to `ParameterS::new(...)` at **`:486-495`**. **These lists are never run through the
   `ParentCitizen` filter.** So a param type mentioning inherited `T` keeps its `RuneParentEnvLookup`.

5. **The two solvers it crashes.** All the call/define solve sites in
   `src/typing/function/function_compiler_solving_layer.rs` concatenate `header_rules +
   per-param(value_type_rules ++ type_outer_ref_rules)` into one `all_rules` and hand it to both the
   solve and `derive_rune_to_type`. So the un-stripped per-param rule reaches:
   - `src/typing/rune_typing/rune_type_solver.rs:471-482` — the rune-type solver's
     `RuneParentEnvLookup` arm; it does an `env.lookup` and `panic!`s on the **error path** (the
     inherited rune's name isn't in the parent env anymore, so lookup fails).
   - `src/typing/infer/compiler_solver.rs:1062-1066` — the value solver's twin, an **unconditional**
     `panic!("vwat: RuneParentEnvLookupSR should have been MKRFA-preprocessed…")`. **This is the
     primary crash** and the reason patching only the rune-type-solver arm greens nothing.

---

## The change, step by step

### Step 1 — Extend the strip to the per-parameter rule lists

**File:** `src/postparsing/function_scout.rs`, inside `scout_function`, at the point where each
parameter's rules are finalized (**`:418-445`**, just before the `alloc_slice_from_vec` at
`:444-445`).

For a `ParentCitizen` function, filter `RuneParentEnvLookup` out of **both**
`param_value_type_rules_vec` and `param_type_outer_ref_rules_vec` before they're allocated — using the
**identical** filter already at `:817`:

```rust
.filter(|rule| !matches!(rule, IRulexSR::RuneParentEnvLookup(_)))
```

Notes for wiring it:
- `maybe_parent` is the same binding used by the top-level strip at `:814`; confirm it is in scope at
  the per-param build point (it is a function-level `let`; verify by reading the surrounding code).
- Filter the `Vec`s **before** `alloc_slice_from_vec`, so you don't allocate then re-allocate.
- Gate on `IFunctionParent::ParentCitizen(_)`, exactly like `:814`. Do not strip for non-citizen
  functions — they have no inherited runes and their `RuneParentEnvLookup` rules (if any) mean
  something else.

### Step 2 — (Recommended) factor the filter into one small helper

The top-level strip (`:813-820`) and the new per-param strip apply the **same** filter. Per the
`find-deadweight` skill (avoid near-duplicate logic), extract a tiny helper and call it at both sites,
e.g.:

```rust
fn strip_parent_env_lookups<'s>(rules: Vec<IRulexSR<'s>>) -> Vec<IRulexSR<'s>> {
    rules.into_iter()
        .filter(|rule| !matches!(rule, IRulexSR::RuneParentEnvLookup(_)))
        .collect()
}
```

Then the top-level site and each per-param list call it under the same `ParentCitizen` gate. This is
optional but preferred; if the surrounding ownership makes it awkward, an inline `.filter(...)` at
each site is acceptable — do not contort the code to share it.

---

## Why plain strip is enough (do NOT re-seed)

The rune only needed the `RuneParentEnvLookup` rule for a *redundant* validation; both its type and
value are supplied independently:

- **TYPE** — `src/typing/rune_typing/derive.rs:26-30`, `derive_rune_to_type` seeds every
  `generic_param`'s type before solving. Because the inherited rune is now in `generic_params` (the
  fold at `:802`), its type is seeded regardless of the rule.
- **VALUE** — two live solve paths, both independent of the rule:
  - *Definition-side* (`evaluate_generic_function_from_non_call_solving`,
    `function_compiler_solving_layer.rs:700+`): a `create_placeholder` loop over `generic_params`
    (`:776-794`) commits each param's value as a placeholder.
  - *Call-site* (`evaluate_generic_function_from_call_for_prototype`, `:399-449`): the inherited rune's
    value arrives from argument-matching (the receiver arg unifies against the `self`-param type) or
    from `container_rune_initial_knowns` when template args are written explicitly.

So a plain strip is correct for the rune-type solver and both live value solves. If re-seeding ever
*were* needed, the canonical MKRFA fold to copy is at `src/typing/overload_resolver.rs:362-384` (look
the rune up in the calling env, push an `InitialKnown`, drop the rule), and the caller contract is
documented at `src/typing/infer_compiler.rs:203-213`. **You should not need either for this fix** —
mentioned only so you recognize the pattern.

Because the strip is at the postparser, it covers all three live `all_rules` assembly sites at once
(`function_compiler_solving_layer.rs:419-423`, `:568-572`, `:732-736`) — cleaner than stripping at
each of them.

---

## The one thing to verify against the repro — the virtual-dispatcher path

There is a fourth, **WIP** solve path:
`evaluate_generic_virtual_dispatcher_function_for_prototype_solving`
(`function_compiler_solving_layer.rs:556-631`). It uses a *preliminary* solve and then, at
**`:622-631`**, reads each generic param's value out of `preliminary_inferences`, `panic!`-ing
(`"implement: create placeholder for missing preliminary inference"`) if a value is absent — it does
**not** use the `create_placeholder` loop.

Risk: if any of the 6 tests routes a citizen method through *this* path, removing the
`RuneParentEnvLookup` rule could remove the thing that was concluding `T` there, and you'd hit that
`unimplemented!` instead. That would be a **separate follow-up** (finish the missing-placeholder
branch), not a defect in the strip. So: after applying the strip, if a test relocates to
`function_compiler_solving_layer.rs:622-631`, record it as a known follow-up and do **not** try to
fix it in this change.

---

## Scope / non-goals

- **In scope:** the 6 `RuneParentEnvLookup` (citizen-method inherited-rune) tests above.
- **NOT in scope: the anonymous-interface cluster** (9 tests panicking at
  `src/typing/macros/anonymous_interface_macro.rs:494`). Those need a *different* fix — accepting
  `IVarNameS::SelfName` in `ParameterS::new` (`src/typing/ast/ast.rs:407`) — after which some of them
  fall into this same rune territory. Leave them alone; they are a separate change.
- **Do not** touch the solver arms (`rune_type_solver.rs:478`, `compiler_solver.rs:1065`), the MKRFA
  helper, or the assembly sites. The postparser strip is the whole fix.
- **Do not** fix the virtual-dispatcher stub (`:622-631`) here even if a test relocates to it.

---

## Verification / test plan

Follow the repo build convention: pipe each build/test run to **one fixed file** in `./tmp/`, and
inspect it with a **separate** command — never chain a heavy command with `| tail`/`| grep`.

1. **Baseline (given): 622 passed / 127 failed / 8 ignored.**

2. Build (must be exit 0, zero new warnings):
   ```bash
   cargo build --manifest-path /Volumes/V/Vale/exp-2-wipbx/FrontendRust/Cargo.toml --lib > ./tmp/rpel-strip.txt 2>&1
   ```

3. Run the 6 target tests before AND after the change, into the same file:
   ```bash
   cargo test --manifest-path /Volumes/V/Vale/exp-2-wipbx/FrontendRust/Cargo.toml --lib \
     upcasting_with_generic_bounds tests_calling_an_abstract_function \
     generic_interface_forwarder generic_interface_forwarder_with_bound \
     test_specializing_interface test_complex_interface > ./tmp/rpel-strip.txt 2>&1
   ```
   ```bash
   grep -E "test result|panicked at|FAILED" ./tmp/rpel-strip.txt
   ```
   - BEFORE: each panics at `rune_type_solver.rs:478` or `compiler_solver.rs:1065`.
   - AFTER: each either **passes**, or relocates to a *different* site (e.g. the virtual-dispatcher
     stub `:622-631`, or an emission stub). Relocation = the strip worked; note where it went.

4. **Full-suite regression guard (mandatory):**
   ```bash
   cargo test --manifest-path /Volumes/V/Vale/exp-2-wipbx/FrontendRust/Cargo.toml --lib > ./tmp/rpel-strip.txt 2>&1
   ```
   ```bash
   grep "test result" ./tmp/rpel-strip.txt
   ```
   - **622 must not decrease.** Target `622 + N passed`, `127 − N failed` where `N` is the number of
     the 6 that fully green.
   - Diff the full failing-test list before vs after; **the set of newly-failing tests must be
     empty.** If anything that was green goes red, STOP and diagnose — do **not** call it
     "pre-existing." If any result changes between two identical runs, STOP: nondeterminism is a P0.

5. **Debugging discipline:** if you need to investigate, add a focused **test case** in the project
   (per the `DMTP` rule) — do **not** write a throwaway program. Do **not** add `#[ignore]` to any
   test.

---

## Critical files

- `src/postparsing/function_scout.rs` — `scout_function`: the fold (`:802`), the existing top-level
  strip (`:813-820`), and the per-param rule build (`:418-445`) where the new strip goes. **This is
  the only file you must edit.**
- `src/typing/rune_typing/rune_type_solver.rs:471-482` — one panic site (context only, do not edit).
- `src/typing/infer/compiler_solver.rs:1062-1066` — the primary panic site (context only, do not edit).
- `src/typing/rune_typing/derive.rs:26-30` — proof the rune's type is seeded independently (context).
- `src/typing/function/function_compiler_solving_layer.rs` — the solve paths and `all_rules` sites
  (`:419-423`, `:568-572`, `:732-736`) and the virtual-dispatcher WIP stub (`:556-631`, watch
  `:622-631`) (context only, do not edit).

## Suggested commit boundary

One small change: the per-param strip (Steps 1–2). Keep the anonymous-interface `SelfName` fix and
the virtual-dispatcher placeholder stub as separate follow-ups. Do not fold them in.
