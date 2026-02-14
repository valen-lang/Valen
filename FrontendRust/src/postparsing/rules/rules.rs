/*
package dev.vale.postparsing.rules

import dev.vale.parsing.ast.{LocationP, MutabilityP, OwnershipP, VariabilityP}
import dev.vale.postparsing._
import dev.vale.{RangeS, StrI, vassert, vassertSome, vcurious, vpass}
import dev.vale.parsing.ast._
import dev.vale.postparsing._

import scala.collection.immutable.List
*/
use crate::postparsing::names::IRuneS;
use crate::postparsing::names::IImpreciseNameS;
use crate::postparsing::itemplatatype::{
  BooleanTemplataType, ITemplataType, IntegerTemplataType, LocationTemplataType,
  MutabilityTemplataType, OwnershipTemplataType, StringTemplataType, VariabilityTemplataType,
};
use crate::parsing::ast::{LocationP, MutabilityP, OwnershipP, VariabilityP};
use crate::utils::range::RangeS;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuneUsage<'a> {
  pub range: RangeS<'a>,
  pub rune: IRuneS<'a>,
}

/*
case class RuneUsage(range: RangeS, rune: IRuneS) {
  vpass()
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlaceholderRuleSR<'a> {
  pub range: RangeS<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum IRulexSR<'a> {
  Placeholder(PlaceholderRuleSR<'a>),
  Literal(LiteralSR<'a>),
  MaybeCoercingLookup(MaybeCoercingLookupSR<'a>),
}

impl IRulexSR<'_> {
  pub fn range(&self) -> &RangeS<'_> {
    match self {
      IRulexSR::Placeholder(x) => &x.range,
      IRulexSR::Literal(x) => &x.range,
      IRulexSR::MaybeCoercingLookup(x) => &x.range,
    }
  }

  pub fn rune_usages(&self) -> Vec<RuneUsage<'_>> {
    match self {
      IRulexSR::Placeholder(_) => vec![],
      IRulexSR::Literal(x) => vec![x.rune.clone()],
      IRulexSR::MaybeCoercingLookup(x) => vec![x.rune.clone()],
    }
  }
}

/*
// This isn't generic over e.g.  because we shouldnt reuse
// this between layers. The generics solver doesn't even know about IRulexSR, doesn't
// need to, it relies on delegates to do any rule-specific things.
// Different stages will likely need different kinds of rules, so best not prematurely
// combine them.
trait IRulexSR {
  def range: RangeS
  def runeUsages: Vector[RuneUsage]
}
*/
/*
case class EqualsSR(range: RangeS, left: RuneUsage, right: RuneUsage) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(left, right)
}
*/
/*
// See SAIRFU and SRCAMP for what's going on with these rules.
case class CoordSendSR(
  range: RangeS,
  senderRune: RuneUsage,
  receiverRune: RuneUsage
) extends IRulexSR {
  vpass()

  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(senderRune, receiverRune)
}
*/
/*
case class DefinitionCoordIsaSR(range: RangeS, resultRune: RuneUsage, subRune: RuneUsage, superRune: RuneUsage) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(resultRune, subRune, superRune)
}
*/
/*
case class CallSiteCoordIsaSR(
  range: RangeS,
  // This is here because when we add this CallSiteCoordIsaSR and its companion DefinitionCoordIsaSR,
  // the DefinitionCoordIsaSR has a resultRune that it usually populates with an ImplTemplata.
  // That rune is in the rules somewhere, but when we filter out the DefinitionCoordIsaSR for call site
  // solves, that rune is still there, and all runes must be solved, so we need something to solve it.
  // So, we make CallSiteCoordIsaSR solve it, and populate it with an ImplTemplata or ImplDefinitionTemplata.
  // It's also similar to how Definition/CallSiteFuncSR work.
  // It also means the call site has access to the impls, which might be nice for ONBIFS and NBIFP.
  // It's an Option because CoordSendSR sometimes produces one of these, and it doesn't care about
  // the result.
  resultRune: Option[RuneUsage],
  subRune: RuneUsage,
  superRune: RuneUsage
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = resultRune.toVector ++ Vector(subRune, superRune)
}
*/
/*
case class KindComponentsSR(
  range: RangeS,
  kindRune: RuneUsage,
  mutabilityRune: RuneUsage
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(kindRune, mutabilityRune)
}
*/
/*
case class CoordComponentsSR(
  range: RangeS,
  resultRune: RuneUsage,
  ownershipRune: RuneUsage,
  kindRune: RuneUsage
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(resultRune, ownershipRune, kindRune)
}
*/
/*
case class PrototypeComponentsSR(
  range: RangeS,
  resultRune: RuneUsage,
  paramsRune: RuneUsage,
  returnRune: RuneUsage
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(resultRune, paramsRune, returnRune)
}
*/
/*
case class ResolveSR(
  range: RangeS,
  resultRune: RuneUsage,
  name: StrI,
  paramsListRune: RuneUsage,
  returnRune: RuneUsage
) extends IRulexSR {
  vpass()
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(resultRune, paramsListRune, returnRune)
}
*/
/*
case class CallSiteFuncSR(
  range: RangeS,
  prototypeRune: RuneUsage,
  name: StrI,
  paramsListRune: RuneUsage,
  returnRune: RuneUsage
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(prototypeRune, paramsListRune, returnRune)
}
*/
/*
case class DefinitionFuncSR(
  range: RangeS,
  resultRune: RuneUsage,
  name: StrI,
  paramsListRune: RuneUsage,
  returnRune: RuneUsage
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(resultRune, paramsListRune, returnRune)
}
*/
/*
// See Possible Values Shouldnt Be Used For Inference (PVSBUFI)
case class OneOfSR(
  range: RangeS,
  rune: RuneUsage,
  literals: Vector[ILiteralSL]
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vassert(literals.nonEmpty)
  override def runeUsages: Vector[RuneUsage] = Vector(rune)
}
*/
/*
case class IsConcreteSR(
  range: RangeS,
  rune: RuneUsage
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(rune)
}
*/
/*
case class IsInterfaceSR(
  range: RangeS,
  rune: RuneUsage
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(rune)
}
*/
/*
case class IsStructSR(
  range: RangeS,
  rune: RuneUsage
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(rune)
}
*/
/*
// TODO: Get rid of this in favor of just CoordComponentsSR.
case class CoerceToCoordSR(
  range: RangeS,
  coordRune: RuneUsage,
  kindRune: RuneUsage
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(coordRune, kindRune)
}
*/
/*
case class RefListCompoundMutabilitySR(
  range: RangeS,
  resultRune: RuneUsage,
  coordListRune: RuneUsage,
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(resultRune, coordListRune)
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct LiteralSR<'a> {
  pub range: RangeS<'a>,
  pub rune: RuneUsage<'a>,
  pub literal: ILiteralSL,
}

/*
case class LiteralSR(
  range: RangeS,
  rune: RuneUsage,
  literal: ILiteralSL
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(rune)
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct MaybeCoercingLookupSR<'a> {
  pub range: RangeS<'a>,
  pub rune: RuneUsage<'a>,
  pub name: IImpreciseNameS<'a>,
}

/*
case class MaybeCoercingLookupSR(
  range: RangeS,
  rune: RuneUsage,
  name: IImpreciseNameS
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vpass()
  override def runeUsages: Vector[RuneUsage] = Vector(rune)
}
*/
/*
// A rule that looks up something that's not a Kind, so it doesn't need a default region.
case class LookupSR(
  range: RangeS,
  rune: RuneUsage,
  name: IImpreciseNameS
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vpass()
  override def runeUsages: Vector[RuneUsage] = Vector(rune)
}
*/
/*
case class MaybeCoercingCallSR(
  range: RangeS,
  resultRune: RuneUsage,
  templateRune: RuneUsage,
  args: Vector[RuneUsage]
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(resultRune, templateRune) ++ args
}
*/
/*
case class CallSR(
  range: RangeS,
  resultRune: RuneUsage,
  templateRune: RuneUsage,
  args: Vector[RuneUsage]
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(resultRune, templateRune) ++ args
}
*/
/*
case class IndexListSR(
  range: RangeS,
  resultRune: RuneUsage,
  listRune: RuneUsage,
  index: Int
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vpass()
  override def runeUsages: Vector[RuneUsage] = Vector(resultRune, listRune)
}
*/
/*
case class RuneParentEnvLookupSR(
  range: RangeS,
  rune: RuneUsage
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vpass()
  override def runeUsages: Vector[RuneUsage] = Vector(rune)
}
*/
/*
// InterpretedAR will overwrite inner's permission and ownership to the given ones.
// We turned InterpretedAR into this
case class AugmentSR(
  range: RangeS,
  resultRune: RuneUsage,
  ownership: Option[OwnershipP],
  innerRune: RuneUsage
) extends IRulexSR {
  vpass()
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(resultRune, innerRune)
}
*/
/*
case class PackSR(
  range: RangeS,
  resultRune: RuneUsage,
  members: Vector[RuneUsage]
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(resultRune) ++ members
}
*/
/*
//case class StaticSizedArraySR(
//  range: RangeS,
//  resultRune: RuneUsage,
//  mutabilityRune: RuneUsage,
//  variabilityRune: RuneUsage,
//  sizeRune: RuneUsage,
//  elementRune: RuneUsage
//) extends IRulexSR {
//  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
//  override def runeUsages: Vector[RuneUsage] = Vector(resultRune, mutabilityRune, variabilityRune, sizeRune, elementRune)
//}
//
//case class RuntimeSizedArraySR(
//  range: RangeS,
//  resultRune: RuneUsage,
//  mutabilityRune: RuneUsage,
//  elementRune: RuneUsage
//) extends IRulexSR {
//  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
//  override def runeUsages: Vector[RuneUsage] = Vector(resultRune, mutabilityRune, elementRune)
//}
*/
#[derive(Clone, Debug, PartialEq)]
pub enum ILiteralSL {
  IntLiteral(IntLiteralSL),
  StringLiteral(StringLiteralSL),
  BoolLiteral(BoolLiteralSL),
  MutabilityLiteral(MutabilityLiteralSL),
  LocationLiteral(LocationLiteralSL),
  OwnershipLiteral(OwnershipLiteralSL),
  VariabilityLiteral(VariabilityLiteralSL),
}

impl ILiteralSL {
  pub fn get_type(&self) -> ITemplataType {
    match self {
      ILiteralSL::IntLiteral(x) => x.get_type(),
      ILiteralSL::StringLiteral(x) => x.get_type(),
      ILiteralSL::BoolLiteral(x) => x.get_type(),
      ILiteralSL::MutabilityLiteral(x) => x.get_type(),
      ILiteralSL::LocationLiteral(x) => x.get_type(),
      ILiteralSL::OwnershipLiteral(x) => x.get_type(),
      ILiteralSL::VariabilityLiteral(x) => x.get_type(),
    }
  }
}

/*
sealed trait ILiteralSL {
  def getType(): ITemplataType
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct IntLiteralSL {
  pub value: i64,
}

impl IntLiteralSL {
  pub fn get_type(&self) -> ITemplataType {
    ITemplataType::IntegerTemplataType(IntegerTemplataType {})
  }
}

/*
case class IntLiteralSL(value: Long) extends ILiteralSL {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def getType(): ITemplataType = IntegerTemplataType()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct StringLiteralSL {
  pub value: String,
}

impl StringLiteralSL {
  pub fn get_type(&self) -> ITemplataType {
    ITemplataType::StringTemplataType(StringTemplataType {})
  }
}

/*
case class StringLiteralSL(value: String) extends ILiteralSL {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def getType(): ITemplataType = StringTemplataType()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct BoolLiteralSL {
  pub value: bool,
}

impl BoolLiteralSL {
  pub fn get_type(&self) -> ITemplataType {
    ITemplataType::BooleanTemplataType(BooleanTemplataType {})
  }
}

/*
case class BoolLiteralSL(value: Boolean) extends ILiteralSL {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def getType(): ITemplataType = BooleanTemplataType()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct MutabilityLiteralSL {
  pub mutability: MutabilityP,
}

impl MutabilityLiteralSL {
  pub fn get_type(&self) -> ITemplataType {
    ITemplataType::MutabilityTemplataType(MutabilityTemplataType {})
  }
}

/*
case class MutabilityLiteralSL(mutability: MutabilityP) extends ILiteralSL {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def getType(): ITemplataType = MutabilityTemplataType()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct LocationLiteralSL {
  pub location: LocationP,
}

impl LocationLiteralSL {
  pub fn get_type(&self) -> ITemplataType {
    ITemplataType::LocationTemplataType(LocationTemplataType {})
  }
}

/*
case class LocationLiteralSL(location: LocationP) extends ILiteralSL {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def getType(): ITemplataType = LocationTemplataType()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct OwnershipLiteralSL {
  pub ownership: OwnershipP,
}

impl OwnershipLiteralSL {
  pub fn get_type(&self) -> ITemplataType {
    ITemplataType::OwnershipTemplataType(OwnershipTemplataType {})
  }
}

/*
case class OwnershipLiteralSL(ownership: OwnershipP) extends ILiteralSL {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def getType(): ITemplataType = OwnershipTemplataType()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct VariabilityLiteralSL {
  pub variability: VariabilityP,
}

impl VariabilityLiteralSL {
  pub fn get_type(&self) -> ITemplataType {
    ITemplataType::VariabilityTemplataType(VariabilityTemplataType {})
  }
}

/*
case class VariabilityLiteralSL(variability: VariabilityP) extends ILiteralSL {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def getType(): ITemplataType = VariabilityTemplataType()
}
*/