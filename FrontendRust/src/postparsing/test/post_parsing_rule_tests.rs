use bumpalo::Bump;
use crate::compile_options::GlobalOptions;
use crate::parsing::tests::utils::compile_file;
use crate::postparsing::ast::ProgramS;
use crate::postparsing::itemplatatype::{
  CoordTemplataType, IntegerTemplataType, ITemplataType, KindTemplataType, MutabilityTemplataType,
  OwnershipTemplataType, VariabilityTemplataType,
};
use crate::postparsing::names::{CodeRuneS, IRuneValS};
use crate::postparsing::post_parser::PostParser;
use crate::{Interner, Keywords};
/*
package dev.vale.postparsing

import dev.vale._
import dev.vale.options.GlobalOptions
import dev.vale.parsing._
import dev.vale.postparsing._
import org.scalatest._

import scala.collection.immutable.List

class PostParsingRuleTests extends FunSuite with Matchers {
*/

fn compile<'a, 'ctx, 'p>(
  interner: &'ctx Interner<'a>,
  keywords: &'ctx Keywords<'a>,
  arena: &'p Bump,
  code: &str,
) -> ProgramS<'a, 'p>
where
  'a: 'ctx,
  'a: 'p,
{
  let options = GlobalOptions {
    sanity_check: true,
    use_overload_index: true,
    use_optimized_solver: true,
    verbose_errors: false,
    debug_output: false,
  };

  let only_file = compile_file(interner, keywords, arena, code).unwrap();
  let post_parser = PostParser::new(options, interner, keywords, arena);
  post_parser
    .scout_program(only_file.file_coord, &only_file)
    .unwrap()
}
/*
  private def compile(code: String, interner: Interner = new Interner()): ProgramS = {
    val compile = PostParserTestCompilation.test(code, interner)
    compile.getScoutput() match {
      case Err(e) => {
        val codeMap = compile.getCodeMap().getOrDie()
        vfail(PostParserErrorHumanizer.humanize(
          SourceCodeUtils.humanizePos(codeMap, _),
          SourceCodeUtils.linesBetween(codeMap, _, _),
          SourceCodeUtils.lineRangeContaining(codeMap, _),
          SourceCodeUtils.lineContaining(codeMap, _),
          e))
      }
      case Ok(t) => t.expectOne()
    }
  }
*/
fn compile_for_error<'a, 'ctx, 'p, 's>(
  _interner: &'ctx crate::Interner<'a>,
  _keywords: &'ctx crate::Keywords<'a>,
  _parse_arena: &'p bumpalo::Bump,
  _scout_arena: &'s bumpalo::Bump,
  _code: &str,
) -> crate::postparsing::post_parser::ICompileErrorS<'a, 's>
where
  'a: 'ctx,
  'a: 'p,
  'a: 's,
{
  panic!("Unimplemented: compile_for_error");
}
/*
  private def compileForError(code: String): ICompileErrorS = {
    PostParserTestCompilation.test(code).getScoutput() match {
      case Err(e) => e
      case Ok(t) => vfail("Successfully compiled!\n" + t.toString)
    }
  }
*/
#[test]
fn predict_simple_templex() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let program = compile(
    &interner,
    &keywords,
    &parse_arena,
    "func main(a int) {}",
  );
  let main = program.lookup_function("main");
  let rune = &main.params.first().unwrap().pattern.coord_rune.as_ref().unwrap().rune;
  let predicted = main
    .rune_to_predicted_type
    .get(&rune)
    .expect("expected rune in rune_to_predicted_type");
  assert_eq!(predicted, &ITemplataType::CoordTemplataType(CoordTemplataType {}));
}
/*
  test("Predict simple templex") {
    val program =
      compile(
        """
          |func main(a int) {}
          |""".stripMargin)
    val main = program.lookupFunction("main")

    vassertSome(main.runeToPredictedType.get(main.params.head.pattern.coordRune.get.rune)) shouldEqual
      CoordTemplataType()
  }
*/
#[test]
fn can_know_rune_type_from_simple_equals() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let program = compile(
    &interner,
    &keywords,
    &parse_arena,
    "func main<T, Y>(a T) where Y = T {}",
  );
  let main = program.lookup_function("main");

  let t_rune = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
    name: interner.intern("T"),
  }));
  let y_rune = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
    name: interner.intern("Y"),
  }));

  let t_predicted = main.rune_to_predicted_type.get(&t_rune).unwrap();
  let y_predicted = main.rune_to_predicted_type.get(&y_rune).unwrap();

  assert_eq!(
    t_predicted,
    &ITemplataType::CoordTemplataType(CoordTemplataType {})
  );
  assert_eq!(
    y_predicted,
    &ITemplataType::CoordTemplataType(CoordTemplataType {})
  );
}
/*
  test("Can know rune type from simple equals") {
    val interner = new Interner()
    val program =
      compile(
        """
          |func main<T, Y>(a T)
          |where Y = T {}
          |""".stripMargin, interner)
    val main = program.lookupFunction("main")

    vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("T"))))) shouldEqual
      CoordTemplataType()
    vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("Y"))))) shouldEqual
      CoordTemplataType()
  }
*/
#[test]
fn predict_knows_type_from_or_rule() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let program = compile(
    &interner,
    &keywords,
    &parse_arena,
    "func main<M Ownership>(a int) where M = any(own, borrow) {}",
  );
  let main = program.lookup_function("main");

  let m_rune = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
    name: interner.intern("M"),
  }));
  let m_predicted = main.rune_to_predicted_type.get(&m_rune).unwrap();
  assert_eq!(
    m_predicted,
    &ITemplataType::OwnershipTemplataType(OwnershipTemplataType {})
  );
}
/*
  test("Predict knows type from Or rule") {
    val interner = new Interner()
    val program =
      compile(
        """
          |func main<M Ownership>(a int)
          |where M = any(own, borrow) {}
          |""".stripMargin, interner)
    val main = program.lookupFunction("main")

    vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("M"))))) shouldEqual
      OwnershipTemplataType()
  }
*/
#[test]
fn predict_coord_component_types() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  // vregionmut() // Put back in with regions
  // val program =
  //   compile(
  //     """
  //       |func main<T>(a T)
  //       |where T = Ref[O, R, K], O Ownership, R Region, K Kind {}
  //       |""".stripMargin, interner)
  // Take out with regions
  let program = compile(
    &interner,
    &keywords,
    &parse_arena,
    "func main<T>(a T) where T = Ref[O, K], O Ownership, K Kind {}",
  );
  let main = program.lookup_function("main");

  let t_rune = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
    name: interner.intern("T"),
  }));
  let o_rune = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
    name: interner.intern("O"),
  }));
  let k_rune = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
    name: interner.intern("K"),
  }));
  assert_eq!(
    main.rune_to_predicted_type.get(&t_rune),
    Some(&ITemplataType::CoordTemplataType(CoordTemplataType {}))
  );
  assert_eq!(
    main.rune_to_predicted_type.get(&o_rune),
    Some(&ITemplataType::OwnershipTemplataType(OwnershipTemplataType {}))
  );
  // vregionmut() // Put back in with regions
  // vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("R"))))) shouldEqual RegionTemplataType()
  assert_eq!(
    main.rune_to_predicted_type.get(&k_rune),
    Some(&ITemplataType::KindTemplataType(KindTemplataType {}))
  );
}
/*
  test("Predict CoordComponent types") {
    val interner = new Interner()
    vregionmut() // Put back in with regions
    // val program =
    //   compile(
    //     """
    //       |func main<T>(a T)
    //       |where T = Ref[O, R, K], O Ownership, R Region, K Kind {}
    //       |""".stripMargin, interner)
    // Take out with regions
    val program =
      compile(
        """
          |func main<T>(a T)
          |where T = Ref[O, K], O Ownership, K Kind {}
          |""".stripMargin, interner)
    val main = program.lookupFunction("main")

    vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("T"))))) shouldEqual CoordTemplataType()
    vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("O"))))) shouldEqual OwnershipTemplataType()
    vregionmut() // Put back in with regions
    // vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("R"))))) shouldEqual RegionTemplataType()
    vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("K"))))) shouldEqual KindTemplataType()
  }
*/
#[test]
fn predict_call_types() {
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let program = compile(
    &interner,
    &keywords,
    &parse_arena,
    "func main<A, B>(p1 A, p2 B) where A = T<B>, T = Option, A = int {}",
  );
  let main = program.lookup_function("main");

  let a_rune = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
    name: interner.intern("A"),
  }));
  let b_rune = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
    name: interner.intern("B"),
  }));
  let t_rune = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
    name: interner.intern("T"),
  }));
  assert_eq!(
    main.rune_to_predicted_type.get(&a_rune),
    Some(&ITemplataType::CoordTemplataType(CoordTemplataType {}))
  );
  assert_eq!(
    main.rune_to_predicted_type.get(&b_rune),
    Some(&ITemplataType::CoordTemplataType(CoordTemplataType {}))
  );
  // We can't know if T it's a Coord->Coord or a Coord->Kind type.
  assert_eq!(main.rune_to_predicted_type.get(&t_rune), None);
}
/*
  test("Predict Call types") {
    val interner = new Interner()
    val program =
      compile(
        """
          |func main<A, B>(p1 A, p2 B)
          |where A = T<B>, T = Option, A = int {}
          |""".stripMargin, interner)
    val main = program.lookupFunction("main")

    vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("A"))))) shouldEqual CoordTemplataType()
    vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("B"))))) shouldEqual CoordTemplataType()
    // We can't know if T it's a Coord->Coord or a Coord->Kind type.
    main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("T")))) shouldEqual None
  }
*/
#[test]
fn predict_array_sequence_types() {
  // Not sure if this test is useful anymore, since we say M, V, N's types up-front now
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let program = compile(
    &interner,
    &keywords,
    &parse_arena,
    "func main<M Mutability, V Variability, N Int, E>(t T) where T Ref = [#N]<M, V>E {}",
  );
  let main = program.lookup_function("main");

  let m_rune = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
    name: interner.intern("M"),
  }));
  let v_rune = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
    name: interner.intern("V"),
  }));
  let n_rune = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
    name: interner.intern("N"),
  }));
  let e_rune = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
    name: interner.intern("E"),
  }));
  let t_rune = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
    name: interner.intern("T"),
  }));
  assert_eq!(
    main.rune_to_predicted_type.get(&m_rune),
    Some(&ITemplataType::MutabilityTemplataType(MutabilityTemplataType {}))
  );
  assert_eq!(
    main.rune_to_predicted_type.get(&v_rune),
    Some(&ITemplataType::VariabilityTemplataType(VariabilityTemplataType {}))
  );
  assert_eq!(
    main.rune_to_predicted_type.get(&n_rune),
    Some(&ITemplataType::IntegerTemplataType(IntegerTemplataType {}))
  );
  assert_eq!(
    main.rune_to_predicted_type.get(&e_rune),
    Some(&ITemplataType::CoordTemplataType(CoordTemplataType {}))
  );
  assert_eq!(
    main.rune_to_predicted_type.get(&t_rune),
    Some(&ITemplataType::CoordTemplataType(CoordTemplataType {}))
  );
}
/*
  // Not sure if this test is useful anymore, since we say M, V, N's types up-front now
  test("Predict array sequence types") {
    val interner = new Interner()
    val program =
      compile(
        """
          |func main<M Mutability, V Variability, N Int, E>(t T)
          |where T Ref = [#N]<M, V>E {}
          |""".stripMargin, interner)
    val main = program.lookupFunction("main")

    vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("M"))))) shouldEqual MutabilityTemplataType()
    vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("V"))))) shouldEqual VariabilityTemplataType()
    vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("N"))))) shouldEqual IntegerTemplataType()
    vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("E"))))) shouldEqual CoordTemplataType()
    vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("T"))))) shouldEqual CoordTemplataType()
  }
*/
#[test]
fn predict_for_is_interface() {
  // Not sure if this test is useful anymore, since we say Kind up-front now
  let arena = Bump::new();
  let parse_arena = Bump::new();
  let interner = Interner::with_arena(&arena);
  let keywords = Keywords::new(&interner);
  let program = compile(
    &interner,
    &keywords,
    &parse_arena,
    "func main<A Kind, B Kind>() where A = isInterface(B) {}",
  );
  let main = program.lookup_function("main");

  let a_rune = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
    name: interner.intern("A"),
  }));
  let b_rune = interner.intern_rune(IRuneValS::CodeRune(CodeRuneS {
    name: interner.intern("B"),
  }));
  assert_eq!(
    main.rune_to_predicted_type.get(&a_rune),
    Some(&ITemplataType::KindTemplataType(KindTemplataType {}))
  );
  assert_eq!(
    main.rune_to_predicted_type.get(&b_rune),
    Some(&ITemplataType::KindTemplataType(KindTemplataType {}))
  );
}
/*
  // Not sure if this test is useful anymore, since we say Kind up-front now
  test("Predict for isInterface") {
    val interner = new Interner()
    val program =
      compile(
        """
          |func main<A Kind, B Kind>()
          |where A = isInterface(B) {}
          |""".stripMargin, interner)
    val main = program.lookupFunction("main")

    vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("A"))))) shouldEqual KindTemplataType()
    vassertSome(main.runeToPredictedType.get(CodeRuneS(interner.intern(StrI("B"))))) shouldEqual KindTemplataType()
  }
*/
/*
}
*/