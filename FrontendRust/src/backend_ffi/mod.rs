// FFI bridge to the C++ Backend (statically linked via build.rs).

pub mod metal_cache;
pub mod metal_lowerer;

use std::ffi::CString;
use std::os::raw::{c_char, c_int};

extern "C" {
    // AST-driven entry. Caller has populated MetalCache+Program via the
    // metal_cache_ffi.h builders (driven by MetalLowerer against a ProgramH).
    // argv carries valeOptSet-style flags (--output_dir, --region_override,
    // etc.); inputs come from the Program parameter, not from .vast files.
    fn backend_compile_program(
        cache: *mut metal_cache::MetalCacheHandleRaw,
        program: *mut std::ffi::c_void,
        argc: c_int, argv: *mut *mut c_char,
    ) -> i32;
}

/// Safe wrapper around `backend_compile_program`. argv carries CLI-style
/// options (e.g. `["backend", "--output_dir", "<out>"]`); inputs come from
/// the Program parameter, not files.
pub fn backend_compile_program_safe(
    cache: &metal_cache::MetalCache,
    program: &metal_cache::Program<'_>,
    argv: &[&str],
) -> i32 {
    let mut c_strings: Vec<CString> = argv
        .iter()
        .map(|s| CString::new(*s).expect("argv element contained NUL"))
        .collect();
    let mut c_argv: Vec<*mut c_char> =
        c_strings.iter_mut().map(|c| c.as_ptr() as *mut c_char).collect();
    unsafe {
        backend_compile_program(
            cache.raw(),
            program.raw(),
            c_argv.len() as c_int,
            c_argv.as_mut_ptr(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// End-to-end: build a hello-world Program purely via the
    /// metal_cache_ffi.h builders (no JSON, no .vast file), hand it to
    /// backend_compile_program, link the produced .o with clang, exec,
    /// assert exit code.
    ///
    /// Source equivalent: `exported func main() int { return 7; }`
    #[test]
    fn hello_world_via_backend_program_ffi() {
        use crate::backend_ffi::metal_cache::*;

        let work = tempfile::tempdir().unwrap();
        let out_dir = work.path().join("out");
        std::fs::create_dir_all(&out_dir).unwrap();
        // Backend's Externs ctor writes `<out_dir>/include/ValeBuiltins.h`;
        // it doesn't mkdir -p.
        std::fs::create_dir_all(out_dir.join("include")).unwrap();

        // 1. Build Program via FFI builders.
        let cache = MetalCache::new();
        let coord = cache.get_package_coordinate("test", &[]);
        let main_name = cache.get_name(coord, "main");
        let proto = cache.get_prototype(main_name, cache.i32_ref(), &[]);

        let seven = cache.expr_constant_int(7, 32);
        let ret = cache.expr_return(seven, cache.i32_ref());
        let body = cache.expr_block(ret, cache.i32_ref());
        let func = cache.new_function(proto, Some(body));

        let pb = cache.new_package_builder(coord);
        pb.add_function("main", func);
        pb.add_export_function("main", proto);
        let pkg = pb.finish();

        let progb = cache.new_program_builder();
        progb.add_package(coord, pkg);
        let program = progb.finish();

        // 2. Compile via the AST-driven backend entry.
        let argv = vec![
            "backend".to_string(),
            "--output_dir".to_string(),
            out_dir.display().to_string(),
        ];
        let argv_refs: Vec<&str> = argv.iter().map(|s| s.as_str()).collect();
        let rc = crate::backend_ffi::backend_compile_program_safe(&cache, &program, &argv_refs);
        assert_eq!(rc, 0, "backend_compile_program returned {}", rc);

        // 3. Link & exec.
        let obj = out_dir.join("build.o");
        assert!(obj.exists(), "backend did not produce {}", obj.display());
        let builtins_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent().unwrap().join("Backend/builtins");
        let builtin_cs: Vec<_> = std::fs::read_dir(&builtins_dir).unwrap()
            .filter_map(|e| e.ok()).map(|e| e.path())
            .filter(|p| p.extension().map_or(false, |x| x == "c")).collect();
        let exe = out_dir.join("a.out");
        let link_status = std::process::Command::new("clang")
            .arg("-o").arg(&exe).arg(&obj).args(&builtin_cs)
            .arg("-lm").arg("-Wno-everything").status().expect("clang spawn");
        assert!(link_status.success(), "clang link failed: {}", link_status);
        let out = std::process::Command::new(&exe).output().expect("exec a.out");
        assert_eq!(out.status.code().unwrap_or(-1), 7,
            "exit code mismatch (stdout={:?} stderr={:?})",
            String::from_utf8_lossy(&out.stdout), String::from_utf8_lossy(&out.stderr));
    }
}
