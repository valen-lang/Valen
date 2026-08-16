use crate::compile_options::GlobalOptions;
use crate::instantiating::instantiated_compilation::InstantiatedCompilation;
use crate::instantiating::instantiated_compilation::InstantiatorCompilationOptions;
use crate::code_source::{CodeSource, Source};
use crate::tests::tests::test_source_from_dir;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::tests::tests::new_test_code_map;
use std::marker::PhantomData;
use std::sync::Arc;
use bumpalo::Bump;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::typing::typing_interner::TypingInterner;

pub fn test<'s, 'ctx, 't, 'i, 'p>(
    compilation_bump: &'ctx bumpalo::Bump,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    instantiating_bump: &'i bumpalo::Bump,
    code: &str,
) -> InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>
where 's: 't, 's: 'i, 'p: 'ctx,
{
    let packages_to_build: Vec<&'p PackageCoordinate<'p>> =
        vec![PackageCoordinate::test_tld(parse_arena, parser_keywords)];
    let code_source: &'ctx CodeSource<'p> = compilation_bump.alloc(CodeSource::new(vec![
        new_test_code_map(parse_arena, code),
        Source::Fn(test_source_from_dir),
    ]));
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
    InstantiatedCompilation::new(
        typing_interner,
        scout_arena,
        keywords,
        parser_keywords,
        parse_arena,
        packages_to_build,
        code_source,
        global_options,
        instantiator_options,
        instantiating_bump,
    )
}

/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct InstantiatedTests<'s, 't> {
  pub _marker: PhantomData<(&'s (), &'t ())>,
}


#[test]
fn test_templates() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r"
func drop(x int) { }
func bork<T>(a T) void where func drop(T)void {
  // implicitly calls drop
}
exported func main() {
  bork(3);
}
";
    let mut compile = test(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    compile.get_monouts();
}
#[test]
#[ignore = "share-blanket / bound-resolution not yet honest for clone-of-borrow-in-generics; needs `&&T` structural distinctness or primitive-borrow flip"]
fn nested_anonymous_substruct_captures_outer() {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);
    let code = r"
interface IF<R, P> {
  func __call(virtual this &IF<R, P>, p P) R;
}
exported func main() int {
  inner = IF<bool, int>((it) => { true });
  outer = IF<bool, int>((it) => { inner(it) });
  return 0;
}
";
    let mut compile = test(&compilation_bump, &typing_interner, &scout_arena, &keywords, &parser_keywords, &parse_arena, &instantiating_bump, code);
    compile.get_monouts();
}
