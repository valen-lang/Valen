//! Tests that drive the typing pass to a known failure and assert on
//! `compiler_error_humanizer::humanize` output byte-for-byte. Mirrors the
//! pattern used by `higher_typing/tests/error_tests.rs`, but for typing-pass
//! errors instead of higher-typing ones.
//!
//! Re-homed from `pass_manager/end_to_end_test.rs` (which previously drove the
//! error through `pass_manager::build`). Asserting against the typing pass
//! directly makes the test scope-tight and lifetime-independent of the
//! backend/clang path.
use crate::code_source::{CodeSource, Source};
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::tests::tests::new_test_code_map;
use crate::typing::compiler_error_humanizer::humanize;
use crate::typing::test::compiler_test_compilation::compiler_test_compilation;
use crate::typing::typing_interner::TypingInterner;
use crate::utils::source_code_utils::{
  humanize_pos_code_map, line_containing, line_range_containing, lines_between,
};
use bumpalo::Bump;

#[test]
fn humanize_couldnt_find_function_no_function_with_that_name() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let typing_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let parser_keywords = Keywords::new_for_parse(&parse_arena);
  let code = "exported func main() int { return nonexistent_function(0); }".to_string();
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
  let err = match compile.get_compiler_outputs() {
    Err(e) => e,
    Ok(_) => panic!("expected typing-pass error for nonexistent_function call, got Ok"),
  };
  let code_map = compile.get_code_map().unwrap();
  let humanized = humanize(
    &scout_arena,
    &typing_interner,
    false,
    &|x| humanize_pos_code_map(&code_map, &x),
    &|a, b| lines_between(&code_map, &a, &b),
    &|x| line_range_containing(&code_map, &x),
    &|x| line_containing(&code_map, &x),
    err,
  );
  let expected_suffix =
        "Couldn't find a suitable function nonexistent_function(i32). No function with that name exists.\n\n";
  assert!(
    humanized.ends_with(expected_suffix),
    "humanized error suffix mismatch\nexpected ending: {:?}\nactual: {:?}",
    expected_suffix,
    humanized,
  );
}
