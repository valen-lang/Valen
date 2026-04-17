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
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::utils::range::RangeS;

use crate::higher_typing::ast::*;

use crate::typing::types::types::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::compiler_outputs::*;
use crate::typing::array_compiler::*;
use crate::interner::Interner;
use crate::typing::function::destructor_compiler::*;
use crate::postparsing::ast::LocationInDenizen;

// mig: struct RSAMutableNewMacro
pub struct RSAMutableNewMacro<'s, 'ctx, 't> {
    pub interner: &'ctx Interner<'s>,
    pub keywords: &'ctx Keywords<'s>,
    pub array_compiler: &'ctx ArrayCompiler<'s, 'ctx, 't>,
    pub destructor_compiler: &'ctx DestructorCompiler<'s, 'ctx, 't>,
}
// mig: impl RSAMutableNewMacro
impl<'s, 'ctx, 't> RSAMutableNewMacro<'s, 'ctx, 't> {
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
impl<'s, 'ctx, 't> RSAMutableNewMacro<'s, 'ctx, 't> {
pub fn generate_function_body(
    &self,
    env: &FunctionEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    generator_id: StrI<'s>,
    life: LocationInFunctionEnvironmentT<'s>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    origin_function: Option<&FunctionA<'s>>,
    param_coords: &[ParameterT<'s, 't>],
    maybe_ret_coord: Option<CoordT<'s, 't>>,
) -> (FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>) {
    panic!("Unimplemented: generate_function_body");
}
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
