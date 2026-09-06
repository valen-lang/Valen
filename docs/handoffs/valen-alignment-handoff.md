# Valen alignment — handoff

Semantic alignment between this compiler and the Valen design corpus: the rulings the spec has made that
we have **not yet built**, the places we are **already aligned** (do not "fix" them), and the axes Valen
has that we lack. The design docs **are** the ratification. **None of this blocks the onion arc** — record
and schedule.

The authoritative spelling/transformation rulebook is the Valen convo corpus (the `convo-30-finalize-syntax`
decisions and `convo-30-plan` §3). Pure spelling changes (`^`, `own`→`ownref`, attributes, colons) are a
separate mechanical migration, not covered here.

## Axes Valen has that we lack

- **Immovable types** (way later): `!Movable`, pinned, self-referential state machines. Class instances
  are immovable *while shared* (handles point at them); strong refs to them are movable; `destructure`
  cashes a class out into a movable `Box<T>`. `ownref` exists only to serve this axis — it is the owning
  reference to an **immovable** instance, and rejecting `ownref` on a *movable* type needs this axis we
  don't have.
- **`interface` vs `open trait`** (later, with traits). Classification: a trait is `interface` iff **a
  class implements it AND every class impl fits the ambient-multi cover** (member-level mentions, no
  *external* group params, no *external* stored borrows); else `open trait`. Blanket-impl traits are
  `open trait`. An `open trait` gets **no RC-strong-in-multi form** — an erased collection is homogeneous
  in erasure kind. Accepted cost: a trait implemented by both structs and classes must be `open trait`, so
  its class implementers get boxed. A *declaration keyword* cannot express impl-coverage classification —
  this is the live divergence on `ensure_deep_exports`' `KindT::Interface(_) => {}` fork.
- **Bare class is position-dependent** — `Rc` in storage, `&Rc` as a parameter. Our scout lowers
  uniformly; matching them means a bare class *param* lowering to `BorrowRef(ShareRef(…))` and a bare
  class *field* to `ShareRef(…)`.

## Confirmed as needed

- **Enforce no shadowing.** design-1:47 — *"a name is declared once per scope, so `x = y` on an existing
  `x` is an error, not a redeclaration. Whether two variables appear to share a name must never change
  what a program means."* **We enforce nothing today** — no shadowing-enforcement code in `parsing/`,
  `postparsing/`, or `typing/` (the few "shadow" hits there are doc comments, not checks). The scout's
  stack frame already carries a `VariableDeclarations` list
  (`postparsing/expression_scout.rs`), so a same-scope duplicate check has its data in hand; that's the
  cheapest place to put it.
- **Split `Vec<T>` from `List<T>`.** design-1:92/1164 — `Vec<T>` is a plain **struct** (plain group
  borrowing, no RC, pure-function-legal to build, Send-capable via allocator params, the collection of
  choice for struct fields; `HashMap`/`HashSet`/`BinaryHeap`/`String` sit in this tier). `List<T>` is its
  RC'd **class** counterpart (Valen 2). Both share the `[]` element child group and the same invalidation
  rule: spine in the parent group, elements in a child group via `[]`. We have one notion today.
- **`Vec<int>` elements still form a child group.** design-1:203, called out explicitly as a trap: the
  inline-field exemption covers owned scalar *fields* only, **not** collection elements, because the test
  is whether the *container* can relocate or remove them — which reallocation does regardless of element
  type. Reading it the other way *"makes a spine op invalidate nothing and turns a stale element reference
  into a read of freed storage."*
- **`comptime`.** design-1:110 — the compile-time binding keyword (`comptime <name>: <kind> = <expr>`),
  replacing `let`/`alias`/`var` at that layer. It is also the **carrier for associated groups**
  (`comptime capture_group: group`, `comptime teardown: group = capture_group`), so it's a prerequisite
  for the trait/erasure work, not just a convenience.

## Ruled by the spec, not yet built — the gap inventory

*(These are **gaps, not bugs** — the language rules them and we simply haven't got there.)*

Groups as real values and the `in`-clause grammar · **effect clauses** (the big one) · `comptime` ·
no-shadowing enforcement · `Copy` as a definitional property · the `maybealias` / `in` relation
vocabulary · poisoning and `dangle` · closed traits and the `trait` / `open trait` unification · the
move/state-passing iterator · threading, `parallel`, async · named args, default parameter values,
visibility modifiers · exhaustive-match and refutable-pattern-in-`if` enforcement (**we enforce
neither**) · `if` / `match` as expressions.

Several of these have since picked up rulings in the upstream corpus — check the rulings before treating
any entry as unspecified.

## Already aligned — do not "fix" these

- Our **`held`** treatment matches theirs exactly. Their ratified `held MyClass` is an *anchored payload
  borrow*, "conceptually `&MyClass in (anonymous, rc.*[])`" — identical to our desugar (`&T in e_g where
  maybealias(e_g, rc.__All), held(e_g)`), and `held(e_g)` being a where-clause fact you must *find* rather
  than mint is their "the anchor is found, never made." Cancelling `RegionT::Held` (the region-value rep)
  did not endanger `held` the surface form.
- **The onion already expresses their whole class-reference model**: bare class field/local =
  `ShareRef(Struct(C))` (`Rc`); bare class param = `BorrowRef(ShareRef(Struct(C)))` (`&Rc`, their anchored
  borrow); `&MyClass` = `BorrowRef(Struct(C))` (payload borrow); `weak MyClass` = `WeakRef(Struct(C))`.
  **Not** eagerly decaying `BorrowRef(ShareRef(X))` → `ShareRef(X)` is what preserves the `&Rc`-vs-`Rc`
  distinction their model rests on — keep it that way.
- Their "storing a borrow into a bare-`MyClass` slot **incs** — the slot drives it" is exactly the
  `BorrowRef(ShareRef(SC)) → ShareRef(SC)` RC-bump shape. Their `&MyClass → Rc` minting is **deferred** on
  their side (the "Horn A/B" problem); the coercion table correctly has no such row.

## Interop projection (live on the extern/export front)

Valen rules that **interfaces get no representable Rust type** — Rust holds an *opaque handle* and calls
functions through it, nothing more. Only the `open trait` / `Box<dyn>` half projects to a real Rust
`dyn`, so an exported observer registry must be spelled `Vec<Box<dyn EventHandler>>`, not
`List<EventHandler>`. Under that rule `tests_exporting_interface` means exporting an opaque handle, not a
dyn-shaped type — **so that test may be asserting the wrong thing.**

## Asking Valen semantics questions

The Valen design session is reachable by mailbox — it answers from the corpus with citations, marks
inference as inference, and routes genuinely open questions to the architect. **Check `mailbox list` for
the current identity**, since they turn over. Ask it before deciding a semantics question locally; the
corpus lookups are the expensive half.

## Lessons learned

*Accumulates wisdom, not events. One or two sentences per entry; prune what nobody can act on.*

- **A surface spec never names internal machinery.** "The doc doesn't mention X" is NOT an alignment
  finding — only "the doc rules X, we do Y" is. Check whether a thing implements a stated surface rule
  before concluding it is orphaned (`&&`, `DerefTE` were both false positives of this shape).
- **There is one architect, not two.** When a design doc contradicts a ruling, the live hypothesis is
  *"the doc is behind,"* never *"two authorities disagree"* — hunt for the ruling the doc hasn't caught up
  with, not for a divergence.
- **A stale spelling reads exactly like a divergence.** Corpus examples are design evidence, never
  conformance fixtures; check a passage's date before taking its spelling as authoritative.
- **A name-based sweep is not a semantic one.** Check what a symbol *is* before trusting what it is
  called — an alignment gap named by spelling may not be the gap it looks like.
