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
- **Struct wrapping a collection** — the `domino-glyphs` case: `Domino { glyphs: HashMap<i32, Glyph> }`
  (the map field stays opaque, so its generics/bounds never reach the importer), driven through a
  `&mut self` mutator, a `&self` method returning a **borrow of a held value bound to a local**, and a
  field accessor.
- **Toolchain on the fork** — the repo pins `rustc-fork` (the Vale rustc fork with `per_instance_mir` and
  `rustc-dev` in its sysroot); see the handoff and `docs/build-compiler.md`.

## Forward

### Instantiator ↔ rustc collector: the inversion — BUILT (Milestone M reached)
Milestone M is reached: rustc's mono collector drives Vale's instantiator through `per_instance_mir`,
and every Rust leaf a program transitively calls resolves to a real rustc `(DefId, GenericArgs)` and is
reified for the collector. The as-built state is in the handoff (`docs/handoffs/rust-interop-handoff.md`,
"The instantiator inversion") and `src/instantiating/rust-interop-design.md` (Details). What is **not**
done: emitting the real Vale bodies (`fill_extra_modules` / borrowed-mode codegen, exp-3's LLVM port),
so nothing runs; and the after-`fill_extra_modules` query overrides (partition filter,
`deduced_param_attrs`). The build order below records how M was reached.

Under `rust_interop`, rustc's mono collector drives, and
`per_instance_mir` runs our monomorphizer. The monomorphizer runs its ordinary queue-drain. A Rust
callee cannot be instantiated, so it is filtered out of the drain and collected as a request (the
"queue filter"). Each `per_instance_mir` call adds one exported function, drains, and returns that
function's collected Rust requests to rustc as `ReifyFnPointer` casts. The non-interop instantiator is
unchanged: it self-drives the whole program and never touches Rust. Same substrate, different driver
(architecture §2.10, §13.7; the Harmonious reference is at `/Volumes/V/Harmonious`).

**Milestone M** is the first goal: rustc's collector fires `per_instance_mir`, which runs the
monomorphizer and returns its Rust requests. Backend codegen may panic until exp-3's LLVM port lands
(see "Running it"), so M reaches "rustc drives our monomorphizer" without producing a runnable artifact.

`per_instance_mir` lives in a new `src/instantiating/rust_interop/` submodule, parallel to
`src/typing/rust_interop/`. It holds an ouroboros `Mutex<InstantiatorState>` that owns the arenas,
interners, and `HinputsT`, and also holds the borrowing `InstantiatorI` and `monouts`. State therefore
persists across calls (memoization), and the lock serializes rustc's parallel collector. When the
feature is off, that state does not exist and the arenas stay stack locals. This solves the
arena-lifetime concern (the instantiator's borrows must outlive the callback that made them), so no
whole-compiler Session-scoped arena migration is needed for M.

Build order. `[core]` is `src/instantiating/` proper and needs approval; `[sub]` is a `rust_interop/`
directory, the driver bin, or `lib.rs` externs.
1. `[core]` **The queue filter**: an `is_rust_backed` arm in `translate_prototype`
   (`instantiator.rs:947-1003`) that keeps a Rust callee out of the translate-body path and collects it
   as a request. This is the one change to `src/instantiating/` proper. Verifiable now via
   `get_monouts()`, and it un-ignores the two probes.
2. `[core]` **Drivability**: give the new submodule crate-internal visibility into the queue-add, the
   drain, and the request-read on `InstantiatorI` and `InstantiatedOutputsI`. Visibility only;
   `translate_program` stays as it is.
3. `[sub]` **Link the codegen and mono externs** (`rustc_codegen_ssa`, `rustc_codegen_llvm`,
   `rustc_monomorphize`) and name the fork's `per_instance_mir` query.
4. `[sub]` **Driver**: return `Compilation::Continue` and install `override_queries` in `config()`,
   marker-gated so pure-Rust crates stay byte-identical. No CodegenBackend override (Harmonious's
   wrapper is vestigial).
5. `[sub]` **`per_instance_mir` plus the stateful ouroboros `Mutex<InstantiatorState>`** in the new
   submodule. Lock, add this exported function, drain (instantiating Vale functions and filtering out
   Rust ones), collect the Rust requests, and build the `ReifyFnPointer` plus `Unreachable` body.
6. `[sub]` **Rust request to `(DefId, GenericArgsRef)`** via `oracle.resolve` and
   `GenericArgs::for_item` (lifetime slots `re_erased`).
7. `[sub]` **Minimal but real `vale-stub-gen`** (marker plus `#[vale::emit_consumer_body]` bodies) so
   exported Vale items get the DefIds the collector walks.

The LLVM and C++-backend emission half (`fill_extra_modules`, borrowed-mode codegen) is a separate
effort (exp-3's). The inversion above is the frontend half and does not depend on it.

### Tier 2 enums — matchable variants
Destructuring `Some`/`None` and constructing them. Per enum this needs: an `oracle.variants` query (does
not exist), one synthesized `StructS` per variant (payload as members, no longer opaque), one
`ImplS`/`ImplT` per variant registered through `add_impl` (the load-bearing wiring — the `sealed` flag
alone gives the interface no variants), and **interface downcast**, which the compiler currently
hard-errors (`CantDowncastToInterface`, "…yet"). So Tier 2 depends on interface downcast landing in core
first, and is substantially larger than Tier 1.

### Running it (tier-2 codegen)
After the inversion hands rustc the Rust leaves, actually running a program still needs the outbound
`GenericArgs` reconstruction (rebuild the full `[i32, Global]` from the lossy Vale arg list to hand back
to rustc), the extern-ABI link on synthesized types, and the C++-backend emission half:
`fill_extra_modules` plus borrowed-mode codegen, which is exp-3's LLVM-port effort. Two more query
overrides join at this point: the partition filter (`collect_and_partition_mono_items`, which removes
consumer items before LLVM) and `deduced_param_attrs` (which closes a silent-UB vector from rustc
analyzing the `unreachable!()` stub body). All of this is downstream of the inversion.

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
- The reference implementation for the inversion is Harmonious (`/Volumes/V/Harmonious`); mirror its
  current code, since its architecture doc lags the code in places. Approach A (Instance-keyed
  `per_instance_mir`, Vale substitutes) is the proven keying; Harmonious tried Approach B
  (`optimized_mir`, rustc substitutes) and reverted.
- Do not build the mechanisms Harmonious retired: the `CodegenBackend` wrapper (install the
  `fill_extra_modules` hook via `config()` instead), a `symbol_name` override (read
  `tcx.symbol_name`), `optimized_mir` (that is Approach B), `mir_shims`, `codegen_fn_attrs` /
  `AvailableExternally` linkage (use the partition filter), and non-generic per-call wrapper functions
  (a Rust callee is a leaf).
