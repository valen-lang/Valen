// The crate the driver host compiles for the reverse-direction zero-sized-struct callback RETURN
// case. `Zst`, `Maker`, `run_maker` are re-exported real Rust items; only `MyMaker` + its `make` are
// Valen-projected. `make` returns a zero-sized `Zst` by value (an `Ignore` return — nothing crosses).
#![feature(register_tool)]
#![register_tool(vale)]

extern crate mycrate;

use std::process::exit;

pub use mycrate::{run_maker, Maker, Zst};

pub const __VALE_STUBS_MARKER: () = ();

pub struct MyMaker {}

impl Maker for MyMaker {
    #[vale::emit_consumer_body]
    fn make(&self) -> Zst {
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
