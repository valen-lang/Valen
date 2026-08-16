# Bound Return Resolution (BRRZ)

When a generic function has a `where func(…)R` bound and the caller has supplied values that determine the bound's parameter runes, the compiler resolves the bound function at call-site solve time and takes its return type as R. This restores a capability that was lost in the 2022 templates-to-generics transition (see MSAE at `docs/Generics.md:862`).

For example, `callAndReturn<E, G>(g &G) E where func(&G)E` called with a concrete closure lets the compiler discover E by resolving `__call(&closure)` and reading its return type. Before BRRZ, the solver would stall on E because no rule shape could infer it.

BRRZ is a relaxation of `ResolveSR`'s puzzle in `compiler_solver.rs`, plus a second handler branch there. The existing puzzle `Vector(paramsListRune, returnRune)` still fires the `predict_function` path (fabricate a placeholder prototype, postpone real resolution per SFWPRL). A new puzzle `Vector(paramsListRune)` fires when only params are known — in that case the handler calls `resolve_function`, the same real overload lookup the post-solve phase uses in `infer_compiler.rs`, and commits both the prototype and the discovered return rune.

**How this affects other rules in the same solve.** The paired `CallSiteFuncSR` rule for the same bound (emitted by `postparsing/rules/templex_scout.rs`) shares the prototype rune and the param/return runes. When BRRZ commits them all via the new branch, `CallSiteFuncSR` fires next and redundantly re-commits the same values. `simple_solver_state.rs`'s `commit_step` permits duplicate conclusions when values agree, so this is safe but cosmetically extra work in the solver trace.

**Why this doesn't re-enable the HashMap regression.** The removed capability allowed inferring generic params from an annotated outer return type (e.g., `func HashMap<K,V,H,E>(hasher H, equator E) HashMap<K,V,H,E>` with no `where` clause), which let callers skip declaring bound args. BRRZ is a different operation: it resolves a `func(…)R` bound using caller-supplied arg coords that arrived via `InitialSend`, and the post-solve bound-arg check in `infer_compiler.rs` still verifies the caller declared everything the callee needs. The safety invariant "callers declare the bound args their callees need" is unchanged — BRRZ only moves the timing of when individual runes get solved, not what gets checked.

**Mid-solve real resolution is safe here (unlike what SFWPRL might suggest).** `findFunction` reads only from caller's snapshotted env, settled `CompilerOutputs` caches, and freshly-constructed inner solvers. The outer solver's in-flight state is never read. The `CompilerOutputs.signatureToFunction` cache terminates nested stamping recursion. Per SROACSD, `ResolveSR` and `DefinitionFuncSR` never coexist in the same rule set, so there's no rule-ordering hazard with a sibling `DefinitionFuncSR` declaring something the lookup would need.

**Where BRRZ is triggered today.** `where func(&G, int)E` bounds in `src/builtins/resources/arrays.vale:36,52` (the `Array<M, E, G>` builtins). User-level calls like `Array<imm>(3, lambda)` and `#[](10, lambda)` both flow through these bounds and exercise BRRZ. The minimal non-array repro is in `src/typing/test/after_regions_tests.rs` — "Bound-driven return rune cannot be inferred from lambda (MSAE general)".

**What callers gain.** The previously-required `Array<mut, int>(n, lambda)` can now be written `Array<mut>(n, lambda)` or even `#[](n, lambda)`, with the element type inferred from the lambda's return.

**Interactions with other arcana.** BRRZ fires inside a per-call-site solve (ECSIIOSZ). It does not interact with DRSINI — BRRZ's trigger condition (a rune is the unconstrained return of a bound) never overlaps with DRSINI's trigger (a rune is an identifying generic param with a declared default), since bound return runes are derived, not identifying. BRRZ is gated by the SROACSD filters — it only fires in call-site solves, where `DefinitionFuncSR` is excluded. BRRZ's canonical cases involve lambdas passed through `where func(&G, ...)E` bounds, so the lambda specialization described in LAGTNGZ is what actually produces the prototype that BRRZ reads the return type from.

The long-form reasoning, safety analysis, and canonical tests are in `docs/Generics.md` under the BRRZ section. The rejected alternative (origin tracking) is recorded at `docs/historical/thoughts-on-origin-tracking.md`.

## See also

- `docs/Generics.md` — BRRZ section (long-form reasoning), MSAE section (historical context), "...but not return types" section (the regression that BRRZ was careful not to re-enable), SFWPRL section (predict-then-resolve pattern that BRRZ operates under).
- `docs/arcana/EachCallSiteIsItsOwnSolve-ECSIIOSZ.md` — the per-call-site solve model that BRRZ runs inside.
- `docs/arcana/DefaultRulesShouldBeIncrementalNotInitial-DRSINI.md` — DRSINI uses the same `incrementallySolve` callback pattern but for a different trigger (generic param defaults, not bound return runes).
- `docs/historical/thoughts-on-origin-tracking.md` — the rejected alternative and why it was rejected.
- `docs/arcana/ByDefaultPullFromWhereDeclared-BDPFWDZ.md` — BRRZ is a pull operation: the mid-solve real lookup reaches across to the callee's definition env rather than pre-harvesting bound info. BDPFWDZ is the broader principle.

// VCOORD: update this
- Drifted against the corpus. The `where func(&G, int)E` bound is at `arrays.vale:48`, not `:36,52`;
  the builtin is `Array<E, G>`, not `Array<M, E, G>` (the mutability param is gone); and the
  user-level example `Array<imm>(3, lambda)` is retired syntax.
- The safety argument turns on "Per SROACSD, `ResolveSR` and `DefinitionFuncSR` never coexist in the
  same rule set" — **SROACSD has no file anywhere**, so the load-bearing citation is unresolvable.
- Scala spellings (`findFunction`, `solveForDefining`/`solveForResolving`).
- Worth a pointer to its sibling family: this covers *function* bounds
  (`rune_to_bound_prototype`); impl bounds (`rune_to_bound_impl`, from `where implements(..)`) are
  not rules at all and are minted post-solve. Nothing here applies to them.
