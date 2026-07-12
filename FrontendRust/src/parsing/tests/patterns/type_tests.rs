
use bumpalo::Bump;
use crate::cast;
use crate::parse_arena::ParseArena;
use crate::keywords::Keywords;
use crate::parsing::ast::{
  INameDeclarationP, ITemplexPT, SharednessP, PatternPP,
};
use crate::parsing::tests::utils::{
  assert_templex_name, compile_pattern_expect, expect_1, expect_2,
};

fn compile<'p, 'ctx>(
  parse_arena: &'ctx ParseArena<'p>,
  keywords: &'ctx Keywords<'p>,
  code: &str,
) -> PatternPP<'p>
where
  'p: 'ctx,
{
  compile_pattern_expect(parse_arena, keywords, code)
}

#[test]
fn ignoring_name() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ int");
  let destination = pattern.destination.unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  assert_templex_name(pattern.templex.as_ref().unwrap(), "int");
  assert!(pattern.destructure.is_none());
}

#[test]
fn runtime_sized_array() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ []int");
  let destination = pattern.destination.unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  let rsa = cast!(
    pattern.templex.as_ref().unwrap(),
    ITemplexPT::RuntimeSizedArray
  );
  assert_templex_name(rsa.element, "int");
  assert!(pattern.destructure.is_none());
}

#[test]
fn sequence_type() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ (int, bool)");
  let destination = pattern.destination.unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  let tuple = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::Tuple);
  let (int_t, bool_t) = expect_2(&tuple.elements);
  assert_templex_name(int_t, "int");
  assert_templex_name(bool_t, "bool");
  assert!(pattern.destructure.is_none());
}

#[test]
fn caret_type_is_error() {
  // `^` is a value-level Move operator only; it's not a templex prefix.
  use crate::parsing::tests::utils::compile_templex;
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let result = compile_templex(&parse_arena, &keywords, "^T");
  assert!(result.is_err(), "expected `^T` at templex level to be a parse error");
}

#[test]
fn heap_prefix_type() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ heap T");
  let heap_own_ref = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::HeapOwnRef);
  assert_templex_name(heap_own_ref.inner, "T");
  assert!(pattern.destructure.is_none());
}

#[test]
fn weak_prefix_type() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ weak T");
  let weak_ref = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::WeakRef);
  assert_templex_name(weak_ref.inner, "T");
  assert!(pattern.destructure.is_none());
}

#[test]
fn borrow_with_region() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ &i'MyStruct");
  let borrow_ref = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::BorrowRef);
  let region = borrow_ref.region.as_ref().unwrap();
  assert_eq!(region.name.as_ref().unwrap().as_str(), "i");
  assert_templex_name(borrow_ref.inner, "MyStruct");
  assert!(pattern.destructure.is_none());
}

#[test]
fn call_type() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let pattern = compile(&parse_arena, &keywords, "_ MyOption<MyList<int>>");
  let destination = pattern.destination.unwrap();
  assert!(matches!(
    destination.decl,
    INameDeclarationP::IgnoredLocalNameDeclaration(_)
  ));
  assert!(destination.mutate.is_none());
  let myoption_call = cast!(pattern.templex.as_ref().unwrap(), ITemplexPT::Call);
  assert_templex_name(myoption_call.template, "MyOption");
  let mylist_type = expect_1(&myoption_call.args);
  let mylist_call = cast!(mylist_type, ITemplexPT::Call);
  assert_templex_name(mylist_call.template, "MyList");
  let int_type = expect_1(&mylist_call.args);
  assert_templex_name(int_type, "int");
  assert!(pattern.destructure.is_none());
}

