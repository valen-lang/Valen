use crate::compile_options::GlobalOptions;
use crate::higher_typing::higher_typing_pass::HigherTypingCompilation;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::utils::code_hierarchy::PackageCoordinate;

/*
package dev.vale.highertyping

import dev.vale.{FileCoordinateMap, Keywords, PackageCoordinate, Tests, _}
import dev.vale.options.GlobalOptions

object HigherTypingTestCompilation {
*/
// mig: fn test
pub fn test<'s, 'ctx, 'p>(
    compilation_bump: &'ctx bumpalo::Bump,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    code: &str,
) -> HigherTypingCompilation<'s, 'ctx, 'p> {
    use crate::utils::code_hierarchy::IPackageResolver;
    let packages_to_build: Vec<&'p PackageCoordinate<'p>> =
        vec![PackageCoordinate::test_tld(parse_arena, parser_keywords)];
    let resolver_concrete =
        crate::utils::code_hierarchy::test_from_vec(parse_arena, vec![code.to_string()])
            .or(crate::tests::tests::get_package_to_resource_resolver());
    let resolver: &'ctx dyn crate::utils::code_hierarchy::IPackageResolver<'p, std::collections::HashMap<String, String>> =
        compilation_bump.alloc(resolver_concrete);
    HigherTypingCompilation::new(
        scout_arena,
        keywords,
        parser_keywords,
        parse_arena,
        packages_to_build,
        resolver,
        GlobalOptions::test(),
    )
}
/*
  def test(code: String*): HigherTypingCompilation = {
    val interner = new Interner()
    val keywords = new Keywords(interner)
    new HigherTypingCompilation(
      GlobalOptions.test(),
      interner,
      keywords,
      Vector(PackageCoordinate.TEST_TLD(interner, keywords)),
      FileCoordinateMap.test(interner, code.toVector)
        .or(Tests.getPackageToResourceResolver))
  }
}
*/
