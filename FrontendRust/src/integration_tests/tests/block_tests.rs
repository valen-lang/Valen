/*
package dev.vale

import dev.vale.postparsing._
import dev.vale.postparsing._
import dev.vale.typing._
import dev.vale.typing.types.BoolT
import dev.vale.von.VonInt
import org.scalatest._

*/
// mig: struct BlockTests
pub struct BlockTests;
/*
class BlockTests extends FunSuite with Matchers {
*/
// mig: fn empty_block
#[test]
fn empty_block() {
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
        "exported func main() int {\n  block {\n  }\n  return 3;\n}\n",
    );
    {
        let test_str = scout_arena.intern_str("test");
        let package_coord = scout_arena.intern_package_coordinate(test_str, &[]);
        let file_coord = scout_arena.intern_file_coordinate(package_coord, "0.vale");
        let scoutput = compile.get_scoutput().expect("get_scoutput failed");
        let program_s = scoutput.file_coord_to_contents.get(file_coord).expect("file_coord not in scoutput");
        let main = program_s.lookup_function("main");
        match main.body {
            crate::postparsing::ast::IBodyS::CodeBody(crate::postparsing::ast::CodeBodyS {
                body: crate::postparsing::expressions::BodySE {
                    block: crate::postparsing::expressions::BlockSE {
                        expr: crate::postparsing::expressions::IExpressionSE::Consecutor(crate::postparsing::expressions::ConsecutorSE { exprs }),
                        ..
                    },
                    ..
                },
            }) if exprs.len() == 2 && matches!(exprs[0], crate::postparsing::expressions::IExpressionSE::Block(_)) => {}
            _ => panic!("expected CodeBody(BodySE(_, _, BlockSE(_, _, ConsecutorSE([BlockSE(_), _]))))"),
        }
    }
    match compile.eval_for_kind_primitive_args(Vec::new()) {
        crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: 3 }) => {}
        other => panic!("expected VonInt(3), got {:?}", other),
    }
}

/*
  test("Empty block") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  block {
        |  }
        |  return 3;
        |}
      """.stripMargin)
    val fileCoord =
      compile.interner.intern(FileCoordinate(
        compile.interner.intern(PackageCoordinate(
          compile.interner.intern(StrI("test")),
          Vector.empty)),
        "0.vale"))
    val scoutput = compile.getScoutput().getOrDie().fileCoordToContents(fileCoord)
    val main = scoutput.lookupFunction("main")
    main.body match { case CodeBodyS(BodySE(_, _,BlockSE(_, _,ConsecutorSE(Vector(BlockSE(_, _,_), _))))) => }

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_block_with_a_variable
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_block_with_a_variable() {
    panic!("Unmigrated test: simple_block_with_a_variable");
}

/*
  test("Simple block with a variable") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  block {
        |    y = 6;
        |  }
        |  return 3;
        |}
      """.stripMargin)
    val fileCoord =
      compile.interner.intern(FileCoordinate(
        compile.interner.intern(PackageCoordinate(
          compile.interner.intern(StrI("test")),
          Vector.empty)),
        "0.vale"))
    val scoutput = compile.getScoutput().getOrDie().fileCoordToContents(fileCoord)
    val main = scoutput.lookupFunction("main")
    val block = main.body match { case CodeBodyS(BodySE(_, _,BlockSE(_, _, ConsecutorSE(Vector(b @ BlockSE(_, _,_), _))))) => b }
    vassert(block.locals.size == 1)
    block.locals.head match {
      case LocalS(CodeVarNameS(StrI("y")), NotUsed, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
*/
// mig: fn simple_block_with_a_variable_another_variable_outside_with_same_name
#[test]
#[ignore = "unmigrated - pending integration-tests body migration"]
fn simple_block_with_a_variable_another_variable_outside_with_same_name() {
    panic!("Unmigrated test: simple_block_with_a_variable_another_variable_outside_with_same_name");
}

/*
  test("Simple block with a variable, another variable outside with same name") {
    val compile = RunCompilation.test(
      """
        |exported func main() int {
        |  block {
        |    y = 6;
        |  }
        |  y = 3;
        |  return y;
        |}
      """.stripMargin)
    val scoutput = compile.getScoutput().getOrDie()

    compile.evalForKind(Vector()) match { case VonInt(3) => }
  }
}

*/
