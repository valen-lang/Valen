/*
package dev.vale.typing.macros

import dev.vale.{RangeS, StrI, vimpl}

import dev.vale.typing.CompilerOutputs
import dev.vale.typing.ast.{ArgLookupTE, ArrayLengthTE, BlockTE, FunctionDefinitionT, FunctionHeaderT, LocationInFunctionEnvironmentT, ParameterT, ReturnTE}

import dev.vale.typing.env.FunctionEnvironmentT
import dev.vale.typing.types.CoordT
import dev.vale.typing.ast._
import dev.vale.typing.env.FunctionEnvironmentBoxT
import dev.vale.typing.ast
import dev.vale.highertyping.FunctionA
import dev.vale.postparsing.LocationInDenizen
*/
use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::higher_typing::ast::*;

use crate::typing::types::types::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::compiler_outputs::*;
use crate::postparsing::ast::LocationInDenizen;

// mig: struct RSALenMacro
pub struct RSALenMacro<'s, 'ctx, 't>(pub std::marker::PhantomData<(&'s (), &'ctx (), &'t ())>);
// TODO: placeholder PhantomData — replace with real fields during body migration

// mig: impl RSALenMacro
impl<'s, 'ctx, 't> RSALenMacro<'s, 'ctx, 't> {}
/*
class RSALenMacro() extends IFunctionBodyMacro {
  val generatorId: String = "vale_runtime_sized_array_len"
*/
// mig: fn generate_function_body
pub fn generate_function_body<'s, 't>(env: FunctionEnvironmentT<'s, 't>, coutputs: CompilerOutputs<'s, 't>, generator_id: StrI<'s>, life: LocationInFunctionEnvironmentT<'s>, call_range: Vec<RangeS<'s>>, call_location: LocationInDenizen<'s>, origin_function: Option<FunctionA<'s>>, param_coords: Vec<ParameterT<'s, 't>>, maybe_ret_coord: Option<CoordT<'s, 't>>) -> (FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>) {
    panic!("Unimplemented: generate_function_body");
}

/*
  def generateFunctionBody(
    env: FunctionEnvironmentT,
    coutputs: CompilerOutputs,
    generatorId: StrI,
    life: LocationInFunctionEnvironmentT,
    callRange: List[RangeS],
      callLocation: LocationInDenizen,
    originFunction: Option[FunctionA],
    paramCoords: Vector[ParameterT],
    maybeRetCoord: Option[CoordT]):
  (FunctionHeaderT, ReferenceExpressionTE) = {
    val header =
      FunctionHeaderT(env.id, Vector.empty, paramCoords, maybeRetCoord.get, Some(env.templata))
    val body =
      BlockTE(
        ReturnTE(
          ArrayLengthTE(
            ArgLookupTE(0, paramCoords(0).tyype))))
    (header, body)
  }

}
*/
