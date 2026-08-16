use crate::lexing::lexing_iterator::LexingIterator;

#[test]
fn advance_past_multibyte_lands_on_next_char() {
  let mut iter = LexingIterator::new("«x");
  let c = iter.advance();
  assert_eq!(c, '«');
  assert_eq!(iter.position, 2);
  assert_eq!(iter.peek(), 'x');
}

#[test]
fn try_skip_str_through_multibyte() {
  let mut iter = LexingIterator::new("«x");
  assert!(iter.try_skip_str("«"));
  assert_eq!(iter.position, 2);
  assert!(iter.try_skip_str("x"));
  assert_eq!(iter.position, 3);
  assert!(iter.at_end());
}

#[test]
fn angle_is_open_or_close_misreads_after_multibyte_drift() {
  use bumpalo::Bump;
  use crate::parse_arena::ParseArena;
  use crate::keywords::Keywords;
  use crate::parsing::tests::utils::compile_file;

  let code = "\
exported func main() int {\n\
  s = \"«»«»«»«»«»«»«»«»«»«»\";\n\
  if (a > b) {\n\
    return 1;\n\
  }\n\
  return 0;\n\
}\n";
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let _ = compile_file(&parse_arena, &keywords, code)
    .unwrap_or_else(|e| panic!("UTF-8 angle misread reproduced: {:?}", e));
}

