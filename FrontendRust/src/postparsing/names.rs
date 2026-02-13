/*
package dev.vale.postparsing

import dev.vale.{CodeLocationS, IInterning, Interner, PackageCoordinate, RangeS, StrI, vassert, vcheck, vimpl, vpass}
*/
use crate::interner::StrI;
use crate::postparsing::ast::LocationInDenizen;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::{CodeLocationS, RangeS};
use std::sync::Arc;

/*
trait INameS extends IInterning
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum INameS {
  FunctionDeclaration(IFunctionDeclarationNameS),
  ImplDeclaration(ImplDeclarationNameS),
  AnonymousSubstructImplDeclaration(AnonymousSubstructImplDeclarationNameS),
  ExportAsName(ExportAsNameS),
  LetName(LetNameS),
  TopLevelStructDeclaration(TopLevelStructDeclarationNameS),
  TopLevelInterfaceDeclaration(TopLevelInterfaceDeclarationNameS),
  LambdaStructDeclaration(LambdaStructDeclarationNameS),
  AnonymousSubstructTemplateName(AnonymousSubstructTemplateNameS),
  RuneName(RuneNameS),
  RuntimeSizedArrayDeclarationName(RuntimeSizedArrayDeclarationNameS),
  StaticSizedArrayDeclarationName(StaticSizedArrayDeclarationNameS),
  GlobalFunctionFamilyName(GlobalFunctionFamilyNameS),
  ArbitraryName(ArbitraryNameS),
  VarName(IVarNameS),
}

/*
trait IImpreciseNameS extends IInterning
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IImpreciseNameS {
  CodeName(Arc<CodeNameS>),
  LambdaImpreciseName(Arc<LambdaImpreciseNameS>),
  PlaceholderImpreciseName(Arc<PlaceholderImpreciseNameS>),
  LambdaStructImpreciseName(Arc<LambdaStructImpreciseNameS>),
  ClosureParamImpreciseName(Arc<ClosureParamImpreciseNameS>),
  PrototypeName(Arc<PrototypeNameS>),
  AnonymousSubstructTemplateImpreciseName(Arc<AnonymousSubstructTemplateImpreciseNameS>),
  AnonymousSubstructConstructorTemplateImpreciseName(
    Arc<AnonymousSubstructConstructorTemplateImpreciseNameS>,
  ),
  ImplImpreciseName(Arc<ImplImpreciseNameS>),
  ImplSubCitizenImpreciseName(Arc<ImplSubCitizenImpreciseNameS>),
  ImplSuperInterfaceImpreciseName(Arc<ImplSuperInterfaceImpreciseNameS>),
  SelfName(Arc<SelfNameS>),
  RuneName(Arc<RuneNameS>),
  ArbitraryName(Arc<ArbitraryNameS>),
}

/*
sealed trait IVarNameS extends INameS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IVarNameS {
  CodeVarName(Arc<StrI>),
  ConstructingMemberName(Arc<StrI>),
  ClosureParamName(CodeLocationS),
  MagicParamName(CodeLocationS),
  IterableName(RangeS),
  IteratorName(RangeS),
  IterationOptionName(RangeS),
  WhileCondResultName(RangeS),
  SelfName,
  AnonymousSubstructMemberName(i32),
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum IFunctionDeclarationNameS {
  FunctionName(FunctionNameS),
  LambdaDeclarationName(LambdaDeclarationNameS),
  ForwarderFunctionDeclarationName(Arc<ForwarderFunctionDeclarationNameS>),
  ConstructorName(Arc<ConstructorNameS>),
  ImmConcreteDestructorName(Arc<ImmConcreteDestructorNameS>),
  ImmInterfaceDestructorName(Arc<ImmInterfaceDestructorNameS>),
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
pub struct LambdaDeclarationNameS {
  pub code_location: CodeLocationS,
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
pub struct FunctionNameS {
  pub name: Arc<StrI>,
  pub code_location: CodeLocationS,
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
pub struct ForwarderFunctionDeclarationNameS {
  pub inner: IFunctionDeclarationNameS,
  pub index: i32,
}
/*
case class ForwarderFunctionDeclarationNameS(inner: IFunctionDeclarationNameS, index: Int) extends IFunctionDeclarationNameS {
  override def packageCoordinate: PackageCoordinate = inner.packageCoordinate
  override def getImpreciseName(interner: Interner): IImpreciseNameS = inner.getImpreciseName(interner)
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum TopLevelCitizenDeclarationNameS {
  TopLevelStructDeclarationName(TopLevelStructDeclarationNameS),
  TopLevelInterfaceDeclarationName(TopLevelInterfaceDeclarationNameS),
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
pub struct TopLevelStructDeclarationNameS {
  pub name: Arc<StrI>,
  pub range: RangeS,
}
/*
case class TopLevelStructDeclarationNameS(name: StrI, range: RangeS) extends IStructDeclarationNameS with TopLevelCitizenDeclarationNameS {
}
*/
/*
sealed trait IInterfaceDeclarationNameS extends ICitizenDeclarationNameS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TopLevelInterfaceDeclarationNameS {
  pub name: Arc<StrI>,
  pub range: RangeS,
}
/*
case class TopLevelInterfaceDeclarationNameS(name: StrI, range: RangeS) extends IInterfaceDeclarationNameS with TopLevelCitizenDeclarationNameS {
}
*/
impl TopLevelCitizenDeclarationNameS {
  pub fn name(&self) -> &Arc<StrI> {
    match self {
      TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(x) => &x.name,
      TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(x) => &x.name,
    }
  }
}

impl From<&TopLevelStructDeclarationNameS> for TopLevelCitizenDeclarationNameS {
  fn from(value: &TopLevelStructDeclarationNameS) -> Self {
    TopLevelCitizenDeclarationNameS::TopLevelStructDeclarationName(value.clone())
  }
}

impl From<&TopLevelInterfaceDeclarationNameS> for TopLevelCitizenDeclarationNameS {
  fn from(value: &TopLevelInterfaceDeclarationNameS) -> Self {
    TopLevelCitizenDeclarationNameS::TopLevelInterfaceDeclarationName(value.clone())
  }
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaStructDeclarationNameS {
  pub lambda_name: LambdaDeclarationNameS,
}
/*
case class LambdaStructDeclarationNameS(lambdaName: LambdaDeclarationNameS) extends INameS {
  def getImpreciseName(interner: Interner): LambdaStructImpreciseNameS = interner.intern(LambdaStructImpreciseNameS(lambdaName.getImpreciseName(interner)))
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LambdaStructImpreciseNameS {
  pub lambda_name: Arc<IImpreciseNameS>,
}
/*
case class LambdaStructImpreciseNameS(lambdaName: LambdaImpreciseNameS) extends IImpreciseNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplDeclarationNameS {
  pub code_location: CodeLocationS,
}

/*
case class ImplDeclarationNameS(codeLocation: CodeLocationS) extends IImplDeclarationNameS {
  override def packageCoordinate: PackageCoordinate = codeLocation.file.packageCoordinate
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructImplDeclarationNameS {
  pub interface: TopLevelInterfaceDeclarationNameS,
}
/*
case class AnonymousSubstructImplDeclarationNameS(interface: TopLevelInterfaceDeclarationNameS) extends IImplDeclarationNameS {
  override def packageCoordinate: PackageCoordinate = interface.packageCoordinate
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExportAsNameS {
  pub code_location: CodeLocationS,
}

/*
case class ExportAsNameS(codeLocation: CodeLocationS) extends INameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LetNameS {
  pub code_location: CodeLocationS,
}
/*
case class LetNameS(codeLocation: CodeLocationS) extends INameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClosureParamNameS {
  pub code_location: CodeLocationS,
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
pub struct MagicParamNameS {
  pub code_location: CodeLocationS,
}
/*
case class MagicParamNameS(codeLocation: CodeLocationS) extends IVarNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructTemplateNameS {
  pub interface_name: TopLevelInterfaceDeclarationNameS,
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
pub struct AnonymousSubstructTemplateImpreciseNameS {
  pub interface_imprecise_name: Box<IImpreciseNameS>,
}
/*
case class AnonymousSubstructTemplateImpreciseNameS(interfaceImpreciseName: IImpreciseNameS) extends IImpreciseNameS {

}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructConstructorTemplateImpreciseNameS {
  pub interface_imprecise_name: Box<IImpreciseNameS>,
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
pub struct CodeVarNameS {
  pub name: Arc<StrI>,
}
/*
case class CodeVarNameS(name: StrI) extends IVarNameS {
  vcheck(name.str != "set", "Can't name a variable 'set'")
  vcheck(name.str != "mut", "Can't name a variable 'mut'")
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstructingMemberNameS {
  pub name: Arc<StrI>,
}
/*
case class ConstructingMemberNameS(name: StrI) extends IVarNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IterableNameS {
  pub range: RangeS,
}
/*
case class IterableNameS(range: RangeS) extends IVarNameS with IImpreciseNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IteratorNameS {
  pub range: RangeS,
}
/*
case class IteratorNameS(range: RangeS) extends IVarNameS with IImpreciseNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct IterationOptionNameS {
  pub range: RangeS,
}
/*
case class IterationOptionNameS(range: RangeS) extends IVarNameS with IImpreciseNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WhileCondResultNameS {
  pub range: RangeS,
}
/*
case class WhileCondResultNameS(range: RangeS) extends IVarNameS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct RuneNameS {
  pub rune: Box<IRuneS>,
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
pub enum IRuneS {
  CodeRune(Arc<CodeRuneS>),
  ImplDropCoordRune(Arc<ImplDropCoordRuneS>),
  ImplDropVoidRune(Arc<ImplDropVoidRuneS>),
  ImplicitRune(Arc<ImplicitRuneS>),
  PureBlockRegionRune(Arc<PureBlockRegionRuneS>),
  CallRegionRune(Arc<CallRegionRuneS>),
  CallPureMergeRegionRune(Arc<CallPureMergeRegionRuneS>),
  ImplicitRegionRune(Arc<ImplicitRegionRuneS>),
  ReachablePrototypeRune(Arc<ReachablePrototypeRuneS>),
  FreeOverrideStructTemplateRune(Arc<FreeOverrideStructTemplateRuneS>),
  FreeOverrideStructRune(Arc<FreeOverrideStructRuneS>),
  FreeOverrideInterfaceRune(Arc<FreeOverrideInterfaceRuneS>),
  LetImplicitRune(Arc<LetImplicitRuneS>),
  MagicParamRune(Arc<MagicParamRuneS>),
  MemberRune(Arc<MemberRuneS>),
  LocalDefaultRegionRune(Arc<LocalDefaultRegionRuneS>),
  DenizenDefaultRegionRune(Arc<DenizenDefaultRegionRuneS>),
  ExportDefaultRegionRune(Arc<ExportDefaultRegionRuneS>),
  ExternDefaultRegionRune(Arc<ExternDefaultRegionRuneS>),
  ImplicitCoercionOwnershipRune(Arc<ImplicitCoercionOwnershipRuneS>),
  ImplicitCoercionKindRune(Arc<ImplicitCoercionKindRuneS>),
  ImplicitCoercionTemplateRune(Arc<ImplicitCoercionTemplateRuneS>),
  ArraySizeImplicitRune(Arc<ArraySizeImplicitRuneS>),
  ArrayMutabilityImplicitRune(Arc<ArrayMutabilityImplicitRuneS>),
  ArrayVariabilityImplicitRune(Arc<ArrayVariabilityImplicitRuneS>),
  ReturnRune(Arc<ReturnRuneS>),
  StructNameRune(Arc<StructNameRuneS>),
  InterfaceNameRune(Arc<InterfaceNameRuneS>),
  SelfRune(Arc<SelfRuneS>),
  SelfOwnershipRune(Arc<SelfOwnershipRuneS>),
  SelfKindRune(Arc<SelfKindRuneS>),
  SelfKindTemplateRune(Arc<SelfKindTemplateRuneS>),
  SelfCoordRune(Arc<SelfCoordRuneS>),
  MacroVoidKindRune(Arc<MacroVoidKindRuneS>),
  MacroVoidCoordRune(Arc<MacroVoidCoordRuneS>),
  MacroSelfKindRune(Arc<MacroSelfKindRuneS>),
  MacroSelfKindTemplateRune(Arc<MacroSelfKindTemplateRuneS>),
  MacroSelfCoordRune(Arc<MacroSelfCoordRuneS>),
  ArgumentRune(Arc<ArgumentRuneS>),
  PatternInputRune(Arc<PatternInputRuneS>),
  ExplicitTemplateArgRune(Arc<ExplicitTemplateArgRuneS>),
  AnonymousSubstructParentInterfaceTemplateRune(
    Arc<AnonymousSubstructParentInterfaceTemplateRuneS>,
  ),
  AnonymousSubstructParentInterfaceKindRune(Arc<AnonymousSubstructParentInterfaceKindRuneS>),
  AnonymousSubstructParentInterfaceCoordRune(Arc<AnonymousSubstructParentInterfaceCoordRuneS>),
  AnonymousSubstructTemplateRune(Arc<AnonymousSubstructTemplateRuneS>),
  AnonymousSubstructKindRune(Arc<AnonymousSubstructKindRuneS>),
  AnonymousSubstructCoordRune(Arc<AnonymousSubstructCoordRuneS>),
  AnonymousSubstructVoidKindRune(Arc<AnonymousSubstructVoidKindRuneS>),
  AnonymousSubstructVoidCoordRune(Arc<AnonymousSubstructVoidCoordRuneS>),
  AnonymousSubstructMemberRune(Arc<AnonymousSubstructMemberRuneS>),
  AnonymousSubstructMethodSelfBorrowCoordRune(Arc<AnonymousSubstructMethodSelfBorrowCoordRuneS>),
  AnonymousSubstructMethodSelfOwnCoordRune(Arc<AnonymousSubstructMethodSelfOwnCoordRuneS>),
  AnonymousSubstructDropBoundPrototypeRune(Arc<AnonymousSubstructDropBoundPrototypeRuneS>),
  AnonymousSubstructDropBoundParamsListRune(Arc<AnonymousSubstructDropBoundParamsListRuneS>),
  AnonymousSubstructFunctionBoundPrototypeRune(Arc<AnonymousSubstructFunctionBoundPrototypeRuneS>),
  AnonymousSubstructFunctionBoundParamsListRune(Arc<AnonymousSubstructFunctionBoundParamsListRuneS>),
  AnonymousSubstructFunctionInterfaceTemplateRune(
    Arc<AnonymousSubstructFunctionInterfaceTemplateRuneS>,
  ),
  AnonymousSubstructFunctionInterfaceKindRune(Arc<AnonymousSubstructFunctionInterfaceKindRuneS>),
  AnonymousSubstructMethodInheritedRune(Arc<AnonymousSubstructMethodInheritedRuneS>),
  FunctorPrototypeRuneName(Arc<FunctorPrototypeRuneNameS>),
  FunctorParamRuneName(Arc<FunctorParamRuneNameS>),
  FunctorReturnRuneName(Arc<FunctorReturnRuneNameS>),
  DispatcherRuneFromImpl(Arc<DispatcherRuneFromImplS>),
  CaseRuneFromImpl(Arc<CaseRuneFromImplS>),
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
pub struct CodeRuneS {
  pub name: Arc<StrI>,
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
pub struct ImplicitRegionRuneS {
  pub original_rune: Box<IRuneS>,
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
pub struct DenizenDefaultRegionRuneS {
  pub denizen_name: INameS,
}

/*
case class DenizenDefaultRegionRuneS(denizenName: INameS) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExportDefaultRegionRuneS {
  pub denizen_name: INameS,
}
/*
case class ExportDefaultRegionRuneS(denizenName: INameS) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ExternDefaultRegionRuneS {
  pub denizen_name: INameS,
}
/*
case class ExternDefaultRegionRuneS(denizenName: INameS) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionOwnershipRuneS {
  pub range: RangeS,
  pub original_coord_rune: Box<IRuneS>,
}
/*
case class ImplicitCoercionOwnershipRuneS(range: RangeS, originalCoordRune: IRuneS) extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionKindRuneS {
  pub range: RangeS,
  pub original_coord_rune: Box<IRuneS>,
}
/*
case class ImplicitCoercionKindRuneS(range: RangeS, originalCoordRune: IRuneS) extends IRuneS {  }
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplicitCoercionTemplateRuneS {
  pub range: RangeS,
  pub original_kind_rune: Box<IRuneS>,
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
pub struct StructNameRuneS {
  pub struct_name: TopLevelCitizenDeclarationNameS,
}
/*
case class StructNameRuneS(structName: ICitizenDeclarationNameS) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct InterfaceNameRuneS {
  pub interface_name: TopLevelCitizenDeclarationNameS,
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
pub struct SelfKindTemplateRuneS {
  pub loc: CodeLocationS,
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
pub struct CodeNameS {
  pub name: Arc<StrI>,
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
pub struct PatternInputRuneS {
  pub code_loc: CodeLocationS,
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
pub struct AnonymousSubstructMemberRuneS {
  pub interface: TopLevelInterfaceDeclarationNameS,
  pub method: IFunctionDeclarationNameS,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMethodSelfBorrowCoordRuneS {
  pub interface: TopLevelInterfaceDeclarationNameS,
  pub method: IFunctionDeclarationNameS,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMethodSelfOwnCoordRuneS {
  pub interface: TopLevelInterfaceDeclarationNameS,
  pub method: IFunctionDeclarationNameS,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructDropBoundPrototypeRuneS {
  pub interface: TopLevelInterfaceDeclarationNameS,
  pub method: IFunctionDeclarationNameS,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructDropBoundParamsListRuneS {
  pub interface: TopLevelInterfaceDeclarationNameS,
  pub method: IFunctionDeclarationNameS,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructFunctionBoundPrototypeRuneS {
  pub interface: TopLevelInterfaceDeclarationNameS,
  pub method: IFunctionDeclarationNameS,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructFunctionBoundParamsListRuneS {
  pub interface: TopLevelInterfaceDeclarationNameS,
  pub method: IFunctionDeclarationNameS,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructFunctionInterfaceTemplateRuneS {
  pub interface: TopLevelInterfaceDeclarationNameS,
  pub method: IFunctionDeclarationNameS,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructFunctionInterfaceKindRuneS {
  pub interface: TopLevelInterfaceDeclarationNameS,
  pub method: IFunctionDeclarationNameS,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AnonymousSubstructMethodInheritedRuneS {
  pub interface: TopLevelInterfaceDeclarationNameS,
  pub method: IFunctionDeclarationNameS,
  pub inner: Box<IRuneS>,
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
pub struct DispatcherRuneFromImplS {
  pub inner_rune: Box<IRuneS>,
}
/*
case class DispatcherRuneFromImplS(innerRune: IRuneS) extends IRuneS
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CaseRuneFromImplS {
  pub inner_rune: Box<IRuneS>,
}
/*
case class CaseRuneFromImplS(innerRune: IRuneS) extends IRuneS

// Only made by typingpass, see if we can take these out
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ConstructorNameS {
  pub tlcd: TopLevelCitizenDeclarationNameS,
}
/*
case class ConstructorNameS(tlcd: ICitizenDeclarationNameS) extends IFunctionDeclarationNameS {
  override def packageCoordinate: PackageCoordinate = tlcd.range.begin.file.packageCoordinate
  override def getImpreciseName(interner: Interner): IImpreciseNameS = tlcd.getImpreciseName(interner)
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImmConcreteDestructorNameS {
  pub package_coordinate: PackageCoordinate,
}
/*
case class ImmConcreteDestructorNameS(packageCoordinate: PackageCoordinate) extends IFunctionDeclarationNameS {
  override def getImpreciseName(interner: Interner): IImpreciseNameS = vimpl()
}
*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImmInterfaceDestructorNameS {
  pub package_coordinate: PackageCoordinate,
}
/*
case class ImmInterfaceDestructorNameS(packageCoordinate: PackageCoordinate) extends IFunctionDeclarationNameS {
  override def getImpreciseName(interner: Interner): IImpreciseNameS = vimpl()
}

*/
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplImpreciseNameS {
  pub sub_citizen_imprecise_name: Box<IImpreciseNameS>,
  pub super_interface_imprecise_name: Box<IImpreciseNameS>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplSubCitizenImpreciseNameS {
  pub sub_citizen_imprecise_name: Box<IImpreciseNameS>,
}
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplSuperInterfaceImpreciseNameS {
  pub super_interface_imprecise_name: Box<IImpreciseNameS>,
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