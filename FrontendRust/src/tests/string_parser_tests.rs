/// String parsing tests
/// Mirrors Frontend/ParsingPass/src/dev/vale/parsing/StringParserTests.scala lines 1-105

use crate::tests::test_utils::*;
use crate::parsing::ast::*;
use crate::{should_have, matches_pattern};
use crate::interner::{Interner, StrI};
use crate::keywords::Keywords;
use crate::lexing::lexer::Lexer;
use crate::lexing::iterator::LexingIterator;
use std::sync::{Arc, Mutex};

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
    let mut interner = Interner::new();
    let keywords = Keywords::new(&mut interner);
    let interner = Arc::new(Mutex::new(interner));
    let keywords = Arc::new(keywords);
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

