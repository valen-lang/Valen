# Rust Interop — handoff

The `rust_interop` feature (`src/typing/rust_interop/`, behind `--features rust_interop`) makes a Vale
program typecheck against real Rust items read from a live rustc `TyCtxt`, and — through the instantiator
inversion below — lets rustc's monomorphization collector drive Vale's instantiator all the way into the
C++ backend: the `fill_extra_modules` hook fires at codegen and emits the whole Vale program's IR into a
rustc-lent module, which rustc verifies. What does **not** run yet is a linked executable — the last tier
(a `bin` stub, the partition filter, and a Tier-2 link-and-run harness) is unbuilt; see "Running a
program". The design sources of truth are `docs/architecture/vale-rust-interop-architecture.md` and, for
the instantiator's role, `src/instantiating/rust-interop-design.md`; the forward work is
`docs/plans/rust-interop-plan.md`.

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
The design of record is `src/instantiating/rust-interop-design.md`; the as-built shape:

- **Recording Rust leaves.** A Rust callee reaches the backend as a synthesized `extern`: the typing
  pass wraps every Rust import in an extern function whose body is a single `ExternFunctionCall`. The
  instantiator records the *real* Rust boundary — the `ExternFunctionCall` node (in `translate_ref_expr`,
  `instantiator.rs`) whose `prototype2` is `is_rust_backed` — inserting its instantiated `PrototypeI`
  into `monouts.rust_instantiation_requests`. The wrapper around it is ordinary Vale that instantiates
  normally. (An earlier over-broad filter in `translate_prototype` intercepted the wrapper prototype
  itself; it was replaced by this node-level recording so the wrapper compiles.) Two `pub(crate)` seams
  (`instantiate_exported_function`, `drain_instantiation_queue`) were extracted from `translate_program`
  (behavior-preserving) so a driver can run one export.
- **The provider.** `src/instantiating/rust_interop/` holds the `per_instance_mir` query provider,
  installed via `override_queries` from a `Compilation::Continue` driver. rustc's collector calls it
  for each Vale stub item (a `#[vale::emit_consumer_body]` fn in a `__VALE_STUBS_MARKER` crate); it
  seeds that export, drains the instantiator, resolves each collected request to a rustc
  `(DefId, GenericArgs)`, and returns a synthetic MIR body — a `ReifyFnPointer` cast per Rust leaf
  (which is what queues them) plus `Unreachable`. The real Vale body is a separate backend concern
  (single-symbol swap, arch §5.2).
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
`run_case_rustc_driven[_full]` in `harness.rs` — green (the composed `domino` case additionally asserts
rustc exits 0 through codegen).

## Running a program

Target: a Vale binary `func main() int { return add_two_numbers(3, 4) }`, linked and run, asserting exit
7. Everything up to emission is built and green; the link-and-run tier is not.

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
- **The `fill_extra_modules` hook (emission).** `DrivenCallbacks::config` installs
  `set_fill_extra_modules_hook(consumer_fill_modules)` (`src/instantiating/rust_interop/`). At codegen the
  handler reads the armed `DriverState`; when its `emit_backend` flag is set it lowers the whole program
  via the ordinary `translate_program` → `populate_metal_cache` and calls `backend_compile_program_into`
  with rustc's lent `(context, module)` (safe wrapper `backend_compile_program_into_safe`,
  `src/backend_ffi/`). `rustc_codegen_fires_our_fill_extra_modules_hook` asserts it fires;
  `rustc_codegen_emits_vale_bodies_into_borrowed_module` asserts the C++ backend emits **and rustc
  verifies** Vale IR into rustc's module (backend rc 0, rustc exits 0). Both green. Still a `lib` crate —
  verified, not linked or run.
- **What's left for the literal assert-7 (Stage 3, unbuilt).**
  - A `--crate-type=bin` stub whose `fn main` forwards `__vale_main`'s exit code
    (`std::process::exit(unsafe { __vale_main() })`). The backend already emits `__vale_main` (external,
    no libc shim) when a Vale `main` export exists.
  - The `collect_and_partition_mono_items` partition filter + `deduced_param_attrs` overrides, so rustc
    emits no competing `__vale_main` placeholder to collide with the backend's real body.
  - A Tier-2 harness that drives rustc to a **linked executable** (today's harness stops at codegen), runs
    it, and asserts the process exits 7. `Expect::Returns(N)` (`corpus.rs`) already carries N for tier 2.

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

The **borrowed-mode entry** lives in `Backend/src/rust_interop/` (`backend_compile_program_into` →
`compileIntoModuleFromRustc`) — the one AI-editable corner of the otherwise-core `Backend/`. It takes
rustc's lent `LLVMContext` + `LLVMModule` as opaque `void*` (from `ModuleLlvm::llcx_raw_mut()` /
`llmod_raw()` on the fork — **not** the TargetMachine, which the fork never exposes), emits Vale IR into
that module, and returns. rustc owns optimization, object emission, and disposal, so this path does
**not** optimize, `generateOutput`, dispose the handles, or call `generateExports`. `GlobalState`
sources its data layout from the module (`LLVMGetModuleDataLayout`), which rustc pre-set. Shared with
the standalone path: `compileValeCode` emits the Vale functions and, when the program exports a `main`,
the region setup/cleanup + `__Vale_Main` wrapper, returning its prototype (or `nullptr` for a library
with no `main`); the caller then emits an entry via one parameterized `makeEntryFunction(name,
emitLibcShim)` — standalone emits libc `main` (with the argc/argv + wasi shim), interop emits
`__vale_main` (external, no shim; rustc's libstd owns `main`). The `fill_extra_modules` hook calls
`compileIntoModuleFromRustc` (see "Running a program").

## The interop-specific core touch-points (design + code)

Three edits in the core typing pass exist solely for interop. Each mirrors existing struct code and is
`#[cfg(feature = "rust_interop")]`-guarded, so a normal build is byte-identical to before:

- the `rust_method_entries` hook in `precompile_interface` (`struct_compiler.rs`) — attaches an enum's
  methods/drop, twin of the one in `precompile_struct`.
- the `RustImportSeed` match in `Compiler::evaluate` (`compiler.rs`) — seeds a struct **or** interface.
- the `is_rust_backed` skip in `compile_interface_core` (`struct_compiler_core.rs`) — keeps Rust methods
  out of the interface vtable, twin of the one in `compile_struct_core`.

The instantiator has one more, also `#[cfg]`-guarded: recording a Rust leaf at the `ExternFunctionCall`
node in `translate_ref_expr` (`src/instantiating/instantiator.rs`), plus the two `pub(crate)` seams it
exposes. See the inversion section. The lazy-postparse defer in `get_or_create_postparsed_function`
(`compiler.rs`) is `#[cfg]`-guarded too.

A few core changes this work required are **not** interop-gated, because they complete previously-stubbed
*general* paths and stand on their own (the default suite guards them): `InterfaceDefinitionT::
generic_param_types` (was `unimplemented!`; needed once an enum's drop is compiled), the extern-signature
check permitting extern **Interfaces** so an imported Rust enum is allowed in an extern's signature (see
the Lessons entry), and the deferred-compile drain deriving each function's outer env from its id in
`evaluate_generic_function_from_non_call` rather than from a threaded `FunctionTemplataT.outer_env` (which
now serves only the from-call/lambda paths).

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
- The borrowed backend emit does **not** consume the driven `monouts` accumulator. `translate_program`
  re-instantiates the whole program from `hinputs` into a finalized `HinputsI` (pure Vale bodies), which
  is exactly what `populate_metal_cache` wants. The rustc-driven `monouts` (an `InstantiatedOutputsI`,
  never a finalized `HinputsI`) exists only to resolve/reify the Rust leaves for rustc's collector.
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
- The fork's `ModuleLlvm` exposes only the borrowed `LLVMContext` + `LLVMModule` (`llcx_raw_mut()` /
  `llmod_raw()`), **never** the TargetMachine — by design. So borrowed-mode codegen sources its data
  layout from the module (`LLVMGetModuleDataLayout`; rustc pre-set it, do not re-set it) and MUST NOT
  dispose any lent handle — rustc owns their lifecycle and disposes them after the hook returns.
