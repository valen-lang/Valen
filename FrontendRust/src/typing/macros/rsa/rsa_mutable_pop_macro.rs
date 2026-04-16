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
use std::collections::{HashMap, HashSet};

use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::utils::range::RangeS;

use crate::postparsing::names::*;
use crate::higher_typing::ast::*;

use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::compiler_outputs::*;
use crate::typing::macros::macros::*;
use crate::interner::Interner;

// mig: struct RSAMutablePopMacro
pub struct RSAMutablePopMacro<'s, 'ctx, 't> {
    pub interner: &'ctx Interner<'s>,
    pub keywords: &'ctx Keywords<'s>,
    pub generator_id: StrI<'s>,
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
