/// While loop parsing tests
/// Mirrors Frontend/ParsingPass/test/dev/vale/parsing/WhileTests.scala

use crate::parsing::tests::test_parse_utils::*;
use crate::parsing::ast::*;
use crate::{should_have};

// Mirrors WhileTests.scala line 11
#[test]
fn test_simple_while_loop() {
    let result = compile_block_contents_expect("while true {}");
    
    should_have!(result, IExpressionPE::While(WhilePE {
        condition: box IExpressionPE::ConstantBool(ConstantBoolPE { value: true, .. }),
        body: box BlockPE {
            inner: box IExpressionPE::Void(_),
            ..
        },
        ..
    }) => {});
}

// Mirrors WhileTests.scala line 17
#[test]
fn test_result_after_while_loop() {
    let result = compile_block_contents_expect("while true {} false");
    
    // The parser returns a Consecutor containing the while loop and the result expression
    should_have!(result, IExpressionPE::Consecutor(ConsecutorPE {
        inners: ref inners,
    }) if inners.len() == 2 && matches!(&inners[0], IExpressionPE::While(WhilePE {
        condition: box IExpressionPE::ConstantBool(ConstantBoolPE { value: true, .. }),
        body: box BlockPE {
            inner: box IExpressionPE::Void(_),
            ..
        },
        ..
    })) => {});
}

// Mirrors WhileTests.scala line 23
#[test]
fn test_while_with_condition_declarations() {
    let result = compile_block_contents_expect("while x = 4; x > 6 { }");
    
    should_have!(result, IExpressionPE::While(WhilePE {
        condition: box IExpressionPE::Consecutor(ConsecutorPE {
            inners: ref inners,
        }),
        body: box BlockPE {
            inner: box IExpressionPE::Void(_),
            ..
        },
        ..
    }) if inners.len() == 2 => {});
}

