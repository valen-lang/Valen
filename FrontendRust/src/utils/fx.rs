// Project-wide hash containers that default to FxBuildHasher (rustc_hash) rather
// than std's RandomState (SipHash-1-3). The compiler only ever hashes its own
// internal types — IRuneS, IdT, IImpreciseNameS, FileCoordinate, etc. — so the
// DoS-resistance of SipHash buys nothing and the per-hash cost shows up
// prominently in profiles. FxBuildHasher is the same hasher rustc itself uses
// for the same reason.
//
// These are type aliases, so calls that worked on `std::collections::HashMap<K, V>`
// generally work here — except `::new()`, which std only impls for RandomState.
// Use `::default()` or `::with_capacity_and_hasher(n, Default::default())`.

pub use rustc_hash::FxBuildHasher;

pub type HashMap<K, V> = std::collections::HashMap<K, V, FxBuildHasher>;
pub type HashSet<T> = std::collections::HashSet<T, FxBuildHasher>;
pub type IndexMap<K, V> = indexmap::IndexMap<K, V, FxBuildHasher>;
pub type IndexSet<T> = indexmap::IndexSet<T, FxBuildHasher>;
