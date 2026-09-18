use crate::postparsing::ast::FunctionS;
use crate::typing::ast::ast::{FunctionAliasingInfoT, FunctionDefinitionT};
use crate::typing::borrow_checker::access_event::AccessEventG;
use crate::typing::borrow_checker::ast_g::ExpressionGE;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::CompilerOutputs;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
  pub fn calculate_aliasing_info<'g>(
    &self,
    function_s: &'s FunctionS<'s>,
    function_t: &'t FunctionDefinitionT<'s, 't>,
    body: ExpressionGE<'s, 't, 'g>,
    access_log: &[&'g AccessEventG<'s, 't>],
  ) -> FunctionAliasingInfoT {
    unimplemented!()
  }
}