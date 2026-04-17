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
use crate::interner::{StrI, Interner};
use crate::keywords::Keywords;

// mig: struct RSAMutablePopMacro
pub struct RSAMutablePopMacro<'s, 'ctx, 't> {
    pub interner: &'ctx Interner<'s>,
    pub keywords: &'ctx Keywords<'s>,
    pub generator_id: StrI<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
// mig: impl RSAMutablePopMacro
impl<'s, 'ctx, 't> RSAMutablePopMacro<'s, 'ctx, 't> {}
/*
class RSAMutablePopMacro(interner: Interner, keywords: Keywords) extends IFunctionBodyMacro {
  val generatorId: StrI = keywords.vale_runtime_sized_array_pop
*/
// mig: fn generate_function_body
pub fn generate_function_body() {
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

    val body =
      BlockTE(
        ReturnTE(
          PopRuntimeSizedArrayTE(
            ArgLookupTE(0, paramCoords(0).tyype))))
    (header, body)
  }

}
*/
