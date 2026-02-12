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
pub enum ITemplexPT {
  AnonymousRune(AnonymousRunePT),
  Bool(BoolPT),
  Point(PointPT),
  Call(CallPT),
  Function(FunctionPT),
  Inline(InlinePT),
  Int(IntPT),
  RegionRune(RegionRunePT),
  Location(LocationPT),
  Tuple(TuplePT),
  Mutability(MutabilityPT),
  NameOrRune(NameOrRunePT),
  Interpreted(InterpretedPT),
  Ownership(OwnershipPT),
  Pack(PackPT),
  Func(FuncPT),
  StaticSizedArray(StaticSizedArrayPT),
  RuntimeSizedArray(RuntimeSizedArrayPT),
  Share(SharePT),
  String(StringPT),
  TypedRune(TypedRunePT),
  Variability(VariabilityPT),
}
impl ITemplexPT {
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
      ITemplexPT::Mutability(r) => r.range,
      ITemplexPT::NameOrRune(n) => n.name.range,
      ITemplexPT::Interpreted(r) => r.range,
      ITemplexPT::Ownership(r) => r.range,
      ITemplexPT::Pack(p) => p.range,
      ITemplexPT::Func(r) => r.range,
      ITemplexPT::StaticSizedArray(r) => r.range,
      ITemplexPT::RuntimeSizedArray(r) => r.range,
      ITemplexPT::Share(r) => r.range,
      ITemplexPT::String(r) => r.range,
      ITemplexPT::TypedRune(r) => r.range,
      ITemplexPT::Variability(r) => r.range,
    }
  }
}
/*
sealed trait ITemplexPT {
  def range: RangeL
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct AnonymousRunePT {
  pub range: RangeL,
}
/*
case class AnonymousRunePT(range: RangeL) extends ITemplexPT {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vpass()
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
*/

#[derive(Clone, Debug, PartialEq)]
pub struct PointPT {
  pub range: RangeL,
  pub inner: Box<ITemplexPT>,
}
/*
case class PointPT(range: RangeL, inner: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
// This is for example func(Int)Bool, func:imm(Int, Int)Str, func:mut()(Str, Bool)
// It's shorthand for IFunction:(mut, (Int), Bool), IFunction:(mut, (Int, Int), Str), IFunction:(mut, (), (Str, Bool))
*/

#[derive(Clone, Debug, PartialEq)]
pub struct CallPT {
  pub range: RangeL,
  pub template: Box<ITemplexPT>,
  pub args: Vec<ITemplexPT>,
}
/*
case class CallPT(range: RangeL, template: ITemplexPT, args: Vector[ITemplexPT]) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionPT {
  pub range: RangeL,
  pub mutability: Option<Box<ITemplexPT>>,
  pub parameters: Box<PackPT>,
  pub return_type: Box<ITemplexPT>,
}
/*
// Mutability is Optional because they can leave it out, and mut will be assumed.
case class FunctionPT(range: RangeL, mutability: Option[ITemplexPT], parameters: PackPT, returnType: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct InlinePT {
  pub range: RangeL,
  pub inner: Box<ITemplexPT>,
}
/*
case class InlinePT(range: RangeL, inner: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct IntPT {
  pub range: RangeL,
  pub value: i64,
}
/*
case class IntPT(range: RangeL, value: Long) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct LocationPT {
  pub range: RangeL,
  pub location: LocationP,
}
/*
case class LocationPT(range: RangeL, location: LocationP) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct TuplePT {
  pub range: RangeL,
  pub elements: Vec<ITemplexPT>,
}
/*
case class TuplePT(range: RangeL, elements: Vector[ITemplexPT]) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct MutabilityPT {
  pub range: RangeL,
  pub mutability: MutabilityP,
}
/*
case class MutabilityPT(range: RangeL, mutability: MutabilityP) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct NameOrRunePT {
  pub name: NameP,
}
/*
case class NameOrRunePT(name: NameP) extends ITemplexPT {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  def range = name.range
  vassert(name.str.str != "_")
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct InterpretedPT {
  pub range: RangeL,
  pub maybe_ownership: Option<Box<OwnershipPT>>,
  pub maybe_region: Option<Box<RegionRunePT>>,
  pub inner: Box<ITemplexPT>,
}
/*
//case class NullablePT(range: Range, inner: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class InterpretedPT(range: RangeL, maybeOwnership: Option[OwnershipPT], maybeRegion: Option[RegionRunePT], inner: ITemplexPT) extends ITemplexPT {
  override def equals(obj: Any): Boolean = vcurious();
  override def hashCode(): Int = vcurious()

  vassert(maybeOwnership.nonEmpty || maybeRegion.nonEmpty)
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct FuncPT {
  pub range: RangeL,
  pub name: NameP,
  pub params_range: RangeL,
  pub parameters: Vec<ITemplexPT>,
  pub return_type: Box<ITemplexPT>,
}
/*
case class FuncPT(range: RangeL, name: NameP, paramsRange: RangeL, parameters: Vector[ITemplexPT], returnType: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct StaticSizedArrayPT {
  pub range: RangeL,
  pub mutability: Box<ITemplexPT>,
  pub variability: Box<ITemplexPT>,
  pub size: Box<ITemplexPT>,
  pub element: Box<ITemplexPT>,
}
/*
case class StaticSizedArrayPT(
  range: RangeL,
  mutability: ITemplexPT,
  variability: ITemplexPT,
  size: ITemplexPT,
  element: ITemplexPT
) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeSizedArrayPT {
  pub range: RangeL,
  pub mutability: Box<ITemplexPT>,
  pub element: Box<ITemplexPT>,
}
/*
case class RuntimeSizedArrayPT(
  range: RangeL,
  mutability: ITemplexPT,
  element: ITemplexPT
) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct SharePT {
  pub range: RangeL,
  pub inner: Box<ITemplexPT>,
}
/*
case class SharePT(range: RangeL, inner: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct StringPT {
  pub range: RangeL,
  pub str: String,
}
/*
case class StringPT(range: RangeL, str: String) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct TypedRunePT {
  pub range: RangeL,
  pub rune: NameP,
  pub tyype: ITypePR,
}
/*
case class TypedRunePT(range: RangeL, rune: NameP, tyype: ITypePR) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct VariabilityPT {
  pub range: RangeL,
  pub variability: VariabilityP,
}
/*
case class VariabilityPT(range: RangeL, variability: VariabilityP) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct RegionRunePT {
  pub range: RangeL,
  pub name: Option<NameP>,
}
/*
case class RegionRunePT(range: RangeL, name: Option[NameP]) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct OwnershipPT {
  pub range: RangeL,
  pub ownership: OwnershipP,
}
/*
case class OwnershipPT(range: RangeL, ownership: OwnershipP) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct PackPT {
  pub range: RangeL,
  pub members: Vec<ITemplexPT>,
}
/*
case class PackPT(range: RangeL, members: Vector[ITemplexPT]) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/
