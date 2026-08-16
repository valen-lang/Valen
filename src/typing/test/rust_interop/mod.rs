// Tier-1 typing-pass tests for Rust interop: Vale's typing pass driven by a real `TyCtxt`.
// The whole subtree is gated, so under the standalone binary none of it exists.
//
// There is deliberately **no fixture oracle**. A hand-written fake cannot produce a `ty::Param`,
// an alias, or anything else rustc-shaped, so it structurally cannot cover generics, projections
// or inherited impl parameters — the surface that actually matters — while still needing an
// update every time the `RustOracle` trait changes. Arch §26b.3.

mod cases;
mod harness;
