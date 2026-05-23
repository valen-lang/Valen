/*
package dev.vale

import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing._
import dev.vale.typing.types._
import dev.vale.von.VonInt
import org.scalatest._
*/
// mig: struct FloatTests
pub struct FloatTests;
/*
class FloatTests extends FunSuite with Matchers {
*/
// mig: fn print_float
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn print_float() { panic!("Unmigrated test: print_float"); }
/*
  test("Print float") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |
        |exported func main() {
        |  a = 42.125;
        |  print(a);
        |}
      """.stripMargin)

    compile.evalForStdout(Vector()).trim() shouldEqual "42.125"
  }
*/
// mig: fn float_arithmetic
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn float_arithmetic() { panic!("Unmigrated test: float_arithmetic"); }
/*
  test("Float arithmetic") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/floatarithmetic.vale"))

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn float_equals
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn float_equals() { panic!("Unmigrated test: float_equals"); }
/*
  test("Float equals") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/floateq.vale"))

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn concat_string_and_float
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn concat_string_and_float() { panic!("Unmigrated test: concat_string_and_float"); }
/*
  test("Concat string and float") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/concatstrfloat.vale"))

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
}

*/
