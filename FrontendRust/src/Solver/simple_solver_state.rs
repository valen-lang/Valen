/*
package dev.vale.solver

import dev.vale.{Err, Ok, RangeS, Result, vassert, vassertSome, vcurious, vfail}

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer


object SimpleSolverState {
*/
// mig: fn apply
fn apply<Rule, Rune, Conclusion>() -> SimpleSolverState<Rule, Rune, Conclusion> {
    panic!("Unimplemented: apply");
}
/*
  def apply[Rule, Rune, Conclusion](): SimpleSolverState[Rule, Rune, Conclusion] = {
    SimpleSolverState[Rule, Rune, Conclusion](
      Vector(),
      Map[Rune, Int](),
      Map[Int, Rune](),
      Vector[Rule](),
      Map[Int, Vector[Vector[Int]]](),
      Map[Int, Conclusion]())
  }
}

*/
// mig: struct SimpleSolverState
pub struct SimpleSolverState<Rule, Rune, Conclusion> {
    steps: Vec<super::Step<Rule, Rune, Conclusion>>,
    user_rune_to_canonical_rune: std::collections::HashMap<Rune, i32>,
    canonical_rune_to_user_rune: std::collections::HashMap<i32, Rune>,
    rules: Vec<Rule>,
    open_rule_to_puzzle_to_runes: std::collections::HashMap<i32, Vec<Vec<i32>>>,
    canonical_rune_to_conclusion: std::collections::HashMap<i32, Conclusion>,
}
// mig: impl SimpleSolverState
impl<Rule, Rune, Conclusion> SimpleSolverState<Rule, Rune, Conclusion> {
/*
case class SimpleSolverState[Rule, Rune, Conclusion](
  private var steps: Vector[Step[Rule, Rune, Conclusion]],

  private var userRuneToCanonicalRune: Map[Rune, Int],
  private var canonicalRuneToUserRune: Map[Int, Rune],

  private var rules: Vector[Rule],

  private var openRuleToPuzzleToRunes: Map[Int, Vector[Vector[Int]]],

  private var canonicalRuneToConclusion: Map[Int, Conclusion]
) extends ISolverState[Rule, Rune, Conclusion] {

*/
// mig: fn equals
    fn equals(&self, _obj: &dyn std::any::Any) -> bool {
        panic!("Unimplemented: equals");
    }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
    fn hash_code(&self) -> i32 {
        panic!("Unimplemented: hash_code");
    }
/*
  override def hashCode(): Int = vfail() // is mutable, should never be hashed

*/
// mig: fn deep_clone
    fn deep_clone(&self) -> SimpleSolverState<Rule, Rune, Conclusion> {
        panic!("Unimplemented: deep_clone");
    }
/*
  def deepClone(): SimpleSolverState[Rule, Rune, Conclusion] = {
    vcurious()
    SimpleSolverState(
      steps,
      userRuneToCanonicalRune,
      canonicalRuneToUserRune,
      rules,
      openRuleToPuzzleToRunes,
      canonicalRuneToConclusion)
  }

*/
// mig: fn sanity_check
    fn sanity_check(&self) {
        panic!("Unimplemented: sanity_check");
    }
/*
  override def sanityCheck(): Unit = {
//    vassert(rules == rules.distinct)
  }

*/
// mig: fn get_rule
    fn get_rule(&self, _rule_index: i32) -> Rule {
        panic!("Unimplemented: get_rule");
    }
/*
  override def getRule(ruleIndex: Int): Rule = rules(ruleIndex)

*/
// mig: fn get_conclusion
    fn get_conclusion(&self, _rune: Rune) -> Option<Conclusion> {
        panic!("Unimplemented: get_conclusion");
    }
/*
  override def getConclusion(rune: Rune): Option[Conclusion] = canonicalRuneToConclusion.get(getCanonicalRune(rune))

*/
// mig: fn get_user_rune
    fn get_user_rune(&self, _rune: i32) -> Rune {
        panic!("Unimplemented: get_user_rune");
    }
/*
  override def getUserRune(rune: Int): Rune = canonicalRuneToUserRune(rune)

*/
// mig: fn get_conclusions
    fn get_conclusions(&self) -> Vec<(i32, Conclusion)> {
        panic!("Unimplemented: get_conclusions");
    }
/*
  override def getConclusions(): Stream[(Int, Conclusion)] = {
    canonicalRuneToConclusion.toStream
  }

*/
// mig: fn userify_conclusions
    fn userify_conclusions(&self) -> Vec<(Rune, Conclusion)> {
        panic!("Unimplemented: userify_conclusions");
    }
/*
  override def userifyConclusions(): Stream[(Rune, Conclusion)] = {
    canonicalRuneToConclusion
      .toStream
      .map({ case (canonicalRune, conclusion) => (canonicalRuneToUserRune(canonicalRune), conclusion) })
  }

*/
// mig: fn get_all_runes
    fn get_all_runes(&self) -> std::collections::HashSet<i32> {
        panic!("Unimplemented: get_all_runes");
    }
/*
  override def getAllRunes(): Set[Int] = {
    openRuleToPuzzleToRunes.values.flatten.flatten.toSet
  }

*/
// mig: fn add_rune
    fn add_rune(&mut self, _rune: Rune) -> i32 {
        panic!("Unimplemented: add_rune");
    }
/*
  override def addRune(rune: Rune): Int = {
    vassert(!userRuneToCanonicalRune.contains(rune))
    val newCanonicalRune = userRuneToCanonicalRune.size
    userRuneToCanonicalRune = userRuneToCanonicalRune + (rune -> newCanonicalRune)
    canonicalRuneToUserRune = canonicalRuneToUserRune + (newCanonicalRune -> rune)
    newCanonicalRune
  }

*/
// mig: fn get_all_rules
    fn get_all_rules(&self) -> Vec<Rule> {
        panic!("Unimplemented: get_all_rules");
    }
/*
  override def getAllRules(): Vector[Rule] = {
    rules
  }

*/
// mig: fn add_rule
    fn add_rule(&mut self, _rule: Rule) -> i32 {
        panic!("Unimplemented: add_rule");
    }
/*
  override def addRule(rule: Rule): Int = {
    val newCanonicalRule = rules.size
    rules = rules :+ rule
//    canonicalRuleToUserRule = canonicalRuleToUserRule + (newCanonicalRule -> rule)
    newCanonicalRule
  }

*/
// mig: fn get_canonical_rune
    fn get_canonical_rune(&self, _rune: Rune) -> i32 {
        panic!("Unimplemented: get_canonical_rune");
    }
/*
  override def getCanonicalRune(rune: Rune): Int = {
    vassertSome(userRuneToCanonicalRune.get(rune))
  }

*/
// mig: fn add_puzzle
    fn add_puzzle(&mut self, _rule_index: i32, _runes: Vec<i32>) {
        panic!("Unimplemented: add_puzzle");
    }
/*
  override def addPuzzle(ruleIndex: Int, runes: Vector[Int]): Unit = {
    val thisRulePuzzleToRunes = openRuleToPuzzleToRunes.getOrElse(ruleIndex, Vector())
    openRuleToPuzzleToRunes = openRuleToPuzzleToRunes + (ruleIndex -> (thisRulePuzzleToRunes :+ runes))
  }

*/
// mig: fn get_next_solvable
    fn get_next_solvable(&self) -> Option<i32> {
        panic!("Unimplemented: get_next_solvable");
    }
/*
  override def getNextSolvable(): Option[Int] = {
    openRuleToPuzzleToRunes
      .filter({ case (_, puzzleToRunes) =>
        puzzleToRunes.exists(runes => {
          runes.forall(rune => canonicalRuneToConclusion.contains(rune))
        })
      })
      // Get rule with lowest ID, keep it deterministic
      .keySet
      .headOption
  }

*/
// mig: fn get_unsolved_rules
    fn get_unsolved_rules(&self) -> Vec<Rule> {
        panic!("Unimplemented: get_unsolved_rules");
    }
/*
  override def getUnsolvedRules(): Vector[Rule] = {
    openRuleToPuzzleToRunes.keySet.toVector.map(rules)
  }

  // Returns whether it's a new conclusion
  def concludeRune[ErrType](newlySolvedRune: Int, newConclusion: Conclusion):
*/
// mig: fn conclude_rune
    fn conclude_rune<ErrType>(
        &mut self,
        _newly_solved_rune: i32,
        _new_conclusion: Conclusion,
    ) -> Result<bool, super::ISolverError<Rune, Conclusion, ErrType>> {
        panic!("Unimplemented: conclude_rune");
    }
/*
  Result[Boolean, ISolverError[Rune, Conclusion, ErrType]] = {
    val isNew =
      canonicalRuneToConclusion.get(newlySolvedRune) match {
        case Some(existingConclusion) => {
          if (existingConclusion != newConclusion) {
            return Err(
              SolverConflict(
                canonicalRuneToUserRune(newlySolvedRune),
                existingConclusion,
                newConclusion))
          }
          false
        }
        case None => true
      }
    canonicalRuneToConclusion = canonicalRuneToConclusion + (newlySolvedRune -> newConclusion)
    Ok(isNew)
  }

  // Success returns number of new conclusions
  override def markRulesSolved[ErrType](ruleIndices: Vector[Int], newConclusions: Map[Int, Conclusion]):
*/
// mig: fn mark_rules_solved
    fn mark_rules_solved<ErrType>(
        &mut self,
        _rule_indices: Vec<i32>,
        _new_conclusions: std::collections::HashMap<i32, Conclusion>,
    ) -> Result<i32, super::ISolverError<Rune, Conclusion, ErrType>> {
        panic!("Unimplemented: mark_rules_solved");
    }
/*
  Result[Int, ISolverError[Rune, Conclusion, ErrType]] = {
    val numNewConclusions =
      newConclusions.map({ case (newlySolvedRune, newConclusion) =>
        concludeRune[ErrType](newlySolvedRune, newConclusion) match {
          case Err(e) => return Err(e)
          case Ok(isNew) => isNew
        }
      }).count(_ == true)

    ruleIndices.foreach(removeRule)

    Ok(numNewConclusions)
  }

  private def removeRule(ruleIndex: Int) = {
*/
// mig: fn remove_rule
    fn remove_rule(&mut self, _rule_index: i32) {
        panic!("Unimplemented: remove_rule");
    }
}
/*
    openRuleToPuzzleToRunes = openRuleToPuzzleToRunes - ruleIndex
  }

  class SimpleStepState(
*/
// mig: struct SimpleStepState
pub struct SimpleStepState<Rule, Rune, Conclusion> {
    _rule_to_puzzles: std::marker::PhantomData<fn(Rule) -> Vec<Vec<Rune>>>,
    _complex: bool,
    _rules: Vec<(i32, Rule)>,
    _phantom_conclusion: std::marker::PhantomData<Conclusion>,
}
// mig: impl SimpleStepState
impl<Rule, Rune, Conclusion> SimpleStepState<Rule, Rune, Conclusion> {
/*
    ruleToPuzzles: Rule => Vector[Vector[Rune]],
    complex: Boolean,
    rules: Vector[(Int, Rule)]
  ) extends IStepState[Rule, Rune, Conclusion] {
    private var alive = true
    private var tentativeStep: Step[Rule, Rune, Conclusion] = Step(complex, rules, Vector(), Map())

    def close(): Step[Rule, Rune, Conclusion] = {
*/
// mig: fn close
    fn close(&mut self) -> super::Step<Rule, Rune, Conclusion> {
        panic!("Unimplemented: close");
    }
/*
      vassert(alive)
      alive = false
      tentativeStep
    }

    override def getConclusion(requestedUserRune: Rune): Option[Conclusion] = {
*/
// mig: fn get_conclusion
    fn get_conclusion(&self, _requested_user_rune: Rune) -> Option<Conclusion> {
        panic!("Unimplemented: get_conclusion");
    }
/*
      vassert(alive)
      SimpleSolverState.this.getConclusion(requestedUserRune)
    }

    override def addRule(rule: Rule): Unit = {
*/
// mig: fn add_rule
    fn add_rule(&mut self, _rule: Rule) {
        panic!("Unimplemented: add_rule");
    }
/*
      vassert(alive)
      val ruleIndex = SimpleSolverState.this.addRule(rule)
      tentativeStep = tentativeStep.copy(addedRules = tentativeStep.addedRules :+ rule)
      ruleToPuzzles(rule).foreach(puzzleUserRunes => {
        val puzzleCanonicalRunes = puzzleUserRunes.map(SimpleSolverState.this.getCanonicalRune)
        SimpleSolverState.this.addPuzzle(ruleIndex, puzzleCanonicalRunes)
      })
    }

    override def getUnsolvedRules(): Vector[Rule] = {
*/
// mig: fn get_unsolved_rules
    fn get_unsolved_rules(&self) -> Vec<Rule> {
        panic!("Unimplemented: get_unsolved_rules");
    }
/*
      vassert(alive)
      SimpleSolverState.this.getUnsolvedRules()
    }

    override def concludeRune[ErrType](rangeS: List[RangeS], newlySolvedUserRune: Rune, conclusion: Conclusion): Unit = {
*/
// mig: fn conclude_rune
    fn conclude_rune<ErrType>(
        &mut self,
        _range_s: Vec<()>,
        _newly_solved_user_rune: Rune,
        _conclusion: Conclusion,
    ) {
        panic!("Unimplemented: conclude_rune");
    }
}
/*
      vassert(alive)
//      val newlySolvedCanonicalRune = SimpleSolverState.this.userRuneToCanonicalRune(newlySolvedUserRune)
      tentativeStep = tentativeStep.copy(conclusions = tentativeStep.conclusions + (newlySolvedUserRune -> conclusion))
//      Ok(true)
    }
  }

  override def initialStep[ErrType](
*/
// mig: fn initial_step
impl<Rule, Rune, Conclusion> SimpleSolverState<Rule, Rune, Conclusion> {
    fn initial_step<ErrType, StS>(
        &mut self,
        _rule_to_puzzles: impl Fn(Rule) -> Vec<Vec<Rune>>,
        _step: impl FnOnce(&mut StS) -> Result<(), super::ISolverError<Rune, Conclusion, ErrType>>,
    ) -> Result<super::Step<Rule, Rune, Conclusion>, super::ISolverError<Rune, Conclusion, ErrType>> {
        panic!("Unimplemented: initial_step");
    }
/*
    ruleToPuzzles: Rule => Vector[Vector[Rune]],
    step: IStepState[Rule, Rune, Conclusion] => Result[Unit, ISolverError[Rune, Conclusion, ErrType]]):
  Result[Step[Rule, Rune, Conclusion], ISolverError[Rune, Conclusion, ErrType]] = {
    val stepState = new SimpleStepState(ruleToPuzzles, false, Vector())
    step(stepState) match {
      case Ok(()) => {
        val step = stepState.close()
        steps = steps :+ step
        Ok(step)
      }
      case Err(e) => {
        stepState.close()
        Err(e)
      }
    }
  }

  override def simpleStep[ErrType](
*/
// mig: fn simple_step
    fn simple_step<ErrType, StS>(
        &mut self,
        _rule_to_puzzles: impl Fn(Rule) -> Vec<Vec<Rune>>,
        _rule_index: i32,
        _rule: Rule,
        _step: impl FnOnce(&mut StS) -> Result<(), super::ISolverError<Rune, Conclusion, ErrType>>,
    ) -> Result<super::Step<Rule, Rune, Conclusion>, super::ISolverError<Rune, Conclusion, ErrType>> {
        panic!("Unimplemented: simple_step");
    }
/*
    ruleToPuzzles: Rule => Vector[Vector[Rune]],
    ruleIndex: Int,
    rule: Rule,
    step: IStepState[Rule, Rune, Conclusion] => Result[Unit, ISolverError[Rune, Conclusion, ErrType]]):
  Result[Step[Rule, Rune, Conclusion], ISolverError[Rune, Conclusion, ErrType]] = {
    val stepState = new SimpleStepState(ruleToPuzzles, false, Vector((ruleIndex, rule)))
    step(stepState) match {
      case Ok(()) => {
        val step = stepState.close()
        steps = steps :+ step
        Ok(step)
      }
      case Err(e) => {
        stepState.close()
        Err(e)
      }
    }
  }

  override def complexStep[ErrType](
*/
// mig: fn complex_step
    fn complex_step<ErrType, StS>(
        &mut self,
        _rule_to_puzzles: impl Fn(Rule) -> Vec<Vec<Rune>>,
        _step: impl FnOnce(&mut StS) -> Result<(), super::ISolverError<Rune, Conclusion, ErrType>>,
    ) -> Result<super::Step<Rule, Rune, Conclusion>, super::ISolverError<Rune, Conclusion, ErrType>> {
        panic!("Unimplemented: complex_step");
    }
/*
    ruleToPuzzles: Rule => Vector[Vector[Rune]],
    step: IStepState[Rule, Rune, Conclusion] => Result[Unit, ISolverError[Rune, Conclusion, ErrType]]):
  Result[Step[Rule, Rune, Conclusion], ISolverError[Rune, Conclusion, ErrType]] = {
    val stepState = new SimpleStepState(ruleToPuzzles, true, Vector())
    step(stepState) match {
      case Ok(()) => {
        val step = stepState.close()
        steps = steps :+ step
        Ok(step)
      }
      case Err(e) => {
        stepState.close()
        Err(e)
      }
    }
  }

*/
// mig: fn get_steps
    fn get_steps(&self) -> Vec<super::Step<Rule, Rune, Conclusion>> {
        panic!("Unimplemented: get_steps");
    }
}
/*
  override def getSteps(): Stream[Step[Rule, Rune, Conclusion]] = steps.toStream
}
*/
