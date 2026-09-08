// A Rust dependency crate for the reverse-direction zero-sized-struct callback RETURN gap.
//
// `Zst {}` is empty, so a callback that returns it by value returns via `PassMode::Ignore` — nothing
// crosses, and the inbound wrapper's LLVM return type is void. `run_maker` invokes the Vale callback,
// discards the returned `Zst`, and returns a fixed scalar so a valid void return is the observable.

pub struct Zst {}

impl Zst {
    pub fn new() -> Zst {
        Zst {}
    }
}

pub trait Maker {
    fn make(&self) -> Zst;
}

/// A generic caller: invokes the Vale callback (which returns a zero-sized `Zst`) and returns 8.
pub fn run_maker<M: Maker>(m: &M) -> i32 {
    let _z = m.make();
    8
}
