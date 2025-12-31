use crate::lexing::RangeL;
use super::ast::{NameP, OwnershipP, LocationP, MutabilityP, VariabilityP};
use super::rules::ITypePR;
/*
package dev.vale.parsing.ast

import dev.vale.lexing.RangeL
import dev.vale.{StrI, vassert, vcurious, vpass}

// See PVSBUFI
*/

#[derive(Clone, Debug, PartialEq)]
pub enum ITemplexPT {
    AnonymousRune(RangeL),
    /*
    case class AnonymousRunePT(range: RangeL) extends ITemplexPT {
      override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
      vpass()
    }
      */
    Bool { range: RangeL, value: bool },
    /*
    case class BoolPT(range: RangeL, value: Boolean) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    //case class BorrowPT(range: RangeL, inner: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    Point { range: RangeL, inner: Box<ITemplexPT> },
    /*
    case class PointPT(range: RangeL, inner: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    // This is for example func(Int)Bool, func:imm(Int, Int)Str, func:mut()(Str, Bool)
    // It's shorthand for IFunction:(mut, (Int), Bool), IFunction:(mut, (Int, Int), Str), IFunction:(mut, (), (Str, Bool))
    */
    Call { range: RangeL, template: Box<ITemplexPT>, args: Vec<ITemplexPT> },
    /*
    case class CallPT(range: RangeL, template: ITemplexPT, args: Vector[ITemplexPT]) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    Function { range: RangeL, mutability: Option<Box<ITemplexPT>>, parameters: Box<PackPT>, return_type: Box<ITemplexPT> },
    /*
    // Mutability is Optional because they can leave it out, and mut will be assumed.
    case class FunctionPT(range: RangeL, mutability: Option[ITemplexPT], parameters: PackPT, returnType: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    Inline { range: RangeL, inner: Box<ITemplexPT> },
    /*
    case class InlinePT(range: RangeL, inner: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    Int { range: RangeL, value: i64 },
    /*
    case class IntPT(range: RangeL, value: Long) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    RegionRune(RegionRunePT),
    Location { range: RangeL, location: LocationP },
    /*
    case class LocationPT(range: RangeL, location: LocationP) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    Tuple { range: RangeL, elements: Vec<ITemplexPT> },
    /*
    case class TuplePT(range: RangeL, elements: Vector[ITemplexPT]) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    Mutability { range: RangeL, mutability: MutabilityP },
    /*
    case class MutabilityPT(range: RangeL, mutability: MutabilityP) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    NameOrRune(NameP),
    /*
    case class NameOrRunePT(name: NameP) extends ITemplexPT {
      override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
      def range = name.range
      vassert(name.str.str != "_")
    }
      */
    Interpreted { range: RangeL, maybe_ownership: Option<Box<OwnershipPT>>, maybe_region: Option<Box<RegionRunePT>>, inner: Box<ITemplexPT> },
    /*
    //case class NullablePT(range: Range, inner: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    case class InterpretedPT(range: RangeL, maybeOwnership: Option[OwnershipPT], maybeRegion: Option[RegionRunePT], inner: ITemplexPT) extends ITemplexPT {
      override def equals(obj: Any): Boolean = vcurious();
      override def hashCode(): Int = vcurious()
    
      vassert(maybeOwnership.nonEmpty || maybeRegion.nonEmpty)
    }
    */
    Ownership(OwnershipPT),
    Pack(PackPT),
    Func { range: RangeL, name: NameP, params_range: RangeL, parameters: Vec<ITemplexPT>, return_type: Box<ITemplexPT> },
    /*
    case class FuncPT(range: RangeL, name: NameP, paramsRange: RangeL, parameters: Vector[ITemplexPT], returnType: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    StaticSizedArray { range: RangeL, mutability: Box<ITemplexPT>, variability: Box<ITemplexPT>, size: Box<ITemplexPT>, element: Box<ITemplexPT> },
    /*
    case class StaticSizedArrayPT(
      range: RangeL,
      mutability: ITemplexPT,
      variability: ITemplexPT,
      size: ITemplexPT,
      element: ITemplexPT
    ) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    RuntimeSizedArray { range: RangeL, mutability: Box<ITemplexPT>, element: Box<ITemplexPT> },
    /*
    case class RuntimeSizedArrayPT(
      range: RangeL,
      mutability: ITemplexPT,
      element: ITemplexPT
    ) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    Share { range: RangeL, inner: Box<ITemplexPT> },
    /*
    case class SharePT(range: RangeL, inner: ITemplexPT) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    String { range: RangeL, str: String },
    /*
    case class StringPT(range: RangeL, str: String) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    TypedRune { range: RangeL, rune: NameP, tyype: ITypePR },
    /*
    case class TypedRunePT(range: RangeL, rune: NameP, tyype: ITypePR) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    Variability { range: RangeL, variability: VariabilityP },
    /*
    case class VariabilityPT(range: RangeL, variability: VariabilityP) extends ITemplexPT { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
}
impl ITemplexPT {
    pub fn range(&self) -> RangeL {
        match self {
            ITemplexPT::AnonymousRune(r) => *r,
            ITemplexPT::Bool { range, .. } => *range,
            ITemplexPT::Point { range, .. } => *range,
            ITemplexPT::Call { range, .. } => *range,
            ITemplexPT::Function { range, .. } => *range,
            ITemplexPT::Inline { range, .. } => *range,
            ITemplexPT::Int { range, .. } => *range,
            ITemplexPT::RegionRune(r) => r.range,
            ITemplexPT::Location { range, .. } => *range,
            ITemplexPT::Tuple { range, .. } => *range,
            ITemplexPT::Mutability { range, .. } => *range,
            ITemplexPT::NameOrRune(n) => n.range,
            ITemplexPT::Interpreted { range, .. } => *range,
            ITemplexPT::Ownership(r) => r.range,
            ITemplexPT::Pack(p) => p.range,
            ITemplexPT::Func { range, .. } => *range,
            ITemplexPT::StaticSizedArray { range, .. } => *range,
            ITemplexPT::RuntimeSizedArray { range, .. } => *range,
            ITemplexPT::Share { range, .. } => *range,
            ITemplexPT::String { range, .. } => *range,
            ITemplexPT::TypedRune { range, .. } => *range,
            ITemplexPT::Variability { range, .. } => *range,
        }
    }
}
/*
sealed trait ITemplexPT {
  def range: RangeL
}
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
