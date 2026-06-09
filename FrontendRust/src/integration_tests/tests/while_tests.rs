/*
package dev.vale

import dev.vale.von.VonInt
import org.scalatest._

*/
// mig: struct WhileTests
pub struct WhileTests;
/*
class WhileTests extends FunSuite with Matchers {
*/
// mig: fn simple_while_loop_that_doesnt_execute
#[test]
fn simple_while_loop_that_doesnt_execute() {
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
        "exported func main() int {\n  while (false) {}\n  return 5;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 5 }) => {}
        other => panic!("expected VonInt(5), got {:?}", other),
    }
}
/*
  test("Simple while loop that doesnt execute") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while (false) {}
        |  return 5;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn test_a_for_ish_while_loop
#[test]
fn test_a_for_ish_while_loop() {
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
        "exported func main() int {\n  i = 0;\n  while (i < 4) {\n    set i = i + 1;\n  }\n  return i;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 4 }) => {}
        other => panic!("expected VonInt(4), got {:?}", other),
    }
}
/*
  test("Test a for-ish while loop") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  i = 0;
        |  while (i < 4) {
        |    set i = i + 1;
        |  }
        |  return i;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(4) => }
  }
*/
// mig: fn tests_a_while_loop_with_a_complex_condition
#[test]
fn tests_a_while_loop_with_a_complex_condition() {
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
        "import ioutils.*;\nimport printutils.*;\nexported func main() int {\n  key = 0;\n  while set key = __getch(); key < 96 {\n    print(key);\n  }\n  return key;\n}\n",
    );
    match compile.eval_for_kind_primitive_args_with_stdin(Vec::new(), vec!["A".to_string(), "B".to_string(), "c".to_string()]).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 99 }) => {}
        other => panic!("expected VonInt(99), got {:?}", other),
    }
}
/*
  test("Tests a while loop with a complex condition") {
    val compile = RunCompilation.test(
      """import ioutils.*;
        |import printutils.*;
        |exported func main() int {
        |  key = 0;
        |  while set key = __getch(); key < 96 {
        |    print(key);
        |  }
        |  return key;
        |}
      """.stripMargin)

    compile.evalForKind(Vector(), Vector("A", "B", "c")) match { case VonInt(99) => }
  }
*/
// mig: fn tests_a_while_loop_with_a_set_in_it
#[test]
fn tests_a_while_loop_with_a_set_in_it() {
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
        "import printutils.*;\nimport ioutils.*;\nimport logic.*;\n\nexported func main() int {\n  key = 0;\n  while set key = __getch(); key != 99 {\n    print(key);\n  }\n  return key;\n}\n",
    );
    match compile.eval_for_kind_primitive_args_with_stdin(Vec::new(), vec!["A".to_string(), "B".to_string(), "c".to_string()]).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 99 }) => {}
        other => panic!("expected VonInt(99), got {:?}", other),
    }
}
/*
  test("Tests a while loop with a set in it") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |import ioutils.*;
        |import logic.*;
        |
        |exported func main() int {
        |  key = 0;
        |  while set key = __getch(); key != 99 {
        |    print(key);
        |  }
        |  return key;
        |}
      """.stripMargin)

    compile.evalForKind(Vector(), Vector("A", "B", "c")) match { case VonInt(99) => }
  }
*/
// mig: fn tests_a_while_loop_with_a_declaration_in_it
#[test]
fn tests_a_while_loop_with_a_declaration_in_it() {
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
        "import printutils.*;\nimport ioutils.*;\nimport logic.*;\n\nexported func main() {\n  while key = __getch(); key != 99 {\n    print(key);\n  }\n}\n",
    );
    compile.eval_for_kind_primitive_args_with_stdin(Vec::new(), vec!["A".to_string(), "B".to_string(), "c".to_string()]).unwrap();
}
/*
  test("Tests a while loop with a declaration in it") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |import ioutils.*;
        |import logic.*;
        |
        |exported func main() {
        |  while key = __getch(); key != 99 {
        |    print(key);
        |  }
        |}
      """.stripMargin)

    compile.evalForKind(Vector(), Vector("A", "B", "c"))
  }
*/
// mig: fn return_from_infinite_while_loop
#[test]
fn return_from_infinite_while_loop() {
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
        "exported func main() int {\n  while (true) {\n    return 9;\n  }\n  return __vbi_panic();\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 9 }) => {}
        other => panic!("expected VonInt(9), got {:?}", other),
    }
}
/*
  test("Return from infinite while loop") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while (true) {
        |    return 9;
        |  }
        |  return __vbi_panic();
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(9) => }
  }
*/
// mig: fn infinite_while_loop_conditional_break
#[test]
fn infinite_while_loop_conditional_break() {
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
        "exported func main() int {\n  while true {\n    if true {\n      break;\n    }\n    4;\n  }\n  return 42;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Infinite while loop conditional break") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while true {
        |    if true {
        |      break;
        |    }
        |    4;
        |  }
        |  return 42;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn infinite_while_loop_unconditional_break
#[test]
fn infinite_while_loop_unconditional_break() {
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
        "exported func main() int {\n  while true {\n    break;\n  }\n  return 42;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Infinite while loop unconditional break") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while true {
        |    break;
        |  }
        |  return 42;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn infinite_while_loop_conditional_break_from_both_sides
#[test]
fn infinite_while_loop_conditional_break_from_both_sides() {
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
        "exported func main() int {\n  while true {\n    if true {\n      break;\n    } else {\n      break;\n    }\n  }\n  return 42;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Infinite while loop conditional break from both sides") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while true {
        |    if true {
        |      break;
        |    } else {
        |      break;
        |    }
        |  }
        |  return 42;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn infinite_while_loop_conditional_return
#[test]
fn infinite_while_loop_conditional_return() {
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
        "exported func main() int {\n  while true {\n    if true {\n      return 42;\n    }\n    73;\n  }\n  return 74;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Infinite while loop conditional return") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while true {
        |    if true {
        |      return 42;
        |    }
        |    73;
        |  }
        |  return 74;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn infinite_while_loop_unconditional_return
#[test]
fn infinite_while_loop_unconditional_return() {
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
        "exported func main() int {\n  while true {\n    return 42;\n  }\n  return 73;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Infinite while loop unconditional return") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while true {
        |    return 42;
        |  }
        |  return 73;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn infinite_while_loop_conditional_return_from_both_sides
#[test]
fn infinite_while_loop_conditional_return_from_both_sides() {
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
        "exported func main() int {\n  while true {\n    if true {\n      return 42;\n    } else {\n      return 73;\n    }\n  }\n  return 74;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Infinite while loop conditional return from both sides") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while true {
        |    if true {
        |      return 42;
        |    } else {
        |      return 73;
        |    }
        |  }
        |  return 74;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn while_with_condition_declaration
#[test]
fn while_with_condition_declaration() {
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
        "exported func main() int {\n  while x = 42; x < 50 { return x; }\n  return 73;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("While with condition declaration") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  while x = 42; x < 50 { return x; }
        |  return 73;
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn each_on_int_range
#[test]
fn each_on_int_range() {
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
        "\nimport intrange.*;\n\nexported func main() int {\n  sum = 0;\n  foreach i in 0..10 {\n    set sum = sum + i;\n  }\n  return sum;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 45 }) => {}
        other => panic!("expected VonInt(45), got {:?}", other),
    }
}
/*
  test("each on int range") {
    val compile = RunCompilation.test(
      """
        |import intrange.*;
        |
        |exported func main() int {
        |  sum = 0;
        |  foreach i in 0..10 {
        |    set sum = sum + i;
        |  }
        |  return sum;
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(45) => }
  }
*/
// mig: fn parallel_foreach
#[test]
fn parallel_foreach() {
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
        "\nimport intrange.*;\nimport list.*;\nimport listprintutils.*;\n\nexported func main() {\n  exponent = 3;\n\n  results =\n    parallel foreach i in 0..5 {\n      i + 1\n    };\n\n  println(&results);\n}\n",
    );
    assert_eq!(compile.eval_for_stdout(Vec::new()).unwrap().trim(), "[1, 2, 3, 4, 5]");
}
/*
  test("Parallel foreach") {
    val compile = RunCompilation.test(
      """
        |import intrange.*;
        |import list.*;
        |import listprintutils.*;
        |
        |exported func main() {
        |  exponent = 3;
        |
        |  results =
        |    parallel foreach i in 0..5 {
        |      i + 1
        |    };
        |
        |  println(&results);
        |}
        |""".stripMargin)
    compile.evalForStdout(Vector()).trim shouldEqual "[1, 2, 3, 4, 5]"
  }
*/
// mig: fn mutable_foreach
#[test]
fn mutable_foreach() {
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
        "\n// A fake 1-element list\nstruct Ship {\n  fuel! int;\n}\nstruct List {\n  ship Ship;\n}\n\nstruct ListIter {\n  ship &Ship;\n  pos! int;\n}\nfunc begin(self &List) ListIter { ListIter(&self.ship, 0) }\nfunc next(iter &ListIter) Opt<&Ship> {\n  if pos = set iter.pos = iter.pos + 1; pos < 1 {\n    Some<&Ship>(iter.ship)\n  } else {\n    None<&Ship>()\n  }\n}\n\nexported func main() int {\n  list = List(Ship(73));\n  foreach i in &list {\n    set i.fuel = 42;\n  }\n  return list.ship.fuel;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Mutable foreach") {
    val compile = RunCompilation.test(
      """
        |// A fake 1-element list
        |struct Ship {
        |  fuel! int;
        |}
        |struct List {
        |  ship Ship;
        |}
        |
        |struct ListIter {
        |  ship &Ship;
        |  pos! int;
        |}
        |func begin(self &List) ListIter { ListIter(&self.ship, 0) }
        |func next(iter &ListIter) Opt<&Ship> {
        |  if pos = set iter.pos = iter.pos + 1; pos < 1 {
        |    Some<&Ship>(iter.ship)
        |  } else {
        |    None<&Ship>()
        |  }
        |}
        |
        |exported func main() int {
        |  list = List(Ship(73));
        |  foreach i in &list {
        |    set i.fuel = 42;
        |  }
        |  return list.ship.fuel;
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn each_on_int_range_with_conditional_break
#[test]
fn each_on_int_range_with_conditional_break() {
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
        "\nimport intrange.*;\nimport list.*;\n\nexported func main() int {\n  sum = 0;\n  results =\n    foreach i in 0..10 {\n      if true {\n        break;\n      }\n      3\n    };\n  return 0;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 0 }) => {}
        other => panic!("expected VonInt(0), got {:?}", other),
    }
}
/*
  test("each on int range with conditional break") {
    val compile = RunCompilation.test(
      """
        |import intrange.*;
        |import list.*;
        |
        |exported func main() int {
        |  sum = 0;
        |  results =
        |    foreach i in 0..10 {
        |      if true {
        |        break;
        |      }
        |      3
        |    };
        |  return 0;
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(0) => }
  }
*/
// mig: fn each_on_int_range_with_unconditional_break
#[test]
fn each_on_int_range_with_unconditional_break() {
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
        "\nimport intrange.*;\n\nexported func main() int {\n  sum = 0;\n  foreach i in 0..10 {\n    break;\n  }\n  return sum;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 0 }) => {}
        other => panic!("expected VonInt(0), got {:?}", other),
    }
}
/*
  test("each on int range with unconditional break") {
    val compile = RunCompilation.test(
      """
        |import intrange.*;
        |
        |exported func main() int {
        |  sum = 0;
        |  foreach i in 0..10 {
        |    break;
        |  }
        |  return sum;
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(0) => }
  }
*/
// mig: fn each_on_int_range_with_conditional_break_from_both_branches
#[test]
fn each_on_int_range_with_conditional_break_from_both_branches() {
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
        "\nimport intrange.*;\n\nexported func main() int {\n  sum = 0;\n  foreach i in 0..10 {\n    if true {\n      break;\n    } else {\n      break;\n    }\n  }\n  return sum;\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 0 }) => {}
        other => panic!("expected VonInt(0), got {:?}", other),
    }
}
/*
  test("each on int range with conditional break from both branches") {
    val compile = RunCompilation.test(
      """
        |import intrange.*;
        |
        |exported func main() int {
        |  sum = 0;
        |  foreach i in 0..10 {
        |    if true {
        |      break;
        |    } else {
        |      break;
        |    }
        |  }
        |  return sum;
        |}
        |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(0) => }
  }

  //
//  test("Tests a while loop with a move in it") {
//    val compile = RunCompilation.test(
//      """
//        |func doThings(m: Marine) { }
//        |struct Marine { hp: int; }
//        |exported func main() int {
//        |  m = Marine(7);
//        |  while (true) {
//        |    doThings(m);
//        |  }
//        |  return 4;
//        |}
//      """.stripMargin)
//
//    // should fail
//
//    compile.evalForKind(Vector()) match { case VonInt(4) => }
//  }
}

*/
