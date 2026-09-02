// The crate the driver host compiles for the main-loop capstone (slice 9).
//
// Rust's `main_loop` owns a loop that calls the Valen callback once per iteration. `Looper` and
// `main_loop` are re-exported real Rust items; only `MyCb` + its `on_tick` are Valen-projected. The
// callback wrapper is emitted once and invoked N times by the loop.
#![feature(register_tool)]
#![register_tool(vale)]

extern crate mycrate;

use std::process::exit;

pub use mycrate::{main_loop, Looper};

pub const __VALE_STUBS_MARKER: () = ();

/// The Valen `struct MyCb {}`, projected as a real Rust type. Empty struct -> ZST.
pub struct MyCb {}

impl Looper for MyCb {
    #[vale::emit_consumer_body]
    fn on_tick(&self, _i: i32) -> i32 {
        unreachable!()
    }
}

#[vale::emit_consumer_body]
pub fn __vale_main() -> i32 {
    unreachable!()
}

fn main() {
    exit(__vale_main());
}

#[inline(never)]
pub unsafe fn __vale_drop<T>(x: *mut T) {
    core::ptr::drop_in_place(x)
}
