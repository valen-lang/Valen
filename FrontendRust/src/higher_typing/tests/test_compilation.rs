use crate::compile_options::GlobalOptions;
use crate::higher_typing::higher_typing_pass::HigherTypingCompilation;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::pass_manager::{CodeSource, Source};
use crate::scout_arena::ScoutArena;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::tests::tests::test_source_from_dir;
use crate::tests::tests::new_test_code_map;


pub fn test<'s, 'ctx, 'p>(
    compilation_bump: &'ctx bumpalo::Bump,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    code: &str,
) -> HigherTypingCompilation<'s, 'ctx, 'p> {
    let packages_to_build: Vec<&'p PackageCoordinate<'p>> =
        vec![PackageCoordinate::test_tld(parse_arena, parser_keywords)];
    let code_source: &'ctx CodeSource<'p> = compilation_bump.alloc(CodeSource::new(vec![
        new_test_code_map(parse_arena, code),
        Source::Fn(test_source_from_dir),
    ]));
    HigherTypingCompilation::new(
        scout_arena,
        keywords,
        parser_keywords,
        parse_arena,
        packages_to_build,
        code_source,
        GlobalOptions::test(),
    )
}
