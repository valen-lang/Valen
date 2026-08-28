// Tests for the interim `valec-rs drive` bridge: the `drive_and_link` dark-box function, the
// `vale-stub-gen` seed (`generate_stub_source`), and the CLI's `run_drive` (parsed args → exit code).
// Unlike the corpus, these do not run a `Case` against a fixture stub on disk — they exercise a
// *generated* stub against a *caller-supplied* rlib, which is exactly what the `drive` CLI does with the
// rlibs Pearl builds via `cargo +rustc-fork build`.
//
// The CLI is tested at its dark-box boundary (@DBAPIZ): `run_drive(&DriveArgs)` is what `main()` calls
// after clap parses argv, so parsing `DriveArgs::parse_from(...)` and asserting the returned exit code
// covers the whole CLI path without spawning the process (which would force building the standalone
// `valec` bin against the interop lib). Each test builds everything in its own `TempDir` (@TMBFIZ).

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use clap::Parser;
use tempfile::TempDir;

use crate::typing::rust_interop::drive::{
  default_sysroot, drive_and_link, run_drive, DriveArgs, DriveInputs, ExternArg,
};
use crate::typing::rust_interop::stub_gen::generate_stub_source_from_vale;

use super::harness::build_dep_rlib;

// The one Vale program the tracer slice drives: import a Rust free function and return its value.
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

// The generator emits the load-bearing stub shape from the scouted program: one `pub use` per import
// (@RTMEIZ), the marker, a `#[vale::emit_consumer_body]` root per exported func, and the bin shim.
#[test]
fn stub_gen_emits_pub_use_and_consumer_body_from_scouted_program() {
  let stub = generate_stub_source_from_vale(TINY_VALE).expect("stub generation should succeed");
  assert!(stub.contains("extern crate tiny;"), "stub:\n{stub}");
  assert!(stub.contains("pub use tiny::seven;"), "stub:\n{stub}");
  assert!(stub.contains("#[vale::emit_consumer_body]"), "stub:\n{stub}");
  assert!(stub.contains("pub fn __vale_main() -> i32"), "stub:\n{stub}");
  assert!(stub.contains("__VALE_STUBS_MARKER"), "stub:\n{stub}");
  assert!(stub.contains("fn main()"), "stub:\n{stub}");
}

// The tracer: a Valen program passes through a caller-supplied std-only rlib and the produced bin
// returns 7. Proves the whole `drive_and_link` path — generate stub, drive rustc to a linked bin,
// run it, forward the exit code.
#[test]
fn drive_and_link_runs_a_std_only_program_returning_seven() {
  let out = TempDir::new().expect("could not create scratch dir");
  let out_dir = out.path();

  // The rlib Pearl would build with cargo; here a bare std-only crate built the canonical way.
  let tiny_rs = out_dir.join("tiny.rs");
  fs::write(&tiny_rs, "pub fn seven() -> i32 { 7 }\n").expect("could not write tiny.rs");
  build_dep_rlib("tiny", &tiny_rs, out_dir);
  let rlib = out_dir.join("libtiny.rlib");

  let result = drive_and_link(&DriveInputs {
    vale_source: TINY_VALE.to_string(),
    externs: vec![ExternArg { name: "tiny".to_string(), rlib: Some(rlib) }],
    dependency_dirs: vec![],
    sysroot: default_sysroot(),
    out_dir: out_dir.to_path_buf(),
  })
  .expect("drive_and_link should succeed");

  assert_eq!(result.process_exit, Some(7), "firings: {:?}", result.firings);
}

// Valen operators (`+`, `==`, …) are library functions in src/builtins/resources/arith.vale, so they
// resolve only when the builtins package is compiled in. This drives `+` and `==` through an `if` — the
// shape NobiliaV's windowed driver needs (compare key codes, guard clicks), with no rust import — to
// prove `drive` links the builtins and the `__vbi_*` intrinsics lower through the interop backend.
#[test]
fn drive_and_link_runs_a_program_using_int_operators() {
  let out = TempDir::new().expect("could not create scratch dir");
  let result = drive_and_link(&DriveInputs {
    vale_source: "exported func main() int { sum = 3 + 4; if sum == 7 { 7 } else { 0 } }"
      .to_string(),
    externs: vec![],
    dependency_dirs: vec![],
    sysroot: default_sysroot(),
    out_dir: out.path().to_path_buf(),
  })
  .expect("drive_and_link should succeed");

  assert_eq!(result.process_exit, Some(7), "firings: {:?}", result.firings);
}

// `!=` on ints — the one operator NobiliaV's driver needs that `==` didn't cover. `x != y` bare-uses
// the locals, so it hits the generic `!=<T>`'s `&T` params (the shape Pearl's `input.mouse_x() != -1`
// produced). Guards that the generic `!=<T>` is a builtin (it lived only in the stdlib before).
#[test]
fn drive_and_link_supports_int_not_equal() {
  let out = TempDir::new().expect("could not create scratch dir");
  let result = drive_and_link(&DriveInputs {
    vale_source: "exported func main() int { x = 3; y = 4; if x != y { 7 } else { 0 } }".to_string(),
    externs: vec![],
    dependency_dirs: vec![],
    sysroot: default_sysroot(),
    out_dir: out.path().to_path_buf(),
  })
  .expect("drive_and_link should succeed");

  assert_eq!(result.process_exit, Some(7), "firings: {:?}", result.firings);
}

// Pearl's real scenario at the `drive_and_link` boundary: a Valen program imports an item the named
// crate only *re-exports* (`greeter` does `pub use helper::seven`), the canonical crate (`helper`) being
// a separate dependency. Linked with only a bare `--extern greeter` + `-L dependency=<deps>`. Proves
// bare-name `--extern` resolution, the generated `pub use` re-export, and that the canonical (`helper`)
// symbol resolves from `-L dependency` alone at link — i.e. one `--extern`, not three.
#[test]
fn drive_and_link_links_a_cargo_crate_through_a_pub_use_re_export() {
  let build = TempDir::new().expect("could not create build dir");
  let deps_dir = build_reexport_crate(build.path());

  let out = TempDir::new().expect("could not create scratch dir");
  let result = drive_and_link(&DriveInputs {
    vale_source: REEXPORT_VALE.to_string(),
    externs: vec![ExternArg { name: "greeter".to_string(), rlib: None }],
    dependency_dirs: vec![deps_dir],
    sysroot: default_sysroot(),
    out_dir: out.path().to_path_buf(),
  })
  .expect("drive_and_link should succeed");

  assert_eq!(result.process_exit, Some(7), "firings: {:?}", result.firings);
}

// The CLI dark box: parse Pearl's exact `drive` arguments and run them. `run_drive` is what `main()`
// calls after clap, so this covers argument parsing (`--extern` bare name, `-L dependency=<dir>`,
// `--out-dir`) plus the whole drive, returning the produced binary's exit code.
#[test]
fn run_drive_parses_the_cli_arguments_and_returns_seven() {
  let build = TempDir::new().expect("could not create build dir");
  let deps_dir = build_reexport_crate(build.path());

  let prog = build.path().join("prog.vale");
  fs::write(&prog, REEXPORT_VALE).expect("write prog.vale");
  let out = TempDir::new().expect("could not create scratch dir");

  let args = DriveArgs::parse_from([
    "valec-rs".to_string(),
    prog.display().to_string(),
    "--extern".to_string(),
    "greeter".to_string(),
    "-L".to_string(),
    format!("dependency={}", deps_dir.display()),
    "--out-dir".to_string(),
    out.path().display().to_string(),
  ]);

  assert_eq!(run_drive(&args), 7);
}
