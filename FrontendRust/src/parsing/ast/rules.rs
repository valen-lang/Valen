use super::ast::NameP;
use super::templex::ITemplexPT;
use crate::lexing::RangeL;
/*
package dev.vale.parsing.ast

import dev.vale.lexing.RangeL
import dev.vale.vcurious
*/

#[derive(Clone, Debug, PartialEq)]
pub enum IRulexPR<'a, 'p> {
  Equals(EqualsPR<'a, 'p>),
  Or(OrPR<'a, 'p>),
  Dot(DotPR<'a, 'p>),
  Components(ComponentsPR<'a, 'p>),
  Typed(TypedPR<'a>),
  Templex(ITemplexPT<'a, 'p>),
  BuiltinCall(BuiltinCallPR<'a, 'p>),
  Pack(PackPR<'a, 'p>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct EqualsPR<'a, 'p> {
  pub range: RangeL,
  pub left: &'p IRulexPR<'a, 'p>,
  pub right: &'p IRulexPR<'a, 'p>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrPR<'a, 'p> {
  pub range: RangeL,
  pub possibilities: &'p [IRulexPR<'a, 'p>],
}

#[derive(Clone, Debug, PartialEq)]
pub struct DotPR<'a, 'p> {
  pub range: RangeL,
  pub container: &'p IRulexPR<'a, 'p>,
  pub member_name: NameP<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ComponentsPR<'a, 'p> {
  pub range: RangeL,
  pub container: ITypePR,
  pub components: &'p [IRulexPR<'a, 'p>],
}

#[derive(Clone, Debug, PartialEq)]
pub struct TypedPR<'a> {
  pub range: RangeL,
  pub rune: Option<NameP<'a>>,
  pub tyype: ITypePR,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltinCallPR<'a, 'p> {
  pub range: RangeL,
  pub name: NameP<'a>,
  pub args: &'p [IRulexPR<'a, 'p>],
}

#[derive(Clone, Debug, PartialEq)]
pub struct PackPR<'a, 'p> {
  pub range: RangeL,
  pub elements: &'p [IRulexPR<'a, 'p>],
}

impl IRulexPR<'_, '_> {
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
Guardian: disable: NECX
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
Guardian: disable: NECX
*/

/*
object RulePUtils {
*/
pub fn get_ordered_rune_declarations_from_rulexes_with_duplicates<'a, 'p>(
  rulexes: &'p [IRulexPR<'a, 'p>],
) -> Vec<NameP<'a>> {
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
pub fn get_ordered_rune_declarations_from_rulex_with_duplicates<'a, 'p>(
  rulex: &IRulexPR<'a, 'p>,
) -> Vec<NameP<'a>> {
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
pub fn get_ordered_rune_declarations_from_templexes_with_duplicates<'a, 'p>(
  templexes: &'p [ITemplexPT<'a, 'p>],
) -> Vec<NameP<'a>> {
  templexes
    .iter()
    .flat_map(get_ordered_rune_declarations_from_templex_with_duplicates)
    .collect()
}
/*
  def getOrderedRuneDeclarationsFromTemplexesWithDuplicates(templexes: Vector[ITemplexPT]): Vector[NameP] = {
    templexes.flatMap(getOrderedRuneDeclarationsFromTemplexWithDuplicates)
  }
*/
pub fn get_ordered_rune_declarations_from_templex_with_duplicates<'a, 'p>(
  templex: &ITemplexPT<'a, 'p>,
) -> Vec<NameP<'a>> {
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
      let mut templexes = vec![(*call.template).clone()];
      templexes.extend(call.args.iter().cloned());
      get_ordered_rune_declarations_from_templexes_with_duplicates(&templexes)
    }
    ITemplexPT::Function(function) => {
      let mutability_templexes = function
        .mutability
        .iter()
        .map(|x| (*x).clone())
        .collect::<Vec<_>>();
      let mut out =
        get_ordered_rune_declarations_from_templexes_with_duplicates(&mutability_templexes);
      out.extend(get_ordered_rune_declarations_from_templex_with_duplicates(
        &ITemplexPT::Pack(function.parameters.clone()),
      ));
      out.extend(get_ordered_rune_declarations_from_templex_with_duplicates(
        function.return_type,
      ));
      out
    }
    ITemplexPT::Func(func) => {
      let mut templexes = func.parameters.to_vec();
      templexes.push((*func.return_type).clone());
      get_ordered_rune_declarations_from_templexes_with_duplicates(&templexes)
    }
    ITemplexPT::Pack(pack) => get_ordered_rune_declarations_from_templexes_with_duplicates(pack.members),
    ITemplexPT::StaticSizedArray(static_sized_array) => {
      let templexes = vec![
        (*static_sized_array.mutability).clone(),
        (*static_sized_array.variability).clone(),
        (*static_sized_array.size).clone(),
        (*static_sized_array.element).clone(),
      ];
      get_ordered_rune_declarations_from_templexes_with_duplicates(&templexes)
    }
    ITemplexPT::RuntimeSizedArray(runtime_sized_array) => {
      let templexes = vec![
        (*runtime_sized_array.mutability).clone(),
        (*runtime_sized_array.element).clone(),
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