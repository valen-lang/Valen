# Region / group borrow checker — handoff

The group-based borrow checker: what it catches today, what remains, and the open region/effect design
decisions behind rung 1 and beyond. Its design doc is
`src/typing/docs/architecture/borrowing-design.md` and its roadmap `docs/plans/path-to-borrowing.md`.
The checker lives in `src/typing/borrow_checker/` (AI-editable); touching the rest of the typing pass
still needs "fire core edits".

## What is built

The only entry point is `check_function` (`check.rs`), called at each user-body typecheck's tail (in
`function_compiler_core.rs` after `coutputs.add_function`) and pure. It runs two phases:
`groupify_function` (`groupify.rs`) walks the finished body into a grouped `IExpressionGE`
(`grouped_ast.rs`) whose reference bindings carry their `GroupExprG` and whose calls carry their churns
(`MutEffectPath`) and joint-argument facts — parameter groups come from `make_kind_g` (`borrow_types.rs`),
the group-annotated `KindGT`/`ITemplataG` mirror of `KindT`/`ITemplataT`; then `check_usages`
(`check_usages.rs`) walks that once, tracks live references, and rejects a use of a reference a churn
invalidated, pointing the diagnostic at the use site.

It catches use-after-churn of any child group reached below a churned group — an array element, or
`a.data[]` when a churn hits `a`'s group — matched by group, with `if`-join union and `while` loop
pre-seeding (no fixpoint), plus the two joint-argument checks (`AliasingIntoDisjointMutGroups`,
`BorrowIntoMovedArgument`); `BorrowErrorKind` (`borrow_error.rs`) renders all three. It also catches
use-after-churn through a *returned* reference (`v = get(&a)` where `get` returns `&int in g[]`):
`groupify` reads the callee's declared return group off its `FunctionS.maybe_return_type` (via
`make_kind_g`) and crosses it to the caller frame with `substitute_groups` (`borrow_types.rs`), so a
later churn of `a` invalidates `v`.

`groupify` derives every borrow's group **compositionally, post-order**: each node's `KindGT` result is
built from its already-grouped children plus the leaf rule (a tracked local's grouped type; a
parameter's, via `make_kind_g`; a call return's, via the callee signature and `substitute_groups`), so
groups flow at every depth. **There is no groupless borrow and no `GroupExprG::Empty`** — a borrow whose
group genuinely can't be derived **panics** (a deferred case), never a placeholder or a soft error; see
`docs/plans/group-generic-closures-plan.md`. Group paths `in g.items` / `in g[]` / `in g...` parse
(`parse_group` in `templex_parser.rs`) and scout (`translate_group_p_into_group_s`).
`group_expr_from_group_s` and `subst_group_expr` are shared `pub(crate)` fns in `borrow_types.rs`.
Member-element paths (`g.tiles[]`) work in parameter and return position. The `...` descendant is landed
end to end: `mut(g...)` normalizes to `mut(g)`, and an ellipsis reference is invalidated by any churn
whose path overlaps its base in either direction (`borrowing-design.md`'s S1).

## What remains

**Next — Scope C, making `mut(g)` load-bearing.** The **producer churn gate** is landed: `check_usages`
rejects a call that churns a group reached through one of the enclosing function's parameters unless the
signature declares a `mut(...)` covering it (`BorrowErrorKind::UndeclaredChurn`; a churn of a local the
function owns is exempt; `producer_gate_tests.rs`). Next is **override effect-matching** — an override's
declared `mut(...)` must match the abstract method's exactly, compared positionally per parameter
(top-level param groups only; a nested-group `mut` on an override pair is a deferred-case error). It is a
borrow-check whose comparison lives in `borrow_checker/` but is invoked from `edge_compiler.rs`'s override
resolution — a second borrow-checker entry point, so it needs "fire core edits", and coordinate with the
Vale4 side (they own `rust_interop` synthesis). See `borrowing-design.md`'s Overrides ruling and its
"Override effect-matching is a borrow-check" proposal; Gap 2 (borrow-checking the abstract method's own
body) was un-ratified — Gaps 1+3 cover the reverse-callback safety.

After Scope C, **group-generic closures** per `docs/plans/group-generic-closures-plan.md` so a closure
capturing a reference (`*(self.capture)`) derives a group instead of panicking — the dominant deferral.
Its prerequisite is done: the `function_scout.rs` group-param strip is removed, so group params now flow
through the whole pass, concluding to the ceremonial `ITemplataT::Group(GroupTemplataT{})`
(`RegionTemplataType` is gone — the Region→Group rename). After closures: optional/weak-borrow returns (a
call or an `x.as<T>()` downcast yielding `Opt<&T>`, whose inner borrow is underivable and panics);
removing the remaining check-phase `Option`s and helpers in `check_usages.rs`/`groupify.rs` (no `Option`
return or field survives without a `VOPT` marker or a `## Design (human-only)` mention); `Box`/`Variant`
child-group sources; the `a | b` union / `rc` grammar; and effect *checking*. Shadowing is not yet
detected-and-panicked. The walk's un-handled child-bearing nodes (and
`ExternFunctionCall`/`InterfaceFunctionCall`, which carry no `loct`) are documented false-negative gaps.
The tests these deferrals block are `#[ignore]`d with a re-enable-with-borrowing `VCOORD` marker —
re-enable them as each feature lands (grep that marker to find them); the `--lib` suite is green
(`cargo test --manifest-path Cargo.toml --lib`; also native + wasi `cargo nextest run`).

**Panic-message accuracy — a small TODO.** The deferred-case panics in `borrow_types.rs`
(`make_kind_g_groupless`'s borrow arm, `group_anon`, the two `Group`-templata arms,
`group_expr_from_group_s`'s `Local` arm) all point at `group-generic-closures-plan.md`, but several are
really the weak / group-argument / `in x`-local / `held` deferrals, not closures. Retarget each message to
its own feature when picking that feature up.

**Interop-lane consumption of the checker.** The Rust borrow-return importer gap is fixed —
`synthesize_extern_function` (`rust_interop/declarations.rs`) synthesizes an extern's borrow return as
`&T in <the elided &self param's group>` and the checker consumes it unchanged (`arg_rune_subst` +
`substitute_groups` resolve a return rune shared with a param). The driven-harness closure failures are
worked around in `migrate.vale`: both `migrate` overloads comment out their closure-based `drop_into(...)`
calls under a re-enable-when-borrowing-ok VCOORD and fall through to `__vbi_panic()`, until group-generic
closures land — so `migrate` currently panics rather than draining its source. (The closure-free
`DropFunctor<T>` that lets the arrays builtin drop cleanly lives in `arrays.vale`, not here.)

## Region and effect decisions (landed)

The region/effect rulings live in `docs/plans/path-to-borrowing.md`: groups live only on the declaration
side (`GroupS`), never on the value type; `ITemplataT::Group` is a ceremonial constant, not the algebra;
a group is never `mut`/`imm`, condemning `RegionT::Iso` (and `RegionT` itself, once `BorrowRefT` is
emptied) as a fossil; borrow creation computes rather than checks, but the checker derives a borrow's
group by tracing to its anchor, not off a `KindT`; a borrow-of-claim must carry the claim's `rc.T`
mention; effect representation is unsettled (live candidate a per-group permission map); `not(mut(…))`
applies to the whole call; the checker iterates the finished tree.

**Design index.** The design doc `src/typing/docs/architecture/borrowing-design.md` covers the two phases,
the grouped AST, `make_kind_g`, `GroupSubtree`. The roadmap `docs/plans/path-to-borrowing.md` carries the
ladder (rungs 0-3), the design rulings (regions are inert cargo, a group is an identity not an extent,
invalidation keyed on reach, the two join disciplines, the two seams, quarantine by capability, per-body,
the whole-signature input), and the region/effect rulings.

## Open design questions (ours)

- **The effect representation.** A bare `mutates: RegionT` is too narrow (upstream answer 21). Live
  candidate is the per-group permission map; the axes are `held` (destruction), `dangle`/`opaque`
  (dereference), and a possible `softmut` tier — **partly independent flags, not an ordered level.** Needs
  an eager canonical form for the group algebra, since map keys are group expressions.
- **Our clone bound has no effect slot** (upstream answer 25).
- **Provenance / "contributing site."** No longer undercut by ranges — every `ExpressionTE` node now
  carries a `RangeS`, so a post-hoc checker can point at the offending line. `CaseRuneFromImpl { inner_rune }`
  remains the in-tree precedent for canonicalize-the-value-keep-the-origin.
- **"Params get runes, locals get classified" needs amending** — answer 19 says some group parameters can
  be *independent*, so the clean split doesn't hold as stated.
- **`BorrowState`'s shape** — still correctly parked behind its stated trigger.
- **Where a minted anchor's release charge lands**, and whether the found-anchor case admits the
  quiet-window certificate. Both are open *upstream* and both land on the consumer-side claim rules.
- **What the expression `&x` forms at a claim-typed local** — payload borrow by concrete sugar,
  compositional borrow-of-claim, or a one-hop argument coercion. Open upstream, and they have asked for our
  input, since our lowering implicitly picks a horn.

## Decisions only the architect can make

1. **Rung 0, rung 1's joint-argument check, and use-after-churn are landed — the start question is
   settled, do not re-raise it.** The scope is in `docs/plans/path-to-borrowing.md`. Groups live only on
   the declaration side, never on the value type — empty `BorrowRefT` to `{ inner }`, a ceremonial
   `ITemplataT::Group(Default)` constant for the uniform group param, `GroupP`/`GroupS` enums + a
   minimal `mut(g)` clause (`GroupB` is not yet a defined enum — it appears only in doc comments as the
   planned borrow-checker algebra), all read by the borrow checker off the scout `FunctionS` and the
   written `ITypeST` (no side-table struct; `FunctionT<'t>`, `ITypeST` never into `'t`). Groups never flow through
   the solver. The still-open effect-domain calls are decisions 2-3 below.
2. **Where does mutability live?** design-1 puts it on the **group**, via signature effect clauses (`func
   heal(e: &Entity in g) mut(g)`), and calls that its one departure from Rust (*"there is no `&mut`"*,
   design-1:361). We dropped `&mut` and never added effect clauses, so mutability currently lives
   **nowhere**. Sits *behind* rung 0, not beside it, since effect targets are group expressions. It is
   bigger than "add effect clauses": `mut(g)` / `mut(g.tiles[])` / `mut(E)` with `E: Effects` / `mut(())`,
   the subtractive `not(mut(…))` forms, the deep-effect rule, parameter shorthands that desugar to **fresh
   anonymous group params instantiated per call site**, plus solve-order pins (design-1:1235 — no negative
   bound discharged against less than `E`'s full solution; re-check on widening). That is a **second solver
   domain** (rung 1) alongside types, i.e. `ITemplataT::Effect` — distinct from the ceremonial group-param
   constant `ITemplataT::Group`, since groups themselves never flow through the solver.
3. **Effect vocabulary staging** — full set or `mut(g)` first? And is effect *inference* in scope
   initially? (Ruled: named functions **declare and are checked**, so no call-graph fixpoint; closures
   genuinely infer, and a *recursive closure* is an unaddressed gap.)

## Waiting on upstream — exactly four

(1) Whether **`mut(E)`'s `E` is a group or its own sort** — they carry our framing of it, with
attribution, and it is genuinely unruled. (2) Where a **minted anchor's release charge** lands. (3)
Whether the **found-anchor** case admits the quiet-window certificate. (4) The **by-value-claim drop
fix** — `x T` by value at a claim-typed `T` decs the claim at scope end, possibly to zero, under an
*empty* effect clause, because the drop-side generated bound carries no effect half where the clone side
does; until their fix lands, the drop path must not assume purity there. Items 2 and 3 land on the
consumer-side claim rules and are the only upstream items any near-term work touches. They have also asked
for **our** input on the `&x`-at-a-claim-place question above.

## Lessons learned

*Accumulates wisdom, not events. One or two sentences per entry; prune what nobody can act on.*

- **Group invalidation is not Rust's exclusion; do not model the borrow as a lock.** A borrow constrains
  nobody. A *destructive* op on an **ancestor** group invalidates references into its **child** groups
  (downward only); a plain member write, a sibling, or mutating the reference's own contents invalidates
  nothing. The destroy is the aggressor and the borrow is the victim — the reverse of a Rust `&mut`.
- **`...` is the descendant group operator (`&T in g...`), not a comment.** Three dots lex as three `.`
  symbols, consumed only by `parse_group` (`templex_parser.rs`) in group position; do not resurrect
  `...`-as-comment.
- **A use-after-churn needs a child group, and an inline-only plain struct forms none.** A child group
  comes only from an independently-destroyable owned thing — a collection/array element, a `Box` pointee, a
  `Variant`/interface payload — never an inline scalar or struct field. So `struct Fleet { flagship Ship; }`
  has nothing a churn can dangle; a rung-2 test needs a runtime-sized-array element or the like.
- **In a borrow-checker fixture, a move is `^local` (prefix), and arithmetic on a borrowed member does not
  read out.** `f(&x, x)` does not move `x` (bare `x` yields a borrow, rejected against an owned param) and
  `x^` does not parse (postfix `^` unshipped); write `^x`. `set a.hp = a.hp + 1` on an `&Entity` member
  fails with `+(&i32, &i32)` not found, so fixtures use literal member writes.
- **To split a class of derivation failures by root cause, panic on the bad state and census the panic's
  caller frame.** When `groupify` began panicking on any underivable borrow, grepping the failing suite's
  backtraces for the panicking helper (`member_result` vs `call_result_kind`) partitioned every failure
  into closure-capture vs optional-borrow-return in one run.
- **A runtime-sized-array local can now drop cleanly** via a closure-free `DropFunctor<T>` in
  `arrays.vale`; do not resurrect the closure-based array drop (closures are not working yet).
- **Do not mirror a foreign reference implementation's structure as a template.** Copying Polonius's
  file/struct layout would import loans/origins/constraints — abstractions for jobs (region inference,
  exclusivity) group borrowing does not have; write a small own-shape layering doc instead.
