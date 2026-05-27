package dev.vale.parsing

import dev.vale.lexing.{BadExpressionEnd, BadStartOfStatementError, ForgotSetKeyword}
import dev.vale.parsing.ast._
import dev.vale.{Collector, StrI}
import org.scalatest._

class AfterRegionsTests extends FunSuite with Collector with TestParseUtils {

  // This test does not pass yet, use #[ignore].
  test("Forgetting set when changing") {
    val error =
      compileStatement(
        """ship.x = 4;""".stripMargin).expectErr()
    error match {
      case ForgotSetKeyword(_) =>
    }
  }

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
