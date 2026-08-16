# Rust Interop — handoff

The `rust_interop` feature (`src/typing/rust_interop/`, behind `--features rust_interop`) makes a Vale
program typecheck against real Rust items read from a live rustc `TyCtxt`. Everything here is
**tier-1 typechecking only** — nothing runs yet; a Vale program that uses `Vec` compiles but does not
execute. The design source of truth is `docs/architecture/vale-rust-interop-architecture.md`; the
forward work is `docs/plans/rust-interop-plan.md`.

## State (regenerate, don't trust stale)

Build/test from the repo root — the crate is flat now (`FrontendRust/` is retired):

- default: `cargo test --manifest-path ./Cargo.toml --lib`
- interop: `cargo test --manifest-path ./Cargo.toml --lib --features rust_interop`

Read the counts from `grep "test result"` — both suites are green, and the numbers move as cases are
added, so a hardcoded figure rots. The interop suite needs the rustc-dev nightly component
(`rustup component add rustc-dev --toolchain <the pinned nightly>`) and is invisible to CI and the
fire-commit gate, so run it by hand. A `cargo clean` is required after any repo move: fixture paths come
from `env!("CARGO_MANIFEST_DIR")`, baked at compile time, so a stale artifact loads fixtures from the old
path and every disk-reading test fails.

## What imports and typechecks today

- **Structs** — opaque import (`synthesize_extern_struct`), with methods (`&self`, `&mut self`, and
  by-value `self`), associated functions called type-prefixed (`Counter.new()`), generic types
  (`Holder<int>`), and a synthesized scope-end drop.
- **`usize`** — a Vale primitive `KindT::USize(USizeT)`, distinct from `int`/`i64`. Other unsigned widths
  and floats still decline.
- **Enums** — opaque sealed interfaces (`KindT::Interface`) via `synthesize_extern_interface`. You can
  receive one, call its inherent methods, pass it, and drop it. Variants are **not** represented: no
  matching `Some`/`None`, no constructing them.
- **Real `std`** — `import rust.alloc.vec.Vec` + `import rust.alloc.alloc.Global` +
  `import rust.core.option.Option`: `Vec.new<int>()`, `v.push(42)`, `v.len()`, `v.pop().unwrap()`, and a
  scope-end drop all typecheck against live rustc. Only the called methods synthesize; the rest of `Vec`'s
  ~150 methods stay id-only (the laziness payoff).

## How a Rust type crosses (fn = symbol)

- An `import rust.crate.mod.Item` resolves through `oracle.resolve_import` → `ResolvedName`.
  `Compiler::evaluate` loops `program.imports` and calls `declare_rust_import` per import, which returns
  an env entry plus an optional `RustImportSeed` — a `StructS` for a struct, an `InterfaceS` for an enum —
  that the loop seeds into the postparsed cache.
- A struct lowers to `KindT::Struct`, an enum to `KindT::Interface`; the branch is in
  `TyCtxtOracle::type_kind`, keyed on `ItemKind::Enum`. The crate-qualified path names the item's
  **canonical** crate (`Vec` is `rust.alloc.vec.Vec`, not `rust.std.vec.Vec`).
- A method or drop is an id-only lazy entry in the type's outer env (`rust_method_entries`), synthesized
  on first call by `create_postparsed_function`, which re-resolves the owner by name (no offset trick). A
  `&self`/`&mut self` receiver is a `ValeSigType::Borrow` emitted as a `BorrowRefSR` in the parameter's
  @PFVSZ outer-ref bucket.

## The interop-specific core touch-points (design + code)

Three edits in the core typing pass exist solely for interop. Each mirrors existing struct code and is
`#[cfg(feature = "rust_interop")]`-guarded, so a normal build is byte-identical to before:

- the `rust_method_entries` hook in `precompile_interface` (`struct_compiler.rs`) — attaches an enum's
  methods/drop, twin of the one in `precompile_struct`.
- the `RustImportSeed` match in `Compiler::evaluate` (`compiler.rs`) — seeds a struct **or** interface.
- the `is_rust_backed` skip in `compile_interface_core` (`struct_compiler_core.rs`) — keeps Rust methods
  out of the interface vtable, twin of the one in `compile_struct_core`.

Everything else is under `src/typing/rust_interop/` (fair to edit). The rest of `src/typing/` is
human-edit-only and any change there needs explicit approval.

## Governing invariant

Whether a postparsed denizen exists must be undetectable to callers: the only operations are "ask an
environment what it holds" and `get_or_create_postparsed_*` by id (always returns, building on a miss).
A read that memoizes is indistinguishable from a pure read, which is what makes lazy synthesis clean.
The `// VCOORD` on the sealed tables in `compiler_outputs.rs` records the enforcement plan.

## Lessons learned

- A Rust method's receiver borrow (`&self`) splits per @PFVSZ: the argument binds to the **value** rune,
  and the borrow concludes a separate **full-type** rune. Wiring the borrow onto the argument rune makes
  the peeled receiver fail `KindIsNotBorrowRef`.
- The "arity underflow" for `Vec::new` is self-inflicted by over-specifying the call. `new` has one own
  generic (`T`; the impl pins `Global`), so `Vec.new<int>()` / `Vec<int>.new()` supply one arg and never
  underflow — `Vec<int, Global>.new()` is what breaks it. The full arity is written only when `Vec` is
  named *as a type*; there is no default-generic-param support and none is wanted.
- Putting a Rust type's methods in its outer env force-compiles them unless the citizen-compile loop skips
  `is_rust_backed` — true for **both** structs (`compile_struct_core`) and interfaces
  (`compile_interface_core`). Do not remove either skip.
- A manufactured drop recovers its owner from the id's last `init_step`: a `StructTemplate` for a struct
  owner, an `InterfaceTemplate` for an enum owner. `create_postparsed_function`'s drop branch must handle
  both, or an enum's drop vfails.
- Generic scope-end drop resolves now — the generated `drop<T>(Owner<T>)` call infers `T` from the value.
  Do not re-assert "it does not resolve" from stale comments.
- `usize` is a real primitive with no literal syntax and no operators — produce-and-pass only.
- The onion typing work (`docs/handoffs/exp-2-handoff.md`) cleared what interop needed: generic
  substitution through reference wraps and argument types reaching the call-site solve. Interop builds on
  those; do not re-derive them as blockers.
