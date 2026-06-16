use bumpalo::Bump;
use crate::parse_arena::ParseArena;
use crate::keywords::Keywords;
use crate::lexing::errors::ParseError;
use crate::parsing::tests::utils::compile_denizen;


// mig: fn func_with_func_bound_with_missing_where
#[test]
#[ignore = "blocked - Rust parser produces TopLevelFunction for `func sum<T>() func moo(&T)void {3}` instead of ParseError::FuncBoundWithoutWhere. Tracked in migration-drive-todo.md Phase 4e."]
fn func_with_func_bound_with_missing_where() {
  // This test does not pass yet, use #[ignore].
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let err = compile_denizen(
    &parse_arena,
    &keywords,
    "func sum<T>() func moo(&T)void {3}").unwrap_err();
  match err {
    ParseError::FuncBoundWithoutWhere(_) => {}
    other => panic!("Expected FuncBoundWithoutWhere, got {:?}", other),
  }
}

