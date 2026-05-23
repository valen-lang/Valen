use crate::interner::StrI;
use crate::utils::arena_index_map::ArenaIndexMap;
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
use crate::scout_arena::ScoutArena;

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
pub trait IExpressionSE<'s> {
  fn range(&self) -> RangeS<'s>;
  /* Guardian: disable-all */
}
/*
trait IExpressionSE {
  def range: RangeS
}
*/
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ProgramS<'s> {
  pub structs: &'s [&'s StructS<'s>],
  pub interfaces: &'s [&'s InterfaceS<'s>],
  pub impls: &'s [&'s ImplS<'s>],
  pub implemented_functions: &'s [&'s FunctionS<'s>],
  pub exports: &'s [&'s ExportAsS<'s>],
  pub imports: &'s [&'s ImportS<'s>],
}
/*
case class ProgramS(
    structs: Vector[StructS],
    interfaces: Vector[InterfaceS],
    impls: Vector[ImplS],
    implementedFunctions: Vector[FunctionS],
    exports: Vector[ExportAsS],
    imports: Vector[ImportS]) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
*/
// V: lets make sure equals and hashCode are mentioned in the shields as exceptions.
// V: lets combine the various "must match scala" shields
// VA: (these are process/shield-editing tasks, not code questions — not investigated here)

impl<'s> ProgramS<'s> {
  pub fn lookup_function(&'s self, name: &str) -> &'s FunctionS<'s> {
    let matches: Vec<&'s FunctionS<'s>> = self
      .implemented_functions
      .iter()
      .filter(|f| match &f.name {
        IFunctionDeclarationNameS::FunctionName(n) => n.name.as_str() == name,
        _ => false,
      })
      .map(|f| *f)
      .collect::<Vec<&'s FunctionS<'s>>>();
    assert_eq!(matches.len(), 1);
    matches[0]
  }
  /*
    def lookupFunction(name: String): FunctionS = {
      val matches =
        implementedFunctions
          .find(f => f.name match { case FunctionNameS(n, _) => n.str == name })
      vassert(matches.size == 1)
      matches.head
    }
  */

  pub fn lookup_interface(&self, name: &str) -> &'s InterfaceS<'s> {
    let matches = self
      .interfaces
      .iter()
      .copied()
      .find(|i| i.name.name.as_str() == name);
    assert_eq!(matches.is_some(), true);
    matches.unwrap()
  }
  /*
    def lookupInterface(name: String): InterfaceS = {
      val matches =
        interfaces
          .find(f => f.name match { case TopLevelCitizenDeclarationNameS(n, _) => n.str == name })
      vassert(matches.size == 1)
      matches.head
    }
  */

  pub fn lookup_struct(&self, name: &str) -> &'s StructS<'s> {
    let matches: Vec<&'s StructS<'s>> = self
      .structs
      .iter()
      .copied()
      .filter(|s| s.name.name.as_str() == name)
      .collect();
    assert_eq!(matches.len(), 1);
    matches[0]
  }
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
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ICitizenAttributeS<'s> {
  Extern(ExternS<'s>),
  Sealed(SealedS),
  Builtin(BuiltinS<'s>),
  MacroCall(MacroCallS<'s>),
  Export(ExportS<'s>),
}
/*
sealed trait ICitizenAttributeS
*/


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IFunctionAttributeS<'s> {
  Extern(ExternS<'s>),
  Pure(PureS),
  Additive(AdditiveS),
  Builtin(BuiltinS<'s>),
  Export(ExportS<'s>),
  UserFunction(UserFunctionS),
}
/*
sealed trait IFunctionAttributeS
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExternS<'s> {
  pub package_coord: &'s PackageCoordinate<'s>,
}
/*
case class ExternS(packageCoord: PackageCoordinate) extends IFunctionAttributeS with ICitizenAttributeS {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PureS;
/*
case object PureS extends IFunctionAttributeS
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AdditiveS;
/*
case object AdditiveS extends IFunctionAttributeS
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SealedS;
/*
case object SealedS extends ICitizenAttributeS
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuiltinS<'s> {
  // AFTERM: can we give everything a lifetime into an arena so we can
  // all have references instead of using Arc everywhere?
  pub generator_name: StrI<'s>,
}
/*
case class BuiltinS(generatorName: StrI) extends IFunctionAttributeS with ICitizenAttributeS {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MacroCallS<'s> {
  pub range: RangeS<'s>,
  pub include: IMacroInclusionP,
  pub macro_name: StrI<'s>,
}
/*
case class MacroCallS(range: RangeS, include: IMacroInclusionP, macroName: StrI) extends ICitizenAttributeS {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExportS<'s> {
  pub package_coordinate: &'s PackageCoordinate<'s>,
}
/*
case class ExportS(packageCoordinate: PackageCoordinate) extends IFunctionAttributeS with ICitizenAttributeS {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UserFunctionS;
/*
case object UserFunctionS extends IFunctionAttributeS // Whether it was written by a human. Mostly for tests right now.
*/

#[derive(Debug, PartialEq)]
pub enum ICitizenS<'s> {
  Struct(StructS<'s>),
  Interface(InterfaceS<'s>),
}
/*
sealed trait ICitizenS {
  def name: ICitizenDeclarationNameS
  def tyype: TemplateTemplataType
  def genericParams: Vector[GenericParameterS]
}
*/

impl<'s> ICitizenS<'s> {
  pub fn name(&self) -> TopLevelCitizenDeclarationNameS<'_> {
    match self {
      ICitizenS::Struct(s) => TopLevelCitizenDeclarationNameS::from(s.name),
      ICitizenS::Interface(i) => TopLevelCitizenDeclarationNameS::from(i.name),
    }
  }
  /* Guardian: disable-all */

  pub fn tyype(&self) -> &TemplateTemplataType<'s> {
    match self {
      ICitizenS::Struct(s) => &s.tyype,
      ICitizenS::Interface(i) => &i.tyype,
    }
  }
  /* Guardian: disable-all */

  pub fn generic_params(&self) -> &'s [&'s GenericParameterS<'s>] {
    match self {
      ICitizenS::Struct(s) => s.generic_params,
      ICitizenS::Interface(i) => i.generic_params,
    }
  }
  /* Guardian: disable-all */
}
/* Guardian: disable-all */

#[derive(Debug, PartialEq)]
pub struct StructS<'s> {
  pub range: RangeS<'s>,
  pub name: &'s TopLevelStructDeclarationNameS<'s>,
  pub attributes: &'s [ICitizenAttributeS<'s>],
  pub weakable: bool,
  pub generic_params: &'s [&'s GenericParameterS<'s>],
  pub mutability_rune: RuneUsage<'s>,
  pub maybe_predicted_mutability: Option<MutabilityP>,
  pub tyype: TemplateTemplataType<'s>,
  pub header_rune_to_explicit_type: ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>,
  pub header_predicted_rune_to_type: ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>,
  pub header_rules: &'s [IRulexSR<'s>],
  pub members_rune_to_explicit_type: ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>,
  pub members_predicted_rune_to_type: ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>,
  pub member_rules: &'s [IRulexSR<'s>],
  pub members: &'s [IStructMemberS<'s>],
  pub internal_methods: &'s [&'s FunctionS<'s>],
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

    members: Vector[IStructMemberS],
    internalMethods: Vector[FunctionS]
) extends ICitizenS {
*/
impl<'s> StructS<'s> {
  pub fn new(
    range: RangeS<'s>,
    name: &'s TopLevelStructDeclarationNameS<'s>,
    attributes: &'s [ICitizenAttributeS<'s>],
    weakable: bool,
    generic_params: &'s [&'s GenericParameterS<'s>],
    mutability_rune: RuneUsage<'s>,
    maybe_predicted_mutability: Option<MutabilityP>,
    tyype: TemplateTemplataType<'s>,
    header_rune_to_explicit_type: ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>,
    header_predicted_rune_to_type: ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>,
    header_rules: &'s [IRulexSR<'s>],
    members_rune_to_explicit_type: ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>,
    members_predicted_rune_to_type: ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>,
    member_rules: &'s [IRulexSR<'s>],
    members: &'s [IStructMemberS<'s>],
    internal_methods: &'s [&'s FunctionS<'s>],
  ) -> Self {
    assert!(
      !generic_params.iter().any(|x| matches!(x.rune.rune, IRuneS::DenizenDefaultRegionRune(_))),
      "vassert: generic_params should not contain DenizenDefaultRegionRuneS"
    );
    assert!(
      !header_rune_to_explicit_type.keys().chain(header_predicted_rune_to_type.keys())
        .chain(members_rune_to_explicit_type.keys()).chain(members_predicted_rune_to_type.keys())
        .any(|rune| matches!(rune, IRuneS::DenizenDefaultRegionRune(_))),
      "vassert: rune-to-type maps should not contain DenizenDefaultRegionRuneS"
    );
    Self {
      range, name, attributes, weakable, generic_params, mutability_rune,
      maybe_predicted_mutability, tyype, header_rune_to_explicit_type,
      header_predicted_rune_to_type, header_rules, members_rune_to_explicit_type,
      members_predicted_rune_to_type, member_rules, members, internal_methods,
    }
  }
}
/*
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

  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()

//  vassert(isTemplate == identifyingRunes.nonEmpty)
}
*/
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IStructMemberS<'s> {
  NormalStructMember(NormalStructMemberS<'s>),
  VariadicStructMember(VariadicStructMemberS<'s>),
}

impl<'s> IStructMemberS<'s> {
  pub fn range(&self) -> RangeS<'_> {
    match self {
      IStructMemberS::NormalStructMember(m) => m.range.clone(),
      IStructMemberS::VariadicStructMember(m) => m.range.clone(),
    }
  }
  /* Guardian: disable-all */

  pub fn variability(&self) -> VariabilityP {
    match self {
      IStructMemberS::NormalStructMember(m) => m.variability,
      IStructMemberS::VariadicStructMember(m) => m.variability,
    }
  }
  /* Guardian: disable-all */

  pub fn type_rune(&self) -> &RuneUsage<'s> {
    match self {
      IStructMemberS::NormalStructMember(m) => &m.type_rune,
      IStructMemberS::VariadicStructMember(m) => &m.type_rune,
    }
  }
  /* Guardian: disable-all */
}
/* Guardian: disable-all */

/*
sealed trait IStructMemberS {
  def range: RangeS
  def variability: VariabilityP
  def typeRune: RuneUsage
}
  */
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NormalStructMemberS<'s> {
  pub range: RangeS<'s>,
  pub name: StrI<'s>,
  pub variability: VariabilityP,
  pub type_rune: RuneUsage<'s>,
}

/*
case class NormalStructMemberS(
    range: RangeS,
    name: StrI,
    variability: VariabilityP,
    typeRune: RuneUsage) extends IStructMemberS {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
  */
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VariadicStructMemberS<'s> {
  pub range: RangeS<'s>,
  pub variability: VariabilityP,
  pub type_rune: RuneUsage<'s>,
}

/*
case class VariadicStructMemberS(
  range: RangeS,
  variability: VariabilityP,
  typeRune: RuneUsage) extends IStructMemberS {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct InterfaceS<'s> {
  pub range: RangeS<'s>,
  pub name: &'s TopLevelInterfaceDeclarationNameS<'s>,
  pub attributes: &'s [ICitizenAttributeS<'s>],
  pub weakable: bool,
  pub generic_params: &'s [&'s GenericParameterS<'s>],
  pub rune_to_explicit_type: ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>,
  pub mutability_rune: RuneUsage<'s>,
  pub maybe_predicted_mutability: Option<MutabilityP>,
  pub predicted_rune_to_type: ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>,
  pub tyype: TemplateTemplataType<'s>,
  pub rules: &'s [IRulexSR<'s>],
  pub internal_methods: &'s [&'s FunctionS<'s>],
}
impl<'s> InterfaceS<'s> {
  pub fn new(
    range: RangeS<'s>,
    name: &'s TopLevelInterfaceDeclarationNameS<'s>,
    attributes: &'s [ICitizenAttributeS<'s>],
    weakable: bool,
    generic_params: &'s [&'s GenericParameterS<'s>],
    rune_to_explicit_type: ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>,
    mutability_rune: RuneUsage<'s>,
    maybe_predicted_mutability: Option<MutabilityP>,
    predicted_rune_to_type: ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>,
    tyype: TemplateTemplataType<'s>,
    rules: &'s [IRulexSR<'s>],
    internal_methods: &'s [&'s FunctionS<'s>],
  ) -> Self {
    assert!(
      !generic_params.iter().any(|x| matches!(x.rune.rune, IRuneS::DenizenDefaultRegionRune(_))),
      "vassert: generic_params should not contain DenizenDefaultRegionRuneS"
    );
    assert!(
      !rune_to_explicit_type.keys().chain(predicted_rune_to_type.keys())
        .any(|rune| matches!(rune, IRuneS::DenizenDefaultRegionRune(_))),
      "vassert: rune-to-type maps should not contain DenizenDefaultRegionRuneS"
    );
    for internal_method in internal_methods {
      assert!(
        generic_params == internal_method.generic_params,
        "vassert: genericParams == internalMethod.genericParams"
      );
    }
    Self {
      range, name, attributes, weakable, generic_params, rune_to_explicit_type,
      mutability_rune, maybe_predicted_mutability, predicted_rune_to_type,
      tyype, rules, internal_methods,
    }
  }
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

  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()

  internalMethods.foreach(internalMethod => {
    vregionmut() // Put this back in when we have regions
    // // .init because every method has a default region as the last region param.
    // vassert(genericParams == internalMethod.genericParams.init)
    // Take this out when we have regions
    vassert(genericParams == internalMethod.genericParams)
  })

}
*/
#[derive(Debug, PartialEq)]
pub struct ImplS<'s> {
  pub range: RangeS<'s>,
  pub name: ImplDeclarationNameS<'s>,
  pub user_specified_identifying_runes: &'s [&'s GenericParameterS<'s>],
  pub rules: &'s [IRulexSR<'s>],
  pub rune_to_explicit_type: ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>,
  pub tyype: ITemplataType<'s>,
  pub struct_kind_rune: RuneUsage<'s>,
  pub sub_citizen_imprecise_name: IImpreciseNameS<'s>,
  pub interface_kind_rune: RuneUsage<'s>,
  pub super_interface_imprecise_name: IImpreciseNameS<'s>,
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
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct ExportAsS<'s> {
  pub range: RangeS<'s>,
  pub rules: &'s [IRulexSR<'s>],
  pub export_name: ExportAsNameS<'s>,
  pub rune: RuneUsage<'s>,
  pub exported_name: StrI<'s>,
}

/*
case class ExportAsS(
  range: RangeS,
  rules: Vector[IRulexSR],
  exportName: ExportAsNameS,
  rune: RuneUsage,
  exportedName: StrI) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
#[derive(Debug, PartialEq)]
pub struct ImportS<'s> {
  pub range: RangeS<'s>,
  pub module_name: StrI<'s>,
  pub package_names: &'s [StrI<'s>],
  pub importee_name: StrI<'s>,
}

/*
case class ImportS(
  range: RangeS,
  moduleName: StrI,
  packageNames: Vector[StrI],
  importeeName: StrI) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/
pub fn interface_s_name<'s>(interface_s: &InterfaceS<'s>) -> TopLevelCitizenDeclarationNameS<'s> {
  TopLevelCitizenDeclarationNameS::from(interface_s.name)
}

/*
object interfaceSName {
  // The extraction method (mandatory)
  def unapply(interfaceS: InterfaceS): Option[TopLevelCitizenDeclarationNameS] = {
    Some(interfaceS.name)
  }
}
*/
pub fn struct_s_name<'s>(struct_s: &StructS<'s>) -> TopLevelCitizenDeclarationNameS<'s> {
  TopLevelCitizenDeclarationNameS::from(struct_s.name)
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
#[derive(Debug, PartialEq)]
pub struct ParameterS<'s> {
  pub range: RangeS<'s>,
  pub virtuality: Option<AbstractSP<'s>>,
  pub pre_checked: bool,
  pub pattern: AtomSP<'s>,
}
impl<'s> ParameterS<'s> {
  pub fn new(range: RangeS<'s>, virtuality: Option<AbstractSP<'s>>, pre_checked: bool, pattern: AtomSP<'s>) -> Self {
    assert!(pattern.coord_rune.is_some(), "vassert: pattern.coordRune.nonEmpty");
    Self { range, virtuality, pre_checked, pattern }
  }
}
/*
case class ParameterS(
  range: RangeS,
  virtuality: Option[AbstractSP],
  preChecked: Boolean,
  pattern: AtomSP) {

  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()

  vassert(pattern.coordRune.nonEmpty)
}
*/
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AbstractSP<'s> {
  pub range: RangeS<'s>,
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
#[derive(Debug, PartialEq)]
pub struct SimpleParameterS<'s> {
  pub origin: Option<AtomSP<'s>>,
  pub name: StrI<'s>,
  pub virtuality: Option<AbstractSP<'s>>,
  pub tyype: IRulexSR<'s>,
}
/*
case class SimpleParameterS(
    origin: Option[AtomSP],
    name: String,
    virtuality: Option[AbstractSP],
    tyype: IRulexSR) {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IBodyS<'s> {
  ExternBody(ExternBodyS),
  AbstractBody(AbstractBodyS),
  GeneratedBody(GeneratedBodyS<'s>),
  CodeBody(CodeBodyS<'s>),
}

/*
sealed trait IBodyS
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExternBodyS {}
/*
case object ExternBodyS extends IBodyS
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AbstractBodyS {}
/*
case object AbstractBodyS extends IBodyS
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GeneratedBodyS<'s> {
  pub generator_id: StrI<'s>,
}
/*
case class GeneratedBodyS(generatorId: StrI) extends IBodyS {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
}
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CodeBodyS<'s> {
  pub body: &'s BodySE<'s>,
}
/*
case class CodeBodyS(body: BodySE) extends IBodyS {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
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
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IGenericParameterTypeS<'s> {
  RegionGenericParameterType(RegionGenericParameterTypeS),
  CoordGenericParameterType(CoordGenericParameterTypeS<'s>),
  OtherGenericParameterType(OtherGenericParameterTypeS<'s>),
}
/*
object IGenericParameterTypeS {
*/

impl<'s> IGenericParameterTypeS<'s> {
  pub fn expect_region(&self) -> &RegionGenericParameterTypeS {
    match self {
      IGenericParameterTypeS::RegionGenericParameterType(x) => x,
      _ => panic!("Expected region generic parameter type"),
    }
  }
  /*
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
  */

  pub fn tyype(&self) -> ITemplataType<'s> {
    match self {
      IGenericParameterTypeS::RegionGenericParameterType(x) => x.tyype(),
      IGenericParameterTypeS::CoordGenericParameterType(x) => x.tyype(),
      IGenericParameterTypeS::OtherGenericParameterType(x) => x.tyype.clone(),
    }
  }
  /*
    def tyype: ITemplataType
    */
}
/*
Guardian: disable-all
}
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RegionGenericParameterTypeS {
  pub mutability: IRegionMutabilityS,
}
/*
case class RegionGenericParameterTypeS(mutability: IRegionMutabilityS) extends IGenericParameterTypeS {
  def tyype: ITemplataType = RegionTemplataType()
}
*/

impl RegionGenericParameterTypeS {
  pub fn tyype<'a>(&self) -> ITemplataType<'a> {
    ITemplataType::RegionTemplataType(RegionTemplataType {})
  }
  /* Guardian: disable-all */
}
/* Guardian: disable-all */

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CoordGenericParameterTypeS<'s> {
  pub coord_region: Option<RuneUsage<'s>>,
  pub kind_mutable: bool,
  pub region_mutable: bool,
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

impl CoordGenericParameterTypeS<'_> {
  pub fn tyype<'a>(&self) -> ITemplataType<'a> {
    assert!(self.coord_region.is_none());
    ITemplataType::CoordTemplataType(CoordTemplataType {})
  }
  /* Guardian: disable-all */
}
/* Guardian: disable-all */

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OtherGenericParameterTypeS<'s> {
  pub tyype: ITemplataType<'s>,
}
impl<'s> OtherGenericParameterTypeS<'s> {
  pub fn new(tyype: ITemplataType<'s>) -> Self {
    assert!(
      !matches!(tyype, ITemplataType::RegionTemplataType(_) | ITemplataType::CoordTemplataType(_)),
      "vwat: Use RegionGenericParameterTypeS or CoordGenericParameterTypeS for these types"
    );
    Self { tyype }
  }
}
/*
case class OtherGenericParameterTypeS(tyype: ITemplataType) extends IGenericParameterTypeS {
  tyype match {
    case RegionTemplataType() | CoordTemplataType() => vwat() // Use other types for this
    case _ =>
  }
}
*/

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GenericParameterS<'s> {
  pub range: RangeS<'s>,
  pub rune: RuneUsage<'s>,
  pub tyype: IGenericParameterTypeS<'s>,
  pub default: Option<GenericParameterDefaultS<'s>>,
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GenericParameterDefaultS<'s> {
  pub result_rune: IRuneS<'s>,
  pub rules: &'s [&'s IRulexSR<'s>],
  pub rune_to_type: &'s [(IRuneS<'s>, ITemplataType<'s>)],
}
/*
// Per @DRSINI, these rules are added incrementally (not in the initial rule set) by
// solveForResolving and evaluateGenericFunctionFromCallForPrototype for unsolved runes.
// `rules` includes the connecting EqualsSR(paramRune, resultRune) so the default is fully
// self-contained — it travels intact when GenericParameterS is inherited (e.g. by struct
// internal methods). `runeToType` carries types for default-only runes (currently just
// resultRune); these get registered into the solver at default-fire time.
// DO NOT SUBMIT is this true?
case class GenericParameterDefaultS(
  resultRune: IRuneS,
  rules: Vector[IRulexSR],
  runeToType: Map[IRuneS, ITemplataType])
*/
#[derive(Debug, PartialEq)]
pub struct FunctionS<'s> {
  pub range: RangeS<'s>,
  pub name: &'s IFunctionDeclarationNameS<'s>,
  pub attributes: &'s [IFunctionAttributeS<'s>],
  pub generic_params: &'s [&'s GenericParameterS<'s>],
  pub rune_to_predicted_type: ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>,
  pub tyype: TemplateTemplataType<'s>,
  pub params: &'s [ParameterS<'s>],
  pub maybe_ret_coord_rune: Option<RuneUsage<'s>>,
  pub rules: &'s [IRulexSR<'s>],
  pub body: &'s IBodyS<'s>,
}
impl<'s> FunctionS<'s> {
  pub fn new(
    range: RangeS<'s>,
    name: &'s IFunctionDeclarationNameS<'s>,
    attributes: &'s [IFunctionAttributeS<'s>],
    generic_params: &'s [&'s GenericParameterS<'s>],
    rune_to_predicted_type: ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>>,
    tyype: TemplateTemplataType<'s>,
    params: &'s [ParameterS<'s>],
    maybe_ret_coord_rune: Option<RuneUsage<'s>>,
    rules: &'s [IRulexSR<'s>],
    body: &'s IBodyS<'s>,
  ) -> Self {
    assert!(
      !generic_params.iter().any(|x| matches!(x.rune.rune, IRuneS::DenizenDefaultRegionRune(_))),
      "vassert: generic_params should not contain DenizenDefaultRegionRuneS"
    );
    assert!(
      !rune_to_predicted_type.keys().any(|rune| matches!(rune, IRuneS::DenizenDefaultRegionRune(_))),
      "vassert: rune_to_predicted_type should not contain DenizenDefaultRegionRuneS"
    );
    match body {
      IBodyS::ExternBody(_) | IBodyS::AbstractBody(_) | IBodyS::GeneratedBody(_) => {
        assert!(
          !matches!(name, IFunctionDeclarationNameS::LambdaDeclarationName(_)),
          "vwat: extern/abstract/generated body must not be lambda"
        );
      }
      IBodyS::CodeBody(code_body) => {
        if !code_body.body.closured_names.is_empty() {
          assert!(
            matches!(name, IFunctionDeclarationNameS::LambdaDeclarationName(_)),
            "vwat: closured code body must be lambda"
          );
        }
      }
    }
    Self {
      range, name, attributes, generic_params, rune_to_predicted_type,
      tyype, params, maybe_ret_coord_rune, rules, body,
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

  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
*/
impl<'s> FunctionS<'s> {
  pub fn is_light(&self) -> bool {
    match &self.body {
      IBodyS::ExternBody(_) | IBodyS::AbstractBody(_) | IBodyS::GeneratedBody(_) => false,
      IBodyS::CodeBody(body) => !body.body.closured_names.is_empty(),
    }
  }
  /*
    def isLight(): Boolean = {
      body match {
        case ExternBodyS | AbstractBodyS | GeneratedBodyS(_) => false
        case CodeBodyS(bodyS) => bodyS.closuredNames.nonEmpty
      }
    }
    */
}
/*
Guardian: disable-all
}
*/

#[derive(Debug, PartialEq)]
pub struct LocationInDenizenBuilder {
  path: Vec<i32>,
  consumed: bool,
  next_child: i32,
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
  val hash = runtime.ScalaRunTime._hashCode(path.toList);
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious();
*/

impl LocationInDenizenBuilder {
  // MIGALLOW: new -> new
  pub fn new(path: Vec<i32>) -> Self {
    Self {
      path,
      consumed: false,
      next_child: 1,
    }
  }
  /* Guardian: disable-all */

  pub fn child(&mut self) -> LocationInDenizenBuilder {
    let child = self.next_child;
    self.next_child += 1;
    let mut child_path = self.path.clone();
    child_path.push(child);
    LocationInDenizenBuilder::new(child_path)
  }
  /*
    def child(): LocationInDenizenBuilder = {
      val child = nextChild
      nextChild = nextChild + 1
      new LocationInDenizenBuilder(path :+ child)
    }
  */

  // Per @DSAUIMZ, this is for NON-interned uses only (expression AST nodes).
  pub fn consume_in<'x>(&mut self, arena: &'x bumpalo::Bump) -> LocationInDenizen<'x> {
    assert!(
      !self.consumed,
      "Location in denizen was already used for something, add a .child() somewhere."
    );
    self.consumed = true;
    LocationInDenizen {
      path: arena.alloc_slice_copy(&self.path),
    }
  }

  // Per @DSAUIMZ, this is for NON-interned uses only (expression AST nodes).
  // Takes a ScoutArena instead of raw Bump to avoid exposing the allocator.
  // V: this feels weird. theres nothing guaranteeing that this LocationInDenizen will actually land anywhere,
  // in which case we're just leaking those allocations. i think we need a LocationInDenizenVal.
  // maybe LocationInDenizenVal can even be a stack-based linked list.
  pub fn consume_in_arena<'x>(&mut self, arena: &ScoutArena<'x>) -> LocationInDenizen<'x> {
    assert!(
      !self.consumed,
      "Location in denizen was already used for something, add a .child() somewhere."
    );
    self.consumed = true;
    LocationInDenizen {
      path: arena.alloc_slice_copy(&self.path),
    }
  }
  /*
    def consume(): LocationInDenizen = {
      assert(!consumed, "Location in denizen was already used for something, add a .child() somewhere.")
      consumed = true
      LocationInDenizen(path)
    }
  */

  // Per @DSAUIMZ, this is the only way to construct a LocationInDenizenVal.
  // Borrows from the builder's Vec, so 'tmp is a stack lifetime, not 's.
  pub fn borrow_val(&mut self) -> LocationInDenizenVal<'_> {
    assert!(
      !self.consumed,
      "Location in denizen was already used for something, add a .child() somewhere."
    );
    self.consumed = true;
    LocationInDenizenVal { path: &self.path }
  }
}
/*
  override def toString: String = path.mkString(".")
}
*/

/// A path identifying a specific location within a denizen (function, struct, etc.).
/// Each element in the path is a child index, forming a tree address.
///
/// Parameterized on lifetime `'x` because LocationInDenizen lives in different
/// arenas depending on its owner:
/// - When inside rune structs (e.g. ImplicitRuneS), it's interned into the
///   `'s` interner arena, so `'x = 's`.
/// - When inside expression structs (e.g. PureSE, FunctionSE), it's allocated
///   in the `'s` scout arena, so `'x = 's`.
///
/// The path is an arena-allocated slice rather than a Vec so that the entire
/// struct can live in an arena without heap pointers.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocationInDenizen<'x> {
  pub path: &'x [i32],
}

/*
case class LocationInDenizen(path: Vector[Int]) {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def equals(obj: Any): Boolean = {
    obj match {
      case LocationInDenizen(thatPath) => path == thatPath
      case _ => false
    }
  }
*/

/// Borrowed view of a LocationInDenizen path, for use as an intern lookup key.
/// Per @DSAUIMZ, fields are private to prevent pre-allocation.
/// Only constructible via LocationInDenizenBuilder::borrow_val().
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocationInDenizenVal<'tmp> {
  path: &'tmp [i32],
}

impl<'tmp> LocationInDenizenVal<'tmp> {
  /// Read access to path contents (for Hash/Eq/Debug implementations).
  pub fn path(&self) -> &[i32] {
    self.path
  }

  /// Per @DSAUIMZ, only called inside intern methods on a miss.
  pub(crate) fn promote_in<'s>(&self, arena: &'s bumpalo::Bump) -> LocationInDenizen<'s> {
    LocationInDenizen { path: arena.alloc_slice_copy(self.path) }
  }

  /// Per @DSAUIMZ, only used inside intern methods to construct stored HashMap keys
  /// from a just-promoted LocationInDenizen.
  pub(crate) fn from_canonical<'s>(lid: &LocationInDenizen<'s>) -> LocationInDenizenVal<'s> {
    LocationInDenizenVal { path: lid.path }
  }
}

impl<'x> LocationInDenizen<'x> {
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

}
/*
Guardian: disable-all
*/

#[derive(Debug, PartialEq)]
pub enum IDenizenS<'s> {
  TopLevelFunction(TopLevelFunctionS<'s>),
  TopLevelImpl(TopLevelImplS<'s>),
  TopLevelExportAs(TopLevelExportAsS<'s>),
  TopLevelImport(TopLevelImportS<'s>),
  TopLevelStruct(TopLevelStructS<'s>),
  TopLevelInterface(TopLevelInterfaceS<'s>),
}
/*
sealed trait IDenizenS
*/

#[derive(Debug, PartialEq)]
pub struct TopLevelFunctionS<'s> {
  pub function: FunctionS<'s>,
}

/*
case class TopLevelFunctionS(function: FunctionS) extends IDenizenS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/

#[derive(Debug, PartialEq)]
pub struct TopLevelImplS<'s> {
  pub impl_: ImplS<'s>,
}
/*
case class TopLevelImplS(impl: ImplS) extends IDenizenS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/

#[derive(Debug, PartialEq)]
pub struct TopLevelExportAsS<'s> {
  pub export: ExportAsS<'s>,
}
/*
case class TopLevelExportAsS(export: ExportAsS) extends IDenizenS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/

#[derive(Debug, PartialEq)]
pub struct TopLevelImportS<'s> {
  pub imporrt: ImportS<'s>,
}
/*
case class TopLevelImportS(imporrt: ImportS) extends IDenizenS { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
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

#[derive(Debug, PartialEq)]
pub enum ICitizenDenizenS<'s> {
  TopLevelStruct(TopLevelStructS<'s>),
  TopLevelInterface(TopLevelInterfaceS<'s>),
}
/*
sealed trait ICitizenDenizenS extends IDenizenS {
*/

impl<'s> ICitizenDenizenS<'s> {
  pub fn citizen(&self) -> ! {
    panic!("ICitizenDenizenS::citizen is dead code")
  }
  /*
    def citizen: ICitizenS
  }
  */
}
/*
Guardian: disable-all
*/

// MIGALLOW: unapply -> as_citizen_denizen
pub fn as_citizen_denizen<'s>(_x: &IDenizenS<'s>) -> Option<ICitizenDenizenS<'s>> {
  panic!("as_citizen_denizen is dead code")
}
/* Guardian: disable-all */


#[derive(Debug, PartialEq)]
pub struct TopLevelStructS<'s> {
  pub strukt: StructS<'s>,
}
/*
case class TopLevelStructS(struct: StructS) extends ICitizenDenizenS {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  override def citizen: ICitizenS = struct
}
*/

#[derive(Debug, PartialEq)]
pub struct TopLevelInterfaceS<'s> {
  pub interface: InterfaceS<'s>,
}
/*
case class TopLevelInterfaceS(interface: InterfaceS) extends ICitizenDenizenS {
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  override def citizen: ICitizenS = interface
}
*/

#[derive(Debug, PartialEq)]
pub struct FileS<'s> {
  pub denizens: Vec<IDenizenS<'s>>,
}
/*
case class FileS(denizens: Vector[IDenizenS])
*/