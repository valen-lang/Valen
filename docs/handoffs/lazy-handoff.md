# Lazy citizen compilation handoff

## Start here next session

**Next: fix the "Generic clone / implicit_clone" ignored cluster** (~12 tests). Find them:
`grep -rn "clone-of-borrow-in-generics\|implicit_clone is not wired" src/integration_tests src/typing/test`
Two blockers, per the ignore reasons:
- **clone-of-a-borrow in generics (11):** clone/bound resolution isn't honest for a borrow — it needs
  `&&T` structural distinctness (a borrow of a borrow is its own type) or a primitive-borrow flip.
- **implicit_clone at let-binding (1):** a user-defined `func implicit_clone(&Ship) Ship` isn't invoked
  at a `let` site — the RHS's Borrow flavor flows into the binding and a later `^` hits vfail at
  `soft_load` (BorrowT + MoveP). Route let-binding through `convert()`'s (Borrow, Own) implicit_clone probe.
Un-ignore each, watch where it lands, fix. All `src/typing/` core → needs "fire core edits".

Measure first (both suites are currently green):
`cargo test --manifest-path ./Cargo.toml --lib --features no_backend integration_tests:: > ./tmp/<name>.txt 2>&1`
`cargo test --manifest-path ./Cargo.toml --lib --features no_backend typing::test:: > ./tmp/<name>.txt 2>&1`
(as of this handoff: integration **186 / 0 / 129**, typing::test **291 / 0 / 27**.)

**Every remaining gap is a parked `#[ignore]` with a greppable reason** (grep the reason to list each):
- **Interfaces** — dispatch/upcast/downcast (`"interfaces branch"`); owned by the interfaces branch.
- **Strings** — the R3 `&@str→&str` share-peel (`"R3"`, `"strings not implemented"`). `fn convert`
  (`convert_helper.rs`) peels a `str` borrow's `ShareRef`, but the instantiator's Reinterpret arm
  (`instantiator.rs:1769`) asserts source == result; the runtime twin is the VM `Share↔Share` transmute
  assert (`heap.rs`).
- **Weaks** — the disabled `weak` builtin module (`"weak"`, `VCOORD: re-enable weaks`).
- **Borrow checking** — borrow-group R1 (`borrow_types.rs:347`) / R5 (`:516`) (`"borrow checker"`); owned
  by another worktree.
- **Generic clone / implicit_clone** — the next target above.
- **migrate builtin** — `src/builtins/resources/migrate.vale` `__vbi_panic()` stub (`"migrate builtin"`);
  gated on borrow-group (uncommenting it regressed the suite to ~215 failures, its `drop_into` lambda
  mutably captures + moves).
- **Immutable / share citizens** — `instantiator.rs:1282` vfail (`"imm/share citizens"`); the minimal
  repro is `struct Marine share {}` fails vs `struct Marine {}` passes (a share-wrapping prototype
  mismatch, isolated to share-citizen instantiation).
- **opaque-extern-drop** — auto-derived drop for extern structs panics (`todo/opaque-extern-drop.md`).
- **Singletons** — Vec/`Deref`/slice discovery, backend borrow-shape arc, array-from-callable
  element-type check, 2 test-isolation flakes.

25 bare `#[ignore]` carry no reason string — worth auditing to complete the picture.

Map (result-producing `foreach`) lowering, closure mutable-capture (all three layers), and inline-struct
overwrite-in-place + write-through-`Deref` are all **landed and green**. The typing repro
`compiler_tests::foreach_with_result_body_compiles` guards Map lowering. Note a by-value functor/lambda
is **not** auto-borrowed into a `&F` parameter — call sites pass it explicitly (`do(&{...})`,
`[]int(10, &Lam())`); that's a source-level workaround, not a core coercion.

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

## Integration test harness (post-ZONION)

`src/integration_tests/tests/` runs on the onion harness (`run_compilation.rs`). The ZONION migration
is complete — every test body is migrated or parked with a real-reason `#[ignore]`. Entry points:
- `test` — builtins loaded into the **root** package (visible without import, the "old way").
- `test_no_builtins` — bare.
- `test_without_borrow_check` / `test_no_builtins_without_borrow_check` — borrow-check off, for tests
  that fail *only* inside the group borrow checker and need to reach later passes.
- `test_multi` — multi-module: the caller supplies its own `Source`s + packages-to-build (one
  `Source::from_code_map` holding several modules), for cross-module `import` tests.

Outputs/eval, all delegating to `InstantiatedCompilation`: `expect_compiler_outputs` /
`get_compiler_outputs` / `get_scoutput` / `get_monouts` / `get_parseds` / `run_primitive_args` /
`eval_for_stdout` / `eval_for_kind_and_stdout` / `eval_for_kind_primitive_args` /
`eval_for_kind_primitive_args_with_stdin` (feeds lines to `__getch`).

`hammer_tests.rs` is deleted (the Hammer pass is gone); `src/end_to_end_tests` is still ZONION-parked.

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

Both suites are **green**; this session's test-migration and Map-lowering work is **uncommitted**
(`git status` for the modified files; `git log --oneline -1` for the tip — nothing committed since).
The old trunk-green / tree-red split is gone: failing tests are no longer temp-`#[ignore]`d and
un-ignored in the tree — each not-yet-supported test now carries a real-reason `#[ignore]` (the
taxonomy is in "Start here"), so the tree is green as-is, not a separate un-ignored red-guide state.

Counts (re-run to refresh — don't trust the numbers, run the command):
- integration: `cargo test --manifest-path ./Cargo.toml --lib --features no_backend integration_tests::` → 186 / 0 / 129.
- typing: `cargo test --manifest-path ./Cargo.toml --lib --features no_backend typing::test::` → 291 / 0 / 27.

Full gates (run before any "fire commit"):
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
  (`{_}` yields `&int`) and any array index that is a reference (`a[__copy_prim(i)]`). A by-value
  struct-functor instance (`Lam()`) is likewise not auto-borrowed and needs an explicit `&Lam()`.
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
- **Mutable capture crosses three independent layers, all now implemented.** A `set x` on a
  mutably-captured var goes through (1) the borrow checker's group derivation (disabled via
  `test_without_borrow_check`; R1 borrow-group is the other worktree's), (2) the instantiator — a
  `Mutate` destination is an ordinary reference expression (a `Deref` writing through the captured
  reference) via `translate_ref_expr`, no address-expr concept, and (3) the testvm's `Mutate` handler,
  whose `Deref`-destination arm (`fn overwrite_through_reference` in `heap.rs`) writes *through* to the
  pointee allocation — a captured primitive overwrites the original local's storage in place, never
  repointing the closure member. General lesson from that hunt: don't read "advanced to a new panic" as
  a regression; it's progress through the stack.
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
- **Map (result-producing `foreach`) lowering evaluates the body twice, on purpose.** A `foreach`
  whose body yields a value per iteration desugars (postparser) to `IExpressionSE::Map`, else to
  `While`. The Map arm (`expression_compiler.rs`) predicts the element type `E` from a throwaway body
  eval to build `List<E>`, then evaluates the body *again* with `list` already declared so a `return`
  inside the body drops the temp list. Single-eval can't do both (need `E` before the list, need the
  list before the body) — the double-eval is the resolution, not an accident.
- **A typing-pass test for a stdlib-shaped feature must be self-contained.** `compiler_test_compilation`
  compiles only the `test` package — no stdlib, no auto-imports. So a test that exercises e.g. Map
  lowering defines its own `List`/`add` and the `begin`/`next`/`isEmpty`/`get` iterator protocol.
  Gotchas: a struct auto-derives its `.drop`, so don't *also* write `func drop(T)` (duplicate-candidate
  panic); and `drop(int)` is NOT intrinsic without builtins — define `func drop(i int){}` if a generic
  bound needs it.
- **`coutputs.functions[0]` is not `main` when builtins are loaded.** Under `test` /
  `test_without_borrow_check`, index 0 is a builtin (e.g. the `Ok` constructor). Look `main` up by name
  (`lookup_function_by_str("main")`), never by position.
