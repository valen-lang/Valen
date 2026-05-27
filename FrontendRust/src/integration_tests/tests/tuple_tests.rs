/*
package dev.vale

import dev.vale.typing.ast.TupleTE
import dev.vale.typing.types.IntT
import dev.vale.typing._
import dev.vale.von.{VonBool, VonInt}
import org.scalatest._
*/
// mig: struct TupleTests
pub struct TupleTests;
/*
class TupleTests extends FunSuite with Matchers {
*/
// mig: fn returning_tuple_from_function_and_dotting_it
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn returning_tuple_from_function_and_dotting_it() {
    panic!("Unmigrated test: returning_tuple_from_function_and_dotting_it");
}
/*
  test("Returning tuple from function and dotting it") {
    val compile = RunCompilation.test(
      """
        |import v.builtins.tup2.*;
        |import v.builtins.drop.*;
        |
        |func makeTup() (int, int) { return (2, 3); }
        |exported func main() int {
        |  return makeTup().1;
        |}
      """.stripMargin, false)

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn tuple_with_two_things
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tuple_with_two_things() {
    panic!("Unmigrated test: tuple_with_two_things");
}
/*
  test("Tuple with two things") {
    val compile = RunCompilation.test(
      """
        |import v.builtins.tup2.*;
        |import v.builtins.drop.*;
        |
        |exported func main() bool {
        |  return (9, true).1;
        |}
        |""".stripMargin, false)
    compile.evalForKind(Vector()) match { case VonBool(true) => }
  }
*/
// mig: fn tuple_type
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tuple_type() {
    panic!("Unmigrated test: tuple_type");
}
/*
  test("Tuple type") {
    val compile = RunCompilation.test(
      """
        |import v.builtins.tup2.*;
        |import v.builtins.drop.*;
        |
        |func moo(a (int, int)) int { return a.1; }
        |
        |exported func main() int {
        |  return moo((3, 4));
        |}
        |""".stripMargin, false)
    compile.evalForKind(Vector()) match { case VonInt(4) => }
  }
*/
// mig: fn simple_tuple_with_one_int
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_tuple_with_one_int() {
    panic!("Unmigrated test: simple_tuple_with_one_int");
}
/*
  test("Simple tuple with one int") {
    val compile = RunCompilation.test("exported func main() int { return (9,).0; }")

    val coutputs = compile.expectCompilerOutputs()
    coutputs.lookupFunction("main").header.returnType.kind shouldEqual IntT.i32
    // Funny story, theres no such thing as a one element tuple! It becomes a one element array.
    Collector.only(coutputs.lookupFunction("main"), { case TupleTE(_, _) => })

    compile.evalForKind(Vector()) match {
      case VonInt(9) =>
    }
  }
}

*/
