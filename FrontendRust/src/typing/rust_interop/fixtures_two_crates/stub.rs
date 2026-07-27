// The crate the host actually compiles, for the two-crate fixture.
//
// Its only job is to force both dependencies to be *loaded*: rustc resolves extern crates lazily,
// so with no reference at all neither would appear in `tcx.crates(())` and the oracle would have
// nothing to walk.
//
// Only the two constructors are re-exported, and deliberately not the types. `pub use` cannot
// bring two `Widget`s into one namespace anyway — which is Rust stating the same collision problem
// this fixture exists to pose, one level up. The oracle finds each `Widget` in its own crate's
// root regardless, since it walks `module_children` per crate rather than reading this file.

extern crate mycrate;
extern crate othercrate;

pub use mycrate::make_widget;
pub use othercrate::make_other_widget;
