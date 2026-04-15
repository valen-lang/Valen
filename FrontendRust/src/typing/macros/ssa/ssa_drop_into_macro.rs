/*
package dev.vale.typing.macros.ssa

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
// mig: struct SSADropIntoMacro
pub struct SSADropIntoMacro<'p, 's> {
    pub generator_id: StrI<'p>,
    pub keywords: &'p Keywords<'p>,
    pub array_compiler: &'s (), // placeholder for ArrayCompiler
}

// mig: impl SSADropIntoMacro
impl<'p, 's> SSADropIntoMacro<'p, 's> {
}
/*
class SSADropIntoMacro(keywords: Keywords, arrayCompiler: ArrayCompiler) extends IFunctionBodyMacro {
  val generatorId: StrI = keywords.vale_static_sized_array_drop_into
*/
// mig: fn generate_function_body
fn generate_function_body(
    env: &(),
    coutputs: &(),
    generator_id: StrI,
    life: &(),
    call_range: &[RangeS],
    call_location: &(),
    origin_function: Option<&()>,
    param_coords: &[()],
    maybe_ret_coord: Option<&()>,
) -> ((), ()) {
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
    coutputs.declareFunctionReturnType(header.toSignature, header.returnType)
    val fate = FunctionEnvironmentBoxT(env)
    val body =
      BlockTE(
        ReturnTE(
          arrayCompiler.evaluateDestroyStaticSizedArrayIntoCallable(
            coutputs,
            fate,
            callRange,
            callLocation,
            ArgLookupTE(0, paramCoords(0).tyype),
            ArgLookupTE(1, paramCoords(1).tyype),
            RegionT())))
    (header, body)
  }
}
*/
