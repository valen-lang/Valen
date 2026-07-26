// The real oracle: answers from a live `TyCtxt`.
//
// `'tcx` appears on this struct but in **no** `RustOracle` signature — that asymmetry is the
// whole point of the trait. The typing pass holds a `&dyn RustOracle<'s, 't>` and never names
// a rustc type, so the nightly-internals dependency stops here.
//
// `TyCtxt<'tcx>` is `Copy`, but `'tcx` is tied to arenas owned by `run_compiler`'s stack
// frame: this oracle cannot outlive the callback that built it, and must never be stashed in
// a static or in `HinputsT`.

use rustc_hir::def::{DefKind, Res};
use rustc_middle::ty::{Ty, TyCtxt, TyKind};
use rustc_span::def_id::DefId;

use crate::interner::StrI;
use crate::scout_arena::ScoutArena;
use crate::typing::names::names::*;
use crate::typing::rust_interop::oracle::{
    RustItemId, RustOracle, ValeSig, ValeSigType,
};
use crate::typing::types::types::*;
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::PackageCoordinate;

/// An item's own generic parameter names, in declaration order.
///
/// `own_params` rather than the full list: a method on `impl<T> Foo<T>` sees the impl's
/// parameters at the low indices of its parent-inclusive list, and Vale's declaration names only
/// what the item itself declares. Resolving by name later means that offset never has to be
/// computed — and names are safe to key on because Rust forbids an item from shadowing a generic
/// parameter name declared by its parent impl (E0403).
fn own_generic_param_names<'s>(
    tcx: TyCtxt<'_>,
    scout_arena: &ScoutArena<'s>,
    def_id: DefId,
) -> Vec<StrI<'s>> {
    tcx.generics_of(def_id)
        .own_params
        .iter()
        .map(|p| scout_arena.intern_str(p.name.as_str()))
        .collect()
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ItemKind {
    Function,
    Type,
    /// A method, with the index of the type it hangs off.
    Method(usize),
}

/// One resolved Rust item. `RustItemId` indexes a flat table of these — functions, types and
/// methods share one id space so the trait needs only one handle type.
struct RustItem<'s> {
    name: String,
    human_name: StrI<'s>,
    def_id: DefId,
    package: &'s PackageCoordinate<'s>,
    kind: ItemKind,
    /// The item's **own** generic parameter names, in declaration order.
    ///
    /// Interned here rather than on demand because the oracle cannot hold the scout arena: a
    /// `&'s ScoutArena<'s>` field would force the arena to outlive `'s`, which is `'s` itself.
    /// Construction is the one place the arena is in hand, and these names never change, so
    /// computing them once is both simpler and the only shape that borrows.
    ///
    /// Names rather than a count because `lower_sig_ty` resolves a `ty::Param` against this list
    /// by name, sidestepping the parent-inclusive index arithmetic entirely.
    generic_params: Vec<StrI<'s>>,
}

pub struct TyCtxtOracle<'tcx, 's> {
    tcx: TyCtxt<'tcx>,
    /// Every importable item, resolved once at construction.
    ///
    /// Precomputed rather than resolved on demand for two reasons: the trait's methods take
    /// `&self` so they cannot memoize without interior mutability, and `resolve_function`
    /// sits on a hot path — every Vale call whose arguments have no Rust-backed receiver
    /// reaches it, so its negative answer has to be a scan of a short list rather than a
    /// rustc query.
    items: Vec<RustItem<'s>>,
}

impl<'tcx, 's> TyCtxtOracle<'tcx, 's> {
    /// Resolve `allowed` — the declared-importable names — against the loaded crate graph.
    ///
    /// Scoping is membership in this list, not a check at the call site. The list is what an
    /// `import rust.X.Y` will eventually populate; supplying it explicitly is the same
    /// mechanism with a different source.
    ///
    /// `crate_name` and `def_path` are the safe accessors — `def_path_str` ICEs outside
    /// diagnostic contexts, and its panic blames rustc internals rather than the call site,
    /// which is what makes it expensive to diagnose (@DPSFDOZ).
    ///
    /// VCOORD: retire the up-front walk and every linear name scan in this file. Both are
    /// scaffolding for a flat single-crate fixture and neither generalizes:
    ///
    /// - **The walk is insufficient, not just slow.** `module_children` on a crate root yields
    ///   only that root's *direct* children, so a nested item — `std::vec::Vec` lives under
    ///   `std::vec`, not under the `std` root — is never seen. Today's fixture works solely
    ///   because its items sit at the crate root. Recursing to fix that is what would make the
    ///   walk expensive, and it would widen name collisions from crate roots to every visible
    ///   item in every loaded crate.
    /// - **Short names are not identity.** `resolve_function`, `resolve_method`, and this walk
    ///   all decide by string equality; `resolve_method` matches a method's *owner* by human
    ///   name. Rust has no uniqueness rule for short names — `new`, `len`, `Error`, `Box` recur
    ///   across crates — and `tcx.crates(())` hands us every loaded crate, so first-match-wins
    ///   silently picks a stranger. Since the matched `DefId` ultimately drives the mangled
    ///   symbol, the failure surfaces as a link error against a plausible-looking symbol, far
    ///   from the mistake.
    ///
    /// The end state enumerates nothing: an `import rust.std.vec.Vec` resolves that one path
    /// segment by segment to exactly one item, keyed by `DefId` thereafter. Cost becomes
    /// O(imports) rather than O(crate graph), and ambiguity stops existing because the user
    /// wrote the full path. Until then, any walk over a rustc-global collection needs a
    /// provenance filter (is this from a crate we control?) rather than a name comparison.
    pub fn new(
        tcx: TyCtxt<'tcx>,
        scout_arena: &ScoutArena<'s>,
        package_coord: &'s PackageCoordinate<'s>,
        allowed: &[&str],
    ) -> Self {
        let mut items: Vec<RustItem<'s>> = Vec::new();

        for &cnum in tcx.crates(()).iter() {
            for child in tcx.module_children(cnum.as_def_id()) {
                let name = child.ident.to_string();
                if !allowed.contains(&name.as_str()) {
                    continue;
                }
                // Filter on DefKind, not just name: a crate's module children include its own
                // `extern crate std`, so an unfiltered name match would hand back a module
                // where a function or type was asked for.
                let kind = match child.res {
                    Res::Def(DefKind::Fn, _) => ItemKind::Function,
                    Res::Def(DefKind::Struct, _) => ItemKind::Type,
                    _ => continue,
                };
                let Res::Def(_, def_id) = child.res else { continue };
                items.push(RustItem {
                    human_name: scout_arena.intern_str(&name),
                    name,
                    def_id,
                    package: package_coord,
                    kind,
                    generic_params: own_generic_param_names(tcx, scout_arena, def_id),
                });
            }
        }

        // Methods come from inherent impls. Trait impls are deliberately not walked yet:
        // "all impls of a trait" is unbounded in Rust because of blanket impls, so that
        // question needs a design rather than a walk (callout map §5.5).
        let type_indices: Vec<usize> = items
            .iter()
            .enumerate()
            .filter(|(_, i)| i.kind == ItemKind::Type)
            .map(|(idx, _)| idx)
            .collect();
        for owner_idx in type_indices {
            let owner_def_id = items[owner_idx].def_id;
            let package = items[owner_idx].package;
            for impl_def_id in tcx.inherent_impls(owner_def_id).iter() {
                for assoc in tcx.associated_items(*impl_def_id).in_definition_order() {
                    if assoc.as_tag() != rustc_middle::ty::AssocTag::Fn {
                        continue;
                    }
                    let name = assoc.name().to_string();
                    items.push(RustItem {
                        human_name: scout_arena.intern_str(&name),
                        name,
                        def_id: assoc.def_id,
                        package,
                        kind: ItemKind::Method(owner_idx),
                        generic_params: own_generic_param_names(tcx, scout_arena, assoc.def_id),
                    });
                }
            }
        }

        TyCtxtOracle { tcx, items }
    }

    /// The Vale kind for an imported Rust type, built from its `rust`-packaged name.
    ///
    /// This is the whole of "a Rust struct is an ordinary Vale struct-kind": five interner
    /// calls, no new name type and no new `KindT` arm. `is_rust_backed` holds for it because
    /// the id carries the reserved `rust` package coordinate.
    fn type_kind<'t>(
        &self,
        idx: usize,
        interner: &TypingInterner<'s, 't>,
    ) -> KindT<'s, 't>
    where
        's: 't,
    {
        let item = &self.items[idx];
        let template_name = interner
            .intern_struct_template_name(StructTemplateNameT { human_name: item.human_name });
        let struct_name = interner.intern_struct_name(StructNameValT {
            template: IStructTemplateNameT::StructTemplate(template_name),
            template_args: &[],
        });
        let id = interner.intern_id(IdValT {
            package_coord: item.package,
            init_steps: &[],
            local_name: INameT::Struct(struct_name),
        });
        KindT::Struct(interner.intern_struct_tt(StructTTValT { id: *id }))
    }

    /// Lowers one rustc type to a Vale kind.
    ///
    /// Panics, deliberately and with the gap named, on anything Vale's IR cannot represent.
    /// The long-term answer is to grow the IR (signedness on `IntT`, a width on `FloatT`, an
    /// unsized concept); until then a panic stating which fact is missing beats a `None` that
    /// would surface as "no such function" for a function that plainly exists.
    /// Lower one position of a signature, keeping a generic parameter *as* a parameter.
    ///
    /// **Keyed on the parameter's name, deliberately, not on its index.** A `ty::Param`'s `index`
    /// is into the item's *parent-inclusive* generic list — for a method on `impl<T> Foo<T>` the
    /// impl's parameters occupy the low indices and the item's own follow — so using it directly
    /// against a declaration that names only the item's own parameters is an off-by-`parent_count`
    /// waiting to happen. Names sidestep the arithmetic entirely, and they are safe to key on
    /// because **Rust forbids an item from reusing a generic parameter name declared by its parent
    /// impl** (E0403), so within an item plus its parents the names are unique.
    ///
    /// (`Generics::param_at(index, tcx)` does the subtraction properly, including for nesting
    /// deeper than one level, and is the right tool if an index ever has to be used. It is
    /// mentioned here so the next reader doesn't hand-roll `index - parent_count`.)
    ///
    /// Getting this wrong would be quiet — a well-formed reference to the wrong slot surfaces at a
    /// call site as a plausible *concrete* type rather than anything resembling a placeholder.
    /// Hence the `pick<A, B>` fixture instantiated at two different types: a swap yields `bool`
    /// where `int` belongs, which a test can see.
    ///
    /// `None` for anything not representable, which drops the whole declaration rather than
    /// importing it with a hole.
    fn lower_sig_ty<'t>(
        &self,
        ty: Ty<'tcx>,
        own_param_names: &[StrI<'s>],
        def_id: DefId,
        interner: &TypingInterner<'s, 't>,
    ) -> Option<ValeSigType<'s, 't>>
    where
        's: 't,
    {
        match ty.kind() {
            TyKind::Param(param) => {
                let name = param.name.as_str();
                match own_param_names.iter().position(|p| p.0 == name) {
                    Some(index) => Some(ValeSigType::Generic(index as u32)),
                    // Not among the item's own parameters, so it was inherited from a parent impl.
                    // Vale's declaration has no slot for it until the container is declared too.
                    None => None,
                }
            }
            // A projection — `<I as Iterator>::Item` and friends. Not merely unbounded: resolving
            // it *requires* the `I: Iterator` predicate to find the impl, and we deliberately read
            // no predicates at all. So the type isn't unreadable-for-now, it's un-normalizable, and
            // importing it would put an alias in the declaration that nothing can resolve.
            TyKind::Alias(..) => None,
            _ => Some(ValeSigType::Kind(self.lower_ty(ty, interner))),
        }
    }

    fn lower_ty<'t>(&self, ty: Ty<'tcx>, interner: &TypingInterner<'s, 't>) -> KindT<'s, 't>
    where
        's: 't,
    {
        match ty.kind() {
            TyKind::Bool => KindT::Bool(BoolT),
            TyKind::Tuple(tys) if tys.is_empty() => KindT::Void(VoidT),
            TyKind::Int(int_ty) => match int_ty {
                rustc_middle::ty::IntTy::I32 => KindT::Int(IntT::I32),
                rustc_middle::ty::IntTy::I64 => KindT::Int(IntT::I64),
                other => panic!(
                    "cannot lower Rust {other:?}: Vale's IntT carries only `bits`, so only \
                     i32 and i64 have a representation today"
                ),
            },
            TyKind::Uint(uint_ty) => panic!(
                "cannot lower Rust {uint_ty:?}: IntT has no signedness, so an unsigned type \
                 would silently become its signed counterpart"
            ),
            TyKind::Float(float_ty) => panic!(
                "cannot lower Rust {float_ty:?}: FloatT is a unit struct with no width field"
            ),
            TyKind::Str | TyKind::Slice(_) | TyKind::Dynamic(..) => panic!(
                "cannot lower Rust {ty:?}: Vale has no unsized concept, so str/[T]/dyn Trait \
                 cannot be value types"
            ),
            TyKind::Adt(adt_def, _) => {
                let did = adt_def.did();
                match self
                    .items
                    .iter()
                    .position(|i| i.kind == ItemKind::Type && i.def_id == did)
                {
                    Some(idx) => self.type_kind(idx, interner),
                    None => panic!(
                        "cannot lower Rust type {:?}: it was not imported. Every Rust item \
                         Vale uses must be explicitly imported (@RTMEIZ), including ones \
                         reached only through another item's signature",
                        self.tcx.def_path(did)
                    ),
                }
            }
            TyKind::Ref(_, inner, _) => {
                // A Rust `&self` receiver arrives as a borrow. `ValeSig` is over `KindT`, and
                // the onion refactor dissolved `CoordT` into the reference wraps inside it,
                // so the borrow is expressed by wrapping rather than by an ownership field.
                // `RegionT::Default` because Vale has no `ITemplataT::Region` variant, so an
                // arg list cannot carry a lifetime and the ~6 solver sites that would need
                // one hardcode this too. Rust lifetimes erase to `re_erased` at the boundary
                // (@ELASZ), so nothing is lost yet — it becomes lossy when group borrowing
                // and real lifetime reconciliation land (callout map §5.3).
                KindT::BorrowRef(interner.alloc(BorrowRefT {
                    inner: self.lower_ty(*inner, interner),
                    region: RegionT::Default,
                }))
            }
            other => panic!("cannot lower Rust type {other:?}: no Vale representation yet"),
        }
    }
}

impl<'tcx, 's, 't> RustOracle<'s, 't> for TyCtxtOracle<'tcx, 's>
where
    's: 't,
{
    fn item_package(&self, item: RustItemId) -> Option<&'s PackageCoordinate<'s>> {
        Some(self.items.get(item.0 as usize)?.package)
    }

    fn fn_sig(
        &self,
        item: RustItemId,
        interner: &TypingInterner<'s, 't>,
    ) -> Option<ValeSig<'s, 't>> {
        let rust_item = self.items.get(item.0 as usize)?;
        let def_id = rust_item.def_id;

        // @EarlyBinder: deliberately NOT instantiating. `instantiate_identity` discards the
        // binder and leaves `ty::Param`s standing, which is exactly what structural reading
        // wants — one reading serves every instantiation. (This same call was a defect under
        // the previous design, where the result was lowered as though the params were types.)
        //
        // Only the outer `EarlyBinder` is opened. The inner `Binder` holds late-bound
        // *lifetimes* and nothing else — type and const parameters are always early-bound — so
        // there is no type information hiding behind `skip_binder`.
        let binder = self.tcx.fn_sig(def_id).instantiate_identity();
        let sig = binder.skip_binder();

        // Interned at construction; see `RustItem::generic_params` for why the arena cannot be
        // held here.
        let generic_params = &rust_item.generic_params;

        let params: Vec<ValeSigType<'s, 't>> = sig
            .inputs()
            .iter()
            .map(|ty| self.lower_sig_ty(*ty, generic_params, def_id, interner))
            .collect::<Option<Vec<_>>>()?;
        let ret = self.lower_sig_ty(sig.output(), generic_params, def_id, interner)?;

        Some(ValeSig {
            generic_params: interner.alloc_slice_copy(generic_params),
            params: interner.alloc_slice_from_vec(params),
            ret,
        })
    }

    fn importable_types(&self) -> Vec<(String, RustItemId)> {
        self.items
            .iter()
            .enumerate()
            .filter(|(_, i)| i.kind == ItemKind::Type)
            .map(|(idx, i)| (i.name.clone(), RustItemId(idx as u32)))
            .collect()
    }

    fn importable_functions(&self) -> Vec<(String, RustItemId)> {
        self.items
            .iter()
            .enumerate()
            .filter(|(_, i)| i.kind == ItemKind::Function)
            .map(|(idx, i)| (i.name.clone(), RustItemId(idx as u32)))
            .collect()
    }

    fn methods(&self, item: RustItemId) -> Vec<(String, RustItemId)> {
        let owner = item.0 as usize;
        self.items
            .iter()
            .enumerate()
            .filter(|(_, i)| i.kind == ItemKind::Method(owner))
            .map(|(idx, i)| (i.name.clone(), RustItemId(idx as u32)))
            .collect()
    }
}
