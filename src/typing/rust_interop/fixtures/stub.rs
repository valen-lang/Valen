// The crate the driver host actually compiles.
//
// Stands in for what will eventually be the vale-stub-gen-emitted stub rlib. Its only job
// today is to force `mycrate` to be *loaded*: rustc resolves extern crates lazily, so with
// no reference at all `mycrate` would never appear in `tcx.crates(())` and the oracle would
// have nothing to walk.
//
// `pub use` rather than a bare `extern crate` on purpose — it is the shape the real stub rlib
// uses (one re-export per Vale `import rust.X.Y`, per @RTMEIZ), and it means the imported
// surface is enumerable as this crate's own module children.
//
// It also carries the rustc-collector-driven-instantiation roots: `__VALE_STUBS_MARKER` marks this
// as a Vale stub crate, and each `#[vale::emit_consumer_body]` fn is a Vale-defined item rustc's
// mono collector walks and hands to our `per_instance_mir` provider. These are inert on the typing
// path (which returns `Compilation::Stop` before codegen); the driven path (`run_case_rustc_driven`)
// compiles them. The hand-written stand-in for the eventual vale-stub-gen output.
#![feature(register_tool)]
#![register_tool(vale)]

extern crate mycrate;

use std::process::exit;

pub use mycrate::{add_two_numbers, make_counter, Counter};

pub const __VALE_STUBS_MARKER: () = ();

/// Vale's `exported func main() int`, as the Rust root rustc's collector walks. The body never runs
/// — `per_instance_mir` replaces it with the synthetic request-list body.
#[vale::emit_consumer_body]
pub fn __vale_main() -> i32 {
    unreachable!()
}

/// The bin entry shim (arch §5.6). A `--crate-type=bin` build needs a real `fn main`; libc's
/// `_start` → libstd startup → this `fn main` → `__vale_main` → Vale's real entry. Under
/// single-symbol, `__vale_main`'s ordinary Rust name resolves to the same rustc-mangled symbol
/// Vale emits its real body under (the partition filter removes the `unreachable!()` placeholder
/// above), so this is a plain call — no `extern "C"`, no link-name. The return is forwarded as the
/// process exit code (not discarded): that is the whole observable of a tier-2 assert-N run.
/// Inert in the lib driven/tier-1 builds (a `fn main` is just a function there, never linked).
fn main() {
    exit(__vale_main());
}

/// The generic drop shim (arch §15.7 "drop is a function"). Vale's synthesized drop of an imported
/// Rust type reifies `__vale_drop::<T>`, which rustc monomorphizes to `drop_in_place::<T>`. Unlike
/// `__vale_main` this is a *real* body (no `#[vale::emit_consumer_body]`) — rustc codegens it.
#[inline(never)]
pub unsafe fn __vale_drop<T>(x: *mut T) {
    core::ptr::drop_in_place(x)
}

/// Defined *here* rather than in a dependency, so nothing can import it.
///
/// The walk resolves against `tcx.crates(())` — the loaded dependency crates — so the crate being
/// compiled is out of scope. `an_item_in_the_compiled_crate_is_not_importable` pins that, and this
/// is the item it asks for.
pub fn stub_only() -> i32 {
    99
}
