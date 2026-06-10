use bumpalo::Bump;
use crate::cast;
use crate::compile_options::GlobalOptions;
use crate::interner::StrI;
use crate::parsing::tests::utils::compile_file;
use crate::parsing::ast::OwnershipP;
use crate::postparsing::ast::{GenericParameterS, ParameterS, ProgramS};
use crate::postparsing::ast::IBodyS::CodeBody;
use crate::postparsing::ast::IGenericParameterTypeS::CoordGenericParameterType;
use crate::postparsing::names::{CodeNameS, CodeRuneS, IRuneValS, IVarNameS};
use crate::postparsing::names::IRuneS::{CodeRune, ImplicitRune};
use crate::postparsing::patterns::{AtomSP, CaptureS};
use crate::postparsing::names::IImpreciseNameS;
use crate::postparsing::rules::rules::{AugmentSR, MaybeCoercingLookupSR};
use crate::postparsing::rules::RuneUsage;
use crate::postparsing::test::traverse::NodeRefS;
use crate::postparsing::post_parser::{CouldntFindRuneS, ICompileErrorS, PostParser};
use crate::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::collect_only_snode;

/*
package dev.vale.postparsing

import dev.vale._
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.BorrowP
import dev.vale.postparsing.patterns.{AtomSP, CaptureS}
import dev.vale.postparsing.rules.{AugmentSR, MaybeCoercingLookupSR, RuneUsage}
import dev.vale.parsing._
import dev.vale.parsing.ast._
import dev.vale.postparsing.patterns.AtomSP
import dev.vale.postparsing.rules._
import org.scalatest._

class PostParsingParametersTests extends FunSuite with Matchers with Collector {
*/
fn compile<'s, 'ctx, 'p>(
  scout_arena: &'ctx ScoutArena<'s>,
  keywords: &'ctx Keywords<'s>,
  parse_arena: &'ctx ParseArena<'p>,
  code: &str,
) -> ProgramS<'s>
where 'p: 's,
{
  let options = GlobalOptions {
    sanity_check: true,
    use_overload_index: true,
    use_optimized_solver: true,
    verbose_errors: false,
    debug_output: false,
  };

  let keywords_p = Keywords::new_for_parse(parse_arena);
  let only_file = compile_file(parse_arena, &keywords_p, code).unwrap();
  // Re-intern FileCoordinate from 'p into 's
  let file_coord_s = scout_arena.intern_file_coordinate(
    scout_arena.intern_package_coordinate(
      scout_arena.intern_str(only_file.file_coord.package_coord.module.as_str()),
      &only_file.file_coord.package_coord.packages.iter().map(|s| scout_arena.intern_str(s.as_str())).collect::<Vec<_>>(),
    ),
    only_file.file_coord.filepath.as_str(),
  );
  let post_parser = PostParser::new(options, scout_arena, keywords, &keywords_p, parse_arena);
  post_parser
    .scout_program(file_coord_s, &only_file)
    .unwrap()
}

/*
  private def compile(code: String, interner: Interner = new Interner()): ProgramS = {
    val compilation = PostParserTestCompilation.test(code, interner)
    compilation.getScoutput() match {
      case Err(e) => {
        val codeMap = compilation.getCodeMap().getOrDie()
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
fn compile_for_error<'s, 'ctx, 'p>(
  scout_arena: &'ctx ScoutArena<'s>,
  keywords: &'ctx Keywords<'s>,
  parse_arena: &'ctx ParseArena<'p>,
  code: &str,
) -> ICompileErrorS<'s>
where 'p: 's,
{
  let options = GlobalOptions {
    sanity_check: true,
    use_overload_index: true,
    use_optimized_solver: true,
    verbose_errors: false,
    debug_output: false,
  };

  let keywords_p = Keywords::new_for_parse(parse_arena);
  let only_file = compile_file(parse_arena, &keywords_p, code).unwrap();
  // Re-intern FileCoordinate from 'p into 's
  let file_coord_s = scout_arena.intern_file_coordinate(
    scout_arena.intern_package_coordinate(
      scout_arena.intern_str(only_file.file_coord.package_coord.module.as_str()),
      &only_file.file_coord.package_coord.packages.iter().map(|s| scout_arena.intern_str(s.as_str())).collect::<Vec<_>>(),
    ),
    only_file.file_coord.filepath.as_str(),
  );
  let post_parser = PostParser::new(options, scout_arena, keywords, &keywords_p, parse_arena);
  match post_parser.scout_program(file_coord_s, &only_file) {
    Ok(_) => panic!("Accidentally compiled!"),
    Err(e) => e,
  }
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
fn coord_rune_rule() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(&scout_arena, &keywords, &parse_arena, "func main<T>(moo T) { }");
  let main = program1.lookup_function("main");

  // vregionmut() // Take out with regions
  // Should have T, the default region, and the return rune
  assert_eq!(main.rune_to_predicted_type.len(), 2);
  // // Should have T, the default region, and the return rune
  // assert_eq!(main.rune_to_predicted_type.len(), 3);

  // vregionmut() // see below
  match main.generic_params {
    [GenericParameterS {
      rune: RuneUsage {
        rune: CodeRune(CodeRuneS { name: StrI("T") }),
        ..
      },
      tyype: CoordGenericParameterType(_),
      default: None,
      ..
    }] => {
      // Put this back in when we have regions
      // , _ // implicit default region
    }
    _ => panic!("expected GenericParameterS(_, RuneUsage(_, CodeRuneS(StrI(\"T\"))), CoordGenericParameterTypeS, None)"),
  }
}
/*
  test("Coord rune rule") {
    val program1 = compile("""func main<T>(moo T) { }""")
    val main = program1.lookupFunction("main")

    vregionmut() // Take out with regions
    // Should have T, the default region, and the return rune
    vassert(main.runeToPredictedType.size == 2)
    // // Should have T, the default region, and the return rune
    // vassert(main.runeToPredictedType.size == 3)

    vregionmut() // see below
    main.genericParams match {
      case Vector(
        GenericParameterS(_, RuneUsage(_, CodeRuneS(StrI("T"))), CoordGenericParameterTypeS(_, _, _), None)
        // Put this back in when we have regions
        // , _ // implicit default region
        ) =>
//      case Vector(
//        GenericParameterS(
//          RangeS(_:10, _:11),RuneUsage(RangeS(_:0, _:23),CodeRuneS(StrI(T))),CoordTemplataType(),None,Vector(),None),
//        GenericParameterS(_,RuneUsage(_,DefaultRegionRuneS()),RegionTemplataType(),None,Vector(ReadWriteRuneAttributeS(_)),None))

      //        // T's implicit region rune, see MNRFGC and IRRAE.
//        GenericParameterS(_, RuneUsage(_, ImplicitRegionRuneS(CodeRuneS(StrI("T")))), RegionTemplataType(), _, _, None)) =>
    }
  }
*/
#[test]
fn returned_rune() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(&scout_arena, &keywords, &parse_arena, "func main<T>(moo T) T { moo }");
  let main = program1.lookup_function("main");

  let t_name = scout_arena.intern_str("T");
  let t_rune = scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: t_name }));
  assert!(
    main.generic_params.iter().any(|p| p.rune.rune == t_rune),
    "genericParams should contain rune for T"
  );
  match &main.maybe_ret_coord_rune {
    Some(RuneUsage {
      rune: CodeRune(CodeRuneS { name: StrI("T") }),
      ..
    }) => {}
    _ => panic!("expected Some(RuneUsage(_, CodeRuneS(\"T\")))"),
  }
}
/*
  test("Returned rune") {
    val interner = new Interner()
    val program1 = compile("""func main<T>(moo T) T { moo }""", interner)
    val main = program1.lookupFunction("main")

    vassert(main.genericParams.map(_.rune.rune).contains(CodeRuneS(interner.intern(StrI("T")))))
    main.maybeRetCoordRune match { case Some(RuneUsage(_, CodeRuneS(StrI("T")))) => }
  }
*/
#[test]
fn borrowed_rune() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(&scout_arena, &keywords, &parse_arena, "func main<T>(moo &T) { }");
  let main = program1.lookup_function("main");

  let t_coord_rune_from_params = match main.params {
    [ParameterS {
      pre_checked: false,
      pattern:
        AtomSP {
          name: Some(CaptureS {
            name: IVarNameS::CodeVarName(StrI("moo")),
            mutate: false,
          }),
          coord_rune: Some(RuneUsage {
            rune: tcr @ ImplicitRune(_),
            ..
          }),
          destructure: None,
          ..
        },
      ..
    }] => tcr,
    _ => panic!("param structure did not match"),
  };

  let t_coord_rune_from_rules: &RuneUsage<'_> = collect_only_snode!(
    NodeRefS::Function(main),
    NodeRefS::AugmentRule(AugmentSR {
      inner_rune: RuneUsage {
        rune: CodeRune(CodeRuneS { name: StrI("T") }),
        ..
      },
      ownership: Some(OwnershipP::Borrow),
      result_rune,
      ..
    }) => Some(result_rune)
  );

  assert_eq!(t_coord_rune_from_params, &t_coord_rune_from_rules.rune);
}
/*
  test("Borrowed rune") {
    val program1 = compile("""func main<T>(moo &T) { }""")
    val main = program1.lookupFunction("main")
    val Vector(param) = main.params

    val tCoordRuneFromParams =
      param match {
        case ParameterS(_,
          _,
          false,
          AtomSP(_,
            Some(CaptureS(CodeVarNameS(StrI("moo")), false)),
            Some(RuneUsage(_, tcr @ ImplicitRuneS(_))),
            None)) => tcr
      }

    val tCoordRuneFromRules =
      main.rules shouldHave {
        case AugmentSR(_, tcr, Some(BorrowP), RuneUsage(_, CodeRuneS(StrI("T")))) => tcr
      }

    tCoordRuneFromParams shouldEqual tCoordRuneFromRules.rune
  }
*/
#[test]
fn anonymous_typed_param() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(&scout_arena, &keywords, &parse_arena, "func main(_ int) { }");
  let main = program1.lookup_function("main");

  let param_rune = match main.params {
    [ParameterS {
      pre_checked: false,
      pattern:
        AtomSP {
          name: None,
          coord_rune: Some(RuneUsage {
            rune: pr @ ImplicitRune(_),
            ..
          }),
          destructure: None,
          ..
        },
      ..
    }] => pr,
    _ => panic!("param structure did not match (expected anonymous typed param)"),
  };

  let rule_rune: RuneUsage = collect_only_snode!(
    NodeRefS::Function(main),
    NodeRefS::MaybeCoercingLookupRule(MaybeCoercingLookupSR {
      name: IImpreciseNameS::CodeName(CodeNameS { name: StrI("int") }),
      rune: pr,
      ..
    }) => Some(pr)
  );
  assert_eq!(&rule_rune.rune, param_rune);
}
/*
  test("Anonymous, typed param") {
    val program1 = compile("""func main(_ int) { }""")
    val main = program1.lookupFunction("main")
    val Vector(param) = main.params
    val paramRune =
      param match {
        case ParameterS(_,
          None,false,
          AtomSP(_,
            Some(CaptureS(CodeVarNameS(StrI(_)),false)),Some(RuneUsage(_,ImplicitRuneS(LocationInDenizen(Vector(2, 1, 1, 1, 1))))),None)) =>
        case ParameterS(_,
          _,
          false,
          AtomSP(_,
            None,
            Some(RuneUsage(_, pr @ ImplicitRuneS(_))),
            None)) => pr
      }

    main.rules shouldHave {
      case MaybeCoercingLookupSR(_, pr, CodeNameS(StrI("int"))) => vassert(pr.rune == paramRune)
    }
  }
*/
// #[test]
// fn regioned_pure_function() {
//   panic!("Unmigrated test: regioned_pure_function");
// }
/*
  vregionmut() // Put back in with regions
  // test("Regioned pure function") {
  //   val bork = compile("pure func main<r', t'>(ship &r'Spaceship) t'{ }")
  //
  //   val main = bork.lookupFunction("main")
  //   main.genericParams.size shouldEqual 2
  // }
*/
// #[test]
// fn regioned_additive_function() {
//   panic!("Unmigrated test: regioned_additive_function");
// }
/*
  vregionmut() // Put back in with regions
  // test("Regioned additive function") {
  //   val bork = compile("additive func main<r', t'>(ship &r'Spaceship) t'{ }")
  //
  //   val main = bork.lookupFunction("main")
  //   main.genericParams.size shouldEqual 2
  //   main.genericParams(0) match {
  //     case GenericParameterS(_,RuneUsage(_,CodeRuneS(StrI("r"))),RegionGenericParameterTypeS(ReadOnlyRegionS),None) =>
  //   }
  // }
*/
#[test]
fn test_param_less_lambda_identifying_runes() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {do({ return 3; })}",
  );
  let main = program1.lookup_function("main");

  // vregionmut() // Put this back in when we have regions
  // main.genericParams.size shouldEqual 1 // only the default region
  // Take this out when we have regions
  assert_eq!(main.generic_params.len(), 0);

  let code_body = cast!(&main.body, CodeBody);
  let lambda = collect_only_snode!(
    NodeRefS::Expression(code_body.body.block.expr),
    NodeRefS::Function(lambda_func) => Some(lambda_func)
  );
  // vregionmut() // Put this back in when we have regions
  // lambda.function.genericParams.size shouldEqual 1 // only the default region
  // Take this out when we have regions
  assert_eq!(lambda.generic_params.len(), 0);
}
/*
  test("Test param-less lambda identifying runes") {
    val bork = compile(
      """
        |exported func main() int {do({ return 3; })}
        |""".stripMargin)

    val main = bork.lookupFunction("main")
    vregionmut() // Put this back in when we have regions
    // main.genericParams.size shouldEqual 1 // only the default region
    // Take this out when we have regions
    main.genericParams.size shouldEqual 0
    val lambda = Collector.onlyOf(main.body, classOf[FunctionSE])
    vregionmut() // Put this back in when we have regions
    // lambda.function.genericParams.size shouldEqual 1 // only the default region
    // Take this out when we have regions
    lambda.function.genericParams.size shouldEqual 0
  }
*/
#[test]
fn test_one_param_lambda_identifying_runes() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {do({ _ })}",
  );
  let main = program1.lookup_function("main");

  // vregionmut() // Put this back in when we have regions
  // main.genericParams.size shouldEqual 1 // Only the default region
  // Take this out when we have regions
  assert_eq!(main.generic_params.len(), 0);

  let code_body = cast!(&main.body, CodeBody);
  let lambda = collect_only_snode!(
    NodeRefS::Expression(code_body.body.block.expr),
    NodeRefS::Function(lambda_func) => Some(lambda_func)
  );
  // vregionmut() // Put this back in when we have regions
  // // magic param + default region
  // lambda.function.genericParams.size shouldEqual 2
  // Take this out when we have regions
  assert_eq!(lambda.generic_params.len(), 1);
}
/*
  test("Test one-param lambda identifying runes") {
    val bork = compile(
      """
        |exported func main() int {do({ _ })}
        |""".stripMargin)

    val main = bork.lookupFunction("main")
    vregionmut() // Put this back in when we have regions
    // main.genericParams.size shouldEqual 1 // Only the default region
    // Take this out when we have regions
    main.genericParams.size shouldEqual 0
    val lambda = Collector.onlyOf(main.body, classOf[FunctionSE])
    vregionmut() // Put this back in when we have regions
    // // magic param + default region
    // lambda.function.genericParams.size shouldEqual 2
    // Take this out when we have regions
    lambda.function.genericParams.size shouldEqual 1
  }
*/
#[test]
fn report_that_default_region_must_be_mentioned_in_generic_params() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let err = compile_for_error(
    &scout_arena,
    &keywords,
    &parse_arena,
    "pure func main<r'>(ship &r'Spaceship) t'{ }",
  );
  match &err {
    ICompileErrorS::CouldntFindRuneS(CouldntFindRuneS { ref name, .. }) => {
      match name.as_str() {
        "t" => {}
        _ => panic!("expected CouldntFindRuneS with name \"t\", got {:?}", err),
      }
    }
    _ => panic!("expected CouldntFindRuneS with name \"t\", got {:?}", err),
  }
}
/*
  test("Report that default region must be mentioned in generic params") {
    compileForError("pure func main<r'>(ship &r'Spaceship) t'{ }") match {
      case CouldntFindRuneS(range, "t") =>
    }
  }
*/
/*
}
*/