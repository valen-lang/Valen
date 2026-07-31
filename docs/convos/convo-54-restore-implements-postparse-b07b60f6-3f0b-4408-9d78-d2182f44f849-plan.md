# Plan document

Source: `/Users/verdagon/.claude/plans/lovely-brewing-yao.md`
Session: b07b60f6-3f0b-4408-9d78-d2182f44f849

---

# Restore `where implements(T, IShip)` — postparse half

> **STATUS: postparse half COMPLETE.** Both RFIGA slices landed. Suite 575 → **579 / 174 / 8**,
> postparse **88/88**, warnings still 8, and `rule_scout.rs:152` is gone from the panic
> distribution. All four denizens carry `impl_bounds`; the solver was not touched. The typing half
> starts at the ZHERE in `function_compiler_solving_layer.rs:756` — see the corrected out-of-scope
> section at the bottom, which supersedes what this plan originally said about where it goes.

## Context

15 of the 174 failing typing tests die at `postparsing/rules/rule_scout.rs:152` on
`POSTPARSER_TRANSLATE_RULEX_NOT_YET_IMPLEMENTED`. All 15 are the same cause: an
`IRulexPR::BuiltinCall`, which in rule position means `implements(T, IShip)`. Only 6 of the 15
write it themselves — the other 9 (all `compiler_project_tests`, plus
`compiler_solver_tests::one_of`) merely load builtins, and `builtins/resources/as.vale` uses it
twice. So `as.vale` is a chokepoint the same way `arrays.vale` was.

**This was collateral damage, not a retirement.** Commit `840e2014a` (the postparse slice, suite
green at 489/0/1) deleted `DefinitionCoordIsaSR` and `CallSiteCoordIsaSR` under *"deleted AugmentSR,
all `Coord*SR`"* — a name-based sweep. They were named `Coord*Isa` only because they related two
Coords. Their twins `DefinitionFuncSR`/`CallSiteFuncSR` survived the same commit, and
`onion-typing-plan.md:104-105` explicitly said **rename**, not delete:

```
- `CallSiteCoordIsaSR` → `CallSiteKindIsaSR`. Same peel-and-check treatment.
- `DefinitionCoordIsaSR` → `DefinitionKindIsaSR`. Same.
```

What the plan *did* mark for deletion was narrower: the ancestor-branch coercion-accept patch
(`onion-typing-plan.md:114`) and `complex_solve`. Those stay deleted.

**But we are not restoring it as a rule.** `CallSiteCoordIsa` had two producers with different
fates: it was self-generated per call argument from `CoordSendSR` (correctly retired — argument
compatibility now happens after resolution via `params_match` → `is_type_convertible`), and it was
pushed by `rule_scout` for a written `where implements(...)` (a user-declared bound, which lost its
representation as collateral). Only the second is being restored, and a declared bound is not a
rule.

**Scope of this plan: the postparse half only.** The typing half (computing the `Isa` in the
post-solve passes) is a separate change. After this lands the 15 tests will *not* green — they move
off the `rule_scout` panic to a later failure in typing. That is the expected outcome.

## Why it does not belong in the solver

Three independent confirmations, all from the pre-sweep code:

- **It never deduced anything.** Both old puzzles were `vec![vec![sub_rune, super_rune]]` — one
  puzzle requiring *both* runes already concluded. It could only fire after the fact.
- **Nothing mid-solve read the result.** The only consumer was `complex_solve` walking
  sender→receiver edges for common ancestors, and that is dead.
- **The old two-variant split existed solely to satisfy the completeness check.**
  `CallSiteCoordIsaSR`'s own comment says the definition variant's result rune "is still there, and
  all runes must be solved, so we need something to solve it." Taking it out of the solver removes
  that pressure entirely — hence one carrier, not two, and no `include_rule_in_*_solve` entries.

## Load-bearing facts to preserve

1. **The result rune is a join key, not bookkeeping.** `instantiator.rs:598` asserts the params and
   args maps have equal length, then `:619` zips them by rune. The definition side mints an
   `INameT::ImplBound` placeholder (`compiler.rs:452-459`); the call site supplies the real impl.
   **They must agree on rune identity across two separate solves**, which is why it is minted at
   postparse and stored — minting it lazily in either post-solve pass would produce two different
   runes and silently break the instantiator's lookup.

2. **All three runes are `RuneUsage`s — keep the ranges.** They are what lets a failed bound point
   at the `implements(...)` the user wrote. This matters concretely: `ITypingPassSolverError::IsaFailed
   { sub, suuper }` has a humanizer arm (`compiler_error_humanizer.rs:492`) but carries **no range of
   its own**, so the bound's ranges are the only source of position information for that diagnostic.

   Separately — and this is about the *solver's demand set*, not the field type — the result rune
   must not end up in `rune_to_type`. The completeness check at `infer_compiler.rs:257` demands a
   conclusion for everything in `rune_to_type ∪ get_all_runes`, and the result rune's conclusion is
   produced *after* that check, by the post-solve pass. Since the bound is not an `IRulexSR` there
   is no `rune_usages()` for it to leak through, so this holds automatically; the note exists so
   nobody "helpfully" seeds it into `derive_rune_to_type`'s `extra_runes_and_types` later.

   (The pre-sweep code did type it, via `rune_to_explicit_type.push((result_rune, ImplTemplataType))`.
   That was necessary when a solver rule concluded it. It isn't now, and doing it would break the
   completeness check.)

3. **Everything downstream of postparse is already live and needs nothing.** Verified this session:
   `environment.rs:582-600`'s `add_entries` arm indexes an `Isa` templata by impl imprecise names
   derived from its `sub_kind`/`super_kind`, with explicit `KindT::KindPlaceholder` arms for both;
   `impl_compiler.rs:533` has `get_parents` accepting `ITemplataT::Isa`; and
   `templata_compiler.rs:425-444` has `assemble_rune_to_impl_bound` harvesting rune-keyed `Isa`
   entries. That chain is why `doUpcast<T>`'s body will typecheck once the typing half lands.

4. **Lambdas cannot have where-clauses.** `function_scout.rs:302-307` asserts
   `template_rules_p.is_empty()` on the `IFunctionParent::ParentFunction` path. This is what makes
   the three `vcurious` asserts at `function_compiler_solving_layer.rs:163`/`:268` sound — do not
   touch them, and expect them not to fire.

## Design

### `ImplBoundS`

New struct in `postparsing/rules/rules.rs`, alongside `RuneUsage` and `CallSR` — it is built of
`RuneUsage`s and is rules-adjacent. House style there is plain, not sealed:
`#[derive(Copy, Clone, Debug, PartialEq)]`, one `'s` lifetime, no `_sealed`, no `new`. (The sealed
tier in `ast.rs` is for structs with invariants to assert; this has none. Compare
`OtherGenericParameterTypeS` at `ast.rs:512`, which is sealed *because* it asserts, against
`GenericParameterS` at `:528`, which is small and plain.)

```rust
pub struct ImplBoundS<'s> {
  pub range: RangeS<'s>,
  pub sub_rune: RuneUsage<'s>,
  pub super_rune: RuneUsage<'s>,
  /// Join key between the callee's declared bound and the caller's supplied impl; see
  /// instantiator.rs:598-619. Its conclusion is produced by the post-solve pass, so it must stay
  /// out of `rune_to_type` — see the completeness check at infer_compiler.rs:257.
  pub result_rune: RuneUsage<'s>,
}
```

### Denizen fields

All four gain `impl_bounds: &'s [ImplBoundS<'s>]` plus the constructor parameter:

- `FunctionS` (`postparsing/ast.rs:544`) — **8** `::new` call sites
- `StructS` (`:168`) — **2** call sites. Note it has two rule lists, `header_rules` and
  `member_rules`; both scout paths feed the same bounds vec.
- `InterfaceS` (`:252`) — **1** call site
- `ImplS` (`:296`) — **2** call sites

Of the 13 constructor call sites, 9 are in `typing/macros/`, `typing/rust_interop/`, and
`expression_compiler.rs` synthesizing denizens — those pass an empty slice.

### Threading

`translate_rulexes` and `translate_rulex` (`rule_scout.rs:22` and `:48`) each gain
`impl_bounds: &mut Vec<ImplBoundS<'s>>`, a sibling of the existing `builder: &mut Vec<IRulexSR>`.
`translate_rulex` is private, so external callers only touch `translate_rulexes`:

- external: `function_scout.rs:292` (free function), `:310` (method in a citizen);
  `post_parser.rs:736` (impl), `:1079` (struct), `:1343` (interface);
  `expression_scout.rs:1092` — **vestigial: it passes a literal `&[]` for `rules_p`, so it can
  never produce anything.** Thread the param and move on.
- internal: `rule_scout.rs:35`, `:92`, `:103` (Equals operands), `:138` (Components)

**`translate_templex` does NOT need threading.** It already carries a twin
`rule_builder: &mut Vec<IRulexSR>` and has 20 call sites, so this looked like the dominant cost.
It isn't: `implements(...)` is an `IRulexPR::BuiltinCall`, a *rule*, and its arguments are parsed
by `parse_rule` — so they recurse through `translate_rulex`, never through `translate_templex`.
Bounds can only originate in the `BuiltinCall` arm. Confirm this holds if the arm is ever extended.

The doc comment at `rule_scout.rs:19-21` already claims two return values while only returning one
(side rules exit via `builder`); update it to describe both out-params.

### ►► Arena hazard — the easiest thing to get wrong ◄◄

`BuiltinCallPR.name` is interned in the **parse** arena (`'p`); `keywords.implements` reachable
from `rule_scout` is interned in the **scout** arena (`'s`, `keywords.rs:442`). `translate_rulex`
receives only `keywords: &Keywords<'s>` and has no access to `PostParser`'s sibling
`keywords_p: &'ctx Keywords<'p>` (`post_parser.rs:526`, per @PPSPASTNZ). So an `StrI` identity
comparison across the two **will not match**, and would fail silently by falling through to the
unknown-builtin panic. Compare on string content — `name.str().as_str() == "implements"` — or
thread the `'p` keywords in deliberately.

### The `implements` arm

Recoverable near-verbatim from `840e2014a^:postparsing/rules/rule_scout.rs:169-197`, minus the two
`builder.push(IRulexSR::…)` calls and the `rune_to_explicit_type` lines (neither channel exists
now). Translate both args to rune usages, mint the result rune with the standard
`lidb.child()` / `ImplicitRuneValS::new(child_lidb.borrow_val())` / `scout_arena.intern_rune`
idiom, push one `ImplBoundS`, return the sub rune usage.

`keywords.implements` is interned in the scout arena (`keywords.rs:442`), so it is reachable here.

Any other builtin name panics naming the builtin and its source position — the same treatment given
to the parse errors earlier today. `is_interface` and `refs` stay unrestored; both need rule
variants that no longer exist (`IsInterfaceSR`, `PackSR`).

## RFIGA

Baseline note: the suite is **575 / 174 / 8**, deliberately RED for the typing slice. The tdd
skill's green-baseline rule is suspended for this arc by the handoff. The meaningful baselines are
**postparse 84/84 green** and **the 15 tests failing at `rule_scout.rs:152`** — both must hold
before starting, and the **A** substeps are measured against them.

1. **Postparse records an `implements` bound on a function.**
   * R: add a postparse test scouting `func moo<T>(a T) where implements(T, MyInterface) { }`;
     assert the resulting `FunctionS.impl_bounds` has one entry whose `sub_rune`/`super_rune` are
     the `T` and `MyInterface` runes.
   * F: run it; expect
     `POSTPARSER_TRANSLATE_RULEX_NOT_YET_IMPLEMENTED: BuiltinCall at <range>` — the arm was made
     to name its variant earlier today, so the failure should identify `BuiltinCall` specifically.
     If it says anything else, the test is not reaching the arm and the R step is wrong.
   * I: add `ImplBoundS`; add the field + constructor param to all four denizens (the other three
     store an empty slice for now and **assert the collected vec is empty with `"vcurious"`**);
     thread the builder through the 10 call sites; fill the `implements` arm; panic informatively on
     other builtin names.
   * G: re-run; expect pass.
   * A: full suite. Expect postparse **84/84**, and the 15 to move **off** `rule_scout.rs:152` to a
     later failure — confirm by grepping the panic distribution, not by the pass count, which will
     not move.

2. **Postparse records an `implements` bound on a struct, interface, and impl.**
   * R: add three postparse tests — a struct, an interface, and an impl, each with
     `where implements(...)` — asserting each denizen carries the bound.
   * F: run them; expect the `"vcurious"` assert from slice 1 to fire.
   * I: replace those three asserts with storage into the respective denizen.
   * G: re-run; expect pass.
   * A: full suite; expect no change from slice 1's numbers.

## Files

- `FrontendRust/src/postparsing/rules/rules.rs` — `ImplBoundS`
- `FrontendRust/src/postparsing/ast.rs` — the four denizen structs and their sealed constructors
- `FrontendRust/src/postparsing/rules/rule_scout.rs` — signatures, the `implements` arm, the
  unknown-builtin panic, the stale doc comment at `:19-21`
- `FrontendRust/src/postparsing/function_scout.rs`, `expression_scout.rs`, `post_parser.rs` —
  call-site threading and the three `"vcurious"` asserts (slice 1) → storage (slice 2)
- `FrontendRust/src/typing/macros/**`, `typing/rust_interop/declarations.rs`,
  `typing/expression/expression_compiler.rs` — 9 synthetic constructor call sites pass an empty
  slice
- `FrontendRust/src/postparsing/test/` — the four new tests
- `FrontendRust/src/postparsing/test/traverse.rs` — **will not break, but should be updated.**
  `visit_function` (`:383-405`) reads `FunctionS` by *field access*, not destructuring, so a new
  field compiles fine and is silently un-traversed. Add a visit line at `:404` and a `NodeRefS`
  variant near `:32`. (`NodeRefS` is only consumed as predicate patterns in three test files, so
  adding a variant is safe.)

Two things that confirm the design rather than needing changes: there are **zero** `FunctionS { .. }`
struct literals anywhere, so the seal holds and only the 8 `::new` sites break; and
`post_parser_tests.rs:1363` and `:1526` assert `FunctionS.rules` is *exactly*
`[Lookup(void), Call([])]` — they stay green precisely because bounds go to a new field rather than
into `rules`.

## Verification

```bash
cargo test --manifest-path ./FrontendRust/Cargo.toml --lib --no-fail-fast > ./tmp/implements-restore.txt 2>&1
grep "test result" ./tmp/implements-restore.txt
grep -c "postparsing" ./tmp/implements-restore.txt
grep -oE "panicked at [a-z_/]*\.rs:[0-9]+" ./tmp/implements-restore.txt | sort | uniq -c | sort -rn
```

Success for this half:
- postparse suite still 84/84
- zero panics at `rule_scout.rs:152`
- the 15 now fail somewhere in `typing/` instead
- warning count still 8

## Explicitly out of scope

- **The typing half.** ►► CORRECTED after implementation — the live account is the ZHERE at
  `function_compiler_solving_layer.rs:756`. This section originally named
  `resolve_conclusions_for_define` as the definition-side home. **That is wrong three ways over:**
  it has no `InferEnv` (which `assemble_impl` needs for the `ImplBound` id), it takes `conclusions`
  immutably, and — decisively — it runs at `infer_compiler.rs:531`, *after*
  `import_conclusions_and_reachable_bounds` has already built the env from those conclusions at
  `:528`. An `Isa` inserted there would never reach any environment.

  The definition-side mint belongs in the one-statement gap between `interpret_results` and
  `check_defining_conclusions_and_resolve`, at four sites:
  `function_compiler_solving_layer.rs:748`, `struct_compiler_generic_args_layer.rs:420` (closes at
  `:431`) and `:531`, and `solve_for_defining` at `infer_compiler.rs:124`. Open whether that is
  four sibling calls or a wrapper — forgetting one is a silent wrong answer, not a compile error.

  It need not precede the solve: per SFWPRL (`docs/Generics.md:355`) the solve deliberately
  postpones citizen resolution, and nothing mid-solve reads an `Isa`.

  The call-site half is unchanged from the original text — check via `is_parent` in
  `check_resolving_conclusions_and_resolve`, which owns its conclusions and has an `InferEnv`. The
  concrete line to replace there is the hardcoded `runes_and_impls = vec![]` at
  `infer_compiler.rs:407`.

  Note the whole impl-bound family is **inert today** — no producer before this change, no
  discharge on either pass — so none of the above is reproducible until the typing half lands.
- Filling `rune_to_bound_impl`. Its two `vcurious` guards and the passthrough `panic!` at
  `function_compiler_solving_layer.rs:175` are deliberate instruments; leave them.
- The ancestor-branch coercion-accept patch — stays deleted per `onion-typing-plan.md:114`.
- `is_interface` and `refs`.
