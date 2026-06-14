use crate::utils::range::RangeS;

use crate::typing::types::types::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::compiler_outputs::*;
use crate::postparsing::ast::LocationInDenizen;
use crate::typing::compiler::Compiler;
use crate::typing::citizen::impl_compiler::IsParentResult;
use crate::typing::ast::expressions::UpcastTE;

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
// deleted: delegate trait removed per god-struct refactor (Compiler now holds all methods directly)
/*
trait IConvertHelperDelegate {
*/
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
/*
class ConvertHelper(
    opts: TypingPassOptions,
    delegate: IConvertHelperDelegate) {
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn convert_exprs(
        &self,
        env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        source_exprs: &[ReferenceExpressionTE<'s, 't>],
        target_pointer_types: &[CoordT<'s, 't>],
    ) -> Vec<ReferenceExpressionTE<'s, 't>> {
        if source_exprs.len() != target_pointer_types.len() {
            panic!(r"num exprs mismatch, source:
{:?}
target:
{:?}", source_exprs, target_pointer_types);
        }

        let mut previous_ref_exprs = Vec::new();
        for (source_expr, target_pointer_type) in source_exprs.iter().zip(target_pointer_types.iter()) {
            let ref_expr =
                self.convert(env, coutputs, range, call_location, *source_expr, *target_pointer_type);
            previous_ref_exprs.push(ref_expr);
        }
        previous_ref_exprs
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
    pub fn convert(
        &self,
        env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        source_expr: ReferenceExpressionTE<'s, 't>,
        target_pointer_type: CoordT<'s, 't>,
    ) -> ReferenceExpressionTE<'s, 't> {
        if source_expr.result().coord == target_pointer_type {
            return source_expr;
        }

        match source_expr.result().coord.kind {
            KindT::Never(_) => return source_expr,
            _ => {}
        }

        let target_ownership = target_pointer_type.ownership;
        let target_type = target_pointer_type.kind;
        let source_ownership = source_expr.result().coord.ownership;
        let source_type = source_expr.result().coord.kind;

        match target_pointer_type.kind {
            KindT::Never(_) => panic!("vcurious: convert targeting Never"),
            _ => {}
        }

        match (source_ownership, target_ownership) {
            (OwnershipT::Own, OwnershipT::Own) => {}
            (OwnershipT::Borrow, OwnershipT::Own) => panic!("Supplied a borrow but target wants to own the argument"),
            (OwnershipT::Own, OwnershipT::Borrow) => panic!("Supplied an owning but target wants to only borrow"),
            (OwnershipT::Borrow, OwnershipT::Borrow) => {}
            (OwnershipT::Share, OwnershipT::Share) => {}
            (OwnershipT::Weak, OwnershipT::Weak) => {}
            _ => panic!("Supplied a {:?} but target wants {:?}", source_ownership, target_ownership),
        }

        if source_type == target_type {
            return source_expr;
        }

        let converted = match (ISubKindTT::try_from(source_type), ISuperKindTT::try_from(target_type)) {
            (Ok(source_sub_kind), Ok(target_super_kind)) => {
                self.convert_with_subkind(env, coutputs, range, call_location, source_expr, source_sub_kind, target_super_kind)
            }
            _ => panic!("vfail: cannot convert {:?} to {:?}", source_type, target_type),
        };
        converted
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
    pub fn convert_with_subkind(
        &self,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        source_expr: ReferenceExpressionTE<'s, 't>,
        source_sub_kind: ISubKindTT<'s, 't>,
        target_super_kind: ISuperKindTT<'s, 't>,
    ) -> ReferenceExpressionTE<'s, 't> {
        match self.is_parent(coutputs, calling_env, range, call_location, source_sub_kind, target_super_kind) {
            IsParentResult::IsParent(is_parent) => {
                assert!(coutputs.get_instantiation_bounds(self.typing_interner, is_parent.impl_id).is_some());
                ReferenceExpressionTE::Upcast(self.typing_interner.alloc(UpcastTE {
                    inner_expr: source_expr,
                    target_super_kind,
                    impl_name: is_parent.impl_id,
                }))
            }
            IsParentResult::IsntParent(_candidates) => {
                panic!("Can't upcast a {:?} to a {:?}", source_sub_kind, target_super_kind)
            }
        }
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
