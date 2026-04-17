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
use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::utils::range::RangeS;

use crate::higher_typing::ast::*;

use crate::typing::types::types::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::compiler_outputs::*;
use crate::interner::Interner;
use crate::postparsing::ast::LocationInDenizen;

// mig: struct RSAMutableCapacityMacro
pub struct RSAMutableCapacityMacro<'s, 'ctx, 't> {
  pub interner: Interner<'s>,
  pub keywords: Keywords<'s>,
  pub generator_id: StrI<'s>,
}
// mig: impl RSAMutableCapacityMacro
impl<'s, 'ctx, 't> RSAMutableCapacityMacro<'s, 'ctx, 't> {}
/*
class RSAMutableCapacityMacro(interner: Interner, keywords: Keywords) extends IFunctionBodyMacro {
  val generatorId: StrI = keywords.vale_runtime_sized_array_capacity
*/
// mig: fn generate_function_body
pub fn generate_function_body<'s, 't>(
    env: FunctionEnvironmentT<'s, 't>,
    coutputs: CompilerOutputs<'s, 't>,
    generator_id: StrI<'s>,
    life: LocationInFunctionEnvironmentT<'s>,
    call_range: Vec<RangeS<'s>>,
    call_location: LocationInDenizen<'s>,
    origin_function: Option<FunctionA<'s>>,
    param_coords: Vec<ParameterT<'s, 't>>,
    maybe_ret_coord: Option<CoordT<'s, 't>>,
) -> (FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>) {
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
