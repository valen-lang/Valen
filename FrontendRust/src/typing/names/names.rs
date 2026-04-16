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
// mig: struct IdT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct IdT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: enum INameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum INameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait INameT extends IInterning
*/
// mig: enum ITemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum ITemplateNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait ITemplateNameT extends INameT
*/
// mig: enum IFunctionTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum IFunctionTemplateNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait IFunctionTemplateNameT extends ITemplateNameT {
  def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): IFunctionNameT
}
*/
// mig: enum IInstantiationNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum IInstantiationNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait IInstantiationNameT extends INameT {
  def template: ITemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
// mig: enum IFunctionNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum IFunctionNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait IFunctionNameT extends IInstantiationNameT {
  def template: IFunctionTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
  def parameters: Vector[CoordT]
}
*/
// mig: enum ISuperKindTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum ISuperKindTemplateNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait ISuperKindTemplateNameT extends ITemplateNameT
*/
// mig: enum ISubKindTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum ISubKindTemplateNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait ISubKindTemplateNameT extends ITemplateNameT
*/
// mig: enum ICitizenTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum ICitizenTemplateNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait ICitizenTemplateNameT extends ISubKindTemplateNameT {
  def makeCitizenName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): ICitizenNameT
}
*/
// mig: enum IStructTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum IStructTemplateNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait IStructTemplateNameT extends ICitizenTemplateNameT {
  def makeStructName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): IStructNameT
  override def makeCitizenName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]):
  ICitizenNameT = {
    makeStructName(interner, templateArgs)
  }
}
*/
// mig: enum IInterfaceTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum IInterfaceTemplateNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait IInterfaceTemplateNameT extends ICitizenTemplateNameT with ISuperKindTemplateNameT {
  def makeInterfaceName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): IInterfaceNameT
}
*/
// mig: enum ISuperKindNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum ISuperKindNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait ISuperKindNameT extends IInstantiationNameT {
  def template: ISuperKindTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
// mig: enum ISubKindNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum ISubKindNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait ISubKindNameT extends IInstantiationNameT {
  def template: ISubKindTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
// mig: enum ICitizenNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum ICitizenNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait ICitizenNameT extends ISubKindNameT {
  def template: ICitizenTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
// mig: enum IStructNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum IStructNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait IStructNameT extends ICitizenNameT with ISubKindNameT {
  override def template: IStructTemplateNameT
  override def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
// mig: enum IInterfaceNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum IInterfaceNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait IInterfaceNameT extends ICitizenNameT with ISubKindNameT with ISuperKindNameT {
  override def template: InterfaceTemplateNameT
  override def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
// mig: enum IImplTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum IImplTemplateNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait IImplTemplateNameT extends ITemplateNameT {
  def makeImplName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]], subCitizen: ICitizenTT): IImplNameT
}
*/
// mig: enum IImplNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum IImplNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait IImplNameT extends IInstantiationNameT {
  def template: IImplTemplateNameT
}

*/
// mig: enum IRegionNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum IRegionNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait IRegionNameT extends INameT
*/
// mig: struct ExportTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ExportTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class ExportTemplateNameT(codeLoc: CodeLocationS) extends ITemplateNameT
*/
// mig: struct ExportNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ExportNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class ExportNameT(template: ExportTemplateNameT, region: RegionT) extends IInstantiationNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = Vector()
}

*/
// mig: struct ImplTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ImplTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class ImplTemplateNameT(codeLocationS: CodeLocationS) extends IImplTemplateNameT {
  vpass()
  override def makeImplName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]], subCitizen: ICitizenTT): ImplNameT = {
    interner.intern(ImplNameT(this, templateArgs, subCitizen))
  }
}
*/
// mig: struct ImplNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ImplNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct ImplBoundTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ImplBoundTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class ImplBoundTemplateNameT(codeLocationS: CodeLocationS) extends IImplTemplateNameT {
  override def makeImplName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]], subCitizen: ICitizenTT): ImplBoundNameT = {
    interner.intern(ImplBoundNameT(this, templateArgs))
  }
}
*/
// mig: struct ImplBoundNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ImplBoundNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct LetNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct LetNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class LetNameT(codeLocation: CodeLocationS) extends INameT
*/
// mig: struct ExportAsNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ExportAsNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class ExportAsNameT(codeLocation: CodeLocationS) extends INameT
*/
// mig: struct RawArrayNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct RawArrayNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class RawArrayNameT(
  mutability: ITemplataT[MutabilityTemplataType],
  elementType: CoordT,
  selfRegion: RegionT
) extends INameT

// This num is really just here to disambiguate it from other reachable prototypes in the environment
*/
// mig: struct ReachablePrototypeNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ReachablePrototypeNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class ReachablePrototypeNameT(num: Int) extends INameT
*/
// mig: struct StaticSizedArrayTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct StaticSizedArrayTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct StaticSizedArrayNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct StaticSizedArrayNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct RuntimeSizedArrayTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct RuntimeSizedArrayTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct RuntimeSizedArrayNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct RuntimeSizedArrayNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class RuntimeSizedArrayNameT(template: RuntimeSizedArrayTemplateNameT, arr: RawArrayNameT) extends ICitizenNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = {
    Vector(arr.mutability, CoordTemplataT(arr.elementType))
  }
}

*/
// mig: enum IPlaceholderNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum IPlaceholderNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait IPlaceholderNameT extends INameT {
  def index: Int
  def rune: IRuneS
}

// This exists because PlaceholderT is a kind, and all kinds need environments to assist
// in call/overload resolution. Environments are associated with templates, so it makes
// some sense to have a "placeholder template" notion.
*/
// mig: struct KindPlaceholderTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct KindPlaceholderTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class KindPlaceholderTemplateNameT(index: Int, rune: IRuneS) extends ISubKindTemplateNameT with ISuperKindTemplateNameT
*/
// mig: struct KindPlaceholderNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct KindPlaceholderNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class KindPlaceholderNameT(template: KindPlaceholderTemplateNameT) extends IPlaceholderNameT with ISubKindNameT with ISuperKindNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = Vector()
  override def rune: IRuneS = template.rune
  override def index: Int = template.index
}

// This exists because we need a different way to refer to a coord generic param's other components,
// see MNRFGC.
*/
// mig: struct NonKindNonRegionPlaceholderNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct NonKindNonRegionPlaceholderNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class NonKindNonRegionPlaceholderNameT(index: Int, rune: IRuneS) extends IPlaceholderNameT

// See NNSPAFOC.
*/
// mig: struct OverrideDispatcherTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct OverrideDispatcherTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct OverrideDispatcherNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct OverrideDispatcherNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct OverrideDispatcherCaseNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct OverrideDispatcherCaseNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: enum IVarNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum IVarNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait IVarNameT extends INameT
*/
// mig: struct TypingPassBlockResultVarNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct TypingPassBlockResultVarNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class TypingPassBlockResultVarNameT(life: LocationInFunctionEnvironmentT) extends IVarNameT
*/
// mig: struct TypingPassFunctionResultVarNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct TypingPassFunctionResultVarNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class TypingPassFunctionResultVarNameT() extends IVarNameT
*/
// mig: struct TypingPassTemporaryVarNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct TypingPassTemporaryVarNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class TypingPassTemporaryVarNameT(life: LocationInFunctionEnvironmentT) extends IVarNameT
*/
// mig: struct TypingPassPatternMemberNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct TypingPassPatternMemberNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class TypingPassPatternMemberNameT(life: LocationInFunctionEnvironmentT) extends IVarNameT
*/
// mig: struct TypingIgnoredParamNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct TypingIgnoredParamNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class TypingIgnoredParamNameT(num: Int) extends IVarNameT
*/
// mig: struct TypingPassPatternDestructureeNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct TypingPassPatternDestructureeNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class TypingPassPatternDestructureeNameT(life: LocationInFunctionEnvironmentT) extends IVarNameT
*/
// mig: struct UnnamedLocalNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct UnnamedLocalNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class UnnamedLocalNameT(codeLocation: CodeLocationS) extends IVarNameT
*/
// mig: struct ClosureParamNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ClosureParamNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class ClosureParamNameT(codeLocation: CodeLocationS) extends IVarNameT
*/
// mig: struct ConstructingMemberNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ConstructingMemberNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class ConstructingMemberNameT(name: StrI) extends IVarNameT
*/
// mig: struct WhileCondResultNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct WhileCondResultNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class WhileCondResultNameT(range: RangeS) extends IVarNameT
*/
// mig: struct IterableNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct IterableNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class IterableNameT(range: RangeS) extends IVarNameT {  }
*/
// mig: struct IteratorNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct IteratorNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class IteratorNameT(range: RangeS) extends IVarNameT {  }
*/
// mig: struct IterationOptionNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct IterationOptionNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class IterationOptionNameT(range: RangeS) extends IVarNameT {  }
*/
// mig: struct MagicParamNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct MagicParamNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class MagicParamNameT(codeLocation2: CodeLocationS) extends IVarNameT
*/
// mig: struct CodeVarNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct CodeVarNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class CodeVarNameT(name: StrI) extends IVarNameT
// We dont use CodeVarName2(0), CodeVarName2(1) etc because we dont want the user to address these members directly.
*/
// mig: struct AnonymousSubstructMemberNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct AnonymousSubstructMemberNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class AnonymousSubstructMemberNameT(index: Int) extends IVarNameT
*/
// mig: struct PrimitiveNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct PrimitiveNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class PrimitiveNameT(humanName: StrI) extends INameT
// Only made in typingpass
*/
// mig: struct PackageTopLevelNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct PackageTopLevelNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class PackageTopLevelNameT() extends INameT
*/
// mig: struct ProjectNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ProjectNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class ProjectNameT(name: StrI) extends INameT
*/
// mig: struct PackageNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct PackageNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class PackageNameT(name: StrI) extends INameT
*/
// mig: struct RuneNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct RuneNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class RuneNameT(rune: IRuneS) extends INameT

// This is the name of a function that we're still figuring out in the function typingpass.
// We have its closured variables, but are still figuring out its template args and params.
*/
// mig: struct BuildingFunctionNameWithClosuredsT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct BuildingFunctionNameWithClosuredsT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class BuildingFunctionNameWithClosuredsT(
  templateName: IFunctionTemplateNameT,
) extends INameT {



}

*/
// mig: struct ExternTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ExternTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class ExternTemplateNameT(
  codeLoc: CodeLocationS,
) extends ITemplateNameT
*/
// mig: struct ExternNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ExternNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class ExternNameT(
  template: ExternTemplateNameT,
  templateArg: RegionT
) extends IInstantiationNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = Vector()
}

*/
// mig: struct ExternFunctionNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ExternFunctionNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct FunctionNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct FunctionNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class FunctionNameT(
  template: FunctionTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT

*/
// mig: struct ForwarderFunctionNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ForwarderFunctionNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class ForwarderFunctionNameT(
  template: ForwarderFunctionTemplateNameT,
  inner: IFunctionNameT
) extends IFunctionNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = inner.templateArgs
  override def parameters: Vector[CoordT] = inner.parameters
}

*/
// mig: struct FunctionBoundTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct FunctionBoundTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct FunctionBoundNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct FunctionBoundNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct PredictedFunctionTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct PredictedFunctionTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct PredictedFunctionNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct PredictedFunctionNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class PredictedFunctionNameT(
    template: PredictedFunctionTemplateNameT,
    templateArgs: Vector[ITemplataT[ITemplataType]],
    parameters: Vector[CoordT]
) extends IFunctionNameT

*/
// mig: struct FunctionTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct FunctionTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct LambdaCallFunctionTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct LambdaCallFunctionTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct LambdaCallFunctionNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct LambdaCallFunctionNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class LambdaCallFunctionNameT(
  template: LambdaCallFunctionTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT

*/
// mig: struct ForwarderFunctionTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ForwarderFunctionTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct ConstructorTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ConstructorTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct SelfNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct SelfNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class SelfNameT() extends IVarNameT
*/
// mig: struct ArbitraryNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ArbitraryNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class ArbitraryNameT() extends INameT
*/
// mig: enum CitizenNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum CitizenNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait CitizenNameT extends ICitizenNameT {
  def template: ICitizenTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
}

*/
// mig: fn unapply
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
// mig: struct StructNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct StructNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class StructNameT(
  template: IStructTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]]
) extends IStructNameT with CitizenNameT {
  vpass()
}

*/
// mig: struct InterfaceNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct InterfaceNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class InterfaceNameT(
  template: InterfaceTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]]
) extends IInterfaceNameT with CitizenNameT {
  vpass()
}

*/
// mig: struct LambdaCitizenTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct LambdaCitizenTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct LambdaCitizenNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct LambdaCitizenNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class LambdaCitizenNameT(
  template: LambdaCitizenTemplateNameT
) extends IStructNameT {
  def templateArgs: Vector[ITemplataT[ITemplataType]] = Vector.empty
  vpass()
}

*/
// mig: enum CitizenTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum CitizenTemplateNameT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
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
// mig: fn unapply
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
// mig: struct StructTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct StructTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct InterfaceTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct InterfaceTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct AnonymousSubstructImplTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct AnonymousSubstructImplTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class AnonymousSubstructImplTemplateNameT(
  interface: IInterfaceTemplateNameT
) extends IImplTemplateNameT {
  override def makeImplName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]], subCitizen: ICitizenTT): IImplNameT = {
    interner.intern(AnonymousSubstructImplNameT(this, templateArgs, subCitizen))
  }
}
*/
// mig: struct AnonymousSubstructImplNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct AnonymousSubstructImplNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class AnonymousSubstructImplNameT(
  template: AnonymousSubstructImplTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  subCitizen: ICitizenTT
) extends IImplNameT


*/
// mig: struct AnonymousSubstructTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct AnonymousSubstructTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct AnonymousSubstructConstructorTemplateNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct AnonymousSubstructConstructorTemplateNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class AnonymousSubstructConstructorTemplateNameT(
  substruct: ICitizenTemplateNameT
) extends IFunctionTemplateNameT {
  override def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): IFunctionNameT = {
    interner.intern(AnonymousSubstructConstructorNameT(this, templateArgs, params))
  }
}

*/
// mig: struct AnonymousSubstructConstructorNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct AnonymousSubstructConstructorNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class AnonymousSubstructConstructorNameT(
  template: AnonymousSubstructConstructorTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT

*/
// mig: struct AnonymousSubstructNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct AnonymousSubstructNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
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
// mig: struct ResolvingEnvNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct ResolvingEnvNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class ResolvingEnvNameT() extends INameT {
  vpass()
}

*/
// mig: struct CallEnvNameT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub struct CallEnvNameT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
case class CallEnvNameT() extends INameT {
  vpass()
}
*/
