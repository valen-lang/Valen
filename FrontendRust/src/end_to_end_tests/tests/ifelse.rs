use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

#[test] fn if_unsafe_fast()       { assert_compile_and_run(&p("programs/if/if.vale"), "unsafe-fast", 42); }
#[test] fn if_naive_rc()          { assert_compile_and_run(&p("programs/if/if.vale"), "naive-rc", 42); }
#[test] fn upcastif_unsafe_fast() { assert_compile_and_run(&p("programs/if/upcastif.vale"), "unsafe-fast", 42); }
#[test] fn upcastif_naive_rc()    { assert_compile_and_run(&p("programs/if/upcastif.vale"), "naive-rc", 42); }
#[test] fn ifnevers_unsafe_fast() { assert_compile_and_run(&p("programs/if/ifnevers.vale"), "unsafe-fast", 42); }
#[test] fn ifnevers_naive_rc()    { assert_compile_and_run(&p("programs/if/ifnevers.vale"), "naive-rc", 42); }
#[test] fn nestedif_unsafe_fast() { assert_compile_and_run(&p("programs/if/nestedif.vale"), "unsafe-fast", 42); }
#[test] fn nestedif_naive_rc()    { assert_compile_and_run(&p("programs/if/nestedif.vale"), "naive-rc", 42); }
