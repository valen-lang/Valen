/*
package dev.vale.solver

import dev.vale.{Err, Interner, Ok, Profiler, RangeS, Result, vassert, vfail, vimpl, vpass}

import scala.collection.immutable.Map
import scala.collection.mutable

*/
use std::marker::PhantomData;

// mig: struct Step
pub struct Step<Rule, Rune, Conclusion> {
    pub complex: bool,
    pub solved_rules: Vec<(i32, Rule)>,
    pub added_rules: Vec<Rule>,
    pub conclusions: std::collections::HashMap<Rune, Conclusion>,
}
// mig: impl Step
impl<Rule, Rune, Conclusion> Step<Rule, Rune, Conclusion> {}
/*
case class Step[Rule, Rune, Conclusion](complex: Boolean, solvedRules: Vector[(Int, Rule)], addedRules: Vector[Rule], conclusions: Map[Rune, Conclusion])


*/
// mig: trait ISolverOutcome
pub trait ISolverOutcome<Rule, Rune, Conclusion, ErrType> {
    fn get_or_die(&self) -> std::collections::HashMap<Rune, Conclusion>;
}
/*
sealed trait ISolverOutcome[Rule, Rune, Conclusion, ErrType] {
  def getOrDie(): Map[Rune, Conclusion]
}
*/
// mig: trait IIncompleteOrFailedSolve
pub trait IIncompleteOrFailedSolve<Rule, Rune, Conclusion, ErrType>: ISolverOutcome<Rule, Rune, Conclusion, ErrType> {
    fn unsolved_rules(&self) -> Vec<Rule>;
    fn unsolved_runes(&self) -> Vec<Rune>;
    fn steps(&self) -> Vec<Step<Rule, Rune, Conclusion>>;
}
/*
sealed trait IIncompleteOrFailedSolve[Rule, Rune, Conclusion, ErrType] extends ISolverOutcome[Rule, Rune, Conclusion, ErrType] {
  def unsolvedRules: Vector[Rule]
  def unsolvedRunes: Vector[Rune]
  def steps: Stream[Step[Rule, Rune, Conclusion]]
}
*/
// mig: struct CompleteSolve
pub struct CompleteSolve<Rule, Rune, Conclusion, ErrType> {
    pub steps: Vec<Step<Rule, Rune, Conclusion>>,
    pub conclusions: std::collections::HashMap<Rune, Conclusion>,
    pub _phantom: PhantomData<ErrType>,
}
// mig: impl CompleteSolve
impl<Rule, Rune, Conclusion, ErrType> CompleteSolve<Rule, Rune, Conclusion, ErrType> {}
/*
case class CompleteSolve[Rule, Rune, Conclusion, ErrType](
  steps: Stream[Step[Rule, Rune, Conclusion]],
  conclusions: Map[Rune, Conclusion]
) extends ISolverOutcome[Rule, Rune, Conclusion, ErrType] {
  override def getOrDie(): Map[Rune, Conclusion] = conclusions
}
*/
// mig: struct IncompleteSolve
pub struct IncompleteSolve<Rule, Rune, Conclusion, ErrType> {
    pub steps: Vec<Step<Rule, Rune, Conclusion>>,
    pub unsolved_rules: Vec<Rule>,
    pub unknown_runes: std::collections::HashSet<Rune>,
    pub incomplete_conclusions: std::collections::HashMap<Rune, Conclusion>,
    pub _phantom: PhantomData<ErrType>,
}
// mig: impl IncompleteSolve
impl<Rule, Rune, Conclusion, ErrType> IncompleteSolve<Rule, Rune, Conclusion, ErrType> {}
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
*/
// mig: struct FailedSolve
pub struct FailedSolve<Rule, Rune, Conclusion, ErrType> {
    pub steps: Vec<Step<Rule, Rune, Conclusion>>,
    pub unsolved_rules: Vec<Rule>,
    pub error: ISolverError<Rune, Conclusion, ErrType>,
}
// mig: impl FailedSolve
impl<Rule, Rune, Conclusion, ErrType> FailedSolve<Rule, Rune, Conclusion, ErrType> {}
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
*/
// mig: struct SolverConflict
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
*/
// mig: struct RuleError
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
*/
// mig: trait ISolverError
pub enum ISolverError<Rune, Conclusion, ErrType> {
    SolverConflict(SolverConflict<Rune, Conclusion, ErrType>),
    RuleError(RuleError<Rune, Conclusion, ErrType>),
}
/*
sealed trait ISolverError[Rune, Conclusion, ErrType]
*/
// mig: trait ISolveRule
pub trait ISolveRule<Rule, Rune, Env, State, Conclusion, ErrType> {
    fn solve<SS, StS>(
        &self,
        _state: &State,
        _env: &Env,
        _solver_state: &SS,
        _rule_index: i32,
        _rule: &Rule,
        _step_state: &StS,
    ) -> Result<(), ISolverError<Rune, Conclusion, ErrType>> {
        panic!("Unimplemented: solve");
    }
    fn complex_solve<SS, StS>(
        &self,
        _state: &State,
        _env: &Env,
        _solver_state: &SS,
        _step_state: &StS,
    ) -> Result<(), ISolverError<Rune, Conclusion, ErrType>> {
        panic!("Unimplemented: complex_solve");
    }
    fn sanity_check_conclusion(&self, _env: &Env, _state: &State, _rune: &Rune, _conclusion: &Conclusion) {
        panic!("Unimplemented: sanity_check_conclusion");
    }
}
/*
// Given enough user specified template params and param inputs, we should be able to
// infer everything.
// This class's purpose is to take those things, and see if it can figure out as many
// inferences as possible.

trait ISolveRule[Rule, Rune, Env, State, Conclusion, ErrType] {
*/
// mig: fn solve
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
/*
  def sanityCheckConclusion(env: Env, state: State, rune: Rune, conclusion: Conclusion): Unit
}

*/
// mig: struct Solver
pub struct Solver<Rule, Rune, Env, State, Conclusion, ErrType> {
    _sanity_check: bool,
    _use_optimized_solver: bool,
    _interner: (),
    _rule_to_puzzles: (),
    _rule_to_runes: (),
    _solve_rule: (),
    _setup_range: (),
    _initial_rules: (),
    _initially_known_runes: (),
    _all_runes: (),
    _phantom: PhantomData<(Rule, Rune, Env, State, Conclusion, ErrType)>,
}
// mig: impl Solver
impl<Rule, Rune, Env, State, Conclusion, ErrType> Solver<Rule, Rune, Env, State, Conclusion, ErrType> {
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
        panic!("Unimplemented: get_all_rules");
    }
/*
  def getAllRules(): Vector[Rule] = {
    solverState.getAllRules()
  }

*/
    // mig: fn add_rules
    pub fn add_rules(&mut self, _rules: Vec<Rule>) {
        panic!("Unimplemented: add_rules");
    }
/*
  def addRules(rules: Vector[Rule]): Unit = {
    rules.foreach(rule => addRule(rule))
  }

*/
    // mig: fn add_rule
    pub fn add_rule(&mut self, _rule: Rule) {
        panic!("Unimplemented: add_rule");
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
        panic!("Unimplemented: userify_conclusions");
    }
/*
  def userifyConclusions(): Stream[(Rune, Conclusion)] = {
    solverState.userifyConclusions()
  }

*/
    // mig: fn get_conclusion
    pub fn get_conclusion(&self, _rune: &Rune) -> Option<Conclusion> {
        panic!("Unimplemented: get_conclusion");
    }
/*
  def getConclusion(rune: Rune): Option[Conclusion] = {
    solverState.getConclusion(rune)
  }

*/
    // mig: fn is_complete
    pub fn is_complete(&self) -> bool {
        panic!("Unimplemented: is_complete");
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
        _rule_indices: Vec<i32>,
        _new_conclusions: std::collections::HashMap<i32, Conclusion>,
    ) -> Result<i32, ISolverError<Rune, Conclusion, ErrType>> {
        panic!("Unimplemented: mark_rules_solved");
    }
/*
  def markRulesSolved[ErrType](ruleIndices: Vector[Int], newConclusions: Map[Int, Conclusion]):
  Result[Int, ISolverError[Rune, Conclusion, ErrType]] = {
    solverState.markRulesSolved(ruleIndices, newConclusions)
  }

*/
    // mig: fn get_canonical_rune
    pub fn get_canonical_rune(&self, _rune: &Rune) -> i32 {
        panic!("Unimplemented: get_canonical_rune");
    }
/*
  def getCanonicalRune(rune: Rune): Int = {
    solverState.getCanonicalRune(rune)
  }

*/
    // mig: fn get_steps
    pub fn get_steps(&self) -> Vec<Step<Rule, Rune, Conclusion>> {
        panic!("Unimplemented: get_steps");
    }
/*
  def getSteps(): Stream[Step[Rule, Rune, Conclusion]] = {
    solverState.getSteps()
  }

*/
    // mig: fn get_all_runes
    pub fn get_all_runes(&self) -> std::collections::HashSet<i32> {
        panic!("Unimplemented: get_all_runes");
    }
/*
  def getAllRunes(): Set[Int] = {
    solverState.getAllRunes()
  }

*/
    // mig: fn get_user_rune
    pub fn get_user_rune(&self, _rune: i32) -> Rune {
        panic!("Unimplemented: get_user_rune");
    }
/*
  def getUserRune(rune: Int): Rune = {
    solverState.getUserRune(rune)
  }

*/
    // mig: fn get_unsolved_rules
    pub fn get_unsolved_rules(&self) -> Vec<Rule> {
        panic!("Unimplemented: get_unsolved_rules");
    }
/*
  def getUnsolvedRules(): Vector[Rule] = {
    solverState.getUnsolvedRules()
  }

*/
    // mig: fn advance
    pub fn advance(
        &mut self,
        _env: &Env,
        _state: &State,
    ) -> Result<bool, FailedSolve<Rule, Rune, Conclusion, ErrType>> {
        panic!("Unimplemented: advance");
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
