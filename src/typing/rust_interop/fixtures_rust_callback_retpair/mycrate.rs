// A Rust dependency crate for the inbound Pair-**return** reverse-direction milestone (slice 8d).
//
// The trait method *returns* a small `{i32,i32}` struct by value. A Valen struct implements it; Rust's
// generic caller invokes it and reads the returned struct (`c.make().sum()`), so the struct crosses
// Valen -> Rust in two registers.

pub struct Small {
    pub a: i32,
    pub b: i32,
}

impl Small {
    pub fn new(a: i32, b: i32) -> Small {
        Small { a, b }
    }
    pub fn sum(&self) -> i32 {
        self.a + self.b
    }
}

pub trait Maker {
    fn make(&self) -> Small;
}

/// Rust owns the call and reads the returned struct.
pub fn run_maker<C: Maker>(c: &C) -> i32 {
    c.make().sum()
}
