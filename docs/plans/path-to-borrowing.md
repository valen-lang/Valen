# The Path to the Borrow Checker

The onion refactor is done and the suite is green. **Rung 0 (groups become real), rung 1's
joint-argument check, and rung 2's use-after-churn dataflow are all landed and green.** This document
maps the path: what is built, what remains, and in what order.

Rung 0 made groups expressible on the declaration side, without putting them on the value type, so
the checker has something to read; on its own it accepts and rejects nothing new. The joint-argument
check (rung 1) is the first thing that rejects programs. Rung 2 adds the checker's first flow-sensitive
analysis: a reference into a **child group** (a runtime-sized-array element) is invalidated by a call
that churns its parent group, and using it afterward is a compile error.

Keep the split in mind: rung 0 is representation and plumbing; checking is rung 1 and up.

## Where it stands

Rung 0's representation, rung 1's joint-argument check, and rung 2's use-after-churn dataflow are
landed (see "What is already done"). The checker lives in `src/typing/borrow_checker/` and its
high-level design doc is `src/typing/docs/borrow-checker-guidelines.md`. What remains: extending
child groups past the runtime-sized-array element (struct-field arrays, `Box`, `Variant`/interface
payloads, and generic `Vec<T>` — the rung-2/rung-3 boundary, revisit before scoping); effect
*checking*; and the group-syntax the parser does not yet produce (`Member`/`Elements`/`Union`,
`g...`, `rc`). The design is settled; this is build work.

## What rung 0 is, and what it is not

Rung 0 delivers exactly three things:

- **Syntax**: parse and scout group declarations (`<g'>`, `<g': T>`), the `in g` clause, and the
  minimal `mut(g)` effect clause.
- **Group representation on the declaration side, never on the value type**: `GroupP`/`GroupS`
  enums (parse/scout) carry group *syntax*; the group/effect metadata lives on the scout-side
  `FunctionS` (each `ParameterS.tyype: ITypeST`, plus `FunctionS.effects`), read directly at the
  checker seam and never copied onto the durable header. The value type (`KindT`) carries no
  group — `BorrowRefT` is emptied to `{ inner }`.
- **A ceremonial group-param value**: group generic params stay uniform with type/int params, with
  a constant `ITemplataT::Group(Default)` value that never enters a `KindT`. Groups never flow
  through the solver.

Rung 0 adds **no new errors** (well-formedness only). The first real check, and *checking* of
effect clauses like `mut(g)`, arrive at rung 1 (a second solver domain, still partly undesigned) —
out of scope here, and something rung 0 must not block. Rung 0 builds the effect clause's
*representation* (`FunctionS.effects`, an `EffectS`); rung 1 consumes it.

## What is already done, so we do not rebuild it

The onion work left more of the foundation in place than the old handoff suggests. Verified
against the current tree:

- **The group data structures, the parser/postparser group syntax, the rung-1 joint-argument check,
  and the rung-2 use-after-churn dataflow are landed, and the crate compiles green** (`cargo nextest
  run --lib --manifest-path Cargo.toml`: measure before quoting; last 755 passed / 0 failed / 70
  skipped). `BorrowRefT` is emptied to
  `{ inner }` — the `region` field is gone, and the `region: RegionT` fields on the name-structs go
  with it (`ExportNameT` done; `RawArrayNameT`/`ExternNameT` still carry theirs). The scout produces
  the symbolic forms: a borrow's `&T in g` lowers to `RegionS::Group(GroupS::Rune | Local)` on the
  `ITypeST` (resolved via `env.all_declared_runes()`), `<g': T>` parses into
  `GenericParameterP.maybe_group_type`, and `mut(g)` / `not(mut(g))` scout onto `FunctionS.effects`
  as `EffectS`. `ParameterS.tyype: ITypeST` is populated — built first, its `@PFVSZ` rune split
  derived from it. The types `GroupP`/`GroupS`/`GroupB`, `EffectP`/`EffectS`/`EffectB`, and
  `ITemplataT::Group(GroupTemplataT)` exist; `GroupB`/`EffectB` live in `src/typing/borrow_checker/`.
  **The rule-side region erases the group to `RegionSR::Unspecified`** (groups never reach the
  solver); the group survives only on the `ITypeST`.
  - **The checker does the joint-argument check.** `check_function` (`src/typing/borrow_checker/borrow_check.rs`),
    called at the tail of each user-body typecheck (`function_compiler_core.rs:358`, after
    `coutputs.add_function`), walks the finished body, collects call nodes, and for each reads the
    callee's declared groups (`ParameterS.tyype`) and `mut` effects (`FunctionS.effects`) off the
    scout `FunctionS`, then rejects two arguments aliasing into distinct mutated groups, and a borrow
    argument rooted in a moved argument's local. Argument identity is a `PlacePath` (root + member
    segments) in `place_path.rs`; the check is in `call_check.rs`; errors are `BorrowErrorKind`
    (`ICompileErrorT::BorrowCheckError`) in `borrow_error.rs`. It reads `GroupS`/`EffectS` directly by
    name; it does **not** mint `GroupB`. Its `collect_calls` walk descends the statement positions a
    nested call can hide in (`LetNormal`/`Return`/`Mutate` alongside the control-flow nodes), and its
    match is **exhaustive** so a new `ExpressionTE` variant cannot silently reintroduce a walk gap; the
    remaining un-descended child-bearing variants (`ExternFunctionCall`, `InterfaceFunctionCall`,
    member/array lookups, `Tuple`, `Construct`, `LetAndLend`) are known gaps to close with a red test
    when a reachable nested call appears there.
  - **A group param concludes to the ceremonial `ITemplataT::Group(GroupTemplataT {})` constant**,
    minted in `create_placeholder`'s `GroupTemplataType` arm; it is inert (substitutes to itself,
    contributes no placeholder). Real region params still mint a `Placeholder`.
  - **The checker also does use-after-churn (rung 2), the first flow-sensitive analysis.**
    `liveness.rs` walks the finished body threading which locals hold a **child-group** reference (an
    array element) and which a churn invalidated. A `Segment::Element` in `place_path.rs` plus
    `is_child_group()` distinguishes an element reference (child group) from a whole-array or member
    reference (parent group, immune). Invalidation is **root-matched and `mut`-gated**: a call
    invalidates only element references rooted in an array handed to a `mut`-group parameter (reusing
    `call_check`'s `param_group_name`/`is_mut_target`). Three dataflow disciplines: straight-line
    in-order threading (a fresh binding clears its local's invalidation), `if`-join **union** with a
    diverging arm (`KindT::Never`) contributing nothing, and a `while` **least-fixpoint** over the
    back-edge (silent passes until the loop-head invalidated set stabilizes, then one reporting pass).
    The error is `BorrowErrorKind::UseAfterChurn`.
  - **What the checker still does not do.** It compares group runes **by name** (via the `PlacePath`
    root and `param_group_name`); `GroupB` is defined but never *minted* — rung 2 did not need it, and
    minting waits for richer group expressions (`Member`/`Elements`/`Union`) that a name comparison
    cannot resolve. Child groups are only array **elements** so far — struct-field arrays, `Box`
    pointees, and `Variant`/interface payloads are not yet child-group sources, and generic `Vec<T>`
    is untouched. Only the single-named-group leaf is parsed — `Member`/`Elements`/`Union`, multi-group
    `mut` folds, `g...` and `rc` are variants the enums carry but the parser does not yet produce.
    `liveness.rs`'s walk has a `_ => Ok(())` fallthrough: a use nested in an unhandled child-bearing
    node (`Tuple`, `Construct`, array ops) is a false negative, the same gap shape as `collect_calls`'
    — close each with a red test when a reachable use appears there.
- **`substitute_templatas_in_kind` handles all four ref wraps** correctly
  (`typing/templata_compiler.rs:540-552`); once `BorrowRefT` is emptied there is no region for it to
  carry through generics, so that concern disappears rather than needing wiring. The only remaining
  `unimplemented!()` ref arms are in the dead `is_descendant_kind` / `is_ancestor_kind`, whose
  callers are commented out.
- **Group *parameters* already have a mint path** to rework: `create_placeholder`
  (`typing/templata_compiler.rs`) mints an opaque `ITemplataT::Placeholder` tagged
  `RegionTemplataType` for a region generic param, which rung 0 replaces with the ceremonial
  `ITemplataT::Group(Default)` (see "The group representation").
- **Argument types now reach the call-site solve.** The reworked
  `assemble_initial_sends_from_args` seeds each argument into the solve as an `InitialKnown` plus an
  `Equals` (`typing/function/function_compiler_solving_layer.rs:927`, consumed at `:453-461`). The
  old "consumed nowhere" note is stale.
- **Control flow survives in the finished tree.** `IfTE` is its own join node with a `result` that
  says whether the join is reachable; `WhileTE` carries the implicit back-edge (Vale `while` is
  `loop`); a break produces `KindT::Never { from_break: true }`; there is no `continue`. A recursive
  check over the finished tree is therefore feasible.
- **`replace_value_type_in_ref` and `UpcastTE` are landed** (`typing/templata_compiler.rs:116`,
  `typing/convert_helper.rs:206`).

## The build work

**Status: the representation's *types*, the parser/postparser *syntax*, and the rung-1 joint-argument
check are landed (see "What is already done"); the fuller machinery below is not.** The joint-argument
check reads `GroupS`/`EffectS` off the scout `FunctionS` directly, and a group param concludes to the
ceremonial `ITemplataT::Group(GroupTemplataT {})` constant. Still to build: minting `GroupB`, and the
rung-2 liveness/invalidation dataflow that consumes it.

### 1. The group representation — never on the value type

This is the core of rung 0. The load-bearing decision: **a group never lives on the value type
(`KindT`), never flows through the solver.** In this codebase structural `Eq`/`Hash`, interner
identity, and monomorphization identity (`IdT`) are one relation (the solver's conflict check at
`solver/simple_solver_state.rs:75` *is* the derived `PartialEq`), so a group on `BorrowRefT` would
split `Vec<int> in a` from `Vec<int> in b` into two identical monomorphizations, trip consistency
asserts, and hit an override panic (`edge_compiler.rs:772`). Groups are inert cargo erased before
codegen. rustc validates this: HIR typeck erases every region to one constant and recomputes regions
in a separate pass, while type params stay symbolic.

So the representation splits four ways:

- **Empty `BorrowRefT` to `{ inner }`** (`typing/types/types.rs`), dropping the always-`Default`
  `region` field — "no group in a `KindT`" true by construction. Behavior-neutral (the field is
  always `Default`, nothing branches on it). **Do this as its own isolated commit** with a
  set-identical failing-set diff (it touches every share-borrow construction site). The ~85
  `RegionT::Default` literals, 36 `context_region: RegionT` params, and the `region` fields on ~8
  expression nodes + 3 identity name-structs (`ExportNameT`, `RawArrayNameT`, `ExternNameT`, inside
  `IdT`) become **removal** targets — the deep sweep can follow the empty-struct commit.
- **Group params stay uniform, valued by a ceremonial constant `ITemplataT::Group(GroupTemplataT)`
  holding `Default`.** A real conclusion (satisfies "every param has a value") that never enters a
  `KindT`. Chosen over skipping group params in the typer, which would bifurcate the generic-param
  list and cause silent arity/index bugs; rustc keeps lifetime params uniform for the same reason.
  Adding the variant is compiler-flagged (fill each): `ITemplataType::GroupTemplataType`
  (`templata/templata.rs:91`), the env-lookup arm (`env/environment.rs:476`), a no-op in
  `get_placeholders_in_templata` (`compiler.rs:186`, reached via `sanity_check_conclusion`), a
  return-self in `substitute_templatas_in_templata`, `visit_templata` (`test/traverse.rs`),
  humanizer arms, and — if groups reach monomorphization — a companion `ITemplataI::Group`. Bind it
  in `create_placeholder` (`templata_compiler.rs:1857`) and seed the param's conclusion as an
  `InitialKnown`. `GroupTemplataT` is a typing-pass templata (so `T`, not `B`); it is never read, so
  it needs no payload (a unit suffices). The conflict check is trivially safe
  (`Group(Default) != Group(Default)` is always false).
- **Group *syntax* lives in three enums, `P`/`S`/`B` by pass** — kept as extensible enums even
  though the first programs exercise only the name/rune leaf:
  - `GroupP` (parse AST): `Name(StrId)`, `Member`, `Elements`, `Union` — leaves are identifiers.
  - `GroupS` (postparse/scout, symbolic; subsumes today's `RegionS::Rune`): `Rune(&RuneUsage)`,
    `Local(CodeVarName)`, `Member`, `Elements`, `Union` — never `IdT`. This is the correct home for
    group syntax (the `ITypeST` borrow node's `region` slot becomes/points at a `GroupS`); putting
    the `IdT`-based `GroupB` on the `ITypeST` would drag typed identities into a pre-typing tree.
  - `GroupB` (borrow-checker, `B` = borrow-checker-only, minted from `GroupS` + conclusions):
    `Empty`, `Rune(IdT)`, `Local(IdT)`, `Member`, `Elements`, `Union` — the checker's reconstruction,
    canonical on `Union` construction (flatten, drop `Empty`, dedup, sort, collapse singletons).
  - **Near-term a group is just a name.** The parser produces only `Name`/`Rune` (the `attack`
    example — `&Entity in r`, `<r': Entity>`, `mut(r)`); building/parsing `Member`/`Elements`/`Union`
    is deferred to the first program that writes a value-path (`in x.items[]`) or union (`in (a|b)`).
- **Group/effect metadata is read straight off the scout `FunctionS`**, borrowed zero-copy from `'s`
  (borrow checking runs at the tail of each function's typecheck, while `'s` is alive), never copied.
  No dedicated side-table struct is needed: `full_env_snapshot.function` is `&'s FunctionS`, in scope
  at the seam, and its `.params` (each `.tyype: ITypeST`) and `.effects` are the group/effect source.
  `FunctionT` becomes `FunctionT<'t>`, copying into `'t` only what survives to instantiation —
  **`ITypeST`/group/effect data must never enter `'t`** or the durable `FunctionHeaderT`. rustc
  precedent: `TypeckResults` is a per-body table consumed by later passes and dropped. Invariant to
  type-enforce: make instantiation take only `FunctionT<'t>`, so it cannot be handed anything still
  borrowing `'s`. The definition-side `ParameterS.tyype: ITypeST` (plan §P, previously discarded at
  `postparsing/function_scout.rs:433`) is now populated and reachable here.

Two ruled constraints the group representation carries from day one:

- **The borrow-of-claim's member-level `rc.T` mention** must survive. A claim borrow carrying no
  multi mention escapes Send, scoping, and erasure coverage — a use-after-free plus a non-atomic
  refcount. This mention lives on the declaration-side `GroupS` (the scout `FunctionS`'s `ITypeST`),
  not on the (now group-free) `BorrowRefT`.
- **Independent group runes** (the "Milano case") must stay expressible: a group can appear only in
  a where-clause, deduced from nothing, so `GroupS` must allow a free-standing group rune.

Why this is tractable: **a group is an identity, not an extent** (the full ruling is under "Design
rulings"). With no outlives relation, variance, or subtyping, the group param is a uniform rune with
a constant value, and the borrow checker does the widening fold structurally rather than the solver.

**Guardrails for the executing tree:**

- Do not add a group/region field to `BorrowRefT` or any `KindT` payload — the value type's borrow
  is `{ inner }`, no permanent always-`Default` staging field.
- Do not put `ITypeST`/effect/group metadata onto the durable `FunctionHeaderT` or into `'t`; it
  lives on the scout `FunctionS`, borrowed from `'s`, dropped after the check.
- Do not skip group params in the typer (they stay uniform with type/int params); do not let an
  `ITemplataT::Group` value reach a `KindT` or a kind-position rune — keep group runes
  `GroupTemplataType`-typed and in the region slot.
- Do not resolve effect-clause or `ITypeST` group runes to `Default`; only `KindT` lowering
  collapses, and with `BorrowRefT` emptied there is no slot to collapse into anyway. The declaration
  side (`GroupS`) stays symbolic.
- Do not make the borrow checker read a group off a `KindT` — it reads `GroupS` off the scout
  `FunctionS`'s `ITypeST`.

### 2. Syntax

The declaration side is nearly free; the use side is the work.

- **`<g'>` (untyped group param) already parses.** No parser change.
- **`<g': T>` (typed group param) is a small parser add**, plus a postparse rule for the
  constraint.
- **The bare `in g` clause is the large item, and it has no grammar today.** Today every group
  *use* needs a tick prefix (`&r'Ship`); the target spells the tick only at the declaration and
  every use bare (`&Ship in g`). So `in g` is not just "add a keyword"; it changes where the group
  *name* is written at a borrow, which the scout lowers into that borrow node's `GroupS::Rune`. It is
  also what forces resolving a live panic: an anonymous region currently panics in postparse
  (`postparsing/rules/templex_scout.rs:region_s_into_region_sr`,
  `POSTPARSER_..._REGION_RUNE_NONE_NOT_YET_IMPLEMENTED`).

Two syntax landmines to plan around:

- **`rc` (the ambient-multi group) has no keyword yet.** Adding it is small, but it only means
  something once bare-group references exist.
- **The descendant step `...` collides with the lexer.** The lexer currently eats `...` as a
  comment (`lexing/lexing_iterator.rs:consume_ellipses_comments`), so `g...` needs lexer
  disambiguation, not just a parser rule.

The **minimal `mut(g)` effect clause** — one positive `mut` over a named group, parsed and scouted
onto `FunctionS.effects` as an `EffectS` (`Mut(GroupS)`) — is rung-0 *representation* (the `attack`
example needs it). Effect *checking*, `not(mut(...))`, multi-group folds, value-paths
(`world.ships[]`), and `...` are all rung 1 and later. So rung 0 needs: the group declaration, a way
to name a group at a borrow, and the bare `mut(g)` clause landed on `FunctionS.effects`.

### 3. Where and how the checker runs

The big call-site rewrite (`plan-phased-calls.md`) is **not** a prerequisite, and the checker does
**not** run per call site. All borrow checking is one **whole-function walk**, run once per function
over the finished body, at the end of compiling that function's `FunctionT`, inside the typing pass.
There is no separate borrowck pass.

The plug-in point is concrete and wired. A user body is compiled into a `FunctionDefinitionT`,
registered with `coutputs.add_function`, and the walk is invoked right after
(`function_compiler_core.rs:358`), on the finished `function2` whose body is a complete
`ExpressionTE` tree. This is the single real seam: the sibling `add_function` sites are extern
forwarders (a synthesized `Return(ExternFunctionCall)`, no user body) and a header-only site, so they
carry nothing to check and are skipped.

The finished function is handed to the walk directly, so it does not depend on being in `coutputs`;
running after `add_function` is simply where it landed.

Because the walk runs on the fully-resolved body, two hazards the old per-call idea had to avoid
simply do not arise:

- It never runs inside candidate selection. The finished tree holds only the one resolved call at
  each site, so a borrow failure can never demote a candidate. The old "never inside
  `attempt_candidate_banner`" discipline is now automatic.
- The per-call disjointness check is one **arm** of this walk, firing when the walk reaches a
  `FunctionCallTE`, not a hook threaded into `call_compiler.rs`. There is nothing to place carefully
  during typing.

Do not call the checker from `call_compiler.rs` or `find_function`; the only entry point is once per
finished function.

### 4. The checker's inputs and contract

The checker walks the finished function's `ExpressionTE` body once, carrying per-frame state for
move-tracking and invalidation. Its contract is deliberately narrow:

- **Inputs**: the finished function, **the scout `FunctionS` borrowed from `'s`** (its
  `ParameterS.tyype: ITypeST` carries each borrow's `GroupS`, and `FunctionS.effects` the declared
  effect clause — this is where the checker reads a borrow's group, since neither lives on the
  `KindT`), plus a read-only `&CompilerOutputs` and a read-only `&Compiler`. Everything is *borrowed*,
  not snapshotted: `FunctionS` is a zero-copy view into `'s`, alive because the checker runs at the
  tail of this function's typecheck. The checker reads struct member layout (for reach and
  sibling-disjointness) straight from the read-only `coutputs`.
- **Output**: errors. The checker mutates nothing (no `&mut CompilerOutputs`, no interner, no arena)
  and triggers no resolution or instantiation. The architect routes the errors it returns.
- **Shape**: the landed stub is `check_function(function: &FunctionDefinitionT, function_s:
  &FunctionS, coutputs: &CompilerOutputs, compiler: &Compiler) -> Result<(), ICompileErrorT>`,
  returning `Ok(())`; the `Result`/single-error form surfaces via `?` at the seam. Multi-error
  accumulation (`-> Vec<BorrowError>`) is a future refactor. Stateless across functions; it gains
  internal per-body state (the dataflow sets) only at the rung whose first test is use-after-churn.

The walk does its work in two node-arms that compose in a fixed order:

- **Borrow creation** at the member and element lookups (`expression_compiler.rs:786` Dot, `:1546`
  Index). Each already yields a `BorrowRef` of the pointee, and that borrow is what the checker keys
  on.
- **The joint-argument check** at each `FunctionCallTE`. This is the per-call disjointness check, an
  arm of this same walk rather than a separate hook.

The creation arm runs before the call arm that consumes a borrow, so the walk hands the call check a
borrow it has already recorded. Write the check as recursive descent over `ExpressionTE`, the same
idiom the typing pass itself uses: match each variant and recurse into children, carrying the
checker's per-frame state, with `IfTE` and `WhileTE` driving the joins and loop re-walks. The
compiler has no walker framework, and `traverse.rs` is test-only, not a basis for this.

## Groundwork, gated on a red test

**No typing-pass change lands without a failing test motivating it** (architect rule). The core is
off-limits to speculative edits, so the "cheaper to do it before the checker exists" argument does
not clear the bar.

- **Source ranges on the call and control-flow nodes are deferred.** Only 6 of 50 `ExpressionTE`
  variants carry a `RangeS`, and none of the call or control nodes do (`FunctionCallTE`, `IfTE`,
  `WhileTE`, `BreakTE`, and the rest), so a post-hoc checker cannot yet point at a call site or an
  `if`. But nothing is added until a borrow-check diagnostic actually needs a range and a red test
  proves the gap. When that test exists, fold in the vestigial `BreakTE.region: RegionT` removal
  while touching those nodes. This is a core edit — it needs "fire core edits".

The per-call group bindings need no separate solver plumbing. Because `BorrowRefT` carries no
group, the checker **derives** each borrow's group structurally — tracing the place back to its
parameter/local anchor and reading that anchor's group off the scout `FunctionS`'s `ITypeST`
(`a in ag`, `b in bg`), matched against the callee header's group parameters. Only the anchors need declaration
data; most body borrows are derived. It does not depend on the solver's discarded `inferences` map.
The only groups this misses are ones that appear on no argument and no result (an independent
where-clause group, or an effect-only `mut(g)`), and those are rung 1 and later.

## The order

1. **Decide** (architect) *(done)*: rung 0 and the joint-argument check are landed.
2. **The seam + the joint-argument check** *(done)*: `check_function` (`borrow_check.rs`) is invoked
   at the tail of each user-body typecheck (`function_compiler_core.rs:358`, after
   `coutputs.add_function`), walks the finished body, and runs the joint-argument check — rejecting two
   arguments aliasing into distinct mutated groups, and a borrow argument rooted in a moved argument's
   local. It reads the callee's `GroupS`/`EffectS` off the scout `FunctionS` and keys on a `PlacePath`
   argument identity; errors are `BorrowErrorKind` (`ICompileErrorT::BorrowCheckError`).
3. **Rung 0 data + syntax + the group constant** *(done)*: `BorrowRefT` emptied to `{ inner }`; the
   `GroupP`/`GroupS`/`GroupB` and `EffectP`/`EffectS`/`EffectB` enums; `ITemplataT::Group(GroupTemplataT)`
   constructed in `create_placeholder`'s `GroupTemplataType` arm (`templata_compiler.rs`);
   `ParameterS.tyype: ITypeST` populated; `&T in g`, `<g': T>`, and `mut(g)`/`not(mut(g))` parse and
   scout. Only the single-named-group leaf is produced so far.
4. **Rung 2 use-after-churn** *(done)*: the `liveness.rs` flow-sensitive walk — child-group element
   references (`place_path` `Segment::Element` + `is_child_group`), root-matched `mut`-gated churn
   invalidation, `if`-join union, and a `while` least-fixpoint. Built **without** `GroupB` (group runes
   compare by name). A runtime-sized-array local now drops cleanly via a closure-free `DropFunctor<T>`
   in `arrays.vale`, which rung 2's fixtures need.
5. **Remaining**: extend child groups past the array element — struct-field arrays, `Box`,
   `Variant`/interface payloads, then generic `Vec<T>` (the rung-2/rung-3 boundary — revisit before
   scoping); mint `GroupB` when `Member`/`Elements`/`Union` group expressions arrive that a name
   comparison cannot resolve; the `g...`/`rc` syntax; and effect *checking* (`ITemplataT::Effect`, a
   separate solver domain, deferrable).

## What still needs a ruling

One open question is shape-determining for the rung 0 representation, so resolve it as that
representation is designed:

- **What does `&x` form at a claim-typed place?** The candidates are a payload borrow, a
  compositional borrow-of-claim, or a one-hop argument coercion. This is open upstream, they have
  asked for our input, and our onion lowering already picks a horn implicitly. It is entangled with
  the `rc.T` mention above.

The other open design items (where mutability lives, the effect vocabulary, the per-group
permission map) all belong to rung 1's effect domain. They can wait.

## The full ladder (rungs 0-3)

Rung 0 is the foundation. Each rung past it catches a distinct class of error.

| Rung | Delivers | New errors | Emits errors? |
|---|---|---|---|
| 0 | `<g'>`/`in g`/`mut(g)` parse+scout; empty `BorrowRefT`; ceremonial `ITemplataT::Group`; group/effect metadata read off the scout `FunctionS` | none (well-formedness only) | no |
| 1 | effect *checking* (`mut(g)` representation lands at rung 0); the first check | disjointness violation (declared disjoint, passed aliasing); permission escalation | yes, the first real check |
| 2 | churn tracking (runtime-sized-array elements) | use-after-churn | yes, plus monotone state |
| 3 | more child-group sources (`Box`, `Variant`, struct fields) and generic `Vec<T>` | the same classes, on realistic code | yes |

Rung 0, rung 1's joint-argument check, and rung 2's use-after-churn on array elements are landed.
Rung 1's effect *checking* and rung 3 (more child-group sources, then generic `Vec<T>`) are next.

**A rung-2 use-after-churn needs a child group, and only certain constructs form one — an inline-only
plain struct forms none, so nothing there is ever invalidated** (see "Child groups" below). This is
why rung 2 lands on a runtime-sized-array *element*, not on `struct Fleet { flagship Ship; }`. The
borrow-checker track still runs in parallel with the generic-bounds track — generic `Vec<T>` is rung
3, but a monomorphic runtime-sized array needs no generics.

## Child groups (from `group-borrowing.vmd`)

The design source of truth for what a churn invalidates is `group-borrowing.vmd` (Nick Smith's group
model, as explained by the architect). Its one rule: **when someone modifies a parent group, invalidate
every reference into that group's child groups** — and nothing else. This is narrower than "a mutation
happened," and getting it right is the whole of rung 2.

- **Invalidation is only ever about child groups.** A reference *to an object* is never invalidated by
  modifying that object; only a reference *into its child groups* is. Concretely, on `d &Entity in r`,
  a call that churns `r` leaves a reference to `d`, to `d.hp` (an inline field), and to the whole
  `d.rings` list all live — only a reference to an *element* `d.rings[i]` dies.
- **A child group forms only from something that owns an independently-destroyable thing**: a
  collection/array **element**, a `Box` pointee, or a `Variant`/interface **payload**. *"If an object
  (even indirectly) owns something that could be independently destroyed, it must be in a child
  group."* An inline scalar or inline struct field is **not** a child group — it dies with its
  container, so a churn cannot dangle a reference to it.
- **Consequence that shaped rung 2: an inline-only plain struct forms no child groups**, so nothing in
  it is ever invalidated — there is no use-after-churn to catch there. Rung 2 therefore lands on a
  runtime-sized-array **element** (the simplest child-group source that compiles today); a struct like
  `Fleet { flagship Ship; }` is the wrong shape for this rung.
- **Where groups come from.** Each local variable forms its own group; groups combine into unions; and
  Variants/collections/`Box` inside a group form child groups. A `mut(g)` call is the churn event —
  it invalidates references into `g`'s child groups.
- **The isolation restriction is what makes aliasing safe.** Items in one group must be mutually
  isolated — they cannot own or hold references into each other — so a function handed several
  references into a group cannot use one to destroy another. This is why `attack(a &Entity in r, d
  &Entity in r) mut(r)` is memory-safe even though it mutates both: no `Entity` in `r` can delete
  another, and neither `a` nor `d` holds a child-group reference.
- **Paths carry child-group invalidation across calls.** A signature can name a callee's mutated child
  group by a path off a parameter (`mut rr: group Ring = e.rings*`), so the caller learns *exactly*
  which of its references the call may invalidate — `e.rings`' elements, and nothing else. This is
  rung-3-and-later machinery; rung 2 only needs the whole-parent-group `mut(g)`.

**How this affects the roadmap.** Rung 2 = churn tracking on array elements. Rung 3 extends the
child-group set (`Box` pointees, `Variant`/interface payloads, struct-field arrays) and then generic
`Vec<T>`, and eventually adds the path grammar for precise cross-call invalidation. The dataflow
machinery (`liveness.rs`) is already general over "which places are child groups"; each new source is
a `place_path` segment plus its child-group predicate, not a new analysis.

## Design rulings behind the checker

These are ruled upstream (`valen-design-1.md` is the authority) and shape both the representation
and the checker. The target program is a `Vec<Ship>` with two borrows into one group, both passed
to one `attack`, surviving mutation, then dying when the container is cleared.

**Groups do not flow through the solver at all.** design-1:1124: "Groups themselves don't conform
to traits," so no trait resolution depends on a group's value. design-1:1118: "T alone in a
signature is group-agnostic. Only `&Foo in g` carries group information." The only group-related
thing the solver ever sees is the ceremonial `ITemplataT::Group(Default)` constant (a group param's
value), which never enters a `KindT` and is never read — so the solver never puts a group into
conflict detection. The widening fold that used to be imagined in the solver is the **borrow
checker's** job: it reads the declaration-side `GroupS` off the scout `FunctionS`'s `ITypeST`,
reconstructs each value's `GroupB`, and applies the widening rule (merge iff no binding with a narrower claim
survives) as an order-independent fold. This matches rustc, whose unification analogue never unifies
regions and is infallible. A group also **never affects overload selection** (ruled): the candidate
filter keys on name, arity, namespace, wrap-chain, and value-type template — never on a group.

- Store provenance, not bare group expressions. The checker collects `(group expression,
  contributing site)` pairs so the fold can blame the right argument. rustc pays for skipping this
  with blame metadata on every constraint and a 1,000-line module for naming `'1`.
- Caveat: if you hit a point mid-*solve* that needs to know which group, that is a finding to report
  upstream — the solve is group-agnostic by design (design-1:1118, :1124), and groups live only on
  the declaration side.

**Effects are their own solver domain, not signature syntax.** design-1:1263: "the compiler unifies
declared effects with derived effects; `!mut(path)` acts as a subtractive constraint on effect
variables." A clause attached to a signature has no variables to solve, widen, and re-check, so
effects are a second solver domain (rung 1): `ITemplataT::Effect`, distinct from the ceremonial
group-param constant `ITemplataT::Group`. One
conformance pin to encode early (design-1:1235, Pin C): a negative bound is satisfied against the
effect variable's fully-solved value, and a solver widening `E` from a later position must re-check
every negative bound that touches `E`.

**A group is an identity, not an extent.** design-1's section is titled "Groups are not lifetimes":
a lifetime carries duration and exclusivity through one mechanism; groups drop exclusivity and name
pointee sets. `g in h` is place-subset (set containment), not an outlives relation. The consequence
is no outlives lattice, no variance, and no subtyping over groups. Everything is invariant, which is
what lets a group live in the existing templata/rune vocabulary at all. Do not import Rust's region
reasoning; it answers a different question.

**What the per-call check verifies.** Omission is a checked disjointness claim: a signature's
mutated groups include its effect targets, and omitting a declared relation between a bound group
and an effect target is a disjointness claim, verified at every binding site.

- Two arguments in one declared group is not an error. The compiler conjures a temporary union group
  at the call site (`f(&game_x, &game_y)` binds `g = {game_x, game_y}`). Temp unions apply to plain
  borrow params only, never to group arguments of parameterized types (`Overlay<a>` vs `Overlay<b>`
  is a type error).
- Distinct owning fields are provably disjoint with no declared relation (the sibling-disjointness
  lemma), so `attack(&fleet.flagship, &fleet.escort)` needs no clause.

**Invalidation is keyed on reach, not on `mut`.** design-1 defines reach as the union, over the op's
effect targets, of the target's contents-territory: every destructible place beneath it, excluding
the target's own member places. That is why `attack` writing `attacker.fuel` invalidates nothing
while `ships.clear()` kills every element borrow. A checker keyed on "a `mut` happened" rejects the
motivating program. This is the same downward-only, child-groups-only rule stated in "Child groups"
above, and it is exactly what rung 2 implements: a `mut(g)` call invalidates only references whose
place steps through a child-group boundary (an array element) rooted in the churned array.

**Two join disciplines, deliberately.** Monotone facts (invalidation, poisoning, mention sets) are
may-facts: union at joins, least fixpoint on loops. Conserved facts (move-state, linear obligations)
admit no safe extreme, so a disagreeing join rejects. The existing move tracker is the conserved
kind; invalidation is the monotone kind. They sit side by side.

**Two node-arms, because the call check alone is insufficient.** `xs[i]` is a place expression, not
a call. design-1:169 calls `&entity.rings[0]` "a fresh borrow derived from a place," and design-1:902
binds group parameters to indexed places. So borrow creation is its own arm of the whole-function
walk, at the member/element-lookup nodes, and the per-call check covers checking only. They compose
in a fixed order: the creation arm runs first and hands the call arm a recorded borrow. Plan the walk
for both node-arms from the start; this is the cheap-now, expensive-later item.

**Quarantine by capability, not visibility.** The checker reads `ExpressionTE`. A parallel fact-IR
was considered and rejected: a second representation with permanent sync cost, and rustc only
affords MIR because MIR has four other consumers. The rule instead constrains what the checker may
do: no `&mut CompilerOutputs`, no interner, no arena, no calls back into resolution; errors are
returned and the architect routes them. The entry point is `check_function(function:
&FunctionDefinitionT, function_s: &FunctionS, coutputs: &CompilerOutputs, compiler: &Compiler)`
(landed returning `Result<(), ICompileErrorT>`; `Vec<BorrowError>` is the future multi-error form).
All inputs are borrowed (`FunctionS` zero-copy from `'s`), so nothing is snapshotted. It is stateless
across functions and gains internal per-body state only at the rung whose first test is
use-after-churn.

**The checker is per-body.** Resolution is symbolic at the typing pass (design-1:125); poisoning is
computed per frame. Every use goes through a typed binding in some frame, each frame knows its own
history, and handing a poisoned value to a callee is itself a use, so callees may assume their
parameters arrive clean (design-1:665). Nothing reads another body's contents.

**"Signatures" means the whole signature, not just the effect clause.** Two mechanisms need more:
`dangle` propagation (design-1:713, a `dangle`-group reference may be handed only to callees that
also accept it as `dangle`), and relation-aware checks (design-1:715/1233, `dangle` violation and
subtractive satisfaction close over declared `maybealias`/`in` relations, which live in
where-clauses). So the read-only input is the effect clause plus `dangle` annotations plus
where-clause relations.

### Region and effect specifics

- `ITemplataT::Group` holds only the ceremonial constant `Default`; it is a group param's *value*,
  not the algebra. The group algebra proper is `GroupB`, minted by the borrow checker; the
  declaration-side `GroupS` names a local's group (`Local`) as readily as a parameter's (`Rune`). The
  group-param mint path already exists (a `PlaceholderTemplataT`), reworked to return the constant.
- A group is never `mut` or `imm`. A group says which places, never what you may do to them. Put
  permission on the group and `read_hp(e &Entity in g)` and `heal(e &Entity in g) mut(g)` stop
  having the same parameter type, and the next question is variance, which Valen refused. Condemned
  as fossils by this: `IRegionMutabilityS::{ReadOnlyRegion, ReadWriteRegion}`, `imm` as a region
  modifier, and `RegionT::Iso` (isolation is a property of how a group was minted, not a variant of
  the algebra). `RegionT` itself becomes vestigial once `BorrowRefT` is emptied.
- Borrow creation computes, it does not check. But the group does **not** live in the value type —
  the checker derives a borrow's group by tracing the place to its anchor and reading the anchor's
  group off the scout `FunctionS`'s `ITypeST`. The checking half is the joint-argument check at call
  sites.
- Effect derivation is rung-1 checking: the checker derives a body's effects and unifies them
  against the declared clause (`FunctionS.effects`). It is not held in the solver and is not a
  rune conclusion.
- Effect representation is unsettled. Attempts, recorded so nobody re-walks them: (i) `ITemplataT::Effect`
  with a folded `mutates` plus an `includes` list, dropped; (ii) "effects as a bound family,"
  refuted because bounds are denizen lookups; (iii) a header field plus a `NotMutSR` rule variant
  discharged in `check_resolving_conclusions_and_resolve`, whose discharge site the architect
  confirmed good; but (iv) a bare `mutates: RegionT` is too narrow, because the permission axis is
  real. The live candidate is a per-group permission map, keyed by group expression, valued by a set
  of promises rather than an ordered level. Its keys are group expressions, so it needs an eager
  canonical form for the algebra.
- `not(mut(...))` applies to the whole call, not to a named subject, so the rule variant needs no
  subject rune, just the banned group expression. The negatives are a list of independent conjuncts,
  not one folded expression.

## Evidence from rustc

The canonical "what to copy and what to skip from Polonius" catalog lives in
`src/typing/docs/borrow-checker-guidelines.md` (§"Similar to Polonius"); this section keeps only the
roadmap rationale. Two surveys of `~/rust` justify the choices above.

- About 15,000 of borrowck's 30,750 lines exist because Rust infers lifetimes: renumbering,
  universal-region discovery, the SCC constraint solver, blame metadata on every constraint, and a
  1,000-line module for inventing names for `'1`. Declared groups skip all of it.
- HIR typeck erases all regions on writeback, and borrowck re-typechecks the whole MIR body (~5,600
  lines) purely to regenerate region constraints — direct precedent for keeping groups off the value
  type. That also argues for keeping our checker inside the typing pass, at the tail of each
  function, where the declaration-side group data is still live in `'s`.
- The one rustc reason borrowck must feed later typeck is **opaque (`impl Trait`) hidden types whose
  regions are inferred**. Explicit groups remove that edge; it reappears only if we adopt
  inferred-lifetime existentials (return-position existentials whose group is inferred, not written).
  That, and a late-bound (higher-ranked) group *closure type* (`func __call<g'>(self, e &Entity in
  g)`, one stored value invoked at many groups), are the two future tripwires that would force a
  group into type identity — re-check both when the closure/trait machinery returns.
- MIR was edited for borrowck after the fact (`FalseEdge`, `FakeRead`, special match lowering) to
  hide CFG structure from it. Lowering to a CFG bought precision, then they spent effort blurring it
  back.
- rustc's structure is two separate traversals: a location-local visitor
  (`check_call_inputs`/`check_call_dest`) plus a body-global dataflow fixpoint. Our structured
  control flow lets one whole-function walk play both roles: the per-call check is an arm, the
  dataflow is carried state (or a preceding propagate pass). Plan the walk for both from the start.
- `rustc_borrowck` is a leaf crate, which is precedent that capability-quarantine works.
- Rust has no declared disjointness; it uses place-overlap detection (`places_conflict.rs`, ~526
  lines). Our same-group-aliases-freely rule deletes that entirely.
- Two-phase borrows are a desugaring artifact: `v.push(v.len())` compiles, the hand-desugared form
  is E0502. Evaluating args before materializing the receiver borrow avoids the problem.
- The price of deferring regions is a second full type-check: HIR typeck runs `.ignoring_regions()`
  and erases every region, then MIR typeck re-walks the whole body to regenerate constraints.
