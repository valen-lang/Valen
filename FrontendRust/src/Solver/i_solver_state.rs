/*
package dev.vale.solver

import dev.vale.{Err, Ok, RangeS, Result}

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer

//trait IStepState[Rule, Rune, Conclusion] {
//  def getConclusion(rune: Rune): Option[Conclusion]
//  def addRule(rule: Rule): Unit
////  def addPuzzle(ruleIndex: Int, runes: Vector[Rune])
//  def getUnsolvedRules(): Vector[Rule]
//  def concludeRune[ErrType](rangeS: List[RangeS], newlySolvedRune: Rune, conclusion: Conclusion): Result[Unit, ISolverError[Rune, Conclusion, ErrType]]
//
//  def close(): Unit
//}

//trait ISolverState[Rule, Rune, Conclusion] {
//  def deepClone(): ISolverState[Rule, Rune, Conclusion]
//  def getCanonicalRune(rune: Rune): Int
//  def getUserRune(rune: Int): Rune
//  def getRule(ruleIndex: Int): Rule
//  def getConclusion(rune: Rune): Option[Conclusion]
//  def getConclusions(): Stream[(Int, Conclusion)]
//  def userifyConclusions(): Stream[(Rune, Conclusion)]
//  def getUnsolvedRules(): Vector[Rule]
//  def getNextSolvable(): Option[Int]
//  def getSteps(): Stream[Step[Rule, Rune, Conclusion]]
//
//  def isComplete(): Boolean
//
//  def addRule(rule: Rule): Int
//  def addRune(rune: Rune): Int
//  def removeRule(ruleIndex: Int): Unit
//  def addStep(step: Step[Rule, Rune, Conclusion]): Unit
//
//  def getAllRunes(): Set[Int]
//  def getAllRules(): Vector[Rule]
//
//  def addPuzzle(ruleIndex: Int, runes: Vector[Int]): Unit
//
////  // TODO DO NOT SUBMIT: add a addRuleAndPuzzles which calls addPuzzle, addRule, and this, and get rid of this
////  def getPuzzlesForRule(rule: Rule): Vector[Vector[Rune]]
//
//  def addRuleAndPuzzles(rule: Rule): Unit
//
//  def sanityCheck(): Unit
//
////  // Success returns number of new conclusions
////  def markRulesSolvedZZZ[ErrType](ruleIndices: Vector[Int], newConclusions: Map[Int, Conclusion]):
////  Result[Int, ISolverError[Rune, Conclusion, ErrType]]
//
////  def addStep(step: Step[Rule, Rune, Conclusion]): Unit
//
//
//  def concludeRune[ErrType](newlySolvedRune: Int, conclusion: Conclusion):
//  Result[Boolean, ISolverError[Rune, Conclusion, ErrType]]
//}
*/
// AFTERM: definitely needs more comments
// AFTERM: lets send a bunch of haikus crawling around figuring out better names for all these things
// mig: trait ISolverState
pub trait ISolverState<Rule, Rune, Conclusion>
where
    Rune: Eq + std::hash::Hash,
{
  // mig: fn step_add_rule
  // from IStepState.addRule, takes puzzles explicitly
  fn step_add_rule(&mut self, rule: Rule, puzzles: Vec<Vec<Rune>>);

// mig: fn step_conclude_rune (from IStepState.concludeRune, commits immediately)
fn step_conclude_rune<ErrType>(
  &mut self,
  range_s: Vec<crate::utils::range::RangeS<'_>>,
  rune: Rune,
  conclusion: Conclusion,
) -> Result<(), super::ISolverError<Rune, Conclusion, ErrType>>;

// mig: fn deep_clone
    fn deep_clone(&self) -> Self
    where
        Self: Sized;
// mig: fn get_canonical_rune
    fn get_canonical_rune(&self, rune: Rune) -> i32;
// mig: fn get_user_rune
    fn get_user_rune(&self, rune: i32) -> Rune;
// mig: fn get_rule
    fn get_rule(&self, rule_index: i32) -> &Rule;
// mig: fn get_conclusion
    fn get_conclusion(&self, rune: Rune) -> Option<Conclusion>;
// mig: fn get_conclusions
    fn get_conclusions(&self) -> Vec<(i32, Conclusion)>;
// mig: fn userify_conclusions
    fn userify_conclusions(&self) -> Vec<(Rune, Conclusion)>;
// mig: fn get_unsolved_rules
    fn get_unsolved_rules(&self) -> Vec<Rule>;
// mig: fn get_next_solvable
    fn get_next_solvable(&self) -> Option<i32>;
// mig: fn get_steps
    fn get_steps(&self) -> Vec<super::Step<Rule, Rune, Conclusion>>;
// mig: fn add_rule
    fn add_rule(&mut self, rule: Rule) -> i32;
// mig: fn add_rune
    fn add_rune(&mut self, rune: Rune) -> i32;
// mig: fn get_all_runes
    fn get_all_runes(&self) -> std::collections::HashSet<i32>;
// mig: fn get_all_rules
    fn get_all_rules(&self) -> Vec<Rule>;
// mig: fn add_puzzle
    fn add_puzzle(&mut self, rule_index: i32, runes: Vec<i32>);
// mig: fn sanity_check
    fn sanity_check(&self);
// mig: fn mark_rules_solved
    fn mark_rules_solved<ErrType>(
        &mut self,
        rule_indices: Vec<i32>,
        new_conclusions: std::collections::HashMap<i32, Conclusion>,
    ) -> Result<i32, super::ISolverError<Rune, Conclusion, ErrType>>;
// mig: fn begin_step (replaces initialStep/simpleStep/complexStep)
    fn begin_step(&mut self, complex: bool, solved_rules: Vec<(i32, Rule)>);
// mig: fn end_step
    fn end_step(
        &mut self,
        rule_indices_to_remove: Vec<i32>,
    ) -> (super::Step<Rule, Rune, Conclusion>, i32);
    // mig: fn conclude_rune
    fn conclude_rune<ErrType>(
        &mut self,
        newly_solved_rune: i32,
        conclusion: Conclusion,
    ) -> Result<bool, super::ISolverError<Rune, Conclusion, ErrType>>;
    fn new() -> Self where Self: Sized;
}
