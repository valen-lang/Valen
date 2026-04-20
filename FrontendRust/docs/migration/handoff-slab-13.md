# Handoff: Typing Pass Slab 13 — Function + Citizen-Core + Macros Method Signature Rewrite (FINAL signature-rewrite slab)

## Who this is for

You're picking up an in-progress Rust port of a Scala compiler frontend. A senior engineer has been moving through the typing pass one "slab" at a time per `quest.md` §12. Slabs 0-12 are done:

- **Slabs 0-7** — arena substrate, names, types, templatas, envs, AST, `CompilerOutputs` data shape, `HinputsT`/`Compiler` scaffolding.
- **Slab 8** — `CompilerOutputs` method signatures (54 sigs).
- **Slab 9** — `object TemplataCompiler` method signatures (35 sigs).
- **Slab 10** — `compiler.rs` residuals + `local_helper.rs`/`struct_compiler.rs` orphans + `class TemplataCompiler` tail (~31 sigs).
- **Slab 11** — expression-layer signatures (39 sigs across 4 files).
- **Slab 12** — solver + resolver signatures (~35 sigs across 3 files + reachability lifts + `IInfererDelegate` vestige cleanup).

**Slab 13 is the sixth and final signature-rewrite slab**, covering the function-compilation layer, citizen-core layer, and the remaining macro file:

- **`src/typing/function/function_compiler.rs`** — 8 stubs (generic function evaluation entry points)
- **`src/typing/function/function_body_compiler.rs`** — 3 stubs (body evaluation driver)
- **`src/typing/function/destructor_compiler.rs`** — 2 stubs (`get_drop_function`, `drop`)
- **`src/typing/citizen/struct_compiler_core.rs`** — 6 stubs (`compile_struct_core`, `translate_citizen_attributes`, etc.)
- **`src/typing/macros/anonymous_interface_macro.rs`** — 4 stubs (the anonymous-interface macro's helpers; the 5th method at line 67 is already signature-complete)

Total: **23 bare `(&self)` stubs** across 5 files. Budget ~2-3 hours focused. Smaller than Slabs 10-12 because no novel translation patterns introduced — everything reuses prior slab conventions.

After Slab 13, the signature-rewrite phase is complete. `cargo check --lib` passes clean with all methods having real parameter/return types and `panic!("Unimplemented: Slab 14 — body migration")` bodies. Slab 14+ begins body migration driven by failing tests — different shape of work (per-method instead of per-file-family).

**Read these first in this order**, then come back:

1. `FrontendRust/docs/migration/handoff-slab-11.md` — the pattern for `NodeEnvironmentBox` → `&mut NodeEnvironmentBuilder<'s, 't>` and `FunctionEnvironmentBoxT` → `&mut FunctionEnvironmentBuilder<'s, 't>` — reused verbatim in Slab 13 since these params appear throughout the function-compilation layer.
2. `FrontendRust/docs/migration/handoff-slab-12.md` — for `InferEnv<'s>`, `IResolvingError<'s, 't>`, `FindFunctionFailure<'s, 't>`, `StampFunctionSuccess<'s, 't>` handling. A few Slab-13 signatures reference these.
3. `FrontendRust/docs/migration/handoff-slab-9.md` — canonical translation-rules reference.
4. `TL-HANDOFF.md` at repo root — file-layout standards + slab roadmap.
5. This doc.

You shouldn't need to read the Scala source externally — every Scala `def` sig is already embedded inline in the `/* def ... */` blocks beneath each Rust stub.

---

## The big picture: why Slab 13 exists

The function-compilation and citizen-core layers are the last sub-compiler families with partial signatures. Scala organizes them as `FunctionCompiler`, `FunctionBodyCompiler` (internal impl of FunctionCompiler's lower half), `DestructorCompiler`, `StructCompilerCore`, and the `AnonymousInterfaceMacro`. In the god-struct refactor, all five's methods were lifted onto `Compiler<'s, 'ctx, 't>` as bare `pub fn foo(&self) { panic!(...) }` stubs. Your job is to fill in the parameter lists.

No new design decisions. Slab 13 reuses every pattern from Slabs 8-12. The only Slab-13-specific wrinkles:

1. **`drop` is a valid Rust method name.** Keep `pub fn drop(&self, ...)` in `destructor_compiler.rs`. Not a keyword, no `r#` prefix needed.
2. **Pre-existing Rust-only disambiguation suffixes.** Several Slab-13 methods have suffix disambiguators from prior god-struct work: `_core` (for `struct_compiler_core.rs` vs. `struct_compiler.rs`), `_ext` (for overload resolution when Scala overloaded a name), `_anonymous_interface` (for the macro's helpers). Preserve all these suffixes — they're earlier slab-work and renaming them would diverge from the current Rust-side names.
3. **`IFunctionGenerator` trait is out of scope.** The trait at `compiler.rs:53` has a `generate` method with `()` placeholders. Slab 13 does NOT fill it — it's a Slab 14+ concern (likely becomes a dispatch-tag enum similar to `FunctionBodyMacro`, following the same post-refactor pattern).

By the end of Slab 13:

- `cargo check --lib` passes with 0 errors, 0 warnings.
- All 23 target stubs across the 5 files have real signatures.
- Bodies remain `panic!("Unimplemented: Slab 14 — body migration");`.
- Scala `/* */` blocks unchanged byte-for-byte.
- Only the 5 target files modified.
- **The signature-rewrite phase is complete.** The typing pass is shaped; Slab 14+ fills bodies.

**What Slab 13 is NOT:**

- No body migration. Bodies stay `panic!()`. Slab 14+.
- No `IFunctionGenerator` trait conversion. Out of scope; Slab 14+.
- No filling of `IEvaluateFunctionResult<'s, 't>` / `EvaluateFunctionSuccess<'s, 't>` / `EvaluateFunctionFailure<'s, 't>` placeholder structs. Use as-is. Slab 14+ fills variants.
- No `InitialKnown` / `InitialSend` field-filling either. Same deferral.
- No `StampFunctionSuccess<'s, 't>` / `FindFunctionFailure<'s, 't>` expansion. Same.
- No touching of the 17 macro files already clean per the audit (`ssa_*`, `rsa_*`, `struct_constructor_macro.rs`, `struct_drop_macro.rs`, `interface_drop_macro.rs`, `lock_weak_macro.rs`, `abstract_body_macro.rs`, etc.). Only `anonymous_interface_macro.rs` needs work.

---

## Stub inventory: the 23 targets

### File 1: `src/typing/function/function_compiler.rs` — 8 stubs

| Line | Stub |
|---|---|
| 197 | `evaluate_generic_function_from_non_call` |
| 226 | `evaluate_templated_light_function_from_call_for_prototype` |
| 254 | `evaluate_templated_function_from_call_for_prototype` |
| 302 | `evaluate_templated_function_from_call_for_prototype_ext` — `_ext` suffix disambiguates from the previous method (Scala overloaded `evaluateTemplatedFunctionFromCallForPrototype` by signature; Rust can't) |
| 344 | `evaluate_generic_virtual_dispatcher_function_for_prototype` |
| 367 | `evaluate_generic_light_function_from_call_for_prototype` |
| 393 | `evaluate_closure_struct` |
| 426 | `determine_closure_variable_member` |

### File 2: `src/typing/function/function_body_compiler.rs` — 3 stubs

| Line | Stub |
|---|---|
| 68 | `declare_and_evaluate_function_body` |
| 189 | `evaluate_function_body` |
| 283 | `evaluate_lets` |

### File 3: `src/typing/function/destructor_compiler.rs` — 2 stubs

| Line | Stub |
|---|---|
| 43 | `get_drop_function` |
| 69 | `drop` — valid Rust method name; no `r#` prefix needed (Gotcha 1) |

### File 4: `src/typing/citizen/struct_compiler_core.rs` — 6 stubs

| Line | Stub |
|---|---|
| 43 | `compile_struct_core` — `_core` suffix distinguishes from `struct_compiler.rs::compile_struct` |
| 179 | `translate_citizen_attributes` |
| 196 | `compile_interface_core` |
| 275 | `make_struct_members` |
| 291 | `make_struct_member` |
| 332 | `make_closure_understruct_core` |

### File 5: `src/typing/macros/anonymous_interface_macro.rs` — 4 stubs

| Line | Stub |
|---|---|
| 174 | `map_runes_anonymous_interface` |
| 221 | `inherited_method_rune_anonymous_interface` |
| 238 | `make_struct_anonymous_interface` |
| 453 | `make_forwarder_function_anonymous_interface` |

Note: a fifth method, `get_interface_sibling_entries_anonymous_interface` at line 67, is already signature-complete (`&self, interface_name: IdT<'s, 't>, interface_a: &'s InterfaceA<'s>) -> Vec<(IdT<'s, 't>, IEnvEntryT<'s, 't>)>`) from prior work. Don't re-touch it — its body stays `panic!()` for Slab 14+.

---

## Signature translation rules

**Same table as Slabs 8-12.** Quick reference for Slab-13-common types:

| Scala | Rust |
|---|---|
| `CompilerOutputs` (param, mutates) | `&mut CompilerOutputs<'s, 't>` — conservative default |
| `NodeEnvironmentBox` | `&mut NodeEnvironmentBuilder<'s, 't>` (Slab 11 Gotcha 1) |
| `FunctionEnvironmentBoxT` | `&mut FunctionEnvironmentBuilder<'s, 't>` (Slab 11 Gotcha 1) |
| `IInDenizenEnvironmentT` | `&'t IInDenizenEnvironmentT<'s, 't>` (Slab-4 override) |
| `FunctionEnvironmentT` (frozen) | `&'t FunctionEnvironmentT<'s, 't>` |
| `NodeEnvironmentT` (frozen) | `&'t NodeEnvironmentT<'s, 't>` |
| `List[RangeS]` / `Vector[RangeS]` | `&[RangeS<'s>]` |
| `RangeS` | `RangeS<'s>` by value |
| `RegionT` / `OwnershipT` | by value |
| `IdT[X]` (any phantom) | `IdT<'s, 't>` by value |
| `CoordT` / `KindT` / `ITemplataT[X]` | by value (Copy per Slab 3) |
| `ICitizenTT` / `StructTT` / `InterfaceTT` | by value |
| `StructDefinitionTemplataT` / `InterfaceDefinitionTemplataT` / `ImplDefinitionTemplataT` / `CitizenDefinitionTemplataT` | `&'t X<'s, 't>` |
| `StructDefinitionT` / `InterfaceDefinitionT` / `FunctionDefinitionT` / `ImplT` / `CitizenDefinitionT` | `&'t X<'s, 't>` |
| `FunctionA` / `StructA` / `InterfaceA` / `ImplA` | `&'s X<'s>` (scout-side) |
| `AtomSP` / `Vector[AtomSP]` | `&'s AtomSP<'s>` / `&[&'s AtomSP<'s>]` |
| `BlockSE` / `IExpressionSE` | `&'s BlockSE<'s>` / `&'s IExpressionSE<'s>` |
| `IRulexSR` / `Vector[IRulexSR]` | `&'s IRulexSR<'s>` / `&[&'s IRulexSR<'s>]` |
| `IRuneS` / `Vector[IRuneS]` | `IRuneS<'s>` by value / `&[IRuneS<'s>]` |
| `GenericParameterS` / `Vector[GenericParameterS]` | `&'s GenericParameterS<'s>` / `&[&'s GenericParameterS<'s>]` |
| `IImpreciseNameS` | `IImpreciseNameS<'s>` by value |
| `IVarNameT` / `IVarNameS` | `IVarNameT<'s, 't>` / `IVarNameS<'s>` by value |
| `ILocalVariableT` | `ILocalVariableT<'s, 't>` by value (Copy per Slab 4 — verify; fall back to `&'t ILocalVariableT<'s, 't>` if compiler rejects) |
| `ReferenceExpressionTE` (param) | `&'t ReferenceExpressionTE<'s, 't>` |
| `ReferenceExpressionTE` (return) | `&'t ReferenceExpressionTE<'s, 't>` |
| `AddressExpressionTE` | `&'t AddressExpressionTE<'s, 't>` |
| `FunctionHeaderT` | `&'t FunctionHeaderT<'s, 't>` (AST type) |
| `FunctionBannerT` | `&'t FunctionBannerT<'s, 't>` (grep if unsure) |
| `PrototypeT` / `SignatureT` | `&'t PrototypeT<'s, 't>` / `&'t SignatureT<'s, 't>` |
| `ParameterT` / `Vector[ParameterT]` | `&'t ParameterT<'s, 't>` / `&[&'t ParameterT<'s, 't>]` |
| `LocationInDenizen` | `LocationInDenizen<'s>` by value |
| `LocationInFunctionEnvironmentT` | `LocationInFunctionEnvironmentT<'s>` by value (AASSNCMCX deviation per TL-HANDOFF — internal `Vec<i32>`) |
| `StrI` | `StrI<'s>` by value |
| `IEvaluateFunctionResult` (return) | `IEvaluateFunctionResult<'s, 't>` by value (currently `_Phantom` placeholder; use as-is) |
| `EvaluateFunctionSuccess` / `EvaluateFunctionFailure` | `EvaluateFunctionSuccess<'s, 't>` / `EvaluateFunctionFailure<'s, 't>` (both `_Phantom` placeholders) |
| `IFunctionGenerator` (param — in `generateFunction` sig) | `&()` with a TODO — see Gotcha 3 |
| `InitialKnown` / `InitialSend` / `Vector[...]` | by value / `&[...]` (both empty stubs from Slab 12) |
| `IFindFunctionFailureReason` | grep first; likely Rust enum — use as-is |
| `CouldntSolveRulesA` / similar scout-side errors | grep; use `&'s X<'s>` if it exists as a scout arena type |
| `IEnvEntryT` (return as part of a tuple) | `&'t IEnvEntryT<'s, 't>` or `IEnvEntryT<'s, 't>` by value — check Slab 4 whether the wrapper enum is Copy. Default: by value; Slab 11's `translate_pattern_list` precedent used this shape |
| `MacroCallS` / `ICitizenAttributeS` | `&'s MacroCallS<'s>` / `&'s ICitizenAttributeS<'s>` (scout) |
| `Vector[X]` (owned return) | `Vec<X>` |
| `Map[K, V]` (return) | `HashMap<K, V>` |
| `Set[X]` | `HashSet<X>` |
| `Profiler.frame(() => { ... })` wrappers in Scala bodies | **ignore** — Rust doesn't use these (per project conventions) |
| `Interner` / `Keywords` (if any appear as explicit params) | **drop** — accessed via `self.typing_interner` / `self.keywords` |

### Receiver classification

All 23 methods on `&self`. None need `&mut self`.

### Derive / lifetime bounds

- Every `impl` block: `<'s, 'ctx, 't>` with `where 's: 't`.
- `pub fn` (not `fn`).
- No new derives.

---

## Shape of a lifted stub

### Example 1: `destructor_compiler.rs::drop`

Before (line 66):

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn drop(&self) {
        panic!("Unimplemented: drop");
    }
/*
  def drop(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    contextRegion: RegionT,
    undestructedExpr2: ReferenceExpressionTE):
  (ReferenceExpressionTE) = { ... }
*/
}
```

After:

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn drop(
        &self,
        env: &'t IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        context_region: RegionT,
        undestructed_expr_2: &'t ReferenceExpressionTE<'s, 't>,
    ) -> &'t ReferenceExpressionTE<'s, 't> {
        panic!("Unimplemented: Slab 14 — body migration");
    }
/*
  def drop(... same as before ...)
*/
}
```

Note: method name stays `drop`. Not reserved in Rust.

### Example 2: `struct_compiler_core.rs::compile_struct_core` (with `_core` suffix)

Preserve the `_core` suffix — it's a prior god-struct-collision rename disambiguating from `StructCompiler.compileStruct` (lifted in Slab 10).

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn compile_struct_core(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        // ... params from the /* */ below ...
    ) -> /* ... */ {
        panic!("Unimplemented: Slab 14 — body migration");
    }
/*
  def compileStruct(... Scala name is still compileStruct; Rust disambiguated ...)
*/
}
```

---

## Gotchas

### Gotcha 1: `drop` is not a Rust keyword

Rust's `Drop` trait and `std::mem::drop` function share the name but `drop` is NOT a reserved word. You can freely write `pub fn drop(&self, ...)` as a method. **No `r#` prefix**. The method body calling context uses `self.drop(...)` which doesn't collide with `std::mem::drop` because method resolution prefers inherent methods.

If the body (Slab 14+) ever wants `std::mem::drop`, it can use `::std::mem::drop(x)` explicitly.

### Gotcha 2: preserve `_core` / `_ext` / `_anonymous_interface` suffix disambiguators

Prior slabs/refactors applied these Rust-only suffixes to resolve god-struct name collisions. Don't re-rename:

- **`_core` suffix**: `compile_struct_core`, `compile_interface_core`, `make_closure_understruct_core` — distinguishes `StructCompilerCore`'s methods from `StructCompiler`'s.
- **`_ext` suffix**: `evaluate_templated_function_from_call_for_prototype_ext` — the Scala side overloaded `evaluateTemplatedFunctionFromCallForPrototype` by parameter type; Rust resolved the collision with the suffix.
- **`_anonymous_interface` suffix**: `map_runes_anonymous_interface`, `inherited_method_rune_anonymous_interface`, `make_struct_anonymous_interface`, `make_forwarder_function_anonymous_interface` — distinguishes the macro's local helpers from Scala's scoped `object AnonymousInterfaceMacro`.

Leave all these as-is. Slab 14+ body migration may or may not revisit; Slab 13 doesn't touch them.

### Gotcha 3: `IFunctionGenerator` trait is out of scope — use `&()` placeholder if a signature references it

`IFunctionGenerator` is a trait at `compiler.rs:53` with its `generate` method carrying `()` placeholders. It's scheduled for conversion to a dispatch-tag enum (similar to `FunctionBodyMacro` at `macros.rs:23`) but that's Slab 14+ work.

Some Slab-13 signatures (e.g. `evaluate_templated_function_from_call_for_prototype` family) may reference `IFunctionGenerator` as a parameter in their Scala blocks. For Slab 13:

```rust
// TODO: Slab 14 — IFunctionGenerator trait becomes a dispatch-tag enum; placeholder for now
generator: &(),
```

Mirrors the Slab 9 `IPlaceholderSubstituter` treatment (Gotcha 4 there). Don't define a new enum in Slab 13.

### Gotcha 4: `IEvaluateFunctionResult<'s, 't>` / `EvaluateFunctionSuccess<'s, 't>` / `EvaluateFunctionFailure<'s, 't>` are `_Phantom` placeholders — use as-is

At `function_compiler.rs:79-100`, all three are:

```rust
pub enum IEvaluateFunctionResult<'s, 't> {
    _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
pub struct EvaluateFunctionSuccess<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
pub struct EvaluateFunctionFailure<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
```

Return them as-is by value. Slab 14+ fills the real fields (`prototype: PrototypeTemplataT[...]`, `inferences: Map[...]`, `instantiationBoundArgs: InstantiationBoundArgumentsT[...]` for `EvaluateFunctionSuccess`; `reason: IDefiningError` for `EvaluateFunctionFailure`) and collapses the phantoms.

**Don't fill the fields in Slab 13.**

### Gotcha 5: `NodeEnvironmentBox` / `FunctionEnvironmentBoxT` — same Slab-11 translation

- `NodeEnvironmentBox` → `&mut NodeEnvironmentBuilder<'s, 't>`
- `NodeEnvironmentT` (read-only snapshot) → `&'t NodeEnvironmentT<'s, 't>`
- `FunctionEnvironmentBoxT` → `&mut FunctionEnvironmentBuilder<'s, 't>`
- `FunctionEnvironmentT` (frozen) → `&'t FunctionEnvironmentT<'s, 't>`

See Slab 11 Gotcha 1 for the full discussion. Both builders exist at `src/typing/env/function_environment_t.rs:1181` (FunctionEnvironmentBuilder) and `:1220` (NodeEnvironmentBuilder).

### Gotcha 6: `FunctionBannerT` / `FunctionHeaderT` — grep if unsure about arena-allocated vs by-value

Most AST types from Slab 5 are arena-allocated (`&'t X<'s, 't>`). Grep for the exact types used in Slab 13 signatures:

- `FunctionHeaderT` — should be `&'t FunctionHeaderT<'s, 't>`
- `FunctionBannerT` — grep `pub struct FunctionBannerT` in `src/typing/ast/`; likely arena-allocated
- `ParameterT` — grep; likely `&'t ParameterT<'s, 't>`

If a type is Copy (small value), pass by value. If it's a heavy struct (holds arena-referenced children), pass by `&'t`. Default: `&'t` for anything labeled `T` at AST scope.

### Gotcha 7: closure params in `function_compiler.rs` / `function_body_compiler.rs`

A few Scala signatures in these files carry continuation closures (similar to the pattern-compiler closures from Slab 11). Same convention:

```rust
continuation: impl FnOnce(
    &mut CompilerOutputs<'s, 't>,
    &mut FunctionEnvironmentBuilder<'s, 't>,
    // ... other params ...
) -> &'t ReferenceExpressionTE<'s, 't>,
```

Use `impl FnOnce` unless the Scala body clearly re-invokes (rare). See Slab 11 Gotcha 5 for the full rationale.

### Gotcha 8: `Vec<(IdT<'s, 't>, IEnvEntryT<'s, 't>)>` return shape for macro `getSiblingEntries`-style methods

Multiple anonymous-interface-macro methods return sibling-entries lists:

```scala
def getInterfaceSiblingEntries(interfaceName: IdT[INameT], interfaceA: InterfaceA): Vector[(IdT[INameT], IEnvEntry)]
```

Already lifted at line 67 as:

```rust
-> Vec<(IdT<'s, 't>, IEnvEntryT<'s, 't>)>
```

Slab 13's 4 anonymous-interface helpers likely use similar return shapes. Match the precedent: `IEnvEntryT<'s, 't>` by value (it's a wrapper enum; verify Copy). Don't convert to `&'t IEnvEntryT<'s, 't>` unless the compiler complains.

### Gotcha 9: `IFindFunctionFailureReason` (used in `destructor_compiler.rs`'s `drop` body and possibly as a return) — grep first

`IFindFunctionFailureReason` is a Scala sealed trait for overload-resolution failure causes. Check if Rust has a corresponding type:

```
Grep pattern: "pub (enum|struct) IFindFunctionFailureReason"
```

If it exists, use by value or by `&'t` as the pattern dictates. If it doesn't (likely — it may not have been ported yet), and a Slab-13 signature needs it, use `()` with a TODO comment. Fallback pattern from Slab 9 Gotcha 4.

### Gotcha 10: bodies stay `panic!()` — resist the temptation

Several Slab-13 methods have short Scala bodies:
- `drop` — 20 lines but mostly a match.
- `get_drop_function` — 8 lines.
- `make_struct_member` — likely a small dispatch.

**Don't port them.** Slab 14.

### Gotcha 11: panic message uses "Slab 14 — body migration"

Match Slabs 10/11/12. Don't retouch stale messages elsewhere.

### Gotcha 12: one-fn impl blocks per TL-HANDOFF convention

Impl headers already exist. You're filling `(&self)` → `(&self, ... params ...) -> RetType`. No new impls needed.

### Gotcha 13: Scala `/* */` blocks frozen

Same as every prior slab. Pre-commit hook enforces byte-level.

### Gotcha 14: no downstream breakage expected

All 23 stubs `panic!()`. No caller exercises them yet.

### Gotcha 15: check `function_compiler.rs`'s imports after lifting

`function_compiler.rs` currently has `use crate::typing::compiler::Compiler;` and little else (line 1 of the file). Your lifted signatures will reference ~15-20 types across names/types/templata/ast/env/scout-side. Add `use` statements as the compiler demands; follow the existing pattern from `expression_compiler.rs` or `infer_compiler.rs` for the full import spread.

Same for `destructor_compiler.rs`, `function_body_compiler.rs`, `struct_compiler_core.rs`, `anonymous_interface_macro.rs` — any needed imports should be added, matching the conventions of surrounding sub-compiler files.

---

## Step-by-step plan

### Step 1: Confirm starting point

```bash
cargo check --lib --manifest-path /Volumes/V/Sylvan/FrontendRust/Cargo.toml > /tmp/sylvan-slab-13.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-13.txt
```

Must print `0`.

### Step 2: Pre-flight type location refresher

These exist (verified during handoff authoring):

- `IEvaluateFunctionResult<'s, 't>` / `EvaluateFunctionSuccess<'s, 't>` / `EvaluateFunctionFailure<'s, 't>` — `src/typing/function/function_compiler.rs:79-100`
- `NodeEnvironmentBuilder<'s, 't>` — `src/typing/env/function_environment_t.rs:1220`
- `FunctionEnvironmentBuilder<'s, 't>` — `src/typing/env/function_environment_t.rs:1181`
- `StampFunctionSuccess<'s, 't>` / `FindFunctionFailure<'s, 't>` — `src/typing/overload_resolver.rs`
- `IFunctionGenerator<'s, 't>` — `src/typing/compiler.rs:53` (stays trait; Slab 13 uses `&()` placeholder)
- `FunctionBodyMacro` — `src/typing/macros/macros.rs:23` (Copy enum — reference pattern for `IFunctionGenerator`'s eventual conversion)

If a grep fails unexpectedly, stop and flag.

### Step 3: Lift file-by-file, smallest → largest

Recommended order:

1. **`destructor_compiler.rs` (2 stubs, ~20 min)** — warm-up. `drop` and `get_drop_function` both take simple env + coutputs + ranges + coord params.
2. **`function_body_compiler.rs` (3 stubs, ~30 min)** — `declare_and_evaluate_function_body`, `evaluate_function_body`, `evaluate_lets`. Builder-heavy.
3. **`anonymous_interface_macro.rs` (4 stubs, ~30 min)** — macro helpers; sibling-entries shape.
4. **`struct_compiler_core.rs` (6 stubs, ~45 min)** — struct/interface compile core + member making.
5. **`function_compiler.rs` (8 stubs, ~60 min)** — largest. 7 `evaluate_*` + 1 `determine_closure_variable_member`. `_ext` disambiguation to preserve.

### Step 4: Incremental verification

After each file:

```bash
cargo check --lib --manifest-path /Volumes/V/Sylvan/FrontendRust/Cargo.toml > /tmp/sylvan-slab-13.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-13.txt
```

Common Slab-13 mistakes:
- `drop` renamed to `drop_` or similar (Gotcha 1).
- `compile_struct_core` renamed to `compile_struct` (Gotcha 2 — collision with Slab-10 lift).
- `NodeEnvironmentBox` / `FunctionEnvironmentBoxT` left as-is (Gotcha 5).
- Trying to fill `EvaluateFunctionSuccess` fields (Gotcha 4).
- `generator: IFunctionGenerator` without the placeholder `&()` (Gotcha 3).
- Missing imports in sparsely-imported files like `destructor_compiler.rs`.
- `_ext` suffix accidentally dropped on `evaluate_templated_function_from_call_for_prototype_ext`.

### Step 5: Final verification

```bash
cargo check --lib --manifest-path /Volumes/V/Sylvan/FrontendRust/Cargo.toml > /tmp/sylvan-slab-13.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-13.txt       # must be 0
grep -c "^warning" /tmp/sylvan-slab-13.txt     # must be 0
tail -3 /tmp/sylvan-slab-13.txt                 # must show "Finished"
```

Sanity greps (adjust paths per filesystem):

```
Grep pattern: "pub fn .*\(&self\) \{ panic" path: FrontendRust/src/typing/function  # must match 0
Grep pattern: "pub fn .*\(&self\) \{ panic" path: FrontendRust/src/typing/citizen/struct_compiler_core.rs  # must match 0
Grep pattern: "pub fn .*\(&self\) \{ panic" path: FrontendRust/src/typing/macros/anonymous_interface_macro.rs  # must match 0
Grep pattern: "Slab 14 — body migration" path: FrontendRust/src/typing/function  # must be ≥13 (8+3+2)
Grep pattern: "Slab 14 — body migration" path: FrontendRust/src/typing/citizen/struct_compiler_core.rs  # must be ≥6
Grep pattern: "Slab 14 — body migration" path: FrontendRust/src/typing/macros/anonymous_interface_macro.rs  # must be ≥4
```

### Step 6: Signature-rewrite phase completion check

**This is the last signature-rewrite slab.** After Slab 13, no `(&self) { panic!("Unimplemented:` stubs should remain in `src/typing/`. Verify:

```
Grep pattern: "pub fn [a-z_]+\(&self\) \{ panic" path: FrontendRust/src/typing  # must match 0 across the whole typing tree
```

Any non-zero result means a stub slipped past (either Slab 13 missed it, or an earlier slab left something behind). Flag in the hand-back if you see a non-zero match.

### Step 7: Diff self-review

```bash
git diff FrontendRust/src/typing/ | head -400
git diff --stat
```

Confirm:
- Only 5 files changed: destructor, function_body, anonymous_interface, struct_compiler_core, function_compiler.
- No Scala `/* */` block edits.
- Every new panic message uses "Slab 14 — body migration".
- No rename of `drop`, `compile_struct_core`, `_ext`, or `_anonymous_interface` names.

### Step 8: Hand off

**Never commit.** Hand back uncommitted; the human tags `slab-13-complete` and celebrates the end of the signature-rewrite phase.

Work-order checkpoints:
- Step 2 — pre-flight greps recorded.
- Step 3 (1/5) — `destructor_compiler.rs` done.
- Step 3 (2/5) — `function_body_compiler.rs` done.
- Step 3 (3/5) — `anonymous_interface_macro.rs` done.
- Step 3 (4/5) — `struct_compiler_core.rs` done.
- Step 3 (5/5) — `function_compiler.rs` done.
- Step 5 — verification greps pass.
- Step 6 — signature-rewrite completion check passes (0 bare stubs in typing/).
- Step 7 — diff self-review.
- Step 8 — hand back.

---

## What "done" looks like

- `cargo check --lib` passes with 0 errors, 0 warnings.
- All 23 target stubs have real signatures inside existing one-fn `impl` blocks. Bodies `panic!()`.
- `drop` method name preserved (no `r#`, no rename).
- `_core`, `_ext`, `_anonymous_interface` suffixes preserved on their respective methods.
- `IFunctionGenerator` param slots replaced with `&()` + TODO comment.
- `IEvaluateFunctionResult` / `EvaluateFunctionSuccess` / `EvaluateFunctionFailure` returned/passed as-is by value.
- `NodeEnvironmentBox` → `&mut NodeEnvironmentBuilder<'s, 't>`; `FunctionEnvironmentBoxT` → `&mut FunctionEnvironmentBuilder<'s, 't>`.
- `coutputs` is `&mut CompilerOutputs<'s, 't>` (conservative default).
- `&'t IInDenizenEnvironmentT<'s, 't>` for env params.
- Scout-side types are `<'s>` only.
- Panic messages read `"Unimplemented: Slab 14 — body migration"`.
- Scala `/* */` blocks unchanged byte-for-byte.
- Only the 5 target files modified.
- **0 bare `(&self)` stubs anywhere in `src/typing/`** — signature-rewrite phase complete.
- Handed back uncommitted.

---

## When you're stuck

- **"`drop` won't compile / conflicts with `Drop` trait"**: it won't. `drop` as a method name is fine. Gotcha 1.
- **"cannot find type `IFunctionGenerator`"**: it exists as a trait at `compiler.rs:53`. But for Slab 13, use `&()` placeholder per Gotcha 3.
- **"can I fill `EvaluateFunctionSuccess` fields since they're obvious from Scala?"**: no. Gotcha 4. Slab 14.
- **"`_ext` / `_core` suffixes look redundant — rename?"**: no. Gotcha 2.
- **"compile error: `NodeEnvironmentBuilder` not in scope"**: add `use crate::typing::env::function_environment_t::NodeEnvironmentBuilder;`. Gotcha 15.
- **"can I port the 20-line `drop` body? it's trivial"**: no. Gotcha 10. Slab 14.
- **"the Rust name `evaluate_templated_function_from_call_for_prototype_ext` is awful — can I rename?"**: no. Gotcha 2. Keep the `_ext`.
- **"`FunctionBannerT` — is it AST or something else?"**: grep `pub struct FunctionBannerT` in `src/typing/ast/`. Likely arena-allocated; use `&'t FunctionBannerT<'s, 't>`.
- **"I want to convert `IFunctionGenerator` to a dispatch-tag enum in Slab 13"**: don't. Out of scope. Slab 14+ makes that decision.

## Where to file questions

- **Design**: Slabs 9-12 handoff docs are the canonical pattern references. This doc is authoritative for Slab-13-specific decisions (`drop` naming, suffix preservation, `IFunctionGenerator` placeholder, `_Phantom` struct preservation).
- **Scala semantics**: each Scala `/* def ... */` block is the spec.
- **Hook rejections**: usually accidental whitespace inside `/* */`.

## Final advice

This is the sixth and **final** signature-rewrite slab. The pattern from Slabs 8-12 carries directly. No novel design decisions required.

Slab-13 wrinkles (all answered above):

1. **`drop` is a valid method name** — no `r#`, no rename. Gotcha 1.
2. **Suffix preservation** — `_core`, `_ext`, `_anonymous_interface` stay. Gotcha 2.
3. **`IFunctionGenerator` stays a trait** — use `&()` placeholder; Slab 14+ converts to dispatch-tag enum. Gotcha 3.
4. **`_Phantom` placeholder types pass through by value** — `IEvaluateFunctionResult`, `EvaluateFunctionSuccess`, `EvaluateFunctionFailure`. Gotcha 4.
5. **`NodeEnvironmentBox` / `FunctionEnvironmentBoxT` translation** — same as Slab 11. Gotcha 5.

After Slab 13:

- All `CompilerOutputs` methods have real sigs (Slab 8).
- All `TemplataCompiler` methods have real sigs (Slabs 9 + 10).
- All `compiler.rs` residuals + expression-layer + solver/resolver + function/citizen/macros have real sigs (Slabs 10-13).
- **The typing pass compiles cleanly with every method bodied by `panic!("Unimplemented: Slab 14 — body migration")`.**
- Signature-rewrite phase complete. ~200 method bodies await.

**Slab 14+** = pure body migration, driven by failing tests. Per `TL-HANDOFF.md`:
> Begin with trivial `CompilerOutputs` one-liners (e.g. `lookup_function` = `self.signature_to_function.get(&PtrKey(signature)).copied()`), then TemplataCompiler id-transforms, then substitution engine, then sub-compiler bodies. `ICompileErrorT` variant filling, the `IBoundArgumentsSource` marker-trait → enum conversion, and the `IPlaceholderSubstituter` trait definition all land inside this phase as body patterns demand them.

Slab 14+ is different-shape work: per-method instead of per-file-family. Work gets test-driven — enable a failing test, migrate only the bodies that test exercises, iterate.

Good luck — and congratulations on closing the signature-rewrite phase.
