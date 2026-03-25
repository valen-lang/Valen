/*
Guardian: disable-all
*/

/*
package dev.vale.solver

import dev.vale.{Err, Interner, Ok, Profiler, RangeS, Result, vassert, vfail, vimpl, vpass}

import scala.collection.immutable.Map
import scala.collection.mutable

*/
use std::marker::PhantomData;

use super::i_solver_state::ISolverState;
use super::simple_solver_state::SimpleSolverState;
use crate::utils::range::RangeS as RangeSTy;

// mig: struct Step
#[derive(Clone, Debug, PartialEq)]
pub struct Step<Rule, Rune, Conclusion>
where
    Rune: Eq + std::hash::Hash,
{
    pub complex: bool,
    pub solved_rules: Vec<(i32, Rule)>,
    pub added_rules: Vec<Rule>,
    pub conclusions: std::collections::HashMap<Rune, Conclusion>,
}
// mig: impl Step
impl<Rule, Rune, Conclusion> Step<Rule, Rune, Conclusion>
where
    Rune: Eq + std::hash::Hash,
{
}
/*
case class Step[Rule, Rune, Conclusion](complex: Boolean, solvedRules: Vector[(Int, Rule)], addedRules: Vector[Rule], conclusions: Map[Rune, Conclusion])

Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub enum SolverOutcome<Rule, Rune, Conclusion, ErrType>
where
    Rune: Eq + std::hash::Hash,
{
    Complete(CompleteSolve<Rule, Rune, Conclusion, ErrType>),
    Incomplete(IncompleteSolve<Rule, Rune, Conclusion, ErrType>),
    Failed(FailedSolve<Rule, Rune, Conclusion, ErrType>),
}
impl<Rule, Rune, Conclusion, ErrType> SolverOutcome<Rule, Rune, Conclusion, ErrType>
where
    Rune: Eq + std::hash::Hash,
{
    pub fn get_or_die(&self) -> std::collections::HashMap<Rune, Conclusion>
    where
        Rule: Clone,
        Rune: Clone + std::hash::Hash + Eq,
        Conclusion: Clone,
    {
        match self {
            SolverOutcome::Complete(c) => c.conclusions.clone(),
            SolverOutcome::Incomplete(_) | SolverOutcome::Failed(_) => {
                panic!("get_or_die called on Incomplete or Failed solve")
            }
        }
    }

    pub fn to_incomplete_or_failed(
        self,
    ) -> Option<IncompleteOrFailedSolve<Rule, Rune, Conclusion, ErrType>> {
        match self {
            SolverOutcome::Complete(_) => None,
            SolverOutcome::Incomplete(i) => Some(IncompleteOrFailedSolve::Incomplete(i)),
            SolverOutcome::Failed(f) => Some(IncompleteOrFailedSolve::Failed(f)),
        }
    }
}

/*
sealed trait ISolverOutcome[Rule, Rune, Conclusion, ErrType] {
  def getOrDie(): Map[Rune, Conclusion]
}
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub enum IncompleteOrFailedSolve<Rule, Rune, Conclusion, ErrType>
where
    Rune: Eq + std::hash::Hash,
{
    Incomplete(IncompleteSolve<Rule, Rune, Conclusion, ErrType>),
    Failed(FailedSolve<Rule, Rune, Conclusion, ErrType>),
}
impl<Rule, Rune, Conclusion, ErrType> IncompleteOrFailedSolve<Rule, Rune, Conclusion, ErrType>
where
    Rune: Eq + std::hash::Hash,
{
    pub fn unsolved_rules(&self) -> Vec<Rule>
    where
        Rule: Clone,
    {
        match self {
            IncompleteOrFailedSolve::Incomplete(i) => i.unsolved_rules.clone(),
            IncompleteOrFailedSolve::Failed(f) => f.unsolved_rules.clone(),
        }
    }

    pub fn unsolved_runes(&self) -> Vec<Rune>
    where
        Rune: Clone,
    {
        match self {
            IncompleteOrFailedSolve::Incomplete(i) => i.unknown_runes.iter().cloned().collect(),
            IncompleteOrFailedSolve::Failed(_) => Vec::new(),
        }
    }

    pub fn steps(&self) -> Vec<Step<Rule, Rune, Conclusion>>
    where
        Rule: Clone,
        Rune: Clone,
        Conclusion: Clone,
    {
        match self {
            IncompleteOrFailedSolve::Incomplete(i) => i.steps.clone(),
            IncompleteOrFailedSolve::Failed(f) => f.steps.clone(),
        }
    }

    pub fn get_or_die(&self) -> std::collections::HashMap<Rune, Conclusion>
    where
        Rune: Clone + std::hash::Hash + Eq,
        Conclusion: Clone,
    {
        panic!("get_or_die called on IncompleteOrFailedSolve")
    }
}
/*
sealed trait IIncompleteOrFailedSolve[Rule, Rune, Conclusion, ErrType] extends ISolverOutcome[Rule, Rune, Conclusion, ErrType] {
  def unsolvedRules: Vector[Rule]
  def unsolvedRunes: Vector[Rune]
  def steps: Stream[Step[Rule, Rune, Conclusion]]
}
Guardian: disable: NECX
*/
// mig: struct CompleteSolve
#[derive(Clone, Debug, PartialEq)]
pub struct CompleteSolve<Rule, Rune, Conclusion, ErrType>
where
    Rune: Eq + std::hash::Hash,
{
    pub steps: Vec<Step<Rule, Rune, Conclusion>>,
    pub conclusions: std::collections::HashMap<Rune, Conclusion>,
    pub _phantom: PhantomData<ErrType>,
}
// mig: impl CompleteSolve
impl<Rule, Rune, Conclusion, ErrType> CompleteSolve<Rule, Rune, Conclusion, ErrType>
where
    Rune: Eq + std::hash::Hash,
{
}
/*
case class CompleteSolve[Rule, Rune, Conclusion, ErrType](
  steps: Stream[Step[Rule, Rune, Conclusion]],
  conclusions: Map[Rune, Conclusion]
) extends ISolverOutcome[Rule, Rune, Conclusion, ErrType] {
  override def getOrDie(): Map[Rune, Conclusion] = conclusions
}
Guardian: disable: NECX
*/
// mig: struct IncompleteSolve
#[derive(Clone, Debug, PartialEq)]
pub struct IncompleteSolve<Rule, Rune, Conclusion, ErrType>
where
    Rune: Eq + std::hash::Hash,
{
    pub steps: Vec<Step<Rule, Rune, Conclusion>>,
    pub unsolved_rules: Vec<Rule>,
    pub unknown_runes: std::collections::HashSet<Rune>,
    pub incomplete_conclusions: std::collections::HashMap<Rune, Conclusion>,
    pub _phantom: PhantomData<ErrType>,
}
// mig: impl IncompleteSolve
impl<Rule, Rune, Conclusion, ErrType> IncompleteSolve<Rule, Rune, Conclusion, ErrType>
where
    Rune: Eq + std::hash::Hash,
{
}

/*
case class IncompleteSolve[Rule, Rune, Conclusion, ErrType](
  steps: Stream[Step[Rule, Rune, Conclusion]],
  unsolvedRules: Vector[Rule],
  unknownRunes: Set[Rune],
  incompleteConclusions: Map[Rune, Conclusion]
) extends IIncompleteOrFailedSolve[Rule, Rune, Conclusion, ErrType] {
  vassert(unknownRunes.nonEmpty)
  vpass()
  override def getOrDie(): Map[Rune, Conclusion] = vfail()
  override def unsolvedRunes: Vector[Rune] = unknownRunes.toVector
}
Guardian: disable: NECX
*/
// mig: struct FailedSolve
#[derive(Clone, Debug, PartialEq)]
pub struct FailedSolve<Rule, Rune, Conclusion, ErrType>
where
    Rune: Eq + std::hash::Hash,
{
    pub steps: Vec<Step<Rule, Rune, Conclusion>>,
    pub unsolved_rules: Vec<Rule>,
    pub error: ISolverError<Rune, Conclusion, ErrType>,
}
// mig: impl FailedSolve
impl<Rule, Rune, Conclusion, ErrType> FailedSolve<Rule, Rune, Conclusion, ErrType>
where
    Rune: Eq + std::hash::Hash,
{
}
/*
case class FailedSolve[Rule, Rune, Conclusion, ErrType](
  steps: Stream[Step[Rule, Rune, Conclusion]],
  unsolvedRules: Vector[Rule],
  error: ISolverError[Rune, Conclusion, ErrType]
) extends IIncompleteOrFailedSolve[Rule, Rune, Conclusion, ErrType] {
  override def getOrDie(): Map[Rune, Conclusion] = vfail()
  vpass()
  override def unsolvedRunes: Vector[Rune] = Vector()
}
Guardian: disable: NECX
*/
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
) extends ISolverError[Rune, Conclusion, ErrType] {
  vpass()
}
Guardian: disable: NECX
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
Guardian: disable: NECX
*/
// mig: trait ISolverError
#[derive(Clone, Debug, PartialEq)]
pub enum ISolverError<Rune, Conclusion, ErrType> {
    SolverConflict(SolverConflict<Rune, Conclusion, ErrType>),
    RuleError(RuleError<Rune, Conclusion, ErrType>),
}
/*
sealed trait ISolverError[Rune, Conclusion, ErrType]
Guardian: disable: NECX
*/
pub trait SolverDelegate<Rule, Rune, Env, State, Conclusion, ErrType>
where
    Rune: Eq + std::hash::Hash,
{
  // SPORK
  fn rule_to_puzzles(&self, rule: &Rule) -> Vec<Vec<Rune>>;
  fn rule_to_runes(&self, rule: &Rule) -> Vec<Rune>;
/*
// Given enough user specified template params and param inputs, we should be able to
// infer everything.
// This class's purpose is to take those things, and see if it can figure out as many
// inferences as possible.

// MIGALLOW: ISolveRule -> SolverDelegate
// SPORK
trait ISolveRule[Rule, Rune, Env, State, Conclusion, ErrType] {
*/
// mig: fn solve
fn solve<S: super::ISolverState<Rule, Rune, Conclusion>>(
  &self,
  state: &State,
  env: &Env,
  rule_index: i32,
  rule: &Rule,
  solver_state: &mut S,
) -> Result<(), ISolverError<Rune, Conclusion, ErrType>>;
/*
  def solve(
    state: State,
    env: Env,
    solverState: ISolverState[Rule, Rune, Conclusion],
    ruleIndex: Int,
    rule: Rule,
    stepState: IStepState[Rule, Rune, Conclusion]):
  Result[Unit, ISolverError[Rune, Conclusion, ErrType]]

*/
// mig: fn complex_solve

fn complex_solve<S: super::ISolverState<Rule, Rune, Conclusion>>(
  &self,
  state: &State,
  env: &Env,
  solver_state: &mut S,
) -> Result<(), ISolverError<Rune, Conclusion, ErrType>>;
/*
  // Called when we can't do any regular solves, we don't have enough
  // runes. This is where we do more interesting rules, like SMCMST.
  // See CSALR for more.
  def complexSolve(
    state: State,
    env: Env,
    solverState: ISolverState[Rule, Rune, Conclusion],
    stepState: IStepState[Rule, Rune, Conclusion]
  ): Result[Unit, ISolverError[Rune, Conclusion, ErrType]]

*/
// mig: fn sanity_check_conclusion
fn sanity_check_conclusion(&self, env: &Env, state: &State, rune: &Rune, conclusion: &Conclusion);
/*
  def sanityCheckConclusion(env: Env, state: State, rune: Rune, conclusion: Conclusion): Unit
}
*/
}

// SPORK
type SolverStateImpl<Rule, Rune, Conclusion> = SimpleSolverState<Rule, Rune, Conclusion>;
/*

*/
// mig: struct Solver
pub struct Solver<'a, Rule, Rune, Env, State, Conclusion, ErrType, D>
where
    Rune: Eq + std::hash::Hash,
{
    sanity_check: bool,
    solver_state: SolverStateImpl<Rule, Rune, Conclusion>,
    delegate: D,
    setup_range: Vec<RangeSTy<'a>>,
    all_runes: Vec<Rune>,
    _phantom: PhantomData<(Env, State, ErrType)>,
}
// mig: impl Solver
impl<'a, Rule, Rune, Env, State, Conclusion, ErrType, D> Solver<'a, Rule, Rune, Env, State, Conclusion, ErrType, D>
where
    Rule: Clone,
    Rune: Clone + std::hash::Hash + Eq,
    Conclusion: Clone + PartialEq,
    D: SolverDelegate<Rule, Rune, Env, State, Conclusion, ErrType>,
{
    /*
class Solver[Rule, Rune, Env, State, Conclusion, ErrType](
    sanityCheck: Boolean,
    useOptimizedSolver: Boolean,
    interner: Interner,
    ruleToPuzzles: Rule => Vector[Vector[Rune]],
    ruleToRunes: Rule => Iterable[Rune],
    solveRule: ISolveRule[Rule, Rune, Env, State, Conclusion, ErrType],
    setupRange: List[RangeS],
    initialRules: IndexedSeq[Rule],
    initiallyKnownRunes: Map[Rune, Conclusion],
    allRunes: Vector[Rune]) {
  */
  pub fn new(
    sanity_check: bool,
    delegate: D,
    setup_range: Vec<RangeSTy<'a>>,
    initial_rules: Vec<Rule>,
    initially_known_runes: std::collections::HashMap<Rune, Conclusion>,
    all_runes: Vec<Rune>,
  ) -> Self {
    let mut solver_state = SolverStateImpl::new();
    for rune in &all_runes {
        solver_state.add_rune(rune.clone());
    }
    if sanity_check {
        solver_state.sanity_check();
    }
    // manual_step equivalent: begin_step, step_conclude_rune for each known, end_step
    solver_state.begin_step(false, vec![]);
    for (rune, conclusion) in initially_known_runes {
        let _ = solver_state.step_conclude_rune::<ErrType>(
            setup_range.clone(),
            rune,
            conclusion,
        );
    }
    let _ = solver_state.end_step(vec![]);
    if sanity_check {
        solver_state.sanity_check();
    }
    for rule in initial_rules {
        let rule_index = solver_state.add_rule(rule.clone());
        let puzzles = delegate.rule_to_puzzles(&rule);
        for puzzle in puzzles {
            let canonical_runes: Vec<i32> =
                puzzle.iter().map(|r| solver_state.get_canonical_rune(r.clone())).collect();
            solver_state.add_puzzle(rule_index, canonical_runes);
        }
        if sanity_check {
            solver_state.sanity_check();
        }
    }
    Solver {
        sanity_check,
        solver_state,
        delegate,
        setup_range,
        all_runes,
        _phantom: PhantomData,
    }
  }
  /*
  private val solverState =
    if (useOptimizedSolver) {
      OptimizedSolverState[Rule, Rune, Conclusion]()
    } else {
      SimpleSolverState[Rule, Rune, Conclusion]()
    }

  Profiler.frame(() => {
    if (sanityCheck) {
      initialRules.flatMap(ruleToRunes).foreach(rune => vassert(allRunes.contains(rune)))
      initiallyKnownRunes.keys.foreach(rune => vassert(allRunes.contains(rune)))
      vassert(allRunes == allRunes.distinct)
    }

    allRunes.foreach(solverState.addRune)

    if (sanityCheck) {
      solverState.sanityCheck()
    }

    manualStep(initiallyKnownRunes).getOrDie()

    if (sanityCheck) {
      solverState.sanityCheck()
    }

    addRules(initialRules.toVector)

    if (sanityCheck) {
      solverState.sanityCheck()
    }
    solverState
  })

*/
    // mig: fn get_all_rules
    pub fn get_all_rules(&self) -> Vec<Rule> {
        self.solver_state.get_all_rules()
    }
/*
  def getAllRules(): Vector[Rule] = {
    solverState.getAllRules()
  }

*/
    // mig: fn add_rules
    pub fn add_rules(&mut self, rules: Vec<Rule>) {
        for rule in rules {
            self.add_rule(rule);
        }
    }
/*
  def addRules(rules: Vector[Rule]): Unit = {
    rules.foreach(rule => addRule(rule))
  }

*/
    // mig: fn add_rule
    pub fn add_rule(&mut self, rule: Rule) {
        let rule_index = self.solver_state.add_rule(rule.clone());
        let puzzles = self.delegate.rule_to_puzzles(&rule);
        for puzzle in puzzles {
            let canonical_runes: Vec<i32> = puzzle
                .iter()
                .map(|r| self.solver_state.get_canonical_rune(r.clone()))
                .collect();
            self.solver_state.add_puzzle(rule_index, canonical_runes);
        }
        if self.sanity_check {
            self.solver_state.sanity_check();
        }
    }
/*
  def addRule(rule: Rule): Unit = {
      val ruleIndex = solverState.addRule(rule)
      if (sanityCheck) {
        solverState.sanityCheck()
      }
      ruleToPuzzles(rule).foreach(puzzleRunes => {
        solverState.addPuzzle(ruleIndex, puzzleRunes.map(solverState.getCanonicalRune).distinct)
      })
      if (sanityCheck) {
        solverState.sanityCheck()
      }
  }

*/
    // mig: fn manual_step
    pub fn manual_step(
        &mut self,
        _new_conclusions: std::collections::HashMap<Rune, Conclusion>,
    ) -> Result<(), ISolverError<Rune, Conclusion, std::convert::Infallible>> {
        panic!("Unimplemented: manual_step");
    }
/*
  def manualStep(newConclusions: Map[Rune, Conclusion]):
  Result[Unit, ISolverError[Rune, Conclusion, Nothing]] = {
    solverState.initialStep(ruleToPuzzles, (stepState: IStepState[Rule, Rune, Conclusion]) => {
      newConclusions.foreach({ case (rune, conclusion) =>
        stepState.concludeRune(RangeS.internal(interner, -6434324) :: setupRange, rune, conclusion)
      })
      Ok(())
    }) match {
      case Ok(step) => {
        step.conclusions.foreach({ case (rune, conclusion) =>
          solverState.concludeRune(solverState.getCanonicalRune(rune), conclusion)
        })
        Ok(Unit)
      }
      case Err(e) => Err(e)
    }
  }

*/
    // mig: fn userify_conclusions
    pub fn userify_conclusions(&self) -> Vec<(Rune, Conclusion)> {
        self.solver_state.userify_conclusions()
    }
/*
  def userifyConclusions(): Stream[(Rune, Conclusion)] = {
    solverState.userifyConclusions()
  }

*/
    // mig: fn get_conclusion
    pub fn get_conclusion(&self, rune: &Rune) -> Option<Conclusion> {
        self.solver_state.get_conclusion(rune.clone())
    }
/*
  def getConclusion(rune: Rune): Option[Conclusion] = {
    solverState.getConclusion(rune)
  }

*/
    // mig: fn is_complete
    pub fn is_complete(&self) -> bool {
        self.solver_state.userify_conclusions().len() == self.all_runes.len()
    }
/*
  def isComplete(): Boolean = {
    // TODO(optimize): There has to be a faster way to do this...
    solverState.userifyConclusions().size == allRunes.size
  }

*/
    // mig: fn mark_rules_solved
    pub fn mark_rules_solved(
        &mut self,
        rule_indices: Vec<i32>,
        new_conclusions: std::collections::HashMap<i32, Conclusion>,
    ) -> Result<i32, ISolverError<Rune, Conclusion, ErrType>> {
        self.solver_state
            .mark_rules_solved(rule_indices, new_conclusions)
    }
/*
  def markRulesSolved[ErrType](ruleIndices: Vector[Int], newConclusions: Map[Int, Conclusion]):
  Result[Int, ISolverError[Rune, Conclusion, ErrType]] = {
    solverState.markRulesSolved(ruleIndices, newConclusions)
  }

*/
    // mig: fn get_canonical_rune
    pub fn get_canonical_rune(&self, rune: &Rune) -> i32 {
        self.solver_state.get_canonical_rune(rune.clone())
    }
/*
  def getCanonicalRune(rune: Rune): Int = {
    solverState.getCanonicalRune(rune)
  }

*/
    // mig: fn get_steps
    pub fn get_steps(&self) -> Vec<Step<Rule, Rune, Conclusion>> {
        self.solver_state.get_steps()
    }
/*
  def getSteps(): Stream[Step[Rule, Rune, Conclusion]] = {
    solverState.getSteps()
  }

*/
    // mig: fn get_all_runes
    pub fn get_all_runes(&self) -> std::collections::HashSet<i32> {
        self.solver_state.get_all_runes()
    }
/*
  def getAllRunes(): Set[Int] = {
    solverState.getAllRunes()
  }

*/
    // mig: fn get_user_rune
    pub fn get_user_rune(&self, rune: i32) -> Rune {
        self.solver_state.get_user_rune(rune)
    }
/*
  def getUserRune(rune: Int): Rune = {
    solverState.getUserRune(rune)
  }

*/
    // mig: fn get_unsolved_rules
    pub fn get_unsolved_rules(&self) -> Vec<Rule> {
        self.solver_state.get_unsolved_rules()
    }
/*
  def getUnsolvedRules(): Vector[Rule] = {
    solverState.getUnsolvedRules()
  }

*/
    // mig: fn advance
    pub fn advance(
        &mut self,
        env: &Env,
        state: &State,
    ) -> Result<bool, FailedSolve<Rule, Rune, Conclusion, ErrType>> {
        if self.sanity_check {
            self.solver_state.sanity_check();
            for (rune, conclusion) in self.solver_state.userify_conclusions() {
                self.delegate
                    .sanity_check_conclusion(env, state, &rune, &conclusion);
            }
        }

        // Stage 1: simple solve
        if let Some(rule_index) = self.solver_state.get_next_solvable() {
            let rule = self.solver_state.get_rule(rule_index).clone();
            self.solver_state
                .begin_step(false, vec![(rule_index, rule.clone())]);
            match self.delegate.solve(
                state,
                env,
                rule_index,
                &rule,
                &mut self.solver_state,
            ) {
                Ok(()) => {
                    let (_step, _num_new) = self.solver_state.end_step(vec![rule_index]);
                    if self.sanity_check {
                        self.solver_state.sanity_check();
                    }
                    return Ok(true);
                }
                Err(e) => {
                    return Err(FailedSolve {
                        steps: self.solver_state.get_steps(),
                        unsolved_rules: self.solver_state.get_unsolved_rules(),
                        error: e,
                    });
                }
            }
        }

        // Stage 2: complex solve
        if !self.solver_state.get_unsolved_rules().is_empty() {
            self.solver_state.begin_step(true, vec![]);
            match self
                .delegate
                .complex_solve(state, env, &mut self.solver_state)
            {
                Ok(()) => {
                    let (_step, num_new) = self.solver_state.end_step(vec![]);
                    if self.sanity_check {
                        self.solver_state.sanity_check();
                    }
                    if num_new > 0 {
                        return Ok(true);
                    }
                }
                Err(e) => {
                    return Err(FailedSolve {
                        steps: self.solver_state.get_steps(),
                        unsolved_rules: self.solver_state.get_unsolved_rules(),
                        error: e,
                    });
                }
            }
        }

        // Stage 3: done
        Ok(false)
    }
/*
  // Returns true if there's more to be done, false if we've gotten as far as we can.
  def advance(env: Env, state: State):
  Result[Boolean, FailedSolve[Rule, Rune, Conclusion, ErrType]] = {
    Profiler.frame(() => {

      if (sanityCheck) {
        solverState.sanityCheck()

        solverState.userifyConclusions().foreach({ case (rune, conclusion) =>
          solveRule.sanityCheckConclusion(env, state, rune, conclusion)
        })
      }

      // Stage 1: Do simple solves

      solverState.getNextSolvable() match {
        case None => // continue onto the next stage
        case Some(solvingRuleIndex) => {
          val rule = solverState.getRule(solvingRuleIndex)
          val step =
            solverState.simpleStep[ErrType](ruleToPuzzles, solvingRuleIndex, rule, solveRule.solve(state, env, solverState, solvingRuleIndex, rule, _)) match {
              case Ok(step) => step
              case Err(e) => return Err(FailedSolve(solverState.getSteps(), solverState.getUnsolvedRules(), e))
            }

          val canonicalConclusions =
            step.conclusions.map({ case (userRune, conclusion) => solverState.getCanonicalRune(userRune) -> conclusion }).toMap

          solverState.markRulesSolved[ErrType](Vector(solvingRuleIndex), canonicalConclusions) match {
            case Err(e) => return Err(FailedSolve(solverState.getSteps(), solverState.getUnsolvedRules(), e))
            case Ok(_) =>
          }

          if (sanityCheck) {
            step.conclusions.foreach({ case (rune, conclusion) =>
              solveRule.sanityCheckConclusion(env, state, rune, conclusion)
            })
            solverState.sanityCheck()
          }
          // Go back to the beginning. Next step, if there's no simple rule ready to solve, then
          // it'll start doing a complex solve if available, or just finish.
          return Ok(true)
        }
      }

      // Stage 2: Do a complex solve if available.

      if (solverState.getUnsolvedRules().nonEmpty) {
        val step =
          solverState.complexStep(ruleToPuzzles, solveRule.complexSolve(state, env, solverState, _)) match {
            case Ok(step) => step
            case Err(e) => return Err(FailedSolve(solverState.getSteps(), solverState.getUnsolvedRules(), e))
          }
        val canonicalConclusions =
          step.conclusions.map({ case (userRune, conclusion) => solverState.getCanonicalRune(userRune) -> conclusion }).toMap
        solverState.markRulesSolved[ErrType](step.solvedRules.map(_._1).toVector, canonicalConclusions) match {
          case Ok(0) => {
            if (sanityCheck) {
              solverState.sanityCheck()
            }
            // There's nothing more to be done. Let's continue on to stage 3.
          }
          case Ok(_) => {
            if (sanityCheck) {
              solverState.sanityCheck()
            }
            return Ok(true) // Go back to stage 1
          }
          case Err(e) => return Err(FailedSolve(solverState.getSteps(), solverState.getUnsolvedRules(), e))
        }
      } else {
        // No more rules to solve, so continue to the wrapping up stages of the solve.
      }

      // Stage 3: We're done! The user should look at the conclusions to see if they're all solved,
      // and they can even add more rules if they want.

      return Ok(false)
    })
  }
}
*/
}
