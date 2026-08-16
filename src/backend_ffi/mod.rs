// FFI bridge to the C++ Backend (statically linked via build.rs).

pub mod metal_cache;
pub mod metal_lowerer;

use std::ffi::CString;
use std::os::raw::c_char;

// Optimization level, matches BACKEND_OPT_LEVEL_* in Backend/src/backend_options_ffi.h.
pub const BACKEND_OPT_LEVEL_O0: i32 = 0;
pub const BACKEND_OPT_LEVEL_O1: i32 = 1;
pub const BACKEND_OPT_LEVEL_O2: i32 = 2;
pub const BACKEND_OPT_LEVEL_O2I: i32 = 3;
pub const BACKEND_OPT_LEVEL_O3: i32 = 4;

/// C-repr mirror of BackendCompileOptionsFFI in
/// Backend/src/backend_options_ffi.h. Field order and types must match.
#[repr(C)]
struct BackendCompileOptionsFFIRaw {
    output_dir: *const c_char,
    triple: *const c_char,
    cpu: *const c_char,
    opt_level: i32,
    pic: u8,
    verify: u8,
    print_asm: u8,
    print_llvmir: u8,
    census: u8,
    flares: u8,
    include_bounds_checks: u8,
    use_atomic_rc: u8,
    force_all_known_live: u8,
    print_mem_overhead: u8,
    enable_replaying: u8,
    replay_whitelist_count: usize,
    replay_whitelist_modules: *const *const c_char,
    replay_whitelist_functions: *const *const c_char,
}

extern "C" {
    // AST-driven entry. Caller has populated MetalCache+Program via the
    // metal_cache_ffi.h builders (driven by MetalLowerer against a ProgramH).
    // ffi_opts carries non-input options; inputs come from the Program.
    fn backend_compile_program(
        cache: *mut metal_cache::MetalCacheHandleRaw,
        program: *mut std::ffi::c_void,
        ffi_opts: *const BackendCompileOptionsFFIRaw,
    ) -> i32;
}

/// Rust-owned build of the FFI options. `output_dir` is required; all
/// other fields have sensible defaults matching Backend/valeopts.h.
pub struct BackendCompileOptions {
    pub output_dir: String,
    pub triple: String,
    pub cpu: String,
    pub opt_level: i32,
    pub pic: bool,
    pub verify: bool,
    pub print_asm: bool,
    pub print_llvmir: bool,
    pub census: bool,
    pub flares: bool,
    pub include_bounds_checks: bool,
    pub use_atomic_rc: bool,
    pub force_all_known_live: bool,
    pub print_mem_overhead: bool,
    pub enable_replaying: bool,
    /// (module, extern) pairs allowed to run during a replay.
    pub replay_whitelist: Vec<(String, String)>,
}

impl Default for BackendCompileOptions {
    fn default() -> Self {
        Self {
            output_dir: String::new(),
            triple: String::new(),
            cpu: String::new(),
            opt_level: BACKEND_OPT_LEVEL_O2I,
            pic: false,
            verify: false,
            print_asm: false,
            print_llvmir: false,
            census: false,
            flares: false,
            include_bounds_checks: true,
            use_atomic_rc: false,
            force_all_known_live: false,
            print_mem_overhead: false,
            enable_replaying: false,
            replay_whitelist: Vec::new(),
        }
    }
}

/// Safe wrapper around the C++ backend entry. Marshals the options into a
/// C-POD struct with caller-owned strings and calls across the FFI.
pub fn backend_compile_program_safe(
    cache: &metal_cache::MetalCache,
    program: &metal_cache::Program<'_>,
    opts: &BackendCompileOptions,
) -> i32 {
    let output_dir_c = CString::new(opts.output_dir.as_str()).expect("output_dir contains NUL");
    let triple_c = CString::new(opts.triple.as_str()).expect("triple contains NUL");
    let cpu_c = CString::new(opts.cpu.as_str()).expect("cpu contains NUL");

    let module_cs: Vec<CString> = opts.replay_whitelist.iter()
        .map(|(m, _)| CString::new(m.as_str()).expect("whitelist module contains NUL"))
        .collect();
    let function_cs: Vec<CString> = opts.replay_whitelist.iter()
        .map(|(_, f)| CString::new(f.as_str()).expect("whitelist function contains NUL"))
        .collect();
    let module_ptrs: Vec<*const c_char> = module_cs.iter().map(|c| c.as_ptr()).collect();
    let function_ptrs: Vec<*const c_char> = function_cs.iter().map(|c| c.as_ptr()).collect();

    let raw = BackendCompileOptionsFFIRaw {
        output_dir: output_dir_c.as_ptr(),
        triple: triple_c.as_ptr(),
        cpu: cpu_c.as_ptr(),
        opt_level: opts.opt_level,
        pic: opts.pic as u8,
        verify: opts.verify as u8,
        print_asm: opts.print_asm as u8,
        print_llvmir: opts.print_llvmir as u8,
        census: opts.census as u8,
        flares: opts.flares as u8,
        include_bounds_checks: opts.include_bounds_checks as u8,
        use_atomic_rc: opts.use_atomic_rc as u8,
        force_all_known_live: opts.force_all_known_live as u8,
        print_mem_overhead: opts.print_mem_overhead as u8,
        enable_replaying: opts.enable_replaying as u8,
        replay_whitelist_count: opts.replay_whitelist.len(),
        replay_whitelist_modules: if module_ptrs.is_empty() { std::ptr::null() } else { module_ptrs.as_ptr() },
        replay_whitelist_functions: if function_ptrs.is_empty() { std::ptr::null() } else { function_ptrs.as_ptr() },
    };

    unsafe {
        backend_compile_program(cache.raw(), program.raw(), &raw)
    }
}
