// mig: struct ArrayListTest
pub struct ArrayListTest;
/*
package dev.vale

import dev.vale.typing.ast.LetNormalTE
import dev.vale.typing.env.AddressibleLocalVariableT
import dev.vale.typing.names.{CodeVarNameT, IdT}
import dev.vale.typing.types.VaryingT
import dev.vale.typing.names.CodeVarNameT
import dev.vale.von.VonInt
import org.scalatest._

class ArrayListTest extends FunSuite with Matchers {
*/
// mig: fn simple_array_list_no_optionals
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_array_list_no_optionals() { panic!("Unmigrated test: simple_array_list_no_optionals"); }
/*
  test("Simple ArrayList, no optionals") {
    val compile = RunCompilation.test(
        """
          |import v.builtins.migrate.*;
          |
          |#!DeriveStructDrop
          |struct List<E Ref> {
          |  array! []<mut>E;
          |}
          |func drop<E>(self List<E>)
          |where func drop(E)void {
          |  [array] = self;
          |  drop(array);
          |}
          |func len<E>(list &List<E>) int { return len(&list.array); }
          |func add<E>(list &List<E>, newElement E) {
          |  oldArray = set list.array = Array<mut, E>(len(&list) + 1);
          |  migrate(oldArray, list.array);
          |  list.array.push(newElement);
          |}
          |func get<E>(list &List<E>, index int) &E {
          |  a = list.array;
          |  return a[index];
          |}
          |exported func main() int {
          |  l = List<int>(Array<mut, int>(0));
          |  add(&l, 5);
          |  add(&l, 9);
          |  add(&l, 7);
          |  return l.get(1);
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn doubling_array_list
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn doubling_array_list() { panic!("Unmigrated test: doubling_array_list"); }
/*
  test("Doubling ArrayList") {
    val compile = RunCompilation.test(
      """
        |import list.*;
        |
        |exported func main() int {
        |  l = List<int>(Array<mut, int>(0));
        |  add(&l, 5);
        |  add(&l, 9);
        |  add(&l, 7);
        |  return l.get(1);
        |}
        |
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn array_list_zero_constructor
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn array_list_zero_constructor() { panic!("Unmigrated test: array_list_zero_constructor"); }
/*
  test("Array list zero-constructor") {
    val compile = RunCompilation.test(
        """import list.*;
          |
          |exported func main() int {
          |  l = List<int>();
          |  add(&l, 5);
          |  add(&l, 9);
          |  add(&l, 7);
          |  return l.get(1);
          |}
          |
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn array_list_len
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn array_list_len() { panic!("Unmigrated test: array_list_len"); }
/*
  test("Array list len") {
    val compile = RunCompilation.test(
        """import list.*;
          |
          |exported func main() int {
          |  l = List<int>();
          |  add(&l, 5);
          |  add(&l, 9);
          |  add(&l, 7);
          |  return l.len();
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn array_list_set
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn array_list_set() { panic!("Unmigrated test: array_list_set"); }
/*
  test("Array list set") {
    val compile = RunCompilation.test(
        """import list.*;
          |
          |exported func main() int {
          |  l = List<int>();
          |  add(&l, 5);
          |  add(&l, 9);
          |  add(&l, 7);
          |  set(&l, 1, 11);
          |  return l.get(1);
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(11) => }
  }
*/
// mig: fn array_list_with_optionals_with_mutable_element
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn array_list_with_optionals_with_mutable_element() { panic!("Unmigrated test: array_list_with_optionals_with_mutable_element"); }
/*
  test("Array list with optionals with mutable element") {
    val compile = RunCompilation.test(
        """import list.*;
          |struct Marine { hp int; }
          |
          |exported func main() int {
          |  l =
          |      List<Marine>(
          |          Array<mut, Marine>(
          |              0,
          |              (index) => { Marine(index) }));
          |  add(&l, Marine(5));
          |  add(&l, Marine(9));
          |  add(&l, Marine(7));
          |  return l.get(1).hp;
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn mutate_mutable_from_in_lambda
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn mutate_mutable_from_in_lambda() { panic!("Unmigrated test: mutate_mutable_from_in_lambda"); }
/*
  test("Mutate mutable from in lambda") {
    val compile = RunCompilation.test(
        """import list.*;
          |struct Marine { hp int; }
          |
          |exported func main() int {
          |  m = Marine(6);
          |  lam = {
          |    set m = Marine(9);
          |  };
          |  lam();
          |  lam();
          |  return m.hp;
          |}
        """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main");
    Collector.only(main, {
      case LetNormalTE(AddressibleLocalVariableT(CodeVarNameT(StrI("m")), VaryingT, _), _) => {
        vpass()
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn move_mutable_from_in_lambda
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn move_mutable_from_in_lambda() { panic!("Unmigrated test: move_mutable_from_in_lambda"); }
/*
  test("Move mutable from in lambda") {
    val compile = RunCompilation.test(
      """import list.*;
        |struct Marine { hp int; }
        |
        |exported func main() int {
        |  m Opt<Marine> = Some(Marine(6));
        |  lam = {
        |    m2 = (set m = None<Marine>()).get();
        |    m2.hp
        |  };
        |  return lam();
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main");
    Collector.only(main, { case LetNormalTE(AddressibleLocalVariableT(CodeVarNameT(StrI("m")), VaryingT, _), _) => })

    compile.evalForKind(Vector()) match { case VonInt(6) => }
  }
*/
// mig: fn remove_from_middle
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn remove_from_middle() { panic!("Unmigrated test: remove_from_middle"); }
/*
  test("Remove from middle") {
    val compile = RunCompilation.test(
        """import list.*;
          |import panicutils.*;
          |struct Marine { hp int; }
          |
          |exported func main() {
          |  l = List<Marine>();
          |  add(&l, Marine(5));
          |  add(&l, Marine(7));
          |  add(&l, Marine(9));
          |  add(&l, Marine(11));
          |  add(&l, Marine(13));
          |  l.remove(2);
          |  vassert(l.get(0).hp == 5);
          |  vassert(l.get(1).hp == 7);
          |  vassert(l.get(2).hp == 11);
          |  vassert(l.get(3).hp == 13);
          |}
        """.stripMargin)

    compile.evalForKind(Vector())
  }
*/
// mig: fn remove_from_beginning
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn remove_from_beginning() { panic!("Unmigrated test: remove_from_beginning"); }
/*
  test("Remove from beginning") {
    val compile = RunCompilation.test(
        """import list.*;
          |import panicutils.*;
          |struct Marine { hp int; }
          |
          |exported func main() {
          |  l = List<Marine>();
          |  add(&l, Marine(5));
          |  add(&l, Marine(7));
          |  l.remove(0);
          |  l.remove(0);
          |  vassert(l.len() == 0);
          |}
        """.stripMargin)

    compile.evalForKind(Vector())
  }
}
*/
