/*
package dev.vale.parsing

import dev.vale.lexing.Lexer
import dev.vale.{Collector, Interner, StrI, vassertOne, vimpl}
import dev.vale.parsing.ast.{CallPT, IDenizenP, GenericParameterP, GenericParametersP, ImplP, MutabilityPT, MutableP, NameOrRunePT, NameP, TopLevelImplP}
import dev.vale.options.GlobalOptions
import org.scalatest._


class ImplTests extends FunSuite with Matchers with Collector with TestParseUtils {
*/
use bumpalo::Bump;
use crate::cast;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::*;

#[test]
fn normal_impl() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let file = compile(&interner, &keywords, "impl MyInterface for SomeStruct;");
  let denizen = expect_1(&file.denizens);
  let impl_ = cast!(denizen, IDenizenP::TopLevelImpl);

  assert!(impl_.generic_params.is_none());
  assert!(impl_.template_rules.is_none());
  assert_templex_name(impl_.struct_.as_ref().unwrap(), "SomeStruct");
  assert_templex_name(&impl_.interface, "MyInterface");
  assert_eq!(impl_.attributes.len(), 0);
}
/*
  test("Normal impl") {
    vassertOne(
      compileFile(
        """
          |impl MyInterface for SomeStruct;
      """.stripMargin).getOrDie().denizens) shouldHave {
      case TopLevelImplP(ImplP(_,
          None,
          None,
          Some(NameOrRunePT(NameP(_, StrI("SomeStruct")))),
          NameOrRunePT(NameP(_, StrI("MyInterface"))),
          Vector())) =>
    }
  }
*/

#[test]
fn templated_impl() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let file = compile(&interner, &keywords, "impl<T> MyInterface<T> for SomeStruct<T>;");
  let denizen = expect_1(&file.denizens);
  let impl_ = cast!(denizen, IDenizenP::TopLevelImpl);

  let generic_params = impl_.generic_params.as_ref().unwrap();
  let generic_param = expect_1(&generic_params.params);
  assert_eq!(generic_param.name.str.str, "T");
  assert!(generic_param.maybe_type.is_none());
  assert!(generic_param.coord_region.is_none());
  assert_eq!(generic_param.attributes.len(), 0);
  assert!(generic_param.maybe_default.is_none());
  assert!(impl_.template_rules.is_none());

  let struct_ = cast!(impl_.struct_.as_ref().unwrap(), ITemplexPT::Call);
  assert_templex_name(struct_.template.as_ref(), "SomeStruct");
  let struct_template_arg = expect_1(&struct_.args);
  assert_templex_name(struct_template_arg, "T");

  let interface = cast!(&impl_.interface, ITemplexPT::Call);
  assert_templex_name(interface.template.as_ref(), "MyInterface");
  let interface_template_arg = expect_1(&interface.args);
  assert_templex_name(interface_template_arg, "T");
  assert_eq!(impl_.attributes.len(), 0);
}
/*
  test("Templated impl") {
    vassertOne(
      compileFile(
        """
          |impl<T> MyInterface<T> for SomeStruct<T>;
      """.stripMargin).getOrDie().denizens) shouldHave {
      case TopLevelImplP(ImplP(_,
        Some(GenericParametersP(_, Vector(GenericParameterP(_, NameP(_, StrI("T")), _, _, Vector(), None)))),
        None,
        Some(CallPT(_,NameOrRunePT(NameP(_, StrI("SomeStruct"))), Vector(NameOrRunePT(NameP(_, StrI("T")))))),
        CallPT(_,NameOrRunePT(NameP(_, StrI("MyInterface"))), Vector(NameOrRunePT(NameP(_, StrI("T"))))),
        Vector())) =>
    }
  }
*/

#[test]
fn impling_a_template_call() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let file = compile(&interner, &keywords, "impl IFunction1<mut, int, int> for MyIntIdentity;");
  let denizen = expect_1(&file.denizens);
  let impl_ = cast!(denizen, IDenizenP::TopLevelImpl);

  assert!(impl_.generic_params.is_none());
  assert!(impl_.template_rules.is_none());
  assert_templex_name(impl_.struct_.as_ref().unwrap(), "MyIntIdentity");

  let interface = cast!(&impl_.interface, ITemplexPT::Call);
  assert_templex_name(interface.template.as_ref(), "IFunction1");
  let (mutability_arg, int_arg1, int_arg2) = expect_3(&interface.args);
  assert_eq!(
    cast!(mutability_arg, ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert_templex_name(int_arg1, "int");
  assert_templex_name(int_arg2, "int");
  assert_eq!(impl_.attributes.len(), 0);
}
/*
  test("Impling a template call") {
    vassertOne(
      compileFile(
        """
          |impl IFunction1<mut, int, int> for MyIntIdentity;
          |""".stripMargin).getOrDie().denizens) shouldHave {
      case TopLevelImplP(ImplP(_,
        None,
        None,
        Some(NameOrRunePT(NameP(_, StrI("MyIntIdentity")))),
        CallPT(_,NameOrRunePT(NameP(_, StrI("IFunction1"))), Vector(MutabilityPT(_,MutableP), NameOrRunePT(NameP(_, StrI("int"))), NameOrRunePT(NameP(_, StrI("int"))))),
        Vector())) =>
    }
  }
}
*/