/*
package dev.vale.solver

import dev.vale.{Collector, Err, Interner, Ok, RangeS, Result, vassert, vfail}
import org.scalatest._

import scala.collection.immutable.Map

class SolverTests extends FunSuite with Matchers with Collector {
*/
use crate::solver::{SimpleSolverState, FailedSolve, ISolverError, make_solver_state};
use super::test_rules::TestRule;
use std::collections::HashMap;
use std::collections::HashSet;
use crate::utils::range::RangeS;
use bumpalo::Bump;
use super::test_rules::Literal;
use super::test_rules::Equals;
use super::test_rules::OneOf;
use super::test_rules::CoordComponents;
use super::test_rules::Pack;
use super::test_rules::Send;
use super::test_rules::Call;
use super::test_rules::Lookup;
use crate::scout_arena::ScoutArena;
// mig: const complex_rule_set
const COMPLEX_RULE_SET_RULES: Vec<()> = vec![];
/*
  val complexRuleSet =
    Vector(
      Literal(-3L, "1448"),
      CoordComponents(-6L, -5L, -5L),
      Literal(-2L, "1337"),
      Equals(-4L, -2L),
      OneOf(-4L, Vector("1337", "73")),
      Equals(-1L, -5L),
      CoordComponents(-1L, -2L, -3L),
      Equals(-6L, -7L))
*/
// mig: const complex_rule_set_equals_rules
const COMPLEX_RULE_SET_EQUALS_RULES: Vec<i32> = vec![];
/*
  val complexRuleSetEqualsRules = Vector(3, 5, 7)
*/
/*
*/
// mig: fn test_simple_and_optimized
    fn test_simple_and_optimized() {
        panic!("Unimplemented: test_simple_and_optimized");
    }
/*
  def testSimpleAndOptimized(testName: String, testTags : org.scalatest.Tag*)(testFun : Boolean => scala.Any)(implicit pos : org.scalactic.source.Position) : scala.Unit = {
    test(testName + " (simple solver)", testTags: _*){ testFun(false) }(pos)
    test(testName + " (optimized solver)", testTags: _*){ testFun(true) }(pos)
  }

*/
// Local advance helper, inlined from the former generic Solver.advance.
// Returns true if there's more to be done, false if we've gotten as far as we can.
fn advance(
    solver_state: &mut SimpleSolverState<TestRule, i64, String>,
    solve_rule: &super::test_rule_solver::TestRuleSolver,
) -> Result<bool, FailedSolve<TestRule, i64, String, String>> {
    solver_state.sanity_check();
    // Stage 1: simple solve
    match solver_state.get_next_solvable() {
        None => {} // continue onto complex solve
        Some(rule_index) => {
            let rule = solver_state.get_rule(rule_index).clone();
            let steps_before = solver_state.get_steps().len();
            match solve_rule.solve_impl(&(), &(), solver_state, rule_index, &rule) {
                Ok(()) => {}
                Err(e) => return Err(FailedSolve {
                    steps: solver_state.get_steps(),
                    conclusions: solver_state.get_conclusions().into_iter().collect(),
                    unsolved_rules: solver_state.get_unsolved_rules(),
                    unsolved_runes: solver_state.get_unsolved_runes(),
                    error: e,
                }),
            }
            let steps_after = solver_state.get_steps().len();
            assert!(steps_after == steps_before + 1);
            assert!(solver_state.rule_is_solved(rule_index));
            solver_state.sanity_check();
            return Ok(true);
        }
    }
    // Stage 2: complex solve
    if !solver_state.get_unsolved_rules().is_empty() {
        let conclusions_before = solver_state.get_conclusions().len();
        match solve_rule.complex_solve_impl(&(), &(), solver_state) {
            Ok(()) => {}
            Err(e) => return Err(FailedSolve {
                steps: solver_state.get_steps(),
                conclusions: solver_state.get_conclusions().into_iter().collect(),
                unsolved_rules: solver_state.get_unsolved_rules(),
                unsolved_runes: solver_state.get_unsolved_runes(),
                error: e,
            }),
        }
        solver_state.sanity_check();
        let conclusions_after = solver_state.get_conclusions().len();
        if conclusions_after > conclusions_before {
            return Ok(true);
        }
    }
    Ok(false)
}
/*
  // Local advance helper. This shows how one would normally interact with the solver state.
  // Returns true if there's more to be done, false if we've gotten as far as we can.
  def advance(
      solverState: SimpleSolverState[IRule, Long, String]):
  Result[Boolean, FailedSolve[IRule, Long, String, String]] = {
    solverState.sanityCheck()
    solverState.userifyConclusions().foreach({ case (rune, conclusion) =>
      TestRuleSolver.sanityCheckConclusionInner(Unit, Unit, rune, conclusion)
    })
    // Stage 1: Do simple solves
    solverState.getNextSolvable() match {
      case None => // continue onto the next stage
      case Some(solvingRuleIndex) => {
        val rule = solverState.getRule(solvingRuleIndex)
        val stepsBefore = solverState.getSteps().size
        TestRuleSolver.solveInner(Unit, Unit, solverState, solvingRuleIndex, rule) match {
          case Ok(()) => {}
          case Err(e) => return Err(FailedSolve(solverState.getSteps(), solverState.getConclusions().toMap, solverState.getUnsolvedRules(), solverState.getUnsolvedRunes(), e))
        }
        val stepsAfter = solverState.getSteps().size
        vassert(stepsAfter == stepsBefore + 1)
        vassert(solverState.ruleIsSolved(solvingRuleIndex)) // Per @CSCDSRZ, only true after simple solve.
        solverState.sanityCheck()
        // Go back to the beginning. Next step, if there's no simple rule ready to solve, then
        // it'll start doing a complex solve if available, or just finish.
        return Ok(true)
      }
    }
    // Stage 2: Do a complex solve if available.
    // Per @CSCDSRZ, complex solve only adds conclusions — we check conclusion count for progress.
    if (solverState.getUnsolvedRules().nonEmpty) {
      val conclusionsBefore = solverState.getConclusions().toMap.size
      TestRuleSolver.complexSolveInner(Unit, Unit, solverState) match {
        case Ok(()) =>
        case Err(e) => return Err(FailedSolve(solverState.getSteps(), solverState.getConclusions().toMap, solverState.getUnsolvedRules(), solverState.getUnsolvedRunes(), e))
      }
      solverState.sanityCheck()
      val conclusionsAfter = solverState.getConclusions().toMap.size
      // Per @CSCDSRZ, check conclusion count (not rules solved) for progress.
      if (conclusionsAfter == conclusionsBefore) {
        // There's nothing more to be done. Let's continue on to stage 3.
      } else {
        return Ok(true) // Go back to stage 1 where the new conclusions may unblock simple solves.
      }
    } else {
      // No more rules to solve, so continue to the wrapping up stages of the solve.
    }
    // Stage 3: We're done! The user should look at the conclusions to see if they're all solved,
    // and they can even add more rules if they want.
    Ok(false)
  }
*/
// mig: fn simple_int_rule
    #[test]
    fn simple_int_rule() {

        let rules: Vec<TestRule> = vec![
            TestRule::Literal(Literal { rune: -1, value: "1337".to_string() }),
        ];
        let result = get_conclusions(rules, true, HashMap::new());
        let expected: HashMap<i64, String> = [(-1, "1337".to_string())].into_iter().collect();
        assert_eq!(result, expected);
    }
/*
  test("Simple int rule") {
    val rules =
      Vector(
        Literal(-1L, "1337"))
    getConclusions(rules, true) shouldEqual Map(-1L -> "1337")
  }
*/
// mig: fn equals_transitive
    #[test]
    fn equals_transitive() {

        let rules: Vec<TestRule> = vec![
            TestRule::Equals(Equals {
                left_rune: -2,
                right_rune: -1,
            }),
            TestRule::Literal(Literal {
                rune: -1,
                value: "1337".to_string(),
            }),
        ];
        let result = get_conclusions(rules, true, HashMap::new());
        let expected: HashMap<i64, String> =
            [(-1, "1337".to_string()), (-2, "1337".to_string())].into_iter().collect();
        assert_eq!(result, expected);
    }
/*
  test("Equals transitive") {
    val rules =
      Vector(
        Equals(-2L, -1L),
        Literal(-1L, "1337"))
    getConclusions(rules, true) shouldEqual
      Map(-1L -> "1337", -2L -> "1337")
  }
*/
// mig: fn incomplete_solve
    #[test]
    fn incomplete_solve() {

        let rules: Vec<TestRule> = vec![TestRule::OneOf(OneOf {
            coord_rune: -1,
            possible_values: vec!["1448".to_string(), "1337".to_string()],
        })];
        let result = get_conclusions(rules, false, HashMap::new());
        let expected: HashMap<i64, String> = HashMap::new();
        assert_eq!(result, expected);
    }
/*
  test("Incomplete solve") {
    val rules =
      Vector(
        OneOf(-1L, Vector("1448", "1337")))
    getConclusions(rules, false) shouldEqual Map()
  }
*/
// mig: fn half_complete_solve
    #[test]
    fn half_complete_solve() {

        let rules: Vec<TestRule> = vec![
            TestRule::OneOf(OneOf {
                coord_rune: -1,
                possible_values: vec!["1448".to_string(), "1337".to_string()],
            }),
            TestRule::Literal(Literal {
                rune: -2,
                value: "1337".to_string(),
            }),
        ];
        let result = get_conclusions(rules, false, HashMap::new());
        let expected: HashMap<i64, String> =
            [(-2, "1337".to_string())].into_iter().collect();
        assert_eq!(result, expected);
    }
/*
  test("Half-complete solve") {
    // Note how these two rules aren't connected to each other at all
    val rules =
      Vector(
        OneOf(-1L, Vector("1448", "1337")),
        Literal(-2L, "1337"))
    getConclusions(rules, false) shouldEqual Map(-2L -> "1337")
  }
*/
// mig: fn one_of
    #[test]
    fn one_of() {

        let rules: Vec<TestRule> = vec![
            TestRule::OneOf(OneOf {
                coord_rune: -1,
                possible_values: vec!["1448".to_string(), "1337".to_string()],
            }),
            TestRule::Literal(Literal {
                rune: -1,
                value: "1337".to_string(),
            }),
        ];
        let result = get_conclusions(rules, true, HashMap::new());
        let expected: HashMap<i64, String> =
            [(-1, "1337".to_string())].into_iter().collect();
        assert_eq!(result, expected);
    }
/*
  test("OneOf") {
    val rules =
      Vector(
        OneOf(-1L, Vector("1448", "1337")),
        Literal(-1L, "1337"))
    getConclusions(rules, true) shouldEqual Map(-1L -> "1337")
  }
*/
// mig: fn solves_a_components_rule
    #[test]
    fn solves_a_components_rule() {

        let rules: Vec<TestRule> = vec![
            TestRule::CoordComponents(CoordComponents {
                coord_rune: -1,
                ownership_rune: -2,
                kind_rune: -3,
            }),
            TestRule::Literal(Literal {
                rune: -2,
                value: "1337".to_string(),
            }),
            TestRule::Literal(Literal {
                rune: -3,
                value: "1448".to_string(),
            }),
        ];
        let result = get_conclusions(rules, true, HashMap::new());
        let expected: HashMap<i64, String> = [
            (-1, "1337/1448".to_string()),
            (-2, "1337".to_string()),
            (-3, "1448".to_string()),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }
/*
  test("Solves a components rule") {
    val rules =
      Vector(
        CoordComponents(-1L, -2L, -3L),
        Literal(-2L, "1337"),
        Literal(-3L, "1448"))
    getConclusions(rules, true) shouldEqual
      Map(-1L -> "1337/1448", -2L -> "1337", -3L -> "1448")
  }
*/
// mig: fn reverse_solve_a_components_rule
    #[test]
    fn reverse_solve_a_components_rule() {

        let rules: Vec<TestRule> = vec![
            TestRule::CoordComponents(CoordComponents {
                coord_rune: -1,
                ownership_rune: -2,
                kind_rune: -3,
            }),
            TestRule::Literal(Literal {
                rune: -1,
                value: "1337/1448".to_string(),
            }),
        ];
        let result = get_conclusions(rules, true, HashMap::new());
        let expected: HashMap<i64, String> = [
            (-1, "1337/1448".to_string()),
            (-2, "1337".to_string()),
            (-3, "1448".to_string()),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }
/*
  test("Reverse-solve a components rule") {
    val rules =
      Vector(
        CoordComponents(-1L, -2L, -3L),
        Literal(-1L, "1337/1448"))
    getConclusions(rules, true) shouldEqual
      Map(-1L -> "1337/1448", -2L -> "1337", -3L -> "1448")
  }
*/
// mig: fn test_infer_pack
    #[test]
    fn test_infer_pack() {

        let rules: Vec<TestRule> = vec![
            TestRule::Literal(Literal { rune: -1, value: "1337".to_string() }),
            TestRule::Literal(Literal { rune: -2, value: "1448".to_string() }),
            TestRule::Pack(Pack {
                result_rune: -3,
                member_runes: vec![-1, -2],
            }),
        ];
        let result = get_conclusions(rules, true, HashMap::new());
        let expected: HashMap<i64, String> = [
            (-1, "1337".to_string()),
            (-2, "1448".to_string()),
            (-3, "1337,1448".to_string()),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }
/*
  test("Test infer Pack") {
    val rules =
      Vector(
        Literal(-1L, "1337"),
        Literal(-2L, "1448"),
        Pack(-3L, Vector(-1L, -2L)))
    getConclusions(rules, true) shouldEqual
      Map(-1L -> "1337", -2L -> "1448", -3L -> "1337,1448")
  }
*/
// mig: fn test_infer_pack_from_result
    #[test]
    fn test_infer_pack_from_result() {

        let rules: Vec<TestRule> = vec![
            TestRule::Literal(Literal {
                rune: -3,
                value: "1337,1448".to_string(),
            }),
            TestRule::Pack(Pack {
                result_rune: -3,
                member_runes: vec![-1, -2],
            }),
        ];
        let result = get_conclusions(rules, true, HashMap::new());
        let expected: HashMap<i64, String> = [
            (-1, "1337".to_string()),
            (-2, "1448".to_string()),
            (-3, "1337,1448".to_string()),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }
/*
  test("Test infer Pack from result") {
    val rules =
      Vector(
        Literal(-3L, "1337,1448"),
        Pack(-3L, Vector(-1L, -2L)))
    getConclusions(rules, true) shouldEqual
      Map(-1L -> "1337", -2L -> "1448", -3L -> "1337,1448")
  }
*/
// mig: fn test_infer_pack_from_empty_result
    #[test]
    fn test_infer_pack_from_empty_result() {

        let rules: Vec<TestRule> = vec![
            TestRule::Literal(Literal {
                rune: -3,
                value: "".to_string(),
            }),
            TestRule::Pack(Pack {
                result_rune: -3,
                member_runes: vec![],
            }),
        ];
        let result = get_conclusions(rules, true, HashMap::new());
        let expected: HashMap<i64, String> = [(-3, "".to_string())].into_iter().collect();
        assert_eq!(result, expected);
    }
/*
  test("Test infer Pack from empty result") {
    val rules =
      Vector(
        Literal(-3L, ""),
        Pack(-3L, Vector()))
    getConclusions(rules, true) shouldEqual
      Map(-3L -> "")
  }
*/
// mig: fn test_cant_solve_empty_pack
    #[test]
    fn test_cant_solve_empty_pack() {

        let rules: Vec<TestRule> = vec![TestRule::Pack(Pack {
            result_rune: -3,
            member_runes: vec![],
        })];
        let result = get_conclusions(rules, false, HashMap::new());
        let expected: HashMap<i64, String> = HashMap::new();
        assert_eq!(result, expected);
    }
/*
  test("Test cant solve empty Pack") {
    val rules =
      Vector(
        Pack(-3L, Vector()))
    getConclusions(rules, false) shouldEqual Map()
  }
*/
// mig: fn complex_rule_set
    #[test]
    fn complex_rule_set() {

        let rules: Vec<TestRule> = vec![
            TestRule::Literal(Literal {
                rune: -3,
                value: "1448".to_string(),
            }),
            TestRule::CoordComponents(CoordComponents {
                coord_rune: -6,
                ownership_rune: -5,
                kind_rune: -5,
            }),
            TestRule::Literal(Literal {
                rune: -2,
                value: "1337".to_string(),
            }),
            TestRule::Equals(Equals {
                left_rune: -4,
                right_rune: -2,
            }),
            TestRule::OneOf(OneOf {
                coord_rune: -4,
                possible_values: vec!["1337".to_string(), "73".to_string()],
            }),
            TestRule::Equals(Equals {
                left_rune: -1,
                right_rune: -5,
            }),
            TestRule::CoordComponents(CoordComponents {
                coord_rune: -1,
                ownership_rune: -2,
                kind_rune: -3,
            }),
            TestRule::Equals(Equals {
                left_rune: -6,
                right_rune: -7,
            }),
        ];
        let conclusions = get_conclusions(rules, true, HashMap::new());
        assert_eq!(
            conclusions.get(&-7),
            Some(&"1337/1448/1337/1448".to_string())
        );
    }
/*
  test("Complex rule set") {
    val conclusions = getConclusions(complexRuleSet, true)
    conclusions.get(-7L) shouldEqual Some("1337/1448/1337/1448")
  }
*/
// mig: fn test_receiving_struct_to_struct
    #[test]
    fn test_receiving_struct_to_struct() {

        let rules: Vec<TestRule> = vec![
            TestRule::Literal(Literal {
                rune: -1,
                value: "Firefly".to_string(),
            }),
            TestRule::Send(Send {
                sender_rune: -2,
                receiver_rune: -1,
            }),
        ];
        let result = get_conclusions(rules, true, HashMap::new());
        let expected: HashMap<i64, String> = [
            (-1, "Firefly".to_string()),
            (-2, "Firefly".to_string()),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }
/*
  test("Test receiving struct to struct") {
    val rules =
      Vector(
        Literal(-1L, "Firefly"),
        Send(-2L, -1L))
    getConclusions(rules, true) shouldEqual
      Map(-1L -> "Firefly", -2L -> "Firefly")
  }
*/
// mig: fn test_receive_struct_from_sent_interface
    #[test]
    fn test_receive_struct_from_sent_interface() {


        let rules: Vec<TestRule> = vec![
            TestRule::Literal(Literal {
                rune: -1,
                value: "Firefly".to_string(),
            }),
            TestRule::Literal(Literal {
                rune: -2,
                value: "ISpaceship".to_string(),
            }),
            TestRule::Send(Send {
                sender_rune: -2,
                receiver_rune: -1,
            }),
        ];
        let failed = expect_solve_failure(rules);

        // commitStep appends the step before checking conflicts, so the conflicting step
        // is captured in the audit trail (matching Scala behavior).
        let conclusions_set: HashSet<(i64, String)> = failed
            .steps
            .iter()
            .flat_map(|s| {
                s.conclusions
                    .iter()
                    .map(|(r, c)| (*r, c.clone()))
            })
            .collect();
        let expected_conclusions: HashSet<(i64, String)> = [
            (-1, "Firefly".to_string()),
            (-2, "ISpaceship".to_string()),
            (-2, "Firefly".to_string()),
        ]
        .into_iter()
        .collect();
        assert_eq!(conclusions_set, expected_conclusions);

        assert_eq!(failed.unsolved_rules.len(), 1);
        match &failed.unsolved_rules[0] {
            TestRule::Send(s) => {
                assert_eq!(s.sender_rune, -2);
                assert_eq!(s.receiver_rune, -1);
            }
            _ => panic!("Expected Send in unsolved_rules"),
        }

        match &failed.error {
            ISolverError::SolverConflict(conflict) => {
                assert_eq!(conflict.rune, -2);
                assert_eq!(conflict.previous_conclusion, "ISpaceship");
                assert_eq!(conflict.new_conclusion, "Firefly");
            }
            _ => panic!("Expected SolverConflict(-2, \"ISpaceship\", \"Firefly\")"),
        }
    }
/*
  test("Test receive struct from sent interface") {
    val rules =
      Vector(
        Literal(-1L, "Firefly"),
        Literal(-2L, "ISpaceship"),
        Send(-2L, -1L))
    expectSolveFailure(rules) match {
      case FailedSolve(steps, conclusions, unsolvedRules, unsolvedRunes, err) => {
        steps.flatMap(_.conclusions).toSet shouldEqual
          Set((-1,"Firefly"), (-2,"ISpaceship"), (-2,"Firefly"))
        unsolvedRules.toSet shouldEqual Set(Send(-2, -1))
        err match {
          case SolverConflict(
            -2,
            // Already concluded this
            "ISpaceship",
            // But now we're concluding that it should have been a Firefly
            "Firefly") =>
        }
      }
    }
  }
*/
// mig: fn test_receive_interface_from_sent_struct
    #[test]
    fn test_receive_interface_from_sent_struct() {

        let rules: Vec<TestRule> = vec![
            TestRule::Literal(Literal {
                rune: -1,
                value: "ISpaceship".to_string(),
            }),
            TestRule::Literal(Literal {
                rune: -2,
                value: "Firefly".to_string(),
            }),
            TestRule::Send(Send {
                sender_rune: -2,
                receiver_rune: -1,
            }),
        ];
        let result = get_conclusions(rules, true, HashMap::new());
        let expected: HashMap<i64, String> = [
            (-1, "ISpaceship".to_string()),
            (-2, "Firefly".to_string()),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }
/*
  test("Test receive interface from sent struct") {
    val rules =
      Vector(
        Literal(-1L, "ISpaceship"),
        Literal(-2L, "Firefly"),
        Send(-2L, -1L))
    // Should be a successful solve
    getConclusions(rules, true) shouldEqual
      Map(-1L -> "ISpaceship", -2L -> "Firefly")
  }
*/
// mig: fn test_complex_solve_most_specific_ancestor
    // Tests @CSCDSRZ: complex solve infers the receiver kind from senders.
    #[test]
    fn test_complex_solve_most_specific_ancestor() {

        let rules: Vec<TestRule> = vec![
            TestRule::Literal(Literal {
                rune: -2,
                value: "Firefly".to_string(),
            }),
            TestRule::Send(Send {
                sender_rune: -2,
                receiver_rune: -1,
            }),
        ];
        let result = get_conclusions(rules, true, HashMap::new());
        let expected: HashMap<i64, String> = [
            (-1, "Firefly".to_string()),
            (-2, "Firefly".to_string()),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }
/*
  test("Test complex solve: most specific ancestor") {
    val rules =
      Vector(
        Literal(-2L, "Firefly"),
        Send(-2L, -1L))
    // Should be a successful solve
    getConclusions(rules, true) shouldEqual
      Map(-1L -> "Firefly", -2L -> "Firefly")
  }
*/
// mig: fn test_complex_solve_calculate_common_ancestor
    // Tests @CSCDSRZ: complex solve finds the common ancestor of multiple senders.
    #[test]
    fn test_complex_solve_calculate_common_ancestor() {

        let rules: Vec<TestRule> = vec![
            TestRule::Literal(Literal {
                rune: -2,
                value: "Firefly".to_string(),
            }),
            TestRule::Literal(Literal {
                rune: -3,
                value: "Serenity".to_string(),
            }),
            TestRule::Send(Send {
                sender_rune: -2,
                receiver_rune: -1,
            }),
            TestRule::Send(Send {
                sender_rune: -3,
                receiver_rune: -1,
            }),
        ];
        let result = get_conclusions(rules, true, HashMap::new());
        let expected: HashMap<i64, String> = [
            (-1, "ISpaceship".to_string()),
            (-2, "Firefly".to_string()),
            (-3, "Serenity".to_string()),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }
/*
  test("Test complex solve: calculate common ancestor") {
    val rules =
      Vector(
        Literal(-2L, "Firefly"),
        Literal(-3L, "Serenity"),
        Send(-2L, -1L),
        Send(-3L, -1L))
    // Should be a successful solve
    getConclusions(rules, true) shouldEqual
      Map(-1L -> "ISpaceship", -2L -> "Firefly", -3L -> "Serenity")
  }
*/
// mig: fn test_complex_solve_descendant_satisfying_call
    // Tests @CSCDSRZ: complex solve picks a descendant that satisfies a call constraint.
    #[test]
    fn test_complex_solve_descendant_satisfying_call() {

        let rules: Vec<TestRule> = vec![
            TestRule::Literal(Literal {
                rune: -2,
                value: "Flamethrower:int".to_string(),
            }),
            TestRule::Send(Send {
                sender_rune: -2,
                receiver_rune: -1,
            }),
            TestRule::Call(Call {
                result_rune: -1,
                name_rune: -3,
                arg_rune: -4,
            }),
            TestRule::Literal(Literal {
                rune: -3,
                value: "IWeapon".to_string(),
            }),
        ];
        let result = get_conclusions(rules, true, HashMap::new());
        let expected: HashMap<i64, String> = [
            (-1, "IWeapon:int".to_string()),
            (-4, "int".to_string()),
            (-2, "Flamethrower:int".to_string()),
            (-3, "IWeapon".to_string()),
        ]
        .into_iter()
        .collect();
        assert_eq!(result, expected);
    }
/*
  test("Test complex solve: descendant satisfying call") {
    val rules =
      Vector(
        Literal(-2L, "Flamethrower:int"),
        Send(-2L, -1L),
        Call(-1L, -3L, -4L),
        Literal(-3L, "IWeapon"))
    // Should be a successful solve
    getConclusions(rules, true) shouldEqual
      Map(
        -1 -> "IWeapon:int",
        -4 -> "int",
        -2 -> "Flamethrower:int",
        -3 -> "IWeapon")
  }
*/
// mig: fn partial_solve
    #[test]
    fn partial_solve() {

        let scout_bump = Bump::new();
        let scout_arena = ScoutArena::new(&scout_bump);
        let rules: Vec<TestRule> = vec![
            TestRule::Literal(Literal { rune: -2, value: "A".to_string() }),
            TestRule::Call(Call {
                result_rune: -3,
                name_rune: -1,
                arg_rune: -2,
            }),
        ];
        let all_runes: Vec<i64> = {
            let mut v: Vec<i64> = rules.iter().flat_map(|r| r.all_runes()).collect();
            v.sort();
            v.dedup();
            v
        };
        let test_solver = super::test_rule_solver::TestRuleSolver {
            scout_arena: &scout_arena,
        };
        let mut solver_state = make_solver_state(
            true,
            false,
            Box::new(super::test_rule_solver::rule_to_puzzles),
            &|rule: &super::test_rules::TestRule| rule.all_runes(),
            rules,
            HashMap::new(),
            all_runes,
        );

        while advance(&mut solver_state, &test_solver).expect("advance") {}
        let first_conclusions: HashMap<i64, String> =
            solver_state.userify_conclusions().into_iter().collect();
        assert_eq!(first_conclusions.get(&-2), Some(&"A".to_string()));

        let mut new_conclusions = HashMap::new();
        new_conclusions.insert(-1i64, "Firefly".to_string());
        solver_state
            .commit_step::<String>(false, vec![], new_conclusions, vec![], HashSet::new())
            .expect("commit_step");

        while advance(&mut solver_state, &test_solver).expect("advance") {}
        let second_conclusions: HashMap<i64, String> =
            solver_state.userify_conclusions().into_iter().collect();
        assert_eq!(second_conclusions.get(&-1), Some(&"Firefly".to_string()));
        assert_eq!(second_conclusions.get(&-2), Some(&"A".to_string()));
        assert_eq!(
            second_conclusions.get(&-3),
            Some(&"Firefly:A".to_string())
        );
    }
/*
  test("Partial Solve") {
    val interner = new Interner()

    // It'll be nice to partially solve some rules, for example before we put them in the overload index.

    // Note how these two rules aren't connected to each other at all
    val rules =
      Vector(
        Literal(-2, "A"),
        Call(-3, -1, -2)) // We dont know the template, -1, yet


    val solverState =
        Solver.makeSolverState[IRule, Long, String](
          true,
          true,
          (rule: IRule) => rule.allPuzzles,
          (rule: IRule) => rule.allRunes.toVector,
          rules,
          Map(),
          rules.flatMap(_.allRunes).distinct)

    while ( {
      advance(solverState) match {
        case Ok(continue) => continue
        case Err(e) => vfail(e)
      }
    }) {}
    val firstConclusions = solverState.userifyConclusions().toMap

    firstConclusions.toMap shouldEqual Map(-2 -> "A")
    solverState.commitStep[String](false, Vector(), Map(-1L -> "Firefly"), Vector()).getOrDie()

    while ( {
      advance(solverState) match {
        case Ok(continue) => continue
        case Err(e) => vfail(e)
      }
    }) {}
    val secondConclusions = solverState.userifyConclusions().toMap

    secondConclusions.toMap shouldEqual
      Map(-1 -> "Firefly", -2 -> "A", -3 -> "Firefly:A")
  }
*/
// mig: fn predicting
    #[test]
    fn predicting() {

        let predictions = solve_with_puzzler(Box::new(|rule: &TestRule| match rule {
            TestRule::Lookup(_) => vec![],
            other => other.all_puzzles(),
        }));
        assert_eq!(predictions.len(), 1, "predicting mode should solve only rune -2");
        assert_eq!(predictions.get(&-2), Some(&"1337".to_string()));

        let conclusions = solve_with_puzzler(Box::new(|rule: &TestRule| rule.all_puzzles()));
        let expected: HashMap<i64, String> = [
            (-1, "Firefly".to_string()),
            (-2, "1337".to_string()),
            (-3, "Firefly:1337".to_string()),
        ]
        .into_iter()
        .collect();
        assert_eq!(conclusions, expected);
    }
/*
//
//  test("bork") {
//    // It'll be nice to partially solve some rules, for example before we put them in the overload index.
//
//    // Note how these two rules aren't connected to each other at all
//    val rules =
//      Vector(
//        Lookup(-5, "Firefly"),
//        Equals(-2, -5),
//        Send(-1,-5)) // We dont know the template, -1, yet
//    getConclusions(rules, true, Map(-5L -> "Firefly")) shouldEqual
//      Map(-1L -> "ISpaceship", -2L -> "Firefly")
//  }

  test("Predicting") {
    // "Predicting" is when the rules arent completely solvable, but we can still run some of them
    // to figure out what we can.
    // For example, in:
    //   #2 = 1337
    //   #3 = #1<#2>
    // we can figure out that #2 is 1337, even if we don't know #1 yet.
    // This is useful for recursive types.
    // See: Recursive Types Must Have Types Predicted (RTMHTP)
*/
// mig: fn solve_with_puzzler
    fn solve_with_puzzler(
        puzzler: Box<dyn Fn(&super::test_rules::TestRule) -> Vec<Vec<i64>>>,
    ) -> HashMap<i64, String> {


        let scout_bump = Bump::new();
        let scout_arena = ScoutArena::new(&scout_bump);
        let rules: Vec<TestRule> = vec![
            TestRule::Lookup(Lookup {
                rune: -1,
                name: "Firefly".to_string(),
            }),
            TestRule::Literal(Literal {
                rune: -2,
                value: "1337".to_string(),
            }),
            TestRule::Call(Call {
                result_rune: -3,
                name_rune: -1,
                arg_rune: -2,
            }),
        ];
        let all_runes: Vec<i64> = {
            let mut v: Vec<i64> = rules.iter().flat_map(|r| r.all_runes()).collect();
            v.sort();
            v.dedup();
            v
        };
        let test_solver = super::test_rule_solver::TestRuleSolver { scout_arena: &scout_arena };
        let mut solver_state = make_solver_state(
            true,
            false,
            puzzler,
            &|rule: &super::test_rules::TestRule| rule.all_runes(),
            rules,
            HashMap::new(),
            all_runes,
        );
        while advance(&mut solver_state, &test_solver).expect("advance") {}
        solver_state.userify_conclusions().into_iter().collect()
    }
/*
    def solveWithPuzzler(puzzler: IRule => Vector[Vector[Long]]) = {
      val interner = new Interner()

      // Below, we're reporting that Lookup has no puzzles that can solve it.
      val rules =
        Vector(
          Lookup(-1, "Firefly"),
          Literal(-2, "1337"),
          Call(-3, -1, -2)) // X = Firefly<A>

      val solverState =
        Solver.makeSolverState[IRule, Long, String](
          true,
          true,
          puzzler,
          (rule: IRule) => rule.allRunes.toVector,
          rules,
          Map(),
          rules.flatMap(_.allRunes).distinct)


      while ( {
        advance(solverState) match {
          case Ok(continue) => continue
          case Err(e) => vfail(e)
        }
      }) {}
      val conclusions = solverState.userifyConclusions().toMap
      conclusions
    }

    val predictions =
      solveWithPuzzler({
        // This Vector() makes it unsolvable
        case Lookup(rune, name) => Vector()
        case rule => rule.allPuzzles
      })
//    vassert(predictionRuleExecutionOrder sameElements Vector(1))
    vassert(predictions.size == 1)
    vassert(predictions(-2) == "1337")

    val conclusions = solveWithPuzzler(_.allPuzzles)
//    vassert(ruleExecutionOrder.length == 3)
    conclusions shouldEqual Map(-1L -> "Firefly", -2L -> "1337", -3L -> "Firefly:1337")
  }
*/
// mig: fn test_conflict
    #[test]
    fn test_conflict() {


        let rules: Vec<TestRule> = vec![
            TestRule::Literal(Literal {
                rune: -1,
                value: "1448".to_string(),
            }),
            TestRule::Literal(Literal {
                rune: -1,
                value: "1337".to_string(),
            }),
        ];
        let failed = expect_solve_failure(rules);
        match &failed.error {
            ISolverError::SolverConflict(conflict) => {
                let mut conclusions =
                    vec![conflict.previous_conclusion.clone(), conflict.new_conclusion.clone()];
                conclusions.sort();
                assert_eq!(conclusions, ["1337", "1448"]);
            }
            _ => panic!("Expected SolverConflict"),
        }
    }
/*
  test("Test conflict") {
    val rules =
      Vector(
        Literal(-1L, "1448"),
        Literal(-1L, "1337"))
    expectSolveFailure(rules) match {
      case FailedSolve(_, _, _, _, SolverConflict(_, conclusionA, conclusionB)) => {
        Vector(conclusionA, conclusionB).sorted shouldEqual Vector("1337", "1448").sorted
      }
    }
  }
*/
// mig: fn expect_solve_failure
    fn expect_solve_failure(
        rules: Vec<super::test_rules::TestRule>,
    ) -> FailedSolve<TestRule, i64, String, String> {


        let scout_bump = Bump::new();
        let scout_arena = ScoutArena::new(&scout_bump);
        let all_runes: Vec<i64> = {
            let mut v: Vec<i64> = rules.iter().flat_map(|r| r.all_runes()).collect();
            v.sort();
            v.dedup();
            v
        };
        let test_solver = super::test_rule_solver::TestRuleSolver {
            scout_arena: &scout_arena,
        };
        let mut solver_state = make_solver_state(
            true,
            false,
            Box::new(super::test_rule_solver::rule_to_puzzles),
            &|rule: &super::test_rules::TestRule| rule.all_runes(),
            rules,
            HashMap::new(),
            all_runes,
        );

        loop {
            match advance(&mut solver_state, &test_solver) {
                Ok(continue_flag) => {
                    if !continue_flag {
                        break;
                    }
                }
                Err(f) => return f,
            }
        }

        panic!("Incorrectly completed the solve")
    }
/*
  private def expectSolveFailure(rules: IndexedSeq[IRule]):
  FailedSolve[IRule, Long, String, String] = {
    val interner = new Interner()

    val solverState =
      Solver.makeSolverState[IRule, Long, String](
        true,
        true,
        (rule: IRule) => rule.allPuzzles,
        (rule: IRule) => rule.allRunes.toVector,
        rules,
        Map(),
        rules.flatMap(_.allRunes).distinct.toVector)

    while ( {
      advance(solverState) match {
        case Ok(continue) => continue
        case Err(e) => return e
      }
    }) {}
    vfail("Incorrectly completed the solve")
  }
*/
// mig: fn get_conclusions
    fn get_conclusions(
        rules: Vec<super::test_rules::TestRule>,
        expect_complete_solve: bool,
        initially_known_runes: HashMap<i64, String>,
    ) -> HashMap<i64, String> {


        let scout_bump = Bump::new();
        let scout_arena = ScoutArena::new(&scout_bump);
        let all_runes_from_rules: HashSet<i64> =
            rules.iter().flat_map(|r| r.all_runes()).collect();
        let all_runes: Vec<i64> = {
            let mut v: Vec<i64> = rules
                .iter()
                .flat_map(|r| r.all_runes())
                .chain(initially_known_runes.keys().cloned())
                .collect();
            v.sort();
            v.dedup();
            v
        };
        let test_solver = super::test_rule_solver::TestRuleSolver {
            scout_arena: &scout_arena,
        };
        let mut solver_state = make_solver_state(
            true,
            false,
            Box::new(super::test_rule_solver::rule_to_puzzles),
            &|rule: &super::test_rules::TestRule| rule.all_runes(),
            rules,
            initially_known_runes,
            all_runes,
        );

        while advance(&mut solver_state, &test_solver).expect("advance") {}

        let conclusions: HashMap<i64, String> =
            solver_state.userify_conclusions().into_iter().collect();
        let conclusions_keys: HashSet<i64> =
            conclusions.keys().cloned().collect();
        assert_eq!(expect_complete_solve, conclusions_keys == all_runes_from_rules);

        conclusions
    }
/*
  private def getConclusions(
    rules: IndexedSeq[IRule],
    expectCompleteSolve: Boolean,
    initiallyKnownRunes: Map[Long, String] = Map()):
  Map[Long, String] = {
    val interner = new Interner()

    val solverState =
      Solver.makeSolverState[IRule, Long, String](
        true,
        true,
        (r: IRule) => r.allPuzzles,
        (rule: IRule) => rule.allRunes.toVector,
        rules,
        initiallyKnownRunes,
        (rules.flatMap(_.allRunes) ++ initiallyKnownRunes.keys).distinct.toVector)


    while ( {
      advance(solverState) match {
        case Ok(continue) => continue
        case Err(e) => vfail(e)
      }
    }) {}
    // If we get here, then there's nothing more the solver can do.
    val conclusionsMap = solverState.userifyConclusions().toMap

    vassert(expectCompleteSolve == (conclusionsMap.keySet == rules.flatMap(_.allRunes).toSet))
    conclusionsMap
  }

  // --- TDD tests: these document expected Step behavior ---

  test("Simple solve produces exactly one step per rule") {
    // A single Literal rule should produce:
    //   1 initial step (from constructor's commitStep for initiallyKnownRunes)
    //   + 1 solve step (from solving the Literal rule)
    //   = 2 total steps
    val interner = new Interner()
    val rules = Vector(Literal(-1L, "1337"))
    val solverState = Solver.makeSolverState[IRule, Long, String](
      true, true,
      (r: IRule) => r.allPuzzles,
      (rule: IRule) => rule.allRunes.toVector,
      rules, Map(), rules.flatMap(_.allRunes).distinct)

    while (advance(solverState) match { case Ok(c) => c case Err(e) => vfail(e) }) {}

    val steps = solverState.getSteps()
    steps.size shouldEqual 2
  }

  test("No duplicate solvedRules entries across steps") {
    // Each rule index should appear in solvedRules of at most one step.
    val interner = new Interner()
    val rules = Vector(Literal(-1L, "1337"))
    val solverState = Solver.makeSolverState[IRule, Long, String](
      true, true,
      (r: IRule) => r.allPuzzles,
      (rule: IRule) => rule.allRunes.toVector,
      rules, Map(), rules.flatMap(_.allRunes).distinct)

    while (advance(solverState) match { case Ok(c) => c case Err(e) => vfail(e) }) {}

    val steps = solverState.getSteps()
    val allSolvedRuleIndices = steps.flatMap(_.solvedRules.map(_._1))
    allSolvedRuleIndices shouldEqual allSolvedRuleIndices.distinct
  }

  test("Multi-rule solve has correct step count") {
    // Two Literal rules + one Equals:
    //   1 initial step
    //   + 3 solve steps (one per rule)
    //   = 4 total
    val interner = new Interner()
    val rules = Vector(
      Literal(-1L, "1337"),
      Literal(-2L, "1337"),
      Equals(-1L, -2L))
    val solverState = Solver.makeSolverState[IRule, Long, String](
      true, true,
      (r: IRule) => r.allPuzzles,
      (rule: IRule) => rule.allRunes.toVector,
      rules, Map(), rules.flatMap(_.allRunes).distinct)

    while (advance(solverState) match { case Ok(c) => c case Err(e) => vfail(e) }) {}

    val steps = solverState.getSteps()
    // initial + 3 solves = 4
    steps.size shouldEqual 4
  }

  test("Solve step records its conclusions") {
    // The step that solves a Literal rule should contain the conclusion
    // from that solve, not an empty map.
    val interner = new Interner()
    val rules = Vector(Literal(-1L, "1337"))
    val solverState = Solver.makeSolverState[IRule, Long, String](
      true, true,
      (r: IRule) => r.allPuzzles,
      (rule: IRule) => rule.allRunes.toVector,
      rules, Map(), rules.flatMap(_.allRunes).distinct)

    while (advance(solverState) match { case Ok(c) => c case Err(e) => vfail(e) }) {}

    val steps = solverState.getSteps()
    // Find the step(s) that solved rule 0
    val solveSteps = steps.filter(_.solvedRules.exists(_._1 == 0))
    // There should be exactly one step that solved this rule
    solveSteps.size shouldEqual 1
    // And it should contain the conclusion
    solveSteps.head.conclusions shouldEqual Map(-1L -> "1337")
  }
}
*/