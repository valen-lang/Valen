// Companion to the `fixtures_missing/mycrate.rs` failure fixture. Same shape as the real
// stub, re-exporting whatever that crate does export, so the only difference between the two
// fixture sets is whether `add_two_numbers` exists.

extern crate mycrate;

pub use mycrate::some_other_function;
