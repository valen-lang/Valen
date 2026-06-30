use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn stradd()   { assert_compile_and_run(&p("programs/strings/stradd.vale"), 42); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn strneq()   { assert_compile_and_run(&p("programs/strings/strneq.vale"), 42); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn strprint() { assert_compile_and_run(&p("programs/strings/strprint.vale"), 42); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn inttostr() { assert_compile_and_run(&p("programs/strings/inttostr.vale"), 4); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn i64tostr() { assert_compile_and_run(&p("programs/strings/i64tostr.vale"), 4); }
