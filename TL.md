# Typing Pass Migration — TL Handoff

## Required Reading

Read these before doing anything else, in this order:

1. **This file** — read top to bottom before starting work
2. **`docs/architecture/typing-pass-design-v3.md`** — architecture and design decisions for the typing pass migration
3. **`FrontendRust/docs/architecture/typing-pass-arenas.md`** — current arena shape and lifetime model
4. **`FrontendRust/docs/arcana/SealedInternedConstruction-SICZ.md`** — the `MustIntern` seal pattern; how `IdT`-style types are constructible only via the interner
5. **`FrontendRust/docs/arcana/IdentityEqualityOnIdentityBearingTypes-IEOIBZ.md`** — identity-bearing types impl `PartialEq` via `std::ptr::eq`; wrappers derive
6. **`FrontendRust/docs/arcana/WhenValuesShouldBeInterned-WVSBIZ.md`** — heuristics + Scala-parity rule for when a type is Interned vs Value-type
7. **`FrontendRust/zen/migration_principles.md`** — migration rules (DCCR, RCSBASC, etc.)
8. **`FrontendRust/zen/testing.md`** — test conventions (`find_func_named`, `expect_1`/`expect_2`, etc.)
9. **`docs/skills/migration-drive.md`** — the junior's instructions (know what they're following)

For historical slab-by-slab progress (Slabs 0–14b), see `docs/historical/slab-chronicle.md`. Per-slab handoff docs with translation tables and gotchas are in `FrontendRust/docs/migration/handoff-slab-*.md`. The historical design docs (`docs/historical/typing-pass-design-v1.md`, `docs/historical/typing-pass-design-v2.md`, `docs/historical/typing-pass-migration-setup.md`) are obsolete — they each carry "DO NOT FOLLOW" banners.

---

## Guiding Principle

**1:1 Scala parity is the highest priority.** Every design decision defers to matching Scala's structure, naming, and behavior. No novel logic, no reorganization, no Rust-idiomatic "improvements" beyond what Rust strictly requires to compile. Where Rust can't directly mirror Scala, prefer `panic!`/`assert!` placeholders over inventing alternatives. When in doubt, port Scala verbatim.

**This applies to bodies as much as to types.** When porting a Scala function body, translate every line literally — including assertions, size checks, intermediate bindings, redundant-looking branches, and code paths that appear dead. Do **not** simplify, collapse, flatten, inline, or "optimize away" anything on the way over, even if you can prove the simplification is semantically equivalent. The Scala source is the spec; your job is transcription, not refactoring. If the Scala writes `if (results.size > 1) vfail()` over a value whose size can never exceed 1, the Rust writes the same check. If the Scala binds an intermediate variable that's used once, the Rust binds the same intermediate. If the Scala has a `match` arm that pattern-matches a case the caller "couldn't reach," the Rust has the same arm. Parity-preserving translation is a narrow target — keep the diff against Scala visually obvious so reviewers can verify line-for-line.

**TLs and reviewers: enforce this on hand-offs.** When suggesting a body translation to a junior, never propose a simplification of the Scala — quote the Scala verbatim and tell them to mirror it. When reviewing a junior's diff, flag any place where the Rust shape diverges from the Scala shape, even when the divergence "obviously works." The migration's value comes from the audit trail; a clever translation that nobody can verify against the Scala is worse than a verbose one that anyone can.

---

## Where We Are

The scaffolding phase is complete. Slabs 0–14b built out every type definition, all ~210 method signatures with proper lifetimes, and all placeholder types (only `IRegionNameT` retains `_Phantom` — 0 Scala implementors found). `cargo check --lib` is clean (0 errors, 22 warnings — all minor lifetime elision).

**Current work: body migration (Slab 15+).** 141 panic-stubbed method bodies across 17 files, plus 88 stale stubs from earlier slabs. All are functionally equivalent — they need real Scala-parity implementations. Work is test-driven: pick a test, run it, implement the body it hits, repeat.

Test infrastructure: 14 test files in `src/typing/test/` with 173 test bodies (compiler_tests 91, compiler_solver_tests 27, compiler_virtual_tests 18, compiler_mutate_tests 12, compiler_lambda_tests 10, compiler_ownership_tests 11, compiler_project_tests 3, compiler_generics_tests 1). All currently panic at the first method body they exercise.

**Recent infrastructure work (interning approach hardening, post-Slab-15b):** During body migration of `simple_program_returning_an_int_explicit`, an assertion `header.to_signature().id == needle_signature.id` was failing because `assemble_name` was constructing un-interned `IdT` values that had different `init_steps` slice pointers than the canonical interned ones. This surfaced a broader gap: Rust's "Interned" classification per @TFITCX was discipline-enforced, not compiler-enforced. Three rounds of work followed, all infrastructure (no body migration progress beyond unblocking the test):

1. **Sealed 21 TFITCX-Interned types** with `MustIntern` (private-constructor witness field per @SICZ): `IdT`, the 15 transient (slice-bearing) Name types (`ImplNameT`, `FunctionNameT`, `StructNameT`, `InterfaceNameT`, etc.), and the 5 Scala-`IInterning` kind payloads (`StructTT`, `InterfaceTT`, `StaticSizedArrayTT`, `RuntimeSizedArrayTT`, `OverloadSetT`). Construction now requires going through the interner — compile error elsewhere. Caught two un-interned construction sites at compile time (`assemble_name`, `get_placeholder_template`); both now route through `intern_*` correctly. The ~57 simple Name types (`PrimitiveNameT`, `PackageTopLevelNameT`, `StructTemplateNameT`, etc.) are Interned per Scala parity but **not yet sealed** — extending the seal to them requires introducing `*NameValT` mirror types (same shape as the 5 kind-payload ValT mirrors), tracked in `FrontendRust/docs/todo.md`.
2. **Reconciled Rust's Interned classification with Scala's `IInterning` trait.** Audit found 14 types Rust had marked Interned that Scala leaves as plain case classes (`SignatureT`, `PrototypeT`, `KindPlaceholderT`, all 11 Templata variants, `CoordListTemplataT`). Reclassified to `/// Value-type` per @TFITCX. The Rust Interned set now matches Scala's `IInterning` set, plus `IdT` as a deliberate Rust-side optimization for `init_steps` slice-pointer-equality.
3. **Pushed identity-equality down to identity-bearing types** per @IEOIBZ. Manual `std::ptr::eq` + `std::ptr::hash` impls now live on `IEnvironmentT`, `FunctionA`, `StructA`, `InterfaceA`, `ImplA`, `FunctionHeaderT`. The 9 wrapper types in `templata.rs` (`FunctionTemplataT`, `StructDefinitionTemplataT`, etc.) just `#[derive(PartialEq, Eq, Hash)]` and the inner ptr-eq propagates. Variant env types (`PackageEnvironmentT`, `FunctionEnvironmentT`, etc.) keep their `self.id == other.id` impls — documented exception, sound because `IdT` is canonical.

Plus IDEPFL uniformity: added Val mirror types for the 5 sealed kind-payload types (`StructTTValT`, `InterfaceTTValT`, `StaticSizedArrayTTValT`, `RuntimeSizedArrayTTValT`, `OverloadSetTValT`) so the kind-payload family follows the same dual-enum pattern as everything else.

Net result: the `header_sig.id == needle_signature.id` assertion is gone; the test now progresses past it and panics at an unrelated mid-migration `compiler.rs:1043 Unimplemented: evaluate — export phase` placeholder, which is the next body to migrate.

**Latest session (env-builder & `&'t self` hardening):** While migrating `finish_function_maybe_deferred` → `declare_and_evaluate_function_body`, JR hit three blockers in sequence — all addressed at TL/architect level:

1. **Removed `FunctionEnvironmentBoxT` from Scala** for `declareAndEvaluateFunctionBody` and `evaluateFunctionBody` (the Box's `setReturnType`/`addEntry`/`addEntries` mutators are never invoked on this entry point). Edited Scala source first, updated Rust audit-trail `/* ... */` blocks to match, then changed Rust signatures to `&'t FunctionEnvironmentT` directly. SCPX clean. See "Editing Scala To Match A Rust Simplification" below.
2. **Eagerly promoted ~23 panic stubs to `&'t self`** across `function_environment_t.rs` and `environment.rs` for methods matching the wrap-self (`lookup_*_inner` → `IEnvironmentT::Variant(self)`) or embed-self (`root_compiling_denizen_env`, `make_child*`) patterns. Doc rule added as design v3 §3.4a. See "Interning Approach" rule #4 below.
3. **Added `snapshot(&self, interner)` to `TemplatasStoreBuilder`, `NodeEnvironmentBox`, `FunctionEnvironmentBuilder`** — Scala's `Box.snapshot` semantics, mid-flight freezes that don't consume the builder. Used by `evaluateFunctionBody`'s `val startingEnv = env.snapshot` line. Design v3 §3.3 updated.

JR is now unblocked on `evaluate_function_body` body migration. Driving test remains `simple_program_returning_an_int_explicit`. The current panic point is wherever the body migration lands next.

**Latest session (NodeEnvironmentBox restructuring + expression-hierarchy equality opt-out):** While JR was migrating `unletLocalWithoutDropping` and adjacent helpers, several scaffolding gaps surfaced — all addressed at TL/architect level:

1. **`result()` dispatch added on `ReferenceExpressionTE` and `AddressExpressionTE`.** Scala's `def result` lives on the trait; the slice pipeline emitted module-level placeholder free fns (`reference_expression_result`, `address_expression_result`) that didn't infer struct context. Wrapped each in an `impl<'s, 't> XExpressionTE<'s, 't> { pub fn result(&self) -> ... }` block dispatching to per-variant `e.result()` panic stubs. Same pattern as the "Proactively Add Inherited Dispatch Methods" section below.

2. **`NodeEnvironmentBuilder` renamed to `NodeEnvironmentBox`** (architect-level Scala-parity rename). The Rust "Builder" naming was a leftover from when design v3 §3.3 framed these as one-shot builders; with `snapshot()` added in slab 15c, the type now has full `Box` semantics. Renaming brought it in line with Scala's `NodeEnvironmentBox`. Same logic still pending for `FunctionEnvironmentBuilder` → `FunctionEnvironmentBoxT`. Added a comment above the struct explaining why we have a Box at all (arena allocation precludes `&mut NodeEnvironmentT`; arena slices `&'t [...]` aren't growable in place).

3. **Reversed design v3 §3.3's "Box deleted in Rust, subsumed by builder-freeze pattern" stance.** The Rust file `function_environment_t.rs` had carried 24 `// (Deleted in Rust per design v3 §3.3)` annotations on every Scala `NodeEnvironmentBox` method, with the actual Rust struct + impl living ~1100 lines later as orphans. Walked the Scala audit-trail block at lines 822-1020 and sliced in proper Rust impls adjacent to each Scala `/* def ... */`. Implemented `add_variable`, `get_all_locals`, `mark_local_unstackified` (verbatim Scala port). Panic-stubbed the other 22 methods. Deleted the orphan struct + impl from line 1950. Updated design v3 §3.3 to reflect the reversal.

4. **`local_helper.rs` 7 method signatures aligned**: `nenv: &NodeEnvironmentT<'s, 't>` → `nenv: &mut NodeEnvironmentBox<'s, 't>` to match Scala's `NodeEnvironmentBox` parameter type and the prevailing convention in `expression_compiler.rs` / `call_compiler.rs` / `pattern_compiler.rs` (~25 sites already use `&mut NodeEnvironmentBox`). Bodies still `panic!()`.

5. **Dropped `NodeEnvironmentBox::build_in` and `FunctionEnvironmentBuilder::build_in`** — both Rust-only (no Scala counterpart), zero user-facing call sites. The only finalizer that fires is `env.snapshot(...)` at `function_body_compiler.rs:267`, mirroring Scala's `Box.snapshot`. `TemplatasStoreBuilder::build_in` kept (it's a genuine one-shot builder, ~8 user-facing sites).

6. **Dropped `derive(PartialEq)` from the entire expression hierarchy in `expressions.rs`** — `ReferenceExpressionTE`, `AddressExpressionTE`, `ExpressionTE`, plus all ~50 per-variant struct types. Mirrors Scala's 52 `override def equals(obj: Any): Boolean = vcurious()` overrides in `ast/expressions.scala`. Rust's "no impl" gives a strictly stronger compile-time error vs Scala's runtime panic. Documented as a vcurious-mirror exception in IEOIBZ + TL.md §3 + design v3 §2 — Arena-allocated types that opt out of equality entirely (no derive, no impl) when the Scala counterpart `vcurious`-disables comparison. New rule for future TLs: check the Scala counterpart's `equals` override before adding `PartialEq` to a Rust port.

7. **`migration-drive.md` got two new notes**: a general "check `///` TFITCX classification before adding Clone/Copy/PartialEq derives" note (so future JRs don't paper over ownership errors with `Clone` on Arena-allocated types — the FunctionHeaderT case JR escalated today), and a "re-read this skill on every compaction" note at the top so JR picks up newly-added gotchas after context resets.

JR is now unblocked on `unletLocalWithoutDropping` body and the surrounding `make_temporary_local_defer` / `unlet_and_drop_all` family. The active test is still `simple_program_returning_an_int_explicit`.

**Latest session (Slab 15e — first end-to-end test passing):** `simple_program_returning_an_int_explicit` is now **green end-to-end**. The body of `Compiler::evaluate`'s post-deferred phase is wired up; eight `Slab 10` panic stubs in `compiler_outputs.rs` filled in (`add_function`, `get_all_structs`/`_interfaces`/`_functions`, `get_kind_exports`/`_function_exports`/`_kind_externs`/`_function_externs`, `get_instantiation_name_to_function_bound_to_rune`); `compile_i_tables` / `make_interface_edge_blueprints` / `ensure_deep_exports` got Scala-shaped skeletons with panics in the unhandled branches; `HinputsT` field types reshaped to `Vec<&'t T>` (Scala's `Vector[T]` is GC-ref); `lookup_function_by_human_name` ported verbatim; `BlockTE::result()` dispatched to `self.inner.result()`; `templata_compiler.rs`'s `assemble_rune_to_function_bound`/`_impl_bound` pattern destructures fixed (`IEnvEntryT::Templata` directly wraps `ITemplataT`, no `TemplataEnvEntryT` wrapper; `PrototypeTemplataT.prototype`; `IsaTemplataT.impl_name`). Driving test promoted to `hardcoding_negative_numbers` (the next test in `compiler_tests.rs` order).

Three meta-lessons from this session were folded into the rules below:

1. **"Aggressively panic for untested branches" applies to code paths, not data values** — a struct field that gets initialized to an empty collection on the test path is correct parity, not a wrong-answer risk. Panic only inside the bodies that wouldn't be reached. See "Good Partial Implementing" below.
2. **SPDMX-vs-TL.md tension on iteration scaffolding** — when SPDMX's heuristic flags a Scala-shaped `.map(|x| panic!())` / `.for_each(|x| panic!())` skeleton as "novel scaffolding," temp-disable SPDMX with the standard rationale; TL.md's "Good Partial Implementing" pattern wins. See "Skeleton-with-panics vs SPDMX" below.
3. **`add_function` taking an explicit `signature: &'t SignatureT` parameter** is a documented Rust-side adaptation to interning (`CompilerOutputs` doesn't hold the typing_interner). SPDMX exception B applies; comment it inline as `// Rust adaptation (SPDMX-B): ...` when adding similar interner-passing parameters.

**Latest session (Slab 15f — typing-pass test traversal + LetSE scaffolding):** `hardcoding_negative_numbers` is now **green end-to-end**. Three pieces of architect/TL-level scaffolding landed; JR is mid-migration on `simple_local`'s LetSE arm.

1. **`src/typing/test/traverse.rs`** (~1740 lines) — Rust analog of Scala's `Collector.only` / `Collector.all`. Mirrors the established postparsing precedent (`src/postparsing/test/traverse.rs`, 1093 lines). `NodeRefT<'s, 't>` enum with ~95 variants, ~75 `visit_*` walkers, 5 `#[macro_export]` macros (`collect_in_tnode!`, `collect_where_tnode!`, `collect_only_tnode!`, plus `_tnodes!` plurals). The architect chose full upfront coverage (vs incremental) — every `ReferenceExpressionTE` / `AddressExpressionTE` / `KindT` / struct-payload `ITemplataT` variant is enumerated. Stop-at-trait for the 74 `INameT` variants, `IEnvironmentT`, attribute traits, etc. (per the postparsing precedent and TL.md §"What This Plan Deliberately Does NOT Cover"). No Guardian annotations (postparsing precedent has none either — pure test scaffolding). The file's `pub mod traverse;` is in `src/typing/test/mod.rs`.

2. **`get_rune_types_from_pattern`** at `src/higher_typing/patterns.rs` — verbatim port of Scala's `PatternSUtils.getRuneTypesFromPattern` as a free `pub fn` (no Rust analog of `object PatternSUtils`). Recurses through `pattern.destructure`, appends `(coord_rune.rune, CoordTemplataType {})`, dedups preserving order. Used by the LetSE arm of `evaluate_expression`.

3. **`LetExprRuneTypeSolverEnv`** at the bottom of `src/typing/expression/expression_compiler.rs` — Scala's `new IRuneTypeSolverEnv { ... }` anonymous class at `ExpressionCompiler.scala:959` becomes a named struct + `impl IRuneTypeSolverEnv<'s>` block, closing over `&'a NodeEnvironmentBox<'s, 't>`. Same shape as `HigherTypingRuneTypeSolverEnv` in `higher_typing_pass.rs:1867` (which collapses 6 anonymous Scala impls into one named struct). `/* Guardian: disable-all */` mirrors the higher-typing precedent. The `Some(_x) => panic!()` arm requires an `ITemplataT::tyype()` getter that doesn't exist yet — separate scaffolding gap, escalate when a test path hits it. The other 3 typing-pass `IRuneTypeSolverEnv` sites (`array_compiler.rs:101`, `templata_compiler.rs:1501` factory, `overload_resolver.rs:455`) get their own per-site structs when their containing functions get migrated — don't try to unify (the factory has a `LambdaStructImpreciseNameS` special case the LetSE inline doesn't).

Three meta-lessons from this session:

1. **Anonymous Scala trait impls map to named per-site Rust structs.** Established Rust precedent: `HigherTypingRuneTypeSolverEnv` collapses 6 anonymous Scala impls in one file into one struct (because they all close over the same fields). For typing-pass, expect ~4 per-site structs (one per anonymous `new IRuneTypeSolverEnv` call) since the bodies differ. Naming convention: `<UseSite>RuneTypeSolverEnv` (e.g. `LetExprRuneTypeSolverEnv`). Place at the bottom of the file with `/* Guardian: disable-all */`. Don't unify across sites unless you've verified the Scala bodies are identical.

2. **Test-traversal scaffolding is TL territory, not JR.** When a test demands `Collector.only` (or any other large piece of test infrastructure with no Scala line-for-line counterpart), JR escalates and TL writes it. Don't expect JR to write hundreds of lines of `NodeRefT`/`visit_*` enumeration through their NNDX-restricted workflow — they shouldn't even try. TL.md §"NNDX Escalation Pattern" applies: TL adds the missing definitions directly.

3. **AIMITIPX rule applies to test-traversal patterns.** No `if matches!` or `if`-guards on `collect_only_tnode!` patterns. Rust's vanilla struct/enum patterns support literal-value matching at any nesting level, so `Some(ConstantIntTE { value: ITemplataT::Integer(-3), .. }) => Some(())` works without any guard. The macro arms support guards (`$pattern if $guard => $body`) for parity with postparsing's macro shape, but don't use them — AIMITIPX shield will block tests that do.

---

## Known Residual Items

- **~150 panic-stubbed method bodies** across 17+ files (top: expression_compiler.rs 21, templata_compiler.rs 20, infer_compiler.rs 15, function_environment_t.rs ~25). Plus ~80 stale stubs labeled "Slab 10" in compiler_outputs.rs (44) and templata_compiler.rs (36) — bumped down today by the eight `compiler_outputs.rs` Slab-10 stubs implemented in Slab 15e. The count is approximate; treat as a rough magnitude, not an exact figure.
- **dispatch_function_body_macro** and friends not wired.
- **LocationInFunctionEnvironmentT.path: Vec<i32>** in `ast/ast.rs` violates AASSNCMCX. Future cleanup turns into `&'t [i32]`.
- **IRegionNameT** retains `_Phantom` — 0 Scala implementors found, deferred.
- **30 cargo warnings** (was 22 in slab 15d, drifted up over 15e/15f) — all minor lifetime elision suggestions, mostly in `hinputs_t.rs`. Touched files in 15e/15f added zero warnings. Cleanup is cosmetic; defer.
- **`lookup_function_by_human_name` should be `lookup_function_by_str`** per SPDMX exception J's pre-approved rename table (`Frontend/TypingPass/.../HinputsT.scala:146` → Rust). Cosmetic; rename the def in `hinputs_t.rs:326` and call sites in `compiler_tests.rs`. No body changes.
- **`add_function` lacks the SPDMX-B adaptation comment** — should carry a `// Rust adaptation (SPDMX-B): signature passed explicitly because CompilerOutputs doesn't hold the typing_interner.` block above the fn. Cosmetic; documents the divergence so reviewers don't flag it.
- **`ITemplataT::tyype()` getter is unimplemented** — Scala has it on the trait; Rust needs a per-variant match returning `ITemplataType<'s>`. Surfaces in `LetExprRuneTypeSolverEnv::lookup`'s `Some(_x) => panic!()` arm. Add when a test path hits the panic.
- **3 typing-pass `IRuneTypeSolverEnv` sites un-migrated**: `array_compiler.rs:101`, `templata_compiler.rs:1501` (the `createRuneTypeSolverEnv` factory), `overload_resolver.rs:455`. Each becomes a per-site named struct following the `LetExprRuneTypeSolverEnv` pattern when its containing function gets migrated. Don't unify — Scala bodies differ.
- **`lookup_nearest_with_imprecise_name`** at `function_environment_t.rs:1079` is panic-stubbed. Will need migration when a test path actually triggers a name lookup through the LetSE arm (none of the currently-passing tests do).

---

## Good Partial Implementing

When replacing a `panic!` stub with real logic, write just the shallow structure of that scope — straight-line variable bindings, function calls, match expressions with all arms — but put `panic!` inside every new branch body, loop body, closure/lambda body, and match arm. Then only fill in the specific arms/branches the driving test actually hits. This applies recursively: when a test hits one of those inner panics, replace *that* panic with its own skeleton-with-panics, fill in only what the test needs, and so on. Each iteration expands one panic into a new layer of structure. **Aggressively panic for anything that might not be executed by current tests.** This minimizes each batch's diff and ensures untested paths crash loudly rather than silently returning wrong results.

**Important clarification: "untested branches" means untested *code paths*, not untested *data values*.** A struct field that gets initialized to `HashMap::new()` on the test path is **executed** — the initializer runs, the empty map is constructed, the struct is built. If Scala produces an empty map on the same input (e.g. a program with no impls produces empty `interfaceEdgeBlueprints`), then an empty Rust map is the *correct* parity translation, not a silent-wrong-answer hazard. Panicking inside the field initializer would break the test for no reason, since the path through it is parity-correct.

The rule applies to **branches that wouldn't be reached if the input is empty**: panic inside the loop body that iterates the (empty) collection, panic inside the match arm for a variant the input doesn't contain, panic inside the closure that's never invoked. Those are the untested code paths. The struct-field-init line itself runs unconditionally on the test path; whatever value it produces matches Scala's value on the same input, so it's correct.

When in doubt, ask "does this line *run* on the test path?" If yes, it must produce the same value Scala produces — empty values are fine. If no (e.g. it's inside a closure body the test never invokes), panic.

---

## Skeleton-With-Panics vs SPDMX

The "Good Partial Implementing" pattern (Scala-shaped iteration with panics in the closure bodies) collides with SPDMX's heuristic in a predictable way. SPDMX sees `.map(|x| panic!())` / `.for_each(|x| panic!())` / nested `for x in ... { panic!() }` and flags it as "novel scaffolding" or "Rust-only iteration structure," recommending `panic!()` for the whole function instead. But whole-function `panic!` breaks the test path, which is non-negotiable — so the skeleton-with-panics IS the right pattern, and SPDMX is the one being too aggressive.

**Resolution: TL temp-disables SPDMX on the affected function with a documented rationale.** This is a TL/architect-level move; juniors must escalate, not temp-disable themselves. Standard rationale boilerplate (paste verbatim into the disable invocation):

> Per TL.md "Good Partial Implementing": this function uses the skeleton-with-panics-in-closures pattern that the migration design endorses. The iteration structure (.map / .for_each / nested for) mirrors Scala's call graph; panics live in the closure bodies. SPDMX's heuristic flags the iteration structure as "novel scaffolding," but the structure IS the Scala parity — without it, the call graph diverges. For the empty-input case (the driving test), the closures never fire and the function is a verified no-op; for non-empty inputs they panic loudly with named placeholders. TL approval: temp-disable SPDMX here, re-enable when the closure bodies get filled in with real logic.

This came up in Slab 15e on `ensure_deep_exports`, `compile_i_tables`, and `make_interface_edge_blueprints`. Expect it to recur on every function whose Scala body is a long `.map.groupBy.mapValues` / `.foreach` chain with side-effecting closures — which is most of the typing pass's emitter-shaped functions. The temp-disable is the standard remedy; re-enable lifts when the closure bodies get implemented with real logic.

---

## Never Add `// AFTERM:` Or `// TODO:` Comments — Only The Architect Can Suggest Them

**Do not add `// AFTERM:` or `// TODO:` comments yourself, and do not tell JR to add them either.** These markers are the architect's tool for tracking deferred cleanup; they only get added when the architect explicitly asks. If you find a thing that "should be cleaned up later" and want to record it, raise it to the architect — don't park it inline as an AFTERM/TODO on your own initiative, and don't include "add a `// AFTERM: ...` comment" as a step in any reply to JR.

This applies even when the deferral seems obviously correct (e.g. a needless snapshot that could be inlined, a misplaced helper that could be moved, a panic stub that's clearly going to need real logic). The architect decides whether the deferral is worth a marker comment vs. fixing immediately vs. ignoring; defaulting to AFTERM/TODO without that decision adds noise to the codebase that nobody asked for.

If you've already drafted a reply to JR that includes "add a `// AFTERM: ...` comment," strike it before sending. If JR proposes an AFTERM/TODO themselves, tell them to drop it and escalate to the architect instead.

---

## Run Solutions By The Architect First

**Before implementing any structural fix or design change, propose the solution to the architect and wait for approval.** This includes lifetime-parameter additions, signature changes that propagate across many files, new abstractions, and any choice between alternatives. Don't start editing — even for fixes that look mechanical — until the architect has signed off on the approach. The cost of a quick check is small; the cost of unwinding a wrong-direction change across ~20 call sites is large.

---

## TL Does Only What JR Can't — Guardian-Blocked Changes Only

**Default to letting JR do the work.** TL/architect intervention is reserved for changes that Guardian would block JR on, or that require explicit architect approval per other rules in this doc. Concretely, TL handles:

- **NNDX-blocked definition adds** (new fn / struct / trait / enum / impl with no Scala line-for-line counterpart): per "NNDX Escalation Pattern" above.
- **SPDMX-blocked skeleton-with-panics** patterns: per "Skeleton-With-Panics vs SPDMX" above — TL issues the temp-disable.
- **Scala source edits** to match a Rust simplification: per "Editing Scala To Match A Rust Simplification" above — TL/architect-only.
- **Guardian annotations** for new definitions without Scala counterparts: per "Guardian Annotations For New Definitions Without Scala Counterparts" — TL adds `/* Guardian: disable-all */` or empty `/* */` blocks.
- **Test-traversal / large test infrastructure** with no Scala line-for-line counterpart: per slab 15f's `traverse.rs` precedent.
- **Structural / lifetime / cross-file refactors** that need architect sign-off per "Run Solutions By The Architect First."

**Everything else is JR's job, including signature shape changes that don't trip Guardian.** Adding an `interner` parameter to a method is JR's call — they don't need to escalate. Adding `&'t self` where needed for back-pointer-emitting methods is JR's call. Renaming a parameter from `env` to `nenv` to match Scala is JR's call. Threading a new field through three call sites is JR's call. The line is "would Guardian fire on this edit?" — if no, JR does it; if yes, TL does it.

This shifts work to JR aggressively. The benefit: faster iteration, less TL bottleneck, JR develops more autonomy on the parts of the migration that are mechanical. The cost: occasional JR mistakes that get caught at code review. That trade-off is correct — the alternative (TL touches every divergence) is what we've been doing and it's slower than it should be.

When in doubt, the rubric: would I want to see this in a JR diff or in a TL diff at hand-back? If "JR diff is fine, I'll spot-check at review," then JR does it.

---

## Adding `interner` Parameters Is Always A Good Rust Adaptation

When Scala parity wants behavior that needs the typing interner (e.g. materializing a `&'t T` snapshot, allocating a slice, interning a name), and Scala didn't have that parameter because Scala used GC + mutable references, **threading an `interner: &TypingInterner<'s, 't>` parameter through the Rust signature is always a fine Rust adaptation.** Document with a `// Rust adaptation (SPDMX-B): ...` comment explaining why the interner is needed (typically: arena-allocation of a result that Scala mutated in place, or re-allocation of a slice that Scala grew via GC).

Concrete cases that have come up:
- `CompilerOutputs::add_function(signature: &'t SignatureT, ...)` — Scala interned via implicit `Interner` access; Rust threads the signature explicitly because `CompilerOutputs` doesn't hold the typing_interner. (Slab 15e.)
- `NodeEnvironmentBox::nearest_block_env(interner)` — Scala's Box delegates to its `var nodeEnvironment` directly; Rust's Box stores mutations in Vecs out-of-arena per design v3 §3.3, so producing a `&'t NodeEnvironmentT` requires snapshotting through the arena.
- The `*Box::add_entry` / `add_entries` family already takes `interner` — same pattern, same reason.

**This is JR-level work, not a TL escalation.** Adding an interner parameter doesn't trip Guardian (it's not a new definition; it's a signature shape change on an existing one). Per "TL Does Only What JR Can't" above, JR should make this kind of adaptation themselves and proceed. If JR is uncertain whether the interner-add is the right shape, they can escalate, but the default answer is yes.

---

## Don't Simplify Scala On The Way Over

Restating the guiding principle as an operating rule because TLs slip on it: **when handing a body off to a junior, quote the Scala verbatim and instruct them to translate every line.** Do not flatten redundant checks, do not collapse impossible branches, do not inline single-use bindings, do not reason "well in Rust we can just…". If you find yourself writing "the Rust method already returns `Option`, so it's a direct return" or "we can skip this size check because it can't happen" in a hand-off, stop — that's a parity violation in the making. The whole migration's auditability rests on the diff being a literal line-for-line port. A junior who follows a verbatim Scala translation produces a reviewable patch; a junior who follows a TL's "smarter" translation produces a patch nobody can compare against the source.

---

## Editing Scala To Match A Rust Simplification

Sometimes the Scala carries machinery that's genuinely dead weight in Rust — most often a Scala-side wrapper or indirection whose mutation/dispatch surface is unused on the call paths you're porting (`FunctionEnvironmentBoxT` is the canonical example: case class with `var nodeEnvironment` and `setReturnType`/`addEntry` mutators, but `evaluateFunctionBody` only ever reads through it). The temptation is to write the Rust without the wrapper and add a "diverges from Scala" note. **Don't.** That note rots; reviewers can't verify the divergence; the audit trail erodes.

**Instead: edit the Scala source first to match what the Rust will become, then update the Rust audit-trail `/* ... */` blocks to reflect the new Scala, then make the Rust change.** Do this only when:

1. The Scala wrapper/indirection is *unused* on the specific call paths you're porting (verify with `grep` across the relevant Scala module — no `setReturnType`, no `addEntry`, no other mutators invoked through it).
2. The Rust replacement is design-doc-blessed (e.g., design v3 §3.3 already says `FunctionEnvironmentBoxT … deleted in Rust`).
3. After the edit, SCPX still passes `--check-all`.

The result: the Rust port is back to being a literal transcription of the (now-simplified) Scala, the `/* ... */` audit blocks faithfully quote the new Scala, and reviewers can compare line-for-line as before.

This is a **TL/architect-level move only.** Juniors must never edit Scala — they escalate. The architect signs off on every Scala edit (`Run Solutions By The Architect First` applies). Today's example: removing `FunctionEnvironmentBoxT` from `declareAndEvaluateFunctionBody` and `evaluateFunctionBody` in `Frontend/TypingPass/.../FunctionBodyCompiler.scala`, then updating `function_body_compiler.rs` audit blocks, then changing the Rust signatures to `&'t FunctionEnvironmentT`.

---

## Cleaning Up After The Slice Pipeline

The slice pipeline (`slice-start` → `slice-rustify` → `slice-placehold` → reconcile) is the right tool for bringing a file with raw `/* scala */` comments up to SCPX parity. It's also incomplete in known ways. After running it on a non-trivial typing-pass file, plan on a manual cleanup pass before `cargo check` will be clean. These are the lessons from running it on `env/environment.rs`.

### `slice-placehold` doesn't infer struct context

For each `// mig: fn foo` it emits `pub fn foo<'s, 't>(&self, …) { panic!() }` at **module scope**. It does not look at what Scala class the `def` is inside, and it does not wrap the stub in an `impl SomeT<'s, 't>` block.

Two failure modes follow:

1. **Cross-variant name collisions.** A Scala trait method overridden by N case classes (e.g. `lookupWithNameInner` overridden by `PackageEnvironmentT`, `CitizenEnvironmentT`, `ExportEnvironmentT`, `ExternEnvironmentT`, `GeneralEnvironmentT`) becomes N module-level `pub fn lookup_with_name_inner` stubs that all collide (`E0428`).
2. **Invalid `&self`.** `&self` outside an `impl`/`trait` is not valid Rust (`E0061`-style) and the per-fn `<'s, 't>` generics on a free fn don't have anywhere to come from in a real method dispatch.

**Cleanup**: wrap each stub in the right `impl<'s, 't> SomeT<'s, 't> where 's: 't { … }` block, **drop the per-fn `<'s, 't>` generics** (the impl provides them), and indent the existing Scala `/* … */` to live inside the impl alongside the Rust fn (per the SCPX adjacency rule, §"Preserve The `/* scala */` Audit Trail" in the design doc). One impl block per stub matches the rest of the file's pattern; consolidating multiple methods into one impl is allowed but not required.

### `slice-placehold` emits bogus `eq`/`hash_code` stubs

Scala's `override def equals/hashCode` is realized in Rust by `impl PartialEq`/`impl Hash`, not by methods named `eq`/`hash_code`. The placehold agent will sometimes emit `pub fn hash_code(&self) -> i32 { panic!() }` stubs that don't correspond to any real Rust dispatch.

**Cleanup**: replace the bogus `pub fn` body with a one-line marker comment:
```rust
// mig: fn hash_code
// (Realized by `impl Hash for FooT` below.)
/*
  override def hashCode(): Int = …
*/
```
Keep the `// mig:` marker (preserves the audit trail) and the Scala `/* … */` block (SCPX). Just don't pretend there's a Rust method.

### NRDX blocks multi-fn diffs — go one fn at a time

The `NoRenamedDefinitions-NRDX` shield's heuristic flags consecutive context-swaps as renames. An Edit that wraps **two adjacent** `// mig: fn foo` and `// mig: fn bar` stubs in their impl blocks in one shot will trip the shield with "fn foo renamed to bar" — even though no rename is happening.

**Cleanup**: do one stub per Edit. Slow but predictable.

### Verify with cargo + SCPX after

After cleanup, two checks:

1. `cargo check --manifest-path FrontendRust/Cargo.toml --lib > tmp/<session>.txt 2>&1` — must be 0 errors. Pre-existing warnings are fine; new ones aren't.
2. `cargo run --manifest-path Luz/shields/ScalaCommentParity-SCPX/Cargo.toml --release -- --check-all` — must report `All 230 files OK`. SCPX is the canary that the audit trail is intact through the wrap.

### Don't dispatch the orchestrator on a hand-edited file

The slice-orchestrator runs all six steps. If the file already has hand-written Rust impls (like `env/environment.rs` did before this session), reconcile-mark only catches the matching-name old definitions and leaves the rest in place. The colliding fresh placehold stubs then need the manual `impl`-wrap cleanup above. Plan for it; don't expect the orchestrator alone to leave a compile-clean file when the input was mid-state.

---

## The Interning Approach (read before adding any new typing-pass type)

Three layered rules govern interning, sealing, and equality. The first decides *what* to intern; the second decides *how the rule is enforced*; the third decides *how `==` works*. All three are documented in arcana — the summary below is for quick orientation.

### 1. Decide Interned vs Value-type per Scala parity (@WVSBIZ)

- A Rust type is `/// Interned (see @TFITCX)` **if and only if** Scala's counterpart `extends IInterning` (`INameT`, `ICitizenTT`, `StaticSizedArrayTT`, `RuntimeSizedArrayTT`, `OverloadSetT`).
- The one deliberate Rust-side divergence is `IdT` — Scala leaves it a plain case class but Rust interns it for `&'t [INameT]` slice-pointer-equality performance.
- Templata types (`CoordTemplataT`, `KindTemplataT`, `PlaceholderTemplataT`, `FunctionTemplataT`, ...), `SignatureT`, `PrototypeT`, `KindPlaceholderT`, `CoordListTemplataT` — **all `/// Value-type`** despite some still flowing through `intern_templata_payload` for caching. The interner method exists; the seal does not.
- When in doubt: grep Scala for `extends IInterning` / `with IInterning` on the type. If absent, Value-type.

### 2. Sealed Interned types via `MustIntern` (@SICZ)

Every `/// Interned` type carries:

```rust
pub _must_intern: crate::typing::typing_interner::MustIntern,
```

`MustIntern(())` has a private unit-field constructor accessible only inside `typing_interner.rs`. Constructing such a literal anywhere else fails compilation with E0423 ("constructor is not visible here due to private fields"). The intern method (`intern_id`, `intern_struct_tt`, etc.) is the only path.

When you add a new Interned type:
1. Add `pub _must_intern: MustIntern,` as the **last field** of the struct.
2. Construct it inside the corresponding `intern_*` method via `_must_intern: MustIntern(())`.
3. Per IDEPFL/DSAUIMZ, also define a parallel `*ValT` mirror struct (no `_must_intern`) that callers pass into `intern_*`. Five examples in `types.rs`: `StructTTValT`, `InterfaceTTValT`, etc.

### 3. Identity equality on identity-bearing types (@IEOIBZ)

Types with identity (anything `/// Arena-allocated` per @TFITCX, accessed via `&'t` references, where two distinct allocations are distinct things) implement their own `PartialEq`/`Eq`/`Hash` via `std::ptr::eq`/`std::ptr::hash` on `&self`:

```rust
impl<'s, 't> PartialEq for FooT<'s, 't> {
  fn eq(&self, other: &Self) -> bool { std::ptr::eq(self, other) }
}
impl<'s, 't> Eq for FooT<'s, 't> {}
impl<'s, 't> std::hash::Hash for FooT<'s, 't> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) { std::ptr::hash(self, state) }
}
```

Wrappers that hold `&'t FooT` (or `&'s` for higher-typing types) just `#[derive(PartialEq, Eq, Hash)]` — the derived field comparison deref-calls the inner ptr-eq impl. Don't write a manual `std::ptr::eq(self.foo, other.foo)` impl on the wrapper; that's the IEOIBZ refactor we just finished undoing.

Identity types with this pattern: `IEnvironmentT`, `FunctionA`, `StructA`, `InterfaceA`, `ImplA`, `FunctionHeaderT`, `IdT` (slight outlier — it ptr-eq's `init_steps`'s slice pointer, not the whole struct, because it lives by value, not by reference).

Documented exceptions (kept as `self.id == other.id`): the variant env types (`PackageEnvironmentT`, `CitizenEnvironmentT`, `FunctionEnvironmentT`, etc.). These are sound because `IdT` is sealed/canonical, and these types are usually compared via `&'t IEnvironmentT` (which goes through `IEnvironmentT::eq`'s ptr-eq) anyway.

**Equality opt-out (vcurious mirror).** Some Arena-allocated types intentionally have **no equality at all** — no derive, no impl. The expression hierarchy is the canonical case: `ReferenceExpressionTE`, `AddressExpressionTE`, `ExpressionTE`, plus the ~50 per-variant struct types in `ast/expressions.rs`. Scala has 52 `override def equals(obj: Any): Boolean = vcurious()` overrides on these in `ast/expressions.scala` — Scala panics on any `==` call. Rust mirrors this by not impl-ing `PartialEq`/`Hash` at all, which is strictly stronger (compile error vs runtime panic).

Before adding a new arena-allocated type, check the Scala counterpart's equality story:

| Scala has | Rust does |
|---|---|
| Default case-class equality (structural) | (Doesn't happen for arena-allocated types in practice) |
| `vcurious()` equals/hashCode overrides | **No PartialEq/Hash impl or derive at all** |
| `override def equals` calling `==` on a specific identity field (e.g. `id`) | Manual ptr-eq via `std::ptr::eq` per @IEOIBZ |

The classification `/// Arena-allocated` is about lifetime/storage (the type lives in the typing arena, accessed via `&'t T`), not about identity semantics. Most Arena-allocated types do have identity and follow @IEOIBZ; the expression hierarchy is the documented opt-out.

### 4. `&'t self` on arena/interned-type methods that emit a `'t`-back-pointer to self (design v3 §3.4a)

Methods on arena-allocated/interned types default to `&self`. Promote to `&'t self` **only** when the method's output borrows from `self` itself — not from a field that's already a `&'t T` ref. Three concrete shapes trigger it:

1. **Embed self as a back-pointer** — `make_child_*` returning `Builder { parent: self, ... }`.
2. **Wrap self in a wrapper enum** — `lookup_*_inner` calling `helper(IEnvironmentT::Variant(self), ...)`.
3. **Return a borrow into a by-value field** — usually sidesteppable by returning the field by value (Copy types like `IdT`).

Pure getters of already-`'t`-ref fields don't need it (`fn templatas(&self) -> &'t TemplatasStoreT { self.templatas }` works under any receiver lifetime — `&'a (&'t T)` flattens to `&'t T` by Copy). Promotion is a per-method decision based on the output, not a blanket rule on the type.

When scaffolding a new panic stub: if Scala wraps `this` in an enum or stores it as a child's parent, write the Rust signature as `&'t self` proactively — even before the body is implemented. Avoids the JR-blocked-on-receiver-lifetime cycle. As of this session, ~28 sites in the typing pass use `&'t self` (5 wired + 23 panic stubs proactively promoted).

Full rationale and the `&self` vs `&'t self` decision tree live in design v3 §3.4a.

---

## TemplatasStoreT Is `&'t` In All Env Structs

As of Slab 15b, all environment structs (`CitizenEnvironmentT`, `FunctionEnvironmentT`, `BuildingFunctionEnvironmentWithClosuredsT`, `BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT`, `NodeEnvironmentT`, `NodeEnvironmentBoxT`, `GeneralEnvironmentT`, `PackageEnvironmentT`) hold `&'t TemplatasStoreT<'s, 't>` instead of owned `TemplatasStoreT<'s, 't>`. This matches Scala's GC reference semantics — Scala environments just hold a reference to the `TemplatasStore`, they don't own it.

**When building a new env struct**, arena-allocate the store first:
```rust
let store = self.typing_interner.alloc(new_templatas_store);  // &'t TemplatasStoreT
```

**When copying an env's templatas into a child env**, just copy the `&'t` reference — no clone needed:
```rust
templatas: parent_env.templatas,  // copies the &'t pointer
```

`TemplatasStoreT::add_entries(&self, ...)` returns a new owned `TemplatasStoreT`. Arena-allocate the result before storing:
```rust
let new_store = self.typing_interner.alloc(
    near_env.templatas.add_entries(self.typing_interner, self.scout_arena, entries)
);
```

`TemplatasStoreBuilder::build_in` already returns `&'t TemplatasStoreT` (it arena-allocates internally).

---

## Proactively Add Inherited Dispatch Methods

The slice pipeline generates stubs only for methods defined **directly on** a Scala trait's body. When a child trait extends a parent (e.g. `IInDenizenEnvironmentT extends IEnvironmentT`), the child enum needs its own delegation methods that widen `self` to the parent enum and call through. The pipeline doesn't generate these — they're invisible inheritance in Scala but explicit wiring in Rust.

**Don't wait for serial JR escalations.** When you see a Scala child trait extending a parent, proactively add all inherited methods on the child enum. Each follows the same pattern:

```rust
impl<'s, 't> IInDenizenEnvironmentT<'s, 't> where 's: 't {
  pub fn method_name(&self, ...) -> ReturnType {
    let as_env: IEnvironmentT<'s, 't> = (*self).into();
    as_env.method_name(...)
  }
  /* Guardian: disable-all */
}
```

Similarly, factory methods on name-template traits (`make_function_name`, `make_struct_name`, `make_interface_name`, `make_impl_name`, `make_citizen_name`) are abstract in Scala and need per-variant match dispatch on the Rust enum. These follow the interning pattern:

```rust
IFunctionTemplateNameT::FunctionTemplate(tmpl) => {
  interner.intern_name(INameValT::Function(FunctionNameT {
    template: tmpl,
    template_args,
    parameters,
  }))
}
```

All of these were swept in Slab 15b. If new trait hierarchies are scaffolded, do the same sweep before handing off body migration to a junior.

---

## NNDX Escalation Pattern

**When a junior is blocked by NNDX on a legitimate Scala counterpart**, the issue is incomplete scaffolding, not a bad shield. The TL adds the missing definition directly — don't temp-disable NNDX. NNDX exists to route definition-creation to the right authority level; the junior escalating is the system working correctly.

Example: Scala's `def globalEnv` on `IEnvironmentT` trait (line 60) becomes a `fn global_env()` match-dispatch method on the Rust enum (per SSTREX). If the slice pipeline didn't generate it, the junior hits NNDX when they try to add it. Correct response: junior stops and escalates; TL adds the accessor.

**How to slice in a missing Rust definition.** The Scala comment blocks are the audit trail — every Rust definition must sit directly above its Scala counterpart. When adding a missing definition:

1. Find the Scala `def` inside its `/* ... */` comment block.
2. Split the comment block: close `*/` just before the target `def`, insert the Rust `impl` block, then reopen `/*` to resume the remaining Scala.
3. The Scala `def` line must end up **inside** the Rust `impl` block, as an inline `/* ... */` comment immediately after the Rust `fn` body — not after the `}` that closes the `impl`. This keeps the Scala counterpart visually adjacent to its Rust translation.

```rust
impl<'s, 't> IEnvironmentT<'s, 't> where 's: 't {
  pub fn global_env(&self) -> &'t GlobalEnvironmentT<'s, 't> {
    match self {
      IEnvironmentT::Package(e) => e.global_env,
      // ...
    }
  }
  /*
    def globalEnv: GlobalEnvironment
  */
}
```

Not like this (Scala comment stranded outside the impl):
```rust
  // WRONG — comment is after the impl's closing brace
}
/*
  def globalEnv: GlobalEnvironment
*/
```

---

## Guardian Annotations For New Definitions Without Scala Counterparts

When adding a Rust function that has no direct Scala counterpart (e.g. delegation wiring for Scala trait inheritance, `From`/`TryFrom` impls, interning Val/Query structs), Guardian's NNDX shield will fire. The TL is ordained and can push through, but must annotate the new code so Guardian doesn't fire on future edits either:

- **Pure wiring (no logic)** — delegation methods, `From`/`TryFrom` match-dispatches, trivial accessors that just forward to another method. Add `/* Guardian: disable-all */` after the function/impl block. These are mechanical and don't need shield scrutiny.
- **Contains logic** — anything with conditionals, assertions, non-trivial transformations. Add an empty `/* */` after the function body (satisfies SCPX's requirement for a Scala comment block, signaling "this definition was reviewed and has no Scala counterpart").

This is TL/architect-level knowledge. The junior (migration-drive.md) should never add these annotations — they escalate to the TL instead, which is the system working correctly.

---

## Preserve The `/* scala */` Audit Trail

The typing/ skeleton has a `/* ... */` block with the Scala source directly below every Rust definition. The `.claude/hooks/check-scala-comments` pre-commit hook does exact-match comparison and rejects any edit inside those blocks. Rules:

- Replace the empty Rust stub in-place with the real definition. Keep the Scala `/* ... */` block below unchanged.
- Never move a Rust definition away from its Scala block.
- A Rust definition grown to need helper structs (e.g. an IdValT companion for an IdT) gets its companion block **adjacent to** the main definition, with a `// (no scala counterpart — …)` note.
- **After any change that touches Scala comment blocks** (splitting, moving, adding new impl blocks between them), run the SCPX shield to verify structural integrity:
  ```
  cargo run --manifest-path Luz/shields/ScalaCommentParity-SCPX/Cargo.toml --release -- --check-all
  ```

---

## Test Promotion Workflow

All 173 tests in `src/typing/test/` have `#[ignore]` except the currently-active test. This means `cargo nextest run` only runs the active test(s), so JR sees regressions immediately — a failure means something they changed broke, not a pre-existing panic.

**When JR reports a test passing:**
1. Verify the pass yourself: `cargo nextest run --manifest-path ./FrontendRust/Cargo.toml > ./tmp/slab15-tests.txt 2>&1`
2. Remove `#[ignore]` from the next simplest test in `compiler_tests.rs`
3. Hand off to JR again

**Choosing the next test:** Start with `compiler_tests.rs` (91 tests, simplest programs). Within that file, go roughly top-to-bottom — the tests are ordered from simple to complex. Once `compiler_tests.rs` is done, move to `compiler_solver_tests.rs`, then `compiler_virtual_tests.rs`, etc.

**Keep passed tests un-ignored.** They serve as regression guards. If a passed test starts failing, that's a real regression JR needs to fix before continuing.

---

## How To Continue

1. **Read the design doc** (`docs/architecture/typing-pass-design-v3.md`) top to bottom. Also read `FrontendRust/docs/architecture/typing-pass-arenas.md` for the current arena shape.
2. **Body migration is test-driven.** The active test (currently `simple_program_returning_an_int_explicit`) drives which panic stubs get implemented. See "Test Promotion Workflow" above.
3. **Don't commit.** The human handles all commits and tags. Hand back uncommitted working trees at batch boundaries.
4. **Group related bodies** into coherent batches for review. Each batch gets a handoff doc listing target methods, driving test(s), and Scala translation gotchas.

---

## Suggested Process For The Incoming TL

- Spawn a junior for each batch. Write a handoff listing the target methods, the test(s) that exercise them, and any Scala translation gotchas for those specific bodies.
- When a design diverges from the design doc, record the divergence in `FrontendRust/docs/reasoning/<topic>.md`. Then update the design doc.
- **Never commit.** Juniors and TLs finish a batch, run `cargo check --lib` clean, self-review their diff, then hand back to the human with uncommitted changes.
- **Expect and invite push-back.** Handoffs are proposals, not spec.
- **Scope discipline.** If edits land in `TL.md`, the design doc, or reasoning docs, announce in the hand-back summary, not folded silently into the diff. TLs revert off-scope edits before review.

---

Questions? The reasoning docs + the per-slab handoffs have additional context. When in doubt, prefer Scala parity over Rust-idiomatic optimization — that's the guiding principle.
