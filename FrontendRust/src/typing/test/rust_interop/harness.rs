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

use std::path::{Path, PathBuf};
use std::process::Command;

use bumpalo::Bump;
use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler as RustcCompiler;
use rustc_middle::ty::TyCtxt;

use crate::code_source::CodeSource;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::tests::tests::new_test_code_map;
use crate::typing::hinputs_t::HinputsT;
use crate::typing::rust_interop::{LoggingOracle, OracleCall, OracleQuery, TyCtxtOracle};
use crate::typing::test::compiler_test_compilation::compiler_test_compilation_with_rust_oracle;
use crate::typing::typing_interner::TypingInterner;

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
        let variant = rendered
            .chars()
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .collect::<String>();
        CompileFailure { variant, detail: rendered }
    }

    /// Did the typing pass fail with this `ICompileErrorT` arm?
    pub fn is(&self, variant: &str) -> bool {
        self.variant == variant
    }
}

/// What one case observed: whether it compiled, and every question the oracle was asked.
///
/// The log is here for one job — **vacuity**. "The program compiled" is weak evidence on its own,
/// because a program that would compile anyway proves nothing about interop. Everything else is
/// better asserted by the Vale program itself or by walking the typed AST.
pub struct CaseOutcome<R> {
    pub compiled: Result<R, CompileFailure>,
    pub oracle_log: Vec<OracleCall>,
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
        self.oracle_log
            .iter()
            .map(|c| c.rendered.as_str())
            .collect::<Vec<_>>()
            .join("\n")
    }
}

struct CaseCallbacks<'a, F, R> {
    vale_source: &'a str,
    allowed: &'a [&'a str],
    extract: F,
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
        let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, self.vale_source)]);
        let typing_interner = TypingInterner::new(&typing_bump);

        let module = scout_arena.intern_str("rust");
        let package = scout_arena.intern_str("mycrate");
        let coord = scout_arena.intern_package_coordinate(module, &[package]);
        let real = TyCtxtOracle::new(tcx, &scout_arena, coord, self.allowed);
        let compiling = tcx.crate_name(rustc_span::def_id::LOCAL_CRATE).to_string();
        let logging = LoggingOracle::new(&real, &compiling);

        let mut compile = compiler_test_compilation_with_rust_oracle(
            &typing_interner,
            &scout_arena,
            &keywords,
            &parser_keywords,
            &parse_arena,
            &code_source,
            &logging,
        );
        let compiled = match compile.get_compiler_outputs() {
            Ok(coutputs) => Ok((self.extract)(coutputs)),
            Err(e) => Err(CompileFailure::from_debug(format!("{e:?}"))),
        };
        self.outcome = Some(CaseOutcome { compiled, oracle_log: logging.calls() });

        Compilation::Stop
    }
}

fn fixtures_dir(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/typing/rust_interop").join(name)
}

fn sysroot() -> String {
    let out = Command::new(std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string()))
        .arg("--print=sysroot")
        .output()
        .expect("could not run rustc to find the sysroot");
    String::from_utf8(out.stdout).expect("sysroot was not utf8").trim().to_string()
}

fn build_dep_rlib(fixture_dir: &Path, out_dir: &Path) {
    std::fs::create_dir_all(out_dir).expect("could not create out dir");
    let status = Command::new(std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string()))
        .arg(fixture_dir.join("mycrate.rs"))
        .args(["--crate-type=lib", "--crate-name=mycrate", "--edition=2021"])
        .arg("--out-dir")
        .arg(out_dir)
        .status()
        .expect("could not run rustc to build the dependency rlib");
    assert!(status.success(), "building the dependency rlib failed");
}

/// Compile `vale_source` against `fixture`'s Rust crate, with `allowed` naming the importable
/// Rust items, and return what `extract` pulled out of the typing pass's output.
///
/// `case_name` names this case's private output directory.
pub fn run_case<R: Send>(
    fixture: &str,
    case_name: &str,
    vale_source: &str,
    allowed: &[&str],
    extract: impl for<'s, 't> Fn(&HinputsT<'s, 't>) -> R + Send,
) -> CaseOutcome<R> {
    try_run_case(fixture, case_name, vale_source, allowed, extract)
        .expect("rustc returned without ever reaching after_expansion")
}

/// `run_case` for the one case that expects rustc itself to fail: `None` means `after_expansion`
/// never ran, so there is no Vale outcome to report.
pub fn try_run_case<R: Send>(
    fixture: &str,
    case_name: &str,
    vale_source: &str,
    allowed: &[&str],
    extract: impl for<'s, 't> Fn(&HinputsT<'s, 't>) -> R + Send,
) -> Option<CaseOutcome<R>> {
    let fixture_dir = fixtures_dir(fixture);
    let out_dir = std::env::temp_dir().join("vale-interop-cases").join(case_name);
    build_dep_rlib(&fixture_dir, &out_dir);

    let rustc_args: Vec<String> = vec![
        "valec-rs".to_string(),
        fixture_dir.join("stub.rs").display().to_string(),
        "--crate-type=lib".to_string(),
        "--crate-name=stub".to_string(),
        "--edition=2021".to_string(),
        format!("--sysroot={}", sysroot()),
        format!("-L{}", out_dir.display()),
        format!("--extern=mycrate={}", out_dir.join("libmycrate.rlib").display()),
        format!("--out-dir={}", out_dir.display()),
    ];

    let mut callbacks = CaseCallbacks { vale_source, allowed, extract, outcome: None };
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
