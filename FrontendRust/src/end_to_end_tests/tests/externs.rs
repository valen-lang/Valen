use crate::end_to_end_tests::{assert_compile_and_run_with_c, programs_dir};

fn run(dir_rel: &str, expected: i32) {
    let dir = programs_dir().join(dir_rel);
    // `native/test.c` is auto-discovered by the Frontend-driven walker in
    // pass_manager::build; no need to pass it via extra_c.
    assert_compile_and_run_with_c(&dir, &[], expected);
}

#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn interfacemutreturnexport() { run("programs/externs/interfacemutreturnexport", 42); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn interfacemutparamexport()  { run("programs/externs/interfacemutparamexport", 42); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn structmutreturnexport()    { run("programs/externs/structmutreturnexport", 42); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn structmutparamexport()     { run("programs/externs/structmutparamexport", 42); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn structmutparamdeepexport() { run("programs/externs/structmutparamdeepexport", 42); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn rsamutparamexport()        { run("programs/externs/rsamutparamexport", 10); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn rsamutreturnexport()       { run("programs/externs/rsamutreturnexport", 42); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn ssamutparamexport()        { run("programs/externs/ssamutparamexport", 10); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn ssamutreturnexport()       { run("programs/externs/ssamutreturnexport", 42); }
