/*
// AFTERM: consider moving to higher_typing

package dev.vale.postparsing

import dev.vale.{Err, Interner, Ok, RangeS, Result, vassert, vassertSome, vfail, vpass, vwat}
import dev.vale.postparsing.rules._
import dev.vale.solver.{FailedSolve, IIncompleteOrFailedSolve, ISolveRule, ISolverError, ISolverState, IStepState, IncompleteSolve, RuleError, Solver, SolverConflict}
import dev.vale._
import dev.vale.postparsing.RuneTypeSolver._
import dev.vale.postparsing.rules._

import scala.collection.immutable.Map
*/
// mig: struct RuneTypeSolveError
pub struct RuneTypeSolveError<'a> {
  pub range: Vec<crate::utils::range::RangeS<'a>>,
  pub failed_solve: (),
}
// mig: impl RuneTypeSolveError
impl<'a> RuneTypeSolveError<'a> {
}
/*
case class RuneTypeSolveError(range: List[RangeS], failedSolve: IIncompleteOrFailedSolve[IRulexSR, IRuneS, ITemplataType, IRuneTypeRuleError]) {
  vpass()
}
*/
// mig: enum IRuneTypeRuleError
pub enum IRuneTypeRuleError<'a> {
  _Phantom(std::marker::PhantomData<&'a ()>),
}
/*
sealed trait IRuneTypeRuleError
*/
// mig: struct FoundCitizenDidntMatchExpectedType
pub struct FoundCitizenDidntMatchExpectedType<'a> {
  pub range: Vec<crate::utils::range::RangeS<'a>>,
  pub expected_type: crate::postparsing::itemplatatype::ITemplataType,
  pub actual_type: crate::postparsing::itemplatatype::ITemplataType,
}
// mig: impl FoundCitizenDidntMatchExpectedType
impl<'a> FoundCitizenDidntMatchExpectedType<'a> {
}
/*
case class FoundCitizenDidntMatchExpectedType(
  range: List[RangeS],
  expectedType: ITemplataType,
  actualType: ITemplataType
) extends IRuneTypeRuleError
*/
// mig: struct FoundTemplataDidntMatchExpectedType
pub struct FoundTemplataDidntMatchExpectedType<'a> {
  pub range: Vec<crate::utils::range::RangeS<'a>>,
  pub expected_type: crate::postparsing::itemplatatype::ITemplataType,
  pub actual_type: crate::postparsing::itemplatatype::ITemplataType,
}
// mig: impl FoundTemplataDidntMatchExpectedType
impl<'a> FoundTemplataDidntMatchExpectedType<'a> {
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
// mig: struct NotEnoughArgumentsForGenericCall
pub struct NotEnoughArgumentsForGenericCall<'a> {
  pub range: Vec<crate::utils::range::RangeS<'a>>,
  pub index_of_non_defaulting_param: i32,
}
// mig: impl NotEnoughArgumentsForGenericCall
impl<'a> NotEnoughArgumentsForGenericCall<'a> {
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
// mig: struct GenericCallArgTypeMismatch
pub struct GenericCallArgTypeMismatch<'a> {
  pub range: Vec<crate::utils::range::RangeS<'a>>,
  pub expected_type: crate::postparsing::itemplatatype::ITemplataType,
  pub actual_type: crate::postparsing::itemplatatype::ITemplataType,
  pub param_index: i32,
}
// mig: impl GenericCallArgTypeMismatch
impl<'a> GenericCallArgTypeMismatch<'a> {
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
// mig: enum IRuneTypingLookupFailedError
pub enum IRuneTypingLookupFailedError<'a> {
  _Phantom(std::marker::PhantomData<&'a ()>),
}
/*
sealed trait IRuneTypingLookupFailedError extends IRuneTypeRuleError
*/
// mig: struct RuneTypingTooManyMatchingTypes
pub struct RuneTypingTooManyMatchingTypes<'a> {
  pub range: crate::utils::range::RangeS<'a>,
  pub name: crate::postparsing::names::IImpreciseNameS<'a>,
}
// mig: impl RuneTypingTooManyMatchingTypes
impl<'a> RuneTypingTooManyMatchingTypes<'a> {
/*
case class RuneTypingTooManyMatchingTypes(range: RangeS, name: IImpreciseNameS) extends IRuneTypingLookupFailedError {
*/
// mig: fn equals
fn equals(&self, obj: ()) -> bool {
  panic!("Unimplemented equals");
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
  panic!("Unimplemented hash_code");
}
} // end impl RuneTypingTooManyMatchingTypes
/*
  override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct RuneTypingCouldntFindType
pub struct RuneTypingCouldntFindType<'a> {
  pub range: crate::utils::range::RangeS<'a>,
  pub name: crate::postparsing::names::IImpreciseNameS<'a>,
}
// mig: impl RuneTypingCouldntFindType
impl<'a> RuneTypingCouldntFindType<'a> {
/*
case class RuneTypingCouldntFindType(range: RangeS, name: IImpreciseNameS) extends IRuneTypingLookupFailedError {
*/
// mig: fn equals
fn equals(&self, obj: ()) -> bool {
  panic!("Unimplemented equals");
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
  panic!("Unimplemented hash_code");
}
} // end impl RuneTypingCouldntFindType
/*
  override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct FoundTemplataDidntMatchExpectedTypeA
pub struct FoundTemplataDidntMatchExpectedTypeA<'a> {
  pub range: Vec<crate::utils::range::RangeS<'a>>,
  pub expected_type: crate::postparsing::itemplatatype::ITemplataType,
  pub actual_type: crate::postparsing::itemplatatype::ITemplataType,
}
// mig: impl FoundTemplataDidntMatchExpectedTypeA
impl<'a> FoundTemplataDidntMatchExpectedTypeA<'a> {
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
// mig: struct FoundPrimitiveDidntMatchExpectedType
pub struct FoundPrimitiveDidntMatchExpectedType<'a> {
  pub range: Vec<crate::utils::range::RangeS<'a>>,
  pub expected_type: crate::postparsing::itemplatatype::ITemplataType,
  pub actual_type: crate::postparsing::itemplatatype::ITemplataType,
}
// mig: impl FoundPrimitiveDidntMatchExpectedType
impl<'a> FoundPrimitiveDidntMatchExpectedType<'a> {
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
// mig: enum IRuneTypeSolverLookupResult
pub enum IRuneTypeSolverLookupResult<'a> {
  _Phantom(std::marker::PhantomData<&'a ()>),
}
/*
sealed trait IRuneTypeSolverLookupResult
*/
// mig: struct PrimitiveRuneTypeSolverLookupResult
pub struct PrimitiveRuneTypeSolverLookupResult {
  pub tyype: crate::postparsing::itemplatatype::ITemplataType,
}
// mig: impl PrimitiveRuneTypeSolverLookupResult
impl PrimitiveRuneTypeSolverLookupResult {
}
/*
case class PrimitiveRuneTypeSolverLookupResult(tyype: ITemplataType) extends IRuneTypeSolverLookupResult
*/
// mig: struct CitizenRuneTypeSolverLookupResult
pub struct CitizenRuneTypeSolverLookupResult<'a, 's> {
  pub tyype: crate::postparsing::itemplatatype::ITemplataType,
  pub generic_params: Vec<crate::postparsing::ast::GenericParameterS<'a, 's>>,
}
// mig: impl CitizenRuneTypeSolverLookupResult
impl<'a, 's> CitizenRuneTypeSolverLookupResult<'a, 's> {
}
/*
case class CitizenRuneTypeSolverLookupResult(tyype: TemplateTemplataType, genericParams: Vector[GenericParameterS]) extends IRuneTypeSolverLookupResult
*/
// mig: struct TemplataLookupResult
pub struct TemplataLookupResult {
  pub templata: crate::postparsing::itemplatatype::ITemplataType,
}
// mig: impl TemplataLookupResult
impl TemplataLookupResult {
}
/*
case class TemplataLookupResult(templata: ITemplataType) extends IRuneTypeSolverLookupResult
*/
// mig: trait IRuneTypeSolverEnv
pub trait IRuneTypeSolverEnv<'a> {
  fn lookup(
    &self,
    range: crate::utils::range::RangeS<'a>,
    name: crate::postparsing::names::IImpreciseNameS<'a>,
  ) -> Result<IRuneTypeSolverLookupResult<'a>, IRuneTypingLookupFailedError<'a>>;
}
/*
trait IRuneTypeSolverEnv {
  // MIGALLOW: lookup -> lookup_rune_type
  def lookup(range: RangeS, name: IImpreciseNameS):
  Result[IRuneTypeSolverLookupResult, IRuneTypingLookupFailedError]
}
*/
// mig: struct RuneTypeSolver
pub struct RuneTypeSolver<'a, 'ctx> {
  pub interner: &'ctx crate::interner::Interner<'a>,
}
// Concrete SolverDelegate for rune type solving.
// In Scala, this is an anonymous ISolveRule created inside RuneTypeSolver.solve().
struct RuneTypeSolverDelegate {
  predicting: bool,
}

impl<'a, E: IRuneTypeSolverEnv<'a>> crate::solver::solver::SolverDelegate<
  crate::postparsing::rules::rules::IRulexSR<'a>,
  crate::postparsing::names::IRuneS<'a>,
  E,
  (),
  crate::postparsing::itemplatatype::ITemplataType,
  IRuneTypeRuleError<'a>,
> for RuneTypeSolverDelegate {
  fn rule_to_puzzles(&self, rule: &crate::postparsing::rules::rules::IRulexSR<'a>) -> Vec<Vec<crate::postparsing::names::IRuneS<'a>>> {
    panic!("RuneTypeSolverDelegate::rule_to_puzzles not yet migrated")
  }

  fn rule_to_runes(&self, rule: &crate::postparsing::rules::rules::IRulexSR<'a>) -> Vec<crate::postparsing::names::IRuneS<'a>> {
    rule.rune_usages().iter().map(|ru| ru.rune.clone()).collect()
  }

  fn solve<S: crate::solver::ISolverState<
    crate::postparsing::rules::rules::IRulexSR<'a>,
    crate::postparsing::names::IRuneS<'a>,
    crate::postparsing::itemplatatype::ITemplataType,
  >>(
    &self,
    state: &(),
    env: &E,
    rule_index: i32,
    rule: &crate::postparsing::rules::rules::IRulexSR<'a>,
    solver_state: &mut S,
  ) -> Result<(), crate::solver::solver::ISolverError<
    crate::postparsing::names::IRuneS<'a>,
    crate::postparsing::itemplatatype::ITemplataType,
    IRuneTypeRuleError<'a>,
  >> {
    panic!("RuneTypeSolverDelegate::solve not yet migrated")
  }

  fn complex_solve<S: crate::solver::ISolverState<
    crate::postparsing::rules::rules::IRulexSR<'a>,
    crate::postparsing::names::IRuneS<'a>,
    crate::postparsing::itemplatatype::ITemplataType,
  >>(
    &self,
    state: &(),
    env: &E,
    solver_state: &mut S,
  ) -> Result<(), crate::solver::solver::ISolverError<
    crate::postparsing::names::IRuneS<'a>,
    crate::postparsing::itemplatatype::ITemplataType,
    IRuneTypeRuleError<'a>,
  >> {
    // Scala: Ok(())
    Ok(())
  }

  fn sanity_check_conclusion(
    &self,
    env: &E,
    state: &(),
    rune: &crate::postparsing::names::IRuneS<'a>,
    conclusion: &crate::postparsing::itemplatatype::ITemplataType,
  ) {
    // Scala: Unit = {} (no-op)
  }
}

// mig: impl RuneTypeSolver
impl<'a, 'ctx> RuneTypeSolver<'a, 'ctx> {
/*
class RuneTypeSolver(interner: Interner) {
*/
// mig: fn get_runes_rune_type
fn get_runes_rune_type(
  _rule: &crate::postparsing::rules::rules::IRulexSR<'a>,
) -> Vec<crate::postparsing::names::IRuneS<'a>> {
  panic!("Unimplemented get_runes_rune_type");
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
fn get_puzzles_rune_type(
  _predicting: bool,
  _rule: &crate::postparsing::rules::rules::IRulexSR<'a>,
) -> Vec<Vec<crate::postparsing::names::IRuneS<'a>>> {
  panic!("Unimplemented get_puzzles_rune_type");
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
fn solve_rule(
  _state: (),
  _rule_index: usize,
  _rule: &crate::postparsing::rules::rules::IRulexSR<'a>,
) -> Result<(), ()> {
  panic!("Unimplemented solve_rule");
}
/*
  private def solveRule(
    state: Unit,
    env: IRuneTypeSolverEnv,
    ruleIndex: Int,
    rule: IRulexSR,
    stepState: IStepState[IRulexSR, IRuneS, ITemplataType]):
  Result[Unit, ISolverError[IRuneS, ITemplataType, IRuneTypeRuleError]] = {
    rule match {
      case KindComponentsSR(range, resultRune, mutabilityRune) => {
        stepState.concludeRune(List(range), resultRune.rune, KindTemplataType())
        stepState.concludeRune(List(range), mutabilityRune.rune, MutabilityTemplataType())
        Ok(())
      }
      case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => {
        stepState.concludeRune(List(range), resultRune.rune, CoordTemplataType())
        stepState.concludeRune(List(range), ownershipRune.rune, OwnershipTemplataType())
        stepState.concludeRune(List(range), kindRune.rune, KindTemplataType())
        Ok(())
      }
      case PrototypeComponentsSR(range, resultRune, paramsRune, returnRune) => {
        stepState.concludeRune(List(range), resultRune.rune, PrototypeTemplataType())
        stepState.concludeRune(List(range), paramsRune.rune, PackTemplataType(CoordTemplataType()))
        stepState.concludeRune(List(range), returnRune.rune, CoordTemplataType())
        Ok(())
      }
      case MaybeCoercingCallSR(range, resultRune, templateRune, argRunes) => {
        vassertSome(stepState.getConclusion(templateRune.rune)) match {
          case TemplateTemplataType(paramTypes, returnType) => {
            argRunes.map(_.rune).zip(paramTypes).foreach({ case (argRune, paramType) =>
              stepState.concludeRune(List(range), argRune, paramType)
            })
            Ok(())
          }
          case other => vwat(other)
        }
      }
      case ResolveSR(range, resultRune, name, paramListRune, returnRune) => {
        stepState.concludeRune(List(range), resultRune.rune, PrototypeTemplataType())
        stepState.concludeRune(List(range), paramListRune.rune, PackTemplataType(CoordTemplataType()))
        stepState.concludeRune(List(range), returnRune.rune, CoordTemplataType())
        Ok(())
      }
      case CallSiteFuncSR(range, resultRune, name, paramListRune, returnRune) => {
        stepState.concludeRune(List(range), resultRune.rune, PrototypeTemplataType())
        stepState.concludeRune(List(range), paramListRune.rune, PackTemplataType(CoordTemplataType()))
        stepState.concludeRune(List(range), returnRune.rune, CoordTemplataType())
        Ok(())
      }
      case DefinitionFuncSR(range, resultRune, name, paramListRune, returnRune) => {
        stepState.concludeRune(List(range), resultRune.rune, PrototypeTemplataType())
        stepState.concludeRune(List(range), paramListRune.rune, PackTemplataType(CoordTemplataType()))
        stepState.concludeRune(List(range), returnRune.rune, CoordTemplataType())
        Ok(())
      }
      case DefinitionCoordIsaSR(range, resultRune, subRune, superRune) => {
        stepState.concludeRune(List(range), resultRune.rune, ImplTemplataType())
        stepState.concludeRune(List(range), subRune.rune, CoordTemplataType())
        stepState.concludeRune(List(range), superRune.rune, CoordTemplataType())
        Ok(())
      }
      case CallSiteCoordIsaSR(range, resultRune, subRune, superRune) => {
        resultRune match {
          case Some(resultRune) => stepState.concludeRune(List(range), resultRune.rune, ImplTemplataType())
          case None =>
        }
        stepState.concludeRune(List(range), subRune.rune, CoordTemplataType())
        stepState.concludeRune(List(range), superRune.rune, CoordTemplataType())
        Ok(())
      }
      case OneOfSR(range, resultRune, literals) => {
        val types = literals.map(_.getType()).toSet
        if (types.size > 1) {
          vfail("OneOf rule's possibilities must all be the same type!")
        }
        stepState.concludeRune(List(range), resultRune.rune, types.head)
        Ok(())
      }
      case EqualsSR(range, leftRune, rightRune) => {
        stepState.getConclusion(leftRune.rune) match {
          case None => {
            stepState.concludeRune(List(range), leftRune.rune, vassertSome(stepState.getConclusion(rightRune.rune)))
            Ok(())
          }
          case Some(left) => {
            stepState.concludeRune(List(range), rightRune.rune, left)
            Ok(())
          }
        }
      }
      case IsConcreteSR(range, rune) => {
        stepState.concludeRune(List(range), rune.rune, KindTemplataType())
        Ok(())
      }
      case IsInterfaceSR(range, rune) => {
        stepState.concludeRune(List(range), rune.rune, KindTemplataType())
        Ok(())
      }
      case IsStructSR(range, rune) => {
        stepState.concludeRune(List(range), rune.rune, KindTemplataType())
        Ok(())
      }
      case RefListCompoundMutabilitySR(range, resultRune, coordListRune) => {
        stepState.concludeRune(List(range), resultRune.rune, MutabilityTemplataType())
        stepState.concludeRune(List(range), coordListRune.rune, PackTemplataType(CoordTemplataType()))
        Ok(())
      }
      case CoerceToCoordSR(range, coordRune, kindRune) => {
        stepState.concludeRune(List(range), coordRune.rune, CoordTemplataType())
        stepState.concludeRune(List(range), kindRune.rune, KindTemplataType())
        Ok(())
      }
      case LiteralSR(range, rune, literal) => {
        stepState.concludeRune(List(range), rune.rune, literal.getType())
        Ok(())
      }
      case LookupSR(range, resultRune, name) => {
        val actualLookupResult =
          env.lookup(range, name) match {
            case Err(e) => return Err(RuleError(e))
            case Ok(x) => x
          }
        actualLookupResult match {
          case PrimitiveRuneTypeSolverLookupResult(tyype) => {
            stepState.concludeRune(List(range), resultRune.rune, tyype)
          }
          case TemplataLookupResult(actualType) => {
            stepState.concludeRune(List(range), resultRune.rune, actualType)
          }
          case CitizenRuneTypeSolverLookupResult(tyype, genericParams) => {
            stepState.concludeRune(List(range), resultRune.rune, tyype)
          }
        }
        Ok(())
      }
      case MaybeCoercingLookupSR(range, rune, name) => {
        val actualLookupResult =
          env.lookup(range, name) match {
            case Err(e) => return Err(RuleError(e))
            case Ok(x) => x
          }
        lookup(env, stepState, range, rune, actualLookupResult)
      }
      case RuneParentEnvLookupSR(range, rune) => {
        val actualLookupResult =
          env.lookup(range, interner.intern(RuneNameS(rune.rune))) match {
            case Err(e) => return Err(RuleError(e))
            case Ok(x) => x
          }
        lookup(env, stepState, range, rune, actualLookupResult)
      }
      case AugmentSR(range, resultRune, ownership, innerRune) => {
        stepState.concludeRune(List(range), resultRune.rune, CoordTemplataType())
        stepState.concludeRune(List(range), innerRune.rune, CoordTemplataType())
        Ok(())
      }
      case PackSR(range, resultRune, memberRunes) => {
        memberRunes.foreach(x => stepState.concludeRune(List(range), x.rune, CoordTemplataType()))
        stepState.concludeRune(List(range), resultRune.rune, PackTemplataType(CoordTemplataType()))
        Ok(())
      }
//      case StaticSizedArraySR(range, resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune) => {
//        stepState.concludeRune(List(range), mutabilityRune.rune, MutabilityTemplataType())
//        stepState.concludeRune(List(range), variabilityRune.rune, VariabilityTemplataType())
//        stepState.concludeRune(List(range), sizeRune.rune, IntegerTemplataType())
//        stepState.concludeRune(List(range), elementRune.rune, CoordTemplataType())
//        Ok(())
//      }
//      case RuntimeSizedArraySR(range, resultRune, mutabilityRune, elementRune) => {
//        stepState.concludeRune(List(range), mutabilityRune.rune, MutabilityTemplataType())
//        stepState.concludeRune(List(range), elementRune.rune, CoordTemplataType())
//        Ok(())
//      }
    }
  }
*/
// mig: fn lookup
fn lookup(
  _env: (),
  _step_state: (),
  _range: crate::utils::range::RangeS<'a>,
  _rune: (),
  _actual_lookup_result: IRuneTypeSolverLookupResult<'a>,
) -> Result<(), ()> {
  panic!("Unimplemented lookup");
}
/*
  private def lookup(
      env: IRuneTypeSolverEnv,
      stepState: IStepState[IRulexSR, IRuneS, ITemplataType],
      range: RangeS,
      rune: RuneUsage,
      actualLookupResult: IRuneTypeSolverLookupResult):
  Result[Unit, ISolverError[IRuneS, ITemplataType, IRuneTypeRuleError]] = {
    val expectedType = vassertSome(stepState.getConclusion(rune.rune))
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
pub fn solve_rune_type<E: IRuneTypeSolverEnv<'a>>(
  &self,
  sanity_check: bool,
  env: &E,
  range: Vec<crate::utils::range::RangeS<'a>>,
  predicting: bool,
  rules_s: &[crate::postparsing::rules::rules::IRulexSR<'a>],
  additional_runes: &[crate::postparsing::names::IRuneS<'a>],
  expect_complete_solve: bool,
  unpreprocessed_initially_known_runes: std::collections::HashMap<crate::postparsing::names::IRuneS<'a>, crate::postparsing::itemplatatype::ITemplataType>,
) -> Result<
  std::collections::HashMap<crate::postparsing::names::IRuneS<'a>, crate::postparsing::itemplatatype::ITemplataType>,
  RuneTypeSolveError<'a>,
> {
  use crate::postparsing::names::IRuneS;
  use crate::postparsing::itemplatatype::ITemplataType;
  use crate::postparsing::rules::rules::IRulexSR;
  use crate::solver::solver::{Solver, ISolverError};
  use std::collections::HashMap;

  // Scala: val initiallyKnownRunes = (if (predicting) Map() else rules.flatMap({...}).toMap) ++ unpreprocessedInitiallyKnownRunes
  // For the non-predicting case, iterate over LookupSR/MaybeCoercingLookupSR rules and pre-compute types via env.lookup.
  // For now, with no rules in the simple test case, this is empty.
  let mut initially_known_runes: HashMap<IRuneS<'a>, ITemplataType> = if predicting {
    HashMap::new()
  } else {
    let mut map = HashMap::new();
    for rule in rules_s {
      match rule {
        IRulexSR::Lookup(lookup) => {
          match env.lookup(lookup.range.clone(), lookup.name.clone()) {
            Err(_e) => {
              return Err(RuneTypeSolveError {
                range: range.clone(),
                failed_solve: (),
              });
            }
            Ok(_result) => {
              // Complex coercion logic for different lookup result types.
              // For now, panic if we actually hit a lookup (the simple test has none).
              panic!("LookupSR pre-computation not yet fully migrated");
            }
          }
        }
        IRulexSR::MaybeCoercingLookup(_lookup) => {
          panic!("MaybeCoercingLookupSR pre-computation not yet fully migrated");
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

  // Compute all_runes = rules.flatMap(getRunes) ++ initiallyKnownRunes.keys ++ additionalRunes, deduplicated
  let mut all_runes_set = std::collections::HashSet::new();
  for rule in rules_s {
    for rune_usage in rule.rune_usages() {
      all_runes_set.insert(rune_usage.rune.clone());
    }
  }
  for k in initially_known_runes.keys() {
    all_runes_set.insert(k.clone());
  }
  for r in additional_runes {
    all_runes_set.insert(r.clone());
  }
  let all_runes: Vec<IRuneS<'a>> = all_runes_set.into_iter().collect();

  let delegate = RuneTypeSolverDelegate { predicting };
  let mut solver: Solver<'a, IRulexSR<'a>, IRuneS<'a>, E, (), ITemplataType, IRuneTypeRuleError<'a>, RuneTypeSolverDelegate> = Solver::new(
    sanity_check,
    delegate,
    range.clone(),
    rules_s.to_vec(),
    initially_known_runes,
    all_runes.clone(),
  );

  // Scala: while ({ solver.advance(env, Unit) match { ... } }) {}
  loop {
    match solver.advance(env, &()) {
      Ok(true) => continue,
      Ok(false) => break,
      Err(e) => {
        return Err(RuneTypeSolveError {
          range,
          failed_solve: (),
        });
      }
    }
  }

  let conclusions: HashMap<IRuneS<'a>, ITemplataType> = solver.userify_conclusions().into_iter().collect();

  // Check completeness
  let unsolved_runes: Vec<IRuneS<'a>> = all_runes.iter()
    .filter(|r| !conclusions.contains_key(*r))
    .cloned()
    .collect();

  if expect_complete_solve && !unsolved_runes.is_empty() {
    Err(RuneTypeSolveError {
      range,
      failed_solve: (),
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
    val initiallyKnownRunes =
        (if (predicting) {
          Map()
        } else {
          // Calculate what types we can beforehand, see KVCIE.
          rules.flatMap({
            case LookupSR(range, rune, name) => {
              name match {
                case CodeNameS(StrI("Array")) => {
                  vpass()
                }
                case _ =>
              }
              env.lookup(range, name) match {
                case Err(e) => {
                  return Err(
                    RuneTypeSolveError(
                      List(range),
                      FailedSolve(Vector().toStream, rules.toVector, RuleError(e))))
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
              name match {
                case CodeNameS(StrI("Array")) => {
                  vpass()
                }
                case _ =>
              }
              env.lookup(range, name) match {
                case Err(e) => {
                  return Err(
                    RuneTypeSolveError(
                      List(range),
                      FailedSolve(Vector().toStream, rules.toVector, RuleError(e))))
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
    val solver =
      new Solver[IRulexSR, IRuneS, IRuneTypeSolverEnv, Unit, ITemplataType, IRuneTypeRuleError](
        sanityCheck,
        useOptimizedSolver,
        interner,
        (rule: IRulexSR) => getPuzzles(predicting, rule),
        getRunes,
        new ISolveRule[IRulexSR, IRuneS, IRuneTypeSolverEnv, Unit, ITemplataType, IRuneTypeRuleError] {
*/

// mig: fn sanity_check_conclusion
fn sanity_check_conclusion(
  _rune: crate::postparsing::names::IRuneS<'a>,
  _conclusion: &crate::postparsing::itemplatatype::ITemplataType,
) {
  panic!("Unimplemented sanity_check_conclusion");
}
/*
          override def sanityCheckConclusion(env: IRuneTypeSolverEnv, state: Unit, rune: IRuneS, conclusion: ITemplataType): Unit = {}
*/
// mig: fn complex_solve
fn complex_solve() -> Result<(), ()> {
  panic!("Unimplemented complex_solve");
}
/*
          // MIGALLOW: complexSolve -> complex_solve
          override def complexSolve(state: Unit, env: IRuneTypeSolverEnv, solverState: ISolverState[IRulexSR, IRuneS, ITemplataType], stepState: IStepState[IRulexSR, IRuneS, ITemplataType]): Result[Unit, ISolverError[IRuneS, ITemplataType, IRuneTypeRuleError]] = {
            Ok(())
          }
*/
// mig: fn solve
fn solve(
  _state: (),
  _env: (),
  _solver_state: (),
  _rule_index: usize,
  _rule: &crate::postparsing::rules::rules::IRulexSR<'a>,
  _step_state: (),
) -> Result<(), ()> {
  panic!("Unimplemented solve");
}
/*
          // MIGALLOW: solve -> solve_rune_type
          override def solve(state: Unit, env: IRuneTypeSolverEnv, solverState: ISolverState[IRulexSR, IRuneS, ITemplataType], ruleIndex: Int, rule: IRulexSR, stepState: IStepState[IRulexSR, IRuneS, ITemplataType]): Result[Unit, ISolverError[IRuneS, ITemplataType, IRuneTypeRuleError]] = {
            solveRule(state, env, ruleIndex, rule, stepState)
          }
        },
        range,
        rules,
        initiallyKnownRunes,
        (rules.flatMap(getRunes) ++ initiallyKnownRunes.keys).distinct.toVector)
    while ({
      solver.advance(env, Unit) match {
        case Ok(continue) => continue
        case Err(e) => return Err(RuneTypeSolveError(range, e))
      }
    }) {}
    val steps = solver.getSteps().toStream
    val conclusions = solver.userifyConclusions().toMap

    val allRunes = solver.getAllRunes().map(solver.getUserRune) ++ additionalRunes
    val unsolvedRunes = allRunes -- conclusions.keySet
    if (expectCompleteSolve && unsolvedRunes.nonEmpty) {
      Err(
        RuneTypeSolveError(
          range,
          IncompleteSolve(
            steps,
            solver.getUnsolvedRules(),
            unsolvedRunes,
            conclusions)))
    } else {
      Ok(conclusions)
    }
  }
}
*/
} // end impl RuneTypeSolver
/*
object RuneTypeSolver {
*/
// mig: fn check_generic_call_without_defaults
fn check_generic_call_without_defaults<'a>(
  _param_types: &[crate::postparsing::itemplatatype::ITemplataType],
  _arg_types: &[crate::postparsing::itemplatatype::ITemplataType],
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
fn check_generic_call<'a>(
  _param_types: &[crate::postparsing::itemplatatype::ITemplataType],
  _arg_types: &[crate::postparsing::itemplatatype::ITemplataType],
) -> Result<(), ()> {
  panic!("Unimplemented check_generic_call");
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