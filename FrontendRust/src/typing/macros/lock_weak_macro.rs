/*
package dev.vale.typing.macros

import dev.vale.{Keywords, RangeS, StrI, vimpl}

import dev.vale.highertyping.FunctionA
import dev.vale.postparsing.LocationInDenizen
import dev.vale.typing.CompilerOutputs
import dev.vale.typing.ast.{ArgLookupTE, BlockTE, FunctionDefinitionT, FunctionHeaderT, LocationInFunctionEnvironmentT, LockWeakTE, ParameterT, ReturnTE}

import dev.vale.typing.env.FunctionEnvironmentT
import dev.vale.typing.expression.ExpressionCompiler
import dev.vale.typing.types._
import dev.vale.typing.ast._
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
use crate::typing::expression::expression_compiler::*;
use crate::postparsing::ast::LocationInDenizen;

// mig: struct LockWeakMacro
pub struct LockWeakMacro<'s, 'ctx, 't> {
    pub keywords: Keywords<'s>,
    pub expression_compiler: ExpressionCompiler<'s, 'ctx, 't>,
}
// mig: impl LockWeakMacro
impl<'s, 'ctx, 't> LockWeakMacro<'s, 'ctx, 't> {}
/*
class LockWeakMacro(
  keywords: Keywords,
  expressionCompiler: ExpressionCompiler
) extends IFunctionBodyMacro {
  val generatorId: StrI = keywords.vale_lock_weak

*/
// mig: fn generate_function_body
fn generate_function_body(
    env: &FunctionEnvironmentT,
    coutputs: &CompilerOutputs,
    generator_id: StrI,
    life: LocationInFunctionEnvironmentT,
    call_range: Vec<RangeS>,
    call_location: LocationInDenizen,
    origin_function: Option<&FunctionA>,
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

    val borrowCoord = paramCoords.head.tyype.copy(ownership = BorrowT)
    val (optCoord, someConstructor, noneConstructor, someImplId, noneImplId) =
      expressionCompiler.getOption(coutputs, env, callRange, callLocation, RegionT(), borrowCoord)
    val lockExpr =
      LockWeakTE(
        ArgLookupTE(0, paramCoords.head.tyype),
        optCoord,
        someConstructor,
        noneConstructor,
        someImplId,
        noneImplId)

    val body = BlockTE(ReturnTE(lockExpr))

    (header, body)
  }

}
*/
