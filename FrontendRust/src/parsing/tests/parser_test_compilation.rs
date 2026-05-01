use crate::compile_options::GlobalOptions;
use crate::parse_arena::ParseArena;
use crate::keywords::Keywords;
use crate::parsing::parser::ParserCompilation;
use crate::utils::code_hierarchy::{IPackageResolver, PackageCoordinate};
use std::collections::HashMap;

/*
package dev.vale.parsing

import dev.vale.options.GlobalOptions
import dev.vale.{FileCoordinateMap, IPackageResolver, Interner, Keywords, PackageCoordinate}

import scala.collection.immutable.Map

object ParserTestCompilation {
*/

/// AFTERM: Check this is faithful to old Scala
/// Mirrors ParserTestCompilation.test in Scala.
pub fn test<'p, 'ctx>(
  parse_arena: &'ctx ParseArena<'p>,
  keywords: &'ctx Keywords<'p>,
  resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
  test_package_coord: &'p PackageCoordinate<'p>,
) -> ParserCompilation<'p, 'ctx>
where
  'p: 'ctx,
{
  ParserCompilation::new(
    GlobalOptions {
      sanity_check: true,
      use_overload_index: true,
      use_optimized_solver: true,
      verbose_errors: true,
      debug_output: true,
    },
    parse_arena,
    keywords,
    vec![test_package_coord],
    resolver,
  )
}
/*
  def test(interner: Interner, keywords: Keywords, code: String*): ParserCompilation = {
    val codeMap = FileCoordinateMap.test(interner, code.toVector)
    new ParserCompilation(
      GlobalOptions(true, true, true, true, true),
      interner,
      keywords,
      Vector(PackageCoordinate.TEST_TLD(interner, keywords)),
      new IPackageResolver[Map[String, String]]() {
        override def resolve(packageCoord: PackageCoordinate): Option[Map[String, String]] = {
          // For testing the parser, we dont want it to fetch things with import statements
          Some(codeMap.resolve(packageCoord).getOrElse(Map("" -> "")))
        }
      })

  }
*/

/*
}
*/