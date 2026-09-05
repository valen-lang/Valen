// The crate the driver host compiles for the by-value-struct reverse-callback case (slice 8).
//
// Like `fixtures_rust_callback_borrow/stub.rs`, but the trait method receives a Rust struct **by
// value** (`Small`), which crosses inbound in registers. `Small`, `Summer`, and `run_summer` are
// re-exported real Rust items; only `MySummer` + its `on_sum` are Valen-projected.
#![feature(register_tool)]
#![register_tool(vale)]

extern crate mycrate;

use std::process::exit;

pub use mycrate::{run_summer, Small, Summer};

pub const __VALE_STUBS_MARKER: () = ();

/// The Valen `struct MySummer {}`, projected as a real Rust type. Empty struct -> ZST.
pub struct MySummer {}

impl Summer for MySummer {
    #[vale::emit_consumer_body]
    fn on_sum(&self, _s: Small) -> i32 {
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
