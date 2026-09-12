// Tier-3 end-to-end test for the real `valen build` pipeline under `--release`: it generates a whole
// cargo project in a `TempDir` (the library `stage_workspace` dark-box, @TMBFIZ) and drives a real
// `cargo build --release` over it through the `valenc-rs` wrapper, the way a user building in release
// would. Unlike the corpus (in-process rustc) and `drive_tests.rs` (in-process `run_wrapper`), this
// exercises the actual orchestrator and real dead-code stripping — the in-process paths pass
// `-Clink-dead-code`, so only an honest `cargo build` can observe a stripped-leaf link failure.
//
// It needs the `valenc-rs` binary built (`cargo +rustc-fork build --features rust_interop --bin
// valenc-rs --bin valen`), which the plain interop `--lib` gate does not build, so it SOFT-SKIPS when
// the binary is absent; a dedicated fire-commit command builds the bin first, then runs this module.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use tempfile::TempDir;

use crate::typing::rust_interop::orchestrator::{stage_workspace, BuildInputs};

/// Locate the `valenc-rs` binary beside the test executable (`<target>/debug/valenc-rs`) — the same
/// sibling-in-the-target-dir layout `valen build` assumes. `None` when it hasn't been built.
fn valenc_rs_bin() -> Option<PathBuf> {
  let exe = std::env::current_exe().ok()?;
  // <target>/debug/deps/<testbin>  ->  <target>/debug
  let target_debug = exe.parent()?.parent()?;
  let bin = target_debug.join("valenc-rs");
  bin.exists().then_some(bin)
}

/// Soft-skip a test when the `valenc-rs` binary isn't built. Yields the binary path, or returns.
macro_rules! require_valenc_rs {
  () => {
    match valenc_rs_bin() {
      Some(bin) => bin,
      None => {
        eprintln!(
          "pipeline_e2e skip: valenc-rs not built. Build it first: cargo +rustc-fork build \
           --features rust_interop --bin valenc-rs --bin valen"
        );
        return;
      }
    }
  };
}

/// Recursively copy a checked-in project template into `to`, skipping any `target` dir (build output).
/// Staging into a fresh tempdir keeps the run isolated (@TMBFIZ) — the checked-in project is never
/// mutated by the build.
fn copy_tree_skipping_target(from: &Path, to: &Path) {
  fs::create_dir_all(to).unwrap();
  for entry in fs::read_dir(from).unwrap().flatten() {
    if entry.file_name() == "target" {
      continue;
    }
    let src = entry.path();
    let dst = to.join(entry.file_name());
    if entry.file_type().unwrap().is_dir() {
      copy_tree_skipping_target(&src, &dst);
    } else {
      fs::copy(&src, &dst).unwrap();
    }
  }
}

/// Generate the whole cargo project for `project` (via the library `stage_workspace`), then drive a
/// real `cargo build --release` over it with `valenc-rs` installed as `RUSTC_WORKSPACE_WRAPPER` — the
/// release counterpart of `run_build`'s debug spawn. Mirrors `run_build`'s env (the fork toolchain
/// comes from the generated `rust-toolchain.toml`, a leaked `RUSTC` is cleared), adding `--release`.
/// Returns cargo's exit code and the produced binary's path (`<build_dir>/target/release/<bin_name>`).
fn valen_build_release(project: &Path, valenc_rs: &Path, bin_name: &str) -> (i32, PathBuf) {
  let build_dir = project.join("target").join("valen-build");
  stage_workspace(&BuildInputs {
    manifest_path: project.join("Valen.toml"),
    build_dir: build_dir.clone(),
    valenc_rs: valenc_rs.to_path_buf(),
  })
  .expect("stage_workspace should succeed");
  let status = Command::new("cargo")
    .current_dir(&build_dir)
    .args(["build", "--release"])
    .env("RUSTC_WORKSPACE_WRAPPER", valenc_rs)
    .env_remove("RUSTC")
    .status()
    .expect("could not spawn cargo");
  let exe = build_dir.join("target").join("release").join(bin_name);
  (status.code().unwrap_or(1), exe)
}

// A `--release` `valen build` of the vendored `driver-check` project must link and produce a binary,
// the same as the debug build does. Its stubbed `nobiliav` bodies are `unimplemented!()`, so the
// honest assertion is a clean build + link, not a run-to-exit.
#[test]
fn release_build_of_driver_check_links() {
  let valenc_rs = require_valenc_rs!();
  let template =
    Path::new(env!("CARGO_MANIFEST_DIR")).join("src/typing/test/rust_interop/driver-check");
  let project = TempDir::new().unwrap();
  copy_tree_skipping_target(&template, project.path());
  let (build_rc, exe) = valen_build_release(project.path(), &valenc_rs, "driver_check");
  assert_eq!(build_rc, 0, "release valen build of driver-check should link");
  assert!(exe.exists(), "the release driver_check binary should be produced");
}
