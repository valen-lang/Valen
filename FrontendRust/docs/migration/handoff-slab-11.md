# Handoff: Typing Pass Slab 11 — Expression-Layer Method Signature Rewrite

## Who this is for

You're picking up an in-progress Rust port of a Scala compiler frontend. A senior engineer has been moving through the typing pass one "slab" at a time per `quest.md` §12. Slabs 0-10 are done:

- **Slabs 0-7** — arena substrate, names, types, templatas, envs, AST, `CompilerOutputs` data shape, `HinputsT`/`Compiler` scaffolding.
- **Slab 8** — `CompilerOutputs` method signatures (54 sigs).
- **Slab 9** — `object TemplataCompiler` method signatures (35 sigs).
- **Slab 10** — `compiler.rs` residuals + `local_helper.rs`/`struct_compiler.rs` object orphans + `class TemplataCompiler` tail (~31 sigs).

Slabs 8-10 established the **signature-rewrite pattern**. Slab 11 is the fourth signature-rewrite slab and continues the pattern on the expression evaluation layer:

- **`src/typing/expression/expression_compiler.rs`** — 21 stubs
- **`src/typing/expression/pattern_compiler.rs`** — 12 stubs
- **`src/typing/expression/block_compiler.rs`** — 2 stubs
- **`src/typing/expression/call_compiler.rs`** — 4 stubs

Total: **39 signatures** across 4 files. Budget ~3-4 hours focused (slightly larger than Slab 10 because of expression-layer complexity — many stubs have 8-10 parameter signatures).

**Read these first in this order**, then come back:

1. `FrontendRust/docs/migration/handoff-slab-10.md` — the prior signature-rewrite slab. Covers the same pattern, split-impl-blocks convention, and the "macros are enums, not traits" pre-flight check.
2. `FrontendRust/docs/migration/handoff-slab-9.md` — the canonical translation-rules reference. Gotcha 6 (closure params) and Gotcha 8 (`&mut coutputs` conservative default) are directly reused.
3. `FrontendRust/docs/migration/handoff-slab-4.md` — especially the **builder-freeze pattern** discussion (see "Box stubs deleted"). Slab 11 needs the full picture because `NodeEnvironmentBox` / `FunctionEnvironmentBoxT` appear in nearly every Slab-11 Scala signature, and their Rust translation is a design decision (answered below in Gotcha 1).
4. `TL-HANDOFF.md` at repo root — file-layout standards ("one fn per impl block, multi-line body") + the current slab roadmap.
5. This doc.

You shouldn't need to read the Scala source externally — every Scala `def` sig is already embedded inline in the `/* def ... */` blocks beneath each Rust stub. Those blocks are authoritative.

---

## The big picture: why Slab 11 exists

The expression-layer sub-compilers (`ExpressionCompiler`, `PatternCompiler`, `BlockCompiler`, `CallCompiler` in Scala) host 39 methods that were lifted onto `impl Compiler` in the Phase 2 god-struct refactor as bare `pub fn foo(&self) { panic!(...) }` stubs. Their Scala `/* def ... */` anchors carry the real signatures — your job is to fill in the Rust parameter lists.

No new design decisions beyond what prior slabs already resolved. The main novel pattern in Slab 11 vs. Slabs 8-10 is **`NodeEnvironmentBox` / `FunctionEnvironmentBoxT` handling** — see Gotcha 1. Two other novel patterns: **closures that capture `&mut` state** (Gotcha 5) and **tuple returns** (Gotcha 6).

By the end of Slab 11:

- `cargo check --lib` passes with 0 errors, 0 warnings.
- All 39 target stubs have real `(&self, ...)` signatures inside one-fn `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't { ... }` blocks.
- Bodies remain `panic!("Unimplemented: Slab 14 — body migration");`.
- Scala `/* */` blocks unchanged byte-for-byte.
- Only the 4 target files touched.

**What Slab 11 is NOT:**

- No body migration. Bodies stay `panic!()`. Slab 14+.
- No `IExpressionCompilerDelegate` / `IBlockCompilerDelegate` trait definitions — these traits don't exist on the Rust side and shouldn't be created. Scala `delegate.foo(...)` in the bodies becomes `self.foo(...)` in Rust (god-struct). For Slab 11 (signatures only) you don't have to touch delegate patterns — the bodies stay panic-stubbed.
- No changes to the `NodeEnvironmentBuilder` / `FunctionEnvironmentBuilder` types in `src/typing/env/function_environment_t.rs`. Use them as-is.
- No `infer_compiler.rs` / `overload_resolver.rs` / `impl_compiler.rs` / `reachability.rs` — Slab 12.
- No `function_compiler.rs` / `struct_compiler_core.rs` / etc. — Slab 13.

---

## Stub inventory: the 39 targets

All stubs are currently `pub fn <name>(&self) { panic!("Unimplemented: <name>"); }` with no other params. Your job is to fill in params and return from the Scala `/* */` block directly below.

### File 1: `src/typing/expression/expression_compiler.rs` — 21 stubs

| Line | Stub |
|---|---|
| 138 | `evaluate_and_coerce_to_reference_expressions` |
| 163 | `evaluate_lookup_for_load` |
| 195 | `evaluate_addressible_lookup_for_mutate` |
| 286 | `evaluate_addressible_lookup` |
| 382 | `make_closure_struct_construct_expression` |
| 454 | `evaluate_and_coerce_to_reference_expression` |
| 485 | `coerce_to_reference_expression` |
| 508 | `evaluate_expected_address_expression` |
| 536 | `evaluate_expression` |
| 1616 | `check_array` |
| 1654 | `get_option` |
| 1730 | `get_result` |
| 1825 | `weak_alias` |
| 1852 | `dot_borrow` |
| 1896 | `evaluate_closure` |
| 1947 | `new_global_function_group_expression` |
| 1970 | `evaluate_block_statements` |
| 1992 | `translate_pattern_list` |
| 2015 | `astronomize_lambda` |
| 2093 | `drop_since` |
| 2166 | `resultify_expressions` |

### File 2: `src/typing/expression/pattern_compiler.rs` — 12 stubs

| Line | Stub |
|---|---|
| 50 | `translate_pattern_list_pattern` |
| 89 | `iterate_translate_list_and_maybe_continue` |
| 135 | `infer_and_translate_pattern` |
| 229 | `inner_translate_sub_pattern_and_maybe_continue` |
| 344 | `destructure_owning` |
| 418 | `destructure_non_owning_and_maybe_continue` |
| 454 | `iterate_destructure_non_owning_and_maybe_continue` |
| 542 | `translate_destroy_struct_inner_and_maybe_continue` |
| 612 | `make_lets_for_own_and_maybe_continue` |
| 660 | `load_result_ownership` |
| 679 | `load_from_struct` |
| 729 | `load_from_static_sized_array` |

### File 3: `src/typing/expression/block_compiler.rs` — 2 stubs

| Line | Stub |
|---|---|
| 63 | `evaluate_block` |
| 103 | `evaluate_block_statements_block` |

Note: the Rust name `evaluate_block_statements_block` disambiguates from `evaluate_block_statements` in `expression_compiler.rs:1970` (both Scala `evaluateBlockStatements` — different sub-compilers). **Preserve the Rust name as-is**; it's a god-struct-collision rename that Slab 4 or a prior sweep already applied.

### File 4: `src/typing/expression/call_compiler.rs` — 4 stubs

| Line | Stub |
|---|---|
| 37 | `evaluate_call` |
| 137 | `evaluate_custom_call` |
| 231 | `check_types` |
| 285 | `evaluate_prefix_call` |

---

## Signature translation rules

**Same table as Slabs 8-10** — reread Slab 9 §"Signature translation rules" if rusty. Quick reference of the types you'll see most in Slab 11:

| Scala | Rust |
|---|---|
| `CompilerOutputs` param (mutates in body) | `&mut CompilerOutputs<'s, 't>` — conservative default (Slab 9 Gotcha 8) |
| `CompilerOutputs` param (strictly read-only) | `&CompilerOutputs<'s, 't>` |
| `NodeEnvironmentBox` | `&mut NodeEnvironmentBuilder<'s, 't>` — **see Gotcha 1** |
| `NodeEnvironmentT` (e.g. `startingNenv` param — the snapshot) | `&'t NodeEnvironmentT<'s, 't>` |
| `FunctionEnvironmentBoxT` | `&mut FunctionEnvironmentBuilder<'s, 't>` — **see Gotcha 1** |
| `FunctionEnvironmentT` (frozen) | `&'t FunctionEnvironmentT<'s, 't>` |
| `IInDenizenEnvironmentT` | `&'t IInDenizenEnvironmentT<'s, 't>` (Slab-4 override) |
| `LocationInFunctionEnvironmentT` | `LocationInFunctionEnvironmentT<'s>` by value (Copy? verify — if Vec-backed per TL-HANDOFF §"Known residual items", then `LocationInFunctionEnvironmentT<'s>` by value with the `Vec<i32>` still inside is fine — AASSNCMCX deviation already documented) |
| `LocationInDenizen` | `LocationInDenizen<'s>` by value (`src/postparsing/ast.rs:1168`, Copy verify) |
| `List[RangeS]` / `Vector[RangeS]` | `&[RangeS<'s>]` |
| `RangeS` | `RangeS<'s>` by value |
| `RegionT` | `RegionT` by value (small enum) |
| `IdT[X]` (any phantom) | `IdT<'s, 't>` by value |
| `StrI` | `StrI<'s>` by value |
| `KindT` / `CoordT` / `ITemplataT[X]` | by value (Copy per Slab 3) |
| `ICitizenTT` / `StructTT` / `InterfaceTT` | by value (Copy per Slab 3) |
| `ExpressionT` (the Scala trait — appears as return type in `evaluateLookupForLoad`) | `ExpressionTE<'s, 't>` by value (inline wrapper enum per Slab 5 — `§7.1 naming carve-out: trait ExpressionT → ExpressionTE`) |
| `ReferenceExpressionTE` (param) | `&'t ReferenceExpressionTE<'s, 't>` (arena ref per AASSNCMCX) |
| `ReferenceExpressionTE` (return) | `&'t ReferenceExpressionTE<'s, 't>` (arena-allocated, returned by ref) |
| `AddressExpressionTE` (param / return) | `&'t AddressExpressionTE<'s, 't>` |
| `Option[ExpressionT]` / `Option[ReferenceExpressionTE]` / `Option[AddressExpressionTE]` | `Option<ExpressionTE<'s, 't>>` / `Option<&'t ReferenceExpressionTE<'s, 't>>` / `Option<&'t AddressExpressionTE<'s, 't>>` |
| `Vector[ReferenceExpressionTE]` | `&[&'t ReferenceExpressionTE<'s, 't>]` |
| `IExpressionSE` | `&'s IExpressionSE<'s>` (scout-side, arena ref) |
| `Vector[IExpressionSE]` | `&[&'s IExpressionSE<'s>]` |
| `BlockSE` | `&'s BlockSE<'s>` (scout-side) |
| `AtomSP` | `&'s AtomSP<'s>` (scout-side pattern) |
| `Vector[AtomSP]` | `&[&'s AtomSP<'s>]` |
| `IRulexSR` | `&'s IRulexSR<'s>` (scout-side, arena ref) |
| `Vector[IRulexSR]` | `&[&'s IRulexSR<'s>]` |
| `IRuneS` | `IRuneS<'s>` by value (Copy, scout-side) |
| `Vector[IRuneS]` | `&[IRuneS<'s>]` |
| `IVarNameT` | `IVarNameT<'s, 't>` by value (inline wrapper enum, Copy) |
| `IVarNameS` | `IVarNameS<'s>` by value (Copy, scout-side) |
| `ILocalVariableT` | `&'t ILocalVariableT<'s, 't>` (Slab 4 — arena-ref; the wrapper enum holds `&'t` refs to concrete variables) — actually **double-check** Slab 4 structure: `ILocalVariableT` is likely a thin inline wrapper enum, in which case pass **by value** (Copy). If it holds `&'t` refs internally, the wrapper itself is Copy and passes by value. Default: `ILocalVariableT<'s, 't>` by value. |
| `Vector[ILocalVariableT]` | `&[ILocalVariableT<'s, 't>]` if Copy; else `&[&'t ILocalVariableT<'s, 't>]` |
| `LoadAsP` | `LoadAsP` by value (parser enum, Copy) |
| `Set[CoordT]` (return, e.g. returnsFromExprs) | `HashSet<CoordT<'s, 't>>` (returned owned; read-only vs mutable not applicable to returns) |
| `Set[IVarNameT]` (return) | `HashSet<IVarNameT<'s, 't>>` |
| Closure param `(A, B) => C` | `impl FnOnce(A, B) -> C` — **see Gotcha 5** (default to `FnOnce`; some may need `FnMut`) |
| Tuple return `(A, B)` / `(A, B, C, D)` | `(A, B)` / `(A, B, C, D)` — **see Gotcha 6** for named fields alternative (don't use; pure tuple is fine) |

### Receiver classification

All 39 methods on `&self`. None need `&mut self` — `Compiler<'s, 'ctx, 't>` holds only immutable refs. Mutation threads through `&mut CompilerOutputs` and `&mut NodeEnvironmentBuilder` parameters.

### Derive / lifetime bounds

- Every `impl` block: `<'s, 'ctx, 't>` with `where 's: 't`.
- `pub fn` (not `fn`).
- No new derives.

---

## Shape of a lifted stub

### Example 1: `call_compiler.rs::evaluate_call`

Before (line 37):

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_call(&self) { panic!("Unimplemented: evaluate_call"); }
/*
  private def evaluateCall(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    range: List[RangeS],
    callLocation: LocationInDenizen,
    contextRegion: RegionT,
    callableExpr: ReferenceExpressionTE,
    explicitTemplateArgRulesS: Vector[IRulexSR],
    explicitTemplateArgRunesS: Vector[IRuneS],
    givenArgsExprs2: Vector[ReferenceExpressionTE]):
  (ReferenceExpressionTE) = { ... }
*/
}
```

After:

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_call(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBuilder<'s, 't>,
        life: LocationInFunctionEnvironmentT<'s>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        context_region: RegionT,
        callable_expr: &'t ReferenceExpressionTE<'s, 't>,
        explicit_template_arg_rules_s: &[&'s IRulexSR<'s>],
        explicit_template_arg_runes_s: &[IRuneS<'s>],
        given_args_exprs_2: &[&'t ReferenceExpressionTE<'s, 't>],
    ) -> &'t ReferenceExpressionTE<'s, 't> {
        panic!("Unimplemented: Slab 14 — body migration");
    }
/*
  private def evaluateCall(... same as before ...)
*/
}
```

### Example 2: `pattern_compiler.rs::translate_pattern_list_pattern` (with closure param)

Before (line 50):

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_pattern_list_pattern(&self) {
        panic!("Unimplemented: translate_pattern_list_pattern");
    }
/*
  def translatePatternList(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    patternsA: Vector[AtomSP],
    patternInputsTE: Vector[ReferenceExpressionTE],
    region: RegionT,
    afterPatternsSuccessContinuation: (CompilerOutputs, NodeEnvironmentBox, Vector[ILocalVariableT]) => ReferenceExpressionTE):
  ReferenceExpressionTE = { ... }
*/
}
```

After:

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_pattern_list_pattern(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        nenv: &mut NodeEnvironmentBuilder<'s, 't>,
        life: LocationInFunctionEnvironmentT<'s>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        patterns_a: &[&'s AtomSP<'s>],
        pattern_inputs_te: &[&'t ReferenceExpressionTE<'s, 't>],
        region: RegionT,
        after_patterns_success_continuation: impl FnOnce(
            &mut CompilerOutputs<'s, 't>,
            &mut NodeEnvironmentBuilder<'s, 't>,
            &[ILocalVariableT<'s, 't>],
        ) -> &'t ReferenceExpressionTE<'s, 't>,
    ) -> &'t ReferenceExpressionTE<'s, 't> {
        panic!("Unimplemented: Slab 14 — body migration");
    }
/*
  def translatePatternList(... same as before ...)
*/
}
```

Note the closure signature: `impl FnOnce(&mut CompilerOutputs, &mut NodeEnvironmentBuilder, &[ILocalVariableT]) -> &'t ReferenceExpressionTE`. See Gotcha 5 for why `FnOnce` and not `Fn`/`FnMut`.

### Example 3: `block_compiler.rs::evaluate_block` (with tuple return)

Before (line 63):

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_block(&self) {
        panic!("Unimplemented: evaluate_block");
    }
/*
  def evaluateBlock(
    parentFate: FunctionEnvironmentBoxT,
    coutputs: CompilerOutputs,
    life: LocationInFunctionEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    region: RegionT,
    block1: BlockSE):
  (BlockTE, Set[IVarNameT], Set[IVarNameT], Set[CoordT]) = { ... }
*/
}
```

After:

```rust
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_block(
        &self,
        parent_fate: &mut FunctionEnvironmentBuilder<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        life: LocationInFunctionEnvironmentT<'s>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        region: RegionT,
        block_1: &'s BlockSE<'s>,
    ) -> (
        &'t BlockTE<'s, 't>,
        HashSet<IVarNameT<'s, 't>>,
        HashSet<IVarNameT<'s, 't>>,
        HashSet<CoordT<'s, 't>>,
    ) {
        panic!("Unimplemented: Slab 14 — body migration");
    }
/*
  def evaluateBlock(... same as before ...)
*/
}
```

---

## Gotchas

### Gotcha 1: `NodeEnvironmentBox` → `&mut NodeEnvironmentBuilder<'s, 't>`; `FunctionEnvironmentBoxT` → `&mut FunctionEnvironmentBuilder<'s, 't>`

This is the central design question of Slab 11, and the answer is **already decided** by Slab 4's builder-freeze pattern.

**Scala:** `NodeEnvironmentBox` is a mutable box wrapper around `NodeEnvironmentT`. Callers mutate via `nenv.addDeclaredLocal(...)`, `nenv.unstackify(...)`, or read via `nenv.snapshot` / `nenv.getVariable(...)`. Same for `FunctionEnvironmentBoxT`.

**Rust:** Slab 4 deleted both box types. The builder-freeze pattern splits mutation from reading:

- **Mutation** happens in a stack-local `NodeEnvironmentBuilder<'s, 't>` / `FunctionEnvironmentBuilder<'s, 't>` (both defined in `src/typing/env/function_environment_t.rs` at lines 1181 and 1220 respectively).
- `build_in(interner)` freezes the builder into a `&'t NodeEnvironmentT<'s, 't>` / `&'t FunctionEnvironmentT<'s, 't>`.

**Signature translation:**

- `nenv: NodeEnvironmentBox` → `nenv: &mut NodeEnvironmentBuilder<'s, 't>` (default — most Scala uses mutate)
- `nenv: NodeEnvironmentT` or `startingNenv: NodeEnvironmentT` (the read-only snapshot) → `nenv: &'t NodeEnvironmentT<'s, 't>`
- `parentFate: FunctionEnvironmentBoxT` → `parent_fate: &mut FunctionEnvironmentBuilder<'s, 't>`

**Conservative default**: if unsure whether the Scala body mutates, use `&mut NodeEnvironmentBuilder`. This matches Slab 9 Gotcha 8 ("when in doubt, `&mut`").

**Return type note**: methods that return a fresh `NodeEnvironmentBox` (e.g. `makeChildNodeEnvironment`) — if you see any in the Slab-11 stub returns, translate as `NodeEnvironmentBuilder<'s, 't>` by value (owned, stack-local, caller freezes with `.build_in(...)`). None of the 39 targets appear to return a box type in their `/* */` signatures, but double-check each.

### Gotcha 2: `evaluateBlockStatements` appears twice — don't unify

`evaluate_block_statements` exists at:
- `expression_compiler.rs:1970` (Scala's `ExpressionCompiler.evaluateBlockStatements`)
- `block_compiler.rs:103` (Scala's `BlockCompiler.evaluateBlockStatements` — **already renamed in Rust to `evaluate_block_statements_block`** to avoid collision)

Both get lifted separately. Don't try to unify them. Preserve the `_block` suffix on the block_compiler one — it's a prior-work god-struct-collision rename (same spirit as Slab 10's `struct_compiler_get_mutability`).

### Gotcha 3: `ExpressionT` (the Scala trait return type) → `ExpressionTE<'s, 't>` by value

Scala's `evaluateLookupForLoad` returns `Option[ExpressionT]`. `ExpressionT` is the Scala trait; Rust's equivalent is the 2-variant `ExpressionTE<'s, 't>` wrapper enum from Slab 5 (§7.1 naming carve-out). It's Copy? No — Slab 5 Gotcha 2 says expression derives drop `Eq/Hash` because of `f64`, but they remain `PartialEq, Debug`. The wrapper enums are 2-variant holding `&'t` refs, so they're effectively Copy-sized but **check** whether `ExpressionTE` derives Copy before passing by value. If not, pass by value anyway (the compiler will tell you if it needs Clone) — `ExpressionTE` is small enough that Copy should be fine to add, but **don't add Copy in Slab 11**. If it doesn't compile, pass `&'t ExpressionTE<'s, 't>` by reference.

Practical Slab-11 recommendation: **try `Option<ExpressionTE<'s, 't>>` first; fall back to `Option<&'t ExpressionTE<'s, 't>>` if the compiler rejects**.

### Gotcha 4: `BlockTE` return from `evaluateBlock` — probably `&'t BlockTE<'s, 't>`

`BlockTE` is an expression AST node (Slab 5). Returns by arena ref: `&'t BlockTE<'s, 't>`. Grep if unsure:

```
Grep pattern: "pub struct BlockTE"
```

Should land in `src/typing/ast/expressions.rs`. It's Copy-excluded per Slab 5 derive deviation. Return by `&'t`.

### Gotcha 5: closure params use `impl FnOnce`, not `Fn` / `FnMut`

Several Slab-11 Scala signatures carry closure params:

- `afterPatternsSuccessContinuation: (CompilerOutputs, NodeEnvironmentBox, Vector[ILocalVariableT]) => ReferenceExpressionTE` (in `translatePatternList`, `iterateTranslateListAndMaybeContinue`, and several destructure-and-continue methods).
- Various `continuation: ... => ReferenceExpressionTE` closures in the pattern-compiler iterate/destructure family.

**Translation**: `impl FnOnce(<translated param types>) -> <return>`.

**Why `FnOnce`, not `Fn` / `FnMut`**: Rust closures that capture `&mut CompilerOutputs` or `&mut NodeEnvironmentBuilder` through their enclosing scope can't be `Fn` (immutable call) or readily `FnMut` (mutable call with captured `&mut` in the environment). `FnOnce` sidesteps the re-borrow issue — the closure is consumed after one call, which matches how continuations actually work in the Scala bodies (called exactly once after successful pattern translation).

Slab 9 Gotcha 6 used `impl Fn(IRuneS<'s>) -> bool` because the closure captured nothing mutable. Slab 11 closures capture nontrivial mutable state, so `FnOnce` is the right default.

**Exception**: if the body clearly calls the closure multiple times (rare — check the Scala body), use `impl FnMut(...)`. Check inside the `/* */` block.

**Don't use `Box<dyn FnOnce>` or `&dyn FnOnce`** — `impl FnOnce` is cheaper and idiomatic.

### Gotcha 6: tuple returns stay tuples, not named structs

Several Slab-11 methods return tuples like `(BlockTE, Set[IVarNameT], Set[IVarNameT], Set[CoordT])` or `(ReferenceExpressionTE, Set[CoordT])`. **Keep as Rust tuples**:

```rust
) -> (&'t BlockTE<'s, 't>, HashSet<IVarNameT<'s, 't>>, HashSet<IVarNameT<'s, 't>>, HashSet<CoordT<'s, 't>>)
```

**Don't** promote to a named struct — that's a Rust-idiomatic refactor, not Scala parity. Scala code uses positional destructure (`val (block2, unstackified, restackified, returns) = evaluateBlock(...)`); Rust does the same with tuple destructure.

If a tuple return gets awkwardly wide (>4 fields), flag it in the handback but don't refactor.

### Gotcha 7: `ILocalVariableT<'s, 't>` — by value, not by ref

Per Slab 4, `ILocalVariableT<'s, 't>` is an inline wrapper enum whose variants hold `&'t` refs to concrete variable structs. The wrapper itself is small (tag + pointer) and should be Copy.

**Verify**: grep `pub enum ILocalVariableT` at `src/typing/env/function_environment_t.rs` and check for `#[derive(Copy, Clone, ...)]`.

Translation:
- `ILocalVariableT` (param or return) → `ILocalVariableT<'s, 't>` by value
- `Vector[ILocalVariableT]` (slice) → `&[ILocalVariableT<'s, 't>]`

If the wrapper isn't Copy for some reason, fall back to `&'t ILocalVariableT<'s, 't>`.

### Gotcha 8: `LocationInFunctionEnvironmentT` — by value with the AASSNCMCX deviation

Per TL-HANDOFF "Known residual items": `LocationInFunctionEnvironmentT.path: Vec<i32>` violates AASSNCMCX (heap `Vec` inside an `'t`-arena-allocated conceptual type) and is flagged for future cleanup. Don't fix in Slab 11 — pass `LocationInFunctionEnvironmentT<'s>` by value. The Vec makes it non-Copy; you may need `.clone()` at body-migration time but for signatures this doesn't matter.

If passing by value causes a lifetime issue, fall back to `&LocationInFunctionEnvironmentT<'s>` (no `'t` — the inner Vec is heap, not arena).

### Gotcha 9: `IRulexSR` and `AtomSP` are scout-side `'s`, not typing `'t`

Scout-pass types (`IRulexSR`, `IRuneS`, `IImpreciseNameS`, `AtomSP`, `BlockSE`, `IExpressionSE`, `LocalS`, `GenericParameterS`, `CitizenAttributeS`, etc.) all carry `<'s>`, not `<'s, 't>`. When passed by reference, they borrow from the scout arena `'s`:

- `&'s AtomSP<'s>` (single pattern)
- `&[&'s AtomSP<'s>]` (slice of patterns)
- `&'s IRulexSR<'s>` (single rule)
- `&[&'s IRulexSR<'s>]` (slice of rules)
- `&'s BlockSE<'s>` (block scout node)
- `&'s IExpressionSE<'s>` (expression scout node)

Don't accidentally write `AtomSP<'s, 't>` — that's a compile error. Always single-lifetime `'s`.

### Gotcha 10: `check_types` method on `call_compiler.rs` — don't confuse with a Rust trait impl

Rust has the convention that `impl Foo for Bar` provides `bar.check_types(...)` via trait dispatch. Slab-11's `check_types` is a plain method on `Compiler` — a Scala instance method that happens to share its name with a hypothetical trait. No special handling needed; translate like any other method.

### Gotcha 11: `evaluate_custom_call` returns `FunctionCallTE`, not `ReferenceExpressionTE`

Most expression-compiler methods return `ReferenceExpressionTE`. `evaluate_custom_call` returns the narrower `FunctionCallTE` (a specific payload struct per Slab 5). Return `&'t FunctionCallTE<'s, 't>`. Don't accidentally widen to `&'t ReferenceExpressionTE`.

### Gotcha 12: bodies stay `panic!()` — resist the temptation (restatement of prior Slab Gotchas)

Several Slab-11 methods have trivial Scala bodies:

- `evaluate_and_coerce_to_reference_expressions` — 7 lines (a `.map` + unzip).
- `resultify_expressions` — likely short.
- `load_result_ownership` — likely a small match.

**Don't port them.** Slab 14+ migrates bodies together.

### Gotcha 13: panic message uses **Slab 14 — body migration**

Match Slab 10's convention: `panic!("Unimplemented: Slab 14 — body migration");`. Don't re-touch the ~120 stale Slab-10-labeled panic messages elsewhere.

### Gotcha 14: one-fn impl blocks per TL-HANDOFF convention

Each method gets its own `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't { pub fn foo(...) { panic!(...) } }` block adjacent to its Scala `/* def ... */` anchor. The existing file structure already has 39 one-fn `impl` wrappers — you're filling in the `(&self)` with real params, not creating new impls.

### Gotcha 15: Scala `/* */` blocks frozen — pre-commit hook enforces

Same as every prior slab. `.claude/hooks/check-scala-comments` rejects byte-level edits to `/* */` blocks.

### Gotcha 16: no downstream breakage expected

All 39 stubs are `panic!()`. No caller exercises them at runtime. Lifting changes signatures but preserves panic bodies. Downstream callers that construct Slab-11 types are all panic-stubbed themselves.

---

## Step-by-step plan

### Step 1: Confirm starting point

```bash
cargo check --lib --manifest-path /Volumes/V/Sylvan/FrontendRust/Cargo.toml > /tmp/sylvan-slab-11.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-11.txt
```

Must print `0`.

### Step 2: Pre-flight type location refresher

All types Slab-11 signatures reference should exist. Spot-check:

- `NodeEnvironmentBuilder<'s, 't>`: `src/typing/env/function_environment_t.rs:1220`
- `FunctionEnvironmentBuilder<'s, 't>`: `src/typing/env/function_environment_t.rs:1181`
- `NodeEnvironmentT<'s, 't>`: `src/typing/env/function_environment_t.rs:207`
- `FunctionEnvironmentT<'s, 't>`: `src/typing/env/function_environment_t.rs` (search)
- `ILocalVariableT<'s, 't>`: `src/typing/env/function_environment_t.rs` (search — verify Copy)
- `IVarNameT<'s, 't>` / `IVarNameS<'s>`: `src/typing/names/names.rs`
- `ExpressionTE<'s, 't>` / `ReferenceExpressionTE<'s, 't>` / `AddressExpressionTE<'s, 't>` / `BlockTE<'s, 't>` / `FunctionCallTE<'s, 't>`: `src/typing/ast/expressions.rs`
- `IExpressionSE<'s>` / `BlockSE<'s>` / `AtomSP<'s>` / `IRulexSR<'s>` / `IRuneS<'s>` / `IVarNameS<'s>`: `src/postparsing/`
- `LoadAsP`: `src/parsing/ast.rs` (search)
- `LocationInFunctionEnvironmentT<'s>`: `src/typing/ast/ast.rs`
- `LocationInDenizen<'s>`: `src/postparsing/ast.rs:1168`

Use Grep to locate anything that doesn't match — everything should exist. If a grep fails unexpectedly, stop and flag before coding.

### Step 3: Lift file-by-file, smallest → largest

Recommended order:

1. **`block_compiler.rs` (2 stubs)** — warm-up with `FunctionEnvironmentBuilder` + tuple return + `BlockSE`. 20-30 min.
2. **`call_compiler.rs` (4 stubs)** — solid mid-range. Mostly `NodeEnvironmentBuilder` + `ReferenceExpressionTE`. 30-45 min.
3. **`pattern_compiler.rs` (12 stubs)** — heavy closure param usage. 60-80 min.
4. **`expression_compiler.rs` (21 stubs)** — largest chunk; varied signatures. 90-120 min.

Follow the order above for maximum momentum. If stuck on a particular closure signature or return type, skip it and come back after adjacent signatures clarify the pattern.

### Step 4: Incremental verification

After each file:

```bash
cargo check --lib --manifest-path /Volumes/V/Sylvan/FrontendRust/Cargo.toml > /tmp/sylvan-slab-11.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-11.txt
```

Fix errors before moving on. Common Slab-11 mistakes:

- `NodeEnvironmentBox` → wrong. Use `NodeEnvironmentBuilder`.
- `NodeEnvironmentT` without `&'t` — always `&'t NodeEnvironmentT<'s, 't>`.
- Forgetting `&mut` on `NodeEnvironmentBuilder` / `FunctionEnvironmentBuilder`.
- `IBlockCompilerDelegate` / `IExpressionCompilerDelegate` as a trait param — don't exist; Scala `delegate.foo(...)` becomes `self.foo(...)` in the body, not in the signature.
- `&'s NodeEnvironmentT` instead of `&'t` — envs live in `'t`, not `'s`.
- Missing `&'s` on scout-side types (`AtomSP`, `IRulexSR`, etc.).
- Closure passed as `impl Fn` when it captures `&mut` — use `FnOnce` or `FnMut`.
- Treating `ExpressionT` as if it doesn't exist on the Rust side — it's `ExpressionTE<'s, 't>` (with the `E` suffix).

### Step 5: Final verification

```bash
cargo check --lib --manifest-path /Volumes/V/Sylvan/FrontendRust/Cargo.toml > /tmp/sylvan-slab-11.txt 2>&1
grep -c "^error" /tmp/sylvan-slab-11.txt       # must be 0
grep -c "^warning" /tmp/sylvan-slab-11.txt     # must be 0
tail -3 /tmp/sylvan-slab-11.txt                 # must show "Finished"
```

Sanity greps:

```
Grep pattern: "pub fn .*\(&self\) \{" path: FrontendRust/src/typing/expression
# ^ should match 0 lines (no more bare (&self) stubs in any of the 4 expression/ files)

Grep pattern: "NodeEnvironmentBox" path: FrontendRust/src/typing/expression
# ^ should match 0 lines (the Box type is deleted; Scala /* */ blocks still reference it but Rust code shouldn't)

Grep pattern: "Slab 14 — body migration" path: FrontendRust/src/typing/expression
# ^ should be exactly 39 (one per Slab-11 lift)
```

### Step 6: Diff self-review

```bash
git diff FrontendRust/src/typing/expression/ | head -500
git diff --stat
```

Confirm:
- Only stub rewrites (no new impls; every impl header already exists).
- No edits inside Scala `/* */` blocks.
- No other files touched (no `compiler.rs`, `env/`, etc.).
- Every new panic message uses "Slab 14 — body migration".

### Step 7: Hand off

**Never commit.** Hand back uncommitted; the human tags `slab-11-complete`.

Work-order checkpoints:
- Step 2 — pre-flight greps recorded.
- Step 3 (1/4) — `block_compiler.rs` done.
- Step 3 (2/4) — `call_compiler.rs` done.
- Step 3 (3/4) — `pattern_compiler.rs` done.
- Step 3 (4/4) — `expression_compiler.rs` done.
- Step 5 — verification greps pass.
- Step 6 — diff self-review.
- Step 7 — hand back.

---

## What "done" looks like

- `cargo check --lib` passes with 0 errors, 0 warnings.
- All 39 target stubs have real signatures inside existing one-fn `impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> where 's: 't { pub fn ... (&self, ...) { panic!(...) } }` blocks.
- `NodeEnvironmentBox` params → `&mut NodeEnvironmentBuilder<'s, 't>`; `FunctionEnvironmentBoxT` → `&mut FunctionEnvironmentBuilder<'s, 't>`.
- Read-only env snapshots (`startingNenv: NodeEnvironmentT`) → `&'t NodeEnvironmentT<'s, 't>`.
- `coutputs` is `&mut CompilerOutputs<'s, 't>` (conservative default).
- Every scout-side type is `&'s X<'s>` (for refs) or `X<'s>` by value (for Copy like `IRuneS`).
- Every typing-side definition/AST node is `&'t X<'s, 't>` (per AASSNCMCX).
- `IdT<'s, 't>` / `KindT<'s, 't>` / `CoordT<'s, 't>` / `ITemplataT<'s, 't>` / `IVarNameT<'s, 't>` / `ICitizenTT<'s, 't>` / `ILocalVariableT<'s, 't>` passed by value.
- Closure params are `impl FnOnce(...)` (or `impl FnMut(...)` if the body clearly re-invokes).
- Tuple returns stay tuples.
- `ExpressionT` → `ExpressionTE<'s, 't>` (Slab 5 naming carve-out).
- Panic messages read `"Unimplemented: Slab 14 — body migration"`.
- Scala `/* */` blocks unchanged byte-for-byte.
- Only the 4 Slab-11 target files modified.
- Handed back uncommitted.

---

## When you're stuck

- **"cannot find type `NodeEnvironmentBox`"**: Gotcha 1. It's deleted. Use `&mut NodeEnvironmentBuilder<'s, 't>`.
- **"cannot find type `FunctionEnvironmentBoxT`"**: Gotcha 1. Use `&mut FunctionEnvironmentBuilder<'s, 't>`.
- **"cannot find trait `IBlockCompilerDelegate` / `IExpressionCompilerDelegate`"**: these traits don't exist on the Rust side. Scala `delegate.foo(...)` calls are body-level concerns (Slab 14+), not signatures. You shouldn't need to reference the delegate trait name at all.
- **"cannot find type `ExpressionT`"**: it's `ExpressionTE<'s, 't>` (the `E` suffix was added in Slab 5 per §7.1 naming carve-out).
- **"`FnOnce` doesn't compile; expects `FnMut`"**: the body re-invokes the closure. Switch to `impl FnMut(...)`.
- **"`FnMut` doesn't compile either; closure captures `&mut` and moves it"**: reread Gotcha 5. `impl FnOnce` is usually the right answer even when the body looks re-entrant — check the Scala carefully; many `continuation` patterns call once.
- **"can't pass `ILocalVariableT<'s, 't>` by value"**: it's not Copy. Fall back to `&'t ILocalVariableT<'s, 't>`.
- **"can't pass `ExpressionTE<'s, 't>` by value"**: it's not Copy (Slab 5 deviation). Fall back to `&'t ExpressionTE<'s, 't>`.
- **"`'s` may not live long enough"**: missing `where 's: 't` on the `impl` header (every impl block already has it — if you accidentally introduced a new impl without the bound, add it).
- **"too many arguments to function"**: rare since Slab 11 doesn't have `interner`/`keywords` dropping; most methods have 8-10 params. Just be precise.
- **"I want to port `resultify_expressions` because it's 5 lines"**: don't. Gotcha 12. Slab 14 ports bodies.
- **"I want to touch `infer_compiler.rs` because it has similar stubs"**: don't. Slab 12.
- **"the Rust name `evaluate_block_statements_block` looks ugly — should I rename?"**: Gotcha 2. Leave it.
- **"should I unify the two `evaluate_block_statements` methods?"**: Gotcha 2. No.
- **"`ExpressionCompiler` has 2194 lines and I'm losing the plot"**: it's the largest file in the slab. Lift in file order top-to-bottom. Use Find to jump between `pub fn` stubs. The impl-header-and-Scala-block structure repeats 21 times.

## Where to file questions

- **Design**: `FrontendRust/docs/migration/handoff-slab-9.md` is the spec for the pattern. If Slab 9 and this doc disagree, Slab 9 wins on general rules. File-specific details here (NodeEnvironmentBox translation, expression-layer ambient types, closure patterns) are authoritative for Slab 11.
- **Scala semantics**: each Scala `/* def ... */` block beneath a stub is the spec.
- **Hook rejections**: read the hook's diff output. Usually accidental whitespace inside `/* */`.

## Final advice

This is the fourth signature-rewrite slab. The pattern from Slabs 8-10 carries directly. The novel Slab-11 patterns are:

1. **`NodeEnvironmentBox` / `FunctionEnvironmentBoxT` translation** — already decided (Gotcha 1). Use `&mut NodeEnvironmentBuilder` / `&mut FunctionEnvironmentBuilder`.
2. **Closure params** — use `impl FnOnce(...)` with `&mut` captured types flowing through as explicit params (Gotcha 5).
3. **Tuple returns** — stay tuples, don't promote to structs (Gotcha 6).
4. **Expression AST return types** — `&'t ReferenceExpressionTE<'s, 't>`, `&'t AddressExpressionTE<'s, 't>`, `&'t BlockTE<'s, 't>`, `&'t FunctionCallTE<'s, 't>`. All arena refs per AASSNCMCX (Slab 5).

After Slab 11, the typing pass has:
- All `CompilerOutputs` methods with real sigs (Slab 8).
- All `TemplataCompiler` (object + class) methods with real sigs (Slabs 9 + 10).
- All `compiler.rs` residuals + `local_helper.rs` / `struct_compiler.rs` orphans (Slab 10).
- All expression-layer sigs (Slab 11).
- ~58 remaining sub-compiler methods still partial (Slabs 12-13).

**Slab 12** = solver + resolver sigs (~35 across `infer_compiler.rs` / `overload_resolver.rs` / `impl_compiler.rs` / `reachability.rs`). The `IInfererDelegate` vestigial trait is deleted in Slab 12.
**Slab 13** = function/citizen/macros sigs (~23 across `function_compiler.rs` / `function_body_compiler.rs` / `destructor_compiler.rs` / `struct_compiler_core.rs` / `anonymous_interface_macro.rs`).

After Slab 13, the signature-rewrite phase is complete. Slab 14+ is pure body migration, driven by tests.

Good luck.
