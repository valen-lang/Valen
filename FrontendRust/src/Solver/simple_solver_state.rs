/*
Guardian: disable-all
*/

/*
package dev.vale.solver

import dev.vale.{Err, Ok, RangeS, Result, vassert, vassertSome, vcurious, vfail}

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer


object SimpleSolverState {
*/
// mig: fn apply
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
struct CurrentStep<Rule, Rune, Conclusion>
where
    Rune: Eq + std::hash::Hash,
{
    step: super::Step<Rule, Rune, Conclusion>,
    num_new_conclusions: i32,
}
/*
*/
// mig: struct SimpleSolverState
pub struct SimpleSolverState<Rule, Rune, Conclusion>
where
    Rune: Eq + std::hash::Hash,
{
    steps: Vec<super::Step<Rule, Rune, Conclusion>>,
    user_rune_to_canonical_rune: std::collections::HashMap<Rune, i32>,
    canonical_rune_to_user_rune: std::collections::HashMap<i32, Rune>,
    rules: Vec<Rule>,
    open_rule_to_puzzle_to_runes: std::collections::HashMap<i32, Vec<Vec<i32>>>,
    canonical_rune_to_conclusion: std::collections::HashMap<i32, Conclusion>,
    current_step: Option<CurrentStep<Rule, Rune, Conclusion>>,
}
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
// mig: impl SimpleSolverState
impl<Rule, Rune, Conclusion> SimpleSolverState<Rule, Rune, Conclusion>
where
    Rule: Clone,
    Rune: Clone + std::hash::Hash + Eq,
    Conclusion: Clone + PartialEq,
{
    pub fn new() -> Self {
        SimpleSolverState {
            steps: vec![],
            user_rune_to_canonical_rune: std::collections::HashMap::new(),
            canonical_rune_to_user_rune: std::collections::HashMap::new(),
            rules: vec![],
            open_rule_to_puzzle_to_runes: std::collections::HashMap::new(),
            canonical_rune_to_conclusion: std::collections::HashMap::new(),
            current_step: None,
        }
    }

    // mig: fn remove_rule
    fn remove_rule(&mut self, rule_index: i32) {
        self.open_rule_to_puzzle_to_runes.remove(&rule_index);
    }
}

// mig: impl ISolverState for SimpleSolverState
impl<Rule, Rune, Conclusion> super::ISolverState<Rule, Rune, Conclusion>
    for SimpleSolverState<Rule, Rune, Conclusion>
where
    Rule: Clone,
    Rune: Clone + std::hash::Hash + Eq,
    Conclusion: Clone + PartialEq,
{
/*
    // MIGALLOW: No equals yet until we know it's really necessary
  override def equals(obj: Any): Boolean = vcurious();
  // MIGALLOW: No hashCode yet until we know it's really necessary
  override def hashCode(): Int = vfail() // is mutable, should never be hashed

*/
// mig: fn deep_clone
    fn deep_clone(&self) -> Self
    where
        Self: Sized,
    {
        SimpleSolverState {
            steps: self.steps.clone(),
            user_rune_to_canonical_rune: self.user_rune_to_canonical_rune.clone(),
            canonical_rune_to_user_rune: self.canonical_rune_to_user_rune.clone(),
            rules: self.rules.clone(),
            open_rule_to_puzzle_to_runes: self.open_rule_to_puzzle_to_runes.clone(),
            canonical_rune_to_conclusion: self.canonical_rune_to_conclusion.clone(),
            current_step: None,
        }
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
        // vassert(rules == rules.distinct)
    }
/*
  override def sanityCheck(): Unit = {
//    vassert(rules == rules.distinct)
  }

*/
// mig: fn get_rule
    fn get_rule(&self, rule_index: i32) -> &Rule {
        &self.rules[rule_index as usize]
    }
/*
  override def getRule(ruleIndex: Int): Rule = rules(ruleIndex)

*/
// mig: fn get_conclusion
    fn get_conclusion(&self, rune: Rune) -> Option<Conclusion> {
        let canonical = self.get_canonical_rune(rune);
        self.canonical_rune_to_conclusion.get(&canonical).cloned()
    }
/*
  override def getConclusion(rune: Rune): Option[Conclusion] = canonicalRuneToConclusion.get(getCanonicalRune(rune))

*/
// mig: fn get_user_rune
    fn get_user_rune(&self, rune: i32) -> Rune {
        self.canonical_rune_to_user_rune
            .get(&rune)
            .expect("rune must exist")
            .clone()
    }
/*
  override def getUserRune(rune: Int): Rune = canonicalRuneToUserRune(rune)

*/
// mig: fn get_conclusions
    fn get_conclusions(&self) -> Vec<(i32, Conclusion)> {
        self.canonical_rune_to_conclusion
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect()
    }
/*
  override def getConclusions(): Stream[(Int, Conclusion)] = {
    canonicalRuneToConclusion.toStream
  }

*/
// mig: fn userify_conclusions
    fn userify_conclusions(&self) -> Vec<(Rune, Conclusion)> {
        self.canonical_rune_to_conclusion
            .iter()
            .map(|(canonical_rune, conclusion)| {
                (
                    self.get_user_rune(*canonical_rune),
                    conclusion.clone(),
                )
            })
            .collect()
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
        self.open_rule_to_puzzle_to_runes
            .values()
            .flat_map(|v| v.iter().flat_map(|r| r.iter().cloned()))
            .collect()
    }
/*
  override def getAllRunes(): Set[Int] = {
    openRuleToPuzzleToRunes.values.flatten.flatten.toSet
  }

*/
// mig: fn add_rune
    fn add_rune(&mut self, rune: Rune) -> i32 {
        assert!(
            !self.user_rune_to_canonical_rune.contains_key(&rune),
            "vassert: rune must not already be registered"
        );
        let new_canonical_rune = self.user_rune_to_canonical_rune.len() as i32;
        self.user_rune_to_canonical_rune
            .insert(rune.clone(), new_canonical_rune);
        self.canonical_rune_to_user_rune
            .insert(new_canonical_rune, rune);
        new_canonical_rune
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
        self.rules.clone()
    }
/*
  override def getAllRules(): Vector[Rule] = {
    rules
  }

*/
// mig: fn add_rule
    fn add_rule(&mut self, rule: Rule) -> i32 {
        let new_canonical_rule = self.rules.len() as i32;
        self.rules.push(rule);
        new_canonical_rule
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
    

    fn get_canonical_rune(&self, rune: Rune) -> i32 {
        *self
            .user_rune_to_canonical_rune
            .get(&rune)
            .expect("vassertSome: rune must be registered")
    }
/*
  override def getCanonicalRune(rune: Rune): Int = {
    vassertSome(userRuneToCanonicalRune.get(rune))
  }

*/
// mig: fn add_puzzle
    fn add_puzzle(&mut self, rule_index: i32, runes: Vec<i32>) {
        let entry = self
            .open_rule_to_puzzle_to_runes
            .entry(rule_index)
            .or_insert_with(Vec::new);
        entry.push(runes);
    }
/*
  override def addPuzzle(ruleIndex: Int, runes: Vector[Int]): Unit = {
    val thisRulePuzzleToRunes = openRuleToPuzzleToRunes.getOrElse(ruleIndex, Vector())
    openRuleToPuzzleToRunes = openRuleToPuzzleToRunes + (ruleIndex -> (thisRulePuzzleToRunes :+ runes))
  }

*/
// mig: fn get_next_solvable
    fn get_next_solvable(&self) -> Option<i32> {
        // Get rule with lowest ID, keep it deterministic (matches Scala)
        self.open_rule_to_puzzle_to_runes
            .iter()
            .filter(|(_, puzzle_to_runes)| {
                puzzle_to_runes.iter().any(|runes| {
                    runes
                        .iter()
                        .all(|r| self.canonical_rune_to_conclusion.contains_key(r))
                })
            })
            .map(|(rule_index, _)| *rule_index)
            .min()
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
        self.open_rule_to_puzzle_to_runes
            .keys()
            .map(|&idx| self.rules[idx as usize].clone())
            .collect()
    }
/*
  override def getUnsolvedRules(): Vector[Rule] = {
    openRuleToPuzzleToRunes.keySet.toVector.map(rules)
  }

*/
// mig: fn conclude_rune
    fn conclude_rune<ErrType>(
        &mut self,
        newly_solved_rune: i32,
        new_conclusion: Conclusion,
    ) -> Result<bool, super::ISolverError<Rune, Conclusion, ErrType>> {
        let is_new = match self.canonical_rune_to_conclusion.get(&newly_solved_rune) {
            Some(existing) => {
                if *existing != new_conclusion {
                    return Err(super::ISolverError::SolverConflict(
                        super::SolverConflict {
                            rune: self.get_user_rune(newly_solved_rune),
                            previous_conclusion: existing.clone(),
                            new_conclusion,
                            _phantom: std::marker::PhantomData,
                        },
                    ));
                }
                false
            }
            None => true,
        };
        self.canonical_rune_to_conclusion
            .insert(newly_solved_rune, new_conclusion);
        Ok(is_new)
    }
/*
  // Returns whether it's a new conclusion
  def concludeRune[ErrType](newlySolvedRune: Int, newConclusion: Conclusion):
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

*/
// mig: fn mark_rules_solved
    fn mark_rules_solved<ErrType>(
        &mut self,
        rule_indices: Vec<i32>,
        new_conclusions: std::collections::HashMap<i32, Conclusion>,
    ) -> Result<i32, super::ISolverError<Rune, Conclusion, ErrType>> {
        let mut num_new_conclusions = 0;
        for (newly_solved_rune, new_conclusion) in new_conclusions {
            let is_new = self.conclude_rune::<ErrType>(newly_solved_rune, new_conclusion)?;
            if is_new {
                num_new_conclusions += 1;
            }
        }
        for rule_index in rule_indices {
            self.remove_rule(rule_index);
        }
        Ok(num_new_conclusions)
    }
/*
  // Success returns number of new conclusions
  override def markRulesSolved[ErrType](ruleIndices: Vector[Int], newConclusions: Map[Int, Conclusion]):
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

*/

    // MIGALLOW: Rust commits immediately; Scala buffered in StepState then committed in markRulesSolved.
    fn step_conclude_rune<ErrType>(
        &mut self,
        _range_s: Vec<crate::utils::range::RangeS<'_>>,
        rune: Rune,
        conclusion: Conclusion,
    ) -> Result<(), super::ISolverError<Rune, Conclusion, ErrType>> {
        let canonical_rune = self.get_canonical_rune(rune.clone());
        let is_new = self.conclude_rune::<ErrType>(canonical_rune, conclusion.clone())?;
        let current = self
            .current_step
            .as_mut()
            .expect("step_conclude_rune called outside of step");
        if is_new {
            current.num_new_conclusions += 1;
        }
        current.step.conclusions.insert(rune, conclusion);
        Ok(())
    }

    fn step_add_rule(&mut self, rule: Rule, puzzles: Vec<Vec<Rune>>) {
        let rule_index = self.add_rule(rule.clone());
        for puzzle_user_runes in &puzzles {
            let canonical_runes: Vec<i32> = puzzle_user_runes
                .iter()
                .map(|r| self.get_canonical_rune(r.clone()))
                .collect();
            self.add_puzzle(rule_index, canonical_runes.clone());
        }
        let current = self
            .current_step
            .as_mut()
            .expect("step_add_rule called outside of step");
        current.step.added_rules.push(rule);
    }

    fn begin_step(&mut self, complex: bool, solved_rules: Vec<(i32, Rule)>) {
        self.current_step = Some(CurrentStep {
            step: super::Step {
                complex,
                solved_rules,
                added_rules: vec![],
                conclusions: std::collections::HashMap::new(),
            },
            num_new_conclusions: 0,
        });
    }

    fn end_step(
        &mut self,
        rule_indices_to_remove: Vec<i32>,
    ) -> (super::Step<Rule, Rune, Conclusion>, i32) {
        let current = self
            .current_step
            .take()
            .expect("end_step called without begin_step");
        for idx in rule_indices_to_remove {
            self.remove_rule(idx);
        }
        self.steps.push(current.step.clone());
        (current.step, current.num_new_conclusions)
    }

    fn get_steps(&self) -> Vec<super::Step<Rule, Rune, Conclusion>> {
        self.steps.clone()
    }
}

/*
  private def removeRule(ruleIndex: Int) = {
    openRuleToPuzzleToRunes = openRuleToPuzzleToRunes - ruleIndex
  }


  // MIGALLOW: No SimpleStepState in Rust
  class SimpleStepState(
  ruleToPuzzles: Rule => Vector[Vector[Rune]],
    complex: Boolean,
    rules: Vector[(Int, Rule)]
  ) extends IStepState[Rule, Rune, Conclusion] {
    private var alive = true
    private var tentativeStep: Step[Rule, Rune, Conclusion] = Step(complex, rules, Vector(), Map())

    def close(): Step[Rule, Rune, Conclusion] = {
      vassert(alive)
      alive = false
      tentativeStep
    }

    override def getConclusion(requestedUserRune: Rune): Option[Conclusion] = {
      vassert(alive)
      SimpleSolverState.this.getConclusion(requestedUserRune)
    }

    override def addRule(rule: Rule): Unit = {
      vassert(alive)
      val ruleIndex = SimpleSolverState.this.addRule(rule)
      tentativeStep = tentativeStep.copy(addedRules = tentativeStep.addedRules :+ rule)
      ruleToPuzzles(rule).foreach(puzzleUserRunes => {
        val puzzleCanonicalRunes = puzzleUserRunes.map(SimpleSolverState.this.getCanonicalRune)
        SimpleSolverState.this.addPuzzle(ruleIndex, puzzleCanonicalRunes)
      })
    }

    override def getUnsolvedRules(): Vector[Rule] = {
      vassert(alive)
      SimpleSolverState.this.getUnsolvedRules()
    }

    override def concludeRune[ErrType](rangeS: List[RangeS], newlySolvedUserRune: Rune, conclusion: Conclusion): Unit = {
      vassert(alive)
      tentativeStep = tentativeStep.copy(conclusions = tentativeStep.conclusions + (newlySolvedUserRune -> conclusion))
    }
  }

  override def initialStep[ErrType](
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

  override def getSteps(): Stream[Step[Rule, Rune, Conclusion]] = steps.toStream
}
*/
