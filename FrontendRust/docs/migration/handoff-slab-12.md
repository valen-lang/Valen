# Handoff: Typing Pass Slab 12 — Solver + Resolver Method Signature Rewrite

## Who this is for

You're picking up an in-progress Rust port of a Scala compiler frontend. A senior engineer has been moving through the typing pass one "slab" at a time per `quest.md` §12. Slabs 0-11 are done:

- **Slabs 0-7** — arena substrate, names, types, templatas, envs, AST, `CompilerOutputs` data shape, `HinputsT`/`Compiler` scaffolding.
- **Slab 8** — `CompilerOutputs` method signatures (54 sigs).
- **Slab 9** — `object TemplataCompiler` method signatures (35 sigs).
- **Slab 10** — `compiler.rs` residuals + `local_helper.rs`/`struct_compiler.rs` object orphans + `class TemplataCompiler` tail (~31 sigs).
- **Slab 11** — expression-layer signatures (`expression_compiler.rs`, `pattern_compiler.rs`, `block_compiler.rs`, `call_compiler.rs`, 39 sigs).

Slab 12 is the fifth signature-rewrite slab, targeting the **solver + resolver layer**:

- **`src/typing/infer_compiler.rs`** — 15 stubs (the inference driver; `solveFor*`, `makeSolverState`, conclusion checkers, `resolve_*` helpers)
- **`src/typing/overload_resolver.rs`** — 11 stubs (function overload resolution; `findFunction`, `paramsMatch`, candidate banner logic)
- **`src/typing/citizen/impl_compiler.rs`** — 9 stubs (`resolveImpl`, `compileImpl`, `isParent`, `isDescendant`, etc.)
- **`src/typing/reachability.rs`** — 8 fns to lift and fix (not `(&self)` stubs; see Gotcha 8)
- **`src/typing/infer/compiler_solver.rs`** — **`IInfererDelegate` trait cleanup** (delete the vestigial marker trait + drop `&dyn IInfererDelegate<'s, 't>` params across ~15 fn signatures already present)

Total: **~35 bare `(&self)` stubs + the reachability lifts + the `IInfererDelegate` cleanup**. Budget ~4 hours focused. This is the most cross-file of the signature-rewrite slabs because `IInfererDelegate` touches both `compiler_solver.rs` and `infer_compiler.rs`.

**Read these first in this order**, then come back:

1. `FrontendRust/docs/migration/handoff-slab-11.md` — the prior signature-rewrite slab. The `NodeEnvironmentBox` → `&mut NodeEnvironmentBuilder<'s, 't>` convention and closure-param handling (`impl FnOnce(...)`) apply directly.
2. `FrontendRust/docs/migration/handoff-slab-9.md` — canonical translation-rules reference.
3. `TL-HANDOFF.md` at repo root — file-layout standards + slab roadmap + the "IInfererDelegate trait stays vestigial" known-residual item (which Slab 12 resolves).
4. `FrontendRust/src/typing/infer/compiler_solver.rs` lines 150-200 — skim the existing `IInfererDelegate` marker trait and spot-check a few of its consumer signatures. Slab 12 deletes the trait; no replacement.
5. This doc.

You shouldn't need to read the Scala source externally — every Scala `def` sig is already embedded inline in the `/* def ... */` blocks beneath each Rust stub.

---

## The big picture: why Slab 12 exists

The solver + resolver layer is the type-inference core of the typing pass. Scala organizes it into three sub-compilers (`InferCompiler`, `OverloadResolver`, `ImplCompiler`) plus a `Reachability` utility object. In the god-struct refactor, all four's methods were lifted onto `Compiler<'s, 'ctx, 't>` as bare `pub fn foo(&self) { panic!(...) }` stubs. Your job is to fill in the parameter lists from the Scala `/* def ... */` anchors.

Two novelties distinguish Slab 12 from prior slabs:

1. **`IInfererDelegate` vestige cleanup.** Scala's `IInfererDelegate` trait carried ~12 methods used by `compiler_solver.rs` to call back into the compiler (it was a dependency-injection escape hatch to avoid a circular dependency between `InferCompiler` and the "delegate" things it needed). In Rust post-god-struct, those callbacks become plain `self.foo(...)` method calls. The trait is vestigial — an empty `pub trait IInfererDelegate<'s, 't> {}` marker — and its consumer signatures still thread `&dyn IInfererDelegate<'s, 't>` params. **Slab 12 deletes both**: the trait AND the param (just drop `delegate: &dyn IInfererDelegate<'s, 't>` from every fn that takes it in `compiler_solver.rs`, and delete the `pub trait IInfererDelegate<'s, 't> {}` declaration at line 156).
2. **`InferEnv` is a partial placeholder.** Scala's `case class InferEnv(originalCallingEnv, parentRanges, callLocation, selfEnv, region)` is referenced all over Slab-12 Scala sigs. Rust has `pub struct InferEnv<'s>(pub std::marker::PhantomData<&'s ()>)` at `infer_compiler.rs:86` — a single-lifetime phantom placeholder. **Use it as-is** in signatures. Slab 14+ fills in the real fields when bodies land. If you need the type to carry `'t` later, flag it — but for signature work, the phantom single-lifetime form is fine.

By the end of Slab 12:

- `cargo check --lib` passes with 0 errors, 0 warnings.
- All 35 bare `(&self)` stubs across the 3 primary files have real signatures.
- `reachability.rs`'s 8 free fns are lifted onto `impl Compiler` per the Slab 9/10 object-to-Compiler convention (or the missing lifetime params are added; see Gotcha 8).
- `IInfererDelegate` trait deleted; `&dyn IInfererDelegate<'s, 't>` params removed from every `compiler_solver.rs` fn signature.
- Bodies remain `panic!("Unimplemented: Slab 14 — body migration");`.
- Scala `/* */` blocks unchanged byte-for-byte.
- Only the 5 target files modified.

**What Slab 12 is NOT:**

- No body migration. Bodies stay `panic!()`. Slab 14+.
- No filling `InferEnv` fields beyond the `_Phantom` placeholder.
- No filling `ITypingPassSolverError` / `IDefiningError` / `FindFunctionFailure` / `IsParentResult` variants — they stay `_Phantom`-only / empty enums. Body migration fills them.
- No `struct_compiler_core.rs` / `function_compiler.rs` / `function_body_compiler.rs` / `destructor_compiler.rs` / `anonymous_interface_macro.rs` — Slab 13.
- No touching of `SimpleSolverState<Rule, Rune, Conclusion>` in `src/solver/simple_solver_state.rs`. Use as-is in signatures.

---

## Stub inventory: the 35 targets + reachability lifts + delegate cleanup

### File 1: `src/typing/infer_compiler.rs` — 15 stubs

| Line | Stub |
|---|---|
| 196 | `solve_for_defining` |
| 235 | `solve_for_resolving` |
| 263 | `partial_solve` |
| 289 | `make_solver_state` |
| 337 | `r#continue` (raw identifier; Scala `continue` is a Rust reserved keyword) |
| 353 | `check_resolving_conclusions_and_resolve` |
| 484 | `interpret_results` |
| 509 | `check_defining_conclusions_and_resolve` |
| 592 | `import_reachable_bounds` |
| 616 | `import_conclusions_and_reachable_bounds` |
| 646 | `resolve_conclusions_for_define` |
| 720 | `resolve_function_call_conclusion` |
| 766 | `resolve_impl_conclusion` |
| 810 | `resolve_template_call_conclusion` |
| 881 | `incrementally_solve` |

### File 2: `src/typing/overload_resolver.rs` — 11 stubs

| Line | Stub |
|---|---|
| 149 | `find_function` |
| 194 | `params_match` |
| 242 | `get_candidate_banners` |
| 267 | `get_candidate_banners_inner` |
| 322 | `attempt_candidate_banner` |
| 529 | `get_param_environments` |
| 550 | `find_potential_function` |
| 604 | `get_banner_param_scores` |
| 647 | `narrow_down_callable_overloads` |
| 859 | `get_array_generator_prototype` |
| 888 | `get_array_consumer_prototype` |

### File 3: `src/typing/citizen/impl_compiler.rs` — 9 stubs

| Line | Stub |
|---|---|
| 87 | `resolve_impl` |
| 160 | `partial_resolve_impl` |
| 221 | `compile_impl` |
| 366 | `calculate_runes_independence` |
| 416 | `assemble_impl_name` |
| 590 | `is_descendant` |
| 633 | `get_impl_parent_given_sub_citizen` |
| 668 | `get_parents` |
| 736 | `is_parent` |

### File 4: `src/typing/reachability.rs` — 8 fn lifts + lifetime-param fixes

Unlike the prior 3 files, reachability.rs does **not** use `(&self)` bare stubs. Instead:

- `Reachables::size()` (line 42) — method on `Reachables` struct; needs body only (stays `panic!()` — body migration).
- `find_reachables` (line 52) — free fn at module scope; lift onto `impl Compiler` per Slab 9 convention.
- `visit_function` (line 85) — free fn; lift onto `impl Compiler`.
- `visit_struct` (line 121) — free fn; lift + add `<'s, 't>` lifetimes to its params (currently missing; see its Scala `//` comment).
- `visit_interface` (line 166) — same.
- `visit_impl` (line 211) — same.
- `visit_static_sized_array` (line 237) — same.
- `visit_runtime_sized_array` (line 272) — same.

**Note on Scala comment style**: reachability.rs uses `//` line-prefix comments inside `/* */` blocks (the whole file's Scala source was wrapped in a big commented-out block, then each fn was commented with `//` within). Hook enforcement still applies to the outer `/* */`.

### File 5: `src/typing/infer/compiler_solver.rs` — `IInfererDelegate` vestige cleanup

Not a signature-lift file — pure deletion:

- **Delete** `pub trait IInfererDelegate<'s, 't> {}` at line 156 (plus its "// vestigial: kept until Step 8 cleanup" comment at line 155).
- **Delete** every `delegate: &dyn IInfererDelegate<'s, 't>,` param from fn signatures in this file. Grep locations (approximately): lines 445, 544, 559, 579, 687, 759, 791, 817, 1247. Check each carefully — don't accidentally delete a `delegate:` param from a non-vestigial trait (there shouldn't be any other delegates in this file, but verify).
- **Don't delete** the Scala `/* ... delegate: IInfererDelegate ... */` block content — those are frozen.

Nothing in `compiler_solver.rs` has a body that references `delegate.foo(...)` — the bodies are all `panic!()`. So dropping the param is safe for compilation.

---

## Signature translation rules

**Same table as Slabs 8-11.** Reread Slab 9 §"Signature translation rules" if rusty. Quick reference of the types you'll see most in Slab 12:

| Scala | Rust |
|---|---|
| `CompilerOutputs` (param, likely mutates) | `&mut CompilerOutputs<'s, 't>` — conservative default |
| `IInDenizenEnvironmentT` | `&'t IInDenizenEnvironmentT<'s, 't>` (Slab-4 override) |
| `InferEnv` (Scala case class) | `InferEnv<'s>` by value — **it's currently `pub struct InferEnv<'s>(pub std::marker::PhantomData<&'s ()>)` — treat as Copy** (the `PhantomData<&'s ()>` is Copy). See Gotcha 2. |
| `SimpleSolverState[IRulexSR, IRuneS, ITemplataT[ITemplataType]]` | `&mut SimpleSolverState<'s, 't, IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>>` — **check first** the generic positions match (see Gotcha 3). Default: `&mut SimpleSolverState<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>>` since `src/solver/simple_solver_state.rs:11` has `SimpleSolverState<Rule, Rune, Conclusion>` generic over three params only, no lifetimes. |
| `List[RangeS]` / `Vector[RangeS]` | `&[RangeS<'s>]` |
| `IdT[X]` (any phantom) | `IdT<'s, 't>` by value |
| `IRulexSR` / `Vector[IRulexSR]` | `&'s IRulexSR<'s>` / `&[&'s IRulexSR<'s>]` |
| `IRuneS` / `Vector[IRuneS]` | `IRuneS<'s>` by value / `&[IRuneS<'s>]` |
| `Map[IRuneS, ITemplataType]` | `&HashMap<IRuneS<'s>, ITemplataType<'s>>` |
| `Map[IRuneS, ITemplataT[ITemplataType]]` | `&HashMap<IRuneS<'s>, ITemplataT<'s, 't>>` |
| `CoordT` / `Vector[CoordT]` | `CoordT<'s, 't>` by value / `&[CoordT<'s, 't>]` |
| `KindT` / `ICitizenTT` / `ISubKindTT` / `ISuperKindTT` | by value (Copy per Slab 3) |
| `StructTT` / `InterfaceTT` | by value (Copy per Slab 3) |
| `ITemplataT[X]` | `ITemplataT<'s, 't>` by value |
| `StructDefinitionTemplataT` / `InterfaceDefinitionTemplataT` / `ImplDefinitionTemplataT` / `CitizenDefinitionTemplataT` | `&'t X<'s, 't>` (heavy templatas) |
| `InitialKnown` / `InitialSend` | `InitialKnown` / `InitialSend` by value — both are currently empty stubs (`pub struct InitialKnown;` / `pub struct InitialSend;` at `infer_compiler.rs:106`/`:114`); Slab 14 fills fields |
| `Vector[InitialKnown]` / `Vector[InitialSend]` | `&[InitialKnown]` / `&[InitialSend]` |
| `CompleteDefineSolve` / `CompleteResolveSolve` | return by value (both are empty `pub struct` stubs in `infer_compiler.rs`; Slab 14 fills fields) |
| `IDefiningError` / `IResolvingError` | `IDefiningError` / `IResolvingError<'s, 't>` — **check lifetime parameterization** per current definitions; `IDefiningError` at line 75 is `pub enum IDefiningError {}` (no lifetimes), `IResolvingError` at line 62 has `<'s, 't>` |
| `Result[CompleteDefineSolve, IDefiningError]` | `Result<CompleteDefineSolve, IDefiningError>` |
| `Result[CompleteResolveSolve, IResolvingError]` | `Result<CompleteResolveSolve, IResolvingError<'s, 't>>` |
| `Result[StampFunctionSuccess, FindFunctionFailure]` | `Result<StampFunctionSuccess<'s, 't>, FindFunctionFailure<'s, 't>>` — grep both types first; `FindFunctionFailure` is at `overload_resolver.rs:106` |
| `FailedSolve[...]` return | Check current Rust type; likely `FailedSolve<'s, 't, IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>` — **grep `pub struct FailedSolve`** in `src/solver/` to verify |
| `ISolverError[...]` | Same check — grep before using |
| `IsParentResult` | `IsParentResult` by value — `pub enum IsParentResult { _Phantom }` at `impl_compiler.rs:46` (empty-enum placeholder; variants `IsParent` / `IsntParent` are separate structs not yet folded in — don't fold in during Slab 12) |
| `IInfererDelegate` (param) | **Delete the param entirely** in `compiler_solver.rs`. For `infer_compiler.rs` stubs that mention `infererDelegate.foo(...)` in the Scala body, the Scala body is commented — the Rust body stays `panic!()`. No `delegate` param on the Rust signature. |
| `RegionT` | `RegionT` by value |
| `PrototypeT` / `SignatureT` | `&'t PrototypeT<'s, 't>` / `&'t SignatureT<'s, 't>` |
| `FunctionA` / `StructA` / `InterfaceA` / `ImplA` | `&'s FunctionA<'s>` etc. (scout-side refs) |
| `EdgeT` / `InterfaceEdgeBlueprintT` | `&'t EdgeT<'s, 't>` / `&'t InterfaceEdgeBlueprintT<'s, 't>` |
| `IImpreciseNameS` | `IImpreciseNameS<'s>` by value (Copy per Slab 2) |
| `IFindFunctionFailureReason` | grep first; likely a Rust enum — use as-is |
| `Reachables` | `&mut Reachables<'s, 't>` for mutating passes; `&Reachables<'s, 't>` for read-only |
| `Boolean` | `bool` |
| `Option[X]` | `Option<X>` |
| `Set[X]` (return, owned) | `HashSet<X>` |
| `Profiler.frame(() => { ... })` wrappers in Scala bodies | **ignore** — Rust doesn't use these (per CLAUDE.md notes). Body migration, not signatures. |

### Receiver classification

All 35 method stubs on `&self`. None need `&mut self` — `Compiler<'s, 'ctx, 't>` holds only immutable refs. Mutation threads through `&mut CompilerOutputs`, `&mut SimpleSolverState`, `&mut Reachables`.

### Derive / lifetime bounds

- Every `impl` block: `<'s, 'ctx, 't>` with `where 's: 't`.
- `pub fn` (not `fn`).
- No new derives.

---

## Shape of a lifted stub

### Example 1: `infer_compiler.rs::solve_for_defining`

Before (line 193):

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn solve_for_defining(&self) { panic!("Unimplemented: solve_for_defining"); }
/*
  def solveForDefining(
    envs: InferEnv,
    coutputs: CompilerOutputs,
    rules: Vector[IRulexSR],
    runeToType: Map[IRuneS, ITemplataType],
    invocationRange: List[RangeS],
    callLocation: LocationInDenizen,
    initialKnowns: Vector[InitialKnown],
    initialSends: Vector[InitialSend],
    includeReachableBoundsForRunes: Vector[IRuneS]):
  Result[CompleteDefineSolve, IDefiningError] = { ... }
*/
}
```

After:

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn solve_for_defining(
        &self,
        envs: InferEnv<'s>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        rules: &[&'s IRulexSR<'s>],
        rune_to_type: &HashMap<IRuneS<'s>, ITemplataType<'s>>,
        invocation_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        initial_knowns: &[InitialKnown],
        initial_sends: &[InitialSend],
        include_reachable_bounds_for_runes: &[IRuneS<'s>],
    ) -> Result<CompleteDefineSolve, IDefiningError> {
        panic!("Unimplemented: Slab 14 — body migration");
    }
/*
  def solveForDefining(... same as before ...)
*/
}
```

### Example 2: `overload_resolver.rs::find_function` (large signature, 11 params)

Before (line 146):

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn find_function(&self) { panic!("Unimplemented: find_function"); }
/*
  def findFunction(
    callingEnv: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    functionName: IImpreciseNameS,
    explicitTemplateArgRulesS: Vector[IRulexSR],
    explicitTemplateArgRunesS: Vector[IRuneS],
    contextRegion: RegionT,
    args: Vector[CoordT],
    extraEnvsToLookIn: Vector[IInDenizenEnvironmentT],
    exact: Boolean):
  Result[StampFunctionSuccess, FindFunctionFailure] = { ... }
*/
}
```

After:

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn find_function(
        &self,
        calling_env: &'t IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        function_name: IImpreciseNameS<'s>,
        explicit_template_arg_rules_s: &[&'s IRulexSR<'s>],
        explicit_template_arg_runes_s: &[IRuneS<'s>],
        context_region: RegionT,
        args: &[CoordT<'s, 't>],
        extra_envs_to_look_in: &[&'t IInDenizenEnvironmentT<'s, 't>],
        exact: bool,
    ) -> Result<StampFunctionSuccess<'s, 't>, FindFunctionFailure<'s, 't>> {
        panic!("Unimplemented: Slab 14 — body migration");
    }
/*
  def findFunction(... same as before ...)
*/
}
```

### Example 3: `reachability.rs::visit_struct` (free fn → `impl Compiler` lift)

Before (line 121):

```rust
fn visit_struct(program: &CompilerOutputs, edge_blueprints: &[InterfaceEdgeBlueprintT], edges: &std::collections::HashMap<InterfaceTT, std::collections::HashMap<StructTT, Vec<PrototypeT>>>, reachables: &mut Reachables, struct_tt: StructTT) {
    panic!("Unimplemented: visit_struct");
}
/*
//  def visitStruct(program: CompilerOutputs, edgeBlueprints: Vector[InterfaceEdgeBlueprint], edges: Map[InterfaceTT, Map[StructTT, Vector[PrototypeT]]], reachables: Reachables, structTT: StructTT): Unit = { ... }
*/
```

After:

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn visit_struct(
        &self,
        program: &CompilerOutputs<'s, 't>,
        edge_blueprints: &[&'t InterfaceEdgeBlueprintT<'s, 't>],
        edges: &HashMap<InterfaceTT<'s, 't>, HashMap<StructTT<'s, 't>, Vec<&'t PrototypeT<'s, 't>>>>,
        reachables: &mut Reachables<'s, 't>,
        struct_tt: StructTT<'s, 't>,
    ) {
        panic!("Unimplemented: Slab 14 — body migration");
    }
}
/*
// def visitStruct(... same as before ...)
*/
```

Note: the lift adds `<'s, 'ctx, 't>` lifetimes, drops them as generic on the fn itself, moves parent types to `&'t`, and makes `edges` use `&'t PrototypeT` in its inner value per AASSNCMCX.

### Example 4: `compiler_solver.rs` — delete `&dyn IInfererDelegate` params

Before (one of many):

```rust
pub fn solve<'s, 't>(
    ...
    delegate: &dyn IInfererDelegate<'s, 't>,
    ...
) -> ... { panic!(); }
/*
  def solve(
    ...
    delegate: IInfererDelegate,
    ...
  ): ... = { ... }
*/
```

After:

```rust
pub fn solve<'s, 't>(
    ...
    // delegate param removed — Slab 12 IInfererDelegate cleanup
    ...
) -> ... { panic!(); }
/*
  def solve(
    ...
    delegate: IInfererDelegate,
    ...
  ): ... = { ... }
*/
```

Leave the Scala `/* */` block unchanged. The `// delegate param removed` Rust comment is optional — you can just cleanly delete the param line.

Then at line ~156, delete:

```rust
// vestigial: kept until Step 8 cleanup because fn signatures still reference `IInfererDelegate<'s, 't>` as a param type
pub trait IInfererDelegate<'s, 't> {}
```

Leave the Scala `/* trait IInfererDelegate { ... */` block alone — just delete the `pub trait` stanza.

---

## Gotchas

### Gotcha 1: `IInfererDelegate` is deleted, not replaced

Don't introduce a new trait or enum in its place. Scala `delegate.foo(...)` call sites in bodies become `self.foo(...)` at body-migration time (Slab 14+). In Slab 12 the bodies are all `panic!()`, so you never see a call site — just delete the param.

**If you accidentally see a non-`panic!()` call site that references `delegate`**, stop and flag. That's an out-of-scope body that shouldn't exist yet.

### Gotcha 2: `InferEnv<'s>` is a `PhantomData` placeholder — pass by value as Copy

The current Rust definition is:

```rust
pub struct InferEnv<'s>(pub std::marker::PhantomData<&'s ()>);
```

Treat it as Copy (PhantomData is Copy-auto). Pass by value. Scala callers construct it as `InferEnv(originalCallingEnv, ranges, callLocation, selfEnv, region)` — that call site is commented-Scala and stays frozen. When Rust bodies eventually need real fields, Slab 14 fills them in and updates call sites; Slab 12 doesn't care.

**Don't add fields to `InferEnv` in Slab 12**. Signature work only.

### Gotcha 3: `SimpleSolverState<Rule, Rune, Conclusion>` is 3-generic, no explicit lifetimes

`src/solver/simple_solver_state.rs:11` defines `pub struct SimpleSolverState<Rule, Rune, Conclusion>` with **no explicit lifetime params**. Scala uses `SimpleSolverState[IRulexSR, IRuneS, ITemplataT[ITemplataType]]`. Rust translation: `SimpleSolverState<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>>`.

When passed as param, `&mut SimpleSolverState<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>>`. Note the borrow adds no lifetime annotation — Rust elides.

If compiler complains about needing explicit lifetime on the `&mut`, add `&'a mut SimpleSolverState<...>` with `'a` declared in the fn's generic list — but try the elided form first.

### Gotcha 4: `FailedSolve` / `ISolverError` / `StampFunctionSuccess` / `FindFunctionFailure` — grep before using

Several Scala return types are Rust types that exist in various states of placeholder. Before writing a return, grep:

- `pub struct FailedSolve` — likely in `src/solver/`
- `pub enum ISolverError` — likely in `src/solver/`
- `pub struct StampFunctionSuccess`
- `pub struct FindFunctionFailure` — already confirmed at `overload_resolver.rs:106` as `_Phantom`

If the type exists as `_Phantom` placeholder, use it as-is. If it doesn't exist at all, either (a) the return type is dead code (commented-Scala only) and you don't need it, or (b) use `()` with a TODO. Prefer (a) by reading the Scala body — if the return is `Result[X, Y]` and `Y` isn't defined in Rust, `Y` probably lives in `src/solver/` or is missing and needs grepping.

### Gotcha 5: reachability.rs free fns — lift to `impl Compiler` (same convention as Slab 9/10 object methods)

The 7 `visit_*` + 1 `find_reachables` free fns in `reachability.rs` are Scala's `object Reachability` methods. Per Slab 9/10 convention, lift onto `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't { pub fn foo(&self, ...) { ... } }`.

Unlike the prior 3 Slab-12 files, reachability.rs's fns already have param lists — but with wrong/missing lifetime params (e.g. `visit_struct`'s `InterfaceEdgeBlueprintT` lacks `<'s, 't>`). Your job during the lift: correct the lifetimes AND add `&self`.

`Reachables::size()` is a method on the `Reachables` struct, not a Compiler method. Leave it as a method on `Reachables`; just ensure the body is `panic!("Unimplemented: Slab 14 — body migration");` (update from the current `"Unimplemented: size"`).

### Gotcha 6: `HashMap<InterfaceTT, HashMap<StructTT, Vec<PrototypeT>>>` in reachability — by-ref slots

Reachability's `edges` parameter is a nested map of concrete kind-TT types. Translation:

```rust
edges: &HashMap<InterfaceTT<'s, 't>, HashMap<StructTT<'s, 't>, Vec<&'t PrototypeT<'s, 't>>>>
```

`InterfaceTT` / `StructTT` are Copy (pass by value keys). Inner `Vec<&'t PrototypeT>` holds arena refs. The outer `&` makes the whole thing a read-only borrow.

Import `std::collections::HashMap` if not already imported (check the file's `use` block).

### Gotcha 7: `r#continue` — raw identifier, keep the prefix

Rust reserves `continue`. The stub at `infer_compiler.rs:337` uses `pub fn r#continue(&self)` to bypass the reservation. **Keep the `r#` prefix** in the lifted signature:

```rust
pub fn r#continue(
    &self,
    envs: InferEnv<'s>,
    state: &mut CompilerOutputs<'s, 't>,
    solver: &mut SimpleSolverState<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>>,
) -> Result<(), FailedSolve<'s, 't, IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>> {
    panic!("Unimplemented: Slab 14 — body migration");
}
```

Don't rename to `continue_` or `do_continue` — that would be a novel Rust divergence.

### Gotcha 8: `compiler_solver.rs` has its own Scala block structure — leave consumer sigs in place when dropping `delegate`

`compiler_solver.rs` fn signatures already have real params. Slab 12 only deletes the `delegate: &dyn IInfererDelegate<'s, 't>,` line from each signature — don't rewrite the rest of the param list.

Spot the pattern:

```rust
pub fn foo<'s, 't>(
    envs: InferEnv<'s>,
    state: &mut CompilerOutputs<'s, 't>,
    delegate: &dyn IInfererDelegate<'s, 't>,   // ← delete this line
    ...
) -> ... { panic!(); }
```

Result:

```rust
pub fn foo<'s, 't>(
    envs: InferEnv<'s>,
    state: &mut CompilerOutputs<'s, 't>,
    ...
) -> ... { panic!(); }
```

### Gotcha 9: bodies stay `panic!()`

Several Slab-12 methods have trivial Scala bodies:
- `r#continue` — 1 line (`compilerSolver.continue(envs, state, solver)`).
- `is_descendant` / `is_parent` — short match bodies.

Don't port them. Slab 14.

### Gotcha 10: panic message uses "Slab 14 — body migration"

Match Slabs 10/11. Don't retouch stale messages elsewhere.

### Gotcha 11: one-fn impl blocks per TL-HANDOFF convention

Each method its own `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't { pub fn foo(...) { panic!(...) } }` block. For the 35 `(&self)` stubs, the impl headers already exist — you're just filling in params. For reachability.rs's 7 free fn lifts, create new impl wrappers.

### Gotcha 12: Scala `/* */` blocks frozen

Same as every prior slab. Pre-commit hook enforces byte-level.

### Gotcha 13: no downstream breakage expected

All 35 stubs are `panic!()`. `compiler_solver.rs`'s fns are also `panic!()`. Dropping the `delegate` param doesn't break any caller because no caller exists yet (every potential caller is also a `panic!()` stub).

### Gotcha 14: `IsParentResult` is a 1-variant `_Phantom` enum right now — return by value

`pub enum IsParentResult { _Phantom }` at `impl_compiler.rs:46`. The concrete `IsParent { ... }` and `IsntParent { ... }` structs exist at lines 52 and 64 but are not yet folded into the enum. For signature work, just return `IsParentResult` — Slab 14+ flips to an `IsParentResult<'s, 't> { IsParent(&'t IsParent<'s, 't>), IsntParent(&'t IsntParent<'s, 't>), ... }` or similar.

If Scala returns `IsParentResult`, Rust returns `IsParentResult`. No lifetime params needed because `_Phantom` has none.

### Gotcha 15: `IDefiningError` is `pub enum IDefiningError {}` — no lifetime params

Empty enum. Use as-is: `Result<CompleteDefineSolve, IDefiningError>`. Slab 14+ fills variants (which may introduce lifetimes).

### Gotcha 16: `IResolvingError<'s, 't>` DOES have lifetime params

Different from `IDefiningError`. Check `infer_compiler.rs:62` — it's currently declared with `<'s, 't>` and two placeholder variants. Return `Result<CompleteResolveSolve, IResolvingError<'s, 't>>`.

---

## Step-by-step plan

### Step 1: Confirm starting point

```bash
cargo check --lib --manifest-path /Volumes/V/Sylvan/FrontendRust/Cargo.toml > /tmp/sylvan-slab-12.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-12.txt
```

Must print `0`.

### Step 2: Pre-flight type location refresher

Grep to confirm these exist (everything should, verified during handoff authoring):

- `pub struct FailedSolve` — `src/solver/`
- `pub enum ISolverError` — `src/solver/`
- `pub struct StampFunctionSuccess` — `src/typing/overload_resolver.rs`
- `pub struct FindFunctionFailure` — `src/typing/overload_resolver.rs:106`
- `pub struct SimpleSolverState` — `src/solver/simple_solver_state.rs:11`
- `pub struct InferEnv` — `src/typing/infer_compiler.rs:86`
- `pub struct CompleteDefineSolve` / `CompleteResolveSolve` — `src/typing/infer_compiler.rs:36`/`:28`
- `pub enum IDefiningError` / `IResolvingError` — `src/typing/infer_compiler.rs:75`/`:62`
- `pub struct InitialKnown` / `InitialSend` — `src/typing/infer_compiler.rs:114`/`:106`
- `pub enum IsParentResult` — `src/typing/citizen/impl_compiler.rs:46`
- `pub struct IInfererDelegate` — **to be deleted** at `src/typing/infer/compiler_solver.rs:156`

If a grep fails unexpectedly, stop and flag before coding.

### Step 3: Order of operations

Recommended lift order:

1. **`compiler_solver.rs` cleanup (15-min task)** — delete the trait + `delegate` params first. Nothing else in Slab 12 depends on the trait existing, so clear it early. Verify `cargo check --lib` is still clean after.
2. **`reachability.rs` (30-45 min)** — smallest logical unit. 7 free-fn lifts + 1 method body swap. Warm-up.
3. **`impl_compiler.rs` (60 min)** — 9 stubs. Solid mid-size.
4. **`overload_resolver.rs` (60-75 min)** — 11 stubs. Medium.
5. **`infer_compiler.rs` (90-120 min)** — 15 stubs, largest. Do last; the pattern is most familiar by this point.

### Step 4: Incremental verification

After each file:

```bash
cargo check --lib --manifest-path /Volumes/V/Sylvan/FrontendRust/Cargo.toml > /tmp/sylvan-slab-12.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-12.txt
```

Fix errors before moving on. Common Slab-12 mistakes:

- Forgot to drop `delegate: &dyn IInfererDelegate<'s, 't>` from a `compiler_solver.rs` sig.
- Deleted `IInfererDelegate` trait but left a `&dyn IInfererDelegate` reference somewhere — grep for leftovers.
- `InferEnv<'s, 't>` instead of `InferEnv<'s>` — it's single-lifetime currently.
- `IResolvingError` without `<'s, 't>` (vs. `IDefiningError` which is `<>`).
- `HashMap` used without `use std::collections::HashMap;` at the top.
- Trying to add fields to `InferEnv` or `InitialKnown` or similar — don't.
- `SimpleSolverState<'s, 't, ...>` — it's 3-generic, no lifetimes of its own.
- Renaming `r#continue` to `continue_` — don't.

### Step 5: Final verification

```bash
cargo check --lib --manifest-path /Volumes/V/Sylvan/FrontendRust/Cargo.toml > /tmp/sylvan-slab-12.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-12.txt       # must be 0
grep -c "^warning" /tmp/sylvan-slab-12.txt     # must be 0
tail -3 /tmp/sylvan-slab-12.txt                 # must show "Finished"
```

Sanity greps:

```
Grep pattern: "pub fn .*\(&self\) \{ panic" path: FrontendRust/src/typing/infer_compiler.rs  # must match 0
Grep pattern: "pub fn .*\(&self\) \{ panic" path: FrontendRust/src/typing/overload_resolver.rs  # must match 0
Grep pattern: "pub fn .*\(&self\) \{ panic" path: FrontendRust/src/typing/citizen/impl_compiler.rs  # must match 0
Grep pattern: "IInfererDelegate" path: FrontendRust/src/typing (excluding /* */ blocks)  # must match 0 in Rust code
Grep pattern: "pub trait IInfererDelegate" path: FrontendRust/src  # must match 0
Grep pattern: "Slab 14 — body migration" path: FrontendRust/src/typing/infer_compiler.rs  # must be ≥15
Grep pattern: "Slab 14 — body migration" path: FrontendRust/src/typing/overload_resolver.rs  # must be ≥11
Grep pattern: "Slab 14 — body migration" path: FrontendRust/src/typing/citizen/impl_compiler.rs  # must be ≥9
Grep pattern: "Slab 14 — body migration" path: FrontendRust/src/typing/reachability.rs  # must be ≥7 (8 if you also bumped Reachables::size's message)
```

### Step 6: Diff self-review

```bash
git diff FrontendRust/src/typing/ | head -400
git diff --stat
```

Confirm:
- 5 files changed (infer_compiler, overload_resolver, impl_compiler, reachability, compiler_solver).
- No Scala `/* */` block edits.
- No other files touched.
- Every new panic message uses "Slab 14 — body migration".
- `IInfererDelegate` trait line is deleted in `compiler_solver.rs`.

### Step 7: Hand off

**Never commit.** Hand back uncommitted; the human tags `slab-12-complete`.

Work-order checkpoints:
- Step 2 — pre-flight greps recorded.
- Step 3 (1/5) — `compiler_solver.rs` `IInfererDelegate` cleanup done.
- Step 3 (2/5) — `reachability.rs` done.
- Step 3 (3/5) — `impl_compiler.rs` done.
- Step 3 (4/5) — `overload_resolver.rs` done.
- Step 3 (5/5) — `infer_compiler.rs` done.
- Step 5 — verification greps pass.
- Step 6 — diff self-review.
- Step 7 — hand back.

---

## What "done" looks like

- `cargo check --lib` passes with 0 errors, 0 warnings.
- All 35 target `(&self)` stubs across the 3 primary files have real signatures inside existing one-fn `impl` blocks. Bodies `panic!()`.
- `reachability.rs`'s 7 free fns (`find_reachables`, `visit_function`, `visit_struct`, `visit_interface`, `visit_impl`, `visit_static_sized_array`, `visit_runtime_sized_array`) lifted onto `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>` with fixed lifetime params. `Reachables::size()` method body bumped to `Slab 14 — body migration`.
- `IInfererDelegate` trait deleted from `compiler_solver.rs`. Every `delegate: &dyn IInfererDelegate<'s, 't>,` param removed from consumer signatures in that file.
- `InferEnv<'s>` used as-is (single-lifetime PhantomData placeholder); not expanded.
- `InitialKnown` / `InitialSend` / `CompleteDefineSolve` / `CompleteResolveSolve` / `IDefiningError` / `IResolvingError<'s, 't>` / `FindFunctionFailure<'s, 't>` / `IsParentResult` used as-is.
- `coutputs` params are `&mut CompilerOutputs<'s, 't>` (conservative default).
- `&'t IInDenizenEnvironmentT<'s, 't>` for every env param.
- Scout-side types are `<'s>` only.
- Scala `/* */` blocks unchanged byte-for-byte.
- Only the 5 target files modified.
- Handed back uncommitted.

---

## When you're stuck

- **"cannot find trait `IInfererDelegate`"**: you deleted it. Correct. Now delete the `&dyn IInfererDelegate` params too. Gotcha 1.
- **"fn takes 8 arguments but was called with 9"**: a caller somewhere still passes `delegate`. But we said no callers... if this fires, grep for the caller and check whether it's a body (which should be `panic!()`) or a misplaced non-stub. Most likely scenario: a `Scala /* */` block was accidentally de-commented; undo.
- **"cannot find type `FailedSolve`"**: grep `src/solver/`. If it doesn't exist, use the return type Scala specifies as a placeholder; fall back to `()` with a TODO (unlikely — the solver types all exist).
- **"`InferEnv` takes 0 type arguments but 2 were supplied"**: you wrote `InferEnv<'s, 't>`. It's `InferEnv<'s>` only. Gotcha 2.
- **"`SimpleSolverState` expects 3 type arguments, not 5"**: same. It's `<Rule, Rune, Conclusion>`, no lifetime params of its own. Gotcha 3.
- **"`IDefiningError` takes 0 lifetime arguments"**: correct — it's `pub enum IDefiningError {}`. Use bare `IDefiningError`. Gotcha 15.
- **"`IResolvingError` takes 2 lifetime arguments but 0 were supplied"**: write `IResolvingError<'s, 't>`. Gotcha 16.
- **"I want to fold `IsParent` / `IsntParent` structs into `IsParentResult`"**: don't. Slab 14. Gotcha 14.
- **"`r#continue` is ugly — can I rename?"**: no. Gotcha 7.
- **"the reachability.rs `//` comments look weird"**: they're Scala-comment-inside-a-`/* */`-block. Don't touch them. Hook still enforces the outer `/* */`.
- **"I want to port `continue` body — it's one line"**: don't. Gotcha 9.
- **"can I touch `function_compiler.rs`?"**: no. Slab 13.

## Where to file questions

- **Design**: Slabs 9-11 handoff docs are the canonical pattern reference. This doc is authoritative for Slab-12-specific decisions (IInfererDelegate deletion, InferEnv phantom handling, reachability lift, SimpleSolverState generics).
- **Scala semantics**: each Scala `/* def ... */` block is the spec.
- **Hook rejections**: usually accidental whitespace inside `/* */`.

## Final advice

This is the fifth signature-rewrite slab. The pattern from Slabs 8-11 carries directly. Slab-12 novelties:

1. **`IInfererDelegate` vestige deletion** — trait + every `delegate: &dyn IInfererDelegate` param in `compiler_solver.rs`. No replacement. Gotcha 1.
2. **`InferEnv<'s>` is a phantom single-lifetime placeholder** — use as-is, don't expand. Gotcha 2.
3. **Reachability lifts** — 7 free fns become `impl Compiler` methods with corrected lifetimes (existing param lists are mostly right but missing `<'s, 't>` on nested types). Gotcha 5.
4. **`IsParentResult` / `IDefiningError` / `InitialKnown` / etc. are `_Phantom` or empty placeholders** — use as-is, don't fill. Gotcha 14.
5. **`r#continue` raw identifier** — keep the prefix. Gotcha 7.

After Slab 12, the typing pass has:
- All `CompilerOutputs` methods with real sigs (Slab 8).
- All `TemplataCompiler` methods with real sigs (Slabs 9 + 10).
- All `compiler.rs` residuals + `local_helper.rs` / `struct_compiler.rs` orphans (Slab 10).
- All expression-layer sigs (Slab 11).
- All solver + resolver + impl + reachability sigs (Slab 12).
- ~23 remaining sub-compiler methods still partial (Slab 13).

**Slab 13** = function/citizen/macros sigs (~23 across `function_compiler.rs` / `function_body_compiler.rs` / `destructor_compiler.rs` / `struct_compiler_core.rs` / `anonymous_interface_macro.rs`). Last signature-rewrite slab.

After Slab 13, the signature-rewrite phase is complete. Slab 14+ is pure body migration, driven by tests.

Good luck.
