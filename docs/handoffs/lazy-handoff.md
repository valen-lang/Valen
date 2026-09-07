# Lazy citizen compilation handoff

## Start here next session

The `expect_compiler_outputs` AST-migration is largely done (policy: reconstruct essential structural
assertions from intent against the onion AST; drop-with-comment when incidental and eval-covered; leave
**parked** when the assertion's concept was deleted from the AST). `src/integration_tests` suite is at
`cargo test --lib integration_tests::` (measure; was 85 at the start of that work). The remaining
integration-test work, in priority order:

- **The real onion-compiler failures the migrated eval/structural tests now surface** — these need
  "fire core edits" (see the failure map in `tmp/zonion-categories.md`): R1 borrow-group
  (`typing/compilation.rs`, `borrow_checker/borrow_types.rs`), R3 Reinterpret kind mismatch
  (`instantiator.rs:1730`), a parse gap on the `Struct[members] = ^x;` member-destructure syntax
  (`parse_and_explore.rs:33`), the mutable-lambda capture path (`expression_compiler.rs:256`), and the
  interface-dispatch/R2 family.
- **Parked because their assertion's concept was deleted** (revive when the concept returns): the
  Addressible-local tests (`array_list_test::mutate_mutable_from_in_lambda`, `move_mutable_from_in_lambda`),
  the Address-member closure test (`closure_tests::mutates_from_inside_a_closure`), and the nested-tuple
  shape tests (`pack_tests::nested_seqs`, `nested_tuples` — the `NodeRefT::Tuple`/`TupleTE` node is gone).
- **weak cluster (`weak_tests.rs`) still parked** — blocked on the disabled `weak` builtin module
  (`VCOORD: re-enable weaks`), not on AST shape.

`tmp/zonion-categories.md` has the full per-cluster log. Traps and the onion AST shape facts learned
during this migration are in Lessons Learned below.

## Impl-bounds plan (core lazy work, pending)

Implement `~/.claude/plans/plan-it-out-ty-tidy-floyd.md`: make `where implements` (impl) bounds a
**full mirror of `where func` (function) bounds** — synthesize an `ImplS` per bound and register it
(full mirror), add a reachable-impl slot to `InstantiationReachableBoundArgumentsT`, add
`resolve_citizen_impl_bounds` / `import_impl_bound`, extend the `Call` arm of `fn evaluate_templex` to
resolve an interface application, wire the three harvest sites, and clear the three `rune_to_bound_impl`
passthrough panics in `src/typing/function/function_compiler_solving_layer.rs`. All `src/typing/` core →
needs the literal "fire core edits".

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

## Integration test revival (ZONION burndown)

`src/integration_tests/tests/` is un-ZONION'd: un-ignored and migrated onto the onion harness (un-stub
the body, drop the deleted Hammer plumbing, point builtins-using tests at `test(...)`). `fn test` in
`run_compilation.rs` is restored — it loads builtins into the **root** package (visible without import,
the "old way"), while `fn test_no_builtins` stays bare. `fn expect_compiler_outputs` /
`get_compiler_outputs` / `get_scoutput` / `run_primitive_args` / `eval_for_stdout` are re-added there as
delegations to `InstantiatedCompilation`, plus `eval_for_kind_and_stdout` (returns `(IVonData, String)`).
`src/end_to_end_tests` is still ZONION-parked. Suite state: `cargo test --lib integration_tests::`
(was 3 passing pre-migration; measure).

The `expect_compiler_outputs` tests are migrated: incidental structural checks dropped-with-comment
keeping the eval; essential ones reconstructed against the onion AST. Remaining integration-test work is
listed under "Start here" — the two `get_monouts` tests, the real onion-compiler gaps (need "fire core
edits"), the parked-because-concept-deleted tests, and the weak cluster.

Still parked on a deleted harness method: **`get_hamuts` tests** reference the deleted Hammer IR —
decide rewrite-vs-retire.

## borrow_checker toggle

`TypingPassOptions.borrow_checker_enabled` (`src/typing/compilation.rs`, default **true** in every
constructor) gates the sole `fn check_function` call in `src/typing/function/function_compiler_core.rs`.
Exactly one test opts out — `tests_generic_s_lambda_calling_parent_function_s_bound` via
`test_no_builtins_without_borrow_check` — so it reaches the instantiator past a deferred
closure-capture group-check in main's group borrow checker. `fn compiler_test_compilation_without_borrow_check`
is marked `// VCOORD: remove this`: a one-test escape hatch, not a general API.

## State / verification

The func-bound substitution/import refactor, the closure-member fix, and the `borrow_checker_enabled`
toggle are **landed on `main`** (and pushed to `origin/main`). The ZONION integration-test migration and
the `run_compilation.rs` harness additions (`fn test` + the five delegations) are **uncommitted** on
this branch — `git status`. This branch was rebased onto main's inline-`Kind`-by-value change, so `fn evaluate_templex`
builds `ITemplataT::Kind(KindTemplataT { .. })` by value (no `interner.alloc`). Gates, all green (re-run
to refresh):
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
- **Do not assume a typed-AST node kept its pre-onion shape.** `CoordT` folded into `KindT` (ownership is
  now `KindT::{Borrow,Own,Share,Weak}Ref` variants; `header.return_type` is a bare `KindT`), and
  `NodeRefT` variants were renamed/removed — inspect the real AST before writing a `collect_only_tnode!`
  pattern; guesses are wrong about half the time.
- **Onion ownership encoding in `KindT`: owned is bare, the rest are wrapped.** An owned kind (primitive
  *or* struct/RSA/interface) is the bare variant (`KindT::Int(IntT::I32)`, `KindT::Struct(stt)`); a
  borrow/share/weak kind keeps its wrapper (`KindT::BorrowRef(BorrowRefT{inner})`, `ShareRef`, `WeakRef`)
  — including shared *primitives* (a `str` is `KindT::ShareRef(inner: KindT::Str(StrT))`, not bare). Local
  variables and struct members hold their kind in a `tyype: KindT` field (the old `coord`/`.kind` is gone).
- **Plain locals are `IVarNameT::Local(LocalNameT)`, not `Member`** (Member is for struct members). And a
  name's string moved behind `imprecise_name: CodeNameS { name: StrI(..) }` on `MemberNameT`,
  `CodeVarNameS`, `LocalNameT` — the flat `name: StrI` field is gone.
- **`LetAndLendTE`/`IfTE`/local vars are flat now.** `IfTE.result` and `LetAndLendTE.result` are fields
  (a `KindT` / a `&BorrowRefT` — a LetAndLend *always* yields a borrow); `OwnershipT` is deleted; the
  `ILocalVariableT::Reference/Addressible` split and `IMemberTypeT::Reference/Address` are gone
  (`LocalVariable`/`StructMemberT` are flat `{name, tyype}`); `AddressMemberLookup` is commented out.
- **The scout/postparse AST is stable across the onion rework.** `IExpressionSE`/`BlockSE`/`LocalS`/
  `IfSE`/`ConstantIntSE`/`NodeRefS` etc. kept their shapes — `get_scoutput` tests need only harness/import
  fixes, no assertion rework.
- **The instantiated AST (`get_monouts` → `HinputsI`) is a separate node family** (`KindIT`, `StructIT`,
  `IdI`, `INameI`, `LetNormalIE`, …; traverse via `only_in_function` in `instantiating/collector.rs`). It
  underwent the **same** ownership fold as the typed AST: `CoordI` / `OwnershipI` / `IMemberTypeI` are
  gone; `KindIT` carries ownership as `BorrowRefIT` / `OwnRefIT` / `ShareRefIT` / `WeakRefIT` variants
  (owned is bare), members are flat `StructMemberI { name, tyype: KindIT }`, and locals expose `tyype:
  KindIT` (the old `collapsed_coord()` / `coord()` and `expect_coord_templata_i` helpers are removed).
  So the shapes mirror the typed side, but the type *names* differ (`…IT`/`…I` suffix).
- **`get_all_user_functions()` counts builtins when the program is built with `test`** (which loads
  builtins into the root package). For a "there are N user functions" assertion use `test_no_builtins`
  so the count reflects only user code.
- **Honest migration of a structural assertion, not output-matching:** reconstruct what the test *should*
  prove and express that against the current AST; if reconstructed ≠ actual, first suspect a wrong *shape*
  (fix it), then a real compiler bug (log + leave red, don't touch the assertion), then a genuinely
  obsolete concept (leave the test parked). Never rewrite the expected value to match whatever the
  compiler currently emits.
- **Restoring a deleted `RunCompilation` test-harness method is usually a one-line delegation**, not new
  plumbing — `InstantiatedCompilation` already exposes `expect_compiler_outputs` / `get_compiler_outputs`
  / `get_scoutput` / `get_monouts`.
- **Recurring agent mistake:** un-stubbing a `/* … */`-commented test body with a mid-body edit that drops
  only the opening `/*` or only the closing `*/` leaves an unclosed comment that silently swallows the
  rest of the file. Edit whole-body (fn signature through the closing brace), and after a bulk pass check
  `/*` count == `*/` count. (Vale `}` at column 0 inside `r"…"` strings also breaks `^}`-based fn-end
  finding — anchor on the next `#[test]`.)
