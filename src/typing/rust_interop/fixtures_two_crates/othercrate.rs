// The other half of the same-short-name fixture; see `mycrate.rs`.
//
// A deliberately *different* shape behind the same short name, so that conflating the two is a
// real type error rather than a harmless aliasing of identical things.
//
// This crate also depends on `mycrate`, which is what lets it pose a **cross-crate** re-export:
// the walk has to descend a module here and land on a definition there. That is `std::vec`'s
// shape — `std` reaches `Vec` by `pub use alloc_crate::vec` — and the intra-crate re-exports in
// `fixtures/` cannot pose it. The harness builds dependency crates in sorted order, so `mycrate`
// is already built when this one compiles.

extern crate mycrate;

/// A cross-crate re-export of an **item**.
pub mod vendored {
    pub use mycrate::make_gadget;
    pub use mycrate::Gadget;
}

/// A cross-crate re-export of a **module**, so the walk descends through an alias whose target
/// lives in another crate.
pub mod toolkit {
    pub use mycrate::tools;
}

pub struct Widget {
    pub flag: bool,
}

pub fn make_other_widget() -> Widget {
    Widget { flag: true }
}

/// The non-colliding half; see `mycrate.rs`'s `Gadget`.
pub struct Doohickey {
    pub value: i32,
}

pub fn make_doohickey() -> Doohickey {
    Doohickey { value: 4 }
}

impl Doohickey {
    pub fn doohickey_value(self) -> i32 {
        self.value
    }
}
