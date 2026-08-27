# Rust Interop

Living design doc for how the Valen compiler interops with Rust. This is the primary authority on the subject. This doc is the authoritative place for the design's intent. If there are any conflicts or inaccuracies, raised them to the architect.

## Design (human-only)

Valen has two modes: **standalone mode** and **interop mode**.

 * Standalone mode means that the compiler offers no Rust interop. It only compiles pure Valen code.
 * Interop mode means that Valen can work with the Rust ecosystem, can call Rust libraries, and Rust libraries can call Valen libraries.

### Valen Wraps Rustc Which Runs Valen

In interop mode, we use rustc as a framework; we let it drive the compilation pipeline, and it calls out to us whenever Valen things come up.

Specifically, `src/typing/rust_interop/driver/main.rs` has this line:
    rustc_driver::run_compiler(&rustc_args, &mut callbacks);
which runs rustc, and tells it the `callbacks` with which it should call into the Valen compiler.

Currently, we **statically link** rustc into valenc, because we use a patched rustc fork. One day, we might upstream those patches and we can then dynamically link against rustc nightly and retire the fork.

That `&mut callbacks` is a **ValenRustInteropCallbacks** (old names "LangCallbacks"/"DrivenCallbacks").

`ValenRustInteropCallbacks` has these three methods:

 * `config`, called shortly after rustc starts up, in which we install queries (explained below).
 * `after_expansion`, called before rustc's instantiation pass, to call valen's parsing+postparsing+typing passes.
 * `after_analysis`, called ????, caches typing pass outputs

`after_expansion` sets a thread_local `VALENC_RUSTC_INTEROP_STATE` struct (elsewhere still called "DRIVER_STATE") for:
 * `per_instance_mir` to read during instantiation
 * `consumer_fill_modules` to read during backend codegen


#### ValeRustInteropCallbacks::config installs the queries and the backend

Our config callback looks like this:
```rs
fn config(&mut self, config: &mut rustc_interface::Config) {
  config.override_queries = Some(vale_override_queries);
  rustc_codegen_llvm::set_fill_extra_modules_hook(consumer_fill_modules);
}
```

`vale_override_queries` overrides three rustc queries:
```rs
pub fn vale_override_queries(_session: &rustc_session::Session, providers: &mut Providers) {
  providers.queries.per_instance_mir = lang_per_instance_mir;
  let _ = DEFAULT_COLLECT_AND_PARTITION.set(providers.queries.collect_and_partition_mono_items);
  let _ = DEFAULT_DEDUCED_PARAM_ATTRS.set(providers.queries.deduced_param_attrs);
  providers.queries.collect_and_partition_mono_items = lang_collect_and_partition_mono_items;
  providers.queries.deduced_param_attrs = lang_deduced_param_attrs;
}
```
Specifically we install:
 * `per_instance_mir`, which will be called during instantiation.
 * `collect_and_partition_mono_items` which will trigger instantiating then removes valen functions from the list of things their backend should emit.
 * `deduced_param_attrs` which removes some function information that rustc incorrectly derives about our valen functions.

`consumer_fill_modules` overrides the backend:
```rs
pub fn consumer_fill_modules<'tcx>(tcx: TyCtxt<'tcx>, allocator: &ExtraModuleAllocator<ModuleLlvm>) {
  // Get a pointer into VALENC_RUSTC_INTEROP_STATE
  let state: &DriverState = ...
  ...
  // Call into Valen backend:
  let rc = emit_vale_into_borrowed_module(state, tcx, allocator);
  ...
}
```

#### ValenRustInteropCallbacks::after_expansion calls Valen's parsing, postparsing, typing passes

`after_expansion` is called after the rustc typing pass.

Our override looks roughly like this:
```rs
fn after_expansion<'tcx>(&mut self, _compiler: &RustcCompiler, tcx: TyCtxt<'tcx>) -> Compilation {
  // Parsing and postparsing here
  ...
  // Set up the Rust oracle
  let real = TyCtxtOracle::new(tcx, self.scout_arena, &import_path_strs);
  let logging = LoggingOracle::new(&real, &compiling);
  let oracle = Oracles::with_rust(&logging);
  // Set up Valen compiler
  let options: TypingPassOptions = ...
  let compiler = Compiler::new(self.scout_arena, &self.typing_interner, self.keywords, &options, oracle);
  // Run the Valen compiler
  match compiler.evaluate(&code_map, astrouts) ...
  // Set VALENC_RUSTC_INTEROP_STATE
  arm_driver_state(self.state_ptr);
  // Signal rustc to continue
  Compilation::Continue
}
```

Oracles is given to the typing pass, and here in interop mode, it has the `rust: &'ctx dyn RustOracle<'s, 't>` populated by the above `Oracles::with_rust` method.
```rs
pub struct Oracles<'ctx, 's, 't> where 's: 't {
  #[cfg(feature = "rust_interop")]
  pub rust: &'ctx dyn RustOracle<'s, 't>,

  ... // PhantomData here
}
```
Our main `RustOracle` impl is the `TyCtxtOracle`, which looks like this:
```rs
pub struct TyCtxtOracle<'tcx, 's> {
  tcx: TyCtxt<'tcx>,
}
```
and has various **read-only** methods to ask rustc about rust functions/structs/etc that the Valen code depends on. See src/typing/docs/typing-rust-interop-design.md for more.

#### ValenRustInteropCallbacks::after_analysis caches typing pass outputs

(TBD when we get there)

#### Query `collect_and_partition_mono_items` triggers instantiation pass

Our `collect_and_partition_mono_items` function pointer points at this function:
```rs
fn lang_collect_and_partition_mono_items<'tcx>(
  tcx: TyCtxt<'tcx>,
  key: (),
) -> MonoItemPartitions<'tcx> {
  // Get rust's default collect_and_partition_mono_items implementation...
  let upstream = DEFAULT_COLLECT_AND_PARTITION.get().expect(...);
  // ...and call it.
  let MonoItemPartitions { codegen_units: upstream_cgus, all_mono_items: reachable, .. } = upstream(tcx, key);
  // Populate the filtered_cgus with new CGUs that don't contain any valen items
  let mut filtered_cgus: Vec<CodegenUnit<'tcx>> = ...
  // Assemble new result.
  MonoItemPartitions { codegen_units: tcx.arena.alloc_from_iter(filtered_cgus), all_mono_items: reachable, }
}
```
Note that the `upstream(tcx, key)` is actually **triggering the entire instantiation pass** (which also calls `per_instance_mir` a bunch of times, btw).

#### Query `per_instance_mir` does instantiation for a Valen function

This runs during the instantiation pass. It mainly does three things:
 * Remember the mangled rust name of the valen main function. (???? why)
 * Call into the Valen instantiator for this export.
 * Build the MIR body of the fake function that we'll give back to rustc.

```rs
fn lang_per_instance_mir<'tcx>(
  tcx: TyCtxt<'tcx>,
  instance: Instance<'tcx>,
) -> Option<&'tcx Body<'tcx>> {
  let def_id = instance.def_id();
  // Skip anything that's not defined in Valen ???? (should we do this by saving the old per_instance_mir?)
  if !is_vale_codegen_target(tcx, def_id) {
    return None;
  }

  // e.g. "__vale_myExportedValenFunction"
  let stub_name = tcx.item_name(def_id).to_string();
  // e.g. "myExportedValenFunction"
  let export_name = stub_name.strip_prefix("__vale_").unwrap_or(&stub_name).to_string();

  let state: &DriverState = ... // Get ptr to VALENC_RUSTC_INTEROP_STATE
  // Remember the rustc-mangled name of Valen's main function, for backend.
  if stub_name == "__vale_main" {
    *state.entry_symbol.borrow_mut() = Some(tcx.symbol_name(instance).name.to_string());
  }

  // Call the Valen instantiator for this export, and gather the newly encountered rust things.
  let requests = state.collect_new_rust_requests(tcx, &export_name);
  let rust_deps = requests.iter().filter_map(|r| r.dep).collect();

  ... // Logging

  // Build the fake MIR body, mentioning the encountered rust things.
  let body = build_dependency_body(tcx, instance, &rust_deps);
  Some(tcx.arena.alloc(body))
}
```

That `__vale_main` stuff is temporary until we properly support Rust calling exported Valen functions.

See src/instantiating/docs/architecture/instantiating-rust-interop-design.md for more on this.

#### Query `deduced_param_attrs` removes incorrect valen function information


### Valen Compiler Runs Inside Cargo

## Design Proposals

S1. `Oracles.rust` is a non-optional `&dyn RustOracle` in interop builds, not an `Option`. The configurations with no real oracle — pure-Valen-semantics tests and the owned-mode standalone compilation — pass a panicking `RustOracle` fake, which is safe because none of them run `import rust` and so none ever consult it. Standalone (non-interop) builds keep no oracle field at all.

## Details

## Test cases

## Background

### Self-evident from the code

 * Nothing makes rustc discover Valen; Valen drives rustc. Our binary statically links rustc (`#![feature(rustc_private)]` + `extern crate rustc_driver`, `src/typing/rust_interop/driver/main.rs`) and calls `rustc_driver::run_compiler(&args, &mut callbacks)` (`harness.rs:727`, `driver/main.rs:185`) inside `catch_with_exit_code`. rustc then calls the `Callbacks` methods (`config` first) on the struct we passed it.
 * The `fill_extra_modules` hook is the one channel not on the `Callbacks` contract: a fork-patched process-global set via `rustc_codegen_llvm::set_fill_extra_modules_hook` (`harness.rs:526`).
 * rustc calls into Valen through six points today. `DrivenCallbacks::config` (`src/typing/test/rust_interop/harness.rs`) installs them: `override_queries = vale_override_queries` plus `set_fill_extra_modules_hook(consumer_fill_modules)`.
 * `vale_override_queries` (`src/instantiating/rust_interop/mod.rs`) overrides three rustc queries: `per_instance_mir` (drives our instantiator), `collect_and_partition_mono_items` (strips our `#[vale::emit_consumer_body]` stub bodies before LLVM codegen), and `deduced_param_attrs` (returns `&[]` for those stubs).
 * `consumer_fill_modules` (`src/instantiating/rust_interop/mod.rs`) is the `fill_extra_modules` fork-patch hook, called before rustc's `start_async_codegen`; this is where our backend emits Vale bodies into rustc's module.
 * Two callbacks structs exist for two jobs: `ValeCallbacks` (`src/typing/rust_interop/driver/main.rs`) runs typing only and returns `Compilation::Stop`; `DrivenCallbacks` (`src/typing/test/rust_interop/harness.rs`) drives codegen and returns `Compilation::Continue`. `ValenRustInteropCallbacks` is the intended unified name for both.
 * The opposite direction (Valen reads facts from rustc, the oracle seam) is a larger set of ordinary query reads, not overrides: `tcx.layout_of` + `tcx.fn_abi_of_instance` for aggregate ABI (`compute_struct_layouts` / `compute_extern_abi` in `src/instantiating/rust_interop/mod.rs`), `tcx.symbol_name` for single-symbol naming, and `module_children` / `fn_sig` for imports.

### Documented

### Undocumented

## Open Questions

## Required Reading

 * design-assistant
