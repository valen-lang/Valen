use crate::end_to_end_tests::{assert_compile_and_run, assert_compile_and_run_dbg, cmd, expect, reject, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

// The debugger tracks branch selection: with a sentinel in BOTH branches (taken = breakpoint 1,
// untaken = breakpoint 2), execution stops at the taken one, and continuing runs to exit without
// ever hitting the untaken one. (Exit code 42 alone can't show this — it's the same either way in
// programs that return the same value; here it differs, but the point is the debugger's branch view.)
#[test]
fn ifelse() {
    assert_compile_and_run_dbg(&p("programs/if/if.vale"), 42, &[
        cmd("br s -p 'lldb breakpoint: if-then' -f if.vale"),
        cmd("br s -p 'lldb breakpoint: if-else' -f if.vale"),
        expect("run", &["stop reason = breakpoint 1"]),
        reject("continue", &["exited with status = 42"], &["stop reason = breakpoint 2"]),
    ]);
}
// VDBG: no debugger gate yet — kind deferred (see #[ignore])
#[test]
#[ignore = "deferred: interface/upcast/downcast"]
fn upcastif() { assert_compile_and_run(&p("programs/if/upcastif.vale"), 42); }
// The taken `return 42` arm is entered and returns; the dead `return 73` arm is never reached.
#[test]
fn ifnevers() {
    assert_compile_and_run_dbg(&p("programs/if/ifnevers.vale"), 42, &[
        cmd("br s -p 'lldb breakpoint: ifnevers-then' -f ifnevers.vale"),
        cmd("br s -p 'lldb breakpoint: ifnevers-else' -f ifnevers.vale"),
        expect("run", &["stop reason = breakpoint 1"]),
        reject("continue", &["exited with status = 42"], &["stop reason = breakpoint 2"]),
    ]);
}
// The middle `else if (true)` branch is selected (breakpoint 1), the final `else` (breakpoint 2) is
// never entered — guards the historical wrong-phi bug (wrong value from the right block).
#[test]
fn nestedif() {
    assert_compile_and_run_dbg(&p("programs/if/nestedif.vale"), 42, &[
        cmd("br s -p 'lldb breakpoint: nestedif-mid' -f nestedif.vale"),
        cmd("br s -p 'lldb breakpoint: nestedif-else' -f nestedif.vale"),
        expect("run", &["stop reason = breakpoint 1"]),
        reject("continue", &["exited with status = 42"], &["stop reason = breakpoint 2"]),
    ]);
}
