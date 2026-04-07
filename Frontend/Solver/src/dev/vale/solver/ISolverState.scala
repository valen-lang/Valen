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

trait ISolverState[Rule, Rune, Conclusion] {
  def deepClone(): ISolverState[Rule, Rune, Conclusion]
  def getCanonicalRune(rune: Rune): Int
  def getUserRune(rune: Int): Rune
  def getRule(ruleIndex: Int): Rule
  def getConclusion(rune: Rune): Option[Conclusion]
  def getConclusions(): Stream[(Int, Conclusion)]
  def userifyConclusions(): Stream[(Rune, Conclusion)]
  def getUnsolvedRules(): Vector[Rule]
  def getNextSolvable(): Option[Int]
  def getSteps(): Stream[Step[Rule, Rune, Conclusion]]

  def isComplete(): Boolean

  def addRule(rule: Rule): Int
  def addRune(rune: Rune): Int
  def removeRule(ruleIndex: Int): Unit
  def addStep(step: Step[Rule, Rune, Conclusion]): Unit

  def getAllRunes(): Set[Int]
  def getAllRules(): Vector[Rule]

  def addPuzzle(ruleIndex: Int, runes: Vector[Int]): Unit

//  // TODO DO NOT SUBMIT: add a addRuleAndPuzzles which calls addPuzzle, addRule, and this, and get rid of this
//  def getPuzzlesForRule(rule: Rule): Vector[Vector[Rune]]

  def addRuleAndPuzzles(rule: Rule): Unit

  def sanityCheck(): Unit

//  // Success returns number of new conclusions
//  def markRulesSolvedZZZ[ErrType](ruleIndices: Vector[Int], newConclusions: Map[Int, Conclusion]):
//  Result[Int, ISolverError[Rune, Conclusion, ErrType]]

//  def addStep(step: Step[Rule, Rune, Conclusion]): Unit


  def concludeRune[ErrType](newlySolvedRune: Int, conclusion: Conclusion):
  Result[Boolean, ISolverError[Rune, Conclusion, ErrType]]
}
