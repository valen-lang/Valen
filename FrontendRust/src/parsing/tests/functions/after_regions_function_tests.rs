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
#[ignore = "unmigrated - pending parsing-pass body migration"]
fn func_with_func_bound_with_missing_where() { panic!("Unmigrated test: func_with_func_bound_with_missing_where"); }
/*
  // This test does not pass yet, use #[ignore].
  test("Func with func bound with missing 'where'") {
    compileDenizen("func sum<T>() func moo(&T)void {3}").expectErr() match {
      case FailedParse(_, _, FuncBoundWithoutWhere(_)) =>
    }
  }

}
*/
