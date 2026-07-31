# Plan document

Source: `/Users/verdagon/.claude/plans/magical-brewing-horizon.md`
Session: 6950c730-dd2e-4200-bbaa-ff7414a08ad5

---

# Refactor: ParameterS becomes pattern-free; destructures + name binding hoist to body-head LetSE

## Context

Vale's postparser currently conflates signature and body concerns on `FunctionS`:

1. **Destructure conflation.** When a param uses destructure syntax like `func foo([a Int, b Bool] &Tup2<Int, Bool>)`, the destructure sub-atoms' rules land in `FunctionS.rules` alongside genuine signature rules. Downstream consumers doing signature-only work (overload resolution, anonymous-interface macro synthesis) walk destructure-only rules that shouldn't affect them. `func foo([a,b] &Tup2)` and `func foo(t &Tup2) { let [a,b] = t; ... }` are equivalent from a caller's perspective, but only the second form scopes destructure rules correctly.

2. **Signature shape opacity.** For a param like `&Bork<&Spork>`, the outer `&` and the inner `Bork<&Spork>` are conceptually distinct — the outer is a shape wrap, the inner is a named type. The anonymous-interface macro needs to mirror a param's outer shape onto a forwarder without unfolding the named type. Currently rules for both concerns share one flat pool, requiring a scan-and-detect walk.

3. **Pattern-in-signature confusion.** `ParameterS.pattern` mixes a param's TYPE (signature: what callers see) with its NAME BINDING and DESTRUCTURE (body: how the callee unpacks the arg). The user wrote `func foo(x Int)` — the caller cares that param 0 is `Int`; `x` is only a body concern (local-variable name for reference from body expressions).

**Intended outcome:** `ParameterS` becomes purely signature-side — it carries the runes and rules that describe the type, and nothing else. Every parameter's name binding and any destructure become a synthesized `LetSE` at the body head. `FunctionS.rules` shrinks to true function-level rules only (generic-param bounds, cross-signature equalities). `LetSE` is already the correct precedent for pattern-scoped rules (`LetSE.rules: &[IRulexSR]`); this plan applies that pattern uniformly.

The current test coverage for parameter destructures is **zero** at every layer (parsing, postparse, typing). Existing parser-level destructure tests use `compile_pattern_expect` which bypasses `parse_parameter`. This plan closes those gaps as it lands the refactor.

## End-state data shape

```rust
pub struct ParameterS<'s> {
    pub range: RangeS<'s>,
    pub virtuality: Option<AbstractSP<'s>>,
    pub pre_checked: bool,
    pub name: IVarNameS<'s>,                    // ALWAYS DesugaredParamName(loc). ABI slot identifier only.
    pub full_kind_rune: RuneUsage<'s>,          // rune for the whole type (outer wraps + named type)
    pub inner_kind_rune: RuneUsage<'s>,         // rune for the named-type root (past outer wraps).
                                                // Equal to full_kind_rune when outer_shape_rules is empty.
    pub outer_shape_rules: &'s [IRulexSR<'s>],  // only BorrowRef/HeapOwnRef/ShareRef/WeakRef, chaining from full to inner
    pub named_type_rules: &'s [IRulexSR<'s>],   // Lookup/Call/etc. describing the named type
    pub rune_to_explicit_type: &'s [(IRuneS<'s>, ITemplataType<'s>)],
    _sealed: (),
}
```

**Removed:** `pattern: AtomSP` field. Body-side concerns (name binding, destructure structure, sub-atom type annotations) move to a synthesized `LetSE` at body head.

```rust
pub enum IVarNameS<'s> {
    // ... existing variants ...
    DesugaredParamName(CodeLocationS<'s>),   // NEW: synthetic ABI-slot identifier
}
```

For every param at postparse, a `LetSE { pattern: <user's AtomSP>, expr: LocalLoadSE(param.name), rules: <destructure sub-atom rules> }` is prepended to the body's `BlockSE.expr` (wrapping in ConsecutorSE as needed). This is unconditional — even `func foo(x Int)` gets `let x = _p0;` at the top. Uniform desugar keeps invariants simple.

**Exception:** extern/abstract/generated bodies have no `BlockSE`. For these:
- If the user's pattern is a bare capture (`x Int`), no let is needed (the name is documentation only). `ParameterS.name = DesugaredParamName(loc)` — the user's `x` is dropped as unused.
- If the user's pattern has a destructure, hard error: `ICompileErrorS::ParamDestructureRequiresBody`.

`FunctionS.rules` now holds only function-level rules (generic-param bounds, cross-signature equalities). Return-position rules stay wherever they live today (deferred to a follow-up plan).

## Invariants worth codifying (via debug_asserts in `ParameterS::new`)

1. `outer_shape_rules` contains ONLY `BorrowRefSR`, `HeapOwnRefSR`, `ShareRefSR`, `WeakRefSR` variants. No other kinds.
2. If `outer_shape_rules` is empty, `full_kind_rune == inner_kind_rune`.
3. Otherwise, `outer_shape_rules` forms a chain: the first rule's `result_rune == full_kind_rune`; the last rule's `inner_rune == inner_kind_rune`; each rule's `inner_rune` matches the next rule's `result_rune`.
4. `named_type_rules` may contain BorrowRef/etc. rules inside template args, but its outermost rule's `result_rune == inner_kind_rune`.
5. `name` is always `IVarNameS::DesugaredParamName(_)`. User names never land on `ParameterS`.

## Design decisions locked in

- **Uniform desugar**: every param gets a synthesized LetSE at body head, regardless of destructure. Simpler than special-casing "just names vs destructures." Perf cost is trivial (one indirection at runtime, likely optimized away by later passes).
- **Where desugaring lives**: postparse-arena, inside `scout_function` after `scout_body` returns. Not parser-arena.
- **Rule bucketing**: per-param fresh `Vec<IRulexSR>` locals, drained into ParameterS. `translate_pattern` and `translate_templex` gain builder params.
- **Three-builder split in `translate_templex`/`translate_pattern`**: outer_shape_builder / named_type_builder / destructure_builder. At depth 0 in `translate_pattern`, `translate_templex` uses outer_shape_builder while walking outer refs and switches to named_type_builder on first non-ref node. `full_kind_rune` = the outermost outer-shape rule's result_rune (or the named-type-root rune if outer_shape is empty). `inner_kind_rune` = the named-type-root rune. Destructure sub-atom recursion uses destructure_builder for all its rules.
- **Extern/abstract/generated + bare param name**: allowed. `ParameterS.name = DesugaredParamName(loc)`; user's identifier discarded.
- **Extern/abstract/generated + destructure**: hard error at postparse. New `ICompileErrorS::ParamDestructureRequiresBody`.
- **Nested Call substitution (`&Container<Interface>`)**: NOT handled by this plan. `inner_kind_rune` points at the named-type root (the Call's result_rune), so substitution swaps the whole `Container<Interface>`. If a use case for deeper substitution emerges, address separately.

## Critical files

- `FrontendRust/src/postparsing/ast.rs` — `ParameterS` shape (remove `pattern`, add runes + rule slices). Update `ParameterS::new` signature and invariant asserts.
- `FrontendRust/src/postparsing/names.rs` — add `IVarNameS::DesugaredParamName` variant. Update matching `IVarNameValS` + interner arms.
- `FrontendRust/src/postparsing/function_scout.rs` — the param loop (lines ~361-450) becomes the routing hub. Post-body-scout: LetSE synthesis + block re-wrap.
- `FrontendRust/src/postparsing/patterns/pattern_scout.rs` — `translate_pattern` gains split builder params + returns the full_kind_rune and inner_kind_rune separately.
- `FrontendRust/src/postparsing/rules/templex_scout.rs` — `translate_templex` gains a caller-passed builder param; caller manages outer/inner boundary.
- `FrontendRust/src/postparsing/post_parser.rs` — new `ICompileErrorS::ParamDestructureRequiresBody` variant.
- Downstream typing consumers of `ParameterS.pattern.kind_rune` — rename to `ParameterS.full_kind_rune` (mechanical sweep).
- `FrontendRust/src/postparsing/variable_uses.rs` — parameters no longer contribute local declarations directly; all body locals come from LetSE scouting. May simplify substantially.

## Reused mechanisms

- **LetSE precedent** (`postparsing/expressions.rs:12-17`) — already carries per-let rules correctly. This plan leverages it verbatim.
- **Postparse ConsecutorSE re-wrap** (`function_scout.rs:1010-1016`) — precedent for allocating a fresh block wrapper after `scout_body` returns.
- **Synthetic-name minting** (`postparsing/names.rs:247-258`) — range-keyed variant precedent; `DesugaredParamName(CodeLocationS)` fits naturally.
- **Typing-side pattern_compiler** (`typing/expression/pattern_compiler.rs`) — already handles destructures from LetSE. All body-side unpack logic is reused as-is.

## Testing philosophy

Integration-style tests per @DBAPIZ — exercise real code paths through public APIs. Postparse assertions on `FunctionS` / `ParameterS` / `LetSE` shapes are integration tests when they use `PostParser::post_parse_program` (the public entry) rather than internal helpers.

Test-suite scoping during early slices: while the typing pass has ongoing session-level cascade errors, tests are filtered to parsing/postparse layers for slices 1-6. Slice 7 escalates to full suite (typing end-to-end fixtures need it). Report to the user before every RFIGA I substep: "Tests correctly failing at expected layer; proceeding with implementation."

## RFIGA plan

Each slice tests ONE behavior change. Tests written before implementation, per the tdd skill (docs/skills/tdd.md).

### Slice 1 — Parser accepts and represents param destructures

Locks the parser-level contract before touching semantics. Confirms `func foo([a, b] &Tup2<Int, Bool>)`, `func foo([a, [b, c]] T)`, `func foo([a Int, b Bool] T)`, `func foo([_, b] T)`, and `func foo([] T)` all parse into a `ParameterP` with a populated `pattern.destructure`.

- **R**: In `FrontendRust/src/parsing/tests/functions/function_tests.rs`, add five tests using `test_parse_function` (or the local convention). Each decodes one source string and asserts `ParameterP.pattern.destructure` shape.
- **F**: `cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing::tests::functions -- param_destructure`. Report state to user.
- **I**: If failing, minimal parser fix. If passing, no code change (slice locks contract).
- **G**: Re-run tests; expect green.
- **A**: Run parsing + lexing test subtree.

### Slice 2 — Add DesugaredParamName variant and per-param name field

Foundation for slices 3+. `ParameterS` gains a `name: IVarNameS` field; `DesugaredParamName(CodeLocationS)` variant added. Existing pattern field remains; existing `pattern.name` usage untouched. Body untouched. Slice is deliberately narrow to isolate the enum change.

- **R**: In `FrontendRust/src/postparsing/test/post_parser_tests.rs`, add:
  - `test_param_gets_desugared_name_at_postparse`: source `func foo(x Int)`. Assert `ParameterS[0].name` matches `IVarNameS::DesugaredParamName(_)`.
- **F**: Run; expect failure (`ParameterS` doesn't have `name` yet).
- **I**:
  - Add `IVarNameS::DesugaredParamName(CodeLocationS<'s>)` in `postparsing/names.rs` and matching `IVarNameValS`.
  - Add `name: IVarNameS<'s>` field to `ParameterS`; update `ParameterS::new`.
  - In `function_scout.rs` param loop, mint `DesugaredParamName(param.range.begin)` per param and pass to `ParameterS::new`.
  - Leave existing `pattern` untouched — this slice adds only.
- **G**: Re-run.
- **A**: postparsing::tests + parsing::tests.

### Slice 3 — Split param-derived rules into outer_shape_rules + named_type_rules on ParameterS

Big architectural risk slice. Moves param-derived rules OFF `FunctionS.rules` and onto per-param buckets with the two-list split. Adds `full_kind_rune` and `inner_kind_rune` fields (`inner_kind_rune == full_kind_rune` when no outer wrap). Existing `pattern` still present but its `kind_rune` becomes redundant (deprecation).

- **R**: In `post_parser_tests.rs`:
  - `test_param_no_outer_wrap_routing`: source `func foo(x Int)`. Assert `FunctionS.rules` has no `LookupSR` for `Int`. Assert `ParameterS[0].outer_shape_rules` is empty. Assert `ParameterS[0].named_type_rules` contains the `Int` lookup. Assert `full_kind_rune == inner_kind_rune`.
  - `test_param_single_ref_wrap_routing`: source `func foo(x &Int)`. Assert `outer_shape_rules` has the `BorrowRefSR` (result=full, inner=inner). Assert `named_type_rules` has the `Int` lookup. Assert `full_kind_rune != inner_kind_rune`.
  - `test_param_nested_ref_wrap_routing`: source `func foo(x &&Int)`. Assert `outer_shape_rules` has TWO chained BorrowRefSRs. Assert the chain from `full_kind_rune` down through an intermediate to `inner_kind_rune`. Assert `named_type_rules` has the `Int` lookup only.
  - `test_param_call_in_named_type_stays_in_named_type_rules`: source `func foo(x Bork<&Spork>)`. Assert `outer_shape_rules` is empty. Assert `named_type_rules` contains ALL of: Lookup Bork, Lookup Spork, BorrowRef &Spork, Call Bork<&Spork>. (The BorrowRef inside the template arg is named-type-level, not outer-shape.)
  - `test_function_rules_no_longer_contains_param_rules`: source `func foo<T>(x T) where T: Kind`. Assert `FunctionS.rules` contains the `T` generic-param bound BUT NOT any param-derived rules.
- **F**: Run; expect failure.
- **I**:
  - Add `outer_shape_rules`, `named_type_rules`, `full_kind_rune`, `inner_kind_rune`, `rune_to_explicit_type` fields to `ParameterS`; update `ParameterS::new` signature; add invariant `debug_assert!`s.
  - Refactor `translate_templex` (`rules/templex_scout.rs`) to take `rule_builder: &mut Vec<IRulexSR>` as an explicit param instead of the current shared reference.
  - Refactor `translate_pattern` (`patterns/pattern_scout.rs:42-129`) to take `outer_shape_builder`, `named_type_builder`, `destructure_builder` and return `(full_kind_rune, inner_kind_rune, name_capture, destructure_atoms)`. At depth 0, walk `pattern_pp.templex`: while node is an outer ref (BorrowRef/HeapOwnRef/ShareRef/WeakRef), emit to outer_shape_builder and recurse; on first non-ref node, switch to named_type_builder and continue. Set `full_kind_rune` = outermost result_rune; `inner_kind_rune` = named-type-root result_rune.
  - In `function_scout.rs` param loop, allocate three fresh `Vec<IRulexSR>` per param. Pass all three to `translate_pattern`. Drain into `ParameterS` on construction. Do NOT push to the shared function-level `rules` vec.
  - Downstream typing consumers of `param.pattern.kind_rune` → mechanical sweep to `param.full_kind_rune`.
- **G**: Re-run.
- **A**: postparsing::tests + parsing::tests.

### Slice 4 — Prepend synthesized LetSE for every param at body head; drop ParameterS.pattern

Uniform desugar. Every param — destructure or not — becomes a LetSE at body head. `ParameterS.pattern` field removed. Anonymous substruct methods, extern/abstract/generated bodies still block-less (handled in slice 6).

- **R**: In `post_parser_tests.rs`:
  - `test_bare_param_desugars_to_let_at_body_head`: source `func foo(x Int) { return x + 1; }`. Assert `ParameterS[0]` has no `pattern` field (compile-time — this test guides the type change). Assert the body's block `expr` is a `ConsecutorSE` whose first expression is a `LetSE`. Assert the LetSE's pattern is `AtomSP { name: Some(CaptureS { name: CodeVarName("x"), mutate: false }), kind_rune: None, destructure: None }`. Assert the LetSE's `expr` is `LocalLoadSE(DesugaredParamName(_))`. Assert `LetSE.rules` is empty (no user-side annotations to translate; the ParameterS already covers the type).
  - `test_destructure_param_desugars_to_let_with_destructure`: source `func foo([a, b] &Tup2<Int, Bool>) { return a + b; }`. Assert the LetSE's pattern preserves destructure `[a, b]`. Assert `LetSE.rules` is empty (no sub-atom types).
  - `test_typed_sub_atom_destructure_puts_sub_atom_rules_on_letse`: source `func foo([a Int, b Bool] &Tup2<Int, Bool>)`. Assert `LetSE.rules` contains `LookupSR` for `Int` and `Bool` (from sub-atoms). Assert `ParameterS[0].outer_shape_rules` + `named_type_rules` DO NOT contain sub-atom-derived rules (they contain Tup2 signature-level rules that also lookup Int/Bool separately; assert the LetSE-side rules are distinct).
- **F**: Run; expect failure.
- **I**:
  - Delete `ParameterS.pattern` field. Update `ParameterS::new` signature. Compile errors will surface every consumer that reads `param.pattern` — update to use `param.name` for name binding queries or read from the LetSE's pattern for destructure structure.
  - In `translate_pattern`, capture the input `AtomSP` (name + kind_rune + destructure) and return it separately from the ParameterS fields.
  - After `scout_body` returns in `function_scout.rs:626-711`, walk each param. For each:
    - Compose an `AtomSP` for the LetSE pattern: `AtomSP { range, name: original_user_name_capture, kind_rune: None (redundant since expr provides type), destructure: original_destructure }`.
    - Allocate LocalLoadSE for `ParameterS.name`.
    - Synthesize LetSE with the destructure_rules drained from the param's destructure builder.
    - Prepend to the body's ConsecutorSE (re-alloc the enclosing BlockSE + BodySE).
  - Rewrite `get_parameter_captures` (`patterns/pattern_scout.rs:15-28`) to no longer walk destructures — all body locals now come from LetSE scouting.
- **G**: Re-run.
- **A**: postparsing::tests + parsing::tests.

### Slice 5 — Nested destructures + ignore + empty edge cases

Ensures the desugar respects the full destructure grammar.

- **R**: In `post_parser_tests.rs`:
  - `test_nested_destructure_preserved`: source `func foo([a, [b, c]] T)`. Assert the LetSE's pattern is the nested `[a, [b, c]]` shape.
  - `test_destructure_ignore`: source `func foo([_, b] T)`. Assert the LetSE's pattern has an ignored capture at position 0; only `b` becomes a local.
  - `test_empty_destructure`: source `func foo([] T)`. Assert the LetSE's pattern has `destructure: Some([])`; no locals.
- **F**: Run; expect failure or unexpected shape.
- **I**: Verify recursion in slice 4's implementation handles all three cases; adjust if any misfires.
- **G**: Re-run.
- **A**: postparsing::tests + parsing::tests.

### Slice 6 — Extern/abstract/generated bodies reject param destructures

Hard error at postparse for `extern func foo([a, b] Tup2);` etc.

- **R**: In `post_parser_tests.rs`, three tests:
  - `test_extern_param_destructure_rejected`
  - `test_abstract_param_destructure_rejected`
  - `test_interface_method_param_destructure_rejected`

  Each asserts `Err(ICompileErrorS::ParamDestructureRequiresBody { .. })` from `post_parse_program`. Also add `test_extern_bare_param_ok`: source `extern func foo(x Int);` — assert this postparses successfully, no destructure error, `ParameterS[0].name` is `DesugaredParamName(_)`.
- **F**: Run; expect failure.
- **I**: Add `ICompileErrorS::ParamDestructureRequiresBody { range: RangeS<'s> }`. In `function_scout.rs` at extern/abstract/generated body branches (~lines 586, 593, 605, 621), scan `explicit_params_s` for any `pattern_destructure.is_some()`. Short-circuit with the error. Add humanizer support in `post_parser_error_humanizer.rs`.
- **G**: Re-run.
- **A**: postparsing::tests + parsing::tests.

### Slice 7 — Typing end-to-end: destructure param produces correct runtime value

Escalates to full-suite scope. Proves the desugar preserves runtime semantics through the full compiler.

- **R**:
  - Add fixture `FrontendRust/src/tests/programs/param_destructure_basic.vale`:
    ```
    exported func main() int { return apply([3, 4]); }
    func apply([a, b] Tup2<int, int>) int { return a + b; }
    ```
  - Add `param_destructure_nested.vale`: nested `[a, [b, c]]` returning summed value.
  - Add `param_destructure_typed_sub_atoms.vale`: `[a Int, b Bool]` (lang-appropriate) returning something branch-driven.
  - Add matching tests in `FrontendRust/src/typing/test/compiler_tests.rs` following the existing fixture-runner convention.
- **F**: Run the new tests. Expect failure. Escalate: run the full test suite to shake loose any consumer that broke silently from slices 3-4.
- **I**: Fix whatever surfaces. Most likely sites: typing consumers of `param.pattern.kind_rune` (should already be renamed to `param.full_kind_rune` per slice 3; if any were missed, catch them here). `pattern_compiler.rs` interaction with the synthesized LetSE — should be zero change since LetSE handling is unaltered. Any `variable_uses` code that was tracking params directly may need updating (all locals now come from LetSE scouting, no direct-param path).
- **G**: Re-run new tests + full suite.
- **A**: Full suite (`cargo test --manifest-path FrontendRust/Cargo.toml --lib`).

### Slice 8 — Anonymous-interface macro benefits from the outer_shape_rules / inner_kind_rune split

Regression check + semantic win. The anonymous-interface macro can now cleanly read `ParameterS.outer_shape_rules` when synthesizing a forwarder's abstract-param shape mirror. If any consumer of `FunctionS.rules` broke silently, this catches it.

- **R**: Grep `typing/` for `.rules` reads on `FunctionS`/`function`. For each consumer, decide: does it need "all param-derived rules"? If so, add a helper `FunctionS::all_rules()` that concats param outer_shape + named_type + function-level rules. Add a single test in `typing/test/compiler_tests.rs`:
  - `test_anonymous_interface_forwarder_uses_outer_shape_rules`: `.vale` fixture with an interface method taking `&Interface` self; verify the synthesized anonymous-substruct implementation compiles and dispatches correctly (assert program output).
- **F**: Run; expect pass if consumers already read via a helper or don't need param rules; fail if a consumer was silently miscompiling.
- **I**: Fix any consumer whose semantics broke. Add `FunctionS::all_rules()` helper if not already present.
- **G**: Re-run.
- **A**: Full suite.

### Slice 9 — Refactor suggestions (no code)

After slices 1-8 are green, review the resulting code. Surface refactor candidates to the user:
- Consolidating the ConsecutorSE-prepend logic into a helper method on `PostParser`.
- Whether `LetSE.rules` and `ParameterS.outer_shape_rules` share a builder-alloc helper.
- Whether the invariant assertions in `ParameterS::new` deserve dedicated test coverage as debug-only vs release-time enforcement.
- Whether `variable_uses.rs` can shrink now that params no longer contribute directly.

No code changes. Present candidates as a list; ask the user which, if any, to address in a follow-up plan.

## Verification

- **After each slice's G substep**: run specific tests added for that slice, confirm green.
- **After each slice's A substep**: run appropriate suite scope (parsing+postparse for slices 1-6; full suite for slice 7+).
- **After slice 9 (end)**: `cargo build --manifest-path FrontendRust/Cargo.toml --lib` clean; `cargo test --manifest-path FrontendRust/Cargo.toml --lib` no new failures vs. pre-refactor baseline. Grep `typing/` for `.pattern.kind_rune` and `.pattern.destructure` — should be zero results (all migrated to `.full_kind_rune` or accessed via LetSE).
- **End-to-end fixture check**: manually run each `param_destructure_*.vale` fixture from slice 7 through the compiler; confirm the emitted output values match expectations. Closes the "did the desugar preserve runtime semantics" question with real observation.

## Known deferred concerns

- **Return-position rules**: this plan doesn't touch return-position rules. Follow-up plan can apply the same two-list split to return position if the anonymous-interface macro (or another consumer) needs it.
- **Nested-Call substitution** for `&Container<Interface>` — inner_kind_rune points at the whole `Container<Interface>` Call result, not the inner `Interface`. Follow-up if a use case emerges.
- **`AnonymousSubstructTemplateName` widening**: unrelated ongoing session concern about `StructS.name` and `ImplS.name`. Deferred.
