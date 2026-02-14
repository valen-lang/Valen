use super::ast::NameP;
use super::templex::ITemplexPT;
use crate::lexing::RangeL;
/*
package dev.vale.parsing.ast

import dev.vale.lexing.RangeL
import dev.vale.vcurious
*/

#[derive(Clone, Debug, PartialEq)]
pub enum IRulexPR<'a> {
  Equals(EqualsPR<'a>),
  Or(OrPR<'a>),
  Dot(DotPR<'a>),
  Components(ComponentsPR<'a>),
  Typed(TypedPR<'a>),
  Templex(ITemplexPT<'a>),
  BuiltinCall(BuiltinCallPR<'a>),
  Pack(PackPR<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct EqualsPR<'a> {
  pub range: RangeL,
  pub left: Box<IRulexPR<'a>>,
  pub right: Box<IRulexPR<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrPR<'a> {
  pub range: RangeL,
  pub possibilities: Vec<IRulexPR<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DotPR<'a> {
  pub range: RangeL,
  pub container: Box<IRulexPR<'a>>,
  pub member_name: NameP<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComponentsPR<'a> {
  pub range: RangeL,
  pub container: ITypePR,
  pub components: Vec<IRulexPR<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypedPR<'a> {
  pub range: RangeL,
  pub rune: Option<NameP<'a>>,
  pub tyype: ITypePR,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltinCallPR<'a> {
  pub range: RangeL,
  pub name: NameP<'a>,
  pub args: Vec<IRulexPR<'a>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PackPR<'a> {
  pub range: RangeL,
  pub elements: Vec<IRulexPR<'a>>,
}

impl IRulexPR<'_> {
  pub fn range(&self) -> RangeL {
    match self {
      IRulexPR::Equals(inner) => inner.range,
      IRulexPR::Or(inner) => inner.range,
      IRulexPR::Dot(inner) => inner.range,
      IRulexPR::Components(inner) => inner.range,
      IRulexPR::Typed(inner) => inner.range,
      IRulexPR::Templex(t) => t.range(),
      IRulexPR::BuiltinCall(inner) => inner.range,
      IRulexPR::Pack(inner) => inner.range,
    }
  }
}
/*
sealed trait IRulexPR {
  def range: RangeL
}
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ITypePR {
  IntType,
  BoolType,
  OwnershipType,
  MutabilityType,
  VariabilityType,
  LocationType,
  CoordType,
  CoordListType,
  PrototypeType,
  KindType,
  RegionType,
  CitizenTemplateType,
}
/*
sealed trait ITypePR
case object IntTypePR extends ITypePR
case object BoolTypePR extends ITypePR
case object OwnershipTypePR extends ITypePR
case object MutabilityTypePR extends ITypePR
case object VariabilityTypePR extends ITypePR
case object LocationTypePR extends ITypePR
case object CoordTypePR extends ITypePR
case object CoordListTypePR extends ITypePR
case object PrototypeTypePR extends ITypePR
case object KindTypePR extends ITypePR
case object RegionTypePR extends ITypePR
case object CitizenTemplateTypePR extends ITypePR
*/

/*
object RulePUtils {

  def getOrderedRuneDeclarationsFromRulexesWithDuplicates(rulexes: Vector[IRulexPR]):
  Vector[NameP] = {
    rulexes.flatMap(getOrderedRuneDeclarationsFromRulexWithDuplicates)
  }

  def getOrderedRuneDeclarationsFromRulexWithDuplicates(rulex: IRulexPR): Vector[NameP] = {
    rulex match {
      case PackPR(range, elements) => getOrderedRuneDeclarationsFromRulexesWithDuplicates(elements)
//      case ResolveSignaturePR(range, nameStrRule, argsPackRule) =>getOrderedRuneDeclarationsFromRulexWithDuplicates(nameStrRule) ++ getOrderedRuneDeclarationsFromRulexWithDuplicates(argsPackRule)
      case EqualsPR(range, left, right) => getOrderedRuneDeclarationsFromRulexWithDuplicates(left) ++ getOrderedRuneDeclarationsFromRulexWithDuplicates(right)
      case OrPR(range, possibilities) => getOrderedRuneDeclarationsFromRulexesWithDuplicates(possibilities)
      case DotPR(range, container, memberName) => getOrderedRuneDeclarationsFromRulexWithDuplicates(container)
      case ComponentsPR(_, container, components) => getOrderedRuneDeclarationsFromRulexesWithDuplicates(components)
      case TypedPR(range, maybeRune, tyype) => maybeRune.toVector
      case TemplexPR(templex) => getOrderedRuneDeclarationsFromTemplexWithDuplicates(templex)
      case BuiltinCallPR(range, name, args) => getOrderedRuneDeclarationsFromRulexesWithDuplicates(args)
    }
  }

  def getOrderedRuneDeclarationsFromTemplexesWithDuplicates(templexes: Vector[ITemplexPT]): Vector[NameP] = {
    templexes.flatMap(getOrderedRuneDeclarationsFromTemplexWithDuplicates)
  }

  def getOrderedRuneDeclarationsFromTemplexWithDuplicates(templex: ITemplexPT): Vector[NameP] = {
    templex match {
      case InterpretedPT(_, _, _, inner) => getOrderedRuneDeclarationsFromTemplexWithDuplicates(inner)
      case StringPT(_, value) => Vector.empty
      case IntPT(_, value) => Vector.empty
      case MutabilityPT(_, mutability) => Vector.empty
      case VariabilityPT(_, mutability) => Vector.empty
      case LocationPT(_, location) => Vector.empty
      case OwnershipPT(_, ownership) => Vector.empty
      case BoolPT(_, value) => Vector.empty
      case NameOrRunePT(name) => Vector.empty
      case TypedRunePT(_, name, tyype) => Vector(name)
      case AnonymousRunePT(_) => Vector.empty
      case CallPT(_, template, args) => getOrderedRuneDeclarationsFromTemplexesWithDuplicates((Vector(template) ++ args))
      case FunctionPT(range, mutability, parameters, returnType) => {
        getOrderedRuneDeclarationsFromTemplexesWithDuplicates(mutability.toVector) ++
          getOrderedRuneDeclarationsFromTemplexWithDuplicates(parameters) ++
          getOrderedRuneDeclarationsFromTemplexWithDuplicates(returnType)
      }
      case FuncPT(_, name, paramsRange, parameters, returnType) => getOrderedRuneDeclarationsFromTemplexesWithDuplicates((parameters :+ returnType))
      case PackPT(_, members) => getOrderedRuneDeclarationsFromTemplexesWithDuplicates(members)
      case StaticSizedArrayPT(_, mutability, variability, size, element) => getOrderedRuneDeclarationsFromTemplexesWithDuplicates(Vector(mutability, variability, size, element))
      case RuntimeSizedArrayPT(_, mutability, element) => getOrderedRuneDeclarationsFromTemplexesWithDuplicates(Vector(mutability, element))
      case TuplePT(_, elements) => getOrderedRuneDeclarationsFromTemplexesWithDuplicates(elements)
    }
  }
}
*/
