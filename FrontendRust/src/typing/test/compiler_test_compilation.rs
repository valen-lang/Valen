/*
package dev.vale.typing

import dev.vale.options.GlobalOptions
import dev.vale.parsing.ast.FileP
import dev.vale.{Builtins, FileCoordinateMap, Keywords, PackageCoordinate, Tests, _}
import dev.vale.highertyping._

import scala.collection.immutable.{List, ListMap, Map, Set}
import scala.collection.mutable

object CompilerTestCompilation {
*/
use bumpalo::Bump;
use crate::compile_options::GlobalOptions;
use crate::instantiating::instantiated_compilation::InstantiatorCompilationOptions;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::typing::compilation::TypingPassCompilation;
use crate::utils::code_hierarchy::{IPackageResolver, PackageCoordinate};
use std::collections::HashMap;
use std::sync::Arc;

// mig: fn test
pub fn compiler_test_compilation<'s, 'ctx, 't, 'p>(
    typing_interner: &'ctx crate::typing::typing_interner::TypingInterner<'s, 't>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    resolver: &'ctx dyn IPackageResolver<'p, HashMap<String, String>>,
) -> TypingPassCompilation<'s, 'ctx, 't, 'p>
where 's: 't,
{
    let test_module = parse_arena.intern_str("test");
    let test_tld = parse_arena.intern_package_coordinate(test_module, &[]);
    let global_options = GlobalOptions {
        sanity_check: true,
        use_overload_index: true,
        use_optimized_solver: true,
        verbose_errors: true,
        debug_output: true,
    };
    let instantiator_options = InstantiatorCompilationOptions {
        debug_out: Arc::new(|x: &str| println!("{}", x)),
    };
    TypingPassCompilation::new(
        typing_interner,
        scout_arena,
        keywords,
        parser_keywords,
        parse_arena,
        vec![test_tld],
        resolver,
        global_options,
        instantiator_options,
    )
}
/*
  def test(code: String, interner: Interner = new Interner()): TypingPassCompilation = {
    val keywords = new Keywords(interner)
    new TypingPassCompilation(
      interner,
      keywords,
      Vector(PackageCoordinate.TEST_TLD(interner, keywords)),
      Builtins.getModulizedCodeMap(interner, keywords)
        .or(FileCoordinateMap.test(interner, code))
        .or(Tests.getPackageToResourceResolver),
      TypingPassOptions(
        GlobalOptions(true, true, true, true, true),
        x => println(x),
        false))
  }
}
*/