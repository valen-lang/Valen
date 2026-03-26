use bumpalo::Bump;
use crate::compile_options::GlobalOptions;
use crate::higher_typing::HigherTypingCompilation;
use crate::higher_typing::astronomer_error_reporter::ICompileErrorA;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::postparsing::itemplatatype::{CoordTemplataType, ITemplataType, PackTemplataType};
use crate::postparsing::names::{CodeRuneS, IRuneValS};
use crate::utils::code_hierarchy::{self, FileCoordinateMap, IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
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

// mig: fn compile_program_for_error
fn compile_program_for_error<'a, 'ctx, 'p, 's>(
    compilation: &mut HigherTypingCompilation<'a, 'ctx, 'p, 's>,
) -> ICompileErrorA<'a, 's>
where
    'a: 'ctx,
    'a: 'p,
    'a: 's,
{
    match compilation.get_astrouts() {
        Ok(result) => panic!("Expected error, but actually parsed invalid program:\n{:?}", result),
        Err(err) => err,
    }
}
/*
  def compileProgramForError(compilation: HigherTypingCompilation): ICompileErrorA = {
    compilation.getAstrouts() match {
      case Ok(result) => vfail("Expected error, but actually parsed invalid program:\n" + result)
      case Err(err) => err
    }
  }
*/
fn setup_test<'a, 'ctx, 'p, 's>(
    interner: &'ctx Interner<'a>,
    keywords: &'ctx Keywords<'a>,
    parser_arena: &'p Bump,
    scout_arena: &'s Bump,
    resolver: &'ctx dyn IPackageResolver<'a, HashMap<String, String>>,
) -> HigherTypingCompilation<'a, 'ctx, 'p, 's> {
    let options = GlobalOptions {
        sanity_check: true,
        use_overload_index: true,
        use_optimized_solver: true,
        verbose_errors: false,
        debug_output: false,
    };
    let test_module = interner.intern("test");
    let test_tld_ref = interner.intern_package_coordinate(test_module, &[]);
    HigherTypingCompilation::new(
        interner,
        keywords,
        vec![test_tld_ref],
        resolver,
        options,
        parser_arena,
        scout_arena,
    )
}

// mig: fn type_simple_main_function
#[test]
fn type_simple_main_function() {
    let interner_arena = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = Bump::new();
    let interner = Interner::with_arena(&interner_arena);
    let keywords = Keywords::new(&interner);
    let resolver = code_hierarchy::test_from_vec(&interner, vec!["exported func main() {\n}\n".to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compilation = setup_test(&interner, &keywords, &parser_arena, &scout_arena, &resolver);
    let _astrouts = compilation.expect_astrouts();
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
    let interner_arena = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = Bump::new();
    let interner = Interner::with_arena(&interner_arena);
    let keywords = Keywords::new(&interner);
    let resolver = code_hierarchy::test_from_vec(&interner, vec!["exported func moo<T>() where T Ref {\n}\n".to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compilation = setup_test(&interner, &keywords, &parser_arena, &scout_arena, &resolver);
    let _astrouts = compilation.expect_astrouts();
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
    let interner_arena = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = Bump::new();
    let interner = Interner::with_arena(&interner_arena);
    let keywords = Keywords::new(&interner);
    let resolver = code_hierarchy::test_from_vec(&interner, vec!["exported func moo<T>(x T) {\n}\n".to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compilation = setup_test(&interner, &keywords, &parser_arena, &scout_arena, &resolver);
    let astrouts = compilation.expect_astrouts();
    let test_tld = PackageCoordinate::test_tld(&interner, &keywords);
    let program = astrouts.get(&test_tld).unwrap();
    let main = program.lookup_function_by_str("moo");
    assert_eq!(
        *main.rune_to_type.get(&interner.intern_rune(
            IRuneValS::CodeRune(CodeRuneS { name: keywords.t })
        )).unwrap(),
        ITemplataType::CoordTemplataType(CoordTemplataType {})
    );
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
    let interner_arena = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = Bump::new();
    let interner = Interner::with_arena(&interner_arena);
    let keywords = Keywords::new(&interner);
    let resolver = code_hierarchy::test_from_vec(&interner, vec!["struct Moo {\n}\n".to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compilation = setup_test(&interner, &keywords, &parser_arena, &scout_arena, &resolver);
    let _astrouts = compilation.expect_astrouts();
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
    let interner_arena = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = Bump::new();
    let interner = Interner::with_arena(&interner_arena);
    let keywords = Keywords::new(&interner);
    let resolver = code_hierarchy::test_from_vec(&interner, vec!["struct Moo<T> {\n  bork T;\n}\n".to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compilation = setup_test(&interner, &keywords, &parser_arena, &scout_arena, &resolver);
    let _astrouts = compilation.expect_astrouts();
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
    let interner_arena = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = Bump::new();
    let interner = Interner::with_arena(&interner_arena);
    let keywords = Keywords::new(&interner);
    let resolver = code_hierarchy::test_from_vec(&interner, vec!["struct Moo<T> {\n  bork T;\n}\nstruct Bork<T> {\n  x Moo<T>;\n}\n".to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compilation = setup_test(&interner, &keywords, &parser_arena, &scout_arena, &resolver);
    let astrouts = compilation.expect_astrouts();
    let test_tld = PackageCoordinate::test_tld(&interner, &keywords);
    let program = astrouts.get(&test_tld).unwrap();
    let main = program.lookup_struct_by_str("Bork");
    assert_eq!(
        *main.header_rune_to_type.get(&interner.intern_rune(
            IRuneValS::CodeRune(CodeRuneS { name: keywords.t })
        )).unwrap(),
        ITemplataType::CoordTemplataType(CoordTemplataType {})
    );
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
    let interner_arena = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = Bump::new();
    let interner = Interner::with_arena(&interner_arena);
    let keywords = Keywords::new(&interner);
    let resolver = code_hierarchy::test_from_vec(&interner, vec!["interface Moo {\n}\n".to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compilation = setup_test(&interner, &keywords, &parser_arena, &scout_arena, &resolver);
    let _astrouts = compilation.expect_astrouts();
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
    let interner_arena = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = Bump::new();
    let interner = Interner::with_arena(&interner_arena);
    let keywords = Keywords::new(&interner);
    let resolver = code_hierarchy::test_from_vec(&interner, vec!["interface Moo<T> where T Ref {\n}\n".to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compilation = setup_test(&interner, &keywords, &parser_arena, &scout_arena, &resolver);
    let _astrouts = compilation.expect_astrouts();
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
    let interner_arena = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = Bump::new();
    let interner = Interner::with_arena(&interner_arena);
    let keywords = Keywords::new(&interner);
    let resolver = code_hierarchy::test_from_vec(&interner, vec!["interface Moo<T> where T Ref {\n  func bork(virtual self &Moo<T>) int;\n}\n".to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compilation = setup_test(&interner, &keywords, &parser_arena, &scout_arena, &resolver);
    let _astrouts = compilation.expect_astrouts();
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
    let interner_arena = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = Bump::new();
    let interner = Interner::with_arena(&interner_arena);
    let keywords = Keywords::new(&interner);
    let resolver = code_hierarchy::test_from_vec(&interner, vec!["struct List<T> {\n  moo T;\n}\nexported func moo<T>(x List<T>) {\n}\n".to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compilation = setup_test(&interner, &keywords, &parser_arena, &scout_arena, &resolver);
    let astrouts = compilation.expect_astrouts();
    let test_tld = PackageCoordinate::test_tld(&interner, &keywords);
    let program = astrouts.get(&test_tld).unwrap();
    let main = program.lookup_function_by_str("moo");
    assert_eq!(
        *main.rune_to_type.get(&interner.intern_rune(
            IRuneValS::CodeRune(CodeRuneS { name: keywords.t })
        )).unwrap(),
        ITemplataType::CoordTemplataType(CoordTemplataType {})
    );
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
    let interner_arena = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = Bump::new();
    let interner = Interner::with_arena(&interner_arena);
    let keywords = Keywords::new(&interner);
    let resolver = code_hierarchy::test_from_vec(&interner, vec!["func moo<T RefList>()\nwhere T = Refs(int, bool)\n{\n}\n".to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compilation = setup_test(&interner, &keywords, &parser_arena, &scout_arena, &resolver);
    let astrouts = compilation.expect_astrouts();
    let test_tld = PackageCoordinate::test_tld(&interner, &keywords);
    let program = astrouts.get(&test_tld).unwrap();
    let main = program.lookup_function_by_str("moo");
    assert_eq!(
        *main.rune_to_type.get(&interner.intern_rune(
            IRuneValS::CodeRune(CodeRuneS { name: keywords.t })
        )).unwrap(),
        ITemplataType::PackTemplataType(PackTemplataType {
            element_type: Box::new(ITemplataType::CoordTemplataType(CoordTemplataType {}))
        })
    );
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
    let interner_arena = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = Bump::new();
    let interner = Interner::with_arena(&interner_arena);
    let keywords = Keywords::new(&interner);
    let resolver = code_hierarchy::test_from_vec(&interner, vec!["func moo<T>()\nwhere func moo(T, bool)str\n{\n}\n".to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compilation = setup_test(&interner, &keywords, &parser_arena, &scout_arena, &resolver);
    let astrouts = compilation.expect_astrouts();
    let test_tld = PackageCoordinate::test_tld(&interner, &keywords);
    let program = astrouts.get(&test_tld).unwrap();
    let main = program.lookup_function_by_str("moo");
    assert_eq!(
        *main.rune_to_type.get(&interner.intern_rune(
            IRuneValS::CodeRune(CodeRuneS { name: keywords.t })
        )).unwrap(),
        ITemplataType::CoordTemplataType(CoordTemplataType {})
    );
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
    let interner_arena = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = Bump::new();
    let interner = Interner::with_arena(&interner_arena);
    let keywords = Keywords::new(&interner);
    let resolver = code_hierarchy::test_from_vec(&interner, vec!["func moo<P RefList>()\nwhere P = Refs(), Prot[P, str]\n{\n}\n".to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compilation = setup_test(&interner, &keywords, &parser_arena, &scout_arena, &resolver);
    let astrouts = compilation.expect_astrouts();
    let test_tld = PackageCoordinate::test_tld(&interner, &keywords);
    let program = astrouts.get(&test_tld).unwrap();
    let main = program.lookup_function_by_str("moo");
    assert_eq!(
        *main.rune_to_type.get(&interner.intern_rune(
            IRuneValS::CodeRune(CodeRuneS { name: interner.intern("P") })
        )).unwrap(),
        ITemplataType::PackTemplataType(PackTemplataType {
            element_type: Box::new(ITemplataType::CoordTemplataType(CoordTemplataType {}))
        })
    );
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
*/
// mig: fn type_simple_impl
// NOVEL CODE
#[test]
fn type_simple_impl() {
    let interner_arena = Bump::new();
    let parser_arena = Bump::new();
    let scout_arena = Bump::new();
    let interner = Interner::with_arena(&interner_arena);
    let keywords = Keywords::new(&interner);
    let resolver = code_hierarchy::test_from_vec(&interner, vec!["interface IMoo {\n}\nstruct Moo {\n}\nimpl IMoo for Moo;\n".to_string()])
        .or(|_: &PackageCoordinate<'_>| -> Option<HashMap<String, String>> { None });
    let mut compilation = setup_test(&interner, &keywords, &parser_arena, &scout_arena, &resolver);
    let _astrouts = compilation.expect_astrouts();
}
/*
// MIGALLOW: no corresponding scala test
*/
