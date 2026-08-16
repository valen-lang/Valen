// A stub crate that does not parse, on purpose.
//
// This is the crate the *in-process* `run_compiler` builds, so a hard error here happens inside
// our test binary — the one hazard that survives hosting rustc in `cargo test --lib`. rustc's
// fatal path exits the process rather than returning, which would take the whole suite down
// rather than failing one test.
//
// A **parse** error specifically, not a type error. `after_expansion` runs before type checking,
// and the callback returns `Compilation::Stop`, so a type error would never be reached — rustc
// halts at our request first. Parsing happens before expansion, so this is the earliest failure
// that can actually reach us.

extern crate mycrate;

pub use mycrate::add_two_numbers;

pub fn definitely_not_valid( -> {
