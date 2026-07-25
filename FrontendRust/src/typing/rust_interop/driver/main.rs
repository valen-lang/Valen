// valec-rs — the rustc-hosted Vale driver.
//
// rustc cannot be called as a library that hands back a `TyCtxt`: the type exists only inside
// `rustc_driver::run_compiler`'s callback, and its `'tcx` is tied to arenas owned by that
// stack frame. So the control flow inverts — **rustc hosts, and Vale's typing pass runs
// inside `Callbacks::after_expansion`**. That is the architecture doc's §20.3 shape, so this
// binary is a miniature of the real thing rather than a detour.
//
// Why a binary rather than a `#[test]`: `run_compiler` effectively owns the process. It
// installs its own panic hook and its fatal-error paths exit rather than return, so hosting
// it inside libtest risks taking the whole suite down on one bad compile. The test spawns
// this binary and checks its output — which is also the only canary shape that catches a
// wrong artifact rather than merely a successful build.

#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use std::path::{Path, PathBuf};
use std::process::Command;

use bumpalo::Bump;
use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;

use std::sync::Arc;

use frontend_rust::code_source::CodeSource;
use frontend_rust::compile_options::GlobalOptions;
use frontend_rust::keywords::Keywords;
use frontend_rust::parse_arena::ParseArena;
use frontend_rust::scout_arena::ScoutArena;
use frontend_rust::tests::tests::new_test_code_map;
use frontend_rust::typing::compilation::{TypingPassCompilation, TypingPassOptions};
use frontend_rust::typing::oracles::Oracles;
use frontend_rust::typing::rust_interop::{LoggingOracle, RustOracle, TyCtxtOracle};
use frontend_rust::typing::typing_interner::TypingInterner;

/// Runs inside rustc, with a real `TyCtxt` in hand.
///
/// **Records only — never asserts.** `install_ice_hook` installs a process-global panic hook,
/// so a panic raised in here gets caught by rustc and reported as "the compiler unexpectedly
/// panicked, this is a bug" with an ICE dump, burying the real message above a backtrace. So
/// the callback accumulates results into `self`, and the assertions run in `main` after
/// `run_compiler` returns — outside rustc's panic-catching region.
///
/// That keeps ICE reporting meaning what it says, which matters more rather than less once we
/// start overriding queries and genuine ICEs become possible. Scoping the hook isn't the
/// alternative: it is global, so there is no region to scope it to.
#[derive(Default)]
struct ValeCallbacks {
    /// Oracle log from the run where the Rust function is importable.
    positive: Option<Result<Vec<String>, String>>,
    /// Result of the same program with nothing importable — the negative control.
    negative: Option<Result<Vec<String>, String>>,
}

impl Callbacks for ValeCallbacks {
    fn after_expansion<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        // `after_expansion` rather than `after_analysis`: every crate is loaded and signatures,
        // ADT defs and module children are all queryable here, but rustc has not yet
        // typechecked bodies — which is everything a read-only typing pass needs and nothing
        // it doesn't.
        //
        // The log has to be pulled out here, because it lives in the oracle, which lives in
        // this frame and dies with it — same `'tcx` nesting as the arenas.
        self.positive = Some(compile_vale(tcx, &["add_two_numbers", "make_counter", "Counter"]));
        self.negative = Some(compile_vale(tcx, &[]));
        Compilation::Stop
    }
}

/// Checks what the callback recorded. Runs after `run_compiler` has returned, so a failure
/// here is an ordinary assertion failure rather than a counterfeit rustc ICE.
fn check(callbacks: &ValeCallbacks) {
    let log = match callbacks.positive.as_ref().expect("the callback never ran") {
        Ok(log) => log,
        Err(e) => panic!("the Vale program failed to typecheck with Rust items importable: {e}"),
    };

    // "It compiled" is weak evidence on its own — this program would compile just as happily
    // if a Vale function of that name were in scope and the oracle were never consulted. The
    // log is what makes consultation an observed fact rather than an inference.
    // The type was imported and declared as a Vale citizen.
    assert!(
        log.iter().any(|l| l.contains(r#"importable_types -> [("Counter""#)),
        "the importer never asked for the importable types:\n{}",
        log.join("\n")
    );
    // Its method was discovered from the Rust side, not declared in Vale.
    assert!(
        log.iter().any(|l| l.contains(r#"methods"#) && l.contains(r#"("get""#)),
        "the importer never discovered Counter's methods:\n{}",
        log.join("\n")
    );
    // A Rust struct crossed as a *return type* — which is how the type reaches Vale at all,
    // by inference from a signature rather than by name.
    assert!(
        log.iter().any(|l| l.contains(r#"resolve_function("make_counter")"#)),
        "the free function that produces the Rust type never resolved:\n{}",
        log.join("\n")
    );
    assert!(
        log.iter().any(|l| l.contains("fn_sig") && l.contains("ret Struct(StructTT")),
        "no signature ever lowered a Rust struct to a Vale kind:\n{}",
        log.join("\n")
    );

    // If this compiled too, the positive case proves nothing about where resolution came from.
    assert!(
        callbacks.negative.as_ref().expect("the callback never ran").is_err(),
        "the program compiled with an empty allowlist, so resolution did not come from Rust"
    );

    println!("OK: a Rust type and its method resolved from a real TyCtxt");
    println!("--- oracle log ---");
    for line in log {
        println!("{line}");
    }
}

/// Runs Vale's typing pass over a program that calls a Rust function, with `allowed` naming
/// the importable Rust paths. Returns the oracle log on success.
fn compile_vale(tcx: TyCtxt<'_>, allowed: &[&str]) -> Result<Vec<String>, String> {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);

    // The arenas are created *inside* the callback on purpose: `'tcx` outlives them, so the
    // nesting is sound and the reverse would not be. Nothing built here survives the
    // callback, which is the containment property the design depends on.
    //
    // The program itself carries most of the assertions. If the return type were not `int`,
    // `main() int` would not typecheck; if the params were not `[int, int]`, the call would
    // not match; if the function did not resolve, this is `CouldntFindFunctionToCallT`.
    let code = r"
exported func main() int {
  return (make_counter()).get();
}";
    let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);

    // Scoping is membership in this list — the same mechanism an `import rust.X.Y` will
    // populate later, from a different source.
    let module = scout_arena.intern_str("rust");
    let package = scout_arena.intern_str("mycrate");
    let coord = scout_arena.intern_package_coordinate(module, &[package]);
    let real = TyCtxtOracle::new(tcx, &scout_arena, coord, allowed);
    // Tag the log with the crate this rustc invocation is compiling. Constant today because
    // only one invocation runs the typing pass; it stops being constant the moment a second
    // compile contributes entries.
    let compiling = tcx.crate_name(rustc_span::def_id::LOCAL_CRATE).to_string();
    let logging = LoggingOracle::new(&real, &compiling);

    let mut global_options = GlobalOptions::apply();
    // `apply()` defaults this off; the typing-pass test harness runs with it on, and this
    // driver replaces one of those tests — so it should not be checking less than the test
    // it stands in for.
    global_options.sanity_check = true;
    let options = TypingPassOptions {
        global_options,
        debug_out: Arc::new(|_: &str| {}),
        tree_shaking_enabled: true,
    };
    // The package the source lives in. An empty list here compiles nothing at all and still
    // returns `Ok` — which is exactly the silent-success the oracle log exists to catch, and
    // did.
    let test_module = parse_arena.intern_str("test");
    let test_tld = parse_arena.intern_package_coordinate(test_module, &[]);

    let mut compilation = TypingPassCompilation::new(
        &typing_interner,
        &scout_arena,
        &keywords,
        &parser_keywords,
        &parse_arena,
        vec![test_tld],
        &code_source,
        options,
        Oracles::with_rust(&logging),
    );

    match compilation.get_compiler_outputs() {
        Ok(_) => Ok(logging.calls().into_iter().map(|c| c.0).collect()),
        Err(e) => Err(format!("{e:?}")),
    }
}

fn sysroot() -> String {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let out = Command::new(rustc)
        .args(["--print", "sysroot"])
        .output()
        .expect("could not run rustc to determine sysroot");
    String::from_utf8(out.stdout)
        .expect("rustc sysroot was not utf8")
        .trim()
        .to_string()
}

fn main() {
    let mut args = std::env::args().skip(1);
    let fixture_dir = PathBuf::from(
        args.next()
            .unwrap_or_else(|| panic!("usage: valec-rs <fixture-dir> <out-dir>")),
    );
    let out_dir = PathBuf::from(
        args.next()
            .unwrap_or_else(|| panic!("usage: valec-rs <fixture-dir> <out-dir>")),
    );
    std::fs::create_dir_all(&out_dir).expect("could not create out dir");

    // Build the dependency rlib first, so the item Vale imports lives in an upstream crate
    // rather than in the crate under compilation. Direct rustc rather than a generated cargo
    // workspace: at one dep crate with one `pub fn`, a workspace generator is machinery we
    // would be writing before we know its shape.
    build_dep_rlib(&fixture_dir, &out_dir);

    rustc_driver::install_ice_hook("https://github.com/verdagon/Vale/issues", |_| {});

    let stub = fixture_dir.join("stub.rs");
    let rustc_args: Vec<String> = vec![
        "valec-rs".to_string(),
        stub.display().to_string(),
        "--crate-type=lib".to_string(),
        "--crate-name=stub".to_string(),
        "--edition=2021".to_string(),
        format!("--sysroot={}", sysroot()),
        format!("-L{}", out_dir.display()),
        format!("--extern=mycrate={}", out_dir.join("libmycrate.rlib").display()),
        format!("--out-dir={}", out_dir.display()),
    ];

    let mut callbacks = ValeCallbacks::default();
    let exit = rustc_driver::catch_with_exit_code(|| {
        rustc_driver::run_compiler(&rustc_args, &mut callbacks);
    });
    if exit != 0 {
        // rustc itself failed. Nothing of ours to check.
        std::process::exit(exit);
    }

    // Restore plain panic reporting before asserting. Moving the assertions out of the
    // callback is necessary but not sufficient: `install_ice_hook` sets a *process-global*
    // hook and never restores it, so a panic here would still be reported as "the compiler
    // unexpectedly panicked, this is a bug" with an ICE dump, even though rustc has already
    // finished. Putting the default back is what makes our failures read as ours.
    std::panic::set_hook(Box::new(|info| eprintln!("{info}")));

    // Outside rustc's panic-catching region, with our own hook: a failure here is ours.
    check(&callbacks);
}

fn build_dep_rlib(fixture_dir: &Path, out_dir: &Path) {
    let rustc = std::env::var("RUSTC").unwrap_or_else(|_| "rustc".to_string());
    let status = Command::new(rustc)
        .arg(fixture_dir.join("mycrate.rs"))
        .args(["--crate-type=lib", "--crate-name=mycrate", "--edition=2021"])
        .arg("--out-dir")
        .arg(out_dir)
        .status()
        .expect("could not run rustc to build the dependency rlib");
    assert!(status.success(), "building the dependency rlib failed");
}
