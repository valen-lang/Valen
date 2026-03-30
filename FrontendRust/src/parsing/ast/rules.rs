use super::ast::NameP;
use super::templex::ITemplexPT;
use crate::lexing::RangeL;
/*
package dev.vale.parsing.ast

import dev.vale.lexing.RangeL
import dev.vale.vcurious
*/

#[derive(Debug, PartialEq)]
pub enum IRulexPR<'p> {
  Equals(EqualsPR<'p>),
  Or(OrPR<'p>),
  Dot(DotPR<'p>),
  Components(ComponentsPR<'p>),
  Typed(TypedPR<'p>),
  Templex(ITemplexPT<'p>),
  BuiltinCall(BuiltinCallPR<'p>),
  Pack(PackPR<'p>),
}
/*
sealed trait IRulexPR {
  def range: RangeL
}
*/

#[derive(Debug, PartialEq)]
pub struct EqualsPR<'p> {
  pub range: RangeL,
  pub left: &'p IRulexPR<'p>,
  pub right: &'p IRulexPR<'p>,
}
/*
case class EqualsPR(range: RangeL, left: IRulexPR, right: IRulexPR) extends IRulexPR { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Debug, PartialEq)]
pub struct OrPR<'p> {
  pub range: RangeL,
  pub possibilities: &'p [IRulexPR<'p>],
}
/*
case class OrPR(range: RangeL, possibilities: Vector[IRulexPR]) extends IRulexPR { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Debug, PartialEq)]
pub struct DotPR<'p> {
  pub range: RangeL,
  pub container: &'p IRulexPR<'p>,
  pub member_name: NameP<'p>,
}
/*
case class DotPR(range: RangeL, container: IRulexPR, memberName: NameP) extends IRulexPR { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Debug, PartialEq)]
pub struct ComponentsPR<'p> {
  pub range: RangeL,
  pub container: ITypePR,
  pub components: &'p [IRulexPR<'p>],
}
/*
case class ComponentsPR(
  range: RangeL,
  container: ITypePR,
  components: Vector[IRulexPR]
) extends IRulexPR { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Debug, PartialEq)]
pub struct TypedPR<'p> {
  pub range: RangeL,
  pub rune: Option<NameP<'p>>,
  pub tyype: ITypePR,
}
/*
case class TypedPR(range: RangeL, rune: Option[NameP], tyype: ITypePR) extends IRulexPR { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class TemplexPR(templex: ITemplexPT) extends IRulexPR {
  def range = templex.range
}
// This is for built-in parser functions, such as exists() or isBaseOf() etc.
*/

#[derive(Debug, PartialEq)]
pub struct BuiltinCallPR<'p> {
  pub range: RangeL,
  pub name: NameP<'p>,
  pub args: &'p [IRulexPR<'p>],
}
/*
case class BuiltinCallPR(range: RangeL, name: NameP, args: Vector[IRulexPR]) extends IRulexPR { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
//case class ResolveSignaturePR(range: RangeL, nameStrRule: IRulexPR, argsPackRule: PackPR) extends IRulexPR { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Debug, PartialEq)]
pub struct PackPR<'p> {
  pub range: RangeL,
  pub elements: &'p [IRulexPR<'p>],
}
/*
case class PackPR(range: RangeL, elements: Vector[IRulexPR]) extends IRulexPR { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

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
*/
pub fn get_ordered_rune_declarations_from_rulexes_with_duplicates<'p>(
  rulexes: &[IRulexPR<'p>],
) -> Vec<NameP<'p>> {
  rulexes
    .iter()
    .flat_map(get_ordered_rune_declarations_from_rulex_with_duplicates)
    .collect()
}
/*
  def getOrderedRuneDeclarationsFromRulexesWithDuplicates(rulexes: Vector[IRulexPR]):
  Vector[NameP] = {
    rulexes.flatMap(getOrderedRuneDeclarationsFromRulexWithDuplicates)
  }
*/
pub fn get_ordered_rune_declarations_from_rulex_with_duplicates<'p>(
  rulex: &IRulexPR<'p>,
) -> Vec<NameP<'p>> {
  match rulex {
    IRulexPR::Pack(pack) => get_ordered_rune_declarations_from_rulexes_with_duplicates(pack.elements),
    IRulexPR::Equals(equals) => {
      let mut out = get_ordered_rune_declarations_from_rulex_with_duplicates(equals.left);
      out.extend(get_ordered_rune_declarations_from_rulex_with_duplicates(equals.right));
      out
    }
    IRulexPR::Or(or) => get_ordered_rune_declarations_from_rulexes_with_duplicates(or.possibilities),
    IRulexPR::Dot(dot) => get_ordered_rune_declarations_from_rulex_with_duplicates(dot.container),
    IRulexPR::Components(components) => {
      get_ordered_rune_declarations_from_rulexes_with_duplicates(components.components)
    }
    IRulexPR::Typed(typed) => typed.rune.iter().cloned().collect(),
    IRulexPR::Templex(templex) => get_ordered_rune_declarations_from_templex_with_duplicates(templex),
    IRulexPR::BuiltinCall(builtin_call) => {
      get_ordered_rune_declarations_from_rulexes_with_duplicates(builtin_call.args)
    }
  }
}
/*
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
*/
pub fn get_ordered_rune_declarations_from_templexes_with_duplicates<'p>(
  templexes: &[&'p ITemplexPT<'p>],
) -> Vec<NameP<'p>> {
  templexes
    .iter()
    .flat_map(|t| get_ordered_rune_declarations_from_templex_with_duplicates(t))
    .collect()
}
/*
  def getOrderedRuneDeclarationsFromTemplexesWithDuplicates(templexes: Vector[ITemplexPT]): Vector[NameP] = {
    templexes.flatMap(getOrderedRuneDeclarationsFromTemplexWithDuplicates)
  }
*/
pub fn get_ordered_rune_declarations_from_templex_with_duplicates<'p>(
  templex: &ITemplexPT<'p>,
) -> Vec<NameP<'p>> {
  match templex {
    ITemplexPT::Interpreted(interpreted) => {
      get_ordered_rune_declarations_from_templex_with_duplicates(interpreted.inner)
    }
    ITemplexPT::String(_)
    | ITemplexPT::Int(_)
    | ITemplexPT::Mutability(_)
    | ITemplexPT::Variability(_)
    | ITemplexPT::Location(_)
    | ITemplexPT::Ownership(_)
    | ITemplexPT::Bool(_)
    | ITemplexPT::NameOrRune(_)
    | ITemplexPT::AnonymousRune(_) => {
      Vec::new()
    }
    ITemplexPT::RegionRune(_) => panic!(
      "PARSING_AST_RULES_GET_ORDERED_RUNE_DECLS_REGION_RUNE_NOT_IN_SCALA_MATCH"
    ),
    ITemplexPT::TypedRune(typed_rune) => vec![typed_rune.rune.clone()],
    ITemplexPT::Call(call) => {
      let mut templexes: Vec<&'p ITemplexPT<'p>> = vec![call.template];
      templexes.extend(call.args.iter().copied());
      get_ordered_rune_declarations_from_templexes_with_duplicates(&templexes)
    }
    ITemplexPT::Function(function) => {
      let mutability_refs: Vec<&'p ITemplexPT<'p>> = function
        .mutability
        .iter()
        .copied()
        .collect();
      let mut out =
        get_ordered_rune_declarations_from_templexes_with_duplicates(&mutability_refs);
      out.extend(get_ordered_rune_declarations_from_templexes_with_duplicates(
        function.parameters.members,
      ));
      out.extend(get_ordered_rune_declarations_from_templex_with_duplicates(
        function.return_type,
      ));
      out
    }
    ITemplexPT::Func(func) => {
      let mut templexes: Vec<&'p ITemplexPT<'p>> = func.parameters.to_vec();
      templexes.push(func.return_type);
      get_ordered_rune_declarations_from_templexes_with_duplicates(&templexes)
    }
    ITemplexPT::Pack(pack) => get_ordered_rune_declarations_from_templexes_with_duplicates(pack.members),
    ITemplexPT::StaticSizedArray(static_sized_array) => {
      let templexes: Vec<&'p ITemplexPT<'p>> = vec![
        static_sized_array.mutability,
        static_sized_array.variability,
        static_sized_array.size,
        static_sized_array.element,
      ];
      get_ordered_rune_declarations_from_templexes_with_duplicates(&templexes)
    }
    ITemplexPT::RuntimeSizedArray(runtime_sized_array) => {
      let templexes: Vec<&'p ITemplexPT<'p>> = vec![
        runtime_sized_array.mutability,
        runtime_sized_array.element,
      ];
      get_ordered_rune_declarations_from_templexes_with_duplicates(&templexes)
    }
    ITemplexPT::Tuple(tuple) => get_ordered_rune_declarations_from_templexes_with_duplicates(tuple.elements),
    ITemplexPT::Inline(_) => panic!(
      "PARSING_AST_RULES_GET_ORDERED_RUNE_DECLS_INLINE_NOT_IN_SCALA_MATCH"
    ),
    ITemplexPT::Point(_) => panic!(
      "PARSING_AST_RULES_GET_ORDERED_RUNE_DECLS_POINT_NOT_IN_SCALA_MATCH"
    ),
    ITemplexPT::Share(_) => panic!(
      "PARSING_AST_RULES_GET_ORDERED_RUNE_DECLS_SHARE_NOT_IN_SCALA_MATCH"
    ),
  }
}
/*
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
*/
/*
}
*/