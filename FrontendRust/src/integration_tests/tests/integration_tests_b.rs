/*
package dev.vale

import dev.vale.finalast._
import dev.vale.instantiating.ast._
import dev.vale.testvm.{ConstraintViolatedException, Heap, IntV, StructInstanceV}
import dev.vale.typing.ast._
import dev.vale.typing.types._
import dev.vale.von.{VonBool, VonFloat, VonInt}
import org.scalatest._
*/
// mig: struct IntegrationTestsB
pub struct IntegrationTestsB;
/*
class IntegrationTestsB extends FunSuite with Matchers {
*/
// mig: fn tests_single_expression_and_single_statement_functions_returns
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_single_expression_and_single_statement_functions_returns() {
    panic!("Unmigrated test: tests_single_expression_and_single_statement_functions_returns");
}
/*
  test("Tests single expression and single statement functions' returns") {
    val compile = RunCompilation.test(
      """
        |struct MyThing { value int; }
        |func moo() MyThing { return MyThing(4); }
        |exported func main() { moo(); }
      """.stripMargin)
    compile.run(Vector())
  }
*/
// mig: fn tests_calling_a_templated_struct_constructor
#[test]
fn tests_calling_a_templated_struct_constructor() {
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
        "\n#!DeriveStructDrop\nstruct MySome<T Ref> { value T; }\n\nexported func main() int {\n  [x] = MySome<int>(4);\n  return x;\n}\n",
    );
    let _ = compile.eval_for_kind_primitive_args(Vec::new());
}
/*
  test("Tests calling a templated struct's constructor") {
    val compile = RunCompilation.test(
      """
        |#!DeriveStructDrop
        |struct MySome<T Ref> { value T; }
        |
        |exported func main() int {
        |  [x] = MySome<int>(4);
        |  return x;
        |}
      """.stripMargin)
    compile.evalForKind(Vector())
  }
*/
// mig: fn test_array_push_pop_len_capacity_drop
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn test_array_push_pop_len_capacity_drop() {
    panic!("Unmigrated test: test_array_push_pop_len_capacity_drop");
}
/*
  test("Test array push, pop, len, capacity, drop") {
    val compile = RunCompilation.test(
      """
        |import castutils.*;
        |import printutils.*;
        |import array.make.*;
        |
        |exported func main() int {
        |  arr = Array<mut, int>(9);
        |  arr.push(420);
        |  arr.push(421);
        |  arr.push(422);
        |  arr.len();
        |  return arr.capacity();
        |  // implicit drop with pops
        |}
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn test_int_generic
#[test]
fn test_int_generic() {
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
        "\nstruct Vec<N Int, T>\n{\n  values [#N]<imm>T;\n}\n\nexported func main() int {\n  v = Vec<3, int>(#[#](3, 4, 5));\n  return v.values.2;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 5 }) => {}
        other => panic!("expected VonInt(5), got {:?}", other),
    }
}
/*
  test("Test int generic") {
    val compile = RunCompilation.test(
      """
        |
        |struct Vec<N Int, T>
        |{
        |  values [#N]<imm>T;
        |}
        |
        |exported func main() int {
        |  v = Vec<3, int>(#[#](3, 4, 5));
        |  return v.values.2;
        |}
      """.stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn tests_upcasting_from_a_struct_to_an_interface
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_upcasting_from_a_struct_to_an_interface() {
    panic!("Unmigrated test: tests_upcasting_from_a_struct_to_an_interface");
}
/*
  test("Tests upcasting from a struct to an interface") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/virtuals/upcasting.vale"))
    compile.run(Vector())
  }
*/
// mig: fn tests_upcasting_from_if
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_upcasting_from_if() {
    panic!("Unmigrated test: tests_upcasting_from_if");
}
/*
  test("Tests upcasting from if") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/if/upcastif.vale"))
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn tests_lambda
#[test]
fn tests_lambda() {
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
        "\nexported func main() int {\n  a = 7;\n  return { a }();\n}\n",
    );
    compile.run_primitive_args(Vec::new());
}
/*
  test("Tests lambda") {
    val compile =
      RunCompilation.test(
        """
          |exported func main() int {
          |  a = 7;
          |  return { a }();
          |}
          |""".stripMargin)
    compile.run(Vector())
  }
*/
// mig: fn tests_generic_with_a_lambda
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_generic_with_a_lambda() {
    panic!("Unmigrated test: tests_generic_with_a_lambda");
}
/*
  test("Tests generic with a lambda") {
    val compile =
      RunCompilation.test(
        """
          |func genFunc<T>(a &T) &T {
          |  return { a }();
          |}
          |exported func main() int {
          |  genFunc(7)
          |}
          |""".stripMargin)
    compile.run(Vector())
  }
*/
// mig: fn tests_generic_s_lambda_calling_parent_function_s_bound
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_generic_s_lambda_calling_parent_function_s_bound() {
    panic!("Unmigrated test: tests_generic_s_lambda_calling_parent_function_s_bound");
}
/*
  test("Tests generic's lambda calling parent function's bound") {
    // See LCCPGB for explanation.
    val compile =
      RunCompilation.test(
        """
          |func genFunc<T>(a &T)
          |where func print(&T)void {
          |  { print(a); }()
          |}
          |exported func main() {
          |  genFunc("hello");
          |}
          |""".stripMargin)
    compile.run(Vector())
  }
*/
// mig: fn tests_generic_with_a_polymorphic_lambda
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_generic_with_a_polymorphic_lambda() {
    panic!("Unmigrated test: tests_generic_with_a_polymorphic_lambda");
}
/*
  test("Tests generic with a polymorphic lambda") {
    // This lambda has an implicit <Y> template param
    val compile =
      RunCompilation.test(
        """
          |func genFunc<T>(a &T) &T {
          |  return (x => a)(true);
          |}
          |exported func main() int {
          |  genFunc(7)
          |}
          |""".stripMargin)
    compile.run(Vector())
  }
*/
// mig: fn tests_generic_with_a_polymorphic_lambda_invoked_twice
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_generic_with_a_polymorphic_lambda_invoked_twice() {
    panic!("Unmigrated test: tests_generic_with_a_polymorphic_lambda_invoked_twice");
}
/*
  test("Tests generic with a polymorphic lambda invoked twice") {
    // This lambda has an implicit <Y> template param, invoked with a bool then a string
    val compile =
      RunCompilation.test(
        """
          |func genFunc<T>(a &T) &T {
          |  lam = (x => a);
          |  lam(true);
          |  return lam("hello");
          |}
          |exported func main() int {
          |  genFunc(7)
          |}
          |""".stripMargin)
    compile.run(Vector())
  }

//  test("Test getting generic value out of lambda") {
//    val compile = RunCompilation.test(
//      """
//        |#!DeriveStructDrop
//        |struct MyStruct<A Ref imm, B Ref imm, C Ref imm, D Ref imm> imm { a A; b B; c C; d D; }
//        |
//        |func bork<A, B, C, D>(m &MyStruct<A, B, C, D>) &D {
//        |  return { m.d }();
//        |}
//        |exported func main() int {
//        |  x = MyStruct(true, 1, "hello", 3);
//        |  return bork(&x);
//        |}
//        |""".stripMargin)
//    compile.evalForKind(Vector()) match { case VonInt(5) => }
//  }
*/
// mig: fn tests_double_closure
#[test]
fn tests_double_closure() {
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
    let source = crate::tests::tests::load_expected("programs/lambdas/doubleclosure.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    compile.run_primitive_args(Vec::new());
}
/*
  test("Tests double closure") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/lambdas/doubleclosure.vale"))
    compile.run(Vector())
  }
*/
// mig: fn tests_from_subdir_file
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_from_subdir_file() {
    panic!("Unmigrated test: tests_from_subdir_file");
}
/*
  test("Tests from subdir file") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/virtuals/round.vale"))
    compile.evalForKind(Vector()) match { case VonInt(8) => }
  }
*/
// mig: fn test_generic_param_default
#[test]
fn test_generic_param_default() {
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
        "\nfunc bork<N Int = 42>() int { return N; }\nexported func main() int { bork() }\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Test generic param default") {
    val compile = RunCompilation.test(
      """
        |func bork<N Int = 42>() int { return N; }
        |exported func main() int { bork() }
      """.stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn tests_calling_a_virtual_function
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_calling_a_virtual_function() {
    panic!("Unmigrated test: tests_calling_a_virtual_function");
}
/*
  test("Tests calling a virtual function") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/virtuals/calling.vale"))
    compile.evalForKind(Vector()) match { case VonInt(7) => }
  }
*/
// mig: fn tests_making_a_variable_with_a_pattern
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_making_a_variable_with_a_pattern() {
    panic!("Unmigrated test: tests_making_a_variable_with_a_pattern");
}
/*
  test("Tests making a variable with a pattern") {
    // Tests putting MyOption<int> as the type of x.
    val compile = RunCompilation.test(
      """
        |interface MyOption<T> where T Ref { }
        |
        |struct MySome<T> where T Ref {}
        |impl<T> MyOption<T> for MySome<T>;
        |
        |func doSomething(opt MyOption<int>) int {
        |  return 9;
        |}
        |
        |exported func main() int {
        |	 x MyOption<int> = MySome<int>();
        |	 return doSomething(x);
        |}
      """.stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn tests_a_linked_list
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_a_linked_list() {
    panic!("Unmigrated test: tests_a_linked_list");
}
/*

  test("Tests a linked list") {
    val compile = RunCompilation.test(
      Tests.loadExpected("programs/virtuals/ordinarylinkedlist.vale"))
    compile.evalForKind(Vector())
  }
*/
// mig: fn tests_a_templated_linked_list
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_a_templated_linked_list() {
    panic!("Unmigrated test: tests_a_templated_linked_list");
}
/*
  test("Tests a templated linked list") {
    val compile = RunCompilation.test(
        Tests.loadExpected("programs/genericvirtuals/templatedlinkedlist.vale"))
    compile.evalForKind(Vector())
  }
*/
// mig: fn tests_calling_an_abstract_function
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_calling_an_abstract_function() {
    panic!("Unmigrated test: tests_calling_an_abstract_function");
}
/*
  test("Tests calling an abstract function") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/genericvirtuals/callingAbstract.vale"))
    compile.evalForKind(Vector()) match { case VonInt(4) => }
  }
*/
// mig: fn template_overrides_are_stamped
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn template_overrides_are_stamped() {
    panic!("Unmigrated test: template_overrides_are_stamped");
}
/*
  test("Template overrides are stamped") {
    // See TIBANFC: Translate Impl Bound Argument Names For Case
    val compile = RunCompilation.test(
        Tests.loadExpected("programs/genericvirtuals/templatedoption.vale"), false)
    compile.evalForKind(Vector()) match { case VonInt(1) => }
  }
*/
// mig: fn tests_a_foreach_for_a_linked_list
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_a_foreach_for_a_linked_list() {
    panic!("Unmigrated test: tests_a_foreach_for_a_linked_list");
}
/*
  test("Tests a foreach for a linked list") {
    val compile = RunCompilation.test(
        Tests.loadExpected("programs/genericvirtuals/foreachlinkedlist.vale"))
    compile.evalForStdout(Vector()) shouldEqual "102030"
  }

  // When we call a function with a virtual parameter, try stamping for all ancestors in its
  // place.
  // We're stamping all ancestors, and all ancestors have virtual.
  // Virtual starts a function family.
  // So, this checks that it and its three ancestors are all stamped and all get their own
  // function families.
//  test("Stamp multiple ancestors") {
//    val compile = RunCompilation.test(Tests.loadExpected("programs/genericvirtuals/stampMultipleAncestors.vale"))
//    val coutputs = compile.expectCompilerOutputs()
//    compile.evalForKind(Vector())
//  }
*/
// mig: fn tests_recursion
#[test]
fn tests_recursion() {
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
    let source = crate::tests::tests::load_expected("programs/functions/recursion.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 120 }) => {}
        other => panic!("expected VonInt(120), got {:?}", other),
    }
}
/*
  test("Tests recursion") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/functions/recursion.vale"))
    compile.evalForKind(Vector()) match { case VonInt(120) => }
  }
*/
// mig: fn tests_generic_recursion
#[test]
fn tests_generic_recursion() {
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
        "\nfunc factorial<T>(one T, x T) T\nwhere func isZero(&T)bool, func *(&T, &T)T, func -(&T, &T)T, func drop(T)void {\n  return if isZero(&x) {\n      one\n    } else {\n      q = &one;\n      x * factorial(one, x - q)\n    };\n}\n\nfunc isZero(x int) bool { x == 0 }\n\nexported func main() int {\n  return factorial(1, 5);\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 120 }) => {}
        other => panic!("expected VonInt(120), got {:?}", other),
    }
}
/*
  test("Tests generic recursion") {
    val compile = RunCompilation.test(
      """
        |func factorial<T>(one T, x T) T
        |where func isZero(&T)bool, func *(&T, &T)T, func -(&T, &T)T, func drop(T)void {
        |  return if isZero(&x) {
        |      one
        |    } else {
        |      q = &one;
        |      x * factorial(one, x - q)
        |    };
        |}
        |
        |func isZero(x int) bool { x == 0 }
        |
        |exported func main() int {
        |  return factorial(1, 5);
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(120) => }
  }
}

*/
