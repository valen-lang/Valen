use super::expressions::BlockPE;
use super::pattern::ParameterP;
use super::rules::{IRulexPR, ITypePR};
use super::templex::{ITemplexPT, RegionRunePT};
use crate::StrI;
use crate::lexing::RangeL;
use crate::utils::code_hierarchy::FileCoordinate;
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
Guardian: disable: NECX
*/

/// Name in source code
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameP<'a>(pub RangeL, pub StrI<'a>);

impl<'a> NameP<'a> {
  pub fn range(&self) -> RangeL {
    self.0
  }

  pub fn str(&self) -> StrI<'a> {
    self.1
  }

  /// Returns the underlying string slice.
  pub fn as_str(&self) -> &'a str {
    self.1.as_str()
  }
}
/*
case class NameP(range: RangeL, str: StrI) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

/// Parsed file
#[derive(Clone, Debug, PartialEq)]
pub struct FileP<'a, 'p> {
  pub file_coord: &'a FileCoordinate<'a>,
  pub comments_ranges: &'p [RangeL],
  pub denizens: &'p [IDenizenP<'a, 'p>],
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
        case TopLevelFunctionP(f) if f.header.name.exists(_.str == name) => f
      })
    vassert(results.size == 1)
    results.head
  }
Guardian: disable: NECX
}
*/

#[derive(Clone, Debug, PartialEq)]
pub enum IDenizenP<'a, 'p> {
  TopLevelFunction(FunctionP<'a, 'p>),
  TopLevelStruct(StructP<'a, 'p>),
  TopLevelInterface(InterfaceP<'a, 'p>),
  TopLevelImpl(ImplP<'a, 'p>),
  TopLevelExportAs(ExportAsP<'a, 'p>),
  TopLevelImport(ImportP<'a, 'p>),
}
/*
sealed trait IDenizenP
case class TopLevelFunctionP(function: FunctionP) extends IDenizenP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class TopLevelStructP(struct: StructP) extends IDenizenP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class TopLevelInterfaceP(interface: InterfaceP) extends IDenizenP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class TopLevelImplP(impl: ImplP) extends IDenizenP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class TopLevelExportAsP(export: ExportAsP) extends IDenizenP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
case class TopLevelImportP(imporrt: ImportP) extends IDenizenP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ImplP<'a, 'p> {
  pub range: RangeL,
  pub generic_params: Option<GenericParametersP<'a, 'p>>,
  pub template_rules: Option<TemplateRulesP<'a, 'p>>,
  // Option because we can say `impl MyInterface;` inside a struct.
  pub struct_: Option<ITemplexPT<'a, 'p>>,
  pub interface: ITemplexPT<'a, 'p>,
  pub attributes: &'p [IAttributeP<'a>],
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
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ExportAsP<'a, 'p> {
  pub range: RangeL,
  pub struct_: ITemplexPT<'a, 'p>,
  pub exported_name: NameP<'a>,
}
/*
case class ExportAsP(
  range: RangeL,
  struct: ITemplexPT,
  exportedName: NameP) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ImportP<'a, 'p> {
  pub range: RangeL,
  pub module_name: NameP<'a>,
  pub package_steps: &'p [NameP<'a>],
  pub importee_name: NameP<'a>,
}
/*
case class ImportP(
  range: RangeL,
  moduleName: NameP,
  packageSteps: Vector[NameP],
  importeeName: NameP) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct WeakableAttributeP {
  pub range: RangeL,
}
/*
case class WeakableAttributeP(range: RangeL) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct SealedAttributeP {
  pub range: RangeL,
}
/*
case class SealedAttributeP(range: RangeL) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct MacroCallP<'a> {
  pub range: RangeL,
  pub inclusion: IMacroInclusionP,
  pub name: NameP<'a>,
}
/*
case class MacroCallP(range: RangeL, inclusion: IMacroInclusionP, name: NameP) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct AbstractAttributeP {
  pub range: RangeL,
}
/*
case class AbstractAttributeP(range: RangeL) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ExternAttributeP {
  pub range: RangeL,
}
/*
case class ExternAttributeP(range: RangeL) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct BuiltinAttributeP<'a> {
  pub range: RangeL,
  pub generator_name: NameP<'a>,
}
/*
case class BuiltinAttributeP(range: RangeL, generatorName: NameP) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct ExportAttributeP {
  pub range: RangeL,
}
/*
case class ExportAttributeP(range: RangeL) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct PureAttributeP {
  pub range: RangeL,
}
/*
case class PureAttributeP(range: RangeL) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct AdditiveAttributeP {
  pub range: RangeL,
}
/*
case class AdditiveAttributeP(range: RangeL) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/
#[derive(Clone, Debug, PartialEq)]
pub struct LinearAttributeP {
  pub range: RangeL,
}
/*
case class LinearAttributeP(range: RangeL) extends IAttributeP { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub enum IAttributeP<'a> {
  WeakableAttribute(WeakableAttributeP),
  SealedAttribute(SealedAttributeP),
  MacroCall(MacroCallP<'a>),
  AbstractAttribute(AbstractAttributeP),
  ExternAttribute(ExternAttributeP),
  BuiltinAttribute(BuiltinAttributeP<'a>),
  ExportAttribute(ExportAttributeP),
  PureAttribute(PureAttributeP),
  AdditiveAttribute(AdditiveAttributeP),
  LinearAttribute(LinearAttributeP),
}
/*
sealed trait IAttributeP
Guardian: disable: NECX
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
Guardian: disable: NECX
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
Guardian: disable: NECX
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
pub struct StructP<'a, 'p> {
  pub range: RangeL,
  pub name: NameP<'a>,
  pub attributes: &'p [IAttributeP<'a>],
  pub mutability: Option<ITemplexPT<'a, 'p>>,
  pub identifying_runes: Option<GenericParametersP<'a, 'p>>,
  pub template_rules: Option<TemplateRulesP<'a, 'p>>,
  pub maybe_default_region_rune: Option<RegionRunePT<'a>>,
  pub body_range: RangeL,
  pub members: StructMembersP<'a, 'p>,
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
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct StructMembersP<'a, 'p> {
  pub range: RangeL,
  pub contents: &'p [IStructContent<'a, 'p>],
}
/*
case class StructMembersP(
  range: RangeL,
  contents: Vector[IStructContent]) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub enum IStructContent<'a, 'p> {
  StructMethod(FunctionP<'a, 'p>),
  NormalStructMember(NormalStructMemberP<'a, 'p>),
  VariadicStructMember(VariadicStructMemberP<'a, 'p>),
}
#[derive(Clone, Debug, PartialEq)]
pub struct NormalStructMemberP<'a, 'p> {
  pub range: RangeL,
  pub name: NameP<'a>,
  pub variability: VariabilityP,
  pub tyype: ITemplexPT<'a, 'p>,
}
#[derive(Clone, Debug, PartialEq)]
pub struct VariadicStructMemberP<'a, 'p> {
  pub range: RangeL,
  pub variability: VariabilityP,
  pub tyype: ITemplexPT<'a, 'p>,
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
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceP<'a, 'p> {
  pub range: RangeL,
  pub name: NameP<'a>,
  pub attributes: &'p [IAttributeP<'a>],
  pub mutability: Option<ITemplexPT<'a, 'p>>,
  pub maybe_identifying_runes: Option<GenericParametersP<'a, 'p>>,
  pub template_rules: Option<TemplateRulesP<'a, 'p>>,
  pub maybe_default_region_rune: Option<RegionRunePT<'a>>,
  pub body_range: RangeL,
  pub members: &'p [FunctionP<'a, 'p>],
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
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionP<'a, 'p> {
  pub range: RangeL,
  pub header: FunctionHeaderP<'a, 'p>,
  pub body: Option<&'p BlockPE<'a, 'p>>,
}
/*
case class FunctionP(
  range: RangeL,
  header: FunctionHeaderP,
  body: Option[BlockPE]) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionHeaderP<'a, 'p> {
  pub range: RangeL,
  pub name: Option<NameP<'a>>,
  pub attributes: &'p [IAttributeP<'a>],
  // If Some(Vector.empty), should show up like the <> in func moo<>(a int, b bool)
  pub generic_parameters: Option<GenericParametersP<'a, 'p>>,
  pub template_rules: Option<TemplateRulesP<'a, 'p>>,
  pub params: Option<ParamsP<'a, 'p>>,
  pub ret: FunctionReturnP<'a, 'p>,
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
Guardian: disable: NECX
}
*/
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionReturnP<'a, 'p> {
  pub range: RangeL,
  pub ret_type: Option<ITemplexPT<'a, 'p>>,
}
/*
case class FunctionReturnP(
  range: RangeL,
  retType: Option[ITemplexPT]
) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct GenericParameterP<'a, 'p> {
  pub range: RangeL,
  pub name: NameP<'a>,
  pub maybe_type: Option<GenericParameterTypeP>,
  pub coord_region: Option<RegionRunePT<'a>>,
  pub attributes: &'p [IRuneAttributeP],
  pub maybe_default: Option<ITemplexPT<'a, 'p>>,
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
Guardian: disable: NECX
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
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct GenericParametersP<'a, 'p> {
  pub range: RangeL,
  pub params: &'p [GenericParameterP<'a, 'p>],
}
/*
case class GenericParametersP(range: RangeL, params: Vector[GenericParameterP]) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct TemplateRulesP<'a, 'p> {
  pub range: RangeL,
  pub rules: &'p [IRulexPR<'a, 'p>],
}
/*
case class TemplateRulesP(range: RangeL, rules: Vector[IRulexPR]) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
*/

#[derive(Clone, Debug, PartialEq)]
pub struct ParamsP<'a, 'p> {
  pub range: RangeL,
  pub params: &'p [ParameterP<'a, 'p>],
}
/*
case class ParamsP(range: RangeL, params: Vector[ParameterP]) { override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious() }
Guardian: disable: NECX
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
Guardian: disable: NECX
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
Guardian: disable: NECX
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
Guardian: disable: NECX
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
Guardian: disable: NECX
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
Guardian: disable: NECX
*/
