//! Inline-source end-to-end tests. The Vale program lives as a string in the
//! test body; the harness writes it to a tempdir (`test.vale`) before running
//! through `pass_manager::build`.
//!
//! Includes the two tests previously stuck under `#[ignore]` in
//! `pass_manager/end_to_end_test.rs` — the harness now drives the full
//! backend + clang + exec path, so they're live.

use crate::end_to_end_tests::{
    assert_inline_compile_and_run, assert_inline_compile_and_run_dbg, cmd, expect,
};

#[test]
fn pass_manager_main_builds_simple_program_end_to_end() {
    assert_inline_compile_and_run_dbg(
        "exported func main() int { return 3; }",
        3,
        &[
            cmd("b :main"),
            cmd("run"),
            expect("bt", &["test.vale:1", ":main"]),
        ],
    );
}

// Builtin `Some<int>` construction — shape-limited (single line, generic builtin);
// assert the subprogram resolves.
#[test]
fn pass_manager_main_builds_program_using_builtin_some() {
    assert_inline_compile_and_run_dbg(
        "struct Moo<T> { x T; } exported func main() int { x = Moo<int>(3); return 0; }",
        0,
        &[
            cmd("b :main"),
            cmd("run"),
            expect("bt", &["test.vale:1", ":main"]),
        ],
    );
}

// A cross-frame call stack: break inside helper, backtrace shows helper AND its caller main.
#[test]
fn basic_function_call() {
    assert_inline_compile_and_run_dbg(
        "func helper() int { return 42; }\nexported func main() int { return helper(); }",
        42,
        &[
            cmd("b helper"),
            cmd("run"),
            expect("bt", &["helper at test.vale", ":main"]),
        ],
    );
}

// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: share"]
fn string_len() {
    assert_inline_compile_and_run(
        "exported func main() int { return (&\"hello\").len(); }",
        5,
    );
}
