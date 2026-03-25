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
pub enum ITemplexPT<'a, 'p> {
  AnonymousRune(AnonymousRunePT),
  Bool(BoolPT),
  Point(PointPT<'a, 'p>),
  Call(CallPT<'a, 'p>),
  Function(FunctionPT<'a, 'p>),
  Inline(InlinePT<'a, 'p>),
  Int(IntPT),
  RegionRune(RegionRunePT<'a>),
  Location(LocationPT),
  Tuple(TuplePT<'a, 'p>),
  Mutability(MutabilityPT),
  NameOrRune(NameOrRunePT<'a>),
  Interpreted(InterpretedPT<'a, 'p>),
  Ownership(OwnershipPT),
  Pack(PackPT<'a, 'p>),
  Func(FuncPT<'a, 'p>),
  StaticSizedArray(StaticSizedArrayPT<'a, 'p>),
  RuntimeSizedArray(RuntimeSizedArrayPT<'a, 'p>),
  Share(SharePT<'a, 'p>),
  String(StringPT),
  TypedRune(TypedRunePT<'a>),
  Variability(VariabilityPT),
}
impl ITemplexPT<'_, '_> {
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
pub struct PointPT<'a, 'p> {
  pub range: RangeL,
  pub inner: &'p ITemplexPT<'a, 'p>,
}
/*
case class PointPT(range: RangeL, inner: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
// This is for example func(Int)Bool, func:imm(Int, Int)Str, func:mut()(Str, Bool)
// It's shorthand for IFunction:(mut, (Int), Bool), IFunction:(mut, (Int, Int), Str), IFunction:(mut, (), (Str, Bool))
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct CallPT<'a, 'p> {
  pub range: RangeL,
  pub template: &'p ITemplexPT<'a, 'p>,
  pub args: &'p [ITemplexPT<'a, 'p>],
}
/*
case class CallPT(range: RangeL, template: ITemplexPT, args: Vector[ITemplexPT]) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionPT<'a, 'p> {
  pub range: RangeL,
  pub mutability: Option<&'p ITemplexPT<'a, 'p>>,
  pub parameters: &'p PackPT<'a, 'p>,
  pub return_type: &'p ITemplexPT<'a, 'p>,
}
/*
// Mutability is Optional because they can leave it out, and mut will be assumed.
case class FunctionPT(range: RangeL, mutability: Option[ITemplexPT], parameters: PackPT, returnType: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct InlinePT<'a, 'p> {
  pub range: RangeL,
  pub inner: &'p ITemplexPT<'a, 'p>,
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
pub struct TuplePT<'a, 'p> {
  pub range: RangeL,
  pub elements: &'p [ITemplexPT<'a, 'p>],
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
pub struct NameOrRunePT<'a>(pub NameP<'a>);
/*
case class NameOrRunePT(name: NameP) extends ITemplexPT {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  def range = name.range
  vassert(name.str != "_")
Guardian: disable: NECX
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct InterpretedPT<'a, 'p> {
  pub range: RangeL,
  pub maybe_ownership: Option<&'p OwnershipPT>,
  pub maybe_region: Option<&'p RegionRunePT<'a>>,
  pub inner: &'p ITemplexPT<'a, 'p>,
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
pub struct FuncPT<'a, 'p> {
  pub range: RangeL,
  pub name: NameP<'a>,
  pub params_range: RangeL,
  pub parameters: &'p [ITemplexPT<'a, 'p>],
  pub return_type: &'p ITemplexPT<'a, 'p>,
}
/*
case class FuncPT(range: RangeL, name: NameP, paramsRange: RangeL, parameters: Vector[ITemplexPT], returnType: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct StaticSizedArrayPT<'a, 'p> {
  pub range: RangeL,
  pub mutability: &'p ITemplexPT<'a, 'p>,
  pub variability: &'p ITemplexPT<'a, 'p>,
  pub size: &'p ITemplexPT<'a, 'p>,
  pub element: &'p ITemplexPT<'a, 'p>,
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
pub struct RuntimeSizedArrayPT<'a, 'p> {
  pub range: RangeL,
  pub mutability: &'p ITemplexPT<'a, 'p>,
  pub element: &'p ITemplexPT<'a, 'p>,
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
pub struct SharePT<'a, 'p> {
  pub range: RangeL,
  pub inner: &'p ITemplexPT<'a, 'p>,
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
pub struct TypedRunePT<'a> {
  pub range: RangeL,
  pub rune: NameP<'a>,
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
pub struct RegionRunePT<'a> {
  pub range: RangeL,
  pub name: Option<NameP<'a>>,
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
pub struct PackPT<'a, 'p> {
  pub range: RangeL,
  pub members: &'p [ITemplexPT<'a, 'p>],
}
/*
case class PackPT(range: RangeL, members: Vector[ITemplexPT]) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/
