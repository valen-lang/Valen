/*
package dev.vale.highlighter

import dev.vale.lexing.RangeL
import dev.vale.options.GlobalOptions
import dev.vale.parsing.{ParserCompilation, ast}
import dev.vale.{Err, FileCoordinateMap, Interner, Keywords, Ok, PackageCoordinate}
import dev.vale.parsing.ast.FileP
import dev.vale.parsing.{ast, _}
import org.scalatest._

class SpannerTests extends FunSuite with Matchers {
*/

// mig: fn compile
fn compile() { panic!("Unimplemented: compile"); }
/*
  private def compile(code: String): FileP = {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val compilation =
      new ParserCompilation(
        GlobalOptions(true, true, true, true, true),
        interner,
        keywords,
        Vector(PackageCoordinate.TEST_TLD(interner, keywords)),
        FileCoordinateMap.test(interner, code))
    compilation.getParseds() match {
      case Err(err) => fail(err.toString)
      case Ok(program0) => program0.expectOne()._1
    }
  }
*/

// mig: fn spanner_simple_function
#[test]
#[ignore = "unmigrated - pending highlighter body migration"]
fn spanner_simple_function() { panic!("Unmigrated test: spanner_simple_function"); }
/*
  test("Spanner simple function") {
    val program1 = compile("func main() int { 3 }")
    val main = program1.lookupFunction("main")
    Spanner.forFunction(main) shouldEqual
      Span(Fn,RangeL(0,21),Vector(
        Span(FnName,RangeL(5,9),Vector.empty),
        Span(Params,RangeL(9,11),Vector.empty),
        Span(Ret,RangeL(12,16),Vector(Span(Typ,RangeL(12,15),Vector.empty))),
        Span(Block,RangeL(16,21),Vector(
          Span(Num,RangeL(18,19),Vector.empty)))))
  }
*/

// mig: fn spanner_map_call
#[test]
#[ignore = "unmigrated - pending highlighter body migration"]
fn spanner_map_call() { panic!("Unmigrated test: spanner_map_call"); }
/*
  test("Spanner map call") {
    val program1 = compile(
      """func main() int {
        |  this.abilities.getImpulse();
        |}
        |""".stripMargin)
    val main = program1.lookupFunction("main")
    Spanner.forFunction(main) match {
      case Span(
        Fn,_,
        Vector(
          Span(FnName,_,Vector()),
          Span(Params,_,Vector()),
          Span(Ret,RangeL(12,16),Vector(Span(Typ,RangeL(12,15),Vector()))),
          Span(Block,_,
            Vector(
              Span(Consecutor,_,
                Vector(
                  Span(Call,_,
                    Vector(
                      Span(MemberAccess,_,
                        Vector(
                          Span(Lookup,_,Vector()),
                          Span(MemberAccess,_,Vector()),
                          Span(Lookup,_,Vector()))),
                      Span(MemberAccess,_,Vector()),
                      Span(CallLookup,_,Vector()))))))))) =>
    }
  }
}
*/
