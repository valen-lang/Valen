use crate::Keywords;
use crate::compile_options::GlobalOptions;
use crate::parse_arena::ParseArena;
use crate::pass_manager::CodeSource;
use crate::scout_arena::ScoutArena;
use crate::postparsing::ScoutCompilation;
use crate::utils::code_hierarchy::PackageCoordinate;



pub fn test<'s, 'ctx, 'p>(
  scout_arena: &'ctx ScoutArena<'s>,
  keywords: &'ctx Keywords<'s>,
  parser_keywords: &'ctx Keywords<'p>,
  parse_arena: &'ctx ParseArena<'p>,
  code_source: &'ctx CodeSource<'p>,
  code: &str,
) -> ScoutCompilation<'s, 'ctx, 'p>
where 'p: 's,
{
  let _ = code;
  let global_options = GlobalOptions {
    sanity_check: true,
    use_overload_index: true,
    use_optimized_solver: true,
    verbose_errors: false,
    debug_output: false,
  };
  let packages_to_build = vec![PackageCoordinate::test_tld(parse_arena, parser_keywords)];
  ScoutCompilation::new(
    scout_arena,
    keywords,
    parser_keywords,
    parse_arena,
    packages_to_build,
    code_source,
    global_options,
  )
}
