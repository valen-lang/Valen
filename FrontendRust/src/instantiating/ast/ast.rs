/*
package dev.vale.instantiating.ast

import dev.vale._
import dev.vale.postparsing._

import scala.collection.immutable._

// We won't always have a return type for a banner... it might have not specified its return
// type, so we're currently evaluating the entire body for it right now.
// If we ever find ourselves wanting the return type for a banner, we need to:
// - Check if it's in the returnTypesByBanner map. If so, good.
// - If not, then check if the banner is in declaredBanners. If so, then we're currently in
//   the process of evaluating the entire body. In this case, throw an error because we're
//   about to infinite loop. Hopefully this is a user error, they need to specify a return
//   type to avoid a cyclical definition.
// - If not in declared banners, then tell FunctionCompiler to start evaluating it.
*/
// mig: struct KindExportI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct KindExportI<'s, 't> {
    pub range: RangeS<'s>,
    pub tyype: KindIT<'t>,
    pub id: IdI<'t, ExportNameI<'t>>,
    pub exported_name: StrI<'s>,
}
// mig: impl KindExportI
/*
case class KindExportI(
  range: RangeS,
  tyype: KindIT[cI],
  // Good for knowing the package of this export for later prefixing the exportedName, also good
  // for getting its region.
  id: IdI[cI, ExportNameI[cI]],
  exportedName: StrI
)  {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for KindExportI` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for KindExportI` below.)
/*
override def hashCode(): Int = vcurious()

}
*/
// mig: struct FunctionExportI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct FunctionExportI<'s, 't> {
    pub range: RangeS<'s>,
    pub prototype: PrototypeI<'s, 't>,
    pub export_id: IdI<'t, ExportNameI<'t>>,
    pub exported_name: StrI<'s>,
}
// mig: impl FunctionExportI
/*
case class FunctionExportI(
  range: RangeS,
  prototype: PrototypeI[cI],
  exportId: IdI[cI, ExportNameI[cI]],
  exportedName: StrI
)  {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for FunctionExportI` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for FunctionExportI` below.)
/*
override def hashCode(): Int = vcurious()
  vpass()
}

//case class KindExternI(
//  tyype: KindIT,
//  packageCoordinate: PackageCoordinate,
//  externName: StrI
//)  {
//  override def equals(obj: Any): Boolean = vcurious();
//override def hashCode(): Int = vcurious()
//
//}
*/
// mig: struct FunctionExternI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct FunctionExternI<'s, 't> {
    pub prototype: PrototypeI<'s, 't>,
    pub extern_name: StrI<'s>,
}
// mig: impl FunctionExternI
/*
case class FunctionExternI(
//  range: RangeS,
  prototype: PrototypeI[cI],
//  packageCoordinate: PackageCoordinate,
  externName: StrI
)  {
  vpass()
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for FunctionExternI` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for FunctionExternI` below.)
/*
override def hashCode(): Int = vcurious()

}
*/
// mig: struct InterfaceEdgeBlueprintI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct InterfaceEdgeBlueprintI<'s, 't> {
    pub interface: IdI<'t, IInterfaceNameI<'t>>,
    pub super_family_root_headers: &'t [(PrototypeI<'s, 't>, i32)],
}
// mig: impl InterfaceEdgeBlueprintI
/*
case class InterfaceEdgeBlueprintI(
  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  interface: IdI[cI, IInterfaceNameI[cI]],
  superFamilyRootHeaders: Vector[(PrototypeI[cI], Int)]) {
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for InterfaceEdgeBlueprintI` below.)
/*
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for InterfaceEdgeBlueprintI` below.)
/*
override def equals(obj: Any): Boolean = vcurious(); }
*/
// mig: struct EdgeI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct EdgeI<'s, 't> {
    pub edge_id: IdI<'t, IImplNameI<'t>>,
    pub sub_citizen: ICitizenIT<'t>,
    pub super_interface: IdI<'t, IInterfaceNameI<'t>>,
    pub rune_to_func_bound: ArenaIndexMap<'t, IRuneS<'s>, IdI<'t, FunctionBoundNameI<'t>>>,
    pub rune_to_impl_bound: ArenaIndexMap<'t, IRuneS<'s>, IdI<'t, ImplBoundNameI<'t>>>,
    pub abstract_func_to_override_func: ArenaIndexMap<'t, IdI<'t, IFunctionNameI<'t>>, PrototypeI<'s, 't>>,
}
// mig: impl EdgeI
/*
case class EdgeI(
  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  edgeId: IdI[cI, IImplNameI[cI]],
  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  subCitizen: ICitizenIT[cI],
  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  superInterface: IdI[cI, IInterfaceNameI[cI]],
  // This is similar to FunctionT.runeToFuncBound
  runeToFuncBound: Map[IRuneS, IdI[cI, FunctionBoundNameI[cI]]],
  runeToImplBound: Map[IRuneS, IdI[cI, ImplBoundNameI[cI]]],
  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  abstractFuncToOverrideFunc: Map[IdI[cI, IFunctionNameI[cI]], PrototypeI[cI]]
) {
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for EdgeI` below.)
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for EdgeI` below.)
/*
  override def equals(obj: Any): Boolean = {
    obj match {
      case EdgeI(thatEdgeId, thatStruct, thatInterface, _, _, _) => {
        val isSame = subCitizen == thatStruct && superInterface == thatInterface
        if (isSame) {
          vassert(edgeId == thatEdgeId)
        }
        isSame
      }
    }
  }
}
*/
// mig: struct FunctionDefinitionI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct FunctionDefinitionI<'s, 't> {
    pub header: FunctionHeaderI<'s, 't>,
    pub rune_to_func_bound: ArenaIndexMap<'t, IRuneS<'s>, IdI<'t, FunctionBoundNameI<'t>>>,
    pub rune_to_impl_bound: ArenaIndexMap<'t, IRuneS<'s>, IdI<'t, ImplBoundNameI<'t>>>,
    pub body: ReferenceExpressionIE<'s, 't>,
}
// mig: impl FunctionDefinitionI
/*
case class FunctionDefinitionI(
  header: FunctionHeaderI,
  runeToFuncBound: Map[IRuneS, IdI[cI, FunctionBoundNameI[cI]]],
  runeToImplBound: Map[IRuneS, IdI[cI, ImplBoundNameI[cI]]],
  body: ReferenceExpressionIE)  {
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for FunctionDefinitionI` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for FunctionDefinitionI` below.)
/*
override def hashCode(): Int = vcurious()

  // We always end a function with a ret, whose result is a Never.
  vassert(body.result.kind == NeverIT[cI](false))
*/
// mig: fn is_pure
impl<'s, 't> FunctionDefinitionI<'s, 't> {
    pub fn is_pure(&self) -> bool {
        panic!("Unimplemented: is_pure")
    }
}
/*
  def isPure: Boolean = header.isPure
}

object getFunctionLastName {
*/
// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<FunctionDefinitionI> for IFunctionNameI` or inline match.)
/*
  def unapply(f: FunctionDefinitionI): Option[IFunctionNameI[cI]] = Some(f.header.id.localName)
}
*/
// mig: struct LocationInFunctionEnvironmentI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct LocationInFunctionEnvironmentI<'t> {
    pub path: &'t [i32],
}
// mig: impl LocationInFunctionEnvironmentI
/*
// A unique location in a function. Environment is in the name so it spells LIFE!
case class LocationInFunctionEnvironmentI(path: Vector[Int]) {
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for LocationInFunctionEnvironmentI` below.)
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
// mig: fn add
impl<'t> LocationInFunctionEnvironmentI<'t> {
    pub fn add(&self, sub_location: i32) -> LocationInFunctionEnvironmentI<'t> {
        panic!("Unimplemented: add")
    }
}
/*
  def +(subLocation: Int): LocationInFunctionEnvironmentI = {
    LocationInFunctionEnvironmentI(path :+ subLocation)
  }
*/
// mig: fn to_string
impl<'t> LocationInFunctionEnvironmentI<'t> {
    pub fn to_string(&self) -> String {
        panic!("Unimplemented: to_string")
    }
}
/*
  override def toString: String = path.mkString(".")
}
*/
// mig: struct AbstractI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct AbstractI;
// mig: impl AbstractI
/*
case class AbstractI()
*/
// mig: struct ParameterI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ParameterI<'s, 't> {
    pub name: IVarNameI<'t>,
    pub virtuality: Option<AbstractI>,
    pub pre_checked: bool,
    pub tyype: CoordI<'t>,
}
// mig: impl ParameterI
/*
case class ParameterI(
  name: IVarNameI[cI],
  virtuality: Option[AbstractI],
  preChecked: Boolean,
  tyype: CoordI[cI]) {
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ParameterI` below.)
/*
  val hash = runtime.ScalaRunTime._hashCode(this)

  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ParameterI` below.)
/*
  // Use same instead, see EHCFBD for why we dont like equals.
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn same
impl<'s, 't> ParameterI<'s, 't> {
    pub fn same(&self, that: &ParameterI<'_, '_>) -> bool {
        panic!("Unimplemented: same")
    }
}
/*
  def same(that: ParameterI): Boolean = {
    name == that.name &&
      virtuality == that.virtuality &&
      tyype == that.tyype
  }
}
*/
// mig: struct SignatureI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct SignatureI<'s, 't> {
    pub id: IdI<'t, IFunctionNameI<'t>>,
}
// mig: impl SignatureI
/*
// A "signature" is just the things required for overload resolution, IOW function name and arg types.

// An autograph could be a super signature; a signature plus attributes like virtual and mutable.
// If we ever need it, a "schema" could be something.

// A FunctionBanner2 is everything in a FunctionHeader2 minus the return type.
// These are only made by the FunctionCompiler, to signal that it's currently being
// evaluated or it's already been evaluated.
// It's easy to see all possible function banners, but not easy to see all possible
// function headers, because functions don't have to specify their return types and
// it takes a complete typingpass evaluate to deduce a function's return type.

case class SignatureI[+R <: IRegionsModeI](id: IdI[R, IFunctionNameI[R]]) {
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for SignatureI` below.)
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
// mig: fn param_types
impl<'s, 't> SignatureI<'s, 't> {
    pub fn param_types(&self) -> Vec<CoordI<'_>> {
        panic!("Unimplemented: param_types")
    }
}
/*
  def paramTypes: Vector[CoordI[R]] = id.localName.parameters
}
*/
// mig: enum IFunctionAttributeI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum IFunctionAttributeI<'s, 't> {
    // Placeholder variant
}
// mig: impl IFunctionAttributeI
/*
sealed trait IFunctionAttributeI
*/
// mig: enum ICitizenAttributeI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ICitizenAttributeI<'s, 't> {
    // Placeholder variant
}
// mig: impl ICitizenAttributeI
/*
sealed trait ICitizenAttributeI
*/
// mig: struct ExternI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ExternI {
    pub package_coord: PackageCoordinate,
}
// mig: impl ExternI
/*
case class ExternI(packageCoord: PackageCoordinate) extends IFunctionAttributeI with ICitizenAttributeI { // For optimization later
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ExternI` below.)
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
// There's no Export2 here, we use separate KindExport and FunctionExport constructs.
//case class Export2(packageCoord: PackageCoordinate) extends IFunctionAttribute2 with ICitizenAttribute2
case object PureI extends IFunctionAttributeI
case object SealedI extends ICitizenAttributeI
case object UserFunctionI extends IFunctionAttributeI // Whether it was written by a human. Mostly for tests right now.
*/
// mig: struct RegionI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct RegionI<'s, 't> {
    pub name: IRegionNameI<'t>,
    pub mutable: bool,
}
// mig: impl RegionI
/*
case class RegionI(
  name: IRegionNameI[cI],
  mutable: Boolean)
*/
// mig: struct FunctionHeaderI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct FunctionHeaderI<'s, 't> {
    pub id: IdI<'t, IFunctionNameI<'t>>,
    pub attributes: &'t [IFunctionAttributeI<'s, 't>],
    pub params: &'t [ParameterI<'s, 't>],
    pub return_type: CoordI<'t>,
}
// mig: impl FunctionHeaderI
/*
case class FunctionHeaderI(
  // This one little name field can illuminate much of how the compiler works, see UINIT.
  id: IdI[cI, IFunctionNameI[cI]],
  attributes: Vector[IFunctionAttributeI],
//  regions: Vector[cIegionI],
  params: Vector[ParameterI],
  returnType: CoordI[cI]) {
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for FunctionHeaderI` below.)
/*
  val hash = runtime.ScalaRunTime._hashCode(this)

  override def hashCode(): Int = hash;

//  val perspectiveRegion =
//    id.localName.templateArgs.last match {
//      case PlaceholderTemplata(IdI(packageCoord, initSteps, r @ RegionPlaceholderNameI(_, _, _, _, _)), RegionTemplataType()) => {
//        IdI(packageCoord, initSteps, r)
//      }
//      case _ => vwat()
//    }
//  if (attributes.contains(PureI)) {
//    // Instantiator relies on this assumption so that it knows when certain things are pure.
//    vassert(perspectiveRegion.localName.originalMaybeNearestPureLocation == Some(LocationInDenizen(Vector())))
//  }
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for FunctionHeaderI` below.)
/*
  override def equals(obj: Any): Boolean = {
    obj match {
      case FunctionHeaderI(thatName, _, _, _) => {
        id == thatName
      }
      case _ => false
    }
  }

  // Make sure there's no duplicate names
  vassert(params.map(_.name).toSet.size == params.size);

  vassert(id.localName.parameters == paramTypes)
*/
// mig: fn is_extern
impl<'s, 't> FunctionHeaderI<'s, 't> {
    pub fn is_extern(&self) -> bool {
        panic!("Unimplemented: is_extern")
    }
}
/*
  def isExtern = attributes.exists({ case ExternI(_) => true case _ => false })
  //  def isExport = attributes.exists({ case Export2(_) => true case _ => false })
*/
// mig: fn is_user_function
impl<'s, 't> FunctionHeaderI<'s, 't> {
    pub fn is_user_function(&self) -> bool {
        panic!("Unimplemented: is_user_function")
    }
}
/*
  def isUserFunction = attributes.contains(UserFunctionI)
//  def getAbstractInterface: Option[InterfaceIT] = toBanner.getAbstractInterface
////  def getOverride: Option[(StructIT, InterfaceIT)] = toBanner.getOverride
//  def getVirtualIndex: Option[Int] = toBanner.getVirtualIndex

//  def toSignature(interner: Interner, keywords: Keywords): SignatureI = {
//    val newLastStep = templateName.last.makeFunctionName(interner, keywords, templateArgs, params)
//    val fullName = FullNameI(templateName.packageCoord, name.initSteps, newLastStep)
//
//    SignatureI(fullName)
//
//  }
//  def paramTypes: Vector[CoordI[cI]] = params.map(_.tyype)
*/
// mig: fn get_abstract_interface
impl<'s, 't> FunctionHeaderI<'s, 't> {
    pub fn get_abstract_interface(&self) -> Option<InterfaceIT<'_>> {
        panic!("Unimplemented: get_abstract_interface")
    }
}
/*
  def getAbstractInterface: Option[InterfaceIT[cI]] = {
    val abstractInterfaces =
      params.collect({
        case ParameterI(_, Some(AbstractI()), _, CoordI(_, ir @ InterfaceIT(_))) => ir
      })
    vassert(abstractInterfaces.size <= 1)
    abstractInterfaces.headOption
  }
*/
// mig: fn get_virtual_index
impl<'s, 't> FunctionHeaderI<'s, 't> {
    pub fn get_virtual_index(&self) -> Option<i32> {
        panic!("Unimplemented: get_virtual_index")
    }
}
/*
  def getVirtualIndex: Option[Int] = {
    val indices =
      params.zipWithIndex.collect({
        case (ParameterI(_, Some(AbstractI()), _, _), index) => index
      })
    vassert(indices.size <= 1)
    indices.headOption
  }

//  maybeOriginFunction.foreach(originFunction => {
//    if (originFunction.genericParameters.size != fullName.last.templateArgs.size) {
//      vfail("wtf m8")
//    }
//  })
*/
// mig: fn to_prototype
impl<'s, 't> FunctionHeaderI<'s, 't> {
    pub fn to_prototype(&self) -> PrototypeI<'_, '_> {
        panic!("Unimplemented: to_prototype")
    }
}
/*
  def toPrototype: PrototypeI[cI] = {
//    val substituter = TemplataCompiler.getPlaceholderSubstituter(interner, fullName, templateArgs)
//    val paramTypes = params.map(_.tyype).map(substituter.substituteForCoord)
//    val newLastStep = fullName.last.makeFunctionName(interner, keywords, templateArgs, paramTypes)
//    val newName = FullNameI(fullName.packageCoord, fullName.initSteps, newLastStep)
    PrototypeI(id, returnType)
  }
*/
// mig: fn to_signature
impl<'s, 't> FunctionHeaderI<'s, 't> {
    pub fn to_signature(&self) -> SignatureI<'_, '_> {
        panic!("Unimplemented: to_signature")
    }
}
/*
  def toSignature: SignatureI[cI] = {
    toPrototype.toSignature
  }
*/
// mig: fn param_types
impl<'s, 't> FunctionHeaderI<'s, 't> {
    pub fn param_types(&self) -> Vec<CoordI<'_>> {
        panic!("Unimplemented: param_types")
    }
}
/*
  def paramTypes: Vector[CoordI[cI]] = id.localName.parameters
*/
// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<FunctionHeaderI> for (IdI, Vec<ParameterI>, CoordI)` or inline match.)
/*
  def unapply(arg: FunctionHeaderI): Option[(IdI[cI, IFunctionNameI[cI]], Vector[ParameterI], CoordI[cI])] = {
    Some(id, params, returnType)
  }
*/
// mig: fn is_pure
impl<'s, 't> FunctionHeaderI<'s, 't> {
    pub fn is_pure(&self) -> bool {
        panic!("Unimplemented: is_pure")
    }
}
/*
  def isPure: Boolean = {
    attributes.collectFirst({ case PureI => }).nonEmpty
  }
}
*/
// mig: struct PrototypeI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct PrototypeI<'s, 't> {
    pub id: IdI<'t, IFunctionNameI<'t>>,
    pub return_type: CoordI<'t>,
}
// mig: impl PrototypeI
/*
case class PrototypeI[+R <: IRegionsModeI](
    id: IdI[R, IFunctionNameI[R]],
    returnType: CoordI[R]) {
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for PrototypeI` below.)
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
// mig: fn param_types
impl<'s, 't> PrototypeI<'s, 't> {
    pub fn param_types(&self) -> Vec<CoordI<'_>> {
        panic!("Unimplemented: param_types")
    }
}
/*
  def paramTypes: Vector[CoordI[R]] = id.localName.parameters
*/
// mig: fn to_signature
impl<'s, 't> PrototypeI<'s, 't> {
    pub fn to_signature(&self) -> SignatureI<'_, '_> {
        panic!("Unimplemented: to_signature")
    }
}
/*
  def toSignature: SignatureI[R] = SignatureI[R](id)
}
*/
// mig: enum IVariableI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum IVariableI<'s, 't> {
    // Placeholder variant
}
// mig: impl IVariableI
/*
sealed trait IVariableI  {
*/
// mig: fn name
impl<'s, 't> IVariableI<'s, 't> {
    pub fn name(&self) -> IVarNameI<'_> {
        panic!("Unimplemented: name")
    }
}
/*
  def name: IVarNameI[cI]
*/
// mig: fn variability
impl<'s, 't> IVariableI<'s, 't> {
    pub fn variability(&self) -> VariabilityI {
        panic!("Unimplemented: variability")
    }
}
/*
  def variability: VariabilityI
*/
// mig: fn collapsed_coord
impl<'s, 't> IVariableI<'s, 't> {
    pub fn collapsed_coord(&self) -> CoordI<'_> {
        panic!("Unimplemented: collapsed_coord")
    }
}
/*
  def collapsedCoord: CoordI[cI]
}
*/
// mig: enum ILocalVariableI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ILocalVariableI<'s, 't> {
    // Placeholder variant
}
// mig: impl ILocalVariableI
/*
sealed trait ILocalVariableI extends IVariableI {
*/
// mig: fn name
impl<'s, 't> ILocalVariableI<'s, 't> {
    pub fn name(&self) -> IVarNameI<'_> {
        panic!("Unimplemented: name")
    }
}
/*
  def name: IVarNameI[cI]
*/
// mig: fn collapsed_coord
impl<'s, 't> ILocalVariableI<'s, 't> {
    pub fn collapsed_coord(&self) -> CoordI<'_> {
        panic!("Unimplemented: collapsed_coord")
    }
}
/*
  def collapsedCoord: CoordI[cI]
}
*/
// mig: struct AddressibleLocalVariableI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct AddressibleLocalVariableI<'s, 't> {
    pub name: IVarNameI<'t>,
    pub variability: VariabilityI,
    pub collapsed_coord: CoordI<'t>,
}
// mig: impl AddressibleLocalVariableI
/*
// Why the difference between reference and addressible:
// If we mutate/move a variable from inside a closure, we need to put
// the local's address into the struct. But, if the closures don't
// mutate/move, then we could just put a regular reference in the struct.
// Lucky for us, the parser figured out if any of our child closures did
// any mutates/moves/borrows.
case class AddressibleLocalVariableI(
  name: IVarNameI[cI],
  variability: VariabilityI,
  collapsedCoord: CoordI[cI]
) extends ILocalVariableI {
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for AddressibleLocalVariableI` below.)
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for AddressibleLocalVariableI` below.)
/*
override def equals(obj: Any): Boolean = vcurious();
}
*/
// mig: struct ReferenceLocalVariableI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ReferenceLocalVariableI<'s, 't> {
    pub name: IVarNameI<'t>,
    pub variability: VariabilityI,
    pub collapsed_coord: CoordI<'t>,
}
// mig: impl ReferenceLocalVariableI
/*
case class ReferenceLocalVariableI(
  name: IVarNameI[cI],
  variability: VariabilityI,
  collapsedCoord: CoordI[cI]
) extends ILocalVariableI {
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ReferenceLocalVariableI` below.)
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ReferenceLocalVariableI` below.)
/*
override def equals(obj: Any): Boolean = vcurious();
  vpass()
}
*/
// mig: struct AddressibleClosureVariableI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct AddressibleClosureVariableI<'s, 't> {
    pub name: IVarNameI<'t>,
    pub closured_vars_struct_type: StructIT<'t>,
    pub variability: VariabilityI,
    pub collapsed_coord: CoordI<'t>,
}
// mig: impl AddressibleClosureVariableI
/*
case class AddressibleClosureVariableI(
  name: IVarNameI[cI],
  closuredVarsStructType: StructIT[cI],
  variability: VariabilityI,
  collapsedCoord: CoordI[cI]
) extends IVariableI {
  vpass()
}
*/
// mig: struct ReferenceClosureVariableI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ReferenceClosureVariableI<'s, 't> {
    pub name: IVarNameI<'t>,
    pub closured_vars_struct_type: StructIT<'t>,
    pub variability: VariabilityI,
    pub collapsed_coord: CoordI<'t>,
}
// mig: impl ReferenceClosureVariableI
/*
case class ReferenceClosureVariableI(
  name: IVarNameI[cI],
  closuredVarsStructType: StructIT[cI],
  variability: VariabilityI,
  collapsedCoord: CoordI[cI]
) extends IVariableI {
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ReferenceClosureVariableI` below.)
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for ReferenceClosureVariableI` below.)
/*
override def equals(obj: Any): Boolean = vcurious();

}
*/