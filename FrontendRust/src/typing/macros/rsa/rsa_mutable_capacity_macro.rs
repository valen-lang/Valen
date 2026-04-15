/*
package dev.vale.typing.macros.rsa

import dev.vale.highertyping.FunctionA
import dev.vale.postparsing._
import dev.vale.typing.CompilerOutputs
import dev.vale.typing.ast._
import dev.vale.typing.env.{FunctionEnvironmentT, TemplataLookupContext}

import dev.vale.typing.macros.IFunctionBodyMacro
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.{Interner, Keywords, Profiler, RangeS, StrI, vassertSome, vimpl}

import dev.vale.postparsing.CodeRuneS
import dev.vale.typing.ast._
import dev.vale.typing.env.TemplataLookupContext
import dev.vale.typing.types._
import dev.vale.typing.ast

*/
// mig: struct RSAMutableCapacityMacro
pub struct RSAMutableCapacityMacro {
  pub interner: Interner,
  pub keywords: Keywords,
  pub generator_id: StrI,
}
// mig: impl RSAMutableCapacityMacro
impl RSAMutableCapacityMacro {}
/*
class RSAMutableCapacityMacro(interner: Interner, keywords: Keywords) extends IFunctionBodyMacro {
  val generatorId: StrI = keywords.vale_runtime_sized_array_capacity
*/
// mig: fn generate_function_body
pub fn generate_function_body(
    env: FunctionEnvironmentT,
    coutputs: CompilerOutputs,
    generator_id: StrI,
    life: LocationInFunctionEnvironmentT,
    call_range: Vec<RangeS>,
    call_location: LocationInDenizen,
    origin_function: Option<FunctionA>,
    param_coords: Vec<ParameterT>,
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
      FunctionHeaderT(
        env.id, Vector.empty, paramCoords, maybeRetCoord.get, Some(env.templata))

//    val CoordTemplata(elementType) =
//      vassertSome(
//        env.lookupNearestWithImpreciseName(
//          interner.intern(RuneNameS(CodeRuneS(keywords.E))), Set(TemplataLookupContext)))

    val body =
      BlockTE(
        ReturnTE(
          RuntimeSizedArrayCapacityTE(
            ArgLookupTE(0, paramCoords(0).tyype))))
    (header, body)
  }

}
*/
