/*
package dev.vale

import dev.vale.passmanager.FullCompilationOptions
import dev.vale.finalast._
import dev.vale.von.VonInt
import dev.vale.{finalast => m}
import org.scalatest._

import scala.collection.immutable.List
*/
// mig: struct ImportTests
pub struct ImportTests;
/*
class ImportTests extends FunSuite with Matchers {
*/
// mig: fn tests_import
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_import() {
    panic!("Unmigrated test: tests_import");
}
/*
  test("Tests import") {
    val moduleACode =
      """
        |import moduleB.moo;
        |
        |exported func main() int {
        |  a = moo();
        |  return a;
        |}
      """.stripMargin

    val moduleBCode =
      """
        |func moo() int { return 42; }
      """.stripMargin

    val interner = new Interner()
    val keywords = new Keywords(interner)
    val map = new FileCoordinateMap[String]()
    map.put(
      interner.intern(FileCoordinate(
        interner.intern(PackageCoordinate(
          interner.intern(StrI("moduleA")),
          Vector.empty)),
        "moduleA.vale")),
      moduleACode)
    map.put(
      interner.intern(FileCoordinate(
        interner.intern(PackageCoordinate(interner.intern(StrI("moduleB")), Vector.empty)),
        "moduleB.vale")),
      moduleBCode)

    val compile =
      new RunCompilation(
        interner,
        keywords,
        Vector(PackageCoordinate.BUILTIN(interner, keywords), interner.intern(PackageCoordinate(interner.intern(StrI("moduleA")), Vector.empty))),
        Builtins.getCodeMap(interner, keywords)
          .or(map)
          .or(Tests.getPackageToResourceResolver),
        FullCompilationOptions())

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn tests_non_imported_module_isnt_brought_in
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_non_imported_module_isnt_brought_in() {
    panic!("Unmigrated test: tests_non_imported_module_isnt_brought_in");
}
/*
  test("Tests non-imported module isn't brought in") {
    val moduleACode =
      """
        |exported func main() int {
        |  a = 42;
        |  return a;
        |}
      """.stripMargin

    val moduleBCode =
      """
        |func moo() int { return 73; }
      """.stripMargin


    val interner = new Interner()
    val keywords = new Keywords(interner)
    val moduleACoord = interner.intern(PackageCoordinate(interner.intern(StrI("moduleA")), Vector.empty))
    val moduleBCoord = interner.intern(PackageCoordinate(interner.intern(StrI("moduleB")), Vector.empty))
    val map = new FileCoordinateMap[String]()
    map.put(
      interner.intern(FileCoordinate(moduleACoord, "moduleA.vale")),
      moduleACode)
    map.put(
      interner.intern(FileCoordinate(moduleBCoord, "moduleB.vale")),
      moduleBCode)

    val compile =
      new RunCompilation(
        interner,
        keywords,
        Vector(PackageCoordinate.BUILTIN(interner, keywords), moduleACoord),
        Builtins.getCodeMap(interner, keywords)
          .or(map)
          .or(Tests.getPackageToResourceResolver),
        FullCompilationOptions())

    vassert(!compile.getParseds().getOrDie().packageCoordToFileCoords.contains(moduleBCoord))

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn tests_import_with_paackage
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_import_with_paackage() {
    panic!("Unmigrated test: tests_import_with_paackage");
}
/*
  test("Tests import with paackage") {
    val moduleACode =
      """
        |import moduleB.bork.*;
        |
        |exported func main() int {
        |  a = moo();
        |  return a;
        |}
      """.stripMargin

    val moduleBCode =
      """
        |func moo() int { return 42; }
      """.stripMargin

    val interner = new Interner()
    val keywords = new Keywords(interner)
    val moduleACoord = interner.intern(PackageCoordinate(interner.intern(StrI("moduleA")), Vector.empty))
    val moduleBCoord = interner.intern(PackageCoordinate(interner.intern(StrI("moduleB")), Vector(interner.intern(StrI("bork")))))
    val map = new FileCoordinateMap[String]()
    map.put(
      interner.intern(FileCoordinate(moduleACoord, "moduleA.vale")),
      moduleACode)
    map.put(
      interner.intern(FileCoordinate(moduleBCoord, "moduleB.vale")),
      moduleBCode)

    val compile =
      new RunCompilation(
        interner,
        keywords,
        Vector(PackageCoordinate.BUILTIN(interner, keywords), moduleACoord),
        Builtins.getCodeMap(interner, keywords)
          .or(map)
          .or(Tests.getPackageToResourceResolver),
        FullCompilationOptions())

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn tests_import_of_directory_with_no_vale_files
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_import_of_directory_with_no_vale_files() {
    panic!("Unmigrated test: tests_import_of_directory_with_no_vale_files");
}
/*
  test("Tests import of directory with no vale files") {
    val moduleACode =
      """
        |import moduleB.bork.*;
        |
        |exported func main() int {
        |  a = 42;
        |  return a;
        |}
      """.stripMargin

    val interner = new Interner()
    val keywords = new Keywords(interner)
    val moduleACoord = interner.intern(PackageCoordinate(interner.intern(StrI("moduleA")), Vector.empty))
    val moduleBCoord = interner.intern(PackageCoordinate(interner.intern(StrI("moduleB")), Vector(interner.intern(StrI("bork")))))
    val map = new FileCoordinateMap[String]()
    map.put(
      interner.intern(FileCoordinate(moduleACoord, "moduleA.vale")),
      moduleACode)

    val compile =
      new RunCompilation(
        interner,
        keywords,
        Vector(PackageCoordinate.BUILTIN(interner, keywords), interner.intern(PackageCoordinate(interner.intern(StrI("moduleA")), Vector.empty))),
        Builtins.getCodeMap(interner, keywords)
          .or(Tests.getPackageToResourceResolver)
          .or(map)
          .or({ case PackageCoordinate(StrI("moduleB"), Vector(StrI("bork"))) => Some(Map()) }),
    FullCompilationOptions())

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }

}

*/
