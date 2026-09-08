use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

// VDBG: no debugger gate yet — closures not modeled
#[test]
fn lambda()    { assert_compile_and_run(&p("programs/lambdas/lambda.vale"), 42); }
// VDBG: no debugger gate yet — closures not modeled
#[test]
#[ignore = "deferred: group-generic-closures — borrow checker can't derive a group for a closure-captured reference (borrow_types.rs:347)"]
fn lambdamut() { assert_compile_and_run(&p("programs/lambdas/lambdamut.vale"), 42); }
