// The crate the driver host compiles for the borrow-argument reverse-callback case (slice 7).
//
// Like `fixtures_rust_callback_scalar/stub.rs`, but the trait method receives a Rust borrow
// (`&Counter`) that the Valen callback calls back out to (`w.peek()`). `Counter`, `Ticker`, and
// `run_ticker` are re-exported real Rust items; only `MyTicker` + its `on_tick` are Valen-projected.
#![feature(register_tool)]
#![register_tool(vale)]

extern crate mycrate;

use std::process::exit;

pub use mycrate::{run_ticker, Counter, Ticker};

pub const __VALE_STUBS_MARKER: () = ();

/// The Valen `struct MyTicker {}`, projected as a real Rust type. Empty struct -> ZST.
pub struct MyTicker {}

impl Ticker for MyTicker {
    #[vale::emit_consumer_body]
    fn on_tick(&self, _w: &Counter) -> i32 {
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
