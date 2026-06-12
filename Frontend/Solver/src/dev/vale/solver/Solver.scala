package dev.vale.solver

import dev.vale.{Err, Interner, Ok, Profiler, RangeS, Result, vassert, vfail, vimpl, vpass}

import scala.collection.immutable.Map
import scala.collection.mutable

case class Step[Rule, Rune, Conclusion](complex: Boolean, solvedRules: Vector[(Int, Rule)], addedRules: Vector[Rule], conclusions: Map[Rune, Conclusion])


case class FailedSolve[Rule, Rune, Conclusion, ErrType](
  steps: Stream[Step[Rule, Rune, Conclusion]],
  conclusions: Map[Rune, Conclusion],
  unsolvedRules: Vector[Rule],
  unsolvedRunes: Vector[Rune],
  error: ISolverError[Rune, Conclusion, ErrType]
)

sealed trait ISolverError[Rune, Conclusion, ErrType]
case class SolveIncomplete[Rune, Conclusion, ErrType]() extends ISolverError[Rune, Conclusion, ErrType]
case class SolverConflict[Rune, Conclusion, ErrType](
  rune: Rune,
  previousConclusion: Conclusion,
  newConclusion: Conclusion
) extends ISolverError[Rune, Conclusion, ErrType]
case class RuleError[Rune, Conclusion, ErrType](
//  ruleIndex: Int,
  err: ErrType
) extends ISolverError[Rune, Conclusion, ErrType]

// Given enough user specified template params and param inputs, we should be able to
// infer everything.
// This class's purpose is to take those things, and see if it can figure out as many
// inferences as possible.

//trait ISolveRule[Rule, Rune, Env, State, Conclusion, ErrType] {
//  def solve(
//    state: State,
//    env: Env,
//    solverState: SimpleSolverState[Rule, Rune, Conclusion],
//    ruleIndex: Int,
//    rule: Rule):
//  Result[Unit, ISolverError[Rune, Conclusion, ErrType]]
//
//  // Called when we can't do any regular solves, we don't have enough
//  // runes. This is where we do more interesting rules, like SMCMST.
//  // See CSALR for more.
//  def complexSolve(
//    state: State,
//    env: Env,
//    solverState: SimpleSolverState[Rule, Rune, Conclusion]
//  ): Result[Unit, ISolverError[Rune, Conclusion, ErrType]]
//
//  def sanityCheckConclusion(env: Env, state: State, rune: Rune, conclusion: Conclusion): Unit
//}

object Solver {
  def makeSolverState[Rule, Rune, Conclusion](
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
