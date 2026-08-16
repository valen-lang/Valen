// Run with: cargo test --manifest-path Cargo.toml --lib parsing::tests::rules::rule_tests


use bumpalo::Bump;
use crate::cast;
use crate::parse_arena::ParseArena;
use crate::keywords::Keywords;
use crate::parsing::ast::*;
use crate::parsing::tests::traverse::NodeRefP;
use crate::parsing::tests::utils::*;
use crate::collect_only_rulex;

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
fn relations() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  {
    let rule = compile(&parse_arena, &keywords, "implements(MyObject, IObject)");
    let builtin = collect_only_rulex!(
      &rule,
      NodeRefP::Rulex(IRulexPR::BuiltinCall(builtin)) => Some(builtin)
    );
    assert_eq!(builtin.name.as_str(), "implements");
    let (myobject_, iobject_) = expect_2(&builtin.args);
    assert_templex_name(cast!(myobject_, IRulexPR::Templex), "MyObject");
    assert_templex_name(cast!(iobject_, IRulexPR::Templex), "IObject");
  }

  {
    let rule = compile(&parse_arena, &keywords, "implements(R, IObject)");
    let builtin = collect_only_rulex!(
      &rule,
      NodeRefP::Rulex(IRulexPR::BuiltinCall(builtin)) => Some(builtin)
    );
    assert_eq!(builtin.name.as_str(), "implements");
    let (r_, iobject_) = expect_2(&builtin.args);
    assert_templex_name(cast!(r_, IRulexPR::Templex), "R");
    assert_templex_name(cast!(iobject_, IRulexPR::Templex), "IObject");
  }

  {
    let rule = compile(&parse_arena, &keywords, "implements(MyObject, T)");
    let builtin = collect_only_rulex!(
      &rule,
      NodeRefP::Rulex(IRulexPR::BuiltinCall(builtin)) => Some(builtin)
    );
    assert_eq!(builtin.name.as_str(), "implements");
    let (myobject_, t_) = expect_2(&builtin.args);
    assert_templex_name(cast!(myobject_, IRulexPR::Templex), "MyObject");
    assert_templex_name(cast!(t_, IRulexPR::Templex), "T");
  }

  {
    let rule = compile(&parse_arena, &keywords, "exists(func +(T)int)");
    let builtin = collect_only_rulex!(
      &rule,
      NodeRefP::Rulex(IRulexPR::BuiltinCall(builtin)) => Some(builtin)
    );
    assert_eq!(builtin.name.as_str(), "exists");
    let func = cast!(cast!(expect_1(&builtin.args), IRulexPR::Templex), ITemplexPT::Func);
    assert_eq!(func.name.as_str(), "+");
    assert_templex_name(*expect_1(func.parameters), "T");
    assert_templex_name(func.return_type, "int");
  }
}

#[test]
fn super_complicated() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  compile(&parse_arena, &keywords, "C = any(StaticArray<I, X>, StaticArray<N, T>)");
}

#[test]
fn func() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let rule = compile(&parse_arena, &keywords, "func moo()T");
  let func = collect_only_rulex!(&rule, NodeRefP::Templex(ITemplexPT::Func(func)) => Some(func));
  assert_eq!(func.name.as_str(), "moo");
  assert!(func.parameters.is_empty());
  assert_templex_name(func.return_type, "T");
}

