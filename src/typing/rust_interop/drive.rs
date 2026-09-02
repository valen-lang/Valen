// The `valenc-rs` wrapper's engine: compile a Valen crate against caller-supplied rlibs by driving rustc
// with Vale's typing pass + instantiator + backend inside its callbacks.
//
// `run_wrapper` is the dark-box API (@DBAPIZ): cargo hands it the per-crate rustc argv, and its crate-root
// extension decides whether to drive Valen (a `.valen`) or pass through to plain rustc (any other root).
// `run_driven_rustc` is the shared driven-compile core; it reads no environment (the sysroot rides the
// argv). The body mirrors the `#[cfg(test)]` `drive_rustc` harness template; the two should later be
// unified onto this function (a noted follow-on), which is why the scoped-borrow reasoning is repeated.

use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;
use std::sync::Arc;

use bumpalo::Bump;
use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler as RustcCompiler;
use rustc_middle::ty::TyCtxt;

use crate::backend_ffi::metal_lowerer::ExternAbi;
use crate::code_source::{CodeSource, Source};
use crate::compile_options::GlobalOptions;
use crate::instantiating::ast::ast::FunctionExportI;
use crate::instantiating::instantiating_interner::InstantiatingInterner;
use crate::instantiating::instantiator::InstantiatedOutputsI;
use crate::instantiating::rust_interop::{
  arm_driver_state, consumer_fill_modules, vale_override_queries, CallbackReq, DriverState,
};
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::postparsing::ScoutCompilation;
use crate::scout_arena::ScoutArena;
use crate::typing::compiler::Compiler;
use crate::typing::hinputs_t::HinputsT;
use crate::typing::oracles::Oracles;
use crate::typing::rust_interop::stub_gen::generate_stub_source_from_vale;
use crate::typing::rust_interop::{LoggingOracle, TyCtxtOracle};
use crate::typing::typing_interner::TypingInterner;
use crate::typing::TypingPassOptions;
use crate::utils::code_hierarchy::{FileCoordinateMap, PackageCoordinate};

/// The rustc sysroot, from `rustc --print sysroot` (honoring `$RUSTC`). An env read, so it lives here
/// for `main()`/tests to call *above* the dark box — never from inside `run_wrapper`/`run_driven_rustc`.
pub fn default_sysroot() -> String {
  let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
  let out = process::Command::new(rustc)
    .arg("--print=sysroot")
    .output()
    .expect("could not run rustc to find the sysroot");
  String::from_utf8(out.stdout).expect("sysroot was not utf8").trim().to_string()
}

/// Everything `run_wrapper` needs, gathered by `main()` above the dark-box boundary (@DBAPIZ).
pub struct WrapperInputs {
  /// The rustc argv for `run_compiler`: argv[0] (the program name) kept, the rustc-path argv[1] cargo
  /// passes already stripped by `main()`, then cargo's flags and the crate-root input positional. For a
  /// `.valen` root the wrapper substitutes the generated stub `.rs` for that positional.
  pub rustc_args: Vec<String>,
}

/// The outcome of a wrapper invocation: rustc's exit code, whether the crate was a Valen crate (so the
/// Valen engine drove it) or a pure-Rust passthrough, and the provider's firing log (empty for a
/// passthrough).
pub struct WrapperResult {
  pub rustc_exit: i32,
  pub drove_valen: bool,
  pub firings: Vec<String>,
}

/// Empty callbacks: the pure-Rust passthrough installs no query overrides and no fill_extra_modules
/// hook, so a non-Valen crate compiles byte-identically to vanilla rustc (@PRCCBIVRZ).
struct NoopCallbacks;
impl Callbacks for NoopCallbacks {}

/// The `valenc-rs` wrapper's dark box (@DBAPIZ): cargo invokes `valenc-rs <rustc> <args…>` once per
/// crate, and `main()` strips the rustc path and hands the rest here. The crate-root file extension
/// decides the path (design §34): a `.valen` root drives the Valen engine (generate the pass-1 stub,
/// install the overrides, run rustc); any other root (a pure-Rust dependency) passes straight through
/// to rustc with no Valen machinery.
pub fn run_wrapper(inputs: &WrapperInputs) -> Result<WrapperResult, String> {
  let valen_input = inputs.rustc_args.iter().position(|arg| arg.ends_with(".valen"));
  match valen_input {
    Some(idx) => {
      let valen_path = &inputs.rustc_args[idx];
      let vale_source = fs::read_to_string(valen_path)
        .map_err(|e| format!("could not read the Valen crate root {valen_path}: {e}"))?;
      // Generate the pass-1 stub (imports → `use`, the marker, `__vale_<export>` roots) and point rustc
      // at it instead of the `.valen`, which rustc cannot parse; every other flag cargo supplied stays.
      let stub_src = generate_stub_source_from_vale(&vale_source)?;
      let stub_path = format!("{valen_path}.rs");
      fs::write(&stub_path, stub_src)
        .map_err(|e| format!("could not write the generated stub to {stub_path}: {e}"))?;
      let mut rustc_args = inputs.rustc_args.clone();
      rustc_args[idx] = stub_path;
      let (rustc_exit, firings) = run_driven_rustc(&rustc_args, &vale_source)?;
      Ok(WrapperResult { rustc_exit, drove_valen: true, firings })
    }
    None => {
      let mut callbacks = NoopCallbacks;
      let rustc_exit = rustc_driver::catch_with_exit_code(|| {
        rustc_driver::run_compiler(&inputs.rustc_args, &mut callbacks);
      });
      Ok(WrapperResult { rustc_exit, drove_valen: false, firings: Vec::new() })
    }
  }
}

/// Set up the instantiator state and drive rustc over `rustc_args` (which must point at a generated stub
/// `.rs`), with Vale's query overrides + the fill_extra_modules hook installed and `vale_source` typed
/// in `after_expansion`. Returns rustc's exit code and the provider's firing log. It runs no produced
/// binary — the caller decides that — and reads no environment (@DBAPIZ). Called by `run_wrapper` once
/// it has substituted the generated stub for the `.valen` crate root.
fn run_driven_rustc(rustc_args: &[String], vale_source: &str) -> Result<(i32, Vec<String>), String> {
  // The instantiator state, all in this frame so it outlives run_compiler (and thus every provider
  // call): four arenas, their interners, the Vale source, and the hinputs/monouts slots.
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let instantiating_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let typing_interner = TypingInterner::new(&typing_bump);
  let instantiating_interner = InstantiatingInterner::new(&instantiating_bump);

  let package_coord = parse_arena.intern_package_coordinate(parse_arena.intern_str("test"), &[]);
  let mut files = FileCoordinateMap::<String>::new();
  files.put(parse_arena.intern_file_coordinate(package_coord, "0.vale"), vale_source.to_string());
  // Compile the compiler builtins (arith/logic/… — where Valen's `==`/`+`/etc. are defined as library
  // functions) alongside the user program, exactly as standalone valec does (pass_manager.rs), so int
  // operators resolve. The builtin package coord is added to the typing scout in `after_expansion`.
  let code_source = CodeSource::new(vec![
    Source::builtins(&parse_arena, &parser_keywords),
    Source::from_code_map(&files),
  ]);

  let global_options = GlobalOptions {
    sanity_check: true,
    use_overload_index: true,
    use_optimized_solver: true,
    verbose_errors: true,
    debug_output: true,
  };

  let hinputs_slot: RefCell<Option<HinputsT>> = RefCell::new(None);
  let typing_error_slot: RefCell<Option<String>> = RefCell::new(None);
  let monouts_slot: RefCell<InstantiatedOutputsI> = RefCell::new(InstantiatedOutputsI::new());
  let function_exports_slot: RefCell<Vec<FunctionExportI>> = RefCell::new(Vec::new());
  let entry_symbol_slot: RefCell<Option<String>> = RefCell::new(None);
  let callbacks_slot: RefCell<Vec<CallbackReq>> = RefCell::new(Vec::new());
  let firings_slot: RefCell<Vec<String>> = RefCell::new(Vec::new());
  let extern_abis_slot: RefCell<HashMap<String, ExternAbi>> = RefCell::new(HashMap::new());
  let state = DriverState {
    opts: &global_options,
    interner: &instantiating_interner,
    typing_interner: &typing_interner,
    scout_arena: &scout_arena,
    keywords: &keywords,
    hinputs: &hinputs_slot,
    monouts: &monouts_slot,
    function_exports: &function_exports_slot,
    entry_symbol: &entry_symbol_slot,
    callbacks: &callbacks_slot,
    firings: &firings_slot,
    extern_abis: &extern_abis_slot,
    emit_backend: true,
  };
  let state_ptr = &state as *const DriverState as *const ();

  let mut callbacks = DrivenCallbacks {
    scout_arena: &scout_arena,
    keywords: &keywords,
    parser_keywords: &parser_keywords,
    parse_arena: &parse_arena,
    package_coord,
    code_source: &code_source,
    typing_interner: &typing_interner,
    global_options: global_options.clone(),
    hinputs_slot: &hinputs_slot,
    typing_error_slot: &typing_error_slot,
    state_ptr,
  };
  let rustc_exit = rustc_driver::catch_with_exit_code(|| {
    rustc_driver::run_compiler(rustc_args, &mut callbacks);
  });
  if let Some(err) = typing_error_slot.into_inner() {
    return Err(format!("the Vale program failed to typecheck, so no body was emitted:\n{err}"));
  }
  Ok((rustc_exit, firings_slot.into_inner()))
}

/// Callbacks for the driven path — the non-test twin of the harness's `DrivenCallbacks`. The
/// arenas/interners and the owned `HinputsT` live in `run_driven_rustc`'s frame; this only holds borrows,
/// runs the Vale typing pass in `after_expansion` (writing the owned `HinputsT` into the caller's slot),
/// and arms the scoped pointer so the `per_instance_mir` provider drives the instantiator during codegen.
struct DrivenCallbacks<'ctx, 's, 't, 'p> {
  scout_arena: &'ctx ScoutArena<'s>,
  keywords: &'ctx Keywords<'s>,
  parser_keywords: &'ctx Keywords<'p>,
  parse_arena: &'ctx ParseArena<'p>,
  package_coord: &'p PackageCoordinate<'p>,
  code_source: &'ctx CodeSource<'p>,
  typing_interner: &'ctx TypingInterner<'s, 't>,
  global_options: GlobalOptions,
  hinputs_slot: &'ctx RefCell<Option<HinputsT<'s, 't>>>,
  typing_error_slot: &'ctx RefCell<Option<String>>,
  state_ptr: *const (),
}

// SAFETY: identical to the harness's — `run_compiler` moves the callbacks onto a thread it spawns but
// *joins* that thread before returning, so the rustc thread has exclusive access to everything this
// borrows for exactly the window it uses them, and the calling thread touches none of it until
// `run_compiler` returns. The `*const ()` points into that same (the caller's) frame.
unsafe impl<'ctx, 's, 't, 'p> Send for DrivenCallbacks<'ctx, 's, 't, 'p> {}

impl<'ctx, 's, 't, 'p> Callbacks for DrivenCallbacks<'ctx, 's, 't, 'p> {
  fn config(&mut self, config: &mut rustc_interface::Config) {
    config.override_queries = Some(vale_override_queries);
    rustc_codegen_llvm::set_fill_extra_modules_hook(consumer_fill_modules);
  }

  fn after_expansion<'tcx>(&mut self, _compiler: &RustcCompiler, tcx: TyCtxt<'tcx>) -> Compilation {
    // The builtins live in the empty/root package (`("", [])`); include it alongside "test" so the
    // operator functions from `Source::builtins` are compiled and `==`/`+`/etc. resolve.
    let builtin_coord = PackageCoordinate::builtin(self.parse_arena, self.parser_keywords);
    let mut scout = ScoutCompilation::new(
      self.scout_arena,
      self.keywords,
      self.parser_keywords,
      self.parse_arena,
      vec![builtin_coord, self.package_coord],
      self.code_source,
      self.global_options.clone(),
    );

    let mut import_paths: Vec<String> = Vec::new();
    if let Ok(scoutput) = scout.get_scoutput() {
      for program in scoutput.file_coord_to_contents.values() {
        for imp in program.imports {
          if imp.module_name == self.keywords.rust {
            let mut segments: Vec<&str> = imp.package_names.iter().map(|s| s.0).collect();
            segments.push(imp.importee_name.0);
            import_paths.push(segments.join("."));
          }
        }
      }
    }
    let mut import_path_strs: Vec<&str> = Vec::new();
    for path in &import_paths {
      if !import_path_strs.contains(&path.as_str()) {
        import_path_strs.push(path.as_str());
      }
    }

    let real = TyCtxtOracle::new(tcx, self.scout_arena, &import_path_strs);
    let compiling = tcx.crate_name(rustc_span::def_id::LOCAL_CRATE).to_string();
    let logging = LoggingOracle::new(&real, &compiling);

    let options = TypingPassOptions {
      global_options: self.global_options.clone(),
      debug_out: Arc::new(|x: &str| println!("{}", x)),
      tree_shaking_enabled: true,
    };

    let code_map = scout.get_code_map().expect("getCodeMap failed");
    let astrouts = scout.expect_scoutput();
    let compiler = Compiler::new(
      self.scout_arena,
      &self.typing_interner,
      self.keywords,
      &options,
      Oracles::with_rust(&logging),
    );
    match compiler.evaluate(&code_map, astrouts) {
      Ok(hinputs) => *self.hinputs_slot.borrow_mut() = Some(hinputs),
      Err(err) => *self.typing_error_slot.borrow_mut() = Some(format!("{err:?}")),
    }

    arm_driver_state(self.state_ptr);
    Compilation::Continue
  }
}
