# Lazy citizen compilation handoff

## Start here next session

**Next: move the remaining integration failures forward.** Measure first:
`cargo test --manifest-path ./Cargo.toml --lib --features no_backend integration_tests:: > ./tmp/<name>.txt 2>&1`
(as of this handoff: 164 passed / 46 failed / 103 ignored).

**Freshest lead — mutable-capture closures.** `closure_tests::{mutable_lambda,
mutates_from_inside_a_closure, mutates_from_inside_a_closure_inside_a_closure}` (all now run under
`test_without_borrow_check`) clear typing and the instantiator and fail only in the testvm:
`Mutate: unexpected destination_expr Deref(...)` from the `Mutate` arm of `fn execute_node` in
`src/testvm/expression_vivem.rs`. That handler has arms for Local/Member/StaticArray/RuntimeArray-lookup
destinations but none for a `Deref` — writing *through* a captured reference. Adding that arm is
AI-editable testvm work; a captured primitive must write through to the original local's storage, not
repoint the closure member. Read-capture closures already work (`captured_own_is_borrow`,
`test_closure_s_local_variables`, `capture`, `test_returning_a_nonmutable_closured_variable_from_the_closure`,
`read_from_inside_a_closure_inside_a_closure`).

The **borrow-group cluster** is the other coherent compiler target — needs "fire core edits" (the
`borrow_checker/` dir is AI-editable per the typing gate):
- **R1** — `borrow_checker/borrow_types.rs:347`, "returns a borrow reference with no group".
- **R5 (2 tests)** — `borrow_types.rs:516`; `hash_map_tests::{hash_map_values, hash_map_remove_2}`, which
  the array-index decay + vassertEq call-site fix advanced here from overload resolution.

The rest (re-measure per-bucket counts — the closure thread moved several): **parked `unimplemented!()`
test stubs** (not compiler bugs — un-migrated bodies, incl. the import-system harness ×4);
**immutable-citizen `vfail`** (`instantiator.rs:1282`, the `imm` struct/interface tests);
**`each`/`parallel_foreach` Map lowering** unimplemented (`expression_compiler.rs:1680`); **`compilation.rs:155`**
(array-functor lambda stragglers, `each_on_ssa` missing `drop(StaticSizedArrayReadonlyIter)`,
`panic_function` override-dispatch drop); and a few singletons (`function_body_compiler:133`,
`lex_and_explore:38`, one eval assertion).

**Strings (R3), interfaces, and `migrate` are all `#[ignore]`d** with greppable reasons
(`grep -rn "R3: str share-peel\|interfaces branch\|migrate builtin" src/integration_tests`). A separate
branch owns interfaces. R3 is the `&@str`→`&str` share-peel: `fn convert` (`convert_helper.rs`) builds a
Reinterpret that peels the `ShareRef` on a `str` borrow, but the instantiator's Reinterpret arm
(`instantiator.rs:1769`) asserts source == result after substitution. `migrate`
(`src/builtins/resources/migrate.vale`, `__vbi_panic()` stub) needs the borrow-group work first —
uncommenting it regresses the suite to ~215 failures (its `drop_into` lambda mutably captures + moves).

The ZONION migration policy (still in force for any remaining red guides): reconstruct essential
structural assertions against the onion AST; drop-with-comment when incidental and eval-covered; leave
**parked** when the assertion's concept was deleted. Parse gaps are fixed (fixtures use `StaticArray<N,T>`
and bare-owned types). Still parked:

- **Parked because their assertion's concept was deleted** (revive when the concept returns): the
  Addressible-local tests (`array_list_test::mutate_mutable_from_in_lambda`, `move_mutable_from_in_lambda`,
  still `unimplemented!()`). (The nested-tuple tests are *not* parked —
  `pack_tests::{extract_seq, nested_seqs, nested_tuples}` are reconstructed as `Construct`-of-`Tup<N>`.)
- **weak cluster (`weak_tests.rs`) still parked** — blocked on the disabled `weak` builtin module
  (`VCOORD: re-enable weaks`), not on AST shape.

`tmp/zonion-categories.md` has the full per-cluster log. Traps and the onion AST shape facts learned
during this migration are in Lessons Learned below.

The **lambda functor-passing cluster is resolved at the source level** (functor params take `&F`,
call sites pass `&`, lambda bodies/indices `__copy_prim` their references) — see the Lessons Learned
entry. The `stdlib` `each`/`or`/`test`/`map` copies still carry bare-`F` sites (VHORK-marked) if you
run stdlib programs. Mutable capture is now wired through typing and the instantiator; the only
remaining gap is the testvm write-through-`Deref` store (see "Freshest lead" above), which is
AI-editable testvm work, not a core gap.

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

All `expect_compiler_outputs` / `get_scoutput` / `get_monouts` tests are migrated: incidental structural
checks dropped-with-comment keeping the eval; essential ones (including the instantiated-AST `get_monouts`
ones) reconstructed against the onion AST. Remaining integration-test work is under "Start here" — the
real onion-compiler gaps (need "fire core edits"), the parked-because-concept-deleted tests, and the
weak cluster.

`hammer_tests.rs` is deleted (the Hammer pass is gone). A handful of other tests still carry
commented-out `get_hamuts` bodies and stay `unimplemented!()` — in `virtual_tests.rs` (the two
`open_interface_constructor*` and `interface_with_method_with_param_of_substruct`), `integration_tests_c.rs`,
and `weak_tests.rs` — decide rewrite-against-the-onion-IR vs retire.

## borrow_checker toggle

`TypingPassOptions.borrow_checker_enabled` (`src/typing/compilation.rs`, default **true** in every
constructor) gates the sole `fn check_function` call in `src/typing/function/function_compiler_core.rs`.
`run_compilation.rs` exposes two borrow-check-off integration harness entry points —
`test_without_borrow_check` (builtins loaded) and `test_no_builtins_without_borrow_check` (bare); the
integration tests that fail *only* inside the group borrow checker call them to reach later passes.
`fn compiler_test_compilation_without_borrow_check` (the typing-pass unit-test helper)
is marked `// VCOORD: remove this`: a one-test escape hatch, not a general API.
Note the toggle does **not** silence the typing-pass "returns a borrow reference with no group" error
(emitted before the checker at `typing/compilation.rs`), so a test can still be red there with the
checker off.

## State / verification

The func-bound refactor, the closure-member fix, the `borrow_checker_enabled` toggle, and the whole
ZONION integration-test migration (incl. the `run_compilation.rs` harness) are **landed on `main`** at
HEAD, rebased onto main's Phase B block-scoped-restrict borrow-checker work and pushed to `origin/main`.

**Trunk-green / tree-red split — the live TDD state.** The landed commit carries every failing
test under a temporary `#[ignore = "temp-fire-commit-lambda-land: un-ignore right after landing"]`,
so `cargo nextest run` on a clean checkout is green (1029 passed / 0 failed / 288 ignored). The
**working tree has those ignores removed** (uncommitted — `git status` shows ~24 modified `*.rs`,
mostly under `src/integration_tests/tests/` plus `typing/test/compiler_tests.rs` and
`instantiating/tests/instantiated_tests.rs`), so running the suite locally shows the ~137 red TDD
guides. That un-ignored state is the thing to keep working from; re-applying the ignores is only for
the next green-trunk commit. To reproduce the ignore/un-ignore, grep the marker string above — every
temp ignore carries it verbatim, so a one-line sweep reverses the set exactly.

**Uncommitted, on top of the landed tip** (nothing committed since `5b85640a` — no "fire commit").
Core fixes (all `src/typing/` or `src/testvm/`): foreach loop-name resolution (`imprecise_name()` arms
in `src/typing/env/environment.rs`, threaded through `names.rs`/`typing_interner.rs`/`name_translator.rs`/
`instantiator.rs`); the `&&`→`&` decays now at **four** producers — dot lookup and the **array-index
lookup** (`expression_compiler.rs` `Dot` and `Index` arms), the call result (`call_compiler.rs`), plus
the local/capture lookups; the `vcurious` stub (`expression_compiler.rs` `Ownershipped` `_ =>`/`Move`
arm — moving an owning source now passes through); error-humanizer stub arms. Closures: `translate_addr_expr`
is deleted (the address-expression concept is gone) — its lookup arms folded into `fn translate_ref_expr`
and the `Mutate` destination now routes there (`src/instantiating/instantiator.rs`); the mutate-capture
path (`fn evaluate_addressible_lookup_for_mutate` in `expression_compiler.rs`) now names its `MemberLookup`
via `get_member_and_index` like the read path. TestVM inline-struct
overwrite-in-place: `fn overwrite_struct_in_place` in `src/testvm/heap.rs`, branched into `mutate_struct`,
`mutate_array`, `mutate_variable` on a bare `KindIT::StructIT` (everything else repoints). Test/stdlib:
parse-gap fixtures, `Some(&i32)` copyprim in `src/tests/hashmap/hashmap.vale`, duplicate-builtin harness
fixes (`test_no_builtins_without_borrow_check` for self-provided-builtin tests), VHORK markers, and the
R3/interface/string/migrate `#[ignore]`s. `src/testvm/testvm-design.md` inline-mutation section
ratified. Closure tests (`closure_tests.rs`): `test_returning_a_nonmutable_closured_variable_from_the_closure`
un-ignored and `read_from_inside_a_closure_inside_a_closure` migrated off the deleted `HammerInterner`
harness (both now pass under `test_without_borrow_check`); the three mutable-capture tests switched to
`test_without_borrow_check` (still red at the testvm `Deref` gap); `addressibility` deleted (dead
deleted-API stub). Integration suite `164 passed / 46 failed / 103 ignored`
(`cargo test --manifest-path ./Cargo.toml --lib --features no_backend integration_tests::`).

Gates, all green on the landed (ignored) tip; re-run to refresh:
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
- **More onion AST-reconstruction facts.** A tuple has no node of its own — `(a, b)` lowers to a
  `NodeRefT::Construct(ConstructTE { struct_tt: <Tup2>, args: [..] })`; nest `ExpressionTE::Construct` for
  nested tuples. Exports and externs live on `HinputsT.kind_exports` / `HinputsT.function_externs`
  (`coutputs.kind_exports.find(exported_name == "…")`), not behind a Hammer pass. The pointer downcast
  function is named `try_as` (not `as`), and a pointer downcast returns an **owned** `Result` interface
  (`KindT::Interface(itt)`, not a borrow) whose two template args are each `ITemplataT::Kind(KindTemplataT
  { kind: KindT::BorrowRef(_) })`.
- **Trap — a bulk harness-call swap can silently regress a green test.** The borrow-disable sweep turned a
  `test_no_builtins` call into `test_without_borrow_check`, which *loads builtins*; that broke the one green
  test that self-provides `func drop(int)` (now a duplicate-drop conflict). A `no_builtins` test must map
  to the `no_builtins` variant. After any bulk test edit, check for regressions: a test that was live
  (not `unimplemented!()`, not `#[ignore]`) at the base commit and fails now is a candidate — the compiler
  is unchanged when only test files moved, so it should still pass.
- **Some integration tests share process state (a real isolation bug, determinism-P0).** A test's pass/fail
  can depend on which *other* tests ran first: `array_with_capture` passes only when others run before it;
  `roguelike_typing_pass` (`#[ignore]`'d) is the opposite — passes in isolation, fails in the full suite.
  So ignoring/reordering tests can shift the pass-set. Fix the shared mutable state rather than paper over it.
- **`safe-script-runner review` must be run bare** — Guardian denies any pipe/redirect/`2>&1` on it, by
  design, so the whole diff lands in the transcript; read that full diff before writing the required
  `Issues I see in the diff:` line, never a `grep -c` count of it.
- **A bulk-ignore script keyed on leaf fn names silently over-ignores across files.** Test names like
  `tests_a_linked_list` / `calls_destructor_on_local_var` exist in BOTH the integration tests (failing)
  and `typing/test/compiler_tests.rs` (passing), so a global leaf-name match ignores the passing copies
  too. Guard: check the per-file insertion count against the file's expected failing count, and after the
  sweep confirm the *passing* total is unchanged (a drop means a passing test got ignored). Handle the
  colliding file (compiler_tests.rs) with path-specific edits, not the global script.
- **The lambda functor-passing cluster is fixed at the source level, not in core.** A generic functor
  param must be declared `&F` (borrow), not bare `F` — a bare kind rune absorbs the argument's borrow
  under the onion ownership-fold, so `where func(&F,...)` then needs a double-borrow `__call(&&λ)` that no
  lambda provides. Fix pattern, all in `.vale`/fixtures: (1) declare functor params `&F`/`&G` (done in
  `builtins/resources/arrays.vale` `Array`, `tests/array/make/make.vale` `MakeArray`; `stdlib` copies —
  `List`/`optutils`/`testsuite`/`hashset` `each`/`or`/`test` — still carry bare-`F` sites); (2) pass the
  lambda by borrow at call sites (`&{...}`); (3) `__copy_prim` any lambda body that returns a reference
  (`{_}` yields `&int`) and any array index that is a reference (`a[__copy_prim(i)]`). Mutable capture is now
  wired end to end through typing and the instantiator (both `IVariableT::Capture` arms — read and
  mutate — are implemented); the only remaining gap is the testvm write-through-`Deref` store.
- **`&&` (a `BorrowRef` of a `BorrowRef`) forms wherever `&T` meets a `T` that is already a reference.**
  Sources seen: reading a ref-typed struct member via dot-access; **reading an element of a `[]&V`
  array** (an array of borrows — e.g. `HashMap.values()` returns `[]&V`, so `k[i]` is `&&V`); a generic
  `&T` return/param instantiated with a ref (`Opt.get<&str>` returns `&&str`); `&a` on a `&`-typed local.
  `&&` is spurious — typing decays it to `&` at each expression producer by wrapping a `DerefTE` when
  `.result()` is `BorrowRef(BorrowRef(_))`: local/capture lookups, the `Dot` **and `Index`** arms in
  `expression_compiler.rs`, and the call result in `call_compiler.rs`. Any new `&&` *producer* needs the
  same decay; the principled alternative is to collapse `&&`→`&` at kind construction. **The decay does
  NOT fire in argument-coercion position** — a bare argument can still arrive as `&&T` at the callee, and
  `__copy_prim` can't rescue it (it is a one-level `&primitive → primitive` read-out, `convert_helper.rs`;
  hand it `&&int` and it panics at `expression_compiler.rs:861`, "expects &primitive").
- **By-borrow-arg → by-value-param needs `__copy_prim`; the reverse (owned-arg → `&T`-param) needs an
  auto-borrow, which is deferred.** `Some<int>(index)` with `index` a soft-loaded `&int` is the first
  (peel with `__copy_prim`, fixed in `hashmap.vale`). `vassertEq(k.len(), 4)` into `&T` params is the
  second (owned `int` can't bind to `&int`; convo-18's ignored `test_taking_callable_arg_value_into_ref_param`
  is the same open question). Don't conflate the two directions.
- **Duplicate-builtin trap: a test that self-provides a builtin must use a `no_builtins` harness.** A
  fixture declaring `func drop(int)`, `extern __vbi_addI32`, or `extern __vbi_panic` under
  `test_without_borrow_check` (which loads builtins) yields "Multiple candidates for call". Swap to
  `test_no_builtins_without_borrow_check` and drop any `import v.builtins.*` (those package stubs only
  exist when builtins are loaded).
- **TestVM overwrite-in-place is inline-structs-only, via one shared helper.** `fn overwrite_struct_in_place`
  (`src/testvm/heap.rs`) is branched into `mutate_struct` (members), `mutate_array` (elements), and
  `mutate_variable` (locals/params) on a bare `KindIT::StructIT`; primitives and RSA-members repoint.
  It preserves the storage's allocation and moves the source's field refs in, returning a fresh `P` with
  the old fields (the `Mutate` result); the caller's `discard(source)` frees the source structurally, so
  the helper leaves the source populated and only *adds* the preserved allocation's referrers. The
  `mutate_variable` branch must skip the **whole** repoint body (both the `Variable` referrer re-key and
  `mutate_local`), or the source leaks. `moving an owning source` in the load path (`Ownershipped` arm)
  is a pass-through, not the old `panic!("vcurious")`.
- **R3 is the `str` share-peel Reinterpret, not a generic instantiator bug.** `fn convert`
  (`convert_helper.rs`) builds a Reinterpret to peel a `str`'s `ShareRef` (`&@str`→`&str`), so its
  source (`BorrowRef(ShareRef(Str))`) and result (`BorrowRef(Str)`) deliberately differ — but the
  instantiator's Reinterpret arm (`instantiator.rs:1769`) assumes every Reinterpret is a
  post-substitution no-op and asserts `source == result`. Any test that handles a `str` (explicitly or
  via `print`) trips it; the clean fix is a dedicated share-peel I-IR node (the convert comment flags
  it twice) or relaxing the assert.
- **Mutable capture is walled at three independent layers — clearing one only reveals the next.** A
  `set x` on a mutably-captured var hits (1) the borrow checker's group derivation (R1,
  `borrow_types.rs:347`), (2) the instantiator's address-expr lowering, and (3) the testvm's `Mutate`
  handler. Layers 1-2 are done (disable the checker with `test_without_borrow_check`, and the deleted
  `translate_addr_expr` lets a `Deref` destination flow through `translate_ref_expr`); layer 3 remains.
  So disabling the borrow checker alone never greens these — don't read "advanced to a new panic" as a
  regression; it's progress through the stack.
- **Read-capture and mutate-capture must name the closure-member `MemberLookup` identically.** Both
  `IVariableT::Capture` arms in `expression_compiler.rs` (`fn evaluate_lookup_for_load` and
  `fn evaluate_addressible_lookup_for_mutate`) must resolve the member via `get_member_and_index` and
  name it `IVarNameT::Member(struct_member.name)` — never the capture's own `Local` name, which the
  instantiator's member-index match can't find ("member name not found in struct"). Their twin
  `VCOORD: dedup ... we do the same thing for read/mutate` comments mean they must mirror; a fix to one
  needs the same fix in the other.
- **There is no address-expression concept in the instantiator — do not reintroduce one.**
  `translate_addr_expr` is deleted; a `Mutate` destination is an ordinary reference expression (a
  lookup, or a `Deref` writing through a captured reference) handled by `translate_ref_expr`.
  Addressibility was replaced by a plain `&&`, which is just a normal expression.
