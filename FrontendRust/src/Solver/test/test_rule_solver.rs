/*
package dev.vale.solver

import dev.vale.{Err, Interner, Ok, RangeS, Result, vassert, vassertSome, vfail, vimpl, vwat}
import org.scalatest._

import scala.collection.immutable.Map

*/
// mig: struct TestRuleSolver
pub struct TestRuleSolver {
    _interner: (),
}
/*
class TestRuleSolver(interner: Interner) extends ISolveRule[IRule, Long, Unit, Unit, String, String] {
*/
// mig: impl TestRuleSolver
impl TestRuleSolver {
/*
*/
// mig: fn sanity_check_conclusion
fn sanity_check_conclusion(&self, _env: &(), _state: &(), _rune: i64, _conclusion: &str) {
    panic!("Unimplemented: sanity_check_conclusion");
}
/*
  override def sanityCheckConclusion(env: Unit, state: Unit, rune: Long, conclusion: String): Unit = {}

*/
// mig: fn instantiate_ancestor_template
fn instantiate_ancestor_template(&self, _descendants: Vec<String>, _ancestor_template: &str) -> String {
    panic!("Unimplemented: instantiate_ancestor_template");
}
/*
  def instantiateAncestorTemplate(descendants: Vector[String], ancestorTemplate: String): String = {
    // IRL, we may want to doublecheck that all descendants *can* instantiate as the ancestor template.
    val descendant = descendants.head
    (descendant, ancestorTemplate) match {
      case (x, y) if x == y => x
      case (x, y) if !x.contains(":") => y
      case ("Flamethrower:int", "IWeapon") => "IWeapon:int"
      case ("Rockets:int", "IWeapon") => "IWeapon:int"
      case other => vimpl(other)
    }
  }

*/
// mig: fn get_ancestors
fn get_ancestors(&self, _descendant: &str, _include_self: bool) -> Vec<String> {
    panic!("Unimplemented: get_ancestors");
}
/*
  def getAncestors(descendant: String, includeSelf: Boolean): Vector[String] = {
    val selfAndAncestors =
      getTemplate(descendant) match {
        case "Firefly" => Vector("ISpaceship")
        case "Serenity" => Vector("ISpaceship")
        case "ISpaceship" => Vector()
        case "Flamethrower" => Vector("IWeapon")
        case "Rockets" => Vector("IWeapon")
        case "IWeapon" => Vector()
        case "int" => Vector()
        case other => vimpl(other)
      }
    selfAndAncestors ++ (if (includeSelf) List(descendant) else List())
  }

*/
// mig: fn get_template
fn get_template(&self, _tyype: &str) -> String {
    panic!("Unimplemented: get_template");
}
/*
  // Turns eg Flamethrower:int into Flamethrower. Firefly just stays Firefly.
  def getTemplate(tyype: String): String = {
    if (tyype.contains(":")) tyype.split(":")(0) else tyype
  }

*/
// mig: fn complex_solve
fn complex_solve<SS, StS>(
    &self,
    _state: &(),
    _env: &(),
    _solver_state: &SS,
    _step_state: &StS,
) -> Result<(), crate::solver::ISolverError<i64, String, String>> {
    panic!("Unimplemented: complex_solve");
}
/*
  override def complexSolve(state: Unit, env: Unit, solverState: ISolverState[IRule, Long, String], stepState: IStepState[IRule, Long, String]): Result[Unit, ISolverError[Long, String, String]] = {
    val unsolvedRules = stepState.getUnsolvedRules()
    val receiverRunes = unsolvedRules.collect({ case Send(_, receiverRune) => receiverRune })
    receiverRunes.foreach(receiver => {
      val receiveRules = unsolvedRules.collect({ case z @ Send(_, r) if r == receiver => z })
      val callRules = unsolvedRules.collect({ case z @ Call(r, _, _) if r == receiver => z })
      val senderConclusions = receiveRules.map(_.senderRune).flatMap(stepState.getConclusion)
      val callTemplates = callRules.map(_.nameRune).flatMap(stepState.getConclusion)
      vassert(callTemplates.distinct.size <= 1)
      // If true, there are some senders/constraints we don't know yet, so lets be
      // careful to not assume between any possibilities below.
      val anyUnknownConstraints =
        (senderConclusions.size != receiveRules.size || callRules.size != callTemplates.size)
      solveReceives(senderConclusions, callTemplates, anyUnknownConstraints) match {
        case None => List()
        case Some(receiverInstantiation) => stepState.concludeRune(List(RangeS.testZero(interner)), receiver, receiverInstantiation)
      }
    })
    Ok(())
  }

*/
// mig: fn solve
fn solve<SS, StS, R: super::test_rules::IRule>(
    &self,
    _state: &(),
    _env: &(),
    _solver_state: &SS,
    _rule_index: i32,
    _rule: &R,
    _step_state: &StS,
) -> Result<(), crate::solver::ISolverError<i64, String, String>> {
    panic!("Unimplemented: solve");
}
/*
  override def solve(state: Unit, env: Unit, solverState: ISolverState[IRule, Long, String], ruleIndex: Int, rule: IRule, stepState: IStepState[IRule, Long, String]): Result[Unit, ISolverError[Long, String, String]] = {
    rule match {
      case Equals(leftRune, rightRune) => {
        stepState.getConclusion(leftRune) match {
          case Some(left) => stepState.concludeRune(List(RangeS.testZero(interner)), rightRune, left); Ok(())
          case None => stepState.concludeRune(List(RangeS.testZero(interner)), leftRune, vassertSome(stepState.getConclusion(rightRune))); Ok(())
        }
      }
      case Lookup(rune, name) => {
        val value = name
        stepState.concludeRune(List(RangeS.testZero(interner)), rune, value)
        Ok(())
      }
      case Literal(rune, literal) => {
        stepState.concludeRune(List(RangeS.testZero(interner)), rune, literal)
        Ok(())
      }
      case OneOf(rune, literals) => {
        val literal = stepState.getConclusion(rune).get
        if (!literals.contains(literal)) {
          return Err(RuleError("conflict!"))
        }
        Ok(())
      }
      case CoordComponents(coordRune, ownershipRune, kindRune) => {
        stepState.getConclusion(coordRune) match {
          case Some(combined) => {
            val Array(ownership, kind) = combined.split("/")
            stepState.concludeRune(List(RangeS.testZero(interner)), ownershipRune, ownership)
            stepState.concludeRune(List(RangeS.testZero(interner)), kindRune, kind)
            Ok(())
          }
          case None => {
            (stepState.getConclusion(ownershipRune), stepState.getConclusion(kindRune)) match {
              case (Some(ownership), Some(kind)) => {
                stepState.concludeRune(List(RangeS.testZero(interner)), coordRune, ownership + "/" + kind)
                Ok(())
              }
              case _ => vfail()
            }
          }
        }
      }
      case Pack(resultRune, memberRunes) => {
        stepState.getConclusion(resultRune) match {
          case Some(result) => {
            val parts = result.split(",")
            memberRunes.zip(parts).foreach({ case (rune, part) =>
              stepState.concludeRune(List(RangeS.testZero(interner)), rune, part)
            })
            Ok(())
          }
          case None => {
            val result = memberRunes.map(stepState.getConclusion).map(_.get).mkString(",")
            stepState.concludeRune(List(RangeS.testZero(interner)), resultRune, result)
            Ok(())
          }
        }
      }
      case Call(resultRune, nameRune, argRune) => {
        val maybeResult = stepState.getConclusion(resultRune)
        val maybeName = stepState.getConclusion(nameRune)
        val maybeArg = stepState.getConclusion(argRune)
        (maybeResult, maybeName, maybeArg) match {
          case (Some(result), Some(templateName), _) => {
            val prefix = templateName + ":"
            vassert(result.startsWith(prefix))
            stepState.concludeRune(List(RangeS.testZero(interner)), argRune, result.slice(prefix.length, result.length))
            Ok(())
          }
          case (_, Some(templateName), Some(arg)) => {
            stepState.concludeRune(List(RangeS.testZero(interner)), resultRune, (templateName + ":" + arg))
            Ok(())
          }
          case other => vwat(other)
        }
      }
      case Send(senderRune, receiverRune) => {
        val receiver = vassertSome(stepState.getConclusion(receiverRune))
        if (receiver == "ISpaceship" || receiver == "IWeapon:int") {
          stepState.addRule(Implements(senderRune, receiverRune))
          Ok(())
        } else {
          // Not receiving into an interface, so sender must be the same
          stepState.concludeRune(List(RangeS.testZero(interner)), senderRune, receiver)
          Ok(())
        }
      }
      case Implements(subRune, superRune) => {
        val sub = vassertSome(stepState.getConclusion(subRune))
        val suuper = vassertSome(stepState.getConclusion(superRune))
        (sub, suuper) match {
          case (x, y) if x == y => Ok(())
          case ("Firefly", "ISpaceship") => Ok(())
          case ("Serenity", "ISpaceship") => Ok(())
          case ("Flamethrower:int", "IWeapon:int") => Ok(())
          case other => vimpl(other)
        }
      }
    }
  }

*/
// mig: fn solve_receives
fn solve_receives(
    &self,
    _senders: Vec<String>,
    _call_templates: Vec<String>,
    _any_unknown_constraints: bool,
) -> Option<String> {
    panic!("Unimplemented: solve_receives");
}
/*
  private def solveReceives(
    senders: Vector[String],
    callTemplates: Vector[String],
    anyUnknownConstraints: Boolean) = {
    val senderTemplates = senders.map(getTemplate)
    // Theoretically possible, not gonna handle it for this test
    vassert(callTemplates.toSet.size <= 1)

    // For example [Flamethrower, Rockets] becomes [[Flamethrower, IWeapon, ISystem], [Rockets, IWeapon, ISystem]]
    val senderAncestorLists = senderTemplates.map(getAncestors(_, true))
    // Calculates the intersection of them all, eg [IWeapon, ISystem]
    val commonAncestors = senderAncestorLists.reduce(_.intersect(_)).toSet
    // Filter by any call templates. eg if there's a X = ISystem:Y call, then we're now [ISystem]
    val commonAncestorsCallConstrained =
      if (callTemplates.isEmpty) commonAncestors else commonAncestors.intersect(callTemplates.toSet)
    // If there are multiple, like [IWeapon, ISystem], get rid of any that are parents of others, now [IWeapon].
    val commonAncestorsNarrowed = narrow(commonAncestorsCallConstrained, anyUnknownConstraints)
    if (commonAncestorsNarrowed.isEmpty) {
      None
    } else {
      val ancestorTemplate = commonAncestorsNarrowed.head
      val ancestorInstantiation = instantiateAncestorTemplate(senders, ancestorTemplate)
      Some(ancestorInstantiation)
    }
  }

*/
// mig: fn narrow
fn narrow(
    &self,
    _ancestor_template_unnarrowed: std::collections::HashSet<String>,
    _any_unknown_constraints: bool,
) -> std::collections::HashSet<String> {
    panic!("Unimplemented: narrow");
}
/*
  def narrow(
    ancestorTemplateUnnarrowed: Set[String],
    anyUnknownConstraints: Boolean):
  Set[String] = {
    val ancestorTemplate =
      if (ancestorTemplateUnnarrowed.size > 1) {
        if (anyUnknownConstraints) {
          // Theres some unknown constraints (calls, receives, isa, etc)
          // so we can't yet conclude what the narrowest one is.
          vfail()
        } else {
          // Then choose the narrowest one.
          // For our particular test data sets, this shortcut should work.
          ancestorTemplateUnnarrowed - "ISpaceship" - "IWeapon"
        }
      } else {
        ancestorTemplateUnnarrowed
      }
    vassert(ancestorTemplate.size <= 1)
    ancestorTemplate
  }

}
*/
}
