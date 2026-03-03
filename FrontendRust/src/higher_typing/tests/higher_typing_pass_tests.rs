/*
TODO: rename

package dev.vale.highertyping

import dev.vale.{Err, Ok, PackageCoordinate, vassertSome, vfail}
import dev.vale.postparsing._
import dev.vale.parsing.Parser
import dev.vale.postparsing.RuneTypeSolveError
import dev.vale._
import org.scalatest._

class HigherTypingPassTests extends FunSuite with Matchers  {
*/
use crate::higher_typing::HigherTypingCompilation;
use crate::higher_typing::astronomer_error_reporter::ICompileErrorA;

// mig: fn compile_program_for_error
fn compile_program_for_error<'a>(compilation: HigherTypingCompilation<'a, '_, '_, '_>) -> Box<dyn ICompileErrorA<'a> + 'a> {
    panic!("Unimplemented: compile_program_for_error");
}
/*
  def compileProgramForError(compilation: HigherTypingCompilation): ICompileErrorA = {
    compilation.getAstrouts() match {
      case Ok(result) => vfail("Expected error, but actually parsed invalid program:\n" + result)
      case Err(err) => err
    }
  }
*/
// mig: fn type_simple_main_function
#[test]
fn type_simple_main_function() {
    panic!("Unmigrated test: type_simple_main_function");
}
/*
  test("Type simple main function") {
    val compilation =
      HigherTypingTestCompilation.test(
      """exported func main() {
        |}
        |""".stripMargin)
    val astrouts = compilation.expectAstrouts()
  }
*/
// mig: fn type_simple_generic_function
#[test]
fn type_simple_generic_function() {
    panic!("Unmigrated test: type_simple_generic_function");
}
/*
  test("Type simple generic function") {
    val compilation =
      HigherTypingTestCompilation.test(
        """exported func moo<T>() where T Ref {
          |}
          |""".stripMargin)
    val astrouts = compilation.expectAstrouts()
  }
*/
// mig: fn infer_coord_type_from_parameters
#[test]
fn infer_coord_type_from_parameters() {
    panic!("Unmigrated test: infer_coord_type_from_parameters");
}
/*
  test("Infer coord type from parameters") {
    val compilation =
      HigherTypingTestCompilation.test(
        """exported func moo<T>(x T) {
          |}
          |""".stripMargin)
    val astrouts = compilation.expectAstrouts()
    val program = vassertSome(astrouts.get(PackageCoordinate.TEST_TLD(compilation.interner, compilation.keywords)))
    val main = program.lookupFunction("moo")
    main.runeToType(CodeRuneS(compilation.keywords.T)) shouldEqual CoordTemplataType()
  }
*/
// mig: fn type_simple_struct
#[test]
fn type_simple_struct() {
    panic!("Unmigrated test: type_simple_struct");
}
/*
  test("Type simple struct") {
    val compilation =
      HigherTypingTestCompilation.test(
        """struct Moo {
          |}
          |""".stripMargin)
    val astrouts = compilation.expectAstrouts()
  }
*/
// mig: fn type_simple_generic_struct
#[test]
fn type_simple_generic_struct() {
    panic!("Unmigrated test: type_simple_generic_struct");
}
/*
  test("Type simple generic struct") {
    val compilation =
      HigherTypingTestCompilation.test(
        """struct Moo<T> {
          |  bork T;
          |}
          |""".stripMargin)
    val astrouts = compilation.expectAstrouts()
  }
*/
// mig: fn template_call_recursively_evaluate
#[test]
fn template_call_recursively_evaluate() {
    panic!("Unmigrated test: template_call_recursively_evaluate");
}
/*
  test("Template call, recursively evaluate") {
    val compilation =
      HigherTypingTestCompilation.test(
        """struct Moo<T> {
          |  bork T;
          |}
          |struct Bork<T> {
          |  x Moo<T>;
          |}
          |""".stripMargin)
    val astrouts = compilation.expectAstrouts()
    val program = vassertSome(astrouts.get(PackageCoordinate.TEST_TLD(compilation.interner, compilation.keywords)))
    val main = program.lookupStruct("Bork")
    main.headerRuneToType(CodeRuneS(compilation.keywords.T)) shouldEqual CoordTemplataType()
  }
*/
// mig: fn type_simple_interface
#[test]
fn type_simple_interface() {
    panic!("Unmigrated test: type_simple_interface");
}
/*
  test("Type simple interface") {
    val compilation =
      HigherTypingTestCompilation.test(
        """interface Moo {
          |}
          |""".stripMargin)
    val astrouts = compilation.expectAstrouts()
  }
*/
// mig: fn type_simple_generic_interface
#[test]
fn type_simple_generic_interface() {
    panic!("Unmigrated test: type_simple_generic_interface");
}
/*
  test("Type simple generic interface") {
    val compilation =
      HigherTypingTestCompilation.test(
        """interface Moo<T> where T Ref {
          |}
          |""".stripMargin)
    val astrouts = compilation.expectAstrouts()
  }
*/
// mig: fn type_simple_generic_interface_method
#[test]
fn type_simple_generic_interface_method() {
    panic!("Unmigrated test: type_simple_generic_interface_method");
}
/*
  test("Type simple generic interface method") {
    val compilation =
      HigherTypingTestCompilation.test(
        """interface Moo<T> where T Ref {
          |  func bork(virtual self &Moo<T>) int;
          |}
          |""".stripMargin)
    val astrouts = compilation.expectAstrouts()
  }
*/
// mig: fn infer_generic_type_through_param_type_template_call
#[test]
fn infer_generic_type_through_param_type_template_call() {
    panic!("Unmigrated test: infer_generic_type_through_param_type_template_call");
}
/*
  test("Infer generic type through param type template call") {
    val compilation =
      HigherTypingTestCompilation.test(
        """struct List<T> {
          |  moo T;
          |}
          |exported func moo<T>(x List<T>) {
          |}
          |""".stripMargin)
    val astrouts = compilation.expectAstrouts()
    val program = vassertSome(astrouts.get(PackageCoordinate.TEST_TLD(compilation.interner, compilation.keywords)))
    val main = program.lookupFunction("moo")
    main.runeToType(CodeRuneS(compilation.keywords.T)) shouldEqual CoordTemplataType()
  }
*/
// mig: fn test_evaluate_pack
#[test]
fn test_evaluate_pack() {
    panic!("Unmigrated test: test_evaluate_pack");
}
/*
  test("Test evaluate Pack") {
    val compilation =
      HigherTypingTestCompilation.test(
        """func moo<T RefList>()
          |where T = Refs(int, bool)
          |{
          |}
          |""".stripMargin)
    val astrouts = compilation.expectAstrouts()
    val program = vassertSome(astrouts.get(PackageCoordinate.TEST_TLD(compilation.interner, compilation.keywords)))
    val main = program.lookupFunction("moo")
    main.runeToType(CodeRuneS(compilation.keywords.T)) shouldEqual PackTemplataType(CoordTemplataType())
  }
*/
// mig: fn test_infer_pack_from_result
#[test]
fn test_infer_pack_from_result() {
    panic!("Unmigrated test: test_infer_pack_from_result");
}
/*
  test("Test infer Pack from result") {
    val compilation =
      HigherTypingTestCompilation.test(
        """func moo<T>()
          |where func moo(T, bool)str
          |{
          |}
          |""".stripMargin)
    val astrouts = compilation.expectAstrouts()
    val program = vassertSome(astrouts.get(PackageCoordinate.TEST_TLD(compilation.interner, compilation.keywords)))
    val main = program.lookupFunction("moo")
    main.runeToType(CodeRuneS(compilation.keywords.T)) shouldEqual CoordTemplataType()
  }
*/
// mig: fn test_infer_pack_from_empty_result
#[test]
fn test_infer_pack_from_empty_result() {
    panic!("Unmigrated test: test_infer_pack_from_empty_result");
}
/*
  test("Test infer Pack from empty result") {
    val compilation =
      HigherTypingTestCompilation.test(
        """func moo<P RefList>()
          |where P = Refs(), Prot[P, str]
          |{
          |}
          |""".stripMargin)
    val astrouts = compilation.expectAstrouts()
    val program = vassertSome(astrouts.get(PackageCoordinate.TEST_TLD(compilation.interner, compilation.keywords)))
    val main = program.lookupFunction("moo")
    main.runeToType(CodeRuneS(compilation.interner.intern(StrI("P")))) shouldEqual PackTemplataType(CoordTemplataType())
  }

//  test("Test cant solve empty Pack") {
//    val compilation =
//      AstronomerTestCompilation.test(
//        """func moo<P>()
//          |where P = ()
//          |{
//          |}
//          |""".stripMargin)
//    compilation.getAstrouts() match {
//      case Err(CouldntSolveRulesA(_, RuneTypeSolveError(range, IncompleteSolve(incompleteConclusions, unsolvedRules, unknownRunes)))) => {
//        vassert(unknownRunes.contains(CodeRuneS(StrI("P"))))
//      }
//    }
//  }

}
*/