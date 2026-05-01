/*
package dev.vale.typing

import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing.expression.CallCompiler
import dev.vale._
import dev.vale.highertyping._
import dev.vale.typing.types._
import dev.vale._
import dev.vale.postparsing._
import dev.vale.postparsing.rules._
import dev.vale.solver.{FailedSolve, RuleError, SolveIncomplete, SolverConflict, Step}
import OverloadResolver.{FindFunctionFailure, InferFailure, SpecificParamDoesntSend, WrongNumberOfArguments}
import dev.vale.Collector.ProgramWithExpect
import dev.vale.postparsing._
import dev.vale.typing.ast._
import dev.vale.typing.infer._
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.ast._
import dev.vale.typing.templata._
import dev.vale.typing.types._
//import dev.vale.typingpass.infer.NotEnoughToSolveError
import org.scalatest._

import scala.io.Source

class CompilerSolverTests extends FunSuite with Matchers {
  // TODO: pull all of the typingpass specific stuff out, the unit test-y stuff
*/
// mig: fn read_code_from_resource
fn read_code_from_resource(resource_filename: &str) -> String {
    panic!("Unimplemented: read_code_from_resource");
}
/*
  def readCodeFromResource(resourceFilename: String): String = {
    val is = Source.fromInputStream(getClass().getClassLoader().getResourceAsStream(resourceFilename))
    vassert(is != null)
    is.mkString("")
  }
*/
// mig: fn test_simple_generic_function
#[test]
#[ignore]
fn test_simple_generic_function() {
    panic!("Unmigrated test: test_simple_generic_function");
}
/*
  test("Test simple generic function") {
    val compile = CompilerTestCompilation.test(
      """
        |func bork<T>(a T) T { return a; }
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    vassert(coutputs.getAllUserFunctions.size == 1)
  }
*/
// mig: fn test_lacking_drop_function
#[test]
#[ignore]
fn test_lacking_drop_function() {
    panic!("Unmigrated test: test_lacking_drop_function");
}
/*
  test("Test lacking drop function") {
    val compile = CompilerTestCompilation.test(
      """
        |func bork<T>(a T) { }
      """.stripMargin)
    compile.getCompilerOutputs().expectErr() match {
      case CouldntFindFunctionToCallT(_, FindFunctionFailure(CodeNameS(StrI("drop")), _, _)) =>
    }
  }
*/
// mig: fn test_having_drop_function_concept_function
#[test]
#[ignore]
fn test_having_drop_function_concept_function() {
    panic!("Unmigrated test: test_having_drop_function_concept_function");
}
/*
  test("Test having drop function concept function") {
    val compile = CompilerTestCompilation.test(
      """
        |func bork<T>(a T) where func drop(T)void { }
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val bork = coutputs.lookupFunction("bork")

    // Only identifying template arg coord should be of PlaceholderT(0)
    bork.header.id.localName.templateArgs match {
      case Vector(CoordTemplataT(CoordT(OwnT,_, KindPlaceholderT(IdT(_, _, KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, _)))))))
      =>
    }

    // Make sure it calls drop, and that it has the right placeholders
    bork.body shouldHave {
      case FunctionCallTE(
        PrototypeT(
          IdT(
            _,
            _,
            FunctionBoundNameT(
              FunctionBoundTemplateNameT(StrI("drop")),
              Vector(),
              Vector(
                CoordT(
                  OwnT,
                  _,
                  KindPlaceholderT(
                    IdT(
                      _,
                      Vector(FunctionTemplateNameT(StrI("bork"),_)),
                      KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, _)))))))),
          CoordT(ShareT,_, VoidT())),
        _,
        _) =>
    }
  }
*/
// mig: fn test_calling_a_generic_function_with_a_concept_function
#[test]
#[ignore]
fn test_calling_a_generic_function_with_a_concept_function() {
    panic!("Unmigrated test: test_calling_a_generic_function_with_a_concept_function");
}
/*
  test("Test calling a generic function with a concept function") {
    val compile = CompilerTestCompilation.test(
      """
        |func moo(x int) { }
        |
        |func bork<T>(a T) T where func moo(T)void { a }
        |
        |exported func main() {
        |  bork(3);
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    main shouldHave {
      case FunctionCallTE(
        PrototypeT(
          IdT(_,
            _,
            FunctionNameT(
              FunctionTemplateNameT(StrI("bork"), _),
              Vector(CoordTemplataT(CoordT(ShareT,RegionT(), IntT(32)))),
              Vector(CoordT(ShareT,RegionT(), IntT(32))))),
          CoordT(ShareT,RegionT(), IntT(32))),
        Vector(ConstantIntTE(IntegerTemplataT(3),32, _)),
        _) =>
    }
  }
*/
// mig: fn test_rune_type_in_generic_param
#[test]
#[ignore]
fn test_rune_type_in_generic_param() {
    panic!("Unmigrated test: test_rune_type_in_generic_param");
}
/*
  test("Test rune type in generic param") {
    val compile = CompilerTestCompilation.test(
      """
        |func bork<I Int>() int { I }
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("bork")
    main.header.id.localName.templateArgs match {
      case Vector(PlaceholderTemplataT(_, IntegerTemplataType())) =>
    }
  }
*/
// mig: fn test_single_parameter_function
#[test]
#[ignore]
fn test_single_parameter_function() {
    panic!("Unmigrated test: test_single_parameter_function");
}
/*
  test("Test single parameter function") {
    val compile = CompilerTestCompilation.test(
      """
        |struct Functor1<F Prot = func(P1)R> imm
        |where P1 Ref, R Ref { }
        |
        |func __call<F Prot = func(P1)R>(self &Functor1<F>, param1 P1) R
        |where P1 Ref, R Ref {
        |  F(param1)
        |}
        |
        |exported func main() int {
        |  Functor1({_})(4)
        |}
      """.stripMargin)

  }
*/
// mig: fn test_calling_a_generic_function_with_a_drop_concept_function
#[test]
#[ignore]
fn test_calling_a_generic_function_with_a_drop_concept_function() {
    panic!("Unmigrated test: test_calling_a_generic_function_with_a_drop_concept_function");
}
/*
  test("Test calling a generic function with a drop concept function") {
    val compile = CompilerTestCompilation.test(
      """
        |func bork<T>(a T) where func drop(T)void {
        |}
        |
        |struct Mork {}
        |
        |exported func main() {
        |  bork(Mork());
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val bork = coutputs.lookupFunction("main")
    val prototype =
      bork.body match {
        case BlockTE(
            ReturnTE(
              ConsecutorTE(
                Vector(FunctionCallTE(prototype, _, _),
                VoidLiteralTE(_))))) => prototype
      }
    prototype match {
      case PrototypeT(
          IdT(
            _,_,
            FunctionNameT(
              FunctionTemplateNameT(StrI("bork"), _),
              Vector(CoordTemplataT(templateArgCoord)),
              Vector(arg))),
          CoordT(ShareT,RegionT(), VoidT())) => {

        templateArgCoord match {
          case CoordT(
              OwnT,
          _,
          StructTT(
                IdT(_,_,
                  StructNameT(StructTemplateNameT(StrI("Mork")),Vector())))) =>
        }

        vassert(arg == templateArgCoord)
      }
    }
  }
*/
// mig: fn humanize_errors
#[test]
#[ignore]
fn humanize_errors() {
    panic!("Unmigrated test: humanize_errors");
}
/*
  test("Humanize errors") {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val tz = List(RangeS.testZero(interner))
    val testPackageCoord = PackageCoordinate.TEST_TLD(interner, keywords)
    val tzCodeLoc = CodeLocationS.testZero(interner)
    val funcTemplateName = FunctionTemplateNameT(interner.intern(StrI("main")), tzCodeLoc)
    val funcTemplateId = IdT(testPackageCoord, Vector(), funcTemplateName)
    val funcName = IdT(testPackageCoord, Vector(), FunctionNameT(FunctionTemplateNameT(interner.intern(StrI("main")), tzCodeLoc), Vector(), Vector()))
    val regionName = funcTemplateId.addStep(interner.intern(KindPlaceholderNameT(interner.intern(KindPlaceholderTemplateNameT(0, DenizenDefaultRegionRuneS(FunctionNameS(funcTemplateName.humanName, funcTemplateName.codeLocation)))))))
    val region = RegionT()


    val fireflyKind = StructTT(IdT(testPackageCoord, Vector(), StructNameT(StructTemplateNameT(StrI("Firefly")), Vector())))
    val fireflyCoord = CoordT(OwnT,region,fireflyKind)
    val serenityKind = StructTT(IdT(testPackageCoord, Vector(), StructNameT(StructTemplateNameT(StrI("Serenity")), Vector())))
    val serenityCoord = CoordT(OwnT,region,serenityKind)
    val ispaceshipKind = InterfaceTT(IdT(testPackageCoord, Vector(), InterfaceNameT(InterfaceTemplateNameT(StrI("ISpaceship")), Vector())))
    val ispaceshipCoord = CoordT(OwnT,region,ispaceshipKind)
    val unrelatedKind = StructTT(IdT(testPackageCoord, Vector(), StructNameT(StructTemplateNameT(StrI("Spoon")), Vector())))
    val unrelatedCoord = CoordT(OwnT,region,unrelatedKind)
    val fireflySignature = SignatureT(IdT(testPackageCoord, Vector(), interner.intern(FunctionNameT(interner.intern(FunctionTemplateNameT(interner.intern(StrI("myFunc")), tz.head.begin)), Vector(), Vector(fireflyCoord)))))
    val fireflyExportId = IdT(testPackageCoord, Vector(), interner.intern(ExportNameT(interner.intern(ExportTemplateNameT(tz.head.begin)), RegionT())))
    val fireflyExport = KindExportT(tz.head, fireflyKind, fireflyExportId, interner.intern(StrI("Firefly")));
    val serenityExportId = IdT(testPackageCoord, Vector(), interner.intern(ExportNameT(interner.intern(ExportTemplateNameT(tz.head.begin)), RegionT())))
    val serenityExport = KindExportT(tz.head, fireflyKind, serenityExportId, interner.intern(StrI("Serenity")));

    val codeStr = "Hello I am A large piece Of code [that has An error]"
    val filenamesAndSources = FileCoordinateMap.test(interner, codeStr)
*/
// mig: fn make_loc
fn make_loc(pos: i32) {
    panic!("Unimplemented: make_loc");
}
/*
    def makeLoc(pos: Int) = CodeLocationS(FileCoordinate.test(interner), pos)
*/
// mig: fn make_range
fn make_range(begin: i32, end: i32) {
    panic!("Unimplemented: make_range");
}
/*
    def makeRange(begin: Int, end: Int) = RangeS(makeLoc(begin), makeLoc(end))

    val humanizePos = (x: CodeLocationS) => SourceCodeUtils.humanizePos(filenamesAndSources, x)
    val linesBetween = (x: CodeLocationS, y: CodeLocationS) => SourceCodeUtils.linesBetween(filenamesAndSources, x, y)
    val lineRangeContaining = (x: CodeLocationS) => SourceCodeUtils.lineRangeContaining(filenamesAndSources, x)
    val lineContaining = (x: CodeLocationS) => SourceCodeUtils.lineContaining(filenamesAndSources, x)

    val unsolvedRules =
      Vector(
        CoordComponentsSR(
          makeRange(0, codeStr.length),
          RuneUsage(makeRange(6, 7), CodeRuneS(interner.intern(StrI("I")))),
          RuneUsage(makeRange(11, 12), CodeRuneS(interner.intern(StrI("A")))),
          RuneUsage(makeRange(33, 52), ImplicitRuneS(LocationInDenizen(Vector(7))))),
        KindComponentsSR(
          makeRange(33, 52),
          RuneUsage(makeRange(33, 52), ImplicitRuneS(LocationInDenizen(Vector(7)))),
          RuneUsage(makeRange(43, 45), CodeRuneS(interner.intern(StrI("An"))))))

    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      TypingPassSolverError(
        tz,
        FailedSolve(
          Vector(
            Step[IRulexSR, IRuneS, ITemplataT[ITemplataType]](
              false,
              Vector(),
              Vector(),
              Map(
                CodeRuneS(interner.intern(StrI("A"))) -> OwnershipTemplataT(OwnT)))).toStream,
          Map(),
          unsolvedRules,
          Vector(),
          RuleError(KindIsNotConcrete(ispaceshipKind)))))
      .nonEmpty)

    val errorText =
      CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
        TypingPassSolverError(
          tz,
          FailedSolve(
            Vector(
              Step[IRulexSR, IRuneS, ITemplataT[ITemplataType]](
                false,
                Vector(),
                Vector(),
                Map(
                  CodeRuneS(interner.intern(StrI("A"))) -> OwnershipTemplataT(OwnT)))).toStream,
            Map(
              CodeRuneS(interner.intern(StrI("A"))) -> OwnershipTemplataT(OwnT)),
            unsolvedRules,
            Vector(
              CodeRuneS(interner.intern(StrI("I"))),
              CodeRuneS(interner.intern(StrI("Of"))),
              CodeRuneS(interner.intern(StrI("An"))),
              ImplicitRuneS(LocationInDenizen(Vector(7)))),
            SolveIncomplete())))
    println(errorText)
    vassert(errorText.nonEmpty)
    vassert(errorText.contains("\n           ^ A: own"))
    vassert(errorText.contains("\n      ^ I: (unknown)"))
    vassert(errorText.contains("\n                                 ^^^^^^^^^^^^^^^^^^^ _7: (unknown)"))
  }
*/
// mig: fn simple_int_rule
#[test]
#[ignore]
fn simple_int_rule() {
    panic!("Unmigrated test: simple_int_rule");
}
/*
  test("Simple int rule") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |exported func main() int where N Int = 3 {
        |  return N;
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), { case ConstantIntTE(IntegerTemplataT(3), 32, _) => })
  }
*/
// mig: fn equals_transitive
#[test]
#[ignore]
fn equals_transitive() {
    panic!("Unmigrated test: equals_transitive");
}
/*
  test("Equals transitive") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |exported func main() int where N Int = 3, M Int = N {
        |  return M;
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), { case ConstantIntTE(IntegerTemplataT(3), 32, _) => })
  }
*/
// mig: fn one_of
#[test]
#[ignore]
fn one_of() {
    panic!("Unmigrated test: one_of");
}
/*
  test("OneOf") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |exported func main() int where N Int = any(2, 3, 4), N = 3 {
        |  return N;
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), { case ConstantIntTE(IntegerTemplataT(3), 32, _) => })
  }
*/
// mig: fn components
#[test]
#[ignore]
fn components() {
    panic!("Unmigrated test: components");
}
/*
  test("Components") {
    val compile = CompilerTestCompilation.test(
      """
        |exported struct MyStruct { }
        |exported func main() X
        |where
        |  MyStruct = Ref[O Ownership, K Kind],
        |  X Ref = Ref[borrow, K]
        |{
        |  return &MyStruct();
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    coutputs.lookupFunction("main").header.returnType match {
      case CoordT(BorrowT, _, StructTT(_)) =>
    }
  }
*/
// mig: fn prototype_rule_call_via_rune
#[test]
#[ignore]
fn prototype_rule_call_via_rune() {
    panic!("Unmigrated test: prototype_rule_call_via_rune");
}
/*
  test("Prototype rule, call via rune") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |func moo(i int, b bool) str { return "hello"; }
        |exported func main() str
        |where mooFunc Prot = func moo(int, bool)str
        |{
        |  return (mooFunc)(5, true);
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), {
      case FunctionCallTE(PrototypeT(simpleNameT("moo"), _), _, _) =>
    })
  }
*/
// mig: fn prototype_rule_call_directly
#[test]
#[ignore]
fn prototype_rule_call_directly() {
    panic!("Unmigrated test: prototype_rule_call_directly");
}
/*
  test("Prototype rule, call directly") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |func moo(i int, b bool) str { return "hello"; }
        |exported func main() str
        |where func moo(int, bool)str
        |{
        |  return moo(5, true);
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), {
      case FunctionCallTE(PrototypeT(simpleNameT("moo"), _), _, _) =>
    })
  }
*/
// mig: fn send_struct_to_struct
#[test]
#[ignore]
fn send_struct_to_struct() {
    panic!("Unmigrated test: send_struct_to_struct");
}
/*
  test("Send struct to struct") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct MyStruct {}
        |func moo(m MyStruct) { }
        |exported func main() {
        |  moo(MyStruct())
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn send_struct_to_interface
#[test]
#[ignore]
fn send_struct_to_interface() {
    panic!("Unmigrated test: send_struct_to_interface");
}
/*
  test("Send struct to interface") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct MyStruct {}
        |interface MyInterface {}
        |impl MyInterface for MyStruct;
        |func moo(m MyInterface) { }
        |exported func main() {
        |  moo(MyStruct())
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn assume_most_specific_generic_param
#[test]
#[ignore]
fn assume_most_specific_generic_param() {
    panic!("Unmigrated test: assume_most_specific_generic_param");
}
/*
  test("Assume most specific generic param") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct MyStruct {}
        |interface MyInterface {}
        |impl MyInterface for MyStruct;
        |func moo<T>(m T) where func drop(T)void { }
        |exported func main() {
        |  moo(MyStruct())
        |}
        |""".stripMargin
    )

    val coutputs = compile.expectCompilerOutputs()
    val arg =
      coutputs.lookupFunction("main").body shouldHave {
        case FunctionCallTE(_, Vector(arg), _) => arg
      }
    arg.result.coord match {
      case CoordT(_, _, StructTT(_)) =>
    }
  }
*/
// mig: fn assume_most_specific_common_ancestor
#[test]
#[ignore]
fn assume_most_specific_common_ancestor() {
    panic!("Unmigrated test: assume_most_specific_common_ancestor");
}
/*
  test("Assume most specific common ancestor") {
    val compile = CompilerTestCompilation.test(
      """
        |interface IShip {}
        |struct Firefly {}
        |impl IShip for Firefly;
        |struct Serenity {}
        |impl IShip for Serenity;
        |func moo<T>(a T, b T) where func drop(T)void { }
        |exported func main() {
        |  moo(Firefly(), Serenity())
        |}
        |""".stripMargin
    )

    val coutputs = compile.expectCompilerOutputs()
    val moo = coutputs.lookupFunction("moo")
    val main = coutputs.lookupFunction("main")
    main.body shouldHave {
      case FunctionCallTE(prototype, Vector(_, _), _) => {
        prototype.id.localName.templateArgs.head match {
          case CoordTemplataT(CoordT(_, _, InterfaceTT(_))) =>
        }
      }
    }
    Collector.all(main, {
      case UpcastTE(_, _, _) =>
    }).size shouldEqual 2
  }
*/
// mig: fn descendant_satisfying_call
#[test]
#[ignore]
fn descendant_satisfying_call() {
    panic!("Unmigrated test: descendant_satisfying_call");
}
/*
  test("Descendant satisfying call") {
    val compile = CompilerTestCompilation.test(
      """
        |interface IShip<T> where T Ref {}
        |struct Firefly<T> where T Ref {}
        |impl<T> IShip<T> for Firefly<T>;
        |func moo<T>(a IShip<T>) { }
        |exported func main() {
        |  moo(Firefly<int>())
        |}
        |""".stripMargin
    )

    val coutputs = compile.expectCompilerOutputs()
    val moo = coutputs.lookupFunction("moo")
    moo.header.params.head.tyype match {
      case CoordT(_, _, InterfaceTT(IdT(_, _, CitizenNameT(_, Vector(CoordTemplataT(CoordT(_, _, KindPlaceholderT(IdT(_,_,KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, _))))))))))) =>
    }
    val main = coutputs.lookupFunction("main")
    main.body shouldHave {
      case FunctionCallTE(
        PrototypeT(IdT(_,_, FunctionNameT(FunctionTemplateNameT(StrI("moo"), _), _, _)), _),
        Vector(
          UpcastTE(
            _,
            InterfaceTT(IdT(_,_,InterfaceNameT(InterfaceTemplateNameT(StrI("IShip")),Vector(CoordTemplataT(CoordT(ShareT,_,IntT(32))))))),
            _)),
        _) =>
    }
  }
*/
// mig: fn reports_incomplete_solve
#[test]
#[ignore]
fn reports_incomplete_solve() {
    panic!("Unmigrated test: reports_incomplete_solve");
}
/*
  test("Reports incomplete solve") {
    val interner = new Interner()
    val compile = CompilerTestCompilation.test(
      """
        |
        |exported func main() int where N Int {
        |  M
        |}
        |""".stripMargin,
      interner)
    compile.getCompilerOutputs() match {
      case Err(TypingPassSolverError(_,FailedSolve(_,_,Vector(),unsolved, SolveIncomplete()))) => {
        unsolved.toSet shouldEqual Set(CodeRuneS(interner.intern(interner.intern(StrI("N")))))
      }
    }
  }
*/
// mig: fn stamps_an_interface_template_via_a_function_return
#[test]
#[ignore]
fn stamps_an_interface_template_via_a_function_return() {
    panic!("Unmigrated test: stamps_an_interface_template_via_a_function_return");
}
/*
  test("Stamps an interface template via a function return") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |interface MyInterface<X Ref> { }
        |
        |struct SomeStruct<X Ref> where func drop(X)void { x X; }
        |impl<X> MyInterface<X> for SomeStruct<X> where func drop(X)void;
        |
        |func doAThing<T>(t T) SomeStruct<T>
        |where func drop(T)void {
        |  return SomeStruct<T>(t);
        |}
        |
        |exported func main() {
        |  doAThing(4);
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn pointer_becomes_share_if_kind_is_immutable
#[test]
#[ignore]
fn pointer_becomes_share_if_kind_is_immutable() {
    panic!("Unmigrated test: pointer_becomes_share_if_kind_is_immutable");
}
/*
  test("Pointer becomes share if kind is immutable") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |
        |struct SomeStruct imm { i int; }
        |
        |func bork(x &SomeStruct) int {
        |  return x.i;
        |}
        |
        |exported func main() int {
        |  return bork(SomeStruct(7));
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    coutputs.lookupFunction("bork").header.params.head.tyype.ownership shouldEqual ShareT
  }
*/
// mig: fn detects_conflict_between_types
#[test]
#[ignore]
fn detects_conflict_between_types() {
    panic!("Unmigrated test: detects_conflict_between_types");
}
/*
  test("Detects conflict between types") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct ShipA {}
        |struct ShipB {}
        |exported func main<N Kind>() where N Kind = ShipA, N Kind = ShipB {
        |}
        |""".stripMargin
    )
    compile.getCompilerOutputs() match {
      case Err(TypingPassSolverError(_, FailedSolve(_, _, _, _, SolverConflict(_, StructDefinitionTemplataT(_, StructA(_, TopLevelStructDeclarationNameS(StrI("ShipA"), _), _, _, _, _, _, _, _, _, _, _, _)), StructDefinitionTemplataT(_, StructA(_, TopLevelStructDeclarationNameS(StrI("ShipB"), _), _, _, _, _, _, _, _, _, _, _, _)))))) =>
      case Err(TypingPassSolverError(_, FailedSolve(_, _, _, _, SolverConflict(_, StructDefinitionTemplataT(_, StructA(_, TopLevelStructDeclarationNameS(StrI("ShipB"), _), _, _, _, _, _, _, _, _, _, _, _)), StructDefinitionTemplataT(_, StructA(_, TopLevelStructDeclarationNameS(StrI("ShipA"), _), _, _, _, _, _, _, _, _, _, _, _)))))) =>
      case Err(TypingPassSolverError(_, FailedSolve(_, _, _, _, SolverConflict(_, KindTemplataT(StructTT(IdT(_,_,StructNameT(StructTemplateNameT(StrI("ShipA")),_)))), KindTemplataT(StructTT(IdT(_,_,StructNameT(StructTemplateNameT(StrI("ShipB")),_)))))))) =>
      case Err(TypingPassSolverError(_, FailedSolve(_, _, _, _, SolverConflict(_, KindTemplataT(StructTT(IdT(_,_,StructNameT(StructTemplateNameT(StrI("ShipB")),_)))), KindTemplataT(StructTT(IdT(_,_,StructNameT(StructTemplateNameT(StrI("ShipA")),_)))))))) =>
      case Err(TypingPassSolverError(_, FailedSolve(_, _, _, _, RuleError(CallResultWasntExpectedType(_,KindTemplataT(StructTT(IdT(_,Vector(),StructNameT(StructTemplateNameT(StrI("ShipB")),Vector()))))))))) =>
      case Err(TypingPassSolverError(_, FailedSolve(_, _, _, _, RuleError(CallResultWasntExpectedType(_,KindTemplataT(StructTT(IdT(_,Vector(),StructNameT(StructTemplateNameT(StrI("ShipA")),Vector()))))))))) =>
      case other => vfail(other)
    }
  }
*/
// mig: fn can_match_kind_templata_type_against_struct_env_entry_struct_templata
#[test]
#[ignore]
fn can_match_kind_templata_type_against_struct_env_entry_struct_templata() {
    panic!("Unmigrated test: can_match_kind_templata_type_against_struct_env_entry_struct_templata");
}
/*
  test("Can match KindTemplataType() against StructEnvEntry / StructTemplata") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |#!DeriveStructDrop
        |struct SomeStruct<T>
        |{ x T; }
        |
        |func bork<X Kind, Z>() Z
        |where X Kind = SomeStruct<int>, X = SomeStruct<Z> {
        |  return 9;
        |}
        |
        |exported func main() int {
        |  return bork();
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()
    coutputs.lookupFunction("bork").header.id.localName.templateArgs.last shouldEqual CoordTemplataT(CoordT(ShareT, RegionT(), IntT(32)))
  }
*/
// mig: fn can_destructure_and_assemble_static_sized_array
#[test]
#[ignore]
fn can_destructure_and_assemble_static_sized_array() {
    panic!("Unmigrated test: can_destructure_and_assemble_static_sized_array");
}
/*
  test("Can destructure and assemble static sized array") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |import v.builtins.arrays.*;
        |import v.builtins.drop.*;
        |
        |func swap<T>(x [#2]T) [#2]T {
        |  [a, b] = x;
        |  return [#](b, a);
        |}
        |
        |exported func main() int {
        |  return swap([#](5, 7)).0;
        |}
        |""".stripMargin
    )
    val coutputs = compile.expectCompilerOutputs()

    val swap = coutputs.lookupFunction("swap")
    swap.header.id.localName.templateArgs.last match {
      case CoordTemplataT(CoordT(OwnT,_, KindPlaceholderT(IdT(_,Vector(FunctionTemplateNameT(StrI("swap"),_)),KindPlaceholderNameT(KindPlaceholderTemplateNameT(0, _)))))) =>
    }

    val main = coutputs.lookupFunction("main")
    val call =
      Collector.only(main, {
        case call @ FunctionCallTE(PrototypeT(IdT(_, _, FunctionNameT(FunctionTemplateNameT(StrI("swap"), _), _, _)), _), _, _) => call
      })
    call.callable.id.localName.templateArgs.last match {
      case CoordTemplataT(CoordT(ShareT, _, IntT(32))) =>
    }
  }
*/
// mig: fn test_equivalent_identifying_runes_in_functions
#[test]
#[ignore]
fn test_equivalent_identifying_runes_in_functions() {
    panic!("Unmigrated test: test_equivalent_identifying_runes_in_functions");
}
/*
  test("Test equivalent identifying runes in functions") {
    // Previously, the compiler would populate placeholders for all identifying runes at once.
    // This meant that it added a placeholder $T and a placeholder $Y at the same time.
    // Of course, that led to a conflict, because $T != $Y.
    // Now, we populate the placeholders one at a time. Both T and Y should be $T now.
    // This should also help when we switch to regions, where we want to say that two generic coords
    // share the same region.
    // See IRAGP.

    val compile = CompilerTestCompilation.test(
      """
        |func bork<T, Y>(a T) Y where T = Y { return a; }
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn iragp_test_equivalent_identifying_runes_in_struct
#[test]
#[ignore]
fn iragp_test_equivalent_identifying_runes_in_struct() {
    panic!("Unmigrated test: iragp_test_equivalent_identifying_runes_in_struct");
}
/*
  test("IRAGP: Test equivalent identifying runes in struct") {
    // See IRAGP, the original problem was for functions but we use the same solution for structs.
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveStructDrop
        |struct Bork<T, Y> where T = Y { t T; y Y; }
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
}
*/
