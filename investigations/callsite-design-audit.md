# Call-site design audit — filter + six phases

Nine read-only agents over the conversation record, the Rust tree, and the language design
corpus. Collected on request because the handoff and the process that produced it are not
trusted.

**Provenance labels.** ARCHITECT RULED = the human said it. ACCEPTED = assistant proposed,
human explicitly agreed. UNRATIFIED = assistant proposed, no response, may have reached the
handoff anyway. ASSERTED = stated as established with no evidence given. VERIFIED = I checked
it in the source myself this session.

---

## Part 1 — What the audit found about the record itself

The handoff carries claims that were **explicitly retracted in the conversation that produced
them**. These are the ones with a retraction on record.

1. **The motive for name-uniqueness.** Handoff line 1636 says the motive is *"the collision is
   reported at the two declarations rather than at whichever call site first happens to reach
   both."* The architect corrected exactly that: *"'better diagnostics' wasnt about reporting at
   the declaration rather than the callsite. instead, it was that we never have to show a user
   all the reasons that all the candidates werent callable."* The assistant answered
   *"Withdrawn."* The withdrawn version is what is in the doc. **VERIFIED.**

2. **The withdrawn argument is filed as a Lesson.** Handoff line 48 — *"a diagnostics motive can
   defeat itself unless the destination is checked too"* — is the "self-defeating" claim, which
   was measuring against a goal the architect had just said was not his. **VERIFIED.**

3. **"The two rulings hold each other up."** Handoff line 1620 claims name-uniqueness catches the
   bounds-distinguished case the filter cannot. Refuted in session by a live test:
   *"I claimed name-uniqueness catches the bounds-distinguished case that filter-is-final can't.
   It doesn't"* — `failure_to_resolve_a_prot_rules_function_doesnt_halt`, where both parameters
   are bare runes, so neither function is in any namespace and nothing keys the check.

4. **An UNRESOLVED block the architect resolved.** Handoff line 1660 is headed *"THE RULE MAKES AN
   ORDINARY CLASS OF PROGRAM UNWRITABLE — UNRESOLVED"* and line 1676 says *"No replacement has
   been named."* The architect answered it — *"a user can write a println(int), they would just
   need to specifically import that function"* — and the answer is recorded forty lines later at
   1704 without the UNRESOLVED block being removed. Two homes, opposite claims.

5. **Phase 0's rationale.** In convo-8 the assistant claimed the parameter's two-way split exists
   because of phases 0 and 4. An agent checked and found the recorded reason was namespace
   lookup, with argument/deduction language appearing nowhere. The assistant flagged its own
   handoff sentence as circular — *"the only place stating this, and I wrote it two hours ago at
   your instruction"* — and then re-asserted it into the handoff later in the same session. Live
   at line 1588.

6. **The sharedness query.** An agent corrected *"don't use `struct_compiler_get_sharedness` —
   it requires the struct to be compiled, not merely declared, and would create a genuine cycle.
   The right source is `coutputs.type_name_to_sharedness`."* The handoff still names the wrong one
   at lines 457 and 1383, the two places someone would look.

7. **A ruling reversed in code with no record.** In convo-54 the architect ruled *"please take it
   out, ty"* about merging conjured impl bounds into the solver's conclusions, on the grounds that
   afterwards *"`inferences` no longer means 'what the solver concluded'"*. The tree now does
   exactly that: `conjure_impl_bounds_for_defining` takes `conclusions: &mut IndexMap` and inserts
   into it. **VERIFIED.**

8. **Unmeasured figures used as fact.** *"I wrote '13 citizens where there are 29' into Lessons.
   My own grep returns 13 — the 29 came from an agent counting over a wider tree, and I passed it
   through as if I'd measured it."* The 29/109 figures remain in a sizing decision at line 424.

9. **A dropped finding.** *"the two solvers currently resolve `Lookup` against different
   environments — caller's in the rune-type pass, callee's in the value pass — and a single
   pre-pass has to pick one, observably."* Called a live bug. Appears nowhere in the handoff.

**The record's own calibration**, from a wind-down: *"Treat them as authoritative on decisions and
unreliable on behaviour. What was decided is trustworthy. What the compiler does is worth a
ten-minute probe before you build on it. Every probe this session found something different from
what it went looking for."*

---

## Part 2 — Things to think about

### 2A. The filter

**The design corpus has no entry for overload resolution.** The words *overload*, *ambiguous
call*, and *namespace* appear zero times across `valen-design-1.md` and `valen-design-2.md`.
*"Module and import syntax"* is listed under Open Questions. Visibility (`pub`, `pub(crate)`,
`pub(super)`) is the only thing ruled. **VERIFIED by grep.** So the filter has no primary source
to re-derive from.

**The two-clause namespace rule was never ratified by anyone.** It is stated only as a
recollection — *"our rule is 'a function lives in T's namespace iff it's defined in T's file and
mentions T in a parameter.' That's a Vale-ism"* — with the assistant's own flag: *"our overload
mission rests on an unchecked assumption."* ASSERTED.

**Clause (a) does not match the tree.** `drop.vale` holds `drop(x int)`, `drop(x bool)`,
`drop(x str)`, and there is no `int.vale`. `arith.vale` holds every operator; `logic.vale` holds
`==` and `not`. On clause (a) as written, none of these is in any namespace. `borrow.vale` and
`void.vale`, which the design owes, do not exist. **VERIFIED — 23 builtin files listed, no
`int.vale`, no `borrow.vale`, no `void.vale`.**

**Imported Rust functions may be ambient.** *"`rust_package_stores` puts Rust functions in the
`rust` package's top-level store, and `PackageEnvironmentT` unions all top-level stores — so
they're ambient, findable from every call site in the program. That's precisely the model this
design replaces."* The standing instruction was *"don't deepen the dependence on ambient
visibility meanwhile"*, and every interop case added since relies on it. Function and call site
**VERIFIED**; the union behaviour not yet.

**Candidate identity ignores the environment.** `FunctionTemplataT` eq/hash is `(range, name)`
and *"ignores `outer_env` entirely, so being in different environments doesn't save you."*
Two synthesized declarations sharing a sentinel range collapse silently — *"No error, no assert,
in either case."*

**Type lookup and function lookup take different paths.** Function names go through the plural
`lookup_all_with_imprecise_name` and get scored. Type names go through
`lookup_nearest_with_imprecise_name`, which is `_ => panic!("Too many with name")`. And
`PackageEnvironmentT::lookup_with_name_inner` *"takes the parameter `_get_only_nearest` and
ignores it"* — so nearest-wins shadowing does not happen.

**Whether `&Ship` mentions `Ship` is flagged OPEN AND LOAD-BEARING.** With no tiebreaker it
decides whether an ordinary `clone(&myShip)` is ambiguous.

**"A bare-rune parameter accepts anything" was never confirmed.** The assistant's own words:
*"the filter's 'bare rune accepts anything' arm absorbs this, but by luck rather than by design;
confirm it deliberately."* Nobody did. Vale4 hit the mirror image: *"an empty `value_type_rules`
doesn't read as absent — it reads as 'a bare rune, which accepts anything'"*, which would have
made every imported Rust function a candidate for every call of matching arity.

**Name-uniqueness cannot reach bare-rune parameters,** and they are the majority case: *"every
generic struct's synthesized constructor takes the struct's member types, which for `Some<T>` is
all bare runes."*

**`===` is ruled never to be overloaded** — *"nothing else should ever overload `===`"* — and it
sits in the bare-rune gap, so the ruling has no enforcement point.

### 2B. Phase 0

**"Argument positions do not adjust" is ruled and dated.** design-1:216, ruled 2026-07-25:
*"`k` is the reference; `*k` is the pointee. Argument positions do not adjust between them. The
`.` operator does, in both directions."* And design-1:255: *"the path operators (`.`, `[]`)
adjust; an argument does not."* Its scope note limits it to the reference/pointee axis.

**There is no auto-borrow at an argument position either.** design-1:741: *"There is no
auto-borrow in argument position: a bare non-`Copy` argument is a C1 error naming `&x`, `x^`, and
`x.clone()`."*

**An unruled fork would repeal that.** design-1:3017, dated 2026-07-29, lists three horns for what
`&x` forms at a claim-typed local, the third being *"a one-hop argument coercion — repeals
'argument positions do not adjust' (2026-07-25). Each horn costs something real; unruled."*
A phase 0 that adjusts arguments is taking horn (iii) implicitly.

**The only ruled argument adjustment is the class-kind anchored lowering,** and it is a different
axis: the caller finds or mints a claim. It can charge (`softmut(rc.T)`), so phase 0 can emit
effects phase 5 must see.

**Auto-move and auto-deref were listed and never discussed.** They appear in the phase-0 row from
convo-8 onward. Only auto-ref was ever worked through. Neither has a worked example anywhere in
the record, and `convert()` has no move arm.

**Phase 0 as one pass is untested against lambdas.** rustc's `check_argument_types` runs
non-closures, drains the solver, then closures — *"That's not lifetimes, not coherence, not
overloading. We have lambdas."* No Vale lambda case was ever constructed.

**Anything phase 0 must emit inherits phase 4's problem.** From rustc's `coerce_unsized`:
*"a coercion must decide whether to write an adjustment or not and there is nowhere to record
maybe… rustc's escape is a whole-body writeback pass — a stage we do not have."* This finding
survived every pruning pass and has no owner.

**Phase 0 cannot tell its two failures apart.** *"target not yet known"* (the explicit-`T` error)
versus *"target known and the argument does not match"* (an ordinary type error). Both present as
"cannot convert this." Recorded open since convo-9, unchanged.

**The wrap chain has no single statically-readable source.** `translate_signature_templex` splits
on written syntax, so a bare class parameter gets `type_outer_ref_rules = []` even though its real
type is an anchored borrow. Both the filter and phase 0 need the chain. The handoff says *"The
shape is known… but its home is not."* The position rule that supplies the second half is
unimplemented and cannot be scout-time.

**Phase 0's peel mechanism rests on solver arms that are unwritten or wrong.** Defect 11 — the
`BorrowRef` peel concludes into `result_rune` where it means `inner_rune` — is *"the peel
direction phase 0 depends on"* and was deliberately left unfixed to keep a measurement clean.
`WeakRef`/`OwnRef` arms were empty and unexercised. `ShareRef` has no rule at all.

**Nobody checked whether `type_outer_ref_rules` are in the call-site solve.** `full_type_rune` is
concluded *only if* they are, and the whole seed-then-let-the-rules-peel mechanism depends on it.
`include_rule_in_call_site_solve` was never checked against them.

### 2C. Impl walking

**A struct may implement one interface template several times.** Generics.md SCIIMT: *"`MyController<T>`
might implement `IObserver<SignalA>` and `IObserver<SignalB>`, so there would be two ImplT's for
it"*, and MLUIBTN: you must therefore look impls up **by template name**. So a walk seeded on the
sub side with an unbound target returns *n* answers, and only the parameter's concrete arguments
pick one — which is what the walk was computing. **VERIFIED.**

**Decision 16 does not defuse it.** `is_parent`'s `assert!(oks.len() <= 1)` is keyed on a resolved
*(sub, super)* pair; `IObserver<SignalA>` and `IObserver<SignalB>` are different pairs. And the
architect's own follow-up when ruling explicit-`T` was *"qq: what happens when a struct has two
impls for a specific trait?"*

**Upcasting is one hop only, by ruling.** Generics.md REMUIDDA: *"we can just index the direct
parents and children of structs and interfaces, and don't have to do any transitive
calculations."* Good news; but the one hop can be interface-to-interface, and
`InterfaceToInterfaceUpcastTE::new` is `unimplemented!()` with zero callers. Also flagged: the
code block in that section inverts the `impl X for Y` convention the rest of the doc uses.

**`get_parents` is a recursive solve, not a lookup.** `resolve_impl` ends in
`check_resolving_conclusions_and_resolve`, which discharges the impl's own bounds by calling
`is_parent`, which calls `resolve_impl` again — once per candidate impl, with no depth guard and
no memo. **VERIFIED.**

**It panics rather than erroring.** `impl_compiler.rs:141` turns any `ICompileErrorT` from that
discharge into a panic. `get_impl_parent_given_sub_citizen` panics three more ways if the super
rune isn't concluded or isn't an interface. And `resolve_impl` panics outright on
`ImplBoundTemplate` — the shape a `where` clause produces inside a generic body. **VERIFIED.**

**It cannot distinguish a real impl edge from a declared bound.** Bounds are minted as
`IsaTemplataT` and indexed under the same key; `is_parent` accepts both from one lookup with
nothing in the return saying which. Under a read-only phase 0 this may not matter, since nothing
is recorded — but a fabricated edge reaching the instantiator is *"a wrong answer rather than an
error."* **VERIFIED** that both kinds come from the same lookup.

**It returns a `Vec`, swallows errors, and is direct-parents-only.** `Err(_) => vec![]` with the
codebase's own comment *"Throwing away error!"*. So a broken impl reads as "no such parent".

**The call-site `is_parent` path has never executed** — zero probe hits recorded. Phase 0 would be
its first real user, so every property above is unverified in practice.

**`is_parent` mutates `CompilerOutputs`** — `add_instantiation_bounds` on the fast path, with
empty vectors. Called *"the largest API-shape risk in the whole seam. No follow-up."* **VERIFIED.**

**But a read-only precedent exists and is named.** `predict_struct_layer` computes a type via
`partial_solve` and deliberately records nothing: *"Usually when we make a StructTT we put the
instantiation bounds into the coutputs, but we unfortunately can't here because we're just
predicting a struct; we'll try to resolve it later and then put the bounds in. Hopefully this
StructTT doesn't escape into the wild."* `partial_solve` is `make_solver_state` + `continue` +
return conclusions, with no phase-3 step. **VERIFIED.**

**The Milano case runs the same trick in reverse** — seed only the known half and read independence
off what the solver failed to conclude. *"the solver's own inability to reach a rune is the
answer."* Prior art for a read-only partial solve.

**Impl resolution is not reliably idempotent.** `add_instantiation_bounds` is write-once with an
equality assert, over an input laundered through a `HashMap`, with the codebase's own comment:
*"sometimes when we evaluate the same thing twice we get different results."*

### 2D. Sends

**A send's three parts are one generation** — the `rune_to_type` entry, the `Equals`, and the seed
must reach the same solve. Splitting them across two solves sharing a map produced
`SolveIncomplete` on `ArgumentRune(0)`. ACCEPTED.

**The measured demand for phase-0 adjustment is currently zero.** Two populations were cited as
exactly that — 22 tests at the `BorrowRef` peel, 11 at the backward `Call`. Both were investigated
and both turned out to be losing candidates, cleared by writing rejections. The full-suite probe
that would split "genuinely needs adjustment" from "losing candidate" **was never run**.

**Adjustment can rescue candidates that should lose.** The worked case: *"Auto-ref would be
actively wrong here. Adjusting the `int` to `&int` so the blanket solves means selecting
`drop(&int)` over `drop(int)` — the wrong callee, and a silent no-op drop."* **VERIFIED.**
This is only safe under filter-is-final, where exactly one candidate is ever solved.

**Do not make solver arms tolerant.** *"That's auto-ref inside the solver, and it's precisely what
the ordering ruling forbids: phase 2 is structural deduction only, and `CoordSendSR`'s deleted
coercion-tolerance branch was exactly that mechanism."*

**A parameter's rules live per-param, and four of five sites forgot to fold them in.** Two still
carry `unimplemented!` tripwires. There is no single source correct for both hand-written and
macro-generated parameters, which is why every site must do the fold. The proposed
`assemble_function_rules` helper with the assert inside was never built.

**`ParameterS::new` asserts two @PFVSZ invariants but not the one that matters** — that a
parameter's value type is described by its own bucket. That is the hole both Vale2's dispatcher
gap and Vale4's synthesizer bug fell through, from opposite directions.

---

## Part 3 — Cases to try

### Phase 0 and sends

- `foo<T>(x &T)` called with an owned `Ship` — phase 0's motivating case (auto-ref).
- `f(x &ISpaceship)` called with `&Firefly` — same wrap chain, different value-type template.
  The case that decides whether the filter's template check is loose.
- `f<T>(a T, b T)` with two disagreeing arguments — two sends at one rune; disagreement must
  surface as the no-MSCA type error.
- `launch<T>(a &T, b &T)` with a `Firefly` and a `Serenity` — no most-specific-common-ancestor.
- `launch<int>(&Firefly<int>())` vs `launch(&Firefly<int>())` — the explicit-`T` ruling.
- `f<T>(a T, b &ISpaceship<T>)` with no explicit args — the program explicit-`T` forbids, and the
  one that would force phase 0 into a fixpoint.
- **A lambda whose parameter types depend on a sibling argument** — the single biggest untested
  assumption. Nothing in the record has one.
- A send into a rune pinned by a `where` clause rather than by explicit args — neither predicate
  says which side of the line it falls on.
- A parameter whose value type is rule-determined but whose wrap chain is not (the bare class
  parameter) — does phase 0 preview a target it can name but not shape?

### Impl walking

- `MyController<T>` implementing `IObserver<SignalA>` and `IObserver<SignalB>` — SCIIMT, the
  ambiguity by construction.
- `impl<I,J,K,ZZ> ISpaceship<I,J,K> for Milano<I,J,K,ZZ>` — an impl rune the interface does not
  determine.
- `impl<H,I,J> ISpaceship<J,I,H> for Firefly<H,I,J>` — the permutation.
- `impl<I,J> ISpaceship<int,I,J> for Raza<I,J>` and `impl<H> ISpaceship<H,H,H> for Enterprise<H>`
  — impls that constrain the interface side; the doc says these need separate dispatchers.
- `func launchAll<T>(x T) where implements(T, IShip) { launch(x); }` — the relation comes from a
  bound, not an impl. `resolve_impl` panics on this shape.
- `Serenity` to `IShip` via `IFirefly` — the one-hop rule; must fail directly and succeed staged.
- A wrapped operand at an `implements` bound — `T` binding to `BorrowRef(Struct(Raza))`.
  *"That's a semantics call, not a mechanical one."* `BadIsaSuperKind`'s humanizer is a `panic!`.
- `@Dog → @Animal` — share upcasts do not work at all today.

### The filter and namespaces

- `has.vale`'s four `has` declarations — no type declared in the file, so no namespace and
  name-uniqueness never fires. Today separated by arity and value-type template.
- `drop.vale`'s `drop(x int)` etc. — in no namespace under clause (a), since there is no
  `int.vale`.
- `drop<T>(v void, x T)` — two parameters, first is `void`. Does param 0 put it in `void`'s
  namespace, and does the union order across parameters?
- `as.vale`'s two `as` overloads — both parameters bare runes, separated only by wrap chain, in a
  file named for the operation. The counterexample to "by-move/by-ref pairs are migration
  phantoms."
- `opt.vale`'s `isEmpty` ×4 and `get` ×4 — bare/borrow twins in the definition file of the type.
- A constructor — mentions its type in the return, not a parameter. In no namespace under clause
  (b1) alone.
- `weak.vale`'s `func lock<T>(w weak T)` — does a weak parameter contribute a namespace?
- A synthesized constructor colliding with a hand-written one of the same name (`HashMap`), and a
  synthesized drop colliding with a written one (`List`).
- `foo(Ship, Rocket)` declared in both `ship.vale` and `rocket.vale` — the cross-namespace case
  no local check can reach, and the reason the `>1 → ambiguity` backstop stays live.
- Two same-named Rust items in one program (`Vec::new` / `String::new`) — the silent-collapse
  hazard. **No test exists**, and it is the failure that produces no error.

### Already in the suite, worth knowing

- `tests_stamping_an_interface_template_from_a_function_param` — two lines, was the sends
  front-runner, now green.
- `drop_bound_on_a_generic_struct_ignores_the_borrow_blanket` — three lines, knowingly red.
- `recursive_struct_with_opt` — five lines, the front-runner at every stage.
- `failure_to_resolve_a_prot_rules_function_doesnt_halt` — asserts that solving eliminates a
  candidate, which filter-is-final forbids. Its fate was never ruled.
- `opt_with_undroppable_mutable_ref_contents` — the current failing case. Parameter is
  `Opt<&Spaceship>`, fully concrete, so it needs a directed check rather than a walk.

---

## Part 4 — Questions

Ordered by how much they gate the work.

1. **What does the filter key on?** The corpus has nothing; the handoff says arity, wrap chain and
   value-type template; you say names and namespaces. If it is names and namespaces alone, what
   separates `has.vale`'s four `has` declarations, and what separates a Vale function from an
   imported Rust one?

2. **What puts a function in a type's namespace?** Clause (a) as written excludes most of the
   builtins. Does a borrow mention its pointee — ruled yes. A weak? A strong ref? A constructor
   that names its type only in the return? Does the union order across parameters as well as
   across wrap depth?

3. **Does phase 0 adjust arguments at all,** given design-1 rules that argument positions do not
   adjust and that there is no auto-borrow in argument position, and that the horn which would
   repeal it is unruled?

4. **May phase 0 walk impls,** given SCIIMT makes the walk ambiguous by construction, and given
   your own follow-up question when you ruled explicit-`T` was exactly that case?

5. **What disambiguates when the walk returns several supers?** There is no error variant today —
   only `assert!(oks.len() <= 1)`, which does not even cover this case.

6. **Which blankets shadow and which forward?** Asked three times, never answered. *"`clone` is
   the odd one out — it's about the handle; everything else forwards."* UNRATIFIED.

7. **Does the ordered union stop at the first match, or collect and then resolve?** You ruled
   "ordered union"; the stopping semantics — which is what makes shadowing real — was supplied by
   the assistant and never confirmed.

8. **Is `@` a `ShareRef` wrap or kind-polymorphic?** Decides whether the share blanket is
   separated from a user `clone` structurally or only by lookup order. If kind-polymorphic, the
   structural separation evaporates and ordering is the sole mechanism.

9. **Is the send predicate narrow or broad?** You deferred it — *"no comment… we'll figure out the
   reasoning later if it matters."* It dissolves under the preview model, but the residue does
   not: phase 0 still cannot tell "target not knowable" from "target known and mismatched".

10. **Does the reversal in item 7 of Part 1 stand?** Conjured impl bounds now merge into the
    solver's conclusions, which you ruled against.

11. **Is `partial_resolve_impl` genuinely read-only?** Never asked, and it is the precedent a
    read-only walk would be built on.

12. **Should the corpus sweep be run now?** You declined it when filter-is-final was ruled. What
    exists is a builtins-only hand spot-check; the stdlib and the Rust fixtures were never swept.
