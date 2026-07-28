// One half of the same-short-name fixture. See `othercrate.rs` for the other, and
// `two_crates_exporting_the_same_short_name_stay_distinct` for what the pair pins.
//
// `Widget` here and `Widget` there are unrelated types that happen to share a short name — which
// is the ordinary case in Rust, not a contrivance: `new`, `len`, `Error` and `Box` recur across
// crates constantly, and Rust has no uniqueness rule for short names at all.

pub struct Widget {
    pub value: i32,
}

pub fn make_widget() -> Widget {
    Widget { value: 1 }
}

/// Takes **this** crate's `Widget`, so that handing it the other crate's is a type error.
///
/// This is what makes distinctness observable. Two same-named types both *importing* proves only
/// that the importer survived them; if Vale had conflated them into one kind, every program in the
/// positive case would still typecheck. Only a cross-crate call can tell the two apart, and it does
/// so by failing.
pub fn widget_value(w: Widget) -> i32 {
    w.value
}

/// The non-colliding half of the fixture, for `imports_from_two_crates`.
///
/// `Gadget` and `othercrate`'s `Doohickey` have distinct short names, so importing both exercises
/// two crates coexisting without also posing the collision. Which of the two questions a case asks
/// is decided purely by its allowlist — that is what "scoping is membership in the allowlist"
/// means, and it is why one fixture directory can serve both.
pub struct Gadget {
    pub value: i32,
}

pub fn make_gadget() -> Gadget {
    Gadget { value: 2 }
}

impl Gadget {
    pub fn gadget_value(self) -> i32 {
        self.value
    }
}
