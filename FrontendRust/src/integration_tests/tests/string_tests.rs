/*
package dev.vale

import dev.vale.typing.ast.ConstantStrTE
import dev.vale.typing._
import dev.vale.von.{VonInt, VonStr}
import org.scalatest._

*/
// mig: struct StringTests
pub struct StringTests;
/*
class StringTests extends FunSuite with Matchers {
*/
// mig: fn simple_string
#[test]
fn simple_string() {
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
        "exported func main() str {\n  return \"sprogwoggle\";\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::ConstantStr(crate::typing::ast::expressions::ConstantStrTE { value: crate::interner::StrI("sprogwoggle"), .. }) => Some(())
        );
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value }) if value == "sprogwoggle" => {}
        other => panic!("expected VonStr(\"sprogwoggle\"), got {:?}", other),
    }
}
/*
  test("Simple string") {
    val compile = RunCompilation.test(
      """
        |exported func main() str {
        |  return "sprogwoggle";
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), { case ConstantStrTE("sprogwoggle", _) => })

    compile.evalForKind(Vector()) match { case VonStr("sprogwoggle") => }
  }
*/
// mig: fn empty_string
#[test]
fn empty_string() {
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
        "exported func main() str {\n  return \"\";\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::ConstantStr(crate::typing::ast::expressions::ConstantStrTE { value: crate::interner::StrI(""), .. }) => Some(())
        );
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value }) if value == "" => {}
        other => panic!("expected VonStr(\"\"), got {:?}", other),
    }
}
/*
  test("Empty string") {
    val compile = RunCompilation.test(
      """
        |exported func main() str {
        |  return "";
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), { case ConstantStrTE("", _) => })

    compile.evalForKind(Vector()) match { case VonStr("") => }
  }
*/
// mig: fn string_with_escapes
#[test]
fn string_with_escapes() {
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
        "exported func main() str {\n  return \"sprog\\nwoggle\";\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::ConstantStr(crate::typing::ast::expressions::ConstantStrTE { value: crate::interner::StrI("sprog\nwoggle"), .. }) => Some(())
        );
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value }) if value == "sprog\nwoggle" => {}
        other => panic!("expected VonStr(\"sprog\\nwoggle\"), got {:?}", other),
    }
}
/*
  test("String with escapes") {
    val compile = RunCompilation.test(
      """
        |exported func main() str {
        |  return "sprog\nwoggle";
        |}
        |""".stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), { case ConstantStrTE("sprog\nwoggle", _) => })

    compile.evalForKind(Vector()) match { case VonStr("sprog\nwoggle") => }
  }
*/
// mig: fn string_with_hex_escape
#[test]
fn string_with_hex_escape() {
    let code = "exported func main() str { return \"sprog\\u001bwoggle\"; }";
    // This assert makes sure the above is making the input we actually intend.
    // Real source files from disk are going to have a backslash character and then a u,
    // they won't have the 0x1b byte.
    assert!(code.contains("\\u001b"));
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
        code,
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::ConstantStr(crate::typing::ast::expressions::ConstantStrTE { value: crate::interner::StrI(x), .. }) => {
                assert_eq!(*x, "sprog\u{001b}woggle");
                Some(())
            }
        );
    }
    let result = match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value }) => value,
        other => panic!("expected VonStr, got {:?}", other),
    };
    assert_eq!(result.len(), 12);
    assert_eq!(result, "sprog\u{001b}woggle");
}
/*
  test("String with hex escape") {
    val code = "exported func main() str { return \"sprog\\u001bwoggle\"; }"
    // This assert makes sure the above is making the input we actually intend.
    // Real source files from disk are going to have a backslash character and then a u,
    // they won't have the 0x1b byte.
    vassert(code.contains("\\u001b"))

    val compile = RunCompilation.test(code)

    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), {
      case ConstantStrTE(x, _) => {
        x shouldEqual "sprog\u001bwoggle"
      }
    })

    val VonStr(result) = compile.evalForKind(Vector())
    result.size shouldEqual 12
    result shouldEqual "sprog\u001bwoggle"
  }

*/
// mig: fn int_to_string
#[test]
fn int_to_string() {
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
    let source = crate::tests::tests::load_expected("programs/strings/inttostr.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 4 }) => {}
        other => panic!("expected VonInt(4), got {:?}", other),
    }
}
/*
  test("int to string") {
    val compile = RunCompilation.test( Tests.loadExpected("programs/strings/inttostr.vale"))
    compile.evalForKind(Vector()) match { case VonInt(4) => }
  }
*/
// mig: fn i64_to_string
#[test]
fn i64_to_string() {
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
    let source = crate::tests::tests::load_expected("programs/strings/i64tostr.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 4 }) => {}
        other => panic!("expected VonInt(4), got {:?}", other),
    }
}
/*
  test("i64 to string") {
    val compile = RunCompilation.test( Tests.loadExpected("programs/strings/i64tostr.vale"))
    compile.evalForKind(Vector()) match { case VonInt(4) => }
  }
*/
// mig: fn string_length
#[test]
fn string_length() {
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
    let source = crate::tests::tests::load_expected("programs/strings/strlen.vale");
    let mut compile = crate::integration_tests::tests::run_compilation::test(
        &compilation_bump,
        &hammer_interner, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena,
        &instantiating_bump,
        &source,
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 12 }) => {}
        other => panic!("expected VonInt(12), got {:?}", other),
    }
}
/*
  test("String length") {
    val compile = RunCompilation.test( Tests.loadExpected("programs/strings/strlen.vale"))

    compile.evalForKind(Vector()) match { case VonInt(12) => }
  }
*/
// mig: fn strings_equal
#[test]
fn strings_equal() {
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
    let source = crate::tests::tests::load_expected("programs/strings/strneq.vale");
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
  test("Strings equal") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/strings/strneq.vale"))

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn string_interpolate
#[test]
fn string_interpolate() {
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
        "func +(s str, i int) str { return s + str(i); }\nfunc ns(i int) int { return i; }\nexported func main() str { return \"\"\"bl\"{ns(4)}rg\"\"\"; }",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value }) if value == "bl\"4rg" => {}
        other => panic!("expected VonStr(\"bl\\\"4rg\"), got {:?}", other),
    }
}
/*
  test("String interpolate") {
    val compile = RunCompilation.test(
      "func +(s str, i int) str { return s + str(i); }\n" +
      "func ns(i int) int { return i; }\n" +
      "exported func main() str { return \"\"\"bl\"{ns(4)}rg\"\"\"; }")

    compile.evalForKind(Vector()) match { case VonStr("bl\"4rg") => }
  }
*/
// mig: fn slice_a_slice
#[test]
fn slice_a_slice() {
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
        "import panicutils.*;\nimport printutils.*;\n\nstruct StrSlice imm {\n  string str;\n  begin int;\n  end int;\n}\nfunc newStrSlice(string str, begin int, end int) StrSlice {\n  vassert(begin >= 0, \"slice begin was negative!\");\n  vassert(end >= 0, \"slice end was negative!\");\n  vassert(begin <= string.len(), \"slice begin was more than length!\");\n  vassert(end <= string.len(), \"slice end was more than length!\");\n  vassert(end >= begin, \"slice end was before begin!\");\n  return StrSlice(string, begin, end);\n}\n\nfunc slice(s str) StrSlice {\n  return newStrSlice(s, 0, s.len());\n}\n\nfunc slice(s str, begin int) StrSlice { return s.slice().slice(begin); }\nfunc slice(s StrSlice, begin int) StrSlice {\n  newBegin = s.begin + begin;\n  vassert(newBegin <= s.string.len(), \"slice begin is more than string length!\");\n  return newStrSlice(s.string, newBegin, s.end);\n}\n\nfunc len(s StrSlice) int {\n  return s.end - s.begin;\n}\n\nfunc slice(s str, begin int, end int) StrSlice {\n  return newStrSlice(s, begin, end);\n}\n\nfunc slice(s StrSlice, begin int, end int) StrSlice {\n  newGlyphBeginOffset = s.begin + begin;\n  newGlyphEndOffset = s.begin + end;\n  return newStrSlice(s.string, newGlyphBeginOffset, newGlyphEndOffset);\n}\n\nexported func main() int {\n  return \"hello\".slice().slice(1, 4).len();\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 3 }) => {}
        other => panic!("expected VonInt(3), got {:?}", other),
    }
}
/*
  test("Slice a slice") {
    val compile = RunCompilation.test(
        """
          |import panicutils.*;
          |import printutils.*;
          |
          |struct StrSlice imm {
          |  string str;
          |  begin int;
          |  end int;
          |}
          |func newStrSlice(string str, begin int, end int) StrSlice {
          |  vassert(begin >= 0, "slice begin was negative!");
          |  vassert(end >= 0, "slice end was negative!");
          |  vassert(begin <= string.len(), "slice begin was more than length!");
          |  vassert(end <= string.len(), "slice end was more than length!");
          |  vassert(end >= begin, "slice end was before begin!");
          |  return StrSlice(string, begin, end);
          |}
          |
          |func slice(s str) StrSlice {
          |  return newStrSlice(s, 0, s.len());
          |}
          |
          |func slice(s str, begin int) StrSlice { return s.slice().slice(begin); }
          |func slice(s StrSlice, begin int) StrSlice {
          |  newBegin = s.begin + begin;
          |  vassert(newBegin <= s.string.len(), "slice begin is more than string length!");
          |  return newStrSlice(s.string, newBegin, s.end);
          |}
          |
          |func len(s StrSlice) int {
          |  return s.end - s.begin;
          |}
          |
          |func slice(s str, begin int, end int) StrSlice {
          |  return newStrSlice(s, begin, end);
          |}
          |
          |func slice(s StrSlice, begin int, end int) StrSlice {
          |  newGlyphBeginOffset = s.begin + begin;
          |  newGlyphEndOffset = s.begin + end;
          |  return newStrSlice(s.string, newGlyphBeginOffset, newGlyphEndOffset);
          |}
          |
          |exported func main() int {
          |  return "hello".slice().slice(1, 4).len();
          |}
          |""".stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
}

*/
