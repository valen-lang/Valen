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

// ============================================================================
// From / TryFrom bridges between sub-enums and concrete names.
//
// No Scala counterpart — Scala's sealed-trait hierarchy handled all of these
// implicitly. Rust needs them spelled out.
//
// - From<&'t XxxNameT> for IYyyNameT  — wrap a concrete ref as a sub-enum.
// - From<&'t INarrowT> for IWideT     — upcast a narrow sub-enum to a wider one.
// - TryFrom<&'t INameT> for &'t IYyyNameT — narrow (arena ref) form; panic
//   stub per handoff §6.3 Gotcha — this path requires TypingInterner to intern
//   the narrower sub-enum, which is Slab 3+ work (intern_* are still `panic!()`).
// ============================================================================

// -- Concrete → INameT -------------------------------------------------------
impl<'s, 't> From<&'t ExportTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ExportTemplateNameT<'s, 't>) -> Self { INameT::ExportTemplate(x) } }
impl<'s, 't> From<&'t ExportNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ExportNameT<'s, 't>) -> Self { INameT::Export(x) } }
impl<'s, 't> From<&'t ImplTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ImplTemplateNameT<'s, 't>) -> Self { INameT::ImplTemplate(x) } }
impl<'s, 't> From<&'t ImplNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ImplNameT<'s, 't>) -> Self { INameT::Impl(x) } }
impl<'s, 't> From<&'t ImplBoundTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ImplBoundTemplateNameT<'s, 't>) -> Self { INameT::ImplBoundTemplate(x) } }
impl<'s, 't> From<&'t ImplBoundNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ImplBoundNameT<'s, 't>) -> Self { INameT::ImplBound(x) } }
impl<'s, 't> From<&'t LetNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t LetNameT<'s, 't>) -> Self { INameT::Let(x) } }
impl<'s, 't> From<&'t ExportAsNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ExportAsNameT<'s, 't>) -> Self { INameT::ExportAs(x) } }
impl<'s, 't> From<&'t RawArrayNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t RawArrayNameT<'s, 't>) -> Self { INameT::RawArray(x) } }
impl<'s, 't> From<&'t ReachablePrototypeNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ReachablePrototypeNameT<'s, 't>) -> Self { INameT::ReachablePrototype(x) } }
impl<'s, 't> From<&'t StaticSizedArrayTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t StaticSizedArrayTemplateNameT<'s, 't>) -> Self { INameT::StaticSizedArrayTemplate(x) } }
impl<'s, 't> From<&'t StaticSizedArrayNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t StaticSizedArrayNameT<'s, 't>) -> Self { INameT::StaticSizedArray(x) } }
impl<'s, 't> From<&'t RuntimeSizedArrayTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t RuntimeSizedArrayTemplateNameT<'s, 't>) -> Self { INameT::RuntimeSizedArrayTemplate(x) } }
impl<'s, 't> From<&'t RuntimeSizedArrayNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t RuntimeSizedArrayNameT<'s, 't>) -> Self { INameT::RuntimeSizedArray(x) } }
impl<'s, 't> From<&'t KindPlaceholderTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t KindPlaceholderTemplateNameT<'s, 't>) -> Self { INameT::KindPlaceholderTemplate(x) } }
impl<'s, 't> From<&'t KindPlaceholderNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t KindPlaceholderNameT<'s, 't>) -> Self { INameT::KindPlaceholder(x) } }
impl<'s, 't> From<&'t NonKindNonRegionPlaceholderNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t NonKindNonRegionPlaceholderNameT<'s, 't>) -> Self { INameT::NonKindNonRegionPlaceholder(x) } }
impl<'s, 't> From<&'t OverrideDispatcherTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t OverrideDispatcherTemplateNameT<'s, 't>) -> Self { INameT::OverrideDispatcherTemplate(x) } }
impl<'s, 't> From<&'t OverrideDispatcherNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t OverrideDispatcherNameT<'s, 't>) -> Self { INameT::OverrideDispatcher(x) } }
impl<'s, 't> From<&'t OverrideDispatcherCaseNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t OverrideDispatcherCaseNameT<'s, 't>) -> Self { INameT::OverrideDispatcherCase(x) } }
impl<'s, 't> From<&'t TypingPassBlockResultVarNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t TypingPassBlockResultVarNameT<'s, 't>) -> Self { INameT::TypingPassBlockResultVar(x) } }
impl<'s, 't> From<&'t TypingPassFunctionResultVarNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t TypingPassFunctionResultVarNameT<'s, 't>) -> Self { INameT::TypingPassFunctionResultVar(x) } }
impl<'s, 't> From<&'t TypingPassTemporaryVarNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t TypingPassTemporaryVarNameT<'s, 't>) -> Self { INameT::TypingPassTemporaryVar(x) } }
impl<'s, 't> From<&'t TypingPassPatternMemberNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t TypingPassPatternMemberNameT<'s, 't>) -> Self { INameT::TypingPassPatternMember(x) } }
impl<'s, 't> From<&'t TypingIgnoredParamNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t TypingIgnoredParamNameT<'s, 't>) -> Self { INameT::TypingIgnoredParam(x) } }
impl<'s, 't> From<&'t TypingPassPatternDestructureeNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t TypingPassPatternDestructureeNameT<'s, 't>) -> Self { INameT::TypingPassPatternDestructuree(x) } }
impl<'s, 't> From<&'t UnnamedLocalNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t UnnamedLocalNameT<'s, 't>) -> Self { INameT::UnnamedLocal(x) } }
impl<'s, 't> From<&'t ClosureParamNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ClosureParamNameT<'s, 't>) -> Self { INameT::ClosureParam(x) } }
impl<'s, 't> From<&'t ConstructingMemberNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ConstructingMemberNameT<'s, 't>) -> Self { INameT::ConstructingMember(x) } }
impl<'s, 't> From<&'t WhileCondResultNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t WhileCondResultNameT<'s, 't>) -> Self { INameT::WhileCondResult(x) } }
impl<'s, 't> From<&'t IterableNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t IterableNameT<'s, 't>) -> Self { INameT::Iterable(x) } }
impl<'s, 't> From<&'t IteratorNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t IteratorNameT<'s, 't>) -> Self { INameT::Iterator(x) } }
impl<'s, 't> From<&'t IterationOptionNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t IterationOptionNameT<'s, 't>) -> Self { INameT::IterationOption(x) } }
impl<'s, 't> From<&'t MagicParamNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t MagicParamNameT<'s, 't>) -> Self { INameT::MagicParam(x) } }
impl<'s, 't> From<&'t CodeVarNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t CodeVarNameT<'s, 't>) -> Self { INameT::CodeVar(x) } }
impl<'s, 't> From<&'t AnonymousSubstructMemberNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t AnonymousSubstructMemberNameT<'s, 't>) -> Self { INameT::AnonymousSubstructMember(x) } }
impl<'s, 't> From<&'t PrimitiveNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t PrimitiveNameT<'s, 't>) -> Self { INameT::Primitive(x) } }
impl<'s, 't> From<&'t PackageTopLevelNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t PackageTopLevelNameT<'s, 't>) -> Self { INameT::PackageTopLevel(x) } }
impl<'s, 't> From<&'t ProjectNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ProjectNameT<'s, 't>) -> Self { INameT::Project(x) } }
impl<'s, 't> From<&'t PackageNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t PackageNameT<'s, 't>) -> Self { INameT::Package(x) } }
impl<'s, 't> From<&'t RuneNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t RuneNameT<'s, 't>) -> Self { INameT::Rune(x) } }
impl<'s, 't> From<&'t BuildingFunctionNameWithClosuredsT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t BuildingFunctionNameWithClosuredsT<'s, 't>) -> Self { INameT::BuildingFunctionNameWithClosureds(x) } }
impl<'s, 't> From<&'t ExternTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ExternTemplateNameT<'s, 't>) -> Self { INameT::ExternTemplate(x) } }
impl<'s, 't> From<&'t ExternNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ExternNameT<'s, 't>) -> Self { INameT::Extern(x) } }
impl<'s, 't> From<&'t ExternFunctionNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ExternFunctionNameT<'s, 't>) -> Self { INameT::ExternFunction(x) } }
impl<'s, 't> From<&'t FunctionNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t FunctionNameT<'s, 't>) -> Self { INameT::Function(x) } }
impl<'s, 't> From<&'t ForwarderFunctionNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ForwarderFunctionNameT<'s, 't>) -> Self { INameT::ForwarderFunction(x) } }
impl<'s, 't> From<&'t FunctionBoundTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t FunctionBoundTemplateNameT<'s, 't>) -> Self { INameT::FunctionBoundTemplate(x) } }
impl<'s, 't> From<&'t FunctionBoundNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t FunctionBoundNameT<'s, 't>) -> Self { INameT::FunctionBound(x) } }
impl<'s, 't> From<&'t PredictedFunctionTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t PredictedFunctionTemplateNameT<'s, 't>) -> Self { INameT::PredictedFunctionTemplate(x) } }
impl<'s, 't> From<&'t PredictedFunctionNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t PredictedFunctionNameT<'s, 't>) -> Self { INameT::PredictedFunction(x) } }
impl<'s, 't> From<&'t FunctionTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t FunctionTemplateNameT<'s, 't>) -> Self { INameT::FunctionTemplate(x) } }
impl<'s, 't> From<&'t LambdaCallFunctionTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t LambdaCallFunctionTemplateNameT<'s, 't>) -> Self { INameT::LambdaCallFunctionTemplate(x) } }
impl<'s, 't> From<&'t LambdaCallFunctionNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t LambdaCallFunctionNameT<'s, 't>) -> Self { INameT::LambdaCallFunction(x) } }
impl<'s, 't> From<&'t ForwarderFunctionTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ForwarderFunctionTemplateNameT<'s, 't>) -> Self { INameT::ForwarderFunctionTemplate(x) } }
impl<'s, 't> From<&'t ConstructorTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ConstructorTemplateNameT<'s, 't>) -> Self { INameT::ConstructorTemplate(x) } }
impl<'s, 't> From<&'t SelfNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t SelfNameT<'s, 't>) -> Self { INameT::Self_(x) } }
impl<'s, 't> From<&'t ArbitraryNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ArbitraryNameT<'s, 't>) -> Self { INameT::Arbitrary(x) } }
impl<'s, 't> From<&'t StructNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t StructNameT<'s, 't>) -> Self { INameT::Struct(x) } }
impl<'s, 't> From<&'t InterfaceNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t InterfaceNameT<'s, 't>) -> Self { INameT::Interface(x) } }
impl<'s, 't> From<&'t LambdaCitizenTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t LambdaCitizenTemplateNameT<'s, 't>) -> Self { INameT::LambdaCitizenTemplate(x) } }
impl<'s, 't> From<&'t LambdaCitizenNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t LambdaCitizenNameT<'s, 't>) -> Self { INameT::LambdaCitizen(x) } }
impl<'s, 't> From<&'t StructTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t StructTemplateNameT<'s, 't>) -> Self { INameT::StructTemplate(x) } }
impl<'s, 't> From<&'t InterfaceTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t InterfaceTemplateNameT<'s, 't>) -> Self { INameT::InterfaceTemplate(x) } }
impl<'s, 't> From<&'t AnonymousSubstructImplTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t AnonymousSubstructImplTemplateNameT<'s, 't>) -> Self { INameT::AnonymousSubstructImplTemplate(x) } }
impl<'s, 't> From<&'t AnonymousSubstructImplNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t AnonymousSubstructImplNameT<'s, 't>) -> Self { INameT::AnonymousSubstructImpl(x) } }
impl<'s, 't> From<&'t AnonymousSubstructTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t AnonymousSubstructTemplateNameT<'s, 't>) -> Self { INameT::AnonymousSubstructTemplate(x) } }
impl<'s, 't> From<&'t AnonymousSubstructConstructorTemplateNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t AnonymousSubstructConstructorTemplateNameT<'s, 't>) -> Self { INameT::AnonymousSubstructConstructorTemplate(x) } }
impl<'s, 't> From<&'t AnonymousSubstructConstructorNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t AnonymousSubstructConstructorNameT<'s, 't>) -> Self { INameT::AnonymousSubstructConstructor(x) } }
impl<'s, 't> From<&'t AnonymousSubstructNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t AnonymousSubstructNameT<'s, 't>) -> Self { INameT::AnonymousSubstruct(x) } }
impl<'s, 't> From<&'t ResolvingEnvNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t ResolvingEnvNameT<'s, 't>) -> Self { INameT::ResolvingEnv(x) } }
impl<'s, 't> From<&'t CallEnvNameT<'s, 't>> for INameT<'s, 't> { fn from(x: &'t CallEnvNameT<'s, 't>) -> Self { INameT::CallEnv(x) } }

// -- Concrete → ITemplateNameT -----------------------------------------------
impl<'s, 't> From<&'t ExportTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t ExportTemplateNameT<'s, 't>) -> Self { ITemplateNameT::ExportTemplate(x) } }
impl<'s, 't> From<&'t ImplTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t ImplTemplateNameT<'s, 't>) -> Self { ITemplateNameT::ImplTemplate(x) } }
impl<'s, 't> From<&'t ImplBoundTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t ImplBoundTemplateNameT<'s, 't>) -> Self { ITemplateNameT::ImplBoundTemplate(x) } }
impl<'s, 't> From<&'t StaticSizedArrayTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t StaticSizedArrayTemplateNameT<'s, 't>) -> Self { ITemplateNameT::StaticSizedArrayTemplate(x) } }
impl<'s, 't> From<&'t RuntimeSizedArrayTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t RuntimeSizedArrayTemplateNameT<'s, 't>) -> Self { ITemplateNameT::RuntimeSizedArrayTemplate(x) } }
impl<'s, 't> From<&'t KindPlaceholderTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t KindPlaceholderTemplateNameT<'s, 't>) -> Self { ITemplateNameT::KindPlaceholderTemplate(x) } }
impl<'s, 't> From<&'t OverrideDispatcherTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t OverrideDispatcherTemplateNameT<'s, 't>) -> Self { ITemplateNameT::OverrideDispatcherTemplate(x) } }
impl<'s, 't> From<&'t OverrideDispatcherCaseNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t OverrideDispatcherCaseNameT<'s, 't>) -> Self { ITemplateNameT::OverrideDispatcherCase(x) } }
impl<'s, 't> From<&'t ExternTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t ExternTemplateNameT<'s, 't>) -> Self { ITemplateNameT::ExternTemplate(x) } }
impl<'s, 't> From<&'t ExternFunctionNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t ExternFunctionNameT<'s, 't>) -> Self { ITemplateNameT::ExternFunction(x) } }
impl<'s, 't> From<&'t FunctionBoundTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t FunctionBoundTemplateNameT<'s, 't>) -> Self { ITemplateNameT::FunctionBoundTemplate(x) } }
impl<'s, 't> From<&'t PredictedFunctionTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t PredictedFunctionTemplateNameT<'s, 't>) -> Self { ITemplateNameT::PredictedFunctionTemplate(x) } }
impl<'s, 't> From<&'t FunctionTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t FunctionTemplateNameT<'s, 't>) -> Self { ITemplateNameT::FunctionTemplate(x) } }
impl<'s, 't> From<&'t LambdaCallFunctionTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t LambdaCallFunctionTemplateNameT<'s, 't>) -> Self { ITemplateNameT::LambdaCallFunctionTemplate(x) } }
impl<'s, 't> From<&'t ForwarderFunctionTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t ForwarderFunctionTemplateNameT<'s, 't>) -> Self { ITemplateNameT::ForwarderFunctionTemplate(x) } }
impl<'s, 't> From<&'t ConstructorTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t ConstructorTemplateNameT<'s, 't>) -> Self { ITemplateNameT::ConstructorTemplate(x) } }
impl<'s, 't> From<&'t LambdaCitizenTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t LambdaCitizenTemplateNameT<'s, 't>) -> Self { ITemplateNameT::LambdaCitizenTemplate(x) } }
impl<'s, 't> From<&'t StructTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t StructTemplateNameT<'s, 't>) -> Self { ITemplateNameT::StructTemplate(x) } }
impl<'s, 't> From<&'t InterfaceTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t InterfaceTemplateNameT<'s, 't>) -> Self { ITemplateNameT::InterfaceTemplate(x) } }
impl<'s, 't> From<&'t AnonymousSubstructImplTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t AnonymousSubstructImplTemplateNameT<'s, 't>) -> Self { ITemplateNameT::AnonymousSubstructImplTemplate(x) } }
impl<'s, 't> From<&'t AnonymousSubstructTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t AnonymousSubstructTemplateNameT<'s, 't>) -> Self { ITemplateNameT::AnonymousSubstructTemplate(x) } }
impl<'s, 't> From<&'t AnonymousSubstructConstructorTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> { fn from(x: &'t AnonymousSubstructConstructorTemplateNameT<'s, 't>) -> Self { ITemplateNameT::AnonymousSubstructConstructorTemplate(x) } }

// -- Concrete → IInstantiationNameT ------------------------------------------
impl<'s, 't> From<&'t ExportNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t ExportNameT<'s, 't>) -> Self { IInstantiationNameT::Export(x) } }
impl<'s, 't> From<&'t ImplNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t ImplNameT<'s, 't>) -> Self { IInstantiationNameT::Impl(x) } }
impl<'s, 't> From<&'t ImplBoundNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t ImplBoundNameT<'s, 't>) -> Self { IInstantiationNameT::ImplBound(x) } }
impl<'s, 't> From<&'t StaticSizedArrayNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t StaticSizedArrayNameT<'s, 't>) -> Self { IInstantiationNameT::StaticSizedArray(x) } }
impl<'s, 't> From<&'t RuntimeSizedArrayNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t RuntimeSizedArrayNameT<'s, 't>) -> Self { IInstantiationNameT::RuntimeSizedArray(x) } }
impl<'s, 't> From<&'t KindPlaceholderNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t KindPlaceholderNameT<'s, 't>) -> Self { IInstantiationNameT::KindPlaceholder(x) } }
impl<'s, 't> From<&'t OverrideDispatcherNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t OverrideDispatcherNameT<'s, 't>) -> Self { IInstantiationNameT::OverrideDispatcher(x) } }
impl<'s, 't> From<&'t OverrideDispatcherCaseNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t OverrideDispatcherCaseNameT<'s, 't>) -> Self { IInstantiationNameT::OverrideDispatcherCase(x) } }
impl<'s, 't> From<&'t ExternNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t ExternNameT<'s, 't>) -> Self { IInstantiationNameT::Extern(x) } }
impl<'s, 't> From<&'t ExternFunctionNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t ExternFunctionNameT<'s, 't>) -> Self { IInstantiationNameT::ExternFunction(x) } }
impl<'s, 't> From<&'t FunctionNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t FunctionNameT<'s, 't>) -> Self { IInstantiationNameT::Function(x) } }
impl<'s, 't> From<&'t ForwarderFunctionNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t ForwarderFunctionNameT<'s, 't>) -> Self { IInstantiationNameT::ForwarderFunction(x) } }
impl<'s, 't> From<&'t FunctionBoundNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t FunctionBoundNameT<'s, 't>) -> Self { IInstantiationNameT::FunctionBound(x) } }
impl<'s, 't> From<&'t PredictedFunctionNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t PredictedFunctionNameT<'s, 't>) -> Self { IInstantiationNameT::PredictedFunction(x) } }
impl<'s, 't> From<&'t LambdaCallFunctionNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t LambdaCallFunctionNameT<'s, 't>) -> Self { IInstantiationNameT::LambdaCallFunction(x) } }
impl<'s, 't> From<&'t StructNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t StructNameT<'s, 't>) -> Self { IInstantiationNameT::Struct(x) } }
impl<'s, 't> From<&'t InterfaceNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t InterfaceNameT<'s, 't>) -> Self { IInstantiationNameT::Interface(x) } }
impl<'s, 't> From<&'t LambdaCitizenNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t LambdaCitizenNameT<'s, 't>) -> Self { IInstantiationNameT::LambdaCitizen(x) } }
impl<'s, 't> From<&'t AnonymousSubstructImplNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t AnonymousSubstructImplNameT<'s, 't>) -> Self { IInstantiationNameT::AnonymousSubstructImpl(x) } }
impl<'s, 't> From<&'t AnonymousSubstructConstructorNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t AnonymousSubstructConstructorNameT<'s, 't>) -> Self { IInstantiationNameT::AnonymousSubstructConstructor(x) } }
impl<'s, 't> From<&'t AnonymousSubstructNameT<'s, 't>> for IInstantiationNameT<'s, 't> { fn from(x: &'t AnonymousSubstructNameT<'s, 't>) -> Self { IInstantiationNameT::AnonymousSubstruct(x) } }

// -- Concrete → IFunctionTemplateNameT --------------------------------------
impl<'s, 't> From<&'t OverrideDispatcherTemplateNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> { fn from(x: &'t OverrideDispatcherTemplateNameT<'s, 't>) -> Self { IFunctionTemplateNameT::OverrideDispatcherTemplate(x) } }
impl<'s, 't> From<&'t ExternFunctionNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> { fn from(x: &'t ExternFunctionNameT<'s, 't>) -> Self { IFunctionTemplateNameT::ExternFunction(x) } }
impl<'s, 't> From<&'t FunctionBoundTemplateNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> { fn from(x: &'t FunctionBoundTemplateNameT<'s, 't>) -> Self { IFunctionTemplateNameT::FunctionBoundTemplate(x) } }
impl<'s, 't> From<&'t PredictedFunctionTemplateNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> { fn from(x: &'t PredictedFunctionTemplateNameT<'s, 't>) -> Self { IFunctionTemplateNameT::PredictedFunctionTemplate(x) } }
impl<'s, 't> From<&'t FunctionTemplateNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> { fn from(x: &'t FunctionTemplateNameT<'s, 't>) -> Self { IFunctionTemplateNameT::FunctionTemplate(x) } }
impl<'s, 't> From<&'t LambdaCallFunctionTemplateNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> { fn from(x: &'t LambdaCallFunctionTemplateNameT<'s, 't>) -> Self { IFunctionTemplateNameT::LambdaCallFunctionTemplate(x) } }
impl<'s, 't> From<&'t ForwarderFunctionTemplateNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> { fn from(x: &'t ForwarderFunctionTemplateNameT<'s, 't>) -> Self { IFunctionTemplateNameT::ForwarderFunctionTemplate(x) } }
impl<'s, 't> From<&'t ConstructorTemplateNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> { fn from(x: &'t ConstructorTemplateNameT<'s, 't>) -> Self { IFunctionTemplateNameT::ConstructorTemplate(x) } }
impl<'s, 't> From<&'t AnonymousSubstructConstructorTemplateNameT<'s, 't>> for IFunctionTemplateNameT<'s, 't> { fn from(x: &'t AnonymousSubstructConstructorTemplateNameT<'s, 't>) -> Self { IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(x) } }

// -- Concrete → IFunctionNameT -----------------------------------------------
impl<'s, 't> From<&'t OverrideDispatcherNameT<'s, 't>> for IFunctionNameT<'s, 't> { fn from(x: &'t OverrideDispatcherNameT<'s, 't>) -> Self { IFunctionNameT::OverrideDispatcher(x) } }
impl<'s, 't> From<&'t ExternFunctionNameT<'s, 't>> for IFunctionNameT<'s, 't> { fn from(x: &'t ExternFunctionNameT<'s, 't>) -> Self { IFunctionNameT::ExternFunction(x) } }
impl<'s, 't> From<&'t FunctionNameT<'s, 't>> for IFunctionNameT<'s, 't> { fn from(x: &'t FunctionNameT<'s, 't>) -> Self { IFunctionNameT::Function(x) } }
impl<'s, 't> From<&'t ForwarderFunctionNameT<'s, 't>> for IFunctionNameT<'s, 't> { fn from(x: &'t ForwarderFunctionNameT<'s, 't>) -> Self { IFunctionNameT::ForwarderFunction(x) } }
impl<'s, 't> From<&'t FunctionBoundNameT<'s, 't>> for IFunctionNameT<'s, 't> { fn from(x: &'t FunctionBoundNameT<'s, 't>) -> Self { IFunctionNameT::FunctionBound(x) } }
impl<'s, 't> From<&'t PredictedFunctionNameT<'s, 't>> for IFunctionNameT<'s, 't> { fn from(x: &'t PredictedFunctionNameT<'s, 't>) -> Self { IFunctionNameT::PredictedFunction(x) } }
impl<'s, 't> From<&'t LambdaCallFunctionNameT<'s, 't>> for IFunctionNameT<'s, 't> { fn from(x: &'t LambdaCallFunctionNameT<'s, 't>) -> Self { IFunctionNameT::LambdaCallFunction(x) } }
impl<'s, 't> From<&'t AnonymousSubstructConstructorNameT<'s, 't>> for IFunctionNameT<'s, 't> { fn from(x: &'t AnonymousSubstructConstructorNameT<'s, 't>) -> Self { IFunctionNameT::AnonymousSubstructConstructor(x) } }

// -- Concrete → ISuperKindTemplateNameT --------------------------------------
impl<'s, 't> From<&'t KindPlaceholderTemplateNameT<'s, 't>> for ISuperKindTemplateNameT<'s, 't> { fn from(x: &'t KindPlaceholderTemplateNameT<'s, 't>) -> Self { ISuperKindTemplateNameT::KindPlaceholderTemplate(x) } }
impl<'s, 't> From<&'t InterfaceTemplateNameT<'s, 't>> for ISuperKindTemplateNameT<'s, 't> { fn from(x: &'t InterfaceTemplateNameT<'s, 't>) -> Self { ISuperKindTemplateNameT::InterfaceTemplate(x) } }

// -- Concrete → ISubKindTemplateNameT ----------------------------------------
impl<'s, 't> From<&'t StaticSizedArrayTemplateNameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> { fn from(x: &'t StaticSizedArrayTemplateNameT<'s, 't>) -> Self { ISubKindTemplateNameT::StaticSizedArrayTemplate(x) } }
impl<'s, 't> From<&'t RuntimeSizedArrayTemplateNameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> { fn from(x: &'t RuntimeSizedArrayTemplateNameT<'s, 't>) -> Self { ISubKindTemplateNameT::RuntimeSizedArrayTemplate(x) } }
impl<'s, 't> From<&'t KindPlaceholderTemplateNameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> { fn from(x: &'t KindPlaceholderTemplateNameT<'s, 't>) -> Self { ISubKindTemplateNameT::KindPlaceholderTemplate(x) } }
impl<'s, 't> From<&'t LambdaCitizenTemplateNameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> { fn from(x: &'t LambdaCitizenTemplateNameT<'s, 't>) -> Self { ISubKindTemplateNameT::LambdaCitizenTemplate(x) } }
impl<'s, 't> From<&'t StructTemplateNameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> { fn from(x: &'t StructTemplateNameT<'s, 't>) -> Self { ISubKindTemplateNameT::StructTemplate(x) } }
impl<'s, 't> From<&'t InterfaceTemplateNameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> { fn from(x: &'t InterfaceTemplateNameT<'s, 't>) -> Self { ISubKindTemplateNameT::InterfaceTemplate(x) } }
impl<'s, 't> From<&'t AnonymousSubstructTemplateNameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> { fn from(x: &'t AnonymousSubstructTemplateNameT<'s, 't>) -> Self { ISubKindTemplateNameT::AnonymousSubstructTemplate(x) } }

// -- Concrete → ICitizenTemplateNameT ----------------------------------------
impl<'s, 't> From<&'t StaticSizedArrayTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> { fn from(x: &'t StaticSizedArrayTemplateNameT<'s, 't>) -> Self { ICitizenTemplateNameT::StaticSizedArrayTemplate(x) } }
impl<'s, 't> From<&'t RuntimeSizedArrayTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> { fn from(x: &'t RuntimeSizedArrayTemplateNameT<'s, 't>) -> Self { ICitizenTemplateNameT::RuntimeSizedArrayTemplate(x) } }
impl<'s, 't> From<&'t LambdaCitizenTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> { fn from(x: &'t LambdaCitizenTemplateNameT<'s, 't>) -> Self { ICitizenTemplateNameT::LambdaCitizenTemplate(x) } }
impl<'s, 't> From<&'t StructTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> { fn from(x: &'t StructTemplateNameT<'s, 't>) -> Self { ICitizenTemplateNameT::StructTemplate(x) } }
impl<'s, 't> From<&'t InterfaceTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> { fn from(x: &'t InterfaceTemplateNameT<'s, 't>) -> Self { ICitizenTemplateNameT::InterfaceTemplate(x) } }
impl<'s, 't> From<&'t AnonymousSubstructTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> { fn from(x: &'t AnonymousSubstructTemplateNameT<'s, 't>) -> Self { ICitizenTemplateNameT::AnonymousSubstructTemplate(x) } }

// -- Concrete → IStructTemplateNameT -----------------------------------------
impl<'s, 't> From<&'t LambdaCitizenTemplateNameT<'s, 't>> for IStructTemplateNameT<'s, 't> { fn from(x: &'t LambdaCitizenTemplateNameT<'s, 't>) -> Self { IStructTemplateNameT::LambdaCitizenTemplate(x) } }
impl<'s, 't> From<&'t StructTemplateNameT<'s, 't>> for IStructTemplateNameT<'s, 't> { fn from(x: &'t StructTemplateNameT<'s, 't>) -> Self { IStructTemplateNameT::StructTemplate(x) } }
impl<'s, 't> From<&'t AnonymousSubstructTemplateNameT<'s, 't>> for IStructTemplateNameT<'s, 't> { fn from(x: &'t AnonymousSubstructTemplateNameT<'s, 't>) -> Self { IStructTemplateNameT::AnonymousSubstructTemplate(x) } }

// -- Concrete → IInterfaceTemplateNameT --------------------------------------
impl<'s, 't> From<&'t InterfaceTemplateNameT<'s, 't>> for IInterfaceTemplateNameT<'s, 't> { fn from(x: &'t InterfaceTemplateNameT<'s, 't>) -> Self { IInterfaceTemplateNameT::InterfaceTemplate(x) } }

// -- Concrete → ISuperKindNameT ----------------------------------------------
impl<'s, 't> From<&'t KindPlaceholderNameT<'s, 't>> for ISuperKindNameT<'s, 't> { fn from(x: &'t KindPlaceholderNameT<'s, 't>) -> Self { ISuperKindNameT::KindPlaceholder(x) } }
impl<'s, 't> From<&'t InterfaceNameT<'s, 't>> for ISuperKindNameT<'s, 't> { fn from(x: &'t InterfaceNameT<'s, 't>) -> Self { ISuperKindNameT::Interface(x) } }

// -- Concrete → ISubKindNameT ------------------------------------------------
impl<'s, 't> From<&'t StaticSizedArrayNameT<'s, 't>> for ISubKindNameT<'s, 't> { fn from(x: &'t StaticSizedArrayNameT<'s, 't>) -> Self { ISubKindNameT::StaticSizedArray(x) } }
impl<'s, 't> From<&'t RuntimeSizedArrayNameT<'s, 't>> for ISubKindNameT<'s, 't> { fn from(x: &'t RuntimeSizedArrayNameT<'s, 't>) -> Self { ISubKindNameT::RuntimeSizedArray(x) } }
impl<'s, 't> From<&'t KindPlaceholderNameT<'s, 't>> for ISubKindNameT<'s, 't> { fn from(x: &'t KindPlaceholderNameT<'s, 't>) -> Self { ISubKindNameT::KindPlaceholder(x) } }
impl<'s, 't> From<&'t StructNameT<'s, 't>> for ISubKindNameT<'s, 't> { fn from(x: &'t StructNameT<'s, 't>) -> Self { ISubKindNameT::Struct(x) } }
impl<'s, 't> From<&'t InterfaceNameT<'s, 't>> for ISubKindNameT<'s, 't> { fn from(x: &'t InterfaceNameT<'s, 't>) -> Self { ISubKindNameT::Interface(x) } }
impl<'s, 't> From<&'t LambdaCitizenNameT<'s, 't>> for ISubKindNameT<'s, 't> { fn from(x: &'t LambdaCitizenNameT<'s, 't>) -> Self { ISubKindNameT::LambdaCitizen(x) } }
impl<'s, 't> From<&'t AnonymousSubstructNameT<'s, 't>> for ISubKindNameT<'s, 't> { fn from(x: &'t AnonymousSubstructNameT<'s, 't>) -> Self { ISubKindNameT::AnonymousSubstruct(x) } }

// -- Concrete → ICitizenNameT ------------------------------------------------
impl<'s, 't> From<&'t StaticSizedArrayNameT<'s, 't>> for ICitizenNameT<'s, 't> { fn from(x: &'t StaticSizedArrayNameT<'s, 't>) -> Self { ICitizenNameT::StaticSizedArray(x) } }
impl<'s, 't> From<&'t RuntimeSizedArrayNameT<'s, 't>> for ICitizenNameT<'s, 't> { fn from(x: &'t RuntimeSizedArrayNameT<'s, 't>) -> Self { ICitizenNameT::RuntimeSizedArray(x) } }
impl<'s, 't> From<&'t StructNameT<'s, 't>> for ICitizenNameT<'s, 't> { fn from(x: &'t StructNameT<'s, 't>) -> Self { ICitizenNameT::Struct(x) } }
impl<'s, 't> From<&'t InterfaceNameT<'s, 't>> for ICitizenNameT<'s, 't> { fn from(x: &'t InterfaceNameT<'s, 't>) -> Self { ICitizenNameT::Interface(x) } }
impl<'s, 't> From<&'t LambdaCitizenNameT<'s, 't>> for ICitizenNameT<'s, 't> { fn from(x: &'t LambdaCitizenNameT<'s, 't>) -> Self { ICitizenNameT::LambdaCitizen(x) } }
impl<'s, 't> From<&'t AnonymousSubstructNameT<'s, 't>> for ICitizenNameT<'s, 't> { fn from(x: &'t AnonymousSubstructNameT<'s, 't>) -> Self { ICitizenNameT::AnonymousSubstruct(x) } }

// -- Concrete → IStructNameT -------------------------------------------------
impl<'s, 't> From<&'t StructNameT<'s, 't>> for IStructNameT<'s, 't> { fn from(x: &'t StructNameT<'s, 't>) -> Self { IStructNameT::Struct(x) } }
impl<'s, 't> From<&'t LambdaCitizenNameT<'s, 't>> for IStructNameT<'s, 't> { fn from(x: &'t LambdaCitizenNameT<'s, 't>) -> Self { IStructNameT::LambdaCitizen(x) } }
impl<'s, 't> From<&'t AnonymousSubstructNameT<'s, 't>> for IStructNameT<'s, 't> { fn from(x: &'t AnonymousSubstructNameT<'s, 't>) -> Self { IStructNameT::AnonymousSubstruct(x) } }

// -- Concrete → IInterfaceNameT ----------------------------------------------
impl<'s, 't> From<&'t InterfaceNameT<'s, 't>> for IInterfaceNameT<'s, 't> { fn from(x: &'t InterfaceNameT<'s, 't>) -> Self { IInterfaceNameT::Interface(x) } }

// -- Concrete → IImplTemplateNameT -------------------------------------------
impl<'s, 't> From<&'t ImplTemplateNameT<'s, 't>> for IImplTemplateNameT<'s, 't> { fn from(x: &'t ImplTemplateNameT<'s, 't>) -> Self { IImplTemplateNameT::ImplTemplate(x) } }
impl<'s, 't> From<&'t ImplBoundTemplateNameT<'s, 't>> for IImplTemplateNameT<'s, 't> { fn from(x: &'t ImplBoundTemplateNameT<'s, 't>) -> Self { IImplTemplateNameT::ImplBoundTemplate(x) } }
impl<'s, 't> From<&'t AnonymousSubstructImplTemplateNameT<'s, 't>> for IImplTemplateNameT<'s, 't> { fn from(x: &'t AnonymousSubstructImplTemplateNameT<'s, 't>) -> Self { IImplTemplateNameT::AnonymousSubstructImplTemplate(x) } }

// -- Concrete → IImplNameT ---------------------------------------------------
impl<'s, 't> From<&'t ImplNameT<'s, 't>> for IImplNameT<'s, 't> { fn from(x: &'t ImplNameT<'s, 't>) -> Self { IImplNameT::Impl(x) } }
impl<'s, 't> From<&'t ImplBoundNameT<'s, 't>> for IImplNameT<'s, 't> { fn from(x: &'t ImplBoundNameT<'s, 't>) -> Self { IImplNameT::ImplBound(x) } }
impl<'s, 't> From<&'t AnonymousSubstructImplNameT<'s, 't>> for IImplNameT<'s, 't> { fn from(x: &'t AnonymousSubstructImplNameT<'s, 't>) -> Self { IImplNameT::AnonymousSubstructImpl(x) } }

// -- Concrete → IPlaceholderNameT --------------------------------------------
impl<'s, 't> From<&'t KindPlaceholderNameT<'s, 't>> for IPlaceholderNameT<'s, 't> { fn from(x: &'t KindPlaceholderNameT<'s, 't>) -> Self { IPlaceholderNameT::KindPlaceholder(x) } }
impl<'s, 't> From<&'t NonKindNonRegionPlaceholderNameT<'s, 't>> for IPlaceholderNameT<'s, 't> { fn from(x: &'t NonKindNonRegionPlaceholderNameT<'s, 't>) -> Self { IPlaceholderNameT::NonKindNonRegionPlaceholder(x) } }

// -- Concrete → IVarNameT ----------------------------------------------------
impl<'s, 't> From<&'t TypingPassBlockResultVarNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t TypingPassBlockResultVarNameT<'s, 't>) -> Self { IVarNameT::TypingPassBlockResultVar(x) } }
impl<'s, 't> From<&'t TypingPassFunctionResultVarNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t TypingPassFunctionResultVarNameT<'s, 't>) -> Self { IVarNameT::TypingPassFunctionResultVar(x) } }
impl<'s, 't> From<&'t TypingPassTemporaryVarNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t TypingPassTemporaryVarNameT<'s, 't>) -> Self { IVarNameT::TypingPassTemporaryVar(x) } }
impl<'s, 't> From<&'t TypingPassPatternMemberNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t TypingPassPatternMemberNameT<'s, 't>) -> Self { IVarNameT::TypingPassPatternMember(x) } }
impl<'s, 't> From<&'t TypingIgnoredParamNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t TypingIgnoredParamNameT<'s, 't>) -> Self { IVarNameT::TypingIgnoredParam(x) } }
impl<'s, 't> From<&'t TypingPassPatternDestructureeNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t TypingPassPatternDestructureeNameT<'s, 't>) -> Self { IVarNameT::TypingPassPatternDestructuree(x) } }
impl<'s, 't> From<&'t UnnamedLocalNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t UnnamedLocalNameT<'s, 't>) -> Self { IVarNameT::UnnamedLocal(x) } }
impl<'s, 't> From<&'t ClosureParamNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t ClosureParamNameT<'s, 't>) -> Self { IVarNameT::ClosureParam(x) } }
impl<'s, 't> From<&'t ConstructingMemberNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t ConstructingMemberNameT<'s, 't>) -> Self { IVarNameT::ConstructingMember(x) } }
impl<'s, 't> From<&'t WhileCondResultNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t WhileCondResultNameT<'s, 't>) -> Self { IVarNameT::WhileCondResult(x) } }
impl<'s, 't> From<&'t IterableNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t IterableNameT<'s, 't>) -> Self { IVarNameT::Iterable(x) } }
impl<'s, 't> From<&'t IteratorNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t IteratorNameT<'s, 't>) -> Self { IVarNameT::Iterator(x) } }
impl<'s, 't> From<&'t IterationOptionNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t IterationOptionNameT<'s, 't>) -> Self { IVarNameT::IterationOption(x) } }
impl<'s, 't> From<&'t MagicParamNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t MagicParamNameT<'s, 't>) -> Self { IVarNameT::MagicParam(x) } }
impl<'s, 't> From<&'t CodeVarNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t CodeVarNameT<'s, 't>) -> Self { IVarNameT::CodeVar(x) } }
impl<'s, 't> From<&'t AnonymousSubstructMemberNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t AnonymousSubstructMemberNameT<'s, 't>) -> Self { IVarNameT::AnonymousSubstructMember(x) } }
impl<'s, 't> From<&'t SelfNameT<'s, 't>> for IVarNameT<'s, 't> { fn from(x: &'t SelfNameT<'s, 't>) -> Self { IVarNameT::Self_(x) } }

// -- Concrete → CitizenNameT / CitizenTemplateNameT --------------------------
impl<'s, 't> From<&'t StructNameT<'s, 't>> for CitizenNameT<'s, 't> { fn from(x: &'t StructNameT<'s, 't>) -> Self { CitizenNameT::Struct(x) } }
impl<'s, 't> From<&'t InterfaceNameT<'s, 't>> for CitizenNameT<'s, 't> { fn from(x: &'t InterfaceNameT<'s, 't>) -> Self { CitizenNameT::Interface(x) } }
impl<'s, 't> From<&'t StructTemplateNameT<'s, 't>> for CitizenTemplateNameT<'s, 't> { fn from(x: &'t StructTemplateNameT<'s, 't>) -> Self { CitizenTemplateNameT::StructTemplate(x) } }
impl<'s, 't> From<&'t InterfaceTemplateNameT<'s, 't>> for CitizenTemplateNameT<'s, 't> { fn from(x: &'t InterfaceTemplateNameT<'s, 't>) -> Self { CitizenTemplateNameT::InterfaceTemplate(x) } }

// -- Sub-enum → wider sub-enum (cascade via .into() on inner concrete ref) ---

impl<'s, 't> From<&'t IFunctionTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(f: &'t IFunctionTemplateNameT<'s, 't>) -> Self {
        match f {
            IFunctionTemplateNameT::OverrideDispatcherTemplate(x) => (*x).into(),
            IFunctionTemplateNameT::ExternFunction(x) => (*x).into(),
            IFunctionTemplateNameT::FunctionBoundTemplate(x) => (*x).into(),
            IFunctionTemplateNameT::PredictedFunctionTemplate(x) => (*x).into(),
            IFunctionTemplateNameT::FunctionTemplate(x) => (*x).into(),
            IFunctionTemplateNameT::LambdaCallFunctionTemplate(x) => (*x).into(),
            IFunctionTemplateNameT::ForwarderFunctionTemplate(x) => (*x).into(),
            IFunctionTemplateNameT::ConstructorTemplate(x) => (*x).into(),
            IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t IFunctionNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(f: &'t IFunctionNameT<'s, 't>) -> Self {
        match f {
            IFunctionNameT::OverrideDispatcher(x) => (*x).into(),
            IFunctionNameT::ExternFunction(x) => (*x).into(),
            IFunctionNameT::Function(x) => (*x).into(),
            IFunctionNameT::ForwarderFunction(x) => (*x).into(),
            IFunctionNameT::FunctionBound(x) => (*x).into(),
            IFunctionNameT::PredictedFunction(x) => (*x).into(),
            IFunctionNameT::LambdaCallFunction(x) => (*x).into(),
            IFunctionNameT::AnonymousSubstructConstructor(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t ISuperKindTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(f: &'t ISuperKindTemplateNameT<'s, 't>) -> Self {
        match f {
            ISuperKindTemplateNameT::KindPlaceholderTemplate(x) => (*x).into(),
            ISuperKindTemplateNameT::InterfaceTemplate(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t ISubKindTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(f: &'t ISubKindTemplateNameT<'s, 't>) -> Self {
        match f {
            ISubKindTemplateNameT::StaticSizedArrayTemplate(x) => (*x).into(),
            ISubKindTemplateNameT::RuntimeSizedArrayTemplate(x) => (*x).into(),
            ISubKindTemplateNameT::KindPlaceholderTemplate(x) => (*x).into(),
            ISubKindTemplateNameT::LambdaCitizenTemplate(x) => (*x).into(),
            ISubKindTemplateNameT::StructTemplate(x) => (*x).into(),
            ISubKindTemplateNameT::InterfaceTemplate(x) => (*x).into(),
            ISubKindTemplateNameT::AnonymousSubstructTemplate(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t ICitizenTemplateNameT<'s, 't>> for ISubKindTemplateNameT<'s, 't> {
    fn from(f: &'t ICitizenTemplateNameT<'s, 't>) -> Self {
        match f {
            ICitizenTemplateNameT::StaticSizedArrayTemplate(x) => (*x).into(),
            ICitizenTemplateNameT::RuntimeSizedArrayTemplate(x) => (*x).into(),
            ICitizenTemplateNameT::LambdaCitizenTemplate(x) => (*x).into(),
            ICitizenTemplateNameT::StructTemplate(x) => (*x).into(),
            ICitizenTemplateNameT::InterfaceTemplate(x) => (*x).into(),
            ICitizenTemplateNameT::AnonymousSubstructTemplate(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t IStructTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> {
    fn from(f: &'t IStructTemplateNameT<'s, 't>) -> Self {
        match f {
            IStructTemplateNameT::LambdaCitizenTemplate(x) => (*x).into(),
            IStructTemplateNameT::StructTemplate(x) => (*x).into(),
            IStructTemplateNameT::AnonymousSubstructTemplate(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t IInterfaceTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> {
    fn from(f: &'t IInterfaceTemplateNameT<'s, 't>) -> Self {
        match f {
            IInterfaceTemplateNameT::InterfaceTemplate(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t ISuperKindNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(f: &'t ISuperKindNameT<'s, 't>) -> Self {
        match f {
            ISuperKindNameT::KindPlaceholder(x) => (*x).into(),
            ISuperKindNameT::Interface(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t ISubKindNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(f: &'t ISubKindNameT<'s, 't>) -> Self {
        match f {
            ISubKindNameT::StaticSizedArray(x) => (*x).into(),
            ISubKindNameT::RuntimeSizedArray(x) => (*x).into(),
            ISubKindNameT::KindPlaceholder(x) => (*x).into(),
            ISubKindNameT::Struct(x) => (*x).into(),
            ISubKindNameT::Interface(x) => (*x).into(),
            ISubKindNameT::LambdaCitizen(x) => (*x).into(),
            ISubKindNameT::AnonymousSubstruct(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t ICitizenNameT<'s, 't>> for ISubKindNameT<'s, 't> {
    fn from(f: &'t ICitizenNameT<'s, 't>) -> Self {
        match f {
            ICitizenNameT::StaticSizedArray(x) => (*x).into(),
            ICitizenNameT::RuntimeSizedArray(x) => (*x).into(),
            ICitizenNameT::Struct(x) => (*x).into(),
            ICitizenNameT::Interface(x) => (*x).into(),
            ICitizenNameT::LambdaCitizen(x) => (*x).into(),
            ICitizenNameT::AnonymousSubstruct(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t IStructNameT<'s, 't>> for ICitizenNameT<'s, 't> {
    fn from(f: &'t IStructNameT<'s, 't>) -> Self {
        match f {
            IStructNameT::Struct(x) => (*x).into(),
            IStructNameT::LambdaCitizen(x) => (*x).into(),
            IStructNameT::AnonymousSubstruct(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t IInterfaceNameT<'s, 't>> for ICitizenNameT<'s, 't> {
    fn from(f: &'t IInterfaceNameT<'s, 't>) -> Self {
        match f {
            IInterfaceNameT::Interface(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t IImplTemplateNameT<'s, 't>> for ITemplateNameT<'s, 't> {
    fn from(f: &'t IImplTemplateNameT<'s, 't>) -> Self {
        match f {
            IImplTemplateNameT::ImplTemplate(x) => (*x).into(),
            IImplTemplateNameT::ImplBoundTemplate(x) => (*x).into(),
            IImplTemplateNameT::AnonymousSubstructImplTemplate(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t IImplNameT<'s, 't>> for IInstantiationNameT<'s, 't> {
    fn from(f: &'t IImplNameT<'s, 't>) -> Self {
        match f {
            IImplNameT::Impl(x) => (*x).into(),
            IImplNameT::ImplBound(x) => (*x).into(),
            IImplNameT::AnonymousSubstructImpl(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t IPlaceholderNameT<'s, 't>> for INameT<'s, 't> {
    fn from(f: &'t IPlaceholderNameT<'s, 't>) -> Self {
        match f {
            IPlaceholderNameT::KindPlaceholder(x) => (*x).into(),
            IPlaceholderNameT::NonKindNonRegionPlaceholder(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t IVarNameT<'s, 't>> for INameT<'s, 't> {
    fn from(f: &'t IVarNameT<'s, 't>) -> Self {
        match f {
            IVarNameT::TypingPassBlockResultVar(x) => (*x).into(),
            IVarNameT::TypingPassFunctionResultVar(x) => (*x).into(),
            IVarNameT::TypingPassTemporaryVar(x) => (*x).into(),
            IVarNameT::TypingPassPatternMember(x) => (*x).into(),
            IVarNameT::TypingIgnoredParam(x) => (*x).into(),
            IVarNameT::TypingPassPatternDestructuree(x) => (*x).into(),
            IVarNameT::UnnamedLocal(x) => (*x).into(),
            IVarNameT::ClosureParam(x) => (*x).into(),
            IVarNameT::ConstructingMember(x) => (*x).into(),
            IVarNameT::WhileCondResult(x) => (*x).into(),
            IVarNameT::Iterable(x) => (*x).into(),
            IVarNameT::Iterator(x) => (*x).into(),
            IVarNameT::IterationOption(x) => (*x).into(),
            IVarNameT::MagicParam(x) => (*x).into(),
            IVarNameT::CodeVar(x) => (*x).into(),
            IVarNameT::AnonymousSubstructMember(x) => (*x).into(),
            IVarNameT::Self_(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t ITemplateNameT<'s, 't>> for INameT<'s, 't> {
    fn from(f: &'t ITemplateNameT<'s, 't>) -> Self {
        match f {
            ITemplateNameT::ExportTemplate(x) => (*x).into(),
            ITemplateNameT::ImplTemplate(x) => (*x).into(),
            ITemplateNameT::ImplBoundTemplate(x) => (*x).into(),
            ITemplateNameT::StaticSizedArrayTemplate(x) => (*x).into(),
            ITemplateNameT::RuntimeSizedArrayTemplate(x) => (*x).into(),
            ITemplateNameT::KindPlaceholderTemplate(x) => (*x).into(),
            ITemplateNameT::OverrideDispatcherTemplate(x) => (*x).into(),
            ITemplateNameT::OverrideDispatcherCase(x) => (*x).into(),
            ITemplateNameT::ExternTemplate(x) => (*x).into(),
            ITemplateNameT::ExternFunction(x) => (*x).into(),
            ITemplateNameT::FunctionBoundTemplate(x) => (*x).into(),
            ITemplateNameT::PredictedFunctionTemplate(x) => (*x).into(),
            ITemplateNameT::FunctionTemplate(x) => (*x).into(),
            ITemplateNameT::LambdaCallFunctionTemplate(x) => (*x).into(),
            ITemplateNameT::ForwarderFunctionTemplate(x) => (*x).into(),
            ITemplateNameT::ConstructorTemplate(x) => (*x).into(),
            ITemplateNameT::LambdaCitizenTemplate(x) => (*x).into(),
            ITemplateNameT::StructTemplate(x) => (*x).into(),
            ITemplateNameT::InterfaceTemplate(x) => (*x).into(),
            ITemplateNameT::AnonymousSubstructImplTemplate(x) => (*x).into(),
            ITemplateNameT::AnonymousSubstructTemplate(x) => (*x).into(),
            ITemplateNameT::AnonymousSubstructConstructorTemplate(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t IInstantiationNameT<'s, 't>> for INameT<'s, 't> {
    fn from(f: &'t IInstantiationNameT<'s, 't>) -> Self {
        match f {
            IInstantiationNameT::Export(x) => (*x).into(),
            IInstantiationNameT::Impl(x) => (*x).into(),
            IInstantiationNameT::ImplBound(x) => (*x).into(),
            IInstantiationNameT::StaticSizedArray(x) => (*x).into(),
            IInstantiationNameT::RuntimeSizedArray(x) => (*x).into(),
            IInstantiationNameT::KindPlaceholder(x) => (*x).into(),
            IInstantiationNameT::OverrideDispatcher(x) => (*x).into(),
            IInstantiationNameT::OverrideDispatcherCase(x) => (*x).into(),
            IInstantiationNameT::Extern(x) => (*x).into(),
            IInstantiationNameT::ExternFunction(x) => (*x).into(),
            IInstantiationNameT::Function(x) => (*x).into(),
            IInstantiationNameT::ForwarderFunction(x) => (*x).into(),
            IInstantiationNameT::FunctionBound(x) => (*x).into(),
            IInstantiationNameT::PredictedFunction(x) => (*x).into(),
            IInstantiationNameT::LambdaCallFunction(x) => (*x).into(),
            IInstantiationNameT::Struct(x) => (*x).into(),
            IInstantiationNameT::Interface(x) => (*x).into(),
            IInstantiationNameT::LambdaCitizen(x) => (*x).into(),
            IInstantiationNameT::AnonymousSubstructImpl(x) => (*x).into(),
            IInstantiationNameT::AnonymousSubstructConstructor(x) => (*x).into(),
            IInstantiationNameT::AnonymousSubstruct(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t CitizenNameT<'s, 't>> for ICitizenNameT<'s, 't> {
    fn from(f: &'t CitizenNameT<'s, 't>) -> Self {
        match f {
            CitizenNameT::Struct(x) => (*x).into(),
            CitizenNameT::Interface(x) => (*x).into(),
        }
    }
}

impl<'s, 't> From<&'t CitizenTemplateNameT<'s, 't>> for ICitizenTemplateNameT<'s, 't> {
    fn from(f: &'t CitizenTemplateNameT<'s, 't>) -> Self {
        match f {
            CitizenTemplateNameT::StructTemplate(x) => (*x).into(),
            CitizenTemplateNameT::InterfaceTemplate(x) => (*x).into(),
        }
    }
}

// -- TryFrom<&'t INameT> for &'t IYyyNameT (interner required; panic stubs) --
// These are the wide→narrow conversions. Producing an &'t to an arena-allocated
// narrower enum requires the TypingInterner, which is still a panic-stub.
// Kept here so `IdT::try_narrow<U>` bound `&'t INameT: TryInto<U>` resolves at
// bound-check time; at runtime these todo! until Slab 3+ fills the interner.

impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t ITemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t ITemplateNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t IInstantiationNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t IInstantiationNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t IFunctionTemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t IFunctionTemplateNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t IFunctionNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t IFunctionNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t ISuperKindTemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t ISuperKindTemplateNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t ISubKindTemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t ISubKindTemplateNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t ICitizenTemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t ICitizenTemplateNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t IStructTemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t IStructTemplateNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t IInterfaceTemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t IInterfaceTemplateNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t ISuperKindNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t ISuperKindNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t ISubKindNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t ISubKindNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t ICitizenNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t ICitizenNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t IStructNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t IStructNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t IInterfaceNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t IInterfaceNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t IImplTemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t IImplTemplateNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t IImplNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t IImplNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t IPlaceholderNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t IPlaceholderNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t IVarNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t IVarNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t CitizenNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t CitizenNameT requires TypingInterner")
    }
}
impl<'s, 't> TryFrom<&'t INameT<'s, 't>> for &'t CitizenTemplateNameT<'s, 't> {
    type Error = ();
    fn try_from(_n: &'t INameT<'s, 't>) -> Result<Self, ()> {
        todo!("TryFrom<&'t INameT> for &'t CitizenTemplateNameT requires TypingInterner")
    }
}
