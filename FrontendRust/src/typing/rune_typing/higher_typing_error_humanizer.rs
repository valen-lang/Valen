// VCOORD: review
//
// Resurrected verbatim from the retired higher_typing pass (commit
// ed9bc564a~1 :: postparsing/post_parser_error_humanizer.rs:87). Onion-era
// rewires:
//   - IRuneTypeRuleError moved from postparsing/rune_type_solver to
//     typing/rune_typing/rune_type_solver.
//   - humanize_imprecise_name still lives in postparsing/post_parser_error_humanizer.

use crate::postparsing::post_parser_error_humanizer::humanize_imprecise_name;
use crate::typing::rune_typing::rune_type_solver::IRuneTypeRuleError;
use crate::utils::range::CodeLocationS;

pub fn humanize_rune_type_error<'s>(
  _code_map: &dyn Fn(CodeLocationS<'s>) -> String,
  error: &IRuneTypeRuleError<'s>,
) -> String {
  match error {
    IRuneTypeRuleError::FoundTemplataDidntMatchExpectedType(_) => {
      panic!("implement: humanize_rune_type_error FoundTemplataDidntMatchExpectedType");
      // "Expected " + humanizeTemplataType(expectedType) + " but found " + humanizeTemplataType(actualType)
    }
    IRuneTypeRuleError::CouldntFindType(e) => {
      format!("Couldn't find anything with the name '{}'", humanize_imprecise_name(e.name))
    }
    IRuneTypeRuleError::NotEnoughArgumentsForGenericCall(_) => {
      panic!("implement: humanize_rune_type_error NotEnoughArgumentsForGenericCall");
      // "Not enough arguments for generic call, expected at least " + (indexOfNonDefaultingParam + 1)
    }
    _ => panic!("implement: humanize_rune_type_error other"),
  }
}
