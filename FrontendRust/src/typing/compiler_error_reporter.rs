use crate::postparsing::names::{IFunctionDeclarationNameS, IImpreciseNameS, INameS, IRuneS};
use crate::postparsing::rules::rules::IRulexSR;
use crate::postparsing::rune_type_solver::RuneTypeSolveError;
use crate::solver::solver::FailedSolve;
use crate::typing::ast::ast::{KindExportT, PrototypeT, SignatureT};
use crate::typing::infer::compiler_solver::ITypingPassSolverError;
use crate::typing::infer_compiler::{IDefiningError, IResolvingError};
use crate::typing::names::names::{IdT, IVarNameT};
use crate::typing::overload_resolver::FindFunctionFailure;
use crate::typing::templata::templata::ITemplataT;
use crate::typing::types::types::{CoordT, InterfaceTT, KindT, StructTT};
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::RangeS;

/*
package dev.vale.typing

import dev.vale.postparsing._
import dev.vale.postparsing.rules.IRulexSR
import dev.vale.solver.FailedSolve
import dev.vale.typing.infer.ITypingPassSolverError
import dev.vale.typing.templata.ITemplataT
import dev.vale.{PackageCoordinate, RangeS, vbreak, vcurious, vfail, vpass}
import dev.vale.typing.types._
import dev.vale.postparsing.RuneTypeSolveError
import dev.vale.solver.FailedSolve
import OverloadResolver._
import dev.vale.typing.ast.{KindExportT, SignatureT}
import dev.vale.typing.names._
import dev.vale.typing.ast._
import dev.vale.typing.types.InterfaceTT
*/

/*
case class CompileErrorExceptionT(err: ICompileErrorT) extends RuntimeException {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
#[derive(Debug)]
pub enum ICompileErrorT<'s, 't> {
    CouldntNarrowDownCandidates { range: &'t [RangeS<'s>], candidates: &'t [PrototypeT<'s, 't>] },
    CouldntSolveRuneTypesT { range: &'t [RangeS<'s>], error: RuneTypeSolveError<'s> },
    NotEnoughGenericArgs { range: &'t [RangeS<'s>] },
    ImplSubCitizenNotFound { range: &'t [RangeS<'s>], name: IImpreciseNameS<'s> },
    ImplSuperInterfaceNotFound { range: &'t [RangeS<'s>], name: IImpreciseNameS<'s> },
    ImmStructCantHaveVaryingMember { range: &'t [RangeS<'s>], struct_name: INameS<'s>, member_name: &'s str },
    ImmStructCantHaveMutableMember { range: &'t [RangeS<'s>], struct_name: INameS<'s>, member_name: &'s str },
    CantReconcileBranchesResults { range: &'t [RangeS<'s>], then_result: CoordT<'s, 't>, else_result: CoordT<'s, 't> },
    IndexedArrayWithNonInteger { range: &'t [RangeS<'s>], types: CoordT<'s, 't> },
    WrongNumberOfDestructuresError { range: &'t [RangeS<'s>], actual_num: i32, expected_num: i32 },
    CantDowncastUnrelatedTypes {
        range: &'t [RangeS<'s>],
        source_kind: KindT<'s, 't>,
        target_kind: KindT<'s, 't>,
        candidates: &'t [FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>],
    },
    CantDowncastToInterface { range: &'t [RangeS<'s>], target_kind: InterfaceTT<'s, 't> },
    CouldntFindTypeT { range: &'t [RangeS<'s>], name: IImpreciseNameS<'s> },
    TooManyTypesWithNameT { range: &'t [RangeS<'s>], name: IImpreciseNameS<'s> },
    ArrayElementsHaveDifferentTypes { range: &'t [RangeS<'s>], types: &'t [CoordT<'s, 't>] },
    UnexpectedArrayElementType { range: &'t [RangeS<'s>], expected_type: CoordT<'s, 't>, actual_type: CoordT<'s, 't> },
    InitializedWrongNumberOfElements { range: &'t [RangeS<'s>], expected_num_elements: i32, num_elements_initialized: i32 },
    NewImmRSANeedsCallable { range: &'t [RangeS<'s>] },
    CannotSubscriptT { range: &'t [RangeS<'s>], tyype: KindT<'s, 't> },
    NonReadonlyReferenceFoundInPureFunctionParameter { range: &'t [RangeS<'s>], param_name: IVarNameT<'s, 't> },
    CouldntFindIdentifierToLoadT { range: &'t [RangeS<'s>], name: IImpreciseNameS<'s> },
    CouldntFindMemberT { range: &'t [RangeS<'s>], member_name: &'s str },
    BodyResultDoesntMatch {
        range: &'t [RangeS<'s>],
        function_name: IFunctionDeclarationNameS<'s>,
        expected_return_type: CoordT<'s, 't>,
        result_type: CoordT<'s, 't>,
    },
    CouldntConvertForReturnT { range: &'t [RangeS<'s>], expected_type: CoordT<'s, 't>, actual_type: CoordT<'s, 't> },
    CouldntConvertForMutateT { range: &'t [RangeS<'s>], expected_type: CoordT<'s, 't>, actual_type: CoordT<'s, 't> },
    CantMoveOutOfMemberT { range: &'t [RangeS<'s>], name: IVarNameT<'s, 't> },
    CouldntFindFunctionToCallT { range: &'t [RangeS<'s>], fff: FindFunctionFailure<'s, 't> },
    CouldntEvaluateFunction { range: &'t [RangeS<'s>], eff: IDefiningError<'s, 't> },
    CouldntEvaluatImpl {
        range: &'t [RangeS<'s>],
        eff: FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>,
    },
    CouldntEvaluateStruct {
        range: &'t [RangeS<'s>],
        eff: FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>,
    },
    CouldntEvaluateInterface {
        range: &'t [RangeS<'s>],
        eff: FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>,
    },
    CouldntFindOverrideT { range: &'t [RangeS<'s>], fff: FindFunctionFailure<'s, 't> },
    ExportedFunctionDependedOnNonExportedKind {
        range: &'t [RangeS<'s>],
        paackage: PackageCoordinate<'s>,
        signature: &'t SignatureT<'s, 't>,
        non_exported_kind: KindT<'s, 't>,
    },
    ExternFunctionDependedOnNonExportedKind {
        range: &'t [RangeS<'s>],
        paackage: PackageCoordinate<'s>,
        signature: &'t SignatureT<'s, 't>,
        non_exported_kind: KindT<'s, 't>,
    },
    ExportedImmutableKindDependedOnNonExportedKind {
        range: &'t [RangeS<'s>],
        paackage: PackageCoordinate<'s>,
        exported_kind: KindT<'s, 't>,
        non_exported_kind: KindT<'s, 't>,
    },
    TypeExportedMultipleTimes { range: &'t [RangeS<'s>], paackage: PackageCoordinate<'s>, exports: &'t [KindExportT<'s, 't>] },
    CantUseUnstackifiedLocal { range: &'t [RangeS<'s>], local_id: IVarNameT<'s, 't> },
    CantUnstackifyOutsideLocalFromInsideWhile { range: &'t [RangeS<'s>], local_id: IVarNameT<'s, 't> },
    CantRestackifyOutsideLocalFromInsideWhile { range: &'t [RangeS<'s>], local_id: IVarNameT<'s, 't> },
    FunctionAlreadyExists { old_function_range: RangeS<'s>, new_function_range: RangeS<'s>, signature: IdT<'s, 't> },
    CantMutateFinalMember { range: &'t [RangeS<'s>], struct_: StructTT<'s, 't>, member_name: IVarNameT<'s, 't> },
    CantMutateFinalElement { range: &'t [RangeS<'s>], coord: CoordT<'s, 't> },
    CantUseReadonlyReferenceAsReadwrite { range: &'t [RangeS<'s>] },
    LambdaReturnDoesntMatchInterfaceConstructor { range: &'t [RangeS<'s>] },
    IfConditionIsntBoolean { range: &'t [RangeS<'s>], actual_type: CoordT<'s, 't> },
    WhileConditionIsntBoolean { range: &'t [RangeS<'s>], actual_type: CoordT<'s, 't> },
    HigherTypingInferError { range: &'t [RangeS<'s>], err: RuneTypeSolveError<'s> },
    AbstractMethodOutsideOpenInterface { range: &'t [RangeS<'s>] },
    TypingPassSolverError {
        range: &'t [RangeS<'s>],
        failed_solve: FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>,
    },
    TypingPassResolvingError { range: &'t [RangeS<'s>], inner: IResolvingError<'s, 't> },
    TypingPassDefiningError { range: &'t [RangeS<'s>], inner: IDefiningError<'s, 't> },
    CantImplNonInterface { range: &'t [RangeS<'s>], templata: ITemplataT<'s, 't> },
    NonCitizenCantImpl { range: &'t [RangeS<'s>], templata: ITemplataT<'s, 't> },
    RangedInternalErrorT { range: &'t [RangeS<'s>], message: &'s str },
}
/*
sealed trait ICompileErrorT { def range: List[RangeS] }
*/
// mig: fn range
impl<'s, 't> ICompileErrorT<'s, 't> {
    pub fn range(&self) -> &[RangeS<'s>] {
        match self {
            Self::CouldntNarrowDownCandidates { range, .. } => *range,
            Self::CouldntSolveRuneTypesT { range, .. } => *range,
            Self::NotEnoughGenericArgs { range, .. } => *range,
            Self::ImplSubCitizenNotFound { range, .. } => *range,
            Self::ImplSuperInterfaceNotFound { range, .. } => *range,
            Self::ImmStructCantHaveVaryingMember { range, .. } => *range,
            Self::ImmStructCantHaveMutableMember { range, .. } => *range,
            Self::CantReconcileBranchesResults { range, .. } => *range,
            Self::IndexedArrayWithNonInteger { range, .. } => *range,
            Self::WrongNumberOfDestructuresError { range, .. } => *range,
            Self::CantDowncastUnrelatedTypes { range, .. } => *range,
            Self::CantDowncastToInterface { range, .. } => *range,
            Self::CouldntFindTypeT { range, .. } => *range,
            Self::TooManyTypesWithNameT { range, .. } => *range,
            Self::ArrayElementsHaveDifferentTypes { range, .. } => *range,
            Self::UnexpectedArrayElementType { range, .. } => *range,
            Self::InitializedWrongNumberOfElements { range, .. } => *range,
            Self::NewImmRSANeedsCallable { range, .. } => *range,
            Self::CannotSubscriptT { range, .. } => *range,
            Self::NonReadonlyReferenceFoundInPureFunctionParameter { range, .. } => *range,
            Self::CouldntFindIdentifierToLoadT { range, .. } => *range,
            Self::CouldntFindMemberT { range, .. } => *range,
            Self::BodyResultDoesntMatch { range, .. } => *range,
            Self::CouldntConvertForReturnT { range, .. } => *range,
            Self::CouldntConvertForMutateT { range, .. } => *range,
            Self::CantMoveOutOfMemberT { range, .. } => *range,
            Self::CouldntFindFunctionToCallT { range, .. } => *range,
            Self::CouldntEvaluateFunction { range, .. } => *range,
            Self::CouldntEvaluatImpl { range, .. } => *range,
            Self::CouldntEvaluateStruct { range, .. } => *range,
            Self::CouldntEvaluateInterface { range, .. } => *range,
            Self::CouldntFindOverrideT { range, .. } => *range,
            Self::ExportedFunctionDependedOnNonExportedKind { range, .. } => *range,
            Self::ExternFunctionDependedOnNonExportedKind { range, .. } => *range,
            Self::ExportedImmutableKindDependedOnNonExportedKind { range, .. } => *range,
            Self::TypeExportedMultipleTimes { range, .. } => *range,
            Self::CantUseUnstackifiedLocal { range, .. } => *range,
            Self::CantUnstackifyOutsideLocalFromInsideWhile { range, .. } => *range,
            Self::CantRestackifyOutsideLocalFromInsideWhile { range, .. } => *range,
            Self::FunctionAlreadyExists { new_function_range, .. } => std::slice::from_ref(new_function_range),
            Self::CantMutateFinalMember { range, .. } => *range,
            Self::CantMutateFinalElement { range, .. } => *range,
            Self::CantUseReadonlyReferenceAsReadwrite { range, .. } => *range,
            Self::LambdaReturnDoesntMatchInterfaceConstructor { range, .. } => *range,
            Self::IfConditionIsntBoolean { range, .. } => *range,
            Self::WhileConditionIsntBoolean { range, .. } => *range,
            Self::HigherTypingInferError { range, .. } => *range,
            Self::AbstractMethodOutsideOpenInterface { range, .. } => *range,
            Self::TypingPassSolverError { range, .. } => *range,
            Self::TypingPassResolvingError { range, .. } => *range,
            Self::TypingPassDefiningError { range, .. } => *range,
            Self::CantImplNonInterface { range, .. } => *range,
            Self::NonCitizenCantImpl { range, .. } => *range,
            Self::RangedInternalErrorT { range, .. } => *range,
        }
    }
}
/*
case class CouldntNarrowDownCandidates(range: List[RangeS], candidates: Vector[PrototypeT[IFunctionNameT]]) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
/*
case class CouldntSolveRuneTypesT(range: List[RangeS], error: RuneTypeSolveError) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class NotEnoughGenericArgs(range: List[RangeS]) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class ImplSubCitizenNotFound(range: List[RangeS], name: IImpreciseNameS) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class ImplSuperInterfaceNotFound(range: List[RangeS], name: IImpreciseNameS) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class ImmStructCantHaveVaryingMember(range: List[RangeS], structName: INameS, memberName: String) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class ImmStructCantHaveMutableMember(range: List[RangeS], structName: INameS, memberName: String) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class CantReconcileBranchesResults(range: List[RangeS], thenResult: CoordT, elseResult: CoordT) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
  thenResult.kind match {
    case NeverT(_) => vfail()
    case _ =>
  }
  elseResult.kind match {
    case NeverT(_) => vfail()
    case _ =>
  }
}
*/
/*
case class IndexedArrayWithNonInteger(range: List[RangeS], types: CoordT) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class WrongNumberOfDestructuresError(range: List[RangeS], actualNum: Int, expectedNum: Int) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class CantDowncastUnrelatedTypes(range: List[RangeS], sourceKind: KindT, targetKind: KindT, candidates: Vector[FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]]) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
/*
case class CantDowncastToInterface(range: List[RangeS], targetKind: InterfaceTT) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class CantUseRuneValueAsExpression(range: List[RangeS], rune: IRuneS) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class CouldntFindTypeT(range: List[RangeS], name: IImpreciseNameS) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class TooManyTypesWithNameT(range: List[RangeS], name: IImpreciseNameS) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class ArrayElementsHaveDifferentTypes(range: List[RangeS], types: Set[CoordT]) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class UnexpectedArrayElementType(range: List[RangeS], expectedType: CoordT, actualType: CoordT) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class InitializedWrongNumberOfElements(range: List[RangeS], expectedNumElements: Int, numElementsInitialized: Int) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class NewImmRSANeedsCallable(range: List[RangeS]) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class CannotSubscriptT(range: List[RangeS], tyype: KindT) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class NonReadonlyReferenceFoundInPureFunctionParameter(range: List[RangeS], paramName: IVarNameT) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class CouldntFindIdentifierToLoadT(range: List[RangeS], name: IImpreciseNameS) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
/*
case class CouldntFindMemberT(range: List[RangeS], memberName: String) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class BodyResultDoesntMatch(range: List[RangeS], functionName: IFunctionDeclarationNameS, expectedReturnType: CoordT, resultType: CoordT) extends ICompileErrorT {
  vpass()
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class CouldntConvertForReturnT(range: List[RangeS], expectedType: CoordT, actualType: CoordT) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
/*
case class CouldntConvertForMutateT(range: List[RangeS], expectedType: CoordT, actualType: CoordT) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class CantMoveOutOfMemberT(range: List[RangeS], name: IVarNameT) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class CouldntFindFunctionToCallT(range: List[RangeS], fff: FindFunctionFailure) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()

  vpass()
}
*/
/*
case class CouldntEvaluateFunction(range: List[RangeS], eff: IDefiningError) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class CouldntEvaluatImpl(range: List[RangeS], eff: FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
/*
case class CouldntEvaluateStruct(range: List[RangeS], eff: FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
/*
case class CouldntEvaluateInterface(range: List[RangeS], eff: FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
/*
case class CouldntFindOverrideT(range: List[RangeS], fff: FindFunctionFailure) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
/*
case class ExportedFunctionDependedOnNonExportedKind(range: List[RangeS], paackage: PackageCoordinate, signature: SignatureT, nonExportedKind: KindT) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class ExternFunctionDependedOnNonExportedKind(range: List[RangeS], paackage: PackageCoordinate, signature: SignatureT, nonExportedKind: KindT) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class ExportedImmutableKindDependedOnNonExportedKind(range: List[RangeS], paackage: PackageCoordinate, exportedKind: KindT, nonExportedKind: KindT) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
/*
case class TypeExportedMultipleTimes(range: List[RangeS], paackage: PackageCoordinate, exports: Vector[KindExportT]) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class CantUseUnstackifiedLocal(range: List[RangeS], localId: IVarNameT) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
/*
case class CantUnstackifyOutsideLocalFromInsideWhile(range: List[RangeS], localId: IVarNameT) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class CantRestackifyOutsideLocalFromInsideWhile(range: List[RangeS], localId: IVarNameT) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class FunctionAlreadyExists(oldFunctionRange: RangeS, newFunctionRange: RangeS, signature: IdT[IFunctionNameT]) extends ICompileErrorT {
  override def range: List[RangeS] = List(newFunctionRange)
  vpass()
}
*/
/*
case class CantMutateFinalMember(range: List[RangeS], struct: StructTT, memberName: IVarNameT) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class CantMutateFinalElement(range: List[RangeS], coord: CoordT) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class CantUseReadonlyReferenceAsReadwrite(range: List[RangeS]) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class LambdaReturnDoesntMatchInterfaceConstructor(range: List[RangeS]) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class IfConditionIsntBoolean(range: List[RangeS], actualType: CoordT) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class WhileConditionIsntBoolean(range: List[RangeS], actualType: CoordT) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class HigherTypingInferError(range: List[RangeS], err: RuneTypeSolveError) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class AbstractMethodOutsideOpenInterface(range: List[RangeS]) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
//case class NotEnoughToSolveError(range: List[RangeS], conclusions: Map[IRuneS, ITemplata[ITemplataType]], unknownRunes: Iterable[IRuneS]) extends ICompileErrorT {
// override def equals(obj: Any): Boolean = vcurious();
//override def hashCode(): Int = vcurious() }
*/
/*
case class TypingPassSolverError(range: List[RangeS], failedSolve: FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
/*
case class TypingPassResolvingError(range: List[RangeS], inner: IResolvingError) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
/*
case class TypingPassDefiningError(range: List[RangeS], inner: IDefiningError) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
/*
//case class CompilerSolverConflict(range: List[RangeS], conclusions: Map[IRuneS, ITemplata[ITemplataType]], rune: IRuneS, conflictingNewConclusion: ITemplata[ITemplataType]) extends ICompileErrorT {
// override def equals(obj: Any): Boolean = vcurious();
//override def hashCode(): Int = vcurious() }
*/
/*
case class CantImplNonInterface(range: List[RangeS], templata: ITemplataT[ITemplataType]) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
/*
case class NonCitizenCantImpl(range: List[RangeS], templata: ITemplataT[ITemplataType]) extends ICompileErrorT {
override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
// REMEMBER: Add any new errors to the "Humanize errors" test
*/
/*
case class RangedInternalErrorT(range: List[RangeS], message: String) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
*/
/*
  vbreak()
}
*/
/*

object ErrorReporter {
*/
/*
  def report(err: ICompileErrorT): Nothing = {
    throw CompileErrorExceptionT(err)
  }
*/
/*
}
*/
