/*
package dev.vale.solver

import dev.vale.{CodeLocationS, FileCoordinateMap, RangeS, repeatStr}
import dev.vale.SourceCodeUtils.{lineContaining, lineRangeContaining, linesBetween}
import dev.vale.RangeS

object SolverErrorHumanizer {
*/
use crate::utils::range::{CodeLocationS, RangeS};
use crate::solver::solver::ISolverError;
use crate::solver::solver::FailedSolve;

// mig: fn humanize_failed_solve
pub fn humanize_failed_solve<'a, Rule, RuneId, Conclusion, ErrType>(
  code_map: impl Fn(&CodeLocationS<'a>) -> String,
  lines_between: impl Fn(&CodeLocationS<'a>, &CodeLocationS<'a>) -> Vec<RangeS<'a>>,
  line_range_containing: impl Fn(&CodeLocationS<'a>) -> RangeS<'a>,
  line_containing: impl Fn(&CodeLocationS<'a>) -> String,
  humanize_rune: impl Fn(RuneId) -> String,
  humanize_conclusion: impl Fn(Conclusion) -> String,
  humanize_rule_error: impl Fn(&ErrType) -> String,
  get_rule_range: impl Fn(&Rule) -> RangeS<'a>,
  get_rune_usages: impl Fn(&Rule) -> Vec<(RuneId, RangeS<'a>)>,
  rule_to_runes: impl Fn(&Rule) -> Vec<RuneId>,
  rule_to_string: impl Fn(&Rule) -> String,
  result: &FailedSolve<Rule, RuneId, Conclusion, ErrType>,
) -> (String, Vec<CodeLocationS<'a>>)
where
  RuneId: Eq + std::hash::Hash + Copy,
  Conclusion: Copy,
  CodeLocationS<'a>: PartialEq + Copy,
  RangeS<'a>: PartialEq + Copy,
{
  let error_body = match &result.error {
    ISolverError::SolverConflict(_c) => panic!("implement: humanize_failed_solve SolverConflict"),
    ISolverError::SolveIncomplete(_) => {
      let names: Vec<String> = result.unsolved_runes.iter().map(|r| humanize_rune(*r)).collect();
      format!("Couldn't solve some runes: {}", names.join(", "))
    }
    ISolverError::RuleError(rule_err) => humanize_rule_error(&rule_err.err),
  };
  let _verbose = true;
  let rules_to_summarize: Vec<&Rule> = result.unsolved_rules.iter()
    .filter(|rule| !get_rule_range(rule).file().is_internal())
    .collect();
  let all_line_begin_locs: Vec<CodeLocationS<'a>> = {
    let raw: Vec<CodeLocationS<'a>> = rules_to_summarize.iter().flat_map(|rule| {
      let range = get_rule_range(rule);
      lines_between(&range.begin, &range.end).into_iter().map(|r| r.begin)
    }).collect();
    let mut distinct = Vec::<CodeLocationS<'a>>::new();
    for item in raw { if !distinct.contains(&item) { distinct.push(item); } }
    distinct
  };
  let all_rune_usages: Vec<(RuneId, RangeS<'a>)> = {
    let raw: Vec<(RuneId, RangeS<'a>)> = rules_to_summarize.iter()
      .flat_map(|rule| get_rune_usages(rule))
      .collect();
    let mut distinct = Vec::<(RuneId, RangeS<'a>)>::new();
    for item in raw { if !distinct.contains(&item) { distinct.push(item); } }
    distinct
  };
  let line_begin_loc_to_rune_usage: std::collections::HashMap<i32, Vec<(RuneId, RangeS<'a>)>> = {
    let mut map: std::collections::HashMap<i32, Vec<(RuneId, RangeS<'a>)>> = std::collections::HashMap::new();
    for rune_usage in &all_rune_usages {
      let usage_begin_line = line_range_containing(&rune_usage.1.begin).begin.offset;
      map.entry(usage_begin_line).or_default().push(*rune_usage);
    }
    map
  };
  let incomplete_conclusions: std::collections::HashMap<RuneId, Conclusion> = result.steps.iter()
    .flat_map(|step| step.conclusions.iter().map(|(r, c)| (*r, *c)))
    .collect();
  let text_from_user_rules: String = {
    let mut sorted_locs = all_line_begin_locs.clone();
    sorted_locs.sort_by_key(|loc| loc.offset);
    sorted_locs.iter().map(|loc| {
      let line = line_containing(loc);
      let rune_lines: String = line_begin_loc_to_rune_usage.get(&loc.offset).map(|usages| {
        let mut sorted_usages = usages.clone();
        sorted_usages.sort_by_key(|u| -u.1.begin.offset);
        sorted_usages.iter().map(|(rune, range)| {
          let num_spaces = range.begin.offset - loc.offset;
          let num_arrows = std::cmp::max(range.end.offset - range.begin.offset, 1);
          let rune_name = humanize_rune(*rune);
          let conclusion_str = incomplete_conclusions.get(rune)
            .map(|c| humanize_conclusion(*c))
            .unwrap_or_else(|| "(unknown)".to_string());
          format!("{}{} {}: {}\n",
            " ".repeat(num_spaces as usize),
            "^".repeat(num_arrows as usize),
            rune_name,
            conclusion_str)
        }).collect()
      }).unwrap_or_default();
      format!("{}\n{}", line, rune_lines)
    }).collect()
  };
  let text_from_steps: String = {
    let fold_result = result.steps.iter().fold(
      ("".to_string(), std::collections::HashSet::<RuneId>::new()),
      |(string_so_far, previously_printed), step| {
        let new_string = format!("{}{}{}{}{}",
          if !step.complex && step.solved_rules.is_empty() { "Supplied:" } else { "" },
          if step.complex { "(complex)  " } else { "" },
          step.solved_rules.iter().map(|(_, r)| rule_to_string(r)).collect::<Vec<_>>().join("  ") + "\n",
          step.conclusions.iter()
            .filter(|(rune, _)| !previously_printed.contains(rune))
            .map(|(rune, conclusion)| format!("  {}: {}\n", humanize_rune(*rune), humanize_conclusion(*conclusion)))
            .collect::<String>(),
          step.added_rules.iter()
            .map(|r| format!("  added rule: {}\n", rule_to_string(r)))
            .collect::<String>(),
        );
        let mut new_printed = previously_printed;
        new_printed.extend(step.conclusions.keys().copied());
        (string_so_far + &new_string, new_printed)
      }
    );
    let unsolved_rules_str: String = result.unsolved_rules.iter()
      .map(|r| format!("Unsolved rule: {}\n", rule_to_string(r)))
      .collect();
    let unsolved_runes_str = if !result.unsolved_runes.is_empty() {
      let names: Vec<String> = result.unsolved_runes.iter().map(|r| humanize_rune(*r)).collect();
      format!("Unsolved runes: {}", names.join(" "))
    } else {
      "".to_string()
    };
    format!("Steps:\n{}{}{}", fold_result.0, unsolved_rules_str, unsolved_runes_str)
  };
  let text = format!("{}\n{}{}", error_body, text_from_user_rules, text_from_steps);
  (text, all_line_begin_locs)
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
