/*
Guardian: disable-all
*/

/*
package dev.vale.postparsing

import dev.vale._
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RegionTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct CoordTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct KindTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IntegerTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BooleanTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct MutabilityTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrototypeTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StringTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocationTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct OwnershipTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariabilityTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PackTemplataType<'s> {
  pub element_type: &'s ITemplataType<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TemplateTemplataType<'s> {
  pub param_types: &'s [ITemplataType<'s>],
  pub return_type: &'s ITemplataType<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ITemplataType<'s> {
  RegionTemplataType(RegionTemplataType),
  CoordTemplataType(CoordTemplataType),
  ImplTemplataType(ImplTemplataType),
  KindTemplataType(KindTemplataType),
  FunctionTemplataType(FunctionTemplataType),
  IntegerTemplataType(IntegerTemplataType),
  BooleanTemplataType(BooleanTemplataType),
  MutabilityTemplataType(MutabilityTemplataType),
  PrototypeTemplataType(PrototypeTemplataType),
  StringTemplataType(StringTemplataType),
  LocationTemplataType(LocationTemplataType),
  OwnershipTemplataType(OwnershipTemplataType),
  VariabilityTemplataType(VariabilityTemplataType),
  PackTemplataType(PackTemplataType<'s>),
  TemplateTemplataType(TemplateTemplataType<'s>),
}

/*
sealed trait ITemplataType
case class RegionTemplataType() extends ITemplataType {
  vpass()
}
case class CoordTemplataType() extends ITemplataType
case class ImplTemplataType() extends ITemplataType
case class KindTemplataType() extends ITemplataType
case class FunctionTemplataType() extends ITemplataType
case class IntegerTemplataType() extends ITemplataType
case class BooleanTemplataType() extends ITemplataType
case class MutabilityTemplataType() extends ITemplataType
case class PrototypeTemplataType() extends ITemplataType
case class StringTemplataType() extends ITemplataType
case class LocationTemplataType() extends ITemplataType
case class OwnershipTemplataType() extends ITemplataType
case class VariabilityTemplataType() extends ITemplataType
case class PackTemplataType(elementType: ITemplataType) extends ITemplataType {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def equals(obj: Any): Boolean = {
    obj match {
      case PackTemplataType(thatElementType) => elementType == thatElementType
      case _ => false
    }
  }
}
// This is CitizenTemplataType() instead of separate ones for struct and interface
// because the RuleTyper doesn't care whether something's a struct or an interface.
case class TemplateTemplataType(
  paramTypes: Vector[ITemplataType],
  returnType: ITemplataType
) extends ITemplataType {
  vassert(!paramTypes.contains(RegionTemplataType()))

  val hash = runtime.ScalaRunTime._hashCode(this)

  override def hashCode(): Int = hash;
}
*/
