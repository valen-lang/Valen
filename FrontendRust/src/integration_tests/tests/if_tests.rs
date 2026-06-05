/*
package dev.vale

import dev.vale.postparsing._
import dev.vale.typing.ast.IfTE
import dev.vale.typing.types._
import dev.vale.testvm.IntV
import dev.vale.postparsing._
import dev.vale.typing._
import dev.vale.typing.types._
import dev.vale.von.{VonInt, VonStr}
import org.scalatest._

*/
// mig: struct IfTests
pub struct IfTests;
/*
class IfTests extends FunSuite with Matchers {
*/
// mig: fn simple_true_branch_returning_an_int
#[test]
fn simple_true_branch_returning_an_int() {
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
        "exported func main() int {\n  return if (true) { 3 } else { 5 };\n}\n",
    );
    {
        let test_str = scout_arena.intern_str("test");
        let package_coord = scout_arena.intern_package_coordinate(test_str, &[]);
        let file_coord = scout_arena.intern_file_coordinate(package_coord, "0.vale");
        let scoutput = compile.get_scoutput().expect("get_scoutput failed");
        let program_s = scoutput.file_coord_to_contents.get(file_coord).expect("file_coord not in scoutput");
        let main = program_s.lookup_function("main");
        let ret: &crate::postparsing::expressions::ReturnSE = crate::collect_only_snode!(
            crate::postparsing::test::traverse::NodeRefS::Function(main),
            crate::postparsing::test::traverse::NodeRefS::Expression(crate::postparsing::expressions::IExpressionSE::Return(r)) => Some(r)
        );
        let iff: &crate::postparsing::expressions::IfSE = crate::collect_only_snode!(
            crate::postparsing::test::traverse::NodeRefS::Expression(ret.inner),
            crate::postparsing::test::traverse::NodeRefS::Expression(crate::postparsing::expressions::IExpressionSE::If(i)) => Some(i)
        );
        crate::collect_only_snode!(
            crate::postparsing::test::traverse::NodeRefS::Expression(iff.condition),
            crate::postparsing::test::traverse::NodeRefS::Expression(crate::postparsing::expressions::IExpressionSE::ConstantBool(crate::postparsing::expressions::ConstantBoolSE { value: true, .. })) => Some(())
        );
        crate::collect_only_snode!(
            crate::postparsing::test::traverse::NodeRefS::Expression(iff.then_body.expr),
            crate::postparsing::test::traverse::NodeRefS::Expression(crate::postparsing::expressions::IExpressionSE::ConstantInt(crate::postparsing::expressions::ConstantIntSE { value: 3, .. })) => Some(())
        );
        crate::collect_only_snode!(
            crate::postparsing::test::traverse::NodeRefS::Expression(iff.else_body.expr),
            crate::postparsing::test::traverse::NodeRefS::Expression(crate::postparsing::expressions::IExpressionSE::ConstantInt(crate::postparsing::expressions::ConstantIntSE { value: 5, .. })) => Some(())
        );
    }
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        crate::collect_only_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::If(_) => Some(())
        );
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 3 }) => {}
        other => panic!("expected VonInt(3), got {:?}", other),
    }
}
/*
  test("Simple true branch returning an int") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  return if (true) { 3 } else { 5 };
        |}
      """.stripMargin)
    val programS =
      compile.getScoutput().getOrDie()
        .fileCoordToContents(
          compile.interner.intern(FileCoordinate(
            compile.interner.intern(PackageCoordinate(
              compile.interner.intern(StrI("test")),
              Vector.empty)),
            "0.vale")))
    val main = programS.lookupFunction("main")
    val ret = Collector.only(main.body, { case r @ ReturnSE(_, _) => r })
    val iff = Collector.only(ret, { case i @ IfSE(_, _, _, _) => i })
    Collector.only(iff.condition, { case ConstantBoolSE(_, true) => })
    Collector.only(iff.thenBody, { case ConstantIntSE(_, 3, _) => })
    Collector.only(iff.elseBody, { case ConstantIntSE(_, 5, _) => })

    val coutputs = compile.expectCompilerOutputs()
    Collector.only(coutputs.lookupFunction("main"), { case IfTE(_, _, _) => })

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_false_branch_returning_an_int
#[test]
fn simple_false_branch_returning_an_int() {
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
        "exported func main() int {\n  return if (false) { 3 } else { 5 };\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 5 }) => {}
        other => panic!("expected VonInt(5), got {:?}", other),
    }
}
/*
  test("Simple false branch returning an int") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  return if (false) { 3 } else { 5 };
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn ladder
#[test]
fn ladder() {
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
        "exported func main() int {\n  return if (false) { 3 } else if (true) { 5 } else { 7 };\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        let ifs: Vec<&crate::typing::ast::expressions::IfTE> = crate::collect_where_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::If(if2) => Some(if2)
        );
        for iff in &ifs {
            assert_eq!(iff.result().coord, crate::typing::types::types::CoordT {
                ownership: crate::typing::types::types::OwnershipT::Share,
                region: crate::typing::types::types::RegionT { region: crate::typing::types::types::IRegionT::Default },
                kind: crate::typing::types::types::KindT::Int(crate::typing::types::types::IntT::I32),
            });
        }
        assert_eq!(ifs.len(), 2);
        let user_funcs = coutputs.get_all_user_functions();
        for func in &user_funcs {
            match func.header.return_type {
                crate::typing::types::types::CoordT { ownership: crate::typing::types::types::OwnershipT::Share, kind: crate::typing::types::types::KindT::Int(crate::typing::types::types::IntT { bits: 32 }), .. } => {}
                crate::typing::types::types::CoordT { ownership: crate::typing::types::types::OwnershipT::Share, kind: crate::typing::types::types::KindT::Bool(_), .. } => {}
                other => panic!("vwat: {:?}", other),
            }
        }
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 5 }) => {}
        other => panic!("expected VonInt(5), got {:?}", other),
    }
}
/*
  test("Ladder") {
    val compile = RunCompilation.testNoBuiltins(
      """
        |exported func main() int {
        |  return if (false) { 3 } else if (true) { 5 } else { 7 };
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val ifs = Collector.all(coutputs.lookupFunction("main"), { case if2 @ IfTE(_, _, _) => if2 })
    ifs.foreach(iff => iff.result.coord shouldEqual CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32))
    ifs.size shouldEqual 2
    val userFuncs = coutputs.getAllUserFunctions
    userFuncs.foreach(func => {
      func.header.returnType match {
        case CoordT(ShareT, _, IntT.i32) =>
        case CoordT(ShareT, _, BoolT()) =>
        case other => vwat(other)
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn moving_from_inside_if
#[test]
fn moving_from_inside_if() {
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
        "\nstruct Marine { x int; }\nexported func main() int {\n  m = Marine(5);\n  return if (false) {\n      [x] = m;\n      x\n    } else {\n      [y] = m;\n      y\n    };\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        let ifs: Vec<&crate::typing::ast::expressions::IfTE> = crate::collect_where_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::If(if2) => Some(if2)
        );
        for iff in &ifs {
            assert_eq!(iff.result().coord, crate::typing::types::types::CoordT {
                ownership: crate::typing::types::types::OwnershipT::Share,
                region: crate::typing::types::types::RegionT { region: crate::typing::types::types::IRegionT::Default },
                kind: crate::typing::types::types::KindT::Int(crate::typing::types::types::IntT::I32),
            });
        }
        let user_funcs = coutputs.get_all_user_functions();
        for func in &user_funcs {
            match func.header.return_type {
                crate::typing::types::types::CoordT { ownership: crate::typing::types::types::OwnershipT::Share, kind: crate::typing::types::types::KindT::Int(crate::typing::types::types::IntT { bits: 32 }), .. } => {}
                crate::typing::types::types::CoordT { ownership: crate::typing::types::types::OwnershipT::Share, kind: crate::typing::types::types::KindT::Bool(_), .. } => {}
                other => panic!("vwat: {:?}", other),
            }
        }
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 5 }) => {}
        other => panic!("expected VonInt(5), got {:?}", other),
    }
}
/*
  test("Moving from inside if") {
    val compile = RunCompilation.testNoBuiltins(
      """
        |struct Marine { x int; }
        |exported func main() int {
        |  m = Marine(5);
        |  return if (false) {
        |      [x] = m;
        |      x
        |    } else {
        |      [y] = m;
        |      y
        |    };
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val ifs = Collector.all(coutputs.lookupFunction("main"), { case if2 @ IfTE(_, _, _) => if2 })
    ifs.foreach(iff => iff.result.coord shouldEqual CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32))
    val userFuncs = coutputs.getAllUserFunctions
    userFuncs.foreach(func => {
      func.header.returnType match {
        case CoordT(ShareT, _, IntT.i32) =>
        case CoordT(ShareT, _, BoolT()) =>
      }
    })

    compile.evalForKind(Vector()) match { case VonInt(5) => }
  }
*/
// mig: fn if_with_complex_condition
#[test]
fn if_with_complex_condition() {
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
        "struct Marine { x int; }\nexported func main() str {\n  m = Marine(5);\n  return if (m.x == 5) { \"#\" }\n  else if (0 == 0) { \"?\" }\n  else { \".\" };\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let main = coutputs.lookup_function_by_str("main");
        let ifs: Vec<&crate::typing::ast::expressions::IfTE> = crate::collect_where_tnode!(
            crate::typing::test::traverse::NodeRefT::FunctionDefinition(main),
            crate::typing::test::traverse::NodeRefT::If(if2) => Some(if2)
        );
        for iff in &ifs {
            assert_eq!(iff.result().coord, crate::typing::types::types::CoordT {
                ownership: crate::typing::types::types::OwnershipT::Share,
                region: crate::typing::types::types::RegionT { region: crate::typing::types::types::IRegionT::Default },
                kind: crate::typing::types::types::KindT::Str(crate::typing::types::types::StrT),
            });
        }
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value }) if value == "#" => {}
        other => panic!("expected VonStr(\"#\"), got {:?}", other),
    }
}
/*
  test("If with complex condition") {
    val compile = RunCompilation.test(
      """
        |struct Marine { x int; }
        |exported func main() str {
        |  m = Marine(5);
        |  return if (m.x == 5) { "#" }
        |  else if (0 == 0) { "?" }
        |  else { "." };
        |}
      """.stripMargin)

    val coutputs = compile.expectCompilerOutputs()
    val ifs = Collector.all(coutputs.lookupFunction("main"), { case if2 @ IfTE(_, _, _) => if2 })
    ifs.foreach(iff => iff.result.coord shouldEqual CoordT(ShareT, RegionT(DefaultRegionT), StrT()))

    compile.evalForKind(Vector()) match { case VonStr("#") => }
  }
*/
// mig: fn if_with_condition_declaration
#[test]
fn if_with_condition_declaration() {
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
        "exported func main() int {\n  return if x = 42; x < 50 { x }\n    else { 73 };\n}\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("If with condition declaration") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  return if x = 42; x < 50 { x }
        |    else { 73 };
        |}
      """.stripMargin)

    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn ret_from_inside_if_will_destroy_locals
#[test]
fn ret_from_inside_if_will_destroy_locals() {
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
        "import printutils.*;\n#!DeriveStructDrop\nstruct Marine { hp int; }\nfunc drop(marine Marine) void {\n  println(\"Destroying marine!\");\n  Marine[weapon] = marine;\n}\nexported func main() int {\n  m = Marine(5);\n  x =\n    if (true) {\n      println(\"In then!\");\n      return 7;\n    } else {\n      println(\"In else!\");\n      m.hp\n    };\n  println(\"In rest!\");\n  return x;\n}\n",
    );
    assert_eq!(compile.eval_for_stdout(Vec::new()), "In then!\nDestroying marine!\n");
}
/*
  test("Ret from inside if will destroy locals") {
    val compile = RunCompilation.test(
      """import printutils.*;
        |#!DeriveStructDrop
        |struct Marine { hp int; }
        |func drop(marine Marine) void {
        |  println("Destroying marine!");
        |  Marine[weapon] = marine;
        |}
        |exported func main() int {
        |  m = Marine(5);
        |  x =
        |    if (true) {
        |      println("In then!");
        |      return 7;
        |    } else {
        |      println("In else!");
        |      m.hp
        |    };
        |  println("In rest!");
        |  return x;
        |}
        |""".stripMargin)

    compile.evalForStdout(Vector()) shouldEqual "In then!\nDestroying marine!\n"
  }
*/
// mig: fn can_continue_if_other_branch_would_have_returned
#[test]
fn can_continue_if_other_branch_would_have_returned() {
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
        "import printutils.*;\n\n#!DeriveStructDrop\nstruct Marine { hp int; }\nfunc drop(marine Marine) void {\n  println(\"Destroying marine!\");\n  Marine[weapon] = marine;\n}\nexported func main() int {\n  m = Marine(5);\n  x =\n    if (false) {\n      println(\"In then!\");\n      return 7;\n    } else {\n      println(\"In else!\");\n      m.hp\n    };\n  println(\"In rest!\");\n  return x;\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let _main = coutputs.lookup_function_by_str("main");
    }
    assert_eq!(compile.eval_for_stdout(Vec::new()), "In else!\nIn rest!\nDestroying marine!\n");
}
/*
  test("Can continue if other branch would have returned") {
    val compile = RunCompilation.test(
      """
        |import printutils.*;
        |
        |#!DeriveStructDrop
        |struct Marine { hp int; }
        |func drop(marine Marine) void {
        |  println("Destroying marine!");
        |  Marine[weapon] = marine;
        |}
        |exported func main() int {
        |  m = Marine(5);
        |  x =
        |    if (false) {
        |      println("In then!");
        |      return 7;
        |    } else {
        |      println("In else!");
        |      m.hp
        |    };
        |  println("In rest!");
        |  return x;
        |}
        |""".stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    compile.evalForStdout(Vector()) shouldEqual "In else!\nIn rest!\nDestroying marine!\n"
  }
*/
// mig: fn destructure_inside_if
#[test]
fn destructure_inside_if() {
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
        "import printutils.*;\nstruct Bork {\n  num int;\n}\nstruct Moo {\n  bork Bork;\n}\n\nexported func main() {\n  zork = 0;\n  while (zork < 4) {\n    moo = Moo(Bork(5));\n    if (true) {\n      [bork] = moo;\n      println(bork.num);\n    } else {\n      drop(moo);\n    }\n    set zork = zork + 1;\n  }\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let _main = coutputs.lookup_function_by_str("main");
    }
    assert_eq!(compile.eval_for_stdout(Vec::new()), "5\n5\n5\n5\n");
}
/*
  test("Destructure inside if") {
    val compile = RunCompilation.test(
      """import printutils.*;
        |struct Bork {
        |  num int;
        |}
        |struct Moo {
        |  bork Bork;
        |}
        |
        |exported func main() {
        |  zork = 0;
        |  while (zork < 4) {
        |    moo = Moo(Bork(5));
        |    if (true) {
        |      [bork] = moo;
        |      println(bork.num);
        |    } else {
        |      drop(moo);
        |    }
        |    set zork = zork + 1;
        |  }
        |}
        |""".stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    compile.evalForStdout(Vector()) shouldEqual "5\n5\n5\n5\n"
  }
*/
// mig: fn if_nevers
#[test]
fn if_nevers() {
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
    let source = crate::tests::tests::load_expected("programs/if/ifnevers.vale");
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
  test("If nevers") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/if/ifnevers.vale"))
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn if_with_panics_and_rets
#[test]
fn if_with_panics_and_rets() {
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
        "exported func main() int {\n  a = 7;\n  if false {\n    panic(\"lol\");\n    return 73;\n  } else {\n    return 42;\n  }\n  return 73;\n}\n\n",
    );
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("If with panics and rets") {
    val compile =
      RunCompilation.test(
        """
          |exported func main() int {
          |  a = 7;
          |  if false {
          |    panic("lol");
          |    return 73;
          |  } else {
          |    return 42;
          |  }
          |  return 73;
          |}
          |
          |""".stripMargin)
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn toast
#[test]
fn toast() {
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
        "exported func main() int {\n  a = 0;\n  if (a == 2) {\n    return 71;\n  } else if (a == 5) {\n    return 73;\n  } else {\n    return 42;\n  }\n}\n",
    );
    {
        let coutputs = compile.expect_compiler_outputs();
        let _main = coutputs.lookup_function_by_str("main");
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 42 }) => {}
        other => panic!("expected VonInt(42), got {:?}", other),
    }
}
/*
  test("Toast") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  a = 0;
        |  if (a == 2) {
        |    return 71;
        |  } else if (a == 5) {
        |    return 73;
        |  } else {
        |    return 42;
        |  }
        |}
        |""".stripMargin)

    val main = compile.expectCompilerOutputs().lookupFunction("main")
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }

}

*/
