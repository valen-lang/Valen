use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::postparsing::ast::*;

use crate::postparsing::ast::LocationInDenizen;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::templata::templata::ITemplataT;
use crate::typing::templata_compiler::peel_all_references;
use crate::typing::types::types::*;
use crate::typing::types::types::{KindT, RegionT};

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn generate_function_body_ssa_len(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    env: &'t FunctionEnvironmentT<'s, 't>,
    generator_id: StrI<'s>,
    life: LocationInFunctionEnvironmentT<'t>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    origin_function: Option<&FunctionS<'s>>,
    param_coords: &[ParameterT<'s, 't>],
    maybe_ret_coord: Option<KindT<'s, 't>>,
  ) -> (FunctionHeaderT<'s, 't>, ExpressionTE<'s, 't>) {
    let header = FunctionHeaderT {
      id: env.id,
      attributes: self.typing_interner.alloc_slice_from_vec(vec![]),
      params: self.typing_interner.alloc_slice_from_vec(param_coords.to_vec()),
      return_type: maybe_ret_coord.expect("vassertSome: maybeRetCoord"),
      maybe_origin_function_templata: Some(env.templata()),
    };
    coutputs.declare_function_return_type(
      self.typing_interner.alloc(header.to_signature()),
      header.return_type,
    );
    let len = match peel_all_references(param_coords[0].tyype) {
      KindT::StaticSizedArray(ssa) => ssa.size(),
      other => panic!("SSALenMacro received non-SSA param: {:?}", other),
    };
    let discard_te =
      ExpressionTE::Discard(self.typing_interner.alloc(DiscardTE::new(ExpressionTE::ArgLookup(
        self.typing_interner.alloc(ArgLookupTE::new(0, param_coords[0].tyype)),
      ))));
    let return_te =
      ExpressionTE::Return(self.typing_interner.alloc(ReturnTE::new(ExpressionTE::ConstantInt(
        self.typing_interner.alloc(ConstantIntTE::new(len, 32, RegionT::Default)),
      ))));
    let body = ExpressionTE::Block(self.typing_interner.alloc(BlockTE::new(
      ExpressionTE::Consecutor(self.typing_interner.alloc(ConsecutorTE::new(
        self.typing_interner.alloc_slice_from_vec(vec![discard_te, return_te]),
      ))),
    )));
    (header, body)
  }
}
