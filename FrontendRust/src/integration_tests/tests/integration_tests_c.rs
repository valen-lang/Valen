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
// mig: struct IntegrationTestsC
pub struct IntegrationTestsC;
/*
class IntegrationTestsC extends FunSuite with Matchers {

*/
// mig: fn tests_floats
#[test]
fn tests_floats() {
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
        "struct Moo imm {\n  x float;\n}\nexported func main() int {\n  return 7;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 7 }) => {}
        other => panic!("expected VonInt(7), got {:?}", other),
    }
}
/*
  test("Tests floats") {
    val compile = RunCompilation.test(
      """
        |struct Moo imm {
        |  x float;
        |}
        |exported func main() int {
        |  return 7;
        |}
      """.stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(7) => }
  }

*/
// mig: fn get_or_function
#[test]
fn get_or_function() {
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
    let source = crate::tests::tests::load_expected("programs/genericvirtuals/getOr.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 9 }) => {}
        other => panic!("expected VonInt(9), got {:?}", other),
    }
}
/*
  test("getOr function") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/genericvirtuals/getOr.vale"))

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }

*/
// mig: fn panic_on_drop_because_of_outstanding_borrow
#[test]
fn panic_on_drop_because_of_outstanding_borrow() {
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
        "\nstruct Ship { hp int; }\n\nexported func main() {\n  ship = Ship(1337);\n  borrow_ship = &ship;\n  ship; // drops it\n}\n",
    );
    let _ = compile.expect_compiler_outputs();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        compile.eval_for_kind_primitive_args(Vec::new())
    }));
    let _panic_payload = result.expect_err("It should panic instead");
}
/*
  // Not sure if this is desirable behavior, because borrow_ship isnt really used after
  // we drop ship. Still, let's have this test so we don't *accidentally* change it.
  test("Panic on drop because of outstanding borrow") {
    val compile = RunCompilation.test(
      """
        |struct Ship { hp int; }
        |
        |exported func main() {
        |  ship = Ship(1337);
        |  borrow_ship = &ship;
        |  ship; // drops it
        |}
        |""".stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    try {
      compile.evalForKind(Vector())
      vfail()
    } catch {
      case ConstraintViolatedException(_) =>
    }
  }

*/
// mig: fn unlet_to_avoid_an_outstanding_borrow_panic
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn unlet_to_avoid_an_outstanding_borrow_panic() {
    panic!("Unmigrated test: unlet_to_avoid_an_outstanding_borrow_panic");
}
/*
  // Not sure if this is desirable behavior, because borrow_ship isnt really used after
  // we drop ship. Still, let's have this test so we don't *accidentally* change it.
  test("Unlet to avoid an outstanding-borrow panic") {
    val compile = RunCompilation.test(
      """
        |struct Ship { hp int; }
        |
        |exported func main() {
        |  ship = Ship(1337);
        |  borrow_ship = &ship;
        |  unlet borrow_ship;
        |  ship; // drops it
        |}
        |""".stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    compile.evalForKind(Vector())
  }

*/
// mig: fn function_return_with_return_upcasts
#[test]
fn function_return_with_return_upcasts() {
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
    let source = crate::tests::tests::load_expected("programs/virtuals/retUpcast.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let do_it = coutputs.lookup_function_by_str("doIt");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(do_it),
            crate::typing::test::traverse::NodeRefT::Upcast(_) => Some(())
        );
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 3 }) => {}
        other => panic!("Expected VonInt(3), got {:?}", other),
    }
}
/*
  test("Function return with return upcasts") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/virtuals/retUpcast.vale"))

    val coutputs = compile.expectCompilerOutputs()
    val doIt = coutputs.lookupFunction("doIt")
    Collector.only(doIt, {
      case UpcastTE(_, _, _) =>
    })

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }

*/
// mig: fn test_shaking
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn test_shaking() {
    panic!("Unmigrated test: test_shaking");
}
/*
  test("Test shaking") {
    // Make sure that functions that cant be called by main will not be included.

    val compile = RunCompilation.test(
      """import printutils.*;
        |func bork(x str) { print(x); }
        |func helperFunc(x int) { print(x); }
        |func helperFunc(x str) { print(x); }
        |exported func main() {
        |  helperFunc(4);
        |}
        |""".stripMargin)
    val hinputs = compile.getMonouts()
    val interner = compile.interner
    val keywords = compile.keywords

    vassert(
      !hinputs.functions.exists(func => func.header.id.localName match {
        case FunctionNameIX(FunctionTemplateNameI(StrI("bork"), _), _, _) => true
        case _ => false
      }))

    vassert(
      hinputs.functions.find(func => func.header.id.localName match {
        case FunctionNameIX(FunctionTemplateNameI(StrI("helperFunc"), _), _, _) => true
        case _ => false
      }).size == 1)
  }

*/
// mig: fn test_overloading_between_borrow_and_weak
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn test_overloading_between_borrow_and_weak() {
    panic!("Unmigrated test: test_overloading_between_borrow_and_weak");
}
/*
  test("Test overloading between borrow and weak") {
    val compile = RunCompilation.test(
      """
        |sealed interface IMoo  {}
        |struct Moo {}
        |impl IMoo for Moo;
        |
        |abstract func func(virtual moo &IMoo) int;
        |abstract func func(virtual moo &&IMoo) int;
        |
        |func func(moo &Moo) int { return 42; }
        |func func(moo &&Moo) int { return 73; }
        |
        |exported func main() int {
        |  return func(&Moo());
        |}
        |""".stripMargin)
    val coutputs = compile.getCompilerOutputs()

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }

*/
// mig: fn truncate_i64_to_i32
#[test]
fn truncate_i64_to_i32() {
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
        "exported func main() int {\n  return TruncateI64ToI32(4300000000i64);\n}\n",
    );
    let _coutputs = compile.expect_compiler_outputs();
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 5032704 }) => {}
        other => panic!("expected VonInt(5032704), got {:?}", other),
    }
}
/*
  test("Truncate i64 to i32") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  return TruncateI64ToI32(4300000000i64);
        |}
        |""".stripMargin)
    val coutputs = compile.expectCompilerOutputs()

    compile.evalForKind(Vector()) match { case VonInt(5032704) => }
  }

*/
// mig: fn return_without_return
#[test]
fn return_without_return() {
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
        "exported func main() int { 73 }\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 73 }) => {}
        other => panic!("expected VonInt(73), got {:?}", other),
    }
}
/*
  test("Return without return") {
    val compile = RunCompilation.test(
      """
        |exported func main() int { 73 }
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(73) => }
  }

*/
// mig: fn test_export_functions
#[test]
fn test_export_functions() {
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
        "exported func moo() int {\n  return 42;\n}\n",
    );
    let _hamuts = compile.get_hamuts();
}
/*
  test("Test export functions") {
    val compile = RunCompilation.test(
      """exported func moo() int {
        |  return 42;
        |}
        |""".stripMargin)
    val hamuts = compile.getHamuts()
  }

*/
// mig: fn test_extern_functions
#[test]
fn test_extern_functions() {
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
    let source = crate::tests::tests::load_expected("programs/externs/extern.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    {
        let math_str = scout_arena.intern_str("math");
        let math_package_coord = *scout_arena.intern_package_coordinate(math_str, &[]);
        let hamuts = compile.get_hamuts();
        let package_h = hamuts.lookup_package(math_package_coord);
        let sqrt_str = scout_arena.intern_str("sqrt");
        let sqrt_extern = package_h.prototype_to_extern.values().find(|fe| fe.maybe_extern_name == sqrt_str);
        assert!(sqrt_extern.is_some());
        match sqrt_extern.unwrap() {
            crate::final_ast::types::HamutsFunctionExtern { maybe_extern_name, prototype, .. } if *maybe_extern_name == sqrt_str => {
                assert_eq!(prototype.id.local_name, sqrt_str);
            }
            other => panic!("expected HamutsFunctionExtern(sqrt, PrototypeH(IdH(sqrt, ...), ...), _), got {:?}", other),
        }
        let extern_sqrt = package_h.lookup_function("sqrt(float)");
        assert!(extern_sqrt.is_extern);
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 4 }) => {}
        other => panic!("expected VonInt(4), got {:?}", other),
    }
}
/*
  test("Test extern functions") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/externs/extern.vale"))
    val interner = compile.interner
    val keywords = compile.keywords

    val packageH = compile.getHamuts().lookupPackage(interner.intern(PackageCoordinate(interner.intern(StrI("math")), Vector.empty)))

    // The extern we make should have the name we expect
    val sqrtExtern = packageH.prototypeToExtern.values.find(_.maybeExternName == "sqrt")
    vassert(sqrtExtern.nonEmpty)
    sqrtExtern.get match {
      case HamutsFunctionExtern("sqrt", PrototypeH(IdH("sqrt", _, _, _), _, _), _) =>
    }

    // We also made an internal function that contains an extern call
    val externSqrt = packageH.lookupFunction("sqrt(float)")
    vassert(externSqrt.isExtern)

    compile.evalForKind(Vector()) match { case VonInt(4) => }
  }

*/
// mig: fn test_narrowing_between_borrow_and_owning_overloads
#[test]
fn test_narrowing_between_borrow_and_owning_overloads() {
    // See NMORFI for why this test is here. Before the SCCTT fix, it couldn't resolve between the two
    // `get` overloads, because the borrow ownership (from the opt.get()) was creeping into the rules
    // too far.
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
        "\nimport panicutils.*;\n\nsealed interface XOpt<T> where T Ref { }\nstruct XNone<T> where T Ref { }\nimpl<T> XOpt<T> for XNone<T>;\n\nabstract func get<T>(virtual opt XOpt<T>) int;\nfunc get<T>(opt XNone<T>) int { __vbi_panic() }\n\nabstract func get<T>(virtual opt &XOpt<T>) int;\nfunc get<T>(opt &XNone<T>) int { return 42; }\n\nexported func main() int {\n  opt XOpt<int> = XNone<int>();\n  return opt.get();\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Test narrowing between borrow and owning overloads") {
    // See NMORFI for why this test is here. Before the SCCTT fix, it couldn't resolve between the two
    // `get` overloads, because the borrow ownership (from the opt.get()) was creeping into the rules
    // too far.

    val compile = RunCompilation.test(
      """
        |import panicutils.*;
        |
        |sealed interface XOpt<T> where T Ref { }
        |struct XNone<T> where T Ref { }
        |impl<T> XOpt<T> for XNone<T>;
        |
        |abstract func get<T>(virtual opt XOpt<T>) int;
        |func get<T>(opt XNone<T>) int { __vbi_panic() }
        |
        |abstract func get<T>(virtual opt &XOpt<T>) int;
        |func get<T>(opt &XNone<T>) int { return 42; }
        |
        |exported func main() int {
        |  opt XOpt<int> = XNone<int>();
        |  return opt.get();
        |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }

*/
// mig: fn test_catch_deref_after_drop
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn test_catch_deref_after_drop() {
    panic!("Unmigrated test: test_catch_deref_after_drop");
}
/*
  test("Test catch deref after drop") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/invalidaccess.vale"))
    try {
      compile.evalForKind(Vector()) match { case VonInt(42) => }
      vfail()
    } catch {
      case ConstraintViolatedException(_) => // good!
    }
  }

*/
// This test is here because we had a bug where the compiler was enforcing that we unstackify
// the same locals from all branches of if, even if they were constraint refs.
// mig: fn using_same_constraint_ref_from_both_branches_of_if
#[test]
fn using_same_constraint_ref_from_both_branches_of_if() {
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
        "\nstruct Moo {}\nfunc foo(a &Moo) int { return 41; }\nfunc bork(a &Moo) int {\n  if (false) {\n    return foo(a);\n  } else if (false) {\n    return foo(a);\n  } else {\n    // continue\n  }\n  return foo(a) + 1;\n}\nexported func main() int {\n  return bork(&Moo());\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  // This test is here because we had a bug where the compiler was enforcing that we unstackify
  // the same locals from all branches of if, even if they were constraint refs.
  test("Using same constraint ref from both branches of if") {
    val compile = RunCompilation.test(
      """
        |struct Moo {}
        |func foo(a &Moo) int { return 41; }
        |func bork(a &Moo) int {
        |  if (false) {
        |    return foo(a);
        |  } else if (false) {
        |    return foo(a);
        |  } else {
        |    // continue
        |  }
        |  return foo(a) + 1;
        |}
        |exported func main() int {
        |  return bork(&Moo());
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }


*/
// Compiler should be fine with moving things from if statements if we return out.
// mig: fn moving_same_thing_from_both_branches_of_if
#[test]
fn moving_same_thing_from_both_branches_of_if() {
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
        "\nstruct Moo {}\nfunc foo(a Moo) int { return 41; }\nfunc bork(a Moo) int {\n  if (false) {\n    return foo(a);\n  } else if (false) {\n    return foo(a);\n  } else {\n    // continue\n  }\n  return 42;\n}\nexported func main() int {\n  return bork(Moo());\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  // Compiler should be fine with moving things from if statements if we return out.
  test("Moving same thing from both branches of if") {
    val compile = RunCompilation.test(
      """
        |struct Moo {}
        |func foo(a Moo) int { return 41; }
        |func bork(a Moo) int {
        |  if (false) {
        |    return foo(a);
        |  } else if (false) {
        |    return foo(a);
        |  } else {
        |    // continue
        |  }
        |  return 42;
        |}
        |exported func main() int {
        |  return bork(Moo());
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }

*/
// mig: fn exporting_array
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn exporting_array() {
    panic!("Unmigrated test: exporting_array");
}
/*
  test("exporting array") {
    val compilation = RunCompilation.test("export []<mut>int as IntArray;")
    val hamuts = compilation.getHamuts()
    val testPackage = hamuts.lookupPackage(PackageCoordinate.TEST_TLD(compilation.interner, compilation.keywords))
    val kindH = vassertSome(testPackage.exportNameToKind.get(compilation.interner.intern(StrI("IntArray"))))

    val builtinPackage = hamuts.lookupPackage(PackageCoordinate.BUILTIN(compilation.interner, compilation.keywords))
    val rsa = vassertSome(builtinPackage.runtimeSizedArrays.find(_.kind == kindH))
    rsa.elementType.kind shouldEqual IntHT.i32
  }

*/
// mig: fn call_borrow_parameter_with_shared_reference
#[test]
fn call_borrow_parameter_with_shared_reference() {
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
        "\nfunc bork<T>(a &T) &T { return a; }\n\nexported func main() int {\n  return bork(6);\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 6 }) => {}
        other => panic!("expected VonInt(6), got {:?}", other),
    }
}
/*
  test("Call borrow parameter with shared reference") {
    val compile = RunCompilation.test(
      """
        |func bork<T>(a &T) &T { return a; }
        |
        |exported func main() int {
        |  return bork(6);
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(6) => }
  }

*/
// mig: fn supplying_bounded_struct_to_struct_accepting
#[test]
fn supplying_bounded_struct_to_struct_accepting() {
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
        "\nstruct Bork<T> where func drop(T)void { a T; }\n\nstruct Spork<T> where func drop(T)void { a T; }\n\nexported func main() int {\n  return Spork<Bork<int>>(Bork(7)).a.a;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 7 }) => {}
        other => panic!("expected VonInt(7), got {:?}", other),
    }
}
/*
  test("Supplying bounded struct to struct accepting") {
    // See OIRCRR.
    //
    // We have a bug where the Spork<...>(...) call site sees that it's calling:
    //   func Spork<Bork<int>>(Bork)Spork<Bork<int>>
    // and it sees that it's supplying a Bork, and so it supplies Bork's bounds as part of the instantiation reachable
    // functions.
    //
    // However, the definition of Spork constructor is:
    //   func Spork<T>(a T) Spork<T> { construct<Spork<T>>(a) }
    // and sees that the parameter is a T, and doesn't expect any particular reachable functions.
    //
    // The call site thinks "I'm giving an argument that's a citizen with bounds. I should include it in the
    // instantiation args.".
    // What it should really think is "I should only give the arguments that the receiver expects."
    // And the receiver only expects reachable functions for params that it *knows* are citizens. Params that the
    // *definition* calls.
    //
    // We can know that by looking at the CallSR rules of the function we're calling, and only include reachables for
    // what comes out of those.

    val compile = RunCompilation.test(
      """
        |struct Bork<T> where func drop(T)void { a T; }
        |// makes implicit constructor:
        |// func Bork<T>(a T) Bork<T> { construct<Bork<T>>(a) }
        |
        |struct Spork<T> where func drop(T)void { a T; }
        |// makes implicit constructor:
        |// func Spork<T>(a T) Spork<T> { construct<Spork<T>>(a) }
        |
        |exported func main() int {
        |  return Spork<Bork<int>>(Bork(7)).a.a;
        |}
    """.stripMargin)

    compile.evalForKind(Vector()) match {
      case VonInt(7) =>
    }
  }

*/
// mig: fn same_type_multiple_times_in_an_invocation
#[test]
fn same_type_multiple_times_in_an_invocation() {
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
        "\nstruct Bork<T> where func drop(T)void {\n  a T;\n}\n\nexported func main() int {\n  return Bork<Bork<Bork<int>>>(Bork(Bork(7))).a.a.a;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 7 }) => {}
        other => panic!("expected VonInt(7), got {:?}", other),
    }
}
/*
  test("Same type multiple times in an invocation") {
    val compile = RunCompilation.test(
      """
        |struct Bork<T> where func drop(T)void {
        |  a T;
        |}
        |
        |exported func main() int {
        |  return Bork<Bork<Bork<int>>>(Bork(Bork(7))).a.a.a;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(7) => }
  }

*/
// mig: fn restackify
#[test]
fn restackify() {
    // Allow set on variables that have been moved already, which is useful for linear style.
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
    let source = crate::tests::tests::load_expected("programs/restackify.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Restackify") {
    // Allow set on variables that have been moved already, which is useful for linear style.
    val compile = RunCompilation.test(Tests.loadExpected("programs/restackify.vale"))
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }

*/
// mig: fn destructure_restackify
#[test]
fn destructure_restackify() {
    // Allow set on variables that have been moved already, which is useful for linear style.
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
    let source = crate::tests::tests::load_expected("programs/destructure_restackify.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Destructure restackify") {
    // Allow set on variables that have been moved already, which is useful for linear style.
    val compile = RunCompilation.test(Tests.loadExpected("programs/destructure_restackify.vale"))
    compile.evalForKind(Vector()) match {
      case VonInt(42) =>
    }
  }

*/
// mig: fn loop_restackify
#[test]
fn loop_restackify() {
    // Allow set on variables that have been moved already, which is useful for linear style.
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
    let source = crate::tests::tests::load_expected("programs/loop_restackify.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Loop restackify") {
    // Allow set on variables that have been moved already, which is useful for linear style.
    val compile = RunCompilation.test(Tests.loadExpected("programs/loop_restackify.vale"))
    compile.evalForKind(Vector()) match {
      case VonInt(42) =>
    }
  }

*/
// mig: fn ignoring_receiver
#[test]
fn ignoring_receiver() {
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
        "\nstruct Marine { hp int; }\nexported func main() int { [_, y] = (Marine(6), Marine(8)); return y.hp; }\n\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        assert_eq!(main.header.return_type, crate::typing::types::types::CoordT {
            ownership: crate::typing::types::types::OwnershipT::Share,
            region: crate::typing::types::types::RegionT { region: crate::typing::types::types::IRegionT::Default },
            kind: crate::typing::types::types::KindT::Int(crate::typing::types::types::IntT::I32),
        });
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 8 }) => {}
        other => panic!("expected VonInt(8), got {:?}", other),
    }
}
/*
  test("Ignoring receiver") {
    val compile = RunCompilation.test(
      """
        |struct Marine { hp int; }
        |exported func main() int { [_, y] = (Marine(6), Marine(8)); return y.hp; }
        |
      """.stripMargin)
    val coutputs = compile.expectCompilerOutputs()
    val main = coutputs.lookupFunction("main");
    main.header.returnType shouldEqual CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32)
    compile.evalForKind(Vector()) match {
      case VonInt(8) =>
    }
  }
}

*/
