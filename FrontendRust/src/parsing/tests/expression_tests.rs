// cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing::tests::expression_tests

use crate::interner::StrI;
use crate::parse_arena::ParseArena;
use crate::keywords::Keywords;
use crate::lexing::errors::ParseError;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::*;
use bumpalo::Bump;

/*
// MIGALLOW: Rust code doesn't need to check bits == None like Scala does.

package dev.vale.parsing

import dev.vale.{Collector, StrI}
import dev.vale.lexing._
import dev.vale.parsing.ast._
import org.scalatest._

class ExpressionTests extends FunSuite with Collector with TestParseUtils {
*/
#[test]
fn simple_int() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "4");
  assert!(matches!(expr, IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. })));
}
/*
  test("Simple int") {
    val expr = compileExpressionExpect("4")
     expr shouldHave { case ConstantIntPE(_, 4, None) => }
  }
*/
#[test]
fn simple_bool() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "true");
  assert!(matches!(expr, IExpressionPE::ConstantBool(ConstantBoolPE { value: true, .. })));
}
/*
  test("Simple bool") {
    compileExpressionExpect("true") shouldHave
      { case ConstantBoolPE(_, true) => }
  }
*/
#[test]
fn i64() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "4i64");
  assert!(matches!(
    expr,
    IExpressionPE::ConstantInt(ConstantIntPE { value: 4, bits: Some(64), .. })
  ));
}
/*
  test("i64") {
    compileExpressionExpect("4i64") shouldHave
      { case ConstantIntPE(_, 4L, Some(64L)) => }
  }
*/
#[test]
fn binary_operator() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "4 + 5");
  match &expr {
    IExpressionPE::BinaryCall(BinaryCallPE {
      function_name: NameP(_, StrI("+")),
      left_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }),
      right_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
      ..
    }) => {}
    _ => panic!("expected 4 + 5 structure"),
  }
}
/*
  test("Binary operator") {
    val expr = compileExpressionExpect("4 + 5")
    expr shouldHave
      { case BinaryCallPE(_,NameP(_,StrI("+")),ConstantIntPE(_,4,_),ConstantIntPE(_,5,_)) => }
  }
*/
#[test]
fn floats() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "4.2");
  assert!(matches!(expr, IExpressionPE::ConstantFloat(ConstantFloatPE { value: 4.2, .. })));
}
/*
  test("Floats") {
    compileExpressionExpect("4.2") shouldHave
      { case ConstantFloatPE(_, 4.2) => }
  }
*/
#[test]
fn number_range() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "0..5");
  match &expr {
    IExpressionPE::Range(RangePE {
      from_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 0, .. }),
      to_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
      ..
    }) => {}
    _ => panic!("expected 0..5 structure"),
  }
}
/*
  test("Number range") {
    compileExpressionExpect("0..5") shouldHave
      { case RangePE(_,ConstantIntPE(_,0,_),ConstantIntPE(_,5,_)) => }
  }
*/
#[test]
fn add_as_call() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "+(4, 5)");
  match &expr {
    IExpressionPE::FunctionCall(FunctionCallPE {
      callable_expr:
        IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("+"))),
          template_args: None,
        }),
      arg_exprs,
      ..
    }) if matches!(
      arg_exprs,
      [
        IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }),
        IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
        ..
      ]
    ) => {}
    _ => panic!("expected +(4, 5) structure"),
  }
}
/*
  test("add as call") {
    compileExpressionExpect("+(4, 5)") shouldHave
      { case FunctionCallPE(_,_,LookupPE(LookupNameP(NameP(_, StrI("+"))), None), Vector(ConstantIntPE(_, 4, _), ConstantIntPE(_, 5, _))) => }
  }
*/
#[test]
fn passing_eq_overload_set() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "moo(4, ==)");
  match &expr {
    IExpressionPE::FunctionCall(FunctionCallPE {
      callable_expr:
        IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("moo"))),
          ..
        }),
      arg_exprs:
        [IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }), IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("=="))),
          template_args: None,
        }), ..],
      ..
    }) => {}
    _ => panic!("expected moo(4, ==) structure"),
  }
}
/*
  // MIGALLOW: Rust code can check for the 4 value.
  test("Passing == overload set") {
    compileExpressionExpect("moo(4, ==)") shouldHave {
      case FunctionCallPE(_,_,
        _,
        Vector(
          _,
          LookupPE(LookupNameP(NameP(_,StrI("=="))),None))) =>
    }
  }
*/
#[test]
fn call_then_binary_operator() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "str(i) + 5");
  match &expr {
    IExpressionPE::BinaryCall(BinaryCallPE {
      function_name: NameP(_, StrI("+")),
      left_expr:
        IExpressionPE::FunctionCall(FunctionCallPE {
          callable_expr:
            IExpressionPE::Lookup(LookupPE {
              name: IImpreciseNameP::LookupName(NameP(_, StrI("str"))),
              template_args: None,
            }),
          arg_exprs,
          ..
        }),
      right_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
      ..
    }) => match arg_exprs {
      [IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("i"))),
        template_args: None,
      })] => {}
      _ => panic!("expected str(i) + 5 structure"),
    },
    _ => panic!("expected str(i) + 5 structure"),
  }
}
/*
  test("Call then binary operator") {
    compileExpressionExpect("str(i) + 5") shouldHave {
      case BinaryCallPE(_,
        NameP(_,StrI("+")),
        FunctionCallPE(_,
          _,
          LookupPE(LookupNameP(NameP(_,StrI("str"))),None),
          Vector(LookupPE(LookupNameP(NameP(_,StrI("i"))),None))),
        ConstantIntPE(_,5,None)) =>
    }
  }
*/
#[test]
fn range() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "a..b");
  match &expr {
    IExpressionPE::Range(RangePE {
      from_expr:
        IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("a"))),
          template_args: None,
        }),
      to_expr:
        IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("b"))),
          template_args: None,
        }),
      ..
    }) => {}
    _ => panic!("expected a..b structure"),
  }
}
/*
  test("range") {
    compileExpressionExpect("a..b") shouldHave
      { case RangePE(_,LookupPE(LookupNameP(NameP(_,StrI("a"))),None),LookupPE(LookupNameP(NameP(_,StrI("b"))),None)) =>}
  }
*/
#[test]
fn regular_call() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "x(y)");
  match &expr {
    IExpressionPE::FunctionCall(FunctionCallPE {
      callable_expr:
        IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
          template_args: None,
        }),
      arg_exprs,
      ..
    }) => match arg_exprs {
      [IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("y"))),
        template_args: None,
      })] => {}
      _ => panic!("expected x(y) structure"),
    },
    _ => panic!("expected x(y) structure"),
  }
}
/*
  test("regular call") {
    compileExpressionExpect("x(y)") shouldHave
      { case FunctionCallPE(_,_,LookupPE(LookupNameP(NameP(_, StrI("x"))), None), Vector(LookupPE(LookupNameP(NameP(_, StrI("y"))), None))) => }
  }
*/
#[test]
fn not() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "not y");
  match &expr {
    IExpressionPE::Not(NotPE {
      inner:
        IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("y"))),
          template_args: None,
        }),
      ..
    }) => {}
    _ => panic!("expected not y structure"),
  }
}
/*
  test("not") {
    compileExpressionExpect("not y") shouldHave
      { case NotPE(_,LookupPE(LookupNameP(NameP(_,StrI("y"))),None)) => }
  }
*/
#[test]
fn borrowing_result_of_function_call() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "&Muta()");
  match &expr {
    IExpressionPE::Augment(AugmentPE {
      target_ownership: OwnershipP::Borrow,
      inner:
        IExpressionPE::FunctionCall(FunctionCallPE {
          callable_expr:
            IExpressionPE::Lookup(LookupPE {
              name: IImpreciseNameP::LookupName(NameP(_, StrI("Muta"))),
              template_args: None,
            }),
          arg_exprs,
          ..
        }),
      ..
    }) if arg_exprs.is_empty() => {}
    _ => panic!("expected &Muta() structure"),
  }
}
/*
  test("Borrowing result of function call") {
    compileExpressionExpect("&Muta()") shouldHave
      { case AugmentPE(_,BorrowP,FunctionCallPE(_,_,LookupPE(LookupNameP(NameP(_,StrI("Muta"))),None),Vector())) => }
  }
*/
#[test]
fn specifying_heap() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "^Muta()");
  match &expr {
    IExpressionPE::Augment(AugmentPE {
      target_ownership: OwnershipP::Own,
      inner: IExpressionPE::FunctionCall(FunctionCallPE { .. }),
      ..
    }) => {}
    _ => panic!("expected ^Muta() structure"),
  }
}
/*
  test("Specifying heap") {
    compileExpressionExpect("^Muta()") shouldHave
      { case AugmentPE(_,OwnP,FunctionCallPE(_,_,_,_)) => }
  }
*/
#[test]
fn inline_call_ignored() {
  // The inl keyword is just parsed as an Own augment. It's effectively a no-op.
  // This is probably to better syntax-highlight the inl keyword even though we ignore it.
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "inl Muta()");
  match &expr {
    IExpressionPE::Augment(AugmentPE {
      target_ownership: OwnershipP::Own,
      inner:
        IExpressionPE::FunctionCall(FunctionCallPE {
          callable_expr:
            IExpressionPE::Lookup(LookupPE {
              name: IImpreciseNameP::LookupName(NameP(_, StrI("Muta"))),
              template_args: None,
            }),
          arg_exprs,
          ..
        }),
      ..
    }) if arg_exprs.is_empty() => {}
    _ => panic!("expected inl Muta() structure"),
  }
}
/*
  // MIGALLOW: Rust can check for the Augment with Own.
  test("inline call ignored") {
    compileExpressionExpect("inl Muta()") shouldHave
      { case FunctionCallPE(_,_,LookupPE(LookupNameP(NameP(_, StrI("Muta"))),None),Vector()) => }
  }
*/
#[test]
fn method_call() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "x . shout ()");
  match &expr {
    IExpressionPE::MethodCall(MethodCallPE {
      subject_expr:
        IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
          template_args: None,
        }),
      method_lookup:
        LookupPE { name: IImpreciseNameP::LookupName(NameP(_, StrI("shout"))), template_args: None },
      arg_exprs,
      ..
    }) if arg_exprs.is_empty() => {}
    _ => panic!("expected x . shout () structure"),
  }
}
/*
  test("Method call") {
    compileExpressionExpect("x . shout ()") shouldHave
      { case MethodCallPE(_,LookupPE(LookupNameP(NameP(_,StrI("x"))),None),_,LookupPE(LookupNameP(NameP(_,StrI("shout"))),None),Vector()) => }
  }
*/
#[test]
fn mapping_method_call() {
  // These arent implemented yet, we currently just parse these as method calls to support
  // snippets on the site.
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "x *. shout ()");
  match &expr {
    IExpressionPE::MethodCall(MethodCallPE {
      subject_expr:
        IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
          template_args: None,
        }),
      method_lookup:
        LookupPE { name: IImpreciseNameP::LookupName(NameP(_, StrI("shout"))), template_args: None },
      arg_exprs,
      ..
    }) if arg_exprs.is_empty() => {}
    _ => panic!("expected x *. shout () structure"),
  }
}
/*
  test("Mapping method call") {
    // These arent implemented yet, we currently just parse these as method calls to support
    // snippets on the site.
    compileExpressionExpect("x *. shout ()") shouldHave
      { case MethodCallPE(_,LookupPE(LookupNameP(NameP(_,StrI("x"))),None),_,LookupPE(LookupNameP(NameP(_,StrI("shout"))),None),Vector()) => }
  }
*/
#[test]
fn method_on_member() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "x.moo.shout()");
  match &expr {
    IExpressionPE::MethodCall(MethodCallPE {
      subject_expr:
        IExpressionPE::Dot(DotPE {
          left:
            IExpressionPE::Lookup(LookupPE {
              name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
              template_args: None,
            }),
          member: NameP(_, StrI("moo")),
          ..
        }),
      method_lookup:
        LookupPE { name: IImpreciseNameP::LookupName(NameP(_, StrI("shout"))), template_args: None },
      arg_exprs,
      ..
    }) if arg_exprs.is_empty() => {}
    _ => panic!("expected x.moo.shout() structure"),
  }
}
/*
  test("Method on member") {
    compileExpressionExpect("x.moo.shout()") shouldHave
      {
        case MethodCallPE(_,
          DotPE(_, LookupPE(LookupNameP(NameP(_, StrI("x"))),None), _, NameP(_,StrI("moo"))),
          _,
          LookupPE(LookupNameP(NameP(_, StrI("shout"))),None),
          Vector()) =>
      }
  }
*/
#[test]
fn moving_method_call() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "(x ).shout()");
  match &expr {
    IExpressionPE::MethodCall(MethodCallPE {
      subject_expr:
        IExpressionPE::SubExpression(SubExpressionPE {
          inner:
            IExpressionPE::Lookup(LookupPE {
              name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
              template_args: None,
            }),
          ..
        }),
      method_lookup:
        LookupPE { name: IImpreciseNameP::LookupName(NameP(_, StrI("shout"))), template_args: None },
      arg_exprs,
      ..
    }) if arg_exprs.is_empty() => {}
    _ => panic!("expected (x ).shout() structure"),
  }
}
/*
  test("Moving method call") {
    compileExpressionExpect("(x ).shout()") shouldHave
      {
      case MethodCallPE(_,
        SubExpressionPE(_, LookupPE(LookupNameP(NameP(_, StrI("x"))),None)),
        _,
        LookupPE(LookupNameP(NameP(_, StrI("shout"))),None),
        Vector()) =>
    }
  }
*/
/*
//  test("Map method call") {
//    compileExpression("x*. shout()") shouldHave
//      {
//      case MethodCallPE(_,
//      LookupPE(LookupNameP(NameP(_, StrI("x"))),None),
//      _,false,
//      LookupPE(LookupNameP(NameP(_, StrI("shout"))),None),
//      Vector()) =>
//    }
//  }
*/
#[test]
fn templated_function_call() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr =
    compile_expression_expect(&parse_arena, &keywords, "toArray<imm>( &result)");
  match &expr {
    IExpressionPE::FunctionCall(FunctionCallPE {
      callable_expr:
        IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("toArray"))),
          template_args: Some(TemplateArgsP { args, .. }),
        }),
      arg_exprs,
      ..
    }) => match (args, arg_exprs) {
      (
        [ITemplexPT::Mutability(MutabilityPT(_, MutabilityP::Immutable)), ..],
        [IExpressionPE::Augment(AugmentPE {
          target_ownership: OwnershipP::Borrow,
          inner:
            IExpressionPE::Lookup(LookupPE {
              name: IImpreciseNameP::LookupName(NameP(_, StrI("result"))),
              template_args: None,
            }),
          ..
        })],
      ) => {}
      _ => panic!("expected toArray<imm>( &result) structure"),
    },
    _ => panic!("expected toArray<imm>( &result) structure"),
  }
}
/*
  test("Templated function call") {
    compileExpressionExpect("toArray<imm>( &result)") shouldHave
      {
        case FunctionCallPE(_,_,
        LookupPE(LookupNameP(NameP(_,StrI("toArray"))),Some(TemplateArgsP(_,Vector(MutabilityPT(_,ImmutableP))))),
        Vector(AugmentPE(_,BorrowP,LookupPE(LookupNameP(NameP(_,StrI("result"))),None)))) =>
      }
  }
*/
#[test]
fn templated_method_call() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr =
    compile_expression_expect(&parse_arena, &keywords, "result.toArray <imm> ()");
  match &expr {
    IExpressionPE::MethodCall(MethodCallPE {
      subject_expr:
        IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("result"))),
          template_args: None,
        }),
      method_lookup:
        LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("toArray"))),
          template_args: Some(TemplateArgsP { args, .. }),
        },
      arg_exprs,
      ..
    }) if arg_exprs.is_empty() => match args {
      [ITemplexPT::Mutability(MutabilityPT(_, MutabilityP::Immutable)), ..] => {}
      _ => panic!("expected result.toArray <imm> () structure"),
    },
    _ => panic!("expected result.toArray <imm> () structure"),
  }
}
/*
  test("Templated method call") {
    compileExpressionExpect("result.toArray <imm> ()") shouldHave
      {
      case MethodCallPE(_,LookupPE(LookupNameP(NameP(_, StrI("result"))),None),_,LookupPE(LookupNameP(NameP(_, StrI("toArray"))),Some(TemplateArgsP(_, Vector(MutabilityPT(_,ImmutableP))))),Vector()) =>
    }
  }
*/
#[test]
fn custom_binaries() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "not y florgle not x");
  match &expr {
    IExpressionPE::BinaryCall(BinaryCallPE {
      function_name: NameP(_, StrI("florgle")),
      left_expr:
        IExpressionPE::Not(NotPE {
          inner:
            IExpressionPE::Lookup(LookupPE {
              name: IImpreciseNameP::LookupName(NameP(_, StrI("y"))),
              template_args: None,
            }),
          ..
        }),
      right_expr:
        IExpressionPE::Not(NotPE {
          inner:
            IExpressionPE::Lookup(LookupPE {
              name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
              template_args: None,
            }),
          ..
        }),
      ..
    }) => {}
    _ => panic!("expected not y florgle not x structure"),
  }
}
/*
  test("Custom binaries") {
    compileExpressionExpect("not y florgle not x") shouldHave
      { case BinaryCallPE(_,NameP(_,StrI("florgle")),NotPE(_,LookupPE(LookupNameP(NameP(_,StrI("y"))),None)),NotPE(_,LookupPE(LookupNameP(NameP(_,StrI("x"))),None))) => }
  }
*/
#[test]
fn custom_with_noncustom_binaries() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "a + b florgle x * y");

  match &expr {
    IExpressionPE::BinaryCall(BinaryCallPE {
      function_name: NameP(_, StrI("florgle")),
      left_expr:
        IExpressionPE::BinaryCall(BinaryCallPE {
          function_name: NameP(_, StrI("+")),
          left_expr:
            IExpressionPE::Lookup(LookupPE {
              name: IImpreciseNameP::LookupName(NameP(_, StrI("a"))),
              template_args: None,
            }),
          right_expr:
            IExpressionPE::Lookup(LookupPE {
              name: IImpreciseNameP::LookupName(NameP(_, StrI("b"))),
              template_args: None,
            }),
          ..
        }),
      right_expr:
        IExpressionPE::BinaryCall(BinaryCallPE {
          function_name: NameP(_, StrI("*")),
          left_expr:
            IExpressionPE::Lookup(LookupPE {
              name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
              template_args: None,
            }),
          right_expr:
            IExpressionPE::Lookup(LookupPE {
              name: IImpreciseNameP::LookupName(NameP(_, StrI("y"))),
              template_args: None,
            }),
          ..
        }),
      ..
    }) => {}
    _ => panic!("expected a + b florgle x * y structure"),
  }
}
/*
  test("Custom with noncustom binaries") {
    compileExpressionExpect("a + b florgle x * y") shouldHave
      {
        case BinaryCallPE(_,
          NameP(_,StrI("florgle")),
          BinaryCallPE(_,
            NameP(_,StrI("+")),
            LookupPE(LookupNameP(NameP(_,StrI("a"))),None),
            LookupPE(LookupNameP(NameP(_,StrI("b"))),None)),
          BinaryCallPE(_,
            NameP(_,StrI("*")),
            LookupPE(LookupNameP(NameP(_,StrI("x"))),None),
            LookupPE(LookupNameP(NameP(_,StrI("y"))),None))) =>
      }
  }
*/
#[test]
fn template_calling() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  {
    let expr = compile_expression_expect(&parse_arena, &keywords, "MyNone< int >()");
    match &expr {
      IExpressionPE::FunctionCall(FunctionCallPE {
        callable_expr: IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("MyNone"))),
          template_args: Some(TemplateArgsP {
            args: [ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("int")))), ..],
            ..
          }),
        }),
        arg_exprs: [],
        ..
      }) => {}
      _ => panic!("expected MyNone<int>() structure"),
    }
  }

  {
    let expr =
      compile_expression_expect(&parse_arena, &keywords, "MySome< MyNone <int> >()");
    match &expr {
      IExpressionPE::FunctionCall(FunctionCallPE {
        callable_expr: IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("MySome"))),
          template_args: Some(TemplateArgsP {
            args: [ITemplexPT::Call(CallPT {
              template: ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("MyNone")))),
              args: [ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("int")))), ..],
              ..
            }), ..],
            ..
          }),
        }),
        arg_exprs: [],
        ..
      }) =>
      {}
      _ => panic!("expected MySome<MyNone<int>>() structure"),
    }
  }
}
/*
  test("Template calling") {
    compileExpressionExpect("MyNone< int >()") shouldHave
      {
        case FunctionCallPE(_,_,LookupPE(LookupNameP(NameP(_, StrI("MyNone"))),Some(TemplateArgsP(_,Vector(NameOrRunePT(NameP(_,StrI("int"))))))),Vector()) =>
    }
    compileExpressionExpect("MySome< MyNone <int> >()") shouldHave
      {
      case FunctionCallPE(_,_,LookupPE(LookupNameP(NameP(_, StrI("MySome"))), Some(TemplateArgsP(_, Vector(CallPT(_,NameOrRunePT(NameP(_, StrI("MyNone"))),Vector(NameOrRunePT(NameP(_, StrI("int"))))))))),Vector()) =>
    }
  }
*/
#[test]
fn greater_than_or_equal() {
  // It turns out, this was only parsing "9 >=" because it was looking for > specifically (in fact, it was looking
  // for + - * / < >) so it parsed as >(9, =) which was bad. We changed the infix operator parser to expect the
  // whitespace on both sides, so that it was forced to parse the entire thing.
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "9 >= 3");
  match &expr {
    IExpressionPE::BinaryCall(BinaryCallPE {
      function_name: NameP(_, StrI(">=")),
      left_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 9, .. }),
      right_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
      ..
    }) => {}
    _ => panic!("expected 9 >= 3 structure"),
  }
}
/*
  test(">=") {
    // It turns out, this was only parsing "9 >=" because it was looking for > specifically (in fact, it was looking
    // for + - * / < >) so it parsed as >(9, =) which was bad. We changed the infix operator parser to expect the
    // whitespace on both sides, so that it was forced to parse the entire thing.
    compileExpressionExpect("9 >= 3") shouldHave
      {
        case BinaryCallPE(_,NameP(_,StrI(">=")),ConstantIntPE(_,9,_),ConstantIntPE(_,3,_)) =>
    }
  }
*/
#[test]
fn indexing() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "arr [4]");
  match &expr {
    IExpressionPE::BraceCall(BraceCallPE {
      subject_expr: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("arr"))),
        template_args: None,
      }),
      arg_exprs: [IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }), ..],
      ..
    }) => {}
    _ => panic!("expected arr [4] structure"),
  }
}
/*
  test("Indexing") {
    compileExpressionExpect("arr [4]") shouldHave
      { case BraceCallPE(_,_,LookupPE(LookupNameP(NameP(_,StrI("arr"))),None),Vector(ConstantIntPE(_,4,_)),_) => }
  }
*/
#[test]
fn single_arg_brace_lambda() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "x => { x }");
  match &expr {
    IExpressionPE::Lambda(LambdaPE {
      function: FunctionP {
        header: FunctionHeaderP {
          params: Some(ParamsP {
            params: [ParameterP {
              pattern: Some(PatternPP {
                destination: Some(DestinationLocalP {
                  decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("x"))),
                  ..
                }),
                ..
              }),
              ..
            }, ..],
            ..
          }),
          ..
        },
        body: Some(BlockPE {
          inner: IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
            template_args: None,
          }),
          ..
        }),
        ..
      },
      ..
    }) => {}
    _ => panic!("expected x => {{ x }} structure"),
  }
}
/*
  // MIGALLOW: Rust code doesn't need to check all the None values like Scala does.
  test("Single arg brace lambda") {
    compileExpressionExpect("x => { x }") shouldHave
      {
        case LambdaPE(_,
          FunctionP(_,
            FunctionHeaderP(_,
              None,Vector(),None,None,
              Some(ParamsP(_,Vector(ParameterP(_,None,None,None,Some(PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_,StrI("x"))), None)),None,None)))))),
              FunctionReturnP(_,None)),
            Some(BlockPE(_,_,_,LookupPE(LookupNameP(NameP(_,StrI("x"))),None))))) =>
      }
  }
*/
#[test]
fn single_arg_no_brace_lambda() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "x => x");
  match &expr {
    IExpressionPE::Lambda(LambdaPE {
      function: FunctionP {
        header: FunctionHeaderP {
          params: Some(ParamsP {
            params: [ParameterP {
              pattern: Some(PatternPP {
                destination: Some(DestinationLocalP {
                  decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("x"))),
                  ..
                }),
                ..
              }),
              ..
            }, ..],
            ..
          }),
          ..
        },
        body: Some(BlockPE {
          inner: IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
            template_args: None,
          }),
          ..
        }),
        ..
      },
      ..
    }) => {}
    _ => panic!("expected x => x structure"),
  }
}
/*
  test("Single arg no-brace lambda") {
    compileExpressionExpect("x => x") shouldHave
      {
        case LambdaPE(_,
          FunctionP(_,
            FunctionHeaderP(_,
              None,Vector(),None,None,
              Some(ParamsP(_,Vector(ParameterP(_,None,None,None,Some(PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_,StrI("x"))), None)),None,None)))))),
              FunctionReturnP(_,None)),
            Some(BlockPE(_,_,_,LookupPE(LookupNameP(NameP(_,StrI("x"))),None))))) =>
      }
  }
*/
#[test]
fn single_arg_typed_brace_lambda() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "(x int) => { x }");
  match &expr {
    IExpressionPE::Lambda(LambdaPE {
      function: FunctionP {
        header: FunctionHeaderP {
          params: Some(ParamsP {
            params: [ParameterP {
              pattern: Some(PatternPP {
                destination: Some(DestinationLocalP {
                  decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("x"))),
                  ..
                }),
                templex: Some(ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("int"))))),
                ..
              }),
              ..
            }, ..],
            ..
          }),
          ..
        },
        body: Some(BlockPE {
          inner: IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
            template_args: None,
          }),
          ..
        }),
        ..
      },
      ..
    }) => {}
    _ => panic!("expected (x int) => {{ x }} structure"),
  }
}
/*
  test("Single arg typed brace lambda") {
    compileExpressionExpect("(x int) => { x }") shouldHave
      {
        case LambdaPE(_,
          FunctionP(_,
            FunctionHeaderP(_,
              None,Vector(),None,None,
              Some(ParamsP(_,Vector(ParameterP(_,None,None,None,Some(PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_,StrI("x"))), None)),Some(NameOrRunePT(NameP(_,StrI("int")))), None)))))),
              _),
          _)) =>
      }
  }
*/
#[test]
fn argless_lambda() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "{_}");
  match &expr {
    IExpressionPE::Lambda(LambdaPE {
      function: FunctionP {
        header: FunctionHeaderP { params: None, ret: FunctionReturnP { ret_type: None, .. }, .. },
        body: Some(BlockPE { inner: IExpressionPE::MagicParamLookup(_), .. }),
        ..
      },
      ..
    }) => {}
    _ => panic!("expected {{_}} structure"),
  }
}
/*
  test("Argless lambda") {
    compileExpressionExpect("{_}") shouldHave
      {
        case LambdaPE(
          None,
          FunctionP(_,
            FunctionHeaderP(_,
              None,Vector(),None,None,None,FunctionReturnP(_,None)),
              Some(BlockPE(_,_,_,MagicParamLookupPE(_))))) =>
      }
  }
*/
#[test]
fn multi_arg_typed_brace_lambda() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "(x, y) => x");
  match &expr {
    IExpressionPE::Lambda(LambdaPE {
      function: FunctionP {
        header: FunctionHeaderP {
          params: Some(ParamsP {
            params: [
              ParameterP {
                pattern: Some(PatternPP {
                  destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("x"))),
                    ..
                  }),
                  ..
                }),
                ..
              },
              ParameterP {
                pattern: Some(PatternPP {
                  destination: Some(DestinationLocalP {
                    decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("y"))),
                    ..
                  }),
                  ..
                }),
                ..
              },
            ],
            ..
          }),
          ..
        },
        body: Some(BlockPE {
          inner: IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
            template_args: None,
          }),
          ..
        }),
        ..
      },
      ..
    }) => {}
    _ => panic!("expected (x, y) => x structure"),
  }
}
/*
  test("Multi arg typed brace lambda") {
    compileExpressionExpect("(x, y) => x") shouldHave
      {
        case LambdaPE(
          None,
          FunctionP(_,
            FunctionHeaderP(_,
              None,Vector(),None,None,
              Some(
                ParamsP(_,
                  Vector(
                    ParameterP(_,None,None,None,Some(PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_,StrI("x"))), None)),None,None))),
                    ParameterP(_,None,None,None,Some(PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_,StrI("y"))), None)),None,None)))))),
              FunctionReturnP(_,None)),
            Some(BlockPE(_,_,_,LookupPE(LookupNameP(NameP(_,StrI("x"))),None))))) =>
      }
  }
*/
#[test]
fn destructuring_lambda() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "([x, y]) => x");
  match &expr {
    IExpressionPE::Lambda(LambdaPE {
      function: FunctionP {
        header: FunctionHeaderP {
          params: Some(ParamsP {
            params: [ParameterP {
              pattern: Some(PatternPP {
                destination: None,
                templex: None,
                destructure: Some(DestructureP {
                  patterns: [
                    PatternPP {
                      destination: Some(DestinationLocalP {
                        decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("x"))),
                        ..
                      }),
                      ..
                    },
                    PatternPP {
                      destination: Some(DestinationLocalP {
                        decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("y"))),
                        ..
                      }),
                      ..
                    },
                  ],
                  ..
                }),
                ..
              }),
              ..
            }, ..],
            ..
          }),
          ..
        },
        body: Some(BlockPE {
          inner: IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
            template_args: None,
          }),
          ..
        }),
        ..
      },
      ..
    }) => {}
    _ => panic!("expected ([x, y]) => x structure"),
  }
}
/*
  test("Destructuring lambda") {
    compileExpressionExpect("([x, y]) => x") shouldHave
      {
        case LambdaPE(
          None,
          FunctionP(_,
            FunctionHeaderP(_,
              None,Vector(),None,None,
              Some(
                ParamsP(_,
                  Vector(
                    ParameterP(_,
                      None,None,None,
                      Some(
                        PatternPP(_,
                          None,None,
                          Some(
                            DestructureP(_,
                              Vector(
                                PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_,StrI("x"))), None)),None,None),
                                PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_,StrI("y"))), None)),None,None)))))))))),
              FunctionReturnP(_,None)),
            Some(BlockPE(_,_,_,LookupPE(LookupNameP(NameP(_,StrI("x"))),None))))) =>
      }
  }
*/
#[test]
fn dot_symbol() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, r#"myPath./("subdir")"#);
  match &expr {
    IExpressionPE::MethodCall(MethodCallPE {
      subject_expr: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("myPath"))),
        template_args: None,
      }),
      method_lookup: LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("/"))),
        template_args: None,
      },
      arg_exprs: [IExpressionPE::ConstantStr(ConstantStrPE { value: StrI("subdir"), .. }), ..],
      ..
    }) => {}
    _ => panic!("expected myPath./(\"subdir\") structure"),
  }
}
/*
  test("dot symbol") {
    compileExpressionExpect("""myPath./("subdir")""") shouldHave
      {
        case MethodCallPE(_,
          LookupPE(LookupNameP(NameP(_,StrI("myPath"))),None),
          _,
          LookupPE(LookupNameP(NameP(_,StrI("/"))),None),
          Vector(ConstantStrPE(_,"subdir"))) =>
      }
  }
*/
#[test]
fn not_equal() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "3 != 4");
  match &expr {
    IExpressionPE::BinaryCall(BinaryCallPE {
      function_name: NameP(_, StrI("!=")),
      left_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
      right_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }),
      ..
    }) => {}
    _ => panic!("expected 3 != 4 structure"),
  }
}
/*
  test("!=") {
    compileExpressionExpect("3 != 4") shouldHave
      {
        case BinaryCallPE(_,NameP(_,StrI("!=")),ConstantIntPE(_,3,_),ConstantIntPE(_,4,_)) =>
    }
  }
*/
#[test]
fn set_call_isnt_interpreted_as_a_set_expression() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "set(true)");
  match &expr {
    IExpressionPE::FunctionCall(FunctionCallPE {
      callable_expr: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("set"))),
        template_args: None,
      }),
      arg_exprs: [IExpressionPE::ConstantBool(ConstantBoolPE { value: true, .. }), ..],
      ..
    }) => {}
    _ => panic!("expected set(true) structure"),
  }
}
/*
  // MIGALLOW: Rust can check for the argument too.
  test("set call isn't interpreted as a set expression") {
    compileExpressionExpect("set(true)") shouldHave {
      case FunctionCallPE(_,_, LookupPE(LookupNameP(NameP(_,StrI("set"))),None), _) =>
    }
  }
*/
#[test]
fn two_d_array_access() {
  // We had a bug where the lexer was interpreting that 2.1 as a float.
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "arr.2.1");
  match &expr {
    IExpressionPE::Dot(DotPE {
      left: IExpressionPE::Dot(DotPE {
        left: IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("arr"))),
          template_args: None,
        }),
        member: NameP(_, StrI("2")),
        ..
      }),
      member: NameP(_, StrI("1")),
      ..
    }) => {}
    _ => panic!("expected arr.2.1 structure"),
  }
}
/*
  test("2D array access") {
    // We had a bug where the lexer was interpreting that 2.1 as a float.
    compileExpressionExpect("arr.2.1") shouldHave {
      case DotPE(_,
        DotPE(_,
          LookupPE(LookupNameP(NameP(_,StrI("arr"))),None),
          _,
          NameP(_,StrI("2"))),
        _,
        NameP(_,StrI("1"))) =>
    }
  }
*/
#[test]
fn lambda_without_surrounding_parens() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "{ 0 }()");
  match &expr {
    IExpressionPE::FunctionCall(FunctionCallPE {
      callable_expr: IExpressionPE::Lambda(_),
      arg_exprs: [],
      ..
    }) => {}
    _ => panic!("expected {{ 0 }}() structure"),
  }
}
/*
  test("lambda without surrounding parens") {
    compileExpressionExpect("{ 0 }()") shouldHave
      {
      case FunctionCallPE(_,_,LambdaPE(None,_),Vector()) =>
    }
  }
*/
#[test]
fn function_call() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "call(sum)");
  match &expr {
    IExpressionPE::FunctionCall(FunctionCallPE {
      callable_expr: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("call"))),
        template_args: None,
      }),
      arg_exprs: [IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("sum"))),
        template_args: None,
      }), ..],
      ..
    }) => {}
    _ => panic!("expected call(sum) structure"),
  }
}
/*
  test("Function call") {
    val program = compileExpressionExpect("call(sum)")
    //    val main = program.lookupFunction("main")

    program shouldHave
      {
      case FunctionCallPE(_, _, LookupPE(LookupNameP(NameP(_, StrI("call"))), None),Vector(LookupPE(LookupNameP(NameP(_, StrI("sum"))), None))) =>
    }
  }
*/
#[test]
fn test_inner_expression_unlet() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "destroy(unlet enemy)");
  match &expr {
    IExpressionPE::FunctionCall(FunctionCallPE {
      callable_expr: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("destroy"))),
        template_args: None,
      }),
      arg_exprs: [IExpressionPE::Unlet(UnletPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("enemy"))),
        ..
      }), ..],
      ..
    }) => {}
    _ => panic!("expected destroy(unlet enemy) structure"),
  }
}
/*
  test("Test inner expression unlet") {
    val program = compileExpressionExpect("destroy(unlet enemy)")

    program shouldHave {
      case FunctionCallPE(_, _, LookupPE(LookupNameP(NameP(_, StrI("destroy"))), None),Vector(UnletPE(_, LookupNameP(NameP(_, StrI("enemy")))))) =>
    }
  }
*/
#[test]
fn detect_break_in_expr() {
  // See BRCOBS
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let err = compile_expression_for_error(&parse_arena, &keywords, "a(b, break)");
  assert!(matches!(err, ParseError::CantUseBreakInExpression(_)));
}
/*
  test("Detect break in expr") {
    // See BRCOBS
    compileExpressionForError(
      """
        |a(b, break)
        |""".stripMargin) match {
      case CantUseBreakInExpression(_) =>
    }
  }
*/
#[test]
fn detect_return_in_expr() {
  // See BRCOBS
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let err = compile_expression_for_error(&parse_arena, &keywords, "a(b, return)");
  assert!(matches!(err, ParseError::CantUseReturnInExpression(_)));
}
/*
  test("Detect return in expr") {
    // See BRCOBS
    compileExpressionForError(
      """
        |a(b, return)
        |""".stripMargin) match {
      case CantUseReturnInExpression(_) =>
    }
  }
*/
#[test]
fn parens() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "2 * (5 - 7)");
  match &expr {
    IExpressionPE::BinaryCall(BinaryCallPE {
      function_name: NameP(_, StrI("*")),
      left_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 2, .. }),
      right_expr: IExpressionPE::SubExpression(SubExpressionPE {
        inner: IExpressionPE::BinaryCall(BinaryCallPE {
          function_name: NameP(_, StrI("-")),
          left_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
          right_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 7, .. }),
          ..
        }),
        ..
      }),
      ..
    }) => {}
    _ => panic!("expected 2 * (5 - 7) structure"),
  }
}
/*
  test("parens") {
    compileExpressionExpect("2 * (5 - 7)") shouldHave
    { case BinaryCallPE(_,NameP(_,StrI("*")),ConstantIntPE(_,2,_),SubExpressionPE(_, BinaryCallPE(_,NameP(_,StrI("-")),ConstantIntPE(_,5,_),ConstantIntPE(_,7,_)))) => }
  }
*/
#[test]
fn precedence_1() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "(5 - 7) * 2");
  match &expr {
    IExpressionPE::BinaryCall(BinaryCallPE {
      function_name: NameP(_, StrI("*")),
      left_expr: IExpressionPE::SubExpression(SubExpressionPE {
        inner: IExpressionPE::BinaryCall(BinaryCallPE {
          function_name: NameP(_, StrI("-")),
          left_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
          right_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 7, .. }),
          ..
        }),
        ..
      }),
      right_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 2, .. }),
      ..
    }) => {}
    _ => panic!("expected (5 - 7) * 2 structure"),
  }
}
/*
  test("Precedence 1") {
    compileExpressionExpect("(5 - 7) * 2") shouldHave
      { case BinaryCallPE(_,NameP(_,StrI("*")),SubExpressionPE(_, BinaryCallPE(_,NameP(_,StrI("-")),ConstantIntPE(_,5,_),ConstantIntPE(_,7,_))), ConstantIntPE(_,2,_)) => }
  }
*/
#[test]
fn precedence_2() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "5 - 7 * 2");
  match &expr {
    IExpressionPE::BinaryCall(BinaryCallPE {
      function_name: NameP(_, StrI("-")),
      left_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
      right_expr: IExpressionPE::BinaryCall(BinaryCallPE {
        function_name: NameP(_, StrI("*")),
        left_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 7, .. }),
        right_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 2, .. }),
        ..
      }),
      ..
    }) => {}
    _ => panic!("expected 5 - 7 * 2 structure"),
  }
}
/*
  test("Precedence 2") {
    compileExpressionExpect("5 - 7 * 2") shouldHave
      { case BinaryCallPE(_,NameP(_,StrI("-")),ConstantIntPE(_,5,_),BinaryCallPE(_,NameP(_,StrI("*")),ConstantIntPE(_,7,_),ConstantIntPE(_,2,_))) => }
  }
*/
#[test]
fn static_array_from_values() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "[#](3, 5, 6)");
  match &expr {
    IExpressionPE::ConstructArray(ConstructArrayPE {
      type_pt: None,
      mutability_pt: Some(ITemplexPT::Mutability(MutabilityPT(_, MutabilityP::Mutable))),
      variability_pt: None,
      size: IArraySizeP::StaticSized(StaticSizedArraySizeP { size_pt: None }),
      initializing_individual_elements: true,
      args: [_, _, _, ..],
      ..
    }) => {}
    _ => panic!("expected [#](3, 5, 6) structure"),
  }
}
/*
  test("static array from values") {
    compileExpressionExpect("[#](3, 5, 6)") shouldHave
      {
      case ConstructArrayPE(_,None,Some(MutabilityPT(_,MutableP)),None,StaticSizedP(None),true,Vector(_, _, _)) =>
      }
  }
*/
#[test]
fn static_array_from_values_with_newlines() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "[#](\n3\n)");
  match &expr {
    IExpressionPE::ConstructArray(ConstructArrayPE { .. }) => {}
    _ => panic!("expected [#](\\n3\\n) structure"),
  }
}
/*
  test("static array from values with newlines") {
    compileExpressionExpect("[#](\n3\n)") shouldHave
      {
        case ConstructArrayPE(_,_,_,_,_,_,_) =>
      }
  }
*/
#[test]
fn static_array_from_callable_with_rune() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "[#N]({_ * 2})");
  match &expr {
    IExpressionPE::ConstructArray(ConstructArrayPE {
      type_pt: None,
      mutability_pt: Some(ITemplexPT::Mutability(MutabilityPT(_, MutabilityP::Mutable))),
      variability_pt: None,
      size: IArraySizeP::StaticSized(StaticSizedArraySizeP {
        size_pt: Some(ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("N"))))),
      }),
      initializing_individual_elements: false,
      args: [IExpressionPE::Lambda(_), ..],
      ..
    }) => {}
    _ => panic!("expected [#N]({{_ * 2}}) structure"),
  }
}
/*
  test("static array from callable with rune") {
    compileExpressionExpect("[#N]({_ * 2})") shouldHave
      {
      case ConstructArrayPE(_,
        None,
        Some(MutabilityPT(_,MutableP)),
        None,
        StaticSizedP(Some(NameOrRunePT(NameP(_,StrI("N"))))),
        false,
        Vector(LambdaPE(None,_))) =>
    }
  }
*/
#[test]
fn less_than_or_equal() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "a <= b");
  match &expr {
    IExpressionPE::BinaryCall(BinaryCallPE {
      function_name: NameP(_, StrI("<=")),
      left_expr: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("a"))),
        template_args: None,
      }),
      right_expr: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("b"))),
        template_args: None,
      }),
      ..
    }) => {}
    _ => panic!("expected a <= b structure"),
  }
}
/*
  test("Less than or equal") {
    compileExpressionExpect("a <= b") shouldHave
      {
        case BinaryCallPE(_,NameP(_,StrI("<=")),LookupPE(LookupNameP(NameP(_,StrI("a"))),None),LookupPE(LookupNameP(NameP(_,StrI("b"))),None)) =>
      }
  }
*/
#[test]
fn static_array_from_callable() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "[#3](triple)");
  match &expr {
    IExpressionPE::ConstructArray(ConstructArrayPE {
      type_pt: None,
      mutability_pt: Some(ITemplexPT::Mutability(MutabilityPT(_, MutabilityP::Mutable))),
      variability_pt: None,
      size: IArraySizeP::StaticSized(StaticSizedArraySizeP {
        size_pt: Some(ITemplexPT::Int(IntPT { value: 3, .. })),
      }),
      initializing_individual_elements: false,
      args: [_, ..],
      ..
    }) => {}
    _ => panic!("expected [#3](triple) structure"),
  }
}
/*
  test("static array from callable") {
    compileExpressionExpect("[#3](triple)") shouldHave
      {
      case ConstructArrayPE(_,
        None,
        Some(MutabilityPT(_,MutableP)),
        None,
        StaticSizedP(Some(IntPT(_,3))),
        false,
        Vector(_)) =>
    }
  }
*/
#[test]
fn immutable_static_array_from_callable() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "#[#3](triple)");
  match &expr {
    IExpressionPE::ConstructArray(ConstructArrayPE {
      type_pt: None,
      mutability_pt: Some(ITemplexPT::Mutability(MutabilityPT(_, MutabilityP::Immutable))),
      variability_pt: None,
      size: IArraySizeP::StaticSized(StaticSizedArraySizeP {
        size_pt: Some(ITemplexPT::Int(IntPT { value: 3, .. })),
      }),
      initializing_individual_elements: false,
      args: [_, ..],
      ..
    }) => {}
    _ => panic!("expected #[#3](triple) structure"),
  }
}
/*
  test("immutable static array from callable") {
    compileExpressionExpect("#[#3](triple)") shouldHave
      {
      case ConstructArrayPE(_,
        None,
        Some(MutabilityPT(_,ImmutableP)),
        None,
        StaticSizedP(Some(IntPT(_,3))),
        false,
        Vector(_)) =>
    }
  }
*/
#[test]
fn immutable_static_array_from_callable_no_size() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "#[#](3, 4, 5)");
  match &expr {
    IExpressionPE::ConstructArray(ConstructArrayPE {
      type_pt: None,
      mutability_pt: Some(ITemplexPT::Mutability(MutabilityPT(_, MutabilityP::Immutable))),
      variability_pt: None,
      size: IArraySizeP::StaticSized(StaticSizedArraySizeP { size_pt: None }),
      initializing_individual_elements: true,
      args: [_, _, _, ..],
      ..
    }) => {}
    _ => panic!("expected #[#](3, 4, 5) structure"),
  }
}
/*
  test("immutable static array from callable, no size") {
    compileExpressionExpect("#[#](3, 4, 5)") shouldHave
      {
      case ConstructArrayPE(_,
        None,
        Some(MutabilityPT(_,ImmutableP)),
        None,
        StaticSizedP(None),
        true,
        Vector(_, _, _)) =>
    }
  }
*/
#[test]
fn runtime_array_from_callable_with_rune() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "[](6, {_ * 2})");
  match &expr {
    IExpressionPE::ConstructArray(ConstructArrayPE {
      type_pt: None,
      mutability_pt: Some(ITemplexPT::Mutability(MutabilityPT(_, MutabilityP::Mutable))),
      variability_pt: None,
      size: IArraySizeP::RuntimeSized,
      initializing_individual_elements: false,
      args: [_, _, ..],
      ..
    }) => {}
    _ => panic!("expected [](6, {{_ * 2}}) structure"),
  }
}
/*
  test("runtime array from callable with rune") {
    compileExpressionExpect("[](6, {_ * 2})") shouldHave
      {
      case ConstructArrayPE(_,
        None,
        Some(MutabilityPT(_,MutableP)),
        None,
        RuntimeSizedP,
        false,
        Vector(_, _)) =>
    }
  }
*/
#[test]
fn runtime_array_from_callable() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "[](6, triple)");
  match &expr {
    IExpressionPE::ConstructArray(ConstructArrayPE {
      type_pt: None,
      mutability_pt: Some(ITemplexPT::Mutability(MutabilityPT(_, MutabilityP::Mutable))),
      variability_pt: None,
      size: IArraySizeP::RuntimeSized,
      initializing_individual_elements: false,
      args: [_, _, ..],
      ..
    }) => {}
    _ => panic!("expected [](6, triple) structure"),
  }
}
/*
  test("runtime array from callable") {
    compileExpressionExpect("[](6, triple)") shouldHave
      {
        case ConstructArrayPE(_,
        None,
        Some(MutabilityPT(_,MutableP)),
        None,
        RuntimeSizedP,
        false,
        Vector(_, _)) =>
      }
  }
*/
#[test]
fn double_rsa_with_type() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "[][]bool(42)");
  match &expr {
    IExpressionPE::ConstructArray(ConstructArrayPE {
      type_pt: Some(ITemplexPT::RuntimeSizedArray(RuntimeSizedArrayPT {
        mutability: ITemplexPT::Mutability(MutabilityPT(_, MutabilityP::Mutable)),
        element: ITemplexPT::NameOrRune(NameOrRunePT(NameP(_, StrI("bool")))),
        ..
      })),
      mutability_pt: Some(ITemplexPT::Mutability(MutabilityPT(_, MutabilityP::Mutable))),
      variability_pt: None,
      size: IArraySizeP::RuntimeSized,
      initializing_individual_elements: false,
      args: [IExpressionPE::ConstantInt(ConstantIntPE { value: 42, .. }), ..],
      ..
    }) => {}
    _ => panic!("expected [][]bool(42) structure"),
  }
}
/*
  test("Double RSA with type") {
    compileExpressionExpect("[][]bool(42)") shouldHave
      {
        case ConstructArrayPE(_,
        Some(RuntimeSizedArrayPT(_,MutabilityPT(_,MutableP),NameOrRunePT(NameP(_,StrI("bool"))))),
        Some(MutabilityPT(_,MutableP)),
        None,
        RuntimeSizedP,
        false,
        Vector(ConstantIntPE(_,42,None)))
        =>
      }
  }
*/
#[test]
fn immutable_runtime_array_from_callable() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "#[](6, triple)");
  match &expr {
    IExpressionPE::ConstructArray(ConstructArrayPE {
      type_pt: None,
      mutability_pt: Some(ITemplexPT::Mutability(MutabilityPT(_, MutabilityP::Immutable))),
      variability_pt: None,
      size: IArraySizeP::RuntimeSized,
      initializing_individual_elements: false,
      args: [_, _, ..],
      ..
    }) => {}
    _ => panic!("expected #[](6, triple) structure"),
  }
}
/*
  test("immutable runtime array from callable") {
    compileExpressionExpect("#[](6, triple)") shouldHave
      {
      case ConstructArrayPE(_,
        None,
        Some(MutabilityPT(_,ImmutableP)),
        None,
        RuntimeSizedP,
        false,
        Vector(_, _)) =>
    }
  }
*/
#[test]
fn one_element_tuple() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "(3,)");
  match &expr {
    IExpressionPE::Tuple(TuplePE {
      elements: [IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }), ..],
      ..
    }) => {}
    _ => panic!("expected (3,) structure"),
  }
}
/*

  test("One element tuple") {
    compileExpressionExpect("(3,)") shouldHave
      { case TuplePE(_,Vector(ConstantIntPE(_,3,_))) => }
  }
*/
#[test]
fn zero_element_tuple() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "()");
  match &expr {
    IExpressionPE::Tuple(TuplePE { elements: [], .. }) => {}
    _ => panic!("expected () structure"),
  }
}
/*
  test("Zero element tuple") {
    compileExpressionExpect("()") shouldHave
      { case TuplePE(_,Vector()) => }
  }
*/
#[test]
fn two_element_tuple() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "(3,4)");
  match &expr {
    IExpressionPE::Tuple(TuplePE {
      elements: [
        IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
        IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }),
        ..
      ],
      ..
    }) => {}
    _ => panic!("expected (3,4) structure"),
  }
}
/*
  test("Two element tuple") {
    compileExpressionExpect("(3,4)") shouldHave
      { case TuplePE(_,Vector(ConstantIntPE(_,3,_), ConstantIntPE(_,4,_))) => }
  }
*/
#[test]
fn three_element_tuple() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "(3,4,5)");
  match &expr {
    IExpressionPE::Tuple(TuplePE {
      elements: [
        IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
        IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }),
        IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
        ..
      ],
      ..
    }) => {}
    _ => panic!("expected (3,4,5) structure"),
  }
}
/*
  test("Three element tuple") {
    compileExpressionExpect("(3,4,5)") shouldHave
      { case TuplePE(_,Vector(ConstantIntPE(_,3,_), ConstantIntPE(_,4,_), ConstantIntPE(_,5,_))) => }
  }
*/
#[test]
fn three_element_tuple_trailing_comma() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "(3,4,5,)");
  match &expr {
    IExpressionPE::Tuple(TuplePE {
      elements: [
        IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }),
        IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. }),
        IExpressionPE::ConstantInt(ConstantIntPE { value: 5, .. }),
        ..
      ],
      ..
    }) => {}
    _ => panic!("expected (3,4,5,) structure"),
  }
}
/*
  test("Three element tuple trailing comma") {
    compileExpressionExpect("(3,4,5,)") shouldHave
      { case TuplePE(_,Vector(ConstantIntPE(_,3,_), ConstantIntPE(_,4,_), ConstantIntPE(_,5,_))) => }
  }
*/
#[test]
fn transmigrate() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "a'x");
  match &expr {
    IExpressionPE::Transmigrate(TransmigratePE {
      target_region: NameP(_, StrI("a")),
      inner: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("x"))),
        template_args: None,
      }),
      ..
    }) => {}
    _ => panic!("expected a'x structure"),
  }
}
/*
  test("Transmigrate") {
    compileExpressionExpect("a'x") shouldHave {
      case TransmigratePE(_,NameP(_,StrI("a")),LookupPE(LookupNameP(NameP(_,StrI("x"))),None)) =>
    }
  }
*/
#[test]
fn call_callable_expr() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr =
    compile_expression_expect(&parse_arena, &keywords, "(something.callable)(3)");
  match &expr {
    IExpressionPE::FunctionCall(FunctionCallPE {
      callable_expr: IExpressionPE::SubExpression(SubExpressionPE {
        inner: IExpressionPE::Dot(DotPE {
          left: IExpressionPE::Lookup(LookupPE {
            name: IImpreciseNameP::LookupName(NameP(_, StrI("something"))),
            template_args: None,
          }),
          member: NameP(_, StrI("callable")),
          ..
        }),
        ..
      }),
      arg_exprs: [_arg, ..],
      ..
    }) => {}
    _ => panic!("expected (something.callable)(3) structure"),
  }
}
/*
  test("Call callable expr") {
    compileExpressionExpect("(something.callable)(3)") shouldHave
      {
      case FunctionCallPE(
          _,_,
          SubExpressionPE(_, DotPE(_,LookupPE(LookupNameP(NameP(_, StrI("something"))),None),_,NameP(_,StrI("callable")))),
          Vector(_)) =>
      }
  }
*/
#[test]
fn array_indexing() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "board[i]");
  match &expr {
    IExpressionPE::BraceCall(BraceCallPE {
      subject_expr: IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("board"))),
        template_args: None,
      }),
      arg_exprs: [IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("i"))),
        template_args: None,
      }), ..],
      callable_readwrite: false,
      ..
    }) => {}
    _ => panic!("expected board[i] structure"),
  }

  let expr2 = compile_expression_expect(&parse_arena, &keywords, "this.board[i]");
  match &expr2 {
    IExpressionPE::BraceCall(BraceCallPE {
      subject_expr: IExpressionPE::Dot(DotPE {
        left: IExpressionPE::Lookup(LookupPE {
          name: IImpreciseNameP::LookupName(NameP(_, StrI("this"))),
          template_args: None,
        }),
        member: NameP(_, StrI("board")),
        ..
      }),
      arg_exprs: [IExpressionPE::Lookup(LookupPE {
        name: IImpreciseNameP::LookupName(NameP(_, StrI("i"))),
        template_args: None,
      }), ..],
      callable_readwrite: false,
      ..
    }) => {}
    _ => panic!("expected this.board[i] structure"),
  }
}
/*
  test("Array indexing") {
    compileExpressionExpect("board[i]") shouldHave
      {
      case BraceCallPE(_,_,LookupPE(LookupNameP(NameP(_,StrI("board"))),None),Vector(LookupPE(LookupNameP(NameP(_,StrI("i"))),None)),false) =>
      }
    compileExpressionExpect("this.board[i]") shouldHave
      {
      case BraceCallPE(_,_,DotPE(_,LookupPE(LookupNameP(NameP(_, StrI("this"))),None),_,NameP(_,StrI("board"))),Vector(LookupPE(LookupNameP(NameP(_,StrI("i"))),None)),false) =>
      }
  }
*/
#[test]
fn mod_and_equal_precedence() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "8 mod 2 == 0");
  match &expr {
    IExpressionPE::BinaryCall(BinaryCallPE {
      function_name: NameP(_, StrI("==")),
      left_expr: IExpressionPE::BinaryCall(BinaryCallPE {
        function_name: NameP(_, StrI("mod")),
        left_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 8, .. }),
        right_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 2, .. }),
        ..
      }),
      right_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 0, .. }),
      ..
    }) => {}
    _ => panic!("expected 8 mod 2 == 0 structure"),
  }
}
/*
  test("mod and == precedence") {
    compileExpressionExpect("""8 mod 2 == 0""") shouldHave
      {
      case BinaryCallPE(_,
      NameP(_, StrI("==")),
        BinaryCallPE(_,
          NameP(_, StrI("mod")),
          ConstantIntPE(_, 8, _),
          ConstantIntPE(_, 2, _)),
        ConstantIntPE(_, 0, _)) =>
    }
  }
*/
#[test]
fn or_and_equal_precedence() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "2 == 0 or false");
  match &expr {
    IExpressionPE::Or(OrPE {
      left: IExpressionPE::BinaryCall(BinaryCallPE {
        function_name: NameP(_, StrI("==")),
        left_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 2, .. }),
        right_expr: IExpressionPE::ConstantInt(ConstantIntPE { value: 0, .. }),
        ..
      }),
      right: BlockPE {
        inner: IExpressionPE::ConstantBool(ConstantBoolPE { value: false, .. }),
        ..
      },
      ..
    }) => {}
    _ => panic!("expected 2 == 0 or false structure"),
  }
}
/*
  test("or and == precedence") {
    compileExpressionExpect("""2 == 0 or false""") shouldHave
      {
      case OrPE(_,
        BinaryCallPE(_,
          NameP(_, StrI("==")),
          ConstantIntPE(_, 2, _),
          ConstantIntPE(_, 0, _)),
        BlockPE(_,_, _, ConstantBoolPE(_,false))) =>
    }
  }
*/
#[test]
fn test_templated_lambda_param() {
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let expr = compile_expression_expect(&parse_arena, &keywords, "(a => a + a)(3)");
  match &expr {
    IExpressionPE::FunctionCall(FunctionCallPE {
      callable_expr: IExpressionPE::SubExpression(SubExpressionPE {
        inner: IExpressionPE::Lambda(LambdaPE {
          function: FunctionP {
            header: FunctionHeaderP {
              params: Some(ParamsP {
                params: [ParameterP {
                  pattern: Some(PatternPP {
                    destination: Some(DestinationLocalP {
                      decl: INameDeclarationP::LocalNameDeclaration(NameP(_, StrI("a"))),
                      ..
                    }),
                    templex: None,
                    destructure: None,
                    ..
                  }),
                  ..
                }, ..],
                ..
              }),
              ..
            },
            body: Some(BlockPE {
              inner: IExpressionPE::BinaryCall(BinaryCallPE {
                function_name: NameP(_, StrI("+")),
                left_expr: IExpressionPE::Lookup(LookupPE {
                  name: IImpreciseNameP::LookupName(NameP(_, StrI("a"))),
                  template_args: None,
                }),
                right_expr: IExpressionPE::Lookup(LookupPE {
                  name: IImpreciseNameP::LookupName(NameP(_, StrI("a"))),
                  template_args: None,
                }),
                ..
              }),
              ..
            }),
            ..
          },
          ..
        }),
        ..
      }),
      arg_exprs: [IExpressionPE::ConstantInt(ConstantIntPE { value: 3, .. }), ..],
      ..
    }) => {}
    _ => panic!("expected (a => a + a)(3) structure"),
  }
}
/*
  test("Test templated lambda param") {
    val program = compileExpressionExpect("(a => a + a)(3)")
    program shouldHave {
      case FunctionCallPE(_, _, SubExpressionPE(_, LambdaPE(_, _)), Vector(ConstantIntPE(_, 3, _))) =>
    }
    program shouldHave {
      case PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("a"))), None)),None,None) =>
    }
    program shouldHave {
      case BinaryCallPE(_, NameP(_, StrI("+")), LookupPE(LookupNameP(NameP(_, StrI("a"))), None), LookupPE(LookupNameP(NameP(_, StrI("a"))), None)) =>
    }
  }
*/
/*
//  // See https://github.com/ValeLang/Vale/issues/108
//  test("Calling with space") {
//    compile(CombinatorParsers.expression(true),
//      """len (cached_dims)""") shouldHave {
//      case FunctionCallPE(_,_,_,_,LookupPE(StringP(_,"len"),None),Vector(LookupPE(StringP(_,"cached_dims"),None)),_) =>
//    }
//  }
}
*/
