// The tier-1 interop harness: compile one Vale program against a real `TyCtxt`.
//
// rustc is **not** a separate process here. `rustc-dev` plus `#![feature(rustc_private)]` links
// rustc's own crates into the test binary, so `run_compiler` is an ordinary call — it runs rustc
// on a thread it spawns and calls back with a live `TyCtxt`. Nothing is serialized, because
// nothing crosses a process.
//
// That is why the corpus lives in the lib's own test target rather than in a `[[bin]]` or an
// integration test. `NodeRefT` and the `collect_*` macros are behind `#[cfg(test)] pub mod test;`
// (`typing/mod.rs`), so they exist *only* here — an integration test links the lib as an ordinary
// dependency and a binary is not `cfg(test)` at all. Hosting rustc and asserting on the typed AST
// are both possible in exactly one place, and this is it. Arch §26b.2 records the measurements.
//
// Three consequences, each load-bearing:
//   - **No `install_ice_hook`.** It sets a process-global panic hook, and this process holds the
//     rest of the suite; installing it would dress every other test's failure as a rustc ICE.
//   - **Only owned data escapes the callback.** `TyCtxt<'tcx>` and Vale's arenas die when
//     `after_expansion` returns. The extractor is higher-ranked (`for<'s, 't>`) with its result
//     type `R` fixed *outside* the quantifier, so `R` cannot mention those lifetimes — which makes
//     this a compile error to violate rather than a rule to remember.
//   - **Per-case output directories**, so concurrent cases do not race on one rlib path (@TMBFIZ).
//
// The surviving hazard is a rustc *fatal* error. Measured: it costs one case, not the run —
// rustc emits its diagnostic and then **unwinds**, so `catch_with_exit_code` turns it back into a
// value. `a_fatal_rustc_error_costs_one_case` in `cases.rs` is the standing regression test.
// Fixtures only need to *parse*, though: `after_expansion` runs before type checking and we return
// `Compilation::Stop`, so a fixture that type-errors is invisible here. Tier 2 would catch that
// rot; tier 1 structurally cannot.

use std::collections::HashMap;
use std::env::var;
use std::fs::read_dir;
use tempfile::TempDir;
use std::path::{Path, PathBuf};
use std::process::Command;

use bumpalo::Bump;
use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler as RustcCompiler;
use rustc_middle::ty::TyCtxt;

use std::sync::Arc;

use crate::backend_ffi::metal_lowerer::ExternAbi;
use crate::code_source::{CodeSource, Source};
use crate::compile_options::GlobalOptions;
use crate::instantiating::instantiating_interner::InstantiatingInterner;
use crate::instantiating::instantiator;
use crate::instantiating::ast::ast::FunctionExportI;
use crate::instantiating::instantiator::InstantiatedOutputsI;
use crate::instantiating::rust_interop::{
  arm_driver_state, consumer_fill_modules, vale_override_queries, DriverState,
};
use crate::typing::compiler::Compiler;
use std::cell::RefCell;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::postparsing::ScoutCompilation;
use crate::scout_arena::ScoutArena;
use crate::typing::compilation::TypingPassCompilation;
use crate::typing::hinputs_t::HinputsT;
use crate::typing::oracles::Oracles;
use crate::typing::rust_interop::{
  Case, Expect, LoggingOracle, OracleCall, OracleQuery, TyCtxtOracle,
};
use crate::typing::typing_interner::TypingInterner;
use crate::typing::TypingPassOptions;
use crate::utils::code_hierarchy::{FileCoordinateMap, PackageCoordinate};

/// A typing-pass failure, owned so it can outlive the compilation that produced it.
///
/// `variant` is the `ICompileErrorT` arm's name, taken from the leading identifier of the derived
/// `Debug` rendering. That the variant name comes first is a documented property of
/// `#[derive(Debug)]` on an enum, and it is deliberately the *only* part of that rendering
/// anything keys on — a negative case should pin *which* error, not how its fields happen to
/// print. The alternative is a `&'static str` method with one arm per variant in
/// `compiler_error_reporter.rs`, which is a core file this arc has otherwise left untouched.
///
/// `detail` is the whole rendering, for the failure message. Nothing asserts on it.
#[derive(Clone, Debug)]
pub struct CompileFailure {
  pub variant: String,
  pub detail: String,
}

impl CompileFailure {
  fn from_debug(rendered: String) -> CompileFailure {
    let variant =
      rendered.chars().take_while(|c| c.is_alphanumeric() || *c == '_').collect::<String>();
    CompileFailure { variant, detail: rendered }
  }

  /// Did the typing pass fail with this `ICompileErrorT` arm?
  pub fn is(&self, variant: &str) -> bool {
    self.variant == variant
  }
}

/// Owned counts from running the instantiator (monomorphizer) on a case's typing output. Present
/// only when the case both compiled and was run through the instantiator (`run_case_instantiated`).
///
/// The instantiator (`instantiator::translate`, typing output → `HinputsI`) is the pass past typing,
/// toward codegen, and it runs with no oracle and no backend. Reaching it at all is the point: a case
/// that typechecks but that the instantiator cannot monomorphize `panic!`s inside `translate`, which
/// surfaces here as a test failure. The counts let a case assert its program actually monomorphized to
/// denizens rather than to nothing.
#[derive(Clone, Copy, Debug)]
pub struct InstantiationSummary {
  pub functions: usize,
  pub structs: usize,
  pub interfaces: usize,
}

/// What one case observed: whether it compiled, every question the oracle was asked, and — when the
/// case was run through the instantiator — the monomorphized denizen counts.
///
/// The log is here for one job — **vacuity**. "The program compiled" is weak evidence on its own,
/// because a program that would compile anyway proves nothing about interop. Everything else is
/// better asserted by the Vale program itself or by walking the typed AST.
pub struct CaseOutcome<R> {
  pub compiled: Result<R, CompileFailure>,
  pub oracle_log: Vec<OracleCall>,
  /// `Some` only when the case ran through the instantiator and typing succeeded.
  pub instantiation: Option<InstantiationSummary>,
}

impl<R> CaseOutcome<R> {
  /// What the extractor pulled out, or a failure naming the error and dumping the log.
  pub fn expect_compiled(&self) -> &R {
    match &self.compiled {
      Ok(r) => r,
      Err(e) => panic!(
        "expected the Vale program to typecheck, but it failed with {}:\n{}\n\
                 --- oracle log ---\n{}",
        e.variant,
        e.detail,
        self.rendered_log()
      ),
    }
  }

  /// The failure, or a panic if the program unexpectedly compiled.
  pub fn expect_failure(&self) -> &CompileFailure {
    match &self.compiled {
      Err(e) => e,
      Ok(_) => panic!(
        "expected the Vale program to fail, but it typechecked:\n--- oracle log ---\n{}",
        self.rendered_log()
      ),
    }
  }

  /// Check the outcome against what the case **declared**, returning the extracted value when
  /// the case was expected to compile.
  ///
  /// This is the half of a case's expectation that tier 1 can check. `Returns(n)` says the
  /// program typechecks *and* yields `n`; only the first half is observable without running it,
  /// and the second is precisely what tier 2 exists for. Keeping the declaration whole here —
  /// rather than letting tier 1 record only "it compiles" — is what lets tier 2 read the same
  /// case and add nothing.
  pub fn check(&self, case: &Case) -> Option<&R> {
    match case.expect {
      Expect::Returns(_) => Some(self.expect_compiled()),
      Expect::FailsToCompile(variant) => {
        let failure = self.expect_failure();
        assert!(
          failure.is(variant),
          "expected the program to fail with {variant}, but it failed with {}:\n{}",
          failure.variant,
          failure.detail
        );
        None
      }
      Expect::RustcFails => panic!(
        "case `{}` declares that rustc fails, so it has no Vale outcome to check — use \
                 `try_run_case` and assert the result is `None`",
        case.name
      ),
    }
  }

  /// Was a question matching `pred` asked? The vacuity assertion.
  pub fn asked(&self, pred: impl Fn(&OracleQuery) -> bool) -> bool {
    self.oracle_log.iter().any(|c| pred(&c.query))
  }

  /// The first answer matching `pred`, for cases that assert on what came back.
  pub fn find_query(&self, pred: impl Fn(&OracleQuery) -> bool) -> Option<&OracleQuery> {
    self.oracle_log.iter().map(|c| &c.query).find(|q| pred(q))
  }

  /// The log as a person reads it. Only ever used in failure messages.
  pub fn rendered_log(&self) -> String {
    self.oracle_log.iter().map(|c| c.rendered.as_str()).collect::<Vec<_>>().join("\n")
  }

  /// The monomorphized denizen counts, or a panic if the case did not run through the instantiator
  /// (or did not compile). Reaching here at all means `translate` did not panic on the program.
  pub fn expect_instantiated(&self) -> &InstantiationSummary {
    self
      .instantiation
      .as_ref()
      .expect("case produced no instantiation — run it with `run_case_instantiated` and expect it to compile")
  }
}

struct CaseCallbacks<'a, F, R> {
  vale_source: &'a str,
  /// The Vale package the case's source is compiled as. Almost always `"test"`; a case naming
  /// the reserved `rust` module is what makes the reservation observable at all.
  package_module: &'a str,
  extract: F,
  /// Run the instantiator (`translate`) on the typing output when the program compiles, recording
  /// the denizen counts. Off by default so most cases stay pure typing-pass tests.
  instantiate: bool,
  outcome: Option<CaseOutcome<R>>,
}

impl<'a, F, R> Callbacks for CaseCallbacks<'a, F, R>
where
  F: for<'s, 't> Fn(&HinputsT<'s, 't>) -> R + Send,
  R: Send,
{
  fn after_expansion<'tcx>(&mut self, _compiler: &RustcCompiler, tcx: TyCtxt<'tcx>) -> Compilation {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);

    // The case's package, built here rather than taken from a helper, because which package the
    // Vale source is compiled as is itself under test — `"test"` is the ordinary value, not a
    // fixed one.
    let package_coord =
      parse_arena.intern_package_coordinate(parse_arena.intern_str(self.package_module), &[]);
    let mut files = FileCoordinateMap::<String>::new();
    files.put(
      parse_arena.intern_file_coordinate(package_coord, "0.vale"),
      self.vale_source.to_string(),
    );
    let code_source = CodeSource::new(vec![Source::from_code_map(&files)]);

    let global_options = GlobalOptions {
      sanity_check: true,
      use_overload_index: true,
      use_optimized_solver: true,
      verbose_errors: true,
      debug_output: true,
    };

    // Parse before building the oracle. The oracle's importable set is exactly the program's real
    // `import rust.X.Y` statements, each joined back into the dotted path the oracle resolves.
    // `ScoutCompilation` needs no oracle, so it runs first and yields the parsed imports; the tiny
    // program is parsed again inside the typing compilation below, which costs nothing.
    let mut import_paths: Vec<String> = Vec::new();
    {
      let mut scout = ScoutCompilation::new(
        &scout_arena,
        &keywords,
        &parser_keywords,
        &parse_arena,
        vec![package_coord],
        &code_source,
        global_options.clone(),
      );
      if let Ok(scoutput) = scout.get_scoutput() {
        for program in scoutput.file_coord_to_contents.values() {
          for imp in program.imports {
            if imp.module_name == keywords.rust {
              let mut segments: Vec<&str> = imp.package_names.iter().map(|s| s.0).collect();
              segments.push(imp.importee_name.0);
              import_paths.push(segments.join("."));
            }
          }
        }
      }
    }
    // String-dedup so a program that names the same item twice does not register it twice.
    let mut import_path_strs: Vec<&str> = Vec::new();
    for path in &import_paths {
      if !import_path_strs.contains(&path.as_str()) {
        import_path_strs.push(path.as_str());
      }
    }

    // No package coordinate is handed to the oracle: each item derives its own from
    // `tcx.def_path`, so items from different crates cannot collide on one coordinate.
    let real = TyCtxtOracle::new(tcx, &scout_arena, &import_path_strs);
    let compiling = tcx.crate_name(rustc_span::def_id::LOCAL_CRATE).to_string();
    let logging = LoggingOracle::new(&real, &compiling);

    let mut compile = TypingPassCompilation::new(
      &typing_interner,
      &scout_arena,
      &keywords,
      &parser_keywords,
      &parse_arena,
      vec![package_coord],
      &code_source,
      TypingPassOptions {
        global_options: global_options.clone(),
        debug_out: Arc::new(|x: &str| println!("{}", x)),
        tree_shaking_enabled: true,
      },
      Oracles::with_rust(&logging),
    );
    let mut instantiation: Option<InstantiationSummary> = None;
    let compiled = match compile.get_compiler_outputs() {
      Ok(coutputs) => {
        let extracted = (self.extract)(coutputs);
        if self.instantiate {
          // The pass past typing: monomorphize the typed program to `HinputsI`. No oracle, no backend
          // — that is `pass_manager::build`'s job downstream. A program that typechecks but cannot be
          // instantiated panics inside `translate`, which fails the test loudly. That is the coverage.
          let instantiating_bump = Bump::new();
          let instantiating_interner = InstantiatingInterner::new(&instantiating_bump);
          let monouts = instantiator::translate(
            &global_options,
            &instantiating_interner,
            &typing_interner,
            &scout_arena,
            &keywords,
            coutputs,
          );
          instantiation = Some(InstantiationSummary {
            functions: monouts.functions.len(),
            structs: monouts.structs.len(),
            interfaces: monouts.interfaces.len(),
          });
        }
        Ok(extracted)
      }
      Err(e) => Err(CompileFailure::from_debug(format!("{e:?}"))),
    };
    self.outcome = Some(CaseOutcome { compiled, oracle_log: logging.calls(), instantiation });

    Compilation::Stop
  }
}

fn fixtures_dir(name: &str) -> PathBuf {
  PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/typing/rust_interop").join(name)
}

/// Compile a fixture's `stub.rs` **to completion**, so a fixture cannot rot into invalid Rust
/// unnoticed.
///
/// Tier 1 structurally cannot catch this: `after_expansion` runs before type checking and the
/// callback returns `Compilation::Stop`, so only *parse* errors ever reach a case. The dependency
/// crates are already covered — `build_dep_rlib` runs rustc over them in full and asserts success —
/// so the stub is the one file nothing was checking.
///
/// Returns rustc's stderr on failure, so the message names the actual error.
pub fn compile_check_fixture(fixture: &str) -> Result<(), String> {
  let fixture_dir = fixtures_dir(fixture);
  // One scratch dir per run, unique and self-cleaning. A `Case`/fixture name is shared across
  // parallel tests, so keying the dir on it let concurrent `build_dep_rlib` runs corrupt one
  // another's rlib; `TempDir` gives each run its own directory, removed on drop.
  let out_dir_tmp = TempDir::new().map_err(|e| format!("could not create scratch dir: {e}"))?;
  let out_dir = out_dir_tmp.path();

  let deps = dep_crates(&fixture_dir);
  for (crate_name, source) in &deps {
    build_dep_rlib(crate_name, source, &out_dir);
  }

  let mut command = Command::new(var("RUSTC").unwrap_or_else(|_| "rustc".to_string()));
  command
    .arg(fixture_dir.join("stub.rs"))
    .args(["--crate-type=lib", "--crate-name=stub", "--edition=2021"])
    .arg(format!("-L{}", out_dir.display()))
    .arg(format!("--out-dir={}", out_dir.display()));
  for (crate_name, _) in &deps {
    command.arg(format!(
      "--extern={crate_name}={}",
      out_dir.join(format!("lib{crate_name}.rlib")).display()
    ));
  }

  let output = command.output().map_err(|e| format!("could not run rustc: {e}"))?;
  if output.status.success() {
    Ok(())
  } else {
    Err(String::from_utf8_lossy(&output.stderr).to_string())
  }
}

/// Every dependency crate a fixture directory declares: one per `*.rs` other than `stub.rs`, with
/// the crate name taken from the file stem.
///
/// Discovered from the directory rather than listed on the `Case`, so that adding a crate to a
/// fixture is one file and the directory stays the single statement of what a fixture *is*.
/// Sorted, because `read_dir` order is filesystem-dependent and rustc's `-L`/`--extern` arguments
/// would otherwise vary run to run — the same determinism discipline the cache keys use
/// (arch §7.6).
fn dep_crates(fixture_dir: &Path) -> Vec<(String, PathBuf)> {
  let mut crates: Vec<(String, PathBuf)> = read_dir(fixture_dir)
    .expect("could not read the fixture directory")
    .map(|e| e.expect("could not read a fixture directory entry").path())
    .filter(|p| p.extension().is_some_and(|e| e == "rs"))
    .filter(|p| p.file_stem().is_some_and(|s| s != "stub"))
    .map(|p| {
      let name = p
        .file_stem()
        .expect("a .rs path always has a stem")
        .to_str()
        .expect("fixture crate names are utf8")
        .to_string();
      (name, p)
    })
    .collect();
  crates.sort();
  crates
}

fn sysroot() -> String {
  let out = Command::new(var("RUSTC").unwrap_or_else(|_| "rustc".to_string()))
    .arg("--print=sysroot")
    .output()
    .expect("could not run rustc to find the sysroot");
  String::from_utf8(out.stdout).expect("sysroot was not utf8").trim().to_string()
}

/// Build one dependency crate to an rlib in `out_dir`.
///
/// `-L out_dir` so a fixture's crates may depend on each other; `dep_crates`' sorted order is what
/// makes that well-defined, since a crate can only name one already built.
fn build_dep_rlib(crate_name: &str, source: &Path, out_dir: &Path) {
  let status = Command::new(var("RUSTC").unwrap_or_else(|_| "rustc".to_string()))
    .arg(source)
    .args(["--crate-type=lib", "--edition=2021"])
    .arg(format!("--crate-name={crate_name}"))
    .arg(format!("-L{}", out_dir.display()))
    .arg("--out-dir")
    .arg(out_dir)
    .status()
    .expect("could not run rustc to build the dependency rlib");
  assert!(status.success(), "building the dependency rlib for `{crate_name}` failed");
}

/// Compile `case`'s Vale program against its Rust fixture, and return what `extract` pulled out of
/// the typing pass's output.
///
/// The case supplies the program, the fixture and the allowlist; the extractor is the tier's, not
/// the case's — how to observe an outcome differs per tier, so it does not belong in the corpus.
pub fn run_case<R: Send>(
  case: &Case,
  extract: impl for<'s, 't> Fn(&HinputsT<'s, 't>) -> R + Send,
) -> CaseOutcome<R> {
  run_case_in_package(case, "test", extract)
}

/// `run_case`, compiling the case's program as a caller-chosen Vale package.
///
/// `"test"` is the ordinary value rather than a default hidden in a helper, so a case that needs
/// to name the reserved `rust` module is an argument rather than a second code path.
pub fn run_case_in_package<R: Send>(
  case: &Case,
  package_module: &str,
  extract: impl for<'s, 't> Fn(&HinputsT<'s, 't>) -> R + Send,
) -> CaseOutcome<R> {
  try_run_case_in_package(case, package_module, false, extract)
    .expect("rustc returned without ever reaching after_expansion")
}

/// `run_case`, but also runs the instantiator (monomorphizer) on the typing output, recording the
/// denizen counts in the outcome's `instantiation`. Use it to prove a case reaches the pass past
/// typing — `translate` panics if the typechecked program cannot be monomorphized, failing the test.
pub fn run_case_instantiated<R: Send>(
  case: &Case,
  extract: impl for<'s, 't> Fn(&HinputsT<'s, 't>) -> R + Send,
) -> CaseOutcome<R> {
  try_run_case_in_package(case, "test", true, extract)
    .expect("rustc returned without ever reaching after_expansion")
}

/// `run_case` for the one case that expects rustc itself to fail: `None` means `after_expansion`
/// never ran, so there is no Vale outcome to report.
pub fn try_run_case<R: Send>(
  case: &Case,
  extract: impl for<'s, 't> Fn(&HinputsT<'s, 't>) -> R + Send,
) -> Option<CaseOutcome<R>> {
  try_run_case_in_package(case, "test", false, extract)
}

/// Callbacks for the rustc-driven path. Unlike `CaseCallbacks`, the arenas/interners and the owned
/// `HinputsT`/`monouts` live in the *caller's* frame (see `run_case_rustc_driven`); this only holds
/// borrows of them, runs the Vale typing pass in `after_expansion` (writing the owned `HinputsT` into
/// the caller's slot), and arms the scoped pointer so the `per_instance_mir` provider can drive the
/// instantiator during codegen.
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
  // Set when the Vale program fails to typecheck. drive_rustc reads it after run_compiler and panics
  // on the test thread, so a broken driven program surfaces its diagnostic instead of looking like an
  // empty `__vale_main -> []` firing log with an undefined `__vale_main` at link.
  typing_error_slot: &'ctx RefCell<Option<String>>,
  state_ptr: *const (),
}

// SAFETY: `rustc_driver::run_compiler` moves the callbacks onto a thread it spawns, but it *joins*
// that thread before returning — the calling (test) thread is parked in `run_compiler` for the whole
// compilation. So the rustc thread has exclusive access to everything this borrows (the arenas,
// interners, and slots that live in `run_case_rustc_driven`'s frame) for exactly the window it uses
// them, and the calling thread touches none of it until `run_compiler` returns. This is the same
// scoped-borrow guarantee `std::thread::scope` formalizes; the `!Send`/`!Sync` arena internals are
// never actually accessed from two threads at once. The `*const ()` is a pointer into that same
// frame, valid for the same window.
unsafe impl<'ctx, 's, 't, 'p> Send for DrivenCallbacks<'ctx, 's, 't, 'p> {}

impl<'ctx, 's, 't, 'p> Callbacks for DrivenCallbacks<'ctx, 's, 't, 'p> {
  fn config(&mut self, config: &mut rustc_interface::Config) {
    config.override_queries = Some(vale_override_queries);
    // Install the codegen-time hook that drives Vale's backend. It's a process-global `OnceLock`
    // (first-wins; every driven run installs the same fn), and no-ops unless a `DriverState` is armed.
    rustc_codegen_llvm::set_fill_extra_modules_hook(consumer_fill_modules);
  }

  fn after_expansion<'tcx>(&mut self, _compiler: &RustcCompiler, tcx: TyCtxt<'tcx>) -> Compilation {
    // Scout once: the imports feed the oracle's allowlist, and the same scout's code map + AST feed
    // typing. Mirrors TypingPassCompilation::get_compiler_outputs, but calls Compiler::evaluate
    // directly so we get an *owned* HinputsT to move into the caller's slot (the compilation itself
    // borrows the tcx-bound oracle and cannot outlive this callback).
    let mut scout = ScoutCompilation::new(
      self.scout_arena,
      self.keywords,
      self.parser_keywords,
      self.parse_arena,
      vec![self.package_coord],
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
      // Capture the failure rather than swallowing it. A None hinputs would otherwise flow silently
      // into empty leaf collection and no body emission, so drive_rustc surfaces this after the run.
      Err(err) => *self.typing_error_slot.borrow_mut() = Some(format!("{err:?}")),
    }

    // Arm the scoped pointer on this (the rustc) thread, where the provider will read it during
    // codegen. The state outlives run_compiler, so the pointer stays valid for every provider call.
    arm_driver_state(self.state_ptr);
    Compilation::Continue
  }
}

/// The result of a rustc-driven run: the provider's per-item firing log, rustc's own exit code
/// (`0` means it drove all the way through codegen without erroring on the reified Rust leaves), and,
/// when the run built and executed a bin, the exit code of that process (`None` for a lib run, or when
/// rustc failed before an executable existed).
pub struct DrivenRun {
  pub firings: Vec<String>,
  pub rustc_exit: i32,
  pub process_exit: Option<i32>,
}

/// Drive instantiation from rustc's mono collector (Milestone M), returning just the firing log. See
/// `run_case_rustc_driven_full` for the version that also reports rustc's exit code.
pub fn run_case_rustc_driven(case: &Case) -> Vec<String> {
  run_case_rustc_driven_full(case).firings
}

/// Drive instantiation from rustc's mono collector (Milestone M): compile the case's stub to
/// completion with the `per_instance_mir` override installed, and return, per Vale item the provider
/// fired on, the Rust requests the instantiator collected for it (plus rustc's exit code). A non-empty
/// firing log means rustc's collector walked a Vale item and drove our monomorphizer — "rustc drives
/// us." An exit code of `0` means rustc then codegen'd the reified leaves without erroring.
pub fn run_case_rustc_driven_full(case: &Case) -> DrivenRun {
  drive_rustc(case, /*emit_backend=*/ false, "lib", /*run_exe=*/ false)
}

/// Stage 2 (`#2b`): drive rustc *and* have the `fill_extra_modules` hook lower + emit the Vale bodies
/// into rustc's borrowed module. Still a lib crate — the emission is verified (the hook asserts the
/// backend's rc is 0) but not linked or run; Stage 3 does that with a bin crate.
pub fn run_case_rustc_driven_emitting(case: &Case) -> DrivenRun {
  drive_rustc(case, /*emit_backend=*/ true, "lib", /*run_exe=*/ false)
}

/// Stage 3 (tier 2): drive rustc to a **linked, runnable bin**, emit the Vale bodies into it, run the
/// produced executable, and report its process exit code in `DrivenRun::process_exit`. This is the
/// only path that observes what `main` actually returns — arch §26b's tier 2 ("run it and check N").
pub fn run_case_rustc_driven_and_run(case: &Case) -> DrivenRun {
  drive_rustc(case, /*emit_backend=*/ true, "bin", /*run_exe=*/ true)
}

fn drive_rustc(case: &Case, emit_backend: bool, crate_type: &str, run_exe: bool) -> DrivenRun {
  let fixture_dir = fixtures_dir(case.fixture);
  // One scratch dir per run, unique and self-cleaning (see compile_check_fixture): keying on
  // case.name raced across the parallel tests that share a case.
  let out_dir_tmp = TempDir::new().expect("could not create scratch dir");
  let out_dir = out_dir_tmp.path();
  let deps = dep_crates(&fixture_dir);
  for (crate_name, source) in &deps {
    build_dep_rlib(crate_name, source, &out_dir);
  }
  let mut rustc_args: Vec<String> = vec![
    "valec-rs".to_string(),
    fixture_dir.join("stub.rs").display().to_string(),
    format!("--crate-type={crate_type}"),
    "--crate-name=stub".to_string(),
    "--edition=2021".to_string(),
    format!("--sysroot={}", sysroot()),
    format!("-L{}", out_dir.display()),
    format!("--out-dir={}", out_dir.display()),
    // Root every local item so the collector walks the (otherwise-uncalled) `__vale_*` stub fns.
    "-Clink-dead-code".to_string(),
  ];
  for (crate_name, _) in &deps {
    rustc_args.push(format!(
      "--extern={crate_name}={}",
      out_dir.join(format!("lib{crate_name}.rlib")).display()
    ));
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
  files.put(parse_arena.intern_file_coordinate(package_coord, "0.vale"), case.vale.to_string());
  let code_source = CodeSource::new(vec![Source::from_code_map(&files)]);

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
    emit_backend,
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
  // Surface a Vale typing failure on the test thread. Without this, a broken driven program leaves
  // hinputs None and reads downstream as an empty `__vale_main -> []` firing log plus an undefined
  // `__vale_main` at link, masking the real cause.
  if let Some(err) = typing_error_slot.into_inner() {
    panic!("driven case '{}' failed to typecheck, so no Vale body was emitted:\n{err}", case.name);
  }
  // Run the produced executable while out_dir_tmp is still alive (it self-deletes on drop). A bin
  // build lands its executable at <out_dir>/<crate-name>; only run when rustc actually linked one.
  let process_exit = if run_exe && rustc_exit == 0 {
    let exe = out_dir.join("stub");
    let output = Command::new(&exe)
      .output()
      .unwrap_or_else(|e| panic!("could not run the driven bin at {}: {e}", exe.display()));
    Some(output.status.code().unwrap_or(-1))
  } else {
    None
  };
  DrivenRun { firings: firings_slot.into_inner(), rustc_exit, process_exit }
}

fn try_run_case_in_package<R: Send>(
  case: &Case,
  package_module: &str,
  instantiate: bool,
  extract: impl for<'s, 't> Fn(&HinputsT<'s, 't>) -> R + Send,
) -> Option<CaseOutcome<R>> {
  let fixture_dir = fixtures_dir(case.fixture);
  // One scratch dir per run, unique and self-cleaning (see compile_check_fixture): keying on
  // case.name raced across the parallel tests that share a case.
  let out_dir_tmp = TempDir::new().expect("could not create scratch dir");
  let out_dir = out_dir_tmp.path();

  let deps = dep_crates(&fixture_dir);
  for (crate_name, source) in &deps {
    build_dep_rlib(crate_name, source, &out_dir);
  }

  let mut rustc_args: Vec<String> = vec![
    "valec-rs".to_string(),
    fixture_dir.join("stub.rs").display().to_string(),
    "--crate-type=lib".to_string(),
    "--crate-name=stub".to_string(),
    "--edition=2021".to_string(),
    format!("--sysroot={}", sysroot()),
    format!("-L{}", out_dir.display()),
    format!("--out-dir={}", out_dir.display()),
  ];
  for (crate_name, _) in &deps {
    rustc_args.push(format!(
      "--extern={crate_name}={}",
      out_dir.join(format!("lib{crate_name}.rlib")).display()
    ));
  }

  let mut callbacks =
    CaseCallbacks { vale_source: case.vale, package_module, extract, instantiate, outcome: None };
  // rustc's fatal-error path does not return — it emits the diagnostic and then **unwinds**,
  // with a `FatalErrorMarker` payload rather than a string. `catch_with_exit_code` is rustc's
  // own way of turning that back into a value, and using it is what keeps a broken fixture from
  // surfacing as a test failure with no message attached.
  //
  // Worth being precise, because the earlier assumption was different: the hazard was expected
  // to be a `process::exit` that would take the whole suite down. It is an ordinary unwind, so
  // even uncaught it would have cost one test — this just makes the outcome legible instead of
  // blank.
  let rustc_exit = rustc_driver::catch_with_exit_code(|| {
    rustc_driver::run_compiler(&rustc_args, &mut callbacks);
  });
  if rustc_exit != 0 {
    return None;
  }
  callbacks.outcome
}
