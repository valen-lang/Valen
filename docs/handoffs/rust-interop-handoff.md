# Rust Interop — handoff

**Every session on this work must first deep-read `docs/architecture/rust-interop-design.md`** (`deep-read`,
following its Required reading) — it is the primary design authority; this handoff records what is true of the
code now, that doc records the intended design.

The `rust_interop` feature (`src/typing/rust_interop/`, behind `--features rust_interop`) makes a Vale
program typecheck against real Rust items read from a live rustc `TyCtxt`, and — through the instantiator
inversion below — lets rustc's monomorphization collector drive Vale's instantiator all the way into the
C++ backend, which emits the program's IR into a rustc-lent module at codegen. A Vale binary calling real
Rust functions now **links and runs**: single-symbol emission binds Vale's bodies (entry and each leaf)
under rustc's own mangled names, a partition filter strips rustc's `unreachable!()` placeholders, and a
Tier-2 harness runs the linked executable and asserts its exit code (`seven()` → 7,
`add_two_numbers(20, 22)` → 42). **The full `domino` test now links and runs → 7**
(`rustc_driven_bin_domino_returns_seven`): a Vale program constructs a Rust struct wrapping a `HashMap`,
calls `&mut self`/`&self` methods on it, gets a `&Glyph` back, and returns through it. The whole
aggregate ABI is sourced from rustc (`tcx.layout_of` + `tcx.fn_abi_of_instance`). See "The aggregate
ABI" for how the coercion modes are handled: a large struct crosses by value as an indirect pointer, a
small (≤8-byte) struct as a single register integer (`Cast`), and a two-scalar struct as a register `Pair`.
Wider `Cast`, float-HFA, and pointer-component Pairs stay deferred with no NobiliaV consumer. Rust also calls
back into Vale (a Vale struct implements an imported Rust trait) — see "Reverse direction"; NobiliaV's entire
real windowed program now builds and runs through this. See "The `valen build` pipeline" for compiling and
running a Vale program against real Rust crates. A real **published crates.io crate** now works end to end:
a program depending on `chrono = "0.4"` runs `TimeDelta.seconds(42i64).num_seconds()` → 42 through `valen
build` with no shim — exercising imported `i64` signatures and `#[track_caller]` functions (the hidden
`&Location` ABI arg, @TCHAPZ). See "What imports and typechecks today".
Design
sources of truth: `docs/architecture/vale-rust-interop-architecture.md`,
`src/instantiating/docs/architecture/instantiating-rust-interop-design.md`, the backend's `Backend/backend-design.md` (the C-ABI /
shim-removal direction), the roadmap `docs/plans/rust-interop-plan.md`, and the intended-design authority
`docs/architecture/rust-interop-design.md` (whose target names the tree does not yet carry; see the next
section).

## Design doc vs as-built (naming and unbuilt pieces)

`docs/architecture/rust-interop-design.md` states the intended design; the tree diverges from its target
names and lacks two of its pieces. Tracked here so neither doc has to carry the gap.

- **Callbacks struct.** The design's `ValenRustInteropCallbacks` is two structs today: `ValeCallbacks`
  (typing-only driver, returns `Compilation::Stop`, `src/typing/rust_interop/driver/main.rs`) and
  `DrivenCallbacks` (drives codegen, returns `Compilation::Continue`, `src/typing/rust_interop/drive.rs`).
  Both implement only `config` and `after_expansion`. The `#[cfg(test)]` harness has its own typecheck-only
  `CaseCallbacks` (`harness.rs`), but its former duplicate driven callbacks is gone — `drive_rustc`
  delegates to `run_driven_rustc`, so `drive.rs`'s `DrivenCallbacks` is the only one.
- **Interop state.** The design's `RUSTC_VALENC_INTEROP_STATE` is the thread-local `DRIVER_STATE` (a
  `Cell<*const ()>`) pointing at the `DriverState` struct, both in `src/instantiating/rust_interop/mod.rs`.
- **`after_analysis` is unbuilt.** No callbacks struct implements it and there is no `.vale-cache`; the
  design reserves it as the cache-write point (@CMWAR).
- **No `ValeCodegenBackend`.** The architecture's wrapper over `LlvmCodegenBackend` (arch §5.4 / App. C.1)
  does not exist. Its intended job is **marker-gated activation**: `provide`/`init` install the query
  overrides and Vale runtime init only for crates carrying `__VALE_STUBS_MARKER`, delegating all codegen
  (`codegen_crate`/`join_codegen`/`link`) to the inner backend — it never emits Vale IR itself; the
  `fill_extra_modules` hook does. The tree instead installs overrides unconditionally via
  `config.override_queries` and injects bodies via `set_fill_extra_modules_hook(consumer_fill_modules)` on
  the stock backend (`DrivenCallbacks::config`, `drive.rs`), with no marker gating; the pure-Rust
  pass-through is gated on the crate root's extension in `run_wrapper` instead, and the design doc's
  deviations list ratifies that. A wrapper backend would not remove the global hook either:
  `codegen_crate<B>` is monomorphized with `B = LlvmCodegenBackend`, so only rustc's own LLVM backend can
  answer `fill_extra_modules`. The design-conformant install is a field on `LlvmCodegenBackend` set through
  `Config::make_codegen_backend` (short-term Next #6).
- **Two passes and the pass-1 root.** The design compiles every Valen crate in two passes with an
  imports-only pass-1 root; the code stays one-pass when the pass-2 stub is empty and its pass-1 root already
  carries the marker, `__vale_main`, `__vale_drop`, and the hand-written projections. Short-term Next #4.
- **Opaque wrapper shape.** The design's `__ValeOpaque<const OID: u128>(PhantomData<*mut ()>, PhantomPinned)`
  carries a per-OID `impl Drop` (monomorphized per OID, so static dispatch), and a Valen struct Rust can name
  (exported, or implementing a Rust trait) carries the markers directly with no `__ValeOpaque` field. The code
  has those two markers plus a real `UnsafeCell<()>` for `!Freeze` (`VALE_OPAQUE_DECL` in `stub_gen.rs`,
  the one spelling, with each marker's reason; the harness `fixtures/stub.rs` hand-writes the same) but
  with a 64-bit FNV-1a typeid (`typeid.rs`), projects every
  struct Rust can name as `pub struct <S><F>(__ValeOpaque<HASH>, PhantomData<(F)>)` — a `__ValeOpaque` *field*
  keyed on the struct's name, not inline markers — and routes drops through the `__vale_drop<T>` shim with no
  `Drop` impl. The design's own crate-root flow still writes `struct Ship { contents: __ValeOpaque<OID> }`,
  which contradicts its Opacity section; the architect has not ruled which passage wins for a *named*
  projection. For an unprojected Valen type in a Rust slot the design's bare `__ValeOpaque<OID>` is what the
  code does, sized by the `layout_of` override (see "A Valen struct crosses to Rust by value"). The design's
  comptime-generic-argument-as-integer table has no code yet.
- **Entry symbol capture.** The design shows a general capture — `record_export_symbol(export_name, symbol)`
  run for every exported function in `per_instance_mir`. The tree instead special-cases only the entry:
  `if stub_name == "__vale_main" { *state.entry_symbol.borrow_mut() = Some(tcx.symbol_name(instance)…) }`
  into a single `DriverState.entry_symbol` slot (`src/instantiating/rust_interop/mod.rs`), because the
  entry is the only inbound Rust→Valen crossing today. The general form — every export emitted under its
  rustc-mangled name, the inbound mirror of `FunctionExternI.link_name` — is unbuilt.

## Borrow groups on Rust imports (built; reverse-direction `mut(g)` is inert until enforced)

The borrowing design (`src/typing/docs/architecture/borrowing-design.md`, ratified) forbids a groupless
borrow: after `groupify_function` every borrow reference carries a group, and a borrow return with no
group is a compile error. `synthesize_extern_function` (`declarations.rs`) meets this for the forward
direction — every imported borrow parameter carries an `in g` group, a `&mut` parameter marks its group
`mut(g)`, and a `&self`-tied borrow return carries the descendant `in g...` form. The reverse direction
mirrors `&mut` too: a callback override's `&mut` parameter/receiver renders `&mut`/`&mut self` in the
generated stub (`stub_gen.rs`, read from the override's `mut(g)` effect clause + each borrow's `in g`
region), and `synthesize_abstract_interface_method` (`declarations.rs`) carries `mut(g)` on the abstract
method. **Design-vs-code gap:** that abstract-method `mut(g)` is not yet *enforced* — override matching
(`params_match` in `overload_resolver.rs`) compares group-erased kinds and ignores effects, and the
borrow checker never groupifies an abstract method (`check_function` runs only for a `CodeBody`), so it is
inert until the cross-abstract/override enforcement (exp-2 owns it, core) reads it. The override's own
`in r`/`mut(r)` still borrow-checks its body normally.

## State (regenerate, don't trust stale)

**Developing the compiler uses stock Rust nightly** — there is no `rust-toolchain.toml` pin. A default
build compiles as plain Rust; `build.rs` links a standalone **LLVM 21** for the C++ backend (Homebrew
`llvm@21`, or whatever `$LLVM_CONFIG` names). The Vale rustc **fork** (`github.com/valen-lang/rust` @
`per-instance-mir`, setup in `docs/build-compiler.md`) is used **only** for `--features rust_interop`,
selected explicitly with `+rustc-fork`. The fork builds LLVM 21 **from source as one shared
`libLLVM.dylib`** (its `config.toml`: `download-ci-llvm = false`, `link-shared = true`) so that under
interop the C++ backend and rustc share a single libLLVM (arch §3.6/§5.7 — two libLLVMs in one process is
duplicate-symbol UB); under `+rustc-fork`, `build.rs` finds that shared libLLVM via its sysroot-sibling
derivation. The fork carries exactly two patches, committed on `per-instance-mir` (`git -C ~/rust log
--oneline -1` for the tip): the `per_instance_mir` query (declaration in `rustc_middle`, default `None`
provider in `rustc_mir_transform`, consulted by `collect_items_of_instance` in `rustc_monomorphize`) and the
`fill_extra_modules` hook (`ExtraBackendMethods::fill_extra_modules` in `rustc_codegen_ssa`, a process-global
`OnceLock` plus `ModuleLlvm::new`/`llcx_raw_mut`/`llmod_raw` in `rustc_codegen_llvm`, and the `extra_modules`
plumbing in `back/write.rs`). Everything else interop does to rustc is a stock `Config::override_queries`
override. The fork ships the `rustc_private` libraries + `rust-src`, so interop needs **no** `rustup
component add rustc-dev`. The `rustc-fork` toolchain is linked to the fork's **stage1** sysroot, and stage1
gets its `rustc-dev` rlibs only from a stage2 build: run `./x build` **and then** `./x build --stage 2`, both
from the fork root (or with `--src ~/rust --build-dir ~/rust/build --config ~/rust/config.toml`; from any
other cwd bootstrap builds a fresh LLVM into `./build` under that cwd). After any change to the fork's
`compiler/` sources, rerun both, then `cargo +rustc-fork clean --manifest-path Cargo.toml -p frontend_rust`:
cargo does not fingerprint sysroot crates and the rlib hashes are stable across rebuilds, so it will not
notice on its own. Full recipe in `docs/build-compiler.md`. Build/test from the repo root:

- default (stock nightly): `cargo test --manifest-path ./Cargo.toml --lib`
- interop (fork): `cargo +rustc-fork test --manifest-path ./Cargo.toml --lib --features rust_interop`

Read the counts from `grep "test result"` — both are green and the numbers move as cases are added, so a
hardcoded figure rots. The interop `--lib` runs in the fire-commit gate; CI does not run it yet (no fork
toolchain on the runners). A `cargo clean` is required after any repo move: fixture paths come from
`env!("CARGO_MANIFEST_DIR")`, baked at compile time, so a stale artifact loads fixtures from the old path
and every disk-reading test fails.

## What imports and typechecks today

- **Structs** — opaque import (`synthesize_extern_struct`), with methods (`&self`, `&mut self`, and
  by-value `self`), associated functions called type-prefixed (`Counter.new()`), generic types
  (`Holder<int>`), and a synthesized scope-end drop. A struct may **wrap a std collection** (the field
  never crosses — it's opaque — so the map's own generics/bounds never reach the importer), and a `&self`
  method returning a **borrow of a held value** (`&Glyph`) bound to a **local** resolves — the
  `domino-glyphs` case (`Domino { glyphs: HashMap<i32, Glyph> }`, driven through `&mut self` add / `&self`
  get-by-borrow / a field accessor).
- **Scalars** — Rust `i32` imports as Valen `int`, `i64` as the distinct `i64` primitive, `usize` as
  `usize`, `bool` as `bool`, and `()` as `void`. Both `lower_ty` (`tyctxt_oracle.rs`, enumeration) and
  `vale_type_name` (`declarations.rs`, synthesis) must name this same set — a type one accepts but the
  other can't name `vfail`-panics rather than declining (the i64 trap; see Lessons). Everything else —
  the other signed widths (`i8`/`i16`/`i128`/`isize`), all unsigned widths (`u8`..`u128`, except
  `usize`), and floats — declines with a `CouldNotPostparseReason`.
- **`#[track_caller]` functions** — a `#[track_caller]` Rust fn carries a hidden trailing `&Location` ABI
  argument absent from its type signature (@TCHAPZ). `compute_extern_abi` marks that trailing coercion
  `Coercion::LocationPtr` (detected via `instance.def.requires_caller_location(tcx)`); the backend
  declares a `ptr` param and passes null for it. So panic-on-error / overflow-checked crate APIs (the
  `.expect()`/`.unwrap()`-wrapping constructors ubiquitous in real crates) import and run — verified end
  to end by a real `chrono = "0.4"` dependency (`TimeDelta.seconds(42i64).num_seconds()` builds and runs
  → 42 through `valen build`).
- **Enums** — opaque sealed interfaces (`KindT::Interface`) via `synthesize_extern_interface`. You can
  receive one, call its inherent methods, pass it, and drop it. Variants are **not** represented: no
  matching `Some`/`None`, no constructing them.
- **Traits** — imported as an interface a Vale struct can implement, so Rust can call back into Vale; see
  the reverse-direction section.
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

## The instantiator inversion (rustc drives us)

Under `rust_interop`, instantiation is driven by rustc's mono collector, not by `translate_program`.
The design of record is `src/instantiating/docs/architecture/instantiating-rust-interop-design.md`; the as-built shape:

- **Recording Rust leaves.** A Rust callee reaches the backend as a synthesized `extern`: the typing
  pass wraps every Rust import in an extern function whose body is a single `ExternFunctionCall`. At the
  `ExternFunctionCall` node (in `translate_ref_expr`, `instantiator.rs`) whose `prototype2` is
  `is_rust_backed`, the instantiator *only records the request* — inserting the instantiated `PrototypeI`
  into `monouts.rust_instantiation_requests`. It does **not** build the leaf's `FunctionExternI` there;
  the provider does, once it resolves the leaf and its real symbol is known (see below). The wrapper is
  ordinary Vale that instantiates normally. Three `pub(crate)` seams were extracted from
  `translate_program` (behavior-preserving): `instantiate_exported_function` and
  `drain_instantiation_queue` let a driver run one export, and `assemble_hinputs` finalizes an
  already-drained accumulator into a `HinputsI` (so the driven `monouts` becomes what the backend lowers
  — see "Running a program", no re-instantiation).
- **The provider.** `src/instantiating/rust_interop/` holds the `per_instance_mir` query provider,
  installed via `override_queries` from a `Compilation::Continue` driver. rustc's collector calls it
  for each Vale stub item (a `#[vale::emit_consumer_body]` fn in a `__VALE_STUBS_MARKER` crate); it
  seeds that export, drains the instantiator, resolves each collected request to a rustc
  `(DefId, GenericArgs)`, and returns a synthetic MIR body — a `ReifyFnPointer` cast per Rust leaf
  (which is what queues them) plus `Unreachable`. For each resolved leaf it also computes
  `tcx.symbol_name` and **materializes** the leaf's `FunctionExternI` (with that mangled symbol as its
  `link_name`) into `monouts` — the provider is the single definition point for a Rust extern, since its
  real symbol is knowable only here. The Vale body itself is emitted by the backend under the same
  mangled name (single-symbol, arch §5.2).
- **Request resolution** (`resolve_request`): free function by crate-qualified path
  (`resolve_crate_qualified_path`); method through its receiver type's `inherent_impls` (peeling ref
  wrappers for `&self`/`&mut self`); associated function through the owner named in the id's init path;
  synthesized drop maps to the generic `__vale_drop<T>` shim in the stub (arch §15.7). Vale type args
  lower to rustc `Ty`s (primitives, and Rust-backed citizens to `Adt`s, recursively).
- **State without `'static`.** The instantiator state (arenas, interners, owned `HinputsT`, `monouts`)
  lives in the driver's stack frame with real lifetimes; the provider reaches it through a scoped
  thread-local raw pointer, and the callbacks carry `unsafe impl Send` justified by `run_compiler`'s
  synchronous join (the `std::thread::scope` guarantee). No `'static`, no ouroboros, no leak.

Verified by the `rustc_collector_drives_*` tests (`src/typing/test/rust_interop/cases.rs`), driven by
`run_case_rustc_driven[_full]` in `harness.rs` — green (the composed `domino` case, via
`run_case_rustc_driven_full`, drives rustc's collector + codegen but does **not** emit the Vale backend;
see the Lessons trap).

## Running a program

A Vale binary calling real Rust functions links and runs. Two Tier-2 tests assert it —
`rustc_driven_bin_links_and_returns_seven` (`seven()` → exit 7) and
`rustc_driven_bin_links_and_returns_from_add_two_numbers` (`add_two_numbers(20, 22)` → exit 42), in
`src/typing/test/rust_interop/cases.rs`, driven by `run_case_rustc_driven_and_run` in `harness.rs`
(`drive_rustc` with `crate_type="bin"`, then runs the produced executable and checks its exit code).

- **Compiling a called Rust function.** A *called* Rust function starts as only a postparsed declaration
  (`create_postparsed_function`); nothing would run `make_extern_function` on it, so it had no compiled
  wrapper in `functions`, no `function_extern`, and no `ExternFunctionCall` node — the emitted `main`
  would call an undeclared symbol. Core now compiles it lazily: when the postparse is first created,
  `get_or_create_postparsed_function` (`compiler.rs`) registers the produced `FunctionS` and calls
  `defer_evaluating_function(EvaluateFunction { function_id })`. The deferred-compile drain near end of
  typing (`compiler.rs`) runs `make_extern_function` on it — its wrapper (body =
  `Return(ExternFunctionCall(extern))`) plus the `function_extern` — exactly as a user `extern func` gets
  from the top-level loop (which skips the `rust` package). The drain derives each function's outer env
  from its id in `evaluate_generic_function_from_non_call` (a method's containing id → the citizen outer
  env via `get_outer_env_for_type`; a free function's → the package env via `make_top_level_environment`),
  so `EvaluateFunction` carries only the id. `create_postparsed_function` takes `&CompilerOutputs`
  (read-only): rust_interop only *produces* the `FunctionS`; core owns registering and deferring it.
- **One demand-driven instantiation (no re-instantiation).** The program is instantiated once, driven by
  rustc's collector. `collect_new_rust_requests` (`src/instantiating/rust_interop/`) seeds an export via
  `instantiate_exported_function` and drains, accumulating into the persistent `DriverState.monouts`; the
  `fill_extra_modules` hook (`consumer_fill_modules`, installed by `DrivenCallbacks::config`) then
  finalizes *that* driven accumulator via `assemble_hinputs` → `populate_metal_cache` and calls the
  backend's single entry through `compile(BackendInputs)` (`src/backend_ffi/`). `BackendInputs` is a
  two-variant enum by mode (`Standalone` | `Interop`); the `Interop` variant carries rustc's lent
  `(context, module)` and the entry symbol. The imported-struct layouts and per-extern ABI descriptors
  ride the metal `Program` instead of `BackendInputs` (S14; see "The aggregate ABI"). There is
  no second `translate_program` pass — the driven `monouts` is exactly what the backend lowers.
- **Single-symbol naming.** The entry and each Rust leaf are emitted under rustc's own mangled name
  (`tcx.symbol_name`). The one extern-name map is `FunctionExternI.link_name` — the real callee symbol
  (rustc's mangling for a Rust leaf, `extern_name` for a C extern); `declareExternFunction` binds it
  verbatim for the `rust` package and composes the `vale_abi_` shim otherwise. The entry's mangled symbol
  is threaded through `BackendInputs` into `makeEntryFunction`. Two query overrides in
  `vale_override_queries` make exactly one definition survive at link: `collect_and_partition_mono_items`
  strips the `#[vale::emit_consumer_body]` stub bodies from rustc's codegen, and `deduced_param_attrs`
  returns `&[]` for them (arch §5.2/§5.3/§22.4).
- **The bin stub.** `fixtures/stub.rs` carries a real `fn main() { exit(__vale_main()) }`; under
  single-symbol its `__vale_main()` call resolves to the mangled symbol the backend emits Vale's entry
  under (rustc's placeholder removed by the partition filter), so it links to Vale's body and forwards
  the exit code. `rustc_codegen_emits_vale_bodies_into_borrowed_module` still asserts the earlier
  lib-crate emit path (backend rc 0, rustc exits 0).

## The aggregate ABI

The domino test (`A_STRUCT_WRAPPING_A_HASHMAP_IS_USED_THROUGH_METHODS`, `corpus.rs`, `domino-glyphs`)
links and runs → 7: `d = Domino.new(); d.add_glyph(Glyph.new(7)); d_ref = d.get_glyph(7); return
d_ref.location();`. The scalar `seven()`/`add_two_numbers` binaries need no aggregate ABI (Rust ABI == C
ABI); domino's struct sizes and calling conventions are read from rustc (`layout_of` +
`fn_abi_of_instance`) rather than a hand-rolled classifier. Design of record:
`instantiating-rust-interop-design.md` (proposals S8–S14). How it works:

- **Backend boundary unified (S12).**
  `compile(BackendInputs)` (`src/backend_ffi/`) → one C++ `backend_compile` (`vale.cpp`);
  `compileStandalone` and `compileIntoModuleFromRustc` are `static` internals of `vale.cpp`.
  `BackendInputs` is a two-variant enum by mode; the `Interop` variant carries only rustc's lent
  `(context, module)` and the entry symbol.

- **Opaque-struct sizing and ABI coercion: built (S8–S14), the full domino runs → 7** (the goal:
  `rustc_driven_bin_domino_returns_seven`; the ladder rungs `rustc_driven_bin_method_returns_seven` (Counter,
  by-value aggregate) and `rustc_driven_bin_borrow_self_method_returns_seven` (borrow + drop) also green;
  native + interop suites green via `cargo nextest run`, with and without `VALE_TEST_BACKEND=wasi`).
  Two general, source-agnostic maps ride the core metal `Package`
  (`Backend/src/metal/ast.h`): `structLayouts` keyed by the humanized struct name (beside
  `externNameToKind`), `externAbis` keyed by the humanized prototype name (beside `externNameToFunction`,
  which is also the key `GlobalState.externFunctions` uses, **not** the extern symbol). They cross via the
  builder FFI (`metal_package_builder_add_struct_layout` / `_add_extern_abi`, one `add(key,value)` per
  entry into a C++ `unordered_map`), never a flat array; standalone passes empty maps. The pieces:
    - **Producer** (`rust_interop/mod.rs`, AI-editable): `compute_struct_layouts` (`tcx.layout_of`) and
      `compute_extern_abi` (`tcx.fn_abi_of_instance`, in the `collect_new_rust_requests` resolve loop where
      each leaf's `Instance` exists, accumulated on `DriverState.extern_abis`). `populate_metal_cache`
      (`metal_lowerer.rs`) is handed both maps and calls the builders in its extern-kind / extern-function
      loops.
    - **Consumers** (core): `Unsafe::defineStruct` (`unsafe.cpp`) looks up
      `program->packages[coord]->structLayouts` (a non-aborting `.find`, since builtin structs' packages
      may not be in the program) and builds `[size/align x i{align*8}]`; `buildCallOrSideCall`
      (`externs.cpp`) and `buildBoundarySignature` (`boundary.cpp`) both consult `lookupExternAbi`
      (boundary.cpp, also non-aborting) so the declared signature and the call agree. `getExternalType`'s
      `{i64}` handle and structural `returnNeedsOutParam` survive only as the descriptor-less C-extern
      fallback (see backend-design.md's "ABI Boundaries").
  **`PassMode` coverage:** `compute_extern_abi` maps `Direct` on a pointer-scalar layout
  (`arg.layout.backend_repr` is `BackendRepr::Scalar` with a `Primitive::Pointer`) → `DirectPtr`, other
  `Direct` → `DirectInt(size.bits())`, `Indirect { on_stack: false }` → an indirect pointer to a
  caller-owned copy (an sret out-pointer for a return, a pointer to a spilled copy for a by-value
  argument; @EACBIPZ), a `Cast` with a single integer unit (an ≤8-byte struct crossing as a bare `iN`,
  e.g. `PieceId`'s 8-byte return → `i64`) → `Coercion::Cast(bits)`, `Ignore` → not passed. `DirectInt`
  and `Cast` share one backend mechanism (a struct reinterpreted through an integer-aligned slot): one
  arg branch and one return branch in `buildCallOrSideCall`, which takes its declared param types and the
  sret flag from `buildBoundarySignature` (the same signature `declareExternFunction` declares from), so
  the call site cannot drift from the declaration. A `Pair` (`ScalarPair`, two integer scalars) is handled
  too — two register params for an arg, an `{iN,iM}` aggregate for a return, both directions. Multi-piece
  `Cast` (`[N x i64]`), float-HFA, pointer-component Pairs, and on-stack byval (`Indirect { on_stack: true }`)
  still `panic!`; a leaf that hits one adds its own arm.
  A **`#[track_caller]`** callee's hidden trailing `&Location` ABI arg (@TCHAPZ) is marked
  `Coercion::LocationPtr` rather than a plain `DirectPtr`: `buildBoundarySignature` declares it a `ptr`
  param and `buildCallOrSideCall` synthesizes a null pointer for it (it consumes no Vale argument — safe
  under `panic = abort`). `LocationPtr` rides the existing per-coercion FFI channel (one more `CoercionKind`
  ordinal), so nothing new crosses the metal FFI; the old `abi->args.size() == params.size()` assert in
  `buildBoundarySignature` is now a *non-`LocationPtr`* count check.

  Two boundary subtleties the domino run forced, both in the consumers:
    - **`sret` needs the LLVM `sret` attribute.** An `Indirect` return passes the out-pointer in the
      platform's hidden result register (x8 on aarch64), which LLVM emits only when the first param carries
      the `sret` type attribute. `declareExternFunction` (`function.cpp`) adds it to the declaration and
      `buildCallOrSideCall` (`externs.cpp`) to the call site, both gated on `lookupExternAbi` (interop only;
      the C-extern shim handles its own ABI). Without it, rustc's callee reads a garbage sret address and
      the 48-byte write corrupts the stack, a *crash* that surfaces in the harness as `process_exit=Some(-1)`
      (`.code()` is `None` for a signal death).
    - **A `DirectPtr` owned value must spill.** A borrow is already a pointer, but a *consuming* drop's
      `*mut T` receives an owned inline value; `buildCallOrSideCall`'s `DirectPtr` arm spills it to a slot
      and passes the address (`drop_in_place` runs on it; Vale keeps the stack slot).

## Reverse direction — a Vale struct implements a Rust trait (Rust calls back into Vale)

Rust calls back into Vale through a trait a Vale struct implements — static dispatch, no `&dyn`. Built
end to end: rustc monomorphizes a generic Rust caller over the Vale struct, dispatches into the Vale
override, and the linked binary runs it. Design proposals S1–S3 in
`src/typing/docs/architecture/typing-rust-interop-design.md`; reference `/Volumes/V/Harmonious/toylangc/`
(`case4_sky_impl_rust_trait`, `callbacks_impl.rs`'s drain, `codegen_extern_wrapper` in `llvm_gen.rs`).

**Frontend + monomorphization.** `import rust.X.Trait` synthesizes an `InterfaceS` whose abstract methods
are the trait's (`synthesize_extern_trait`, `declarations.rs`); a Vale struct's `impl` resolves and matches
through the existing override machinery (a missing/wrong override is `CouldntFindOverrideT`); rustc
monomorphizes `run_callback::<MyCb>` with the Vale struct as the type arg and walks to
`<MyCb as Callback>::on_call` (needed `resolve_local_type` for a Vale-struct-as-generic-arg). Tests
`imports_a_rust_trait`, `a_struct_implements_a_rust_trait`, `a_trait_impl_missing_its_override_is_rejected`,
`rustc_discovers_a_valen_trait_impl_callback`.

**Running the callback.** When `per_instance_mir` fires on a Vale codegen target that is not a
`FunctionExportI` (a trait-impl method Rust reached), `collect_callback` (`mod.rs`) instantiates the concrete
override via `translate_prototype` + `drain_instantiation_queue` (no core-instantiator change), computes its
inbound ABI (`compute_extern_abi`, role-agnostic), reifies any Rust leaves the callback body itself calls
(shared `resolve_new_requests`), and records `(rustc symbol, vale name)` on `DriverState.callbacks`. The
backend emits one wrapper per callback under its rustc-mangled symbol (single-symbol) via
`emitInboundCallbackWrapper` (`Backend/src/rust_interop/rust_interop.cpp`), reached from the one core hook
after `makeEntryFunction` (`vale.cpp`). Callbacks sort by symbol (deterministic). The concrete override is
found among same-named `hinputs.functions` by `get_abstract_interface().is_none()` (the abstract interface
method shares the name).

Proven inbound (linked bin runs), tests in `cases.rs`: `rust_calls_back_a_valen_callback_returns_seven`
(→7), `a_valen_callback_takes_a_scalar_arg` (scalar arg →35), `a_valen_callback_receives_a_rust_borrow` (a
shared borrow inbound + the callback calls back out to Rust →5), `a_valen_callback_receives_a_rust_struct_by_value`
(a small integer-`Pair` struct by value →9), `a_valen_callback_returns_a_rust_struct_by_value` (a `Pair`
return →9), and the capstone `rust_owns_a_loop_calling_the_callback` (Rust owns a loop calling the callback
N times →10). A void return and a multi-arg wrapper are implemented, but only single-arg shapes have fixtures.

**The real NobiliaV `on_tick` builds and runs.** `MainLoopCallback::on_tick(&self, w &NobiliaWindow, input
&FrameInput)` — two imported-type borrow params, void return, invoked through a generic *method* caller
(`w.main_loop::<C>(&cb)`) — compiles and runs end to end. NobiliaV's real `driver.vale` (windowed, winit +
wgpu) and its bounded twin `driver_check.vale` (auto-exits after 30 frames → 7) both build through `valen
build` against the real multi-crate `nobiliav` graph and run. Two things make it work:

- **The stub generator projects the Vale struct + its trait impl.** `generate_stub_source` emits a `pub
  struct <S> {}` + `impl <ImportedTrait> for <S> { #[vale::emit_consumer_body] fn <m>(...) { unreachable!() } }`
  per Vale struct that implements an imported trait (see the `valen build` section). That it does so in
  pass 1 is a design-vs-code gap — the design puts every such projection in pass 2 (Next #4). Without it,
  `resolve_local_type(<S>)` finds nothing → the outbound `main_loop::<S>` leaf is ARGS-UNCONVERTIBLE → no
  `FunctionExternI` is materialized → the backend aborts on an undeclared extern (`buildCallOrSideCall` in
  `externs.cpp`). Do not read that backend assert as a backend bug — it faithfully reports a leaf the stub
  never let resolve.
- **The reachable-bounds gather takes no inner env for an imported citizen.** Compiling the synthesized
  interface's abstract-method header, `check_defining_conclusions_and_resolve` (`infer_compiler.rs`) and the
  harvest in `templata_compiler.rs` derive a citizen's function bounds via `resolve_citizen_bounds`
  (`struct_compiler.rs`) — from the postparsed declaration, never `get_inner_env_for_type`. An imported
  opaque citizen used only as a parameter type (e.g. `FrameInput`, never constructed, so never compiled and
  never given an inner env) contributes no bounds instead of `None`.unwrap()ing at `get_inner_env_for_type`.
  Do not reintroduce an inner-env fetch on that path.

Covered by the wired `a_trait_method_with_two_imported_params` (`cases.rs`, →7) — the two-imported-borrow-param
callback via a generic *method* caller. Note the corpus drives hand-written fixture stubs, so it does **not**
exercise `generate_stub_source`; the projection is guarded by the
`stub_gen_projects_a_valen_struct_that_implements_a_rust_trait` / `stub_gen_projects_mut_borrows_as_mut_references`
unit tests and the `run_wrapper` E2E `wrapper_drives_a_reverse_and_forward_mut_callback_to_exit_seven`
(a Vale struct impls a `&mut self`/`&mut T` trait, and forward `&mut C`/`&mut self` calls — both directions
compile, link, run → 7), all in `drive_tests.rs`.

## The `valen build` pipeline

`valen build [--manifest-path <Valen.toml>]` compiles a Valen crate against real Rust dependencies: it
generates a Cargo workspace and runs `RUSTC_WORKSPACE_WRAPPER=valenc-rs cargo build`, so cargo invokes
`valenc-rs` once per crate — a `.valen` crate drives Valen, a pure-Rust dependency passes through to plain
rustc. A binary `.valen` importing a rust path-dep builds and runs → exit 7, automated in the `pipeline_e2e`
tracer (see the Lessons entry on that separately-gated harness). Design of record:
`docs/architecture/rust-interop-design.md` (three programs — `valenc` pure-Valen, `valenc-rs` the wrapper,
`valen` the orchestrator); reference implementation `/Volumes/V/Harmonious/toylangc/`. All the code is
AI-editable, in `src/typing/rust_interop/`:

- **`run_wrapper`** (`drive.rs`) — the `valenc-rs` dark box (@DBAPIZ). It dispatches on the crate-root file
  extension: a `.valen` root is read, its pass-1 stub generated and **substituted for the `.valen` in cargo's
  argv** (rustc cannot parse `.valen`), then driven; any other root passes through via `NoopCallbacks` (zero
  overrides installed → byte-identical to vanilla rustc, @PRCCBIVRZ). The `valenc-rs` bin (`driver/main.rs`)
  is the thin `main()` over it — it strips cargo's rustc-path argv[1] and injects a `--sysroot` if absent.
- **`run_driven_rustc`** (`drive.rs`) — the one driven-compile engine: builds the arenas/`DriverState`/
  `DrivenCallbacks` in one frame, installs the query overrides + `set_fill_extra_modules_hook`, and runs
  rustc over the (stub) argv. Its `compile_builtins` argument gates `Source::builtins` +
  `PackageCoordinate::builtin` (as `pass_manager.rs` does) so Valen operators resolve and their `__vbi_*`
  intrinsics lower — always on for `run_wrapper`, and on for the whole test corpus too. The `#[cfg(test)]`
  test harness `drive_rustc` (`harness.rs`) delegates to it, passing its own `emit_backend`/`compile_builtins`
  and building the fixture argv itself; the shared scout→oracle preamble lives in `driven_code_source` /
  `scout_package_coords` / `collect_rust_import_paths` (`drive.rs`), which the harness's typecheck-only
  `CaseCallbacks` also calls.
- **`generate_stub_source`** (`stub_gen.rs`) — the `vale-stub-gen` seed (arch §6.4, @RTMEIZ), **parse-driven**:
  from the parse tree (`FileP.denizens`, via `ScoutCompilation::get_parseds()`, no scout/rustc) it emits one
  `pub use` per `import rust.X.Y`, a `#[vale::emit_consumer_body]` `__vale_<export>` root per exported func,
  the marker, the `__vale_drop` shim, and — for the reverse direction, against the design, which puts this in
  pass 2 (Next #4) — a `pub struct <S> {}` + `impl
  <ImportedTrait> for <S>` per Vale struct that implements an imported trait, each override rendered from the
  Vale `func <m>(self &<S>, ...)` (a `self &<S>` receiver → `&self`, or `&mut self` when its region is in the
  override's `mut(g)` set; a `&T` param → `&T`/`&mut T` by its region's mutability; `int` → `i32`) with
  a `#[vale::emit_consumer_body]` body. This projection is what lets `resolve_local_type` find the struct so a
  generic Rust caller monomorphized over it resolves; scope is a ZST callback struct with borrow/scalar-param
  overrides (a data-carrying struct or an unrenderable param type is a loud `StubGenError`, not a wrong stub).
  It still errors on an *exported* Vale struct/trait — the pass-2, `HinputsT`-driven exported-declaration
  emission the library form adds. This is the pass-1 `lib.rs`/crate root in the design's two-lib-file split:
  the pass-1 file is parse-driven, only the pass-2 exported decls are typing-driven — do not read design §30
  as making the crate root typing-driven.
- **`generate_workspace` / `run_build`** (`orchestrator.rs`) — the `valen` orchestrator. `generate_workspace`
  is pure (parsed `Manifest` → the cargo file set, no cargo/fs); it emits a standalone `[workspace]` table in
  the generated `Cargo.toml` so the package never tries to fold into a parent cargo workspace it is nested
  under (the build dir sits at `<project>/target/valen-build`, typically inside the user's real workspace —
  without `[workspace]` cargo errors "believes it's in a workspace when it's not"). `run_build` reads the
  `Valen.toml` (`toml`+`serde`), writes the workspace, copies the project `src/`, clears the driven crate's
  incremental cache only when `BuildInputs.clear_incremental` is set — the `valen` bin passes `false` so
  rebuilds reuse rustc's incremental cache (the no-fork warm-rebuild fix; see the incremental Lessons trap),
  and spawns a **debug** `cargo build` with `RUSTC_WORKSPACE_WRAPPER=valenc-rs` (release doesn't link —
  Next #3). The `valen` bin (`valen/main.rs`) is the thin `main()` over it, finding
  `valenc-rs` beside itself (so no PATH setup). Interim shape: a single flat cargo package (not the design's
  multi-project workspace), cargo's `[[bin]] path` points straight at the `.valen`, and rust-dep paths render
  verbatim; `valen-dependencies` and the multi-project layout are the library/permanent form.

Integer-scalar `Pair` (a small struct crossing as two register scalars, e.g. `{i32,i32}`) is built, as an
argument and a return, in both directions (a `Coercion::Pair(bits0, bits1)` threaded through the metal FFI to
`buildBoundarySignature`/`buildCallOrSideCall` and the inbound wrapper). A **zero-sized** imported struct
crossing by value (`Coercion::Ignore`) is handled as an argument and a return, both directions: nothing
crosses the ABI, so each side synthesizes the empty value locally (`LLVMGetUndef` of its translated type in
`buildCallOrSideCall`'s return arm and the inbound wrapper's arg/return arms) rather than moving bits — this
is interop only; a by-value struct across the **standalone C-ABI export** boundary is separate and unbuilt
(see Next). Still deferred (loud `panic!`/assert,
confirmed no NobiliaV consumer): a `Pair` whose component is a pointer/float (fat pointer/slice, HFA),
multi-piece `Cast` (`[N x i64]`), and on-stack byval (`Indirect { on_stack: true }`).

## Next

**Do next session** — the deferrals from the `i64` and `#[track_caller]` landings (full detail in
Short-term #8–#10 below):
- Lower non-`__vbi_` builtin externs (e.g. `TruncateI64ToI32`) on the interop path (#8).
- Make `synthesize_extern_function` return `Some(Err(reason))` instead of `None`, so an
  unnameable-but-enumerated type declines cleanly instead of `vfail`-crashing (#9).
- Default `[[bin]]` in `Valen.toml` to `src/main.valen` + the project name when absent (#10).

The `valen build` pipeline works for the binary + rust-dependency case (above). Reference implementation
`/Volumes/V/Harmonious/toylangc/` (`src/build.rs` orchestrator + `src/main.rs` wrapper dispatch).

**NobiliaV's entire real program builds and runs** through `valen build` — the reverse-callback endeavor is
proven on the real target. maple's windowed `driver.valen` (the full winit/wgpu/objc2-*/geometry/render/app/
nobiliav graph) builds and runs headless to a clean composite with the **auto-generated lambda form** (no
hand-written forwarder; see the auto-substruct section above), and warm-rebuilds cleanly. The `Valen.toml`
lives in the NobiliaV repo at `crates/nobiliav/valen/Valen.toml` (two `[[bin]]`s for `driver`/`driver_check`,
`[rust-dependencies] nobiliav = { path = "../../.." }`), with the `.valen` crate roots under `valen/src/`. The
open work is below — churn *enforcement* (the `--no-borrow-check` gap), `Deref`-reached imports, pulling the
stdlib into the interop compilation, release-mode linking, and Phase 3 libraries.

**Foundation (landed `8e2459c5`) — a hand-written generic forwarder wrapping a lambda.** `w.run(&MyCb((w,
input) => {...}))` where `MyCb<F>` is a *user-written* forwarder struct implementing an imported rust trait
and wrapping a functor `F`, so rust calls back into the Vale override whose body does `(&self.f)(...)`. The
generic lambda-forwarder reverse callback works end to end (`wrapper_drives_a_stateless_lambda_forwarder_to_exit_seven`
→ 7). The value-position-`ITypeST`/@TNLTZACZ and undeclared-generic typing walls are landed. This is the
foundation the auto-generated substruct (above) builds on — it removes the hand-written `MyCb`.

**Move fixture doc-comments into the tests that use them** (architect directive, incoming from the exp-1
borrow-checker tree). Today each item in `src/typing/rust_interop/fixtures/mycrate.rs` (and the sibling
`fixtures_*` crates) carries a doc comment explaining what its shape probes. That explanation belongs at
the use site, not in the fixture: whenever a test or corpus case (`corpus.rs`, `test/rust_interop/cases.rs`)
uses a fixture, the test says what about that fixture matters to *it*, and the fixture stays bare. A fixture
is shared across many tests, so a probe rationale living on it speaks for only one of them.

## Auto-generated anonymous substruct for imported Rust traits (built end to end)

A lambda handed directly to an imported Rust trait — `SomeTrait((args) => {…})` — now compiles, links, and
runs with **no hand-written forwarder struct/impl/override**. The compiler fires the existing
anonymous-substruct macro on the imported trait (exactly as it does for a native interface), and projects
the synthesized substruct all the way to Rust codegen through a two-pass driver. Proven end to end by
`wrapper_drives_an_auto_generated_forwarder_to_exit_seven` (`drive_tests.rs`, → 7): `MainLoop((w2) =>
{ w2.poke(); })` with the hand-written `MyCb`/`impl`/`on_tick` deleted, driven through `run_wrapper` against
a real rlib. This is the feature NobiliaV asked for (write the lambda, not the forwarder). It covers
**`&mut`-signature (churning) callbacks** too — NobiliaV's real `on_tick(&mut self, w: &mut NobiliaWindow,
…)` — under `--no-borrow-check` (churn *enforcement* is still deferred; see below).

**How it works (all code in the sites below):**
- **Typing (fire the macro).** `Compiler::evaluate`'s `RustImportSeed::Interface` arm (`compiler.rs`, core)
  fires `get_interface_sibling_entries_anonymous_interface` on the imported trait when the trait is
  *anon-eligible* (a bool threaded on `RustImportSeed::Interface`, computed by `sig_is_anon_representable`).
  The generated denizens are minted in the reserved native package `RUST_TRAIT_ANON_MODULE`
  (`"rust_trait_anon"`, `reserved.rs`) — NOT the `rust` package (the function-compile loop skips `rust`, and
  `citizen_def_id_and_args` must see the substruct as a local type). They are threaded into package stores
  **grouped by full nesting** (`{coord + init_steps}`), the same way the native denizen loop does — the
  constructor/forwarders land top-level, the substruct's `drop` nested under `[AnonymousSubstructTemplate]`,
  each in the store its id names. Flattening them to the package top level (an early `per_crate` attempt)
  mis-nests the drop and breaks its instantiation (see Lessons).
- **De-Seal.** `synthesize_extern_trait` (`declarations.rs`) no longer stamps `Sealed` (the anon macro bails
  on a sealed interface); safe because the #374 sealed check is gated behind `!is_internal_method` and a
  trait's abstract methods are internal. Enums (`synthesize_extern_interface`) stay sealed.
- **Value-position `where func __call` bound.** The macro's bound is reified by `resolve_citizen_bounds`
  with only the citizen env + generic-substitution map, so `synthesize_abstract_interface_method`
  (`declarations.rs`) builds each param's `tyype` AND `maybe_return_type` in **value-position** form
  (`value_position_type_st`: a primitive → zero-arg `Call(Name)` @TNLTZACZ, a Rust citizen → its **short**
  single-segment name resolved by unconditional cross-package lookup, a borrow → wraps its inner), mirroring
  what the parser produces for a native method — NOT the internal value-runes (a shape the parser never
  produces, which is why the bound choked before). Params and the return go through the same builder ("a
  param is just another parameter").
- **Named projection, not opaque-self.** `citizen_def_id_and_args` (`src/instantiating/rust_interop/mod.rs`)
  has an `INameI::AnonymousSubstruct` arm that resolves the substruct to the deterministic mangled Rust name
  `anon_substruct_rust_name(interface) = "<Interface>__anon"` via `resolve_local_type`, so only the *functor*
  crosses opaque (`__ValeOpaque<typeid(functor)>`), exactly the hand-written `MyCb<F>` shape.
- **The one name-agreement seam (P0).** `anon_substruct_rust_name` (`typeid.rs`) is the single definition of
  the substruct's Rust name, called by BOTH the stub generator and `citizen_def_id_and_args`. If they ever
  disagreed the callback is silently dropped (`collect_callback` finds no impl, no diagnostic).
- **Pass-2 (HinputsT-driven) stub.** `generate_pass2_stub` (`stub_gen.rs`) walks the typed program's
  `interface_template_to_sub_citizen_to_edge`, finds each anon substruct implementing a rust-backed trait
  (`is_rust_backed(edge.super_interface)`), and emits `pub struct <I>__anon<T..>(__ValeOpaque<HASH>,
  PhantomData<..>)` + `impl<T..> <I> for <I>__anon<T..>` with `#[vale::emit_consumer_body]` override bodies
  (each param's `KindT` rendered by `render_rust_kind`). A `&mut self`/`&mut T` param renders `&mut` from the
  abstract method's `mut(g)` **effects**, read off the retained postparse — `abstract_method_mut_flags` finds
  the abstract method's `FunctionS` via `Compiler::get_super_template(interner, abstract_id)` +
  `peek_postparsed_function`, and `param_tyype_is_mut` matches each param's `BorrowRef` region group against the
  method's `EffectS::Mut` groups structurally (`GroupS: PartialEq`, no human-name keying @ATAFLBZ); the typed
  `KindT` alone carries no mutability (@BCHATZ). Deterministic (sorted; no HashMap iteration order reaches
  output). It exists only post-typing, so it can't be the parse-driven pass-1 stub.
- **Two-pass driver.** `run_driven_rustc` (`drive.rs`) allocates the arenas/interners/slots ONCE spanning
  both `run_compiler` calls. Pass 1 types the `.valen`; if `generate_pass2_stub` is non-empty it stashes the
  appended stub and returns `Compilation::Stop` before arming/codegen; pass 2 (a `skip_typing` callbacks)
  reuses pass-1's `HinputsT` (tcx-free — survives the second `tcx`), skips typing, and codegens the appended
  stub. **Design-vs-code gap:** when the appended stub is empty the code short-circuits to one pass; the
  design (`rust-interop-design.md`, the two-pass deviation bullet) says every Valen crate compiles in two
  passes, no exceptions, so that short-circuit is to be removed. `debug_assert`s that the codegen slots are
  pristine at pass-2 entry. The appended stub is
  written to `--out-dir` (never next to the crate root — that pollutes checked-in fixtures) and predeclares
  `__ValeOpaque` only when the pass-1 stub lacks it; it is *appended* after the pass-1 text so
  `#![register_tool(vale)]` stays at the crate-root top.
- **Warm-rebuild determinism (exports-first).** On a warm rebuild rustc serves mono items from the
  incremental cache and skips `per_instance_mir`, so `monouts` starts empty and
  `lang_collect_and_partition_mono_items` (`mod.rs`, `eval_always`) is its sole re-populator — but it
  re-fires in rustc's *unordered* CGU order, so a callback (which READS `monouts` to build the typeid
  universe) can re-fire before the export (which WRITES it), reading an empty universe → the
  `collect_callback` universe-presence panic. This is a @P0 nondeterminism (bites ~half of cold-build
  layouts; see the per-lineage Lessons entry). Fix: before the CGU loop, re-fire `per_instance_mir` for
  every export first, seeded from the **sorted** `hinputs.function_exports` (resolved directly via
  `resolve_local_fn` + `Instance::new_raw`, which also covers a non-`main` export that is in no CGU) —
  populate-then-read, the invariant cold gets for free (Sky's "B6" cure). Two defensive @P0 hardenings in
  `collect_callback` alongside: the impl match asserts a *unique* match (was last-match-wins) and the edge
  lookup asserts *exactly one* edge (was `.find()` over a std HashMap), so an ambiguous program fails loud
  rather than order-dependent. Deferred (architect OK'd): swap the core `HashMap`
  `interface_template_to_sub_citizen_to_edge` to an ordered map — the uniqueness assert makes it safe meanwhile.
- **Humanizer.** `compiler_error_humanizer.rs` now renders `AnonymousSubstructConstructor` /
  `AnonymousSubstructConstructorTemplate` (were `panic!("implement: …")`) via
  `humanize_anonymous_substruct_constructor_template` — so a failed anon-constructor resolution reports
  cleanly instead of masking the real error with a humanizer panic.

**Anon-eligibility gate (what's supported).** A trait is anon-eligible iff every method's params and return
are value-position-representable (primitives, `void`, imported Rust citizens by borrow — **shared or `&mut`**
— and generics), checked by `sig_is_anon_representable` (`declarations.rs`) on the Self-mapped sig. An
ineligible trait still imports and works through a **hand-written** forwarder; its anon constructor just
isn't available. `&mut` signatures ARE supported: the pass-2 stub renders `&mut self`/`&mut T` by reading the
abstract method's `mut(g)` effects off the retained postparse (see the pass-2 bullet). What is **not** built
is churn *enforcement* (below) — a churning `&mut` callback compiles and runs only under `--no-borrow-check`.
Tested `&mut` shapes are `&mut self`, `&mut <ImportedCitizen>`, and mixes with `&`. Exotic shapes fail LOUD,
not silently: `render_impl_method_typed` renders `&mut` only on a `KindT::BorrowRef` param and ignores return
mutability, so a `&mut` return renders shared → rustc E0053, and a `&mut` generic → `StubGenError`. If one
must work, it is a named follow-up, not a covered case.

**Two passes for every Valen crate is ratified design.** `rust-interop-design.md`'s deviation list says a
Valen crate compiles in two rustc passes (pass 1 types and emits the stub, pass 2 codegens it), binary or
library, with or without an anon substruct. The code's empty-stub one-pass short-circuit in
`run_driven_rustc` is the remaining gap (see the two-pass driver bullet above).

**Driver-check through real `valen build` — DONE and verified.** `driver_check.valen` uses the auto-generated
`MainLoopCallback((win, inp) => {…})` form — no hand-written forwarder — and
`automates_driver_check_reverse_callback` (`pipeline_e2e.rs`) builds it AND **runs it to exit 7** through the
real `valen build --no-borrow-check` orchestrator on NobiliaV's exact driver-check shape (bins built, test
green). The vendored `nobiliav` (`driver-check/nobiliav/src/window.rs`) is NobiliaV's real **`&mut` churn**
shape: `on_tick(&mut self, w: &mut NobiliaWindow, input: &FrameInput)`, `rotate_camera`/`request_exit` as
`&mut self`, and a deterministic bounded `&mut self` frame loop (plain fields + a `MAX_FRAMES` cap, no wall
clock/windowing). `--no-borrow-check` because the lambda churns the window (enforcement is the gap below);
`release_build_of_driver_check_links` builds it release the same way (`valen_build_release` sets
`VALEN_BORROW_CHECK=0`). **Confirmed in the wild:** maple's real windowed `driver.valen` (the full
winit/wgpu/objc2/geometry/render/app/nobiliav graph) builds and runs headless to a clean composite via
`valen build --no-borrow-check` on these bins, with the hand-written `TickForwarder` deleted — the lambda
form is NobiliaV's real deliverable.

*Guardian note:* AFEOX's editable-extension allowlist now includes `.valen` (added beside `.vale`; the earlier
omission was stale from the vale→valen rename), so editing `.valen` fixtures needs no Guardian disable.

**Churn enforcement — still the gap (`&mut` representation is done, enforcement is not).** A `&mut`-signature
trait auto-generates and its stub renders `&mut`, but the borrow checker does not yet *enforce* the churn: a
`where func` bound prototype has no mechanism to introduce a region generic parameter — `parse_prototype`
(`src/parsing/templex_parser.rs`) parses `func <name>(<tuple>)<return>` with no `<…>` slot, so a churn region
on the bound (`func __call<r'>(&F, &Win in r, &Inp) mut(r)`) is a parse error and leaving it free leaves an
undetermined `ImplicitRune` → `CouldntSolveRuneTypesT`. So a churning `&mut` callback builds and runs only
under `--no-borrow-check` (which NobiliaV/maple use); with the checker ON it is rejected.
`generic_forwarder_with_churning_closure_param_compiles` (`after_regions_tests.rs`) PASSES via
`compiler_test_compilation_without_borrow_check` (the `&Win mut` sugar) — it documents the borrow-check-OFF
path, NOT a red marker for enforcement (a checker-ON test is unwritten). Enforcement is a **feature**
(per-call region quantification on the `where func` bound, HRTB-shaped), not a syntax anyone missed — and it
is orthogonal to the `&mut`-signature support, which is complete.

**Tests guarding this feature.** Native (`after_regions_tests.rs`):
`native_anon_substruct_with_multi_param_abstract_method_compiles`,
`native_anon_substruct_with_concrete_citizen_borrow_param_compiles`. Interop (`cases.rs` / `drive_tests.rs`):
`a_lambda_typechecks_against_a_rust_trait`, `a_lambda_typechecks_against_a_two_imported_param_trait`,
`pass2_stub_projects_the_anon_substruct_for_an_imported_trait`,
`pass2_stub_projects_mut_borrows_for_an_imported_trait` (the `&mut` stub render),
`wrapper_drives_an_auto_generated_forwarder_to_exit_seven`,
`wrapper_drives_an_auto_generated_mut_forwarder_to_exit_seven` (auto-gen `&mut` churn → 7,
`--no-borrow-check`). Pipeline (`pipeline_e2e.rs`): `automates_driver_check_reverse_callback` (run → 7) and
`warm_rebuild_of_auto_generated_forwarder_runs_seven` (the exports-first P0 guard — loops 10 fresh cold-build
lineages). All uncommitted on `exp-4-wipbx`; run the suites and
read the counts: `cargo +rustc-fork test --manifest-path ./Cargo.toml --lib --features rust_interop`
(interop), `cargo nextest run --manifest-path Cargo.toml` (native), `VALE_TEST_BACKEND=wasi cargo nextest
run --manifest-path Cargo.toml` (wasi).

*Landed slice 1 — the stub projects the forwarder as the design's opaque wrapper.* Contrary to an earlier
"the functor is opaque bytes, not a separate rust type" framing (which was WRONG — corrected by deep-reading
`vale-rust-interop-architecture.md` §10 and the Sky precedent), a Vale struct crossing to rust is the
**wrapper-as-field opaque shape**: `pub struct MyCb<F>(__ValeOpaque<HASH>, PhantomData<(F)>)`, never real
fields. rustc keeps MyCb's own DefId (so the `impl` resolves) but sees an opaque blob; Vale owns the size via
a `layout_of` override (see "the size question" below). `generate_stub_source` (`stub_gen.rs`) now predeclares
`__ValeOpaque<const T: u64>` (`VALE_OPAQUE_DECL`) and emits every projected struct as that 2-field wrapper + a generic
`impl<F> Cb for MyCb<F>` (generics from `StructP.identifying_runes` / `ImplP.generic_params`). A `typeid(&str)
-> u64` FNV-1a helper is new at `src/typing/rust_interop/typeid.rs` (deterministic — pinned-literal fence in
its tests; @P0 no-nondeterminism). Guarded by `stub_gen_projects_a_generic_forwarder_as_opaque_wrapper`
(`drive_tests.rs`); the two pre-existing reverse-callback dark-box tests were updated to the wrapper shape and
all reverse-callback E2Es still pass through it (a fieldless struct is the degenerate `PhantomData<()>` case,
still a ZST). Mirrors Sky exactly: `toylangc/src/stub_gen.rs:138,189-195` (Sky's wrapper is `__ToylangOpaque
<const T: u64>` — **u64 not the doc's u128**), typeid `toylangc/src/typeid.rs:39`.

*Landed slice 2 — the instantiator lowers an internal Vale type (the functor) to `__ValeOpaque<typeid>`.*
The outbound leaf `run::<MyCb<Lambda>>` used to panic in `rust_request_arg_tys` because the lambda functor is
an `INameI::LambdaCitizen` that `citizen_def_id_and_args` doesn't handle. New `citizen_or_opaque_to_rustc_ty`
+ `opaque_ty` + `build_opaque_args` in `src/instantiating/rust_interop/mod.rs`: a Valen-internal citizen that
isn't a projected stub type nor a `rust`-module dependency lowers to `__ValeOpaque<typeid(humanize_id(id))>`
(found via `resolve_local_type(tcx, "__ValeOpaque")`, const built with `ty::Const::from_bits(tcx, tid as
u128, TypingEnv::fully_monomorphized(), tcx.types.u64)`). Mirrors Sky `oracle.rs:1330` `build_opaque_args`.
Guarded by `wrapper_lowers_a_forwarder_arg_to_a_noncallback_rust_fn` (`drive_tests.rs`) — a forwarder handed
to a rust `fn touch<C: Trait>(_cb: &C){}` that never calls back, isolating the outbound arg-lowering from the
inbound callback.

*Landed parts 1–2 — represent the dropped `C: MainLoop` bound, and make extern functions carry bounds.* The
synthesized rust caller `run<C: MainLoop>` used to **drop its `C: MainLoop` trait bound**. Now `fn_sig`
(`tyctxt_oracle.rs`) reads `tcx.predicates_of(def_id)` and surfaces each `where P: Trait` whose subject is an
own generic param and whose trait is imported (`ItemKind::Trait` present in `self.items`; auto-traits/Sized are
dropped) as a `ValeSigImplBound` on `ValeSig`; `synthesize_extern_function` (`declarations.rs`) emits it as an
`ImplBoundS` (sub = the param's rune, super = `bind_sig_type(trait citizen)` into `header_rules`, result =
a fresh `ImplicitRune` minted like the postparser's `rule_scout.rs` pattern). And the core
`make_extern_function` (`function_compiler_core.rs`) now threads the solved `instantiation_bound_params` into
its `FunctionDefinitionT` (the ExternBody arm passed it in), instead of hard-coding empty — the leaf
`ExternFunction` prototype stays bound-free (`translate_prototype` returns early for it, `instantiator.rs:1027`;
rustc discharges the trait obligation). This fixed the `instantiator.rs:659/1065` `1 != 0` assert (a real
latent bug: `ExternBody`+`impl_bounds` was a shape pure Valen never produced and the compiler silently
mis-compiled). Both parts are needed and stay: resolving `run`'s bound is what records the concrete impl id
`MyCb<lambda>: MainLoop` (`is_parent` → `add_instantiation_bounds`, `impl_compiler.rs:854-856`) — the datum the
real fix consumes.

*Landed slice 3 — the concrete override is minted STATICALLY, no interface fat pointer.* Guarded by
`wrapper_drives_a_stateless_lambda_forwarder_to_exit_seven` (`drive_tests.rs`, → 7). `collect_callback`
(`src/instantiating/rust_interop/mod.rs`) now, uniformly for generic and non-generic (the degenerate zero-subs
case, @NNGZ):

  1. **Reverse-decodes the rustc callback `instance` to identify the concrete Valen impl.** `read_opaque_typeid`
     reads a `__ValeOpaque<typeid>` const via `ty::Const::try_to_leaf().to_u64()` (the inverse of
     `build_opaque_args`); an inline `typeid →` kind pass over the citizens in `monouts` fails loud on an opaque
     type Vale never instantiated (§10.9 recovery); the impl is matched among `monouts.interface_to_impls` by
     projecting each candidate's sub-citizen forward through `citizen_or_opaque_to_rustc_ty` and comparing rustc
     types. The Self type comes from `tcx.type_of(tcx.impl_of_assoc(instance.def_id())).instantiate(tcx, instance.args)`.
  2. **Resolves the concrete override STATICALLY via `resolve_override_prototype`** (`instantiator.rs`, `pub fn`
     factored out of `translate_override` by the impl-bounded-devirtualization landing) — reading the impl's typed
     id + bound args from `monouts.instantiated_impl_to_typed_impl_and_bounds`. This is the same seam a
     devirtualized `where implements` call uses, and it yields the plain concrete override directly with **no
     vtable and no abstract-method dispatcher** — the dispatcher's body is what would virtual-dispatch and build an
     interface fat pointer, and it is never instantiated.
  3. **Emits the wrapper for the plain concrete override** `on_tick<lambda>(&MyCb<lambda>, …)`, a struct-receiver
     body.

  This is why it works: a concrete override for a *generic* impl is an instantiator product, and the reverse
  callback is **static dispatch** (Rust calls the concrete override directly). Driving `translate_override` — the
  *virtual*-dispatch path — was the earlier misstep: it instantiated the abstract dispatcher method
  `on_tick(&MainLoop, …)` into `monouts.functions`, whose body virtual-dispatches, and the backend then aborted
  building an `&MainLoop` interface fat pointer (a synthesized extern interface has no interface-ref-struct
  layout). `resolve_override_prototype` sidesteps all of it. The impl is identified against the §10.9
  typeid→kind universe table `DriverState.opaque_universe` (registered after every drain by
  `register_instantiated_kinds`), the same table the `layout_of` override reads (see "A Valen struct crosses
  to Rust by value").

**Interop autoderef — built (single-step, shared `Deref`).** A method reached through a type's shared
`Deref` resolves and runs by a callsite receiver rewrite to `deref(recv)`, mirroring rustc's callsite
autoderef. Interop-scoped by design (Valen has no `Deref` trait of its own — it defers to Rust's) and fires
only for a rust-backed receiver. Proven end to end by `rustc_collector_drives_a_deref_reached_method`
(tier-1) and `rustc_driven_bin_deref_reached_method_returns_seven` (tier-2 → 7) on the synthetic
`Sheath: Deref<Target=Core>` fixture (`Core::read(&self) -> i32`, `mycrate.rs`). The pieces:
- `discover_deref_targets` (`tyctxt_oracle.rs`) probes each imported **non-generic** ADT for a shared
  `Deref<Target=U>` with a **non-generic named-ADT** `U`; registers `U` + its inherent methods and adds a
  `deref` method (`ItemKind::DerefMethod`) whose `fn_sig` is built directly as `&Source -> &Target` (never
  lowering `deref`'s raw sig, whose `Self` is a param and whose return is the declined `Target` projection).
- `deref_target_imports` reports the auto-added targets; `Compiler::evaluate`'s import loop declares each
  implicitly, exactly as an explicit import.
- `try_autoderef_overload_call` (`call_compiler.rs`, `#[cfg(rust_interop)]`) fires in the `Err` arm of
  `evaluate_call` when a method misses on a rust-backed receiver: resolve `deref`, rewrite the receiver to
  `deref(recv)`, re-resolve. Single-step and terminating because only originally-imported types get a `deref`.
- `resolve_deref_request` (`src/instantiating/rust_interop/mod.rs`) resolves the `deref` leaf through the
  `Deref` impl (inherent-method resolution misses it).

**A Valen struct crosses to Rust by value (the `layout_of` override) — built.** A data-carrying Valen
struct (`struct Ship { fuel int; }`) passes through a generic Rust fn by value, lives by value inside a real
`Vec<Ship>` at its real stride, and is read and written through element borrows — including the benchmark
shape, two aliased refs to the vec with a field write through one element borrow read back through the
other. Never `repr(C)`: the element stays opaque to Rust. Tier-2 guards in `cases.rs`:
`rustc_driven_bin_vale_struct_round_trips_by_value_returns_42` (`id<T>` round trip),
`rustc_driven_bin_vec_of_vale_structs_read_through_borrow_returns_42`,
`rustc_driven_bin_aliased_vec_element_write_returns_42`. The pieces:
- An unprojected Valen type in a Rust slot crosses as bare `__ValeOpaque<typeid>` (`opaque_ty`,
  `src/instantiating/rust_interop/mod.rs`; design "everything else"), and `__ValeOpaque` is the design's
  marker struct plus a real `UnsafeCell<()>` so it is `!Freeze` (`VALE_OPAQUE_DECL`, `stub_gen.rs`, with
  each marker's reason; the harness `fixtures/stub.rs` hand-writes it; pinned by
  `a_valen_type_is_not_freeze_to_rustc` through the harness's `probe_fixture_tcx`). Its own fields are
  zero-sized; the size is the override's, reported over however many marker fields the struct has.
- `lang_layout_of` (`mod.rs`, installed in `vale_override_queries`, saved default in `DEFAULT_LAYOUT_OF`)
  intercepts exactly the `__ValeOpaque` ADT: `read_opaque_typeid` → `DriverState.opaque_universe` → the
  struct's `StructDefinitionI.members`, each lowered by `kind_to_rustc_ty` and sized by rustc's own
  `layout_of`, then a C-offset walk (`c_layout_over`; the same layout LLVM gives the backend's non-packed
  struct of those members) reported as `BackendRepr::Memory { sized: true }` with one offset per marker
  field (§10.4.5). Derived types (`&`, `*mut`, `Option<..>`) and a typeid naming nothing instantiated fall
  through to rustc (Sky's lesson: intercepting derived types corrupts them). A Valen *interface* by value,
  or a member with no rustc type (`str`, `float`, a Valen reference), panics loud rather than sizing wrong.
- `DriverState.opaque_universe` is the §10.9 typeid→kind table: `register_instantiated_kinds` appends every
  struct/interface in `monouts` after each drain (export and callback), i.e. before any rustc query on the
  new leaves — populate-then-read. It is a separate cell from `monouts` because the ABI queries that re-enter
  `layout_of` (`compute_extern_abi` → `fn_abi_of_instance`) fire while the resolve loop holds `monouts`
  mutably (see Lessons). `opaque_typeid` is the one definition of the key both sides use.
- Element access is the fixture-crate helper `at<T>(v: &Vec<T>, i: i64) -> &T` (`mycrate.rs`): the
  `i as usize` is its own Rust (Valen never converts `i64`↔`usize`), and the bare `&T in g...` return is
  the borrow shape a Valen program already receives. A benchmark crate writes its own `at`. Importing it
  needed one importer fix: `bind_sig_type`'s nested-`Borrow` arm now wraps the rune the inner settles to
  (the generic's own rune for `&T`), the return-side twin of the `&C`-parameter lesson.
- exp-2's speed cases (05-08, 13) are the downstream consumer: this is the shape they respell onto
  (`Vec<Ship>` by value + an `at`-style accessor). The alias *metadata* correctness for the descendant return
  group (`&T in g...`) is their §2b `borrow_checker` work, in parallel; nothing here waits on it.

**Then: real `Vec::get`.** `Vec::get`/`first`/`last`/indexing are slice (`[T]`) methods reached from
`Vec<T>` through `Deref<Target=[T]>`. The autoderef mechanism above is the prerequisite; `Vec` needs four
more pieces it deliberately does NOT cover: (1) a **generic** `Deref` source (`Vec<T>` — `discover_deref_targets`
takes only non-generic sources); (2) the slice `[T]` target (declines `Unsized` in `lower_ty`);
(3) a Valen `usize` index — `usize` has no literals or operators, so a program cannot produce one, and the
compiler never converts `int`/`i64` to it; (4) the `Option<&T>` return (a borrow nested in an enum, plus its
descendant return group — the §2b `noalias` coordination with exp-2). Tracked by the ignored
`a_real_vec_element_accessor_is_importable` (`cases.rs`).

**Short-term (very soon):**
1. **Pull the stdlib into the interop compilation.** Today only the compiler builtins are compiled in
   (`run_driven_rustc`), so a Valen program can use operators but not stdlib collections (`Option`, `Vec`,
   `str`, …). A feature, not a cleanup: needs the stdlib's per-import tree-shaking and native-impl
   interop-compat (mirror `pass_manager.rs`'s stdlib handling).
2. **Restore the by-value-struct C-ABI export test before calling the endeavor done.** A **by-value
   (owned) struct** passed or returned across the standalone C-ABI export boundary is unimplemented —
   two gaps, both empty-struct-triggered but general to by-value owned structs (all existing struct
   exports cross by `&` borrow or as a `share` handle, so this was never exercised): (a) an owned
   by-value struct **arg** crosses as a pointer C-param (`hostBoundaryType` `OwnRef` → pointer,
   `boundary.cpp`) but `exportFunction`'s `receiveHostObjectIntoVale` never loads through it → a `toRef`
   type mismatch (SIGABRT); (b) `generateExports` emits invalid C for an empty-struct **return**
   ("initializer for aggregate with no elements requires explicit braces"). This is the standalone
   export path, distinct from the interop `Ignore`/`Pair` coercion work (which is complete). The
   `zst_struct_exported_by_value` test (`src/end_to_end_tests/tests/externs.rs`) captures both shapes
   and is `#[ignore]`d pending this; un-ignore it once by-value struct export is implemented (fix the
   owned-arg receive to load through the pointer, and the generated-C empty-aggregate initializer).
3. **Release-mode `valen build` doesn't link (merged-CGU DCE; distinct mechanism from the landed incremental fix).** `run_build`
   (`orchestrator.rs`) only ever runs a plain **debug** `cargo build` — no `--release`, no
   `codegen-units`/`CARGO_INCREMENTAL`/profile, and `generate_workspace`'s generated `Cargo.toml` declares
   no `[profile.*]` — so release is untested and unhandled. Building the generated package release by hand
   (`RUSTUP_TOOLCHAIN=rustc-fork RUSTC_WORKSPACE_WRAPPER=<…>/valenc-rs cargo build --release --manifest-path
   <…>/valen-build/Cargo.toml`) fails at link two ways: **default (`codegen-units=16`)** → a duplicate `main`
   (`main.llvm.<hash>` defined in several objects); **`codegen-units=1`** → undefined leaf symbols
   (`NobiliaWindow::new`, `FrameInput::{key,mouse_x,mouse_y}`, `NobiliaWindow::rotate_camera`),
   `ld: symbol(s) not found for architecture arm64` (macOS/arm64). Root cause is CGU-layout sensitivity
   (a *different* mechanism from the landed incremental fix's `per_instance_mir` side-effect skip): a reified Rust leaf is reachable only through the `ReifyFnPointer` casts in
   `__vale_main`'s synthetic body (`build_dependency_body`, `src/instantiating/rust_interop/mod.rs`), which
   the partition filter (`lang_collect_and_partition_mono_items` gated by `is_vale_codegen_target`, same
   file) strips from rustc's own objects — the real caller lives in the out-of-band `vale_cgu` module the
   partitioner never analyzed. Debug's per-item CGUs keep each leaf externally visible; release's
   merged/fewer-CGU layout collapses a leaf with its stripped `__vale_main` referrer and internalizes/DCEs
   it → the `vale_cgu` call is unresolved. The duplicate `main` is rustc-internal — interop emits no `main`
   (`makeEntryFunction` with `emitLibcShim=false`, `vale.cpp`); leading **unconfirmed** hypothesis is rustc's
   entry wrapper (`maybe_create_entry_wrapper`) firing in >1 CGU under the merged/opt layout. Confirm next
   session by dumping the entry item's and the leaves' CGU membership/linkage under `--release` — rustc's
   `-Zprint-mono-items`/`-Csave-temps` for the partition, or set `print_llvmir` (+ an `output_dir`) in the
   driven `BackendCompileOptions` (`emit_vale_into_borrowed_module`, `mod.rs`) so `finalizeCompile` writes
   `build.ll` of the emitted module (the `noalias.rs` e2e test shows the read-and-string-match pattern). No in-tree
   repro yet — a `--release` `valen build` of the vendored `driver-check` fixture
   (`src/typing/test/rust_interop/driver-check/`) likely reproduces the duplicate `main` (layout- not
   body-dependent). Interim mitigations (NobiliaV-side, in its `gamedev-handoff.md`, not applied here):
   `[profile.dev] opt-level = 2` on the generated `Cargo.toml` + rebuild **debug** with the fork toolchain
   (near-release speed; wiped by the next `valen build`), or build the pure-Rust twin in release. Fix
   directions: (a) AI-editable — make the partition filter force the reified leaves (and any item referenced
   only by a stripped Vale item) to `External`/`GloballyShared` linkage in the rebuilt CGUs and keep a single
   entry-CGU home (`mod.rs`); (b) AI-editable stopgap — `generate_workspace` emits a `[profile.release]`
   reproducing the debug working point (many `codegen-units`, incremental on) and the cache-clear stops
   hard-coding `target/debug/incremental` (`orchestrator.rs`); (c) permanent, fork-side ("fire core edits") —
   make the `fill_extra_modules` output participate in the partitioner's reachability/linkage. (This is a
   fork route; the landed incremental fix's no-fork `per_instance_mir` re-fire did not need it.)
4. **Align the two-pass driver with `rust-interop-design.md` — do this first.** The design's deviation
   list says every Valen crate compiles in two rustc passes, and that pass 2 projects every Vale struct that
   implements a Rust trait. The code deviates in two ways, both in AI-editable `rust_interop/`:
   (a) `run_driven_rustc` (`drive.rs`) short-circuits to one pass when the appended stub is empty — remove
   the short-circuit so pass 2 always runs; (b) `generate_stub_source` (`stub_gen.rs`, parse-driven, pass 1)
   projects hand-written `struct` + `impl <RustTrait>` pairs, while `generate_pass2_stub` (`HinputsT`-driven,
   pass 2) projects only the anon substruct — move the hand-written projection into `generate_pass2_stub`
   and delete pass 1's impl renderer (`render_impl_method`, `render_param_type`, `render_rust_type`,
   `map_scalar_name`, `render_receiver`, `borrow_region_is_mut`, and the `StructP`/`ImplP` walk);
   (c) the design's pass-1 root is imports only — the `fn main` + `__vale_main` root (a binary) or the
   exported-function roots (a library) are appended in pass 2 alongside the impl projections, so move the
   `__vale_main` emission out of `generate_stub_source` too. Nothing in pass 1 consumes any of it: Valen
   types the struct from the `.valen`, and rustc's typeck, mono collection, and `resolve_local_type` all run
   in pass 2. The design puts `#![feature(register_tool)]` + `#![register_tool(vale)]`, `__VALE_STUBS_MARKER`,
   and the `__vale_drop` shim in pass 2 as well, so pass 1's file is literally the `use` lines. When
   assembling the pass-2 file, the two inner attributes must be its first items (Rust rejects an inner
   attribute after any item), so they go before the carried-over `use` lines, not after. The design's
   `after_expansion` sketch still gates on the marker, which pass 1's file no longer carries; the code
   gates on the `.valen` extension instead, and the sketch is the stale side. One renderer instead of two
   (see the simplicity lesson). The typed renderer must then handle a
   hand-written struct's own generic params (read them off the `StructDefinitionT`, not `hinputs.structs`
   instances as `anon_substruct_arity` does) — the one piece of new work.
5. **A *named* projection of a Valen struct is still zero bytes to rustc.** A Valen struct Rust can name
   (one implementing a Rust trait, or an anon substruct) is projected as `pub struct <S>(__ValeOpaque<HASH>,
   PhantomData<..>)` where `HASH` is `typeid(<S>'s name)` — a name hash, not an instantiated id — so the
   `layout_of` override finds nothing in `opaque_universe` for that field and rustc sizes `<S>` at zero. An
   *unprojected* Valen type is sized correctly (see "A Valen struct crosses to Rust by value"). Harmless
   while every named-projection crossing is by borrow (`run(&self, cb: &C)`, `main_loop(&mut cb)`): a pointer
   is a pointer whatever rustc thinks is behind it. The first Rust API that takes, returns, stores, or copies a
   *trait-implementing* Valen struct **by value** gets `PassMode::Ignore` — nothing crosses. Closing it means
   keying the override on the named projection too (Sky's shipped shape, `rustc-lang-facade/src/queries/
   layout.rs:61-206`, keys on the projected name), which also needs the architect's ruling on the named
   projection's shape (the "Opaque wrapper shape" bullet under "Design doc vs as-built").
6. **Retire the fork's process-global `fill_extra_modules` hook.** `valenc-rs` installs its hook by calling
   `rustc_codegen_llvm::set_fill_extra_modules_hook`, which stores it in a `static OnceLock`; nothing else in
   rustc lets a driver set a static that changes compilation, and it is the one piece of patch 2 an upstream
   reviewer would refuse. The replacement is per-backend-instance state: `LlvmCodegenBackend` gains a `pub
   extra_modules: Option<FillExtraModulesHook>` field, and `DrivenCallbacks::config` (`drive.rs`) constructs
   that backend with the hook and installs it via `Config::make_codegen_backend` — the same seam miri uses
   (`rustc_interface::util::DummyCodegenBackend`). Fork-only plus one Valen call site; the existing hook tests
   pin the behavior. Full plan, with the RFIGA and the four `// DO NOT SUBMIT` markers it also resolves:
   `docs/handoffs/rust-interop-backend-hook-plan.md`. What an upstream pitch of patch 2 would then still
   face, in the order a reviewer would raise it: "why not `-Clinker-plugin-lto`?" (answer: Valen is in-process,
   not a second toolchain; plugin LTO cannot reach the bitcode in rustup-shipped std rlibs so std becomes a call
   boundary; rustc's plugin-LTO path has no Darwin branch and is tested only on linux-x86_64 with lld; neither
   architecture doc evaluates it by name yet); the trait method should return `Vec<Self::Module>` and let
   `rustc_codegen_ssa` name and wrap them as it does for `codegen_allocator`, with a `ModuleKind` variant so the
   LTO policy for extra modules is explicit; and the raw handles (`ModuleLlvm::new` pub, `llcx_raw_mut`,
   `llmod_raw`) need a sanctioned `unsafe` API story. `traits/backend.rs` has had about four reviewed PRs in
   three years, nearly all through bjorn3, and rust-gpu's out-of-tree `target_override` was deleted once
   already, so frame it as "a backend contributes pre-built CGUs" and pre-negotiate on Zulip. No rust-lang
   maintainer has seen any of the fork; `Harmonious/convo-with-rustc.md` is a model roleplaying a reviewer.
7. **Small stale spots to clear when nearby.** The header comment on `automates_driver_check_reverse_callback`
   (`pipeline_e2e.rs`) still says the fixture uses a hand-written forwarder and is blocked on Guardian; both are
   false. `fixtures_mut_callback/stub.rs` carries a hand-written `MyCb` impl nothing references (the corpus case
   uses the lambda form). `render_rust_kind`'s doc comment (`stub_gen.rs`) says `&mut` is unreachable through a
   callback boundary; its caller renders `&mut` above it.
8. **Non-`__vbi_` builtin externs don't lower on the interop path.** A builtin `extern func` that is not a
   `__vbi_` intrinsic — e.g. `TruncateI64ToI32(x i64) int` (`src/builtins/resources/arith.vale`) — gets no
   interop `FunctionExternI`, so calling it from a driven / `valen build` program aborts the backend at
   `buildCallOrSideCall`'s missing-extern assert (`Backend/src/function/expressions/externs.cpp`). The
   backend's intrinsic switch there recognizes only `__vbi_`-prefixed names, and the interop leaf path
   materializes `rust` leaves, not builtin externs. Worked around in the corpus + the chrono sample by
   returning `i64` from `main` directly (the backend truncates for the exit code) instead of calling
   `TruncateI64ToI32`. Fix: lower non-`__vbi_` builtin externs on the interop path, or back them with
   `__vbi_` intrinsics.
9. **Synthesizing an unnameable-but-enumerated type `vfail`s instead of declining (defensive, no red test).**
   `synthesize_extern_function` (`declarations.rs`) returns `None` when a type can't be named, and
   `illuminate_function` (`compiler.rs`, core) treats `None` as a bug and panics. After the i64 fix no
   current type triggers this (every scalar `lower_ty` accepts, `vale_type_name` now names), so a *future*
   divergence between the two would crash rather than decline. Harden by returning
   `Some(Err(CouldNotPostparseReason::…))` on an unnameable type so it surfaces as a clean
   `CouldNotPostparseFunction` diagnostic.
10. **`[[bin]]` in `Valen.toml` could default.** cargo only auto-discovers `src/main.rs`, never a `.valen`,
    so the orchestrator requires an explicit `[[bin]] { name, source }` to point cargo's generated
    `[[bin]] path` at the crate root (`generate_workspace`, `orchestrator.rs`); an empty `bins` yields no
    cargo target and cargo builds nothing. Default it — assume `src/main.valen` with `name = <project
    name>` when `[[bin]]` is absent (mirroring cargo's `src/main.rs` convention) — to trim the manifest
    boilerplate.

**Debugging under interop (source-level DWARF for Valen code in a rustc build).** Standalone Valen
binaries emit real DWARF — function/statement/local DIEs, and (new) a real `DW_AT_comp_dir` so lldb
can `list`/show source and source-pattern breakpoints (`br s -p`) resolve. Interop Valen code emits
**none**: `collect_new_rust_requests` lowers with an **empty** code map
(`rust_interop/mod.rs`, the `empty_code_map` at the `populate_metal_cache` call — "interop debugging
out of scope"), so every node's `SourceLocation` resolves to a no-op and no Vale DWARF is produced;
the interop `compile(BackendInputs { … })` call passes `absolute_source_paths: vec![]` to match.

The **backend is already mode-agnostic** — no `Backend/` work needed. `compileIntoModuleFromRustc`
already calls `loadSourcePaths` and `finalizeCompile` (→ `finalizeDebugInfo`), and
`getOrCreateDIFile`/`getOrCreateCompileUnit` resolve each file's directory from
`GlobalState::sourcePaths` (the standalone comp_dir fix: a `basename → abspath` map conveyed on
`BackendInputs.absolute_source_paths`, with the CU anchored to a user source file). So turning it on
is all frontend (`rust_interop/`, AI-editable) plus one real unknown:

1. **Spike the coexistence question FIRST — it decides easy-vs-rabbit-hole.** Interop emits Vale IR
   into rustc's *borrowed* `LLVMModule`, which already carries rustc's own `DICompileUnit`/debug info,
   and rustc owns optimization + object emission + dsymutil. Whether a second (Vale) CU + its DIFiles
   survive in that module and are consumed by the debugger is untested. Prove or disprove this with a
   throwaway `--debug` interop build before doing the threading; if Vale DWARF is dropped or corrupts
   rustc's, that's the blocker to solve first.
2. **Give the interop lowering a real code map + source paths.** Capture the Vale source code map the
   typing compilation already builds (standalone taps `compilation.get_code_map()`) and the `.valen`
   source file absolute paths, stash them on `DriverState`, and thread them to `populate_metal_cache`
   (replacing `empty_code_map`) and into the interop `BackendInputs.absolute_source_paths`. That alone
   makes `SourceLocation`s resolve and `sourcePaths` non-empty.
3. **Plumb `--debug` into the interop options.** Interop opts come from cargo/`valen build`, not
   `valec -g`; today nothing sets `opt->debug` on the interop path — pick the trigger (a `valen build`
   flag / cargo profile).
4. **A new debug test harness.** Every debugger gate is standalone (`compile_inline_debug` → lldb);
   interop debugging needs a rustc-driven build + lldb gate (extend the interop harness). Keep interop
   debugging parked until the step-1 spike is green.

**Medium-term goal — a standalone Rust program calls into a Vale library** (Phase 3 libraries; demo target
`../testproj3`). The built "Rust → Vale" today is the reverse-callback direction only, and it runs inside a
**Valen-driven** build (a `.valen` crate root, entry `__vale_main`); there is no path where an ordinary
cargo-built Rust program (its own `fn main`) depends on a Vale crate and calls its exported functions. Three
gaps: (1) per-export inbound symbol capture is special-cased to `__vale_main` (recorded in
`DriverState.entry_symbol`, `src/instantiating/rust_interop/mod.rs`) — generalize it so every export is
emitted under its rustc-mangled name, the inbound mirror of `FunctionExternI.link_name`; (2) the pass-2 typing-driven `lib.rs` (exported
declarations from `HinputsT`) is unbuilt — `generate_pass2_stub` (`stub_gen.rs`) projects only anon
substructs today; (3) the build is always Valen-driven into a bin (`run_build`, `orchestrator.rs`) — a
Rust-driven build that consumes a Vale crate as a cargo dependency does not exist. Also needs the two rustc
passes for a library crate. A genuine `../testproj3` (a Rust `fn main` calling a Vale-exported function)
requires all three; it is not assemblable from today's capability.

The **`vale`→`valen` rename** (internal symbols — `__vale_main`,
`__VALE_STUBS_MARKER`, `is_vale_codegen_target`, the argv[0] literals — still spell `vale`; the two bins are
already `valenc-rs`/`valen`) is a mechanical tree-wide sweep.

Durable facts:
- **Almost entirely AI-editable.** The whole rustc-integration lives in `rust_interop/` dirs (including the
  orchestrator). The only in-repo core is *reads* of `HinputsT` (`src/typing/hinputs_t.rs`) and the export AST
  (`FunctionExportT`/`KindExportT` in `src/typing/ast/ast.rs`, `FunctionExportI` in
  `src/instantiating/ast/ast.rs`), and the root `Cargo.toml` (the `toml`/`serde` deps + the `valenc-rs`/`valen`
  bins) — which Guardian's AFEOX shield blocks regardless of "fire core edits", so a human must disable
  Guardian for a `.toml` edit.
- **Pure-Rust byte-identity (@PRCCBIVRZ) is dispatch-gated, and built.** `run_wrapper` routes a non-`.valen`
  crate to `NoopCallbacks` + `run_compiler` with no overrides installed, structurally identical to vanilla
  rustc. `Config::make_codegen_backend` is a *stock* `pub` field on `rustc_interface::Config` (honored in
  `run_compiler`, and what miri uses to install `DummyCodegenBackend` with a closure field), but a delegating
  wrapper backend does not help here: `codegen_crate<B>` is monomorphized with `B = LlvmCodegenBackend`, so
  the hook must live on rustc's own backend type (short-term Next #6). The fork's only interop patches are
  the `per_instance_mir` query and the `fill_extra_modules` hook.
- **Two-pass needs no on-disk cache.** `HinputsT` is arena-bound and unserializable (no derives), but its
  arenas live in the driver frame independent of the `tcx`, so one `HinputsT` survives two `run_compiler`
  calls in one frame. Defer the design's `.vale-cache`/`after_analysis` serialization until cross-invocation
  reuse is actually needed.

## The C++ backend under interop

The C++ backend (`Backend/`, ~60 files) builds and links against the fork's shared libLLVM 21 in
**both** builds — a state that used to hold only for the standalone (non-interop) build. `build.rs`
derives the fork's `llvm-config` from the toolchain sysroot (`<sysroot>/../llvm/bin/llvm-config`), links
the single `libLLVM` dylib (`--link-shared`, not per-component static archives) and bakes an rpath to
it; `Backend/CMakeLists.txt` uses `find_package(LLVM ...)` with **no** version pin (the fork's
`21.1.8-rust-dev` suffix defeats a numeric `find_package(LLVM <N>)` match, and build.rs controls the
version through `LLVM_DIR`). Under `--features rust_interop` the backend now links **alongside** rustc's
`librustc_driver`, and both resolve to the one shared libLLVM — no dual-LLVM duplicate-symbol UB, which
is what the old backend-disable early-return in `build.rs` guarded against (that gate is gone; only the
`no_backend` feature still skips the backend). The LLVM 16 → 21 source port was mechanical (see the
Lessons entry).

The single backend entry `backend_compile` (`vale.cpp`) dispatches by mode to two `static` internals of
`vale.cpp`: `compileStandalone` (owns context/machine/module + object emission) and the **borrowed-mode**
`compileIntoModuleFromRustc`. `Backend/src/rust_interop/rust_interop.cpp` — which holds
`emitInboundCallbackWrapper` (the Rust→Vale reverse-callback wrappers; see "Reverse direction") — is the one
AI-editable corner of the otherwise-core `Backend/`. The borrowed mode takes an `LLVMContext` + `LLVMModule` as
opaque `void*`: `consumer_fill_modules` (`src/instantiating/rust_interop/mod.rs`) mints the module with rustc's
own `ModuleLlvm::new`, hands the backend `llcx_raw_mut()` / `llmod_raw()` (**not** the TargetMachine, which the
fork never exposes), and returns the filled module to rustc as a `ModuleCodegen::new_regular`, so it rides
rustc's optimize/ThinLTO/emit pipeline as one more CGU (`ModuleKind::Regular`: `Finished` in debug,
`NeedsThinLto` in release). rustc owns optimization, object emission, and disposal, so this path does
**not** optimize, `generateOutput`, dispose the handles, or call `generateExports`. `GlobalState`
sources its data layout from the module (`LLVMGetModuleDataLayout`), which rustc pre-set. Both paths run
the shared `finalizeCompile` tail, which `LLVMVerifyModule`s the emitted module when `verify` is set and
returns a `VerifyFailed` rc rather than `exit`ing (so a driven in-process test fails cleanly, not a
whole-binary abort); every test compile sets `verify` (interop in `emit_vale_into_borrowed_module`,
standalone e2e in `end_to_end_tests/mod.rs`), so malformed emitted IR fails the suite. Shared with
the standalone path: `compileValeCode` emits the Vale functions and, when the program exports a `main`,
the region setup/cleanup + `__Vale_Main` wrapper, returning its prototype (or `nullptr` for a library
with no `main`); the caller then emits an entry via one parameterized `makeEntryFunction(name,
emitLibcShim)` — standalone emits libc `main` (with the argc/argv + wasi shim), interop emits the entry
under the rustc-mangled `__vale_main` symbol the driver threads in through the FFI (external, no shim;
rustc's libstd owns `main`, and the stub's `fn main` links to this body). `compileIntoModuleFromRustc`
takes that entry symbol (via `BackendInputs.Interop`); the `fill_extra_modules` hook reaches it through
`compile(BackendInputs)` → `backend_compile` (see "Running a program").

## The interop-specific core touch-points (design + code)

Three edits in the core typing pass exist solely for interop. Each mirrors existing struct code and is
`#[cfg(feature = "rust_interop")]`-guarded, so a normal build is byte-identical to before:

- the `rust_method_entries` hook in `precompile_interface` (`struct_compiler.rs`) — attaches an enum's
  methods/drop, twin of the one in `precompile_struct`.
- the `RustImportSeed` match in `Compiler::evaluate` (`compiler.rs`) — seeds a struct **or** interface,
  and for a synthesized interface also registers its abstract methods into the postparsed function
  cache, mirroring the native `program_a.interfaces` indexing loop (a synthesized trait's methods reach
  resolution only through this).
- the `is_rust_backed` skip in `compile_interface_core` (`struct_compiler_core.rs`) — keeps a rust
  enum's inherent methods out of the interface vtable, but **excepts a synthesized trait's abstract
  methods** (identified by an `AbstractBody` via `peek_postparsed_function`), which must enter the
  vtable so an `impl` resolves against them; the `compile_struct_core` twin has no such exception.

The instantiator has one more, also `#[cfg]`-guarded: recording a Rust leaf's request at the
`ExternFunctionCall` node in `translate_ref_expr` (`src/instantiating/instantiator.rs`), plus the three
`pub(crate)` seams it exposes (`instantiate_exported_function`, `drain_instantiation_queue`,
`assemble_hinputs`). See the inversion section. The lazy-postparse defer in
`get_or_create_postparsed_function` (`compiler.rs`) is `#[cfg]`-guarded too.

A few core changes this work required are **not** interop-gated, because they complete previously-stubbed
*general* paths or serve both modes (the default suite guards them): `InterfaceDefinitionT::
generic_param_types` (was `unimplemented!`; needed once an enum's drop is compiled), the extern-signature
check permitting extern **Interfaces** so an imported Rust enum is allowed in an extern's signature (see
the Lessons entry), the deferred-compile drain deriving each function's outer env from its id in
`evaluate_generic_function_from_non_call`, the single-symbol backend naming: `FunctionExternI` now
carries one `link_name` (the real callee symbol), `declareExternFunction` (`Backend/src/function/`) binds
it verbatim vs. composing the `vale_abi_` shim by a `packageCoordinate->projectName == "rust"` check, and
`makeEntryFunction` takes the entry symbol, and — for the `&mut` stub rendering — `Compiler::evaluate`
(`compiler.rs`) now returns `(HinputsT, CompilerOutputs)` rather than just `HinputsT`, with
`TypingPassCompilation` (`compilation.rs`) caching both (`coutputs_cache`, `cached_coutputs`). The postparsed
`FunctionS.effects` carry the `mut(g)` the typed `KindT` drops (@BCHATZ), so retaining `CompilerOutputs` past
`evaluate` is what lets the pass-2 stub read them; the standalone `pass_manager` path just ignores the second
tuple element.

The AI-editable interop code is any `rust_interop/` directory: `src/typing/rust_interop/`,
`src/instantiating/rust_interop/`, and `Backend/src/rust_interop/`. Everything else in `src/typing/`,
`src/instantiating/`, and **all of `Backend/`** is core; a change there needs the architect's explicit
literal "fire core edits" (per the `.claude/CLAUDE.md` in each). Reading core to diagnose or propose is
fine; editing it is not, without that phrase.

## Governing invariant

Whether a postparsed denizen exists must be undetectable to callers: the only operations are "ask an
environment what it holds" and `get_or_create_postparsed_*` by id (always returns, building on a miss).
A read that memoizes is indistinguishable from a pure read, which is what makes lazy synthesis clean.
The `// VCOORD` on the sealed tables in `compiler_outputs.rs` records the enforcement plan.

## Lessons learned

- **Prioritize simplicity very highly in rust interop; its complexity is the highest medium-term risk we
  have.** Prefer one uniform mechanism over a fast path plus a general path, even when the fast path is
  cheap: e.g. every Valen crate runs two rustc passes, with no one-pass short-circuit for the empty-stub
  case. A special case is a second thing to keep correct.
- **`lower_ty` (enumeration) and `vale_type_name` (synthesis) must agree on the scalar surface.** `lower_ty`
  accepted rust `i64` but `vale_type_name` named only 32-bit ints, so an `i64` import was offered as an
  overload candidate then `vfail`-panicked at `illuminate_function` (`compiler.rs`) when materialized — the
  `None` synthesis returns is the "genuine bug" path there, not a decline. When a plausibly-supported import
  panics with "no postparsed function," suspect an enumeration-vs-synthesis disagreement over a type, not a
  missing feature.
- **A `#[track_caller]` Rust fn's `fn_abi` has one more argument than its `fn_sig`** — a hidden trailing
  `&Location` (@TCHAPZ) — so building the Vale prototype from `fn_sig` and the ABI descriptor from
  `fn_abi_of_instance` disagree on arg count for every such callee. Detect via
  `instance.def.requires_caller_location(tcx)` and *keep* the arg (`LocationPtr`, a null ptr); dropping it to
  match the signature is ABI-level UB (the callee reads one arg more than the caller provides). Common:
  panic-on-error / overflow-checked constructors across real crates are `#[track_caller]`.
- **Reduce a real-crate interop bug against the exact declaration — attributes included, not just the type
  shape.** A local fixture mirroring `chrono::TimeDelta`'s struct shape did NOT reproduce the
  `buildBoundarySignature` abort, because the cause was the `#[track_caller]` *attribute*, not the 16-byte
  struct. A shape-only repro passing is not evidence the bug is elsewhere.
- **Only `__vbi_`-prefixed builtin externs lower on the interop path.** A builtin `extern func` without that
  prefix (e.g. `TruncateI64ToI32`, `arith.vale`) has no interop `FunctionExternI` and aborts the backend at
  `buildCallOrSideCall`'s missing-extern assert. Don't reach for such a builtin in an interop / `valen build`
  program until Next #8 lands — e.g. return `i64` from `main` directly (the backend truncates for the exit
  code) instead of calling `TruncateI64ToI32`.
- **A nextest `leaky` flag on a `backend_ffi::metal_cache` FFI test is known noise, not the P0 nondeterminism
  to halt on.** It lands on a *different* C++-FFI test each run because it is a leak-*timeout* heuristic; the
  test RESULTS are deterministic (all pass), only the WARNING moves. Ignore it — do NOT stop and re-investigate
  — unless one becomes a real `FAILED` (that would be different).
- **The pass-2 stub renders `&mut` from the abstract method's `mut(g)` EFFECTS, never from the typed `KindT`.**
  `KindT` drops borrow mutability (@BCHATZ), so mutability is recovered from the retained postparsed
  `FunctionS.effects` (`param_tyype_is_mut` matches a param's `BorrowRef` region group against `EffectS::Mut`
  groups structurally). Do not try to recover `&mut` from `KindT` or add mutability to `KindT` — retain
  `CompilerOutputs` past `evaluate` instead.
- **Generated denizens must be threaded into package stores by their FULL nesting (`{coord + init_steps}`),
  never flattened to the package top level.** The anon-substruct macro nests the substruct's `drop` under
  `[AnonymousSubstructTemplate]` (like every struct drop) while the constructor/forwarders are top-level.
  The native denizen loop groups by `name.init_steps` (`compiler.rs` ~793), so each lands in the store its
  id names; a flattened drain (an early `per_crate`-style `init_steps: []`) put the nested drop's env entry
  at top level, so drop-call resolution built a top-level `drop` super_template that no nested `drop`
  definition matched → `translate_prototype` `vassert_one` empty (instantiator.rs). Do not force-compile the
  nested denizens either — the function-compile loop skips non-empty init_steps by design; they compile
  on-demand at their call sites once nested correctly.
- **Anon-eligibility (and any per-method value-position check) must run on the MAPPED sig, not the raw one.**
  A trait method's raw sig has `self` as `&Generic(Self)`; `synthesize_extern_trait` maps `Self` → the
  interface citizen before building params. Checking the raw sig sees `&Generic` (no value-position form)
  and marks every trait ineligible. Compute eligibility inside `synthesize_extern_trait`, on `mapped_sig`.
- **A synthesized abstract method's param/return `tyype` must be value-position (name-`Call`), mirroring the
  parser — not the internal value-rune.** `resolve_citizen_bounds` evaluates a `where func` bound's types
  with only the citizen env + substitution map (no header rules), so a bare internal `value_rune` can't be
  substituted (`evaluate_templex: can't substitute`). Native anon substructs dodge this because their method
  params ARE interface generics (in the substitution map); a Rust trait's concrete params are not. Build them
  the way the parser does (`value_position_type_st`): a Rust citizen resolves by its SHORT single-segment
  name via unconditional cross-package lookup (do not reach for the multi-segment path).
- **The pass-2 stub is APPENDED, written to `--out-dir`, and predeclares `__ValeOpaque` only if absent.**
  Prepending anything (even the `__ValeOpaque` predecl) pushes `#![register_tool(vale)]` off the crate-root
  top, so `#[vale::emit_consumer_body]` stops resolving (E0433). Writing it next to the crate root pollutes
  checked-in fixtures (the harness crate root is a source fixture) — write to `--out-dir`. The pass-1
  generated stub already declares `__ValeOpaque`; a hand-written fixture stub does not — predeclare only when
  the pass-1 text lacks it, or you double-define it.
- **Never key a Rust item's identity on a human-name string (@ATAFLBZ).** Matching the anon-substruct
  instance to its edge by `human_namee.as_str() == …` trips the `no_rust_item_identity_comes_from_a_human_name`
  guard. Compare the interned template names structurally instead (identity/`==` on the interned name), which
  is also correct (two crates can share a short name).
- **Guardian AFEOX's editable-extension allowlist is stale: it has `.vale` but not `.valen`.** Editing a
  `.valen` test fixture is refused. Do not loosen the shield yourself; the fix is a one-line AFEOX allowlist
  addition (architect / guardian-diagnose), or a temp-disable to apply a staged patch.
- **A Vale struct crosses to rust as an opaque `__ValeOpaque<HASH>` wrapper, NOT with real fields — and the
  nested functor IS a separate rust type (`__ValeOpaque<typeid>`), not just bytes.** Do not conclude from
  "rust sees an opaque blob" that the functor needs no rust representation: the wrapper-as-field shape is
  `pub struct MyCb<F>(__ValeOpaque<HASH>, PhantomData<(F)>)`, and a Vale-internal type used as a rust generic
  arg (the functor) is rewritten to `__ValeOpaque<typeid>` (arch §10.7 Case 2). rustc keeps MyCb's DefId but
  never decomposes it; Vale owns the size via a `layout_of` override. Sky ships exactly this end to end
  (`toylangc/src/stub_gen.rs:138,189`, `oracle.rs:1330`, `rustc-lang-facade/src/queries/layout.rs`).
- **The "where does the opaque size come from" question is answered by rustc, not a Vale size computation.**
  Vale computes struct sizes only in its C++ backend (via LLVM), never in the Rust frontend. The `layout_of`
  override doesn't compute sizes: the consumer returns each field's concrete rustc `Ty`, and rustc's own
  `tcx.layout_of` sizes them recursively (bottoming out at primitives / imported types), plus a manual
  C-offset walk. No backend round-trip. Mirror `rustc-lang-facade/src/queries/layout.rs:65-206`.
- **The `ExternBody` compile arm now threads `instantiation_bound_params` — do not reintroduce the empty
  hard-code.** `make_extern_function` (`function_compiler_core.rs`) takes the solved bounds and puts them on the
  `FunctionDefinitionT`, like the CodeBody/AbstractBody/GeneratedBody arms; only the leaf `ExternFunction`
  prototype stays bound-free (rustc discharges its trait obligation; `translate_prototype` returns early for a
  leaf, `instantiator.rs:1027`). Before this, `ExternBody` + non-empty `impl_bounds` was silently mis-compiled
  (bounds dropped) — a shape pure Valen never produced, so it went unnoticed. Native `try_as`
  (`src/builtins/resources/as.vale`) is `GeneratedBody` (quoted `extern("name")` → `BuiltinAttribute`), which
  always threaded bounds — it is not a counterexample.
- **A concrete override for a *generic* impl is an INSTANTIATOR product, never a typing product — for anyone.**
  Do not try to make typing (`compile_i_tables`/`look_for_override`) record `on_tick<lambda>`; typing mints one
  override per (impl × abstract method) at the impl's own generality — concrete for a non-generic impl
  (`getFuel<Raza>`), generic for a generic impl (`on_tick<F>` + a `CaseRuneFromImpl(F)` case-override under the
  `OverrideDispatcher`). The concrete monomorphization is `translate_override` (`instantiator.rs:746`), driven
  by a virtual dispatch (`InterfaceFunctionCall` → `new_abstract_funcs` → `translate_abstract_func`). The
  reverse callback has no Vale virtual call (Rust calls the override), so nothing triggers it — that, not a
  "typing forgot to record it" gap, is why the old `collect_callback` missed. **The fix resolves the override
  STATICALLY via `resolve_override_prototype` (not `translate_override`)** — see the reverse-callback lesson
  below; a non-generic callback is the degenerate zero-subs case, never a separate path.
- **Impl-bounded method calls devirtualize to static calls on `main` — do not re-introduce virtual dispatch for
  them.** `cb.method()` on a `where implements(C, Bork)` generic compiles to a direct call to the concrete
  override (a `BoundFunctionCallTE` node), not an upcast-and-virtual-dispatch that builds an interface fat pointer.
  Generic-impl *virtual* dispatch itself is also functional natively (`generic_forwarder_plain_interface_virtual_dispatch`
  passes). So a backend `makeInterfaceFatPtrWithoutChecking` assert
  (`LLVMTypeOf(ptrLE) == getInterfaceRefStruct(...)`, `Backend/src/region/common/defaultlayout/structs.cpp`) on a
  static-dispatch program means something instantiated the *virtual* path (an abstract dispatcher method) where it
  should have resolved the override statically — see the reverse-callback lesson below; it is not a backend bug.
- **A `where func` bound prototype cannot introduce a region (or any) generic parameter, so a churning
  closure forwarder cannot be borrow-CHECKED** (it builds and runs fine under `--no-borrow-check`; only
  enforcement is missing). `parse_prototype` (`src/parsing/templex_parser.rs`) parses
  `func <name> ( <tuple> ) <return>` with no `<…>` slot, so a churn region declared on a bound
  (`func __call<r'>(&F, &Win in r, &Inp) mut(r)`) is `ParseError::BadPrototypeParams`, and left free the region is
  an undetermined `ImplicitRune` → `CouldntSolveRuneTypesT` (unsolved param-type `KindList`).
  `generic_forwarder_with_churning_closure_param_compiles` (`src/typing/test/after_regions_tests.rs`) is GREEN
  — it exercises the borrow-check-OFF path (`compiler_test_compilation_without_borrow_check`), not a red
  marker for enforcement. This is a feature — per-call region quantification on bound prototypes — not a
  syntax anyone missed. Orthogonal to `&mut`-signature support, which is complete.
- **When the instantiator (vanilla, extensively tested) panics on a vanilla-shaped need, suspect rust_interop
  is feeding it a shape pure Valen never produces — do not remove the assert and do not assume a core gap.**
  The `rune_to_impl_bound_arg.len() == rune_to_bound_impl.len()` asserts (`instantiator.rs:659,1065`) caught a
  real self-inconsistency (call resolved 1 bound-arg, callee template declared 0); removing them just moved the
  same mismatch one layer down. The mismatch was the (now fixed) `ExternBody`+`impl_bounds` drop, not the
  instantiator.
- **`where implements(..)` (impl bounds) as instantiation bounds was an onion regression, now fixed on `main`
  across several layers** (define-side impl-bound harvest in `resolve_conclusions_for_define`; an instantiator
  ref-peel; TestVM stubs; the edge-blueprint template-keying). A minimal pure-Valen repro is a
  `where implements(T, IShip)` generic merely called with a concrete type
  (`impl_bounded_generic_is_merely_called_with_a_concrete_type`, `after_regions_integration_tests.rs`) — it hit
  `instantiator.rs:1065` `1 != 0`. Func bounds (`where func`) were never regressed; only impl bounds. When a
  `where implements` shape misbehaves, suspect the define-side harvest before anything interop-specific.
- **Debugging technique that pinned these: run the interop case AND a native equivalent through the same
  instrumentation and diff** — a probe in `collect_callback` (AI-editable) dumping each `on_tick` denizen's
  shape (`abstract`, `in_bounds_map`, generic-vs-concrete via the `KindPlaceholder`/`CaseRuneFromImpl` in its
  `local_name`) showed only generic overrides exist, no concrete `on_tick<lambda>` — the exact missing datum.
- **ITypeST must evaluate to the same `ITemplataT` the rules do.** A value-position type name lowers to a
  zero-arg template `Call` (@TNLTZACZ) on the rules side *and* the ITypeST side; `evaluate_templex` applies
  it via `predict_struct`/`predict_interface`. Substituting an ITypeST yields a general `ITemplataT` (a
  `StructDefinition` in template position, a `KindT` after applying) — never "always a `KindT`". Trap:
  `params_types` is NOT a dormant migration field — bound satisfaction searches the environments of the
  runes it mentions (@ENECCLZ), so a bound param's ITypeST must name the *substitution* rune (`&G` =
  BorrowRef of the generic), never a flat borrow-result rune whose conclusion is `&lambda`.
- **The reverse callback resolves its override STATICALLY via `resolve_override_prototype` — never
  `translate_override`.** The reverse callback is static dispatch (Rust calls the concrete override directly), so
  `collect_callback` calls `resolve_override_prototype` (`instantiator.rs`, `pub fn`; reads the impl's typed id +
  bound args from `monouts.instantiated_impl_to_typed_impl_and_bounds`) to get the plain concrete override
  (`on_tick<lambda>`, struct receiver) directly — no vtable, no abstract dispatcher method. Driving
  `translate_override` (the *virtual*-dispatch path) instead instantiates the abstract dispatcher
  `on_tick(&MainLoop,…)` whose body virtual-dispatches, and the backend aborts building an `&MainLoop` interface
  fat pointer (a synthesized extern interface has no interface-ref-struct layout). To identify the impl, Rust's
  `instance.args` (carrying `C = __ValeOpaque<typeid>`) is decoded via `read_opaque_typeid`
  (`ty::Const::try_to_leaf().to_u64()`) + a presence check against `DriverState.opaque_universe`, and matched
  by forward-projecting each candidate through `citizen_or_opaque_to_rustc_ty`.
- **Debugging a re-enabled or migrated feature: build the last commit where it passed and print the same
  values.** That pinpoints what the migration relocated — onion moved a method's type-connecting rules off
  `header_rules` onto per-param, and moved citizen-bound resolution from rule-running to flat substitution
  (`9f5ca549`), which is what the anon-substruct macro's inherited runes tripped over.
- The interop `--lib` gate runs only when a commit touches `rust_interop/**` (the host
  `fire-commit-config.toml` gates it), so a typing- or instantiating-pass landing that doesn't can reach
  `main` with the interop suite untested. Re-run `+rustc-fork` interop after rebasing onto any such landing:
  a group-param-flow change once left three instantiator sites without a group-templata arm, a latent panic
  only a driven generic-region-param test (the reverse+forward `&mut` E2E) surfaces.
- A group generic parameter now reaches the instantiator (the postparser no longer strips it); it is the
  ceremonial empty `GroupTemplataI` (mirrors typing's `GroupTemplataT`), never read — `translate_templata`
  yields it, `assemble_placeholder_map_inner` adds no mapping for it, `instantiated_humanizer` renders
  `"Group"`. Do not look for a payload.
- `mut(a, b)` (two groups in one effect clause) is a parse error (`parse_group_in_parens`, `parser.rs`) —
  each clause takes one group, so write `mut(a) mut(b)`; the old parser silently kept only the first group.
- The borrow checker no longer accepts a groupless borrow: a Rust import's borrow return (`&Glyph`,
  `&V`) needs an importer-synthesized `&T in g` group and `mut(g)` effects, or it is rejected as
  underivable — the old silently-untracked `Empty` group is gone.
- A Rust method's receiver borrow (`&self`) splits per @PFVSZ: the argument binds to the **value** rune,
  and the borrow concludes a separate **full-type** rune. Wiring the borrow onto the argument rune makes
  the peeled receiver fail `KindIsNotBorrowRef`.
- A synthesized extern's borrow of a **generic** — a `&C` parameter, or a `&T` return / nested borrow
  (`at<T>(&Vec<T>, i64) -> &T`) — must wrap the rune `bind_sig_type` *returns* (the generic's own rune),
  not the rune offered to it: for a generic inner `bind_sig_type` adds no rule binding the offered rune, so
  wiring the borrow onto it leaves the type unsolved (`CouldntSolveRuneTypesT` on a `__rust_argN`). Concrete
  borrows (`&Counter`) hide this because `bind_sig_type` returns the offered rune for them. Both sites —
  the parameter split in `synthesize_extern_function` and the `Borrow` arm of `bind_sig_type` — use the
  returned rune; keep it that way.
- A Vale struct passed as a Rust generic's type argument (`run_callback::<MyCb>`) resolves to a **local**
  rustc DefId in the stub crate — `resolve_local_type`, the type twin of `resolve_local_fn` — because
  `resolve_crate_qualified_path` walks only loaded dependency crates. Branch on
  `id.package_coord.module.0 == RUST_MODULE` to pick dependency vs local resolution; a local type reaching
  the dependency path is the `ARGS-UNCONVERTIBLE` firing, not an `UNRESOLVED` one.
- The "arity underflow" for `Vec::new` is self-inflicted by over-specifying the call. `new` has one own
  generic (`T`; the impl pins `Global`), so `Vec.new<int>()` / `Vec<int>.new()` supply one arg and never
  underflow — `Vec<int, Global>.new()` is what breaks it. The full arity is written only when `Vec` is
  named *as a type*; there is no default-generic-param support and none is wanted.
- Putting a Rust type's methods in its outer env force-compiles them unless the citizen-compile loop skips
  `is_rust_backed` — for structs (`compile_struct_core`) and for a rust enum's inherent methods in
  interfaces (`compile_interface_core`). Do not remove the skips — but the interface one **must except a
  synthesized trait's abstract methods** (an `AbstractBody`): those are the interface's vtable contract,
  and skipping them drops them from `InterfaceDefinitionT.internal_methods`, which silently disables
  override enforcement (a missing override then compiles).
- A manufactured drop recovers its owner from the id's last `init_step`: a `StructTemplate` for a struct
  owner, an `InterfaceTemplate` for an enum owner. `create_postparsed_function`'s drop branch must handle
  both, or an enum's drop vfails.
- Generic scope-end drop resolves now — the generated `drop<T>(Owner<T>)` call infers `T` from the value.
  Do not re-assert "it does not resolve" from stale comments.
- `usize` is a real primitive with no literal syntax and no operators — produce-and-pass only.
- The onion typing work (`docs/handoffs/exp-2-handoff.md`) cleared what interop needed: generic
  substitution through reference wraps and argument types reaching the call-site solve. Interop builds on
  those; do not re-derive them as blockers.
- Under `rust_interop` a Rust callee is a **leaf**: the instantiator records it as a request at its
  `ExternFunctionCall` node (not by intercepting the wrapper prototype in `translate_prototype`), and
  `per_instance_mir` reifies it for rustc's collector. Sky (Harmonious) rejected per-call-site wrappers
  because they can't be generic; the typing-pass extern wrapper (one per Rust fn, body =
  `ExternFunctionCall`) is fine and is what the node-level recording relies on. The non-interop
  instantiator is unchanged and never touches Rust.
- A *called* Rust function does not compile itself — a callsite never adds to coutputs, and the
  top-level compile loop skips the `rust` package. So core compiles it lazily: when its postparse is
  created (`get_or_create_postparsed_function`), core registers it and `defer_evaluating_function`s it,
  and the deferred-compile drain runs `make_extern_function` on it (wrapper + `function_extern`) exactly
  as the top-level loop does for a user `extern func`. Without that defer, a driven
  `func main(){ add_two_numbers() }` yields `hinputs.functions == [main]`, `function_externs == []`, and
  the backend calls an undeclared symbol.
- An extern function's signature may reference an imported Rust **enum**, which is a `KindT::Interface`
  (with an `Extern` attribute), not a `KindT::Struct`. `make_extern_function`'s non-exported-kind guard
  (`kind_is_fine_in_extern_func` in `compiler.rs`) must whitelist extern Interfaces the same way it
  whitelists extern Structs, or `ExternFunctionDependedOnNonExportedKind` rejects any Rust fn that touches
  an enum — an enum's drop, `Vec::pop` (returns `Option`), a method on an imported enum. The symmetric
  *exported*-kind checks likely share the gap if exercised.
- rust_interop is a pure *producer*: `create_postparsed_function` takes `&CompilerOutputs` (read-only) and
  only synthesizes the `FunctionS`; core (`get_or_create_postparsed_function`) owns registering it and
  deferring its compile. Keep it that way — AI-editable `rust_interop` must not mutate core accumulator
  state, and the shared ref enforces that structurally.
- A new `RustOracle` trait method MUST be forwarded in `LoggingOracle` (`logging_oracle.rs`) — it is a
  decorator over `self.inner`, and a method left to the trait default silently returns the empty/`None`
  answer, so the real oracle's answer never reaches the caller. `deref_target_imports` returning `[]`
  through the un-forwarded decorator (while the underlying `TyCtxtOracle` had the target) is what made
  autoderef targets vanish; the file's own note on `resolve` warns of exactly this.
- `tcx.impl_trait_ref(impl_did)` returns `EarlyBinder<TraitRef>` directly in this fork (not
  `Option<..>`); `.instantiate_identity().self_ty()` reads the impl's self type. `for_each_relevant_impl`
  can yield blanket impls, so match the self type's `ty_adt_def().map(|d| d.did())` against the owner
  before trusting it.
- **A query override must not `borrow()` `DriverState.monouts`: rustc re-enters it while our own resolve
  loop holds `monouts` mutably.** `resolve_new_requests` (and the callback's inbound-ABI read) run
  `compute_extern_abi` → `fn_abi_of_instance` → `layout_of` under the `borrow_mut()` taken in
  `collect_new_rust_requests`, so an override reading `monouts` through its `RefCell` panics
  "already mutably borrowed". Anything an override needs goes in its own cell (`opaque_universe`), filled
  right after each drain, before the queries that read it.
- A hand-built `LayoutData` needs `rustc_hashes::Hash64` for its `randomization_seed`; `rustc_hashes` is a
  sysroot `rustc_private` crate the fork ships, declared in `src/lib.rs`'s interop `extern crate` block.
- **A Valen type must be `!Freeze` to rustc, and only a real `UnsafeCell<()>` field does that.** rustc
  emits `noalias readonly` on every `&T` parameter with `T: Freeze`, which is a miscompile for a pointee
  Vale writes through an aliasing group borrow. `PhantomData<*mut ()>` and `PhantomPinned` remove
  Send/Sync/Unpin but not Freeze (`core::marker` has explicit `impl Freeze for PhantomData<T>` and raw
  pointers), so do not "simplify" `__ValeOpaque`'s `UnsafeCell<()>` into a `PhantomData` of anything.
- The driven tests key their rustc scratch dir per run on a `tempfile::TempDir::new()`, not on the `Case`
  name. A `Case` is a shared `const` reused by several `#[test]`s (e.g. `CALLS_A_RUST_FREE_FUNCTION` drives
  three), and cargo runs `#[test]`s on parallel threads in one process, so a name-keyed dir let them build
  `libmycrate.rlib` into the same place concurrently and corrupt each other (`failed to map object file` /
  `No such file`) — timing-dependent, clean only under `--test-threads=1`. This is the filesystem twin of
  "per-driven-run state must not be a global"; `env::temp_dir()` is the shared system root, not unique.
- The program is instantiated **once**. The borrowed emit finalizes the *driven* `monouts` via the
  `assemble_hinputs` seam (extracted from `translate_program`'s tail) — it does not re-run
  `translate_program`. A Rust extern is created at its definition point: the eager loop for a C extern
  (from `hinputs.function_externs`, `extern_name` in hand), the provider for a Rust leaf (its mangled
  symbol is knowable only after resolution). Do not reintroduce a second whole-program pass, and do not
  push a placeholder `FunctionExternI` at the call site to overwrite later.
- The provider reaches the instantiator state through a **scoped thread-local pointer, not a `'static`**.
  `run_compiler` joins its spawned thread synchronously, so the state can live in the driver's stack
  frame (real lifetimes) and the callbacks can `unsafe impl Send` — the `std::thread::scope` guarantee.
  Reaching for `'static`/ouroboros/leak here is unnecessary; don't.
- Per-driven-run state must not be a global. cargo runs the driven tests in **parallel**, so a shared
  `static` log/results buffer races — a test can see another's data or none of its own (a P0
  nondeterminism). Keep such state in the per-run `DriverState`.
- Vale represents `Vec<int>` as `Vec<int, Global>` with **both** args explicit (the allocator is a real
  imported type), so `build_generic_args` never has to synthesize a defaulted allocator param. Do not
  assume Vale under-names generic-type args.
- The backend-driving test suites (`end_to_end_tests`, the `backend_ffi` FFI tests) are
  `cfg(all(test, not(feature = "rust_interop")))`. The interop build now *does* link the C++ backend
  (shared LLVM 21), so the old "missing `metal_cache`/`backend` symbols" abort no longer applies — but
  these suites drive full owned-mode codegen/execution through `pass_manager::build`, which the interop
  path never exercises and which is untested against the interop rustc-private linkage. Leave them gated
  until someone verifies they pass under the feature; do not un-gate blindly.
- The LLVM 16 → 21 backend port was mechanical: `Backend/` is pure LLVM-C (no `IRBuilder`), already
  opaque-pointer (`LLVMBuild*2` everywhere) and on the new PassBuilder, so the whole port was deleting
  three dead `llvm-c/Transforms/{Scalar,Utils,IPO}.h` includes (removed in LLVM 21). Do not budget a
  major C++ rewrite for an LLVM major bump here.
- Sharing one libLLVM is mandatory — two in a process is duplicate-symbol UB (process-global pass
  registries / `cl::opt`), so the fork must be built `link-shared`. A `download-ci-llvm` LLVM is
  static-only: no shared `libLLVM.dylib` and no cmake config, so it cannot be linked against. There is
  no shortcut from the prebuilt CI LLVM — `download-ci-llvm = false` plus a from-source build is required.
- TRAP: a plain `./x build` on the fork regenerates the stage1 sysroot **without** the `rustc-dev` component
  (the `rustc_private` rlibs): bootstrap's `RustcLink` copies a compiler's rlibs into a sysroot only when that
  sysroot's compiler builds the next stage, so `./x build --stage 2` is what deposits them into stage1, and
  stage2 itself never gets them. The symptom is dyld failing to load `librustc_driver-<hash>.dylib` from the
  target libdir, or `can't find crate for rustc_driver`. Confirm a sysroot has `librustc_middle-*.rmeta` under
  `lib/rustlib/<target>/lib/` before trusting it carries rustc-dev.
- Do not spell "never disk-cache" as `cache_on_disk_if { false }` on `per_instance_mir` or any query: rustc's
  query macro keys on the modifier's presence and generates the load-from-disk path whatever the value, so the
  line is a no-op that upstream itself deleted elsewhere; omitting the modifier is the convention. The
  architecture doc's §4.2, §22.4.1, and Appendix B.1 still show the old spelling; the design doc's deviations
  list is the authority.
- Do not type-erase the extra modules (`Vec<Box<dyn Any>>` plus a downcast in `codegen_crate`) to get the
  `fill_extra_modules` hook out of the LLVM backend: rustc erases only at the dylib `Box<dyn CodegenBackend>`
  boundary and keeps `B::Module` concrete everywhere inside `codegen_crate<B>`, and the trait method mirrors
  `codegen_allocator`. The global is the reviewable defect, and its fix is a backend field (Next #6).
- `generateExports` is the standalone-valec **C-ABI** export boundary: it writes C `.h`/`.c` files,
  requires an `outputDir`, and emits **no** LLVM into the module. The interop/borrowed path must not
  call it — interop exports go through single-symbol (Vale bodies under rustc-mangled names), not C
  headers. `compileValeCode` (not `generateExports`) is what emits the actual Vale function bodies.
- The backend's extern name is the *real callee symbol* — `FunctionExternI.link_name`, one metal map:
  rustc's mangled name for a Rust leaf (from `tcx.symbol_name`), the user's `extern_name` for a C extern.
  The `vale_abi_<project>_` shim and `getFunctionExternName`'s `projectName_` composition exist only for
  the standalone C-extern path (the backend can't lower the C ABI itself yet — the S9/S10 removal target
  in `Backend/backend-design.md`); `declareExternFunction` picks between them by
  `packageCoordinate->projectName == "rust"`. `FunctionExternI.num_inherited_generic_parameters` is
  written but read nowhere in `src/` — probably dead since the onion rework; check before wiring to it.
- The fork's `ModuleLlvm` exposes only the borrowed `LLVMContext` + `LLVMModule` (`llcx_raw_mut()` /
  `llmod_raw()`), **never** the TargetMachine — by design. So borrowed-mode codegen sources its data
  layout from the module (`LLVMGetModuleDataLayout`; rustc pre-set it, do not re-set it) and MUST NOT
  dispose any lent handle — rustc owns their lifecycle and disposes them after the hook returns.
- `run_case_rustc_driven_full` passes `emit_backend=false` (`harness.rs`): it drives rustc's collector
  and rustc's own codegen of the stub crate, but does **not** emit the Vale backend. So "domino drives
  through codegen, rc 0" says nothing about Vale IR emission, struct sizing, or ABI — none of
  `compileValeCode`/`defineStruct`/the boundary runs. Verify emission at `run_case_rustc_driven_and_run`
  (or `_emitting`), never at `_full`.
- The sret out-parameter path already exists in `buildCallOrSideCall` + `returnNeedsOutParam`
  (`boundary.cpp`) and is reused for `Indirect`. The ABI descriptor's job is the Direct-vs-Indirect
  *choice* — a small struct crosses in a register (`DirectInt`), not sret — plus the right slot type;
  do not rebuild the sret machinery, and do not keep the structural "every struct return uses sret" rule
  for interop externs. Reusing the path is **not** sufficient on its own, though: a direct rustc call
  (no clang shim) also needs the LLVM `sret` type attribute on the out-pointer param (declaration + call
  site), or the pointer lands in the wrong register and rustc's callee scribbles over the stack. See the
  domino-ABI notes in "The aggregate ABI".
- A large struct crosses an interop extern as an **argument** the same way it does as a return: an
  indirect pointer to a caller-owned copy (rustc `Indirect { on_stack: false }`), never LLVM `byval`
  (@EACBIPZ). The `byval` attribute coincidentally survives a lone argument but corrupts the call once a
  second argument follows, so a one-argument test cannot prove the argument ABI — use a second argument,
  ideally behind an sret return.
- A 16-byte 4×i32 struct is `PassMode::Indirect` on aarch64, **not** `Cast`/`Pair` — only structs ≤8 bytes
  cross as `Cast` (a single integer). Read the actual `PassMode` from `fn_abi_of_instance` (a temporary
  print in `compute_extern_abi`) before designing an ABI arm; a source-reading of the aarch64 classify
  code and a downstream user's guess both put a 16-byte struct in the wrong mode.
- A driven case that fails to typecheck panics with the diagnostic (`drive_rustc` in `harness.rs`). Do
  not read a bare undefined `__vale_main` / empty `__vale_main -> []` firing log as an ABI or
  instantiation bug: a `None` hinputs from a swallowed typing error produced exactly that shape before
  the harness surfaced it.
- Data crossing Rust→C++ goes through the metal **builder FFI**: one `add(key, value)` call per entry
  into a C++-owned `std::unordered_map`, the way `metal_package_builder_add_extern_function` and the whole
  metal AST already cross, not a flat `#[repr(C)]` array smuggled through `InteropInputs`. The interop
  layout/ABI maps ride the metal `Program` for this reason (S14); a first build that used a flat
  `struct_layouts` array plus `GlobalState` side-maps was torn out. A genuinely positional list (one
  function's ordered arg coercions) may cross as an array; only *maps* must cross per-entry.
- Trap (tooling): an lldb breakpoint with an `-o "expr …" -o "continue"` command crashes lldb on the
  interop test binary — the breakpoint fires but the `expr` aborts, eating the output, which reads as
  "never hit." Inspect backend state by running to the natural abort and printing via `-k "frame select N"`
  / `-k "expr …"`, and pass rustc's signals (`process handle SIGUSR1 -s false -n false`, same for `SIGUSR2`).
- To backtrace an interop codegen abort (a `toRef`/`LLVMVerifyModule` assert), run the interop test binary
  directly under lldb: `RUSTUP_TOOLCHAIN=rustc-fork lldb -b -k "thread backtrace" -k quit -o run -- \
  target/debug/deps/frontend_rust-<hash> <test_name> --test-threads=1`. Two gotchas: `RUSTUP_TOOLCHAIN=rustc-fork`
  is required — running the binary bare builds the fixture crate with stock rustc and dies on a red-herring
  `E0514` toolchain mismatch, not the real bug — and post-crash commands must use `-k` (batch `-o` steps are
  skipped once the process stops on the assert). There are two `frontend_rust-<hash>` binaries in
  `target/debug/deps/`: the interop one (built `--features rust_interop`) is the one to run.
  Do not conclude a function is uncalled from a silent breakpoint-command run.
- Valen operators (`==`, `+`, `>=`, …) are **library functions** (`src/builtins/resources/arith.vale`),
  not typing-pass builtins — a compilation that omits the builtins package has no `==` and fails with
  `CouldntFindFunctionToCallT name "=="`. `run_driven_rustc` adds `Source::builtins` +
  `PackageCoordinate::builtin` (as `pass_manager.rs` does) under its `compile_builtins` flag, which
  `run_wrapper` and the whole interop test corpus set, so operators resolve everywhere.
- `!=` is the generic `!=<T>` (`not(a == b)`), and it now lives in the **builtins** `logic.vale` (moved
  out of the stdlib), beside `not`, which its body calls. Do **not** put it in `arith.vale`: `arith` is
  loaded without `logic` in many builtin bundles (`builtin_source_for_arith = ["arith","implicit_clone"]`),
  so `not` is out of scope there and `arith.vale` itself stops compiling — it broke ~33 standalone tests.
- The `valenc-rs`/`valen` artifacts' baked rpath must cover **both** `<sysroot>/lib` **and** `rustc --print
  target-libdir`: the `rustc_private` dylibs (`librustc_driver-<hash>.dylib`) live in the target libdir,
  while `<sysroot>/lib` holds only a *different-hash* copy. `build.rs`'s `emit_rustc_private_rpath` bakes
  both; without the target-libdir rpath the standalone binary dies in dyld before `main` (tests hide this
  because cargo/nextest sets the library path at run time). So `run_build` sets **no** `DYLD_LIBRARY_PATH`
  when it spawns `valenc-rs` — the baked rpath resolves the dylib, and pointing dyld at `<sysroot>/lib`
  would load the wrong-hash `librustc_driver`.
- The full `valen build` pipeline e2e is automated but **gated separately** from the `--lib` suite, in
  `pipeline_e2e` (`src/typing/test/rust_interop/pipeline_e2e.rs`): it drives the `run_build` dark-box over a
  project staged in a `TempDir` (real cargo → the real `valenc-rs` wrapper → link → run). The real bins can't
  be built by `cargo test --lib`, and `CARGO_BIN_EXE`/a `tests/` target drags in `valec` (which can't link the
  interop lib), so the module **locates** the bins beside its own test exe (`current_exe()` → `<target>/debug`)
  and **soft-skips** when they're absent — a dedicated fire-commit command builds them (`--bin valenc-rs --bin
  valen`) then runs the module, and the plain interop `--lib` gate leaves the tests skipped. It covers a
  forward tracer (→7), `driver-check` (build+link), and the warm-rebuild suite that drove the incremental
  fix — cold→warm over one `TempDir` via the `cold_edit_warm_exits` helper (`BuildInputs.clear_incremental`
  gates the cache wipe): a no-edit rebuild (runs + reuses the cache), a forward call-swap, a reverse
  callback-body edit, and a nested callback-outbound swap.
- The design's "stub generation is typing-driven, not parse-driven" (§30) is about the **pass-2 library
  `lib.rs`** (exported declarations, which need typed facts). The **pass-1 crate root** — imports→`use` +
  `__vale_main` + marker, what `generate_stub_source` emits — is parse-driven and correct that way; do not
  "fix" it to run off `HinputsT`.
- The interop test harness now compiles the builtins in (`compile_builtins`, on for the whole driven +
  typecheck corpus), so operators resolve and paths gated on builtin bounds — e.g. the reachable-bounds
  gather in `check_defining_conclusions_and_resolve` — fire in-suite; `a_driven_case_resolves_a_builtin_operator`
  (`cases.rs`) guards that parity. It compiles only the builtins, **not the stdlib** (Next #1), so a shape
  that needs a stdlib collection (`Option`/`Vec`/`str`) still reproduces only through `valen build` /
  `run_wrapper` — copy the user's project and run `valen build`, or add a `drive_tests.rs`-style `run_wrapper`
  test, for those.
- A backend "missing extern" abort (`buildCallOrSideCall` in `externs.cpp`, `externFuncIter != end()`) on a
  Rust leaf is usually **upstream**, not a backend bug: the leaf's request resolved to `dep: None` (so no
  `FunctionExternI` was materialized), which the driven firing log shows as `<path> => ARGS-UNCONVERTIBLE` or
  `=> UNRESOLVED`. On the `valen build` path those firings are dropped, so a temporary `eprintln!` in
  `resolve_new_requests` / `lang_per_instance_mir` (AI-editable) surfaces them before the abort. A Vale
  struct passed as a Rust generic's type arg is `ARGS-UNCONVERTIBLE` when `resolve_local_type` can't find it —
  which is exactly when the stub generator hasn't projected that struct into the crate root.
- A generated `Cargo.toml` needs its own `[workspace]` table, or cargo refuses it whenever the build dir is
  nested inside an existing workspace (the common real-repo case): "believes it's in a workspace when it's
  not." A scratchpad build outside any workspace hides this — always test the pipeline from inside a real repo.
- **A warm-rebuild nondeterminism is PER-LINEAGE (per cold build), not per-warm-rebuild — test it with
  fresh cold builds, not repeated warm ones.** rustc bakes the CGU layout at *cold* build time and reuses it
  for every warm rebuild of that cache, so a given cache is deterministically good or bad; looping N warm
  rebuilds within one cache samples one layout and cannot catch it (a good lineage passes forever, a bad one
  panics on warm #1). Loop FRESH lineages (new `TempDir` → cold → one warm each) to sample cold-build
  layouts. This is why the exports-first fix (above) was needed and why "panics every warm rebuild" (one bad
  cache) and "~50%" (across cold builds) were the same bug. The second lesson: when the re-fire loop
  re-populates `monouts`, anything a callback READS (the typeid universe) must be WRITTEN by an export
  re-fired earlier in the same call — populate-then-read, seeded from the authoritative export list, not
  rustc's CGU order.
- **Incremental warm rebuilds are fixed — don't undo the two load-bearing pieces.** A warm `valen build`
  rebuild once linked an undefined `__vale_main`: rustc serves `items_of_instance` from disk and skips
  `per_instance_mir`, whose side effects are the emit's only inputs, so `vale_cgu` came out fresh-but-empty
  (never a stale object). The no-fork fix, now in: **(A)** `lang_collect_and_partition_mono_items` re-fires
  `tcx.per_instance_mir(instance)` for each `is_vale_codegen_target` item (safe — the query declares no
  disk-cache modifier and rustc memoizes it in memory ⇒ once per instance per build, so no double-fire), and **(B)**
  `generate_stub_source` bakes an FNV-1a digest of the `.valen` source into every
  `#[vale::emit_consumer_body]` attr (root *and* override), which `per_instance_mir` reads via
  `has_attrs_with_path`, so a body-only call-graph change flips `per_instance_mir` — hence
  `items_of_instance` — red and rustc re-collects. The attr-arg mechanism works (the planned `pub const`
  fallback was not needed). Remove neither: without A the warm emit is empty; without B a same-imports edit
  (calling a different already-imported fn, or swapping an outbound callback leaf) reuses a stale partition
  and rustc panics **"unstable fingerprints for `per_instance_mir`"** — the signature of an impure query
  whose untracked input (the Valen source, read via `DriverState`) changed. `valen/main.rs` now passes
  `clear_incremental = false`. Do **not** swap to `CARGO_INCREMENTAL=0` — its merged-CGU layout DCEs the
  reified leaves (`main_loop::<MyCb>`, `__vale_drop::<T>`), a *different* failure (Next #3). Guarded by the
  `pipeline_e2e` warm-rebuild tests.
- Emitting a boundary wrapper/thunk, make each `ret`/param LLVM type *equal* the declared signature type,
  not merely layout-match it. `LLVMVerifyModule` (the shared `finalizeCompile` tail, on for every test
  compile) rejects a named struct returned into an `{iN,iM}`-typed function or a value `ret` into a void
  function even when the bytes line up — bugs codegen silently tolerated until verify was enabled (a reverse
  `Pair`-return wrapper and a ZST-return wrapper both hid this way). When verify flags one, reinterpret the
  value through a slot to the exact declared type before returning.
- A zero-sized imported struct crosses an interop extern as `PassMode::Ignore`: nothing is on the ABI (the
  boundary signature emits no param / a void return), so a Vale-side value must be *synthesized*
  (`LLVMGetUndef` of its translated type), never received from a C param or returned as a value. Do not read
  an `Ignore` arg's absent C-param as a bug — advancing the param index past it is the bug.
- A fix is not proven until a test exercises it red→green — that a guard/patch was *removed* from the code
  (e.g. the `is_rust_backed` reachable-bounds guard) is not proof the underlying behavior is now correct.
  A `--lib` case may not even reach the fixed path (needs the right builtins/stdlib in scope), and the
  shape that does may abort earlier on an unrelated codegen gap; wire the real test and watch it fail first.
- `src/lib.rs` carries a crate-wide `#![allow(unused_variables, unused_imports)]` (and `dead_code`), so
  removing a symbol's last use leaves its `use` line silently dead — the compiler will not flag it. After a
  deletion (e.g. collapsing a duplicated struct), grep each freed import's remaining uses by hand and drop
  the orphans; a clean `cargo check` is not evidence the imports are all live.
