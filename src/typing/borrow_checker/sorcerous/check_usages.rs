use crate::typing::borrow_checker::ast_g::ExpressionGE;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::CompilerOutputs;
use bumpalo::Bump;
use crate::postparsing::ast::FunctionS;
use crate::postparsing::rules::types::ITypeST;
use crate::StrI;
use crate::typing::ast::ast::{FunctionAliasingInfoT, FunctionDefinitionT};
use crate::typing::borrow_checker::kind_g::KindGT;
use crate::typing::borrow_checker::templata_g::ITemplataG;
use crate::typing::templata::templata::ITemplataT;
use crate::typing::types::types::KindT;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
  pub fn check_usages<'g>(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    function_g_body: ExpressionGE<'s, 't, 'g>,
  ) -> Result<(), ICompileErrorT<'s, 't>> {
    unimplemented!()
  }
}