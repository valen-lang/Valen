/*
package dev.vale.typing.macros

import dev.vale.{Keywords, RangeS, StrI, vimpl}
import dev.vale.typing.CompilerOutputs
import dev.vale.typing.ast.{ArgLookupTE, BlockTE, FunctionDefinitionT, FunctionHeaderT, IsSameInstanceTE, LocationInFunctionEnvironmentT, ParameterT, ReturnTE}
import dev.vale.typing.citizen.StructCompiler
import dev.vale.typing.env.FunctionEnvironmentT
import dev.vale.typing.types.CoordT
import dev.vale.highertyping.FunctionA
import dev.vale.postparsing.LocationInDenizen
import dev.vale.typing.ast
import dev.vale.typing.ast._
import dev.vale.typing.function.FunctionCompilerCore
*/
// mig: struct SameInstanceMacro
pub struct SameInstanceMacro {
    pub generator_id: StrI,
}
// mig: impl SameInstanceMacro
impl SameInstanceMacro {}
/*
class SameInstanceMacro(keywords: Keywords) extends IFunctionBodyMacro {
  val generatorId: StrI = keywords.vale_same_instance
*/
// mig: fn generate_function_body
fn generate_function_body(
    env: &FunctionEnvironmentT,
    coutputs: &CompilerOutputs,
    generator_id: StrI,
    life: LocationInFunctionEnvironmentT,
    call_range: &[RangeS],
    call_location: LocationInDenizen,
    origin_function: Option<&FunctionA>,
    param_coords: &[ParameterT],
    maybe_ret_coord: Option<CoordT>,
) -> (FunctionHeaderT, ReferenceExpressionTE) {
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
          IsSameInstanceTE(
            ArgLookupTE(0, paramCoords(0).tyype), ArgLookupTE(1, paramCoords(1).tyype))))
    (header, body)
  }
}
*/
