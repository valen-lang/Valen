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

#[test]
fn pass_manager_main_builds_program_using_builtin_some() {
  let work = tempfile::tempdir().unwrap();
  let src_dir = work.path().join("src");
  fs::create_dir_all(&src_dir).unwrap();
  fs::write(
    src_dir.join("main.vale"),
    "exported func main() int { x = Some<int>(3); return 0; }",
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

#[test]
fn pass_manager_build_returns_humanized_couldnt_find_function() {
  let work = tempfile::tempdir().unwrap();
  let src_dir = work.path().join("src");
  fs::create_dir_all(&src_dir).unwrap();
  fs::write(
    src_dir.join("main.vale"),
    "exported func main() int { return nonexistent_function(0); }",
  )
  .unwrap();
  let out_dir = work.path().join("out");
  fs::create_dir_all(&out_dir).unwrap();

  let parse_bump = bumpalo::Bump::new();
  let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
  let keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);

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

  let opts = crate::pass_manager::pass_manager::parse_opts(
    &parse_arena,
    crate::pass_manager::pass_manager::Options {
      inputs: vec![],
      output_dir_path: None,
      input_vpst_dir: None,
      benchmark: false,
      output_vpst: true,
      output_vast: true,
      output_highlights: false,
      include_builtins: true,
      mode: None,
      sanity_check: false,
      use_optimized_solver: true,
      use_overload_index: true,
      verbose_errors: false,
      debug_output: false,
    },
    args,
  );

  let result = crate::pass_manager::pass_manager::build(&parse_arena, &keywords, &opts);
  let err = result.expect_err("expected compile error for nonexistent_function call");
  let expected_suffix =
    "Couldn't find a suitable function nonexistent_function(i32). No function with that name exists.\n\n";
  assert!(
    err.ends_with(expected_suffix),
    "humanized error suffix mismatch\nexpected ending: {:?}\nactual: {:?}",
    expected_suffix,
    err,
  );
}

#[test]
fn pass_manager_build_returns_humanized_higher_typing_couldnt_find_type() {
  let work = tempfile::tempdir().unwrap();
  let src_dir = work.path().join("src");
  fs::create_dir_all(&src_dir).unwrap();
  fs::write(
    src_dir.join("main.vale"),
    "exported func main(a Bork) {\n}\n",
  )
  .unwrap();
  let out_dir = work.path().join("out");
  fs::create_dir_all(&out_dir).unwrap();

  let parse_bump = bumpalo::Bump::new();
  let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
  let keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);

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

  let opts = crate::pass_manager::pass_manager::parse_opts(
    &parse_arena,
    crate::pass_manager::pass_manager::Options {
      inputs: vec![],
      output_dir_path: None,
      input_vpst_dir: None,
      benchmark: false,
      output_vpst: true,
      output_vast: true,
      output_highlights: false,
      include_builtins: true,
      mode: None,
      sanity_check: false,
      use_optimized_solver: true,
      use_overload_index: true,
      verbose_errors: false,
      debug_output: false,
    },
    args,
  );

  let result = crate::pass_manager::pass_manager::build(&parse_arena, &keywords, &opts);
  let err = result.expect_err("expected higher-typing error for unknown type 'Bork'");
  let expected_suffix =
    r#"Couldn't solve generics rules:
Couldn't find anything with the name 'Bork'
exported func main(a Bork) {
                           ^ _3: (unknown)
                     ^^^^ _21111: (unknown)
Steps:
Unsolved rule: _21111 = "Bork"
Unsolved rule: _3 = "void"

exported func main(a Bork) {
"#;
  assert!(
    err.ends_with(expected_suffix),
    "humanized error suffix mismatch\nexpected ending: {:?}\nactual: {:?}",
    expected_suffix,
    err,
  );
}
/* Guardian: disable-all */
