# Call-site / overload / dispatch model — handoff

The ratified design for how a call resolves: candidate lookup, the phase pipeline, overload resolution,
and the namespace-based dispatch model. This is the frame every "where does this belong?" question about
solving, deduction, coercion, or dispatch resolves against.

**►► THE CURRENT DESIGN LIVES IN `docs/plans/plan-phased-calls.md` ◄◄** That is the authoritative 8-phase
model, and it **supersedes the 6-phase model below** — where they disagree, the plan wins. It retires the
sends machinery (an `ArgumentStep` matches the argument `KindT` against the parameter's `ITypeST`), the
rune-type solver, and `complex_solve`; the reject-the-losing-candidate solver arms (`KindIsNotBorrowRef`
and kin) become ordinary match failures under it. The 6-phase model is kept here for the parts the plan
reuses — namespace membership, filter-is-final's intent.

## CALL-SITE PHASES — the 6-phase model

Every call site runs this sequence, **on the one candidate the static filter selected**.

| # | phase | owns |
|---|---|---|
| **0** | **prepare** | preview each parameter's type as far as it is statically known, adjust the argument to match — auto-ref, auto-move, auto-deref, upcast — then send it at the parameter's rune |
| 1 | rune-typing | what *type* each rune is (Kind / Integer / Template / …) |
| 2 | value-solve | what *value* each rune has. **Structural deduction only** |
| 3 | resolve | the citizen/function resolutions phase 2 postponed per SFWPRL, plus declared bounds (`implements` via `is_parent`; `not(mut(..))` when effects land) |
| **4** | **convert** | perform the conversion phase 0 previewed, against the now-concluded parameter type, and emit the code |
| 5 | borrow check | joint-argument + use-after-churn; a whole-function two-phase walk (`check_function`), not a call-site phase |

**Candidate lookup is not a phase.** It encloses the sequence rather than sitting in it: it takes a name
plus argument types and *produces* the callee, so it cannot be a step in a sequence that presupposes one.
It searches the namespaces of the **peeled** value type — @PFVSZ's stated rationale, *"the typing pass
ignores the outermost references when looking for functions to call."*

**►► A strong ref contributes its payload's namespace too, as an ORDERED union ◄◄** A
`ShareRef(Struct(Ship))` argument contributes **both** the strong-ref namespace and `Ship`'s, with the
strong-ref one searched first. This is Rust's arbitrary-self-types shape — `impl Ship { fn foo(self:
Rc<Self>) }` puts a method whose receiver is `Rc<Ship>` into *Ship's* namespace — and it is **automatic
for strong refs only, not a general user feature**: no user-defined smart pointer gets to claim another
type's namespace.

**`Box<T>` reaches its payload too, but by `[]`, not by a `Deref` trait — Valen has none.** design-1:1508
makes a Box's target one of the `[]` path segments, *"collection element / Box deref. Child group.
(Unified across `Vec`, `List`, `Box`, arrays.)"* — **do not justify Box's reach by analogy to Rust's
`Deref`**, or it will later read as evidence that Valen has that concept; the mechanism is `[]` and it is
already unified.

**This is not optional, and it is not really about `clone`.** A bare class parameter is an anchored
borrow, `BorrowRef(ShareRef(Struct(C)))`. If namespace membership cannot see through `ShareRef`, then
`func launch(s Ship)` in `ship.vale` is in *no* namespace at all. That is **every method on every class**,
not an edge case.

**Ordered ≠ tiebreaking.** Ordering governs *which namespaces are searched* — candidate-set construction,
which already has rules ("the namespaces of the arg types"). No-tiebreaking bans preferring one candidate
over another *within* the set. Different layer, no conflict. The payoff: a user's class method shadows a
same-named strong-ref-flavored builtin — exactly as Rust's inherent impls beat trait impls.

**This closes the "does `&Ship` mention `Ship`" open question** — yes, and a strong ref mentions its
payload as well. Call side and declaration side must see through identically or they never rendezvous.

**Selection happens before the phases, and it is where the whole call resolves:**

```
lookup by name                                     ← the candidate set
static filter — arity, wrap chain, value template  ← NO solving
    0 → not found · 1 → win · >1 → ambiguity error
phases 0 · 1 · 2 · 3 · 4 · 5                       ← on the winner, once
```

**There is no per-candidate loop.** Because the filter is final, solving never eliminates anything, so
exactly one candidate is ever solved and nothing speculative is discarded. A per-candidate phases-0–3 loop
with 4–5 on the winner is the shape to expect if you reason from the current code (`attempt_candidate_banner`
still works that way) — but it is what filter-is-final replaces. Independently, phase 4 could never have
run speculatively: `convert()`'s `&NC→NC` arm stamps a monomorphization into `coutputs`, so running it for
a losing candidate would emit code for a call that never happens.

**Why the phases exist at all** — STCMBDP (`docs/Generics.md`, the *"declaring a function needs its param
types; knowing those needs its requirements checked; checking those needs the function to exist"* triangle;
2023-era but the reasoning is durable): one link has to break, and the choice was **check calls later**.

## Phases 0 and 4 are the same conversion, previewed then committed

Not two different kinds of conversion sorted by what is knowable.

**Phase 0 previews the parameter type as far as it is statically determined — the wrap chain from
`type_outer_ref_rules`, plus the value type with any explicit template args substituted — and adjusts the
argument to match: auto-ref, auto-move, auto-deref, and upcast.** It is pure. **Phase 4 performs that
conversion and emits the code**, on the winner, once.

Phase 4 cannot move earlier, and the reason is side effects rather than knowledge: `convert()`'s `&NC→NC`
arm stamps a monomorphization into `coutputs`, so running it speculatively would emit code for a call that
never happens.

**Any part of phase 0 that must *emit* rather than constrain inherits phase 4's problem.** This is where
rustc's own separation breaks: `coerce_unsized` drives `SelectionContext::select` directly from inside
coercion, because a coercion must decide whether to write an adjustment and there is nowhere to record
*maybe*. rustc's escape is a whole-body writeback pass that rewrites adjustments after the fact — a stage
we do not have, so a phase 0 that emits cannot be fixed up later.

`foo<T>(x &T)` called with an owned `Ship` shows why phase 0 must exist: the wrap chain reads statically as
`[BorrowRef]`, so the argument is adjusted to `BorrowRef(Struct(Ship))` *first*, and only then does a send
give the right answer. Seeding the raw `Struct(Ship)` into the `BorrowRef` result rune errors with
`KindIsNotBorrowRef` (`solve_rule`'s BorrowRef arm) rather than concluding a wrong value.

**Do not read the split as "shape conversions before the solve, upcasts after."** A value type is
statically known whenever it has no unsolved runes left — fully concrete, or with every rune pinned by an
explicit template argument — and phase 0 upcasts in both. The rule is *as early as the target is known*.

**The wrap rules do the peeling — do not hand-peel.** `type_outer_ref_rules` are ordinary bidirectional
rules in the solve (`get_puzzles(BorrowRef) = [[inner], [result]]`). Seed `full_type_rune` and the wrap
rule fires in its peel direction and concludes the inner rune for free. `solve_rule`'s `BorrowRef` arm
peels correctly — result-known concludes the inner rune, inner-known concludes the result rune — so the
mechanism phase 0 depends on is in place (this was once feared a wrong-rune defect; it is not).

## The wrap chain's source (`type_outer_ref_rules` is not the sole source)

`translate_signature_type_st` (`postparsing/rules/templex_scout.rs`) splits a parameter's `ITypeST` on
**written syntax** — its recursive peeler (`split_type_st_into`) peels every `BorrowRef` / `WeakRef` /
`OwnRef` layer into `type_outer_ref_rules` and hands the value root to `translate_type_st_into_rune` for
`value_type_rules`. A parameter written bare as `x MyClass` has no wrap layer, so it gets
`type_outer_ref_rules = []` and `full_type_rune == value_type_rune` — even though its real type is the
anchored borrow `BorrowRef(ShareRef(Struct(MyClass)))`.

**The position rule is not implemented, and it cannot be scout-time.** `ShareRefT` has six live
construction sites and none is a position rule. Classness is a property of the citizen's *definition*, so
the scout genuinely cannot know it. design-2:162: the anchored borrow *"is what bare means in a parameter,
**which works only where the position rule can see the position**"* — and inside a generic bound it cannot,
since `T` is opaque, which is why `@` was minted. **So it is a typing-time interpretation, not surface
sugar.**

**What this costs — less than it looks.** The chain a candidate is filtered on is *written wraps ++ the
position rule applied given the citizen's kind*, and the second half is a **lookup, not a solve**. The
filter already reads each parameter's value-type *template name*; from the name you have the definition,
and the definition carries sharedness (via `struct_compiler_get_sharedness`, which reads `lookup_struct(...).sharedness`).
**Filter-is-final survives intact.** Two riders: phase 0 has the identical dependency (one fix, not two);
and a bare-rune parameter `x T` is genuinely undecidable at filter time because bare `T` is
kind-polymorphic (design-1:387) — the filter's "bare rune accepts anything" arm absorbs this, but by luck
rather than by design; confirm it deliberately.

**Where the wrap chain lives is still open** — the shape is known (chain = written wraps ++ the position
rule given the citizen's kind, the second half a lookup) but its home is not. Blocking nothing today, since
neither the filter nor phase 0 is built.

## Lookups come out of the solver

`LookupSR` is discharged in a pre-pass: resolve the path, seed an `InitialKnown`, strip the rule, so the
solver never sees one. `RuneParentEnvLookupSR` is the in-tree precedent and its solve arm in
`compiler_solver.rs` is a `vwat` panic stating exactly that recipe. `Lookup` is the cleaner case —
`get_puzzles` gives it an empty puzzle, and its arm reads no conclusions and never touches
`CompilerOutputs`, only `env.self_env` and the path.

**The ordering change is wanted, not tolerated.** An unknown name is the easiest thing a user can fix, so
those failures belong before the solve. Consequence to expect: in an already-erroring program a
`SolverConflict` may name a different rune, and some solver tests assert on humanized text.

**One thing stops it being a single pass.** `GenericParameterDefaultS` carries its own `rules`, injected
mid-solve by the @DRSINI callback, and a default like `= int` contains a `Lookup`. The pre-pass must walk
`generic_params[].default.rules` as a second source, and decide what happens when it concludes a default's
runes and the default never fires.

**The general shape.** The solver's rule kinds split in two: structural (`Equals`, `BorrowRef`, `WeakRef`,
`OwnRef`, `KindList`) and resolution (`Lookup`, `Call`, `Resolve`, `DefinitionFunc`, `CallSiteFunc`). Phase
2 is structural deduction only, so the resolution half is phase-1 and phase-3 work implemented as rules —
`Resolve` is literally SFWPRL's postponement wearing rule clothing. `Lookup` moves first because it is the
only one whose arm reads nothing.

## No most-specific-common-ancestor

`launch<T>(a &T, b &T)` called with a `Firefly` and a `Serenity` is a **type error**; the user writes the
erasure. `T` unifies exactly — first argument wins, the rest must match. Rust rejects the same program, and
its LUB machinery (`try_find_coercion_lub`) is reachable only from match arms, if/else, loop/break, array
literals and the return coercion — **never from call arguments**. This retires SMCMST/CSALR, which chose
"halt, then guess the most specific," and is consistent with three decided things: Valen refuses variance
outright, the overload redesign says *no specificity, no fallback, no tiebreakers*, and `complex_solve` is
already dead.

**A generic argument that needs an upcast must be written explicitly** — `launch<int>(&Firefly<int>())`,
not `launch(&Firefly<int>())`. Rust's rule, chosen because it simplifies the implementation. Deduction
through an upcast *is* mechanically possible (`get_impl_parent_given_sub_citizen` seeds from the sub side
and reads the super side out) but we deliberately do not do it. Consequence: **no impl walking anywhere in
phases 0–2.** Phase 4's upcast runs between an argument whose type is known and a parameter type fully
concluded; that is `convert()` / `convert_via_upcast`'s existing job.

## A send is `Equals`, and every parameter gets one

No guard, and no predicate deciding which runes are eligible. **The preview ordering is what makes that
safe**: by the time a send fires, phase 0 has already adjusted the argument to whatever the parameter was
statically known to be, so the send either agrees with the rules or carries the only information there was.

| case | outcome |
|---|---|
| rune undetermined, one argument sends to it | seed; nothing to conflict with |
| rune undetermined, two arguments send to it (`f<T>(a T, b T)`) | both fire; agreement is a no-op, **disagreement is the type error no-MSCA requires** |
| `f(x &ISpaceship)` called with `&Firefly` | parameter is statically known, so **phase 0 upcasts the argument first**; the send then agrees |
| `launch<int>(&Firefly<int>())` against `&ISpaceship<T>` | `T` is pinned by the explicit arg, so the target reads statically as `ISpaceship<int>`; upcast in phase 0, then agree |
| `foo<T>(x &T)` with an owned `Ship` | phase 0 auto-refs first, so the send seeds `BorrowRef(Struct(Ship))` rather than the bare kind |

The one case a seed could break — a determined rune meeting a differing argument — is exactly the case
where the parameter type was knowable, which is exactly the case phase 0 converts. That is what
`CoordSendSR`'s deleted coercion-tolerance branch was working around, and ordering removes the need for it.

**Explicit `T` is what makes the preview total.** `f<T>(a T, b &ISpaceship<T>)` with no explicit args would
need argument 0 solved before argument 1's target is knowable — interleaving sends with solving, which is
phase 2's job leaking into phase 0. The explicit-`T` ruling forbids that program, so phase 0 stays a single
static pass rather than a fixpoint.

**Where the machinery already is.** `assemble_initial_sends_from_args` (`function_compiler_solving_layer.rs`)
builds `InitialSend { sender_rune, receiver_rune, send_templata }`, and its result **is consumed** at three
solving sites: each `InitialSend` becomes an `InitialKnown` plus an `Equals` (sender → receiver) rule and a
`KindTemplataType` for the sender rune, all fed into the solve, threaded through `solve_for_defining` /
`make_solver_state`. (A separate `old_assemble_initial_sends_from_args` feeds a fourth site.) The sends
already go against the **peeled** `value_type_rune`, not `full_type_rune`. So this producer is already
wired, not awaiting threading — what remains is the phase-0 shape adjustment on top of it.

## Overload resolution

**Single rule.** Collect all candidates whose params match the args. If 0 → "no function found." If 1 →
win; if that candidate has bound-resolution failures or other rejections, surface THOSE specific reasons
directly (don't wrap in `CouldntFindFunctionToCallT`). If >1 → ambiguity error; user disambiguates
explicitly.

**No specificity, no phases, no fallback, no tiebreakers.** Two equally-matching candidates is always an
ambiguity error. The one thing that looks like an exception and is not: the namespace union is *ordered*,
so a user's class method shadows a same-named builtin blanket — that is candidate-**set construction**, not
preference *within* the set. Rust draws the line in the same place, inherent impls before trait impls.

**►► THE FILTER IS FINAL, AND IT IS PURELY STATIC ◄◄** "params match the args" is decided **before
value-solving**, from information available with no solving:

- **arity**
- each parameter's **wrap chain** — `type_outer_ref_rules` is a list of `BorrowRef` / `WeakRef` / `OwnRef`
  rules, so the variants are readable directly
- each parameter's **value-type template name**, or "it is a bare rune, which accepts anything"

**►► THE WRAP CHAIN IS COMPARED UP TO ADJUSTMENT, NOT BY EQUALITY ◄◄** The admissible gaps are exactly what
**phase 0 can later deliver** — auto-ref, auto-move, auto-deref — and nothing else. That is the invariant:
*the filter admits exactly what phase 0 can perform*. Reading "compare the wrap chains" as equality breaks
phase 0's motivating case, `foo<T>(x &T)` called with an owned `Ship`. Two gaps are **not** admissible:
**bare to `ShareRef`** (minting a claim from a payload is deferred upstream, so a claim parameter is an
exact requirement) and **borrow to double-borrow** (the coercion table has no such row, and `&e` on a
borrow-typed place yields `&T`, so `clone(x &&T) &T` is rejected against a `&Ship` argument on shape
alone).

**Whatever survives that filter is the answer.** Solving never eliminates a candidate. *Which function am
I calling* never depends on generic inference.

**►► THE FILTER IS LOOSE; BOUND RESOLUTION IS EXACT. DIFFERENT LOOKUPS ◄◄** They read as contradictory
until you notice they are not the same mechanism — bound resolution requests the exact match by threading an
`exact` bool through `find_function` (`overload_resolver.rs`), reached via `resolve_function_call_conclusion`
(`infer_compiler.rs`). (There is no `resolve_function` in `typing/compiler.rs`.) The `&&T` distinctness
argument lives entirely in that exact half.

The accepted cost: a program with two structurally-matching overloads where only one would typecheck is
**rejected**, not silently resolved. Example — `foo<T>(a Vec<T>, b T)` and `foo(a Vec<int>, b str)` called
with `(Vec<int>, str)`: both pass the static filter, only the second solves, and the user must
disambiguate. Expected to be thin.

## A name is declared at most once per namespace — Vale has no overloading

> Two functions with the same name in the same namespace are an **error at the declarations**, whether or
> not their parameter shapes overlap. The user renames one.

**The stricter rule is also the cheaper check.** Overlap asks *does some substitution make these accept one
tuple*, a unification search. Name identity is a failed hashmap insert.

**This makes Vale a language with no overloading**, in the Cardelli–Wegner sense. What remains is
*namespace-qualified lookup*: an argument's type selects the namespace before any candidate set exists,
which is structurally what a receiver type does when it selects a Rust inherent impl. Do not describe
`foo(x int)` in `int.vale` and `foo(x str)` in `str.vale` as overloading; those two are never candidates
for one call.

**What survives as genuine multiplicity is the cross-namespace union**, since a call searches the
namespaces of *every* argument type. That is why the `>1 → ambiguity` backstop stays live.

**Consequence for the builtins: `drop.vale`, `arith.vale`, `clone.vale` and `logic.vale` dissolve** into
per-type files — `drop(x int)` into `int.vale`, and so on. Required rather than cosmetic: legality is now
per-namespace and those files are not any of their types' files, so today's eight `drop` declarations live
in no namespace at all.

**►► `drop<T>(v void, x T)` IS DEAD — DELETE IT RATHER THAN REHOMING IT ◄◄** It exists only to satisfy a
bound spelled `where D Prot = func drop(void, E)void` which no longer exists anywhere (`grep "drop(void"`
returns nothing). Its one live effect is a cost — every program importing `v.builtins.drop` scouts its
`where func drop(T)void`, the `KindList([T])` that stood in front of 35 of one cluster's 38 tests. Deleting
it may green something; verify by deletion and a suite run.

### The rule makes an ordinary class of program unwritable — UNRESOLVED

Bare/borrow twins are the *easy* collisions (9 in `arith.vale`, 4 in `clone.vale`, 2 in `logic.vale`), and
C1 retires those. The rest is not that shape:

- **Cross-product overload sets over two concrete types.** `stdlib/src/str.vale` declares `StrSlice`
  beside the `str` operations and holds the full cross product — `contains` ×4, `find` ×4, `==` ×4, `<=>`
  ×4, `slice` ×5. Each mentions *both* types, so each lands in **both** namespaces.
  `tests/castutils/castutils.vale` has six `+` over `int`/`str`/`bool`/`float`.
- **Owned/borrow abstract pairs with different semantics.** `opt.vale`'s `get<T>(opt Some<T>) T` beside
  `get<T>(opt &Some<T>) &T` — different return types, one consumes and one borrows.

**The load-bearing consequence: a user cannot write `println(int)` and `println(str)`**, because they do
not own `int.vale` or `str.vale`. Rust expresses these through trait impls — a second axis of
qualification. Vale's only axis is the file a type is declared in. **No replacement has been named, and
this is not a migration detail.**

**The corpus bends to it where it can, and the resolutions are decided:** drop `vassert(bool)` and the
one-arg `vassertEq`, keeping the `msg str` forms; rename the two-argument `HashMap`/`RHashMap` into helpers
calling their three-argument versions; delete `Array<E>(size int)`; rename `arith.vale`'s unary/binary
minus to `__negate`/`__subtract`; give the `opt.vale`/`result.vale` owned-borrow pairs distinct names.
`borrow.vale` and `void.vale` are owed (`drop<T>(x &T)` belongs to `&T`'s namespace; `drop(x void)` /
`clone(x void)` to void's). **Arity does not separate two declarations — only names do** (a per-name-and-arity
relaxation may come later; build nothing assuming it).

**Functions whose parameters are all bare runes** mention no concrete type, so they belong to *no*
namespace and are import-only — name-uniqueness has nothing to key on. Both live instances are ruled:
`===` is never overloaded by anything, and `as.vale`'s pair is already distinctly named — currently
`try_as` (borrow-taking) and `try_take_as` (owning). **`has.vale` needs no change** — its four `has` declarations live in a
file declaring no type, so the static filter separates them on arity and value-type template. **Import-only
functions are where overloads live**, by design.

## Dispatch model — namespace-based, no Self specialness

**`x.foo()` and `foo(x)` search the exact same candidate set.** No Self-based namespace, no separate
dispatch path for dot-syntax.

**Dot is NOT *pure* sugar; it performs a receiver adjustment.** A "purely sugar over the free-function call
form" reading is refuted by the corpus: `keys.append(*k)` has an **owned** receiver and `append` takes
`&self mut`, so `.` must **autoref**; `set self.hp -= 10` has a **borrow** receiver, so `.` must **deref**.
(The argument's `*` is unrelated — arguments do not adjust; only the receiver does.) What survives is the
**namespace** half: adjustment changes the *receiver's type*, not *which functions are findable*. So the
correct statement is **"`.` is sugar for the free-function form after a receiver adjustment."**

**Still open upstream:** the exact adjustment rule. A deref-first proposal was floated and **withdrawn**
(it flipped the RC default versus Rust and solved the wrong problem). The current shape under adversarial
review is *"where a handle and its referent are distinct, `.clone()` reaches the referent, so no
handle-duplication operation is a `.clone()` candidate"* — which would make the auto-derived reference
clone **bound-only machinery**. Don't build the adjustment rule until it lands; the *namespace* work is
unblocked either way.

**Namespace membership rule.** A function lives in type T's namespace iff it is defined in T's file **and**
either: (b1) it mentions T in a parameter, **or** (b2) it is named T.

**(b2) is what makes constructors reachable at all.** A synthesized constructor's parameters are the
struct's *member* types — for `struct Some<T> { value T; }` that is `Some(value T)`, which mentions no
concrete type — so under (b1) alone it would be in no namespace. The clause also rescues every function
that names its type only in the **return**: `str(x int) str`, `float(x &int) float`.

**"Mentions T in a parameter" sees through the reference wraps.** `&Ship` mentions `Ship`, and a
`ShareRef(Struct(Ship))` mentions **both** the strong-ref namespace and `Ship` (the ordered union above).
Declaration side and call side must see through identically or they never rendezvous: the call side peels,
so the declaration side must too.

**Lookup rule.** The resolver collects candidates from (1) the union of namespaces of every arg type at the
call site, plus (2) any function explicitly imported into the current scope (utility functions like `min(a,
b)` in `math.vale`). Both sources contribute; the strict ambiguity rule applies to the combined set.

**No "first parameter is special."** `foo(ship, rocket)` looks in BOTH Ship's namespace AND Rocket's. This
generalizes Rust's Self-dispatch. **Per-call lookup, no namespace import ceremony** — having a value of
type T at a call site is enough to make T's namespace searchable for that call.

## Typeclass-like operations (clone, drop, eq, hash, …)

Language provides two blankets per op (borrow and share flavors); user provides Own-flavored
implementations per type:

```vale
// the borrow-flavored blankets — for when T is a borrow type
// (no borrow.vale exists yet: the drop blanket below is live in drop.vale;
//  the clone blanket is currently commented out in clone.vale)
func clone<T>(x &&T) &T { x }              // satisfies clone(&T)T bound
func drop<T>(x &T) Void { }                // satisfies drop(T)void bound

// at class kind there is NO written blanket — the compiler synthesizes the claim
// clone (the RC bump), the way it synthesizes a struct's drop; see below

// in ship.vale — user-defined for their owned type
func clone(s &Ship) Ship { /* deep copy */ }   // user's deep-copy
func drop(s Ship) Void { /* destructor */ }    // user's owned destructor
```

**No `clone` for own types by default.** `clone` means "get another handle to this thing" — for borrows
that's the ref, for share that's an RC bump, for owns the user must opt in.

**►► THE SHARE CLONE IS NOT A WRITTEN FUNCTION — RATIFIED ◄◄** The bound shape is plain `clone<T>(x &T) T`
with `mut(E)`, and it has two satisfiers: at class kind the **compiler-synthesized** claim clone (a real
function performing the inc, the way a struct's `drop` is synthesized), and at struct kind the user's
hand-written deep copy. A rune holds whatever unification hands it, so at a claim the bound already takes
`&@Ship` and returns `@Ship`; there is no second blanket function and `@` never appears in a user-written
clone.

**►► ORDERING IS WHAT KEEPS THIS APART FROM A USER `clone`, AND IT IS THE ONLY MECHANISM ◄◄** The ordered
union searches the strong-ref namespace before the payload's. Wrap-depth distinctness does **not** back it
up: `@` normalizes to identity at struct kind, so the claim-flavored signature at a struct is the same
depth as the user's. Treat ordering as load-bearing on its own.

## Bound resolution

A bound `where exists clone(&T) T` becomes a namespace-scoped exact-match lookup at instantiation time,
simulating a call:
- For T=Ship: look in Ship's namespace for `clone(&Ship) Ship`. Find user's. ✓
- For T=&Ship: look in `&T`'s namespace for `clone(&&Ship) &Ship`. Find the borrow blanket. ✓

**Bound resolution does NOT coerce.** It's an exact-shape lookup. Auto-borrow at call sites is for
direct-call resolution only; the bound mechanism never coerces.

## Drop is move-only

`drop(bare_local)` is a compile error. Only `drop(^local)` is valid. Auto-drop insertion at scope-end emits
`LoadAsP::Move` so the drop call receives the correct ownership. This eliminates the "user thinks they're
consuming but the borrow blanket silently no-ops" landmine.

## Open questions

- **Telling phase 0's two failures apart** — *"the target is not knowable yet"* (the explicit-`T` error)
  versus *"the target is known and the argument does not match"* (an ordinary type error). Both present as
  "cannot convert this" and want different diagnostics.
- **Can phase 0 be a single static pass, given lambdas?** rustc's `check_argument_types` runs a two-pass
  loop (non-closures, a solver drain, then closures) because a closure literal's parameter types are
  deduced from the expectation, which the *other* arguments must pin down first. Explicit-`T` makes the
  preview total for *type* arguments and says nothing about a lambda whose signature depends on a sibling
  argument. **Do not treat "phase 0 is one pass" as settled until a lambda case is worked through.** If it
  needs speculation, the shape to copy is rustc's `fudge_inference_if_ok`: guess inside a rolled-back
  snapshot, keep only a hint, re-verify for real.

## Lessons learned

*Accumulates wisdom, not events. One or two sentences per entry; prune what nobody can act on.*

- **Reason from the ratified design, not the current code.** `attempt_candidate_banner` still runs a
  per-candidate phases-0–3 loop; that is the shape filter-is-final *replaces*, not the target. In a
  mid-migration tree most of what you read is the thing being migrated away from.
- **A conditional in a rule costs you the reverse direction; a conditional at a site costs nothing.** A
  rule that inspects a value to decide what to produce cannot be inverted — decide where the answer is
  known and emit an unconditional rule.
- **The filter admits exactly what phase 0 can perform.** Reading "compare the wrap chains" as equality is
  the natural mistake and it breaks phase 0's own motivating case (`foo<T>(x &T)` with an owned `Ship`).
- **A rule that outlaws a shape moves programs rather than deleting them.** Name-uniqueness pushes
  cross-product overloads into import-only files, the one bucket no declaration-time check reaches — so a
  diagnostics motive can defeat itself unless the destination is checked too.
- **Take work out of the solver wherever it can reasonably go.** Name resolution and callee resolution
  living there as rules is complexity to relocate, not to accept; `Lookup` moves first because its arm
  reads nothing.
