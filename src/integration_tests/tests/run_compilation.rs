//! Dark-box integration harness on the onion path. `test`/`test_no_builtins` compile a real `.vale`
//! program through `InstantiatedCompilation` to a `HinputsI`; `RunCompilation` then runs `main` in
//! the TestVM and returns the computed VON. Replaces the deleted Hammer/`ProgramH` path (mirrors
//! `testvm::test::vivem_tests::run_vale`).

use std::io::stdout;
use std::sync::Arc;
use bumpalo::Bump;
use crate::code_source::{CodeSource, Source};
use crate::compile_options::GlobalOptions;
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::instantiated_compilation::{InstantiatedCompilation, InstantiatorCompilationOptions};
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
    let packages_to_build: Vec<&'p PackageCoordinate<'p>> =
        vec![PackageCoordinate::test_tld(parse_arena, parser_keywords)];
    let code_source: &'ctx CodeSource<'p> = compilation_bump.alloc(CodeSource::new(vec![
        new_test_code_map(parse_arena, code),
        Source::Fn(test_source_from_dir),
    ]));
    let compilation = InstantiatedCompilation::new(
        typing_interner, scout_arena, keywords, parser_keywords, parse_arena,
        packages_to_build, code_source, global_options(), instantiator_options(), instantiating_bump,
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
