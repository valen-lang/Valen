// The dependency crate for the fatal-rustc-error fixture. Deliberately *valid* — the breakage
// belongs in `stub.rs`, which is the crate our in-process `run_compiler` compiles.
//
// `mycrate.rs` is built by a separate `rustc` subprocess, so an error here would only fail that
// subprocess and surface as an ordinary assertion. That would test the wrong thing.

pub fn add_two_numbers(a: i32, b: i32) -> i32 {
    a + b
}
