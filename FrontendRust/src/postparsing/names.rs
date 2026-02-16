/*
package dev.vale.postparsing

import dev.vale.{CodeLocationS, IInterning, Interner, PackageCoordinate, RangeS, StrI, vassert, vcheck, vimpl, vpass}
*/
use crate::interner::StrI;
use crate::postparsing::ast::LocationInDenizen;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::{CodeLocationS, RangeS};

/*
trait INameS extends IInterning
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum INameS<'a> {
  FunctionDeclaration(IFunctionDeclarationNameS<'a>),
  ImplDeclaration(ImplDeclarationNameS<'a>),
  AnonymousSubstructImplDeclaration(AnonymousSubstructImplDeclarationNameS<'a>),
  ExportAsName(ExportAsNameS<'a>),
  LetName(LetNameS<'a>),
  TopLevelStructDeclaration(TopLevelStructDeclarationNameS<'a>),
  TopLevelInterfaceDeclaration(TopLevelInterfaceDeclarationNameS<'a>),
  LambdaStructDeclaration(LambdaStructDeclarationNameS<'a>),
  AnonymousSubstructTemplateName(AnonymousSubstructTemplateNameS<'a>),
  RuneName(RuneNameS<'a>),
  RuntimeSizedArrayDeclarationName(RuntimeSizedArrayDeclarationNameS),
  StaticSizedArrayDeclarationName(StaticSizedArrayDeclarationNameS),
  GlobalFunctionFamilyName(GlobalFunctionFamilyNameS),
  ArbitraryName(ArbitraryNameS),
  VarName(IVarNameS<'a>),
}

/*
trait IImpreciseNameS extends IInterning
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IImpreciseNameS<'a> {
  CodeName(&'a CodeNameS<'a>),
  LambdaImpreciseName(&'a LambdaImpreciseNameS),
  PlaceholderImpreciseName(&'a PlaceholderImpreciseNameS),
  LambdaStructImpreciseName(&'a LambdaStructImpreciseNameS<'a>),
  ClosureParamImpreciseName(&'a ClosureParamImpreciseNameS),
  PrototypeName(&'a PrototypeNameS),
  AnonymousSubstructTemplateImpreciseName(&'a AnonymousSubstructTemplateImpreciseNameS<'a>),
  AnonymousSubstructConstructorTemplateImpreciseName(
    &'a AnonymousSubstructConstructorTemplateImpreciseNameS<'a>,
  ),
  ImplImpreciseName(&'a ImplImpreciseNameS<'a>),
  ImplSubCitizenImpreciseName(&'a ImplSubCitizenImpreciseNameS<'a>),
  ImplSuperInterfaceImpreciseName(&'a ImplSuperInterfaceImpreciseNameS<'a>),
  SelfName(&'a SelfNameS),
  RuneName(&'a RuneNameS<'a>),
  ArbitraryName(&'a ArbitraryNameS),
}

impl<'a> IImpreciseNameS<'a> {
  /// Pointer to the canonical interned payload. Use `std::ptr::eq(a.canonical_ptr(), b.canonical_ptr())` for identity comparison.
  pub fn canonical_ptr(&self) -> *const () {
    match self {
      IImpreciseNameS::CodeName(r) => *r as *const _ as *const (),
      IImpreciseNameS::LambdaImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::PlaceholderImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::LambdaStructImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::ClosureParamImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::PrototypeName(r) => *r as *const _ as *const (),
      IImpreciseNameS::AnonymousSubstructTemplateImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::AnonymousSubstructConstructorTemplateImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::ImplImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::ImplSubCitizenImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::ImplSuperInterfaceImpreciseName(r) => *r as *const _ as *const (),
      IImpreciseNameS::SelfName(r) => *r as *const _ as *const (),
      IImpreciseNameS::RuneName(r) => *r as *const _ as *const (),
      IImpreciseNameS::ArbitraryName(r) => *r as *const _ as *const (),
    }
  }

  /// Returns true iff both refer to the same canonical interned value.
  #[inline(always)]
  pub fn ptr_eq(&self, other: &IImpreciseNameS<'a>) -> bool {
    std::ptr::eq(self.canonical_ptr(), other.canonical_ptr())
  }
}

/// Value-struct for LambdaStructImpreciseNameS key. Shallow: references canonical child.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaStructImpreciseNameValS<'a> {
  pub lambda_name: &'a IImpreciseNameS<'a>,
}

/// Value-struct for AnonymousSubstructTemplateImpreciseNameS key. Shallow: references canonical child.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructTemplateImpreciseNameValS<'a> {
  pub interface_imprecise_name: &'a IImpreciseNameS<'a>,
}

/// Value-struct for AnonymousSubstructConstructorTemplateImpreciseNameS key. Shallow: references canonical child.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructConstructorTemplateImpreciseNameValS<'a> {
  pub interface_imprecise_name: &'a IImpreciseNameS<'a>,
}

/// Value-struct for ImplImpreciseNameS key. Shallow: references canonical children.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplImpreciseNameValS<'a> {
  pub sub_citizen_imprecise_name: &'a IImpreciseNameS<'a>,
  pub super_interface_imprecise_name: &'a IImpreciseNameS<'a>,
}

/// Value-struct for ImplSubCitizenImpreciseNameS key. Shallow: references canonical child.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplSubCitizenImpreciseNameValS<'a> {
  pub sub_citizen_imprecise_name: &'a IImpreciseNameS<'a>,
}

/// Value-struct for ImplSuperInterfaceImpreciseNameS key. Shallow: references canonical child.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplSuperInterfaceImpreciseNameValS<'a> {
  pub super_interface_imprecise_name: &'a IImpreciseNameS<'a>,
}

/// Value-struct for RuneNameS key. Shallow: references canonical child rune.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuneNameValS<'a> {
  pub rune: &'a IRuneS<'a>,
}

/// Value/key form of imprecise name for interner lookups. Storage uses canonical `IImpreciseNameS<'a>`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IImpreciseNameValS<'a> {
  CodeName(CodeNameS<'a>),
  LambdaImpreciseName(LambdaImpreciseNameS),
  PlaceholderImpreciseName(PlaceholderImpreciseNameS),
  LambdaStructImpreciseName(LambdaStructImpreciseNameValS<'a>),
  ClosureParamImpreciseName(ClosureParamImpreciseNameS),
  PrototypeName(PrototypeNameS),
  AnonymousSubstructTemplateImpreciseName(AnonymousSubstructTemplateImpreciseNameValS<'a>),
  AnonymousSubstructConstructorTemplateImpreciseName(
    AnonymousSubstructConstructorTemplateImpreciseNameValS<'a>,
  ),
  ImplImpreciseName(ImplImpreciseNameValS<'a>),
  ImplSubCitizenImpreciseName(ImplSubCitizenImpreciseNameValS<'a>),
  ImplSuperInterfaceImpreciseName(ImplSuperInterfaceImpreciseNameValS<'a>),
  SelfName(SelfNameS),
  RuneName(RuneNameValS<'a>),
  ArbitraryName(ArbitraryNameS),
}

/*
sealed trait IVarNameS extends INameS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IVarNameS<'a> {
  CodeVarName(StrI<'a>),
  ConstructingMemberName(StrI<'a>),
  ClosureParamName(CodeLocationS<'a>),
  MagicParamName(CodeLocationS<'a>),
  IterableName(RangeS<'a>),
  IteratorName(RangeS<'a>),
  IterationOptionName(RangeS<'a>),
  WhileCondResultName(RangeS<'a>),
  SelfName,
  AnonymousSubstructMemberName(i32),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IFunctionDeclarationNameS<'a> {
  FunctionName(FunctionNameS<'a>),
  LambdaDeclarationName(LambdaDeclarationNameS<'a>),
  ForwarderFunctionDeclarationName(&'a ForwarderFunctionDeclarationNameS<'a>),
  ConstructorName(&'a ConstructorNameS<'a>),
  ImmConcreteDestructorName(&'a ImmConcreteDestructorNameS<'a>),
  ImmInterfaceDestructorName(&'a ImmInterfaceDestructorNameS<'a>),
}

/*
trait IFunctionDeclarationNameS extends INameS {
  def packageCoordinate: PackageCoordinate
  def getImpreciseName(interner: Interner): IImpreciseNameS
}
*/
/*
trait IImplDeclarationNameS extends INameS {
  def packageCoordinate: PackageCoordinate
}
*/
/*
trait ICitizenDeclarationNameS extends INameS {
  def range: RangeS
  def packageCoordinate: PackageCoordinate
  def getImpreciseName(interner: Interner): IImpreciseNameS
}
*/
/*
//case class FreeDeclarationNameS(codeLocationS: CodeLocationS) extends IFunctionDeclarationNameS {
//
//  override def packageCoordinate: PackageCoordinate = codeLocationS.file.packageCoordinate
//  override def getImpreciseName(interner: Interner): IImpreciseNameS = interner.intern(FreeImpreciseNameS())
//}
//case class FreeImpreciseNameS() extends IImpreciseNameS {
//
//}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaDeclarationNameS<'a> {
  pub code_location: CodeLocationS<'a>,
}

/*
case class LambdaDeclarationNameS(
//  parentName: INameS,
  codeLocation: CodeLocationS
) extends IFunctionDeclarationNameS {

  override def packageCoordinate: PackageCoordinate = codeLocation.file.packageCoordinate
  override def getImpreciseName(interner: Interner): LambdaImpreciseNameS = interner.intern(LambdaImpreciseNameS())
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaImpreciseNameS {}
/*
case class LambdaImpreciseNameS() extends IImpreciseNameS {
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PlaceholderImpreciseNameS {
  pub index: i32,
}
/*
case class PlaceholderImpreciseNameS(index: Int) extends IImpreciseNameS {
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionNameS<'a> {
  pub name: StrI<'a>,
  pub code_location: CodeLocationS<'a>,
}

/*
case class FunctionNameS(name: StrI, codeLocation: CodeLocationS) extends IFunctionDeclarationNameS {
  override def packageCoordinate: PackageCoordinate = codeLocation.file.packageCoordinate
  override def getImpreciseName(interner: Interner): IImpreciseNameS = interner.intern(CodeNameS(name))
}
//case class AbstractVirtualDropFunctionDeclarationNameS(interfaceName: TopLevelCitizenDeclarationNameS) extends IFunctionDeclarationNameS {
//  override def packageCoordinate: PackageCoordinate = interfaceName.packageCoord
//  override def getImpreciseName(interner: Interner): IImpreciseNameS = interner.intern(CodeNameS(Scout.VIRTUAL_DROP_FUNCTION_NAME))
//}
//case class OverrideVirtualDropFunctionDeclarationNameS(implName: IImplDeclarationNameS) extends IFunctionDeclarationNameS {
//  override def packageCoordinate: PackageCoordinate = implName.packageCoord
//  override def getImpreciseName(interner: Interner): IImpreciseNameS = interner.intern(CodeNameS(Scout.VIRTUAL_DROP_FUNCTION_NAME))
//}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ForwarderFunctionDeclarationNameS<'a> {
  pub inner: IFunctionDeclarationNameS<'a>,
  pub index: i32,
}
/*
case class ForwarderFunctionDeclarationNameS(inner: IFunctionDeclarationNameS, index: Int) extends IFunctionDeclarationNameS {
  override def packageCoordinate: PackageCoordinate = inner.packageCoordinate
  override def getImpreciseName(interner: Interner): IImpreciseNameS = inner.getImpreciseName(interner)
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TopLevelCitizenDeclarationNameS<'a> {
  TopLevelStructDeclarationName(TopLevelStructDeclarationNameS<'a>),
  TopLevelInterfaceDeclarationName(TopLevelInterfaceDeclarationNameS<'a>),
}

/*
sealed trait TopLevelCitizenDeclarationNameS extends ICitizenDeclarationNameS {
  def name: StrI
  def range: RangeS
  override def packageCoordinate: PackageCoordinate = range.file.packageCoordinate
  override def getImpreciseName(interner: Interner): IImpreciseNameS = interner.intern(CodeNameS(name))
}
*/
/*
object TopLevelCitizenDeclarationNameS {
  def unapply(n: TopLevelCitizenDeclarationNameS): Option[(StrI, RangeS)] = {
    Some((n.name, n.range))
  }
}
*/
/*
sealed trait IStructDeclarationNameS extends ICitizenDeclarationNameS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TopLevelStructDeclarationNameS<'a> {
  pub name: StrI<'a>,
  pub range: RangeS<'a>,
}
/*
case class TopLevelStructDeclarationNameS(name: StrI, range: RangeS) extends IStructDeclarationNameS with TopLevelCitizenDeclarationNameS {
}
*/
/*
sealed trait IInterfaceDeclarationNameS extends ICitizenDeclarationNameS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TopLevelInterfaceDeclarationNameS<'a> {
  pub name: StrI<'a>,
  pub range: RangeS<'a>,
}
/*
case class TopLevelInterfaceDeclarationNameS(name: StrI, range: RangeS) extends IInterfaceDeclarationNameS with TopLevelCitizenDeclarationNameS {
}
*/
impl<'a> TopLevelCitizenDeclarationNameS<'a> {
  pub fn name(&self) -> StrI<'a> {
    match self {
      TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(x) => x.name,
      TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(x) => x.name,
    }
  }
}

impl<'a> From<&TopLevelStructDeclarationNameS<'a>> for TopLevelCitizenDeclarationNameS<'a> {
  fn from(value: &TopLevelStructDeclarationNameS<'a>) -> Self {
    TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(value.clone())
  }
}

impl<'a> From<&TopLevelInterfaceDeclarationNameS<'a>> for TopLevelCitizenDeclarationNameS<'a> {
  fn from(value: &TopLevelInterfaceDeclarationNameS<'a>) -> Self {
    TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(value.clone())
  }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaStructDeclarationNameS<'a> {
  pub lambda_name: LambdaDeclarationNameS<'a>,
}
/*
case class LambdaStructDeclarationNameS(lambdaName: LambdaDeclarationNameS) extends INameS {
  def getImpreciseName(interner: Interner): LambdaStructImpreciseNameS = interner.intern(LambdaStructImpreciseNameS(lambdaName.getImpreciseName(interner)))
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaStructImpreciseNameS<'a> {
  pub lambda_name: &'a IImpreciseNameS<'a>,
}
/*
case class LambdaStructImpreciseNameS(lambdaName: LambdaImpreciseNameS) extends IImpreciseNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplDeclarationNameS<'a> {
  pub code_location: CodeLocationS<'a>,
}

/*
case class ImplDeclarationNameS(codeLocation: CodeLocationS) extends IImplDeclarationNameS {
  override def packageCoordinate: PackageCoordinate = codeLocation.file.packageCoordinate
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructImplDeclarationNameS<'a> {
  pub interface: TopLevelInterfaceDeclarationNameS<'a>,
}
/*
case class AnonymousSubstructImplDeclarationNameS(interface: TopLevelInterfaceDeclarationNameS) extends IImplDeclarationNameS {
  override def packageCoordinate: PackageCoordinate = interface.packageCoordinate
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExportAsNameS<'a> {
  pub code_location: CodeLocationS<'a>,
}

/*
case class ExportAsNameS(codeLocation: CodeLocationS) extends INameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LetNameS<'a> {
  pub code_location: CodeLocationS<'a>,
}
/*
case class LetNameS(codeLocation: CodeLocationS) extends INameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClosureParamNameS<'a> {
  pub code_location: CodeLocationS<'a>,
}
/*
case class ClosureParamNameS(codeLocation: CodeLocationS) extends IVarNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClosureParamImpreciseNameS {}
/*
case class ClosureParamImpreciseNameS() extends IImpreciseNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrototypeNameS {}
/*
// All prototypes can be looked up via this name.
case class PrototypeNameS() extends IImpreciseNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagicParamNameS<'a> {
  pub code_location: CodeLocationS<'a>,
}
/*
case class MagicParamNameS(codeLocation: CodeLocationS) extends IVarNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructTemplateNameS<'a> {
  pub interface_name: TopLevelInterfaceDeclarationNameS<'a>,
}
/*
case class AnonymousSubstructTemplateNameS(interfaceName: TopLevelInterfaceDeclarationNameS) extends IStructDeclarationNameS {
  vpass()
  override def packageCoordinate: PackageCoordinate = interfaceName.packageCoordinate
  override def getImpreciseName(interner: Interner): IImpreciseNameS = interner.intern(AnonymousSubstructTemplateImpreciseNameS(interfaceName.getImpreciseName(interner)))
  override def range: RangeS = interfaceName.range
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructTemplateImpreciseNameS<'a> {
  pub interface_imprecise_name: &'a IImpreciseNameS<'a>,
}
/*
case class AnonymousSubstructTemplateImpreciseNameS(interfaceImpreciseName: IImpreciseNameS) extends IImpreciseNameS {

}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructConstructorTemplateImpreciseNameS<'a> {
  pub interface_imprecise_name: &'a IImpreciseNameS<'a>,
}
/*
case class AnonymousSubstructConstructorTemplateImpreciseNameS(interfaceImpreciseName: IImpreciseNameS) extends IImpreciseNameS {

}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMemberNameS {
  pub index: i32,
}
/*
case class AnonymousSubstructMemberNameS(index: Int) extends IVarNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CodeVarNameS<'a> {
  pub name: StrI<'a>,
}
/*
case class CodeVarNameS(name: StrI) extends IVarNameS {
  vcheck(name.str != "set", "Can't name a variable 'set'")
  vcheck(name.str != "mut", "Can't name a variable 'mut'")
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstructingMemberNameS<'a> {
  pub name: StrI<'a>,
}
/*
case class ConstructingMemberNameS(name: StrI) extends IVarNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IterableNameS<'a> {
  pub range: RangeS<'a>,
}
/*
case class IterableNameS(range: RangeS) extends IVarNameS with IImpreciseNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IteratorNameS<'a> {
  pub range: RangeS<'a>,
}
/*
case class IteratorNameS(range: RangeS) extends IVarNameS with IImpreciseNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IterationOptionNameS<'a> {
  pub range: RangeS<'a>,
}
/*
case class IterationOptionNameS(range: RangeS) extends IVarNameS with IImpreciseNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhileCondResultNameS<'a> {
  pub range: RangeS<'a>,
}
/*
case class WhileCondResultNameS(range: RangeS) extends IVarNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuneNameS<'a> {
  pub rune: &'a IRuneS<'a>,
}
/*
case class RuneNameS(rune: IRuneS) extends INameS with IImpreciseNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuntimeSizedArrayDeclarationNameS {}
/*
case class RuntimeSizedArrayDeclarationNameS() extends INameS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StaticSizedArrayDeclarationNameS {}
/*
case class StaticSizedArrayDeclarationNameS() extends INameS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IRuneS<'a> {
  CodeRune(&'a CodeRuneS<'a>),
  ImplDropCoordRune(&'a ImplDropCoordRuneS),
  ImplDropVoidRune(&'a ImplDropVoidRuneS),
  ImplicitRune(&'a ImplicitRuneS),
  PureBlockRegionRune(&'a PureBlockRegionRuneS),
  CallRegionRune(&'a CallRegionRuneS),
  CallPureMergeRegionRune(&'a CallPureMergeRegionRuneS),
  ImplicitRegionRune(&'a ImplicitRegionRuneS<'a>),
  ReachablePrototypeRune(&'a ReachablePrototypeRuneS),
  FreeOverrideStructTemplateRune(&'a FreeOverrideStructTemplateRuneS),
  FreeOverrideStructRune(&'a FreeOverrideStructRuneS),
  FreeOverrideInterfaceRune(&'a FreeOverrideInterfaceRuneS),
  LetImplicitRune(&'a LetImplicitRuneS),
  MagicParamRune(&'a MagicParamRuneS),
  MemberRune(&'a MemberRuneS),
  LocalDefaultRegionRune(&'a LocalDefaultRegionRuneS),
  DenizenDefaultRegionRune(&'a DenizenDefaultRegionRuneS<'a>),
  ExportDefaultRegionRune(&'a ExportDefaultRegionRuneS<'a>),
  ExternDefaultRegionRune(&'a ExternDefaultRegionRuneS<'a>),
  ImplicitCoercionOwnershipRune(&'a ImplicitCoercionOwnershipRuneS<'a>),
  ImplicitCoercionKindRune(&'a ImplicitCoercionKindRuneS<'a>),
  ImplicitCoercionTemplateRune(&'a ImplicitCoercionTemplateRuneS<'a>),
  ArraySizeImplicitRune(&'a ArraySizeImplicitRuneS),
  ArrayMutabilityImplicitRune(&'a ArrayMutabilityImplicitRuneS),
  ArrayVariabilityImplicitRune(&'a ArrayVariabilityImplicitRuneS),
  ReturnRune(&'a ReturnRuneS),
  StructNameRune(&'a StructNameRuneS<'a>),
  InterfaceNameRune(&'a InterfaceNameRuneS<'a>),
  SelfRune(&'a SelfRuneS),
  SelfOwnershipRune(&'a SelfOwnershipRuneS),
  SelfKindRune(&'a SelfKindRuneS),
  SelfKindTemplateRune(&'a SelfKindTemplateRuneS<'a>),
  SelfCoordRune(&'a SelfCoordRuneS),
  MacroVoidKindRune(&'a MacroVoidKindRuneS),
  MacroVoidCoordRune(&'a MacroVoidCoordRuneS),
  MacroSelfKindRune(&'a MacroSelfKindRuneS),
  MacroSelfKindTemplateRune(&'a MacroSelfKindTemplateRuneS),
  MacroSelfCoordRune(&'a MacroSelfCoordRuneS),
  ArgumentRune(&'a ArgumentRuneS),
  PatternInputRune(&'a PatternInputRuneS<'a>),
  ExplicitTemplateArgRune(&'a ExplicitTemplateArgRuneS),
  AnonymousSubstructParentInterfaceTemplateRune(
    &'a AnonymousSubstructParentInterfaceTemplateRuneS,
  ),
  AnonymousSubstructParentInterfaceKindRune(&'a AnonymousSubstructParentInterfaceKindRuneS),
  AnonymousSubstructParentInterfaceCoordRune(&'a AnonymousSubstructParentInterfaceCoordRuneS),
  AnonymousSubstructTemplateRune(&'a AnonymousSubstructTemplateRuneS),
  AnonymousSubstructKindRune(&'a AnonymousSubstructKindRuneS),
  AnonymousSubstructCoordRune(&'a AnonymousSubstructCoordRuneS),
  AnonymousSubstructVoidKindRune(&'a AnonymousSubstructVoidKindRuneS),
  AnonymousSubstructVoidCoordRune(&'a AnonymousSubstructVoidCoordRuneS),
  AnonymousSubstructMemberRune(&'a AnonymousSubstructMemberRuneS<'a>),
  AnonymousSubstructMethodSelfBorrowCoordRune(&'a AnonymousSubstructMethodSelfBorrowCoordRuneS<'a>),
  AnonymousSubstructMethodSelfOwnCoordRune(&'a AnonymousSubstructMethodSelfOwnCoordRuneS<'a>),
  AnonymousSubstructDropBoundPrototypeRune(&'a AnonymousSubstructDropBoundPrototypeRuneS<'a>),
  AnonymousSubstructDropBoundParamsListRune(&'a AnonymousSubstructDropBoundParamsListRuneS<'a>),
  AnonymousSubstructFunctionBoundPrototypeRune(&'a AnonymousSubstructFunctionBoundPrototypeRuneS<'a>),
  AnonymousSubstructFunctionBoundParamsListRune(&'a AnonymousSubstructFunctionBoundParamsListRuneS<'a>),
  AnonymousSubstructFunctionInterfaceTemplateRune(
    &'a AnonymousSubstructFunctionInterfaceTemplateRuneS<'a>,
  ),
  AnonymousSubstructFunctionInterfaceKindRune(&'a AnonymousSubstructFunctionInterfaceKindRuneS<'a>),
  AnonymousSubstructMethodInheritedRune(&'a AnonymousSubstructMethodInheritedRuneS<'a>),
  FunctorPrototypeRuneName(&'a FunctorPrototypeRuneNameS),
  FunctorParamRuneName(&'a FunctorParamRuneNameS),
  FunctorReturnRuneName(&'a FunctorReturnRuneNameS),
  DispatcherRuneFromImpl(&'a DispatcherRuneFromImplS<'a>),
  CaseRuneFromImpl(&'a CaseRuneFromImplS<'a>),
}

impl<'a> IRuneS<'a> {
  /// Pointer to the canonical interned payload. Use `std::ptr::eq(a.canonical_ptr(), b.canonical_ptr())` for identity comparison.
  pub fn canonical_ptr(&self) -> *const () {
    match self {
      IRuneS::CodeRune(r) => *r as *const _ as *const (),
      IRuneS::ImplDropCoordRune(r) => *r as *const _ as *const (),
      IRuneS::ImplDropVoidRune(r) => *r as *const _ as *const (),
      IRuneS::ImplicitRune(r) => *r as *const _ as *const (),
      IRuneS::PureBlockRegionRune(r) => *r as *const _ as *const (),
      IRuneS::CallRegionRune(r) => *r as *const _ as *const (),
      IRuneS::CallPureMergeRegionRune(r) => *r as *const _ as *const (),
      IRuneS::ImplicitRegionRune(r) => *r as *const _ as *const (),
      IRuneS::ReachablePrototypeRune(r) => *r as *const _ as *const (),
      IRuneS::FreeOverrideStructTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::FreeOverrideStructRune(r) => *r as *const _ as *const (),
      IRuneS::FreeOverrideInterfaceRune(r) => *r as *const _ as *const (),
      IRuneS::LetImplicitRune(r) => *r as *const _ as *const (),
      IRuneS::MagicParamRune(r) => *r as *const _ as *const (),
      IRuneS::MemberRune(r) => *r as *const _ as *const (),
      IRuneS::LocalDefaultRegionRune(r) => *r as *const _ as *const (),
      IRuneS::DenizenDefaultRegionRune(r) => *r as *const _ as *const (),
      IRuneS::ExportDefaultRegionRune(r) => *r as *const _ as *const (),
      IRuneS::ExternDefaultRegionRune(r) => *r as *const _ as *const (),
      IRuneS::ImplicitCoercionOwnershipRune(r) => *r as *const _ as *const (),
      IRuneS::ImplicitCoercionKindRune(r) => *r as *const _ as *const (),
      IRuneS::ImplicitCoercionTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::ArraySizeImplicitRune(r) => *r as *const _ as *const (),
      IRuneS::ArrayMutabilityImplicitRune(r) => *r as *const _ as *const (),
      IRuneS::ArrayVariabilityImplicitRune(r) => *r as *const _ as *const (),
      IRuneS::ReturnRune(r) => *r as *const _ as *const (),
      IRuneS::StructNameRune(r) => *r as *const _ as *const (),
      IRuneS::InterfaceNameRune(r) => *r as *const _ as *const (),
      IRuneS::SelfRune(r) => *r as *const _ as *const (),
      IRuneS::SelfOwnershipRune(r) => *r as *const _ as *const (),
      IRuneS::SelfKindRune(r) => *r as *const _ as *const (),
      IRuneS::SelfKindTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::SelfCoordRune(r) => *r as *const _ as *const (),
      IRuneS::MacroVoidKindRune(r) => *r as *const _ as *const (),
      IRuneS::MacroVoidCoordRune(r) => *r as *const _ as *const (),
      IRuneS::MacroSelfKindRune(r) => *r as *const _ as *const (),
      IRuneS::MacroSelfKindTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::MacroSelfCoordRune(r) => *r as *const _ as *const (),
      IRuneS::ArgumentRune(r) => *r as *const _ as *const (),
      IRuneS::PatternInputRune(r) => *r as *const _ as *const (),
      IRuneS::ExplicitTemplateArgRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructParentInterfaceTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructParentInterfaceKindRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructParentInterfaceCoordRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructKindRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructCoordRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructVoidKindRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructVoidCoordRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructMemberRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructMethodSelfBorrowCoordRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructMethodSelfOwnCoordRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructDropBoundPrototypeRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructDropBoundParamsListRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructFunctionBoundPrototypeRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructFunctionBoundParamsListRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructFunctionInterfaceTemplateRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructFunctionInterfaceKindRune(r) => *r as *const _ as *const (),
      IRuneS::AnonymousSubstructMethodInheritedRune(r) => *r as *const _ as *const (),
      IRuneS::FunctorPrototypeRuneName(r) => *r as *const _ as *const (),
      IRuneS::FunctorParamRuneName(r) => *r as *const _ as *const (),
      IRuneS::FunctorReturnRuneName(r) => *r as *const _ as *const (),
      IRuneS::DispatcherRuneFromImpl(r) => *r as *const _ as *const (),
      IRuneS::CaseRuneFromImpl(r) => *r as *const _ as *const (),
    }
  }

  /// Returns true iff both refer to the same canonical interned value.
  #[inline(always)]
  pub fn ptr_eq(&self, other: &IRuneS<'a>) -> bool {
    std::ptr::eq(self.canonical_ptr(), other.canonical_ptr())
  }
}

/// Value-struct for ImplicitRegionRuneS key. Shallow: references canonical child rune.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitRegionRuneValS<'a> {
  pub original_rune: &'a IRuneS<'a>,
}

/// Value-struct for ImplicitCoercionOwnershipRuneS key. Shallow: references canonical child rune.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionOwnershipRuneValS<'a> {
  pub range: RangeS<'a>,
  pub original_coord_rune: &'a IRuneS<'a>,
}

/// Value-struct for ImplicitCoercionKindRuneS key. Shallow: references canonical child rune.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionKindRuneValS<'a> {
  pub range: RangeS<'a>,
  pub original_coord_rune: &'a IRuneS<'a>,
}

/// Value-struct for ImplicitCoercionTemplateRuneS key. Shallow: references canonical child rune.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionTemplateRuneValS<'a> {
  pub range: RangeS<'a>,
  pub original_kind_rune: &'a IRuneS<'a>,
}

/// Value-struct for AnonymousSubstructMethodInheritedRuneS key. Shallow: references canonical child rune.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMethodInheritedRuneValS<'a> {
  pub interface: TopLevelInterfaceDeclarationNameS<'a>,
  pub method: IFunctionDeclarationNameS<'a>,
  pub inner: &'a IRuneS<'a>,
}

/// Value-struct for DispatcherRuneFromImplS key. Shallow: references canonical child rune.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DispatcherRuneFromImplValS<'a> {
  pub inner_rune: &'a IRuneS<'a>,
}

/// Value-struct for CaseRuneFromImplS key. Shallow: references canonical child rune.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CaseRuneFromImplValS<'a> {
  pub inner_rune: &'a IRuneS<'a>,
}

/// Value/key form of rune for interner lookups. Used when constructing runes before
/// canonicalizing via `intern_rune`. Storage fields use canonical `IRuneS<'a>`.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IRuneValS<'a> {
  CodeRune(CodeRuneS<'a>),
  ImplDropCoordRune(ImplDropCoordRuneS),
  ImplDropVoidRune(ImplDropVoidRuneS),
  ImplicitRune(ImplicitRuneS),
  PureBlockRegionRune(PureBlockRegionRuneS),
  CallRegionRune(CallRegionRuneS),
  CallPureMergeRegionRune(CallPureMergeRegionRuneS),
  ImplicitRegionRune(ImplicitRegionRuneValS<'a>),
  ReachablePrototypeRune(ReachablePrototypeRuneS),
  FreeOverrideStructTemplateRune(FreeOverrideStructTemplateRuneS),
  FreeOverrideStructRune(FreeOverrideStructRuneS),
  FreeOverrideInterfaceRune(FreeOverrideInterfaceRuneS),
  LetImplicitRune(LetImplicitRuneS),
  MagicParamRune(MagicParamRuneS),
  MemberRune(MemberRuneS),
  LocalDefaultRegionRune(LocalDefaultRegionRuneS),
  DenizenDefaultRegionRune(DenizenDefaultRegionRuneS<'a>),
  ExportDefaultRegionRune(ExportDefaultRegionRuneS<'a>),
  ExternDefaultRegionRune(ExternDefaultRegionRuneS<'a>),
  ImplicitCoercionOwnershipRune(ImplicitCoercionOwnershipRuneValS<'a>),
  ImplicitCoercionKindRune(ImplicitCoercionKindRuneValS<'a>),
  ImplicitCoercionTemplateRune(ImplicitCoercionTemplateRuneValS<'a>),
  ArraySizeImplicitRune(ArraySizeImplicitRuneS),
  ArrayMutabilityImplicitRune(ArrayMutabilityImplicitRuneS),
  ArrayVariabilityImplicitRune(ArrayVariabilityImplicitRuneS),
  ReturnRune(ReturnRuneS),
  StructNameRune(StructNameRuneS<'a>),
  InterfaceNameRune(InterfaceNameRuneS<'a>),
  SelfRune(SelfRuneS),
  SelfOwnershipRune(SelfOwnershipRuneS),
  SelfKindRune(SelfKindRuneS),
  SelfKindTemplateRune(SelfKindTemplateRuneS<'a>),
  SelfCoordRune(SelfCoordRuneS),
  MacroVoidKindRune(MacroVoidKindRuneS),
  MacroVoidCoordRune(MacroVoidCoordRuneS),
  MacroSelfKindRune(MacroSelfKindRuneS),
  MacroSelfKindTemplateRune(MacroSelfKindTemplateRuneS),
  MacroSelfCoordRune(MacroSelfCoordRuneS),
  ArgumentRune(ArgumentRuneS),
  PatternInputRune(PatternInputRuneS<'a>),
  ExplicitTemplateArgRune(ExplicitTemplateArgRuneS),
  AnonymousSubstructParentInterfaceTemplateRune(AnonymousSubstructParentInterfaceTemplateRuneS),
  AnonymousSubstructParentInterfaceKindRune(AnonymousSubstructParentInterfaceKindRuneS),
  AnonymousSubstructParentInterfaceCoordRune(AnonymousSubstructParentInterfaceCoordRuneS),
  AnonymousSubstructTemplateRune(AnonymousSubstructTemplateRuneS),
  AnonymousSubstructKindRune(AnonymousSubstructKindRuneS),
  AnonymousSubstructCoordRune(AnonymousSubstructCoordRuneS),
  AnonymousSubstructVoidKindRune(AnonymousSubstructVoidKindRuneS),
  AnonymousSubstructVoidCoordRune(AnonymousSubstructVoidCoordRuneS),
  AnonymousSubstructMemberRune(AnonymousSubstructMemberRuneS<'a>),
  AnonymousSubstructMethodSelfBorrowCoordRune(AnonymousSubstructMethodSelfBorrowCoordRuneS<'a>),
  AnonymousSubstructMethodSelfOwnCoordRune(AnonymousSubstructMethodSelfOwnCoordRuneS<'a>),
  AnonymousSubstructDropBoundPrototypeRune(AnonymousSubstructDropBoundPrototypeRuneS<'a>),
  AnonymousSubstructDropBoundParamsListRune(AnonymousSubstructDropBoundParamsListRuneS<'a>),
  AnonymousSubstructFunctionBoundPrototypeRune(AnonymousSubstructFunctionBoundPrototypeRuneS<'a>),
  AnonymousSubstructFunctionBoundParamsListRune(AnonymousSubstructFunctionBoundParamsListRuneS<'a>),
  AnonymousSubstructFunctionInterfaceTemplateRune(AnonymousSubstructFunctionInterfaceTemplateRuneS<'a>),
  AnonymousSubstructFunctionInterfaceKindRune(AnonymousSubstructFunctionInterfaceKindRuneS<'a>),
  AnonymousSubstructMethodInheritedRune(AnonymousSubstructMethodInheritedRuneValS<'a>),
  FunctorPrototypeRuneName(FunctorPrototypeRuneNameS),
  FunctorParamRuneName(FunctorParamRuneNameS),
  FunctorReturnRuneName(FunctorReturnRuneNameS),
  DispatcherRuneFromImpl(DispatcherRuneFromImplValS<'a>),
  CaseRuneFromImpl(CaseRuneFromImplValS<'a>),
}

/*
// We differentiate rune names from regular names, we scout out what's actually
// a rune so we can inform the typingpass. The typingpass wants to know so it can know
// how to handle this thing; if it's a name, we expect it to exist in the
// environment already, but if it's a rune we can assign something into it.
// Also, we might refer to a rune that was defined in our container's container's
// container, so asking "is this thing here a rune" involves looking at all our
// containers. That's much easier for the scout, so thats a nice bonus.
// We have all these subclasses instead of a string so we don't have to have
// prefixes and names like __implicit_0, __paramRune_0, etc.
// This extends INameS so we can use it as a lookup key in Compiler's environments.
trait IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CodeRuneS<'a> {
  pub name: StrI<'a>,
}

/*
case class CodeRuneS(name: StrI) extends IRuneS {
  vpass()
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplDropCoordRuneS {}
/*
case class ImplDropCoordRuneS() extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplDropVoidRuneS {}
/*
case class ImplDropVoidRuneS() extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitRuneS {
  pub lid: LocationInDenizen,
}
/*
case class ImplicitRuneS(lid: LocationInDenizen) extends IRuneS {
  vpass()
  lid match {
    case LocationInDenizen(Vector(2, 1, 1, 2)) => {
      vpass()
    }
    case _ =>
  }
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PureBlockRegionRuneS {
  pub lid: LocationInDenizen,
}
/*
case class PureBlockRegionRuneS(lid: LocationInDenizen) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallRegionRuneS {
  pub lid: LocationInDenizen,
}
/*
case class CallRegionRuneS(lid: LocationInDenizen) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CallPureMergeRegionRuneS {
  pub lid: LocationInDenizen,
}
/*
case class CallPureMergeRegionRuneS(lid: LocationInDenizen) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitRegionRuneS<'a> {
  pub original_rune: &'a IRuneS<'a>,
}
/*
case class ImplicitRegionRuneS(originalRune: IRuneS) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReachablePrototypeRuneS {
  pub num: i32,
}
/*
case class ReachablePrototypeRuneS(num: Int) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FreeOverrideStructTemplateRuneS {}
/*
case class FreeOverrideStructTemplateRuneS() extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FreeOverrideStructRuneS {}
/*
case class FreeOverrideStructRuneS() extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FreeOverrideInterfaceRuneS {}
/*
case class FreeOverrideInterfaceRuneS() extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LetImplicitRuneS {
  pub lid: LocationInDenizen,
}
/*
case class LetImplicitRuneS(lid: LocationInDenizen) extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MagicParamRuneS {
  pub lid: LocationInDenizen,
}
/*
case class MagicParamRuneS(lid: LocationInDenizen) extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MemberRuneS {
  pub member_index: i32,
}
/*
case class MemberRuneS(memberIndex: Int) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocalDefaultRegionRuneS {
  pub lid: LocationInDenizen,
}
/*

case class LocalDefaultRegionRuneS(lid: LocationInDenizen) extends IRuneS
// This has a name because there might be multiple default regions in play sometimes.
// When a function calls the constructor for a struct, the function has its own default region,
// but it's also evaluating the rules for the struct. Best not mix them up.
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DenizenDefaultRegionRuneS<'a> {
  pub denizen_name: INameS<'a>,
}

/*
case class DenizenDefaultRegionRuneS(denizenName: INameS) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExportDefaultRegionRuneS<'a> {
  pub denizen_name: INameS<'a>,
}
/*
case class ExportDefaultRegionRuneS(denizenName: INameS) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExternDefaultRegionRuneS<'a> {
  pub denizen_name: INameS<'a>,
}
/*
case class ExternDefaultRegionRuneS(denizenName: INameS) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionOwnershipRuneS<'a> {
  pub range: RangeS<'a>,
  pub original_coord_rune: &'a IRuneS<'a>,
}
/*
case class ImplicitCoercionOwnershipRuneS(range: RangeS, originalCoordRune: IRuneS) extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionKindRuneS<'a> {
  pub range: RangeS<'a>,
  pub original_coord_rune: &'a IRuneS<'a>,
}
/*
case class ImplicitCoercionKindRuneS(range: RangeS, originalCoordRune: IRuneS) extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionTemplateRuneS<'a> {
  pub range: RangeS<'a>,
  pub original_kind_rune: &'a IRuneS<'a>,
}
/*
case class ImplicitCoercionTemplateRuneS(range: RangeS, originalKindRune: IRuneS) extends IRuneS {  }

*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArraySizeImplicitRuneS {}
/*
// Used to type the templex handed to the size part of the static sized array expressions
case class ArraySizeImplicitRuneS() extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArrayMutabilityImplicitRuneS {}
/*
// Used to type the templex handed to the mutability part of the static sized array expressions
case class ArrayMutabilityImplicitRuneS() extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArrayVariabilityImplicitRuneS {}
/*
// Used to type the templex handed to the variability part of the static sized array expressions
case class ArrayVariabilityImplicitRuneS() extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ReturnRuneS {}
/*
case class ReturnRuneS() extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructNameRuneS<'a> {
  pub struct_name: TopLevelCitizenDeclarationNameS<'a>,
}
/*
case class StructNameRuneS(structName: ICitizenDeclarationNameS) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InterfaceNameRuneS<'a> {
  pub interface_name: TopLevelCitizenDeclarationNameS<'a>,
}
/*
case class InterfaceNameRuneS(interfaceName: ICitizenDeclarationNameS) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfRuneS {}
/*
// Vale has no notion of Self, it's just a convenient name for a first parameter.
case class SelfRuneS() extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfOwnershipRuneS {}
/*
case class SelfOwnershipRuneS() extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfKindRuneS {}
/*
case class SelfKindRuneS() extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfKindTemplateRuneS<'a> {
  pub loc: CodeLocationS<'a>,
}
/*
case class SelfKindTemplateRuneS(loc: CodeLocationS) extends IRuneS {
  vpass()
}
  */
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfCoordRuneS {}
/*
case class SelfCoordRuneS() extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MacroVoidKindRuneS {}
/*
case class MacroVoidKindRuneS() extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MacroVoidCoordRuneS {}
/*
case class MacroVoidCoordRuneS() extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MacroSelfKindRuneS {}
/*
case class MacroSelfKindRuneS() extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MacroSelfKindTemplateRuneS {}
/*
case class MacroSelfKindTemplateRuneS() extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MacroSelfCoordRuneS {}
/*
case class MacroSelfCoordRuneS() extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CodeNameS<'a> {
  pub name: StrI<'a>,
}

/*
case class CodeNameS(name: StrI) extends IImpreciseNameS {
  vpass()
  vassert(name.str != "_")
}
  */
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct GlobalFunctionFamilyNameS {
  pub name: String,
}
/*
// When we're calling a function, we're addressing an overload set, not a specific function.
// If we want a specific function, we use TopLevelDeclarationNameS.
case class GlobalFunctionFamilyNameS(name: String) extends INameS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArgumentRuneS {
  pub arg_index: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PatternInputRuneS<'a> {
  pub code_loc: CodeLocationS<'a>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExplicitTemplateArgRuneS {
  pub index: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructParentInterfaceTemplateRuneS {}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructParentInterfaceKindRuneS {}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructParentInterfaceCoordRuneS {}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructTemplateRuneS {}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructKindRuneS {}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructCoordRuneS {}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructVoidKindRuneS {}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructVoidCoordRuneS {}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMemberRuneS<'a> {
  pub interface: TopLevelInterfaceDeclarationNameS<'a>,
  pub method: IFunctionDeclarationNameS<'a>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMethodSelfBorrowCoordRuneS<'a> {
  pub interface: TopLevelInterfaceDeclarationNameS<'a>,
  pub method: IFunctionDeclarationNameS<'a>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMethodSelfOwnCoordRuneS<'a> {
  pub interface: TopLevelInterfaceDeclarationNameS<'a>,
  pub method: IFunctionDeclarationNameS<'a>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructDropBoundPrototypeRuneS<'a> {
  pub interface: TopLevelInterfaceDeclarationNameS<'a>,
  pub method: IFunctionDeclarationNameS<'a>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructDropBoundParamsListRuneS<'a> {
  pub interface: TopLevelInterfaceDeclarationNameS<'a>,
  pub method: IFunctionDeclarationNameS<'a>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructFunctionBoundPrototypeRuneS<'a> {
  pub interface: TopLevelInterfaceDeclarationNameS<'a>,
  pub method: IFunctionDeclarationNameS<'a>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructFunctionBoundParamsListRuneS<'a> {
  pub interface: TopLevelInterfaceDeclarationNameS<'a>,
  pub method: IFunctionDeclarationNameS<'a>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructFunctionInterfaceTemplateRuneS<'a> {
  pub interface: TopLevelInterfaceDeclarationNameS<'a>,
  pub method: IFunctionDeclarationNameS<'a>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructFunctionInterfaceKindRuneS<'a> {
  pub interface: TopLevelInterfaceDeclarationNameS<'a>,
  pub method: IFunctionDeclarationNameS<'a>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMethodInheritedRuneS<'a> {
  pub interface: TopLevelInterfaceDeclarationNameS<'a>,
  pub method: IFunctionDeclarationNameS<'a>,
  pub inner: &'a IRuneS<'a>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctorPrototypeRuneNameS {}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctorParamRuneNameS {
  pub index: i32,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctorReturnRuneNameS {}
/*

// These are only made by the typingpass
case class ArgumentRuneS(argIndex: Int) extends IRuneS {  }
case class PatternInputRuneS(codeLoc: CodeLocationS) extends IRuneS {  }
case class ExplicitTemplateArgRuneS(index: Int) extends IRuneS {  }
case class AnonymousSubstructParentInterfaceTemplateRuneS() extends IRuneS {  }
case class AnonymousSubstructParentInterfaceKindRuneS() extends IRuneS {  }
case class AnonymousSubstructParentInterfaceCoordRuneS() extends IRuneS {  }
case class AnonymousSubstructTemplateRuneS() extends IRuneS {  }
case class AnonymousSubstructKindRuneS() extends IRuneS {  }
case class AnonymousSubstructCoordRuneS() extends IRuneS {  }
case class AnonymousSubstructVoidKindRuneS() extends IRuneS {  }
case class AnonymousSubstructVoidCoordRuneS() extends IRuneS {  }
case class AnonymousSubstructMemberRuneS(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS {  }
case class AnonymousSubstructMethodSelfBorrowCoordRuneS(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
case class AnonymousSubstructMethodSelfOwnCoordRuneS(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
case class AnonymousSubstructDropBoundPrototypeRuneS(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
case class AnonymousSubstructDropBoundParamsListRuneS(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
case class AnonymousSubstructFunctionBoundPrototypeRuneS(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
case class AnonymousSubstructFunctionBoundParamsListRuneS(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
//case class AnonymousSubstructFunctionInterfaceKindRune(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
//case class AnonymousSubstructFunctionInterfaceOwnershipRune(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
case class AnonymousSubstructFunctionInterfaceTemplateRune(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
case class AnonymousSubstructFunctionInterfaceKindRune(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS) extends IRuneS { }
case class AnonymousSubstructMethodInheritedRuneS(interface: TopLevelInterfaceDeclarationNameS, method: IFunctionDeclarationNameS, inner: IRuneS) extends IRuneS {
  this match {
    case AnonymousSubstructMethodInheritedRuneS(TopLevelInterfaceDeclarationNameS(StrI("Bork"),_),FunctionNameS(StrI("bork"),_),ImplicitRuneS(LocationInDenizen(Vector(2, 1, 1, 2, 1, 1)))) => {
      vpass()
    }
    case _ =>
  }
}
case class FunctorPrototypeRuneNameS() extends IRuneS
case class FunctorParamRuneNameS(index: Int) extends IRuneS
case class FunctorReturnRuneNameS() extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SelfNameS {}
/*
// Vale has no notion of Self, it's just a convenient name for a first parameter.
case class SelfNameS() extends IVarNameS with IImpreciseNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ArbitraryNameS {}
/*
// A miscellaneous name, for when a name doesn't really make sense, like it's the only entry in the environment or something.
case class ArbitraryNameS() extends INameS with IImpreciseNameS

*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DispatcherRuneFromImplS<'a> {
  pub inner_rune: &'a IRuneS<'a>,
}
/*
case class DispatcherRuneFromImplS(innerRune: IRuneS) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CaseRuneFromImplS<'a> {
  pub inner_rune: &'a IRuneS<'a>,
}
/*
case class CaseRuneFromImplS(innerRune: IRuneS) extends IRuneS

// Only made by typingpass, see if we can take these out
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstructorNameS<'a> {
  pub tlcd: TopLevelCitizenDeclarationNameS<'a>,
}
/*
case class ConstructorNameS(tlcd: ICitizenDeclarationNameS) extends IFunctionDeclarationNameS {
  override def packageCoordinate: PackageCoordinate = tlcd.range.begin.file.packageCoordinate
  override def getImpreciseName(interner: Interner): IImpreciseNameS = tlcd.getImpreciseName(interner)
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImmConcreteDestructorNameS<'a> {
  pub package_coordinate: PackageCoordinate<'a>,
}
/*
case class ImmConcreteDestructorNameS(packageCoordinate: PackageCoordinate) extends IFunctionDeclarationNameS {
  override def getImpreciseName(interner: Interner): IImpreciseNameS = vimpl()
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImmInterfaceDestructorNameS<'a> {
  pub package_coordinate: PackageCoordinate<'a>,
}
/*
case class ImmInterfaceDestructorNameS(packageCoordinate: PackageCoordinate) extends IFunctionDeclarationNameS {
  override def getImpreciseName(interner: Interner): IImpreciseNameS = vimpl()
}

*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplImpreciseNameS<'a> {
  pub sub_citizen_imprecise_name: &'a IImpreciseNameS<'a>,
  pub super_interface_imprecise_name: &'a IImpreciseNameS<'a>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplSubCitizenImpreciseNameS<'a> {
  pub sub_citizen_imprecise_name: &'a IImpreciseNameS<'a>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplSuperInterfaceImpreciseNameS<'a> {
  pub super_interface_imprecise_name: &'a IImpreciseNameS<'a>,
}
/*
case class ImplImpreciseNameS(subCitizenImpreciseName: IImpreciseNameS, superInterfaceImpreciseName: IImpreciseNameS) extends IImpreciseNameS { }
case class ImplSubCitizenImpreciseNameS(subCitizenImpreciseName: IImpreciseNameS) extends IImpreciseNameS { }
case class ImplSuperInterfaceImpreciseNameS(superInterfaceImpreciseName: IImpreciseNameS) extends IImpreciseNameS { }

// See NSIDN for why we need this virtual name
//case class VirtualFreeImpreciseNameS() extends IImpreciseNameS { }
//case class AbstractVirtualFreeDeclarationNameS(codeLoc: CodeLocationS) extends IFunctionDeclarationNameS {
//  override def getImpreciseName(interner: Interner): IImpreciseNameS = interner.intern(VirtualFreeImpreciseNameS())
//  override def packageCoordinate: PackageCoordinate = codeLoc.file.packageCoord
//}
//case class OverrideVirtualFreeDeclarationNameS(codeLoc: CodeLocationS) extends IFunctionDeclarationNameS {
//  override def getImpreciseName(interner: Interner): IImpreciseNameS = interner.intern(VirtualFreeImpreciseNameS())
//  override def packageCoordinate: PackageCoordinate = codeLoc.file.packageCoord
//}
*/