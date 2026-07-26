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

use crate::typing::names::names::IdT;
use crate::typing::types::types::*;
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::PackageCoordinate;

/// Which Vale kind a Rust item maps onto. A Rust `struct` is a struct-kind; a Rust
/// `enum` is a closed-interface-kind (a closed sum type *is* a Vale closed trait);
/// a Rust `trait` is an open interface. `union` is deferred.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum RustKind {
    Struct,
    Enum,
    Trait,
    Union,
}

/// An opaque handle to a resolved Rust item. Valid only within one invocation —
/// never serialized, never stored in `HinputsT`. The durable identity of a Rust item
/// is its path, carried by the `rust`-packaged `IdT`.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RustItemId(pub u32);

/// A Rust function signature, lowered to Vale kinds.
///
/// Note this is expressed over `KindT`, not `CoordT`: the onion refactor dissolved
/// `CoordT` into the reference wraps inside `KindT` itself, so a Rust `&self` /
/// `&mut self` receiver arrives here already wrapped as a `BorrowRef`.
#[derive(Copy, Clone, Debug)]
pub struct ValeSig<'s, 't> {
    pub params: &'t [KindT<'s, 't>],
    pub ret: KindT<'s, 't>,
}

/// A `pub` field of a Rust struct, lowered to Vale terms.
///
/// Only `pub` fields are answerable. Vale is an external consumer of a Rust type, so
/// its private internals are opaque — but its public fields are as readable as they
/// are to any downstream Rust crate.
#[derive(Copy, Clone, Debug)]
pub struct RustFieldInfo<'s, 't> {
    pub tyype: KindT<'s, 't>,
    pub index: usize,
}

pub trait RustOracle<'s, 't> {
    /// Resolve a `rust`-packaged path (e.g. `rust.std.vec.Vec`) to an item handle.
    fn resolve_path(&self, id: &IdT<'s, 't>) -> Option<RustItemId>;

    /// Which Vale kind should this Rust item be interned as?
    fn kind(&self, item: RustItemId) -> Option<RustKind>;

    /// Find a method by name on a Rust-backed receiver type.
    fn resolve_method(&self, receiver: &IdT<'s, 't>, method_name: &str) -> Option<RustItemId>;

    /// Find a free function by name among the Rust items currently in scope.
    ///
    /// Unlike `resolve_method` there is no receiver to key on, so "in scope" is the
    /// oracle's to define — it is the side that knows which Rust paths were imported.
    /// Answering `None` for an unknown name is the common case and must stay cheap:
    /// every Vale call whose args have no Rust-backed receiver reaches this.
    fn resolve_function(&self, function_name: &str) -> Option<RustItemId>;

    /// The package coordinate a Rust item lives in, for building its Vale id.
    ///
    /// Only needed for free functions; a method nests under its receiver's id instead.
    fn item_package(&self, item: RustItemId) -> Option<&'s PackageCoordinate<'s>>;

    /// The signature of a Rust function, instantiated at `args` and lowered to Vale kinds.
    ///
    /// @EarlyBinder: the implementation must instantiate with the call's concrete
    /// args BEFORE lowering. Lowering out of the `EarlyBinder` first and reusing the
    /// result across monomorphizations silently substitutes wrong types.
    fn fn_sig(
        &self,
        item: RustItemId,
        args: &[KindT<'s, 't>],
        interner: &TypingInterner<'s, 't>,
    ) -> Option<ValeSig<'s, 't>>;

    /// A `pub` field of a Rust-backed struct. `None` for a private or absent field.
    fn field(&self, owner: &IdT<'s, 't>, field_name: &str) -> Option<RustFieldInfo<'s, 't>>;

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
