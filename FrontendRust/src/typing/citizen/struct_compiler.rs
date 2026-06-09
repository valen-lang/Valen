use crate::keywords::Keywords;
use crate::utils::range::RangeS;

use crate::postparsing::names::*;
use crate::higher_typing::ast::*;

use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::ast::citizens::*;
use crate::typing::env::environment::*;
use crate::typing::env::i_env_entry::IEnvEntryT;
use crate::typing::env::function_environment_t::*;
use crate::typing::compiler_outputs::*;
use crate::interner::Interner;
use crate::typing::templata_compiler::*;
use crate::typing::infer_compiler::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::postparsing::ast::LocationInDenizen;
use crate::postparsing::rules::rules::*;
use std::marker::PhantomData;
use crate::postparsing::ast::ICitizenAttributeS;

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

/*
// See ODMFRC.
*/
pub struct UncheckedDefiningConclusions<'s, 't> {
    pub envs: InferEnv<'s, 't>,
    pub ranges: Vec<RangeS<'s>>,
    pub call_location: LocationInDenizen<'s>,
    pub definition_rules: Vec<IRulexSR<'s>>,
    pub conclusions: std::collections::HashMap<IRuneS<'s>, ITemplataT<'s, 't>>,
}
/*
case class UncheckedDefiningConclusions(
    envs: InferEnv,
    ranges: List[RangeS],
    callLocation: LocationInDenizen,
    definitionRules: Vector[IRulexSR],
    conclusions: Map[IRuneS, ITemplataT[ITemplataType]])
*/
// deleted: delegate trait removed per god-struct refactor (Compiler now holds all methods directly)
/*
trait IStructCompilerDelegate {
*/
/*
  def evaluateGenericFunctionFromNonCallForHeader(
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    functionTemplata: FunctionTemplataT):
  FunctionHeaderT
*/
/*
  def scoutExpectedFunctionForPrototype(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    functionName: IImpreciseNameS,
    explicitTemplateArgRulesS: Vector[IRulexSR],
    positionalExplicitTemplateArgRunesS: Vector[IRuneS],
    receivingRuneToExplicitTemplateArgRune: Vector[(RuneUsage, RuneUsage)],
    contextRegion: RegionT,
    args: Vector[CoordT],
    extraEnvsToLookIn: Vector[IInDenizenEnvironmentT],
    exact: Boolean):
  StampFunctionSuccess
}
*/

pub enum IResolveOutcome<'s, 't, T> {
    ResolveSuccess(ResolveSuccess<'s, 't, T>),
    ResolveFailure(ResolveFailure<'s, 't, T>),
}
/*
sealed trait IResolveOutcome[+T <: KindT] {
*/
fn resolve_outcome_expect<'s, 't, T>(this: IResolveOutcome<'s, 't, T>) -> ResolveSuccess<'s, 't, T> { panic!("Unimplemented: expect"); }
/*
  def expect(): ResolveSuccess[T]
}
*/

pub struct ResolveSuccess<'s, 't, T> {
    pub kind: T,
    pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
impl<'s, 't, T> ResolveSuccess<'s, 't, T> {
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
#[derive(Debug)]
pub struct ResolveFailure<'s, 't, T> {
    pub range: Vec<RangeS<'s>>,
    pub x: IResolvingError<'s, 't>,
    pub _phantom: std::marker::PhantomData<T>,
}
impl<'s, 't, T> ResolveFailure<'s, 't, T> {
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
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn resolve_struct(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        call_range: &'t [RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        struct_templata: StructDefinitionTemplataT<'s, 't>,
        uncoerced_template_args: &[ITemplataT<'s, 't>],
    ) -> IResolveOutcome<'s, 't, StructTT<'s, 't>> {
        self.resolve_struct_layer(coutputs, calling_env, call_range, call_location, struct_templata, uncoerced_template_args)
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
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn precompile_struct(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        struct_templata: StructDefinitionTemplataT<'s, 't>,
    ) -> () {
        let declaring_env = struct_templata.declaring_env;
        let struct_a = struct_templata.origin_struct;
        let struct_template_id = self.resolve_struct_template(
            self.typing_interner.alloc(struct_templata)
        );
        coutputs.declare_type(struct_template_id);
        match struct_a.maybe_predicted_mutability {
            None => {}
            Some(predicted_mutability) => {
                coutputs.declare_type_mutability(
                    struct_template_id,
                    ITemplataT::Mutability(MutabilityTemplataT {
                        mutability: crate::typing::templata::conversions::evaluate_mutability(predicted_mutability),
                    }),
                );
            }
        }
        // Build internal method entries for the outer env
        let internal_method_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> =
            struct_a.internal_methods.iter().map(|internal_method| {
                let function_name = self.translate_generic_function_name(internal_method.name);
                (INameT::from(function_name), IEnvEntryT::Function(internal_method))
            }).collect();
        let sibling_key = struct_template_id.add_step(
            self.typing_interner,
            INameT::PackageTopLevel(self.typing_interner.intern_package_top_level_name(
                PackageTopLevelNameT { _phantom: PhantomData }
            )),
        );
        let sibling_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> =
            declaring_env.global_env().name_to_top_level_environment.iter()
                .filter(|(id, _)| **id == *sibling_key)
                .flat_map(|(_, ts)| ts.name_to_entry.iter().map(|(n, e)| (*n, *e)))
                .collect();
        let all_outer_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> =
            internal_method_entries.into_iter().chain(sibling_entries.into_iter()).collect();
        let mut outer_store = TemplatasStoreBuilder::new(struct_template_id);
        outer_store.add_entries(self.scout_arena, all_outer_entries);
        let outer_templatas = outer_store.build_in(self.typing_interner);
        let outer_env = self.typing_interner.alloc(CitizenEnvironmentT {
            global_env: declaring_env.global_env(),
            parent_env: declaring_env,
            template_id: *struct_template_id,
            id: *struct_template_id,
            templatas: outer_templatas,
        });
        let outer_env_ref = IInDenizenEnvironmentT::Citizen(outer_env);
        coutputs.declare_type_outer_env(struct_template_id, outer_env_ref);
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
            // Internal methods declared inside the struct body (see IMRFDI). Mirrors the
            // interface registration below: the struct's outer env is the single home for
            // its internal methods, and lookups from per-instantiation envs walk up to find
            // them.
            structA.internalMethods
              .map(internalMethod => {
                val functionName = nameTranslator.translateGenericFunctionName(internalMethod.name)
                (functionName -> FunctionEnvEntry(internalMethod))
              }) ++
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
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn precompile_interface(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        interface_templata: InterfaceDefinitionTemplataT<'s, 't>,
    ) -> () {
        let declaring_env = interface_templata.declaring_env;
        let interface_a = interface_templata.origin_interface;
        let interface_template_id = self.resolve_interface_template(
            self.typing_interner.alloc(interface_templata)
        );
        coutputs.declare_type(interface_template_id);
        match interface_a.maybe_predicted_mutability {
            None => {}
            Some(predicted_mutability) => {
                coutputs.declare_type_mutability(
                    interface_template_id,
                    ITemplataT::Mutability(MutabilityTemplataT {
                        mutability: crate::typing::templata::conversions::evaluate_mutability(predicted_mutability),
                    }),
                );
            }
        }
        // We do this here because we might compile a virtual function somewhere before we compile
        // the interface. The virtual function will need to know if the type is sealed to know
        // whether it's allowed to be virtual on this interface.
        coutputs.declare_type_sealed(
            *interface_template_id,
            interface_a.attributes.iter().any(|a| matches!(a, ICitizenAttributeS::Sealed(_))),
        );
        // Build internal method entries for the outer env
        let internal_method_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> =
            interface_a.internal_methods.iter().map(|internal_method| {
                let function_name = self.translate_generic_function_name(internal_method.name);
                let local_name = match function_name {
                    IFunctionTemplateNameT::FunctionTemplate(r) => INameT::FunctionTemplate(r),
                    IFunctionTemplateNameT::ForwarderFunctionTemplate(r) => INameT::ForwarderFunctionTemplate(r),
                    IFunctionTemplateNameT::ConstructorTemplate(r) => INameT::ConstructorTemplate(r),
                    IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(r) => INameT::AnonymousSubstructConstructorTemplate(r),
                    IFunctionTemplateNameT::LambdaCallFunctionTemplate(r) => INameT::LambdaCallFunctionTemplate(r),
                    IFunctionTemplateNameT::OverrideDispatcherTemplate(r) => INameT::OverrideDispatcherTemplate(r),
                    IFunctionTemplateNameT::ExternFunction(r) => INameT::ExternFunction(r),
                    IFunctionTemplateNameT::FunctionBoundTemplate(r) => INameT::FunctionBoundTemplate(r),
                    IFunctionTemplateNameT::PredictedFunctionTemplate(r) => INameT::PredictedFunctionTemplate(r),
                };
                (local_name, IEnvEntryT::Function(internal_method))
            }).collect();
        // Merge in sibling entries from the global environment
        let sibling_key = interface_template_id.add_step(
            self.typing_interner,
            INameT::PackageTopLevel(self.typing_interner.intern_package_top_level_name(
                PackageTopLevelNameT { _phantom: PhantomData }
            )),
        );
        let sibling_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> =
            declaring_env.global_env().name_to_top_level_environment.iter()
                .filter(|(id, _)| **id == *sibling_key)
                .flat_map(|(_, ts)| ts.name_to_entry.iter().map(|(n, e)| (*n, *e)))
                .collect();
        let all_outer_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> =
            internal_method_entries.into_iter().chain(sibling_entries.into_iter()).collect();
        let mut outer_store = TemplatasStoreBuilder::new(interface_template_id);
        outer_store.add_entries(self.scout_arena, all_outer_entries);
        let outer_templatas = outer_store.build_in(self.typing_interner);
        let outer_env = self.typing_interner.alloc(CitizenEnvironmentT {
            global_env: declaring_env.global_env(),
            parent_env: declaring_env,
            template_id: *interface_template_id,
            id: *interface_template_id,
            templatas: outer_templatas,
        });
        let outer_env_ref = IInDenizenEnvironmentT::Citizen(outer_env);
        coutputs.declare_type_outer_env(interface_template_id, outer_env_ref);
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
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn compile_struct(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        struct_templata: StructDefinitionTemplataT<'s, 't>,
    ) -> Result<UncheckedDefiningConclusions<'s, 't>, ICompileErrorT<'s, 't>> {
        self.compile_struct_layer(coutputs, parent_ranges, call_location, struct_templata)
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
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn predict_interface(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        call_range: &'t [RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        interface_templata: InterfaceDefinitionTemplataT<'s, 't>,
        uncoerced_template_args: &[ITemplataT<'s, 't>],
    ) -> InterfaceTT<'s, 't> {
        self.predict_interface_layer(coutputs, calling_env, call_range, call_location, interface_templata, uncoerced_template_args)
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
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn predict_struct(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        call_range: &'t [RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        struct_templata: StructDefinitionTemplataT<'s, 't>,
        uncoerced_template_args: &[ITemplataT<'s, 't>],
    ) -> StructTT<'s, 't> {
        self.predict_struct_layer(coutputs, calling_env, call_range, call_location, struct_templata, uncoerced_template_args)
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
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn resolve_interface(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        call_range: &'t [RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        interface_templata: InterfaceDefinitionTemplataT<'s, 't>,
        uncoerced_template_args: &[ITemplataT<'s, 't>],
    ) -> IResolveOutcome<'s, 't, InterfaceTT<'s, 't>> {
        self.resolve_interface_layer(coutputs, calling_env, call_range, call_location, interface_templata, uncoerced_template_args)
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
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn compile_interface(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        interface_templata: InterfaceDefinitionTemplataT<'s, 't>,
    ) -> Result<UncheckedDefiningConclusions<'s, 't>, ICompileErrorT<'s, 't>> {
        self.compile_interface_layer(coutputs, parent_ranges, call_location, interface_templata)
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
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn make_closure_understruct(
        &self,
        containing_function_env: &'t NodeEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        name: IFunctionDeclarationNameS<'s>,
        function_s: &'s FunctionA<'s>,
        members: &[&'t NormalStructMemberT<'s, 't>],
    ) -> Result<(StructTT<'s, 't>, MutabilityT, FunctionTemplataT<'s, 't>), ICompileErrorT<'s, 't>> {
        self.make_closure_understruct_core(
            containing_function_env, coutputs, parent_ranges, call_location, name, function_s, members)
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

/*
object StructCompiler {
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_compound_type_mutability(
        &self,
        member_types: &[CoordT<'s, 't>],
    ) -> MutabilityT {
        panic!("Unimplemented: Slab 15 — body migration");
    }
    /*
      def getCompoundTypeMutability(memberTypes2: Vector[CoordT])
      : MutabilityT = {
        val membersOwnerships = memberTypes2.map(_.ownership)
        val allMembersImmutable = membersOwnerships.isEmpty || membersOwnerships.toSet == Set(ShareT)
        if (allMembersImmutable) ImmutableT else MutableT
      }
    */
}
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn struct_compiler_get_mutability(
        &self,
        sanity_check: bool,
        coutputs: &mut CompilerOutputs<'s, 't>,
        original_calling_denizen_id: IdT<'s, 't>,
        region: RegionT,
        struct_tt: StructTT<'s, 't>,
        bound_arguments_source: IBoundArgumentsSource<'s, 't>,
    ) -> ITemplataT<'s, 't> {
        let definition = coutputs.lookup_struct(struct_tt.id, self);
        let transformer = self.get_placeholder_substituter(
            sanity_check,
            original_calling_denizen_id,
            struct_tt.id,
            bound_arguments_source,
        );
        let result = transformer.substitute_for_templata(coutputs, definition.mutability);
        result
    }
    /* Guardian: disable-all */
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