/*
package dev.vale

import dev.vale.highertyping.{ICompileErrorA, ProgramA}
import dev.vale.passmanager.{FullCompilation, FullCompilationOptions}
import dev.vale.finalast.{IdH, IntHT, OwnH, ProgramH, PrototypeH, YonderH}
import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.FileP
import dev.vale.postparsing._
import dev.vale.typing.ast._
import dev.vale.instantiating.ast._
import dev.vale.typing.names._
import dev.vale.typing.{HinputsT, ICompileErrorT, ast}
import dev.vale.typing.types._
import dev.vale.testvm.{ConstraintViolatedException, Heap, IntV, PrimitiveKindV, ReferenceV, StructInstanceV, Vivem}
import dev.vale.highertyping.ICompileErrorA

import java.io.FileNotFoundException
import dev.vale.typing.ast
import dev.vale.{finalast => m}
import dev.vale.testvm.ReferenceV
import org.scalatest._
import dev.vale.passmanager.FullCompilation
import dev.vale.finalast.IdH
import dev.vale.instantiating.ast.HinputsI
import dev.vale.lexing.{FailedParse, RangeL}
import dev.vale.postparsing.ICompileErrorS
import dev.vale.typing.ast._
import dev.vale.typing.types.StrT
import dev.vale.von.{IVonData, VonBool, VonFloat, VonInt}

import scala.collection.immutable.List

*/
// mig: struct IntegrationTestsA
pub struct IntegrationTestsA;
/*
class IntegrationTestsA extends FunSuite with Matchers {
*/
// mig: fn roguelike_typing_pass
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn roguelike_typing_pass() { panic!("Unmigrated test: roguelike_typing_pass"); }
/*
  test("Roguelike typing pass") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/roguelike.vale"), true)
    compile.getCompilerOutputs() match {
      case Ok(_) =>
      case Err(e) => { println("DIAG-RAW-ERR: " + e); throw new RuntimeException("compile failed") }
    }
  }

  //  test("Scratch scratch") {
  //    val compile =
  //      RunCompilation.test(
  //        """
  //          |scratch code here
  //          |""".stripMargin)
  //    compile.evalForKind(Vector())
  //  }
*/
// mig: fn simple_program_returning_an_int
#[test]
fn simple_program_returning_an_int() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &typing_bump, &instantiating_bump,
        "exported func main() int { return 3; }", false,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 3 }) => {}
        other => panic!("expected VonInt(3), got {:?}", other),
    }
}
/*
  test("Simple program returning an int") {
    val compile = RunCompilation.test("exported func main() int { return 3; }", false)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_program_with_drop
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_program_with_drop() { panic!("Unmigrated test: simple_program_with_drop"); }
/*
  test("Simple program with drop") {
    val compile = RunCompilation.test("import v.builtins.drop.*; exported func main() int { return 3; }", false)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_program_with_arith
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_program_with_arith() { panic!("Unmigrated test: simple_program_with_arith"); }
/*
  test("Simple program with arith") {
    val compile = RunCompilation.test("import v.builtins.arith.*; exported func main() int { return 3; }", false)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_program_with_logic
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_program_with_logic() { panic!("Unmigrated test: simple_program_with_logic"); }
/*
  test("Simple program with logic") {
    val compile = RunCompilation.test("import v.builtins.logic.*; exported func main() int { return 3; }", false)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_program_with_migrate
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_program_with_migrate() { panic!("Unmigrated test: simple_program_with_migrate"); }
/*
  test("Simple program with migrate") {
    val compile = RunCompilation.test("import v.builtins.migrate.*; exported func main() int { return 3; }", false)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_program_with_str
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_program_with_str() { panic!("Unmigrated test: simple_program_with_str"); }
/*
  test("Simple program with str") {
    val compile = RunCompilation.test("import v.builtins.str.*; exported func main() int { return 3; }", false)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_program_with_arrays
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_program_with_arrays() { panic!("Unmigrated test: simple_program_with_arrays"); }
/*
  test("Simple program with arrays") {
    val compile = RunCompilation.test("import v.builtins.arrays.*; exported func main() int { return 3; }", false)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_program_with_mainargs
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_program_with_mainargs() { panic!("Unmigrated test: simple_program_with_mainargs"); }
/*
  test("Simple program with mainargs") {
    val compile = RunCompilation.test("import v.builtins.mainargs.*; exported func main() int { return 3; }", false)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_program_with_as
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_program_with_as() { panic!("Unmigrated test: simple_program_with_as"); }
/*
  test("Simple program with as") {
    val compile = RunCompilation.test("import v.builtins.as.*; exported func main() int { return 3; }", false)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_program_with_print
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_program_with_print() { panic!("Unmigrated test: simple_program_with_print"); }
/*
  test("Simple program with print") {
    val compile = RunCompilation.test("import v.builtins.print.*; exported func main() int { return 3; }", false)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_program_with_tup
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_program_with_tup() { panic!("Unmigrated test: simple_program_with_tup"); }
/*
  test("Simple program with tup") {
    val compile = RunCompilation.test("import v.builtins.tup2.*; exported func main() int { return 3; }", false)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_program_with_panic
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_program_with_panic() { panic!("Unmigrated test: simple_program_with_panic"); }
/*
  test("Simple program with panic") {
    val compile = RunCompilation.test("import v.builtins.panic.*; exported func main() int { return 3; }", false)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_program_with_opt
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_program_with_opt() { panic!("Unmigrated test: simple_program_with_opt"); }
/*
  test("Simple program with opt") {
    val compile = RunCompilation.test("import v.builtins.opt.*; exported func main() int { return 3; }", false)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_program_with_result
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_program_with_result() { panic!("Unmigrated test: simple_program_with_result"); }
/*
  test("Simple program with result") {
    val compile = RunCompilation.test("import v.builtins.result.*; exported func main() int { return 3; }", false)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_program_with_sameinstance
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_program_with_sameinstance() { panic!("Unmigrated test: simple_program_with_sameinstance"); }
/*
  test("Simple program with sameinstance") {
    val compile = RunCompilation.test("import v.builtins.sameinstance.*; exported func main() int { return 3; }", false)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_program_with_weak
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_program_with_weak() { panic!("Unmigrated test: simple_program_with_weak"); }
/*
  test("Simple program with weak") {
    val compile = RunCompilation.test("import v.builtins.weak.*; exported func main() int { return 3; }", false)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn hardcoding_negative_numbers
#[test]
fn hardcoding_negative_numbers() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &typing_bump, &instantiating_bump,
        "exported func main() int { return -3; }", true,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: -3 }) => {}
        other => panic!("expected VonInt(-3), got {:?}", other),
    }
}
/*
  test("Hardcoding negative numbers") {
    val compile = RunCompilation.test("exported func main() int { return -3; }")
    compile.evalForKind(Vector()) match { case VonInt(-3) => }
  }
*/
// mig: fn taking_an_argument_and_returning_it
#[test]
fn taking_an_argument_and_returning_it() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &typing_bump, &instantiating_bump,
        "exported func main(a int) int { return a; }", true,
    );
    match compile.eval_for_kind_primitive_args(vec![
        crate::testvm::values::PrimitiveKindV::Int(crate::testvm::values::IntV {
            value: 5, bits: 32, _phantom: std::marker::PhantomData,
        }),
    ]) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 5 }) => {}
        other => panic!("expected VonInt(5), got {:?}", other),
    }
}
/*
  test("Taking an argument and returning it") {
    val compile = RunCompilation.test("exported func main(a int) int { return a; }")
    compile.evalForKind(Vector(IntV(5, 32))) match { case VonInt(5) => }
  }
*/
// mig: fn tests_adding_two_numbers
#[test]
fn tests_adding_two_numbers() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &typing_bump, &instantiating_bump,
        "exported func main() int { return +(2, 3); }", true,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 5 }) => {}
        other => panic!("expected VonInt(5), got {:?}", other),
    }
}
/*
  test("Tests adding two numbers") {
    val compile = RunCompilation.test("exported func main() int { return +(2, 3); }")
    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn tests_adding_two_floats
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_adding_two_floats() { panic!("Unmigrated test: tests_adding_two_floats"); }
/*
  test("Tests adding two floats") {
    val compile = RunCompilation.test("exported func main() float { return +(2.5, 3.5); }")
    compile.evalForKind(Vector()) match { case VonFloat(6.0f) => }
  }
*/
// mig: fn tests_inline_adding
#[test]
fn tests_inline_adding() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &typing_bump, &instantiating_bump,
        "exported func main() int { return 2 + 3; }", true,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 5 }) => {}
        other => panic!("expected VonInt(5), got {:?}", other),
    }
}
/*
  test("Tests inline adding") {
    val compile = RunCompilation.test("exported func main() int { return 2 + 3; }")
    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn test_constraint_ref
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn test_constraint_ref() { panic!("Unmigrated test: test_constraint_ref"); }
/*
  test("Test constraint ref") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/constraintRef.vale"))
    compile.evalForKind(Vector()) match { case VonInt(8) => }
  }
*/
// mig: fn test_borrow_ref
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn test_borrow_ref() { panic!("Unmigrated test: test_borrow_ref"); }
/*
  test("Test borrow ref") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/borrowRef.vale"))
    compile.evalForKind(Vector()) match { case VonInt(8) => }
  }
*/
// mig: fn tests_inline_adding_more
#[test]
fn tests_inline_adding_more() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &typing_bump, &instantiating_bump,
        "exported func main() int { return 2 + 3 + 4 + 5 + 6; }", true,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 20 }) => {}
        other => panic!("expected VonInt(20), got {:?}", other),
    }
}
/*
  test("Tests inline adding more") {
    val compile = RunCompilation.test("exported func main() int { return 2 + 3 + 4 + 5 + 6; }")
    compile.evalForKind(Vector()) match { case VonInt(20) => }
  }
*/
// mig: fn simple_lambda
#[test]
fn simple_lambda() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &typing_bump, &instantiating_bump,
        "exported func main() int { return {7}(); }", true,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 7 }) => {}
        other => panic!("expected VonInt(7), got {:?}", other),
    }
}
/*
  test("Simple lambda") {
    val compile = RunCompilation.test("exported func main() int { return {7}(); }")
    compile.evalForKind(Vector()) match { case VonInt(7) => }
  }
*/
// mig: fn lambda_with_one_magic_arg
#[test]
fn lambda_with_one_magic_arg() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &typing_bump, &instantiating_bump,
        "exported func main() int { return {_}(3); }", true,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 3 }) => {}
        other => panic!("expected VonInt(3), got {:?}", other),
    }
}
/*
  test("Lambda with one magic arg") {
    val compile = RunCompilation.test("exported func main() int { return {_}(3); }")
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }

*/
// mig: fn lambda_with_a_type_specified_param
#[test]
fn lambda_with_a_type_specified_param() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &typing_bump, &instantiating_bump,
        "exported func main() int { return (a int) => { return +(a,a); }(3); }", true,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 6 }) => {}
        other => panic!("expected VonInt(6), got {:?}", other),
    }
}
/*
  // Test that the lambda's arg is the right type, and the name is right
  test("Lambda with a type specified param") {
    val compile = RunCompilation.test("exported func main() int { return (a int) => { return +(a,a); }(3); }");
    compile.evalForKind(Vector()) match { case VonInt(6) => }
  }
*/
// mig: fn test_overloads
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn test_overloads() { panic!("Unmigrated test: test_overloads"); }
/*
  test("Test overloads") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/functions/overloads.vale"))
    compile.evalForKind(Vector()) match { case VonInt(6) => }
  }

*/
// mig: fn test_block
#[test]
fn test_block() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &typing_bump, &instantiating_bump,
        "exported func main() int {true; 200; return 300;}", true,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 300 }) => {}
        other => panic!("expected VonInt(300), got {:?}", other),
    }
}
/*
  test("Test block") {
    val compile = RunCompilation.test("exported func main() int {true; 200; return 300;}")
    compile.evalForKind(Vector()) match { case VonInt(300) => }
  }
*/
// mig: fn test_generic
#[test]
fn test_generic() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &typing_bump, &instantiating_bump,
        "\nfunc drop(x int) { }\nfunc bork<T>(a T) void where func drop(T)void {\n  // implicitly calls drop\n}\nexported func main() {\n  bork(3);\n}\n",
        false,
    );
    let _ = compile.eval_for_kind_primitive_args(Vec::new());
}
/*
  test("Test generic") {
    val compile = RunCompilation.test(
      """
        |func drop(x int) { }
        |func bork<T>(a T) void where func drop(T)void {
        |  // implicitly calls drop
        |}
        |exported func main() {
        |  bork(3);
        |}
      """.stripMargin, false)
    compile.evalForKind(Vector())
  }
*/
// mig: fn test_multiple_invocations_of_generic
#[test]
fn test_multiple_invocations_of_generic() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &typing_bump, &instantiating_bump,
        "\nfunc bork<T>(a T, b T) T where func drop(T)void { return a; }\nexported func main() int {true bork false; 2 bork 2; return 3 bork 3;}\n",
        true,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 3 }) => {}
        other => panic!("expected VonInt(3), got {:?}", other),
    }
}
/*
  test("Test multiple invocations of generic") {
    val compile = RunCompilation.test(
      """
        |func bork<T>(a T, b T) T where func drop(T)void { return a; }
        |exported func main() int {true bork false; 2 bork 2; return 3 bork 3;}
      """.stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn test_mutating_a_local_var
#[test]
fn test_mutating_a_local_var() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &typing_bump, &instantiating_bump,
        "exported func main() {a = 3; set a = 4; }", true,
    );
    compile.run_primitive_args(Vec::new());
}
/*
  test("Test mutating a local var") {
    val compile = RunCompilation.test("exported func main() {a = 3; set a = 4; }")
    compile.run(Vector())
  }
*/
// mig: fn test_returning_a_local_mutable_var
#[test]
fn test_returning_a_local_mutable_var() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &typing_bump, &instantiating_bump,
        "exported func main() int {a = 3; set a = 4; return a;}", true,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 4 }) => {}
        other => panic!("expected VonInt(4), got {:?}", other),
    }
}
/*
  test("Test returning a local mutable var") {
    val compile = RunCompilation.test("exported func main() int {a = 3; set a = 4; return a;}")
    compile.evalForKind(Vector()) match { case VonInt(4) => }
  }
*/
// mig: fn test_taking_a_callable_param
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn test_taking_a_callable_param() { panic!("Unmigrated test: test_taking_a_callable_param"); }
/*
  test("Test taking a callable param") {
    val compile = RunCompilation.test(
      """
        |func do<T>(callable T) int where func(&T)int, func drop(T)void { return callable(); }
        |exported func main() int { return do({ 3 }); }
      """.stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn stamps_an_interface_template_via_a_function_parameter
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn stamps_an_interface_template_via_a_function_parameter() { panic!("Unmigrated test: stamps_an_interface_template_via_a_function_parameter"); }
/*
  test("Stamps an interface template via a function parameter") {
    val compile = RunCompilation.test(
      """
        |interface MyInterface<T Ref> { }
        |func doAThing<T>(i MyInterface<T>) { }
        |
        |struct SomeStruct<T Ref> { }
        |func doAThing<T>(s SomeStruct<T>) { }
        |impl<T> MyInterface<T> for SomeStruct<T>;
        |
        |export MyInterface<int> as SomeIntInterface;
        |export SomeStruct<int> as SomeIntStruct;
        |
        |exported func main(a SomeStruct<int>) {
        |  doAThing<int>(a);
        |}
      """.stripMargin)
    val packageH = compile.getHamuts().lookupPackage(PackageCoordinate.TEST_TLD(compile.interner, compile.keywords))
    val heap = new Heap(System.out)
    val ref =
      heap.add(OwnH, YonderH, StructInstanceV(
        packageH.lookupStruct("SomeStruct<i32>"),
        Some(Vector())))
    compile.run(heap, Vector(ref))
  }
*/
// mig: fn tests_unstackifying_a_variable_multiple_times_in_a_function
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn tests_unstackifying_a_variable_multiple_times_in_a_function() { panic!("Unmigrated test: tests_unstackifying_a_variable_multiple_times_in_a_function"); }
/*
  test("Tests unstackifying a variable multiple times in a function") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/multiUnstackify.vale"))
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn reads_a_struct_member
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn reads_a_struct_member() { panic!("Unmigrated test: reads_a_struct_member"); }
/*
  test("Reads a struct member") {
    val compile = RunCompilation.test(
      """
        |struct MyStruct { a int; }
        |exported func main() int { ms = MyStruct(7); return ms.a; }
      """.stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(7) => }
  }
*/
// mig: fn add_two_i64
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn add_two_i64() { panic!("Unmigrated test: add_two_i64"); }
/*
  test("Add two i64") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/add64ret.vale"))
    val coutputs = compile.getCompilerOutputs()
    val hamuts = compile.getHamuts()
    compile.evalForKind(Vector()) match { case VonInt(42L) => }
  }
*/
// mig: fn equals_equals_equals_true
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn equals_equals_equals_true() { panic!("Unmigrated test: equals_equals_equals_true"); }
/*
  test("=== true") {
    val compile = RunCompilation.test(
      """
        |struct MyStruct { a int; }
        |exported func main() bool {
        |  a = MyStruct(7);
        |  return &a === &a;
        |}
      """.stripMargin)
    compile.evalForKind(Vector()) match { case VonBool(true) => }
  }
*/
// mig: fn equals_equals_equals_false
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn equals_equals_equals_false() { panic!("Unmigrated test: equals_equals_equals_false"); }
/*
  test("=== false") {
    val compile = RunCompilation.test(
      """
        |struct MyStruct { a int; }
        |exported func main() bool {
        |  a = MyStruct(7);
        |  b = MyStruct(7);
        |  return &a === &b;
        |}
      """.stripMargin)
    compile.evalForKind(Vector()) match { case VonBool(false) => }
  }
*/
// mig: fn lambda_can_call_sibling_lambda
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn lambda_can_call_sibling_lambda() { panic!("Unmigrated test: lambda_can_call_sibling_lambda"); }
/*
  // See LCCSL
  test("Lambda can call sibling lambda") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  continueF = (x) => { x };
        |  barkF = (x) => { continueF(x) };
        |  return barkF(42);
        |}
    """.stripMargin)
    compile.evalForKind(Vector()) match {
      case VonInt(42) =>
    }
  }
*/
// mig: fn set_swapping_locals
#[test]
fn set_swapping_locals() {
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
    let code = crate::tests::tests::load_expected("programs/mutswaplocals.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &typing_bump, &instantiating_bump,
        &code, true,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("set swapping locals") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/mutswaplocals.vale"))
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn simple_extern_function
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_extern_function() { panic!("Unmigrated test: simple_extern_function"); }
/*
  test("Simple extern function") {
    val compile = RunCompilation.test(
      """
        |extern func __vbi_addI32(left int, right int) int;
        |exported func main() int { return __vbi_addI32(27, 15); }
        |""".stripMargin,
      false)
    compile.evalForKind(Vector()) match {
      case VonInt(42) =>
    }
  }
*/
// mig: fn extern_function_returning_extern_struct
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn extern_function_returning_extern_struct() { panic!("Unmigrated test: extern_function_returning_extern_struct"); }
/*
  test("Extern function returning extern struct") {
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
    compile.evalForKind(Vector()) match {
      case VonInt(42) =>
    }
  }


*/
// mig: fn extern_rust_vec
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn extern_rust_vec() { panic!("Unmigrated test: extern_rust_vec"); }
/*
  test("Extern rust Vec") {
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
    compile.evalForKind(Vector()) match {
      case VonInt(42) =>
    }
  }
*/
// mig: fn extern_rust_vec_capacity
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn extern_rust_vec_capacity() { panic!("Unmigrated test: extern_rust_vec_capacity"); }
/*
  test("Extern rust Vec capacity") {
    val compile = RunCompilation.test(
      """
        |extern struct Vec<T> imm {
        |  extern func with_capacity(c i64) Vec<T>;
        |  extern func capacity(self Vec<T>) i64;
        |}
        |exported func main() i64 {
        |  v = Vec<int>.with_capacity(42i64);
        |  return Vec<int>.capacity(v);
        |}
        |""".stripMargin,
      false)
    compile.evalForKind(Vector()) match {
      case VonInt(42) =>
    }
  }
*/
// mig: fn extern_method_on_generic_extern_struct_returns_expected_value
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn extern_method_on_generic_extern_struct_returns_expected_value() { panic!("Unmigrated test: extern_method_on_generic_extern_struct_returns_expected_value"); }
/*
  test("Extern method on generic extern struct returns expected value") {
    // Validates the FunctionExternT genericParameterInheritance plumbing — the typing-pass
    // chain through Compiler.scala → FunctionCompilerCore → CompilerOutputs → HinputsT, and
    // Instantiator's linear-scan lookup of inheritance counts when collecting generic externs
    // at callsites. Vivem matches on the humanized fullyQualifiedName (not the wire-format
    // SimpleId), so the wire-format reshape is exercised separately in HammerTests; this
    // test catches regressions in the call-path plumbing that downstream needs.
    val compile = RunCompilation.test(
      """
        |extern struct Vec<T> imm {
        |  extern func with_capacity(c i64) Vec<T>;
        |  extern func capacity(self Vec<T>) i64;
        |}
        |exported func main() i64 {
        |  v = Vec<int>.with_capacity(42i64);
        |  return v.capacity();
        |}
        |""".stripMargin,
      false)
    compile.evalForKind(Vector()) match {
      case VonInt(42) =>
    }
  }

  // Known failure 2020-08-20
  // The reason this isnt working:
  // The InterfaceCall2 instruction is only ever created as part of an abstract function's body.
  // (Yes, abstract functions have a body, specifically only containing an InterfaceCall2 on the param)
  // So, if there's *already* a body there, we won't be making the InterfaceCall2 instruction.
  // Short term, let's disallow default implementations.
//  test("Tests virtual doesn't get called if theres a better override") {
//    val compile = RunCompilation.test(
//      """
//        |interface MyOption { }
//        |
//        |struct MySome {
//        |  value MyList;
//        |}
//        |impl MyOption for MySome;
//        |
//        |struct MyNone { }
//        |impl MyOption for MyNone;
//        |
//        |
//        |struct MyList {
//        |  value int;
//        |  next MyOption;
//        |}
//        |
//        |func sum(list *MyList) int {
//        |  list.value + sum(list.next)
//        |}
//        |
//        |func sum(virtual opt *MyOption) int { panic("called virtual sum!") }
//        |func sum(opt *MyNone impl MyOption) int { return 0; }
//        |func sum(opt *MySome impl MyOption) int {
//        |   sum(opt.value)
//        |}
//        |
//        |
//        |exported func main() int {
//        |  list = MyList(10, MySome(MyList(20, MySome(MyList(30, MyNone())))));
//        |  return sum(&list);
//        |}
//        |
//        |""".stripMargin)
//    val hamuts = compile.getHamuts();
//    compile.evalForKind(Vector()) match { case VonInt(60) => }
//  }

}

*/
