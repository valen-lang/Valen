use bumpalo::Bump;
use crate::postparsing::ast::FunctionS;
use crate::postparsing::rules::types::ITypeST;
use crate::StrI;
use crate::typing::ast::ast::FunctionDefinitionT;
use crate::typing::borrow_checker::access_event::AccessEventG;
use crate::typing::borrow_checker::ast_g::ExpressionGE;
use crate::typing::borrow_checker::kind_g::KindGT;
use crate::typing::borrow_checker::templata_g::ITemplataG;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::templata::templata::ITemplataT;
use crate::typing::types::types::KindT;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
  pub fn groupify_function<'g>(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    function_s: &'s FunctionS<'s>,
    function_t: &'t FunctionDefinitionT<'s, 't>,
    arena: &'g Bump,
  ) -> Result<(ExpressionGE<'s, 't, 'g>, Vec<&'g AccessEventG<'s, 't>>), ICompileErrorT<'s, 't>> {
    unimplemented!()
  }

  fn make_kind_g<'g>(
    &self,
    kind: KindT<'s, 't>,
    tyype: &'s ITypeST<'s>,
    param_name: Option<StrI<'s>>,
  ) -> KindGT<'s, 't, 'g> {
    unimplemented!()
  }

  fn make_templata_g(
    &self,
    templata: ITemplataT<'s, 't>,
    written: Option<&'s ITypeST<'s>>,
    param_name: Option<StrI<'s>>,
  ) -> ITemplataG<'s, 't> {
    unimplemented!()
  }
}