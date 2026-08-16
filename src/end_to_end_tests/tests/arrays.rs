use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn ssamutfromcallable()        { assert_compile_and_run(&p("programs/arrays/ssamutfromcallable.vale"), 42); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn ssamutfromvalues()          { assert_compile_and_run(&p("programs/arrays/ssamutfromvalues.vale"), 42); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn rsaimm()                    { assert_compile_and_run(&p("programs/arrays/rsaimm.vale"), 3); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn rsamut()                    { assert_compile_and_run(&p("programs/arrays/rsamut.vale"), 3); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn rsamutdestroyintocallable() { assert_compile_and_run(&p("programs/arrays/rsamutdestroyintocallable.vale"), 42); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn ssamutdestroyintocallable() { assert_compile_and_run(&p("programs/arrays/ssamutdestroyintocallable.vale"), 42); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn rsamutlen()                 { assert_compile_and_run(&p("programs/arrays/rsamutlen.vale"), 5); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn rsamutcapacity()            { assert_compile_and_run(&p("programs/arrays/rsamutcapacity.vale"), 42); }
#[test]
#[ignore = "deferred at experimental-2 squash baseline"]
fn swaprsamutdestroy()         { assert_compile_and_run(&p("programs/arrays/swaprsamutdestroy.vale"), 42); }
