// The crate the driver host compiles for the inbound Pair-return reverse-callback case (slice 8d).
//
// Like the other reverse-callback stubs, but the trait method *returns* a small `{i32,i32}` struct by
// value, which crosses Valen -> Rust in two registers. `Small`, `Maker`, `run_maker` are re-exported
// real Rust; only `MyMaker` + its `make` are Valen-projected.
#![feature(register_tool)]
#![register_tool(vale)]

extern crate mycrate;

use std::process::exit;

pub use mycrate::{run_maker, Maker, Small};

pub const __VALE_STUBS_MARKER: () = ();

/// The Valen `struct MyMaker {}`, projected as a real Rust type. Empty struct -> ZST.
pub struct MyMaker {}

impl Maker for MyMaker {
    #[vale::emit_consumer_body]
    fn make(&self) -> Small {
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
