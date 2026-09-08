#![allow(non_snake_case)]

use crate::end_to_end_tests::{assert_compile_and_run_dbg, cmd, expect, reject, programs_dir};

fn p(rel: &str) -> std::path::PathBuf {
    programs_dir().join(rel)
}

// The chained set-swap exchanges the two locals' values: before it a.fuel=1/b.fuel=2, after it
// a.fuel=2/b.fuel=1 (each `set` returns the old value, threaded across a and b).
#[test]
fn mutswaplocals() {
    assert_compile_and_run_dbg(&p("programs/mutswaplocals.vale"), 42, &[
        cmd("br s -p 'lldb breakpoint: mutswaplocals-before' -f mutswaplocals.vale"),
        cmd("run"),
        expect("frame variable -P 1 a", &["fuel = 1"]),
        expect("frame variable -P 1 b", &["fuel = 2"]),
        cmd("br s -p 'lldb breakpoint: mutswaplocals-after' -f mutswaplocals.vale"),
        cmd("continue"),
        expect("frame variable -P 1 a", &["fuel = 2"]),
        expect("frame variable -P 1 b", &["fuel = 1"]),
    ]);
}

// A local is tracked across a move-out + restackify: fuel 35 -> 42.
#[test]
fn restackify() {
    assert_compile_and_run_dbg(&p("programs/restackify.vale"), 42, &[
        cmd("br s -p 'lldb breakpoint: restackify-before' -f restackify.vale"),
        cmd("br s -p 'lldb breakpoint: restackify-after' -f restackify.vale"),
        cmd("run"),
        expect("frame variable -P 1 ship", &["fuel = 35"]),
        cmd("continue"),
        expect("frame variable -P 1 ship", &["fuel = 42"]),
    ]);
}

// A destructure that also restackifies leaves both locals live and inspectable.
#[test]
fn destructure_restackify() {
    assert_compile_and_run_dbg(&p("programs/destructure_restackify.vale"), 42, &[
        cmd("br s -p 'lldb breakpoint: destructure-restackify-ready' -f destructure_restackify.vale"),
        cmd("run"),
        expect("frame variable fuel", &["fuel = 42"]),
        expect("frame variable -P 1 ship", &["fuel = 42"]),
    ]);
}

// Loop-carried mutation of both the counter and a struct field across iterations.
#[test]
fn loop_restackify() {
    assert_compile_and_run_dbg(&p("programs/loop_restackify.vale"), 42, &[
        cmd("br s -p 'lldb breakpoint: loop-restackify-iter' -f loop_restackify.vale"),
        cmd("run"),
        expect("frame variable i", &["i = 0"]),
        expect("frame variable -P 1 ship", &["fuel = 27"]),
        cmd("continue"),
        expect("frame variable i", &["i = 1"]),
        expect("frame variable -P 1 ship", &["fuel = 32"]),
        cmd("continue"),
        expect("frame variable i", &["i = 2"]),
        expect("frame variable -P 1 ship", &["fuel = 37"]),
    ]);
}

// Primitive mutation is observable: the sentinel before `set x = 42` sees 73, the one after sees 42.
#[test]
fn mutlocal() {
    assert_compile_and_run_dbg(&p("programs/mutlocal.vale"), 42, &[
        cmd("br s -p 'lldb breakpoint: mutlocal-before' -f mutlocal.vale"),
        cmd("run"),
        expect("frame variable x", &["x = 73"]),
        cmd("br s -p 'lldb breakpoint: mutlocal-after' -f mutlocal.vale"),
        cmd("continue"),
        expect("frame variable x", &["x = 42"]),
    ]);
}

// A borrow-ref local's pointee struct walks its fields.
#[test]
fn constraintRef() {
    assert_compile_and_run_dbg(&p("programs/constraintRef.vale"), 8, &[
        cmd("br s -p 'lldb breakpoint: constraintRef-ready' -f constraintRef.vale"),
        cmd("run"),
        expect("frame variable -P 1 carrier", &["hp = 400", "interceptors = 8"]),
    ]);
}

// The reused local ID doesn't corrupt playerRow: the sentinel sits after `playerRow = 4`, so it's
// live and reads 4 (if the Unstackify local-ID-reuse bug returned, this value would be wrong).
#[test]
fn unstackifyret() {
    assert_compile_and_run_dbg(&p("programs/unstackifyret.vale"), 42, &[
        cmd("br s -p 'lldb breakpoint: unstackifyret-set' -f unstackifyret.vale"),
        cmd("run"),
        expect("frame variable playerRow", &["playerRow = 4"]),
    ]);
}

// The live `return 42` (breakpoint 1) is reached; the unreachable trailing `__vbi_panic`
// (breakpoint 2) is never entered — continuing runs straight to exit 42.
#[test]
fn unreachablemoot() {
    assert_compile_and_run_dbg(&p("programs/unreachablemoot.vale"), 42, &[
        cmd("br s -p 'lldb breakpoint: unreachablemoot-live' -f unreachablemoot.vale"),
        cmd("br s -p 'lldb breakpoint: unreachablemoot-dead' -f unreachablemoot.vale"),
        expect("run", &["stop reason = breakpoint 1"]),
        reject("continue", &["exited with status = 42"], &["stop reason = breakpoint 2"]),
    ]);
}

// The panic fires: execution stops at the panic call site (breakpoint 1), and continuing never
// reaches the code after it (breakpoint 2) — the panic halts the program (exit 1, via run()).
#[test]
fn panic() {
    assert_compile_and_run_dbg(&p("programs/panic.vale"), 1, &[
        cmd("br s -p 'lldb breakpoint: panic-site' -f panic.vale"),
        cmd("br s -p 'lldb breakpoint: panic-after' -f panic.vale"),
        expect("run", &["stop reason = breakpoint 1"]),
        reject("continue", &[], &["stop reason = breakpoint 2"]),
    ]);
}

// The guarded panic inside `if (false)` (breakpoint 2) never fires; execution reaches the return
// (breakpoint 1) and continues to exit 42.
#[test]
fn panicnot() {
    assert_compile_and_run_dbg(&p("programs/panicnot.vale"), 42, &[
        cmd("br s -p 'lldb breakpoint: panicnot-return' -f panicnot.vale"),
        cmd("br s -p 'lldb breakpoint: panicnot-dead' -f panicnot.vale"),
        expect("run", &["stop reason = breakpoint 1"]),
        reject("continue", &["exited with status = 42"], &["stop reason = breakpoint 2"]),
    ]);
}

// From inside two nested blocks, outer- and enclosing-block locals are all in scope.
#[test]
fn nestedblocks() {
    assert_compile_and_run_dbg(&p("programs/nestedblocks.vale"), 42, &[
        cmd("br s -p 'lldb breakpoint: nestedblocks-inner' -f nestedblocks.vale"),
        cmd("run"),
        expect("frame variable originalIndex", &["originalIndex = 9"]),
        expect("frame variable i", &["i = 1"]),
        expect("frame variable neighborIndex", &["neighborIndex = 10"]),
    ]);
}
