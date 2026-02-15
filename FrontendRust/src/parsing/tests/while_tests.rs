/*
package dev.vale.parsing

import dev.vale.{Collector, Interner, StrI}
import dev.vale.parsing.ast._
import dev.vale.lexing.{Lexer, LexingIterator}
import dev.vale.options.GlobalOptions
import org.scalatest._


class WhileTests extends FunSuite with Collector with TestParseUtils {
*/
use bumpalo::Bump;
use crate::cast;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::*;

#[test]
fn simple_while_loop() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(&interner, &keywords, "while true {}");
  let while_ = cast!(expr, IExpressionPE::While);
  let condition = cast!(while_.condition.as_ref(), IExpressionPE::ConstantBool);
  assert!(condition.value);
  assert!(while_.body.maybe_pure.is_none());
  assert!(while_.body.maybe_default_region.is_none());
  cast!(while_.body.inner.as_ref(), IExpressionPE::Void);
}
/*
  test("Simple while loop") {
    compileBlockContentsExpect("while true {}") shouldHave {
      case WhilePE(_, ConstantBoolPE(_, true), BlockPE(_, None,None,VoidPE(_))) =>
    }
  }
*/
#[test]
fn result_after_while_loop() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(&interner, &keywords, "while true {} false");
  let consecutor = cast!(expr, IExpressionPE::Consecutor);
  let (while_expr, false_expr) = expect_2(&consecutor.inners);

  let while_ = cast!(while_expr, IExpressionPE::While);
  let condition = cast!(while_.condition.as_ref(), IExpressionPE::ConstantBool);
  assert!(condition.value);
  assert!(while_.body.maybe_pure.is_none());
  assert!(while_.body.maybe_default_region.is_none());
  cast!(while_.body.inner.as_ref(), IExpressionPE::Void);

  let false_ = cast!(false_expr, IExpressionPE::ConstantBool);
  assert!(!false_.value);
}
/*
  test("Result after while loop") {
    compileBlockContentsExpect("while true {} false") shouldHave {
      case WhilePE(_, ConstantBoolPE(_, true), BlockPE(_, None,None,VoidPE(_))) =>
    }
  }
*/
#[test]
fn while_with_condition_declarations() {
  let arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(&interner, &keywords, "while x = 4; x > 6 { }");
  let while_ = cast!(expr, IExpressionPE::While);

  let condition = cast!(while_.condition.as_ref(), IExpressionPE::Consecutor);
  let (let_x, x_greater_than_six) = expect_2(&condition.inners);

  let let_x = cast!(let_x, IExpressionPE::Let);
  let x_destination = let_x.pattern.destination.as_ref().unwrap();
  assert_destination_local_name(x_destination, "x");
  assert!(let_x.pattern.templex.is_none());
  assert!(let_x.pattern.destructure.is_none());
  let four = cast!(let_x.source.as_ref(), IExpressionPE::ConstantInt);
  assert_eq!(four.value, 4);
  assert_eq!(four.bits, None);

  let greater_than = cast!(x_greater_than_six, IExpressionPE::BinaryCall);
  assert_eq!(greater_than.function_name.str.str, ">");
  assert_lookup_name(greater_than.left_expr.as_ref(), "x");
  let six = cast!(greater_than.right_expr.as_ref(), IExpressionPE::ConstantInt);
  assert_eq!(six.value, 6);
  assert_eq!(six.bits, None);

  assert!(while_.body.maybe_pure.is_none());
  assert!(while_.body.maybe_default_region.is_none());
  cast!(while_.body.inner.as_ref(), IExpressionPE::Void);
}
/*
  test("While with condition declarations") {
    compileBlockContentsExpect("while x = 4; x > 6 { }") shouldHave {
      case WhilePE(_,
        ConsecutorPE(
          Vector(
            LetPE(_,PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("x"))), None)),None,None),ConstantIntPE(_,4,None)),
          BinaryCallPE(_,NameP(_,StrI(">")),LookupPE(LookupNameP(NameP(_, StrI("x"))),None),ConstantIntPE(_,6,None)))),
        BlockPE(_,None,None,VoidPE(_))) =>
    }
  }
}
*/