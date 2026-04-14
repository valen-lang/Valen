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
// mig: struct CompileErrorExceptionT
// mig: impl CompileErrorExceptionT
/*
case class CompileErrorExceptionT(err: ICompileErrorT) extends RuntimeException {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: enum ICompileErrorT
/*
sealed trait ICompileErrorT { def range: List[RangeS] }
*/
// mig: struct CouldntNarrowDownCandidates
// mig: impl CouldntNarrowDownCandidates
/*
case class CouldntNarrowDownCandidates(range: List[RangeS], candidates: Vector[RangeS]) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct CouldntSolveRuneTypesT
// mig: impl CouldntSolveRuneTypesT
/*
case class CouldntSolveRuneTypesT(range: List[RangeS], error: RuneTypeSolveError) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
// mig: struct NotEnoughGenericArgs
// mig: impl NotEnoughGenericArgs
/*
case class NotEnoughGenericArgs(range: List[RangeS]) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct ImplSubCitizenNotFound
// mig: impl ImplSubCitizenNotFound
/*
case class ImplSubCitizenNotFound(range: List[RangeS], name: IImpreciseNameS) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct ImplSuperInterfaceNotFound
// mig: impl ImplSuperInterfaceNotFound
/*
case class ImplSuperInterfaceNotFound(range: List[RangeS], name: IImpreciseNameS) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct ImmStructCantHaveVaryingMember
// mig: impl ImmStructCantHaveVaryingMember
/*
case class ImmStructCantHaveVaryingMember(range: List[RangeS], structName: INameS, memberName: String) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct ImmStructCantHaveMutableMember
// mig: impl ImmStructCantHaveMutableMember
/*
case class ImmStructCantHaveMutableMember(range: List[RangeS], structName: INameS, memberName: String) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct CantReconcileBranchesResults
// mig: impl CantReconcileBranchesResults
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
// mig: struct IndexedArrayWithNonInteger
// mig: impl IndexedArrayWithNonInteger
/*
case class IndexedArrayWithNonInteger(range: List[RangeS], types: CoordT) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct WrongNumberOfDestructuresError
// mig: impl WrongNumberOfDestructuresError
/*
case class WrongNumberOfDestructuresError(range: List[RangeS], actualNum: Int, expectedNum: Int) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct CantDowncastUnrelatedTypes
// mig: impl CantDowncastUnrelatedTypes
/*
case class CantDowncastUnrelatedTypes(range: List[RangeS], sourceKind: KindT, targetKind: KindT, candidates: Vector[FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]]) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct CantDowncastToInterface
// mig: impl CantDowncastToInterface
/*
case class CantDowncastToInterface(range: List[RangeS], targetKind: InterfaceTT) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct CouldntFindTypeT
// mig: impl CouldntFindTypeT
/*
case class CouldntFindTypeT(range: List[RangeS], name: IImpreciseNameS) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct TooManyTypesWithNameT
// mig: impl TooManyTypesWithNameT
/*
case class TooManyTypesWithNameT(range: List[RangeS], name: IImpreciseNameS) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct ArrayElementsHaveDifferentTypes
// mig: impl ArrayElementsHaveDifferentTypes
/*
case class ArrayElementsHaveDifferentTypes(range: List[RangeS], types: Set[CoordT]) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct UnexpectedArrayElementType
// mig: impl UnexpectedArrayElementType
/*
case class UnexpectedArrayElementType(range: List[RangeS], expectedType: CoordT, actualType: CoordT) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct InitializedWrongNumberOfElements
// mig: impl InitializedWrongNumberOfElements
/*
case class InitializedWrongNumberOfElements(range: List[RangeS], expectedNumElements: Int, numElementsInitialized: Int) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct NewImmRSANeedsCallable
// mig: impl NewImmRSANeedsCallable
/*
case class NewImmRSANeedsCallable(range: List[RangeS]) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct CannotSubscriptT
// mig: impl CannotSubscriptT
/*
case class CannotSubscriptT(range: List[RangeS], tyype: KindT) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct NonReadonlyReferenceFoundInPureFunctionParameter
// mig: impl NonReadonlyReferenceFoundInPureFunctionParameter
/*
case class NonReadonlyReferenceFoundInPureFunctionParameter(range: List[RangeS], paramName: IVarNameT) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct CouldntFindIdentifierToLoadT
// mig: impl CouldntFindIdentifierToLoadT
/*
case class CouldntFindIdentifierToLoadT(range: List[RangeS], name: IImpreciseNameS) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct CouldntFindMemberT
// mig: impl CouldntFindMemberT
/*
case class CouldntFindMemberT(range: List[RangeS], memberName: String) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct BodyResultDoesntMatch
// mig: impl BodyResultDoesntMatch
/*
case class BodyResultDoesntMatch(range: List[RangeS], functionName: IFunctionDeclarationNameS, expectedReturnType: CoordT, resultType: CoordT) extends ICompileErrorT {
  vpass()
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
// mig: struct CouldntConvertForReturnT
// mig: impl CouldntConvertForReturnT
/*
case class CouldntConvertForReturnT(range: List[RangeS], expectedType: CoordT, actualType: CoordT) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct CouldntConvertForMutateT
// mig: impl CouldntConvertForMutateT
/*
case class CouldntConvertForMutateT(range: List[RangeS], expectedType: CoordT, actualType: CoordT) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct CantMoveOutOfMemberT
// mig: impl CantMoveOutOfMemberT
/*
case class CantMoveOutOfMemberT(range: List[RangeS], name: IVarNameT) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct CouldntFindFunctionToCallT
// mig: impl CouldntFindFunctionToCallT
/*
case class CouldntFindFunctionToCallT(range: List[RangeS], fff: FindFunctionFailure) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()

  vpass()
}
*/
// mig: struct CouldntEvaluateFunction
// mig: impl CouldntEvaluateFunction
/*
case class CouldntEvaluateFunction(range: List[RangeS], eff: IDefiningError) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct CouldntEvaluatImpl
// mig: impl CouldntEvaluatImpl
/*
case class CouldntEvaluatImpl(range: List[RangeS], eff: FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct CouldntEvaluateStruct
// mig: impl CouldntEvaluateStruct
/*
case class CouldntEvaluateStruct(range: List[RangeS], eff: FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct CouldntEvaluateInterface
// mig: impl CouldntEvaluateInterface
/*
case class CouldntEvaluateInterface(range: List[RangeS], eff: FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct CouldntFindOverrideT
// mig: impl CouldntFindOverrideT
/*
case class CouldntFindOverrideT(range: List[RangeS], fff: FindFunctionFailure) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct ExportedFunctionDependedOnNonExportedKind
// mig: impl ExportedFunctionDependedOnNonExportedKind
/*
case class ExportedFunctionDependedOnNonExportedKind(range: List[RangeS], paackage: PackageCoordinate, signature: SignatureT, nonExportedKind: KindT) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct ExternFunctionDependedOnNonExportedKind
// mig: impl ExternFunctionDependedOnNonExportedKind
/*
case class ExternFunctionDependedOnNonExportedKind(range: List[RangeS], paackage: PackageCoordinate, signature: SignatureT, nonExportedKind: KindT) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct ExportedImmutableKindDependedOnNonExportedKind
// mig: impl ExportedImmutableKindDependedOnNonExportedKind
/*
case class ExportedImmutableKindDependedOnNonExportedKind(range: List[RangeS], paackage: PackageCoordinate, exportedKind: KindT, nonExportedKind: KindT) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct TypeExportedMultipleTimes
// mig: impl TypeExportedMultipleTimes
/*
case class TypeExportedMultipleTimes(range: List[RangeS], paackage: PackageCoordinate, exports: Vector[KindExportT]) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct CantUseUnstackifiedLocal
// mig: impl CantUseUnstackifiedLocal
/*
case class CantUseUnstackifiedLocal(range: List[RangeS], localId: IVarNameT) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct CantUnstackifyOutsideLocalFromInsideWhile
// mig: impl CantUnstackifyOutsideLocalFromInsideWhile
/*
case class CantUnstackifyOutsideLocalFromInsideWhile(range: List[RangeS], localId: IVarNameT) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct CantRestackifyOutsideLocalFromInsideWhile
// mig: impl CantRestackifyOutsideLocalFromInsideWhile
/*
case class CantRestackifyOutsideLocalFromInsideWhile(range: List[RangeS], localId: IVarNameT) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct FunctionAlreadyExists
// mig: impl FunctionAlreadyExists
/*
case class FunctionAlreadyExists(oldFunctionRange: RangeS, newFunctionRange: RangeS, signature: IdT[IFunctionNameT]) extends ICompileErrorT {
  override def range: List[RangeS] = List(newFunctionRange)
  vpass()
}
*/
// mig: struct CantMutateFinalMember
// mig: impl CantMutateFinalMember
/*
case class CantMutateFinalMember(range: List[RangeS], struct: StructTT, memberName: IVarNameT) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct CantMutateFinalElement
// mig: impl CantMutateFinalElement
/*
case class CantMutateFinalElement(range: List[RangeS], coord: CoordT) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct CantUseReadonlyReferenceAsReadwrite
// mig: impl CantUseReadonlyReferenceAsReadwrite
/*
case class CantUseReadonlyReferenceAsReadwrite(range: List[RangeS]) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct LambdaReturnDoesntMatchInterfaceConstructor
// mig: impl LambdaReturnDoesntMatchInterfaceConstructor
/*
case class LambdaReturnDoesntMatchInterfaceConstructor(range: List[RangeS]) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct IfConditionIsntBoolean
// mig: impl IfConditionIsntBoolean
/*
case class IfConditionIsntBoolean(range: List[RangeS], actualType: CoordT) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct WhileConditionIsntBoolean
// mig: impl WhileConditionIsntBoolean
/*
case class WhileConditionIsntBoolean(range: List[RangeS], actualType: CoordT) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct CantMoveFromGlobal
// mig: impl CantMoveFromGlobal
/*
case class CantMoveFromGlobal(range: List[RangeS], name: String) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct HigherTypingInferError
// mig: impl HigherTypingInferError
/*
case class HigherTypingInferError(range: List[RangeS], err: RuneTypeSolveError) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct AbstractMethodOutsideOpenInterface
// mig: impl AbstractMethodOutsideOpenInterface
/*
case class AbstractMethodOutsideOpenInterface(range: List[RangeS]) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
//case class NotEnoughToSolveError(range: List[RangeS], conclusions: Map[IRuneS, ITemplata[ITemplataType]], unknownRunes: Iterable[IRuneS]) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct TypingPassSolverError
// mig: impl TypingPassSolverError
/*
case class TypingPassSolverError(range: List[RangeS], failedSolve: FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct TypingPassResolvingError
// mig: impl TypingPassResolvingError
/*
case class TypingPassResolvingError(range: List[RangeS], inner: IResolvingError) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct TypingPassDefiningError
// mig: impl TypingPassDefiningError
/*
case class TypingPassDefiningError(range: List[RangeS], inner: IDefiningError) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vpass()
}
//case class CompilerSolverConflict(range: List[RangeS], conclusions: Map[IRuneS, ITemplata[ITemplataType]], rune: IRuneS, conflictingNewConclusion: ITemplata[ITemplataType]) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct CantImplNonInterface
// mig: impl CantImplNonInterface
/*
case class CantImplNonInterface(range: List[RangeS], templata: ITemplataT[ITemplataType]) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
// mig: struct NonCitizenCantImpl
// mig: impl NonCitizenCantImpl
/*
case class NonCitizenCantImpl(range: List[RangeS], templata: ITemplataT[ITemplataType]) extends ICompileErrorT { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
// REMEMBER: Add any new errors to the "Humanize errors" test
*/
// mig: struct RangedInternalErrorT
// mig: impl RangedInternalErrorT
/*
case class RangedInternalErrorT(range: List[RangeS], message: String) extends ICompileErrorT {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  vbreak()
}

object ErrorReporter {
*/
// mig: def report
/*
  def report(err: ICompileErrorT): Nothing = {
    throw CompileErrorExceptionT(err)
  }
*/
/*
}
*/
