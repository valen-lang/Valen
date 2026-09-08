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
use std::process::Command;
use std::fs;
use crate::backend_ffi::BACKEND_OPT_LEVEL_O0;

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

pub enum KeepDir {
    Temp(tempfile::TempDir),
    Kept(std::path::PathBuf),
}
impl KeepDir {
    pub fn path(&self) -> &std::path::Path {
        match self {
            KeepDir::Temp(t) => t.path(),
            KeepDir::Kept(p) => p.as_path(),
        }
    }
}

pub struct CompiledProgram {
    exe: PathBuf,
    pub cwd: PathBuf,
    _work: KeepDir,
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
/// - `configure_backend` may mutate the default backend options.
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

/// Like `compile_inline` but with DWARF debug info enabled (backend `--debug`,
/// opt level O0 so the metadata survives). Returns the `CompiledProgram` plus
/// the on-disk path of the `test.vale` source, so debugger tests can assert
/// against the file lldb resolves frames to.
pub fn compile_inline_debug(code: &str) -> (CompiledProgram, PathBuf) {
    let src_dir = tempfile::tempdir().unwrap();
    let src_file = src_dir.path().join("test.vale");
    fs::write(&src_file, code).unwrap();
    let src_file_path = src_file.clone();
    let cp = compile_inputs(
        vec![src_dir.path().to_path_buf()],
        &[],
        |opts| {
            opts.debug = true;
            opts.opt_level = BACKEND_OPT_LEVEL_O0;
        },
        vec![src_dir],
    );
    (cp, src_file_path)
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

    // Under nextest, each test process gets a unique NEXTEST_TEST_NAME env
    // var. Route outputs to a predictable per-test path so failed runs stay
    // inspectable — `cd tmp/vale-test-runs/<name>/out; ./a.out` after a
    // failing `cargo nextest run <name>`. Under plain `cargo test`, fall
    // back to the auto-deleting tempdir behavior.
    let work = if let Ok(name) = std::env::var("NEXTEST_TEST_NAME") {
        let safe = name.replace("::", "-");
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tmp/vale-test-runs")
            .join(safe);
        let _ = std::fs::remove_dir_all(&root);
        std::fs::create_dir_all(&root).unwrap();
        println!("Test outputs: {}", root.display());
        KeepDir::Kept(root)
    } else {
        KeepDir::Temp(tempfile::tempdir().unwrap())
    };
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
    if std::env::var("VALE_FLARES").is_ok() {
        backend_opts.flares = true;
    }
    if std::env::var("VALE_LLVM_IR").is_ok() {
        backend_opts.print_llvmir = true;
    }
    // VALE_TEST_CENSUS=1 compiles the program in census mode: every heap
    // object is tracked in a live-object census (a global counter + hashtable),
    // and main asserts the counter is 0 at exit ("Memory leaks!"). Unlike
    // LSan (a no-op on macOS), this is generated C + an assert, so it catches
    // RC leaks (under-release) and cycle leaks on every platform. Use it to
    // audit FFI reference-count balance.
    if std::env::var("VALE_TEST_CENSUS").is_ok() {
        backend_opts.census = true;
    }
    backend_opts.verify = true;
    configure_backend(&mut backend_opts);
    // When the backend emits DWARF (--debug), link clang with -g so the debug
    // info survives the link, and (on macOS) so a sibling .dSYM is produced by
    // the dsymutil step below.
    let link_with_debug = backend_opts.debug;

    let builtins_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("Backend/builtins");
    let test_builtins = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("Backend/test_builtins/testbuiltins.c");

    let mut extra_inputs: Vec<PathBuf> = vec![test_builtins];
    for c in extra_c {
        extra_inputs.push(c.to_path_buf());
    }

    let backend = target_backend();
    // ASan/LSan are opt-in via VALE_TEST_ASAN=1 (Native only; WASI's
    // wasmtime executor doesn't run the address-sanitizer runtime). Off by
    // default because ASan roughly halves test throughput and its LSan
    // component may report false positives against Vale's intentionally-
    // immortal IMMUTABLE_SHARE objects. Turn on when auditing FFI RC
    // balance — e.g. after the always-OWN extern-arg ABI change.
    let asan_enabled = matches!(backend, Backend::Native)
        && std::env::var("VALE_TEST_ASAN").is_ok();
    let clang_cfg = crate::pass_manager::pass_manager::ClangConfig {
        builtins_dir,
        extra_inputs,
        clang_path: backend.clang_path(),
        libc_path: None,
        executable_name: backend.exe_name().to_string(),
        asan: asan_enabled,
        debug_symbols: link_with_debug,
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

    if link_with_debug && matches!(backend, Backend::Native) {
        // On macOS the DWARF lives in the .o files after link; lldb finds it
        // via a sibling .dSYM bundle that dsymutil materializes. Without this
        // the binary has no resolvable line info even with --debug + -g.
        let dsym_status = Command::new("dsymutil")
            .arg(&bp.exe_path)
            .status()
            .expect("dsymutil spawn failed");
        assert!(dsym_status.success(), "dsymutil failed");
    }

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
        // On Apple targets the LSan runtime lives inside ASan and is off by
        // default at runtime; set ASAN_OPTIONS=detect_leaks=1 so `cargo
        // nextest run` under VALE_TEST_ASAN=1 actually reports leaks. Point
        // LSAN_OPTIONS at the checked-in suppressions file to filter false
        // positives from Apple's ObjC runtime (class caches allocated at
        // dyld-init that are never freed by design). abort_on_error+
        // halt_on_error make surviving leak reports fail the exit code.
        let asan_env = std::env::var("VALE_TEST_ASAN").is_ok();
        let out = match self.backend {
            Backend::Native => {
                let mut cmd = std::process::Command::new(&self.exe);
                cmd.current_dir(&self.cwd).args(args);
                if asan_env {
                    cmd.env("ASAN_OPTIONS", "detect_leaks=1:abort_on_error=1:halt_on_error=1");
                    let suppressions = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                        .join("lsan-suppressions.txt");
                    cmd.env("LSAN_OPTIONS", format!("suppressions={}", suppressions.display()));
                }
                cmd.output().expect("exec failed")
            },
            Backend::Wasi => {
                // wasmtime: `wasmtime run --dir=. a.out.wasm <args>`.
                // `--dir=.` grants the program filesystem access to the
                // cwd it was invoked in (needed for file-extern tests, etc.).
                // Guest args go straight after the module — no `--` separator,
                // which this wasmtime forwards into the guest's argv (it would
                // land as argv[1], shifting the real args down one).
                let mut cmd = std::process::Command::new("wasmtime");
                cmd.current_dir(&self.cwd)
                    .arg("run")
                    .arg("--dir=.")
                    .arg(&self.exe)
                    .args(args);
                cmd.output().expect("wasmtime exec failed")
            }
        };
        ExecResult {
            exit_code: out.status.code().unwrap_or(-1),
            stdout: String::from_utf8_lossy(&out.stdout).into_owned(),
            stderr: String::from_utf8_lossy(&out.stderr).into_owned(),
        }
    }

    /// Drive `lldb -b` against this program with the given command sequence
    /// (one entry per `-o`), returning the combined stdout+stderr. Native only
    /// (macOS lldb + the dsymutil'd binary from `compile_inline_debug`).
    pub fn lldb_capture(&self, commands: &[&str]) -> String {
        let mut cmd = Command::new("lldb");
        cmd.arg("-b");
        for c in commands {
            cmd.arg("-o").arg(c);
        }
        cmd.arg(&self.exe);
        let out = cmd.output().expect("lldb spawn failed");
        format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        )
    }

    /// Run an lldb session and assert each `expected_substrings` entry appears
    /// somewhere in the combined output. Substring-based so tests don't couple
    /// to exact lldb formatting, which drifts between lldb versions.
    pub fn lldb_check(&self, commands: &[&str], expected_substrings: &[&str]) {
        let combined = self.lldb_capture(commands);
        for needle in expected_substrings {
            assert!(
                combined.contains(needle),
                "lldb output missing expected substring {:?}\nlldb commands: {:?}\nfull output:\n{}",
                needle, commands, combined,
            );
        }
    }

    /// Like `lldb_check`, but the expected substrings must appear **in order** —
    /// each found after the previous one's match. For stepping tests this turns
    /// "landed on 2, then 3, then 4" into a real sequence assertion rather than
    /// unordered set membership (which would pass if the stops came out of order
    /// or a line appeared incidentally).
    pub fn lldb_check_ordered(&self, commands: &[&str], expected_in_order: &[&str]) {
        let combined = self.lldb_capture(commands);
        let mut cursor = 0usize;
        for needle in expected_in_order {
            match combined[cursor..].find(needle) {
                Some(rel) => cursor += rel + needle.len(),
                None => panic!(
                    "lldb output missing {:?} in order (after offset {})\nlldb commands: {:?}\nfull output:\n{}",
                    needle, cursor, commands, combined,
                ),
            }
        }
    }

    /// Dump DWARF DIEs via `llvm-dwarfdump` and return the combined
    /// stdout+stderr. Asserting on raw DIEs instead of lldb's rendered output
    /// keeps structural gates decoupled from lldb-version formatting drift.
    /// Resolves the binary from `$PATH`, then the homebrew llvm install; panics
    /// if neither exists (no graceful skip — callers rely on the output).
    /// Native only (targets the dsymutil'd `.dSYM`).
    pub fn dwarfdump_capture(&self, args: &[&str]) -> String {
        let on_path = Command::new("llvm-dwarfdump")
            .arg("--version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        let bin = if on_path {
            "llvm-dwarfdump".to_string()
        } else {
            let fallback = "/opt/homebrew/opt/llvm/bin/llvm-dwarfdump";
            assert!(
                Path::new(fallback).exists(),
                "llvm-dwarfdump not on PATH and {fallback} doesn't exist"
            );
            fallback.to_string()
        };
        let dsym = format!("{}.dSYM", self.exe.display());
        let target = if Path::new(&dsym).exists() {
            dsym
        } else {
            self.exe.display().to_string()
        };
        let out = Command::new(&bin)
            .args(args)
            .arg(&target)
            .output()
            .expect("llvm-dwarfdump spawn failed");
        format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr),
        )
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

/// One step of a debugger session: an lldb command plus what its own output
/// must (`expect`) and must not (`reject`) contain. Steps run in order; each
/// step's checks are scoped to the output that command produced (see
/// `run_dbg_session`), so a value can't accidentally match a breakpoint echo or
/// another step's text.
pub struct Step<'a> {
    pub cmd: &'a str,
    pub expect: &'a [&'a str],
    pub reject: &'a [&'a str],
}

/// A step that just runs a command; its output isn't asserted on (`b`, `run`,
/// `continue`, a bare `thread step-over`).
pub fn cmd(c: &str) -> Step<'_> {
    Step { cmd: c, expect: &[], reject: &[] }
}

/// A step whose output must contain every `present` substring.
pub fn expect<'a>(c: &'a str, present: &'a [&'a str]) -> Step<'a> {
    Step { cmd: c, expect: present, reject: &[] }
}

/// A step whose output must contain every `present` substring and none of the
/// `absent` ones — for asserting an untaken branch / unreached line.
pub fn reject<'a>(c: &'a str, present: &'a [&'a str], absent: &'a [&'a str]) -> Step<'a> {
    Step { cmd: c, expect: present, reject: absent }
}

/// Split a combined `lldb -b` capture into one segment per command. lldb echoes
/// each `-o` command as a `(lldb) <cmd>` line before its output, so we locate
/// those echoes in order (commands may repeat — e.g. several `continue`s — so we
/// scan forward past each match) and take the text between consecutive echoes.
/// A command whose echo isn't found yields an empty segment (its `expect`s then
/// fail with the full session embedded).
fn split_lldb_session<'a>(out: &'a str, cmds: &[&str]) -> Vec<&'a str> {
    let mut echo_start: Vec<Option<usize>> = Vec::with_capacity(cmds.len());
    let mut echo_end: Vec<Option<usize>> = Vec::with_capacity(cmds.len());
    let mut from = 0usize;
    for c in cmds {
        let needle = format!("(lldb) {}", c);
        if let Some(rel) = out[from..].find(&needle) {
            let start = from + rel;
            let end = start + needle.len();
            echo_start.push(Some(start));
            echo_end.push(Some(end));
            from = end;
        } else {
            echo_start.push(None);
            echo_end.push(None);
        }
    }
    let mut segs = Vec::with_capacity(cmds.len());
    for i in 0..cmds.len() {
        match echo_end[i] {
            None => segs.push(""),
            Some(start) => {
                let mut next = out.len();
                for j in (i + 1)..cmds.len() {
                    if let Some(s) = echo_start[j] {
                        next = s;
                        break;
                    }
                }
                segs.push(&out[start..next]);
            }
        }
    }
    segs
}

/// Run a step script against a compiled program and assert each step's scoped
/// output. One `lldb -b` invocation drives all the commands.
fn run_dbg_session(cp: &CompiledProgram, steps: &[Step]) {
    let cmds: Vec<&str> = steps.iter().map(|s| s.cmd).collect();
    let out = cp.lldb_capture(&cmds);
    let segs = split_lldb_session(&out, &cmds);
    for (i, step) in steps.iter().enumerate() {
        let seg = segs[i];
        for needle in step.expect {
            assert!(
                seg.contains(needle),
                "lldb step {} `{}`: output missing {:?}\n--- this step's output ---\n{}\n--- full session ---\n{}",
                i, step.cmd, needle, seg, out,
            );
        }
        for needle in step.reject {
            assert!(
                !seg.contains(needle),
                "lldb step {} `{}`: output unexpectedly contains {:?}\n--- this step's output ---\n{}\n--- full session ---\n{}",
                i, step.cmd, needle, seg, out,
            );
        }
    }
}

/// Copy a single-file fixture into a fresh temp dir (preserving its basename)
/// and compile it from there. Per test-review #9 we never compile a repo path in
/// place — and the temp dir becomes the program's `source_dir`, so lldb can
/// resolve the source (DWARF's `comp_dir "."`) for source-anchored breakpoints.
/// Debug info is emitted on Native only.
fn compile_fixture_debug(vale_path: &Path, native: bool) -> CompiledProgram {
    let src_dir = tempfile::tempdir().unwrap();
    let base = vale_path.file_name().expect("fixture path has no file name");
    let dest = src_dir.path().join(base);
    fs::copy(vale_path, &dest)
        .unwrap_or_else(|e| panic!("copying {:?} into temp dir failed: {}", vale_path, e));
    compile_inputs(
        vec![dest],
        &[],
        |opts| {
            if native {
                opts.debug = true;
                opts.opt_level = BACKEND_OPT_LEVEL_O0;
            }
        },
        vec![src_dir],
    )
}

/// Like `assert_compile_and_run`, but on Native also drives a scoped lldb
/// session (`steps`) against the program — so a normal behavior test doubles as
/// a debug-info regression gate that asserts real debugger behavior.
///
/// The debug half is Native-only: lldb/dsymutil aren't in the wasi toolchain, so
/// under wasi this compiles+runs and asserts the exit code exactly as
/// `assert_compile_and_run` does (no `--debug`, no lldb), and logs a loud skip so
/// a wasi pass isn't mistaken for validated debug coverage. On Native it compiles
/// with `--debug`/O0 (so DWARF survives + a `.dSYM` is materialized) first.
pub fn assert_compile_and_run_dbg(vale_path: &Path, expected: i32, steps: &[Step]) {
    let native = matches!(target_backend(), Backend::Native);
    let cp = compile_fixture_debug(vale_path, native);
    let r = cp.run(&[]);
    assert_eq!(
        r.exit_code, expected,
        "stdout={:?} stderr={:?}",
        r.stdout, r.stderr
    );
    if native {
        run_dbg_session(&cp, steps);
    } else {
        eprintln!(
            "SKIP: debug gate for {:?} requires the Native backend (lldb/dSYM); \
             ran exit-code check only under wasi.",
            vale_path
        );
    }
}

/// Inline-source counterpart of `assert_compile_and_run_dbg`. The lldb session
/// resolves frames to `test.vale` (the name `compile_inline` writes the source
/// under). Native-only debug half, same as above.
pub fn assert_inline_compile_and_run_dbg(code: &str, expected: i32, steps: &[Step]) {
    let native = matches!(target_backend(), Backend::Native);
    let cp = compile_inline(code, |opts| {
        if native {
            opts.debug = true;
            opts.opt_level = BACKEND_OPT_LEVEL_O0;
        }
    });
    let r = cp.run(&[]);
    assert_eq!(
        r.exit_code, expected,
        "stdout={:?} stderr={:?}",
        r.stdout, r.stderr
    );
    if native {
        run_dbg_session(&cp, steps);
    } else {
        eprintln!(
            "SKIP: inline debug gate requires the Native backend (lldb/dSYM); \
             ran exit-code check only under wasi."
        );
    }
}
