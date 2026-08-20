use std::io::stdout;
use std::sync::Arc;
use bumpalo::Bump;
use crate::builtins::builtins::{builtin_source_for_arrays, empty_v_builtins_stub};
use crate::code_source::{CodeSource, Source};
use crate::compile_options::GlobalOptions;
use crate::instantiating::instantiated_compilation::{InstantiatedCompilation, InstantiatorCompilationOptions};
use crate::keywords::Keywords;
use crate::parse_arena::ParseArena;
use crate::scout_arena::ScoutArena;
use crate::tests::tests::{new_test_code_map, test_source_from_dir};
use crate::testvm::vivem::{empty_stdin, execute_with_primitive_args, null_stdout};
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::testvm::von::{IVonData, VonInt};

/// Dark-box harness: compile a real `.vale` program all the way through the instantiator to a
/// `HinputsI`, then run its `main` in the TestVM and hand back the computed VON return value. This
/// replaces the old hand-built `ProgramH` fixtures (S6) — the tests now assert on the same path the
/// real compiler drives. `with_builtins` prepends the `v.builtins.arrays` bundle (arrays + arith +
/// drop + implicit_clone) so arithmetic-using fixtures resolve `+`; without it the program stands
/// alone (mirrors the two instantiator test harnesses).
fn run_vale(code: &str, with_builtins: bool) -> IVonData {
    let parse_bump = Bump::new();
    let scout_bump = Bump::new();
    let typing_bump = Bump::new();
    let instantiating_bump = Bump::new();
    let compilation_bump = Bump::new();
    let vivem_bump = Bump::new();
    let parse_arena = ParseArena::new(&parse_bump);
    let scout_arena = ScoutArena::new(&scout_bump);
    let keywords = Keywords::new_for_scout(&scout_arena);
    let parser_keywords = Keywords::new_for_parse(&parse_arena);
    let typing_interner = TypingInterner::new(&typing_bump);

    let packages_to_build: Vec<&PackageCoordinate> =
        vec![PackageCoordinate::test_tld(&parse_arena, &parser_keywords)];
    let code_source: &CodeSource = compilation_bump.alloc(CodeSource::new(
        if with_builtins {
            vec![
                builtin_source_for_arrays(&parse_arena, &parser_keywords),
                new_test_code_map(&parse_arena, code),
                Source::Fn(empty_v_builtins_stub),
            ]
        } else {
            vec![
                new_test_code_map(&parse_arena, code),
                Source::Fn(test_source_from_dir),
            ]
        }));
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
    let mut compile = InstantiatedCompilation::new(
        &typing_interner,
        &scout_arena,
        &keywords,
        &parser_keywords,
        &parse_arena,
        packages_to_build,
        code_source,
        global_options,
        instantiator_options,
        &instantiating_bump,
    );

    // Populate the monouts cache under the `&mut` borrow, then read the cached HinputsI and the
    // interner as two coexisting `&` borrows for the VM run.
    compile.get_monouts();
    let program_h = compile.cached_monouts();
    let interner = &compile.instantiating_interner;

    let mut stdout = stdout();
    let result = execute_with_primitive_args(
        program_h,
        interner,
        &scout_arena,
        &[],
        &mut stdout,
        &vivem_bump,
        &empty_stdin,
        &null_stdout,
    );
    result.expect("VM run failed")
}

#[test]
fn return_7() {
    let von = run_vale("exported func main() int { return 7; }", false);
    match von {
        IVonData::Int(VonInt { value: 7 }) => {}
        other => panic!("expected VonInt(7), got {:?}", other),
    }
}

#[test]
fn adding() {
    let code = r"
import v.builtins.arith.*;
exported func main() int { return 52 + 53 + 54; }
";
    let von = run_vale(code, true);
    match von {
        IVonData::Int(VonInt { value: 159 }) => {}
        other => panic!("expected VonInt(159), got {:?}", other),
    }
}
