/*
package dev.vale.solver

import dev.vale.{Collector, Err, Interner, Ok, RangeS, vassert, vfail}
import org.scalatest._

import scala.collection.immutable.Map

class SolverTests extends FunSuite with Matchers with Collector {
*/
// mig: const complex_rule_set
const complex_rule_set_rules: Vec<()> = vec![];
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
const complex_rule_set_equals_rules: Vec<i32> = vec![];
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
// mig: fn simple_int_rule
    #[test]
    fn simple_int_rule() {
        use super::test_rules::{Literal, TestRule};
        use std::collections::HashMap;

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
        use super::test_rules::{Equals, Literal, TestRule};
        use std::collections::HashMap;

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
        use super::test_rules::{OneOf, TestRule};
        use std::collections::HashMap;

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
        use super::test_rules::{Literal, OneOf, TestRule};
        use std::collections::HashMap;

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
        use super::test_rules::{Literal, OneOf, TestRule};
        use std::collections::HashMap;

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
        use super::test_rules::{CoordComponents, Literal, TestRule};
        use std::collections::HashMap;

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
        use super::test_rules::{CoordComponents, Literal, TestRule};
        use std::collections::HashMap;

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
        use super::test_rules::{Literal, Pack, TestRule};
        use std::collections::HashMap;

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
        use super::test_rules::{Literal, Pack, TestRule};
        use std::collections::HashMap;

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
        use super::test_rules::{Literal, Pack, TestRule};
        use std::collections::HashMap;

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
        use super::test_rules::{Pack, TestRule};
        use std::collections::HashMap;

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
        use super::test_rules::{CoordComponents, Equals, Literal, OneOf, TestRule};
        use std::collections::HashMap;

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
        use super::test_rules::{Literal, Send, TestRule};
        use std::collections::HashMap;

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
        use super::test_rules::{Literal, Send, TestRule};
        use crate::solver::ISolverError;
        use std::collections::HashSet;

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

        // MIGALLOW: Rust's immediate-commit design means FailedSolve.steps differs from Scala.
        // The step that produced the conflicting conclusion is never recorded. Steps contain
        // only (-1, "Firefly") and (-2, "ISpaceship"). Scala would also have (-2, "Firefly").
        // The conflicting conclusion is captured in error (SolverConflict).
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
      case FailedSolve(steps, unsolvedRules, err) => {
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
        use super::test_rules::{Literal, Send, TestRule};
        use std::collections::HashMap;

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
    #[test]
    fn test_complex_solve_most_specific_ancestor() {
        use super::test_rules::{Literal, Send, TestRule};
        use std::collections::HashMap;

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
    #[test]
    fn test_complex_solve_calculate_common_ancestor() {
        use super::test_rules::{Literal, Send, TestRule};
        use std::collections::HashMap;

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
    #[test]
    fn test_complex_solve_descendant_satisfying_call() {
        use super::test_rules::{Call, Literal, Send, TestRule};
        use std::collections::HashMap;

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
        use super::test_rules::{Call, Literal, TestRule};
        use crate::solver::Solver;
        use crate::utils::range::RangeS;
        use bumpalo::Bump;

        let arena = Bump::new();
        let interner = crate::Interner::with_arena(&arena);
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
        let delegate = super::test_rule_solver::TestRuleSolver {
            interner: &interner,
        };
        let mut solver = Solver::new(
            true,
            delegate,
            vec![RangeS::test_zero(&interner)],
            rules,
            std::collections::HashMap::new(),
            all_runes,
        );

        while solver.advance(&(), &()).expect("advance") {}
        let first_conclusions: std::collections::HashMap<i64, String> =
            solver.userify_conclusions().into_iter().collect();
        assert_eq!(first_conclusions.get(&-2), Some(&"A".to_string()));

        let canonical_1 = solver.get_canonical_rune(&-1i64);
        let mut new_conclusions = std::collections::HashMap::new();
        new_conclusions.insert(canonical_1, "Firefly".to_string());
        solver
            .mark_rules_solved(vec![], new_conclusions)
            .expect("mark_rules_solved");

        while solver.advance(&(), &()).expect("advance") {}
        let second_conclusions: std::collections::HashMap<i64, String> =
            solver.userify_conclusions().into_iter().collect();
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


    val solver =
      new Solver(
        true,
        true,
        interner,
        (rule: IRule) => rule.allPuzzles,
        (rule: IRule) => rule.allRunes.toVector,
        new TestRuleSolver(interner),
        List(RangeS.testZero(interner)),
        rules,
        Map(),
        rules.flatMap(_.allRunes).distinct)

    while ( {
      solver.advance(Unit, Unit) match {
        case Ok(continue) => continue
        case Err(e) => vfail(e)
      }
    }) {}
    val firstConclusions = solver.userifyConclusions().toMap

    firstConclusions.toMap shouldEqual Map(-2 -> "A")
    solver.markRulesSolved(Vector(), Map(solver.getCanonicalRune(-1) -> "Firefly"))

    while ( {
      solver.advance(Unit, Unit) match {
        case Ok(continue) => continue
        case Err(e) => vfail(e)
      }
    }) {}
    val secondConclusions = solver.userifyConclusions().toMap

    secondConclusions.toMap shouldEqual
      Map(-1 -> "Firefly", -2 -> "A", -3 -> "Firefly:A")
  }
*/
// mig: fn predicting
    #[test]
    fn predicting() {
        use super::test_rules::TestRule;
        use std::collections::HashMap;

        let predictions = solve_with_puzzler(|rule: &TestRule| match rule {
            TestRule::Lookup(_) => vec![],
            other => other.all_puzzles(),
        });
        assert_eq!(predictions.len(), 1, "predicting mode should solve only rune -2");
        assert_eq!(predictions.get(&-2), Some(&"1337".to_string()));

        let conclusions = solve_with_puzzler(|rule: &TestRule| rule.all_puzzles());
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
        puzzler: impl Fn(&super::test_rules::TestRule) -> Vec<Vec<i64>>,
    ) -> std::collections::HashMap<i64, String> {
        use super::test_rules::{Call, Lookup, Literal, TestRule};
        use crate::solver::Solver;
        use crate::utils::range::RangeS;
        use bumpalo::Bump;

        let arena = Bump::new();
        let interner = crate::Interner::with_arena(&arena);
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
        let delegate = super::test_rule_solver::CustomPuzzlerDelegate {
            base: super::test_rule_solver::TestRuleSolver { interner: &interner },
            puzzler,
        };
        let mut solver = Solver::new(
            true,
            delegate,
            vec![RangeS::test_zero(&interner)],
            rules,
            std::collections::HashMap::new(),
            all_runes,
        );
        while solver.advance(&(), &()).expect("advance") {}
        solver.userify_conclusions().into_iter().collect()
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

      val solver =
        new Solver[IRule, Long, Unit, Unit, String, String](
          true,
          true,
          interner,
          puzzler,
          (rule: IRule) => rule.allRunes.toVector,
          new TestRuleSolver(interner),
          List(RangeS.testZero(interner)),
          rules,
          Map(),
          rules.flatMap(_.allRunes).distinct)


      while ( {
        solver.advance(Unit, Unit) match {
          case Ok(continue) => continue
          case Err(e) => vfail(e)
        }
      }) {}
      val conclusions = solver.userifyConclusions().toMap
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
        use super::test_rules::{Literal, TestRule};
        use crate::solver::ISolverError;

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
      case FailedSolve(_, _, SolverConflict(_, conclusionA, conclusionB)) => {
        Vector(conclusionA, conclusionB).sorted shouldEqual Vector("1337", "1448").sorted
      }
    }
  }
*/
// mig: fn expect_solve_failure
    fn expect_solve_failure(
        rules: Vec<super::test_rules::TestRule>,
    ) -> crate::solver::FailedSolve<super::test_rules::TestRule, i64, String, String> {
        use crate::solver::Solver;
        use crate::utils::range::RangeS;
        use bumpalo::Bump;
        use std::collections::HashMap;

        let arena = Bump::new();
        let interner = crate::Interner::with_arena(&arena);
        let all_runes: Vec<i64> = {
            let mut v: Vec<i64> = rules.iter().flat_map(|r| r.all_runes()).collect();
            v.sort();
            v.dedup();
            v
        };
        let delegate = super::test_rule_solver::TestRuleSolver {
            interner: &interner,
        };
        let mut solver = Solver::new(
            true,
            delegate,
            vec![RangeS::test_zero(&interner)],
            rules,
            HashMap::new(),
            all_runes,
        );

        loop {
            match solver.advance(&(), &()) {
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

    val solver =
      new Solver[IRule, Long, Unit, Unit, String, String](
        true,
        true,
        interner,
        (rule: IRule) => rule.allPuzzles,
        (rule: IRule) => rule.allRunes.toVector,
        new TestRuleSolver(interner),
        List(RangeS.testZero(interner)),
        rules,
        Map(),
        rules.flatMap(_.allRunes).distinct.toVector)
    while ( {
      solver.advance(Unit, Unit) match {
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
        initially_known_runes: std::collections::HashMap<i64, String>,
    ) -> std::collections::HashMap<i64, String> {
        use crate::solver::Solver;
        use crate::utils::range::RangeS;
        use bumpalo::Bump;

        let arena = Bump::new();
        let interner = crate::Interner::with_arena(&arena);
        let all_runes_from_rules: std::collections::HashSet<i64> =
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
        let delegate = super::test_rule_solver::TestRuleSolver {
            interner: &interner,
        };
        let mut solver = Solver::new(
            true,
            delegate,
            vec![RangeS::test_zero(&interner)],
            rules,
            initially_known_runes,
            all_runes,
        );

        while solver.advance(&(), &()).expect("advance") {}

        let conclusions: std::collections::HashMap<i64, String> =
            solver.userify_conclusions().into_iter().collect();
        let conclusions_keys: std::collections::HashSet<i64> =
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

    val solver =
      new Solver[IRule, Long, Unit, Unit, String, String](
        true,
        true,
        interner,
        (r: IRule) => r.allPuzzles,
        (rule: IRule) => rule.allRunes.toVector,
        new TestRuleSolver(interner),
        List(RangeS.testZero(interner)),
        rules,
        initiallyKnownRunes,
        (rules.flatMap(_.allRunes) ++ initiallyKnownRunes.keys).distinct.toVector)

    while ( {
      solver.advance(Unit, Unit) match {
        case Ok(continue) => continue
        case Err(e) => vfail(e)
      }
    }) {}
    // If we get here, then there's nothing more the solver can do.
    val conclusionsMap = solver.userifyConclusions().toMap

    vassert(expectCompleteSolve == (conclusionsMap.keySet == rules.flatMap(_.allRunes).toSet))
    conclusionsMap
  }
}
*/