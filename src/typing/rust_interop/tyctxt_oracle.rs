// The real oracle: answers from a live `TyCtxt`.
//
// `'tcx` appears on this struct but in **no** `RustOracle` signature — that asymmetry is the
// whole point of the trait. The typing pass holds a `&dyn RustOracle<'s, 't>` and never names
// a rustc type, so the nightly-internals dependency stops here.
//
// `TyCtxt<'tcx>` is `Copy`, but `'tcx` is tied to arenas owned by `run_compiler`'s stack
// frame: this oracle cannot outlive the callback that built it, and must never be stashed in
// a static or in `HinputsT`.

use std::collections::HashSet;

use rustc_hir::def::{DefKind, Res};
use rustc_middle::ty::{Ty, TyCtxt, TyKind};
use rustc_span::def_id::DefId;

use crate::interner::StrI;
use crate::postparsing::ast::ImportS;
use crate::scout_arena::ScoutArena;
use crate::typing::env::environment::{ImportedItemKind, ResolvedName};
use crate::typing::names::names::*;
use crate::typing::rust_interop::oracle::{
  DeclineReason, RustItemId, RustOracle, ValeSig, ValeSigType,
};
use crate::typing::rust_interop::reserved::RUST_MODULE;
use crate::typing::templata::templata::{ITemplataT, KindTemplataT};
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
  tcx
    .generics_of(def_id)
    .own_params
    .iter()
    .map(|p| scout_arena.intern_str(p.name.as_str()))
    .collect()
}

/// The parent-inclusive generic parameter names for an item: the parent's params first (at the low
/// indices), then the item's own — the exact order rustc numbers `ty::Param`s and `GenericArgs::for_item`
/// fills them.
///
/// A method on a generic type references its **impl's** parameters in its signature — `Vec::push`'s
/// `value: T` and `Vec::new`'s return `Vec<T, Global>` both name the impl's `T` — but `generics_of(method)`
/// reports those under `.parent`, leaving the method's *own* params empty. Lowering that signature against
/// only the own params rejects the inherited `T` as an `InheritedParameter`, so a method needs this full
/// list. Which impl matters: `Vec::new` sits in `impl<T> Vec<T>` (parent params `[T]`, `Global` concrete),
/// while `Vec::push` sits in `impl<T, A> Vec<T, A>` (parent params `[T, A]`) — the parent walk picks up
/// each. A free function or a top-level type has no parent, so this reduces to `own_generic_param_names`.
fn parent_inclusive_generic_param_names<'s>(
  tcx: TyCtxt<'_>,
  scout_arena: &ScoutArena<'s>,
  def_id: DefId,
) -> Vec<StrI<'s>> {
  let generics = tcx.generics_of(def_id);
  let mut names = match generics.parent {
    Some(parent) => parent_inclusive_generic_param_names(tcx, scout_arena, parent),
    None => Vec::new(),
  };
  names.extend(generics.own_params.iter().map(|p| scout_arena.intern_str(p.name.as_str())));
  names
}

/// The Vale package coordinate for a Rust item: the reserved `rust` module, then the item's own
/// crate, then the module path it sits under.
///
/// **Asked of rustc rather than reconstructed.** `tcx.def_path` is the *definition* path and is
/// unique by construction, so two crates each exporting a `Widget` land in different packages and
/// can never intern to one Vale id. The alternative — one coordinate handed to the constructor and
/// stamped on every item — is exactly what made them indistinguishable, and it is the shape
/// @ATAFLBZ warns about: identity from a short name rather than from the thing itself.
///
/// The **last** named segment is the item's own name and belongs in `local_name`, not in the
/// coordinate; everything before it is module nesting. Segments carrying no name — an `impl`
/// block, for one — contribute nothing to a source-level path and are skipped.
///
/// One consequence to know: this is the *definition* path, so `std::vec::Vec` yields
/// `rust.["alloc","vec"]`, because `std::vec` is a re-export of `alloc::vec`. That is right for
/// identity and wrong for a diagnostic, which should echo the path the user wrote. rustc keeps a
/// whole lossy BFS query (`visible_parent_map`) purely to invert one into the other, and it is a
/// diagnostics problem rather than a resolution one — see the naming design in
/// `synthesized-declarations-plan.md` §10.0.
fn package_coord_for<'s>(
  tcx: TyCtxt<'_>,
  scout_arena: &ScoutArena<'s>,
  def_id: DefId,
) -> &'s PackageCoordinate<'s> {
  let named: Vec<String> = tcx
    .def_path(def_id)
    .data
    .iter()
    .filter_map(|segment| segment.data.get_opt_name())
    .map(|name| name.to_string())
    .collect();

  let mut packages = vec![scout_arena.intern_str(tcx.crate_name(def_id.krate).as_str())];
  for module_segment in named.iter().take(named.len().saturating_sub(1)) {
    packages.push(scout_arena.intern_str(module_segment));
  }

  scout_arena.intern_package_coordinate(scout_arena.intern_str(RUST_MODULE), &packages)
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub(crate) enum ItemKind {
  Function,
  Type,
  /// A Rust enum — imported as an opaque sealed interface. Like `Type` it owns inherent methods and a
  /// drop; it differs only in lowering to `KindT::Interface` instead of `KindT::Struct`.
  Enum,
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

/// Resolve one **crate-qualified** dotted path (`crate.module….item`) to a single item.
///
/// The first segment names the crate, the last names the item, and any segments between are the
/// module path within that crate. Because the crate is named, resolution is unambiguous: there is no
/// cross-crate scan and no plurality — `crate1.Widget` and `crate2.Widget` are different paths naming
/// different items. Returns `None` when the crate is not loaded, a module segment is missing, or the
/// final item is not a `fn`/`struct`.
///
/// A re-exported item resolves through the re-export chain to its canonical `DefId` (e.g.
/// `std.vec.Vec` reaches `alloc`'s `Vec`), so identity still comes from the `DefId`, never the path.
///
/// **Intermediate segments must be modules** — the `DefKind::Mod` filter stops a struct named `vec`
/// from swallowing the `vec` in `std::vec::Vec`, the same `DefKind` filter the final item needs.
pub(crate) fn resolve_crate_qualified_path<'tcx>(
  tcx: TyCtxt<'tcx>,
  path: &str,
) -> Option<(DefId, ItemKind)> {
  let segments: Vec<&str> = path.split('.').collect();
  // Need at least `crate.item`: the first segment is the crate, the last is the item.
  let (crate_name, rest) = segments.split_first()?;
  let (item_name, module_segments) = rest.split_last()?;

  // The named crate must be a loaded dependency. `tcx.crates(())` is exactly those external crates;
  // `LOCAL_CRATE` (the compiled stub) is deliberately not among them, so it is never importable.
  let cnum = tcx.crates(()).iter().copied().find(|c| tcx.crate_name(*c).as_str() == *crate_name)?;

  let mut module = cnum.as_def_id();
  for segment in module_segments {
    module = tcx
      .module_children(module)
      .iter()
      // ataflbz-allow: selection — matching a written module segment against a module's children.
      .find(|c| c.ident.to_string() == **segment) // ataflbz-allow: selection
      .and_then(|c| match c.res {
        Res::Def(DefKind::Mod, def_id) => Some(def_id),
        _ => None,
      })?;
  }

  for child in tcx.module_children(module) {
    // Selection — the final segment deciding which child is admitted. Identity comes from the
    // `DefId` captured below, not from this name match.
    let is_other_item = child.ident.to_string() != *item_name; // ataflbz-allow: selection
    if is_other_item {
      continue;
    }
    // Filter on DefKind: a module's children include re-exported modules, `extern crate` entries,
    // etc., so a name match alone could hand back a module where a fn/struct was asked for.
    let kind = match child.res {
      Res::Def(DefKind::Fn, _) => ItemKind::Function,
      Res::Def(DefKind::Struct, _) => ItemKind::Type,
      Res::Def(DefKind::Enum, _) => ItemKind::Enum,
      _ => continue,
    };
    let Res::Def(_, def_id) = child.res else { continue };
    return Some((def_id, kind));
  }
  None
}

impl<'tcx, 's> TyCtxtOracle<'tcx, 's> {
  /// Resolve `allowed` — the declared-importable paths — against the loaded crate graph.
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
  /// - **Short names are not identity.** The allowlist is matched by string equality, and
  ///   `tcx.crates(())` hands us every loaded crate. Rust has no uniqueness rule for short
  ///   names — `new`, `len`, `Error`, `Box` recur across crates — so a name can match in more
  ///   than one place. **The item's identity no longer comes from that match**: each carries
  ///   its own `DefId` and a `package_coord` derived from `tcx.def_path`, so two crates'
  ///   `Widget`s stay two types (`two_crates_exporting_the_same_short_name_stay_distinct`).
  ///   What remains name-shaped is *selection* — which items the allowlist admits — and that
  ///   is the allowlist's own semantics rather than an identity claim.
  ///
  /// The end state enumerates nothing: an `import rust.std.vec.Vec` resolves that one path
  /// segment by segment to exactly one item, keyed by `DefId` thereafter. Cost becomes
  /// O(imports) rather than O(crate graph), and ambiguity stops existing because the user
  /// wrote the full path.
  pub fn new(tcx: TyCtxt<'tcx>, scout_arena: &ScoutArena<'s>, allowed: &[&str]) -> Self {
    let mut items: Vec<RustItem<'s>> = Vec::new();

    for path in allowed {
      // The short name is the final segment; the crate + module segments before it disambiguate.
      let short_name = path.rsplit('.').next().unwrap_or(path);
      if let Some((def_id, kind)) = resolve_crate_qualified_path(tcx, path) {
        items.push(RustItem {
          human_name: scout_arena.intern_str(short_name),
          name: short_name.to_string(),
          def_id,
          package: package_coord_for(tcx, scout_arena, def_id),
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
      .filter(|(_, i)| matches!(i.kind, ItemKind::Type | ItemKind::Enum))
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
            // Parent-inclusive: a method's signature names the impl's params (`self Vec<T, A>`,
            // `value: T`), which live under the method's `.parent`, not its own params.
            generic_params: parent_inclusive_generic_param_names(tcx, scout_arena, assoc.def_id),
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
  /// `args` are the ADT's own generic arguments, which ride the interned name.
  ///
  /// Carrying them here is necessary but not sufficient: a synthesized declaration does not
  /// embed this kind, it names the type through rules. So `declarations.rs` reads these back off
  /// the name to decide whether one `LookupSR` will do or whether it needs `LookupSR` + `CallSR`.
  /// Filling this in without that second half changes nothing observable — measured 2026-07-26.
  fn type_kind<'t>(
    &self,
    idx: usize,
    args: rustc_middle::ty::GenericArgsRef<'tcx>,
    interner: &TypingInterner<'s, 't>,
  ) -> Result<KindT<'s, 't>, DeclineReason>
  where
    's: 't,
  {
    let item = &self.items[idx];
    // Only type arguments. Lifetimes erase at the boundary (@ELASZ) and Vale's arg list has no
    // slot for them; a const generic would need one Vale does not have, and reaches
    // `lower_ty`'s catch-all rather than being silently skipped.
    let template_args: Vec<ITemplataT<'s, 't>> = args
      .types()
      .map(|arg| {
        Ok(ITemplataT::Kind(interner.alloc(KindTemplataT { kind: self.lower_ty(arg, interner)? })))
      })
      .collect::<Result<Vec<_>, DeclineReason>>()?;
    let template_args = interner.alloc_slice_from_vec(template_args);
    // A Rust enum imports as an opaque sealed interface (`KindT::Interface`); everything else is a
    // struct kind. Only the interned name and kind differ — the args ride identically.
    if item.kind == ItemKind::Enum {
      let template = interner
        .intern_interface_template_name(InterfaceTemplateNameT { human_namee: item.human_name });
      let interface_name =
        interner.intern_interface_name(InterfaceNameValT { template, template_args });
      let id = interner.intern_id(IdValT {
        package_coord: item.package,
        init_steps: &[],
        local_name: INameT::Interface(interface_name),
      });
      Ok(KindT::Interface(interner.intern_interface_tt(InterfaceTTValT { id: *id })))
    } else {
      let template_name =
        interner.intern_struct_template_name(StructTemplateNameT { human_name: item.human_name });
      let struct_name = interner.intern_struct_name(StructNameValT {
        template: IStructTemplateNameT::StructTemplate(template_name),
        template_args,
      });
      let id = interner.intern_id(IdValT {
        package_coord: item.package,
        init_steps: &[],
        local_name: INameT::Struct(struct_name),
      });
      Ok(KindT::Struct(interner.intern_struct_tt(StructTTValT { id: *id })))
    }
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
  /// `Err` for anything not representable, which drops the whole declaration rather than
  /// importing it with a hole — and carries *why*, so the eventual lookup failure can say
  /// something better than "couldn't find it".
  fn lower_sig_ty<'t>(
    &self,
    ty: Ty<'tcx>,
    own_param_names: &[StrI<'s>],
    def_id: DefId,
    interner: &TypingInterner<'s, 't>,
  ) -> Result<ValeSigType<'s, 't>, DeclineReason>
  where
    's: 't,
  {
    match ty.kind() {
      TyKind::Param(param) => {
        let name = param.name.as_str();
        match own_param_names.iter().position(|p| p.0 == name) {
          Some(index) => Ok(ValeSigType::Generic(index as u32)),
          // Not among the item's own parameters, so it was inherited from a parent impl.
          // Vale's declaration has no slot for it until the container is declared too.
          None => Err(DeclineReason::InheritedParameter),
        }
      }
      // A projection — `<I as Iterator>::Item` and friends. Not merely unbounded: resolving
      // it *requires* the `I: Iterator` predicate to find the impl, and we deliberately read
      // no predicates at all. So the type isn't unreadable-for-now, it's un-normalizable, and
      // importing it would put an alias in the declaration that nothing can resolve.
      TyKind::Alias(..) => Err(DeclineReason::UnnormalizableAlias),
      // An imported citizen, kept **unapplied** with its arguments as signature positions of
      // their own. Lowering it to a settled `KindT` here is what used to lose a generic
      // argument (`Holder<i32>` and `Holder<bool>` interning alike) and then, once the args
      // were read, what panicked on `Holder<T>` — a `ty::Param` has no `KindT`. Recursing
      // through `lower_sig_ty` handles both, because a parameter is a legal *position* even
      // though it is not a legal type.
      TyKind::Adt(adt_def, adt_args) => {
        let did = adt_def.did();
        let idx = self
          .items
          .iter()
          .position(|i| matches!(i.kind, ItemKind::Type | ItemKind::Enum) && i.def_id == did)
          .ok_or(DeclineReason::UnimportedType)?;
        let args: Vec<ValeSigType<'s, 't>> = adt_args
          .types()
          .map(|arg| self.lower_sig_ty(arg, own_param_names, def_id, interner))
          .collect::<Result<Vec<_>, _>>()?;
        Ok(ValeSigType::Citizen {
          name: self.items[idx].human_name,
          package: self.items[idx].package,
          args: interner.alloc_slice_from_vec(args),
        })
      }
      // A reference — a `&self` receiver or a borrowed parameter. Kept structural (a borrow *of
      // a position*) so the inner citizen keeps its package path and an inner generic keeps its
      // slot, exactly as the non-reference cases above do. The settled `KindT::BorrowRef` that
      // `lower_ty` would build in the fallthrough carries neither, which is why a `&Counter`
      // receiver could not be synthesized before this arm.
      TyKind::Ref(_, inner, _) => Ok(ValeSigType::Borrow(interner.alloc(self.lower_sig_ty(
        *inner,
        own_param_names,
        def_id,
        interner,
      )?))),
      _ => Ok(ValeSigType::Kind(self.lower_ty(ty, interner)?)),
    }
  }

  /// `Err` rather than a panic, since 2026-07-27.
  ///
  /// These were panics on the reasoning that returning nothing produced a *lie* — "couldn't find
  /// function `foo`" for a function that plainly exists — and a crash beat a lie. Both halves of
  /// that were right; the conclusion was not, because these fire during **enumeration** rather
  /// than at a use site, so one `u64` anywhere in a crate's export surface made the whole crate
  /// unimportable. Carrying the reason out is what dissolves the choice: the declaration is
  /// dropped like any other un-representable one, and the reason is available to whatever
  /// eventually fails to find it.
  fn lower_ty<'t>(
    &self,
    ty: Ty<'tcx>,
    interner: &TypingInterner<'s, 't>,
  ) -> Result<KindT<'s, 't>, DeclineReason>
  where
    's: 't,
  {
    match ty.kind() {
      TyKind::Bool => Ok(KindT::Bool(BoolT)),
      TyKind::Tuple(tys) if tys.is_empty() => Ok(KindT::Void(VoidT)),
      TyKind::Int(int_ty) => match int_ty {
        rustc_middle::ty::IntTy::I32 => Ok(KindT::Int(IntT::I32)),
        rustc_middle::ty::IntTy::I64 => Ok(KindT::Int(IntT::I64)),
        _ => Err(DeclineReason::IntWidth),
      },
      // `usize` imports as the Vale `usize` primitive (a distinct kind, never unified with
      // `int`/`i64`). The other unsigned widths (`u8`..`u64`) still decline for now.
      TyKind::Uint(rustc_middle::ty::UintTy::Usize) => Ok(KindT::USize(USizeT)),
      TyKind::Uint(_) => Err(DeclineReason::UnsignedInteger),
      TyKind::Float(_) => Err(DeclineReason::Float),
      TyKind::Str | TyKind::Slice(_) | TyKind::Dynamic(..) => Err(DeclineReason::Unsized),
      TyKind::Adt(adt_def, adt_args) => {
        let did = adt_def.did();
        match self
          .items
          .iter()
          .position(|i| matches!(i.kind, ItemKind::Type | ItemKind::Enum) && i.def_id == did)
        {
          Some(idx) => self.type_kind(idx, adt_args, interner),
          None => Err(DeclineReason::UnimportedType),
        }
      }
      TyKind::Ref(_, inner, _) => {
        // A Rust `&self` receiver arrives as a borrow. `ValeSig` is over `KindT`, and the onion
        // refactor dissolved `CoordT` into the reference wraps inside it, so the borrow is expressed
        // by wrapping rather than by an ownership field. `BorrowRefT` carries no region (BCHATZ): Rust
        // lifetimes erase to `re_erased` at the boundary (@ELASZ), so nothing is lost yet — it becomes
        // lossy when group borrowing and real lifetime reconciliation land.
        Ok(KindT::BorrowRef(interner.alloc(BorrowRefT { inner: self.lower_ty(*inner, interner)? })))
      }
      _ => Err(DeclineReason::Unrepresentable),
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

  fn resolve(&self, name: &ResolvedName<'s>) -> Option<RustItemId> {
    // A name resolves to the one table item whose coordinate, short name, and kind all match.
    // Identity still comes from the `DefId` behind the item; this match is *selection* — which
    // already-resolved item a canonical name picks out — exactly the role the allowlist scan
    // played, now keyed by the full coordinate rather than a bare short name.
    let want_kind = match name.kind {
      ImportedItemKind::Type => ItemKind::Type,
      ImportedItemKind::Function => ItemKind::Function,
      ImportedItemKind::Enum => ItemKind::Enum,
    };
    self
      .items
      .iter()
      .position(|item| {
        // Selection, not identity — picking which already-resolved item a *full* canonical
        // coordinate + short name admits. The item's identity is still its `DefId`, and the
        // coordinate (`tcx.def_path`-derived) makes the pair unique, so this is not a bare
        // short-name match.
        item.kind == want_kind
                && item.human_name == name.importee_name // ataflbz-allow: selection
                && item.package.module == name.module_name
                && item.package.packages.as_slice() == name.package_names
      })
      .map(|idx| RustItemId(idx as u32))
  }

  fn resolve_import(&self, import: &ImportS<'s>) -> Option<ResolvedName<'s>> {
    // Join the import's crate + module + importee segments into the crate-qualified path the
    // resolver walks. The reserved `rust` module is already matched by the caller and is not part
    // of the rustc path.
    let mut path = String::new();
    for seg in import.package_names {
      if !path.is_empty() {
        path.push('.');
      }
      path.push_str(seg.0);
    }
    if !path.is_empty() {
      path.push('.');
    }
    path.push_str(import.importee_name.0);

    // One crate-qualified path resolves to at most one item; find it in the already-resolved table
    // and hand back its canonical name. A method's `DefId` never comes back from the resolver (it
    // filters to fn/struct), so only top-level `Type` and `Function` items match.
    let (def_id, _kind) = resolve_crate_qualified_path(self.tcx, &path)?;
    let item = self.items.iter().find(|item| item.def_id == def_id)?;
    let kind = match item.kind {
      ItemKind::Type => ImportedItemKind::Type,
      ItemKind::Function => ImportedItemKind::Function,
      ItemKind::Enum => ImportedItemKind::Enum,
      ItemKind::Method(_) => return None,
    };
    Some(ResolvedName {
      module_name: item.package.module,
      package_names: item.package.packages.as_slice(),
      importee_name: item.human_name,
      kind,
    })
  }

  fn fn_sig(&self, item: RustItemId, interner: &TypingInterner<'s, 't>) -> Option<ValeSig<'s, 't>> {
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

    // VCOORD: the `DeclineReason` is dropped here, and this is exactly where the side table
    // attaches. Enumeration is the only place that knows *which item* declined and *why* at the
    // same moment, so a table populated here is what lets the eventual lookup failure say
    // "found `first`, but its return type names an associated type" instead of "couldn't find
    // `first`". Until it exists the reason is computed and thrown away — deliberately, because
    // unifying the exits is worth landing on its own and the table's consumer may sit in core.
    let params: Vec<ValeSigType<'s, 't>> = sig
      .inputs()
      .iter()
      .map(|ty| self.lower_sig_ty(*ty, generic_params, def_id, interner))
      .collect::<Result<Vec<_>, _>>()
      .ok()?;
    let ret = self.lower_sig_ty(sig.output(), generic_params, def_id, interner).ok()?;

    Some(ValeSig {
      generic_params: interner.alloc_slice_copy(generic_params),
      params: interner.alloc_slice_from_vec(params),
      ret,
    })
  }

  fn type_generic_params(
    &self,
    item: RustItemId,
    interner: &TypingInterner<'s, 't>,
  ) -> &'t [StrI<'s>] {
    match self.items.get(item.0 as usize) {
      Some(rust_item) => interner.alloc_slice_copy(&rust_item.generic_params),
      None => &[],
    }
  }

  fn methods(&self, item: RustItemId) -> Vec<(String, RustItemId)> {
    let owner = item.0 as usize;
    self
      .items
      .iter()
      .enumerate()
      .filter(|(_, i)| i.kind == ItemKind::Method(owner))
      .map(|(idx, i)| (i.name.clone(), RustItemId(idx as u32)))
      .collect()
  }
}
