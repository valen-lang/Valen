use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn stradd()   { assert_compile_and_run(&p("programs/strings/stradd.vale"), 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn strneq()   { assert_compile_and_run(&p("programs/strings/strneq.vale"), 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn strprint() { assert_compile_and_run(&p("programs/strings/strprint.vale"), 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn inttostr() { assert_compile_and_run(&p("programs/strings/inttostr.vale"), 4); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn i64tostr() { assert_compile_and_run(&p("programs/strings/i64tostr.vale"), 4); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn floattostr()      { assert_compile_and_run(&p("programs/strings/floattostr.vale"), 9); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn strcmp()          { assert_compile_and_run(&p("programs/strings/strcmp.vale"), 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn substring()       { assert_compile_and_run(&p("programs/strings/substring.vale"), 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn strindexof()      { assert_compile_and_run(&p("programs/strings/strindexof.vale"), 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn strtoascii()      { assert_compile_and_run(&p("programs/strings/strtoascii.vale"), 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn strfromascii()    { assert_compile_and_run(&p("programs/strings/strfromascii.vale"), 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn stradd_empty()    { assert_compile_and_run(&p("programs/strings/stradd_empty.vale"), 42); }
#[test] #[ignore = "deferred: borrow-shape backend arc (vcoord Phase 2 / *int_ptr)"] fn stradd_chained()  { assert_compile_and_run(&p("programs/strings/stradd_chained.vale"), 42); }
