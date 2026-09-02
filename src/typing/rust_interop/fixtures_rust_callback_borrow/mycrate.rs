// A Rust dependency crate for the borrow-argument reverse-direction milestone (slice 7).
//
// The trait method receives a **Rust borrow** (`&Counter`), and the Valen callback calls back *out*
// to Rust through it (`w.peek()`). So this case crosses the boundary three times: Rust -> Valen (the
// call), Rust -> Valen (the `&Counter` argument), and Valen -> Rust (the `peek` call on it).

pub struct Counter {
    value: i32,
}

impl Counter {
    pub fn new() -> Counter {
        Counter { value: 5 }
    }
    /// A `&self` borrow-receiver method the Valen callback calls back out to.
    pub fn peek(&self) -> i32 {
        self.value
    }
}

pub trait Ticker {
    fn on_tick(&self, w: &Counter) -> i32;
}

/// Rust owns the call and owns the `Counter`; it hands a borrow of it inbound to the Valen callback.
pub fn run_ticker<C: Ticker>(c: &C) -> i32 {
    let w = Counter::new();
    c.on_tick(&w)
}
