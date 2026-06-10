/*
package dev.vale.solver

import dev.vale.{Err, Ok, RangeS, Result, vassert, vassertSome, vfail, vimpl, vwat}

import scala.collection.immutable.Map

*/
use super::test_rules::*;
use crate::solver::{ISolverError, RuleError, SimpleSolverState};
use crate::scout_arena::ScoutArena;
use std::collections::HashMap;
use std::collections::HashSet;
use std::marker::PhantomData;

// mig: struct TestRuleSolver
pub struct TestRuleSolver<'ctx, 's> {
    pub scout_arena: &'ctx ScoutArena<'s>,
}
/*
object TestRuleSolver {
*/

// mig: impl TestRuleSolver
impl<'ctx, 's> TestRuleSolver<'ctx, 's> {
/*
*/
/*
  def sanityCheckConclusionInner(env: Unit, state: Unit, rune: Long, conclusion: String): Unit = {}

*/
// mig: fn complex_solve_impl
pub fn complex_solve_impl(
    &self,
    _state: &(),
    _env: &(),
    solver_state: &mut SimpleSolverState<TestRule, i64, String>,
) -> Result<(), ISolverError<i64, String, String>> {
    let unsolved_rules = solver_state.get_unsolved_rules();
    let receiver_runes: Vec<i64> = {
        let mut v: Vec<i64> = unsolved_rules
            .iter()
            .filter_map(|r| {
                if let TestRule::Send(Send { receiver_rune, .. }) = r {
                    Some(*receiver_rune)
                } else {
                    None
                }
            })
            .collect();
        v.sort();
        v.dedup();
        v
    };
    let mut new_conclusions: HashMap<i64, String> = HashMap::new();
    for receiver in receiver_runes {
        let receive_rules: Vec<&TestRule> = unsolved_rules
            .iter()
            .filter(|r| {
                if let TestRule::Send(Send { receiver_rune, .. }) = r {
                    *receiver_rune == receiver
                } else {
                    false
                }
            })
            .collect();
        let call_rules: Vec<&TestRule> = unsolved_rules
            .iter()
            .filter(|r| {
                if let TestRule::Call(Call { result_rune, .. }) = r {
                    *result_rune == receiver
                } else {
                    false
                }
            })
            .collect();
        let sender_conclusions: Vec<String> = receive_rules
            .iter()
            .filter_map(|r| {
                if let TestRule::Send(s) = r {
                    solver_state.get_conclusion(&s.sender_rune)
                } else {
                    None
                }
            })
            .collect();
        let call_templates: Vec<String> = call_rules
            .iter()
            .filter_map(|r| {
                if let TestRule::Call(c) = r {
                    solver_state.get_conclusion(&c.name_rune)
                } else {
                    None
                }
            })
            .collect();
        let any_unknown_constraints = sender_conclusions.len() != receive_rules.len()
            || call_rules.len() != call_templates.len();
        if let Some(receiver_instantiation) = self.solve_receives(
            sender_conclusions,
            call_templates,
            any_unknown_constraints,
        )
        {
            new_conclusions.insert(receiver, receiver_instantiation);
        }
    }
    // Complex solve only produces conclusions, not solved/new rules.
    solver_state.commit_step::<String>(true, vec![], new_conclusions, vec![], HashSet::new())?;
    Ok(())
}
/*
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
    solverState.commitStep[String](true, Vector(), newConclusions, Vector()) match { case Ok(_) => case Err(e) => return Err(e) }
    Ok(())
  }

*/
// mig: fn solve_impl
pub fn solve_impl(
  &self,
  _state: &(),
  _env: &(),
  solver_state: &mut SimpleSolverState<TestRule, i64, String>,
  rule_index: i32,
  rule: &TestRule,
) -> Result<(), ISolverError<i64, String, String>> {
    match rule {
        TestRule::Equals(Equals { left_rune, right_rune }) => {
            match solver_state.get_conclusion(left_rune) {
                Some(left) => {
                    solver_state.commit_step::<String>(false, vec![rule_index],[(*right_rune, left)].into_iter().collect(), vec![], HashSet::new())
                }
                None => {
                    let right = solver_state
                        .get_conclusion(right_rune)
                        .expect("right rune must have conclusion");
                    solver_state.commit_step::<String>(false, vec![rule_index],[(*left_rune, right)].into_iter().collect(), vec![], HashSet::new())
                }
            }
        }
        TestRule::Lookup(Lookup { rune, name }) => {
            solver_state.commit_step::<String>(false, vec![rule_index],[(*rune, name.clone())].into_iter().collect(), vec![], HashSet::new())
        }
        TestRule::Literal(Literal { rune, value }) => {
            solver_state.commit_step::<String>(false, vec![rule_index],[(*rune, value.clone())].into_iter().collect(), vec![], HashSet::new())
        }
        TestRule::OneOf(OneOf { coord_rune, possible_values }) => {
            let literal = solver_state
                .get_conclusion(coord_rune)
                .expect("OneOf rune must have conclusion");
            if !possible_values.contains(&literal) {
                return Err(ISolverError::RuleError(RuleError {
                    err: "conflict!".to_string(),
                    _phantom: PhantomData,
                }));
            }
            solver_state.commit_step::<String>(false, vec![rule_index],HashMap::new(), vec![], HashSet::new())
        }
        TestRule::CoordComponents(CoordComponents {
            coord_rune,
            ownership_rune,
            kind_rune,
        }) => {
            match solver_state.get_conclusion(coord_rune) {
                Some(combined) => {
                    let parts: Vec<&str> = combined.split('/').collect();
                    let (ownership, kind) = (parts[0].to_string(), parts[1].to_string());
                    solver_state.commit_step::<String>(false, vec![rule_index],[(*ownership_rune, ownership), (*kind_rune, kind)].into_iter().collect(), vec![], HashSet::new())
                }
                None => {
                    let ownership = solver_state
                        .get_conclusion(ownership_rune)
                        .expect("ownership required");
                    let kind = solver_state
                        .get_conclusion(kind_rune)
                        .expect("kind required");
                    let combined = format!("{}/{}", ownership, kind);
                    solver_state.commit_step::<String>(false, vec![rule_index],[(*coord_rune, combined)].into_iter().collect(), vec![], HashSet::new())
                }
            }
        }
        TestRule::Pack(Pack {
            result_rune,
            member_runes,
        }) => {
            match solver_state.get_conclusion(result_rune) {
                Some(result) => {
                    let parts: Vec<&str> = result.split(',').collect();
                    let conclusions: HashMap<i64, String> = member_runes.iter().zip(parts.iter())
                        .map(|(rune, part)| (*rune, (*part).to_string()))
                        .collect();
                    solver_state.commit_step::<String>(false, vec![rule_index],conclusions, vec![], HashSet::new())
                }
                None => {
                    let result: String = member_runes
                        .iter()
                        .map(|r| {
                            solver_state
                                .get_conclusion(r)
                                .expect("member rune must have conclusion")
                        })
                        .collect::<Vec<_>>()
                        .join(",");
                    solver_state.commit_step::<String>(false, vec![rule_index],[(*result_rune, result)].into_iter().collect(), vec![], HashSet::new())
                }
            }
        }
        TestRule::Call(Call {
            result_rune,
            name_rune,
            arg_rune,
        }) => {
            let maybe_result = solver_state.get_conclusion(result_rune);
            let maybe_name = solver_state.get_conclusion(name_rune);
            let maybe_arg = solver_state.get_conclusion(arg_rune);
            match (maybe_result, maybe_name, maybe_arg) {
                (Some(result), Some(template_name), _) => {
                    let prefix = format!("{}:", template_name);
                    assert!(result.starts_with(&prefix));
                    let arg = result[prefix.len()..].to_string();
                    solver_state.commit_step::<String>(false, vec![rule_index],[(*arg_rune, arg)].into_iter().collect(), vec![], HashSet::new())
                }
                (_, Some(template_name), Some(arg)) => {
                    let result = format!("{}:{}", template_name, arg);
                    solver_state.commit_step::<String>(false, vec![rule_index],[(*result_rune, result)].into_iter().collect(), vec![], HashSet::new())
                }
                _ => panic!("Call rule needs name+arg or result+name"),
            }
        }
        TestRule::Send(Send {
            sender_rune,
            receiver_rune,
        }) => {
            let receiver = solver_state
                .get_conclusion(receiver_rune)
                .expect("receiver must have conclusion");
            if receiver == "ISpaceship" || receiver == "IWeapon:int" {
                let new_rule = TestRule::Implements(Implements {
                    sub_rune: *sender_rune,
                    super_rune: *receiver_rune,
                });
                solver_state.commit_step::<String>(false, vec![rule_index],HashMap::new(), vec![new_rule], HashSet::new())
            } else {
                solver_state.commit_step::<String>(false, vec![rule_index],[(*sender_rune, receiver)].into_iter().collect(), vec![], HashSet::new())
            }
        }
        TestRule::Implements(Implements { sub_rune, super_rune }) => {
            let sub = solver_state
                .get_conclusion(sub_rune)
                .expect("sub must have conclusion");
            let suuper = solver_state
                .get_conclusion(super_rune)
                .expect("super must have conclusion");
            match (sub.as_str(), suuper.as_str()) {
                (x, y) if x == y => {},
                ("Firefly", "ISpaceship") => {},
                ("Serenity", "ISpaceship") => {},
                ("Flamethrower:int", "IWeapon:int") => {},
                _ => panic!("Unimplemented Implements case: {} -> {}", sub, suuper),
            }
            solver_state.commit_step::<String>(false, vec![rule_index],HashMap::new(), vec![], HashSet::new())
        }
    }
}
/*
  def solveInner(state: Unit, env: Unit, solverState: SimpleSolverState[IRule, Long, String], ruleIndex: Int, rule: IRule): Result[Unit, ISolverError[Long, String, String]] = {
    rule match {
      case Equals(leftRune, rightRune) => {
        solverState.getConclusion(leftRune) match {
          case Some(left) => {
            //            solverState.commitStep[String](rightRune, left) match { case Ok(_) => case Err(e) => return Err(e) }
            solverState.commitStep[String](false, Vector(ruleIndex), Map(rightRune -> left), Vector())
          }
          case None => {
            solverState.commitStep[String](false, Vector(ruleIndex), Map(leftRune -> vassertSome(solverState.getConclusion(rightRune))), Vector())
          }
        }
      }
      case Lookup(rune, name) => {
        val value = name
        solverState.commitStep[String](false, Vector(ruleIndex), Map(rune -> value), Vector())
      }
      case Literal(rune, literal) => {
        solverState.commitStep[String](false, Vector(ruleIndex), Map(rune -> literal), Vector())
      }
      case OneOf(rune, literals) => {
        val literal = solverState.getConclusion(rune).get
        if (!literals.contains(literal)) {
          return Err(RuleError("conflict!"))
        }
        solverState.commitStep[String](false, Vector(ruleIndex), Map(), Vector())
      }
      case CoordComponents(coordRune, ownershipRune, kindRune) => {
        solverState.getConclusion(coordRune) match {
          case Some(combined) => {
            val Array(ownership, kind) = combined.split("/")
            solverState.commitStep[String](false, Vector(ruleIndex), Map(ownershipRune -> ownership, kindRune -> kind), Vector())
          }
          case None => {
            (solverState.getConclusion(ownershipRune), solverState.getConclusion(kindRune)) match {
              case (Some(ownership), Some(kind)) => {
                solverState.commitStep[String](false, Vector(ruleIndex), Map(coordRune -> (ownership + "/" + kind)), Vector())
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
            solverState.commitStep[String](false, Vector(ruleIndex), memberRunes.zip(parts).toMap, Vector())
          }
          case None => {
            val result = memberRunes.map(solverState.getConclusion).map(_.get).mkString(",")
            solverState.commitStep[String](false, Vector(ruleIndex), Map(resultRune -> result), Vector())
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
            solverState.commitStep[String](false, Vector(ruleIndex), Map(argRune -> result.slice(prefix.length, result.length)), Vector())
          }
          case (_, Some(templateName), Some(arg)) => {
            solverState.commitStep[String](false, Vector(ruleIndex), Map(resultRune -> (templateName + ":" + arg)), Vector())
          }
          case other => vwat(other)
        }
      }
      case Send(senderRune, receiverRune) => {
        val receiver = vassertSome(solverState.getConclusion(receiverRune))
        if (receiver == "ISpaceship" || receiver == "IWeapon:int") {
          solverState.commitStep[String](false, Vector(ruleIndex), Map(), Vector(Implements(senderRune, receiverRune)))
        } else {
          // Not receiving into an interface, so sender must be the same
          solverState.commitStep[String](false, Vector(ruleIndex), Map(senderRune -> receiver), Vector())
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
        solverState.commitStep[String](false, Vector(ruleIndex), Map(), Vector())
      }
    }
  }

*/
// mig: fn instantiate_ancestor_template
fn instantiate_ancestor_template(&self, descendants: Vec<String>, ancestor_template: &str) -> String {
    let descendant = descendants.first().expect("descendants non-empty");
    match (descendant.as_str(), ancestor_template) {
        (x, y) if x == y => descendant.clone(),
        (x, y) if !x.contains(':') => y.to_string(),
        ("Flamethrower:int", "IWeapon") => "IWeapon:int".to_string(),
        ("Rockets:int", "IWeapon") => "IWeapon:int".to_string(),
        other => panic!("Unimplemented instantiate_ancestor_template: {:?}", other),
    }
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
fn get_ancestors(&self, descendant: &str, include_self: bool) -> Vec<String> {
    let self_and_ancestors: Vec<String> = match self.get_template(descendant).as_str() {
        "Firefly" => vec!["ISpaceship".to_string()],
        "Serenity" => vec!["ISpaceship".to_string()],
        "ISpaceship" => vec![],
        "Flamethrower" => vec!["IWeapon".to_string()],
        "Rockets" => vec!["IWeapon".to_string()],
        "IWeapon" => vec![],
        "int" => vec![],
        other => panic!("Unimplemented get_ancestors: {}", other),
    };
    let mut result = self_and_ancestors;
    if include_self {
        result.push(descendant.to_string());
    }
    result
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
fn get_template(&self, tyype: &str) -> String {
    if tyype.contains(':') {
        tyype.split(':').next().unwrap_or(tyype).to_string()
    } else {
        tyype.to_string()
    }
}
/*
  // Turns eg Flamethrower:int into Flamethrower. Firefly just stays Firefly.
  def getTemplate(tyype: String): String = {
    if (tyype.contains(":")) tyype.split(":")(0) else tyype
  }

*/
// mig: fn solve_receives
fn solve_receives(
    &self,
    senders: Vec<String>,
    call_templates: Vec<String>,
    any_unknown_constraints: bool,
) -> Option<String> {
    let sender_templates: Vec<String> = senders.iter().map(|s| self.get_template(s)).collect();
    assert!(call_templates.iter().collect::<HashSet<_>>().len() <= 1);
    let sender_ancestor_lists: Vec<Vec<String>> = sender_templates
        .iter()
        .map(|s| self.get_ancestors(s, true))
        .collect();
    let common_ancestors: HashSet<String> = sender_ancestor_lists
        .iter()
        .fold(None, |acc: Option<HashSet<String>>, list| {
            let set: HashSet<String> = list.iter().cloned().collect();
            Some(match acc {
                None => set,
                Some(a) => a.intersection(&set).cloned().collect(),
            })
        })
        .unwrap_or_default();
    let call_templates_set: HashSet<String> =
        call_templates.iter().cloned().collect();
    let common_ancestors_call_constrained = if call_templates_set.is_empty() {
        common_ancestors
    } else {
        common_ancestors
            .intersection(&call_templates_set)
            .cloned()
            .collect()
    };
    let common_ancestors_narrowed =
        self.narrow(common_ancestors_call_constrained, any_unknown_constraints);
    if common_ancestors_narrowed.is_empty() {
        None
    } else {
        let ancestor_template = common_ancestors_narrowed
            .iter()
            .next()
            .expect("non-empty");
        Some(self.instantiate_ancestor_template(senders, ancestor_template))
    }
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
    ancestor_template_unnarrowed: HashSet<String>,
    any_unknown_constraints: bool,
) -> HashSet<String> {
    let ancestor_template = if ancestor_template_unnarrowed.len() > 1 {
        if any_unknown_constraints {
            panic!("narrow: any_unknown_constraints with multiple ancestors");
        } else {
            ancestor_template_unnarrowed
                .into_iter()
                .filter(|x| *x != "ISpaceship" && *x != "IWeapon")
                .collect()
        }
    } else {
        ancestor_template_unnarrowed
    };
    assert!(ancestor_template.len() <= 1);
    ancestor_template
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

// mig: fn rule_to_puzzles (free function for use with make_solver_state)
pub fn rule_to_puzzles(rule: &TestRule) -> Vec<Vec<i64>> {
    rule.all_puzzles()
}
