/*
package dev.vale.parsing

import dev.vale.lexing.{BadExpressionEnd, BadStartOfStatementError, ForgotSetKeyword}
import dev.vale.parsing.ast._
import dev.vale.{Collector, StrI}
import org.scalatest._

class AfterRegionsTests extends FunSuite with Collector with TestParseUtils {
*/
// mig: fn forgetting_set_when_changing
#[test]
#[ignore = "unmigrated - pending parsing-pass body migration"]
fn forgetting_set_when_changing() { panic!("Unmigrated test: forgetting_set_when_changing"); }
/*
  // This test does not pass yet, use #[ignore].
  test("Forgetting set when changing") {
    val error =
      compileStatement(
        """ship.x = 4;""".stripMargin).expectErr()
    error match {
      case ForgotSetKeyword(_) =>
    }
  }
*/
// mig: fn report_leaving_out_semicolon_or_ending_body_after_expression_for_paren
#[test]
#[ignore = "unmigrated - pending parsing-pass body migration"]
fn report_leaving_out_semicolon_or_ending_body_after_expression_for_paren() { panic!("Unmigrated test: report_leaving_out_semicolon_or_ending_body_after_expression_for_paren"); }
/*
  // This test does not pass yet, use #[ignore].
  test("Report leaving out semicolon or ending body after expression, for paren") {
    compileBlockContents(
      """
        |  a = 3;
        |  set x = 7 )
        """.stripMargin).expectErr() match {
      case BadStartOfStatementError(_) =>
    }
  }

}
*/
