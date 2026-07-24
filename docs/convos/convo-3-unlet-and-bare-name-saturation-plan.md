# Plan document

Source: `/Users/verdagon/.claude/plans/reactive-weaving-kettle.md`
Session: 905f1008-da8f-4e28-b62d-c05c370863ab

---

# Postparse: saturate bare type-names into zero-arg `Call`s (option A)

## Context

A bare type-name in a templex (`Moo` in `&Moo`, a param type, a struct member type) currently lowers to a plain `Lookup` rule. In the rune-type solver that `Lookup` yields the name's **definition** type — a `TemplateTemplataType` (a type *constructor*) for a citizen, not a `KindTemplataType` (a concrete type). But the surrounding rule (e.g. a `BorrowRef`) expects a Kind, so the solver must *coerce* Template→Kind — a context-directed step that needs a two-pass "predicting" mechanism and is the source of the `&Moo` `SolverConflict` we hit while chasing the interop tests.

We're dropping higher-kinded types (C++-style template-template-parameters) for now — if they return, they'll return as an explicit opt-in marker, single-pass. So **every bare type-name is a Kind** (a nullary or fully-applied type). We front-load that at scout time (explicitly no new name-resolution pass): a bare name lowers to an **explicit zero-argument application** — `Lookup(template) + Call(kind, template, args=[])` — uniform with how `Moo<int>` already lowers. The result rune is a Kind by construction, so the rune-type solver never sees a Template-where-a-Kind-is-wanted, and the coercion / `predicting` / `MaybeCoercing` machinery becomes dead. This is the rustc model: a nullary struct is `Adt(Moo, [])`; a missing `<>` is just an empty args list.

This plan is the **postparse (scout) half**. The typing half is the coupled companion slice — see **Coupling**.

## The change

**Parser: no changes.** The templex AST already distinguishes bare `Moo` (`ITemplexPT::NameOrRune`) from `Moo<int>` (`ITemplexPT::Call`); this is entirely a lowering (postparse) change. (The future HKT opt-in marker would be the only parser work, and it's deferred.)

All code-written bare type-name lowering funnels through a single function (`add_lookup_rule`, called from exactly one site), so the core change is one place; every type position (params, returns, struct members, impl subject/interface, generic bounds, patterns, expression type positions) routes through `translate_templex` and inherits it.

### 1. Core — `src/postparsing/rules/templex_scout.rs`, `NameOrRune` else-branch (`:264-278`)
The else-branch (commented `// e.g. "int"`) today returns `add_lookup_rule(...)`, i.e. a bare `Lookup`, and returns that rune. Change it to:
- call `add_lookup_rule(...)` to emit the `Lookup` and get the **template rune** (unchanged);
- emit a zero-arg `Call` over it — mint a fresh result rune with the idiom `scout_arena.intern_rune(ImplicitRune(ImplicitRuneValS::new(lidb.child().borrow_val())))`, push `IRulexSR::Call(CallSR { range, result_rune, template_rune, args: <empty arena slice> })`;
- return the **result rune** (the kind), not the Lookup rune.

`CallSR` is `{ range, result_rune, template_rune, args: &'s [RuneUsage] }` (`rules/rules.rs:137`); `args` can legitimately be empty (`scout_arena.alloc_slice_from_vec(vec![])`). The wrap helpers (`translate_borrow_ref_templex` etc., `:113+`) take the returned inner rune opaquely, so `&Moo` transparently wraps the Call's kind rune with **no change** to the BorrowRef/WeakRef/OwnRef arms (confirmed).

### 2. Extract a shared Call-emitting helper (DRY)
The `ITemplexPT::Call` arm (`:317-354`) already builds `Call(CallSR{...})` from a template rune + arg runes. Extract `add_call_rule(scout_arena, lidb, rule_builder, range, template_rune, arg_runes) -> RuneUsage` and use it from both the existing Call arm and the new bare-name path (with `arg_runes = []`). Keeps the two application sites identical and the existing Call behavior byte-for-byte.

### 3. `()` empty tuple — `templex_scout.rs:440`
Currently a lone `Lookup` for the empty-tuple template, with a comment that it matches "how any zero-arg kind template (e.g. `Spaceship`) is handled." That rationale **inverts** here (zero-arg kinds now saturate via `Call`). For consistency wrap it in a zero-arg `Call` too (reuse `add_call_rule`) and update the stale comment. Non-empty tuples (`:457`) and arrays (`:401`) already emit Lookup+Call — untouched.

### 4. Implicit `void` return — `function_scout.rs:573` — leave as-is
The synthetic implicit-void return (omitted return type) builds a bare `Lookup(void)` outside `translate_templex`. `void` is a primitive, so a bare `Lookup(void)` resolves straight to a Kind — no coercion, no wrapping needed. Leaving it means an *explicit* `void` return gets `Lookup+Call` while an *implicit* one stays a bare `Lookup`; that divergence is harmless (both yield the void Kind). Note it; do not wrap. (Closure-param synthetic lookup at `:912` is a lambda struct-name, not a type templex — untouched.)

### 5. Postparse tests — `src/postparsing/test/post_parser_tests.rs`
Two mechanical breakages: (a) slice patterns `value_type_rules: [Lookup(int)]` / `foo.rules: [Lookup(void)]` gain a second element → `[Lookup, Call]`; (b) rune-identity guards where a member/kind rune was the `Lookup`'s rune must now track the `Call`'s `result_rune`. Update:
- `test_struct` (`:139`, guard `:151`) — member type-rune is now the `Call` result.
- `impl_` (`:267`; `:276`/`:287`) — `struct_kind_rune` / `interface_kind_rune` are now `Call` results.
- `test_param_no_outer_wrap_routing` (`:1335`), `..._single_ref_..` (`:1364`), `..._held_ref_..` (`:1393`), `..._own_ref_..` (`:1425`) — `value_type_rules: [Lookup(int)]` → `[Lookup, Call]`.
- `test_function_rules_no_longer_contains_param_rules` (`:1484`) — `foo.rules: [Lookup(void)]` → `[Lookup, Call]`.
- Confirm `test_param_nested_ref_wrap_routing` (`:1456`) still passes (it doesn't slice `value_type_rules`; rune identities still track the value-type root).

### 6. (Optional) test infra + humanizer
- `test/traverse.rs` has **no `CallRule` `NodeRefS` variant**, so the new `Call`s are invisible to `collect_*_snode!`. Add one (enum `~:78`, `visit_rulex` `Call` arm `:729`) only if new tests should assert the `Call` shape; not required to fix the breakages above (they update via slice/rune patterns, not by counting Calls).
- `post_parser_error_humanizer.rs:236` renders a zero-arg `Call` as `$0 = $1<>` (empty angle brackets) — cosmetically odd, functionally fine; optional tweak to print `$0 = $1` when `args` is empty.

## Coupling (why this is one half)
This scout change **cannot be green alongside a linked typing pass by itself**: a wrapped primitive (`int` → `Lookup(int→Kind) + Call`) reaches the rune-type-solver `Call` arm, which today panics on a non-`Template` template. So the postparse change must land with the typing companion:
- rune-type-solver `Call` arm (`rune_type_solver.rs`) treats "zero args applied to an already-Kind template = identity";
- delete the `lookup_rune_type` Template→Kind coercion branch, the commented `MaybeCoercing*` arms, and any `predicting` two-pass;
- verify the value solver (`compiler_solver.rs` / `resolve_template_call_conclusion`) resolves the new zero-arg `Call`s (citizen zero-arg application → the kind; primitive → identity).

Sequence like the earlier slices: land this postparse change with typing unlinked / accepted-red, then the typing companion relinks and makes the full suite green. The **postparse test suite** (this plan's verification) is independent and goes green on its own.

## Verification
- `cargo test --manifest-path FrontendRust/Cargo.toml --lib postparsing::test > ./tmp/saturate-bare-names.txt 2>&1` → green after the ~7 test updates (redirect fully, then inspect the file separately — never chain heavy build + grep).
- Read the updated assertions to confirm the emitted shapes: `&Moo` → `Lookup(Moo) + Call([]) + BorrowRef(inner = Call.result)`; `x int` param → `Lookup(int) + Call([])`; `Moo<int>` unchanged (`Lookup(Moo) + Call([int])`).
- Confirm no new postparse warnings and that the `add_call_rule` extraction leaves the existing `Call` arm behavior identical (diff the generated rules for a `Moo<int>` test before/after).
