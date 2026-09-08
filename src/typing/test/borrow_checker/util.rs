use super::super::compiler_test_compilation::compiler_test_compilation_with_borrow_check;
use crate::builtins::builtins::{builtin_source_for_arrays, empty_v_builtins_stub};
use crate::code_source::{CodeSource, Source};
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::tests::tests::new_test_code_map;
use crate::typing::test::humanize_helper::{assert_humanized_eq, humanize_compile_error};
use crate::typing::typing_interner::TypingInterner;
use bumpalo::Bump;

/// Compile `code` and assert its rendered borrow-check diagnostic equals `expected`. Snapshot-style,
/// like rustc's UI `.stderr` goldens: on a mismatch `assert_humanized_eq` prints the actual output to
/// paste back in, so re-blessing a legitimate wording/range change is a copy-paste.
pub fn assert_borrow_error_renders(code: &str, expected: &str) {
  let (parse_bump, scout_bump, typing_bump) = (Bump::new(), Bump::new(), Bump::new());
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation_with_borrow_check(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected a borrow error, got Ok");
  assert_humanized_eq(&humanize_compile_error(&mut compile, err), expected);
}

/// Compile `code` and assert it compiles clean — rustc's pass-test model (a clean compile, no
/// diagnostic output).
pub fn assert_compiles_clean(code: &str) {
  let (parse_bump, scout_bump, typing_bump) = (Bump::new(), Bump::new(), Bump::new());
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation_with_borrow_check(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}

/// Compile `code` and assert the borrow checker's per-parameter `noalias` verdict for the function
/// named `function_human_name` — one bool per parameter, in signature order, true where the parameter
/// is the sole reference into a group no other parameter aliases.
pub fn assert_param_noalias(code: &str, function_human_name: &str, expected: &[bool]) {
  let (parse_bump, scout_bump, typing_bump) = (Bump::new(), Bump::new(), Bump::new());
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation_with_borrow_check(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let hinputs = compile.expect_compiler_outputs();
  assert_eq!(hinputs.param_noalias(function_human_name), expected);
}

/// Compile `code` and assert one restrict region of `function_human_name`: the one carrying the bare
/// integer landmark `marker` in its span. A restrict region is a span where one reference is the sole
/// live reference into its group (so the backend may emit `!alias.scope`/`!noalias` on the accesses
/// inside it). Pinning by `marker` — a bare `<marker>;` statement placed inside the region — keeps the
/// test stable against node renumbering. Asserts the region's scope group name, its disjoint group
/// names, and its access and call counts.
pub fn assert_restrict_region(
  code: &str,
  function_human_name: &str,
  marker: i32,
  expected_scope_group: &str,
  expected_disjoint_groups: &[&str],
  expected_access_count: usize,
  expected_call_count: usize,
) {
  let (parse_bump, scout_bump, typing_bump) = (Bump::new(), Bump::new(), Bump::new());
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code_source = CodeSource::new(vec![new_test_code_map(&parse_arena, code)]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation_with_borrow_check(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let hinputs = compile.expect_compiler_outputs();
  let region = hinputs
    .restrict_regions(function_human_name)
    .iter()
    .find(|r| r.markers.contains(&marker))
    .unwrap_or_else(|| panic!("no restrict region carrying marker {marker} in {function_human_name}"));
  assert_eq!(region.scope_group_name().as_str(), expected_scope_group);
  assert_eq!(
    region.disjoint_group_names(),
    expected_disjoint_groups.iter().map(|s| s.to_string()).collect::<Vec<_>>()
  );
  assert_eq!(region.accesses.len(), expected_access_count);
  assert_eq!(region.calls.len(), expected_call_count);
}

/// Like `assert_borrow_error_renders`, but the code source also carries the array builtins, so a
/// fixture may use runtime-sized arrays (`Array<int>(n)`, `a[i]`). The fixture must `import
/// v.builtins.arrays.*;` (and any other builtins it needs).
pub fn assert_borrow_error_renders_with_arrays(code: &str, expected: &str) {
  let (parse_bump, scout_bump, typing_bump) = (Bump::new(), Bump::new(), Bump::new());
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code_source = CodeSource::new(vec![
    builtin_source_for_arrays(&parse_arena, &parser_keywords),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation_with_borrow_check(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  let err = compile.get_compiler_outputs().err().expect("expected a borrow error, got Ok");
  assert_humanized_eq(&humanize_compile_error(&mut compile, err), expected);
}

/// Like `assert_compiles_clean`, but the code source also carries the array builtins.
pub fn assert_compiles_clean_with_arrays(code: &str) {
  let (parse_bump, scout_bump, typing_bump) = (Bump::new(), Bump::new(), Bump::new());
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code_source = CodeSource::new(vec![
    builtin_source_for_arrays(&parse_arena, &parser_keywords),
    new_test_code_map(&parse_arena, code),
    Source::Fn(empty_v_builtins_stub),
  ]);
  let typing_interner = TypingInterner::new(&typing_bump);
  let mut compile = compiler_test_compilation_with_borrow_check(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}
