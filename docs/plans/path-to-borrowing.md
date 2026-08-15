# The Path to the Borrow Checker

The onion refactor is done and the suite is green, so the next quest is the region borrow
checker. This document maps the path from here to *starting* it: what is already built, what we
still have to build, and in what order.

The first step is **rung 0: "groups become real."** Rung 0 makes groups expressible and puts a
real region on every borrow. It does not check anything. No program is accepted or rejected that
was not before. The first actual borrow check is rung 1, which is a separate and later step.

Keep that split in mind throughout: rung 0 is representation and plumbing; checking comes after.

## The one gate

Starting rung 0 is an architect decision, and it is the only thing that blocks the work.

Everything else on this page is either already done, ordinary build work, or deferrable to a
later rung. The design behind rung 0 is settled; once the decision is made, it is build work, not
more design.

## What rung 0 is, and what it is not

Rung 0 delivers exactly three things:

- **Syntax**: parse and scout group declarations (`<g'>`) and the `in g` clause.
- **A region value type**: add an `ITemplataT::Region` variant so a group can be a rune value.
- **A real region on the borrow**: replace today's placeholder `RegionT::Default` with an actual
  group.

Rung 0 adds **no new errors** (well-formedness only). The first real check, and effect clauses
like `mut(g)`, arrive at rung 1. Rung 1 is a second solver domain and is still partly undesigned,
so this document treats it as out of scope and something rung 0 must not block.

## What is already done, so we do not rebuild it

The onion work left more of the foundation in place than the old handoff suggests. Verified
against the current tree:

- **`substitute_templatas_in_kind` handles all four ref wraps**, and it already preserves a
  borrow's region across substitution (`typing/templata_compiler.rs:540-552`). The seam that
  carries a region through generics is shaped correctly; it just carries `Default` today. The only
  remaining `unimplemented!()` ref arms are in the dead `is_descendant_kind` / `is_ancestor_kind`,
  whose callers are commented out.
- **Group *parameters* already have a representation.** `create_placeholder` mints an opaque
  `ITemplataT::Placeholder` tagged `RegionTemplataType` for a region generic param
  (`typing/templata_compiler.rs:1516`). The model "a region is a placeholder referring to a generic
  parameter" is real and works.
- **The `<g'>` group-param declaration already parses** (it is the region-typed generic param,
  `parsing/parser.rs:parse_generic_parameter`).
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

### 1. A real region on the borrow

This is the core of rung 0, and it is bigger than it first looks.

Today `RegionT` is a flat two-variant `Copy` enum, `{ Iso, Default }`
(`typing/types/types.rs:16`). Only `BorrowRefT` carries a region; the other three wraps have no
region field. Every borrow in the tree carries `RegionT::Default`. And the rune *value* domain has
no region at all: `ITemplataT` has no `Region` variant, so a region *parameter* is faked as an
opaque placeholder minted through a fallback named, tellingly,
`create_non_kind_non_region_placeholder_inner`.

So "put a real value in the region slot" is four pieces:

- **Add `ITemplataT::Region`**, the missing value variant. The downstream instantiating pass
  already has the equivalent (`instantiating/ast/templata.rs:70`), which is a precedent to follow.
- **Make `RegionT` a real, nestable algebra.** A concrete group is recursive (a join like `a + b`,
  a container's child group, an iso boundary). A recursive value cannot be an inline `Copy` enum,
  because `KindT` is a deliberately 16-byte `Copy`/`Eq`/`Hash` type (`@WVSBIZ`). So `RegionT` has to
  become arena-interned with a `*ValT` companion, the same shape as `StructTT` / `StructTTValT`.
- **Add a real region-placeholder mint path**, to replace the `non-kind-non-region` fallback that
  stands in for it today.
- **Design the concrete-group-expression representation.** It does not exist yet. Only group
  *parameters* do.

The representation carries two ruled constraints from day one:

- **It must carry the borrow-of-claim's member-level `rc.T` mention.** A claim borrow that carries
  no multi mention escapes Send, scoping, and erasure coverage, which is a use-after-free plus a
  non-atomic refcount. Our `BorrowRef(ShareRef(...))` carries only its region slot today, so the new
  representation is where this mention lives.
- **It must not preclude independent group runes** (the "Milano case"). A group can appear only in
  a where-clause, deduced from nothing, so the representation must allow a free-standing group rune.

One property makes all of this tractable: **a group is an identity, not an extent.** Groups name
pointee sets, not lifetimes. There is no outlives relation, no variance, and no subtyping over
groups, so a group fits the existing templata/rune vocabulary. Through the solve, a region is inert
cargo: nothing mid-solve needs to know *which* group a region is. The solver accumulates the group
expressions bound to each region rune, and a later order-independent fold applies the widening
rule.

Threading surface, measured against the current tree:

- ~85 `RegionT::Default` literals in non-test source.
- 36 signatures thread `context_region: RegionT` by value.
- 8 expression nodes carry a `region` field, plus 3 identity name-structs
  (`ExportNameT`, `RawArrayNameT`, `ExternNameT`) embed one.

### 2. Syntax

The declaration side is nearly free; the use side is the work.

- **`<g'>` (untyped group param) already parses.** No parser change.
- **`<g': T>` (typed group param) is a small parser add**, plus a postparse rule for the
  constraint.
- **The bare `in g` clause is the large item, and it has no grammar today.** Today every region
  *use* needs a tick prefix (`&r'Ship`); the target spells the tick only at the declaration and
  every use bare (`&Ship in g`). So `in g` is not just "add a keyword"; it moves where the region
  lives at a use site. It is also what forces resolving a live panic: an anonymous region currently
  panics in postparse (`postparsing/rules/templex_scout.rs:region_s_into_region_sr`,
  `POSTPARSER_..._REGION_RUNE_NONE_NOT_YET_IMPLEMENTED`).

Two syntax landmines to plan around:

- **`rc` (the ambient-multi group) has no keyword yet.** Adding it is small, but it only means
  something once bare-group references exist.
- **The descendant step `...` collides with the lexer.** The lexer currently eats `...` as a
  comment (`lexing/lexing_iterator.rs:consume_ellipses_comments`), so `g...` needs lexer
  disambiguation, not just a parser rule.

Effect clauses (`mut(...)`, `not(mut(...))`), value-paths (`world.ships[]`), and `...` are all
rung 1 and later. Rung 0 needs only the group declaration plus a way to name a group at a borrow.

### 3. Where and how the checker runs

The big call-site rewrite (`plan-phased-calls.md`) is **not** a prerequisite, and the checker does
**not** run per call site. All borrow checking is one **whole-function walk**, run once per function
over the finished body, at the end of compiling that function's `FunctionT`, inside the typing pass.
There is no separate borrowck pass.

The plug-in point is concrete. A function body is compiled into a `FunctionDefinitionT`
(`function_compiler_core.rs:304`), then registered with `coutputs.add_function` (`:309`). The walk
runs here, on the finished `function2`, whose body is already a complete `ExpressionTE` tree. The
other body-bearing function kinds register at the sibling `add_function` sites (`:196`, `:396`); each
is a plug-in point.

Whether the walk runs before or after `coutputs.add_function` is left open (see "What still needs a
ruling"). The finished function is handed to the walk directly, so it does not depend on this
function being in `coutputs`.

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

- **Inputs**: the finished function, plus a read-only `&CompilerOutputs` and a read-only `&Compiler`.
  Read-only borrows are enough, so nothing has to be snapshotted into a separate input struct. The
  checker reads struct member layout (for reach and sibling-disjointness) straight from the read-only
  `coutputs`.
- **Output**: a list of errors. The checker mutates nothing (no `&mut CompilerOutputs`, no interner,
  no arena) and triggers no resolution or instantiation. The architect routes the errors it returns.
- **Shape**: `check_function(function: &FunctionDefinitionT, coutputs: &CompilerOutputs, compiler:
  &Compiler) -> Vec<BorrowError>`, stateless across functions. It gains internal per-body state (the
  dataflow sets) only at the rung whose first test is use-after-churn.

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

## Groundwork worth doing early

One item is independent of the region decision and cheaper to do before the checker exists.

- **Add source ranges to the call and control-flow nodes.** Only 6 of 50 `ExpressionTE` variants
  carry a `RangeS`, and none of the call or control nodes do (`FunctionCallTE`, `IfTE`, `WhileTE`,
  `BreakTE`, and the rest). A post-hoc checker cannot point at a call site or an `if` without them.
  Adding ranges now is far cheaper than retrofitting them once a checker depends on the tree shape.

The per-call group bindings need no separate plumbing. Once rung 0 puts a real region on every
`BorrowRefT`, each argument's group rides on that argument expression's own `BorrowRef` region, and a
return-position group rides on the call's result. Both are already in the finished tree, so the walk
reads a call's groups straight off its argument and result nodes, matched against the callee header's
region parameters. It does not depend on the solver's discarded `inferences` map. The only groups
this misses are ones that appear on no argument and no result (an independent where-clause group, or
an effect-only `mut(g)`), and those are rung 1 and later.

## The order

1. **Decide** (architect): go or no-go on rung 0. Nothing past rung 0 starts until this.
2. **Groundwork** (region-independent, do anytime): add source ranges to the call and control
   nodes.
3. **Rung 0 core**: the region representation (`ITemplataT::Region`, interned recursive `RegionT`,
   a real region-placeholder mint, the concrete-group-expression rep with the `rc.T` mention and
   independent-group-rune constraints). Thread the real region through the ~85 `Default` sites.
4. **Rung 0 syntax**: `<g': T>`, then the `in g` clause (which resolves the anonymous-region panic).
5. **The seam**: stand up the whole-function walk and invoke it at the end of compiling each
   `FunctionT` (`function_compiler_core.rs:304-309` and the sibling `add_function` sites), passing the
   finished function plus a read-only `&CompilerOutputs` and `&Compiler`, returning errors. Rung 0 is
   well-formedness only, so the walk does nothing yet; this step just proves the plumbing: it exists,
   runs once per function, and returns an empty error list.

Rung 1 follows: the first real check plus effect clauses (`mut(g)`), which is a second solver
domain (`ITemplataT::Effect`). It is deferrable and does not gate the start.

## What still needs a ruling

Two open questions remain. The first is shape-determining for the rung 0 representation, so resolve
it as that representation is designed:

- **What does `&x` form at a claim-typed place?** The candidates are a payload borrow, a
  compositional borrow-of-claim, or a one-hop argument coercion. This is open upstream, they have
  asked for our input, and our onion lowering already picks a horn implicitly. It is entangled with
  the `rc.T` mention above.

The second is a small ordering choice, not a design fork:

- **Does the whole-function walk run before or after `coutputs.add_function`**
  (`function_compiler_core.rs:309`)? The finished function is handed to the checker directly, so its
  own data never depends on being registered; the only effect is whether the checker sees its own
  function in the read-only `coutputs` while checking, which it should not need. Undecided.

The other open design items (where mutability lives, the effect vocabulary, the per-group
permission map) all belong to rung 1's effect domain. They can wait.

## The full ladder (rungs 0-3)

Rung 0 is the foundation. Each rung past it catches a distinct class of error.

| Rung | Delivers | New errors | Emits errors? |
|---|---|---|---|
| 0 | `<g'>`/`in g` parse+scout; `ITemplataT::Region`; real region on `BorrowRefT` | none (well-formedness only) | no |
| 1 | effect clauses (`mut(g)`); the first check | disjointness violation (declared disjoint, passed aliasing); permission escalation | yes, the first real check |
| 2 | churn tracking | use-after-churn; a borrow sibling to a churning receiver | yes, plus monotone state |
| 3 | `Vec<T>` (rides the generics/bounds track) | the same classes, on realistic code | yes |

Rungs 0 through 2 need no generics and no `Vec`; plain structs suffice, e.g.
`struct Fleet { flagship Ship; escort Ship; }`. So the borrow-checker track and the
generic-bounds track run in parallel, not in sequence.

## Design rulings behind the checker

These are ruled upstream (`valen-design-1.md` is the authority) and shape both the representation
and the checker. The target program is a `Vec<Ship>` with two borrows into one group, both passed
to one `attack`, surviving mutation, then dying when the container is cleared.

**Regions are inert cargo through type-solving.** Nothing mid-solve needs to know which group a
region is. design-1:1124: "Groups themselves don't conform to traits," so no trait resolution ever
depends on a group's value. design-1:1118: "T alone in a signature is group-agnostic. Only `&Foo in
g` carries group information." So the solver never puts regions into conflict detection. It
accumulates the group expressions bound to each region rune, and a later order-independent fold
applies the widening rule (merge iff no binding with a narrower claim survives). This matches rustc,
whose unification analogue never unifies regions and is infallible.

- Store provenance, not bare group expressions. Collect `(group expression, contributing site)`
  pairs so the fold can blame the right argument. rustc pays for skipping this with blame metadata
  on every constraint and a 1,000-line module for naming `'1`.
- Caveat: if you hit a point mid-solve that needs to know which group, that is a finding to report
  upstream. It would contradict design-1:1118 and :1124.

**Effects are their own solver domain, not signature syntax.** design-1:1263: "the compiler unifies
declared effects with derived effects; `!mut(path)` acts as a subtractive constraint on effect
variables." A clause attached to a signature has no variables to solve, widen, and re-check, so
effects are a second solver domain: `ITemplataT::Effect` beside `ITemplataT::Region`. One
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
motivating program.

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
&FunctionDefinitionT, coutputs: &CompilerOutputs, compiler: &Compiler) -> Vec<BorrowError>`. A
read-only `coutputs` and `compiler` are enough, so nothing is snapshotted. It is stateless across
functions and gains internal per-body state only at the rung whose first test is use-after-churn.

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

- `ITemplataT`'s region payload is the group algebra, and it names a local's group as readily as a
  parameter's. The group-parameter path already exists (a `PlaceholderTemplataT`); a concrete group
  expression needs the new variant.
- A region is never `mut` or `imm`. A region says which places, never what you may do to them. Put
  permission on the region and `read_hp(e &Entity in g)` and `heal(e &Entity in g) mut(g)` stop
  having the same parameter type, and the next question is variance, which Valen refused. Condemned
  as fossils by this: `IRegionMutabilityS::{ReadOnlyRegion, ReadWriteRegion}`, `imm` as a region
  modifier, and `RegionT::Iso` (isolation is a property of how a group was minted, not a variant of
  the algebra).
- Borrow creation computes, it does not check. The group lives in the type, on the `BorrowRef`'s
  region slot, which is where all borrow construction sites stamp `Default` today. The checking half
  is the joint-argument check at call sites.
- Effect derivation: the solver produces the bindings; the effect falls out by substitution
  afterward. It is not held in the solver and is not a rune conclusion.
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

Two surveys of `~/rust` justify the choices above.

- About 15,000 of borrowck's 30,750 lines exist because Rust infers lifetimes: renumbering,
  universal-region discovery, the SCC constraint solver, blame metadata on every constraint, and a
  1,000-line module for inventing names for `'1`. Declared groups skip all of it.
- HIR typeck erases all regions on writeback, and borrowck re-typechecks the whole MIR body (~5,600
  lines) purely to regenerate region constraints. That argues for keeping our checker in the typing
  pass, where the group is already on the type.
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
