use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::postparsing::ast::*;
use crate::postparsing::ast::*;
use crate::postparsing::itemplatatype::{ITemplataType, KindTemplataType};
use crate::postparsing::names::*;
use crate::postparsing::rules::rules::EqualsSR;
use crate::postparsing::rules::*;
use crate::postparsing::*;
use crate::solver::solver::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::ast::expressions::*;
use crate::typing::compilation::TypingPassOptions;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::function::function_compiler::*;
use crate::typing::hinputs_t::InstantiationBoundArgumentsT;
use crate::typing::infer::compiler_solver::ITypingPassSolverError;
use crate::typing::infer_compiler::IConclusionResolveError;
use crate::typing::infer_compiler::{
  include_rule_in_call_site_solve, include_rule_in_definition_solve, CompleteDefineSolve,
  CompleteResolveSolve, IDefiningError, IResolvingError, InferEnv, InitialKnown, InitialSend,
};
use crate::typing::names::names::*;
use crate::typing::templata::templata::*;
use crate::typing::templata_compiler::{peel_all_references, peel_n_references};
use crate::typing::types::types::*;
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::fx::{HashMap, HashSet};
use crate::utils::fx::{IndexMap, IndexSet};
use crate::utils::range::RangeS;
use std::iter::empty;
use std::marker::PhantomData;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
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
    args: &[KindT<'s, 't>],
  ) -> IEvaluateFunctionResult<'s, 't> {
    panic!("Unimplemented: evaluate_templated_function_from_call_for_prototype");
    // val function = outerEnv.function
    // checkClosureConcernsHandled(outerEnv)
    // val callSiteRules = TemplataCompiler.assembleCallSiteRules(function.rules)
    // val initialSends = assembleInitialSendsFromArgs(callRange.head, function, args.map(Some(_)))
    // val CompleteDefineSolve(inferredTemplatas, instantiationBoundParams) =
    //   inferCompiler.solveForDefining(
    //     InferEnv(originalCallingEnv, callRange, callLocation, outerEnv, contextRegion),
    //     coutputs, callSiteRules, function.runeToType, callRange, callLocation,
    //     assembleKnownTemplatas(function, explicitTemplateArgs), initialSends, Vector()
    //   ) match {
    //     case Err(e) => throw CompileErrorExceptionT(TypingPassDefiningError(callRange, e))
    //     case Ok(i) => (i)
    //   }
    // val runedEnv =
    //   addRunedDataToNearEnv(
    //     outerEnv, function.genericParameters.map(_.rune.rune), inferredTemplatas,
    //     instantiationBoundParams.runeToCitizenRuneToReachablePrototype.values.flatMap(_.citizenRuneToReachablePrototype.values).toVector.map(PrototypeTemplataT(_)))
    // val header =
    //   middleLayer.getOrEvaluateFunctionForHeader(
    //     outerEnv, runedEnv, coutputs, callRange, callLocation, function, instantiationBoundParams)
    // // Lambdas cant have bounds, right?
    // vcurious(instantiationBoundParams.runeToBoundPrototype.isEmpty)
    // vcurious(instantiationBoundParams.runeToCitizenRuneToReachablePrototype.isEmpty)
    // vcurious(instantiationBoundParams.runeToBoundImpl.isEmpty)
    // val instantiationBoundArgs =
    //   InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT](
    //     instantiationBoundParams.runeToBoundPrototype,
    //     instantiationBoundParams.runeToCitizenRuneToReachablePrototype.map({ case (x, InstantiationReachableBoundArgumentsT(y)) =>
    //       x -> InstantiationReachableBoundArgumentsT[IFunctionNameT](y)
    //     }),
    //     instantiationBoundParams.runeToBoundImpl)
    // coutputs.addInstantiationBounds(
    //   opts.globalOptions.sanityCheck, interner, outerEnv.denizenTemplateId,
    //   header.id, instantiationBoundArgs)
    // EvaluateFunctionSuccess(PrototypeTemplataT(header.toPrototype), inferredTemplatas, instantiationBoundArgs)
  }

  pub fn evaluate_templated_function_from_call_for_banner(
    &self,
    declaring_env: &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    original_calling_env: IInDenizenEnvironmentT<'s, 't>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    already_specified_template_args: &[ITemplataT<'s, 't>],
    context_region: RegionT,
    args: &[KindT<'s, 't>],
  ) -> Result<IEvaluateFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
    let function = declaring_env.function;
    // Check preconditions
    self.check_closure_concerns_handled(declaring_env);

    let all_rules: Vec<IRulexSR<'s>> = function
      .header_rules
      .iter()
      .copied()
      .chain(function.params.iter().flat_map(|p| {
        p.value_type_rules.iter().copied().chain(p.type_outer_ref_rules.iter().copied())
      }))
      .collect();
    let mut call_site_rules: Vec<IRulexSR<'s>> =
      all_rules.iter().copied().filter(|r| include_rule_in_call_site_solve(r)).collect();

    let call_range_t: &'t [RangeS<'s>] = self.typing_interner.alloc_slice_copy(call_range);
    // VTBRX: thread coutputs/calling_env/call_range_t/call_location/context_region into this call (seam signature change, Edit 2).
    let initial_sends = self.assemble_initial_sends_from_args(
      call_range[0],
      function,
      &args.iter().map(|a| Some(*a)).collect::<Vec<_>>(),
      coutputs,
      original_calling_env,
      call_range_t,
      call_location,
      context_region,
    );
    let mut initial_knowns = self.assemble_known_templatas(function, already_specified_template_args);

    let mut rune_to_type: IndexMap<IRuneS<'s>, ITemplataType<'s>> = self.derive_rune_to_type(
      coutputs,
      original_calling_env,
      call_range.to_vec(),
      function.generic_params,
      &all_rules,
      IndexMap::default(),
    );

    // Feed each argument type into its param's value_type_rune: the send becomes an Equals rule plus
    // an InitialKnown, and its sender rune gets a Kind type. Without this the arg types never reach
    // the solve, and any param-bearing call comes back SolveIncomplete.
    for s in initial_sends {
      initial_knowns.push(InitialKnown { rune: s.sender_rune, templata: s.send_templata });
      call_site_rules.push(IRulexSR::Equals(EqualsSR {
        range: s.sender_rune.range,
        left: s.sender_rune,
        right: s.receiver_rune,
      }));
      rune_to_type.insert(s.sender_rune.rune, ITemplataType::KindTemplataType(KindTemplataType {}));
    }

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
        // Empty because this resolves a call (see DBDAR) rather than defining, so the callee's bounds
        // must be proven rather than conjured.
        &[],
        &rune_to_type,
        call_range_t,
        call_location,
        &initial_knowns,
        &[],
      ) {
        Err(e) => {
          return Ok(IEvaluateFunctionResult::EvaluateFunctionFailure(EvaluateFunctionFailure {
            reason: e,
          }))
        }
        Ok(inferred_templatas) => inferred_templatas,
      };

    // See FunctionCompiler doc for what outer/runes/inner envs are.
    let reachable_bounds: Vec<PrototypeTemplataT<'s, 't>> = instantiation_bound_params
      .rune_to_citizen_rune_to_reachable_prototype
      .values()
      .flat_map(|r| {
        panic!("implement: evaluate_templated_function_from_call_for_banner reachable bounds");
        #[allow(unreachable_code)]
        empty::<PrototypeTemplataT<'s, 't>>()
      })
      .collect();

    let runed_env: &'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> =
      self.typing_interner.alloc(self.add_runed_data_to_near_env(
        declaring_env,
        &function.generic_params.iter().map(|gp| gp.rune.rune).collect::<Vec<_>>(),
        &inferences,
        &reachable_bounds,
      ));

    let prototype_templata = self.get_or_evaluate_templated_function_for_banner(
      declaring_env,
      runed_env,
      coutputs,
      call_range_t,
      call_location,
      function,
      instantiation_bound_params,
    )?;

    // Lambdas cant have bounds, right?
    assert!(instantiation_bound_params.rune_to_bound_prototype.is_empty(), "vcurious");
    assert!(
      instantiation_bound_params.rune_to_citizen_rune_to_reachable_prototype.is_empty(),
      "vcurious"
    );
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
      *original_calling_env.denizen_template_id(),
      prototype_templata.prototype.id,
      instantiation_bound_args,
    );
    Ok(IEvaluateFunctionResult::EvaluateFunctionSuccess(EvaluateFunctionSuccess {
      prototype: self.typing_interner.alloc(prototype_templata),
      inferences,
      instantiation_bound_args,
    }))
  }

  pub fn evaluate_templated_light_banner_from_call(
    &self,
    near_env: &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    original_calling_env: IInDenizenEnvironmentT<'s, 't>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    explicit_template_args: &[ITemplataT<'s, 't>],
    context_region: RegionT,
    args: &[KindT<'s, 't>],
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
    //
    // VCOORD: A user param's type-binding rules live per-param rather than in header_rules, so both the
    // solve and rune-typing must fold them in or the param runes are never bound (@PFVSZ). The
    // sites at :407, :556 and :720 do; this one does not, so it is wrong for any function with a
    // source-written parameter. Fold `params.flat_map(value_type_rules ++ type_outer_ref_rules)`
    // in the way they do.
    let all_rules: Vec<IRulexSR<'s>> = function
      .header_rules
      .iter()
      .copied()
      .chain(function.params.iter().flat_map(|p| {
        p.value_type_rules.iter().copied().chain(p.type_outer_ref_rules.iter().copied())
      }))
      .collect();
    let mut call_site_rules: Vec<IRulexSR<'s>> =
      all_rules.iter().copied().filter(|r| include_rule_in_call_site_solve(r)).collect();

    let call_range_t: &'t [RangeS<'s>] = self.typing_interner.alloc_slice_copy(call_range);
    // VTBRX: thread coutputs/calling_env/call_range_t/call_location/context_region into this call (seam signature change, Edit 2).
    let initial_sends = self.assemble_initial_sends_from_args(
      call_range[0],
      function,
      &args.iter().map(|a| Some(*a)).collect::<Vec<_>>(),
      coutputs,
      original_calling_env,
      call_range_t,
      call_location,
      context_region,
    );
    let mut initial_knowns = self.assemble_known_templatas(function, explicit_template_args);

    let mut rune_to_type: IndexMap<IRuneS<'s>, ITemplataType<'s>> = self.derive_rune_to_type(
      coutputs,
      original_calling_env,
      call_range.to_vec(),
      function.generic_params,
      &all_rules,
      IndexMap::default(),
    );

    // Feed each argument type into its param's value_type_rune: the send becomes an Equals rule plus
    // an InitialKnown, and its sender rune gets a Kind type. Without this the arg types never reach
    // the solve, and any param-bearing call comes back SolveIncomplete.
    for s in initial_sends {
      initial_knowns.push(InitialKnown { rune: s.sender_rune, templata: s.send_templata });
      call_site_rules.push(IRulexSR::Equals(EqualsSR {
        range: s.sender_rune.range,
        left: s.sender_rune,
        right: s.receiver_rune,
      }));
      rune_to_type.insert(s.sender_rune.rune, ITemplataType::KindTemplataType(KindTemplataType {}));
    }

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
        // Empty because this resolves a call (see DBDAR) rather than defining, so the callee's bounds
        // must be proven rather than conjured.
        &[],
        &rune_to_type,
        call_range_t,
        call_location,
        &initial_knowns,
        &[],
      ) {
        Err(e) => {
          return Ok(IEvaluateFunctionResult::EvaluateFunctionFailure(EvaluateFunctionFailure {
            reason: e,
          }))
        }
        Ok(inferred_templatas) => inferred_templatas,
      };

    // See FunctionCompiler doc for what outer/runes/inner envs are.
    let reachable_bounds: Vec<PrototypeTemplataT<'s, 't>> = instantiation_bound_params
      .rune_to_citizen_rune_to_reachable_prototype
      .values()
      .flat_map(|m| m.citizen_rune_to_reachable_prototype.values().copied())
      .map(|p| PrototypeTemplataT { prototype: self.typing_interner.alloc(p) })
      .collect();

    let runed_env: &'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> =
      self.typing_interner.alloc(self.add_runed_data_to_near_env(
        near_env,
        &function.generic_params.iter().map(|gp| gp.rune.rune).collect::<Vec<_>>(),
        &inferences,
        &reachable_bounds,
      ));

    let prototype_templata = self.get_or_evaluate_templated_function_for_banner(
      near_env,
      runed_env,
      coutputs,
      call_range_t,
      call_location,
      function,
      instantiation_bound_params,
    )?;

    // Lambdas cant have bounds, right?
    assert!(instantiation_bound_params.rune_to_bound_prototype.is_empty(), "vcurious");
    assert!(
      instantiation_bound_params.rune_to_citizen_rune_to_reachable_prototype.is_empty(),
      "vcurious"
    );
    assert!(instantiation_bound_params.rune_to_bound_impl.is_empty(), "vcurious");
    let instantiation_bound_args = self.typing_interner.alloc(InstantiationBoundArgumentsT {
      rune_to_bound_prototype: self.typing_interner.alloc_index_map_from_iter(
        instantiation_bound_params.rune_to_bound_prototype.iter().map(|(k, v)| (*k, *v)),
      ),
      rune_to_citizen_rune_to_reachable_prototype: self.typing_interner.alloc_index_map_from_iter(
        instantiation_bound_params
          .rune_to_citizen_rune_to_reachable_prototype
          .iter()
          .map(|(k, v)| (*k, *v)),
      ),
      rune_to_bound_impl: self.typing_interner.alloc_index_map_from_iter(
        instantiation_bound_params.rune_to_bound_impl.iter().map(|(k, v)| (*k, *v)),
      ),
    });
    coutputs.add_instantiation_bounds(
      self.opts.global_options.sanity_check,
      self.typing_interner,
      *original_calling_env.denizen_template_id(),
      prototype_templata.prototype.id,
      instantiation_bound_args,
    );
    Ok(IEvaluateFunctionResult::EvaluateFunctionSuccess(EvaluateFunctionSuccess {
      prototype: self.typing_interner.alloc(prototype_templata),
      inferences,
      instantiation_bound_args,
    }))
  }

  pub fn assemble_known_templatas(
    &self,
    function: &FunctionS<'s>,
    explicit_template_args: &[ITemplataT<'s, 't>],
  ) -> Vec<InitialKnown<'s, 't>> {
    function
      .generic_params
      .iter()
      .zip(explicit_template_args.iter())
      .map(|(generic_param, explicit_arg)| InitialKnown {
        rune: generic_param.rune,
        templata: *explicit_arg,
      })
      .collect()
  }

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

  // IOW, add the necessary data to turn the near env into the runed env.
  // The reachable_bounds_from_params_and_return harvest violates @BDPFWDZ — the bound prototypes
  // are pushed downward from each citizen-typed param's inner env into this near-env.
  pub fn add_runed_data_to_near_env(
    &self,
    near_env: &BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
    identifying_runes: &[IRuneS<'s>],
    templatas_by_rune: &IndexMap<IRuneS<'s>, ITemplataT<'s, 't>>,
    reachable_bounds_from_params_and_return: &[PrototypeTemplataT<'s, 't>],
  ) -> BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> {
    let identifying_templatas: Vec<ITemplataT<'s, 't>> =
      identifying_runes.iter().map(|r| *templatas_by_rune.get(r).unwrap()).collect();

    // reachableBoundsFromParamsAndReturn.zipWithIndex.toVector
    //   .map({ case (t, i) => (interner.intern(ReachablePrototypeNameT(i)), TemplataEnvEntry(t)) }) ++
    // templatasByRune.toVector
    //   .map({ case (k, v) => (interner.intern(RuneNameT(k)), TemplataEnvEntry(v)) })
    let entries_list: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> =
      reachable_bounds_from_params_and_return
        .iter()
        .enumerate()
        .map(|(i, t)| -> (INameT<'s, 't>, IEnvEntryT<'s, 't>) {
          let name = self
            .typing_interner
            .intern_reachable_prototype_name(ReachablePrototypeNameT { num: i as i32 });
          (
            INameT::ReachablePrototype(name),
            IEnvEntryT::Templata(ITemplataT::Prototype(self.typing_interner.alloc(*t))),
          )
        })
        .chain(templatas_by_rune.iter().map(|(k, v)| {
          let rune_name = self.typing_interner.intern_rune_name(RuneNameT { rune: *k });
          (INameT::Rune(rune_name), IEnvEntryT::Templata(*v))
        }))
        .collect();

    // newEntries = templatas.addEntries(interner, entries_list)
    let new_entries = self.typing_interner.alloc(near_env.templatas.add_entries(
      self.typing_interner,
      self.scout_arena,
      entries_list,
    ));

    let default_region = RegionT::Default;

    let template_args: &'t [ITemplataT<'s, 't>] =
      self.typing_interner.alloc_slice_from_vec(identifying_templatas);
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

  pub fn evaluate_generic_function_from_call_for_prototype(
    &self,
    outer_env: &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    calling_env: IInDenizenEnvironmentT<'s, 't>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    explicit_template_args: &[ITemplataT<'s, 't>],
    context_region: RegionT,
    args: &[Option<KindT<'s, 't>>],
    container_rune_initial_knowns: &[InitialKnown<'s, 't>],
  ) -> Result<IResolveFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
    let function = outer_env.function;
    self.check_closure_concerns_handled(outer_env);

    // A user param's type-binding rules live per-param (value_type_rules + type_outer_ref_rules),
    // not in function.rules, so both the call-site solve and rune-typing must fold them in or the
    // param runes are never bound (@PFVSZ produced-but-not-consumed). This is the call-site twin
    // of the defining-path wiring at :671. Return-type rules already ride in function.rules. Both
    // call_site_rules and derive_rune_to_type (below) use all_rules. // VCOORD: rewrite comment
    let all_rules: Vec<IRulexSR<'s>> = function
      .header_rules
      .iter()
      .copied()
      .chain(function.params.iter().flat_map(|p| {
        p.value_type_rules.iter().copied().chain(p.type_outer_ref_rules.iter().copied())
      }))
      .collect();
    let mut call_site_rules: Vec<IRulexSR<'s>> =
      all_rules.iter().copied().filter(|r| include_rule_in_call_site_solve(r)).collect();

    let call_range_t = self.typing_interner.alloc_slice_copy(call_range);
    let initial_sends = self.assemble_initial_sends_from_args(
      call_range[0],
      function,
      args,
      coutputs,
      calling_env,
      call_range_t,
      call_location,
      context_region,
    );

    let envs = InferEnv {
      original_calling_env: calling_env,
      parent_ranges: call_range_t,
      call_location,
      self_env: IEnvironmentT::BuildingWithClosureds(outer_env),
      context_region,
    };
    let mut rune_to_type: IndexMap<IRuneS<'s>, ITemplataType<'s>> = self.derive_rune_to_type(
      coutputs,
      IInDenizenEnvironmentT::BuildingWithClosureds(outer_env),
      call_range.to_vec(),
      function.generic_params,
      &all_rules,
      IndexMap::default(),
    );
    let invocation_range = call_range;
    let mut initial_knowns: Vec<InitialKnown<'s, 't>> = {
      let mut v = self.assemble_known_templatas(function, explicit_template_args);
      v.extend(container_rune_initial_knowns.iter().copied());
      v
    };
    // Per @BCHATZ, fill group generic parameters with `GroupTemplataT{}`.
    for generic_param in function.generic_params {
      if let Some(ITemplataType::GroupTemplataType(_)) = rune_to_type.get(&generic_param.rune.rune) {
        initial_knowns
          .push(InitialKnown { rune: generic_param.rune, templata: ITemplataT::Group(GroupTemplataT {}) });
      }
    }
    for s in initial_sends {
      initial_knowns.push(InitialKnown { rune: s.sender_rune, templata: s.send_templata });
      call_site_rules.push(IRulexSR::Equals(EqualsSR {
        range: s.sender_rune.range,
        left: s.sender_rune,
        right: s.receiver_rune,
      }));
      rune_to_type.insert(s.sender_rune.rune, ITemplataType::KindTemplataType(KindTemplataType {}));
    }
    let include_reachable_bounds_for_runes: Vec<IRuneS<'s>> = function
      .params
      .iter()
      .map(|p| p.value_type_rune.rune)
      .chain(function.maybe_ret_kind_rune.map(|ru| ru.rune))
      .collect();

    let mut solver = self.make_solver_state(
      envs,
      coutputs,
      &call_site_rules,
      &rune_to_type,
      invocation_range,
      &initial_knowns,
    );

    let mut loop_check = function.generic_params.len() as i32 + 1;

    // Per @DRSINI, defaults are added here incrementally as a fallback, only for runes
    // that remain unsolved after argument inference.
    match self.incrementally_solve(envs, coutputs, &mut solver, |_coutputs, solver_state| {
      if loop_check == 0 {
        panic!("RangedInternalErrorT: Infinite loop detected in incremental call solve!");
      }
      loop_check -= 1;

      match self.get_first_unsolved_identifying_rune(function.generic_params, |rune| {
        solver_state.get_conclusion(&rune).is_some()
      }) {
        None => false,
        Some((generic_param, index)) => {
          assert!(index >= explicit_template_args.len() as i32);

          match &generic_param.default {
            Some(default_rules) => {
              match solver_state.commit_step::<ITypingPassSolverError>(
                false,
                vec![],
                IndexMap::default(),
                default_rules.rules.iter().map(|r| **r).collect(),
                std::iter::once(default_rules.result_rune).collect(),
              ) {
                Ok(()) => {}
                Err(_) => panic!("getOrDie"),
              };
              true
            }
            None => false,
          }
        }
      }
    }) {
      Err(f) => {
        return Ok(IResolveFunctionResult::ResolveFunctionFailure(ResolveFunctionFailure {
          reason: IResolvingError::ResolvingSolveFailedOrIncomplete(f),
        }));
      }
      Ok(true) => {}
      Ok(false) => {} // Incomplete, will be detected as SolveIncomplete below.
    }

    let CompleteResolveSolve {
      conclusions: inferred_templatas,
      rune_to_bound: rune_to_function_bound,
    } = match self.check_resolving_conclusions_and_resolve(
      envs,
      coutputs,
      invocation_range,
      call_location,
      &rune_to_type,
      &call_site_rules,
      function.impl_bounds,
      &include_reachable_bounds_for_runes,
      &mut solver,
    )? {
      Err(e) => {
        return Ok(IResolveFunctionResult::ResolveFunctionFailure(ResolveFunctionFailure {
          reason: e,
        }));
      }
      Ok(i) => i,
    };

    let identifying_runes: Vec<IRuneS<'s>> =
      function.generic_params.iter().map(|gp| gp.rune.rune).collect();
    let reachable_bound_protos: Vec<PrototypeTemplataT<'s, 't>> = rune_to_function_bound
      .rune_to_citizen_rune_to_reachable_prototype
      .iter()
      .flat_map(|(_rune, x)| x.citizen_rune_to_reachable_prototype.values().copied())
      .map(|proto| PrototypeTemplataT { prototype: self.typing_interner.alloc(proto) })
      .collect();
    let runed_env = self.typing_interner.alloc(self.add_runed_data_to_near_env(
      outer_env,
      &identifying_runes,
      &inferred_templatas,
      &reachable_bound_protos,
    ));

    let prototype =
      self.get_generic_function_prototype_from_call(runed_env, coutputs, call_range, function)?;

    let prototype_templata = self
      .typing_interner
      .alloc(PrototypeTemplataT { prototype: self.typing_interner.alloc(prototype) });

    coutputs.add_instantiation_bounds(
      self.opts.global_options.sanity_check,
      self.typing_interner,
      *calling_env.root_compiling_denizen_env().denizen_template_id(),
      prototype.id,
      self.typing_interner.alloc(rune_to_function_bound),
    );

    Ok(IResolveFunctionResult::ResolveFunctionSuccess(ResolveFunctionSuccess {
      prototype: prototype_templata,
      inferences: inferred_templatas,
    }))
  }

  pub fn evaluate_generic_virtual_dispatcher_function_for_prototype_solving(
    &self,
    near_env: &'t BuildingFunctionEnvironmentWithClosuredsT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    calling_env: IInDenizenEnvironmentT<'s, 't>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    args: &[Option<KindT<'s, 't>>],
  ) -> Result<IDefineFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
    let function = near_env.function;
    self.check_closure_concerns_handled(near_env);

    let all_rules: Vec<IRulexSR<'s>> = function
      .header_rules
      .iter()
      .copied()
      .chain(function.params.iter().flat_map(|p| {
        p.value_type_rules.iter().copied().chain(p.type_outer_ref_rules.iter().copied())
      }))
      .collect();
    let function_definition_rules: Vec<IRulexSR<'s>> =
      all_rules.iter().copied().filter(|r| include_rule_in_definition_solve(r)).collect();
    let function_rune_to_type: IndexMap<IRuneS<'s>, ITemplataType<'s>> = self.derive_rune_to_type(
      coutputs,
      calling_env,
      call_range.to_vec(),
      function.generic_params,
      &all_rules,
      IndexMap::default(),
    );

    // VTBRX: thread coutputs/calling_env/call_range_t/call_location/context_region into this call (defining-path twin, Edit 2).
    // Defining path keeps old_ for now: it has no context_region, and §2A upcast does not apply to
    // its placeholder args (it defines the function rather than resolving a concrete call).
    let initial_sends = self.old_assemble_initial_sends_from_args(call_range[0], function, args);

    let preliminary_envs = InferEnv {
      original_calling_env: calling_env,
      parent_ranges: self.typing_interner.alloc_slice_copy(call_range),
      call_location,
      self_env: IEnvironmentT::BuildingWithClosureds(near_env),
      context_region: RegionT::Default,
    };
    let mut preliminary_solve_initial_knowns = Vec::new();
    let mut preliminary_rules = function_definition_rules.clone();
    let mut preliminary_rune_to_type = function_rune_to_type.clone();
    for initial_send in initial_sends {
      preliminary_solve_initial_knowns.push(InitialKnown {
        rune: initial_send.sender_rune,
        templata: initial_send.send_templata,
      });
      // VCOORD: see if we can simplify away this rule? can we not just feed it directly?
      preliminary_rules.push(IRulexSR::Equals(EqualsSR {
        range: initial_send.sender_rune.range,
        left: initial_send.sender_rune,
        right: initial_send.receiver_rune,
      }));
      preliminary_rune_to_type.insert(
        initial_send.sender_rune.rune,
        ITemplataType::KindTemplataType(KindTemplataType {}),
      );
    }
    let mut preliminary_solver_state = self.make_solver_state(
      preliminary_envs,
      coutputs,
      &preliminary_rules,
      &preliminary_rune_to_type,
      &{
        let mut ranges = vec![function.range];
        ranges.extend_from_slice(call_range);
        ranges
      },
      &preliminary_solve_initial_knowns,
    );
    match self.r#continue(preliminary_envs, coutputs, &mut preliminary_solver_state) {
      Ok(()) => {}
      Err(_f) => {
        panic!("implement: TypingPassSolverError from preliminary continue");
      }
    }

    let preliminary_inferences: IndexMap<IRuneS<'s>, ITemplataT<'s, 't>> =
      preliminary_solver_state.userify_conclusions().into_iter().collect();

    let placeholder_initial_knowns_from_function: Vec<InitialKnown<'s, 't>> = function
      .generic_params
      .iter()
      .enumerate()
      .flat_map(|(index, generic_param)| {
        match preliminary_inferences.get(&generic_param.rune.rune) {
          Some(&x) => Some(InitialKnown { rune: generic_param.rune, templata: x }),
          // Per @BCHATZ, fill group generic parameters with `GroupTemplataT{}`.
          None => match function_rune_to_type.get(&generic_param.rune.rune) {
            Some(ITemplataType::GroupTemplataType(_)) => Some(InitialKnown {
              rune: generic_param.rune,
              templata: ITemplataT::Group(GroupTemplataT {}),
            }),
            _ => panic!("implement: create placeholder for missing preliminary inference"),
          },
        }
      })
      .collect();

    let CompleteDefineSolve { conclusions: inferences, rune_to_bound: instantiation_bound_params } =
      match self.solve_for_defining(
        InferEnv {
          original_calling_env: calling_env,
          parent_ranges: self.typing_interner.alloc_slice_copy(call_range),
          call_location,
          self_env: IEnvironmentT::BuildingWithClosureds(near_env),
          context_region: RegionT::Default,
        },
        coutputs,
        &function_definition_rules,
        function.impl_bounds,
        &function_rune_to_type,
        &{
          let mut ranges = vec![function.range];
          ranges.extend_from_slice(call_range);
          ranges
        },
        call_location,
        &placeholder_initial_knowns_from_function,
        &{
          let mut runes: Vec<IRuneS<'s>> =
            function.params.iter().map(|p| p.full_type_rune.rune).collect();
          if let Some(r) = function.maybe_ret_kind_rune {
            runes.push(r.rune);
          }
          runes
        },
      ) {
        Err(f) => {
          return Err(ICompileErrorT::TypingPassDefiningError {
            range: self.typing_interner.alloc_slice_copy(&{
              let mut ranges = vec![function.range];
              ranges.extend_from_slice(call_range);
              ranges
            }),
            inner: f,
          });
        }
        Ok(c) => c,
      };
    let reachable_bounds: Vec<PrototypeTemplataT<'s, 't>> = instantiation_bound_params
      .rune_to_citizen_rune_to_reachable_prototype
      .values()
      .flat_map(|m| m.citizen_rune_to_reachable_prototype.values().copied())
      .map(|p| PrototypeTemplataT { prototype: self.typing_interner.alloc(p) })
      .collect();
    let runed_env = self.add_runed_data_to_near_env(
      near_env,
      &function.generic_params.iter().map(|p| p.rune.rune).collect::<Vec<_>>(),
      &inferences,
      &reachable_bounds,
    );

    let runed_env_ref = self.typing_interner.alloc(runed_env);
    let prototype = self.get_generic_function_prototype_from_call(
      runed_env_ref,
      coutputs,
      call_range,
      function,
    )?;

    Ok(IDefineFunctionResult::DefineFunctionSuccess(DefineFunctionSuccess {
      prototype: self
        .typing_interner
        .alloc(PrototypeTemplataT { prototype: self.typing_interner.alloc(prototype) }),
      inferences,
      instantiation_bound_params: instantiation_bound_params,
    }))
  }

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
      IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(r) => {
        INameT::AnonymousSubstructConstructorTemplate(r)
      }
      IFunctionTemplateNameT::LambdaCallFunctionTemplate(r) => {
        INameT::LambdaCallFunctionTemplate(r)
      }
      IFunctionTemplateNameT::OverrideDispatcherTemplate(r) => {
        INameT::OverrideDispatcherTemplate(r)
      }
      IFunctionTemplateNameT::ExternFunction(r) => INameT::ExternFunction(r),
      IFunctionTemplateNameT::FunctionBoundTemplate(r) => INameT::FunctionBoundTemplate(r),
      IFunctionTemplateNameT::PredictedFunctionTemplate(r) => INameT::PredictedFunctionTemplate(r),
    };
    let function_template_id =
      near_env.parent_env.id().add_step(self.typing_interner, function_name_local);

    // A user param's type-binding rules live per-param (value_type_rules + type_outer_ref_rules),
    // not in function.rules, so the solve must fold them in or the param runes are never bound
    // (@PFVSZ produced-but-not-consumed). Return-type rules already ride in function.rules. Both
    // the value solve (definition_rules) and rune-typing (derive_rune_to_type below) use all_rules. VCOORD: rewrite this comment
    let all_rules: Vec<IRulexSR<'s>> = function
      .header_rules
      .iter()
      .copied()
      .chain(function.params.iter().flat_map(|p| {
        p.value_type_rules.iter().copied().chain(p.type_outer_ref_rules.iter().copied())
      }))
      .collect();
    let definition_rules: Vec<IRulexSR<'s>> =
      all_rules.iter().copied().filter(|r| include_rule_in_definition_solve(r)).collect();

    let mut seen = HashSet::default();
    let mut param_and_return_runes: Vec<IRuneS<'s>> = Vec::new();
    for param in function.params.iter() {
      let coord_rune = param.value_type_rune;
      if seen.insert(coord_rune.rune) {
        param_and_return_runes.push(coord_rune.rune);
      }
    }
    if let Some(ret_coord_rune) = function.maybe_ret_kind_rune {
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
      context_region: RegionT::Default,
    };

    let rune_to_type: IndexMap<IRuneS<'s>, ITemplataType<'s>> = self.derive_rune_to_type(
      coutputs,
      near_env_as_in_denizen,
      range.clone(),
      function.generic_params,
      &all_rules,
      IndexMap::default(),
    );
    let mut solver =
      self.make_solver_state(envs, coutputs, &definition_rules, &rune_to_type, &range, &[]);

    let get_first_unsolved = |generic_parameters: &'s [&'s GenericParameterS<'s>],
                              is_solved: &dyn Fn(IRuneS<'s>) -> bool| {
      self.get_first_unsolved_identifying_rune(generic_parameters, |rune| is_solved(rune))
    };
    let result = self.incrementally_solve(envs, coutputs, &mut solver, |coutputs, solver_state| {
      match get_first_unsolved(function.generic_params, &|rune| {
        solver_state.get_conclusion(&rune).is_some()
      }) {
        None => false,
        Some((generic_param, index)) => {
          let placeholder_pure_height = None;
          let templata = self.create_placeholder(
            coutputs,
            near_env_as_in_denizen,
            *function_template_id,
            generic_param,
            index,
            &rune_to_type,
            placeholder_pure_height,
            true,
          );
          solver_state
            .commit_step::<()>(
              false,
              vec![],
              {
                let mut m = IndexMap::default();
                m.insert(generic_param.rune.rune, templata);
                m
              },
              vec![],
              IndexSet::default(),
            )
            .unwrap();
          true
        }
      }
    });
    match result {
      Err(f) => {
        return Err(ICompileErrorT::TypingPassSolverError {
          range: self.typing_interner.alloc_slice_from_vec(range.clone()),
          failed_solve: f,
        })
      }
      Ok(true) => {}
      Ok(false) => {} // Incomplete, will be detected in checkDefiningConclusionsAndResolve
    }

    let mut inferences = match self.interpret_results(&rune_to_type, &mut solver) {
      Err(e) => {
        return Err(ICompileErrorT::TypingPassSolverError {
          range: self.typing_interner.alloc_slice_from_vec(range.clone()),
          failed_solve: e,
        })
      }
      Ok(conclusions) => conclusions,
    };

    self.conjure_impl_bounds_for_defining(envs, function.impl_bounds, &mut inferences);

    let instantiation_bound_params = match self.check_defining_conclusions_and_resolve(
      envs,
      coutputs,
      &range,
      call_location,
      &definition_rules,
      &param_and_return_runes,
      &inferences,
    ) {
      Err(f) => match f {
        IConclusionResolveError::CouldntFindFunctionForConclusionResolve { .. } => {
          panic!("TypingPassDefiningError: CouldntFindFunctionForConclusionResolve")
        }
        IConclusionResolveError::ReturnTypeConflictInConclusionResolve { .. } => {
          panic!("TypingPassDefiningError: ReturnTypeConflictInConclusionResolve")
        }
        IConclusionResolveError::CouldntFindImplForConclusionResolve { .. } => {
          panic!("TypingPassDefiningError: CouldntFindImplForConclusionResolve")
        }
        IConclusionResolveError::CouldntFindKindForConclusionResolve(_) => {
          panic!("TypingPassDefiningError: CouldntFindKindForConclusionResolve")
        }
      },
      Ok(c) => c,
    };

    let identifying_runes: Vec<IRuneS<'s>> =
      function.generic_params.iter().map(|gp| gp.rune.rune).collect();
    let reachable_bounds: Vec<PrototypeTemplataT<'s, 't>> = instantiation_bound_params
      .rune_to_citizen_rune_to_reachable_prototype
      .iter()
      .flat_map(|(_, rb)| rb.citizen_rune_to_reachable_prototype.iter().map(|(_, proto)| proto))
      .map(|proto| PrototypeTemplataT { prototype: self.typing_interner.alloc(*proto) })
      .collect();
    let runed_env =
      self.add_runed_data_to_near_env(near_env, &identifying_runes, &inferences, &reachable_bounds);
    let runed_env: &'t BuildingFunctionEnvironmentWithClosuredsAndTemplateArgsT<'s, 't> =
      self.typing_interner.alloc(runed_env);

    let header = self.get_or_evaluate_function_for_header(
      near_env,
      runed_env,
      coutputs,
      parent_ranges,
      call_location,
      function,
      instantiation_bound_params,
    )?;

    Ok(header)
  }

  /// Conjures, for each `where implements(Sub, Super)` the denizen declares, an `Isa` that
  /// satisfies it.
  /// Runs after the solve because nothing mid-solve reads an `Isa`. Per SFWPRL
  /// (docs/Generics.md:355) the solve postpones resolving structs and interfaces precisely so a
  /// fact like this can arrive late. Runs before the conclusions become an environment so that
  /// these can be included in that environment.
  pub fn conjure_impl_bounds_for_defining(
    &self,
    envs: InferEnv<'s, 't>,
    impl_bounds: &[ImplBoundS<'s>],
    conclusions: &mut IndexMap<IRuneS<'s>, ITemplataT<'s, 't>>,
  ) {
    for impl_bound in impl_bounds {
      let sub_kind = expect_kind_templata(
        *conclusions
          .get(&impl_bound.sub_rune.rune)
          .expect("vassertSome: implements() sub operand not in conclusions"),
      )
      .kind;
      let super_kind = expect_kind_templata(
        *conclusions
          .get(&impl_bound.super_rune.rune)
          .expect("vassertSome: implements() super operand not in conclusions"),
      )
      .kind;
      let template = self.typing_interner.intern_impl_bound_template_name(ImplBoundTemplateNameT {
        code_location: impl_bound.range.begin,
      });
      let bound_name = self
        .typing_interner
        .intern_impl_bound_name(ImplBoundNameValT { template, template_args: &[] });
      let impl_name = *envs
        .original_calling_env
        .denizen_id()
        .add_step(self.typing_interner, INameT::ImplBound(bound_name));
      let isa =
        IsaTemplataT { declaration_range: impl_bound.range, impl_name, sub_kind, super_kind };
      conclusions
        .insert(impl_bound.result_rune.rune, ITemplataT::Isa(self.typing_interner.alloc(isa)));
    }
  }

  // VCOORD: delete this
  pub fn old_assemble_initial_sends_from_args(
    &self,
    call_range: RangeS<'s>,
    function: &FunctionS<'s>,
    args: &[Option<KindT<'s, 't>>],
  ) -> Vec<InitialSend<'s, 't>> {
    function
      .params
      .iter()
      .map(|p| p.value_type_rune)
      .zip(args.iter())
      .enumerate()
      .flat_map(|(arg_index, (param_full_type_rune, arg_maybe))| {
        match arg_maybe {
          None => None,
          Some(unpeeled_arg) => {
            // We feed in the peeled arg to the `value_type_rune`, then later on after
            // the solve we sort out the cloning/auto-ref'ing.
            let peeled_arg = peel_all_references(*unpeeled_arg);
            let sender_rune = RuneUsage {
              range: call_range,
              rune: self.scout_arena.intern_rune(IRuneValS::ArgumentRune(ArgumentRuneS {
                arg_index: arg_index as i32,
              })),
            };
            Some(InitialSend {
              sender_rune,
              receiver_rune: param_full_type_rune,
              send_templata: ITemplataT::Kind(
                self.typing_interner.alloc(KindTemplataT { kind: peeled_arg }),
              ),
            })
          }
        }
      })
      .collect()
  }

  pub fn assemble_initial_sends_from_args(
    &self,
    call_range: RangeS<'s>,
    function: &FunctionS<'s>,
    args: &[Option<KindT<'s, 't>>],
    coutputs: &mut CompilerOutputs<'s, 't>,
    calling_env: IInDenizenEnvironmentT<'s, 't>, // VCOORD: why do we need calling_env? sus.
    call_range_t: &'t [RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    context_region: RegionT,
  ) -> Vec<InitialSend<'s, 't>> {
    let mut sends: Vec<InitialSend<'s, 't>> = Vec::new();
    for (arg_index, (param, arg_maybe)) in function.params.iter().zip(args.iter()).enumerate() {
      let Some(unpeeled_arg) = arg_maybe else {
        continue;
      };
      // Peel the arg to match the parameter's value slot before seeding its value_type_rune.
      // Ask whether the value slot is a rune (a generic like `T`, perhaps under the param's own
      // ref wraps) rather than a concrete type produced by a Call (e.g. `int` or `Opt<T>`). A
      // rune slot binds whatever the arg is, references and all, so we peel only the param's own
      // written wraps and keep the rest. That is how an explicitly-bound `T = &Spaceship` keeps
      // its `&`. A concrete slot instead reads out the arg's outer reference, a spurious mention
      // borrow.
      // VCOORD: This is here only temporarily because we don't want to change the entire solver
      // quite yet. We cant move plan-phased-calls.md's 4A entirely to here because 4A needs
      // to read things that were determined by the explicit template args.
      let value_slot_is_rune = !param.value_type_rules.iter().any(
        |r| matches!(r, IRulexSR::Call(c) if c.result_rune.rune == param.value_type_rune.rune),
      );
      let peeled_arg = if value_slot_is_rune {
        peel_n_references(*unpeeled_arg, param.type_outer_ref_rules.len())
          .unwrap_or_else(|| peel_all_references(*unpeeled_arg))
      } else {
        peel_all_references(*unpeeled_arg)
      };
      // Phase 2A: if the arg's template differs from the param's and the arg implements the
      // param's interface, seed the upcast interface kind so the send's Equals stops
      // conflicting. convert_via_upcast emits the actual upcast later.
      let send_kind = self
        .compute_upcast_coerced_arg(
          coutputs,
          calling_env,
          call_range_t,
          call_location,
          context_region,
          peeled_arg,
          param,
        )
        .unwrap_or(peeled_arg);
      let sender_rune = RuneUsage {
        range: call_range,
        rune: self
          .scout_arena
          .intern_rune(IRuneValS::ArgumentRune(ArgumentRuneS { arg_index: arg_index as i32 })),
      };
      sends.push(InitialSend {
        sender_rune,
        receiver_rune: param.value_type_rune,
        send_templata: ITemplataT::Kind(
          self.typing_interner.alloc(KindTemplataT { kind: send_kind }),
        ),
      });
    }
    sends
  }
}
