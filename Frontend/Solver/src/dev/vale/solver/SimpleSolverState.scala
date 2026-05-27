package dev.vale.solver

import dev.vale.{Err, Ok, RangeS, Result, vassert, vassertSome, vcurious, vfail, vimpl}

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer

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

  def sanityCheck(): Unit = {
//    vassert(rules == rules.distinct)
  }

  def getRule(ruleIndex: Int): Rule = rules(ruleIndex)

  def getConclusion(rune: Rune): Option[Conclusion] = runeToConclusion.get(rune)

//  def getUserRune(rune: Int): Rune = canonicalRuneToUserRune(rune)

  def getConclusions(): Stream[(Rune, Conclusion)] = {
    runeToConclusion.toStream
  }

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

  def commitStep[ErrType](
      complex: Boolean,
      solvedRuleIndices: Vector[Int],
      conclusions: Map[Rune, Conclusion],
      newRules: Vector[Rule],
      // `newRunes` extends allRunes mid-solve, used when incrementally committing rules
      // that introduce previously-unknown runes (e.g. default-only runes that travel inside
      // GenericParameterDefaultS, see DRSINI).
      // DO NOT SUBMIT undefault this param
      newRunes: Set[Rune] = Set.empty):
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
