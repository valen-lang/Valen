/*
package dev.vale.highertyping

import dev.vale.{RangeS, StrI, vassert, vcurious, vpass, vwat}
import dev.vale.parsing.ast.MutabilityP
import dev.vale.postparsing.rules._
import dev.vale.postparsing._
import dev.vale.parsing._
import dev.vale.postparsing._

import scala.collection.immutable.List
*/

use std::collections::HashMap;
use crate::interner::StrI;
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::parsing::MutabilityP;
use crate::postparsing::ast::{
    GenericParameterS, ICitizenAttributeS, IFunctionAttributeS,
    IStructMemberS, ParameterS, IBodyS,
};
use crate::postparsing::itemplatatype::{ITemplataType, TemplateTemplataType};
use crate::postparsing::names::{
    INameS, IRuneS, IStructDeclarationNameS, IImplDeclarationNameS,
    IImpreciseNameS, IFunctionDeclarationNameS, TopLevelInterfaceDeclarationNameS,
};
use crate::postparsing::rules::{IRulexSR, RuneUsage};
use crate::utils::range::RangeS;

// mig: struct ProgramA
#[derive(Debug)]
pub struct ProgramA<'a, 's> {
    pub structs: &'s [&'s StructA<'a, 's>],
    pub interfaces: &'s [&'s InterfaceA<'a, 's>],
    pub impls: &'s [&'s ImplA<'a, 's>],
    pub functions: &'s [&'s FunctionA<'a, 's>],
    pub exports: &'s [&'s ExportAsA<'a, 's>],
}
/*
case class ProgramA(
    structs: Vector[StructA],
    interfaces: Vector[InterfaceA],
    impls: Vector[ImplA],
    functions: Vector[FunctionA],
    exports: Vector[ExportAsA]) {
*/
// mig: impl ProgramA
impl<'a, 's> ProgramA<'a, 's> {
/*
*/
// mig: fn equals
pub fn equals(&self, _obj: &dyn std::any::Any) -> bool {
    panic!("Unimplemented: equals");
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
pub fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  override def hashCode(): Int = vcurious()

*/
// mig: fn lookup_function_by_name
pub fn lookup_function_by_name(&self, _name: &INameS<'a>) -> &FunctionA<'a, 's> {
    panic!("Unimplemented: lookup_function_by_name");
}
/*
  def lookupFunction(name: INameS) = {
    val matches = functions.filter(_.name == name)
    vassert(matches.size == 1)
    matches.head
  }
*/
// mig: fn lookup_function_by_str
pub fn lookup_function_by_str(&self, name: &str) -> &'s FunctionA<'a, 's> {
    let matches: Vec<_> = self.functions.iter().filter(|function| {
      match &function.name {
        IFunctionDeclarationNameS::FunctionName(n) => n.name.as_str() == name,
        _ => false,
      }
    }).collect();
    assert!(matches.len() == 1);
    matches[0]
}
/*
  def lookupFunction(name: String) = {
    val matches = functions.filter(function => {
      function.name match {
        case FunctionNameS(n, _) => n.str == name
        case _ => false
      }
    })
    vassert(matches.size == 1)
    matches.head
  }
*/
// mig: fn lookup_interface
pub fn lookup_interface(&self, _name: &INameS<'a>) -> &InterfaceA<'a, 's> {
    panic!("Unimplemented: lookup_interface");
}
/*
  def lookupInterface(name: INameS) = {
    val matches = interfaces.find(_.name == name)
    vassert(matches.size == 1)
    matches.head match {
      case i @ InterfaceA(_, _, _, _, _, _, _, _, _, _, _) => i
    }
  }
*/
// mig: fn lookup_struct_by_name
pub fn lookup_struct_by_name(&self, _name: &INameS<'a>) -> &StructA<'a, 's> {
    panic!("Unimplemented: lookup_struct_by_name");
}
/*
  def lookupStruct(name: INameS) = {
    val matches = structs.find(_.name == name)
    vassert(matches.size == 1)
    matches.head match {
      case i @ StructA(_, _, _, _, _, _, _, _, _, _, _, _, _) => i
    }
  }
*/
// mig: fn lookup_struct_by_str
pub fn lookup_struct_by_str(&self, name: &str) -> &StructA<'a, 's> {
    let matches: Vec<_> = self.structs.iter().filter(|s| {
      match &s.name {
        IStructDeclarationNameS::TopLevelStructDeclarationName(n) => n.name.as_str() == name,
        _ => false,
      }
    }).collect();
    assert!(matches.len() == 1);
    matches[0]
}
}
/*
  def lookupStruct(name: String) = {
    val matches = structs.filter(struct => {
      struct.name match {
        case TopLevelCitizenDeclarationNameS(n, _) => n.str == name
        case _ => false
      }
    })
    vassert(matches.size == 1)
    matches.head
  }
}
}
*/
// mig: struct StructA
#[derive(Debug)]
pub struct StructA<'a, 's> {
    pub range: RangeS<'a>,
    pub name: IStructDeclarationNameS<'a>,
    pub attributes: &'s [ICitizenAttributeS<'a>],
    pub weakable: bool,
    pub mutability_rune: RuneUsage<'a>,
    pub maybe_predicted_mutability: Option<MutabilityP>,
    pub tyype: TemplateTemplataType,
    pub generic_parameters: &'s [&'s GenericParameterS<'a, 's>],
    pub header_rune_to_type: ArenaIndexMap<'s, IRuneS<'a>, ITemplataType>,
    pub header_rules: &'s [IRulexSR<'a>],
    pub members_rune_to_type: ArenaIndexMap<'s, IRuneS<'a>, ITemplataType>,
    pub member_rules: &'s [IRulexSR<'a>],
    pub members: &'s [IStructMemberS<'a>],
}
/*
case class StructA(
  range: RangeS,
  name: IStructDeclarationNameS,
  attributes: Vector[ICitizenAttributeS],
  weakable: Boolean,
  mutabilityRune: RuneUsage,

  // This is needed for recursive structures like
  //   struct ListNode<T> imm where T Ref {
  //     tail ListNode<T>;
  //   }
  maybePredictedMutability: Option[MutabilityP],
  tyype: TemplateTemplataType,
  genericParameters: Vector[GenericParameterS],

  // These are separated so that these alone can be run during resolving, see SMRASDR.
  headerRuneToType: Map[IRuneS, ITemplataType],
  headerRules: Vector[IRulexSR],

  // These are separated so they can be skipped during resolving, see SMRASDR.
  membersRuneToType: Map[IRuneS, ITemplataType],
  memberRules: Vector[IRulexSR],
  members: Vector[IStructMemberS]
) extends CitizenA {
  val hash = range.hashCode() + name.hashCode()
*/
// mig: impl StructA
impl<'a, 's> StructA<'a, 's> {
pub fn new(
    range: RangeS<'a>,
    name: IStructDeclarationNameS<'a>,
    attributes: &'s [ICitizenAttributeS<'a>],
    weakable: bool,
    mutability_rune: RuneUsage<'a>,
    maybe_predicted_mutability: Option<MutabilityP>,
    tyype: TemplateTemplataType,
    generic_parameters: &'s [&'s GenericParameterS<'a, 's>],
    header_rune_to_type: ArenaIndexMap<'s, IRuneS<'a>, ITemplataType>,
    header_rules: &'s [IRulexSR<'a>],
    members_rune_to_type: ArenaIndexMap<'s, IRuneS<'a>, ITemplataType>,
    member_rules: &'s [IRulexSR<'a>],
    members: &'s [IStructMemberS<'a>],
) -> Self {
    // These should be removed by the higher typer
    for rule in header_rules.iter() {
        match rule {
            IRulexSR::MaybeCoercingCall(_) => panic!("vwat: MaybeCoercingCallSR in header rules"),
            IRulexSR::MaybeCoercingLookup(_) => panic!("vwat: MaybeCoercingLookupSR in header rules"),
            _ => {}
        }
    }
    for rule in member_rules.iter() {
        match rule {
            IRulexSR::MaybeCoercingCall(_) => panic!("vwat: MaybeCoercingCallSR in member rules"),
            IRulexSR::MaybeCoercingLookup(_) => panic!("vwat: MaybeCoercingLookupSR in member rules"),
            _ => {}
        }
    }
    assert!(
        !generic_parameters.iter().any(|x| matches!(x.rune.rune, IRuneS::DenizenDefaultRegionRune(_))),
        "vassert: generic_parameters should not contain DenizenDefaultRegionRuneS"
    );
    assert!(
        !header_rune_to_type.keys().any(|rune| matches!(rune, IRuneS::DenizenDefaultRegionRune(_))),
        "vassert: header_rune_to_type should not contain DenizenDefaultRegionRuneS"
    );
    assert!(
        !members_rune_to_type.keys().any(|rune| matches!(rune, IRuneS::DenizenDefaultRegionRune(_))),
        "vassert: members_rune_to_type should not contain DenizenDefaultRegionRuneS"
    );
    Self { range, name, attributes, weakable, mutability_rune, maybe_predicted_mutability, tyype, generic_parameters, header_rune_to_type, header_rules, members_rune_to_type, member_rules, members }
}
/*
*/
// mig: fn hash_code
pub fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  override def hashCode(): Int = hash;

  vpass()

  // These should be removed by the higher typer
  headerRules.collect({
    case MaybeCoercingCallSR(_, _, _, _) => vwat()
    case MaybeCoercingLookupSR(_, _, _) => vwat()
  })
  memberRules.collect({
    case MaybeCoercingCallSR(_, _, _, _) => vwat()
    case MaybeCoercingLookupSR(_, _, _) => vwat()
  })

  vassert(
    !genericParameters.exists({ case x =>
      x.rune.rune match {
        case DenizenDefaultRegionRuneS(_) => true
        case _ => false
      }
    }))
  vassert(
    !headerRuneToType.exists({ case (rune, _) =>
      rune match {
        case DenizenDefaultRegionRuneS(_) => true
        case _ => false
      }
    }))
  vassert(
    !membersRuneToType.exists({ case (rune, _) =>
      rune match {
        case DenizenDefaultRegionRuneS(_) => true
        case _ => false
      }
    }))
*/
// mig: fn equals
pub fn equals(&self, _obj: &dyn std::any::Any) -> bool {
    panic!("Unimplemented: equals");
}
}
/*
  override def equals(obj: Any): Boolean = {
    if (!obj.isInstanceOf[StructA]) { return false }
    val that = obj.asInstanceOf[StructA]
    return range == that.range && name == that.name;
  }

//  vassert((knowableRunes -- runeToType.keySet).isEmpty)
//  vassert((localRunes -- runeToType.keySet).isEmpty)
}
}
*/
// mig: struct ImplA
#[derive(Debug)]
pub struct ImplA<'a, 's> {
    pub range: RangeS<'a>,
    pub name: IImplDeclarationNameS<'a>,
    pub generic_params: &'s [&'s GenericParameterS<'a, 's>],
    pub rules: &'s [IRulexSR<'a>],
    pub rune_to_type: ArenaIndexMap<'s, IRuneS<'a>, ITemplataType>,
    pub sub_citizen_rune: RuneUsage<'a>,
    pub sub_citizen_imprecise_name: IImpreciseNameS<'a>,
    pub interface_kind_rune: RuneUsage<'a>,
    pub super_interface_imprecise_name: IImpreciseNameS<'a>,
}
/*
case class ImplA(
  range: RangeS,
  name: IImplDeclarationNameS,
  genericParams: Vector[GenericParameterS],
  rules: Vector[IRulexSR],
  runeToType: Map[IRuneS, ITemplataType],
  subCitizenRune: RuneUsage,
  subCitizenImpreciseName: IImpreciseNameS,
  interfaceKindRune: RuneUsage,
  superInterfaceImpreciseName: IImpreciseNameS) {

  // These should be removed by the higher typer
  rules.collect({
    case MaybeCoercingCallSR(_, _, _, _) => vwat()
    case MaybeCoercingLookupSR(_, _, _) => vwat()
  })

  val hash = range.hashCode() + name.hashCode()
*/
// mig: impl ImplA
impl<'a, 's> ImplA<'a, 's> {
pub fn new(
    range: RangeS<'a>,
    name: IImplDeclarationNameS<'a>,
    generic_params: &'s [&'s GenericParameterS<'a, 's>],
    rules: &'s [IRulexSR<'a>],
    rune_to_type: ArenaIndexMap<'s, IRuneS<'a>, ITemplataType>,
    sub_citizen_rune: RuneUsage<'a>,
    sub_citizen_imprecise_name: IImpreciseNameS<'a>,
    interface_kind_rune: RuneUsage<'a>,
    super_interface_imprecise_name: IImpreciseNameS<'a>,
) -> Self {
    // These should be removed by the higher typer
    for rule in rules.iter() {
        match rule {
            IRulexSR::MaybeCoercingCall(_) => panic!("vwat: MaybeCoercingCallSR should be removed by higher typer"),
            IRulexSR::MaybeCoercingLookup(_) => panic!("vwat: MaybeCoercingLookupSR should be removed by higher typer"),
            _ => {}
        }
    }
    Self { range, name, generic_params, rules, rune_to_type, sub_citizen_rune, sub_citizen_imprecise_name, interface_kind_rune, super_interface_imprecise_name }
}
/*
*/
// mig: fn hash_code
pub fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  override def hashCode(): Int = hash;
*/
// mig: fn equals
pub fn equals(&self, _obj: &dyn std::any::Any) -> bool {
    panic!("Unimplemented: equals");
}
}
/*
  override def equals(obj: Any): Boolean = {
    if (!obj.isInstanceOf[ImplA]) { return false }
    val that = obj.asInstanceOf[ImplA]
    return range == that.range && name == that.name;
  }
}
}
*/
// mig: struct ExportAsA
#[derive(Debug)]
pub struct ExportAsA<'a, 's> {
    pub range: RangeS<'a>,
    pub exported_name: StrI<'a>,
    pub rules: &'s [IRulexSR<'a>],
    pub rune_to_type: ArenaIndexMap<'s, IRuneS<'a>, ITemplataType>,
    pub type_rune: RuneUsage<'a>,
}
/*
case class ExportAsA(
  range: RangeS,
  exportedName: StrI,
  rules: Vector[IRulexSR],
  runeToType: Map[IRuneS, ITemplataType],
  typeRune: RuneUsage)
{
  val hash = range.hashCode() + exportedName.hashCode
*/
// mig: impl ExportAsA
impl<'a, 's> ExportAsA<'a, 's> {
/*
*/
// mig: fn hash_code
pub fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  override def hashCode(): Int = hash;
*/
// mig: fn equals
pub fn equals(&self, _obj: &dyn std::any::Any) -> bool {
    panic!("Unimplemented: equals");
}
}
/*
  override def equals(obj: Any): Boolean = {
    if (!obj.isInstanceOf[ImplA]) { return false }
    val that = obj.asInstanceOf[ExportAsA]
    return range == that.range && exportedName == that.exportedName;
  }
}
}
*/
// mig: trait CitizenA
pub trait CitizenA<'a, 's> {
/*
sealed trait CitizenA {
*/
// mig: fn tyype
fn tyype(&self) -> &TemplateTemplataType;
/*
  def tyype: TemplateTemplataType
*/
// mig: fn generic_parameters
fn generic_parameters(&self) -> &[GenericParameterS<'a, 's>];
/*
  def genericParameters: Vector[GenericParameterS]
*/
}
/*
}
*/
// mig: struct InterfaceA
#[derive(Debug)]
pub struct InterfaceA<'a, 's> {
    pub range: RangeS<'a>,
    pub name: &'a TopLevelInterfaceDeclarationNameS<'a>,
    pub attributes: &'s [ICitizenAttributeS<'a>],
    pub weakable: bool,
    pub mutability_rune: RuneUsage<'a>,
    pub maybe_predicted_mutability: Option<MutabilityP>,
    pub tyype: TemplateTemplataType,
    pub generic_parameters: &'s [&'s GenericParameterS<'a, 's>],
    pub rune_to_type: ArenaIndexMap<'s, IRuneS<'a>, ITemplataType>,
    pub rules: &'s [IRulexSR<'a>],
    pub internal_methods: &'s [&'s FunctionA<'a, 's>],
}
/*
case class InterfaceA(
  range: RangeS,
  name: TopLevelInterfaceDeclarationNameS,
  attributes: Vector[ICitizenAttributeS],
  weakable: Boolean,
  mutabilityRune: RuneUsage,
  // This is needed for recursive structures like
  //   struct ListNode<T> imm where T Ref {
  //     tail ListNode<T>;
  //   }
  maybePredictedMutability: Option[MutabilityP],
  tyype: TemplateTemplataType,
//    knowableRunes: Set[IRuneS],
  genericParameters: Vector[GenericParameterS],
//    localRunes: Set[IRuneS],
  runeToType: Map[IRuneS, ITemplataType],
  rules: Vector[IRulexSR],

  // See IMRFDI
  internalMethods: Vector[FunctionA]
) extends CitizenA {
  // These should be removed by the higher typer
  rules.collect({
    case MaybeCoercingCallSR(_, _, _, _) => vwat()
    case MaybeCoercingLookupSR(_, _, _) => vwat()
  })

  vassert(
    !genericParameters.exists({ case x =>
      x.rune.rune match {
        case DenizenDefaultRegionRuneS(_) => true
        case _ => false
      }
    }))
  vassert(
    !runeToType.exists({ case (rune, _) =>
      rune match {
        case DenizenDefaultRegionRuneS(_) => true
        case _ => false
      }
    }))

  val hash = range.hashCode() + name.hashCode()
*/
// mig: impl InterfaceA
impl<'a, 's> InterfaceA<'a, 's> {
pub fn new(
    range: RangeS<'a>,
    name: &'a TopLevelInterfaceDeclarationNameS<'a>,
    attributes: &'s [ICitizenAttributeS<'a>],
    weakable: bool,
    mutability_rune: RuneUsage<'a>,
    maybe_predicted_mutability: Option<MutabilityP>,
    tyype: TemplateTemplataType,
    generic_parameters: &'s [&'s GenericParameterS<'a, 's>],
    rune_to_type: ArenaIndexMap<'s, IRuneS<'a>, ITemplataType>,
    rules: &'s [IRulexSR<'a>],
    internal_methods: &'s [&'s FunctionA<'a, 's>],
) -> Self {
    // These should be removed by the higher typer
    for rule in rules.iter() {
        match rule {
            IRulexSR::MaybeCoercingCall(_) => panic!("vwat: MaybeCoercingCallSR should be removed by higher typer"),
            IRulexSR::MaybeCoercingLookup(_) => panic!("vwat: MaybeCoercingLookupSR should be removed by higher typer"),
            _ => {}
        }
    }
    assert!(
        !generic_parameters.iter().any(|x| matches!(x.rune.rune, IRuneS::DenizenDefaultRegionRune(_))),
        "vassert: generic_parameters should not contain DenizenDefaultRegionRuneS"
    );
    assert!(
        !rune_to_type.keys().any(|rune| matches!(rune, IRuneS::DenizenDefaultRegionRune(_))),
        "vassert: rune_to_type should not contain DenizenDefaultRegionRuneS"
    );
    for internal_method in internal_methods.iter() {
        assert!(
            generic_parameters == internal_method.generic_parameters,
            "vassert: internal method generic_parameters must match interface generic_parameters"
        );
    }
    Self { range, name, attributes, weakable, mutability_rune, maybe_predicted_mutability, tyype, generic_parameters, rune_to_type, rules, internal_methods }
}
/*
*/
// mig: fn hash_code
pub fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  override def hashCode(): Int = hash;
*/
// mig: fn equals
pub fn equals(&self, _obj: &dyn std::any::Any) -> bool {
    panic!("Unimplemented: equals");
}
}
/*
  override def equals(obj: Any): Boolean = {
    if (!obj.isInstanceOf[InterfaceA]) { return false }
    val that = obj.asInstanceOf[InterfaceA]
    return range == that.range && name == that.name;
  }

//  vassert((knowableRunes -- runeToType.keySet).isEmpty)
//  vassert((localRunes -- runeToType.keySet).isEmpty)

  internalMethods.foreach(internalMethod => {
    vassert(genericParameters == internalMethod.genericParameters)
  })
}
*/
/*
}
*/
// mig: mod interface_name
pub mod interface_name {
    use super::*;
/*
object interfaceName {
*/
// mig: fn unapply
pub fn unapply<'a, 's>(_interface_a: &'s InterfaceA<'a, 's>) -> Option<&'a TopLevelInterfaceDeclarationNameS<'a>> {
    panic!("Unimplemented: unapply");
}
}
/*
  // The extraction method (mandatory)
  def unapply(interfaceA: InterfaceA): Option[INameS] = {
    Some(interfaceA.name)
  }
}
*/
/*
}
*/
// mig: mod struct_name
pub mod struct_name {
    use super::*;
/*
object structName {
*/
// mig: fn unapply
pub fn unapply<'a, 's>(_struct_a: &'s StructA<'a, 's>) -> Option<&'a IStructDeclarationNameS<'a>> {
    panic!("Unimplemented: unapply");
}
}
/*
  // The extraction method (mandatory)
  def unapply(structA: StructA): Option[INameS] = {
    Some(structA.name)
  }
}
*/
/*
}
*/
/*
// remember, by doing a "m", CaptureSP("m", Destructure("Marine", Vector("hp, "item"))), by having that
// CaptureSP/"m" there, we're changing the nature of that Destructure; "hp" and "item" will be
// borrows rather than owns.

// So, when the scout is assigning everything a name, it's actually forcing us to always have
// borrowing destructures.

// We should change Scout to not assign names... or perhaps, it can assign names for the parameters,
// but secretly, typingpass will consider arguments to have actual names of __arg_0, __arg_1, and let
// the PatternCompiler introduce the actual names.

// Also remember, if a parameter has no name, it can't be varying.

// Underlying class for all XYZFunctionS types
*/
// mig: struct FunctionA
#[derive(Debug)]
pub struct FunctionA<'a, 's> {
    pub range: RangeS<'a>,
    pub name: IFunctionDeclarationNameS<'a>,
    pub attributes: &'s [IFunctionAttributeS<'a>],
    pub tyype: TemplateTemplataType,
    pub generic_parameters: &'s [&'s GenericParameterS<'a, 's>],
    pub rune_to_type: ArenaIndexMap<'s, IRuneS<'a>, ITemplataType>,
    pub params: &'s [ParameterS<'a>],
    pub maybe_ret_coord_rune: Option<RuneUsage<'a>>,
    pub rules: &'s [IRulexSR<'a>],
    pub body: IBodyS<'a, 's>,
}
/*
case class FunctionA(
    range: RangeS,
    name: IFunctionDeclarationNameS,

    // One day we might put a List of import statements here. After all, imports apply to
    // everything in the file.

    attributes: Vector[IFunctionAttributeS],

    tyype: TemplateTemplataType,
    // This is not necessarily only what the user specified, the compiler can add
    // things to the end here, see CCAUIR.
    genericParameters: Vector[GenericParameterS],

    runeToType: Map[IRuneS, ITemplataType],

    params: Vector[ParameterS],

    // We need to leave it an option to signal that the compiler can infer the return type.
    maybeRetCoordRune: Option[RuneUsage],

    rules: Vector[IRulexSR],
    body: IBodyS
) {
  val hash = range.hashCode() + name.hashCode()
  vpass()

  // These should be removed by the higher typer
  rules.collect({
    case MaybeCoercingCallSR(_, _, _, _) => vwat()
    case MaybeCoercingLookupSR(_, _, _) => vwat()
  })

  vassert(
    !genericParameters.exists({ case x =>
      x.rune.rune match { case DenizenDefaultRegionRuneS(_) => true case _ => false }
    }))
  vassert(
    !runeToType.exists({ case (rune, _) =>
      rune match {
        case DenizenDefaultRegionRuneS(_) => true
        case _ => false
      }
    }))

  vassert(range.begin.file.packageCoordinate == name.packageCoordinate)
*/
// mig: impl FunctionA
impl<'a, 's> FunctionA<'a, 's> {
pub fn new(
    range: RangeS<'a>,
    name: IFunctionDeclarationNameS<'a>,
    attributes: &'s [IFunctionAttributeS<'a>],
    tyype: TemplateTemplataType,
    generic_parameters: &'s [&'s GenericParameterS<'a, 's>],
    rune_to_type: ArenaIndexMap<'s, IRuneS<'a>, ITemplataType>,
    params: &'s [ParameterS<'a>],
    maybe_ret_coord_rune: Option<RuneUsage<'a>>,
    rules: &'s [IRulexSR<'a>],
    body: IBodyS<'a, 's>,
) -> Self {
    // These should be removed by the higher typer
    for rule in rules.iter() {
        match rule {
            IRulexSR::MaybeCoercingCall(_) => panic!("vwat: MaybeCoercingCallSR should be removed by higher typer"),
            IRulexSR::MaybeCoercingLookup(_) => panic!("vwat: MaybeCoercingLookupSR should be removed by higher typer"),
            _ => {}
        }
    }
    assert!(
        !generic_parameters.iter().any(|x| matches!(x.rune.rune, IRuneS::DenizenDefaultRegionRune(_))),
        "vassert: generic_parameters should not contain DenizenDefaultRegionRuneS"
    );
    assert!(
        !rune_to_type.keys().any(|rune| matches!(rune, IRuneS::DenizenDefaultRegionRune(_))),
        "vassert: rune_to_type should not contain DenizenDefaultRegionRuneS"
    );
    Self { range, name, attributes, tyype, generic_parameters, rune_to_type, params, maybe_ret_coord_rune, rules, body }
}
/*
*/
// mig: fn hash_code
pub fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  override def hashCode(): Int = hash;
*/
// mig: fn equals
pub fn equals(&self, _obj: &dyn std::any::Any) -> bool {
    panic!("Unimplemented: equals");
}
/*
  override def equals(obj: Any): Boolean = {
    if (!obj.isInstanceOf[FunctionA]) { return false }
    val that = obj.asInstanceOf[FunctionA]
    return range == that.range && name == that.name;
  }

  rules.foreach(rule => rule.runeUsages.foreach(rune => vassert(runeToType.contains(rune.rune))))
  params.flatMap(_.pattern.coordRune).foreach(runeUsage => {
    vassert(runeToType.contains(runeUsage.rune))
  })

//  // Make sure we have to solve all the identifying runes.
//  vassert((identifyingRunes.toSet -- localRunes).isEmpty)
//
//  vassert((knowableRunes -- runeToType.keySet).isEmpty)
//  vassert((localRunes -- runeToType.keySet).isEmpty)

*/
// mig: fn is_light
pub fn is_light(&self) -> bool {
    panic!("Unimplemented: is_light");
}
/*
  def isLight(): Boolean = {
    body match {
      case ExternBodyS | AbstractBodyS | GeneratedBodyS(_) => true
      case CodeBodyS(bodyA) => bodyA.closuredNames.isEmpty
    }
  }
*/
// mig: fn is_lambda
pub fn is_lambda(&self) -> bool {
    panic!("Unimplemented: is_lambda");
}
}
/*
  def isLambda(): Boolean = {
    name match {
      case LambdaDeclarationNameS(_) => true
      case _ => false
    }
  }
}
*/
/*
}
*/
