/*
package dev.vale.solver

import dev.vale.{CodeLocationS, FileCoordinateMap, RangeS, repeatStr}
import dev.vale.SourceCodeUtils.{lineContaining, lineRangeContaining, linesBetween}
import dev.vale.RangeS

object SolverErrorHumanizer {
*/
use crate::utils::range::{CodeLocationS, RangeS};

// mig: fn humanize_failed_solve
pub fn humanize_failed_solve<'a, Rule, RuneId, Conclusion, ErrType, SolveResult>(
  _code_map: impl Fn(&CodeLocationS<'a>) -> String,
  _lines_between: impl Fn(&CodeLocationS<'a>, &CodeLocationS<'a>) -> Vec<RangeS<'a>>,
  _line_range_containing: impl Fn(&CodeLocationS<'a>) -> RangeS<'a>,
  _line_containing: impl Fn(&CodeLocationS<'a>) -> String,
  _humanize_rune: impl Fn(&RuneId) -> String,
  _humanize_conclusion: impl Fn(&Conclusion) -> String,
  _humanize_rule_error: impl Fn(&ErrType) -> String,
  _get_rule_range: impl Fn(&Rule) -> RangeS<'a>,
  _get_rune_usages: impl Fn(&Rule) -> Vec<(RuneId, RangeS<'a>)>,
  _rule_to_runes: impl Fn(&Rule) -> Vec<RuneId>,
  _rule_to_string: impl Fn(&Rule) -> String,
  _result: &SolveResult,
) -> (String, Vec<CodeLocationS<'a>>)
{
  panic!("Unimplemented: humanize_failed_solve");
}
/*
  def humanizeFailedSolve[Rule, RuneID, Conclusion, ErrType](
    codeMap: CodeLocationS => String,
    linesBetween: (CodeLocationS, CodeLocationS) => Vector[RangeS],
    lineRangeContaining: (CodeLocationS) => RangeS,
    lineContaining: (CodeLocationS) => String,
    humanizeRune: RuneID => String,
    humanizeConclusion: (Conclusion) => String,
    humanizeRuleError: (ErrType) => String,
    getRuleRange: (Rule) => RangeS,
    getRuneUsages: (Rule) => Iterable[(RuneID, RangeS)],
    ruleToRunes: (Rule) => Iterable[RuneID],
    ruleToString: (Rule) => String,
    result: FailedSolve[Rule, RuneID, Conclusion, ErrType]):
  // Returns text and all line begins
  (String, Vector[CodeLocationS]) = {
    val errorBody =
      (result match {
        case FailedSolve(_, conclusions, unsolvedRules, unsolvedRunes, error) => {
          error match {
            case SolverConflict(rune, previousConclusion, newConclusion) => {
              "Conflict, thought rune " + humanizeRune(rune) + " was " + humanizeConclusion(previousConclusion) + " but now concluding it's " + humanizeConclusion(newConclusion)
            }
            case SolveIncomplete() => {
              "Couldn't solve some runes: "  + unsolvedRunes.toVector.map(humanizeRune).mkString(", ")
            }
            case RuleError(err) => {
              humanizeRuleError(err)
            }
          }
        }
      })

    val verbose = true
    val rulesToSummarize = result.unsolvedRules.filter(!getRuleRange(_).file.isInternal)

    val allLineBeginLocs =
      rulesToSummarize.flatMap(rule => {
        val range = getRuleRange(rule)
        val RangeS(begin, end) = range

        linesBetween(begin, end).map({ case RangeS(begin, _) => begin })
      })
        .distinct
    val allRuneUsages = rulesToSummarize.flatMap(getRuneUsages).distinct
    val lineBeginLocToRuneUsage =
      allRuneUsages
        .map(runeUsage => {
          val usageBeginLine = lineRangeContaining(runeUsage._2.begin).begin.offset
          (usageBeginLine, runeUsage)
        })
        .groupBy(_._1)
        .mapValues(_.map(_._2))

    val incompleteConclusions = result.steps.flatMap(_.conclusions).toMap

    val textFromUserRules =
      allLineBeginLocs
        // Show the lines in order
        .sortBy(_.offset)
        .map({ case loc @ CodeLocationS(file, lineBegin) =>
          lineContaining(loc) + "\n" +
            lineBeginLocToRuneUsage
              .getOrElse(lineBegin, Vector())
              // Show the runes from right to left
              .sortBy(-_._2.begin.offset)
              .map({ case (rune, range) =>
                val numSpaces = range.begin.offset - lineBegin
                val numArrows = Math.max(range.end.offset - range.begin.offset, 1)
                val runeName = humanizeRune(rune)
                  repeatStr(" ", numSpaces) + repeatStr("^", numArrows) + " " +
                    runeName + ": " +
                  incompleteConclusions.get(rune).map(humanizeConclusion(_)).getOrElse("(unknown)") +
                  "\n"
              }).mkString("")
        }).mkString("")

    val textFromSteps =
      "Steps:\n" +
      result.steps.foldLeft(("", Set[RuneID]()))({
        case ((stringSoFar, previouslyPrintedConclusions), Step(complex, rules, addedRules, newConclusions)) => {
          val newString =
            "" +
              (if (!complex && rules.isEmpty) "Supplied:" else "") +
              (if (complex) "(complex)  " else "") +
              rules.map(_._2).map(ruleToString).mkString("  ") + "\n" +
              (newConclusions -- previouslyPrintedConclusions).map({ case (newRune, newConclusion) =>
                "  " + humanizeRune(newRune) + ": " + humanizeConclusion(newConclusion) + "\n"
              }).mkString("") +
              addedRules.map("  added rule: " + ruleToString(_) + "\n").mkString("")
          (stringSoFar + newString, previouslyPrintedConclusions ++ newConclusions.keySet)
        }
      })._1 +
        result.unsolvedRules.map(unsolvedRule => {
          "Unsolved rule: " + ruleToString(unsolvedRule) + "\n"
        }).mkString("") +
        (if (result.unsolvedRunes.nonEmpty) {
          "Unsolved runes: " + result.unsolvedRunes.map(humanizeRune).mkString(" ")
        } else {
          ""
        })

    val text = errorBody + "\n" + textFromUserRules + textFromSteps
    (text, allLineBeginLocs.toVector)
  }

}
*/
