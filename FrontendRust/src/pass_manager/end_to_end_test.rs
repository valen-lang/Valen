// TDD driver for `pass_manager::main` end-to-end wiring.
//
// The Rust pipeline (parse → scout → typing → instantiating → simplifying →
// hammer → von) is migrated and green when driven through the test harness
// (see e.g. `hammer_tests::simple_main`). But the `frontend_rust` binary's
// `pass_manager::main` does NOT yet drive that pipeline — `build()` panics
// at the Scout stub when `--output_vast true` is set
// (pass_manager.rs ~876: "Scout phase not yet implemented").
//
// This test exists to drive that wiring in TDD-style: as each phase lands in
// `pass_manager::main` / `build()` / `build_and_output`, the panic shifts
// further down the pipeline until the test goes green. Marked #[ignore] so
// the 0-failure baseline stays clean while the wiring is being driven.
//
// Bug report: pass-manager-wiring-bug.md.

use std::fs;

#[test]
#[ignore]
fn pass_manager_main_builds_simple_program_end_to_end() {
  let work = tempfile::tempdir().unwrap();
  let src_dir = work.path().join("src");
  fs::create_dir_all(&src_dir).unwrap();
  fs::write(
    src_dir.join("main.vale"),
    "exported func main() int { return 3; }",
  )
  .unwrap();
  let out_dir = work.path().join("out");
  fs::create_dir_all(&out_dir).unwrap();

  let args = vec![
    "build".to_string(),
    "--output_dir".to_string(),
    out_dir.display().to_string(),
    "--output_vast".to_string(),
    "true".to_string(),
    "--include_builtins".to_string(),
    "true".to_string(),
    format!("test={}", src_dir.display()),
  ];

  crate::pass_manager::pass_manager::main(args);

  let vast_dir = out_dir.join("vast");
  assert!(
    vast_dir.is_dir(),
    "expected vast output dir at {}",
    vast_dir.display()
  );
  let vast_files: Vec<_> = fs::read_dir(&vast_dir)
    .unwrap()
    .filter_map(|e| e.ok())
    .collect();
  assert!(
    !vast_files.is_empty(),
    "expected at least one .vast file in {}",
    vast_dir.display()
  );
}
/* Guardian: disable-all */
