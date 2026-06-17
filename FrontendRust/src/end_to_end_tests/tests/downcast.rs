#![allow(non_snake_case)]

use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

#[test] fn downcastBorrowSuccessful_unsafe_fast() { assert_compile_and_run(&p("programs/downcast/downcastBorrowSuccessful.vale"), "unsafe-fast", 42); }
#[test] fn downcastBorrowSuccessful_naive_rc()    { assert_compile_and_run(&p("programs/downcast/downcastBorrowSuccessful.vale"), "naive-rc", 42); }
#[test] fn downcastBorrowFailed_unsafe_fast()     { assert_compile_and_run(&p("programs/downcast/downcastBorrowFailed.vale"), "unsafe-fast", 42); }
#[test] fn downcastBorrowFailed_naive_rc()        { assert_compile_and_run(&p("programs/downcast/downcastBorrowFailed.vale"), "naive-rc", 42); }
#[test] fn downcastOwningSuccessful_unsafe_fast() { assert_compile_and_run(&p("programs/downcast/downcastOwningSuccessful.vale"), "unsafe-fast", 42); }
#[test] fn downcastOwningSuccessful_naive_rc()    { assert_compile_and_run(&p("programs/downcast/downcastOwningSuccessful.vale"), "naive-rc", 42); }
#[test] fn downcastOwningFailed_unsafe_fast()     { assert_compile_and_run(&p("programs/downcast/downcastOwningFailed.vale"), "unsafe-fast", 42); }
#[test] fn downcastOwningFailed_naive_rc()        { assert_compile_and_run(&p("programs/downcast/downcastOwningFailed.vale"), "naive-rc", 42); }
