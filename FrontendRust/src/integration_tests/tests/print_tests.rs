/*
package dev.vale

import org.scalatest._
*/
// mig: struct PrintTests
pub struct PrintTests;
/*
class PrintTests extends FunSuite with Matchers {
*/
// mig: fn printlning_an_int
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn printlning_an_int() {
    panic!("Unmigrated test: printlning_an_int");
}
/*
  test("Println'ing an int") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |exported func main() {
        |  println(6);
        |}
      """.stripMargin)

    compile.evalForStdout(Vector()) shouldEqual "6\n"
  }
*/
// mig: fn printlning_a_bool
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn printlning_a_bool() {
    panic!("Unmigrated test: printlning_a_bool");
}
/*
  test("Println'ing a bool") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |exported func main() {
        |  println(true);
        |}
      """.stripMargin)

    compile.evalForStdout(Vector()) shouldEqual "true\n"
  }
*/
/*
}
*/
