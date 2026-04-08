package dev.vale.solver

import dev.vale.{Err, Ok, RangeS, Result, vassert, vassertSome, vcurious, vfail, vimpl}

import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer


object SimpleSolverState {
  def apply[Rule, Rune, Conclusion](ruleToPuzzles: Rule => Vector[Vector[Rune]]): SimpleSolverState[Rule, Rune, Conclusion] = {
    SimpleSolverState[Rule, Rune, Conclusion](
      ruleToPuzzles,
      Vector(),
//      Map[Rune, Int](),
//      Map[Int, Rune](),
      Vector[Rule](),
      Set[Rune](),
      Map[Int, Vector[Vector[Rune]]](),
      Map[Rune, Conclusion]())
  }
}

case class SimpleSolverState[Rule, Rune, Conclusion](
  private val ruleToPuzzles_ : (Rule) => Vector[Vector[Rune]],

  private var steps: Vector[Step[Rule, Rune, Conclusion]],
//
//  private var userRuneToCanonicalRune: Map[Rune, Int],
//  private var canonicalRuneToUserRune: Map[Int, Rune],

  private var rules: Vector[Rule],

  private var allRunes: Set[Rune],

  var openRuleToPuzzleToRunes: Map[Int, Vector[Vector[Rune]]],

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

  def addRuleAndPuzzles(rule: Rule): Unit = {
    val ruleIndex = addRule(rule)
    sanityCheck()
    ruleToPuzzles_(rule).foreach(puzzle => {
      addPuzzle(ruleIndex, puzzle.distinct) // TODO: is distinct necessary?
    })
    sanityCheck()
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

  def addRune(rune: Rune): Int = {
    vassert(!allRunes.contains(rune))
    val index = allRunes.size
    allRunes += rune
    index
  }

  private def addRule(rule: Rule): Int = {
    val newCanonicalRule = rules.size
    rules = rules :+ rule
    ruleToPuzzles_(rule).foreach(_.foreach(rune => vassert(allRunes.contains(rune))))
//    canonicalRuleToUserRule = canonicalRuleToUserRule + (newCanonicalRule -> rule)
    newCanonicalRule
  }

  private def addStep(step: Step[Rule, Rune, Conclusion]): Unit = {
    steps = steps :+ step
  }

  private def addPuzzle(ruleIndex: Int, runes: Vector[Rune]): Unit = {
    val thisRulePuzzleToRunes = openRuleToPuzzleToRunes.getOrElse(ruleIndex, Vector())
    openRuleToPuzzleToRunes = openRuleToPuzzleToRunes + (ruleIndex -> (thisRulePuzzleToRunes :+ runes))
  }

  def commitStep[ErrType](complex: Boolean, solvedRuleIndices: Vector[Int], conclusions: Map[Rune, Conclusion], newRules: Vector[Rule]):
  Result[Unit, ISolverError[Rune, Conclusion, ErrType]] = {
    val step = Step[Rule, Rune, Conclusion](complex, solvedRuleIndices.map(ruleIndex => (ruleIndex, rules(ruleIndex))), newRules, conclusions)
    conclusions.foreach({ case (rune, conclusion) =>
      concludeRune[ErrType](rune, conclusion) match {
        case Ok(_) =>
        case Err(e) => return Err(e)
      }
    })
    solvedRuleIndices.foreach(ruleIndex => removeRule(ruleIndex))
    addStep(step)
    newRules.foreach(rule => addRuleAndPuzzles(rule))
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

  // Returns whether it's a new conclusion
  private def concludeRune[ErrType](newlySolvedRune: Rune, newConclusion: Conclusion):
  Result[Boolean, ISolverError[Rune, Conclusion, ErrType]] = {
    val isNew =
      runeToConclusion.get(newlySolvedRune) match {
        case Some(existingConclusion) => {
          if (existingConclusion != newConclusion) {
            return Err(
              SolverConflict(
                newlySolvedRune,
                existingConclusion,
                newConclusion))
          }
          false
        }
        case None => true
      }
    runeToConclusion = runeToConclusion + (newlySolvedRune -> newConclusion)
    Ok(isNew)
  }

  def removeRule(ruleIndex: Int) = {
    openRuleToPuzzleToRunes = openRuleToPuzzleToRunes - ruleIndex
  }

  def getSteps(): Stream[Step[Rule, Rune, Conclusion]] = steps.toStream
}
