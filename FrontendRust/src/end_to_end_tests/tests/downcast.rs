#![allow(non_snake_case)]

use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

#[test] fn downcastBorrowSuccessful() { assert_compile_and_run(&p("programs/downcast/downcastBorrowSuccessful.vale"), 42); }
#[test] fn downcastBorrowFailed()     { assert_compile_and_run(&p("programs/downcast/downcastBorrowFailed.vale"), 42); }
#[test] fn downcastOwningSuccessful() { assert_compile_and_run(&p("programs/downcast/downcastOwningSuccessful.vale"), 42); }
#[test] fn downcastOwningFailed()     { assert_compile_and_run(&p("programs/downcast/downcastOwningFailed.vale"), 42); }
