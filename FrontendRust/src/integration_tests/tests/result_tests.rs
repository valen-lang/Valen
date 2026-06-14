use crate::integration_tests::tests::run_compilation::test;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::simplifying::hammer_interner::HammerInterner;
use crate::testvm::vivem::VmRuntimeErrorV;
use crate::typing::typing_interner::TypingInterner;
use crate::von::ast::IVonData;
use crate::von::ast::VonInt;
use crate::von::ast::VonStr;
// mig: struct ResultTests
pub struct ResultTests;

/*
package dev.vale

import dev.vale.testvm.PanicException
import dev.vale.von.{VonInt, VonStr}
import org.scalatest._

class ResultTests extends FunSuite with Matchers {
*/
// mig: fn test_borrow_is_ok_and_expect_for_ok
#[test]
fn test_borrow_is_ok_and_expect_for_ok() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r#"
import v.builtins.panicutils.*;
import v.builtins.result.*;

exported func main() int {
  result Result<int, str> = Ok<int, str>(42);
  return if (result.is_ok()) { result.expect("eh") }
    else { panic("wat") };
}
"#,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}

/*
  test("Test borrow is_ok and expect for Ok") {
    val compile = RunCompilation.test(
        """
          |import v.builtins.panicutils.*;
          |import v.builtins.result.*;
          |
          |exported func main() int {
          |  result Result<int, str> = Ok<int, str>(42);
          |  return if (result.is_ok()) { result.expect("eh") }
          |    else { panic("wat") };
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn test_is_err_and_borrow_expect_err_for_err
#[test]
fn test_is_err_and_borrow_expect_err_for_err() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r#"
import v.builtins.panicutils.*;
import v.builtins.result.*;

exported func main() str {
  result Result<int, str> = Err<int, str>("file not found!");
  return if (result.is_err()) { result.expect_err("eh") }
    else { panic("fail!") };
}
"#,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Str(VonStr { value }) if value == "file not found!" => {}
        other => panic!("expected VonStr(\"file not found!\"), got {:?}", other),
    }
}

/*
  test("Test is_err and borrow expect_err for Err") {
    val compile = RunCompilation.test(
        """
          |import v.builtins.panicutils.*;
          |import v.builtins.result.*;
          |
          |exported func main() str {
          |  result Result<int, str> = Err<int, str>("file not found!");
          |  return if (result.is_err()) { result.expect_err("eh") }
          |    else { panic("fail!") };
          |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonStr("file not found!") => }
  }
*/
// mig: fn test_owning_expect
#[test]
fn test_owning_expect() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r#"
import v.builtins.panicutils.*;
import v.builtins.result.*;

exported func main() int {
  result Result<int, str> = Ok<int, str>(42);
  return (result).expect("eh");
}
"#,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Int(VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}

/*
  test("Test owning expect") {
    val compile = RunCompilation.test(
      """
        |import v.builtins.panicutils.*;
        |import v.builtins.result.*;
        |
        |exported func main() int {
        |  result Result<int, str> = Ok<int, str>(42);
        |  return (result).expect("eh");
        |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn test_owning_expect_err
#[test]
fn test_owning_expect_err() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r#"
import v.builtins.panicutils.*;
import v.builtins.result.*;

exported func main() str {
  result Result<int, str> = Err<int, str>("file not found!");
  return (result).expect_err("eh");
}
"#,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()).unwrap() {
        IVonData::Str(VonStr { value }) if value == "file not found!" => {}
        other => panic!("expected VonStr(\"file not found!\"), got {:?}", other),
    }
}

/*
  test("Test owning expect_err") {
    val compile = RunCompilation.test(
      """
        |import v.builtins.panicutils.*;
        |import v.builtins.result.*;
        |
        |exported func main() str {
        |  result Result<int, str> = Err<int, str>("file not found!");
        |  return (result).expect_err("eh");
        |}
        """.stripMargin)

    compile.evalForKind(Vector()) match { case VonStr("file not found!") => }
  }
*/
// mig: fn test_expect_panics_for_err
#[test]
fn test_expect_panics_for_err() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r#"
import v.builtins.panicutils.*;
import v.builtins.result.*;

exported func main() int {
  result Result<int, str> = Err<int, str>("file not found!");
  return result.expect("eh");
}
"#,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        Err(VmRuntimeErrorV::PanicException(_)) => {}
        other => panic!("Expected PanicException, got {:?}", other),
    }
}

/*
  test("Test expect() panics for Err") {
    val compile = RunCompilation.test(
        """
          |import v.builtins.panicutils.*;
          |import v.builtins.result.*;
          |
          |exported func main() int {
          |  result Result<int, str> = Err<int, str>("file not found!");
          |  return result.expect("eh");
          |}
        """.stripMargin)

    try {
      compile.evalForKind(Vector())
      vfail()
    } catch {
      case PanicException() =>
    }
  }
*/
// mig: fn test_expect_err_panics_for_ok
#[test]
fn test_expect_err_panics_for_ok() {
    let compilation_bump = bumpalo::Bump::new();
    let parse_bump = bumpalo::Bump::new();
    let scout_bump = bumpalo::Bump::new();
    let typing_bump = bumpalo::Bump::new();
    let instantiating_bump = bumpalo::Bump::new();
    let hammer_bump = bumpalo::Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let hammer_interner = HammerInterner::new(&hammer_bump);
    let typing_interner = TypingInterner::new(&typing_bump);
    let mut compile = test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        r#"
import v.builtins.panicutils.*;
import v.builtins.result.*;

exported func main() str {
  result Result<int, str> = Ok<int, str>(73);
  return result.expect_err("eh");
}
"#,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        Err(VmRuntimeErrorV::PanicException(_)) => {}
        other => panic!("Expected PanicException, got {:?}", other),
    }
}

/*
  test("Test expect_err() panics for Ok") {
    val compile = RunCompilation.test(
      """
        |import v.builtins.panicutils.*;
        |import v.builtins.result.*;
        |
        |exported func main() str {
        |  result Result<int, str> = Ok<int, str>(73);
        |  return result.expect_err("eh");
        |}
        """.stripMargin)

    try {
      compile.evalForKind(Vector())
      vfail()
    } catch {
      case PanicException() =>
    }
  }
*/

/*
}

*/
