use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: share"]
fn stradd()   { assert_compile_and_run(&p("programs/strings/stradd.vale"), 42); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: share"]
fn strneq()   { assert_compile_and_run(&p("programs/strings/strneq.vale"), 42); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: share"]
fn strprint() { assert_compile_and_run(&p("programs/strings/strprint.vale"), 42); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: share"]
fn inttostr() { assert_compile_and_run(&p("programs/strings/inttostr.vale"), 4); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: share"]
fn i64tostr() { assert_compile_and_run(&p("programs/strings/i64tostr.vale"), 4); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: share"]
fn floattostr()      { assert_compile_and_run(&p("programs/strings/floattostr.vale"), 9); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: share"]
fn strcmp()          { assert_compile_and_run(&p("programs/strings/strcmp.vale"), 42); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: share"]
fn substring()       { assert_compile_and_run(&p("programs/strings/substring.vale"), 42); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: share"]
fn strindexof()      { assert_compile_and_run(&p("programs/strings/strindexof.vale"), 42); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: share"]
fn strtoascii()      { assert_compile_and_run(&p("programs/strings/strtoascii.vale"), 42); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: share"]
fn strfromascii()    { assert_compile_and_run(&p("programs/strings/strfromascii.vale"), 42); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: share"]
fn stradd_empty()    { assert_compile_and_run(&p("programs/strings/stradd_empty.vale"), 42); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: share"]
fn stradd_chained()  { assert_compile_and_run(&p("programs/strings/stradd_chained.vale"), 42); }
