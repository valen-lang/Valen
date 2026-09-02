// A Rust dependency crate for the by-value-struct reverse-direction milestone (slice 8).
//
// The trait method receives a Rust struct **by value** (`Small`), so the struct's bytes cross
// Rust -> Valen in registers (a small aggregate) rather than behind a pointer. The Valen callback
// then consumes it back out to Rust via a by-value method (`sum`).

pub struct Small {
    pub a: i32,
    pub b: i32,
}

impl Small {
    /// A `&self` method the Valen callback calls on the by-value struct it received (Valen's dot-call
    /// borrows the receiver). The struct still crosses inbound *by value* — this reads it afterward.
    pub fn sum(&self) -> i32 {
        self.a + self.b
    }
}

pub trait Summer {
    fn on_sum(&self, s: Small) -> i32;
}

/// Rust owns the call and makes the `Small`, handing it inbound by value to the Valen callback.
pub fn run_summer<C: Summer>(c: &C) -> i32 {
    c.on_sum(Small { a: 3, b: 6 })
}
