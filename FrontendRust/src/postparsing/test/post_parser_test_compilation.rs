use crate::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::postparsing::ScoutCompilation;

/*
package dev.vale.postparsing

import dev.vale.{FileCoordinateMap, Interner, Keywords, PackageCoordinate}
import dev.vale.options.GlobalOptions

object PostParserTestCompilation {
*/
// mig: fn test
pub fn test<'s, 'ctx, 'p>(
  scout_arena: &'ctx ScoutArena<'s>,
  keywords: &'ctx Keywords<'s>,
  parse_arena: &'ctx ParseArena<'p>,
  code: &str,
) -> ScoutCompilation<'s, 'ctx, 'p>
where 'p: 's,
{
  panic!("Unimplemented: test");
}
/*
  def test(code: String, interner: Interner = new Interner()): ScoutCompilation = {
    val keywords = new Keywords(interner)
    new ScoutCompilation(
      GlobalOptions(true, true, true, false, false),
      interner,
      keywords,
      Vector(PackageCoordinate.TEST_TLD(interner, keywords)),
      FileCoordinateMap.test(interner, Vector(code)))
  }
*/
/*
}
*/
