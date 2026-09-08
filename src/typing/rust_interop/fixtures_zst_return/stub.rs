// The crate the driver host compiles for the forward-direction zero-sized-struct return case.
//
// Pure forward direction (Vale calls Rust) — no callback/trait projection. `Alpha` is a re-exported
// real Rust item; only the fixed `__vale_main` root is Valen-projected.
#![feature(register_tool)]
#![register_tool(vale)]

extern crate mycrate;

use std::process::exit;

pub use mycrate::Alpha;

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
