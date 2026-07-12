use crate::postparsing::ast::IBodyS;
use crate::postparsing::expressions::IExpressionSE;


// The head expression of a normal (code) function body. Panics with the actual body if it isn't a
// code body, since a wrong body variant (extern/abstract) means the test's setup is off.
pub fn expect_code_body_expr<'s>(body: &'s IBodyS<'s>) -> &'s IExpressionSE<'s> {
  match body {
    IBodyS::CodeBody(code_body) => code_body.body.block.expr,
    other => panic!("expected a code body, got {:?}", other),
  }
}
