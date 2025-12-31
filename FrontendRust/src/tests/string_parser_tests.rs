/// String parsing tests
/// Mirrors Frontend/ParsingPass/src/dev/vale/parsing/StringParserTests.scala lines 1-105

use crate::tests::test_parse_utils::*;
use crate::parsing::ast::*;
use crate::{should_have};
use crate::interner::{Interner};
use crate::keywords::Keywords;
use crate::lexing::lexer::Lexer;
use crate::lexing::lexing_iterator::LexingIterator;
use std::sync::{Arc};

/// Test: Simple string
/// Mirrors StringParserTests.scala line 13
#[test]
fn test_simple_string() {
    let expr = compile_expression_expect(r#""moo""#);
    should_have!(expr, IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
        assert_eq!(value, "moo");
    });
}

/// Test: String with newline
/// Mirrors StringParserTests.scala line 18
#[test]
fn test_string_with_newline() {
    let expr = compile_expression_expect("\"\"\"m\noo\"\"\"");
    should_have!(expr, IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
        assert_eq!(value, "m\noo");
    });
}

/// Test: String with escaped braces
/// Mirrors StringParserTests.scala line 23
#[test]
fn test_string_with_escaped_braces() {
    let expr = compile_expression_expect(r#""\{\}""#);
    should_have!(expr, IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
        assert_eq!(value, "{}");
    });
}

/// Test: String with quote inside
/// Mirrors StringParserTests.scala line 28
#[test]
fn test_string_with_quote_inside() {
    let expr = compile_expression_expect(r#""m\"oo""#);
    should_have!(expr, IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
        assert_eq!(value, "m\"oo");
    });
}

/// Test: String with unicode
/// Mirrors StringParserTests.scala line 33
#[test]
fn test_string_with_unicode() {
    // Test parseFourDigitHexNum directly
    let interner = Arc::new(Interner::new());
    let keywords = Arc::new(Keywords::new(&interner));
    let lexer = Lexer::new(interner.clone(), keywords.clone());
    
    assert_eq!(
        lexer.parse_four_digit_hex_num(&mut LexingIterator::new("000a".to_string()), 0),
        Some(10)
    );

    // Test unicode escape sequences in strings
    let expr = compile_expression_expect(r#""\u000a""#);
    should_have!(expr, IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
        assert_eq!(value, "\n");
    });

    let expr = compile_expression_expect(r#""\u001b""#);
    should_have!(expr, IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
        assert_eq!(value, "\u{001b}");
    });

    let expr = compile_expression_expect(r#""foo\u001bbar""#);
    should_have!(expr, IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
        assert_eq!(value, "foo\u{001b}bar");
    });
}

/// Test: String with apostrophe inside
/// Mirrors StringParserTests.scala line 55
#[test]
fn test_string_with_apostrophe_inside() {
    let expr = compile_expression_expect(r#""m'oo""#);
    should_have!(expr, IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
        assert_eq!(value, "m'oo");
    });

    let expr = compile_expression_expect(r#""""m'oo""""#);
    should_have!(expr, IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
        assert_eq!(value, "m'oo");
    });
}

/// Test: Short string interpolating
/// Mirrors StringParserTests.scala line 62
#[test]
fn test_short_string_interpolating() {
    let expr = compile_expression_expect(r#""bl{4}rg""#);
    should_have!(expr, IExpressionPE::StrInterpolate(StrInterpolatePE { parts, .. }) => {
        assert_eq!(parts.len(), 3);
        should_have!(&parts[0], IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
            assert_eq!(value, "bl");
        });
        should_have!(&parts[1], IExpressionPE::ConstantInt(ConstantIntPE { value, .. }) => {
            assert_eq!(*value, 4);
        });
        should_have!(&parts[2], IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
            assert_eq!(value, "rg");
        });
    });
}

/// Test: Short string interpolating with call
/// Mirrors StringParserTests.scala line 67
#[test]
fn test_short_string_interpolating_with_call() {
    let expr = compile_expression_expect(r#""bl{ns(4)}rg""#);
    should_have!(expr, IExpressionPE::StrInterpolate(StrInterpolatePE { parts, .. }) => {
        assert_eq!(parts.len(), 3);
        should_have!(&parts[0], IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
            assert_eq!(value, "bl");
        });
        should_have!(&parts[1], IExpressionPE::FunctionCall(FunctionCallPE { callable_expr, arg_exprs, .. }) => {
            should_have!(callable_expr.as_ref(), IExpressionPE::Lookup(LookupPE { name: IImpreciseNameP::LookupName(name_p), .. }) => {
                assert_eq!(name_p.str.str, "ns");
            });
            assert_eq!(arg_exprs.len(), 1);
            should_have!(&arg_exprs[0], IExpressionPE::ConstantInt(ConstantIntPE { value, .. }) => {
                assert_eq!(*value, 4);
            });
        });
        should_have!(&parts[2], IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
            assert_eq!(value, "rg");
        });
    });
}

/// Test: Long string interpolating
/// Mirrors StringParserTests.scala line 78
#[test]
fn test_long_string_interpolating() {
    let expr = compile_expression_expect(r#""""bl{4}rg""""#);
    should_have!(expr, IExpressionPE::StrInterpolate(StrInterpolatePE { parts, .. }) => {
        assert_eq!(parts.len(), 3);
        should_have!(&parts[0], IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
            assert_eq!(value, "bl");
        });
        should_have!(&parts[1], IExpressionPE::ConstantInt(ConstantIntPE { value, .. }) => {
            assert_eq!(*value, 4);
        });
        should_have!(&parts[2], IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
            assert_eq!(value, "rg");
        });
    });
}

/// Test: Long string doesn't interpolate with brace then newline
/// Mirrors StringParserTests.scala line 83
#[test]
fn test_long_string_doesnt_interpolate_with_brace_then_newline() {
    let expr = compile_expression_expect("\"\"\"bl{\n4}rg\"\"\"");
    should_have!(expr, IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
        assert_eq!(value, "bl{\n4}rg");
    });
}

/// Test: Long string interpolates with brace then backslash
/// Mirrors StringParserTests.scala line 89
#[test]
fn test_long_string_interpolates_with_brace_then_backslash() {
    let expr = compile_expression_expect("\"\"\"bl{\\\n4}rg\"\"\"");
    should_have!(expr, IExpressionPE::StrInterpolate(StrInterpolatePE { parts, .. }) => {
        assert_eq!(parts.len(), 3);
        should_have!(&parts[0], IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
            assert_eq!(value, "bl");
        });
        should_have!(&parts[1], IExpressionPE::ConstantInt(ConstantIntPE { value, .. }) => {
            assert_eq!(*value, 4);
        });
        should_have!(&parts[2], IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
            assert_eq!(value, "rg");
        });
    });
}

/// Test: Long string interpolating with call
/// Mirrors StringParserTests.scala line 95
#[test]
fn test_long_string_interpolating_with_call() {
    let expr = compile_expression_expect(r#""""bl"{ns(4)}rg""""#);
    should_have!(expr, IExpressionPE::StrInterpolate(StrInterpolatePE { parts, .. }) => {
        assert_eq!(parts.len(), 3);
        should_have!(&parts[0], IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
            assert_eq!(value, "bl\"");
        });
        should_have!(&parts[1], IExpressionPE::FunctionCall(FunctionCallPE { callable_expr, arg_exprs, .. }) => {
            should_have!(callable_expr.as_ref(), IExpressionPE::Lookup(LookupPE { name: IImpreciseNameP::LookupName(name_p), .. }) => {
                assert_eq!(name_p.str.str, "ns");
            });
            assert_eq!(arg_exprs.len(), 1);
            should_have!(&arg_exprs[0], IExpressionPE::ConstantInt(ConstantIntPE { value, .. }) => {
                assert_eq!(*value, 4);
            });
        });
        should_have!(&parts[2], IExpressionPE::ConstantStr(ConstantStrPE { value, .. }) => {
            assert_eq!(value, "rg");
        });
    });
}

/*
package dev.vale.parsing

import dev.vale.lexing.{Lexer, LexingIterator}
import dev.vale.parsing.ast._
import dev.vale.{Collector, Interner, Keywords, StrI, vimpl}
import dev.vale.parsing.ast.{ConstantIntPE, ConstantStrPE, FunctionCallPE, LookupNameP, LookupPE, NameP, StrInterpolatePE}
import org.scalatest.FunSuite
import org.scalatest.Matchers.convertToAnyShouldWrapper
import org.scalatest._
import org.scalatest._

class StringParserTests extends FunSuite with Collector with TestParseUtils {
  test("Simple string") {
    compileExpressionExpect(""""moo"""") shouldHave
      { case ConstantStrPE(_, "moo") => }
  }

  test("String with newline") {
    compileExpressionExpect("\"\"\"m\noo\"\"\"") shouldHave
      { case ConstantStrPE(_, "m\noo") => }
  }

  test("String with escaped braces") {
    compileExpressionExpect("\"\\{\\}\"") shouldHave
      { case ConstantStrPE(_, "{}") => }
  }

  test("String with quote inside") {
    compileExpressionExpect(""""m\"oo"""") shouldHave
      { case ConstantStrPE(_, "m\"oo") => }
  }

  test("String with unicode") {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val lexer = new Lexer(interner, keywords)
    lexer.parseFourDigitHexNum(new LexingIterator("000a", 0)) shouldEqual Some(10)

    compileExpressionExpect("\"\\u000a\"") match { case ConstantStrPE(_, "\n") => }
    compileExpressionExpect("\"\\u001b\"") match { case ConstantStrPE(_, "\u001b") => }
    compileExpressionExpect("\"foo\\u001bbar\"") match { case ConstantStrPE(_, "foo\u001bbar") => }
    // FALL NOT TO TEMPTATION
    // Scala has some issues here.
    // The above "\"\\u001b\"" seems like it could be expressed """"\\u001b"""" but it can't.
    // Nothing seems to work:
    // - vassert("\"\\u001b\"" == """"\u001b"""") fails
    // - vassert("\"\\u001b\"" == """"\\u001b"""") fails
    // - vassert("\"\\u001b\"" == """\"\\u001b\"""") fails
    // This took quite a while to figure out.
    // So, just stick with regular scala string literals, scala's good with those.
    // Other tests have this, search TEMPTATION.
    // NOW GO YE AND PROSPER
  }

  test("String with apostrophe inside") {
    compileExpressionExpect(""""m'oo"""") shouldHave
      { case ConstantStrPE(_, "m'oo") => }
    compileExpressionExpect("\"\"\"m\'oo\"\"\"") shouldHave
      { case ConstantStrPE(_, "m'oo") => }
  }

  test("Short string interpolating") {
    compileExpressionExpect(""""bl{4}rg"""") shouldHave
      { case StrInterpolatePE(_, Vector(ConstantStrPE(_, "bl"), ConstantIntPE(_, 4, _), ConstantStrPE(_, "rg"))) => }
  }

  test("Short string interpolating with call") {
    compileExpressionExpect(""""bl{ns(4)}rg"""") shouldHave
      {
      case StrInterpolatePE(_,
        Vector(
          ConstantStrPE(_, "bl"),
          FunctionCallPE(_, _, LookupPE(LookupNameP(NameP(_, StrI("ns"))), _), Vector(ConstantIntPE(_, 4, _))),
          ConstantStrPE(_, "rg"))) =>
    }
  }

  test("Long string interpolating") {
    compileExpressionExpect("\"\"\"bl{4}rg\"\"\"") shouldHave
      { case StrInterpolatePE(_, Vector(ConstantStrPE(_, "bl"), ConstantIntPE(_, 4, _), ConstantStrPE(_, "rg"))) => }
  }

  test("Long string doesnt interpolate with brace then newline") {
    compileExpressionExpect(
      "\"\"\"bl{\n4}rg\"\"\"") shouldHave
      { case ConstantStrPE(_, "bl{\n4}rg") => }
  }

  test("Long string interpolates with brace then backslash") {
    compileExpressionExpect(
      "\"\"\"bl{\\\n4}rg\"\"\"") shouldHave
      { case StrInterpolatePE(_, Vector(ConstantStrPE(_, "bl"), ConstantIntPE(_, 4, _), ConstantStrPE(_, "rg"))) => }
  }

  test("Long string interpolating with call") {
    compileExpressionExpect("\"\"\"bl\"{ns(4)}rg\"\"\"") shouldHave
      {
      case StrInterpolatePE(_,
      Vector(
        ConstantStrPE(_, "bl\""),
        FunctionCallPE(_, _, LookupPE(LookupNameP(NameP(_, StrI("ns"))), _), Vector(ConstantIntPE(_, 4, _))),
        ConstantStrPE(_, "rg"))) =>
    }
  }
}

*/