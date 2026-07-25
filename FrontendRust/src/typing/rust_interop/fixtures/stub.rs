// The crate the driver host actually compiles.
//
// Stands in for what will eventually be the vale-stub-gen-emitted stub rlib. Its only job
// today is to force `mycrate` to be *loaded*: rustc resolves extern crates lazily, so with
// no reference at all `mycrate` would never appear in `tcx.crates(())` and the oracle would
// have nothing to walk.
//
// `pub use` rather than a bare `extern crate` on purpose — it is the shape the real stub rlib
// uses (one re-export per Vale `import rust.X.Y`, per @RTMEIZ), and it means the imported
// surface is enumerable as this crate's own module children.

extern crate mycrate;

pub use mycrate::{add_two_numbers, make_counter, Counter};
