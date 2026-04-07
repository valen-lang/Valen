package dev.vale.solver

import dev.vale.{Err, Interner, Ok, RangeS, Result, vassert, vassertSome, vfail, vimpl, vwat}
import org.scalatest._

import scala.collection.immutable.Map

class TestRuleSolver(interner: Interner) extends ISolveRule[IRule, Long, Unit, Unit, String, String] {
  override def sanityCheckConclusion(env: Unit, state: Unit, rune: Long, conclusion: String): Unit = {}

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

  // Turns eg Flamethrower:int into Flamethrower. Firefly just stays Firefly.
  def getTemplate(tyype: String): String = {
    if (tyype.contains(":")) tyype.split(":")(0) else tyype
  }

  override def complexSolve(state: Unit, env: Unit, solverState: SimpleSolverState[IRule, Long, String]): Result[Unit, ISolverError[Long, String, String]] = {
    val unsolvedRules = solverState.getUnsolvedRules()
    val receiverRunes = unsolvedRules.collect({ case Send(_, receiverRune) => receiverRune })
    receiverRunes.foreach(receiver => {
      val receiveRules = unsolvedRules.collect({ case z @ Send(_, r) if r == receiver => z })
      val callRules = unsolvedRules.collect({ case z @ Call(r, _, _) if r == receiver => z })
      val senderConclusions = receiveRules.map(_.senderRune).flatMap(solverState.getConclusion)
      val callTemplates = callRules.map(_.nameRune).flatMap(solverState.getConclusion)
      vassert(callTemplates.distinct.size <= 1)
      // If true, there are some senders/constraints we don't know yet, so lets be
      // careful to not assume between any possibilities below.
      val anyUnknownConstraints =
        (senderConclusions.size != receiveRules.size || callRules.size != callTemplates.size)
      solveReceives(senderConclusions, callTemplates, anyUnknownConstraints) match {
        case None => List()
        case Some(receiverInstantiation) => solverState.concludeRune[String](receiver, receiverInstantiation) match { case Ok(_) => case Err(e) => return Err(e) }
      }
    })
    Ok(())
  }

  override def solve(state: Unit, env: Unit, solverState: SimpleSolverState[IRule, Long, String], ruleIndex: Int, rule: IRule): Result[Unit, ISolverError[Long, String, String]] = {
    rule match {
      case Equals(leftRune, rightRune) => {
        solverState.getConclusion(leftRune) match {
          case Some(left) => {
            solverState.concludeRune[String](rightRune, left) match { case Ok(_) => case Err(e) => return Err(e) }
          }
          case None => {
            solverState.concludeRune[String](leftRune, vassertSome(solverState.getConclusion(rightRune))) match { case Ok(_) => case Err(e) => return Err(e) }
          }
        }
        Ok(())
      }
      case Lookup(rune, name) => {
        val value = name
        solverState.concludeRune[String](rune, value) match { case Ok(_) => case Err(e) => return Err(e) }
        Ok(())
      }
      case Literal(rune, literal) => {
        solverState.concludeRune[String](rune, literal) match { case Ok(_) => case Err(e) => return Err(e) }
        Ok(())
      }
      case OneOf(rune, literals) => {
        val literal = solverState.getConclusion(rune).get
        if (!literals.contains(literal)) {
          return Err(RuleError("conflict!"))
        }
        Ok(())
      }
      case CoordComponents(coordRune, ownershipRune, kindRune) => {
        solverState.getConclusion(coordRune) match {
          case Some(combined) => {
            val Array(ownership, kind) = combined.split("/")
            solverState.concludeRune[String](ownershipRune, ownership) match { case Ok(_) => case Err(e) => return Err(e) }
            solverState.concludeRune[String](kindRune, kind) match { case Ok(_) => case Err(e) => return Err(e) }
            Ok(())
          }
          case None => {
            (solverState.getConclusion(ownershipRune), solverState.getConclusion(kindRune)) match {
              case (Some(ownership), Some(kind)) => {
                solverState.concludeRune[String](coordRune, ownership + "/" + kind) match { case Ok(_) => case Err(e) => return Err(e) }
                Ok(())
              }
              case _ => vfail()
            }
          }
        }
      }
      case Pack(resultRune, memberRunes) => {
        solverState.getConclusion(resultRune) match {
          case Some(result) => {
            val parts = result.split(",")
            memberRunes.zip(parts).foreach({ case (rune, part) =>
              solverState.concludeRune[String](rune, part) match { case Ok(_) => case Err(e) => return Err(e) }
            })
            Ok(())
          }
          case None => {
            val result = memberRunes.map(solverState.getConclusion).map(_.get).mkString(",")
            solverState.concludeRune[String](resultRune, result) match { case Ok(_) => case Err(e) => return Err(e) }
            Ok(())
          }
        }
      }
      case Call(resultRune, nameRune, argRune) => {
        val maybeResult = solverState.getConclusion(resultRune)
        val maybeName = solverState.getConclusion(nameRune)
        val maybeArg = solverState.getConclusion(argRune)
        (maybeResult, maybeName, maybeArg) match {
          case (Some(result), Some(templateName), _) => {
            val prefix = templateName + ":"
            vassert(result.startsWith(prefix))
            solverState.concludeRune[String](argRune, result.slice(prefix.length, result.length)) match { case Ok(_) => case Err(e) => return Err(e) }
            Ok(())
          }
          case (_, Some(templateName), Some(arg)) => {
            solverState.concludeRune[String](resultRune, (templateName + ":" + arg)) match { case Ok(_) => case Err(e) => return Err(e) }
            Ok(())
          }
          case other => vwat(other)
        }
      }
      case Send(senderRune, receiverRune) => {
        val receiver = vassertSome(solverState.getConclusion(receiverRune))
        if (receiver == "ISpaceship" || receiver == "IWeapon:int") {
          solverState.addRule(Implements(senderRune, receiverRune))
          vimpl()
          Ok(())
        } else {
          // Not receiving into an interface, so sender must be the same
          solverState.concludeRune[String](senderRune, receiver) match { case Ok(_) => case Err(e) => return Err(e) }
          Ok(())
        }
      }
      case Implements(subRune, superRune) => {
        val sub = vassertSome(solverState.getConclusion(subRune))
        val suuper = vassertSome(solverState.getConclusion(superRune))
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
