# Vale2 Handoff — Onion typing; the typing slice is in progress
<!-- guardian-require-skill: update-handoff -->

**IMPORTANT: Use the /update-handoff skill before editing this file. (Enforced: Guardian's
RSBEX gate denies edits by sessions whose transcript shows the skill was never loaded.)**

**Start here.** The parser, postparse, and higher_typing-retirement slices are landed on `experimental-2`; the **typing slice's structural refactor is done** — `CoordT` / `OwnershipT` / `LocationT` gone from live `typing/`, `KindT` carrying its four ref wraps, one flat `ExpressionTE` whose `result()` is a `KindT`, `SoftLoadTE` dissolved, every node sealed. What remains of onion typing is coercion-layer completeness (`convert` / `is_type_convertible`'s ref rows), interner validity-table enforcement, the `str`/share bare-mention lowering, and retiring the `implicit_clone` probe — work accumulates as TEMP CHECKPOINTs and `git log` is the git shape.

**The RED-slice phase is over: the enabled `--lib` suite passes** (measure with the PICK UP HERE command; read the measurement traps under "Current state" first). The deferred feature families — closures/lambdas, `str`/share lowering, the anonymous-interface macro, and weaks — are `#[ignore]`d rather than failing. `simplifying/`/`hammer`/`final_ast` are deleted; `backend_ffi`, `instantiating`, `testvm` and `integration_tests` are linked in `lib.rs` — the pipeline is wired end-to-end.

**The borrow checker is live** — a two-phase whole-function walk in `src/typing/borrow_checker/` catching use-after-churn and the two joint-argument checks. Rung 0 (groups on the declaration side, `BorrowRefT` emptied of its region) and the group syntax (`&T in g` / `g.items` / `g[]` / `g...`, `<g': T>`, `mut(g)` / `not(mut(g))`) are built.

**Read order for a fresh session:**
0. **"LESSONS LEARNED"** — short, and it will save you an afternoon.
1. **"PICK UP HERE"** — the current state, the uncommitted work, and what Vale4 is blocked on.
2. **"CAPABILITY LADDER"** — where the failures actually are, as first-blocker counts. The build order.
3. **"Current state"** — the tree, the onion surface, and the measurement traps.
4. **"Resolved design decisions"** — the locked model.
5. **"THE COMPLETE OPEN LIST"** — what is genuinely still open, grouped by what unblocks it. Read its own "almost none of this gates the next work" preamble first.
6. **"Upstream rulings"** — the design corpus's answers, and the one place they are written down.

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
- **"Retired" in a design section does not mean the code knows.** The `implicit_clone` probe is retired by ruling and still has a live function, two live error variants, 10 corpus files and ~60 test references. Say which one you mean.
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
- **A partial set with a total name gets its fold forgotten.** `header_rules` omits each param's own type-binding rules; all five reader sites in `function_compiler_solving_layer.rs` now fold them in (it was once three of five) — check every reader when a field's name promises more than it holds.
- **A failure-set diff is meaningless if the test binary did not compile.** Confirm a `test result` line exists before comparing runs, or a broken test build reads as every test suddenly passing.
- **Lambdas do not generally use the anonymous interface macro.** Only a lambda passed where an *interface* is expected does; a direct lambda call, or a lambda/struct satisfying a concept-function bound (`__call(&G)E`), does not — so a failing lambda test is usually not an anonymous-substruct failure.
- **In a borrow-checker fixture, a move is `^local` (prefix), and arithmetic on a borrowed member does not read out.** `f(&x, x)` does not move `x` (bare `x` yields a borrow, rejected against an owned param) and `x^` does not parse (postfix `^` unshipped); write `^x`. `set a.hp = a.hp + 1` on an `&Entity` member fails with `+(&i32, &i32)` not found, so fixtures use literal member writes.
- **Group invalidation is not Rust's exclusion; do not model the borrow as a lock.** A borrow constrains nobody. A *destructive* op on an **ancestor** group invalidates references into its **child** groups (downward only); a plain member write, a sibling, or mutating the reference's own contents invalidates nothing. The destroy is the aggressor and the borrow is the victim — the reverse of a Rust `&mut`, which is a standing lock its place must respect. Rung 2 implements exactly this: a `mut(g)` call invalidates only child-group (array-element) references rooted in the churned array.
- **`...` is the descendant group operator (`&T in g...`), not a comment.** The lexer's `consume_ellipses_comments` was removed; do not resurrect `...`-as-comment. Three dots lex as three `.` symbols, consumed only by `parse_group` (`templex_parser.rs`) in group position.
- **A use-after-churn needs a child group, and an inline-only plain struct forms none** (`group-borrowing.vmd`). A child group comes only from an independently-destroyable owned thing — a collection/array element, a `Box` pointee, a `Variant`/interface payload — never an inline scalar or struct field, nor a whole-array reference. So `struct Fleet { flagship Ship; }` has nothing a churn can dangle; do not try to write a rung-2 test against it. Rung 2 lands on a runtime-sized-array element for this reason; the "rungs 0-2 need no `Vec`, plain structs suffice" framing was wrong.
- **A runtime-sized-array local can now drop cleanly** via a closure-free `DropFunctor<T>` in `arrays.vale`; do not resurrect the closure-based array drop (closures are not working yet). Before this, every RSA test dodged block-end drop by erroring earlier, so a clean array program did not compile.
- **Emptying a struct field reaches past its constructors, through its type.** Removing `BorrowRefT.region` (a `RegionT`) meant chasing `RegionT` through the humanizer's display params (`humanize_id`/`kind`/`name`/`generic_args`, each carrying an `Option<RegionT>`) and out of the `IdT` name-structs (`ExportNameT`). Grep the field's *type*, not just its name, to bound the sweep.
- **A `--lib` build passing does not mean the tests build.** `typing/mod.rs` gates `pub mod test;` behind `#[cfg(test)]`, so a change can compile under `cargo build --lib` yet break the test build (typing's macros construct `ParameterS`/`FunctionS`, its tests construct `ExportNameT`). Run `cargo test --lib --no-run` before calling a typing-touching change done.
- **"It compiles" does not prove a syntax was captured.** `&T in g[]` compiled clean while `parse_group` silently dropped the `[]` and produced a plain `Rune` group; only a parser/postparse unit test asserting the scouted `GroupS`/AST shape caught it. Assert the shape, not just that a fixture compiles.
- **A `todo!`'s comment can name the wrong cause.** `make_kind_g`'s arm labelled "position rule (`rc`, class tier)" was in fact hit only by ordinary generics instantiated with a reference (written type a bare rune, `ITypeST::Rune`), never by a class. Log the actual `(KindT, ITypeST)` shapes reaching a `todo!` across the corpus before trusting its label.
- **To split a class of derivation failures by root cause, panic on the bad state and census the panic's caller frame.** When groupify began panicking on any underivable borrow, grepping the failing suite's backtraces for the panicking helper (`member_result` vs `call_result_kind`) partitioned every failure into closure-capture vs optional-borrow-return in one run.
- **`cargo test --lib` can report failures a fresh rebuild doesn't — incremental staleness, not a flaky test.** One run showed 42 failed / 429 ignored; a `touch`-and-rebuild (and `cargo nextest run`) then showed 0 failed / 471 ignored, stably across 7 runs. The tell: the ignored count matched the source `#[ignore]` count exactly (`grep -rEc '#\[ignore' src`), so the stale binary had been running tests the source marks ignored. `nextest` (a process per test) sidesteps it; when a suite result surprises you, force a rebuild before trusting or acting on it.
- **The interop lane is gated on `rust_interop/**` changes, so a commit that doesn't touch that path never runs it — even when it breaks it.** Removing the `function_scout` group-param strip let group templatas reach the instantiator, whose `Group` arms were missing; native + wasi stayed green (they don't build the instantiator/interop) and the gap landed latent, caught only when an interop test rebased onto it. A change that alters what flows *through* the pass (group params, new templata shapes) can break the instantiator/interop without touching `rust_interop/**`; run the interop lane by hand for such changes.

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
- **Authorize removals category-by-category.** The architect clears one kind of edit at a time (*"just that, no other kinds of typing pass edits"*); surface the boundary of a cascade and wait for the go-ahead rather than bundling the next category in.
- **Only three kinds of work: a simplifying refactor, a new feature driven by a red test, or a bugfix driven by a red test.** Nothing else — no speculative plumbing, no "cheaper to do it now before it's needed". A "cheaper to do it now, before the checker depends on it" argument does not clear the bar; the core stays untouched until a red test proves the change is needed. (Minting `GroupB` stays deferred on this bar — no observable behavior to red-test yet. Source ranges cleared it once the borrow-check diagnostic needed them, and landed.)
- **Do not preserve backwards compatibility or maintain unused code.** Code as if there are no users: a superseded function whose only callers are its own recursion is dead — delete it, don't keep it working alongside its replacement.
- **Do not mirror a foreign reference implementation's structure as a template.** Copying Polonius's file/struct layout would import loans/origins/constraints — abstractions for jobs (region inference, exclusivity) group borrowing does not have; write a small own-shape layering doc instead.
- **No `Option` return or struct field, and no fallback / default / graceful-degradation, without explicit human sign-off** — a `// VOPT:` / `// VFALLBACK:` marker on the site, or a mention in a `*-design.md` `## Design (human-only)` section. "The data might not exist" is not a reason for an `Option`; make the data exist (or panic on the impossible). A two-state enum is *not* an acceptable substitute for a banned `Option`.

**Recurring agent mistakes**

- **Reasoning from current code as though it were the target.** In a mid-migration tree most of what you read is the thing being migrated *away from*. Say which one you are describing; when they disagree, the ruling is the target.
- **Promoting a reading of the implementation to a refutation of the architecture.** This once marked the ratified call-site phase model as "WRONG" in this file and cost an afternoon re-deriving it.
- **Treating one file's silence as the project's silence.** Grepping only this handoff missed a design that was ratified in the convo logs.
- **Writing corrections backward.** "We thought X and were wrong" is noise to a reader who never held X; state the trap forward.
- **Estimating from headings instead of reading.** A deadweight estimate made by skimming section titles was five times too aggressive — the day logs were invariants, not events.

**Structural**

- **More than one "what to do next" section means all but one are wrong.** The PICK-UP block is current; if it and the Horizon Plan or the open list disagree, it wins.

**This handoff covers the Onion typing mission.** Two related concerns are out of scope here and tracked
separately: the overload-resolution / dispatch-model redesign (still applies, not started) and the replay /
FFI design (deferred — the borrow-shape FFI semantics on top of the already-wired backend).

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
> **The design-1 audit has been run** and its findings are **folded into this file**; its separate doc is gone. It covered decisions 1–15, the reference-model decisions, and the coercion table. What survives is the **audit method** — three false positives of one shape, plus the dating discipline — under "Run the design-2 audit". The ruled-but-not-yet-built gap inventory is tracked separately.
>
> **Still unaudited: `valen-design-2.md`** (RC / multi / class tier). Decisions 1 (weak's shape) and 2 (share's shape) are design-2 territory and were **not** checked. A second pass is owed.

## ►► PICK UP HERE ◄◄

**Measure before quoting any count.** `cargo test --manifest-path Cargo.toml --lib
--no-fail-fast`, then census first blockers with
`grep -o "panicked at src/typing/[a-z_/]*\.rs:[0-9]*" <file> | sort | uniq -c | sort -rn` — the census
says where the work is, and it moves far more than the total does.

**►► THE CALL-SITE DESIGN LIVES IN `docs/plans/plan-phased-calls.md` ◄◄** That is the current,
authoritative 8-phase model. It retires the sends machinery (an `ArgumentStep` matches the argument
`KindT` against the parameter's `ITypeST`), the rune-type solver, and `complex_solve`; the
reject-the-losing-candidate solver arms (`KindIsNotBorrowRef` and kin) become ordinary match failures
under it.

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
git log --oneline -1 exp-1-wipbx exp-2-wipbx exp-3-wipbx exp-4-wipbx
git log --oneline exp-2-wipbx..exp-4-wipbx     # Vale4 commits we have not absorbed
```

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

**`vale-rust-interop-architecture.md` lives in our tree** at `docs/architecture/`, so fixing
the *"interface" means two different kinds* vocabulary collision it carries is **ours**, not
upstream's. Any citation of it as a `Vale4/…` path is wrong. (There is no `docs/convos/rust_interop/`
subtree; the two `convo-5-…` files in `docs/convos/` are the base and `-plan` halves of one conversation.)

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
2. **`UpcastTE::new` is filled** (`replace_value_type_in_ref` over the inner expression's type, where
   both of Vale4's interop tests sit); the only remaining upcast `unimplemented!` is the zero-caller
   `InterfaceToInterfaceUpcastTE::new`.
3. **Export/extern boundary** — the `is_primitive` rename plus `peel_all_references` at both the
   check and the map lookup. Vale4's other front, waiting on the naming decision.
4. **The defect inventory.** Several are one-liners.
5. **The `implicit_clone` probe deletion** — retired by ruling, still live in the code. See "What
   blocks / what to preserve" for its extent.

**The declaration side must peel wherever the call side does.**
`evaluate_maybe_virtuality`, `ssa_len`, `rsa_pop` and `as_subtype` each matched a bare kind against a
parameter that arrives wrapped; `peel_all_references` for reading a type and
`replace_value_type_in_ref` for rebuilding one are the two helpers this keeps wanting.

#### MEDIUM TERM — re-link what the arc unlinked, and pay the migration debt

Goal: **the compiler is whole again** end to end, and the corpus stops lying about the language.

- **`integration_tests/` is linked** — `pub mod integration_tests;` is active; only its `#[cfg(test)]`
  gate is commented, so it compiles its tests under `cargo test`. It carries the generic-virtuals
  fixtures — Milano / Serenity / Raza / Enterprise — the *only* exercise of the override-dispatch
  machinery.
- **`simplifying/` and `final_ast` are already deleted** — the planned direction (delete
  `simplifying/`/`hammer` and `ProgramH`/`final_ast`, emit onion `HinputsI` straight to the backend) has
  landed; the instantiator already emits `HinputsI` as the sole backend contract. What remains is
  `instantiating/` itself, which is active and matches typing's live `ExpressionTE::While/Return/Break`
  (the `ReferenceExpressionTE` enum it was once said to match has zero hits under `typing/`).
- **Tier 1 of the syntax migration** — `^` postfix, `own`→`ownref`, and
  `#!DeriveStructDrop`→`#explicitly_destroyed`. All three are measured and ready.
- **The `&&`-as-weak corpus sites** — the compiler cannot drive this migration, because `&&T`
  stays a legal type and nothing errors. Needs the hand-list.
- **Source ranges on AST nodes — DONE** (`acb43c66`, defect 15 closed). Every `ExpressionTE` variant
  now carries a `range: RangeS`, populated at every construction site, so a borrow-check diagnostic can
  point at the exact node. The borrow-check underivable diagnostic is what motivated it.

#### LONG TERM — the borrow checker, and the FFI semantics

Goal: **the language's actual point.** This is where the design work cashes out. (The pipeline is
already wired end-to-end — parser → postparse → typing → instantiator → backend — so there is no
backend *arc* to stand up, only FFI-semantics and borrow-checker work layered on it.)

- **Rung 0 (groups become real), rung 1's joint-argument check, and rung 2's use-after-churn are
  landed.** Groups live **only on the declaration side**, never on the value type: `BorrowRefT` emptied
  to `{ inner }`; group/effect syntax in `GroupP`/`GroupS` enums (`GroupB` not yet minted) + the minimal `mut(g)` clause;
  a ceremonial `ITemplataT::Group(GroupTemplataT {})` constant for the uniform group param (minted in
  `create_placeholder`). The checker reads groups off the scout `FunctionS` and the written `ITypeST`,
  never off a `KindT`, and builds the group-annotated `KindGT` mirror (`make_kind_g`) rather than
  comparing group names. Use-after-churn of a child group (a runtime-sized-array element, or one
  reached through a member) across a `mut(g)` call is caught, with `if`-join union and `while` loop
  pre-seeding; the scope is in `docs/plans/path-to-borrowing.md`. This **supersedes** the earlier "add
  `ITemplataT::Region`, make `RegionT` an interned recursive algebra, put a real region on `BorrowRefT`"
  scoping — do not resurrect it.
- **Rung 1's effect *checking*, and more child-group sources** — extend child-group sources (`Box`,
  `Variant`/interface, struct-field arrays) and then generic `Vec<T>`, plus the union path
  grammar for precise cross-call invalidation. Both borrow entry points are confirmed and landed:
  borrow *creation* at the member/element seam (rung 2's element refs), and the joint-argument check
  at call sites (rung 1). Rung 3 (returned-reference use-after-churn) is landed.
- **Effects** — strictly behind rung 0, because an effect target *is* a group expression. The
  representation is still unsettled (see the effect-representation block); the live candidate is a
  per-group permission map, since `held` and `dangle` are ratified permission splits that a bare
  group cannot hold.
- **The FFI semantics on the wired backend** — `Backend/` C++ already walks the onion; what remains is
  the pre-+1'd FFI boundary and the ~50 deferred FFI tests (measure the
  `#[ignore = "deferred: borrow-shape backend arc …"]` set in
  `end_to_end_tests/tests/externs.rs` before quoting). `FRMACZ` still documents the superseded always-OWN ABI.

#### The one thing that would most change this plan

**Rung 0**, now underway. Everything in the long-term horizon sits behind it. Its representation and
seam are landed; what remains gating the rest is the typing-side semantics (the group constant is already
minted; mint `GroupB`) plus the rung-1 effect-domain decisions. Short and medium term proceed independently.

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
| **Upcasting** | `convert_via_upcast`, `convert_helper.rs` | `UpcastTE::new` is filled (`replace_value_type_in_ref` over the inner expression's type); the remaining `unimplemented!` is the zero-caller `InterfaceToInterfaceUpcastTE::new`. Share upcasts still fail (defect 4). Where both of Vale4's interop tests sit |

**Do not plan against generic bounds, applied generics, placeholder substitution, member access,
abstract bodies, or generic drop.** The `KindList` arm, the template-position scout defect,
`substitute_templatas_in_kind`, `dot_borrow`, `generate_function_body_abstract_body` and `drop`'s
citizen arms were each the biggest cluster on the board in turn, and all now fail **zero** tests.
Clearing a cluster mostly moves its tests to the next blocker, so distrust any count as a measure of
what a fix buys.

**Parse failures are effectively cleared** — the `can_turn_a_borrow_coord_into_an_owning_coord` test
once cited as the last survivor does not exist in the tree. The bucket was never "error-handling stubs": it was retired syntax
the corpus had not caught up with, drained in four waves where each revealed the next — a builtin's
`Ref` rune type, then `Ref`/`Kind` annotations tree-wide, then `^`-in-type-position and `[#N]T`, then
two retired where-clause builtins.

**Move tracking across branches is not its own capability** — `if_branches_must_move_same_variables`
and its sibling sit inside the member-access 31, which is why they have never reached the join.

**Almost nothing on the open-questions list gates this ladder.** Member access needs only the
no decision at all; bounds needs nothing from the region/effect design; export/extern needs the
`is_primitive` rename plus `peel_all_references`. Every open *design* item gates **rung 0 and
beyond**, which sits behind all of it.

### Ready to start (no decision needed)

1. **Add the `*` deref operator** — ruled, small, and nothing is blocked on it, so it can slot in whenever. Parser (we have no `*` prefix operator at all) + postparse node + the two-depth `set` distinction. **Our decisions 2 and 13 already have the shape**: a lookup yields the address of the slot and the read-path `DerefTE` peels exactly one storage layer, so `k` is the stored reference and `*k` is simply *one more peel*; `set k = …` targets the raw `&&T` address-of-slot while `set *k = …` targets the `&T`. That is a parser addition plus a `DerefTE` at a new site, not a model change.

2. **Work the DEFECT INVENTORY** — a list of things that are actually broken, each with cited evidence. See the defect-inventory section below. Highest-value entries, roughly in order:
    - **The export gate panics on `exported func moo(firefly &Firefly)`** — an ordinary borrow param — and needs `peel_all_references` before both the `is_primitive` call *and* the map lookup, not just filled arms.
    - **A latent if-join assert-failure, now reachable since member access compiles.**
    - **Share upcasts don't work at all**, and **`inner_find_reachable_allocations`** in `testvm/heap.rs` is missing three arms.
    - **`SharedImplingMismatch` now exists and is enforced** (`impl_compiler.rs` errors on a sub/super sharedness mismatch), so the invariant `look_for_override` depends on is checked — no longer a gap.
    - **Dead and verified**: `initially_known_runes` (safe, with an error-ordering caveat), `PrimitiveRuneTypeSolverLookupResult`, three warnings, the `lookup_rune_type` coercion (confirm by `panic!`-probe first; deleting its enum variants is a compile break).

3. **Member access is built, shape B, and `dot_borrow` no longer exists** — the `Dot` and `Index` arms
   of `expression_compiler.rs` each check that the container is a place and then peel with
   `peel_all_references` *only for matching*, leaving the expression's own wraps intact. Two arms are
   still stubbed there: a weak or placeholder container is a compile error, and a bare kind is an
   rvalue by construction (decision 7) wanting `make_temporary_local_borrow`, coercion row 7.

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
2. **The confirmed alignment items** — no-shadowing enforcement (we have none), `Vec`/`List` tier split, the `Vec<int>`-elements-are-a-child-group pin, and `comptime`. The attribute rewrite is tier 1 of the syntax migration and targets **`#explicitly_destroyed`**, not `#derive(StructDrop)`.

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
> **compiler-synthesized rather than written**. **The `>1 → ambiguity` branch stays live** as the
> cross-namespace backstop — do not delete it as unreachable.
>
> **And:** **a strong ref contributes its payload's
> namespace too, as an ordered union** (Rust's arbitrary-self-types shape, automatic for strong refs
> only, `Box<T>` auto-derefs as Rust does) — which **answers "does `&Ship` mention `Ship`"**: yes, and
> a strong ref mentions its payload as well. **Generic bounds** (`solve_rule`'s `KindList` arm) and **applied
> generics** (the template-position scout defect) both fail zero tests and are no longer clusters.

**Decisions only the architect can make**
5. **`is_primitive`** — the fix is *renaming*, not moving the `Str` row. Which two names?
6. **The three override mismatches** — make the abstract bare (matching its four siblings) or borrow the overrides? Lean exact-match: Valen refused variance deliberately. **Note the diagnosis changes once position-dependence lands** — today it's borrow-vs-strong-ref; after, it's borrow-of-payload vs borrow-of-claim.
7. **Tier 1 — go?** All three parts are ready and measured: `^`, `own`, and
   `#!DeriveStructDrop`→`#explicitly_destroyed`.
8. Smaller: delete the 3 removable warnings; keep or delete `evaluate_addressible_lookup_for_mutate`'s shadowed `IVariableT::Capture` arm, which holds un-ported Scala; when to rewrite the `&&`-as-weak corpus sites (compiler can't drive it).

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
lookup, so `x @Ship` should still search `Ship`'s namespace for callable methods. `split_type_st_into`
(`translate_signature_type_st`'s recursive peeler) already routes every outer ref layer to
`type_outer_ref_rules`, so a restored `ShareRefSR` handled there as one more ref wrap lands in the
outer-refs bucket by construction.

**Doc work** — fix the *"interface" means two kinds* vocabulary collision in `docs/architecture/vale-rust-interop-architecture.md`, **now ours**.

**Closed, recorded so nobody re-opens them:** the `set` spec report (sent + ruled); the Vale4 reply (sent); the Luz curate queue (drained + committed); `Guardian/Luz` (deleted, 7 symlinks deliberately broken); the design-1 audit doc (folded into this file + deleted); the `lookup_rune_type` coercion arm (**dead by evidence** — probe run, suite byte-identical, zero hits); the move-tracker join question; the `self`-receiver hazard; the borrow-creation seam; per-body checking; and every one of the seven "genuinely unfinished reasoning" threads from the start of the arc.

## Current state

The onion reference surface is in the parser and postparse, and **the typing slice compiles** — the
compile cascade that defined this arc is finished. What remains is filling stubs, not chasing type
errors. Everything outside typing is green: parser, lexer, postparse, solver, utils, humanizers.

**`simplifying/`/`hammer`/`final_ast` are deleted; `backend_ffi`, `instantiating`, `testvm` and
`integration_tests` are linked in `lib.rs`** — the pipeline is wired end-to-end. (Verify the `valec`
bin's own build state before quoting it; check `grep -n "pub mod" src/lib.rs`.)

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
- **`heap` is dropped entirely** — the `heap T` surface path (`parse_ref_prefix` handles only
  weak/own/held/&). There is no value-model `HeapOwnRefT` in typing — `KindT` has only
  `BorrowRef`/`OwnRef`/`ShareRef`/`WeakRef`.
- **`borrow` is gone entirely** (not registered, not consumed). **`share` is not a registered keyword**
  either, but is still a live lexer token — the literal `share` in a citizen header produces
  `SharednessP::Shared`. Only `own`/`weak`/`held` are registered `Keyword` fields.
- **`held` is a region**: `RegionSR` = `Unspecified | Held | Rune`, but `RegionP` = `Unspecified | Held | Group`.

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
- **`TypingIgnoredParamNameI` sits dead in the active instantiator** — defined and referenced only in
  enum declarations, never constructed anywhere in `src/`.
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
- **Placeholder sharedness was removed** (and `lookup_mutability` with it). `get_sharedness` survives but
  has no live callers today. Citizen sharedness is read via `lookup_struct(...).sharedness` /
  `illuminate_type(...).sharedness` — there is no `declare_type_sharedness` (only a TODO comment).

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

**`is_type_convertible` is wrong, not merely incomplete.** For `&X→X` it returns `false` for
non-cloneable inners (where `convert()` succeeds), and `X→&X` returns `false` too (its panic is commented
out) — the only live panic is the catch-all `_` arm for genuinely unknown pairs. Its
reference arms cover a both-borrow-refs recursion and a primitive `&P→P` read-out; the recurse guard
**wrongly accepts `&&X→&X`**, which is harmless only until genuine double-borrows exist. The real fix
is aligning with `convert()`'s coercion-table arms, or driving it off a dry-run `convert()`.

**►► In the current code `is_type_convertible` still performs BOTH overload jobs** — candidate/membership
param-matching (`overload_resolver.rs`, the non-exact branch, returning `SpecificParamDoesntSend`) and the
exact-vs-coercion per-param bool vector. The ratified design retires both (filter-is-final, purely static,
so it never asks "does this convert?"), leaving only phase 4's real conversion and the gate-checks — but
that is **not yet built**, so today the predicate is load-bearing.

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
- `#!DeriveAnonymousSubstruct` → no Valen analogue. One site. Ours to resolve.

**TRAPS AND CORRECTIONS — each of these would have cost us:**

- **`T: Drop` does not detect every type with a drop.** design-1:1668 — Valen has **two spellings of `drop`**, and only the `impl Drop` form satisfies the bound; the free-function form (`func drop(self: File)`) does not, yet **both give the type a drop**. So a `#derive(Copy)` gate written as *"does this satisfy `T: Drop`?"* — the natural shape, and the one our bound machinery already points at — **passes `File { fd: int }` straight through**, on design-1's own example. Check for a `drop` **function**, never for the bound. Rust has no analogue, so Rust instinct actively misleads here.
  - **Both spellings make the type *affine*, not linear** — design-1:2082, *"Both run the destructor at scope end and so make the type **affine**."* The vocabulary is easy to invert here because the surrounding intuition ("a drop means you must be careful") pulls the wrong way; **drop *absence* is what creates a linear obligation.**
- **The interop doc's "interface" is NOT design-2's `interface`.** `docs/architecture/vale-rust-interop-architecture.md` — **now in OUR tree**, and cited by design-1:2554 as the FFI authority — uses Vale-era vocabulary where "interface" means design-1's `trait` / `open trait`. **Since it's ours now, fixing the vocabulary is our job, not upstream's.** So interop-doc "interface" **projects** to Rust (sealed ones as enum + sealed trait), while design-2 `interface` (class tier, RC-erased) gets **no projection at all** — Rust holds an opaque handle and calls through it (design-2:833, *"This is the intended answer, not an unfilled gap"*). Live on the `tests_exporting_interface` front: a test asserting a real `dyn` for an interface is asserting the wrong thing.
- **`Copy` × linear is still open — fail closed.** design-1:1801's *"linear is orthogonal to Copy"* stands per the architect, but `File { fd: int }` (all fields `Copy`; the obligation comes from the `drop` *function*, which can reach an fd or a syscall) shows an all-fields-`Copy` condition does not exclude a double close. Narrow question routed: **does `#derive(Copy)` reject a type with a `drop`?** The by-move-consumer quadrant (`Future<T>`, no `drop`) may get a different answer — its double-await is harmless exactly when every field is `Copy`. **Reject in both cases meanwhile, marked provisional in-source so it can't harden into a claim.**
- **`.` performs receiver adjustment.** Our dispatch model's claim that dot is *pure* sugar is refuted by the corpus: `keys.append(*k)` (owned receiver, `&self mut` method) requires an autoref, and `set self.hp -= 10` requires a deref. The adjustment is on the **receiver** `keys`; the argument's `*` is a separate rule (arguments do not adjust). What survives is the **namespace** half — dot doesn't change *which* functions are findable, so the no-Self-specialness dispatch rule stands. The repair is one clause: **`.` is sugar for the free-function form after a receiver adjustment.**

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

**Still open upstream:** three questions remain (the `mut(E)` sort, the minted-anchor release charge,
and the found-anchor certificate), tracked with the region/effect work.

**Answered, and each closes something we'd been carrying:**
- **Our namespace/dispatch model is NOT a divergence — it's unsettled upstream.** design-1 says *nothing* about how a candidate set is assembled at a call site; the only nearby item is design-1:2563 listing **module and import syntax as an open question**, while visibility (`pub(crate)`/`pub(super)`) presupposes a module system that hasn't been designed. **Flag for later: when Valen decides modules, that is the moment to compare** — a file-based namespace rule and a module system are hard to retrofit against each other. Supporting our sub-assumption that `&Ship` and `Ship` are distinct: design-1:989 gives reference and box types **their own impls** (`impl SomeTrait for &dyn OtherTrait<...>`), so they aren't aliases of the pointee.
- **EFFECTS NEED A CHECKING PASS, NOT AN INFERENCE PASS.** design-1:1265's *"unifies **declared** effects with **derived** effects"* is a **check of the body against the signature** — you cannot unify a declaration that doesn't exist — and design-1:850's *"a method with no effect clause has no external effects"* means absence **asserts** purity. *"Valen infers purity"* is about **spelling** (you never write `pure`), not about deriving an omitted clause. Consequences: a named function with no clause whose body mutates is a **compile error**; **no call-graph fixpoint is needed**, since a recursive call checks against the callee's *declared* clause. **Closures are the exception and genuinely infer** (design-1:1604), and **a recursive closure would need a fixpoint — the doc does not address it. Genuine gap; record rather than assume.** So the build is: a checking pass for named functions + a computation for closures + the `E: Effects` solver with Pin C's re-check discipline. Materially smaller than first sized.
- **Override parameter shapes: unspecified upstream, but lean EXACT MATCH.** design-1:884 governs *effects* only (impls may narrow, not widen); there is no parallel rule for reference modes or returns. The structural reason to require exact match: **Valen has no variance anywhere and refused it deliberately** — `concerns/soundness/02:21` records that *"'variance'/'invariant'/'covariant' appear nowhere"*, and the disposition was *"banning cross-instantiation subtyping outright."* A narrowing override would be the language's **first** instance of variance, introduced at a corner rather than as a decision.

**Also dissolved, and worth not re-raising:** whether `c.clone()` on a class claim deep-copies the payload. It never arises — **a bare mention already copies a claim** (rubric:112, *"class refs auto-copy (inc/share)"*; design-2:880 for captures), so a rung-1 user writes `d = c` and never reaches for `.clone()`. design-1:1332's contrary spelling is the pre-C1 residue described above; its *substantive* point survives (a claim-bearing **slot** can be copied from, a payload **borrow** cannot — the `graveyard.append(u)` rejection).

## Defect inventory
Grouped by what you would do with them. **Everything here is traced from source and cited**; where something could not be settled by reading, that is stated rather than guessed.

### Broken today

1. **The export/extern gate silently accepts an ordinary borrow parameter** (it no longer panics). `is_primitive` (`typing/compiler.rs`) now returns `true` for `KindT::BorrowRef | OwnRef | ShareRef | WeakRef`, so `ensure_deep_exports`' `!is_primitive(param) && !exported_kind_to_export.contains(...)` check short-circuits for a `&Firefly` param — Firefly is never required to be exported, and nothing panics. The `unimplemented!()` ref-wrap arms survive in `is_descendant_kind` (dead — its only caller is commented out) and inside `ensure_deep_exports` itself, but neither fires for a plain borrow parameter; they fire only when an *exported kind* is a ref-wrap. Whether silently accepting is correct is the open question now, not the panic.
2. **A latent assert-failure at the if-join, now reachable.** The `If` arm of `evaluate_expression` diffs `nenv.snapshot(...)` against *itself* — a botched transcription of a Scala "if block env" that no longer exists. So the restackified result is the entire current restackified set, the arm re-marks each, and `mark_local_restackified` (`env/function_environment_t.rs`) opens with `assert!(!contains(...))`. **Any `if` compiled while a local is restackified should assert-fail.**
3. **Not a defect: dropping a `Str` is implemented.** The `Str` arm in `destructor_compiler.rs` emits `ExpressionTE::Discard(...)` (the same discard treatment as the primitive/ref arms), under the comment "Discard here will drop the reference count." — not `unimplemented!()`. Unlike the citizen arms it resolves no named `drop` function.
4. **Share upcasts don't work at all.** `convert_via_upcast` (`typing/convert_helper.rs`) calls `ISubKindTT::try_from`, which rejects wrapped kinds. `convert()`'s borrow path passes peeled citizens (fine); the no-borrow path passes the full type, so `@Dog → @Animal` arrives as `ShareRef(...)`, `try_from` fails, and it reports **`CouldntConvertT` instead of upcasting**. Consequence: **the 18 class-tier interfaces are probably not a working baseline** — check before treating them as one.
5. **`inner_find_reachable_allocations` (`testvm/heap.rs`) is missing THREE arms, not one.** `KindV` has 8 variants and it handles 5, so **`Str`, `Opaque`, AND `ArrayInstance`** hit a bare `panic!()`. Latent (one call path, one test, empty members) — but the `ArrayInstance` gap means the leak check would panic on any array-rooted heap before `str` even enters. Fix as one arm-set.
6. **A move from inside a `while` whose body never falls through goes unreported** — the *only* real move-tracker defect. The move check in `evaluate_expression` is gated on `match body.result { KindT::Never(_) => {} .. }`, so a body ending in `break`/`return` skips it and an illegal move of an outer local is silently accepted. Captured by the ignored test `reports_when_moving_from_inside_a_while_that_never_falls_through` (`typing/test/compiler_tests.rs`); the sibling `reports_when_moving_from_inside_a_while` (no break) still errors correctly. **The fix drops the `Never` guard so the check runs regardless of the body's result — deferred until a borrow-checker feature needs it (test-first ruling), not fixed speculatively.**
7. **Not a defect: the `Block` arm's move propagation is correct.** The report of a missing `continues` guard (a bare block ending in `return`/`break` pushing marks upward) does not reproduce — do not re-file it from reading the arm.
8. **`&self` is a parse-level stub.** The parser recognizes it (a two-token lookahead in `pattern_parser.rs` setting `self_borrow`), but postparsing builds a **rule-free `ImplicitRune`** — nothing ties it to the enclosing citizen and **the borrow-ness is discarded**. Related: bare `self` inside a citizen body **panics** in `function_scout.rs` with `POSTPARSER_SCOUT_FUNCTION_PARAM_TYPE_REQUIRED_NOT_YET_IMPLEMENTED`, contradicting a stale parser comment claiming it defaults to the containing struct. **And there is essentially no `Self` type to desugar to** — the only `"Self"` string in `src` is a filter in `rust_interop/tyctxt_oracle.rs` (dropping Rust's implicit trait `Self` generic param); `SelfRuneS` is dead, `SelfFullTypeRuneS` is macro-only, and `SelfNameS` is a *variable* name that never meets a user-written `CodeVarName("self")`. **Sequencing: implementing `&self` requires inventing `Self` first.**
9. **`SharedImplingMismatch` exists and is enforced.** `impl_compiler.rs` computes the sub-citizen and super-interface sharedness across an impl and returns `ICompileErrorT::SharedImplingMismatch` on mismatch (declared in `compiler_error_reporter.rs`, humanized in `compiler_error_humanizer.rs`, and asserted by `after_regions_error_tests.rs`). The older `WeakableImplingMismatch` is gone (weakability was folded into sharedness). So **`look_for_override`'s shape-copy rests on a checked invariant, not a bare convention** — this defect is resolved.
10. **`stdlib/src/ifunction/ifunction1.vale`** reads `interface IFunction1<M Mutability, P1 Ref, R Ref> M {` — the trailing `M` sits in the sharedness slot but is a **generic Mutability parameter** from pre-migration syntax; the lexer accepts only the literal `share` there, so **the file is unbuildable as written**. It is also the **only trace anywhere that sharedness was once template-parametric**, which would contradict the parse-time-known assumption in `struct_compiler.rs`. (`stdlib/src/str.vale` uses the retired `imm` sharedness spelling, not this Mutability-parametric shape.) (This is also the `IFunction1` with **no implementors** and an arity mismatch against its local twin under `tests/ifunction/`.)

### More defects (numbering continues the inventory)

11. **Not a defect: the `BorrowRef` peel writes the correct rune.** In `solve_rule` (`typing/infer/compiler_solver.rs`), the result-known arm concludes into `inner_rune` and the inner-known arm into `result_rune`; on a `commit_step` error it wraps `InternalSolverError`, not an unimplemented panic. `get_puzzles` gives `BorrowRef` both directions. The peel-to-wrong-rune bug once reported here does not exist in the current code — the mechanism phase 0 depends on is in place.
12. **Not a defect: the `While` arm's move propagation is correct.** The report that a loop-body move never propagates outward (the arm not calling `mark_local_unstackified`) was false — do not re-file it. The one real loop move-tracker defect is defect 6.
13. **Not a defect: no `While` duplicate-block or wrong-local bug.** The "verbatim duplicate block reporting the wrong local" report was false; the sole real loop move-check defect is the `Never`-guard skip, defect 6.
14. **Our loop rule rejects the desugar of `for`.** `CantUnstackifyOutsideLocalFromInsideWhile` forbids moves of outer locals outright; upstream confirmed move-and-restore is intended to compile, and the `for` desugar *is* one — `while Some[(it2, x)] = it^.next() { body; set it = it2 }`. **A defect, not a strictness preference.**
15. **DONE** (`acb43c66`) — every `ExpressionTE` variant now carries a `range: RangeS`, populated at all construction sites (a core edit, done under "fire core edits"). The borrow checker's underivable-borrow diagnostic motivated it. Remaining tidy: the vestigial `BreakTE.region: RegionT` was not removed alongside it — fold that in when convenient.

### Dead code, verified

- **The `initially_known_runes` local in `solve_rune_types`** (`typing/rune_typing/rune_type_solver.rs` — note the bare name also hits an unrelated live field in `solver/solver.rs`) — **redundant and safe to remove**, with a structural proof rather than an argument from absence: `Call`'s puzzle names the template rune and `get_next_solvable` only returns rules whose puzzle is fully concluded, so its `.expect("Call: template rune unsolved")` is **unreachable by construction**, prepass or not. `Lookup`'s empty puzzle is vacuously always solvable, so the loop cannot terminate while any `Lookup` is unsolved. It is the orphaned `else` branch of the `if predicting` block removed in `dea61d925`. **One caveat: removal changes step ordering, which can change *which rune* a `SolverConflict` names** in already-erroring programs (same error/no-error outcome), and some solver tests assert on humanized text. Cleanup: `unpreprocessed_initially_known_runes` loses its prefix.
- **`PrimitiveRuneTypeSolverLookupResult` is never constructed** — only its declaration, its `IRuneTypeSolverLookupResult::Primitive` variant, and three read-only match arms, one of which is inside `lookup_rune_type`. All three `IRuneTypeSolverEnv::lookup` impls return only `Citizen` or `Templata`.
- **The `lookup_rune_type` Template→Kind coercion** is **dead in practice** — traced by construction-site enumeration, not execution. **Confirm before deleting: replace the arm body with a `panic!` and run the suite.** Two traps on the way out: deleting it orphans `check_generic_call`, and **`NotEnoughArgumentsForGenericCall` / `FoundTemplataDidntMatchExpectedType` are matched BY NAME in `higher_typing_error_humanizer.rs`** (live, reached from `compiler_error_humanizer.rs`) — deleting those two variants is a compile break. A crate-wide `#![allow(dead_code)]` means nothing warns.
- **Warnings are 7, not 8** (`grep -c "^warning"` counts rustc's own summary line). **Three are genuinely removable**: a duplicate `ITemplataT::Kind(_)` arm in `environment.rs` with a byte-identical body to the one just above it, and the dead catch-all in `get_puzzles` (`other => panic!(...)`) — `get_runes` is a comment-free one-liner with no catch-all, and no "sanity_checked"/"hand-duplicate of rune_usages()" comment exists in the tree. **Four are deliberate signposts** that self-clear as their work lands. **One needs judgment, not a delete**: `evaluate_addressible_lookup_for_mutate`'s shadowed `IVariableT::Capture(_)` arm is unreachable but holds a `panic!("implement: … ReferenceClosureVariableT")` plus ~20 lines of canonical Scala — un-ported work, not cruft.
- **Two of `ITypePR`'s five variants are unreachable** — `BoolType` and `CitizenTemplateType` have **no construction sites anywhere**, appearing only as arms in `rule_scout.rs`'s type translation, where `CitizenTemplateType`'s is a `panic!`. (`IntType` / `CoordListType` / `RegionType` are live. `KindType` was already removed with the Components rule, which had been minting a rune constrained by nothing.)
- **Two dead files**: `Backend/vstl/` (zero references; its 3 carets are already postfix in an older dialect) and `builtins/resources/functor1.vale` (not in `builtins.rs::ENTRIES`, referenced nowhere).

### Confirmed safe — hazards that turn out not to exist

- **Our move tracker already implements the CONSERVED discipline exactly as spec'd.** In the `If` arm, branches compile in by-value child envs, effects are recovered by diffing against the parent snapshot, and **disagreement is an error** (*"Must move same variables from inside branches!"*) — not union, not intersection, not last-writer-wins. Sets are `IndexSet`, so order-insensitive. `return`/`break` are handled correctly by **discarding** the diverging branch's effects. Two tests pin it (`if_branches_must_move_same_variables`, `..._different_order_compiles`) and both now reach the join, so a regression there surfaces immediately.
- **Loops have no fixpoint and no back edge** — moves out of a loop are forbidden outright (`CantUnstackifyOutsideLocalFromInsideWhile`), and the body's effects are never merged. **A monotone least-fixpoint analysis has no structure to attach to**; the `While` arm compiles the body exactly once.
- **The `self` silent-corruption hazard does NOT exist.** **Zero** bare `self` receivers in the corpus; exactly one `&self`, in a parser unit test that never reaches typing. All 92 `self`/`this` receivers spell ownership explicitly (69 `self &T`, 19 `self T`, 23 `this …`, 4 weak/`&&`). Better than neutral: **all 19 owned receivers are genuine consumers** (15 `drop`s, 4 `Subprocess` joins, `Opt.or`, `HashSet.add`), so the corpus is *already* consistent with the incoming rule. Adopting "bare `self` = owned" costs **nothing** in migration. There is even a test named `this_isnt_special_if_was_explicit_param` — we're already aligned with design-1:397's "the receiver is not special."
- **The `is_primitive` divergence is NOT a bug to reconcile — it's two predicates sharing one name.** A user **cannot** export `str`: the grammar accepts `export str as X;` (there's even a passing test for `export int`), but the export path panics on every builtin and the backend `exit(1)`s. So **`typing/compiler.rs`'s `Str => true` is load-bearing** — it means *"needs no user export declaration"*, and flipping it would make an exported/extern `str` param permanently unsatisfiable, immediately breaking `stdlib/src/path/path.vale` and `command/command.vale`. **`typing/types/types.rs`'s `Str => false` is the ABI/representation sense** and is equally correct — and the backend agrees, classifying `Str` with the handle types at four independent sites. The header generator emits `typedef struct vtest_str { uint64_t _reserved; }` for **every package regardless of exports**, plus auto-registered `str_len`/`str_char_at`/`str_alias`/`str_dealias`/`str_ref_eq`; a golden test pins this against a program containing no `str` at all. **The fix is renaming, not moving the `Str` row.** (Six of seventeen variants disagree, incidentally — `OverloadSet` is a second, inverted disagreement, currently harmless.)

### Easy things to get wrong here

- **`MaybeCoercing` and `predicting` are already gone** — `predicting` in `dea61d925`, and `MaybeCoercing*` was never an `IRulexSR` variant at all (the enum has 12; every surviving mention is a comment). **One live nuisance: `solve_rule`'s catch-all `unreachable!` message names rule kinds that no longer exist.**
- **`replace_value_type_in_ref` has ZERO surviving ZHERE markers.** `as_subtype_macro.rs` has one call site and no ZHERE; `UpcastTE::new` is implemented (it calls `replace_value_type_in_ref`) and carries no marker. Sibling worth knowing: `InterfaceToInterfaceUpcastTE::new` carries a `VCOORD` + `unimplemented!()` with **zero callers**.
- **The validity-table question is latent everywhere**, because share barely flows through typing — two live `KindT::ShareRef(_) => unimplemented!()` arms, both in `typing/compiler.rs`. Only `look_for_override`'s could genuinely bite, since there the citizen changes *identity* (interface → struct) rather than just substitution — that is defect 9.
- **The `rune_type_solver.rs` markers the handoff once described are gone** — no ZHERE "fill this Call arm" note and no `VCOORD` arcana note remain in that file.
- **The `T: Drop` trap** is stated in full under ruling 13's traps — check for a `drop` **function**, never the bound, and further distinguish declared from synthesized.

### The interface tier census

**18 of ~133 interfaces are `share`-declared (~13.5%); ~86.5% are struct-tier.** The minority is *structured*, not scattered: **11 are the `externs/interfaceimm*` family** (FFI boundary tests, all `sealed exported`), and 7 are immutable-linked-list / ancestor-stamping fixtures. Only 2 live in Rust fixtures, one of which is a parser unit test with no implementors. Implementor counts run 1-3, so retiering is mechanically cheap. **Surface spelling is `share`, placed after the name and generics** — the `SharednessP` slot in the lexer's citizen header. Two disambiguations: **`imm` is the retired Scala-era spelling for that same slot** (proven by 11 one-to-one pairs against the stale build tree), while **`imm` as a *region* modifier is a different, still-live construct** — don't conflate. Asymmetry worth noting: `share` is far more common on **structs** (49) than interfaces (16).

## The compiler as it actually is

### Four facts that are easy to get backwards

1. **An unannotated `&T` synthesizes no region rune.** It is `RegionP::Unspecified` → `RegionSR::Unspecified`. Exactly **one** origin producer of `RegionSR::Rune` exists: the synthesized closure parameter (`function_scout.rs`). No explicit `'r`/region annotation produces one (the anon-interface macro's rune remapper only rewrites an existing `Rune`'s inner). Do not conclude from the rune-type solver's `BorrowRef` region arm that bare borrows feed it — they don't.
2. **The call site runs phases 1–2 twice**, once over explicit template args in `overload_resolver.rs` and once over the callee's full rule set in `function_compiler_solving_layer.rs`. That is the call site doing two phases in two passes. **It is not evidence against the phase model** — the ratified phase design (`docs/plans/plan-phased-calls.md`) is the authority; this is the implementation being mid-migration toward it.
3. **`ExpressionTE`'s `result` field is not uniformly a `KindT`.** Seven nodes store a narrowed arena ref — `LocalLookupTE`, `LetAndLendTE`, `MemberLookupTE`, `StaticSizedArrayLookupTE`, and `RuntimeSizedArrayLookupTE` hold `&'t BorrowRefT`, `BorrowToWeakTE` holds `&'t WeakRefT`, `ConstantStrTE` holds `&'t ShareRefT`. **`ExpressionTE::result()` is the only correct accessor.**
4. **`simplifying/` and `final_ast` are gone from `src/` entirely** — `simplifying` survives only in comments noting its removal, `final_ast` has zero occurrences. `von` is not a top-level directory either (only the module file `testvm/von.rs`). The instantiator is active and matches typing's live `ExpressionTE::While/Return/Break`; `ReferenceExpressionTE` is a distinct enum with zero hits under `typing/`. Of the originally-listed archaeology, only `testvm/` and `pass_manager/` (plus `instantiating/`) remain.

### Verified mechanics

**Groups / regions.** The region-on-the-value-type model is retired: `BorrowRefT` is `{ inner }`
with no region, and groups live only on declaration-side structures. `docs/plans/path-to-borrowing.md`
is the design and the built-vs-unwired line. Still-true mechanics:
- **`ITemplataType` carries `GroupTemplataType`; `ITemplataT` carries a `Group(GroupTemplataT)`
  variant** — the `RegionTemplataType` twin is gone (the Region→Group rename). A group generic param
  (`<g'>`, a `RegionGenericParameterTypeS`) types as `GroupTemplataType` and concludes to the ceremonial
  `ITemplataT::Group(GroupTemplataT {})` constant *everywhere*: on the defining side via
  `create_placeholder`'s `GroupTemplataType` arm (`templata_compiler.rs`), and seeded directly on the
  call-site and virtual-dispatcher resolve paths (`function_compiler_solving_layer.rs`). It is inert:
  `substitute_templatas_in_templata` returns it unchanged and `get_placeholders_in_templata` treats it
  as no placeholder — a group param never mints a `Placeholder`. The instantiator mirrors it with
  `ITemplataI::Group(GroupTemplataI {})` (`instantiating/`).
- **`KindT` is a deliberately 16-byte `Copy+Eq+Hash` enum (`@WVSBIZ`)** — this is *why* a group must
  never live in it: it would join type equality, arena interning, and monomorphization identity.
- **`RegionT` (`Iso`/`Default`) is vestigial scaffolding** — still on `context_region` params and the
  `RawArrayNameT`/`ExternNameT` name-structs; removing it fully is a follow-up sweep. `RegionT::Iso`
  is constructed zero times.
- **`substitute_templatas_in_kind` recurses through all four ref wraps** (`templata_compiler.rs`),
  rebuilding each around its substituted inner; with `BorrowRefT` group-free there is no region to
  carry through generics.

**Bounds — a bound is a DENIZEN LOOKUP, never a predicate.**
- The entire bound vocabulary is `PrototypeT` and `IdT` (`InstantiationBoundArgumentsT`, in `hinputs_t.rs`). Discharge is calling the **overload resolver**. The only failure vocabulary is "couldn't find a function/impl" or "return type mismatched" (four `IConclusionResolveError` variants). `IRulexSR`'s twelve variants contain **no general assertion rule**.
- **Bounds are checked once per CALL-SITE RESOLVE in typing — not at instantiation.** `add_instantiation_bounds` (`compiler_outputs.rs`) is write-once with an equality assert, explicitly *not* a merge or re-check. The instantiator does **zero** verification, and doesn't compile.
- **The real per-call-site re-check lives one system over**: `assemble_call_site_rules` (`templata_compiler.rs`) re-runs **every rule except `DefinitionFunc`** at each call site, filtered by `include_rule_in_call_site_solve`.
- `assemble_rune_to_*_bound` lives in **`templata_compiler.rs`**, not `impl_compiler.rs`. Assembly is **not uniform** — two unrelated mechanisms across five denizen paths plus six hand-written empty literals. **No single seam to extend.**
- **Impl-bound checking is live** — `infer_compiler.rs` loops the denizen's `impl_bounds`, proves each via `is_parent`, and populates `rune_to_bound_impl` from the `runes_and_impls` accumulator (`vec![]` is only its initial value).
- **The substituter is narrow**: `substitute_templatas_in_templata` matches 6 of `ITemplataT`'s 16 variants (Kind/Placeholder substitute; Integer/Boolean/Group pass through; Prototype panics), the rest hitting a wildcard panic. `IPlaceholderSubstituter` has two methods, both thin delegators that do not panic; the three `panic!("Unimplemented: Slab 15")` calls are on `is_root_function`/`is_root_struct`/`is_root_interface` in `compiler.rs`.
- **Argument types DO enter the call-site value solve.** `assemble_initial_sends_from_args`'s result is consumed at three solving sites: each `InitialSend` becomes an `InitialKnown` + an `Equals` (sender → receiver) rule + a `KindTemplataType` for the sender rune, all fed into the solve. So the arg's Kind reaches the solve; the earlier "consumed nowhere / machinery has to be built" reading is stale — the phase-0 shape adjustment on top of it is what remains.

**Body compilation and the expression walk.**
- **Control flow DOES survive into the finished `ExpressionTE` tree** — the load-bearing fact for a post-hoc checker. Structural nesting plus `KindT::Never { from_break }` is enough to derive successors: `IfTE` is its own join point and its `result` says whether the join is reachable; `WhileTE`'s back edge is total and implicit (Vale's `while` *is* `loop`; the condition desugars into the body with a `Break`); break targets are the nearest enclosing `WhileTE`, sound because break can't cross a function boundary; return always targets function exit. **There is no `continue` in the language.**
- **Every `ExpressionTE` variant now carries a `range: RangeS`** (`acb43c66`) — the post-hoc checker can point at any node. Synthesized macro nodes with no user source use `RangeS::internal`; a real range is threaded everywhere one exists.
- **`typing/test/traverse.rs` is a complete live traversal skeleton** — all 47 `ExpressionTE` variants, with collect macros. It is the template for a checker's walk and arguably wants promoting out of `test/`.
- **There is no `typing/reachability.rs`.** The three `panic!("Unimplemented: Slab 15")` stubs are the root-detection methods `is_root_function`/`is_root_struct`/`is_root_interface` on the compiler (`compiler.rs`).
- **`DeferTE` has undefined semantics** — live, emitted from five sites via `make_temporary_local_defer`, and encodes neither *when* the deferred expression runs nor what happens if the inner one diverges. Every consumer that would have defined it is dead code. A checker must pick a semantics and state it.
- **The if-join disagreement check is CONDITIONAL** on `then_continues == else_continues`. If exactly one branch is `Never`, the compiler **unilaterally adopts the survivor's moves with no comparison**. The failure is a `RangedInternalErrorT`, not a designed diagnostic.
- **No linear-obligation tracking exists**, and the reason is structural: `drop_since` auto-generates drops for everything live at block end, so nothing is ever *required* to be consumed. Under ruling 13 (drop *absence* creates the obligation) **that auto-drop machinery is what has to become conditional.** Upstream's landed shape: *"conditional on the fields, resolved per instantiation, carried as a bound on what is generated rather than deferred to monomorphization"* — so **a generic container's drop existing at all is instantiation-dependent**; `Vec<int>` and `Vec<Future<int>>` differ in whether a scope-end drop exists.

### Outstanding ZHEREs — two, plus one ZLOOK

> **Do not go looking for a ZHERE on the `implements` mint** — that work landed on both sides and the
> marker is gone. Same for `solve_rule`'s `KindList` arm and the "Bad template call" one; both are
> now `VCOORD`s recording what is left.

Two ZHEREs remain, both in `expression/expression_compiler.rs` — grep for them rather than trusting any location written here:
- the now-dead `ExpressionTE::LocalLookup => Unlet` sub-arm in `Ownershipped`'s `Move` case (`^local` never reaches `Ownershipped` since the scout routes it to `Unlet`).
- implement `weak x` (`LoadAsWeak`) as a `WeakRef` of the source.

(The closure-var mention is already implemented and wired; `as_subtype_macro.rs` has one call site and no ZHERE; `rune_type_solver.rs`'s marker is gone.)

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
- **`own` = the new `OwnRef` wrap.** At class kind it's the *exclusive* state (sole reference; `own self` is the class destructor's receiver); at struct kind redundant with bare. **Valen has since narrowed this** (`own` → `ownref`, immovable-only). The narrowing is a later item; the `OwnRef` wrap itself is unaffected.
- **`weak` stays `weak T`.** Heap-owned is **not** a language keyword — it's a library `Box<T>`, matching Valen; `heap` is removed. This is distinct from `own`: `own` is the language-level *exclusive* state, while `Box<T>` is heap allocation. **`Box<T>` is not fully user-space, though** — design-1:1523 makes `entity.armor[]` a **child group** and design-1:1525 makes `Box<T>` affine-or-linear according to `T`, so the borrow checker needs compiler knowledge of it. Bites at rung 1, not now. Open internal-model question: with heap-ownership library-side, there is **no value-model `HeapOwnRefT` wrap** at all (`KindT` carries only the four ref wraps; `HeapOwnRef` is only a proposed future variant) — and something must still supply that `[]` child group when heap-owned surfaces.
- **Erasure / trait model (all long-term / deferred):** `interface I` (class-tier: no `dyn`, bare = strong `ShareRef`, `I in r` = strong into a non-ambient multi, `weak I`) vs `open trait T` (struct-tier, Rust-`dyn`: `&dyn T` / `Box<dyn T>`, bare `T` = a bound). **Sharedness (share vs single on the definition) carries both the class/struct AND the interface/open-trait split** — no separate keyword; `dyn` appears only for open-traits.
- **The colon is one of the two intended divergences** in `name: type`: Vale2 **allows but does not require** it; documented Valen always writes it. Even that is only house style — design-1:2350 permits the colonless form for experimentation. Vale2 is the one behind: the parser has no colon support at all today. (The other divergence is that a mention always yields a reference, copied out only at a bare-value receiver; both are provisional and slated for an experimental flag — see the banner at the top of this file.) **Everything else that differs is a bug.**
- **The `in`-clause group grammar — named groups, member/element paths, and the descendant step
  (`g...`) are landed; unions and `rc` are not.** The group-param **tick lives only at the
  declaration** (`<g'>` untyped / `<g': T>` typed, both parse) and **every use is bare**: `&Ship in g`,
  `&Ship in g.items`, `&Ship in g.items[]`, and `&Ship in g...` parse (`parse_group` in
  `templex_parser.rs`) and scout (`translate_group_p_into_group_s`) to
  `RegionS::Group(GroupS::Rune | Local | Member | Elements | Ellipsis)`, in parameter *and* return
  position (the return type lands on `FunctionS.maybe_return_type`); `mut(g)` / `not(mut(g))` /
  `mut(g...)` parse and scout to `EffectS`. The old `&i'MyStruct` apostrophe-*prefix* borrow-use syntax
  is **retired** (the declaration tick stays). Unimplemented — `GroupP`/`GroupS` variants the parser
  does not yet produce: union (`a | b`) and the ambient-multi group `rc`.

### Reference model
Four ratified decisions, in force. Together they **retire `RegionT::Held` entirely — do NOT add it** — and reshape how a mention lowers.

1. **C1 — what a bare mention yields** (design-1:159-171, :437). **This is design-1's ruling, kept only as the experimental-flag alternative; Vale2 diverges by default for the foreseeable future, possibly permanently — a mention always yields a reference, even for `Copy`, read out to a value only at a bare-value receiver (see the two-divergences banner).** design-1's C1: *"**There is no auto-borrow**: a bare non-`Copy` argument is a C1 error naming `&x`, `x^`, and `x.clone()`."* Three arms: a bare mention **copies** if `Copy`, yielding a *fresh, isolated group* that nothing mutating or destroying the source can invalidate; **errors** for a non-`Copy` struct; **errors** for an unbounded generic `T`. `&` lives in expression position at reference-*creation* sites. A borrow is itself `Copy`, so passing one along stays bare (`resize_buffers(graphics)`) — a borrow reaching a borrow parameter needs no mark. **But reading a value *out* through a reference into an argument does**: argument positions do not adjust (design-1:216), so `keys.append(*k)` is written with the `*` even at a `Copy` key, and *"the `*` is load-bearing there and not decoration"* (design-1:1862). Field and method access are where auto-deref applies — `k.field`, `k.method()` — not argument positions.
   - **`RegionT::Held` is cancelled and must not be reintroduced.** Under C1 there is no bare non-`Copy` use at all, so there is no bare-use ambiguity for a region to mark.
   - **The `LocalLoad` collapse stays while we diverge.** `Use` and `LoadAsBorrow` are merged into one plain `LocalLoad` because a mention already *is* a reference — which is exactly the current divergence, so the collapse holds. The C1 flag would undo it — bare becomes an error, `&x` the borrow, and the `BorrowRef(NC) → bare NC` auto-clone goes with it — but it is off by default and may stay so indefinitely.
   - **Consequence for the borrow checker (under C1):** C1's `Copy` arm yields a fresh isolated group, so a bare `Copy` mention creates **no alias at all** — materially less invalidation surface to track. Under the current divergence a bare `Copy` mention is instead a borrow of the slot until read out, so that smaller surface arrives only under the C1 flag.

2. **Eager auto-deref (chosen over lazy).** A lookup is uniformly the *address-of-slot* — **`LocalLookupTE` stays uniform, do NOT make it idempotent** (`LocalLookupTE::new` always allocs the raw address-of-slot `BorrowRefT`; the related comment is the `// "undecayed"` note in `expression_compiler.rs`, where the `&&`→`&` decay happens at the call site). At the **read/use path** (right after the lookup), if the result is `BorrowRef(inner)` where `inner` is itself a reference kind, insert a **`DerefTE`** peeling exactly ONE storage layer → the stored reference. So a `&Ship` local mentions as `&Ship`, not `&&Ship`; this covers local/member/array lookups uniformly. The **mutate path keeps the raw `&&`** address-of-slot (it needs the slot to write). `DerefTE` (`ast/expressions.rs`, `{ inner, result }`, `result = peel_one_reference(inner.result())`) is landed and wired into `result()` + `test/traverse.rs`. Chosen over lazy because it reuses `convert()` (row 7 handles the rare re-borrow to a `&&` target) and makes "mention = reference" literally true in the type. Remaining work is `ZHERE`-marked in `expression_compiler.rs`.

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
    - **The live query is `get_sharedness`** (`compiler.rs`), reading `.sharedness` off the illuminated citizen definition; `struct_compiler_get_sharedness` (`struct_compiler.rs`, caller `struct_constructor_macro.rs`) is the other live reader. There is **no `declare_type_sharedness`** (only a TODO comment). `get_sharedness` exists but has no live callers today (its other mentions are commented out); `lookup_mutability` is gone. **This is the query the position rule will need.**
3. **Bare-use and `&`** are governed by C1 and the reference model — see Reference-model decision 1. **`RegionT::Held` does not exist and must not be introduced**; a lookup is the uniform address-of-slot, with a read-path `DerefTE` peeling one storage layer.
4. **Coord's fate.** `Coord` disappears entirely; walking is pure `Kind`. `BorrowRefT` is now `{ inner }` with no region field — groups do not live on the value type (see `docs/plans/path-to-borrowing.md`). `RegionT` survives only as vestigial scaffolding (`context_region` params, `RawArrayNameT`/`ExternNameT`) pending its sweep.
5. **`convert()` / auto-coercion.** See the coercion table below. **The `implicit_clone` probe mechanism is retired** — what survives is the structural rows plus the upcast.
6. **Backend representation.** All IR stages get the onion — T-IR, I-IR, H-IR. `CoordH` disappears symmetric to `CoordT`. Backend C++ / Metal eventually walk the onion (large end-state refactor; scoped as a follow-up Backend arc after the frontend arc lands).


7. **The expression hierarchy flattens.** No `ReferenceExpressionTE` / `AddressExpressionTE` / `AddressResultT` / `IExpressionResultT`; one `ExpressionTE`, and `ExpressionTE::result()` returns a `KindT`. An addressible expression is just a reference expression plus a reference: the lvalue lookups (`LocalLookup`, `MemberLookup`, `StaticSizedArrayLookup`, `RuntimeSizedArrayLookup`) each return a `BorrowRef` of the thing they point at, e.g. a borrow-ref of an int element, or a borrow-ref of a shared-ref class. Cost: lvalue-vs-value is no longer in the type, so `evaluate_expected_address_expression`'s demand loses its compile-time basis.
8. **`SoftLoadTE` dissolves.** A lookup already hands back a `BorrowRef`, so there is nothing left to load. Its call sites collapse onto the lookup result. The load's remaining job, reading an `&P` out into a `P`, is a `convert()` coercion rather than an instruction.
9. **An owned value is a bare kind**, i.e. zero ref layers. Constructors (`NewRuntimeSizedArray`, `StaticArrayFromCallable`, owned `Construct`) produce bare kinds. **`HeapOwnRef` comes much later**: don't design for it, and stub it with `panic!("implement: ...")` wherever it surfaces in a dispatch. (Caveat, see the reference/ownership surface model: heap-ownership is a library `Box<T>`, so `HeapOwnRef` may end up vestigial; the exclusive surface form is `own`, whose wrap target is open.)
10. **Expression nodes store their result.** Every `*TE` carries a `result` field plus a private `_sealed`, and is built through a mandatory `new()` that computes the result and allocates any wrap payload into the arena. Wrap-producing nodes store the payload ref (`&'t BorrowRefT`), so reading a pointee is a field access rather than a walk. Per-struct `result()` getters are gone; only `ExpressionTE::result()` remains.
11. **`set` takes a borrow and yields the old value.** `MutateTE`'s destination is always a `BorrowRef`, and its result is the destination's pointee, i.e. the value that was replaced. So `set` is the one sanctioned move-out-of-a-borrow. It's safe because the hole is refilled in the same operation, exactly like Rust's `mem::replace`, which is why `CantMoveOutOfMemberT` can stay the rule for bare moves. This retires RMLRMO (`docs/old/Compiler/Templar/Addresses.md`), whose "result in the member's type" conclusion assumed a place-typed destination.
    - **The `VCOORD: onion old-value semantics to confirm` marker is still present** on `MutateTE.result` (`ast/expressions.rs`) — not discharged. The ruling scopes to *movable* types, and the corpus rationale for design-1:1703's unnameable temporary was checked rather than assumed: it has two motives and **neither argues against yielding**.
    - **The ordering motive is load-bearing and easy to break.** `valen-approach-convo-12:307` wanted *install-new-before-tearing-down-old* so a reentrant observer always sees a consistent object; the temporary was unnameable as a *consequence* of choosing "drop it" over "hand it back," not as an independent requirement. **A returning `set` preserves it only if we install first and hand back second.** Implementing it as hand-back-then-install would satisfy the ruling and silently break the reason underneath it — this belongs in a source comment wherever the lowering lands.
    - The second motive (`convo-12:514`, an adversarial pin) is that the linear *(was "linear-strict" — see ruling 13's vocabulary inversion)* error was *engineered* to stop a silent linear drop. A returning `set` serves that strictly better: the value is handed to the caller and discarding it is the ordinary unconsumed-linear error.
    - **Two pins to keep wired:** a **poisoned** old value hands its poison to the receiving binding (`convo-12:738`, design-1:713 — we have no poison-travel yet, so this is a list item, not code); and destroy-in-place lowering becomes conditional on the result being *unused* (as-if, so no semantic exposure).
    - **Grammar consequence: `set` must be expression-valued and legal as the RHS of a `set`** (`set x = set y = set x = None` swaps two vars). Our parser appears to satisfy this already — `set` parses to `IExpressionPE::Mutate`, reachable both from statement position and from the atom parser in `expression_parser.rs`, with the mutatee/source split taken at the first `=` so chaining recurses. **Read from the code, not verified by a run** — worth a test case when the parser is next touched.
    - **Residual:** `swap` still does not compose from `set`, because the innermost `set` needs a value to install. It works for any type with a spare inhabitant (`set x = set y = set x = None`) and not otherwise, so design-1:2551(b) is not a place-parameter grammar hole but a **vacancy hole, one stdlib function wide, over spare-inhabitant-free types**. Expect `swap` to stay stdlib-with-unsafe-internals.
12. **`BorrowRefT` carries no region** — rung 0 emptied it to `{ inner }`. `RegionT` (`Iso`, `Default`) still exists as vestigial scaffolding on other sites (`context_region` params, the `RawArrayNameT`/`ExternNameT` name-structs); removing it fully is a follow-up sweep.
13. **Addressibility is retired for good**, not deferred. Master's `IVariableT` had Addressible-vs-Reference as its outer axis and ownership as its inner one; the 4→2 collapse (`538fdb12a`) kept Local/Closure and dropped the other axis, which left `determine_if_local_is_addressible` as wreckage (now commented out) rather than drift — `determine_closure_variable_member` is NOT wreckage; it remains live in `function_compiler.rs`. The replacement is an LLVM-style model: every local is storage, a lookup yields a pointer to it. Mutation-sharing becomes `&x` to the same storage; moves become move-out-of-a-borrow (a compile error, the user writes `^`); lifetime becomes groups (declaration-side, not on `BorrowRefT`); layout is the hammer's problem, not typing's. The borrow checker enforces the lifetime half through rung 2 so far (joint-argument + use-after-churn); more child-group sources and effect checking remain. Addressibility is **orthogonal to the onion**: it was about whether a variable's slot is indirected, while onion layers are about what the value is.
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

    **This is why explicit `T` simplifies the implementation.** With the super side concrete, `is_parent` looks up one specific sub/super pair and the legal row is simply a different pair. Had we kept deduction-through-upcast, `get_parents` would return *both* supers for `Firefly` and we would have needed a disambiguation rule — which, without specialization, could only have been an ambiguity error anyway.

    Related: `SharedImplingMismatch` (defect 9) is now written a few lines away in the same function; the overlap check is the one still missing.

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
| 5 | bare `K` (from `^local`, literal, ctor) | bare `K` | pass-through |
| 6 | `HeapOwnRef(K)` / `ShareRef(SC)` / `WeakRef(SC)` (from `^local`) | same shape | pass-through |
| 7 | bare `K` (from a literal or ctor) | `BorrowRef(K, r)` | materialize a hidden local, lend it, defer its drop, e.g. `&2` |

Plus the upcast, which `convert_via_upcast` owns.

**Errors — no silent coerce:**

- (d) `BorrowRef(BorrowRef(K, r_i), r_o)` → `BorrowRef(K, r)`. Double-borrow arises only from an
  explicit `&&x` or generic instantiation. The borrow blanket satisfies it via bound resolution;
  auto-coercion does not peel it.
- `BorrowRef(OwnRef(K), r)` → `OwnRef(K)` — move-out-of-borrow; the user writes `^local`.
- `BorrowRef(NC, r)` → bare `NC` — a read-out that would need a clone. The user writes `.clone()`.
- Kind mismatch across any coerce site.

**Status in `convert()`.** Rows 5, 6 and the upcast are implemented; **row 7 is NOT** — its arm
(`convert_helper.rs`) is `panic!("Temporary locals temporarily disabled until we remove overloading")`
with the materialize/lend/defer body commented out (the bare-K→`ShareRef` arm panics the same way). Row
(d) is an error by design. The numbering is sparse because it outlived the probe rows and the
region-differ row that used to sit between these.

**The coercion sites.** `convert()` has eight physical call sites collapsing to six coercion categories
(the if-expression's then+else arms and the two mutate paths each pair up); they enumerate every place a
coercion can happen:

- call arguments, via `convert_exprs` from `evaluate_call`
- a function body's result against its declared return type
- a `return` statement
- each branch of an if/else, against the common type
- a `Mutate` / `LocalMutate` destination
- a let / pattern binding

Not all six pre-guard with `is_type_convertible` (the pattern binding doesn't). `convert()` returns `CouldntConvertT` / `CouldntUpcastT` only for the upcast rows (via `convert_via_upcast`); for other failure shapes it **panics** — targeting `Never`, the disabled bare→borrow / bare→share rows, and the catch-all `vfail: cannot convert`.

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
- **`pointify_kind` is commented out** — it branches on sharedness (`Single` → ownership-if-mutable,
  `Shared` → `Share`) with no assert, and has no call site anywhere (neither live nor commented).

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
- **`pass_manager.rs` and `pass_manager/full_compilation.rs`** are declared unconditionally (ungated) in
  `pass_manager/mod.rs`, and hold **no** `higher_typing` imports — there are no `use crate::higher_typing::*`
  imports left anywhere in the tree (the module was retired). They want deletion or rewiring once the
  pipeline shape settles.

**Pipeline shape:** wired end-to-end (parser → postparse → typing → instantiator → backend); the typing
slice is the live front, and the instantiator and backend consume its onion output. There is no `hammer`
stage — it and `ProgramH`/`final_ast` are deleted.

### What blocks / what to preserve

- **`AliasTE` / coercion-accept patches / coherent-collapse arms are gone** — no live `#[cfg(any())]`-gated arms remain in `typing/`; every `Augment` / `AliasTE` / coherent-collapse reference is now an inert comment.
- **►► THE `implicit_clone` PROBE IS RETIRED IN THE DESIGN AND STILL LIVE IN THE CODE ◄◄** The
  coercion table is the design statement; **the removal is unstarted work nobody has listed.** What is
  still there: `convert_via_implicit_clone` (`typing/convert_helper.rs`) and its single call site, the
  two error variants `NoImplicitCloneDefinedT` / `ImplicitCloneRejectedT` — **still emitted**, with
  live humanizer arms — the `implicit_clone` keyword, five builtin func registrations, one
  `expression_compiler.rs` site (`wrap_in_implicit_clone`, currently a `panic!("implement")` stub with its
  real logic commented out), **10 `.vale` corpus files** and roughly 60 test references. Deleting
  it is a real slice, not a sweep: the `.vale` sites and the tests that assert on the two errors have
  to go with it, and `user_defined_implicit_clone_allows_bare_use_of_struct` is the ignored test that
  asserts the retired rule outright. **Do not read the coercion table as a description of the tree.**
- **The resolver structural-consistency principle** (Augment DIR1 Shared-arm reject-on-contradiction) — the specific check migrates to whatever mechanism enforces onion-typing structural constraints in the solver. Preserve the spirit, not the check.
- **The "drop(bare_local) is a compile error, drop(^local) is mandatory" rule** — semantics, not representation. Preserve.
- **6 rune-type-inference test fixtures** — pack literal (`Refs(int, bool)`), empty pack + `Prot[P, str]`, plain-param "undefined name" error, param-position / template-call / recursive-field rune-type-map assertions — **were NOT preserved**: `docs/regression-fixtures-from-retired-higher-typing.md` does not exist (the archive was never created or was lost). Reconstruct them from `git` history and re-author against `KindListSR` + `KindTemplataType` + on-demand rune-type derivation (`derive_rune_to_type`) when the typing slice lands the rune-type solver at `typing/rune_typing/`.

### Deferred test coverage (add after the main goals land)

- **Lookup-failure errors have no production test.** `CouldntFindTypeT` / `TooManyTypesWithNameT` are only exercised by humanizer-format tests; nothing compiles source that emits them, and `TooManyTypesWithNameT`'s humanizer is a stub. Retiring `explicify_lookups` moved detection into the rune-type solver, which wraps them as the coarser `HigherTypingInferError` — so specific-variant surfacing could degrade unnoticed. Fix: re-author the preserved "undefined name" fixture (above) as an end-to-end test asserting the specific error, add a `TooManyTypesWithNameT` case + fill its humanizer, and pin the taxonomy decision (keep `HigherTypingInferError` vs. unwrap to the specific variants).

### Critical reminders

- **NEVER commit without the architect's literal "fire commit" or "fire commit temporary".** Grammar note: the target-branch slot uses `with <target>`, sharing the `with` keyword with the CI opt-in. Parser rule: `with CI` = CI gate; `with <anything-else>` = target branch. Both can appear in the same invocation.
- **NO `#[ignore]` additions** without architect approval. If a test regresses during the onion arc, surface it and get direction; don't silently ignore.
- **Surface before reverting.** Onion typing is a big arc and mid-flight discoveries will surface things the initial design didn't foresee. Surface the situation + alternatives before undoing landed work.
- **The "green suite at commit time" invariant is suspended during the typing slice.** Typing is intentionally red while the semantic cascade runs — do not treat compile errors as regressions; they're expected fallout. The invariant reactivates once typing is green again.
- **When re-linking a module**, expect a wave of compile / semantic errors. That is the intended behavior — the parser slice weaponized structural mismatches. Work through them; don't stub around them without architect approval.

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

**Onion-typing arc-specific** (the old `/Volumes/V/Vale2/` base is gone; in-repo paths below):
- `docs/plans/onion-typing-plan.md` — big-bang plan (17 design gates with provisional leans, 10 areas of change, out-of-scope list). Predates the postparse-planning architectural discoveries above; treat as pre-refinement reference.
- `docs/plans/postparse-slice-plan.md` — the executed postparse slice plan (variant deletions/renames, scout stage rewrites, solver-side dispatch table updates, sub-commit sequencing, per-sub-slice RFIGA discipline). Also captures the higher_typing collapse + rune-type solver relocation direction (the collapse itself has since executed).
- `~/.claude/plans/please-plan-out-these-transient-balloon.md` — the parser slice's RFIGA plan (T1-T7 templex, E1-E5 expression, C1-C4 cleanup). Already executed but useful as a template.
- `~/.claude/plans/quirky-soaring-summit.md` — the executed higher_typing retirement plan.
- `onion-typing-scouting.md` and `docs/regression-fixtures-from-retired-higher-typing.md` **no longer exist** — reconstruct the 6 rune-type fixtures from `git` history when needed.

**Reusable mechanical scripts:**
- `/Volumes/V/Vale2/tmp/scripts/onion_typing_import_fix.py` — the safe-script-runner transform used for the typing-side import cleanup. Handles 13 retirement/rename categories on single-line and multi-line `use ...` blocks. Extend the `RETIRED_SYMBOLS` / `RENAMES` dicts to reuse for other retirements.
- `/Volumes/V/Vale2/tmp/scripts/comment_retired_arms.py` — the safe-script-runner transform that commented ~200 retired-variant match arms in typing/. Uses brace tracking to comment the whole arm body (single-line or block). Also extensible via its `RETIRED_PATTERN_TOKENS` list.

**Repo standards:**
- `CLAUDE.md` (project root) — standing rules for this repo.
- `~/.claude/CLAUDE.md` (your user global) — global rules (no `cd && cargo`, etc.).
- `docs/skills/valec-guidelines.md` — reviewer notes (never discard Err payload; no jargon-soup / historical / timeline comments; count-gating rules).
- `Luz/skills/prose-reviewer.md` — comment/prose rules (invariant framing, active voice, front-loading, generalization).
- `docs/skills/typing-reviewer.md` — typing-pass reviewer notes.
- `docs/skills/tdd.md` — RFIGA workflow (R/F/I/G/A per slice).
- `docs/skills/diagnose.md` — root-cause protocol.
- `docs/skills/fire-commit.md` — commit + push protocol.

**Design reference (pre-onion, some sections stale):**
- `docs/architecture/bare-clone-borrow-move-design.md` — long-term destination; some parts conflict with onion typing per scouting doc §9.1. Read scouting doc's reconciliation notes before treating as canonical.
- `docs/architecture/instantiator-design.md` / `instantiator_design_2.md` — instantiator/I-IR architecture; both need onion-typing rewrites when the instantiator slice lands.
- `docs/todo/opaque-extern-drop.md` — extern-struct drop design (waits on onion arc + Backend arc).
