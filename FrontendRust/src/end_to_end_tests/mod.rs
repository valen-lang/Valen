//! End-to-end harness: drives a real Vale program (file, directory, or inline
//! source) through `pass_manager::build` (the production entry `valec` uses),
//! invokes clang on the produced `.o` + abi files, and provides a
//! `CompiledProgram` whose binary can be exec'd and asserted on.
//!
//! This is the unified home for in-process backend tests, replacing the
//! out-of-process `TesterRust` driver and folding in inline-source and
//! pass_manager-driven tests that previously lived in
//! `backend_ffi/metal_lowerer.rs` and `pass_manager/end_to_end_test.rs`.

use std::path::{Path, PathBuf};

pub mod tests;

/// Which backend the harness should compile + run programs through.
///
/// Selected per-process via the `VALE_TEST_BACKEND` env var (`native`
/// default, `wasi` to cross-compile to `wasm32-wasi` and execute under
/// wasmtime). Every test runs against the selected backend unless it
/// opts out via `wasi_skip!("reason")`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Backend {
    Native,
    Wasi,
}

pub fn target_backend() -> Backend {
    match std::env::var("VALE_TEST_BACKEND").as_deref() {
        Ok("wasi") => Backend::Wasi,
        _ => Backend::Native,
    }
}

impl Backend {
    pub fn exe_name(self) -> &'static str {
        match self {
            Backend::Native => "a.out",
            Backend::Wasi => "a.out.wasm",
        }
    }

    pub fn target_triple(self) -> Option<&'static str> {
        match self {
            Backend::Native => None,
            Backend::Wasi => Some("wasm32-wasi"),
        }
    }

    /// wasi-sdk sysroot path. Resolved from the `WASI_SDK_PATH` env var if
    /// set, else `~/wasi-sdk`. Returns the `share/wasi-sysroot` subdir.
    pub fn sysroot(self) -> Option<PathBuf> {
        match self {
            Backend::Native => None,
            Backend::Wasi => Some(wasi_sdk_path().join("share/wasi-sysroot")),
        }
    }

    /// Clang binary to invoke. wasi-sdk ships its own clang preconfigured
    /// for wasi; the host clang generally won't find the wasi sysroot
    /// libraries even with `--target=wasm32-wasi --sysroot=...`.
    pub fn clang_path(self) -> Option<String> {
        match self {
            Backend::Native => None,
            Backend::Wasi => Some(
                wasi_sdk_path()
                    .join("bin/clang")
                    .display()
                    .to_string(),
            ),
        }
    }
}

fn wasi_sdk_path() -> PathBuf {
    if let Ok(p) = std::env::var("WASI_SDK_PATH") {
        return PathBuf::from(p);
    }
    let home = std::env::var("HOME").expect("HOME unset");
    PathBuf::from(home).join("wasi-sdk")
}

/// Inside a `#[test]` fn, returns early when running under the wasi
/// backend with a reason logged. Place at the very top of the fn body.
#[macro_export]
macro_rules! wasi_skip {
    ($reason:expr) => {
        if $crate::end_to_end_tests::target_backend()
            == $crate::end_to_end_tests::Backend::Wasi
        {
            eprintln!("wasi_skip: {}", $reason);
            return;
        }
    };
}

pub struct CompiledProgram {
    exe: PathBuf,
    pub cwd: PathBuf,
    _work: tempfile::TempDir,
    _extra_keepalive: Vec<tempfile::TempDir>,
    backend: Backend,
}

pub struct ExecResult {
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

/// Compile a Vale program to a native binary via `pass_manager::build`.
///
/// - `primary_vale` may be a single `.vale` file or a directory; the directory
///   case is registered as a `vtest=<dir>` project (whose `.vale` files at the
///   top level get walked).
/// - `extra_c` lists additional C sources to link with clang (e.g. extern
///   tests' `native/test.c`).
/// - `configure_backend` may mutate the default backend options (e.g. set
///   `enable_replaying = true` for replay tests).
pub fn compile_program(
    primary_vale: &Path,
    extra_c: &[&Path],
    configure_backend: impl FnOnce(&mut crate::backend_ffi::BackendCompileOptions),
) -> CompiledProgram {
    compile_inputs(
        vec![primary_vale.to_path_buf()],
        extra_c,
        configure_backend,
        Vec::new(),
    )
}

/// Compile a single inline Vale program to a native binary.
///
/// Writes `code` to a `test.vale` in a temp source directory, registers it
/// as the `vtest=` project, and dispatches through the same harness path
/// as `compile_program`. The temp source directory is kept alive for the
/// life of the returned `CompiledProgram`.
pub fn compile_inline(
    code: &str,
    configure_backend: impl FnOnce(&mut crate::backend_ffi::BackendCompileOptions),
) -> CompiledProgram {
    let src_dir = tempfile::tempdir().unwrap();
    let src_file = src_dir.path().join("test.vale");
    std::fs::write(&src_file, code).unwrap();
    compile_inputs(
        vec![src_dir.path().to_path_buf()],
        &[],
        configure_backend,
        vec![src_dir],
    )
}

fn compile_inputs(
    vale_inputs: Vec<PathBuf>,
    extra_c: &[&Path],
    configure_backend: impl FnOnce(&mut crate::backend_ffi::BackendCompileOptions),
    keepalive: Vec<tempfile::TempDir>,
) -> CompiledProgram {
    let parse_bump = bumpalo::Bump::new();
    let parse_arena = crate::parse_arena::ParseArena::new(&parse_bump);
    let keywords = crate::keywords::Keywords::new_for_parse(&parse_arena);

    let work = tempfile::tempdir().unwrap();
    let out_dir = work.path().join("out");
    std::fs::create_dir_all(&out_dir).unwrap();

    let mut cli_args: Vec<String> = vec![
        "build".to_string(),
        "--output_dir".to_string(),
        out_dir.display().to_string(),
        "--output_vast".to_string(),
        "true".to_string(),
        "--include_builtins".to_string(),
        "true".to_string(),
        "--sanity_check".to_string(),
        "false".to_string(),
    ];
    for input in &vale_inputs {
        cli_args.push(format!("vtest={}", input.display()));
    }
    let opts = crate::pass_manager::pass_manager::parse_opts(
        &parse_arena,
        crate::pass_manager::pass_manager::Options {
            inputs: vec![],
            output_dir_path: None,
            benchmark: false,
            output_vast: true,
            include_builtins: true,
            mode: None,
            sanity_check: false,
            use_optimized_solver: true,
            use_overload_index: true,
            verbose_errors: false,
            debug_output: false,
        },
        cli_args,
    );

    let mut backend_opts = crate::backend_ffi::BackendCompileOptions::default();
    backend_opts.output_dir = out_dir.display().to_string();
    configure_backend(&mut backend_opts);

    let builtins_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("Backend/builtins");
    let test_builtins = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("Backend/test_builtins/testbuiltins.c");

    let mut extra_inputs: Vec<PathBuf> = vec![test_builtins];
    for c in extra_c {
        extra_inputs.push(c.to_path_buf());
    }

    let backend = target_backend();
    let clang_cfg = crate::pass_manager::pass_manager::ClangConfig {
        builtins_dir,
        extra_inputs,
        clang_path: backend.clang_path(),
        libc_path: None,
        executable_name: backend.exe_name().to_string(),
        asan: false,
        debug_symbols: false,
        pic: false,
        pie: false,
        windows: false,
        target_triple: backend.target_triple().map(str::to_string),
        sysroot: backend.sysroot(),
    };

    let bp = crate::pass_manager::pass_manager::build(
        &parse_arena,
        &keywords,
        &opts,
        backend_opts,
        &clang_cfg,
    )
    .unwrap_or_else(|e| panic!("pass_manager::build failed:\n{}", e));
    assert_eq!(bp.rc, 0, "backend returned {}", bp.rc);

    CompiledProgram {
        exe: bp.exe_path,
        cwd: out_dir,
        _work: work,
        _extra_keepalive: keepalive,
        backend,
    }
}

impl CompiledProgram {
    pub fn run(&self, args: &[&str]) -> ExecResult {
        let out = match self.backend {
            Backend::Native => std::process::Command::new(&self.exe)
                .current_dir(&self.cwd)
                .args(args)
                .output()
                .expect("exec failed"),
            Backend::Wasi => {
                // wasmtime: `wasmtime run --dir=. a.out.wasm -- <args>`.
                // `--dir=.` grants the program filesystem access to the
                // cwd it was invoked in (needed for replay-bin reads,
                // file-extern tests, etc.).
                let mut cmd = std::process::Command::new("wasmtime");
                cmd.current_dir(&self.cwd)
                    .arg("run")
                    .arg("--dir=.")
                    .arg(&self.exe);
                if !args.is_empty() {
                    cmd.arg("--");
                    cmd.args(args);
                }
                cmd.output().expect("wasmtime exec failed")
            }
        };
        ExecResult {
            exit_code: out.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        }
    }
}

pub fn programs_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/tests")
}

pub fn assert_compile_and_run(vale_path: &Path, expected: i32) {
    let cp = compile_program(vale_path, &[], |_| {});
    let r = cp.run(&[]);
    assert_eq!(
        r.exit_code, expected,
        "stdout={:?} stderr={:?}",
        r.stdout, r.stderr
    );
}

pub fn assert_compile_and_run_with_c(
    vale_dir: &Path,
    extra_c: &[&Path],
    expected: i32,
) {
    let cp = compile_program(vale_dir, extra_c, |_| {});
    let r = cp.run(&[]);
    assert_eq!(
        r.exit_code, expected,
        "stdout={:?} stderr={:?}",
        r.stdout, r.stderr
    );
}

pub fn assert_inline_compile_and_run(code: &str, expected: i32) {
    let cp = compile_inline(code, |_| {});
    let r = cp.run(&[]);
    assert_eq!(
        r.exit_code, expected,
        "stdout={:?} stderr={:?}",
        r.stdout, r.stderr
    );
}

pub fn assert_replay_test(
    vale_dir: &Path,
    extra_c: &[&Path],
    first_expected: i32,
    repeated_expected: i32,
) {
    let cp = compile_program(
        vale_dir,
        extra_c,
        |opts| { opts.enable_replaying = true; },
    );
    let r0 = cp.run(&[]);
    assert_eq!(
        r0.exit_code, first_expected,
        "run 0 (no record/replay): stdout={:?} stderr={:?}",
        r0.stdout, r0.stderr
    );
    let r1 = cp.run(&["--vale_record", "recording.bin"]);
    assert_eq!(
        r1.exit_code, repeated_expected,
        "run 1 (record): stdout={:?} stderr={:?}",
        r1.stdout, r1.stderr
    );
    let r2 = cp.run(&["--vale_replay", "recording.bin"]);
    assert_eq!(
        r2.exit_code, repeated_expected,
        "run 2 (replay): stdout={:?} stderr={:?}",
        r2.stdout, r2.stderr
    );
}

#[cfg(test)]
mod smoke {
    use super::*;

    #[test]
    #[ignore = "deferred at experimental-2 squash baseline"]
    fn smoke_structimm() {
        let p = programs_dir().join("programs/structs/structimm.vale");
        assert_compile_and_run(&p, 5);
    }
}
