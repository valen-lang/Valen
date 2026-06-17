use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

#[test] fn stradd_unsafe_fast()   { assert_compile_and_run(&p("programs/strings/stradd.vale"), "unsafe-fast", 42); }
#[test] fn stradd_naive_rc()      { assert_compile_and_run(&p("programs/strings/stradd.vale"), "naive-rc", 42); }
#[test] fn strneq_unsafe_fast()   { assert_compile_and_run(&p("programs/strings/strneq.vale"), "unsafe-fast", 42); }
#[test] fn strneq_naive_rc()      { assert_compile_and_run(&p("programs/strings/strneq.vale"), "naive-rc", 42); }
#[test] fn strprint_unsafe_fast() { assert_compile_and_run(&p("programs/strings/strprint.vale"), "unsafe-fast", 42); }
#[test] fn strprint_naive_rc()    { assert_compile_and_run(&p("programs/strings/strprint.vale"), "naive-rc", 42); }
#[test] fn inttostr_unsafe_fast() { assert_compile_and_run(&p("programs/strings/inttostr.vale"), "unsafe-fast", 4); }
#[test] fn inttostr_naive_rc()    { assert_compile_and_run(&p("programs/strings/inttostr.vale"), "naive-rc", 4); }
#[test] fn i64tostr_unsafe_fast() { assert_compile_and_run(&p("programs/strings/i64tostr.vale"), "unsafe-fast", 4); }
#[test] fn i64tostr_naive_rc()    { assert_compile_and_run(&p("programs/strings/i64tostr.vale"), "naive-rc", 4); }
