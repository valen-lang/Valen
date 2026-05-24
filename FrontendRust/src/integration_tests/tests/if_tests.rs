/*
package dev.vale

import dev.vale.postparsing._
import dev.vale.typing.ast.IfTE
import dev.vale.typing.types._
import dev.vale.testvm.IntV
import dev.vale.postparsing._
import dev.vale.typing._
import dev.vale.typing.types._
import dev.vale.von.{VonInt, VonStr}
import org.scalatest._

*/
// mig: struct IfTests
pub struct IfTests;
/*
class IfTests extends FunSuite with Matchers {
*/
// mig: fn simple_true_branch_returning_an_int
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_true_branch_returning_an_int() {
    panic!("Unmigrated test: simple_true_branch_returning_an_int");
}
/*
  test("Simple true branch returning an int") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  return if (true) { 3 } else { 5 };
        |}
      """.stripMargin)
    val programS =
      compile.getScoutput().getOrDie()
        .fileCoordToContents(
          compile.interner.intern(FileCoordinate(
            compile.interner.intern(PackageCoordinate(
              compile.interner.intern(StrI("test")),
              Vector.empty)),
            "0.vale")))
    val main = programS.lookupFunction("main")
    val ret = Collector.only(main.body, { case r @ ReturnSE(_, _) => r })
    val iff = Collector.only(ret, { case i @ IfSE(_, _, _, _) => i })
    Collector.only(iff.condition, { case ConstantBoolSE(_, true) => })
    Collector.only(iff.thenBody, { case ConstantIntSE(_, 3, _) => })
    Collector.only(iff.elseBody, { case ConstantIntSE(_, 5, _) => })

    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), { case IfTE(_, _, _) => })

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_false_branch_returning_an_int
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_false_branch_returning_an_int() {
    panic!("Unmigrated test: simple_false_branch_returning_an_int");
}
/*
  test("Simple false branch returning an int") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  return if (false) { 3 } else { 5 };
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn ladder
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn ladder() {
    panic!("Unmigrated test: ladder");
}
/*
  test("Ladder") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  return if (false) { 3 } else if (true) { 5 } else { 7 };
        |}
      """.stripMargin, false)

    val coutputs = compile.expectCompilerOutputs()
    val ifs = Collector.all(coutputs.lookupFunction("main"), { case if2 @ IfTE(_, _, _) => if2 })
    ifs.foreach(iff => iff.result.coord shouldEqual CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32))
    ifs.size shouldEqual 2
    val userFuncs = coutputs.getAllUserFunctions
    userFuncs.foreach(func => {
      func.header.returnType match {
        case CoordT(ShareT, _, IntT.i32) =>
        case CoordT(ShareT, _, BoolT()) =>
        case other => vwat(other)
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn moving_from_inside_if
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn moving_from_inside_if() {
    panic!("Unmigrated test: moving_from_inside_if");
}
/*
  test("Moving from inside if") {
    val compile = RunCompilation.test(
      """
        |struct Marine { x int; }
        |exported func main() int {
        |  m = Marine(5);
        |  return if (false) {
        |      [x] = m;
        |      x
        |    } else {
        |      [y] = m;
        |      y
        |    };
        |}
      """.stripMargin, false)

    val coutputs = compile.expectCompilerOutputs()
    val ifs = Collector.all(coutputs.lookupFunction("main"), { case if2 @ IfTE(_, _, _) => if2 })
    ifs.foreach(iff => iff.result.coord shouldEqual CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32))
    val userFuncs = coutputs.getAllUserFunctions
    userFuncs.foreach(func => {
      func.header.returnType match {
        case CoordT(ShareT, _, IntT.i32) =>
        case CoordT(ShareT, _, BoolT()) =>
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn if_with_complex_condition
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn if_with_complex_condition() {
    panic!("Unmigrated test: if_with_complex_condition");
}
/*
  test("If with complex condition") {
    val compile = RunCompilation.test(
      """
        |struct Marine { x int; }
        |exported func main() str {
        |  m = Marine(5);
        |  return if (m.x == 5) { "#" }
        |  else if (0 == 0) { "?" }
        |  else { "." };
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val ifs = Collector.all(coutputs.lookupFunction("main"), { case if2 @ IfTE(_, _, _) => if2 })
    ifs.foreach(iff => iff.result.coord shouldEqual CoordT(ShareT, RegionT(DefaultRegionT), StrT()))

    compile.evalForKind(Vector()) match { case VonStr("#") => }
  }
*/
// mig: fn if_with_condition_declaration
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn if_with_condition_declaration() {
    panic!("Unmigrated test: if_with_condition_declaration");
}
/*
  test("If with condition declaration") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  return if x = 42; x < 50 { x }
        |    else { 73 };
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn ret_from_inside_if_will_destroy_locals
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn ret_from_inside_if_will_destroy_locals() {
    panic!("Unmigrated test: ret_from_inside_if_will_destroy_locals");
}
/*
  test("Ret from inside if will destroy locals") {
    val compile = RunCompilation.test(
      """import printutils.*;
        |#!DeriveStructDrop
        |struct Marine { hp int; }
        |func drop(marine Marine) void {
        |  println("Destroying marine!");
        |  Marine[weapon] = marine;
        |}
        |exported func main() int {
        |  m = Marine(5);
        |  x =
        |    if (true) {
        |      println("In then!");
        |      return 7;
        |    } else {
        |      println("In else!");
        |      m.hp
        |    };
        |  println("In rest!");
        |  return x;
        |}
        |""".stripMargin)

    compile.evalForStdout(Vector()) shouldEqual "In then!\nDestroying marine!\n"
  }
*/
// mig: fn can_continue_if_other_branch_would_have_returned
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn can_continue_if_other_branch_would_have_returned() {
    panic!("Unmigrated test: can_continue_if_other_branch_would_have_returned");
}
/*
  test("Can continue if other branch would have returned") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |
        |#!DeriveStructDrop
        |struct Marine { hp int; }
        |func drop(marine Marine) void {
        |  println("Destroying marine!");
        |  Marine[weapon] = marine;
        |}
        |exported func main() int {
        |  m = Marine(5);
        |  x =
        |    if (false) {
        |      println("In then!");
        |      return 7;
        |    } else {
        |      println("In else!");
        |      m.hp
        |    };
        |  println("In rest!");
        |  return x;
        |}
        |""".stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    compile.evalForStdout(Vector()) shouldEqual "In else!\nIn rest!\nDestroying marine!\n"
  }
*/
// mig: fn destructure_inside_if
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn destructure_inside_if() {
    panic!("Unmigrated test: destructure_inside_if");
}
/*
  test("Destructure inside if") {
    val compile = RunCompilation.test(
      """import printutils.*;
        |struct Bork {
        |  num int;
        |}
        |struct Moo {
        |  bork Bork;
        |}
        |
        |exported func main() {
        |  zork = 0;
        |  while (zork < 4) {
        |    moo = Moo(Bork(5));
        |    if (true) {
        |      [bork] = moo;
        |      println(bork.num);
        |    } else {
        |      drop(moo);
        |    }
        |    set zork = zork + 1;
        |  }
        |}
        |""".stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    compile.evalForStdout(Vector()) shouldEqual "5\n5\n5\n5\n"
  }
*/
// mig: fn if_nevers
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn if_nevers() {
    panic!("Unmigrated test: if_nevers");
}
/*
  test("If nevers") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/if/ifnevers.vale"))
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn if_with_panics_and_rets
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn if_with_panics_and_rets() {
    panic!("Unmigrated test: if_with_panics_and_rets");
}
/*
  test("If with panics and rets") {
    val compile =
      RunCompilation.test(
        """
          |exported func main() int {
          |  a = 7;
          |  if false {
          |    panic("lol");
          |    return 73;
          |  } else {
          |    return 42;
          |  }
          |  return 73;
          |}
          |
          |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn toast
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn toast() {
    panic!("Unmigrated test: toast");
}
/*
  test("Toast") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  a = 0;
        |  if (a == 2) {
        |    return 71;
        |  } else if (a == 5) {
        |    return 73;
        |  } else {
        |    return 42;
        |  }
        |}
        |""".stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }

}

*/
