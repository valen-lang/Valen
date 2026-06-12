use std::collections::HashMap;
use std::collections::HashSet;
use std::hash::Hash;
use std::marker::PhantomData;
/*
package dev.vale.solver

import dev.vale.{Err, Ok, RangeS, Result, vassert, vassertSome, vcurious, vfail, vimpl}

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer

*/
// mig: struct SimpleSolverState
pub struct SimpleSolverState<Rule, Rune, Conclusion>
where
    Rune: Eq + Hash,
{
    rule_to_puzzles: Box<dyn Fn(&Rule) -> Vec<Vec<Rune>>>,
    steps: Vec<super::Step<Rule, Rune, Conclusion>>,
    rules: Vec<Rule>,
    all_runes: HashSet<Rune>,
    open_rule_to_puzzle_to_runes: HashMap<i32, Vec<Vec<Rune>>>,
    rune_to_conclusion: HashMap<Rune, Conclusion>,
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

  override def equals(obj: Any): Boolean = vcurious()
  override def hashCode(): Int = vfail() // is mutable, should never be hashed

*/
// mig: impl SimpleSolverState
impl<Rule, Rune, Conclusion> SimpleSolverState<Rule, Rune, Conclusion>
where
    Rule: Clone,
    Rune: Clone + Hash + Eq,
    Conclusion: Clone + PartialEq,
{
/*

*/
// mig: fn sanity_check
    pub fn sanity_check(&self) {
        // vassert(rules == rules.distinct)
    }
/*
  def sanityCheck(): Unit = {
//    vassert(rules == rules.distinct)
  }

*/
// mig: fn get_rule
    pub fn get_rule(&self, rule_index: i32) -> &Rule {
        &self.rules[rule_index as usize]
    }
/*
  def getRule(ruleIndex: Int): Rule = rules(ruleIndex)

*/
// mig: fn get_conclusion
    pub fn get_conclusion(&self, rune: &Rune) -> Option<Conclusion> {
        self.rune_to_conclusion.get(rune).cloned()
    }
/*
  def getConclusion(rune: Rune): Option[Conclusion] = runeToConclusion.get(rune)

//  def getUserRune(rune: Int): Rune = canonicalRuneToUserRune(rune)

*/
// mig: fn get_conclusions
    pub fn get_conclusions(&self) -> Vec<(Rune, Conclusion)> {
        self.rune_to_conclusion.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
/*
  def getConclusions(): Stream[(Rune, Conclusion)] = {
    runeToConclusion.toStream
  }

*/
// mig: fn userify_conclusions
    pub fn userify_conclusions(&self) -> Vec<(Rune, Conclusion)> {
        self.rune_to_conclusion.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
/*
  def userifyConclusions(): Stream[(Rune, Conclusion)] = {
    runeToConclusion.toStream
  }

*/
    // mig: fn get_all_runes (matches Scala's getAllRunes() -> Set[Rune])
    pub fn get_all_runes(&self) -> HashSet<Rune> {
        self.all_runes.clone()
    }
/*
  def getAllRunes(): Set[Rune] = {
    allRunes
  }

*/
    // mig: fn is_complete
    pub fn is_complete(&self) -> bool {
        self.rune_to_conclusion.len() == self.all_runes.len()
    }
/*
  def isComplete(): Boolean = {
    userifyConclusions().size == getAllRunes().size
  }

//  def addRune(rune: Rune): Int = {
//    vassert(!allRunes.contains(rune))
//    val index = allRunes.size
//    allRunes += rune
//    index
//  }

*/
    // mig: fn commit_step (matches Scala's commitStep)
    pub fn commit_step<ErrType>(
        &mut self,
        complex: bool,
        solved_rule_indices: Vec<i32>,
        conclusions: HashMap<Rune, Conclusion>,
        new_rules: Vec<Rule>,
        new_runes: HashSet<Rune>,
    ) -> Result<(), super::ISolverError<Rune, Conclusion, ErrType>> {
        self.all_runes.extend(new_runes);
        let solved_rules: Vec<(i32, Rule)> = solved_rule_indices
            .iter()
            .map(|&idx| (idx, self.rules[idx as usize].clone()))
            .collect();
        let step = super::Step {
            complex,
            solved_rules,
            added_rules: new_rules.clone(),
            conclusions: conclusions.clone(),
        };
        // Append step before checking for conflicts (audit trail captures conflicting step)
        self.steps.push(step);
        // Check and apply conclusions
        for (rune, new_conclusion) in &conclusions {
            if let Some(existing) = self.rune_to_conclusion.get(rune) {
                if existing != new_conclusion {
                    return Err(super::ISolverError::SolverConflict(
                        super::SolverConflict {
                            rune: rune.clone(),
                            previous_conclusion: existing.clone(),
                            new_conclusion: new_conclusion.clone(),
                            _phantom: PhantomData,
                        },
                    ));
                }
            }
            self.rune_to_conclusion.insert(rune.clone(), new_conclusion.clone());
        }
        // Remove solved rules
        for &rule_index in &solved_rule_indices {
            self.open_rule_to_puzzle_to_runes.remove(&rule_index);
        }
        // Add new rules with puzzles
        for rule in new_rules {
            let rule_index = self.rules.len() as i32;
            self.rules.push(rule.clone());
            let puzzles = (self.rule_to_puzzles)(&rule);
            for puzzle in &puzzles {
                for rune in puzzle {
                    assert!(self.all_runes.contains(rune), "vassert: rune in puzzle must be in allRunes");
                }
            }
            self.sanity_check();
            for puzzle in puzzles {
                let entry = self.open_rule_to_puzzle_to_runes
                    .entry(rule_index)
                    .or_insert_with(Vec::new);
                entry.push(puzzle);
            }
            self.sanity_check();
        }
        Ok(())
    }
/*
  def commitStep[ErrType](
    complex: Boolean,
    solvedRuleIndices: Vector[Int],
    conclusions: Map[Rune, Conclusion],
    newRules: Vector[Rule],
    // `newRunes` extends allRunes mid-solve, used when incrementally committing rules
    // that introduce previously-unknown runes (e.g. default-only runes that travel inside
    // GenericParameterDefaultS, see DRSINI). Pass `Set.empty` when not introducing new runes.
    newRunes: Set[Rune]):
  Result[Unit, ISolverError[Rune, Conclusion, ErrType]] = {
    allRunes = allRunes ++ newRunes
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
// mig: fn get_next_solvable
    pub fn get_next_solvable(&self) -> Option<i32> {
        // Get rule with lowest ID, keep it deterministic (matches Scala)
        self.open_rule_to_puzzle_to_runes
            .iter()
            .filter(|(_, puzzle_to_runes)| {
                puzzle_to_runes.iter().any(|runes| {
                    runes
                        .iter()
                        .all(|r| self.rune_to_conclusion.contains_key(r))
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
    pub fn get_unsolved_rules(&self) -> Vec<Rule> {
        self.open_rule_to_puzzle_to_runes
            .keys()
            .map(|&idx| self.rules[idx as usize].clone())
            .collect()
    }
/*
  def getUnsolvedRules(): Vector[Rule] = {
    openRuleToPuzzleToRunes.keySet.toVector.map(rules)
  }

*/
    // mig: fn get_unsolved_runes (matches Scala's getUnsolvedRunes)
    pub fn get_unsolved_runes(&self) -> Vec<Rune> {
        self.all_runes.difference(
            &self.rune_to_conclusion.keys().cloned().collect()
        ).cloned().collect()
    }
/*
  def getUnsolvedRunes(): Vector[Rune] = {
    (getAllRunes() -- getConclusions().map(_._1)).toVector
  }

*/
    // mig: fn get_steps
    pub fn get_steps(&self) -> Vec<super::Step<Rule, Rune, Conclusion>> {
        self.steps.clone()
    }
/*
  def getSteps(): Stream[Step[Rule, Rune, Conclusion]] = steps.toStream

*/
    // mig: fn rule_is_solved (matches Scala's ruleIsSolved)
    pub fn rule_is_solved(&self, rule_index: i32) -> bool {
        !self.open_rule_to_puzzle_to_runes.contains_key(&rule_index)
    }
/*
  def ruleIsSolved(solvingRuleIndex: Int): Boolean = {
    !openRuleToPuzzleToRunes.contains(solvingRuleIndex)
  }
}

object SimpleSolverState {
*/
pub fn new(
    rule_to_puzzles: Box<dyn Fn(&Rule) -> Vec<Vec<Rune>>>,
    all_runes: Vec<Rune>,
) -> Self {
    SimpleSolverState {
        rule_to_puzzles,
        steps: vec![],
        rules: vec![],
        all_runes: all_runes.into_iter().collect(),
        open_rule_to_puzzle_to_runes: HashMap::new(),
        rune_to_conclusion: HashMap::new(),
    }
}
/*
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

      solverState.commitStep(false, Vector(), initiallyKnownRunes, initialRules.toVector, Set.empty).getOrDie()

      if (sanityCheck) {
        solverState.sanityCheck()
      }
      solverState
    }
  }
}
*/
}
/*
 */