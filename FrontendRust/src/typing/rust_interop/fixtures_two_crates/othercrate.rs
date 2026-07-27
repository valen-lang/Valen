// The other half of the same-short-name fixture; see `mycrate.rs`.
//
// A deliberately *different* shape behind the same short name, so that conflating the two is a
// real type error rather than a harmless aliasing of identical things.

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
