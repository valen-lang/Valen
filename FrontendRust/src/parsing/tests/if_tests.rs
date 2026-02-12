// Run with: cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing::tests::if_tests

/*
package dev.vale.parsing

import dev.vale.{Collector, StrI, vimpl}
import dev.vale.parsing.ast.{AugmentPE, BinaryCallPE, BlockPE, BorrowP, ConsecutorPE, ConstantBoolPE, ConstantIntPE, DestructureP, FunctionCallPE, IfPE, LetPE, LocalNameDeclarationP, LookupNameP, LookupPE, MethodCallPE, NameP, NotPE, PatternPP, VoidPE}
import dev.vale.parsing.ast._
import dev.vale.options.GlobalOptions
import org.scalatest._


class IfTests extends FunSuite with Matchers with Collector with TestParseUtils {
*/
use crate::cast;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::*;

#[test]
fn ifs() {
  let expr = compile_expression_expect("if true { doBlarks(&x) } else { }");
  let if_ = cast!(expr, IExpressionPE::If);

  let condition = cast!(if_.condition.as_ref(), IExpressionPE::ConstantBool);
  assert!(condition.value);

  assert!(if_.then_body.maybe_pure.is_none());
  assert!(if_.then_body.maybe_default_region.is_none());
  let then_call = cast!(if_.then_body.inner.as_ref(), IExpressionPE::FunctionCall);
  assert_lookup_name(then_call.callable_expr.as_ref(), "doBlarks");
  let first_arg = expect_1(&then_call.arg_exprs);
  let borrow_x = cast!(first_arg, IExpressionPE::Augment);
  assert_eq!(borrow_x.target_ownership, OwnershipP::Borrow);
  assert_lookup_name(borrow_x.inner.as_ref(), "x");

  assert!(if_.else_body.maybe_pure.is_none());
  assert!(if_.else_body.maybe_default_region.is_none());
  cast!(if_.else_body.inner.as_ref(), IExpressionPE::Void);
}
/*
  test("ifs") {
    compileExpressionExpect("if true { doBlarks(&x) } else { }") shouldHave {

      case IfPE(_,
        ConstantBoolPE(_, true),
        BlockPE(_,None,None,
          FunctionCallPE(_,_,LookupPE(LookupNameP(NameP(_, StrI("doBlarks"))),None),
            Vector(
              AugmentPE(_,BorrowP,LookupPE(LookupNameP(NameP(_, StrI("x"))),None))))),
        BlockPE(_,None,None,VoidPE(_))) =>
    }
  }
*/
#[test]
fn if_let() {
  let expr = compile_expression_expect("if [u] = a {}");
  let if_ = cast!(expr, IExpressionPE::If);

  let let_ = cast!(if_.condition.as_ref(), IExpressionPE::Let);
  assert!(let_.pattern.destination.is_none());
  assert!(let_.pattern.templex.is_none());
  let destructure = let_.pattern.destructure.as_ref().unwrap();
  let u_pattern = expect_1(&destructure.patterns);
  let u_destination = u_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(u_destination, "u");
  assert!(u_pattern.templex.is_none());
  assert!(u_pattern.destructure.is_none());
  assert_lookup_name(let_.source.as_ref(), "a");

  assert!(if_.then_body.maybe_pure.is_none());
  assert!(if_.then_body.maybe_default_region.is_none());
  cast!(if_.then_body.inner.as_ref(), IExpressionPE::Void);
  assert!(if_.else_body.maybe_pure.is_none());
  assert!(if_.else_body.maybe_default_region.is_none());
  cast!(if_.else_body.inner.as_ref(), IExpressionPE::Void);
}
/*
  test("if let") {
    compileExpressionExpect("if [u] = a {}") shouldHave {
      case IfPE(_,
        LetPE(_,
          PatternPP(_,None,None,
            Some(
              DestructureP(_,
                Vector(
                  PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("u"))), None)),None,None))))),
          LookupPE(LookupNameP(NameP(_, StrI("a"))),None)),
        BlockPE(_,None,None,VoidPE(_)),
        BlockPE(_,None,None,VoidPE(_))) =>
    }
  }
*/
#[test]
fn if_with_condition_declarations() {
  let expr = compile_expression_expect("if x = 4; not x.isEmpty() { }");
  let if_ = cast!(expr, IExpressionPE::If);

  let condition = cast!(if_.condition.as_ref(), IExpressionPE::Consecutor);
  let (let_x, not_x_isempty) = expect_2(&condition.inners);
  let let_x = cast!(let_x, IExpressionPE::Let);
  let x_destination = let_x.pattern.destination.as_ref().unwrap();
  assert_destination_local_name(x_destination, "x");
  assert!(let_x.pattern.templex.is_none());
  assert!(let_x.pattern.destructure.is_none());
  let four = cast!(let_x.source.as_ref(), IExpressionPE::ConstantInt);
  assert_eq!(four.value, 4);
  assert_eq!(four.bits, None);

  let not = cast!(not_x_isempty, IExpressionPE::Not);
  let x_isempty = cast!(not.inner.as_ref(), IExpressionPE::MethodCall);
  assert_lookup_name(x_isempty.subject_expr.as_ref(), "x");
  assert_name(&x_isempty.method_lookup.name, "isEmpty");
  assert!(x_isempty.method_lookup.template_args.is_none());
  assert!(x_isempty.arg_exprs.is_empty());

  assert!(if_.then_body.maybe_pure.is_none());
  assert!(if_.then_body.maybe_default_region.is_none());
  cast!(if_.then_body.inner.as_ref(), IExpressionPE::Void);
  assert!(if_.else_body.maybe_pure.is_none());
  assert!(if_.else_body.maybe_default_region.is_none());
  cast!(if_.else_body.inner.as_ref(), IExpressionPE::Void);
}
/*
  test("If with condition declarations") {
    compileExpressionExpect("if x = 4; not x.isEmpty() { }") shouldHave {
      case IfPE(_,
        ConsecutorPE(
          Vector(
            LetPE(_,PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("x"))), None)),None,None),ConstantIntPE(_,4,None)),
            NotPE(_,MethodCallPE(_,LookupPE(LookupNameP(NameP(_, StrI("x"))),None),_,LookupPE(LookupNameP(NameP(_, StrI("isEmpty"))),None),Vector())))),
        BlockPE(_,None,None,VoidPE(_)),
        BlockPE(_,None,None,VoidPE(_))) =>
    }
  }
*/
#[test]
fn if_with_condition_declarations_and_block_contents() {
  let expr = compile_block_contents_expect("newLen = if num == 0 { 1 } else { 2 };");
  let consecutor = cast!(expr, IExpressionPE::Consecutor);
  let (let_new_len, final_void) = expect_2(&consecutor.inners);

  let let_new_len = cast!(let_new_len, IExpressionPE::Let);
  let new_len_destination = let_new_len.pattern.destination.as_ref().unwrap();
  assert_destination_local_name(new_len_destination, "newLen");
  assert!(let_new_len.pattern.templex.is_none());
  assert!(let_new_len.pattern.destructure.is_none());

  let if_ = cast!(let_new_len.source.as_ref(), IExpressionPE::If);
  let equals = cast!(if_.condition.as_ref(), IExpressionPE::BinaryCall);
  assert_eq!(equals.function_name.str.str, "==");
  assert_lookup_name(equals.left_expr.as_ref(), "num");
  assert_eq!(
    cast!(equals.right_expr.as_ref(), IExpressionPE::ConstantInt).value,
    0
  );

  assert!(if_.then_body.maybe_pure.is_none());
  assert!(if_.then_body.maybe_default_region.is_none());
  assert_eq!(
    cast!(if_.then_body.inner.as_ref(), IExpressionPE::ConstantInt).value,
    1
  );
  assert!(if_.else_body.maybe_pure.is_none());
  assert!(if_.else_body.maybe_default_region.is_none());
  let two = cast!(if_.else_body.inner.as_ref(), IExpressionPE::ConstantInt);
  assert_eq!(two.value, 2);
  assert_eq!(two.bits, None);

  cast!(final_void, IExpressionPE::Void);
}
/*
  test("19") {
    compileBlockContentsExpect(
      "newLen = if num == 0 { 1 } else { 2 };") shouldHave {
      case ConsecutorPE(
        Vector(
          LetPE(_,
            PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("newLen"))), None)),None,None),
            IfPE(_,
              BinaryCallPE(_,NameP(_,StrI("==")),LookupPE(LookupNameP(NameP(_, StrI("num"))),None),ConstantIntPE(_,0,_)),
              BlockPE(_,None,None,ConstantIntPE(_,1,_)),
              BlockPE(_,None,None,ConstantIntPE(_,2,None)))),
          VoidPE(_))) =>
    }
  }
}
*/
