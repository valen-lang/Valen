// Run with: cargo test --manifest-path Cargo.toml --lib parsing::tests::rules::kind_rule_tests
use crate::cast;
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::*;
use bumpalo::Bump;

fn compile<'p, 'ctx>(
  parse_arena: &'ctx ParseArena<'p>,
  keywords: &'ctx Keywords<'p>,
  code: &str,
) -> IRulexPR<'p>
where
  'p: 'ctx,
{
  compile_rulex_expect(parse_arena, keywords, code)
}

#[test]
fn kind_matches_plain_int() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let rule = compile(&parse_arena, &keywords, "int");
  let templex = cast!(rule, IRulexPR::Templex);
  assert_templex_name(&templex, "int");
}

#[test]
fn rune_with_value() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let rule = compile(&parse_arena, &keywords, "T = int");
  let equals = cast!(rule, IRulexPR::Equals);
  let left = cast!(equals.left, IRulexPR::Templex);
  assert_templex_name(left, "T");
  let right = cast!(equals.right, IRulexPR::Templex);
  assert_templex_name(right, "int");
}

#[test]
fn rune_with_sequence_in_value_spot() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let rule = compile(&parse_arena, &keywords, "T = (int, bool)");
  let equals = cast!(rule, IRulexPR::Equals);
  let left = cast!(equals.left, IRulexPR::Templex);
  assert_templex_name(left, "T");
  let right = cast!(equals.right, IRulexPR::Templex);
  let tuple = cast!(right, ITemplexPT::Tuple);
  let (int_, bool_) = expect_2(&tuple.elements);
  assert_templex_name(int_, "int");
  assert_templex_name(bool_, "bool");
}

#[test]
fn lone_sequence() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let rule = compile(&parse_arena, &keywords, "(int, bool)");
  let templex = cast!(rule, IRulexPR::Templex);
  let tuple = cast!(templex, ITemplexPT::Tuple);
  let (int_, bool_) = expect_2(&tuple.elements);
  assert_templex_name(int_, "int");
  assert_templex_name(bool_, "bool");
}

#[test]
fn templated_struct_one_arg() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let rule = compile(&parse_arena, &keywords, "Moo<int>");
  match rule {
    IRulexPR::Templex(ITemplexPT::Call(CallPT {
      template: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("Moo")), .. }),
      args: [ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("int")), .. })],
      ..
    })) => {}
    other => panic!("expected `Moo<int>` → Call(Moo, [int]), got {:?}", other),
  }
}

#[test]
fn rwkilc() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let rule = compile(&parse_arena, &keywords, "List<int>");
  let templex = cast!(rule, IRulexPR::Templex);
  let call = cast!(templex, ITemplexPT::Call);
  assert_templex_name(call.template, "List");
  let arg = expect_1(&call.args);
  assert_templex_name(arg, "int");

  let rule = compile(&parse_arena, &keywords, "K Int");
  let typed = cast!(rule, IRulexPR::Typed);
  assert_eq!(typed.rune.as_ref().unwrap().as_str(), "K");
  assert_eq!(typed.tyype, ITypePR::IntType);

  let rule = compile(&parse_arena, &keywords, "K<int>");
  let templex = cast!(rule, IRulexPR::Templex);
  let call = cast!(templex, ITemplexPT::Call);
  assert_templex_name(call.template, "K");
  let arg = expect_1(&call.args);
  assert_templex_name(arg, "int");
}

#[test]
fn templated_struct_rune_arg() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let rule = compile(&parse_arena, &keywords, "Moo<R>");
  let templex = cast!(rule, IRulexPR::Templex);
  let call = cast!(templex, ITemplexPT::Call);
  assert_templex_name(call.template, "Moo");
  let arg = expect_1(&call.args);
  assert_templex_name(arg, "R");
}

#[test]
fn templated_struct_multiple_args() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let rule = compile(&parse_arena, &keywords, "Moo<int, str>");
  let templex = cast!(rule, IRulexPR::Templex);
  let call = cast!(templex, ITemplexPT::Call);
  assert_templex_name(call.template, "Moo");
  let (int_, str_) = expect_2(&call.args);
  assert_templex_name(int_, "int");
  assert_templex_name(str_, "str");
}

#[test]
fn templated_struct_arg_is_another_templated_struct_with_one_arg() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let rule = compile(&parse_arena, &keywords, "Moo<Blarg<int>>");
  let templex = cast!(rule, IRulexPR::Templex);
  let call = cast!(templex, ITemplexPT::Call);
  assert_templex_name(call.template, "Moo");
  let nested = cast!(expect_1(&call.args), ITemplexPT::Call);
  assert_templex_name(nested.template, "Blarg");
  let arg = expect_1(&nested.args);
  assert_templex_name(arg, "int");
}

#[test]
fn templated_struct_arg_is_another_templated_struct_with_multiple_arg() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let rule = compile(&parse_arena, &keywords, "Moo<Blarg<int, str>>");
  let templex = cast!(rule, IRulexPR::Templex);
  let call = cast!(templex, ITemplexPT::Call);
  assert_templex_name(call.template, "Moo");
  let nested = cast!(expect_1(&call.args), ITemplexPT::Call);
  assert_templex_name(nested.template, "Blarg");
  let (int_, str_) = expect_2(&nested.args);
  assert_templex_name(int_, "int");
  assert_templex_name(str_, "str");
}

#[test]
fn static_array_type() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  match compile_templex_expect(&parse_arena, &keywords, "StaticArray<_, _>") {
    ITemplexPT::Call(CallPT {
      template: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("StaticArray")), .. }),
      args: [ITemplexPT::AnonymousRune(_), ITemplexPT::AnonymousRune(_)],
      ..
    }) => {}
    other => panic!("unexpected: {:?}", other),
  }

  match compile_templex_expect(&parse_arena, &keywords, "StaticArray<3, int>") {
    ITemplexPT::Call(CallPT {
      template: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("StaticArray")), .. }),
      args:
        [ITemplexPT::Int(IntPT { value: 3, .. }), ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("int")), .. })],
      ..
    }) => {}
    other => panic!("unexpected: {:?}", other),
  }

  match compile_templex_expect(&parse_arena, &keywords, "StaticArray<N, T>") {
    ITemplexPT::Call(CallPT {
      template: ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("StaticArray")), .. }),
      args:
        [ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("N")), .. }), ITemplexPT::NameOrRune(NameOrRunePT { name: NameP(_, StrI("T")), .. })],
      ..
    }) => {}
    other => panic!("unexpected: {:?}", other),
  }
}

#[test]
fn regular_sequence() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let tuple = cast!(compile_templex_expect(&parse_arena, &keywords, "()"), ITemplexPT::Tuple);
  assert_eq!(tuple.elements.len(), 0);

  let tuple = cast!(compile_templex_expect(&parse_arena, &keywords, "(int)"), ITemplexPT::Tuple);
  assert_templex_name(*expect_1(tuple.elements), "int");

  let tuple =
    cast!(compile_templex_expect(&parse_arena, &keywords, "(int, bool)"), ITemplexPT::Tuple);
  let (int_, bool_) = expect_2(tuple.elements);
  assert_templex_name(int_, "int");
  assert_templex_name(bool_, "bool");

  let tuple =
    cast!(compile_templex_expect(&parse_arena, &keywords, "(_, bool)"), ITemplexPT::Tuple);
  let (anonymous_, bool_) = expect_2(tuple.elements);
  cast!(anonymous_, ITemplexPT::AnonymousRune);
  assert_templex_name(bool_, "bool");

  let tuple = cast!(compile_templex_expect(&parse_arena, &keywords, "(_, _)"), ITemplexPT::Tuple);
  let (anonymous1_, anonymous2_) = expect_2(tuple.elements);
  cast!(anonymous1_, ITemplexPT::AnonymousRune);
  cast!(anonymous2_, ITemplexPT::AnonymousRune);
}

#[test]
fn prototype_kind_rule() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let templex = compile_templex_expect(&parse_arena, &keywords, "func moo(int)void");
  let prototype = cast!(templex, ITemplexPT::Func);
  assert_eq!(prototype.name.as_str(), "moo");
  assert_templex_name(*expect_1(prototype.parameters), "int");
  assert_templex_name(prototype.return_type, "void");

  let templex = compile_templex_expect(&parse_arena, &keywords, "func moo(T)R");
  let prototype = cast!(templex, ITemplexPT::Func);
  assert_eq!(prototype.name.as_str(), "moo");
  assert_templex_name(*expect_1(prototype.parameters), "T");
  assert_templex_name(prototype.return_type, "R");
}
