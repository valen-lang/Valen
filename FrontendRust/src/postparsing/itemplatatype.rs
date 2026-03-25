/*
Guardian: disable-all
*/

/*
package dev.vale.postparsing

import dev.vale._
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RegionTemplataType {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CoordTemplataType {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplTemplataType {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct KindTemplataType {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionTemplataType {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IntegerTemplataType {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BooleanTemplataType {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MutabilityTemplataType {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrototypeTemplataType {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StringTemplataType {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocationTemplataType {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OwnershipTemplataType {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VariabilityTemplataType {}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PackTemplataType {
  pub element_type: Box<ITemplataType>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TemplateTemplataType {
  pub param_types: Vec<ITemplataType>,
  pub return_type: Box<ITemplataType>,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum ITemplataType {
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
  PackTemplataType(PackTemplataType),
  TemplateTemplataType(TemplateTemplataType),
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
  val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash;
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

  val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash;
}
Guardian: disable: NECX
*/
