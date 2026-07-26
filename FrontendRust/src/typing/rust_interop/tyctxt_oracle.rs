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
    RustFieldInfo, RustItemId, RustKind, RustOracle, ValeSig,
};
use crate::typing::types::types::*;
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::PackageCoordinate;

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
    fn resolve_path(&self, _id: &IdT<'s, 't>) -> Option<RustItemId> {
        None
    }

    fn kind(&self, item: RustItemId) -> Option<RustKind> {
        match self.items.get(item.0 as usize)?.kind {
            ItemKind::Type => Some(RustKind::Struct),
            _ => None,
        }
    }

    fn resolve_method(&self, receiver: &IdT<'s, 't>, method_name: &str) -> Option<RustItemId> {
        // The receiver's human name identifies which imported type this is; a Rust method
        // hangs off exactly one.
        let INameT::Struct(struct_name) = receiver.local_name else { return None };
        let IStructTemplateNameT::StructTemplate(template) = struct_name.template else {
            return None;
        };
        let owner_name = template.human_name.0;
        self.items
            .iter()
            .position(|i| match i.kind {
                ItemKind::Method(owner) => {
                    i.name == method_name && self.items[owner].name == owner_name
                }
                _ => false,
            })
            .map(|i| RustItemId(i as u32))
    }

    fn resolve_function(&self, function_name: &str) -> Option<RustItemId> {
        self.items
            .iter()
            .position(|i| i.kind == ItemKind::Function && i.name == function_name)
            .map(|i| RustItemId(i as u32))
    }

    fn item_package(&self, item: RustItemId) -> Option<&'s PackageCoordinate<'s>> {
        Some(self.items.get(item.0 as usize)?.package)
    }

    fn fn_sig(
        &self,
        item: RustItemId,
        args: &[KindT<'s, 't>],
        interner: &TypingInterner<'s, 't>,
    ) -> Option<ValeSig<'s, 't>> {
        let def_id = self.items.get(item.0 as usize)?.def_id;

        // `instantiate_identity` is a no-op unwrap: it discards the `EarlyBinder` and hands
        // back the signature with `ty::Param` placeholders still in it. That is correct only
        // for a function with no generics, where there is nothing to substitute — and it is
        // silently wrong for anything else, because lowering would read placeholders and
        // produce a plausible-looking result.
        //
        // Substituting properly needs the call's Vale `args` rebuilt as rustc `GenericArgs`
        // (`generics_of` + `mk_args` + `re_erased`), which is the lossy-args problem the
        // architecture doc records as Option A's sharpest weakness (§8.10, callout map §5.3).
        // Until that is settled, refuse loudly rather than read placeholders.
        let generics = self.tcx.generics_of(def_id);
        if generics.count() > 0 {
            panic!(
                "cannot lower generic Rust function {:?}: it has {} generic parameter(s), and \
                 instantiating at the call's args requires rebuilding rustc GenericArgs from \
                 Vale's arg list — see arch §8.10 / callout map §5.3. Vale args at this call \
                 site were {args:?}",
                self.tcx.def_path(def_id),
                generics.count()
            );
        }
        // @EarlyBinder: with no generic parameters this is the identity, so lowering after it
        // is sound. When generics land, this must instantiate at `args` BEFORE lowering —
        // doing it in the other order silently reuses one lowering across every
        // monomorphization.
        let binder = self.tcx.fn_sig(def_id).instantiate_identity();
        let sig = binder.skip_binder();
        let params: Vec<KindT<'s, 't>> =
            sig.inputs().iter().map(|ty| self.lower_ty(*ty, interner)).collect();
        let ret = self.lower_ty(sig.output(), interner);
        Some(ValeSig { params: interner.alloc_slice_from_vec(params), ret })
    }

    fn field(&self, _owner: &IdT<'s, 't>, _field_name: &str) -> Option<RustFieldInfo<'s, 't>> {
        None
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
