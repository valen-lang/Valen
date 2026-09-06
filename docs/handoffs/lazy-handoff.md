# Lazy citizen compilation handoff

## Start here next session

Implement the **impl-bounds plan**: `~/.claude/plans/plan-it-out-ty-tidy-floyd.md`. Make `where
implements` (impl) bounds work as a **full mirror of `where func` (function) bounds** — synthesize an
`ImplS` per bound and register it (registration decision is settled: full mirror), add a
reachable-impl slot to `InstantiationReachableBoundArgumentsT`, add `resolve_citizen_impl_bounds` /
`import_impl_bound`, extend the `Call` arm of `fn evaluate_templex` so it can resolve an interface
application, wire the three harvest sites, and clear the three `rune_to_bound_impl` passthrough
panics in `src/typing/function/function_compiler_solving_layer.rs`. All `src/typing/` core → needs
the literal "fire core edits".

## The endeavor

Move struct/interface/impl/function compilation off the eager loops onto a lazy, resolve-driven
"illuminate → resolve → compile" model. Source of truth:
`src/typing/docs/architecture/lazy-compilation-design.md` (design). Earlier RFIGA plan:
`~/.claude/plans/please-plan-all-these-parsed-wolf.md`.

## Function bounds — done (the model impl bounds copies)

A `where func drop(T)void` bound is captured in the postparser as a synthesized abstract `FunctionS`
in the `func_bounds` field on `StructS`/`InterfaceS`/`FunctionS`. In the typing pass:
- `fn resolve_citizen_bounds` (`src/typing/citizen/struct_compiler.rs`) produces a citizen's bounds in
  a use-site's terms by binding its generic runes **directly** to the use-site args (no placeholder
  round-trip), evaluating each bound's param/return `ITypeST` via `fn evaluate_templex`.
- `fn import_function_bound` + `fn register_bound_function_s` (`src/typing/templata_compiler.rs`)
  re-anchor a bound under the calling denizen and register its postparsed `FunctionS`.
- Reachable (inherited-from-citizen) bounds are harvested at three sites:
  `fn check_defining_conclusions_and_resolve` (`src/typing/infer_compiler.rs`), `fn get_reachable_bounds`
  (`src/typing/templata_compiler.rs`), `fn look_for_override` (`src/typing/edge_compiler.rs`).
- The instantiator consumes them via `rune_to_bound_prototype` /
  `rune_to_citizen_rune_to_reachable_prototype` in `fn assemble_instantiation_bound_param_to_arg`
  (`src/instantiating/instantiator.rs`), resolving a bound call through `func_id_to_bound_arg_prototype`.

## Closure-capture member naming — fixed

Reading a captured variable in a lambda body lowers to a `MemberLookup` on the closure struct; its
`member_name` must be the struct's `MemberNameT` (`IVarNameT::Member`), calculated from the struct via
`get_member_and_index` — not the capture's original `Local` name. The `IVariableT::Capture` arm of
`fn evaluate_lookup_for_load` in `src/typing/expression/expression_compiler.rs` now does this.
Regression: `closure_capture_read_names_its_member` in `src/typing/test/compiler_lambda_tests.rs`.

## Integration/e2e tests are ZONION-parked

Almost all `src/integration_tests` and `src/end_to_end_tests` tests are `#[ignore] // ZONION` stubs
(`unimplemented!()`), parked by the onion re-link commit `4fc5c4de`, which also deleted the
builtins-loading `test(...)` harness. The current `test_no_builtins` /
`test_no_builtins_without_borrow_check` (`src/integration_tests/tests/run_compilation.rs`) build only
the user program — no builtins — so a `where func drop(T)void` bound at `T=int` can't be satisfied
unless the program self-provides `func drop(x int) void { }` (as
`supplying_bounded_struct_to_struct_accepting` in `integration_tests_c.rs` now does; it is un-ignored
and green). Reviving the ZONION batch properly needs a builtins-loading integration harness restored.

## State / verification

This session's work (the func-bound substitution/import refactor, the closure-member fix, the two new
tests) is **uncommitted** — see `git status`. Gates, all green (re-run to refresh):
- native: `cargo nextest run --manifest-path ./Cargo.toml`
- wasi: `VALE_TEST_BACKEND=wasi cargo nextest run --manifest-path ./Cargo.toml`
- rust_interop: `cargo +rustc-fork test --manifest-path ./Cargo.toml --lib --features rust_interop`

## Constraints

- Determinism is P0. The lazy queue must be insertion-ordered and traversed deterministically.
- `main` is always exported (convention — write `exported func main`); a non-exported `main` in a test
  is a bug to fix, not to accommodate.

## Lessons Learned

- **A MemberLookup names a *member*, not a variable.** Since `24338999` gave members their own
  `MemberNameT`, any path building a `MemberLookupTE` must name it `IVarNameT::Member` off the struct
  (`get_member_and_index`), never a `Local` — the instantiator's exact-variant `==` matches nothing
  otherwise and panics "member name not found". The `dot`-access path was fixed then; the
  closure-capture read is the one that got missed.
- **`test_no_builtins` loads no builtins, by design.** A test program relying on `drop`/`print`/etc.
  must define them itself or nothing resolves; only the deleted `test(...)` harness loaded `drop.vale`,
  and primitive drops were never intrinsic.
- **Bind a citizen's generic runes straight to the use-site args, never to conjured placeholders.**
  While compiling denizen F, never materialize placeholders rooted in another denizen — substitute
  directly. That is why `resolve_citizen_bounds` takes the args rather than calling `create_placeholder`.
- **Trap:** `fn get_parents` in `impl_compiler.rs` walks only the sub-citizen's file; impl enqueue needs
  both the sub-citizen's and super-interface's files.
- **Trap:** the interface sealed flag is read by `fn evaluate_maybe_virtuality`
  (`function_compiler_middle_layer.rs`) via `fn lookup_sealed`, which panics on a miss and never touches
  the interface's env — lazy illuminate must be forced there before the read.
