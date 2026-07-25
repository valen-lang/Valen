// A dependency crate that does NOT export `add_two_numbers`.
//
// Used to exercise the driver's *failure* path: with the function absent, the Vale program
// cannot typecheck, `check` panics, and we get to see how that failure is presented. That
// presentation is the whole point of asserting after `run_compiler` returns and restoring the
// default panic hook first — without both, an ordinary assertion failure is reported as
// "the compiler unexpectedly panicked. this is a bug" with an ICE dump.

pub fn some_other_function(a: i32) -> i32 {
    a
}
