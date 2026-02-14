use crate::compile_options::GlobalOptions;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::parsing::parser::ParserCompilation;
use crate::utils::code_hierarchy::{FileCoordinate, FileCoordinateMap, IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use std::sync::Arc;

/*
package dev.vale.parsing

import dev.vale.options.GlobalOptions
import dev.vale.{FileCoordinateMap, IPackageResolver, Interner, Keywords, PackageCoordinate}

import scala.collection.immutable.Map

object ParserTestCompilation {
*/

/// MIGTODO: Check this is faithful to old Scala
/// Mirrors ParserTestCompilation.test in Scala.
pub fn test<'a>(
  interner: &Interner<'a>,
  keywords: &'a Keywords<'a>,
  code: &[String],
) -> ParserCompilation<'a, 'a> {
  let test_module = interner.intern("test");
  let test_package_coord = interner.intern_package_coordinate(PackageCoordinate {
    module: test_module,
    packages: vec![],
  });

  let mut code_map = FileCoordinateMap::new();
  for (index, contents) in code.iter().enumerate() {
    let filepath = if code.len() == 1 {
      "test.vale".to_string()
    } else {
      format!("{}.vale", index)
    };
    let file_coord = interner.intern_file_coordinate(FileCoordinate {
      package_coord: Arc::new(test_package_coord.clone()),
      filepath,
    });
    code_map.put(Arc::new(file_coord.clone()), contents.clone());
  }

  struct ParserTestResolver<'a> {
    code_map: FileCoordinateMap<'a, String>,
  }
  impl<'a> IPackageResolver<'a, HashMap<String, String>> for ParserTestResolver<'a> {
    fn resolve(&self, package_coord: &std::sync::Arc<PackageCoordinate<'a>>) -> Option<HashMap<String, String>> {
      // For testing the parser, we dont want it to fetch things with import statements.
      Some(
        self
          .code_map
          .resolve(package_coord)
          .unwrap_or_else(|| HashMap::from([("".to_string(), "".to_string())])),
      )
    }
  }

  let resolver: Box<dyn IPackageResolver<'a, HashMap<String, String>> + 'a> =
    Box::new(ParserTestResolver { code_map });
  ParserCompilation::new(
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