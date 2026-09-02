// A Rust dependency crate for the forward-direction (Vale calls Rust) `Pair` ABI milestone.
//
// `Small2 { a: i32, b: i32 }` is an 8-byte two-scalar struct that crosses as `PassMode::Pair`. It is
// both **returned** by value (`new`) and taken **by value** as an argument (`add_small`), so Vale
// exercises the outbound Pair return and Pair argument paths.

pub struct Small2 {
    pub a: i32,
    pub b: i32,
}

impl Small2 {
    /// Returns a `Small2` by value — an outbound Pair **return**.
    pub fn new(a: i32, b: i32) -> Small2 {
        Small2 { a, b }
    }
    /// A `&self` reader (a borrow, not a Pair) — used to observe a received/constructed `Small2`.
    pub fn sum(&self) -> i32 {
        self.a + self.b
    }
}

/// Takes a `Small2` by value — an outbound Pair **argument**.
pub fn add_small(s: Small2) -> i32 {
    s.a + s.b
}
