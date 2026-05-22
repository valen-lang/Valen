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
pub struct IdI<'s, 't, R, T>(std::marker::PhantomData<&'s &'t (R, T)>);
// TODO: populate fields when src/instantiating/ast/names.rs is fully migrated.
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
#[cfg(any())]
impl<'s, 't> IdI<'s, 't> {
    pub fn package_id(&self) -> IdI<'s, 't> { panic!("Unimplemented: package_id"); }
}
/*
  def packageId: IdI[R, PackageTopLevelNameI[R]] = {
    IdI(packageCoord, Vector(), PackageTopLevelNameI())
  }

*/
// mig: fn init_id
#[cfg(any())]
impl<'s, 't> IdI<'s, 't> {
    pub fn init_id(&self) -> IdI<'s, 't> { panic!("Unimplemented: init_id"); }
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
#[cfg(any())]
impl<'s, 't> IdI<'s, 't> {
    pub fn init_non_package_id(&self) -> Option<IdI<'s, 't>> { panic!("Unimplemented: init_non_package_id"); }
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
#[cfg(any())]
impl<'s, 't> IdI<'s, 't> {
    pub fn steps(&self) -> &'t [INameI<'s, 't>] { panic!("Unimplemented: steps"); }
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
#[cfg(any())]
pub fn add_step<'s, 't>(old: &IdI<'s, 't>, new_last: INameI<'s, 't>) -> IdI<'s, 't> { panic!("Unimplemented: add_step"); }
/*
  def addStep[R <: IRegionsModeI, I <: INameI[R], Y <: INameI[R]](old: IdI[R, I], newLast: Y): IdI[R, Y] = {
    IdI[R, Y](old.packageCoord, old.steps, newLast)
  }
}

*/
// mig: enum INameI
pub enum INameI<'s, 't, R> { _Phantom(std::marker::PhantomData<&'s &'t R>) }
// TODO: populate variants when src/instantiating/ast/names.rs is fully migrated.

// mig: impl INameI
/*
sealed trait INameI[+R <: IRegionsModeI]
*/
// mig: enum ITemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum ITemplateNameI<'s, 't> {
    // Variants TBD
}
// mig: impl ITemplateNameI
/*
sealed trait ITemplateNameI[+R <: IRegionsModeI] extends INameI[R]
*/
// mig: enum IFunctionTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum IFunctionTemplateNameI<'s, 't> {
    // Variants TBD
}
// mig: impl IFunctionTemplateNameI
/*
sealed trait IFunctionTemplateNameI[+R <: IRegionsModeI] extends ITemplateNameI[R] {
//  def makeFunctionName(keywords: Keywords, templateArgs: Vector[ITemplataI[R]], params: Vector[CoordI]): IFunctionNameI
}
*/
// mig: enum IInstantiationNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum IInstantiationNameI<'s, 't> {
    // Variants TBD
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum IFunctionNameI<'s, 't> {
    // Variants TBD
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum ISuperKindTemplateNameI<'s, 't> {
    // Variants TBD
}
// mig: impl ISuperKindTemplateNameI
/*
sealed trait ISuperKindTemplateNameI[+R <: IRegionsModeI] extends ITemplateNameI[R]
*/
// mig: enum ISubKindTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum ISubKindTemplateNameI<'s, 't> {
    // Variants TBD
}
// mig: impl ISubKindTemplateNameI
/*
sealed trait ISubKindTemplateNameI[+R <: IRegionsModeI] extends ITemplateNameI[R]
*/
// mig: enum ICitizenTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum ICitizenTemplateNameI<'s, 't> {
    // Variants TBD
}
// mig: impl ICitizenTemplateNameI
/*
sealed trait ICitizenTemplateNameI[+R <: IRegionsModeI] extends ISubKindTemplateNameI[R] {
//  def makeCitizenName(templateArgs: Vector[ITemplataI[R]]): ICitizenNameI
}
*/
// mig: enum IStructTemplateNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum IStructTemplateNameI<'s, 't> {
    // Variants TBD
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum IInterfaceTemplateNameI<'s, 't> {
    // Variants TBD
}
// mig: impl IInterfaceTemplateNameI
/*
sealed trait IInterfaceTemplateNameI[+R <: IRegionsModeI] extends ICitizenTemplateNameI[R] with ISuperKindTemplateNameI[R] {
//  def makeInterfaceName(templateArgs: Vector[ITemplataI[R]]): IInterfaceNameI
}
*/
// mig: enum ISuperKindNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum ISuperKindNameI<'s, 't> {
    // Variants TBD
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum ISubKindNameI<'s, 't> {
    // Variants TBD
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum ICitizenNameI<'s, 't> {
    // Variants TBD
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum IStructNameI<'s, 't> {
    // Variants TBD
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum IInterfaceNameI<'s, 't> {
    // Variants TBD
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum IImplTemplateNameI<'s, 't> {
    // Variants TBD
}
// mig: impl IImplTemplateNameI
/*
sealed trait IImplTemplateNameI[+R <: IRegionsModeI] extends ITemplateNameI[R] {
//  def makeImplName(templateArgs: Vector[ITemplataI[R]], subCitizen: ICitizenIT): IImplNameI
}
*/
// mig: enum IImplNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum IImplNameI<'s, 't> {
    // Variants TBD
}
// mig: impl IImplNameI
/*
sealed trait IImplNameI[+R <: IRegionsModeI] extends IInstantiationNameI[R] {
  def template: IImplTemplateNameI[R]
}

*/
// mig: enum IRegionNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum IRegionNameI<'s, 't> {
    // Variants TBD
}
// mig: impl IRegionNameI
/*
sealed trait IRegionNameI[+R <: IRegionsModeI] extends INameI[R]
*/
// mig: struct RegionNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct RegionNameI<'s, 't> {
    pub rune: IRuneS<'s>,
}
/*
case class RegionNameI[+R <: IRegionsModeI](rune: IRuneS) extends IRegionNameI[R]
*/
// mig: struct DenizenDefaultRegionNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct DenizenDefaultRegionNameI<'s, 't>;
/*
case class DenizenDefaultRegionNameI[+R <: IRegionsModeI]() extends IRegionNameI[R]
*/
// mig: struct ExportTemplateNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ExportTemplateNameI<'s, 't> {
    pub code_loc: CodeLocationS<'s>,
}
/*
case class ExportTemplateNameI[+R <: IRegionsModeI](codeLoc: CodeLocationS) extends ITemplateNameI[R]
*/
// mig: struct ExportNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ExportNameI<'s, 't> {
    pub template: ExportTemplateNameI<'s, 't>,
    pub region: RegionTemplataI<'s, 't>,
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ExternTemplateNameI<'s, 't> {
    pub code_loc: CodeLocationS<'s>,
}
/*
case class ExternTemplateNameI[+R <: IRegionsModeI](codeLoc: CodeLocationS) extends ITemplateNameI[R]
*/
// mig: struct ExternNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ExternNameI<'s, 't> {
    pub template: ExternTemplateNameI<'s, 't>,
    pub region: RegionTemplataI<'s, 't>,
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ImplTemplateNameI<'s, 't> {
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ImplNameI<'s, 't> {
    pub template: IImplTemplateNameI<'s, 't>,
    pub template_args: &'t [ITemplataI<'s, 't>],
    pub sub_citizen: ICitizenIT<'s, 't>,
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ImplBoundTemplateNameI<'s, 't> {
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ImplBoundNameI<'s, 't> {
    pub template: ImplBoundTemplateNameI<'s, 't>,
    pub template_args: &'t [ITemplataI<'s, 't>],
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct LetNameI<'s, 't> {
    pub code_location: CodeLocationS<'s>,
}
/*
case class LetNameI[+R <: IRegionsModeI](codeLocation: CodeLocationS) extends INameI[R]
*/
// mig: struct ExportAsNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ExportAsNameI<'s, 't> {
    pub code_location: CodeLocationS<'s>,
}
/*
case class ExportAsNameI[+R <: IRegionsModeI](codeLocation: CodeLocationS) extends INameI[R]
*/
// mig: struct RawArrayNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct RawArrayNameI<'s, 't> {
    pub mutability: MutabilityI,
    pub element_type: CoordTemplataI<'s, 't>,
    pub self_region: RegionTemplataI<'s, 't>,
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ReachablePrototypeNameI<'s, 't> {
    pub num: i32,
}
/*
case class ReachablePrototypeNameI[+R <: IRegionsModeI](num: Int) extends INameI[R]
*/
// mig: struct StaticSizedArrayTemplateNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct StaticSizedArrayTemplateNameI<'s, 't>;
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct StaticSizedArrayNameI<'s, 't> {
    pub template: StaticSizedArrayTemplateNameI<'s, 't>,
    pub size: i64,
    pub variability: VariabilityI,
    pub arr: RawArrayNameI<'s, 't>,
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
#[cfg(any())]
impl<'s, 't> StaticSizedArrayNameI<'s, 't> {
    pub fn template_args(&self) -> &'t [ITemplataI<'s, 't>] { panic!("Unimplemented: template_args"); }
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct RuntimeSizedArrayTemplateNameI<'s, 't>;
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct RuntimeSizedArrayNameI<'s, 't> {
    pub template: RuntimeSizedArrayTemplateNameI<'s, 't>,
    pub arr: RawArrayNameI<'s, 't>,
}
/*
case class RuntimeSizedArrayNameI[+R <: IRegionsModeI](
  template: RuntimeSizedArrayTemplateNameI[R],
  arr: RawArrayNameI[R]
) extends ICitizenNameI[R] {
*/
// mig: fn template_args
#[cfg(any())]
impl<'s, 't> RuntimeSizedArrayNameI<'s, 't> {
    pub fn template_args(&self) -> &'t [ITemplataI<'s, 't>] { panic!("Unimplemented: template_args"); }
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct OverrideDispatcherTemplateNameI<'s, 't> {
    pub impl_id: IdI<'s, 't>,
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct OverrideDispatcherNameI<'s, 't> {
    pub template: OverrideDispatcherTemplateNameI<'s, 't>,
    pub template_args: &'t [ITemplataI<'s, 't>],
    pub parameters: &'t [CoordI<'s, 't>],
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct OverrideDispatcherCaseNameI<'s, 't> {
    pub independent_impl_template_args: &'t [ITemplataI<'s, 't>],
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct CaseFunctionFromImplNameI<'s, 't> {
    pub template: CaseFunctionFromImplTemplateNameI<'s, 't>,
    pub template_args: &'t [ITemplataI<'s, 't>],
    pub parameters: &'t [CoordI<'s, 't>],
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct CaseFunctionFromImplTemplateNameI<'s, 't> {
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum IVarNameI<'s, 't> {
    // Variants TBD
}
// mig: impl IVarNameI
/*
sealed trait IVarNameI[+R <: IRegionsModeI] extends INameI[R]
*/
// mig: struct TypingPassBlockResultVarNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct TypingPassBlockResultVarNameI<'s, 't> {
    pub life: LocationInFunctionEnvironmentI<'s, 't>,
}
/*
case class TypingPassBlockResultVarNameI[+R <: IRegionsModeI](life: LocationInFunctionEnvironmentI) extends IVarNameI[R]
*/
// mig: struct TypingPassFunctionResultVarNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct TypingPassFunctionResultVarNameI<'s, 't>;
/*
case class TypingPassFunctionResultVarNameI[+R <: IRegionsModeI]() extends IVarNameI[R]
*/
// mig: struct TypingPassTemporaryVarNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct TypingPassTemporaryVarNameI<'s, 't> {
    pub life: LocationInFunctionEnvironmentI<'s, 't>,
}
/*
case class TypingPassTemporaryVarNameI[+R <: IRegionsModeI](life: LocationInFunctionEnvironmentI) extends IVarNameI[R]
*/
// mig: struct TypingPassPatternMemberNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct TypingPassPatternMemberNameI<'s, 't> {
    pub life: LocationInFunctionEnvironmentI<'s, 't>,
}
/*
case class TypingPassPatternMemberNameI[+R <: IRegionsModeI](life: LocationInFunctionEnvironmentI) extends IVarNameI[R]
*/
// mig: struct TypingIgnoredParamNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct TypingIgnoredParamNameI<'s, 't> {
    pub num: i32,
}
/*
case class TypingIgnoredParamNameI[+R <: IRegionsModeI](num: Int) extends IVarNameI[R]
*/
// mig: struct TypingPassPatternDestructureeNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct TypingPassPatternDestructureeNameI<'s, 't> {
    pub life: LocationInFunctionEnvironmentI<'s, 't>,
}
/*
case class TypingPassPatternDestructureeNameI[+R <: IRegionsModeI](life: LocationInFunctionEnvironmentI) extends IVarNameI[R]
*/
// mig: struct UnnamedLocalNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct UnnamedLocalNameI<'s, 't> {
    pub code_location: CodeLocationS<'s>,
}
/*
case class UnnamedLocalNameI[+R <: IRegionsModeI](codeLocation: CodeLocationS) extends IVarNameI[R]
*/
// mig: struct ClosureParamNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ClosureParamNameI<'s, 't> {
    pub code_location: CodeLocationS<'s>,
}
/*
case class ClosureParamNameI[+R <: IRegionsModeI](codeLocation: CodeLocationS) extends IVarNameI[R]
*/
// mig: struct ConstructingMemberNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ConstructingMemberNameI<'s, 't> {
    pub name: StrI<'s>,
}
/*
case class ConstructingMemberNameI[+R <: IRegionsModeI](name: StrI) extends IVarNameI[R]
*/
// mig: struct WhileCondResultNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct WhileCondResultNameI<'s, 't> {
    pub range: RangeS,
}
/*
case class WhileCondResultNameI[+R <: IRegionsModeI](range: RangeS) extends IVarNameI[R]
*/
// mig: struct IterableNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct IterableNameI<'s, 't> {
    pub range: RangeS,
}
/*
case class IterableNameI[+R <: IRegionsModeI](range: RangeS) extends IVarNameI[R] {  }
*/
// mig: struct IteratorNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct IteratorNameI<'s, 't> {
    pub range: RangeS,
}
/*
case class IteratorNameI[+R <: IRegionsModeI](range: RangeS) extends IVarNameI[R] {  }
*/
// mig: struct IterationOptionNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct IterationOptionNameI<'s, 't> {
    pub range: RangeS,
}
/*
case class IterationOptionNameI[+R <: IRegionsModeI](range: RangeS) extends IVarNameI[R] {  }
*/
// mig: struct MagicParamNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct MagicParamNameI<'s, 't> {
    pub code_location_2: CodeLocationS<'s>,
}
/*
case class MagicParamNameI[+R <: IRegionsModeI](codeLocation2: CodeLocationS) extends IVarNameI[R]
*/
// mig: struct CodeVarNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct CodeVarNameI<'s, 't> {
    pub name: StrI<'s>,
}
/*
case class CodeVarNameI[+R <: IRegionsModeI](name: StrI) extends IVarNameI[R]
// We dont use CodeVarName2(0), CodeVarName2(1) etc because we dont want the user to address these members directly.
*/
// mig: struct AnonymousSubstructMemberNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct AnonymousSubstructMemberNameI<'s, 't> {
    pub index: i32,
}
/*
case class AnonymousSubstructMemberNameI[+R <: IRegionsModeI](index: Int) extends IVarNameI[R]
*/
// mig: struct PrimitiveNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct PrimitiveNameI<'s, 't> {
    pub human_name: StrI<'s>,
}
/*
case class PrimitiveNameI[+R <: IRegionsModeI](humanName: StrI) extends INameI[R]
// Only made in typingpass
*/
// mig: struct PackageTopLevelNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct PackageTopLevelNameI<'s, 't>;
/*
case class PackageTopLevelNameI[+R <: IRegionsModeI]() extends INameI[R]
*/
// mig: struct ProjectNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ProjectNameI<'s, 't> {
    pub name: StrI<'s>,
}
/*
case class ProjectNameI[+R <: IRegionsModeI](name: StrI) extends INameI[R]
*/
// mig: struct PackageNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct PackageNameI<'s, 't> {
    pub name: StrI<'s>,
}
/*
case class PackageNameI[+R <: IRegionsModeI](name: StrI) extends INameI[R]
*/
// mig: struct RuneNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct RuneNameI<'s, 't> {
    pub rune: IRuneS<'s>,
}
/*
case class RuneNameI[+R <: IRegionsModeI](rune: IRuneS) extends INameI[R]

// This is the name of a function that we're still figuring out in the function typingpass.
// We have its closured variables, but are still figuring out its template args and params.
*/
// mig: struct BuildingFunctionNameWithClosuredsI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct BuildingFunctionNameWithClosuredsI<'s, 't> {
    pub template_name: IFunctionTemplateNameI<'s, 't>,
}

/*
case class BuildingFunctionNameWithClosuredsI[+R <: IRegionsModeI](
  templateName: IFunctionTemplateNameI[R],
) extends INameI[R] {



}

*/
// mig: struct ExternFunctionNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ExternFunctionNameI<'s, 't> {
    pub human_name: StrI<'s>,
    pub parameters: &'t [CoordI<'s, 't>],
}
/*
case class ExternFunctionNameI[+R <: IRegionsModeI](
  humanName: StrI,
  parameters: Vector[CoordI[R]]
) extends IFunctionNameI[R] with IFunctionTemplateNameI[R] {
  override def template: IFunctionTemplateNameI[R] = this

//  override def makeFunctionName(
//    interner: Interner,
//    keywords: Keywords,
//    templateArgs: Vector[ITemplataI[R]],
//    params: Vector[CoordI]):
//  IFunctionNameI = this

  override def templateArgs: Vector[ITemplataI[R]] = Vector.empty
}

*/
// mig: struct FunctionNameIX
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct FunctionNameIX<'s, 't> {
    pub template: FunctionTemplateNameI<'s, 't>,
    pub template_args: &'t [ITemplataI<'s, 't>],
    pub parameters: &'t [CoordI<'s, 't>],
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ForwarderFunctionNameI<'s, 't> {
    pub template: ForwarderFunctionTemplateNameI<'s, 't>,
    pub inner: IFunctionNameI<'s, 't>,
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct FunctionBoundTemplateNameI<'s, 't> {
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct FunctionBoundNameI<'s, 't> {
    pub template: FunctionBoundTemplateNameI<'s, 't>,
    pub template_args: &'t [ITemplataI<'s, 't>],
    pub parameters: &'t [CoordI<'s, 't>],
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ReachableFunctionTemplateNameI<'s, 't> {
    pub human_name: StrI<'s>,
}

/*
case class ReachableFunctionTemplateNameI[+R <: IRegionsModeI](
    humanName: StrI
) extends INameI[R] with IFunctionTemplateNameI[R]

*/
// mig: struct ReachableFunctionNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ReachableFunctionNameI<'s, 't> {
    pub template: ReachableFunctionTemplateNameI<'s, 't>,
    pub template_args: &'t [ITemplataI<'s, 't>],
    pub parameters: &'t [CoordI<'s, 't>],
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct FunctionTemplateNameI<'s, 't> {
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
// mig: struct LambdaCallFunctionTemplateNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct LambdaCallFunctionTemplateNameI<'s, 't> {
    pub code_location: CodeLocationS<'s>,
    pub param_types: &'t [CoordT<'s, 't>],
}

/*
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct LambdaCallFunctionNameI<'s, 't> {
    pub template: LambdaCallFunctionTemplateNameI<'s, 't>,
    pub template_args: &'t [ITemplataI<'s, 't>],
    pub parameters: &'t [CoordI<'s, 't>],
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ForwarderFunctionTemplateNameI<'s, 't> {
    pub inner: IFunctionTemplateNameI<'s, 't>,
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ConstructorTemplateNameI<'s, 't> {
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct SelfNameI<'s, 't>;
/*
case class SelfNameI[+R <: IRegionsModeI]() extends IVarNameI[R]
*/
// mig: struct ArbitraryNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ArbitraryNameI<'s, 't>;
/*
case class ArbitraryNameI[+R <: IRegionsModeI]() extends INameI[R]
*/
// mig: enum CitizenNameI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum CitizenNameI<'s, 't> {
    // Variants TBD
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct StructNameI<'s, 't> {
    pub template: IStructTemplateNameI<'s, 't>,
    pub template_args: &'t [ITemplataI<'s, 't>],
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct InterfaceNameI<'s, 't> {
    pub template: IInterfaceTemplateNameI<'s, 't>,
    pub template_args: &'t [ITemplataI<'s, 't>],
}

/*
case class InterfaceNameI[+R <: IRegionsModeI](
  template: IInterfaceTemplateNameI[R],
  templateArgs: Vector[ITemplataI[R]]
) extends IInterfaceNameI[R] with CitizenNameI[R] {
  vpass()
}

*/
// mig: struct LambdaCitizenTemplateNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct LambdaCitizenTemplateNameI<'s, 't> {
    pub code_location: CodeLocationS<'s>,
}

/*
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct LambdaCitizenNameI<'s, 't> {
    pub template: LambdaCitizenTemplateNameI<'s, 't>,
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
#[cfg(any())]
pub enum CitizenTemplateNameI<'s, 't> {
    // Variants TBD
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct StructTemplateNameI<'s, 't> {
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct InterfaceTemplateNameI<'s, 't> {
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
#[cfg(any())]
impl<'s, 't> InterfaceTemplateNameI<'s, 't> {
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct AnonymousSubstructImplTemplateNameI<'s, 't> {
    pub interface: IInterfaceTemplateNameI<'s, 't>,
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct AnonymousSubstructImplNameI<'s, 't> {
    pub template: AnonymousSubstructImplTemplateNameI<'s, 't>,
    pub template_args: &'t [ITemplataI<'s, 't>],
    pub sub_citizen: ICitizenIT<'s, 't>,
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct AnonymousSubstructTemplateNameI<'s, 't> {
    pub interface: IInterfaceTemplateNameI<'s, 't>,
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct AnonymousSubstructConstructorTemplateNameI<'s, 't> {
    pub substruct: ICitizenTemplateNameI<'s, 't>,
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct AnonymousSubstructConstructorNameI<'s, 't> {
    pub template: AnonymousSubstructConstructorTemplateNameI<'s, 't>,
    pub template_args: &'t [ITemplataI<'s, 't>],
    pub parameters: &'t [CoordI<'s, 't>],
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct AnonymousSubstructNameI<'s, 't> {
    pub template: AnonymousSubstructTemplateNameI<'s, 't>,
    pub template_args: &'t [ITemplataI<'s, 't>],
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
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct ResolvingEnvNameI<'s, 't>;

/*
case class ResolvingEnvNameI[+R <: IRegionsModeI]() extends INameI[R] {
  vpass()
}

*/
// mig: struct CallEnvNameI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
#[cfg(any())]
pub struct CallEnvNameI<'s, 't>;
/*
case class CallEnvNameI[+R <: IRegionsModeI]() extends INameI[R] {
  vpass()
}
*/
