/*
package dev.vale

import dev.vale.typing.ast.TupleTE
import dev.vale.typing.types.IntT
import dev.vale.typing._
import dev.vale.von.{VonBool, VonInt}
import org.scalatest._
*/
// mig: struct TupleTests
pub struct TupleTests;
/*
class TupleTests extends FunSuite with Matchers {
*/
// mig: fn returning_tuple_from_function_and_dotting_it
#[test]
fn returning_tuple_from_function_and_dotting_it() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test_no_builtins(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "import v.builtins.tup2.*;\nimport v.builtins.drop.*;\n\nfunc makeTup() (int, int) { return (2, 3); }\nexported func main() int {\n  return makeTup().1;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 3 }) => {}
        other => panic!("Expected VonInt(3), got {:?}", other),
    }
}
/*
  test("Returning tuple from function and dotting it") {
    val compile = RunCompilation.testNoBuiltins(
      """
        |import v.builtins.tup2.*;
        |import v.builtins.drop.*;
        |
        |func makeTup() (int, int) { return (2, 3); }
        |exported func main() int {
        |  return makeTup().1;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn tuple_with_two_things
#[test]
fn tuple_with_two_things() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test_no_builtins(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "import v.builtins.tup2.*;\nimport v.builtins.drop.*;\n\nexported func main() bool {\n  return (9, true).1;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Bool(crate::von::ast::VonBool { value: true }) => {}
        other => panic!("Expected VonBool(true), got {:?}", other),
    }
}
/*
  test("Tuple with two things") {
    val compile = RunCompilation.testNoBuiltins(
      """
        |import v.builtins.tup2.*;
        |import v.builtins.drop.*;
        |
        |exported func main() bool {
        |  return (9, true).1;
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonBool(true) => }
  }
*/
// mig: fn tuple_type
#[test]
fn tuple_type() {
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
    let mut compile = crate::integration_tests::tests::run_compilation::test_no_builtins(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        "import v.builtins.tup2.*;\nimport v.builtins.drop.*;\n\nfunc moo(a (int, int)) int { return a.1; }\n\nexported func main() int {\n  return moo((3, 4));\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 4 }) => {}
        other => panic!("Expected VonInt(4), got {:?}", other),
    }
}
/*
  test("Tuple type") {
    val compile = RunCompilation.testNoBuiltins(
      """
        |import v.builtins.tup2.*;
        |import v.builtins.drop.*;
        |
        |func moo(a (int, int)) int { return a.1; }
        |
        |exported func main() int {
        |  return moo((3, 4));
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(4) => }
  }
*/
// mig: fn simple_tuple_with_one_int
#[test]
fn simple_tuple_with_one_int() {
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
        "exported func main() int { return (9,).0; }",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        assert_eq!(main.header.return_type.kind, crate::typing::types::types::KindT::Int(crate::typing::types::types::IntT::I32));
        // Funny story, theres no such thing as a one element tuple! It becomes a one element array.
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::Tuple(_) => Some(())
        );
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 9 }) => {}
        other => panic!("Expected VonInt(9), got {:?}", other),
    }
}
/*
  test("Simple tuple with one int") {
    val compile = RunCompilation.test("exported func main() int { return (9,).0; }")

    val coutputs = compile.expectCompilerOutputs()
    coutputs.lookupFunction("main").header.returnType.kind shouldEqual IntT.i32
    // Funny story, theres no such thing as a one element tuple! It becomes a one element array.
    Collector.only(coutputs.lookupFunction("main"), { case TupleTE(_, _) => })

    compile.evalForKind(Vector()) match {
      case VonInt(9) =>
    }
  }
}

*/
