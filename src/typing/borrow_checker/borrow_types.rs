//! Build the borrow checker's group-annotated `KindGT` for a value, and re-express its groups in
//! another frame.
//!
//! `KindGT` mirrors the typing pass's `KindT` variant-for-variant, and `ITemplataG` mirrors
//! `ITemplataT`, so no structure is lost. The only additions are a `GroupExprG` on each borrow and
//! `KindGT` in place of every nested `KindT`. Groups are read from the written `ITypeST` at every
//! depth — including borrows nested inside generic type arguments, as in `&Opt<&Thing in g> in d`.
//! See docs/architecture/borrowing-design.md.
//!
//! The mirror stops at the solver's group-free domain: the signature/definition templatas
//! (`Prototype`, `Function`, `*Definition`, `ExternFunction`) reuse their `T` payload, because groups
//! never flow through the solver, so mirroring the whole signature machinery would carry no group.
//!
//! Two entry points:
//!  * `make_kind_g(kind, tyype, param_name)` builds a `KindGT` with groups in `tyype`'s own frame.
//!  * `substitute_groups(kindg, subst)` crosses a `KindGT`'s groups into another frame — the same
//!    operation as `substitute_templatas_in_kind`. A call result needs this to rename the callee's
//!    group runes to the caller's; a parameter does not (empty substitution).

use crate::interner::StrI;
use crate::postparsing::names::{IImpreciseNameS, IRuneS};
use crate::postparsing::rules::types::{GroupS, ITypeST, RegionS};
use crate::typing::compiler::Compiler;
use crate::typing::names::names::{IdT, INameT, IVarNameT};
use crate::typing::templata::templata::{
  ExternFunctionTemplataT, FunctionTemplataT, ITemplataT, ImplDefinitionTemplataT,
  InterfaceDefinitionTemplataT, IsaTemplataT, KindListTemplataT, PlaceholderTemplataT,
  PrototypeTemplataT, RuntimeSizedArrayTemplateTemplataT, StaticSizedArrayTemplateTemplataT,
  StructDefinitionTemplataT,
};
use crate::typing::types::types::{
  BoolT, FloatT, IntT, KindPlaceholderT, KindT, NeverT, OverloadSetT, StrT, USizeT, VoidT,
};
use crate::utils::fx::IndexMap;

/// Which group(s) a borrow points into, or a function mutates. Borrow-checker-only; leaves carry
/// their existing scout/typed identities (`IRuneS`, a param name, a local name) — the checker mints
/// no ids. Mirrors the scout-side `GroupS`, with an `rc`-style anonymous param group.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum GroupExprG<'s, 't> {
  /// A group param, e.g. `<g'>` — the scout rune itself; the checker mints no id.
  Rune(IRuneS<'s>),
  /// A param's group when it comes from neither a rune nor another param, keyed by the param's
  /// interned `IVarNameT` — so it names an unnamed param (`self`, a magic/closure param) too.
  ParamAnonymousGroup(IVarNameT<'s, 't>),
  /// A local's implicitly-declared group, keyed by the local's interned `IVarNameT` — so it names a
  /// temporary (which has no source name) as well as a user local.
  Local(IVarNameT<'s, 't>),
  /// `x.items` — the named member.
  Member { base: Box<GroupExprG<'s, 't>>, member_name: StrI<'s> },
  /// `x.items[]` — an element of the member.
  Elements { base: Box<GroupExprG<'s, 't>> },
  /// `x...` — somewhere within x's territory (a descendant, exact spot unknown).
  Ellipsis { base: Box<GroupExprG<'s, 't>> },
  /// `(a | b)` — a union of groups.
  Union { members: Vec<GroupExprG<'s, 't>> },
}

/// The group-annotated mirror of `KindT`: the same variants, plus a group on each `BorrowRef` and
/// `KindGT` wherever `KindT` nests. Borrow-checker-only; not interned, not `Copy`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum KindGT<'s, 't> {
  Never(NeverT),
  Void(VoidT),
  Int(IntT),
  Bool(BoolT),
  Str(StrT),
  Float(FloatT),
  USize(USizeT),
  Struct(StructGT<'s, 't>),
  Interface(InterfaceGT<'s, 't>),
  StaticSizedArray(StaticSizedArrayGT<'s, 't>),
  RuntimeSizedArray(RuntimeSizedArrayGT<'s, 't>),
  /// A placeholder represents a kind but nests none, so it reuses the `T` payload.
  KindPlaceholder(&'t KindPlaceholderT<'s, 't>),
  /// An overload set nests no kind, so it reuses the `T` payload.
  OverloadSet(&'t OverloadSetT<'s, 't>),
  BorrowRef(BorrowRefGT<'s, 't>),
  OwnRef(Box<KindGT<'s, 't>>),
  ShareRef(Box<KindGT<'s, 't>>),
  WeakRef(Box<KindGT<'s, 't>>),
}

/// Mirror of `StructTT`: its `id` for identity, plus its `KindGT`-ified generic args so a group
/// nested in an argument survives.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructGT<'s, 't> {
  pub id: IdT<'s, 't>,
  pub template_args: Vec<ITemplataG<'s, 't>>,
}

/// Mirror of `InterfaceTT`. See `StructGT`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceGT<'s, 't> {
  pub id: IdT<'s, 't>,
  pub template_args: Vec<ITemplataG<'s, 't>>,
}

/// Mirror of `StaticSizedArrayTT`, with a `KindGT` element.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayGT<'s, 't> {
  pub id: IdT<'s, 't>,
  pub size: ITemplataG<'s, 't>,
  pub element: Box<KindGT<'s, 't>>,
}

/// Mirror of `RuntimeSizedArrayTT`, with a `KindGT` element.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayGT<'s, 't> {
  pub id: IdT<'s, 't>,
  pub element: Box<KindGT<'s, 't>>,
}

/// Mirror of `BorrowRefT`, plus the group it borrows into.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct BorrowRefGT<'s, 't> {
  pub group: GroupExprG<'s, 't>,
  pub inner: Box<KindGT<'s, 't>>,
}

/// Mirror of `ITemplataT`: every nested `KindT` becomes `KindGT`. The solver's group-free templatas
/// (prototypes, definitions, extern functions) reuse their `T` payload — see the module header.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum ITemplataG<'s, 't> {
  Kind(KindTemplataG<'s, 't>),
  Group(GroupExprG<'s, 't>),
  Placeholder(&'t PlaceholderTemplataT<'s, 't>),
  Integer(i64),
  Boolean(bool),
  String(StrI<'s>),
  Prototype(&'t PrototypeTemplataT<'s, 't>),
  Isa(&'t IsaTemplataT<'s, 't>),
  CoordList(&'t KindListTemplataT<'s, 't>),
  RuntimeSizedArrayTemplate(RuntimeSizedArrayTemplateTemplataT),
  StaticSizedArrayTemplate(StaticSizedArrayTemplateTemplataT),
  Function(&'t FunctionTemplataT<'s, 't>),
  StructDefinition(&'t StructDefinitionTemplataT<'s, 't>),
  InterfaceDefinition(&'t InterfaceDefinitionTemplataT<'s, 't>),
  ImplDefinition(&'t ImplDefinitionTemplataT<'s, 't>),
  ExternFunction(&'t ExternFunctionTemplataT<'s, 't>),
}

/// Mirror of `KindTemplataT`, with a `KindGT` (boxed, since `KindGT` nests it back).
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct KindTemplataG<'s, 't> {
  pub kind: Box<KindGT<'s, 't>>,
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
  /// Build a `KindGT`: structure from `kind` (the typing pass's `KindT`), and a group for every borrow
  /// layer at every depth from the written `tyype`, walked in parallel. `param_name` keys an
  /// unannotated borrow's anonymous group (`None` outside a parameter). Where the typed kind carries a
  /// wrap the written type never had — a generic parameter `x T` instantiated with a reference, whose
  /// written type is a bare rune, or the bare-class position rule — the written type gives no `in g`,
  /// so the outer borrow takes the parameter's anonymous group and the subtree is groupless
  /// (`make_kind_g_groupless`). Pure.
  pub fn make_kind_g(
    &self,
    kind: KindT<'s, 't>,
    tyype: &'s ITypeST<'s>,
    param_name: Option<IVarNameT<'s, 't>>,
  ) -> KindGT<'s, 't> {
    match kind {
      KindT::Never(x) => KindGT::Never(x),
      KindT::Void(x) => KindGT::Void(x),
      KindT::Int(x) => KindGT::Int(x),
      KindT::Bool(x) => KindGT::Bool(x),
      KindT::Str(x) => KindGT::Str(x),
      KindT::Float(x) => KindGT::Float(x),
      KindT::USize(x) => KindGT::USize(x),
      KindT::KindPlaceholder(p) => KindGT::KindPlaceholder(p),
      KindT::OverloadSet(o) => KindGT::OverloadSet(o),

      KindT::Struct(s) => KindGT::Struct(StructGT {
        id: *s.id,
        template_args: self.make_citizen_args(self.citizen_template_args(*s.id), tyype, param_name),
      }),
      KindT::Interface(i) => KindGT::Interface(InterfaceGT {
        id: *i.id,
        template_args: self.make_citizen_args(self.citizen_template_args(*i.id), tyype, param_name),
      }),

      // A static-sized array is written `StaticArray<N, T>` — a `Call` whose second arg is the element.
      KindT::StaticSizedArray(a) => match tyype {
        ITypeST::Call(c) if c.args.len() == 2 => KindGT::StaticSizedArray(StaticSizedArrayGT {
          id: a.name,
          size: self.make_templata_g(a.size(), None, param_name),
          element: Box::new(self.make_kind_g(a.element_type(), c.args[1], param_name)),
        }),
        // The written type is a bare rune (a generic parameter instantiated with an array): no written
        // groups to read, so the value is groupless.
        _ => self.make_kind_g_groupless(kind),
      },
      KindT::RuntimeSizedArray(a) => match tyype {
        ITypeST::RuntimeSizedArray(st) => KindGT::RuntimeSizedArray(RuntimeSizedArrayGT {
          id: a.name,
          element: Box::new(self.make_kind_g(a.element_type(), st.element, param_name)),
        }),
        _ => self.make_kind_g_groupless(kind),
      },

      KindT::BorrowRef(b) => match tyype {
        ITypeST::BorrowRef(st) => KindGT::BorrowRef(BorrowRefGT {
          group: match st.region {
            RegionS::Group(gs) => group_expr_from_group_s(gs),
            // A borrow written without an `in g` gets a fresh anonymous group keyed by the parameter.
            RegionS::Unspecified => group_anon(param_name),
            // A `held` borrow's own group representation is deferred; conservatively the parameter's
            // anonymous group.
            RegionS::Held => group_anon(param_name),
          },
          inner: Box::new(self.make_kind_g(b.inner, st.inner, param_name)),
        }),
        // The typed kind is a borrow but the written type is not — a generic parameter `x T`
        // instantiated with a reference (written type a bare rune `T`), or the bare-class position
        // rule (written type a name). The written type gives no `in g`, so the borrow takes the
        // parameter's anonymous group and its referent is groupless.
        _ => KindGT::BorrowRef(BorrowRefGT {
          group: group_anon(param_name),
          inner: Box::new(self.make_kind_g_groupless(b.inner)),
        }),
      },
      KindT::OwnRef(w) => match tyype {
        ITypeST::OwnRef(st) => KindGT::OwnRef(Box::new(self.make_kind_g(w.inner, st.inner, param_name))),
        _ => KindGT::OwnRef(Box::new(self.make_kind_g_groupless(w.inner))),
      },
      KindT::WeakRef(w) => match tyype {
        ITypeST::WeakRef(st) => KindGT::WeakRef(Box::new(self.make_kind_g(w.inner, st.inner, param_name))),
        _ => KindGT::WeakRef(Box::new(self.make_kind_g_groupless(w.inner))),
      },
      // A claim roots the ambient multi `rc`, so this layer carries no group. The written type is a
      // bare citizen (no wrap), so the payload recurses against the same `tyype`.
      KindT::ShareRef(w) => KindGT::ShareRef(Box::new(self.make_kind_g(w.inner, tyype, param_name))),
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

  /// Build a citizen's `KindGT` generic args. Each `Kind` templata's group comes from the
  /// positionally-matching written `Call` arg; a non-`Kind` templata has no written type.
  fn make_citizen_args(
    &self,
    templata_args: &'t [ITemplataT<'s, 't>],
    tyype: &'s ITypeST<'s>,
    param_name: Option<IVarNameT<'s, 't>>,
  ) -> Vec<ITemplataG<'s, 't>> {
    // The written `Call`'s args line up positionally with the citizen's template args; a non-`Call`
    // written type has none.
    let written_args: &[&'s ITypeST<'s>] = match tyype {
      ITypeST::Call(c) => c.args,
      _ => &[],
    };
    templata_args
      .iter()
      .enumerate()
      .map(|(i, t)| self.make_templata_g(*t, written_args.get(i).copied(), param_name))
      .collect()
  }

  /// Mirror one templata into an `ITemplataG`. Only a `Kind` templata carries a group, read from its
  /// written type; the rest are group-free and pass through (the solver-domain `Isa`/`CoordList` reuse
  /// their `T` payload, since they never come from a user-written type).
  fn make_templata_g(
    &self,
    templata: ITemplataT<'s, 't>,
    written: Option<&'s ITypeST<'s>>,
    param_name: Option<IVarNameT<'s, 't>>,
  ) -> ITemplataG<'s, 't> {
    match templata {
      ITemplataT::Kind(k) => match written {
        Some(w) => ITemplataG::Kind(KindTemplataG { kind: Box::new(self.make_kind_g(k.kind, w, param_name)) }),
        // A citizen argument with no written type (the citizen was written as a bare rune, or the arg
        // list did not line up): no written groups to read, so the argument's kind is groupless.
        None => ITemplataG::Kind(KindTemplataG { kind: Box::new(self.make_kind_g_groupless(k.kind)) }),
      },
      ITemplataT::Placeholder(p) => ITemplataG::Placeholder(p),
      ITemplataT::Integer(v) => ITemplataG::Integer(v),
      ITemplataT::Boolean(v) => ITemplataG::Boolean(v),
      ITemplataT::String(v) => ITemplataG::String(v),
      ITemplataT::Prototype(p) => ITemplataG::Prototype(p),
      ITemplataT::Isa(isa) => ITemplataG::Isa(isa),
      ITemplataT::CoordList(list) => ITemplataG::CoordList(list),
      ITemplataT::RuntimeSizedArrayTemplate(x) => ITemplataG::RuntimeSizedArrayTemplate(x),
      ITemplataT::StaticSizedArrayTemplate(x) => ITemplataG::StaticSizedArrayTemplate(x),
      // The `T` value is inert (a group may not ride a `KindT`), so the real group comes from the
      // written arg — a group argument scouts to a rune reference (`Overlay<a>` → `Rune(a)`).
      ITemplataT::Group(_) => match written {
        Some(ITypeST::Rune(ru)) => ITemplataG::Group(GroupExprG::Rune(ru.rune.rune)),
        // A group argument written as a union/path, or with no written source: not yet derivable (a
        // deferred case). No empty groups exist, so this panics rather than producing one.
        _ => panic!(
          "vfail: group argument written as a union/path or with no source — a deferred case; see \
           docs/plans/group-generic-closures-plan.md"
        ),
      },
      ITemplataT::Function(f) => ITemplataG::Function(f),
      ITemplataT::StructDefinition(d) => ITemplataG::StructDefinition(d),
      ITemplataT::InterfaceDefinition(d) => ITemplataG::InterfaceDefinition(d),
      ITemplataT::ImplDefinition(d) => ITemplataG::ImplDefinition(d),
      ITemplataT::ExternFunction(f) => ITemplataG::ExternFunction(f),
    }
  }

  /// Build a `KindGT` from a `KindT` with no written type — every borrow gets the empty group. This is
  /// the value a generic parameter instantiated with a wrapped type takes: its written type is a bare
  /// rune, so there are no `in g` annotations to read, and a churn never matches the empty group (the
  /// sound conservative reading).
  pub(crate) fn make_kind_g_groupless(&self, kind: KindT<'s, 't>) -> KindGT<'s, 't> {
    match kind {
      KindT::Never(x) => KindGT::Never(x),
      KindT::Void(x) => KindGT::Void(x),
      KindT::Int(x) => KindGT::Int(x),
      KindT::Bool(x) => KindGT::Bool(x),
      KindT::Str(x) => KindGT::Str(x),
      KindT::Float(x) => KindGT::Float(x),
      KindT::USize(x) => KindGT::USize(x),
      KindT::KindPlaceholder(p) => KindGT::KindPlaceholder(p),
      KindT::OverloadSet(o) => KindGT::OverloadSet(o),
      KindT::Struct(s) => KindGT::Struct(StructGT {
        id: *s.id,
        template_args: self.citizen_args_groupless(self.citizen_template_args(*s.id)),
      }),
      KindT::Interface(i) => KindGT::Interface(InterfaceGT {
        id: *i.id,
        template_args: self.citizen_args_groupless(self.citizen_template_args(*i.id)),
      }),
      KindT::StaticSizedArray(a) => KindGT::StaticSizedArray(StaticSizedArrayGT {
        id: a.name,
        size: self.templata_groupless(a.size()),
        element: Box::new(self.make_kind_g_groupless(a.element_type())),
      }),
      KindT::RuntimeSizedArray(a) => KindGT::RuntimeSizedArray(RuntimeSizedArrayGT {
        id: a.name,
        element: Box::new(self.make_kind_g_groupless(a.element_type())),
      }),
      // A borrow reached here has no written type to read a group from and no place-path source to
      // derive one — a deferred case (a closure-captured reference, a weak lock's `Opt<&T>`, or another
      // nested borrow in a not-yet-supported feature). No groupless borrow may exist, so panic loudly.
      // See docs/plans/group-generic-closures-plan.md.
      KindT::BorrowRef(_) => panic!(
        "vfail: borrow with no derivable group — a deferred case (closure capture / weak-nested / \
         nested reference field); see docs/plans/group-generic-closures-plan.md"
      ),
      KindT::OwnRef(w) => KindGT::OwnRef(Box::new(self.make_kind_g_groupless(w.inner))),
      KindT::ShareRef(w) => KindGT::ShareRef(Box::new(self.make_kind_g_groupless(w.inner))),
      KindT::WeakRef(w) => KindGT::WeakRef(Box::new(self.make_kind_g_groupless(w.inner))),
    }
  }

  /// A citizen's generic arguments, groupless.
  fn citizen_args_groupless(&self, templata_args: &'t [ITemplataT<'s, 't>]) -> Vec<ITemplataG<'s, 't>> {
    templata_args.iter().map(|t| self.templata_groupless(*t)).collect()
  }

  /// Mirror one templata with no written type — a `Kind` argument becomes groupless, a `Group`
  /// argument the empty group, the rest pass through.
  fn templata_groupless(&self, templata: ITemplataT<'s, 't>) -> ITemplataG<'s, 't> {
    match templata {
      ITemplataT::Kind(k) => {
        ITemplataG::Kind(KindTemplataG { kind: Box::new(self.make_kind_g_groupless(k.kind)) })
      }
      // A group argument with no written source has no derivable group (a deferred case). No empty
      // groups exist, so this panics rather than producing one.
      ITemplataT::Group(_) => panic!(
        "vfail: group argument with no derivable group — a deferred case; see \
         docs/plans/group-generic-closures-plan.md"
      ),
      ITemplataT::Placeholder(p) => ITemplataG::Placeholder(p),
      ITemplataT::Integer(v) => ITemplataG::Integer(v),
      ITemplataT::Boolean(v) => ITemplataG::Boolean(v),
      ITemplataT::String(v) => ITemplataG::String(v),
      ITemplataT::Prototype(p) => ITemplataG::Prototype(p),
      ITemplataT::Isa(isa) => ITemplataG::Isa(isa),
      ITemplataT::CoordList(list) => ITemplataG::CoordList(list),
      ITemplataT::RuntimeSizedArrayTemplate(x) => ITemplataG::RuntimeSizedArrayTemplate(x),
      ITemplataT::StaticSizedArrayTemplate(x) => ITemplataG::StaticSizedArrayTemplate(x),
      ITemplataT::Function(f) => ITemplataG::Function(f),
      ITemplataT::StructDefinition(d) => ITemplataG::StructDefinition(d),
      ITemplataT::InterfaceDefinition(d) => ITemplataG::InterfaceDefinition(d),
      ITemplataT::ImplDefinition(d) => ITemplataG::ImplDefinition(d),
      ITemplataT::ExternFunction(f) => ITemplataG::ExternFunction(f),
    }
  }

  /// Cross a `KindGT`'s groups into another frame: rewrite each borrow's `GroupExprG` through `subst`
  /// (a callee group rune → the caller group it was bound to). Structure is unchanged.
  pub fn substitute_groups(
    &self,
    kindg: &KindGT<'s, 't>,
    subst: &IndexMap<IRuneS<'s>, GroupExprG<'s, 't>>,
  ) -> KindGT<'s, 't> {
    match kindg {
      KindGT::Never(x) => KindGT::Never(*x),
      KindGT::Void(x) => KindGT::Void(*x),
      KindGT::Int(x) => KindGT::Int(*x),
      KindGT::Bool(x) => KindGT::Bool(*x),
      KindGT::Str(x) => KindGT::Str(*x),
      KindGT::Float(x) => KindGT::Float(*x),
      KindGT::USize(x) => KindGT::USize(*x),
      KindGT::KindPlaceholder(p) => KindGT::KindPlaceholder(p),
      KindGT::OverloadSet(o) => KindGT::OverloadSet(o),
      KindGT::Struct(s) => KindGT::Struct(StructGT {
        id: s.id,
        template_args: s.template_args.iter().map(|a| self.substitute_templata_g(a, subst)).collect(),
      }),
      KindGT::Interface(i) => KindGT::Interface(InterfaceGT {
        id: i.id,
        template_args: i.template_args.iter().map(|a| self.substitute_templata_g(a, subst)).collect(),
      }),
      KindGT::StaticSizedArray(a) => KindGT::StaticSizedArray(StaticSizedArrayGT {
        id: a.id,
        size: self.substitute_templata_g(&a.size, subst),
        element: Box::new(self.substitute_groups(&a.element, subst)),
      }),
      KindGT::RuntimeSizedArray(a) => KindGT::RuntimeSizedArray(RuntimeSizedArrayGT {
        id: a.id,
        element: Box::new(self.substitute_groups(&a.element, subst)),
      }),
      KindGT::BorrowRef(b) => KindGT::BorrowRef(BorrowRefGT {
        group: subst_group_expr(&b.group, subst),
        inner: Box::new(self.substitute_groups(&b.inner, subst)),
      }),
      KindGT::OwnRef(inner) => KindGT::OwnRef(Box::new(self.substitute_groups(inner, subst))),
      KindGT::ShareRef(inner) => KindGT::ShareRef(Box::new(self.substitute_groups(inner, subst))),
      KindGT::WeakRef(inner) => KindGT::WeakRef(Box::new(self.substitute_groups(inner, subst))),
    }
  }

  fn substitute_templata_g(
    &self,
    templata: &ITemplataG<'s, 't>,
    subst: &IndexMap<IRuneS<'s>, GroupExprG<'s, 't>>,
  ) -> ITemplataG<'s, 't> {
    match templata {
      ITemplataG::Kind(k) => {
        ITemplataG::Kind(KindTemplataG { kind: Box::new(self.substitute_groups(&k.kind, subst)) })
      }
      // A citizen's group argument crosses frames like any other group.
      ITemplataG::Group(g) => ITemplataG::Group(subst_group_expr(g, subst)),
      // No nested group to rewrite (the solver-domain `Isa`/`CoordList` carry none).
      other => other.clone(),
    }
  }

}

/// Convert a scout-side `GroupS` to a `GroupExprG`. A group rune carries its own scout identity, so
/// this needs no frame; structure (member/elements/union) maps across unchanged. Shared with
/// `groupify` (the checker's other consumer of scout groups).
pub(crate) fn group_expr_from_group_s<'s, 't>(group: &GroupS<'s>) -> GroupExprG<'s, 't> {
  match group {
    GroupS::Rune(ru) => GroupExprG::Rune(ru.rune),
    // `in x` names a local, but a written group carries only a source name, not the local's interned
    // `IVarNameT`; resolving it needs scope, which this frame-free conversion lacks. It is not reached by
    // the supported paths (effects reach a caller local through rune substitution instead), so a group
    // written directly as a local name is a deferred case — panic rather than produce a groupless group.
    GroupS::Local(_) => panic!(
      "vfail: a group written as a local name (`in x`) is not yet supported; see \
       docs/plans/group-generic-closures-plan.md"
    ),
    GroupS::Member { base, member_name } => {
      GroupExprG::Member { base: Box::new(group_expr_from_group_s(base)), member_name: *member_name }
    }
    GroupS::Elements { base } => {
      GroupExprG::Elements { base: Box::new(group_expr_from_group_s(base)) }
    }
    GroupS::Ellipsis { base } => {
      GroupExprG::Ellipsis { base: Box::new(group_expr_from_group_s(base)) }
    }
    GroupS::Union { members } => {
      GroupExprG::Union { members: members.iter().map(|m| group_expr_from_group_s(m)).collect() }
    }
  }
}

/// Rewrite a `GroupExprG`'s rune leaves through `subst`; leave anonymous-param, locals and structure.
/// Shared with `groupify`.
pub(crate) fn subst_group_expr<'s, 't>(
  group: &GroupExprG<'s, 't>,
  subst: &IndexMap<IRuneS<'s>, GroupExprG<'s, 't>>,
) -> GroupExprG<'s, 't> {
  match group {
    // VLOOOOK: fallback — needs VFALLBACK approval or removal
    GroupExprG::Rune(rune) => subst.get(rune).cloned().unwrap_or(GroupExprG::Rune(*rune)),
    // Keyed by a param name, not a rune, so it does not cross frames through the rune substitution.
    GroupExprG::ParamAnonymousGroup(name) => GroupExprG::ParamAnonymousGroup(*name),
    // A local belongs to the current body, so it never crosses a frame.
    GroupExprG::Local(id) => GroupExprG::Local(*id),
    GroupExprG::Member { base, member_name } => {
      GroupExprG::Member { base: Box::new(subst_group_expr(base, subst)), member_name: *member_name }
    }
    GroupExprG::Elements { base } => {
      GroupExprG::Elements { base: Box::new(subst_group_expr(base, subst)) }
    }
    GroupExprG::Ellipsis { base } => {
      GroupExprG::Ellipsis { base: Box::new(subst_group_expr(base, subst)) }
    }
    GroupExprG::Union { members } => {
      GroupExprG::Union { members: members.iter().map(|m| subst_group_expr(m, subst)).collect() }
    }
  }
}

/// A borrow's group when its written type carries no `in g`: the parameter's anonymous group. Only the
/// surface-most borrow of a parameter reaches here; a borrow with no `in g` and no parameter context
/// has no derivable group (a deferred case) and panics.
fn group_anon<'s, 't>(param_name: Option<IVarNameT<'s, 't>>) -> GroupExprG<'s, 't> {
  param_name.map(GroupExprG::ParamAnonymousGroup).unwrap_or_else(|| {
    panic!(
      "vfail: borrow with no group and no parameter context — a deferred case; see \
       docs/plans/group-generic-closures-plan.md"
    )
  })
}

/// Install `group` as the outermost borrow's group of a value type — used when a body place expression
/// derives its referent group from the place path rather than from a written `in g`. A non-borrow
/// result is returned unchanged (nothing to annotate).
pub(crate) fn with_outer_group<'s, 't>(kind: KindGT<'s, 't>, group: GroupExprG<'s, 't>) -> KindGT<'s, 't> {
  match kind {
    KindGT::BorrowRef(b) => KindGT::BorrowRef(BorrowRefGT { group, inner: b.inner }),
    other => other,
  }
}

