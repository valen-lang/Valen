use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::higher_typing::ast::*;

use crate::typing::types::types::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::compiler_outputs::*;
use crate::typing::compiler::Compiler;
use crate::postparsing::ast::LocationInDenizen;
use crate::typing::compiler_error_reporter::ICompileErrorT;

/*
package dev.vale.typing.macros.rsa

import dev.vale.{Keywords, RangeS, StrI, vimpl}

import dev.vale.highertyping.FunctionA
import dev.vale.postparsing.LocationInDenizen
import dev.vale.typing.ast._
import dev.vale.typing.env._
import dev.vale.typing.{ArrayCompiler, CompilerOutputs}

import dev.vale.typing.macros.IFunctionBodyMacro
import dev.vale.typing.types._
import dev.vale.typing.ast._
import dev.vale.typing.env.FunctionEnvironmentBoxT
import dev.vale.typing.ast

*/
// (Scala `class RSADropIntoMacro(keywords, arrayCompiler)` absorbed onto `Compiler`;
//  the method body lives at `Compiler::generate_function_body_rsa_drop_into` below.)
/*
class RSADropIntoMacro(keywords: Keywords, arrayCompiler: ArrayCompiler) extends IFunctionBodyMacro {
  val generatorId: StrI = keywords.vale_runtime_sized_array_drop_into

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn generate_function_body_rsa_drop_into(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &FunctionEnvironmentT<'s, 't>,
        generator_id: StrI<'s>,
        life: LocationInFunctionEnvironmentT<'t>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        origin_function: Option<&FunctionA<'s>>,
        param_coords: &[ParameterT<'s, 't>],
        maybe_ret_coord: Option<CoordT<'s, 't>>,
    ) -> Result<(FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>), ICompileErrorT<'s, 't>> {
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
    val fate = FunctionEnvironmentBoxT(env)
    val body =
      BlockTE(
        ReturnTE(
          arrayCompiler.evaluateDestroyRuntimeSizedArrayIntoCallable(
            coutputs,
            fate,
            callRange,
            callLocation,
            ArgLookupTE(0, paramCoords(0).tyype),
            ArgLookupTE(1, paramCoords(1).tyype),
            RegionT(DefaultRegionT))))
    (header, body)
  }

}
*/
}
