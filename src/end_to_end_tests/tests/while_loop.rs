use crate::end_to_end_tests::{assert_compile_and_run_dbg, cmd, expect, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

// The loop counter increments across iterations while the body line is re-hit.
#[test]
fn while_loop() {
    assert_compile_and_run_dbg(&p("programs/while/while.vale"), 42, &[
        cmd("br s -p 'lldb breakpoint: while-iter' -f while.vale"),
        cmd("run"),
        expect("frame variable a", &["a = 1"]),
        cmd("continue"),
        expect("frame variable a", &["a = 2"]),
        cmd("continue"),
        expect("frame variable a", &["a = 3"]),
    ]);
}
