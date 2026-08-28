// Tests for the `valen` orchestrator's dark box (@DBAPIZ): `generate_workspace` is pure — it turns a
// parsed `Valen.toml` (`Manifest`) into the cargo-workspace file set with no cargo and no filesystem, so
// every "this manifest → these files" case is a fast, deterministic unit test. `run_build` (which writes
// those files, copies `src/`, and runs cargo) is covered black-box by the `valen build` tracer.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use tempfile::TempDir;

use crate::typing::rust_interop::orchestrator::{
  generate_workspace, stage_workspace, BinTarget, BuildInputs, DepSpec, GeneratedWorkspace, Manifest,
  Project,
};

const PROG_VALEN: &str = "import rust.tiny.seven; exported func main() int { return seven(); }";

// Slice 1: a one-binary, one-rust-path-dep manifest generates a Cargo.toml (package, edition 2021, the
// dep, a `[[bin]]` pointing at the `.valen`) and a rustc-fork `rust-toolchain.toml`.
#[test]
fn generate_workspace_emits_expected_manifests() {
  let manifest = Manifest {
    project: Project {
      name: "prog".to_string(),
      version: "0.1.0".to_string(),
      edition: "experimental".to_string(),
    },
    rust_dependencies: BTreeMap::from([(
      "tiny".to_string(),
      DepSpec::Path { path: "/abs/tiny".to_string() },
    )]),
    valen_dependencies: BTreeMap::new(),
    bins: vec![BinTarget { name: "prog".to_string(), source: "src/main.valen".to_string() }],
  };

  let workspace = generate_workspace(&manifest);

  let cargo = workspace_file(&workspace, "Cargo.toml");
  assert!(cargo.contains("name = \"prog\""), "Cargo.toml:\n{cargo}");
  assert!(cargo.contains("version = \"0.1.0\""), "Cargo.toml:\n{cargo}");
  assert!(cargo.contains("edition = \"2021\""), "Cargo.toml:\n{cargo}");
  assert!(cargo.contains("tiny = { path = \"/abs/tiny\" }"), "Cargo.toml:\n{cargo}");
  assert!(cargo.contains("[[bin]]"), "Cargo.toml:\n{cargo}");
  assert!(cargo.contains("path = \"src/main.valen\""), "Cargo.toml:\n{cargo}");

  let toolchain = workspace_file(&workspace, "rust-toolchain.toml");
  assert!(toolchain.contains("channel = \"rustc-fork\""), "rust-toolchain.toml:\n{toolchain}");
}

// Look up one generated file's contents by its relative path (panics if absent — the one non-panicking
// branch is the found file, per @NCTOBPAOPX).
fn workspace_file<'w>(workspace: &'w GeneratedWorkspace, rel: &str) -> &'w str {
  workspace
    .files
    .iter()
    .find(|(path, _)| path == Path::new(rel))
    .map(|(_, contents)| contents.as_str())
    .unwrap_or_else(|| panic!("workspace has no {rel}"))
}

// Slice 2: `run_build`'s staging (its effectful half, up to the cargo spawn) reads a `Valen.toml`,
// generates the workspace, writes it, and copies the project `src/` into the build dir. The cargo→7
// spawn is a manual e2e — a `--lib` test cannot obtain the real `valenc-rs` binary. Hermetic (@TMBFIZ).
#[test]
fn run_build_stages_the_workspace() {
  let root = TempDir::new().expect("could not create root dir");
  let project = root.path().join("prog");
  fs::create_dir_all(project.join("src")).expect("mkdir project/src");
  fs::write(
    project.join("Valen.toml"),
    "[project]\nname = \"prog\"\nversion = \"0.1.0\"\nedition = \"experimental\"\n\n[rust-dependencies]\ntiny = { path = \"/abs/tiny\" }\n\n[[bin]]\nname = \"prog\"\nsource = \"src/main.valen\"\n",
  )
  .expect("write Valen.toml");
  fs::write(project.join("src/main.valen"), PROG_VALEN).expect("write src/main.valen");

  let build_dir = root.path().join("build");
  stage_workspace(&BuildInputs {
    manifest_path: project.join("Valen.toml"),
    build_dir: build_dir.clone(),
    valenc_rs: PathBuf::from("/unused/valenc-rs"),
  })
  .expect("stage_workspace should succeed");

  let cargo = fs::read_to_string(build_dir.join("Cargo.toml")).expect("Cargo.toml written");
  assert!(cargo.contains("name = \"prog\""), "Cargo.toml:\n{cargo}");
  assert!(cargo.contains("tiny = { path = \"/abs/tiny\" }"), "Cargo.toml:\n{cargo}");
  assert!(cargo.contains("path = \"src/main.valen\""), "Cargo.toml:\n{cargo}");

  let toolchain =
    fs::read_to_string(build_dir.join("rust-toolchain.toml")).expect("toolchain written");
  assert!(toolchain.contains("channel = \"rustc-fork\""), "toolchain:\n{toolchain}");

  let copied = fs::read_to_string(build_dir.join("src/main.valen")).expect("src copied");
  assert_eq!(copied, PROG_VALEN);
}
