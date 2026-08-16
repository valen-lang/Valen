use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

#[test] fn ifelse()   { assert_compile_and_run(&p("programs/if/if.vale"), 42); }
#[test] fn upcastif() { assert_compile_and_run(&p("programs/if/upcastif.vale"), 42); }
#[test] fn ifnevers() { assert_compile_and_run(&p("programs/if/ifnevers.vale"), 42); }
#[test] fn nestedif() { assert_compile_and_run(&p("programs/if/nestedif.vale"), 42); }
