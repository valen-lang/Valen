use crate::utils::range::{RangeS, CodeLocationS};
use crate::interner::Interner;
use crate::postparsing::*;
use crate::postparsing::names::*;
use crate::postparsing::itemplatatype::ITemplataType;
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
use crate::postparsing::post_parser_error_humanizer::humanize_imprecise_name;
use crate::postparsing::post_parser_error_humanizer::humanize_name_for_struct_declaration;
use crate::postparsing::post_parser_error_humanizer::humanize_ownership;
use crate::postparsing::post_parser_error_humanizer::humanize_rule;
use crate::postparsing::post_parser_error_humanizer::humanize_rune;
use crate::postparsing::post_parser_error_humanizer::humanize_rune_type_error;
use crate::postparsing::post_parser_error_humanizer::humanize_templata_type;
use crate::postparsing::rune_type_solver::IRuneTypeRuleError;
use crate::typing::templata::conversions::unevaluate_ownership;
use crate::solver::solver_error_humanizer::humanize_failed_solve as solver_humanize_failed_solve;
use crate::utils::source_code_utils::humanize_package;
use std::iter::once;


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
    ICompileErrorT::CouldntSolveRuneTypesT { range: _, error: _ } => {
      panic!("implement: humanize CouldntSolveRuneTypesT")
      // "Couldn't solve rune types:\n" +
      //   HigherTypingErrorHumanizer.humanizeRuneTypeSolveError(
      //     codeMap, linesBetween, lineRangeContaining, lineContaining, error)
    }
    ICompileErrorT::UnexpectedArrayElementType { range: _, expected_type, actual_type } => {
      format!("Unexpected type for array element, tried to put a {} into an array of {}",
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *actual_type }))),
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *expected_type }))))
    }
    ICompileErrorT::IndexedArrayWithNonInteger { range: _, types } => {
      format!("Indexed array with non-integer: {}",
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *types }))))
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
    ICompileErrorT::LambdaReturnDoesntMatchInterfaceConstructor { range: _ } => {
      "Argument function return type doesn't match interface method param".to_string()
    }
    ICompileErrorT::CantUseUnstackifiedLocal { range: _, local_id } => {
      format!("Can't use local that was already moved: {}",
        humanize_name(scout_arena, typing_interner, code_map, INameT::from(*local_id), None))
    }
    ICompileErrorT::CantUnstackifyOutsideLocalFromInsideWhile { range: _, local_id } => {
      format!("Can't move a local ({}) from inside a while loop.",
        humanize_name(scout_arena, typing_interner, code_map, INameT::from(*local_id), None))
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
      format!("Mutate couldn't convert {} to expected destination type {}",
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *actual_type }))),
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *expected_type }))))
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
      format!("Couldn't find anything named `{}`!", humanize_imprecise_name(*name))
    }
    ICompileErrorT::CantUseRuneValueAsExpression { range: _, rune } => {
      format!("Can't use rune `{}` as a value expression. Did you mean a local variable with a similar name?", humanize_rune(*rune))
    }
    ICompileErrorT::WeakableImplingMismatch { range: _, struct_weakable, interface_weakable } => {
      format!("Weakable mismatch in impl: struct {} weakable, but interface {}.",
        if *struct_weakable { "is" } else { "is not" },
        if *interface_weakable { "is" } else { "is not" })
    }
    ICompileErrorT::TookWeakRefOfNonWeakableError { range: _ } => {
      "Took a weak reference of something that isn't weakable. Did you mean to add the `weakable` keyword?".to_string()
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
    ICompileErrorT::ArrayElementsHaveDifferentTypes { range: _, types } => {
      let types_str = types.iter().map(|c|
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *c })))
      ).collect::<Vec<_>>().join(", ");
      format!("Array's elements have different types: {}", types_str)
      // "Array's elements have different types: " + types.mkString(", ")
    }
    ICompileErrorT::ExportedFunctionDependedOnNonExportedKind { range: _, paackage, signature, non_exported_kind } => {
      format!(r"Exported function:
{}
depends on kind:
{}
that wasn't exported from package {}",
        humanize_signature(scout_arena, typing_interner, code_map, **signature),
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Kind(typing_interner.alloc(KindTemplataT { kind: *non_exported_kind }))),
        humanize_package(paackage))
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
        humanize_package(paackage))
    }
    ICompileErrorT::ExportedKindDependedOnNonExportedKind { range: _, paackage, exported_kind, non_exported_kind } => {
      format!("Exported kind {} depends on kind {} that wasn't exported from package {}",
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Kind(typing_interner.alloc(KindTemplataT { kind: *exported_kind }))),
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Kind(typing_interner.alloc(KindTemplataT { kind: *non_exported_kind }))),
        humanize_package(paackage))
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
      format!("If condition should be a bool, but was: {}",
        humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *actual_type }))))
    }
    ICompileErrorT::WhileConditionIsntBoolean { range: _, actual_type } => {
      format!("If condition should be a bool, but was: {:?}", actual_type)
    }
    ICompileErrorT::CantImplNonInterface { range: _, templata } => {
      format!("Can't extend a non-interface: {}",
        humanize_templata(scout_arena, typing_interner, code_map, *templata))
    }
    ICompileErrorT::NonCitizenCantImpl { range: _, templata: _ } => {
      panic!("implement: humanize NonCitizenCantImpl")
    }
    ICompileErrorT::TypingPassSolverError { range: _, failed_solve } => {
      humanize_candidate_and_failed_solve(scout_arena, typing_interner, code_map, lines_between, line_range_containing, line_containing, failed_solve)
    }
    ICompileErrorT::HigherTypingInferError { range: _, err } => {
      let inner_msg = match &err.failed_solve.error {
        ISolverError::RuleError(re) => {
          crate::postparsing::post_parser_error_humanizer::humanize_rune_type_error(code_map, &re.err)
        }
        ISolverError::SolverConflict(_) | ISolverError::SolveIncomplete(_) => {
          format!("{:?}", err.failed_solve.error)
        }
      };
      format!(": Couldn't solve generics types:\n{}", inner_msg)
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

pub fn humanize_defining_error<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, verbose: bool, code_map: &dyn Fn(CodeLocationS<'s>) -> String, lines_between: &dyn Fn(CodeLocationS<'s>, CodeLocationS<'s>) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS<'s>) -> String, err: &IDefiningError<'s, 't>) -> String {
  match err {
    IDefiningError::DefiningResolveConclusionError(inner) => {
      humanize_conclusion_resolve_error(scout_arena, typing_interner, verbose, code_map, lines_between, line_range_containing, line_containing, inner)
    }
    IDefiningError::DefiningSolveFailedOrIncomplete(inner) => {
      humanize_failed_solve(scout_arena, typing_interner, verbose, code_map, lines_between, line_range_containing, line_containing, inner.clone())
    }
  }
}

pub fn humanize_resolve_failure<'s, 't>(verbose: bool, code_map: &dyn Fn(CodeLocationS<'s>) -> String, lines_between: &dyn Fn(CodeLocationS<'s>, CodeLocationS<'s>) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS<'s>) -> String, fff: ResolveFailure<'s, 't, KindT<'s, 't>>) -> String {
  panic!("Unimplemented: humanize_resolve_failure");
  // val ResolveFailure(range, reason) = fff
  // humanizeResolvingError(verbose, codeMap, linesBetween, lineRangeContaining, lineContaining, reason)
}

pub fn humanize_resolving_error<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, verbose: bool, code_map: &dyn Fn(CodeLocationS<'s>) -> String, lines_between: &dyn Fn(CodeLocationS<'s>, CodeLocationS<'s>) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS<'s>) -> String, error: &IResolvingError<'s, 't>) -> String {
  match error {
    IResolvingError::ResolvingResolveConclusionError(inner) => {
      humanize_conclusion_resolve_error(scout_arena, typing_interner, verbose, code_map, lines_between, line_range_containing, line_containing, inner.as_ref())
    }
    IResolvingError::ResolvingSolveFailedOrIncomplete(inner) => {
      humanize_failed_solve(scout_arena, typing_interner, verbose, code_map, lines_between, line_range_containing, line_containing, inner.clone())
    }
  }
}

pub fn humanize_failed_solve<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, verbose: bool, code_map: &dyn Fn(CodeLocationS<'s>) -> String, lines_between: &dyn Fn(CodeLocationS<'s>, CodeLocationS<'s>) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS<'s>) -> String, error: FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>) -> String {
  humanize_candidate_and_failed_solve(scout_arena, typing_interner, code_map, lines_between, line_range_containing, line_containing, &error)
}

pub fn humanize_conclusion_resolve_error<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, verbose: bool, code_map: &dyn Fn(CodeLocationS<'s>) -> String, lines_between: &dyn Fn(CodeLocationS<'s>, CodeLocationS<'s>) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS<'s>) -> String, error: &IConclusionResolveError<'s, 't>) -> String {
  match error {
    IConclusionResolveError::CouldntFindKindForConclusionResolve(_) => panic!("implement: humanize_conclusion_resolve_error CouldntFindKindForConclusionResolve"),
    IConclusionResolveError::CouldntFindFunctionForConclusionResolve { range, fff } => {
      humanize_find_function_failure(scout_arena, typing_interner, verbose, code_map, lines_between, line_range_containing, line_containing, range.to_vec(), fff)
    }
    IConclusionResolveError::ReturnTypeConflictInConclusionResolve { range: _, expected_return_type, actual } => {
      "Found function: ".to_string() + &humanize_id(scout_arena, typing_interner, code_map, actual.id, None) +
        " which returns " + &humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: actual.return_type }))) +
        " but expected return type of " + &humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *expected_return_type })))
    }
    IConclusionResolveError::CouldntFindImplForConclusionResolve { .. } => panic!("implement: humanize_conclusion_resolve_error CouldntFindImplForConclusionResolve"),
  }
}

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
    humanize_imprecise_name(*name),
    args_str,
    tail)
}

pub fn humanize_banner<'s>(code_map: &dyn Fn(CodeLocationS<'s>) -> String, banner: FunctionBannerT) -> String {
  panic!("Unimplemented: humanize_banner");
  // banner.originFunctionTemplata match {
  //   case None => "(internal)"
  //   case Some(x) => printableName(codeMap, x.function.name)
  // }
}

fn printable_name<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, name: INameS<'s>) -> String {
  match name {
    INameS::VarName(n) => {
      panic!("implement: printable_name VarName");
      // name.str
    }
    INameS::TopLevelStructDeclaration(n) => n.name.0.to_string(),
    INameS::TopLevelInterfaceDeclaration(n) => n.name.0.to_string(),
    INameS::FunctionDeclaration(n) => match n {
      IFunctionDeclarationNameS::FunctionName(fn_name) => format!("{}: {}", code_map(fn_name.code_location), fn_name.name.0),
      IFunctionDeclarationNameS::LambdaDeclarationName(_) => {
        panic!("implement: printable_name LambdaDeclarationName");
        // codeMap(codeLocation) + ": " + "(lambda)"
      }
      IFunctionDeclarationNameS::ConstructorName(_) => {
        panic!("implement: printable_name ConstructorName");
        // codeMap(range.begin) + ": " + name.str
      }
      _ => panic!("implement: printable_name FunctionDeclaration other"),
    },
    INameS::AnonymousSubstructTemplateName(_) => {
      panic!("implement: printable_name AnonymousSubstructTemplateName");
      // name.str + ".anonymous"
    }
    _ => panic!("implement: printable_name other"),
  }
}

fn printable_kind_name(kind: KindT) -> String {
  panic!("Unimplemented: printable_kind_name");
  // kind match {
  //   case IntT(bits) => "i" + bits
  //   case BoolT() => "bool"
  //   case FloatT() => "float"
  //   case StrT() => "str"
  //   case StructTT(f) => printableId(f)
  // }
}

fn printable_id<'s, 't>(id: IdT<'s, 't>) -> String {
  panic!("Unimplemented: printable_id");
  // id.localName match {
  //   case CitizenNameT(humanName, templateArgs) => humanName + (if (templateArgs.isEmpty) "" else "<" + templateArgs.map(_.toString.mkString) + ">")
  //   case x => x.toString
  // }
}

fn printable_var_name<'s, 't>(name: IVarNameT<'s, 't>) -> String {
  match name {
    IVarNameT::CodeVar(n) => n.name.0.to_string(),
    _ => panic!("implement: printable_var_name other"),
  }
}

fn get_file(function_a: FunctionA) -> FileCoordinate {
  panic!("Unimplemented: get_file");
  // functionA.range.file
}

fn humanize_rejection_reason<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, verbose: bool, code_map: &dyn Fn(CodeLocationS<'s>) -> String, lines_between: &dyn Fn(CodeLocationS<'s>, CodeLocationS<'s>) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS<'s>) -> String, invocation_range: &Vec<RangeS<'s>>, reason: &IFindFunctionFailureReason<'s, 't>) -> String {
  match reason {
    IFindFunctionFailureReason::FindFunctionResolveFailure { reason } => {
      humanize_resolving_error(scout_arena, typing_interner, verbose, code_map, lines_between, line_range_containing, line_containing, reason)
    }
    IFindFunctionFailureReason::RuleTypeSolveFailure { reason } => {
      let code_map_ref = |c: &CodeLocationS<'s>| code_map(*c);
      let lines_between_ref = |a: &CodeLocationS<'s>, b: &CodeLocationS<'s>| lines_between(*a, *b);
      let line_range_containing_ref = |c: &CodeLocationS<'s>| line_range_containing(*c);
      let line_containing_ref = |c: &CodeLocationS<'s>| line_containing(*c);
      let humanize_rule_error_fn = |rt_err: &IRuneTypeRuleError<'s>|
        humanize_rune_type_error(code_map, rt_err);
      solver_humanize_failed_solve(
        code_map_ref,
        lines_between_ref,
        line_range_containing_ref,
        line_containing_ref,
        humanize_rune,
        |tyype: ITemplataType<'s>| humanize_templata_type(&tyype),
        humanize_rule_error_fn,
        |rule: &IRulexSR<'s>| *rule.range(),
        |rule: &IRulexSR<'s>| rule.rune_usages().iter().map(|u| (u.rune, u.range)).collect(),
        |rule: &IRulexSR<'s>| rule.rune_usages().iter().map(|u| u.rune).collect(),
        humanize_rule,
        &reason.failed_solve,
      ).0
    }
    IFindFunctionFailureReason::WrongNumberOfArguments { supplied, expected } => {
      "Number of params doesn't match! Supplied ".to_string() + &supplied.to_string() + " but function takes " + &expected.to_string()
    }
    IFindFunctionFailureReason::WrongNumberOfTemplateArguments { supplied, expected } => {
      "Number of template params doesn't match! Supplied ".to_string() + &supplied.to_string() + " but function takes " + &expected.to_string()
    }
    IFindFunctionFailureReason::SpecificParamDoesntMatchExactly { index, argument, parameter } => {
      "Index ".to_string() + &index.to_string() + " argument " + &humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *argument }))) +
        " isn't the same exact type as expected parameter " + &humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *parameter })))
    }
    IFindFunctionFailureReason::SpecificParamDoesntSend { index, argument, parameter } => {
      " Index ".to_string() + &index.to_string() + " argument " + &humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *argument }))) +
        " can't be given to expected parameter " + &humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *parameter })))
    }
    IFindFunctionFailureReason::SpecificParamRegionDoesntMatch { .. } => panic!("implement: humanize_rejection_reason SpecificParamRegionDoesntMatch"),
    IFindFunctionFailureReason::SpecificParamVirtualityDoesntMatch { index } => {
      "Virtualities don't match at index ".to_string() + &index.to_string()
    }
    IFindFunctionFailureReason::InferFailure { .. } => panic!("implement: humanize_rejection_reason InferFailure"),
    IFindFunctionFailureReason::Outscored => panic!("implement: humanize_rejection_reason Outscored"),
    IFindFunctionFailureReason::CouldntEvaluateTemplateError { reason } => {
      "Couldn't evaluate template: ".to_string()
        + &humanize_defining_error(scout_arena, typing_interner, true, code_map, lines_between, line_range_containing, line_containing, reason)
    }
  }
}

pub fn humanize_rule_error<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, lines_between: &dyn Fn(CodeLocationS<'s>, CodeLocationS<'s>) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS<'s>) -> String, error: ITypingPassSolverError<'s, 't>) -> String {
  match error {
    ITypingPassSolverError::IsaFailed { sub, suuper } => {
      "Kind ".to_string()
        + &humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Kind(typing_interner.alloc(KindTemplataT { kind: sub })))
        + " does not implement interface "
        + &humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Kind(typing_interner.alloc(KindTemplataT { kind: suuper })))
    }
    ITypingPassSolverError::BadIsaSubKind { kind } => {
      "Kind ".to_string()
        + &humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Kind(typing_interner.alloc(KindTemplataT { kind })))
        + " cannot be a sub-kind."
    }
    ITypingPassSolverError::CantGetComponentsOfPlaceholderPrototype { .. } => panic!("implement: humanize_rule_error CantGetComponentsOfPlaceholderPrototype"),
    ITypingPassSolverError::ReturnTypeConflict { .. } => panic!("implement: humanize_rule_error ReturnTypeConflict"),
    ITypingPassSolverError::CantShareMutable { kind } => {
      "Can't share a mutable kind: ".to_string() + &humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Kind(typing_interner.alloc(KindTemplataT { kind })))
    }
    ITypingPassSolverError::BadIsaSuperKind { .. } => panic!("implement: humanize_rule_error BadIsaSuperKind"),
    ITypingPassSolverError::SendingNonIdenticalKinds { .. } => panic!("implement: humanize_rule_error SendingNonIdenticalKinds"),
    ITypingPassSolverError::SendingNonCitizen { .. } => panic!("implement: humanize_rule_error SendingNonCitizen"),
    ITypingPassSolverError::CantCheckPlaceholder { .. } => panic!("implement: humanize_rule_error CantCheckPlaceholder"),
    ITypingPassSolverError::CouldntFindFunction { .. } => panic!("implement: humanize_rule_error CouldntFindFunction"),
    ITypingPassSolverError::CouldntResolveKind { .. } => panic!("implement: humanize_rule_error CouldntResolveKind"),
    ITypingPassSolverError::WrongNumberOfTemplateArgs { expected_min_num_args, expected_max_num_args } => {
      if expected_min_num_args == expected_max_num_args {
        format!("Wrong number of template args, expected {}.", expected_min_num_args)
      } else {
        format!("Wrong number of template args, expected {} or {}.", expected_min_num_args, expected_max_num_args)
      }
    }
    ITypingPassSolverError::LookupFailed { .. } => panic!("implement: humanize_rule_error LookupFailed"),
    ITypingPassSolverError::KindIsNotConcrete { kind } => {
      "Expected kind to be concrete, but was not. Kind: ".to_string() + &humanize_kind(scout_arena, typing_interner, code_map, kind, None)
    }
    ITypingPassSolverError::OneOfFailed { .. } => panic!("implement: humanize_rule_error OneOfFailed"),
    ITypingPassSolverError::KindIsNotInterface { .. } => panic!("implement: humanize_rule_error KindIsNotInterface"),
    ITypingPassSolverError::CallResultIsntCallable { result } => {
      "Generic call result isn't callable: ".to_string() + &humanize_templata(scout_arena, typing_interner, code_map, result)
    }
    ITypingPassSolverError::CallResultWasntExpectedType { expected, actual } => {
      "Expected an instantiation of ".to_string() + &humanize_templata(scout_arena, typing_interner, code_map, expected) + " but got " + &humanize_templata(scout_arena, typing_interner, code_map, actual)
    }
    ITypingPassSolverError::OwnershipDidntMatch { coord, expected_ownership } => {
      "Given type ".to_string() + &humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord }))) +
        " doesn't have expected ownership " + &humanize_ownership(unevaluate_ownership(expected_ownership))
    }
    ITypingPassSolverError::ReceivingDifferentOwnerships { .. } => panic!("implement: humanize_rule_error ReceivingDifferentOwnerships"),
    ITypingPassSolverError::NoAncestorsSatisfyCall { params } => {
      "No ancestors satisfy call: ".to_string()
        + &params.iter().map(|(rune, coord)| {
            humanize_rune(*rune) + " = " + &humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *coord })))
          }).collect::<Vec<_>>().join(", ")
    }
    ITypingPassSolverError::KindIsNotStruct { .. } => panic!("implement: humanize_rule_error KindIsNotStruct"),
    ITypingPassSolverError::CouldntFindImpl { .. } => panic!("implement: humanize_rule_error CouldntFindImpl"),
    ITypingPassSolverError::CantSharePlaceholder { .. } => panic!("implement: humanize_rule_error CantSharePlaceholder"),
    ITypingPassSolverError::NoCommonAncestors { .. } => panic!("implement: humanize_rule_error NoCommonAncestors"),
    ITypingPassSolverError::CantDetermineNarrowestKind { .. } => panic!("implement: humanize_rule_error CantDetermineNarrowestKind"),
    ITypingPassSolverError::FunctionDoesntHaveName { .. } => panic!("implement: humanize_rule_error FunctionDoesntHaveName"),
    ITypingPassSolverError::InternalSolverError { range: _, err } => {
      match err {
        ISolverError::SolverConflict(c) => {
          "Solver conflict on rune ".to_string() + &humanize_rune(c.rune) + ": was " +
            &humanize_templata(scout_arena, typing_interner, code_map, c.previous_conclusion) +
            " but now concluding " + &humanize_templata(scout_arena, typing_interner, code_map, c.new_conclusion)
        }
        ISolverError::RuleError(r) => {
          humanize_rule_error(scout_arena, typing_interner, code_map, lines_between, line_range_containing, line_containing, r.err)
        }
        ISolverError::SolveIncomplete(_) => "Solve incomplete".to_string(),
      }
    }
  }
}

pub fn humanize_candidate_and_failed_solve<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, lines_between: &dyn Fn(CodeLocationS<'s>, CodeLocationS<'s>) -> Vec<RangeS<'s>>, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, line_containing: &dyn Fn(CodeLocationS<'s>) -> String, result: &FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>>) -> String {
  let (text, _line_begins) = solver_humanize_failed_solve(
    |loc| code_map(*loc),
    |a, b| lines_between(*a, *b),
    |loc| line_range_containing(*loc),
    |loc| line_containing(*loc),
    |rune| humanize_rune(rune),
    |t| humanize_templata(scout_arena, typing_interner, &|loc| code_map(loc), t),
    |err| humanize_rule_error(scout_arena, typing_interner, code_map, lines_between, line_range_containing, line_containing, *err),
    |rule: &IRulexSR<'s>| *rule.range(),
    |rule: &IRulexSR<'s>| rule.rune_usages().iter().map(|u| (u.rune, u.range)).collect(),
    |rule: &IRulexSR<'s>| rule.rune_usages().iter().map(|u| u.rune).collect(),
    |rule: &IRulexSR<'s>| humanize_rule(rule),
    result,
  );
  text
}

pub fn humanize_candidate<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>, candidate: &ICalleeCandidate<'s, 't>) -> String {
  match candidate {
    ICalleeCandidate::Header(HeaderCalleeCandidate { header }) => {
      humanize_id(scout_arena, typing_interner, code_map, header.id, None)
    }
    ICalleeCandidate::PrototypeTemplata(PrototypeTemplataCalleeCandidate { prototype_t }) => {
      humanize_name(scout_arena, typing_interner, code_map, prototype_t.id.local_name, None) + ":\n"
    }
    ICalleeCandidate::Function(FunctionCalleeCandidate { ft }) => {
      let begin = line_range_containing(ft.function.range.begin).begin;
      code_map(begin) + ":\n" +
        &format!("{:?}", line_range_containing(begin).begin) + "\n"
    }
  }
}

pub fn humanize_templata<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, templata: ITemplataT<'s, 't>) -> String {
  match templata {
    ITemplataT::RuntimeSizedArrayTemplate(_) => "Array".to_string(),
    ITemplataT::StaticSizedArrayTemplate(_) => "StaticArray".to_string(),
    ITemplataT::InterfaceDefinition(interface_def) => interface_def.origin_interface.name.name.0.to_string(),
    ITemplataT::StructDefinition(struct_def) => humanize_name_for_struct_declaration(struct_def.origin_struct.name),
    ITemplataT::Integer(value) => value.to_string(),
    ITemplataT::Ownership(ownership) => match ownership.ownership {
      OwnershipT::Own => "own".to_string(),
      OwnershipT::Borrow => "borrow".to_string(),
      OwnershipT::Weak => "weak".to_string(),
      OwnershipT::Share => "share".to_string(),
    },
    ITemplataT::Prototype(prototype_templata) => humanize_id(scout_arena, typing_interner, code_map, prototype_templata.prototype.id, None),
    ITemplataT::Coord(coord_templata) => humanize_coord(scout_arena, typing_interner, code_map, coord_templata.coord),
    ITemplataT::Kind(kind_templata) => humanize_kind(scout_arena, typing_interner, code_map, kind_templata.kind, None),
    ITemplataT::CoordList(coord_list) => {
      "(".to_string() + &coord_list.coords.iter().map(|c| humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *c })))).collect::<Vec<_>>().join(", ") + ")"
    }
    ITemplataT::String(value) => panic!("implement: humanize_templata String"),
    ITemplataT::Placeholder(p) => match p.tyype {
      ITemplataType::CoordTemplataType(_) => "$".to_string() + &humanize_id(scout_arena, typing_interner, code_map, p.id, None),
      _ => crate::postparsing::post_parser_error_humanizer::humanize_templata_type(&p.tyype) + "$" + &humanize_id(scout_arena, typing_interner, code_map, p.id, None),
    },
    _ => panic!("implement: humanize_templata other"),
  }
}

fn humanize_coord<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, coord: CoordT<'s, 't>) -> String {
  let CoordT { ownership, region, kind, .. } = coord;
  let ownership_str = match ownership {
    OwnershipT::Own => "",
    OwnershipT::Share => "",
    OwnershipT::Borrow => "&",
    OwnershipT::Weak => "&&",
  };
  let kind_str = humanize_kind(scout_arena, typing_interner, code_map, kind, Some(region));
  format!("{}{}", ownership_str, kind_str)
}

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
      humanize_imprecise_name(*s.name)),
    KindT::Interface(name) => humanize_id(scout_arena, typing_interner, code_map, name.id, containing_region),
    KindT::Struct(name) => humanize_id(scout_arena, typing_interner, code_map, name.id, containing_region),
    KindT::RuntimeSizedArray(rsa) => {
      "Array<".to_string() +
        &humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: rsa.element_type() }))) +
        ">"
    }
    KindT::StaticSizedArray(ssa) => format!("StaticSizedArray({:?})", ssa.name),
  }
}

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

pub fn humanize_name<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, name: INameT<'s, 't>, containing_region: Option<RegionT>) -> String {
  match name {
    INameT::AnonymousSubstructConstructor(n) => {
      panic!("implement: humanize_name AnonymousSubstructConstructor");
      // humanizeName(codeMap, template) +
      //   "<" + templateArgs.map(humanizeTemplata(codeMap, _)).mkString(", ") + ">" +
      //   "(" + parameters.map(CoordTemplataT).map(humanizeTemplata(codeMap, _)).mkString(", ") + ")"
    }
    INameT::AnonymousSubstructConstructorTemplate(n) => {
      panic!("implement: humanize_name AnonymousSubstructConstructorTemplate");
      // "asc:" + humanizeName(codeMap, substruct)
    }
    INameT::Self_(_) => "self".to_string(),
    INameT::OverrideDispatcherTemplate(n) => {
      panic!("implement: humanize_name OverrideDispatcherTemplate");
      // "ovdt:" + humanizeId(codeMap, implId, None)
    }
    INameT::OverrideDispatcher(n) => {
      panic!("implement: humanize_name OverrideDispatcher");
      // "ovd:" + humanizeId(codeMap, implId, None) +
      //   humanizeGenericArgs(codeMap, templateArgs, None) +
      //   "(" + parameters.map(CoordTemplataT).map(humanizeTemplata(codeMap, _)).mkString(", ") + ")"
    }
    INameT::Iterator(n) => {
      panic!("implement: humanize_name Iterator");
      // "it:" + codeMap(range.begin)
    }
    INameT::Iterable(n) => {
      panic!("implement: humanize_name Iterable");
      // "ib:" + codeMap(range.begin)
    }
    INameT::IterationOption(n) => {
      panic!("implement: humanize_name IterationOption");
      // "io:" + codeMap(range.begin)
    }
    INameT::ImplTemplate(n) => {
      panic!("implement: humanize_name ImplTemplate");
      // "implt:" + codeMap(codeLoc)
    }
    INameT::ForwarderFunction(n) => {
      panic!("implement: humanize_name ForwarderFunction");
      // humanizeName(codeMap, inner)
    }
    INameT::ForwarderFunctionTemplate(n) => {
      let inner_name = match n.inner {
        IFunctionTemplateNameT::FunctionTemplate(r) => INameT::FunctionTemplate(r),
        IFunctionTemplateNameT::ForwarderFunctionTemplate(r) => INameT::ForwarderFunctionTemplate(r),
        IFunctionTemplateNameT::ConstructorTemplate(r) => INameT::ConstructorTemplate(r),
        IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(r) => INameT::AnonymousSubstructConstructorTemplate(r),
        IFunctionTemplateNameT::LambdaCallFunctionTemplate(r) => INameT::LambdaCallFunctionTemplate(r),
        IFunctionTemplateNameT::OverrideDispatcherTemplate(r) => INameT::OverrideDispatcherTemplate(r),
        IFunctionTemplateNameT::ExternFunction(r) => INameT::ExternFunction(r),
        IFunctionTemplateNameT::FunctionBoundTemplate(r) => INameT::FunctionBoundTemplate(r),
        IFunctionTemplateNameT::PredictedFunctionTemplate(r) => INameT::PredictedFunctionTemplate(r),
      };
      format!("fwd{}:{}", n.index, humanize_name(scout_arena, typing_interner, code_map, inner_name, containing_region))
    }
    INameT::MagicParam(n) => {
      panic!("implement: humanize_name MagicParam");
      // "mp:" + codeMap(codeLoc)
    }
    INameT::ClosureParam(n) => {
      panic!("implement: humanize_name ClosureParam");
      // "λP:" + codeMap(codeLocation)
    }
    INameT::ConstructingMember(n) => {
      panic!("implement: humanize_name ConstructingMember");
      // "cm:" + name
    }
    INameT::TypingPassBlockResultVar(n) => {
      panic!("implement: humanize_name TypingPassBlockResultVar");
      // "b:" + life
    }
    INameT::TypingPassFunctionResultVar(n) => {
      panic!("implement: humanize_name TypingPassFunctionResultVar");
      // "(result)"
    }
    INameT::TypingPassTemporaryVar(n) => format!("t:{:?}", n.life),
    INameT::FunctionBoundTemplate(n) => n.human_name.0.to_string(),
    INameT::LambdaCallFunctionTemplate(n) => "λF:".to_string() + &code_map(n.code_location),
    INameT::LambdaCitizenTemplate(n) => "λC:".to_string() + &code_map(n.code_location),
    INameT::LambdaCallFunction(n) => {
      humanize_name(scout_arena, typing_interner, code_map, INameT::LambdaCallFunctionTemplate(n.template), None) +
        &humanize_generic_args(scout_arena, typing_interner, code_map, n.template_args, None) +
        "(" + &n.parameters.iter().map(|p| humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *p })))).collect::<Vec<_>>().join(", ") + ")"
    }
    INameT::FunctionBound(n) => {
      humanize_name(scout_arena, typing_interner, code_map, INameT::FunctionBoundTemplate(n.template), None) +
        &humanize_generic_args(scout_arena, typing_interner, code_map, n.template_args, containing_region) +
        "(" + &n.parameters.iter().map(|p| humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *p })))).collect::<Vec<_>>().join(", ") + ")"
    }
    INameT::KindPlaceholder(n) => humanize_name(scout_arena, typing_interner, code_map, INameT::KindPlaceholderTemplate(n.template), None),
    INameT::KindPlaceholderTemplate(n) => crate::postparsing::post_parser_error_humanizer::humanize_rune(n.rune),
    INameT::CodeVar(n) => n.name.0.to_string(),
    INameT::LambdaCitizen(n) => humanize_name(scout_arena, typing_interner, code_map, INameT::LambdaCitizenTemplate(n.template), containing_region) + "<>",
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
    INameT::AnonymousSubstruct(n) => {
      humanize_name(scout_arena, typing_interner, code_map, INameT::InterfaceTemplate(match n.template.interface { IInterfaceTemplateNameT::InterfaceTemplate(t) => t }), containing_region) +
        &humanize_generic_args(scout_arena, typing_interner, code_map, n.template_args, containing_region)
    }
    INameT::AnonymousSubstructTemplate(n) => {
      humanize_name(scout_arena, typing_interner, code_map, INameT::InterfaceTemplate(match n.interface { IInterfaceTemplateNameT::InterfaceTemplate(t) => t }), containing_region) + ".anonymous"
    }
    INameT::StructTemplate(n) => n.human_name.0.to_string(),
    INameT::InterfaceTemplate(n) => n.human_namee.0.to_string(),
    INameT::NonKindNonRegionPlaceholder(n) => crate::postparsing::post_parser_error_humanizer::humanize_rune(n.rune),
    INameT::PredictedFunction(n) => {
      humanize_name(scout_arena, typing_interner, code_map, INameT::PredictedFunctionTemplate(n.template), None) +
        &humanize_generic_args(scout_arena, typing_interner, code_map, n.template_args, containing_region) +
        "(" + &n.parameters.iter().map(|p| humanize_templata(scout_arena, typing_interner, code_map, ITemplataT::Coord(typing_interner.alloc(CoordTemplataT { coord: *p })))).collect::<Vec<_>>().join(", ") + ")"
    }
    INameT::PredictedFunctionTemplate(n) => n.human_name.0.to_string(),
    _ => panic!("implement: humanize_name other"),
  }
}

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
      .chain(once(last_str))
      .collect::<Vec<_>>().join(", ");
    format!("<{}>", parts)
  }
}

pub fn humanize_signature<'s, 't>(scout_arena: &ScoutArena<'s>, typing_interner: &TypingInterner<'s, 't>, code_map: &dyn Fn(CodeLocationS<'s>) -> String, signature: SignatureT<'s, 't>) -> String {
  humanize_id(scout_arena, typing_interner, code_map, signature.id, None)
}

