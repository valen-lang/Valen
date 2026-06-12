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
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::higher_typing::ast::*;
use crate::solver::solver::*;
use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::typing::infer_compiler::IConclusionResolveError;
use crate::typing::infer::compiler_solver::ITypingPassSolverError;
use std::iter::empty;
use std::marker::PhantomData;

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
        original_calling_env: IInDenizenEnvironmentT<'s, 't>,
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
        TemplataCompiler.assembleCallSiteRules(function.rules)

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
    pub fn evaluate_templated_function_from_call_for_banner(
        &self,
        declaring_env: &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        original_calling_env: IInDenizenEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        already_specified_template_args: &[ITemplataT<'s, 't>],
        context_region: RegionT,
        args: &[CoordT<'s, 't>],
    ) -> Result<IEvaluateFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
        let function = declaring_env.function;
        // Check preconditions
        self.check_closure_concerns_handled(declaring_env);

        let call_site_rules = self.assemble_call_site_rules(function.rules);

        let initial_sends = self.assemble_initial_sends_from_args(call_range[0], function, &args.iter().map(|a| Some(*a)).collect::<Vec<_>>());
        let initial_knowns = self.assemble_known_templatas(function, already_specified_template_args);

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
                    self_env: declaring_env.into(),
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
                Err(e) => return Ok(IEvaluateFunctionResult::EvaluateFunctionFailure(EvaluateFunctionFailure { reason: e })),
                Ok(inferred_templatas) => inferred_templatas,
            };

        // See FunctionCompiler doc for what outer/runes/inner envs are.
        let reachable_bounds: Vec<PrototypeTemplataT<'s, 't>> =
            instantiation_bound_params.rune_to_citizen_rune_to_reachable_prototype.values()
                .flat_map(|r| {
                    panic!("implement: evaluate_templated_function_from_call_for_banner reachable bounds");
                    #[allow(unreachable_code)]
                    empty::<PrototypeTemplataT<'s, 't>>()
                })
                .collect();

        let runed_env: &'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> =
            self.typing_interner.alloc(self.add_runed_data_to_near_env(
                declaring_env,
                &function.generic_parameters.iter().map(|gp| gp.rune.rune).collect::<Vec<_>>(),
                &inferences,
                &reachable_bounds));

        let prototype_templata =
            self.get_or_evaluate_templated_function_for_banner(
                declaring_env, runed_env, coutputs, call_range_t, call_location, function, instantiation_bound_params)?;

        // Lambdas cant have bounds, right?
        assert!(instantiation_bound_params.rune_to_bound_prototype.is_empty(), "vcurious");
        assert!(instantiation_bound_params.rune_to_citizen_rune_to_reachable_prototype.is_empty(), "vcurious");
        assert!(instantiation_bound_params.rune_to_bound_impl.is_empty(), "vcurious");
        let instantiation_bound_args = self.typing_interner.alloc(InstantiationBoundArgumentsT {
            rune_to_bound_prototype: self.typing_interner.alloc_index_map_from_iter(
                instantiation_bound_params.rune_to_bound_prototype.iter()
                    .map(|(_k, _v)| panic!("implement: evaluate_templated_function_from_call_for_banner — rune_to_bound_prototype passthrough"))
            ),
            rune_to_citizen_rune_to_reachable_prototype: self.typing_interner.alloc_index_map_from_iter(
                instantiation_bound_params.rune_to_citizen_rune_to_reachable_prototype.iter()
                    .map(|(_x, _v)| panic!("implement: evaluate_templated_function_from_call_for_banner — InstantiationReachableBoundArgumentsT mapping"))
            ),
            rune_to_bound_impl: self.typing_interner.alloc_index_map_from_iter(
                instantiation_bound_params.rune_to_bound_impl.iter()
                    .map(|(_k, _v)| panic!("implement: evaluate_templated_function_from_call_for_banner — rune_to_bound_impl passthrough"))
            ),
        });
        coutputs.add_instantiation_bounds(
            self.opts.global_options.sanity_check,
            self.typing_interner,
            original_calling_env.denizen_template_id(),
            prototype_templata.prototype.id,
            instantiation_bound_args);
        Ok(IEvaluateFunctionResult::EvaluateFunctionSuccess(EvaluateFunctionSuccess {
            prototype: self.typing_interner.alloc(prototype_templata),
            inferences,
            instantiation_bound_args,
        }))
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
        TemplataCompiler.assembleCallSiteRules(function.rules)

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
    pub fn evaluate_templated_light_banner_from_call(
        &self,
        near_env: &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        original_calling_env: IInDenizenEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        explicit_template_args: &[ITemplataT<'s, 't>],
        context_region: RegionT,
        args: &[CoordT<'s, 't>],
    ) -> Result<IEvaluateFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
        let function = near_env.function;
        // Check preconditions
        match &function.body {
            IBodyS::CodeBody(body1) => assert!(body1.body.closured_names.is_empty()),
            _ => {}
        }

        // Per @ECSIIOSZ, this is the per-call-site solver for function call resolution: argument
        // types become InitialSends, explicit template args become InitialKnowns, and
        // assemble_call_site_rules filters per SROACSD.
        let call_site_rules = self.assemble_call_site_rules(function.rules);

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
                Err(e) => return Ok(IEvaluateFunctionResult::EvaluateFunctionFailure(EvaluateFunctionFailure { reason: e })),
                Ok(inferred_templatas) => inferred_templatas,
            };

        // See FunctionCompiler doc for what outer/runes/inner envs are.
        let reachable_bounds: Vec<PrototypeTemplataT<'s, 't>> =
            instantiation_bound_params.rune_to_citizen_rune_to_reachable_prototype.values()
                .flat_map(|m| m.citizen_rune_to_reachable_prototype.values().copied())
                .map(|p| PrototypeTemplataT { prototype: self.typing_interner.alloc(p) })
                .collect();

        let runed_env: &'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> =
            self.typing_interner.alloc(self.add_runed_data_to_near_env(
                near_env,
                &function.generic_parameters.iter().map(|gp| gp.rune.rune).collect::<Vec<_>>(),
                &inferences,
                &reachable_bounds));

        let prototype_templata =
            self.get_or_evaluate_templated_function_for_banner(
                near_env, runed_env, coutputs, call_range_t, call_location, function, instantiation_bound_params)?;

        // Lambdas cant have bounds, right?
        assert!(instantiation_bound_params.rune_to_bound_prototype.is_empty(), "vcurious");
        assert!(instantiation_bound_params.rune_to_citizen_rune_to_reachable_prototype.is_empty(), "vcurious");
        assert!(instantiation_bound_params.rune_to_bound_impl.is_empty(), "vcurious");
        let instantiation_bound_args = self.typing_interner.alloc(InstantiationBoundArgumentsT {
            rune_to_bound_prototype: self.typing_interner.alloc_index_map_from_iter(
                instantiation_bound_params.rune_to_bound_prototype.iter().map(|(k, v)| (*k, *v))
            ),
            rune_to_citizen_rune_to_reachable_prototype: self.typing_interner.alloc_index_map_from_iter(
                instantiation_bound_params.rune_to_citizen_rune_to_reachable_prototype.iter().map(|(k, v)| (*k, *v))
            ),
            rune_to_bound_impl: self.typing_interner.alloc_index_map_from_iter(
                instantiation_bound_params.rune_to_bound_impl.iter().map(|(k, v)| (*k, *v))
            ),
        });
        coutputs.add_instantiation_bounds(
            self.opts.global_options.sanity_check,
            self.typing_interner,
            original_calling_env.denizen_template_id(),
            prototype_templata.prototype.id,
            instantiation_bound_args);
        Ok(IEvaluateFunctionResult::EvaluateFunctionSuccess(EvaluateFunctionSuccess {
            prototype: self.typing_interner.alloc(prototype_templata),
            inferences,
            instantiation_bound_args,
        }))
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

    // Per @ECSIIOSZ, this is the per-call-site solver for function call resolution: argument
    // types become InitialSends, explicit template args become InitialKnowns, and
    // assembleCallSiteRules filters per SROACSD.
    val callSiteRules =
      TemplataCompiler.assembleCallSiteRules(function.rules)

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
    pub fn check_closure_concerns_handled(
        &self,
        near_env: &BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
    ) {
        let function = near_env.function;
        match &function.body {
            IBodyS::CodeBody(code_body) => {
                for name in code_body.body.closured_names.iter() {
                    let translated = self.translate_var_name_step(*name);
                    assert!(near_env.variables.iter().any(|v| v.name() == translated));
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
  // The reachableBoundsFromParamsAndReturn harvest violates @BDPFWDZ — the bound prototypes
  // are pushed downward from each citizen-typed param's inner env into this near-env.
*/
    // IOW, add the necessary data to turn the near env into the runed env.
    // The reachable_bounds_from_params_and_return harvest violates @BDPFWDZ — the bound prototypes
    // are pushed downward from each citizen-typed param's inner env into this near-env.
    pub fn add_runed_data_to_near_env(
        &self,
        near_env: &BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
        identifying_runes: &[IRuneS<'s>],
        templatas_by_rune: &HashMap<IRuneS<'s>, ITemplataT<'s, 't>>,
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
                    let name = self.typing_interner.intern_reachable_prototype_name(ReachablePrototypeNameT { num: i as i32});
                    (INameT::ReachablePrototype(name), IEnvEntryT::Templata(ITemplataT::Prototype(self.typing_interner.alloc(*t))))
                })
                .chain(
                    templatas_by_rune.iter()
                        .map(|(k, v)| {
                            let rune_name = self.typing_interner.intern_rune_name(RuneNameT { rune: *k});
                            (INameT::Rune(rune_name), IEnvEntryT::Templata(*v))
                        })
                )
                .collect();

        // newEntries = templatas.addEntries(interner, entries_list)
        let new_entries = self.typing_interner.alloc(near_env.templatas.add_entries(self.typing_interner, self.scout_arena, entries_list));

        let default_region = RegionT { region: IRegionT::Default };

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

    val defaultRegion = RegionT(DefaultRegionT)

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
    pub fn evaluate_generic_function_from_call_for_prototype(
        &self,
        outer_env: &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        explicit_template_args: &[ITemplataT<'s, 't>],
        context_region: RegionT,
        args: &[Option<CoordT<'s, 't>>],
        container_rune_initial_knowns: &[InitialKnown<'s, 't>],
    ) -> Result<IResolveFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
        let function = outer_env.function;
        self.check_closure_concerns_handled(outer_env);

        let call_site_rules = self.assemble_call_site_rules(function.rules);

        let initial_sends = self.assemble_initial_sends_from_args(call_range[0], function, args);

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
        let initial_knowns: Vec<InitialKnown<'s, 't>> = {
            let mut v = self.assemble_known_templatas(function, explicit_template_args);
            v.extend(container_rune_initial_knowns.iter().copied());
            v
        };
        let include_reachable_bounds_for_runes: Vec<IRuneS<'s>> =
            function.params.iter()
                .flat_map(|p| p.pattern.coord_rune.map(|ru| ru.rune))
                .chain(function.maybe_ret_coord_rune.map(|ru| ru.rune))
                .collect();

        // No MKRFA preprocessing needed: `function.rules` is declaration-scoped (same solver as
        // the function's own generic params), so the postparser never emits RuneParentEnvLookupSR
        // into it. If this site ever starts consuming expression-level rules, MKRFA preprocessing
        // MUST be added — see OverloadResolver.scala:311 and docs/refactor-thoughts/mkrfa-protocol-leak.md.
        let mut solver = self.make_solver_state(
            envs, coutputs, &call_site_rules, &rune_to_type, invocation_range, &initial_knowns, &initial_sends);

        let mut loop_check = function.generic_parameters.len() as i32 + 1;

        // Per @DRSINI, defaults are added here incrementally as a fallback, only for runes
        // that remain unsolved after argument inference.
        match self.incrementally_solve(
            envs, coutputs, &mut solver,
            |_coutputs, solver_state| {
                if loop_check == 0 {
                    panic!("RangedInternalErrorT: Infinite loop detected in incremental call solve!");
                }
                loop_check -= 1;

                match self.get_first_unsolved_identifying_rune(
                    function.generic_parameters,
                    |rune| solver_state.get_conclusion(&rune).is_some(),
                ) {
                    None => false,
                    Some((generic_param, index)) => {
                        assert!(index >= explicit_template_args.len() as i32);

                        match &generic_param.default {
                            Some(default_rules) => {
                                match solver_state.commit_step::<ITypingPassSolverError>(
                                    false, vec![], HashMap::new(),
                                    default_rules.rules.iter().map(|r| **r).collect(),
                                    default_rules.rune_to_type.iter().map(|(k, _)| *k).collect()) {
                                    Ok(()) => {}
                                    Err(_) => panic!("getOrDie"),
                                };
                                true
                            }
                            None => {
                                false
                            }
                        }
                    }
                }
            },
        ) {
            Err(f) => {
                return Ok(IResolveFunctionResult::ResolveFunctionFailure(ResolveFunctionFailure {
                    reason: IResolvingError::ResolvingSolveFailedOrIncomplete(f),
                }));
            }
            Ok(true) => {}
            Ok(false) => {} // Incomplete, will be detected as SolveIncomplete below.
        }

        let CompleteResolveSolve { conclusions: inferred_templatas, rune_to_bound: rune_to_function_bound } =
            match self.check_resolving_conclusions_and_resolve(
                envs, coutputs, invocation_range, call_location, &rune_to_type, &call_site_rules, &include_reachable_bounds_for_runes, &mut solver,
            )? {
                Err(e) => {
                    return Ok(IResolveFunctionResult::ResolveFunctionFailure(ResolveFunctionFailure {
                        reason: e,
                    }));
                }
                Ok(i) => i,
            };

        let identifying_runes: Vec<IRuneS<'s>> =
            function.generic_parameters.iter().map(|gp| gp.rune.rune).collect();
        let reachable_bound_protos: Vec<PrototypeTemplataT<'s, 't>> =
            rune_to_function_bound.rune_to_citizen_rune_to_reachable_prototype.iter()
                .flat_map(|(_rune, x)| x.citizen_rune_to_reachable_prototype.values().copied())
                .map(|proto| PrototypeTemplataT { prototype: self.typing_interner.alloc(proto) })
                .collect();
        let runed_env = self.typing_interner.alloc(self.add_runed_data_to_near_env(
            outer_env, &identifying_runes, &inferred_templatas, &reachable_bound_protos));

        let prototype = self.get_generic_function_prototype_from_call(
            runed_env, coutputs, call_range, function)?;

        let prototype_templata = self.typing_interner.alloc(PrototypeTemplataT { prototype: self.typing_interner.alloc(prototype) });

        coutputs.add_instantiation_bounds(
            self.opts.global_options.sanity_check,
            self.typing_interner,
            calling_env.root_compiling_denizen_env().denizen_template_id(),
            prototype.id,
            self.typing_interner.alloc(rune_to_function_bound),
        );

        Ok(IResolveFunctionResult::ResolveFunctionSuccess(ResolveFunctionSuccess {
            prototype: prototype_templata,
            inferences: inferred_templatas,
        }))
    }
/*
  def evaluateGenericFunctionFromCallForPrototype(
    // The environment the function was defined in.
    outerEnv: BuildingFunctionEnvironmentWithClosuredsT,
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    explicitTemplateArgs: Vector[ITemplataT[ITemplataType]],
    contextRegion: RegionT,
    args: Vector[Option[CoordT]],
    containerRuneInitialKnowns: Vector[InitialKnown] = Vector.empty):
  (IResolveFunctionResult) = {
    val function = outerEnv.function
    // Check preconditions
    checkClosureConcernsHandled(outerEnv)

    val callSiteRules =
        TemplataCompiler.assembleCallSiteRules(function.rules)

    val initialSends = assembleInitialSendsFromArgs(callRange.head, function, args)


    val envs = InferEnv(callingEnv, callRange, callLocation, outerEnv, contextRegion)
    val rules = callSiteRules
    val runeToType = function.runeToType
    val invocationRange = callRange
    val initialKnowns = assembleKnownTemplatas(function, explicitTemplateArgs) ++ containerRuneInitialKnowns
    val includeReachableBoundsForRunes =
      function.params.flatMap(_.pattern.coordRune.map(_.rune)) ++ function.maybeRetCoordRune.map(_.rune)

    // No MKRFA preprocessing needed: `function.rules` is declaration-scoped (same solver as
    // the function's own generic params), so the postparser never emits RuneParentEnvLookupSR
    // into it. If this site ever starts consuming expression-level rules, MKRFA preprocessing
    // MUST be added — see OverloadResolver.scala:311 and docs/refactor-thoughts/mkrfa-protocol-leak.md.
    val solver =
      inferCompiler.makeSolverState(envs, coutputs, rules, runeToType, invocationRange, initialKnowns, initialSends)

    var loopCheck = function.genericParameters.size + 1

    // Per @DRSINI, defaults are added here incrementally as a fallback, only for runes
    // that remain unsolved after argument inference.
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
                solverState.commitStep[ITypingPassSolverError](
                  false, Vector(), Map(), defaultRules.rules, defaultRules.runeToType.keySet).getOrDie()
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
    pub fn evaluate_generic_virtual_dispatcher_function_for_prototype_solving(
        &self,
        near_env: &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        args: &[Option<CoordT<'s, 't>>],
    ) -> Result<IDefineFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
        let function = near_env.function;
        self.check_closure_concerns_handled(near_env);

        let function_definition_rules: Vec<IRulexSR<'s>> =
            function.rules.iter().copied().filter(|r| include_rule_in_definition_solve(r)).collect();

        let initial_sends = self.assemble_initial_sends_from_args(call_range[0], function, args);

        let preliminary_envs = InferEnv {
            original_calling_env: calling_env,
            parent_ranges: self.typing_interner.alloc_slice_copy(call_range),
            call_location,
            self_env: IEnvironmentT::BuildingWithClosureds(near_env),
            context_region: RegionT { region: IRegionT::Default },
        };
        let preliminary_rune_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
            function.rune_to_type.iter().map(|(k, v)| (*k, *v)).collect();
        let mut preliminary_solver_state =
            self.make_solver_state(
                preliminary_envs,
                coutputs,
                &function_definition_rules,
                &preliminary_rune_to_type,
                &{
                    let mut ranges = vec![function.range];
                    ranges.extend_from_slice(call_range);
                    ranges
                },
                &[],
                &initial_sends);
        match self.r#continue(preliminary_envs, coutputs, &mut preliminary_solver_state) {
            Ok(()) => {}
            Err(_f) => { panic!("implement: TypingPassSolverError from preliminary continue"); }
        }

        let preliminary_inferences: HashMap<IRuneS<'s>, ITemplataT<'s, 't>> =
            preliminary_solver_state.userify_conclusions().into_iter().collect();

        let placeholder_initial_knowns_from_function: Vec<InitialKnown<'s, 't>> =
            function.generic_parameters.iter().enumerate().flat_map(|(index, generic_param)| {
                match preliminary_inferences.get(&generic_param.rune.rune) {
                    Some(&x) => Some(InitialKnown { rune: generic_param.rune, templata: x }),
                    None => { panic!("implement: create placeholder for missing preliminary inference"); }
                }
            }).collect();

        let CompleteDefineSolve { conclusions: inferences, rune_to_bound: instantiation_bound_params } =
            match self.solve_for_defining(
                InferEnv {
                    original_calling_env: calling_env,
                    parent_ranges: self.typing_interner.alloc_slice_copy(call_range),
                    call_location,
                    self_env: IEnvironmentT::BuildingWithClosureds(near_env),
                    context_region: RegionT { region: IRegionT::Default },
                },
                coutputs,
                &function_definition_rules,
                &preliminary_rune_to_type,
                &{
                    let mut ranges = vec![function.range];
                    ranges.extend_from_slice(call_range);
                    ranges
                },
                call_location,
                &placeholder_initial_knowns_from_function,
                &[],
                &{
                    let mut runes: Vec<IRuneS<'s>> = function.params.iter()
                        .flat_map(|p| p.pattern.coord_rune.map(|r| r.rune))
                        .collect();
                    if let Some(r) = function.maybe_ret_coord_rune {
                        runes.push(r.rune);
                    }
                    runes
                },
            ) {
                Err(_f) => { panic!("implement: TypingPassDefiningError from solve_for_defining"); }
                Ok(c) => c,
            };
        let reachable_bounds: Vec<PrototypeTemplataT<'s, 't>> =
            instantiation_bound_params.rune_to_citizen_rune_to_reachable_prototype.values()
                .flat_map(|m| m.citizen_rune_to_reachable_prototype.values().copied())
                .map(|p| PrototypeTemplataT { prototype: self.typing_interner.alloc(p) })
                .collect();
        let runed_env =
            self.add_runed_data_to_near_env(
                near_env,
                &function.generic_parameters.iter().map(|p| p.rune.rune).collect::<Vec<_>>(),
                &inferences,
                &reachable_bounds);

        let runed_env_ref = self.typing_interner.alloc(runed_env);
        let prototype =
            self.get_generic_function_prototype_from_call(
                runed_env_ref, coutputs, call_range, function)?;

        Ok(IDefineFunctionResult::DefineFunctionSuccess(DefineFunctionSuccess {
            prototype: self.typing_interner.alloc(PrototypeTemplataT { prototype: self.typing_interner.alloc(prototype) }),
            inferences,
            instantiation_bound_params: instantiation_bound_params,
        }))
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
    val preliminaryEnvs = InferEnv(callingEnv, callRange, callLocation, nearEnv, RegionT(DefaultRegionT))
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
            val placeholderPureHeight = vregionmut(None)
            val templata =
              templataCompiler.createPlaceholder(
                coutputs, callingEnv, callingEnv.id, genericParam, index, function.runeToType, placeholderPureHeight, true)
            Some(InitialKnown(genericParam.rune, templata))
          }
        }
      })

    // Now that we have placeholders, let's do the rest of the solve, so we can get a full
    // prototype out of it.

    val CompleteDefineSolve(inferences, instantiationBoundParams) =
      inferCompiler.solveForDefining(
        InferEnv(callingEnv, callRange, callLocation, nearEnv, RegionT(DefaultRegionT)),
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
    pub fn evaluate_generic_function_from_non_call_solving(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        near_env: &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
    ) -> Result<&'t FunctionHeaderT<'s, 't>, ICompileErrorT<'s, 't>> {
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
        let function_template_id = near_env.parent_env.id().add_step(self.typing_interner, function_name_local);

        let definition_rules: Vec<IRulexSR<'s>> = function.rules.iter().copied()
            .filter(|r| include_rule_in_definition_solve(r))
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
        let near_env_as_in_denizen = IInDenizenEnvironmentT::BuildingWithClosureds(near_env);
        let near_env_as_env = IEnvironmentT::BuildingWithClosureds(near_env);
        let envs = InferEnv {
            original_calling_env: near_env_as_in_denizen,
            parent_ranges: parent_ranges_alloc,
            call_location,
            self_env: near_env_as_env,
            context_region: RegionT { region: IRegionT::Default },
        };

        let rune_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> = function.rune_to_type.iter()
            .map(|(k, v)| (*k, *v))
            .collect();
        let mut solver = self.make_solver_state(
            envs, coutputs, &definition_rules, &rune_to_type, &range, &[], &[]);

        let get_first_unsolved = |generic_parameters: &'s [&'s GenericParameterS<'s>], is_solved: &dyn Fn(IRuneS<'s>) -> bool| {
            self.get_first_unsolved_identifying_rune(generic_parameters, |rune| is_solved(rune))
        };
        let result = self.incrementally_solve(
            envs, coutputs, &mut solver,
            |coutputs, solver_state| {
                match get_first_unsolved(
                    function.generic_parameters,
                    &|rune| solver_state.get_conclusion(&rune).is_some(),
                ) {
                    None => false,
                    Some((generic_param, index)) => {
                        let placeholder_pure_height = None;
                        let templata = self.create_placeholder(
                            coutputs, near_env_as_in_denizen, *function_template_id,
                            generic_param, index, &rune_to_type, placeholder_pure_height, true);
                        solver_state.commit_step::<()>(
                            false, vec![], {
                                let mut m = HashMap::new();
                                m.insert(generic_param.rune.rune, templata);
                                m
                            }, vec![], HashSet::new()).unwrap();
                        true
                    }
                }
            });
        match result {
            Err(f) => return Err(ICompileErrorT::TypingPassSolverError {
                range: self.typing_interner.alloc_slice_from_vec(range.clone()),
                failed_solve: f,
            }),
            Ok(true) => {}
            Ok(false) => {} // Incomplete, will be detected in checkDefiningConclusionsAndResolve
        }

        let inferences = match self.interpret_results(&rune_to_type, &mut solver) {
            Err(e) => return Err(ICompileErrorT::TypingPassSolverError {
                range: self.typing_interner.alloc_slice_from_vec(range.clone()),
                failed_solve: e,
            }),
            Ok(conclusions) => conclusions,
        };

        let instantiation_bound_params = match self.check_defining_conclusions_and_resolve(
            envs, coutputs, &range, call_location, &definition_rules, &param_and_return_runes, &inferences,
        ) {
            Err(f) => {
                match f {
                    IConclusionResolveError::CouldntFindFunctionForConclusionResolve { .. } => panic!("TypingPassDefiningError: CouldntFindFunctionForConclusionResolve"),
                    IConclusionResolveError::ReturnTypeConflictInConclusionResolve { .. } => panic!("TypingPassDefiningError: ReturnTypeConflictInConclusionResolve"),
                    IConclusionResolveError::CouldntFindImplForConclusionResolve { .. } => panic!("TypingPassDefiningError: CouldntFindImplForConclusionResolve"),
                    IConclusionResolveError::CouldntFindKindForConclusionResolve(_) => panic!("TypingPassDefiningError: CouldntFindKindForConclusionResolve"),
                }
            }
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
            near_env, runed_env, coutputs, parent_ranges, call_location, function, instantiation_bound_params)?;

        Ok(header)
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

    val envs = InferEnv(nearEnv, parentRanges, callLocation, nearEnv, RegionT(DefaultRegionT))
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
              solverState.commitStep[Nothing](false, Vector(), Map(genericParam.rune.rune -> templata), Vector(), Set.empty).getOrDie()
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
