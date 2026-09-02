// The crate the driver host compiles for the two-imported-params reverse-callback repro.
//
// `Alpha`, `Beta`, `Cb`, `run_cb` are re-exported real Rust items; only `MyCb` + its `go` are
// Valen-projected. `go` takes two imported-type borrow params and returns void.
#![feature(register_tool)]
#![register_tool(vale)]

extern crate mycrate;

use std::process::exit;

pub use mycrate::{Alpha, Beta, Cb};

pub const __VALE_STUBS_MARKER: () = ();

pub struct MyCb {}

impl Cb for MyCb {
    #[vale::emit_consumer_body]
    fn go(&self, _x: &Alpha, _y: &Beta) {
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
