/*
package dev.vale.typing.macros.rsa

import dev.vale.highertyping.FunctionA
import dev.vale.postparsing._
import dev.vale.typing.{ArrayCompiler, CompilerOutputs, ast}

import dev.vale.typing.ast._
import dev.vale.typing.env.{FunctionEnvironmentT, TemplataLookupContext}

import dev.vale.typing.macros.IFunctionBodyMacro
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.{Interner, Keywords, Profiler, RangeS, StrI, vassert, vassertSome, vimpl}

import dev.vale.postparsing.CodeRuneS
import dev.vale.typing.ast._
import dev.vale.typing.env.TemplataLookupContext
import dev.vale.typing.function.DestructorCompiler
import dev.vale.typing.templata.MutabilityTemplataT
import dev.vale.typing.types.RuntimeSizedArrayTT

*/
// mig: struct RSAMutableNewMacro
pub struct RSAMutableNewMacro<'p, 's> {
    pub interner: &'p Interner<'p>,
    pub keywords: &'p Keywords<'p>,
    pub array_compiler: &'s ArrayCompiler<'p, 's>,
    pub destructor_compiler: &'s DestructorCompiler<'p, 's>,
}
// mig: impl RSAMutableNewMacro
impl<'p, 's> RSAMutableNewMacro<'p, 's> {
}
/*
class RSAMutableNewMacro(
  interner: Interner,
  keywords: Keywords,
  arrayCompiler: ArrayCompiler,
  destructorCompiler: DestructorCompiler
) extends IFunctionBodyMacro {
  val generatorId: StrI = keywords.vale_runtime_sized_array_mut_new

*/
// mig: fn generate_function_body
pub fn generate_function_body(
    &self,
    env: &FunctionEnvironmentT<'p, 's>,
    coutputs: &mut CompilerOutputs<'p, 's>,
    generator_id: StrI<'p>,
    life: LocationInFunctionEnvironmentT<'p, 's>,
    call_range: &[RangeS<'p>],
    call_location: LocationInDenizen<'p>,
    origin_function: Option<&FunctionA<'p>>,
    param_coords: &[ParameterT<'p, 's>],
    maybe_ret_coord: Option<CoordT<'p, 's>>,
) -> (FunctionHeaderT<'p, 's>, ReferenceExpressionTE<'p, 's>) {
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
    coutputs.declareFunctionReturnType(header.toSignature, header.returnType)

    val CoordTemplataT(elementType) =
      vassertSome(
        env.lookupNearestWithImpreciseName(
          interner.intern(RuneNameS(CodeRuneS(keywords.E))), Set(TemplataLookupContext)))

    val mutability =
      ITemplataT.expectMutability(
        vassertSome(
          env.lookupNearestWithImpreciseName(
            interner.intern(RuneNameS(CodeRuneS(keywords.M))), Set(TemplataLookupContext))))

    val arrayTT = arrayCompiler.resolveRuntimeSizedArray(elementType, mutability, RegionT())

    val body =
      BlockTE(
        ReturnTE(
          NewMutRuntimeSizedArrayTE(
            arrayTT,
            RegionT(),
            ArgLookupTE(0, paramCoords(0).tyype))))
//            freePrototype)))
    (header, body)
  }

}
*/
