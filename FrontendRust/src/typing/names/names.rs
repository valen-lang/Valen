/*
package dev.vale.typing.names

import dev.vale.postparsing._
import dev.vale.typing.ast.LocationInFunctionEnvironmentT
import dev.vale.typing.expression.CallCompiler
import dev.vale._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.typing.templata.ITemplataT._
import dev.vale.typing.types._

// Scout's/Astronomer's name parts correspond to where they are in the source code,
// but Compiler's correspond more to what packages and stamped functions / structs
// they're in. See TNAD.
*/
use crate::interner::StrI;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::{CodeLocationS, RangeS};
use crate::postparsing::names::IRuneS;
use crate::typing::types::types::{CoordT, RegionT, ICitizenTT};
use crate::typing::templata::templata::ITemplataT;
use crate::typing::ast::ast::LocationInFunctionEnvironmentT;

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IdT<'s, 't, T: Copy>
where 's: 't,
{
    pub package_coord: &'s PackageCoordinate<'s>,
    pub init_steps: &'t [&'t INameT<'s, 't>],
    pub local_name: T,
}
/*
case class IdT[+T <: INameT](
  packageCoord: PackageCoordinate,
  initSteps: Vector[INameT],
  localName: T
)  {
  this match {
    case _ =>
  }

  // Placeholders should only be the last name, getPlaceholdersInKind assumes it
  initSteps.foreach({
    case KindPlaceholderNameT(_) => vfail()
    case KindPlaceholderTemplateNameT(_, _) => vfail()
    case _ =>
  })
  // Placeholders are under the template name.
  // There's really no other way; we make the placeholders before knowing the function's
  // instantated name.
  localName match {
    case KindPlaceholderNameT(_) => {
      initSteps.last match {
        case _ : ITemplateNameT =>
        case OverrideDispatcherNameT(_, _, _) => {
          initSteps.init.last match {
            case _ : ITemplateNameT =>
            case other => vfail(other)
          }
        }
        case other => vfail(other)
      }
    }
    case _ =>
  }

  // PackageTopLevelName2 is just here because names have to have a last step.
  vassert(initSteps.collectFirst({ case PackageTopLevelNameT() => }).isEmpty)

  vcurious(initSteps.distinct == initSteps)

  override def equals(obj: Any): Boolean = {
    obj match {
      case IdT(thatPackageCoord, thatInitSteps, thatLast) => {
        packageCoord == thatPackageCoord && initSteps == thatInitSteps && localName == thatLast
      }
      case _ => false
    }
  }

  def packageId(interner: Interner): IdT[PackageTopLevelNameT] = {
    IdT(packageCoord, Vector(), interner.intern(PackageTopLevelNameT()))
  }

  def initId(interner: Interner): IdT[INameT] = {
    if (initSteps.isEmpty) {
      IdT(packageCoord, Vector(), interner.intern(PackageTopLevelNameT()))
    } else {
      IdT(packageCoord, initSteps.init, initSteps.last)
    }
  }

  def initNonPackageId(): Option[IdT[INameT]] = {
    if (initSteps.isEmpty) {
      None
    } else {
      Some(IdT(packageCoord, initSteps.init, initSteps.last))
    }
  }

  def steps: Vector[INameT] = {
    localName match {
      case PackageTopLevelNameT() => initSteps
      case _ => initSteps :+ localName
    }
  }
  def addStep[Y <: INameT](newLast: Y): IdT[Y] = {
    IdT[Y](packageCoord, steps, newLast)
  }
}

*/
// (no scala counterpart — narrow -> wide conversion per handoff §6.3)
impl<'s, 't, T> IdT<'s, 't, T>
where 's: 't, T: Copy + Into<&'t INameT<'s, 't>>,
{
    pub fn widen(self) -> IdT<'s, 't, &'t INameT<'s, 't>> {
        IdT {
            package_coord: self.package_coord,
            init_steps: self.init_steps,
            local_name: self.local_name.into(),
        }
    }
}

// (no scala counterpart — generic upcast per handoff §6.3)
impl<'s, 't, T> IdT<'s, 't, T>
where 's: 't, T: Copy,
{
    pub fn widen_to<U: Copy>(self) -> IdT<'s, 't, U>
    where T: Into<U>,
    {
        IdT {
            package_coord: self.package_coord,
            init_steps: self.init_steps,
            local_name: self.local_name.into(),
        }
    }
}

// (no scala counterpart — wide -> narrow conversion per handoff §6.3)
impl<'s, 't> IdT<'s, 't, &'t INameT<'s, 't>>
where 's: 't,
{
    pub fn try_narrow<U: Copy>(self) -> Option<IdT<'s, 't, U>>
    where &'t INameT<'s, 't>: TryInto<U>,
    {
        self.local_name.try_into().ok().map(|local_name| IdT {
            package_coord: self.package_coord,
            init_steps: self.init_steps,
            local_name,
        })
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum INameT<'s, 't> {
    ExportTemplate(&'t ExportTemplateNameT<'s, 't>),
    Export(&'t ExportNameT<'s, 't>),
    ImplTemplate(&'t ImplTemplateNameT<'s, 't>),
    Impl(&'t ImplNameT<'s, 't>),
    ImplBoundTemplate(&'t ImplBoundTemplateNameT<'s, 't>),
    ImplBound(&'t ImplBoundNameT<'s, 't>),
    Let(&'t LetNameT<'s, 't>),
    ExportAs(&'t ExportAsNameT<'s, 't>),
    RawArray(&'t RawArrayNameT<'s, 't>),
    ReachablePrototype(&'t ReachablePrototypeNameT<'s, 't>),
    StaticSizedArrayTemplate(&'t StaticSizedArrayTemplateNameT<'s, 't>),
    StaticSizedArray(&'t StaticSizedArrayNameT<'s, 't>),
    RuntimeSizedArrayTemplate(&'t RuntimeSizedArrayTemplateNameT<'s, 't>),
    RuntimeSizedArray(&'t RuntimeSizedArrayNameT<'s, 't>),
    KindPlaceholderTemplate(&'t KindPlaceholderTemplateNameT<'s, 't>),
    KindPlaceholder(&'t KindPlaceholderNameT<'s, 't>),
    NonKindNonRegionPlaceholder(&'t NonKindNonRegionPlaceholderNameT<'s, 't>),
    OverrideDispatcherTemplate(&'t OverrideDispatcherTemplateNameT<'s, 't>),
    OverrideDispatcher(&'t OverrideDispatcherNameT<'s, 't>),
    OverrideDispatcherCase(&'t OverrideDispatcherCaseNameT<'s, 't>),
    TypingPassBlockResultVar(&'t TypingPassBlockResultVarNameT<'s, 't>),
    TypingPassFunctionResultVar(&'t TypingPassFunctionResultVarNameT<'s, 't>),
    TypingPassTemporaryVar(&'t TypingPassTemporaryVarNameT<'s, 't>),
    TypingPassPatternMember(&'t TypingPassPatternMemberNameT<'s, 't>),
    TypingIgnoredParam(&'t TypingIgnoredParamNameT<'s, 't>),
    TypingPassPatternDestructuree(&'t TypingPassPatternDestructureeNameT<'s, 't>),
    UnnamedLocal(&'t UnnamedLocalNameT<'s, 't>),
    ClosureParam(&'t ClosureParamNameT<'s, 't>),
    ConstructingMember(&'t ConstructingMemberNameT<'s, 't>),
    WhileCondResult(&'t WhileCondResultNameT<'s, 't>),
    Iterable(&'t IterableNameT<'s, 't>),
    Iterator(&'t IteratorNameT<'s, 't>),
    IterationOption(&'t IterationOptionNameT<'s, 't>),
    MagicParam(&'t MagicParamNameT<'s, 't>),
    CodeVar(&'t CodeVarNameT<'s, 't>),
    AnonymousSubstructMember(&'t AnonymousSubstructMemberNameT<'s, 't>),
    Primitive(&'t PrimitiveNameT<'s, 't>),
    PackageTopLevel(&'t PackageTopLevelNameT<'s, 't>),
    Project(&'t ProjectNameT<'s, 't>),
    Package(&'t PackageNameT<'s, 't>),
    Rune(&'t RuneNameT<'s, 't>),
    BuildingFunctionNameWithClosureds(&'t BuildingFunctionNameWithClosuredsT<'s, 't>),
    ExternTemplate(&'t ExternTemplateNameT<'s, 't>),
    Extern(&'t ExternNameT<'s, 't>),
    ExternFunction(&'t ExternFunctionNameT<'s, 't>),
    Function(&'t FunctionNameT<'s, 't>),
    ForwarderFunction(&'t ForwarderFunctionNameT<'s, 't>),
    FunctionBoundTemplate(&'t FunctionBoundTemplateNameT<'s, 't>),
    FunctionBound(&'t FunctionBoundNameT<'s, 't>),
    PredictedFunctionTemplate(&'t PredictedFunctionTemplateNameT<'s, 't>),
    PredictedFunction(&'t PredictedFunctionNameT<'s, 't>),
    FunctionTemplate(&'t FunctionTemplateNameT<'s, 't>),
    LambdaCallFunctionTemplate(&'t LambdaCallFunctionTemplateNameT<'s, 't>),
    LambdaCallFunction(&'t LambdaCallFunctionNameT<'s, 't>),
    ForwarderFunctionTemplate(&'t ForwarderFunctionTemplateNameT<'s, 't>),
    ConstructorTemplate(&'t ConstructorTemplateNameT<'s, 't>),
    Self_(&'t SelfNameT<'s, 't>),
    Arbitrary(&'t ArbitraryNameT<'s, 't>),
    Struct(&'t StructNameT<'s, 't>),
    Interface(&'t InterfaceNameT<'s, 't>),
    LambdaCitizenTemplate(&'t LambdaCitizenTemplateNameT<'s, 't>),
    LambdaCitizen(&'t LambdaCitizenNameT<'s, 't>),
    StructTemplate(&'t StructTemplateNameT<'s, 't>),
    InterfaceTemplate(&'t InterfaceTemplateNameT<'s, 't>),
    AnonymousSubstructImplTemplate(&'t AnonymousSubstructImplTemplateNameT<'s, 't>),
    AnonymousSubstructImpl(&'t AnonymousSubstructImplNameT<'s, 't>),
    AnonymousSubstructTemplate(&'t AnonymousSubstructTemplateNameT<'s, 't>),
    AnonymousSubstructConstructorTemplate(&'t AnonymousSubstructConstructorTemplateNameT<'s, 't>),
    AnonymousSubstructConstructor(&'t AnonymousSubstructConstructorNameT<'s, 't>),
    AnonymousSubstruct(&'t AnonymousSubstructNameT<'s, 't>),
    ResolvingEnv(&'t ResolvingEnvNameT<'s, 't>),
    CallEnv(&'t CallEnvNameT<'s, 't>),
}
/*
sealed trait INameT extends IInterning
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ITemplateNameT<'s, 't> {
    ExportTemplate(&'t ExportTemplateNameT<'s, 't>),
    ImplTemplate(&'t ImplTemplateNameT<'s, 't>),
    ImplBoundTemplate(&'t ImplBoundTemplateNameT<'s, 't>),
    StaticSizedArrayTemplate(&'t StaticSizedArrayTemplateNameT<'s, 't>),
    RuntimeSizedArrayTemplate(&'t RuntimeSizedArrayTemplateNameT<'s, 't>),
    KindPlaceholderTemplate(&'t KindPlaceholderTemplateNameT<'s, 't>),
    OverrideDispatcherTemplate(&'t OverrideDispatcherTemplateNameT<'s, 't>),
    OverrideDispatcherCase(&'t OverrideDispatcherCaseNameT<'s, 't>),
    ExternTemplate(&'t ExternTemplateNameT<'s, 't>),
    ExternFunction(&'t ExternFunctionNameT<'s, 't>),
    FunctionBoundTemplate(&'t FunctionBoundTemplateNameT<'s, 't>),
    PredictedFunctionTemplate(&'t PredictedFunctionTemplateNameT<'s, 't>),
    FunctionTemplate(&'t FunctionTemplateNameT<'s, 't>),
    LambdaCallFunctionTemplate(&'t LambdaCallFunctionTemplateNameT<'s, 't>),
    ForwarderFunctionTemplate(&'t ForwarderFunctionTemplateNameT<'s, 't>),
    ConstructorTemplate(&'t ConstructorTemplateNameT<'s, 't>),
    LambdaCitizenTemplate(&'t LambdaCitizenTemplateNameT<'s, 't>),
    StructTemplate(&'t StructTemplateNameT<'s, 't>),
    InterfaceTemplate(&'t InterfaceTemplateNameT<'s, 't>),
    AnonymousSubstructImplTemplate(&'t AnonymousSubstructImplTemplateNameT<'s, 't>),
    AnonymousSubstructTemplate(&'t AnonymousSubstructTemplateNameT<'s, 't>),
    AnonymousSubstructConstructorTemplate(&'t AnonymousSubstructConstructorTemplateNameT<'s, 't>),
}
/*
sealed trait ITemplateNameT extends INameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IFunctionTemplateNameT<'s, 't> {
    OverrideDispatcherTemplate(&'t OverrideDispatcherTemplateNameT<'s, 't>),
    ExternFunction(&'t ExternFunctionNameT<'s, 't>),
    FunctionBoundTemplate(&'t FunctionBoundTemplateNameT<'s, 't>),
    PredictedFunctionTemplate(&'t PredictedFunctionTemplateNameT<'s, 't>),
    FunctionTemplate(&'t FunctionTemplateNameT<'s, 't>),
    LambdaCallFunctionTemplate(&'t LambdaCallFunctionTemplateNameT<'s, 't>),
    ForwarderFunctionTemplate(&'t ForwarderFunctionTemplateNameT<'s, 't>),
    ConstructorTemplate(&'t ConstructorTemplateNameT<'s, 't>),
    AnonymousSubstructConstructorTemplate(&'t AnonymousSubstructConstructorTemplateNameT<'s, 't>),
}
/*
sealed trait IFunctionTemplateNameT extends ITemplateNameT {
  def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): IFunctionNameT
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IInstantiationNameT<'s, 't> {
    Export(&'t ExportNameT<'s, 't>),
    Impl(&'t ImplNameT<'s, 't>),
    ImplBound(&'t ImplBoundNameT<'s, 't>),
    StaticSizedArray(&'t StaticSizedArrayNameT<'s, 't>),
    RuntimeSizedArray(&'t RuntimeSizedArrayNameT<'s, 't>),
    KindPlaceholder(&'t KindPlaceholderNameT<'s, 't>),
    OverrideDispatcher(&'t OverrideDispatcherNameT<'s, 't>),
    OverrideDispatcherCase(&'t OverrideDispatcherCaseNameT<'s, 't>),
    Extern(&'t ExternNameT<'s, 't>),
    ExternFunction(&'t ExternFunctionNameT<'s, 't>),
    Function(&'t FunctionNameT<'s, 't>),
    ForwarderFunction(&'t ForwarderFunctionNameT<'s, 't>),
    FunctionBound(&'t FunctionBoundNameT<'s, 't>),
    PredictedFunction(&'t PredictedFunctionNameT<'s, 't>),
    LambdaCallFunction(&'t LambdaCallFunctionNameT<'s, 't>),
    Struct(&'t StructNameT<'s, 't>),
    Interface(&'t InterfaceNameT<'s, 't>),
    LambdaCitizen(&'t LambdaCitizenNameT<'s, 't>),
    AnonymousSubstructImpl(&'t AnonymousSubstructImplNameT<'s, 't>),
    AnonymousSubstructConstructor(&'t AnonymousSubstructConstructorNameT<'s, 't>),
    AnonymousSubstruct(&'t AnonymousSubstructNameT<'s, 't>),
}
/*
sealed trait IInstantiationNameT extends INameT {
  def template: ITemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IFunctionNameT<'s, 't> {
    OverrideDispatcher(&'t OverrideDispatcherNameT<'s, 't>),
    ExternFunction(&'t ExternFunctionNameT<'s, 't>),
    Function(&'t FunctionNameT<'s, 't>),
    ForwarderFunction(&'t ForwarderFunctionNameT<'s, 't>),
    FunctionBound(&'t FunctionBoundNameT<'s, 't>),
    PredictedFunction(&'t PredictedFunctionNameT<'s, 't>),
    LambdaCallFunction(&'t LambdaCallFunctionNameT<'s, 't>),
    AnonymousSubstructConstructor(&'t AnonymousSubstructConstructorNameT<'s, 't>),
}
/*
sealed trait IFunctionNameT extends IInstantiationNameT {
  def template: IFunctionTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
  def parameters: Vector[CoordT]
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ISuperKindTemplateNameT<'s, 't> {
    KindPlaceholderTemplate(&'t KindPlaceholderTemplateNameT<'s, 't>),
    InterfaceTemplate(&'t InterfaceTemplateNameT<'s, 't>),
}
/*
sealed trait ISuperKindTemplateNameT extends ITemplateNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ISubKindTemplateNameT<'s, 't> {
    StaticSizedArrayTemplate(&'t StaticSizedArrayTemplateNameT<'s, 't>),
    RuntimeSizedArrayTemplate(&'t RuntimeSizedArrayTemplateNameT<'s, 't>),
    KindPlaceholderTemplate(&'t KindPlaceholderTemplateNameT<'s, 't>),
    LambdaCitizenTemplate(&'t LambdaCitizenTemplateNameT<'s, 't>),
    StructTemplate(&'t StructTemplateNameT<'s, 't>),
    InterfaceTemplate(&'t InterfaceTemplateNameT<'s, 't>),
    AnonymousSubstructTemplate(&'t AnonymousSubstructTemplateNameT<'s, 't>),
}
/*
sealed trait ISubKindTemplateNameT extends ITemplateNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ICitizenTemplateNameT<'s, 't> {
    StaticSizedArrayTemplate(&'t StaticSizedArrayTemplateNameT<'s, 't>),
    RuntimeSizedArrayTemplate(&'t RuntimeSizedArrayTemplateNameT<'s, 't>),
    LambdaCitizenTemplate(&'t LambdaCitizenTemplateNameT<'s, 't>),
    StructTemplate(&'t StructTemplateNameT<'s, 't>),
    InterfaceTemplate(&'t InterfaceTemplateNameT<'s, 't>),
    AnonymousSubstructTemplate(&'t AnonymousSubstructTemplateNameT<'s, 't>),
}
/*
sealed trait ICitizenTemplateNameT extends ISubKindTemplateNameT {
  def makeCitizenName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): ICitizenNameT
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IStructTemplateNameT<'s, 't> {
    LambdaCitizenTemplate(&'t LambdaCitizenTemplateNameT<'s, 't>),
    StructTemplate(&'t StructTemplateNameT<'s, 't>),
    AnonymousSubstructTemplate(&'t AnonymousSubstructTemplateNameT<'s, 't>),
}
/*
sealed trait IStructTemplateNameT extends ICitizenTemplateNameT {
  def makeStructName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): IStructNameT
  override def makeCitizenName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]):
  ICitizenNameT = {
    makeStructName(interner, templateArgs)
  }
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IInterfaceTemplateNameT<'s, 't> {
    InterfaceTemplate(&'t InterfaceTemplateNameT<'s, 't>),
}
/*
sealed trait IInterfaceTemplateNameT extends ICitizenTemplateNameT with ISuperKindTemplateNameT {
  def makeInterfaceName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): IInterfaceNameT
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ISuperKindNameT<'s, 't> {
    KindPlaceholder(&'t KindPlaceholderNameT<'s, 't>),
    Interface(&'t InterfaceNameT<'s, 't>),
}
/*
sealed trait ISuperKindNameT extends IInstantiationNameT {
  def template: ISuperKindTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ISubKindNameT<'s, 't> {
    StaticSizedArray(&'t StaticSizedArrayNameT<'s, 't>),
    RuntimeSizedArray(&'t RuntimeSizedArrayNameT<'s, 't>),
    KindPlaceholder(&'t KindPlaceholderNameT<'s, 't>),
    Struct(&'t StructNameT<'s, 't>),
    Interface(&'t InterfaceNameT<'s, 't>),
    LambdaCitizen(&'t LambdaCitizenNameT<'s, 't>),
    AnonymousSubstruct(&'t AnonymousSubstructNameT<'s, 't>),
}
/*
sealed trait ISubKindNameT extends IInstantiationNameT {
  def template: ISubKindTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ICitizenNameT<'s, 't> {
    StaticSizedArray(&'t StaticSizedArrayNameT<'s, 't>),
    RuntimeSizedArray(&'t RuntimeSizedArrayNameT<'s, 't>),
    Struct(&'t StructNameT<'s, 't>),
    Interface(&'t InterfaceNameT<'s, 't>),
    LambdaCitizen(&'t LambdaCitizenNameT<'s, 't>),
    AnonymousSubstruct(&'t AnonymousSubstructNameT<'s, 't>),
}
/*
sealed trait ICitizenNameT extends ISubKindNameT {
  def template: ICitizenTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IStructNameT<'s, 't> {
    Struct(&'t StructNameT<'s, 't>),
    LambdaCitizen(&'t LambdaCitizenNameT<'s, 't>),
    AnonymousSubstruct(&'t AnonymousSubstructNameT<'s, 't>),
}
/*
sealed trait IStructNameT extends ICitizenNameT with ISubKindNameT {
  override def template: IStructTemplateNameT
  override def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IInterfaceNameT<'s, 't> {
    Interface(&'t InterfaceNameT<'s, 't>),
}
/*
sealed trait IInterfaceNameT extends ICitizenNameT with ISubKindNameT with ISuperKindNameT {
  override def template: InterfaceTemplateNameT
  override def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IImplTemplateNameT<'s, 't> {
    ImplTemplate(&'t ImplTemplateNameT<'s, 't>),
    ImplBoundTemplate(&'t ImplBoundTemplateNameT<'s, 't>),
    AnonymousSubstructImplTemplate(&'t AnonymousSubstructImplTemplateNameT<'s, 't>),
}
/*
sealed trait IImplTemplateNameT extends ITemplateNameT {
  def makeImplName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]], subCitizen: ICitizenTT): IImplNameT
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IImplNameT<'s, 't> {
    Impl(&'t ImplNameT<'s, 't>),
    ImplBound(&'t ImplBoundNameT<'s, 't>),
    AnonymousSubstructImpl(&'t AnonymousSubstructImplNameT<'s, 't>),
}
/*
sealed trait IImplNameT extends IInstantiationNameT {
  def template: IImplTemplateNameT
}

*/
// TODO: placeholder PhantomData — replace with real fields during body migration
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IRegionNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait IRegionNameT extends INameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExportTemplateNameT<'s, 't> {
    pub code_loc: CodeLocationS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class ExportTemplateNameT(codeLoc: CodeLocationS) extends ITemplateNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExportNameT<'s, 't> {
    pub template: &'t ExportTemplateNameT<'s, 't>,
    pub region: RegionT,
}
/*
case class ExportNameT(template: ExportTemplateNameT, region: RegionT) extends IInstantiationNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = Vector()
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ImplTemplateNameT<'s, 't> {
    pub code_location_s: CodeLocationS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class ImplTemplateNameT(codeLocationS: CodeLocationS) extends IImplTemplateNameT {
  vpass()
  override def makeImplName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]], subCitizen: ICitizenTT): ImplNameT = {
    interner.intern(ImplNameT(this, templateArgs, subCitizen))
  }
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ImplNameT<'s, 't> {
    pub template: &'t ImplTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub sub_citizen: ICitizenTT<'s, 't>,
}
/*
case class ImplNameT(
  template: ImplTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  // The instantiator wants this so it can know the struct type up-front before monomorphizing the
  // whole impl, so it can hoist some bounds out of the struct, like NBIFP.
  subCitizen: ICitizenTT
) extends IImplNameT {
  vpass()
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ImplBoundTemplateNameT<'s, 't> {
    pub code_location_s: CodeLocationS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class ImplBoundTemplateNameT(codeLocationS: CodeLocationS) extends IImplTemplateNameT {
  override def makeImplName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]], subCitizen: ICitizenTT): ImplBoundNameT = {
    interner.intern(ImplBoundNameT(this, templateArgs))
  }
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ImplBoundNameT<'s, 't> {
    pub template: &'t ImplBoundTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
}
/*
case class ImplBoundNameT(
  template: ImplBoundTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]]
) extends IImplNameT {

}

//// The name of an impl that is subclassing some interface. To find all impls subclassing an interface,
//// look for this name.
//case class ImplImplementingSuperInterfaceNameT(superInterface: FullNameT[IInterfaceTemplateNameT]) extends IImplTemplateNameT
//// The name of an impl that is augmenting some sub citizen. To find all impls subclassing an interface,
//// look for this name.
//case class ImplAugmentingSubCitizenNameT(subCitizen: FullNameT[ICitizenTemplateNameT]) extends IImplTemplateNameT

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LetNameT<'s, 't> {
    pub code_location: CodeLocationS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class LetNameT(codeLocation: CodeLocationS) extends INameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExportAsNameT<'s, 't> {
    pub code_location: CodeLocationS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class ExportAsNameT(codeLocation: CodeLocationS) extends INameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RawArrayNameT<'s, 't> {
    pub mutability: ITemplataT<'s, 't>,
    pub element_type: CoordT<'s, 't>,
    pub self_region: RegionT,
}
/*
case class RawArrayNameT(
  mutability: ITemplataT[MutabilityTemplataType],
  elementType: CoordT,
  selfRegion: RegionT
) extends INameT

// This num is really just here to disambiguate it from other reachable prototypes in the environment
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ReachablePrototypeNameT<'s, 't> {
    pub num: i32,
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
/*
case class ReachablePrototypeNameT(num: Int) extends INameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayTemplateNameT<'s, 't> {
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
/*
case class StaticSizedArrayTemplateNameT() extends ICitizenTemplateNameT {
  override def makeCitizenName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): ICitizenNameT = {
    vassert(templateArgs.size == 4)
    val size = expectInteger(templateArgs(0))
    val mutability = expectMutability(templateArgs(1))
    val variability = expectVariability(templateArgs(2))
    val elementType = expectCoordTemplata(templateArgs(3)).coord
    val selfRegion = vregionmut(RegionT())
    interner.intern(StaticSizedArrayNameT(this, size, variability, interner.intern(RawArrayNameT(mutability, elementType, selfRegion))))
  }
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayNameT<'s, 't> {
    pub template: &'t StaticSizedArrayTemplateNameT<'s, 't>,
    pub size: ITemplataT<'s, 't>,
    pub variability: ITemplataT<'s, 't>,
    pub arr: &'t RawArrayNameT<'s, 't>,
}
/*
case class StaticSizedArrayNameT(
  template: StaticSizedArrayTemplateNameT,
  size: ITemplataT[IntegerTemplataType],
  variability: ITemplataT[VariabilityTemplataType],
  arr: RawArrayNameT) extends ICitizenNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = {
    Vector(size, arr.mutability, variability, CoordTemplataT(arr.elementType))
  }
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayTemplateNameT<'s, 't> {
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
/*
case class RuntimeSizedArrayTemplateNameT() extends ICitizenTemplateNameT {
  override def makeCitizenName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): ICitizenNameT = {
    vassert(templateArgs.size == 2)
    val mutability = expectMutability(templateArgs(0))
    val elementType = expectCoordTemplata(templateArgs(1)).coord
    val region = vregionmut(RegionT())
    interner.intern(RuntimeSizedArrayNameT(this, interner.intern(RawArrayNameT(mutability, elementType, region))))
  }
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayNameT<'s, 't> {
    pub template: &'t RuntimeSizedArrayTemplateNameT<'s, 't>,
    pub arr: &'t RawArrayNameT<'s, 't>,
}
/*
case class RuntimeSizedArrayNameT(template: RuntimeSizedArrayTemplateNameT, arr: RawArrayNameT) extends ICitizenNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = {
    Vector(arr.mutability, CoordTemplataT(arr.elementType))
  }
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IPlaceholderNameT<'s, 't> {
    KindPlaceholder(&'t KindPlaceholderNameT<'s, 't>),
    NonKindNonRegionPlaceholder(&'t NonKindNonRegionPlaceholderNameT<'s, 't>),
}
/*
sealed trait IPlaceholderNameT extends INameT {
  def index: Int
  def rune: IRuneS
}

// This exists because PlaceholderT is a kind, and all kinds need environments to assist
// in call/overload resolution. Environments are associated with templates, so it makes
// some sense to have a "placeholder template" notion.
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct KindPlaceholderTemplateNameT<'s, 't> {
    pub index: i32,
    pub rune: IRuneS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class KindPlaceholderTemplateNameT(index: Int, rune: IRuneS) extends ISubKindTemplateNameT with ISuperKindTemplateNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct KindPlaceholderNameT<'s, 't> {
    pub template: &'t KindPlaceholderTemplateNameT<'s, 't>,
}
/*
case class KindPlaceholderNameT(template: KindPlaceholderTemplateNameT) extends IPlaceholderNameT with ISubKindNameT with ISuperKindNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = Vector()
  override def rune: IRuneS = template.rune
  override def index: Int = template.index
}

// This exists because we need a different way to refer to a coord generic param's other components,
// see MNRFGC.
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct NonKindNonRegionPlaceholderNameT<'s, 't> {
    pub index: i32,
    pub rune: IRuneS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class NonKindNonRegionPlaceholderNameT(index: Int, rune: IRuneS) extends IPlaceholderNameT

// See NNSPAFOC.
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OverrideDispatcherTemplateNameT<'s, 't> {
    pub impl_id: IdT<'s, 't, &'t IImplTemplateNameT<'s, 't>>,
}
/*
case class OverrideDispatcherTemplateNameT(
  implId: IdT[IImplTemplateNameT]
) extends IFunctionTemplateNameT {
  override def makeFunctionName(
    interner: Interner,
    keywords: Keywords,
    templateArgs: Vector[ITemplataT[ITemplataType]],
    params: Vector[CoordT]):
  OverrideDispatcherNameT = {
    interner.intern(OverrideDispatcherNameT(this, templateArgs, params))
  }
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OverrideDispatcherNameT<'s, 't> {
    pub template: &'t OverrideDispatcherTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub parameters: &'t [CoordT<'s, 't>],
}
/*
case class OverrideDispatcherNameT(
  template: OverrideDispatcherTemplateNameT,
  // This will have placeholders in it after the typing pass.
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT {
  vpass()
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OverrideDispatcherCaseNameT<'s, 't> {
    pub independent_impl_template_args: &'t [ITemplataT<'s, 't>],
}
/*
case class OverrideDispatcherCaseNameT(
  // These are the templatas for the independent runes from the impl, like the <ZZ> for Milano, see
  // OMCNAGP.
  independentImplTemplateArgs: Vector[ITemplataT[ITemplataType]]
) extends ITemplateNameT with IInstantiationNameT {
  override def template: ITemplateNameT = this
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = independentImplTemplateArgs
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IVarNameT<'s, 't> {
    TypingPassBlockResultVar(&'t TypingPassBlockResultVarNameT<'s, 't>),
    TypingPassFunctionResultVar(&'t TypingPassFunctionResultVarNameT<'s, 't>),
    TypingPassTemporaryVar(&'t TypingPassTemporaryVarNameT<'s, 't>),
    TypingPassPatternMember(&'t TypingPassPatternMemberNameT<'s, 't>),
    TypingIgnoredParam(&'t TypingIgnoredParamNameT<'s, 't>),
    TypingPassPatternDestructuree(&'t TypingPassPatternDestructureeNameT<'s, 't>),
    UnnamedLocal(&'t UnnamedLocalNameT<'s, 't>),
    ClosureParam(&'t ClosureParamNameT<'s, 't>),
    ConstructingMember(&'t ConstructingMemberNameT<'s, 't>),
    WhileCondResult(&'t WhileCondResultNameT<'s, 't>),
    Iterable(&'t IterableNameT<'s, 't>),
    Iterator(&'t IteratorNameT<'s, 't>),
    IterationOption(&'t IterationOptionNameT<'s, 't>),
    MagicParam(&'t MagicParamNameT<'s, 't>),
    CodeVar(&'t CodeVarNameT<'s, 't>),
    AnonymousSubstructMember(&'t AnonymousSubstructMemberNameT<'s, 't>),
    Self_(&'t SelfNameT<'s, 't>),
}
/*
sealed trait IVarNameT extends INameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TypingPassBlockResultVarNameT<'s, 't> {
    pub life: &'s LocationInFunctionEnvironmentT<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class TypingPassBlockResultVarNameT(life: LocationInFunctionEnvironmentT) extends IVarNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TypingPassFunctionResultVarNameT<'s, 't> {
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
/*
case class TypingPassFunctionResultVarNameT() extends IVarNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TypingPassTemporaryVarNameT<'s, 't> {
    pub life: &'s LocationInFunctionEnvironmentT<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class TypingPassTemporaryVarNameT(life: LocationInFunctionEnvironmentT) extends IVarNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TypingPassPatternMemberNameT<'s, 't> {
    pub life: &'s LocationInFunctionEnvironmentT<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class TypingPassPatternMemberNameT(life: LocationInFunctionEnvironmentT) extends IVarNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TypingIgnoredParamNameT<'s, 't> {
    pub num: i32,
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
/*
case class TypingIgnoredParamNameT(num: Int) extends IVarNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct TypingPassPatternDestructureeNameT<'s, 't> {
    pub life: &'s LocationInFunctionEnvironmentT<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class TypingPassPatternDestructureeNameT(life: LocationInFunctionEnvironmentT) extends IVarNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct UnnamedLocalNameT<'s, 't> {
    pub code_location: CodeLocationS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class UnnamedLocalNameT(codeLocation: CodeLocationS) extends IVarNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ClosureParamNameT<'s, 't> {
    pub code_location: CodeLocationS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class ClosureParamNameT(codeLocation: CodeLocationS) extends IVarNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ConstructingMemberNameT<'s, 't> {
    pub name: StrI<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class ConstructingMemberNameT(name: StrI) extends IVarNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct WhileCondResultNameT<'s, 't> {
    pub range: RangeS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class WhileCondResultNameT(range: RangeS) extends IVarNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IterableNameT<'s, 't> {
    pub range: RangeS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class IterableNameT(range: RangeS) extends IVarNameT {  }
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IteratorNameT<'s, 't> {
    pub range: RangeS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class IteratorNameT(range: RangeS) extends IVarNameT {  }
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IterationOptionNameT<'s, 't> {
    pub range: RangeS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class IterationOptionNameT(range: RangeS) extends IVarNameT {  }
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MagicParamNameT<'s, 't> {
    pub code_location2: CodeLocationS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class MagicParamNameT(codeLocation2: CodeLocationS) extends IVarNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CodeVarNameT<'s, 't> {
    pub name: StrI<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class CodeVarNameT(name: StrI) extends IVarNameT
// We dont use CodeVarName2(0), CodeVarName2(1) etc because we dont want the user to address these members directly.
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AnonymousSubstructMemberNameT<'s, 't> {
    pub index: i32,
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
/*
case class AnonymousSubstructMemberNameT(index: Int) extends IVarNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrimitiveNameT<'s, 't> {
    pub human_name: StrI<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class PrimitiveNameT(humanName: StrI) extends INameT
// Only made in typingpass
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PackageTopLevelNameT<'s, 't> {
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
/*
case class PackageTopLevelNameT() extends INameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ProjectNameT<'s, 't> {
    pub name: StrI<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class ProjectNameT(name: StrI) extends INameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PackageNameT<'s, 't> {
    pub name: StrI<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class PackageNameT(name: StrI) extends INameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuneNameT<'s, 't> {
    pub rune: IRuneS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class RuneNameT(rune: IRuneS) extends INameT

// This is the name of a function that we're still figuring out in the function typingpass.
// We have its closured variables, but are still figuring out its template args and params.
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BuildingFunctionNameWithClosuredsT<'s, 't> {
    pub template_name: &'t IFunctionTemplateNameT<'s, 't>,
}
/*
case class BuildingFunctionNameWithClosuredsT(
  templateName: IFunctionTemplateNameT,
) extends INameT {



}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExternTemplateNameT<'s, 't> {
    pub code_loc: CodeLocationS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class ExternTemplateNameT(
  codeLoc: CodeLocationS,
) extends ITemplateNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExternNameT<'s, 't> {
    pub template: &'t ExternTemplateNameT<'s, 't>,
    pub template_arg: RegionT,
}
/*
case class ExternNameT(
  template: ExternTemplateNameT,
  templateArg: RegionT
) extends IInstantiationNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = Vector()
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExternFunctionNameT<'s, 't> {
    pub human_name: StrI<'s>,
    pub parameters: &'t [CoordT<'s, 't>],
}
/*
case class ExternFunctionNameT(
  humanName: StrI,
  parameters: Vector[CoordT]
) extends IFunctionNameT with IFunctionTemplateNameT {
  override def template: IFunctionTemplateNameT = this

  override def makeFunctionName(
    interner: Interner,
    keywords: Keywords,
    templateArgs: Vector[ITemplataT[ITemplataType]],
    params: Vector[CoordT]):
  IFunctionNameT = this

  override def templateArgs: Vector[ITemplataT[ITemplataType]] = Vector.empty
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FunctionNameT<'s, 't> {
    pub template: &'t FunctionTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub parameters: &'t [CoordT<'s, 't>],
}
/*
case class FunctionNameT(
  template: FunctionTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ForwarderFunctionNameT<'s, 't> {
    pub template: &'t ForwarderFunctionTemplateNameT<'s, 't>,
    pub inner: &'t IFunctionNameT<'s, 't>,
}
/*
case class ForwarderFunctionNameT(
  template: ForwarderFunctionTemplateNameT,
  inner: IFunctionNameT
) extends IFunctionNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = inner.templateArgs
  override def parameters: Vector[CoordT] = inner.parameters
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FunctionBoundTemplateNameT<'s, 't> {
    pub human_name: StrI<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class FunctionBoundTemplateNameT(
  humanName: StrI,
  // Removed this because we want various function bounds from various places to merge
  // together, see MFBFDP.
  // codeLocation: CodeLocationS
) extends INameT with IFunctionTemplateNameT {
  vpass()
  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): FunctionBoundNameT = {
    interner.intern(FunctionBoundNameT(this, templateArgs, params))
  }
}

// We tried splitting this out into a ReachableFunctionNameT, so each function could
// keep separate its direct instantiation bound params (e.g. where func drop(T)void on
// the function itself) as opposed to its indirect instantiation bound params (ones
// declared on the params' kind struct/interfaces' definitions).
// See RFNTIOB for why we reverted that.
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FunctionBoundNameT<'s, 't> {
    pub template: &'t FunctionBoundTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub parameters: &'t [CoordT<'s, 't>],
}
/*
case class FunctionBoundNameT(
  template: FunctionBoundTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT

// PredictedFunctionNameT and PredictedFunctionTemplateNameT are special names similar to
// FunctionBoundNameT, they're temporary ones created during solving, to put into the result
// runes. At the end of solving, just afterward, they're turned into actual FunctionBoundNameT
// or resolved from the calling environment.
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PredictedFunctionTemplateNameT<'s, 't> {
    pub human_name: StrI<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class PredictedFunctionTemplateNameT(
    humanName: StrI
) extends INameT with IFunctionTemplateNameT {
  vpass()
  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): PredictedFunctionNameT = {
    interner.intern(PredictedFunctionNameT(this, templateArgs, params))
  }
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PredictedFunctionNameT<'s, 't> {
    pub template: &'t PredictedFunctionTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub parameters: &'t [CoordT<'s, 't>],
}
/*
case class PredictedFunctionNameT(
    template: PredictedFunctionTemplateNameT,
    templateArgs: Vector[ITemplataT[ITemplataType]],
    parameters: Vector[CoordT]
) extends IFunctionNameT

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FunctionTemplateNameT<'s, 't> {
    pub human_name: StrI<'s>,
    pub code_location: CodeLocationS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class FunctionTemplateNameT(
    humanName: StrI,
    codeLocation: CodeLocationS
) extends INameT with IFunctionTemplateNameT {
  vpass()
  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
    interner.intern(FunctionNameT(this, templateArgs, params))
  }
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LambdaCallFunctionTemplateNameT<'s, 't> {
    pub code_location: CodeLocationS<'s>,
    pub param_types: &'t [CoordT<'s, 't>],
}
/*
case class LambdaCallFunctionTemplateNameT(
  codeLocation: CodeLocationS,
  paramTypes: Vector[CoordT]
) extends INameT with IFunctionTemplateNameT {
  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
    // Post instantiator, the params will be real, but our template paramTypes will still be placeholders
    // vassert(params == paramTypes)
    interner.intern(LambdaCallFunctionNameT(this, templateArgs, params))
  }
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LambdaCallFunctionNameT<'s, 't> {
    pub template: &'t LambdaCallFunctionTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub parameters: &'t [CoordT<'s, 't>],
}
/*
case class LambdaCallFunctionNameT(
  template: LambdaCallFunctionTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ForwarderFunctionTemplateNameT<'s, 't> {
    pub inner: &'t IFunctionTemplateNameT<'s, 't>,
    pub index: i32,
}
/*
case class ForwarderFunctionTemplateNameT(
  inner: IFunctionTemplateNameT,
  index: Int
) extends INameT with IFunctionTemplateNameT {
  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
    interner.intern(ForwarderFunctionNameT(this, inner.makeFunctionName(interner, keywords, templateArgs, params)))//, index))
  }
}


//case class AbstractVirtualDropFunctionTemplateNameT(
//  implName: INameT
//) extends INameT with IFunctionTemplateNameT {
//  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
//    interner.intern(
//      AbstractVirtualDropFunctionNameT(implName, templateArgs, params))
//  }
//}

//case class AbstractVirtualDropFunctionNameT(
//  implName: INameT,
//  templateArgs: Vector[ITemplata[ITemplataType]],
//  parameters: Vector[CoordT]
//) extends INameT with IFunctionNameT

//case class OverrideVirtualDropFunctionTemplateNameT(
//  implName: INameT
//) extends INameT with IFunctionTemplateNameT {
//  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
//    interner.intern(
//      OverrideVirtualDropFunctionNameT(implName, templateArgs, params))
//  }
//}

//case class OverrideVirtualDropFunctionNameT(
//  implName: INameT,
//  templateArgs: Vector[ITemplata[ITemplataType]],
//  parameters: Vector[CoordT]
//) extends INameT with IFunctionNameT

//case class LambdaTemplateNameT(
//  codeLocation: CodeLocationS
//) extends INameT with IFunctionTemplateNameT {
//  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
//    interner.intern(FunctionNameT(interner.intern(FunctionTemplateNameT(keywords.underscoresCall, codeLocation)), templateArgs, params))
//  }
//}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ConstructorTemplateNameT<'s, 't> {
    pub code_location: CodeLocationS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class ConstructorTemplateNameT(
  codeLocation: CodeLocationS
) extends INameT with IFunctionTemplateNameT {
  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = vimpl()
}

//case class FreeTemplateNameT(codeLoc: CodeLocationS) extends INameT with IFunctionTemplateNameT {
//  vpass()
//  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
//    params match {
//      case Vector(coord) => {
//        interner.intern(FreeNameT(this, templateArgs, coord))
//      }
//      case other => vwat(other)
//    }
//  }
//}
//case class FreeNameT(
//  template: FreeTemplateNameT,
//  templateArgs: Vector[ITemplata[ITemplataType]],
//  coordT: CoordT
//) extends IFunctionNameT {
//  override def parameters: Vector[CoordT] = Vector(coordT)
//}

//// See NSIDN for why we have these virtual names
//case class AbstractVirtualFreeTemplateNameT(codeLoc: CodeLocationS) extends INameT with IFunctionTemplateNameT {
//  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
//    val Vector(CoordT(ShareT, kind)) = params
//    interner.intern(AbstractVirtualFreeNameT(templateArgs, kind))
//  }
//}
//// See NSIDN for why we have these virtual names
//case class AbstractVirtualFreeNameT(templateArgs: Vector[ITemplata[ITemplataType]], param: KindT) extends IFunctionNameT {
//  override def parameters: Vector[CoordT] = Vector(CoordT(ShareT, param))
//}
//
//// See NSIDN for why we have these virtual names
//case class OverrideVirtualFreeTemplateNameT(codeLoc: CodeLocationS) extends INameT with IFunctionTemplateNameT {
//  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
//    val Vector(CoordT(ShareT, kind)) = params
//    interner.intern(OverrideVirtualFreeNameT(templateArgs, kind))
//  }
//}
//// See NSIDN for why we have these virtual names
//case class OverrideVirtualFreeNameT(templateArgs: Vector[ITemplata[ITemplataType]], param: KindT) extends IFunctionNameT {
//  override def parameters: Vector[CoordT] = Vector(CoordT(ShareT, param))
//}

// Vale has no Self, its just a convenient first name parameter.
// See also SelfNameS.
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SelfNameT<'s, 't> {
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
/*
case class SelfNameT() extends IVarNameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ArbitraryNameT<'s, 't> {
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
/*
case class ArbitraryNameT() extends INameT
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CitizenNameT<'s, 't> {
    Struct(&'t StructNameT<'s, 't>),
    Interface(&'t InterfaceNameT<'s, 't>),
}
/*
sealed trait CitizenNameT extends ICitizenNameT {
  def template: ICitizenTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
}

*/
fn citizen_name_unapply() { panic!("Unmigrated unapply"); }
/*
object CitizenNameT {
  def unapply(c: CitizenNameT): Option[(ICitizenTemplateNameT, Vector[ITemplataT[ITemplataType]])] = {
    c match {
      case StructNameT(template, templateArgs) => Some((template, templateArgs))
      case InterfaceNameT(template, templateArgs) => Some((template, templateArgs))
    }
  }
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructNameT<'s, 't> {
    pub template: &'t IStructTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
}
/*
case class StructNameT(
  template: IStructTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]]
) extends IStructNameT with CitizenNameT {
  vpass()
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceNameT<'s, 't> {
    pub template: &'t InterfaceTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
}
/*
case class InterfaceNameT(
  template: InterfaceTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]]
) extends IInterfaceNameT with CitizenNameT {
  vpass()
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LambdaCitizenTemplateNameT<'s, 't> {
    pub code_location: CodeLocationS<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class LambdaCitizenTemplateNameT(
  codeLocation: CodeLocationS
) extends IStructTemplateNameT {
  override def makeStructName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): IStructNameT = {
    vassert(templateArgs.isEmpty)
    interner.intern(LambdaCitizenNameT(this))
  }
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LambdaCitizenNameT<'s, 't> {
    pub template: &'t LambdaCitizenTemplateNameT<'s, 't>,
}
/*
case class LambdaCitizenNameT(
  template: LambdaCitizenTemplateNameT
) extends IStructNameT {
  def templateArgs: Vector[ITemplataT[ITemplataType]] = Vector.empty
  vpass()
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CitizenTemplateNameT<'s, 't> {
    StructTemplate(&'t StructTemplateNameT<'s, 't>),
    InterfaceTemplate(&'t InterfaceTemplateNameT<'s, 't>),
}
/*
sealed trait CitizenTemplateNameT extends ICitizenTemplateNameT {
  def humanName: StrI
  // We don't include a CodeLocation here because:
  // - There's no struct overloading, so there should only ever be one, we don't have to disambiguate
  //   with code locations
  // - It makes it easier to determine the CitizenTemplateNameT from a CitizenNameT which doesn't
  //   remember its code location.
  //codeLocation: CodeLocationS

//  override def makeCitizenName(interner: Interner, templateArgs: Vector[ITemplata[ITemplataType]]): ICitizenNameT = {
//    interner.intern(CitizenNameT(this, templateArgs))
//  }
}

*/
fn citizen_template_name_unapply() { panic!("Unmigrated unapply"); }
/*
object CitizenTemplateNameT {
  def unapply(x: CitizenTemplateNameT): Option[StrI] = {
    x match {
      case StructTemplateNameT(humanName) => Some(humanName)
      case InterfaceTemplateNameT(humanName) => Some(humanName)
      case _ => None
    }
  }
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructTemplateNameT<'s, 't> {
    pub human_name: StrI<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class StructTemplateNameT(
  humanName: StrI,
  // We don't include a CodeLocation here because:
  // - There's no struct overloading, so there should only ever be one, we don't have to disambiguate
  //   with code locations
  // - It makes it easier to determine the StructTemplateNameT from a StructNameT which doesn't
  //   remember its code location.
  //   (note from later: not sure this is true anymore, since StructNameT contains a StructTemplateNameT)
  //codeLocation: CodeLocationS
) extends IStructTemplateNameT with CitizenTemplateNameT {
  vpass()

  override def makeStructName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): IStructNameT = {
    interner.intern(StructNameT(this, templateArgs))
  }
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceTemplateNameT<'s, 't> {
    pub human_namee: StrI<'s>,
    pub _phantom: std::marker::PhantomData<&'t ()>,
}
/*
case class InterfaceTemplateNameT(
  humanNamee: StrI,
  // We don't include a CodeLocation here because:
  // - There's no struct overloading, so there should only ever be one, we don't have to disambiguate
  //   with code locations
  // - It makes it easier to determine the InterfaceTemplateNameT from a InterfaceNameT which doesn't
  //   remember its code location.
  //codeLocation: CodeLocationS
) extends IInterfaceTemplateNameT with CitizenTemplateNameT with ICitizenTemplateNameT {
  override def humanName = humanNamee
  override def makeInterfaceName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): IInterfaceNameT = {
    interner.intern(InterfaceNameT(this, templateArgs))
  }
  override def makeCitizenName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): ICitizenNameT = {
    makeInterfaceName(interner, templateArgs)
  }
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AnonymousSubstructImplTemplateNameT<'s, 't> {
    pub interface: &'t IInterfaceTemplateNameT<'s, 't>,
}
/*
case class AnonymousSubstructImplTemplateNameT(
  interface: IInterfaceTemplateNameT
) extends IImplTemplateNameT {
  override def makeImplName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]], subCitizen: ICitizenTT): IImplNameT = {
    interner.intern(AnonymousSubstructImplNameT(this, templateArgs, subCitizen))
  }
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AnonymousSubstructImplNameT<'s, 't> {
    pub template: &'t AnonymousSubstructImplTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub sub_citizen: ICitizenTT<'s, 't>,
}
/*
case class AnonymousSubstructImplNameT(
  template: AnonymousSubstructImplTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  subCitizen: ICitizenTT
) extends IImplNameT


*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AnonymousSubstructTemplateNameT<'s, 't> {
    pub interface: &'t IInterfaceTemplateNameT<'s, 't>,
}
/*
case class AnonymousSubstructTemplateNameT(
  // This happens to be the same thing that appears before this AnonymousSubstructNameT in a FullNameT.
  // This is really only here to help us calculate the imprecise name for this thing.
  interface: IInterfaceTemplateNameT
) extends IStructTemplateNameT {
  override def makeStructName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): IStructNameT = {
    interner.intern(AnonymousSubstructNameT(this, templateArgs))
  }
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AnonymousSubstructConstructorTemplateNameT<'s, 't> {
    pub substruct: &'t ICitizenTemplateNameT<'s, 't>,
}
/*
case class AnonymousSubstructConstructorTemplateNameT(
  substruct: ICitizenTemplateNameT
) extends IFunctionTemplateNameT {
  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
    interner.intern(AnonymousSubstructConstructorNameT(this, templateArgs, params))
  }
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AnonymousSubstructConstructorNameT<'s, 't> {
    pub template: &'t AnonymousSubstructConstructorTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
    pub parameters: &'t [CoordT<'s, 't>],
}
/*
case class AnonymousSubstructConstructorNameT(
  template: AnonymousSubstructConstructorTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AnonymousSubstructNameT<'s, 't> {
    pub template: &'t AnonymousSubstructTemplateNameT<'s, 't>,
    pub template_args: &'t [ITemplataT<'s, 't>],
}
/*
case class AnonymousSubstructNameT(
  // This happens to be the same thing that appears before this AnonymousSubstructNameT in a FullNameT.
  // This is really only here to help us calculate the imprecise name for this thing.
  template: AnonymousSubstructTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]]
) extends IStructNameT {

}
//case class AnonymousSubstructImplNameT() extends INameT {
//
//}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ResolvingEnvNameT<'s, 't> {
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
/*
case class ResolvingEnvNameT() extends INameT {
  vpass()
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CallEnvNameT<'s, 't> {
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
/*
case class CallEnvNameT() extends INameT {
  vpass()
}
*/
