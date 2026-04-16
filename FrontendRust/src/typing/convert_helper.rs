/*
package dev.vale.typing

import dev.vale.typing.ast.ReferenceExpressionTE
import dev.vale.typing.env.{GlobalEnvironment, IInDenizenEnvironmentT}

import dev.vale.{RangeS, vcurious, vfail}

import dev.vale.typing.types._
import dev.vale._
import dev.vale.typing.ast._
import dev.vale.typing.citizen.{IsParent, IsParentResult, IsntParent}

import dev.vale.typing.function._
//import dev.vale.astronomer.IRulexSR
import dev.vale.typing.citizen.ImplCompiler
import dev.vale.typing.env.IDenizenEnvironmentBoxT
import dev.vale.typing.templata._
import dev.vale.typing.types._

import scala.collection.immutable.List
//import dev.vale.carpenter.CovarianceCarpenter
import dev.vale.postparsing._

*/
use std::collections::{HashMap, HashSet};

use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::postparsing::names::*;
use crate::higher_typing::ast::*;
use crate::typing::citizen::impl_compiler::IsParentResult;

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
use crate::postparsing::ast::LocationInDenizen;
use crate::postparsing::itemplatatype::ITemplataType;
use crate::typing::compilation::*;
use crate::typing::compiler::Compiler;
// mig: trait IConvertHelperDelegate
// deleted: delegate trait removed per god-struct refactor (Compiler now holds all methods directly)
/*
trait IConvertHelperDelegate {
*/
// mig: fn is_parent
/*
  def isParent(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    descendantCitizenRef: ISubKindTT,
    ancestorInterfaceRef: ISuperKindTT):
  IsParentResult
}


*/
// mig: struct ConvertHelper
// vestigial: kept until Step 8 cleanup because sub-compilers still hold `convert_helper: ConvertHelper<'s, 't>` fields
pub struct ConvertHelper<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
// mig: impl ConvertHelper
/*
class ConvertHelper(
    opts: TypingPassOptions,
    delegate: IConvertHelperDelegate) {
*/
// mig: fn convert_exprs
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn convert_exprs(
        &self,
        env: &IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        source_exprs: Vec<ReferenceExpressionTE<'s, 't>>,
        target_pointer_types: Vec<CoordT<'s, 't>>,
    ) -> Vec<ReferenceExpressionTE<'s, 't>> {
        panic!("Unimplemented: convert_exprs");
    }
/*
  def convertExprs(
      env: IInDenizenEnvironmentT,
      coutputs: CompilerOutputs,
      range: List[RangeS],
      callLocation: LocationInDenizen,
      sourceExprs: Vector[ReferenceExpressionTE],
      targetPointerTypes: Vector[CoordT]):
  (Vector[ReferenceExpressionTE]) = {
    if (sourceExprs.size != targetPointerTypes.size) {
      throw CompileErrorExceptionT(RangedInternalErrorT(range, "num exprs mismatch, source:\n" + sourceExprs + "\ntarget:\n" + targetPointerTypes))
    }

    (sourceExprs zip targetPointerTypes).foldLeft((Vector[ReferenceExpressionTE]()))({
      case ((previousRefExprs), (sourceExpr, targetPointerType)) => {
        val refExpr =
          convert(env, coutputs, range, callLocation, sourceExpr, targetPointerType)
        (previousRefExprs :+ refExpr)
      }

    })
  }


*/
}

// mig: fn convert
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn convert(
        &self,
        env: &IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        source_expr: ReferenceExpressionTE<'s, 't>,
        target_pointer_type: CoordT<'s, 't>,
    ) -> ReferenceExpressionTE<'s, 't> {
        panic!("Unimplemented: convert");
    }
/*
  def convert(
      env: IInDenizenEnvironmentT,
      coutputs: CompilerOutputs,
      range: List[RangeS],
      callLocation: LocationInDenizen,
      sourceExpr: ReferenceExpressionTE,
      targetPointerType: CoordT):
  (ReferenceExpressionTE) = {
    val sourcePointerType = sourceExpr.result.coord

    if (sourceExpr.result.coord == targetPointerType) {
      return sourceExpr
    }


    sourceExpr.result.coord.kind match {
      case NeverT(_) => return sourceExpr
      case _ =>
    }


    val CoordT(targetOwnership, _, targetType) = targetPointerType;
    val CoordT(sourceOwnership, _, sourceType) = sourcePointerType;

    targetPointerType.kind match {
      case NeverT(_) => vcurious()
      case _ =>
    }


    // We make the hammer aware of nevers.
//    if (sourceType == Never2()) {
//      return (CompilerReinterpret2(sourceExpr, targetPointerType))
//    }


    (sourceOwnership, targetOwnership) match {
      case (OwnT, OwnT) =>
      case (BorrowT, OwnT) => {
        throw CompileErrorExceptionT(RangedInternalErrorT(range, "Supplied a borrow but target wants to own the argument"))
      }

      case (OwnT, BorrowT) => {
        throw CompileErrorExceptionT(RangedInternalErrorT(range, "Supplied an owning but target wants to only borrow"))
      }

      case (BorrowT, BorrowT) =>
      case (ShareT, ShareT) =>
      case (WeakT, WeakT) =>
      case _ => throw CompileErrorExceptionT(RangedInternalErrorT(range, "Supplied a " + sourceOwnership + " but target wants " + targetOwnership))
    }


    val sourceExprConverted =
      if (sourceType == targetType) {
        sourceExpr
      } else {
        (sourceType, targetType) match {
          case (s : ISubKindTT, i : ISuperKindTT) => {
            convert(env, coutputs, range, callLocation, sourceExpr, s, i)
          }

          case _ => vfail()
        }

      };

    (sourceExprConverted)
  }


*/
}

// mig: fn convert
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn convert_with_subkind(
        &self,
        calling_env: &IInDenizenEnvironmentT,
        coutputs: &mut CompilerOutputs,
        range: &[RangeS],
        call_location: LocationInDenizen,
        source_expr: ReferenceExpressionTE,
        source_sub_kind: ISubKindTT,
        target_super_kind: ISuperKindTT,
    ) -> ReferenceExpressionTE {
        panic!("Unimplemented: convert");
    }
/*
  def convert(
    callingEnv: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    range: List[RangeS],
    callLocation: LocationInDenizen,
    sourceExpr: ReferenceExpressionTE,
    sourceSubKind: ISubKindTT,
    targetSuperKind: ISuperKindTT):
  (ReferenceExpressionTE) = {
    delegate.isParent(coutputs, callingEnv, range, callLocation, sourceSubKind, targetSuperKind) match {
      case IsParent(_, _, implId) => {
        vassert(coutputs.getInstantiationBounds(implId).nonEmpty)
        UpcastTE(
          sourceExpr, targetSuperKind, implId)
      }

      case IsntParent(candidates) => {
        throw CompileErrorExceptionT(RangedInternalErrorT(range, "Can't upcast a " + sourceSubKind + " to a " + targetSuperKind + ": " + candidates))
      }

    }

  }

}
*/
}
