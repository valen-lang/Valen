use bumpalo::Bump;
use crate::postparsing::ast::FunctionS;
use crate::postparsing::rules::types::ITypeST;
use crate::StrI;
use crate::typing::ast::ast::{FunctionAliasingInfoT, FunctionDefinitionT};
use crate::typing::borrow_checker::ast_g::ExpressionGE;
use crate::typing::borrow_checker::kind_g::KindGT;
use crate::typing::borrow_checker::templata_g::ITemplataG;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::templata::templata::ITemplataT;
use crate::typing::types::types::KindT;

mod check_usages;
mod groupify_function;
mod calculate_aliasing_info;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
  pub fn check_function<'g>(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    function_s: &'s FunctionS<'s>,
    function_t: &'t FunctionDefinitionT<'s, 't>,
    check_arena: &'g Bump,
  ) -> Result<FunctionAliasingInfoT, ICompileErrorT<'s, 't>> {
    let (body_g, access_log) =
        self.groupify_function(coutputs, function_s, function_t, check_arena)?;
    self.check_usages(coutputs, body_g)?;
    Ok(self.calculate_aliasing_info(function_s, function_t, body_g, &*access_log))
  }
}
