use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::postparsing::ast::*;

use crate::postparsing::ast::LocationInDenizen;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::types::types::*;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn generate_function_body_rsa_len(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    env: &'t FunctionEnvironmentT<'s, 't>,
    generator_id: StrI<'s>,
    loct: LocT<'t>,
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
    // This is a compiler-generated builtin body, so its nodes have no user source; the honest range is a synthesized internal one.
    let synth_range = RangeS::internal(self.scout_arena, -70040);
    let body = ExpressionTE::Block(self.typing_interner.alloc(BlockTE::new(synth_range, ExpressionTE::Return(
      self.typing_interner.alloc(ReturnTE::new(synth_range, ExpressionTE::ArrayLength(
        self.typing_interner.alloc(ArrayLengthTE::new(synth_range, ExpressionTE::ArgLookup(
          self.typing_interner.alloc(ArgLookupTE::new(synth_range, 0, param_coords[0].tyype)),
        ))),
      ))),
    ))));
    (header, body)
  }
}
