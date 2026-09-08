// A Rust dependency crate for the forward-direction zero-sized-struct return gap.
//
// `Alpha {}` is an empty (zero-sized) struct. rustc classifies a by-value ZST return as
// `PassMode::Ignore`, so `Alpha::new()` exercises the interop extern's Ignore-return coercion —
// nothing crosses the ABI, and Vale must synthesize the empty value on its own side.

pub struct Alpha {}

impl Alpha {
    /// Returns a zero-sized `Alpha` by value — an `Ignore`-return coercion.
    pub fn new() -> Alpha {
        Alpha {}
    }
}
