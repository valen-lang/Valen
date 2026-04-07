/* Guardian: disable-all */
/*
package dev.vale.solver

import dev.vale.{Err, RangeS, Result}

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer

*/
// AFTERM: definitely needs more comments
// AFTERM: lets send a bunch of haikus crawling around figuring out better names for all these things
// mig: trait ISolverState
pub trait ISolverState<Rule, Rune, Conclusion>
where
    Rune: Eq + std::hash::Hash,
{
  /*
  // MIGALLOW: IStepState merged into ISolverState
trait IStepState[Rule, Rune, Conclusion] {

  def getConclusion(rune: Rune): Option[Conclusion]
*/
  // mig: fn step_add_rule
  // from IStepState.addRule, takes puzzles explicitly
  fn step_add_rule(&mut self, rule: Rule, puzzles: Vec<Vec<Rune>>);

/*
  def addRule(rule: Rule): Unit
//  def addPuzzle(ruleIndex: Int, runes: Vector[Rune])
  def getUnsolvedRules(): Vector[Rule]
  */

// mig: fn step_conclude_rune (from IStepState.concludeRune, commits immediately)
fn step_conclude_rune<ErrType>(
  &mut self,
  range_s: Vec<crate::utils::range::RangeS<'_>>,
  rune: Rune,
  conclusion: Conclusion,
) -> Result<(), super::ISolverError<Rune, Conclusion, ErrType>>;

  /*
  def concludeRune[ErrType](rangeS: List[RangeS], newlySolvedRune: Rune, conclusion: Conclusion): Unit
}
trait ISolverState[Rule, Rune, Conclusion] {
// MIGALLOW: IStepState merged into ISolverState
*/
// mig: fn deep_clone
    fn deep_clone(&self) -> Self
    where
        Self: Sized;
/*
  def deepClone(): ISolverState[Rule, Rune, Conclusion]
*/
// mig: fn get_canonical_rune
    fn get_canonical_rune(&self, rune: Rune) -> i32;
/*
  def getCanonicalRune(rune: Rune): Int
*/
// mig: fn get_user_rune
    fn get_user_rune(&self, rune: i32) -> Rune;
/*
  def getUserRune(rune: Int): Rune
*/
// mig: fn get_rule
    fn get_rule(&self, rule_index: i32) -> &Rule;
/*
  def getRule(ruleIndex: Int): Rule
*/
// mig: fn get_conclusion
    fn get_conclusion(&self, rune: Rune) -> Option<Conclusion>;
/*
  def getConclusion(rune: Rune): Option[Conclusion]
*/
// mig: fn get_conclusions
    fn get_conclusions(&self) -> Vec<(i32, Conclusion)>;
/*
  def getConclusions(): Stream[(Int, Conclusion)]
*/
// mig: fn userify_conclusions
    fn userify_conclusions(&self) -> Vec<(Rune, Conclusion)>;
/*
  def userifyConclusions(): Stream[(Rune, Conclusion)]
*/
// mig: fn get_unsolved_rules
    fn get_unsolved_rules(&self) -> Vec<Rule>;
/*
  def getUnsolvedRules(): Vector[Rule]
*/
// mig: fn get_next_solvable
    fn get_next_solvable(&self) -> Option<i32>;
/*
  def getNextSolvable(): Option[Int]
*/
// mig: fn get_steps
    fn get_steps(&self) -> Vec<super::Step<Rule, Rune, Conclusion>>;
/*
  def getSteps(): Stream[Step[Rule, Rune, Conclusion]]
*/
// mig: fn add_rule
    fn add_rule(&mut self, rule: Rule) -> i32;
/*
  def addRule(rule: Rule): Int
*/
// mig: fn add_rune
    fn add_rune(&mut self, rune: Rune) -> i32;
/*
  def addRune(rune: Rune): Int
*/
// mig: fn get_all_runes
    fn get_all_runes(&self) -> std::collections::HashSet<i32>;
/*
  def getAllRunes(): Set[Int]
*/
// mig: fn get_all_rules
    fn get_all_rules(&self) -> Vec<Rule>;
/*
  def getAllRules(): Vector[Rule]
*/
// mig: fn add_puzzle
    fn add_puzzle(&mut self, rule_index: i32, runes: Vec<i32>);
/*
  def addPuzzle(ruleIndex: Int, runes: Vector[Int]): Unit
*/
// mig: fn sanity_check
    fn sanity_check(&self);
/*
  def sanityCheck(): Unit
*/
/*
  // Success returns number of new conclusions
  def markRulesSolved[ErrType](ruleIndices: Vector[Int], newConclusions: Map[Int, Conclusion]):
  Result[Int, ISolverError[Rune, Conclusion, ErrType]]
*/
// mig: fn mark_rules_solved
    fn mark_rules_solved<ErrType>(
        &mut self,
        rule_indices: Vec<i32>,
        new_conclusions: std::collections::HashMap<i32, Conclusion>,
    ) -> Result<i32, super::ISolverError<Rune, Conclusion, ErrType>>;
/*
  def initialStep[ErrType](
    ruleToPuzzles: Rule => Vector[Vector[Rune]],
    step: IStepState[Rule, Rune, Conclusion] => Result[Unit, ISolverError[Rune, Conclusion, ErrType]]):
  Result[Step[Rule, Rune, Conclusion], ISolverError[Rune, Conclusion, ErrType]]
*/
// mig: fn begin_step (replaces initialStep/simpleStep/complexStep)
    fn begin_step(&mut self, complex: bool, solved_rules: Vec<(i32, Rule)>);
/*
  def simpleStep[ErrType](
    ruleToPuzzles: Rule => Vector[Vector[Rune]],
    ruleIndex: Int,
    rule: Rule,
    step: IStepState[Rule, Rune, Conclusion] => Result[Unit, ISolverError[Rune, Conclusion, ErrType]]):
  Result[Step[Rule, Rune, Conclusion], ISolverError[Rune, Conclusion, ErrType]]
*/
// mig: fn end_step
    fn end_step(
        &mut self,
        rule_indices_to_remove: Vec<i32>,
    ) -> (super::Step<Rule, Rune, Conclusion>, i32);
/*
  def complexStep[ErrType](
    ruleToPuzzles: Rule => Vector[Vector[Rune]],
    step: IStepState[Rule, Rune, Conclusion] => Result[Unit, ISolverError[Rune, Conclusion, ErrType]]):
  Result[Step[Rule, Rune, Conclusion], ISolverError[Rune, Conclusion, ErrType]]
*/
    // mig: fn conclude_rune
    fn conclude_rune<ErrType>(
        &mut self,
        newly_solved_rune: i32,
        conclusion: Conclusion,
    ) -> Result<bool, super::ISolverError<Rune, Conclusion, ErrType>>;
/*
  def concludeRune[ErrType](newlySolvedRune: Int, conclusion: Conclusion):
  Result[Boolean, ISolverError[Rune, Conclusion, ErrType]]
}
*/
    fn new() -> Self where Self: Sized;
}
