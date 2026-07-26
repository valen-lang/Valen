// Rust interop — the typing-pass side.
//
// Only compiled under the `rust_interop` cargo feature (i.e. the rustc-linked binary).
// Under the standalone binary this module does not exist, and every seam hook in the
// core typing files compiles out with it, so nothing references anything in here.
//
// See docs/convos/rust_interop/rust-interop-frontend-plan.md and
// docs/convos/rust_interop/rust-interop-callout-map.md.

pub mod declarations;
pub mod fixture;
pub mod importer;
pub mod logging_oracle;
pub mod oracle;
pub mod reserved;
pub mod seam;
pub mod tyctxt_oracle;

pub use fixture::{FixtureFunction, FixtureOracle};
pub use importer::{import_rust_types, rust_package_stores};
pub use logging_oracle::{LoggingOracle, OracleCall};
pub use oracle::{RustFieldInfo, RustItemId, RustKind, RustOracle, ValeSig};
pub use tyctxt_oracle::TyCtxtOracle;
pub use reserved::{citizen_id, is_rust_backed, is_rust_backed_kind, peel_refs, RUST_MODULE};
pub use seam::{maybe_rust_field, push_rust_call_candidates};
