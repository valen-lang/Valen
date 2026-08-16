// Backend (Midas) options assembly. The actual backend call happens inside
// pass_manager::build (which feeds the in-process MetalCache
// populated by MetalLowerer straight into backend_compile_program).

use std::path::Path;

use frontend_rust::backend_ffi::{
    BackendCompileOptions,
    BACKEND_OPT_LEVEL_O0, BACKEND_OPT_LEVEL_O1, BACKEND_OPT_LEVEL_O2,
    BACKEND_OPT_LEVEL_O2I, BACKEND_OPT_LEVEL_O3,
};

/// Assemble the options struct that pass_manager::build hands to
/// backend_compile_program. `--triple` is left blank here; pass_manager
/// injects it from ClangConfig.
#[allow(clippy::too_many_arguments)]
pub fn build_backend_options(
    output_dir: &Path,
    maybe_opt_level: Option<&str>,
    maybe_cpu: Option<&str>,
    // `-o <name>` is consumed by valec for the final clang invocation;
    // the backend writes a fixed `build.o` regardless of this name.
    _executable_name: &str,
    flares: bool,
    census: bool,
    verify: bool,
    opt_level: &str,
    llvm_ir: bool,
    asm: bool,
    enable_replaying: bool,
    replay_whitelist_extern: &str,
    pic: bool,
    print_mem_overhead: bool,
    use_atomic_rc: bool,
    force_all_known_live: bool,
    include_bounds_checks: bool,
) -> BackendCompileOptions {
    let mut opts = BackendCompileOptions::default();
    opts.output_dir = output_dir.display().to_string();
    opts.verify = verify;
    opts.flares = flares;
    opts.census = census;
    opts.print_llvmir = llvm_ir;
    opts.print_asm = asm;
    opts.enable_replaying = enable_replaying;
    opts.pic = pic;
    opts.print_mem_overhead = print_mem_overhead;
    opts.use_atomic_rc = use_atomic_rc;
    opts.force_all_known_live = force_all_known_live;
    opts.include_bounds_checks = include_bounds_checks;

    if let Some(cpu) = maybe_cpu {
        opts.cpu = cpu.to_string();
    }

    // Prefer maybe_opt_level when set; otherwise honor the opt_level string
    // (still matches the pre-refactor midas argv behavior, where an explicit
    // maybe_opt_level was appended first and opt_level=="O0" was skipped).
    let level_str = maybe_opt_level.unwrap_or(opt_level);
    opts.opt_level = match level_str {
        "O0" => BACKEND_OPT_LEVEL_O0,
        "O1" => BACKEND_OPT_LEVEL_O1,
        "O2" => BACKEND_OPT_LEVEL_O2,
        "O2i" => BACKEND_OPT_LEVEL_O2I,
        "O3" => BACKEND_OPT_LEVEL_O3,
        other => panic!("Unknown opt_level: {}", other),
    };

    if !replay_whitelist_extern.is_empty() {
        let (module, function) = replay_whitelist_extern.split_once('.')
            .expect("replay_whitelist_extern must be in the form module.function");
        opts.replay_whitelist.push((module.to_string(), function.to_string()));
    }

    opts
}
