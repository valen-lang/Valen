use crate::utils::range::{RangeS, CodeLocationS};
use crate::interner::Interner;
use crate::postparsing::*;
use crate::postparsing::names::*;
use crate::postparsing::rules::rules::*;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::ast::expressions::*;
use crate::typing::compiler_error_reporter::*;
use crate::typing::compiler_outputs::*;
use crate::typing::infer::compiler_solver::*;
use crate::typing::infer_compiler::*;
use crate::typing::overload_resolver::*;
use crate::typing::compilation::TypingPassOptions;
use crate::solver::solver::*;
use crate::higher_typing::ast::*;
use crate::higher_typing::ast::FunctionA;
use crate::typing::citizen::struct_compiler::*;
use crate::utils::code_hierarchy::FileCoordinate;

/*
package dev.vale.typing

import dev.vale._
import dev.vale.postparsing._
import dev.vale.postparsing.rules.IRulexSR
import dev.vale.solver.{FailedSolve, RuleError, SolveIncomplete, SolverErrorHumanizer}
import dev.vale.typing.types._
import dev.vale.SourceCodeUtils.{humanizePos, lineBegin, lineContaining, lineRangeContaining, linesBetween}
import dev.vale.highertyping.FunctionA
import PostParserErrorHumanizer._
import dev.vale.postparsing.rules.IRulexSR
import dev.vale.postparsing.PostParserErrorHumanizer
import OverloadResolver._
import dev.vale.highertyping._
import dev.vale.typing.CompilerErrorHumanizer.humanizeResolvingError
import dev.vale.typing.ast._
import dev.vale.typing.infer._
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.ast._
import dev.vale.typing.templata.Conversions
import dev.vale.typing.types.CoordT
import dev.vale.typing.citizen.ResolveFailure

object CompilerErrorHumanizer {
*/
pub fn humanize<'s, 't>(verbose: bool, code_map: &dyn Fn(CodeLocationS) -> String, lines_between: &dyn Fn(CodeLocationS, CodeLocationS) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS) -> String, err: ICompileErrorT<'s, 't>) -> String {
  panic!("Unimplemented: humanize");
}
/*
  def humanize(
      verbose: Boolean,
    codeMap: CodeLocationS => String,
    linesBetween: (CodeLocationS, CodeLocationS) => Vector[RangeS],
    lineRangeContaining: (CodeLocationS) => RangeS,
    lineContaining: (CodeLocationS) => String,
      err: ICompileErrorT):
  String = {
    val errorStrBody =
      err match {
        case TypingPassDefiningError(range, inner) => {
          humanizeDefiningError(verbose, codeMap, linesBetween, lineRangeContaining, lineContaining, inner)
        }
        case TypingPassResolvingError(range, inner) => {
          humanizeResolvingError(verbose, codeMap, linesBetween, lineRangeContaining, lineContaining, inner)
        }
        case RangedInternalErrorT(range, message) => {
          "Internal error: " + message
        }
        case CouldntFindOverrideT(range, fff) => {
          "Couldn't find an override:\n" +
            humanizeFindFunctionFailure(verbose, codeMap, linesBetween, lineRangeContaining, lineContaining, range, fff)
        }
        case NewImmRSANeedsCallable(range) => {
          "To make an immutable runtime-sized array, need two params: capacity int, plus lambda to populate that many elements."
        }
        case CouldntSolveRuneTypesT(range, error) => {
          "Couldn't solve rune types:\n" +
            HigherTypingErrorHumanizer.humanizeRuneTypeSolveError(
              codeMap, linesBetween, lineRangeContaining, lineContaining, error)
        }
        case UnexpectedArrayElementType(range, expectedType, actualType) => {
          "Unexpected type for array element, tried to put a " + humanizeTemplata(codeMap, CoordTemplataT(actualType)) + " into an array of " + humanizeTemplata(codeMap, CoordTemplataT(expectedType))
        }
        case IndexedArrayWithNonInteger(range, tyype) => {
          "Indexed array with non-integer: " + humanizeTemplata(codeMap, CoordTemplataT(tyype))
        }
        case CantUseReadonlyReferenceAsReadwrite(range) => {
          "Can't make readonly reference into a readwrite one!"
        }
        case CouldntFindOverrideT(range, fff) => {
          "Couldn't find override: " + humanizeFindFunctionFailure(verbose, codeMap, linesBetween, lineRangeContaining, lineContaining, range, fff)
        }
        case CantReconcileBranchesResults(range, thenResult, elseResult) => {
          "If branches return different types: " + humanizeTemplata(codeMap, CoordTemplataT(thenResult)) + " and " + humanizeTemplata(codeMap, CoordTemplataT(elseResult))
        }
        case CantMoveOutOfMemberT(range, name) => {
          "Cannot move out of member (" + name + ")"
        }
        case CantMutateFinalMember(range, struct, memberName) => {
          "Cannot mutate final member '" + printableVarName(memberName) + "' of container " + humanizeTemplata(codeMap, KindTemplataT(struct))
        }
        case CantMutateFinalElement(range, coord) => {
          "Cannot change a slot in array " + humanizeTemplata(codeMap, CoordTemplataT(coord)) + " to point to a different element; it's an array of final references."
        }
        case LambdaReturnDoesntMatchInterfaceConstructor(range) => {
          "Argument function return type doesn't match interface method param"
        }
        case CantUseUnstackifiedLocal(range, name) => {
          "Can't use local that was already moved: " + humanizeName(codeMap, name)
        }
        case CantUnstackifyOutsideLocalFromInsideWhile(range, name) => {
          "Can't move a local (" + name + ") from inside a while loop."
        }
        case CannotSubscriptT(range, tyype) => {
          "Cannot subscript type: " + humanizeTemplata(codeMap, KindTemplataT(tyype)) + "!"
        }
        case CouldntConvertForReturnT(range, expectedType, actualType) => {
          "Couldn't convert " + humanizeTemplata(codeMap, CoordTemplataT(actualType)) + " to expected return type " + humanizeTemplata(codeMap, CoordTemplataT(expectedType))
        }
        case CouldntConvertForMutateT(range, expectedType, actualType) => {
          "Mutate couldn't convert " + actualType + " to expected destination type " + expectedType
        }
        case CouldntFindMemberT(range, memberName) => {
          "Couldn't find member " + memberName + "!"
        }
        case CouldntEvaluatImpl(range, eff) => {
          "Couldn't evaluate impl statement:\n" +
            humanizeCandidateAndFailedSolve(codeMap, linesBetween, lineRangeContaining, lineContaining, eff)
      }
        case BodyResultDoesntMatch(range, functionName, expectedReturnType, resultType) => {
          "Function " + printableName(codeMap, functionName) + " return type " + humanizeTemplata(codeMap, CoordTemplataT(expectedReturnType)) + " doesn't match body's result: " + humanizeTemplata(codeMap, CoordTemplataT(resultType))
        }
        case CouldntFindIdentifierToLoadT(range, name) => {
          "Couldn't find anything named `" + PostParserErrorHumanizer.humanizeImpreciseName(name) + "`!"
        }
        case NonReadonlyReferenceFoundInPureFunctionParameter(range, name) => {
          "Parameter `" + name + "` should be readonly, because it's in a pure function."
        }
        case CouldntFindTypeT(range, name) => {
          "Couldn't find any type named `" + name + "`!"
        }
        case CouldntNarrowDownCandidates(range, candidateRanges) => {
          "Multiple candidates for call:" +
            candidateRanges.map(range => "\n" + codeMap(range.begin) + ":\n  " + lineContaining(range.begin)).mkString("")
        }
        case ImmStructCantHaveVaryingMember(range, structName, memberName) => {
          "Immutable struct (\"" + printableName(codeMap, structName) + "\") cannot have varying member (\"" + memberName + "\")."
        }
        case ImmStructCantHaveMutableMember(range, structName, memberName) => {
          "Immutable struct (\"" + printableName(codeMap, structName) + "\") cannot have mutable member (\"" + memberName + "\")."
        }
        case WrongNumberOfDestructuresError(range, actualNum, expectedNum) => {
          "Wrong number of receivers; receiving " + actualNum + " but should be " + expectedNum + "."
        }
        case CantDowncastUnrelatedTypes(range, sourceKind, targetKind, candidates) => {
          "Can't downcast `" + humanizeTemplata(codeMap, KindTemplataT(sourceKind)) + "` to unrelated `" + humanizeTemplata(codeMap, KindTemplataT(targetKind)) + "`"
        }
        case CantDowncastToInterface(range, targetKind) => {
          "Can't downcast to an interface (" + targetKind + ") yet."
        }
        case ArrayElementsHaveDifferentTypes(range, types) => {
          "Array's elements have different types: " + types.mkString(", ")
        }
        case ExportedFunctionDependedOnNonExportedKind(range, paackage, signature, nonExportedKind) => {
          "Exported function:\n" + humanizeSignature(codeMap, signature) + "\ndepends on kind:\n" + humanizeTemplata(codeMap, KindTemplataT(nonExportedKind)) + "\nthat wasn't exported from package " + SourceCodeUtils.humanizePackage(paackage)
        }
        case TypeExportedMultipleTimes(range, paackage, exports) => {
          "Type exported multiple times:" + exports.map(export => {
            val posStr = codeMap(export.range.begin)
            val line = lineContaining(export.range.begin)
            s"\n  ${posStr}: ${line}"
          })
        }
        case ExternFunctionDependedOnNonExportedKind(range, paackage, signature, nonExportedKind) => {
          "Extern function " + signature + " depends on kind " + nonExportedKind + " that wasn't exported from package " + paackage
        }
        case ExportedImmutableKindDependedOnNonExportedKind(range, paackage, signature, nonExportedKind) => {
          "Exported kind " + signature + " depends on kind " + nonExportedKind + " that wasn't exported from package " + paackage
        }
        case InitializedWrongNumberOfElements(range, expectedNumElements, numElementsInitialized) => {
          "Supplied " + numElementsInitialized + " elements, but expected " + expectedNumElements + "."
        }
        case CouldntFindFunctionToCallT(range, fff) => {
          humanizeFindFunctionFailure(verbose, codeMap, linesBetween, lineRangeContaining, lineContaining, range, fff)
        }
        case CouldntEvaluateFunction(range, eff) => {
          "Couldn't evaluate function:\n" +
          humanizeDefiningError(
            verbose, codeMap, linesBetween, lineRangeContaining, lineContaining, eff)
        }
        case FunctionAlreadyExists(oldFunctionRange, newFunctionRange, signature) => {
          "Function " + humanizeId(codeMap, signature) + " already exists! Previous declaration at:\n" +
            codeMap(oldFunctionRange.begin)
        }
        case AbstractMethodOutsideOpenInterface(range) => {
          "Open (non-sealed) interfaces can't have abstract methods defined outside the interface."
        }
        case IfConditionIsntBoolean(range, actualType) => {
          "If condition should be a bool, but was: " + actualType
        }
        case WhileConditionIsntBoolean(range, actualType) => {
          "If condition should be a bool, but was: " + actualType
        }
        case CantImplNonInterface(range, struct) => {
          "Can't extend a non-interface: " + struct
        }
        case TypingPassSolverError(range, failedSolve) => {
          val (text, lineBegins) =
            SolverErrorHumanizer.humanizeFailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError](
              codeMap,
              linesBetween,
              lineRangeContaining,
              lineContaining,
              humanizeRune,
              t => humanizeTemplata(codeMap, t),
              err => humanizeRuleError(codeMap, linesBetween, lineRangeContaining, lineContaining, err),
              (rule: IRulexSR) => rule.range,
              (rule: IRulexSR) => rule.runeUsages.map(usage => (usage.rune, usage.range)),
              (rule: IRulexSR) => rule.runeUsages.map(_.rune),
              PostParserErrorHumanizer.humanizeRule,
              failedSolve)
          text
        }
        case HigherTypingInferError(range, err) => {
          HigherTypingErrorHumanizer.humanizeRuneTypeSolveError(
            codeMap, linesBetween, lineRangeContaining, lineContaining, err)
        }
      }

//    val errorId = "T"
    err.range.reverse.map(range => {
      val posStr = codeMap(range.begin)
      val lineContents = lineContaining(range.begin)
      f"At ${posStr}:\n${lineContents}\n"
    }).mkString("") +
    errorStrBody + "\n"
  }
*/
pub fn humanize_defining_error<'s, 't>(verbose: bool, code_map: &dyn Fn(CodeLocationS) -> String, lines_between: &dyn Fn(CodeLocationS, CodeLocationS) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS) -> String, err: IDefiningError<'s, 't>) -> String {
  panic!("Unimplemented: humanize_defining_error");
}
/*
  def humanizeDefiningError(
      verbose: Boolean,
      codeMap: CodeLocationS => String,
      linesBetween: (CodeLocationS, CodeLocationS) => Vector[RangeS],
      lineRangeContaining: (CodeLocationS) => RangeS,
      lineContaining: (CodeLocationS) => String,
      err: IDefiningError):
  String = {
    err match {
      case DefiningResolveConclusionError(inner) => {
        humanizeConclusionResolveError(verbose, codeMap, linesBetween, lineRangeContaining, lineContaining, inner)
      }
      case DefiningSolveFailedOrIncomplete(inner) => {
        humanizeFailedSolve(verbose, codeMap, linesBetween, lineRangeContaining, lineContaining, inner)
      }
    }
  }
*/
pub fn humanize_resolve_failure<'s, 't>(verbose: bool, code_map: &dyn Fn(CodeLocationS) -> String, lines_between: &dyn Fn(CodeLocationS, CodeLocationS) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS) -> String, fff: ResolveFailure<'s, 't, KindT<'s, 't>>) -> String {
  panic!("Unimplemented: humanize_resolve_failure");
}
/*
  def humanizeResolveFailure(
    verbose: Boolean,
    codeMap: CodeLocationS => String,
    linesBetween: (CodeLocationS, CodeLocationS) => Vector[RangeS],
    lineRangeContaining: (CodeLocationS) => RangeS,
    lineContaining: (CodeLocationS) => String,
    fff: ResolveFailure[KindT]):
  String = {
    val ResolveFailure(range, reason) = fff
    humanizeResolvingError(verbose, codeMap, linesBetween, lineRangeContaining, lineContaining, reason)
  }
*/
pub fn humanize_resolving_error<'s, 't>(verbose: bool, code_map: &dyn Fn(CodeLocationS) -> String, lines_between: &dyn Fn(CodeLocationS, CodeLocationS) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS) -> String, error: IResolvingError<'s, 't>) -> String {
  panic!("Unimplemented: humanize_resolving_error");
}
/*
  def humanizeResolvingError(
      verbose: Boolean,
      codeMap: CodeLocationS => String,
      linesBetween: (CodeLocationS, CodeLocationS) => Vector[RangeS],
      lineRangeContaining: (CodeLocationS) => RangeS,
      lineContaining: (CodeLocationS) => String,
      error: IResolvingError):
  String = {
    error match {
      case ResolvingResolveConclusionError(inner) => {
        humanizeConclusionResolveError(verbose, codeMap, linesBetween, lineRangeContaining, lineContaining, inner)
      }
      case ResolvingSolveFailedOrIncomplete(inner) => {
        humanizeFailedSolve(verbose, codeMap, linesBetween, lineRangeContaining, lineContaining, inner)
      }
      case other => vimpl(other)
    }
  }
*/
pub fn humanize_failed_solve<'s, 't>(verbose: bool, code_map: &dyn Fn(CodeLocationS) -> String, lines_between: &dyn Fn(CodeLocationS, CodeLocationS) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS) -> String, error: FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>) -> String {
  panic!("Unimplemented: humanize_failed_solve");
}
/*
  def humanizeFailedSolve(
      verbose: Boolean,
      codeMap: CodeLocationS => String,
      linesBetween: (CodeLocationS, CodeLocationS) => Vector[RangeS],
      lineRangeContaining: (CodeLocationS) => RangeS,
      lineContaining: (CodeLocationS) => String,
      error: FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]):
  String = {
    humanizeCandidateAndFailedSolve(codeMap, linesBetween, lineRangeContaining, lineContaining, error)
  }
*/
pub fn humanize_conclusion_resolve_error<'s, 't>(verbose: bool, code_map: &dyn Fn(CodeLocationS) -> String, lines_between: &dyn Fn(CodeLocationS, CodeLocationS) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS) -> String, error: IConclusionResolveError<'s, 't>) -> String {
  panic!("Unimplemented: humanize_conclusion_resolve_error");
}
/*
  def humanizeConclusionResolveError(
      verbose: Boolean,
      codeMap: CodeLocationS => String,
      linesBetween: (CodeLocationS, CodeLocationS) => Vector[RangeS],
      lineRangeContaining: (CodeLocationS) => RangeS,
      lineContaining: (CodeLocationS) => String,
      error: IConclusionResolveError):
  String = {
    error match {
      case CouldntFindKindForConclusionResolve(inner) => {
        humanizeResolveFailure(verbose, codeMap, linesBetween, lineRangeContaining, lineContaining, inner)
      }
      case CouldntFindFunctionForConclusionResolve(range, inner) => {
        humanizeFindFunctionFailure(verbose, codeMap, linesBetween, lineRangeContaining, lineContaining, range, inner)
      }
      case ReturnTypeConflictInConclusionResolve(range, expectedReturnType, actualPrototype) => {
        "Found function: " + humanizeId(codeMap, actualPrototype.id) + " which returns " + humanizeTemplata(codeMap, CoordTemplataT(actualPrototype.returnType)) + " but expected return type of " + humanizeTemplata(codeMap, CoordTemplataT(expectedReturnType))
      }
      case other => vimpl(other)
    }
  }
*/
pub fn humanize_find_function_failure<'s, 't>(verbose: bool, code_map: &dyn Fn(CodeLocationS) -> String, lines_between: &dyn Fn(CodeLocationS, CodeLocationS) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS) -> String, invocation_range: Vec<RangeS<'s>>, fff: FindFunctionFailure<'s, 't>) -> String {
  panic!("Unimplemented: humanize_find_function_failure");
}
/*
  def humanizeFindFunctionFailure(
    verbose: Boolean,
    codeMap: CodeLocationS => String,
    linesBetween: (CodeLocationS, CodeLocationS) => Vector[RangeS],
    lineRangeContaining: (CodeLocationS) => RangeS,
    lineContaining: (CodeLocationS) => String,
    invocationRange: List[RangeS],
    fff: OverloadResolver.FindFunctionFailure): String = {

    val FindFunctionFailure(name, args, rejectedCalleeToReason) = fff
    "Couldn't find a suitable function " +
      PostParserErrorHumanizer.humanizeImpreciseName(name) +
      "(" +
      args.map({
        case tyype => humanizeTemplata(codeMap, CoordTemplataT(tyype))
      }).mkString(", ") +
      "). " +
      (if (rejectedCalleeToReason.isEmpty) {
        "No function with that name exists.\n"
      } else {
        "Rejected candidates:\n\n" +
        rejectedCalleeToReason.zipWithIndex.map({ case ((candidate, reason), index) =>
          "Candidate " + (index + 1) + " (of " + rejectedCalleeToReason.size + "): " +
            humanizeCandidate(codeMap, lineRangeContaining, candidate) +
            humanizeRejectionReason(verbose, codeMap, linesBetween, lineRangeContaining, lineContaining, invocationRange, reason) + "\n\n"
        }).mkString("")
      })
  }
*/
pub fn humanize_banner(code_map: &dyn Fn(CodeLocationS) -> String, banner: FunctionBannerT) -> String {
  panic!("Unimplemented: humanize_banner");
}
/*
  def humanizeBanner(
    codeMap: CodeLocationS => String,
    banner: FunctionBannerT):
  String = {
    banner.originFunctionTemplata match {
      case None => "(internal)"
      case Some(x) => printableName(codeMap, x.function.name)
    }
  }
*/
fn printable_name(code_map: &dyn Fn(CodeLocationS) -> String, name: INameS) -> String {
  panic!("Unimplemented: printable_name");
}
/*
  private def printableName(
    codeMap: CodeLocationS => String,
    name: INameS):
  String = {
    name match {
      case CodeVarNameS(name) => name.str
      case TopLevelCitizenDeclarationNameS(name, codeLocation) => name.str
      case LambdaDeclarationNameS(codeLocation) => codeMap(codeLocation) + ": " + "(lambda)"
      case FunctionNameS(name, codeLocation) => codeMap(codeLocation) + ": " + name.str
      case ConstructorNameS(TopLevelCitizenDeclarationNameS(name, range)) => codeMap(range.begin) + ": " + name.str
      case ImmConcreteDestructorNameS(_) => vimpl()
      case ImmInterfaceDestructorNameS(_) => vimpl()
//      case DropNameS(_) => vimpl()
    }
  }
*/
fn printable_kind_name(kind: KindT) -> String {
  panic!("Unimplemented: printable_kind_name");
}
/*
  private def printableKindName(kind: KindT): String = {
    kind match {
      case IntT(bits) => "i" + bits
      case BoolT() => "bool"
      case FloatT() => "float"
      case StrT() => "str"
      case StructTT(f) => printableId(f)
    }
  }
*/
fn printable_id<'s, 't>(id: IdT<'s, 't>) -> String {
  panic!("Unimplemented: printable_id");
}
/*
  private def printableId(id: IdT[INameT]): String = {
    id.localName match {
      case CitizenNameT(humanName, templateArgs) => humanName + (if (templateArgs.isEmpty) "" else "<" + templateArgs.map(_.toString.mkString) + ">")
      case x => x.toString
    }
  }
*/
fn printable_var_name(name: IVarNameT) -> String {
  panic!("Unimplemented: printable_var_name");
}
/*
  private def printableVarName(
    name: IVarNameT):
  String = {
    name match {
      case CodeVarNameT(n) => n.str
    }
  }
*/
fn get_file(function_a: FunctionA) -> FileCoordinate {
  panic!("Unimplemented: get_file");
}
/*
  private def getFile(functionA: FunctionA): FileCoordinate = {
    functionA.range.file
  }
*/
fn humanize_rejection_reason<'s, 't>(verbose: bool, code_map: &dyn Fn(CodeLocationS) -> String, lines_between: &dyn Fn(CodeLocationS, CodeLocationS) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS) -> String, invocation_range: Vec<RangeS<'s>>, reason: IFindFunctionFailureReason<'s, 't>) -> String {
  panic!("Unimplemented: humanize_rejection_reason");
}
/*
  private def humanizeRejectionReason(
      verbose: Boolean,
    codeMap: CodeLocationS => String,
    linesBetween: (CodeLocationS, CodeLocationS) => Vector[RangeS],
    lineRangeContaining: (CodeLocationS) => RangeS,
    lineContaining: (CodeLocationS) => String,
      invocationrange: List[RangeS],
      reason: IFindFunctionFailureReason): String = {

    (reason match {
      case FindFunctionResolveFailure(reason) => {
        humanizeResolvingError(verbose, codeMap, linesBetween, lineRangeContaining, lineContaining, reason)
      }
      case RuleTypeSolveFailure(RuneTypeSolveError(range, failedSolve)) => {
        SolverErrorHumanizer.humanizeFailedSolve(
          codeMap,
          linesBetween,
          lineRangeContaining,
          lineContaining,
          humanizeRune,
          (a: ITemplataType) => humanizeTemplataType(a),
          (a: IRuneTypeRuleError) => PostParserErrorHumanizer.humanizeRuneTypeError(codeMap, a),
          (rule: IRulexSR) => rule.range,
          (rule: IRulexSR) => rule.runeUsages.map(usage => (usage.rune, usage.range)),
          (rule: IRulexSR) => rule.runeUsages.map(_.rune),
          PostParserErrorHumanizer.humanizeRule,
          failedSolve)._1
      }
      case WrongNumberOfArguments(supplied, expected) => {
        "Number of params doesn't match! Supplied " + supplied + " but function takes " + expected
      }
      case WrongNumberOfTemplateArguments(supplied, expected) => {
        "Number of template params doesn't match! Supplied " + supplied + " but function takes " + expected
      }
      case SpecificParamDoesntMatchExactly(index, arg, param) => {
          "Index " + index + " argument " + humanizeTemplata(codeMap, CoordTemplataT(arg)) +
          " isn't the same exact type as expected parameter " + humanizeTemplata(codeMap, CoordTemplataT(param))
      }
      case SpecificParamDoesntSend(index, arg, param) => {
          " Index " + index + " argument " + humanizeTemplata(codeMap, CoordTemplataT(arg)) +
          " can't be given to expected parameter " + humanizeTemplata(codeMap, CoordTemplataT(param))
      }
      case SpecificParamRegionDoesntMatch(rune, suppliedMutable, expectedMutable) => {
        " Generic param " + humanizeRune(rune) + " expected a " + expectedMutable + " region, but received a " + suppliedMutable + " region."
      }
      case SpecificParamVirtualityDoesntMatch(index) => {
        "Virtualities don't match at index " + index
      }
//      case Outscored() => "Outscored!"
      case InferFailure(reason) => {
        humanizeCandidateAndFailedSolve(codeMap, linesBetween, lineRangeContaining, lineContaining, reason)
      }
    })
  }
*/
pub fn humanize_rule_error<'s, 't>(code_map: &dyn Fn(CodeLocationS) -> String, lines_between: &dyn Fn(CodeLocationS, CodeLocationS) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS) -> String, error: ITypingPassSolverError<'s, 't>) -> String {
  panic!("Unimplemented: humanize_rule_error");
}
/*
  def humanizeRuleError(
    codeMap: CodeLocationS => String,
    linesBetween: (CodeLocationS, CodeLocationS) => Vector[RangeS],
    lineRangeContaining: (CodeLocationS) => RangeS,
    lineContaining: (CodeLocationS) => String,
    error: ITypingPassSolverError
  ): String = {
    error match {
      case IsaFailed(sub, suuper) => {
        "Kind " + humanizeTemplata(codeMap, KindTemplataT(sub)) + " does not implement interface " + humanizeTemplata(codeMap, KindTemplataT(suuper))
      }
      case BadIsaSubKind(kind) => {
        "Kind " + humanizeTemplata(codeMap, KindTemplataT(kind)) + " cannot be a sub-kind."
      }
      case CantGetComponentsOfPlaceholderPrototype(_) => {
        "Can't get components of placeholder."
      }
      case ReturnTypeConflict(_, expectedReturnType, actualPrototype) => {
        "Found function: " + humanizeId(codeMap, actualPrototype.id) + " which returns " + humanizeTemplata(codeMap, CoordTemplataT(actualPrototype.returnType)) + " but expected return type of " + humanizeTemplata(codeMap, CoordTemplataT(expectedReturnType))
      }
      case CantShareMutable(kind) => {
        "Can't share a mutable kind: " + humanizeTemplata(codeMap, KindTemplataT(kind))
      }
      case BadIsaSuperKind(kind) => {
        "Bad super kind in isa: " + humanizeTemplata(codeMap, KindTemplataT(kind))
      }
      case SendingNonIdenticalKinds(sendCoord, receiveCoord) => {
        "Sending non-identical kinds: " + humanizeTemplata(codeMap, CoordTemplataT(sendCoord)) + " and " + humanizeTemplata(codeMap, CoordTemplataT(receiveCoord))
      }
      case SendingNonCitizen(kind) => {
        "Sending non-struct non-interface Kind: " + humanizeTemplata(codeMap, KindTemplataT(kind))
      }
      case CantCheckPlaceholder(range) => {
        "Cant check a placeholder!"
      }
      case CouldntFindFunction(range, fff) => {
        "Couldn't find function to call: " +
          humanizeFindFunctionFailure(
            false, codeMap, linesBetween, lineRangeContaining, lineContaining, range, fff)
      }
      case CouldntResolveKind(rf) => {
        "Couldn't find type: " + humanizeResolveFailure(false, codeMap, linesBetween, lineRangeContaining, lineContaining, rf)
      }
      case WrongNumberOfTemplateArgs(expectedMinNumArgs, expectedMaxNumArgs) => {
        if (expectedMinNumArgs == expectedMaxNumArgs) {
          "Wrong number of template args, expected " + expectedMinNumArgs + "."
        } else {
          "Wrong number of template args, expected " + expectedMinNumArgs + " or " + expectedMaxNumArgs + "."
        }
      }
      case LookupFailed(name) => "Couldn't find anything named: " + humanizeImpreciseName(name)
      case KindIsNotConcrete(kind) => {
        "Expected kind to be concrete, but was not. Kind: " + humanizeKind(codeMap, kind)
      }
      case OneOfFailed(rule) => {
        "One-of rule failed."
      }
      case KindIsNotInterface(kind) => {
        "Expected kind to be interface, but was not. Kind: " + humanizeKind(codeMap, kind)
      }
      case CallResultIsntCallable(result) => {
        "Generic call result isn't callable: " + humanizeTemplata(codeMap, result)
      }
      case CallResultWasntExpectedType(expected, actual) => {
        "Expected an instantiation of " + humanizeTemplata(codeMap, expected) + " but got " + humanizeTemplata(codeMap, actual)
      }
      case OwnershipDidntMatch(coord, expectedOwnership) => {
        "Given type " + humanizeTemplata(codeMap, CoordTemplataT(coord)) + " doesn't have expected ownership " + humanizeOwnership(Conversions.unevaluateOwnership(expectedOwnership))
      }
      case ReceivingDifferentOwnerships(params) => {
        "Received conflicting ownerships: " +
          params.map({ case (rune, coord) =>
            humanizeRune(rune) + " = " + humanizeTemplata(codeMap, CoordTemplataT(coord))
          }).mkString(", ")
      }
      case NoAncestorsSatisfyCall(params) => {
        "No ancestors satisfy call: " +
          params.map({ case (rune, coord) =>
            humanizeRune(rune) + " = " + humanizeTemplata(codeMap, CoordTemplataT(coord))
          }).mkString(", ")
      }
    }
  }
*/
pub fn humanize_candidate_and_failed_solve<'s, 't>(code_map: &dyn Fn(CodeLocationS) -> String, lines_between: &dyn Fn(CodeLocationS, CodeLocationS) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS) -> String, result: FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>) -> String {
  panic!("Unimplemented: humanize_candidate_and_failed_solve");
}
/*
  def humanizeCandidateAndFailedSolve(
    codeMap: CodeLocationS => String,
    linesBetween: (CodeLocationS, CodeLocationS) => Vector[RangeS],
    lineRangeContaining: (CodeLocationS) => RangeS,
    lineContaining: (CodeLocationS) => String,
    result: FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]):
  String = {
    val (text, lineBegins) =
      SolverErrorHumanizer.humanizeFailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError](
        codeMap,
        linesBetween,
        lineRangeContaining,
        lineContaining,
        humanizeRune,
        t => humanizeTemplata(codeMap, t),
        err => humanizeRuleError(codeMap, linesBetween, lineRangeContaining, lineContaining, err),
        (rule: IRulexSR) => rule.range,
        (rule: IRulexSR) => rule.runeUsages.map(usage => (usage.rune, usage.range)),
        (rule: IRulexSR) => rule.runeUsages.map(_.rune),
        PostParserErrorHumanizer.humanizeRule,
        result)
    text
  }
*/
pub fn humanize_candidate(code_map: &dyn Fn(CodeLocationS) -> String, line_range_containing: &dyn Fn(CodeLocationS) -> RangeS, candidate: ICalleeCandidate) -> String {
  panic!("Unimplemented: humanize_candidate");
}
/*
  def humanizeCandidate(
    codeMap: CodeLocationS => String,
    lineRangeContaining: (CodeLocationS) => RangeS,
    candidate: ICalleeCandidate) = {
    candidate match {
      case HeaderCalleeCandidate(header) => {
        humanizeId(codeMap, header.id)
      }
      case PrototypeTemplataCalleeCandidate(prototypeT) => {
////        vimpl() // Need a good test case that shows this is even possible.
//        val begin = vimpl()//lineRangeContaining(range.begin).begin
//        codeMap(begin) + ":\n" +
//          lineRangeContaining(begin).begin + "\n"
        prototypeT.id.localName + ":\n"
      }
      case FunctionCalleeCandidate(ft) => {
        val begin = lineRangeContaining(ft.function.range.begin).begin
        codeMap(begin) + ":\n" +
          lineRangeContaining(begin).begin + "\n"
      }
    }
  }
*/
pub fn humanize_templata<'s, 't>(code_map: &dyn Fn(CodeLocationS) -> String, templata: ITemplataT<'s, 't>) -> String {
  panic!("Unimplemented: humanize_templata");
}
/*
  def humanizeTemplata(
    codeMap: CodeLocationS => String,
    templata: ITemplataT[ITemplataType]):
  String = {
    templata match {
      case RuntimeSizedArrayTemplateTemplataT() => "Array"
      case StaticSizedArrayTemplateTemplataT() => "StaticArray"
      case InterfaceDefinitionTemplataT(env, originInterface) => originInterface.name.name.str
      case StructDefinitionTemplataT(env, originStruct) => PostParserErrorHumanizer.humanizeName(originStruct.name)
      case VariabilityTemplataT(variability) => {
        variability match {
          case FinalT => "final"
          case VaryingT => "vary"
        }
      }
      case IntegerTemplataT(value) => value.toString
      case MutabilityTemplataT(mutability) => {
        mutability match {
          case MutableT => "mut"
          case ImmutableT => "imm"
        }
      }
      case OwnershipTemplataT(ownership) => {
        ownership match {
          case OwnT => "own"
          case BorrowT => "borrow"
          case WeakT => "weak"
          case ShareT => "share"
        }
      }
      case PrototypeTemplataT(prototype) => {
        humanizeId(codeMap, prototype.id)
      }
      case CoordTemplataT(coord) => {
        humanizeCoord(codeMap, coord)
      }
      case KindTemplataT(kind) => {
        humanizeKind(codeMap, kind)
      }
      case CoordListTemplataT(coords) => {
        "(" + coords.map(CoordTemplataT).map(humanizeTemplata(codeMap, _)).mkString(", ") + ")"
      }
      case StringTemplataT(value) => "\"" + value + "\""
      case PlaceholderTemplataT(id, tyype) => {
        tyype match {
          case CoordTemplataType() => "$" + humanizeId(codeMap, id)
          case _ => humanizeTemplataType(tyype) + "$" + humanizeId(codeMap, id)
        }
      }
      case other => vimpl(other)
    }
  }
*/
fn humanize_coord(code_map: &dyn Fn(CodeLocationS) -> String, coord: CoordT) -> String {
  panic!("Unimplemented: humanize_coord");
}
/*
  private def humanizeCoord(
      codeMap: CodeLocationS => String,
      coord: CoordT
  ) = {
    val CoordT(ownership, region, kind) = coord

    val ownershipStr =
      ownership match {
        case OwnT => ""
        case ShareT => ""
        case BorrowT => "&"
        case WeakT => "&&"
      }
    val kindStr = humanizeKind(codeMap, kind, Some(region))
    ownershipStr + kindStr
  }
*/
fn humanize_kind(code_map: &dyn Fn(CodeLocationS) -> String, kind: KindT, containing_region: Option<RegionT>) -> String {
  panic!("Unimplemented: humanize_kind");
}
/*
  private def humanizeKind(
      codeMap: CodeLocationS => String,
      kind: KindT,
      containingRegion: Option[RegionT] = None
  ) = {
    kind match {
      case IntT(bits) => "i" + bits
      case BoolT() => "bool"
      case KindPlaceholderT(name) => "Kind$" + humanizeId(codeMap, name)
      case StrT() => "str"
      case NeverT(_) => "never"
      case VoidT() => "void"
      case FloatT() => "float"
      case OverloadSetT(_, name) => {
        "(overloads: " +
            PostParserErrorHumanizer.humanizeImpreciseName(name) +
            ")"
      }
      case InterfaceTT(name) => humanizeId(codeMap, name, containingRegion)
      case StructTT(name) => humanizeId(codeMap, name, containingRegion)
      case contentsRuntimeSizedArrayTT(mutability, elementType, region) => {
        "Array<" +
            humanizeTemplata(codeMap, mutability) + ", " +
            humanizeTemplata(codeMap, CoordTemplataT(elementType)) +
            ">"
      }
      case contentsStaticSizedArrayTT(size, mutability, variability, elementType, region) => {
        //        humanizeTemplata(codeMap, region) + "'" +
        "StaticArray<" +
            humanizeTemplata(codeMap, size) + ", " +
            humanizeTemplata(codeMap, mutability) + ", " +
            humanizeTemplata(codeMap, variability) + ", " +
            humanizeTemplata(codeMap, CoordTemplataT(elementType)) +
            ">"
      }
    }
  }
*/
pub fn humanize_id<'s, 't, T: Copy + 't>(code_map: &dyn Fn(CodeLocationS) -> String, name: IdT<'s, 't>, containing_region: Option<RegionT>) -> String
where 's: 't,
{
  panic!("Unimplemented: humanize_id");
}
/*
  def humanizeId[T <: INameT](
    codeMap: CodeLocationS => String,
    name: IdT[T],
    containingRegion: Option[RegionT] = None):
  String = {
    (if (name.initSteps.nonEmpty) {
      name.initSteps.map(n => humanizeName(codeMap, n)).mkString(".") + "."
    } else {
      ""
    }) +
      humanizeName(codeMap, name.localName, containingRegion)
  }
*/
pub fn humanize_name(code_map: &dyn Fn(CodeLocationS) -> String, name: INameT, containing_region: Option<RegionT>) -> String {
  panic!("Unimplemented: humanize_name");
}
/*
  def humanizeName(
    codeMap: CodeLocationS => String,
    name: INameT,
    containingRegion: Option[RegionT] = None):
  String = {
    name match {
      case AnonymousSubstructConstructorNameT(template, templateArgs, parameters) => {
        humanizeName(codeMap, template) +
          "<" + templateArgs.map(humanizeTemplata(codeMap, _)).mkString(", ") + ">" +
          "(" + parameters.map(CoordTemplataT).map(humanizeTemplata(codeMap, _)).mkString(", ") + ")"
      }
      case AnonymousSubstructConstructorTemplateNameT(substruct) => {
        "asc:" + humanizeName(codeMap, substruct)
      }
      case SelfNameT() => "self"
      case OverrideDispatcherTemplateNameT(implId) => "ovdt:" + humanizeId(codeMap, implId)
      case OverrideDispatcherNameT(OverrideDispatcherTemplateNameT(implId), templateArgs, parameters) => {
        "ovd:" + humanizeId(codeMap, implId) +
        humanizeGenericArgs(codeMap, templateArgs, None) +
            "(" + parameters.map(CoordTemplataT).map(humanizeTemplata(codeMap, _)).mkString(", ") + ")"
      }
      case IteratorNameT(range) => "it:" + codeMap(range.begin)
      case IterableNameT(range) => "ib:" + codeMap(range.begin)
      case IterationOptionNameT(range) => "io:" + codeMap(range.begin)
      case ImplTemplateNameT(codeLoc) => "implt:" + codeMap(codeLoc)
      case ForwarderFunctionNameT(_, inner) => humanizeName(codeMap, inner)
      case ForwarderFunctionTemplateNameT(inner, index) => "fwd" + index + ":" + humanizeName(codeMap, inner)
      case MagicParamNameT(codeLoc) => "mp:" + codeMap(codeLoc)
      case ClosureParamNameT(codeLocation) => "λP:" + codeMap(codeLocation)
      case ConstructingMemberNameT(name) => "cm:" + name
      case TypingPassBlockResultVarNameT(life) => "b:" + life
      case TypingPassFunctionResultVarNameT() => "(result)"
      case TypingPassTemporaryVarNameT(life) => "t:" + life
      case FunctionBoundTemplateNameT(humanName) => humanName.str
      case LambdaCallFunctionTemplateNameT(codeLocation, _) => "λF:" + codeMap(codeLocation)
      case LambdaCitizenTemplateNameT(codeLocation) => "λC:" + codeMap(codeLocation)
      case LambdaCallFunctionNameT(template, templateArgs, parameters) => {
        humanizeName(codeMap, template) +
          humanizeGenericArgs(codeMap, templateArgs, None) +
          "(" + parameters.map(CoordTemplataT).map(humanizeTemplata(codeMap, _)).mkString(", ") + ")"
      }
      case FunctionBoundNameT(template, templateArgs, parameters) => {
        humanizeName(codeMap, template) +
          humanizeGenericArgs(codeMap, templateArgs, None) +
          "(" + parameters.map(CoordTemplataT).map(humanizeTemplata(codeMap, _)).mkString(", ") + ")"
      }
      case KindPlaceholderNameT(template) => humanizeName(codeMap, template)
      case KindPlaceholderTemplateNameT(index, rune) => humanizeRune(rune)
      case CodeVarNameT(name) => name.str
      case LambdaCitizenNameT(template) => humanizeName(codeMap, template) + "<>"
      case FunctionTemplateNameT(humanName, codeLoc) => humanName.str
      case ExternFunctionNameT(humanName, parameters) => humanName.str
      case FunctionNameT(templateName, templateArgs, parameters) => {
        humanizeName(codeMap, templateName) +
          humanizeGenericArgs(codeMap, templateArgs, containingRegion) +
          (if (parameters.nonEmpty) {
            "(" + parameters.map(CoordTemplataT).map(humanizeTemplata(codeMap, _)).mkString(", ") + ")"
          } else {
            ""
          })
      }
      case CitizenNameT(humanName, templateArgs) => {
        humanizeName(codeMap, humanName) +
          humanizeGenericArgs(codeMap, templateArgs, containingRegion)
      }
      case RuntimeSizedArrayNameT(RuntimeSizedArrayTemplateNameT(), RawArrayNameT(mutability, elementType, region)) => {
        "[]<" +
          (mutability match {
            case MutabilityTemplataT(ImmutableT) => "imm"
            case MutabilityTemplataT(MutableT) => "mut"
            case other => humanizeTemplata(codeMap, other)
          }) + ", " +
          humanizeTemplata(codeMap, CoordTemplataT(elementType))
      }
      case StaticSizedArrayNameT(StaticSizedArrayTemplateNameT(), size, variability, RawArrayNameT(mutability, elementType, region)) => {
        "[]<" +
          humanizeTemplata(codeMap, size) + ", "
          humanizeTemplata(codeMap, mutability) + ", "
          humanizeTemplata(codeMap, variability) + ">"
          humanizeTemplata(codeMap, CoordTemplataT(elementType))
      }
      case AnonymousSubstructNameT(interface, templateArgs) => {
        humanizeName(codeMap, interface) +
          "<" + templateArgs.map(humanizeTemplata(codeMap, _)).mkString(", ") + ">"
      }
      case AnonymousSubstructTemplateNameT(interface) => {
        humanizeName(codeMap, interface) + ".anonymous"
      }
      case StructTemplateNameT(humanName) => humanName.str
      case InterfaceTemplateNameT(humanName) => humanName.str
      case NonKindNonRegionPlaceholderNameT(index, rune) => humanizeRune(rune)
    }
  }
*/
fn humanize_generic_args<'s, 't>(code_map: &dyn Fn(CodeLocationS) -> String, template_args: Vec<ITemplataT<'s, 't>>, containing_region: Option<RegionT>) -> String {
  panic!("Unimplemented: humanize_generic_args");
}
/*
  private def humanizeGenericArgs(
    codeMap: CodeLocationS => String,
    templateArgs: Vector[ITemplataT[ITemplataType]],
    containingRegion: Option[RegionT]
  ) = {
    (
      if (templateArgs.nonEmpty) {
        "<" +
          (templateArgs.init.map(humanizeTemplata(codeMap, _)) ++
              templateArgs.lastOption.map(region => {
                containingRegion match {
                  case None => humanizeTemplata(codeMap, region)
                  case Some(r) => "_"
                }
              })).mkString(", ") +
          ">"
      } else {
        ""
      })
  }
*/
pub fn humanize_signature(code_map: &dyn Fn(CodeLocationS) -> String, signature: SignatureT) -> String {
  panic!("Unimplemented: humanize_signature");
}
/*
  def humanizeSignature(codeMap: CodeLocationS => String, signature: SignatureT): String = {
    humanizeId(codeMap, signature.id)
  }
}
*/
