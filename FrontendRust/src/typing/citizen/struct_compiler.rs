/*
package dev.vale.typing.citizen

import dev.vale.highertyping.FunctionA
import dev.vale._
import dev.vale.postparsing._
import dev.vale.postparsing.rules.IRulexSR
import dev.vale.typing.ast.{FunctionHeaderT, PrototypeT}
import dev.vale.typing.env.IInDenizenEnvironmentT
import dev.vale.typing._
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.highertyping._
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale.parsing._
import dev.vale.postparsing.patterns.AtomSP
import dev.vale.postparsing.rules._
import dev.vale.typing.env._
import dev.vale.typing.function._
import dev.vale.typing.ast._
import dev.vale.typing.templata.ITemplataT.expectMutability

import scala.collection.immutable.List
import scala.collection.mutable
*/
use std::collections::{HashMap, HashSet};

use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::utils::range::RangeS;

use crate::postparsing::names::*;
use crate::higher_typing::ast::*;

use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::compiler_outputs::*;
use crate::typing::compilation::*;

// mig: struct WeakableImplingMismatch
pub struct WeakableImplingMismatch {
    pub struct_weakable: bool,
    pub interface_weakable: bool,
}
// mig: impl WeakableImplingMismatch
impl WeakableImplingMismatch {}
/*
case class WeakableImplingMismatch(structWeakable: Boolean, interfaceWeakable: Boolean) extends Throwable {
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
fn equals(&self, obj: &dyn std::any::Any) -> bool {
    panic!("Unimplemented: equals");
}
/*
override def equals(obj: Any): Boolean = vcurious(); }

// See ODMFRC.
*/
// mig: struct UncheckedDefiningConclusions
pub struct UncheckedDefiningConclusions<'s, 't> {
    pub envs: InferEnv<'s>,
    pub ranges: Vec<RangeS<'s>>,
    pub call_location: LocationInDenizen<'s>,
    pub definition_rules: Vec<IRulexSR<'s>>,
    pub conclusions: std::collections::HashMap<IRuneS<'s>, ITemplataT<'s, 't>>,
}
// mig: impl UncheckedDefiningConclusions
impl<'s, 't> UncheckedDefiningConclusions<'s, 't> {}
/*
case class UncheckedDefiningConclusions(
    envs: InferEnv,
    ranges: List[RangeS],
    callLocation: LocationInDenizen,
    definitionRules: Vector[IRulexSR],
    conclusions: Map[IRuneS, ITemplataT[ITemplataType]])
*/
// mig: trait IStructCompilerDelegate
pub trait IStructCompilerDelegate<'s, 't> {
/*
trait IStructCompilerDelegate {
*/
// mig: fn evaluate_generic_function_from_non_call_for_header
fn evaluate_generic_function_from_non_call_for_header(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    parent_ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    function_templata: FunctionTemplataT<'s>,
) -> FunctionHeaderT<'s, 't> {
    panic!("Unimplemented: evaluate_generic_function_from_non_call_for_header");
}
/*
  def evaluateGenericFunctionFromNonCallForHeader(
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    functionTemplata: FunctionTemplataT):
  FunctionHeaderT
*/
// mig: fn scout_expected_function_for_prototype
fn scout_expected_function_for_prototype(
    &self,
    env: &dyn IInDenizenEnvironmentT<'s, 't>,
    coutputs: &CompilerOutputs<'s, 't>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    function_name: IImpreciseNameS<'s>,
    explicit_template_arg_rules_s: &[IRulexSR<'s>],
    explicit_template_arg_runes_s: &[IRuneS<'s>],
    context_region: RegionT<'s, 't>,
    args: &[CoordT<'s, 't>],
    extra_envs_to_look_in: &[&dyn IInDenizenEnvironmentT<'s, 't>],
    exact: bool,
) -> StampFunctionSuccess<'s, 't> {
    panic!("Unimplemented: scout_expected_function_for_prototype");
}
/*
  def scoutExpectedFunctionForPrototype(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    functionName: IImpreciseNameS,
    explicitTemplateArgRulesS: Vector[IRulexSR],
    explicitTemplateArgRunesS: Vector[IRuneS],
    contextRegion: RegionT,
    args: Vector[CoordT],
    extraEnvsToLookIn: Vector[IInDenizenEnvironmentT],
    exact: Boolean):
  StampFunctionSuccess
}
*/
}

// mig: enum IResolveOutcome
pub enum IResolveOutcome<'s, 't, T: KindT<'s, 't>> {
/*
sealed trait IResolveOutcome[+T <: KindT] {
*/
// mig: fn expect
fn expect(self) -> ResolveSuccess<'s, 't, T>;
/*
  def expect(): ResolveSuccess[T]
}
*/
}

// mig: struct ResolveSuccess
pub struct ResolveSuccess<'s, 't, T: KindT<'s, 't>> {
    pub kind: T,
}
// mig: impl ResolveSuccess
impl<'s, 't, T: KindT<'s, 't>> ResolveSuccess<'s, 't, T> {
// mig: fn expect
fn expect(self) -> ResolveSuccess<'s, 't, T> {
    panic!("Unimplemented: expect");
}
/*
case class ResolveSuccess[+T <: KindT](kind: T) extends IResolveOutcome[T] {
*/
}
/*
  override def expect(): ResolveSuccess[T] = this
}
*/
// mig: struct ResolveFailure
pub struct ResolveFailure<'s, 't, T: KindT<'s, 't>> {
    pub range: Vec<RangeS<'s>>,
    pub x: IResolvingError<'s, 't>,
}
// mig: impl ResolveFailure
impl<'s, 't, T: KindT<'s, 't>> ResolveFailure<'s, 't, T> {
// mig: fn expect
fn expect(self) -> ResolveSuccess<'s, 't, T> {
    panic!("Unimplemented: expect");
}
/*
case class ResolveFailure[+T <: KindT](range: List[RangeS], x: IResolvingError) extends IResolveOutcome[T] {
*/
}
/*
  override def expect(): ResolveSuccess[T] = {
    throw CompileErrorExceptionT(TypingPassResolvingError(range, x))
  }
}
*/
// mig: struct StructCompiler
pub struct StructCompiler<'s, 'ctx, 't> {
    pub opts: TypingPassOptions<'s>,
    pub interner: &'ctx Interner<'s>,
    pub keywords: &'ctx Keywords<'s>,
    pub name_translator: NameTranslator<'s>,
    pub templata_compiler: TemplataCompiler<'s, 'ctx, 't>,
    pub infer_compiler: InferCompiler<'s, 't>,
    pub delegate: Box<dyn IStructCompilerDelegate<'s, 't>>,
    pub template_args_layer: StructCompilerGenericArgsLayer<'s, 'ctx, 't>,
}
// mig: impl StructCompiler
impl<'s, 'ctx, 't> StructCompiler<'s, 'ctx, 't> {
/*
class StructCompiler(
    opts: TypingPassOptions,
    interner: Interner,
    keywords: Keywords,
    nameTranslator: NameTranslator,
    templataCompiler: TemplataCompiler,
    inferCompiler: InferCompiler,
    delegate: IStructCompilerDelegate) {
  val templateArgsLayer =
    new StructCompilerGenericArgsLayer(
      opts, interner, keywords, nameTranslator, templataCompiler, inferCompiler, delegate)
*/
// mig: fn resolve_struct
fn resolve_struct(
    &self,
    coutputs: &CompilerOutputs<'s>,
    calling_env: &dyn IInDenizenEnvironmentT<'s>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    struct_templata: StructDefinitionTemplataT<'s>,
    uncoerced_template_args: &[ITemplataT<'s>],
) -> IResolveOutcome<'s, StructTT<'s>> {
    panic!("Unimplemented: resolve_struct");
}
/*
  def resolveStruct(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    structTemplata: StructDefinitionTemplataT,
    uncoercedTemplateArgs: Vector[ITemplataT[ITemplataType]]):
  IResolveOutcome[StructTT] = {
    Profiler.frame(() => {
      templateArgsLayer.resolveStruct(
        coutputs, callingEnv, callRange, callLocation, structTemplata, uncoercedTemplateArgs)
    })
  }
*/
// mig: fn precompile_struct
fn precompile_struct(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    struct_templata: StructDefinitionTemplataT<'s>,
) -> () {
    panic!("Unimplemented: precompile_struct");
}
/*
  def precompileStruct(
    coutputs: CompilerOutputs,
    structTemplata: StructDefinitionTemplataT):
  Unit = {
    val StructDefinitionTemplataT(declaringEnv, structA) = structTemplata

    val structTemplateId = templataCompiler.resolveStructTemplate(structTemplata)

    coutputs.declareType(structTemplateId)

    structA.maybePredictedMutability match {
      case None =>
      case Some(predictedMutability) => {
        coutputs.declareTypeMutability(
          structTemplateId,
          MutabilityTemplataT(Conversions.evaluateMutability(predictedMutability)))
      }
    }

    // We declare the struct's outer environment this early because of MDATOEF.
    val outerEnv =
      CitizenEnvironmentT(
        declaringEnv.globalEnv,
        declaringEnv,
        structTemplateId,
        structTemplateId,
        TemplatasStore(structTemplateId, Map(), Map())
          .addEntries(
            interner,
            // Merge in any things from the global environment that say they're part of this
            // structs's namespace (see IMRFDI and CODME).
            // StructFreeMacro will put a free function here.
            declaringEnv.globalEnv.nameToTopLevelEnvironment
              .get(structTemplateId.addStep(interner.intern(PackageTopLevelNameT())))
              .toVector
              .flatMap(_.entriesByNameT)))
    coutputs.declareTypeOuterEnv(structTemplateId, outerEnv)
  }
*/
// mig: fn precompile_interface
fn precompile_interface(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    interface_templata: InterfaceDefinitionTemplataT<'s>,
) -> () {
    panic!("Unimplemented: precompile_interface");
}
/*
  def precompileInterface(
    coutputs: CompilerOutputs,
    interfaceTemplata: InterfaceDefinitionTemplataT):
  Unit = {
    val InterfaceDefinitionTemplataT(declaringEnv, interfaceA) = interfaceTemplata

    val interfaceTemplateId = templataCompiler.resolveInterfaceTemplate(interfaceTemplata)

    coutputs.declareType(interfaceTemplateId)

    interfaceA.maybePredictedMutability match {
      case None =>
      case Some(predictedMutability) => {
        coutputs.declareTypeMutability(
          interfaceTemplateId,
          MutabilityTemplataT(Conversions.evaluateMutability(predictedMutability)))
      }
    }

    // We do this here because we might compile a virtual function somewhere before we compile the interface.
    // The virtual function will need to know if the type is sealed to know whether it's allowed to be
    // virtual on this interface.
    coutputs.declareTypeSealed(interfaceTemplateId, interfaceA.attributes.contains(SealedS))


    // We declare the interface's outer environment this early because of MDATOEF.
    val outerEnv =
      CitizenEnvironmentT(
        declaringEnv.globalEnv,
        declaringEnv,
        interfaceTemplateId,
        interfaceTemplateId,
        TemplatasStore(interfaceTemplateId, Map(), Map())
          .addEntries(
            interner,
            // TODO: Take those internal methods that were defined inside the interface, and move them to
            // just be name-prefixed like Free is, see IMRFDI.
            interfaceA.internalMethods
              .map(internalMethod => {
                val functionName = nameTranslator.translateGenericFunctionName(internalMethod.name)
                (functionName -> FunctionEnvEntry(internalMethod))
              }) ++
              // Merge in any things from the global environment that say they're part of this
              // interface's namespace (see IMRFDI and CODME).
              declaringEnv.globalEnv.nameToTopLevelEnvironment
                .get(interfaceTemplateId.addStep(interner.intern(PackageTopLevelNameT())))
                .toVector
                .flatMap(_.entriesByNameT)))
    coutputs.declareTypeOuterEnv(interfaceTemplateId, outerEnv)
  }
*/
// mig: fn compile_struct
fn compile_struct(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    parent_ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    struct_templata: StructDefinitionTemplataT<'s>,
) -> UncheckedDefiningConclusions<'s, 't> {
    panic!("Unimplemented: compile_struct");
}
/*
  def compileStruct(
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    structTemplata: StructDefinitionTemplataT):
  UncheckedDefiningConclusions = {
    Profiler.frame(() => {
      templateArgsLayer.compileStruct(coutputs, parentRanges, callLocation, structTemplata)
    })
  }

  // See SFWPRL for how this is different from resolveInterface.
*/
// mig: fn predict_interface
fn predict_interface(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    calling_env: &dyn IInDenizenEnvironmentT<'s, 't>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    interface_templata: InterfaceDefinitionTemplataT<'s>,
    uncoerced_template_args: &[ITemplataT<'s, 't>],
) -> InterfaceTT<'s, 't> {
    panic!("Unimplemented: predict_interface");
}
/*
  def predictInterface(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    // We take the entire templata (which includes environment and parents) so we can incorporate
    // their rules as needed
    interfaceTemplata: InterfaceDefinitionTemplataT,
    uncoercedTemplateArgs: Vector[ITemplataT[ITemplataType]]):
  (InterfaceTT) = {
    templateArgsLayer.predictInterface(
      coutputs, callingEnv, callRange, callLocation, interfaceTemplata, uncoercedTemplateArgs)
  }

  // See SFWPRL for how this is different from resolveStruct.
*/
// mig: fn predict_struct
fn predict_struct(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    calling_env: &dyn IInDenizenEnvironmentT<'s, 't>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    struct_templata: StructDefinitionTemplataT<'s>,
    uncoerced_template_args: &[ITemplataT<'s, 't>],
) -> StructTT<'s, 't> {
    panic!("Unimplemented: predict_struct");
}
/*
  def predictStruct(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    // We take the entire templata (which includes environment and parents) so we can incorporate
    // their rules as needed
    structTemplata: StructDefinitionTemplataT,
    uncoercedTemplateArgs: Vector[ITemplataT[ITemplataType]]):
  (StructTT) = {
    templateArgsLayer.predictStruct(
      coutputs, callingEnv, callRange, callLocation, structTemplata, uncoercedTemplateArgs)
  }
*/
// mig: fn resolve_interface
fn resolve_interface(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    calling_env: &dyn IInDenizenEnvironmentT<'s, 't>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    interface_templata: InterfaceDefinitionTemplataT<'s>,
    uncoerced_template_args: &[ITemplataT<'s, 't>],
) -> IResolveOutcome<'s, 't, InterfaceTT<'s, 't>> {
    panic!("Unimplemented: resolve_interface");
}
/*
  def resolveInterface(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    // We take the entire templata (which includes environment and parents) so we can incorporate
    // their rules as needed
    interfaceTemplata: InterfaceDefinitionTemplataT,
    uncoercedTemplateArgs: Vector[ITemplataT[ITemplataType]]):
  IResolveOutcome[InterfaceTT] = {
    val success =
      templateArgsLayer.resolveInterface(
        coutputs, callingEnv, callRange, callLocation, interfaceTemplata, uncoercedTemplateArgs)

    success
  }
*/
// mig: fn compile_interface
fn compile_interface(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    parent_ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    interface_templata: InterfaceDefinitionTemplataT<'s>,
) -> UncheckedDefiningConclusions<'s, 't> {
    panic!("Unimplemented: compile_interface");
}
/*
  def compileInterface(
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    // We take the entire templata (which includes environment and parents) so we can incorporate
    // their rules as needed
    interfaceTemplata: InterfaceDefinitionTemplataT):
  UncheckedDefiningConclusions = {
    templateArgsLayer.compileInterface(
      coutputs, parentRanges, callLocation, interfaceTemplata)
  }

  // Makes a struct to back a closure
*/
// mig: fn make_closure_understruct
fn make_closure_understruct(
    &self,
    containing_function_env: NodeEnvironmentT<'s, 't>,
    coutputs: &CompilerOutputs<'s, 't>,
    parent_ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    name: IFunctionDeclarationNameS<'s>,
    function_s: &FunctionA<'s>,
    members: &[NormalStructMemberT<'s, 't>],
) -> (StructTT<'s, 't>, MutabilityT, FunctionTemplataT<'s>) {
    panic!("Unimplemented: make_closure_understruct");
}
/*
  def makeClosureUnderstruct(
    containingFunctionEnv: NodeEnvironmentT,
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    name: IFunctionDeclarationNameS,
    functionS: FunctionA,
    members: Vector[NormalStructMemberT]):
  (StructTT, MutabilityT, FunctionTemplataT) = {
//    Profiler.reentrant("StructCompiler-makeClosureUnderstruct", name.codeLocation.toString, () => {
      templateArgsLayer.makeClosureUnderstruct(containingFunctionEnv, coutputs, parentRanges, callLocation, name, functionS, members)
//    })
  }

//  def getMemberCoords(coutputs: CompilerOutputs, structTT: StructTT): Vector[CoordT] = {
//    coutputs.lookupStruct(structTT).members.map(_.tyype).map({
//      case ReferenceMemberTypeT(coord) => coord
//      case AddressMemberTypeT(_) => {
//        // At time of writing, the only one who calls this is the inferer, who wants to know so it
//        // can match incoming arguments into a destructure. Can we even destructure things with
//        // addressible members?
//        vcurious()
//      }
//    })
//  }

}
*/
}

pub mod StructCompiler {
/*
object StructCompiler {
*/
// mig: fn get_compound_type_mutability
pub fn get_compound_type_mutability(member_types: &[CoordT<'_, '_>]) -> MutabilityT {
    panic!("Unimplemented: get_compound_type_mutability");
}
/*
  def getCompoundTypeMutability(memberTypes2: Vector[CoordT])
  : MutabilityT = {
    val membersOwnerships = memberTypes2.map(_.ownership)
    val allMembersImmutable = membersOwnerships.isEmpty || membersOwnerships.toSet == Set(ShareT)
    if (allMembersImmutable) ImmutableT else MutableT
  }
*/
// mig: fn get_mutability
pub fn get_mutability(
    sanity_check: bool,
    interner: &Interner,
    keywords: &Keywords,
    coutputs: &CompilerOutputs<'_, '_>,
    original_calling_denizen_id: IdT,
    region: RegionT<'_, '_>,
    struct_tt: StructTT<'_, '_>,
    bound_arguments_source: &dyn IBoundArgumentsSource<'_, '_>,
) -> ITemplataT<'_, '_> {
    panic!("Unimplemented: get_mutability");
}
/*
  def getMutability(
    sanityCheck: Boolean,
    interner: Interner,
    keywords: Keywords,
    coutputs: CompilerOutputs,
    originalCallingDenizenId: IdT[ITemplateNameT],
    region: RegionT,
    structTT: StructTT,
    boundArgumentsSource: IBoundArgumentsSource):
  ITemplataT[MutabilityTemplataType] = {
    val definition = coutputs.lookupStruct(structTT.id)
    val transformer =
      TemplataCompiler.getPlaceholderSubstituter(
        sanityCheck,
        interner, keywords,
        originalCallingDenizenId,
        structTT.id, boundArgumentsSource)
    val result = transformer.substituteForTemplata(coutputs, definition.mutability)
    ITemplataT.expectMutability(result)
  }
}
*/
}

