use bumpalo::Bump;
use crate::parse_arena::ParseArena;
use crate::keywords::Keywords;
use crate::lexing::errors::ParseError;
use crate::parsing::tests::utils::compile_denizen;

/*
package dev.vale.parsing.functions

import dev.vale.lexing.{BadFunctionBodyError, FailedParse, FuncBoundWithoutWhere, LightFunctionMustHaveParamTypes}
import dev.vale.parsing._
import dev.vale.parsing.ast._
import dev.vale.{Collector, StrI, vassertOne}
import org.scalatest._


class AfterRegionsFunctionTests extends FunSuite with Collector with TestParseUtils {
*/
// mig: fn func_with_func_bound_with_missing_where
#[test]
#[ignore = "ignored upstream in Scala (`// This test does not pass yet, use #[ignore].`): parser doesn't yet detect missing 'where' for func bound"]
fn func_with_func_bound_with_missing_where() {
  // This test does not pass yet, use #[ignore].
  let parse_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let keywords = Keywords::new_for_parse(&parse_arena);
  let err = compile_denizen(
    &parse_arena,
    &keywords,
    "func sum<T>() func moo(&T)void {3}").unwrap_err();
  match err {
    ParseError::FuncBoundWithoutWhere(_) => {}
    other => panic!("Expected FuncBoundWithoutWhere, got {:?}", other),
  }
}
/*
  // This test does not pass yet, use #[ignore].
  test("Func with func bound with missing 'where'") {
    compileDenizen("func sum<T>() func moo(&T)void {3}").expectErr() match {
      case FailedParse(_, _, FuncBoundWithoutWhere(_)) =>
    }
  }

}
*/
