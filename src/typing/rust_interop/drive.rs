// The `valenc-rs` wrapper's engine: compile a Valen crate against caller-supplied rlibs by driving rustc
// with Vale's typing pass + instantiator + backend inside its callbacks.
//
// `run_wrapper` is the dark-box API (@DBAPIZ): cargo hands it the per-crate rustc argv, and its crate-root
// extension decides whether to drive Valen (a `.valen`) or pass through to plain rustc (any other root).
// `run_driven_rustc` is the one driven-compile engine; it reads no environment (the sysroot rides the
// argv). The `#[cfg(test)]` test harness `drive_rustc` (`harness.rs`) delegates to it rather than
// re-implementing it, and its typecheck-only `CaseCallbacks` shares the scout→oracle preamble via the
// helpers below (`driven_code_source` / `scout_package_coords` / `collect_rust_import_paths`).

use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;
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
  OpaqueKindI,
};
use crate::utils::fx::IndexMap;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::postparsing::ScoutCompilation;
use crate::scout_arena::ScoutArena;
use crate::typing::compiler::Compiler;
use crate::typing::hinputs_t::HinputsT;
use crate::typing::oracles::Oracles;
use crate::typing::rust_interop::stub_gen::{
  generate_pass2_stub, generate_stub_source_from_vale, source_digest, VALE_OPAQUE_DECL,
};
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

/// The `VALEN_BORROW_CHECK` contract: absent or `"1"` runs the borrow checker, `"0"` skips it, and
/// anything else is an error rather than a silent default. Pure — it takes the variable's value, not
/// the environment — so the env read stays in `main()` above the dark box (@DBAPIZ) and the contract
/// is unit-testable. `valen build` sets the variable both ways explicitly, so a value leaked into the
/// user's shell cannot override the flag it was given.
pub fn parse_borrow_check_env(value: Option<&str>) -> Result<bool, String> {
  match value {
    None => Ok(true),
    Some("1") => Ok(true),
    Some("0") => Ok(false),
    Some(other) => {
      Err(format!("VALEN_BORROW_CHECK must be \"1\" (on) or \"0\" (off), got {other:?}"))
    }
  }
}

/// Everything `run_wrapper` needs, gathered by `main()` above the dark-box boundary (@DBAPIZ).
pub struct WrapperInputs {
  /// The rustc argv for `run_compiler`: argv[0] (the program name) kept, the rustc-path argv[1] cargo
  /// passes already stripped by `main()`, then cargo's flags and the crate-root input positional. For a
  /// `.valen` root the wrapper substitutes the generated stub `.rs` for that positional.
  pub rustc_args: Vec<String>,
  /// Whether the group borrow checker runs on the driven Valen program. `main()` reads it from
  /// `VALEN_BORROW_CHECK` (see `parse_borrow_check_env`) above the dark box; `valen build
  /// --no-borrow-check` is what sets that env, so a user can see whether interop alone builds and runs
  /// a program the checker does not yet accept.
  pub borrow_check: bool,
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

// --- Shared driven/typecheck preamble ---------------------------------------------------------------
// The three interop rustc-callback paths (this file's `DrivenCallbacks`, and the test harness's
// `DrivenCallbacks` + `CaseCallbacks`) build the same scout → oracle → typing preamble. These helpers
// are the one home for the pieces that were copied across all three; each callback keeps only its own
// tail (drive codegen, or stop and assert). `compile_builtins` threads through so operators
// (`==`/`+`/…, library functions in the builtins) resolve on the paths that want them.

/// The `CodeSource` for a compile: the user files, optionally preceded by the compiler builtins
/// (arith/logic/…) so Valen's operator library functions resolve. `Source::from_code_map` copies the
/// map, so the result is tied to the arena, not to `files`.
pub(crate) fn driven_code_source<'a, 'ctx>(
  parse_arena: &'ctx ParseArena<'a>,
  parser_keywords: &'ctx Keywords<'a>,
  files: &FileCoordinateMap<'a, String>,
  compile_builtins: bool,
) -> CodeSource<'a>
where
  'a: 'ctx,
{
  let mut sources = Vec::new();
  if compile_builtins {
    sources.push(Source::builtins(parse_arena, parser_keywords));
  }
  sources.push(Source::from_code_map(files));
  CodeSource::new(sources)
}

/// The package coordinates to scout/typecheck: the program's own package, preceded by the builtin
/// package coordinate (`("", [])`) when `compile_builtins` is set so `Source::builtins`' functions get
/// compiled.
pub(crate) fn scout_package_coords<'a, 'ctx>(
  package_coord: &'a PackageCoordinate<'a>,
  parse_arena: &'ctx ParseArena<'a>,
  parser_keywords: &'ctx Keywords<'a>,
  compile_builtins: bool,
) -> Vec<&'a PackageCoordinate<'a>>
where
  'a: 'ctx,
{
  let mut coords = Vec::new();
  if compile_builtins {
    coords.push(PackageCoordinate::builtin(parse_arena, parser_keywords));
  }
  coords.push(package_coord);
  coords
}

/// Scout the program and collect the deduped dotted paths of its `import rust.X.Y` statements — the
/// oracle's importable allowlist. `get_scoutput` memoizes, so a caller that reuses the same scout
/// afterward (for its code map + AST) pays nothing for this read.
pub(crate) fn collect_rust_import_paths<'s, 'ctx, 'p>(
  scout: &mut ScoutCompilation<'s, 'ctx, 'p>,
  keywords: &Keywords<'s>,
) -> Vec<String> {
  let mut paths: Vec<String> = Vec::new();
  if let Ok(scoutput) = scout.get_scoutput() {
    for program in scoutput.file_coord_to_contents.values() {
      for imp in program.imports {
        if imp.module_name == keywords.rust {
          let mut segments: Vec<&str> = imp.package_names.iter().map(|s| s.0).collect();
          segments.push(imp.importee_name.0);
          let joined = segments.join(".");
          if !paths.contains(&joined) {
            paths.push(joined);
          }
        }
      }
    }
  }
  paths
}

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
      let (rustc_exit, firings) = run_driven_rustc(
        &rustc_args,
        &vale_source,
        /*emit_backend=*/ true,
        /*compile_builtins=*/ true,
        inputs.borrow_check,
      )?;
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
/// binary — the caller decides that — and reads no environment (@DBAPIZ). `emit_backend` chooses whether
/// the fill_extra_modules hook lowers the Vale bodies (off for a firing-log-only run); `compile_builtins`
/// chooses whether the operator library functions are compiled in; `borrow_check` whether the group
/// borrow checker runs (`main()` reads it from the environment, so this stays environment-free). Called
/// by `run_wrapper` (backend and builtins on, the checker as `WrapperInputs` says) and by the test
/// harness's `drive_rustc` (which threads its own).
pub(crate) fn run_driven_rustc(
  rustc_args: &[String],
  vale_source: &str,
  emit_backend: bool,
  compile_builtins: bool,
  borrow_check: bool,
) -> Result<(i32, Vec<String>), String> {
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
  // The builtins (where Valen's `==`/`+`/etc. live as library functions) are compiled alongside the user
  // program when `compile_builtins` is set, exactly as standalone valec does (pass_manager.rs), so
  // operators resolve; the matching builtin package coord is added to the scout in `after_expansion`.
  let code_source = driven_code_source(&parse_arena, &parser_keywords, &files, compile_builtins);

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
  let opaque_universe_slot: RefCell<IndexMap<u64, OpaqueKindI>> = RefCell::new(IndexMap::default());
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
    opaque_universe: &opaque_universe_slot,
    emit_backend,
  };
  let state_ptr = &state as *const DriverState as *const ();

  // The pass-1 stub's `emit_consumer_body` attrs carry this digest so rustc's incremental cache tracks
  // the `.valen` body; the appended pass-2 decls must carry the same one.
  let src_digest = source_digest(vale_source);
  // Pass-1 fills this with the anon-substruct projection to compile in pass 2, or leaves it None (the
  // one-pass common case).
  let pass2_stub_slot: RefCell<Option<String>> = RefCell::new(None);

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
    compile_builtins,
    borrow_check,
    emit_pass2_stub_into: &pass2_stub_slot,
    src_digest,
    skip_typing: false,
  };
  let pass1_exit = rustc_driver::catch_with_exit_code(|| {
    rustc_driver::run_compiler(rustc_args, &mut callbacks);
  });
  if let Some(err) = typing_error_slot.borrow_mut().take() {
    return Err(format!("the Vale program failed to typecheck, so no body was emitted:\n{err}"));
  }

  let Some(pass2_decls) = pass2_stub_slot.borrow_mut().take() else {
    // One pass: no anon substructs implementing imported traits, so pass 1 already codegened.
    return Ok((pass1_exit, firings_slot.into_inner()));
  };

  // Two passes (design S2): pass 1 typed and returned Stop before codegen, so every codegen slot must
  // still be pristine — instantiation runs only in pass 2. Assert it so a future change that leaks
  // pass-1 codegen fails loud instead of silently double-processing (belt-and-suspenders).
  debug_assert!(firings_slot.borrow().is_empty(), "pass 1 must not fire the provider before pass 2");
  debug_assert!(callbacks_slot.borrow().is_empty(), "pass 1 must not collect callbacks");
  debug_assert!(function_exports_slot.borrow().is_empty(), "pass 1 must not record exports");
  debug_assert!(entry_symbol_slot.borrow().is_none(), "pass 1 must not record the entry symbol");

  // Append the anon-substruct projection to the pass-1 stub and point pass 2 at the appended file. The
  // pass-1 stub (imports + marker + `__vale_main` + hand-written projections) stays verbatim, so
  // `__vale_main` still drives instantiation. `__ValeOpaque` is predeclared ONLY when the pass-1 stub
  // doesn't already carry it: `run_wrapper`'s generated stub does (so we don't duplicate it), but a
  // hand-written harness fixture stub does not, and the appended `<I>__anon` wrapper references it.
  let stub_path = rustc_args.iter().find(|a| a.ends_with(".rs")).ok_or_else(|| {
    "two-pass: no .rs crate root in rustc_args to append the pass-2 stub to".to_string()
  })?;
  let pass1_text = fs::read_to_string(stub_path)
    .map_err(|e| format!("two-pass: could not read the pass-1 stub {stub_path}: {e}"))?;
  let opaque_predecl = if pass1_text.contains("struct __ValeOpaque") {
    ""
  } else {
    VALE_OPAQUE_DECL
  };
  // Write the appended stub into `--out-dir` (a scratch dir), NOT next to the crate root — for the test
  // harness the crate root is a checked-in fixture whose directory must not be polluted. Fall back to
  // the crate root's own path only if no `--out-dir` was given.
  let out_dir = rustc_args.iter().find_map(|a| a.strip_prefix("--out-dir="));
  let stub_basename =
    Path::new(stub_path).file_name().and_then(|n| n.to_str()).unwrap_or("crate_root.rs");
  let pass2_path = match out_dir {
    Some(dir) => format!("{dir}/{stub_basename}.pass2.rs"),
    None => format!("{stub_path}.pass2.rs"),
  };
  // Append (never prepend): the pass-1 text must stay at the crate root top so its inner attributes
  // (`#![register_tool(vale)]`, which makes `#[vale::emit_consumer_body]` resolve) remain the first
  // tokens. The `__ValeOpaque` predecl and anon decls go after it, the predecl before its first use.
  fs::write(&pass2_path, format!("{pass1_text}\n{opaque_predecl}{pass2_decls}"))
    .map_err(|e| format!("two-pass: could not write the pass-2 stub {pass2_path}: {e}"))?;
  let pass2_args: Vec<String> = rustc_args
    .iter()
    .map(|a| if a == stub_path { pass2_path.clone() } else { a.clone() })
    .collect();

  let mut callbacks2 = DrivenCallbacks {
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
    compile_builtins,
    borrow_check,
    emit_pass2_stub_into: &pass2_stub_slot,
    src_digest,
    skip_typing: true,
  };
  let pass2_exit = rustc_driver::catch_with_exit_code(|| {
    rustc_driver::run_compiler(&pass2_args, &mut callbacks2);
  });
  if let Some(err) = typing_error_slot.into_inner() {
    return Err(format!("pass 2 of the driven compile failed:\n{err}"));
  }
  Ok((pass2_exit, firings_slot.into_inner()))
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
  compile_builtins: bool,
  borrow_check: bool,
  // Two-pass driver (design S2 / arch §30). In PASS 1 (`skip_typing == false`) `after_expansion` types
  // the `.valen` and then, if the typed program contains anon substructs implementing imported traits,
  // writes their HinputsT-driven Rust projection into `emit_pass2_stub_into` and returns Stop (no
  // codegen — pass 2 does it). If there are none, it codegens in this one pass exactly as before. In
  // PASS 2 (`skip_typing == true`) it reuses pass 1's `HinputsT` (already in `hinputs_slot`), skips
  // typing, and codegens the appended stub. `src_digest` is baked into the pass-2 stub's
  // `emit_consumer_body` attrs, matching the pass-1 stub so rustc's incremental cache stays consistent.
  emit_pass2_stub_into: &'ctx RefCell<Option<String>>,
  src_digest: u64,
  skip_typing: bool,
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
    // PASS 2: pass 1 already typed this program (its `HinputsT` is in `hinputs_slot`); do not re-type.
    // Just arm the driver state so the `per_instance_mir` provider drives instantiation + codegen of
    // the appended stub under this second `tcx` (the reused `HinputsT` is tcx-free, so this is sound).
    if self.skip_typing {
      arm_driver_state(self.state_ptr);
      return Compilation::Continue;
    }

    let mut scout = ScoutCompilation::new(
      self.scout_arena,
      self.keywords,
      self.parser_keywords,
      self.parse_arena,
      scout_package_coords(
        self.package_coord,
        self.parse_arena,
        self.parser_keywords,
        self.compile_builtins,
      ),
      self.code_source,
      self.global_options.clone(),
    );

    let import_paths = collect_rust_import_paths(&mut scout, self.keywords);
    let import_path_strs: Vec<&str> = import_paths.iter().map(|s| s.as_str()).collect();

    let real = TyCtxtOracle::new(tcx, self.scout_arena, &import_path_strs);
    let compiling = tcx.crate_name(rustc_span::def_id::LOCAL_CRATE).to_string();
    let logging = LoggingOracle::new(&real, &compiling);

    let options = TypingPassOptions {
      global_options: self.global_options.clone(),
      debug_out: Arc::new(|x: &str| println!("{}", x)),
      tree_shaking_enabled: true,
      borrow_checker_enabled: self.borrow_check,
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
    // Retain the postparsed `coutputs` as a frame-local: the pass-2 stub generator (called just below,
    // still in pass 1) reads the abstract methods' `mut(g)` effects off it to render `&mut`. Pass 2
    // reuses only `hinputs` (from the slot) to codegen the already-generated stub text, so `coutputs`
    // need not outlive this call.
    let coutputs = match compiler.evaluate(&code_map, astrouts) {
      Ok((hinputs, coutputs)) => {
        *self.hinputs_slot.borrow_mut() = Some(hinputs);
        coutputs
      }
      Err(err) => {
        *self.typing_error_slot.borrow_mut() = Some(format!("{err:?}"));
        // Typing failed — no body to emit; `run_driven_rustc` surfaces the error. Stop before codegen.
        return Compilation::Stop;
      }
    };

    // If typing produced anon substructs implementing imported traits, their Rust projection can only
    // be emitted now (they exist only post-typing) — so build the appended pass-2 stub and Stop BEFORE
    // arming/codegen; `run_driven_rustc` then runs pass 2 over the appended stub. An empty projection
    // (the common case: no such substructs) falls through to the ordinary one-pass codegen below, so
    // non-feature crates are unchanged.
    let pass2 = {
      let borrowed = self.hinputs_slot.borrow();
      let hinputs = borrowed.as_ref().expect("hinputs set on the Ok branch above");
      match generate_pass2_stub(hinputs, &coutputs, self.typing_interner, self.src_digest) {
        Ok(stub) => stub,
        Err(e) => {
          *self.typing_error_slot.borrow_mut() =
            Some(format!("pass-2 stub generation failed: {e}"));
          return Compilation::Stop;
        }
      }
    };
    if !pass2.is_empty() {
      *self.emit_pass2_stub_into.borrow_mut() = Some(pass2);
      return Compilation::Stop;
    }

    arm_driver_state(self.state_ptr);
    Compilation::Continue
  }
}

#[cfg(test)]
mod tests {
  use super::parse_borrow_check_env;

  // Absent → on: a cargo build that runs `valenc-rs` with no env set still borrow-checks. This is
  // what keeps a build that spawns cargo without `run_build` (the release e2e) checking.
  #[test]
  fn borrow_check_env_defaults_on_when_absent() {
    assert_eq!(parse_borrow_check_env(None), Ok(true));
  }

  // The two values `valen build` sets — explicitly both ways, so a leaked env cannot override it.
  #[test]
  fn borrow_check_env_reads_one_as_on_and_zero_as_off() {
    assert_eq!(parse_borrow_check_env(Some("1")), Ok(true));
    assert_eq!(parse_borrow_check_env(Some("0")), Ok(false));
  }

  // Anything else is loud, not a silent default: a typo'd `VALEN_BORROW_CHECK=false` must not
  // quietly leave the checker on, nor quietly turn it off.
  #[test]
  fn borrow_check_env_rejects_any_other_value() {
    match parse_borrow_check_env(Some("false")) {
      Err(msg) => assert!(msg.contains("VALEN_BORROW_CHECK"), "msg: {msg}"),
      other => panic!("expected a loud error, got {other:?}"),
    }
  }
}
