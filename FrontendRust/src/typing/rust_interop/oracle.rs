// The oracle seam: the one interface through which the typing pass asks questions
// about Rust items.
//
// INVARIANT: no rustc type appears in any signature here — no `TyCtxt`, no `'tcx`,
// no `DefId`, no `Ty`. Every input and output is Vale-owned. That is what keeps the
// typing pass free of `#![feature(rustc_private)]`; a rustc type in this file would
// leak the nightly-internals dependency straight into the core IR.
//
// The seam is deliberately PER-QUESTION. There is no `struct_def`/`interface_def`
// query, because a Rust type has no Vale `StructDefinitionT` — it is not a definition
// we hold, it is a thing we can ask specific questions about.
//
// See docs/convos/rust_interop/vale-rust-interop-architecture.md §8.10 and
// docs/convos/rust_interop/rust-interop-frontend-plan.md §5.

use crate::interner::StrI;
use crate::typing::names::names::IdT;
use crate::typing::types::types::*;
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::PackageCoordinate;

/// An opaque handle to a resolved Rust item. Valid only within one invocation —
/// never serialized, never stored in `HinputsT`. The durable identity of a Rust item
/// is its path, carried by the `rust`-packaged `IdT`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RustItemId(pub u32);

/// One position in a Rust signature: either a settled type, or a reference to one of the
/// function's own generic parameters.
///
/// The second arm is what makes a generic Rust function representable at all. A signature is read
/// **once**, structurally, and a `fn pick<A, B>(a: A, b: B) -> A` comes back as
/// `[Generic(0), Generic(1)] -> Generic(0)` rather than as any particular instantiation. Vale's own
/// solver does the substituting afterwards, so there is never a per-instantiation query back to
/// rustc — which is precisely what the previous design could not express, since a resolved
/// prototype has to pick one instantiation and a generic function has none.
#[derive(Copy, Clone, Debug)]
pub enum ValeSigType<'s, 't> {
    /// A type that needs no substitution — a primitive, or an imported citizen.
    Kind(KindT<'s, 't>),
    /// The function's own generic parameter at this index.
    ///
    /// The index is into **this item's own** parameters, with any parent (impl) parameters already
    /// subtracted, so a caller can use it directly against the declaration it is building. Getting
    /// that subtraction wrong yields a well-formed reference to the wrong slot, which is invisible
    /// at a glance — hence `pick<A, B>` at `<int, bool>` as the canary: a swapped index produces a
    /// plausible wrong concrete type rather than an obvious placeholder.
    Generic(u32),
}

/// A Rust function signature, lowered to Vale terms.
///
/// Note this is expressed over `KindT`, not `CoordT`: the onion refactor dissolved
/// `CoordT` into the reference wraps inside `KindT` itself, so a Rust `&self` /
/// `&mut self` receiver arrives here already wrapped as a `BorrowRef`.
#[derive(Copy, Clone, Debug)]
pub struct ValeSig<'s, 't> {
    /// The item's own generic parameter names, in declaration order. Empty for a non-generic
    /// function, which is the degenerate case rather than a separate one. `ValeSigType::Generic`
    /// indexes into this.
    pub generic_params: &'t [StrI<'s>],
    pub params: &'t [ValeSigType<'s, 't>],
    pub ret: ValeSigType<'s, 't>,
}

/// The questions the typing pass asks about Rust items.
///
/// **Every query here is asked once per *item*, never once per call site.** That is the whole
/// shape of the design: the oracle is a binding generator consulted while declarations are being
/// synthesized, not a service the resolver calls while compiling a body.
///
/// Five methods lived here under the previous design and are now gone, having lost their last
/// callers when the per-call-site seam was retired: `resolve_path`, `kind`, `resolve_method`,
/// `resolve_function`, and `field`. They are deleted rather than parked because two of them
/// (`resolve_method`, `resolve_function`) matched Rust items by **human name string** — the
/// @ATAFLBZ hazard (arch §26.13.5), where a short name is not identity and a wrong `DefId`
/// eventually drives a wrong mangled symbol. Keeping a dead-but-callable name matcher is how that
/// comes back. The removal also makes "nothing queries the oracle per call site" unrepresentable
/// rather than merely tested.
pub trait RustOracle<'s, 't> {
    /// The package coordinate a Rust item lives in, for building its Vale id.
    ///
    /// Only needed for free functions; a method nests under its receiver's id instead.
    fn item_package(&self, item: RustItemId) -> Option<&'s PackageCoordinate<'s>>;

    /// The signature of a Rust function, read **structurally** and lowered to Vale terms.
    ///
    /// @EarlyBinder: this deliberately does *not* instantiate. It reads the signature with its
    /// generic parameters intact and hands back `ValeSigType::Generic(i)` where one appears, so a
    /// single reading serves every instantiation and Vale's solver does the substituting. That
    /// inverts the previous contract, which demanded instantiation with a call's concrete args
    /// before lowering — a rule that could only ever be honoured by minting one prototype per call
    /// site, which is exactly what a generic function makes impossible.
    ///
    /// Consequently `instantiate_identity()` is the *correct* accessor here, where it was wrong
    /// before: discarding the binder to inspect `ty::Param`s is the whole point, rather than an
    /// oversight that silently reads placeholders as if they were types.
    fn fn_sig(
        &self,
        item: RustItemId,
        interner: &TypingInterner<'s, 't>,
    ) -> Option<ValeSig<'s, 't>>;

    /// Every Rust type that should be declared as a Vale citizen, with its human name.
    ///
    /// Enumerable rather than queried by name because the importer materializes the whole
    /// set up front — which is only tractable because `@RTMEIZ` requires every Rust item
    /// Vale uses to be explicitly imported, making the surface finite and declared.
    fn importable_types(&self) -> Vec<(String, RustItemId)> {
        Vec::new()
    }

    /// Every importable Rust free function, with its human name.
    ///
    /// The enumerable counterpart to `resolve_function`. Needed because free functions are
    /// materialized into the reserved `rust` package's store up front, rather than answered
    /// one name at a time — the same reason `importable_types` exists.
    fn importable_functions(&self) -> Vec<(String, RustItemId)> {
        Vec::new()
    }

    /// The methods of a Rust type, by name.
    ///
    /// The whole list, not one lookup at a time, because an environment's store is built
    /// eagerly. A Rust method has no Vale AST behind it, so what lands in the store is a
    /// prototype rather than a function definition.
    fn methods(&self, _item: RustItemId) -> Vec<(String, RustItemId)> {
        Vec::new()
    }
}

// There was a `StubOracle` here — an implementation whose every query returned `None`, held
// by the typing pass when nothing was being asked. `Oracles::none()` says the same thing
// without an implementation to carry, so it is gone: absence is now spelled as absence
// rather than as an object that answers nothing.
