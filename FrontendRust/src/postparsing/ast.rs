use std::collections::HashMap;

use crate::interner::StrI;
use crate::parsing::ast::{IMacroInclusionP, MutabilityP, VariabilityP};
use crate::postparsing::expressions::BodySE;
use crate::postparsing::itemplatatype::{
  CoordTemplataType, ITemplataType, RegionTemplataType, TemplateTemplataType,
};
use crate::postparsing::names::{
  ExportAsNameS, IFunctionDeclarationNameS, IImpreciseNameS, IRuneS,
  TopLevelCitizenDeclarationNameS, TopLevelInterfaceDeclarationNameS, TopLevelStructDeclarationNameS,
  ImplDeclarationNameS,
};
use crate::postparsing::patterns::AtomSP;
use crate::postparsing::rules::{IRulexSR, RuneUsage};
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::RangeS;

/*
package dev.vale.postparsing

import dev.vale._
import dev.vale.parsing.ast.{IMacroInclusionP, IRuneAttributeP, MutabilityP, VariabilityP}
import dev.vale.postparsing.rules.{IRulexSR, RuneUsage}
import dev.vale.parsing._
import dev.vale.postparsing.patterns._
import dev.vale.postparsing.rules._

import scala.collection.immutable.List
*/
pub trait IExpressionSE<'a> {
  fn range(&self) -> RangeS<'a>;
}

/*
trait IExpressionSE {
  def range: RangeS
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ProgramS<'a, 's> {
  pub structs: &'s [StructS<'a, 's>],
  pub interfaces: &'s [InterfaceS<'a, 's>],
  pub impls: &'s [ImplS<'a, 's>],
  pub implemented_functions: &'s [FunctionS<'a, 's>],
  pub exports: &'s [ExportAsS<'a, 's>],
  pub imports: &'s [ImportS<'a, 's>],
}

impl<'a, 's> ProgramS<'a, 's> {
  pub fn lookup_function(&self, name: &str) -> &FunctionS<'a, 's> {
    let matches: Vec<&FunctionS<'a, 's>> = self
      .implemented_functions
      .iter()
      .filter(|f| match &f.name {
        IFunctionDeclarationNameS::FunctionName(n) => n.name.as_str() == name,
        _ => false,
      })
      .collect();
    assert_eq!(matches.len(), 1);
    matches[0]
  }

  pub fn lookup_interface(&self, name: &str) -> &InterfaceS<'a, 's> {
    let matches: Vec<&InterfaceS<'a, 's>> = self
      .interfaces
      .iter()
      .filter(|i| i.name.name.as_str() == name)
      .collect();
    assert_eq!(matches.len(), 1);
    matches[0]
  }

  pub fn lookup_struct(&self, name: &str) -> &StructS<'a, 's> {
    let matches: Vec<&StructS<'a, 's>> = self
      .structs
      .iter()
      .filter(|s| s.name.name.as_str() == name)
      .collect();
    assert_eq!(matches.len(), 1);
    matches[0]
  }
}

/*
case class ProgramS(
    structs: Vector[StructS],
    interfaces: Vector[InterfaceS],
    impls: Vector[ImplS],
    implementedFunctions: Vector[FunctionS],
    exports: Vector[ExportAsS],
    imports: Vector[ImportS]) {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
*/
/*
  def lookupFunction(name: String): FunctionS = {
    val matches =
      implementedFunctions
        .find(f => f.name match { case FunctionNameS(n, _) => n.str == name })
    vassert(matches.size == 1)
    matches.head
  }
*/
/*
  def lookupInterface(name: String): InterfaceS = {
    val matches =
      interfaces
        .find(f => f.name match { case TopLevelCitizenDeclarationNameS(n, _) => n.str == name })
    vassert(matches.size == 1)
    matches.head
  }
*/
/*
  def lookupStruct(name: String): StructS = {
    val matches =
      structs
        .find(f => f.name match { case TopLevelCitizenDeclarationNameS(n, _) => n.str == name })
    vassert(matches.size == 1)
    matches.head
  }
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ExternS<'a> {
  pub package_coord: &'a PackageCoordinate<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BuiltinS<'a> {
  // MIGTODO: can we give everything a lifetime into an arena so we can
  // all have references instead of using Arc everywhere?
  pub generator_name: StrI<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MacroCallS<'a> {
  pub range: RangeS<'a>,
  pub include: IMacroInclusionP,
  pub macro_name: StrI<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExportS<'a> {
  pub package_coordinate: &'a PackageCoordinate<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PureS;

#[derive(Clone, Debug, PartialEq)]
pub struct AdditiveS;

#[derive(Clone, Debug, PartialEq)]
pub struct SealedS;

#[derive(Clone, Debug, PartialEq)]
pub struct UserFunctionS;

#[derive(Clone, Debug, PartialEq)]
pub enum ICitizenAttributeS<'a> {
  Extern(ExternS<'a>),
  Sealed(SealedS),
  Builtin(BuiltinS<'a>),
  MacroCall(MacroCallS<'a>),
  Export(ExportS<'a>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum IFunctionAttributeS<'a> {
  Extern(ExternS<'a>),
  Pure(PureS),
  Additive(AdditiveS),
  Builtin(BuiltinS<'a>),
  Export(ExportS<'a>),
  UserFunction(UserFunctionS),
}

/*
sealed trait ICitizenAttributeS
sealed trait IFunctionAttributeS
case class ExternS(packageCoord: PackageCoordinate) extends IFunctionAttributeS with ICitizenAttributeS {
  val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash;
}
case object PureS extends IFunctionAttributeS
case object AdditiveS extends IFunctionAttributeS
case object SealedS extends ICitizenAttributeS
case class BuiltinS(generatorName: StrI) extends IFunctionAttributeS with ICitizenAttributeS {
  val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash;
}
case class MacroCallS(range: RangeS, include: IMacroInclusionP, macroName: StrI) extends ICitizenAttributeS {
  val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash;
}
case class ExportS(packageCoordinate: PackageCoordinate) extends IFunctionAttributeS with ICitizenAttributeS {
  val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash;
}
case object UserFunctionS extends IFunctionAttributeS // Whether it was written by a human. Mostly for tests right now.
*/
#[derive(Clone, Debug, PartialEq)]
pub enum ICitizenS<'a, 's> {
  Struct(StructS<'a, 's>),
  Interface(InterfaceS<'a, 's>),
}

impl<'a, 's> ICitizenS<'a, 's> {
  pub fn name(&self) -> TopLevelCitizenDeclarationNameS<'_> {
    match self {
      ICitizenS::Struct(s) => TopLevelCitizenDeclarationNameS::from(&s.name),
      ICitizenS::Interface(i) => TopLevelCitizenDeclarationNameS::from(&i.name),
    }
  }

  pub fn tyype(&self) -> &TemplateTemplataType {
    match self {
      ICitizenS::Struct(s) => &s.tyype,
      ICitizenS::Interface(i) => &i.tyype,
    }
  }

  pub fn generic_params(&self) -> &'s [GenericParameterS<'a>] {
    match self {
      ICitizenS::Struct(s) => s.generic_params,
      ICitizenS::Interface(i) => i.generic_params,
    }
  }
}

/*
sealed trait ICitizenS {
  def name: ICitizenDeclarationNameS
  def tyype: TemplateTemplataType
  def genericParams: Vector[GenericParameterS]
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct StructS<'a, 's> {
  pub range: RangeS<'a>,
  pub name: TopLevelStructDeclarationNameS<'a>,
  pub attributes: &'s [ICitizenAttributeS<'a>],
  pub weakable: bool,
  pub generic_params: &'s [GenericParameterS<'a>],
  pub mutability_rune: RuneUsage<'a>,
  pub maybe_predicted_mutability: Option<MutabilityP>,
  pub tyype: TemplateTemplataType,
  pub header_rune_to_explicit_type: HashMap<IRuneS<'a>, ITemplataType>,
  pub header_predicted_rune_to_type: HashMap<IRuneS<'a>, ITemplataType>,
  pub header_rules: &'s [IRulexSR<'a>],
  pub members_rune_to_explicit_type: HashMap<IRuneS<'a>, ITemplataType>,
  pub members_predicted_rune_to_type: HashMap<IRuneS<'a>, ITemplataType>,
  pub member_rules: &'s [IRulexSR<'a>],
  pub members: &'s [IStructMemberS<'a>],
}

/*
case class StructS(
    range: RangeS,
    name: TopLevelStructDeclarationNameS,
    attributes: Vector[ICitizenAttributeS],
    weakable: Boolean,
    genericParams: Vector[GenericParameterS],
    mutabilityRune: RuneUsage,

    // This is needed for recursive structures like
    //   struct ListNode<T> imm where T Ref {
    //     tail ListNode<T>;
    //   }
    maybePredictedMutability: Option[MutabilityP],
    tyype: TemplateTemplataType,

    // These are separated so that these alone can be run during resolving, see SMRASDR.
    headerRuneToExplicitType: Map[IRuneS, ITemplataType],
    headerPredictedRuneToType: Map[IRuneS, ITemplataType],
    headerRules: Vector[IRulexSR],
    // These are separated so they can be skipped during resolving, see SMRASDR.
    membersRuneToExplicitType: Map[IRuneS, ITemplataType],
    membersPredictedRuneToType: Map[IRuneS, ITemplataType],
    memberRules: Vector[IRulexSR],

    members: Vector[IStructMemberS]
) extends ICitizenS {

  vassert(
    !genericParams.exists({ case x =>
      x.rune.rune match {
        case DenizenDefaultRegionRuneS(_) => true
        case _ => false
      }
    }))
  vassert(
    !(membersRuneToExplicitType ++ membersPredictedRuneToType ++ headerRuneToExplicitType ++ headerPredictedRuneToType)
        .keys
        .exists({ case rune =>
      rune match {
        case DenizenDefaultRegionRuneS(_) => true
        case _ => false
      }
    }))

  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()

//  vassert(isTemplate == identifyingRunes.nonEmpty)
}
*/
#[derive(Clone, Debug, PartialEq)]
pub enum IStructMemberS<'a> {
  NormalStructMember(NormalStructMemberS<'a>),
  VariadicStructMember(VariadicStructMemberS<'a>),
}

impl IStructMemberS<'_> {
  pub fn range(&self) -> RangeS<'_> {
    match self {
      IStructMemberS::NormalStructMember(m) => m.range.clone(),
      IStructMemberS::VariadicStructMember(m) => m.range.clone(),
    }
  }

  pub fn variability(&self) -> VariabilityP {
    match self {
      IStructMemberS::NormalStructMember(m) => m.variability,
      IStructMemberS::VariadicStructMember(m) => m.variability,
    }
  }

  pub fn type_rune(&self) -> &RuneUsage<'_> {
    match self {
      IStructMemberS::NormalStructMember(m) => &m.type_rune,
      IStructMemberS::VariadicStructMember(m) => &m.type_rune,
    }
  }
}

/*
sealed trait IStructMemberS {
  def range: RangeS
  def variability: VariabilityP
  def typeRune: RuneUsage
}
  */
#[derive(Clone, Debug, PartialEq)]
pub struct NormalStructMemberS<'a> {
  pub range: RangeS<'a>,
  pub name: StrI<'a>,
  pub variability: VariabilityP,
  pub type_rune: RuneUsage<'a>,
}

/*
case class NormalStructMemberS(
    range: RangeS,
    name: StrI,
    variability: VariabilityP,
    typeRune: RuneUsage) extends IStructMemberS {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
  */
#[derive(Clone, Debug, PartialEq)]
pub struct VariadicStructMemberS<'a> {
  pub range: RangeS<'a>,
  pub variability: VariabilityP,
  pub type_rune: RuneUsage<'a>,
}

/*
case class VariadicStructMemberS(
  range: RangeS,
  variability: VariabilityP,
  typeRune: RuneUsage) extends IStructMemberS {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceS<'a, 's> {
  pub range: RangeS<'a>,
  pub name: TopLevelInterfaceDeclarationNameS<'a>,
  pub attributes: &'s [ICitizenAttributeS<'a>],
  pub weakable: bool,
  pub generic_params: &'s [GenericParameterS<'a>],
  pub rune_to_explicit_type: HashMap<IRuneS<'a>, ITemplataType>,
  pub mutability_rune: RuneUsage<'a>,
  pub maybe_predicted_mutability: Option<MutabilityP>,
  pub predicted_rune_to_type: HashMap<IRuneS<'a>, ITemplataType>,
  pub tyype: TemplateTemplataType,
  pub rules: &'s [IRulexSR<'a>],
  pub internal_methods: &'s [FunctionS<'a, 's>],
}

/*
case class InterfaceS(
  range: RangeS,
  name: TopLevelInterfaceDeclarationNameS,
  attributes: Vector[ICitizenAttributeS],
  weakable: Boolean,
  genericParams: Vector[GenericParameterS],
  runeToExplicitType: Map[IRuneS, ITemplataType],
  mutabilityRune: RuneUsage,

  // This is needed for recursive structures like
  //   struct ListNode<T> imm where T Ref {
  //     tail ListNode<T>;
  //   }
  maybePredictedMutability: Option[MutabilityP],
  predictedRuneToType: Map[IRuneS, ITemplataType],
  tyype: TemplateTemplataType,

  rules: Vector[IRulexSR],

  // See IMRFDI
  internalMethods: Vector[FunctionS]
) extends ICitizenS {

  vassert(
    !genericParams.exists({ case x =>
      x.rune.rune match {
        case DenizenDefaultRegionRuneS(_) => true
        case _ => false
      }
    }))
  vassert(
    !(runeToExplicitType ++ predictedRuneToType).exists({ case (rune, _) =>
      rune match {
        case DenizenDefaultRegionRuneS(_) => true
        case _ => false
      }
    }))

  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()

  internalMethods.foreach(internalMethod => {
    vregionmut() // Put this back in when we have regions
    // // .init because every method has a default region as the last region param.
    // vassert(genericParams == internalMethod.genericParams.init)
    // Take this out when we have regions
    vassert(genericParams == internalMethod.genericParams)
  })

}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ImplS<'a, 's> {
  pub range: RangeS<'a>,
  pub name: ImplDeclarationNameS<'a>,
  pub user_specified_identifying_runes: &'s [GenericParameterS<'a>],
  pub rules: &'s [IRulexSR<'a>],
  pub rune_to_explicit_type: HashMap<IRuneS<'a>, ITemplataType>,
  pub tyype: ITemplataType,
  pub struct_kind_rune: RuneUsage<'a>,
  pub sub_citizen_imprecise_name: IImpreciseNameS<'a>,
  pub interface_kind_rune: RuneUsage<'a>,
  pub super_interface_imprecise_name: IImpreciseNameS<'a>,
}

/*
case class ImplS(
    range: RangeS,
    // The name of an impl is the human name of the subcitizen, see INSHN.
    name: ImplDeclarationNameS,
    userSpecifiedIdentifyingRunes: Vector[GenericParameterS],
    rules: Vector[IRulexSR],
    runeToExplicitType: Map[IRuneS, ITemplataType],
    tyype: ITemplataType,
    structKindRune: RuneUsage,
    subCitizenImpreciseName: IImpreciseNameS,
    interfaceKindRune: RuneUsage,
    superInterfaceImpreciseName: IImpreciseNameS) {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ExportAsS<'a, 's> {
  pub range: RangeS<'a>,
  pub rules: &'s [IRulexSR<'a>],
  pub export_name: ExportAsNameS<'a>,
  pub rune: RuneUsage<'a>,
  pub exported_name: StrI<'a>,
}

/*
case class ExportAsS(
  range: RangeS,
  rules: Vector[IRulexSR],
  exportName: ExportAsNameS,
  rune: RuneUsage,
  exportedName: StrI) {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ImportS<'a, 's> {
  pub range: RangeS<'a>,
  pub module_name: StrI<'a>,
  pub package_names: &'s [StrI<'a>],
  pub importee_name: StrI<'a>,
}

/*
case class ImportS(
  range: RangeS,
  moduleName: StrI,
  packageNames: Vector[StrI],
  importeeName: StrI) {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
pub fn interface_s_name<'a, 's>(interface_s: &InterfaceS<'a, 's>) -> TopLevelCitizenDeclarationNameS<'a> {
  TopLevelCitizenDeclarationNameS::from(&interface_s.name)
}

/*
object interfaceSName {
  // The extraction method (mandatory)
  def unapply(interfaceS: InterfaceS): Option[TopLevelCitizenDeclarationNameS] = {
    Some(interfaceS.name)
  }
}
*/
pub fn struct_s_name<'a, 's>(struct_s: &StructS<'a, 's>) -> TopLevelCitizenDeclarationNameS<'a> {
  TopLevelCitizenDeclarationNameS::from(&struct_s.name)
}

/*
object structSName {
  // The extraction method (mandatory)
  def unapply(structS: StructS): Option[TopLevelCitizenDeclarationNameS] = {
    Some(structS.name)
  }
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
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ParameterS<'a> {
  pub range: RangeS<'a>,
  pub virtuality: Option<AbstractSP<'a>>,
  pub pre_checked: bool,
  pub pattern: AtomSP<'a>,
}

/*
case class ParameterS(
  range: RangeS,
  virtuality: Option[AbstractSP],
  preChecked: Boolean,
  pattern: AtomSP) {

  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()

  vassert(pattern.coordRune.nonEmpty)
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct AbstractSP<'a> {
  pub range: RangeS<'a>,
  pub is_internal_method: bool,
}

/*
case class AbstractSP(
  range: RangeS,
  // True if this is defined inside an interface
  // False if this is a free function somewhere else
  isInternalMethod: Boolean
)
*/
#[derive(Clone, Debug, PartialEq)]
pub struct SimpleParameterS<'a> {
  pub origin: Option<AtomSP<'a>>,
  pub name: String,
  pub virtuality: Option<AbstractSP<'a>>,
  pub tyype: IRulexSR<'a>,
}

/*
case class SimpleParameterS(
    origin: Option[AtomSP],
    name: String,
    virtuality: Option[AbstractSP],
    tyype: IRulexSR) {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct GeneratedBodyS<'a> {
  pub generator_id: StrI<'a>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CodeBodyS<'a, 's> {
  pub body: BodySE<'a, 's>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExternBodyS {}

#[derive(Clone, Debug, PartialEq)]
pub struct AbstractBodyS {}

#[derive(Clone, Debug, PartialEq)]
pub enum IBodyS<'a, 's> {
  ExternBody(ExternBodyS),
  AbstractBody(AbstractBodyS),
  GeneratedBody(GeneratedBodyS<'a>),
  CodeBody(CodeBodyS<'a, 's>),
}

/*
sealed trait IBodyS
case object ExternBodyS extends IBodyS
case object AbstractBodyS extends IBodyS
case class GeneratedBodyS(generatorId: StrI) extends IBodyS {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
case class CodeBodyS(body: BodySE) extends IBodyS {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IRegionMutabilityS {
  ReadWriteRegion,
  ReadOnlyRegion,
  ImmutableRegion,
  AdditiveRegion,
}

/*
sealed trait IRegionMutabilityS
case object ReadWriteRegionS extends IRegionMutabilityS
case object ReadOnlyRegionS extends IRegionMutabilityS
case object ImmutableRegionS extends IRegionMutabilityS
case object AdditiveRegionS extends IRegionMutabilityS
*/
#[derive(Clone, Debug, PartialEq)]
pub enum IGenericParameterTypeS<'a> {
  RegionGenericParameterType(RegionGenericParameterTypeS),
  CoordGenericParameterType(CoordGenericParameterTypeS<'a>),
  OtherGenericParameterType(OtherGenericParameterTypeS),
}

impl IGenericParameterTypeS<'_> {
  pub fn expect_region(&self) -> &RegionGenericParameterTypeS {
    match self {
      IGenericParameterTypeS::RegionGenericParameterType(x) => x,
      _ => panic!("Expected region generic parameter type"),
    }
  }

  pub fn tyype(&self) -> ITemplataType {
    match self {
      IGenericParameterTypeS::RegionGenericParameterType(x) => x.tyype(),
      IGenericParameterTypeS::CoordGenericParameterType(x) => x.tyype(),
      IGenericParameterTypeS::OtherGenericParameterType(x) => x.tyype.clone(),
    }
  }
}

/*
object IGenericParameterTypeS {
  def expectRegion(x: IGenericParameterTypeS): RegionGenericParameterTypeS = {
    x match {
      case z @ RegionGenericParameterTypeS(_) => z
      case _ => vfail()
    }
  }
}
*/
/*
sealed trait IGenericParameterTypeS {
  def tyype: ITemplataType
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct RegionGenericParameterTypeS {
  pub mutability: IRegionMutabilityS,
}

impl RegionGenericParameterTypeS {
  pub fn tyype(&self) -> ITemplataType {
    ITemplataType::RegionTemplataType(RegionTemplataType {})
  }
}

/*
case class RegionGenericParameterTypeS(mutability: IRegionMutabilityS) extends IGenericParameterTypeS {
  def tyype: ITemplataType = RegionTemplataType()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct CoordGenericParameterTypeS<'a> {
  pub coord_region: Option<RuneUsage<'a>>,
  pub kind_mutable: bool,
  pub region_mutable: bool,
}

impl CoordGenericParameterTypeS<'_> {
  pub fn tyype(&self) -> ITemplataType {
    assert!(self.coord_region.is_none());
    ITemplataType::CoordTemplataType(CoordTemplataType {})
  }
}

/*
case class CoordGenericParameterTypeS(
    coordRegion: Option[RuneUsage],
    kindMutable: Boolean,
    regionMutable: Boolean
) extends IGenericParameterTypeS {
  vassert(coordRegion.isEmpty) // not implemented yet

  def tyype: ITemplataType = CoordTemplataType()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct OtherGenericParameterTypeS {
  pub tyype: ITemplataType,
}

/*
case class OtherGenericParameterTypeS(tyype: ITemplataType) extends IGenericParameterTypeS {
  tyype match {
    case RegionTemplataType() | CoordTemplataType() => vwat() // Use other types for this
    case _ =>
  }
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct GenericParameterS<'a> {
  pub range: RangeS<'a>,
  pub rune: RuneUsage<'a>,
  pub tyype: IGenericParameterTypeS<'a>,
  pub default: Option<GenericParameterDefaultS<'a>>,
}

/*
case class GenericParameterS(
  range: RangeS,
  rune: RuneUsage,
  tyype: IGenericParameterTypeS,
  default: Option[GenericParameterDefaultS])
*/
/*
//sealed trait IRuneAttributeS
//case class ImmutableRuneAttributeS(range: RangeS) extends IRuneAttributeS
//case class ReadWriteRuneAttributeS(range: RangeS) extends IRuneAttributeS
//case class ReadOnlyRuneAttributeS(range: RangeS) extends IRuneAttributeS
*/
#[derive(Clone, Debug, PartialEq)]
pub struct GenericParameterDefaultS<'a> {
  pub result_rune: IRuneS<'a>,
  pub rules: Vec<IRulexSR<'a>>,
}

/*
case class GenericParameterDefaultS(
  // One day, when we want more rules in here, we might need to have a runeToType map
  // and other things to make it its own little world.
  resultRune: IRuneS,
  rules: Vector[IRulexSR])
*/
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionS<'a, 's> {
  pub range: RangeS<'a>,
  pub name: IFunctionDeclarationNameS<'a>,
  pub attributes: &'s [IFunctionAttributeS<'a>],
  pub generic_params: &'s [GenericParameterS<'a>],
  pub rune_to_predicted_type: HashMap<IRuneS<'a>, ITemplataType>,
  pub tyype: TemplateTemplataType,
  pub params: &'s [ParameterS<'a>],
  pub maybe_ret_coord_rune: Option<RuneUsage<'a>>,
  pub rules: &'s [IRulexSR<'a>],
  pub body: IBodyS<'a, 's>,
}

impl<'a, 's> FunctionS<'a, 's> {
  pub fn is_light(&self) -> bool {
    match &self.body {
      IBodyS::ExternBody(_) | IBodyS::AbstractBody(_) | IBodyS::GeneratedBody(_) => false,
      IBodyS::CodeBody(body) => !body.body.closured_names.is_empty(),
    }
  }
}

/*
// Underlying class for all XYZFunctionS types
case class FunctionS(
  range: RangeS,
  name: IFunctionDeclarationNameS,
  attributes: Vector[IFunctionAttributeS],

  genericParams: Vector[GenericParameterS],
  runeToPredictedType: Map[IRuneS, ITemplataType],
  tyype: TemplateTemplataType,

  params: Vector[ParameterS],

  // We need to leave it an option to signal that the compiler can infer the return type.
  maybeRetCoordRune: Option[RuneUsage],

  rules: Vector[IRulexSR],
  body: IBodyS
) {
  vpass()

  // Put this back in when we have regions
  // // Every function needs a region generic parameter, see DRIAGP.
  // vassert(genericParams.nonEmpty)
  // Take this out when we have regions
  vassert(
    !genericParams.exists({ case x =>
      x.rune.rune match { case DenizenDefaultRegionRuneS(_) => true case _ => false }
    }))
  vassert(
    !runeToPredictedType.exists({ case (rune, _) =>
      rune match {
        case DenizenDefaultRegionRuneS(_) => true
        case _ => false
      }
    }))

  body match {
    case ExternBodyS | AbstractBodyS | GeneratedBodyS(_) => {
      name match {
        case LambdaDeclarationNameS(_) => vwat()
        case _ =>
      }
    }
    case CodeBodyS(body) => {
      if (body.closuredNames.nonEmpty) {
        name match {
          case LambdaDeclarationNameS(_) =>
          case _ => vwat()
        }
      }
    }
  }

  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
*/
/*
  def isLight(): Boolean = {
    body match {
      case ExternBodyS | AbstractBodyS | GeneratedBodyS(_) => false
      case CodeBodyS(bodyS) => bodyS.closuredNames.nonEmpty
    }
  }
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocationInDenizen {
  pub path: Vec<i32>,
}

impl LocationInDenizen {
  pub fn before(&self, that: &LocationInDenizen) -> bool {
    for (this_step, that_step) in self.path.iter().zip(that.path.iter()) {
      if this_step < that_step {
        return true;
      }
      if this_step > that_step {
        return false;
      }
    }
    if self.path.len() < that.path.len() {
      return true;
    }
    if self.path.len() > that.path.len() {
      return false;
    }
    false
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LocationInDenizenBuilder {
  path: Vec<i32>,
  consumed: bool,
  next_child: i32,
}

impl LocationInDenizenBuilder {
  pub fn new(path: Vec<i32>) -> Self {
    Self {
      path,
      consumed: false,
      next_child: 1,
    }
  }

  pub fn child(&mut self) -> LocationInDenizenBuilder {
    let child = self.next_child;
    self.next_child += 1;
    let mut child_path = self.path.clone();
    child_path.push(child);
    LocationInDenizenBuilder::new(child_path)
  }

  pub fn consume(&mut self) -> LocationInDenizen {
    assert!(
      !self.consumed,
      "Location in denizen was already used for something, add a .child() somewhere."
    );
    self.consumed = true;
    LocationInDenizen {
      path: self.path.clone(),
    }
  }
}

/*
// A Denizen is a thing at the top level of a file, like structs, functions, impls, exports, etc.
// This is a class with a consumed boolean so that we're sure we don't use it twice.
// Anyone that uses it should call the consume() method.
// Move semantics would be nice here... alas.
class LocationInDenizenBuilder(path: Vector[Int]) {
  private var consumed: Boolean = false
  private var nextChild: Int = 1

  // Note how this is hashing `path`, not `this` like usual.
  val hash = runtime.ScalaRunTime._hashCode(path.toList); override def hashCode(): Int = hash; override def equals(obj: Any): Boolean = vcurious();

  def child(): LocationInDenizenBuilder = {
    val child = nextChild
    nextChild = nextChild + 1
    new LocationInDenizenBuilder(path :+ child)
  }

  def consume(): LocationInDenizen = {
    assert(!consumed, "Location in denizen was already used for something, add a .child() somewhere.")
    consumed = true
    LocationInDenizen(path)
  }

  override def toString: String = path.mkString(".")
}
*/
/*
case class LocationInDenizen(path: Vector[Int]) {
  val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash;
  override def equals(obj: Any): Boolean = {
    obj match {
      case LocationInDenizen(thatPath) => path == thatPath
      case _ => false
    }
  }
*/
/*
  def before(that: LocationInDenizen): Boolean = {
    this.path.zip(that.path).foreach({ case (thisStep, thatStep) =>
      if (thisStep < thatStep) {
        return true
      }
      if (thisStep > thatStep) {
        return false
      }
    })
    // If we get here, their steps match up... but one might have more steps than the other.
    if (this.path.length < that.path.length) {
      return true
    }
    if (this.path.length > that.path.length) {
      return false
    }
    // They're equal.
    return false
  }
}

*/
#[derive(Clone, Debug, PartialEq)]
pub struct TopLevelFunctionS<'a, 's> {
  pub function: FunctionS<'a, 's>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopLevelImplS<'a, 's> {
  pub impl_: ImplS<'a, 's>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopLevelExportAsS<'a, 's> {
  pub export: ExportAsS<'a, 's>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopLevelImportS<'a, 's> {
  pub imporrt: ImportS<'a, 's>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopLevelStructS<'a, 's> {
  pub strukt: StructS<'a, 's>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TopLevelInterfaceS<'a, 's> {
  pub interface: InterfaceS<'a, 's>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum IDenizenS<'a, 's> {
  TopLevelFunction(TopLevelFunctionS<'a, 's>),
  TopLevelImpl(TopLevelImplS<'a, 's>),
  TopLevelExportAs(TopLevelExportAsS<'a, 's>),
  TopLevelImport(TopLevelImportS<'a, 's>),
  TopLevelStruct(TopLevelStructS<'a, 's>),
  TopLevelInterface(TopLevelInterfaceS<'a, 's>),
}

#[derive(Clone, Debug, PartialEq)]
pub enum ICitizenDenizenS<'a, 's> {
  TopLevelStruct(TopLevelStructS<'a, 's>),
  TopLevelInterface(TopLevelInterfaceS<'a, 's>),
}

impl<'a, 's> ICitizenDenizenS<'a, 's> {
  pub fn citizen(&self) -> ICitizenS<'a, 's> {
    match self {
      ICitizenDenizenS::TopLevelStruct(s) => ICitizenS::Struct(s.strukt.clone()),
      ICitizenDenizenS::TopLevelInterface(i) => ICitizenS::Interface(i.interface.clone()),
    }
  }
}

pub fn as_citizen_denizen<'a, 's>(x: &IDenizenS<'a, 's>) -> Option<ICitizenDenizenS<'a, 's>> {
  match x {
    IDenizenS::TopLevelStruct(s) => Some(ICitizenDenizenS::TopLevelStruct(s.clone())),
    IDenizenS::TopLevelInterface(i) => Some(ICitizenDenizenS::TopLevelInterface(i.clone())),
    _ => None,
  }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FileS<'a, 's> {
  pub denizens: Vec<IDenizenS<'a, 's>>,
}

/*
sealed trait IDenizenS
case class TopLevelFunctionS(function: FunctionS) extends IDenizenS { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class TopLevelImplS(impl: ImplS) extends IDenizenS { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class TopLevelExportAsS(export: ExportAsS) extends IDenizenS { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class TopLevelImportS(imporrt: ImportS) extends IDenizenS { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/
/*
object ICitizenDenizenS {
  def unapply(x: IDenizenS): Option[ICitizenS] = {
    x match {
      case TopLevelStructS(s) => Some(s)
      case TopLevelInterfaceS(i) => Some(i)
      case _ => None
    }
  }
}
*/
/*
sealed trait ICitizenDenizenS extends IDenizenS {
  def citizen: ICitizenS
}
*/
/*
case class TopLevelStructS(struct: StructS) extends ICitizenDenizenS {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def citizen: ICitizenS = struct
}
*/
/*
case class TopLevelInterfaceS(interface: InterfaceS) extends ICitizenDenizenS {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  override def citizen: ICitizenS = interface
}
*/
/*
case class FileS(denizens: Vector[IDenizenS])
*/