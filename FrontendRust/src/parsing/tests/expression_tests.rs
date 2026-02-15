// cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing::tests::expression_tests

use bumpalo::Bump;
use crate::cast;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::lexing::errors::ParseError;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::assert_templex_name;
use crate::parsing::tests::utils::*;
use crate::parsing::tests::utils::{assert_lookup_name, assert_name};

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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "4");
  assert!(matches!(
    expr,
    IExpressionPE::ConstantInt(ConstantIntPE { value: 4, .. })
  ));
}
/*
  test("Simple int") {
    val expr = compileExpressionExpect("4")
     expr shouldHave { case ConstantIntPE(_, 4, None) => }
  }
*/
#[test]
fn simple_bool() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "true");
  assert!(matches!(
    expr,
    IExpressionPE::ConstantBool(ConstantBoolPE { value: true, .. })
  ));
}
/*
  test("Simple bool") {
    compileExpressionExpect("true") shouldHave
      { case ConstantBoolPE(_, true) => }
  }
*/
#[test]
fn i64() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "4i64");
  assert!(matches!(
    expr,
    IExpressionPE::ConstantInt(ConstantIntPE {
      value: 4,
      bits: Some(64),
      ..
    })
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "4 + 5");
  let binary = cast!(expr, IExpressionPE::BinaryCall);
  assert_eq!(binary.function_name.str.str, "+");
  assert_eq!(
    cast!(binary.left_expr.as_ref(), IExpressionPE::ConstantInt).value,
    4
  );
  assert_eq!(
    cast!(binary.right_expr.as_ref(), IExpressionPE::ConstantInt).value,
    5
  );
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "4.2");
  assert!(matches!(
    expr,
    IExpressionPE::ConstantFloat(ConstantFloatPE { value: 4.2, .. })
  ));
}
/*
  test("Floats") {
    compileExpressionExpect("4.2") shouldHave
      { case ConstantFloatPE(_, 4.2) => }
  }
*/
#[test]
fn number_range() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "0..5");
  let range = cast!(expr, IExpressionPE::Range);
  assert_eq!(
    cast!(range.from_expr.as_ref(), IExpressionPE::ConstantInt).value,
    0
  );
  assert_eq!(
    cast!(range.to_expr.as_ref(), IExpressionPE::ConstantInt).value,
    5
  );
}
/*
  test("Number range") {
    compileExpressionExpect("0..5") shouldHave
      { case RangePE(_,ConstantIntPE(_,0,_),ConstantIntPE(_,5,_)) => }
  }
*/
#[test]
fn add_as_call() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "+(4, 5)");
  let function_call = cast!(expr, IExpressionPE::FunctionCall);
  assert_lookup_name(function_call.callable_expr.as_ref(), "+");
  let (first_arg, second_arg) = expect_2(&function_call.arg_exprs);
  assert_eq!(cast!(first_arg, IExpressionPE::ConstantInt).value, 4);
  assert_eq!(cast!(second_arg, IExpressionPE::ConstantInt).value, 5);
}
/*
  test("add as call") {
    compileExpressionExpect("+(4, 5)") shouldHave
      { case FunctionCallPE(_,_,LookupPE(LookupNameP(NameP(_, StrI("+"))), None), Vector(ConstantIntPE(_, 4, _), ConstantIntPE(_, 5, _))) => }
  }
*/
#[test]
fn passing_eq_overload_set() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "moo(4, ==)");
  let function_call = cast!(expr, IExpressionPE::FunctionCall);
  assert_lookup_name(function_call.callable_expr.as_ref(), "moo");
  let (first_arg, second_arg) = expect_2(&function_call.arg_exprs);
  let constant_int = cast!(first_arg, IExpressionPE::ConstantInt);
  assert_eq!(constant_int.value, 4);
  assert_lookup_name(second_arg, "==");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "str(i) + 5");
  let binary = cast!(expr, IExpressionPE::BinaryCall);
  assert_eq!(binary.function_name.str.str, "+");
  let str_call = cast!(binary.left_expr.as_ref(), IExpressionPE::FunctionCall);
  assert_lookup_name(str_call.callable_expr.as_ref(), "str");
  let first_arg = expect_1(&str_call.arg_exprs);
  assert_lookup_name(first_arg, "i");
  let five_int = cast!(binary.right_expr.as_ref(), IExpressionPE::ConstantInt);
  assert_eq!(five_int.value, 5);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "a..b");
  let range = cast!(expr, IExpressionPE::Range);
  assert_lookup_name(range.from_expr.as_ref(), "a");
  assert_lookup_name(range.to_expr.as_ref(), "b");
}
/*
  test("range") {
    compileExpressionExpect("a..b") shouldHave
      { case RangePE(_,LookupPE(LookupNameP(NameP(_,StrI("a"))),None),LookupPE(LookupNameP(NameP(_,StrI("b"))),None)) =>}
  }
*/
#[test]
fn regular_call() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "x(y)");
  let function_call = cast!(expr, IExpressionPE::FunctionCall);
  assert_lookup_name(function_call.callable_expr.as_ref(), "x");
  let first_arg = expect_1(&function_call.arg_exprs);
  assert_lookup_name(first_arg, "y");
}
/*
  test("regular call") {
    compileExpressionExpect("x(y)") shouldHave
      { case FunctionCallPE(_,_,LookupPE(LookupNameP(NameP(_, StrI("x"))), None), Vector(LookupPE(LookupNameP(NameP(_, StrI("y"))), None))) => }
  }
*/
#[test]
fn not() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "not y");
  let not = cast!(expr, IExpressionPE::Not);
  assert_lookup_name(not.inner.as_ref(), "y");
}
/*
  test("not") {
    compileExpressionExpect("not y") shouldHave
      { case NotPE(_,LookupPE(LookupNameP(NameP(_,StrI("y"))),None)) => }
  }
*/
#[test]
fn borrowing_result_of_function_call() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "&Muta()");
  let augment = cast!(expr, IExpressionPE::Augment);
  assert_eq!(augment.target_ownership, OwnershipP::Borrow);
  let function_call = cast!(augment.inner.as_ref(), IExpressionPE::FunctionCall);
  assert_lookup_name(function_call.callable_expr.as_ref(), "Muta");
  assert_eq!(function_call.arg_exprs.len(), 0);
}
/*
  test("Borrowing result of function call") {
    compileExpressionExpect("&Muta()") shouldHave
      { case AugmentPE(_,BorrowP,FunctionCallPE(_,_,LookupPE(LookupNameP(NameP(_,StrI("Muta"))),None),Vector())) => }
  }
*/
#[test]
fn specifying_heap() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "^Muta()");
  let augment = cast!(expr, IExpressionPE::Augment);
  assert_eq!(augment.target_ownership, OwnershipP::Own);
  cast!(augment.inner.as_ref(), IExpressionPE::FunctionCall);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "inl Muta()");
  let augment = cast!(expr, IExpressionPE::Augment);
  assert_eq!(augment.target_ownership, OwnershipP::Own);
  let function_call = cast!(augment.inner.as_ref(), IExpressionPE::FunctionCall);
  assert_lookup_name(function_call.callable_expr.as_ref(), "Muta");
  assert_eq!(function_call.arg_exprs.len(), 0);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "x . shout ()");
  let method_call = cast!(expr, IExpressionPE::MethodCall);
  assert_lookup_name(method_call.subject_expr.as_ref(), "x");
  assert_name(&method_call.method_lookup.name, "shout");
  assert_eq!(method_call.arg_exprs.len(), 0);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "x *. shout ()");
  let method_call = cast!(expr, IExpressionPE::MethodCall);
  assert_lookup_name(method_call.subject_expr.as_ref(), "x");
  assert_name(&method_call.method_lookup.name, "shout");
  assert_eq!(method_call.arg_exprs.len(), 0);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "x.moo.shout()");
  let shout_call = cast!(expr, IExpressionPE::MethodCall);
  assert_name(&shout_call.method_lookup.name, "shout");
  assert_eq!(shout_call.arg_exprs.len(), 0);
  let x_dot_moo = cast!(shout_call.subject_expr.as_ref(), IExpressionPE::Dot);
  assert_lookup_name(&x_dot_moo.left, "x");
  assert_eq!(x_dot_moo.member.str.str, "moo");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "(x ).shout()");
  let shout_call = cast!(expr, IExpressionPE::MethodCall);
  assert_name(&shout_call.method_lookup.name, "shout");
  assert_eq!(shout_call.arg_exprs.len(), 0);
  let x_subexpr = cast!(
    shout_call.subject_expr.as_ref(),
    IExpressionPE::SubExpression
  );
  assert_lookup_name(x_subexpr.inner.as_ref(), "x");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "toArray<imm>( &result)");
  let function_call = cast!(expr, IExpressionPE::FunctionCall);

  let toarray_lookup = cast!(function_call.callable_expr.as_ref(), IExpressionPE::Lookup);
  assert_name(&toarray_lookup.name, "toArray");
  let template_args = toarray_lookup.template_args.as_ref().unwrap();
  let first_template_arg = expect_1(&template_args.args);
  let mutability = cast!(first_template_arg, ITemplexPT::Mutability);
  assert_eq!(mutability.mutability, MutabilityP::Immutable);

  let first_arg = expect_1(&function_call.arg_exprs);
  let augment = cast!(first_arg, IExpressionPE::Augment);
  assert_eq!(augment.target_ownership, OwnershipP::Borrow);
  assert_lookup_name(augment.inner.as_ref(), "result");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "result.toArray <imm> ()");
  let method_call = cast!(expr, IExpressionPE::MethodCall);

  assert_lookup_name(method_call.subject_expr.as_ref(), "result");
  let toarray_lookup = cast!(&method_call.method_lookup.name, IImpreciseNameP::LookupName);
  assert_eq!(toarray_lookup.str.str, "toArray");
  let template_args = method_call.method_lookup.template_args.as_ref().unwrap();
  let first_template_arg = expect_1(&template_args.args);
  let mutability = cast!(first_template_arg, ITemplexPT::Mutability);
  assert_eq!(mutability.mutability, MutabilityP::Immutable);
  assert_eq!(method_call.arg_exprs.len(), 0);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "not y florgle not x");
  let binary = cast!(expr, IExpressionPE::BinaryCall);
  assert_eq!(binary.function_name.str.str, "florgle");
  let not_left = cast!(binary.left_expr.as_ref(), IExpressionPE::Not);
  assert_lookup_name(not_left.inner.as_ref(), "y");
  let not_right = cast!(binary.right_expr.as_ref(), IExpressionPE::Not);
  assert_lookup_name(not_right.inner.as_ref(), "x");
}
/*
  test("Custom binaries") {
    compileExpressionExpect("not y florgle not x") shouldHave
      { case BinaryCallPE(_,NameP(_,StrI("florgle")),NotPE(_,LookupPE(LookupNameP(NameP(_,StrI("y"))),None)),NotPE(_,LookupPE(LookupNameP(NameP(_,StrI("x"))),None))) => }
  }
*/
#[test]
fn custom_with_noncustom_binaries() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "a + b florgle x * y");
  let binary = cast!(expr, IExpressionPE::BinaryCall);
  assert_eq!(binary.function_name.str.str, "florgle");
  let add_binary = cast!(binary.left_expr.as_ref(), IExpressionPE::BinaryCall);
  assert_eq!(add_binary.function_name.str.str, "+");
  assert_lookup_name(add_binary.left_expr.as_ref(), "a");
  assert_lookup_name(add_binary.right_expr.as_ref(), "b");
  let times_binary = cast!(binary.right_expr.as_ref(), IExpressionPE::BinaryCall);
  assert_eq!(times_binary.function_name.str.str, "*");
  assert_lookup_name(times_binary.left_expr.as_ref(), "x");
  assert_lookup_name(times_binary.right_expr.as_ref(), "y");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  {
    let expr = compile_expression_expect(&interner, &keywords, "MyNone< int >()");// UIIOVCP
    let function_call = cast!(expr, IExpressionPE::FunctionCall);
    let mynone_lookup = cast!(function_call.callable_expr.as_ref(), IExpressionPE::Lookup);
    assert_name(&mynone_lookup.name, "MyNone");
    let template_args = mynone_lookup.template_args.as_ref().unwrap();
    let first_template_arg = expect_1(&template_args.args);
    assert_templex_name(first_template_arg, "int");
    assert_eq!(function_call.arg_exprs.len(), 0);
  }

  {
    let expr = compile_expression_expect(&interner, &keywords, "MySome< MyNone <int> >()");// UIIOVCP
    let function_call = cast!(expr, IExpressionPE::FunctionCall);
    let mysome_lookup = cast!(function_call.callable_expr.as_ref(), IExpressionPE::Lookup);
    assert_name(&mysome_lookup.name, "MySome");
    let template_args = mysome_lookup.template_args.as_ref().unwrap();
    let first_template_arg = expect_1(&template_args.args);

    let mynone_lookup = cast!(first_template_arg, ITemplexPT::Call);
    assert_templex_name(&mynone_lookup.template, "MyNone");
    let first_call_arg = expect_1(&mynone_lookup.args);
    assert_templex_name(first_call_arg, "int");
    assert_eq!(function_call.arg_exprs.len(), 0);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "9 >= 3");
  let binary = cast!(expr, IExpressionPE::BinaryCall);
  assert_eq!(binary.function_name.str.str, ">=");
  assert_eq!(
    cast!(binary.left_expr.as_ref(), IExpressionPE::ConstantInt).value,
    9
  );
  assert_eq!(
    cast!(binary.right_expr.as_ref(), IExpressionPE::ConstantInt).value,
    3
  );
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "arr [4]");
  let brace_call = cast!(expr, IExpressionPE::BraceCall);
  assert_lookup_name(brace_call.subject_expr.as_ref(), "arr");
  let first_arg = expect_1(&brace_call.arg_exprs);
  assert_eq!(cast!(first_arg, IExpressionPE::ConstantInt).value, 4);
}
/*
  test("Indexing") {
    compileExpressionExpect("arr [4]") shouldHave
      { case BraceCallPE(_,_,LookupPE(LookupNameP(NameP(_,StrI("arr"))),None),Vector(ConstantIntPE(_,4,_)),_) => }
  }
*/
#[test]
fn single_arg_brace_lambda() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "x => { x }");
  let lambda = cast!(expr, IExpressionPE::Lambda);
  let function = &lambda.function;
  let params = function.header.params.as_ref().unwrap();
  let first_param = expect_1(&params.params);
  let pattern = first_param.pattern.as_ref().unwrap();
  let destination = pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "x");
  let block = &lambda.function.body.as_ref().unwrap();
  assert_lookup_name(&block.inner, "x");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "x => x");
  let lambda = cast!(expr, IExpressionPE::Lambda);
  let function = &lambda.function;
  let params = function.header.params.as_ref().unwrap();
  let first_param = expect_1(&params.params);
  let pattern = first_param.pattern.as_ref().unwrap();
  let destination = pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "x");
  let block = &lambda.function.body.as_ref().unwrap();
  assert_lookup_name(&block.inner, "x");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "(x int) => { x }");
  let lambda = cast!(expr, IExpressionPE::Lambda);
  let function = &lambda.function;
  let params = function.header.params.as_ref().unwrap();
  let first_param = expect_1(&params.params);
  let pattern = first_param.pattern.as_ref().unwrap();
  let templex = pattern.templex.as_ref().unwrap();
  assert_templex_name(templex, "int");
  let destination = pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "x");
  let block = &lambda.function.body.as_ref().unwrap();
  assert_lookup_name(&block.inner, "x");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "{_}");
  let lambda = cast!(expr, IExpressionPE::Lambda);
  let function = &lambda.function;
  assert!(function.header.ret.ret_type.is_none());
  assert!(function.header.params.is_none());
  let block = &lambda.function.body.as_ref().unwrap();
  assert!(matches!(*block.inner, IExpressionPE::MagicParamLookup(_)));
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "(x, y) => x");
  let lambda = cast!(expr, IExpressionPE::Lambda);
  let function = &lambda.function;
  let params = function.header.params.as_ref().unwrap();
  let (first_param, second_param) = expect_2(&params.params);
  let pattern = first_param.pattern.as_ref().unwrap();
  let destination = pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "x");
  let pattern = second_param.pattern.as_ref().unwrap();
  let destination = pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "y");
  let block = &lambda.function.body.as_ref().unwrap();
  assert_lookup_name(&block.inner, "x");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "([x, y]) => x");
  let lambda = cast!(expr, IExpressionPE::Lambda);
  let function = &lambda.function;
  let params = function.header.params.as_ref().unwrap();
  let first_param = expect_1(&params.params);
  let pattern = first_param.pattern.as_ref().unwrap();
  assert!(pattern.destination.is_none());
  assert!(pattern.templex.is_none());
  let destructure = pattern.destructure.as_ref().unwrap();
  let (x_pattern, y_pattern) = expect_2(&destructure.patterns);
  let x_destination = x_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(x_destination, "x");
  let y_destination = y_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(y_destination, "y");
  let block = &lambda.function.body.as_ref().unwrap();
  assert_lookup_name(&block.inner, "x");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, r#"myPath./("subdir")"#);
  let method_call = cast!(expr, IExpressionPE::MethodCall);
  assert_lookup_name(method_call.subject_expr.as_ref(), "myPath");
  let method_lookup = cast!(&method_call.method_lookup.name, IImpreciseNameP::LookupName);
  assert_eq!(method_lookup.str.str, "/");
  let first_arg = expect_1(&method_call.arg_exprs);
  let constant_str = cast!(first_arg, IExpressionPE::ConstantStr);
  assert_eq!(constant_str.value, "subdir");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "3 != 4");
  let binary = cast!(expr, IExpressionPE::BinaryCall);
  assert_eq!(binary.function_name.str.str, "!=");
  assert_eq!(
    cast!(binary.left_expr.as_ref(), IExpressionPE::ConstantInt).value,
    3
  );
  assert_eq!(
    cast!(binary.right_expr.as_ref(), IExpressionPE::ConstantInt).value,
    4
  );
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "set(true)");
  let function_call = cast!(expr, IExpressionPE::FunctionCall);
  assert_lookup_name(function_call.callable_expr.as_ref(), "set");
  let first_arg = expect_1(&function_call.arg_exprs);
  assert!(matches!(
    first_arg,
    IExpressionPE::ConstantBool(ConstantBoolPE { value: true, .. })
  ));
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "arr.2.1");
  let outer_dot = cast!(expr, IExpressionPE::Dot);
  assert_eq!(outer_dot.member.str.str, "1");
  let inner_dot = cast!(outer_dot.left.as_ref(), IExpressionPE::Dot);
  assert_eq!(inner_dot.member.str.str, "2");
  assert_lookup_name(&inner_dot.left, "arr");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "{ 0 }()");
  let function_call = cast!(expr, IExpressionPE::FunctionCall);
  cast!(function_call.callable_expr.as_ref(), IExpressionPE::Lambda);
  assert_eq!(function_call.arg_exprs.len(), 0);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "call(sum)");
  let function_call = cast!(expr, IExpressionPE::FunctionCall);
  assert_lookup_name(function_call.callable_expr.as_ref(), "call");
  let first_arg = expect_1(&function_call.arg_exprs);
  assert_lookup_name(first_arg, "sum");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "destroy(unlet enemy)");
  let function_call = cast!(expr, IExpressionPE::FunctionCall);
  assert_lookup_name(function_call.callable_expr.as_ref(), "destroy");
  let first_arg = expect_1(&function_call.arg_exprs);
  let unlet = cast!(first_arg, IExpressionPE::Unlet);
  let lookup_name = cast!(&unlet.name, IImpreciseNameP::LookupName);
  assert_eq!(lookup_name.str.str, "enemy");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let err = compile_expression_for_error(&interner, &keywords, "a(b, break)");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let err = compile_expression_for_error(&interner, &keywords, "a(b, return)");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "2 * (5 - 7)");
  let binary = cast!(expr, IExpressionPE::BinaryCall);
  assert_eq!(binary.function_name.str.str, "*");
  assert_eq!(
    cast!(binary.left_expr.as_ref(), IExpressionPE::ConstantInt).value,
    2
  );
  let subexpr = cast!(binary.right_expr.as_ref(), IExpressionPE::SubExpression);
  let inner_binary = cast!(subexpr.inner.as_ref(), IExpressionPE::BinaryCall);
  assert_eq!(inner_binary.function_name.str.str, "-");
  assert_eq!(
    cast!(inner_binary.left_expr.as_ref(), IExpressionPE::ConstantInt).value,
    5
  );
  assert_eq!(
    cast!(inner_binary.right_expr.as_ref(), IExpressionPE::ConstantInt).value,
    7
  );
}
/*
  test("parens") {
    compileExpressionExpect("2 * (5 - 7)") shouldHave
    { case BinaryCallPE(_,NameP(_,StrI("*")),ConstantIntPE(_,2,_),SubExpressionPE(_, BinaryCallPE(_,NameP(_,StrI("-")),ConstantIntPE(_,5,_),ConstantIntPE(_,7,_)))) => }
  }
*/
#[test]
fn precedence_1() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "(5 - 7) * 2");
  let binary = cast!(expr, IExpressionPE::BinaryCall);
  assert_eq!(binary.function_name.str.str, "*");
  let subexpr = cast!(binary.left_expr.as_ref(), IExpressionPE::SubExpression);
  let inner_binary = cast!(subexpr.inner.as_ref(), IExpressionPE::BinaryCall);
  assert_eq!(inner_binary.function_name.str.str, "-");
  assert_eq!(
    cast!(inner_binary.left_expr.as_ref(), IExpressionPE::ConstantInt).value,
    5
  );
  assert_eq!(
    cast!(inner_binary.right_expr.as_ref(), IExpressionPE::ConstantInt).value,
    7
  );
  assert_eq!(
    cast!(binary.right_expr.as_ref(), IExpressionPE::ConstantInt).value,
    2
  );
}
/*
  test("Precedence 1") {
    compileExpressionExpect("(5 - 7) * 2") shouldHave
      { case BinaryCallPE(_,NameP(_,StrI("*")),SubExpressionPE(_, BinaryCallPE(_,NameP(_,StrI("-")),ConstantIntPE(_,5,_),ConstantIntPE(_,7,_))), ConstantIntPE(_,2,_)) => }
  }
*/
#[test]
fn precedence_2() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "5 - 7 * 2");
  let binary = cast!(expr, IExpressionPE::BinaryCall);
  assert_eq!(binary.function_name.str.str, "-");
  assert_eq!(
    cast!(binary.left_expr.as_ref(), IExpressionPE::ConstantInt).value,
    5
  );
  let right_binary = cast!(binary.right_expr.as_ref(), IExpressionPE::BinaryCall);
  assert_eq!(right_binary.function_name.str.str, "*");
  assert_eq!(
    cast!(right_binary.left_expr.as_ref(), IExpressionPE::ConstantInt).value,
    7
  );
  assert_eq!(
    cast!(right_binary.right_expr.as_ref(), IExpressionPE::ConstantInt).value,
    2
  );
}
/*
  test("Precedence 2") {
    compileExpressionExpect("5 - 7 * 2") shouldHave
      { case BinaryCallPE(_,NameP(_,StrI("-")),ConstantIntPE(_,5,_),BinaryCallPE(_,NameP(_,StrI("*")),ConstantIntPE(_,7,_),ConstantIntPE(_,2,_))) => }
  }
*/
#[test]
fn static_array_from_values() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "[#](3, 5, 6)");
  let construct_array = cast!(expr, IExpressionPE::ConstructArray);
  assert!(construct_array.type_pt.is_none());
  let mutability = construct_array.mutability_pt.as_ref().unwrap();
  assert_eq!(
    cast!(mutability, ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert!(construct_array.variability_pt.is_none());
  assert!(matches!(
    &construct_array.size,
    IArraySizeP::StaticSized(StaticSizedArraySizeP { size_pt: None })
  ));
  assert_eq!(construct_array.initializing_individual_elements, true);
  assert_eq!(construct_array.args.len(), 3);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "[#](\n3\n)");
  cast!(expr, IExpressionPE::ConstructArray);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "[#N]({_ * 2})");
  let construct_array = cast!(expr, IExpressionPE::ConstructArray);
  assert!(construct_array.type_pt.is_none());
  let mutability = construct_array.mutability_pt.as_ref().unwrap();
  assert_eq!(
    cast!(mutability, ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert!(construct_array.variability_pt.is_none());
  let static_sized = cast!(&construct_array.size, IArraySizeP::StaticSized);
  let size_templex = static_sized.size_pt.as_ref().unwrap();
  assert_templex_name(size_templex, "N");
  assert_eq!(construct_array.initializing_individual_elements, false);
  let first_arg = expect_1(&construct_array.args);
  cast!(first_arg, IExpressionPE::Lambda);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "a <= b");
  let binary = cast!(expr, IExpressionPE::BinaryCall);
  assert_eq!(binary.function_name.str.str, "<=");
  assert_lookup_name(binary.left_expr.as_ref(), "a");
  assert_lookup_name(binary.right_expr.as_ref(), "b");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "[#3](triple)");
  let construct_array = cast!(expr, IExpressionPE::ConstructArray);
  assert!(construct_array.type_pt.is_none());
  let mutability = construct_array.mutability_pt.as_ref().unwrap();
  assert_eq!(
    cast!(mutability, ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert!(construct_array.variability_pt.is_none());
  let static_sized = cast!(&construct_array.size, IArraySizeP::StaticSized);
  let size_templex = static_sized.size_pt.as_ref().unwrap();
  assert_eq!(cast!(size_templex, ITemplexPT::Int).value, 3);
  assert_eq!(construct_array.initializing_individual_elements, false);
  expect_1(&construct_array.args);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "#[#3](triple)");
  let construct_array = cast!(expr, IExpressionPE::ConstructArray);
  assert!(construct_array.type_pt.is_none());
  let mutability = construct_array.mutability_pt.as_ref().unwrap();
  assert_eq!(
    cast!(mutability, ITemplexPT::Mutability).mutability,
    MutabilityP::Immutable
  );
  assert!(construct_array.variability_pt.is_none());
  let static_sized = cast!(&construct_array.size, IArraySizeP::StaticSized);
  let size_templex = static_sized.size_pt.as_ref().unwrap();
  assert_eq!(cast!(size_templex, ITemplexPT::Int).value, 3);
  assert_eq!(construct_array.initializing_individual_elements, false);
  expect_1(&construct_array.args);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "#[#](3, 4, 5)");
  let construct_array = cast!(expr, IExpressionPE::ConstructArray);
  assert!(construct_array.type_pt.is_none());
  let mutability = construct_array.mutability_pt.as_ref().unwrap();
  assert_eq!(
    cast!(mutability, ITemplexPT::Mutability).mutability,
    MutabilityP::Immutable
  );
  assert!(construct_array.variability_pt.is_none());
  assert!(matches!(
    &construct_array.size,
    IArraySizeP::StaticSized(StaticSizedArraySizeP { size_pt: None })
  ));
  assert_eq!(construct_array.initializing_individual_elements, true);
  assert_eq!(construct_array.args.len(), 3);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "[](6, {_ * 2})");
  let construct_array = cast!(expr, IExpressionPE::ConstructArray);
  assert!(construct_array.type_pt.is_none());
  let mutability = construct_array.mutability_pt.as_ref().unwrap();
  assert_eq!(
    cast!(mutability, ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert!(construct_array.variability_pt.is_none());
  assert!(matches!(&construct_array.size, IArraySizeP::RuntimeSized));
  assert_eq!(construct_array.initializing_individual_elements, false);
  assert_eq!(construct_array.args.len(), 2);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "[](6, triple)");
  let construct_array = cast!(expr, IExpressionPE::ConstructArray);
  assert!(construct_array.type_pt.is_none());
  let mutability = construct_array.mutability_pt.as_ref().unwrap();
  assert_eq!(
    cast!(mutability, ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert!(construct_array.variability_pt.is_none());
  assert!(matches!(&construct_array.size, IArraySizeP::RuntimeSized));
  assert_eq!(construct_array.initializing_individual_elements, false);
  assert_eq!(construct_array.args.len(), 2);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "[][]bool(42)");
  let construct_array = cast!(expr, IExpressionPE::ConstructArray);
  let array_type = construct_array.type_pt.as_ref().unwrap();
  let rsa = cast!(array_type, ITemplexPT::RuntimeSizedArray);
  assert_eq!(
    cast!(rsa.mutability.as_ref(), ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert_templex_name(&rsa.element, "bool");
  let mutability = construct_array.mutability_pt.as_ref().unwrap();
  assert_eq!(
    cast!(mutability, ITemplexPT::Mutability).mutability,
    MutabilityP::Mutable
  );
  assert!(construct_array.variability_pt.is_none());
  assert!(matches!(&construct_array.size, IArraySizeP::RuntimeSized));
  assert_eq!(construct_array.initializing_individual_elements, false);
  let first_arg = expect_1(&construct_array.args);
  assert_eq!(cast!(first_arg, IExpressionPE::ConstantInt).value, 42);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "#[](6, triple)");
  let construct_array = cast!(expr, IExpressionPE::ConstructArray);
  assert!(construct_array.type_pt.is_none());
  let mutability = construct_array.mutability_pt.as_ref().unwrap();
  assert_eq!(
    cast!(mutability, ITemplexPT::Mutability).mutability,
    MutabilityP::Immutable
  );
  assert!(construct_array.variability_pt.is_none());
  assert!(matches!(&construct_array.size, IArraySizeP::RuntimeSized));
  assert_eq!(construct_array.initializing_individual_elements, false);
  assert_eq!(construct_array.args.len(), 2);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "(3,)");
  let tuple = cast!(expr, IExpressionPE::Tuple);
  let first_element = expect_1(&tuple.elements);
  assert_eq!(cast!(first_element, IExpressionPE::ConstantInt).value, 3);
}
/*

  test("One element tuple") {
    compileExpressionExpect("(3,)") shouldHave
      { case TuplePE(_,Vector(ConstantIntPE(_,3,_))) => }
  }
*/
#[test]
fn zero_element_tuple() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "()");
  let tuple = cast!(expr, IExpressionPE::Tuple);
  assert_eq!(tuple.elements.len(), 0);
}
/*
  test("Zero element tuple") {
    compileExpressionExpect("()") shouldHave
      { case TuplePE(_,Vector()) => }
  }
*/
#[test]
fn two_element_tuple() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "(3,4)");
  let tuple = cast!(expr, IExpressionPE::Tuple);
  let (first_element, second_element) = expect_2(&tuple.elements);
  assert_eq!(cast!(first_element, IExpressionPE::ConstantInt).value, 3);
  assert_eq!(cast!(second_element, IExpressionPE::ConstantInt).value, 4);
}
/*
  test("Two element tuple") {
    compileExpressionExpect("(3,4)") shouldHave
      { case TuplePE(_,Vector(ConstantIntPE(_,3,_), ConstantIntPE(_,4,_))) => }
  }
*/
#[test]
fn three_element_tuple() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "(3,4,5)");
  let tuple = cast!(expr, IExpressionPE::Tuple);
  let (first_element, second_element, third_element) = expect_3(&tuple.elements);
  assert_eq!(cast!(first_element, IExpressionPE::ConstantInt).value, 3);
  assert_eq!(cast!(second_element, IExpressionPE::ConstantInt).value, 4);
  assert_eq!(cast!(third_element, IExpressionPE::ConstantInt).value, 5);
}
/*
  test("Three element tuple") {
    compileExpressionExpect("(3,4,5)") shouldHave
      { case TuplePE(_,Vector(ConstantIntPE(_,3,_), ConstantIntPE(_,4,_), ConstantIntPE(_,5,_))) => }
  }
*/
#[test]
fn three_element_tuple_trailing_comma() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "(3,4,5,)");
  let tuple = cast!(expr, IExpressionPE::Tuple);
  let (first_element, second_element, third_element) = expect_3(&tuple.elements);
  assert_eq!(cast!(first_element, IExpressionPE::ConstantInt).value, 3);
  assert_eq!(cast!(second_element, IExpressionPE::ConstantInt).value, 4);
  assert_eq!(cast!(third_element, IExpressionPE::ConstantInt).value, 5);
}
/*
  test("Three element tuple trailing comma") {
    compileExpressionExpect("(3,4,5,)") shouldHave
      { case TuplePE(_,Vector(ConstantIntPE(_,3,_), ConstantIntPE(_,4,_), ConstantIntPE(_,5,_))) => }
  }
*/
#[test]
fn transmigrate() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "a'x");
  let transmigrate = cast!(expr, IExpressionPE::Transmigrate);
  assert_eq!(transmigrate.target_region.str.str, "a");
  assert_lookup_name(transmigrate.inner.as_ref(), "x");
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "(something.callable)(3)");
  let function_call = cast!(expr, IExpressionPE::FunctionCall);
  let subexpr = cast!(
    function_call.callable_expr.as_ref(),
    IExpressionPE::SubExpression
  );
  let dot = cast!(subexpr.inner.as_ref(), IExpressionPE::Dot);
  assert_lookup_name(&dot.left, "something");
  assert_eq!(dot.member.str.str, "callable");
  expect_1(&function_call.arg_exprs);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "board[i]");
  let brace_call = cast!(expr, IExpressionPE::BraceCall);
  assert_lookup_name(brace_call.subject_expr.as_ref(), "board");
  let first_arg = expect_1(&brace_call.arg_exprs);
  assert_lookup_name(first_arg, "i");
  assert_eq!(brace_call.callable_readwrite, false);

  let expr2 = compile_expression_expect(&interner, &keywords, "this.board[i]");
  let brace_call2 = cast!(expr2, IExpressionPE::BraceCall);
  let dot = cast!(brace_call2.subject_expr.as_ref(), IExpressionPE::Dot);
  assert_lookup_name(&dot.left, "this");
  assert_eq!(dot.member.str.str, "board");
  let first_arg2 = expect_1(&brace_call2.arg_exprs);
  assert_lookup_name(first_arg2, "i");
  assert_eq!(brace_call2.callable_readwrite, false);
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "8 mod 2 == 0");
  let binary = cast!(expr, IExpressionPE::BinaryCall);
  assert_eq!(binary.function_name.str.str, "==");
  let left_binary = cast!(binary.left_expr.as_ref(), IExpressionPE::BinaryCall);
  assert_eq!(left_binary.function_name.str.str, "mod");
  assert_eq!(
    cast!(left_binary.left_expr.as_ref(), IExpressionPE::ConstantInt).value,
    8
  );
  assert_eq!(
    cast!(left_binary.right_expr.as_ref(), IExpressionPE::ConstantInt).value,
    2
  );
  assert_eq!(
    cast!(binary.right_expr.as_ref(), IExpressionPE::ConstantInt).value,
    0
  );
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "2 == 0 or false");
  let or = cast!(expr, IExpressionPE::Or);
  let left_binary = cast!(or.left.as_ref(), IExpressionPE::BinaryCall);
  assert_eq!(left_binary.function_name.str.str, "==");
  assert_eq!(
    cast!(left_binary.left_expr.as_ref(), IExpressionPE::ConstantInt).value,
    2
  );
  assert_eq!(
    cast!(left_binary.right_expr.as_ref(), IExpressionPE::ConstantInt).value,
    0
  );
  let right_block = &or.right;
  assert!(matches!(
    *right_block.inner,
    IExpressionPE::ConstantBool(ConstantBoolPE { value: false, .. })
  ));
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
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_expression_expect(&interner, &keywords, "(a => a + a)(3)");
  let function_call = cast!(expr, IExpressionPE::FunctionCall);
  let subexpr = cast!(
    function_call.callable_expr.as_ref(),
    IExpressionPE::SubExpression
  );
  let lambda = cast!(subexpr.inner.as_ref(), IExpressionPE::Lambda);
  let params = lambda.function.header.params.as_ref().unwrap();
  let first_param = expect_1(&params.params);
  let pattern = first_param.pattern.as_ref().unwrap();
  let destination = pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "a");
  assert!(pattern.templex.is_none());
  assert!(pattern.destructure.is_none());
  let block = lambda.function.body.as_ref().unwrap();
  let binary = cast!(&*block.inner, IExpressionPE::BinaryCall);
  assert_eq!(binary.function_name.str.str, "+");
  assert_lookup_name(binary.left_expr.as_ref(), "a");
  assert_lookup_name(binary.right_expr.as_ref(), "a");
  let first_arg = expect_1(&function_call.arg_exprs);
  assert_eq!(cast!(first_arg, IExpressionPE::ConstantInt).value, 3);
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
