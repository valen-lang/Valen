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
        let ret = match main.body {
            crate::postparsing::ast::IBodyS::CodeBody(crate::postparsing::ast::CodeBodyS {
                body: crate::postparsing::expressions::BodySE { block: crate::postparsing::expressions::BlockSE { expr, .. }, .. },
            }) => {
                let mut returns: Vec<&crate::postparsing::expressions::ReturnSE> = Vec::new();
                let mut stack: Vec<&crate::postparsing::expressions::IExpressionSE> = vec![*expr];
                while let Some(e) = stack.pop() {
                    match e {
                        crate::postparsing::expressions::IExpressionSE::Return(r) => returns.push(r),
                        crate::postparsing::expressions::IExpressionSE::Consecutor(c) => {
                            for child in c.exprs.iter() { stack.push(child); }
                        }
                        _ => {}
                    }
                }
                assert_eq!(returns.len(), 1);
                returns[0]
            }
            _ => panic!("expected CodeBody"),
        };
        let iff = {
            let mut found: Option<&crate::postparsing::expressions::IfSE> = None;
            let mut stack: Vec<&crate::postparsing::expressions::IExpressionSE> = vec![ret.inner];
            while let Some(e) = stack.pop() {
                match e {
                    crate::postparsing::expressions::IExpressionSE::If(i) => { assert!(found.is_none()); found = Some(i); }
                    crate::postparsing::expressions::IExpressionSE::Block(b) => stack.push(b.expr),
                    crate::postparsing::expressions::IExpressionSE::Consecutor(c) => {
                        for child in c.exprs.iter() { stack.push(child); }
                    }
                    _ => {}
                }
            }
            found.expect("expected If somewhere in ret.inner")
        };
        match iff.condition {
            crate::postparsing::expressions::IExpressionSE::ConstantBool(crate::postparsing::expressions::ConstantBoolSE { value: true, .. }) => {}
            _ => panic!("expected condition ConstantBool(_, true)"),
        }
        match iff.then_body.expr {
            crate::postparsing::expressions::IExpressionSE::ConstantInt(crate::postparsing::expressions::ConstantIntSE { value: 3, .. }) => {}
            _ => panic!("expected thenBody ConstantInt(_, 3, _)"),
        }
        match iff.else_body.expr {
            crate::postparsing::expressions::IExpressionSE::ConstantInt(crate::postparsing::expressions::ConstantIntSE { value: 5, .. }) => {}
            _ => panic!("expected elseBody ConstantInt(_, 5, _)"),
        }
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
Guardian: temp-disable: SPDMX — No collect_only_snode macro exists in this codebase (only collect_only_tnode for typing AST + collect_only_hnode for final AST). The postparsing AST has no traverse-macro yet, so inline destructure is the established Rust adaptation. In-file precedent: empty_block / simple_block_with_a_variable in block_tests.rs both walk CodeBody->BodySE->BlockSE->ConsecutorSE inline. — FrontendRust/guardian-logs/request-546-1780524601782/hook-546/IfTests--16.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
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
#[ignore = "unmigrated - pending integration-tests body migration"]
fn ladder() {
    panic!("Unmigrated test: ladder");
}
/*
  test("Ladder") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  return if (false) { 3 } else if (true) { 5 } else { 7 };
        |}
      """.stripMargin, false)

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
#[ignore = "unmigrated - pending integration-tests body migration"]
fn moving_from_inside_if() {
    panic!("Unmigrated test: moving_from_inside_if");
}
/*
  test("Moving from inside if") {
    val compile = RunCompilation.test(
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
      """.stripMargin, false)

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
#[ignore = "unmigrated - pending integration-tests body migration"]
fn if_with_complex_condition() {
    panic!("Unmigrated test: if_with_complex_condition");
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
#[ignore = "unmigrated - pending integration-tests body migration"]
fn if_with_condition_declaration() {
    panic!("Unmigrated test: if_with_condition_declaration");
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
#[ignore = "unmigrated - pending integration-tests body migration"]
fn ret_from_inside_if_will_destroy_locals() {
    panic!("Unmigrated test: ret_from_inside_if_will_destroy_locals");
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
#[ignore = "unmigrated - pending integration-tests body migration"]
fn can_continue_if_other_branch_would_have_returned() {
    panic!("Unmigrated test: can_continue_if_other_branch_would_have_returned");
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
#[ignore = "unmigrated - pending integration-tests body migration"]
fn destructure_inside_if() {
    panic!("Unmigrated test: destructure_inside_if");
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
#[ignore = "unmigrated - pending integration-tests body migration"]
fn if_nevers() {
    panic!("Unmigrated test: if_nevers");
}
/*
  test("If nevers") {
    val compile = RunCompilation.test(Tests.loadExpected("programs/if/ifnevers.vale"))
    compile.evalForKind(Vector()) match { case VonInt(42) => }
  }
*/
// mig: fn if_with_panics_and_rets
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn if_with_panics_and_rets() {
    panic!("Unmigrated test: if_with_panics_and_rets");
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
#[ignore = "unmigrated - pending integration-tests body migration"]
fn toast() {
    panic!("Unmigrated test: toast");
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
