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
pub struct ImplT<'s, 't> {
    pub templata: ImplDefinitionTemplataT<'s, 't>,
    pub instantiated_id: IdT<'s, 't>,
    pub template_id: IdT<'s, 't>,
    pub sub_citizen_template_id: IdT<'s, 't>,
    pub sub_citizen: ICitizenTT<'s, 't>,
    pub super_interface: InterfaceTT<'s, 't>,
    pub super_interface_template_id: IdT<'s, 't>,
    pub instantiation_bound_params: InstantiationBoundArgumentsT<'s, 't>,
    pub rune_index_to_independence: Vec<bool>,
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
    fn equals(&self, obj: &KindExportT<'s, 't>) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> KindExportT<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()

}
*/
}
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
    fn equals(&self, obj: &FunctionExportT<'s, 't>) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> FunctionExportT<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()
  vpass()
}
*/
}
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
    fn equals(&self, obj: &KindExternT<'s, 't>) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> KindExternT<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()

}
*/
}
pub struct FunctionExternT<'s, 't> {
    pub range: RangeS<'s>,
    pub extern_placeholdered_id: IdT<'s, 't>,
    pub prototype: PrototypeT<'s, 't>,
    pub extern_name: StrI<'s>,
}
/*
case class FunctionExternT(
  range: RangeS,
  externPlaceholderedId: IdT[ExternNameT],
  prototype: PrototypeT[IFunctionNameT],
  externName: StrI
)  {
*/
impl<'s, 't> FunctionExternT<'s, 't> {
    fn equals(&self, obj: &FunctionExternT<'s, 't>) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> FunctionExternT<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()

}
*/
}
pub struct InterfaceEdgeBlueprintT<'s, 't> {
    pub interface: IdT<'s, 't>,
    pub super_family_root_headers: Vec<(PrototypeT<'s, 't>, i32)>,
}
/*
case class InterfaceEdgeBlueprintT(
  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  interface: IdT[IInterfaceNameT],
  superFamilyRootHeaders: Vector[(PrototypeT[IFunctionNameT], Int)]) {
    val hash = runtime.ScalaRunTime._hashCode(this);
*/
impl<'s, 't> InterfaceEdgeBlueprintT<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = hash;
*/
}
impl<'s, 't> InterfaceEdgeBlueprintT<'s, 't> {
    fn equals(&self, obj: &InterfaceEdgeBlueprintT<'s, 't>) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious(); }
*/
}
pub struct OverrideT<'s, 't> {
    pub dispatcher_call_id: IdT<'s, 't>,
    pub impl_placeholder_to_dispatcher_placeholder: Vec<(IdT<'s, 't>, ITemplataT<'s, 't>)>,
    pub impl_placeholder_to_case_placeholder: Vec<(IdT<'s, 't>, ITemplataT<'s, 't>)>,
    pub dispatcher_and_case_placeholdered_impl_reachable_prototypes: HashMap<IRuneS<'s>, HashMap<IRuneS<'s>, PrototypeT<'s, 't>>>,
    pub case_id: IdT<'s, 't>,
    pub override_prototype: PrototypeT<'s, 't>,
    pub dispatcher_instantiation_bound_params: InstantiationBoundArgumentsT<'s, 't>,
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
pub struct EdgeT<'s, 't> {
    pub edge_id: IdT<'s, 't>,
    pub sub_citizen: ICitizenTT<'s, 't>,
    pub super_interface: IdT<'s, 't>,
    pub instantiation_bound_params: InstantiationBoundArgumentsT<'s, 't>,
    pub abstract_func_to_override_func: HashMap<IdT<'s, 't>, OverrideT<'s, 't>>,
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
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
}
impl<'s, 't> EdgeT<'s, 't> {
    fn equals(&self, obj: &EdgeT<'s, 't>) -> bool { panic!("Unimplemented: equals"); }
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
pub struct FunctionDefinitionT<'s, 't> {
    pub header: FunctionHeaderT<'s, 't>,
    pub instantiation_bound_params: InstantiationBoundArgumentsT<'s, 't>,
    pub body: ReferenceExpressionTE<'s, 't>,
}
/*
case class FunctionDefinitionT(
  header: FunctionHeaderT,
  instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT],
  body: ReferenceExpressionTE)  {
*/
impl<'s, 't> FunctionDefinitionT<'s, 't> {
    fn equals(&self, obj: &FunctionDefinitionT<'s, 't>) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> FunctionDefinitionT<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  override def hashCode(): Int = vcurious()

*/
}
impl<'s, 't> FunctionDefinitionT<'s, 't> where 's: 't, {
    fn new(
        header: FunctionHeaderT<'s, 't>,
        instantiation_bound_params: InstantiationBoundArgumentsT<'s, 't>,
        body: &'t ReferenceExpressionTE<'s, 't>,
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
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct LocationInFunctionEnvironmentT<'s> {
    pub path: Vec<i32>,
    pub _phantom: std::marker::PhantomData<&'s ()>,
}
/*
// A unique location in a function. Environment is in the name so it spells LIFE!
case class LocationInFunctionEnvironmentT(path: Vector[Int]) {
*/
impl<'s> LocationInFunctionEnvironmentT<'s> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
}
impl<'s> LocationInFunctionEnvironmentT<'s> {
    fn add(&self, sub_location: i32) -> LocationInFunctionEnvironmentT<'s> { panic!("Unimplemented: add"); }
/*
  def +(subLocation: Int): LocationInFunctionEnvironmentT = {
    LocationInFunctionEnvironmentT(path :+ subLocation)
  }
*/
}
impl<'s> LocationInFunctionEnvironmentT<'s> {
    fn to_string(&self) -> String { panic!("Unimplemented: to_string"); }
/*
  override def toString: String = path.mkString(".")
}
*/
}
pub struct AbstractT;
/*
case class AbstractT()
*/
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
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

  // Use same instead, see EHCFBD for why we dont like equals.
*/
}
impl<'s, 't> ParameterT<'s, 't> {
    fn equals(&self, obj: &ParameterT<'s, 't>) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> ParameterT<'s, 't> {
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
pub enum ICalleeCandidate<'s, 't> {
    Function(FunctionCalleeCandidate<'s, 't>),
    Header(&'t HeaderCalleeCandidate<'s, 't>),
    PrototypeTemplata(PrototypeTemplataCalleeCandidate<'s, 't>),
}
/*
sealed trait ICalleeCandidate
*/
pub struct FunctionCalleeCandidate<'s, 't> {
    pub ft: FunctionTemplataT<'s, 't>,
}
/*
case class FunctionCalleeCandidate(ft: FunctionTemplataT) extends ICalleeCandidate {
*/
impl<'s, 't> FunctionCalleeCandidate<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
*/
}
pub struct HeaderCalleeCandidate<'s, 't> {
    pub header: FunctionHeaderT<'s, 't>,
}
/*
case class HeaderCalleeCandidate(header: FunctionHeaderT) extends ICalleeCandidate {
*/
impl<'s, 't> HeaderCalleeCandidate<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
*/
}
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
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
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
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious();
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
override def hashCode(): Int = hash;
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
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious();
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SignatureT<'s, 't> {
    pub id: IdT<'s, 't>,
}
/*
case class SignatureT(id: IdT[IFunctionNameT]) {
*/
impl<'s, 't> SignatureT<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
}
impl<'s, 't> SignatureT<'s, 't> {
    fn param_types(&self) -> Vec<CoordT<'s, 't>> { panic!("Unimplemented: param_types"); }
/*
  def paramTypes: Vector[CoordT] = id.localName.parameters
}
*/
}

// (no scala counterpart — Rust-only interning scaffolding)
// Transient Val for interning: holds a stack-borrowed IdValT<'s, 't, 'tmp> so
// callers can construct a lookup key without first arena-allocating init_steps.
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct SignatureValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub id: IdValT<'s, 't, 'tmp>,
}

pub struct SignatureValQuery<'a, 's, 't, 'tmp>(pub &'a SignatureValT<'s, 't, 'tmp>)
where 's: 't, 't: 'tmp;

impl<'a, 's, 't, 'tmp> std::hash::Hash for SignatureValQuery<'a, 's, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.0.hash(state); }
}

impl<'a, 's, 't, 'tmp> hashbrown::Equivalent<SignatureValT<'s, 't, 't>> for SignatureValQuery<'a, 's, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    fn equivalent(&self, key: &SignatureValT<'s, 't, 't>) -> bool {
        crate::typing::names::names::IdValQuery(&self.0.id).equivalent(&key.id)
    }
}
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
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

  // Use same instead, see EHCFBD for why we dont like equals.
*/
}
impl<'s, 't> FunctionBannerT<'s, 't> {
    fn equals(&self, obj: &FunctionBannerT<'s, 't>) -> bool { panic!("Unimplemented: equals"); }
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
}
impl<'s, 't> FunctionBannerT<'s, 't> {
    fn same(&self, that: &FunctionBannerT<'s, 't>) -> bool { panic!("Unimplemented: same"); }
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
}
impl<'s, 't> FunctionBannerT<'s, 't> {
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
pub enum IFunctionAttributeT<'s> {
    Extern(ExternT<'s>),
    Pure,
    Additive,
    UserFunction,
}
/*
sealed trait IFunctionAttributeT
*/
pub enum ICitizenAttributeT<'s> {
    Extern(ExternT<'s>),
    Sealed,
}
/*
sealed trait ICitizenAttributeT
*/
pub struct ExternT<'s> {
    pub package_coord: PackageCoordinate<'s>,
}
/*
case class ExternT(packageCoord: PackageCoordinate) extends IFunctionAttributeT with ICitizenAttributeT { // For optimization later
*/
impl<'s> ExternT<'s> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
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
pub struct FunctionHeaderT<'s, 't> {
    pub id: IdT<'s, 't>,
    pub attributes: Vec<IFunctionAttributeT<'s>>,
    pub params: Vec<ParameterT<'s, 't>>,
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
impl<'s, 't> FunctionHeaderT<'s, 't> {
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
}
impl<'s, 't> FunctionHeaderT<'s, 't> {
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
}
impl<'s, 't> FunctionHeaderT<'s, 't> {
    fn equals(&self, obj: &FunctionHeaderT<'s, 't>) -> bool { panic!("Unimplemented: equals"); }
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
}
impl<'s, 't> FunctionHeaderT<'s, 't> {
    fn is_extern(&self) -> bool { panic!("Unimplemented: is_extern"); }
/*
  def isExtern = attributes.exists({ case ExternT(_) => true case _ => false })
*/
/*
  //  def isExport = attributes.exists({ case Export2(_) => true case _ => false })
*/
}
impl<'s, 't> FunctionHeaderT<'s, 't> {
    fn is_user_function(&self) -> bool { panic!("Unimplemented: is_user_function"); }
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
}
impl<'s, 't> FunctionHeaderT<'s, 't> {
    fn get_abstract_interface(&self) -> Option<&InterfaceTT<'s, 't>> { panic!("Unimplemented: get_abstract_interface"); }
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
}
impl<'s, 't> FunctionHeaderT<'s, 't> {
    fn get_virtual_index(&self) -> Option<i32> { panic!("Unimplemented: get_virtual_index"); }
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
}
impl<'s, 't> FunctionHeaderT<'s, 't> {
    fn to_banner(&self) -> FunctionBannerT<'s, 't> { panic!("Unimplemented: to_banner"); }
/*
  def toBanner: FunctionBannerT = FunctionBannerT(maybeOriginFunctionTemplata, id)
*/
}
impl<'s, 't> FunctionHeaderT<'s, 't> {
    fn to_prototype(&self) -> PrototypeT<'s, 't> { panic!("Unimplemented: to_prototype"); }
/*
  def toPrototype: PrototypeT[IFunctionNameT] = {
//    val substituter = TemplataCompiler.getPlaceholderSubstituter(interner, fullName, templateArgs)
//    val paramTypes = params.map(_.tyype).map(substituter.substituteForCoord)
//    val newLastStep = fullName.last.makeFunctionName(interner, keywords, templateArgs, paramTypes)
//    val newName = FullNameT(fullName.packageCoord, fullName.initSteps, newLastStep)
    PrototypeT(id, returnType)
  }
*/
}
impl<'s, 't> FunctionHeaderT<'s, 't> {
    fn to_signature(&self) -> SignatureT<'s, 't> { panic!("Unimplemented: to_signature"); }
/*
  def toSignature: SignatureT = {
    toPrototype.toSignature
  }
*/
}
impl<'s, 't> FunctionHeaderT<'s, 't> {
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
    fn hash_code(&self) -> i32 { panic!("Unimplemented: hash_code"); }
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
}
impl<'s, 't> PrototypeT<'s, 't> where 's: 't, {
    fn param_types(&self) -> Vec<CoordT<'s, 't>> { panic!("Unimplemented: param_types"); }
/*
  def paramTypes: Vector[CoordT] = id.localName.parameters
*/
}
impl<'s, 't> PrototypeT<'s, 't> where 's: 't, {
    fn to_signature(&self) -> SignatureT<'s, 't> { panic!("Unimplemented: to_signature"); }
/*
  def toSignature: SignatureT = SignatureT(id)
}
*/
}

// (no scala counterpart — Rust-only interning scaffolding)
// Transient Val for interning: inner IdValT borrows its init_steps slice from
// a stack-local builder via 'tmp, so construction doesn't arena-allocate.
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub struct PrototypeValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    pub id: IdValT<'s, 't, 'tmp>,
    pub return_type: CoordT<'s, 't>,
}

pub struct PrototypeValQuery<'a, 's, 't, 'tmp>(pub &'a PrototypeValT<'s, 't, 'tmp>)
where 's: 't, 't: 'tmp;

impl<'a, 's, 't, 'tmp> std::hash::Hash for PrototypeValQuery<'a, 's, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) { self.0.hash(state); }
}

impl<'a, 's, 't, 'tmp> hashbrown::Equivalent<PrototypeValT<'s, 't, 't>> for PrototypeValQuery<'a, 's, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
    fn equivalent(&self, key: &PrototypeValT<'s, 't, 't>) -> bool {
        crate::typing::names::names::IdValQuery(&self.0.id).equivalent(&key.id)
            && self.0.return_type == key.return_type
    }
}
