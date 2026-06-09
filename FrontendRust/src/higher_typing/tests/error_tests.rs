/*
package dev.vale.highertyping

import dev.vale.postparsing._
import dev.vale.{Err, Ok, SourceCodeUtils, StrI, vassert, vfail}
import org.scalatest._
import dev.vale.solver._

class ErrorTests extends FunSuite with Matchers  {
*/

// mig: fn compile_program_for_error
fn compile_program_for_error<'s, 'ctx, 'p>(
    compilation: &mut crate::higher_typing::higher_typing_pass::HigherTypingCompilation<'s, 'ctx, 'p>,
) -> crate::higher_typing::astronomer_error_reporter::ICompileErrorA<'s> {
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

// mig: fn report_type_not_found
#[test]
fn report_type_not_found() {
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let compilation_bump = bumpalo::Bump::new();
    report_type_not_found_inner(&parse_bump, &scout_bump, &compilation_bump);
}

fn report_type_not_found_inner<'s>(
    parse_bump: &'s bumpalo::Bump,
    scout_bump: &'s bumpalo::Bump,
    compilation_bump: &'s bumpalo::Bump,
) {
    use crate::interner::StrI;
    use crate::keywords::Keywords;
    use crate::parse_arena::ParseArena;
    use crate::scout_arena::ScoutArena;
    use crate::utils::range::CodeLocationS;

    let parse_arena = ParseArena::new(parse_bump);
    let scout_arena = ScoutArena::new(scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let code = "exported func main(a Bork) {\n}\n";
    let mut compilation = crate::higher_typing::tests::test_compilation::test(
        compilation_bump, &scout_arena, &keywords, &parser_keywords, &parse_arena, code);
    let err = compile_program_for_error(&mut compilation);
    match &err {
        crate::higher_typing::astronomer_error_reporter::ICompileErrorA::CouldntSolveRules(
            crate::higher_typing::astronomer_error_reporter::CouldntSolveRulesA {
                error: crate::postparsing::rune_type_solver::RuneTypeSolveError {
                    failed_solve: crate::solver::solver::FailedSolve {
                        error: crate::solver::solver::ISolverError::RuleError(crate::solver::solver::RuleError {
                            err: crate::postparsing::rune_type_solver::IRuneTypeRuleError::CouldntFindType(
                                crate::postparsing::rune_type_solver::RuneTypingCouldntFindType {
                                    name: crate::postparsing::names::IImpreciseNameS::CodeName(crate::postparsing::names::CodeNameS { name: StrI("Bork") }),
                                    ..
                                }),
                            ..
                        }),
                        ..
                    },
                    ..
                },
                ..
            }
        ) => {
            let code_map = compilation.get_code_map().unwrap();
            let humanize_pos_fn = |x: CodeLocationS<'s>| crate::utils::source_code_utils::humanize_pos_code_map(&code_map, &x);
            let lines_between_fn = |x: CodeLocationS<'s>, y: CodeLocationS<'s>| crate::utils::source_code_utils::lines_between(&code_map, &x, &y);
            let line_range_containing_fn = |x: CodeLocationS<'s>| crate::utils::source_code_utils::line_range_containing(&code_map, &x);
            let line_containing_fn = |x: CodeLocationS<'s>| crate::utils::source_code_utils::line_containing(&code_map, &x);
            let error_text =
                crate::higher_typing::higher_typing_error_humanizer::humanize(
                    &humanize_pos_fn, &lines_between_fn, &line_range_containing_fn, &line_containing_fn, &err);
            assert!(error_text.contains("Couldn't find anything with the name 'Bork'"));
        }
        _ => panic!("expected CouldntSolveRules(...RuneTypingCouldntFindType(CodeNameS(\"Bork\")))"),
    }
}
/* Guardian: disable-all */
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
