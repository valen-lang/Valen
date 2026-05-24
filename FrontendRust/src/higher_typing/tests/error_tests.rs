/*
package dev.vale.highertyping

import dev.vale.postparsing._
import dev.vale.{Err, Ok, SourceCodeUtils, StrI, vassert, vfail}
import org.scalatest._
import dev.vale.solver._

class ErrorTests extends FunSuite with Matchers  {
*/

// mig: fn compile_program_for_error
fn compile_program_for_error() { panic!("Unimplemented: compile_program_for_error"); }
/*
  def compileProgramForError(compilation: HigherTypingCompilation): ICompileErrorA = {
    compilation.getAstrouts() match {
      case Ok(result) => vfail("Expected error, but actually parsed invalid program:\n" + result)
      case Err(err) => err
    }
  }
*/

// mig: fn report_type_not_found
#[test]
#[ignore = "unmigrated - pending higher-typing-pass body migration"]
fn report_type_not_found() { panic!("Unmigrated test: report_type_not_found"); }
/*
  test("Report type not found") {
    val compilation =
      HigherTypingTestCompilation.test(
        """exported func main(a Bork) {
          |}
          |""".stripMargin)


    compileProgramForError(compilation) match {
      case e @ CouldntSolveRulesA(_,RuneTypeSolveError(_,FailedSolve(_,_,_,_,RuleError(RuneTypingCouldntFindType(_,CodeNameS(StrI("Bork"))))))) => {
        val codeMap = compilation.getCodeMap().getOrDie()
        val errorText =
          HigherTypingErrorHumanizer.humanize(
            SourceCodeUtils.humanizePos(codeMap, _),
            SourceCodeUtils.linesBetween(codeMap, _, _),
            SourceCodeUtils.lineRangeContaining(codeMap, _),
            SourceCodeUtils.lineContaining(codeMap, _),
            e)
        vassert(errorText.contains("Couldn't find anything with the name 'Bork'"))
      }
    }
  }

//  test("Report couldnt solve rules") {
//    val compilation =
//      HigherTypingTestCompilation.test(
//        """
//          |func moo<A>(x int) {
//          |  42
//          |}
//          |exported func main() {
//          |  moo();
//          |}
//          |""".stripMargin)
//
//    compileProgramForError(compilation) match {
//      case e @ CouldntSolveRulesA(range, failure) => {
//        val errorText = HigherTypingErrorHumanizer.humanize(compilation.getCodeMap().getOrDie(), e)
//        vassert(errorText.contains("olve"))
//      }
//    }
//  }
}
*/
