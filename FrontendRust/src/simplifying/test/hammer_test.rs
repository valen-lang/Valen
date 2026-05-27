// From Frontend/SimplifyingPass/test/dev/vale/simplifying/HammerTest.scala
/*
package dev.vale.simplifying

import dev.vale.finalast.StackifyH
import dev.vale.{Collector, PackageCoordinate, vassert}
import dev.vale.parsing.ast.FileP
import dev.vale.highertyping.ICompileErrorA
import dev.vale.Result
import dev.vale.typing._
import org.scalatest._
import dev.vale.finalast.VariableIdH
import dev.vale.postparsing.ICompileErrorS

import scala.collection.immutable.List


*/
// mig: struct HammerTest
pub struct HammerTest {
}

// mig: impl HammerTest
// (impl block suppressed per simplifying-pass policy — test fns emitted at module scope)

/*
class HammerTest extends FunSuite with Matchers with Collector {
*/
// mig: fn local_ids_unique
#[test]
#[ignore = "unmigrated - pending simplifying-pass body migration"]
fn local_ids_unique() {
    panic!("Unmigrated test: local_ids_unique");
}

/*
  test("Local IDs unique") {
    val compile = HammerTestCompilation.test(
        """
          |exported func main() {
          |  a = 6;
          |  if (true) {
          |    b = 7;
          |    c = 8;
          |  } else {
          |    while (false) {
          |      d = 9;
          |    }
          |    e = 10;
          |  }
          |  f = 11;
          |}
          |""".stripMargin)
    val hamuts = compile.getHamuts()
    val paackage = hamuts.lookupPackage(PackageCoordinate.TEST_TLD(compile.interner, compile.keywords))
    val main = paackage.lookupFunction("main")

    vassert(paackage.exportNameToFunction.exists(_._2 == main.prototype))

    val stackifies = collect(main, { case s @ StackifyH(_, _, _) => s })
    val localIds = stackifies.map(_.local.id.number).toVector.sorted
    localIds shouldEqual localIds.distinct.toVector
    vassert(localIds.size >= 6)
  }
}

*/
// NOVEL CODE: no Scala counterpart. Minimal end-to-end driving test for hammer
// body migration — the simplest possible program (return an int; no control flow,
// no generics), modeled structurally on `local_ids_unique` above. Running it drives
// the hammer pipeline from the first `panic!()` outward (currently
// `HammerTestCompilation::test`). As the harness + entry points get real bodies,
// grow this to mirror local_ids_unique's shape: get_hamuts() →
// lookup_package(TEST_TLD) → lookup_function("main") → assert the export exists.
#[test]
fn returns_int() {
    use super::test_compilation::test;
    let _compile = test("exported func main() int { return 7; }");
}
