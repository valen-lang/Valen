# Rust Interop — handoff

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
small (≤8-byte) struct as a single register integer (`Cast`). Multi-register-split shapes (`Pair`, wider
`Cast`, float-HFA) are deferred with no NobiliaV consumer. See "The interim `valec-rs drive` bridge" for
compiling and running a Vale program against already-built Rust rlibs.
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
  `DrivenCallbacks` (drives codegen, returns `Compilation::Continue`,
  `src/typing/test/rust_interop/harness.rs`). Both implement only `config` and `after_expansion`.
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
  the stock backend (`harness.rs`), with no marker gating — fine for a single-crate driven test, but the
  pass-through gating is exactly what the real cargo build (many pure-Rust crates) will need.
- **Entry symbol capture.** The design shows a general capture — `record_export_symbol(export_name, symbol)`
  run for every exported function in `per_instance_mir`. The tree instead special-cases only the entry:
  `if stub_name == "__vale_main" { *state.entry_symbol.borrow_mut() = Some(tcx.symbol_name(instance)…) }`
  into a single `DriverState.entry_symbol` slot (`src/instantiating/rust_interop/mod.rs`), because the
  entry is the only inbound Rust→Valen crossing today. The general form — every export emitted under its
  rustc-mangled name, the inbound mirror of `FunctionExternI.link_name` — is unbuilt.

## State (regenerate, don't trust stale)

The repo pins the Vale rustc **fork** (`rust-toolchain.toml` → `rustc-fork`; the fork is
`github.com/Verdagon/rust` @ `per-instance-mir`, setup in `docs/build-compiler.md`). The fork builds
LLVM 21 **from source as one shared `libLLVM.dylib`** (its `config.toml`: `download-ci-llvm = false`,
`link-shared = true`) so the C++ backend and rustc can share a single libLLVM (arch §3.6/§5.7 — two
static libLLVMs in one process is duplicate-symbol UB). It carries the interop patches (the
`per_instance_mir` query + `fill_extra_modules`) and ships the `rustc_private` libraries + `rust-src`, so
interop needs **no** `rustup component add rustc-dev`; a standalone build compiles on it identically (the
patches are inert without a plugin). The `rustc-fork` toolchain is linked to the fork's **stage1**
sysroot, not stage2: a plain `./x build` populates stage1 with the `rustc-dev` component, but a
`./x build --stage 2` regenerates the stage2 sysroot *without* it (see the Lessons trap), so stage1 is
the complete one. Build/test from the repo root:

- default: `cargo test --manifest-path ./Cargo.toml --lib`
- interop: `cargo test --manifest-path ./Cargo.toml --lib --features rust_interop`

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
  the call site cannot drift from the declaration. Multi-piece `Cast` (`[N x i64]`), `Pair` (`ScalarPair`
  types), float-HFA, and on-stack byval (`Indirect { on_stack: true }`) still `panic!`; a leaf that hits
  one adds its own arm.

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

## The interim `valec-rs drive` bridge

A Vale program is compiled, linked, and run against **already-built** Rust rlibs by `valec-rs drive
prog.vale --extern <name>[=rlib] -L dependency=<dir>` — the interim manual bridge that unblocked NobiliaV
ahead of the permanent generated-Cargo-workspace + `RUSTC_WORKSPACE_WRAPPER` pipeline (arch §18/§20, mirror
`/Volumes/V/Harmonious/toylangc/src/build.rs`, still the destination). Confirmed end-to-end by NobiliaV: a
windowed driver (real window + wgpu device up), a headless `add_tile`→render→pick, and a full operator-driven
loop all run through it. The pieces are all AI-editable, in `src/typing/rust_interop/`:

- **`drive_and_link`** (`drive.rs`) — the dark-box (@DBAPIZ): scout → generate the stub → assemble the rustc
  argv → `run_compiler` with the query overrides installed → run the produced bin, forwarding its exit code.
  Reads no env (the sysroot is an input; `default_sysroot`/`run_drive` above it read env). The permanent
  pipeline reuses this (it forwards the same `--extern`/`-L` flags, from cargo instead of a human); only the
  manual front-end retires. It duplicates the `#[cfg(test)] drive_rustc` harness template — migrating the
  harness onto `drive_and_link` (and its fixtures onto `generate_stub_source`) is a follow-on.
- **`generate_stub_source`** (`stub_gen.rs`) — the real `vale-stub-gen` seed (arch §6.4, @RTMEIZ): from the
  **scouted** `ProgramS` (a `ScoutCompilation` run needs no rustc), emit one `pub use` per `import rust.X.Y`,
  a `#[vale::emit_consumer_body]` root per exported func, and the marker + `__vale_drop` shim. It errors on an
  exported Vale struct/trait — the `HinputsT`-driven emission the permanent form adds. Replaces the
  hand-written `fixtures/stub.rs` for the driven path.
- **The CLI** (`driver/main.rs`) — a thin clap `drive` subcommand over `run_drive`/`DriveArgs`. A bare
  `--extern <name>` (no `=rlib`) auto-resolves the hashed `lib<name>-<hash>.rlib` from the `-L dependency`
  dirs. One `--extern` suffices for a facade crate: `import rust.nobiliav.PieceId` resolves *through* a
  `pub use` to the canonical crate, whose mangled symbol links from `-L dependency=<deps>` alone.
- **Builtins compiled in** — `drive_and_link` adds `Source::builtins` + `PackageCoordinate::builtin` to the
  compilation (as `pass_manager.rs:422-428` does), so Valen operators resolve and their `__vbi_*` intrinsics
  lower through the interop backend. The **stdlib is not** included (a follow-on if a drive program ever needs
  stdlib collections/etc.).

The register-split ABI arms (`Pair`, multi-piece `Cast`, float-HFA) stay deferred with a *confirmed* absence
of NobiliaV consumer: their `TileSpec` (16 bytes) is `Indirect` (already handled), and `PieceId` (8 bytes)
is the single-integer `Cast` that is now done.

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
`compileIntoModuleFromRustc`. `Backend/src/rust_interop/rust_interop.cpp` — once the borrowed-mode entry,
now an empty placeholder kept for future interop-specific C++ — is the one AI-editable corner of the
otherwise-core `Backend/`. The borrowed mode takes rustc's lent `LLVMContext` + `LLVMModule` as opaque
`void*` (from `ModuleLlvm::llcx_raw_mut()` / `llmod_raw()` on the fork — **not** the TargetMachine, which
the fork never exposes), emits Vale IR into that module, and returns. rustc owns optimization, object emission, and disposal, so this path does
**not** optimize, `generateOutput`, dispose the handles, or call `generateExports`. `GlobalState`
sources its data layout from the module (`LLVMGetModuleDataLayout`), which rustc pre-set. Shared with
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
- the `RustImportSeed` match in `Compiler::evaluate` (`compiler.rs`) — seeds a struct **or** interface.
- the `is_rust_backed` skip in `compile_interface_core` (`struct_compiler_core.rs`) — keeps Rust methods
  out of the interface vtable, twin of the one in `compile_struct_core`.

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
`evaluate_generic_function_from_non_call`, and the single-symbol backend naming: `FunctionExternI` now
carries one `link_name` (the real callee symbol), `declareExternFunction` (`Backend/src/function/`) binds
it verbatim vs. composing the `vale_abi_` shim by a `packageCoordinate->projectName == "rust"` check, and
`makeEntryFunction` takes the entry symbol.

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
- TRAP: `./x build --stage 2` on the fork regenerates the stage2 sysroot **without** the `rustc-dev`
  component (the `rustc_private` rlibs), silently breaking interop with `can't find crate for
  rustc_driver` on the next recompile — a compile that had passed on cached artifacts. A plain
  `./x build` populates **stage1** with rustc-dev; link `rustc-fork` at stage1. Confirm a sysroot has
  `librustc_middle-*.rmeta` under `lib/rustlib/<target>/lib/` before trusting it carries rustc-dev.
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
  Do not conclude a function is uncalled from a silent breakpoint-command run.
- Valen operators (`==`, `+`, `>=`, …) are **library functions** (`src/builtins/resources/arith.vale`),
  not typing-pass builtins — a compilation that omits the builtins package has no `==` and fails with
  `CouldntFindFunctionToCallT name "=="`. `drive_and_link` adds `Source::builtins` +
  `PackageCoordinate::builtin` (as `pass_manager.rs` does) so operators resolve; a bare user-file-only
  compilation (the original harness/`drive` shape) cannot use them.
- `!=` is the generic `!=<T>` (`not(a == b)`), and it now lives in the **builtins** `logic.vale` (moved
  out of the stdlib), beside `not`, which its body calls. Do **not** put it in `arith.vale`: `arith` is
  loaded without `logic` in many builtin bundles (`builtin_source_for_arith = ["arith","implicit_clone"]`),
  so `not` is out of scope there and `arith.vale` itself stops compiling — it broke ~33 standalone tests.
- The `valec-rs` artifact's baked rpath must cover **both** `<sysroot>/lib` **and** `rustc --print
  target-libdir`: the `rustc_private` dylibs (`librustc_driver-<hash>.dylib`) live in the target libdir,
  while `<sysroot>/lib` holds only a *different-hash* copy. `build.rs`'s `emit_rustc_private_rpath` bakes
  both; without the target-libdir rpath the standalone binary dies in dyld before `main` (tests hide this
  because cargo/nextest sets the library path at run time).
