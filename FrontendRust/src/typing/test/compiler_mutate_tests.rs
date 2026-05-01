/*
package dev.vale.typing

import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.{CodeLocationS, Collector, Err, FileCoordinateMap, Interner, PackageCoordinate, RangeS, vassert, vfail}
import dev.vale._
import dev.vale.highertyping.ProgramA
import dev.vale.parsing._
import dev.vale.postparsing.PostParser
import OverloadResolver.{FindFunctionFailure, WrongNumberOfArguments}
import dev.vale.postparsing._
import dev.vale.typing.ast.{ConstantIntTE, LocalLookupTE, MutateTE, ReferenceMemberLookupTE, SignatureT}
import dev.vale.typing.names._
import dev.vale.typing.types._
import dev.vale.typing.ast._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import org.scalatest._

import scala.collection.immutable.List
import scala.io.Source

class CompilerMutateTests extends FunSuite with Matchers {
  // TODO: pull all of the typingpass specific stuff out, the unit test-y stuff
*/
// mig: fn read_code_from_resource
pub fn read_code_from_resource(resource_filename: &str) -> String {
  panic!("Unimplemented: read_code_from_resource");
}
/*
  def readCodeFromResource(resourceFilename: String): String = {
    val is = Source.fromInputStream(getClass().getClassLoader().getResourceAsStream(resourceFilename))
    vassert(is != null)
    is.mkString("")
  }
*/
// mig: fn test_mutating_a_local_var
#[test]
#[ignore]
fn test_mutating_a_local_var() {
  panic!("Unmigrated test: test_mutating_a_local_var");
}
/*
  test("Test mutating a local var") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |exported func main() {a = 3; set a = 4; }
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs();
    val main = coutputs.lookupFunction("main")
    Collector.only(main, { case MutateTE(LocalLookupTE(_,ReferenceLocalVariableT(CodeVarNameT(StrI("a")), VaryingT, _)), ConstantIntTE(IntegerTemplataT(4), _, _)) => })

    val lookup = Collector.only(main, { case l @ LocalLookupTE(range, localVariable) => l })
    val resultCoord = lookup.result.coord
    resultCoord shouldEqual CoordT(ShareT, RegionT(), IntT.i32)
  }
*/
// mig: fn test_mutable_member_permission
#[test]
#[ignore]
fn test_mutable_member_permission() {
  panic!("Unmigrated test: test_mutable_member_permission");
}
/*
  test("Test mutable member permission") {
    val compile =
      CompilerTestCompilation.test(
        """
          |
          |struct Engine { fuel int; }
          |struct Spaceship { engine! Engine; }
          |exported func main() {
          |  ship = Spaceship(Engine(10));
          |  set ship.engine = Engine(15);
          |}
          |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs();
    val main = coutputs.lookupFunction("main")

    val lookup = Collector.only(main, { case l @ ReferenceMemberLookupTE(_, _, _, _, _) => l })
    val resultCoord = lookup.result.coord
    // See RMLRMO, it should result in the same type as the member.
    resultCoord match {
      case CoordT(OwnT, _, StructTT(_)) =>
      case x => vfail(x.toString)
    }
  }
*/
// mig: fn local_set_upcasts
#[test]
#[ignore]
fn local_set_upcasts() {
  panic!("Unmigrated test: local_set_upcasts");
}
/*
  test("Local-set upcasts") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |interface IXOption<T Ref> where func drop(T)void { }
        |struct XSome<T Ref> where func drop(T)void { value T; }
        |impl<T Ref> IXOption<T> for XSome<T> where func drop(T)void;
        |struct XNone<T Ref> where func drop(T)void { }
        |impl<T Ref> IXOption<T> for XNone<T> where func drop(T)void;
        |
        |exported func main() {
        |  m IXOption<int> = XNone<int>();
        |  set m = XSome(6);
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case MutateTE(_, UpcastTE(_, _, _)) =>
    })
  }
*/
// mig: fn expr_set_upcasts
#[test]
#[ignore]
fn expr_set_upcasts() {
  panic!("Unmigrated test: expr_set_upcasts");
}
/*
  test("Expr-set upcasts") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.drop.*;
        |
        |interface IXOption<T Ref> where func drop(T)void { }
        |struct XSome<T Ref> where func drop(T)void { value T; }
        |impl<T Ref> IXOption<T> for XSome<T>;
        |struct XNone<T Ref> where func drop(T)void { }
        |impl<T Ref> IXOption<T> for XNone<T>;
        |
        |struct Marine {
        |  weapon! IXOption<int>;
        |}
        |exported func main() {
        |  m = Marine(XNone<int>());
        |  set m.weapon = XSome(6);
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case MutateTE(_, UpcastTE(_, _, _)) =>
    })
  }
*/
// mig: fn reports_when_we_try_to_mutate_an_imm_struct
#[test]
#[ignore]
fn reports_when_we_try_to_mutate_an_imm_struct() {
  panic!("Unmigrated test: reports_when_we_try_to_mutate_an_imm_struct");
}
/*
  test("Reports when we try to mutate an imm struct") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct Vec3 imm { x float; y float; z float; }
        |exported func main() int {
        |  v = Vec3(3.0, 4.0, 5.0);
        |  set v.x = 10.0;
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(CantMutateFinalMember(_, structTT, memberName)) => {
        structTT match {
          case StructTT(IdT(_, _, StructNameT(StructTemplateNameT(StrI("Vec3")), Vector()))) =>
        }
        memberName match {
          case CodeVarNameT(StrI("x")) =>
        }
      }
    }
  }
*/
// mig: fn reports_when_we_try_to_mutate_a_final_member_in_a_struct
#[test]
#[ignore]
fn reports_when_we_try_to_mutate_a_final_member_in_a_struct() {
  panic!("Unmigrated test: reports_when_we_try_to_mutate_a_final_member_in_a_struct");
}
/*
  test("Reports when we try to mutate a final member in a struct") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |struct Vec3 { x float; y float; z float; }
        |exported func main() int {
        |  v = Vec3(3.0, 4.0, 5.0);
        |  set v.x = 10.0;
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(CantMutateFinalMember(_, structTT, memberName)) => {
        structTT match {
          case StructTT(IdT(_, _, StructNameT(StructTemplateNameT(StrI("Vec3")), Vector()))) =>
        }
        memberName match {
          case CodeVarNameT(StrI("x")) =>
        }
      }
    }
  }
*/
// mig: fn reports_when_we_try_to_mutate_an_element_in_an_imm_static_sized_array
#[test]
#[ignore]
fn reports_when_we_try_to_mutate_an_element_in_an_imm_static_sized_array() {
  panic!("Unmigrated test: reports_when_we_try_to_mutate_an_element_in_an_imm_static_sized_array");
}
/*
  test("Reports when we try to mutate an element in an imm static-sized array") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.arrays.*;
        |import v.builtins.drop.*;
        |
        |exported func main() int {
        |  arr = #[#10]({_});
        |  set arr[4] = 10;
        |  return 73;
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(CantMutateFinalElement(_, arrRef2)) => {
        arrRef2.kind match {
          case contentsStaticSizedArrayTT(IntegerTemplataT(10),MutabilityTemplataT(ImmutableT),VariabilityTemplataT(FinalT),CoordT(ShareT,_, IntT(_)), _) =>
        }
      }
    }
  }
*/
// mig: fn reports_when_we_try_to_mutate_a_local_variable_with_wrong_type
#[test]
#[ignore]
fn reports_when_we_try_to_mutate_a_local_variable_with_wrong_type() {
  panic!("Unmigrated test: reports_when_we_try_to_mutate_a_local_variable_with_wrong_type");
}
/*
  test("Reports when we try to mutate a local variable with wrong type") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |exported func main() {
        |  a = 5;
        |  set a = "blah";
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(CouldntConvertForMutateT(_, CoordT(ShareT, _, IntT.i32), CoordT(ShareT, RegionT(), StrT()))) =>
      case _ => vfail()
    }
  }
*/
// mig: fn reports_when_we_try_to_override_a_non_interface
#[test]
#[ignore]
fn reports_when_we_try_to_override_a_non_interface() {
  panic!("Unmigrated test: reports_when_we_try_to_override_a_non_interface");
}
/*
  test("Reports when we try to override a non-interface") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |impl int for Bork;
        |struct Bork { }
        |exported func main() {
        |  Bork();
        |}
        |""".stripMargin)
    compile.getCompilerOutputs() match {
      case Err(CantImplNonInterface(_, KindTemplataT(IntT(32)))) =>
      case _ => vfail()
    }
  }
*/
// mig: fn can_mutate_an_element_in_a_runtime_sized_array
#[test]
#[ignore]
fn can_mutate_an_element_in_a_runtime_sized_array() {
  panic!("Unmigrated test: can_mutate_an_element_in_a_runtime_sized_array");
}
/*
  test("Can mutate an element in a runtime-sized array") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.arrays.*;
        |import v.builtins.drop.*;
        |
        |exported func main() int {
        |  arr = Array<mut, int>(3);
        |  arr.push(0);
        |  arr.push(1);
        |  arr.push(2);
        |  set arr[1] = 10;
        |  return 73;
        |}
        |""".stripMargin)
    compile.expectCompilerOutputs()
  }
*/
// mig: fn can_restackify_in_destructure_pattern
#[test]
#[ignore]
fn can_restackify_in_destructure_pattern() {
  panic!("Unmigrated test: can_restackify_in_destructure_pattern");
}
/*
  test("Can restackify in destructure pattern") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveStructDrop
        |struct Ship { fuel int; }
        |
        |/// TODO: Bring tuples back
        |#!DeriveStructDrop
        |struct GetFuelResult { fuel int; ship Ship; }
        |
        |func GetFuel(ship Ship) GetFuelResult {
        |  return GetFuelResult(ship.fuel, ship);
        |}
        |
        |exported func main() int {
        |  ship = Ship(42);
        |  [fuel, set ship] = GetFuel(ship);
        |  [f] = ship;
        |  return fuel;
        |}
        |""".stripMargin)
    compile.expectCompilerOutputs()
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
    val nameStr = interner.intern(StrI("main"))
    val testPackageCoord = PackageCoordinate.TEST_TLD(interner, keywords)
    val tzCodeLoc = CodeLocationS.testZero(interner)
    val fireflyKind = StructTT(IdT(PackageCoordinate.TEST_TLD(interner, keywords), Vector.empty, interner.intern(StructNameT(StructTemplateNameT(StrI("Firefly")), Vector.empty))))
    val fireflyCoord = CoordT(OwnT,RegionT(), fireflyKind)
    val serenityKind = StructTT(IdT(PackageCoordinate.TEST_TLD(interner, keywords), Vector.empty, interner.intern(StructNameT(StructTemplateNameT(StrI("Serenity")), Vector.empty))))
    val serenityCoord = CoordT(OwnT,RegionT(), serenityKind)

    val filenamesAndSources = FileCoordinateMap.test(interner, "blah blah blah\nblah blah blah")

    val humanizePos = (x: CodeLocationS) => SourceCodeUtils.humanizePos(filenamesAndSources, x)
    val linesBetween = (x: CodeLocationS, y: CodeLocationS) => SourceCodeUtils.linesBetween(filenamesAndSources, x, y)
    val lineRangeContaining = (x: CodeLocationS) => SourceCodeUtils.lineRangeContaining(filenamesAndSources, x)
    val lineContaining = (x: CodeLocationS) => SourceCodeUtils.lineContaining(filenamesAndSources, x)

    val tz = List(RangeS.testZero(interner))
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntFindTypeT(tz, CodeNameS(interner.intern(StrI("Spaceship"))))).nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntFindFunctionToCallT(
        tz,
        FindFunctionFailure(interner.intern(CodeNameS(interner.intern(StrI("")))), Vector.empty, Map())))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CannotSubscriptT(
        tz,
        fireflyKind))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntFindIdentifierToLoadT(
        tz,
        interner.intern(CodeNameS(StrI("spaceship")))))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntFindMemberT(
        tz,
        "hp"))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      BodyResultDoesntMatch(
        tz,
        FunctionNameS(interner.intern(StrI("myFunc")), tzCodeLoc), fireflyCoord, serenityCoord))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntConvertForReturnT(
        tz,
        fireflyCoord, serenityCoord))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntConvertForMutateT(
        tz,
        fireflyCoord, serenityCoord))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CouldntConvertForMutateT(
        tz,
        fireflyCoord, serenityCoord))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CantMoveOutOfMemberT(
        tz,
        interner.intern(CodeVarNameT(StrI("hp")))))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CantReconcileBranchesResults(
        tz,
        fireflyCoord,
        serenityCoord))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CantUseUnstackifiedLocal(
        tz,
        interner.intern(CodeVarNameT(StrI("firefly")))))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      FunctionAlreadyExists(
        tz.head,
        tz.head,
        IdT(testPackageCoord, Vector.empty, interner.intern(FunctionNameT(interner.intern(FunctionTemplateNameT(interner.intern(StrI("myFunc")), tz.head.begin)), Vector(), Vector())))))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CantMutateFinalMember(
        tz,
        serenityKind,
        interner.intern(CodeVarNameT(StrI("bork")))))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      LambdaReturnDoesntMatchInterfaceConstructor(
        tz))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      IfConditionIsntBoolean(
        tz, fireflyCoord))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      WhileConditionIsntBoolean(
        tz, fireflyCoord))
      .nonEmpty)
    vassert(CompilerErrorHumanizer.humanize(false, humanizePos, linesBetween, lineRangeContaining, lineContaining,
      CantImplNonInterface(
        tz, KindTemplataT(fireflyKind)))
      .nonEmpty)
  }
}
*/
