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
// mig: struct ProgramA
pub struct ProgramA {
    pub structs: Vec<StructA>,
    pub interfaces: Vec<InterfaceA>,
    pub impls: Vec<ImplA>,
    pub functions: Vec<FunctionA>,
    pub exports: Vec<ExportAsA>,
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
impl ProgramA {
/*
*/
// mig: fn equals
pub fn equals(&self, obj: &dyn std::any::Any) -> bool {
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
// mig: fn lookup_function
pub fn lookup_function(&self, name: INameS) -> FunctionA {
    panic!("Unimplemented: lookup_function");
}
/*
  def lookupFunction(name: INameS) = {
    val matches = functions.filter(_.name == name)
    vassert(matches.size == 1)
    matches.head
  }
*/
// mig: fn lookup_function
pub fn lookup_function(&self, name: String) -> FunctionA {
    panic!("Unimplemented: lookup_function");
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
pub fn lookup_interface(&self, name: INameS) -> InterfaceA {
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
// mig: fn lookup_struct
pub fn lookup_struct(&self, name: INameS) -> StructA {
    panic!("Unimplemented: lookup_struct");
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
// mig: fn lookup_struct
pub fn lookup_struct(&self, name: String) -> StructA {
    panic!("Unimplemented: lookup_struct");
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
pub struct StructA {
    pub range: RangeS,
    pub name: IStructDeclarationNameS,
    pub attributes: Vec<ICitizenAttributeS>,
    pub weakable: bool,
    pub mutability_rune: RuneUsage,
    pub maybe_predicted_mutability: Option<MutabilityP>,
    pub tyype: TemplateTemplataType,
    pub generic_parameters: Vec<GenericParameterS>,
    pub header_rune_to_type: HashMap<IRuneS, ITemplataType>,
    pub header_rules: Vec<IRulexSR>,
    pub members_rune_to_type: HashMap<IRuneS, ITemplataType>,
    pub member_rules: Vec<IRulexSR>,
    pub members: Vec<IStructMemberS>,
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
impl StructA {
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
pub fn equals(&self, obj: &dyn std::any::Any) -> bool {
    panic!("Unimplemented: equals");
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
pub struct ImplA {
    pub range: RangeS,
    pub name: IImplDeclarationNameS,
    pub generic_params: Vec<GenericParameterS>,
    pub rules: Vec<IRulexSR>,
    pub rune_to_type: HashMap<IRuneS, ITemplataType>,
    pub sub_citizen_rune: RuneUsage,
    pub sub_citizen_imprecise_name: IImpreciseNameS,
    pub interface_kind_rune: RuneUsage,
    pub super_interface_imprecise_name: IImpreciseNameS,
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
impl ImplA {
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
pub fn equals(&self, obj: &dyn std::any::Any) -> bool {
    panic!("Unimplemented: equals");
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
pub struct ExportAsA {
    pub range: RangeS,
    pub exported_name: StrI,
    pub rules: Vec<IRulexSR>,
    pub rune_to_type: HashMap<IRuneS, ITemplataType>,
    pub type_rune: RuneUsage,
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
impl ExportAsA {
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
pub fn equals(&self, obj: &dyn std::any::Any) -> bool {
    panic!("Unimplemented: equals");
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
pub trait CitizenA {
/*
sealed trait CitizenA {
*/
// mig: fn tyype
fn tyype(&self) -> TemplateTemplataType;
/*
  def tyype: TemplateTemplataType
*/
// mig: fn generic_parameters
fn generic_parameters(&self) -> Vec<GenericParameterS>;
/*
  def genericParameters: Vector[GenericParameterS]
*/
}
/*
}
*/
// mig: struct InterfaceA
pub struct InterfaceA {
    pub range: RangeS,
    pub name: TopLevelInterfaceDeclarationNameS,
    pub attributes: Vec<ICitizenAttributeS>,
    pub weakable: bool,
    pub mutability_rune: RuneUsage,
    pub maybe_predicted_mutability: Option<MutabilityP>,
    pub tyype: TemplateTemplataType,
    pub generic_parameters: Vec<GenericParameterS>,
    pub rune_to_type: HashMap<IRuneS, ITemplataType>,
    pub rules: Vec<IRulexSR>,
    pub internal_methods: Vec<FunctionA>,
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
impl InterfaceA {
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
pub fn equals(&self, obj: &dyn std::any::Any) -> bool {
    panic!("Unimplemented: equals");
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
/*
object interfaceName {
*/
// mig: fn unapply
pub fn unapply(interface_a: InterfaceA) -> Option<INameS> {
    panic!("Unimplemented: unapply");
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
/*
object structName {
*/
// mig: fn unapply
pub fn unapply(struct_a: StructA) -> Option<INameS> {
    panic!("Unimplemented: unapply");
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
pub struct FunctionA {
    pub range: RangeS,
    pub name: IFunctionDeclarationNameS,
    pub attributes: Vec<IFunctionAttributeS>,
    pub tyype: TemplateTemplataType,
    pub generic_parameters: Vec<GenericParameterS>,
    pub rune_to_type: HashMap<IRuneS, ITemplataType>,
    pub params: Vec<ParameterS>,
    pub maybe_ret_coord_rune: Option<RuneUsage>,
    pub rules: Vec<IRulexSR>,
    pub body: IBodyS,
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
impl FunctionA {
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
pub fn equals(&self, obj: &dyn std::any::Any) -> bool {
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
