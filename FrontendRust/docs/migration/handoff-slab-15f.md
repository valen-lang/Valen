# Slab 15f Handoff — `hardcoding_negative_numbers` is green; next driving test is `simple_local`

## Welcome

Start here:

1. Read `TL.md` (top to bottom — the required-reading list at the top is real, not boilerplate).
2. Read `docs/skills/migration-drive.md` — your operating instructions. **Re-read it after every compaction**, since the TL adds notes there as gotchas surface.
3. Drive on: `compiler_tests.rs::simple_local`. JR's already part-way through it — the LetSE arm of `evaluate_expression` is the active body.

## Where you're picking up

Slab 15f landed three pieces of test-infrastructure scaffolding plus enough body migration to get `hardcoding_negative_numbers` green end-to-end. The slab was mostly TL/architect-level — one big new file (`src/typing/test/traverse.rs`) and two small named-struct-from-anonymous-Scala-class rustifications.

The next test (`simple_local` — `a = 42; return a;`) is the first one with a let-binding, which routes through `evaluate_expression`'s `LetSE` arm (currently a `_ => panic!(...)` catch-all at `expression_compiler.rs:781`). JR is mid-migration on that arm.

## What landed in 15f

### 1. `src/typing/test/traverse.rs` — Rust analog of Scala's `Collector`

`Collector.only(node, partialFn)` in Scala uses runtime reflection (`Product.productIterator`) to walk arbitrary case-class trees. Rust replaces it with hand-enumerated traversal, mirroring the well-established postparsing precedent at `src/postparsing/test/traverse.rs` (1093 lines).

The new file is ~1740 lines. Highlights:

- **`NodeRefT<'s, 't>` enum** with ~95 variants covering all 48 `ReferenceExpressionTE` variants, all 5 `AddressExpressionTE` variants, all 13 struct-payload `ITemplataT` variants, all 6 non-leaf `KindT` variants (`StructTT`, `InterfaceTT`, `StaticSizedArrayTT`, `RuntimeSizedArrayTT`, `KindPlaceholderT`, `OverloadSetT`), top-level definitions (`HinputsT`, `FunctionDefinitionT`, `StructDefinitionT`, `InterfaceDefinitionT`, `EdgeT`, `OverrideT`, `InterfaceEdgeBlueprintT`, `ParameterT`, `InstantiationBoundArgumentsT`), exports/externs, and trait-level emit-only `Name`/`Environment`/`VarName`/`FunctionAttribute`/`CitizenAttribute`/`StructMember`/`LocalVariable` variants.
- **~75 `visit_*` walker functions** that descend through children. Each emits the node at its trait level *and* its concrete variant — tests can match either.
- **5 exported macros** (verbatim copies of postparsing's, with `_snode → _tnode`): `collect_in_tnode!`, `collect_in_tnodes!`, `collect_where_tnode!`, `collect_where_tnodes!`, `collect_only_tnode!`, `collect_only_tnodes!`.
- **Stop-at-trait types** (per the postparsing precedent): the 74 `INameT` variants, `IEnvironmentT`, `IVarNameT`, attribute traits, struct-member trait, local-variable trait. Tests destructure these inside the closure pattern, not as `NodeRefT::*` arms. Same for `FunctionTemplataT` / `StructDefinitionTemplataT` / `InterfaceDefinitionTemplataT` / `ImplDefinitionTemplataT` and `OverloadSetT` (don't descend into env / FunctionA / StructA — different arena, different concerns).

**Test invocation pattern** (vanilla pattern, no `if`/`matches!` guard — AIMITIPX shield enforces this):

```rust
crate::collect_only_tnode!(
    crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
    crate::typing::test::traverse::NodeRefT::ConstantInt(
        crate::typing::ast::expressions::ConstantIntTE {
            value: crate::typing::templata::templata::ITemplataT::Integer(-3),
            ..
        }
    ) => Some(())
);
```

### 2. `src/higher_typing/patterns.rs` — `get_rune_types_from_pattern`

Scala's `PatternSUtils.getRuneTypesFromPattern` ported as a free function (no Rust analog of `object PatternSUtils`). Recurses through `pattern.destructure`, appends `(coord_rune.rune, CoordTemplataType {})` if present, dedups preserving order. Returns `Vec<(IRuneS<'s>, ITemplataType<'s>)>`. Used by the LetSE arm.

### 3. `src/typing/expression/expression_compiler.rs` — `LetExprRuneTypeSolverEnv`

Scala's `new IRuneTypeSolverEnv { ... }` anonymous class at `ExpressionCompiler.scala:959` becomes a named struct + impl block at the end of the Rust file. Closes over `&'a NodeEnvironmentBox<'s, 't>`. Same pattern as `HigherTypingRuneTypeSolverEnv` in `higher_typing_pass.rs:1867` (which collapses 6 anonymous Scala impls into one named struct).

The `lookup` body translates Scala's match arms verbatim:
- `Some(StructDefinition(t))` / `Some(InterfaceDefinition(t))` → `Citizen(CitizenRuneTypeSolverLookupResult { tyype: TemplateTemplataType(...), generic_params })`
- `None` → `Err(CouldntFindType(...))`
- `Some(_x)` (anything else) → **panics**, because it requires an `ITemplataT::tyype()` getter that doesn't exist yet in Rust. Separate scaffolding gap; escalate if your test path hits it.

The other 3 typing-pass `IRuneTypeSolverEnv` sites (`array_compiler.rs:101`, `templata_compiler.rs:1501` factory, `overload_resolver.rs:455`) are **not** migrated yet. When their containing functions get migrated, each gets its own named struct following the same pattern. Don't try to unify — Scala bodies differ (the factory has a `LambdaStructImpreciseNameS` special case the LetSE inline doesn't).

## Likely path through `simple_local`

Test program: `func main() int { a = 42; return a; }`

1. **Run the test first.** `cargo nextest run --manifest-path ./FrontendRust/Cargo.toml > ./tmp/<session>.txt 2>&1`. The currently-failing site is the `_ => panic!(...)` catch-all in `evaluate_expression` at `expression_compiler.rs:781`, which fires on `LetSE` because no arm matches it.

2. **Migrate the LetSE arm.** Quote the Scala verbatim from `ExpressionCompiler.scala:1282-1336` (lines 952-1006 of the Rust audit-trail comment block, depending on offset). Translate line-for-line. The two helpers above (`get_rune_types_from_pattern`, `LetExprRuneTypeSolverEnv`) cover the two NNDX-blocked references; everything else should be already-migrated APIs you can call directly.

3. **`patternCompiler.inferAndTranslatePattern`** is called inside the LetSE arm. If it's panic-stubbed, that's a separate body migration — finish the LetSE structure (skeleton-with-panics in any not-yet-implemented closures) first, then drill in if the test path actually goes through it.

4. **The trailing `Consecutor` / `Return`** — `simple_local`'s body is `let a = 42; return a;` which becomes a `Consecutor` of `[LetSE, ReturnSE]`. Both arms need to be reachable. The `Consecutor` arm is already migrated (as of slab 15e); `LetSE` is your work; `ReturnSE` should be too.

5. **No `Collector.only` assertion in `simple_local`** — it just compiles and exits. So the new test traversal helpers from 15f aren't directly exercised by this test (they were exercised by `hardcoding_negative_numbers`, which stays un-ignored as a regression guard).

## State you should know about

- **`hardcoding_negative_numbers` is green and stays un-ignored** as a regression guard. If it starts failing, that's a real regression you need to fix before continuing on `simple_local`.
- **`simple_program_returning_an_int_explicit` is also green and stays un-ignored** — same regression-guard role.
- **`ITemplataT::tyype()` getter does not exist in Rust** yet. The `Some(_x) => panic!()` arm in `LetExprRuneTypeSolverEnv::lookup` requires it. If your test path hits that panic, escalate — TL writes the getter (it's a per-variant match returning `ITemplataType<'s>`).
- **`lookup_nearest_with_imprecise_name`** is panic-stubbed at `function_environment_t.rs:1079`. The LetSE arm doesn't actually need it on this test path (the test program has no struct/interface lookups in the let), so the panic shouldn't fire. If it does, that's a body migration — escalate.
- **`AtomSP::destructure`** is `Option<&'s [AtomSP<'s>]>` (already arena-allocated). The test pattern `let a = 42` has `destructure: None` — recursion in `get_rune_types_from_pattern` is a no-op for this test.
- **The `_ => panic!(...)` catch-all at `expression_compiler.rs:781`** is your migration site. Replace it with the LetSE arm; leave the catch-all (or keep its panic) for variants you haven't migrated yet. The `Consecutor`-arm precedent above it shows the shape.
- **AIMITIPX rule** applies to test patterns: no `if matches!()` or `if`-guards on the macro patterns. Use vanilla destructure with literal values inline. The test traversal macros support guards (the trailing `if $guard => $body` arm exists), but don't use them.

## The non-negotiables

- **Don't commit.** The architect handles all commits.
- **Don't Scala-edit.** Architect-level only (TL.md §"Editing Scala To Match A Rust Simplification").
- **Don't add novel logic.** TL.md "Guiding Principle" and SPDMX. Verbatim translation; if Rust can't compile a verbatim port, escalate.
- **Don't temp-disable Guardian shields.** Always escalate; the TL/architect issues the disable.
- **Don't edit `traverse.rs` yourself.** NNDX will block, correctly. If a future test needs a `NodeRefT` variant or `visit_*` extension, escalate.
- **Don't try to use `LetExprRuneTypeSolverEnv` outside the LetSE arm.** It's private to `expression_compiler.rs`. The 3 other typing-pass `IRuneTypeSolverEnv` sites need their own structs (TL writes them when their functions get migrated).
- **Escalate with context.** TL.md and migration-drive.md both list what to include — file/line on both Rust and Scala sides, exact error message, TFITCX classifications, options-considered with trade-offs. Quote, don't paraphrase.

Good luck.
