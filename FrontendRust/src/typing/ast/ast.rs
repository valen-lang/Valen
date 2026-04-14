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
// mig: struct ImplT
pub struct ImplT<'s> {
    pub templata: ImplDefinitionTemplataT<'s>,
    pub instantiated_id: IdT<IImplNameT>,
    pub template_id: IdT<IImplTemplateNameT>,
    pub sub_citizen_template_id: IdT<ICitizenTemplateNameT>,
    pub sub_citizen: ICitizenTT<'s>,
    pub super_interface: InterfaceTT<'s>,
    pub super_interface_template_id: IdT<IInterfaceTemplateNameT>,
    pub instantiation_bound_params: InstantiationBoundArgumentsT<FunctionBoundNameT, ImplBoundNameT>,
    pub rune_index_to_independence: Vec<bool>,
}
// mig: impl ImplT
impl<'s> ImplT<'s> {}
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
// mig: struct KindExportT
pub struct KindExportT<'p> {
    pub range: RangeS<'p>,
    pub tyype: KindT<'p>,
    pub id: IdT<ExportNameT>,
    pub exported_name: StrI<'p>,
}
// mig: impl KindExportT
impl<'p> KindExportT<'p> {}
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
// mig: fn equals
fn equals(&self, obj: &KindExportT) -> bool {
    panic!("Unimplemented: equals");
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  override def hashCode(): Int = vcurious()

}
*/
// mig: struct FunctionExportT
pub struct FunctionExportT<'s> {
    pub range: RangeS<'s>,
    pub prototype: PrototypeT<IFunctionNameT>,
    pub export_id: IdT<ExportNameT>,
    pub exported_name: StrI<'s>,
}
// mig: impl FunctionExportT
impl<'s> FunctionExportT<'s> {}
/*
case class FunctionExportT(
  range: RangeS,
  prototype: PrototypeT[IFunctionNameT],
  exportId: IdT[ExportNameT],
  exportedName: StrI
)  {
*/
// mig: fn equals
fn equals(&self, obj: &FunctionExportT) -> bool {
    panic!("Unimplemented: equals");
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  override def hashCode(): Int = vcurious()
  vpass()
}
*/
// mig: struct KindExternT
pub struct KindExternT<'p> {
    pub tyype: KindT<'p>,
    pub package_coordinate: PackageCoordinate,
    pub extern_name: StrI<'p>,
}
// mig: impl KindExternT
impl<'p> KindExternT<'p> {}
/*
case class KindExternT(
  tyype: KindT,
  packageCoordinate: PackageCoordinate,
  externName: StrI
)  {
*/
// mig: fn equals
fn equals(&self, obj: &KindExternT) -> bool {
    panic!("Unimplemented: equals");
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  override def hashCode(): Int = vcurious()

}
*/
// mig: struct FunctionExternT
pub struct FunctionExternT<'s> {
    pub range: RangeS<'s>,
    pub extern_placeholdered_id: IdT<ExternNameT>,
    pub prototype: PrototypeT<IFunctionNameT>,
    pub extern_name: StrI<'s>,
}
// mig: impl FunctionExternT
impl<'s> FunctionExternT<'s> {}
/*
case class FunctionExternT(
  range: RangeS,
  externPlaceholderedId: IdT[ExternNameT],
  prototype: PrototypeT[IFunctionNameT],
  externName: StrI
)  {
*/
// mig: fn equals
fn equals(&self, obj: &FunctionExternT) -> bool {
    panic!("Unimplemented: equals");
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  override def hashCode(): Int = vcurious()

}
*/
// mig: struct InterfaceEdgeBlueprintT
pub struct InterfaceEdgeBlueprintT {
    pub interface: IdT<IInterfaceNameT>,
    pub super_family_root_headers: Vec<(PrototypeT<IFunctionNameT>, i32)>,
}
// mig: impl InterfaceEdgeBlueprintT
impl InterfaceEdgeBlueprintT {}
/*
case class InterfaceEdgeBlueprintT(
  // The typing pass keys this by placeholdered name, and the instantiator keys this by non-placeholdered names
  interface: IdT[IInterfaceNameT],
  superFamilyRootHeaders: Vector[(PrototypeT[IFunctionNameT], Int)]) {
    val hash = runtime.ScalaRunTime._hashCode(this);
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  override def hashCode(): Int = hash;
*/
// mig: fn equals
fn equals(&self, obj: &InterfaceEdgeBlueprintT) -> bool {
    panic!("Unimplemented: equals");
}
/*
  override def equals(obj: Any): Boolean = vcurious(); }
*/
// mig: struct OverrideT
pub struct OverrideT<'s> {
    pub dispatcher_call_id: IdT<OverrideDispatcherNameT>,
    pub impl_placeholder_to_dispatcher_placeholder: Vec<(IdT<IPlaceholderNameT>, ITemplataT<ITemplataType>)>,
    pub impl_placeholder_to_case_placeholder: Vec<(IdT<IPlaceholderNameT>, ITemplataT<ITemplataType>)>,
    pub dispatcher_and_case_placeholdered_impl_reachable_prototypes: HashMap<IRuneS<'s>, HashMap<IRuneS<'s>, PrototypeT<FunctionBoundNameT>>>,
    pub case_id: IdT<OverrideDispatcherCaseNameT>,
    pub override_prototype: PrototypeT<IFunctionNameT>,
    pub dispatcher_instantiation_bound_params: InstantiationBoundArgumentsT<FunctionBoundNameT, ImplBoundNameT>,
}
// mig: impl OverrideT
impl<'s> OverrideT<'s> {}
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
// mig: struct EdgeT
pub struct EdgeT<'s> {
    pub edge_id: IdT<IImplNameT>,
    pub sub_citizen: ICitizenTT<'s>,
    pub super_interface: IdT<IInterfaceNameT>,
    pub instantiation_bound_params: InstantiationBoundArgumentsT<FunctionBoundNameT, ImplBoundNameT>,
    pub abstract_func_to_override_func: HashMap<IdT<IFunctionNameT>, OverrideT<'s>>,
}
// mig: impl EdgeT
impl<'s> EdgeT<'s> {}
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
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
// mig: fn equals
fn equals(&self, obj: &EdgeT) -> bool {
    panic!("Unimplemented: equals");
}
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
// mig: struct FunctionDefinitionT
pub struct FunctionDefinitionT<'s> {
    pub header: FunctionHeaderT,
    pub instantiation_bound_params: InstantiationBoundArgumentsT<FunctionBoundNameT, ImplBoundNameT>,
    pub body: ReferenceExpressionTE<'s>,
}
// mig: impl FunctionDefinitionT
impl<'s> FunctionDefinitionT<'s> {}
/*
case class FunctionDefinitionT(
  header: FunctionHeaderT,
  instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT],
  body: ReferenceExpressionTE)  {
*/
// mig: fn equals
fn equals(&self, obj: &FunctionDefinitionT) -> bool {
    panic!("Unimplemented: equals");
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  override def hashCode(): Int = vcurious()

  // We always end a function with a ret, whose result is a Never.
  vassert(body.result.kind == NeverT(false))
*/
// mig: fn is_pure
fn is_pure(&self) -> bool {
    panic!("Unimplemented: is_pure");
}
/*
  def isPure: Boolean = header.isPure
}

object getFunctionLastName {
*/
// mig: fn unapply
fn unapply(f: &FunctionDefinitionT) -> Option<&IFunctionNameT> {
    panic!("Unimplemented: unapply");
}
/*
  def unapply(f: FunctionDefinitionT): Option[IFunctionNameT] = Some(f.header.id.localName)
}
*/
// mig: struct LocationInFunctionEnvironmentT
pub struct LocationInFunctionEnvironmentT {
    pub path: Vec<i32>,
}
// mig: impl LocationInFunctionEnvironmentT
impl LocationInFunctionEnvironmentT {}
/*
// A unique location in a function. Environment is in the name so it spells LIFE!
case class LocationInFunctionEnvironmentT(path: Vector[Int]) {
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
// mig: fn add
fn add(&self, sub_location: i32) -> LocationInFunctionEnvironmentT {
    panic!("Unimplemented: add");
}
/*
  def +(subLocation: Int): LocationInFunctionEnvironmentT = {
    LocationInFunctionEnvironmentT(path :+ subLocation)
  }
*/
// mig: fn to_string
fn to_string(&self) -> String {
    panic!("Unimplemented: to_string");
}
/*
  override def toString: String = path.mkString(".")
}
*/
// mig: struct AbstractT
pub struct AbstractT;
// mig: impl AbstractT
impl AbstractT {}
/*
case class AbstractT()
*/
// mig: struct ParameterT
pub struct ParameterT {
    pub name: IVarNameT,
    pub virtuality: Option<AbstractT>,
    pub pre_checked: bool,
    pub tyype: CoordT,
}
// mig: impl ParameterT
impl ParameterT {}
/*
case class ParameterT(
  name: IVarNameT,
  virtuality: Option[AbstractT],
  preChecked: Boolean,
  tyype: CoordT)  {
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

  // Use same instead, see EHCFBD for why we dont like equals.
*/
// mig: fn equals
fn equals(&self, obj: &ParameterT) -> bool {
    panic!("Unimplemented: equals");
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn same
fn same(&self, that: &ParameterT) -> bool {
    panic!("Unimplemented: same");
}
/*
  def same(that: ParameterT): Boolean = {
    name == that.name &&
      virtuality == that.virtuality &&
      tyype == that.tyype
  }
}
*/
// mig: trait ICalleeCandidate
pub trait ICalleeCandidate {}
/*
sealed trait ICalleeCandidate
*/
// mig: struct FunctionCalleeCandidate
pub struct FunctionCalleeCandidate<'s> {
    pub ft: FunctionTemplataT<'s>,
}
// mig: impl FunctionCalleeCandidate
impl<'s> FunctionCalleeCandidate<'s> {}
/*
case class FunctionCalleeCandidate(ft: FunctionTemplataT) extends ICalleeCandidate {
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
*/
// mig: struct HeaderCalleeCandidate
pub struct HeaderCalleeCandidate {
    pub header: FunctionHeaderT,
}
// mig: impl HeaderCalleeCandidate
impl HeaderCalleeCandidate {}
/*
case class HeaderCalleeCandidate(header: FunctionHeaderT) extends ICalleeCandidate {
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
*/
// mig: struct PrototypeTemplataCalleeCandidate
pub struct PrototypeTemplataCalleeCandidate {
    pub prototype_t: PrototypeT<IFunctionNameT>,
}
// mig: impl PrototypeTemplataCalleeCandidate
impl PrototypeTemplataCalleeCandidate {}
/*
case class PrototypeTemplataCalleeCandidate(
  // We don't want a range because we want to merge all sorts of different bound functions, see MFBFDP.
  //   range: RangeS,
  prototypeT: PrototypeT[IFunctionNameT]) extends ICalleeCandidate {
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}

//sealed trait IValidCalleeCandidate {
//  def range: Option[RangeS]
//  def paramTypes: Vector[CoordT]
//}
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
// mig: struct SignatureT
pub struct SignatureT {
    pub id: IdT<IFunctionNameT>,
}
// mig: impl SignatureT
impl SignatureT {}
/*
case class SignatureT(id: IdT[IFunctionNameT]) {
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
// mig: fn param_types
fn param_types(&self) -> Vec<CoordT> {
    panic!("Unimplemented: param_types");
}
/*
  def paramTypes: Vector[CoordT] = id.localName.parameters
}
*/
// mig: struct FunctionBannerT
pub struct FunctionBannerT<'s> {
    pub origin_function_templata: Option<FunctionTemplataT<'s>>,
    pub name: IdT<IFunctionNameT>,
}
// mig: impl FunctionBannerT
impl<'s> FunctionBannerT<'s> {}
/*
case class FunctionBannerT(
  originFunctionTemplata: Option[FunctionTemplataT],
  name: IdT[IFunctionNameT])   {
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

  // Use same instead, see EHCFBD for why we dont like equals.
*/
// mig: fn equals
fn equals(&self, obj: &FunctionBannerT) -> bool {
    panic!("Unimplemented: equals");
}
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn same
fn same(&self, that: &FunctionBannerT) -> bool {
    panic!("Unimplemented: same");
}
/*
  def same(that: FunctionBannerT): Boolean = {
    originFunctionTemplata.map(_.function) == that.originFunctionTemplata.map(_.function) && name == that.name
  }



//  def unapply(arg: FunctionBannerT):
//  Option[(FullNameT[IFunctionNameT], Vector[ParameterT])] =
//    Some(templateName, params)
*/
// mig: fn to_string
fn to_string(&self) -> String {
    panic!("Unimplemented: to_string");
}
/*
  override def toString: String = {
    // # is to signal that we override this
//    "FunctionBanner2#(" + templateName + ")"
//        "FunctionBanner2#(" + templateName + ", " + params + ")"
    "FunctionBanner2#(" + name + ")"
  }
}
*/
// mig: trait IFunctionAttributeT
pub trait IFunctionAttributeT {}
/*
sealed trait IFunctionAttributeT
*/
// mig: trait ICitizenAttributeT
pub trait ICitizenAttributeT {}
/*
sealed trait ICitizenAttributeT
*/
// mig: struct ExternT
pub struct ExternT {
    pub package_coord: PackageCoordinate,
}
// mig: impl ExternT
impl ExternT {}
/*
case class ExternT(packageCoord: PackageCoordinate) extends IFunctionAttributeT with ICitizenAttributeT { // For optimization later
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
// There's no Export2 here, we use separate KindExport and FunctionExport constructs.
//case class Export2(packageCoord: PackageCoordinate) extends IFunctionAttribute2 with ICitizenAttribute2
case object PureT extends IFunctionAttributeT
case object AdditiveT extends IFunctionAttributeT
case object SealedT extends ICitizenAttributeT
case object UserFunctionT extends IFunctionAttributeT // Whether it was written by a human. Mostly for tests right now.
*/
// mig: struct FunctionHeaderT
pub struct FunctionHeaderT {
    pub id: IdT<IFunctionNameT>,
    pub attributes: Vec<Box<dyn IFunctionAttributeT>>,
    pub params: Vec<ParameterT>,
    pub return_type: CoordT,
    pub maybe_origin_function_templata: Option<FunctionTemplataT>,
}
// mig: impl FunctionHeaderT
impl FunctionHeaderT {}
/*
case class FunctionHeaderT(
  // This one little name field can illuminate much of how the compiler works, see UINIT.
  id: IdT[IFunctionNameT],
  attributes: Vector[IFunctionAttributeT],
  params: Vector[ParameterT],
  returnType: CoordT,
  maybeOriginFunctionTemplata: Option[FunctionTemplataT]) {
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

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
// mig: fn equals
fn equals(&self, obj: &FunctionHeaderT) -> bool {
    panic!("Unimplemented: equals");
}
/*
  override def equals(obj: Any): Boolean = {
    obj match {
      case FunctionHeaderT(thatName, _, _, _, _) => {
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
fn is_extern(&self) -> bool {
    panic!("Unimplemented: is_extern");
}
/*
  def isExtern = attributes.exists({ case ExternT(_) => true case _ => false })
  //  def isExport = attributes.exists({ case Export2(_) => true case _ => false })
*/
// mig: fn is_user_function
fn is_user_function(&self) -> bool {
    panic!("Unimplemented: is_user_function");
}
/*
  def isUserFunction = attributes.contains(UserFunctionT)
//  def getAbstractInterface: Option[InterfaceTT] = toBanner.getAbstractInterface
////  def getOverride: Option[(StructTT, InterfaceTT)] = toBanner.getOverride
//  def getVirtualIndex: Option[Int] = toBanner.getVirtualIndex

//  def toSignature(interner: Interner, keywords: Keywords): SignatureT = {
//    val newLastStep = templateName.last.makeFunctionName(interner, keywords, templateArgs, params)
//    val fullName = FullNameT(templateName.packageCoord, name.initSteps, newLastStep)
//
//    SignatureT(fullName)
//
//  }
//  def paramTypes: Vector[CoordT] = params.map(_.tyype)
*/
// mig: fn get_abstract_interface
fn get_abstract_interface(&self) -> Option<&InterfaceTT> {
    panic!("Unimplemented: get_abstract_interface");
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
// mig: fn get_virtual_index
fn get_virtual_index(&self) -> Option<i32> {
    panic!("Unimplemented: get_virtual_index");
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
// mig: fn to_banner
fn to_banner(&self) -> FunctionBannerT {
    panic!("Unimplemented: to_banner");
}
/*
  def toBanner: FunctionBannerT = FunctionBannerT(maybeOriginFunctionTemplata, id)
*/
// mig: fn to_prototype
fn to_prototype(&self) -> PrototypeT<IFunctionNameT> {
    panic!("Unimplemented: to_prototype");
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
// mig: fn to_signature
fn to_signature(&self) -> SignatureT {
    panic!("Unimplemented: to_signature");
}
/*
  def toSignature: SignatureT = {
    toPrototype.toSignature
  }
*/
// mig: fn param_types
fn param_types(&self) -> Vec<CoordT> {
    panic!("Unimplemented: param_types");
}
/*
  def paramTypes: Vector[CoordT] = id.localName.parameters
*/
// mig: fn unapply
fn unapply(arg: &FunctionHeaderT) -> Option<(&IdT<IFunctionNameT>, &Vec<ParameterT>, &CoordT)> {
    panic!("Unimplemented: unapply");
}
/*
  def unapply(arg: FunctionHeaderT): Option[(IdT[IFunctionNameT], Vector[ParameterT], CoordT)] = {
    Some(id, params, returnType)
  }
*/
// mig: fn is_pure
fn is_pure(&self) -> bool {
    panic!("Unimplemented: is_pure");
}
/*
  def isPure: Boolean = {
    attributes.collectFirst({ case PureT => }).nonEmpty
  }
}
*/
// mig: struct PrototypeT
pub struct PrototypeT<T: IFunctionNameT> {
    pub id: IdT<T>,
    pub return_type: CoordT,
}
// mig: impl PrototypeT
impl<T: IFunctionNameT> PrototypeT<T> {}
/*
case class PrototypeT[+T <: IFunctionNameT](
    id: IdT[T],
    returnType: CoordT) {
*/
// mig: fn hash_code
fn hash_code(&self) -> i32 {
    panic!("Unimplemented: hash_code");
}
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
*/
// mig: fn param_types
fn param_types(&self) -> Vec<CoordT> {
    panic!("Unimplemented: param_types");
}
/*
  def paramTypes: Vector[CoordT] = id.localName.parameters
*/
// mig: fn to_signature
fn to_signature(&self) -> SignatureT {
    panic!("Unimplemented: to_signature");
}
/*
  def toSignature: SignatureT = SignatureT(id)
}
*/