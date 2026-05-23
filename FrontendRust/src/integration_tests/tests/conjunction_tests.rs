// mig: struct ConjunctionTests
pub struct ConjunctionTests;
/*
package dev.vale

import dev.vale.von.VonBool
import org.scalatest._

class ConjunctionTests extends FunSuite with Matchers {
*/
// mig: fn and
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn and() { panic!("Unmigrated test: and"); }
/*
  test("And") {
    val compile = RunCompilation.test("exported func main() bool { return true and true; }")
    compile.evalForKind(Vector()) match { case VonBool(true) => }
  }
*/
// mig: fn or
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn or() { panic!("Unmigrated test: or"); }
/*
  test("Or") {
    val compile = RunCompilation.test("exported func main() bool { return true or false; }")
    compile.evalForKind(Vector()) match { case VonBool(true) => }
  }
*/
// mig: fn and_short_circuiting
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn and_short_circuiting() { panic!("Unmigrated test: and_short_circuiting"); }
/*
  test("And short-circuiting") {
    val compile = RunCompilation.test(
      """
        |func printAndFalse() bool { print("bork!"); return false; }
        |exported func main() bool { return printAndFalse() and printAndFalse(); }
        |""".stripMargin)

    compile.evalForStdout(Vector()) shouldEqual "bork!"
  }
*/
// mig: fn or_short_circuiting
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn or_short_circuiting() { panic!("Unmigrated test: or_short_circuiting"); }
/*
  test("Or short-circuiting") {
    val compile = RunCompilation.test(
      """
        |func printAndTrue() bool { print("bork!"); return true; }
        |exported func main() bool { return printAndTrue() or printAndTrue(); }
        |""".stripMargin)

    compile.evalForStdout(Vector()) shouldEqual "bork!"
  }
}

*/
