// cargo test --manifest-path FrontendRust/Cargo.toml --lib parsing::tests::statement_tests

/*
package dev.vale.parsing

import dev.vale.{Collector, Interner, StrI, vimpl, vwat}
import dev.vale.parsing.ast.{AugmentPE, BlockPE, BorrowP, ConsecutorPE, ConstantBoolPE, ConstantIntPE, ConstantStrPE, DestructPE, DestructureP, DotPE, EachPE, FunctionCallPE, IExpressionPE, IfPE, LetPE, LocalNameDeclarationP, LookupNameP, LookupPE, MutatePE, NameOrRunePT, NameP, PatternPP, Patterns, ReturnPE, TuplePE, UnletPE, VoidPE}
import dev.vale.parsing.ast._
import dev.vale.lexing._
import dev.vale.options.GlobalOptions
import org.scalatest._

class StatementTests extends FunSuite with Collector with TestParseUtils {
*/
use crate::cast;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::lexing::errors::ParseError;
use crate::parsing::ast::*;
use crate::parsing::tests::utils::*;

#[test]
fn simple_let() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(&interner, &keywords, "x = 4;");
  let consecutor = cast!(expr, IExpressionPE::Consecutor);
  let (let_x, final_void) = expect_2(&consecutor.inners);
  let let_x = cast!(let_x, IExpressionPE::Let);
  let destination = let_x.pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "x");
  assert!(let_x.pattern.templex.is_none());
  assert!(let_x.pattern.destructure.is_none());
  assert_eq!(cast!(let_x.source.as_ref(), IExpressionPE::ConstantInt).value, 4);
  cast!(final_void, IExpressionPE::Void);
}
/*
  test("Simple let") {
    compileBlockContentsExpect( "x = 4;") shouldHave {
      case LetPE(_, PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("x"))), None)), None, None), ConstantIntPE(_, 4, _)) =>
    }
  }
*/

#[test]
fn multiple_statements() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(&interner, &keywords, "4");
  let four = cast!(expr, IExpressionPE::ConstantInt);
  assert_eq!(four.value, 4);

  let expr = compile_block_contents_expect(&interner, &keywords, "4;");
  let consecutor = cast!(expr, IExpressionPE::Consecutor);
  let (four, final_void) = expect_2(&consecutor.inners);
  assert_eq!(cast!(four, IExpressionPE::ConstantInt).value, 4);
  cast!(final_void, IExpressionPE::Void);

  let expr = compile_block_contents_expect(&interner, &keywords, "4; 3");
  let consecutor = cast!(expr, IExpressionPE::Consecutor);
  let (four, three) = expect_2(&consecutor.inners);
  assert_eq!(cast!(four, IExpressionPE::ConstantInt).value, 4);
  assert_eq!(cast!(three, IExpressionPE::ConstantInt).value, 3);

  let expr = compile_block_contents_expect(&interner, &keywords, "4; 3;");
  let consecutor = cast!(expr, IExpressionPE::Consecutor);
  let (four, three, final_void) = expect_3(&consecutor.inners);
  assert_eq!(cast!(four, IExpressionPE::ConstantInt).value, 4);
  assert_eq!(cast!(three, IExpressionPE::ConstantInt).value, 3);
  cast!(final_void, IExpressionPE::Void);
}
/*
  test("multiple statements") {
    compileBlockContentsExpect(
      """4""".stripMargin) shouldHave {
      case ConstantIntPE(_, 4, _) =>
    }

    compileBlockContentsExpect(
      """4;""".stripMargin) shouldHave {
      case ConsecutorPE(Vector(ConstantIntPE(_, 4, _), VoidPE(_))) =>
    }

    compileBlockContentsExpect(
      """4; 3""".stripMargin) shouldHave {
      case ConsecutorPE(Vector(ConstantIntPE(_, 4, _), ConstantIntPE(_, 3, _))) =>
    }

    compileBlockContentsExpect(
      """4; 3;""".stripMargin) shouldHave {
      case ConsecutorPE(Vector(ConstantIntPE(_, 4, _), ConstantIntPE(_, 3, _), VoidPE(_))) =>
    }
  }
*/

#[test]
fn test_8() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "[x, y] = (4, 5);");
  let let_ = cast!(expr, IExpressionPE::Let);
  assert!(let_.pattern.destination.is_none());
  assert!(let_.pattern.templex.is_none());
  let destructure = let_.pattern.destructure.as_ref().unwrap();
  let (x_pattern, y_pattern) = expect_2(&destructure.patterns);
  let x_destination = x_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(x_destination, "x");
  assert!(x_pattern.templex.is_none());
  assert!(x_pattern.destructure.is_none());
  let y_destination = y_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(y_destination, "y");
  assert!(y_pattern.templex.is_none());
  assert!(y_pattern.destructure.is_none());
  let tuple = cast!(let_.source.as_ref(), IExpressionPE::Tuple);
  let (four, five) = expect_2(&tuple.elements);
  assert_eq!(cast!(four, IExpressionPE::ConstantInt).value, 4);
  assert_eq!(cast!(five, IExpressionPE::ConstantInt).value, 5);
}
/*
  test("8") {
    compileStatementExpect("[x, y] = (4, 5);") shouldHave {
      case LetPE(_,
          PatternPP(_,
            None,
            None,
            Some(
              DestructureP(_,
                Vector(
                  PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("x"))), None)),None,None),
                  PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("y"))), None)),None,None))))),
          TuplePE(_,Vector(ConstantIntPE(_, 4, _), ConstantIntPE(_, 5, _)))) =>
    }
  }
*/

#[test]
fn test_9() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "set x.a = 5;");
  let mutate = cast!(expr, IExpressionPE::Mutate);
  let dot = cast!(mutate.mutatee.as_ref(), IExpressionPE::Dot);
  assert_lookup_name(dot.left.as_ref(), "x");
  assert_eq!(dot.member.str.str, "a");
  assert_eq!(cast!(mutate.source.as_ref(), IExpressionPE::ConstantInt).value, 5);
}
/*
  test("9") {
    compileStatementExpect("set x.a = 5;") shouldHave {
      case MutatePE(_, DotPE(_, LookupPE(LookupNameP(NameP(_, StrI("x"))), None), _, NameP(_, StrI("a"))), ConstantIntPE(_, 5, _)) =>
    }
  }
*/

#[test]
fn test_1_pe() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, r#"set board.PE.PE.symbol = "v";"#);
  let mutate = cast!(expr, IExpressionPE::Mutate);
  let symbol_dot = cast!(mutate.mutatee.as_ref(), IExpressionPE::Dot);
  assert_eq!(symbol_dot.member.str.str, "symbol");
  let second_pe_dot = cast!(symbol_dot.left.as_ref(), IExpressionPE::Dot);
  assert_eq!(second_pe_dot.member.str.str, "PE");
  let first_pe_dot = cast!(second_pe_dot.left.as_ref(), IExpressionPE::Dot);
  assert_eq!(first_pe_dot.member.str.str, "PE");
  assert_lookup_name(first_pe_dot.left.as_ref(), "board");
  assert_eq!(cast!(mutate.source.as_ref(), IExpressionPE::ConstantStr).value, "v");
}
/*
  test("1PE") {
    compileStatementExpect("""set board.PE.PE.symbol = "v";""") shouldHave {
      case MutatePE(_, DotPE(_, DotPE(_, DotPE(_, LookupPE(LookupNameP(NameP(_, StrI("board"))), None), _, NameP(_, StrI("PE"))), _, NameP(_, StrI("PE"))), _, NameP(_, StrI("symbol"))), ConstantStrPE(_, "v")) =>
    }
  }
*/

#[test]
fn test_simple_let() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "x = 3;");
  let let_ = cast!(expr, IExpressionPE::Let);
  let destination = let_.pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "x");
  assert!(let_.pattern.templex.is_none());
  assert!(let_.pattern.destructure.is_none());
  assert_eq!(cast!(let_.source.as_ref(), IExpressionPE::ConstantInt).value, 3);
}
/*
  test("Test simple let") {
    compileStatementExpect("x = 3;") shouldHave {
      case LetPE(_,PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("x"))), None)),None,None),ConstantIntPE(_, 3, _)) =>
    }
  }
*/

#[test]
fn test_simple_mut() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "set x = 5;");
  let mutate = cast!(expr, IExpressionPE::Mutate);
  assert_lookup_name(mutate.mutatee.as_ref(), "x");
  assert_eq!(cast!(mutate.source.as_ref(), IExpressionPE::ConstantInt).value, 5);
}
/*
  test("Test simple mut") {
    compileStatementExpect("set x = 5;") shouldHave {
      case MutatePE(_, LookupPE(LookupNameP(NameP(_, StrI("x"))), None),ConstantIntPE(_, 5, _)) =>
    }
  }
*/

#[test]
fn test_expr_starting_with_return() {
  // This test is here because we had a bug where we didn't check that there
  // was whitespace after a "return".
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "retcode()");
  let function_call = cast!(expr, IExpressionPE::FunctionCall);
  assert_lookup_name(function_call.callable_expr.as_ref(), "retcode");
  assert!(function_call.arg_exprs.is_empty());
}
/*
  test("Test expr starting with return") {
    // This test is here because we had a bug where we didn't check that there
    // was whitespace after a "return".
    compileStatementExpect("retcode()") shouldHave {
      case FunctionCallPE(_,_,LookupPE(LookupNameP(NameP(_, StrI("retcode"))),None),Vector()) =>
    }
  }
*/

#[test]
fn test_inner_set() {
  // This test is here because we had a bug where we didn't check that there
  // was whitespace after a "return".
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "oldArray = set list.array = newArray;");
  let let_ = cast!(expr, IExpressionPE::Let);
  let destination = let_.pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "oldArray");
  assert!(let_.pattern.templex.is_none());
  assert!(let_.pattern.destructure.is_none());
  let mutate = cast!(let_.source.as_ref(), IExpressionPE::Mutate);
  let dot = cast!(mutate.mutatee.as_ref(), IExpressionPE::Dot);
  assert_lookup_name(dot.left.as_ref(), "list");
  assert_eq!(dot.member.str.str, "array");
  assert_lookup_name(mutate.source.as_ref(), "newArray");
}
/*
  test("Test inner set") {
    // This test is here because we had a bug where we didn't check that there
    // was whitespace after a "return".
    compileStatementExpect(
      "oldArray = set list.array = newArray;") shouldHave {
      case LetPE(_,
        PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("oldArray"))), None)),None,None),
        MutatePE(_,
          DotPE(_,LookupPE(LookupNameP(NameP(_, StrI("list"))),None),_,NameP(_, StrI("array"))),
          LookupPE(LookupNameP(NameP(_, StrI("newArray"))),None))) =>
    }
  }
*/

#[test]
fn test_if_statement_producing() {
  // This test is here because we had a bug where we didn't check that there
  // was whitespace after a "return".
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "if true { 3 } else { 4 }");
  let if_ = cast!(expr, IExpressionPE::If);
  assert!(cast!(if_.condition.as_ref(), IExpressionPE::ConstantBool).value);
  assert_eq!(cast!(if_.then_body.inner.as_ref(), IExpressionPE::ConstantInt).value, 3);
  assert_eq!(cast!(if_.else_body.inner.as_ref(), IExpressionPE::ConstantInt).value, 4);
}
/*
  test("Test if-statement producing") {
    // This test is here because we had a bug where we didn't check that there
    // was whitespace after a "return".
    compileStatementExpect(
      "if true { 3 } else { 4 }") shouldHave {
      case IfPE(_,
        ConstantBoolPE(_,true),
        BlockPE(_,None,None,ConstantIntPE(_,3,_)),
        BlockPE(_,None,None,ConstantIntPE(_,4,_))) =>
    }
  }
*/

#[test]
fn test_destruct() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "destruct x;");
  let destruct = cast!(expr, IExpressionPE::Destruct);
  assert_lookup_name(destruct.inner.as_ref(), "x");
}
/*
  test("Test destruct") {
    compileStatementExpect("destruct x;") shouldHave {
      case DestructPE(_,LookupPE(LookupNameP(NameP(_, StrI("x"))), None)) =>
    }
  }
*/

#[test]
fn test_unlet() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "unlet x");
  let unlet = cast!(expr, IExpressionPE::Unlet);
  assert_name(&unlet.name, "x");
}
/*
  test("Test unlet") {
    compileStatementExpect("unlet x") shouldHave {
      case UnletPE(_,LookupNameP(NameP(_, StrI("x")))) =>
    }
  }
*/

#[test]
fn dot_on_function_calls_result() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "Wizard(8).charges");
  let dot = cast!(expr, IExpressionPE::Dot);
  let wizard_call = cast!(dot.left.as_ref(), IExpressionPE::FunctionCall);
  assert_lookup_name(wizard_call.callable_expr.as_ref(), "Wizard");
  let first_arg = expect_1(&wizard_call.arg_exprs);
  assert_eq!(cast!(first_arg, IExpressionPE::ConstantInt).value, 8);
  assert_eq!(dot.member.str.str, "charges");
}
/*
  test("Dot on function call's result") {
    compileStatementExpect("Wizard(8).charges") shouldHave {
      case DotPE(_,
          FunctionCallPE(_,_,
            LookupPE(LookupNameP(NameP(_, StrI("Wizard"))), None),
            Vector(ConstantIntPE(_, 8, _))),
        _,
      NameP(_, StrI("charges"))) =>
    }
  }
*/

#[test]
fn let_with_pattern_with_only_a_capture() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "a = m;");
  let let_ = cast!(expr, IExpressionPE::Let);
  let destination = let_.pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "a");
  assert!(let_.pattern.templex.is_none());
  assert!(let_.pattern.destructure.is_none());
  assert_lookup_name(let_.source.as_ref(), "m");
}
/*
  test("Let with pattern with only a capture") {
    compileStatementExpect("a = m;") shouldHave {
      case LetPE(_,Patterns.capture("a"),LookupPE(LookupNameP(NameP(_, StrI("m"))), None)) =>
    }
  }
*/

#[test]
fn let_with_simple_pattern() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "a Moo = m;");
  let let_ = cast!(expr, IExpressionPE::Let);
  let destination = let_.pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "a");
  let templex = let_.pattern.templex.as_ref().unwrap();
  assert_templex_name(templex, "Moo");
  assert!(let_.pattern.destructure.is_none());
  assert_lookup_name(let_.source.as_ref(), "m");
}
/*
  test("Let with simple pattern") {
    compileStatementExpect("a Moo = m;") shouldHave {
      case LetPE(_,
        PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("a"))), None)),Some(NameOrRunePT(NameP(_, StrI("Moo")))),None),
        LookupPE(LookupNameP(NameP(_, StrI("m"))), None)) =>
    }
  }
*/

#[test]
fn let_with_simple_pattern_in_destructure() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "[a Moo] = m;");
  let let_ = cast!(expr, IExpressionPE::Let);
  assert!(let_.pattern.destination.is_none());
  assert!(let_.pattern.templex.is_none());
  let destructure = let_.pattern.destructure.as_ref().unwrap();
  let inner_pattern = expect_1(&destructure.patterns);
  let destination = inner_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "a");
  let inner_templex = inner_pattern.templex.as_ref().unwrap();
  assert_templex_name(inner_templex, "Moo");
  assert!(inner_pattern.destructure.is_none());
  assert_lookup_name(let_.source.as_ref(), "m");
}
/*
  test("Let with simple pattern in destructure") {
    compileStatementExpect("[a Moo] = m;") shouldHave {
      case LetPE(_,
          PatternPP(_,_,
            None,
            Some(DestructureP(_,Vector(PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("a"))), None)),Some(NameOrRunePT(NameP(_, StrI("Moo")))),None))))),
          LookupPE(LookupNameP(NameP(_, StrI("m"))), None)) =>
    }
  }
*/

#[test]
fn let_with_destructuring_pattern() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "Muta[ ] = m;");
  let let_ = cast!(expr, IExpressionPE::Let);
  assert!(let_.pattern.destination.is_none());
  let templex = let_.pattern.templex.as_ref().unwrap();
  assert_templex_name(templex, "Muta");
  let destructure = let_.pattern.destructure.as_ref().unwrap();
  assert!(destructure.patterns.is_empty());
  assert_lookup_name(let_.source.as_ref(), "m");
}
/*
  test("Let with destructuring pattern") {
    compileStatementExpect("Muta[ ] = m;") shouldHave {
      case LetPE(_,PatternPP(_,None,Some(NameOrRunePT(NameP(_, StrI("Muta")))),Some(DestructureP(_,Vector()))),LookupPE(LookupNameP(NameP(_, StrI("m"))), None)) =>
    }
  }
*/

#[test]
fn destructure_pattern_with_let_and_set() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "[a, set x] = m;");
  let let_ = cast!(expr, IExpressionPE::Let);
  assert!(let_.pattern.destination.is_none());
  assert!(let_.pattern.templex.is_none());
  let destructure = let_.pattern.destructure.as_ref().unwrap();
  let (a_pattern, x_pattern) = expect_2(&destructure.patterns);
  let a_destination = a_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(a_destination, "a");
  assert!(a_destination.mutate.is_none());
  assert!(a_pattern.templex.is_none());
  assert!(a_pattern.destructure.is_none());
  let x_destination = x_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(x_destination, "x");
  assert!(x_destination.mutate.is_some());
  assert!(x_pattern.templex.is_none());
  assert!(x_pattern.destructure.is_none());
}
/*
  test("Destructure pattern with let and set") {
    compileStatementExpect("[a, set x] = m;") shouldHave {
      case LetPE(_,
        PatternPP(_,
          None,None,
          Some(
            DestructureP(_,
              Vector(
                PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_,StrI("a"))),None)),None,None),
                PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_,StrI("x"))),Some(_))),None,None))))),
        _) =>
    }
  }
*/

#[test]
fn ret() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "return 3;");
  let ret = cast!(expr, IExpressionPE::Return);
  assert_eq!(cast!(ret.expr.as_ref(), IExpressionPE::ConstantInt).value, 3);
}
/*
  test("Ret") {
    compileStatementExpect("return 3;") shouldHave {
      case ReturnPE(_,ConstantIntPE(_, 3, _)) =>
    }
  }
*/

#[test]
fn foreach() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "foreach i in myList { }");
  let each = cast!(expr, IExpressionPE::Each);
  assert!(each.maybe_pure.is_none());
  let destination = each.entry_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "i");
  assert!(each.entry_pattern.templex.is_none());
  assert!(each.entry_pattern.destructure.is_none());
  assert_lookup_name(each.iterable_expr.as_ref(), "myList");
  cast!(each.body.inner.as_ref(), IExpressionPE::Void);
}
/*
  test("foreach") {
    compileStatementExpect("foreach i in myList { }") shouldHave {
      case EachPE(_,
      None,
      PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("i"))), None)),None,None),
      _,
      LookupPE(LookupNameP(NameP(_, StrI("myList"))),None),
      BlockPE(_,None,None,_)) =>
    }
  }
*/

#[test]
fn foreach_with_borrow() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "foreach i in &myList { }");
  let each = cast!(expr, IExpressionPE::Each);
  assert!(each.maybe_pure.is_none());
  let destination = each.entry_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "i");
  assert!(each.entry_pattern.templex.is_none());
  assert!(each.entry_pattern.destructure.is_none());
  let borrow = cast!(each.iterable_expr.as_ref(), IExpressionPE::Augment);
  assert_eq!(borrow.target_ownership, OwnershipP::Borrow);
  assert_lookup_name(borrow.inner.as_ref(), "myList");
  cast!(each.body.inner.as_ref(), IExpressionPE::Void);
}
/*
  test("foreach with borrow") {
    compileStatementExpect("foreach i in &myList { }") shouldHave {
      case EachPE(_,
      None,
      PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("i"))), None)),None,None),
      _,
      AugmentPE(_, BorrowP, LookupPE(LookupNameP(NameP(_, StrI("myList"))),None)),
      BlockPE(_,None,None,_)) =>
    }
  }
*/

#[test]
fn foreach_with_two_receivers() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "foreach [a, b] in myList { }");
  let each = cast!(expr, IExpressionPE::Each);
  assert!(each.maybe_pure.is_none());
  assert!(each.entry_pattern.destination.is_none());
  assert!(each.entry_pattern.templex.is_none());
  let destructure = each.entry_pattern.destructure.as_ref().unwrap();
  let (a_pattern, b_pattern) = expect_2(&destructure.patterns);
  let a_destination = a_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(a_destination, "a");
  assert!(a_pattern.templex.is_none());
  assert!(a_pattern.destructure.is_none());
  let b_destination = b_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(b_destination, "b");
  assert!(b_pattern.templex.is_none());
  assert!(b_pattern.destructure.is_none());
  assert_lookup_name(each.iterable_expr.as_ref(), "myList");
  cast!(each.body.inner.as_ref(), IExpressionPE::Void);
}
/*
  test("foreach with two receivers") {
    compileStatementExpect("foreach [a, b] in myList { }") shouldHave {
      case EachPE(_,
        None,
        PatternPP(_,
          None,None,
          Some(
            DestructureP(_,
              Vector(
                PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("a"))), None)),None,None),
                PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("b"))), None)),None,None))))),
        _,
        LookupPE(LookupNameP(NameP(_, StrI("myList"))),None),
        BlockPE(_,None,None,_)) =>
    }
  }
*/

#[test]
fn foreach_complex_iterable() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "foreach i in myList = 3; myList { }");
  let each = cast!(expr, IExpressionPE::Each);
  assert!(each.maybe_pure.is_none());
  let destination = each.entry_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "i");
  assert!(each.entry_pattern.templex.is_none());
  assert!(each.entry_pattern.destructure.is_none());
  let iterable_consecutor = cast!(each.iterable_expr.as_ref(), IExpressionPE::Consecutor);
  let (let_mylist, lookup_mylist) = expect_2(&iterable_consecutor.inners);
  let let_mylist = cast!(let_mylist, IExpressionPE::Let);
  let mylist_destination = let_mylist.pattern.destination.as_ref().unwrap();
  assert_destination_local_name(mylist_destination, "myList");
  assert!(let_mylist.pattern.templex.is_none());
  assert!(let_mylist.pattern.destructure.is_none());
  assert_eq!(cast!(let_mylist.source.as_ref(), IExpressionPE::ConstantInt).value, 3);
  assert_lookup_name(lookup_mylist, "myList");
  cast!(each.body.inner.as_ref(), IExpressionPE::Void);
}
/*
  test("foreach complex iterable") {
    compileStatementExpect("foreach i in myList = 3; myList { }") shouldHave {
      case EachPE(_,
        None,
        PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("i"))), None)),None,None),
        _,
        ConsecutorPE(
          Vector(
            LetPE(_,PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("myList"))), None)),None,None),ConstantIntPE(_,3,_)),
            LookupPE(LookupNameP(NameP(_, StrI("myList"))),None))),
        BlockPE(_,None,None,VoidPE(_))) =>
    }
  }
*/

#[test]
fn multiple_statements_2() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  compile_block_contents_expect(
    &interner,
    &keywords,
    "
      42;
      43;
      ",
  );
}
/*
  test("Multiple statements") {
    compileBlockContentsExpect(
      """
        |42;
        |43;
        |""".stripMargin)
  }
*/

#[test]
fn if_and_another_statement() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  compile_block_contents_expect(
    &interner,
    &keywords,
    "
      newCapacity = if (true) { 1 } else { 2 };
      newArray = 3;
      ",
  );
}
/*
  test("If and another statement") {
    compileBlockContentsExpect(
      """
        |newCapacity = if (true) { 1 } else { 2 };
        |newArray = 3;
        |""".stripMargin)
  }
*/

#[test]
fn test_blocks_trailing_void_presence() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(&interner, &keywords, "moo()");
  let function_call = cast!(expr, IExpressionPE::FunctionCall);
  assert_lookup_name(function_call.callable_expr.as_ref(), "moo");
  assert!(function_call.arg_exprs.is_empty());

  let expr = compile_block_contents_expect(&interner, &keywords, "moo();");
  let consecutor = cast!(expr, IExpressionPE::Consecutor);
  let (moo_call, final_void) = expect_2(&consecutor.inners);
  let moo_call = cast!(moo_call, IExpressionPE::FunctionCall);
  assert_lookup_name(moo_call.callable_expr.as_ref(), "moo");
  assert!(moo_call.arg_exprs.is_empty());
  cast!(final_void, IExpressionPE::Void);
}
/*
  test("Test block's trailing void presence") {
    compileBlockContentsExpect(
      "moo()") shouldHave {
      case FunctionCallPE(_, _, LookupPE(LookupNameP(NameP(_, StrI("moo"))), None), Vector()) =>
    }

    compileBlockContentsExpect(
      "moo();") shouldHave {
      case ConsecutorPE(Vector(FunctionCallPE(_, _, LookupPE(LookupNameP(NameP(_, StrI("moo"))), None), Vector()), VoidPE(_))) =>
    }
  }
*/

#[test]
fn block_with_statement_and_result() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(
    &interner,
    &keywords,
    "
      b;
      a
    ",
  );
  let consecutor = cast!(expr, IExpressionPE::Consecutor);
  let (b_lookup, a_lookup) = expect_2(&consecutor.inners);
  assert_lookup_name(b_lookup, "b");
  assert_lookup_name(a_lookup, "a");
}
/*
  test("Block with statement and result") {
    compileBlockContentsExpect(
      """
        |b;
        |a
      """.stripMargin) shouldHave {
      case Vector(LookupPE(LookupNameP(NameP(_, StrI("b"))), None), LookupPE(LookupNameP(NameP(_, StrI("a"))), None)) =>
    }
  }
*/

#[test]
fn block_with_result() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_statement_expect(&interner, &keywords, "3");
  assert_eq!(cast!(expr, IExpressionPE::ConstantInt).value, 3);
}
/*
  test("Block with result") {
    compileStatementExpect("3") shouldHave {
      case ConstantIntPE(_, 3, _) =>
    }
  }
*/

#[test]
fn block_with_result_that_could_be_an_expr() {
  // = doThings(a); could be misinterpreted as an expression doThings(=, a) if we're
  // not careful.
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(
    &interner,
    &keywords,
    "
      a = 2;
      doThings(a)
    ",
  );
  let consecutor = cast!(expr, IExpressionPE::Consecutor);
  let (let_a, do_things_call) = expect_2(&consecutor.inners);
  let let_a = cast!(let_a, IExpressionPE::Let);
  let destination = let_a.pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "a");
  assert!(let_a.pattern.templex.is_none());
  assert!(let_a.pattern.destructure.is_none());
  assert_eq!(cast!(let_a.source.as_ref(), IExpressionPE::ConstantInt).value, 2);
  let do_things_call = cast!(do_things_call, IExpressionPE::FunctionCall);
  assert_lookup_name(do_things_call.callable_expr.as_ref(), "doThings");
  let first_arg = expect_1(&do_things_call.arg_exprs);
  assert_lookup_name(first_arg, "a");
}
/*
  test("Block with result that could be an expr") {
    // = doThings(a); could be misinterpreted as an expression doThings(=, a) if we're
    // not careful.
    compileBlockContentsExpect(
      """
        |a = 2;
        |doThings(a)
      """.stripMargin) shouldHave {
      case Vector(
        LetPE(_, PatternPP(_, Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("a"))), None)), None, None), ConstantIntPE(_, 2, _)),
        FunctionCallPE(_, _, LookupPE(LookupNameP(NameP(_, StrI("doThings"))), None), Vector(LookupPE(LookupNameP(NameP(_, StrI("a"))), None)))) =>
    }
  }
*/

#[test]
fn mutating_as_statement() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(&interner, &keywords, "set x = 6;");
  let consecutor = cast!(expr, IExpressionPE::Consecutor);
  let (mutate, final_void) = expect_2(&consecutor.inners);
  let mutate = cast!(mutate, IExpressionPE::Mutate);
  assert_lookup_name(mutate.mutatee.as_ref(), "x");
  assert_eq!(cast!(mutate.source.as_ref(), IExpressionPE::ConstantInt).value, 6);
  cast!(final_void, IExpressionPE::Void);
}
/*
  test("Mutating as statement") {
    val program =
      compileBlockContentsExpect(
        "set x = 6;")
    program shouldHave {
      case MutatePE(_,LookupPE(LookupNameP(NameP(_, StrI("x"))), None),ConstantIntPE(_, 6, _)) =>
    }
  }
*/

#[test]
fn lone_block() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(
    &interner,
    &keywords,
    "
      block {
        a
      }
    ",
  );
  let block = cast!(expr, IExpressionPE::Block);
  assert_lookup_name(block.inner.as_ref(), "a");
}
/*
  test("Lone block") {
    compileBlockContentsExpect(
      """
        |block {
        |  a
        |}
      """.stripMargin) shouldHave {
      case BlockPE(_,None,None,LookupPE(LookupNameP(NameP(_, StrI("a"))),None)) =>
    }
  }
*/

#[test]
fn pure_block() {
  // Just make sure it parses, so that we can highlight it.
  // The pure block feature doesn't actually exist yet.
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  compile_block_contents_expect(
    &interner,
    &keywords,
    "
      pure block {
        a
      }
    ",
  );
}
/*
  test("Pure block") {
    // Just make sure it parses, so that we can highlight it.
    // The pure block feature doesn't actually exist yet.
    compileBlockContentsExpect(
      """
        |pure block {
        |  a
        |}
      """.stripMargin)
  }
*/

#[test]
fn unsafe_pure_block() {
  // Just make sure it parses, so that we can highlight it.
  // The unsafe pure block feature doesn't actually exist yet.
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  compile_block_contents_expect(
    &interner,
    &keywords,
    "
      unsafe pure block {
        a
      }
    ",
  );
}
/*
  test("Unsafe pure block") {
    // Just make sure it parses, so that we can highlight it.
    // The unsafe pure block feature doesn't actually exist yet.
    compileBlockContentsExpect(
      """
        |unsafe pure block {
        |  a
        |}
      """.stripMargin)
  }
*/

#[test]
fn report_leaving_out_semicolon_or_ending_body_after_expression_for_square() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let err = compile_statement(
    &interner,
    &keywords,
    "
      block {
        floop() ]
      }
      ",
  )
  .unwrap_err();
  assert!(matches!(err, ParseError::BadStartOfStatementError(_)));
}
/*
  test("Report leaving out semicolon or ending body after expression, for square") {
    compileStatement(
      """
        |block {
        |  floop() ]
        |}
        """.stripMargin).expectErr() match {
      case BadStartOfStatementError(_) =>
    }
  }
*/

#[test]
fn empty_block() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(
    &interner,
    &keywords,
    "
      block {
      }
      return 3;
      ",
  );
  let consecutor = cast!(expr, IExpressionPE::Consecutor);
  let (block, ret, final_void) = expect_3(&consecutor.inners);
  let block = cast!(block, IExpressionPE::Block);
  cast!(block.inner.as_ref(), IExpressionPE::Void);
  let ret = cast!(ret, IExpressionPE::Return);
  assert_eq!(cast!(ret.expr.as_ref(), IExpressionPE::ConstantInt).value, 3);
  cast!(final_void, IExpressionPE::Void);
}
/*
  test("Empty block") {
    compileBlockContentsExpect(
      """
        |block {
        |}
        |return 3;
    """.stripMargin) match {
      case ConsecutorPE(
        Vector(
          BlockPE(_,None,None,VoidPE(_)),
          ReturnPE(_,ConstantIntPE(_,3,None)), VoidPE(_))) =>
    }
  }
*/

#[test]
fn cant_use_set_as_a_local_name() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let err = compile_statement(&interner, &keywords, "[set] = (6,)").unwrap_err();
  assert!(matches!(
    err,
    ParseError::CantUseThatLocalName { ref name, .. } if name == "set"
  ));
}
/*
  test("Cant use set as a local name") {
    val error = compileStatement(
      """[set] = (6,)""".stripMargin).expectErr()
    error match {
      case CantUseThatLocalName(_, "set") =>
    }
  }
*/

#[test]
fn foreach_2() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(
    &interner,
    &keywords,
    "
      foreach i in a {
        i
      }
      ",
  );
  let each = cast!(expr, IExpressionPE::Each);
  assert!(each.maybe_pure.is_none());
  let destination = each.entry_pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "i");
  assert!(each.entry_pattern.templex.is_none());
  assert!(each.entry_pattern.destructure.is_none());
  assert_lookup_name(each.iterable_expr.as_ref(), "a");
  assert_lookup_name(each.body.inner.as_ref(), "i");
}
/*
  test("foreach 2") {
    val programS =
      compileBlockContentsExpect(
        """
          |foreach i in a {
          |  i
          |}
          |""".stripMargin)
    programS shouldHave {
      case EachPE(_,
        None,
        PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, StrI("i"))), None)),None,None),
        _,
        LookupPE(LookupNameP(NameP(_, StrI("a"))),None),
        BlockPE(_,None,None,
          LookupPE(LookupNameP(NameP(_, StrI("i"))),None))) =>
    }
  }
*/

#[test]
fn foreach_expr() {
  let interner = Interner::new();
  let keywords = Keywords::new(&interner);
  let expr = compile_block_contents_expect(
    &interner,
    &keywords,
    "
      a = foreach i in c { i };
      ",
  );
  let consecutor = cast!(expr, IExpressionPE::Consecutor);
  let (let_a, final_void) = expect_2(&consecutor.inners);
  let let_a = cast!(let_a, IExpressionPE::Let);
  let destination = let_a.pattern.destination.as_ref().unwrap();
  assert_destination_local_name(destination, "a");
  assert!(let_a.pattern.templex.is_none());
  assert!(let_a.pattern.destructure.is_none());
  cast!(let_a.source.as_ref(), IExpressionPE::Each);
  cast!(final_void, IExpressionPE::Void);
}
/*
  test("foreach expr") {
    val programS =
      compileBlockContentsExpect(
        """
          |a = foreach i in c { i };
          |""".stripMargin)
    programS shouldHave {
      case ConsecutorPE(Vector(
        LetPE(_,
          PatternPP(_,Some(DestinationLocalP(LocalNameDeclarationP(NameP(_,StrI("a"))), None)),None,None),
          EachPE(_,_,_,_,_,_)),
        VoidPE(_))) =>
    }
  }

}
*/