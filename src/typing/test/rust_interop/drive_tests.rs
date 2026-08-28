// Tests for the `valenc-rs` wrapper's engine (@DBAPIZ) and the `vale-stub-gen` seed
// (`generate_stub_source`). Unlike the corpus, these do not run a `Case` against a fixture stub on disk —
// they exercise a *generated* stub against a *caller-supplied* rlib, the shape cargo drives (handing the
// wrapper `--extern`/`-L dependency` per crate). Each test builds everything in its own `TempDir` (@TMBFIZ).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

use crate::typing::rust_interop::drive::{default_sysroot, run_wrapper, WrapperInputs};
use crate::typing::rust_interop::stub_gen::generate_stub_source_from_vale;

use super::harness::build_dep_rlib;

// The one Vale program the tracer drives: import a Rust free function and return its value.
const TINY_VALE: &str = "import rust.tiny.seven; exported func main() int { return seven(); }";

// A Vale program that imports through a re-export: `greeter` only `pub use`s `helper::seven`.
const REEXPORT_VALE: &str =
  "import rust.greeter.seven; exported func main() int { return seven(); }";

// Build, with cargo on the fork toolchain, a `greeter` crate that re-exports `helper::seven` (helper's
// `seven` is the canonical item; greeter names it only via `pub use`). Returns the `deps` dir holding
// both hashed rlibs. `--target-dir` is explicit because this environment sets a shared cargo target dir.
// Hermetic: path dependency only, no network.
fn build_reexport_crate(root: &Path) -> PathBuf {
  let helper = root.join("helper");
  fs::create_dir_all(helper.join("src")).expect("mkdir helper/src");
  fs::write(
    helper.join("Cargo.toml"),
    "[package]\nname = \"helper\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[lib]\ncrate-type = [\"rlib\"]\n",
  )
  .expect("write helper Cargo.toml");
  fs::write(helper.join("src/lib.rs"), "pub fn seven() -> i32 { 7 }\n").expect("write helper lib.rs");

  let greeter = root.join("greeter");
  fs::create_dir_all(greeter.join("src")).expect("mkdir greeter/src");
  fs::write(
    greeter.join("Cargo.toml"),
    "[package]\nname = \"greeter\"\nversion = \"0.0.0\"\nedition = \"2021\"\n\n[lib]\ncrate-type = [\"rlib\"]\n\n[dependencies]\nhelper = { path = \"../helper\" }\n",
  )
  .expect("write greeter Cargo.toml");
  fs::write(greeter.join("src/lib.rs"), "pub use helper::seven;\n").expect("write greeter lib.rs");
  fs::write(greeter.join("rust-toolchain.toml"), "[toolchain]\nchannel = \"rustc-fork\"\n")
    .expect("write rust-toolchain.toml");

  let target_dir = root.join("target");
  let sysroot = default_sysroot();
  let build_out = Command::new("cargo")
    .current_dir(&greeter)
    .arg("build")
    .arg("--offline")
    .arg("--target-dir")
    .arg(&target_dir)
    .env("RUSTUP_TOOLCHAIN", "rustc-fork")
    .env("DYLD_LIBRARY_PATH", format!("{sysroot}/lib"))
    .env_remove("RUSTC")
    .output()
    .expect("could not spawn cargo build");
  assert!(
    build_out.status.success(),
    "cargo build (fork) failed:\nstdout:\n{}\nstderr:\n{}",
    String::from_utf8_lossy(&build_out.stdout),
    String::from_utf8_lossy(&build_out.stderr),
  );
  target_dir.join("debug/deps")
}

// Drive a `.valen` binary crate through the wrapper the way cargo would — write it, build a
// `--crate-type=bin` arg set (plus `extra` flags like `--extern`/`-L` for imports), assert it drove
// Valen and rustc exited 0, then run the produced binary and return its exit code.
fn wrapper_run_binary(out_dir: &Path, vale_source: &str, extra: Vec<String>) -> i32 {
  let valen = out_dir.join("prog.valen");
  fs::write(&valen, vale_source).expect("could not write prog.valen");
  let mut rustc_args = vec![
    "valenc-rs".to_string(),
    valen.display().to_string(),
    "--crate-type=bin".to_string(),
    "--crate-name=prog".to_string(),
    "--edition=2021".to_string(),
    format!("--sysroot={}", default_sysroot()),
    format!("--out-dir={}", out_dir.display()),
    // Root every local item so the collector walks the (otherwise-uncalled) `__vale_*` stub fns.
    "-Clink-dead-code".to_string(),
  ];
  rustc_args.extend(extra);

  let result = run_wrapper(&WrapperInputs { rustc_args }).expect("run_wrapper should succeed");
  assert_eq!(result.rustc_exit, 0, "firings: {:?}", result.firings);
  assert!(result.drove_valen, "a .valen crate must drive the Valen engine");

  let exe = out_dir.join("prog");
  let output = Command::new(&exe).output().expect("could not run the produced binary");
  output.status.code().unwrap_or(-1)
}

// Find cargo's content-hashed `lib<name>-<hash>.rlib` in a deps dir — the explicit path cargo hands the
// wrapper as `--extern <name>=<path>`. (The interim `drive` CLI resolved this itself; the wrapper does
// not, since real cargo always supplies the path.)
fn find_rlib(deps_dir: &Path, name: &str) -> PathBuf {
  let prefix = format!("lib{name}-");
  fs::read_dir(deps_dir)
    .expect("could not read the deps dir")
    .flatten()
    .map(|entry| entry.path())
    .find(|path| {
      path
        .file_name()
        .and_then(|f| f.to_str())
        .is_some_and(|f| f.starts_with(&prefix) && f.ends_with(".rlib"))
    })
    .unwrap_or_else(|| panic!("no lib{name}-*.rlib in {}", deps_dir.display()))
}

// The generator emits the load-bearing stub shape from the parsed program: one `pub use` per import
// (@RTMEIZ), the marker, a `#[vale::emit_consumer_body]` root per exported func, and the bin shim.
#[test]
fn stub_gen_emits_pub_use_and_consumer_body_from_parsed_program() {
  let stub = generate_stub_source_from_vale(TINY_VALE).expect("stub generation should succeed");
  assert!(stub.contains("extern crate tiny;"), "stub:\n{stub}");
  assert!(stub.contains("pub use tiny::seven;"), "stub:\n{stub}");
  assert!(stub.contains("#[vale::emit_consumer_body]"), "stub:\n{stub}");
  assert!(stub.contains("pub fn __vale_main() -> i32"), "stub:\n{stub}");
  assert!(stub.contains("__VALE_STUBS_MARKER"), "stub:\n{stub}");
  assert!(stub.contains("fn main()"), "stub:\n{stub}");
}

// The pure-Rust passthrough (@PRCCBIVRZ). A crate whose root is `.rs` is not a Valen crate, so
// `run_wrapper`'s extension dispatch takes the passthrough branch: it installs no query overrides and no
// fill_extra_modules hook (`drove_valen == false`, no firings), compiling exactly as vanilla rustc would.
// Here that produces `libplain.rlib` and exits 0.
#[test]
fn wrapper_passes_through_a_pure_rust_crate() {
  let out = TempDir::new().expect("could not create scratch dir");
  let out_dir = out.path();
  let rs = out_dir.join("plain.rs");
  fs::write(&rs, "pub fn f() -> i32 { 3 }\n").expect("write plain.rs");

  let result = run_wrapper(&WrapperInputs {
    rustc_args: vec![
      "valenc-rs".to_string(),
      rs.display().to_string(),
      "--crate-type=lib".to_string(),
      "--crate-name=plain".to_string(),
      "--edition=2021".to_string(),
      format!("--sysroot={}", default_sysroot()),
      format!("--out-dir={}", out_dir.display()),
    ],
  })
  .expect("run_wrapper should succeed");

  assert_eq!(result.rustc_exit, 0, "firings: {:?}", result.firings);
  assert!(!result.drove_valen, "a .rs crate must take the passthrough branch");
  assert!(result.firings.is_empty(), "the passthrough installs no Valen machinery");
}

// The tracer: a `.valen` binary that imports a caller-supplied std-only rlib links and runs → 7. The
// wrapper generates the pass-1 stub, substitutes it for the `.valen`, drives rustc to a linked bin, and
// (via the helper) the bin is run to check the forwarded exit code.
#[test]
fn wrapper_drives_a_valen_binary_to_exit_seven() {
  let out = TempDir::new().expect("could not create scratch dir");
  let out_dir = out.path();

  // The rlib the program imports (a bare std-only crate built the canonical way, as Pearl's cargo would).
  let tiny_rs = out_dir.join("tiny.rs");
  fs::write(&tiny_rs, "pub fn seven() -> i32 { 7 }\n").expect("could not write tiny.rs");
  build_dep_rlib("tiny", &tiny_rs, out_dir);
  let rlib = out_dir.join("libtiny.rlib");

  let exit = wrapper_run_binary(
    out_dir,
    TINY_VALE,
    vec![format!("--extern=tiny={}", rlib.display()), format!("-L{}", out_dir.display())],
  );
  assert_eq!(exit, 7);
}

// Valen operators (`+`, `==`, …) are library functions (src/builtins/resources/arith.vale), resolved only
// when the builtins package is compiled in. Driving `+` and `==` through an `if` — the shape NobiliaV's
// driver needs — proves the wrapper links the builtins and the `__vbi_*` intrinsics lower through the
// interop backend, with no rust import.
#[test]
fn wrapper_drives_int_operators_to_exit_seven() {
  let out = TempDir::new().expect("could not create scratch dir");
  let exit = wrapper_run_binary(
    out.path(),
    "exported func main() int { sum = 3 + 4; if sum == 7 { 7 } else { 0 } }",
    vec![],
  );
  assert_eq!(exit, 7);
}

// `!=` on ints — the one operator NobiliaV's driver needs that `==` didn't cover. `x != y` bare-uses the
// locals, hitting the generic `!=<T>`'s `&T` params. Guards that `!=<T>` is a builtin (it lived only in
// the stdlib before).
#[test]
fn wrapper_drives_int_not_equal_to_exit_seven() {
  let out = TempDir::new().expect("could not create scratch dir");
  let exit = wrapper_run_binary(
    out.path(),
    "exported func main() int { x = 3; y = 4; if x != y { 7 } else { 0 } }",
    vec![],
  );
  assert_eq!(exit, 7);
}

// Pearl's real scenario through the wrapper: a Valen program imports an item the named crate only
// *re-exports* (`greeter` does `pub use helper::seven`), the canonical crate (`helper`) being a separate
// dependency. Linked with the explicit `--extern greeter=<rlib>` cargo hands the wrapper + `-L
// dependency=<deps>`. Proves the generated `pub use` re-export and that the canonical (`helper`) symbol
// resolves from `-L dependency` alone — i.e. one `--extern`, not three.
#[test]
fn wrapper_links_a_cargo_crate_through_a_pub_use_re_export() {
  let build = TempDir::new().expect("could not create build dir");
  let deps_dir = build_reexport_crate(build.path());
  let greeter_rlib = find_rlib(&deps_dir, "greeter");

  let out = TempDir::new().expect("could not create scratch dir");
  let exit = wrapper_run_binary(
    out.path(),
    REEXPORT_VALE,
    vec![
      format!("--extern=greeter={}", greeter_rlib.display()),
      format!("-Ldependency={}", deps_dir.display()),
    ],
  );
  assert_eq!(exit, 7);
}
