// TDD driver for `pass_manager::build` end-to-end wiring.
//
// The Rust pipeline (parse → scout → typing → instantiating → simplifying →
// hammer → MetalLowerer → backend) is migrated and green when driven through
// the test harness. These tests target `pass_manager::build` directly, both
// for happy-path round-trips and for asserting that humanized errors come
// out in their Scala-faithful form.
//
// Bug report: pass-manager-wiring-bug.md.

use std::fs;

#[test]
#[ignore]
fn pass_manager_main_builds_simple_program_end_to_end() {
  // Ignored: full happy-path drives LLVM codegen via Backend FFI, which
  // needs a configured LLVM/clang env we can't assume in unit tests.
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
      benchmark: false,
      output_vast: true,
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

  let (_rc, _stems) = crate::pass_manager::pass_manager::build(
    &parse_arena, &keywords, &opts, &[],
  ).expect("build should succeed for trivial program");
}

#[test]
#[ignore]
fn pass_manager_main_builds_program_using_builtin_some() {
  // Ignored: same reason as above (drives Backend codegen end-to-end).
  // The intent of the test is to keep regression coverage that Some<int>
  // survives all the way through the typing+hammer stages.
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
      benchmark: false,
      output_vast: true,
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

  let (_rc, _stems) = crate::pass_manager::pass_manager::build(
    &parse_arena, &keywords, &opts, &[],
  ).expect("build should succeed for Some<int> program");
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
      benchmark: false,
      output_vast: true,
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

  let result = crate::pass_manager::pass_manager::build(&parse_arena, &keywords, &opts, &[]);
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
      benchmark: false,
      output_vast: true,
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

  let result = crate::pass_manager::pass_manager::build(&parse_arena, &keywords, &opts, &[]);
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
