/*
package dev.vale.solver

import dev.vale.{Err, Ok, RangeS, Result, vassert, vassertSome, vcurious, vfail, vimpl}

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer


object SimpleSolverState {
  def apply[Rule, Rune, Conclusion](ruleToPuzzles: Rule => Vector[Vector[Rune]], allRunes: Vector[Rune]): SimpleSolverState[Rule, Rune, Conclusion] = {
    SimpleSolverState[Rule, Rune, Conclusion](
      ruleToPuzzles,
      Vector(),
//      Map[Rune, Int](),
//      Map[Int, Rune](),
      Vector[Rule](),
      allRunes.toSet,
      Map[Int, Vector[Vector[Rune]]](),
      Map[Rune, Conclusion]())
  }

  object Solver {
    def apply[Rule, Rune, Conclusion](
        sanityCheck: Boolean,
        useOptimizedSolver: Boolean,
        ruleToPuzzles: Rule => Vector[Vector[Rune]],
        ruleToRunes: Rule => Iterable[Rune],
        initialRules: IndexedSeq[Rule],
        initiallyKnownRunes: Map[Rune, Conclusion],
        allRunes: Vector[Rune]
    ): SimpleSolverState[Rule, Rune, Conclusion] = {
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
case class SimpleSolverState[Rule, Rune, Conclusion](
  private val ruleToPuzzles_ : (Rule) => Vector[Vector[Rune]],

  private var steps: Vector[Step[Rule, Rune, Conclusion]],
//
//  private var userRuneToCanonicalRune: Map[Rune, Int],
//  private var canonicalRuneToUserRune: Map[Int, Rune],

  private var rules: Vector[Rule],

  private var allRunes: Set[Rune],

    private var openRuleToPuzzleToRunes: Map[Int, Vector[Vector[Rune]]],

  private var runeToConclusion: Map[Rune, Conclusion]
) {

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
  override def equals(obj: Any): Boolean = vcurious()
  override def hashCode(): Int = vfail() // is mutable, should never be hashed

*/
    fn new() -> Self {
        SimpleSolverState::new()
    }

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

// mig: fn sanity_check
    fn sanity_check(&self) {
        // vassert(rules == rules.distinct)
    }
/*
  def sanityCheck(): Unit = {
//    vassert(rules == rules.distinct)
  }

*/
// mig: fn get_rule
    fn get_rule(&self, rule_index: i32) -> &Rule {
        &self.rules[rule_index as usize]
    }
/*
  def getRule(ruleIndex: Int): Rule = rules(ruleIndex)

*/
// mig: fn get_conclusion
    fn get_conclusion(&self, rune: Rune) -> Option<Conclusion> {
        let canonical = self.get_canonical_rune(rune);
        self.canonical_rune_to_conclusion.get(&canonical).cloned()
    }
/*
  def getConclusion(rune: Rune): Option[Conclusion] = runeToConclusion.get(rune)

//  def getUserRune(rune: Int): Rune = canonicalRuneToUserRune(rune)

  def getConclusions(): Stream[(Rune, Conclusion)] = {
    runeToConclusion.toStream
  }

*/
// mig: fn get_user_rune
    fn get_user_rune(&self, rune: i32) -> Rune {
        self.canonical_rune_to_user_rune
            .get(&rune)
            .expect("rune must exist")
            .clone()
    }

// mig: fn get_conclusions
    fn get_conclusions(&self) -> Vec<(i32, Conclusion)> {
        self.canonical_rune_to_conclusion
            .iter()
            .map(|(k, v)| (*k, v.clone()))
            .collect()
    }

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
  def userifyConclusions(): Stream[(Rune, Conclusion)] = {
    runeToConclusion.toStream
  }

  def getAllRunes(): Set[Rune] = {
    allRunes
  }

  def isComplete(): Boolean = {
    userifyConclusions().size == getAllRunes().size
  }

//  def addRune(rune: Rune): Int = {
//    vassert(!allRunes.contains(rune))
//    val index = allRunes.size
//    allRunes += rune
//    index
//  }

  def commitStep[ErrType](complex: Boolean, solvedRuleIndices: Vector[Int], conclusions: Map[Rune, Conclusion], newRules: Vector[Rule]):
  Result[Unit, ISolverError[Rune, Conclusion, ErrType]] = {
    val step = Step[Rule, Rune, Conclusion](complex, solvedRuleIndices.map(ruleIndex => (ruleIndex, rules(ruleIndex))), newRules, conclusions)
    // Append step before checking for conflicts, so the audit trail captures
    // the conflicting step even when we return an error below.
    steps = steps :+ step
    conclusions.foreach({ case (newlySolvedRune, newConclusion) =>
      runeToConclusion.get(newlySolvedRune) match {
        case Some(existingConclusion) => {
          if (existingConclusion != newConclusion) {
            return Err(
              SolverConflict(
                newlySolvedRune,
                existingConclusion,
                newConclusion))
          }
        }
        case None =>
      }
      runeToConclusion = runeToConclusion + (newlySolvedRune -> newConclusion)
    })
    solvedRuleIndices.foreach(ruleIndex => openRuleToPuzzleToRunes = openRuleToPuzzleToRunes - ruleIndex)
    newRules.foreach(rule => {
      val ruleIndex = {
        val newCanonicalRule = rules.size
        rules = rules :+ rule
        ruleToPuzzles_(rule).foreach(_.foreach(rune => vassert(allRunes.contains(rune))))
        //    canonicalRuleToUserRule = canonicalRuleToUserRule + (newCanonicalRule -> rule)
        newCanonicalRule
      }
      sanityCheck()
      ruleToPuzzles_(rule).foreach(puzzle => {
        {
          val thisRulePuzzleToRunes = openRuleToPuzzleToRunes.getOrElse(ruleIndex, Vector())
          openRuleToPuzzleToRunes = openRuleToPuzzleToRunes + (ruleIndex -> (thisRulePuzzleToRunes :+ puzzle.distinct))
        } // TODO: is distinct necessary?
      })
      sanityCheck()
    })
    Ok(())
  }

*/
// mig: fn get_all_runes
    fn get_all_runes(&self) -> std::collections::HashSet<i32> {
        self.open_rule_to_puzzle_to_runes
            .values()
            .flat_map(|v| v.iter().flat_map(|r| r.iter().cloned()))
            .collect()
    }

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

// mig: fn get_all_rules
    fn get_all_rules(&self) -> Vec<Rule> {
        self.rules.clone()
    }

// mig: fn add_rule
    fn add_rule(&mut self, rule: Rule) -> i32 {
        let new_canonical_rule = self.rules.len() as i32;
        self.rules.push(rule);
        new_canonical_rule
    }

// mig: fn get_canonical_rune


    fn get_canonical_rune(&self, rune: Rune) -> i32 {
        *self
            .user_rune_to_canonical_rune
            .get(&rune)
            .expect("vassertSome: rune must be registered")
    }

// mig: fn add_puzzle
    fn add_puzzle(&mut self, rule_index: i32, runes: Vec<i32>) {
        let entry = self
            .open_rule_to_puzzle_to_runes
            .entry(rule_index)
            .or_insert_with(Vec::new);
        entry.push(runes);
    }

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
  def getNextSolvable(): Option[Int] = {
    openRuleToPuzzleToRunes
      .filter({ case (_, puzzleToRunes) =>
        puzzleToRunes.exists(runes => {
          runes.forall(rune => runeToConclusion.contains(rune))
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
  def getUnsolvedRules(): Vector[Rule] = {
    openRuleToPuzzleToRunes.keySet.toVector.map(rules)
  }
  def getUnsolvedRunes(): Vector[Rune] = {
    (getAllRunes() -- getConclusions().map(_._1)).toVector
  }

  def getSteps(): Stream[Step[Rule, Rune, Conclusion]] = steps.toStream

  def ruleIsSolved(solvingRuleIndex: Int): Boolean = {
    !openRuleToPuzzleToRunes.contains(solvingRuleIndex)
  }
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
