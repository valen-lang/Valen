/*
package dev.vale

import dev.vale.parsing.ast.ImmutableP
import dev.vale.typing.NewImmRSANeedsCallable
import dev.vale.typing.ast.{LetNormalTE, NewImmRuntimeSizedArrayTE, RuntimeSizedArrayLookupTE, StaticSizedArrayLookupTE}
import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing.names._
import dev.vale.typing.types._
import dev.vale.typing._
import dev.vale.typing.ast._
import dev.vale.typing.names.CodeVarNameT
import dev.vale.typing.templata.MutabilityTemplataT
import dev.vale.typing.types._
import dev.vale.von.{VonBool, VonInt, VonStr}
import org.scalatest._
*/
// mig: struct ArrayTests
pub struct ArrayTests;
/*
class ArrayTests extends FunSuite with Matchers {
*/
// mig: fn returning_static_array_from_function_and_dotting_it
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn returning_static_array_from_function_and_dotting_it() {
    panic!("Unmigrated test: returning_static_array_from_function_and_dotting_it");
}
/*
  test("Returning static array from function and dotting it") {
    val compile = RunCompilation.test(
      """
        |func makeArray() [#5]int { return [#](2, 3, 4, 5, 6); }
        |exported func main() int {
        |  a = makeArray();
        |  x = a.3;
        |  [_, _, _, _, _] = a;
        |  return x;
        |}
      """.stripMargin, false)

    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn simple_static_array_and_runtime_index_lookup
#[test]
fn simple_static_array_and_runtime_index_lookup() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let scout_arena = crate::scout_arena::ScoutArena::new(&scout_bump);
    let keywords = crate::keywords::Keywords::new_for_scout(&scout_arena);
    let parser_keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);
    let hammer_interner = crate::simplifying::hammer_interner::HammerInterner::new(&hammer_bump);
    let typing_interner = crate::typing::typing_interner::TypingInterner::new(&typing_bump);
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "\nexported func main() int {\n  i = 2;\n  a = [#](2, 3, 4, 5, 6);\n  return a[i];\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::StaticSizedArrayLookup(_) => Some(())
        );
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 4 }) => {}
        other => panic!("expected VonInt(4), got {:?}", other),
    }
}
/*
  test("Simple static array and runtime index lookup") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  i = 2;
        |  a = [#](2, 3, 4, 5, 6);
        |  return a[i];
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), {
      case StaticSizedArrayLookupTE(_,_,_,_,_, _) => {
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(4) => }
  }
*/
// mig: fn destroy_ssa_of_imms_into_function
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn destroy_ssa_of_imms_into_function() {
    panic!("Unmigrated test: destroy_ssa_of_imms_into_function");
}
/*
  test("Destroy SSA of imms into function") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  a = [#](13, 14, 15);
        |  sum = 0;
        |  drop_into(a, &(e) => { set sum = sum + e; });
        |  return sum;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn destroy_rsa_of_imms_into_function
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn destroy_rsa_of_imms_into_function() {
    panic!("Unmigrated test: destroy_rsa_of_imms_into_function");
}
/*
  test("Destroy RSA of imms into function") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  a = Array<imm, int>(3, {13 + _});
        |  sum = 0;
        |  drop_into(a, &(e) => { set sum = sum + e; });
        |  return sum;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn destroy_ssa_of_muts_into_function
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn destroy_ssa_of_muts_into_function() {
    panic!("Unmigrated test: destroy_ssa_of_muts_into_function");
}
/*
  test("Destroy SSA of muts into function") {
    val compile = RunCompilation.test(
      """
        |struct Spaceship { fuel int; }
        |exported func main() int {
        |  a = [#](Spaceship(13), Spaceship(14), Spaceship(15));
        |  sum = 0;
        |  drop_into(a, &(e) => { set sum = sum + e.fuel; });
        |  return sum;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn destroy_rsa_of_muts_into_function
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn destroy_rsa_of_muts_into_function() {
    panic!("Unmigrated test: destroy_rsa_of_muts_into_function");
}
/*
  test("Destroy RSA of muts into function") {
    val compile = RunCompilation.test(
      """
        |import array.make.*;
        |struct Spaceship { fuel int; }
        |exported func main() int {
        |  a = MakeArray<Spaceship>(3, &{Spaceship(13 + _)});
        |  sum = 0;
        |  drop_into(a, &(e) => { set sum = sum + e.fuel; });
        |  return sum;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn migrate_rsa
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn migrate_rsa() {
    panic!("Unmigrated test: migrate_rsa");
}
/*
  test("Migrate RSA") {
    val compile = RunCompilation.test(
      """
        |import array.make.*;
        |struct Spaceship { fuel int; }
        |exported func main() int {
        |  a = Array<mut, Spaceship>(3, &{Spaceship(41 + _)});
        |  b = Array<mut, Spaceship>(3);
        |  migrate(a, &b);
        |  return b[1].fuel;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn migrate_ssa
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn migrate_ssa() {
    panic!("Unmigrated test: migrate_ssa");
}
/*
  test("Migrate SSA") {
    val compile = RunCompilation.test(
      """
        |import array.make.*;
        |struct Spaceship { fuel int; }
        |exported func main() int {
        |  a = [#3](&{Spaceship(41 + _)});
        |  b = Array<mut, Spaceship>(3);
        |  migrate(a, &b);
        |  return b[1].fuel;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn unspecified_mutability_static_array_from_lambda_defaults_to_mutable
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn unspecified_mutability_static_array_from_lambda_defaults_to_mutable() {
    panic!("Unmigrated test: unspecified_mutability_static_array_from_lambda_defaults_to_mutable");
}
/*
  test("Unspecified-mutability static array from lambda defaults to mutable") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  i = 3;
        |  a = [#5](&{_ * 42});
        |  return a[1];
        |}
        |""".stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), {
      case StaticSizedArrayLookupTE(_,_,arrayType, _,_, _) => {
        arrayType.mutability shouldEqual MutabilityTemplataT(MutableT)
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn immutable_static_array_from_lambda
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn immutable_static_array_from_lambda() {
    panic!("Unmigrated test: immutable_static_array_from_lambda");
}
/*
  test("Immutable static array from lambda") {
    val compile = RunCompilation.test( Tests.loadExpected("programs/arrays/ssaimmfromcallable.vale"))

    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), {
      case StaticSizedArrayLookupTE(_,_,arrayType, _,_, _) => {
        arrayType.mutability shouldEqual MutabilityTemplataT(ImmutableT)
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn mutable_static_array_from_lambda
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn mutable_static_array_from_lambda() {
    panic!("Unmigrated test: mutable_static_array_from_lambda");
}
/*
  test("Mutable static array from lambda") {
    val compile = RunCompilation.test( Tests.loadExpected("programs/arrays/ssamutfromcallable.vale"))

    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), {
      case StaticSizedArrayLookupTE(_,_,arrayType, _,_, _) => {
        arrayType.mutability shouldEqual MutabilityTemplataT(MutableT)
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn immutable_static_array_from_values
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn immutable_static_array_from_values() {
    panic!("Unmigrated test: immutable_static_array_from_values");
}
/*
  test("Immutable static array from values") {
    val compile = RunCompilation.test( Tests.loadExpected("programs/arrays/ssaimmfromvalues.vale"))

    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), {
      case StaticSizedArrayLookupTE(_,_,arrayType, _,_, _) => {
        arrayType.mutability shouldEqual MutabilityTemplataT(ImmutableT)
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn mutable_static_array_from_values
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn mutable_static_array_from_values() {
    panic!("Unmigrated test: mutable_static_array_from_values");
}
/*
  test("Mutable static array from values") {
    val compile = RunCompilation.test( Tests.loadExpected("programs/arrays/ssamutfromvalues.vale"))

    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), {
      case StaticSizedArrayLookupTE(_,_,arrayType, _,_, _) => {
        arrayType.mutability shouldEqual MutabilityTemplataT(MutableT)
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn unspecified_mutability_runtime_array_from_lambda_defaults_to_mutable
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn unspecified_mutability_runtime_array_from_lambda_defaults_to_mutable() {
    panic!("Unmigrated test: unspecified_mutability_runtime_array_from_lambda_defaults_to_mutable");
}
/*
  test("Unspecified-mutability runtime array from lambda defaults to mutable") {
    val compile = RunCompilation.test(
      """
        |import array.make.*;
        |exported func main() int {
        |  i = 3;
        |  a = MakeArray<int>(5, &{_ * 42});
        |  return a[1];
        |}
        |""".stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), {
      case RuntimeSizedArrayLookupTE(_,_,arrayType, _, _) => {
        arrayType.mutability shouldEqual MutabilityTemplataT(MutableT)
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn immutable_runtime_array_from_lambda
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn immutable_runtime_array_from_lambda() {
    panic!("Unmigrated test: immutable_runtime_array_from_lambda");
}
/*
  test("Immutable runtime array from lambda") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/arrays/rsaimmfromcallable.vale"))

    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), {
      case RuntimeSizedArrayLookupTE(_,_,arrayType, _, _) => {
        arrayType.mutability shouldEqual MutabilityTemplataT(ImmutableT)
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn mutable_runtime_array_from_lambda
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn mutable_runtime_array_from_lambda() {
    panic!("Unmigrated test: mutable_runtime_array_from_lambda");
}
/*
  test("Mutable runtime array from lambda") {
    val compile =
      RunCompilation.test(
        Tests.loadExpected("programs/arrays/rsamutfromcallable.vale"))

    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), {
      case RuntimeSizedArrayLookupTE(_,_,arrayType, _, _) => {
        arrayType.mutability shouldEqual MutabilityTemplataT(MutableT)
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn take_arraysequence_as_a_parameter
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn take_arraysequence_as_a_parameter() {
    panic!("Unmigrated test: take_arraysequence_as_a_parameter");
}
/*
  //m [<mut> 3 * [#3]<mut>int] = [mut][ [mut][1, 2, 3], [mut][4, 5, 6], [mut][7, 8, 9] ];
  test("Take arraysequence as a parameter") {
    val compile = RunCompilation.test(
      """
        |func doThings(arr [#5]<imm>int) int {
        |  return arr.3;
        |}
        |exported func main() int {
        |  a = #[#](2, 3, 4, 5, 6);
        |  return doThings(a);
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn borrow_arraysequence_as_a_parameter
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn borrow_arraysequence_as_a_parameter() {
    panic!("Unmigrated test: borrow_arraysequence_as_a_parameter");
}
/*
  test("Borrow arraysequence as a parameter") {
    val compile = RunCompilation.test(
      """
        |struct MutableStruct {
        |  x int;
        |}
        |
        |func doThings(arr &[#3]^MutableStruct) int {
        |  return arr.2.x;
        |}
        |exported func main() int {
        |  a = [#](MutableStruct(2), MutableStruct(3), MutableStruct(4));
        |  return doThings(&a);
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(4) => }
  }
*/
// mig: fn array_map_with_int
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn array_map_with_int() {
    panic!("Unmigrated test: array_map_with_int");
}
/*
  // the argument to __Array doesnt even have to be a struct or a lambda or an
  // interface or whatever, its just passed straight through to the prototype
  test("array map with int") {
    val compile = RunCompilation.test(
      """
        |func __call(lol int, i int) int { return i; }
        |
        |exported func main() int {
        |  a = #[]int(10, 1337);
        |  return a.3;
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case NewImmRuntimeSizedArrayTE(contentsRuntimeSizedArrayTT(MutabilityTemplataT(ImmutableT), CoordT(ShareT, _, IntT(_)), _), _, _, _, _) =>
    })

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn new_rsa
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn new_rsa() {
    panic!("Unmigrated test: new_rsa");
}
/*
  test("new rsa") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  a = []int(3);
        |  a.push(73);
        |  a.push(42);
        |  a.push(73);
        |  return a.1;
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case LetNormalTE(ReferenceLocalVariableT(CodeVarNameT(StrI("a")),_,CoordT(OwnT,_,contentsRuntimeSizedArrayTT(MutabilityTemplataT(MutableT),CoordT(ShareT,_, IntT(_)), _))), _) =>
    })

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn array_map_with_lambda
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn array_map_with_lambda() {
    panic!("Unmigrated test: array_map_with_lambda");
}
/*
  test("array map with lambda") {
    val compile = RunCompilation.test(
      """
        |struct Lam imm {}
        |func __call(lam Lam, i int) int { return i; }
        |
        |exported func main() int
        |where F Prot = func(Lam, int)int {
        |  a = #[]int(10, Lam());
        |  return a.3;
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main")
    Collector.only(main, {
      case NewImmRuntimeSizedArrayTE(contentsRuntimeSizedArrayTT(MutabilityTemplataT(ImmutableT), CoordT(ShareT, _, IntT(_)), _), _, _, _, _) =>
    })

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn make_array_map_with_struct
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn make_array_map_with_struct() {
    panic!("Unmigrated test: make_array_map_with_struct");
}
/*
  test("MakeArray map with struct") {
    val compile = RunCompilation.test(
        """
          |import array.make.*;
          |
          |struct Lam imm {}
          |func __call(lam Lam, i int) int { return i; }
          |
          |exported func main() int {
          |  a = MakeArray<int>(10, Lam());
          |  return a.3;
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn make_array_map_with_lambda
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn make_array_map_with_lambda() {
    panic!("Unmigrated test: make_array_map_with_lambda");
}
/*
  test("MakeArray map with lambda") {
    val compile = RunCompilation.test(
        """
          |import array.make.*;
          |exported func main() int {
          |  a = MakeArray<int>(10, {_});
          |  return a.3;
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn array_map_with_interface
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn array_map_with_interface() {
    panic!("Unmigrated test: array_map_with_interface");
}
/*
  test("array map with interface") {
    val compile = RunCompilation.test(
        """
          |import array.make.*;
          |
          |sealed interface IThing {
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
          |""".stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn array_map_taking_a_closure_which_captures_something
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn array_map_taking_a_closure_which_captures_something() {
    panic!("Unmigrated test: array_map_taking_a_closure_which_captures_something");
}
/*
  test("Array map taking a closure which captures something") {
    val compile = RunCompilation.test(
        """import array.make.*;
          |exported func main() int {
          |  x = 7;
          |  a = MakeImmArray<int>(10, { _ + x });
          |  return a.3;
          |}
        """.stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(10) => }
  }
*/
// mig: fn simple_array_map_with_runtime_index_lookup
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_array_map_with_runtime_index_lookup() {
    panic!("Unmigrated test: simple_array_map_with_runtime_index_lookup");
}
/*
  test("Simple array map with runtime index lookup") {
    val compile = RunCompilation.test(
        """import array.make.*;
          |exported func main() int {
          |  a = MakeImmArray<int>(10, {_});
          |  i = 5;
          |  return a[i];
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn nested_array
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn nested_array() {
    panic!("Unmigrated test: nested_array");
}
/*
  test("Nested array") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  return [#]([#](2)).0.0;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(2) => }
  }

*/
// mig: fn two_dimensional_array
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn two_dimensional_array() {
    panic!("Unmigrated test: two_dimensional_array");
}
/*
  test("Two dimensional array") {
    val compile = RunCompilation.test(
        """import array.make.*;
          |exported func main() int {
          |  board =
          |      MakeArray<Array<mut, int>>(
          |          3,
          |          (row) => { MakeArray<int>(3, { row + _ }) });
          |  return board.1.2;
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn array_with_capture
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn array_with_capture() {
    panic!("Unmigrated test: array_with_capture");
}
/*
  test("Array with capture") {
    val compile = RunCompilation.test(
        """import array.make.*;
          |struct IntBox {
          |  i int;
          |}
          |
          |exported func main() int {
          |  box = IntBox(7);
          |  board = MakeArray<int>(3, &(col) => { box.i });
          |  return board.1;
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(7) => }
  }
*/
// mig: fn capture
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn capture() {
    panic!("Unmigrated test: capture");
}
/*
  test("Capture") {
    val compile = RunCompilation.test(
      """
        |func myFunc<T, F>(generator F) T
        |where func(&F, int)T, func drop(F)void
        |{
        |  return generator(9);
        |}
        |
        |struct IntBox {
        |  i int;
        |}
        |
        |exported func main() int {
        |  box = IntBox(7);
        |  lam = (col) => { box.i };
        |  board = myFunc<int>(&lam);
        |  return board;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(7) => }
  }

*/
// mig: fn mutate_array
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn mutate_array() {
    panic!("Unmigrated test: mutate_array");
}
/*
  test("Mutate array") {
    val compile = RunCompilation.test(
        """import array.make.*;
          |exported func main() int {
          |  arr = MakeArray<int>(3, {_});
          |  set arr[1] = 1337;
          |  return arr.1;
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(1337) => }
  }
*/
// mig: fn capture_mutable_array
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn capture_mutable_array() {
    panic!("Unmigrated test: capture_mutable_array");
}
/*
  test("Capture mutable array") {
    val compile = RunCompilation.test(
        """import array.make.*;
          |struct MyIntIdentity {}
          |func __call(this &MyIntIdentity, i int) int { return i; }
          |exported func main() {
          |  m = MyIntIdentity();
          |  arr = MakeArray<int>(10, &m);
          |  lam = { print(str(arr.6)); };
          |  lam();
          |}
        """.stripMargin)

    compile.evalForStdout(Vector()) shouldEqual "6"
  }
*/
// mig: fn swap_out_of_array
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn swap_out_of_array() {
    panic!("Unmigrated test: swap_out_of_array");
}
/*
  test("Swap out of array") {
    val compile = RunCompilation.test(
        """import array.make.*;
          |struct Goblin { }
          |
          |exported func main() int {
          |  arr = MakeArray<Goblin>(1, i => Goblin());
          |  set arr.0 = Goblin();
          |  return 4;
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(4) => }
  }

*/
// mig: fn test_array_length
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn test_array_length() {
    panic!("Unmigrated test: test_array_length");
}
/*
  test("Test array length") {
    val compile = RunCompilation.test(
        """import array.make.*;
          |exported func main() int {
          |  a = MakeArray<int>(11, {_});
          |  return len(&a);
          |}
        """.stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(11) => }
  }
*/
// mig: fn map_using_array_construct
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn map_using_array_construct() {
    panic!("Unmigrated test: map_using_array_construct");
}
/*
  test("Map using array construct") {
    val compile = RunCompilation.test(
        """
          |import array.make.*;
          |exported func main() int {
          |  board = MakeArray<int>(5, {_});
          |  result =
          |      MakeArray<int>(5, &(i) => {
          |        board[i] + 2
          |      });
          |  return result.2;
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(4) => }
  }
*/
// mig: fn map_from_hardcoded_values
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn map_from_hardcoded_values() {
    panic!("Unmigrated test: map_from_hardcoded_values");
}
/*
  test("Map from hardcoded values") {
    val compile = RunCompilation.test(
        """
          |import array.make.*;
          |func toArray<N Int, E, SourceM Mutability>(seq &[#N]<SourceM>E) []E
          |where func clone(&E)E {
          |  return MakeArray<E>(N, &{ clone(seq[_]) });
          |}
          |exported func main() int {
          |  return #[#]int(6, 4, 3, 5, 2, 8).toArray()[3];
          |}
          |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn nested_imm_arrays
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn nested_imm_arrays() {
    panic!("Unmigrated test: nested_imm_arrays");
}
/*
  test("Nested imm arrays") {
    val compile = RunCompilation.test(
      """
        |import array.make.*;
        |exported func main() int {
        |  return #[#]#[]int(
        |    #[#]int(6, 60).toImmArray(),
        |    #[#]int(4, 40).toImmArray(),
        |    #[#]int(3, 30).toImmArray()
        |  ).toImmArray()[2][1];
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(30) => }
  }
*/
// mig: fn array_foreach
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn array_foreach() {
    panic!("Unmigrated test: array_foreach");
}
/*
  test("Array foreach") {
    val compile = RunCompilation.test(
      """
        |import array.make.*;
        |import array.each.*;
        |exported func main() int {
        |  sum = 0;
        |  [#]int(6, 60, 103)&.each(&{ set sum = sum + _; });
        |  return sum;
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(169) => }
  }
*/
// mig: fn array_has
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn array_has() {
    panic!("Unmigrated test: array_has");
}
/*
  test("Array has") {
    val compile = RunCompilation.test(
        """
          |import array.has.*;
          |exported func main() bool {
          |  return [#]int(6, 60, 103)&.has(103);
          |}
          |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonBool(true) => }
  }

*/
// mig: fn each_on_ssa
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn each_on_ssa() {
    panic!("Unmigrated test: each_on_ssa");
}
/*
  test("each on SSA") {
    val compile = RunCompilation.test(
        """
          |import array.make.*;
          |import array.iter.*;
          |exported func main() {
          |  planets = [#]("Venus", "Earth", "Mars");
          |  foreach planet in planets {
          |    print(planet);
          |  }
          |}
          |""".stripMargin)
    compile.evalForStdout(Vector()) shouldEqual "VenusEarthMars"
  }
*/
// mig: fn change_mutability
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn change_mutability() {
    panic!("Unmigrated test: change_mutability");
}
/*
  test("Change mutability") {
    val compile = RunCompilation.test(
      """import array.make.*;
        |exported func main() str {
        |  a = MakeArray<str>(10, { str(_) });
        |  b = a.toImmArray();
        |  return a.3;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonStr("3") => }
  }
*/
// mig: fn reports_when_making_new_imm_rsa_without_lambda
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn reports_when_making_new_imm_rsa_without_lambda() {
    panic!("Unmigrated test: reports_when_making_new_imm_rsa_without_lambda");
}
/*
  test("Reports when making new imm rsa without lambda") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  a = #[]int(3);
        |  a.push(73);
        |  return a.0;
        |}
      """.stripMargin)

    compile.getCompilerOutputs().expectErr() match {
      case NewImmRSANeedsCallable(_) =>
    }
  }

//  test("Destroy lambda with mutable captures") {
//    val compile = RunCompilation.test(
//      Samples.get("generics/iter.vale") +
//        """
//          |exported func main() int {
//          |  list = Array<mut, int>(3, *!IFunction1<mut, int, int>({_}));
//          |  n = 7;
//          |  newArray =
//          |      Array<mut, int>(3, *!IFunction1<mut, int, int>((index) => {
//          |        = if (index == 1) {
//          |            = n;
//          |          } else {
//          |            a = list.(index);
//          |            = a * 2;
//          |          }
//          |      }));
//          |  return newArray.0;
//          |}
//          |""".stripMargin)
//    compile.evalForKind(Vector()) match { case VonInt(0) => }
//  }



//  test("Map using map()") {
//    val compile = RunCompilation.test(
//      """
//        |func map
//        |:(n: Int, T: reference, F: kind)
//        |(arr: *[n T], generator: *F) {
//        |  Array<mut>(n, (i) => { generator(arr.(i))})
//        |}
//        |exported func main() int {
//        |  board = Array<mut>(5, (x) => { x});
//        |  result = map(board, {_});
//        |  return result.3;
//        |}
//      """.stripMargin)
//
//    compile.evalForKind(Vector()) match { case VonInt(3) => }
//  }
*/
// mig: fn new_immutable_array
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn new_immutable_array() {
    panic!("Unmigrated test: new_immutable_array");
}
/*
  test("New immutable array") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  arr = Array<mut, int>(3);
        |  arr.push(13);
        |  arr.push(14);
        |  arr.push(15);
        |  immArr = toImmArray(&arr);
        |  return immArr[1];
        |}
        |
        |func toImmArray<E Ref imm>(arr &[]E) Array<imm, E> {
        |  Array<imm, E>(arr.len(), &{ arr[_] })
        |}
        |""".stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(14) => }
  }

}

*/
