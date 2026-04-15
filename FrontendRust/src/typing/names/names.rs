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
pub struct IdT;
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
pub enum INameT {}
/*
sealed trait INameT extends IInterning
*/
// mig: enum ITemplateNameT
pub enum ITemplateNameT {}
/*
sealed trait ITemplateNameT extends INameT
*/
// mig: enum IFunctionTemplateNameT
pub enum IFunctionTemplateNameT {}
/*
sealed trait IFunctionTemplateNameT extends ITemplateNameT {
  def makeFunctionName(interner: Interner, keywords: Keywords, templateArgs: Vector[ITemplataT[ITemplataType]], params: Vector[CoordT]): IFunctionNameT
}
*/
// mig: enum IInstantiationNameT
pub enum IInstantiationNameT {}
/*
sealed trait IInstantiationNameT extends INameT {
  def template: ITemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
// mig: enum IFunctionNameT
pub enum IFunctionNameT {}
/*
sealed trait IFunctionNameT extends IInstantiationNameT {
  def template: IFunctionTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
  def parameters: Vector[CoordT]
}
*/
// mig: enum ISuperKindTemplateNameT
pub enum ISuperKindTemplateNameT {}
/*
sealed trait ISuperKindTemplateNameT extends ITemplateNameT
*/
// mig: enum ISubKindTemplateNameT
pub enum ISubKindTemplateNameT {}
/*
sealed trait ISubKindTemplateNameT extends ITemplateNameT
*/
// mig: enum ICitizenTemplateNameT
pub enum ICitizenTemplateNameT {}
/*
sealed trait ICitizenTemplateNameT extends ISubKindTemplateNameT {
  def makeCitizenName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): ICitizenNameT
}
*/
// mig: enum IStructTemplateNameT
pub enum IStructTemplateNameT {}
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
pub enum IInterfaceTemplateNameT {}
/*
sealed trait IInterfaceTemplateNameT extends ICitizenTemplateNameT with ISuperKindTemplateNameT {
  def makeInterfaceName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]]): IInterfaceNameT
}
*/
// mig: enum ISuperKindNameT
pub enum ISuperKindNameT {}
/*
sealed trait ISuperKindNameT extends IInstantiationNameT {
  def template: ISuperKindTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
// mig: enum ISubKindNameT
pub enum ISubKindNameT {}
/*
sealed trait ISubKindNameT extends IInstantiationNameT {
  def template: ISubKindTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
// mig: enum ICitizenNameT
pub enum ICitizenNameT {}
/*
sealed trait ICitizenNameT extends ISubKindNameT {
  def template: ICitizenTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
// mig: enum IStructNameT
pub enum IStructNameT {}
/*
sealed trait IStructNameT extends ICitizenNameT with ISubKindNameT {
  override def template: IStructTemplateNameT
  override def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
// mig: enum IInterfaceNameT
pub enum IInterfaceNameT {}
/*
sealed trait IInterfaceNameT extends ICitizenNameT with ISubKindNameT with ISuperKindNameT {
  override def template: InterfaceTemplateNameT
  override def templateArgs: Vector[ITemplataT[ITemplataType]]
}
*/
// mig: enum IImplTemplateNameT
pub enum IImplTemplateNameT {}
/*
sealed trait IImplTemplateNameT extends ITemplateNameT {
  def makeImplName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]], subCitizen: ICitizenTT): IImplNameT
}
*/
// mig: enum IImplNameT
pub enum IImplNameT {}
/*
sealed trait IImplNameT extends IInstantiationNameT {
  def template: IImplTemplateNameT
}

*/
// mig: enum IRegionNameT
pub enum IRegionNameT {}
/*
sealed trait IRegionNameT extends INameT
*/
// mig: struct ExportTemplateNameT
pub struct ExportTemplateNameT;
/*
case class ExportTemplateNameT(codeLoc: CodeLocationS) extends ITemplateNameT
*/
// mig: struct ExportNameT
pub struct ExportNameT;
/*
case class ExportNameT(template: ExportTemplateNameT, region: RegionT) extends IInstantiationNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = Vector()
}

*/
// mig: struct ImplTemplateNameT
pub struct ImplTemplateNameT;
/*
case class ImplTemplateNameT(codeLocationS: CodeLocationS) extends IImplTemplateNameT {
  vpass()
  override def makeImplName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]], subCitizen: ICitizenTT): ImplNameT = {
    interner.intern(ImplNameT(this, templateArgs, subCitizen))
  }
}
*/
// mig: struct ImplNameT
pub struct ImplNameT;
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
pub struct ImplBoundTemplateNameT;
/*
case class ImplBoundTemplateNameT(codeLocationS: CodeLocationS) extends IImplTemplateNameT {
  override def makeImplName(interner: Interner, templateArgs: Vector[ITemplataT[ITemplataType]], subCitizen: ICitizenTT): ImplBoundNameT = {
    interner.intern(ImplBoundNameT(this, templateArgs))
  }
}
*/
// mig: struct ImplBoundNameT
pub struct ImplBoundNameT;
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
pub struct LetNameT;
/*
case class LetNameT(codeLocation: CodeLocationS) extends INameT
*/
// mig: struct ExportAsNameT
pub struct ExportAsNameT;
/*
case class ExportAsNameT(codeLocation: CodeLocationS) extends INameT
*/
// mig: struct RawArrayNameT
pub struct RawArrayNameT;
/*
case class RawArrayNameT(
  mutability: ITemplataT[MutabilityTemplataType],
  elementType: CoordT,
  selfRegion: RegionT
) extends INameT

// This num is really just here to disambiguate it from other reachable prototypes in the environment
*/
// mig: struct ReachablePrototypeNameT
pub struct ReachablePrototypeNameT;
/*
case class ReachablePrototypeNameT(num: Int) extends INameT
*/
// mig: struct StaticSizedArrayTemplateNameT
pub struct StaticSizedArrayTemplateNameT;
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
pub struct StaticSizedArrayNameT;
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
pub struct RuntimeSizedArrayTemplateNameT;
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
pub struct RuntimeSizedArrayNameT;
/*
case class RuntimeSizedArrayNameT(template: RuntimeSizedArrayTemplateNameT, arr: RawArrayNameT) extends ICitizenNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = {
    Vector(arr.mutability, CoordTemplataT(arr.elementType))
  }
}

*/
// mig: enum IPlaceholderNameT
pub enum IPlaceholderNameT {}
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
pub struct KindPlaceholderTemplateNameT;
/*
case class KindPlaceholderTemplateNameT(index: Int, rune: IRuneS) extends ISubKindTemplateNameT with ISuperKindTemplateNameT
*/
// mig: struct KindPlaceholderNameT
pub struct KindPlaceholderNameT;
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
pub struct NonKindNonRegionPlaceholderNameT;
/*
case class NonKindNonRegionPlaceholderNameT(index: Int, rune: IRuneS) extends IPlaceholderNameT

// See NNSPAFOC.
*/
// mig: struct OverrideDispatcherTemplateNameT
pub struct OverrideDispatcherTemplateNameT;
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
pub struct OverrideDispatcherNameT;
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
pub struct OverrideDispatcherCaseNameT;
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
pub enum IVarNameT {}
/*
sealed trait IVarNameT extends INameT
*/
// mig: struct TypingPassBlockResultVarNameT
pub struct TypingPassBlockResultVarNameT;
/*
case class TypingPassBlockResultVarNameT(life: LocationInFunctionEnvironmentT) extends IVarNameT
*/
// mig: struct TypingPassFunctionResultVarNameT
pub struct TypingPassFunctionResultVarNameT;
/*
case class TypingPassFunctionResultVarNameT() extends IVarNameT
*/
// mig: struct TypingPassTemporaryVarNameT
pub struct TypingPassTemporaryVarNameT;
/*
case class TypingPassTemporaryVarNameT(life: LocationInFunctionEnvironmentT) extends IVarNameT
*/
// mig: struct TypingPassPatternMemberNameT
pub struct TypingPassPatternMemberNameT;
/*
case class TypingPassPatternMemberNameT(life: LocationInFunctionEnvironmentT) extends IVarNameT
*/
// mig: struct TypingIgnoredParamNameT
pub struct TypingIgnoredParamNameT;
/*
case class TypingIgnoredParamNameT(num: Int) extends IVarNameT
*/
// mig: struct TypingPassPatternDestructureeNameT
pub struct TypingPassPatternDestructureeNameT;
/*
case class TypingPassPatternDestructureeNameT(life: LocationInFunctionEnvironmentT) extends IVarNameT
*/
// mig: struct UnnamedLocalNameT
pub struct UnnamedLocalNameT;
/*
case class UnnamedLocalNameT(codeLocation: CodeLocationS) extends IVarNameT
*/
// mig: struct ClosureParamNameT
pub struct ClosureParamNameT;
/*
case class ClosureParamNameT(codeLocation: CodeLocationS) extends IVarNameT
*/
// mig: struct ConstructingMemberNameT
pub struct ConstructingMemberNameT;
/*
case class ConstructingMemberNameT(name: StrI) extends IVarNameT
*/
// mig: struct WhileCondResultNameT
pub struct WhileCondResultNameT;
/*
case class WhileCondResultNameT(range: RangeS) extends IVarNameT
*/
// mig: struct IterableNameT
pub struct IterableNameT;
/*
case class IterableNameT(range: RangeS) extends IVarNameT {  }
*/
// mig: struct IteratorNameT
pub struct IteratorNameT;
/*
case class IteratorNameT(range: RangeS) extends IVarNameT {  }
*/
// mig: struct IterationOptionNameT
pub struct IterationOptionNameT;
/*
case class IterationOptionNameT(range: RangeS) extends IVarNameT {  }
*/
// mig: struct MagicParamNameT
pub struct MagicParamNameT;
/*
case class MagicParamNameT(codeLocation2: CodeLocationS) extends IVarNameT
*/
// mig: struct CodeVarNameT
pub struct CodeVarNameT;
/*
case class CodeVarNameT(name: StrI) extends IVarNameT
// We dont use CodeVarName2(0), CodeVarName2(1) etc because we dont want the user to address these members directly.
*/
// mig: struct AnonymousSubstructMemberNameT
pub struct AnonymousSubstructMemberNameT;
/*
case class AnonymousSubstructMemberNameT(index: Int) extends IVarNameT
*/
// mig: struct PrimitiveNameT
pub struct PrimitiveNameT;
/*
case class PrimitiveNameT(humanName: StrI) extends INameT
// Only made in typingpass
*/
// mig: struct PackageTopLevelNameT
pub struct PackageTopLevelNameT;
/*
case class PackageTopLevelNameT() extends INameT
*/
// mig: struct ProjectNameT
pub struct ProjectNameT;
/*
case class ProjectNameT(name: StrI) extends INameT
*/
// mig: struct PackageNameT
pub struct PackageNameT;
/*
case class PackageNameT(name: StrI) extends INameT
*/
// mig: struct RuneNameT
pub struct RuneNameT;
/*
case class RuneNameT(rune: IRuneS) extends INameT

// This is the name of a function that we're still figuring out in the function typingpass.
// We have its closured variables, but are still figuring out its template args and params.
*/
// mig: struct BuildingFunctionNameWithClosuredsT
pub struct BuildingFunctionNameWithClosuredsT;
/*
case class BuildingFunctionNameWithClosuredsT(
  templateName: IFunctionTemplateNameT,
) extends INameT {



}

*/
// mig: struct ExternTemplateNameT
pub struct ExternTemplateNameT;
/*
case class ExternTemplateNameT(
  codeLoc: CodeLocationS,
) extends ITemplateNameT
*/
// mig: struct ExternNameT
pub struct ExternNameT;
/*
case class ExternNameT(
  template: ExternTemplateNameT,
  templateArg: RegionT
) extends IInstantiationNameT {
  override def templateArgs: Vector[ITemplataT[ITemplataType]] = Vector()
}

*/
// mig: struct ExternFunctionNameT
pub struct ExternFunctionNameT;
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
pub struct FunctionNameT;
/*
case class FunctionNameT(
  template: FunctionTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT

*/
// mig: struct ForwarderFunctionNameT
pub struct ForwarderFunctionNameT;
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
pub struct FunctionBoundTemplateNameT;
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
pub struct FunctionBoundNameT;
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
pub struct PredictedFunctionTemplateNameT;
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
pub struct PredictedFunctionNameT;
/*
case class PredictedFunctionNameT(
    template: PredictedFunctionTemplateNameT,
    templateArgs: Vector[ITemplataT[ITemplataType]],
    parameters: Vector[CoordT]
) extends IFunctionNameT

*/
// mig: struct FunctionTemplateNameT
pub struct FunctionTemplateNameT;
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
pub struct LambdaCallFunctionTemplateNameT;
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
pub struct LambdaCallFunctionNameT;
/*
case class LambdaCallFunctionNameT(
  template: LambdaCallFunctionTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT

*/
// mig: struct ForwarderFunctionTemplateNameT
pub struct ForwarderFunctionTemplateNameT;
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
pub struct ConstructorTemplateNameT;
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
pub struct SelfNameT;
/*
case class SelfNameT() extends IVarNameT
*/
// mig: struct ArbitraryNameT
pub struct ArbitraryNameT;
/*
case class ArbitraryNameT() extends INameT
*/
// mig: enum CitizenNameT
pub enum CitizenNameT {}
/*
sealed trait CitizenNameT extends ICitizenNameT {
  def template: ICitizenTemplateNameT
  def templateArgs: Vector[ITemplataT[ITemplataType]]
}

*/
// mig: fn unapply
fn unapply() { panic!("Unmigrated unapply"); }
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
pub struct StructNameT;
/*
case class StructNameT(
  template: IStructTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]]
) extends IStructNameT with CitizenNameT {
  vpass()
}

*/
// mig: struct InterfaceNameT
pub struct InterfaceNameT;
/*
case class InterfaceNameT(
  template: InterfaceTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]]
) extends IInterfaceNameT with CitizenNameT {
  vpass()
}

*/
// mig: struct LambdaCitizenTemplateNameT
pub struct LambdaCitizenTemplateNameT;
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
pub struct LambdaCitizenNameT;
/*
case class LambdaCitizenNameT(
  template: LambdaCitizenTemplateNameT
) extends IStructNameT {
  def templateArgs: Vector[ITemplataT[ITemplataType]] = Vector.empty
  vpass()
}

*/
// mig: enum CitizenTemplateNameT
pub enum CitizenTemplateNameT {}
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
fn unapply() { panic!("Unmigrated unapply"); }
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
pub struct StructTemplateNameT;
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
pub struct InterfaceTemplateNameT;
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
pub struct AnonymousSubstructImplTemplateNameT;
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
pub struct AnonymousSubstructImplNameT;
/*
case class AnonymousSubstructImplNameT(
  template: AnonymousSubstructImplTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  subCitizen: ICitizenTT
) extends IImplNameT


*/
// mig: struct AnonymousSubstructTemplateNameT
pub struct AnonymousSubstructTemplateNameT;
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
pub struct AnonymousSubstructConstructorTemplateNameT;
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
pub struct AnonymousSubstructConstructorNameT;
/*
case class AnonymousSubstructConstructorNameT(
  template: AnonymousSubstructConstructorTemplateNameT,
  templateArgs: Vector[ITemplataT[ITemplataType]],
  parameters: Vector[CoordT]
) extends IFunctionNameT

*/
// mig: struct AnonymousSubstructNameT
pub struct AnonymousSubstructNameT;
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
pub struct ResolvingEnvNameT;
/*
case class ResolvingEnvNameT() extends INameT {
  vpass()
}

*/
// mig: struct CallEnvNameT
pub struct CallEnvNameT;
/*
case class CallEnvNameT() extends INameT {
  vpass()
}
*/
