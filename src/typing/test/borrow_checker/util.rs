use super::super::compiler_test_compilation::compiler_test_compilation;
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
  let mut compile = compiler_test_compilation(
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
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
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
  let mut compile = compiler_test_compilation(
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
  let mut compile = compiler_test_compilation(
    &typing_interner,
    &scout_arena,
    &keywords,
    &parser_keywords,
    &parse_arena,
    &code_source,
  );
  compile.expect_compiler_outputs();
}
