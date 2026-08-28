// The interim `valec-rs drive` bridge's engine: drive a Valen program to a linked, runnable binary
// against caller-supplied, already-built rlibs (which Pearl produces with `cargo +rustc-fork build`).
//
// `drive_and_link` is the dark-box API (@DBAPIZ): structured inputs in, a structured `DriveResult` out,
// and it reads NO environment — the sysroot is passed in, gathered by `main()` above it. It is the
// permanent piece: the real cargo-workspace pipeline forwards the same `--extern`/`-L dependency` flags,
// just from cargo instead of a human, so only the manual front-end (the `drive` CLI + hand-run cargo)
// retires. The body mirrors the `#[cfg(test)]` `drive_rustc` harness template; the two should later be
// unified onto this function (a noted follow-on), which is why the scoped-borrow reasoning is repeated.

use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::sync::Arc;

use bumpalo::Bump;
use clap::Parser;
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
  arm_driver_state, consumer_fill_modules, vale_override_queries, DriverState,
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

/// One `--extern <name>[=<rlib>]`. `rlib: None` means the bare form — resolve the hashed
/// `lib<name>-<hash>.rlib` from the `-L dependency` dirs (added in a later slice).
pub struct ExternArg {
  pub name: String,
  pub rlib: Option<PathBuf>,
}

/// Everything `drive_and_link` needs, gathered by `main()` above the dark-box boundary (@DBAPIZ). No
/// field is read from the environment inside the function — `sysroot` in particular is an input.
pub struct DriveInputs {
  /// The Valen program text.
  pub vale_source: String,
  /// The directly-imported crates (`import rust.<name>...`), each an `--extern`.
  pub externs: Vec<ExternArg>,
  /// Cargo's `target/debug/deps/` dirs, each a `-L dependency=<dir>`; rustc resolves the transitive
  /// graph from here.
  pub dependency_dirs: Vec<PathBuf>,
  /// The rustc sysroot (`rustc --print sysroot`), gathered by the caller.
  pub sysroot: String,
  /// Scratch dir for the generated stub and rustc's `--out-dir` (and where the bin lands).
  pub out_dir: PathBuf,
}

/// The outcome of a drive: rustc's exit code (0 = drove through codegen), the produced binary's exit
/// code (`None` if rustc never linked one), and the provider's per-item firing log.
pub struct DriveResult {
  pub rustc_exit: i32,
  pub process_exit: Option<i32>,
  pub firings: Vec<String>,
}

/// The rustc sysroot, from `rustc --print sysroot` (honoring `$RUSTC`). An env read, so it lives here
/// for `main()`/tests to call *above* `drive_and_link` — never from inside the dark box.
pub fn default_sysroot() -> String {
  let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
  let out = process::Command::new(rustc)
    .arg("--print=sysroot")
    .output()
    .expect("could not run rustc to find the sysroot");
  String::from_utf8(out.stdout).expect("sysroot was not utf8").trim().to_string()
}

/// `valec-rs drive` arguments. Parsed by clap in `main()`, then handed to `run_drive` — the dark-box
/// boundary (@DBAPIZ), so `main()` does only argv parsing above it, and tests drive through here.
#[derive(Parser, Debug)]
pub struct DriveArgs {
  /// The Valen program to compile, link, and run.
  pub program: PathBuf,
  /// A directly-imported crate: `--extern <name>` (bare — resolved from the -L dirs by crate name) or
  /// `--extern <name>=<rlib>`. Repeatable, one per crate the program `import rust.<name>...`s.
  #[arg(long = "extern", value_name = "NAME[=RLIB]")]
  pub externs: Vec<String>,
  /// A library search path, `-L [KIND=]<dir>` (e.g. cargo's `-L dependency=target/debug/deps`).
  /// Repeatable; rustc resolves the transitive graph from here.
  #[arg(short = 'L', value_name = "[KIND=]DIR")]
  pub library_paths: Vec<String>,
  /// Scratch dir for the generated stub and the produced binary (default: a fresh temp dir).
  #[arg(long = "out-dir", value_name = "DIR")]
  pub out_dir: Option<PathBuf>,
}

/// Run the `drive` subcommand from parsed args: read the program, resolve inputs, drive it, print any
/// firings/errors, and return the produced binary's exit code (or 1 on a setup/typing error). `main()`
/// calls `std::process::exit` with the returned code.
pub fn run_drive(args: &DriveArgs) -> i32 {
  let vale_source = match fs::read_to_string(&args.program) {
    Ok(source) => source,
    Err(e) => {
      eprintln!("valec-rs: could not read {}: {e}", args.program.display());
      return 1;
    }
  };
  let externs = args.externs.iter().map(|spec| parse_extern(spec)).collect();
  let dependency_dirs = args.library_paths.iter().map(|spec| parse_library_path(spec)).collect();
  let out_dir = match &args.out_dir {
    Some(dir) => dir.clone(),
    None => default_out_dir(),
  };
  if let Err(e) = fs::create_dir_all(&out_dir) {
    eprintln!("valec-rs: could not create out dir {}: {e}", out_dir.display());
    return 1;
  }
  let inputs = DriveInputs {
    vale_source,
    externs,
    dependency_dirs,
    sysroot: default_sysroot(),
    out_dir,
  };
  match drive_and_link(&inputs) {
    Ok(result) => {
      for firing in &result.firings {
        eprintln!("{firing}");
      }
      match result.process_exit {
        Some(code) => code,
        None => {
          eprintln!(
            "valec-rs: rustc did not produce a runnable binary (rustc exit {})",
            result.rustc_exit
          );
          if result.rustc_exit == 0 {
            1
          } else {
            result.rustc_exit
          }
        }
      }
    }
    Err(e) => {
      eprintln!("valec-rs: {e}");
      1
    }
  }
}

/// Parse one `--extern` value: `name=rlib` (explicit path) or a bare `name` (resolved from -L dirs).
fn parse_extern(spec: &str) -> ExternArg {
  match spec.split_once('=') {
    Some((name, rlib)) => ExternArg { name: name.to_string(), rlib: Some(PathBuf::from(rlib)) },
    None => ExternArg { name: spec.to_string(), rlib: None },
  }
}

/// Parse one `-L` value. rustc's form is `[KIND=]PATH`; the bridge uses these only as dependency dirs,
/// so a leading `KIND=` (e.g. `dependency=`) is dropped and the path kept.
fn parse_library_path(spec: &str) -> PathBuf {
  match spec.split_once('=') {
    Some((_kind, path)) => PathBuf::from(path),
    None => PathBuf::from(spec),
  }
}

/// A default scratch dir under the system temp dir, unique to this process.
fn default_out_dir() -> PathBuf {
  env::temp_dir().join(format!("valec-rs-drive-{}", process::id()))
}

/// Generate a stub from `inputs.vale_source`, compile it to a `--crate-type=bin` against the supplied
/// rlibs with Vale's query overrides installed, run the produced binary, and report its exit code.
pub fn drive_and_link(inputs: &DriveInputs) -> Result<DriveResult, String> {
  let stub_src = generate_stub_source_from_vale(&inputs.vale_source)?;
  let stub_path = inputs.out_dir.join("stub.rs");
  fs::write(&stub_path, stub_src)
    .map_err(|e| format!("could not write the generated stub to {}: {e}", stub_path.display()))?;

  let mut rustc_args: Vec<String> = vec![
    "valec-rs".to_string(),
    stub_path.display().to_string(),
    "--crate-type=bin".to_string(),
    "--crate-name=stub".to_string(),
    "--edition=2021".to_string(),
    format!("--sysroot={}", inputs.sysroot),
    format!("-L{}", inputs.out_dir.display()),
    format!("--out-dir={}", inputs.out_dir.display()),
    // Root every local item so the collector walks the (otherwise-uncalled) `__vale_*` stub fns.
    "-Clink-dead-code".to_string(),
  ];
  for ext in &inputs.externs {
    let rlib = match &ext.rlib {
      Some(path) => path.clone(),
      None => resolve_rlib_from_deps(&ext.name, &inputs.dependency_dirs)?,
    };
    rustc_args.push(format!("--extern={}={}", ext.name, rlib.display()));
  }
  for dir in &inputs.dependency_dirs {
    rustc_args.push(format!("-Ldependency={}", dir.display()));
  }

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
  files.put(
    parse_arena.intern_file_coordinate(package_coord, "0.vale"),
    inputs.vale_source.clone(),
  );
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
    rustc_driver::run_compiler(&rustc_args, &mut callbacks);
  });
  if let Some(err) = typing_error_slot.into_inner() {
    return Err(format!("the Vale program failed to typecheck, so no body was emitted:\n{err}"));
  }
  let process_exit = if rustc_exit == 0 {
    let exe = inputs.out_dir.join("stub");
    let output = process::Command::new(&exe)
      .output()
      .map_err(|e| format!("could not run the driven bin at {}: {e}", exe.display()))?;
    Some(output.status.code().unwrap_or(-1))
  } else {
    None
  };
  Ok(DriveResult { rustc_exit, process_exit, firings: firings_slot.into_inner() })
}

/// Resolve a bare `--extern <name>` to its rlib by scanning the `-L dependency` dirs for the crate's
/// artifact. Cargo names rlibs `lib<name>-<hash>.rlib` (content-hashed) in `target/debug/deps/`, so a
/// literal path would mean hunting the hash each time; this finds it by crate name. Also accepts an
/// un-hashed `lib<name>.rlib`. Exactly one match is required — none or several is a clear error.
fn resolve_rlib_from_deps(name: &str, dependency_dirs: &[PathBuf]) -> Result<PathBuf, String> {
  let hashed_prefix = format!("lib{name}-");
  let exact = format!("lib{name}.rlib");
  let mut matches: Vec<PathBuf> = Vec::new();
  for dir in dependency_dirs {
    let Ok(entries) = fs::read_dir(dir) else {
      continue;
    };
    for entry in entries.flatten() {
      let path = entry.path();
      let Some(file_name) = path.file_name().and_then(|f| f.to_str()) else {
        continue;
      };
      let is_hashed = file_name.starts_with(&hashed_prefix) && file_name.ends_with(".rlib");
      if is_hashed || file_name == exact {
        matches.push(path);
      }
    }
  }
  match matches.as_slice() {
    [one] => Ok(one.clone()),
    [] => Err(format!(
      "--extern {name}: found no lib{name}[-<hash>].rlib in any -L dependency dir (looked in {} dir(s))",
      dependency_dirs.len()
    )),
    many => Err(format!(
      "--extern {name}: ambiguous — {} candidate rlibs in the -L dependency dirs: {}",
      many.len(),
      many.iter().map(|p| p.display().to_string()).collect::<Vec<_>>().join(", ")
    )),
  }
}

/// Callbacks for the driven path — the non-test twin of the harness's `DrivenCallbacks`. The
/// arenas/interners and the owned `HinputsT` live in `drive_and_link`'s frame; this only holds borrows,
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
