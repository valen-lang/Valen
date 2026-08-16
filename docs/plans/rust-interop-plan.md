# Rust Interop — forward plan

Goal: usable Rust interop — a Vale program that names real Rust types (`Vec`, `Option`, …), typechecks
against live rustc, and eventually runs. Current state and mechanisms live in the handoff
(`docs/handoffs/rust-interop-handoff.md`); the design is in
`docs/architecture/vale-rust-interop-architecture.md`. This doc is the forward roadmap. Each item below
becomes its own RFIGA plan (`docs/skills/tdd.md`) when it is actually picked up.

## Done (tier-1 typechecking, uncommitted or committed on the working branch)

- **Vec end to end** — borrow-receiver methods (`&self`/`&mut self`), the fixed-impl-param constructor
  (`Vec.new<int>()`), `usize` as a Vale primitive, and real `Vec<int, Global>` exercising
  `new`/`push`/`len`/`pop().unwrap()`/scope-end drop.
- **Opaque enums (Tier 1)** — `Option`/`Result`-shaped enums import as sealed interfaces; you can receive
  one, call its inherent methods, pass it, and drop it. No variants.

## Forward

### Tier 2 enums — matchable variants
Destructuring `Some`/`None` and constructing them. Per enum this needs: an `oracle.variants` query (does
not exist), one synthesized `StructS` per variant (payload as members, no longer opaque), one
`ImplS`/`ImplT` per variant registered through `add_impl` (the load-bearing wiring — the `sealed` flag
alone gives the interface no variants), and **interface downcast**, which the compiler currently
hard-errors (`CantDowncastToInterface`, "…yet"). So Tier 2 depends on interface downcast landing in core
first, and is substantially larger than Tier 1.

### Running it (tier-2 codegen)
Everything today is tier-1 typechecking; nothing executes. Making `v.push(42)` a real call needs the
outbound `GenericArgs` reconstruction (rebuild `[i32, Global]` to hand back to rustc), the extern-ABI link
on synthesized types, and actually invoking the Rust code. Large, cross-cutting, downstream — the milestone
past typing.

### Broaden the type surface
- **Other scalars** — `u8`..`u64`, `i8`..`i16`, `f32`/`f64` still decline. Follow the `usize` template
  (`KindT::USize`) or reconsider them as a family; needed for `Vec<u8>` and byte APIs.
- **`str` / `&[T]` / `dyn`** decline as `Unsized` — harder (unsized), needed for `String`/`&str`.

### Housekeeping and value semantics
- Confirm the `std` re-export import alias: `import rust.std.vec.Vec` should resolve to the same item as
  `rust.alloc.vec.Vec`.
- To *use* a `Vec`'s contents: indexing (`v[i]` → `Index::index` returning `&T`), iteration, and
  operators/comparison on `usize` (which has none today).

## Notes for whoever picks this up

- The three interop-specific core edits (the `precompile_interface` hook, the `RustImportSeed` seed match,
  the `compile_interface_core` skip) are the pattern for any new denizen family: mirror the struct path and
  cfg-guard it. Core changes need explicit approval; `src/typing/rust_interop/` does not.
- The opaque-enum win rides on inherent methods (`unwrap`/`is_some`/`map`), which arrive via
  `inherent_impls` with no variant work — that is why Tier 1 is useful before Tier 2 exists.
