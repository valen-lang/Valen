use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

#[test]
fn structmutfield()      { assert_compile_and_run(&p("programs/structs/structmutfield.vale"), 5); }
#[test]
#[ignore = "deferred: share (RC of a share member inside a mut struct)"]
fn memberrefcount()      { assert_compile_and_run(&p("programs/structs/memberrefcount.vale"), 5); }
#[test]
fn bigstructmutfield()   { assert_compile_and_run(&p("programs/structs/bigstructmutfield.vale"), 42); }
#[test]
fn structmut()           { assert_compile_and_run(&p("programs/structs/structmut.vale"), 8); }
#[test]
fn structmutstore()      { assert_compile_and_run(&p("programs/structs/structmutstore.vale"), 42); }
#[test]
fn structmutstoreinner() { assert_compile_and_run(&p("programs/structs/structmutstoreinner.vale"), 42); }
