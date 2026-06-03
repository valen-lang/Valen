/*
package dev.vale

import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing._
import dev.vale.typing.types._
import dev.vale.von.VonInt
import org.scalatest._
*/
// mig: struct FloatTests
pub struct FloatTests;
/*
class FloatTests extends FunSuite with Matchers {
*/
// mig: fn print_float
#[test]
fn print_float() {
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
        "import printutils.*;\n\nexported func main() {\n  a = 42.125;\n  print(a);\n}\n",
        true,
    );
    assert_eq!(compile.eval_for_stdout(Vec::new()).trim(), "42.125");
}
/*
  test("Print float") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |
        |exported func main() {
        |  a = 42.125;
        |  print(a);
        |}
      """.stripMargin)

    compile.evalForStdout(Vector()).trim() shouldEqual "42.125"
  }
*/
// mig: fn float_arithmetic
#[test]
fn float_arithmetic() {
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
    let source = crate::tests::tests::load_expected("programs/floatarithmetic.vale");
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
  test("Float arithmetic") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/floatarithmetic.vale"))

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn float_equals
#[test]
fn float_equals() {
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
    let source = crate::tests::tests::load_expected("programs/floateq.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source, true,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Float equals") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/floateq.vale"))

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn concat_string_and_float
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn concat_string_and_float() { panic!("Unmigrated test: concat_string_and_float"); }
/*
  test("Concat string and float") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/concatstrfloat.vale"))

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
}

*/
