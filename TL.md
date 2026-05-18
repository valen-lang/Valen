# Typing Pass Migration — TL Handoff

**JR does not have access to this file.** When citing TL.md sections in hand-backs to JR, paraphrase the relevant rule inline rather than referencing it by name — JR can't look it up.

**Re-read this file every time you compact** — it changes often, and the prior conversation drops on compaction but this file doesn't.

**Brevity rule:** any addition to this file must be one sentence, max 25 words, unless the user explicitly asks for more.

## Required Reading

Read these, right now, before responding to the user. Read them in full.

1. **This file** — read top to bottom before starting work
2. **`docs/architecture/typing-pass-design-v3.md`** — architecture and design decisions for the typing pass migration
3. **`FrontendRust/docs/arcana/SealedInternedConstruction-SICZ.md`** — the `MustIntern` seal pattern; how `IdT`-style types are constructible only via the interner
4. **`FrontendRust/docs/arcana/IdentityEqualityOnIdentityBearingTypes-IEOIBZ.md`** — identity-bearing types impl `PartialEq` via `std::ptr::eq`; wrappers derive
5. **`FrontendRust/docs/arcana/WhenValuesShouldBeInterned-WVSBIZ.md`** — heuristics + Scala-parity rule for when a type is Interned vs Value-type
6. **`FrontendRust/zen/migration_principles.md`** — migration rules (DCCR, RCSBASC, architect-level escape hatch)
7. **`FrontendRust/zen/testing.md`** — test conventions (`find_func_named`, `expect_1`/`expect_2`, etc.)
8. **`docs/skills/migration-drive.md`** — the junior's instructions (know what they're following)
9. FrontendRust/docs/arcana/IdenticalInputsIdenticalOutputs-IIIOZ.md
10. Luz/shields/ScalaParityDuringMigration-SPDMX.md

---

## Guiding Principle

**1:1 Scala parity is the highest priority.** Every design decision defers to matching Scala's structure, naming, and behavior. No novel logic, no reorganization, no Rust-idiomatic "improvements" beyond what Rust strictly requires to compile. The body-translation rule (translate every line literally, no simplifications) is in `migration_principles.md` DCCR + `migration-drive.md`.

**Second priority: every Rust function must be immediately followed on the next line by a `/* ... */` comment containing its Scala equivalent.** Use the "Slicing In New Definitions" pattern below; pre-existing fns lacking an adjacent block are tech debt, not precedent.

**TL/reviewer addition:** when handing off, quote the Scala verbatim and tell JR to mirror it; when reviewing, flag any place the Rust shape diverges from the Scala shape, even when the divergence "obviously works." A clever translation nobody can verify against the Scala is worse than a verbose one anyone can.

**Known principled divergence: covariant generic parameters.** Scala uses covariant generic parameters (e.g. `PrototypeT[+T <: IFunctionNameT]`, `IdT[+T <: INameT]`) to thread compile-time narrowing through containers; Rust does not try to mimic this. Containers carry the wide enum (`IdT<'s, 't>` with `local_name: INameT`), and use sites narrow at runtime via `TryFrom` (`IFunctionNameT::try_from(name).unwrap()`) or the corresponding `expect_*` accessor. The `.unwrap()` / `expect()` is the documented runtime stand-in for Scala's compile-time `T <: …` bound.

**Guardian isn't perfect — bad edits slip through.** Some Scala-divergences land without firing any shield (e.g. `resolve_struct_layer` inlined Scala's `solveForResolving` helper as three separate calls; SPDMX caught it the second time when JR mirrored the same shape into `resolve_interface_layer`). Stay vigilant on review: every diff gets eyes against Scala, even when Guardian is silent. When you spot the divergence, course-correct JR (and the prior precedent if needed) — don't let the broken pattern propagate.

**Don't trust JR when they argue to bypass Guardian.** "The same pattern was already approved in X" is often JR pointing at a slipped-through bug as evidence the new bug is fine. Use your own judgment against the Scala source, not JR's appeal-to-precedent. If both sites are wrong, refactor both — don't temp-disable.

---

## Where We Are

The scaffolding phase is complete. Slabs 0–14b built out every type definition, every method signature with proper lifetimes, and all placeholder types (only `IRegionNameT` retains `_Phantom`). `cargo check --lib` is clean.

**Current work: body migration (Slab 15+).** Work is test-driven: pick a test, run it, implement the body it hits, repeat.

Test infrastructure: 14 test files in `src/typing/test/` with 173 test bodies (compiler_tests 91, compiler_solver_tests 27, compiler_virtual_tests 18, compiler_mutate_tests 12, compiler_lambda_tests 10, compiler_ownership_tests 11, compiler_project_tests 3, compiler_generics_tests 1). All currently panic at the first method body they exercise.

**Latest passing test: `tests_panic_return_type` (Slab 15i).** The next ignored test in `compiler_tests.rs` order is the current driving target. Pre-15i passing tests (`simple_program_returning_an_int_explicit`, `hardcoding_negative_numbers`, `taking_an_argument_and_returning_it`, etc.) remain un-ignored as regression guards.

---

## Recurring bug classes

**Hand-rolled enumerations of "is-a-Trait" sets.** Scala's `case x : ITrait => …` matches every subtype automatically; Rust ports that hand-enumerate the variants drift the moment one is added — always use `TryFrom<WideEnum> for NarrowEnum::is_ok()` instead (the scaffolding-generated TryFrom impls cover every variant correctly).

**Hand-rolled `ptr::eq(self, other)` on a Polyvalue's outer `&self`.** Works while the enum is always held behind `&'t Outer` (the outer address coincides with the arena address); silently breaks the moment it's flipped to by-value — `self` becomes a stack address, two by-value copies of the same logical wrapper compare unequal, and any `HashMap`/`HashSet` keyed on them silently corrupts. Caught once in `environment.rs:60-67` after the `IEnvironmentT` by-value flip. Rule: Polyvalue enums must `#[derive(PartialEq, Eq, Hash)]` — see @PVECFPZ.

**`'t: 'ctx` / `'s: 'ctx` are already implied by the `Compiler` struct** (via well-formedness on its `&'ctx X<'s, 't>` fields — the struct couldn't exist otherwise), so it's fine to restate them explicitly on a local `impl` `where` clause when rustc fails to propagate the implied bound through HRTB/invariance (e.g. `Box<dyn FnOnce(&Compiler<'s, 'ctx, 't>, ...) + 'ctx>` in pattern_compiler.rs's CPS chain). Never declare the reverse `'ctx: 't` — that's the bound rustc *suggests* but it's architecturally backwards (Compiler is stack/`'ctx` data and dies before `'t`).

**Parallel Builder/Frozen APIs diverging asymmetrically from Scala.** When one Scala API (e.g. `TemplatasStore.addEntries`) is split into a Rust Builder + Frozen pair (`TemplatasStoreBuilder::add_entries` at `environment.rs:851-862` vs `TemplatasStoreT::add_entries` at `environment.rs:942-979`), both must mirror Scala's full logic including special-case branches — review them side-by-side against the single Scala source.

**Two-channel errors collapsed into one.** When a Scala fn both `throw`s `CompileErrorExceptionT` *and* returns `Result[_, SomeLocalError]`, the Rust mirror is nested `Result<Result<_, SomeLocalError>, ICompileErrorT>` — outer is the exception channel (every caller `?`-propagates), inner is the business channel (callers inspect and react). Merging them into a single Result loses the "always propagate" vs "caller decides" distinction.

---

## Known Residual Items

- **dispatch_function_body_macro** and friends not wired.
- **LocationInFunctionEnvironmentT.path: Vec<i32>** in `ast/ast.rs` violates AASSNCMCX. Future cleanup turns into `&'t [i32]`.
- **IRegionNameT** retains `_Phantom` — 0 Scala implementors found, deferred.
- **`lookup_function_by_human_name` should be `lookup_function_by_str`** per SPDMX exception J's pre-approved rename table (`Frontend/TypingPass/.../HinputsT.scala:146` → Rust). Cosmetic; rename the def in `hinputs_t.rs:326` and call sites in `compiler_tests.rs`. No body changes.
- **`add_function` lacks the SPDMX-B adaptation comment** — should carry a `// Rust adaptation (SPDMX-B): signature passed explicitly because CompilerOutputs doesn't hold the typing_interner.` block above the fn. Cosmetic; documents the divergence so reviewers don't flag it.
- **`ITemplataT::tyype()` getter is unimplemented** — Scala has it on the trait; Rust needs a per-variant match returning `ITemplataType<'s>`. Surfaces in `LetExprRuneTypeSolverEnv::lookup`'s `Some(_x) => panic!()` arm. Add when a test path hits the panic.
- **3 typing-pass `IRuneTypeSolverEnv` sites un-migrated**: `array_compiler.rs:101`, `templata_compiler.rs:1501` (the `createRuneTypeSolverEnv` factory), `overload_resolver.rs:455`. Each becomes a per-site named struct following the `LetExprRuneTypeSolverEnv` pattern when its containing function gets migrated. Don't unify — Scala bodies differ.
- **`lookup_nearest_with_imprecise_name`** at `function_environment_t.rs:1079` is panic-stubbed. Will need migration when a test path actually triggers a name lookup through the LetSE arm (none of the currently-passing tests do).
- **~32 slice-pipeline orphan free fns at module scope across `src/typing/`** waiting to be wrapped in `impl SomeT` blocks. Each surfaces as a JR escalation when its first call site materializes. Batch-sweep plan at `/Users/verdagon/.claude/plans/lets-do-proactive-please-proud-feather.md` — ~1.5 hours, structural-only (bodies stay `panic!`).
- **Revisit `/// Arena-allocated` vs `/// Temporary state` classifications.** Scaffolding may have over-eagerly marked types Arena-allocated when they're really transient solver outputs or function-return wrappers (`LocationInFunctionEnvironmentT`, `CompleteResolveSolve`/`CompleteDefineSolve`, `HinputsT`). Walk every `/// Arena-allocated` type and re-classify ones that don't need arena identity. Architect-level decision per type.
- **Wrapper enums (IEnvironmentT family) reclassified as Polyvalue** per @TFITCX; the closed-set-fat-pointer mental model + by-value eq/hash trap documented at @PVECFPZ. `IEnvironmentT`/`IInDenizenEnvironmentT` are now held by value at field/parameter sites with derived eq/hash.
- **SPDMX recurring class: structural-shape diff that's a Rust→Scala bug-fix.** Seen on `compiler_tests.rs:1102-1114` (the "Automatically drops struct" test, where the Rust pattern was matching `template_args` against the OwnT/StructTT coord but Scala's third `FunctionNameT` field is `parameters`); SPDMX flagged the corrective re-shape. If it fires again, worth amending into SPDMX rather than temp-disabling each time.
- **`RUST_MIN_STACK=16777216` set globally in `/Volumes/V/Sylvan/.cargo/config.toml`** — debug-mode workaround for postparser `scout_expression`'s giant per-arm-locals frame bloat (~21KB/frame; 96 frames of normal recursion exhausts the default 2MB stack on `typing_pass_on_roguelike`). Release builds pass unmodified. **Post-migration TODO: revert the config change** once `scout_expression` is split per-arm into helper fns (or its `(StackFrame, &IExpressionSE, VariableUses, VariableUses)`-tuple locals are Boxed) so default stack suffices in debug too.
- **Post-migration sweep: scrub `if`-guards inside test `matches!`/match patterns** — Scala-parity tests destructure literals all the way down, so any `if foo.bar == "..."` guard means JR shallowed out a pattern that should have been deepened.
- **Eliminate sources of nondeterminism.** Ptr-hashing on `@IEOIBZ` identity types, `IdT`, and `PtrKey<T>` is nondeterministic across runs and leaks into output via `HashMap`/`HashSet` iteration. `ArenaIndexMap` (insertion-ordered) is unaffected — the AASSNCMCX sweep accidentally fixed most cases. Remaining at-risk: `HashMap<PtrKey<IdT>, V>` in `CompilerOutputs` and `HashMap<IdT, V>` in `HinputsT` (both Temporary state). **Short-term:** extend ArenaIndexMap to ptr-hashed-key maps regardless of containing-struct class. **Long-term:** content-based hashing on `@IEOIBZ` types + `IdT` (touches the @IEOIBZ pattern). Bit-reproducible output requires both. Defer until the instantiator gets attention or a test flakes.

---

## Good Partial Implementing

When replacing a `panic!` stub with real logic, write just the shallow structure of that scope — straight-line variable bindings, function calls, match expressions with all arms — but put `panic!` inside every new branch body, loop body, closure/lambda body, and match arm. Then only fill in the specific arms/branches the driving test actually hits. This applies recursively: when a test hits one of those inner panics, replace *that* panic with its own skeleton-with-panics, fill in only what the test needs, and so on. Each iteration expands one panic into a new layer of structure. **Aggressively panic for anything that might not be executed by current tests.** This minimizes each batch's diff and ensures untested paths crash loudly rather than silently returning wrong results.

**"Untested branches" means untested *code paths*, not untested *data values*.** A struct field initialized to `HashMap::new()` runs on the test path — if Scala produces an empty map on the same input, the empty Rust map is correct parity. Panic only inside branches that wouldn't be reached if the input is empty: loop bodies iterating empty collections, match arms for variants the input doesn't contain, closures that are never invoked. When in doubt: does this line *run* on the test path? Yes → produce Scala's value (empty is fine). No → panic.

**SPDMX caveat.** SPDMX sees `.map(|x| panic!())` / `.for_each(|x| panic!())` / nested `for x in ... { panic!() }` and flags it as "novel scaffolding," recommending whole-function `panic!()` instead. Whole-function panic breaks the test path, so the skeleton-with-panics IS the right pattern. **Resolution: TL temp-disables SPDMX on the affected function** (juniors must escalate, not temp-disable themselves). Standard rationale boilerplate:

> Per TL.md "Good Partial Implementing": this function uses the skeleton-with-panics-in-closures pattern that the migration design endorses. The iteration structure (.map / .for_each / nested for) mirrors Scala's call graph; panics live in the closure bodies. SPDMX's heuristic flags the iteration structure as "novel scaffolding," but the structure IS the Scala parity — without it, the call graph diverges. For the empty-input case (the driving test), the closures never fire and the function is a verified no-op; for non-empty inputs they panic loudly with named placeholders. TL approval: temp-disable SPDMX here, re-enable when the closure bodies get filled in with real logic.

Expect this on most emitter-shaped typing-pass functions (long `.map.groupBy.mapValues` / `.foreach` chains). Re-enable SPDMX when the closure bodies get filled with real logic.

---

## Architect Approval Required

**Run structural solutions by the architect first.** Before implementing any structural fix or design change, propose the solution and wait for approval. This also applies to anything you write into `for-jr.md` — get architect approval before handing instructions to JR, even when the answer looks obvious. This includes lifetime-parameter additions, signature changes that propagate across many files, new abstractions, and any choice between alternatives. Don't start editing — even for fixes that look mechanical — until the architect has signed off. The cost of a quick check is small; the cost of unwinding a wrong-direction change across ~20 call sites is large.

**Never add `// AFTERM:` or `// TODO:` comments** — yours or JR's. These markers are the architect's tool for tracking deferred cleanup; they get added only when the architect explicitly asks. This applies even when the deferral seems obviously correct (a needless snapshot, a misplaced helper, a panic stub that clearly needs real logic). Raise the deferral to the architect; don't park it inline. If JR proposes an AFTERM/TODO, tell them to drop it and escalate.

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

**JR can issue Guardian temp-disables themselves** via `mcp__guardian__guardian_temp_disable`; they only escalate to TL when the temp-disable itself fails or is rejected.

**Threading an `interner: &TypingInterner<'s, 't>` parameter is always a fine Rust adaptation** — Scala didn't take an interner because it used GC, but Rust often needs one to arena-allocate a result Scala mutated in place. Document with `// Rust adaptation (SPDMX-B): <why>` above the fn. JR-level work; doesn't trip Guardian.

**When JR escalates something Guardian wouldn't have blocked (e.g. a lifetime error with a documented arena-alloc shape), hand the fix back as instructions for JR to apply — don't land it yourself.**

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

## Proactively Add Inherited Dispatch Methods

The slice pipeline only stubs methods defined directly on a Scala trait's body. Scala trait-extends-trait inheritance, abstract factory methods, and dispatch-tag enums all need explicit Rust delegation the pipeline doesn't generate. When you see a Scala child trait extending a parent (or a sealed trait with named implementors per SSTREX), proactively add all inherited dispatch methods on the child enum — don't wait for serial JR escalations. See "Slicing In New Definitions" below for the slice-in mechanics; annotate the new methods with `/* Guardian: disable-all */`.

---

## Slicing In New Definitions

When JR is blocked by NNDX on a legitimate Scala counterpart, the issue is incomplete scaffolding, not a bad shield — TL adds the missing definition directly (don't temp-disable NNDX). The junior escalating is the system working correctly.

**Pre-flight.** Grep the target type for existing `pub fn`s — JR sometimes proposes a Rust-idiomatic name (`step`) when the Scala-parity port already exists under the operator translation (`add` for `def +`). Recent miss: JR's `life.step(1)` escalation when `add(interner, n)` already lived at `ast/ast.rs:387`.

**Escalation hygiene.** JR should cite the Scala source (`Frontend/.../X.scala:NNN`), not the Rust audit-trail line.

**How to slice in.** Find the Scala `def` inside its `/* ... */` block; split the block — close `*/` just before the target `def`, insert the Rust `impl`, reopen `/*` for the remaining Scala. The Scala `def` line must end up **inside** the Rust `impl` block, as an inline `/* ... */` immediately after the Rust `fn` body — not after the `impl`'s closing brace.

```rust
+*/
+impl<'s, 't> IEnvironmentT<'s, 't> where 's: 't {
+  pub fn global_env(&self) -> &'t GlobalEnvironmentT<'s, 't> {
+    match self {
+      IEnvironmentT::Package(e) => e.global_env,
+      // ...
+    }
+  }
+  /*
    def globalEnv: GlobalEnvironment
+  */
+}
+/*
```

Note how the existing scala code is unchanged, and we are making an addition *around* it.

### Sub-case: name-collision disambiguation

Rust flattens multiple Scala compiler classes (`Compiler`, `ImplCompiler`, `TemplataCompiler`, ...) onto one `Compiler` struct. Same-named Scala methods (e.g. `Compiler`'s anonymous `IInfererDelegate.isDescendant(kind: KindT)` and `ImplCompiler.isDescendant(kind: ISubKindTT)`) collide at compile time. Append a `_<distinguishing-arg-type>` suffix to the *newer / less-established* slice — leave existing call sites intact. Add a comment above the new fn:

```rust
// Rust adaptation: collides with Compiler::is_descendant lifted from
// ImplCompiler.scala (which Rust flattened onto Compiler); appended `_kind`
// suffix. Scala uses class-level disambiguation that Rust lacks.
```

NNDX fires on the rename, so this is TL territory. Don't reach for materializing an `IInfererDelegate` struct unless the architect signs off — the flatten-onto-Compiler convention is precedent since `sanity_check_conclusion`.

### Sub-case: no Scala counterpart at all

For Rust definitions with no Scala counterpart (delegation wiring for Scala trait inheritance, `From`/`TryFrom` impls, interning Val/Query structs), annotate so Guardian doesn't re-fire:

- **Pure wiring (no logic)** — delegation methods, `From`/`TryFrom` match-dispatches, trivial forwarding accessors. Add `/* Guardian: disable-all */` after the fn/impl block.
- **Contains logic** — conditionals, assertions, non-trivial transformations. Add an empty `/* */` after the body (satisfies SCPX, signals "reviewed and has no Scala counterpart").

JR never adds these annotations — they escalate to TL.

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

## Writing Scala-Parity Tests

Scala tests built on `Collector.only(scope, { case … })` should port to the existing `collect_only_*node!` traverse macros (`postparsing/test/traverse.rs`, `typing/test/traverse.rs`) — one macro call per Scala `Collector.only` call, the whole `case` pattern inlined verbatim — not to positional `expect_N` + `cast!` + `assert_eq!` chains, which over-constrain on element count and are invisible to a tree walker. Use literal patterns where Scala does: `StrI<'s>` is `pub struct StrI<'s>(pub &'s str)`, so `StrI("x")` matches inline exactly like Scala's `StrI("x")` — no `if name.as_str() == "x"` guard needed. Trailing `VoidSE` from semicolon-terminated blocks is invisible to recursive collectors, so don't add Rust-only stripping in the scout to "fix" a port — rewrite the over-constrained test instead.

## Test Promotion Workflow

All 173 tests in `src/typing/test/` have `#[ignore]` except the currently-active test. This means `cargo nextest run` only runs the active test(s), so JR sees regressions immediately — a failure means something they changed broke, not a pre-existing panic.

**When JR reports a test passing:**
1. Verify the pass yourself: `cargo nextest run --manifest-path ./FrontendRust/Cargo.toml > ./tmp/slab15-tests.txt 2>&1`
2. Remove `#[ignore]` from the next simplest test in `compiler_tests.rs`
3. Hand off to JR again

**Choosing the next test:** Start with `compiler_tests.rs` (91 tests, simplest programs). Within that file, go roughly top-to-bottom — the tests are ordered from simple to complex. Once `compiler_tests.rs` is done, move to `compiler_solver_tests.rs`, then `compiler_virtual_tests.rs`, etc.

**Keep passed tests un-ignored.** They serve as regression guards. If a passed test starts failing, that's a real regression JR needs to fix before continuing.

---

## Commit Message Format

When the architect says "fire commit," structure the message as:

1. **First line: 1–3-sentence TL;DR** of what this commit does. The whole summary fits on the first line (no hard wrap into the body); think headline, not subject. Tools that show only the first line should give the reader the gist.
2. **Body: what the commit contains** — paragraph(s) describing the changes in enough detail that a reviewer can verify scope without reading the diff. Group by file or by concern, whichever reads better.
3. **Trailing list: notable situations / new arcana / complicated comments.** Bullet list at the end calling out anything unusual: scaffolding fixes that needed architect approval, new arcana documents created, Guardian temp-disables added, comments inserted that explain non-obvious invariants, Scala source edits, etc. Empty list is fine — omit the section if nothing notable happened.

Use a HEREDOC to preserve formatting (per the standard commit protocol). Don't add a Co-Authored-By trailer unless the architect explicitly asks for one.

---

## JR Escalations Land In `for-tl.md`

JR writes every escalation (scaffolding gap, NNDX block, SPDMX skeleton, lifetime puzzle, alternatives needing a call) into `for-tl.md` at the repo root. **Check `for-tl.md` at the start of every turn** and whenever the architect hands work back without a specific escalation in chat — it's the source of truth for what JR is blocked on. Chat mentions are courtesy; the file is canonical. After resolving an escalation, strike the section (or remove it) so the file shows only open items.

When the architect says just "z", that's the signal to check `for-tl.md` for something new that you must address. After resolving each escalation, propose to the architect one preventive tweak (e.g. a sentence to add to `docs/skills/migration-drive.md`) so the same class of escalation doesn't recur. Always Read the actual Rust/Scala files JR cites — line numbers and snippets in `for-tl.md` are JR's framing, and the real code often contradicts or complicates their summary. Also grep TL.md and `docs/architecture/typing-pass-design-v3.md` for keywords from JR's question — the answer (or a directly relevant precedent) is often already written down. When you finish handling an escalation, write the response/instructions for JR into `for-jr.md` at the repo root so JR can pick it up between turns.

---

## How To Continue

1. **Read the design doc** (`docs/architecture/typing-pass-design-v3.md`) top to bottom — Part 1 covers the arena shape.
2. **Body migration is test-driven.** The active test (most recently `tests_panic_return_type` as of Slab 15i; the next `#[ignore]`'d test in `compiler_tests.rs` order becomes the driving target) drives which panic stubs get implemented. See "Test Promotion Workflow" above.
3. **Only commit when the architect says "fire commit".** Hand back uncommitted working trees at batch boundaries by default. The architect handles tags. When (and only when) the architect says the literal phrase "fire commit," run `git commit` with a message in the format below.
4. **Group related bodies** into coherent batches for review. Each batch gets a handoff doc listing target methods, driving test(s), and Scala translation gotchas. Spawn a junior per batch. Expect and invite push-back — handoffs are proposals, not spec.
5. **When a design diverges from the design doc, record the divergence in `FrontendRust/docs/reasoning/<topic>.md`, then update the design doc.**
6. **Scope discipline.** If edits land in `TL.md`, the design doc, or reasoning docs, announce in the hand-back summary, not folded silently into the diff. Revert off-scope edits before review.

---

Questions? The reasoning docs + the per-slab handoffs have additional context. When in doubt, prefer Scala parity over Rust-idiomatic optimization — that's the guiding principle.
