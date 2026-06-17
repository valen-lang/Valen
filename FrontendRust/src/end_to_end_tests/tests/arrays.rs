use crate::end_to_end_tests::{assert_compile_and_run, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

#[test] fn ssamutfromcallable_unsafe_fast()        { assert_compile_and_run(&p("programs/arrays/ssamutfromcallable.vale"), "unsafe-fast", 42); }
#[test] fn ssamutfromcallable_naive_rc()           { assert_compile_and_run(&p("programs/arrays/ssamutfromcallable.vale"), "naive-rc", 42); }
#[test] fn ssamutfromvalues_unsafe_fast()          { assert_compile_and_run(&p("programs/arrays/ssamutfromvalues.vale"), "unsafe-fast", 42); }
#[test] fn ssamutfromvalues_naive_rc()             { assert_compile_and_run(&p("programs/arrays/ssamutfromvalues.vale"), "naive-rc", 42); }
#[test] fn rsaimm_unsafe_fast()                    { assert_compile_and_run(&p("programs/arrays/rsaimm.vale"), "unsafe-fast", 3); }
#[test] fn rsaimm_naive_rc()                       { assert_compile_and_run(&p("programs/arrays/rsaimm.vale"), "naive-rc", 3); }
#[test] fn rsamut_unsafe_fast()                    { assert_compile_and_run(&p("programs/arrays/rsamut.vale"), "unsafe-fast", 3); }
#[test] fn rsamut_naive_rc()                       { assert_compile_and_run(&p("programs/arrays/rsamut.vale"), "naive-rc", 3); }
#[test] fn rsamutdestroyintocallable_unsafe_fast() { assert_compile_and_run(&p("programs/arrays/rsamutdestroyintocallable.vale"), "unsafe-fast", 42); }
#[test] fn rsamutdestroyintocallable_naive_rc()    { assert_compile_and_run(&p("programs/arrays/rsamutdestroyintocallable.vale"), "naive-rc", 42); }
#[test] fn ssamutdestroyintocallable_unsafe_fast() { assert_compile_and_run(&p("programs/arrays/ssamutdestroyintocallable.vale"), "unsafe-fast", 42); }
#[test] fn ssamutdestroyintocallable_naive_rc()    { assert_compile_and_run(&p("programs/arrays/ssamutdestroyintocallable.vale"), "naive-rc", 42); }
#[test] fn rsamutlen_unsafe_fast()                 { assert_compile_and_run(&p("programs/arrays/rsamutlen.vale"), "unsafe-fast", 5); }
#[test] fn rsamutlen_naive_rc()                    { assert_compile_and_run(&p("programs/arrays/rsamutlen.vale"), "naive-rc", 5); }
#[test] fn rsamutcapacity_unsafe_fast()            { assert_compile_and_run(&p("programs/arrays/rsamutcapacity.vale"), "unsafe-fast", 42); }
#[test] fn rsamutcapacity_naive_rc()               { assert_compile_and_run(&p("programs/arrays/rsamutcapacity.vale"), "naive-rc", 42); }
#[test] fn swaprsamutdestroy_unsafe_fast()         { assert_compile_and_run(&p("programs/arrays/swaprsamutdestroy.vale"), "unsafe-fast", 42); }
#[test] fn swaprsamutdestroy_naive_rc()            { assert_compile_and_run(&p("programs/arrays/swaprsamutdestroy.vale"), "naive-rc", 42); }
