// Run with: cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing::tests::rules::kind_rule_tests
use bumpalo::Bump;
use crate::cast;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::*;

/*
package dev.vale.parsing.rules

import dev.vale.{Collector, StrI, vimpl}
import dev.vale.parsing.ast.{AnonymousRunePT, CallPT, ComponentsPR, EqualsPR, FinalP, IRulexPR, ImmutableP, IntPT, IntTypePR, InterpretedPT, KindTypePR, MutabilityPT, MutableP, NameOrRunePT, NameP, FuncPT, ShareP, StaticSizedArrayPT, TemplexPR, TuplePT, TypedPR, VariabilityPT}
import dev.vale.parsing.templex.TemplexParser
import dev.vale.parsing._
import dev.vale.parsing.ast._
import org.scalatest._

class KindRuleTests extends FunSuite with Matchers with Collector with TestParseUtils {
  private def compile[T](code: String): IRulexPR = {
    compileRulex(code)
//    compile(new TemplexParser().parseRule(_), code)
  }
*/
fn compile<'a, 'ctx>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  code: &str,
) -> IRulexPR<'a>
where
  'a: 'ctx,
{
  compile_rulex_expect(interner, keywords, code)
}

#[test]
fn empty_kind_rule() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let rule = compile(&interner, &keywords, "_ Kind");
  let typed = cast!(rule, IRulexPR::Typed);
  assert!(typed.rune.is_none());
  assert_eq!(typed.tyype, ITypePR::KindType);
}
/*
  test("Empty Kind rule") {
    compile("_ Kind") shouldHave {
      case TypedPR(_,None,KindTypePR) =>
    }
  }
*/
#[test]
fn kind_with_rune() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let rule = compile(&interner, &keywords, "T Kind");
  let typed = cast!(rule, IRulexPR::Typed);
  assert_eq!(typed.rune.as_ref().unwrap().as_str(), "T");
  assert_eq!(typed.tyype, ITypePR::KindType);
}
/*
  test("Kind with rune") {
    compile("T Kind") shouldHave {
      case TypedPR(_,Some(NameP(_, StrI("T"))),KindTypePR) =>
    }
    //runedTKind("T")
  }
*/
#[test]
fn kind_with_destructure_only() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let rule = compile(&interner, &keywords, "Kind[_]");
  let components = cast!(rule, IRulexPR::Components);
  assert_eq!(components.container, ITypePR::KindType);
  let only_component = cast!(expect_1(&components.components), IRulexPR::Templex);
  cast!(only_component, ITemplexPT::AnonymousRune);
}
/*
  test("Kind with destructure only") {
    compile("Kind[_]") shouldHave {
      case ComponentsPR(_,KindTypePR,Vector(TemplexPR(AnonymousRunePT(_)))) =>
    }
//        KindPR(None, KindTypePR, None, None)
  }
*/
#[test]
fn kind_matches_plain_int() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let rule = compile(&interner, &keywords, "int");
  let templex = cast!(rule, IRulexPR::Templex);
  assert_templex_name(&templex, "int");
}
/*
  test("Kind matches plain Int") {
    compile("int") shouldHave {
      case TemplexPR(NameOrRunePT(NameP(_, StrI("int")))) =>
    }
  }
*/
#[test]
fn kind_with_value() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let rule = compile(&interner, &keywords, "T Kind = int");
  let equals = cast!(rule, IRulexPR::Equals);
  let left = cast!(equals.left.as_ref(), IRulexPR::Typed);
  assert_eq!(left.rune.as_ref().unwrap().as_str(), "T");
  assert_eq!(left.tyype, ITypePR::KindType);
  let right = cast!(equals.right.as_ref(), IRulexPR::Templex);
  assert_templex_name(right, "int");
}
/*
  test("Kind with value") {
    compile("T Kind = int") shouldHave {
      case EqualsPR(_,TypedPR(_,Some(NameP(_, StrI("T"))),KindTypePR),TemplexPR(NameOrRunePT(NameP(_, StrI("int"))))) =>
    }
  }
*/
#[test]
fn kind_with_sequence_in_value_spot() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let rule = compile(&interner, &keywords, "T Kind = (int, bool)");
  let equals = cast!(rule, IRulexPR::Equals);
  let left = cast!(equals.left.as_ref(), IRulexPR::Typed);
  assert_eq!(left.rune.as_ref().unwrap().as_str(), "T");
  assert_eq!(left.tyype, ITypePR::KindType);
  let right = cast!(equals.right.as_ref(), IRulexPR::Templex);
  let tuple = cast!(right, ITemplexPT::Tuple);
  let (int_, bool_) = expect_2(&tuple.elements);
  assert_templex_name(int_, "int");
  assert_templex_name(bool_, "bool");
}
/*
  test("Kind with sequence in value spot") {
    compile("T Kind = (int, bool)") shouldHave {
      case EqualsPR(_,
          TypedPR(_,Some(NameP(_, StrI("T"))),KindTypePR),
          TemplexPR(
            TuplePT(_,
              Vector(NameOrRunePT(NameP(_, StrI("int"))), NameOrRunePT(NameP(_, StrI("bool"))))))) =>
    }
  }
*/
#[test]
fn lone_sequence() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let rule = compile(&interner, &keywords, "(int, bool)");
  let templex = cast!(rule, IRulexPR::Templex);
  let tuple = cast!(templex, ITemplexPT::Tuple);
  let (int_, bool_) = expect_2(&tuple.elements);
  assert_templex_name(int_, "int");
  assert_templex_name(bool_, "bool");
}
/*
  test("Lone sequence") {
    compile("(int, bool)") shouldHave {
      case TemplexPR(
          TuplePT(_,
            Vector(NameOrRunePT(NameP(_, StrI("int"))), NameOrRunePT(NameP(_, StrI("bool")))))) =>
    }
  }
*/
#[test]
fn templated_struct_one_arg() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let rule = compile(&interner, &keywords, "Moo<int>");
  let templex = cast!(rule, IRulexPR::Templex);
  let call = cast!(templex, ITemplexPT::Call);
  assert_templex_name(call.template.as_ref(), "Moo");
  let arg = expect_1(&call.args);
  assert_templex_name(arg, "int");

  let rule = compile(&interner, &keywords, "Moo<@int>");
  let templex = cast!(rule, IRulexPR::Templex);
  let call = cast!(templex, ITemplexPT::Call);
  assert_templex_name(call.template.as_ref(), "Moo");
  let arg = expect_1(&call.args);
  let interpreted = cast!(arg, ITemplexPT::Interpreted);
  assert_eq!(
    interpreted.maybe_ownership.as_ref().unwrap().ownership,
    OwnershipP::Share
  );
  assert!(interpreted.maybe_region.is_none());
  assert_templex_name(interpreted.inner.as_ref(), "int");
}
/*
  test("Templated struct, one arg") {
    compile("Moo<int>") shouldHave {
      case TemplexPR(CallPT(_,NameOrRunePT(NameP(_, StrI("Moo"))),Vector(NameOrRunePT(NameP(_, StrI("int")))))) =>
    }
    compile("Moo<@int>") shouldHave {
      case TemplexPR(CallPT(_,NameOrRunePT(NameP(_, StrI("Moo"))),Vector(InterpretedPT(_,Some(OwnershipPT(_, ShareP)), None,NameOrRunePT(NameP(_, StrI("int"))))))) =>
    }
  }
*/
#[test]
fn rwkilc() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let rule = compile(&interner, &keywords, "List<int>");
  let templex = cast!(rule, IRulexPR::Templex);
  let call = cast!(templex, ITemplexPT::Call);
  assert_templex_name(call.template.as_ref(), "List");
  let arg = expect_1(&call.args);
  assert_templex_name(arg, "int");

  let rule = compile(&interner, &keywords, "K Int");
  let typed = cast!(rule, IRulexPR::Typed);
  assert_eq!(typed.rune.as_ref().unwrap().as_str(), "K");
  assert_eq!(typed.tyype, ITypePR::IntType);

  let rule = compile(&interner, &keywords, "K<int>");
  let templex = cast!(rule, IRulexPR::Templex);
  let call = cast!(templex, ITemplexPT::Call);
  assert_templex_name(call.template.as_ref(), "K");
  let arg = expect_1(&call.args);
  assert_templex_name(arg, "int");
}
/*
  test("RWKILC") {
    compile("List<int>") shouldHave {
      case TemplexPR(CallPT(_,NameOrRunePT(NameP(_, StrI("List"))),Vector(NameOrRunePT(NameP(_, StrI("int")))))) =>
    }
    compile("K Int") shouldHave {
        case TypedPR(_,Some(NameP(_, StrI("K"))),IntTypePR) =>
    }
    compile("K<int>") shouldHave {
        case TemplexPR(CallPT(_,NameOrRunePT(NameP(_, StrI("K"))),Vector(NameOrRunePT(NameP(_, StrI("int")))))) =>
    }
  }
*/
#[test]
fn templated_struct_rune_arg() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let rule = compile(&interner, &keywords, "Moo<R>");
  let templex = cast!(rule, IRulexPR::Templex);
  let call = cast!(templex, ITemplexPT::Call);
  assert_templex_name(call.template.as_ref(), "Moo");
  let arg = expect_1(&call.args);
  assert_templex_name(arg, "R");
}
/*
  test("Templated struct, rune arg") {
    // Make sure every pattern on the way down to kind can match Int
    compile("Moo<R>") shouldHave {
        case TemplexPR(CallPT(_,NameOrRunePT(NameP(_, StrI("Moo"))),Vector(NameOrRunePT(NameP(_, StrI("R")))))) =>
    }
  }
*/
#[test]
fn templated_struct_multiple_args() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let rule = compile(&interner, &keywords, "Moo<int, str>");
  let templex = cast!(rule, IRulexPR::Templex);
  let call = cast!(templex, ITemplexPT::Call);
  assert_templex_name(call.template.as_ref(), "Moo");
  let (int_, str_) = expect_2(&call.args);
  assert_templex_name(int_, "int");
  assert_templex_name(str_, "str");
}
/*
  test("Templated struct, multiple args") {
    // Make sure every pattern on the way down to kind can match Int
    compile("Moo<int, str>") shouldHave {
        case TemplexPR(CallPT(_,NameOrRunePT(NameP(_, StrI("Moo"))),Vector(NameOrRunePT(NameP(_, StrI("int"))), NameOrRunePT(NameP(_, StrI("str")))))) =>
    }
  }
*/
#[test]
fn templated_struct_arg_is_another_templated_struct_with_one_arg() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let rule = compile(&interner, &keywords, "Moo<Blarg<int>>");
  let templex = cast!(rule, IRulexPR::Templex);
  let call = cast!(templex, ITemplexPT::Call);
  assert_templex_name(call.template.as_ref(), "Moo");
  let nested = cast!(expect_1(&call.args), ITemplexPT::Call);
  assert_templex_name(nested.template.as_ref(), "Blarg");
  let arg = expect_1(&nested.args);
  assert_templex_name(arg, "int");
}
/*
  test("Templated struct, arg is another templated struct with one arg") {
    // Make sure every pattern on the way down to kind can match Int
    compile("Moo<Blarg<int>>") shouldHave {
        case TemplexPR(
          CallPT(_,
            NameOrRunePT(NameP(_, StrI("Moo"))),
            Vector(
                CallPT(_,
                  NameOrRunePT(NameP(_, StrI("Blarg"))),
                  Vector(NameOrRunePT(NameP(_, StrI("int")))))))) =>
    }
  }
*/
#[test]
fn templated_struct_arg_is_another_templated_struct_with_multiple_arg() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let rule = compile(&interner, &keywords, "Moo<Blarg<int, str>>");
  let templex = cast!(rule, IRulexPR::Templex);
  let call = cast!(templex, ITemplexPT::Call);
  assert_templex_name(call.template.as_ref(), "Moo");
  let nested = cast!(expect_1(&call.args), ITemplexPT::Call);
  assert_templex_name(nested.template.as_ref(), "Blarg");
  let (int_, str_) = expect_2(&nested.args);
  assert_templex_name(int_, "int");
  assert_templex_name(str_, "str");
}
/*
  test("Templated struct, arg is another templated struct with multiple arg") {
    // Make sure every pattern on the way down to kind can match Int
    compile("Moo<Blarg<int, str>>") shouldHave {
        case TemplexPR(
          CallPT(_,
            NameOrRunePT(NameP(_, StrI("Moo"))),
            Vector(
                CallPT(_,
                  NameOrRunePT(NameP(_, StrI("Blarg"))),
                  Vector(NameOrRunePT(NameP(_, StrI("int"))), NameOrRunePT(NameP(_, StrI("str")))))))) =>
    }
  }
*/
#[test]
fn static_sized_array() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let array = cast!(
    compile_templex_expect(&interner, &keywords, "[#_]_"),
    ITemplexPT::StaticSizedArray
  );
  assert_eq!(
    cast!(array.mutability.as_ref(), ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert_eq!(
    cast!(array.variability.as_ref(), ITemplexPT::Variability).variability,
    VariabilityP::Final
  );
  cast!(array.size.as_ref(), ITemplexPT::AnonymousRune);
  cast!(array.element.as_ref(), ITemplexPT::AnonymousRune);

  let array = cast!(
    compile_templex_expect(&interner, &keywords, "[#_]<imm>_"),
    ITemplexPT::StaticSizedArray
  );
  assert_eq!(
    cast!(array.mutability.as_ref(), ITemplexPT::Mutability).mutability,
    MutabilityP::Immutable
  );
  assert_eq!(
    cast!(array.variability.as_ref(), ITemplexPT::Variability).variability,
    VariabilityP::Final
  );
  cast!(array.size.as_ref(), ITemplexPT::AnonymousRune);
  cast!(array.element.as_ref(), ITemplexPT::AnonymousRune);

  let array = cast!(
    compile_templex_expect(&interner, &keywords, "[#3]int"),
    ITemplexPT::StaticSizedArray
  );
  assert_eq!(
    cast!(array.mutability.as_ref(), ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert_eq!(
    cast!(array.variability.as_ref(), ITemplexPT::Variability).variability,
    VariabilityP::Final
  );
  assert_eq!(cast!(array.size.as_ref(), ITemplexPT::Int).value, 3);
  assert_templex_name(array.element.as_ref(), "int");

  let array = cast!(
    compile_templex_expect(&interner, &keywords, "[#N]int"),
    ITemplexPT::StaticSizedArray
  );
  assert_eq!(
    cast!(array.mutability.as_ref(), ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert_eq!(
    cast!(array.variability.as_ref(), ITemplexPT::Variability).variability,
    VariabilityP::Final
  );
  assert_templex_name(array.size.as_ref(), "N");
  assert_templex_name(array.element.as_ref(), "int");

  let array = cast!(
    compile_templex_expect(&interner, &keywords, "[#_]int"),
    ITemplexPT::StaticSizedArray
  );
  assert_eq!(
    cast!(array.mutability.as_ref(), ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert_eq!(
    cast!(array.variability.as_ref(), ITemplexPT::Variability).variability,
    VariabilityP::Final
  );
  cast!(array.size.as_ref(), ITemplexPT::AnonymousRune);
  assert_templex_name(array.element.as_ref(), "int");

  let array = cast!(
    compile_templex_expect(&interner, &keywords, "[#N]T"),
    ITemplexPT::StaticSizedArray
  );
  assert_eq!(
    cast!(array.mutability.as_ref(), ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert_eq!(
    cast!(array.variability.as_ref(), ITemplexPT::Variability).variability,
    VariabilityP::Final
  );
  assert_templex_name(array.size.as_ref(), "N");
  assert_templex_name(array.element.as_ref(), "T");
}
/*
  test("Static sized array") {
    compileTemplex("[#_]_") shouldHave {
      case StaticSizedArrayPT(_,MutabilityPT(_,MutableP), VariabilityPT(_,FinalP), AnonymousRunePT(_),AnonymousRunePT(_)) =>
    }
    compileTemplex("[#_]<imm>_") shouldHave {
      case StaticSizedArrayPT(_,MutabilityPT(_,ImmutableP), VariabilityPT(_,FinalP), AnonymousRunePT(_),AnonymousRunePT(_)) =>
    }
    compileTemplex("[#3]int") shouldHave {
      case StaticSizedArrayPT(_,MutabilityPT(_,MutableP), VariabilityPT(_,FinalP), IntPT(_,3),NameOrRunePT(NameP(_, StrI("int")))) =>
    }
    compileTemplex("[#N]int") shouldHave {
        case StaticSizedArrayPT(_,MutabilityPT(_,MutableP), VariabilityPT(_,FinalP), NameOrRunePT(NameP(_, StrI("N"))),NameOrRunePT(NameP(_, StrI("int")))) =>
    }
    compileTemplex("[#_]int") shouldHave {
        case StaticSizedArrayPT(_,MutabilityPT(_,MutableP), VariabilityPT(_,FinalP), AnonymousRunePT(_),NameOrRunePT(NameP(_, StrI("int")))) =>
    }
    compileTemplex("[#N]T") shouldHave {
        case StaticSizedArrayPT(_,MutabilityPT(_,MutableP), VariabilityPT(_,FinalP), NameOrRunePT(NameP(_, StrI("N"))),NameOrRunePT(NameP(_, StrI("T")))) =>
    }
  }
*/
#[test]
fn regular_sequence() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let tuple = cast!(
    compile_templex_expect(&interner, &keywords, "()"),
    ITemplexPT::Tuple
  );
  assert_eq!(tuple.elements.len(), 0);

  let tuple = cast!(
    compile_templex_expect(&interner, &keywords, "(int)"),
    ITemplexPT::Tuple
  );
  assert_templex_name(expect_1(&tuple.elements), "int");

  let tuple = cast!(
    compile_templex_expect(&interner, &keywords, "(int, bool)"),
    ITemplexPT::Tuple
  );
  let (int_, bool_) = expect_2(&tuple.elements);
  assert_templex_name(int_, "int");
  assert_templex_name(bool_, "bool");

  let tuple = cast!(
    compile_templex_expect(&interner, &keywords, "(_, bool)"),
    ITemplexPT::Tuple
  );
  let (anonymous_, bool_) = expect_2(&tuple.elements);
  cast!(anonymous_, ITemplexPT::AnonymousRune);
  assert_templex_name(bool_, "bool");

  let tuple = cast!(
    compile_templex_expect(&interner, &keywords, "(_, _)"),
    ITemplexPT::Tuple
  );
  let (anonymous1_, anonymous2_) = expect_2(&tuple.elements);
  cast!(anonymous1_, ITemplexPT::AnonymousRune);
  cast!(anonymous2_, ITemplexPT::AnonymousRune);
}
/*
  test("Regular sequence") {
    compileTemplex("()") shouldHave {
        case TuplePT(_,Vector()) =>
    }
    compileTemplex("(int)") shouldHave {
        case TuplePT(_,Vector(NameOrRunePT(NameP(_, StrI("int"))))) =>
    }
    compileTemplex("(int, bool)") shouldHave {
        case TuplePT(_,Vector(NameOrRunePT(NameP(_, StrI("int"))), NameOrRunePT(NameP(_, StrI("bool"))))) =>
    }
    compileTemplex("(_, bool)") shouldHave {
        case TuplePT(_,Vector(AnonymousRunePT(_), NameOrRunePT(NameP(_, StrI("bool"))))) =>
    }
    compileTemplex("(_, _)") shouldHave {
        case TuplePT(_,Vector(AnonymousRunePT(_), AnonymousRunePT(_))) =>
    }
  }

//  test("Callable kind rule") {
//    compile(callableRulePR, "func(Int)Void") shouldHave {//        case FunctionPT(None,PackPT(Vector(NameOrRunePT(StringP(_, "int")))),NameOrRunePT(StringP(_, "void")))
//    compile(callableRulePR, "func(T)R") shouldHave {//        case FunctionPT(None,PackPT(Vector(NameOrRunePT(StringP(_, "T")))),NameOrRunePT(StringP(_, "R")))
//  }
*/
#[test]
fn prototype_kind_rule() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let templex = compile_templex_expect(&interner, &keywords, "func moo(int)void");
  let prototype = cast!(templex, ITemplexPT::Func);
  assert_eq!(prototype.name.as_str(), "moo");
  assert_templex_name(expect_1(&prototype.parameters), "int");
  assert_templex_name(prototype.return_type.as_ref(), "void");

  let templex = compile_templex_expect(&interner, &keywords, "func moo(T)R");
  let prototype = cast!(templex, ITemplexPT::Func);
  assert_eq!(prototype.name.as_str(), "moo");
  assert_templex_name(expect_1(&prototype.parameters), "T");
  assert_templex_name(prototype.return_type.as_ref(), "R");
}
/*
  test("Prototype kind rule") {
    compileTemplex("func moo(int)void") shouldHave {
        case FuncPT(_,NameP(_, StrI("moo")), _, Vector(NameOrRunePT(NameP(_, StrI("int")))),NameOrRunePT(NameP(_, StrI("void")))) =>
    }
    compileTemplex("func moo(T)R") shouldHave {
        case FuncPT(_,NameP(_, StrI("moo")), _, Vector(NameOrRunePT(NameP(_, StrI("T")))),NameOrRunePT(NameP(_, StrI("R")))) =>
    }
  }
}
*/