// mig: struct OptTests
pub struct OptTests;
/*
package dev.vale

import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing._
import dev.vale.typing.types._
import dev.vale.von.VonInt
import org.scalatest._

class OptTests extends FunSuite with Matchers {
*/
// mig: fn test_empty_and_get_for_some
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn test_empty_and_get_for_some() { panic!("Unmigrated test: test_empty_and_get_for_some"); }
/*
  test("Test empty and get for Some") {
    val compile = RunCompilation.testNoBuiltins(
        """
          |import v.builtins.opt.*;
          |
          |exported func main() int {
          |  opt Opt<int> = Some(9);
          |  return if (opt.isEmpty()) { 0 }
          |    else { opt.get() };
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn test_empty_and_get_for_none
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn test_empty_and_get_for_none() { panic!("Unmigrated test: test_empty_and_get_for_none"); }
/*
  test("Test empty and get for None") {
    val compile = RunCompilation.test(
        """
          |exported func main() int {
          |  opt Opt<int> = None<int>();
          |  return if (opt.isEmpty()) { 0 }
          |    else { opt.get() };
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(0) => }
  }
*/
// mig: fn test_empty_and_get_for_borrow
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn test_empty_and_get_for_borrow() { panic!("Unmigrated test: test_empty_and_get_for_borrow"); }
/*
  test("Test empty and get for borrow") {
    val compile = RunCompilation.test(
        """
          |// This is the same as the one in optutils.vale, just named differently,
          |// so its easier to debug.
          |func borrowGet<T>(opt &Some<T>) &T { &opt.value }
          |
          |struct Spaceship { fuel int; }
          |exported func main() int {
          |  s = Spaceship(42);
          |  bork = Some<&Spaceship>(&s);
          |  return bork.borrowGet().fuel;
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
}

*/
