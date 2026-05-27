use crate::interner::StrI;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::{CodeLocationS, RangeS};
use crate::postparsing::names::IRuneS;
use crate::instantiating::ast::types::{CoordI, ICitizenIT, MutabilityI, VariabilityI};
use crate::instantiating::ast::templata::{ITemplataI, CoordTemplataI, RegionTemplataI};
use crate::instantiating::ast::ast::LocationInFunctionEnvironmentI;
use crate::typing::types::types::CoordT;

/*
package dev.vale.instantiating.ast

import dev.vale._
import dev.vale.instantiating.ast.ITemplataI._
import dev.vale.postparsing._
import dev.vale.typing.types.CoordT

// Scout's/Astronomer's name parts correspond to where they are in the source code,
// but Compiler's correspond more to what packages and stamped functions / structs
// they're in. See TNAD.
*/
// mig: struct IdI
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IdI<'s, 'i, R> {
    pub package_coord: &'s PackageCoordinate<'s>,
    pub init_steps: &'i [INameI<'s, 'i, R>],
    pub local_name: INameI<'s, 'i, R>,
}
// mig: impl IdI
/*
case class IdI[+R <: IRegionsModeI, +I <: INameI[R]](
  packageCoord: PackageCoordinate,
  initSteps: Vector[INameI[R]],
  localName: I
) {
  // PackageTopLevelName2 is just here because names have to have a last step.
  vassert(initSteps.collectFirst({ case PackageTopLevelNameI() => }).isEmpty)

  vcurious(initSteps.distinct == initSteps)

*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for IdI` above.)
/*
  override def equals(obj: Any): Boolean = {
    obj match {
      case IdI(thatPackageCoord, thatInitSteps, thatLast) => {
        packageCoord == thatPackageCoord && initSteps == thatInitSteps && localName == thatLast
      }
      case _ => false
    }
  }

*/
// mig: fn package_id
// (was cfg-gated)
impl<'s, 'i, R> IdI<'s, 'i, R> {
    pub fn package_id(&self) -> IdI<'s, 'i, R> { panic!("Unimplemented: package_id"); }
}
/*
  def packageId: IdI[R, PackageTopLevelNameI[R]] = {
    IdI(packageCoord, Vector(), PackageTopLevelNameI())
  }

*/
// mig: fn init_id
// (was cfg-gated)
impl<'s, 'i, R> IdI<'s, 'i, R> {
    pub fn init_id(&self) -> IdI<'s, 'i, R> { panic!("Unimplemented: init_id"); }
}
/*
  def initId: IdI[R, INameI[R]] = {
    if (initSteps.isEmpty) {
      IdI(packageCoord, Vector(), PackageTopLevelNameI())
    } else {
      IdI(packageCoord, initSteps.init, initSteps.last)
    }
  }

*/
// mig: fn init_non_package_id
// (was cfg-gated)
impl<'s, 'i, R> IdI<'s, 'i, R> {
    pub fn init_non_package_id(&self) -> Option<IdI<'s, 'i, R>> { panic!("Unimplemented: init_non_package_id"); }
}
/*
  def initNonPackageId(): Option[IdI[R, INameI[R]]] = {
    if (initSteps.isEmpty) {
      None
    } else {
      Some(IdI(packageCoord, initSteps.init, initSteps.last))
    }
  }

*/
// mig: fn steps
// (was cfg-gated)
impl<'s, 'i, R> IdI<'s, 'i, R> {
    pub fn steps(&self) -> &'i[INameI<'s, 'i, R>] { panic!("Unimplemented: steps"); }
}
/*
  def steps: Vector[INameI[R]] = {
    localName match {
      case PackageTopLevelNameI() => initSteps
      case _ => initSteps :+ localName
    }
  }
}

object INameI {
*/
// mig: fn add_step
// (was cfg-gated)
pub fn add_step<'s, 'i, R>(old: &IdI<'s, 'i, R>, new_last: INameI<'s, 'i, R>) -> IdI<'s, 'i, R> { panic!("Unimplemented: add_step"); }
/*
  def addStep[R <: IRegionsModeI, I <: INameI[R], Y <: INameI[R]](old: IdI[R, I], newLast: Y): IdI[R, Y] = {
    IdI[R, Y](old.packageCoord, old.steps, newLast)
  }
}

*/
// mig: enum INameI
/// Polyvalue (see @TFITCX) — derive Eq/Hash; never hand-roll `ptr::eq` on the outer `&self` (see @PVECFPZ).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum INameI<'s, 'i, R> {
    RegionName(&'i RegionNameI<'s, 'i, R>),
    DenizenDefaultRegionName(&'i DenizenDefaultRegionNameI<'s, 'i, R>),
    ExportTemplate(&'i ExportTemplateNameI<'s, 'i, R>),
    Export(&'i ExportNameI<'s, 'i, R>),
    ExternTemplate(&'i ExternTemplateNameI<'s, 'i, R>),
    Extern(&'i ExternNameI<'s, 'i, R>),
    ImplTemplate(&'i ImplTemplateNameI<'s, 'i, R>),
    Impl(&'i ImplNameI<'s, 'i, R>),
    ImplBoundTemplate(&'i ImplBoundTemplateNameI<'s, 'i, R>),
    ImplBound(&'i ImplBoundNameI<'s, 'i, R>),
    Let(&'i LetNameI<'s, 'i, R>),
    ExportAs(&'i ExportAsNameI<'s, 'i, R>),
    RawArray(&'i RawArrayNameI<'s, 'i, R>),
    ReachablePrototype(&'i ReachablePrototypeNameI<'s, 'i, R>),
    StaticSizedArrayTemplate(&'i StaticSizedArrayTemplateNameI<'s, 'i, R>),
    StaticSizedArray(&'i StaticSizedArrayNameI<'s, 'i, R>),
    RuntimeSizedArrayTemplate(&'i RuntimeSizedArrayTemplateNameI<'s, 'i, R>),
    RuntimeSizedArray(&'i RuntimeSizedArrayNameI<'s, 'i, R>),
    OverrideDispatcherTemplate(&'i OverrideDispatcherTemplateNameI<'s, 'i, R>),
    OverrideDispatcher(&'i OverrideDispatcherNameI<'s, 'i, R>),
    OverrideDispatcherCase(&'i OverrideDispatcherCaseNameI<'s, 'i, R>),
    CaseFunctionFromImpl(&'i CaseFunctionFromImplNameI<'s, 'i, R>),
    CaseFunctionFromImplTemplate(&'i CaseFunctionFromImplTemplateNameI<'s, 'i, R>),
    TypingPassBlockResultVar(&'i TypingPassBlockResultVarNameI<'s, 'i, R>),
    TypingPassFunctionResultVar(&'i TypingPassFunctionResultVarNameI<'s, 'i, R>),
    TypingPassTemporaryVar(&'i TypingPassTemporaryVarNameI<'s, 'i, R>),
    TypingPassPatternMember(&'i TypingPassPatternMemberNameI<'s, 'i, R>),
    TypingIgnoredParam(&'i TypingIgnoredParamNameI<'s, 'i, R>),
    TypingPassPatternDestructuree(&'i TypingPassPatternDestructureeNameI<'s, 'i, R>),
    UnnamedLocal(&'i UnnamedLocalNameI<'s, 'i, R>),
    ClosureParam(&'i ClosureParamNameI<'s, 'i, R>),
    ConstructingMember(&'i ConstructingMemberNameI<'s, 'i, R>),
    WhileCondResult(&'i WhileCondResultNameI<'s, 'i, R>),
    Iterable(&'i IterableNameI<'s, 'i, R>),
    Iterator(&'i IteratorNameI<'s, 'i, R>),
    IterationOption(&'i IterationOptionNameI<'s, 'i, R>),
    MagicParam(&'i MagicParamNameI<'s, 'i, R>),
    CodeVar(&'i CodeVarNameI<'s, 'i, R>),
    AnonymousSubstructMember(&'i AnonymousSubstructMemberNameI<'s, 'i, R>),
    Primitive(&'i PrimitiveNameI<'s, 'i, R>),
    PackageTopLevel(&'i PackageTopLevelNameI<'s, 'i, R>),
    Project(&'i ProjectNameI<'s, 'i, R>),
    Package(&'i PackageNameI<'s, 'i, R>),
    Rune(&'i RuneNameI<'s, 'i, R>),
    BuildingFunctionNameWithClosureds(&'i BuildingFunctionNameWithClosuredsI<'s, 'i, R>),
    ExternFunction(&'i ExternFunctionNameI<'s, 'i, R>),
    FunctionNameIX(&'i FunctionNameIX<'s, 'i, R>),
    ForwarderFunction(&'i ForwarderFunctionNameI<'s, 'i, R>),
    FunctionBoundTemplate(&'i FunctionBoundTemplateNameI<'s, 'i, R>),
    FunctionBound(&'i FunctionBoundNameI<'s, 'i, R>),
    ReachableFunctionTemplate(&'i ReachableFunctionTemplateNameI<'s, 'i, R>),
    ReachableFunction(&'i ReachableFunctionNameI<'s, 'i, R>),
    FunctionTemplate(&'i FunctionTemplateNameI<'s, 'i, R>),
    LambdaCallFunctionTemplate(&'i LambdaCallFunctionTemplateNameI<'s, 'i, R>),
    LambdaCallFunction(&'i LambdaCallFunctionNameI<'s, 'i, R>),
    ForwarderFunctionTemplate(&'i ForwarderFunctionTemplateNameI<'s, 'i, R>),
    ConstructorTemplate(&'i ConstructorTemplateNameI<'s, 'i, R>),
    Self_(&'i SelfNameI<'s, 'i, R>),
    Arbitrary(&'i ArbitraryNameI<'s, 'i, R>),
    StructName(&'i StructNameI<'s, 'i, R>),
    InterfaceName(&'i InterfaceNameI<'s, 'i, R>),
    LambdaCitizenTemplate(&'i LambdaCitizenTemplateNameI<'s, 'i, R>),
    LambdaCitizen(&'i LambdaCitizenNameI<'s, 'i, R>),
    StructTemplate(&'i StructTemplateNameI<'s, 'i, R>),
    InterfaceTemplate(&'i InterfaceTemplateNameI<'s, 'i, R>),
    AnonymousSubstructImplTemplate(&'i AnonymousSubstructImplTemplateNameI<'s, 'i, R>),
    AnonymousSubstructImpl(&'i AnonymousSubstructImplNameI<'s, 'i, R>),
    AnonymousSubstructTemplate(&'i AnonymousSubstructTemplateNameI<'s, 'i, R>),
    AnonymousSubstructConstructorTemplate(&'i AnonymousSubstructConstructorTemplateNameI<'s, 'i, R>),
    AnonymousSubstructConstructor(&'i AnonymousSubstructConstructorNameI<'s, 'i, R>),
    AnonymousSubstruct(&'i AnonymousSubstructNameI<'s, 'i, R>),
    ResolvingEnv(&'i ResolvingEnvNameI<'s, 'i, R>),
    CallEnv(&'i CallEnvNameI<'s, 'i, R>),
}

/// Interning transient (see @TFITCX) — mirror of INameI holding payloads by value
/// (used as HashMap lookup key in the interner). Per typing-pass parity (INameValT).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum INameValI<'s, 'i, R> {
    RegionName(RegionNameI<'s, 'i, R>),
    DenizenDefaultRegionName(DenizenDefaultRegionNameI<'s, 'i, R>),
    ExportTemplate(ExportTemplateNameI<'s, 'i, R>),
    Export(ExportNameI<'s, 'i, R>),
    ExternTemplate(ExternTemplateNameI<'s, 'i, R>),
    Extern(ExternNameI<'s, 'i, R>),
    ImplTemplate(ImplTemplateNameI<'s, 'i, R>),
    Impl(ImplNameI<'s, 'i, R>),
    ImplBoundTemplate(ImplBoundTemplateNameI<'s, 'i, R>),
    ImplBound(ImplBoundNameI<'s, 'i, R>),
    Let(LetNameI<'s, 'i, R>),
    ExportAs(ExportAsNameI<'s, 'i, R>),
    RawArray(RawArrayNameI<'s, 'i, R>),
    ReachablePrototype(ReachablePrototypeNameI<'s, 'i, R>),
    StaticSizedArrayTemplate(StaticSizedArrayTemplateNameI<'s, 'i, R>),
    StaticSizedArray(StaticSizedArrayNameI<'s, 'i, R>),
    RuntimeSizedArrayTemplate(RuntimeSizedArrayTemplateNameI<'s, 'i, R>),
    RuntimeSizedArray(RuntimeSizedArrayNameI<'s, 'i, R>),
    OverrideDispatcherTemplate(OverrideDispatcherTemplateNameI<'s, 'i, R>),
    OverrideDispatcher(OverrideDispatcherNameI<'s, 'i, R>),
    OverrideDispatcherCase(OverrideDispatcherCaseNameI<'s, 'i, R>),
    CaseFunctionFromImpl(CaseFunctionFromImplNameI<'s, 'i, R>),
    CaseFunctionFromImplTemplate(CaseFunctionFromImplTemplateNameI<'s, 'i, R>),
    TypingPassBlockResultVar(TypingPassBlockResultVarNameI<'s, 'i, R>),
    TypingPassFunctionResultVar(TypingPassFunctionResultVarNameI<'s, 'i, R>),
    TypingPassTemporaryVar(TypingPassTemporaryVarNameI<'s, 'i, R>),
    TypingPassPatternMember(TypingPassPatternMemberNameI<'s, 'i, R>),
    TypingIgnoredParam(TypingIgnoredParamNameI<'s, 'i, R>),
    TypingPassPatternDestructuree(TypingPassPatternDestructureeNameI<'s, 'i, R>),
    UnnamedLocal(UnnamedLocalNameI<'s, 'i, R>),
    ClosureParam(ClosureParamNameI<'s, 'i, R>),
    ConstructingMember(ConstructingMemberNameI<'s, 'i, R>),
    WhileCondResult(WhileCondResultNameI<'s, 'i, R>),
    Iterable(IterableNameI<'s, 'i, R>),
    Iterator(IteratorNameI<'s, 'i, R>),
    IterationOption(IterationOptionNameI<'s, 'i, R>),
    MagicParam(MagicParamNameI<'s, 'i, R>),
    CodeVar(CodeVarNameI<'s, 'i, R>),
    AnonymousSubstructMember(AnonymousSubstructMemberNameI<'s, 'i, R>),
    Primitive(PrimitiveNameI<'s, 'i, R>),
    PackageTopLevel(PackageTopLevelNameI<'s, 'i, R>),
    Project(ProjectNameI<'s, 'i, R>),
    Package(PackageNameI<'s, 'i, R>),
    Rune(RuneNameI<'s, 'i, R>),
    BuildingFunctionNameWithClosureds(BuildingFunctionNameWithClosuredsI<'s, 'i, R>),
    ExternFunction(ExternFunctionNameI<'s, 'i, R>),
    FunctionNameIX(FunctionNameIX<'s, 'i, R>),
    ForwarderFunction(ForwarderFunctionNameI<'s, 'i, R>),
    FunctionBoundTemplate(FunctionBoundTemplateNameI<'s, 'i, R>),
    FunctionBound(FunctionBoundNameI<'s, 'i, R>),
    ReachableFunctionTemplate(ReachableFunctionTemplateNameI<'s, 'i, R>),
    ReachableFunction(ReachableFunctionNameI<'s, 'i, R>),
    FunctionTemplate(FunctionTemplateNameI<'s, 'i, R>),
    LambdaCallFunctionTemplate(LambdaCallFunctionTemplateNameI<'s, 'i, R>),
    LambdaCallFunction(LambdaCallFunctionNameI<'s, 'i, R>),
    ForwarderFunctionTemplate(ForwarderFunctionTemplateNameI<'s, 'i, R>),
    ConstructorTemplate(ConstructorTemplateNameI<'s, 'i, R>),
    Self_(SelfNameI<'s, 'i, R>),
    Arbitrary(ArbitraryNameI<'s, 'i, R>),
    StructName(StructNameI<'s, 'i, R>),
    InterfaceName(InterfaceNameI<'s, 'i, R>),
    LambdaCitizenTemplate(LambdaCitizenTemplateNameI<'s, 'i, R>),
    LambdaCitizen(LambdaCitizenNameI<'s, 'i, R>),
    StructTemplate(StructTemplateNameI<'s, 'i, R>),
    InterfaceTemplate(InterfaceTemplateNameI<'s, 'i, R>),
    AnonymousSubstructImplTemplate(AnonymousSubstructImplTemplateNameI<'s, 'i, R>),
    AnonymousSubstructImpl(AnonymousSubstructImplNameI<'s, 'i, R>),
    AnonymousSubstructTemplate(AnonymousSubstructTemplateNameI<'s, 'i, R>),
    AnonymousSubstructConstructorTemplate(AnonymousSubstructConstructorTemplateNameI<'s, 'i, R>),
    AnonymousSubstructConstructor(AnonymousSubstructConstructorNameI<'s, 'i, R>),
    AnonymousSubstruct(AnonymousSubstructNameI<'s, 'i, R>),
    ResolvingEnv(ResolvingEnvNameI<'s, 'i, R>),
    CallEnv(CallEnvNameI<'s, 'i, R>),
}

// mig: impl INameI
/*
sealed trait INameI[+R <: IRegionsModeI]
*/
// mig: enum ITemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum ITemplateNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl ITemplateNameI
/*
sealed trait ITemplateNameI[+R <: IRegionsModeI] extends INameI[R]
*/
// mig: enum IFunctionTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IFunctionTemplateNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl IFunctionTemplateNameI
/*
sealed trait IFunctionTemplateNameI[+R <: IRegionsModeI] extends ITemplateNameI[R] {
//  def makeFunctionName(keywords: Keywords, templateArgs: Vector[ITemplataI[R]], params: Vector[CoordI]): IFunctionNameI
}
*/
// mig: enum IInstantiationNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IInstantiationNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl IInstantiationNameI
/*
sealed trait IInstantiationNameI[+R <: IRegionsModeI] extends INameI[R] {
  def template: ITemplateNameI[R]
  def templateArgs: Vector[ITemplataI[R]]
}
*/
// mig: enum IFunctionNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IFunctionNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl IFunctionNameI
/*
sealed trait IFunctionNameI[+R <: IRegionsModeI] extends IInstantiationNameI[R] {
  def template: IFunctionTemplateNameI[R]
  def templateArgs: Vector[ITemplataI[R]]
  def parameters: Vector[CoordI[R]]
}
*/
// mig: enum ISuperKindTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum ISuperKindTemplateNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl ISuperKindTemplateNameI
/*
sealed trait ISuperKindTemplateNameI[+R <: IRegionsModeI] extends ITemplateNameI[R]
*/
// mig: enum ISubKindTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum ISubKindTemplateNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl ISubKindTemplateNameI
/*
sealed trait ISubKindTemplateNameI[+R <: IRegionsModeI] extends ITemplateNameI[R]
*/
// mig: enum ICitizenTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum ICitizenTemplateNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl ICitizenTemplateNameI
/*
sealed trait ICitizenTemplateNameI[+R <: IRegionsModeI] extends ISubKindTemplateNameI[R] {
//  def makeCitizenName(templateArgs: Vector[ITemplataI[R]]): ICitizenNameI
}
*/
// mig: enum IStructTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IStructTemplateNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl IStructTemplateNameI
/*
sealed trait IStructTemplateNameI[+R <: IRegionsModeI] extends ICitizenTemplateNameI[R] {
//  def makeStructName(templateArgs: Vector[ITemplataI[R]]): IStructNameI
//  override def makeCitizenName(templateArgs: Vector[ITemplataI[R]]):
//  ICitizenNameI = {
//    makeStructName(templateArgs)
//  }
}
*/
// mig: enum IInterfaceTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IInterfaceTemplateNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl IInterfaceTemplateNameI
/*
sealed trait IInterfaceTemplateNameI[+R <: IRegionsModeI] extends ICitizenTemplateNameI[R] with ISuperKindTemplateNameI[R] {
//  def makeInterfaceName(templateArgs: Vector[ITemplataI[R]]): IInterfaceNameI
}
*/
// mig: enum ISuperKindNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum ISuperKindNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl ISuperKindNameI
/*
sealed trait ISuperKindNameI[+R <: IRegionsModeI] extends IInstantiationNameI[R] {
  def template: ISuperKindTemplateNameI[R]
  def templateArgs: Vector[ITemplataI[R]]
}
*/
// mig: enum ISubKindNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum ISubKindNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl ISubKindNameI
/*
sealed trait ISubKindNameI[+R <: IRegionsModeI] extends IInstantiationNameI[R] {
  def template: ISubKindTemplateNameI[R]
  def templateArgs: Vector[ITemplataI[R]]
}
*/
// mig: enum ICitizenNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum ICitizenNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl ICitizenNameI
/*
sealed trait ICitizenNameI[+R <: IRegionsModeI] extends ISubKindNameI[R] {
  def template: ICitizenTemplateNameI[R]
  def templateArgs: Vector[ITemplataI[R]]
}
*/
// mig: enum IStructNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IStructNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl IStructNameI
/*
sealed trait IStructNameI[+R <: IRegionsModeI] extends ICitizenNameI[R] with ISubKindNameI[R] {
  override def template: IStructTemplateNameI[R]
  override def templateArgs: Vector[ITemplataI[R]]
}
*/
// mig: enum IInterfaceNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IInterfaceNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl IInterfaceNameI
/*
sealed trait IInterfaceNameI[+R <: IRegionsModeI] extends ICitizenNameI[R] with ISubKindNameI[R] with ISuperKindNameI[R] {
  override def template: IInterfaceTemplateNameI[R]
  override def templateArgs: Vector[ITemplataI[R]]
}
*/
// mig: enum IImplTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IImplTemplateNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl IImplTemplateNameI
/*
sealed trait IImplTemplateNameI[+R <: IRegionsModeI] extends ITemplateNameI[R] {
//  def makeImplName(templateArgs: Vector[ITemplataI[R]], subCitizen: ICitizenIT): IImplNameI
}
*/
// mig: enum IImplNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IImplNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl IImplNameI
/*
sealed trait IImplNameI[+R <: IRegionsModeI] extends IInstantiationNameI[R] {
  def template: IImplTemplateNameI[R]
}

*/
// mig: enum IRegionNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IRegionNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl IRegionNameI
/*
sealed trait IRegionNameI[+R <: IRegionsModeI] extends INameI[R]
*/
// mig: struct RegionNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct RegionNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub rune: IRuneS<'s>,
}
/*
case class RegionNameI[+R <: IRegionsModeI](rune: IRuneS) extends IRegionNameI[R]
*/
// mig: struct DenizenDefaultRegionNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct DenizenDefaultRegionNameI<'s, 'i, R>(pub std::marker::PhantomData<(&'s (), &'i (), R)>);
/*
case class DenizenDefaultRegionNameI[+R <: IRegionsModeI]() extends IRegionNameI[R]
*/
// mig: struct ExportTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ExportTemplateNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub code_loc: CodeLocationS<'s>,
}
/*
case class ExportTemplateNameI[+R <: IRegionsModeI](codeLoc: CodeLocationS) extends ITemplateNameI[R]
*/
// mig: struct ExportNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ExportNameI<'s, 'i, R> {
    pub template: ExportTemplateNameI<'s, 'i, R>,
    pub region: RegionTemplataI<'s, 'i, R>,
}
/*
case class ExportNameI[+R <: IRegionsModeI](
  template: ExportTemplateNameI[R],
  region: RegionTemplataI[R]
) extends IInstantiationNameI[R] {
  override def templateArgs: Vector[ITemplataI[R]] = Vector(region)
}

*/
// mig: struct ExternTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ExternTemplateNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub code_loc: CodeLocationS<'s>,
}
/*
case class ExternTemplateNameI[+R <: IRegionsModeI](codeLoc: CodeLocationS) extends ITemplateNameI[R]
*/
// mig: struct ExternNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ExternNameI<'s, 'i, R> {
    pub template: ExternTemplateNameI<'s, 'i, R>,
    pub region: RegionTemplataI<'s, 'i, R>,
}
/*
case class ExternNameI[+R <: IRegionsModeI](
  template: ExternTemplateNameI[R],
  region: RegionTemplataI[R]
) extends IInstantiationNameI[R] {
  override def templateArgs: Vector[ITemplataI[R]] = Vector(region)
}

*/
// mig: struct ImplTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ImplTemplateNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub code_location_s: CodeLocationS<'s>,
}
/*
case class ImplTemplateNameI[+R <: IRegionsModeI](codeLocationS: CodeLocationS) extends IImplTemplateNameI[R] {
  vpass()
//  override def makeImplName(templateArgs: Vector[ITemplataI[R]], subCitizen: ICitizenIT): ImplNameI = {
//    ImplNameI(this, templateArgs, subCitizen)
//  }
}
*/
// mig: struct ImplNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ImplNameI<'s, 'i, R> {
    pub template: IImplTemplateNameI<'s, 'i, R>,
    pub template_args: &'i[ITemplataI<'s, 'i, R>],
    pub sub_citizen: ICitizenIT<'s, 'i, R>,
}
/*
case class ImplNameI[+R <: IRegionsModeI](
  template: IImplTemplateNameI[R],
  templateArgs: Vector[ITemplataI[R]],
  // The instantiator wants this so it can know the struct type up-front before monomorphizing the
  // whole impl, so it can hoist some bounds out of the struct, like NBIFP.
  subCitizen: ICitizenIT[R]
) extends IImplNameI[R] {
  vpass()
}

*/
// mig: struct ImplBoundTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ImplBoundTemplateNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub code_location_s: CodeLocationS<'s>,
}
/*
case class ImplBoundTemplateNameI[+R <: IRegionsModeI](codeLocationS: CodeLocationS) extends IImplTemplateNameI[R] {
//  override def makeImplName(templateArgs: Vector[ITemplataI[R]], subCitizen: ICitizenIT): ImplBoundNameI = {
//    ImplBoundNameI(this, templateArgs)
//  }
}
*/
// mig: struct ImplBoundNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ImplBoundNameI<'s, 'i, R> {
    pub template: ImplBoundTemplateNameI<'s, 'i, R>,
    pub template_args: &'i[ITemplataI<'s, 'i, R>],
}

/*
case class ImplBoundNameI[+R <: IRegionsModeI](
  template: ImplBoundTemplateNameI[R],
  templateArgs: Vector[ITemplataI[R]]
) extends IImplNameI[R] {

}

//// The name of an impl that is subclassing some interface. To find all impls subclassing an interface,
//// look for this name.
//case class ImplImplementingSuperInterfaceNameI[+R <: IRegionsModeI](superInterface: FullNameI[IInterfaceTemplateNameI]) extends IImplTemplateNameI
//// The name of an impl that is augmenting some sub citizen. To find all impls subclassing an interface,
//// look for this name.
//case class ImplAugmentingSubCitizenNameI[+R <: IRegionsModeI](subCitizen: FullNameI[ICitizenTemplateNameI]) extends IImplTemplateNameI

*/
// mig: struct LetNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct LetNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub code_location: CodeLocationS<'s>,
}
/*
case class LetNameI[+R <: IRegionsModeI](codeLocation: CodeLocationS) extends INameI[R]
*/
// mig: struct ExportAsNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ExportAsNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub code_location: CodeLocationS<'s>,
}
/*
case class ExportAsNameI[+R <: IRegionsModeI](codeLocation: CodeLocationS) extends INameI[R]
*/
// mig: struct RawArrayNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct RawArrayNameI<'s, 'i, R> {
    pub mutability: MutabilityI,
    pub element_type: CoordTemplataI<'s, 'i, R>,
    pub self_region: RegionTemplataI<'s, 'i, R>,
}

/*
case class RawArrayNameI[+R <: IRegionsModeI](
  mutability: MutabilityI,
  elementType: CoordTemplataI[R],
  selfRegion: RegionTemplataI[R]
) extends INameI[R] {
}

*/
// mig: struct ReachablePrototypeNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ReachablePrototypeNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'s (), &'i (), R)>,
    pub num: i32,
}
/*
case class ReachablePrototypeNameI[+R <: IRegionsModeI](num: Int) extends INameI[R]
*/
// mig: struct StaticSizedArrayTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct StaticSizedArrayTemplateNameI<'s, 'i, R>(pub std::marker::PhantomData<(&'s (), &'i (), R)>);
/*
case class StaticSizedArrayTemplateNameI[+R <: IRegionsModeI]() extends ICitizenTemplateNameI[R] {
//  override def makeCitizenName(templateArgs: Vector[ITemplataI[R]]): ICitizenNameI = {
//    vassert(templateArgs.size == 5)
//    val size = expectIntegerTemplata(templateArgs(0)).value
//    val mutability = expectMutabilityTemplata(templateArgs(1)).mutability
//    val variability = expectVariabilityTemplata(templateArgs(2)).variability
//    val elementType = expectCoordTemplata(templateArgs(3)).coord
//    val selfRegion = expectRegionTemplata(templateArgs(4))
//    StaticSizedArrayNameI(this, size, variability, RawArrayNameI(mutability, elementType, selfRegion))
//  }
}

*/
// mig: struct StaticSizedArrayNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct StaticSizedArrayNameI<'s, 'i, R> {
    pub template: StaticSizedArrayTemplateNameI<'s, 'i, R>,
    pub size: i64,
    pub variability: VariabilityI,
    pub arr: RawArrayNameI<'s, 'i, R>,
}

/*
case class StaticSizedArrayNameI[+R <: IRegionsModeI](
  template: StaticSizedArrayTemplateNameI[R],
  size: Long,
  variability: VariabilityI,
  arr: RawArrayNameI[R]
) extends ICitizenNameI[R] {

*/
// mig: fn template_args
// (was cfg-gated)
impl<'s, 'i, R> StaticSizedArrayNameI<'s, 'i, R> {
    pub fn template_args(&self) -> &'i[ITemplataI<'s, 'i, R>] { panic!("Unimplemented: template_args"); }
}
/*
  override def templateArgs: Vector[ITemplataI[R]] = {
    Vector(
      IntegerTemplataI(size),
      MutabilityTemplataI(arr.mutability),
      VariabilityTemplataI(variability),
      arr.elementType)
  }
}
*/
// mig: struct RuntimeSizedArrayTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct RuntimeSizedArrayTemplateNameI<'s, 'i, R>(pub std::marker::PhantomData<(&'s (), &'i (), R)>);
/*
case class RuntimeSizedArrayTemplateNameI[+R <: IRegionsModeI]() extends ICitizenTemplateNameI[R] {
//  override def makeCitizenName(templateArgs: Vector[ITemplataI[R]]): ICitizenNameI = {
//    vassert(templateArgs.size == 3)
//    val mutability = expectMutabilityTemplata(templateArgs(0)).mutability
//    val elementType = expectCoordTemplata(templateArgs(1)).coord
//    val region = expectRegionTemplata(templateArgs(2))
//    RuntimeSizedArrayNameI(this, RawArrayNameI(mutability, elementType, region))
//  }
}

*/
// mig: struct RuntimeSizedArrayNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct RuntimeSizedArrayNameI<'s, 'i, R> {
    pub template: RuntimeSizedArrayTemplateNameI<'s, 'i, R>,
    pub arr: RawArrayNameI<'s, 'i, R>,
}
/*
case class RuntimeSizedArrayNameI[+R <: IRegionsModeI](
  template: RuntimeSizedArrayTemplateNameI[R],
  arr: RawArrayNameI[R]
) extends ICitizenNameI[R] {
*/
// mig: fn template_args
// (was cfg-gated)
impl<'s, 'i, R> RuntimeSizedArrayNameI<'s, 'i, R> {
    pub fn template_args(&self) -> &'i[ITemplataI<'s, 'i, R>] { panic!("Unimplemented: template_args"); }
}
/*
  override def templateArgs: Vector[ITemplataI[R]] = {
    Vector(
      MutabilityTemplataI(arr.mutability),
      arr.elementType)
  }
}
// See NNSPAFOC.
*/
// mig: struct OverrideDispatcherTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct OverrideDispatcherTemplateNameI<'s, 'i, R> {
    pub impl_id: IdI<'s, 'i, R>,
}
/*
case class OverrideDispatcherTemplateNameI[+R <: IRegionsModeI](
  implId: IdI[R, IImplTemplateNameI[R]]
) extends IFunctionTemplateNameI[R] {
//  override def makeFunctionName(
//    interner: Interner,
//    keywords: Keywords,
//    templateArgs: Vector[ITemplataI[R]],
//    params: Vector[CoordI]):
//  OverrideDispatcherNameI = {
//    interner.intern(OverrideDispatcherNameI(this, templateArgs, params))
//  }
}

*/
// mig: struct OverrideDispatcherNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct OverrideDispatcherNameI<'s, 'i, R> {
    pub template: OverrideDispatcherTemplateNameI<'s, 'i, R>,
    pub template_args: &'i[ITemplataI<'s, 'i, R>],
    pub parameters: &'i[CoordI<'s, 'i, R>],
}

/*
case class OverrideDispatcherNameI[+R <: IRegionsModeI](
  template: OverrideDispatcherTemplateNameI[R],
  // This will have placeholders in it after the typing pass.
  templateArgs: Vector[ITemplataI[R]],
  parameters: Vector[CoordI[R]]
) extends IFunctionNameI[R] {
  vpass()
}

*/
// mig: struct OverrideDispatcherCaseNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct OverrideDispatcherCaseNameI<'s, 'i, R> {
    pub independent_impl_template_args: &'i[ITemplataI<'s, 'i, R>],
}

/*
case class OverrideDispatcherCaseNameI[+R <: IRegionsModeI](
  // These are the templatas for the independent runes from the impl, like the <ZZ> for Milano, see
  // OMCNAGP.
  independentImplTemplateArgs: Vector[ITemplataI[R]]
) extends ITemplateNameI[R] with IInstantiationNameI[R] {
  override def template: ITemplateNameI[R] = this
  override def templateArgs: Vector[ITemplataI[R]] = independentImplTemplateArgs
}

*/
// mig: struct CaseFunctionFromImplNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct CaseFunctionFromImplNameI<'s, 'i, R> {
    pub template: CaseFunctionFromImplTemplateNameI<'s, 'i, R>,
    pub template_args: &'i[ITemplataI<'s, 'i, R>],
    pub parameters: &'i[CoordI<'s, 'i, R>],
}

/*
case class CaseFunctionFromImplNameI[+R <: IRegionsModeI](
    template: CaseFunctionFromImplTemplateNameI[R],
    // This will have placeholders in it after the typing pass.
    templateArgs: Vector[ITemplataI[R]],
    parameters: Vector[CoordI[R]]
) extends IFunctionNameI[R] {
  vpass()
}

*/
// mig: struct CaseFunctionFromImplTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct CaseFunctionFromImplTemplateNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub human_name: StrI<'s>,
    pub rune_in_impl: IRuneS<'s>,
    pub rune_in_citizen: IRuneS<'s>,
}

/*
case class CaseFunctionFromImplTemplateNameI[+R <: IRegionsModeI](
    humanName: StrI,
    runeInImpl: IRuneS,
    runeInCitizen: IRuneS
) extends IFunctionTemplateNameI[R] {
  vpass()
}

*/
// mig: enum IVarNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum IVarNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl IVarNameI
/*
sealed trait IVarNameI[+R <: IRegionsModeI] extends INameI[R]
*/
// mig: struct TypingPassBlockResultVarNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct TypingPassBlockResultVarNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'s (), R)>,
    pub life: LocationInFunctionEnvironmentI<'i>,
}
/*
case class TypingPassBlockResultVarNameI[+R <: IRegionsModeI](life: LocationInFunctionEnvironmentI) extends IVarNameI[R]
*/
// mig: struct TypingPassFunctionResultVarNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct TypingPassFunctionResultVarNameI<'s, 'i, R>(pub std::marker::PhantomData<(&'s (), &'i (), R)>);
/*
case class TypingPassFunctionResultVarNameI[+R <: IRegionsModeI]() extends IVarNameI[R]
*/
// mig: struct TypingPassTemporaryVarNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct TypingPassTemporaryVarNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'s (), R)>,
    pub life: LocationInFunctionEnvironmentI<'i>,
}
/*
case class TypingPassTemporaryVarNameI[+R <: IRegionsModeI](life: LocationInFunctionEnvironmentI) extends IVarNameI[R]
*/
// mig: struct TypingPassPatternMemberNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct TypingPassPatternMemberNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'s (), R)>,
    pub life: LocationInFunctionEnvironmentI<'i>,
}
/*
case class TypingPassPatternMemberNameI[+R <: IRegionsModeI](life: LocationInFunctionEnvironmentI) extends IVarNameI[R]
*/
// mig: struct TypingIgnoredParamNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct TypingIgnoredParamNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'s (), &'i (), R)>,
    pub num: i32,
}
/*
case class TypingIgnoredParamNameI[+R <: IRegionsModeI](num: Int) extends IVarNameI[R]
*/
// mig: struct TypingPassPatternDestructureeNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct TypingPassPatternDestructureeNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'s (), R)>,
    pub life: LocationInFunctionEnvironmentI<'i>,
}
/*
case class TypingPassPatternDestructureeNameI[+R <: IRegionsModeI](life: LocationInFunctionEnvironmentI) extends IVarNameI[R]
*/
// mig: struct UnnamedLocalNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct UnnamedLocalNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub code_location: CodeLocationS<'s>,
}
/*
case class UnnamedLocalNameI[+R <: IRegionsModeI](codeLocation: CodeLocationS) extends IVarNameI[R]
*/
// mig: struct ClosureParamNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ClosureParamNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub code_location: CodeLocationS<'s>,
}
/*
case class ClosureParamNameI[+R <: IRegionsModeI](codeLocation: CodeLocationS) extends IVarNameI[R]
*/
// mig: struct ConstructingMemberNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ConstructingMemberNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub name: StrI<'s>,
}
/*
case class ConstructingMemberNameI[+R <: IRegionsModeI](name: StrI) extends IVarNameI[R]
*/
// mig: struct WhileCondResultNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct WhileCondResultNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub range: RangeS<'s>,
}
/*
case class WhileCondResultNameI[+R <: IRegionsModeI](range: RangeS) extends IVarNameI[R]
*/
// mig: struct IterableNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct IterableNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub range: RangeS<'s>,
}
/*
case class IterableNameI[+R <: IRegionsModeI](range: RangeS) extends IVarNameI[R] {  }
*/
// mig: struct IteratorNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct IteratorNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub range: RangeS<'s>,
}
/*
case class IteratorNameI[+R <: IRegionsModeI](range: RangeS) extends IVarNameI[R] {  }
*/
// mig: struct IterationOptionNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct IterationOptionNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub range: RangeS<'s>,
}
/*
case class IterationOptionNameI[+R <: IRegionsModeI](range: RangeS) extends IVarNameI[R] {  }
*/
// mig: struct MagicParamNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct MagicParamNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub code_location_2: CodeLocationS<'s>,
}
/*
case class MagicParamNameI[+R <: IRegionsModeI](codeLocation2: CodeLocationS) extends IVarNameI[R]
*/
// mig: struct CodeVarNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct CodeVarNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub name: StrI<'s>,
}
/*
case class CodeVarNameI[+R <: IRegionsModeI](name: StrI) extends IVarNameI[R]
// We dont use CodeVarName2(0), CodeVarName2(1) etc because we dont want the user to address these members directly.
*/
// mig: struct AnonymousSubstructMemberNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct AnonymousSubstructMemberNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'s (), &'i (), R)>,
    pub index: i32,
}
/*
case class AnonymousSubstructMemberNameI[+R <: IRegionsModeI](index: Int) extends IVarNameI[R]
*/
// mig: struct PrimitiveNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct PrimitiveNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub human_name: StrI<'s>,
}
/*
case class PrimitiveNameI[+R <: IRegionsModeI](humanName: StrI) extends INameI[R]
// Only made in typingpass
*/
// mig: struct PackageTopLevelNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct PackageTopLevelNameI<'s, 'i, R>(pub std::marker::PhantomData<(&'s (), &'i (), R)>);
/*
case class PackageTopLevelNameI[+R <: IRegionsModeI]() extends INameI[R]
*/
// mig: struct ProjectNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ProjectNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub name: StrI<'s>,
}
/*
case class ProjectNameI[+R <: IRegionsModeI](name: StrI) extends INameI[R]
*/
// mig: struct PackageNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct PackageNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub name: StrI<'s>,
}
/*
case class PackageNameI[+R <: IRegionsModeI](name: StrI) extends INameI[R]
*/
// mig: struct RuneNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct RuneNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub rune: IRuneS<'s>,
}
/*
case class RuneNameI[+R <: IRegionsModeI](rune: IRuneS) extends INameI[R]

// This is the name of a function that we're still figuring out in the function typingpass.
// We have its closured variables, but are still figuring out its template args and params.
*/
// mig: struct BuildingFunctionNameWithClosuredsI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct BuildingFunctionNameWithClosuredsI<'s, 'i, R> {
    pub template_name: IFunctionTemplateNameI<'s, 'i, R>,
}

/*
case class BuildingFunctionNameWithClosuredsI[+R <: IRegionsModeI](
  templateName: IFunctionTemplateNameI[R],
) extends INameI[R] {



}

*/
// mig: struct ExternFunctionNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ExternFunctionNameI<'s, 'i, R> {
    pub human_name: StrI<'s>,
    pub template_args: &'i[ITemplataI<'s, 'i, R>],
    pub parameters: &'i[CoordI<'s, 'i, R>],
}
/*
case class ExternFunctionNameI[+R <: IRegionsModeI](
  humanName: StrI,
  templateArgs: Vector[ITemplataI[R]],
  parameters: Vector[CoordI[R]]
) extends IFunctionNameI[R] with IFunctionTemplateNameI[R] {
  override def template: IFunctionTemplateNameI[R] = this
}

*/
// mig: struct FunctionNameIX
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct FunctionNameIX<'s, 'i, R> {
    pub template: FunctionTemplateNameI<'s, 'i, R>,
    pub template_args: &'i[ITemplataI<'s, 'i, R>],
    pub parameters: &'i[CoordI<'s, 'i, R>],
}

/*
case class FunctionNameIX[+R <: IRegionsModeI](
  template: FunctionTemplateNameI[R],
  templateArgs: Vector[ITemplataI[R]],
  parameters: Vector[CoordI[R]]
) extends IFunctionNameI[R]

*/
// mig: struct ForwarderFunctionNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ForwarderFunctionNameI<'s, 'i, R> {
    pub template: ForwarderFunctionTemplateNameI<'s, 'i, R>,
    pub inner: IFunctionNameI<'s, 'i, R>,
}

/*
case class ForwarderFunctionNameI[+R <: IRegionsModeI](
  template: ForwarderFunctionTemplateNameI[R],
  inner: IFunctionNameI[R]
) extends IFunctionNameI[R] {
  override def templateArgs: Vector[ITemplataI[R]] = inner.templateArgs
  override def parameters: Vector[CoordI[R]] = inner.parameters
}

*/
// mig: struct FunctionBoundTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct FunctionBoundTemplateNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub human_name: StrI<'s>,
}

/*
case class FunctionBoundTemplateNameI[+R <: IRegionsModeI](
  humanName: StrI,
  // We used to have a CodeLocation here, but took it out because we want to merge duplicate bounds, see MFBFDP.
  //   codeLocation: CodeLocationS
) extends INameI[R] with IFunctionTemplateNameI[R] {
//  override def makeFunctionName(keywords: Keywords, templateArgs: Vector[ITemplataI[R]], params: Vector[CoordI]): FunctionBoundNameI = {
//    interner.intern(FunctionBoundNameI(this, templateArgs, params))
//  }
}

*/
// mig: struct FunctionBoundNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct FunctionBoundNameI<'s, 'i, R> {
    pub template: FunctionBoundTemplateNameI<'s, 'i, R>,
    pub template_args: &'i[ITemplataI<'s, 'i, R>],
    pub parameters: &'i[CoordI<'s, 'i, R>],
}

/*
case class FunctionBoundNameI[+R <: IRegionsModeI](
  template: FunctionBoundTemplateNameI[R],
  templateArgs: Vector[ITemplataI[R]],
  parameters: Vector[CoordI[R]]
) extends IFunctionNameI[R]

*/
// mig: struct ReachableFunctionTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ReachableFunctionTemplateNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub human_name: StrI<'s>,
}

/*
case class ReachableFunctionTemplateNameI[+R <: IRegionsModeI](
    humanName: StrI
) extends INameI[R] with IFunctionTemplateNameI[R]

*/
// mig: struct ReachableFunctionNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ReachableFunctionNameI<'s, 'i, R> {
    pub template: ReachableFunctionTemplateNameI<'s, 'i, R>,
    pub template_args: &'i[ITemplataI<'s, 'i, R>],
    pub parameters: &'i[CoordI<'s, 'i, R>],
}

/*
case class ReachableFunctionNameI[+R <: IRegionsModeI](
    template: ReachableFunctionTemplateNameI[R],
    templateArgs: Vector[ITemplataI[R]],
    parameters: Vector[CoordI[R]]
) extends IFunctionNameI[R]

*/
// mig: struct FunctionTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct FunctionTemplateNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub human_name: StrI<'s>,
    pub code_location: CodeLocationS<'s>,
}

/*
case class FunctionTemplateNameI[+R <: IRegionsModeI](
    humanName: StrI,
    codeLocation: CodeLocationS
) extends INameI[R] with IFunctionTemplateNameI[R] {
  vpass()
//  override def makeFunctionName(keywords: Keywords, templateArgs: Vector[ITemplataI[R]], params: Vector[CoordI]): IFunctionNameI = {
//    interner.intern(FunctionNameI(this, templateArgs, params))
//  }
}

*/
// Per @LAGTNGZ, paramTypes stays baked in (specialization happened earlier).
// mig: struct LambdaCallFunctionTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct LambdaCallFunctionTemplateNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub code_location: CodeLocationS<'s>,
    pub param_types: (), // was &'t [CoordT<'s, 't>] — needs proper 't lifetime, see below
}

/*
// Per @LAGTNGZ, paramTypes stays baked in (specialization happened earlier).
case class LambdaCallFunctionTemplateNameI[+R <: IRegionsModeI](
  codeLocation: CodeLocationS,
  paramTypes: Vector[CoordT]
) extends INameI[R] with IFunctionTemplateNameI[R] {
//  override def makeFunctionName(keywords: Keywords, templateArgs: Vector[ITemplataI[R]], params: Vector[CoordI]): IFunctionNameI = {
//    // Post instantiator, the params will be real, but our template paramTypes will still be placeholders
//    // vassert(params == paramTypes)
//    interner.intern(LambdaCallFunctionNameI(this, templateArgs, params))
//  }
}

*/
// mig: struct LambdaCallFunctionNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct LambdaCallFunctionNameI<'s, 'i, R> {
    pub template: LambdaCallFunctionTemplateNameI<'s, 'i, R>,
    pub template_args: &'i[ITemplataI<'s, 'i, R>],
    pub parameters: &'i[CoordI<'s, 'i, R>],
}

/*
case class LambdaCallFunctionNameI[+R <: IRegionsModeI](
  template: LambdaCallFunctionTemplateNameI[R],
  templateArgs: Vector[ITemplataI[R]],
  parameters: Vector[CoordI[R]]
) extends IFunctionNameI[R]

*/
// mig: struct ForwarderFunctionTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ForwarderFunctionTemplateNameI<'s, 'i, R> {
    pub inner: IFunctionTemplateNameI<'s, 'i, R>,
    pub index: i32,
}

/*
case class ForwarderFunctionTemplateNameI[+R <: IRegionsModeI](
  inner: IFunctionTemplateNameI[R],
  index: Int
) extends INameI[R] with IFunctionTemplateNameI[R] {
//  override def makeFunctionName(keywords: Keywords, templateArgs: Vector[ITemplataI[R]], params: Vector[CoordI]): IFunctionNameI = {
//    interner.intern(ForwarderFunctionNameI(this, inner.makeFunctionName(keywords, templateArgs, params)))//, index))
//  }
}


//case class AbstractVirtualDropFunctionTemplateNameI[+R <: IRegionsModeI](
//  implName: INameI[R]
//) extends INameI[R] with IFunctionTemplateNameI {
//  override def makeFunctionName(keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordI]): IFunctionNameI = {
//    interner.intern(
//      AbstractVirtualDropFunctionNameI(implName, templateArgs, params))
//  }
//}

//case class AbstractVirtualDropFunctionNameI[+R <: IRegionsModeI](
//  implName: INameI[R],
//  templateArgs: Vector[ITemplata[ITemplataType]],
//  parameters: Vector[CoordI]
//) extends INameI[R] with IFunctionNameI

//case class OverrideVirtualDropFunctionTemplateNameI[+R <: IRegionsModeI](
//  implName: INameI[R]
//) extends INameI[R] with IFunctionTemplateNameI {
//  override def makeFunctionName(keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordI]): IFunctionNameI = {
//    interner.intern(
//      OverrideVirtualDropFunctionNameI(implName, templateArgs, params))
//  }
//}

//case class OverrideVirtualDropFunctionNameI[+R <: IRegionsModeI](
//  implName: INameI[R],
//  templateArgs: Vector[ITemplata[ITemplataType]],
//  parameters: Vector[CoordI]
//) extends INameI[R] with IFunctionNameI

//case class LambdaTemplateNameI[+R <: IRegionsModeI](
//  codeLocation: CodeLocationS
//) extends INameI[R] with IFunctionTemplateNameI {
//  override def makeFunctionName(keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordI]): IFunctionNameI = {
//    interner.intern(FunctionNameI(interner.intern(FunctionTemplateNameI(keywords.underscoresCall, codeLocation)), templateArgs, params))
//  }
//}
*/
// mig: struct ConstructorTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ConstructorTemplateNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub code_location: CodeLocationS<'s>,
}

/*
case class ConstructorTemplateNameI[+R <: IRegionsModeI](
  codeLocation: CodeLocationS
) extends INameI[R] with IFunctionTemplateNameI[R] {
//  override def makeFunctionName(keywords: Keywords, templateArgs: Vector[ITemplataI[R]], params: Vector[CoordI]): IFunctionNameI = vimpl()
}

//case class FreeTemplateNameI[+R <: IRegionsModeI](codeLoc: CodeLocationS) extends INameI[R] with IFunctionTemplateNameI {
//  vpass()
//  override def makeFunctionName(keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordI]): IFunctionNameI = {
//    params match {
//      case Vector(coord) => {
//        interner.intern(FreeNameI(this, templateArgs, coord))
//      }
//      case other => vwat(other)
//    }
//  }
//}
//case class FreeNameI[+R <: IRegionsModeI](
//  template: FreeTemplateNameI,
//  templateArgs: Vector[ITemplata[ITemplataType]],
//  coordT: CoordI
//) extends IFunctionNameI {
//  override def parameters: Vector[CoordI] = Vector(coordI)
//}

//// See NSIDN for why we have these virtual names
//case class AbstractVirtualFreeTemplateNameI[+R <: IRegionsModeI](codeLoc: CodeLocationS) extends INameI[R] with IFunctionTemplateNameI {
//  override def makeFunctionName(keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordI]): IFunctionNameI = {
//    val Vector(CoordI(ShareI, kind)) = params
//    interner.intern(AbstractVirtualFreeNameI(templateArgs, kind))
//  }
//}
//// See NSIDN for why we have these virtual names
//case class AbstractVirtualFreeNameI[+R <: IRegionsModeI](templateArgs: Vector[ITemplata[ITemplataType]], param: KindI) extends IFunctionNameI {
//  override def parameters: Vector[CoordI] = Vector(CoordI(ShareI, param))
//}
//
//// See NSIDN for why we have these virtual names
//case class OverrideVirtualFreeTemplateNameI[+R <: IRegionsModeI](codeLoc: CodeLocationS) extends INameI[R] with IFunctionTemplateNameI {
//  override def makeFunctionName(keywords: Keywords, templateArgs: Vector[ITemplata[ITemplataType]], params: Vector[CoordI]): IFunctionNameI = {
//    val Vector(CoordI(ShareI, kind)) = params
//    interner.intern(OverrideVirtualFreeNameI(templateArgs, kind))
//  }
//}
//// See NSIDN for why we have these virtual names
//case class OverrideVirtualFreeNameI[+R <: IRegionsModeI](templateArgs: Vector[ITemplata[ITemplataType]], param: KindI) extends IFunctionNameI {
//  override def parameters: Vector[CoordI] = Vector(CoordI(ShareI, param))
//}

// Vale has no Self, its just a convenient first name parameter.
// See also SelfNameS.
*/
// mig: struct SelfNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct SelfNameI<'s, 'i, R>(pub std::marker::PhantomData<(&'s (), &'i (), R)>);
/*
case class SelfNameI[+R <: IRegionsModeI]() extends IVarNameI[R]
*/
// mig: struct ArbitraryNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ArbitraryNameI<'s, 'i, R>(pub std::marker::PhantomData<(&'s (), &'i (), R)>);
/*
case class ArbitraryNameI[+R <: IRegionsModeI]() extends INameI[R]
*/
// mig: enum CitizenNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum CitizenNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl CitizenNameI
/*
sealed trait CitizenNameI[+R <: IRegionsModeI] extends ICitizenNameI[R] {
  def template: ICitizenTemplateNameI[R]
  def templateArgs: Vector[ITemplataI[R]]
}

object CitizenNameI {
*/
// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<Wide> for Narrow` or inline match.)
/*
  def unapply[R <: IRegionsModeI](c: CitizenNameI[R]): Option[(ICitizenTemplateNameI[R], Vector[ITemplataI[R]])] = {
    c match {
      case StructNameI(template, templateArgs) => Some((template, templateArgs))
      case InterfaceNameI(template, templateArgs) => Some((template, templateArgs))
    }
  }
}

*/
// mig: struct StructNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct StructNameI<'s, 'i, R> {
    pub template: IStructTemplateNameI<'s, 'i, R>,
    pub template_args: &'i[ITemplataI<'s, 'i, R>],
}

/*
case class StructNameI[+R <: IRegionsModeI](
  template: IStructTemplateNameI[R],
  templateArgs: Vector[ITemplataI[R]]
) extends IStructNameI[R] with CitizenNameI[R] {
  vpass()
}

*/
// mig: struct InterfaceNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct InterfaceNameI<'s, 'i, R> {
    pub template: IInterfaceTemplateNameI<'s, 'i, R>,
    pub template_args: &'i[ITemplataI<'s, 'i, R>],
}

/*
case class InterfaceNameI[+R <: IRegionsModeI](
  template: IInterfaceTemplateNameI[R],
  templateArgs: Vector[ITemplataI[R]]
) extends IInterfaceNameI[R] with CitizenNameI[R] {
  vpass()
}

*/
// Per @LAGTNGZ, closure struct isn't parameterized; one struct corresponds to many LambdaCallFunctionNameIs.
// mig: struct LambdaCitizenTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct LambdaCitizenTemplateNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub code_location: CodeLocationS<'s>,
}

/*
// Per @LAGTNGZ, closure struct isn't parameterized; one struct corresponds to many LambdaCallFunctionNameIs.
case class LambdaCitizenTemplateNameI[+R <: IRegionsModeI](
  codeLocation: CodeLocationS
) extends IStructTemplateNameI[R] {
//  override def makeStructName(templateArgs: Vector[ITemplataI[R]]): IStructNameI = {
//    vassert(templateArgs.isEmpty)
//    interner.intern(LambdaCitizenNameI(this))
//  }
}

*/
// mig: struct LambdaCitizenNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct LambdaCitizenNameI<'s, 'i, R> {
    pub template: LambdaCitizenTemplateNameI<'s, 'i, R>,
}

/*
case class LambdaCitizenNameI[+R <: IRegionsModeI](
  template: LambdaCitizenTemplateNameI[R]
) extends IStructNameI[R] {
  def templateArgs: Vector[ITemplataI[R]] = Vector.empty
  vpass()
}

*/
// mig: enum CitizenTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// (was cfg-gated)
pub enum CitizenTemplateNameI<'s, 'i, R> {
    _Phantom(std::marker::PhantomData<(&'s (), &'i (), R)>),
}
// mig: impl CitizenTemplateNameI
/*
sealed trait CitizenTemplateNameI[+R <: IRegionsModeI] extends ICitizenTemplateNameI[R] {
  def humanName: StrI
  // We don't include a CodeLocation here because:
  // - There's no struct overloading, so there should only ever be one, we don't have to disambiguate
  //   with code locations
  // - It makes it easier to determine the CitizenTemplateNameI from a CitizenNameI which doesn't
  //   remember its code location.
  //codeLocation: CodeLocationS

//  override def makeCitizenName(templateArgs: Vector[ITemplata[ITemplataType]]): ICitizenNameI = {
//    interner.intern(CitizenNameI(this, templateArgs))
//  }
}

*/
// mig: struct StructTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct StructTemplateNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub human_name: StrI<'s>,
}
/*
case class StructTemplateNameI[+R <: IRegionsModeI](
  humanName: StrI,
  // We don't include a CodeLocation here because:
  // - There's no struct overloading, so there should only ever be one, we don't have to disambiguate
  //   with code locations
  // - It makes it easier to determine the StructTemplateNameI from a StructNameI which doesn't
  //   remember its code location.
  //   (note from later: not sure this is true anymore, since StructNameI contains a StructTemplateNameI)
  //codeLocation: CodeLocationS
) extends IStructTemplateNameI[R] with CitizenTemplateNameI[R] {
  vpass()

//  override def makeStructName(templateArgs: Vector[ITemplataI[R]]): IStructNameI = {
//    interner.intern(StructNameI(this, templateArgs))
//  }
}
*/
// mig: struct InterfaceTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct InterfaceTemplateNameI<'s, 'i, R> {
    pub _marker: std::marker::PhantomData<(&'i (), R)>,
    pub human_namee: StrI<'s>,
}
/*
case class InterfaceTemplateNameI[+R <: IRegionsModeI](
  humanNamee: StrI,
  // We don't include a CodeLocation here because:
  // - There's no struct overloading, so there should only ever be one, we don't have to disambiguate
  //   with code locations
  // - It makes it easier to determine the InterfaceTemplateNameI from a InterfaceNameI which doesn't
  //   remember its code location.
  //codeLocation: CodeLocationS
) extends IInterfaceTemplateNameI[R] with CitizenTemplateNameI[R] with ICitizenTemplateNameI[R] {
*/
// mig: fn human_name
// (was cfg-gated)
impl<'s, 'i, R> InterfaceTemplateNameI<'s, 'i, R> {
    pub fn human_name(&self) -> StrI<'s> { panic!("Unimplemented: human_name"); }
}
/*
  override def humanName = humanNamee
//  override def makeInterfaceName(templateArgs: Vector[ITemplataI[R]]): IInterfaceNameI = {
//    interner.intern(InterfaceNameI(this, templateArgs))
//  }
//  override def makeCitizenName(templateArgs: Vector[ITemplataI[R]]): ICitizenNameI = {
//    makeInterfaceName(templateArgs)
//  }
}

*/
// mig: struct AnonymousSubstructImplTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct AnonymousSubstructImplTemplateNameI<'s, 'i, R> {
    pub interface: IInterfaceTemplateNameI<'s, 'i, R>,
}
/*
case class AnonymousSubstructImplTemplateNameI[+R <: IRegionsModeI](
  interface: IInterfaceTemplateNameI[R]
) extends IImplTemplateNameI[R] {
//  override def makeImplName(templateArgs: Vector[ITemplataI[R]], subCitizen: ICitizenIT): IImplNameI = {
//    AnonymousSubstructImplNameI(this, templateArgs, subCitizen)
//  }
}
*/
// mig: struct AnonymousSubstructImplNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct AnonymousSubstructImplNameI<'s, 'i, R> {
    pub template: AnonymousSubstructImplTemplateNameI<'s, 'i, R>,
    pub template_args: &'i[ITemplataI<'s, 'i, R>],
    pub sub_citizen: ICitizenIT<'s, 'i, R>,
}

/*
case class AnonymousSubstructImplNameI[+R <: IRegionsModeI](
  template: AnonymousSubstructImplTemplateNameI[R],
  templateArgs: Vector[ITemplataI[R]],
  subCitizen: ICitizenIT[R]
) extends IImplNameI[R]


*/
// mig: struct AnonymousSubstructTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct AnonymousSubstructTemplateNameI<'s, 'i, R> {
    pub interface: IInterfaceTemplateNameI<'s, 'i, R>,
}
/*
case class AnonymousSubstructTemplateNameI[+R <: IRegionsModeI](
  // This happens to be the same thing that appears before this AnonymousSubstructNameI in a FullNameT.
  // This is really only here to help us calculate the imprecise name for this thing.
  interface: IInterfaceTemplateNameI[R]
) extends IStructTemplateNameI[R] {
//  override def makeStructName(templateArgs: Vector[ITemplataI[R]]): IStructNameI = {
//    interner.intern(AnonymousSubstructNameI(this, templateArgs))
//  }
}
*/
// mig: struct AnonymousSubstructConstructorTemplateNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct AnonymousSubstructConstructorTemplateNameI<'s, 'i, R> {
    pub substruct: ICitizenTemplateNameI<'s, 'i, R>,
}

/*
case class AnonymousSubstructConstructorTemplateNameI[+R <: IRegionsModeI](
  substruct: ICitizenTemplateNameI[R]
) extends IFunctionTemplateNameI[R] {
//  override def makeFunctionName(keywords: Keywords, templateArgs: Vector[ITemplataI[R]], params: Vector[CoordI]): IFunctionNameI = {
//    interner.intern(AnonymousSubstructConstructorNameI(this, templateArgs, params))
//  }
}

*/
// mig: struct AnonymousSubstructConstructorNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct AnonymousSubstructConstructorNameI<'s, 'i, R> {
    pub template: AnonymousSubstructConstructorTemplateNameI<'s, 'i, R>,
    pub template_args: &'i[ITemplataI<'s, 'i, R>],
    pub parameters: &'i[CoordI<'s, 'i, R>],
}

/*
case class AnonymousSubstructConstructorNameI[+R <: IRegionsModeI](
  template: AnonymousSubstructConstructorTemplateNameI[R],
  templateArgs: Vector[ITemplataI[R]],
  parameters: Vector[CoordI[R]]
) extends IFunctionNameI[R]

*/
// mig: struct AnonymousSubstructNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct AnonymousSubstructNameI<'s, 'i, R> {
    pub template: AnonymousSubstructTemplateNameI<'s, 'i, R>,
    pub template_args: &'i[ITemplataI<'s, 'i, R>],
}

/*
case class AnonymousSubstructNameI[+R <: IRegionsModeI](
  // This happens to be the same thing that appears before this AnonymousSubstructNameI in a FullNameT.
  // This is really only here to help us calculate the imprecise name for this thing.
  template: AnonymousSubstructTemplateNameI[R],
  templateArgs: Vector[ITemplataI[R]]
) extends IStructNameI[R] {

}
//case class AnonymousSubstructImplNameI[+R <: IRegionsModeI]() extends INameI[R] {
//
//}

*/
// mig: struct ResolvingEnvNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct ResolvingEnvNameI<'s, 'i, R>(pub std::marker::PhantomData<(&'s (), &'i (), R)>);

/*
case class ResolvingEnvNameI[+R <: IRegionsModeI]() extends INameI[R] {
  vpass()
}

*/
// mig: struct CallEnvNameI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
// (was cfg-gated)
pub struct CallEnvNameI<'s, 'i, R>(pub std::marker::PhantomData<(&'s (), &'i (), R)>);
/*
case class CallEnvNameI[+R <: IRegionsModeI]() extends INameI[R] {
  vpass()
}
*/
