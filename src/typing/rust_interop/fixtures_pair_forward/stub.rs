// The crate the driver host compiles for the forward-direction `Pair` ABI cases.
//
// Pure forward direction (Vale calls Rust) — no callback/trait projection. `Small2` and its items are
// re-exported real Rust; only the fixed `__vale_main` root is Valen-projected.
#![feature(register_tool)]
#![register_tool(vale)]

extern crate mycrate;

use std::process::exit;

pub use mycrate::{add_small, Small2};

pub const __VALE_STUBS_MARKER: () = ();

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
