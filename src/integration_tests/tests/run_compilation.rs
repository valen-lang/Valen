//! Dark-box integration harness on the onion path. `test`/`test_no_builtins` compile a real `.vale`
//! program through `InstantiatedCompilation` to a `HinputsI`; `RunCompilation` then runs `main` in
//! the TestVM and returns the computed VON. Replaces the deleted Hammer/`ProgramH` path (mirrors
//! `testvm::test::vivem_tests::run_vale`).

use std::cell::RefCell;
use std::io::stdout;
use std::sync::Arc;
use bumpalo::Bump;
use crate::code_source::{CodeSource, Source};
use crate::compile_options::GlobalOptions;
use crate::interner::StrI;
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::instantiated_compilation::{InstantiatedCompilation, InstantiatorCompilationOptions};
use crate::postparsing::ast::ProgramS;
use crate::postparsing::post_parser::ICompileErrorS;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::hinputs_t::HinputsT;
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::tests::tests::{new_test_code_map, test_source_from_dir};
use crate::testvm::values::PrimitiveKindV;
use crate::testvm::vivem::{empty_stdin, execute_with_primitive_args, regular_stdout, VmRuntimeErrorV};
use crate::testvm::von::IVonData;
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::PackageCoordinate;

fn global_options() -> GlobalOptions {
    GlobalOptions {
        sanity_check: true,
        use_overload_index: true,
        use_optimized_solver: true,
        verbose_errors: true,
        debug_output: true,
    }
}

fn instantiator_options() -> InstantiatorCompilationOptions {
    InstantiatorCompilationOptions { debug_out: Arc::new(|x: &str| println!("{}", x)) }
}

/// Compile `code` with the builtins available in the root package — the "old way", where
/// the program can use `+`, `/`, `drop`, etc. without importing them. Mirrors the pre-onion
/// `test(...)` harness that the onion re-link deleted.
pub fn test<'s, 'ctx, 't, 'i, 'p>(
    compilation_bump: &'ctx Bump,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    instantiating_bump: &'i Bump,
    code: &str,
) -> RunCompilation<'s, 'ctx, 't, 'i, 'p>
where 's: 't, 's: 'i, 'p: 'ctx,
{
    build(
        compilation_bump, typing_interner, scout_arena, keywords, parser_keywords, parse_arena,
        instantiating_bump, code, true, true,
    )
}

/// Like `test` (builtins loaded), but with the group borrow checker disabled — for a test whose
/// program hits a not-yet-supported onion borrow-checker case (the borrow-group annotation gap) and
/// needs to reach the later passes it actually exercises.
/// VCOORD: delete this once the onion borrow checker handles these cases.
pub fn test_without_borrow_check<'s, 'ctx, 't, 'i, 'p>(
    compilation_bump: &'ctx Bump,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    instantiating_bump: &'i Bump,
    code: &str,
) -> RunCompilation<'s, 'ctx, 't, 'i, 'p>
where 's: 't, 's: 'i, 'p: 'ctx,
{
    build(
        compilation_bump, typing_interner, scout_arena, keywords, parser_keywords, parse_arena,
        instantiating_bump, code, true, false,
    )
}

/// Compile `code` alone (no builtins) — the program must stand on its own.
pub fn test_no_builtins<'s, 'ctx, 't, 'i, 'p>(
    compilation_bump: &'ctx Bump,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    instantiating_bump: &'i Bump,
    code: &str,
) -> RunCompilation<'s, 'ctx, 't, 'i, 'p>
where 's: 't, 's: 'i, 'p: 'ctx,
{
    build(
        compilation_bump, typing_interner, scout_arena, keywords, parser_keywords, parse_arena,
        instantiating_bump, code, false, true,
    )
}

/// Like `test_no_builtins`, but with the group borrow checker disabled — for a test that
/// deliberately exercises a later pass past a not-yet-supported borrow-checker case.
/// VCOORD: delete this
pub fn test_no_builtins_without_borrow_check<'s, 'ctx, 't, 'i, 'p>(
    compilation_bump: &'ctx Bump,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    instantiating_bump: &'i Bump,
    code: &str,
) -> RunCompilation<'s, 'ctx, 't, 'i, 'p>
where 's: 't, 's: 'i, 'p: 'ctx,
{
    build(
        compilation_bump, typing_interner, scout_arena, keywords, parser_keywords, parse_arena,
        instantiating_bump, code, false, false,
    )
}

fn build<'s, 'ctx, 't, 'i, 'p>(
    compilation_bump: &'ctx Bump,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    parser_keywords: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    instantiating_bump: &'i Bump,
    code: &str,
    include_builtins: bool,
    borrow_checker_enabled: bool,
) -> RunCompilation<'s, 'ctx, 't, 'i, 'p>
where 's: 't, 's: 'i, 'p: 'ctx,
{
    let mut packages_to_build: Vec<&'p PackageCoordinate<'p>> = Vec::new();
    let mut sources: Vec<Source<'p>> = Vec::new();
    if include_builtins {
        // The "old way": `Source::builtins` puts all builtin code in the root ("") package
        // (with empty `v.builtins.<X>` stubs so any `import` still resolves), so the test
        // program can use `+`, `/`, `drop`, etc. without importing anything.
        packages_to_build.push(PackageCoordinate::builtin(parse_arena, parser_keywords));
        sources.push(Source::builtins(parse_arena, parser_keywords));
    }
    packages_to_build.push(PackageCoordinate::test_tld(parse_arena, parser_keywords));
    sources.push(new_test_code_map(parse_arena, code));
    sources.push(Source::Fn(test_source_from_dir));
    let code_source: &'ctx CodeSource<'p> = compilation_bump.alloc(CodeSource::new(sources));
    let compilation = InstantiatedCompilation::new(
        typing_interner, scout_arena, keywords, parser_keywords, parse_arena,
        packages_to_build, code_source, global_options(), instantiator_options(),
        borrow_checker_enabled, instantiating_bump,
    );
    RunCompilation { compilation, scout_arena }
}

pub struct RunCompilation<'s, 'ctx, 't, 'i, 'p>
where 's: 't, 's: 'i,
{
    pub compilation: InstantiatedCompilation<'s, 'ctx, 't, 'i, 'p>,
    pub scout_arena: &'ctx ScoutArena<'s>,
}

impl<'s, 'ctx, 't, 'i, 'p> RunCompilation<'s, 'ctx, 't, 'i, 'p>
where 's: 't, 's: 'i,
{
    /// Drive the instantiator, yielding the monomorphized `HinputsI`.
    pub fn get_monouts(&mut self) -> &HinputsI<'s, 'i> {
        self.compilation.get_monouts()
    }

    /// Drive the typing pass and return its outputs (the typed AST) for structural assertions.
    /// Panics if compilation errored.
    pub fn expect_compiler_outputs(&mut self) -> &HinputsT<'s, 't> {
        self.compilation.expect_compiler_outputs()
    }

    /// Drive the typing pass, returning either its outputs or the typing-pass compile error.
    pub fn get_compiler_outputs(&mut self) -> Result<&HinputsT<'s, 't>, ICompileErrorT<'s, 't>> {
        self.compilation.get_compiler_outputs()
    }

    /// Drive the scout pass and return its per-file output for structural assertions.
    pub fn get_scoutput(&mut self) -> Result<&FileCoordinateMap<'s, ProgramS<'s>>, ICompileErrorS<'s>> {
        self.compilation.get_scoutput()
    }

    /// Run `main` in the TestVM with primitive args, discarding the return value (for void-`main` tests).
    pub fn run_primitive_args<'v>(
        &mut self,
        args: Vec<PrimitiveKindV<'v, 'i, 's>>,
    ) -> Result<(), VmRuntimeErrorV<'s>> {
        self.eval_for_kind_primitive_args(args).map(|_| ())
    }

    /// Run `main` in the TestVM and return everything it printed to stdout as a `String`.
    pub fn eval_for_stdout<'v>(
        &mut self,
        args: Vec<PrimitiveKindV<'v, 'i, 's>>,
    ) -> Result<String, VmRuntimeErrorV<'s>> {
        self.compilation.get_monouts();
        let program_h = self.compilation.cached_monouts();
        let interner = &self.compilation.instantiating_interner;
        let mut vivem_dout = stdout();
        let vivem_bump = Bump::new();
        let captured = RefCell::new(String::new());
        let capture_stdout = |s: StrI<'s>| {
            captured.borrow_mut().push_str(s.0);
        };
        execute_with_primitive_args(
            program_h,
            interner,
            self.scout_arena,
            &args,
            &mut vivem_dout,
            &vivem_bump,
            &empty_stdin,
            &capture_stdout,
        )?;
        Ok(captured.take())
    }

    /// Run `main` in the TestVM, returning both its VON result and everything it printed to stdout.
    pub fn eval_for_kind_and_stdout<'v>(
        &mut self,
        args: Vec<PrimitiveKindV<'v, 'i, 's>>,
    ) -> Result<(IVonData, String), VmRuntimeErrorV<'s>> {
        self.compilation.get_monouts();
        let program_h = self.compilation.cached_monouts();
        let interner = &self.compilation.instantiating_interner;
        let mut vivem_dout = stdout();
        let vivem_bump = Bump::new();
        let captured = RefCell::new(String::new());
        let capture_stdout = |s: StrI<'s>| {
            captured.borrow_mut().push_str(s.0);
        };
        let von = execute_with_primitive_args(
            program_h,
            interner,
            self.scout_arena,
            &args,
            &mut vivem_dout,
            &vivem_bump,
            &empty_stdin,
            &capture_stdout,
        )?;
        Ok((von, captured.take()))
    }

    /// Compile through the instantiator, run `main` in the TestVM with primitive args, return the VON.
    pub fn eval_for_kind_primitive_args<'v>(
        &mut self,
        args: Vec<PrimitiveKindV<'v, 'i, 's>>,
    ) -> Result<IVonData, VmRuntimeErrorV<'s>> {
        self.compilation.get_monouts();
        let program_h = self.compilation.cached_monouts();
        let interner = &self.compilation.instantiating_interner;
        let mut vivem_dout = stdout();
        let vivem_bump = Bump::new();
        execute_with_primitive_args(
            program_h,
            interner,
            self.scout_arena,
            &args,
            &mut vivem_dout,
            &vivem_bump,
            &empty_stdin,
            &regular_stdout,
        )
    }
}
