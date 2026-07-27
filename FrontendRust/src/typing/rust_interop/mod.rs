// Rust interop — the typing-pass side.
//
// Only compiled under the `rust_interop` cargo feature (i.e. the rustc-linked binary).
// Under the standalone binary this module does not exist, and every seam hook in the
// core typing files compiles out with it, so nothing references anything in here.
//
// See docs/convos/rust_interop/rust-interop-frontend-plan.md and
// docs/convos/rust_interop/rust-interop-callout-map.md.

pub mod corpus;
pub mod declarations;
pub mod importer;
pub mod logging_oracle;
pub mod oracle;
pub mod reserved;
pub mod tyctxt_oracle;

pub use corpus::{Case, Expect};
pub use importer::rust_package_stores;
pub use logging_oracle::{LoggingOracle, OracleCall, OracleQuery, SigPosition, SigShape};
pub use oracle::{RustItemId, RustOracle, ValeSig};
pub use tyctxt_oracle::TyCtxtOracle;
pub use reserved::{citizen_id, is_rust_backed, is_rust_backed_kind, peel_refs, RUST_MODULE};

// `fixture.rs` lived here: a hand-written `RustOracle` answering from a canned table, so a test
// could exercise the seam without rustc. It is gone because it could not produce a `ty::Param`,
// an alias, or anything else rustc-shaped — so it structurally could not cover generics or
// projections, the surface that matters — while still needing an update on every trait change.
// Tier 1 hosts a real `TyCtxt` inside `cargo test --lib` (arch §26b.2), which is strictly more
// coverage for less maintenance, and `Oracles::none()` already says "no oracle" without an
// implementation to carry.
//
// `seam.rs` lived here: a candidate source that asked the oracle for a callee at every call
// site, plus a field hook. Both are gone rather than parked — synthesized declarations mean a
// Rust function is found by ordinary name lookup and compiled by the ordinary machinery, so
// there is nothing left for a per-call-site hook to do. Kept as a note instead of as code
// because a dead-but-constructible seam is exactly how an abandoned design gets restored by
// accident; the reasoning is in synthesized-declarations-plan.md §11.
