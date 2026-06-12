package dev.vale.postparsing

import dev.vale.postparsing.rules._
import dev.vale.solver.{FailedSolve, ISolverError, SimpleSolverState, SolveIncomplete, Solver}
import dev.vale.{Err, Ok, RangeS, Result, vassert, vimpl, vpass}
import dev.vale._
import dev.vale.postparsing.rules._

import scala.collection.immutable.Map

case class IdentifiabilitySolveError(range: List[RangeS], failedSolve: FailedSolve[IRulexSR, IRuneS, Boolean, IIdentifiabilityRuleError]) {
  vpass()
}

sealed trait IIdentifiabilityRuleError

// Identifiability is whether the denizen has enough identifying runes to uniquely identify all its
// instantiations. It's only used as a check, and will throw an error if there's a rune that can't
// be derived from the identifying runes.
object IdentifiabilitySolver {
  def getRunes(rule: IRulexSR): Vector[IRuneS] = {
    val sanityCheck =
      rule match {
        case MaybeCoercingLookupSR(range, rune, literal) => Vector(rune)
        case LookupSR(range, rune, literal) => Vector(rune)
        case RuneParentEnvLookupSR(range, rune) => Vector(rune)
        case EqualsSR(range, left, right) => Vector(left, right)
        case KindComponentsSR(range, resultRune, mutabilityRune) => Vector(resultRune, mutabilityRune)
        case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => Vector(resultRune, ownershipRune, kindRune)
        case PrototypeComponentsSR(range, resultRune, paramsRune, returnRune) => Vector(resultRune, paramsRune, returnRune)
        case ResolveSR(range, resultRune, name, paramsListRune, returnRune) => Vector(resultRune, paramsListRune, returnRune)
        case CallSiteFuncSR(range, prototypeRune, name, paramsListRune, returnRune) => Vector(prototypeRune, paramsListRune, returnRune)
        case DefinitionFuncSR(range, resultRune, name, paramsListRune, returnRune) => Vector(resultRune, paramsListRune, returnRune)
        case CallSiteCoordIsaSR(range, resultRune, sub, suuper) => resultRune.toVector ++ Vector(sub, suuper)
        case DefinitionCoordIsaSR(range, resultRune, sub, suuper) => Vector(resultRune, sub, suuper)
        case OneOfSR(range, rune, literals) => Vector(rune)
        case IsConcreteSR(range, rune) => Vector(rune)
        case IsInterfaceSR(range, rune) => Vector(rune)
        case IsStructSR(range, rune) => Vector(rune)
        case CoerceToCoordSR(range, coordRune, kindRune) => Vector(coordRune, kindRune)
        case LiteralSR(range, rune, literal) => Vector(rune)
        case AugmentSR(range, resultRune, maybeOwnership, innerRune) => Vector(resultRune, innerRune)
        case MaybeCoercingCallSR(range, resultRune, templateRune, args) => Vector(resultRune, templateRune) ++ args
//        case PrototypeSR(range, resultRune, name, parameters, returnTypeRune) => Vector(resultRune) ++ parameters ++ Vector(returnTypeRune)
        case PackSR(range, resultRune, members) => Vector(resultRune) ++ members
//        case StaticSizedArraySR(range, resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune) => Vector(resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune)
//        case RuntimeSizedArraySR(range, resultRune, mutabilityRune, elementRune) => Vector(resultRune, mutabilityRune, elementRune)
//        case ManualSequenceSR(range, resultRune, elements) => Vector(resultRune) ++ elements
        case RefListCompoundMutabilitySR(range, resultRune, coordListRune) => Vector(resultRune, coordListRune)
//        case CoordListSR(range, resultRune, elements) => Vector(resultRune) ++ elements
      }
    val result = rule.runeUsages
    vassert(result.map(_.rune) sameElements sanityCheck.map(_.rune))
    result.map(_.rune)
  }

  def getPuzzles(rule: IRulexSR): Vector[Vector[IRuneS]] = {
    rule match {
      case EqualsSR(range, leftRune, rightRune) => Vector(Vector(leftRune.rune), Vector(rightRune.rune))
      case MaybeCoercingLookupSR(range, rune, _) => Vector(Vector())
      case LookupSR(range, rune, _) => Vector(Vector())
      case RuneParentEnvLookupSR(range, rune) => {
        // This Vector() literally means nothing can solve this puzzle.
        // It needs to be passed in via identifying rune.
        Vector()
      }
      case MaybeCoercingCallSR(range, resultRune, templateRune, args) => {
        // We can't determine the template from the result and args because we might be coercing its
        // returned kind to a coord. So we need the template.
        // We can't determine the return type because we don't know whether we're coercing or not.
        Vector(Vector(resultRune.rune, templateRune.rune), Vector(templateRune.rune) ++ args.map(_.rune))
      }
      case PackSR(range, resultRune, members) => {
        // Packs are always lists of coords
        Vector(Vector(resultRune.rune), members.map(_.rune))
      }
      case DefinitionCoordIsaSR(range, resultRune, subRune, superRune) => Vector(Vector())
      case CallSiteCoordIsaSR(range, resultRune, subRune, superRune) => Vector(Vector())
      case KindComponentsSR(range, resultRune, mutabilityRune) => Vector(Vector())
      case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => Vector(Vector())
      case PrototypeComponentsSR(range, resultRune, ownershipRune, kindRune) => Vector(Vector())
      case ResolveSR(range, resultRune, nameRune, paramsListRune, returnRune) => Vector(Vector())
      case CallSiteFuncSR(range, resultRune, nameRune, paramsListRune, returnRune) => Vector(Vector())
      case DefinitionFuncSR(range, resultRune, name, paramsListRune, returnRune) => Vector(Vector())
      case OneOfSR(range, rune, literals) => Vector(Vector())
      case IsConcreteSR(range, rune) => Vector(Vector(rune.rune))
      case IsInterfaceSR(range, rune) => Vector(Vector())
      case IsStructSR(range, rune) => Vector(Vector())
      case CoerceToCoordSR(range, coordRune, kindRune) => Vector(Vector())
      case LiteralSR(range, rune, literal) => Vector(Vector())
      case AugmentSR(range, resultRune, ownership, innerRune) => Vector(Vector())
//      case StaticSizedArraySR(range, resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune) => Vector(Vector(resultRune.rune), Vector(mutabilityRune.rune, variabilityRune.rune, sizeRune.rune, elementRune.rune))
//      case RuntimeSizedArraySR(range, resultRune, mutabilityRune, elementRune) => Vector(Vector(resultRune.rune), Vector(mutabilityRune.rune, elementRune.rune))

//      case ManualSequenceSR(range, resultRune, elements) => Vector(Vector(resultRune.rune))
      case RefListCompoundMutabilitySR(range, resultRune, coordListRune) => Vector(Vector())
        // solverState.addPuzzle(ruleIndex, Vector(senderRune, receiverRune))
//      case CoordListSR(range, resultRune, elements) => Vector(Vector())
    }
  }

  private def solveRule(
    solverState: SimpleSolverState[IRulexSR, IRuneS, Boolean],
    ruleIndex: Int,
    rule: IRulexSR):
  Result[Unit, ISolverError[IRuneS, Boolean, IIdentifiabilityRuleError]] = {
    rule match {
      case KindComponentsSR(range, resultRune, mutabilityRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, mutabilityRune.rune -> true), Vector(), Set.empty)
      }
      case CoordComponentsSR(range, resultRune, ownershipRune, kindRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, ownershipRune.rune -> true, kindRune.rune -> true), Vector(), Set.empty)
      }
      case PrototypeComponentsSR(range, resultRune, paramsRune, returnRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, paramsRune.rune -> true, returnRune.rune -> true), Vector(), Set.empty)
      }
      case MaybeCoercingCallSR(range, resultRune, templateRune, argRunes) => {
        val conclusions =
          argRunes.map(_.rune).map({ case argRune => (argRune -> true) }).toMap ++
              Map(resultRune.rune -> true, templateRune.rune -> true)
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), conclusions, Vector(), Set.empty)
      }
      case ResolveSR(range, resultRune, name, paramListRune, returnRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, paramListRune.rune -> true, returnRune.rune -> true), Vector(), Set.empty)
      }
      case CallSiteFuncSR(range, resultRune, name, paramListRune, returnRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, paramListRune.rune -> true, returnRune.rune -> true), Vector(), Set.empty)
      }
      case DefinitionFuncSR(range, resultRune, name, paramsListRune, returnRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, paramsListRune.rune -> true, returnRune.rune -> true), Vector(), Set.empty)
      }
      case DefinitionCoordIsaSR(range, resultRune, subRune, superRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, subRune.rune -> true, superRune.rune -> true), Vector(), Set.empty)
      }
      case CallSiteCoordIsaSR(range, resultRune, subRune, superRune) => {
        val conclusions = Map(subRune.rune -> true, superRune.rune -> true) ++
            (resultRune match {
              case None => Map()
              case Some(resultRune) => Map(resultRune.rune -> true)
            })
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), conclusions, Vector(), Set.empty)
      }
      case OneOfSR(range, resultRune, literals) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true), Vector(), Set.empty)
      }
      case EqualsSR(range, leftRune, rightRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(leftRune.rune -> true, rightRune.rune -> true), Vector(), Set.empty)
      }
      case IsConcreteSR(range, rune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(rune.rune -> true), Vector(), Set.empty)
      }
      case IsInterfaceSR(range, rune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(rune.rune -> true), Vector(), Set.empty)
      }
      case IsStructSR(range, rune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(rune.rune -> true), Vector(), Set.empty)
      }
      case RefListCompoundMutabilitySR(range, resultRune, coordListRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, coordListRune.rune -> true), Vector(), Set.empty)
      }
      case CoerceToCoordSR(range, coordRune, kindRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(kindRune.rune -> true, coordRune.rune -> true), Vector(), Set.empty)
      }
      case LiteralSR(range, rune, literal) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(rune.rune -> true), Vector(), Set.empty)
      }
      case LookupSR(range, rune, name) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(rune.rune -> true), Vector(), Set.empty)
      }
      case MaybeCoercingLookupSR(range, rune, name) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(rune.rune -> true), Vector(), Set.empty)
      }
      case RuneParentEnvLookupSR(range, rune) => {
        vimpl()
//        (env(RuneNameS(rune.rune)), vassertSome(stepState.getConclusion(rune.rune))) match {
//          case (true, true) =>
//          case (TemplateTemplataType(Vector(), true), true) =>
//          case (TemplateTemplataType(Vector(), result), expected) if result == expected =>
//          case (from, to) if from == to =>
//          case (from, to) => {
//            return Err(SolverConflict(rune.rune, to, from))
//          }
//        }
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), vimpl(), Vector(), Set.empty)
      }
      case MaybeCoercingLookupSR(range, rune, name) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(rune.rune -> true), Vector(), Set.empty)
      }
      case AugmentSR(range, resultRune, ownership, innerRune) => {
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), Map(resultRune.rune -> true, innerRune.rune -> true), Vector(), Set.empty)
      }
      case PackSR(range, resultRune, memberRunes) => {
        val conclusions = Map(resultRune.rune -> true) ++ memberRunes.map(x => (x.rune -> true))
        solverState.commitStep[IIdentifiabilityRuleError](false, Vector(ruleIndex), conclusions, Vector(), Set.empty)
      }
//      case StaticSizedArraySR(range, resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune) => {
//        solverState.commitStep[IIdentifiabilityRuleError]resultRune.rune, true), Vector())
//        solverState.commitStep[IIdentifiabilityRuleError]mutabilityRune.rune, true), Vector())
//        solverState.commitStep[IIdentifiabilityRuleError]variabilityRune.rune, true), Vector())
//        solverState.commitStep[IIdentifiabilityRuleError]sizeRune.rune, true), Vector())
//        solverState.commitStep[IIdentifiabilityRuleError]elementRune.rune, true), Vector())
//        Ok(())
//      }
//      case RuntimeSizedArraySR(range, resultRune, mutabilityRune, elementRune) => {
//        solverState.commitStep[IIdentifiabilityRuleError]resultRune.rune, true), Vector())
//        solverState.commitStep[IIdentifiabilityRuleError]mutabilityRune.rune, true), Vector())
//        solverState.commitStep[IIdentifiabilityRuleError]elementRune.rune, true), Vector())
//        Ok(())
//      }
    }
  }

  def solve(
    sanityCheck: Boolean,
    useOptimizedSolver: Boolean,
    interner: Interner,
    callRange: List[RangeS],
    rules: IndexedSeq[IRulexSR],
    identifyingRunes: Iterable[IRuneS]):
  Result[Map[IRuneS, Boolean], IdentifiabilitySolveError] = {
    val initiallyKnownRunes = identifyingRunes.map(r => (r, true)).toMap
    val solverState =
      Solver.makeSolverState(
        sanityCheck,
        useOptimizedSolver,
        (rule: IRulexSR) => getPuzzles(rule),
        getRunes,
        rules,
        initiallyKnownRunes,
        (rules.flatMap(getRunes) ++ initiallyKnownRunes.keys).distinct.toVector)
    while ( {
      solverState.sanityCheck()
      solverState.getNextSolvable() match {
        case None => false // break
        case Some(solvingRuleIndex) => {
          val rule = solverState.getRule(solvingRuleIndex)
          val stepsBefore = solverState.getSteps().size
          solveRule(solverState, solvingRuleIndex, rule) match {
            case Ok(()) => {}
            case Err(e) => return Err(IdentifiabilitySolveError(callRange, FailedSolve(solverState.getSteps(), solverState.getConclusions().toMap, solverState.getUnsolvedRules(), solverState.getUnsolvedRunes(), e)))
          }
          val stepsAfter = solverState.getSteps().size
          vassert(stepsAfter == stepsBefore + 1)
          vassert(solverState.ruleIsSolved(solvingRuleIndex)) // Per @CSCDSRZ, only true after simple solve.
          solverState.sanityCheck()
          // Go back to the beginning. Next step, if there's no simple rule ready to solve, then
          // it'll start doing a complex solve if available, or just finish.
          true
        }
      }
    }) {}
    // If we get here, then there's nothing more the solver can do.

    val steps = solverState.getSteps().toStream
    val conclusions = solverState.userifyConclusions().toMap

    val allRunes = solverState.getAllRunes()
    val unsolvedRunes = allRunes -- conclusions.keySet
    if (unsolvedRunes.nonEmpty) {
      Err(
        IdentifiabilitySolveError(
          callRange,
          FailedSolve(
            steps,
            conclusions,
            solverState.getUnsolvedRules(),
            unsolvedRunes.toVector,
            SolveIncomplete())))
    } else {
      Ok(conclusions)
    }
  }
}
