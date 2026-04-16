/*
package dev.vale.typing.macros.rsa

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
use crate::typing::array_compiler::*;
use crate::postparsing::ast::LocationInDenizen;

// mig: struct RSADropIntoMacro
pub struct RSADropIntoMacro<'s, 'ctx, 't> {
    pub keywords: Keywords<'s>,
    pub array_compiler: ArrayCompiler<'s, 'ctx, 't>,
}

// mig: impl RSADropIntoMacro
impl<'s, 'ctx, 't> RSADropIntoMacro<'s, 'ctx, 't> {}
/*
class RSADropIntoMacro(keywords: Keywords, arrayCompiler: ArrayCompiler) extends IFunctionBodyMacro {
  val generatorId: StrI = keywords.vale_runtime_sized_array_drop_into

*/
// mig: fn generate_function_body
fn generate_function_body(
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
      FunctionHeaderT(env.id, Vector.empty, paramCoords, maybeRetCoord.get, Some(env.templata))
    val fate = FunctionEnvironmentBoxT(env)
    val body =
      BlockTE(
        ReturnTE(
          arrayCompiler.evaluateDestroyRuntimeSizedArrayIntoCallable(
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
