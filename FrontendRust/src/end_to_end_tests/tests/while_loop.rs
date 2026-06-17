use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

#[test] fn while_unsafe_fast() { assert_compile_and_run(&p("programs/while/while.vale"), "unsafe-fast", 42); }
#[test] fn while_naive_rc()    { assert_compile_and_run(&p("programs/while/while.vale"), "naive-rc", 42); }
