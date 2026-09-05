use crate::interner::Interner;
use crate::keywords::Keywords;
use crate::postparsing::ast::*;
use crate::postparsing::ast::*;
use crate::postparsing::names::*;
use crate::postparsing::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::ast::expressions::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::function_environment_t::{CapturedVariableT, IVariableT};
use crate::typing::env::i_env_entry::IEnvEntryT;
use crate::typing::env::i_env_entry::*;
use crate::typing::function::function_compiler::*;
use crate::typing::infer_compiler::InitialKnown;
use crate::typing::names::names::*;
use crate::typing::templata::templata::KindTemplataT;
use crate::typing::templata::templata::*;
use crate::typing::templata_compiler::IBoundArgumentsSource;
use crate::typing::types::types::*;
use crate::utils::range::RangeS;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn evaluate_templated_closure_function_from_call_for_banner(
    &self,
    parent_env: IEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    calling_env: IInDenizenEnvironmentT<'s, 't>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    closure_struct_ref: StructTT<'s, 't>,
    function: &'s FunctionS<'s>,
    already_specified_template_args: &[ITemplataT<'s, 't>],
    context_region: RegionT,
    arg_types: &[KindT<'s, 't>],
  ) -> Result<IEvaluateFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
    let (variables, entries) = self.make_closure_variables_and_entries(
      coutputs,
      *calling_env.denizen_template_id(),
      closure_struct_ref,
    );
    let name = self.typing_interner.alloc(parent_env.id().add_step(
      self.typing_interner,
      self.translate_generic_template_function_name(function.name, arg_types),
    ));
    // Register the lambda's __call FunctionS under its LambdaCallFunctionTemplate id — the id the
    // borrow checker's resolve_callee looks up — so a call to a lambda has a postparsed present.
    coutputs.register_postparsed_function(*name, function);
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
      outer_env,
      coutputs,
      calling_env,
      call_range,
      call_location,
      already_specified_template_args,
      context_region,
      arg_types,
    )
  }

  pub fn evaluate_templated_closure_function_from_call_for_prototype(
    &self,
    outer_env: IEnvironmentT,
    coutputs: CompilerOutputs,
    calling_env: IInDenizenEnvironmentT,
    call_range: Vec<RangeS>,
    call_location: LocationInDenizen,
    closure_struct_ref: StructTT,
    function: FunctionS,
    already_specified_template_args: Vec<ITemplataT>,
    context_region: RegionT,
    arg_types: Vec<KindT>,
  ) -> IEvaluateFunctionResult<'_, '_> {
    panic!("Unimplemented: evaluate_templated_closure_function_from_call_for_prototype");
    // val (variables, entries) = makeClosureVariablesAndEntries(coutputs, callingEnv.denizenTemplateId, closureStructRef)
    // val name = outerEnv.id.addStep(nameTranslator.translateGenericTemplateFunctionName(function.name, argTypes))
    // val newEnv = BuildingFunctionEnvironmentWithClosuredsT(outerEnv.globalEnv, outerEnv, name, TemplatasStore(name, Map(), Map()).addEntries(interner, entries), function, variables, false)
    // ordinaryOrTemplatedLayer.evaluateTemplatedFunctionFromCallForPrototype(newEnv, coutputs, callingEnv, callRange, callLocation, alreadySpecifiedTemplateArgs, contextRegion, argTypes)
  }

  pub fn evaluate_templated_light_function_from_call_for_prototype2(
    &self,
    parent_env: IEnvironmentT,
    coutputs: CompilerOutputs,
    calling_env: IInDenizenEnvironmentT,
    call_range: Vec<RangeS>,
    call_location: LocationInDenizen,
    function: FunctionS,
    explicit_template_args: Vec<ITemplataT>,
    context_region: RegionT,
    arg_types: Vec<KindT>,
  ) -> IEvaluateFunctionResult<'_, '_> {
    panic!("Unimplemented: evaluate_templated_light_function_from_call_for_prototype2");
    // checkNotClosure(function)
    // val outerEnvId = parentEnv.id.addStep(nameTranslator.translateGenericTemplateFunctionName(function.name, argTypes))
    // val outerEnv = makeEnvWithoutClosureStuff(parentEnv, function, outerEnvId, false)
    // ordinaryOrTemplatedLayer.evaluateTemplatedFunctionFromCallForPrototype(outerEnv, coutputs, callingEnv, callRange, callLocation, explicitTemplateArgs, contextRegion, argTypes)
  }

  pub fn evaluate_generic_light_function_from_call_for_prototype2(
    &self,
    parent_env: IEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    calling_env: IInDenizenEnvironmentT<'s, 't>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    function: &'s FunctionS<'s>,
    explicit_template_args: &[ITemplataT<'s, 't>],
    context_region: RegionT,
    args: &[Option<KindT<'s, 't>>],
    container_rune_initial_knowns: &[InitialKnown<'s, 't>],
  ) -> Result<IResolveFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
    self.check_not_closure(function);

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
    let outer_env_id = parent_env.id().add_step(self.typing_interner, function_name_local);
    let outer_env = self.make_env_without_closure_stuff(parent_env, function, outer_env_id, false);
    self.evaluate_generic_function_from_call_for_prototype(
      outer_env,
      coutputs,
      calling_env,
      call_range,
      call_location,
      explicit_template_args,
      context_region,
      args,
      container_rune_initial_knowns,
    )
  }

  pub fn evaluate_generic_virtual_dispatcher_function_for_prototype_closure_or_light(
    &self,
    parent_env: IEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    calling_env: IInDenizenEnvironmentT<'s, 't>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    function: &'s FunctionS<'s>,
    args: &[Option<KindT<'s, 't>>],
  ) -> Result<IDefineFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
    self.check_not_closure(function);
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
    let outer_env_id = parent_env.id().add_step(self.typing_interner, function_name_local);
    let outer_env = self.make_env_without_closure_stuff(parent_env, function, outer_env_id, true);
    self.evaluate_generic_virtual_dispatcher_function_for_prototype_solving(
      outer_env,
      coutputs,
      calling_env,
      call_range,
      call_location,
      args,
    )
  }

  pub fn evaluate_generic_light_function_from_non_call(
    &self,
    parent_env: IEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    parent_ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    function: &'s FunctionS<'s>,
  ) -> Result<&'t FunctionHeaderT<'s, 't>, ICompileErrorT<'s, 't>> {
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
    let outer_env_id = parent_env.id().add_step(self.typing_interner, function_name_local);
    let outer_env = self.make_env_without_closure_stuff(parent_env, function, outer_env_id, true);
    self.evaluate_generic_function_from_non_call_solving(
      coutputs,
      outer_env,
      parent_ranges,
      call_location,
    )
  }

  pub fn evaluate_templated_light_banner_from_call_closure_or_light(
    &self,
    parent_env: IEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    calling_env: IInDenizenEnvironmentT<'s, 't>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    function: &'s FunctionS<'s>,
    explicit_template_args: &[ITemplataT<'s, 't>],
    context_region: RegionT,
    arg_types: &[KindT<'s, 't>],
  ) -> Result<IEvaluateFunctionResult<'s, 't>, ICompileErrorT<'s, 't>> {
    self.check_not_closure(function);

    let outer_env_id = parent_env.id().add_step(
      self.typing_interner,
      self.translate_generic_template_function_name(function.name, arg_types),
    );
    // Register a light lambda's __call FunctionS under its LambdaCallFunctionTemplate id (the id the
    // borrow checker's resolve_callee looks up), mirroring the closure banner above.
    coutputs.register_postparsed_function(outer_env_id, function);
    let outer_env = self.make_env_without_closure_stuff(parent_env, function, outer_env_id, false);
    self.evaluate_templated_light_banner_from_call(
      outer_env,
      coutputs,
      calling_env,
      call_range,
      call_location,
      explicit_template_args,
      context_region,
      arg_types,
    )
  }

  pub fn evaluate_templated_function_from_call_for_banner_closure_or_light(
    &self,
    parent_env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    calling_env: IInDenizenEnvironmentT,
    function: FunctionS,
    call_range: Vec<RangeS>,
    call_location: LocationInDenizen,
    already_specified_template_args: Vec<ITemplataT>,
    context_region: RegionT,
    arg_types: Vec<KindT>,
  ) -> IEvaluateFunctionResult<'_, '_> {
    panic!("Unimplemented: evaluate_templated_function_from_call_for_banner");
    // val outerEnvId = parentEnv.id.addStep(nameTranslator.translateGenericFunctionName(function.name))
    // val outerEnv = makeEnvWithoutClosureStuff(parentEnv, function, outerEnvId, false)
    // ordinaryOrTemplatedLayer.evaluateTemplatedFunctionFromCallForBanner(
    //   outerEnv, coutputs, callingEnv, callRange, callLocation, alreadySpecifiedTemplateArgs, contextRegion, argTypes)
  }

  fn make_env_without_closure_stuff(
    &self,
    outer_env: IEnvironmentT<'s, 't>,
    function: &'s FunctionS<'s>,
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

  fn check_not_closure(&self, function: &'s FunctionS<'s>) {
    match &function.body {
      IBodyS::CodeBody(body1) => assert!(body1.body.closured_names.is_empty()),
      IBodyS::ExternBody(_) => {}
      IBodyS::GeneratedBody(_) => {}
      IBodyS::AbstractBody(_) => {}
    }
  }

  fn make_closure_variables_and_entries(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    original_calling_denizen_id: IdT<'s, 't>,
    closure_struct_ref: StructTT<'s, 't>,
  ) -> (Vec<IVariableT<'s, 't>>, Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)>) {
    let closure_struct_def = coutputs.lookup_struct(*closure_struct_ref.id, self);
    let substituter = self.get_placeholder_substituter(
      self.opts.global_options.sanity_check,
      &original_calling_denizen_id,
      closure_struct_ref.id,
      // This is a parameter, so we can grab bounds from it.
      IBoundArgumentsSource::InheritBoundsFromTypeItself,
    );
    let variables: Vec<IVariableT<'s, 't>> = closure_struct_def
      .members
      .iter()
      .map(|member| {
        IVariableT::Capture(self.typing_interner.alloc(CapturedVariableT {
          name: IVarNameT::Local(self.typing_interner.intern_local_name(LocalNameT {
            imprecise_name: member.name.imprecise_name,
            loct: member.name.loct,
          })),
          closured_vars_struct_type: self.typing_interner.alloc(closure_struct_ref),
          kind: substituter.substitute_for_kind(coutputs, member.tyype),
        }))
      })
      .collect();
    let entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> = vec![(
      closure_struct_ref.id.local_name,
      IEnvEntryT::Templata(ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT {
        kind: KindT::Struct(self.typing_interner.alloc(closure_struct_ref)),
      }))),
    )];
    (variables, entries)
  }
}
