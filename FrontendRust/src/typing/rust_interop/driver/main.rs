// valec-rs — the rustc-hosted Vale driver.
//
// rustc cannot be called as a library that hands back a `TyCtxt`: the type exists only inside
// `rustc_driver::run_compiler`'s callback, and its `'tcx` is tied to arenas owned by that
// stack frame. So the control flow inverts — **rustc hosts, and Vale's typing pass runs
// inside `Callbacks::after_expansion`**. That is the architecture doc's §20.3 shape, so this
// binary is a miniature of the real thing rather than a detour.
//
// **This binary carries no assertions.** It used to: hosting rustc was believed to require a
// binary, because `run_compiler` installs a process-global panic hook and its fatal paths exit
// rather than return, so a `#[test]` looked like a way to lose the whole suite. Measurement
// (arch §26b.2) says otherwise — a fatal rustc error costs exactly one test — so the interop
// corpus lives in `typing/test/rust_interop/` where it can also reach `collect_*` and assert on
// the typed AST. What is left here is a compiler: it compiles, it reports, it exits.
//
// It stays because it is the seed of the real `valec-rs` (arch §3.2), not because anything
// tests it. The next step for it is taking the Vale source from argv rather than holding a
// built-in program.

#![feature(rustc_private)]

extern crate rustc_driver;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_middle;
extern crate rustc_session;
extern crate rustc_span;

use std::path::{Path, PathBuf};
use std::process::{exit, Command};
use std::sync::Arc;

use bumpalo::Bump;
use rustc_driver::{Callbacks, Compilation};
use rustc_interface::interface::Compiler;
use rustc_middle::ty::TyCtxt;

use frontend_rust::code_source::CodeSource;
use frontend_rust::compile_options::GlobalOptions;
use frontend_rust::keywords::Keywords;
use frontend_rust::parse_arena::ParseArena;
use frontend_rust::scout_arena::ScoutArena;
use frontend_rust::tests::tests::new_test_code_map;
use frontend_rust::typing::compilation::{TypingPassCompilation, TypingPassOptions};
use frontend_rust::typing::oracles::Oracles;
use frontend_rust::typing::rust_interop::{LoggingOracle, TyCtxtOracle};
use frontend_rust::typing::typing_interner::TypingInterner;

/// The Rust items this driver's built-in program is allowed to see.
///
/// Scoping is membership in this list — the same mechanism an `import rust.X.Y` will populate
/// later, from a different source.
const ALLOWED: &[&str] = &["add_two_numbers", "make_counter", "Counter", "pick", "first"];

/// Runs inside rustc, with a real `TyCtxt` in hand.
///
/// Records only. The outcome has to be pulled out here because everything the typing pass
/// borrows — the arenas, the oracle, the log — lives in this frame and dies with it, nested
/// inside `'tcx`.
#[derive(Default)]
struct ValeCallbacks {
    outcome: Option<Result<Vec<String>, String>>,
}

impl Callbacks for ValeCallbacks {
    fn after_expansion<'tcx>(&mut self, _compiler: &Compiler, tcx: TyCtxt<'tcx>) -> Compilation {
        // `after_expansion` rather than `after_analysis`: every crate is loaded and signatures,
        // ADT defs and module children are all queryable here, but rustc has not yet
        // typechecked bodies — which is everything a read-only typing pass needs and nothing
        // it doesn't.
        self.outcome = Some(compile_vale(tcx, ALLOWED));
        Compilation::Stop
    }
}

/// Runs Vale's typing pass over a program that uses Rust items, with `allowed` naming the
/// importable Rust paths. Returns the oracle log on success.
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
    // nesting is sound and the reverse would not be. Nothing built here survives the callback,
    // which is the containment property the design depends on.
    //
    // The program exercises the whole seam at once — a generic call, a free function, a Rust
    // type reaching Vale by inference from a signature, a method, and a value that needs a
    // scope-end drop. The corpus splits these apart so a failure localizes; here they are
    // together on purpose, because this is a smoke run rather than a test.
    let code = r"
exported func main() int {
  x = pick<int, bool>(add_two_numbers(3, 4), true);
  c = make_counter();
  return (make_counter()).get();
}";
    let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);

    // No package coordinate is handed in: each item derives its own from `tcx.def_path`.
    let real = TyCtxtOracle::new(tcx, &scout_arena, allowed);
    // Tag the log with the crate this rustc invocation is compiling. Constant today because
    // only one invocation runs the typing pass; it stops being constant the moment a second
    // compile contributes entries.
    let compiling = tcx.crate_name(rustc_span::def_id::LOCAL_CRATE).to_string();
    let logging = LoggingOracle::new(&real, &compiling);

    let mut global_options = GlobalOptions::apply();
    global_options.sanity_check = true;
    let options = TypingPassOptions {
        global_options,
        debug_out: Arc::new(|_: &str| {}),
        tree_shaking_enabled: true,
    };
    // The package the source lives in. An empty list here compiles nothing at all and still
    // returns `Ok` — which is the silent-success the oracle log exists to catch, and did.
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
        Ok(_) => Ok(logging.calls().into_iter().map(|c| c.rendered).collect()),
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

    // Appropriate here, unlike in the test corpus: this process is rustc's, so an ICE really
    // is rustc breaking and should say so. It matters more rather than less once we start
    // overriding queries and genuine ICEs become possible.
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
    let rustc_exit = rustc_driver::catch_with_exit_code(|| {
        rustc_driver::run_compiler(&rustc_args, &mut callbacks);
    });
    if rustc_exit != 0 {
        // rustc itself failed. Nothing of ours to report.
        exit(rustc_exit);
    }

    match callbacks.outcome {
        None => {
            eprintln!("valec-rs: rustc returned without ever reaching after_expansion");
            exit(1);
        }
        Some(Err(e)) => {
            eprintln!("valec-rs: the Vale program failed to typecheck: {e}");
            exit(1);
        }
        Some(Ok(log)) => {
            println!("OK: typechecked against a real TyCtxt");
            println!("--- oracle log ---");
            for line in log {
                println!("{line}");
            }
        }
    }
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
