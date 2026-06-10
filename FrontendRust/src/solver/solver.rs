/*
package dev.vale.solver

import dev.vale.{Err, Interner, Ok, Profiler, RangeS, Result, vassert, vfail, vimpl, vpass}

import scala.collection.immutable.Map
import scala.collection.mutable

*/
use std::marker::PhantomData;

use super::simple_solver_state::SimpleSolverState;
use std::collections::HashMap;
use std::collections::HashSet;
use std::convert::Infallible;
use std::hash::Hash;

// mig: struct Step
#[derive(Clone, Debug, PartialEq)]
pub struct Step<Rule, Rune, Conclusion>
where
    Rune: Eq + Hash,
{
    pub complex: bool,
    pub solved_rules: Vec<(i32, Rule)>,
    pub added_rules: Vec<Rule>,
    pub conclusions: HashMap<Rune, Conclusion>,
}
// mig: impl Step
impl<Rule, Rune, Conclusion> Step<Rule, Rune, Conclusion>
where
    Rune: Eq + Hash,
{
}
/*
case class Step[Rule, Rune, Conclusion](complex: Boolean, solvedRules: Vector[(Int, Rule)], addedRules: Vector[Rule], conclusions: Map[Rune, Conclusion])


case class FailedSolve[Rule, Rune, Conclusion, ErrType](
  steps: Stream[Step[Rule, Rune, Conclusion]],
  conclusions: Map[Rune, Conclusion],
  unsolvedRules: Vector[Rule],
  unsolvedRunes: Vector[Rune],
  error: ISolverError[Rune, Conclusion, ErrType]
)

*/

// mig: struct FailedSolve
#[derive(Clone, Debug, PartialEq)]
pub struct FailedSolve<Rule, Rune, Conclusion, ErrType>
where
    Rune: Eq + Hash,
{
    pub steps: Vec<Step<Rule, Rune, Conclusion>>,
    pub conclusions: HashMap<Rune, Conclusion>,
    pub unsolved_rules: Vec<Rule>,
    pub unsolved_runes: Vec<Rune>,
    pub error: ISolverError<Rune, Conclusion, ErrType>,
}
// mig: impl FailedSolve
impl<Rule, Rune, Conclusion, ErrType> FailedSolve<Rule, Rune, Conclusion, ErrType>
where
    Rune: Eq + Hash,
{
}

// mig: trait ISolverError
#[derive(Clone, Debug, PartialEq)]
pub enum ISolverError<Rune, Conclusion, ErrType> {
  SolverConflict(SolverConflict<Rune, Conclusion, ErrType>),
  RuleError(RuleError<Rune, Conclusion, ErrType>),
  SolveIncomplete(SolveIncomplete<Rune, Conclusion, ErrType>),
}
/*
sealed trait ISolverError[Rune, Conclusion, ErrType]
case class SolveIncomplete[Rune, Conclusion, ErrType]() extends ISolverError[Rune, Conclusion, ErrType]
*/
// mig: struct SolveIncomplete
#[derive(Clone, Debug, PartialEq)]
pub struct SolveIncomplete<Rune, Conclusion, ErrType> {
    pub _phantom: PhantomData<(Rune, Conclusion, ErrType)>,
}
// mig: struct SolverConflict
#[derive(Clone, Debug, PartialEq)]
pub struct SolverConflict<Rune, Conclusion, ErrType> {
    pub rune: Rune,
    pub previous_conclusion: Conclusion,
    pub new_conclusion: Conclusion,
    pub _phantom: PhantomData<ErrType>,
}
// mig: impl SolverConflict
impl<Rune, Conclusion, ErrType> SolverConflict<Rune, Conclusion, ErrType> {}
/*
case class SolverConflict[Rune, Conclusion, ErrType](
  rune: Rune,
  previousConclusion: Conclusion,
  newConclusion: Conclusion
) extends ISolverError[Rune, Conclusion, ErrType]
*/
// mig: struct RuleError
#[derive(Clone, Debug, PartialEq)]
pub struct RuleError<Rune, Conclusion, ErrType> {
    pub err: ErrType,
    pub _phantom: PhantomData<(Rune, Conclusion)>,
}
// mig: impl RuleError
impl<Rune, Conclusion, ErrType> RuleError<Rune, Conclusion, ErrType> {}
/*
case class RuleError[Rune, Conclusion, ErrType](
//  ruleIndex: Int,
  err: ErrType
) extends ISolverError[Rune, Conclusion, ErrType]

// Given enough user specified template params and param inputs, we should be able to
// infer everything.
// This class's purpose is to take those things, and see if it can figure out as many
// inferences as possible.

//trait ISolveRule[Rule, Rune, Env, State, Conclusion, ErrType] {
//  def solve(
//    state: State,
//    env: Env,
//    solverState: SimpleSolverState[Rule, Rune, Conclusion],
//    ruleIndex: Int,
//    rule: Rule):
//  Result[Unit, ISolverError[Rune, Conclusion, ErrType]]
//
//  // Called when we can't do any regular solves, we don't have enough
//  // runes. This is where we do more interesting rules, like SMCMST.
//  // See CSALR for more.
//  def complexSolve(
//    state: State,
//    env: Env,
//    solverState: SimpleSolverState[Rule, Rune, Conclusion]
//  ): Result[Unit, ISolverError[Rune, Conclusion, ErrType]]
//
//  def sanityCheckConclusion(env: Env, state: State, rune: Rune, conclusion: Conclusion): Unit
//}

*/
/*
object Solver {
  def makeSolverState[Rule, Rune, Conclusion](
      sanityCheck: Boolean,
      useOptimizedSolver: Boolean,
      ruleToPuzzles: Rule => Vector[Vector[Rune]],
      ruleToRunes: Rule => Iterable[Rune],
      initialRules: IndexedSeq[Rule],
      initiallyKnownRunes: Map[Rune, Conclusion],
      allRunes: Vector[Rune]
  ): SimpleSolverState[Rule, Rune, Conclusion] = {
*/
// mig: fn make_solver_state (matches Scala's Solver.makeSolverState)
pub fn make_solver_state<Rule, Rune, Conclusion>(
    sanity_check: bool,
    _use_optimized_solver: bool,
    rule_to_puzzles: Box<dyn Fn(&Rule) -> Vec<Vec<Rune>>>,
    rule_to_runes: &dyn Fn(&Rule) -> Vec<Rune>,
    initial_rules: Vec<Rule>,
    initially_known_runes: HashMap<Rune, Conclusion>,
    all_runes: Vec<Rune>,
) -> SimpleSolverState<Rule, Rune, Conclusion>
where
    Rule: Clone,
    Rune: Clone + Hash + Eq,
    Conclusion: Clone + PartialEq,
{
    let mut solver_state = SimpleSolverState::new(rule_to_puzzles, all_runes.clone());

    if sanity_check {
        for rule in &initial_rules {
            for rune in rule_to_runes(rule) {
                assert!(all_runes.contains(&rune), "vassert: rune from rule must be in allRunes");
            }
        }
        for rune in initially_known_runes.keys() {
            assert!(all_runes.contains(rune), "vassert: known rune must be in allRunes");
        }
    }

    if sanity_check {
        solver_state.sanity_check();
    }

    // Matches Scala: solverState.commitStep(false, Vector(), initiallyKnownRunes, initialRules.toVector).getOrDie()
    match solver_state.commit_step::<Infallible>(
        false,
        vec![],
        initially_known_runes,
        initial_rules,
        HashSet::new(),
    ) {
        Ok(()) => {},
        Err(_) => panic!("Initial commitStep should not fail"),
    }

    if sanity_check {
        solver_state.sanity_check();
    }

    solver_state
}
/*
    val solverState =
      if (useOptimizedSolver) {
        SimpleSolverState[Rule, Rune, Conclusion](ruleToPuzzles, allRunes)
        // One day, after Rust migration: OptimizedSolverState[Rule, Rune, Conclusion](ruleToPuzzles)
      } else {
        SimpleSolverState[Rule, Rune, Conclusion](ruleToPuzzles, allRunes)
      }

    if (sanityCheck) {
      initialRules.flatMap(ruleToRunes).foreach(rune => vassert(allRunes.contains(rune)))
      initiallyKnownRunes.keys.foreach(rune => vassert(allRunes.contains(rune)))
      vassert(allRunes == allRunes.distinct)
    }

    if (sanityCheck) {
      solverState.sanityCheck()
    }

    solverState.commitStep(false, Vector(), initiallyKnownRunes, initialRules.toVector).getOrDie()

    if (sanityCheck) {
      solverState.sanityCheck()
    }
    solverState
  }
}
*/
