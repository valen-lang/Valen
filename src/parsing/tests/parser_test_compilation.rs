use crate::code_source::CodeSource;
use crate::compile_options::GlobalOptions;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::parsing::parser::ParserCompilation;
use crate::utils::code_hierarchy::PackageCoordinate;
pub fn test<'p, 'ctx>(
  parse_arena: &'ctx ParseArena<'p>,
  keywords: &'ctx Keywords<'p>,
  code_source: &'ctx CodeSource<'p>,
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
    code_source,
  )
}
