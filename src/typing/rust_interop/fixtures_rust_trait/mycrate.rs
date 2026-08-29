// A Rust dependency crate for the reverse-direction (Rust-calls-back-into-Valen) milestone.
//
// A trait a Valen struct implements, plus a generic function bounded by it. The generic function is
// monomorphized with the Valen struct as `C`, and its body calls `c.on_call()` back into the
// Valen-provided override — static dispatch, no `&dyn`.

pub trait Callback {
    fn on_call(&self) -> i32;
}

/// Rust owns this call; monomorphized to `run_callback::<MyCb>`, its `c.on_call()` dispatches
/// statically to `<MyCb as Callback>::on_call`, whose body Valen supplies.
pub fn run_callback<C: Callback>(c: &C) -> i32 {
    c.on_call()
}
