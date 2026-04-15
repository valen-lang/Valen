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
// mig: struct RSALenMacro
pub struct RSALenMacro;

// mig: impl RSALenMacro
impl RSALenMacro {}
/*
class RSALenMacro() extends IFunctionBodyMacro {
  val generatorId: String = "vale_runtime_sized_array_len"
*/
// mig: fn generate_function_body
pub fn generate_function_body(env: FunctionEnvironmentT, coutputs: CompilerOutputs, generatorId: StrI, life: LocationInFunctionEnvironmentT, callRange: Vec<RangeS>, callLocation: LocationInDenizen, originFunction: Option<FunctionA>, paramCoords: Vec<ParameterT>, maybeRetCoord: Option<CoordT>) -> (FunctionHeaderT, ReferenceExpressionTE) {
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
