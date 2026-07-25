
use bumpalo::Bump;
use crate::compile_options::GlobalOptions;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::code_source::CodeSource;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::typing::compilation::TypingPassCompilation;
use std::sync::Arc;
use crate::typing::typing_interner::TypingInterner;
use crate::typing::TypingPassOptions;
use crate::typing::oracles::Oracles;
#[cfg(feature = "rust_interop")]
use crate::typing::rust_interop::RustOracle;

fn test_typing_pass_options() -> TypingPassOptions {
    let global_options = GlobalOptions {
        sanity_check: true,
        use_overload_index: true,
        use_optimized_solver: true,
        verbose_errors: true,
        debug_output: true,
    };
    TypingPassOptions {
        global_options,
        debug_out: Arc::new(|x: &str| println!("{}", x)),
        tree_shaking_enabled: true,
    }
}

/// `TypingPassCompilation::new` for tests that don't exercise Rust interop — the same
/// arguments, with an oracle that knows nothing supplied for you.
///
/// Tests reach for this rather than the constructor so that a test about Vale semantics
/// never has to mention the build mode. Production still states its oracle explicitly.
pub fn typing_pass_compilation_for_test<'s, 'ctx, 't, 'p>(
    typing_interner: &'ctx TypingInterner<'s, 't>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    packages_to_build: Vec<&'p PackageCoordinate<'p>>,
    code_source: &'ctx CodeSource<'p>,
    typing_options: TypingPassOptions,
) -> TypingPassCompilation<'s, 'ctx, 't, 'p>
where 's: 't,
{
    TypingPassCompilation::new(
        typing_interner,
        scout_arena,
        keywords,
        parser_keywords,
        parse_arena,
        packages_to_build,
        code_source,
        typing_options,
        Oracles::none(),
    )
}

pub fn compiler_test_compilation<'s, 'ctx, 't, 'p>(
    typing_interner: &'ctx TypingInterner<'s, 't>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    code_source: &'ctx CodeSource<'p>,
) -> TypingPassCompilation<'s, 'ctx, 't, 'p>
where 's: 't,
{
    let test_module = parse_arena.intern_str("test");
    let test_tld = parse_arena.intern_package_coordinate(test_module, &[]);
    typing_pass_compilation_for_test(
        typing_interner,
        scout_arena,
        keywords,
        parser_keywords,
        parse_arena,
        vec![test_tld],
        code_source,
        test_typing_pass_options(),
    )
}

/// Same as `compiler_test_compilation`, but with a caller-supplied Rust oracle, for
/// tests that exercise the interop seam against a fixture.
#[cfg(feature = "rust_interop")]
pub fn compiler_test_compilation_with_rust_oracle<'s, 'ctx, 't, 'p>(
    typing_interner: &'ctx TypingInterner<'s, 't>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    code_source: &'ctx CodeSource<'p>,
    rust_oracle: &'ctx dyn RustOracle<'s, 't>,
) -> TypingPassCompilation<'s, 'ctx, 't, 'p>
where 's: 't,
{
    let test_module = parse_arena.intern_str("test");
    let test_tld = parse_arena.intern_package_coordinate(test_module, &[]);
    TypingPassCompilation::new(
        typing_interner,
        scout_arena,
        keywords,
        parser_keywords,
        parse_arena,
        vec![test_tld],
        code_source,
        test_typing_pass_options(),
        Oracles::with_rust(rust_oracle),
    )
}
