use crate::interner::StrI;
use crate::lexing::RangeL;
use std::sync::Arc;
use crate::utils::code_hierarchy::{FileCoordinate};
use super::templex::{ITemplexPT, RegionRunePT};
use super::expressions::BlockPE;
use super::pattern::{ParameterP};
use super::rules::{IRulexPR, ITypePR};
/*
package dev.vale.parsing.ast

import dev.vale.lexing.{RangeL, WordLE}
import dev.vale.{FileCoordinate, StrI, vassert, vcurious, vpass}
*/

/// Something that exists in the source code. An Option[UnitP] is better than a boolean
/// because it also contains the range it was found.
#[derive(Clone, Debug, PartialEq)]
pub struct UnitP {
    pub range: RangeL,
}
/*
// Something that exists in the source code. An Option[UnitP] is better than a boolean
// because it also contains the range it was found.
case class UnitP(range: RangeL) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

/// Name in source code
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameP {
    pub range: RangeL,
    pub str: Arc<StrI>,
}
/*
case class NameP(range: RangeL, str: StrI) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

/// Parsed file
#[derive(Clone, Debug, PartialEq)]
pub struct FileP {
    pub file_coord: Arc<FileCoordinate>,
    pub comments_ranges: Vec<RangeL>,
    pub denizens: Vec<IDenizenP>,
}
/*
case class FileP(
  fileCoord: FileCoordinate,
  commentsRanges: Vector[RangeL],
  denizens: Vector[IDenizenP]) {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
  def lookupFunction(name: String) = {
    val results =
      denizens.collect({
        case TopLevelFunctionP(f) if f.header.name.exists(_.str.str == name) => f
      })
    vassert(results.size == 1)
    results.head
  }
}
*/

#[derive(Clone, Debug, PartialEq)]
pub enum IDenizenP {
    TopLevelFunction(FunctionP),
    TopLevelStruct(StructP),
    TopLevelInterface(InterfaceP),
    TopLevelImpl(ImplP),
    TopLevelExportAs(ExportAsP),
    TopLevelImport(ImportP),
}
/*
sealed trait IDenizenP
case class TopLevelFunctionP(function: FunctionP) extends IDenizenP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class TopLevelStructP(struct: StructP) extends IDenizenP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class TopLevelInterfaceP(interface: InterfaceP) extends IDenizenP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class TopLevelImplP(impl: ImplP) extends IDenizenP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class TopLevelExportAsP(export: ExportAsP) extends IDenizenP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class TopLevelImportP(imporrt: ImportP) extends IDenizenP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ImplP {
    pub range: RangeL,
    pub generic_params: Option<GenericParametersP>,
    pub template_rules: Option<TemplateRulesP>,
    // Option because we can say `impl MyInterface;` inside a struct.
    pub struct_: Option<ITemplexPT>,
    pub interface: ITemplexPT,
    pub attributes: Vec<IAttributeP>,
}
/*
case class ImplP(
  range: RangeL,
  genericParams: Option[GenericParametersP],
  templateRules: Option[TemplateRulesP],
  // Option because we can say `impl MyInterface;` inside a struct.
  struct: Option[ITemplexPT],
  interface: ITemplexPT,
  attributes: Vector[IAttributeP]
) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ExportAsP {
    pub range: RangeL,
    pub struct_: ITemplexPT,
    pub exported_name: NameP,
}
/*
case class ExportAsP(
  range: RangeL,
  struct: ITemplexPT,
  exportedName: NameP) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ImportP {
    pub range: RangeL,
    pub module_name: NameP,
    pub package_steps: Vec<NameP>,
    pub importee_name: NameP,
}
/*
case class ImportP(
  range: RangeL,
  moduleName: NameP,
  packageSteps: Vector[NameP],
  importeeName: NameP) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub enum IAttributeP {
    WeakableAttribute(RangeL),
    /*
    //sealed trait IAttributeP
    //case class ExportP(range: RangeP) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    case class WeakableAttributeP(range: RangeL) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    SealedAttribute(RangeL),
    /*
    case class SealedAttributeP(range: RangeL) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    MacroCall { range: RangeL, inclusion: IMacroInclusionP, name: NameP },
    /*
    case class MacroCallP(range: RangeL, inclusion: IMacroInclusionP, name: NameP) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
    */
    AbstractAttribute(RangeL),
    ExternAttribute(RangeL),
    BuiltinAttribute { range: RangeL, generator_name: NameP },
    ExportAttribute(RangeL),
    PureAttribute(RangeL),
    AdditiveAttribute(RangeL),
    LinearAttribute(RangeL),
}
/*
sealed trait IAttributeP
case class AbstractAttributeP(range: RangeL) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class ExternAttributeP(range: RangeL) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class BuiltinAttributeP(range: RangeL, generatorName: NameP) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class ExportAttributeP(range: RangeL) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class PureAttributeP(range: RangeL) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class AdditiveAttributeP(range: RangeL) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class LinearAttributeP(range: RangeL) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
//case class RuleAttributeP(rule: IRulexPR) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IMacroInclusionP {
    CallMacro,
    DontCallMacro,
}
/*
sealed trait IMacroInclusionP
case object CallMacroP extends IMacroInclusionP
case object DontCallMacroP extends IMacroInclusionP
*/

#[derive(Clone, Debug, PartialEq)]
pub enum IRuneAttributeP {
    ImmutableRuneAttribute(RangeL),
    MutableRuneAttribute(RangeL),
    ReadOnlyRegionRuneAttribute(RangeL),
    ReadWriteRegionRuneAttribute(RangeL),
    ImmutableRegionRuneAttribute(RangeL),
    AdditiveRegionRuneAttribute(RangeL),
    PoolRuneAttribute(RangeL),
    ArenaRuneAttribute(RangeL),
    BumpRuneAttribute(RangeL),
}
/*
sealed trait IRuneAttributeP {
  def range: RangeL
}
case class ImmutableRuneAttributeP(range: RangeL) extends IRuneAttributeP
case class MutableRuneAttributeP(range: RangeL) extends IRuneAttributeP
//case class TypeRuneAttributeP(range: RangeL, tyype: ITypePR) extends IRuneAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class ReadOnlyRegionRuneAttributeP(range: RangeL) extends IRuneAttributeP
case class ReadWriteRegionRuneAttributeP(range: RangeL) extends IRuneAttributeP
case class ImmutableRegionRuneAttributeP(range: RangeL) extends IRuneAttributeP
case class AdditiveRegionRuneAttributeP(range: RangeL) extends IRuneAttributeP
case class PoolRuneAttributeP(range: RangeL) extends IRuneAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class ArenaRuneAttributeP(range: RangeL) extends IRuneAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class BumpRuneAttributeP(range: RangeL) extends IRuneAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/


impl IRuneAttributeP {
    pub fn range(&self) -> RangeL {
        match self {
            IRuneAttributeP::ImmutableRuneAttribute(r) => *r,
            IRuneAttributeP::MutableRuneAttribute(r) => *r,
            IRuneAttributeP::ReadOnlyRegionRuneAttribute(r) => *r,
            IRuneAttributeP::ReadWriteRegionRuneAttribute(r) => *r,
            IRuneAttributeP::ImmutableRegionRuneAttribute(r) => *r,
            IRuneAttributeP::AdditiveRegionRuneAttribute(r) => *r,
            IRuneAttributeP::PoolRuneAttribute(r) => *r,
            IRuneAttributeP::ArenaRuneAttribute(r) => *r,
            IRuneAttributeP::BumpRuneAttribute(r) => *r,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructP {
    pub range: RangeL,
    pub name: NameP,
    pub attributes: Vec<IAttributeP>,
    pub mutability: Option<ITemplexPT>,
    pub identifying_runes: Option<GenericParametersP>,
    pub template_rules: Option<TemplateRulesP>,
    pub maybe_default_region_rune: Option<RegionRunePT>,
    pub body_range: RangeL,
    pub members: StructMembersP,
}
/*
case class StructP(
  range: RangeL,
  name: NameP,
  attributes: Vector[IAttributeP],
  mutability: Option[ITemplexPT],
  identifyingRunes: Option[GenericParametersP],
  templateRules: Option[TemplateRulesP],
  maybeDefaultRegionRuneP: Option[RegionRunePT],
  bodyRange: RangeL,
  members: StructMembersP) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct StructMembersP {
    pub range: RangeL,
    pub contents: Vec<IStructContent>,
}
/*
case class StructMembersP(
  range: RangeL,
  contents: Vector[IStructContent]) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub enum IStructContent {
    StructMethod(FunctionP),
    NormalStructMember(NormalStructMemberP),
    VariadicStructMember(VariadicStructMemberP),
}
#[derive(Clone, Debug, PartialEq)]
pub struct NormalStructMemberP {
    pub range: RangeL,
    pub name: NameP,
    pub variability: VariabilityP,
    pub tyype: ITemplexPT,
}
#[derive(Clone, Debug, PartialEq)]
pub struct VariadicStructMemberP {
    pub range: RangeL,
    pub variability: VariabilityP,
    pub tyype: ITemplexPT,
}
/*
sealed trait IStructContent
case class StructMethodP(func: FunctionP) extends IStructContent { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class NormalStructMemberP(
  range: RangeL,
  name: NameP,
  variability: VariabilityP,
  tyype: ITemplexPT
) extends IStructContent { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class VariadicStructMemberP(
  range: RangeL,
  variability: VariabilityP,
  tyype: ITemplexPT
) extends IStructContent { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceP {
    pub range: RangeL,
    pub name: NameP,
    pub attributes: Vec<IAttributeP>,
    pub mutability: Option<ITemplexPT>,
    pub maybe_identifying_runes: Option<GenericParametersP>,
    pub template_rules: Option<TemplateRulesP>,
    pub maybe_default_region_rune: Option<RegionRunePT>,
    pub body_range: RangeL,
    pub members: Vec<FunctionP>,
}
/*
case class InterfaceP(
  range: RangeL,
  name: NameP,
  attributes: Vector[IAttributeP],
  mutability: Option[ITemplexPT],
  maybeIdentifyingRunes: Option[GenericParametersP],
  templateRules: Option[TemplateRulesP],
  maybeDefaultRegionRuneP: Option[RegionRunePT],
  bodyRange: RangeL,
  members: Vector[FunctionP]) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionP {
    pub range: RangeL,
    pub header: FunctionHeaderP,
    pub body: Option<Box<BlockPE>>,
}
/*
case class FunctionP(
  range: RangeL,
  header: FunctionHeaderP,
  body: Option[BlockPE]) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionHeaderP {
    pub range: RangeL,
    pub name: Option<NameP>,
    pub attributes: Vec<IAttributeP>,
    // If Some(Vector.empty), should show up like the <> in func moo<>(a int, b bool)
    pub generic_parameters: Option<GenericParametersP>,
    pub template_rules: Option<TemplateRulesP>,
    pub params: Option<ParamsP>,
    pub ret: FunctionReturnP,
}
/*
case class FunctionHeaderP(
  range: RangeL,
  name: Option[NameP],
  attributes: Vector[IAttributeP],

  // If Some(Vector.empty), should show up like the <> in func moo<>(a int, b bool)
  genericParameters: Option[GenericParametersP],
  templateRules: Option[TemplateRulesP],

  params: Option[ParamsP],
  ret: FunctionReturnP
) {
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionReturnP {
    pub range: RangeL,
    pub ret_type: Option<ITemplexPT>,
}
/*
case class FunctionReturnP(
  range: RangeL,
  retType: Option[ITemplexPT]
) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct GenericParameterP {
    pub range: RangeL,
    pub name: NameP,
    pub maybe_type: Option<GenericParameterTypeP>,
    pub coord_region: Option<RegionRunePT>,
    pub attributes: Vec<IRuneAttributeP>,
    pub maybe_default: Option<ITemplexPT>,
}
/*
case class GenericParameterP(
  range: RangeL,
  name: NameP,
  maybeType: Option[GenericParameterTypeP],
  coordRegion: Option[RegionRunePT],
  attributes: Vector[IRuneAttributeP],
  maybeDefault: Option[ITemplexPT]
) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct GenericParameterTypeP {
    pub range: RangeL,
    pub tyype: ITypePR,
}
/*
case class GenericParameterTypeP(
  range: RangeL,
  tyype: ITypePR
)
*/

#[derive(Clone, Debug, PartialEq)]
pub struct GenericParametersP {
    pub range: RangeL,
    pub params: Vec<GenericParameterP>,
}
/*
case class GenericParametersP(range: RangeL, params: Vector[GenericParameterP]) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct TemplateRulesP {
    pub range: RangeL,
    pub rules: Vec<IRulexPR>,
}
/*
case class TemplateRulesP(range: RangeL, rules: Vector[IRulexPR]) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ParamsP {
    pub range: RangeL,
    pub params: Vec<ParameterP>,
}
/*
case class ParamsP(range: RangeL, params: Vector[ParameterP]) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MutabilityP {
    Mutable,
    Immutable,
}
/*
sealed trait MutabilityP
case object MutableP extends MutabilityP { override def toString: String = "mut" }
case object ImmutableP extends MutabilityP { override def toString: String = "imm" }
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VariabilityP {
    Final,
    Varying,
}
/*
sealed trait VariabilityP
case object FinalP extends VariabilityP { override def toString: String = "final" }
case object VaryingP extends VariabilityP { override def toString: String = "vary" }
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OwnershipP {
    Own,
    Borrow,
    Live,
    Weak,
    Share,
}
/*
sealed trait OwnershipP
case object OwnP extends OwnershipP { override def toString: String = "own" }
case object BorrowP extends OwnershipP { override def toString: String = "borrow" }
case object LiveP extends OwnershipP { override def toString: String = "live" }
case object WeakP extends OwnershipP { override def toString: String = "weak" }
case object ShareP extends OwnershipP { override def toString: String = "share" }
*/


/// This represents how to load something.
/// If something's a Share, then nothing will happen,
/// so this only applies to mutables.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadAsP {
    // This means we want to move it. Thisll become a OwnP or ShareP.
    Move,
    // This means we want to use it, and want to make sure that it doesn't drop.
    // If permission is None, then we're probably in a dot. For example, x.launch()
    // should be mapped to launch(&!x) if x is mutable, or launch(&x) if it's readonly.
    LoadAsBorrow,
    // This means we want to get a weak reference to it. Thisll become a WeakP.
    LoadAsWeak,
    // This represents unspecified. It basically means, use whatever ownership already there.
    Use,
}
/*
// This represents how to load something.
// If something's a Share, then nothing will happen,
// so this only applies to mutables.
sealed trait LoadAsP
// This means we want to move it. Thisll become a OwnP or ShareP.
case object MoveP extends LoadAsP
// This means we want to use it, and want to make sure that it doesn't drop.
// If permission is None, then we're probably in a dot. For example, x.launch()
// should be mapped to launch(&!x) if x is mutable, or launch(&x) if it's readonly.
case object LoadAsBorrowP extends LoadAsP { val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash; }
// This means we want to get a weak reference to it. Thisll become a WeakP.
case object LoadAsWeakP extends LoadAsP { val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash; }
// This represents unspecified. It basically means, use whatever ownership already there.
case object UseP extends LoadAsP
*/

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LocationP {
    Inline,
    Yonder,
}
/*
sealed trait LocationP
case object InlineP extends LocationP { override def toString: String = "inl" }
case object YonderP extends LocationP { override def toString: String = "heap" }
*/
