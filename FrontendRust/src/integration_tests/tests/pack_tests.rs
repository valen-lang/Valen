/*
package dev.vale
//import dev.vale.typingpass.types.{IntT, PackTT}
import dev.vale.typing.ast.TupleTE
import dev.vale.von.VonInt
import org.scalatest._
*/
// mig: struct PackTests
pub struct PackTests;
/*
class PackTests extends FunSuite with Matchers {
*/
// mig: fn extract_seq
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn extract_seq() { panic!("Unmigrated test: extract_seq"); }
/*
  test("Extract seq") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  [x, y] = (5, 6);
        |  return x;
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.all(main, { case TupleTE(Vector(_, _), _) => }).size shouldEqual 1

    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn nested_seqs
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn nested_seqs() { panic!("Unmigrated test: nested_seqs"); }
/*
  test("Nested seqs") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  [x, [y, z]] = ((4, 5), (6, 7));
        |  return y;
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.all(main, {
      case TupleTE(
        Vector(
          TupleTE(Vector(_, _), _),
          TupleTE(Vector(_, _), _)),
        _) =>
    }).size shouldEqual 1

    compile.evalForKind(Vector()) match { case VonInt(6) => }
  }
*/
// mig: fn nested_tuples
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn nested_tuples() { panic!("Unmigrated test: nested_tuples"); }
/*
  test("Nested tuples") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  [x, [y, z]] = (5, (6, false));
        |  return x;
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.all(main, { case TupleTE(Vector(_, TupleTE(Vector(_, _), _)), _) => }).size shouldEqual 1

    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }

}

*/
