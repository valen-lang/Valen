// The `valen` orchestrator — the cargo-like front for interop builds. `valen build` parses a
// `Valen.toml`, generates a Cargo workspace, and runs `RUSTC_WORKSPACE_WRAPPER=valenc-rs cargo
// +rustc-fork build` over it (design §257-303). cargo then invokes `valenc-rs` once per crate; a
// `.valen` crate drives Valen, a pure-Rust dependency passes through to plain rustc.
//
// `generate_workspace` is the pure dark box (@DBAPIZ): a parsed manifest in, the workspace file set out,
// no cargo and no filesystem. `run_build` (a later slice) is the thin effectful shell — it writes those
// files, copies the project `src/`, and spawns cargo.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::Deserialize;

/// A parsed `Valen.toml`.
#[derive(Debug, Deserialize)]
pub struct Manifest {
  pub project: Project,
  #[serde(default, rename = "rust-dependencies")]
  pub rust_dependencies: BTreeMap<String, DepSpec>,
  #[serde(default, rename = "valen-dependencies")]
  pub valen_dependencies: BTreeMap<String, DepSpec>,
  #[serde(default, rename = "bin")]
  pub bins: Vec<BinTarget>,
}

#[derive(Debug, Deserialize)]
pub struct Project {
  pub name: String,
  pub version: String,
  pub edition: String,
}

/// A dependency's spec: either a crates.io version string or a local `{ path = ... }`.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum DepSpec {
  Version(String),
  Path { path: String },
}

#[derive(Debug, Deserialize)]
pub struct BinTarget {
  pub name: String,
  /// The crate root — a `.valen` file, relative to the project (e.g. `src/main.valen`).
  pub source: String,
}

/// The cargo files `generate_workspace` produces, as (path relative to the build dir, contents).
/// `run_build` writes these, then copies the project `src/` and runs cargo.
pub struct GeneratedWorkspace {
  pub files: Vec<(PathBuf, String)>,
}

/// Turn a parsed `Valen.toml` into the cargo files that build it — pure, no cargo and no filesystem.
///
/// Interim shape: a single flat package (not the design's multi-project workspace), enough for
/// NobiliaV's one-binary case. cargo's `[[bin]] path` points straight at the `.valen`: cargo accepts a
/// non-`.rs` target path, and `valenc-rs` turns it into the compiled stub. Rust deps render verbatim (a
/// path dep carries an absolute path); adjusting paths to a generated layout and the
/// `valen-dependencies` are the permanent form's job.
pub fn generate_workspace(manifest: &Manifest) -> GeneratedWorkspace {
  let mut cargo = String::new();
  cargo.push_str("[package]\n");
  cargo.push_str(&format!("name = \"{}\"\n", manifest.project.name));
  cargo.push_str(&format!("version = \"{}\"\n", manifest.project.version));
  cargo.push_str("edition = \"2021\"\n\n");
  cargo.push_str("[dependencies]\n");
  for (name, spec) in &manifest.rust_dependencies {
    cargo.push_str(&render_dep(name, spec));
  }
  for bin in &manifest.bins {
    cargo.push_str("\n[[bin]]\n");
    cargo.push_str(&format!("name = \"{}\"\n", bin.name));
    cargo.push_str(&format!("path = \"{}\"\n", bin.source));
  }

  let toolchain = "[toolchain]\nchannel = \"rustc-fork\"\n".to_string();

  GeneratedWorkspace {
    files: vec![
      (PathBuf::from("Cargo.toml"), cargo),
      (PathBuf::from("rust-toolchain.toml"), toolchain),
    ],
  }
}

/// Render one `[dependencies]` line: a crates.io version, or a local `{ path = ... }`.
fn render_dep(name: &str, spec: &DepSpec) -> String {
  match spec {
    DepSpec::Version(version) => format!("{name} = \"{version}\"\n"),
    DepSpec::Path { path } => format!("{name} = {{ path = \"{path}\" }}\n"),
  }
}

/// Everything `run_build` needs, gathered by the `valen` bin's `main()` above the boundary (@DBAPIZ).
pub struct BuildInputs {
  /// The project's `Valen.toml`.
  pub manifest_path: PathBuf,
  /// Where to generate the cargo workspace (e.g. `<project>/target/valen-build`).
  pub build_dir: PathBuf,
  /// The `valenc-rs` wrapper binary cargo uses as `RUSTC_WORKSPACE_WRAPPER`.
  pub valenc_rs: PathBuf,
}

/// Read the `Valen.toml`, generate the cargo workspace, write it under `build_dir`, and copy the
/// project's `src/` in. This is the effectful half of a build up to — but not including — the cargo
/// spawn, so a test can assert the staged tree without needing the `valenc-rs` binary or a toolchain.
pub fn stage_workspace(inputs: &BuildInputs) -> Result<(), String> {
  let manifest_text = fs::read_to_string(&inputs.manifest_path)
    .map_err(|e| format!("could not read {}: {e}", inputs.manifest_path.display()))?;
  let manifest: Manifest = toml::from_str(&manifest_text)
    .map_err(|e| format!("could not parse {}: {e}", inputs.manifest_path.display()))?;

  let workspace = generate_workspace(&manifest);
  for (rel, contents) in &workspace.files {
    let dest = inputs.build_dir.join(rel);
    if let Some(parent) = dest.parent() {
      fs::create_dir_all(parent)
        .map_err(|e| format!("could not create {}: {e}", parent.display()))?;
    }
    fs::write(&dest, contents).map_err(|e| format!("could not write {}: {e}", dest.display()))?;
  }

  // Copy the project's `src/` into the generated package so cargo's `[[bin]] path = "src/….valen"`
  // resolves; the `.valen` is the crate root, which `valenc-rs` turns into the compiled stub.
  let project_dir = inputs
    .manifest_path
    .parent()
    .ok_or_else(|| format!("{} has no parent dir", inputs.manifest_path.display()))?;
  copy_dir_recursive(&project_dir.join("src"), &inputs.build_dir.join("src"))
    .map_err(|e| format!("could not copy the project src/ into the build dir: {e}"))?;
  Ok(())
}

/// Stage the workspace, then run `cargo build` over it with `valenc-rs` installed as
/// `RUSTC_WORKSPACE_WRAPPER`, returning cargo's exit code. The generated `rust-toolchain.toml` selects
/// the fork, and `valenc-rs`'s baked rpath finds `librustc_driver`, so no toolchain/dylib env is set
/// here; a leaked `RUSTC` is cleared so it cannot override the wrapper. cargo invokes `valenc-rs` once
/// per crate — a `.valen` crate drives Valen, a pure-Rust dependency passes through.
pub fn run_build(inputs: &BuildInputs) -> Result<i32, String> {
  stage_workspace(inputs)?;
  let status = Command::new("cargo")
    .current_dir(&inputs.build_dir)
    .arg("build")
    .env("RUSTC_WORKSPACE_WRAPPER", &inputs.valenc_rs)
    .env_remove("RUSTC")
    .status()
    .map_err(|e| format!("could not spawn cargo: {e}"))?;
  Ok(status.code().unwrap_or(1))
}

/// Recursively copy `from` into `to` (files and subdirs), used to plant the project `src/` in the
/// generated package.
fn copy_dir_recursive(from: &Path, to: &Path) -> io::Result<()> {
  fs::create_dir_all(to)?;
  for entry in fs::read_dir(from)? {
    let entry = entry?;
    let source = entry.path();
    let dest = to.join(entry.file_name());
    if entry.file_type()?.is_dir() {
      copy_dir_recursive(&source, &dest)?;
    } else {
      fs::copy(&source, &dest)?;
    }
  }
  Ok(())
}
