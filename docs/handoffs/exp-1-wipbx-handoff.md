# exp-1-wipbx handoff

## Current state

Green. Regenerate:

- Build: `cargo build --lib --features no_backend` (exit 0, 9 pre-existing warnings).
- Tests: `cargo test --lib --features no_backend` (789 passed, 0 failed, 60 ignored).

`src/lib.rs` linkage: `postparsing`, `scout_arena`, `typing`, `instantiating`, `testvm` are live.
`backend_ffi`, `pass_manager`, `integration_tests`, `end_to_end_tests` are commented out (the onion /
no_backend arc, not sealing). The `TEMP-UNLINK-SEALING` comment at `src/lib.rs:27` is stale now that
typing is re-linked; delete it next time that region is touched.

## Active project: seal the postparse interner

Plan: `~/.claude/plans/please-plan-out-sealing-zany-karp.md`. Pattern mirrored from the typing pass's
`MustIntern` (arcana `docs/arcana/SealedInternedConstruction-SICZ.md`, @SICZ). Design ratified in
`src/typing/typing-design.md` "Names": *"An imprecise name must be interned, always. Even the one in
the declaration name."*

**Goal.** Make a canonical interned reference (e.g. `&'s CodeNameS`) impossible to forge, so
`ptr_eq`/`canonical_ptr` is sound. Two forge routes exist today: `ScoutArena::alloc<T>` (carries a
`// DO NOT SUBMIT` comment) and `'static`/const promotion. A private witness field on the payload
closes both, because you then can't construct the payload value at all outside the interner.

**Mechanism (proven end-to-end on the imprecise-name hierarchy).**
- Witness: `pub struct ScoutInterned(());` in `src/scout_arena.rs` (private unit field, so only that
  module can write `ScoutInterned(())`).
- Each sealed canonical payload in `src/postparsing/names.rs` carries `pub _must_intern: ScoutInterned`,
  filled only inside the private `alloc_imprecise_name_canonical` (and siblings) in `scout_arena.rs`.
- Val/canonical split: the interner's Val key is a separate witness-free struct (`CodeNameValS { name }`)
  and the canonical is the witnessed one (`CodeNameS { name, _must_intern }`). Callers build the ValS;
  `alloc_*_canonical` converts ValS -> sealed canonical.
- To obtain a canonical `&'s` payload directly (e.g. a declaration's `imprecise_name` field), call the
  `intern_*` helper, e.g. `scout_arena.intern_code_name(name) -> &'s CodeNameS` (`scout_arena.rs:196`).
  Helpers exist for every sealed imprecise payload (`intern_iterable_name`, `intern_magic_param_name`, ...).

**Sealed so far** (9 imprecise-name payloads carry `_must_intern`, `src/postparsing/names.rs`):
`CodeNameS`, `ConstructingMemberImpreciseNameS`, `IterableNameS`, `IteratorNameS`,
`IterationOptionNameS`, `WhileCondResultNameS`, `MagicParamNameS`, `DesugaredParamNameS`,
`ClosureParamImpreciseNameS`. All 11 `IVarDeclarationNameS` payloads now hold their imprecise name by
interned `&'s` ref (Phase 0.5 done). Recount with `grep -c 'pub _must_intern' src/postparsing/names.rs`.

**The project is NOT finished. Remaining, all in scope:**
- **Close the `alloc<T>` forge hole.** `src/scout_arena.rs:101-104` still has `pub fn alloc<T>` under a
  literal `// DO NOT SUBMIT people can use this to forge things that were supposed to be interned`. It
  can no longer forge the 9 sealed payloads (their constructor is unnameable), but it still forges every
  unsealed one. Closing it (restrict to non-interned types, or route interned allocation only through
  the `alloc_*_canonical` helpers) is the headline loose end.
- **Seal the rest of the imprecise-name hierarchy** (~13): the empty markers `SelfNameS`,
  `ArbitraryNameS`, `PlaceholderImpreciseNameS`, `PrototypeNameS`, `LambdaImpreciseNameS`; the shallow
  ones `LambdaStructImpreciseNameS`, the `AnonymousSubstruct*ImpreciseNameS`, the `Impl*ImpreciseNameS`;
  `RuneNameS`, `AnonymousSubstructMemberNameS`.
- **Seal Phase 2:** runes (~55; 6 already have a private `lid`+`new()`), `INameS` (~11),
  `IFunctionDeclarationNameS` (6).

## What's next

- **Debug hides the witness.** Only `CodeNameS` has a hand-written `Debug` (`names.rs:1416`) that omits
  `_must_intern`; the other sealed payloads still derive `Debug`, so their `_must_intern: ScoutInterned(())`
  will leak into any humanized-name golden the moment one appears. Give them the same treatment (a small
  macro would make each a one-liner) as they surface or before Phase 2.
- **Continue Phase 1 / Phase 2** per the plan, one payload at a time. The per-payload recipe is: add
  `_must_intern` + a `*ValS` key struct in `names.rs`; point the Val enum variant at the `*ValS`; add /
  update the `intern_*` helper; fill the witness in the `alloc_*_canonical` arm; then fix construction
  sites (`Val::X(FooS {..})` -> `Val::X(FooValS {..})`, canonical sites -> `intern_foo(..)`) and
  destructuring patterns (add `..`).
- **Phase 0 forge fixes** (independent, any time): route `src/typing/rust_interop/declarations.rs:508`,
  `src/typing/macros/struct_constructor_macro.rs` (the `ConstructorNameS` site),
  `src/typing/compiler_error_humanizer.rs:134`, and the 2 test sites in
  `src/typing/test/compiler_solver_tests.rs` through the interner; move the `SELF_IMPRECISE_NAME`
  static into the interner.
- **Optional:** a `compile_fail` test proving `CodeNameS { name }` / `scout_arena.alloc(CodeNameS {..})`
  no longer compile outside the interner; convert the 3 live `ptr_eq` sites in
  `anonymous_interface_macro.rs` to `==`.

## Name-model reshape (background, settled)

Separate from sealing but interleaved with it. `LocalNameT` / `MemberNameT` (typing) are now
`{ imprecise_name: &'s CodeNameS, life: LocationInFunctionEnvironmentT }` (was `{ name }`). A
declaration's life is its LID moved into LIFE space via `LocationInFunctionEnvironmentT::from_lid`
(a declaration's life *is* its lid, no `.0`). The instantiator's `translate_var_name`
(`src/instantiating/instantiator.rs`) is the lowering boundary: it humanizes each imprecise name to a
`StrI` so `LocalNameI`/`MemberNameI` carry a lowered `StrI` and the backend is decoupled from the
frontend's name variants. Member lookup is by spelling (`get_member_and_index` in
`src/typing/ast/citizens.rs` compares imprecise names, never full lid-bearing identity).

## Lessons Learned

- **A sealing witness leaks into `Debug`.** `derive(Debug)` on a witnessed payload prints
  `_must_intern: ScoutInterned(())`, which churns every humanized-name golden. Hand-write `Debug` to
  omit it when sealing a payload that reaches humanized output.
- **`impl<'s> fmt::Debug for T` trips the harness's "ambiguous definition" check** (it reads `fmt::Debug`
  as defining `fmt`, colliding with `fn fmt`). Import the trait (`use std::fmt::{self, Debug, Formatter};`,
  which Guardian shield UUSNNCBX also requires) and write `impl<'s> Debug for T`.
- **A destructuring pattern with a trailing `..` survives a new private field**; only exhaustive
  field-listing patterns break. Prefer `Foo { a, .. }` in matches to make sealing cheap.
- **Separate mechanical fallout from in-flight logic before mass-editing.** When re-linking a red pass,
  a payload-shape change (mechanical: `CodeNameS`->`CodeNameValS`) and a signature refactor (logic:
  `from_lid` taking `&LocationInDenizen`) both surface as `E0308`; fix only the former unless asked.
- **Architect preference: declarations are identity, never interned (@WVSBIZ); but the imprecise name
  a declaration carries IS interned, always.** These two coexist: the declaration is built directly,
  its `imprecise_name` field is an interned `&'s` ref.
