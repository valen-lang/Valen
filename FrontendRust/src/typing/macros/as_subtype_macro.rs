/*
package dev.vale.typing.macros

import dev.vale.{Keywords, RangeS, StrI, vassertSome, vfail, vimpl, vwat}

import dev.vale.highertyping.FunctionA
import dev.vale.postparsing.LocationInDenizen
import dev.vale.typing.{CantDowncastToInterface, CantDowncastUnrelatedTypes, CompileErrorExceptionT, CompilerOutputs, RangedInternalErrorT}

import dev.vale.typing.ast.{ArgLookupTE, AsSubtypeTE, BlockTE, FunctionCallTE, FunctionDefinitionT, FunctionHeaderT, LocationInFunctionEnvironmentT, ParameterT, ReferenceExpressionTE, ReturnTE}

import dev.vale.typing.citizen.{ImplCompiler, IsParent, IsntParent}

import dev.vale.typing.env.FunctionEnvironmentT
import dev.vale.typing.expression.ExpressionCompiler
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.typing.ast._
import dev.vale.typing.env.FunctionEnvironmentBoxT
import dev.vale.typing.types.InterfaceTT
import dev.vale.typing.ast
import dev.vale.typing.function.DestructorCompiler
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
use crate::typing::citizen::impl_compiler::*;
use crate::typing::expression::expression_compiler::*;
use crate::postparsing::ast::LocationInDenizen;

// mig: struct AsSubtypeMacro
pub struct AsSubtypeMacro<'s, 'ctx, 't> {
    pub keywords: &'ctx Keywords<'s>,
    pub impl_compiler: ImplCompiler<'s, 'ctx, 't>,
    pub expression_compiler: ExpressionCompiler<'s, 'ctx, 't>,
    pub destructor_compiler: DestructorCompiler<'s, 'ctx, 't>,
}

// mig: impl AsSubtypeMacro
impl<'s, 'ctx, 't> AsSubtypeMacro<'s, 'ctx, 't> {
}
/*
class AsSubtypeMacro(
  keywords: Keywords,
  implCompiler: ImplCompiler,
  expressionCompiler: ExpressionCompiler,
  destructorCompiler: DestructorCompiler) extends IFunctionBodyMacro {
  val generatorId: StrI = keywords.vale_as_subtype
*/
// mig: fn generate_function_body
fn generate_function_body(
    env: &FunctionEnvironmentT,
    coutputs: &mut CompilerOutputs,
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

    val CoordTemplataT(CoordT(_, _, targetKind)) = vassertSome(env.id.localName.templateArgs.headOption)
    val CoordT(incomingOwnership, _, _) = vassertSome(env.id.localName.parameters.headOption)

    val incomingCoord = paramCoords(0).tyype
    val incomingKind = incomingCoord.kind

    // Because we dont yet put borrows in structs
//    val resultOwnership = incomingCoord.ownership
    val resultOwnership = incomingOwnership
    val successCoord = CoordT(resultOwnership, RegionT(), targetKind)
    val failCoord = CoordT(resultOwnership, RegionT(), incomingKind)
    val (resultCoord, okConstructor, okResultImpl, errConstructor, errResultImpl) =
      expressionCompiler.getResult(coutputs, env, callRange, callLocation, RegionT(), successCoord, failCoord)
    if (resultCoord != vassertSome(maybeRetCoord)) {
      throw CompileErrorExceptionT(RangedInternalErrorT(callRange, "Bad result coord:\n" + resultCoord + "\nand\n" + vassertSome(maybeRetCoord)))
    }


    val subKind = targetKind match { case x : ISubKindTT => x case other => vwat(other) }

    val superKind = incomingKind match { case x : ISuperKindTT => x case other => vwat(other) }


    val implId =
      implCompiler.isParent(coutputs, env, callRange, callLocation, subKind, superKind) match {
        case IsParent(_, _, implId) => implId
      }


    val asSubtypeExpr =
      AsSubtypeTE(
        ArgLookupTE(0, incomingCoord),
        successCoord,
        resultCoord,
        okConstructor,
        errConstructor,
        implId,
        okResultImpl,
        errResultImpl)

    (header, BlockTE(ReturnTE(asSubtypeExpr)))
  }

}
*/
