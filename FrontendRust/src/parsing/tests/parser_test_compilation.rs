use crate::compile_options::GlobalOptions;
use crate::interner::Interner;
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

/// MIGTODO: Check this is faithful to old Scala
/// Mirrors ParserTestCompilation.test in Scala.
pub fn test<'arena, 'i, 'k, 'b>(
  interner: &'i Interner<'arena>,
  keywords: &'k Keywords<'arena>,
  resolver: &'b dyn IPackageResolver<'arena, HashMap<String, String>>,
  test_package_coord: &'arena PackageCoordinate<'arena>,
) -> ParserCompilation<'arena, 'i, 'k, 'b>
where
  'i: 'arena,
  'k: 'arena,
  'b: 'arena,
{

  ParserCompilation::<'arena, 'i, 'k, 'b>::new(
    GlobalOptions {
      sanity_check: true,
      use_overload_index: true,
      use_optimized_solver: true,
      verbose_errors: true,
      debug_output: true,
    },
    interner,
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