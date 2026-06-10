use bumpalo::Bump;
use crate::cast;
use crate::compile_options::GlobalOptions;
use crate::interner::StrI;
use crate::parsing::tests::utils::compile_file;
use crate::postparsing::ast::{IBodyS, ProgramS};
use crate::postparsing::expressions::{
  ConsecutorSE, FunctionCallSE, FunctionSE, IExpressionSE, IVariableUseCertainty, LetSE, LocalS,
  OwnershippedSE,
};
use crate::postparsing::names::IVarNameS;
use crate::postparsing::post_parser::{ICompileErrorS, PostParser};
use crate::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::postparsing::test::traverse::NodeRefS;
use crate::postparsing::post_parser::VariableNameAlreadyExists;
use crate::postparsing::expressions::BlockSE;
use crate::collect_only_snode;

/*
package dev.vale.postparsing

import dev.vale.{Collector, Err, FileCoordinateMap, Interner, Ok, SourceCodeUtils, StrI, vassert, vfail}
import dev.vale.options.GlobalOptions
import dev.vale.parsing.Parser
import org.scalatest._

import scala.runtime.Nothing$

class PostParserVariableTests extends FunSuite with Matchers {
*/
fn compile_for_error<'s, 'ctx, 'p>(
  scout_arena: &'ctx ScoutArena<'s>,
  keywords: &'ctx Keywords<'s>,
  parse_arena: &'ctx ParseArena<'p>,
  code: &str,
) -> ICompileErrorS<'s>
where 'p: 's,
{
  let options = GlobalOptions {
    sanity_check: true,
    use_overload_index: true,
    use_optimized_solver: true,
    verbose_errors: false,
    debug_output: false,
  };

  let keywords_p = Keywords::new_for_parse(parse_arena);
  let only_file = compile_file(parse_arena, &keywords_p, code).unwrap();
  // Re-intern FileCoordinate from 'p into 's
  let file_coord_s = scout_arena.intern_file_coordinate(
    scout_arena.intern_package_coordinate(
      scout_arena.intern_str(only_file.file_coord.package_coord.module.as_str()),
      &only_file.file_coord.package_coord.packages.iter().map(|s| scout_arena.intern_str(s.as_str())).collect::<Vec<_>>(),
    ),
    only_file.file_coord.filepath.as_str(),
  );
  let post_parser = PostParser::new(options, scout_arena, keywords, &keywords_p, parse_arena);
  match post_parser.scout_program(file_coord_s, &only_file) {
    Ok(_) => panic!("Accidentally compiled!"),
    Err(e) => e,
  }
}
/*
  private def compileForError(code: String): ICompileErrorS = {
    PostParserTestCompilation.test(code).getScoutput() match {
      case Err(e) => e
      case Ok(t) => vfail("Successfully compiled!\n" + t.toString)
    }
  }
*/
fn compile<'s, 'ctx, 'p>(
  scout_arena: &'ctx ScoutArena<'s>,
  keywords: &'ctx Keywords<'s>,
  parse_arena: &'ctx ParseArena<'p>,
  code: &str,
) -> ProgramS<'s>
where 'p: 's,
{
  let options = GlobalOptions {
    sanity_check: true,
    use_overload_index: true,
    use_optimized_solver: true,
    verbose_errors: false,
    debug_output: false,
  };

  let keywords_p = Keywords::new_for_parse(parse_arena);
  let only_file = compile_file(parse_arena, &keywords_p, code).unwrap();
  // Re-intern FileCoordinate from 'p into 's
  let file_coord_s = scout_arena.intern_file_coordinate(
    scout_arena.intern_package_coordinate(
      scout_arena.intern_str(only_file.file_coord.package_coord.module.as_str()),
      &only_file.file_coord.package_coord.packages.iter().map(|s| scout_arena.intern_str(s.as_str())).collect::<Vec<_>>(),
    ),
    only_file.file_coord.filepath.as_str(),
  );
  let post_parser = PostParser::new(options, scout_arena, keywords, &keywords_p, parse_arena);
  post_parser
    .scout_program(file_coord_s, &only_file)
    .unwrap()
}
/*
  private def compile(code: String): ProgramS = {
    val interner = new Interner()
    PostParserTestCompilation.test(code).getScoutput() match {
      case Err(e) => {
        val codeMap = FileCoordinateMap.test(interner, code)
        vfail(
          PostParserErrorHumanizer.humanize(
            SourceCodeUtils.humanizePos(codeMap, _),
            SourceCodeUtils.linesBetween(codeMap, _, _),
            SourceCodeUtils.lineRangeContaining(codeMap, _),
            SourceCodeUtils.lineContaining(codeMap, _),
            e))
      }
      case Ok(t) => t.expectOne()
    }
  }
*/
#[test]
fn regular_variable() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int { x = 4; }",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let locals = &code_body.body.block.locals;
  assert_eq!(locals.len(), 1);
  let local = &locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(StrI(\"x\")), NotUsed x6)"),
  }
}
/*
  test("Regular variable") {
    val program1 = compile("exported func main() int { x = 4; }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    vassert(body.block.locals.size == 1)
    body.block.locals.head match {
      case LocalS(
      CodeVarNameS(StrI("x")),
      NotUsed, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn typeless_local_has_no_coord_rune() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int { x = 4; }",
  );
  let main = program1.lookup_function("main");
  let local = collect_only_snode!(
    NodeRefS::Function(main),
    NodeRefS::Expression(IExpressionSE::Let(
      let_se @ LetSE { .. }
    )) => Some(let_se)
  );
  assert_eq!(local.pattern.coord_rune, None);
}
/*
  test("Type-less local has no coord rune") {
    val program1 = compile("exported func main() int { x = 4; }")
    val main = program1.lookupFunction("main")
    val local = Collector.only(main, { case let @ LetSE(_, rules, pattern, _) => let })
    local.pattern.coordRune shouldEqual None
  }
*/
#[test]
fn reports_defining_same_name_variable() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let err = compile_for_error(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() { x = 4; x = 5; }",
  );
  match &err {
    ICompileErrorS::VariableNameAlreadyExists(
      VariableNameAlreadyExists {
        name: IVarNameS::CodeVarName(StrI("x")),
        ..
      },
    ) => {}
    _ => panic!("expected VariableNameAlreadyExists(_, CodeVarName(\"x\")), got {:?}", err),
  }
}
/*
  test("Reports defining same-name variable") {
    compileForError("exported func main() { x = 4; x = 5; }") match {
      case VariableNameAlreadyExists(_, CodeVarNameS(StrI("x"))) =>
    }
  }
*/
#[test]
fn self_is_pointing_to_function() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int { x = 4; doBlarks(&x); }",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::Used,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), Used, NotUsed, ...)"),
  }
}
/*
  test("Self is pointing to function") {
    val program1 = compile("exported func main() int { x = 4; doBlarks(&x); }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
      CodeVarNameS(StrI("x")),
      Used, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn self_is_pointing_to_method() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int { x = 4; x.doBlarks(); }",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::Used,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), Used, NotUsed, ...)"),
  }
}
/*
  test("Self is pointing to method") {
    val program1 = compile("exported func main() int { x = 4; x.doBlarks(); }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
      CodeVarNameS(StrI("x")),
      Used, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn self_is_moving_to_function() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int { x = 4; doBlarks(x); }",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::Used,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), NotUsed, Used, ...)"),
  }
}
/*
  test("Self is moving to function") {
    val program1 = compile("exported func main() int { x = 4; doBlarks(x); }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
      CodeVarNameS(StrI("x")),
      NotUsed, Used, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn self_is_moving_to_method() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int { x = 4; (x).doBlarks(); }",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::Used,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), NotUsed, Used, ...)"),
  }
}
/*
  test("Self is moving to method") {
    val program1 = compile("exported func main() int { x = 4; (x).doBlarks(); }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
      CodeVarNameS(StrI("x")),
      NotUsed, Used, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn self_is_mutating_mutable() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int { x = 4; set x = 6; }",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::Used,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), NotUsed, NotUsed, Used, ...)"),
  }
}
/*
  test("Self is mutating mutable") {
    val program1 = compile("exported func main() int { x = 4; set x = 6; }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
      CodeVarNameS(StrI("x")),
       NotUsed, NotUsed, Used, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn self_is_moving_and_mutating_same_variable() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int { x = 4; set x = +(x, 1); }",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::Used,
      self_mutated: IVariableUseCertainty::Used,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), NotUsed, Used, Used, ...)"),
  }
}
/*
  test("Self is moving and mutating same variable") {
    val program1 = compile("exported func main() int { x = 4; set x = +(x, 1); }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
      CodeVarNameS(StrI("x")),
       NotUsed, Used, Used, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn child_is_pointing() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  ({ doBlarks(&x); })();
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::Used,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(..., child_borrowed: Used)"),
  }
}
/*
  test("Child is pointing") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  ({ doBlarks(&x); })();
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           NotUsed, NotUsed, NotUsed, Used, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn child_is_moving() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  ({ doBlarks(x); })();
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::Used,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(..., child_moved: Used)"),
  }
}
/*
  test("Child is moving") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  ({ doBlarks(x); })();
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           NotUsed, NotUsed, NotUsed, NotUsed, Used, NotUsed) =>
    }
  }
*/
#[test]
fn child_is_mutating() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  ({ set x = 9; })();
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::Used,
    } => {}
    _ => panic!("expected LocalS(..., child_mutated: Used)"),
  }
}
/*
  test("Child is mutating") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  ({ set x = 9; })();
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           NotUsed, NotUsed, NotUsed, NotUsed, NotUsed, Used) =>
    }
  }
*/
#[test]
fn self_maybe_pointing() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { doBlarks(&x); } else { }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::Used,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), Used, NotUsed, ...)"),
  }
}
/*
  test("Self maybe pointing") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { doBlarks(&x); } else { }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(CodeVarNameS(StrI("x")), Used, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn self_maybe_moving() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { doBlarks(x); } else { }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::Used,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), NotUsed, Used, ...)"),
  }
}
/*
  test("Self maybe moving") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { doBlarks(x); } else { }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           NotUsed, Used, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn self_maybe_mutating() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { set x = 9; } else { }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::Used,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), NotUsed, NotUsed, Used, ...)"),
  }
}
/*
  test("Self maybe mutating") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { set x = 9; } else { }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           NotUsed, NotUsed, Used, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn children_maybe_pointing() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { { doBlarks(&x); }(); } else { }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::Used,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(..., child_borrowed: Used)"),
  }
}
/*
  test("Children maybe pointing") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { { doBlarks(&x); }(); } else { }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
          NotUsed, NotUsed, NotUsed, Used, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn children_maybe_moving() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { { doBlarks(x); }(); } else { }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::Used,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(..., child_moved: Used)"),
  }
}
/*
  test("Children maybe moving") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { { doBlarks(x); }(); } else { }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
          NotUsed, NotUsed, NotUsed, NotUsed, Used, NotUsed) =>
    }
  }
*/
#[test]
fn children_maybe_mutating() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { { set x = 9; }(); } else { }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::Used,
    } => {}
    _ => panic!("expected LocalS(..., child_mutated: Used)"),
  }
}
/*
  test("Children maybe mutating") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { { set x = 9; }(); } else { }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
          NotUsed, NotUsed, NotUsed, NotUsed, NotUsed, Used) =>
    }
  }
*/
#[test]
fn self_both_pointing() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { doBoinks(&x); } else { doBloops(&x); }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::Used,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), Used, NotUsed, ...)"),
  }
}
/*
  test("Self both pointing") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { doBoinks(&x); } else { doBloops(&x); }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           Used, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn children_both_pointing() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { { doBoinks(&x); }(); } else { { doBloops(&x); }(); }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::Used,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(..., child_borrowed: Used)"),
  }
}
/*
  test("Children both pointing") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { { doBoinks(&x); }(); } else { { doBloops(&x); }(); }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
          NotUsed, NotUsed, NotUsed, Used, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn self_both_moving() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { doBoinks(x); } else { doBloops(x); }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::Used,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), NotUsed, Used, ...)"),
  }
}
/*
  test("Self both moving") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { doBoinks(x); } else { doBloops(x); }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           NotUsed, Used, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn children_both_moving() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { { doBoinks(x); }(); } else { { doBloops(x); }(); }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::Used,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(..., child_moved: Used)"),
  }
}
/*
  test("Children both moving") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { { doBoinks(x); }(); } else { { doBloops(x); }(); }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
          NotUsed, NotUsed, NotUsed, NotUsed, Used, NotUsed) =>
    }
  }
*/
#[test]
fn self_both_mutating() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { set x = 9; } else { set x = 8; }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::Used,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), NotUsed, NotUsed, Used, ...)"),
  }
}
/*
  test("Self both mutating") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { set x = 9; } else { set x = 8; }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           NotUsed, NotUsed, Used, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn children_both_mutating() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { { set x = 9; }(); } else { { set x = 8; }(); }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::Used,
    } => {}
    _ => panic!("expected LocalS(..., child_mutated: Used)"),
  }
}
/*
  test("Children both mutating") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { { set x = 9; }(); } else { { set x = 8; }(); }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           NotUsed, NotUsed, NotUsed, NotUsed, NotUsed, Used) =>
    }
  }
*/
#[test]
fn self_pointing_or_moving() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { doThings(&x); } else { moveThis(x); }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::Used,
      self_moved: IVariableUseCertainty::Used,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), Used, Used, NotUsed, ...)"),
  }
}
/*
  test("Self pointing or moving") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { doThings(&x); } else { moveThis(x); }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           Used, Used, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn children_pointing_or_moving() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { { doThings(&x); }(); } else { { moveThis(x); }(); }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::Used,
      child_moved: IVariableUseCertainty::Used,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(..., child_borrowed: Used, child_moved: Used)"),
  }
}
/*
  test("Children pointing or moving") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { { doThings(&x); }(); } else { { moveThis(x); }(); }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
          NotUsed, NotUsed, NotUsed, Used, Used, NotUsed) =>
    }
  }
*/
#[test]
fn self_mutating_or_moving() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { set x = 9; } else { moveThis(x); }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::Used,
      self_mutated: IVariableUseCertainty::Used,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), NotUsed, Used, Used, ...)"),
  }
}
/*
  test("Self mutating or moving") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { set x = 9; } else { moveThis(x); }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           NotUsed, Used, Used, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn children_mutating_or_moving() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { { set x = 9; }(); } else { { moveThis(x); }(); }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::Used,
      child_mutated: IVariableUseCertainty::Used,
    } => {}
    _ => panic!("expected LocalS(..., child_moved: Used, child_mutated: Used)"),
  }
}
/*
  test("Children mutating or moving") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { { set x = 9; }(); } else { { moveThis(x); }(); }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           NotUsed, NotUsed, NotUsed, NotUsed, Used, Used) =>
    }
  }
*/
#[test]
fn self_moving_and_mutating_same_variable() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int { x = 4; set x = +(x, 1); }",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::Used,
      self_mutated: IVariableUseCertainty::Used,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), NotUsed, Used, Used, ...)"),
  }
}
/*
  test("Self moving and mutating same variable") {
    val program1 = compile("exported func main() int { x = 4; set x = +(x, 1); }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           NotUsed, Used, Used, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn children_moving_and_mutating_same_variable() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int { x = 4; { set x = +(x, 1); }(); }",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::Used,
      child_mutated: IVariableUseCertainty::Used,
    } => {}
    _ => panic!("expected LocalS(..., child_moved: Used, child_mutated: Used)"),
  }
}
/*
  test("Children moving and mutating same variable") {
    val program1 = compile("exported func main() int { x = 4; { set x = +(x, 1); }(); }")
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           NotUsed, NotUsed, NotUsed, NotUsed, Used, Used) =>
    }
  }
*/
#[test]
fn self_borrowing_param() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "func main(x int) {
  print(&x);
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::Used,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), Used, NotUsed, ...)"),
  }
}
/*
  test("Self borrowing param") {
    val program1 = compile(
      """
        |func main(x int) {
        |  print(&x);
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           Used, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn children_borrowing_param() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "func main(x int) {
  { print(&x); }();
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::Used,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(..., child_borrowed: Used)"),
  }
}
/*
  test("Children borrowing param") {
    val program1 = compile(
      """
        |func main(x int) {
        |  { print(&x); }();
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
          NotUsed, NotUsed, NotUsed, Used, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn self_loading_or_mutating_or_moving() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { set x = 9; } else if (true) { moveThis(x); } else { blark(&x); }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::Used,
      self_moved: IVariableUseCertainty::Used,
      self_mutated: IVariableUseCertainty::Used,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), Used, Used, Used, ...)"),
  }
}
/*
  test("Self loading or mutating or moving") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { set x = 9; } else if (true) { moveThis(x); } else { blark(&x); }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           Used, Used, Used, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn children_loading_or_mutating_or_moving() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  x = 4;
  if (true) { { set x = 9; }(); } else if (true) { { moveThis(x); }(); } else { { blark(&x); }(); }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::Used,
      child_moved: IVariableUseCertainty::Used,
      child_mutated: IVariableUseCertainty::Used,
    } => {}
    _ => panic!("expected LocalS(..., child_*: Used)"),
  }
}
/*
  test("Children loading or mutating or moving") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = 4;
        |  if (true) { { set x = 9; }(); } else if (true) { { moveThis(x); }(); } else { { blark(&x); }(); }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           NotUsed, NotUsed, NotUsed, Used, Used, Used) =>
    }
  }
*/
#[test]
fn while_condition_borrowing() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "struct Marine {}
exported func main() int {
  x = Marine();
  while (&x) { }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::Used,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), Used, NotUsed, ...)"),
  }
}
/*
  test("While condition borrowing") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = Marine();
        |  while (&x) { }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    // x is always borrowed because the condition of a while is always run
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           Used, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
#[test]
fn while_body_maybe_loading() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "struct Marine {}
exported func main() int {
  x = Marine();
  while (true) { doThing(&x); }
}",
  );
  let main = program1.lookup_function("main");
  let code_body = cast!(&main.body, IBodyS::CodeBody);
  let local = &code_body.body.block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::CodeVarName(StrI("x")),
      self_borrowed: IVariableUseCertainty::Used,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(CodeVarNameS(\"x\"), Used, NotUsed, ...)"),
  }
}
/*
  test("While body maybe loading") {
    val program1 = compile(
      """
        |exported func main() int {
        |  x = Marine();
        |  while (true) { doThing(&x); }
        |}
      """.stripMargin)
    val main = program1.lookupFunction("main")
    val CodeBodyS(body) = main.body
    body.block.locals.head match {
      case LocalS(
          CodeVarNameS(StrI("x")),
           Used, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
fn extract_lambda_block_from_main<'s>(
  body: &'s IBodyS<'s>,
) -> &'s BlockSE<'s> {
  let code_body = cast!(body, IBodyS::CodeBody);
  let block = code_body.body.block;
  let exprs: &[&IExpressionSE] = match block.expr {
    IExpressionSE::Consecutor(ConsecutorSE { exprs }) => exprs,
    IExpressionSE::FunctionCall(fc) => return extract_block_from_lambda_call(fc),
    _ => panic!("expected ConsecutorSE or FunctionCall in block expr"),
  };
  for expr in exprs {
    if let IExpressionSE::FunctionCall(fc) = *expr {
      if let Some(lam_block) = try_extract_block_from_lambda_call(fc) {
        return lam_block;
      }
    }
  }
  panic!("no lambda call found in main body")
}

fn try_extract_block_from_lambda_call<'s>(
  fc: &FunctionCallSE<'s>,
) -> Option<&'s BlockSE<'s>> {
  let inner = match fc.callable_expr {
    IExpressionSE::Ownershipped(OwnershippedSE { inner_expr, .. }) => inner_expr,
    IExpressionSE::Function(func_se) => return extract_block_from_func_se(func_se),
    _ => return None,
  };
  match inner {
    IExpressionSE::Function(func_se) => extract_block_from_func_se(func_se),
    _ => None,
  }
}

fn extract_block_from_func_se<'s>(
  func_se: &FunctionSE<'s>,
) -> Option<&'s BlockSE<'s>> {
  let code_body = match &func_se.function.body {
    IBodyS::CodeBody(c) => c,
    _ => return None,
  };
  Some(code_body.body.block)
}

fn extract_block_from_lambda_call<'s>(
  fc: &FunctionCallSE<'s>,
) -> &'s BlockSE<'s> {
  try_extract_block_from_lambda_call(fc).expect("callable is not lambda")
}
#[test]
fn include_closure_var_in_locals() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "struct Marine {}
exported func main() int {
  m = Marine();
  { m.shout() }();
}",
  );
  let main = program1.lookup_function("main");
  let lam_block = extract_lambda_block_from_main(&main.body);
  let local = &lam_block.locals[0];
  match local {
    LocalS {
      var_name: IVarNameS::ClosureParamName(_),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(ClosureParamNameS(_), NotUsed x6)"),
  }
}
/*
  test("Include closure var in locals") {
    val program1 = compile(
      """
        |exported func main() int {
        |  m = Marine();
        |  { m.shout() }();
        |}
      """.stripMargin)
    val scoutput = program1
    val main = scoutput.lookupFunction("main")
    val CodeBodyS(BodySE(_, _, BlockSE(_, _, ConsecutorSE(exprs)))) = main.body
    // __Closure is shown as not used... we could change scout to automatically
    // borrow it whenever we try to access a closure variable?
    val lamBlock =
      exprs.collect({
        case FunctionCallSE(_, _, OwnershippedSE(_, FunctionSE(FunctionS(_, _, _, _, _, _, _, _, _, CodeBodyS(innerBody))), _), _) => innerBody.block
      }).head
    lamBlock.locals.head match {
      case LocalS(name, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed) => {
        name match {
          case ClosureParamNameS(_) =>
        }
      }
    }
  }
*/
#[test]
fn include_underscore_in_locals() {
  let parse_bump = Bump::new();
  let scout_bump = Bump::new();
  let parse_arena = ParseArena::new(&parse_bump);
  let scout_arena = ScoutArena::new(&scout_bump);
  let keywords = Keywords::new_for_scout(&scout_arena);
  let program1 = compile(
    &scout_arena,
    &keywords,
    &parse_arena,
    "exported func main() int {
  { print(_) }(3);
}",
  );
  let main = program1.lookup_function("main");
  let lam_block = extract_lambda_block_from_main(&main.body);
  let locals = &lam_block.locals;
  let closure_param = locals
    .iter()
    .find(|l| matches!(&l.var_name, IVarNameS::ClosureParamName(_)))
    .expect("no ClosureParamName local found");
  match closure_param {
    LocalS {
      var_name: IVarNameS::ClosureParamName(_),
      self_borrowed: IVariableUseCertainty::NotUsed,
      self_moved: IVariableUseCertainty::NotUsed,
      self_mutated: IVariableUseCertainty::NotUsed,
      child_borrowed: IVariableUseCertainty::NotUsed,
      child_moved: IVariableUseCertainty::NotUsed,
      child_mutated: IVariableUseCertainty::NotUsed,
    } => {}
    _ => panic!("expected LocalS(ClosureParamNameS(_), NotUsed x6)"),
  }
}
/*
  test("Include _ in locals") {
    val program1 = compile(
      """
        |exported func main() int {
        |  { print(_) }(3);
        |}
      """.stripMargin)
    val scoutput = program1
    val main = scoutput.lookupFunction("main")
    val CodeBodyS(BodySE(_, _, BlockSE(_, _, ConsecutorSE(exprs)))) = main.body
    // __Closure is shown as not used... we could change scout to automatically
    // borrow it whenever we try to access a closure variable?
    val lamBlock =
      exprs.collect({
        case FunctionCallSE(_, _, OwnershippedSE(_, FunctionSE(FunctionS(_, _, _, _, _, _, _, _, _, CodeBodyS(innerBody))), _), _) => innerBody.block
      }).head
    val locals = lamBlock.locals
    locals.find(_.varName match { case ClosureParamNameS(_) => true case _ => false }).get match {
      case LocalS(ClosureParamNameS(_), NotUsed, NotUsed, NotUsed, NotUsed, NotUsed, NotUsed) =>
    }
  }
*/
/*
}
*/