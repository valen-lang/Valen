# exp-1-wipbx handoff

## Current state

On `main` — this branch's work is merged; `exp-1-wipbx` and `origin/main` share a tip. The branch is
backend-enabled (past the onion arc): `src/lib.rs` links every module including `backend_ffi`,
`pass_manager`, `end_to_end_tests`, `integration_tests`. There is no `no_backend`-unlinked state here
anymore; `--features no_backend` still exists as a feature but the default/gate build carries the C++
backend.

Green. The gates (from `fire-commit-config.toml`), with last measured counts — re-run to refresh:

- `cargo nextest run --manifest-path Cargo.toml` (native) — 813 passed.
- `VALE_TEST_BACKEND=wasi cargo nextest run --manifest-path Cargo.toml` — 813 passed.
- `cargo test --manifest-path Cargo.toml --lib --features rust_interop` — 858 passed (gate for changes
  under `src/typing/rust_interop/**`; needs `rustc-dev` on the pinned nightly).

## Active project: seal the postparse interner

Plan: `~/.claude/plans/please-plan-out-sealing-zany-karp.md`. Pattern mirrored from the typing pass's
`MustIntern` (arcana `docs/arcana/SealedInternedConstruction-SICZ.md`, @SICZ). Ratified in
`src/typing/typing-design.md` "Names": *"An imprecise name must be interned, always. Even the one in
the declaration name."*

**Goal.** Make a canonical interned reference (e.g. `&'s CodeNameS`) impossible to forge, so
`ptr_eq`/`canonical_ptr` is sound. A private witness field on the payload closes the `alloc`/`'static`
forge routes, because the payload value then can't be constructed outside the interner module.

**Mechanism.** `pub struct ScoutInterned(())` in `src/scout_arena.rs` (private unit field). A sealed
canonical payload in `src/postparsing/names.rs` carries `pub _must_intern: ScoutInterned`, filled only
inside the private `alloc_*_canonical` helpers. Dual-enum split: a witness-free `*ValS` key struct
(`CodeNameValS { name }`) that callers build, and the witnessed canonical (`CodeNameS { name,
_must_intern }`) that `alloc_*_canonical` produces. To get a canonical `&'s` payload directly, call the
`intern_*` helper — `fn intern_code_name` in `scout_arena.rs` returns `&'s CodeNameS`; one exists per
sealed payload.

**Sealed:** the entire imprecise-name hierarchy — every `IImpreciseNameValS` variant plus the
function-imprecise payloads. Count: `grep -c 'pub _must_intern' src/postparsing/names.rs`.

**Remaining (still in scope):**
- **Close the `alloc<T>` forge hole.** `fn alloc<T>` in `src/scout_arena.rs` is still fully generic (its
  `// DO NOT SUBMIT` marker was removed, but the hole stands). It can no longer forge a sealed payload
  (constructor unnameable) but forges every unsealed one. Closing it is only safe once all interned
  postparse payloads are sealed — then `alloc(RuneValS{..})` won't compile and generic `alloc` can stay
  for the ~200 non-interned call sites (AST/type-syntax nodes) that legitimately need it.
- **Seal Phase 2:** runes (~55 `IRuneS` payloads; some already have a private `lid`+`new()`) and the
  non-function `INameS` declaration-name payloads (`TopLevelStructDeclarationNameS`,
  `TopLevelInterfaceDeclarationNameS`, `LambdaStructDeclarationNameS`, `ExportAsNameS`, `LetNameS`,
  `ImplDeclarationNameS`, `GlobalFunctionFamilyNameS`, …). Function declaration names are NOT in this
  list — they are now identity, not interned (see below).

## What's next

- **Debug hides the witness.** Only `CodeNameS` has a hand-written `Debug` (in `names.rs`) that omits
  `_must_intern`; every other sealed payload derives `Debug`, so `_must_intern: ScoutInterned(())` leaks
  into any humanized-name golden the moment one renders. Hand-write `Debug` (a small macro would make
  each a one-liner) as they surface or before Phase 2.
- **Per-payload seal recipe:** add `_must_intern` + a `*ValS` key in `names.rs`; point the Val enum
  variant at the `*ValS`; add/update the `intern_*` helper; fill the witness in the `alloc_*_canonical`
  arm; then fix construction sites (`Val::X(FooS{..})` -> `Val::X(FooValS{..})`, canonical sites ->
  `intern_foo(..)`) and destructuring patterns (add `..`).
- **Optional:** a `compile_fail` test proving `CodeNameS { .. }` / `scout_arena.alloc(CodeNameS{..})`
  no longer compile outside the interner; convert the 3 live `ptr_eq` sites in
  `anonymous_interface_macro.rs` to `==`.

## Name model (settled background)

Both variable and function names now split into a **declaration** side (identity, built directly, not
interned, @WVSBIZ) and an **imprecise** side (interned lookup key):
- Variables: `IVarDeclarationNameS` embeds an interned `&'s <ImpreciseNameS>` + a `lid`; typing's
  `LocalNameT`/`MemberNameT` are `{ imprecise_name: &'s CodeNameS, life }` (life = the lid moved into
  LIFE space via `LocationInFunctionEnvironmentT::from_lid`; a declaration's life *is* its lid, no `.0`).
- Functions: `IFunctionDeclarationNameS` is identity (not interned) — built directly and wrapped in
  `INameS::FunctionDeclaration` (mirroring `INameS::VarName`), each variant carrying `code_location` +
  a `lid` and embedding its interned imprecise name; `imprecise_name()` returns the new interned
  `IFunctionImpreciseNameS`. `FunctionName`/`ConstructorName` reduce to a shared `CodeNameS` spelling
  (a call resolves via `IImpreciseNameS::CodeName`, since a use-site can't know it names a function),
  `LambdaDeclarationName` reuses the empty marker, and `ForwarderFunctionDeclarationName` wraps its
  inner via `ForwarderFunctionImpreciseNameS`.

The instantiator's `translate_var_name` (`src/instantiating/instantiator.rs`) is the lowering boundary:
it humanizes each imprecise name to a `StrI`, so the instantiated/metal names carry a lowered `StrI`
and the backend is decoupled from every frontend name variant — which is why the name-model change
needs no backend-code edits. Member lookup is by spelling (`get_member_and_index` in
`src/typing/ast/citizens.rs`).

## Lessons Learned

- **The backend is decoupled from postparse names by the instantiator's `StrI` humanization.** A
  frontend name-model change (sealing, de-interning) ripples through postparse + typing but not into
  `backend_ffi`/`pass_manager`/codegen — they read the lowered `StrI`, not `IFunctionDeclarationNameS`.
- **A sealing witness leaks into `Debug`.** `derive(Debug)` on a witnessed payload prints
  `_must_intern: ScoutInterned(())`, churning humanized-name goldens. Hand-write `Debug` to omit it.
- **`impl<'s> fmt::Debug for T` trips the harness's "ambiguous definition" check** (reads `fmt::Debug`
  as defining `fmt`). Import the trait (`use std::fmt::{self, Debug, Formatter};`) and write `impl Debug`.
- **A destructuring pattern with a trailing `..` survives a new private field**; only exhaustive
  field-listing patterns break. Prefer `Foo { a, .. }` in matches to make sealing cheap.
- **`IVarDeclarationNameS` (identity) can still live inside the interned `INameS` as `VarName` — built
  directly, bypassing `intern_name`.** That precedent is how function declaration names de-interned
  without leaving `INameS`; look there before assuming a name must be interned to be an `INameS`.
- **WIPBX auto-allows `git add <paths> && git commit` on a `wipbx` branch but hard-blocks
  `git commit --amend`.** To rewrite the last commit's message, `git reset --soft HEAD~1` then re-commit
  fresh. AFEOX blocks AI edits to non-`.rs`/`.md`/`.cpp`/`.c`/`.h`/`.vale` files (e.g. `.toml`).
- **`main` can advance past this branch's premises.** This branch was built on the `no_backend` onion
  state; `main` re-linked the backend underneath it, so the onion `lib.rs`/gate changes were stale and
  dropped on rebase. Re-check `origin/main`'s direction before assuming a branch-local invariant holds.
