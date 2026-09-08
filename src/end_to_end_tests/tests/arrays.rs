use crate::end_to_end_tests::{assert_compile_and_run, assert_compile_and_run_dbg, cmd, expect, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

// A static-sized-array local walks element by element (generator makes each distinct).
#[test]
fn ssamutfromcallable() {
    assert_compile_and_run_dbg(&p("programs/arrays/ssamutfromcallable.vale"), 42, &[
        cmd("br s -p 'lldb breakpoint: ssamutfromcallable-ready' -f ssamutfromcallable.vale"),
        cmd("run"),
        expect("frame variable -P 1 a", &["[0] = 0", "[1] = 42", "[2] = 84", "[3] = 126", "[4] = 168"]),
    ]);
}
// A static-sized-array local walks its distinct literal elements.
#[test]
fn ssamutfromvalues() {
    assert_compile_and_run_dbg(&p("programs/arrays/ssamutfromvalues.vale"), 42, &[
        cmd("br s -p 'lldb breakpoint: ssamutfromvalues-ready' -f ssamutfromvalues.vale"),
        cmd("run"),
        expect("frame variable -P 1 a", &["[0] = 23", "[1] = 31", "[2] = 37", "[3] = 42", "[4] = 49"]),
    ]);
}
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: runtime-sized array (RSA) — standing order to defer RSA"]
fn rsaimm()                    { assert_compile_and_run(&p("programs/arrays/rsaimm.vale"), 3); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: runtime-sized array (RSA) — standing order to defer RSA"]
fn rsamut()                    { assert_compile_and_run(&p("programs/arrays/rsamut.vale"), 3); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: runtime-sized array (RSA) — standing order to defer RSA"]
fn rsamutdestroyintocallable() { assert_compile_and_run(&p("programs/arrays/rsamutdestroyintocallable.vale"), 42); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: group-generic-closures — borrow checker can't derive a group for a closure-captured reference (borrow_types.rs:347)"]
fn ssamutdestroyintocallable() { assert_compile_and_run(&p("programs/arrays/ssamutdestroyintocallable.vale"), 42); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: runtime-sized array (RSA) — standing order to defer RSA"]
fn rsamutlen()                 { assert_compile_and_run(&p("programs/arrays/rsamutlen.vale"), 5); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: runtime-sized array (RSA) — standing order to defer RSA"]
fn rsamutcapacity()            { assert_compile_and_run(&p("programs/arrays/rsamutcapacity.vale"), 42); }
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: runtime-sized array (RSA) — standing order to defer RSA"]
fn swaprsamutdestroy()         { assert_compile_and_run(&p("programs/arrays/swaprsamutdestroy.vale"), 42); }
