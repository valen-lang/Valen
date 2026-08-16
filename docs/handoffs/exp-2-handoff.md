# Vale2 Handoff — Onion typing; the typing slice is in progress
<!-- guardian-require-skill: update-handoff -->

**IMPORTANT: Use the /update-handoff skill before editing this file. (Enforced: Guardian's
RSBEX gate denies edits by sessions whose transcript shows the skill was never loaded.)**

**Start here.** The parser, postparse, and higher_typing-retirement slices are landed on `experimental-2`; the **typing slice's structural refactor is done** — `CoordT` / `OwnershipT` / `LocationT` gone from live `typing/`, `KindT` carrying its four ref wraps, one flat `ExpressionTE` whose `result()` is a `KindT`, `SoftLoadTE` dissolved, every node sealed. What remains of onion typing is coercion-layer completeness (`convert` / `is_type_convertible`'s ref rows), interner validity-table enforcement, the `str`/share bare-mention lowering, and retiring the `implicit_clone` probe — work accumulates as TEMP CHECKPOINTs and `git log` is the git shape.

**The RED-slice phase is over: the enabled `--lib` suite passes** (measure with the PICK UP HERE command; read the measurement traps under "Current state" first). The deferred feature families — closures/lambdas, `str`/share lowering, the anonymous-interface macro, and weaks — are `#[ignore]`d rather than failing; `simplifying/`, `hammer/`, `backend/`, `testvm/`, `integration_tests/` stay commented out or `#[cfg(any())]`-gated in `lib.rs`.

**Read order for a fresh session:**
0. **"LESSONS LEARNED"** — short, and it will save you an afternoon.
1. **"PICK UP HERE"** — the current state, the uncommitted work, and what Vale4 is blocked on.
2. **"CAPABILITY LADDER"** — where the failures actually are, as first-blocker counts. The build order.
3. **"CALL-SITE PHASES"** — the pipeline the whole typing pass is organised around, and the frame every "where does this belong?" question resolves against. Read it before proposing anything about solving, deduction, coercion, or dispatch.
4. **"Current state"** — the tree, the onion surface, and the measurement traps.
5. **"Resolved design decisions"** — the locked model.
6. **"THE COMPLETE OPEN LIST"** — what is genuinely still open, grouped by what unblocks it. Read its own "almost none of this gates the next work" preamble first.
7. **"Upstream rulings"** — the design corpus's answers, and the one place they are written down.

Everything else is reference; skip it until you need it.

## ►► LESSONS LEARNED ◄◄

*Accumulates. Prune an entry when nobody could act on it.*

**Traps**

- **A name-based sweep is not a semantic one.** `implements` was deleted because it was spelled `Coord*Isa`, and `SelfCoordRuneS` went the same way — neither was Coord-era. Check what a symbol *is* before trusting what it is called.
- **Panics hide panics.** A cluster count is a first-blocker count, never a total. Clearing the largest cluster on the board can green **zero** tests and merely fragment it.
- **A discarded `Err` payload can hide a whole capability.** Six `.expect()`s on a `ParseError` that carried position and message concealed the cause of 50 failing tests; printing the payload drained the bucket to one in an afternoon. It happened again with a `FindFunctionFailure` thrown away to print a bare string, hiding 26.
- **A cluster that looks like a feature gap can be one omission upstream of it.** All 26 abstract-body failures were the same function, `drop`, failing for a reason that had nothing to do with abstract bodies. Census the failures before planning against the site.
- **Commented-out Scala beside live Rust is evidence, not clutter.** The unconsumed sends were diagnosable in seconds because the Scala that passed them into the solve is still sitting three lines above the code that dropped the parameter.
- **`#![allow(unused_variables)]` at the crate root means a dropped binding never warns.** `assemble_initial_sends_from_args` is built at four call sites and read at none, silently.
- **`T: Drop` does not detect every type with a drop** — the free-function spelling does not satisfy the bound. Check for a drop *function*.
- **Never tell another session their finding is stale on the strength of a change in your working tree.** Verify with `git log --all -S` first.
- **Check master's copy before declaring machinery never-ported.** A deliberate mid-migration removal looks identical to an unfinished port in the working tree, and the removed version documents what the replacement must cover; `git show master:<file>` settles it in one command.
- **"Retired" in a design section does not mean the code knows.** The `implicit_clone` probe is retired by ruling and still has a live function, two live error variants, 10 corpus files and ~40 test references. Say which one you mean.
- **A line number into a living document rots exactly like a `file:line` into moving code.** Half this file's `design-1:NNNN` citations broke in one upstream doc pass. Cite the quoted phrase; it is what survives.
- **Never read the postponed set as an oracle.** rustc's `deduce_closure_signature` inspects pending obligations to type a closure, which couples the solve and resolve phases through evaluation order and is very hard to test for. Postponed work is to be discharged, never consulted.
- **A postponed conversion loses information.** When rustc defers a coercion as a predicate instead of performing it, discharging it late degrades it to plain subtyping and the adjustment can no longer be recorded. Deferring is only free for things that *constrain*; anything that must be emitted decays.
- **A borrowed design should be checked against the version it was borrowed from.** rustc's default solver still resolves traits *inside* unification via eager normalization; the separation we copied is where they are heading, not where they are. Borrowing from a destination is fine, but say so.
- **A shared helper is the wrong home for a concept only one caller has.** Sends belong to calls, so putting them in `make_solver_state` would have made structs, interfaces, impls and arrays all pass empty. Check who else calls it before threading a parameter.
- **A conditional in a rule costs you the reverse direction; a conditional at a site costs nothing.** A rule that inspects a value to decide what to produce cannot be inverted, because two inputs can yield one output — so decide where the answer is known and emit an unconditional rule.
- **Layers are the incrementality mechanism, not overhead.** A pass boundary is where a hash can be compared and downstream work skipped, so merging passes to simplify works *against* language-server support rather than toward it.
- **Reaching through a claim is free; taking a value out of one is not.** Binding, storing, passing, returning and a *method receiver* all copy the claim, and peeling straight to the payload for a receiver at a destructible place is a use-after-free.
- **Measure a corpus population with every spelling it has.** A `share` census that misses the retired `imm` spelling undercounts by more than half, and a migration sized off it is wrong in the same proportion.
- **A rule that outlaws a shape moves programs rather than deleting them.** Name-uniqueness pushes cross-product overloads into import-only files, which is the one bucket no declaration-time check reaches — so a diagnostics motive can defeat itself unless the destination is checked too.
- **A rule meeting a shape it has no arm for is looking at a losing candidate, not an internal error.** The overload resolver solves every candidate it finds by name, so a mismatch must reject that candidate; tolerating the shape instead is auto-coercion inside the solver, which is what the phase-0 ordering exists to avoid.
- **A partial set with a total name gets its fold forgotten.** `header_rules` omits each param's own type-binding rules, and three of five sites remembered to fold them in — check every reader when a field's name promises more than it holds.
- **A failure-set diff is meaningless if the test binary did not compile.** Confirm a `test result` line exists before comparing runs, or a broken test build reads as every test suddenly passing.
- **Lambdas do not generally use the anonymous interface macro.** Only a lambda passed where an *interface* is expected does; a direct lambda call, or a lambda/struct satisfying a concept-function bound (`__call(&G)E`), does not — so a failing lambda test is usually not an anonymous-substruct failure.

**Architect preferences, generalized**

- **Do not treat non-generics as special cases.** A guard like `if args.is_empty()` is usually a symptom of something upstream reporting the wrong shape.
- **Primitives should not be special.** They are the only type names the environment holds as finished kinds rather than templates, and every downstream special case traces back to that one registration.
- **Structural distinctness is the tool for keeping candidates apart** — used for `&&T`, for overlapping impls, and for overlapping overloads. Reach for it before reaching for a tiebreaker.
- **The codebase is mid-migration and the old code is expected to change.** Its current shape is not a constraint on the design.
- **Surface before reverting.** A deliberate change causing fallout is a decision to raise, not to undo.
- **A simple intent deserves mechanism of the same size.** If the rule states in one sentence and the implementation needs a flag, a rule variant and a graph query, the logic is in the wrong place — move it to where the facts it needs already are.
- **Take work out of the solver wherever it can reasonably go.** The solver's job is structural deduction; name resolution and callee resolution living there as rules is complexity to relocate, not to accept.
- **Renaming is an acceptable price.** Where two things want one name, making the user rename one is preferable to a mechanism that tells them apart.
- **Widen on the way in rather than narrowing on the way in.** An error type that holds less than its producers do forces each of them to flatten, and a flatten silently drops whatever did not fit.

**Recurring agent mistakes**

- **Reasoning from current code as though it were the target.** In a mid-migration tree most of what you read is the thing being migrated *away from*. Say which one you are describing; when they disagree, the ruling is the target.
- **Promoting a reading of the implementation to a refutation of the architecture.** This once marked the ratified call-site phase model as "WRONG" in this file and cost an afternoon re-deriving it.
- **Treating one file's silence as the project's silence.** Grepping only this handoff missed a design that was ratified in the convo logs.
- **Writing corrections backward.** "We thought X and were wrong" is noise to a reader who never held X; state the trap forward.
- **Estimating from headings instead of reading.** A deadweight estimate made by skimming section titles was five times too aggressive — the day logs were invariants, not events.

**Structural**

- **More than one "what to do next" section means all but one are wrong.** The PICK-UP block is current; if it and the Horizon Plan or the open list disagree, it wins.

**Three missions live here.** "Onion typing" (below) is the active one. "Overload resolution & dispatch model redesign" is a distinct concern that still applies but has not started. "Replay / FFI design" is deferred until the backend arc.

**Working model.** The architect drives the semantic work; Claude assists on demand (bulk edits, script sweeps, doc catch-up) rather than initiating.

> ## ►► `/Volumes/V/LangNotesValen/Valen/valen-design-1.md` **IS Vale2's language design** ◄◄
> *(Read this before trusting any design statement in this file.)*
>
> It is not a peer project's spec we are "converging with" or "aligning to." It is **the** specification. Its companion `valen-design-2.md` covers the RC / multi / class layer.
>
> **There are exactly TWO intended divergences from the design:**
> - **The colon.** Vale2 *allows but does not require* `name: type`; documented Valen always writes it. (Only house style — design-1:2350 permits the colonless form for experimentation.)
> - **A mention always yields a reference; the copy happens at the receiver.** Mentioning a local or parameter produces a reference (`&i32`) even for a `Copy` type, and the value is read out (a `__copy_prim` for a primitive) only where that reference reaches a receiver that wants a bare value — the general form of auto-borrowing a bare argument. design-1's C1 (Reference-model decision 1) instead copies a bare `Copy` mention at the mention itself (design-1:204, *"`Copy` type: it copies"*) and errors on a bare non-`Copy` argument, naming `&x` / `x^` / `x.clone()`.
>
> Each goes behind an experimental compiler flag after the current call-site work lands. The colon stays under evaluation; the mention/load divergence is expected to hold for the foreseeable future, possibly permanently, so C1 is the flagged alternative rather than a near-term target.
>
> **Every other difference is a bug in this handoff or in the compiler, not a fork to be decided.** When this file and design-1 disagree, **design-1 wins**; fix this file.
>
> **There is ONE architect, not two** — the same person owns Vale2 and Valen. So a Vale2 ruling can never "diverge" from a Valen ruling; there is nobody to diverge from. When design-1 contradicts something we've ruled, the live hypothesis is **"the doc is behind"** — a ruling made in a conversation and never folded back — not "two authorities disagree." Decision 11 is the worked example: our `set`-yields-old ruling turned out to match `valen-approach-convo-19:1967`, which design-1:1703 had never caught up with. **An audit finding is therefore "find the ruling the doc missed," not "record a divergence."**
>
> **The Valen design session is reachable by mailbox** and is the front line for language/semantics questions — it answers from the corpus with citations, marks inference as inference, and routes genuinely open questions to the architect. Ask it before deciding a semantics question locally; the corpus lookups are the expensive half.
>
> **The design-1 audit has been run** and its findings are **folded into this file**; its separate doc is gone. It covered decisions 1–15, the reference-model decisions, and the coercion table. What survives lives in two places: the **audit method** — three false positives of one shape, plus the dating discipline — under "Run the design-2 audit"; and the **ruled-but-not-yet-built gap inventory** in the Valen alignment section.
>
> **Still unaudited: `valen-design-2.md`** (RC / multi / class tier). Decisions 1 (weak's shape) and 2 (share's shape) are design-2 territory and were **not** checked. A second pass is owed.

## ►► PICK UP HERE ◄◄

**Measure before quoting any count.** `cargo test --manifest-path Cargo.toml --lib
--no-fail-fast`, then census first blockers with
`grep -o "panicked at src/typing/[a-z_/]*\.rs:[0-9]*" <file> | sort | uniq -c | sort -rn` — the census
says where the work is, and it moves far more than the total does.

**►► THE CALL-SITE DESIGN LIVES IN `docs/plans/plan-phased-calls.md` ◄◄** That is the current,
authoritative 8-phase model, and it **supersedes the "CALL-SITE PHASES" and "Overload resolution"
sections below** — where they disagree, the plan wins. It retires the sends machinery (an
`ArgumentStep` matches the argument `KindT` against the parameter's `ITypeST`), the rune-type solver,
and `complex_solve`; the reject-the-losing-candidate solver arms (`KindIsNotBorrowRef` and kin) become
ordinary match failures under it.

**►► THE `ITypeST` SLICE IS THE §P FOUNDATION, IN PROGRESS ◄◄** The postparser builds a read-only type
tree `ITypeST` (`postparsing/rules/types.rs`) via `translate_templex_into_type_st` (`templex_scout.rs`),
and `ResolveSR` now carries `params_types` / `return_type`, populated in the `ITemplexPT::Func` arm.
Two helpers sit in `templex_scout.rs`, built but **not yet called**: `translate_type_st_into_rune`
(derive: tree → rules + value rune, the plan's Post-cleanup direction) and `map_runes_in_type_st`
(rewrite an `ITypeST`'s runes). The derive belongs in the postparser — it needs `lidb`/`env`, which the
typing macros lack; those build from pre-minted semantic runes and cannot use it.

**The anonymous interface macro is DISABLED** so typing could re-link — see the capability-ladder row.
Its 9 anonymous/substruct tests are `#[ignore]`d, alongside the other deferred families (measure the
suite with the command above).

**Get the tree and branch shape rather than trusting a written one** — both rot within the day:

```
git status --short
git log --oneline -1 experimental experimental-2 experimental-4
git log --oneline experimental-2..experimental     # what we have not absorbed
```

**`experimental-2` runs behind `experimental`** whenever the Vale4 interop work lands there and
ratchets. **Their commits do reach our files** — the lookup-path change touched
`typing/infer/compiler_solver.rs` — so read the incoming diff rather than assuming a clean rebase.

**Vale4's `opt_with_undroppable_contents` passes.** Its sibling
`opt_with_undroppable_mutable_ref_contents` is the live case — a `Some<&Spaceship>` argument at an
`Opt<&Spaceship>` parameter, which the plan's §2 upcastability step resolves.

For archaeology: `8d40eff9d` and `699241ffb` reshaped the interop seam from the Vale4 side — **read
them before planning interop work.** This handoff is gitignored, so it never lands in either tree.

**►► SEQUENCE BY CAPABILITY, NOT BY PANIC SITE ◄◄** A test stops at its *first* blocker, so a cluster
count is a first-blocker count and never a total. **Panics hide panics**: clearing a cluster moves its
tests to their next blocker rather than greening them. The parse bucket went 50 → 1 across four fixes
while the suite total barely moved, and clearing the 38-test `KindList` arm greened **nothing** — the
cluster fragmented six ways. Phrase work as capabilities, which is also what makes it sequenceable
jointly with Vale4. The **CAPABILITY LADDER** is the build order; the **HORIZON PLAN** is the next
section.

**►► UPSTREAM'S DOCS ARE THE SPECIFICATION AGAIN ◄◄** `valen-design-1.md` and `valen-design-2.md` as
of 2026-07-26 have absorbed the ruling backlog, so read them rather than reconstructing from mailbox
threads. Settled in the text, with citations, and safe to build against: `not(mut(…))` as the
subtractive spelling with `!` surviving only in negative impls (design-1:53, :1580); the full
*reference and the pointee* section including `[]`-adjusts (design-1:218-274); the linear/affine
correction, *"drop **absence** is what creates the obligation"* (design-1:2019); `set` yielding the
displaced old value (design-1:57-85, with design-2 R1 deferring to it at :475); bare class `T` in
return position as a strong claim (design-2:1036); the class-`for` verdicts, verdict 5 now ACCEPT
(design-2:777); and both of answer 27's soundness holds — `+ Clone` carries `duplication`, and the
generic-field case closed via `@` (design-1:2973-2974). **The erasure path is not on hold.**

`#explicitly_destroyed` is used throughout design-1, and **bare `#name` attributes are admitted**
(design-1:2716) — so `#!DeriveStructDrop` → `#explicitly_destroyed` is a clean one-for-one, both
meaning *suppress*, and the tier-1 attribute migration is unblocked.

**►► `design-1:NNNN` LINE NUMBERS PREDATING THAT PASS ARE WRONG — GREP THE QUOTED PHRASE ◄◄** The
rewrite moved passages by 300–1300 lines, and design-1 is now 3,122 lines. Spot-checked: the
citations for *"Groups themselves don't conform to traits"* (given as 1124) resolve at **1485**,
*"there is no auto-borrow"* (437) at **710**, the two `drop` spellings (1668) at **2082**, the
colonless form (2350) at **2757**. `set`'s unnameable temporary is **gone entirely**, superseded.
Citations added after the pass resolve correctly. **Every citation in this file carries its quoted
phrase — that is the durable half; grep it.** A line number into a living document is the same
fragile shape as a `file:line` into moving code.

**Check a passage's date before trusting its spelling**, and note that nothing ruled after
2026-07-27 has been checked against upstream at all.

**`vale-rust-interop-architecture.md` lives in our tree** at `docs/convos/rust_interop/`, so fixing
the *"interface" means two different kinds* vocabulary collision it carries is **ours**, not
upstream's. Any citation of it as a `Vale4/…` path is wrong. Confusable: two `convo-5-*` files exist
in different subtrees — ours under `docs/convos/`, theirs under `docs/convos/rust_interop/`.

### ►► THE HORIZON PLAN — short, medium, long ◄◄

The organising fact: **the design phase is over and the investigation phase is over.** What remains is
build, and the build order is the capability ladder below. Three horizons, each with a different
kind of blocker.

#### SHORT TERM — finish the typing slice, capability by capability

Goal: **a suite that is green again**, so the "green at commit time" invariant can come back on.
**Nothing on this list is waiting on a design decision** — every shape question the top rungs raised
is ruled, so this is build work.

1. **Triage the real compile errors** — the largest row, and the only one where the compiler is
   *wrong* rather than unfinished. Each reaches a humanized diagnostic, so read the message rather
   than the panic site.
2. **`UpcastTE::new`** — one `unimplemented!` its own `VCOORD` answers, and where both of Vale4's
   interop tests now sit.
3. **Export/extern boundary** — the `is_primitive` rename plus `peel_all_references` at both the
   check and the map lookup. Vale4's other front, waiting on the naming decision.
4. **The defect inventory.** Several are one-liners.
5. **The `implicit_clone` probe deletion** — retired by ruling, still live in the code. See "What
   blocks / what to preserve" for its extent.

**The declaration side must peel wherever the call side does.** `dot_borrow`,
`evaluate_maybe_virtuality`, `ssa_len`, `rsa_pop` and `as_subtype` each matched a bare kind against a
parameter that arrives wrapped; `peel_all_references` for reading a type and
`replace_value_type_in_ref` for rebuilding one are the two helpers this keeps wanting.

#### MEDIUM TERM — re-link what the arc unlinked, and pay the migration debt

Goal: **the compiler is whole again** end to end, and the corpus stops lying about the language.

- **Re-enable `integration_tests/`** (commented out of `lib.rs`, contributes zero tests
  today). The architect confirmed these worked recently and are wanted back soon. They carry the
  generic-virtuals fixtures — Milano / Serenity / Raza / Enterprise — which are the *only* exercise
  of the override-dispatch machinery.
- **`instantiating/` and `simplifying/`** — worse than gated, they are **stale and would not
  compile** (they match on `ReferenceExpressionTE::While/Return/Break`, an enum with zero hits under
  `typing/`). ~3 weeks out per what we told Vale4.
- **Tier 1 of the syntax migration** — `^` postfix, `own`→`ownref`, and
  `#!DeriveStructDrop`→`#explicitly_destroyed`. All three are measured and ready.
- **The `&&`-as-weak corpus sites** — the compiler cannot drive this migration, because `&&T`
  stays a legal type and nothing errors. Needs the hand-list.
- **Source ranges on AST nodes** — only 6 of 49 `ExpressionTE` variants carry a `RangeS`. Any
  post-hoc checker's diagnostics need them, and it is much cheaper before the checker exists.

#### LONG TERM — the borrow checker, and the backend arc

Goal: **the language's actual point.** This is where the design work of the last three days cashes
out.

- **Rung 0 — groups become real.** The gate on the whole borrow-checker track. Now well-scoped: see
  the regions block below for what exists (`ITemplataT::Placeholder` with `RegionTemplataType` for
  group *parameters*) versus what does not (any representation for a *concrete* group expression).
  Two prerequisites are typing-slice work regardless of regions — argument types reaching the
  call-site solve, and `substitute_templatas_in_kind` for the four ref wraps.
- **Rungs 1–3** — effect clauses and the first call-site check, then churn tracking, then `Vec<T>`.
  Two entry points confirmed, not one: borrow *creation* at the member/element seam, and the
  joint-argument check at call sites.
- **Effects** — strictly behind rung 0, because an effect target *is* a group expression. The
  representation is still unsettled (see the effect-representation block); the live candidate is a
  per-group permission map, since `held` and `dangle` are ratified permission splits that a bare
  group cannot hold.
- **The backend arc** — `Backend/` C++ walking the onion, the pre-+1'd FFI boundary, and the 52
  deferred FFI tests. `FRMACZ` still documents the superseded always-OWN ABI.

#### The one thing that would most change this plan

**Rung 0.** Everything in the long-term horizon sits behind it, and it is the one item on the whole
board that is entirely an architect decision rather than grind. Short and medium term proceed without
it; nothing past rung 0 starts until it does.

## ►► CAPABILITY LADDER — the build order ◄◄

**These are FIRST-blocker counts**: a test stops at its first failure, so clearing a capability moves
its tests to their next blocker rather than greening them. Re-measure after any rebase.

| Capability | Blocking symbol | Notes |
|---|---|---|
| *(not stubs)* **real compile errors** | `expect_compiler_outputs`, `compilation.rs` | the largest row, and each reaches a genuine humanized diagnostic; triage individually, not as a cluster |
| **`is_type_convertible`'s two holes** | `templata_compiler.rs` | bare-to-borrow and borrow read-out. **Not a fill-the-arms job** — see the block on why the predicate is wrong rather than incomplete, and loses both overload jobs |
| **Export/extern boundary** ← *Vale4's front line* | `declare_function_return_type`, `compiler_outputs.rs` | `is_primitive` rename + `peel_all_references` at both the check and the map lookup; blocked on the naming decision, not on work |
| **Rune-type solving** | `solve_rune_types` and `solve_rule`, `rune_type_solver.rs` | includes the `Lookup` pre-computation error path |
| **Anonymous substructs** — *macro disabled* | `anonymous_interface_macro.rs` | Disabled pending the phased-calls ITypeST migration (`docs/plans/plan-phased-calls.md`): its module is commented out of `macros/mod.rs`, and its `macros.rs` dispatch arm returns `vec![]`. Its 9 anonymous/substruct tests carry `#[ignore]` under a `VCOORD: re-enable anonymous interface macro after we do the ITypeST migration`. **Lambdas compile through this macro**, so the lambda cluster stays red until it is re-enabled — those failures are the disable, not new regressions. The three `ResolveSR` construction sites here are what forced the choice: they need the new `params_types`/`return_type` (`ITypeST`) fields, and migrating them (hand-wrapping runes as `ITypeST::Rune`, remapping via `map_runes_in_type_st`) is deferred to that slice. |
| **Dropping a `Str`** | `drop`'s `Str` arm, `destructor_compiler.rs` | the one drop arm still unfilled; its comment says "decrement a reference count" |
| **Upcasting** | `UpcastTE::new`, `ast/expressions.rs` | `unimplemented!` its `VCOORD` answers — `replace_value_type_in_ref` over the inner expression's type. Where both of Vale4's interop tests now sit |

**Do not plan against generic bounds, applied generics, placeholder substitution, member access,
abstract bodies, or generic drop.** The `KindList` arm, the template-position scout defect,
`substitute_templatas_in_kind`, `dot_borrow`, `generate_function_body_abstract_body` and `drop`'s
citizen arms were each the biggest cluster on the board in turn, and all now fail **zero** tests.
Clearing a cluster mostly moves its tests to the next blocker, so distrust any count as a measure of
what a fix buys.

**Parse failures are down to one**, `can_turn_a_borrow_coord_into_an_owning_coord`, which is an
architect call rather than work. The bucket was never "error-handling stubs": it was retired syntax
the corpus had not caught up with, drained in four waves where each revealed the next — a builtin's
`Ref` rune type, then `Ref`/`Kind` annotations tree-wide, then `^`-in-type-position and `[#N]T`, then
two retired where-clause builtins.

**Move tracking across branches is not its own capability** — `if_branches_must_move_same_variables`
and its sibling sit inside the member-access 31, which is why they have never reached the join.

**Almost nothing on the open-questions list gates this ladder.** Member access needs only the
no decision at all; bounds needs nothing from the region/effect design; export/extern needs the
`is_primitive` rename plus `peel_all_references`. Every open *design* item gates **rung 0 and
beyond**, which sits behind all of it.

### ►► CALL-SITE PHASES — the earlier 6-phase model ◄◄

**`docs/plans/plan-phased-calls.md` is the current design (8 phases); read it first.** This section and
"Overload resolution" below are kept for the parts the plan reuses — namespace membership, filter-is-final's
intent — but where they disagree, **the plan wins**. Notably the plan filters candidates by name and
namespace only (not by subtype), replaces sends with an `ArgumentStep` over each parameter's `ITypeST`,
and retires the rune-type solver.

Every call site runs this sequence, **on the one candidate the static filter selected**. It is the
frame the whole typing pass is organised around, and the frame every "where does this belong?"
question resolves against.

| # | phase | owns |
|---|---|---|
| **0** | **prepare** | preview each parameter's type as far as it is statically known, adjust the argument to match — auto-ref, auto-move, auto-deref, upcast — then send it at the parameter's rune |
| 1 | rune-typing | what *type* each rune is (Kind / Integer / Template / …) |
| 2 | value-solve | what *value* each rune has. **Structural deduction only** |
| 3 | resolve | the citizen/function resolutions phase 2 postponed per SFWPRL, plus declared bounds (`implements` via `is_parent`; `not(mut(..))` when effects land) |
| **4** | **convert** | perform the conversion phase 0 previewed, against the now-concluded parameter type, and emit the code |
| 5 | borrow check | rung 1+; not started |

**Candidate lookup is not a phase.** It encloses the sequence rather than sitting in it: it takes a
name plus argument types and *produces* the callee, so it cannot be a step in a sequence that
presupposes one. It searches the namespaces of the **peeled** value type — @PFVSZ's stated
rationale, *"the typing pass ignores the outermost references when looking for functions to call."*

**►► A strong ref contributes its payload's namespace too, as an ORDERED union ◄◄**
A `ShareRef(Struct(Ship))` argument contributes **both** the strong-ref namespace and `Ship`'s, with
the strong-ref one searched first. This is Rust's arbitrary-self-types shape — `impl Ship { fn foo(self: Rc<Self>) }`
puts a method whose receiver is `Rc<Ship>` into *Ship's* namespace — and it is **automatic for strong
refs only, not a general user feature**: no user-defined smart pointer gets to claim another type's
namespace.

**`Box<T>` reaches its payload too, but by `[]`, not by a `Deref` trait — Valen has none.**
design-1:1508 makes a Box's target one of the `[]` path segments, *"collection element / Box deref.
Child group. (Unified across `Vec`, `List`, `Box`, arrays.)"* — the same segment this file relies on
when it argues `xs[i]` is a place expression rather than a call. **Do not justify Box's reach by
analogy to Rust's `Deref`**, or it will later read as evidence that Valen has that concept; the
mechanism is `[]` and it is already unified.

*(Stated in terms of the wrap rather than `@`, deliberately. Under design-1's kind-polymorphic
reading `@T` ≡ `T` at struct kind, so an `@`-phrased version of this rule would collapse to a no-op
there. The rule is about `ShareRef` and is independent of how the `@` question lands.)*

**This is not optional, and it is not really about `clone`.** A bare class parameter is an anchored
borrow, `BorrowRef(ShareRef(Struct(C)))`. If namespace membership cannot see through `ShareRef`, then
`func launch(s Ship)` in `ship.vale` is in *no* namespace at all — not `Ship`'s, and not `@T`'s, since
clause (a) demands it live in `share.vale`. That is **every method on every class**, not an edge case.

**Ordered ≠ tiebreaking.** Ordering governs *which namespaces are searched* — candidate-set
construction, which already has rules ("the namespaces of the arg types"). No-tiebreaking bans
preferring one candidate over another *within* the set. Different layer, no conflict. The practical
payoff: a user's class method shadows a same-named strong-ref-flavored builtin — today the
compiler-synthesized claim clone — exactly as Rust's inherent impls beat trait impls, which is what
keeps the anchored-borrow parameter shape from colliding with it (both are `BorrowRef(ShareRef(…))`).

**This closes the "does `&Ship` mention `Ship`" open question** — yes, and a strong ref mentions its
payload as well. Call side and declaration side must see through identically or they never rendezvous.

**Selection happens before the phases, and it is where the whole call resolves:**

```
lookup by name                                     ← the candidate set
static filter — arity, wrap chain, value template  ← NO solving (see Overload resolution)
    0 → not found · 1 → win · >1 → ambiguity error
phases 0 · 1 · 2 · 3 · 4 · 5                       ← on the winner, once
```

**There is no per-candidate loop.** Because the filter is final, solving never eliminates anything,
so exactly one candidate is ever solved and nothing speculative is discarded. **A per-candidate
phases-0–3 loop with 4–5 on the winner is the shape to expect if you reason from the current code**
— `attempt_candidate_banner` still works that way — but it is what filter-is-final replaces.

Independently of that, phase 4 could never have run speculatively anyway: `convert()` is not
side-effect-free — its `&NC→NC` arm stamps a monomorphization into `coutputs`, so running it for a
losing candidate would emit code for a call that never happens.

**Why the phases exist at all** — STCMBDP (`docs/Generics.md:394`, 2023-era but the reasoning is
durable): declaring a function needs its param types; knowing those needs its requirements checked;
checking those needs the function to exist. One link has to break, and the choice was **check calls
later**. Everything else here follows from that.

**►► PHASES 0 AND 4 ARE THE SAME CONVERSION, PREVIEWED AND THEN COMMITTED ◄◄** — not two different
kinds of conversion sorted by what is knowable.

**Phase 0 previews the parameter type as far as it is statically determined — the wrap chain from
`type_outer_ref_rules`, plus the value type with any explicit template args substituted — and adjusts
the argument to match: auto-ref, auto-move, auto-deref, and upcast.** It is pure; it works out what
the argument must become. **Phase 4 performs that conversion and emits the code**, on the winner,
once.

Phase 4 cannot move earlier, and the reason is side effects rather than knowledge: `convert()`'s
`&NC→NC` arm stamps a monomorphization into `coutputs`, so running it speculatively would emit code
for a call that never happens.

**►► LOOKUPS COME OUT OF THE SOLVER — the direction, tentatively ◄◄** `LookupSR` is discharged in a
pre-pass: resolve the path, seed an `InitialKnown`, strip the rule, so the solver never sees one.
`RuneParentEnvLookupSR` is the in-tree precedent and its solve arm in `compiler_solver.rs` is a
`vwat` panic stating exactly that recipe. `Lookup` is the cleaner case — `get_puzzles` gives it an
empty puzzle, and its arm reads no conclusions and never touches `CompilerOutputs`, only
`env.self_env` and the path.

**The ordering change is wanted, not tolerated.** An unknown name is the easiest thing a user can fix,
so those failures belong before the solve rather than surfacing as downstream nonsense about runes
that were never going to resolve. Consequence to expect: in an already-erroring program a
`SolverConflict` may name a different rune, and some solver tests assert on humanized text.

**One thing stops it being a single pass over the rule list.** `GenericParameterDefaultS` carries its
own `rules`, injected mid-solve by the @DRSINI callback, and a default like `= int` contains a
`Lookup`. The pre-pass must walk `generic_params[].default.rules` as a second source, and decide what
happens when it concludes a default's runes and the default never fires.

**The general shape this is one instance of.** The solver's rule kinds split in two: structural
(`Equals`, `BorrowRef`, `WeakRef`, `OwnRef`, `KindList`) and resolution (`Lookup`, `Call`, `Resolve`,
`DefinitionFunc`, `CallSiteFunc`). Phase 2 is structural deduction only, so the resolution half is
phase-1 and phase-3 work implemented as rules — `Resolve` is literally SFWPRL's postponement wearing
rule clothing. `Lookup` moves first because it is the only one whose arm reads nothing.

**►► ANY PART OF PHASE 0 THAT MUST *EMIT* RATHER THAN CONSTRAIN INHERITS PHASE 4'S PROBLEM ◄◄** This
is where rustc's own separation breaks, and the reason transfers: `coerce_unsized` drives
`SelectionContext::select` directly from inside coercion, because a coercion must decide whether to
write an adjustment or not and there is nowhere to record *maybe*. On ambiguity it declines to coerce
rather than guessing. rustc's escape is a whole-body writeback pass that rewrites adjustments after
the fact — a stage we do not have, so a phase 0 that emits cannot be fixed up later.

`foo<T>(x &T)` called with an owned `Ship` shows why phase 0 must exist: the wrap chain reads
statically as `[BorrowRef]`, so the argument is adjusted to `BorrowRef(Struct(Ship))` *first*, and
only then does a send give the right answer. Seeding the raw `Struct(Ship)` would both conclude the
wrong value and hit the `BorrowRef` peel arm's `_ => unimplemented!()`.

**Do not read the split as "shape conversions before the solve, upcasts after."** A value type is
statically known whenever it has no unsolved runes left — fully concrete, or with every rune pinned by
an explicit template argument — and phase 0 upcasts in both of those. The rule is *as early as the
target is known*, and @PFVSZ's two halves are what make the wrap chain and the value-type template
readable that early.

**The wrap rules do the peeling — do not hand-peel.** `type_outer_ref_rules` are ordinary
bidirectional rules in the solve (`get_puzzles(BorrowRef) = [[inner], [result]]`). Seed
`full_type_rune` and the wrap rule fires in its peel direction and concludes the inner rune for free.
That is why the split is two *rule lists* rather than a rune plus metadata. **Defect 11 is exactly
this direction** — `solve_rule`'s `BorrowRef` peel concludes into `result_rune` where it means
`inner_rune` — so it blocks the mechanism phase 0 depends on, and is load-bearing rather than
incidental.

**►► BUT `type_outer_ref_rules` IS THE WRONG *SOLE* SOURCE FOR THE WRAP CHAIN ◄◄**
`translate_signature_templex` (`postparsing/rules/templex_scout.rs`) splits a parameter on **written
syntax** — it peels `BorrowRef` / `WeakRef` / `OwnRef` templexes into `type_outer_ref_rules` and
leaves the rest as `value_type_rules`. A parameter written bare as `x MyClass` has no wrap templex at
all, so it gets `type_outer_ref_rules = []` and `full_type_rune == value_type_rune` — even though its
real type is the anchored borrow `BorrowRef(ShareRef(Struct(MyClass)))`.

**►► CLASS SEMANTICS WAIT FOR THE LOWERING MOVE — DO NOT ANNOTATE THE CORPUS IN THE MEANTIME ◄◄**
`share` citizens are declared today and then treated as plain structs, because nothing lowers a bare
class mention to a claim. Nothing on the capability ladder depends on that changing. The alternative —
requiring `@` written everywhere until inference lands — costs roughly 109 annotation sites across 29
citizens, concentrated in the extern/export family, most of which would be un-annotated again once
lowering infers. **So the lowering move is the route by which classes start working**, which is the
justification it should be weighed on rather than the two that weakened.

**The position rule is not implemented, and it cannot be scout-time.** `ShareRefT` has five live
construction sites and none is a position rule: `generate_function_body_struct_constructor`,
`generate_function_body_struct_drop`, `ConstantStrTE::new`, `replace_value_type_in_ref`, and
`evaluate_custom_call` — the last of which keys on the expression's *already-computed* type, not on
any definition property. The only sharedness→wrap logic that ever existed is commented-out Coord-era
`CoordComponentsSR` handling in `compiler_solver.rs`. Classness is a property of the citizen's
*definition*, so the scout genuinely cannot know it.

**Upstream proves it by construction**: design-2:162 says the anchored borrow *"is what bare means in
a parameter, **which works only where the position rule can see the position**"* — and inside a
generic bound it cannot, since `T` is opaque, which is exactly why `@` was minted. If it were surface
sugar there would be no position where the rule fails and `@` would be unnecessary. **So it is a
typing-time interpretation, not surface sugar.**

**What this costs — less than it looks.** The chain a candidate is filtered on is *written wraps ++
the position rule applied given the citizen's kind*, and the second half is a **lookup, not a solve**.
The filter already reads each parameter's value-type *template name*; from the name you have the
definition, and the definition carries sharedness (`declare_type_sharedness` /
`struct_compiler_get_sharedness`). **Filter-is-final survives intact.** Two riders:

- **Phase 0 has the identical dependency** — it shape-adjusts against the parameter's wrap chain, so a
  bare class parameter gives it nothing to adjust toward either. One fix, not two.
- **A bare-rune parameter `x T` is genuinely undecidable at filter time**, because bare `T` is
  kind-polymorphic (design-1:387) — an anchored borrow at a class instantiation, an owned value at a
  struct one. The filter's "bare rune accepts anything" arm absorbs this, but by luck rather than by
  design; confirm it deliberately.

**This is the third instance of one upstream open problem, and reporting the trio is worth more than
reporting ours alone.** design-1:3028 (`^`'s kind-dependence, *"correct at one kind and wrong at the
other with no bound to disambiguate — no `T: struct` spelling exists"*) and design-1:2978 (`.`-autoref
needs a kind split that a symbolically-checked opaque `T` cannot make) are both open and both dated
07-26. All three want the same thing: **a way to know a type parameter's kind at a symbolic check.**

**►► A generic argument that needs an upcast must be written explicitly ◄◄** —
`launch<int>(&Firefly<int>())`, not `launch(&Firefly<int>())`. Rust's rule, chosen because it
simplifies the implementation. Deduction through an upcast *is* mechanically possible —
`get_impl_parent_given_sub_citizen` seeds an impl's solve from the **sub** side and reads the super
side out, so `Firefly<int>` yields `ISpaceship<int>` with no prior knowledge of `T` — but we
deliberately do not do it. Consequence: **no impl walking anywhere in phases 0–2.** Phase 4's upcast
runs between an argument whose type is known and a parameter type fully concluded, with no runes
left; that is `convert()` / `convert_via_upcast`'s existing job.

**►► There is no most-specific-common-ancestor. ◄◄** `launch<T>(a &T, b &T)` called with a
`Firefly` and a `Serenity` is a **type error**; the user writes the erasure. `T` unifies exactly —
first argument wins, the rest must match. Rust rejects the same program, and its LUB machinery
(`try_find_coercion_lub`) is reachable only from match arms, if/else, loop/break, array literals and
the return coercion — **never from call arguments**. This retires SMCMST/CSALR
(`docs/old/…/Infer Templar.md:501-576`), which chose "halt, then guess the most specific," and it is
consistent with three things already decided: Valen refuses variance outright, the overload redesign
says *no specificity, no fallback, no tiebreakers*, and `complex_solve` is already dead (see CSCDSRZ,
whose death is load-bearing). Note the two rulings above are independent: this one is about a
parameter that *is* a bare rune, where there is no target template to walk toward at all.

**What this retires.** `onion-typing-plan.md:103-105` planned `CoordSendSR` → `KindSendSR` with
"peels outer ref layers … then runs `is_parent` … dynamic self-replacement into `KindIsaSR`
preserved verbatim." **Superseded on every clause.** The peel is done by the wrap rules; the
`is_parent` walk is not done at all; the self-replacement dies with `complex_solve`. This reconciles
that plan with the architect's later ruling (convo-7:2706) that isa comes out of the solver because
it "was really just for arguments."

**Where the machinery already is.** `assemble_initial_sends_from_args` builds
`InitialSend { sender_rune, receiver_rune, send_templata }` at four sites and **its result is
consumed nowhere** — `#![allow(unused_variables)]` at the crate root is why nothing warns. That producer
becomes phase 0's output. It needs the shape adjustment, and it needs threading into
`solve_for_defining`/`solve_for_resolving` and on to `make_solver_state`. **It also currently sends
against `full_type_rune` unpeeled**, which is harmless only because the output is discarded; see the
open question below.

**►► A SEND IS `Equals`, AND EVERY PARAMETER GETS ONE ◄◄**

No guard, and no predicate deciding which runes are eligible. **The preview ordering is what makes
that safe**: by the time a send fires, phase 0 has already adjusted the argument to whatever the
parameter was statically known to be, so the send either agrees with the rules or carries the only
information there was.

| case | outcome |
|---|---|
| rune undetermined, one argument sends to it | seed; nothing to conflict with |
| rune undetermined, two arguments send to it (`f<T>(a T, b T)`) | both fire; agreement is a no-op, and **disagreement is the type error no-MSCA requires** |
| `f(x &ISpaceship)` called with `&Firefly` | parameter is statically known, so **phase 0 upcasts the argument first**; the send then agrees with the rules |
| `launch<int>(&Firefly<int>())` against `&ISpaceship<T>` | `T` is pinned by the explicit arg, so the target reads statically as `ISpaceship<int>`; upcast in phase 0, then agree |
| `foo<T>(x &T)` with an owned `Ship` | phase 0 auto-refs first, so the send seeds `BorrowRef(Struct(Ship))` rather than the bare kind |

The one case a seed could once have broken — a determined rune meeting a differing argument — is
exactly the case where the parameter type was knowable, which is exactly the case phase 0 converts.
That is what `CoordSendSR`'s deleted coercion-tolerance branch was working around, and ordering
removes the need for it rather than reinstating it.

**Explicit `T` is what makes the preview total.** `f<T>(a T, b &ISpaceship<T>)` with no explicit args
would need argument 0 solved before argument 1's target is knowable — interleaving sends with solving,
which is phase 2's job leaking into phase 0. The explicit-`T` ruling forbids that program, so phase 0
stays a single static pass rather than a fixpoint. That ruling was taken to simplify the
implementation; **this is where it pays.**

**What is foreclosed:** deciding any of this by solving first and patching up whatever is left
unsolved. That is halt-then-revisit — what `complex_solve` did, and its death is load-bearing.

**One thing this section does not settle.** Phase 0 must tell *"target not yet known"* apart from
*"target known and the argument does not match."* The first is the explicit-`T` error, the second an
ordinary type error, and both present as "cannot convert this." They want different diagnostics.

### In flight — the syntax migration, tiered so nothing is ever done blind

The corpus syntax migration splits by whether the change alters the AST, and the tiers are ordered so each has a verifier:

- **TIER 1 — spelling-only, do NOW, parser-verified.** `^`→postfix, `own`→`ownref` **(rename only — do not also narrow it; see below)**, `#!DeriveStructDrop`→`#explicitly_destroyed`, optional colons. These leave the AST identical (`^x` and `x^` are both `IExpressionPE::Move`), so the **146 `parse_sample_test!` cases plus postparse's 84 are a real verifier** — and they are green *independently of typing's state*. Not blind. This is the plan at `~/.claude/plans/compressed-whistling-sketch.md`, now approved. Doing it now also stops the corpus accreting more stale syntax.
- **TIER 2 — meaning-changing, do AFTER the feature lands, error-driven.** C1's `&` insertion, `*` insertion, the `set` / `set *` split. **Never sweep these.** Implement the rule so the old form becomes a *compile error*, then fix what the compiler rejects; a green suite means the migration is complete by construction. The hazard that makes a blind sweep unacceptable: a missed borrow becomes a strong ref and **still typechecks**.
- **TIER 3 — much later.** Position-dependent bare class, and anything riding on design-2.

**Tier-1 caret inventory** *(counted once, by a method not recorded; the shape is reliable, the
figures are not — **re-measure before sizing any sweep**)*. 511 carets repo-wide: **271 are Vale
source, 240 are not.** The arithmetic that matters: **252 plain prefix `^bareLocal`** (mechanical)
**+ 4 restructures + 12 type-position repairs + 3 dead-legacy**. The attribute counts beside them
*have* been re-measured — see the inventory under the inversion hazard.

- **The dangerous bucket is 218 humanizer caret-arrow carets**, concentrated in two files: `after_regions_error_tests.rs` (**107**, lines 219-225 and 576-581) and `compiler_solver_tests.rs` (**106**, lines 512-514 and 1076-1079). `typing/test/` totals 303 carets of which only **85** are Vale source — an earlier "~73" figure undercounted. A blind sweep mangles these.
- **FOUR sites are constructs, not spellings** — illegal under local-names-only, needing a local bound first. Three are in a `remove` function; grep the snippet rather than trusting a line, and **note there are two `hashmap.vale`s** — the one you want is `tests/hashmap/`, not `tests/regionhashmap/`, which is shorter and has neither site:
  - `tests/hashmap/hashmap.vale`, in `func remove` — **`^innerRemove(`**
  - same function — **`^(^maybeNeighbor).get()`**, an outer `^` on a parenthesized call result
  - `tests/list/list.vale`, in `func remove` — **`(^set temp = None<E>()).get()`**, a `^` applied to a `set` expression nested three deep. The nastiest of the four.
  - `move_call_via_caret`'s `"^Muta()"`
- **A separate bucket of 12 type-position carets, already broken today** — `func drop(self ^Moo)`, `func moo(m ^Muta)`, `wand ^Wand;`, `&[#3]^MutableStruct`. Two sit in currently-failing typing tests with `BadTypeExpression`. **These want `ownref`, not a postfix `^`.**
- **Two tests to handle deliberately.** `caret_type_is_error` asserts `^T` at templex level is a parse error, is **currently passing**, and **must survive** — it's the only `is_err()` assertion in all of `parsing/tests`, `postparsing/test`, and `lexing`. And `move_call_via_caret` asserts `"^Muta()"` parses to `Move(FunctionCall)`, is **currently green**, and is the exact inverse of the new rule — delete or invert it.
- **`own` is nearly free**: the `Keywords::own` field and its two `intern_str("own")` initializers, one consumer (`parse_templex_atom_and_call_and_prefixes`'s `OwnRefPT` arm), two humanizer arms (`IRulexSR::OwnRef` in `post_parser_error_humanizer.rs`, `KindT::OwnRef` in `compiler_error_humanizer.rs`), and **only two fixture sites**, both passing. **Zero `.vale` files use it** — the keyword landed in `41f88a790` and no corpus file caught up. The Rust identifiers (`OwnRefPT`/`OwnRefSR`/`KindT::OwnRef`) already read as "ownref" and need no rename.
  - **The rename is tier 1; the *narrowing* is not.** After it, the parser still accepts `ownref Point` on a movable struct, which design-1:1555 calls *"not terser or safer, it is wrong"* — `ownref` is for **immovable** types only. Rejecting it needs the movability axis, which we do not have. Ship the rename knowing the parser stays permissive; see the `own`→`ownref` entry in the Valen alignment section for the end state.

**BLIND-SURFACE ANSWER (the question this file used to ask).** `integration_tests/` isn't `#[cfg(any())]`-gated — it's **commented out of `lib.rs`**, so it compiles **zero tests**; its 68 carets, 20 `#!Derive` sites and 8 type-position carets get no verification at all and won't break if we get them wrong. `parse_sample_test!` covers **146 of 195** `.vale` files, but the blind `.vale` surface is **tiny**: 12 carets (the three `*restackify.vale` files) and 2 attributes (`regionhashmap/hashmap.vale`). The genuinely unverified bucket is **`builtins/resources/*.vale`** — 30 carets, 10 attribute sites, **not in the parse-sample corpus at all**; they reach the parser only via typing tests, which are **169 of 226 red**.

**►► `#!` IS SEMANTIC, NOT COSMETIC — THE INVERSION HAZARD ◄◄** `#DeriveStructDrop` → `CallMacro` (run it); `#!DeriveStructDrop` → `DontCallMacro` (**suppress** it), dispatched in `determine_macros_to_call` (`typing/compiler.rs`) and `compile_struct_core`. **Every attribute site in the tree is `#!`** — the lexer has arms for the bangless form but nothing writes it, so the corpus is uniformly *suppress*. Rewriting `#!DeriveStructDrop` → `#derive(StructDrop)` would therefore **invert every one of them.** **The target is `#explicitly_destroyed`** (ruling 13): a bare `#name`, admitted upstream, also meaning *suppress*, so the rewrite preserves meaning.

**Attribute inventory, measured**: `#!DeriveStructDrop` **65**, `#!DeriveInterfaceDrop` **14**,
`#!DeriveAnonymousSubstruct` **2** — 81 total across `.vale` and `.rs`. Only the first is a
one-for-one rename; see ruling 13 for why the other two are not.

~252 mechanical sites ⇒ `safe-script-runner` with a **context-aware** transform, never a global one.

**The rule for tier 1: change the `.vale` source text, never the assertion values.** The only deliberate exceptions are tests asserting the old syntax is rejected.

### Standing answers already given to Vale4

In case they come back on any: bounds are near-term, so
sequence behind them and prepare against roughly the shape the codebase implies, since what remains
is patching and hole-filling. A design pass before the work lands was **declined**, under a general
*make-it-work-then-adapt* policy that applies equally to `GenericParameterDefaultS` — which is **in
scope for rework, not settled**. `reachability.rs` is **optional for us**; they can have whatever
shape they want or write it themselves. `instantiating/` is roughly three weeks out. Regions are
independently roadmapped with the architecture settled, `convert()` belongs to the not-yet-started
dispatch mission, and their `is_primitive` finding is right but **the fix is renaming, not moving the
`Str` row**.

### Ready to start (no decision needed)

1. **Add the `*` deref operator** — ruled, small, and nothing is blocked on it, so it can slot in whenever. Parser (we have no `*` prefix operator at all) + postparse node + the two-depth `set` distinction. **Our decisions 2 and 13 already have the shape**: a lookup yields the address of the slot and the read-path `DerefTE` peels exactly one storage layer, so `k` is the stored reference and `*k` is simply *one more peel*; `set k = …` targets the raw `&&T` address-of-slot while `set *k = …` targets the `&T`. That is a parser addition plus a `DerefTE` at a new site, not a model change.

2. **Work the DEFECT INVENTORY** — a list of things that are actually broken, each with cited evidence. See the defect-inventory section below. Highest-value entries, roughly in order:
    - **The export gate panics on `exported func moo(firefly &Firefly)`** — an ordinary borrow param — and needs `peel_all_references` before both the `is_primitive` call *and* the map lookup, not just filled arms.
    - **A latent if-join assert-failure, now reachable since member access compiles.**
    - **Share upcasts don't work at all**, and **`inner_find_reachable_allocations`** in `testvm/heap.rs` is missing three arms.
    - **`SharednessImplingMismatch`** wants writing three lines from its precedent — the invariant `look_for_override` depends on is enforced nowhere and never was.
    - **Dead and verified**: `initially_known_runes` (safe, with an error-ordering caveat), `PrimitiveRuneTypeSolverLookupResult`, three warnings, the `lookup_rune_type` coercion (confirm by `panic!`-probe first; deleting its enum variants is a compile break).

3. **Member access is built, shape B, and `dot_borrow` no longer exists** — the `Dot` and `Index` arms
   of `expression_compiler.rs` each check that the container is a place and then peel with
   `peel_all_references` *only for matching*, leaving the expression's own wraps intact. Two arms are
   still stubbed there: a weak or placeholder container is a compile error, and a bare kind is an
   rvalue by construction (decision 7) wanting `make_temporary_local_defer`, coercion row 7.

   **►► THE CLAIM RULES LIVE AT THE CONSUMER, AND A STRAIGHT PEEL FOR A METHOD
   RECEIVER IS A USE-AFTER-FREE ◄◄** *(Ratified upstream, with the edges below.)* Peeling a `ShareRef`
   for matching decides nothing on its own; what follows decides the semantics.

   - **Traversal — peel, free, no refcount traffic.** Dot-field and indexing *through* a claim
     encountered mid-path never copy it, at either grade. `ships[0].name` reaches the payload directly
     and is legal in parallel bodies and freeze windows.
   - **Value exit — a claim copy, not a peel.** Where the place is *used as a value* — bound, stored,
     passed by value, or **returned** — the claim is copied. A bare class return is a strong claim
     (ruling 16), so returning an element is a claim copy discharged at the return. **The peel must not
     swallow return position.**
   - **Method receiver — routes through the anchored lowering.** The anchor is *found* by tracing to a
     written local (the common case, and free), otherwise **minted** into a hidden call-scoped claim,
     charged and window-barred. At a destructible place such as a list element the trace fails and the
     mint is mandatory: it is what keeps the receiver alive across a callee that empties the container.
     Peel straight to the payload for an anchorless element receiver and
     `ships[0].launch()` where `launch` clears `ships` is a use-after-free.
   - **A write through the traversed path that displaces a claim-bearing value escalates to the entry
     tier.** The payload-tier-only answer is the documented trap.
   - **Autoref at a claim place is per-callee, never uniform** *(ruled upstream)*: a clone receiver
     takes the borrow of the claim, while a payload-borrow method derefs through the claim first — a
     uniform peel and a uniform no-peel each make one of those two callable and the other not.

   Open upstream, and both touch this arm: whether the found-anchor case admits the quiet-window
   certificate, and where a minted anchor's release charge lands.

4. **Triage the `is_primitive` divergence.** `typing/types/types.rs` says `Str` is **not** primitive; `typing/compiler.rs` says it **is**; the export/extern ABI gate uses the second, so an exported function taking a `Str` param **skips the must-be-exported check**. Live defect on the boundary the interop work sits on. The same gate leaves `BorrowRef` / `OwnRef` / `ShareRef` as `unimplemented!()`.

### Owed, not urgent

1. **Run the design-2 audit** — parked, but well equipped. Read the "one architect / find the ruling the doc missed" framing in the banner above and the **design-2 provisional map** in the upstream-rulings section: several provisional items sit *inside* ratified chapters, and design-2:957 says the doc contradicts itself on purpose about inc/dec. Then the method, below.

   **►► AUDIT METHOD — read before running the design-2 pass ◄◄** *(Distilled from the design-1 audit, whose separate doc has been folded in here.)*

   **design-1 and design-2 specify the SURFACE LANGUAGE.** They say nothing about internal type-system machinery and explicitly exclude implementation status. So ***"the doc doesn't mention X" is NOT a finding.*** Only ***"the doc rules X, we do Y"*** is.

   The design-1 pass produced **three false positives, all the same shape**, recorded because the pattern is easy to repeat:
   - **`&&`** — absent from the surface because it is inert type-space machinery, which is exactly what our own decision 3 says about it.
   - **`DerefTE`** — absent from the surface because it is the *lowering* that implements a stated surface rule ("a borrow is `Copy`").
   - **C1's reach** — I asked whether C1 covers returns and if/else arms. The doc answered it, in a line already quoted earlier in the same session.

   Two of the three were *"this machinery may be unmotivated because the spec doesn't name it."* **A surface spec never names machinery. Check whether the thing implements a stated rule before concluding it is orphaned.**

   **And check dates.** Three pre-C1 artifacts surfaced in a single afternoon (design-1:1332's `t.clone()` spelling, design-1:171's auto-deref paragraph, a bare non-`Copy` place passed to `replace()` in a port). **A stale spelling reads exactly like a divergence.** Corpus examples are *design evidence, never conformance fixtures.*
2. **The confirmed alignment items** — no-shadowing enforcement (we have none), `Vec`/`List` tier split, the `Vec<int>`-elements-are-a-child-group pin, and `comptime`. All in the Valen alignment section below. The attribute rewrite is tier 1 of the migration and targets **`#explicitly_destroyed`**, not `#derive(StructDrop)` — see the inversion hazard.

### ►► THE COMPLETE OPEN LIST ◄◄
Everything below is open. Nothing else is. Grouped by what unblocks it.

> **►► READ FIRST: almost none of this gates the next work. ◄◄** The capability ladder needs no decision across its top rungs. Generic bounds needs nothing from the region or effect design; export/extern needs the `is_primitive` rename plus `peel_all_references`. **Every design item below gates rung 0 and beyond, which sits behind the whole ladder.** Do not let this list set the order of work.
>
> **Closed, do not re-open:** the parse bucket (50 → 1, and the one
> survivor is an architect call, not work); `ensure_deep_exports`' silent under-approximation
> (found, reproduced with a new test, fixed); the `implements` postparse half; the Components /
> `Kind` / `ITypePR::KindType` removal; the four discarded `SolverConflict` payloads; and the
> `rule_scout` catch-all that named nothing.
>
> **Also closed, do not re-open:** the six-phase call-site pipeline (phases 0
> and 4 added); **filter-is-final** with a purely static candidate filter; **no most-specific-common-
> ancestor**; **explicit `T` when a generic argument needs an upcast**; **overlapping impls outlawed**
> (decision 16); **overlapping overloads outlawed**; and the share clone at class kind
> **compiler-synthesized rather than written**. All are recorded in CALL-SITE PHASES, §Overload resolution, and decision
> 16. **The `>1 → ambiguity` branch stays live** as the cross-namespace backstop — do not delete it as
> unreachable.
>
> **And:** **a strong ref contributes its payload's
> namespace too, as an ordered union** (Rust's arbitrary-self-types shape, automatic for strong refs
> only, `Box<T>` auto-derefs as Rust does) — which **answers "does `&Ship` mention `Ship`"**: yes, and
> a strong ref mentions its payload as well. See CALL-SITE PHASES. **Generic bounds** (`solve_rule`'s `KindList` arm) and **applied
> generics** (the template-position scout defect) both fail zero tests and are no longer clusters.

**Design questions still open (ours)**
- **The effect representation.** A bare `mutates: RegionT` is too narrow (upstream answer 21). Live candidate is the per-group permission map; the axes are `held` (destruction), `dangle`/`opaque` (dereference), and a possible `softmut` tier — **partly independent flags, not an ordered level.** Needs an eager canonical form for the group algebra, since map keys are group expressions.
- **Our clone bound has no effect slot** (upstream answer 25).
- **Provenance / "contributing site."** Undercut by defect 15 — most AST nodes carry no `RangeS`, so a genuinely post-hoc checker cannot point at the offending line. Decide whether to add ranges or thread a side-table (which makes it not post-hoc). `CaseRuneFromImpl { inner_rune }` remains the in-tree precedent for canonicalize-the-value-keep-the-origin.
- **"Params get runes, locals get classified" needs amending** — answer 19 says some group parameters can be *independent*, so the clean split doesn't hold as stated.
- **`BorrowState`'s shape** — still correctly parked behind its stated trigger.
- **Telling phase 0's two failures apart** — *"the target is not knowable yet"* (the explicit-`T` error) versus *"the target is known and the argument does not match"* (an ordinary type error). Both present as "cannot convert this" and want different diagnostics. Everything else about sends is settled; see CALL-SITE PHASES.
- **►► Can phase 0 be a single static pass, given lambdas? ◄◄** rustc's `check_argument_types` runs a two-pass loop — non-closures, a solver drain, then closures — because a closure literal's parameter types are deduced from the expectation, which the *other* arguments must pin down first. That is attributable to neither lifetimes nor coherence nor rustc's lack of overloading, so it applies to us verbatim. Explicit-`T` makes the preview total for *type* arguments and says nothing about a lambda whose signature depends on a sibling argument. **Do not treat "phase 0 is one pass" as settled until a lambda case is worked through.** If it does need speculation, the shape to copy is rustc's `fudge_inference_if_ok`: guess inside a rolled-back snapshot, keep only a hint, re-verify for real.
- **Where the wrap chain comes from**, given that `type_outer_ref_rules` cannot be its sole source. The position rule is a typing-time interpretation, not surface sugar, so a bare class parameter's real chain is invisible to `translate_signature_templex`. Both the static filter and phase 0 need it. The shape is known — chain = written wraps ++ the position rule given the citizen's kind, and the second half is a lookup — but its home is not. **Blocking nothing today**, since neither the filter nor phase 0 is built.
- **Where a minted anchor's release charge lands**, and whether the found-anchor case admits the quiet-window certificate. Both are open *upstream* and both land on the consumer-side claim rules; see the fork under "Ready to start".
- **What the expression `&x` forms at a claim-typed local** — payload borrow by concrete sugar, compositional borrow-of-claim, or a one-hop argument coercion. Open upstream, and they have asked for our input, since our lowering implicitly picks a horn.

**Decisions only the architect can make**
1. **Rung 0 — do we start?** The gate on the entire borrow-checker track, and everything in the long-term horizon sits behind it. **Its representation work is scoped** (see the regions block): `ITemplataT` needs a `Region` variant, `RegionT` becomes an interned recursive algebra, and two prerequisites are typing-slice work regardless — argument types reaching the call-site solve, and `substitute_templatas_in_kind` for ref wraps.
2. **Where does mutability live?** design-1 puts it on the **group**, via signature effect clauses (`func heal(e: &Entity in g) mut(g)`), and calls that its one departure from Rust (*"there is no `&mut`"*, design-1:361). We dropped `&mut` and never added effect clauses, so mutability currently lives **nowhere**. Sits *behind* rung 0, not beside it, since effect targets are group expressions. It is bigger than "add effect clauses": `mut(g)` / `mut(g.tiles[])` / `mut(E)` with `E: Effects` / `mut(())`, the subtractive `not(mut(…))` forms, the deep-effect rule, parameter shorthands that desugar to **fresh anonymous group params instantiated per call site**, plus solve-order pins (design-1:1235 — no negative bound discharged against less than `E`'s full solution; re-check on widening). That is a **second solver domain** alongside types and groups, i.e. `ITemplataT::Effect` beside `ITemplataT::Region`.
3. **Effect vocabulary staging** — full set or `mut(g)` first? And is effect *inference* in scope initially? (Ruled: named functions **declare and are checked**, so no call-graph fixpoint; closures genuinely infer, and a *recursive closure* is an unaddressed gap.)
5. **`is_primitive`** — the fix is *renaming*, not moving the `Str` row. Which two names?
6. **The three override mismatches** — make the abstract bare (matching its four siblings) or borrow the overrides? Lean exact-match: Valen refused variance deliberately. **Note the diagnosis changes once position-dependence lands** — today it's borrow-vs-strong-ref; after, it's borrow-of-payload vs borrow-of-claim.
7. **Tier 1 — go?** All three parts are ready and measured: `^`, `own`, and
   `#!DeriveStructDrop`→`#explicitly_destroyed`.
8. Smaller: delete the 3 removable warnings; keep or delete `evaluate_addressible_lookup_for_mutate`'s shadowed `IVariableT::Capture` arm, which holds un-ported Scala; when to rewrite the `&&`-as-weak corpus sites (compiler can't drive it).

**Waiting on upstream — exactly four, and this is the list.** (1) Whether **`mut(E)`'s `E` is a
group or its own sort** — they carry our framing of it, with attribution, and it is genuinely unruled.
(2) Where a **minted anchor's release charge** lands. (3) Whether the **found-anchor** case admits the
quiet-window certificate. (4) The **by-value-claim drop fix** — `x T` by value at a claim-typed `T`
decs the claim at scope end, possibly to zero, under an *empty* effect clause, because the drop-side
generated bound carries no effect half where the clone side does; until their fix lands, the drop
path must not assume purity there. Items 2 and 3 land on the consumer-side claim rules and are the
only upstream items any near-term work touches. They have also asked for **our** input on the
`&x`-at-a-claim-place question in the open list above.

**Settled with upstream, so do not re-raise:** the `&&T` source-route witness is folded into
design-1's open question with attribution — the ruling can rest on written-type formability plus our
address-of-slot reason. The kind-bound proposal is recorded against both surviving kind-dependence
items (`^` and `.`-autoref) as the first concrete carrier proposal; **our own third instance is
retracted**, because under the ruling above neither insertion happens at a rune. And `.`-adjustment
is ruled — see the `ShareRef` fork under "Ready to start".

**Two divergences from the corpus, both long-term:**
- **`interface` vs `open trait` is a keyword here and a predicate upstream.** We treat sharedness as *carrying* the split with no separate keyword. design-2:876 classifies by **impl coverage** — a trait is an `interface` iff a class implements it *and every class impl fits the ambient-multi cover* — with blanket-impl traits forced to `open trait` regardless. A declaration keyword cannot express that. Live on `ensure_deep_exports`' `KindT::Interface(_) => {}` fork.
- **A "do not fix" pin we don't carry.** design-2:481 (R1b): `set tile.a = None; set tile.b = None` over two strong fields must **keep failing**, because statement 1's discarded temp drops at end-of-statement and invalidates `tile`. *"Making the discarded result's drop lazier would rescue this shape and break R1's no-observable-gap guarantee; do not 'fix' it."* Belongs beside decision 11's ordering pin — which phrases the order *install-first, hand-back-second* while design-1:59 phrases it *out-then-in*. Both land in the same place **only because the drop is end-of-statement**; say so wherever the lowering goes.

**The design-2 audit is deferred**, twice, by the architect. Its method and dating discipline are in
"Owed, not urgent"; the provisional map is in the upstream-rulings section. A second silent arm in
`ensure_deep_exports` remains **by choice**: `KindT::Interface(_) => {}` checks nothing, with a
`VCOORD` stating the tier fork rather than picking it.

**Build queue** — consuming the sends (the largest cluster, and the one needing a ruling), the
fifteen-item defect inventory, the verified dead code, tier 1, the `*` operator, and
**deleting the `implicit_clone` probe**, which the design retired and the code still carries — see
"What blocks / what to preserve" for its extent.

**Restore the `@T` share templex** — parser `ShareRefPT` → postparse `ShareRefSR`
→ `IRulexSR::ShareRef` → a solve arm. Its **one** surface use is the **by-value claim parameter**
(`x @Ship`, which changes which side owes the release and so affects purity; spelling still
provisional upstream) — the share clone is compiler-synthesized rather than written, and a bare class
name in type-argument position already denotes the claim, so `Crate<Ship>` needs no `@` either.
`KindT::ShareRef` already exists, so this is surface-only. Do it alongside the still-unfilled
`WeakRef`/`OwnRef` wrap arms — all three want the same bidirectional body, and nothing in the corpus
reaches any of them today. **Cleared to build.** `@` is a *normalizing operator* — identity at every
non-class kind, `@@T` reduces to `@T`, and a redundant `@` is rejected as literal source with a fixit
while normalizing silently when it arrives by substitution. **A wrap node minted only at rc-class
kind is a conforming implementation of that.**

**And when it lands, decide which bucket `ShareRefSR` goes in** — `type_outer_ref_rules` or
`value_type_rules`. The architect leans outer-refs: the peeled value type is what drives namespace
lookup, so `x @Ship` should still search `Ship`'s namespace for callable methods.
Note `translate_signature_templex` peels **three** wraps today while PFVSZ's stated
invariant and `ParameterS::new`'s `debug_assert!` both say **four**, and typing's
`peel_all_references` peels four — so the tree already answers this two ways.

**One parsing call still outstanding** — `func bork() ^&SomeStruct` in
`can_turn_a_borrow_coord_into_an_owning_coord`, the last parse failure. Mechanically `^&X` → `&X`, by
the same rule applied to `^Moo`: an owned value is a bare kind (decision 9), and every drop in the
`.vale` corpus already spells it bare. **But that erases what the test exists to check** — converting
a borrow coord into an owning coord, a conversion the onion may not have. Change the source and the
test no longer tests its name; delete it and that is a call.

**Doc work** — fix the *"interface" means two kinds* vocabulary collision in `docs/convos/rust_interop/vale-rust-interop-architecture.md`, **now ours**; and `onion-typing-scouting.md` remains partially stale.

**Closed, recorded so nobody re-opens them:** the `set` spec report (sent + ruled); the Vale4 reply (sent); the Luz curate queue (drained + committed); `Guardian/Luz` (deleted, 7 symlinks deliberately broken); the design-1 audit doc (folded into this file + deleted); the `lookup_rune_type` coercion arm (**dead by evidence** — probe run, suite byte-identical, zero hits); the move-tracker join question; the `self`-receiver hazard; the borrow-creation seam; per-body checking; and every one of the seven "genuinely unfinished reasoning" threads from the start of the arc.

### Designed but not started
The **region borrow checker** — ladder, quarantine rule, and rustc evidence are recorded in `docs/plans/path-to-borrowing.md`. Rung 0 (groups become real) is entirely architect work and is gated on decision 1 above.

## Current state

The onion reference surface is in the parser and postparse, and **the typing slice compiles** — the
compile cascade that defined this arc is finished. What remains is filling stubs, not chasing type
errors. Everything outside typing is green: parser, lexer, postparse, solver, utils, humanizers.

**`simplifying/`, `hammer/`, `backend/`, `testvm/` and `integration_tests/` stay commented out of
`lib.rs`.** The `valec` bin does not compile either — its driver references the gated `backend_ffi`
and `pass_manager::pass_manager` — which is accepted mid-arc red and is not in the `--lib` count.

### Measurement traps

- **`cargo check --lib` hides every test.** `typing/mod.rs` gates `pub mod test;` behind
  `#[cfg(test)]`, so the lib number excludes all of `typing/test/`. Quote the **test-build** number.
- **Deleting a dead import raises the error count.** Rustc suppresses body uses of a name whose `use`
  failed; removing the import unmasks them. The count moves in both directions for reasons unrelated
  to progress.
- **A live parse error blanks out a file's diagnostics.** While one exists, that file's error count
  is noise rather than signal.
- **`expect_kind_templata` is filled** (`typing/templata/templata.rs`) and is not a front line. It was
  once the single largest cluster; do not re-plan against it.

Every count in this file is a snapshot for shape, not a target.

### The parser/postparse surface

- **`own` is the `OwnRef` wrap** — parser `OwnRefPT`, postparse `OwnRefSR`, `translate_own_ref_templex`,
  humanizer, onion-wrap permitted-list and traverse, all mirroring `WeakRef`.
- **`@` is dropped from the surface** — both the `@T` share templex (`ShareRefPT` / `ShareRefSR` /
  `IRulexSR::ShareRef`) and the `@x` share expression (`SharePE` / `LoadAsP::LoadAsShare`).
  **►► THE `@T` TEMPLEX HALF MUST COME BACK ◄◄** — the by-value claim parameter (`x @Ship`) needs a
  spelling and `@` is unwritable today. See the `@T` restoration item in the build queue.
  **The `@x` expression half stays dropped.**
- **`heap` is dropped entirely** — the `heap T` surface path. The value-model `HeapOwnRefT` under
  typing is untouched.
- **`borrow` and `share` keywords are retired** (dead registered fields, never consumed).
- **`held` is a region**: `RegionP` / `RegionSR` = `Unspecified | Held | Rune`.

### Invariants and archaeology from the migration

The mechanical sweeps are in `git log`. What is worth keeping is the things a future change could
break without noticing, and the archaeology that stops someone resurrecting a bug.

**Traps and load-bearing invariants**

- **`determine_closure_variable_member`: `OwnRef` deliberately falls in the *wrap* branch** —
  capturing an `own` local stores a borrow — resting on the live drop invariant in
  `struct_compiler_core.rs`, *"drops only capture borrows"*. **Flipping that is adding
  `| KindT::OwnRef(_)` to the first arm**, and nothing else.
- **`is_light()` treats Extern, Abstract and Generated bodies as `light`, not lambdas.** That is what
  lets a generated `drop` compile.
- **`FileCoordinateMap.file_coord_to_contents` is an `IndexMap`, not a `HashMap`** — deliberately, to
  keep the `Compiler::evaluate` loop @IIIOZ-compliant, since it iterates into the environment seed.
- **`^x` routes to `Unlet`, not to `Ownershipped`.** The scout's `coerce` sends `Use` and
  `LoadAsBorrow` to a plain `LocalLoad`, `Move` to the existing `IExpressionSE::Unlet` node — the same
  op as `unlet x`, converging them — and `LoadAsWeak` to `Ownershipped(LocalLoad)`. `^local` therefore
  reuses the `Unlet` typing handler, which already calls `mark_local_unstackified`, **so the move is
  tracked**. The old `Ownershipped` path did not track it.
- **`replace_value_type_in_ref` copies the wrap shape blindly.** Whether it should instead enforce the
  validity table — promoting a share citizen to `ShareRef` — is **undecided**. Its one surviving ZHERE
  is in `as_subtype_macro.rs`, covering two call sites.
- **`TypingIgnoredParamNameI` sits dead in the gated instantiator**, unverifiable while
  `instantiating/` is commented out.
- **Two test assertions are deliberately shaped and reversible**: `compiler_mutate_tests` asserts
  `.result.inner`, because a lookup yields a *borrow of* the thing (decision 7). Say so if you want
  the full `BorrowRef` shape asserted instead.

**Archaeology — do not resurrect these**

- **`SelfCoordRuneS` was deleted by the postparse slice as if it were Coord-era. It wasn't.**
  `SelfFullTypeRuneS` was added back across five parallel sites so the anon-interface forwarder's
  self param can carry the abstract param's wraps. **A name-based sweep trap, the same shape as the
  one that took out `implements`.**
- **`call_compiler.rs`'s `Borrow|Share` vs `assert == Borrow` mismatch was a regression from
  `71e91d6a2`**, the sharedness-arc squash, which deleted the `get_mutability`-derived expected
  ownership and hardcoded the assert. Both the Scala and the faithful port at `7a65955a0` were
  coherent. Under the onion the assert is structural, so nothing needs resurrecting.
- **Placeholder sharedness was removed**, and with it `get_sharedness` and `lookup_mutability`. Citizen
  sharedness (`declare_type_sharedness`) was untouched and is the live query.

**Wiring that is easy to mistake for a gap**

- **@PFVSZ rules are folded into the solve at BOTH the defining and call-site paths**
  (`function_compiler_solving_layer.rs`). A user parameter's type-binding rules live per-param, as
  `value_type_rules` + `type_outer_ref_rules`, and the solve originally read only `function.rules`, so
  param runes were never bound. Both paths now fold
  `params.flat_map(value_type_rules ++ type_outer_ref_rules)` into an `all_rules` set feeding both
  `definition_rules` and `derive_rune_to_type`. **The reachable-bounds list keys on `value_type_rune`,
  not `full_type_rune`.**
- **`CopyPrimTE` peels its borrow.** `__copy_prim(&P)` must yield a bare `P`, so the arm calls
  `peel_one_reference` and asserts primitive.
- **`add_zero_arg_call_rule`** takes no `arg_runes` — neither caller ever passed a non-empty vector.
  It is `@TNLTZACZ`'s producer, and `ITemplexPT::Call` deliberately does *not* share it, because it
  mints its result rune before translating the template and args and that ordering feeds the rune's
  `LocationInDenizen` path.

**`is_type_convertible` is wrong, not merely incomplete, and the tripwire keeps it loud.** It panics
for `&X→X` and `X→&X` — two shapes `convert()` handles but the predicate would silently reject. Its
reference arms cover a both-borrow-refs recursion and a primitive `&P→P` read-out; the recurse guard
**wrongly accepts `&&X→&X`**, which is harmless only until genuine double-borrows exist. The real fix
is aligning with `convert()`'s coercion-table arms, or driving it off a dry-run `convert()`.

**►► `is_type_convertible` LOSES BOTH OVERLOAD JOBS, not just the tiebreak.** The tiebreaker is ruled
for deletion, which retires the exact-vs-coercion bool, and membership no longer goes through it
either — the candidate filter is **purely static**, so it never asks "does this convert?" at all. What
survives is phase 4's real conversion and the gate-checks, both of which want `convert()` rather than
a predicate.

**A commit-shape note that has mattered twice:** when the Vale4 worktree (`experimental-4`) is checked
out on our tip, land corrections as a **follow-up commit rather than an amend** — amending rewrites
the commit under them.

## Upstream rulings and answers

**Numbered 1–27, and the numbers are cited from elsewhere in this file, from the convo exports, and
by upstream — do not renumber them.** Most are rulings and binding on us. **Four are not**, and
treating them as settled will get the next case wrong: **19** warns against banking a simplification,
**22** records a confirmed doc gap (*"genuinely underspecified"*), and **24** and **25** are findings
about *our* compiler rather than decisions about the language. Message archives are under
`tmp/messages/`.

**A framing correction first, because it changes what an audit finding IS.** There is **one architect**, not two — the same person owns Vale2 and Valen. So when design-1 contradicts a ruling, the live hypothesis is *"the doc is behind,"* never *"two authorities disagree."* The design-2 audit should hunt for **rulings the docs haven't caught up with**, not for divergences. Ruling 1 below is exactly that shape: our decision 11 turned out to match a Valen ruling from `valen-approach-convo-19-constraint-class-hybrid.md:1967` that was never folded into design-1.

1. **`set` yields the displaced old value**, when the type is movable. **Confirms decision 11.** Mechanism: *move the old value out into `set`'s result, then move the new one in, then let control flow proceed* — the **ordering is load-bearing**, see the motive below. design-1:1703 and design-2:457-459 are **superseded**: `set` on a linear *(was "linear-strict" — see ruling 13's vocabulary inversion)* place is no longer an error (it yields a linear value, and discarding it is the ordinary unconsumed-linear error, pointed at the value rather than at `set`); the revival-`set` exception becomes a degenerate case rather than a carve-out; `replace()` loses its special standing. **Swapping falls out of chaining**: `set x = set y = set x = None`. Empirical backing: the 24-file `ValenRL-Single` port runs on this ruling with **zero** `replace(` in actual code.
2. **`*` is restored to the language.** `k` = the reference; `*k` = the pointee; `set k = …` **re-points**; `set *k = …` **writes through**. Field and method access still auto-deref — `k.field`, `k.method()`, `set self.hp -= 10` unchanged, no `(*k).field` anywhere. This **vindicates design-1:169** and **flips design-1:171, design-1:1496, and rubric:161**, which all said a bare mention names the pointee; §171's *"Valen has no such ambiguity to resolve"* was simply false. The key-walk becomes `keys.append(*k)`. It also makes `replace` writable in-language, closing design-1:2551(b)'s grammar half: `func replace<g': T>(r: &T in g mut, new: T) -> T mut(g) { return set *r = new^ }`.
3. **`Copy` is opt-in, like Rust** — `#derive(Copy)`, not structural. Consequence to expect: `struct Point { x: int, y: int }` is **not** `Copy`, so `foo(p)` is a C1 error until someone annotates it.
4. **Class `for` binds the fail-fast cursor** (Java `modCount`-style; `debug_assert` on structural mutation, compiled out in release). Struct collections keep the compile-time-poison lending cursor. Not near-term for us. The reasoning is a design invariant worth keeping: **`*` should live where group parameters live** — the fail-fast cursor yields a claim copy or `Copy` value rather than a payload borrow, so a rung-1 user iterating a `List` never holds a borrow-typed loop variable and never meets `*`.
5. **The widening rule is "no binding with a narrower claim survives it"** (design-1:1037) — **NOT** "the merge doesn't outlive the call," which was our first formulation and would have rejected the accepted code at design-1:1024-1026. Non-persistence is one sufficient condition, not the rule; container inserts and joins are merges that legitimately persist. Corollary: **merging and declared aliasing-relations are orthogonal** — `maybealias` is not what unlocks a widening (widening needs *containment*, and `a ⊆ a+b` definitionally). Call-site group arguments reject borrowed-or-owned (design-1:1035, rationale unstated and routed).
6. **The anchored-borrow class reference model is RATIFIED** (design-2:145, *"ruled 2026-07-23"*). `softmut`'s provisionality (design-2:185) scopes to exactly one half — **callee purity, and only when the anchor is minted rather than found**. Our FFI direction rides on the *position rule* (parameter = borrow of a claim), which is the ratified half, so **the pre-+1'd boundary direction is on solid ground.**
7. **`Copy ⟹ Clone`**, and `#derive(Copy)` implies `Clone` — no writing both, a deliberate ergonomic divergence from Rust. **Consequence we can bank: `implicit_clone` retires completely.** Primitives are `Copy`, `Copy ⟹ Clone`, so `clone` covers the one case the probe was being retained for. That is what retires the whole target-site probe family, `BorrowRef(NC) → bare NC` included.
8. **`.clone()` copies as deeply as it can terminate** — through owned storage and through borrows, **stopping at claims and weaks**, where the graph is unbounded. Claims-as-base-case is not an ergonomic choice: without it `#derive(Clone)` never terminates on the corpus's tile↔unit cycles, so `Rc::clone`-as-base-case is preserved for Rust's own reason.

    | receiver | `.clone()` yields |
    |---|---|
    | `&Missile` (borrow) | a `Missile` — **reaches through**; honest error if `Missile` has no clone |
    | `MyClass` (claim) | another claim, an inc — the base case |
    | `weak T` | another weak (it's a key, and `Copy`) |
    | owned struct | deep copy, recursing through owned fields and `Box`, **stopping at** claim / weak / borrow fields |
    | `&MyClass` (payload borrow of a class) | reaches the payload → needs the hand-written deep clone → **errors** if the class has none *(deliberate, confirmed)* |

    Alongside: **deep-cloning a class is hand-written, never autogenerated** (the clone boundary is a semantic choice — a compiler walking fields can't know the chased unit should be shared while the path list is copied), and **a linter catches `.clone()` on a claim**, since rung-1 users will expect a deep copy. **Accepted residue:** `path_to_target.clone()` still yields a second handle to the same list, so the FINDINGS #14 aliasing bug survives as deliberate rung-1 Java behavior — the answer for "I want a new list" is a container API (`List.from(other)`), not `.clone()`.
9. **The expression `&e` on a borrow-typed place yields `&T`, not `&&T`** — deliberately scoped to *expression formation*, not to type formation. See decision 3 for why the scoping is the whole ballgame for us.
10. **Call-site widening is LEGAL with a written union and moved arguments** — `f<a + b>(overlay_a^, overlay_b^)` compiles; `f(overlay_a, overlay_b)` stays rejected. The rule: *widening fires where the destination union is **written**, the source is **owned and consumed**, and **no narrower binding survives** — which a struct move establishes and a **strong-ref move does not**.* Calls join container inserts and owned `dyn` erasure as a **third** widening site. Design-1:1035's "borrowed or owned" was aimed at **strong refs**, never at moves (`concerns/soundness/02:30`: *"Do NOT phrase the rule as 'owned-value insertion' — that includes strong refs and silently re-opens the hole"*). **The strong-ref half is the part to implement carefully**: moving an RC handle does *not* establish no-narrow-binding-survives, because sibling claims persist.
11. **`and` / `or` / `not` — all three.** No `&&`, no `||`, no unary `!`. `!=` stays (Python-shaped: `!=` for inequality, `not` for negation). **We were already right here** — this was Valen moving to *our* spelling, not us diverging. Their corpus has 62 files to migrate; ours is already correct.
12. **`!mut(...)` is re-spelled `not(mut(...))` — parens REQUIRED.** `not(mut(l.items))`, `not(mut(T::capture_group))`, `not(mut(g...))`. Anything written against `!mut` needs re-spelling, including the Pin C conformance note in the borrow-checker section.
13. **DROP IS AUTO-GENERATED, and the linear/affine vocabulary was INVERTED in the docs.** See the dedicated block below — this one propagates through everything we've written.
14. **`#derive(Copy)` is rejected on a type with a *user-written* drop.** The auto-generated structural drop **never** blocks it — otherwise no struct could ever be `Copy`, since every struct now has a drop by default. The all-fields-`Copy` derive condition does most of the work by itself; the user-written-drop check catches the residue (`File { fd: int }`, where the obligation attaches to the *type* rather than being inherited from a field). **Implementation consequence: the gate must check for a `drop` FUNCTION (never the `T: Drop` bound — see the trap below) AND further distinguish *declared* from *synthesized*, or it rejects everything.**
15. **`#derive(Copy)` and `#explicitly_destroyed` are mutually exclusive** — not for safety (each copy carries its own obligation and definite-consumption enforces both) but because the annotations cancel. **Flagged upstream as a possible future relaxation, so do not build it as load-bearing.** By contrast **`#derive(Clone)` on an `#explicitly_destroyed` type IS allowed**: `Copy` silently multiplies an obligation, `.clone()` is something you asked for out loud.
16. **Bare class `T` in RETURN position is a strong claim** — closing the design-2:961 gap. Forced, really: an anchored borrow's anchor is the *caller's* claim, and on return there is no frame below to anchor it.
17. **The C1 error menu is conditional** — print only options the user can actually take. `&x` / `x^` always; `x.clone()` only if the type has a clone; `#derive(Copy)` only if the type is Copy-eligible. Free for us, since the eligibility predicate must exist anyway to check the derive. (Related known bug upstream, rubric:176: the menu currently *"offers `.clone()` on linear types, which do not have it."*)
18. **`Class<T>` is DEFERRED** after an adversarial pass — don't build against it. The blocker is sharp: for a class, destruction fires at `rc → 0`, which is not a point a user can insert a consumption at, so a class holding a **linear** payload is stranded. Five sub-decisions survive if it is ever revived (no auto-wrap; auto-deref yes; shallowest-wins on collisions; `.clone()` yields another handle; conditional menu entry).
19. **There IS a "Milano case" for groups — do not bank the simplification.** design-1's full-grammar example declares `outer_g` appearing **only in a where clause** (`where g in outer_g`): no parameter carries it, no return position mentions it, no path derives it. Seed the solver with arguments and `g` falls out while `outer_g` does not. **How `outer_g` resolves is not stated** — the explicit group-argument spelling `f<g>(x)` is attested with no deduction story. **Treat independent group runes as live; the representation must not preclude them.** Two of our three counter-bullets did survive: the parameter shorthand is determined by construction, and return position is determined — but as a *binding site checked in the callee's body*, not deduced from arguments (*"returns are not a provenance-laundering channel"*). **And where we predicted it would first appear — impl-has-more-params-than-interface — Valen answers it differently from Milano: that shape is *erasure*, and the rule is COVERAGE.** A value may erase to `dyn Trait` only if the dyn type's bound associated groups contain **every** external group parameter of the concrete type; erasure may widen mentions, never drop them. Corollary: a concrete type with external group params cannot erase to a trait declaring no associated groups. So types mint an independent rune; **groups get absorbed by widening, or the erasure is rejected.**
20. **Forming a borrow through a binding IS a use of that binding** — so deriving through a dead reference is caught by the ordinary use rule, and creation is innocent *as a rule*. Creation stakes no claim (no `&mut` to conflict with); *"holding a reference constrains nobody; mutating with live aliases is legal; only using a stale reference is an error."* **BUT there is one attested creation-site rejection that is not reducible to a use** — the **joint-argument check**: *"an argument move of `x` is a destruction event for the call's own binding check: no sibling argument of the same call may bind into `x`'s group or territory."* `f(&x.field, x^)` is REJECTED, and the argument list is checked **as a set** precisely so no evaluation order need be fixed. **So two entry points, confirmed.**
21. **Effect TARGETS are group expressions; EFFECTS are not.** Every associated effect budget in the docs is declared **as a group** — `comptime Advance: group = ()` (the step effect), `comptime teardown: group`, `comptime capture_group: group`. **No `comptime X: Effects` exists anywhere.** So the group algebra is the whole target language. **But an effect carries a permission axis the target cannot hold, and two of three are ratified today:** (a) **`held`** exempts a borrow from *the destruction component* of entry-tier reach "and only that" — payload-tier and child-group reach pass through unchanged, and design-1's own gap paragraph calls this *"the minimal instance of Valen 1's reserved 'member writes yes, destruction no' middle-tier gap"*; (b) **`dangle(g)` / `opaque(g)`** is a verified no-dereference promise over a group the signature may still otherwise charge, and it **propagates through the call graph**; (c) **`softmut`** (provisional) would add a tier between pure and mut, and its audit re-keys ~40 mut-keyed rules. **A representation where an effect is nothing but a group will have to grow.**
22. **`mut(E)`'s `E` — group or its own sort — is GENUINELY UNDERSPECIFIED**, confirmed as a real doc gap rather than a reading we missed. `mut(E)` treats `E` as an effect while the bound forms substitute *groups* into it (`where Ea: !mut(B::capture_group)`). One hard boundary is stated: *"There is no closure over effect variables (`mut(E...)` is not a form) — the solver's domain stays fixed."* **Keep the two separable rather than betting either way.** Upstream is carrying our framing of this (the tick-vs-`: Effects` asymmetry, and that the attested `comptime` kinds are only `type` and `group`) into their open-questions list with attribution.
23. **Positive effects fold; SUBTRACTIVE ones explicitly do NOT.** `mut(g, h)` ≡ `mut(g + h)` — `+` is set union, `()` its identity. But the subtractive comma form is a **conjunction of independent checks**: *"each conjunct is checked independently against `E`'s solution; unions of positive effects don't distribute across the subtraction."* Plus Pin C (re-check every negative bound whenever the solver widens `E`) and **relation-aware satisfaction** (tested against declared `maybealias`/`in` facts, not syntactically). **A folded-and-forgotten negative bound conforms to neither — store the list.**
24. **We are stricter than the language on loops, and it's a defect** — see defect 14. *"`set` re-binds and revives, so a path that moves and then re-binds agrees with one that never moved,"* and move-state is a **conserved** fact that "must return to itself across the back edge." The revival-`set` carve-out exists precisely to make the `for` desugar legal over a linear element type.
25. **Our clone bound needs an effect slot.** The bound is plain `func clone(&T) T` with `mut(E)` — a clone can charge, since a claim-typed `T`'s clone is an inc, and the existential is the carrier. At a claim-typed `T` the witness is the **compiler-synthesized** claim clone, a real function performing the inc. `Copy ⟹ Clone` with `E = ()` at a `Copy` type. A generic that **iterates while cloning** must declare `E: not(mut(l.items))` itself or be rejected at its own declaration. **Do not lean on design-1:430's** *"`&T` accepts struct kinds only"* — structs-only `&T` bounds are repealed, a `&T` bound does not exclude classes, and a mismatch reports as the ordinary unsatisfied bound at the use site.
26. **Class-`for` verdict flips are now concrete** (supersedes ruling 4's forward reference). The collection carries a monotone **`epoch`** op-counter — *distinct from* the multi entry's `gen`; epoch counts structural ops, gen counts deaths, **do not conflate**. Verdict 3 (`for u in xs { xs.push(…) }`) now **compiles** with a release-free `debug_assert` on the epoch bump. Verdict 4 no longer poisons. **Verdict 5 (`graveyard.append(u)`) is ACCEPT — and 0b.12 ("no claim from a borrow") is NOT weakened; its premise simply doesn't arise because `u` is already a claim. Recording it as "0b.12 relaxed" will get the next case wrong.** Deliberate tier split: over a *struct* collection the same loop yields a borrow and appending is a C1 error wanting `.clone()`.
27. **Two soundness-grade questions, both since CLOSED** (design-1:2973-2974): what budgets an *erased* clone's effect — `+ Clone` carries `duplication` — and what a derive generates over a *generic* field whose kind decides whether it's a stopping point, closed via `@`. **The erasure path is not on hold.** Also settled: `!` survives in `unsafe impl !Sync for MyType`, as a negative impl rather than logical negation, outside the `not(...)` rule (design-1:53).

**►► RULING 13 IN FULL — the drop/linear inversion, because it propagates ◄◄**

**Drop is auto-generated**, per kind:

| kind | drop |
|---|---|
| **struct** | auto-generated; **opt out with `#explicitly_destroyed`** |
| **class** | auto-generated |
| **interface** | auto-generated (abstract; dispatches to the impl'ing class's) |
| **trait** | **not** generated — traits are assumed linear; declare an abstract drop or extend `Drop` for droppability |

**And the vocabulary was inverted in the docs — the correction matches standard substructural usage:**
- **linear** = must be consumed **explicitly, exactly once**. **No drop exists.**
- **affine** = **has** a drop; scope end handles it; may be discarded.

So design-1:97's *"linearity determined by the presence of a `drop` function"* is **backwards** — **drop *absence* is what creates the obligation**, and design-1:1620 is being rewritten. **"linear *(was "linear-strict" — see ruling 13's vocabulary inversion)*" retires as redundant.** This had been caught by their ports twice, routed, and never fixed, and the phrase *"linear-via-drop-presence"* propagated into every doc summary in their corpus — so **assume any vocabulary we took from them before 2026-07-25 is inverted.** Concretely: the `File { fd: int }` case is **affine**; the `Future<T>` (by-move consumers, no drop) case is **linear**. Those are the two halves of the `#derive(Copy)` question and they now have the right names.

**Our three derives map as follows:**
- `#!DeriveStructDrop` (**65 sites**) → `#explicitly_destroyed`. One-for-one, and safe to run: bare `#name` attributes are admitted (design-1:2716) and both spellings mean *suppress*, so the rewrite preserves meaning. Our corpus only ever *suppressing* is exactly right: invocation is now the default and there is nothing to spell.
- `#!DeriveInterfaceDrop` → **not an attribute at all — it's the KIND choice.** `interface` carries a drop, `trait` doesn't. **Not a pure rename**: the two kinds also differ on erasure (interface is class-tier RC-erased, `open trait` is struct-tier `Box<dyn>`), so check what our interfaces actually *are* first (see the tier census below).
- `#!DeriveAnonymousSubstruct` → no Valen analogue. Two sites. Ours to resolve.

**TRAPS AND CORRECTIONS — each of these would have cost us:**

- **`T: Drop` does not detect every type with a drop.** design-1:1668 — Valen has **two spellings of `drop`**, and only the `impl Drop` form satisfies the bound; the free-function form (`func drop(self: File)`) does not, yet **both give the type a drop**. So a `#derive(Copy)` gate written as *"does this satisfy `T: Drop`?"* — the natural shape, and the one our bound machinery already points at — **passes `File { fd: int }` straight through**, on design-1's own example. Check for a `drop` **function**, never for the bound. Rust has no analogue, so Rust instinct actively misleads here.
  - **Both spellings make the type *affine*, not linear** — design-1:2082, *"Both run the destructor at scope end and so make the type **affine**."* The vocabulary is easy to invert here because the surrounding intuition ("a drop means you must be careful") pulls the wrong way; **drop *absence* is what creates a linear obligation.**
- **The interop doc's "interface" is NOT design-2's `interface`.** `docs/convos/rust_interop/vale-rust-interop-architecture.md` — **now in OUR tree**, moved here by `699241ffb`, and cited by design-1:2554 as the FFI authority — uses Vale-era vocabulary where "interface" means design-1's `trait` / `open trait`. **Since it's ours now, fixing the vocabulary is our job, not upstream's.** So interop-doc "interface" **projects** to Rust (sealed ones as enum + sealed trait), while design-2 `interface` (class tier, RC-erased) gets **no projection at all** — Rust holds an opaque handle and calls through it (design-2:833, *"This is the intended answer, not an unfilled gap"*). Live on the `tests_exporting_interface` front: a test asserting a real `dyn` for an interface is asserting the wrong thing.
- **`Copy` × linear is still open — fail closed.** design-1:1801's *"linear is orthogonal to Copy"* stands per the architect, but `File { fd: int }` (all fields `Copy`; the obligation comes from the `drop` *function*, which can reach an fd or a syscall) shows an all-fields-`Copy` condition does not exclude a double close. Narrow question routed: **does `#derive(Copy)` reject a type with a `drop`?** The by-move-consumer quadrant (`Future<T>`, no `drop`) may get a different answer — its double-await is harmless exactly when every field is `Copy`. **Reject in both cases meanwhile, marked provisional in-source so it can't harden into a claim.**
- **`.` performs receiver adjustment.** Our dispatch model's claim that dot is *pure* sugar is refuted by the corpus: `keys.append(*k)` (owned receiver, `&self mut` method) requires an autoref, and `set self.hp -= 10` requires a deref. The adjustment is on the **receiver** `keys`; the argument's `*` is a separate rule (arguments do not adjust). What survives is the **namespace** half — dot doesn't change *which* functions are findable, so the overload mission's no-Self-specialness rule stands. The repair is one clause: **`.` is sugar for the free-function form after a receiver adjustment.**

**Design-2 provisional map** — for the audit; treat as ratified unless listed here:
- **Wholly provisional:** design-2:568-641 (the entire `softmut` / single-classes / constraint-refs chapter; `:570` reads *"STATUS: PROVISIONAL — not yet ratified"*), and design-2:971 (*"true hybrid architecture"*, EXPLORATORY). Single classes carry an extra tier of caution at `:605`.
- **Provisional items sitting INSIDE ratified chapters** — the false-positive generators: `Class<T>` (`:518`/`:529`, *"proposed 2026-07-15"*, never ruled, sitting mid-way through the ratified classiness-ladder table); **class-kind `own`** (`:61` — semantics ruled but *"placeholder spelling, rename owed"*, so don't hard-code the keyword); sealed-class attestation spelling (`:541`); atomic-RC analog (`:949`, TBD whether it exists at all).
- **Read design-2:957 before filing any inc/dec finding.** The doc contradicts itself *on purpose*: the inc/dec effect refinement was adopted 2026-07-13 and never folded into the base text, so *"three states in one file; the coarse one is what a reader hits first."*
- **Open Questions sections outrank body text** in design-2 where they disagree — they are the later thought, and `:957` says so outright.
- **Stale under ruling 4:** design-2:1168, class-container iteration yielding a lending borrow. The trace verdicts have since been re-ruled in place (design-2:777) — verdict 3 compiles, verdict 4 no longer poisons, verdict 5 is ACCEPT — so read them as current, and read `:777`'s own warning with them: **verdict 5 is not a 0b.12 relaxation**, its premise simply does not arise. Upstream records that the opposite reading was made once already and cost a corpus scour.

**A dating discipline.** **C1 is young against a much older corpus and nothing has swept for it**, so a stale spelling reads exactly like a divergence. Known pre-C1 artifacts: design-1:1332's `t.clone()` spelling (dated 2026-07-10), design-1:171's auto-deref paragraph (predates the `*` ruling), and `ValenRL-Rung4/src/slot_map.valen:79` passing a bare non-`Copy` place to `replace()`, which is a C1 error as written. **Treat corpus examples as design evidence, never conformance fixtures**, and check a passage's date before taking its spelling as authoritative.

**Settled, so do not re-raise:** what `.clone()` resolves to (ruling 8); whether `&k` forms `&&T` (ruling 9, expression-scoped); bare class `T` in return position (ruling 16); design-1:1035's rationale (ruling 10); design-1:164's diagnostic (ruling 17); `#derive(Copy)` × drop-bearing (ruling 14). **Deref-first method resolution is proposed and WITHDRAWN** — we found it flips the claim default versus Rust, and it solved the wrong problem: Rust already resolves the good case, and the footgun is purely that the auto-derived reference clone is available as a *fallback*. The fix is excluding that candidate, not reordering the search — which makes decision 3's "bound-only machinery" a **consequence** rather than a stipulation.

**`Copy` × linear is CLOSED** (design-1:2224, *"**`Copy` and linear are mutually exclusive.**
`#derive(Copy)` requires **every field to be `Copy`**, and is rejected outright on a type with a
user-written `drop` or with `#explicitly_destroyed`"*). `Future<T>` is `#explicitly_destroyed` with
no drop, so it is rejected — **which is exactly the fail-closed behavior already shipped, so no code
changes**; what goes is the in-source provisional hedge. The doc also carries ruling 15's requested
caveat verbatim: the `#explicitly_destroyed` half is *"not unsound, merely incoherent… and may
relax."* **The field condition is the transitive one** — a `#derive(Copy) struct Handle { f: File }`
is rejected because `File` is not `Copy`, so a wrapper cannot launder a resource-owning type into a
copyable one.

**Still open upstream:** three questions, enumerated under "Waiting on upstream" in the open list —
that is the single home for them, so add there rather than here.

**Answered, and each closes something we'd been carrying:**
- **Our namespace/dispatch model is NOT a divergence — it's unsettled upstream.** design-1 says *nothing* about how a candidate set is assembled at a call site; the only nearby item is design-1:2563 listing **module and import syntax as an open question**, while visibility (`pub(crate)`/`pub(super)`) presupposes a module system that hasn't been designed. **Flag for later: when Valen decides modules, that is the moment to compare** — a file-based namespace rule and a module system are hard to retrofit against each other. Supporting our sub-assumption that `&Ship` and `Ship` are distinct: design-1:989 gives reference and box types **their own impls** (`impl SomeTrait for &dyn OtherTrait<...>`), so they aren't aliases of the pointee.
- **EFFECTS NEED A CHECKING PASS, NOT AN INFERENCE PASS.** design-1:1265's *"unifies **declared** effects with **derived** effects"* is a **check of the body against the signature** — you cannot unify a declaration that doesn't exist — and design-1:850's *"a method with no effect clause has no external effects"* means absence **asserts** purity. *"Valen infers purity"* is about **spelling** (you never write `pure`), not about deriving an omitted clause. Consequences: a named function with no clause whose body mutates is a **compile error**; **no call-graph fixpoint is needed**, since a recursive call checks against the callee's *declared* clause. **Closures are the exception and genuinely infer** (design-1:1604), and **a recursive closure would need a fixpoint — the doc does not address it. Genuine gap; record rather than assume.** So the build is: a checking pass for named functions + a computation for closures + the `E: Effects` solver with Pin C's re-check discipline. Materially smaller than first sized.
- **Override parameter shapes: unspecified upstream, but lean EXACT MATCH.** design-1:884 governs *effects* only (impls may narrow, not widen); there is no parallel rule for reference modes or returns. The structural reason to require exact match: **Valen has no variance anywhere and refused it deliberately** — `concerns/soundness/02:21` records that *"'variance'/'invariant'/'covariant' appear nowhere"*, and the disposition was *"banning cross-instantiation subtyping outright."* A narrowing override would be the language's **first** instance of variance, introduced at a corner rather than as a decision.

**Also dissolved, and worth not re-raising:** whether `c.clone()` on a class claim deep-copies the payload. It never arises — **a bare mention already copies a claim** (rubric:112, *"class refs auto-copy (inc/share)"*; design-2:880 for captures), so a rung-1 user writes `d = c` and never reaches for `.clone()`. design-1:1332's contrary spelling is the pre-C1 residue described above; its *substantive* point survives (a claim-bearing **slot** can be copied from, a payload **borrow** cannot — the `graveyard.append(u)` rejection).

## Defect inventory
Grouped by what you would do with them. **Everything here is traced from source and cited**; where something could not be settled by reading, that is stated rather than guessed.

### Broken today

1. **The export/extern gate PANICS on an ordinary borrow parameter.** `exported func moo(firefly &Firefly)` reaches an `unimplemented!()` ref-wrap arm via `ensure_deep_exports` (`typing/compiler.rs`) — verified by running the suite, with a stack trace. Not an error: a panic. **Filling the four ref-wrap arms with `false` is NOT sufficient** — a `BorrowRef(Struct(Firefly))` then misses `exported_kind_to_export`, which is **keyed by BARE kinds**, so it reports a spurious error while the humanizer (which peels wraps) prints `Firefly`. **The gate needs `peel_all_references` before both the `is_primitive` call and the map lookup.** `is_descendant_kind`, in the same file, has the identical four arms.
2. **A latent assert-failure at the if-join, now reachable.** The `If` arm of `evaluate_expression` diffs `nenv.snapshot(...)` against *itself* — a botched transcription of a Scala "if block env" that no longer exists. So the restackified result is the entire current restackified set, the arm re-marks each, and `mark_local_restackified` (`env/function_environment_t.rs`) opens with `assert!(!contains(...))`. **Any `if` compiled while a local is restackified should assert-fail.**
3. **Dropping a `Str` is unimplemented.** `drop`'s `Str` arm in `destructor_compiler.rs` is a bare `unimplemented!()` under a comment reading "decrement a reference count" — the one kind that does not resolve a drop function the way the citizen arms do.
4. **Share upcasts don't work at all.** `convert_via_upcast` (`typing/convert_helper.rs`) calls `ISubKindTT::try_from`, which rejects wrapped kinds. `convert()`'s borrow path passes peeled citizens (fine); the no-borrow path passes the full type, so `@Dog → @Animal` arrives as `ShareRef(...)`, `try_from` fails, and it reports **`CouldntConvertT` instead of upcasting**. Consequence: **the 18 class-tier interfaces are probably not a working baseline** — check before treating them as one.
5. **`inner_find_reachable_allocations` (`testvm/heap.rs`) is missing THREE arms, not one.** `KindV` has 8 variants and it handles 5, so **`Str`, `Opaque`, AND `ArrayInstance`** hit a bare `panic!()`. Latent (one call path, one test, empty members) — but the `ArrayInstance` gap means the leak check would panic on any array-rooted heap before `str` even enters. Fix as one arm-set.
6. **Loops skip their move check when the body is `Never`** — a body that always returns or breaks can unstackify an outer local with no diagnostic and no propagation. The same `While` arm also contains a **verbatim duplicate** of its own preceding block, and both report the wrong local (`body_unstackified_` where `body_restackified_` was meant).
7. **The plain `Block` arm lacks the `continues` guard the `if` arm has** — it propagates child unstackifications to the parent unconditionally, so a bare block ending in `return`/`break` still pushes its marks upward.
8. **`&self` is a parse-level stub.** The parser recognizes it (a two-token lookahead in `pattern_parser.rs` setting `self_borrow`), but postparsing builds a **rule-free `ImplicitRune`** — nothing ties it to the enclosing citizen and **the borrow-ness is discarded**. Related: bare `self` inside a citizen body **panics** in `function_scout.rs` with `POSTPARSER_SCOUT_FUNCTION_PARAM_TYPE_REQUIRED_NOT_YET_IMPLEMENTED`, contradicting a stale parser comment claiming it defaults to the containing struct. **And there is no `Self` type to desugar to** — no `"Self"` string anywhere in `src`; `SelfRuneS` is dead, `SelfFullTypeRuneS` is macro-only, and `SelfNameS` is a *variable* name that never meets a user-written `CodeVarName("self")`. **Sequencing: implementing `&self` requires inventing `Self` first.**
9. **`SharednessImplingMismatch` doesn't exist and never did.** `impl_compiler.rs` validates exactly one citizen attribute across an impl — weakability — and zero `sharedness`. Confirmed absent from the Scala era too (the compiled class files survive; only `WeakableImplingMismatch` exists). So **`look_for_override`'s blind shape-copy rests on a convention, not an invariant**, and the instantiator makes the same unbacked assumption, deriving impl sharedness purely from the super interface. A mismatched impl compiles cleanly and then panics downstream with no useful diagnostic. **The fix shape is obvious: a `SharednessImplingMismatch` three lines from its precedent.**
10. **`stdlib/src/ifunction/ifunction1.vale`** reads `interface IFunction1<M Mutability, P1 Ref, R Ref> M {` — the trailing `M` sits in the sharedness slot but is a **generic Mutability parameter** from pre-migration syntax; the lexer accepts only the literal `share` there, so **the file is unbuildable as written**. It is also the **only trace anywhere that sharedness was once template-parametric**, which would contradict the parse-time-known assumption in `struct_compiler.rs`. Same stale shape in `stdlib/src/str.vale`. (This is also the `IFunction1` with **no implementors** and an arity mismatch against its local twin under `tests/ifunction/`.)

### More defects (numbering continues the inventory)

11. **The `BorrowRef` peel writes to the wrong rune.** In `solve_rule` (`typing/infer/compiler_solver.rs`), the "result known, inner unknown" arm concludes into `result_rune` where it means `inner_rune`. `result_rune` already has a conclusion — that is the match guard — and the new value differs, so `commit_step` returns `SolverConflict`, which the arm handles with an unimplemented-wrapping `panic!`. Reachable: `get_puzzles` gives `BorrowRef` both directions. **This is the peel direction phase 0 depends on**, which makes it load-bearing rather than incidental.
12. **No move inside a loop body ever propagates outward.** The `While` arm never calls `mark_local_unstackified` at all, unlike the `If` and `Block` arms.
13. **The loop move-check is skipped entirely when the body is `Never`** — the whole check is wrapped in `match body.result { Never => {}, _ => checks }`, so a body that unconditionally breaks or returns bypasses it.
14. **Our loop rule rejects the desugar of `for`.** `CantUnstackifyOutsideLocalFromInsideWhile` forbids moves of outer locals outright; upstream confirmed move-and-restore is intended to compile, and the `for` desugar *is* one — `while Some[(it2, x)] = it^.next() { body; set it = it2 }`. **A defect, not a strictness preference.**
15. **Most AST nodes carry no source range** — 6 of 49. A hard constraint on any checker's diagnostics, cheapest to fix before the checker exists.

### Dead code, verified

- **The `initially_known_runes` local in `solve_rune_types`** (`typing/rune_typing/rune_type_solver.rs` — note the bare name also hits an unrelated live field in `solver/solver.rs`) — **redundant and safe to remove**, with a structural proof rather than an argument from absence: `Call`'s puzzle names the template rune and `get_next_solvable` only returns rules whose puzzle is fully concluded, so its `.expect("Call: template rune unsolved")` is **unreachable by construction**, prepass or not. `Lookup`'s empty puzzle is vacuously always solvable, so the loop cannot terminate while any `Lookup` is unsolved. It is the orphaned `else` branch of the `if predicting` block removed in `dea61d925`. **One caveat: removal changes step ordering, which can change *which rune* a `SolverConflict` names** in already-erroring programs (same error/no-error outcome), and some solver tests assert on humanized text. Cleanup: `unpreprocessed_initially_known_runes` loses its prefix.
- **`PrimitiveRuneTypeSolverLookupResult` is never constructed** — only its declaration, its `IRuneTypeSolverLookupResult::Primitive` variant, and three read-only match arms, one of which is inside `lookup_rune_type`. All three `IRuneTypeSolverEnv::lookup` impls return only `Citizen` or `Templata`.
- **The `lookup_rune_type` Template→Kind coercion** is **dead in practice** — traced by construction-site enumeration, not execution. **Confirm before deleting: replace the arm body with a `panic!` and run the suite.** Two traps on the way out: deleting it orphans `check_generic_call`, and **`NotEnoughArgumentsForGenericCall` / `FoundTemplataDidntMatchExpectedType` are matched BY NAME in `higher_typing_error_humanizer.rs`** (live, reached from `compiler_error_humanizer.rs`) — deleting those two variants is a compile break. A crate-wide `#![allow(dead_code)]` means nothing warns.
- **Warnings are 7, not 8** (`grep -c "^warning"` counts rustc's own summary line). **Three are genuinely removable**: a duplicate `ITemplataT::Kind(_)` arm in `environment.rs` with a byte-identical body to the one just above it, and the two dead catch-alls in `get_runes` and `get_puzzles` — the latter two have a comment directly above already saying *"this whole sanity_checked block is a debug-mode hand-duplicate of `rune_usages()`; it could be deleted outright."* **Four are deliberate signposts** that self-clear as their work lands. **One needs judgment, not a delete**: `evaluate_addressible_lookup_for_mutate`'s shadowed `IVariableT::Capture(_)` arm is unreachable but holds a `panic!("implement: … ReferenceClosureVariableT")` plus ~20 lines of canonical Scala — un-ported work, not cruft.
- **Two of `ITypePR`'s five variants are unreachable** — `BoolType` and `CitizenTemplateType` have **no construction sites anywhere**, appearing only as arms in `rule_scout.rs`'s type translation, where `CitizenTemplateType`'s is a `panic!`. (`IntType` / `CoordListType` / `RegionType` are live. `KindType` was already removed with the Components rule, which had been minting a rune constrained by nothing.)
- **Two dead files**: `Backend/vstl/` (zero references; its 3 carets are already postfix in an older dialect) and `builtins/resources/functor1.vale` (not in `builtins.rs::ENTRIES`, referenced nowhere).

### Confirmed safe — hazards that turn out not to exist

- **Our move tracker already implements the CONSERVED discipline exactly as spec'd.** In the `If` arm, branches compile in by-value child envs, effects are recovered by diffing against the parent snapshot, and **disagreement is an error** (*"Must move same variables from inside branches!"*) — not union, not intersection, not last-writer-wins. Sets are `IndexSet`, so order-insensitive. `return`/`break` are handled correctly by **discarding** the diverging branch's effects. Two tests pin it (`if_branches_must_move_same_variables`, `..._different_order_compiles`) and both now reach the join, so a regression there surfaces immediately.
- **Loops have no fixpoint and no back edge** — moves out of a loop are forbidden outright (`CantUnstackifyOutsideLocalFromInsideWhile`), and the body's effects are never merged. **A monotone least-fixpoint analysis has no structure to attach to**; the `While` arm compiles the body exactly once.
- **The `self` silent-corruption hazard does NOT exist.** **Zero** bare `self` receivers in the corpus; exactly one `&self`, in a parser unit test that never reaches typing. All 92 `self`/`this` receivers spell ownership explicitly (69 `self &T`, 19 `self T`, 23 `this …`, 4 weak/`&&`). Better than neutral: **all 19 owned receivers are genuine consumers** (15 `drop`s, 4 `Subprocess` joins, `Opt.or`, `HashSet.add`), so the corpus is *already* consistent with the incoming rule. Adopting "bare `self` = owned" costs **nothing** in migration. There is even a test named `this_isnt_special_if_was_explicit_param` — we're already aligned with design-1:397's "the receiver is not special."
- **The `is_primitive` divergence is NOT a bug to reconcile — it's two predicates sharing one name.** A user **cannot** export `str`: the grammar accepts `export str as X;` (there's even a passing test for `export int`), but the export path panics on every builtin and the backend `exit(1)`s. So **`typing/compiler.rs`'s `Str => true` is load-bearing** — it means *"needs no user export declaration"*, and flipping it would make an exported/extern `str` param permanently unsatisfiable, immediately breaking `stdlib/src/path/path.vale` and `command/command.vale`. **`typing/types/types.rs`'s `Str => false` is the ABI/representation sense** and is equally correct — and the backend agrees, classifying `Str` with the handle types at four independent sites. The header generator emits `typedef struct vtest_str { uint64_t _reserved; }` for **every package regardless of exports**, plus auto-registered `str_len`/`str_char_at`/`str_alias`/`str_dealias`/`str_ref_eq`; a golden test pins this against a program containing no `str` at all. **The fix is renaming, not moving the `Str` row.** (Six of sixteen variants disagree, incidentally — `OverloadSet` is a second, inverted disagreement, currently harmless.)

### Easy things to get wrong here

- **`MaybeCoercing` and `predicting` are already gone** — `predicting` in `dea61d925`, and `MaybeCoercing*` was never an `IRulexSR` variant at all (the enum has 12; every surviving mention is a comment). **One live nuisance: `solve_rule`'s catch-all `unreachable!` message names rule kinds that no longer exist.**
- **`replace_value_type_in_ref` has ONE surviving ZHERE, not five** — in `as_subtype_macro.rs`, covering two call sites. The two `edge_compiler.rs` sites are done and `UpcastTE::new` was re-marked `VCOORD`. Sibling worth knowing: `InterfaceToInterfaceUpcastTE::new` carries the identical `VCOORD` + `unimplemented!()` with **zero callers**.
- **The validity-table question is latent everywhere**, because share barely flows through typing — four live `KindT::ShareRef(_) => unimplemented!()` arms across `typing/compiler.rs` and `templata_compiler.rs`. Only `look_for_override`'s could genuinely bite, since there the citizen changes *identity* (interface → struct) rather than just substitution — that is defect 9.
- **The `rune_type_solver.rs` "fill this Call arm" ZHERE is stale by `git blame`** — marker and implementation landed in the **same commit** (`07a792c9a2`). Safe to delete. Still open a few lines away: a `VCOORD: arcana instead`.
- **The `T: Drop` trap** is stated in full under ruling 13's traps — check for a `drop` **function**, never the bound, and further distinguish declared from synthesized.

### The interface tier census

**18 of ~133 interfaces are `share`-declared (~13.5%); ~86.5% are struct-tier.** The minority is *structured*, not scattered: **11 are the `externs/interfaceimm*` family** (FFI boundary tests, all `sealed exported`), and 7 are immutable-linked-list / ancestor-stamping fixtures. Only 2 live in Rust fixtures, one of which is a parser unit test with no implementors. Implementor counts run 1-3, so retiering is mechanically cheap. **Surface spelling is `share`, placed after the name and generics** — the `SharednessP` slot in the lexer's citizen header. Two disambiguations: **`imm` is the retired Scala-era spelling for that same slot** (proven by 11 one-to-one pairs against the stale build tree), while **`imm` as a *region* modifier is a different, still-live construct** — don't conflate. Asymmetry worth noting: `share` is far more common on **structs** (49) than interfaces (16).

## The compiler as it actually is

### Four facts that are easy to get backwards

1. **An unannotated `&T` synthesizes no region rune.** It is `RegionP::Unspecified` → `RegionSR::Unspecified`. Exactly **two** producers of `RegionSR::Rune` exist in the tree: an explicit `'r` annotation, and the synthesized closure parameter. Do not conclude from the rune-type solver's `BorrowRef` region arm that bare borrows feed it — they don't.
2. **The call site runs phases 1–2 twice**, once over explicit template args in `overload_resolver.rs` and once over the callee's full rule set in `function_compiler_solving_layer.rs`. That is the call site doing two phases in two passes. **It is not evidence against the phase model** — CALL-SITE PHASES is the ratified design and the authority; this is the implementation being mid-migration toward it.
3. **`ExpressionTE`'s `result` field is not uniformly a `KindT`.** Eight nodes store a narrowed arena ref — `LocalLookupTE` and the four member/array lookups plus `LetAndLendTE` hold `&'t BorrowRefT`, `BorrowToWeakTE` holds `&'t WeakRefT`, `ConstantStrTE` holds `&'t ShareRefT`. **`ExpressionTE::result()` is the only correct accessor.**
4. **`instantiating/` and `simplifying/` are stale, not merely gated — they would not compile.** They match on `ReferenceExpressionTE::While/Return/Break`, an enum with zero hits under `typing/`. Same for `final_ast`, `von`, `testvm`, `pass_manager/`. **Archaeology, not reference implementations.**

### Verified mechanics

**Regions.**
- **`ITemplataT` has NO `Region` variant** (15 variants; none). But **`ITemplataType` DOES have `RegionTemplataType`**. So the rune-*type* domain has regions and the rune-*value* domain does not. A live comment in `rust_interop/tyctxt_oracle.rs` confirms it.
- **A region *generic parameter* does get a value**: `create_placeholder` (`templata_compiler.rs`) routes every non-`Kind` rune type to `create_non_kind_non_region_placeholder_inner`, yielding `ITemplataT::Placeholder(PlaceholderTemplataT { id, tyype: RegionTemplataType })`. **So the architect's model — "a region is a `PlaceholderTemplataT` referring to a generic parameter" — is real and already implemented.** What does *not* exist is a representation for a *concrete* group expression.
- **Writing an explicit `&'r Moo` is a latent hole.** The rune-type solver types the region rune and it enters `all_runes`, but `solve_rule`'s `BorrowRef` arm **never reads `r.region`**, so the completeness check fails with `SolveIncomplete`. An *anonymous* region doesn't survive postparse — `translate_templex` panics on it. No typing test uses region syntax.
- **Nothing branches on a region.** `grep "RegionT::Default =>|RegionT::Iso =>"` returns **zero**. `RegionT::Iso` is constructed zero times in live code. Only two sites read the field. **The real consumer is the derived `PartialEq`/`Hash`** on `BorrowRefT` → `KindT` → type equality and arena interning — so region equality is free *because* everything is `Default`. A real algebra makes it semantic (`a+b == b+a`), hitting `params_match`'s exact comparison and interning. `convert_helper.rs` carries a comment already saying "unreachable while every borrow carries `RegionT::Default`".
- **`RegionT::Default` / `Iso` are migration scaffolding** — we are mid onion-typing migration and they were always coming out. Do not read the ~52 `RegionT::Default` literals as accumulated debt.
- **Threading cost, if regions become an algebra** *(counts measured once and not since — re-measure before quoting)*: ~52 `RegionT::Default` literals; **9 `BorrowRefT` construction sites** across `ast/expressions.rs`, `function_compiler.rs`, `call_compiler.rs` and `compiler_solver.rs`, all stamping `Default`; ~45 signatures threading `context_region: RegionT` by value; 8 expression nodes with `region` fields; `FunctionEnvironment*.default_region` at six sites. `InferEnv` is `#[derive(Copy)]` with a by-value region. **`KindT` is a deliberately 16-byte `Copy+Eq+Hash` enum (`@WVSBIZ`)**, so a recursive `RegionT` must become arena-interned with a `*ValT` companion. And **`RegionT` is already part of denizen identity** — a field of `ExportNameT`, `RawArrayNameT`, `ExternNameT`, i.e. inside `IdT`.
- **`substitute_templatas_in_kind` recurses through all four ref wraps** (`templata_compiler.rs`), rebuilding each around its substituted inner and preserving a `BorrowRef`'s region — so a generic substitution survives the ref layers.

**Bounds — a bound is a DENIZEN LOOKUP, never a predicate.**
- The entire bound vocabulary is `PrototypeT` and `IdT` (`InstantiationBoundArgumentsT`, in `hinputs_t.rs`). Discharge is calling the **overload resolver**. The only failure vocabulary is "couldn't find a function/impl" or "return type mismatched" (four `IConclusionResolveError` variants). `IRulexSR`'s twelve variants contain **no general assertion rule**.
- **Bounds are checked once per CALL-SITE RESOLVE in typing — not at instantiation.** `add_instantiation_bounds` (`compiler_outputs.rs`) is write-once with an equality assert, explicitly *not* a merge or re-check. The instantiator does **zero** verification, and doesn't compile.
- **The real per-call-site re-check lives one system over**: `assemble_call_site_rules` (`templata_compiler.rs`) re-runs **every rule except `DefinitionFunc`** at each call site, filtered by `include_rule_in_call_site_solve`.
- `assemble_rune_to_*_bound` lives in **`templata_compiler.rs`**, not `impl_compiler.rs`. Assembly is **not uniform** — two unrelated mechanisms across five denizen paths plus six hand-written empty literals. **No single seam to extend.**
- **Impl bounds are vestigial** — every producer commented out, `runes_and_impls` hardcoded `vec![]`.
- **The substituter is narrow**: `substitute_templatas_in_templata` handles 4 of `ITemplataT`'s 15 variants; two of `IPlaceholderSubstituter`'s five methods are `panic!("Unimplemented: Slab 15")`.
- **Argument types do not enter the call-site value solve.** `assemble_initial_sends_from_args`'s result **is never consumed anywhere**. Callee generic params come only from explicit template args plus `@DRSINI` defaults; arg/param compatibility is checked afterward by `params_match` → `is_type_convertible`. **So "the solver already deduces group params from arguments" is false — that machinery has to be built.**

**Body compilation and the expression walk.**
- **Control flow DOES survive into the finished `ExpressionTE` tree** — the load-bearing fact for a post-hoc checker. Structural nesting plus `KindT::Never { from_break }` is enough to derive successors: `IfTE` is its own join point and its `result` says whether the join is reachable; `WhileTE`'s back edge is total and implicit (Vale's `while` *is* `loop`; the condition desugars into the body with a `Break`); break targets are the nearest enclosing `WhileTE`, sound because break can't cross a function boundary; return always targets function exit. **There is no `continue` in the language.**
- **Only 6 of 49 nodes carry a `RangeS`** — the five lookups and `DerefTE`. `LetNormal`, `Unlet`, `If`, `While`, `Break`, `Return`, `FunctionCall`, `Destroy`, `Discard` carry **none**. **A post-hoc checker cannot point at the offending line for most node kinds.** Either add `range` fields or thread a side-table during compilation (which makes it not post-hoc).
- **`typing/test/traverse.rs` is a complete live traversal skeleton** — all 49 variants, with collect macros. It is the template for a checker's walk and arguably wants promoting out of `test/`.
- **`typing/reachability.rs` is a live module whose every method is `panic!("Unimplemented: Slab 15")`.**
- **`DeferTE` has undefined semantics** — live, emitted from five sites via `make_temporary_local_defer`, and encodes neither *when* the deferred expression runs nor what happens if the inner one diverges. Every consumer that would have defined it is dead code. A checker must pick a semantics and state it.
- **The if-join disagreement check is CONDITIONAL** on `then_continues == else_continues`. If exactly one branch is `Never`, the compiler **unilaterally adopts the survivor's moves with no comparison**. The failure is a `RangedInternalErrorT`, not a designed diagnostic.
- **No linear-obligation tracking exists**, and the reason is structural: `drop_since` auto-generates drops for everything live at block end, so nothing is ever *required* to be consumed. Under ruling 13 (drop *absence* creates the obligation) **that auto-drop machinery is what has to become conditional.** Upstream's landed shape: *"conditional on the fields, resolved per instantiation, carried as a bound on what is generated rather than deferred to monomorphization"* — so **a generic container's drop existing at all is instantiation-dependent**; `Vec<int>` and `Vec<Future<int>>` differ in whether a scope-end drop exists.

### Region and effect decisions
The region and effect rulings (`ITemplataT`'s region payload is the group algebra; a region is never `mut`/`imm`, condemning `RegionT::Iso` as a fossil; borrow creation computes rather than checks; a borrow-of-claim must carry the claim's `rc.T` mention; effect derivation by substitution; effect representation is unsettled, live candidate a per-group permission map; `not(mut(…))` applies to the whole call; the checker iterates the finished tree) now live in `docs/plans/path-to-borrowing.md`.

### Outstanding ZHEREs — five, plus one ZLOOK

> **Do not go looking for a ZHERE on the `implements` mint** — that work landed on both sides and the
> marker is gone. Same for `solve_rule`'s `KindList` arm and the "Bad template call" one; both are
> now `VCOORD`s recording what is left.

Five ZHEREs remain, all in `typing/` — grep for them rather than trusting any location written here:
- `expression/expression_compiler.rs` — implement the closure-var mention (a member-lookup into the closure struct); currently unwired.
- `expression/expression_compiler.rs` — the now-dead `ExpressionTE::LocalLookup => Unlet` sub-arm in `Ownershipped`'s `Move` case (`^local` never reaches `Ownershipped` since the scout routes it to `Unlet`).
- `expression/expression_compiler.rs` — implement `weak x` (`LoadAsWeak`) as a `WeakRef` of the source.
- `macros/as_subtype_macro.rs` — two `replace_value_type_in_ref` call sites.
- `rune_typing/rune_type_solver.rs` — **this one is itself stale**; delete the marker.

**One `ZLOOK`** *(new marker kind, weaker than ZHERE — "worth examining," not "do the work here")*:
- `get_drop_function` (`typing/function/destructor_compiler.rs`) — it passes **three empty slices** for
  the explicit template args, so dropping a generic would have to infer `T` backwards from the
  argument. Nothing at the call site says "no type arguments"; you have to count positions. Vale4 hit
  this from the interop side and `opt_with_undroppable_contents` is the pure-Vale case. The
  Harmonious/Sky reframe: the synthesizer is the **one caller that never has to infer**, since it
  stands at the binding holding the resolved type, so it should write `drop::<T>(…)` itself. Open
  condition: whether this runs after types are resolved.

Not a ZHERE but still true: `is_type_convertible`'s reference rows are *wrong*, not merely missing — `&NC→NC` returns `false` where `convert()` succeeds. The primitive `&P→P` and both-borrow cases are fixed; the tripwire keeps the rest loud. Real fix: align with `convert()`'s coercion-table arms, or drive it off a dry-run `convert()`.

### The reference/ownership surface model
New, ratified direction, lifted here out of the transient `tmp/messages/` mailbox thread:
- **`held` is a where-clause FACT on a nameable group**, not a region value: `held T ≡ &T in e_g where maybealias(e_g, rc.__All), held(e_g)`. The `held(g)` fact rep + `held … in g` are DEFERRED. **`RegionT::Held` does not exist** — the "temporary bridge" idea is dropped. Per Reference-model decision 1 below (mention = reference), there is no bare-use ambiguity to mark, so `RegionT::Held` is never introduced. `RegionT` is `{ Iso, Default }` **today**; `Iso` is condemned as a fossil in `docs/plans/path-to-borrowing.md`, so do not build against it either.
- **`&` = borrow; bare = own** (Rust-shaped).
- **`&&` = genuine borrow-of-borrow** — the old weak meaning is retired. Distinct from `weak`; arises from generics / explicit `&&x`; rare (the `clone<T>(&&T) &T` blanket). Nests under generic instantiation; reduced to `&` only at coercion sites, never globally collapsed.
- **`own` = the new `OwnRef` wrap.** At class kind it's the *exclusive* state (sole reference; `own self` is the class destructor's receiver); at struct kind redundant with bare. **Valen has since narrowed this** — see the alignment list below (`own` → `ownref`, immovable-only). The narrowing is a later item; the `OwnRef` wrap itself is unaffected.
- **`weak` stays `weak T`.** Heap-owned is **not** a language keyword — it's a library `Box<T>`, matching Valen; `heap` is removed. This is distinct from `own`: `own` is the language-level *exclusive* state, while `Box<T>` is heap allocation. **`Box<T>` is not fully user-space, though** — design-1:1523 makes `entity.armor[]` a **child group** and design-1:1525 makes `Box<T>` affine-or-linear according to `T`, so the borrow checker needs compiler knowledge of it. Bites at rung 1, not now. Open internal-model question: with heap-ownership library-side, the value-model `HeapOwnRefT` wrap may be **vestigial** — revisit when heap-owned actually surfaces, and note something must still supply that `[]` child group.
- **Erasure / trait model (all long-term / deferred):** `interface I` (class-tier: no `dyn`, bare = strong `ShareRef`, `I in r` = strong into a non-ambient multi, `weak I`) vs `open trait T` (struct-tier, Rust-`dyn`: `&dyn T` / `Box<dyn T>`, bare `T` = a bound). **Sharedness (share vs single on the definition) carries both the class/struct AND the interface/open-trait split** — no separate keyword; `dyn` appears only for open-traits.
- **The colon is one of the two intended divergences** in `name: type`: Vale2 **allows but does not require** it; documented Valen always writes it. Even that is only house style — design-1:2350 permits the colonless form for experimentation. Vale2 is the one behind: the parser has no colon support at all today. (The other divergence is that a mention always yields a reference, copied out only at a bare-value receiver; both are provisional and slated for an experimental flag — see the banner at the top of this file.) **Everything else that differs is a bug.**
- **The `in`-clause region grammar** is **designed, not implemented**. Per Valen's canonical (adopted): the group-param **tick lives only at the declaration** — `<g'>` (untyped) / `<g': T>` (typed) — and **every use is bare**. So `&Ship in g` (not `in g'`); a value's own group is bare; the ambient-multi group is `rc` (not `rc'`); value-paths (`world.ships[]`) and `...` descendant steps carry no tick. `borrow_with_region` still tests the old `&i'MyStruct` apostrophe-*prefix* syntax; re-author when the `in`-clause slice lands.

### Valen alignment — later TODOs
Their design docs **are** the ratification, so the transformation rulebook is the authoritative spelling spec. Sources: `/Volumes/V/LangNotesValen/Valen/valen-approach-convo-30-finalize-syntax.md` (the decisions) and `…-convo-30-plan.md` (§3 is the rulebook). None of the below blocks the onion arc; record and schedule.

**Migration tiering.** `^`→postfix, `own`→`ownref`, `#!DeriveStructDrop`→`#explicitly_destroyed`, and optional colons are **tier 1** — AST-identical, parser-verified, approved, do now. See the migration block at the top of this file for the tiers, the hazards, and the rule that a meaning-changing rewrite is never swept but always driven by compile errors.

- **`*` is a language operator.** `k` = the reference, `*k` = the pointee, `set k = …` re-points, `set *k = …` writes through; field and method access still auto-deref, so no `(*k).field` anywhere. We have **no `*` prefix operator at all** — parser + postparse node + the two-depth `set` distinction. This is **tier 2** for the corpus (a site wanting the pointee will type-error rather than silently work, so the migration is compile-error-driven), but the *feature* is ruled and buildable now. Note it flips design-1:171, design-1:1496, and rubric:161, which all predate it.

- **`^` is POSTFIX and applies to LOCAL NAMES ONLY** — design-1:93, *"`^` (postfix). Move operator on local names (**not on paths**)"*; restated at 2308. We parse `^` as a prefix operator — `Prefix::Move` in `expression_parser.rs`, lowering to `IExpressionPE::Move` — and our corpus contains two sites that are illegal under the spec because they move a *call result*: `move_call_via_caret`'s `"^Muta()"`, and `^innerRemove(` in `tests/hashmap/hashmap.vale`'s `func remove`. Both must bind a local first. See the tier-1 inventory for all four construct sites. Flipping this touches the parser, the expression tests, and every `^x` in the corpus (~264 sites, overwhelmingly `^<local>`).
- **Attribute syntax is `#name` or `#name(args)`, never `#[name]`** — design-1:2279/2283, *"No brackets around the attribute name — `#name(args)`, simpler than Rust's `#[name(args)]`,"* with bare `#name` admitted at design-1:2716. That is the **grammar**; it does not say what each of our three attributes becomes. The mappings are under ruling 13, and the one that matters is `#!DeriveStructDrop` → **`#explicitly_destroyed`** — a bare `#name`, meaning-preserving. **Rewriting it to `#derive(StructDrop)` instead would invert every site**; see the inversion hazard in the migration block. Touches the lexer, the parser, and every fixture.
- **Optional colon in `name: type`.** We have zero colon support in `parsing/`. Add it as accepted-but-not-required. Coupled to the next item: colonless only parses if `&` can't be bitwise-and.
- **`&` is borrow-only, permanently — no bitwise-and.** Bitwise-and gets its own spelling (`bitand` or similar). This is the accepted price of a colonless form being parseable. Record before we need bitwise operators.
- **`own` → `ownref`, narrowed to immovable types** (much later, gated on the whole movability axis). `ownref` is a reference *mode* — peer of strong / `&` / `weak` — meaning "the owning reference to an **immovable** instance." Movable ownership transfers by plain move, no keyword; `ownref` appears essentially only in destructors of immovable types and generic consumers that want to accept them. It does **not** nest inside an *ownership/storage* container (`Box<ownref T>` is redundant like `Box<&T>`; `Box<own T>` was always just `Box<T>`), but it **may** appear in a reified parameter slot (`Fn3<ownref Ship, i64, bool>` — rare). **Our nestable `KindT::OwnRef` onion layer is NOT in conflict with this** — their rule is about ownership containers, not nesting generally. (Caveat: their *plan* doc flattens this to "NEVER nests in a container," which contradicts their own `Fn3` carve-out.)
- **Immovable types are a new axis we don't have** (way later): `!Movable`, pinned, self-referential state machines. Class instances are immovable *while shared* (handles point at them); strong refs to them are movable; `destructure` cashes a class out into a movable `Box<T>`. `ownref` exists only to serve this axis.
- **`interface` vs `open trait`** (later, with traits). Classification: a trait is `interface` iff **a class implements it AND every class impl fits the ambient-multi cover** (member-level mentions, no *external* group params, no *external* stored borrows); else `open trait`. Blanket-impl traits are `open trait`. An `open trait` gets **no RC-strong-in-multi form** — an erased collection is homogeneous in erasure kind. Accepted cost: a trait implemented by both structs and classes must be `open trait`, so its class implementers get boxed.
- **Bare class is position-dependent** — `Rc` in storage, `&Rc` as a parameter (see the class-reference table below). Our scout lowers uniformly; matching them means a bare class *param* lowering to `BorrowRef(ShareRef(…))` and a bare class *field* to `ShareRef(…)`.

**Confirmed as needed:**

- **Enforce no shadowing.** design-1:47 — *"a name is declared once per scope, so `x = y` on an existing `x` is an error, not a redeclaration. Whether two variables appear to share a name must never change what a program means."* **We enforce nothing today** — zero hits for "shadow" across `parsing/`, `postparsing/`, `typing/`. The scout's stack frame already carries a `VariableDeclarations` list (`postparsing/expression_scout.rs`), so a same-scope duplicate check has its data in hand; that's the cheapest place to put it.
- **Split `Vec<T>` from `List<T>`.** design-1:92/1164 — `Vec<T>` is a plain **struct** (plain group borrowing, no RC, pure-function-legal to build, Send-capable via allocator params, the collection of choice for struct fields; `HashMap`/`HashSet`/`BinaryHeap`/`String` sit in this tier). `List<T>` is its RC'd **class** counterpart (Valen 2). Both share the `[]` element child group and the same invalidation rule: spine in the parent group, elements in a child group via `[]`. We have one notion today.
- **`Vec<int>` elements still form a child group.** design-1:203, called out explicitly as a trap: the inline-field exemption covers owned scalar *fields* only, **not** collection elements, because the test is whether the *container* can relocate or remove them — which reallocation does regardless of element type. Reading it the other way *"makes a spine op invalidate nothing and turns a stale element reference into a read of freed storage."*
- **`comptime`.** design-1:110 — the compile-time binding keyword (`comptime <name>: <kind> = <expr>`), replacing `let`/`alias`/`var` at that layer. It is also the **carrier for associated groups** (`comptime capture_group: group`, `comptime teardown: group = capture_group`), so it's a prerequisite for the trait/erasure work, not just a convenience.

**RULED BY THE SPEC, NOT YET BUILT — the gap inventory.** *(These are **gaps, not bugs** — the language rules them and we simply haven't got there.)* Groups as real values and the `in`-clause grammar · **effect clauses** (the big one — see decision 1 in the PICK-UP block) · `comptime` · no-shadowing enforcement · `Copy` as a definitional property · territory closure `g...` · the `maybealias` / `in` relation vocabulary · poisoning and `dangle` · closed traits and the `trait` / `open trait` unification · the move/state-passing iterator · threading, `parallel`, async · named args, default parameter values, visibility modifiers · exhaustive-match and refutable-pattern-in-`if` enforcement (**we enforce neither**) · `if` / `match` as expressions. Several of these have since picked up rulings — check "Upstream rulings" before treating any entry as unspecified.

**Already aligned — do not "fix" these:**
- Our **`held`** treatment matches theirs exactly. Their ratified `held MyClass` is an *anchored payload borrow*, "conceptually `&MyClass in (anonymous, rc.*[])`" — identical to our desugar above (`&T in e_g where maybealias(e_g, rc.__All), held(e_g)`), and `held(e_g)` being a where-clause fact you must *find* rather than mint is their "the anchor is found, never made." Cancelling `RegionT::Held` (the region-value rep) did not endanger `held` the surface form.
- **The onion already expresses their whole class-reference model**: bare class field/local = `ShareRef(Struct(C))` (`Rc`); bare class param = `BorrowRef(ShareRef(Struct(C)))` (`&Rc`, their anchored borrow); `&MyClass` = `BorrowRef(Struct(C))` (payload borrow); `weak MyClass` = `WeakRef(Struct(C))`. **Not** eagerly decaying `BorrowRef(ShareRef(X))` → `ShareRef(X)` is what preserves the `&Rc`-vs-`Rc` distinction their model rests on — keep it that way.
- Their "storing a borrow into a bare-`MyClass` slot **incs** — the slot drives it" is exactly the `BorrowRef(ShareRef(SC)) → ShareRef(SC)` RC-bump shape. Their `&MyClass → Rc` minting is **deferred** on their side (the "Horn A/B" problem); the coercion table correctly has no such row.

**`set` yields the displaced old value when the type is movable**, which is what decision 11 and
`MutateTE.result` already do. The rule that a `set` on a linear place is an error, with a carved
exception for revival-`set`, is **derivable rather than special-cased**: a linear `set` yields a
linear value, and discarding it is the ordinary "linear value never consumed" error — a better
diagnostic, since it points at the discarded value rather than at `set`. Revival-`set` displaces
nothing, so the exception falls out. `replace()` and `swap()` stop being "the sanctioned alternative"
and become ordinary library functions.

The Valen design session is reachable by mailbox; **check `mailbox list` for the current identity**,
since they turn over.

**Interop projection, live on the extern/export front.** Valen rules that **interfaces get no
representable Rust type** — Rust holds an *opaque handle* and calls functions through it, nothing
more. Only the `open trait` / `Box<dyn>` half projects to a real Rust `dyn`, so an exported observer
registry must be spelled `Vec<Box<dyn EventHandler>>`, not `List<EventHandler>`. Under that rule
`tests_exporting_interface` means exporting an opaque handle, not a dyn-shaped type — **so that test
may be asserting the wrong thing.**

### Region borrow checker — designed, not started
The full design lives in `docs/plans/path-to-borrowing.md`: the ladder (rungs 0-3), the design rulings (regions are inert cargo, a group is an identity not an extent, invalidation keyed on reach, the two join disciplines, the two seams, quarantine by capability, per-body, the whole-signature input), the region and effect rulings, and the rustc evidence. Rung 0 (groups become real) is entirely architect work; nothing past it starts until the go-ahead.

### Reference model
Four ratified decisions, in force. Together they **retire `RegionT::Held` entirely — do NOT add it** — and reshape how a mention lowers.

1. **C1 — what a bare mention yields** (design-1:159-171, :437). **This is design-1's ruling, kept only as the experimental-flag alternative; Vale2 diverges by default for the foreseeable future, possibly permanently — a mention always yields a reference, even for `Copy`, read out to a value only at a bare-value receiver (see the two-divergences banner).** design-1's C1: *"**There is no auto-borrow**: a bare non-`Copy` argument is a C1 error naming `&x`, `x^`, and `x.clone()`."* Three arms: a bare mention **copies** if `Copy`, yielding a *fresh, isolated group* that nothing mutating or destroying the source can invalidate; **errors** for a non-`Copy` struct; **errors** for an unbounded generic `T`. `&` lives in expression position at reference-*creation* sites. A borrow is itself `Copy`, so passing one along stays bare (`resize_buffers(graphics)`) — a borrow reaching a borrow parameter needs no mark. **But reading a value *out* through a reference into an argument does**: argument positions do not adjust (design-1:216), so `keys.append(*k)` is written with the `*` even at a `Copy` key, and *"the `*` is load-bearing there and not decoration"* (design-1:1862). Field and method access are where auto-deref applies — `k.field`, `k.method()` — not argument positions.
   - **`RegionT::Held` is cancelled and must not be reintroduced.** Under C1 there is no bare non-`Copy` use at all, so there is no bare-use ambiguity for a region to mark.
   - **The `LocalLoad` collapse stays while we diverge.** `Use` and `LoadAsBorrow` are merged into one plain `LocalLoad` because a mention already *is* a reference — which is exactly the current divergence, so the collapse holds. The C1 flag would undo it — bare becomes an error, `&x` the borrow, and the `BorrowRef(NC) → bare NC` auto-clone goes with it — but it is off by default and may stay so indefinitely.
   - **Consequence for the borrow checker (under C1):** C1's `Copy` arm yields a fresh isolated group, so a bare `Copy` mention creates **no alias at all** — materially less invalidation surface to track. Under the current divergence a bare `Copy` mention is instead a borrow of the slot until read out, so that smaller surface arrives only under the C1 flag.

2. **Eager auto-deref (chosen over lazy).** A lookup is uniformly the *address-of-slot* — **`LocalLookupTE` stays uniform, do NOT make it idempotent** (there's a `// NOTE:` guard on it). At the **read/use path** (right after the lookup), if the result is `BorrowRef(inner)` where `inner` is itself a reference kind, insert a **`DerefTE`** peeling exactly ONE storage layer → the stored reference. So a `&Ship` local mentions as `&Ship`, not `&&Ship`; this covers local/member/array lookups uniformly. The **mutate path keeps the raw `&&`** address-of-slot (it needs the slot to write). `DerefTE` (`ast/expressions.rs`, `{ inner, result }`, `result = peel_one_reference(inner.result())`) is landed and wired into `result()` + `test/traverse.rs`. Chosen over lazy because it reuses `convert()` (row 7 handles the rare re-borrow to a `&&` target) and makes "mention = reference" literally true in the type. Remaining work is `ZHERE`-marked in `expression_compiler.rs`.

3. **`&&` exists ONLY for bounds, and is still needed.** Genuine `&&` (borrow-of-borrow) is induced by generics ranging over reference types: the typeclass blankets take their receiver by `&`, so a `clone(&T) T` bound at `T = &Ship` needs `clone(&&Ship) &Ship`. Bound resolution is **exact-shape** (inert type-space), so genuine `&&` never flows through normal expression evaluation — only the blanket body performs the one sanctioned `&&→&`. Mention never produces a genuine `&&`. `convert()` keeps `&&→&` a **deliberate error** at call sites.
   - **Why it is load-bearing:** it is what keeps bound resolution finding **exactly one** candidate. Collapse `&&Ship ≡ &Ship` and a `clone(&T) T` bound at `T = &Ship` matches *both* the user's `clone(&Ship) Ship` and the borrow blanket on params — two candidates, which the dispatch rule ("**>1 → ambiguity error**") rejects. The determinism comes *from* the shapes being distinct: `clone(&Ship) -> Ship` and `clone(&&Ship) -> &Ship` differ in parameter type, so exactly one matches at each instantiation; collapse them and both match on parameters, differing only in return type. The collapse does not lose precision — it breaks bound resolution. **Rust is the existence proof**: `<&T as Clone>::clone` has type `fn(&&T) -> &T` in std today, and the `&self` sugar hides the double borrow rather than avoiding it. Asymmetric risk, too — keeping the nesting costs nothing if nothing spells it; removing it costs a distinction std demonstrably needs.
   - **Upstream endorses this, on our own grounds.** design-1:272: *"**`&&T` remains an inhabited type**: it arises by substitution (`f<T>(x: &T)` instantiated at `T = &Engine`), and **an implementation using an address-of-slot model for assignment needs it**."* That is decision 13, cited back at us as a reason. design-1:274 adds that `&&T` sits with `Option<&T>` and `Vec<&T>`, and design-1:384's receiver table carries the `&Engine → &&Engine` bound-only impl outright. **So `&&` is not a divergence and should not be raised as one** — which is different from the open item two bullets down, where upstream wants a *witness* we happen to have.
   - **The asymmetry worth remembering:** only bounds whose signature *already* takes `&T` generate the second `&`. `drop<T>(x &T)` satisfying a `drop(T)void` bound at `T = &Ship` needs just one.
   - **The blanket survives in its current spelling.** *The expression `&e` on a borrow-typed place yields `&T`, not `&&T`* — **scoped to expression formation, deliberately not to type formation.** That scoping is the whole ballgame for us, because **our `&&` is a written type-position templex, not a substituted receiver.** `parse_templex_atom_and_call_and_prefixes` parses `&` in type position and recurses (its own comment: *"`&&T` parses as nested BorrowRef via the recursive call — double-borrow"*), so decision 3's blanket is spelled `func clone<T>(x &&T) &T` with `&&` literally in source. Valen's equivalent arises by substituting `Self = &Ship` into `&self` and nobody writes `&&` at all. **Three distinct origins, and only the first was ruled on:** (a) the expression `&k` — expression formation, `expression_parser.rs`/`Prefix::Borrow`; (b) the written type `&&T` — type formation, `parse_templex_atom_and_call_and_prefixes`; (c) substitution — pure solving, no syntax. (a) and (b) share no code at any layer (different parser, AST family, postparse function, typing path), so the ruling does not touch us. **If a future ruling is ever phrased as "`&&T` is not a formable type," it kills our spelling** — the mitigation is moving to (c)-style once traits/impls exist, since a substituted type never passes through a formation rule.
   - **►► THE KILL-CONDITION IS ANSWERED, AND OUR SPELLING IS WHAT ANSWERS IT ◄◄** Upstream's open item asks whether *"`&&T` is not a formable type"* — their own substitution witness turns out not to be callable. **Our written type-position `&&T` (`parse_templex_atom_and_call_and_prefixes`) is the source route**, and it is folded into their open question with attribution: the ruling rests on written-type formability plus the address-of-slot implementation reason, with no expression ever forming one. Decision 3's spelling is safe.
   - **`&&T` stays inhabited, and it has a second job.** Under decision 13's LLVM model `set k = …` re-points via a raw `&&Ship` address-of-slot while `set *k = …` targets the `&Ship` referent. That is a property of our *lowering*, not of the language — naming a local isn't a computed place, so design-1 needs no `&&T` for it. The language-level residue is real though: **without `&&T` you could not write a function taking a re-pointable borrow.**
   - **Bonus, and a better shape than we were arguing for:** since no source expression can form `&&T`, the blanket's argument **cannot be constructed by hand**, so `.` can never reach it. "Bound-only machinery you can't invoke" stops being a stipulation and becomes a *consequence* — nothing has to state that `.` skips it.

4. **A generic type param `T` is always single-ownership.** Sharedness is now **structural** (the `ShareRef` wrap); share citizens only ever appear `ShareRef`-wrapped, and a `ShareRef` handle is itself single-owned. So a **placeholder carries NO sharedness/mutability** — every sharedness-dependent decision (drop/clone/`weak`/bare-legality) dispatches on the concrete type's wrap structure, deferred to instantiation. `create_kind_placeholder_inner` takes no `kind_ownership`; see decision 2 for the live sharedness query.

    **►► THE INVARIANT IS "NOT AS A VALUE", NOT "NEVER BARE" ◄◄** *(ruled with upstream)* A rune holds
    whatever unification hands it. `func print<T>(x &T)` called with a payload borrow binds
    `T = Struct(MyClass)` — a bare class in a rune — and called with a claim binds `T = @Ship`; one
    generic accepts both. What a share citizen may not be is a **value**; it may freely be the pointee
    of a borrow, which is exactly what a payload borrow is. **`&` is the kind-non-uniform operator,
    not `T`.**

    **Nothing enforces this and nothing needs to.** A function returning a bare share citizen cannot
    be *declared*, because a class name in return position lowers to the claim — so a producer bound
    at that instantiation has no witness and fails as an ordinary "couldn't find a function". The
    mirror case, a by-value use, has no constructible call site. Same shape as decision 3's `&&T`:
    unreachable by construction rather than rejected by a check.

### Docs to treat as partially stale, and loose ends
- **`onion-typing-scouting.md`** — its `file:line` map of the `@`/`heap`/`ShareRef` surface sites is now wrong (those were removed; `OwnRef` added). Re-scout before trusting it.
- **`~/.claude/plans/partitioned-kindling-origami.md`** — the parser/postparse `own`+removal plan, now DONE.
- **LangNotes-Delta** holds their own record of the convergence (their proposal docs were updated on their side).

## Mission — Onion typing

The structural refactor is landed; the remaining onion work is listed under "Start here". The one detail that lives here is the validity table.

**Validity table** — a `KindT` is legal only in these layer/citizen combinations. The interner does **not** enforce this yet; wiring that gate is the remaining structural item.

| Layer / bare | non-share citizen | share citizen |
|---|---|---|
| bare (value) | ✓ | ✗ |
| `OwnRef` | ✓ | ✓ |
| `ShareRef` | ✗ | ✓ |
| `BorrowRef` | ✓ | ✓ |
| `WeakRef` | ✗ | ✓ |


### Resolved design decisions

Each of these shapes the frontend cascade below it.

1. **Weak's shape.** Distinct `Kind::WeakRef(&WeakRefT)` variant. Surface spelling: `weak Spaceship` keyword — frees `&&` for double-borrow. WeakRef wraps share-flavored citizens only.
2. **Share's shape.** A property of the citizen's *definition* (its declared mutability). NOT stored on `StructTT` / `InterfaceTT`, and nowhere in the `KindT` enum. Not an onion layer. Share citizens cannot be held bare — must be wrapped in one of `HeapOwnRef` / `ShareRef` / `BorrowRef` / `WeakRef`.
    - **The live query is `declare_type_sharedness`** (`compiler_outputs.rs`), read through `struct_compiler_get_sharedness` (`struct_compiler.rs`), whose only caller is `struct_constructor_macro.rs`. **`get_sharedness` and `lookup_mutability` do not exist** — they were deleted with placeholder sharedness, and every remaining `get_sharedness` mention under `typing/` is commented out. **This is the query the position rule will need**; see the wrap-chain block under CALL-SITE PHASES.
3. **Bare-use and `&`** are governed by C1 and the reference model — see Reference-model decision 1. **`RegionT::Held` does not exist and must not be introduced**; a lookup is the uniform address-of-slot, with a read-path `DerefTE` peeling one storage layer.
4. **Coord's fate.** `Coord` disappears entirely; walking is pure `Kind`. Region lives on `BorrowRefT`, the only ref layer that carries it.
5. **`convert()` / auto-coercion.** See the coercion table below. **The `implicit_clone` probe mechanism is retired** — what survives is the structural rows plus the upcast.
6. **Backend representation.** All IR stages get the onion — T-IR, I-IR, H-IR. `CoordH` disappears symmetric to `CoordT`. Backend C++ / Metal eventually walk the onion (large end-state refactor; scoped as a follow-up Backend arc after the frontend arc lands).


7. **The expression hierarchy flattens.** No `ReferenceExpressionTE` / `AddressExpressionTE` / `AddressResultT` / `IExpressionResultT`; one `ExpressionTE`, and `ExpressionTE::result()` returns a `KindT`. An addressible expression is just a reference expression plus a reference: the lvalue lookups (`LocalLookup`, both member lookups, both array lookups) each return a `BorrowRef` of the thing they point at, e.g. a borrow-ref of an int element, or a borrow-ref of a shared-ref class. Cost: lvalue-vs-value is no longer in the type, so `evaluate_expected_address_expression`'s demand loses its compile-time basis.
8. **`SoftLoadTE` dissolves.** A lookup already hands back a `BorrowRef`, so there is nothing left to load. Its call sites collapse onto the lookup result. The load's remaining job, reading an `&P` out into a `P`, is a `convert()` coercion rather than an instruction.
9. **An owned value is a bare kind**, i.e. zero ref layers. Constructors (`NewRuntimeSizedArray`, `StaticArrayFromCallable`, owned `Construct`) produce bare kinds. **`HeapOwnRef` comes much later**: don't design for it, and stub it with `panic!("implement: ...")` wherever it surfaces in a dispatch. (Caveat, see the reference/ownership surface model: heap-ownership is a library `Box<T>`, so `HeapOwnRef` may end up vestigial; the exclusive surface form is `own`, whose wrap target is open.)
10. **Expression nodes store their result.** Every `*TE` carries a `result` field plus a private `_sealed`, and is built through a mandatory `new()` that computes the result and allocates any wrap payload into the arena. Wrap-producing nodes store the payload ref (`&'t BorrowRefT`), so reading a pointee is a field access rather than a walk. Per-struct `result()` getters are gone; only `ExpressionTE::result()` remains.
11. **`set` takes a borrow and yields the old value.** `MutateTE`'s destination is always a `BorrowRef`, and its result is the destination's pointee, i.e. the value that was replaced. So `set` is the one sanctioned move-out-of-a-borrow. It's safe because the hole is refilled in the same operation, exactly like Rust's `mem::replace`, which is why `CantMoveOutOfMemberT` can stay the rule for bare moves. This retires RMLRMO (`docs/old/Compiler/Templar/Addresses.md`), whose "result in the member's type" conclusion assumed a place-typed destination.
    - **The `VCOORD: onion old-value semantics to confirm` marker is DISCHARGED.** The ruling scopes to *movable* types, and the corpus rationale for design-1:1703's unnameable temporary was checked rather than assumed: it has two motives and **neither argues against yielding**.
    - **The ordering motive is load-bearing and easy to break.** `valen-approach-convo-12:307` wanted *install-new-before-tearing-down-old* so a reentrant observer always sees a consistent object; the temporary was unnameable as a *consequence* of choosing "drop it" over "hand it back," not as an independent requirement. **A returning `set` preserves it only if we install first and hand back second.** Implementing it as hand-back-then-install would satisfy the ruling and silently break the reason underneath it — this belongs in a source comment wherever the lowering lands.
    - The second motive (`convo-12:514`, an adversarial pin) is that the linear *(was "linear-strict" — see ruling 13's vocabulary inversion)* error was *engineered* to stop a silent linear drop. A returning `set` serves that strictly better: the value is handed to the caller and discarding it is the ordinary unconsumed-linear error.
    - **Two pins to keep wired:** a **poisoned** old value hands its poison to the receiving binding (`convo-12:738`, design-1:713 — we have no poison-travel yet, so this is a list item, not code); and destroy-in-place lowering becomes conditional on the result being *unused* (as-if, so no semantic exposure).
    - **Grammar consequence: `set` must be expression-valued and legal as the RHS of a `set`** (`set x = set y = set x = None` swaps two vars). Our parser appears to satisfy this already — `set` parses to `IExpressionPE::Mutate`, reachable both from statement position and from the atom parser in `expression_parser.rs`, with the mutatee/source split taken at the first `=` so chaining recurses. **Read from the code, not verified by a run** — worth a test case when the parser is next touched.
    - **Residual:** `swap` still does not compose from `set`, because the innermost `set` needs a value to install. It works for any type with a spare inhabitant (`set x = set y = set x = None`) and not otherwise, so design-1:2551(b) is not a place-parameter grammar hole but a **vacancy hole, one stdlib function wide, over spare-inhabitant-free types**. Expect `swap` to stay stdlib-with-unsafe-internals.
12. **Regions are all `RegionT::Default` for now.** `RegionT` is an enum (`Iso`, `Default`), and every `BorrowRef` the typing pass builds stamps `Default`. Region threading is deferred.
13. **Addressibility is retired for good**, not deferred. Master's `IVariableT` had Addressible-vs-Reference as its outer axis and ownership as its inner one; the 4→2 collapse (`538fdb12a`) kept Local/Closure and dropped the other axis, which left `determine_if_local_is_addressible` and `determine_closure_variable_member` as wreckage rather than drift. The replacement is an LLVM-style model: every local is storage, a lookup yields a pointer to it. Mutation-sharing becomes `&x` to the same storage; moves become move-out-of-a-borrow (a compile error, the user writes `^`); lifetime becomes regions on `BorrowRefT`; layout is the hammer's problem, not typing's. Note nothing enforces the lifetime half yet, since `LocalLookupTE::new` hardcodes `RegionT::Default`. Addressibility is **orthogonal to the onion**: it was about whether a variable's slot is indirected, while onion layers are about what the value is.
14. **`Copy` is a property of the citizen's definition**, the way sharedness is. C1's first arm is keyed on it, and design-1 is the spec, so Vale2 has a `Copy` property rather than an `implicit_clone` probe. **`implicit_clone` retires completely**, since `Copy ⟹ Clone` and primitives are `Copy`, so `clone` covers the one case a probe was ever kept for.
    - **`Copy` is OPT-IN, `#derive(Copy)`** — not structural. A struct of all-`Copy` fields is *eligible* but is not `Copy` until the author says so. Expect the ergonomic consequence: `struct Point { x: int, y: int }` is a C1 error on first bare mention.
    - **The derive gate must check for a `drop` FUNCTION, never the `T: Drop` bound** — see the trap below. It is the single easiest thing here to get wrong.
    - **C1 does not queue behind the generic-bounds family.** Of its four arms only `where T: Copy` needs bounds; the unbounded-generic arm needs nothing, being the absence of a bound and the erroring default. Shipping the three non-bound arms rejects programs that should be accepted and never accepts programs that should be rejected — fail-safe, with the `T: Copy` refinement a monotone add-on.
    - **Guaranteed `Copy`:** primitives (design-1:1517, design-2:518), **borrows** (design-1:169 — load-bearing, it is what makes pass-along bare), **weak refs** (design-2:266, plain `(index, generation)` data). **Guaranteed not:** constraint refs (design-2:628), `ownref` (it carries the duty to destroy), `Box` and owning values, anything linear. `dyn` needs no rule of its own — `&dyn X in g` is a borrow so `Copy`, `Box<dyn X>` is owning so not.
    - **Class strong refs are OUTSIDE the `Copy` axis entirely.** A bare class mention is governed by the class reference model, not C1's first arm: in storage it is a claim, and a claim *copy* is an effect-charging increment (design-2:51), whereas C1's rationale for auto-copying is that a `Copy` read costs nothing. As a parameter it is an anchored borrow and nothing is duplicated. **Do not model claims as `Copy`.**
15. **`y = x` copies iff `Copy`**, yielding a **fresh, isolated group** that mentions nothing of the source, so nothing that mutates or destroys the source can invalidate it. This is C1's first arm. Not implemented. The ignored `user_defined_implicit_clone_allows_bare_use_of_struct` test asserts the retired probe rule and needs re-authoring or deleting.

16. **Overlapping impls are OUTLAWED — Rust's coherence rule, without specialization.** Two impls **overlap** when some substitution makes their (sub, super) pairs the same; that is a compile error at the impls, not a resolution question at the call site.

    | | verdict |
    |---|---|
    | `impl ISpaceship<int> for Firefly` declared twice | **error** |
    | `impl<T> ISpaceship<T> for Firefly<T>` **and** `impl ISpaceship<int> for Firefly<int>` | **error** — the specific one does *not* win; there is no specialization |
    | `impl ISpaceship<int> for Firefly` **and** `impl ISpaceship<bool> for Firefly` | **legal** — different super kinds, so a different pair |

    **Nothing enforces this today.** There is no coherence or overlap check anywhere in `typing/` — zero hits for conflicting/overlap/duplicate-impl detection. What exists instead is a bare `assert!(oks.len() <= 1)` inside `is_parent` (`typing/citizen/impl_compiler.rs`), which resolves *every* impl relating a sub/super pair and then asserts at most one succeeded. So both error rows are an assertion failure at the first call site that touches the relation — no diagnostic, no source location. The check belongs at impl-declaration time; the `assert!` then documents an invariant something upstream actually guarantees.

    **This is why explicit `T` simplifies the implementation** (see the CALL-SITE PHASES section). With the super side concrete, `is_parent` looks up one specific sub/super pair and the legal row is simply a different pair. Had we kept deduction-through-upcast, `get_parents` would return *both* supers for `Firefly` and we would have needed a disambiguation rule — which, without specialization, could only have been an ambiguity error anyway.

    Related and owed alongside it: `SharednessImplingMismatch` (defect 9) is the other never-written impl check, a few lines away in the same function.

### Coercion table

Notation: `NC` = non-share citizen kind, `SC` = share citizen kind, `P` = primitive.

**There is no `implicit_clone` probe** *in the design*. `Copy ⟹ Clone` is ruled and `#derive(Copy)`
implies `Clone`, so `clone` covers the primitive case a probe was ever kept for, and under C1 the user
writes `.clone()` explicitly. **Do not add a target-site probe row** — the source→target analysis
below is the map of where a coercion can occur, and every legal one is structural or an upcast.
**The code has not caught up**: `convert_via_implicit_clone` is live and still emits the two
`ImplicitClone*` errors. See "What blocks / what to preserve" for the extent.

| # | Source | Target | Op |
|---|---|---|---|
| 4 | `BorrowRef(K, r)` | `BorrowRef(K, r')` (regions differ) | pass-through |
| 5 | bare `K` (from `^local`, literal, ctor) | bare `K` | pass-through |
| 6 | `HeapOwnRef(K)` / `ShareRef(SC)` / `WeakRef(SC)` (from `^local`) | same shape | pass-through |
| 7 | bare `K` (from a literal or ctor) | `BorrowRef(K, r)` | materialize a hidden local, lend it, defer its drop, e.g. `&2` |

Plus the upcast, which `convert_via_upcast` owns.

**Errors — no silent coerce:**

- (d) `BorrowRef(BorrowRef(K, r_i), r_o)` → `BorrowRef(K, r)`. Double-borrow arises only from an
  explicit `&&x` or generic instantiation. The borrow blanket satisfies it via bound resolution;
  auto-coercion does not peel it.
- `BorrowRef(HeapOwnRef(K), r)` → `HeapOwnRef(K)` — move-out-of-borrow; the user writes `^local`.
- `BorrowRef(NC, r)` → bare `NC` — a read-out that would need a clone. The user writes `.clone()`.
- Kind mismatch across any coerce site.

**Status in `convert()`.** Rows 5, 6, 7 and the upcast are implemented; row (d) is an error by design.
**Row 4 is unreachable** until regions get real, since every borrow carries `RegionT::Default` — and
nothing in `convert()` unifies the two regions, which is still undecided. The numbering is sparse
because it outlived the probe rows that used to sit between these.

**The coercion sites.** `convert()` has exactly six callers, and they enumerate every place a coercion can happen:

- call arguments, via `convert_exprs` from `evaluate_call`
- a function body's result against its declared return type
- a `return` statement
- each branch of an if/else, against the common type
- a `Mutate` / `LocalMutate` destination
- a let / pattern binding

Not all six pre-guard with `is_type_convertible` (the pattern binding doesn't), so `convert()` reports conversion failures as `CouldntConvertT` / `CouldntUpcastT` rather than panicking.

### The value model as it stands

`KindT` carries the four ref wraps; `CoordT` / `OwnershipT` / `LocationT` are dissolved or dissolving.
Conventions and shapes a change here could break:

- **Every `*TE` node stores its `result`** and is sealed behind a mandatory `new()` (decision 10).
  `ExpressionTE::result()` is the canonical accessor, and the convention is **present the full onion,
  never silently drop to `.inner`**. `ExpressionTE::kind()` is a bare alias for it — a coord-era
  leftover that wants deleting.
- **The four ref-wrap structs are NOT interned.** They are Polyvalues: derived structural `Eq`/`Hash`,
  no `_must_intern`, no `*ValT` twin. They compare by value, so they never need canonicalizing.
  `interner.alloc(BorrowRefT { .. })` is correct, and there is no interner work pending for the ref
  layers. **`TypingInterner` is a real hash-consing table**, structurally what rustc's `CtxtInterners`
  is — five dedup maps, with `intern_name` / `intern_id` / `intern_prototype` / `intern_signature` /
  `intern_kind_payload` doing lookup-then-allocate-on-miss behind the `@SICZ` construction seal. Only
  the bare `alloc` / `alloc_slice_*` are raw bump, and they serve the identity-bearing nodes @WVSBIZ
  says must never be interned. `ScoutArena` has the same shape for `'s`.
- **`convert()` matches `(source_kind, target_pointer_type)` pairs directly**, one arm per coercion-table
  row, with `convert_via_upcast` split out. The old `(source_ownership, target_ownership)` match is gone
  — that two-axis decomposition was the thing the onion dissolves.
- **The struct-member model is flat**: `StructMemberT { name, tyype: KindT }` (`typing/ast/citizens.rs`).
  The recipe at consumers is `member.tyype`.
- **`visit_kind` emits a node for every onion layer** on the way down before the base kind
  (`typing/test/traverse.rs`). `visit_coord` / `NodeRefT::Coord` do not exist.
- **`pointify_kind` is commented out** with its sole call site; the assert there was tautological
  against a hardcoded `SharednessT::Single`.

#### Latent hazards

- **`InterfaceToInterfaceUpcastTE::new` is `unimplemented!()`** (`typing/ast/expressions.rs`), carrying
  `VCOORD: preserve the inner wrap and swap the innermost citizen`, with zero callers. Its sibling
  `UpcastTE::new` is implemented (it calls `replace_value_type_in_ref`), and `convert_via_upcast` routes
  through it as a live path.

### How rune-typing is wired

- **The rune-type solver is called multi-site on demand, not once at entry** — from
  `array_compiler.rs`, `overload_resolver.rs` and `expression_compiler.rs`, for arrays, overload
  candidates and `let` bindings. Deliberate, and easy to mistake for a missing single entry point.
- **Rune-type maps are derived on demand and stored nowhere** — `derive_rune_to_type`, in
  `typing/rune_typing/derive.rs`. No cache is needed because rune-typing does **not** recurse across
  denizens: a denizen referencing another reads that one's *declared signature* off the postparse AST
  rather than solving it, so each derive is self-contained.
- **`pass_manager.rs` and `pass_manager/full_compilation.rs`** stay gated in `pass_manager/mod.rs` and
  hold the last `use crate::higher_typing::*` imports in the tree. Harmless while gated; they want
  deletion or rewiring once the pipeline shape settles, as do the one-line mentions in
  `simplifying/`, `instantiating/` and `integration_tests/`.

**Slice ordering:** the typing slice, then the instantiator / hammer / backend arcs.

### What blocks / what to preserve

- **`AliasTE` / coercion-accept patches / coherent-collapse arms are gone** — no live `#[cfg(any())]`-gated arms remain in `typing/`; every `Augment` / `AliasTE` / coherent-collapse reference is now an inert comment.
- **►► THE `implicit_clone` PROBE IS RETIRED IN THE DESIGN AND STILL LIVE IN THE CODE ◄◄** The
  coercion table is the design statement; **the removal is unstarted work nobody has listed.** What is
  still there: `convert_via_implicit_clone` (`typing/convert_helper.rs`) and its four call sites, the
  two error variants `NoImplicitCloneDefinedT` / `ImplicitCloneRejectedT` — **still emitted**, with
  live humanizer arms — the `implicit_clone` keyword, six builtin registrations, one
  `expression_compiler.rs` site, **10 `.vale` corpus files** and roughly 40 test references. Deleting
  it is a real slice, not a sweep: the `.vale` sites and the tests that assert on the two errors have
  to go with it, and `user_defined_implicit_clone_allows_bare_use_of_struct` is the ignored test that
  asserts the retired rule outright. **Do not read the coercion table as a description of the tree.**
- **The resolver structural-consistency principle** (Augment DIR1 Shared-arm reject-on-contradiction) — the specific check migrates to whatever mechanism enforces onion-typing structural constraints in the solver. Preserve the spirit, not the check.
- **The "drop(bare_local) is a compile error, drop(^local) is mandatory" rule** — semantics, not representation. Preserve.
- **6 rune-type-inference test fixtures** preserved verbatim in `docs/regression-fixtures-from-retired-higher-typing.md` — cover pack literal (`Refs(int, bool)`), empty pack + `Prot[P, str]`, plain-param "undefined name" error, param-position / template-call / recursive-field rune-type-map assertions. Re-author against `KindListSR` + `KindTemplataType` + on-demand rune-type derivation (`derive_rune_to_type`) when the typing slice lands the rune-type solver at `typing/rune_typing/`.

### Deferred test coverage (add after the main goals land)

- **Lookup-failure errors have no production test.** `CouldntFindTypeT` / `TooManyTypesWithNameT` are only exercised by humanizer-format tests; nothing compiles source that emits them, and `TooManyTypesWithNameT`'s humanizer is a stub. Retiring `explicify_lookups` moved detection into the rune-type solver, which wraps them as the coarser `HigherTypingInferError` — so specific-variant surfacing could degrade unnoticed. Fix: re-author the preserved "undefined name" fixture (above) as an end-to-end test asserting the specific error, add a `TooManyTypesWithNameT` case + fill its humanizer, and pin the taxonomy decision (keep `HigherTypingInferError` vs. unwrap to the specific variants).

### Critical reminders

- **NEVER commit without the architect's literal "fire commit" or "fire commit temporary".** Grammar note: the target-branch slot uses `with <target>`, sharing the `with` keyword with the CI opt-in. Parser rule: `with CI` = CI gate; `with <anything-else>` = target branch. Both can appear in the same invocation.
- **NO `#[ignore]` additions** without architect approval. If a test regresses during the onion arc, surface it and get direction; don't silently ignore.
- **Surface before reverting.** Onion typing is a big arc and mid-flight discoveries will surface things the initial design didn't foresee. Surface the situation + alternatives before undoing landed work.
- **The "green suite at commit time" invariant is suspended during the typing slice.** Typing is intentionally red while the semantic cascade runs — do not treat compile errors as regressions; they're expected fallout. The invariant reactivates once typing is green again.
- **When re-linking a module**, expect a wave of compile / semantic errors. That is the intended behavior — the parser slice weaponized structural mismatches. Work through them; don't stub around them without architect approval.

---

## Mission — Overload resolution & dispatch model redesign

The type-system parts of the earlier framing (distinguishing `Borrow + share-kind` from `Share T` via three collapse-site fixes + auto-alias) shipped as the coherent-collapse route in `f47279978` and are now what onion typing supplants. What remains active is the **lookup/dispatch model** below.

> **►► THIS SECTION IS PRE-ONION IN PLACES — READ IT WITH SUSPICION.** The share-clone block under
> §Typeclass-like operations is current; assume stale siblings exist elsewhere in the section.

### Overload resolution

**Single rule.** Collect all candidates whose params match the args. If 0 → "no function found." If 1 → win; if that candidate has bound-resolution failures or other rejections, surface THOSE specific reasons directly (don't wrap in `CouldntFindFunctionToCallT`). If >1 → ambiguity error; user disambiguates explicitly.

**No specificity, no phases, no fallback, no tiebreakers.** Two equally-matching candidates is always an ambiguity error.

**The one thing that looks like an exception and is not.** The namespace union is *ordered*, so a
user's class method shadows a same-named builtin blanket (see CALL-SITE PHASES). That is
candidate-**set construction** — which already has rules, "the namespaces of the arg types" being
one — and not preference *within* the set, which is what this bans. Rust draws the line in the same
place, inherent impls before trait impls. **Stated here as well as at the rule itself**, because an
absolute sitting next to an unstated exception erodes toward the convenient reading.

**►► THE FILTER IS FINAL, AND IT IS PURELY STATIC ◄◄** — "params match the args"
is decided **before value-solving**, from information available with no solving at all:

- **arity**
- each parameter's **wrap chain** — `type_outer_ref_rules` is a list of `BorrowRef` / `WeakRef` /
  `OwnRef` rules, so the variants are readable directly
- each parameter's **value-type template name**, or "it is a bare rune, which accepts anything" —
  `value_type_rules`' outermost `Call` is templated on a `Lookup` by name

Both halves come off @PFVSZ's split, which is the third job that split does (the others are phases 0
and 4; see CALL-SITE PHASES).

**►► THE WRAP CHAIN IS COMPARED UP TO ADJUSTMENT, NOT BY EQUALITY ◄◄** The admissible gaps are exactly
what **phase 0 can later deliver** — auto-ref, auto-move, auto-deref — and nothing else. That is the
invariant to hold: *the filter admits exactly what phase 0 can perform*, so a drift either admits a
candidate phase 0 cannot fix up or rejects one it could have. Reading "compare the wrap chains" as
equality is the natural mistake and it breaks phase 0's own motivating case, `foo<T>(x &T)` called
with an owned `Ship`, where the parameter is `[BorrowRef]` and the argument `[]`.

Two gaps are **not** admissible, and each does real work. **Bare to `ShareRef`** — minting a claim from
a payload is deferred upstream, so a claim parameter is an exact requirement. **Borrow to
double-borrow** — the coercion table has no such row and ruling 9 makes `&e` on a borrow-typed place
yield `&T`, so `clone(x &&T) &T` is rejected against a `&Ship` argument on shape alone.

Note the two cases resolve differently from that one rule. `drop<T>(x &T)` against an owned `int` *is*
an admissible auto-ref, so shape does not exclude it and the namespace does — the blanket lives in
`&T`'s file and a peeled lookup never searches there. The `&&T` blanket is excluded by shape *and* by
namespace.

**Whatever survives that filter is the answer.** Solving never eliminates a candidate. Exactly one
candidate is ever solved, there is no speculative work to discard, and *which function am I calling*
never depends on generic inference.

**►► THE FILTER IS LOOSE; BOUND RESOLUTION IS EXACT. DIFFERENT LOOKUPS ◄◄** They read as contradictory
until you notice they are not the same mechanism — see §Bound resolution for the exact half, which
`resolve_function` in `typing/compiler.rs` requests by passing `exact = true`. Decision 3's `&&T`
distinctness argument lives entirely in that half and is untouched by anything the filter admits.

**The shape to stress-test it against** is two inherent impls of one generic type carrying disjoint
bounds and a same-named method. That is where rustc's `method::probe` has to run the solver per
candidate and *eliminate* on failed obligations — so if applicability can ever turn on a bound rather
than on a parameter shape, a static filter cannot decide it. Name-uniqueness per namespace makes this
an error at the declarations instead, which is why the two rulings hold each other up.

The accepted cost: a program with two structurally-matching overloads where only one would actually
typecheck is **rejected**, not silently resolved. Example — `foo<T>(a Vec<T>, b T)` and
`foo(a Vec<int>, b str)` called with `(Vec<int>, str)`: both pass the static filter, only the second
solves, and the user must disambiguate. That class needs two overloads whose parameter *templates*
agree while their rune structure disagrees, and is expected to be thin.

**►► A NAME IS DECLARED AT MOST ONCE PER NAMESPACE — VALE HAS NO OVERLOADING ◄◄**

> Two functions with the same name in the same namespace are an **error at the declarations**,
> whether or not their parameter shapes overlap. The user renames one.

This supersedes the earlier *overlapping* overloads rule, which allowed `launch(Ship, Planet)`
alongside `launch(Ship, Star)` because no argument tuple satisfies both. Both are now errors, and the
motive is diagnostics: the collision is reported at the two declarations rather than at whichever call
site first happens to reach both.

**The stricter rule is also the cheaper check.** Overlap asks *does some substitution make these
accept one tuple*, a unification search. Name identity is a failed hashmap insert. So the local
declaration-time check the table below wants is now complete and trivial rather than partial.

**This makes Vale a language with no overloading**, in the Cardelli–Wegner sense of one name denoting
several definitions chosen by argument type. What remains is *namespace-qualified lookup*: an
argument's type selects the namespace before any candidate set exists, which is structurally what a
receiver type does when it selects a Rust inherent impl — impl blocks and files being two spellings of
the same qualification. Do not describe `foo(x int)` in `int.vale` and `foo(x str)` in `str.vale` as
overloading; those two are never candidates for one call.

**What survives as genuine multiplicity is the cross-namespace union**, since a call searches the
namespaces of *every* argument type rather than one receiver's. That is the "no first parameter is
special" ruling, it has no Rust analogue, and it is why the `>1 → ambiguity` backstop stays live.

**Consequence for the builtins: `drop.vale`, `arith.vale`, `clone.vale` and `logic.vale` dissolve**
into per-type files — `drop(x int)` into `int.vale`, and so on for each type a function is about. That
is required rather than cosmetic, because legality is now per-namespace and those files are not any of
their types' files, so today's eight `drop` declarations live in no namespace at all.

**►► THE RULE MAKES AN ORDINARY CLASS OF PROGRAM UNWRITABLE — UNRESOLVED ◄◄** Bare/borrow twins are
the *easy* collisions (9 in `arith.vale`, 4 in `clone.vale`, 2 in `logic.vale`), and C1 does retire
those. The rest is not that shape and C1 says nothing about it:

- **Cross-product overload sets over two concrete types.** `stdlib/src/str.vale` declares `StrSlice`
  beside the `str` operations and holds the full cross product — `contains` ×4, `find` ×4, `==` ×4,
  `<=>` ×4, `slice` ×5. Each mentions *both* types, so each lands in **both** namespaces.
  `tests/castutils/castutils.vale` has six `+` over `int`/`str`/`bool`/`float`.
  `builtins/resources/migrate.vale` has two `migrate` that both mention `&[]E`.
- **Owned/borrow abstract pairs with different semantics.** `opt.vale`'s `get<T>(opt Some<T>) T`
  beside `get<T>(opt &Some<T>) &T` — different return types, one consumes and one borrows. C1 outlaws
  bare mentions at *call sites*; it does not retire a declaration that legitimately consumes.

**The load-bearing consequence: a user cannot write `println(int)` and `println(str)`**, because they
do not own `int.vale` or `str.vale`. Rust expresses these through trait impls — a second axis of
qualification. Vale's only axis is the file a type is declared in. `tests/printutils/printutils.vale`
declares `println` ×4 and `print` ×6 today. **No replacement has been named, and this is not a
migration detail.**

**`borrow.vale` and `void.vale` are owed** — `drop<T>(x &T)` belongs to `&T`'s namespace, and
`drop(x void)` / `clone(x void)` to void's. Neither file exists.

**►► `drop<T>(v void, x T)` IS DEAD — DELETE IT RATHER THAN REHOMING IT ◄◄** It exists only to satisfy
a bound spelled `where D Prot = func drop(void, E)void`, which `arrays.vale`'s two `__free_replaced`
externs carried and which no longer exists anywhere — `grep "drop(void"` over the corpus returns
nothing. Its body is a bare forwarder to `drop(^x)`, no corpus site calls a two-argument `drop`, and
the compiler never synthesizes one: `drop_since` builds a `Consecutive` of `Unlet` plus one-argument
drops and a trailing void literal. **Its one live effect is a cost** — every program importing
`v.builtins.drop` scouts its `where func drop(T)void`, which is the `KindList([T])` that stood in front
of 35 of one cluster's 38 tests. Deleting it may green something rather than break something; verify by
deletion and a suite run. This is also the worked example of name-uniqueness paying off: it lands in
`void.vale` beside `drop(x void)`, collides, and the right answer turns out to be deletion rather than
an arity rule.

**Arity does not separate two declarations — only names do.** `print(s Ship)` beside
`print(s Ship, verbose bool)` is an error even though no call could confuse them. *(A per-name-and-arity
relaxation may come later; nothing should be built assuming it.)*

**The corpus bends to it, and the resolutions are decided:** drop `vassert(bool)` and the one-arg
`vassertEq`, keeping the `msg str` forms; rename the two-argument `HashMap` and `RHashMap` into helpers
that call their three-argument versions; delete `Array<E>(size int)`; rename `arith.vale`'s unary and
binary minus to `__negate` and `__subtract`; and give the `opt.vale` / `result.vale` owned-borrow pairs
distinct names — `get`, `isEmpty`, `expect`, `expect_err`, four apiece.

**`has.vale` needs no change, and the reason generalizes.** Its four `has` declarations live in a file
that declares no type, so none is in any namespace and uniqueness never fires; the static filter
separates them on arity and value-type template. **Import-only functions are where overloads live**,
and that is the design rather than a gap.

**Operators are not a special case.** A token desugars to a name — `__negate`, `__subtract` — so
"the user cannot rename an operator" is not an argument for anything.

**►► A GAP THE RULE DOES NOT REACH: functions whose parameters are all bare runes ◄◄** They mention no
concrete type, so they belong to *no* namespace and are import-only — and name-uniqueness is defined
per namespace, so it has nothing to key on. Either a file's own scope counts as a namespace for this
purpose, or these need a separate check. **Both live instances are ruled rather than open**: `===` is
never to be overloaded by anything, and `as.vale`'s pair is renamed — the borrow-taking one keeps `as`,
the owning one becomes `take_as`. That is two declarations plus the `(^ship).as<Raza>()` call sites,
which today are the two `downcastOwning*` corpus programs and their Rust fixtures. The gap survives the
rename; only its known instances go away.

**Where it is checked — deliberately not global.** The **call-site filter already errors before any
solving**, which satisfies the real requirement ("we should not even try to solve those"). A
declaration-time check is worth adding only where it is *local*:

| collision | detectable | where |
|---|---|---|
| same namespace — two `foo(Ship)` in `ship.vale` | **locally**, one file | declaration time, better error |
| cross-namespace — `foo(Ship, Rocket)` declared in both `ship.vale` and `rocket.vale` | only once you know which namespaces union | call site, before solving |

So the `>1 → ambiguity` branch is **live, not dead** — it is the backstop for the cross-namespace
case, which no local check can reach. A whole-program pass would catch it, but nothing else here
needs whole-program visibility and it is not worth buying that for a better error location.

**This is what made namespace membership load-bearing.** With no tiebreaker covering for a wrong
answer, whether `&Ship` mentions `Ship` decides whether an ordinary `clone(&myShip)` is ambiguous.
It does; see CALL-SITE PHASES.

### Dispatch model — namespace-based, no Self specialness

**`x.foo()` and `foo(x)` search the exact same candidate set.** No Self-based namespace, no separate dispatch path for dot-syntax.

**Dot is NOT *pure* sugar; it performs a receiver adjustment.** A "purely sugar over the free-function call form" reading is refuted by the corpus: `keys.append(*k)` has an **owned** receiver and `append` takes `&self mut`, so `.` must **autoref**; `set self.hp -= 10` has a **borrow** receiver, so `.` must **deref**. (The argument's `*` is unrelated — arguments do not adjust; only the receiver does.) Both shapes are everywhere in design-1 (`xs.push(y)`, `dict.remove(k)`, `handlers.append(box(...))`), so a no-adjustment reading would require rewriting most method calls in the spec.

What survives is the **namespace** half, which is the substantive claim: adjustment changes the *receiver's type*, not *which functions are findable*. So the correct statement is **"`.` is sugar for the free-function form after a receiver adjustment"** — one clause added, and everything below about namespaces, per-call lookup, and no-first-parameter-specialness stands unchanged.

**Still open upstream:** the exact adjustment rule. A deref-first proposal was floated and **withdrawn** (it flipped the RC default versus Rust and was solving the wrong problem). The current shape under adversarial review is *"where a handle and its referent are distinct, `.clone()` reaches the referent, so no handle-duplication operation is a `.clone()` candidate"* — which would make the auto-derived reference clone **bound-only machinery**, exactly what decision 3 already assumes. Don't build the adjustment rule until it lands; the *namespace* work is unblocked either way.

**Namespace membership rule.** A function lives in type T's namespace iff it is defined in T's file
(i.e. the file that defines type T) **and** either:
- (b1) it mentions T in a parameter, **or**
- (b2) **it is named T.**

**(b2) is what makes constructors reachable at all.** A synthesized constructor's parameters are the
struct's *member* types — for `struct Some<T> { value T; }` that is `Some(value T)`, which mentions no
concrete type — so under (b1) alone it would be in no namespace and findable only by import, which is
not how anyone writes `Some(5)`. The clause also rescues every function that names its type only in
the **return**: `str(x int) str`, `float(x &int) float`, `Array<E>(size int) []E`, all of which land in
no namespace under (b1) once the builtins are split per type.

Examples:
- If `ship.vale` defines both `Ship` and `Captain`, a function `func foo(s Ship) Void` defined there lives in Ship's namespace only. A function `func bar(c Captain) Void` lives in Captain's namespace only. A function `func baz(s Ship, c Captain) Void` lives in both. A function `func qux() Void` lives in neither (only findable via explicit import).
- `Ship` and `&Ship` are **different namespaces**. `Ship`'s namespace lives in `ship.vale`. `&Ship`'s namespace lives in the builtin file that defines `&T` parametrically (`borrow.vale`).
- Same for `&&T`, `Vec<T>`, `Tup2<A,B>` — each parametric language-level type has an associated builtin file whose functions become that type's namespace, propagated to all instantiations.

**"Mentions T in a parameter" sees through the reference wraps.** `&Ship` mentions `Ship`, and a
`ShareRef(Struct(Ship))` mentions **both** the strong-ref namespace and `Ship` — see the ordered-union
block in CALL-SITE PHASES, which is where that rule and its reasoning live. Stated in terms of the
wrap, not `@`, for the reason given there: `@` is contested and the rule does not depend on it. Declaration side and call side must see through
identically or they never rendezvous: the call side already peels, so the declaration side must too.

**Lookup rule.** Resolver collects candidates from:
1. The union of namespaces of every arg type at the call site.
2. Plus any function explicitly imported into the current scope (for utility functions like `min(a, b)` in `math.vale` which doesn't define a type — user must explicitly `import` math.vale to make its functions findable from non-namespace lookups).

Both sources contribute; the strict ambiguity rule applies to the combined candidate set.

**No "first parameter is special."** `foo(ship, rocket)` looks in BOTH Ship's namespace AND Rocket's namespace. A function findable through either argument is in the candidate set. This generalizes Rust's Self-dispatch — Rust dispatches by Self type only; this model dispatches by every arg's type.

**Per-call lookup, no namespace import ceremony.** Having a value of type T at a call site is enough to make T's namespace searchable for that call. Users don't need to separately `import` Ship's `clone` from Ship's type — both come together automatically by virtue of using a Ship value.

### Typeclass-like operations (clone, drop, eq, hash, ...)

Language provides two blankets per op (borrow and share flavors); user provides Own-flavored implementations per type:

```vale
// in borrow.vale (builtin) — for when T is a borrow type
func clone<T>(x &&T) &T { x }              // satisfies clone(&T)T bound
func drop<T>(x &T) Void { }                // satisfies drop(T)void bound

// at class kind there is NO written blanket — the compiler synthesizes the claim
// clone (the RC bump), the way it synthesizes a struct's drop; see the note below

// in ship.vale — user-defined for their owned type
func clone(s &Ship) Ship { /* deep copy */ }   // user's deep-copy
func drop(s Ship) Void { /* destructor */ }    // user's owned destructor
```

**No `clone` for own types by default.** Users write `func copy(&Ship) Ship` themselves if they want an "entirely new instance" verb; the language doesn't blanket-copy classes. `clone` means "get another handle to this thing" — for borrows that's the ref, for share that's an RC bump, for owns the user must opt in.

**►► THE SHARE CLONE IS NOT A WRITTEN FUNCTION — RATIFIED ◄◄** The bound shape is plain
`clone<T>(x &T) T` with `mut(E)`, and it has two satisfiers: at class kind the **compiler-synthesized**
claim clone — a real function performing the inc, the way a struct's `drop` is synthesized — and at
struct kind the user's hand-written deep copy. A rune holds whatever unification hands it
(reference-model decision 4), so at a claim the bound already takes `&@Ship` and returns `@Ship`;
there is no second blanket function and `@` never appears in a user-written clone. `@`'s own
mechanics are under the `@T` restoration item in the build queue.

**►► ORDERING IS WHAT KEEPS THIS APART FROM A USER `clone`, AND IT IS THE ONLY MECHANISM ◄◄** The
ordered union searches the strong-ref namespace before the payload's, which is what keeps the
compiler-synthesized claim clone and a user's `clone(&Ship) Ship` from colliding. Wrap-depth
distinctness does **not** back it up: `@` normalizes to identity at struct kind, so the claim-flavored
signature at a struct is the same depth as the user's. Treat ordering as load-bearing on its own.

**`@` has no surface syntax today.** `BorrowRefSR` / `WeakRefSR` / `OwnRefSR` are live `IRulexSR`
variants; `ShareRefSR` and `ShareRefPT` are absent, leaving two commented-out corpses in
`rune_type_solver.rs`. **`KindT::ShareRef` exists**, so the onion can represent the type — what is
missing is any way to write it. Restoring it is additive rather than a model change: parser
`ShareRefPT` → postparse `ShareRefSR` → an `IRulexSR::ShareRef` variant → a solve arm, whose body is
already specified by its three siblings — bidirectional wrap/peel, no region, identical to the
still-unfilled `WeakRef` and `OwnRef` arms. All three want the same body. The node is minted **only at
rc-class kind**, which is what makes a wrap a conforming implementation of a normalizing `@`.

Related: defect 4 — share upcasts do not work at all, because `convert_via_upcast` rejects wrapped
kinds and reports `CouldntConvertT` for `@Dog → @Animal`. Same family, same missing path.

### Bound resolution

A bound `where exists clone(&T) T` becomes a namespace-scoped exact-match lookup at instantiation time, simulating a call:
- For T=Ship: look in Ship's namespace for `clone(&Ship) Ship`. Find user's. ✓
- For T=&Ship: look in `&T`'s namespace for `clone(&&Ship) &Ship`. Find the borrow blanket. ✓

**Bound resolution does NOT coerce.** It's an exact-shape lookup. Auto-borrow at call sites is for direct-call resolution only; the bound mechanism never coerces.

### Drop is move-only

`drop(bare_local)` is a compile error. Only `drop(^local)` is valid. Auto-drop insertion at scope-end emits `LoadAsP::Move` so the drop call receives the correct ownership. This eliminates the "user thinks they're consuming but the borrow blanket silently no-ops" landmine.

---

## Mission — Replay / FFI design for the own-based world

Partly landed, partly deferred. An `experimental` rebase brought a large chunk of this into the tree: the **no-refcount FFI boundary** (`6978d3639` — refs move/consume across the boundary, no refcount bookkeeping; auto-generated accessors consume their receiver), the deletion of the **Linear region + determinism** machinery (`6978d3639`), the removal of **Fearless FFI** (`cf7ad2c1b` — side-calling stack swap, universal-ref sentinel encryption, orphan generational handles), builtins updated for the no-refcount boundary (`8fffa3681`), and the retirement of the old **record/replay** end-to-end suite in favor of an externs/goldens harness (`18c0e6450`), with **52 FFI tests deferred** on the borrow-shape backend arc (`1ef78718f`). The old `imm`-extern `replay::*` tests no longer exist in that form. The notes below are updated to the post-rebase world; the design direction (scramble+map, move/consume) is still the target, but several mechanisms it named have already changed or been deleted.

### Compile-time-determined FFI representation

C-side dynamic behavior ("does C hold the ref across calls?", "does C mutate?") is NOT discoverable from the Vale side. We instead determine the FFI shape statically from `(exported?, shareability, ownership)`:

| FFI shape | Examples | Replay mechanism |
|---|---|---|
| **By value (bytes)** | primitives, OwnInline (exported value layout) | bytes serialization |
| **By pointer (scrambled + mapped)** | Share, OwnHeap, Borrow, Weak, opaque externs | scramble on Vale→C; int256 → `recordedRefToReplayedRefMap` on C→Vale |

Two FFI mechanisms, mapped from the static type. The typing/lowering pass decides which one applies; the backend follows.

**Ownership at the boundary is move/consume, not refcounted** (landed `6978d3639`). Representation is unchanged — by-pointer values stay opaque handles (scramble+map) — but crossing the boundary *transfers* the reference rather than adjusting a refcount: Vale→C gives the reference up, C→Vale receives it as owned. There is no bump-now/dec-later bookkeeping at the boundary anymore.

> **SUPERSEDING DIRECTION (agreed, NOT implemented):** params come in **pre-+1'd by the caller, externs included** — C receives a *borrow*, never `_dealias`es it, and calls `_alias` only to retain it past the call. `FRMACZ` argues for always-OWN on the grounds of *uniformity with ordinary Vale calls*; under Valen's anchored-borrow ruling a bare class param is no longer a move, so always-OWN now breaks that uniformity rather than preserving it. Returns still transfer. This removes the boundary's most error-prone obligation (a forgotten `_dealias` leaks; under the new rule doing nothing is correct) and needs no new Vale syntax for the "C keeps it" case. **`FRMACZ` and the extern test corpus still describe the old ABI until this lands.**

### Language-level invariant: C can't modify Vale data through pointers

The pointer path stays simple IF C never mutates the bytes behind a Vale pointer. We enforce this as a language rule. In debug mode, scramble (XOR with a per-call key, or replace with poison bytes) at Vale→C; unscramble at C→Vale. Accidental C-side dereference produces garbage; well-behaved C that just stores the pointer is unaffected. This eliminates any need for snapshot-on-pointer-crossing.

### Recording asymmetry

This is design intent to be **rebuilt**, not current code: the recording machinery it names (`mapRefFromRecordingFile`, `determinism.cpp`, the recording files) was deleted with the Linear/determinism removal (`6978d3639`), and the record/replay suite was retired. Treat the shapes below as the target for the eventual replay rebuild, not as live paths.

- **Outgoing (Vale → C)**: nothing recorded for inline values (Vale-side execution is deterministic; replay reproduces the outgoing bytes). For pointers, just apply the scramble.
- **Incoming (C → Vale)**: bytes recorded for inline values (C may have legitimately mutated them via the value param/return path). For pointers, write int256 and map the recorded ref to the live Vale-side ref on replay.

### No out-pointers

C-side mutation of inline values happens **only via by-value params + returns** (or via embedding inside other by-value params/returns). No `Foo*` out-pointers. This keeps the design uniform — "by pointer" always means "opaque handle that C holds but doesn't dereference."

### Implications for the Own split

When Own splits into OwnInline + OwnHeap:
- **OwnInline + exported** → by-value path (bytes)
- **OwnInline + not exported** → structurally impossible at FFI (C can't accept inline bytes without layout)
- **OwnHeap** → by-pointer path (scrambled + mapped) regardless of export-ness
- **Share** → by-pointer path (scramble+map), identity-bearing. Crossing the boundary **moves/consumes** the reference (Vale→C transfers it, C→Vale receives it as owned), not refcount bookkeeping — the no-refcount boundary landed in `6978d3639`. Share still does NOT linearize to bytes; it stays a pointer/handle.

### Replay-test port plan

The old record/replay suite (the 16 `*imm*` + 4 non-imm `replay::*` tests) was **retired** in the rebase (`18c0e6450`), replaced by an externs/goldens harness, and **52 FFI tests were deferred** on the borrow-shape backend arc (`1ef78718f`). So this is no longer a "port the `*imm*` tests" task — those tests are gone. The coverage intent still holds, re-authored against the new harness:

- **Bytes path** (value-data crossing FFI as bytes) — exercised by **OwnInline + exported**, once the OwnInline split lands.
- **Pointer + map path** — exercised by `share` and (eventually) OwnHeap; net-new coverage, since the retired `imm` path linearized rather than mapping.

Both wait on the OwnInline split and the borrow-shape backend arc; the 52 deferred FFI tests are the concrete backlog.

### Backend pre-flight blockers (partly landed in the rebase)

The frontend arc (onion typing) comes first. The `experimental` rebase already resolved or mooted several of these; below is the post-rebase state.

1. **`Backend/src/region/common/primitives.h` Own assertions** — `translatePrimitive` asserted `referenceM->ownership == Ownership::OWN` for Int/Bool/Float/Void. Our Q1 borrow-shape work made it legitimate for primitives to flow non-Own. **Resolution direction (agreed): Option A2** — backend eventually accepts non-Own primitive references with the foundation that primitive borrows will become real LLVM pointers when `*int_ptr = 42`-style semantics land.
   - *Phase 1* (**landed**, per `1ef78718f` "primitives Phase-1"): drop the asserts, always return scalar. The `linear.cpp::translateType` half is **moot** — `linear.cpp` was deleted with the Linear region. `primitives.h` still exists; verify its current assert state against the landed Phase-1 work before treating Phase 1 as fully done.
   - *Phase 2 (when `*int_ptr` lands)*: `translatePrimitive` dispatches on ownership — scalar for Own, pointer for Borrow.
   - **Rejected**: Option B (lower primitives to Own at Rust→C++ FFI in `metal_lowerer::lower_coord_to_reference`) — would actively destroy the borrow-flavor info exactly when the type system needs it. Do not implement.
2. **Linear-region audit for Own input** — **MOOT.** `determinism.cpp` and the entire Linear region + determinism machinery were deleted in `6978d3639`. There is no Linear serialization path left to audit.
3. **Scramble/unscramble helper** (debug-mode) — still pending; scramble+map remains the eventual direction. Not blocking, but enforces the load-bearing "C-can't-mutate-Vale-data" rule.
4. **13 new backend `// VCOORD:` sites** the rebase brought in (from `ebd6f5bec`) flag code that's backwards under the new FFI model — notably `vale.cpp:353` ("every `sharedness == SHARED` gate in this exported-header block is backwards") and `rcimm.cpp` ("do we still encrypt/decrypt?"). These are the concrete backend cleanup list for when the Backend arc starts; `git grep '// VCOORD:' -- 'Backend/*'`.

Under onion typing, the borrow-of-share dispatch (sub-slice-4b) that would have been needed to complete the coherent-collapse route becomes a natural consequence of the onion structure — worth revisiting when the Backend arc starts.

---

## Build / test / verify

```bash
# Library build (fastest check)
cargo check --manifest-path Cargo.toml --lib > tmp/onion-arc.txt 2>&1

# Full test suite
cargo test --manifest-path Cargo.toml --lib --no-fail-fast > tmp/onion-arc.txt 2>&1
grep "test result" tmp/onion-arc.txt | tail -1

# Specific test (fastest for diagnosing one failure)
cargo test --manifest-path Cargo.toml --lib <test_name_substring> --no-fail-fast > tmp/onion-arc.txt 2>&1
```

**Per CLAUDE.md**: pipe build/test output to a single fixed file for the session, not a new file per command. Never chain heavy commands with `| tail` / `| grep` / `| head` — redirect fully, then inspect the file with a separate command.

**Never use `cd FrontendRust && cargo ...`** — always `--manifest-path Cargo.toml`.

**Suite state:** RED — typing is re-linked and mid-slice. PICK UP HERE gives the command that measures it. Before quoting any number, note the traps: `--lib` hides all tests, deleting a dead import raises the count, and a live parse error blanks a file's diagnostics.

## Marker conventions

Don't put in a comment marker (`// ZLOOK`, `// ZHERE`, `// VCOORD`, etc.) unless the architect explicitly says so. There's no difference or convention between them that you need to know about, just put them in where he says.

`// VTRACE: hide` and `// VTRACE: show` are structure rather than work — `hide` on a pass-through function, `show` on a big match, both read by the `collapsed-call-tree` skill. Same permission rule: propose the sites, get them approved, then place them.

## Where to find more context

**Onion-typing arc-specific:**
- `/Volumes/V/Vale2/onion-typing-scouting.md` — 10-investigator scouting map of what has to change across the codebase, organized by subsystem with `file:line` refs.
- `/Volumes/V/Vale2/onion-typing-plan.md` — big-bang plan (17 design gates with provisional leans, 10 areas of change, out-of-scope list). Predates the postparse-planning architectural discoveries above; treat as pre-refinement reference.
- `/Volumes/V/Vale2/postparse-slice-plan.md` — the executed postparse slice plan (variant deletions/renames, scout stage rewrites, solver-side dispatch table updates, sub-commit sequencing, per-sub-slice RFIGA discipline). Also captures the higher_typing collapse + rune-type solver relocation direction (the collapse itself has since executed).
- `/Users/verdagon/.claude/plans/please-plan-out-these-transient-balloon.md` — the parser slice's RFIGA plan (T1-T7 templex, E1-E5 expression, C1-C4 cleanup). Already executed but useful as a template for the postparsing / typing slice plans.
- `/Users/verdagon/.claude/plans/quirky-soaring-summit.md` — the executed higher_typing retirement plan.
- `/Volumes/V/Vale2/FrontendRust/docs/regression-fixtures-from-retired-higher-typing.md` — 6 preserved Vale fixtures + Rust test bodies from the retired higher_typing tests, for eventual re-authoring at typing (3 true gaps: pack literal, empty pack + Prot, plain-param undefined-name; 3 partial gaps: param-position / template-call / recursive-field rune-type-map assertions).

**Reusable mechanical scripts:**
- `/Volumes/V/Vale2/tmp/scripts/onion_typing_import_fix.py` — the safe-script-runner transform used for the typing-side import cleanup. Handles 13 retirement/rename categories on single-line and multi-line `use ...` blocks. Extend the `RETIRED_SYMBOLS` / `RENAMES` dicts to reuse for other retirements.
- `/Volumes/V/Vale2/tmp/scripts/comment_retired_arms.py` — the safe-script-runner transform that commented ~200 retired-variant match arms in typing/. Uses brace tracking to comment the whole arm body (single-line or block). Also extensible via its `RETIRED_PATTERN_TOKENS` list.

**Repo standards:**
- `CLAUDE.md` (project root) — standing rules for this repo.
- `~/.claude/CLAUDE.md` (your user global) — global rules (no `cd && cargo`, etc.).
- `docs/skills/valec-reviewer.md` — reviewer notes (never discard Err payload; no jargon-soup / historical / timeline comments; count-gating rules).
- `Luz/skills/prose-reviewer.md` — comment/prose rules (invariant framing, active voice, front-loading, generalization).
- `src/typing/docs/skills/typing-reviewer.md` — typing-pass reviewer notes.
- `docs/skills/tdd.md` — RFIGA workflow (R/F/I/G/A per slice).
- `docs/skills/diagnose.md` — root-cause protocol.
- `docs/skills/fire-commit.md` — commit + push protocol.

**Design reference (pre-onion, some sections stale):**
- `docs/architecture/bare-clone-borrow-move-design.md` — long-term destination; some parts conflict with onion typing per scouting doc §9.1. Read scouting doc's reconciliation notes before treating as canonical.
- `docs/architecture/instantiator-design.md` / `instantiator_design_2.md` — instantiator/I-IR architecture; both need onion-typing rewrites when the instantiator slice lands.
- `todo/opaque-extern-drop.md` — extern-struct drop design (waits on onion arc + Backend arc).
