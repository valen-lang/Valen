use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

#[test] fn structimm_unsafe_fast()           { assert_compile_and_run(&p("programs/structs/structimm.vale"), "unsafe-fast", 5); }
#[test] fn structimm_naive_rc()              { assert_compile_and_run(&p("programs/structs/structimm.vale"), "naive-rc", 5); }
#[test] fn memberrefcount_unsafe_fast()      { assert_compile_and_run(&p("programs/structs/memberrefcount.vale"), "unsafe-fast", 5); }
#[test] fn memberrefcount_naive_rc()         { assert_compile_and_run(&p("programs/structs/memberrefcount.vale"), "naive-rc", 5); }
#[test] fn bigstructimm_unsafe_fast()        { assert_compile_and_run(&p("programs/structs/bigstructimm.vale"), "unsafe-fast", 42); }
#[test] fn bigstructimm_naive_rc()           { assert_compile_and_run(&p("programs/structs/bigstructimm.vale"), "naive-rc", 42); }
#[test] fn structmut_unsafe_fast()           { assert_compile_and_run(&p("programs/structs/structmut.vale"), "unsafe-fast", 8); }
#[test] fn structmut_naive_rc()              { assert_compile_and_run(&p("programs/structs/structmut.vale"), "naive-rc", 8); }
#[test] fn structmutstore_unsafe_fast()      { assert_compile_and_run(&p("programs/structs/structmutstore.vale"), "unsafe-fast", 42); }
#[test] fn structmutstore_naive_rc()         { assert_compile_and_run(&p("programs/structs/structmutstore.vale"), "naive-rc", 42); }
#[test] fn structmutstoreinner_unsafe_fast() { assert_compile_and_run(&p("programs/structs/structmutstoreinner.vale"), "unsafe-fast", 42); }
#[test] fn structmutstoreinner_naive_rc()    { assert_compile_and_run(&p("programs/structs/structmutstoreinner.vale"), "naive-rc", 42); }
