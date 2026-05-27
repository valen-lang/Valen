package dev.vale

import dev.vale.passmanager.FullCompilationOptions
import dev.vale.finalast._
import dev.vale.typing.types.ShareT
import dev.vale.testvm.PanicException
import dev.vale.simplifying._
import dev.vale.von.VonInt
import dev.vale.{finalast => m}
import org.scalatest._

import scala.collection.immutable.List

class HammerTests extends FunSuite with Matchers {
  // Hammer tests only test the general structure of things, not the generated nodes.
  // The generated nodes will be tested by end-to-end tests.

  test("Simple main") {
    val compile = RunCompilation.test(
      "exported func main() int { return 3; }")
    val hamuts = compile.getHamuts()

    val testPackage = hamuts.lookupPackage(PackageCoordinate.TEST_TLD(compile.interner, compile.keywords))
    vassert(testPackage.getAllUserFunctions.size == 1)
    testPackage.getAllUserFunctions.head.prototype.id.fullyQualifiedName shouldEqual """main"""
  }

//  // Make sure a ListNode struct made it out
//  test("Templated struct makes it into hamuts") {
//    val compile = RunCompilation.test(
//      """
//        |struct ListNode<T> imm where T: Ref {
//        |  tail: *ListNode<T>;
//        |}
//        |func main(a: *ListNode:int) {}
//      """.stripMargin)
//    val hamuts = compile.getHamuts()
//    hamuts.structs.find(_.fullName.parts.last.humanName == "ListNode").get;
//  }

  test("Two templated structs make it into hamuts") {
    val compile = RunCompilation.test(
      """
        |func __pretend<T>() T { __vbi_panic() }
        |
        |interface MyOption<T Ref imm> imm { }
        |struct MyNone<T Ref imm> imm { }
        |impl<T Ref imm> MyOption<T> for MyNone<T>;
        |struct MySome<T Ref imm> imm { value T; }
        |impl<T Ref imm> MyOption<T> for MySome<T>;
        |
        |exported func main() {
        |  x = __pretend<MySome<int>>();
        |  y = __pretend<MyNone<int>>();
        |  z MyOption<int> = x;
        |}
      """.stripMargin)
    val packageH = compile.getHamuts().lookupPackage(PackageCoordinate.TEST_TLD(compile.interner, compile.keywords))
    vassertSome(
      packageH.interfaces.find(interface => {
        interface.id.fullyQualifiedName == """MyOption<i32>"""
      }))

    val mySome = packageH.structs.find(_.id.fullyQualifiedName == """MySome<i32>""").get;
    vassert(mySome.members.size == 1);
    vassert(mySome.members.head.tyype == CoordH[IntHT](MutableShareH, InlineH, IntHT.i32))

    val myNone = packageH.structs.find(_.id.fullyQualifiedName == """MyNone<i32>""").get;
    vassert(myNone.members.isEmpty);
  }

  // Known failure 2020-08-20
  // Maybe we can turn off tree shaking?
  // Maybe this just violates requirements?
//  test("Virtual and override functions make it into hamuts") {
//    val compile = RunCompilation.test(
//      """
//        |interface Blark imm { }
//        |func wot(virtual b *Blark) int abstract;
//        |struct MyStruct export imm {}
//        |impl Blark for MyStruct;
//        |func wot(b *MyStruct impl Blark) int { return 9; }
//      """.stripMargin)
//    val packageH = compile.getHamuts().lookupPackage(PackageCoordinate.TEST_TLD(compile.interner, compile.keywords))
//    packageH.nonExternFunctions.find(f => f.prototype.fullName.fullyQualifiedName.startsWith("""F("wot"""")).get;
//    packageH.nonExternFunctions.find(f => f.prototype.fullName.fullyQualifiedName == """F("MyStruct")""").get;
//    vassert(packageH.abstractFunctions.size == 2)
//    vassert(packageH.getAllUserImplementedFunctions.size == 1)
//    vassert(packageH.getAllUserFunctions.size == 1)
//  }

  test("Tests stripping things after panic") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  __vbi_panic();
        |  a = 42;
        |  return a;
        |}
      """.stripMargin)
    val packageH = compile.getHamuts().lookupPackage(PackageCoordinate.TEST_TLD(compile.interner, compile.keywords))
    val main = packageH.lookupFunction("main")
    main.body match {
      case BlockH(CallH(PrototypeH(fullNameH, Vector(), CoordH(_, _, NeverHT(_))), Vector())) => {
        vassert(fullNameH.fullyQualifiedName.contains("__vbi_panic"))
      }
    }
  }

  test("panic in expr") {
    val compile = RunCompilation.test(
      """
        |import intrange.*;
        |
        |exported func main() int {
        |  return 3 + __vbi_panic();
        |}
        |""".stripMargin)
    val packageH = compile.getHamuts().lookupPackage(PackageCoordinate.TEST_TLD(compile.interner, compile.keywords))
    val main = packageH.lookupFunction("main")
    val intExpr =
      main.body match {
        case BlockH(
          ConsecutorH(Vector(
            intExpr,
            CallH(
              PrototypeH(_,Vector(),CoordH(_,_,NeverHT(_))),
              Vector())))) => {
          intExpr
        }
      }
    Collector.only(intExpr, {
      case ConstantIntH(3, 32) =>
    })
  }

  test("Tests export function") {
    val compile = RunCompilation.test(
      """
        |exported func moo() int { return 42; }
        |""".stripMargin)
    val packageH = compile.getHamuts().lookupPackage(PackageCoordinate.TEST_TLD(compile.interner, compile.keywords))
    val moo = packageH.lookupFunction("moo")
    packageH.exportNameToFunction(compile.interner.intern(StrI("moo"))) shouldEqual moo.prototype
  }

  test("Tests export struct") {
    val compile = RunCompilation.test(
      """
        |exported struct Moo { }
        |""".stripMargin)
    val packageH = compile.getHamuts().lookupPackage(PackageCoordinate.TEST_TLD(compile.interner, compile.keywords))
    val moo = packageH.lookupStruct("Moo")
    packageH.exportNameToKind(compile.interner.intern(StrI("Moo"))) shouldEqual moo.getRef
  }

  test("Tests export interface") {
    val compile = RunCompilation.test(
      """
        |exported interface Moo { }
        |""".stripMargin)
    val packageH = compile.getHamuts().lookupPackage(PackageCoordinate.TEST_TLD(compile.interner, compile.keywords))
    val moo = packageH.lookupInterface("Moo")
    packageH.exportNameToKind(compile.interner.intern(StrI("Moo"))) shouldEqual moo.getRef
  }

  test("Tests exports from two modules, different names") {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    val map = new FileCoordinateMap[String]()
    map.put(
      interner.intern(FileCoordinate(
        interner.intern(PackageCoordinate(
          interner.intern(StrI("moduleA")),
          Vector.empty)),
        "StructA.vale")),
      "exported struct StructA { a int; }")
    map.put(
      interner.intern(FileCoordinate(
        interner.intern(PackageCoordinate(
          interner.intern(StrI("moduleB")),
          Vector.empty)),
        "StructB.vale")),
      "exported struct StructB { a int; }")

    val compile =
      new RunCompilation(
        interner,
        keywords,
        Vector(
          PackageCoordinate.BUILTIN(interner, keywords),
          interner.intern(PackageCoordinate(
            interner.intern(StrI("moduleA")),
            Vector.empty)),
          interner.intern(PackageCoordinate(
            interner.intern(StrI("moduleB")),
            Vector.empty))),
        Builtins.getCodeMap(interner, keywords)
          .or(map)
          .or(Tests.getPackageToResourceResolver),
        FullCompilationOptions())
    val hamuts = compile.getHamuts()

    val packageA = hamuts.lookupPackage(interner.intern(PackageCoordinate(compile.interner.intern(StrI("moduleA")), Vector.empty)))
    val fullNameA = vassertSome(packageA.exportNameToKind.get(compile.interner.intern(StrI("StructA"))))

    val packageB = hamuts.lookupPackage(interner.intern(PackageCoordinate(compile.interner.intern(StrI("moduleB")), Vector.empty)))
    val fullNameB = vassertSome(packageB.exportNameToKind.get(compile.interner.intern(StrI("StructB"))))

    vassert(fullNameA != fullNameB)
  }

  // Intentional known failure, need to separate things internally inside Hammer
//  test("Tests exports from two modules, same name") {
//    val compile =
//      new RunCompilation(
//        Vector(PackageCoordinate.BUILTIN, PackageCoordinate(compile.interner.intern(StrI("moduleA")), Vector.empty), PackageCoordinate(compile.interner.intern(StrI("moduleB")), Vector.empty)),
//        Builtins.getCodeMap()
//          .or(
//            FileCoordinateMap(Map())
//              .add("moduleA", Vector.empty, "MyStruct.vale", "struct MyStruct export { a int; }")
//              .add("moduleB", Vector.empty, "MyStruct.vale", "struct MyStruct export { a int; }"))
//          .or(Tests.getPackageToResourceResolver),
//        FullCompilationOptions())
//    val hamuts = compile.getHamuts()
//
//    val packageA = hamuts.lookupPackage(PackageCoordinate(compile.interner.intern(StrI("moduleA")), Vector.empty))
//    val fullNameA = vassertSome(packageA.exportNameToKind.get("StructA"))
//
//    val packageB = hamuts.lookupPackage(PackageCoordinate(compile.interner.intern(StrI("moduleB")), Vector.empty))
//    val fullNameB = vassertSome(packageB.exportNameToKind.get("StructA"))
//
//    vassert(fullNameA != fullNameB)
//  }

  test("Top-level extern function's wire-format SimpleId has flat shape") {
    // numInheritedGenericParameters is 0 for a top-level extern, so Hammer should not reshape.
    // The leaf step retains whatever templateArgs the function has (empty here, since this is
    // a non-generic extern). This is a smoke test that the no-reshape path returns rawSimpleId.
    val compile = RunCompilation.test(
      """
        |extern struct Vec<T> imm;
        |extern func VecOuterNew<T>() Vec<T>;
        |exported func main() int {
        |  v = VecOuterNew<int>();
        |  return 42;
        |}
        |""".stripMargin,
      false)
    val hamuts = compile.getHamuts()
    val packageH = hamuts.lookupPackage(PackageCoordinate.TEST_TLD(compile.interner, compile.keywords))
    val externs = packageH.prototypeToExtern.values.toVector
    val outerNew = externs.find(_.simpleId.steps.last.name == "VecOuterNew").get
    // VecOuterNew<int>: leaf step keeps own template arg, no reshape (no parent citizen).
    val leaf = outerNew.simpleId.steps.last
    vassert(leaf.name == "VecOuterNew")
    vassert(leaf.templateArgs.size == 1)  // <int>
  }

  test("Mixed own + inherited template args split correctly in wire-format SimpleId") {
    // Per @PRIIROZ, the function's templateArgs are ordered [own..., inherited...]. For
    // `Foo<A>.bar<C>(c C)` monomorphized as `Foo<i32>.bar<i64>(42i64)`, the leaf step's
    // templateArgs are [i64, i32] (C own first, A inherited last). After reshape, the
    // 1 trailing inherited arg moves to the Foo step:
    //   [..., Foo<i32>, bar<i64>]
    // Asserts both the splitAt count (1, not 0 or 2) and the splitAt direction. Uses
    // i32 + i64 (not bool/str etc.) because NameHammer.simplifyKind currently only
    // handles IntIT.
    val compile = RunCompilation.test(
      """
        |extern struct Foo<A> imm {
        |  extern func bar<C>(c C) int;
        |}
        |exported func main() int {
        |  return Foo<int>.bar<str>("hello");
        |}
        |""".stripMargin,
      false)
    val hamuts = compile.getHamuts()
    val packageH = hamuts.lookupPackage(PackageCoordinate.TEST_TLD(compile.interner, compile.keywords))
    val externs = packageH.prototypeToExtern.values.toVector
    val barExtern = externs.find(_.simpleId.steps.last.name == "bar").get
    val steps = barExtern.simpleId.steps
    vassert(steps.size >= 2)
    val leaf = steps.last
    val parent = steps(steps.size - 2)
    vassert(leaf.name == "bar")
    vassert(leaf.templateArgs.size == 1)  // C (own) remains on the leaf.
    vassert(parent.name == "Foo")
    vassert(parent.templateArgs.size == 1)  // A (inherited) moved up to the citizen.
  }

  test("Extern method in generic extern struct puts container args on citizen step in wire-format SimpleId") {
    // The reshape moves the inherited container template args (T -> i32) off the leaf function
    // step onto the immediately preceding citizen step. Final shape: [..., Vec<i32>, new]
    // rather than [..., Vec, new<i32>]. This is what Backend's rustifySimpleId expects per
    // @SMLRZ, so it can emit `Vec<i32>::new` rather than `Vec::new<i32>`.
    val compile = RunCompilation.test(
      """
        |extern struct Vec<T> imm {
        |  extern func new() Vec<T>;
        |}
        |exported func main() int {
        |  v = Vec<int>.new();
        |  return 42;
        |}
        |""".stripMargin,
      false)
    val hamuts = compile.getHamuts()
    val packageH = hamuts.lookupPackage(PackageCoordinate.TEST_TLD(compile.interner, compile.keywords))
    val externs = packageH.prototypeToExtern.values.toVector
    val newExtern = externs.find(_.simpleId.steps.last.name == "new").get
    val steps = newExtern.simpleId.steps
    vassert(steps.size >= 2)
    val leaf = steps.last
    val parent = steps(steps.size - 2)
    vassert(leaf.name == "new")
    vassert(leaf.templateArgs.isEmpty)  // No own template args, no surviving args after reshape.
    vassert(parent.name == "Vec")
    vassert(parent.templateArgs.size == 1)  // <i32> moved up from the leaf.
  }
}
