/// If statement parsing tests
/// Mirrors Frontend/ParsingPass/test/dev/vale/parsing/IfTests.scala

use crate::tests::test_parse_utils::*;
use crate::parsing::ast::*;
use crate::{should_have};

// Mirrors IfTests.scala line 11
#[test]
fn test_ifs() {
    let result = compile_expression_expect("if true { doBlarks(&x) } else { }");
    
    should_have!(result, IExpressionPE::If(IfPE {
        condition: box IExpressionPE::ConstantBool(ConstantBoolPE { value: true, .. }),
        then_body: box BlockPE {
            inner: box IExpressionPE::FunctionCall(FunctionCallPE {
                callable_expr: box IExpressionPE::Lookup(LookupPE {
                    name: IImpreciseNameP::LookupName(NameP { str, .. }),
                    ..
                }),
                arg_exprs: ref args,
                ..
            }),
            ..
        },
        else_body: box BlockPE {
            inner: box IExpressionPE::Void(_),
            ..
        },
        ..
    }) if str.str == "doBlarks" && args.len() == 1 => {});
}

// Mirrors IfTests.scala line 24
#[test]
fn test_if_let() {
    let result = compile_expression_expect("if [u] = a {}");
    
    should_have!(result, IExpressionPE::If(IfPE {
        condition: box IExpressionPE::Let(LetPE {
            pattern: PatternPP {
                destination: None,
                templex: None,
                destructure: Some(DestructureP {
                    patterns: ref patterns,
                    ..
                }),
                ..
            },
            source: box IExpressionPE::Lookup(LookupPE {
                name: IImpreciseNameP::LookupName(NameP { str, .. }),
                ..
            }),
            ..
        }),
        then_body: box BlockPE {
            inner: box IExpressionPE::Void(_),
            ..
        },
        else_body: box BlockPE {
            inner: box IExpressionPE::Void(_),
            ..
        },
        ..
    }) if str.str == "a" && patterns.len() == 1 => {});
}

// Mirrors IfTests.scala line 39
#[test]
fn test_if_with_condition_declarations() {
    let result = compile_expression_expect("if x = 4; not x.isEmpty() { }");
    
    should_have!(result, IExpressionPE::If(IfPE {
        condition: box IExpressionPE::Consecutor(ConsecutorPE {
            inners: ref inners,
        }),
        then_body: box BlockPE {
            inner: box IExpressionPE::Void(_),
            ..
        },
        else_body: box BlockPE {
            inner: box IExpressionPE::Void(_),
            ..
        },
        ..
    }) if inners.len() == 2 => {});
}

// Mirrors IfTests.scala line 51
#[test]
fn test_19() {
    let result = compile_block_contents_expect("newLen = if num == 0 { 1 } else { 2 };");
    
    should_have!(result, IExpressionPE::Consecutor(ConsecutorPE {
        inners: ref inners,
    }) if inners.len() == 2 && matches!(inners[0], IExpressionPE::Let(LetPE {
        source: box IExpressionPE::If(_),
        ..
    })) => {});
}

