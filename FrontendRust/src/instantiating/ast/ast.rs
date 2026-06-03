use crate::interner::StrI;
use crate::utils::range::RangeS;
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::postparsing::names::IRuneS;
use crate::instantiating::ast::types::{CoordI, KindIT, ICitizenIT, MutabilityI, VariabilityI, cI, StructIT};
use crate::instantiating::ast::names::{
    IdI, INameI,
    IFunctionNameI, IImplNameI, IInterfaceNameI, IStructNameI, ICitizenNameI,
    IRegionNameI, IVarNameI,
    ExportNameI, FunctionBoundNameI, ImplBoundNameI,
};
use crate::instantiating::instantiating_interner::MustIntern;
use crate::instantiating::instantiating_interner::InstantiatingInterner;
use crate::instantiating::ast::expressions::ReferenceExpressionIE;

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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct KindExportI<'s, 'i> {
    pub range: RangeS<'s>,
    pub tyype: KindIT<'s, 'i, cI>,
    pub id: IdI<'s, 'i, cI>,
    pub exported_name: StrI<'s>,
}
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
// mig: impl KindExportI
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FunctionExportI<'s, 'i> where 's: 'i {
    pub range: RangeS<'s>,
    pub prototype: &'i PrototypeI<'s, 'i, cI>,
    pub export_id: IdI<'s, 'i, cI>,
    pub exported_name: StrI<'s>,
}
/*
case class FunctionExportI(
  range: RangeS,
  prototype: PrototypeI[cI],
  exportId: IdI[cI, ExportNameI[cI]],
  exportedName: StrI
)  {
*/
// mig: impl FunctionExportI
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FunctionExternI<'s, 'i> where 's: 'i {
    pub prototype: &'i PrototypeI<'s, 'i, cI>,
    // How many of the function's trailing generic-arg slots were inherited from a parent
    // citizen template, per @PRIIROZ (0 = no inheritance / top-level extern). Hammer uses
    // this to reshape the wire-format SimpleId so container template args land on the
    // citizen step (e.g. Vec<i32>::capacity rather than Vec::capacity<i32>), which is
    // what the Backend's rustifySimpleId expects per @SMLRZ.
    pub num_inherited_generic_parameters: i32,
}
/*
case class FunctionExternI(
    prototype: PrototypeI[cI],
    // How many of the function's trailing generic-arg slots were inherited from a parent
    // citizen template, per @PRIIROZ (0 = no inheritance / top-level extern). Hammer uses
    // this to reshape the wire-format SimpleId so container template args land on the
    // citizen step (e.g. Vec<i32>::capacity rather than Vec::capacity<i32>), which is
    // what the Backend's rustifySimpleId expects per @SMLRZ.
    numInheritedGenericParameters: Int) {
  vpass()
*/
// mig: impl FunctionExternI
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for FunctionExternI` below.)
/*
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Canonical groups equals/hashCode on one physical line — see the eq block above.)
/*
}
*/
// mig: struct KindExternI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct KindExternI<'s, 'i> where 's: 'i {
    pub r#struct: &'i StructIT<'s, 'i, cI>,
}
/*
case class KindExternI(struct: StructIT[cI]) {
*/
// mig: impl KindExternI
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by the #[derive(PartialEq, Eq)] above.)
/*
  override def equals(obj: Any): Boolean = vcurious(); override def hashCode(): Int = vcurious()
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Canonical groups equals/hashCode on one physical line — see the eq block above.)
/*
}
*/

// mig: struct InterfaceEdgeBlueprintI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceEdgeBlueprintI<'s, 'i> where 's: 'i {
    pub interface: IdI<'s, 'i, cI>,
    pub super_family_root_headers: &'i [(&'i PrototypeI<'s, 'i, cI>, i32)],
}
/*
case class InterfaceEdgeBlueprintI(
  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  interface: IdI[cI, IInterfaceNameI[cI]],
  superFamilyRootHeaders: Vector[(PrototypeI[cI], Int)]) {
*/
// mig: impl InterfaceEdgeBlueprintI
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
#[derive(PartialEq, Eq, Debug)]
pub struct EdgeI<'s, 'i> where 's: 'i {
    pub edge_id: IdI<'s, 'i, cI>,
    pub sub_citizen: ICitizenIT<'s, 'i, cI>,
    pub super_interface: IdI<'s, 'i, cI>,
    pub rune_to_func_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i, cI>>,
    pub rune_to_impl_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i, cI>>,
    pub abstract_func_to_override_func: ArenaIndexMap<'i, IdI<'s, 'i, cI>, &'i PrototypeI<'s, 'i, cI>>,
}
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
// mig: impl EdgeI
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
#[derive(Debug)]
pub struct FunctionDefinitionI<'s, 'i> where 's: 'i {
    pub header: FunctionHeaderI<'s, 'i>,
    pub rune_to_func_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i, cI>>,
    pub rune_to_impl_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i, cI>>,
    pub body: ReferenceExpressionIE<'s, 'i, cI>,
}
/*
case class FunctionDefinitionI(
  header: FunctionHeaderI,
  runeToFuncBound: Map[IRuneS, IdI[cI, FunctionBoundNameI[cI]]],
  runeToImplBound: Map[IRuneS, IdI[cI, ImplBoundNameI[cI]]],
  body: ReferenceExpressionIE)  {
*/
// mig: impl FunctionDefinitionI
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
impl<'s, 'i> FunctionDefinitionI<'s, 'i> {
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LocationInFunctionEnvironmentI<'i> {
    pub path: &'i [i32],
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
impl<'i> LocationInFunctionEnvironmentI<'i> {
    pub fn add(&self, sub_location: i32) -> LocationInFunctionEnvironmentI<'i> {
        panic!("Unimplemented: add")
    }
}
/*
  def +(subLocation: Int): LocationInFunctionEnvironmentI = {
    LocationInFunctionEnvironmentI(path :+ subLocation)
  }
*/
// mig: fn to_string
impl<'i> LocationInFunctionEnvironmentI<'i> {
    pub fn to_string(&self) -> String {
        self.path.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(".")
    }
}
/*
  override def toString: String = path.mkString(".")
}
*/
// mig: struct AbstractI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AbstractI;
// mig: impl AbstractI
/*
case class AbstractI()
*/
// mig: struct ParameterI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ParameterI<'s, 'i> where 's: 'i {
    pub name: IVarNameI<'s, 'i, cI>,
    pub virtuality: Option<AbstractI>,
    pub pre_checked: bool,
    pub tyype: CoordI<'s, 'i, cI>,
}
/*
case class ParameterI(
  name: IVarNameI[cI],
  virtuality: Option[AbstractI],
  preChecked: Boolean,
  tyype: CoordI[cI]) {
*/
// mig: impl ParameterI
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
impl<'s, 'i> ParameterI<'s, 'i> {
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
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SignatureI<'s, 'i, R> {
    pub id: IdI<'s, 'i, R>,
    pub _must_intern: MustIntern,
}

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SignatureIValI<'s, 'i, R> {
    pub id: IdI<'s, 'i, R>,
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
impl<'s, 'i, R> SignatureI<'s, 'i, R> {
    pub fn param_types(&self) -> Vec<()> {
        panic!("Unimplemented: param_types")
    }
}
/*
  def paramTypes: Vector[CoordI[R]] = id.localName.parameters
}
*/
// mig: enum IFunctionAttributeI
/// Polyvalue
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IFunctionAttributeI<'s> {
    PureI,
    UserFunctionI,
    ExternI(ExternI<'s>),
}
/*
sealed trait IFunctionAttributeI
*/
// mig: impl IFunctionAttributeI
// mig: enum ICitizenAttributeI
/// Polyvalue
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ICitizenAttributeI<'s> {
    SealedI,
    ExternI(ExternI<'s>),
}
/*
sealed trait ICitizenAttributeI
*/
// mig: impl ICitizenAttributeI
// mig: struct ExternI
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExternI<'s> {
    pub package_coord: crate::utils::code_hierarchy::PackageCoordinate<'s>,
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RegionI<'s, 'i> where 's: 'i {
    pub name: IRegionNameI<'s, 'i, cI>,
    pub mutable: bool,
}
/*
case class RegionI(
  name: IRegionNameI[cI],
  mutable: Boolean)
*/
// mig: impl RegionI
// mig: struct FunctionHeaderI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FunctionHeaderI<'s, 'i> where 's: 'i {
    // This one little name field can illuminate much of how the compiler works, see UINIT.
    pub id: IdI<'s, 'i, cI>,
    pub attributes: &'i [IFunctionAttributeI<'s>],
//  regions: Vector[cIegionI],
    pub params: &'i [ParameterI<'s, 'i>],
    pub return_type: CoordI<'s, 'i, cI>,
}
/*
case class FunctionHeaderI(
  // This one little name field can illuminate much of how the compiler works, see UINIT.
  id: IdI[cI, IFunctionNameI[cI]],
  attributes: Vector[IFunctionAttributeI],
//  regions: Vector[cIegionI],
  params: Vector[ParameterI],
  returnType: CoordI[cI]) {
*/
// mig: impl FunctionHeaderI
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
impl<'s, 'i> FunctionHeaderI<'s, 'i> {
    pub fn is_extern(&self) -> bool {
        panic!("Unimplemented: is_extern")
    }
}
/*
  def isExtern = attributes.exists({ case ExternI(_) => true case _ => false })
  //  def isExport = attributes.exists({ case Export2(_) => true case _ => false })
*/
// mig: fn is_user_function
impl<'s, 'i> FunctionHeaderI<'s, 'i> {
    pub fn is_user_function(&self) -> bool {
        self.attributes.contains(&IFunctionAttributeI::UserFunctionI)
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
impl<'s, 'i> FunctionHeaderI<'s, 'i> {
    pub fn get_abstract_interface(&self) -> Option<&'i crate::instantiating::ast::types::InterfaceIT<'s, 'i, cI>> {
        let abstract_interfaces: Vec<_> = self.params.iter().filter_map(|p| match (p.virtuality, p.tyype.kind) {
            (Some(AbstractI), crate::instantiating::ast::types::KindIT::InterfaceIT(ir)) => Some(ir),
            _ => None,
        }).collect();
        assert!(abstract_interfaces.len() <= 1);
        abstract_interfaces.into_iter().next()
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
impl<'s, 'i> FunctionHeaderI<'s, 'i> {
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
impl<'s, 'i> FunctionHeaderI<'s, 'i> {
    pub fn to_prototype(&self, interner: &InstantiatingInterner<'s, 'i>) -> PrototypeI<'s, 'i, cI> {
        //    val substituter = TemplataCompiler.getPlaceholderSubstituter(interner, fullName, templateArgs)
        //    val paramTypes = params.map(_.tyype).map(substituter.substituteForCoord)
        //    val newLastStep = fullName.last.makeFunctionName(interner, keywords, templateArgs, paramTypes)
        //    val newName = FullNameI(fullName.packageCoord, fullName.initSteps, newLastStep)
        *interner.intern_prototype_ci(PrototypeIValI { id: self.id, return_type: self.return_type })
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
impl<'s, 'i> FunctionHeaderI<'s, 'i> {
    pub fn to_signature(&self) -> SignatureI<'_, '_, ()> {
        panic!("Unimplemented: to_signature")
    }
}
/*
  def toSignature: SignatureI[cI] = {
    toPrototype.toSignature
  }
*/
// mig: fn param_types
impl<'s, 'i> FunctionHeaderI<'s, 'i> where 's: 'i {
    pub fn param_types(&self) -> Vec<CoordI<'s, 'i, cI>> {
        IFunctionNameI::try_from(self.id.local_name).unwrap().parameters().to_vec()
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
impl<'s, 'i> FunctionHeaderI<'s, 'i> {
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
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrototypeI<'s, 'i, R> {
    pub id: IdI<'s, 'i, R>,
    pub return_type: CoordI<'s, 'i, R>,
    pub _must_intern: MustIntern,
}

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrototypeIValI<'s, 'i, R> {
    pub id: IdI<'s, 'i, R>,
    pub return_type: CoordI<'s, 'i, R>,
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
impl<'s, 'i, R: Copy> PrototypeI<'s, 'i, R> where 's: 'i {
    pub fn param_types(&self) -> Vec<CoordI<'s, 'i, R>> {
        IFunctionNameI::try_from(self.id.local_name).unwrap().parameters().to_vec()
    }
}
/*
  def paramTypes: Vector[CoordI[R]] = id.localName.parameters
*/
// mig: fn to_signature
impl<'s, 'i, R> PrototypeI<'s, 'i, R> {
    pub fn to_signature(&self) -> SignatureI<'_, '_, ()> {
        panic!("Unimplemented: to_signature")
    }
}
/*
  def toSignature: SignatureI[R] = SignatureI[R](id)
}
*/
// mig: enum IVariableI
/// Polyvalue
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IVariableI<'s, 'i> where 's: 'i {
    AddressibleLocalVariableI(&'i AddressibleLocalVariableI<'s, 'i>),
    ReferenceLocalVariableI(&'i ReferenceLocalVariableI<'s, 'i>),
    AddressibleClosureVariableI(&'i AddressibleClosureVariableI<'s, 'i>),
    ReferenceClosureVariableI(&'i ReferenceClosureVariableI<'s, 'i>),
}
/*
sealed trait IVariableI  {
*/
// mig: impl IVariableI
// mig: fn name
impl<'s, 'i> IVariableI<'s, 'i> {
    pub fn name(&self) -> () {
        panic!("Unimplemented: name")
    }
}
/*
  def name: IVarNameI[cI]
*/
// mig: fn variability
impl<'s, 'i> IVariableI<'s, 'i> {
    pub fn variability(&self) -> () {
        panic!("Unimplemented: variability")
    }
}
/*
  def variability: VariabilityI
*/
// mig: fn collapsed_coord
impl<'s, 'i> IVariableI<'s, 'i> {
    pub fn collapsed_coord(&self) -> () {
        panic!("Unimplemented: collapsed_coord")
    }
}
/*
  def collapsedCoord: CoordI[cI]
}
*/
// mig: enum ILocalVariableI
/// Polyvalue
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ILocalVariableI<'s, 'i> where 's: 'i {
    AddressibleLocalVariableI(&'i AddressibleLocalVariableI<'s, 'i>),
    ReferenceLocalVariableI(&'i ReferenceLocalVariableI<'s, 'i>),
}
/*
sealed trait ILocalVariableI extends IVariableI {
*/
// mig: impl ILocalVariableI
// mig: fn name
impl<'s, 'i> ILocalVariableI<'s, 'i> {
    pub fn name(&self) -> IVarNameI<'s, 'i, cI> {
        match self {
            ILocalVariableI::ReferenceLocalVariableI(rlv) => rlv.name,
            ILocalVariableI::AddressibleLocalVariableI(alv) => alv.name,
        }
    }
}
/*
  def name: IVarNameI[cI]
*/
// mig: fn collapsed_coord
impl<'s, 'i> ILocalVariableI<'s, 'i> {
    pub fn collapsed_coord(&self) -> CoordI<'s, 'i, cI> {
        match self {
            ILocalVariableI::AddressibleLocalVariableI(alv) => alv.collapsed_coord,
            ILocalVariableI::ReferenceLocalVariableI(rlv) => rlv.collapsed_coord,
        }
    }
}
/*
  def collapsedCoord: CoordI[cI]
}
*/
// mig: fn variability (inherited from IVariableI, no direct ILocalVariableI Scala decl)
impl<'s, 'i> ILocalVariableI<'s, 'i> {
    pub fn variability(&self) -> VariabilityI {
        match self {
            ILocalVariableI::AddressibleLocalVariableI(alv) => alv.variability,
            ILocalVariableI::ReferenceLocalVariableI(rlv) => rlv.variability,
        }
    }
}
/* */
// mig: struct AddressibleLocalVariableI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AddressibleLocalVariableI<'s, 'i> where 's: 'i {
    pub name: IVarNameI<'s, 'i, cI>,
    pub variability: VariabilityI,
    pub collapsed_coord: CoordI<'s, 'i, cI>,
}
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
// mig: impl AddressibleLocalVariableI
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ReferenceLocalVariableI<'s, 'i> where 's: 'i {
    pub name: IVarNameI<'s, 'i, cI>,
    pub variability: VariabilityI,
    pub collapsed_coord: CoordI<'s, 'i, cI>,
}
/*
case class ReferenceLocalVariableI(
  name: IVarNameI[cI],
  variability: VariabilityI,
  collapsedCoord: CoordI[cI]
) extends ILocalVariableI {
*/
// mig: impl ReferenceLocalVariableI
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AddressibleClosureVariableI<'s, 'i> where 's: 'i {
    pub name: IVarNameI<'s, 'i, cI>,
    pub closured_vars_struct_type: StructIT<'s, 'i, cI>,
    pub variability: VariabilityI,
    pub collapsed_coord: CoordI<'s, 'i, cI>,
}
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
// mig: impl AddressibleClosureVariableI
// mig: struct ReferenceClosureVariableI
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ReferenceClosureVariableI<'s, 'i> where 's: 'i {
    pub name: IVarNameI<'s, 'i, cI>,
    pub closured_vars_struct_type: StructIT<'s, 'i, cI>,
    pub variability: VariabilityI,
    pub collapsed_coord: CoordI<'s, 'i, cI>,
}
/*
case class ReferenceClosureVariableI(
  name: IVarNameI[cI],
  closuredVarsStructType: StructIT[cI],
  variability: VariabilityI,
  collapsedCoord: CoordI[cI]
) extends IVariableI {
*/
// mig: impl ReferenceClosureVariableI
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