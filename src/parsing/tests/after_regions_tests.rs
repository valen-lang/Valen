use bumpalo::Bump;
use crate::parse_arena::ParseArena;
use crate::keywords::Keywords;
use crate::lexing::errors::ParseError;
use crate::parsing::tests::utils::{compile_statement, compile_block_contents};



#[test]
fn forgetting_set_when_changing() {
  // This test does not pass yet, use #[ignore].
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let error =
    compile_statement(
      &parse_arena,
      &keywords,
      "ship.x = 4;").unwrap_err();
  match error {
    ParseError::ForgotSetKeyword(_) => {}
    other => panic!("Expected ForgotSetKeyword, got {:?}", other),
  }
}

