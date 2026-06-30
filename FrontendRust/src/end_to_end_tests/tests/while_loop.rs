use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn while_loop() { assert_compile_and_run(&p("programs/while/while.vale"), 42); }
