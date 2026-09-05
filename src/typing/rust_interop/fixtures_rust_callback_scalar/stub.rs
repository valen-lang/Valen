// The crate the driver host compiles for the scalar-argument reverse-callback case (slice 6).
//
// Like `fixtures_rust_trait/stub.rs`, but the trait method takes a scalar `i32` argument, so Rust
// hands a value inbound across the boundary in addition to the `&self` receiver. Hand-written
// stand-in for the eventual vale-stub-gen output: it projects `MyAdder` as a real rustc type and
// `impl Adder for MyAdder`, whose `add` body is a `#[vale::emit_consumer_body]` placeholder Valen's
// backend fills. rustc monomorphizing `run_adder::<MyAdder>` walks `c.add(n)` to
// `<MyAdder as Adder>::add`.
#![feature(register_tool)]
#![register_tool(vale)]

extern crate mycrate;

use std::process::exit;

pub use mycrate::{run_adder, Adder};

pub const __VALE_STUBS_MARKER: () = ();

/// The Valen `struct MyAdder {}`, projected as a real Rust type. Empty struct -> ZST.
pub struct MyAdder {}

impl Adder for MyAdder {
    #[vale::emit_consumer_body]
    fn add(&self, _n: i32) -> i32 {
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
