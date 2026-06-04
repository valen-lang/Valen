use crate::utils::range::{RangeS, CodeLocationS};
use crate::interner::Interner;
use crate::postparsing::*;
use crate::postparsing::names::*;
use crate::scout_arena::ScoutArena;
use crate::typing::typing_interner::TypingInterner;
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
use crate::typing::types::types::OwnershipT;

/*
package dev.vale.typing

import dev.vale._
import dev.vale.postparsing._
import dev.vale.postparsing.rules.IRulexSR
import dev.vale.solver.{FailedSolve, RuleError, SolveIncomplete, SolverConflict, SolverErrorHumanizer}
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
// Mirror of canonical Scala CompilerErrorHumanizer.humanize. Per MACTX, several canonical
// arms (CantUseRuneValueAsExpression, KindIsNotStruct, CouldntFindImpl, CantSharePlaceholder,
// NoCommonAncestors, CantDetermineNarrowestKind, FunctionDoesntHaveName, InternalSolverError)
// are present in the audit-trail above but their Rust ICompileErrorT variants haven't been
// added yet — they fall through to the catch-all `_ => panic!` below until ported.
pub fn humanize<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, verbose: bool, code_map: &dyn Fn(CodeLocationS<'s>) -> String, lines_between: &dyn Fn(CodeLocationS<'s>, CodeLocationS<'s>) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS<'s>) -> String, err: ICompileErrorT<'s, 't>) -> String {
  let error_str_body = match &err {
    ICompileErrorT::TypingPassDefiningError { range: _, inner } => {
      humanize_defining_error(scout_arena, typing_interner, verbose, code_map, lines_between, line_range_containing, line_containing, inner)
    }
    ICompileErrorT::TypingPassResolvingError { range: _, inner } => {
      humanize_resolving_error(scout_arena, typing_interner, verbose, code_map, lines_between, line_range_containing, line_containing, inner)
    }
    ICompileErrorT::RangedInternalErrorT { range: _, message } => {
      format!("Internal error: {}", message)
    }
    ICompileErrorT::CouldntFindOverrideT { range, fff } => {
      format!("Couldn't find an override:\n{}",
        humanize_find_function_failure(scout_arena, typing_interner, verbose, code_map, lines_between, line_range_containing, line_containing, range.to_vec(), fff))
    }
    ICompileErrorT::NewImmRSANeedsCallable { range: _ } => {
      "To make an immutable runtime-sized array, need two params: capacity int, plus lambda to populate that many elements.".to_string()
    }
    ICompileErrorT::CouldntSolveRuneTypesT { range: _, error: _ } => {
      panic!("implement: humanize CouldntSolveRuneTypesT")
    }
    ICompileErrorT::UnexpectedArrayElementType { range: _, expected_type: _, actual_type: _ } => {
      panic!("implement: humanize UnexpectedArrayElementType")
    }
    ICompileErrorT::IndexedArrayWithNonInteger { range: _, types: _ } => {
      panic!("implement: humanize IndexedArrayWithNonInteger")
    }
    ICompileErrorT::CantUseReadonlyReferenceAsReadwrite { range: _ } => {
      "Can't make readonly reference into a readwrite one!".to_string()
    }
    ICompileErrorT::CantReconcileBranchesResults { range: _, then_result, else_result } => {
      "If branches return different types: ".to_string()
        + &humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *then_result })))
        + " and "
        + &humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *else_result })))
    }
    ICompileErrorT::CantMoveOutOfMemberT { range: _, name } => {
      format!("Cannot move out of member ({:?})", name)
    }
    ICompileErrorT::CantMutateFinalMember { range: _, struct_, member_name } => {
      format!("Cannot mutate final member '{}' of container {}",
        printable_var_name(*member_name),
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Kind(typing_interner.alloc(KindTemplataT { kind: KindT::Struct(typing_interner.intern_struct_tt(StructTTValT { id: struct_.id })) }))))
    }
    ICompileErrorT::CantMutateFinalElement { range: _, coord: _ } => {
      panic!("implement: humanize CantMutateFinalElement")
    }
    ICompileErrorT::LambdaReturnDoesntMatchInterfaceConstructor { range: _ } => {
      "Argument function return type doesn't match interface method param".to_string()
    }
    ICompileErrorT::CantUseUnstackifiedLocal { range: _, local_id } => {
      format!("Can't use local that was already moved: {}",
        humanize_name(scout_arena, typing_interner, code_map, INameT::from(*local_id), None))
    }
    ICompileErrorT::CantUnstackifyOutsideLocalFromInsideWhile { range: _, local_id } => {
      format!("Can't move a local ({:?}) from inside a while loop.", local_id)
    }
    ICompileErrorT::CannotSubscriptT { range: _, tyype } => {
      format!("Cannot subscript type: {}!",
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Kind(typing_interner.alloc(KindTemplataT { kind: *tyype }))))
    }
    ICompileErrorT::CouldntConvertForReturnT { range: _, expected_type, actual_type } => {
      format!("Couldn't convert {} to expected return type {}",
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *actual_type }))),
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *expected_type }))))
    }
    ICompileErrorT::CouldntConvertForMutateT { range: _, expected_type, actual_type } => {
      format!("Mutate couldn't convert {:?} to expected destination type {:?}", actual_type, expected_type)
    }
    ICompileErrorT::CouldntFindMemberT { range: _, member_name } => {
      format!("Couldn't find member {}!", member_name)
    }
    ICompileErrorT::CouldntEvaluatImpl { range: _, eff } => {
      format!("Couldn't evaluate impl statement:\n{}",
        humanize_candidate_and_failed_solve(scout_arena, typing_interner, code_map, lines_between, line_range_containing, line_containing, eff))
    }
    ICompileErrorT::BodyResultDoesntMatch { range: _, function_name, expected_return_type, result_type } => {
      format!("Function {} return type {} doesn't match body's result: {}",
        printable_name(scout_arena, typing_interner, code_map, INameS::FunctionDeclaration(scout_arena.alloc(*function_name))),
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *expected_return_type }))),
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *result_type }))))
    }
    ICompileErrorT::CouldntFindIdentifierToLoadT { range: _, name } => {
      format!("Couldn't find anything named `{}`!", crate::postparsing::post_parser_error_humanizer::humanize_imprecise_name(*name))
    }
    ICompileErrorT::NonReadonlyReferenceFoundInPureFunctionParameter { range: _, param_name } => {
      format!("Parameter `{:?}` should be readonly, because it's in a pure function.", param_name)
    }
    ICompileErrorT::CouldntFindTypeT { range: _, name } => {
      format!("Couldn't find any type named `{:?}`!", name)
    }
    ICompileErrorT::CouldntNarrowDownCandidates { range: _, candidates } => {
      let parts: Vec<String> = candidates.iter().map(|proto| {
        format!("\n  {}", humanize_id(scout_arena, typing_interner, code_map, proto.id, None))
      }).collect();
      format!("Multiple candidates for call:{}", parts.join(""))
    }
    ICompileErrorT::ImmStructCantHaveVaryingMember { range: _, struct_name, member_name } => {
      format!("Immutable struct (\"{}\") cannot have varying member (\"{}\").",
        printable_name(scout_arena, typing_interner, code_map, *struct_name), member_name)
    }
    ICompileErrorT::ImmStructCantHaveMutableMember { range: _, struct_name, member_name } => {
      format!("Immutable struct (\"{}\") cannot have mutable member (\"{}\").",
        printable_name(scout_arena, typing_interner, code_map, *struct_name), member_name)
    }
    ICompileErrorT::WrongNumberOfDestructuresError { range: _, actual_num, expected_num } => {
      format!("Wrong number of receivers; receiving {} but should be {}.", actual_num, expected_num)
    }
    ICompileErrorT::CantDowncastUnrelatedTypes { range: _, source_kind, target_kind, candidates: _ } => {
      format!("Can't downcast `{}` to unrelated `{}`",
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Kind(typing_interner.alloc(KindTemplataT { kind: *source_kind }))),
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Kind(typing_interner.alloc(KindTemplataT { kind: *target_kind }))))
    }
    ICompileErrorT::CantDowncastToInterface { range: _, target_kind } => {
      format!("Can't downcast to an interface ({:?}) yet.", target_kind)
    }
    ICompileErrorT::ArrayElementsHaveDifferentTypes { range: _, types: _ } => {
      panic!("implement: humanize ArrayElementsHaveDifferentTypes")
    }
    ICompileErrorT::ExportedFunctionDependedOnNonExportedKind { range: _, paackage, signature, non_exported_kind } => {
      format!("Exported function:\n{}\ndepends on kind:\n{}\nthat wasn't exported from package {}",
        humanize_signature(scout_arena, typing_interner, code_map, **signature),
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Kind(typing_interner.alloc(KindTemplataT { kind: *non_exported_kind }))),
        crate::utils::source_code_utils::humanize_package(paackage))
    }
    ICompileErrorT::TypeExportedMultipleTimes { range: _, paackage: _, exports } => {
      let parts: Vec<String> = exports.iter().map(|export| {
        let pos_str = code_map(export.range.begin);
        let line = line_containing(export.range.begin);
        format!("\n  {}: {}", pos_str, line)
      }).collect();
      format!("Type exported multiple times:{}", parts.join(""))
    }
    ICompileErrorT::ExternFunctionDependedOnNonExportedKind { range: _, paackage, signature, non_exported_kind } => {
      format!("Extern function {} depends on kind {} that wasn't exported from package {}",
        humanize_signature(scout_arena, typing_interner, code_map, **signature),
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Kind(typing_interner.alloc(KindTemplataT { kind: *non_exported_kind }))),
        crate::utils::source_code_utils::humanize_package(paackage))
    }
    ICompileErrorT::ExportedImmutableKindDependedOnNonExportedKind { range: _, paackage, exported_kind, non_exported_kind } => {
      format!("Exported kind {} depends on kind {} that wasn't exported from package {}",
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Kind(typing_interner.alloc(KindTemplataT { kind: *exported_kind }))),
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Kind(typing_interner.alloc(KindTemplataT { kind: *non_exported_kind }))),
        crate::utils::source_code_utils::humanize_package(paackage))
    }
    ICompileErrorT::InitializedWrongNumberOfElements { range: _, expected_num_elements, num_elements_initialized } => {
      format!("Supplied {} elements, but expected {}.", num_elements_initialized, expected_num_elements)
    }
    ICompileErrorT::CouldntFindFunctionToCallT { range, fff } => {
      humanize_find_function_failure(scout_arena, typing_interner, verbose, code_map, lines_between, line_range_containing, line_containing, range.to_vec(), fff)
    }
    ICompileErrorT::CouldntEvaluateFunction { range: _, eff } => {
      format!("Couldn't evaluate function:\n{}",
        humanize_defining_error(scout_arena, typing_interner, verbose, code_map, lines_between, line_range_containing, line_containing, eff))
    }
    ICompileErrorT::FunctionAlreadyExists { old_function_range, new_function_range: _, signature } => {
      format!("Function {} already exists! Previous declaration at:\n{}",
        humanize_id(scout_arena, typing_interner, code_map, *signature, None),
        code_map(old_function_range.begin))
    }
    ICompileErrorT::AbstractMethodOutsideOpenInterface { range: _ } => {
      "Open (non-sealed) interfaces can't have abstract methods defined outside the interface.".to_string()
    }
    ICompileErrorT::IfConditionIsntBoolean { range: _, actual_type } => {
      format!("If condition should be a bool, but was: {:?}", actual_type)
    }
    ICompileErrorT::WhileConditionIsntBoolean { range: _, actual_type } => {
      format!("If condition should be a bool, but was: {:?}", actual_type)
    }
    ICompileErrorT::CantImplNonInterface { range: _, templata } => {
      format!("Can't extend a non-interface: {:?}", templata)
    }
    ICompileErrorT::NonCitizenCantImpl { range: _, templata: _ } => {
      panic!("implement: humanize NonCitizenCantImpl")
    }
    ICompileErrorT::TypingPassSolverError { range: _, failed_solve } => {
      humanize_candidate_and_failed_solve(scout_arena, typing_interner, code_map, lines_between, line_range_containing, line_containing, failed_solve)
    }
    ICompileErrorT::HigherTypingInferError { range: _, err: _ } => {
      panic!("implement: humanize HigherTypingInferError")
    }
    ICompileErrorT::TooManyTypesWithNameT { range: _, name: _ } => {
      panic!("implement: humanize TooManyTypesWithNameT")
    }
    ICompileErrorT::NotEnoughGenericArgs { range: _ } => {
      panic!("implement: humanize NotEnoughGenericArgs")
    }
    ICompileErrorT::ImplSubCitizenNotFound { range: _, name: _ } => {
      panic!("implement: humanize ImplSubCitizenNotFound")
    }
    ICompileErrorT::ImplSuperInterfaceNotFound { range: _, name: _ } => {
      panic!("implement: humanize ImplSuperInterfaceNotFound")
    }
    ICompileErrorT::CantRestackifyOutsideLocalFromInsideWhile { range: _, local_id: _ } => {
      panic!("implement: humanize CantRestackifyOutsideLocalFromInsideWhile")
    }
    ICompileErrorT::CouldntEvaluateStruct { range: _, eff: _ } => {
      panic!("implement: humanize CouldntEvaluateStruct")
    }
    ICompileErrorT::CouldntEvaluateInterface { range: _, eff: _ } => {
      panic!("implement: humanize CouldntEvaluateInterface")
    }
  };
  // err.range.reverse.map(range => { ... }).mkString("") + errorStrBody + "\n"
  let prefix: String = err.range().iter().rev().map(|range| {
    let pos_str = code_map(range.begin);
    let line_contents = line_containing(range.begin);
    format!("At {}:\n{}\n", pos_str, line_contents)
  }).collect::<Vec<_>>().join("");
  format!("{}{}\n", prefix, error_str_body)
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
        case CantUseRuneValueAsExpression(range, rune) => {
          "Can't use rune `" + humanizeRune(rune) + "` as a value expression. Did you mean a local variable with a similar name?"
        }
        case NonReadonlyReferenceFoundInPureFunctionParameter(range, name) => {
          "Parameter `" + name + "` should be readonly, because it's in a pure function."
        }
        case CouldntFindTypeT(range, name) => {
          "Couldn't find any type named `" + name + "`!"
        }
        case CouldntNarrowDownCandidates(range, candidates) => {
          "Multiple candidates for call:" +
            candidates.map(proto => "\n  " + humanizeId(codeMap, proto.id, None)).mkString("")
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
          "Function " + humanizeId(codeMap, signature, None) + " already exists! Previous declaration at:\n" +
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
pub fn humanize_defining_error<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, verbose: bool, code_map: &dyn Fn(CodeLocationS<'s>) -> String, lines_between: &dyn Fn(CodeLocationS<'s>, CodeLocationS<'s>) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS<'s>) -> String, err: &IDefiningError<'s, 't>) -> String {
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
pub fn humanize_resolving_error<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, verbose: bool, code_map: &dyn Fn(CodeLocationS<'s>) -> String, lines_between: &dyn Fn(CodeLocationS<'s>, CodeLocationS<'s>) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS<'s>) -> String, error: &IResolvingError<'s, 't>) -> String {
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
        "Found function: " + humanizeId(codeMap, actualPrototype.id, None) + " which returns " + humanizeTemplata(codeMap, CoordTemplataT(actualPrototype.returnType)) + " but expected return type of " + humanizeTemplata(codeMap, CoordTemplataT(expectedReturnType))
      }
      case other => vimpl(other)
    }
  }
*/
pub fn humanize_find_function_failure<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, verbose: bool, code_map: &dyn Fn(CodeLocationS<'s>) -> String, lines_between: &dyn Fn(CodeLocationS<'s>, CodeLocationS<'s>) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS<'s>) -> String, invocation_range: Vec<RangeS<'s>>, fff: &FindFunctionFailure<'s, 't>) -> String {
  let FindFunctionFailure { name, args, rejected_callee_to_reason } = fff;
  let args_str = args.iter().map(|tyype| {
    humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *tyype })))
  }).collect::<Vec<_>>().join(", ");
  let tail = if rejected_callee_to_reason.is_empty() {
    "No function with that name exists.\n".to_string()
  } else {
    let parts = rejected_callee_to_reason.iter().enumerate().map(|(index, (candidate, reason))| {
      format!("Candidate {} (of {}): {}{}\n\n",
        index + 1, rejected_callee_to_reason.len(),
        humanize_candidate(scout_arena, typing_interner, code_map, line_range_containing, candidate),
        humanize_rejection_reason(scout_arena, typing_interner, verbose, code_map, lines_between, line_range_containing, line_containing, &invocation_range, reason))
    }).collect::<Vec<_>>().join("");
    format!("Rejected candidates:\n\n{}", parts)
  };
  format!("Couldn't find a suitable function {}({}). {}",
    crate::postparsing::post_parser_error_humanizer::humanize_imprecise_name(*name),
    args_str,
    tail)
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
fn printable_name<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, name: INameS<'s>) -> String {
  match name {
    INameS::VarName(n) => panic!("implement: printable_name VarName"),
    INameS::TopLevelStructDeclaration(n) => n.name.0.to_string(),
    INameS::TopLevelInterfaceDeclaration(n) => n.name.0.to_string(),
    INameS::FunctionDeclaration(n) => match n {
      IFunctionDeclarationNameS::FunctionName(fn_name) => format!("{}: {}", code_map(fn_name.code_location), fn_name.name.0),
      _ => panic!("implement: printable_name FunctionDeclaration other"),
    },
    _ => panic!("implement: printable_name other"),
  }
}
/*
  private def printableName(
    codeMap: CodeLocationS => String,
    name: INameS):
  String = {
    name match {
      case CodeVarNameS(name) => name.str
      case TopLevelCitizenDeclarationNameS(name, codeLocation) => name.str
      case AnonymousSubstructTemplateNameS(TopLevelInterfaceDeclarationNameS(name, _)) => name.str + ".anonymous"
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
fn printable_var_name<'s, 't>(name: IVarNameT<'s, 't>) -> String {
  match name {
    IVarNameT::CodeVar(n) => n.name.0.to_string(),
    _ => panic!("implement: printable_var_name other"),
  }
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
fn humanize_rejection_reason<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, verbose: bool, code_map: &dyn Fn(CodeLocationS<'s>) -> String, lines_between: &dyn Fn(CodeLocationS<'s>, CodeLocationS<'s>) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS<'s>) -> String, invocation_range: &Vec<RangeS<'s>>, reason: &IFindFunctionFailureReason<'s, 't>) -> String {
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
pub fn humanize_rule_error<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, lines_between: &dyn Fn(CodeLocationS<'s>, CodeLocationS<'s>) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS<'s>) -> String, error: ITypingPassSolverError<'s, 't>) -> String {
  match error {
    ITypingPassSolverError::IsaFailed { .. } => panic!("implement: humanize_rule_error IsaFailed"),
    ITypingPassSolverError::BadIsaSubKind { .. } => panic!("implement: humanize_rule_error BadIsaSubKind"),
    ITypingPassSolverError::CantGetComponentsOfPlaceholderPrototype { .. } => panic!("implement: humanize_rule_error CantGetComponentsOfPlaceholderPrototype"),
    ITypingPassSolverError::ReturnTypeConflict { .. } => panic!("implement: humanize_rule_error ReturnTypeConflict"),
    ITypingPassSolverError::CantShareMutable { .. } => panic!("implement: humanize_rule_error CantShareMutable"),
    ITypingPassSolverError::BadIsaSuperKind { .. } => panic!("implement: humanize_rule_error BadIsaSuperKind"),
    ITypingPassSolverError::SendingNonIdenticalKinds { .. } => panic!("implement: humanize_rule_error SendingNonIdenticalKinds"),
    ITypingPassSolverError::SendingNonCitizen { .. } => panic!("implement: humanize_rule_error SendingNonCitizen"),
    ITypingPassSolverError::CantCheckPlaceholder { .. } => panic!("implement: humanize_rule_error CantCheckPlaceholder"),
    ITypingPassSolverError::CouldntFindFunction { .. } => panic!("implement: humanize_rule_error CouldntFindFunction"),
    ITypingPassSolverError::CouldntResolveKind { .. } => panic!("implement: humanize_rule_error CouldntResolveKind"),
    ITypingPassSolverError::WrongNumberOfTemplateArgs { .. } => panic!("implement: humanize_rule_error WrongNumberOfTemplateArgs"),
    ITypingPassSolverError::LookupFailed { .. } => panic!("implement: humanize_rule_error LookupFailed"),
    ITypingPassSolverError::KindIsNotConcrete { kind } => {
      "Expected kind to be concrete, but was not. Kind: ".to_string() + &humanize_kind(scout_arena, typing_interner, code_map, kind, None)
    }
    ITypingPassSolverError::OneOfFailed { .. } => panic!("implement: humanize_rule_error OneOfFailed"),
    ITypingPassSolverError::KindIsNotInterface { .. } => panic!("implement: humanize_rule_error KindIsNotInterface"),
    ITypingPassSolverError::CallResultIsntCallable { .. } => panic!("implement: humanize_rule_error CallResultIsntCallable"),
    ITypingPassSolverError::CallResultWasntExpectedType { .. } => panic!("implement: humanize_rule_error CallResultWasntExpectedType"),
    ITypingPassSolverError::OwnershipDidntMatch { .. } => panic!("implement: humanize_rule_error OwnershipDidntMatch"),
    ITypingPassSolverError::ReceivingDifferentOwnerships { .. } => panic!("implement: humanize_rule_error ReceivingDifferentOwnerships"),
    ITypingPassSolverError::NoAncestorsSatisfyCall { .. } => panic!("implement: humanize_rule_error NoAncestorsSatisfyCall"),
    ITypingPassSolverError::KindIsNotStruct { .. } => panic!("implement: humanize_rule_error KindIsNotStruct"),
    ITypingPassSolverError::CouldntFindImpl { .. } => panic!("implement: humanize_rule_error CouldntFindImpl"),
    ITypingPassSolverError::CantSharePlaceholder { .. } => panic!("implement: humanize_rule_error CantSharePlaceholder"),
    ITypingPassSolverError::NoCommonAncestors { .. } => panic!("implement: humanize_rule_error NoCommonAncestors"),
    ITypingPassSolverError::CantDetermineNarrowestKind { .. } => panic!("implement: humanize_rule_error CantDetermineNarrowestKind"),
    ITypingPassSolverError::FunctionDoesntHaveName { .. } => panic!("implement: humanize_rule_error FunctionDoesntHaveName"),
    ITypingPassSolverError::InternalSolverError { .. } => panic!("implement: humanize_rule_error InternalSolverError"),
  }
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
        "Found function: " + humanizeId(codeMap, actualPrototype.id, None) + " which returns " + humanizeTemplata(codeMap, CoordTemplataT(actualPrototype.returnType)) + " but expected return type of " + humanizeTemplata(codeMap, CoordTemplataT(expectedReturnType))
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
        "Expected kind to be concrete, but was not. Kind: " + humanizeKind(codeMap, kind, None)
      }
      case OneOfFailed(rule) => {
        "One-of rule failed."
      }
      case KindIsNotInterface(kind) => {
        "Expected kind to be interface, but was not. Kind: " + humanizeKind(codeMap, kind, None)
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
      case KindIsNotStruct(kind) => {
        "Expected kind to be struct, but was not. Kind: " + humanizeKind(codeMap, kind, None)
      }
      case CouldntFindImpl(range, fail) => {
        "Couldn't find impl: " + fail
      }
      case CantSharePlaceholder(kind) => {
        "Can't share a placeholder kind: " + humanizeTemplata(codeMap, KindTemplataT(kind))
      }
      case NoCommonAncestors(params) => {
        "No common ancestors: " +
          params.map({ case (rune, coord) =>
            humanizeRune(rune) + " = " + humanizeTemplata(codeMap, CoordTemplataT(coord))
          }).mkString(", ")
      }
      case CantDetermineNarrowestKind(kinds) => {
        "Can't determine narrowest kind among: " + kinds.map(humanizeKind(codeMap, _, None)).mkString(", ")
      }
      case FunctionDoesntHaveName(range, name) => {
        "Function doesn't have name: " + humanizeName(codeMap, name)
      }
      case InternalSolverError(range, err) => {
        err match {
          case SolverConflict(rune, previousConclusion, newConclusion) => {
            "Solver conflict on rune " + humanizeRune(rune) + ": was " + humanizeTemplata(codeMap, previousConclusion) + " but now concluding " + humanizeTemplata(codeMap, newConclusion)
          }
          case RuleError(innerErr) => {
            humanizeRuleError(codeMap, linesBetween, lineRangeContaining, lineContaining, innerErr)
          }
          case SolveIncomplete() => {
            "Solve incomplete"
          }
        }
      }
    }
  }
*/
pub fn humanize_candidate_and_failed_solve<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, lines_between: &dyn Fn(CodeLocationS<'s>, CodeLocationS<'s>) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS<'s>) -> String, result: &FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>) -> String {
  let (text, _line_begins) = crate::solver::solver_error_humanizer::humanize_failed_solve(
    |loc| code_map(*loc),
    |a, b| lines_between(*a, *b),
    |loc| line_range_containing(*loc),
    |loc| line_containing(*loc),
    |rune| crate::postparsing::post_parser_error_humanizer::humanize_rune(rune),
    |t| humanize_templata(scout_arena, typing_interner, &|loc| code_map(loc), t),
    |err| humanize_rule_error(scout_arena, typing_interner, code_map, lines_between, line_range_containing, line_containing, err),
    |rule: &IRulexSR<'s>| *rule.range(),
    |rule: &IRulexSR<'s>| rule.rune_usages().iter().map(|u| (u.rune, u.range)).collect(),
    |rule: &IRulexSR<'s>| rule.rune_usages().iter().map(|u| u.rune).collect(),
    |rule: &IRulexSR<'s>| crate::postparsing::post_parser_error_humanizer::humanize_rule(rule),
    result,
  );
  text
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
pub fn humanize_candidate<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, candidate: &ICalleeCandidate<'s, 't>) -> String {
  panic!("Unimplemented: humanize_candidate");
}
/*
  def humanizeCandidate(
    codeMap: CodeLocationS => String,
    lineRangeContaining: (CodeLocationS) => RangeS,
    candidate: ICalleeCandidate) = {
    candidate match {
      case HeaderCalleeCandidate(header) => {
        humanizeId(codeMap, header.id, None)
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
pub fn humanize_templata<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, templata: ITemplataT<'s, 't>) -> String {
  match templata {
    ITemplataT::RuntimeSizedArrayTemplate(_) => "Array".to_string(),
    ITemplataT::StaticSizedArrayTemplate(_) => "StaticArray".to_string(),
    ITemplataT::InterfaceDefinition(_) => panic!("implement: humanize_templata InterfaceDefinition"),
    ITemplataT::StructDefinition(_) => panic!("implement: humanize_templata StructDefinition"),
    ITemplataT::Variability(variability) => panic!("implement: humanize_templata Variability"),
    ITemplataT::Integer(value) => panic!("implement: humanize_templata Integer"),
    ITemplataT::Mutability(mutability) => panic!("implement: humanize_templata Mutability"),
    ITemplataT::Ownership(ownership) => match ownership.ownership {
      OwnershipT::Own => "own".to_string(),
      OwnershipT::Borrow => "borrow".to_string(),
      OwnershipT::Weak => "weak".to_string(),
      OwnershipT::Share => "share".to_string(),
    },
    ITemplataT::Prototype(prototype) => panic!("implement: humanize_templata Prototype"),
    ITemplataT::Coord(coord_templata) => humanize_coord(scout_arena, typing_interner, code_map, coord_templata.coord),
    ITemplataT::Kind(kind_templata) => humanize_kind(scout_arena, typing_interner, code_map, kind_templata.kind, None),
    ITemplataT::CoordList(coords) => panic!("implement: humanize_templata CoordList"),
    ITemplataT::String(value) => panic!("implement: humanize_templata String"),
    ITemplataT::Placeholder(_) => panic!("implement: humanize_templata Placeholder"),
    _ => panic!("implement: humanize_templata other"),
  }
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
        humanizeId(codeMap, prototype.id, None)
      }
      case CoordTemplataT(coord) => {
        humanizeCoord(codeMap, coord)
      }
      case KindTemplataT(kind) => {
        humanizeKind(codeMap, kind, None)
      }
      case CoordListTemplataT(coords) => {
        "(" + coords.map(CoordTemplataT).map(humanizeTemplata(codeMap, _)).mkString(", ") + ")"
      }
      case StringTemplataT(value) => "\"" + value + "\""
      case PlaceholderTemplataT(id, tyype) => {
        tyype match {
          case CoordTemplataType() => "$" + humanizeId(codeMap, id, None)
          case _ => humanizeTemplataType(tyype) + "$" + humanizeId(codeMap, id, None)
        }
      }
      case other => vimpl(other)
    }
  }
*/
fn humanize_coord<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, coord: CoordT<'s, 't>) -> String {
  let CoordT { ownership, region, kind } = coord;
  let ownership_str = match ownership {
    OwnershipT::Own => "",
    OwnershipT::Share => "",
    OwnershipT::Borrow => "&",
    OwnershipT::Weak => "&&",
  };
  let kind_str = humanize_kind(scout_arena, typing_interner, code_map, kind, Some(region));
  format!("{}{}", ownership_str, kind_str)
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
fn humanize_kind<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, kind: KindT<'s, 't>, containing_region: Option<RegionT>) -> String {
  match kind {
    KindT::Int(IntT { bits }) => format!("i{}", bits),
    KindT::Bool(_) => "bool".to_string(),
    KindT::KindPlaceholder(name) => format!("Kind${}", humanize_id(scout_arena, typing_interner, code_map, name.id, None)),
    KindT::Str(_) => "str".to_string(),
    KindT::Never(_) => "never".to_string(),
    KindT::Void(_) => "void".to_string(),
    KindT::Float(_) => "float".to_string(),
    KindT::OverloadSet(s) => format!("(overloads: {})",
      crate::postparsing::post_parser_error_humanizer::humanize_imprecise_name(*s.name)),
    KindT::Interface(name) => humanize_id(scout_arena, typing_interner, code_map, name.id, containing_region),
    KindT::Struct(name) => humanize_id(scout_arena, typing_interner, code_map, name.id, containing_region),
    KindT::RuntimeSizedArray(rsa) => panic!("implement: humanize_kind RuntimeSizedArray"),
    KindT::StaticSizedArray(ssa) => panic!("implement: humanize_kind StaticSizedArray"),
  }
}
/*
  private def humanizeKind(
      codeMap: CodeLocationS => String,
      kind: KindT,
      containingRegion: Option[RegionT]
  ) = {
    kind match {
      case IntT(bits) => "i" + bits
      case BoolT() => "bool"
      case KindPlaceholderT(name) => "Kind$" + humanizeId(codeMap, name, None)
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
pub fn humanize_id<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, name: IdT<'s, 't>, containing_region: Option<RegionT>) -> String
where 's: 't,
{
  let prefix = if !name.init_steps.is_empty() {
    name.init_steps.iter().map(|n| humanize_name(scout_arena, typing_interner, code_map, *n, None)).collect::<Vec<_>>().join(".") + "."
  } else {
    "".to_string()
  };
  prefix + &humanize_name(scout_arena, typing_interner, code_map, name.local_name, containing_region)
}
/*
  def humanizeId[T <: INameT](
    codeMap: CodeLocationS => String,
    name: IdT[T],
    containingRegion: Option[RegionT]):
  String = {
    (if (name.initSteps.nonEmpty) {
      name.initSteps.map(n => humanizeName(codeMap, n)).mkString(".") + "."
    } else {
      ""
    }) +
      humanizeName(codeMap, name.localName, containingRegion)
  }
*/
pub fn humanize_name<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, name: INameT<'s, 't>, containing_region: Option<RegionT>) -> String {
  match name {
    INameT::AnonymousSubstructConstructor(n) => panic!("implement: humanize_name AnonymousSubstructConstructor"),
    INameT::AnonymousSubstructConstructorTemplate(n) => panic!("implement: humanize_name AnonymousSubstructConstructorTemplate"),
    INameT::Self_(_) => "self".to_string(),
    INameT::OverrideDispatcherTemplate(n) => panic!("implement: humanize_name OverrideDispatcherTemplate"),
    INameT::OverrideDispatcher(n) => panic!("implement: humanize_name OverrideDispatcher"),
    INameT::Iterator(n) => panic!("implement: humanize_name Iterator"),
    INameT::Iterable(n) => panic!("implement: humanize_name Iterable"),
    INameT::IterationOption(n) => panic!("implement: humanize_name IterationOption"),
    INameT::ImplTemplate(n) => panic!("implement: humanize_name ImplTemplate"),
    INameT::ForwarderFunction(n) => panic!("implement: humanize_name ForwarderFunction"),
    INameT::ForwarderFunctionTemplate(n) => panic!("implement: humanize_name ForwarderFunctionTemplate"),
    INameT::MagicParam(n) => panic!("implement: humanize_name MagicParam"),
    INameT::ClosureParam(n) => panic!("implement: humanize_name ClosureParam"),
    INameT::ConstructingMember(n) => panic!("implement: humanize_name ConstructingMember"),
    INameT::TypingPassBlockResultVar(n) => panic!("implement: humanize_name TypingPassBlockResultVar"),
    INameT::TypingPassFunctionResultVar(n) => panic!("implement: humanize_name TypingPassFunctionResultVar"),
    INameT::TypingPassTemporaryVar(n) => panic!("implement: humanize_name TypingPassTemporaryVar"),
    INameT::FunctionBoundTemplate(n) => n.human_name.0.to_string(),
    INameT::LambdaCallFunctionTemplate(n) => panic!("implement: humanize_name LambdaCallFunctionTemplate"),
    INameT::LambdaCitizenTemplate(n) => panic!("implement: humanize_name LambdaCitizenTemplate"),
    INameT::LambdaCallFunction(n) => panic!("implement: humanize_name LambdaCallFunction"),
    INameT::FunctionBound(n) => panic!("implement: humanize_name FunctionBound"),
    INameT::KindPlaceholder(n) => humanize_name(scout_arena, typing_interner, code_map, INameT::KindPlaceholderTemplate(n.template), None),
    INameT::KindPlaceholderTemplate(n) => panic!("implement: humanize_name KindPlaceholderTemplate"),
    INameT::CodeVar(n) => n.name.0.to_string(),
    INameT::LambdaCitizen(n) => panic!("implement: humanize_name LambdaCitizen"),
    INameT::FunctionTemplate(n) => n.human_name.0.to_string(),
    INameT::ExternFunction(n) => n.human_name.0.to_string(),
    INameT::Function(n) => {
      humanize_name(scout_arena, typing_interner, code_map, INameT::FunctionTemplate(n.template), None) +
        &humanize_generic_args(scout_arena, typing_interner, code_map, n.template_args, containing_region) +
        &(if !n.parameters.is_empty() {
          "(".to_string() + &n.parameters.iter().map(|p| humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *p })))).collect::<Vec<_>>().join(", ") + ")"
        } else {
          "".to_string()
        })
    }
    INameT::Struct(sn) => {
      let template_name = match sn.template {
        IStructTemplateNameT::LambdaCitizenTemplate(n) => INameT::LambdaCitizenTemplate(n),
        IStructTemplateNameT::StructTemplate(n) => INameT::StructTemplate(n),
        IStructTemplateNameT::AnonymousSubstructTemplate(n) => INameT::AnonymousSubstructTemplate(n),
      };
      humanize_name(scout_arena, typing_interner, code_map, template_name, None) +
        &humanize_generic_args(scout_arena, typing_interner, code_map, sn.template_args, containing_region)
    }
    INameT::Interface(sn) => {
      humanize_name(scout_arena, typing_interner, code_map, INameT::InterfaceTemplate(sn.template), None) +
        &humanize_generic_args(scout_arena, typing_interner, code_map, sn.template_args, containing_region)
    }
    INameT::AnonymousSubstruct(n) => panic!("implement: humanize_name AnonymousSubstruct"),
    INameT::AnonymousSubstructTemplate(n) => panic!("implement: humanize_name AnonymousSubstructTemplate"),
    INameT::StructTemplate(n) => n.human_name.0.to_string(),
    INameT::InterfaceTemplate(n) => n.human_namee.0.to_string(),
    INameT::NonKindNonRegionPlaceholder(n) => panic!("implement: humanize_name NonKindNonRegionPlaceholder"),
    _ => panic!("implement: humanize_name other"),
  }
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
      case OverrideDispatcherTemplateNameT(implId) => "ovdt:" + humanizeId(codeMap, implId, None)
      case OverrideDispatcherNameT(OverrideDispatcherTemplateNameT(implId), templateArgs, parameters) => {
        "ovd:" + humanizeId(codeMap, implId, None) +
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
      case PredictedFunctionTemplateNameT(humanName) => humanName.str
      case PredictedFunctionNameT(template, templateArgs, parameters) => {
        humanizeName(codeMap, template) +
          humanizeGenericArgs(codeMap, templateArgs, None) +
          "(" + parameters.map(CoordTemplataT).map(humanizeTemplata(codeMap, _)).mkString(", ") + ")"
      }
      case KindPlaceholderNameT(template) => humanizeName(codeMap, template)
      case KindPlaceholderTemplateNameT(index, rune) => humanizeRune(rune)
      case CodeVarNameT(name) => name.str
      case LambdaCitizenNameT(template) => humanizeName(codeMap, template) + "<>"
      case FunctionTemplateNameT(humanName, codeLoc) => humanName.str
      case ExternFunctionNameT(humanName, templateArgs, parameters) => {
        humanName.str +
          humanizeGenericArgs(codeMap, templateArgs, containingRegion) +
          (if (parameters.nonEmpty) {
            "(" + parameters.map(CoordTemplataT).map(humanizeTemplata(codeMap, _)).mkString(", ") + ")"
          } else {
            ""
          })
      }
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
fn humanize_generic_args<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, template_args: &[ITemplataT<'s, 't>], containing_region: Option<RegionT>) -> String {
  if template_args.is_empty() {
    "".to_string()
  } else {
    let init = &template_args[..template_args.len() - 1];
    let last = template_args.last().unwrap();
    let last_str = match containing_region {
      None => humanize_templata(scout_arena, typing_interner, code_map, *last),
      Some(_) => "_".to_string(),
    };
    let parts = init.iter().map(|t| humanize_templata(scout_arena, typing_interner, code_map, *t))
      .chain(std::iter::once(last_str))
      .collect::<Vec<_>>().join(", ");
    format!("<{}>", parts)
  }
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
pub fn humanize_signature<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, signature: SignatureT<'s, 't>) -> String {
  humanize_id(scout_arena, typing_interner, code_map, signature.id, None)
}
/*
  def humanizeSignature(codeMap: CodeLocationS => String, signature: SignatureT): String = {
    humanizeId(codeMap, signature.id, None)
  }
}
*/
