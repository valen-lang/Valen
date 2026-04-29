/*
package dev.vale.typing

import dev.vale.typing.ast.{AsSubtypeTE, FunctionHeaderT, PrototypeT, SignatureT}
import dev.vale.typing.names._
import dev.vale.typing.templata.CoordTemplataT
import dev.vale.typing.types._
import dev.vale.{Collector, StrI, Tests, vassert}
import dev.vale.typing.types.InterfaceTT
import org.scalatest._

import scala.collection.immutable.Set

class CompilerVirtualTests extends FunSuite with Matchers {
*/
// mig: fn regular_interface_and_struct
#[test]
#[ignore]
fn regular_interface_and_struct() {
    panic!("Unmigrated test: regular_interface_and_struct");
}
/*
  test("Regular interface and struct") {
    val compile = CompilerTestCompilation.test(
      """
        |sealed interface Opt { }
        |
        |struct Some { x int; }
        |impl Opt for Some;
      """.stripMargin)
    val interner = compile.interner
    val coutputs = compile.expectCompilerOutputs()

    // Make sure there's two drop functions
    val dropFuncNames =
      coutputs.functions.map(_.header.id).collect({
        case f @ IdT(_, _, FunctionNameT(FunctionTemplateNameT(StrI("drop"), _), _, _)) => f
      })
    vassert(dropFuncNames.size == 2)

    val interface = coutputs.lookupInterface("Opt")
    interface.internalMethods
  }
*/
// mig: fn regular_open_interface_and_struct_no_anonymous_interface
#[test]
#[ignore]
fn regular_open_interface_and_struct_no_anonymous_interface() {
    panic!("Unmigrated test: regular_open_interface_and_struct_no_anonymous_interface");
}
/*
  test("Regular open interface and struct, no anonymous interface") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveAnonymousSubstruct
        |interface Opt { }
        |
        |struct Some { x int; }
        |impl Opt for Some;
      """.stripMargin)
    val interner = compile.interner
    val coutputs = compile.expectCompilerOutputs()

    // Make sure there's two drop functions
    val dropFuncNames =
      coutputs.functions.map(_.header.id).collect({
        case f @ IdT(_, _, FunctionNameT(FunctionTemplateNameT(StrI("drop"), _), _, _)) => f
      })
    dropFuncNames.size shouldEqual 2

//    val interface = coutputs.lookupInterface("Opt")
//    interface.internalMethods.collect({
//      case (PrototypeT(FullNameT(_, _, FreeNameT(FreeTemplateNameT(_), _, coord)), _), _) => {
//        vassert(coord.kind == interface.ref)
//      }
//    }).size shouldEqual 1
  }
*/
// mig: fn implementing_two_interfaces_causes_no_vdrop_conflict
#[test]
#[ignore]
fn implementing_two_interfaces_causes_no_vdrop_conflict() {
    panic!("Unmigrated test: implementing_two_interfaces_causes_no_vdrop_conflict");
}
/*
  test("Implementing two interfaces causes no vdrop conflict") {
    // See NIIRII
    val compile = CompilerTestCompilation.test(
      """
        |struct MyStruct {}
        |
        |interface IA {}
        |impl IA for MyStruct;
        |
        |interface IB {}
        |impl IB for MyStruct;
        |
        |func bork(a IA) {}
        |func zork(b IB) {}
        |exported func main() {
        |  bork(MyStruct());
        |  zork(MyStruct());
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn upcast
#[test]
#[ignore]
fn upcast() {
    panic!("Unmigrated test: upcast");
}
/*
  test("Upcast") {
    val compile = CompilerTestCompilation.test(
      """
        |
        |interface IShip {}
        |struct Raza { fuel int; }
        |impl IShip for Raza;
        |
        |exported func main() {
        |  ship IShip = Raza(42);
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn virtual_with_body
#[test]
#[ignore]
fn virtual_with_body() {
    panic!("Unmigrated test: virtual_with_body");
}
/*
  test("Virtual with body") {
    CompilerTestCompilation.test(
      """
        |interface IBork { }
        |struct Bork { }
        |impl IBork for Bork;
        |
        |func rebork(virtual result *IBork) bool { true }
        |exported func main() {
        |  rebork(&Bork());
        |}
        |""".stripMargin)
  }
*/
// mig: fn templated_interface_and_struct
#[test]
#[ignore]
fn templated_interface_and_struct() {
    panic!("Unmigrated test: templated_interface_and_struct");
}
/*
  test("Templated interface and struct") {
    val compile = CompilerTestCompilation.test(
      """
        |sealed interface Opt<T Ref>
        |where func drop(T)void
        |{ }
        |
        |struct Some<T>
        |where func drop(T)void
        |{ x T; }
        |
        |impl<T> Opt<T> for Some<T>
        |where func drop(T)void;
        |""".stripMargin)
    val interner = compile.interner
    val coutputs = compile.expectCompilerOutputs()
    val dropFuncNames =
      coutputs.functions.map(_.header.id).collect({
        case f @ IdT(_, _, FunctionNameT(FunctionTemplateNameT(StrI("drop"), _), _, _)) => f
      })
    dropFuncNames.size shouldEqual 2
  }
*/
// mig: fn custom_drop_with_concept_function
#[test]
#[ignore]
fn custom_drop_with_concept_function() {
    panic!("Unmigrated test: custom_drop_with_concept_function");
}
/*
  test("Custom drop with concept function") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveInterfaceDrop
        |sealed interface Opt<T Ref> { }
        |
        |abstract func drop<T>(virtual opt Opt<T>)
        |where func drop(T)void;
        |
        |#!DeriveStructDrop
        |struct Some<T> { x T; }
        |impl<T> Opt<T> for Some<T>;
        |
        |func drop<T>(opt Some<T>)
        |where func drop(T)void
        |{
        |  [x] = opt;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn test_complex_interface
#[test]
#[ignore]
fn test_complex_interface() {
    panic!("Unmigrated test: test_complex_interface");
}
/*
  test("Test complex interface") {
    val compile = CompilerTestCompilation.test(
      Tests.loadExpected("programs/genericvirtuals/templatedinterface.vale"))
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn test_specializing_interface
#[test]
#[ignore]
fn test_specializing_interface() {
    panic!("Unmigrated test: test_specializing_interface");
}
/*
  test("Test specializing interface") {
    val compile = CompilerTestCompilation.test(
      Tests.loadExpected("programs/genericvirtuals/specializeinterface.vale"))
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn use_bound_from_struct
#[test]
#[ignore]
fn use_bound_from_struct() {
    panic!("Unmigrated test: use_bound_from_struct");
}
/*
  test("Use bound from struct") {
    // See NBIFP.
    // Without it, when it tries to compile (1), at (2) it tries to resolve BorkForwarder
    // and fails bound (3) because (1) has no such bound.
    // NBIFP says we should first get that knowledge from (2).
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveStructDrop
        |struct BorkForwarder<Lam>
        |where func __call(&Lam)int // 3
        |{
        |  lam Lam;
        |}
        |
        |
        |func bork<Lam>( // 1
        |  self &BorkForwarder<Lam> // 2
        |) int {
        |  return (self.lam)();
        |}
        |
        |exported func main() {
        |  b = BorkForwarder({ 7 });
        |  b.bork();
        |  [_] = b;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn basic_interface_forwarder
#[test]
#[ignore]
fn basic_interface_forwarder() {
    panic!("Unmigrated test: basic_interface_forwarder");
}
/*
  test("Basic interface forwarder") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveInterfaceDrop
        |sealed interface Bork {
        |  func bork(virtual self &Bork) int;
        |}
        |
        |#!DeriveStructDrop
        |struct BorkForwarder<Lam>
        |where func drop(Lam)void, func __call(&Lam)int {
        |  lam Lam;
        |}
        |
        |impl<Lam> Bork for BorkForwarder<Lam>;
        |
        |func bork<Lam>(self &BorkForwarder<Lam>) int {
        |  return (self.lam)();
        |}
        |
        |exported func main() int {
        |  f = BorkForwarder({ 7 });
        |  z = f.bork();
        |  [_] = f;
        |  return z;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn generic_interface_forwarder
#[test]
#[ignore]
fn generic_interface_forwarder() {
    panic!("Unmigrated test: generic_interface_forwarder");
}
/*
  test("Generic interface forwarder") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveInterfaceDrop
        |sealed interface Bork<T Ref> {
        |  func bork(virtual self &Bork<T>) int;
        |}
        |
        |#!DeriveStructDrop
        |struct BorkForwarder<T Ref, Lam>
        |where func drop(Lam)void, func __call(&Lam)T {
        |  lam Lam;
        |}
        |
        |impl<T, Lam> Bork<T> for BorkForwarder<T, Lam>;
        |
        |func bork<T, Lam>(self &BorkForwarder<T, Lam>) T {
        |  return (self.lam)();
        |}
        |
        |exported func main() int {
        |  f = BorkForwarder<int>({ 7 });
        |  z = f.bork();
        |  [_] = f;
        |  return z;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn generic_interface_forwarder_with_bound
#[test]
#[ignore]
fn generic_interface_forwarder_with_bound() {
    panic!("Unmigrated test: generic_interface_forwarder_with_bound");
}
/*
  test("Generic interface forwarder with bound") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveInterfaceDrop
        |sealed interface Bork<T Ref>
        |where func threeify(T)T {
        |  func bork(virtual self &Bork<T>) int;
        |}
        |
        |#!DeriveStructDrop
        |struct BorkForwarder<T Ref, Lam>
        |where func drop(Lam)void, func __call(&Lam)T, func threeify(T)T {
        |  lam Lam;
        |}
        |
        |impl<T, Lam> Bork<T> for BorkForwarder<T, Lam>;
        |
        |func bork<T, Lam>(self &BorkForwarder<T, Lam>) T {
        |  return (self.lam)().threeify();
        |}
        |
        |func threeify(x int) int { 3 }
        |
        |exported func main() int {
        |  f = BorkForwarder<int>({ 7 });
        |  z = f.bork();
        |  [_] = f;
        |  return z;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn basic_interface_anonymous_subclass
#[test]
#[ignore]
fn basic_interface_anonymous_subclass() {
    panic!("Unmigrated test: basic_interface_anonymous_subclass");
}
/*
  test("Basic interface anonymous subclass") {
    val compile = CompilerTestCompilation.test(
      """
        |interface Bork {
        |  func bork(virtual self &Bork) int;
        |}
        |
        |exported func main() int {
        |  f = Bork({ 7 });
        |  return f.bork();
        |}
        |
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn integer_is_compatible_with_interface_anonymous_substruct
#[test]
#[ignore]
fn integer_is_compatible_with_interface_anonymous_substruct() {
    panic!("Unmigrated test: integer_is_compatible_with_interface_anonymous_substruct");
}
/*
  test("Integer is compatible with interface anonymous substruct") {
    // We had a bug where the forwarder function was trying to solve the interface rules.
    // But the forwarder function is just:
    //   struct Forwarder<R, P1, Lam>
    //   where func __call(Lam, P1)R
    //   { }
    //   func forwarder:__call<R, P1, Lam>(&Forwarder<R, P1, Lam>, P1)R { }
    // and doesn't ever mention the interface.
    // We would just take out any mention of the interface, but it's hard to inherit everything but the interface.
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.drop.*;
        |interface AFunction2<R Ref, P1 Ref> {
        |  func doCall(virtual this &AFunction2<R, P1>, a P1) R;
        |}
        |func __call(x6 int, x42 int)str { "hi" }
        |exported func main() str {
        |  func = AFunction2<str, int>(6);
        |  return func.doCall(42);
        |}
    """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn lambda_is_compatible_with_interface_anonymous_substruct
#[test]
#[ignore]
fn lambda_is_compatible_with_interface_anonymous_substruct() {
    panic!("Unmigrated test: lambda_is_compatible_with_interface_anonymous_substruct");
}
/*
  test("Lambda is compatible with interface anonymous substruct") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.str.*;
        |
        |interface AFunction2<R Ref, P1 Ref> {
        |  func __call(virtual this &AFunction2<R, P1>, a P1) R;
        |}
        |exported func main() str {
        |  func = AFunction2<str, int>((i) => { str(i) });
        |  return func(42);
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn implementing_a_non_generic_interface_call
#[test]
#[ignore]
fn implementing_a_non_generic_interface_call() {
    panic!("Unmigrated test: implementing_a_non_generic_interface_call");
}
/*
  test("Implementing a non-generic interface call") {
    val compile = CompilerTestCompilation.test(
      """
        |#!DeriveInterfaceDrop
        |interface IObserver<T Ref> { }
        |
        |#!DeriveStructDrop
        |struct MyThing { }
        |
        |impl<T> IObserver<T> for MyThing;
        |
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }
*/
// mig: fn anonymous_substruct_8
#[test]
#[ignore]
fn anonymous_substruct_8() {
    panic!("Unmigrated test: anonymous_substruct_8");
}
/*
  test("Anonymous substruct 8") {
    val compile = CompilerTestCompilation.test(
      """
        |import v.builtins.arrays.*;
        |//import array.make.*;
        |
        |interface IThing {
        |  func __call(virtual self &IThing, i int) int;
        |}
        |
        |struct MyThing { }
        |func __call(self &MyThing, i int) int { i }
        |
        |impl IThing for MyThing;
        |
        |exported func main() int {
        |  i IThing = MyThing();
        |  a = Array<imm, int>(10, &i);
        |  return a.3;
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
  }

}

*/
