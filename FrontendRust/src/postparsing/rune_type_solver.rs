/*
package dev.vale.postparsing

import dev.vale.{Err, Interner, Ok, RangeS, Result, vassert, vassertSome, vfail, vpass, vwat}
import dev.vale.postparsing.rules._
import dev.vale.solver.{FailedSolve, ISolverError, RuleError, SimpleSolverState, SolveIncomplete, Solver, SolverConflict}
import dev.vale._
import dev.vale.postparsing.RuneTypeSolver._
import dev.vale.postparsing.rules._

import scala.collection.immutable.Map
*/
use crate::postparsing::itemplatatype::{ITemplataType, CoordTemplataType, KindTemplataType};
use crate::postparsing::names::{IRuneS, IImpreciseNameS, IImpreciseNameValS, RuneNameValS};
use crate::postparsing::ast::GenericParameterS;
use crate::postparsing::rules::rules::{IRulexSR, RuneUsage};
use crate::scout_arena::ScoutArena;
use crate::solver::{FailedSolve, ISolverError, SimpleSolverState, SolveIncomplete, RuleError, make_solver_state};
use crate::utils::range::RangeS;
use crate::postparsing::itemplatatype::*;
use std::collections::HashMap;
use crate::postparsing::itemplatatype::ImplTemplataType;
use std::collections::HashSet;
use std::marker::PhantomData;


// mig: struct RuneTypeSolveError
#[derive(Debug)]
pub struct RuneTypeSolveError<'s> {
  pub range: Vec<RangeS<'s>>,
  pub failed_solve: FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataType<'s>, IRuneTypeRuleError<'s>>,
}
/*
case class RuneTypeSolveError(range: List[RangeS], failedSolve: FailedSolve[IRulexSR, IRuneS, ITemplataType, IRuneTypeRuleError]) {
  vpass()
}
*/
// mig: impl RuneTypeSolveError
// mig: enum IRuneTypeRuleError
#[derive(Debug)]
pub enum IRuneTypeRuleError<'s> {
  FoundCitizenDidntMatchExpectedType(FoundCitizenDidntMatchExpectedType<'s>),
  FoundTemplataDidntMatchExpectedType(FoundTemplataDidntMatchExpectedType<'s>),
  NotEnoughArgumentsForGenericCall(NotEnoughArgumentsForGenericCall<'s>),
  GenericCallArgTypeMismatch(GenericCallArgTypeMismatch<'s>),
  TooManyMatchingTypes(RuneTypingTooManyMatchingTypes<'s>),
  CouldntFindType(RuneTypingCouldntFindType<'s>),
}
impl<'s> IRuneTypeRuleError<'s> {
  pub fn as_lookup_failed(self) -> Option<IRuneTypingLookupFailedError<'s>> {
    match self {
      IRuneTypeRuleError::TooManyMatchingTypes(x) => Some(IRuneTypingLookupFailedError::TooManyMatchingTypes(x)),
      IRuneTypeRuleError::CouldntFindType(x) => Some(IRuneTypingLookupFailedError::CouldntFindType(x)),
      _ => None,
    }
  }
  /*
  Guardian: disable-all
  */
}
/*
Guardian: disable-all
*/

impl<'s> From<IRuneTypingLookupFailedError<'s>> for IRuneTypeRuleError<'s> {
  fn from(e: IRuneTypingLookupFailedError<'s>) -> Self {
    match e {
      IRuneTypingLookupFailedError::TooManyMatchingTypes(x) => IRuneTypeRuleError::TooManyMatchingTypes(x),
      IRuneTypingLookupFailedError::CouldntFindType(x) => IRuneTypeRuleError::CouldntFindType(x),
    }
  }
}
/*
sealed trait IRuneTypeRuleError
*/
// mig: struct FoundCitizenDidntMatchExpectedType
#[derive(Debug)]
pub struct FoundCitizenDidntMatchExpectedType<'s> {
  pub range: Vec<RangeS<'s>>,
  pub expected_type: ITemplataType<'s>,
  pub actual_type: ITemplataType<'s>,
}
/*
case class FoundCitizenDidntMatchExpectedType(
  range: List[RangeS],
  expectedType: ITemplataType,
  actualType: ITemplataType
) extends IRuneTypeRuleError
*/
// mig: impl FoundCitizenDidntMatchExpectedType
// mig: struct FoundTemplataDidntMatchExpectedType
#[derive(Debug)]
pub struct FoundTemplataDidntMatchExpectedType<'s> {
  pub range: Vec<RangeS<'s>>,
  pub expected_type: ITemplataType<'s>,
  pub actual_type: ITemplataType<'s>,
}
/*
case class FoundTemplataDidntMatchExpectedType(
  range: List[RangeS],
  expectedType: ITemplataType,
  actualType: ITemplataType
) extends IRuneTypeRuleError {
  vpass()
}
*/
// mig: impl FoundTemplataDidntMatchExpectedType
// mig: struct NotEnoughArgumentsForGenericCall
#[derive(Debug)]
pub struct NotEnoughArgumentsForGenericCall<'s> {
  pub range: Vec<RangeS<'s>>,
  pub index_of_non_defaulting_param: i32,
}
/*
case class NotEnoughArgumentsForGenericCall(
  range: List[RangeS],
//  citizen: ICitizenS,
  indexOfNonDefaultingParam: Int
) extends IRuneTypeRuleError {
  vpass()
}
  */
// mig: impl NotEnoughArgumentsForGenericCall
// mig: struct GenericCallArgTypeMismatch
#[derive(Debug)]
pub struct GenericCallArgTypeMismatch<'s> {
  pub range: Vec<RangeS<'s>>,
  pub expected_type: ITemplataType<'s>,
  pub actual_type: ITemplataType<'s>,
  pub param_index: i32,
}
/*
case class GenericCallArgTypeMismatch(
  range: List[RangeS],
//  citizen: ICitizenS,
  expectedType: ITemplataType,
  actualType: ITemplataType,
  paramIndex: Int
) extends IRuneTypeRuleError
*/
// mig: impl GenericCallArgTypeMismatch
// mig: enum IRuneTypingLookupFailedError
pub enum IRuneTypingLookupFailedError<'s> {
  TooManyMatchingTypes(RuneTypingTooManyMatchingTypes<'s>),
  CouldntFindType(RuneTypingCouldntFindType<'s>),
}
/*
sealed trait IRuneTypingLookupFailedError extends IRuneTypeRuleError
*/
// mig: struct RuneTypingTooManyMatchingTypes
#[derive(Debug)]
pub struct RuneTypingTooManyMatchingTypes<'s> {
  pub range: RangeS<'s>,
  pub name: IImpreciseNameS<'s>,
}
// mig: impl RuneTypingTooManyMatchingTypes
impl<'s> RuneTypingTooManyMatchingTypes<'s> {
/*
case class RuneTypingTooManyMatchingTypes(range: RangeS, name: IImpreciseNameS) extends IRuneTypingLookupFailedError {
*/
// mig: fn equals
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
} // end impl RuneTypingTooManyMatchingTypes
/*
  override def hashCode(): Int = vcurious()
}
*/
// mig: struct RuneTypingCouldntFindType
#[derive(Debug)]
pub struct RuneTypingCouldntFindType<'s> {
  pub range: RangeS<'s>,
  pub name: IImpreciseNameS<'s>,
}
// mig: impl RuneTypingCouldntFindType
impl<'s> RuneTypingCouldntFindType<'s> {
/*
case class RuneTypingCouldntFindType(range: RangeS, name: IImpreciseNameS) extends IRuneTypingLookupFailedError {
*/
// mig: fn equals
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
} // end impl RuneTypingCouldntFindType
/*
  override def hashCode(): Int = vcurious()
}
*/
// mig: struct FoundTemplataDidntMatchExpectedTypeA
#[derive(Debug)]
pub struct FoundTemplataDidntMatchExpectedTypeA<'s> {
  pub range: Vec<RangeS<'s>>,
  pub expected_type: ITemplataType<'s>,
  pub actual_type: ITemplataType<'s>,
}
/*
case class FoundTemplataDidntMatchExpectedTypeA(
    range: List[RangeS],
    expectedType: ITemplataType,
    actualType: ITemplataType
) extends IRuneTypingLookupFailedError {
  vpass()
}
*/
// mig: impl FoundTemplataDidntMatchExpectedTypeA
// mig: struct FoundPrimitiveDidntMatchExpectedType
pub struct FoundPrimitiveDidntMatchExpectedType<'s> {
  pub range: Vec<RangeS<'s>>,
  pub expected_type: ITemplataType<'s>,
  pub actual_type: ITemplataType<'s>,
}
/*
case class FoundPrimitiveDidntMatchExpectedType(
  range: List[RangeS],
  expectedType: ITemplataType,
  actualType: ITemplataType
) extends IRuneTypingLookupFailedError {
  vpass()
}
*/
// mig: impl FoundPrimitiveDidntMatchExpectedType
// mig: enum IRuneTypeSolverLookupResult
#[derive(PartialEq)]
pub enum IRuneTypeSolverLookupResult<'s> {
  Primitive(PrimitiveRuneTypeSolverLookupResult<'s>),
  Citizen(CitizenRuneTypeSolverLookupResult<'s>),
  Templata(TemplataLookupResult<'s>),
}
/*
sealed trait IRuneTypeSolverLookupResult
*/
// mig: struct PrimitiveRuneTypeSolverLookupResult
#[derive(PartialEq)]
pub struct PrimitiveRuneTypeSolverLookupResult<'s> {
  pub tyype: ITemplataType<'s>,
}
/*
case class PrimitiveRuneTypeSolverLookupResult(tyype: ITemplataType) extends IRuneTypeSolverLookupResult
*/
// mig: impl PrimitiveRuneTypeSolverLookupResult
// mig: struct CitizenRuneTypeSolverLookupResult
#[derive(PartialEq)]
pub struct CitizenRuneTypeSolverLookupResult<'s> {
  pub tyype: ITemplataType<'s>,
  pub generic_params: &'s [&'s GenericParameterS<'s>],
}
/*
case class CitizenRuneTypeSolverLookupResult(tyype: TemplateTemplataType, genericParams: Vector[GenericParameterS]) extends IRuneTypeSolverLookupResult
*/
// mig: impl CitizenRuneTypeSolverLookupResult
// mig: struct TemplataLookupResult
#[derive(PartialEq)]
pub struct TemplataLookupResult<'s> {
  pub templata: ITemplataType<'s>,
}
/*
case class TemplataLookupResult(templata: ITemplataType) extends IRuneTypeSolverLookupResult
*/
// mig: impl TemplataLookupResult
// mig: trait IRuneTypeSolverEnv
pub trait IRuneTypeSolverEnv<'s> {
  fn lookup(
    &self,
    range: RangeS<'s>,
    name: IImpreciseNameS<'s>,
  ) -> Result<IRuneTypeSolverLookupResult<'s>, IRuneTypingLookupFailedError<'s>>;
}
/*
trait IRuneTypeSolverEnv {
  // MIGALLOW: lookup -> lookup_rune_type
  def lookup(range: RangeS, name: IImpreciseNameS):
  Result[IRuneTypeSolverLookupResult, IRuneTypingLookupFailedError]
}
*/

// mig: struct RuneTypeSolver
pub struct RuneTypeSolver<'s, 'ctx> {
  pub scout_arena: &'ctx ScoutArena<'s>,
}
/*
class RuneTypeSolver(interner: Interner) {
*/
// mig: impl RuneTypeSolver
impl<'s, 'ctx> RuneTypeSolver<'s, 'ctx> {
  pub fn solve_rune_type<E: IRuneTypeSolverEnv<'s>>(
    &self,
    sanity_check: bool,
    env: &E,
    range: Vec<RangeS<'s>>,
    predicting: bool,
    rules_s: &[IRulexSR<'s>],
    additional_runes: &[IRuneS<'s>],
    expect_complete_solve: bool,
    unpreprocessed_initially_known_runes: HashMap<IRuneS<'s>, ITemplataType<'s>>,
  ) -> Result<
    HashMap<IRuneS<'s>, ITemplataType<'s>>,
    RuneTypeSolveError<'s>,
  > {
    solve_rune_type(self.scout_arena, sanity_check, env, range, predicting, rules_s, additional_runes, expect_complete_solve, unpreprocessed_initially_known_runes)
  }
  /* Guardian: disable-all */
}
// mig: fn get_runes_rune_type
fn get_runes_rune_type<'s>(
  rule: &IRulexSR<'s>,
) -> Vec<IRuneS<'s>> {
  rule.rune_usages().iter().map(|ru| ru.rune.clone()).collect()
}
/*
  // MIGALLOW: getRunes -> get_runes_rune_type
  def getRunes(rule: IRulexSR): Vector[IRuneS] = {
    val sanityCheck: Vector[RuneUsage] =
      rule match {
        case MaybeCoercingLookupSR(range, rune, literal) => Vector(rune)
        case LookupSR(range, rune, literal) => Vector(rune)
        case RuneParentEnvLookupSR(range, rune) => Vector(rune)
        case EqualsSR(range, left, right) => Vector(left, right)
        case DefinitionCoordIsaSR(range, result, sub, suuper) => Vector(result, sub, suuper)
        case CallSiteCoordIsaSR(range, result, sub, suuper) => result.toVector ++ Vector(sub, suuper)
        case KindComponentsSR(range, resultRune, mutabilityRune) => Vector(resultRune, mutabilityRune)
        case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => Vector(resultRune, ownershipRune, kindRune)
        case PrototypeComponentsSR(range, resultRune, paramsRune, returnRune) => Vector(resultRune, paramsRune, returnRune)
        case ResolveSR(range, resultRune, name, paramsListRune, returnRune) => Vector(resultRune, paramsListRune, returnRune)
        case CallSiteFuncSR(range, resultRune, name, paramsListRune, returnRune) => Vector(resultRune, paramsListRune, returnRune)
        case DefinitionFuncSR(range, resultRune, name, paramsListRune, returnRune) => Vector(resultRune, paramsListRune, returnRune)
        case OneOfSR(range, rune, literals) => Vector(rune)
        case IsConcreteSR(range, rune) => Vector(rune)
        case IsInterfaceSR(range, rune) => Vector(rune)
        case IsStructSR(range, rune) => Vector(rune)
        case CoerceToCoordSR(range, coordRune, kindRune) => Vector(coordRune, kindRune)
        case LiteralSR(range, rune, literal) => Vector(rune)
        case AugmentSR(range, resultRune, ownership, innerRune) => Vector(resultRune, innerRune)
        case MaybeCoercingCallSR(range, resultRune, templateRune, args) => Vector(resultRune, templateRune) ++ args
//        case PrototypeSR(range, resultRune, name, parameters, returnTypeRune) => Vector(resultRune, returnTypeRune) ++ parameters
        case PackSR(range, resultRune, members) => Vector(resultRune) ++ members
//        case StaticSizedArraySR(range, resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune) => Vector(resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune)
//        case RuntimeSizedArraySR(range, resultRune, mutabilityRune, elementRune) => Vector(resultRune, mutabilityRune, elementRune)
//        case ManualSequenceSR(range, resultRune, elements) => Vector(resultRune) ++ elements
        case RefListCompoundMutabilitySR(range, resultRune, coordListRune) => Vector(resultRune, coordListRune)
//        case CoordListSR(range, resultRune, elements) => Vector(resultRune) ++ elements
      }
    val result = rule.runeUsages
    vassert(result.map(_.rune) sameElements sanityCheck.map(_.rune))
    result.map(_.rune)
  }
*/
// mig: fn get_puzzles_rune_type
fn get_puzzles_rune_type<'s>(
  predicting: bool,
  rule: &IRulexSR<'s>,
) -> Vec<Vec<IRuneS<'s>>> {

  match rule {
    IRulexSR::Equals(x) => vec![vec![x.left.rune.clone()], vec![x.right.rune.clone()]],
    IRulexSR::Lookup(_x) => {
      if predicting {
        vec![]
      } else {
        vec![vec![]]
      }
    }
    IRulexSR::MaybeCoercingLookup(x) => {
      if predicting {
        vec![]
      } else {
        vec![vec![x.rune.rune.clone()]]
      }
    }
    IRulexSR::RuneParentEnvLookup(x) => {
      if predicting {
        vec![]
      } else {
        vec![vec![x.rune.rune.clone()]]
      }
    }
    IRulexSR::MaybeCoercingCall(x) => {
      vec![vec![x.result_rune.rune.clone(), x.template_rune.rune.clone()]]
    }
    IRulexSR::Pack(_) => vec![vec![]],
    IRulexSR::DefinitionCoordIsa(_) => vec![vec![]],
    IRulexSR::CallSiteCoordIsa(_) => vec![vec![]],
    IRulexSR::KindComponents(_) => vec![vec![]],
    IRulexSR::CoordComponents(_) => vec![vec![]],
    IRulexSR::PrototypeComponents(_) => vec![vec![]],
    IRulexSR::Resolve(_) => vec![vec![]],
    IRulexSR::CallSiteFunc(_) => vec![vec![]],
    IRulexSR::DefinitionFunc(_) => vec![vec![]],
    IRulexSR::OneOf(_) => vec![vec![]],
    IRulexSR::IsConcrete(x) => vec![vec![x.rune.rune.clone()]],
    IRulexSR::IsInterface(_) => vec![vec![]],
    IRulexSR::IsStruct(_) => vec![vec![]],
    IRulexSR::CoerceToCoord(_) => vec![vec![]],
    IRulexSR::Literal(_) => vec![vec![]],
    IRulexSR::Augment(_) => vec![vec![]],
    IRulexSR::RefListCompoundMutability(_) => vec![vec![]],
    IRulexSR::Call(_) => panic!("IRulexSR::Call not yet migrated in rune_type get_puzzles"),
    IRulexSR::CoordSend(_) => panic!("IRulexSR::CoordSend not yet migrated in rune_type get_puzzles"),
    IRulexSR::IndexList(_) => panic!("IRulexSR::IndexList not yet migrated in rune_type get_puzzles"),
  }
}
/*
  // MIGALLOW: getPuzzles -> get_puzzles_rune_type
  def getPuzzles(predicting: Boolean, rule: IRulexSR): Vector[Vector[IRuneS]] = {
    rule match {
      case EqualsSR(range, leftRune, rightRune) => Vector(Vector(leftRune.rune), Vector(rightRune.rune))
      case LookupSR(range, rune, _) => {
        // If the type might be ambiguous, we would have done a MaybeCoercingLookupSR.

        if (predicting) {
          // This Vector() means nothing can solve this puzzle.
          // We dont want to do a lookup when we're just predicting.
          Vector()
        } else {
          // Vector(Vector()) because we can solve it immediately, by just doing the lookup.
          Vector(Vector())
        }
      }
      case MaybeCoercingLookupSR(range, rune, _) => {
        if (predicting) {
          // This Vector() literally means nothing can solve this puzzle.
          // It needs to be passed in via plan/solve's initiallyKnownRunes parameter.
          Vector()
        } else {
          // We need to know the type beforehand, because we don't know if we'll be coercing or not.
          Vector(Vector(rune.rune))
        }
      }
      case RuneParentEnvLookupSR(range, rune) => {
        if (predicting) {
          // This Vector() literally means nothing can solve this puzzle.
          // It needs to be passed in via plan/solve's initiallyKnownRunes parameter.
          Vector()
        } else {
          // We need to know the type beforehand, because we don't know if we'll be coercing or not.
          Vector(Vector(rune.rune))
        }
      }
      case MaybeCoercingCallSR(range, resultRune, templateRune, args) => {
        // We can't determine the template from the result and args because we might be coercing its
        // returned kind to a coord. So we need the template.
        // We can't determine the return type because we don't know whether we're coercing or not.
        Vector(Vector(resultRune.rune, templateRune.rune))
      }
      case PackSR(range, resultRune, members) => {
        // Packs are always lists of coords
        Vector(Vector())
      }
      case DefinitionCoordIsaSR(range, resultRune, subRune, superRune) => Vector(Vector())
      case CallSiteCoordIsaSR(range, resultRune, subRune, superRune) => Vector(Vector())
      case KindComponentsSR(range, resultRune, mutabilityRune) => Vector(Vector())
      case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => Vector(Vector())
      case PrototypeComponentsSR(range, resultRune, paramsRune, returnRune) => Vector(Vector())
      case ResolveSR(range, resultRune, nameRune, paramsListRune, returnRune) => Vector(Vector())
      case CallSiteFuncSR(range, resultRune, nameRune, paramsListRune, returnRune) => Vector(Vector())
      case DefinitionFuncSR(range, resultRune, name, paramsListRune, returnRune) => Vector(Vector())
      case OneOfSR(range, rune, literals) => Vector(Vector())
      case IsConcreteSR(range, rune) => Vector(Vector(rune.rune))
      case IsInterfaceSR(range, rune) => Vector(Vector())
      case IsStructSR(range, rune) => Vector(Vector())
      case CoerceToCoordSR(range, coordRune, kindRune) => Vector(Vector())
      case LiteralSR(range, rune, literal) => Vector(Vector())
      case AugmentSR(range, resultRune, ownership, innerRune) => Vector(Vector())
//      case StaticSizedArraySR(range, resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune) => Vector(Vector(resultRune.rune))
//      case RuntimeSizedArraySR(range, resultRune, mutabilityRune, elementRune) => Vector(Vector(resultRune.rune))
//      case ManualSequenceSR(range, resultRune, elements) => Vector(Vector(resultRune.rune))
      case RefListCompoundMutabilitySR(range, resultRune, coordListRune) => Vector(Vector())
        // solverState.addPuzzle(ruleIndex, Vector(senderRune, receiverRune))
//      case CoordListSR(range, resultRune, elements) => Vector(Vector())
    }
  }
*/
// mig: fn solve_rule

fn solve_rule<'s, E: IRuneTypeSolverEnv<'s>>(
  scout_arena: &ScoutArena<'s>,
  env: &E,
  rule_index: i32,
  rule: &IRulexSR<'s>,
  solver_state: &mut SimpleSolverState<
    IRulexSR<'s>,
    IRuneS<'s>,
    ITemplataType<'s>,
  >,
) -> Result<(), ISolverError<
  IRuneS<'s>,
  ITemplataType<'s>,
  IRuneTypeRuleError<'s>,
>> {


  match rule {
    IRulexSR::KindComponents(x) => {
      solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], [
        (x.kind_rune.rune.clone(), ITemplataType::KindTemplataType(KindTemplataType {})),
        (x.mutability_rune.rune.clone(), ITemplataType::MutabilityTemplataType(MutabilityTemplataType {})),
      ].into_iter().collect(), vec![], HashSet::new())
    }
    IRulexSR::CoordComponents(x) => {
      solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], [
        (x.result_rune.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {})),
        (x.ownership_rune.rune.clone(), ITemplataType::OwnershipTemplataType(OwnershipTemplataType {})),
        (x.kind_rune.rune.clone(), ITemplataType::KindTemplataType(KindTemplataType {})),
      ].into_iter().collect(), vec![], HashSet::new())
    }
    IRulexSR::PrototypeComponents(x) => {
      solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], [
        (x.result_rune.rune.clone(), ITemplataType::PrototypeTemplataType(PrototypeTemplataType {})),
        (x.params_rune.rune.clone(), ITemplataType::PackTemplataType(PackTemplataType { element_type: scout_arena.alloc(ITemplataType::CoordTemplataType(CoordTemplataType {})) })),
        (x.return_rune.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {})),
      ].into_iter().collect(), vec![], HashSet::new())
    }
    IRulexSR::MaybeCoercingCall(x) => {
      match solver_state.get_conclusion(&x.template_rune.rune).expect("MaybeCoercingCallSR: template rune has no conclusion") {
        ITemplataType::TemplateTemplataType(TemplateTemplataType { param_types, return_type: _ }) => {
          let conclusions: HashMap<IRuneS<'s>, ITemplataType<'s>> = x.args.iter().map(|a| a.rune.clone()).zip(param_types.iter().cloned()).collect();
          solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], conclusions, vec![], HashSet::new())
        }
        other => panic!("MaybeCoercingCallSR: unexpected template type: {:?}", other),
      }
    }
    IRulexSR::Resolve(x) => {
      solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], [
        (x.result_rune.rune.clone(), ITemplataType::PrototypeTemplataType(PrototypeTemplataType {})),
        (x.params_list_rune.rune.clone(), ITemplataType::PackTemplataType(PackTemplataType { element_type: scout_arena.alloc(ITemplataType::CoordTemplataType(CoordTemplataType {})) })),
        (x.return_rune.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {})),
      ].into_iter().collect(), vec![], HashSet::new())
    }
    IRulexSR::CallSiteFunc(x) => {
      solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], [
        (x.prototype_rune.rune.clone(), ITemplataType::PrototypeTemplataType(PrototypeTemplataType {})),
        (x.params_list_rune.rune.clone(), ITemplataType::PackTemplataType(PackTemplataType { element_type: scout_arena.alloc(ITemplataType::CoordTemplataType(CoordTemplataType {})) })),
        (x.return_rune.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {})),
      ].into_iter().collect(), vec![], HashSet::new())
    }
    IRulexSR::DefinitionFunc(x) => {
      solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], [
        (x.result_rune.rune.clone(), ITemplataType::PrototypeTemplataType(PrototypeTemplataType {})),
        (x.params_list_rune.rune.clone(), ITemplataType::PackTemplataType(PackTemplataType { element_type: scout_arena.alloc(ITemplataType::CoordTemplataType(CoordTemplataType {})) })),
        (x.return_rune.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {})),
      ].into_iter().collect(), vec![], HashSet::new())
    }
    IRulexSR::DefinitionCoordIsa(x) => {
        solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], [
            (x.result_rune.rune.clone(), ITemplataType::ImplTemplataType(ImplTemplataType {})),
            (x.sub_rune.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {})),
            (x.super_rune.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {})),
        ].into_iter().collect(), vec![], HashSet::new())
    }
    IRulexSR::CallSiteCoordIsa(x) => {
        let mut conclusions: HashMap<IRuneS<'s>, ITemplataType<'s>> = [
            (x.sub_rune.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {})),
            (x.super_rune.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {})),
        ].into_iter().collect();
        if let Some(result_rune) = &x.result_rune {
            conclusions.insert(result_rune.rune.clone(), ITemplataType::ImplTemplataType(ImplTemplataType {}));
        }
        solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], conclusions, vec![], HashSet::new())
    }
    IRulexSR::OneOf(x) => {
      let types: HashSet<ITemplataType<'s>> = x.literals.iter().map(|l| l.get_type()).collect();
      if types.len() > 1 {
        panic!("OneOf rule's possibilities must all be the same type!");
      }
      let the_type = types.into_iter().next().unwrap();
      solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], [(x.rune.rune.clone(), the_type)].into_iter().collect(), vec![], HashSet::new())
    }
    IRulexSR::Equals(x) => {
      let left_conclusion = solver_state.get_conclusion(&x.left.rune);
      match left_conclusion {
        None => {
          let right_conclusion = solver_state.get_conclusion(&x.right.rune).expect("Neither side of EqualsSR has a conclusion");
          solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], [(x.left.rune.clone(), right_conclusion)].into_iter().collect(), vec![], HashSet::new())
        }
        Some(left) => {
          solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], [(x.right.rune.clone(), left)].into_iter().collect(), vec![], HashSet::new())
        }
      }
    }
    IRulexSR::IsConcrete(_) => panic!("IRulexSR::IsConcrete not yet migrated in rune_type solve_rule"),
    IRulexSR::IsInterface(x) => {
      solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], [(x.rune.rune.clone(), ITemplataType::KindTemplataType(KindTemplataType {}))].into_iter().collect(), vec![], HashSet::new())
    }
    IRulexSR::IsStruct(_) => panic!("IRulexSR::IsStruct not yet migrated in rune_type solve_rule"),
    IRulexSR::RefListCompoundMutability(_) => panic!("IRulexSR::RefListCompoundMutability not yet migrated in rune_type solve_rule"),
    IRulexSR::CoerceToCoord(x) => {
      solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], [
        (x.coord_rune.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {})),
        (x.kind_rune.rune.clone(), ITemplataType::KindTemplataType(KindTemplataType {})),
      ].into_iter().collect(), vec![], HashSet::new())
    }
    IRulexSR::Literal(x) => {
      solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], [(x.rune.rune.clone(), x.literal.get_type())].into_iter().collect(), vec![], HashSet::new())
    }
    IRulexSR::Lookup(x) => {
      let actual_lookup_result =
          match env.lookup(x.range.clone(), x.name.clone()) {
            Err(_e) => panic!("LookupSR solve error path not yet migrated"),
            Ok(r) => r,
          };
      let tyype = match actual_lookup_result {
        IRuneTypeSolverLookupResult::Primitive(p) => p.tyype,
        IRuneTypeSolverLookupResult::Templata(t) => t.templata,
        IRuneTypeSolverLookupResult::Citizen(c) => c.tyype,
      };
      solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], [(x.rune.rune.clone(), tyype)].into_iter().collect(), vec![], HashSet::new())
    }
    IRulexSR::MaybeCoercingLookup(x) => {
      let actual_lookup_result =
          match env.lookup(x.range.clone(), x.name.clone()) {
            Err(_e) => panic!("MaybeCoercingLookupSR solve error path not yet migrated"),
            Ok(r) => r,
          };
      // AFTERM: lookup_rune_type only validates, doesn't conclude runes. Need to add commitStep here.
      lookup_rune_type(env, solver_state, x.range.clone(), &x.rune, actual_lookup_result)?;
      solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], HashMap::new(), vec![], HashSet::new())
    }
    IRulexSR::RuneParentEnvLookup(x) => {
      let lookup_name = scout_arena.intern_imprecise_name(IImpreciseNameValS::RuneName(RuneNameValS { rune: x.rune.rune.clone() }));
      let actual_lookup_result =
          match env.lookup(x.range.clone(), lookup_name) {
            Err(_e) => panic!("RuneParentEnvLookupSR solve error path not yet migrated"),
            Ok(r) => r,
          };
      lookup_rune_type(env, solver_state, x.range.clone(), &x.rune, actual_lookup_result)?;
      solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], HashMap::new(), vec![], HashSet::new())
    }
    IRulexSR::Augment(x) => {
      solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], [
        (x.result_rune.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {})),
        (x.inner_rune.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {})),
      ].into_iter().collect(), vec![], HashSet::new())
    }
    IRulexSR::Pack(x) => {
      let mut conclusions: HashMap<IRuneS<'s>, ITemplataType<'s>> = x.members.iter()
        .map(|m| (m.rune.clone(), ITemplataType::CoordTemplataType(CoordTemplataType {})))
        .collect();
      conclusions.insert(x.result_rune.rune.clone(), ITemplataType::PackTemplataType(PackTemplataType { element_type: scout_arena.alloc(ITemplataType::CoordTemplataType(CoordTemplataType {})) }));
      solver_state.commit_step::<IRuneTypeRuleError<'s>>(false, vec![rule_index], conclusions, vec![], HashSet::new())
    }
    IRulexSR::CoordSend(_) => panic!("IRulexSR::CoordSend not yet migrated in rune_type solve_rule"),
    IRulexSR::Call(_) => panic!("IRulexSR::Call not yet migrated in rune_type solve_rule"),
    IRulexSR::IndexList(_) => panic!("IRulexSR::IndexList not yet migrated in rune_type solve_rule"),
  }
}
/*
  private def solveRule(
    env: IRuneTypeSolverEnv,
    solverState: SimpleSolverState[IRulexSR, IRuneS, ITemplataType],
    ruleIndex: Int,
    rule: IRulexSR):
  Result[Unit, ISolverError[IRuneS, ITemplataType, IRuneTypeRuleError]] = {
    rule match {
      case KindComponentsSR(range, resultRune, mutabilityRune) => {
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> KindTemplataType(), mutabilityRune.rune -> MutabilityTemplataType()), Vector(), Set.empty)
      }
      case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => {
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> CoordTemplataType(), ownershipRune.rune -> OwnershipTemplataType(), kindRune.rune -> KindTemplataType()), Vector(), Set.empty)
      }
      case PrototypeComponentsSR(range, resultRune, paramsRune, returnRune) => {
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> PrototypeTemplataType(), paramsRune.rune -> PackTemplataType(CoordTemplataType()), returnRune.rune -> CoordTemplataType()), Vector(), Set.empty)
      }
      case MaybeCoercingCallSR(range, resultRune, templateRune, argRunes) => {
        vassertSome(solverState.getConclusion(templateRune.rune)) match {
          case TemplateTemplataType(paramTypes, returnType) => {
            val conclusions = argRunes.map(_.rune).zip(paramTypes).map({ case (argRune, paramType) => (argRune -> paramType) }).toMap
            solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), conclusions, Vector(), Set.empty)
          }
          case other => vwat(other)
        }
      }
      case ResolveSR(range, resultRune, name, paramListRune, returnRune) => {
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> PrototypeTemplataType(), paramListRune.rune -> PackTemplataType(CoordTemplataType()), returnRune.rune -> CoordTemplataType()), Vector(), Set.empty)
      }
      case CallSiteFuncSR(range, resultRune, name, paramListRune, returnRune) => {
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> PrototypeTemplataType(), paramListRune.rune -> PackTemplataType(CoordTemplataType()), returnRune.rune -> CoordTemplataType()), Vector(), Set.empty)
      }
      case DefinitionFuncSR(range, resultRune, name, paramListRune, returnRune) => {
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> PrototypeTemplataType(), paramListRune.rune -> PackTemplataType(CoordTemplataType()), returnRune.rune -> CoordTemplataType()), Vector(), Set.empty)
      }
      case DefinitionCoordIsaSR(range, resultRune, subRune, superRune) => {
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> ImplTemplataType(), subRune.rune -> CoordTemplataType(), superRune.rune -> CoordTemplataType()), Vector(), Set.empty)
      }
      case CallSiteCoordIsaSR(range, resultRune, subRune, superRune) => {
        val conclusions = Map(subRune.rune -> CoordTemplataType(), superRune.rune -> CoordTemplataType()) ++
            (resultRune match {
              case Some(resultRune) => Map(resultRune.rune -> ImplTemplataType())
              case None => Map[IRuneS, ITemplataType]()
            })
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), conclusions, Vector(), Set.empty)
      }
      case OneOfSR(range, resultRune, literals) => {
        val types = literals.map(_.getType()).toSet
        if (types.size > 1) {
          vfail("OneOf rule's possibilities must all be the same type!")
        }
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> types.head), Vector(), Set.empty)
      }
      case EqualsSR(range, leftRune, rightRune) => {
        solverState.getConclusion(leftRune.rune) match {
          case None => {
            solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(leftRune.rune -> vassertSome(solverState.getConclusion(rightRune.rune))), Vector(), Set.empty)
          }
          case Some(left) => {
            solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(rightRune.rune -> left), Vector(), Set.empty)
          }
        }
      }
      case IsConcreteSR(range, rune) => {
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(rune.rune -> KindTemplataType()), Vector(), Set.empty)
      }
      case IsInterfaceSR(range, rune) => {
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(rune.rune -> KindTemplataType()), Vector(), Set.empty)
      }
      case IsStructSR(range, rune) => {
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(rune.rune -> KindTemplataType()), Vector(), Set.empty)
      }
      case RefListCompoundMutabilitySR(range, resultRune, coordListRune) => {
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> MutabilityTemplataType(), coordListRune.rune -> PackTemplataType(CoordTemplataType())), Vector(), Set.empty)
      }
      case CoerceToCoordSR(range, coordRune, kindRune) => {
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(coordRune.rune -> CoordTemplataType(), kindRune.rune -> KindTemplataType()), Vector(), Set.empty)
      }
      case LiteralSR(range, rune, literal) => {
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(rune.rune -> literal.getType()), Vector(), Set.empty)
      }
      case LookupSR(range, resultRune, name) => {
        val actualLookupResult =
          env.lookup(range, name) match {
            case Err(e) => return Err(RuleError(e))
            case Ok(x) => x
          }
        val tyype = actualLookupResult match {
          case PrimitiveRuneTypeSolverLookupResult(tyype) => tyype
          case TemplataLookupResult(actualType) => actualType
          case CitizenRuneTypeSolverLookupResult(tyype, genericParams) => tyype
        }
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> tyype), Vector(), Set.empty)
      }
      case MaybeCoercingLookupSR(range, rune, name) => {
        val actualLookupResult =
          env.lookup(range, name) match {
            case Err(e) => return Err(RuleError(e))
            case Ok(x) => x
          }
        lookup(env, solverState, range, rune, actualLookupResult) match {
          case Ok(()) => solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(), Vector(), Set.empty)
          case Err(e) => Err(e)
        }
      }
      case RuneParentEnvLookupSR(range, rune) => {
        val actualLookupResult =
          env.lookup(range, interner.intern(RuneNameS(rune.rune))) match {
            case Err(e) => return Err(RuleError(e))
            case Ok(x) => x
          }
        lookup(env, solverState, range, rune, actualLookupResult) match {
          case Ok(()) => solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(), Vector(), Set.empty)
          case Err(e) => Err(e)
        }
      }
      case AugmentSR(range, resultRune, ownership, innerRune) => {
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> CoordTemplataType(), innerRune.rune -> CoordTemplataType()), Vector(), Set.empty)
      }
      case PackSR(range, resultRune, memberRunes) => {
        val conclusions = memberRunes.map(x => (x.rune -> CoordTemplataType())).toMap + (resultRune.rune -> PackTemplataType(CoordTemplataType()))
        solverState.commitStep[IRuneTypeRuleError](false, Vector(ruleIndex), conclusions, Vector(), Set.empty)
      }
//      case StaticSizedArraySR(range, resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune) => {
//        solverState.concludeRune[IRuneTypeRuleError](mutabilityRune.rune MutabilityTemplataType()) match { case Ok(_) => case Err(e) => return Err(e) }
//        solverState.concludeRune[IRuneTypeRuleError](variabilityRune.rune VariabilityTemplataType()) match { case Ok(_) => case Err(e) => return Err(e) }
//        solverState.concludeRune[IRuneTypeRuleError](sizeRune.rune IntegerTemplataType()) match { case Ok(_) => case Err(e) => return Err(e) }
//        solverState.concludeRune[IRuneTypeRuleError](elementRune.rune CoordTemplataType()) match { case Ok(_) => case Err(e) => return Err(e) }
//        Ok(())
//      }
//      case RuntimeSizedArraySR(range, resultRune, mutabilityRune, elementRune) => {
//        solverState.concludeRune[IRuneTypeRuleError](mutabilityRune.rune MutabilityTemplataType()) match { case Ok(_) => case Err(e) => return Err(e) }
//        solverState.concludeRune[IRuneTypeRuleError](elementRune.rune CoordTemplataType()) match { case Ok(_) => case Err(e) => return Err(e) }
//        Ok(())
//      }
    }
  }
*/
// mig: fn lookup

fn lookup_rune_type<'s, E: IRuneTypeSolverEnv<'s>>(
  _env: &E,
  solver_state: &mut SimpleSolverState<
    IRulexSR<'s>,
    IRuneS<'s>,
    ITemplataType<'s>,
  >,
  _range: RangeS<'s>,
  rune: &RuneUsage<'s>,
  actual_lookup_result: IRuneTypeSolverLookupResult<'s>,
) -> Result<(), ISolverError<
  IRuneS<'s>,
  ITemplataType<'s>,
  IRuneTypeRuleError<'s>,
>> {
  let expected_type = solver_state.get_conclusion(&rune.rune).expect("lookup_rune_type: no conclusion for rune");
  match actual_lookup_result {
    IRuneTypeSolverLookupResult::Primitive(p) => {
      match &expected_type {
        ITemplataType::CoordTemplataType(_) | ITemplataType::KindTemplataType(_) => {}
        x if *x == p.tyype => {}
        _ => panic!("lookup_rune_type Primitive error path not yet migrated"),
      }
    }
    IRuneTypeSolverLookupResult::Templata(t) => {
      let actual_type = t.templata;
      match (&actual_type, &expected_type) {
        (x, y) if x == y => {} // Matches, so is fine
        (ITemplataType::KindTemplataType(_), ITemplataType::CoordTemplataType(_)) => {} // Will convert, so is fine
        (ITemplataType::TemplateTemplataType(tt), ITemplataType::CoordTemplataType(_) | ITemplataType::KindTemplataType(_))
            if tt.param_types.is_empty()
                && matches!(tt.return_type, ITemplataType::KindTemplataType(_) | ITemplataType::CoordTemplataType(_)) => {
          // Then it's an implicit call.
          match check_generic_call(vec![_range.clone()], &[], &[]) {
            Ok(()) => {},
            Err(e) => return Err(ISolverError::RuleError(RuleError { err: e, _phantom: PhantomData })),
          }
        }
        _ => panic!("lookup_rune_type Templata FoundTemplataDidntMatchExpectedType not yet migrated"),
      }
    }
    IRuneTypeSolverLookupResult::Citizen(c) => {
      match &expected_type {
        ITemplataType::CoordTemplataType(_) | ITemplataType::KindTemplataType(_) => {
          // Then it's an implicit call, straight from being looked up.
          match check_generic_call(vec![_range.clone()], &c.generic_params, &[]) {
            Ok(()) => {},
            Err(e) => return Err(ISolverError::RuleError(RuleError { err: e, _phantom: PhantomData })),
          }
        }
        x if *x == c.tyype => {}
        _ => panic!("lookup_rune_type Citizen error path not yet migrated"),
      }
    }
  }
  Ok(())
}
/*
  private def lookup(
      env: IRuneTypeSolverEnv,
      solverState: SimpleSolverState[IRulexSR, IRuneS, ITemplataType],
      range: RangeS,
      rune: RuneUsage,
      actualLookupResult: IRuneTypeSolverLookupResult):
  Result[Unit, ISolverError[IRuneS, ITemplataType, IRuneTypeRuleError]] = {
    val expectedType = vassertSome(solverState.getConclusion(rune.rune))
    actualLookupResult match {
      case PrimitiveRuneTypeSolverLookupResult(tyype) => {
        expectedType match {
          case CoordTemplataType() | KindTemplataType() => // Either is fine
          // This could happen for e.g. Array and StaticArray, which are both primitive templates.
          case x if x == tyype => {
            // Not an implicit call, and it matches, proceed.
          }
          case _ => return Err(RuleError(FoundPrimitiveDidntMatchExpectedType(List(range), expectedType, tyype)))
        }
      }
      case TemplataLookupResult(actualType) => {
        (actualType, expectedType) match {
          case (x, y) if x == y => // Matches, so is fine
          case (KindTemplataType(), CoordTemplataType()) => // Will convert, so is fine
          case (TemplateTemplataType(Vector(), KindTemplataType() | CoordTemplataType()), CoordTemplataType() | KindTemplataType()) => {
            // Then it's an implicit call.
            checkGenericCallWithoutDefaults(List(range), Vector(), Vector()) match {
              case Ok(()) =>
              case Err(e) => return Err(RuleError(e))
            }
          }
          case _ => return Err(RuleError(FoundTemplataDidntMatchExpectedType(List(range), expectedType, actualType)))
        }
      }
      case CitizenRuneTypeSolverLookupResult(tyype, genericParams) => {
        expectedType match {
          case CoordTemplataType() | KindTemplataType() => {
            // Then it's an implicit call, straight from being looked up.
            checkGenericCall(List(range), genericParams, Vector()) match {
              case Ok(()) =>
              case Err(e) => return Err(RuleError(e))
            }
          }
          case x if x == tyype => {
            // Not an implicit call, and it matches, proceed.
          }
          case _ => return Err(RuleError(FoundCitizenDidntMatchExpectedType(List(range), expectedType, tyype)))
        }
      }
    }
    Ok(())
  }
*/
// mig: fn solve_rune_type
pub fn solve_rune_type<'s, E: IRuneTypeSolverEnv<'s>>(
  // V: we took out self here, do we have a coherent story about when something should be self/impl'd
  // VA: In Scala, solveRuneType was a method on class RuneTypeSolver(interner). In Rust, it was
  // VA: extracted to a free function while RuneTypeSolver exists as a thin delegating wrapper.
  // VA: The dominant pattern across postparsing solvers is free functions: identifiability_solver.rs
  // VA: and rule_scout.rs are entirely free functions with no struct. The RuneTypeSolver struct is
  // VA: the exception — it could be removed to match the peer files, or the free functions could be
  // VA: moved into it to match Scala's class structure. Currently inconsistent.
  scout_arena: &ScoutArena<'s>,
  sanity_check: bool,
  env: &E,
  range: Vec<RangeS<'s>>,
  predicting: bool,
  rules_s: &[IRulexSR<'s>],
  additional_runes: &[IRuneS<'s>],
  expect_complete_solve: bool,
  unpreprocessed_initially_known_runes: HashMap<IRuneS<'s>, ITemplataType<'s>>,
) -> Result<
  HashMap<IRuneS<'s>, ITemplataType<'s>>,
  RuneTypeSolveError<'s>,
> {




  // For the non-predicting case, iterate over LookupSR/MaybeCoercingLookupSR rules and pre-compute types via env.lookup.
  // For now, with no rules in the simple test case, this is empty.
  let mut initially_known_runes: HashMap<IRuneS<'s>, ITemplataType<'s>> = if predicting {
    HashMap::new()
  } else {
    let mut map = HashMap::new();
    for rule in rules_s {
      match rule {
        IRulexSR::Lookup(lookup) => {
          match env.lookup(lookup.range.clone(), lookup.name.clone()) {
            Err(_e) => {
              panic!("LookupSR pre-computation error path not yet migrated");
            }
            Ok(result) => {
              let entries: Vec<(IRuneS<'s>, ITemplataType)> = match &result {
                // We don't know whether we'll coerce this into a kind or a coord.
                IRuneTypeSolverLookupResult::Primitive(p) => {
                  match &p.tyype {
                    ITemplataType::KindTemplataType(_) => vec![],
                    ITemplataType::TemplateTemplataType(t) if t.param_types.is_empty() => vec![],
                    other => vec![(lookup.rune.rune.clone(), other.clone())],
                  }
                }
                IRuneTypeSolverLookupResult::Citizen(c) => {
                  match &c.tyype {
                    ITemplataType::TemplateTemplataType(t) if t.param_types.is_empty() && matches!(&*t.return_type, ITemplataType::KindTemplataType(_)) => vec![],
                    other => vec![(lookup.rune.rune.clone(), other.clone())],
                  }
                }
                IRuneTypeSolverLookupResult::Templata(t) => {
                  match &t.templata {
                    ITemplataType::TemplateTemplataType(tt) if tt.param_types.is_empty() && matches!(&*tt.return_type, ITemplataType::KindTemplataType(_)) => vec![],
                    ITemplataType::KindTemplataType(_) => vec![],
                    other => vec![(lookup.rune.rune.clone(), other.clone())],
                  }
                }
              };
              for (k, v) in entries {
                map.insert(k, v);
              }
            }
          }
        }
        IRulexSR::MaybeCoercingLookup(lookup) => {
          match env.lookup(lookup.range.clone(), lookup.name.clone()) {
            Err(e) => {
              return Err(RuneTypeSolveError {
                range: vec![lookup.range.clone()],
                failed_solve: FailedSolve {
                    steps: vec![],
                    conclusions: HashMap::new(),
                    unsolved_rules: rules_s.to_vec(),
                    unsolved_runes: vec![],
                    error: ISolverError::RuleError(
                      RuleError {
                        err: e.into(),
                        _phantom: PhantomData,
                      }
                    ),
                },
              });
            }
            Ok(result) => {
              let entries: Vec<(IRuneS<'s>, ITemplataType)> = match &result {
                // We don't know whether we'll coerce this into a kind or a coord.
                IRuneTypeSolverLookupResult::Primitive(p) => {
                  match &p.tyype {
                    ITemplataType::KindTemplataType(_) => vec![],
                    ITemplataType::TemplateTemplataType(t) if t.param_types.is_empty() => vec![],
                    other => vec![(lookup.rune.rune.clone(), other.clone())],
                  }
                }
                IRuneTypeSolverLookupResult::Citizen(c) => {
                  match &c.tyype {
                    ITemplataType::TemplateTemplataType(t) if t.param_types.is_empty() && matches!(&*t.return_type, ITemplataType::KindTemplataType(_)) => vec![],
                    other => vec![(lookup.rune.rune.clone(), other.clone())],
                  }
                }
                IRuneTypeSolverLookupResult::Templata(t) => {
                  match &t.templata {
                    ITemplataType::TemplateTemplataType(tt) if tt.param_types.is_empty() && matches!(&*tt.return_type, ITemplataType::KindTemplataType(_)) => vec![],
                    ITemplataType::KindTemplataType(_) => vec![],
                    other => vec![(lookup.rune.rune.clone(), other.clone())],
                  }
                }
              };
              for (k, v) in entries {
                map.insert(k, v);
              }
            }
          }
        }
        _ => {
          // Other rules don't contribute to initially known runes
        }
      }
    }
    map
  };
  // unpreprocessedInitiallyKnownRunes comes after (takes priority, see Scala comment)
  for (k, v) in unpreprocessed_initially_known_runes {
    initially_known_runes.insert(k, v);
  }

  // Compute all_runes for solver = rules.flatMap(getRunes) ++ initiallyKnownRunes.keys, deduplicated
  // (additionalRunes are NOT included here — they're added after solving for the completeness check)
  let mut all_runes_set = HashSet::new();
  for rule in rules_s {
    for rune_usage in rule.rune_usages() {
      all_runes_set.insert(rune_usage.rune.clone());
    }
  }
  for k in initially_known_runes.keys() {
    all_runes_set.insert(k.clone());
  }
  let solver_runes: Vec<IRuneS<'s>> = all_runes_set.into_iter().collect();

  let predicting_copy = predicting;
  let mut solver_state = make_solver_state(
    sanity_check,
    false,
    Box::new(move |rule: &IRulexSR<'s>| get_puzzles_rune_type(predicting_copy, rule)),
    &get_runes_rune_type,
    rules_s.to_vec(),
    initially_known_runes,
    solver_runes,
  );

  // Inline advance loop (matches Scala's while loop in solve())
  loop {
    solver_state.sanity_check();
    solver_state.userify_conclusions().iter().for_each(|(rune, conclusion)| {
      sanity_check_conclusion(rune.clone(), conclusion);
    });
    // Stage 1: simple solve
    match solver_state.get_next_solvable() {
      None => break, // No more solvable rules
      Some(rule_index) => {
        let rule = solver_state.get_rule(rule_index).clone();
        let steps_before = solver_state.get_steps().len();
        match solve_rule(scout_arena, env, rule_index, &rule, &mut solver_state) {
          Ok(()) => {}
          Err(e) => {
            return Err(RuneTypeSolveError {
              range,
              failed_solve: FailedSolve {
                steps: solver_state.get_steps(),
                conclusions: solver_state.get_conclusions().into_iter().collect(),
                unsolved_rules: solver_state.get_unsolved_rules(),
                unsolved_runes: solver_state.get_unsolved_runes(),
                error: e,
              },
            });
          }
        }
        let steps_after = solver_state.get_steps().len();
        assert!(steps_after == steps_before + 1);
        assert!(solver_state.rule_is_solved(rule_index));
        solver_state.sanity_check();
      }
    }
  }

  let conclusions: HashMap<IRuneS<'s>, ITemplataType<'s>> = solver_state.userify_conclusions().into_iter().collect();
  let unsolved_runes = solver_state.get_unsolved_runes();

  if expect_complete_solve && !unsolved_runes.is_empty() {
    let steps = solver_state.get_steps();
    let unsolved_rules = solver_state.get_unsolved_rules();
    Err(RuneTypeSolveError {
      range,
      failed_solve: FailedSolve {
          steps,
          conclusions: conclusions.clone(),
          unsolved_rules,
          unsolved_runes,
          error: ISolverError::SolveIncomplete(
            SolveIncomplete {
              _phantom: PhantomData,
            }
          ),
      },
    })
  } else {
    Ok(conclusions)
  }
}
/*
  def solve(
    sanityCheck: Boolean,
    useOptimizedSolver: Boolean,
    env: IRuneTypeSolverEnv,
    range: List[RangeS],
    predicting: Boolean,
    rules: IndexedSeq[IRulexSR],
    // Some runes don't appear in the rules, for example if they are in the identifying runes,
    // but not in any of the members or rules.
    additionalRunes: Iterable[IRuneS],
    expectCompleteSolve: Boolean,
    unpreprocessedInitiallyKnownRunes: Map[IRuneS, ITemplataType]):
  Result[Map[IRuneS, ITemplataType], RuneTypeSolveError] = {
    val initialRunes = (rules.flatMap(_.runeUsages).map(_.rune) ++ additionalRunes).toVector
    val initiallyKnownRunes =
        (if (predicting) {
          Map()
        } else {
          // Calculate what types we can beforehand, see KVCIE.
          rules.flatMap({
            case LookupSR(range, rune, name) => {
              env.lookup(range, name) match {
                case Err(e) => {
                  return Err(
                    RuneTypeSolveError(
                      List(range),
                      FailedSolve(Vector().toStream, Map(), rules.toVector, initialRunes, RuleError(e))))
                }
                // We don't know whether we'll coerce this into a kind or a coord.
                case Ok(PrimitiveRuneTypeSolverLookupResult(KindTemplataType())) => List()
                case Ok(PrimitiveRuneTypeSolverLookupResult(t@TemplateTemplataType(Vector(), _))) => List()
                // We'll load this as is. If its a call with params, leave it to the call site to figure out how to coerce the return.
                case Ok(PrimitiveRuneTypeSolverLookupResult(tyype)) => List(rune.rune -> tyype)

                // We don't know whether we'll coerce this into a kind or a coord.
                case Ok(CitizenRuneTypeSolverLookupResult(TemplateTemplataType(Vector(), KindTemplataType()), _)) => List()
                // We can't automatically coerce this, so we can use it as is.
                case Ok(CitizenRuneTypeSolverLookupResult(tyype, _)) => List(rune.rune -> tyype)

                // We don't know whether we'll coerce this into a kind or a coord.
                case Ok(TemplataLookupResult(TemplateTemplataType(Vector(), KindTemplataType()))) => List()
                case Ok(TemplataLookupResult(KindTemplataType())) => List()
                // If it's not a kind, then we'll use it as it is.
                case Ok(TemplataLookupResult(tyype)) => List(rune.rune -> tyype)
                case _ => vwat()
              }
            }
            case MaybeCoercingLookupSR(range, rune, name) => {
              env.lookup(range, name) match {
                case Err(e) => {
                  return Err(
                    RuneTypeSolveError(
                      List(range),
                      FailedSolve(Vector().toStream, Map(), rules.toVector, initialRunes, RuleError(e))))
                }
                // We don't know whether we'll coerce this into a kind or a coord.
                case Ok(PrimitiveRuneTypeSolverLookupResult(KindTemplataType())) => List()
                case Ok(PrimitiveRuneTypeSolverLookupResult(t@TemplateTemplataType(Vector(), _))) => List()
                // We'll load this as is. If its a call with params, leave it to the call site to figure out how to coerce the return.
                case Ok(PrimitiveRuneTypeSolverLookupResult(tyype)) => List(rune.rune -> tyype)

                // We don't know whether we'll coerce this into a kind or a coord.
                case Ok(CitizenRuneTypeSolverLookupResult(TemplateTemplataType(Vector(), KindTemplataType()), _)) => List()
                // We can't automatically coerce this, so we can use it as is.
                case Ok(CitizenRuneTypeSolverLookupResult(tyype, _)) => List(rune.rune -> tyype)

                // We don't know whether we'll coerce this into a kind or a coord.
                case Ok(TemplataLookupResult(TemplateTemplataType(Vector(), KindTemplataType()))) => List()
                case Ok(TemplataLookupResult(KindTemplataType())) => List()
                // If it's not a kind, then we'll use it as it is.
                case Ok(TemplataLookupResult(tyype)) => List(rune.rune -> tyype)
                case _ => vwat()
              }
            }
            case _ => List()
          }).toMap
        }) ++
      // This comes after, because we trust the initially known conclusions more. For example,
      // an initially known conclusion might know that a pattern's incoming rune should be a coord,
      // while the above code might think it's a template.
      unpreprocessedInitiallyKnownRunes
    val solverState =
      Solver.makeSolverState(
        sanityCheck,
        useOptimizedSolver,
        (rule: IRulexSR) => getPuzzles(predicting, rule),
        getRunes,
        rules,
        initiallyKnownRunes,
        (rules.flatMap(getRunes) ++ initiallyKnownRunes.keys).distinct.toVector)
*/

// mig: fn sanity_check_conclusion
fn sanity_check_conclusion<'s>(
  _rune: IRuneS<'s>,
  _conclusion: &ITemplataType<'s>,
) {
}
/*
    while ({
      solverState.sanityCheck()
      solverState.getNextSolvable() match {
        case None => false // break
        case Some(solvingRuleIndex) => {
          val rule = solverState.getRule(solvingRuleIndex)
          val stepsBefore = solverState.getSteps().size
          solveRule(env, solverState, solvingRuleIndex, rule) match {
            case Err(e) => {
              return Err(RuneTypeSolveError(range, FailedSolve(solverState.getSteps(), solverState.getConclusions().toMap, solverState.getUnsolvedRules(), solverState.getUnsolvedRunes(), e)))
            }
            case Ok(()) =>
          }
          val stepsAfter = solverState.getSteps().size
          vassert(stepsAfter == stepsBefore + 1)
          vassert(solverState.ruleIsSolved(solvingRuleIndex)) // Per @CSCDSRZ, only true after simple solve.
          solverState.sanityCheck()
          true // continue
        }
      }
    }) {}
*/
// mig: fn complex_solve
fn complex_solve() -> Result<(), ()> {
  panic!("Unimplemented complex_solve");
}
/*
    val steps = solverState.getSteps().toStream
    val conclusions = solverState.userifyConclusions().toMap

    val allRunes = solverState.getAllRunes() ++ additionalRunes
    val unsolvedRunes = allRunes -- conclusions.keySet
*/
// mig: fn solve
fn solve<'s>(
  _state: (),
  _env: (),
  _solver_state: (),
  _rule_index: usize,
  _rule: &IRulexSR<'s>,
  _step_state: (),
) -> Result<(), ()> {
  panic!("Unimplemented solve");
}
/*
    if (expectCompleteSolve && unsolvedRunes.nonEmpty) {
      Err(
        RuneTypeSolveError(
          range,
          FailedSolve(
            steps,
            solverState.getConclusions().toMap,
            solverState.getUnsolvedRules(),
            unsolvedRunes.toVector,
            SolveIncomplete())))
    } else {
      Ok(conclusions)
    }
  }
}
*/
/*
object RuneTypeSolver {
*/
// mig: fn check_generic_call_without_defaults
fn check_generic_call_without_defaults<'s>(
  _param_types: &[ITemplataType<'s>],
  _arg_types: &[ITemplataType<'s>],
) -> Result<(), ()> {
  panic!("Unimplemented check_generic_call_without_defaults");
}
/*
  def checkGenericCallWithoutDefaults(
      range: List[RangeS],
      paramTypes: Vector[ITemplataType],
      argTypes: Vector[ITemplataType]):
  Result[Unit, IRuneTypeRuleError] = {
    paramTypes.zipWithIndex.foreach({ case (paramType, index) =>
      if (index < argTypes.length) {
        val actualType = argTypes(index)
        // Make sure the given type matches the expected one
        if (paramType == actualType) {
          // Matches, proceed.
        } else {
          return Err(GenericCallArgTypeMismatch(range, paramType, actualType, index))
        }
      } else {
        return Err(NotEnoughArgumentsForGenericCall(range, index))
      }
    })

    Ok(())
  }
*/
// mig: fn check_generic_call
fn check_generic_call<'s>(
  range: Vec<RangeS<'s>>,
  citizen_generic_params: &[&GenericParameterS<'s>],
  arg_types: &[ITemplataType<'s>],
) -> Result<(), IRuneTypeRuleError<'s>> {
  for (index, generic_param) in citizen_generic_params.iter().enumerate() {
    if index < arg_types.len() {
      let actual_type = &arg_types[index];
      if generic_param.tyype.tyype() == *actual_type {
        // Matches, proceed.
      } else {
        return Err(IRuneTypeRuleError::GenericCallArgTypeMismatch(GenericCallArgTypeMismatch {
          range: range.clone(),
          expected_type: generic_param.tyype.tyype(),
          actual_type: actual_type.clone(),
          param_index: index as i32,
        }));
      }
    } else {
      if generic_param.default.is_some() {
        // Good, can just use that default
      } else {
        return Err(IRuneTypeRuleError::NotEnoughArgumentsForGenericCall(NotEnoughArgumentsForGenericCall {
          range: range.clone(),
          index_of_non_defaulting_param: index as i32,
        }));
      }
    }
  }

  Ok(())
}
/*
  def checkGenericCall(
    range: List[RangeS],
    citizenGenericParams: Vector[GenericParameterS],
    argTypes: Vector[ITemplataType]):
  Result[Unit, IRuneTypeRuleError] = {
    citizenGenericParams.zipWithIndex.foreach({ case (genericParam, index) =>
      if (index < argTypes.length) {
        val actualType = argTypes(index)
        // Make sure the given type matches the expected one
        if (genericParam.tyype.tyype == actualType) {
          // Matches, proceed.
        } else {
          return Err(GenericCallArgTypeMismatch(range, genericParam.tyype.tyype, actualType, index))
        }
      } else {
        if (genericParam.default.nonEmpty) {
          // Good, can just use that default
        } else {
          return Err(NotEnoughArgumentsForGenericCall(range, index))
        }
      }
    })

    Ok(())
  }
}
*/