// The query services the typing pass can consult.
//
// This exists to keep `#[cfg]` out of the typing pass. Threading a cfg-gated oracle field
// through `Compiler` and `TypingPassCompilation` meant a gate on every field, every
// constructor parameter, every initializer, and a cfg-diverged pair of otherwise-identical
// constructor calls — about ten gates for one optional service. Bundling them here moves all
// of that into one definition: `Compiler` holds a plain `Oracles` with no gate, every
// constructor takes one with no gate, and non-interop call sites say `Oracles::none()`, which
// compiles in both configurations.
//
// The containment rule still holds: with the feature off there is no field, nothing here
// names `RustOracle`, and the standalone build references the interop module nowhere.
//
// This is also where a second oracle goes when one arrives — the architecture doc's inbound
// seam is a separate service with a different shape — and adding it costs one field here
// rather than another pass of gates through the constructors.

#[cfg(feature = "rust_interop")]
use crate::typing::rust_interop::RustOracle;

#[cfg(not(feature = "rust_interop"))]
use std::marker::PhantomData;

/// Miscellaneous (see @TFITCX)
#[derive(Clone, Copy)]
pub struct Oracles<'ctx, 's, 't>
where 's: 't,
{
    /// Answers questions about Rust items. `None` when nothing is being asked — a test about
    /// Vale semantics, or a compilation with no Rust dependencies — in which case every seam
    /// falls through to ordinary Vale behavior.
    #[cfg(feature = "rust_interop")]
    pub rust: Option<&'ctx dyn RustOracle<'s, 't>>,

    /// Keeps the lifetime parameters used when there are no fields to use them.
    #[cfg(not(feature = "rust_interop"))]
    _marker: PhantomData<(&'ctx (), &'s (), &'t ())>,
}

impl<'ctx, 's, 't> Oracles<'ctx, 's, 't>
where 's: 't,
{
    /// No oracles. Compiles in both configurations, so ordinary callers never mention the
    /// build mode.
    pub fn none() -> Self {
        Oracles {
            #[cfg(feature = "rust_interop")]
            rust: None,
            #[cfg(not(feature = "rust_interop"))]
            _marker: PhantomData,
        }
    }

    #[cfg(feature = "rust_interop")]
    pub fn with_rust(rust: &'ctx dyn RustOracle<'s, 't>) -> Self {
        Oracles { rust: Some(rust) }
    }
}

impl<'ctx, 's, 't> Default for Oracles<'ctx, 's, 't>
where 's: 't,
{
    fn default() -> Self {
        Oracles::none()
    }
}
