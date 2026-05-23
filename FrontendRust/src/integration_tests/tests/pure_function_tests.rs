/*
package dev.vale

import dev.vale.simplifying.VonHammer
import dev.vale.finalast.YonderH
import dev.vale.typing._
import dev.vale.typing.types.StrT
import dev.vale.testvm.StructInstanceV
import dev.vale.von.VonInt
import dev.vale.{finalast => m}
import org.scalatest._
*/
// mig: struct PureFunctionTests
pub struct PureFunctionTests;
/*
class PureFunctionTests extends FunSuite with Matchers {
*/
// mig: fn simple_pure_function
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_pure_function() {
    panic!("Unmigrated test: simple_pure_function");
}
/*
  test("Simple pure function") {
    val compile =
      RunCompilation.test(
        """
          |struct Engine {
          |  fuel int;
          |}
          |struct Spaceship {
          |  engine Engine;
          |}
          |pure func pfunc(s &Spaceship) int {
          |  return s.engine.fuel;
          |}
          |exported func main() int {
          |  s = Spaceship(Engine(10));
          |  return pfunc(&s);
          |}
          |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(10) => }
  }
}

*/
