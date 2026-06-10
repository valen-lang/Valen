use crate::higher_typing::astronomer_error_reporter::ICompileErrorA;
use crate::postparsing::itemplatatype::ITemplataType;
use crate::postparsing::post_parser_error_humanizer::humanize_rule;
use crate::postparsing::post_parser_error_humanizer::humanize_rune;
use crate::postparsing::post_parser_error_humanizer::humanize_rune_type_error;
use crate::postparsing::post_parser_error_humanizer::humanize_templata_type;
use crate::postparsing::rules::IRulexSR;
use crate::postparsing::rune_type_solver::IRuneTypeRuleError;
use crate::solver::solver_error_humanizer::humanize_failed_solve;
use crate::utils::range::CodeLocationS;
use crate::utils::range::RangeS;
/*
package dev.vale.highertyping

import dev.vale.postparsing.rules.IRulexSR
import dev.vale.postparsing._
import dev.vale.solver.SolverErrorHumanizer
import dev.vale.{CodeLocationS, FileCoordinateMap, RangeS}
import dev.vale.SourceCodeUtils.{humanizePos, lineContaining, nextThingAndRestOfLine}
import dev.vale.postparsing.PostParserErrorHumanizer
import dev.vale.solver.FailedSolve

object HigherTypingErrorHumanizer {
*/

// mig: fn assemble_error
pub fn assemble_error<'s>(
    filenames_and_sources: &dyn Fn(CodeLocationS<'s>) -> String,
    line_containing: &dyn Fn(CodeLocationS<'s>) -> String,
    range: RangeS<'s>,
    error_str_body: String,
) -> String {
    let pos_str = filenames_and_sources(range.begin);
    let next_stuff = line_containing(range.begin);
    let error_id = "A";
    format!("{} error {}: {}\n{}\n", pos_str, error_id, error_str_body, next_stuff)
}
/*
  def assembleError(
    filenamesAndSources: CodeLocationS => String,
    lineContaining: (CodeLocationS) => String,
    range: RangeS,
    errorStrBody: String) = {
    val posStr = filenamesAndSources(range.begin)
    val nextStuff = lineContaining(range.begin)
    val errorId = "A"
    f"${posStr} error ${errorId}: ${errorStrBody}\n${nextStuff}\n"
  }
*/

// mig: fn humanize_rune_type_solve_error
pub fn humanize_rune_type_solve_error() -> String { panic!("Unimplemented: humanize_rune_type_solve_error"); }
/*
  def humanizeRuneTypeSolveError(
    codeMap: CodeLocationS => String,
    linesBetween: (CodeLocationS, CodeLocationS) => Vector[RangeS],
    lineRangeContaining: (CodeLocationS) => RangeS,
    lineContaining: (CodeLocationS) => String,
    err: RuneTypeSolveError):
  String = {
    ": Couldn't solve generics types:\n" +
    SolverErrorHumanizer.humanizeFailedSolve[IRulexSR, IRuneS, ITemplataType, IRuneTypeRuleError](
      codeMap,
      linesBetween,
      lineRangeContaining,
      lineContaining,
      PostParserErrorHumanizer.humanizeRune,
      (tyype: ITemplataType) => tyype.toString,
      err => PostParserErrorHumanizer.humanizeRuneTypeError(codeMap, err),
      (rule: IRulexSR) => rule.range,
      (rule: IRulexSR) => rule.runeUsages.map(u => (u.rune, u.range)),
      (rule: IRulexSR) => rule.runeUsages.map(_.rune),
      PostParserErrorHumanizer.humanizeRule,
      err.failedSolve)._1
  }
*/

// mig: fn humanize
pub fn humanize<'s>(
    code_map: &dyn Fn(CodeLocationS<'s>) -> String,
    lines_between: &dyn Fn(CodeLocationS<'s>, CodeLocationS<'s>) -> Vec<RangeS<'s>>,
    line_range_containing: &dyn Fn(CodeLocationS<'s>) -> RangeS<'s>,
    line_containing: &dyn Fn(CodeLocationS<'s>) -> String,
    err: &ICompileErrorA<'s>,
) -> String {
    let error_str_body =
        match err {
            ICompileErrorA::RangedInternalError(_) => panic!("implement: humanize RangedInternalErrorA"),
            ICompileErrorA::CouldntFindType(_) => panic!("implement: humanize CouldntFindTypeA"),
            ICompileErrorA::CouldntSolveRules(e) => {
                let code_map_ref = |c: &CodeLocationS<'s>| code_map(*c);
                let lines_between_ref = |a: &CodeLocationS<'s>, b: &CodeLocationS<'s>| lines_between(*a, *b);
                let line_range_containing_ref = |c: &CodeLocationS<'s>| line_range_containing(*c);
                let line_containing_ref = |c: &CodeLocationS<'s>| line_containing(*c);
                let humanize_rule_error = |rt_err: &IRuneTypeRuleError<'s>|
                    humanize_rune_type_error(code_map, rt_err);
                format!(": Couldn't solve generics rules:\n{}",
                    humanize_failed_solve(
                        code_map_ref,
                        lines_between_ref,
                        line_range_containing_ref,
                        line_containing_ref,
                        humanize_rune,
                        |tyype: ITemplataType<'s>| humanize_templata_type(&tyype),
                        humanize_rule_error,
                        |rule: &IRulexSR<'s>| *rule.range(),
                        |rule: &IRulexSR<'s>| rule.rune_usages().iter().map(|u| (u.rune, u.range)).collect(),
                        |rule: &IRulexSR<'s>| rule.rune_usages().iter().map(|u| u.rune).collect(),
                        humanize_rule,
                        &e.error.failed_solve,
                    ).0)
            }
            ICompileErrorA::WrongNumArgsForTemplate(_) => panic!("implement: humanize WrongNumArgsForTemplateA"),
            _ => panic!("implement: humanize other arm"),
        };
    assemble_error(code_map, line_containing, err.range(), error_str_body)
}
/*
  def humanize(
    codeMap: CodeLocationS => String,
    linesBetween: (CodeLocationS, CodeLocationS) => Vector[RangeS],
    lineRangeContaining: (CodeLocationS) => RangeS,
    lineContaining: (CodeLocationS) => String,
      err: ICompileErrorA):
  String = {
    val errorStrBody =
      err match {
        case RangedInternalErrorA(range, message) => {
          ": internal error: " + message
        }
        case CouldntFindTypeA(range, name) => {
          ": Couldn't find type `" + PostParserErrorHumanizer.humanizeImpreciseName(name) + "`:\n"
        }
        case CouldntSolveRulesA(range, err) => {
          ": Couldn't solve generics rules:\n" +
          SolverErrorHumanizer.humanizeFailedSolve[IRulexSR, IRuneS, ITemplataType, IRuneTypeRuleError](
            codeMap,
            linesBetween,
            lineRangeContaining,
            lineContaining,
            PostParserErrorHumanizer.humanizeRune,
            (tyype: ITemplataType) => PostParserErrorHumanizer.humanizeTemplataType(tyype),
            err => PostParserErrorHumanizer.humanizeRuneTypeError(codeMap, err),
            (rule: IRulexSR) => rule.range,
            (rule: IRulexSR) => rule.runeUsages.map(u => (u.rune, u.range)),
            (rule: IRulexSR) => rule.runeUsages.map(_.rune),
            PostParserErrorHumanizer.humanizeRule,
            err.failedSolve)._1
        }
        case WrongNumArgsForTemplateA(range, expectedNumArgs, actualNumArgs) => {
          ": Expected " + expectedNumArgs + " template args but received " + actualNumArgs + "\n"
        }
      }
    assembleError(codeMap, lineContaining, err.range, errorStrBody)
  }
}
*/
