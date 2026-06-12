package dev.vale.solver

import dev.vale.{Err, Ok, RangeS, Result, vassert, vassertSome, vfail, vimpl, vwat}

import scala.collection.immutable.Map

object TestRuleSolver {
  def sanityCheckConclusionInner(env: Unit, state: Unit, rune: Long, conclusion: String): Unit = {}

  // Per @CSCDSRZ, this only concludes runes — it doesn't mark any rules as solved.
  def complexSolveInner(state: Unit, env: Unit, solverState: SimpleSolverState[IRule, Long, String]): Result[Unit, ISolverError[Long, String, String]] = {
    val unsolvedRules = solverState.getUnsolvedRules()
    val receiverRunes = unsolvedRules.collect({ case Send(_, receiverRune) => receiverRune })
    val newConclusions =
      receiverRunes.flatMap(receiver => {
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
          case Some(receiverInstantiation) => List(receiver -> receiverInstantiation)
        }
      }).toMap
    solverState.commitStep[String](true, Vector(), newConclusions, Vector(), Set.empty) match { case Ok(_) => case Err(e) => return Err(e) }
    Ok(())
  }

  def solveInner(state: Unit, env: Unit, solverState: SimpleSolverState[IRule, Long, String], ruleIndex: Int, rule: IRule): Result[Unit, ISolverError[Long, String, String]] = {
    rule match {
      case Equals(leftRune, rightRune) => {
        solverState.getConclusion(leftRune) match {
          case Some(left) => {
            //            solverState.commitStep[String](rightRune, left) match { case Ok(_) => case Err(e) => return Err(e) }
            solverState.commitStep[String](false, Vector(ruleIndex), Map(rightRune -> left), Vector(), Set.empty)
          }
          case None => {
            solverState.commitStep[String](false, Vector(ruleIndex), Map(leftRune -> vassertSome(solverState.getConclusion(rightRune))), Vector(), Set.empty)
          }
        }
      }
      case Lookup(rune, name) => {
        val value = name
        solverState.commitStep[String](false, Vector(ruleIndex), Map(rune -> value), Vector(), Set.empty)
      }
      case Literal(rune, literal) => {
        solverState.commitStep[String](false, Vector(ruleIndex), Map(rune -> literal), Vector(), Set.empty)
      }
      case OneOf(rune, literals) => {
        val literal = solverState.getConclusion(rune).get
        if (!literals.contains(literal)) {
          return Err(RuleError("conflict!"))
        }
        solverState.commitStep[String](false, Vector(ruleIndex), Map(), Vector(), Set.empty)
      }
      case CoordComponents(coordRune, ownershipRune, kindRune) => {
        solverState.getConclusion(coordRune) match {
          case Some(combined) => {
            val Array(ownership, kind) = combined.split("/")
            solverState.commitStep[String](false, Vector(ruleIndex), Map(ownershipRune -> ownership, kindRune -> kind), Vector(), Set.empty)
          }
          case None => {
            (solverState.getConclusion(ownershipRune), solverState.getConclusion(kindRune)) match {
              case (Some(ownership), Some(kind)) => {
                solverState.commitStep[String](false, Vector(ruleIndex), Map(coordRune -> (ownership + "/" + kind)), Vector(), Set.empty)
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
            solverState.commitStep[String](false, Vector(ruleIndex), memberRunes.zip(parts).toMap, Vector(), Set.empty)
          }
          case None => {
            val result = memberRunes.map(solverState.getConclusion).map(_.get).mkString(",")
            solverState.commitStep[String](false, Vector(ruleIndex), Map(resultRune -> result), Vector(), Set.empty)
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
            solverState.commitStep[String](false, Vector(ruleIndex), Map(argRune -> result.slice(prefix.length, result.length)), Vector(), Set.empty)
          }
          case (_, Some(templateName), Some(arg)) => {
            solverState.commitStep[String](false, Vector(ruleIndex), Map(resultRune -> (templateName + ":" + arg)), Vector(), Set.empty)
          }
          case other => vwat(other)
        }
      }
      case Send(senderRune, receiverRune) => {
        val receiver = vassertSome(solverState.getConclusion(receiverRune))
        if (receiver == "ISpaceship" || receiver == "IWeapon:int") {
          solverState.commitStep[String](false, Vector(ruleIndex), Map(), Vector(Implements(senderRune, receiverRune)), Set.empty)
        } else {
          // Not receiving into an interface, so sender must be the same
          solverState.commitStep[String](false, Vector(ruleIndex), Map(senderRune -> receiver), Vector(), Set.empty)
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
        solverState.commitStep[String](false, Vector(ruleIndex), Map(), Vector(), Set.empty)
      }
    }
  }

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

