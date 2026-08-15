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
use crate::postparsing::ast::ImportS;
use crate::typing::env::environment::ResolvedName;
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
    /// A type that needs no substitution — a primitive.
    Kind(KindT<'s, 't>),
    /// An imported citizen applied to arguments, each of which is itself a signature position.
    ///
    /// **Why this cannot be a `Kind`.** A `KindT` is a settled type, and `Holder<T>` is not settled
    /// — its argument is the enclosing item's generic parameter. Lowering it eagerly means lowering
    /// `T`, which has no `KindT` at all, so a `Holder<T>` parameter either panics in the oracle or
    /// silently loses its argument. Both happened before this variant existed.
    ///
    /// Recursive, so `Holder<Holder<int>>` and `Holder<T>` are the same case at different depths.
    /// `args` is empty for a non-generic citizen, which is the degenerate case rather than a
    /// separate one — a citizen's name resolves to a *template* either way, so it always needs the
    /// application step.
    ///
    /// The name and its **package coordinate**, which together are what a declaration writes into
    /// its `LookupSR` as a path. The coordinate is not decoration: two crates can export the same
    /// short name, and a bare name would find both and panic. Identity still comes from the
    /// coordinate the importer registers the citizen under — this carries it to the declaration so
    /// both ends agree by construction.
    Citizen {
        name: StrI<'s>,
        package: &'s PackageCoordinate<'s>,
        args: &'t [ValeSigType<'s, 't>],
    },
    /// The function's own generic parameter at this index.
    ///
    /// The index is into **this item's own** parameters, with any parent (impl) parameters already
    /// subtracted, so a caller can use it directly against the declaration it is building. Getting
    /// that subtraction wrong yields a well-formed reference to the wrong slot, which is invisible
    /// at a glance — hence `pick<A, B>` at `<int, bool>` as the canary: a swapped index produces a
    /// plausible wrong concrete type rather than an obvious placeholder.
    Generic(u32),
}

/// Why a Rust signature has no Vale form.
///
/// **Structure only — no rendering here.** A case asserts the variant; the wording a person reads is
/// built where diagnostics are built, which is also where the arenas to hold it live (§26b.4). The
/// reason travels because it *is* the point of declining: a bare `None` makes the eventual failure
/// read *"couldn't find function `foo`"* for a function that plainly exists, and avoiding that lie
/// is why "for now, panic" was chosen over declining back on 2026-07-25. Declining **with** the
/// reason is what makes the panic unnecessary rather than merely relocated.
///
/// Every variant below was a `panic!` in `lower_ty` until 2026-07-27.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DeclineReason {
    /// An integer width `IntT` cannot hold — it carries only `bits`, and only 32 and 64 are mapped.
    IntWidth,
    /// `IntT` has no signedness, so an unsigned type would silently become its signed counterpart.
    UnsignedInteger,
    /// `FloatT` is a unit struct with no width field, so `f32` and `f64` would intern identically.
    Float,
    /// Vale has no unsized concept, so `str` / `[T]` / `dyn Trait` cannot be value types.
    Unsized,
    /// A type reached only through this signature and never imported (@RTMEIZ).
    UnimportedType,
    /// A projection such as `<I as Iterator>::Item`. Normalizing it *requires* reading the
    /// `I: Iterator` predicate to find the impl, and no predicates are read at all — so it is
    /// un-normalizable rather than merely unread.
    UnnormalizableAlias,
    /// A `ty::Param` inherited from a parent impl. Vale's declaration has no slot for it until the
    /// container is declared too.
    InheritedParameter,
    /// A rustc type kind with no Vale representation yet — the catch-all.
    Unrepresentable,
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

    /// The rustc item a resolved canonical name refers to, or `None` if this oracle has no such item.
    ///
    /// This is the inverse of the name a top-level denizen's `IdT` carries: given the `ResolvedName`
    /// rebuilt from an id (its `package_coord` + `local_name`), hand back the `RustItemId` so lazy
    /// synthesis can query the signature. It replaces the offset-encoding trick, where the item index
    /// was decoded from a synthesized code-location offset. Only top-level types and free functions
    /// resolve here; a method is found via its owner's `ResolvedName` plus `methods`.
    fn resolve(&self, _name: &ResolvedName<'s>) -> Option<RustItemId> {
        None
    }

    /// The canonical name a crate-qualified `import rust.crate.X.Y` statement resolves to, following
    /// re-exports. `None` when it resolves to nothing — the crate isn't loaded, a module segment is
    /// missing, or the target is a module rather than a fn/struct — which the caller treats as an
    /// unresolvable-import error. Singular: the crate is named, so there is no cross-crate ambiguity.
    fn resolve_import(&self, _import: &ImportS<'s>) -> Option<ResolvedName<'s>> {
        None
    }

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

    /// A Rust type's own generic parameter names, in declaration order.
    ///
    /// Empty for a non-generic type — the degenerate case, not a separate one. This is what makes a
    /// synthesized `StructS` a *template* rather than a finished type, and therefore what gives a
    /// `CallSR` something to apply arguments to. Without it, two instantiations of one generic Rust
    /// type intern to the same argument-less Vale kind.
    fn type_generic_params(
        &self,
        _item: RustItemId,
        _interner: &TypingInterner<'s, 't>,
    ) -> &'t [StrI<'s>] {
        &[]
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
