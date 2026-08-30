use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::postparsing::ast::*;

use crate::postparsing::ast::LocationInDenizen;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::types::types::*;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn generate_function_body_rsa_drop_into(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    env: &FunctionEnvironmentT<'s, 't>,
    generator_id: StrI<'s>,
    loct: LocT<'t>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    origin_function: Option<&FunctionS<'s>>,
    param_coords: &[ParameterT<'s, 't>],
    maybe_ret_coord: Option<KindT<'s, 't>>,
  ) -> Result<(FunctionHeaderT<'s, 't>, ExpressionTE<'s, 't>), ICompileErrorT<'s, 't>> {
    panic!("Unimplemented: generate_function_body_rsa_drop_into");
    // val header =
    //   FunctionHeaderT(env.id, Vector.empty, paramCoords, maybeRetCoord.get, Some(env.templata))
    // val fate = FunctionEnvironmentBoxT(env)
    // val body =
    //   BlockTE(
    //     ReturnTE(
    //       arrayCompiler.evaluateDestroyRuntimeSizedArrayIntoCallable(
    //         coutputs, fate, callRange, callLocation,
    //         ArgLookupTE(0, paramCoords(0).tyype),
    //         ArgLookupTE(1, paramCoords(1).tyype),
    //         RegionT(DefaultRegionT))))
    // (header, body)
  }
}
