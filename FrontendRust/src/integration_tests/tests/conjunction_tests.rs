/*
package dev.vale

import dev.vale.von.VonBool
import org.scalatest._

*/
// mig: struct ConjunctionTests
pub struct ConjunctionTests;
/*
class ConjunctionTests extends FunSuite with Matchers {
*/
// mig: fn and
#[test]
fn and() {
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
        "exported func main() bool { return true and true; }",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Bool(crate::von::ast::VonBool { value: true }) => {}
        other => panic!("expected VonBool(true), got {:?}", other),
    }
}
/*
  test("And") {
    val compile = RunCompilation.test("exported func main() bool { return true and true; }")
    compile.evalForKind(Vector()) match { case VonBool(true) => }
  }
*/
// mig: fn or
#[test]
fn or() {
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
        "exported func main() bool { return true or false; }",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Bool(crate::von::ast::VonBool { value: true }) => {}
        other => panic!("expected VonBool(true), got {:?}", other),
    }
}
/*
  test("Or") {
    val compile = RunCompilation.test("exported func main() bool { return true or false; }")
    compile.evalForKind(Vector()) match { case VonBool(true) => }
  }
*/
// mig: fn and_short_circuiting
#[test]
fn and_short_circuiting() {
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
        "func printAndFalse() bool { print(\"bork!\"); return false; }\nexported func main() bool { return printAndFalse() and printAndFalse(); }\n",
    );
    assert_eq!(compile.eval_for_stdout(Vec::new()), "bork!");
}
/*
  test("And short-circuiting") {
    val compile = RunCompilation.test(
      """
        |func printAndFalse() bool { print("bork!"); return false; }
        |exported func main() bool { return printAndFalse() and printAndFalse(); }
        |""".stripMargin)

    compile.evalForStdout(Vector()) shouldEqual "bork!"
  }
*/
// mig: fn or_short_circuiting
#[test]
fn or_short_circuiting() {
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
        "func printAndTrue() bool { print(\"bork!\"); return true; }\nexported func main() bool { return printAndTrue() or printAndTrue(); }\n",
    );
    assert_eq!(compile.eval_for_stdout(Vec::new()), "bork!");
}
/*
  test("Or short-circuiting") {
    val compile = RunCompilation.test(
      """
        |func printAndTrue() bool { print("bork!"); return true; }
        |exported func main() bool { return printAndTrue() or printAndTrue(); }
        |""".stripMargin)

    compile.evalForStdout(Vector()) shouldEqual "bork!"
  }
}

*/
