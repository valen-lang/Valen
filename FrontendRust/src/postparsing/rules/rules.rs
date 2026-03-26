/*
package dev.vale.postparsing.rules

import dev.vale.parsing.ast.{LocationP, MutabilityP, OwnershipP, VariabilityP}
import dev.vale.postparsing._
import dev.vale.{RangeS, StrI, vassert, vassertSome, vcurious, vpass}
import dev.vale.parsing.ast._
import dev.vale.postparsing._

import scala.collection.immutable.List
*/
use crate::interner::StrI;
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
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub enum IRulexSR<'a, 's> {
  Equals(EqualsSR<'a>),
  Literal(LiteralSR<'a>),
  MaybeCoercingLookup(MaybeCoercingLookupSR<'a>),
  Lookup(LookupSR<'a>),
  MaybeCoercingCall(MaybeCoercingCallSR<'a, 's>),
  Call(CallSR<'a, 's>),
  RuneParentEnvLookup(RuneParentEnvLookupSR<'a>),
  Augment(AugmentSR<'a>),
  OneOf(OneOfSR<'a, 's>),
  IsInterface(IsInterfaceSR<'a>),
  CoordComponents(CoordComponentsSR<'a>),
  CoerceToCoord(CoerceToCoordSR<'a>),
  Pack(PackSR<'a, 's>),
  CallSiteFunc(CallSiteFuncSR<'a>),
  DefinitionFunc(DefinitionFuncSR<'a>),
  Resolve(ResolveSR<'a>),
  CoordSend(CoordSendSR<'a>),
  DefinitionCoordIsa(DefinitionCoordIsaSR<'a>),
  CallSiteCoordIsa(CallSiteCoordIsaSR<'a>),
  KindComponents(KindComponentsSR<'a>),
  PrototypeComponents(PrototypeComponentsSR<'a>),
  IsConcrete(IsConcreteSR<'a>),
  IsStruct(IsStructSR<'a>),
  RefListCompoundMutability(RefListCompoundMutabilitySR<'a>),
  IndexList(IndexListSR<'a>),
}
/*
Guardian: disable-all
// This isn't generic over e.g.  because we shouldnt reuse
// this between layers. The generics solver doesn't even know about IRulexSR, doesn't
// need to, it relies on delegates to do any rule-specific things.
// Different stages will likely need different kinds of rules, so best not prematurely
// combine them.
trait IRulexSR {
  def range: RangeS
  def runeUsages: Vector[RuneUsage]
}
Guardian: disable: NECX
*/

impl<'a, 's> IRulexSR<'a, 's> {
  pub fn range<'r>(&'r self) -> &'r RangeS<'a> {
    match self {
      IRulexSR::Equals(x) => &x.range,
      IRulexSR::Literal(x) => &x.range,
      IRulexSR::MaybeCoercingLookup(x) => &x.range,
      IRulexSR::Lookup(x) => &x.range,
      IRulexSR::MaybeCoercingCall(x) => &x.range,
      IRulexSR::Call(x) => &x.range,
      IRulexSR::RuneParentEnvLookup(x) => &x.range,
      IRulexSR::Augment(x) => &x.range,
      IRulexSR::OneOf(x) => &x.range,
      IRulexSR::IsInterface(x) => &x.range,
      IRulexSR::CoordComponents(x) => &x.range,
      IRulexSR::CoerceToCoord(x) => &x.range,
      IRulexSR::Pack(x) => &x.range,
      IRulexSR::CallSiteFunc(x) => &x.range,
      IRulexSR::DefinitionFunc(x) => &x.range,
      IRulexSR::Resolve(x) => &x.range,
      IRulexSR::CoordSend(x) => &x.range,
      IRulexSR::DefinitionCoordIsa(x) => &x.range,
      IRulexSR::CallSiteCoordIsa(x) => &x.range,
      IRulexSR::KindComponents(x) => &x.range,
      IRulexSR::PrototypeComponents(x) => &x.range,
      IRulexSR::IsConcrete(x) => &x.range,
      IRulexSR::IsStruct(x) => &x.range,
      IRulexSR::RefListCompoundMutability(x) => &x.range,
      IRulexSR::IndexList(x) => &x.range,
    }
    /*
    Guardian: disable-all
    */
  }
  /* Guardian: disable-all */

  pub fn rune_usages<'r>(&'r self) -> Vec<RuneUsage<'a>> {
    match self {
      IRulexSR::Equals(x) => vec![x.left.clone(), x.right.clone()],
      IRulexSR::Literal(x) => vec![x.rune.clone()],
      IRulexSR::MaybeCoercingLookup(x) => vec![x.rune.clone()],
      IRulexSR::Lookup(x) => vec![x.rune.clone()],
      IRulexSR::MaybeCoercingCall(x) => {
        let mut usages = vec![x.result_rune.clone(), x.template_rune.clone()];
        usages.extend(x.args.iter().cloned());
        usages
      }
      IRulexSR::Call(x) => {
        let mut usages = vec![x.result_rune.clone(), x.template_rune.clone()];
        usages.extend(x.args.iter().cloned());
        usages
      }
      IRulexSR::RuneParentEnvLookup(x) => vec![x.rune.clone()],
      IRulexSR::Augment(x) => vec![x.result_rune.clone(), x.inner_rune.clone()],
      IRulexSR::OneOf(x) => vec![x.rune.clone()],
      IRulexSR::IsInterface(x) => vec![x.rune.clone()],
      IRulexSR::CoordComponents(x) => {
        vec![x.result_rune.clone(), x.ownership_rune.clone(), x.kind_rune.clone()]
      }
      IRulexSR::CoerceToCoord(x) => vec![x.coord_rune.clone(), x.kind_rune.clone()],
      IRulexSR::Pack(x) => {
        let mut usages = vec![x.result_rune.clone()];
        usages.extend(x.members.iter().cloned());
        usages
      }
      IRulexSR::CallSiteFunc(x) => vec![x.prototype_rune.clone(), x.params_list_rune.clone(), x.return_rune.clone()],
      IRulexSR::DefinitionFunc(x) => vec![x.result_rune.clone(), x.params_list_rune.clone(), x.return_rune.clone()],
      IRulexSR::Resolve(x) => vec![x.result_rune.clone(), x.params_list_rune.clone(), x.return_rune.clone()],
      IRulexSR::CoordSend(x) => vec![x.sender_rune.clone(), x.receiver_rune.clone()],
      IRulexSR::DefinitionCoordIsa(x) => vec![x.result_rune.clone(), x.sub_rune.clone(), x.super_rune.clone()],
      IRulexSR::CallSiteCoordIsa(x) => {
        let mut usages: Vec<RuneUsage<'a>> = x.result_rune.iter().cloned().collect();
        usages.push(x.sub_rune.clone());
        usages.push(x.super_rune.clone());
        usages
      }
      IRulexSR::KindComponents(x) => vec![x.kind_rune.clone(), x.mutability_rune.clone()],
      IRulexSR::PrototypeComponents(x) => vec![x.result_rune.clone(), x.params_rune.clone(), x.return_rune.clone()],
      IRulexSR::IsConcrete(x) => vec![x.rune.clone()],
      IRulexSR::IsStruct(x) => vec![x.rune.clone()],
      IRulexSR::RefListCompoundMutability(x) => vec![x.result_rune.clone(), x.coord_list_rune.clone()],
      IRulexSR::IndexList(x) => vec![x.result_rune.clone(), x.list_rune.clone()],
    }
  }
  /* Guardian: disable-all */
}
/* Guardian: disable-all */

#[derive(Clone, Debug, PartialEq)]
pub struct EqualsSR<'a> {
  pub range: RangeS<'a>,
  pub left: RuneUsage<'a>,
  pub right: RuneUsage<'a>,
}
/*
case class EqualsSR(range: RangeS, left: RuneUsage, right: RuneUsage) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(left, right)
}
Guardian: disable: NECX
*/
// See SAIRFU and SRCAMP for what's going on with these rules.
#[derive(Clone, Debug, PartialEq)]
pub struct CoordSendSR<'a> {
  pub range: RangeS<'a>,
  pub sender_rune: RuneUsage<'a>,
  pub receiver_rune: RuneUsage<'a>,
}
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
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct DefinitionCoordIsaSR<'a> {
  pub range: RangeS<'a>,
  pub result_rune: RuneUsage<'a>,
  pub sub_rune: RuneUsage<'a>,
  pub super_rune: RuneUsage<'a>,
}
/*
case class DefinitionCoordIsaSR(range: RangeS, resultRune: RuneUsage, subRune: RuneUsage, superRune: RuneUsage) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(resultRune, subRune, superRune)
}
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct CallSiteCoordIsaSR<'a> {
  pub range: RangeS<'a>,
  // This is here because when we add this CallSiteCoordIsaSR and its companion DefinitionCoordIsaSR,
  // the DefinitionCoordIsaSR has a resultRune that it usually populates with an ImplTemplata.
  // That rune is in the rules somewhere, but when we filter out the DefinitionCoordIsaSR for call site
  // solves, that rune is still there, and all runes must be solved, so we need something to solve it.
  // So, we make CallSiteCoordIsaSR solve it, and populate it with an ImplTemplata or ImplDefinitionTemplata.
  // It's also similar to how Definition/CallSiteFuncSR work.
  // It also means the call site has access to the impls, which might be nice for ONBIFS and NBIFP.
  // It's an Option because CoordSendSR sometimes produces one of these, and it doesn't care about
  // the result.
  pub result_rune: Option<RuneUsage<'a>>,
  pub sub_rune: RuneUsage<'a>,
  pub super_rune: RuneUsage<'a>,
}
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
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct KindComponentsSR<'a> {
  pub range: RangeS<'a>,
  pub kind_rune: RuneUsage<'a>,
  pub mutability_rune: RuneUsage<'a>,
}
/*
case class KindComponentsSR(
  range: RangeS,
  kindRune: RuneUsage,
  mutabilityRune: RuneUsage
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(kindRune, mutabilityRune)
}
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct CoordComponentsSR<'a> {
  pub range: RangeS<'a>,
  pub result_rune: RuneUsage<'a>,
  pub ownership_rune: RuneUsage<'a>,
  pub kind_rune: RuneUsage<'a>,
}
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
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct PrototypeComponentsSR<'a> {
  pub range: RangeS<'a>,
  pub result_rune: RuneUsage<'a>,
  pub params_rune: RuneUsage<'a>,
  pub return_rune: RuneUsage<'a>,
}
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
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ResolveSR<'a> {
  pub range: RangeS<'a>,
  pub result_rune: RuneUsage<'a>,
  pub name: StrI<'a>,
  pub params_list_rune: RuneUsage<'a>,
  pub return_rune: RuneUsage<'a>,
}
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
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct CallSiteFuncSR<'a> {
  pub range: RangeS<'a>,
  pub prototype_rune: RuneUsage<'a>,
  pub name: StrI<'a>,
  pub params_list_rune: RuneUsage<'a>,
  pub return_rune: RuneUsage<'a>,
}
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
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct DefinitionFuncSR<'a> {
  pub range: RangeS<'a>,
  pub result_rune: RuneUsage<'a>,
  pub name: StrI<'a>,
  pub params_list_rune: RuneUsage<'a>,
  pub return_rune: RuneUsage<'a>,
}
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
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct OneOfSR<'a, 's> {
  pub range: RangeS<'a>,
  pub rune: RuneUsage<'a>,
  pub literals: &'s [ILiteralSL],
}
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
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct IsConcreteSR<'a> {
  pub range: RangeS<'a>,
  pub rune: RuneUsage<'a>,
}
/*
case class IsConcreteSR(
  range: RangeS,
  rune: RuneUsage
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(rune)
}
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct IsInterfaceSR<'a> {
  pub range: RangeS<'a>,
  pub rune: RuneUsage<'a>,
}
/*
case class IsInterfaceSR(
  range: RangeS,
  rune: RuneUsage
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(rune)
}
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct IsStructSR<'a> {
  pub range: RangeS<'a>,
  pub rune: RuneUsage<'a>,
}
/*
case class IsStructSR(
  range: RangeS,
  rune: RuneUsage
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(rune)
}
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct CoerceToCoordSR<'a> {
  pub range: RangeS<'a>,
  pub coord_rune: RuneUsage<'a>,
  pub kind_rune: RuneUsage<'a>,
}
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
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct RefListCompoundMutabilitySR<'a> {
  pub range: RangeS<'a>,
  pub result_rune: RuneUsage<'a>,
  pub coord_list_rune: RuneUsage<'a>,
}
/*
case class RefListCompoundMutabilitySR(
  range: RangeS,
  resultRune: RuneUsage,
  coordListRune: RuneUsage,
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(resultRune, coordListRune)
}
Guardian: disable: NECX
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
Guardian: disable: NECX
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
Guardian: disable: NECX
*/
// A rule that looks up something that's not a Kind, so it doesn't need a default region.
#[derive(Clone, Debug, PartialEq)]
pub struct LookupSR<'a> {
  pub range: RangeS<'a>,
  pub rune: RuneUsage<'a>,
  pub name: IImpreciseNameS<'a>,
}
/*
case class LookupSR(
  range: RangeS,
  rune: RuneUsage,
  name: IImpreciseNameS
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vpass()
  override def runeUsages: Vector[RuneUsage] = Vector(rune)
}
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct MaybeCoercingCallSR<'a, 's> {
  pub range: RangeS<'a>,
  pub result_rune: RuneUsage<'a>,
  pub template_rune: RuneUsage<'a>,
  pub args: &'s [RuneUsage<'a>],
}
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
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct CallSR<'a, 's> {
  pub range: RangeS<'a>,
  pub result_rune: RuneUsage<'a>,
  pub template_rune: RuneUsage<'a>,
  pub args: &'s [RuneUsage<'a>],
}
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
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct IndexListSR<'a> {
  pub range: RangeS<'a>,
  pub result_rune: RuneUsage<'a>,
  pub list_rune: RuneUsage<'a>,
  pub index: i32,
}
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
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct RuneParentEnvLookupSR<'a> {
  pub range: RangeS<'a>,
  pub rune: RuneUsage<'a>,
}
/*
case class RuneParentEnvLookupSR(
  range: RangeS,
  rune: RuneUsage
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  vpass()
  override def runeUsages: Vector[RuneUsage] = Vector(rune)
}
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct AugmentSR<'a> {
  pub range: RangeS<'a>,
  pub result_rune: RuneUsage<'a>,
  pub ownership: Option<OwnershipP>,
  pub inner_rune: RuneUsage<'a>,
}
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
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct PackSR<'a, 's> {
  pub range: RangeS<'a>,
  pub result_rune: RuneUsage<'a>,
  pub members: &'s [RuneUsage<'a>],
}
/*
case class PackSR(
  range: RangeS,
  resultRune: RuneUsage,
  members: Vector[RuneUsage]
) extends IRulexSR {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def runeUsages: Vector[RuneUsage] = Vector(resultRune) ++ members
}
Guardian: disable: NECX
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
  /* Guardian: disable-all */
}

/*
sealed trait ILiteralSL {
  def getType(): ITemplataType
}
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct IntLiteralSL {
  pub value: i64,
}
/*
case class IntLiteralSL(value: Long) extends ILiteralSL {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def getType(): ITemplataType = IntegerTemplataType()
}
Guardian: disable: NECX
*/

impl IntLiteralSL {
  pub fn get_type(&self) -> ITemplataType {
    ITemplataType::IntegerTemplataType(IntegerTemplataType {})
  }
  /* Guardian: disable-all */
}
/* Guardian: disable-all */

#[derive(Clone, Debug, PartialEq)]
pub struct StringLiteralSL {
  pub value: String,
}
/*
case class StringLiteralSL(value: String) extends ILiteralSL {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def getType(): ITemplataType = StringTemplataType()
}
Guardian: disable: NECX
*/

impl StringLiteralSL {
  pub fn get_type(&self) -> ITemplataType {
    ITemplataType::StringTemplataType(StringTemplataType {})
  }
  /* Guardian: disable-all */
}
/* Guardian: disable-all */

#[derive(Clone, Debug, PartialEq)]
pub struct BoolLiteralSL {
  pub value: bool,
}
/*
case class BoolLiteralSL(value: Boolean) extends ILiteralSL {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def getType(): ITemplataType = BooleanTemplataType()
}
Guardian: disable: NECX
*/

impl BoolLiteralSL {
  pub fn get_type(&self) -> ITemplataType {
    ITemplataType::BooleanTemplataType(BooleanTemplataType {})
  }
  /* Guardian: disable-all */
}
/* Guardian: disable-all */

#[derive(Clone, Debug, PartialEq)]
pub struct MutabilityLiteralSL {
  pub mutability: MutabilityP,
}
/*
case class MutabilityLiteralSL(mutability: MutabilityP) extends ILiteralSL {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def getType(): ITemplataType = MutabilityTemplataType()
}
Guardian: disable: NECX
*/

impl MutabilityLiteralSL {
  pub fn get_type(&self) -> ITemplataType {
    ITemplataType::MutabilityTemplataType(MutabilityTemplataType {})
  }
  /* Guardian: disable-all */
}
/* Guardian: disable-all */

#[derive(Clone, Debug, PartialEq)]
pub struct LocationLiteralSL {
  pub location: LocationP,
}
/*
case class LocationLiteralSL(location: LocationP) extends ILiteralSL {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def getType(): ITemplataType = LocationTemplataType()
}
Guardian: disable: NECX
*/

impl LocationLiteralSL {
  pub fn get_type(&self) -> ITemplataType {
    ITemplataType::LocationTemplataType(LocationTemplataType {})
  }
  /* Guardian: disable-all */
}
/* Guardian: disable-all */

#[derive(Clone, Debug, PartialEq)]
pub struct OwnershipLiteralSL {
  pub ownership: OwnershipP,
}
/*
case class OwnershipLiteralSL(ownership: OwnershipP) extends ILiteralSL {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def getType(): ITemplataType = OwnershipTemplataType()
}
Guardian: disable: NECX
*/

impl OwnershipLiteralSL {
  pub fn get_type(&self) -> ITemplataType {
    ITemplataType::OwnershipTemplataType(OwnershipTemplataType {})
  }
  /* Guardian: disable-all */
}
/* Guardian: disable-all */

#[derive(Clone, Debug, PartialEq)]
pub struct VariabilityLiteralSL {
  pub variability: VariabilityP,
}
/*
case class VariabilityLiteralSL(variability: VariabilityP) extends ILiteralSL {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def getType(): ITemplataType = VariabilityTemplataType()
}
Guardian: disable: NECX
*/

impl VariabilityLiteralSL {
  pub fn get_type(&self) -> ITemplataType {
    ITemplataType::VariabilityTemplataType(VariabilityTemplataType {})
  }
  /* Guardian: disable-all */
}
/* Guardian: disable-all */
