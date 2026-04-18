package dev.vale.postparsing

import dev.vale.parsing.ast.{FinalP, LoadAsBorrowP, MutableP, UseP}
import dev.vale.postparsing.patterns.{AtomSP, CaptureS}
import dev.vale.postparsing.rules.{LiteralSR, MaybeCoercingLookupSR, MutabilityLiteralSL, RuneUsage}
import dev.vale._
import org.scalatest._

class AfterRegionsErrorTests extends FunSuite with Matchers with Collector {

  private def compile(code: String, interner: Interner = new Interner()): ProgramS = {
    val compile = PostParserTestCompilation.test(code, interner)
    compile.getScoutput() match {
      case Err(e) => {
        val codeMap = compile.getCodeMap().getOrDie()
        vfail(
          PostParserErrorHumanizer.humanize(
            SourceCodeUtils.humanizePos(codeMap, _),
            SourceCodeUtils.linesBetween(codeMap, _, _),
            SourceCodeUtils.lineRangeContaining(codeMap, _),
            SourceCodeUtils.lineContaining(codeMap, _),
            e))
      }
      case Ok(t) => t.expectOne()
    }
  }

  private def compileForError(code: String): ICompileErrorS = {
    PostParserTestCompilation.test(code).getScoutput() match {
      case Err(e) => e
      case Ok(t) => vfail("Successfully compiled!\n" + t.toString)
    }
  }

  test("Reports when non-kind interface in impl") {
    val err = compileForError(
      """
        |struct Moo {}
        |interface IMoo {}
        |impl &IMoo for Moo;
        |""".stripMargin)
    err match {
      case CantOwnershipInterfaceInImpl(_) =>
    }
  }

  test("Reports when non-kind struct in impl") {
    val err = compileForError(
      """
        |struct Moo {}
        |interface IMoo {}
        |impl IMoo for &Moo;
        |""".stripMargin)
    err match {
      case CantOwnershipStructInImpl(_) =>
    }
  }

  test("Abstract func without virtual") {
    val err = compileForError(
      """
        |sealed interface ISpaceship<X Ref, Y Ref, Z Ref> { }
        |abstract func launch<X, Y, Z>(self &ISpaceship<X, Y, Z>, bork X) where func drop(X)void;
        |""".stripMargin)
    err match {
      case VirtualAndAbstractGoTogether(_) =>
    }
  }

  test("Test one-anonymous-param lambda identifying runes") {
    val bork = compile(
      """
        |exported func main() int {do((_) => { true })}
        |""".stripMargin)

    val main = bork.lookupFunction("main")
    // We dont support regions yet, so scout should filter them out.
    main.genericParams.size shouldEqual 0
    val lambda = Collector.onlyOf(main.body, classOf[FunctionSE])
    // Per @LAGTNGZ, the postparser creates one GenericParameterS per untyped lambda
    // param, regardless of whether the user wrote `<T>` or `(_)`. LAGTNGZ still governs how the typing pass
    // specializes the lambda (per-call template expansion into LambdaCallFunctionTemplateNameT).
    lambda.function.genericParams.size shouldEqual 1
    val underscoreParam =
      lambda.function.params.find(p => p.pattern.name.isEmpty).get // the `_` ignored-name param
    val underscoreRune = underscoreParam.pattern.coordRune.get.rune
    lambda.function.runeToPredictedType(underscoreRune) shouldEqual CoordTemplataType()
    lambda.function.genericParams.map(_.rune.rune) should contain (underscoreRune)
  }
}