// mig: struct BlockTests
pub struct BlockTests;

/*
package dev.vale

import dev.vale.postparsing._
import dev.vale.postparsing._
import dev.vale.typing._
import dev.vale.typing.types.BoolT
import dev.vale.von.VonInt
import org.scalatest._

class BlockTests extends FunSuite with Matchers {
*/
// mig: fn empty_block
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn empty_block() {
    panic!("Unmigrated test: empty_block");
}

/*
  test("Empty block") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  block {
        |  }
        |  return 3;
        |}
      """.stripMargin)
    val fileCoord =
      compile.interner.intern(FileCoordinate(
        compile.interner.intern(PackageCoordinate(
          compile.interner.intern(StrI("test")),
          Vector.empty)),
        "0.vale"))
    val scoutput = compile.getScoutput().getOrDie().fileCoordToContents(fileCoord)
    val main = scoutput.lookupFunction("main")
    main.body match { case CodeBodyS(BodySE(_, _,BlockSE(_, _,ConsecutorSE(Vector(BlockSE(_, _,_), _))))) => }

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_block_with_a_variable
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_block_with_a_variable() {
    panic!("Unmigrated test: simple_block_with_a_variable");
}

/*
  test("Simple block with a variable") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  block {
        |    y = 6;
        |  }
        |  return 3;
        |}
      """.stripMargin)
    val fileCoord =
      compile.interner.intern(FileCoordinate(
        compile.interner.intern(PackageCoordinate(
          compile.interner.intern(StrI("test")),
          Vector.empty)),
        "0.vale"))
    val scoutput = compile.getScoutput().getOrDie().fileCoordToContents(fileCoord)
    val main = scoutput.lookupFunction("main")
    val block = main.body match { case CodeBodyS(BodySE(_, _,BlockSE(_, _, ConsecutorSE(Vector(b @ BlockSE(_, _,_), _))))) => b }
    vassert(block.locals.size == 1)
    block.locals.head match {
      case LocalS(CodeVarNameS(StrI("y")), NotUsed, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_block_with_a_variable_another_variable_outside_with_same_name
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_block_with_a_variable_another_variable_outside_with_same_name() {
    panic!("Unmigrated test: simple_block_with_a_variable_another_variable_outside_with_same_name");
}

/*
  test("Simple block with a variable, another variable outside with same name") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  block {
        |    y = 6;
        |  }
        |  y = 3;
        |  return y;
        |}
      """.stripMargin)
    val scoutput = compile.getScoutput().getOrDie()

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
}

*/
