use std::collections::{HashMap, HashSet};
use crate::typing::compiler::Compiler;
use crate::typing::function::function_compiler::*;
use crate::typing::compilation::TypingPassOptions;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::typing::infer_compiler::{include_rule_in_definition_solve, InitialKnown, InitialSend, InferEnv, CompleteResolveSolve, CompleteDefineSolve, IResolvingError, IDefiningError};
use crate::typing::hinputs_t::InstantiationBoundArgumentsT;
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::postparsing::itemplatatype::ITemplataType;
use crate::utils::range::RangeS;
use crate::postparsing::names::*;
use crate::postparsing::ast::*;
use crate::postparsing::*;
use crate::postparsing::rules::*;
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
use crate::higher_typing::ast::*;
use crate::solver::solver::*;
use crate::interner::Interner;
use crate::keywords::Keywords;

/*
package dev.vale.typing.function

import dev.vale.{Err, Interner, Keywords, Ok, Profiler, RangeS, StrI, typing, vassert, vassertSome, vcurious, vfail, vimpl, vpass, vregionmut}
import dev.vale.highertyping.FunctionA
import dev.vale.postparsing.rules.{IRulexSR, RuneUsage}
import dev.vale.typing.citizen.StructCompiler
import dev.vale.postparsing.patterns._
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale.postparsing.{IEnvironmentS => _, _}
import dev.vale.typing.OverloadResolver._
import dev.vale.typing._
import dev.vale.typing.ast._
import dev.vale.typing.env._
import dev.vale.typing.function._
import dev.vale.solver.{FailedSolve, Solver, Step}
import dev.vale.typing.ast.{FunctionBannerT, FunctionHeaderT, PrototypeT}
import dev.vale.typing.env._
import dev.vale.typing.infer.ITypingPassSolverError
import dev.vale.typing.{CompilerOutputs, ConvertHelper, InferCompiler, InitialKnown, InitialSend, TemplataCompiler, TypingPassOptions}
import dev.vale.typing.names._
import dev.vale.typing.templata.ITemplataT._
import dev.vale.typing.templata._
import dev.vale.typing.types.CoordT
//import dev.vale.typingpass.infer.{InferSolveFailure, InferSolveSuccess}
import dev.vale.vwat

import scala.collection.immutable.{List, Set}

// When typingpassing a function, these things need to happen:
// - Spawn a local environment for the function
// - Add any closure args to the environment
// - Incorporate any template arguments into the environment
// There's a layer to take care of each of these things.
// This file is the outer layer, which spawns a local environment for the function.
*/
/*
class FunctionCompilerSolvingLayer(
    opts: TypingPassOptions,
    interner: Interner,
    keywords: Keywords,
    nameTranslator: NameTranslator,
    templataCompiler: TemplataCompiler,
    inferCompiler: InferCompiler,
    convertHelper: ConvertHelper,
    structCompiler: StructCompiler,
    delegate: IFunctionCompilerDelegate) {
  val middleLayer = new FunctionCompilerMiddleLayer(opts, interner, keywords, nameTranslator, templataCompiler, convertHelper, structCompiler, delegate)

  // We would want only the prototype instead of the entire header if, for example,
  // we were calling the function. This is necessary for a recursive function like
  // func main():Int{main()}
  // Preconditions:
  // - either no closured vars, or they were already added to the env.
  // - env is the environment the templated function was made in
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_templated_function_from_call_for_prototype_solving(
        &self,
        outer_env: &BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        original_calling_env: &IInDenizenEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        explicit_template_args: &[ITemplataT<'s, 't>],
        context_region: RegionT,
        args: &[CoordT<'s, 't>],
    ) -> IEvaluateFunctionResult<'s, 't> {
        panic!("Unimplemented: evaluate_templated_function_from_call_for_prototype");
    }

/*
  def evaluateTemplatedFunctionFromCallForPrototype(
    // The environment the function was defined in.
    outerEnv: BuildingFunctionEnvironmentWithClosuredsT,
    coutputs: CompilerOutputs,
    originalCallingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    explicitTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
    args: Vector[CoordT]):
  (IEvaluateFunctionResult) = {
    val function = outerEnv.function
    // Check preconditions
    checkClosureConcernsHandled(outerEnv)

    val callSiteRules =
        TemplataCompiler.assembleCallSiteRules(
            function.rules, function.genericParameters, explicitTemplateArgs.size)

    val initialSends = assembleInitialSendsFromArgs(callRange.head, function, args.map(Some(_)))
    val CompleteDefineSolve(inferredTemplatas, instantiationBoundParams) =
      inferCompiler.solveForDefining(
        InferEnv(originalCallingEnv, callRange, callLocation, outerEnv, contextRegion),
        coutputs,
        callSiteRules,
        function.runeToType,
        callRange,
        callLocation,
        assembleKnownTemplatas(function, explicitTemplateArgs),
        initialSends,
        Vector()
      ) match {
        case Err(e) => throw CompileErrorExceptionT(TypingPassDefiningError(callRange, e))
        case Ok(i) => (i)
      }

    val runedEnv =
      addRunedDataToNearEnv(
        outerEnv,
        function.genericParameters.map(_.rune.rune),
        inferredTemplatas,
        instantiationBoundParams.runeToCitizenRuneToReachablePrototype.values.flatMap(_.citizenRuneToReachablePrototype.values).toVector.map(PrototypeTemplataT(_)))

    val header =
      middleLayer.getOrEvaluateFunctionForHeader(
        outerEnv, runedEnv, coutputs, callRange, callLocation, function, instantiationBoundParams)

    // Lambdas cant have bounds, right?
    vcurious(instantiationBoundParams.runeToBoundPrototype.isEmpty)
    vcurious(instantiationBoundParams.runeToCitizenRuneToReachablePrototype.isEmpty)
    vcurious(instantiationBoundParams.runeToBoundImpl.isEmpty)
    val instantiationBoundArgs =
      InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT](
        instantiationBoundParams.runeToBoundPrototype,
        instantiationBoundParams.runeToCitizenRuneToReachablePrototype.map({ case (x, InstantiationReachableBoundArgumentsT(y)) =>
          x -> InstantiationReachableBoundArgumentsT[IFunctionNameT](y)
        }),
        instantiationBoundParams.runeToBoundImpl)
    coutputs.addInstantiationBounds(
      opts.globalOptions.sanityCheck,
      interner, outerEnv.denizenTemplateId,
      header.id, instantiationBoundArgs)
    EvaluateFunctionSuccess(PrototypeTemplataT(header.toPrototype), inferredTemplatas, instantiationBoundArgs)
  }

  // Preconditions:
  // - either no closured vars, or they were already added to the env.
  // - env is the environment the templated function was made in
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_templated_function_from_call_for_banner(
        &self,
        declaring_env: &BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        original_calling_env: &IInDenizenEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        already_specified_template_args: &[ITemplataT<'s, 't>],
        context_region: RegionT,
        args: &[CoordT<'s, 't>],
    ) -> IEvaluateFunctionResult<'s, 't> {
        panic!("Unimplemented: evaluate_templated_function_from_call_for_banner");
    }

/*
  def evaluateTemplatedFunctionFromCallForBanner(
      // The environment the function was defined in.
      declaringEnv: BuildingFunctionEnvironmentWithClosuredsT,
      coutputs: CompilerOutputs,
      originalCallingEnv: IInDenizenEnvironmentT, // See CSSNCE
      callRange: List[RangeS],
      callLocation: LocationInDenizen,
      alreadySpecifiedTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
      args: Vector[CoordT]):
  (IEvaluateFunctionResult) = {
    val function = declaringEnv.function
    // Check preconditions
    checkClosureConcernsHandled(declaringEnv)

    val callSiteRules =
        TemplataCompiler.assembleCallSiteRules(
            function.rules, function.genericParameters, 0)

    val initialSends = assembleInitialSendsFromArgs(callRange.head, function, args.map(Some(_)))
    val CompleteDefineSolve(inferredTemplatas, instantiationBoundParams) = {
      // We could probably just solveForResolving (see DBDAR) but seems more future-proof to solveForDefining.
      inferCompiler.solveForDefining(
        InferEnv(originalCallingEnv, callRange, callLocation, declaringEnv, contextRegion),
        coutputs,
        callSiteRules,
        function.runeToType,
        callRange,
        callLocation,
        assembleKnownTemplatas(function, alreadySpecifiedTemplateArgs),
        initialSends,
        Vector()
      ) match {
        case Err(e) => return EvaluateFunctionFailure(e)
        case Ok(i) => (i)
      }
    }

    val runedEnv =
      addRunedDataToNearEnv(
        declaringEnv,
        function.genericParameters.map(_.rune.rune),
        inferredTemplatas,
        instantiationBoundParams.runeToCitizenRuneToReachablePrototype.values.flatMap(_.citizenRuneToReachablePrototype.values).toVector.map(PrototypeTemplataT(_)))

    val prototype =
      middleLayer.getOrEvaluateTemplatedFunctionForBanner(
        declaringEnv, runedEnv, coutputs, callRange, callLocation, function, instantiationBoundParams)

    // Lambdas cant have bounds, right?
    vcurious(instantiationBoundParams.runeToBoundPrototype.isEmpty)
    vcurious(instantiationBoundParams.runeToCitizenRuneToReachablePrototype.isEmpty)
    vcurious(instantiationBoundParams.runeToBoundImpl.isEmpty)
    val instantiationBoundArgs =
      InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT](
        instantiationBoundParams.runeToBoundPrototype,
        instantiationBoundParams.runeToCitizenRuneToReachablePrototype.map({ case (x, InstantiationReachableBoundArgumentsT(y)) =>
          x -> InstantiationReachableBoundArgumentsT[IFunctionNameT](y)
        }),
        instantiationBoundParams.runeToBoundImpl)
    coutputs.addInstantiationBounds(
      opts.globalOptions.sanityCheck,
      interner, originalCallingEnv.denizenTemplateId,
      prototype.prototype.id, instantiationBoundArgs)
    EvaluateFunctionSuccess(prototype, inferredTemplatas, instantiationBoundArgs)
  }

  // This is called while we're trying to figure out what functionSs to call when there
  // are a lot of overloads available.
  // This assumes it met any type bound restrictions (or, will; not implemented yet)
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_templated_light_banner_from_call(
        &self,
        near_env: &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        original_calling_env: &'t IInDenizenEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        explicit_template_args: &[ITemplataT<'s, 't>],
        context_region: RegionT,
        args: &[CoordT<'s, 't>],
    ) -> IEvaluateFunctionResult<'s, 't> {
        let function = near_env.function;
        // Check preconditions
        match &function.body {
            IBodyS::CodeBody(body1) => assert!(body1.body.closured_names.is_empty()),
            _ => {}
        }

        let call_site_rules =
            self.assemble_call_site_rules(
                function.rules, function.generic_parameters, explicit_template_args.len() as i32);

        let initial_sends = self.assemble_initial_sends_from_args(call_range[0], function, &args.iter().map(|a| Some(*a)).collect::<Vec<_>>());
        let initial_knowns = self.assemble_known_templatas(function, explicit_template_args);

        let rune_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
            function.rune_to_type.iter().map(|(k, v)| (*k, *v)).collect();

        let call_range_t: &'t [RangeS<'s>] = self.typing_interner.alloc_slice_copy(call_range);

        // We could probably just solveForResolving (see DBDAR) but seems more future-proof to solveForDefining.
        let CompleteDefineSolve { conclusions: inferences, rune_to_bound: instantiation_bound_params } =
            match self.solve_for_defining(
                InferEnv {
                    original_calling_env,
                    parent_ranges: call_range_t,
                    call_location,
                    self_env: near_env.into(),
                    context_region,
                },
                coutputs,
                &call_site_rules,
                &rune_to_type,
                call_range_t,
                call_location,
                &initial_knowns,
                &initial_sends,
                &[],
            ) {
                Err(e) => return IEvaluateFunctionResult::EvaluateFunctionFailure(EvaluateFunctionFailure { reason: e }),
                Ok(inferred_templatas) => inferred_templatas,
            };

        // See FunctionCompiler doc for what outer/runes/inner envs are.
        let reachable_bounds: Vec<PrototypeTemplataT<'s, 't>> =
            instantiation_bound_params.rune_to_citizen_rune_to_reachable_prototype.values()
                .flat_map(|r| {
                    panic!("implement: evaluate_templated_light_banner_from_call reachable bounds");
                    #[allow(unreachable_code)]
                    std::iter::empty::<PrototypeTemplataT<'s, 't>>()
                })
                .collect();

        // Rust adaptation (SPDMX-B): arena-allocate so callee can borrow as &'t; Scala relies on GC.
        let runed_env: &'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> =
            self.typing_interner.alloc(self.add_runed_data_to_near_env(
                near_env,
                &function.generic_parameters.iter().map(|gp| gp.rune.rune).collect::<Vec<_>>(),
                &inferences,
                &reachable_bounds));

        let prototype_templata =
            self.get_or_evaluate_templated_function_for_banner(
                near_env, runed_env, coutputs, call_range_t, call_location, function, instantiation_bound_params);

        // Lambdas cant have bounds, right?
        assert!(instantiation_bound_params.rune_to_bound_prototype.is_empty(), "vcurious");
        assert!(instantiation_bound_params.rune_to_citizen_rune_to_reachable_prototype.is_empty(), "vcurious");
        assert!(instantiation_bound_params.rune_to_bound_impl.is_empty(), "vcurious");
        let instantiation_bound_args = self.typing_interner.alloc(InstantiationBoundArgumentsT {
            rune_to_bound_prototype: ArenaIndexMap::new_in(self.typing_interner.bump()),
            rune_to_citizen_rune_to_reachable_prototype: ArenaIndexMap::new_in(self.typing_interner.bump()),
            rune_to_bound_impl: ArenaIndexMap::new_in(self.typing_interner.bump()),
        });
        coutputs.add_instantiation_bounds(
            self.opts.global_options.sanity_check,
            self.typing_interner,
            original_calling_env.denizen_template_id(),
            prototype_templata.prototype.id,
            instantiation_bound_args);
        IEvaluateFunctionResult::EvaluateFunctionSuccess(EvaluateFunctionSuccess {
            prototype: self.typing_interner.alloc(prototype_templata),
            inferences,
            instantiation_bound_args,
        })
    }

/*
  def evaluateTemplatedLightBannerFromCall(
      // The environment the function was defined in.
      nearEnv: BuildingFunctionEnvironmentWithClosuredsT,
      coutputs: CompilerOutputs,
    originalCallingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
      explicitTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
      args: Vector[CoordT]):
  (IEvaluateFunctionResult) = {
    val function = nearEnv.function
    // Check preconditions
    function.body match {
      case CodeBodyS(body1) => vassert(body1.closuredNames.isEmpty)
      case _ =>
    }

    val callSiteRules =
      TemplataCompiler.assembleCallSiteRules(
        function.rules, function.genericParameters, explicitTemplateArgs.size)

    val initialSends = assembleInitialSendsFromArgs(callRange.head, function, args.map(Some(_)))
    val initialKnowns = assembleKnownTemplatas(function, explicitTemplateArgs)
    val CompleteDefineSolve(inferences, instantiationBoundParams) =
    // We could probably just solveForResolving (see DBDAR) but seems more future-proof to solveForDefining.
      inferCompiler.solveForDefining(
        InferEnv(originalCallingEnv, callRange, callLocation, nearEnv, contextRegion),
        coutputs,
        callSiteRules,
        function.runeToType,
        callRange,
        callLocation,
        initialKnowns,
        initialSends,
        Vector()) match {
      case Err(e) => return EvaluateFunctionFailure(e)
      case Ok(inferredTemplatas) => inferredTemplatas
    }

    // See FunctionCompiler doc for what outer/runes/inner envs are.
    val runedEnv =
      addRunedDataToNearEnv(
        nearEnv,
        function.genericParameters.map(_.rune.rune),
        inferences,
        instantiationBoundParams.runeToCitizenRuneToReachablePrototype.values.flatMap(_.citizenRuneToReachablePrototype.values).toVector.map(PrototypeTemplataT(_)))

    val prototypeTemplata =
      middleLayer.getOrEvaluateTemplatedFunctionForBanner(
        nearEnv, runedEnv, coutputs, callRange, callLocation, function, instantiationBoundParams)

    // Lambdas cant have bounds, right?
    vcurious(instantiationBoundParams.runeToBoundPrototype.isEmpty)
    vcurious(instantiationBoundParams.runeToCitizenRuneToReachablePrototype.isEmpty)
    vcurious(instantiationBoundParams.runeToBoundImpl.isEmpty)
    val instantiationBoundArgs =
      InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT](
        instantiationBoundParams.runeToBoundPrototype,
        instantiationBoundParams.runeToCitizenRuneToReachablePrototype.map({ case (x, InstantiationReachableBoundArgumentsT(y)) =>
          x -> InstantiationReachableBoundArgumentsT[IFunctionNameT](y)
        }),
        instantiationBoundParams.runeToBoundImpl)
    coutputs.addInstantiationBounds(
      opts.globalOptions.sanityCheck,
      interner, originalCallingEnv.denizenTemplateId,
      prototypeTemplata.prototype.id, instantiationBoundArgs)
    EvaluateFunctionSuccess(prototypeTemplata, inferences, instantiationBoundArgs)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn assemble_known_templatas(
        &self,
        function: &FunctionA<'s>,
        explicit_template_args: &[ITemplataT<'s, 't>],
    ) -> Vec<InitialKnown<'s, 't>> {
        function.generic_parameters.iter()
            .zip(explicit_template_args.iter())
            .map(|(generic_param, explicit_arg)| {
                InitialKnown {
                    rune: generic_param.rune,
                    templata: *explicit_arg,
                }
            })
            .collect()
    }

/*
  private def assembleKnownTemplatas(
    function: FunctionA,
    explicitTemplateArgs: Vector[ITemplataT[ITemplataType]]):
  Vector[InitialKnown] = {
    function.genericParameters.zip(explicitTemplateArgs).map({
      case (genericParam, explicitArg) => {
        InitialKnown(genericParam.rune, explicitArg)
      }
    })
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn check_closure_concerns_handled(
        &self,
        near_env: &BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
    ) {
        let function = near_env.function;
        match &function.body {
            IBodyS::CodeBody(code_body) => {
                for _name in code_body.body.closured_names.iter() {
                    panic!("Unimplemented: check_closure_concerns_handled — closured name assertion");
                }
            }
            _ => {}
        }
    }

/*
  private def checkClosureConcernsHandled(
    // The environment the function was defined in.
    nearEnv: BuildingFunctionEnvironmentWithClosuredsT
  ): Unit = {
    val function = nearEnv.function
    function.body match {
      case CodeBodyS(body1) => {
        body1.closuredNames.foreach(name => {
          vassert(nearEnv.variables.exists(_.name == nameTranslator.translateNameStep(name)))
        })
      }
      case _ =>
    }
  }

  // IOW, add the necessary data to turn the near env into the runed env.
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn add_runed_data_to_near_env(
        &self,
        near_env: &BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
        identifying_runes: &[IRuneS<'s>],
        templatas_by_rune: &std::collections::HashMap<IRuneS<'s>, ITemplataT<'s, 't>>,
        reachable_bounds_from_params_and_return: &[PrototypeTemplataT<'s, 't>],
    ) -> BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> {
        let identifying_templatas: Vec<ITemplataT<'s, 't>> =
            identifying_runes.iter().map(|r| *templatas_by_rune.get(r).unwrap()).collect();

        // reachableBoundsFromParamsAndReturn.zipWithIndex.toVector
        //   .map({ case (t, i) => (interner.intern(ReachablePrototypeNameT(i)), TemplataEnvEntry(t)) }) ++
        // templatasByRune.toVector
        //   .map({ case (k, v) => (interner.intern(RuneNameT(k)), TemplataEnvEntry(v)) })
        let entries_list: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> =
            reachable_bounds_from_params_and_return.iter().enumerate()
                .map(|(i, t)| -> (INameT<'s, 't>, IEnvEntryT<'s, 't>) {
                    panic!("Unimplemented: add_runed_data_to_near_env ReachablePrototypeNameT");
                })
                .chain(
                    templatas_by_rune.iter()
                        .map(|(k, v)| {
                            let rune_name = self.typing_interner.intern_rune_name(RuneNameT { rune: *k, _phantom: std::marker::PhantomData });
                            (INameT::Rune(rune_name), IEnvEntryT::Templata(*v))
                        })
                )
                .collect();

        // newEntries = templatas.addEntries(interner, entries_list)
        let new_entries = self.typing_interner.alloc(near_env.templatas.add_entries(self.typing_interner, self.scout_arena, entries_list));

        let default_region = RegionT;

        let template_args: &'t [ITemplataT<'s, 't>] = self.typing_interner.alloc_slice_from_vec(identifying_templatas);
        BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT {
            global_env: near_env.global_env,
            parent_env: near_env.parent_env,
            id: near_env.id,
            template_args,
            templatas: new_entries,
            function: near_env.function,
            variables: near_env.variables,
            is_root_compiling_denizen: near_env.is_root_compiling_denizen,
            default_region,
        }
    }

/*
  private def addRunedDataToNearEnv(
    nearEnv: BuildingFunctionEnvironmentWithClosuredsT,
    identifyingRunes: Vector[IRuneS],
    templatasByRune: Map[IRuneS, ITemplataT[ITemplataType]],
    reachableBoundsFromParamsAndReturn: Vector[PrototypeTemplataT[IFunctionNameT]]
    // I suspect we'll eventually need some impl bounds here
  ): BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT = {
    val BuildingFunctionEnvironmentWithClosuredsT(globalEnv, parentEnv, id, templatas, function, variables, isRootCompilingDenizen) = nearEnv

    val identifyingTemplatas = identifyingRunes.map(templatasByRune)

    val newEntries =
      templatas.addEntries(
        interner,
        reachableBoundsFromParamsAndReturn.zipWithIndex.toVector
          .map({ case (t, i) => (interner.intern(ReachablePrototypeNameT(i)), TemplataEnvEntry(t)) }) ++
        templatasByRune.toVector
          .map({ case (k, v) => (interner.intern(RuneNameT(k)), TemplataEnvEntry(v)) }))

    val defaultRegion = RegionT()

    BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT(
      globalEnv,
      parentEnv,
      id,
      identifyingTemplatas,
      newEntries,
      function,
      variables,
      isRootCompilingDenizen,
      defaultRegion)
  }

  // We would want only the prototype instead of the entire header if, for example,
  // we were calling the function. This is necessary for a recursive function like
  // func main():Int{main()}
  // Preconditions:
  // - either no closured vars, or they were already added to the env.
  // - env is the environment the templated function was made in
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_generic_function_from_call_for_prototype(
        &self,
        outer_env: &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: &'t IInDenizenEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        explicit_template_args: &[ITemplataT<'s, 't>],
        context_region: RegionT,
        args: &[Option<CoordT<'s, 't>>],
    ) -> IResolveFunctionResult<'s, 't> {
        let function = outer_env.function;
        self.check_closure_concerns_handled(outer_env);

        let call_site_rules = self.assemble_call_site_rules(
            function.rules, function.generic_parameters, explicit_template_args.len() as i32);

        let initial_sends = self.assemble_initial_sends_from_args(call_range[0], function, args);

        // Rust adaptation (SPDMX-B): re-allocate call_range into the typing arena to satisfy
        // InferEnv's `&'t [RangeS<'s>]` field. Scala doesn't need this because GC.
        let call_range_t = self.typing_interner.alloc_slice_copy(call_range);
        let envs = InferEnv {
            original_calling_env: calling_env,
            parent_ranges: call_range_t,
            call_location,
            self_env: IEnvironmentT::BuildingWithClosureds(outer_env),
            context_region,
        };
        let rune_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
            function.rune_to_type.iter().map(|(k, v)| (*k, *v)).collect();
        let invocation_range = call_range;
        let initial_knowns = self.assemble_known_templatas(function, explicit_template_args);
        let include_reachable_bounds_for_runes: Vec<IRuneS<'s>> =
            function.params.iter()
                .flat_map(|p| p.pattern.coord_rune.map(|ru| ru.rune))
                .chain(function.maybe_ret_coord_rune.map(|ru| ru.rune))
                .collect();

        let mut solver = self.make_solver_state(
            envs, coutputs, &call_site_rules, &rune_to_type, invocation_range, &initial_knowns, &initial_sends);

        let loop_check = function.generic_parameters.len() as i32 + 1;

        match self.incrementally_solve(
            envs, coutputs, &mut solver,
            |solver_state| {
                panic!("implement: evaluateGenericFunctionFromCallForPrototype incrementallySolve callback");
            },
        ) {
            Err(f) => {
                return IResolveFunctionResult::ResolveFunctionFailure(ResolveFunctionFailure {
                    reason: IResolvingError::ResolvingSolveFailedOrIncomplete(f),
                });
            }
            Ok(true) => {}
            Ok(false) => {} // Incomplete, will be detected as SolveIncomplete below.
        }

        let CompleteResolveSolve { conclusions: inferred_templatas, rune_to_bound: rune_to_function_bound } =
            match self.check_resolving_conclusions_and_resolve(
                envs, coutputs, invocation_range, call_location, &rune_to_type, &call_site_rules, &include_reachable_bounds_for_runes, &mut solver,
            ) {
                Err(e) => {
                    return IResolveFunctionResult::ResolveFunctionFailure(ResolveFunctionFailure {
                        reason: e,
                    });
                }
                Ok(i) => i,
            };

        let identifying_runes: Vec<IRuneS<'s>> =
            function.generic_parameters.iter().map(|gp| gp.rune.rune).collect();
        let reachable_bound_protos: Vec<PrototypeTemplataT<'s, 't>> =
            rune_to_function_bound.rune_to_citizen_rune_to_reachable_prototype.iter()
                .flat_map(|(_rune, x)| {
                    panic!("implement: evaluateGenericFunctionFromCallForPrototype reachable_bound_protos");
                    #[allow(unreachable_code)]
                    std::iter::empty::<PrototypeTemplataT<'s, 't>>()
                })
                .collect();
        let runed_env = self.typing_interner.alloc(self.add_runed_data_to_near_env(
            outer_env, &identifying_runes, &inferred_templatas, &reachable_bound_protos));

        let prototype = self.get_generic_function_prototype_from_call(
            runed_env, coutputs, call_range, function);

        let prototype_templata = self.typing_interner.alloc(PrototypeTemplataT { prototype: self.typing_interner.alloc(prototype) });

        coutputs.add_instantiation_bounds(
            self.opts.global_options.sanity_check,
            self.typing_interner,
            calling_env.root_compiling_denizen_env().denizen_template_id(),
            prototype.id,
            self.typing_interner.alloc(rune_to_function_bound),
        );

        IResolveFunctionResult::ResolveFunctionSuccess(ResolveFunctionSuccess {
            prototype: prototype_templata,
            inferences: inferred_templatas,
        })
    }
/*
Guardian: temp-disable: SPDMX — The omitted Scala block is `outerEnv.id match { case IdT(_,Vector(),FunctionTemplateNameT(StrI("Bork"),_)) => vpass(); case _ => }` — a no-op debugging guardrail (vpass is a breakpoint-only function). It has no logical effect and is Exception F (debugging-only code). — FrontendRust/guardian-logs/request-254-1777775550299/hook-254/evaluate_generic_function_from_call_for_prototype--516.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def evaluateGenericFunctionFromCallForPrototype(
    // The environment the function was defined in.
    outerEnv: BuildingFunctionEnvironmentWithClosuredsT,
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    explicitTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
    args: Vector[Option[CoordT]]):
  (IResolveFunctionResult) = {
    val function = outerEnv.function
    // Check preconditions
    checkClosureConcernsHandled(outerEnv)

    val callSiteRules =
        TemplataCompiler.assembleCallSiteRules(
            function.rules, function.genericParameters, explicitTemplateArgs.size)

    val initialSends = assembleInitialSendsFromArgs(callRange.head, function, args)


    val envs = InferEnv(callingEnv, callRange, callLocation, outerEnv, contextRegion)
    val rules = callSiteRules
    val runeToType = function.runeToType
    val invocationRange = callRange
    val initialKnowns = assembleKnownTemplatas(function, explicitTemplateArgs)
    val includeReachableBoundsForRunes =
      function.params.flatMap(_.pattern.coordRune.map(_.rune)) ++ function.maybeRetCoordRune.map(_.rune)

    val solver =
      inferCompiler.makeSolverState(envs, coutputs, rules, runeToType, invocationRange, initialKnowns, initialSends)

    var loopCheck = function.genericParameters.size + 1

    // Incrementally solve and add default generic parameters (and context region).
    inferCompiler.incrementallySolve(
      envs, coutputs, solver,
      (solverState) => {
        if (loopCheck == 0) {
          throw CompileErrorExceptionT(RangedInternalErrorT(callRange, "Infinite loop detected in incremental call solve!"))
        }
        loopCheck = loopCheck - 1

        TemplataCompiler.getFirstUnsolvedIdentifyingRune(
          function.genericParameters,
          (rune) => solverState.getConclusion(rune).nonEmpty) match {
          case None => false
          case Some((genericParam, index)) => {
            // This unsolved rune better be one we didn't explicitly hand in already.
            vassert(index >= explicitTemplateArgs.size)

            genericParam.default match {
              case Some(defaultRules) => {
                solverState.commitStep[ITypingPassSolverError](false, Vector(), Map(), defaultRules.rules).getOrDie()
                true
              }
              case None => {
                // There are no defaults for this.
                false
              }
            }
          }
        }
      }) match {
      case Err(f@FailedSolve(_, _, _, _, _)) => {
        return (ResolveFunctionFailure(ResolvingSolveFailedOrIncomplete(f)))
      }
      case Ok(true) =>
      case Ok(false) => // Incomplete, will be detected as SolveIncomplete below.
    }

    outerEnv.id match {
      case IdT(_,Vector(),FunctionTemplateNameT(StrI("Bork"),_)) => {
        vpass()
      }
      case _ =>
    }

    val CompleteResolveSolve(inferredTemplatas, runeToFunctionBound) =
      inferCompiler.checkResolvingConclusionsAndResolve(
        envs, coutputs, invocationRange, callLocation, runeToType, rules, includeReachableBoundsForRunes, solver) match {
        case Err(e) => return (ResolveFunctionFailure(e))
        case Ok(i) => (i)
      }

    val runedEnv =
      addRunedDataToNearEnv(
        outerEnv,
        function.genericParameters.map(_.rune.rune),
        inferredTemplatas,
        runeToFunctionBound.runeToCitizenRuneToReachablePrototype.values.flatMap(_.citizenRuneToReachablePrototype.values).toVector.map(PrototypeTemplataT(_)))

    val prototype =
      middleLayer.getGenericFunctionPrototypeFromCall(
        runedEnv, coutputs, callRange, function)

    coutputs.addInstantiationBounds(
      opts.globalOptions.sanityCheck,
      interner, callingEnv.rootCompilingDenizenEnv.denizenTemplateId,
      prototype.id, runeToFunctionBound)

    ResolveFunctionSuccess(PrototypeTemplataT(prototype), inferredTemplatas)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_generic_virtual_dispatcher_function_for_prototype_solving(
        &self,
        near_env: &BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: &IInDenizenEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        args: &[Option<CoordT<'s, 't>>],
    ) -> IDefineFunctionResult<'s, 't> {
        panic!("Unimplemented: evaluate_generic_virtual_dispatcher_function_for_prototype");
    }

/*
  def evaluateGenericVirtualDispatcherFunctionForPrototype(
    // The environment the function was defined in.
    nearEnv: BuildingFunctionEnvironmentWithClosuredsT,
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    args: Vector[Option[CoordT]]):
  IDefineFunctionResult = {
    val function = nearEnv.function
    // Check preconditions
    checkClosureConcernsHandled(nearEnv)

    val functionDefinitionRules =
      function.rules.filter(InferCompiler.includeRuleInDefinitionSolve)

    val initialSends = assembleInitialSendsFromArgs(callRange.head, function, args)

    // This is so that we can feed in the self interface to see what it indirectly determines.
    // It will turn a:
    //   func map<T, F>(self Opt<T>, f F, t T) { ... }
    // into a:
    //   func map<F>(self Opt<$0>, f F, t $0) { ... }
    val preliminaryEnvs = InferEnv(callingEnv, callRange, callLocation, nearEnv, RegionT())
    val preliminarySolverState =
      inferCompiler.makeSolverState(
        preliminaryEnvs,
        coutputs,
        functionDefinitionRules,
        function.runeToType,
        function.range :: callRange,
        Vector(),
        initialSends)
    inferCompiler.continue(preliminaryEnvs, coutputs, preliminarySolverState) match {
      case Ok(()) =>
      case Err(f) => {
        throw CompileErrorExceptionT(typing.TypingPassSolverError(function.range :: callRange, f))
      }
    }

    // Skip checking that the conclusions are all there, because we don't assume that they will all be there. We expect
    // an incomplete solve.
    val preliminaryInferences = preliminarySolverState.userifyConclusions().toMap
    // Now we can use preliminaryInferences to know whether or not we need a placeholder for an
    // identifying rune.
    // Our
    //   func map<F>(self Opt<$0>, f F) { ... }
    // will need one placeholder, for F.

    val placeholderInitialKnownsFromFunction =
      function.genericParameters.zipWithIndex.flatMap({ case (genericParam, index) =>
        preliminaryInferences.get(genericParam.rune.rune) match {
          case Some(x) => Some(InitialKnown(genericParam.rune, x))
          case None => {
            // Make a placeholder for every argument even if it has a default, see DUDEWCD.
//            val runeType = vassertSome(function.runeToType.get(genericParam.rune.rune))
            vimpl()
            val placeholderPureHeight = vregionmut(None)
            val templata =
              templataCompiler.createPlaceholder(
                coutputs, callingEnv, callingEnv.id, genericParam, index, function.runeToType, placeholderPureHeight, false)
            Some(InitialKnown(genericParam.rune, templata))
          }
        }
      })

    // Now that we have placeholders, let's do the rest of the solve, so we can get a full
    // prototype out of it.

    val CompleteDefineSolve(inferences, instantiationBoundParams) =
      inferCompiler.solveForDefining(
        InferEnv(callingEnv, callRange, callLocation, nearEnv, RegionT()),
        coutputs,
        functionDefinitionRules,
        function.runeToType,
        function.range :: callRange,
        callLocation,
        placeholderInitialKnownsFromFunction,
        Vector(),
        function.params.flatMap(_.pattern.coordRune.map(_.rune)) ++ function.maybeRetCoordRune.map(_.rune)) match {
        case Err(f) => throw CompileErrorExceptionT(TypingPassDefiningError(function.range :: callRange, f))
        case Ok(c) => c
      }
    val runedEnv =
      addRunedDataToNearEnv(
        nearEnv,
        function.genericParameters.map(_.rune.rune),
        inferences,
        instantiationBoundParams.runeToCitizenRuneToReachablePrototype.values.flatMap(_.citizenRuneToReachablePrototype.values).toVector.map(PrototypeTemplataT(_)))

    val prototype =
      middleLayer.getGenericFunctionPrototypeFromCall(
        runedEnv, coutputs, callRange, function)

    // Usually when we call a function, we add instantiation bounds. However, we're
    // not calling a function here, we're defining it.
    DefineFunctionSuccess(PrototypeTemplataT(prototype), inferences, instantiationBoundParams)
  }

  // Preconditions:
  // - either no closured vars, or they were already added to the env.
*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_generic_function_from_non_call_solving(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        near_env: &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
    ) -> &'t FunctionHeaderT<'s, 't> {
        let function = near_env.function;

        let mut range: Vec<RangeS<'s>> = Vec::with_capacity(1 + parent_ranges.len());
        range.push(function.range);
        range.extend_from_slice(parent_ranges);
        self.check_closure_concerns_handled(near_env);

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
        let _function_template_id = near_env.parent_env.id().add_step(self.typing_interner, function_name_local);

        let definition_rules: Vec<&'s IRulexSR<'s>> = function.rules.iter()
            .filter(|r| include_rule_in_definition_solve(*r))
            .collect();

        let mut seen = HashSet::new();
        let mut param_and_return_runes: Vec<IRuneS<'s>> = Vec::new();
        for param in function.params.iter() {
            if let Some(coord_rune) = param.pattern.coord_rune {
                if seen.insert(coord_rune.rune) {
                    param_and_return_runes.push(coord_rune.rune);
                }
            }
        }
        if let Some(ret_coord_rune) = function.maybe_ret_coord_rune {
            if seen.insert(ret_coord_rune.rune) {
                param_and_return_runes.push(ret_coord_rune.rune);
            }
        }

        let parent_ranges_alloc = self.typing_interner.alloc_slice_from_vec(parent_ranges.to_vec());
        let near_env_as_in_denizen = self.typing_interner.alloc(
            IInDenizenEnvironmentT::BuildingWithClosureds(near_env));
        let near_env_as_env = IEnvironmentT::BuildingWithClosureds(near_env);
        let envs = InferEnv {
            original_calling_env: near_env_as_in_denizen,
            parent_ranges: parent_ranges_alloc,
            call_location,
            self_env: near_env_as_env,
            context_region: RegionT,
        };

        let rune_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> = function.rune_to_type.iter()
            .map(|(k, v)| (*k, *v))
            .collect();
        let mut solver = self.make_solver_state(
            envs, coutputs, &definition_rules, &rune_to_type, &range, &[], &[]);

        let result = self.incrementally_solve(
            envs, coutputs, &mut solver,
            |_solver_state| { panic!("Unimplemented: incrementally_solve on_incomplete callback") });
        match result {
            Err(_f) => { panic!("Unimplemented: FailedSolve handling in evaluate_generic_function_from_non_call_solving") }
            Ok(true) => {}
            Ok(false) => {} // Incomplete, will be detected in checkDefiningConclusionsAndResolve
        }

        let inferences = match self.interpret_results(&rune_to_type, &mut solver) {
            Err(_e) => { panic!("Unimplemented: interpretResults error handling") }
            Ok(conclusions) => conclusions,
        };

        let instantiation_bound_params = match self.check_defining_conclusions_and_resolve(
            envs, coutputs, &range, call_location, &definition_rules, &param_and_return_runes, &inferences,
        ) {
            Err(_f) => { panic!("Unimplemented: checkDefiningConclusionsAndResolve error handling") }
            Ok(c) => c,
        };

        let identifying_runes: Vec<IRuneS<'s>> = function.generic_parameters.iter()
            .map(|gp| gp.rune.rune)
            .collect();
        let reachable_bounds: Vec<PrototypeTemplataT<'s, 't>> =
            instantiation_bound_params.rune_to_citizen_rune_to_reachable_prototype.iter()
                .flat_map(|(_, rb)| rb.citizen_rune_to_reachable_prototype.iter().map(|(_, proto)| proto))
                .map(|proto| PrototypeTemplataT { prototype: self.typing_interner.alloc(*proto) })
                .collect();
        let runed_env = self.add_runed_data_to_near_env(
            near_env, &identifying_runes, &inferences, &reachable_bounds);
        let runed_env: &'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> =
            self.typing_interner.alloc(runed_env);

        let header = self.get_or_evaluate_function_for_header(
            near_env, runed_env, coutputs, parent_ranges, call_location, function, instantiation_bound_params);

        header
    }

/*
  def evaluateGenericFunctionFromNonCall(
    coutputs: CompilerOutputs,
    nearEnv: BuildingFunctionEnvironmentWithClosuredsT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen
  ): FunctionHeaderT = {
    val function = nearEnv.function
    val range = function.range :: parentRanges
    // Check preconditions
    checkClosureConcernsHandled(nearEnv)

    val functionTemplateId =
      nearEnv.parentEnv.id.addStep(
        nameTranslator.translateGenericFunctionName(nearEnv.function.name))

    val definitionRules = function.rules.filter(InferCompiler.includeRuleInDefinitionSolve)

    // This is so we can automatically grab the bounds from parameters and returns, see NBIFP.
    val paramAndReturnRunes =
      (function.params.flatMap(_.pattern.coordRune.map(_.rune)) ++ function.maybeRetCoordRune.map(_.rune)).distinct.toVector

    val envs = InferEnv(nearEnv, parentRanges, callLocation, nearEnv, RegionT())
    val solver =
      inferCompiler.makeSolverState(
        envs, coutputs, definitionRules, function.runeToType, range, Vector(), Vector())
    // Incrementally solve and add placeholders, see IRAGP.
    inferCompiler.incrementallySolve(
      envs, coutputs, solver,
      // Each step happens after the solver has done all it possibly can. Sometimes this can lead
      // to races, see RRBFS.
      (solverState) => {
        TemplataCompiler.getFirstUnsolvedIdentifyingRune(function.genericParameters, (rune) => solverState.getConclusion(rune).nonEmpty) match {
          case None => false
          case Some((genericParam, index)) => {
            // Make a placeholder for every argument even if it has a default, see DUDEWCD.
            val placeholderPureHeight = vregionmut(None)
            val templata =
              templataCompiler.createPlaceholder(
                coutputs, nearEnv, functionTemplateId, genericParam, index, function.runeToType, placeholderPureHeight, true)
            { // solver.manualStep(Map(genericParam.rune.rune -> templata))
              solverState.commitStep[Nothing](false, Vector(), Map(genericParam.rune.rune -> templata), Vector()).getOrDie()
//              solverState.addStep(step)
//              step.conclusions.foreach({ case (rune, conclusion) =>
//                solverState.concludeRune(solverState.getCanonicalRune(rune), conclusion)
//              })
            }
            true
          }
        }
      }) match {
        case Err(f @ FailedSolve(_, _, _, _, err)) => {
          throw CompileErrorExceptionT(typing.TypingPassSolverError(function.range :: parentRanges, f))
        }
        case Ok(true) =>
        case Ok(false) => // Incomplete, will be detected in the below checkDefiningConclusionsAndResolve
      }
    val inferences =
      inferCompiler.interpretResults(function.runeToType, solver) match {
        case Err(e) => throw CompileErrorExceptionT(typing.TypingPassSolverError(function.range :: parentRanges, e))
        case Ok(conclusions) => conclusions
      }

    nearEnv.id match {
      case IdT(_,Vector(),FunctionTemplateNameT(StrI("Bork"),_)) => {
        vpass()
      }
      case _ =>
    }

    val instantiationBoundParams =
      inferCompiler.checkDefiningConclusionsAndResolve(
        envs, coutputs, range, callLocation, definitionRules, paramAndReturnRunes, inferences) match {
        case Err(f) => throw CompileErrorExceptionT(TypingPassDefiningError(range, DefiningResolveConclusionError(f)))
        case Ok(c) => c
      }

    val runedEnv =
      addRunedDataToNearEnv(
        nearEnv, function.genericParameters.map(_.rune.rune), inferences,
        instantiationBoundParams.runeToCitizenRuneToReachablePrototype.values.flatMap(_.citizenRuneToReachablePrototype.values).toVector.map(PrototypeTemplataT(_)))

    val header =
      middleLayer.getOrEvaluateFunctionForHeader(
        nearEnv, runedEnv, coutputs, parentRanges, callLocation, function, instantiationBoundParams)

    // We don't add these here because we aren't instantiating anything here, we're compiling a
    // function
    // not calling it.
    // coutputs.addInstantiationBounds(header.toPrototype.id, runeToFunctionBound)

    header
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn assemble_initial_sends_from_args(
        &self,
        call_range: RangeS<'s>,
        function: &FunctionA<'s>,
        args: &[Option<CoordT<'s, 't>>],
    ) -> Vec<InitialSend<'s, 't>> {
        function.params.iter()
            .map(|p| p.pattern.coord_rune.unwrap())
            .zip(args.iter())
            .enumerate()
            .flat_map(|(arg_index, (param_rune, arg))| {
                match arg {
                    None => None,
                    Some(arg_templata) => {
                        let sender_rune = RuneUsage {
                            range: call_range,
                            rune: self.scout_arena.intern_rune(
                                IRuneValS::ArgumentRune(ArgumentRuneS { arg_index: arg_index as i32 })),
                        };
                        Some(InitialSend {
                            sender_rune,
                            receiver_rune: param_rune,
                            send_templata: ITemplataT::Coord(
                                self.typing_interner.alloc(CoordTemplataT { coord: *arg_templata })),
                        })
                    }
                }
            })
            .collect()
    }

/*
  private def assembleInitialSendsFromArgs(callRange: RangeS, function: FunctionA, args: Vector[Option[CoordT]]):
  Vector[InitialSend] = {
    function.params.map(_.pattern.coordRune.get).zip(args).zipWithIndex
      .flatMap({
        case ((_, None), _) => None
        case ((paramRune, Some(argTemplata)), argIndex) => {
          Some(InitialSend(RuneUsage(callRange, ArgumentRuneS(argIndex)), paramRune, CoordTemplataT(argTemplata)))
        }
      })
  }
}
*/
}
