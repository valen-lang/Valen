// mig: struct WhileTests
pub struct WhileTests;
/*
package dev.vale

import dev.vale.von.VonInt
import org.scalatest._

class WhileTests extends FunSuite with Matchers {
*/
// mig: fn simple_while_loop_that_doesnt_execute
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_while_loop_that_doesnt_execute() {
    panic!("Unmigrated test: simple_while_loop_that_doesnt_execute");
}
/*
  test("Simple while loop that doesnt execute") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while (false) {}
        |  return 5;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn test_a_for_ish_while_loop
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn test_a_for_ish_while_loop() {
    panic!("Unmigrated test: test_a_for_ish_while_loop");
}
/*
  test("Test a for-ish while loop") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  i = 0;
        |  while (i < 4) {
        |    set i = i + 1;
        |  }
        |  return i;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(4) => }
  }
*/
// mig: fn tests_a_while_loop_with_a_complex_condition
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_a_while_loop_with_a_complex_condition() {
    panic!("Unmigrated test: tests_a_while_loop_with_a_complex_condition");
}
/*
  test("Tests a while loop with a complex condition") {
    val compile = RunCompilation.test(
      """import ioutils.*;
        |import printutils.*;
        |exported func main() int {
        |  key = 0;
        |  while set key = __getch(); key < 96 {
        |    print(key);
        |  }
        |  return key;
        |}
      """.stripMargin)

    compile.evalForKind(Vector(), Vector("A", "B", "c")) match { case VonInt(99) => }
  }
*/
// mig: fn tests_a_while_loop_with_a_set_in_it
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_a_while_loop_with_a_set_in_it() {
    panic!("Unmigrated test: tests_a_while_loop_with_a_set_in_it");
}
/*
  test("Tests a while loop with a set in it") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |import ioutils.*;
        |import logic.*;
        |
        |exported func main() int {
        |  key = 0;
        |  while set key = __getch(); key != 99 {
        |    print(key);
        |  }
        |  return key;
        |}
      """.stripMargin)

    compile.evalForKind(Vector(), Vector("A", "B", "c")) match { case VonInt(99) => }
  }
*/
// mig: fn tests_a_while_loop_with_a_declaration_in_it
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_a_while_loop_with_a_declaration_in_it() {
    panic!("Unmigrated test: tests_a_while_loop_with_a_declaration_in_it");
}
/*
  test("Tests a while loop with a declaration in it") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |import ioutils.*;
        |import logic.*;
        |
        |exported func main() {
        |  while key = __getch(); key != 99 {
        |    print(key);
        |  }
        |}
      """.stripMargin)

    compile.evalForKind(Vector(), Vector("A", "B", "c"))
  }
*/
// mig: fn return_from_infinite_while_loop
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn return_from_infinite_while_loop() {
    panic!("Unmigrated test: return_from_infinite_while_loop");
}
/*
  test("Return from infinite while loop") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while (true) {
        |    return 9;
        |  }
        |  return __vbi_panic();
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn infinite_while_loop_conditional_break
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn infinite_while_loop_conditional_break() {
    panic!("Unmigrated test: infinite_while_loop_conditional_break");
}
/*
  test("Infinite while loop conditional break") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while true {
        |    if true {
        |      break;
        |    }
        |    4;
        |  }
        |  return 42;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn infinite_while_loop_unconditional_break
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn infinite_while_loop_unconditional_break() {
    panic!("Unmigrated test: infinite_while_loop_unconditional_break");
}
/*
  test("Infinite while loop unconditional break") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while true {
        |    break;
        |  }
        |  return 42;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn infinite_while_loop_conditional_break_from_both_sides
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn infinite_while_loop_conditional_break_from_both_sides() {
    panic!("Unmigrated test: infinite_while_loop_conditional_break_from_both_sides");
}
/*
  test("Infinite while loop conditional break from both sides") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while true {
        |    if true {
        |      break;
        |    } else {
        |      break;
        |    }
        |  }
        |  return 42;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn infinite_while_loop_conditional_return
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn infinite_while_loop_conditional_return() {
    panic!("Unmigrated test: infinite_while_loop_conditional_return");
}
/*
  test("Infinite while loop conditional return") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while true {
        |    if true {
        |      return 42;
        |    }
        |    73;
        |  }
        |  return 74;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn infinite_while_loop_unconditional_return
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn infinite_while_loop_unconditional_return() {
    panic!("Unmigrated test: infinite_while_loop_unconditional_return");
}
/*
  test("Infinite while loop unconditional return") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while true {
        |    return 42;
        |  }
        |  return 73;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn infinite_while_loop_conditional_return_from_both_sides
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn infinite_while_loop_conditional_return_from_both_sides() {
    panic!("Unmigrated test: infinite_while_loop_conditional_return_from_both_sides");
}
/*
  test("Infinite while loop conditional return from both sides") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while true {
        |    if true {
        |      return 42;
        |    } else {
        |      return 73;
        |    }
        |  }
        |  return 74;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn while_with_condition_declaration
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn while_with_condition_declaration() {
    panic!("Unmigrated test: while_with_condition_declaration");
}
/*
  test("While with condition declaration") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while x = 42; x < 50 { return x; }
        |  return 73;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn each_on_int_range
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn each_on_int_range() {
    panic!("Unmigrated test: each_on_int_range");
}
/*
  test("each on int range") {
    val compile = RunCompilation.test(
      """
        |import intrange.*;
        |
        |exported func main() int {
        |  sum = 0;
        |  foreach i in 0..10 {
        |    set sum = sum + i;
        |  }
        |  return sum;
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(45) => }
  }
*/
// mig: fn parallel_foreach
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn parallel_foreach() {
    panic!("Unmigrated test: parallel_foreach");
}
/*
  test("Parallel foreach") {
    val compile = RunCompilation.test(
      """
        |import intrange.*;
        |import list.*;
        |import listprintutils.*;
        |
        |exported func main() {
        |  exponent = 3;
        |
        |  results =
        |    parallel foreach i in 0..5 {
        |      i + 1
        |    };
        |
        |  println(&results);
        |}
        |""".stripMargin)
    compile.evalForStdout(Vector()).trim shouldEqual "[1, 2, 3, 4, 5]"
  }
*/
// mig: fn mutable_foreach
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn mutable_foreach() {
    panic!("Unmigrated test: mutable_foreach");
}
/*
  test("Mutable foreach") {
    val compile = RunCompilation.test(
      """
        |// A fake 1-element list
        |struct Ship {
        |  fuel! int;
        |}
        |struct List {
        |  ship Ship;
        |}
        |
        |struct ListIter {
        |  ship &Ship;
        |  pos! int;
        |}
        |func begin(self &List) ListIter { ListIter(&self.ship, 0) }
        |func next(iter &ListIter) Opt<&Ship> {
        |  if pos = set iter.pos = iter.pos + 1; pos < 1 {
        |    Some<&Ship>(iter.ship)
        |  } else {
        |    None<&Ship>()
        |  }
        |}
        |
        |exported func main() int {
        |  list = List(Ship(73));
        |  foreach i in &list {
        |    set i.fuel = 42;
        |  }
        |  return list.ship.fuel;
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn each_on_int_range_with_conditional_break
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn each_on_int_range_with_conditional_break() {
    panic!("Unmigrated test: each_on_int_range_with_conditional_break");
}
/*
  test("each on int range with conditional break") {
    val compile = RunCompilation.test(
      """
        |import intrange.*;
        |import list.*;
        |
        |exported func main() int {
        |  sum = 0;
        |  results =
        |    foreach i in 0..10 {
        |      if true {
        |        break;
        |      }
        |      3
        |    };
        |  return 0;
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(0) => }
  }
*/
// mig: fn each_on_int_range_with_unconditional_break
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn each_on_int_range_with_unconditional_break() {
    panic!("Unmigrated test: each_on_int_range_with_unconditional_break");
}
/*
  test("each on int range with unconditional break") {
    val compile = RunCompilation.test(
      """
        |import intrange.*;
        |
        |exported func main() int {
        |  sum = 0;
        |  foreach i in 0..10 {
        |    break;
        |  }
        |  return sum;
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(0) => }
  }
*/
// mig: fn each_on_int_range_with_conditional_break_from_both_branches
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn each_on_int_range_with_conditional_break_from_both_branches() {
    panic!("Unmigrated test: each_on_int_range_with_conditional_break_from_both_branches");
}
/*
  test("each on int range with conditional break from both branches") {
    val compile = RunCompilation.test(
      """
        |import intrange.*;
        |
        |exported func main() int {
        |  sum = 0;
        |  foreach i in 0..10 {
        |    if true {
        |      break;
        |    } else {
        |      break;
        |    }
        |  }
        |  return sum;
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(0) => }
  }

  //
//  test("Tests a while loop with a move in it") {
//    val compile = RunCompilation.test(
//      """
//        |func doThings(m: Marine) { }
//        |struct Marine { hp: int; }
//        |exported func main() int {
//        |  m = Marine(7);
//        |  while (true) {
//        |    doThings(m);
//        |  }
//        |  return 4;
//        |}
//      """.stripMargin)
//
//    // should fail
//
//    compile.evalForKind(Vector()) match { case VonInt(4) => }
//  }
}

*/
