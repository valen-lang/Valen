// A Rust dependency crate for the scalar-argument reverse-direction milestone (slice 6).
//
// Like `fixtures_rust_trait`, but the trait method takes a scalar `i32` argument. A Valen struct
// implements the trait; a generic Rust function is monomorphized with it and passes the argument
// inbound across the boundary — the first time a *value* crosses Rust->Valen (the `&self`-only
// callback never did).

pub trait Adder {
    fn add(&self, n: i32) -> i32;
}

/// Rust owns this call; monomorphized to `run_adder::<MyAdder>`, its `c.add(n)` dispatches statically
/// to `<MyAdder as Adder>::add`, whose body Valen supplies, passing the scalar `n` inbound.
pub fn run_adder<C: Adder>(c: &C, n: i32) -> i32 {
    c.add(n)
}
