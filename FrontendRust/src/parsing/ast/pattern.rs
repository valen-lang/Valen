use super::ast::NameP;
use super::templex::ITemplexPT;
use crate::lexing::RangeL;
/*
package dev.vale.parsing.ast

import dev.vale.lexing.RangeL
import dev.vale._
*/

#[derive(Clone, Debug, PartialEq)]
pub struct AbstractP {
  pub range: RangeL,
}
/*
//sealed trait IVirtualityP
case class AbstractP(range: RangeL)// extends IVirtualityP
//case class OverrideP(range: RangeP, tyype: ITemplexPT) extends IVirtualityP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterP<'a> {
  pub range: RangeL,
  pub virtuality: Option<AbstractP>,
  pub maybe_pre_checked: Option<RangeL>,
  pub self_borrow: Option<RangeL>,
  pub pattern: Option<PatternPP<'a>>,
}
/*
case class ParameterP(
    range: RangeL,
    virtuality: Option[AbstractP],
    maybePreChecked: Option[RangeL],
    selfBorrow: Option[RangeL],
    pattern: Option[PatternPP]) {

  vassert(selfBorrow.nonEmpty || pattern.nonEmpty)
}
*/

#[derive(Clone, Debug, PartialEq)]
pub struct DestinationLocalP<'a> {
  pub decl: INameDeclarationP<'a>,
  pub mutate: Option<RangeL>,
}
/*
case class DestinationLocalP(decl: INameDeclarationP, mutate: Option[RangeL])
*/

#[derive(Clone, Debug, PartialEq)]
pub struct PatternPP<'a> {
  pub range: RangeL,
  pub destination: Option<DestinationLocalP<'a>>,
  pub templex: Option<ITemplexPT<'a>>,
  pub destructure: Option<DestructureP<'a>>,
}
/*
case class PatternPP(
    range: RangeL,
    destination: Option[DestinationLocalP],

    // If they just have a destructure, this will probably be a ManualSequence(None).
    // If they have just parens, this will probably be a Pack(None).
    // Let's be careful to not allow destructuring packs without Pack here, see MEDP.
    templex: Option[ITemplexPT],

    // Eventually, add an ellipsis: Boolean field here... except we also have
    // to account for the difference between a: T... and a...: T (in one, T is a
    // single type and in the other, T is a pack of types). And we might also want
    // to account for nested parens, like struct Fn:((#Params...), (#Rets...))

    destructure: Option[DestructureP])
*/

#[derive(Clone, Debug, PartialEq)]
pub struct DestructureP<'a> {
  pub range: RangeL,
  pub patterns: Vec<PatternPP<'a>>,
}
/*
case class DestructureP(
  range: RangeL,
  patterns: Vector[PatternPP]) {

  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/

#[derive(Clone, Debug, PartialEq)]
pub enum INameDeclarationP<'a> {
  LocalNameDeclaration(NameP<'a>),
  IgnoredLocalNameDeclaration(RangeL),
  IterableNameDeclaration(RangeL),
  IteratorNameDeclaration(RangeL),
  IterationOptionNameDeclaration(RangeL),
  ConstructingMemberNameDeclaration(NameP<'a>),
}
impl INameDeclarationP<'_> {
  pub fn range(&self) -> RangeL {
    match self {
      INameDeclarationP::LocalNameDeclaration(n) => n.range,
      INameDeclarationP::IgnoredLocalNameDeclaration(r) => *r,
      INameDeclarationP::IterableNameDeclaration(r) => *r,
      INameDeclarationP::IteratorNameDeclaration(r) => *r,
      INameDeclarationP::IterationOptionNameDeclaration(r) => *r,
      INameDeclarationP::ConstructingMemberNameDeclaration(n) => n.range,
    }
  }
}
/*
sealed trait INameDeclarationP {
  def range: RangeL
}
case class LocalNameDeclarationP(name: NameP) extends INameDeclarationP {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious(); override def range: RangeL = name.range
  if (name.str.str == "_") {
    vwat()
  }
}
case class IgnoredLocalNameDeclarationP(range: RangeL) extends INameDeclarationP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious(); }
case class IterableNameDeclarationP(range: RangeL) extends INameDeclarationP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class IteratorNameDeclarationP(range: RangeL) extends INameDeclarationP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class IterationOptionNameDeclarationP(range: RangeL) extends INameDeclarationP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class ConstructingMemberNameDeclarationP(name: NameP) extends INameDeclarationP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious(); override def range: RangeL = name.range }
*/

/*
object Patterns {
  object capturedWithTypeRune {
    def unapply(arg: PatternPP): Option[(String, String)] = {
      arg match {
        case PatternPP(_, Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, name)), None)), Some(NameOrRunePT(NameP(_, kindRune))), None) => Some((name.str, kindRune.str))
        case _ => None
      }
    }
  }
  object withType {
    def unapply(arg: PatternPP): Option[ITemplexPT] = {
      arg.templex
    }
  }
  object capture {
    def unapply(arg: PatternPP): Option[String] = {
      arg match {
        case PatternPP(_, Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, name)), None)), None, None) => Some(name.str)
        case _ => None
      }
    }
  }
  object fromEnv {
    def unapply(arg: PatternPP): Option[String] = {
      arg match {
        case PatternPP(_, None | Some(DestinationLocalP(IgnoredLocalNameDeclarationP(_), None)), Some(NameOrRunePT(NameP(_, kindName))), None) => Some(kindName.str)
        case _ => None
      }
    }
  }
  object withDestructure {
    def unapply(arg: PatternPP): Option[Vector[PatternPP]] = {
      arg.destructure match {
        case None => None
        case Some(DestructureP(_, patterns)) => Some(patterns)
      }
    }
  }
  object capturedWithType {
    def unapply(arg: PatternPP): Option[(String, ITemplexPT)] = {
      arg match {
        case PatternPP(_, Some(DestinationLocalP(LocalNameDeclarationP(NameP(_, name)), None)), Some(templex), None) => Some((name.str, templex))
        case _ => None
      }
    }
  }
}
*/
