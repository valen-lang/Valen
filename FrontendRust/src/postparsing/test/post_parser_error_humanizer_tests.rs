use bumpalo::Bump;
use crate::postparsing::ast::ProgramS;
use crate::postparsing::post_parser::{
  ExternHasBodyS, ICompileErrorS, InterfaceMethodNeedsSelf, VariableNameAlreadyExists,
};
use crate::postparsing::post_parser_error_humanizer::humanize;
use crate::postparsing::names::IVarNameS;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::utils::range::RangeS;
use crate::utils::source_code_utils::{
  humanize_pos_code_map, line_containing, line_range_containing, lines_between,
};
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::Keywords;

/*
package dev.vale.postparsing

import dev.vale.{CodeLocationS, Err, FileCoordinateMap, Interner, Ok, RangeS, SourceCodeUtils, StrI, vassert, vfail}
import dev.vale.options.GlobalOptions
import dev.vale.parsing._
import dev.vale.postparsing.rules._
import org.scalatest._

class PostParserErrorHumanizerTests extends FunSuite with Matchers {
*/
fn compile<'s, 'ctx, 'p>(
  scout_arena: &'ctx ScoutArena<'s>,
  keywords: &'ctx Keywords<'s>,
  parse_arena: &'ctx ParseArena<'p>,
  code: &str,
) -> ProgramS<'s>
{
  panic!("Unimplemented: compile");
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
fn compile_for_error<'s, 'ctx, 'p>(
  scout_arena: &'ctx ScoutArena<'s>,
  keywords: &'ctx Keywords<'s>,
  parse_arena: &'ctx ParseArena<'p>,
  code: &str,
) -> ICompileErrorS<'s>
{
  panic!("Unimplemented: compile_for_error");
}
/*
  private def compileForError(code: String): ICompileErrorS = {
    PostParserTestCompilation.test(code).getScoutput() match {
      case Err(e) => e
      case Ok(t) => vfail("Successfully compiled!\n" + t.toString)
  }
}
*/
#[test]
fn humanize_errors() {
  let scout_bump = Bump::new();
  let scout_arena = ScoutArena::new(&scout_bump);
  let code_map = FileCoordinateMap::<'_, String>::test(&scout_arena, "blah blah blah\nblah blah blah".to_string());
  let tz = RangeS::test_zero(&scout_arena);

  let humanize_pos = |x: &_| humanize_pos_code_map(&code_map, x);
  let lines_between_fn = |x: &_, y: &_| lines_between(&code_map, x, y);
  let line_range_containing_fn = |x: &_| line_range_containing(&code_map, x);
  let line_containing_fn = |x: &_| line_containing(&code_map, x);

  let err1 = ICompileErrorS::VariableNameAlreadyExists(VariableNameAlreadyExists {
    range: tz.clone(),
    name: IVarNameS::CodeVarName(scout_arena.intern_str("Spaceship")),
  });
  assert!(!humanize(humanize_pos, lines_between_fn, line_range_containing_fn, line_containing_fn, &err1).is_empty());

  let err2 = ICompileErrorS::InterfaceMethodNeedsSelf(InterfaceMethodNeedsSelf { range: tz.clone() });
  assert!(!humanize(humanize_pos, lines_between_fn, line_range_containing_fn, line_containing_fn, &err2).is_empty());

  let err3 = ICompileErrorS::ExternHasBodyS(ExternHasBodyS { range: tz.clone() });
  assert!(!humanize(humanize_pos, lines_between_fn, line_range_containing_fn, line_containing_fn, &err3).is_empty());
}
/*
  test("Humanize errors") {
    val interner = new Interner()
    val codeMap = FileCoordinateMap.test(interner, "blah blah blah\nblah blah blah")
    val tz = RangeS.testZero(interner)

    val humanizePos = (x: CodeLocationS) => SourceCodeUtils.humanizePos(codeMap, x)
    val linesBetween = (x: CodeLocationS, y: CodeLocationS) => SourceCodeUtils.linesBetween(codeMap, x, y)
    val lineRangeContaining = (x: CodeLocationS) => SourceCodeUtils.lineRangeContaining(codeMap, x)
    val lineContaining = (x: CodeLocationS) => SourceCodeUtils.lineContaining(codeMap, x)

    vassert(PostParserErrorHumanizer.humanize(humanizePos, linesBetween, lineRangeContaining, lineContaining,
      VariableNameAlreadyExists(tz, CodeVarNameS(interner.intern(StrI("Spaceship")))))
      .nonEmpty)
    vassert(PostParserErrorHumanizer.humanize(humanizePos, linesBetween, lineRangeContaining, lineContaining,
      InterfaceMethodNeedsSelf(tz))
      .nonEmpty)
    vassert(PostParserErrorHumanizer.humanize(humanizePos, linesBetween, lineRangeContaining, lineContaining,
      ForgotSetKeywordError(tz))
      .nonEmpty)
    vassert(PostParserErrorHumanizer.humanize(humanizePos, linesBetween, lineRangeContaining, lineContaining,
      ExternHasBody(tz))
      .nonEmpty)
  }
}
*/