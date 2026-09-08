// The crate the driver host compiles for the reverse-direction zero-sized-struct callback ARGUMENT
// case. `Zst`, `Cb`, `run_cb` are re-exported real Rust items; only `MyCb` + its `on` are
// Valen-projected. `on` takes a zero-sized `Zst` by value (an `Ignore` inbound arg) then a scalar.
#![feature(register_tool)]
#![register_tool(vale)]

extern crate mycrate;

use std::process::exit;

pub use mycrate::{run_cb, Cb, Zst};

pub const __VALE_STUBS_MARKER: () = ();

pub struct MyCb {}

impl Cb for MyCb {
    #[vale::emit_consumer_body]
    fn on(&self, _z: Zst, _n: i32) -> i32 {
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
