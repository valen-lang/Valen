use crate::interner::StrI;
use crate::postparsing::names::IImpreciseNameS;
use crate::postparsing::names::IRuneS;
use crate::postparsing::names::IVarNameS;
use crate::postparsing::rules::rules::IntLiteralSL;
use crate::postparsing::rules::RuneUsage;
use crate::utils::range::RangeS;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ITypeST<'s> {
  AnonymousRune(&'s AnonymousRuneST<'s>),
  Bool(&'s BoolST<'s>),
  Call(&'s CallST<'s>),
  Function(&'s FunctionST<'s>),
  Int(&'s IntST<'s>),
  Tuple(&'s TupleST<'s>),
  Name(&'s NameST<'s>),
  Rune(&'s RuneUsageST<'s>),
  BorrowRef(&'s BorrowRefST<'s>),
  WeakRef(&'s WeakRefST<'s>),
  OwnRef(&'s OwnRefST<'s>),
  Pack(&'s PackST<'s>),
  // Func(&'s FuncST<'s>),
  RuntimeSizedArray(&'s RuntimeSizedArrayST<'s>),
  String(&'s StringST<'s>),
}
impl<'s> ITypeST<'s> {
  pub fn range(&self) -> RangeS<'s> {
    match self {
      ITypeST::AnonymousRune(r) => r.range,
      ITypeST::Bool(r) => r.range,
      ITypeST::Call(r) => r.range,
      ITypeST::Function(r) => r.range,
      ITypeST::Int(r) => r.range,
      ITypeST::Tuple(r) => r.range,
      ITypeST::Name(n) => n.range,
      ITypeST::Rune(n) => n.rune.range,
      ITypeST::BorrowRef(r) => r.range,
      ITypeST::WeakRef(r) => r.range,
      ITypeST::OwnRef(r) => r.range,
      ITypeST::Pack(p) => p.range,
      ITypeST::RuntimeSizedArray(r) => r.range,
      ITypeST::String(r) => r.range,
    }
  }

  /// Collects every rune this type mentions, e.g. `{T}` for `&T` and `{Lam}` for `&Lam`, recursing
  /// through the ref wraps and applications. An AnonymousRune has no rune identity, so it adds none.
  pub fn collect_rune_mentions(&self, out: &mut Vec<IRuneS<'s>>) {
    match self {
      ITypeST::Rune(r) => out.push(r.rune.rune),
      ITypeST::BorrowRef(r) => r.inner.collect_rune_mentions(out),
      ITypeST::WeakRef(r) => r.inner.collect_rune_mentions(out),
      ITypeST::OwnRef(r) => r.inner.collect_rune_mentions(out),
      ITypeST::RuntimeSizedArray(r) => r.element.collect_rune_mentions(out),
      ITypeST::Call(r) => {
        r.template.collect_rune_mentions(out);
        for arg in r.args.iter().copied() {
          arg.collect_rune_mentions(out);
        }
      }
      ITypeST::Tuple(r) => {
        for e in r.elements.iter().copied() {
          e.collect_rune_mentions(out);
        }
      }
      ITypeST::Pack(r) => {
        for m in r.members.iter().copied() {
          m.collect_rune_mentions(out);
        }
      }
      ITypeST::Function(r) => {
        for m in r.parameters.members.iter().copied() {
          m.collect_rune_mentions(out);
        }
        r.return_type.collect_rune_mentions(out);
      }
      ITypeST::AnonymousRune(_)
      | ITypeST::Name(_)
      | ITypeST::Int(_)
      | ITypeST::Bool(_)
      | ITypeST::String(_) => {}
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AnonymousRuneST<'s> {
  pub range: RangeS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BoolST<'s> {
  pub range: RangeS<'s>,
  pub value: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CallST<'s> {
  pub range: RangeS<'s>,
  pub template: &'s ITypeST<'s>,
  pub args: &'s [&'s ITypeST<'s>],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FunctionST<'s> {
  pub range: RangeS<'s>,
  pub parameters: &'s PackST<'s>,
  pub return_type: &'s ITypeST<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IntST<'s> {
  pub range: RangeS<'s>,
  pub value: IntLiteralSL,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TupleST<'s> {
  pub range: RangeS<'s>,
  pub elements: &'s [&'s ITypeST<'s>],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NameST<'s> {
  pub range: RangeS<'s>,
  pub name: IImpreciseNameS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RuneUsageST<'s> {
  pub rune: RuneUsage<'s>,
}

/// The region of a borrow reference. `held` and an explicit group annotation are sibling values
/// here alongside "no annotation", so a borrow's region lives in one slot.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RegionS<'s> {
  /// No group written: `&Ship`.
  Unspecified,
  /// A held reference: `held Ship`. A borrow into an anonymous group the callee treats as
  /// undestroyable, proven at the call site by the caller.
  Held,
  /// An explicit group annotation: `&Ship in g`.
  Group(&'s GroupS<'s>),
}

/// A group expression on a borrow's `in ...` clause or in an effect clause. Scout-stage: symbolic,
/// with a group param resolved to a `Rune` and a local kept as a `Local` name. A leaf is never an
/// `IdT`; that resolution happens in the borrow checker, as `GroupB`. Extensible: near-term the
/// scout only produces `Rune`; the rest arrive with value-path/union groups.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum GroupS<'s> {
  /// `in g`: a group param.
  Rune(&'s RuneUsage<'s>),
  /// `in x`: a local.
  Local(IVarNameS<'s>),
  /// `in x.items`: the named member.
  Member { base: &'s GroupS<'s>, member_name: StrI<'s> },
  /// `in x.items[]`: an element of the member.
  Elements { base: &'s GroupS<'s> },
  /// `in (a | b)`: a union of groups.
  Union { members: &'s [&'s GroupS<'s>] },
}

/// An effect clause on a function signature: `mut(g)` / `not(mut(g))`. Scout-stage, over a `GroupS`.
/// Lands (borrowed from `'s`) in the per-`FunctionT` side table, never on the durable header.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum EffectS<'s> {
  Mut(&'s GroupS<'s>),
  NotMut(&'s GroupS<'s>),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BorrowRefST<'s> {
  pub range: RangeS<'s>,
  pub inner: &'s ITypeST<'s>,
  pub region: RegionS<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WeakRefST<'s> {
  pub range: RangeS<'s>,
  pub inner: &'s ITypeST<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OwnRefST<'s> {
  pub range: RangeS<'s>,
  pub inner: &'s ITypeST<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PackST<'s> {
  pub range: RangeS<'s>,
  pub members: &'s [&'s ITypeST<'s>],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RuntimeSizedArrayST<'s> {
  pub range: RangeS<'s>,
  pub element: &'s ITypeST<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StringST<'s> {
  pub range: RangeS<'s>,
  pub str: StrI<'s>,
}
