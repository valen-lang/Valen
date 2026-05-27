use crate::typing::compiler::Compiler;
use crate::typing::function::function_compiler::*;
use crate::utils::range::RangeS;
use crate::postparsing::names::*;
use crate::postparsing::ast::*;
use crate::postparsing::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::compiler_outputs::*;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::infer_compiler::InitialKnown;
use crate::higher_typing::ast::*;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::typing::ast::citizens::{IStructMemberT, NormalStructMemberT, IMemberTypeT, ReferenceMemberTypeT, AddressMemberTypeT};
use crate::typing::env::function_environment_t::{IVariableT, ReferenceClosureVariableT, AddressibleClosureVariableT};
use crate::typing::env::i_env_entry::IEnvEntryT;
use crate::typing::templata_compiler::IBoundArgumentsSource;
use crate::typing::templata::templata::KindTemplataT;

/*
package dev.vale.typing.function

import dev.vale.{Interner, Keywords, Profiler, RangeS, vassert, vcurious, vfail, vimpl}
import dev.vale.highertyping.FunctionA
import dev.vale.postparsing._
import dev.vale.typing.citizen.StructCompiler
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale.postparsing.IFunctionDeclarationNameS
import dev.vale.typing._
import dev.vale.typing.ast._
import dev.vale.typing.env._
import dev.vale.typing.function._
import dev.vale.typing.ast.{FunctionBannerT, FunctionHeaderT, PrototypeT}
import dev.vale.typing.env.{AddressibleClosureVariableT, BuildingFunctionEnvironmentWithClosuredsT, IEnvEntry, IInDenizenEnvironmentT, IVariableT, ReferenceClosureVariableT, TemplataEnvEntry, TemplatasStore}
import dev.vale.typing.{CompilerOutputs, ConvertHelper, InferCompiler, InitialKnown, TemplataCompiler, TypingPassOptions, env}
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._

import scala.collection.immutable.{List, Map}

// When typingpassing a function, these things need to happen:
// - Spawn a local environment for the function
// - Add any closure args to the environment
// - Incorporate any template arguments into the environment
// There's a layer to take care of each of these things.
// This file is the outer layer, which spawns a local environment for the function.
*/
/*
class FunctionCompilerClosureOrLightLayer(
    opts: TypingPassOptions,
    interner: Interner,
    keywords: Keywords,
    nameTranslator: NameTranslator,
    templataCompiler: TemplataCompiler,
    inferCompiler: InferCompiler,
    convertHelper: ConvertHelper,
    structCompiler: StructCompiler,
    delegate: IFunctionCompilerDelegate) {
  val ordinaryOrTemplatedLayer =
    new FunctionCompilerSolvingLayer(
      opts, interner, keywords, nameTranslator, templataCompiler, inferCompiler, convertHelper, structCompiler, delegate)
//
//  // This is for the early stages of Compiler when it's scanning banners to put in
//  // its env. We just want its banner, we don't want to evaluate it.
//  def predictOrdinaryLightFunctionBanner(
//    outerEnv: IEnvironment,
//    coutputs: CompilerOutputs,
//    function: FunctionA):
//  (FunctionBannerT) = {
//    checkNotClosure(function);
//    vassert(!function.isTemplate)
//
//    val newEnv = makeEnvWithoutClosureStuff(outerEnv, function)
//    ordinaryOrTemplatedLayer.predictOrdinaryFunctionBanner(
//      newEnv, coutputs)
//  }


//  def evaluateOrdinaryLightFunctionFromNonCallForBanner(
//    outerEnv: IEnvironment,
//    coutputs: CompilerOutputs,
//    callRange: List[RangeS],
//    function: FunctionA,
//    verifyConclusions: Boolean):
//  (PrototypeTemplata) = {
//    checkNotClosure(function);
//    vassert(!function.isTemplate)
//
//    val newEnv = makeEnvWithoutClosureStuff(outerEnv, function)
//
//    vcurious(function.isLambda())
//    // We dont declare the template here, we declare it when the lambda is declared.
////    coutputs.declareFunction(newEnv.fullName)
////    coutputs.declareFunctionOuterEnv(newEnv.fullName, outerEnv)
//
//    ordinaryOrTemplatedLayer.evaluateOrdinaryFunctionFromNonCallForBanner(
//      newEnv, coutputs, callRange, verifyConclusions)
//  }

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_templated_closure_function_from_call_for_banner(
        &self,
        parent_env: IEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        closure_struct_ref: StructTT<'s, 't>,
        function: &'s FunctionA<'s>,
        already_specified_template_args: &[ITemplataT<'s, 't>],
        context_region: RegionT,
        arg_types: &[CoordT<'s, 't>],
    ) -> Result<IEvaluateFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
        let (variables, entries) = self.make_closure_variables_and_entries(coutputs, calling_env.denizen_template_id(), closure_struct_ref);
        let name = self.typing_interner.alloc(
            parent_env.id().add_step(self.typing_interner,
                self.translate_generic_template_function_name(function.name, arg_types)));
        let mut builder = TemplatasStoreBuilder::new(name);
        builder.add_entries(self.scout_arena, entries);
        let templatas = builder.build_in(self.typing_interner);
        let variables_t = self.typing_interner.alloc_slice_from_vec(variables);
        let outer_env = self.typing_interner.alloc(BuildingFunctionEnvironmentWithClosuredsT {
            global_env: parent_env.global_env(),
            parent_env,
            id: **name,
            templatas,
            function,
            variables: variables_t,
            is_root_compiling_denizen: false,
        });
        self.evaluate_templated_function_from_call_for_banner(
            outer_env, coutputs, calling_env, call_range, call_location,
            already_specified_template_args, context_region, arg_types)
    }
/*
  def evaluateTemplatedClosureFunctionFromCallForBanner(
      parentEnv: IEnvironmentT,
      coutputs: CompilerOutputs,
      callingEnv: IInDenizenEnvironmentT,
      callRange: List[RangeS],
      callLocation: LocationInDenizen,
      closureStructRef: StructTT,
      function: FunctionA,
      alreadySpecifiedTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
      argTypes: Vector[CoordT]):
  (IEvaluateFunctionResult) = {
    val (variables, entries) = makeClosureVariablesAndEntries(coutputs, callingEnv.denizenTemplateId, closureStructRef)
    val name = parentEnv.id.addStep(nameTranslator.translateGenericTemplateFunctionName(function.name, argTypes))
//    coutputs.declareType(name)
    val outerEnv =
      BuildingFunctionEnvironmentWithClosuredsT(
        parentEnv.globalEnv,
        parentEnv,
        name,
        TemplatasStore(name, Map(), Map()).addEntries(interner, entries),
        function,
        variables,
        false)
//    coutputs.declareTypeOuterEnv(name, outerEnv)

    ordinaryOrTemplatedLayer.evaluateTemplatedFunctionFromCallForBanner(
      outerEnv, coutputs, callingEnv, callRange, callLocation, alreadySpecifiedTemplateArgs, contextRegion, argTypes)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_templated_closure_function_from_call_for_prototype(
        &self,
        outer_env: IEnvironmentT,
        coutputs: CompilerOutputs,
        calling_env: IInDenizenEnvironmentT,
        call_range: Vec<RangeS>,
        call_location: LocationInDenizen,
        closure_struct_ref: StructTT,
        function: FunctionA,
        already_specified_template_args: Vec<ITemplataT>,
        context_region: RegionT,
        arg_types: Vec<CoordT>,
    ) -> IEvaluateFunctionResult<'_, '_> {
        panic!("Unimplemented: evaluate_templated_closure_function_from_call_for_prototype");
    }
/*
  // Per @LAGTNGZ, each call site reaches here with its own argTypes, producing a distinct LambdaCallFunctionTemplateNameT.
  def evaluateTemplatedClosureFunctionFromCallForPrototype(
    outerEnv: IEnvironmentT,
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    closureStructRef: StructTT,
    function: FunctionA,
    alreadySpecifiedTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
    argTypes: Vector[CoordT]):
  (IEvaluateFunctionResult) = {
    val (variables, entries) = makeClosureVariablesAndEntries(coutputs, callingEnv.denizenTemplateId, closureStructRef)
    val name = outerEnv.id.addStep(nameTranslator.translateGenericTemplateFunctionName(function.name, argTypes))
    val newEnv =
      env.BuildingFunctionEnvironmentWithClosuredsT(
        outerEnv.globalEnv,
        outerEnv,
        name,
        TemplatasStore(name, Map(), Map()).addEntries(interner, entries),
        function,
        variables,
        false)
    ordinaryOrTemplatedLayer.evaluateTemplatedFunctionFromCallForPrototype(
      newEnv, coutputs, callingEnv, callRange, callLocation, alreadySpecifiedTemplateArgs, contextRegion, argTypes)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_templated_light_function_from_call_for_prototype2(
        &self,
        parent_env: IEnvironmentT,
        coutputs: CompilerOutputs,
        calling_env: IInDenizenEnvironmentT,
        call_range: Vec<RangeS>,
        call_location: LocationInDenizen,
        function: FunctionA,
        explicit_template_args: Vec<ITemplataT>,
        context_region: RegionT,
        arg_types: Vec<CoordT>,
    ) -> IEvaluateFunctionResult<'_, '_> {
        panic!("Unimplemented: evaluate_templated_light_function_from_call_for_prototype2");
    }
/*
  def evaluateTemplatedLightFunctionFromCallForPrototype2(
      parentEnv: IEnvironmentT,
      coutputs: CompilerOutputs,
      callingEnv: IInDenizenEnvironmentT, // See CSSNCE
      callRange: List[RangeS],
      callLocation: LocationInDenizen,
      function: FunctionA,
      explicitTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
      argTypes: Vector[CoordT]):
  (IEvaluateFunctionResult) = {
    checkNotClosure(function);

    val outerEnvId = parentEnv.id.addStep(nameTranslator.translateGenericTemplateFunctionName(function.name, argTypes))
    val outerEnv = makeEnvWithoutClosureStuff(parentEnv, function, outerEnvId, false)
    ordinaryOrTemplatedLayer.evaluateTemplatedFunctionFromCallForPrototype(
      outerEnv, coutputs, callingEnv, callRange, callLocation, explicitTemplateArgs, contextRegion, argTypes)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_generic_light_function_from_call_for_prototype2(
        &self,
        parent_env: IEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        function: &'s FunctionA<'s>,
        explicit_template_args: &[ITemplataT<'s, 't>],
        context_region: RegionT,
        args: &[Option<CoordT<'s, 't>>],
        container_rune_initial_knowns: &[InitialKnown<'s, 't>],
    ) -> Result<IResolveFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
        self.check_not_closure(function);

        let function_template_name = self.translate_generic_function_name(function.name);
        let function_name_local: INameT<'s, 't> = match function_template_name {
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
        let outer_env_id = parent_env.id().add_step(self.typing_interner, function_name_local);
        let outer_env = self.make_env_without_closure_stuff(parent_env, function, outer_env_id, false);
        self.evaluate_generic_function_from_call_for_prototype(
            outer_env, coutputs, calling_env, call_range, call_location, explicit_template_args, context_region, args, container_rune_initial_knowns)
    }
/*
  // Per @LAGTNGZ, "Generic" here means true generics (one solve, later stamped); contrast the Templated methods used for lambdas.
  def evaluateGenericLightFunctionFromCallForPrototype2(
    parentEnv: IEnvironmentT,
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    function: FunctionA,
    explicitTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
    args: Vector[Option[CoordT]],
    containerRuneInitialKnowns: Vector[InitialKnown] = Vector.empty):
  (IResolveFunctionResult) = {
    checkNotClosure(function);

    val outerEnvId = parentEnv.id.addStep(nameTranslator.translateGenericFunctionName(function.name))
    val outerEnv = makeEnvWithoutClosureStuff(parentEnv, function, outerEnvId, false)
    ordinaryOrTemplatedLayer.evaluateGenericFunctionFromCallForPrototype(
      outerEnv, coutputs, callingEnv, callRange, callLocation, explicitTemplateArgs, contextRegion, args, containerRuneInitialKnowns)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_generic_virtual_dispatcher_function_for_prototype_closure_or_light(
        &self,
        parent_env: IEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        function: &'s FunctionA<'s>,
        args: &[Option<CoordT<'s, 't>>],
    ) -> Result<IDefineFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
        self.check_not_closure(function);
        let function_template_name = self.translate_generic_function_name(function.name);
        let function_name_local: INameT<'s, 't> = match function_template_name {
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
        let outer_env_id = parent_env.id().add_step(self.typing_interner, function_name_local);
        let outer_env = self.make_env_without_closure_stuff(parent_env, function, outer_env_id, true);
        self.evaluate_generic_virtual_dispatcher_function_for_prototype_solving(
            outer_env, coutputs, calling_env, call_range, call_location, args)
    }

/*
  def evaluateGenericVirtualDispatcherFunctionForPrototype(
    parentEnv: IEnvironmentT,
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    function: FunctionA,
    args: Vector[Option[CoordT]]):
  IDefineFunctionResult = {
    checkNotClosure(function);
    val outerEnvId = parentEnv.id.addStep(nameTranslator.translateGenericFunctionName(function.name))
    val outerEnv = makeEnvWithoutClosureStuff(parentEnv, function, outerEnvId, true)
    ordinaryOrTemplatedLayer.evaluateGenericVirtualDispatcherFunctionForPrototype(
      outerEnv, coutputs, callingEnv, callRange, callLocation, args)
  }


//  def evaluateOrdinaryLightFunctionFromNonCallForHeader(
//    outerEnv: IEnvironment,
//    coutputs: CompilerOutputs,
//    parentRanges: List[RangeS],
//    function: FunctionA,
//    verifyConclusions: Boolean):
//  (FunctionHeaderT) = {
//    vassert(!function.isTemplate)
//
//    val newEnv = makeEnvWithoutClosureStuff(outerEnv, function)
//    coutputs.declareFunction(newEnv.fullName)
//    coutputs.declareFunctionOuterEnv(newEnv.fullName, newEnv)
//    ordinaryOrTemplatedLayer.evaluateOrdinaryFunctionFromNonCallForHeader(
//      newEnv, coutputs, parentRanges, verifyConclusions)
//  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_generic_light_function_from_non_call(
        &self,
        parent_env: IEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        function: &'s FunctionA<'s>,
    ) -> Result<&'t FunctionHeaderT<'s, 't>, ICompileErrorT<'s, 't>> {
        let function_template_name = self.translate_generic_function_name(function.name);
        let function_name_local: INameT<'s, 't> = match function_template_name {
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
        let outer_env_id = parent_env.id().add_step(self.typing_interner, function_name_local);
        let outer_env = self.make_env_without_closure_stuff(parent_env, function, outer_env_id, true);
        self.evaluate_generic_function_from_non_call_solving(
            coutputs, outer_env, parent_ranges, call_location)
    }
/*
  def evaluateGenericLightFunctionFromNonCall(
    parentEnv: IEnvironmentT,
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    function: FunctionA):
  (FunctionHeaderT) = {
    val outerEnvId = parentEnv.id.addStep(nameTranslator.translateGenericFunctionName(function.name))
    val outerEnv = makeEnvWithoutClosureStuff(parentEnv, function, outerEnvId, true)
    ordinaryOrTemplatedLayer.evaluateGenericFunctionFromNonCall(
      coutputs, outerEnv, parentRanges, callLocation)
  }

//  def evaluateTemplatedLightFunctionFromNonCallForHeader(
//    outerEnv: IEnvironment,
//    coutputs: CompilerOutputs,
//    parentRanges: List[RangeS],
//    function: FunctionA,
//    verifyConclusions: Boolean):
//  (FunctionHeaderT) = {
////    vassert(function.isTemplate)
//
//    val newEnv = makeEnvWithoutClosureStuff(outerEnv, function)
//    ordinaryOrTemplatedLayer.evaluateTemplatedFunctionFromNonCallForHeader(
//      newEnv, coutputs, parentRanges, verifyConclusions)
//  }

//  // We would want only the prototype instead of the entire header if, for example,
//  // we were calling the function. This is necessary for a recursive function like
//  // func main():Int{main()}
//  def evaluateOrdinaryLightFunctionFromCallForPrototype(
//    outerEnv: IEnvironment,
//    coutputs: CompilerOutputs,
//    callingEnv: IEnvironment, // See CSSNCE
//    callRange: List[RangeS],
//    function: FunctionA
//  ): PrototypeTemplata = {
//    checkNotClosure(function)
//    vassert(!function.isTemplate)
//
//    val name = makeNameWithClosureds(outerEnv, function.name)
//    val newEnv =
//      env.BuildingFunctionEnvironmentWithClosureds(
//        outerEnv.globalEnv,
//        outerEnv,
//        name,
//        TemplatasStore(name, Map(), Map()),
//        function,
//        Vector.empty)
//    ordinaryOrTemplatedLayer.evaluateOrdinaryFunctionFromCallForPrototype(
//      newEnv, callingEnv, coutputs, callRange)
//  }
//
//  def evaluateOrdinaryClosureFunctionFromNonCallForBanner(
//    outerEnv: IEnvironment,
//    coutputs: CompilerOutputs,
//    callRange: List[RangeS],
//    closureStructRef: StructTT,
//    function: FunctionA,
//    verifyConclusions: Boolean):
//  (PrototypeTemplata) = {
//    vassert(!function.isTemplate)
//
//    val name = makeNameWithClosureds(outerEnv, function.name)
//    coutputs.declareFunction(name)
//    coutputs.declareFunctionOuterEnv(name, outerEnv)
//    val (variables, entries) = makeClosureVariablesAndEntries(coutputs, closureStructRef)
//    val newEnv =
//      env.BuildingFunctionEnvironmentWithClosureds(
//        outerEnv.globalEnv,
//        outerEnv,
//        name,
//        TemplatasStore(name, Map(), Map()).addEntries(interner, entries),
//        function,
//        variables)
//    ordinaryOrTemplatedLayer.evaluateOrdinaryFunctionFromNonCallForBanner(
//      newEnv, coutputs, callRange, verifyConclusions)
//  }
//
//  def evaluateOrdinaryClosureFunctionFromNonCallForHeader(
//    containingEnv: IEnvironment,
//    coutputs: CompilerOutputs,
//    parentRanges: List[RangeS],
//    closureStructRef: StructTT,
//    function: FunctionA,
//    verifyConclusions: Boolean):
//  (FunctionHeaderT) = {
//    // We dont here because it knows from how many variables
//    // it closures... but even lambdas without closured vars are still closures and are still
//    // backed by structs.
//    vassert(!function.isTemplate)
//
//    val name = makeNameWithClosureds(containingEnv, function.name)
//    val outerEnv = GeneralEnvironment.childOf(interner, containingEnv, name)
//
//    coutputs.declareFunction(name)
//    coutputs.declareFunctionOuterEnv(name, outerEnv)
//    val (variables, entries) = makeClosureVariablesAndEntries(coutputs, closureStructRef)
//    val newEnv =
//      env.BuildingFunctionEnvironmentWithClosureds(
//        outerEnv.globalEnv,
//        outerEnv,
//        name,
//        TemplatasStore(name, Map(), Map()).addEntries(interner, entries),
//        function,
//        variables)
//    ordinaryOrTemplatedLayer.evaluateOrdinaryFunctionFromNonCallForHeader(
//      newEnv, coutputs, parentRanges, verifyConclusions)
//  }
//
//  def evaluateOrdinaryClosureFunctionFromCallForPrototype(
//    outerEnv: IEnvironment,
//    coutputs: CompilerOutputs,
//    parentRanges: List[RangeS],
//    callingEnv: IEnvironment, // See CSSNCE
//    closureStructRef: StructTT,
//    function: FunctionA):
//  (PrototypeTemplata) = {
//    // We dont here because it knows from how many variables
//    // it closures... but even lambdas without closured vars are still closures and are still
//    // backed by structs.
//    vassert(!function.isTemplate)
//
//    val name = makeNameWithClosureds(outerEnv, function.name)
//
//    val (variables, entries) = makeClosureVariablesAndEntries(coutputs, closureStructRef)
//    val newEnv =
//      env.BuildingFunctionEnvironmentWithClosureds(
//        outerEnv.globalEnv,
//        outerEnv,
//        name,
//        TemplatasStore(name, Map(), Map()).addEntries(interner, entries),
//        function,
//        variables)
//    ordinaryOrTemplatedLayer.evaluateOrdinaryFunctionFromCallForPrototype(
//      newEnv, callingEnv, coutputs, parentRanges)
//  }
//
//  def evaluateTemplatedClosureFunctionFromNonCallForHeader(
//    outerEnv: IEnvironment,
//    coutputs: CompilerOutputs,
//    parentRanges: List[RangeS],
//    closureStructRef: StructTT,
//    function: FunctionA,
//    verifyConclusions: Boolean):
//  (FunctionHeaderT) = {
//    // We dont here because it knows from how many variables
//    // it closures... but even lambdas without closured vars are still closures and are still
//    // backed by structs.
//    vassert(!function.isTemplate)
//
//    val name = makeNameWithClosureds(outerEnv, function.name)
//    coutputs.declareFunction(parentRanges, name)
//    val (variables, entries) = makeClosureVariablesAndEntries(coutputs, closureStructRef)
//    val newEnv =
//      env.BuildingFunctionEnvironmentWithClosureds(
//        outerEnv.globalEnv,
//        outerEnv,
//        name,
//        TemplatasStore(name, Map(), Map()).addEntries(interner, entries),
//        function,
//        variables)
//    ordinaryOrTemplatedLayer.evaluateTemplatedFunctionFromNonCallForHeader(
//      newEnv, coutputs, parentRanges, verifyConclusions)
//  }

  // This is called while we're trying to figure out what function1s to call when there
  // are a lot of overloads available.
  // This assumes it met any type bound restrictions (or, will; not implemented yet)
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_templated_light_banner_from_call_closure_or_light(
        &self,
        parent_env: IEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        function: &'s FunctionA<'s>,
        explicit_template_args: &[ITemplataT<'s, 't>],
        context_region: RegionT,
        arg_types: &[CoordT<'s, 't>],
    ) -> Result<IEvaluateFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
        self.check_not_closure(function);

        let outer_env_id = parent_env.id().add_step(
            self.typing_interner,
            self.translate_generic_template_function_name(function.name, arg_types));
        let outer_env = self.make_env_without_closure_stuff(parent_env, function, outer_env_id, false);
        self.evaluate_templated_light_banner_from_call(
            outer_env, coutputs, calling_env, call_range, call_location,
            explicit_template_args, context_region, arg_types)
    }
/*
  def evaluateTemplatedLightBannerFromCall(
      parentEnv: IEnvironmentT,
      coutputs: CompilerOutputs,
      callingEnv: IInDenizenEnvironmentT, // See CSSNCE
      callRange: List[RangeS],
      callLocation: LocationInDenizen,
      function: FunctionA,
      explicitTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
      argTypes: Vector[CoordT]):
  (IEvaluateFunctionResult) = {
    checkNotClosure(function)

    val outerEnvId = parentEnv.id.addStep(nameTranslator.translateGenericTemplateFunctionName(function.name, argTypes))
    val outerEnv = makeEnvWithoutClosureStuff(parentEnv, function, outerEnvId, false)
    ordinaryOrTemplatedLayer.evaluateTemplatedLightBannerFromCall(
        outerEnv, coutputs, callingEnv, callRange, callLocation, explicitTemplateArgs, contextRegion, argTypes)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_templated_function_from_call_for_banner_closure_or_light(
        &self,
        parent_env: IInDenizenEnvironmentT,
        coutputs: CompilerOutputs,
        calling_env: IInDenizenEnvironmentT,
        function: FunctionA,
        call_range: Vec<RangeS>,
        call_location: LocationInDenizen,
        already_specified_template_args: Vec<ITemplataT>,
        context_region: RegionT,
        arg_types: Vec<CoordT>,
    ) -> IEvaluateFunctionResult<'_, '_> {
        panic!("Unimplemented: evaluate_templated_function_from_call_for_banner");
    }
/*
  def evaluateTemplatedFunctionFromCallForBanner(
      parentEnv: IInDenizenEnvironmentT,
      coutputs: CompilerOutputs,
      callingEnv: IInDenizenEnvironmentT, // See CSSNCE
      function: FunctionA,
      callRange: List[RangeS],
      callLocation: LocationInDenizen,
      alreadySpecifiedTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
      argTypes: Vector[CoordT]):
  (IEvaluateFunctionResult) = {
    val outerEnvId = parentEnv.id.addStep(nameTranslator.translateGenericFunctionName(function.name))
    val outerEnv = makeEnvWithoutClosureStuff(parentEnv, function, outerEnvId, false)
    ordinaryOrTemplatedLayer.evaluateTemplatedFunctionFromCallForBanner(
        outerEnv, coutputs, callingEnv, callRange, callLocation, alreadySpecifiedTemplateArgs, contextRegion, argTypes)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    fn make_env_without_closure_stuff(
        &self,
        outer_env: IEnvironmentT<'s, 't>,
        function: &'s FunctionA<'s>,
        template_id: &'t IdT<'s, 't>,
        is_root_compiling_denizen: bool,
    ) -> &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't> {
        let templatas = TemplatasStoreBuilder::new(template_id).build_in(self.typing_interner);
        self.typing_interner.alloc(BuildingFunctionEnvironmentWithClosuredsT {
            global_env: outer_env.global_env(),
            parent_env: outer_env,
            id: *template_id,
            templatas,
            function,
            variables: &[],
            is_root_compiling_denizen,
        })
    }
/*
  private def makeEnvWithoutClosureStuff(
    outerEnv: IEnvironmentT,
    function: FunctionA,
    templateId: IdT[IFunctionTemplateNameT],
    isRootCompilingDenizen: Boolean
  ): BuildingFunctionEnvironmentWithClosuredsT = {
    env.BuildingFunctionEnvironmentWithClosuredsT(
      outerEnv.globalEnv,
      outerEnv,
      templateId,
      TemplatasStore(templateId, Map(), Map()),
      function,
      Vector.empty,
      isRootCompilingDenizen)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    fn check_not_closure(&self, function: &'s FunctionA<'s>) {
        match &function.body {
            IBodyS::CodeBody(body1) => assert!(body1.body.closured_names.is_empty()),
            IBodyS::ExternBody(_) => {}
            IBodyS::GeneratedBody(_) => {}
            IBodyS::AbstractBody(_) => {}
        }
    }
/*
  private def checkNotClosure(function: FunctionA) = {
    function.body match {
      case CodeBodyS(body1) => vassert(body1.closuredNames.isEmpty)
      case ExternBodyS =>
      case GeneratedBodyS(_) =>
      case AbstractBodyS =>
      case _ => vfail()
    }
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    fn make_closure_variables_and_entries(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        original_calling_denizen_id: IdT<'s, 't>,
        closure_struct_ref: StructTT<'s, 't>,
    ) -> (Vec<IVariableT<'s, 't>>, Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)>) {
        let closure_struct_def = coutputs.lookup_struct(closure_struct_ref.id, self);
        let substituter = self.get_placeholder_substituter(
            self.opts.global_options.sanity_check,
            original_calling_denizen_id,
            closure_struct_ref.id,
            // This is a parameter, so we can grab bounds from it.
            IBoundArgumentsSource::InheritBoundsFromTypeItself,
        );
        let variables: Vec<IVariableT<'s, 't>> =
            closure_struct_def.members.iter().map(|member| {
                match member {
                    IStructMemberT::Normal(NormalStructMemberT { name: var_name, variability, tyype: IMemberTypeT::Reference(ReferenceMemberTypeT { reference }) }) => {
                        IVariableT::ReferenceClosure(ReferenceClosureVariableT {
                            name: *var_name,
                            closured_vars_struct_type: self.typing_interner.alloc(closure_struct_ref),
                            variability: *variability,
                            coord: substituter.substitute_for_coord(coutputs, *reference),
                        })
                    }
                    IStructMemberT::Normal(NormalStructMemberT { name: var_name, variability, tyype: IMemberTypeT::Address(AddressMemberTypeT { reference }) }) => {
                        IVariableT::AddressibleClosure(AddressibleClosureVariableT {
                            name: *var_name,
                            closured_vars_struct_type: self.typing_interner.alloc(closure_struct_ref),
                            variability: *variability,
                            coord: substituter.substitute_for_coord(coutputs, *reference),
                        })
                    }
                    IStructMemberT::Variadic(_) => panic!("implement: make_closure_variables_and_entries — VariadicStructMemberT"),
                }
            }).collect();
        let entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> = vec![
            (closure_struct_ref.id.local_name, IEnvEntryT::Templata(ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: KindT::Struct(self.typing_interner.alloc(closure_struct_ref)) })))),
        ];
        (variables, entries)
    }
/*
  private def makeClosureVariablesAndEntries(
      coutputs: CompilerOutputs,
      originalCallingDenizenId: IdT[ITemplateNameT],
      closureStructRef: StructTT):
  (Vector[IVariableT], Vector[(INameT, IEnvEntry)]) = {
    val closureStructDef = coutputs.lookupStruct(closureStructRef.id);
    val substituter =
      TemplataCompiler.getPlaceholderSubstituter(
        opts.globalOptions.sanityCheck,
        interner, keywords,
        originalCallingDenizenId,
        closureStructRef.id,
        // This is a parameter, so we can grab bounds from it.
        InheritBoundsFromTypeItself)
    val variables =
      closureStructDef.members.map(member => {
        val varName = member.name
        member match {
          case NormalStructMemberT(name, variability, ReferenceMemberTypeT(reference)) => {
            ReferenceClosureVariableT(
              varName, closureStructRef, variability, substituter.substituteForCoord(coutputs, reference))
          }
          case NormalStructMemberT(name, variability, AddressMemberTypeT(reference)) => {
            AddressibleClosureVariableT(
              varName, closureStructRef, variability, substituter.substituteForCoord(coutputs, reference))
          }
          case VariadicStructMemberT(name, tyype) => vimpl()
        }
      })
    val entries =
      Vector[(INameT, IEnvEntry)](
        closureStructRef.id.localName ->
          TemplataEnvEntry(KindTemplataT(closureStructRef)))
    (variables, entries)
  }
}
*/
}
