// A Rust dependency crate for the reverse-direction zero-sized-struct callback ARGUMENT gap.
//
// `Zst {}` is empty, so a by-value `Zst` argument crosses as `PassMode::Ignore` (no C param). The
// trait method takes the ZST arg BEFORE a real scalar (`n: i32`), so an inbound wrapper that wrongly
// consumes a C param for the ZST would misalign `n` — the scalar is the observable that catches it.

pub struct Zst {}

pub trait Cb {
    fn on(&self, z: Zst, n: i32) -> i32;
}

/// A generic caller: constructs a `Zst` and hands it, plus a scalar, to the Vale callback.
pub fn run_cb<C: Cb>(c: &C) -> i32 {
    c.on(Zst {}, 42)
}
