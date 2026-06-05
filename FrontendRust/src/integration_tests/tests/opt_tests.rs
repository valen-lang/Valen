// mig: struct OptTests
pub struct OptTests;
/*
package dev.vale

import dev.vale.typing.env.ReferenceLocalVariableT
import dev.vale.typing._
import dev.vale.typing.types._
import dev.vale.von.VonInt
import org.scalatest._

class OptTests extends FunSuite with Matchers {
*/
// mig: fn test_empty_and_get_for_some
#[test]
fn test_empty_and_get_for_some() {
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
        "\nimport v.builtins.opt.*;\n\nexported func main() int {\n  opt Opt<int> = Some(9);\n  return if (opt.isEmpty()) { 0 }\n    else { opt.get() };\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 9 }) => {}
        other => panic!("expected VonInt(9), got {:?}", other),
    }
}
/*
  test("Test empty and get for Some") {
    val compile = RunCompilation.testNoBuiltins(
        """
          |import v.builtins.opt.*;
          |
          |exported func main() int {
          |  opt Opt<int> = Some(9);
          |  return if (opt.isEmpty()) { 0 }
          |    else { opt.get() };
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn test_empty_and_get_for_none
#[test]
fn test_empty_and_get_for_none() {
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
        "\nexported func main() int {\n  opt Opt<int> = None<int>();\n  return if (opt.isEmpty()) { 0 }\n    else { opt.get() };\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 0 }) => {}
        other => panic!("expected VonInt(0), got {:?}", other),
    }
}
/*
  test("Test empty and get for None") {
    val compile = RunCompilation.test(
        """
          |exported func main() int {
          |  opt Opt<int> = None<int>();
          |  return if (opt.isEmpty()) { 0 }
          |    else { opt.get() };
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(0) => }
  }
*/
// mig: fn test_empty_and_get_for_borrow
#[test]
fn test_empty_and_get_for_borrow() {
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
        "\n// This is the same as the one in optutils.vale, just named differently,\n// so its easier to debug.\nfunc borrowGet<T>(opt &Some<T>) &T { &opt.value }\n\nstruct Spaceship { fuel int; }\nexported func main() int {\n  s = Spaceship(42);\n  bork = Some<&Spaceship>(&s);\n  return bork.borrowGet().fuel;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Test empty and get for borrow") {
    val compile = RunCompilation.test(
        """
          |// This is the same as the one in optutils.vale, just named differently,
          |// so its easier to debug.
          |func borrowGet<T>(opt &Some<T>) &T { &opt.value }
          |
          |struct Spaceship { fuel int; }
          |exported func main() int {
          |  s = Spaceship(42);
          |  bork = Some<&Spaceship>(&s);
          |  return bork.borrowGet().fuel;
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/

/*
}

*/
