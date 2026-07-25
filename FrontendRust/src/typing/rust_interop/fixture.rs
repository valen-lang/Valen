// A canned RustOracle for tests.
//
// Lets the seam be exercised end to end without rustc: a test declares the Rust
// functions it expects to exist, and this answers as though rustc had confirmed them.
// Nothing here reads a real crate — that is the TyCtxt-backed oracle's job.

use crate::interner::StrI;
use crate::scout_arena::ScoutArena;
use crate::typing::names::names::IdT;
use crate::typing::rust_interop::oracle::{
    RustFieldInfo, RustItemId, RustKind, RustOracle, ValeSig,
};
use crate::typing::types::types::*;
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::PackageCoordinate;

/// One Rust free function the fixture pretends exists.
pub struct FixtureFunction<'s, 't> {
    pub name: &'s str,
    pub params: Vec<KindT<'s, 't>>,
    pub ret: KindT<'s, 't>,
}

/// A `RustOracle` backed by a fixed table instead of rustc.
///
/// Only the queries a test needs are answered; everything else returns `None`, which
/// every seam treats as "not a Rust question, carry on as before".
pub struct FixtureOracle<'s, 't> {
    package_coord: &'s PackageCoordinate<'s>,
    functions: Vec<FixtureFunction<'s, 't>>,
}

impl<'s, 't> FixtureOracle<'s, 't> {
    /// `module` and `packages` name the Rust package the functions live in, e.g.
    /// ("rust", ["mycrate"]). The coord is interned up front because `item_package`
    /// hands back a `&'s` reference to it.
    pub fn new(
        scout_arena: &ScoutArena<'s>,
        module: StrI<'s>,
        packages: &[StrI<'s>],
        functions: Vec<FixtureFunction<'s, 't>>,
    ) -> Self {
        FixtureOracle {
            package_coord: scout_arena.intern_package_coordinate(module, packages),
            functions,
        }
    }

    fn function(&self, item: RustItemId) -> Option<&FixtureFunction<'s, 't>> {
        self.functions.get(item.0 as usize)
    }
}

impl<'s, 't> RustOracle<'s, 't> for FixtureOracle<'s, 't> {
    fn resolve_path(&self, _id: &IdT<'s, 't>) -> Option<RustItemId> {
        None
    }

    fn kind(&self, _item: RustItemId) -> Option<RustKind> {
        None
    }

    fn resolve_method(&self, _receiver: &IdT<'s, 't>, _method_name: &str) -> Option<RustItemId> {
        None
    }

    fn resolve_function(&self, function_name: &str) -> Option<RustItemId> {
        self.functions
            .iter()
            .position(|f| f.name == function_name)
            .map(|index| RustItemId(index as u32))
    }

    fn item_package(&self, item: RustItemId) -> Option<&'s PackageCoordinate<'s>> {
        self.function(item).map(|_| self.package_coord)
    }

    fn fn_sig(
        &self,
        item: RustItemId,
        _args: &[KindT<'s, 't>],
        interner: &TypingInterner<'s, 't>,
    ) -> Option<ValeSig<'s, 't>> {
        let function = self.function(item)?;
        // The params are allocated into the typing arena here rather than stored as a
        // slice, so the fixture itself needs no arena at construction. A real oracle
        // instantiates at `_args` before lowering (@EarlyBinder); the fixture is
        // non-generic, so there is nothing to instantiate.
        Some(ValeSig {
            params: interner.alloc_slice_copy(&function.params),
            ret: function.ret,
        })
    }

    fn field(&self, _owner: &IdT<'s, 't>, _field_name: &str) -> Option<RustFieldInfo<'s, 't>> {
        None
    }
}
