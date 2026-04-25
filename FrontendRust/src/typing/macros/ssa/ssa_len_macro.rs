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

/*
package dev.vale.typing.macros.ssa

import dev.vale._
import dev.vale.highertyping.FunctionA
import dev.vale.postparsing.LocationInDenizen
import dev.vale.typing.{CompileErrorExceptionT, CompilerOutputs, RangedInternalErrorT}

import dev.vale.typing.ast.{ArgLookupTE, BlockTE, ConsecutorTE, ConstantIntTE, DiscardTE, FunctionDefinitionT, FunctionHeaderT, LocationInFunctionEnvironmentT, ParameterT, ReturnTE}

import dev.vale.typing.env.FunctionEnvironmentT
import dev.vale.typing.macros.IFunctionBodyMacro
import dev.vale.typing.types._
import dev.vale.typing.ast._
import dev.vale.typing.types.StaticSizedArrayTT
import dev.vale.typing.ast

*/
// (Scala `class SSALenMacro(keywords)` absorbed onto `Compiler`; the method
//  body lives at `Compiler::generate_function_body_ssa_len` below.)
/*
class SSALenMacro(keywords: Keywords) extends IFunctionBodyMacro {
  val generatorId: StrI = keywords.vale_static_sized_array_len

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn generate_function_body_ssa_len(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &FunctionEnvironmentT<'s, 't>,
        generator_id: StrI<'s>,
        life: LocationInFunctionEnvironmentT<'s>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        origin_function: Option<&FunctionA<'s>>,
        param_coords: &[ParameterT<'s, 't>],
        maybe_ret_coord: Option<CoordT<'s, 't>>,
    ) -> (FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>) {
        panic!("Unimplemented: generate_function_body_ssa_len");
    } // VI: invalid

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
    coutputs.declareFunctionReturnType(header.toSignature, header.returnType)
    val len =
      header.paramTypes match {
        case Vector(CoordT(_, _, contentsStaticSizedArrayTT(size, _, _, _, _))) => size
        case _ => throw CompileErrorExceptionT(RangedInternalErrorT(callRange, "SSALenMacro received non-SSA param: " + header.paramTypes))
      }

    val body =
      BlockTE(
        ConsecutorTE(
          Vector(
            DiscardTE(ArgLookupTE(0, paramCoords(0).tyype)),
            ReturnTE(
              ConstantIntTE(len, 32, RegionT())))))
    (header, body)
  }

}
*/
}
