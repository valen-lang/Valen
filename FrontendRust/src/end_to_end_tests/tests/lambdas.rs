use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

#[test] fn lambda()    { assert_compile_and_run(&p("programs/lambdas/lambda.vale"), 42); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn lambdamut() { assert_compile_and_run(&p("programs/lambdas/lambdamut.vale"), 42); }
