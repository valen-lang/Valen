// valen — the cargo-like orchestrator for interop builds.
//
// `valen build [--manifest-path <Valen.toml>]` generates a Cargo workspace from the `Valen.toml` and
// runs `cargo build` over it with `valenc-rs` installed as `RUSTC_WORKSPACE_WRAPPER`. main() is the thin
// shell over `run_build` (@DBAPIZ): it only gathers the manifest path, the build dir, and the sibling
// `valenc-rs` binary, then calls `run_build`.
//
// `#![feature(rustc_private)]` is required only because this bin links the interop-enabled library
// (which pulls in rustc's private crates); valen itself never touches rustc — it shells out to cargo.

#![feature(rustc_private)]

use std::env;
use std::path::PathBuf;
use std::process::exit;

use frontend_rust::typing::rust_interop::orchestrator::{run_build, BuildInputs};

fn main() {
  let argv: Vec<String> = env::args().collect();
  if argv.get(1).map(String::as_str) != Some("build") {
    eprintln!("usage: valen build [--manifest-path <Valen.toml>]");
    exit(2);
  }

  let mut manifest_path = PathBuf::from("Valen.toml");
  let mut rest = argv.iter().skip(2);
  while let Some(arg) = rest.next() {
    match arg.as_str() {
      "--manifest-path" => manifest_path = PathBuf::from(rest.next().cloned().unwrap_or_default()),
      other => {
        eprintln!("valen: unknown argument {other}");
        exit(2);
      }
    }
  }

  let project_dir =
    manifest_path.parent().map(|p| p.to_path_buf()).unwrap_or_else(|| PathBuf::from("."));
  let build_dir = project_dir.join("target").join("valen-build");
  // valenc-rs sits beside valen in the same target dir.
  let valenc_rs = env::current_exe()
    .ok()
    .and_then(|exe| exe.parent().map(|dir| dir.join("valenc-rs")))
    .unwrap_or_else(|| PathBuf::from("valenc-rs"));

  match run_build(&BuildInputs { manifest_path, build_dir, valenc_rs }) {
    Ok(code) => exit(code),
    Err(e) => {
      eprintln!("valen: {e}");
      exit(1);
    }
  }
}
