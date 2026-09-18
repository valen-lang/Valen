//! Build the borrow checker's group-annotated `KindGT` for a value, and re-express its groups in
//! another frame — now on the canonical `borrow_checker::{kind_g, group_expr, templata_g}` types.
//!
//! `KindGT` mirrors the typing pass's `KindT` variant-for-variant; the only addition is a `GroupExprG`
//! on each borrow. Groups are read from the written `ITypeST` for the outer borrow layers and array
//! elements. Template arguments are **group-free** in the canonical model (`KindTemplataG` holds a raw
//! `KindT`; groups never flow through the solver), so a citizen's args are a plain structural mirror.
//!
//! Two entry points:
//!  * `make_kind_g(kind, tyype, param_name, arena)` builds a `KindGT` with groups in `tyype`'s frame.
//!  * `substitute_groups(kindg, subst, arena)` crosses a `KindGT`'s groups into another frame.
//!
//! Everything is arena-allocated in the per-check `'g` bump: compound `KindGT` payloads and the
//! recursive `GroupExprG` bases live in `arena`; the group leaves borrow the `'s`/`'t` scout/typed
//! data; the `*TemplataG` wrappers of a non-`Kind` citizen arg are interned into `'t`.

use bumpalo::Bump;

use crate::postparsing::names::IRuneS;
use crate::postparsing::rules::types::{GroupS, ITypeST, RegionS};
use crate::typing::borrow_checker::group_expr::GroupExprG;
use crate::typing::borrow_checker::kind_g::{
  BoolGT, BorrowRefGT, FloatGT, IntGT, InterfaceGT, KindGT, KindPlaceholderGT, NeverGT, OverloadSetGT,
  OwnRefGT, RuntimeSizedArrayGT, ShareRefGT, StaticSizedArrayGT, StrGT, StructGT, USizeGT, VoidGT,
  WeakRefGT,
};
use crate::typing::borrow_checker::templata_g::{
  ExternFunctionTemplataG, FunctionTemplataG, GroupTemplataG, ITemplataG, ImplDefinitionTemplataG,
  InterfaceDefinitionTemplataG, IsaTemplataG, KindListTemplataG, KindTemplataG, PlaceholderTemplataG,
  PrototypeTemplataG, RuntimeSizedArrayTemplateTemplataG, StaticSizedArrayTemplateTemplataG,
  StructDefinitionTemplataG,
};
use crate::typing::compiler::Compiler;
use crate::typing::names::names::{IdT, INameT, IVarNameT};
use crate::typing::templata::templata::ITemplataT;
use crate::typing::types::types::KindT;
use crate::utils::fx::IndexMap;
use std::marker::PhantomData;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
  /// Build a `KindGT`: structure from `kind` (the typing pass's `KindT`), and a group for every borrow
  /// layer from the written `tyype`, walked in parallel. `param_name` keys an unannotated borrow's
  /// anonymous group (`None` outside a parameter). Where the typed kind carries a wrap the written type
  /// never had, the outer borrow takes the parameter's anonymous group and the subtree is groupless
  /// (`make_kind_g_groupless`). Pure.
  pub fn make_kind_g<'g>(
    &self,
    kind: KindT<'s, 't>,
    tyype: &'s ITypeST<'s>,
    param_name: Option<&'t IVarNameT<'s, 't>>,
    arena: &'g Bump,
  ) -> KindGT<'s, 't, 'g> {
    match kind {
      KindT::Never(x) => KindGT::Never(NeverGT { from_break: x.from_break }),
      KindT::Void(_) => KindGT::Void(VoidGT),
      KindT::Int(x) => KindGT::Int(IntGT { bits: x.bits }),
      KindT::Bool(_) => KindGT::Bool(BoolGT),
      KindT::Str(_) => KindGT::Str(StrGT),
      KindT::Float(_) => KindGT::Float(FloatGT),
      KindT::USize(_) => KindGT::USize(USizeGT),
      KindT::KindPlaceholder(p) => {
        KindGT::KindPlaceholder(arena.alloc(KindPlaceholderGT { id: p.id, _phantom: PhantomData }))
      }
      KindT::OverloadSet(o) => {
        KindGT::OverloadSet(arena.alloc(OverloadSetGT { env_id: o.env.id(), _phantom: PhantomData }))
      }

      KindT::Struct(s) => KindGT::Struct(arena.alloc(StructGT {
        id: s.id,
        template_args: self.citizen_args(self.citizen_template_args(*s.id), arena),
      })),
      KindT::Interface(i) => KindGT::Interface(arena.alloc(InterfaceGT {
        id: i.id,
        template_args: self.citizen_args(self.citizen_template_args(*i.id), arena),
      })),

      // A static-sized array is written `StaticArray<N, T>` — a `Call` whose second arg is the element.
      KindT::StaticSizedArray(a) => match tyype {
        ITypeST::Call(c) if c.args.len() == 2 => KindGT::StaticSizedArray(arena.alloc(StaticSizedArrayGT {
          name: a.name,
          element_type: arena.alloc(self.make_kind_g(a.element_type(), c.args[1], param_name, arena)),
        })),
        _ => self.make_kind_g_groupless(kind, arena),
      },
      KindT::RuntimeSizedArray(a) => match tyype {
        ITypeST::RuntimeSizedArray(st) => KindGT::RuntimeSizedArray(arena.alloc(RuntimeSizedArrayGT {
          name: a.name,
          element_type: arena.alloc(self.make_kind_g(a.element_type(), st.element, param_name, arena)),
        })),
        _ => self.make_kind_g_groupless(kind, arena),
      },

      KindT::BorrowRef(b) => match tyype {
        ITypeST::BorrowRef(st) => KindGT::BorrowRef(arena.alloc(BorrowRefGT {
          group: match st.region {
            RegionS::Group(gs) => group_expr_from_group_s(gs, arena),
            RegionS::Unspecified => group_anon(param_name),
            RegionS::Held => group_anon(param_name),
          },
          inner: self.make_kind_g(b.inner, st.inner, param_name, arena),
        })),
        _ => KindGT::BorrowRef(arena.alloc(BorrowRefGT {
          group: group_anon(param_name),
          inner: self.make_kind_g_groupless(b.inner, arena),
        })),
      },
      KindT::OwnRef(w) => match tyype {
        ITypeST::OwnRef(st) => {
          KindGT::OwnRef(arena.alloc(OwnRefGT { inner: self.make_kind_g(w.inner, st.inner, param_name, arena) }))
        }
        _ => KindGT::OwnRef(arena.alloc(OwnRefGT { inner: self.make_kind_g_groupless(w.inner, arena) })),
      },
      KindT::WeakRef(w) => match tyype {
        ITypeST::WeakRef(st) => {
          KindGT::WeakRef(arena.alloc(WeakRefGT { inner: self.make_kind_g(w.inner, st.inner, param_name, arena) }))
        }
        _ => KindGT::WeakRef(arena.alloc(WeakRefGT { inner: self.make_kind_g_groupless(w.inner, arena) })),
      },
      // A claim roots the ambient multi `rc`, so this layer carries no group; the written type is a bare
      // citizen, so the payload recurses against the same `tyype`.
      KindT::ShareRef(w) => {
        KindGT::ShareRef(arena.alloc(ShareRefGT { inner: self.make_kind_g(w.inner, tyype, param_name, arena) }))
      }
    }
  }

  /// A citizen kind's generic template args, off its `IdT`'s citizen name.
  fn citizen_template_args(&self, id: IdT<'s, 't>) -> &'t [ITemplataT<'s, 't>] {
    match id.local_name {
      INameT::Struct(n) => n.template_args,
      INameT::Interface(n) => n.template_args,
      _ => &[],
    }
  }

  /// A citizen's `KindGT` generic args: a plain structural mirror (template args carry no groups).
  fn citizen_args<'g>(
    &self,
    templata_args: &'t [ITemplataT<'s, 't>],
    arena: &'g Bump,
  ) -> &'g [ITemplataG<'s, 't>] {
    let v: Vec<ITemplataG<'s, 't>> = templata_args.iter().map(|t| self.templata_g(*t)).collect();
    arena.alloc_slice_fill_iter(v)
  }

  /// Mirror one typing templata into an `ITemplataG` — group-free and structural. A `Kind` carries a
  /// raw `KindT`; the solver-domain payloads become their `*TemplataG` wrappers, interned into `'t`; the
  /// ceremonial `Group` becomes the empty `GroupTemplataG`.
  fn templata_g(&self, templata: ITemplataT<'s, 't>) -> ITemplataG<'s, 't> {
    match templata {
      ITemplataT::Kind(k) => ITemplataG::Kind(KindTemplataG { kind: k.kind }),
      ITemplataT::Placeholder(p) => {
        ITemplataG::Placeholder(self.typing_interner.alloc(PlaceholderTemplataG { id: p.id, tyype: p.tyype }))
      }
      ITemplataT::Integer(v) => ITemplataG::Integer(v),
      ITemplataT::Boolean(v) => ITemplataG::Boolean(v),
      ITemplataT::String(v) => ITemplataG::String(v),
      ITemplataT::Prototype(p) => {
        ITemplataG::Prototype(self.typing_interner.alloc(PrototypeTemplataG { prototype: p.prototype }))
      }
      ITemplataT::Isa(isa) => ITemplataG::Isa(self.typing_interner.alloc(IsaTemplataG {
        declaration_range: isa.declaration_range,
        impl_name: isa.impl_name,
        sub_kind: isa.sub_kind,
        super_kind: isa.super_kind,
      })),
      ITemplataT::CoordList(list) => {
        ITemplataG::CoordList(self.typing_interner.alloc(KindListTemplataG { kinds: list.kinds }))
      }
      ITemplataT::RuntimeSizedArrayTemplate(_) => {
        ITemplataG::RuntimeSizedArrayTemplate(RuntimeSizedArrayTemplateTemplataG {})
      }
      ITemplataT::StaticSizedArrayTemplate(_) => {
        ITemplataG::StaticSizedArrayTemplate(StaticSizedArrayTemplateTemplataG {})
      }
      // Ceremonial: the group-param value never enters a `KindT` and is never read.
      ITemplataT::Group(_) => ITemplataG::Group(GroupTemplataG {}),
      ITemplataT::Function(f) => ITemplataG::Function(
        self.typing_interner.alloc(FunctionTemplataG { function_template_id: f.function_template_id }),
      ),
      ITemplataT::StructDefinition(d) => {
        ITemplataG::StructDefinition(self.typing_interner.alloc(StructDefinitionTemplataG {
          struct_template_id: d.struct_template_id,
          tyype: d.tyype,
        }))
      }
      ITemplataT::InterfaceDefinition(d) => {
        ITemplataG::InterfaceDefinition(self.typing_interner.alloc(InterfaceDefinitionTemplataG {
          interface_template_id: d.interface_template_id,
          tyype: d.tyype,
        }))
      }
      ITemplataT::ImplDefinition(d) => ITemplataG::ImplDefinition(
        self.typing_interner.alloc(ImplDefinitionTemplataG { impl_template_id: d.impl_template_id }),
      ),
      ITemplataT::ExternFunction(f) => {
        ITemplataG::ExternFunction(self.typing_interner.alloc(ExternFunctionTemplataG { header: f.header }))
      }
    }
  }

  /// Build a `KindGT` from a `KindT` with no written type. Every borrow layer would need a group it
  /// cannot derive here, so a borrow panics (a deferred case); non-borrows mirror structurally, with
  /// group-free citizen args.
  pub(crate) fn make_kind_g_groupless<'g>(&self, kind: KindT<'s, 't>, arena: &'g Bump) -> KindGT<'s, 't, 'g> {
    match kind {
      KindT::Never(x) => KindGT::Never(NeverGT { from_break: x.from_break }),
      KindT::Void(_) => KindGT::Void(VoidGT),
      KindT::Int(x) => KindGT::Int(IntGT { bits: x.bits }),
      KindT::Bool(_) => KindGT::Bool(BoolGT),
      KindT::Str(_) => KindGT::Str(StrGT),
      KindT::Float(_) => KindGT::Float(FloatGT),
      KindT::USize(_) => KindGT::USize(USizeGT),
      KindT::KindPlaceholder(p) => {
        KindGT::KindPlaceholder(arena.alloc(KindPlaceholderGT { id: p.id, _phantom: PhantomData }))
      }
      KindT::OverloadSet(o) => {
        KindGT::OverloadSet(arena.alloc(OverloadSetGT { env_id: o.env.id(), _phantom: PhantomData }))
      }
      KindT::Struct(s) => KindGT::Struct(arena.alloc(StructGT {
        id: s.id,
        template_args: self.citizen_args(self.citizen_template_args(*s.id), arena),
      })),
      KindT::Interface(i) => KindGT::Interface(arena.alloc(InterfaceGT {
        id: i.id,
        template_args: self.citizen_args(self.citizen_template_args(*i.id), arena),
      })),
      KindT::StaticSizedArray(a) => KindGT::StaticSizedArray(arena.alloc(StaticSizedArrayGT {
        name: a.name,
        element_type: arena.alloc(self.make_kind_g_groupless(a.element_type(), arena)),
      })),
      KindT::RuntimeSizedArray(a) => KindGT::RuntimeSizedArray(arena.alloc(RuntimeSizedArrayGT {
        name: a.name,
        element_type: arena.alloc(self.make_kind_g_groupless(a.element_type(), arena)),
      })),
      KindT::BorrowRef(_) => panic!(
        "vfail: borrow with no derivable group — a deferred case (closure capture / weak-nested / \
         nested reference field); see docs/plans/group-generic-closures-plan.md"
      ),
      KindT::OwnRef(w) => KindGT::OwnRef(arena.alloc(OwnRefGT { inner: self.make_kind_g_groupless(w.inner, arena) })),
      KindT::ShareRef(w) => {
        KindGT::ShareRef(arena.alloc(ShareRefGT { inner: self.make_kind_g_groupless(w.inner, arena) }))
      }
      KindT::WeakRef(w) => {
        KindGT::WeakRef(arena.alloc(WeakRefGT { inner: self.make_kind_g_groupless(w.inner, arena) }))
      }
    }
  }

  /// Cross a `KindGT`'s groups into another frame: rewrite each borrow's `GroupExprG` through `subst`
  /// (a callee group rune → the caller group it was bound to). Structure is unchanged; template args
  /// carry no groups, so they are copied through.
  pub fn substitute_groups<'g>(
    &self,
    kindg: KindGT<'s, 't, 'g>,
    subst: &IndexMap<IRuneS<'s>, GroupExprG<'s, 't, 'g>>,
    arena: &'g Bump,
  ) -> KindGT<'s, 't, 'g> {
    match kindg {
      KindGT::Never(_) | KindGT::Void(_) | KindGT::Int(_) | KindGT::Bool(_) | KindGT::Str(_)
      | KindGT::Float(_) | KindGT::USize(_) | KindGT::KindPlaceholder(_) | KindGT::OverloadSet(_)
      | KindGT::Struct(_) | KindGT::Interface(_) => kindg,
      KindGT::StaticSizedArray(a) => KindGT::StaticSizedArray(arena.alloc(StaticSizedArrayGT {
        name: a.name,
        element_type: arena.alloc(self.substitute_groups(*a.element_type, subst, arena)),
      })),
      KindGT::RuntimeSizedArray(a) => KindGT::RuntimeSizedArray(arena.alloc(RuntimeSizedArrayGT {
        name: a.name,
        element_type: arena.alloc(self.substitute_groups(*a.element_type, subst, arena)),
      })),
      KindGT::BorrowRef(b) => KindGT::BorrowRef(arena.alloc(BorrowRefGT {
        group: subst_group_expr(b.group, subst, arena),
        inner: self.substitute_groups(b.inner, subst, arena),
      })),
      KindGT::OwnRef(w) => KindGT::OwnRef(arena.alloc(OwnRefGT { inner: self.substitute_groups(w.inner, subst, arena) })),
      KindGT::ShareRef(w) => KindGT::ShareRef(arena.alloc(ShareRefGT { inner: self.substitute_groups(w.inner, subst, arena) })),
      KindGT::WeakRef(w) => KindGT::WeakRef(arena.alloc(WeakRefGT { inner: self.substitute_groups(w.inner, subst, arena) })),
    }
  }
}

/// Convert a scout-side `GroupS` to a `GroupExprG`, allocating recursive bases in `arena`. A group rune
/// carries its own scout identity (`&'s IRuneS`), so leaves need no frame. `Elements` maps to
/// `ChildElements` (the destructible collection child group — the only kind a written group produces).
pub(crate) fn group_expr_from_group_s<'s, 't, 'g>(
  group: &'s GroupS<'s>,
  arena: &'g Bump,
) -> GroupExprG<'s, 't, 'g> {
  match group {
    GroupS::Rune(ru) => GroupExprG::Rune(&ru.rune),
    GroupS::Local(_) => panic!(
      "vfail: a group written as a local name (`in x`) is not yet supported; see \
       docs/plans/group-generic-closures-plan.md"
    ),
    GroupS::Member { base, member_name } => GroupExprG::Member {
      base: arena.alloc(group_expr_from_group_s(base, arena)),
      member_name: *member_name,
    },
    GroupS::Elements { base } => {
      GroupExprG::ChildElements { base: arena.alloc(group_expr_from_group_s(base, arena)) }
    }
    GroupS::Ellipsis { base } => {
      GroupExprG::Ellipsis { base: arena.alloc(group_expr_from_group_s(base, arena)) }
    }
    GroupS::Union { members } => {
      let ms: Vec<&'g GroupExprG<'s, 't, 'g>> =
        members.iter().map(|m| &*arena.alloc(group_expr_from_group_s(m, arena))).collect();
      GroupExprG::Union { members: arena.alloc_slice_fill_iter(ms) }
    }
  }
}

/// Rewrite a `GroupExprG`'s rune leaves through `subst`; leave anonymous-param, locals and structure,
/// reallocating rewritten bases in `arena`.
pub(crate) fn subst_group_expr<'s, 't, 'g>(
  group: GroupExprG<'s, 't, 'g>,
  subst: &IndexMap<IRuneS<'s>, GroupExprG<'s, 't, 'g>>,
  arena: &'g Bump,
) -> GroupExprG<'s, 't, 'g> {
  match group {
    // VLOOOOK: fallback — needs VFALLBACK approval or removal
    GroupExprG::Rune(rune) => subst.get(rune).copied().unwrap_or(GroupExprG::Rune(rune)),
    GroupExprG::ParamAnonymousGroup(name) => GroupExprG::ParamAnonymousGroup(name),
    GroupExprG::Local(id) => GroupExprG::Local(id),
    GroupExprG::Member { base, member_name } => GroupExprG::Member {
      base: arena.alloc(subst_group_expr(*base, subst, arena)),
      member_name,
    },
    GroupExprG::ChildElements { base } => {
      GroupExprG::ChildElements { base: arena.alloc(subst_group_expr(*base, subst, arena)) }
    }
    GroupExprG::InlineElements { base } => {
      GroupExprG::InlineElements { base: arena.alloc(subst_group_expr(*base, subst, arena)) }
    }
    GroupExprG::Variant { base, variant_name } => GroupExprG::Variant {
      base: arena.alloc(subst_group_expr(*base, subst, arena)),
      variant_name,
    },
    GroupExprG::Ellipsis { base } => {
      GroupExprG::Ellipsis { base: arena.alloc(subst_group_expr(*base, subst, arena)) }
    }
    GroupExprG::Union { members } => {
      let ms: Vec<&'g GroupExprG<'s, 't, 'g>> =
        members.iter().map(|m| &*arena.alloc(subst_group_expr(**m, subst, arena))).collect();
      GroupExprG::Union { members: arena.alloc_slice_fill_iter(ms) }
    }
  }
}

/// A borrow's group when its written type carries no `in g`: the parameter's anonymous group. Only the
/// surface-most borrow of a parameter reaches here; a borrow with no `in g` and no parameter context
/// has no derivable group (a deferred case) and panics.
fn group_anon<'s, 't, 'g>(param_name: Option<&'t IVarNameT<'s, 't>>) -> GroupExprG<'s, 't, 'g> {
  param_name.map(GroupExprG::ParamAnonymousGroup).unwrap_or_else(|| {
    panic!(
      "vfail: borrow with no group and no parameter context — a deferred case; see \
       docs/plans/group-generic-closures-plan.md"
    )
  })
}

/// Install `group` as the outermost borrow's group of a value type — used when a body place expression
/// derives its referent group from the place path rather than from a written `in g`. A non-borrow
/// result is returned unchanged.
pub(crate) fn with_outer_group<'s, 't, 'g>(
  kind: KindGT<'s, 't, 'g>,
  group: GroupExprG<'s, 't, 'g>,
  arena: &'g Bump,
) -> KindGT<'s, 't, 'g> {
  match kind {
    KindGT::BorrowRef(b) => KindGT::BorrowRef(arena.alloc(BorrowRefGT { group, inner: b.inner })),
    other => other,
  }
}
