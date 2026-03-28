use super::ast::{LocationP, MutabilityP, NameP, OwnershipP, VariabilityP};
use super::rules::ITypePR;
use crate::lexing::RangeL;
/*
package dev.vale.parsing.ast

import dev.vale.lexing.RangeL
import dev.vale.{StrI, vassert, vcurious, vpass}

// See PVSBUFI
*/

#[derive(Clone, Debug, PartialEq)]
pub enum ITemplexPT<'p> {
  AnonymousRune(AnonymousRunePT),
  Bool(BoolPT),
  Point(PointPT<'p>),
  Call(CallPT<'p>),
  Function(FunctionPT<'p>),
  Inline(InlinePT<'p>),
  Int(IntPT),
  RegionRune(RegionRunePT<'p>),
  Location(LocationPT),
  Tuple(TuplePT<'p>),
  Mutability(MutabilityPT),
  NameOrRune(NameOrRunePT<'p>),
  Interpreted(InterpretedPT<'p>),
  Ownership(OwnershipPT),
  Pack(PackPT<'p>),
  Func(FuncPT<'p>),
  StaticSizedArray(StaticSizedArrayPT<'p>),
  RuntimeSizedArray(RuntimeSizedArrayPT<'p>),
  Share(SharePT<'p>),
  String(StringPT),
  TypedRune(TypedRunePT<'p>),
  Variability(VariabilityPT),
}
impl ITemplexPT<'_> {
  pub fn range(&self) -> RangeL {
    match self {
      ITemplexPT::AnonymousRune(r) => r.range,
      ITemplexPT::Bool(r) => r.range,
      ITemplexPT::Point(r) => r.range,
      ITemplexPT::Call(r) => r.range,
      ITemplexPT::Function(r) => r.range,
      ITemplexPT::Inline(r) => r.range,
      ITemplexPT::Int(r) => r.range,
      ITemplexPT::RegionRune(r) => r.range,
      ITemplexPT::Location(r) => r.range,
      ITemplexPT::Tuple(r) => r.range,
      ITemplexPT::Mutability(r) => r.0,
      ITemplexPT::NameOrRune(n) => n.0.range(),
      ITemplexPT::Interpreted(r) => r.range,
      ITemplexPT::Ownership(r) => r.0,
      ITemplexPT::Pack(p) => p.range,
      ITemplexPT::Func(r) => r.range,
      ITemplexPT::StaticSizedArray(r) => r.range,
      ITemplexPT::RuntimeSizedArray(r) => r.range,
      ITemplexPT::Share(r) => r.range,
      ITemplexPT::String(r) => r.range,
      ITemplexPT::TypedRune(r) => r.range,
      ITemplexPT::Variability(r) => r.0,
    }
  }
}
/*
sealed trait ITemplexPT {
  def range: RangeL
}
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct AnonymousRunePT {
  pub range: RangeL,
}
/*
case class AnonymousRunePT(range: RangeL) extends ITemplexPT {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vpass()
Guardian: disable: NECX
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct BoolPT {
  pub range: RangeL,
  pub value: bool,
}
/*
case class BoolPT(range: RangeL, value: Boolean) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
//case class BorrowPT(range: RangeL, inner: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct PointPT<'p> {
  pub range: RangeL,
  pub inner: &'p ITemplexPT<'p>,
}
/*
case class PointPT(range: RangeL, inner: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
// This is for example func(Int)Bool, func:imm(Int, Int)Str, func:mut()(Str, Bool)
// It's shorthand for IFunction:(mut, (Int), Bool), IFunction:(mut, (Int, Int), Str), IFunction:(mut, (), (Str, Bool))
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct CallPT<'p> {
  pub range: RangeL,
  pub template: &'p ITemplexPT<'p>,
  pub args: &'p [ITemplexPT<'p>],
}
/*
case class CallPT(range: RangeL, template: ITemplexPT, args: Vector[ITemplexPT]) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionPT<'p> {
  pub range: RangeL,
  pub mutability: Option<&'p ITemplexPT<'p>>,
  pub parameters: &'p PackPT<'p>,
  pub return_type: &'p ITemplexPT<'p>,
}
/*
// Mutability is Optional because they can leave it out, and mut will be assumed.
case class FunctionPT(range: RangeL, mutability: Option[ITemplexPT], parameters: PackPT, returnType: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct InlinePT<'p> {
  pub range: RangeL,
  pub inner: &'p ITemplexPT<'p>,
}
/*
case class InlinePT(range: RangeL, inner: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct IntPT {
  pub range: RangeL,
  pub value: i64,
}
/*
case class IntPT(range: RangeL, value: Long) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct LocationPT {
  pub range: RangeL,
  pub location: LocationP,
}
/*
case class LocationPT(range: RangeL, location: LocationP) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct TuplePT<'p> {
  pub range: RangeL,
  pub elements: &'p [ITemplexPT<'p>],
}
/*
case class TuplePT(range: RangeL, elements: Vector[ITemplexPT]) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct MutabilityPT(pub RangeL, pub MutabilityP);
/*
case class MutabilityPT(range: RangeL, mutability: MutabilityP) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct NameOrRunePT<'p>(pub NameP<'p>);
impl<'p> NameOrRunePT<'p> {
  pub fn new(name: NameP<'p>) -> Self {
    assert!(name.as_str() != "_", "vassert: NameOrRunePT name must not be \"_\"");
    Self(name)
  }
}
/*
case class NameOrRunePT(name: NameP) extends ITemplexPT {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  def range = name.range
  vassert(name.str != "_")
Guardian: disable: NECX
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct InterpretedPT<'p> {
  pub range: RangeL,
  pub maybe_ownership: Option<&'p OwnershipPT>,
  pub maybe_region: Option<&'p RegionRunePT<'p>>,
  pub inner: &'p ITemplexPT<'p>,
}
impl<'p> InterpretedPT<'p> {
  pub fn new(range: RangeL, maybe_ownership: Option<&'p OwnershipPT>, maybe_region: Option<&'p RegionRunePT<'p>>, inner: &'p ITemplexPT<'p>) -> Self {
    assert!(maybe_ownership.is_some() || maybe_region.is_some(), "vassert: InterpretedPT must have ownership or region");
    Self { range, maybe_ownership, maybe_region, inner }
  }
}
/*
//case class NullablePT(range: Range, inner: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class InterpretedPT(range: RangeL, maybeOwnership: Option[OwnershipPT], maybeRegion: Option[RegionRunePT], inner: ITemplexPT) extends ITemplexPT {
  override def equals(obj: Any): Boolean = vcurious();
  override def hashCode(): Int = vcurious()

  vassert(maybeOwnership.nonEmpty || maybeRegion.nonEmpty)
Guardian: disable: NECX
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct FuncPT<'p> {
  pub range: RangeL,
  pub name: NameP<'p>,
  pub params_range: RangeL,
  pub parameters: &'p [ITemplexPT<'p>],
  pub return_type: &'p ITemplexPT<'p>,
}
/*
case class FuncPT(range: RangeL, name: NameP, paramsRange: RangeL, parameters: Vector[ITemplexPT], returnType: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct StaticSizedArrayPT<'p> {
  pub range: RangeL,
  pub mutability: &'p ITemplexPT<'p>,
  pub variability: &'p ITemplexPT<'p>,
  pub size: &'p ITemplexPT<'p>,
  pub element: &'p ITemplexPT<'p>,
}
/*
case class StaticSizedArrayPT(
  range: RangeL,
  mutability: ITemplexPT,
  variability: ITemplexPT,
  size: ITemplexPT,
  element: ITemplexPT
) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeSizedArrayPT<'p> {
  pub range: RangeL,
  pub mutability: &'p ITemplexPT<'p>,
  pub element: &'p ITemplexPT<'p>,
}
/*
case class RuntimeSizedArrayPT(
  range: RangeL,
  mutability: ITemplexPT,
  element: ITemplexPT
) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct SharePT<'p> {
  pub range: RangeL,
  pub inner: &'p ITemplexPT<'p>,
}
/*
case class SharePT(range: RangeL, inner: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct StringPT {
  pub range: RangeL,
  pub str: String,
}
/*
case class StringPT(range: RangeL, str: String) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct TypedRunePT<'p> {
  pub range: RangeL,
  pub rune: NameP<'p>,
  pub tyype: ITypePR,
}
/*
case class TypedRunePT(range: RangeL, rune: NameP, tyype: ITypePR) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct VariabilityPT(pub RangeL, pub VariabilityP);
/*
case class VariabilityPT(range: RangeL, variability: VariabilityP) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct RegionRunePT<'p> {
  pub range: RangeL,
  pub name: Option<NameP<'p>>,
}
/*
case class RegionRunePT(range: RangeL, name: Option[NameP]) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct OwnershipPT(pub RangeL, pub OwnershipP);
/*
case class OwnershipPT(range: RangeL, ownership: OwnershipP) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct PackPT<'p> {
  pub range: RangeL,
  pub members: &'p [ITemplexPT<'p>],
}
/*
case class PackPT(range: RangeL, members: Vector[ITemplexPT]) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/
