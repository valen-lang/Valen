// mig: struct OwnershipTests
pub struct OwnershipTests;

/*
package dev.vale

import dev.vale.postparsing.patterns.AtomSP
import dev.vale.typing.ast.{FunctionCallTE, LetAndLendTE, LetNormalTE, UnletTE}
import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing.names.{IdT, FunctionNameT, FunctionTemplateNameT, StructNameT, StructTemplateNameT, TypingPassTemporaryVarNameT}
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.postparsing._
import dev.vale.typing._
import dev.vale.typing.ast._
import dev.vale.typing.templata.functionNameT
import dev.vale.typing.types._
import org.scalatest._
import dev.vale.von.VonInt

class OwnershipTests extends FunSuite with Matchers {
*/
// mig: fn borrowing_a_temporary_mutable_makes_a_local_var
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn borrowing_a_temporary_mutable_makes_a_local_var() {
    panic!("Unmigrated test: borrowing_a_temporary_mutable_makes_a_local_var");
}

/*
  test("Borrowing a temporary mutable makes a local var") {
    val compile = RunCompilation.test(
      """
        |struct Muta { hp int; }
        |exported func main() int {
        |  return (&Muta(9)).hp;
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, {
      case letTE @ LetAndLendTE(ReferenceLocalVariableT(TypingPassTemporaryVarNameT(_),FinalT,_),refExpr,targetOwnership) => {
        refExpr.result.coord match {
          case CoordT(OwnT, _, StructTT(simpleNameT("Muta"))) =>
        }
        targetOwnership shouldEqual BorrowT
        letTE.result.coord.ownership shouldEqual BorrowT
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn owning_ref_method_call
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn owning_ref_method_call() {
    panic!("Unmigrated test: owning_ref_method_call");
}

/*
  test("Owning ref method call") {
    val compile = RunCompilation.test(
      """
        |struct Muta { hp int; }
        |func take(m Muta) int {
        |  return m.hp;
        |}
        |exported func main() int {
        |  m = Muta(9);
        |  return (m).hp;
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn derive_drop
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn derive_drop() {
    panic!("Unmigrated test: derive_drop");
}

/*
  test("Derive drop") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |
        |struct Muta { }
        |
        |exported func main() {
        |  Muta();
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, { case FunctionCallTE(functionNameT("drop"), _, _) => })
    Collector.all(main, { case FunctionCallTE(_, _, _) => }).size shouldEqual 2

    compile.evalForKind(Vector())
  }
*/
// mig: fn custom_drop_result_is_an_owning_ref_calls_destructor
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn custom_drop_result_is_an_owning_ref_calls_destructor() {
    panic!("Unmigrated test: custom_drop_result_is_an_owning_ref_calls_destructor");
}

/*
  test("Custom drop result is an owning ref, calls destructor") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |
        |#!DeriveStructDrop
        |struct Muta { }
        |
        |func drop(m ^Muta) void {
        |  println("Destroying!");
        |  Muta[ ] = m;
        |}
        |
        |exported func main() {
        |  Muta();
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, { case FunctionCallTE(functionNameT("drop"), _, _) => })
    Collector.all(main, { case FunctionCallTE(_, _, _) => }).size shouldEqual 2

    compile.evalForStdout(Vector()) shouldEqual "Destroying!\n"
  }
*/
// mig: fn saves_return_value_then_destroys_temporary
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn saves_return_value_then_destroys_temporary() {
    panic!("Unmigrated test: saves_return_value_then_destroys_temporary");
}

/*
  test("Saves return value then destroys temporary") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |
        |#!DeriveStructDrop
        |struct Muta { hp int; }
        |
        |func drop(m ^Muta) {
        |  println("Destroying!");
        |  Muta[hp] = m;
        |}
        |
        |exported func main() int {
        |  return (Muta(10)).hp;
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, { case FunctionCallTE(functionNameT("drop"), _, _) => })

    compile.evalForKindAndStdout(Vector()) match { case (VonInt(10), "Destroying!\n") => }
  }
*/
// mig: fn calls_destructor_on_local_var
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn calls_destructor_on_local_var() {
    panic!("Unmigrated test: calls_destructor_on_local_var");
}

/*
  test("Calls destructor on local var") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |
        |#!DeriveStructDrop
        |struct Muta { }
        |
        |func drop(m ^Muta) {
        |  println("Destroying!");
        |  Muta[ ] = m;
        |}
        |
        |exported func main() {
        |  a = Muta();
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, { case FunctionCallTE(functionNameT("drop"), _, _) => })
    Collector.all(main, { case FunctionCallTE(_, _, _) => }).size shouldEqual 2

    compile.evalForStdout(Vector()) shouldEqual "Destroying!\n"
  }
*/
// mig: fn calls_destructor_on_local_var_unless_moved
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn calls_destructor_on_local_var_unless_moved() {
    panic!("Unmigrated test: calls_destructor_on_local_var_unless_moved");
}

/*
  test("Calls destructor on local var unless moved") {
    // Should call the destructor in moo, but not in main
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |
        |#!DeriveStructDrop
        |struct Muta { }
        |
        |func drop(m ^Muta) {
        |  println("Destroying!");
        |  Muta[ ] = m;
        |}
        |
        |func moo(m ^Muta) {
        |}
        |
        |exported func main() {
        |  a = Muta();
        |  moo(a);
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()

    // Destructor should only be calling println, NOT the destructor (itself)
    val destructor =
      vassertOne(
        coutputs.functions.find(func => {
          func.header.id.localName match {
            case FunctionNameT(FunctionTemplateNameT(StrI("drop"), _), _, Vector(CoordT(OwnT, _, StructTT(IdT(_, _, StructNameT(StructTemplateNameT(StrI("Muta")), _)))))) => true
            case _ => false
          }
        }))
    // The only function lookup should be println
    Collector.only(destructor, { case FunctionCallTE(functionNameT("println"), _, _) => })
    // Only one call (the above println)
    Collector.all(destructor, { case FunctionCallTE(_, _, _) => }).size shouldEqual 1

    // moo should be calling the destructor
    val moo = coutputs.lookupFunction("moo")
    Collector.only(moo, { case FunctionCallTE(functionNameT("drop"), _, _) => })
    Collector.only(moo, { case FunctionCallTE(_, _, _) => })

    // main should not be calling the destructor
    val main = coutputs.lookupFunction("main")
    Collector.all(main, { case FunctionCallTE(functionNameT("drop"), _, _) => true }).size shouldEqual 0

    compile.evalForStdout(Vector()) shouldEqual "Destroying!\n"
  }
*/
// mig: fn saves_return_value_then_destroys_local_var
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn saves_return_value_then_destroys_local_var() {
    panic!("Unmigrated test: saves_return_value_then_destroys_local_var");
}

/*
  test("Saves return value then destroys local var") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |
        |#!DeriveStructDrop
        |struct Muta { hp int; }
        |
        |func drop(m ^Muta) {
        |  println("Destroying!");
        |  Muta[hp] = m;
        |}
        |
        |exported func main() int {
        |  a = Muta(10);
        |  return a.hp;
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    Collector.only(main, { case FunctionCallTE(functionNameT("drop"), _, _) => })
    Collector.all(main, { case FunctionCallTE(_, _, _) => }).size shouldEqual 2

    compile.evalForKindAndStdout(Vector()) match { case (VonInt(10), "Destroying!\n") => }
  }
*/
// mig: fn gets_from_temporary_struct_a_members_member
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn gets_from_temporary_struct_a_members_member() {
    panic!("Unmigrated test: gets_from_temporary_struct_a_members_member");
}

/*
  test("Gets from temporary struct a member's member") {
    val compile = RunCompilation.test(
      """
        |struct Wand {
        |  charges int;
        |}
        |struct Wizard {
        |  wand ^Wand;
        |}
        |exported func main() int {
        |  return Wizard(Wand(10)).wand.charges;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(10) => }
  }

  // test that when we create a block closure, we hoist to the beginning its constructor,
  // and hoist to the end its destructor.

  // test that when we borrow an owning, we hoist its destructor to the end.
*/
// mig: fn unstackifies_local_vars
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn unstackifies_local_vars() {
    panic!("Unmigrated test: unstackifies_local_vars");
}

/*
  test("Unstackifies local vars") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  i = 0;
        |  return i;
        |}
      """.stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")

    val numVariables =
      Collector.all(main, {
        case LetAndLendTE(_, _, _) =>
        case LetNormalTE(_, _) =>
      }).size
    Collector.all(main, { case UnletTE(_) => }).size shouldEqual numVariables
  }
*/
// mig: fn basic_builder_pattern
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn basic_builder_pattern() {
    panic!("Unmigrated test: basic_builder_pattern");
}

/*
  test("Basic builder pattern") {
    val compile = RunCompilation.test(
      """
        |struct Ship { hp! int; fuel! int; }
        |func setHp(ship Ship, hp int) Ship {
        |  set ship.hp = hp;
        |  return ship;
        |}
        |func setFuel(ship Ship, fuel int) Ship {
        |  set ship.fuel = fuel;
        |  return ship;
        |}
        |exported func main() int {
        |  ship = Ship(0, 0).setHp(42).setFuel(43);
        |  return ship.hp;
        |}
        |""".stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn member_access_on_returned_owning_ref
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn member_access_on_returned_owning_ref() {
    panic!("Unmigrated test: member_access_on_returned_owning_ref");
}

/*
  test("Member access on returned owning ref") {
    val compile = RunCompilation.test(
      """
        |struct Ship { hp int; }
        |exported func main() int {
        |  return Ship(42).hp;
        |}
        |""".stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }

}

*/
