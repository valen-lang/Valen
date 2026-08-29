// The crate the driver host compiles for the reverse-direction (Rust-calls-Valen-callback) case.
//
// Hand-written stand-in for the eventual vale-stub-gen output. Beyond the usual roots
// (`__VALE_STUBS_MARKER`, `__vale_main`, `__vale_drop`), it carries the two pieces the reverse
// direction needs that the forward cases never did:
//   - `MyCb` projected as a real, rustc-visible Rust type (an empty Valen struct → a ZST here), so
//     `run_callback::<MyCb>` can be named and monomorphized and MyCb's DefId lives in this crate.
//   - `impl Callback for MyCb`, whose `on_call` body is `#[vale::emit_consumer_body]` — the
//     placeholder rustc typechecks, and which Valen's backend fills with the real body. rustc
//     monomorphizing `run_callback::<MyCb>` walks `c.on_call()` to `<MyCb as Callback>::on_call`.
#![feature(register_tool)]
#![register_tool(vale)]

extern crate mycrate;

use std::process::exit;

pub use mycrate::{run_callback, Callback};

pub const __VALE_STUBS_MARKER: () = ();

/// The Valen `struct MyCb {}`, projected as a real Rust type. Empty struct → ZST; layout is read
/// from rustc, so no `repr` is needed.
pub struct MyCb {}

impl Callback for MyCb {
    #[vale::emit_consumer_body]
    fn on_call(&self) -> i32 {
        unreachable!()
    }
}

/// Vale's `exported func main() int`. `per_instance_mir` replaces this body with the synthetic
/// request-list body, which mentions `run_callback::<MyCb>` so rustc's collector queues it.
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
