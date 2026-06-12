use std::collections::HashMap;
use indexmap::IndexMap;
use crate::typing::compiler::Compiler;
use crate::utils::range::RangeS;
use crate::postparsing::ast::{LocationInDenizen, IRegionMutabilityS};
use crate::postparsing::names::*;
use crate::postparsing::rules::rules::*;
use crate::postparsing::itemplatatype::ITemplataType;
use crate::postparsing::rune_type_solver::{RuneTypeSolveError, solve_rune_type, IRuneTypeSolverEnv, IRuneTypeSolverLookupResult, IRuneTypingLookupFailedError};
use crate::postparsing::*;
use crate::solver::solver::FailedSolve;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::ReferenceExpressionTE;
use crate::typing::compiler_outputs::*;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::FunctionEnvironmentT;
use crate::typing::function::function_compiler::{StampFunctionSuccess, IResolveFunctionResult, IEvaluateFunctionResult};
use crate::typing::infer_compiler::{InferEnv, InitialKnown};
use crate::typing::templata_compiler::IBoundArgumentsSource;
use crate::typing::names::names::*;
use crate::typing::templata::templata::*;
use crate::typing::types::types::*;
use crate::typing::env::environment::ILookupContext;
use crate::postparsing::rune_type_solver::TemplataLookupResult;
use crate::postparsing::rune_type_solver::RuneTypingCouldntFindType;
use crate::postparsing::rules::rules::RuneParentEnvLookupSR;
use crate::postparsing::names::{IImpreciseNameValS, RuneNameValS};
use crate::postparsing::names::CodeNameS;
use crate::typing::env::environment::{IInDenizenEnvironmentT};
use crate::typing::infer::compiler_solver::ITypingPassSolverError;
use crate::typing::infer_compiler::IResolvingError;
use crate::typing::infer_compiler::IDefiningError;
use crate::typing::typing_interner::TypingInterner;
use crate::scout_arena::ScoutArena;
use crate::typing::types::types::CoordT;
use crate::typing::types::types::OwnershipT;
use crate::typing::types::types::KindT;
use crate::typing::types::types::IntT;
use crate::higher_typing::higher_typing_pass::explicify_lookups;
use std::collections::HashSet;

/*
package dev.vale.typing

import dev.vale._
import dev.vale.postparsing._
import dev.vale.postparsing.rules._
import dev.vale.solver.FailedSolve
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing.function._
import dev.vale.typing.infer.ITypingPassSolverError
import dev.vale.typing.types._
import dev.vale.highertyping._
import dev.vale.typing.function._
import dev.vale.postparsing.PostParserErrorHumanizer
import dev.vale.solver.FailedSolve
import OverloadResolver._
import dev.vale.highertyping.HigherTypingPass.explicifyLookups
import dev.vale.typing.ast._
import dev.vale.typing.env._
import dev.vale.typing.templata._
import dev.vale.typing.ast._
import dev.vale.typing.names._

import scala.collection.immutable.{Map, Set}
import scala.collection.mutable
import scala.collection.mutable.ArrayBuffer
//import dev.vale.astronomer.ruletyper.{IRuleTyperEvaluatorDelegate, RuleTyperEvaluator, RuleTyperSolveFailure, RuleTyperSolveSuccess}
//import dev.vale.postparsing.rules.{EqualsSR, TemplexSR, TypedSR}
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale.postparsing.ExplicitTemplateArgRuneS
import OverloadResolver.{IFindFunctionFailureReason, InferFailure, FindFunctionFailure, SpecificParamVirtualityDoesntMatch, WrongNumberOfArguments, WrongNumberOfTemplateArguments}
import dev.vale.typing.env._
//import dev.vale.typingpass.infer.infer.{InferSolveFailure, InferSolveSuccess}
import dev.vale.Profiler

import scala.collection.immutable.List

object OverloadResolver {

*/
#[derive(Debug)]
pub enum IFindFunctionFailureReason<'s, 't> {
    WrongNumberOfArguments { supplied: i32, expected: i32 },
    WrongNumberOfTemplateArguments { supplied: i32, expected: i32 },
    SpecificParamDoesntSend { index: i32, argument: CoordT<'s, 't>, parameter: CoordT<'s, 't> },
    SpecificParamDoesntMatchExactly { index: i32, argument: CoordT<'s, 't>, parameter: CoordT<'s, 't> },
    SpecificParamRegionDoesntMatch {
        rune: IRuneS<'s>,
        supplied_mutability: IRegionMutabilityS,
        callee_mutability: IRegionMutabilityS,
    },
    SpecificParamVirtualityDoesntMatch { index: i32 },
    Outscored,
    RuleTypeSolveFailure { reason: RuneTypeSolveError<'s> },
    InferFailure { reason: FailedSolve<IRulexSR<'s>, IRuneS<'s>, ITemplataT<'s, 't>, ITypingPassSolverError<'s, 't>> },
    FindFunctionResolveFailure { reason: IResolvingError<'s, 't> },
    CouldntEvaluateTemplateError { reason: IDefiningError<'s, 't> },
}
/*
  sealed trait IFindFunctionFailureReason
*/
/*
  case class WrongNumberOfArguments(supplied: Int, expected: Int) extends IFindFunctionFailureReason {
    vpass()

    override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  }
*/
/*
  case class WrongNumberOfTemplateArguments(supplied: Int, expected: Int) extends IFindFunctionFailureReason { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
/*
  case class SpecificParamDoesntSend(index: Int, argument: CoordT, parameter: CoordT) extends IFindFunctionFailureReason { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
/*
  case class SpecificParamDoesntMatchExactly(index: Int, argument: CoordT, parameter: CoordT) extends IFindFunctionFailureReason {
    override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
    vpass()
  }
*/
/*
  case class SpecificParamRegionDoesntMatch(rune: IRuneS, suppliedMutability: IRegionMutabilityS, calleeMutability: IRegionMutabilityS) extends IFindFunctionFailureReason {
    override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
    vpass()
  }
*/
/*
  case class SpecificParamVirtualityDoesntMatch(index: Int) extends IFindFunctionFailureReason { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
/*
  case class Outscored() extends IFindFunctionFailureReason { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
/*
  case class RuleTypeSolveFailure(reason: RuneTypeSolveError) extends IFindFunctionFailureReason { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
/*
  case class InferFailure(reason: FailedSolve[IRulexSR, IRuneS, ITemplataT[ITemplataType], ITypingPassSolverError]) extends IFindFunctionFailureReason { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
/*
  case class FindFunctionResolveFailure(reason: IResolvingError) extends IFindFunctionFailureReason { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }
*/
/*
  case class CouldntEvaluateTemplateError(reason: IDefiningError) extends IFindFunctionFailureReason { override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious() }


*/
#[derive(Copy, Clone, Debug)]
pub struct FindFunctionFailure<'s, 't> {
    pub name: IImpreciseNameS<'s>,
    pub args: &'t [CoordT<'s, 't>],
    pub rejected_callee_to_reason: &'t [(ICalleeCandidate<'s, 't>, IFindFunctionFailureReason<'s, 't>)],
}
/*
  case class FindFunctionFailure(
    name: IImpreciseNameS,
    args: Vector[CoordT],
    // All the banners we rejected, and the reason why
    rejectedCalleeToReason: Iterable[(ICalleeCandidate, IFindFunctionFailureReason)]
  ) {
    vpass()
    override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  }

*/
pub struct EvaluateFunctionFailure2;
/*
  case class EvaluateFunctionFailure2(
    name: IImpreciseNameS,
    args: Vector[CoordT],
    // All the banners we rejected, and the reason why
    rejectedCalleeToReason: Iterable[(PrototypeT[IFunctionNameT], IFindFunctionFailureReason)]
  ) {
    vpass()
    override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
  }
}

*/
/*
class OverloadResolver(
    opts: TypingPassOptions,
    interner: Interner,
    keywords: Keywords,
    templataCompiler: TemplataCompiler,
    inferCompiler: InferCompiler,
    functionCompiler: FunctionCompiler) {
  val runeTypeSolver = new RuneTypeSolver(interner)

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn find_function(
        &self,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        function_name: IImpreciseNameS<'s>,
        explicit_template_arg_rules_s: &[IRulexSR<'s>],
        positional_explicit_template_arg_runes_s: &[IRuneS<'s>],
        receiving_rune_to_explicit_template_arg_rune: &[(RuneUsage<'s>, RuneUsage<'s>)],
        context_region: RegionT,
        args: &[CoordT<'s, 't>],
        extra_envs_to_look_in: &[IInDenizenEnvironmentT<'s, 't>],
        exact: bool,
    ) -> Result<Result<StampFunctionSuccess<'s, 't>, FindFunctionFailure<'s, 't>>, ICompileErrorT<'s, 't>> {
        let potential_banner = self.find_potential_function(
            calling_env,
            coutputs,
            call_range,
            call_location,
            function_name,
            explicit_template_arg_rules_s,
            positional_explicit_template_arg_runes_s,
            receiving_rune_to_explicit_template_arg_rune,
            context_region,
            args,
            extra_envs_to_look_in,
            exact)?;
        match potential_banner {
            Err(e) => Ok(Err(e)),
            Ok(potential_banner) => {
                Ok(Ok(StampFunctionSuccess {
                    prototype: potential_banner.prototype,
                    inferences: HashMap::new(),
                }))
            }
        }
    }
/*
  def findFunction(
    callingEnv: IInDenizenEnvironmentT,
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
  Result[StampFunctionSuccess, FindFunctionFailure] = {
    Profiler.frame(() => {
      findPotentialFunction(
        callingEnv,
        coutputs,
        callRange,
        callLocation,
        functionName,
        explicitTemplateArgRulesS,
        positionalExplicitTemplateArgRunesS,
        receivingRuneToExplicitTemplateArgRune,
        contextRegion,
        args,
        extraEnvsToLookIn,
        exact) match {
        case Err(e) => return Err(e)
        case Ok(potentialBanner) => {
          Ok(StampFunctionSuccess(potentialBanner.prototype, Map()))
//          Ok(
//            stampPotentialFunctionForPrototype(
//              coutputs, callingEnv, callRange, callLocation, potentialBanner, contextRegion, args))
        }
      }
    })
  }

*/
    pub fn params_match(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        desired_params: &[CoordT<'s, 't>],
        candidate_params: &[CoordT<'s, 't>],
        exact: bool,
    ) -> Result<(), IFindFunctionFailureReason<'s, 't>> {
        if desired_params.len() != candidate_params.len() {
            return Err(IFindFunctionFailureReason::WrongNumberOfArguments {
                supplied: desired_params.len() as i32, expected: candidate_params.len() as i32 });
        }
        for (param_index, (desired_param, candidate_param)) in desired_params.iter().zip(candidate_params.iter()).enumerate() {
            if exact {
                if desired_param != candidate_param {
                    return Err(IFindFunctionFailureReason::SpecificParamDoesntMatchExactly {
                        index: param_index as i32, argument: *desired_param, parameter: *candidate_param });
                }
            } else {
                if !self.is_type_convertible(coutputs, calling_env, parent_ranges, call_location, *desired_param, *candidate_param) {
                    return Err(IFindFunctionFailureReason::SpecificParamDoesntSend {
                        index: param_index as i32, argument: *desired_param, parameter: *candidate_param });
                }
            }
        }
        Ok(())
    }
/*
  private def paramsMatch(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    desiredParams: Vector[CoordT],
    candidateParams: Vector[CoordT],
    exact: Boolean):
  Result[Unit, IFindFunctionFailureReason] = {
    if (desiredParams.size != candidateParams.size) {
      return Err(WrongNumberOfArguments(desiredParams.size, candidateParams.size))
    }
    desiredParams.zip(candidateParams).zipWithIndex.foreach({
      case ((desiredParam, candidateParam), paramIndex) => {
        val desiredTemplata = desiredParam
        val candidateType = candidateParam

        if (exact) {
          if (desiredTemplata != candidateType) {
            return Err(SpecificParamDoesntMatchExactly(paramIndex, desiredTemplata, candidateType))
          }
        } else {
          if (!templataCompiler.isTypeConvertible(coutputs, callingEnv, parentRanges, callLocation, desiredTemplata, candidateType)) {
            return Err(SpecificParamDoesntSend(paramIndex, desiredTemplata, candidateType))
          }
        }
      }
    })
    // Would have bailed out early if there was a false
    Ok(())
  }

*/
}

pub struct SearchedEnvironment<'s, 't> {
    pub needle: IImpreciseNameS<'s>,
    pub environment: IInDenizenEnvironmentT<'s, 't>,
    pub matching_templatas: Vec<ITemplataT<'s, 't>>,
}
/*
  case class SearchedEnvironment(
    needle: IImpreciseNameS,
    environment: IInDenizenEnvironmentT,
    matchingTemplatas: Vector[ITemplataT[ITemplataType]])

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_candidate_banners(
        &self,
        env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        function_name: IImpreciseNameS<'s>,
        param_filters: &[CoordT<'s, 't>],
        extra_envs_to_look_in: &[IInDenizenEnvironmentT<'s, 't>],
        searched_envs: &mut Vec<SearchedEnvironment<'s, 't>>,
        results: &mut Vec<ICalleeCandidate<'s, 't>>,
    ) {
        self.get_candidate_banners_inner(env, coutputs, range, function_name, searched_envs, results);
        for e in self.get_param_environments(coutputs, range, param_filters) {
            self.get_candidate_banners_inner(e, coutputs, range, function_name, searched_envs, results);
        }
        for e in self.get_placeholder_extra_call_envs(env, coutputs, range, param_filters) {
            self.get_candidate_banners_inner(e, coutputs, range, function_name, searched_envs, results);
        }
        // Empirically dead on the current Vale corpus (verified 2026-06-08 by reverting
        // to a no-op shape and running the full suite — 1064/1064 still passed). Only
        // EdgeCompiler's override resolution passes non-empty here, and both envs it
        // passes are redundantly reached via the param-environments / calling-env paths.
        // Kept for 1:1 Scala parity with `extraEnvsToLookIn.foreach(e => …)`.
        for e in extra_envs_to_look_in {
            self.get_candidate_banners_inner(*e, coutputs, range, function_name, searched_envs, results);
        }
    }
/*
  private def getCandidateBanners(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    range: List[RangeS],
    functionName: IImpreciseNameS,
    paramFilters: Vector[CoordT],
    extraEnvsToLookIn: Vector[IInDenizenEnvironmentT],
    searchedEnvs: Accumulator[SearchedEnvironment],
    results: Accumulator[ICalleeCandidate]):
  Unit = {
    getCandidateBannersInner(env, coutputs, range, functionName, searchedEnvs, results)
    getParamEnvironments(coutputs, range, paramFilters)
      .foreach(e => getCandidateBannersInner(e, coutputs, range, functionName, searchedEnvs, results))
    // When calling a method on a placeholder (well, any function involving a
    // placeholder argument really), also look in the environments for any interfaces that we know
    // that placeholder impls (see @BDPFWDZ). See also `AfterRegionsIntegrationTests."Method call on impl-bounded
    // generic dispatches through interface"` in IntegrationTests.
    getPlaceholderExtraCallEnvs(env, coutputs, range, paramFilters)
      .foreach(e => getCandidateBannersInner(e, coutputs, range, functionName, searchedEnvs, results))
    extraEnvsToLookIn
      .foreach(e => getCandidateBannersInner(e, coutputs, range, functionName, searchedEnvs, results))
  }

*/
    pub fn get_candidate_banners_inner(
        &self,
        env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        function_name: IImpreciseNameS<'s>,
        searched_envs: &mut Vec<SearchedEnvironment<'s, 't>>,
        results: &mut Vec<ICalleeCandidate<'s, 't>>,
    ) {
        let mut seen = HashSet::new();
        let candidates: Vec<ITemplataT<'s, 't>> =
            env.lookup_all_with_imprecise_name(
                function_name,
                HashSet::from([ILookupContext::ExpressionLookupContext]),
                self.typing_interner)
                .into_iter().filter(|c| seen.insert(*c)).collect();
        searched_envs.push(SearchedEnvironment {
            needle: function_name,
            environment: env,
            matching_templatas: candidates.clone(),
        });
        for candidate in candidates.iter() {
            match candidate {
                ITemplataT::Kind(KindTemplataT { kind: KindT::OverloadSet(_) }) => {
                    panic!("implement: get_candidate_banners_inner OverloadSet");
                }
                ITemplataT::Kind(KindTemplataT { kind: KindT::Struct(_) }) => {
                    panic!("implement: get_candidate_banners_inner Struct");
                }
                ITemplataT::Kind(KindTemplataT { kind: KindT::Interface(_) }) => {
                    panic!("implement: get_candidate_banners_inner Interface");
                }
                ITemplataT::ExternFunction(_) => {
                    panic!("implement: get_candidate_banners_inner ExternFunction");
                }
                ITemplataT::Prototype(proto_templata) => {
                    assert!(coutputs.get_instantiation_bounds(self.typing_interner, proto_templata.prototype.id).is_some());
                    results.push(ICalleeCandidate::PrototypeTemplata(PrototypeTemplataCalleeCandidate { prototype_t: *proto_templata.prototype }));
                }
                ITemplataT::Function(ft) => {
                    results.push(ICalleeCandidate::Function(FunctionCalleeCandidate { ft: **ft }));
                }
                _ => {
                    panic!("implement: get_candidate_banners_inner other templata");
                }
            }
        }
    }
/*
  private def getCandidateBannersInner(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    range: List[RangeS],
    functionName: IImpreciseNameS,
    searchedEnvs: Accumulator[SearchedEnvironment],
    results: Accumulator[ICalleeCandidate]):
  Unit = {
    val candidates =
      env.lookupAllWithImpreciseName(functionName, Set(ExpressionLookupContext)).toVector.distinct
    searchedEnvs.add(SearchedEnvironment(functionName, env, candidates))
    candidates.foreach({
      case KindTemplataT(OverloadSetT(overloadsEnv, nameInOverloadsEnv)) => {
        getCandidateBannersInner(
          overloadsEnv, coutputs, range, nameInOverloadsEnv, searchedEnvs, results)
      }
      case KindTemplataT(sr@StructTT(_)) => {
        val structEnv = coutputs.getOuterEnvForType(range, TemplataCompiler.getStructTemplate(sr.id))
        getCandidateBannersInner(
          structEnv, coutputs, range, interner.intern(CodeNameS(keywords.underscoresCall)), searchedEnvs, results)
      }
      case KindTemplataT(sr@InterfaceTT(_)) => {
        val interfaceEnv = coutputs.getOuterEnvForType(range, TemplataCompiler.getInterfaceTemplate(sr.id))
        getCandidateBannersInner(
          interfaceEnv, coutputs, range, interner.intern(CodeNameS(keywords.underscoresCall)), searchedEnvs, results)
      }
      case ExternFunctionTemplataT(header) => {
        results.add(HeaderCalleeCandidate(header))
      }
      case PrototypeTemplataT(prototype) => {
        vassert(coutputs.getInstantiationBounds(prototype.id).nonEmpty)
        results.add(PrototypeTemplataCalleeCandidate(prototype))
      }
      case ft@FunctionTemplataT(_, _) => {
        results.add(FunctionCalleeCandidate(ft))
      }
    })
  }

*/
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct AttemptedCandidate<'s, 't> {
    pub prototype: &'t PrototypeT<'s, 't>,
}
/*
  case class AttemptedCandidate(
      // Pure and region will go here
      prototype: PrototypeT[IFunctionNameT]
  )

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn attempt_candidate_banner(
        &self,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        explicit_template_arg_rules_without_connections: &[IRulexSR<'s>],
        positional_explicit_template_arg_runes_s: &[IRuneS<'s>],
        receiving_rune_to_explicit_template_arg_rune: &[(RuneUsage<'s>, RuneUsage<'s>)],
        context_region: RegionT,
        args: &[CoordT<'s, 't>],
        candidate: ICalleeCandidate<'s, 't>,
        exact: bool,
    ) -> Result<Result<AttemptedCandidate<'s, 't>, IFindFunctionFailureReason<'s, 't>>, ICompileErrorT<'s, 't>> {
        // Scala: anonymous `new IRuneTypeSolverEnv { override def lookup(...) }` inside attemptCandidateBanner
        struct OverloadRuneTypeSolverEnv<'a, 's, 't> where 's: 't {
            calling_env: IInDenizenEnvironmentT<'s, 't>,
            typing_interner: &'a TypingInterner<'s, 't>,
            scout_arena: &'a ScoutArena<'s>,
        }
        impl<'a, 's, 't> IRuneTypeSolverEnv<'s> for OverloadRuneTypeSolverEnv<'a, 's, 't> where 's: 't {
            fn lookup(
                &self,
                range: RangeS<'s>,
                name_s: IImpreciseNameS<'s>,
            ) -> Result<IRuneTypeSolverLookupResult<'s>, IRuneTypingLookupFailedError<'s>> {
                let mut filter = HashSet::new();
                filter.insert(ILookupContext::TemplataLookupContext);
                match self.calling_env.lookup_nearest_with_imprecise_name(name_s, filter, self.typing_interner) {
                    Some(x) => Ok(IRuneTypeSolverLookupResult::Templata(TemplataLookupResult { templata: x.tyype(self.scout_arena) })),
                    None => Err(IRuneTypingLookupFailedError::CouldntFindType(RuneTypingCouldntFindType { range, name: name_s })),
                }
            }
        }
        /* Guardian: disable-all */
        match candidate {
            ICalleeCandidate::Function(FunctionCalleeCandidate { ft }) => {
                // See OFCBT.
                let identifying_rune_templata_types = ft.function.tyype.param_types;
                // Now we want to check that the user didn't specify too many right here.
                // The function can inherit runes from its container, so subtract those first.
                let own_rune_count = identifying_rune_templata_types.len() - receiving_rune_to_explicit_template_arg_rune.len();
                if positional_explicit_template_arg_runes_s.len() > own_rune_count {
                    panic!("implement: attemptCandidateBanner WrongNumberOfTemplateArguments");
                } else {
                    let explicit_template_arg_rules_with_connections: Vec<IRulexSR<'s>> = {
                        let mut v = explicit_template_arg_rules_without_connections.to_vec();
                        for (receiving_rune, callsite_rune) in receiving_rune_to_explicit_template_arg_rune.iter() {
                            if !ft.function.generic_parameters.iter().any(|gp| gp.rune.rune == receiving_rune.rune) {
                                panic!("Supplied rune {:?} that doesn't exist in called function {:?}", receiving_rune, ft.function.name);
                            }
                            v.push(IRulexSR::Equals(EqualsSR { range: callsite_rune.range, left: *receiving_rune, right: *callsite_rune }));
                        }
                        v
                    };
                    // Now that we know what types are expected, we can FINALLY rule-type these explicitly
                    // specified template args! (The rest of the rule-typing happened back in the astronomer,
                    // this is the one time we delay it, see MDRTCUT).
                    // Args supplied through receivingRuneToExplicitTemplateArgRune (the named channel for
                    // container template args) also need their callsite rune seeded with the expected type,
                    // otherwise MaybeCoercingLookupSR for those args can't fire in the rune-type solver.
                    let receiving_rune_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
                        ft.function.generic_parameters.iter().map(|gp| (gp.rune.rune, gp.tyype.tyype())).collect();
                    let callsite_rune_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
                        receiving_rune_to_explicit_template_arg_rune.iter()
                            .filter_map(|(receiving_rune, callsite_rune)| {
                                receiving_rune_to_type.get(&receiving_rune.rune).map(|t| (callsite_rune.rune, *t))
                            })
                            .collect();
                    // There might be less explicitly specified template args than there are types, and that's
                    // fine. Hopefully the rest will be figured out by the rule evaluator.
                    let explicit_template_arg_rune_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> = {
                        let mut m = callsite_rune_to_type;
                        for (r, t) in positional_explicit_template_arg_runes_s.iter().copied()
                            .zip(identifying_rune_templata_types.iter().copied())
                        {
                            m.insert(r, t);
                        }
                        m
                    };

                    let rune_type_solve_env =
                        OverloadRuneTypeSolverEnv { calling_env, typing_interner: self.typing_interner, scout_arena: self.scout_arena };

                    // Scala: runeTypeSolver.solve(sanityCheck, useOptimizedSolver, env, ...)
                    // Note: Rust solve_rune_type doesn't accept useOptimizedSolver (pre-existing API difference)
                    let rules_s_deref: Vec<IRulexSR<'s>> =
                        explicit_template_arg_rules_with_connections.clone();
                    let combined_explicit_runes: Vec<IRuneS<'s>> = {
                        let mut v = positional_explicit_template_arg_runes_s.to_vec();
                        v.extend(receiving_rune_to_explicit_template_arg_rune.iter().map(|(_, callsite_rune)| callsite_rune.rune));
                        v
                    };
                    match solve_rune_type(
                        self.scout_arena,
                        self.opts.global_options.sanity_check,
                        &rune_type_solve_env,
                        call_range.to_vec(),
                        false,
                        &rules_s_deref,
                        &combined_explicit_runes,
                        true,
                        explicit_template_arg_rune_to_type.clone(),
                    ) {
                        Err(_e) => {
                            panic!("implement: attemptCandidateBanner RuleTypeSolveFailure");
                        }
                        Ok(rune_a_to_type_with_implicitly_coercing_lookups_s) => {
                            let rune_type_solve_env = self.create_rune_type_solver_env(calling_env);
                            // VIOLATES @IIIOZ: still HashMap because explicify_lookups takes &mut HashMap.
                            // Determinism here is blocked on cascading explicify_lookups + calculate_rune_types
                            // (in higher_typing_pass.rs) to IndexMap. Deferred to a separate sweep.
                            let mut rune_a_to_type: HashMap<IRuneS<'s>, ITemplataType<'s>> =
                                HashMap::from_iter(rune_a_to_type_with_implicitly_coercing_lookups_s.iter().map(|(k, v)| (*k, *v)));
                            let mut rule_builder: Vec<IRulexSR<'s>> = Vec::new();
                            match explicify_lookups(
                                &rune_type_solve_env,
                                self.scout_arena,
                                &mut rune_a_to_type,
                                &mut rule_builder,
                                rules_s_deref.clone(),
                            ) {
                                Err(_e) => {
                                    panic!("implement: attemptCandidateBanner explicifyLookups error path");
                                }
                                Ok(()) => {}
                            }
                            let rules_without_implicit_coercions_a = rule_builder;

                            // We preprocess out the rune parent env lookups, see MKRFA.
                            let (initial_knowns, rules_without_rune_parent_env_lookups): (Vec<InitialKnown>, Vec<IRulexSR<'s>>) =
                                rules_without_implicit_coercions_a.iter().fold(
                                    (Vec::new(), Vec::new()),
                                    |(mut previous_conclusions, mut remaining_rules), rule| {
                                        match rule {
                                            IRulexSR::RuneParentEnvLookup(RuneParentEnvLookupSR { rune, .. }) => {
                                                let name = self.scout_arena.intern_imprecise_name(
                                                    IImpreciseNameValS::RuneName(RuneNameValS { rune: rune.rune }));
                                                let mut filter = HashSet::new();
                                                filter.insert(ILookupContext::TemplataLookupContext);
                                                let templata = calling_env.lookup_nearest_with_imprecise_name(
                                                    name, filter, self.typing_interner).unwrap();
                                                previous_conclusions.push(InitialKnown { rune: *rune, templata });
                                                (previous_conclusions, remaining_rules)
                                            }
                                            rule => {
                                                remaining_rules.push(*rule);
                                                (previous_conclusions, remaining_rules)
                                            }
                                        }
                                    },
                                );

                            let mut combined_rune_to_type = explicit_template_arg_rune_to_type;
                            combined_rune_to_type.extend(rune_a_to_type.iter());

                            // We only want to solve the template arg runes
                            let call_range_t = self.typing_interner.alloc_slice_copy(call_range);
                            match self.solve_for_resolving(
                                InferEnv {
                                    original_calling_env: calling_env,
                                    parent_ranges: call_range_t,
                                    call_location,
                                    self_env: ft.outer_env.into(),
                                    context_region,
                                },
                                coutputs,
                                &rules_without_rune_parent_env_lookups,
                                &combined_rune_to_type,
                                call_range,
                                call_location,
                                &[],
                                &initial_knowns,
                                &[],
                            )? {
                                Err(_e) => {
                                    panic!("implement: attemptCandidateBanner FindFunctionResolveFailure");
                                }
                                Ok(complete_resolve_solve) => {
                                    let positional_explicitly_specified_template_arg_templatas: Vec<ITemplataT<'s, 't>> =
                                        positional_explicit_template_arg_runes_s.iter()
                                            .map(|r| *complete_resolve_solve.conclusions.get(r).unwrap())
                                            .collect();
                                    let receiving_rune_to_explicit_template_arg_templata: Vec<InitialKnown<'s, 't>> =
                                        receiving_rune_to_explicit_template_arg_rune.iter()
                                            .map(|(receiving_rune, explicit_template_arg_rune)| {
                                                InitialKnown {
                                                    rune: *receiving_rune,
                                                    templata: *complete_resolve_solve.conclusions.get(&explicit_template_arg_rune.rune).unwrap(),
                                                }
                                            })
                                            .collect();

                                    if ft.function.is_lambda() {
                                        assert!(receiving_rune_to_explicit_template_arg_templata.is_empty(), "implement: lambda receiving rune templatas");
                                        // We pass in our env because the callee needs to see functions declared here, see CSSNCE.
                                        match self.evaluate_templated_function_from_call_for_prototype(
                                            coutputs, calling_env, call_range, call_location, ft,
                                            &positional_explicitly_specified_template_arg_templatas, context_region, args,
                                        )? {
                                            IEvaluateFunctionResult::EvaluateFunctionFailure(failure) => {
                                                Ok(Err(IFindFunctionFailureReason::CouldntEvaluateTemplateError { reason: failure.reason }))
                                            }
                                            IEvaluateFunctionResult::EvaluateFunctionSuccess(eval_success) => {
                                                match self.params_match(
                                                    coutputs, calling_env, call_range, call_location,
                                                    args, &eval_success.prototype.prototype.param_types(), exact,
                                                ) {
                                                    Err(rejection_reason) => Ok(Err(rejection_reason)),
                                                    Ok(()) => {
                                                        assert!(coutputs.get_instantiation_bounds(self.typing_interner, eval_success.prototype.prototype.id).is_some());
                                                        Ok(Ok(AttemptedCandidate { prototype: eval_success.prototype.prototype }))
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        // We pass in our env because the callee needs to see functions declared here, see CSSNCE.
                                        match self.evaluate_generic_light_function_from_call_for_prototype(
                                            coutputs, call_range, call_location, calling_env, ft,
                                            &positional_explicitly_specified_template_arg_templatas, RegionT { region: IRegionT::Default }, args, &receiving_rune_to_explicit_template_arg_templata,
                                        )? {
                                            IResolveFunctionResult::ResolveFunctionFailure(failure) => {
                                                Ok(Err(IFindFunctionFailureReason::FindFunctionResolveFailure { reason: failure.reason }))
                                            }
                                            IResolveFunctionResult::ResolveFunctionSuccess(resolve_success) => {
                                                match self.params_match(
                                                    coutputs, calling_env, call_range, call_location,
                                                    args, &resolve_success.prototype.prototype.param_types(), exact,
                                                ) {
                                                    Err(rejection_reason) => Ok(Err(rejection_reason)),
                                                    Ok(()) => {
                                                        assert!(coutputs.get_instantiation_bounds(self.typing_interner, resolve_success.prototype.prototype.id).is_some());
                                                        Ok(Ok(AttemptedCandidate { prototype: resolve_success.prototype.prototype }))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            ICalleeCandidate::Header(_) => {
                panic!("implement: attemptCandidateBanner HeaderCalleeCandidate");
            }
            ICalleeCandidate::PrototypeTemplata(PrototypeTemplataCalleeCandidate { prototype_t }) => {
                // We get here if we're considering a function that's being passed in as a bound.
                let substituter = self.get_placeholder_substituter(
                    self.opts.global_options.sanity_check,
                    calling_env.denizen_template_id(),
                    prototype_t.id,
                    IBoundArgumentsSource::InheritBoundsFromTypeItself,
                );
                let func_name = IFunctionNameT::try_from(prototype_t.id.local_name).unwrap_or_else(|_| panic!("attemptCandidateBanner PrototypeTemplata: local_name not IFunctionNameT"));
                let params: Vec<CoordT<'s, 't>> = func_name.parameters().iter().map(|param_type| {
                    substituter.substitute_for_coord(coutputs, *param_type)
                }).collect();
                match self.params_match(coutputs, calling_env, call_range, call_location, args, &params, exact) {
                    Err(rejection_reason) => Ok(Err(rejection_reason)),
                    Ok(()) => {
                        assert!(coutputs.get_instantiation_bounds(self.typing_interner, prototype_t.id).is_some());
                        Ok(Ok(AttemptedCandidate { prototype: self.typing_interner.alloc(prototype_t) }))
                    }
                }
            }
        }
    }
/*
  private def attemptCandidateBanner(
    callingEnv: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    explicitTemplateArgRulesWithoutConnections: Vector[IRulexSR],
    positionalExplicitTemplateArgRunesS: Vector[IRuneS],
    receivingRuneToExplicitTemplateArgRune: Vector[(RuneUsage, RuneUsage)],
    contextRegion: RegionT,
    args: Vector[CoordT],
    candidate: ICalleeCandidate,
    exact: Boolean):
  Result[AttemptedCandidate, IFindFunctionFailureReason] = {
    candidate match {
      case FunctionCalleeCandidate(ft@FunctionTemplataT(declaringEnv, function)) => {
        // See OFCBT.
//        if (ft.function.isTemplate) {
//          function.tyype match {
//            case TemplateTemplataType(identifyingRuneTemplataTypes, FunctionTemplataType()) => {
        val identifyingRuneTemplataTypes = function.tyype.paramTypes
        // Now we want to check that the user didn't specify too many right here.
        // The function can inherit runes from its container, so subtract those first.
        val ownRuneCount = identifyingRuneTemplataTypes.size - receivingRuneToExplicitTemplateArgRune.size
        if (positionalExplicitTemplateArgRunesS.size > ownRuneCount) {
          return Err(WrongNumberOfTemplateArguments(positionalExplicitTemplateArgRunesS.size, ownRuneCount))
        }
        val explicitTemplateArgRulesWithConnections =
          explicitTemplateArgRulesWithoutConnections ++
            receivingRuneToExplicitTemplateArgRune.map({ case (receivingRune, callsiteRune) =>
              if (!function.genericParameters.exists(_.rune.rune == receivingRune.rune)) {
                throw CompileErrorExceptionT(RangedInternalErrorT(callRange, s"Supplied rune $receivingRune that doesn't exist in called function ${function.name}"))
              }
              EqualsSR(callsiteRune.range, receivingRune, callsiteRune)
            })

          // Now that we know what types are expected, we can FINALLY rule-type these explicitly
          // specified template args! (The rest of the rule-typing happened back in the astronomer,
          // this is the one time we delay it, see MDRTCUT).
          // Args supplied through receivingRuneToExplicitTemplateArgRune (the named channel for
          // container template args) also need their callsite rune seeded with the expected type,
          // otherwise MaybeCoercingLookupSR for those args can't fire in the rune-type solver.
          val receivingRuneToType: Map[IRuneS, ITemplataType] =
            function.genericParameters.map(gp => gp.rune.rune -> gp.tyype.tyype).toMap
          val callsiteRuneToType: Map[IRuneS, ITemplataType] =
            receivingRuneToExplicitTemplateArgRune.flatMap({ case (receivingRune, callsiteRune) =>
              receivingRuneToType.get(receivingRune.rune).map(callsiteRune.rune -> _)
            }).toMap
          val explicitTemplateArgRuneToType =
            callsiteRuneToType ++
          // There might be less explicitly specified template args than there are types, and that's
          // fine. Hopefully the rest will be figured out by the rule evaluator.
              positionalExplicitTemplateArgRunesS.zip(identifyingRuneTemplataTypes).toMap
          val runeTypeSolveEnv =
            new IRuneTypeSolverEnv {
              override def lookup(range: RangeS, nameS: IImpreciseNameS):
              Result[IRuneTypeSolverLookupResult, IRuneTypingLookupFailedError] = {
                callingEnv.lookupNearestWithImpreciseName(nameS, Set(TemplataLookupContext)) match {
                  case Some(x) => Ok(TemplataLookupResult(x.tyype))
                  case None => Err(RuneTypingCouldntFindType(range, nameS))
                }
              }
            }

          // And now that we know the types that are expected of these template arguments, we can
          // run these template argument templexes through the solver so it can evaluate them in
          // context of the current environment and spit out some templatas.
          runeTypeSolver.solve(
            opts.globalOptions.sanityCheck,
            opts.globalOptions.useOptimizedSolver,
            runeTypeSolveEnv,
            callRange,
            false,
            explicitTemplateArgRulesWithConnections,
            positionalExplicitTemplateArgRunesS ++ receivingRuneToExplicitTemplateArgRune.map(_._2.rune),
            true,
            explicitTemplateArgRuneToType) match {
            case Err(e@RuneTypeSolveError(_, _)) => {
              Err(RuleTypeSolveFailure(e))
            }
            case Ok(runeAToTypeWithImplicitlyCoercingLookupsS) => {
              // rulesA is the equals rules, but rule typed. Now we'll run them through the solver to get
              // some actual templatas.

              val runeTypeSolveEnv = TemplataCompiler.createRuneTypeSolverEnv(callingEnv)

              val runeAToType =
                mutable.HashMap[IRuneS, ITemplataType]((runeAToTypeWithImplicitlyCoercingLookupsS.toSeq): _*)
              // We've now calculated all the types of all the runes, but the LookupSR rules are still a bit
              // loose. We intentionally ignored the types of the things they're looking up, so we could know
              // what types we *expect* them to be, so we could coerce.
              // That coercion is good, but lets make it more explicit.
              val ruleBuilder = ArrayBuffer[IRulexSR]()
              explicifyLookups(
                runeTypeSolveEnv,
                runeAToType, ruleBuilder, explicitTemplateArgRulesWithConnections) match {
                case Err(RuneTypingTooManyMatchingTypes(range, name)) => throw CompileErrorExceptionT(TooManyTypesWithNameT(range :: callRange, name))
                case Err(RuneTypingCouldntFindType(range, name)) => throw CompileErrorExceptionT(CouldntFindTypeT(range :: callRange, name))
                case Ok(()) =>
              }
              val rulesWithoutImplicitCoercionsA = ruleBuilder.toVector

              // We preprocess out the rune parent env lookups, see MKRFA. Per @ECSIIOSZ, this is
              // the canonical per-call-site setup; other call-site solvers (ArrayCompiler etc.)
              // should mirror this shape before calling makeSolver/solveForResolving.
              //
              // ⚠ This fold is a recurring-bug pattern. MKRFA is unenforced; the value
              // solver's RuneParentEnvLookupSR handler is a silent no-op, so forgetting to
              // run this produces unrelated "couldn't solve" errors elsewhere. ArrayCompiler
              // sat in violation for ~4 years until April 2026. When adding a new
              // expression-scoped solver caller, either copy this fold verbatim OR (preferably)
              // land the shared-helper refactor in docs/refactor-thoughts/mkrfa-protocol-leak.md.
              val (initialKnowns, rulesWithoutRuneParentEnvLookups) =
                rulesWithoutImplicitCoercionsA.foldLeft((Vector[InitialKnown](), Vector[IRulexSR]()))({
                  case ((previousConclusions, remainingRules), RuneParentEnvLookupSR(_, rune)) => {
                    val templata =
                      vassertSome(
                        callingEnv.lookupNearestWithImpreciseName(
                          interner.intern(RuneNameS(rune.rune)), Set(TemplataLookupContext)))
                    val newConclusions = previousConclusions :+ InitialKnown(rune, templata)
                    (newConclusions, remainingRules)
                  }
                  case ((previousConclusions, remainingRules), rule) => {
                    (previousConclusions, remainingRules :+ rule)
                  }
                })

//                  val callEnv =
//                    GeneralEnvironment.childOf(
//                      interner, callingEnv, callingEnv.fullName.addStep(CallEnvNameT()))

              // We only want to solve the template arg runes
              inferCompiler.solveForResolving(
                InferEnv(callingEnv, callRange, callLocation, declaringEnv, contextRegion),
                coutputs,
                rulesWithoutRuneParentEnvLookups,
                explicitTemplateArgRuneToType ++ runeAToType,
                callRange,
                callLocation,
                Vector(),
                initialKnowns,
                Vector()) match {
                case (Err(e)) => {
                  Err(FindFunctionResolveFailure(e))
                }
                case (Ok(CompleteResolveSolve(explicitRuneSToTemplata, _))) => {
                  val positionalExplicitlySpecifiedTemplateArgTemplatas =
                    positionalExplicitTemplateArgRunesS.map(explicitRuneSToTemplata)
                  val receivingRuneToExplicitTemplateArgTemplata =
                    receivingRuneToExplicitTemplateArgRune.map({ case (receivingRune, explicitTemplateArgRune) =>
                      InitialKnown(
                        receivingRune,
                        vassertSome(explicitRuneSToTemplata.get(explicitTemplateArgRune.rune)))
                    })

                  if (ft.function.isLambda()) {
                    vassert(receivingRuneToExplicitTemplateArgTemplata.isEmpty) // implement
                    // We pass in our env because the callee needs to see functions declared here, see CSSNCE.
                    functionCompiler.evaluateTemplatedFunctionFromCallForPrototype(
                      coutputs, callingEnv, callRange, callLocation, ft, positionalExplicitlySpecifiedTemplateArgTemplatas.toVector, contextRegion, args) match {
                      case (EvaluateFunctionFailure(reason)) => Err(CouldntEvaluateTemplateError(reason))
                      case (EvaluateFunctionSuccess(prototype, conclusions, _)) => {
                        paramsMatch(coutputs, callingEnv, callRange, callLocation, args, prototype.prototype.paramTypes, exact) match {
                          case Err(rejectionReason) => Err(rejectionReason)
                          case Ok(()) => {
                            vassert(coutputs.getInstantiationBounds(prototype.prototype.id).nonEmpty)
                            Ok(AttemptedCandidate(prototype.prototype))
                          }
                        }
                      }
                    }
                  } else {
                    // We pass in our env because the callee needs to see functions declared here, see CSSNCE.
                    functionCompiler.evaluateGenericLightFunctionFromCallForPrototype(
                      coutputs, callRange, callLocation, callingEnv, ft, positionalExplicitlySpecifiedTemplateArgTemplatas.toVector, RegionT(DefaultRegionT), args,
                      receivingRuneToExplicitTemplateArgTemplata) match {
                      case (ResolveFunctionFailure(reason)) => Err(FindFunctionResolveFailure(reason))
                      case (ResolveFunctionSuccess(prototype, conclusions)) => {
                        paramsMatch(coutputs, callingEnv, callRange, callLocation, args, prototype.prototype.paramTypes, exact) match {
                          case Err(rejectionReason) => Err(rejectionReason)
                          case Ok(()) => {
                            vassert(coutputs.getInstantiationBounds(prototype.prototype.id).nonEmpty)
                            Ok(AttemptedCandidate(prototype.prototype))
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      case HeaderCalleeCandidate(header) => {
        paramsMatch(coutputs, callingEnv, callRange, callLocation, args, header.paramTypes, exact) match {
          case Ok(_) => {
            Ok(AttemptedCandidate(header.toPrototype))
          }
          case Err(fff) => Err(fff)
        }
      }
      case PrototypeTemplataCalleeCandidate(prototype) => {
        // We get here if we're considering a function that's being passed in as a bound.
        val substituter =
          TemplataCompiler.getPlaceholderSubstituter(
            opts.globalOptions.sanityCheck,
            interner,
            keywords,
            callingEnv.denizenTemplateId,
            prototype.id,
            // These types are phrased in terms of the calling denizen already, so we can grab their
            // bounds.
            InheritBoundsFromTypeItself)
        val params = prototype.id.localName.parameters.map(paramType => {
          substituter.substituteForCoord(coutputs, paramType)
        })
        paramsMatch(coutputs, callingEnv, callRange, callLocation, args, params, exact) match {
          case Ok(_) => {
            // This can be for example:
            //   func bork<T>(a T) where func drop(T)void {
            //     drop(a);
            //   }
            // We're calling a function that came from a bound.
            // Function bounds (like the `func drop(T)void` don't have bounds themselves)
            // so we just supply an empty map here.
            val bounds = Map[IRuneS, PrototypeTemplataT[IFunctionNameT]]()

            vassert(coutputs.getInstantiationBounds(prototype.id).nonEmpty)
            Ok(AttemptedCandidate(prototype))
          }
          case Err(fff) => Err(fff)
        }
      }
    }
  }

*/
    pub fn get_param_environments(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        param_filters: &[CoordT<'s, 't>],
    ) -> Vec<IInDenizenEnvironmentT<'s, 't>> {
        param_filters.iter().flat_map(|tyype| {
            match tyype.kind {
                KindT::Struct(sr) => { vec![coutputs.get_outer_env_for_type(range, self.get_struct_template(sr.id))] }
                KindT::Interface(ir) => { vec![coutputs.get_outer_env_for_type(range, self.get_interface_template(ir.id))] }
                KindT::KindPlaceholder(kp) => { vec![coutputs.get_outer_env_for_type(range, self.get_placeholder_template(kp.id))] }
                _ => Vec::new()
            }
        }).collect()
    }

    pub fn get_placeholder_extra_call_envs(
        &self,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        range: &[RangeS<'s>],
        param_filters: &[CoordT<'s, 't>],
    ) -> Vec<IInDenizenEnvironmentT<'s, 't>> {
        let mut collected: Vec<IInDenizenEnvironmentT<'s, 't>> = Vec::new();
        let mut seen: HashSet<IdT<'s, 't>> = HashSet::new();
        // Look through each parameter, and if it's a placeholder that impls an interface, grab
        // the interface env so that callers can look inside them for methods too (see @BDPFWDZ).
        for tyype in param_filters.iter() {
            match tyype.kind {
                KindT::KindPlaceholder(kp) => {
                    let placeholder_imprecise = match get_imprecise_name(self.scout_arena, kp.id.local_name) {
                        None => panic!("Placeholder localName had no imprecise name: {:?}", kp.id.local_name),
                        Some(n) => n,
                    };
                    let impl_key = self.scout_arena.intern_imprecise_name(
                        IImpreciseNameValS::ImplSubCitizenImpreciseName(ImplSubCitizenImpreciseNameValS { sub_citizen_imprecise_name: placeholder_imprecise }));
                    let lookup_filter = [ILookupContext::TemplataLookupContext].into_iter().collect::<HashSet<_>>();
                    let matching = calling_env.lookup_all_with_imprecise_name(impl_key, lookup_filter, self.typing_interner);
                    for m in matching {
                        match m {
                            ITemplataT::Isa(&IsaTemplataT { super_kind: KindT::Interface(super_id), .. }) => {
                                let template_id = self.get_interface_template(super_id.id);
                                if !seen.contains(&template_id) {
                                    seen.insert(template_id);
                                    collected.push(coutputs.get_outer_env_for_type(range, template_id));
                                }
                            }
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }
        collected
    }
/*
  // Gets all the environments for all the arguments.
  private def getParamEnvironments(coutputs: CompilerOutputs, range: List[RangeS], paramFilters: Vector[CoordT]):
  Vector[IInDenizenEnvironmentT] = {
    paramFilters.flatMap({ case tyype =>
      (tyype.kind match {
        case sr @ StructTT(_) => Vector(coutputs.getOuterEnvForType(range, TemplataCompiler.getStructTemplate(sr.id)))
        case ir @ InterfaceTT(_) => Vector(coutputs.getOuterEnvForType(range, TemplataCompiler.getInterfaceTemplate(ir.id)))
        case KindPlaceholderT(id) => Vector(coutputs.getOuterEnvForType(range, TemplataCompiler.getPlaceholderTemplate(id)))
        case _ => Vector.empty
      })
    })
  }

  private def getPlaceholderExtraCallEnvs(
    callingEnv: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    range: List[RangeS],
    paramFilters: Vector[CoordT]
  ): Vector[IInDenizenEnvironmentT] = {
    val collected = ArrayBuffer[IInDenizenEnvironmentT]()
    val seen = mutable.Set[IdT[INameT]]()
    // Look through each parameter, and if it's a placeholder that impls an interface, grab
    // the interface env so that callers can look inside them for methods too (see @BDPFWDZ).
    paramFilters.foreach({ case tyype =>
      tyype.kind match {
        case KindPlaceholderT(id) => {
          val placeholderImprecise =
            TemplatasStore.getImpreciseName(interner, id.localName) match {
              case None => vfail("Placeholder localName had no imprecise name: " + id.localName)
              case Some(n) => n
            }
          val implKey = interner.intern(ImplSubCitizenImpreciseNameS(placeholderImprecise))
          val matching = callingEnv.lookupAllWithImpreciseName(implKey, Set(TemplataLookupContext))
          matching.foreach({
            case IsaTemplataT(_, _, _, InterfaceTT(superId)) => {
              val templateId = TemplataCompiler.getInterfaceTemplate(superId)
              if (!seen.contains(templateId)) {
                seen.add(templateId)
                collected += coutputs.getOuterEnvForType(range, templateId)
              }
            }
            case _ =>
          })
        }
        case _ =>
      }
    })
    collected.toVector
  }

*/
    pub fn find_potential_function(
        &self,
        env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        function_name: IImpreciseNameS<'s>,
        explicit_template_arg_rules_s: &[IRulexSR<'s>],
        positional_explicit_template_arg_runes_s: &[IRuneS<'s>],
        receiving_rune_to_explicit_template_arg_rune: &[(RuneUsage<'s>, RuneUsage<'s>)],
        context_region: RegionT,
        args: &[CoordT<'s, 't>],
        extra_envs_to_look_in: &[IInDenizenEnvironmentT<'s, 't>],
        exact: bool,
    ) -> Result<Result<AttemptedCandidate<'s, 't>, FindFunctionFailure<'s, 't>>, ICompileErrorT<'s, 't>> {
        // This is here for debugging, so when we dont find something we can see what envs we searched
        let mut searched_envs: Vec<SearchedEnvironment<'s, 't>> = Vec::new();
        let mut undeduped_candidates: Vec<ICalleeCandidate<'s, 't>> = Vec::new();
        self.get_candidate_banners(
            env, coutputs, call_range, function_name, args, extra_envs_to_look_in,
            &mut searched_envs, &mut undeduped_candidates);
        let mut seen = HashSet::new();
        let candidates: Vec<ICalleeCandidate<'s, 't>> =
            undeduped_candidates.into_iter().filter(|c| seen.insert(*c)).collect();
        let mut successes: Vec<AttemptedCandidate<'s, 't>> = Vec::new();
        let mut failed_to_reason: Vec<(ICalleeCandidate<'s, 't>, IFindFunctionFailureReason<'s, 't>)> = Vec::new();
        for candidate in candidates.iter() {
            match self.attempt_candidate_banner(
                env, coutputs, call_range, call_location, explicit_template_arg_rules_s,
                positional_explicit_template_arg_runes_s,
                receiving_rune_to_explicit_template_arg_rune,
                context_region, args, *candidate, exact)?
            {
                Ok(s) => { successes.push(s); }
                Err(e) => { failed_to_reason.push((*candidate, e)); }
            }
        }

        if successes.is_empty() {
            Ok(Err(FindFunctionFailure {
                name: function_name,
                args: self.typing_interner.alloc_slice_copy(args),
                rejected_callee_to_reason: self.typing_interner.alloc_slice_from_vec(failed_to_reason),
            }))
        } else if successes.len() == 1 {
            Ok(Ok(successes.into_iter().next().unwrap()))
        } else {
            let (best, _outscore_reason_by_banner) =
                self.narrow_down_callable_overloads(coutputs, env, call_range, call_location, &successes, args)?;
            Ok(Ok(best))
        }
    }
/*
  // Checks to see if there's a function that *could*
  // exist that takes in these parameter types, and returns what the signature *would* look like.
  // Only considers when arguments match exactly.
  // If given something in maybeSuperInterfaceRef2, it will search for a function that
  // overrides that interfaceTT in that position. If we ever support multimethods we
  // might need to take a list of these, same length as the arg types... or combine
  // them somehow.
  // sure that we're really looking for only a "potential" function?
  def findPotentialFunction(
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
  Result[AttemptedCandidate, FindFunctionFailure] = {
    // This is here for debugging, so when we dont find something we can see what envs we searched
    val searchedEnvs = new Accumulator[SearchedEnvironment]()
    val undedupedCandidates = new Accumulator[ICalleeCandidate]()
    getCandidateBanners(
      env, coutputs, callRange, functionName, args, extraEnvsToLookIn, searchedEnvs, undedupedCandidates)
    val candidates = undedupedCandidates.buildArray().distinct
    val attempted =
      candidates.map(candidate => {
        attemptCandidateBanner(
          env, coutputs, callRange, callLocation, explicitTemplateArgRulesS,
          positionalExplicitTemplateArgRunesS,
          receivingRuneToExplicitTemplateArgRune,
          contextRegion, args, candidate, exact)
          .mapError(e => (candidate -> e))
      })
    val (successes, failedToReason) = Result.split(attempted)

    if (successes.isEmpty) {
      Err(FindFunctionFailure(functionName, args, failedToReason))
    } else if (successes.size == 1) {
      Ok(successes.head)
    } else {
      val (best, outscoreReasonByBanner) =
        narrowDownCallableOverloads(coutputs, env, callRange, callLocation, successes, args)
      Ok(best)
    }
  }

*/
    pub fn get_banner_param_scores(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        candidate: &'t PrototypeT<'s, 't>,
        arg_types: &[CoordT<'s, 't>],
    ) -> Option<Vec<bool>> {
        let initial: Option<Vec<bool>> = Some(Vec::new());
        let result = candidate.param_types().iter().zip(arg_types.iter()).fold(initial, |acc, (param_type, arg_type)| {
            match acc {
                None => None,
                Some(mut previous) => {
                    if arg_type == param_type {
                        previous.push(false);
                        Some(previous)
                    } else {
                        if self.is_type_convertible(coutputs, calling_env, parent_ranges, call_location, *arg_type, *param_type) {
                            previous.push(true);
                            Some(previous)
                        } else {
                            None
                        }
                    }
                }
            }
        });
        if let Some(ref a) = result {
            assert_eq!(a.len(), arg_types.len());
        }
        result
    }
/*
  // Returns either:
  // - None if banners incompatible
  // - Some(param to needs-conversion)
  private def getBannerParamScores(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    candidate: PrototypeT[IFunctionNameT],
    argTypes: Vector[CoordT]):
  (Option[Vector[Boolean]]) = {
    val initial: Option[Vector[Boolean]] = Some(Vector())
    val result =
    candidate.paramTypes.zip(argTypes)
      .foldLeft(initial)({
        case (None, _) => None
        case (Some(previous), (paramType, argType)) => {
          if (argType == paramType) {
            Some(previous :+ false)
          } else {
            if (templataCompiler.isTypeConvertible(coutputs, callingEnv, parentRanges, callLocation, argType, paramType)) {
              Some(previous :+ true)
            } else {
              None
            }
          }
        }
      })
    result match {
      case Some(a) => vassert(a.length == argTypes.length)
      case None =>
    }
    result
  }

*/
    pub fn narrow_down_callable_overloads(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        unfiltered_banners: &[AttemptedCandidate<'s, 't>],
        arg_types: &[CoordT<'s, 't>],
    ) -> Result<(AttemptedCandidate<'s, 't>, HashMap<AttemptedCandidate<'s, 't>, IFindFunctionFailureReason<'s, 't>>), ICompileErrorT<'s, 't>> {
        let deduped_banners: Vec<AttemptedCandidate<'s, 't>> = {
            let mut seen = HashSet::new();
            unfiltered_banners.iter().filter(|b| seen.insert(**b)).copied().collect()
        };
        // Group by paramTypes, prefer ordinary over bound
        let mut param_types_to_banners: HashMap<Vec<CoordT<'s, 't>>, Vec<AttemptedCandidate<'s, 't>>> = HashMap::new();
        for banner in &deduped_banners {
            param_types_to_banners.entry(banner.prototype.param_types().to_vec()).or_default().push(*banner);
        }
        let banners: Vec<AttemptedCandidate<'s, 't>> = param_types_to_banners.into_values().flat_map(|v| v).collect();

        let banner_index_to_score: Vec<Vec<bool>> =
            banners.iter().map(|banner| {
                self.get_banner_param_scores(coutputs, calling_env, call_range, call_location, banner.prototype, arg_types)
                    .unwrap_or_else(|| panic!("vassertSome: getBannerParamScores"))
            }).collect();

        let param_index_to_surviving_banner_indices: Vec<Vec<usize>> =
            (0..arg_types.len()).map(|param_index| {
                let banner_index_to_requires_conversion: Vec<bool> =
                    banner_index_to_score.iter().map(|scores| scores[param_index]).collect();
                if banner_index_to_requires_conversion.iter().all(|&b| b) {
                    (0..banner_index_to_score.len()).collect()
                } else if banner_index_to_requires_conversion.iter().all(|&b| !b) {
                    (0..banner_index_to_score.len()).collect()
                } else {
                    banner_index_to_requires_conversion.iter().enumerate()
                        .filter(|(_, &req)| req).map(|(i, _)| i).collect()
                }
            }).collect();

        let all_indices: Vec<usize> = (0..banner_index_to_score.len()).collect();
        let surviving_banner_indices: Vec<usize> =
            param_index_to_surviving_banner_indices.iter().fold(all_indices, |a, b| {
                a.into_iter().filter(|i| b.contains(i)).collect()
            });

        // Split normal vs bound candidates
        let mut normal_indices_and_candidates: Vec<(usize, &'t PrototypeT<'s, 't>)> = Vec::new();
        let mut bound_indices_and_candidates: Vec<(usize, &'t PrototypeT<'s, 't>)> = Vec::new();
        for &i in &surviving_banner_indices {
            let candidate = &banners[i];
            match candidate.prototype.id.local_name {
                INameT::FunctionBound(_) => { bound_indices_and_candidates.push((i, candidate.prototype)); }
                _ => { normal_indices_and_candidates.push((i, candidate.prototype)); }
            }
        }

        let final_banner_index =
            if normal_indices_and_candidates.len() > 1 {
                let duplicate_banners: Vec<PrototypeT<'s, 't>> =
                    normal_indices_and_candidates.iter().map(|(_, p)| **p).collect();
                return Err(ICompileErrorT::CouldntNarrowDownCandidates {
                    range: self.typing_interner.alloc_slice_copy(call_range),
                    candidates: self.typing_interner.alloc_slice_from_vec(duplicate_banners),
                });
            } else if normal_indices_and_candidates.len() == 1 {
                normal_indices_and_candidates[0].0
            } else if !bound_indices_and_candidates.is_empty() {
                let mut sorted_by_steps = bound_indices_and_candidates.clone();
                sorted_by_steps.sort_by_key(|(_, proto)| proto.id.steps().len());
                let (shortest_candidate_index, shortest_candidate) = sorted_by_steps[0];
                for (_, other_candidate) in sorted_by_steps.iter().skip(1) {
                    assert!(other_candidate.id.init_steps.starts_with(shortest_candidate.id.init_steps));
                }
                shortest_candidate_index
            } else {
                panic!("No candidate is a clear winner!")
            };

        let rejection_reason_by_banner: HashMap<AttemptedCandidate<'s, 't>, IFindFunctionFailureReason<'s, 't>> =
            banners.iter().enumerate()
                .filter(|(i, _)| *i != final_banner_index)
                .map(|(_, banner)| (*banner, IFindFunctionFailureReason::Outscored))
                .collect();

        Ok((banners[final_banner_index], rejection_reason_by_banner))
    }
/*
  private def narrowDownCallableOverloads(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    unfilteredBanners: Iterable[AttemptedCandidate],
    argTypes: Vector[CoordT]):
  (
    AttemptedCandidate,
    // Rejection reason by banner
    Map[AttemptedCandidate, IFindFunctionFailureReason]) = {

    // Sometimes a banner might come from many different environments (remember,
    // when we do a call, we look in the environments of all the arguments' types).
    // Here we weed out these duplicates.
    val dedupedBanners =
//      unfilteredBanners.foldLeft(Vector[IValidCalleeCandidate]())({
//        case (potentialBannerByBannerSoFar, currentPotentialBanner) => {
//          if (potentialBannerByBannerSoFar.exists(_.range == currentPotentialBanner.range)) {
//            potentialBannerByBannerSoFar
//          } else {
//            potentialBannerByBannerSoFar :+ currentPotentialBanner
//          }
//        }
//      })
        unfilteredBanners.toVector.distinct

    // If there are multiple overloads with the same exact parameter list,
    // then get rid of the templated ones; ordinary ones get priority.
    val banners =
      dedupedBanners.groupBy(_.prototype.paramTypes).values.flatMap({ potentialBannersWithSameParamTypes =>
        val ordinaryBanners =
          potentialBannersWithSameParamTypes
        if (ordinaryBanners.isEmpty) {
          // No ordinary banners, so include all the templated ones
          potentialBannersWithSameParamTypes
        } else {
          // There are some ordinary banners, so only consider the ordinary banners
          ordinaryBanners
        }
      }).toVector

    val bannerIndexToScore =
      banners.map(banner => {
        vassertSome(getBannerParamScores(coutputs, callingEnv, callRange, callLocation, banner.prototype, argTypes))
      })

    // For any given parameter:
    // - If all candidates require a conversion, keep going
    //   (This might be a mistake, should we throw an error instead?)
    // - If no candidates require a conversion, keep going
    // - If some candidates require a conversion, disqualify those candidates

    val paramIndexToSurvivingBannerIndices =
      argTypes.indices.map(paramIndex => {
        val bannerIndexToRequiresConversion =
          bannerIndexToScore.zipWithIndex.map({
            case (paramIndexToScore, bannerIndex) => paramIndexToScore(paramIndex)
          })
        if (bannerIndexToRequiresConversion.forall(_ == true)) {
          // vfail("All candidates require conversion for param " + paramIndex)
          bannerIndexToScore.indices
        } else if (bannerIndexToRequiresConversion.forall(_ == false)) {
          bannerIndexToScore.indices
        } else {
          val survivingBannerIndices =
            bannerIndexToRequiresConversion.zipWithIndex.filter(_._1).map(_._2)
          survivingBannerIndices
        }
      })
    // Now, each parameter knows what candidates it disqualifies.
    // See if there's exactly one candidate that all parameters agree on.
    val survivingBannerIndices =
      paramIndexToSurvivingBannerIndices.foldLeft(bannerIndexToScore.indices.toVector)({
        case (a, b) => a.intersect(b)
      })

    val (normalIndicesAndCandidates, boundIndicesAndCandidates) =
      survivingBannerIndices
          .map(i => i -> banners(i))
          .foldLeft((List[(Int, PrototypeT[IFunctionNameT])](), List[(Int, PrototypeT[FunctionBoundNameT])]()))({
            case ((normalCandidates, boundCandidates), (index, thisCandidate)) => {
              thisCandidate.prototype match {
                case PrototypeT(IdT(packageCoord, initSteps, FunctionBoundNameT(tn, firstTemplateArgs, firstParameters)), firstReturnType) => {
                  val thisBoundCandidate = PrototypeT(IdT(packageCoord, initSteps, FunctionBoundNameT(tn, firstTemplateArgs, firstParameters)), firstReturnType)
                  (normalCandidates, (index -> thisBoundCandidate) :: boundCandidates)
                }
                case other => {
                  ((index -> other) :: normalCandidates, boundCandidates)
                }
              }
            }
          })

    val finalBannerIndex =
      if (normalIndicesAndCandidates.size > 1) {
        val duplicateBanners = normalIndicesAndCandidates.map(_._2)
        throw CompileErrorExceptionT(
          CouldntNarrowDownCandidates(
            callRange,
            duplicateBanners.toVector))
      } else if (normalIndicesAndCandidates.size == 1) {
        normalIndicesAndCandidates.head._1
      } else if (boundIndicesAndCandidates.nonEmpty) {
        val sortedByNameLength = boundIndicesAndCandidates.sortBy(_._2.id.steps.length)
        val (shortestCandidateIndex, shortestCandidate) = sortedByNameLength.head
        sortedByNameLength.tail.foreach(otherCandidate => {
          vassert(otherCandidate._2.id.initSteps.startsWith(shortestCandidate.id.initSteps))
//          val duplicateBanners = normalIndicesAndCandidates.map(_._2)
//          throw CompileErrorExceptionT(
//            CouldntNarrowDownCandidates(
//              callRange,
//              vimpl()))
        })
        shortestCandidateIndex
      } else {
        vfail("No candidate is a clear winner!")
      }

    val rejectedBanners =
      banners.zipWithIndex.filter(_._2 != finalBannerIndex).map(_._1)
    val rejectionReasonByBanner =
      rejectedBanners.map((_, Outscored())).toMap

    (banners(finalBannerIndex), rejectionReasonByBanner)
  }

//  def stampPotentialFunctionForBanner(
//    callingEnv: IDenizenEnvironmentBoxT,
//    coutputs: CompilerOutputs,
//    callRange: List[RangeS],
//    callLocation: LocationInDenizen,
//    potentialBanner: PrototypeT[IFunctionNameT],
//    contextRegion: RegionT,
//    verifyConclusions: Boolean):
//  (PrototypeTemplataT[IFunctionNameT]) = {
//    potentialBanner match {
////      case ValidCalleeCandidate(banner, _, ft @ FunctionTemplataT(_, _)) => {
//////        if (ft.function.isTemplate) {
////          val (EvaluateFunctionSuccess(successBanner, conclusions, _)) =
////            functionCompiler.evaluateTemplatedLightFunctionFromCallForPrototype(
////              coutputs, callingEnv, callRange, callLocation, ft, Vector.empty, contextRegion, banner.paramTypes);
////          successBanner
//////        } else {
//////          functionCompiler.evaluateOrdinaryFunctionFromNonCallForBanner(
//////            coutputs, callRange, ft, verifyConclusions)
//////        }
////      }
//      case ValidHeaderCalleeCandidate(header) => {
//        vassert(coutputs.getInstantiationBounds(header.toPrototype.id).nonEmpty)
//        PrototypeTemplataT(header.toPrototype)
//      }
//    }
//  }

//  private def stampPotentialFunctionForPrototype(
//    coutputs: CompilerOutputs,
//    callingEnv: IInDenizenEnvironmentT, // See CSSNCE
//    callRange: List[RangeS],
//    callLocation: LocationInDenizen,
//    potentialBanner: PrototypeT[IFunctionNameT],
//    contextRegion: RegionT,
//    args: Vector[CoordT]):
//  StampFunctionSuccess = {
//    potentialBanner match {
////      case ValidCalleeCandidate(header, templateArgs, ft @ FunctionTemplataT(_, _)) => {
////        if (ft.function.isLambda()) {
//////          if (ft.function.isTemplate) {
////            functionCompiler.evaluateTemplatedFunctionFromCallForPrototype(
////                coutputs,callRange, callLocation, callingEnv, ft, templateArgs, contextRegion, args) match {
////              case EvaluateFunctionSuccess(prototype, inferences, _) => StampFunctionSuccess(prototype, inferences)
////              case (eff@EvaluateFunctionFailure(_)) => vfail(eff.toString)
////            }
//////          } else {
//////            // debt: look into making FunctionCompiler's methods accept function templatas
//////            // so we dont pass in the wrong environment again
//////            functionCompiler.evaluateOrdinaryFunctionFromCallForPrototype(
//////              coutputs, callingEnv, callRange, ft)
//////          }
////        } else {
////          functionCompiler.evaluateGenericLightFunctionFromCallForPrototype(
////            coutputs, callRange, callLocation, callingEnv, ft, templateArgs, contextRegion, args) match {
////            case ResolveFunctionSuccess(prototype, inferences) => {
////              StampFunctionSuccess(prototype, inferences)
////            }
////            case (ResolveFunctionFailure(fffr)) => {
////              throw CompileErrorExceptionT(TypingPassResolvingError(callRange, fffr))
////            }
////          }
////        }
////      }
//      case ValidHeaderCalleeCandidate(header) => {
////        val declarationRange = vassertSome(header.maybeOriginFunctionTemplata).function.range
//        vassert(coutputs.getInstantiationBounds(header.toPrototype.id).nonEmpty)
//        StampFunctionSuccess(PrototypeTemplataT(header.toPrototype), Map())
//      }
//      case ValidPrototypeTemplataCalleeCandidate(prototype) => {
//        vassert(coutputs.getInstantiationBounds(prototype.prototype.id).nonEmpty)
//        StampFunctionSuccess(prototype, Map())
//      }
//    }
//  }

*/
    pub fn get_array_generator_prototype(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        calling_env: IInDenizenEnvironmentT<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        callable_te: ReferenceExpressionTE<'s, 't>,
        context_region: RegionT,
    ) -> &'t PrototypeT<'s, 't> {
        let func_name = self.scout_arena.intern_imprecise_name(
            IImpreciseNameValS::CodeName(CodeNameS { name: self.keywords.underscores_call }));
        let param_filters = vec![
            callable_te.result().underlying_coord(),
            CoordT {
                ownership: OwnershipT::Share,
                region: RegionT { region: IRegionT::Default },
                kind: KindT::Int(IntT { bits: 32 }),
            },
        ];
        match self.find_function(calling_env, coutputs, range, call_location, func_name, &[], &[], &[], context_region, &param_filters, &[], false)
            .unwrap_or_else(|_e| panic!("Unimplemented: get_array_generator_prototype — propagated ICompileErrorT"))
        {
            Err(_e) => panic!("CouldntFindFunctionToCallT"),
            Ok(sfs) => sfs.prototype,
        }
    }
/*
  def getArrayGeneratorPrototype(
    coutputs: CompilerOutputs,
    callingEnv: IInDenizenEnvironmentT,
    range: List[RangeS],
    callLocation: LocationInDenizen,
    callableTE: ReferenceExpressionTE,
    contextRegion: RegionT):
  PrototypeT[IFunctionNameT] = {
    val funcName = interner.intern(CodeNameS(keywords.underscoresCall))
    val paramFilters =
      Vector(
        callableTE.result.underlyingCoord,
        CoordT(ShareT, RegionT(DefaultRegionT), IntT.i32))
      findFunction(
        callingEnv, coutputs, range, callLocation, funcName, Vector.empty, Vector.empty, Vector.empty, contextRegion,
        paramFilters, Vector.empty, false) match {
        case Err(e) => throw CompileErrorExceptionT(CouldntFindFunctionToCallT(range, e))
        case Ok(x) => x.prototype
      }
  }

*/
    // Rust adaptation: arena-allocated ReferenceExpressionTE — caller needs to keep the value to pass to DestroyStaticSizedArrayIntoFunctionTE, so we take &'t.
    pub fn get_array_consumer_prototype(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        fate: &'t FunctionEnvironmentT<'s, 't>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        callable_te: ReferenceExpressionTE<'s, 't>,
        element_type: CoordT<'s, 't>,
        context_region: RegionT,
    ) -> Result<&'t PrototypeT<'s, 't>, ICompileErrorT<'s, 't>> {
        let func_name = self.scout_arena.intern_imprecise_name(
            IImpreciseNameValS::CodeName(CodeNameS { name: self.keywords.underscores_call }));
        let param_filters = vec![callable_te.result().underlying_coord(), element_type];
        let calling_env = IInDenizenEnvironmentT::from(fate);
        match self.find_function(calling_env, coutputs, range, call_location, func_name, &[], &[], &[], context_region, &param_filters, &[], false)?
        {
            Err(_e) => panic!("CouldntFindFunctionToCallT"),
            Ok(sfs) => Ok(sfs.prototype),
        }
    }
/*
  def getArrayConsumerPrototype(
    coutputs: CompilerOutputs,
    fate: FunctionEnvironmentBoxT,
    range: List[RangeS],
    callLocation: LocationInDenizen,
    callableTE: ReferenceExpressionTE,
    elementType: CoordT,
    contextRegion: RegionT):
  PrototypeT[IFunctionNameT] = {
    val funcName = interner.intern(CodeNameS(keywords.underscoresCall))
    val paramFilters =
      Vector(
        callableTE.result.underlyingCoord,
        elementType)
    findFunction(
      fate.snapshot, coutputs, range, callLocation, funcName, Vector.empty, Vector.empty, Vector.empty, contextRegion, paramFilters, Vector.empty, false) match {
      case Err(e) => throw CompileErrorExceptionT(CouldntFindFunctionToCallT(range, e))
      case Ok(x) => x.prototype
    }
  }
}
*/
}
