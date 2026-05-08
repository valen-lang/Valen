package dev.vale.postparsing

import dev.vale._
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.{FinalP, LoadAsBorrowP, MutableP, UseP}
import dev.vale.postparsing.patterns.{AtomSP, CaptureS}
import dev.vale.postparsing.rules.{LiteralSR, MaybeCoercingLookupSR, MutabilityLiteralSL, RuneUsage}
import dev.vale.parsing._
import dev.vale.parsing.ast._
import dev.vale.postparsing.patterns._
import dev.vale.postparsing.rules._
import dev.vale.solver.{FailedSolve, SolveIncomplete}
import org.scalatest._

class PostParserTests extends FunSuite with Matchers with Collector {

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
      case Ok(t) => vfail("Accidentally compiled!\n")
    }
  }

  vregionmut() // Put this back in with regions
  // test("Every function gets region generic param") {
  //   val program1 = compile("func moo() int { 3 }")
  //
  //   val moo = program1.lookupFunction("moo")
  //   moo.genericParams match {
  //     case Vector(
  //       GenericParameterS(_,
  //         RuneUsage(_,DenizenDefaultRegionRuneS(_)),
  //         RegionGenericParameterTypeS(ReadWriteRegionS),
  //         None)) =>
  //   }
  // }

  // See: User Must Specify Enough Identifying Runes (UMSEIR)
  test("Test UMSEIR") {
    // This should work, its fine that the _ is there because we can always figure out what
    // that rune is, from the identifying runes.
    val main =
    compile(
      """
        |func moo<T>(a T)
        |where K Ref, T = Map<K, _> { ... }
        |""".stripMargin).lookupFunction("moo")

    // This should fail, because we can't figure out what it is, given the identifying runes.
    val error = compileForError(
      """
        |func moo<K, V>(a Map<K, V, _>) { ... }
        |""".stripMargin)
    error match {
      case IdentifyingRunesIncompleteS(_, IdentifiabilitySolveError(_, FailedSolve(_, _, _, unsolvedRunes, SolveIncomplete()))) => {
        // The param rune, and the _ rune are both unknown
        vassert(unsolvedRunes.size == 2)
      }
    }
  }

  test("Lookup +") {
    val program1 = compile("exported func main() int { return +(3, 4); }")
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    val ret = Collector.only(block.expr, { case x @ ReturnSE(_, _) => x })
    val call = Collector.only(ret.inner, { case x @ FunctionCallSE(_, _, _, _) => x })
    Collector.only(call.callableExpr, { case x @ OverloadSetSE(OutsideLoadSE(_, _, Vector(LoadPartSE(CodeNameS(StrI("+")), _)))) => x })
  }

  test("Struct") {
    val program1 = compile("struct Moo { x int; }")
    val imoo = program1.lookupStruct("Moo")

    imoo.headerRules shouldHave {
      case LiteralSR(_, r, MutabilityLiteralSL(MutableP)) => vassert(r == imoo.mutabilityRune)
    }
    imoo.memberRules shouldHave {
      case MaybeCoercingLookupSR(_, m, CodeNameS(StrI("int"))) => vassert(m == imoo.members(0).typeRune)
    }
    imoo.members match {
      case Vector(NormalStructMemberS(_, StrI("x"), FinalP, _)) =>
    }
  }

  test("Linear struct") {
    val program1 = compile("linear struct Moo { x int; }")

    val myStruct = program1.lookupStruct("Moo")
    Collector.only(myStruct.attributes, {
      case MacroCallS(_, DontCallMacroP, StrI("DeriveStructDrop")) =>
    })
  }

  test("Lambda") {
    val program1 = compile("exported func main() int { return {_ + _}(4, 6); }")

    val CodeBodyS(BodySE(_, _, BlockSE(_, _, expr))) = program1.lookupFunction("main").body
    val lambda =
      Collector.only(expr, {
        case ReturnSE(_, FunctionCallSE(_, _, OwnershippedSE(_, FunctionSE(lambda@FunctionS(_, _, _, _, _, _, _, _, _, _)), LoadAsBorrowP), _)) => lambda
      })
    // See: Lambdas Dont Need Explicit Identifying Runes (LDNEIR)
    lambda.genericParams match {
      case Vector(
        GenericParameterS(_,RuneUsage(_,mp1b @ MagicParamRuneS(_)),CoordGenericParameterTypeS(None,_,false),None),
        GenericParameterS(_,RuneUsage(_,mp2b @ MagicParamRuneS(_)),CoordGenericParameterTypeS(None,_,false),None)
        // Put this back in when we have regions
        // , _
        ) => {
        vassert(mp1b != mp2b) // Two different runes
      }
    }

    vregionmut() // see above
  }

  test("Interface") {
    val program1 = compile("interface IMoo { func blork(virtual this &IMoo, a bool)void; }")
    val imoo = program1.lookupInterface("IMoo")

    val blork = imoo.internalMethods.head
    blork.name match {
      case FunctionNameS(StrI("blork"), _) =>
    }
  }

  test("Generic interface") {
    val interner = new Interner()
    val program1 = compile("interface IMoo<T> { func blork(virtual this &IMoo, a T)void; }", interner)
    val imoo = program1.lookupInterface("IMoo")

    val blork = imoo.internalMethods.head
    blork.name match {
      case FunctionNameS(StrI("blork"), _) =>
    }

    val imooRunes = imoo.genericParams.map(_.rune)
    val t = CodeRuneS(interner.intern(StrI("T")))
    vassert(imooRunes(0).rune == t)
    vassert(imooRunes.exists(_.rune == t))
    // Interface methods of generic interfaces will have the same identifying runes of their
    // generic interfaces, see IMCBT.
    vassert(blork.genericParams.map(_.rune.rune).contains(CodeRuneS(interner.intern(StrI("T")))))
  }

  test("Impl") {
    val program1 = compile("impl IMoo for Moo;")
    val impl = program1.impls.head
    impl.rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("Moo"))) => vassert(r == impl.structKindRune)
    }
    impl.rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("IMoo"))) => vassert(r == impl.interfaceKindRune)
    }
  }

  test("Method call") {
    val program1 = compile("exported func main() int { return true.shout(); }")
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    val ret = Collector.only(block, { case r @ ReturnSE(_, _) => r })
    Collector.only(ret, { case FunctionCallSE(_, _, OverloadSetSE(OutsideLoadSE(_, _, Vector(LoadPartSE(CodeNameS(StrI("shout")), _)))), Vector(ConstantBoolSE(_,true))) => })
//    { case ReturnSE(_,FunctionCallSE(_,_,Vector()) => }
  }

  test("Moving method call") {
    val program1 = compile("exported func main() int { x = 4; return (x).shout(); }")
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    val ret = Collector.only(block, { case r @ ReturnSE(_, _) => r })
    Collector.only(ret, { case FunctionCallSE(_, _, OverloadSetSE(OutsideLoadSE(_, _, Vector(LoadPartSE(CodeNameS(StrI("shout")), _)))), Vector(LocalLoadSE(_,CodeVarNameS(StrI("x")), UseP))) => })
  }

  test("Method call with explicit template arg preserves the <T>") {
    val program1 = compile("exported func main() { x = 4; x.foo<int>(); }")
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    val outside =
      Collector.only(block, {
        case OverloadSetSE(o @ OutsideLoadSE(_, _, Vector(LoadPartSE(CodeNameS(StrI("foo")), _)))) => o
      })
    val Vector(LoadPartSE(_, fooExplicitArgs)) = outside.parts
    // Expected: <int> appears as one explicit arg rune, with a corresponding lookup rule.
    // Actual: fooExplicitArgs is empty and rules is empty — `<int>` was dropped.
    vassert(fooExplicitArgs.size == 1)
    outside.rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("int"))) => vassert(r == fooExplicitArgs.head)
    }
  }

  vregionmut() // Put this back in with regions
  // test("Pure regioned function") {
  //   val program1 = compile("pure func moo<r'>(ship &r'Spaceship) { }")
  //   val moo = program1.lookupFunction("moo")
  //
  //   moo.genericParams match {
  //     case Vector(
  //       GenericParameterS(_,RuneUsage(_,CodeRuneS(StrI(r))),RegionGenericParameterTypeS(ReadOnlyRegionS),None)
  //       // Put this back in when we have regions
  //       // ,GenericParameterS(_,RuneUsage(_,DenizenDefaultRegionRuneS(FunctionNameS(StrI("moo"),_))),RegionGenericParameterTypeS(ReadWriteRegionS),None)
  //     ) =>
  //   }
  // }

  vregionmut() // Put this back in with regions
//   test("Pure regioned function with explicit self region") {
//     val program1 = compile("pure func moo<r', t' rw>(ship &r'Spaceship) t'{ }")
//     val moo = program1.lookupFunction("moo")
//
//     moo.genericParams match {
// //      case Vector(
// //        GenericParameterS(_,RuneUsage(_,CodeRuneS(StrI("r"))),Vector(),None),
// //        GenericParameterS(_,RuneUsage(_,CodeRuneS(StrI("t"))),Vector(),None),
// //        GenericParameterS(_,RuneUsage(_,DefaultRegionRuneS()),Vector(ReadWriteRuneAttributeS(_)),None)) (of class scala.collection.immutable.Vector)
//       case Vector(
//         GenericParameterS(_,RuneUsage(_,CodeRuneS(StrI("r"))), RegionGenericParameterTypeS(ReadOnlyRegionS), None),
//         GenericParameterS(_, RuneUsage(_,CodeRuneS(StrI("t"))), RegionGenericParameterTypeS(ReadWriteRegionS), None)) =>
//     }
//   }

  test("Function with magic lambda and regular lambda") {
    // There was a bug that confused the two, and an underscore would add a magic param to every lambda after it

    val program1 =
      compile(
        """exported func main() int {
          |  {_};
          |  (a) => {a};
          |}
        """.stripMargin)
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    val BlockSE(_, _, ConsecutorSE(things)) = block
    val lambdas = Collector.all(things, { case f @ FunctionSE(_) => f }).toList
    lambdas.head.function.params match {
      case Vector(_, ParameterS(_, _, false, AtomSP(_, Some(CaptureS(MagicParamNameS(_), false)), Some(RuneUsage(_, MagicParamRuneS(_))), None))) =>
    }
    lambdas.last.function.params match {
      case Vector(_, ParameterS(_, _, false, AtomSP(_, Some(CaptureS(CodeVarNameS(StrI("a")), false)), Some(RuneUsage(_, ImplicitRuneS(_))), None))) =>
    }
  }


  test("Constructing members") {
    val program1 = compile(
      """func MyStruct() {
        |  self.x = 4;
        |  self.y = true;
        |}
        |""".stripMargin)
    val main = program1.lookupFunction("MyStruct")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    block.locals match {
      case Vector(
        LocalS(ConstructingMemberNameS(StrI("x")), NotUsed, Used, NotUsed, NotUsed, NotUsed, NotUsed),
        LocalS(ConstructingMemberNameS(StrI("y")), NotUsed, Used, NotUsed, NotUsed, NotUsed, NotUsed)) =>
    }
    val exprs = block.expr match { case ConsecutorSE(exprs) => exprs }
    Collector.only(exprs, {
      case LetSE(_,
      _,
      AtomSP(_, Some(CaptureS(ConstructingMemberNameS(StrI("x")), false)), _, None),
      ConstantIntSE(_, 4, _)) =>
    })
    Collector.only(exprs, {
      case LetSE(_,
        _,
        AtomSP(_, Some(CaptureS(ConstructingMemberNameS(StrI("y")), false)), _, None),
        ConstantBoolSE(_, true)) =>
    })
    Collector.only(exprs, {
      case FunctionCallSE(_, _,
          OverloadSetSE(OutsideLoadSE(_, _, Vector(LoadPartSE(CodeNameS(StrI("MyStruct")), _)))),
        Vector(
          LocalLoadSE(_, ConstructingMemberNameS(StrI("x")), UseP),
          LocalLoadSE(_, ConstructingMemberNameS(StrI("y")), UseP))) =>
    })
  }

  test("InitializingRuntimeSizedArrayRequiresSizeAndCallable too few") {
    val error = compileForError(
      """func MyStruct() {
        |  ship = []();
        |}
        |""".stripMargin)
    error match {
      case InitializingRuntimeSizedArrayRequiresSizeAndCallable(_) =>
    }
  }

  test("InitializingRuntimeSizedArrayRequiresSizeAndCallable too many") {
    val error = compileForError(
      """func MyStruct() {
        |  ship = [](4, {_}, 10);
        |}
        |""".stripMargin)
    error match {
      case InitializingRuntimeSizedArrayRequiresSizeAndCallable(_) =>
    }
  }

  test("InitializingStaticSizedArrayRequiresSizeAndCallable too few") {
    val error = compileForError(
      """func MyStruct() {
        |  ship = [#5]();
        |}
        |""".stripMargin)
    error match {
      case InitializingStaticSizedArrayRequiresSizeAndCallable(_) =>
    }
  }

  test("InitializingStaticSizedArrayRequiresSizeAndCallable too many") {
    val error = compileForError(
      """func MyStruct() {
        |  ship = [#5](4, {_});
        |}
        |""".stripMargin)
    error match {
      case InitializingStaticSizedArrayRequiresSizeAndCallable(_) =>
    }
  }

  test("Test loading from member") {
    val program1 = compile(
      """func main() {
        |  moo = MyStruct();
        |  return moo.x;
        |}
        |""".stripMargin)
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    Collector.only(block,
      { case ReturnSE(_, DotSE(_,LocalLoadSE(_,CodeVarNameS(StrI("moo")),LoadAsBorrowP),StrI("x"),true)) => })

  }

  test("Test loading from member 2") {
    val program1 = compile(
      """func main() {
        |  moo = MyStruct();
        |  return &moo.x;
        |}
        |""".stripMargin)
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    Collector.only(block, {
      case ReturnSE(_, OwnershippedSE(_, DotSE(_,LocalLoadSE(_,CodeVarNameS(StrI("moo")),LoadAsBorrowP),x,true),LoadAsBorrowP)) =>
    })
  }

  test("Constructing members, borrowing another member") {
    val program1 = compile(
      """func MyStruct() {
        |  self.x = 4;
        |  self.y = &self.x;
        |}
        |""".stripMargin)
    val main = program1.lookupFunction("MyStruct")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    block.locals match {
      case Vector(
        LocalS(ConstructingMemberNameS(StrI("x")), Used, Used, NotUsed, NotUsed, NotUsed, NotUsed),
        LocalS(ConstructingMemberNameS(StrI("y")), NotUsed, Used, NotUsed, NotUsed, NotUsed, NotUsed)) =>
    }
    Collector.only(block, {
      case LetSE(_, _,
        AtomSP(_, Some(CaptureS(ConstructingMemberNameS(StrI("x")), false)), _, None),
        ConstantIntSE(_, 4, _)) =>
    })
    Collector.only(block, {
      case LetSE(_, _,
        AtomSP(_, Some(CaptureS(ConstructingMemberNameS(StrI("y")), false)), _, None),
        LocalLoadSE(_, ConstructingMemberNameS(StrI("x")), LoadAsBorrowP)) =>
    })
    Collector.only(block, {
      case FunctionCallSE(_, _,
          OverloadSetSE(OutsideLoadSE(_, _, Vector(LoadPartSE(CodeNameS(StrI("MyStruct")), _)))),
        Vector(
        LocalLoadSE(_, ConstructingMemberNameS(StrI("x")), UseP),
        LocalLoadSE(_, ConstructingMemberNameS(StrI("y")), UseP))) =>
    })
  }

  test("foreach") {
    val program1 = compile(
      """func main() {
        |  myList = 0;
        |  foreach i in myList { }
        |}
        |""".stripMargin)

    val function = program1.lookupFunction("main")
    val CodeBodyS(body) = function.body
    body.block shouldHave {
      case LocalS(IterableNameS(_),Used,NotUsed,NotUsed,NotUsed,NotUsed,NotUsed) =>
    }
    body.block shouldHave {
      case LocalS(IteratorNameS(_),Used,NotUsed,NotUsed,NotUsed,NotUsed,NotUsed) =>
    }
    body.block shouldHave {
      case LocalS(IterationOptionNameS(_),Used,Used,NotUsed,NotUsed,NotUsed,NotUsed) =>
    }
    body.block shouldHave {
      case LocalS(CodeVarNameS(StrI("i")),NotUsed,NotUsed,NotUsed,NotUsed,NotUsed,NotUsed) =>
    }
    body.block shouldHave {
      case LetSE(_,_,
        AtomSP(_,Some(CaptureS(IterableNameS(_), false)),None,None),
        LocalLoadSE(_,CodeVarNameS(StrI("myList")),UseP)) =>
    }
    body.block shouldHave {
      case LetSE(_,_,
        AtomSP(_,Some(CaptureS(IteratorNameS(_), false)),None,None),
        FunctionCallSE(_,_,
            OverloadSetSE(OutsideLoadSE(_,_,Vector(LoadPartSE(CodeNameS(StrI("begin")),_)))),
          Vector(LocalLoadSE(_,IterableNameS(_),LoadAsBorrowP)))) =>
    }
    body.block shouldHave {
      case WhileSE(_, _) =>
    }
    body.block shouldHave {
      case LetSE(_,_,
        AtomSP(_,Some(CaptureS(IterationOptionNameS(_), false)),None,None),
        FunctionCallSE(_,_,
            OverloadSetSE(OutsideLoadSE(_,_,Vector(LoadPartSE(CodeNameS(StrI("next")),_)))),
          Vector(
            LocalLoadSE(_,IteratorNameS(_),LoadAsBorrowP)))) =>
    }
    body.block shouldHave {
      case FunctionCallSE(_,_,
          OverloadSetSE(OutsideLoadSE(_,_,Vector(LoadPartSE(CodeNameS(StrI("isEmpty")),_)))),
        Vector(
          LocalLoadSE(_,IterationOptionNameS(_),LoadAsBorrowP))) =>
    }
    body.block shouldHave {
      case BreakSE(_) =>
    }
    body.block shouldHave {
      case LetSE(_,_,
        AtomSP(_,Some(CaptureS(CodeVarNameS(StrI("i")), false)),None,None),
        FunctionCallSE(_,_,
            OverloadSetSE(OutsideLoadSE(_,_,Vector(LoadPartSE(CodeNameS(StrI("get")),_)))),
          Vector(LocalLoadSE(_,IterationOptionNameS(_),UseP)))) =>
    }
    body.block shouldHave {
      case LocalLoadSE(_,IterationOptionNameS(_),UseP) =>
    }
  }

  test("this isnt special if was explicit param") {
    val program1 = compile(
      """func moo(self &MyStruct) {
        |  println(self.x);
        |}
        |""".stripMargin)
    val main = program1.lookupFunction("moo")
    Collector.only(main.body, {
      case FunctionCallSE(_,_,
          OverloadSetSE(OutsideLoadSE(_, _, Vector(LoadPartSE(CodeNameS(StrI("println")), _)))),
        Vector(DotSE(_, LocalLoadSE(_, CodeVarNameS(StrI("self")), LoadAsBorrowP), StrI("x"), true))) =>
    })
    Collector.all(main.body, { case FunctionCallSE(_, _, _, _) => }).size shouldEqual 1
  }

  test("Reports when mutating nonexistant local") {
    val err = compileForError(
      """exported func main() int {
        |  set a = a + 1;
        |}
        |""".stripMargin)
    err match {
      case CouldntFindVarToMutateS(_, "a") =>
    }
  }

  test("Reports when extern function has body") {
    val err = compileForError(
      """
        |extern func bork() int {
        |  3
        |}
        |""".stripMargin)
    err match {
      case ExternHasBody(_) =>
    }
  }

  test("Reports when we forget set") {
    val err = compileForError(
      """
        |exported func main() {
        |  x = "world!";
        |  x = "changed";
        |}
        |""".stripMargin)
    err match {
      case VariableNameAlreadyExists(_, CodeVarNameS(StrI("x"))) =>
      case _ => vfail()
    }
  }

  test("Reports when interface method doesnt have self") {
    val err = compileForError("interface IMoo { func blork(a bool)void; }")
    err match {
      case InterfaceMethodNeedsSelf(_) =>
      case _ => vfail()
    }
  }

  test("Statement after result or return") {
    compileForError(
      """
        |func doCivicDance(virtual this Car) {
        |  return 4;
        |  7
        |}
        """.stripMargin) match {
      case StatementAfterReturnS(_) =>
    }
  }

  test("Report type mismatch") {
    compileForError(
      """
        |struct Vec<N, T> where N Int
        |{
        |  values [#N]<imm>T;
        |}
        |
      """.stripMargin) match {
      case RuneExplicitTypeConflictS(_, CodeRuneS(StrI("N")), _) =>
    }
  }

  test("Free function call with explicit template arg makes single-part OutsideLoadSE with one explicit arg rune") {
    val program1 = compile("exported func main() { f<int>(); }")
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    val outside =
      Collector.only(block, {
        case OverloadSetSE(x @ OutsideLoadSE(_, _, Vector(LoadPartSE(CodeNameS(StrI("f")), Vector(_))))) => x
      })
    // The single explicit arg rune should be tied to a lookup of `int` in the rules.
    val Vector(LoadPartSE(_, Vector(argRune))) = outside.parts
    outside.rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("int"))) => vassert(r == argRune)
    }
  }

  test("Namespace method call makes two-part OutsideLoadSE in source order (container first, function last)") {
    val program1 = compile("exported func main() { Vec<int>.foo(); }")
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    val (rules, intRune) =
      Collector.only(block, {
        case OverloadSetSE(OutsideLoadSE(_, rules,
          Vector(
            LoadPartSE(CodeNameS(StrI("Vec")), Vector(intRune)),
            LoadPartSE(CodeNameS(StrI("foo")), Vector())
          ))) => (rules, intRune)
      })
    rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("int"))) => vassert(r == intRune)
    }
  }

  test("Namespace method call with multi-arg container makes part with multiple explicit arg runes") {
    val program1 = compile("exported func main() { Pair<int, bool>.make(); }")
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    val (rules, intRune, boolRune) =
      Collector.only(block, {
        case OverloadSetSE(OutsideLoadSE(_, rules,
          Vector(
            LoadPartSE(CodeNameS(StrI("Pair")), Vector(intRune, boolRune)),
            LoadPartSE(CodeNameS(StrI("make")), Vector()))
          )) => (rules, intRune, boolRune)
      })
    rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("int"))) => vassert(r == intRune)
    }
    rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("bool"))) => vassert(r == boolRune)
    }
  }

  // Ignored: postparser currently drops method-level template args on namespace method calls.
  // For `S<int>.foo<bool>()`, only `int` makes it into the rules/explicit-args; the `<bool>` on
  // `foo` is silently lost. Same blocker as `Namespace method call with both container and method
  // generic args` in CompilerTests. When that's lifted, this test should pass.
  ignore("Namespace method call with both container and method args makes both parts have explicit arg runes") {
    val program1 = compile("exported func main() { S<int>.foo<bool>(); }")
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    val (rules, intRune, boolRune) =
      Collector.only(block, {
        case OverloadSetSE(OutsideLoadSE(_, rules,
          Vector(
            LoadPartSE(CodeNameS(StrI("S")), Vector(intRune)),
            LoadPartSE(CodeNameS(StrI("foo")), Vector(boolRune))),
          )) => (rules, intRune, boolRune)
      })
    rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("int"))) => vassert(r == intRune)
    }
    rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("bool"))) => vassert(r == boolRune)
    }
  }

  // Ignored: pre-existing FunctionScout limitation rejects user-declared runes on
  // interface/struct internal methods (FunctionScout.scala:114 — `case ParentInterface(_, _, _, _)
  // => vassert(userDeclaredRunes.isEmpty)`). Same blocker as the "Namespace method call with both
  // container and method generic args" CompilerTests test. When that's lifted, this should pass
  // with parent-at-end ordering (Z before K before V).
  ignore("Interface internal method has identifying runes [own, parent...] (parent runes at end)") {
    val interner = new Interner()
    val program1 = compile(
      """interface IS<K, V> { func zork<Z>(virtual this &IS<K, V>, z Z)void; }""",
      interner)
    val is = program1.lookupInterface("IS")

    val zork = is.internalMethods.head
    val zorkRunes = zork.genericParams.map(_.rune.rune)
    val z = CodeRuneS(interner.intern(StrI("Z")))
    val k = CodeRuneS(interner.intern(StrI("K")))
    val v = CodeRuneS(interner.intern(StrI("V")))
    val zIdx = zorkRunes.indexOf(z)
    val kIdx = zorkRunes.indexOf(k)
    val vIdx = zorkRunes.indexOf(v)
    vassert(zIdx >= 0 && kIdx >= 0 && vIdx >= 0)
    vassert(zIdx < kIdx)
    vassert(kIdx < vIdx)
  }

  // Ignored: struct internal methods are silently dropped at the postparser today
  // (PostParser.scala:681 — `case StructMethodP(_) => Vector.empty`). PR 2.5 wires them
  // through the same way as interface internal methods. Once that lands, uncomment the
  // body below; it should pass with the same parent-at-end ordering as interfaces.
  ignore("Struct internal method has identifying runes [own, parent...] (parent runes at end)") {
//    val interner = new Interner()
//    val program1 = compile(
//      """struct S<K, V> { func zork<Z>(self &S<K, V>, z Z)void { } }""",
//      interner)
//    val s = program1.lookupStruct("S")
//
//    val zork = s.internalMethods.head
//    val zorkRunes = zork.genericParams.map(_.rune.rune)
//    val z = CodeRuneS(interner.intern(StrI("Z")))
//    val k = CodeRuneS(interner.intern(StrI("K")))
//    val v = CodeRuneS(interner.intern(StrI("V")))
//    // Own rune Z first; inherited K and V from S at the end (in their declared order).
//    val zIdx = zorkRunes.indexOf(z)
//    val kIdx = zorkRunes.indexOf(k)
//    val vIdx = zorkRunes.indexOf(v)
//    vassert(zIdx >= 0 && kIdx >= 0 && vIdx >= 0)
//    vassert(zIdx < kIdx)
//    vassert(kIdx < vIdx)
  }

  // Ignored: FunctionScout.scala:114 rejects user-declared runes on struct internal methods.
  // When that's lifted, zork's genericParams should be [N, K, V] per @PRIIROZ.
  ignore("Struct internal method zork<N> in S<K, V> has generic params [N, K, V] (own first, parent runes at end)") {
    val interner = new Interner()
    val program1 = compile(
      """struct S<K, V> { func zork<N>(self &S<K, V>, n N)void { } }""",
      interner)
    val s = program1.lookupStruct("S")

    val zork = s.internalMethods.head
    val zorkRunes = zork.genericParams.map(_.rune.rune)
    val n = CodeRuneS(interner.intern(StrI("N")))
    val k = CodeRuneS(interner.intern(StrI("K")))
    val v = CodeRuneS(interner.intern(StrI("V")))
    val nIdx = zorkRunes.indexOf(n)
    val kIdx = zorkRunes.indexOf(k)
    val vIdx = zorkRunes.indexOf(v)
    vassert(nIdx >= 0 && kIdx >= 0 && vIdx >= 0)
    vassert(nIdx < kIdx)
    vassert(kIdx < vIdx)
  }

  // Ignored: parser doesn't yet support nested container chains like S<X>.T<Y>.foo().
  // When it does, parts should come out in source order: [S(int,bool), T(float), zork(double)].
  // At the definition site, zork's genericParams would be [N, Q, K, V] per @PRIIROZ
  // (own first, then containers' innermost-first).
  ignore("Three-part chain S<K, V>.T<Q>.zork<N> makes three-part OutsideLoadSE in source order") {
    val program1 = compile("exported func main() { S<int, bool>.T<float>.zork<double>(); }")
    val main = program1.lookupFunction("main")

    val CodeBodyS(BodySE(_, _, block)) = main.body
    val outside =
      Collector.only(block, {
        case OverloadSetSE(x @ OutsideLoadSE(_, _,
          Vector(
            LoadPartSE(CodeNameS(StrI("S")), Vector(_, _)),
            LoadPartSE(CodeNameS(StrI("T")), Vector(_)),
            LoadPartSE(CodeNameS(StrI("zork")), Vector(_))),
          )) => x
      })
    val Vector(
      LoadPartSE(_, Vector(sArg0Rune, sArg1Rune)),
      LoadPartSE(_, Vector(tArgRune)),
      LoadPartSE(_, Vector(zorkArgRune))) = outside.parts
    outside.rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("int"))) => vassert(r == sArg0Rune)
    }
    outside.rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("bool"))) => vassert(r == sArg1Rune)
    }
    outside.rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("float"))) => vassert(r == tArgRune)
    }
    outside.rules shouldHave {
      case MaybeCoercingLookupSR(_, r, CodeNameS(StrI("double"))) => vassert(r == zorkArgRune)
    }
  }
}