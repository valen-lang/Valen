use std::collections::HashMap;

use crate::interner::StrI;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::RangeS;

use crate::postparsing::names::*;

use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::ast::expressions::*;
use crate::typing::hinputs_t::*;
use crate::typing::typing_interner::TypingInterner;
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::typing::names::names::IdValQuery;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::ptr::eq;
use std::ptr::hash;

/*
package dev.vale.typing.ast

import dev.vale.highertyping.FunctionA
import dev.vale.typing.names._
import dev.vale.typing.templata.FunctionTemplataT
import dev.vale.{PackageCoordinate, RangeS, vassert, vcurious, vfail}
import dev.vale.typing.types._
import dev.vale._
import dev.vale.postparsing._
import dev.vale.typing._
import dev.vale.typing.env.IInDenizenEnvironmentT
import dev.vale.typing.templata._
import dev.vale.typing.types._

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
/// Arena-allocated (see @TFITCX)
pub struct ImplT<'s, 't> {
    pub templata: ImplDefinitionTemplataT<'s, 't>,
    pub instantiated_id: IdT<'s, 't>,
    pub template_id: IdT<'s, 't>,
    pub sub_citizen_template_id: IdT<'s, 't>,
    pub sub_citizen: ICitizenTT<'s, 't>,
    pub super_interface: InterfaceTT<'s, 't>,
    pub super_interface_template_id: IdT<'s, 't>,
    pub instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
    pub rune_index_to_independence: &'t [bool],
}
/*
case class ImplT(
  // These are ICitizenTT and InterfaceTT which likely have placeholder templatas in them.
  // We do this because a struct might implement an interface in multiple ways, see SCIIMT.
  // We have the template names as well as the placeholders for better searching, see MLUIBTN.

  templata: ImplDefinitionTemplataT,

  instantiatedId: IdT[IImplNameT],
  templateId: IdT[IImplTemplateNameT],

  subCitizenTemplateId: IdT[ICitizenTemplateNameT],
  subCitizen: ICitizenTT,

  superInterface: InterfaceTT,
  superInterfaceTemplateId: IdT[IInterfaceTemplateNameT],

  // This is similar to FunctionT.instantiationBoundParams.
  // We'll line up anything in here with the instantiation bound args to form a nice
  // map the instantiator can use. See IBAMIBP.
  instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT],

  runeIndexToIndependence: Vector[Boolean],
) {
  vpass()
}
*/
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct KindExportT<'s, 't> {
    pub range: RangeS<'s>,
    pub tyype: KindT<'s, 't>,
    pub id: IdT<'s, 't>,
    pub exported_name: StrI<'s>,
}
/*
case class KindExportT(
  range: RangeS,
  tyype: KindT,
  // Good for knowing the package of this export for later prefixing the exportedName, also good
  // for getting its region.
  id: IdT[ExportNameT],
  exportedName: StrI
)  {
*/
impl<'s, 't> KindExportT<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
  override def hashCode(): Int = vcurious()

}
*/
}
/// Arena-allocated (see @TFITCX)
pub struct FunctionExportT<'s, 't> {
    pub range: RangeS<'s>,
    pub prototype: PrototypeT<'s, 't>,
    pub export_id: IdT<'s, 't>,
    pub exported_name: StrI<'s>,
}
/*
case class FunctionExportT(
  range: RangeS,
  prototype: PrototypeT[IFunctionNameT],
  exportId: IdT[ExportNameT],
  exportedName: StrI
)  {
*/
impl<'s, 't> FunctionExportT<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
  override def hashCode(): Int = vcurious()
  vpass()
}
*/
}
/// Arena-allocated (see @TFITCX)
pub struct KindExternT<'s, 't> {
    pub tyype: KindT<'s, 't>,
    pub package_coordinate: PackageCoordinate<'s>,
    pub extern_name: StrI<'s>,
}
/*
case class KindExternT(
  tyype: KindT,
  packageCoordinate: PackageCoordinate,
  externName: StrI
)  {
*/
impl<'s, 't> KindExternT<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
  override def hashCode(): Int = vcurious()

}
*/
}
/// Arena-allocated (see @TFITCX)
pub struct FunctionExternT<'s, 't> {
    pub range: RangeS<'s>,
    pub extern_placeholdered_id: IdT<'s, 't>,
    pub prototype: PrototypeT<'s, 't>,
    pub extern_name: StrI<'s>,
    pub generic_parameter_inheritance: Option<GenericParametersInheritance>,
}
/*
case class FunctionExternT(
  range: RangeS,
  externPlaceholderedId: IdT[ExternNameT],
  prototype: PrototypeT[IFunctionNameT],
  externName: StrI,
  // None if this was a top-level function; not inside a struct.
  // Some if this was a function declared inside a struct.
  // The count is how many generic parameters were inherited from the container, per @PRIIROZ.
  // Hammer uses this to reshape the wire-format SimpleId so container template args land on
  // the citizen step (e.g. Vec<i32>::capacity rather than Vec::capacity<i32>), which is what
  // the Backend's rustifySimpleId expects per @SMLRZ. AFTERM: would also let a future tcx
  // lookup find the right rustc associated function from the container's impl.
  genericParameterInheritance: Option[GenericParametersInheritance]
)  {
*/
impl<'s, 't> FunctionExternT<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
  override def hashCode(): Int = vcurious()

}
*/
}
/// Arena-allocated (see @TFITCX)
pub struct InterfaceEdgeBlueprintT<'s, 't> {
    pub interface: IdT<'s, 't>,
    pub super_family_root_headers: &'t [(PrototypeT<'s, 't>, i32)],
}
/*
case class InterfaceEdgeBlueprintT(
  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  interface: IdT[IInterfaceNameT],
  superFamilyRootHeaders: Vector[(PrototypeT[IFunctionNameT], Int)]) {
    val hash = runtime.ScalaRunTime._hashCode(this);
*/
impl<'s, 't> InterfaceEdgeBlueprintT<'s, 't> {
/*
  override def hashCode(): Int = hash;
*/
/*
  override def equals(obj: Any): Boolean = vcurious(); }
*/
}
/// Arena-allocated (see @TFITCX)
pub struct OverrideT<'s, 't> {
    pub dispatcher_call_id: IdT<'s, 't>,
    pub impl_placeholder_to_dispatcher_placeholder: &'t [(IdT<'s, 't>, ITemplataT<'s, 't>)],
    pub impl_placeholder_to_case_placeholder: &'t [(IdT<'s, 't>, ITemplataT<'s, 't>)],
    pub dispatcher_and_case_placeholdered_impl_reachable_prototypes: ArenaIndexMap<'t, IRuneS<'s>, ArenaIndexMap<'t, IRuneS<'s>, PrototypeT<'s, 't>>>,
    pub case_id: IdT<'s, 't>,
    pub override_prototype: PrototypeT<'s, 't>,
    pub dispatcher_instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
}
/*
case class OverrideT(
  // This is the name of the conceptual function called by the abstract function.
  // It has enough information to do simple dispatches, but not all cases, it can't handle
  // the Milano case, see OMCNAGP.
  // This will have some placeholders from the abstract function; this is the abstract function
  // calling the dispatcher.
  // This is like:
  //   abstract func send<T>(self &IObserver<T>, event T) void
  // calling:
  //   func send<int>(self &IObserver<int>, event int) void
  // or a more complex case:
  //   func send<Opt<int>>(self &IObserver<Opt<int>>, event Opt<int>) void
  // as you can see there may be some interesting templatas in there like that Opt<int>, they
  // might not be simple placeholders.
  dispatcherCallId: IdT[OverrideDispatcherNameT],

  implPlaceholderToDispatcherPlaceholder: Vector[(IdT[IPlaceholderNameT], ITemplataT[ITemplataType])],
  implPlaceholderToCasePlaceholder: Vector[(IdT[IPlaceholderNameT], ITemplataT[ITemplataType])],

  // These are the prototypes we'll pull from the impl's own bounds, and these CaseFunctionFromImplNameT names contain
  // the rune that the impl internally refers to them as.
  dispatcherAndCasePlaceholderedImplReachablePrototypes: Map[IRuneS, Map[IRuneS, PrototypeT[FunctionBoundNameT]]],

  // This is the name of the conceptual case that's calling the override prototype. It'll have
  // template args inherited from the dispatcher function and template args inherited from the
  // translated from the impl into "case placeholders". After typing pass these will be placeholders, and after
  // instantiator these will be actual real templatas.
  caseId: IdT[OverrideDispatcherCaseNameT],

  // The override function we're calling.
  // Conceptually, this is being called from the case's environment. It might even have some complex stuff
  // in the template args.
  overridePrototype: PrototypeT[IFunctionNameT],

  // Any FunctionT has a runeToFunctionBound, which is a map of the function's rune to its required
  // bounds. This is the one for our conceptual dispatcher function.
  dispatcherInstantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT],
)
*/
/// Arena-allocated (see @TFITCX)
pub struct EdgeT<'s, 't> {
    pub edge_id: IdT<'s, 't>,
    pub sub_citizen: ICitizenTT<'s, 't>,
    pub super_interface: IdT<'s, 't>,
    pub instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
    pub abstract_func_to_override_func: ArenaIndexMap<'t, IdT<'s, 't>, &'t OverrideT<'s, 't>>,
}
/*
case class EdgeT(
  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  edgeId: IdT[IImplNameT],
  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  subCitizen: ICitizenTT,
  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  superInterface: IdT[IInterfaceNameT],
  // This is similar to FunctionT.runeToFuncBound
  instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT],
  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  abstractFuncToOverrideFunc: Map[IdT[IFunctionNameT], OverrideT]
) {
  vpass()
*/
impl<'s, 't> EdgeT<'s, 't> {
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
/*
  override def equals(obj: Any): Boolean = {
    obj match {
      case EdgeT(thatEdgeId, thatStruct, thatInterface, _, _) => {
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
}
/// Arena-allocated (see @TFITCX)
pub struct FunctionDefinitionT<'s, 't> {
    pub header: &'t FunctionHeaderT<'s, 't>,
    pub instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
    pub body: ReferenceExpressionTE<'s, 't>,
}
/*
case class FunctionDefinitionT(
  header: FunctionHeaderT,
  instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT],
  body: ReferenceExpressionTE)  {
*/
impl<'s, 't> FunctionDefinitionT<'s, 't> {
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
/*
  override def hashCode(): Int = vcurious()

*/
}
impl<'s, 't> FunctionDefinitionT<'s, 't> where 's: 't, {
    fn new(
        header: FunctionHeaderT<'s, 't>,
        instantiation_bound_params: InstantiationBoundArgumentsT<'s, 't>,
        body: ReferenceExpressionTE<'s, 't>,
    ) -> FunctionDefinitionT<'s, 't> { panic!("Unimplemented: FunctionDefinitionT::new"); }
/*
  // We always end a function with a ret, whose result is a Never.
  vassert(body.result.kind == NeverT(false))
*/
}
impl<'s, 't> FunctionDefinitionT<'s, 't> {
    fn is_pure(&self) -> bool { panic!("Unimplemented: is_pure"); }
/*
  def isPure: Boolean = header.isPure
}

object getFunctionLastName {
*/
}
fn get_function_last_name_unapply<'s, 't>(f: &'t FunctionDefinitionT<'s, 't>) -> Option<&'t IFunctionNameT<'s, 't>> { panic!("Unimplemented: unapply"); }
/*
  def unapply(f: FunctionDefinitionT): Option[IFunctionNameT] = Some(f.header.id.localName)
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LocationInFunctionEnvironmentT<'s, 't> {
    pub path: &'t [i32],
    pub _phantom: PhantomData<&'s ()>,
}
/*
// A unique location in a function. Environment is in the name so it spells LIFE!
case class LocationInFunctionEnvironmentT(path: Vector[Int]) {
*/
impl<'s, 't> LocationInFunctionEnvironmentT<'s, 't> {
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
    // Rust adaptation (SPDMX-B): interner needed to allocate &'t [i32] slice for arena-immutable storage.
    pub fn add(&self, interner: &TypingInterner<'s, 't>, sub_location: i32) -> LocationInFunctionEnvironmentT<'s, 't> {
        let mut new_path: Vec<i32> = self.path.to_vec();
        new_path.push(sub_location);
        LocationInFunctionEnvironmentT { path: interner.alloc_slice_from_vec(new_path), _phantom: PhantomData }
    }
/*
  def +(subLocation: Int): LocationInFunctionEnvironmentT = {
    LocationInFunctionEnvironmentT(path :+ subLocation)
  }
*/
    fn to_string(&self) -> String { panic!("Unimplemented: to_string"); }
/*
  override def toString: String = path.mkString(".")
}
*/
}
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct AbstractT;
/*
case class AbstractT()
*/
/// Arena-allocated (see @TFITCX)
#[derive(Clone, Debug)]
pub struct ParameterT<'s, 't> {
    pub name: IVarNameT<'s, 't>,
    pub virtuality: Option<AbstractT>,
    pub pre_checked: bool,
    pub tyype: CoordT<'s, 't>,
}
/*
case class ParameterT(
  name: IVarNameT,
  virtuality: Option[AbstractT],
  preChecked: Boolean,
  tyype: CoordT)  {
*/
impl<'s, 't> ParameterT<'s, 't> {
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

  // Use same instead, see EHCFBD for why we dont like equals.
*/
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
    fn same(&self, that: &ParameterT<'s, 't>) -> bool { panic!("Unimplemented: same"); }
/*
  def same(that: ParameterT): Boolean = {
    name == that.name &&
      virtuality == that.virtuality &&
      tyype == that.tyype
  }
}
*/
}
/// Temporary state (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ICalleeCandidate<'s, 't> {
    Function(FunctionCalleeCandidate<'s, 't>),
    Header(&'t HeaderCalleeCandidate<'s, 't>),
    PrototypeTemplata(PrototypeTemplataCalleeCandidate<'s, 't>),
}
/*
sealed trait ICalleeCandidate
*/
/// Temporary state (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FunctionCalleeCandidate<'s, 't> {
    pub ft: FunctionTemplataT<'s, 't>,
}
/*
case class FunctionCalleeCandidate(ft: FunctionTemplataT) extends ICalleeCandidate {
*/
impl<'s, 't> FunctionCalleeCandidate<'s, 't> {
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
*/
}
/// Temporary state (see @TFITCX)
#[derive(PartialEq, Eq, Hash, Debug)]
pub struct HeaderCalleeCandidate<'s, 't> {
    pub header: FunctionHeaderT<'s, 't>,
}
/*
case class HeaderCalleeCandidate(header: FunctionHeaderT) extends ICalleeCandidate {
*/
impl<'s, 't> HeaderCalleeCandidate<'s, 't> {
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
*/
}
/// Temporary state (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrototypeTemplataCalleeCandidate<'s, 't> {
    pub prototype_t: PrototypeT<'s, 't>,
}
/*
case class PrototypeTemplataCalleeCandidate(
  // We don't want a range because we want to merge all sorts of different bound functions, see MFBFDP.
  //   range: RangeS,
  prototypeT: PrototypeT[IFunctionNameT]) extends ICalleeCandidate {
*/
impl<'s, 't> PrototypeTemplataCalleeCandidate<'s, 't> {
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}

*/
/*
//sealed trait IValidCalleeCandidate {
//  def range: Option[RangeS]
//  def paramTypes: Vector[CoordT]
//}
*/
/*
//case class ValidHeaderCalleeCandidate(
//  header: FunctionHeaderT
//) extends IValidCalleeCandidate {
//  val hash = runtime.ScalaRunTime._hashCode(this);
//override def hashCode(): Int = hash;
//override def equals(obj: Any): Boolean = vcurious();
//
//  override def range: Option[RangeS] = header.maybeOriginFunctionTemplata.map(_.function.range)
//  override def paramTypes: Vector[CoordT] = header.paramTypes.toVector
//}
*/
/*
//case class ValidPrototypeTemplataCalleeCandidate(
//  prototype: PrototypeTemplataT[IFunctionNameT]
//) extends IValidCalleeCandidate {
//  val hash = runtime.ScalaRunTime._hashCode(this);
//override def hashCode(): Int = hash;
//  override def equals(obj: Any): Boolean = {
//    val that = obj.asInstanceOf[ValidPrototypeTemplataCalleeCandidate]
//    if (that == null) {
//      return false
//    }
//    prototype == that.prototype
//  }
//
//  override def range: Option[RangeS] = None
//  override def paramTypes: Vector[CoordT] = prototype.prototype.id.localName.parameters.toVector
//}
*/
/*
////case class ValidCalleeCandidate(
////  banner: FunctionHeaderT,
////  templateArgs: Vector[ITemplataT[ITemplataType]],
////  function: FunctionTemplataT
////) extends IValidCalleeCandidate {
////  val hash = runtime.ScalaRunTime._hashCode(this);
//override def hashCode(): Int = hash;
//override def equals(obj: Any): Boolean = vcurious();
////
////  override def range: Option[RangeS] = banner.maybeOriginFunctionTemplata.map(_.function.range)
////  override def paramTypes: Vector[CoordT] = banner.paramTypes.toVector
////}

*/
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

*/
}
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SignatureT<'s, 't> {
    pub id: IdT<'s, 't>,
}
/*
case class SignatureT(id: IdT[IFunctionNameT]) {
*/
impl<'s, 't> SignatureT<'s, 't> {
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
    fn param_types(&self) -> Vec<CoordT<'s, 't>> { panic!("Unimplemented: param_types"); }
/*
  def paramTypes: Vector[CoordT] = id.localName.parameters
}
*/
}

// (no scala counterpart — Rust-only interning scaffolding)
// Transient Val for interning: holds a stack-borrowed IdValT<'s, 't, 'tmp> so
// callers can construct a lookup key without first arena-allocating init_steps.
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct SignatureValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub id: IdValT<'s, 't, 'tmp>,
}

/// Interning transient (see @TFITCX)
pub struct SignatureValQuery<'a, 's, 't, 'tmp>(pub &'a SignatureValT<'s, 't, 'tmp>)
where 's: 't, 't: 'tmp;

impl<'a, 's, 't, 'tmp> Hash for SignatureValQuery<'a, 's, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    fn hash<H: Hasher>(&self, state: &mut H) { self.0.hash(state); }
    /* Guardian: disable-all */
}

impl<'a, 's, 't, 'tmp> hashbrown::Equivalent<SignatureValT<'s, 't, 't>> for SignatureValQuery<'a, 's, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    fn equivalent(&self, key: &SignatureValT<'s, 't, 't>) -> bool {
        IdValQuery(&self.0.id).equivalent(&key.id)
    }
    /* Guardian: disable-all */
}
/// Value-type (see @TFITCX)
pub struct FunctionBannerT<'s, 't> {
    pub origin_function_templata: Option<FunctionTemplataT<'s, 't>>,
    pub name: IdT<'s, 't>,
}
/*
case class FunctionBannerT(
  originFunctionTemplata: Option[FunctionTemplataT],
  name: IdT[IFunctionNameT])   {
*/
impl<'s, 't> FunctionBannerT<'s, 't> {
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

  // Use same instead, see EHCFBD for why we dont like equals.
*/
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
    pub fn same(&self, that: &FunctionBannerT<'s, 't>) -> bool {
        let self_func = self.origin_function_templata.map(|t| t.function as *const _);
        let that_func = that.origin_function_templata.map(|t| t.function as *const _);
        self_func == that_func && self.name == that.name
    }
/*
  def same(that: FunctionBannerT): Boolean = {
    originFunctionTemplata.map(_.function) == that.originFunctionTemplata.map(_.function) && name == that.name
  }
*/
/*



//  def unapply(arg: FunctionBannerT):
//  Option[(FullNameT[IFunctionNameT], Vector[ParameterT])] =
//    Some(templateName, params)
*/
    fn to_string(&self) -> String { panic!("Unimplemented: to_string"); }
/*
  override def toString: String = {
    // # is to signal that we override this
//    "FunctionBanner2#(" + templateName + ")"
//        "FunctionBanner2#(" + templateName + ", " + params + ")"
    "FunctionBanner2#(" + name + ")"
  }
}
*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Clone, PartialEq, Debug)]
pub enum IFunctionAttributeT<'s> {
    Extern(ExternT<'s>),
    Pure,
    Additive,
    UserFunction,
}
/*
sealed trait IFunctionAttributeT
*/
/// Arena-allocated (see @TFITCX)
pub enum ICitizenAttributeT<'s> {
    Extern(ExternT<'s>),
    Sealed,
}
/*
sealed trait ICitizenAttributeT
*/
/// Arena-allocated (see @TFITCX)
#[derive(Clone, PartialEq, Debug)]
pub struct ExternT<'s> {
    pub package_coord: PackageCoordinate<'s>,
}
/*
case class ExternT(packageCoord: PackageCoordinate) extends IFunctionAttributeT with ICitizenAttributeT { // For optimization later
*/
impl<'s> ExternT<'s> {
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
*/
/*
// There's no Export2 here, we use separate KindExport and FunctionExport constructs.
//case class Export2(packageCoord: PackageCoordinate) extends IFunctionAttribute2 with ICitizenAttribute2
*/
/*
case object PureT extends IFunctionAttributeT
*/
/*
case object AdditiveT extends IFunctionAttributeT
*/
/*
case object SealedT extends ICitizenAttributeT
*/
/*
case object UserFunctionT extends IFunctionAttributeT // Whether it was written by a human. Mostly for tests right now.
*/
}
/// Arena-allocated (see @TFITCX)
#[derive(Debug)]
pub struct FunctionHeaderT<'s, 't> {
    pub id: IdT<'s, 't>,
    pub attributes: &'t [IFunctionAttributeT<'s>],
    pub params: &'t [ParameterT<'s, 't>],
    pub return_type: CoordT<'s, 't>,
    pub maybe_origin_function_templata: Option<FunctionTemplataT<'s, 't>>,
}
/*
case class FunctionHeaderT(
  // This one little name field can illuminate much of how the compiler works, see UINIT.
  id: IdT[IFunctionNameT],
  attributes: Vector[IFunctionAttributeT],
  params: Vector[ParameterT],
  returnType: CoordT,
  maybeOriginFunctionTemplata: Option[FunctionTemplataT]) {
*/

// Identity equality per @IEOIBZ — `FunctionHeaderT` is arena-allocated.
impl<'s, 't> PartialEq for FunctionHeaderT<'s, 't> {
    fn eq(&self, other: &Self) -> bool { eq(self, other) }
    /* Guardian: disable-all */
}
impl<'s, 't> Eq for FunctionHeaderT<'s, 't> {}
impl<'s, 't> Hash for FunctionHeaderT<'s, 't> {
    fn hash<H: Hasher>(&self, state: &mut H) { hash(self, state) }
    /* Guardian: disable-all */
}
impl<'s, 't> FunctionHeaderT<'s, 't> {
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
    fn new(
        id: IdT<'s, 't>,
        attributes: Vec<IFunctionAttributeT<'s>>,
        params: Vec<ParameterT<'s, 't>>,
        return_type: CoordT<'s, 't>,
        maybe_origin_function_templata: Option<FunctionTemplataT<'s, 't>>,
    ) -> FunctionHeaderT<'s, 't> { panic!("Unimplemented: FunctionHeaderT::new"); }
/*
  vassert({
    maybeOriginFunctionTemplata match {
      case None =>
      case Some(originFunctionTemplata) => {
        val templateName = TemplataCompiler.getFunctionTemplate(id)
        val placeholderInThisFunctionNames =
          Collector.all(id, {
            case KindPlaceholderT(name) => name
            case PlaceholderTemplataT(name, _) => name
          })
        // Filter out any placeholders that came from the parent, in case this is a lambda function.
        val selfPlaceholdersInThisFunctionName =
          placeholderInThisFunctionNames.filter({ case IdT(packageCoord, initSteps, last) =>
            val parentName = IdT(packageCoord, initSteps.init, initSteps.last)
            // Not sure which one it is, this should catch both.
            parentName == id || parentName == templateName
          })

        if (originFunctionTemplata.function.isLambda()) {
          // make sure there are no placeholders
          vassert(selfPlaceholdersInThisFunctionName.isEmpty)
        } else {
          if (originFunctionTemplata.function.genericParameters.isEmpty) {
            // make sure there are no placeholders
            vassert(selfPlaceholdersInThisFunctionName.isEmpty)
          } else {
            // Make sure all the placeholders in the generic parameters exist as template args in
            // the original function definition.
            selfPlaceholdersInThisFunctionName.foreach({
              case placeholderName @ IdT(_, _, NonKindNonRegionPlaceholderNameT(index,rune)) => {
                id.localName.templateArgs(index) match {
                  case PlaceholderTemplataT(placeholderNameAtIndex, _) => {
                    vassert(placeholderName == placeholderNameAtIndex)
                  }
                  case _ => vfail()
                }
              }
              case placeholderName @ IdT(_, _, KindPlaceholderNameT(KindPlaceholderTemplateNameT(index, rune))) => {
                id.localName.templateArgs(index) match {
                  case KindTemplataT(KindPlaceholderT(placeholderNameAtIndex)) => {
                    vassert(placeholderName == placeholderNameAtIndex)
                  }
                  case CoordTemplataT(CoordT(_, _, KindPlaceholderT(placeholderNameAtIndex))) => {
                    vassert(placeholderName == placeholderNameAtIndex)
                  }
                  case _ => vfail()
                }
              }
            })
          }
        }
      }
    }
    true
  })

*/
/*
  override def equals(obj: Any): Boolean = {
    obj match {
      case FunctionHeaderT(thatName, _, _, _, _) => {
        id == thatName
      }
      case _ => false
    }
  }

*/
/*
  // Make sure there's no duplicate names
  vassert(params.map(_.name).toSet.size == params.size);

  vassert(id.localName.parameters == paramTypes)
*/
    fn is_extern(&self) -> bool { panic!("Unimplemented: is_extern"); }
/*
  def isExtern = attributes.exists({ case ExternT(_) => true case _ => false })
*/
/*
  //  def isExport = attributes.exists({ case Export2(_) => true case _ => false })
*/
    pub fn is_user_function(&self) -> bool { self.attributes.contains(&IFunctionAttributeT::UserFunction) }
/*
  def isUserFunction = attributes.contains(UserFunctionT)
*/
/*
//  def getAbstractInterface: Option[InterfaceTT] = toBanner.getAbstractInterface
*/
/*
////  def getOverride: Option[(StructTT, InterfaceTT)] = toBanner.getOverride
*/
/*
//  def getVirtualIndex: Option[Int] = toBanner.getVirtualIndex

*/
/*
//  def toSignature(interner: Interner, keywords: Keywords): SignatureT = {
//    val newLastStep = templateName.last.makeFunctionName(interner, keywords, templateArgs, params)
//    val fullName = FullNameT(templateName.packageCoord, name.initSteps, newLastStep)
//
//    SignatureT(fullName)
//
//  }
*/
/*
//  def paramTypes: Vector[CoordT] = params.map(_.tyype)
*/
    pub fn get_abstract_interface(&self) -> Option<InterfaceTT<'s, 't>> {
        let abstract_interfaces: Vec<InterfaceTT<'s, 't>> =
            self.params.iter().filter_map(|param| {
                match param {
                    ParameterT { virtuality: Some(AbstractT), tyype: CoordT { kind: KindT::Interface(ir), .. }, .. } => Some(**ir),
                    _ => None,
                }
            }).collect();
        assert!(abstract_interfaces.len() <= 1);
        abstract_interfaces.into_iter().next()
    }
/*
  def getAbstractInterface: Option[InterfaceTT] = {
    val abstractInterfaces =
      params.collect({
        case ParameterT(_, Some(AbstractT()), _, CoordT(_, _, ir @ InterfaceTT(_))) => ir
      })
    vassert(abstractInterfaces.size <= 1)
    abstractInterfaces.headOption
  }
*/
    pub fn get_virtual_index(&self) -> Option<usize> {
        let indices: Vec<usize> = self.params.iter().enumerate()
            .filter_map(|(index, p)| if p.virtuality.is_some() { Some(index) } else { None })
            .collect();
        assert!(indices.len() <= 1);
        indices.into_iter().next()
    }
/*
  def getVirtualIndex: Option[Int] = {
    val indices =
      params.zipWithIndex.collect({
        case (ParameterT(_, Some(AbstractT()), _, _), index) => index
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
    pub fn to_banner(&self) -> FunctionBannerT<'s, 't> {
        FunctionBannerT { origin_function_templata: self.maybe_origin_function_templata, name: self.id }
    }
/*
  def toBanner: FunctionBannerT = FunctionBannerT(maybeOriginFunctionTemplata, id)
*/
    pub fn to_prototype(&self) -> PrototypeT<'s, 't> {
        PrototypeT { id: self.id, return_type: self.return_type }
    }
/*
  def toPrototype: PrototypeT[IFunctionNameT] = {
//    val substituter = TemplataCompiler.getPlaceholderSubstituter(interner, fullName, templateArgs)
//    val paramTypes = params.map(_.tyype).map(substituter.substituteForCoord)
//    val newLastStep = fullName.last.makeFunctionName(interner, keywords, templateArgs, paramTypes)
//    val newName = FullNameT(fullName.packageCoord, fullName.initSteps, newLastStep)
    PrototypeT(id, returnType)
  }
*/
    pub fn to_signature(&self) -> SignatureT<'s, 't> {
        self.to_prototype().to_signature()
    }
/*
  def toSignature: SignatureT = {
    toPrototype.toSignature
  }
*/
    fn param_types(&self) -> Vec<CoordT<'s, 't>> { panic!("Unimplemented: param_types"); }
/*
  def paramTypes: Vector[CoordT] = id.localName.parameters
*/
}
fn function_header_unapply<'a, 's, 't>(arg: &'a FunctionHeaderT<'s, 't>) -> Option<(&'a IdT<'s, 't>, &'a Vec<ParameterT<'s, 't>>, &'a CoordT<'s, 't>)> { panic!("Unimplemented: unapply"); }
/*
  def unapply(arg: FunctionHeaderT): Option[(IdT[IFunctionNameT], Vector[ParameterT], CoordT)] = {
    Some(id, params, returnType)
  }
*/
impl<'s, 't> FunctionHeaderT<'s, 't> {
    fn is_pure(&self) -> bool { panic!("Unimplemented: is_pure"); }
/*
  def isPure: Boolean = {
    attributes.collectFirst({ case PureT => }).nonEmpty
  }
}
*/
}
// Monomorphic per `docs/reasoning/idt-typed-view-alternatives.md` (same
// treatment as IdT). Scala's `PrototypeT[+T <: IFunctionNameT]` phantom
// parameter is erased in Rust.
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrototypeT<'s, 't>
where 's: 't,
{
    pub id: IdT<'s, 't>,
    pub return_type: CoordT<'s, 't>,
}
/*
case class PrototypeT[+T <: IFunctionNameT](
    id: IdT[T],
    returnType: CoordT) {
*/
impl<'s, 't> PrototypeT<'s, 't> where 's: 't, {
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
    pub fn param_types(&self) -> &'t [CoordT<'s, 't>] {
        IFunctionNameT::try_from(self.id.local_name)
            .unwrap_or_else(|_| panic!("param_types called on non-function name: {:?}", self.id.local_name))
            .parameters()
    }
/*
  def paramTypes: Vector[CoordT] = id.localName.parameters
*/
    pub fn to_signature(&self) -> SignatureT<'s, 't> {
        SignatureT { id: self.id }
    }
/*
  def toSignature: SignatureT = SignatureT(id)
}
*/
}

// (no scala counterpart — Rust-only interning scaffolding)
// Transient Val for interning: inner IdValT borrows its init_steps slice from
// a stack-local builder via 'tmp, so construction doesn't arena-allocate.
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct PrototypeValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub id: IdValT<'s, 't, 'tmp>,
    pub return_type: CoordT<'s, 't>,
}

/// Interning transient (see @TFITCX)
pub struct PrototypeValQuery<'a, 's, 't, 'tmp>(pub &'a PrototypeValT<'s, 't, 'tmp>)
where 's: 't, 't: 'tmp;

impl<'a, 's, 't, 'tmp> Hash for PrototypeValQuery<'a, 's, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    fn hash<H: Hasher>(&self, state: &mut H) { self.0.hash(state); }
    /* Guardian: disable-all */
}

impl<'a, 's, 't, 'tmp> hashbrown::Equivalent<PrototypeValT<'s, 't, 't>> for PrototypeValQuery<'a, 's, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    fn equivalent(&self, key: &PrototypeValT<'s, 't, 't>) -> bool {
        IdValQuery(&self.0.id).equivalent(&key.id)
            && self.0.return_type == key.return_type
    }
    /* Guardian: disable-all */
}
