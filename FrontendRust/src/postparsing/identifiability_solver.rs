use crate::scout_arena::ScoutArena;
use crate::postparsing::names::IRuneS;
use crate::postparsing::rules::rules::IRulexSR;
use crate::solver::{
    IncompleteOrFailedSolve, IncompleteSolve, ISolverError, Solver, SolverDelegate,
};
use crate::utils::range::RangeS;
use std::collections::{HashMap, HashSet};
/*
package dev.vale.postparsing

import dev.vale.postparsing.rules._
import dev.vale.solver.{FailedSolve, ISolverError, SimpleSolverState, SolveIncomplete, Solver}
import dev.vale.{Err, Ok, RangeS, Result, vassert, vimpl, vpass}
import dev.vale._
import dev.vale.postparsing.rules._

import scala.collection.immutable.Map
*/

#[derive(Clone, Debug, PartialEq)]
pub struct IdentifiabilitySolveError<'s> {
  pub range: Vec<RangeS<'s>>,
  pub failed_solve: IncompleteOrFailedSolve<IRulexSR<'s>, IRuneS<'s>, bool, IIdentifiabilityRuleError>,
}
/*
case class IdentifiabilitySolveError(range: List[RangeS], failedSolve: FailedSolve[IRulexSR, IRuneS, Boolean, IIdentifiabilityRuleError]) {
  vpass()
}
*/
/*
sealed trait IIdentifiabilityRuleError
*/
#[derive(Clone, Debug, PartialEq)]
pub enum IIdentifiabilityRuleError {}

struct IdentifiabilitySolverDelegate<'s> {
  call_range: Vec<RangeS<'s>>,
}
/* Guardian: disable-all */

impl<'s> SolverDelegate<IRulexSR<'s>, IRuneS<'s>, (), (), bool, IIdentifiabilityRuleError>
  for IdentifiabilitySolverDelegate<'s>
{
  fn rule_to_puzzles(&self, rule: &IRulexSR<'s>) -> Vec<Vec<IRuneS<'s>>> {
    get_puzzles(rule)
  }
  /* Guardian: disable-all */

  fn rule_to_runes(&self, rule: &IRulexSR<'s>) -> Vec<IRuneS<'s>> {
    get_runes(rule)
  }
  /* Guardian: disable-all */

  fn solve<S: crate::solver::ISolverState<IRulexSR<'s>, IRuneS<'s>, bool>>(
    &self,
    _state: &(),
    _env: &(),
    rule_index: i32,
    rule: &IRulexSR<'s>,
    solver_state: &mut S,
  ) -> Result<(), ISolverError<IRuneS<'s>, bool, IIdentifiabilityRuleError>> {
    solve_rule_impl(
      rule_index,
      &self.call_range,
      rule,
      solver_state,
    )
  }
  /* Guardian: disable-all */

  fn complex_solve<S: crate::solver::ISolverState<IRulexSR<'s>, IRuneS<'s>, bool>>(
    &self,
    _state: &(),
    _env: &(),
    _solver_state: &mut S,
  ) -> Result<(), ISolverError<IRuneS<'s>, bool, IIdentifiabilityRuleError>> {
    Ok(())
  }
  /* Guardian: disable-all */

  fn sanity_check_conclusion(
    &self,
    _env: &(),
    _state: &(),
    _rune: &IRuneS<'s>,
    _conclusion: &bool,
  ) {
  }
  /* Guardian: disable-all */
}
/*
// Identifiability is whether the denizen has enough identifying runes to uniquely identify all its
// instantiations. It's only used as a check, and will throw an error if there's a rune that can't
// be derived from the identifying runes.
object IdentifiabilitySolver {
*/
fn get_runes<'s>(rule: &IRulexSR<'s>) -> Vec<IRuneS<'s>>
where {
  rule.rune_usages().into_iter().map(|u| u.rune).collect()
}
/*
  def getRunes(rule: IRulexSR): Vector[IRuneS] = {
    val sanityCheck =
      rule match {
        case MaybeCoercingLookupSR(range, rune, literal) => Vector(rune)
        case LookupSR(range, rune, literal) => Vector(rune)
        case RuneParentEnvLookupSR(range, rune) => Vector(rune)
        case EqualsSR(range, left, right) => Vector(left, right)
        case KindComponentsSR(range, resultRune, mutabilityRune) => Vector(resultRune, mutabilityRune)
        case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => Vector(resultRune, ownershipRune, kindRune)
        case PrototypeComponentsSR(range, resultRune, paramsRune, returnRune) => Vector(resultRune, paramsRune, returnRune)
        case ResolveSR(range, resultRune, name, paramsListRune, returnRune) => Vector(resultRune, paramsListRune, returnRune)
        case CallSiteFuncSR(range, prototypeRune, name, paramsListRune, returnRune) => Vector(prototypeRune, paramsListRune, returnRune)
        case DefinitionFuncSR(range, resultRune, name, paramsListRune, returnRune) => Vector(resultRune, paramsListRune, returnRune)
        case CallSiteCoordIsaSR(range, resultRune, sub, suuper) => resultRune.toVector ++ Vector(sub, suuper)
        case DefinitionCoordIsaSR(range, resultRune, sub, suuper) => Vector(resultRune, sub, suuper)
        case OneOfSR(range, rune, literals) => Vector(rune)
        case IsConcreteSR(range, rune) => Vector(rune)
        case IsInterfaceSR(range, rune) => Vector(rune)
        case IsStructSR(range, rune) => Vector(rune)
        case CoerceToCoordSR(range, coordRune, kindRune) => Vector(coordRune, kindRune)
        case LiteralSR(range, rune, literal) => Vector(rune)
        case AugmentSR(range, resultRune, maybeOwnership, innerRune) => Vector(resultRune, innerRune)
        case MaybeCoercingCallSR(range, resultRune, templateRune, args) => Vector(resultRune, templateRune) ++ args
//        case PrototypeSR(range, resultRune, name, parameters, returnTypeRune) => Vector(resultRune) ++ parameters ++ Vector(returnTypeRune)
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
fn get_puzzles<'s>(rule: &IRulexSR<'s>) -> Vec<Vec<IRuneS<'s>>> {
  match rule {
    IRulexSR::Equals(x) => vec![vec![x.left.rune.clone()], vec![x.right.rune.clone()]],
    IRulexSR::MaybeCoercingLookup(_) => vec![vec![]],
    IRulexSR::Lookup(_) => vec![vec![]],
    IRulexSR::RuneParentEnvLookup(_) => {
      // This Vector() literally means nothing can solve this puzzle.
      // It needs to be passed in via identifying rune.
      vec![]
    }
    IRulexSR::MaybeCoercingCall(x) => {
      // We can't determine the template from the result and args because we might be coercing its
      // returned kind to a coord. So we need the template.
      // We can't determine the return type because we don't know whether we're coercing or not.
      let mut second = vec![x.template_rune.rune.clone()];
      second.extend(x.args.iter().map(|a| a.rune.clone()));
      vec![
        vec![x.result_rune.rune.clone(), x.template_rune.rune.clone()],
        second,
      ]
    }
    IRulexSR::Pack(x) => {
      // Packs are always lists of coords
      vec![vec![x.result_rune.rune.clone()], x.members.iter().map(|m| m.rune.clone()).collect()]
    }
    IRulexSR::DefinitionCoordIsa(_) => panic!("IRulexSR::DefinitionCoordIsa not yet migrated in identifiability get_puzzles"),
    IRulexSR::CallSiteCoordIsa(_) => panic!("IRulexSR::CallSiteCoordIsa not yet migrated in identifiability get_puzzles"),
    IRulexSR::KindComponents(_) => panic!("IRulexSR::KindComponents not yet migrated in identifiability get_puzzles"),
    IRulexSR::CoordComponents(_) => vec![vec![]],
    IRulexSR::PrototypeComponents(_) => vec![vec![]],
    IRulexSR::Resolve(_) => vec![vec![]],
    IRulexSR::CallSiteFunc(_) => vec![vec![]],
    IRulexSR::DefinitionFunc(_) => vec![vec![]],
    IRulexSR::OneOf(_) => vec![vec![]],
    IRulexSR::IsConcrete(_) => panic!("IRulexSR::IsConcrete not yet migrated in identifiability get_puzzles"),
    IRulexSR::IsInterface(_) => vec![vec![]],
    IRulexSR::IsStruct(_) => panic!("IRulexSR::IsStruct not yet migrated in identifiability get_puzzles"),
    IRulexSR::CoerceToCoord(_) => vec![vec![]],
    IRulexSR::Literal(_) => vec![vec![]],
    IRulexSR::Augment(_) => vec![vec![]],
    IRulexSR::Call(_) => panic!("IRulexSR::Call not yet migrated in identifiability get_puzzles"),
    IRulexSR::CoordSend(_) => panic!("IRulexSR::CoordSend not yet migrated in identifiability get_puzzles"),
    IRulexSR::RefListCompoundMutability(_) => panic!("IRulexSR::RefListCompoundMutability not yet migrated in identifiability get_puzzles"),
    IRulexSR::IndexList(_) => panic!("IRulexSR::IndexList not yet migrated in identifiability get_puzzles"),
  }
}
/*
  def getPuzzles(rule: IRulexSR): Vector[Vector[IRuneS]] = {
    rule match {
      case EqualsSR(range, leftRune, rightRune) => Vector(Vector(leftRune.rune), Vector(rightRune.rune))
      case MaybeCoercingLookupSR(range, rune, _) => Vector(Vector())
      case LookupSR(range, rune, _) => Vector(Vector())
      case RuneParentEnvLookupSR(range, rune) => {
        // This Vector() literally means nothing can solve this puzzle.
        // It needs to be passed in via identifying rune.
        Vector()
      }
      case MaybeCoercingCallSR(range, resultRune, templateRune, args) => {
        // We can't determine the template from the result and args because we might be coercing its
        // returned kind to a coord. So we need the template.
        // We can't determine the return type because we don't know whether we're coercing or not.
        Vector(Vector(resultRune.rune, templateRune.rune), Vector(templateRune.rune) ++ args.map(_.rune))
      }
      case PackSR(range, resultRune, members) => {
        // Packs are always lists of coords
        Vector(Vector(resultRune.rune), members.map(_.rune))
      }
      case DefinitionCoordIsaSR(range, resultRune, subRune, superRune) => Vector(Vector())
      case CallSiteCoordIsaSR(range, resultRune, subRune, superRune) => Vector(Vector())
      case KindComponentsSR(range, resultRune, mutabilityRune) => Vector(Vector())
      case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => Vector(Vector())
      case PrototypeComponentsSR(range, resultRune, ownershipRune, kindRune) => Vector(Vector())
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
//      case StaticSizedArraySR(range, resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune) => Vector(Vector(resultRune.rune), Vector(mutabilityRune.rune, variabilityRune.rune, sizeRune.rune, elementRune.rune))
//      case RuntimeSizedArraySR(range, resultRune, mutabilityRune, elementRune) => Vector(Vector(resultRune.rune), Vector(mutabilityRune.rune, elementRune.rune))

//      case ManualSequenceSR(range, resultRune, elements) => Vector(Vector(resultRune.rune))
      case RefListCompoundMutabilitySR(range, resultRune, coordListRune) => Vector(Vector())
        // solverState.addPuzzle(ruleIndex, Vector(senderRune, receiverRune))
//      case CoordListSR(range, resultRune, elements) => Vector(Vector())
    }
  }
*/
fn solve_rule_impl<'s, S: crate::solver::ISolverState<IRulexSR<'s>, IRuneS<'s>, bool>>(
  _rule_index: i32,
  call_range: &[RangeS<'s>],
  rule: &IRulexSR<'s>,
  solver_state: &mut S,
) -> Result<(), ISolverError<IRuneS<'s>, bool, IIdentifiabilityRuleError>> {
  let mut range_s = vec![rule.range().clone()];
  range_s.extend(call_range.iter().cloned());
  match rule {
    IRulexSR::KindComponents(_) => panic!("IRulexSR::KindComponents not yet migrated in identifiability solve_rule"),
    IRulexSR::CoordComponents(x) => {
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s.clone(),
        x.result_rune.rune.clone(),
        true,
      )?;
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s.clone(),
        x.ownership_rune.rune.clone(),
        true,
      )?;
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s, x.kind_rune.rune.clone(), true,
      )?;
      Ok(())
    }
    IRulexSR::PrototypeComponents(x) => {
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s.clone(), x.result_rune.rune.clone(), true,
      )?;
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s.clone(), x.params_rune.rune.clone(), true,
      )?;
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s, x.return_rune.rune.clone(), true,
      )?;
      Ok(())
    }
    IRulexSR::MaybeCoercingCall(x) => {
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s.clone(),
        x.result_rune.rune.clone(),
        true,
      )?;
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s.clone(),
        x.template_rune.rune.clone(),
        true,
      )?;
      for arg in x.args {
        solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
          range_s.clone(),
          arg.rune.clone(),
          true,
        )?;
      }
      Ok(())
    }
    IRulexSR::Resolve(x) => {
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s.clone(), x.result_rune.rune.clone(), true,
      )?;
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s.clone(), x.params_list_rune.rune.clone(), true,
      )?;
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s, x.return_rune.rune.clone(), true,
      )?;
      Ok(())
    }
    IRulexSR::CallSiteFunc(x) => {
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s.clone(), x.prototype_rune.rune.clone(), true,
      )?;
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s.clone(), x.params_list_rune.rune.clone(), true,
      )?;
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s, x.return_rune.rune.clone(), true,
      )?;
      Ok(())
    }
    IRulexSR::DefinitionFunc(x) => {
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s.clone(), x.result_rune.rune.clone(), true,
      )?;
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s.clone(), x.params_list_rune.rune.clone(), true,
      )?;
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s, x.return_rune.rune.clone(), true,
      )?;
      Ok(())
    }
    IRulexSR::DefinitionCoordIsa(_) => panic!("IRulexSR::DefinitionCoordIsa not yet migrated in identifiability solve_rule"),
    IRulexSR::CallSiteCoordIsa(_) => panic!("IRulexSR::CallSiteCoordIsa not yet migrated in identifiability solve_rule"),
    IRulexSR::OneOf(x) => {
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s, x.rune.rune.clone(), true,
      )?;
      Ok(())
    }
    IRulexSR::Equals(x) => {
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s.clone(),
        x.left.rune.clone(),
        true,
      )?;
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s, x.right.rune.clone(), true,
      )?;
      Ok(())
    }
    IRulexSR::IsConcrete(_) => panic!("IRulexSR::IsConcrete not yet migrated in identifiability solve_rule"),
    IRulexSR::IsInterface(x) => {
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s, x.rune.rune.clone(), true,
      )?;
      Ok(())
    }
    IRulexSR::IsStruct(_) => panic!("IRulexSR::IsStruct not yet migrated in identifiability solve_rule"),
    IRulexSR::RefListCompoundMutability(_) => panic!("IRulexSR::RefListCompoundMutability not yet migrated in identifiability solve_rule"),
    IRulexSR::CoerceToCoord(x) => {
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s.clone(),
        x.kind_rune.rune.clone(),
        true,
      )?;
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s, x.coord_rune.rune.clone(), true,
      )?;
      Ok(())
    }
    IRulexSR::Literal(x) => {
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s, x.rune.rune.clone(), true,
      )?;
      Ok(())
    }
    IRulexSR::Lookup(x) => {
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s, x.rune.rune.clone(), true,
      )?;
      Ok(())
    }
    IRulexSR::MaybeCoercingLookup(x) => {
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s, x.rune.rune.clone(), true,
      )?;
      Ok(())
    }
    IRulexSR::RuneParentEnvLookup(_) => {
      panic!("unimplemented");
    }
    IRulexSR::Augment(x) => {
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s.clone(),
        x.result_rune.rune.clone(),
        true,
      )?;
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s, x.inner_rune.rune.clone(), true,
      )?;
      Ok(())
    }
    IRulexSR::Call(_) => panic!("IRulexSR::Call not yet migrated in identifiability solve_rule"),
    IRulexSR::CoordSend(_) => panic!("IRulexSR::CoordSend not yet migrated in identifiability solve_rule"),
    IRulexSR::Pack(x) => {
      for member in x.members {
        solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
          range_s.clone(), member.rune.clone(), true,
        )?;
      }
      solver_state.step_conclude_rune::<IIdentifiabilityRuleError>(
        range_s, x.result_rune.rune.clone(), true,
      )?;
      Ok(())
    }
    IRulexSR::IndexList(_) => panic!("IRulexSR::IndexList not yet migrated in identifiability solve_rule"),
  }
}
/*
  private def solveRule(
    solverState: SimpleSolverState[IRulexSR, IRuneS, Boolean],
    ruleIndex: Int,
    rule: IRulexSR):
  Result[Unit, ISolverError[IRuneS, Boolean, IIdentifiabilityRuleError]] = {
    rule match {
      case KindComponentsSR(range, resultRune, mutabilityRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, mutabilityRune.rune -> true), Vector())
      }
      case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, ownershipRune.rune -> true, kindRune.rune -> true), Vector())
      }
      case PrototypeComponentsSR(range, resultRune, paramsRune, returnRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, paramsRune.rune -> true, returnRune.rune -> true), Vector())
      }
      case MaybeCoercingCallSR(range, resultRune, templateRune, argRunes) => {
        val conclusions =
          argRunes.map(_.rune).map({ case argRune => (argRune -> true) }).toMap ++
              Map(resultRune.rune -> true, templateRune.rune -> true)
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), conclusions, Vector())
      }
      case ResolveSR(range, resultRune, name, paramListRune, returnRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, paramListRune.rune -> true, returnRune.rune -> true), Vector())
      }
      case CallSiteFuncSR(range, resultRune, name, paramListRune, returnRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, paramListRune.rune -> true, returnRune.rune -> true), Vector())
      }
      case DefinitionFuncSR(range, resultRune, name, paramsListRune, returnRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, paramsListRune.rune -> true, returnRune.rune -> true), Vector())
      }
      case DefinitionCoordIsaSR(range, resultRune, subRune, superRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, subRune.rune -> true, superRune.rune -> true), Vector())
      }
      case CallSiteCoordIsaSR(range, resultRune, subRune, superRune) => {
        val conclusions = Map(subRune.rune -> true, superRune.rune -> true) ++
            (resultRune match {
              case None => Map()
              case Some(resultRune) => Map(resultRune.rune -> true)
            })
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), conclusions, Vector())
      }
      case OneOfSR(range, resultRune, literals) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true), Vector())
      }
      case EqualsSR(range, leftRune, rightRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(leftRune.rune -> true, rightRune.rune -> true), Vector())
      }
      case IsConcreteSR(range, rune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(rune.rune -> true), Vector())
      }
      case IsInterfaceSR(range, rune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(rune.rune -> true), Vector())
      }
      case IsStructSR(range, rune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(rune.rune -> true), Vector())
      }
      case RefListCompoundMutabilitySR(range, resultRune, coordListRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, coordListRune.rune -> true), Vector())
      }
      case CoerceToCoordSR(range, coordRune, kindRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(kindRune.rune -> true, coordRune.rune -> true), Vector())
      }
      case LiteralSR(range, rune, literal) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(rune.rune -> true), Vector())
      }
      case LookupSR(range, rune, name) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(rune.rune -> true), Vector())
      }
      case MaybeCoercingLookupSR(range, rune, name) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(rune.rune -> true), Vector())
      }
      case RuneParentEnvLookupSR(range, rune) => {
        vimpl()
//        (env(RuneNameS(rune.rune)), vassertSome(stepState.getConclusion(rune.rune))) match {
//          case (true, true) =>
//          case (TemplateTemplataType(Vector(), true), true) =>
//          case (TemplateTemplataType(Vector(), result), expected) if result == expected =>
//          case (from, to) if from == to =>
//          case (from, to) => {
//            return Err(SolverConflict(rune.rune, to, from))
//          }
//        }
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), vimpl(), Vector())
      }
      case MaybeCoercingLookupSR(range, rune, name) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(rune.rune -> true), Vector())
      }
      case AugmentSR(range, resultRune, ownership, innerRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, innerRune.rune -> true), Vector())
      }
      case PackSR(range, resultRune, memberRunes) => {
        val conclusions = Map(resultRune.rune -> true) ++ memberRunes.map(x => (x.rune -> true))
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), conclusions, Vector())
      }
//      case StaticSizedArraySR(range, resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune) => {
//        solverState.commitStep[IIdentifiabilityRuleError]resultRune.rune, true), Vector())
//        solverState.commitStep[IIdentifiabilityRuleError]mutabilityRune.rune, true), Vector())
//        solverState.commitStep[IIdentifiabilityRuleError]variabilityRune.rune, true), Vector())
//        solverState.commitStep[IIdentifiabilityRuleError]sizeRune.rune, true), Vector())
//        solverState.commitStep[IIdentifiabilityRuleError]elementRune.rune, true), Vector())
//        Ok(())
//      }
//      case RuntimeSizedArraySR(range, resultRune, mutabilityRune, elementRune) => {
//        solverState.commitStep[IIdentifiabilityRuleError]resultRune.rune, true), Vector())
//        solverState.commitStep[IIdentifiabilityRuleError]mutabilityRune.rune, true), Vector())
//        solverState.commitStep[IIdentifiabilityRuleError]elementRune.rune, true), Vector())
//        Ok(())
//      }
    }
  }
*/
pub(crate) fn solve_identifiability<'s>(
  sanity_check: bool,
  _use_optimized_solver: bool,
  _scout_arena: &ScoutArena<'s>,
  call_range: &[RangeS<'s>],
  rules: &'s [IRulexSR<'s>],
  identifying_runes: &[IRuneS<'s>],
) -> Result<HashMap<IRuneS<'s>, bool>, IdentifiabilitySolveError<'s>> {
  let initially_known_runes: HashMap<_, _> =
    identifying_runes.iter().map(|r| (r.clone(), true)).collect();

  let all_runes: Vec<IRuneS<'s>> = {
    let mut set = HashSet::new();
    let mut out = Vec::new();
    for r in rules
      .iter()
      .flat_map(get_runes)
      .chain(initially_known_runes.keys().cloned())
    {
      if set.insert(r.clone()) {
        out.push(r);
      }
    }
    out
  };

  let delegate = IdentifiabilitySolverDelegate {
    call_range: call_range.to_vec(),
  };
  let mut solver = Solver::new(
    sanity_check,
    delegate,
    call_range.to_vec(),
    rules.to_vec(),
    initially_known_runes,
    all_runes,
  );

  while {
    match solver.advance(&(), &()) {
      Ok(continue_) => continue_,
      Err(e) => {
        return Err(IdentifiabilitySolveError {
          range: call_range.to_vec(),
          failed_solve: IncompleteOrFailedSolve::Failed(e),
        })
      }
    }
  } {}
  // If we get here, then there's nothing more the solver can do.

  let steps = solver.get_steps();
  let conclusions: HashMap<_, _> = solver.userify_conclusions().into_iter().collect();

  let all_rune_ids = solver.get_all_runes();
  let all_runes_user: HashSet<IRuneS<'s>> = all_rune_ids
    .iter()
    .map(|&id| solver.get_user_rune(id))
    .collect();
  let conclusions_set: HashSet<_> = conclusions.keys().cloned().collect();
  let unsolved_runes: HashSet<_> = all_runes_user
    .difference(&conclusions_set)
    .cloned()
    .collect();

  if !unsolved_runes.is_empty() {
    Err(IdentifiabilitySolveError {
      range: call_range.to_vec(),
      failed_solve: IncompleteOrFailedSolve::Incomplete(IncompleteSolve {
        steps,
        unsolved_rules: solver.get_unsolved_rules(),
        unknown_runes: unsolved_runes,
        incomplete_conclusions: conclusions,
        _phantom: std::marker::PhantomData,
      }),
    })
  } else {
    Ok(conclusions)
  }
}
/*
  def solve(
    sanityCheck: Boolean,
    useOptimizedSolver: Boolean,
    interner: Interner,
    callRange: List[RangeS],
    rules: IndexedSeq[IRulexSR],
    identifyingRunes: Iterable[IRuneS]):
  Result[Map[IRuneS, Boolean], IdentifiabilitySolveError] = {
    val initiallyKnownRunes = identifyingRunes.map(r => (r, true)).toMap
    val solverState =
      Solver.makeSolverState(
        sanityCheck,
        useOptimizedSolver,
        (rule: IRulexSR) => getPuzzles(rule),
        getRunes,
        rules,
        initiallyKnownRunes,
        (rules.flatMap(getRunes) ++ initiallyKnownRunes.keys).distinct.toVector)
    while ( {
      solverState.sanityCheck()
      solverState.getNextSolvable() match {
        case None => false // break
        case Some(solvingRuleIndex) => {
          val rule = solverState.getRule(solvingRuleIndex)
          val stepsBefore = solverState.getSteps().size
          solveRule(solverState, solvingRuleIndex, rule) match {
            case Ok(()) => {}
            case Err(e) => return Err(IdentifiabilitySolveError(callRange, FailedSolve(solverState.getSteps(), solverState.getConclusions().toMap, solverState.getUnsolvedRules(), solverState.getUnsolvedRunes(), e)))
          }
          val stepsAfter = solverState.getSteps().size
          vassert(stepsAfter == stepsBefore + 1)
          vassert(solverState.ruleIsSolved(solvingRuleIndex))
          solverState.sanityCheck()
          // Go back to the beginning. Next step, if there's no simple rule ready to solve, then
          // it'll start doing a complex solve if available, or just finish.
          true
        }
      }
    }) {}
    // If we get here, then there's nothing more the solver can do.

    val steps = solverState.getSteps().toStream
    val conclusions = solverState.userifyConclusions().toMap

    val allRunes = solverState.getAllRunes()
    val unsolvedRunes = allRunes -- conclusions.keySet
    if (unsolvedRunes.nonEmpty) {
      Err(
        IdentifiabilitySolveError(
          callRange,
          FailedSolve(
            steps,
            conclusions,
            solverState.getUnsolvedRules(),
            unsolvedRunes.toVector,
            SolveIncomplete())))
    } else {
      Ok(conclusions)
    }
  }
}
*/