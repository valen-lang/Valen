use crate::interner::StrI;
use crate::utils::range::RangeS;

use crate::postparsing::ast::*;

use crate::postparsing::ast::LocationInDenizen;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::*;
use crate::typing::env::environment::get_imprecise_name;
use crate::typing::env::environment::IInDenizenEnvironmentT;
use crate::typing::env::function_environment_t::*;
use crate::typing::function::function_compiler::StampFunctionSuccess;
use crate::typing::infer_compiler::IResolvingError;
use crate::typing::overload_resolver::IFindFunctionFailureReason;
use crate::typing::templata::templata::FunctionTemplataT;
use crate::typing::types::types::RegionT;
use crate::typing::types::types::*;
use crate::utils::fx::IndexMap;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn generate_function_body_abstract_body(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    env: &'t FunctionEnvironmentT<'s, 't>,
    generator_id: StrI<'s>,
    loct: LocT<'t>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    origin_function: Option<&'s FunctionS<'s>>,
    params2: &[ParameterT<'s, 't>],
    maybe_ret_coord: Option<KindT<'s, 't>>,
  ) -> Result<(FunctionHeaderT<'s, 't>, ExpressionTE<'s, 't>), ICompileErrorT<'s, 't>> {
    let return_reference_type2 = maybe_ret_coord.expect("vassertSome: maybeRetCoord");
    assert!(params2.iter().any(|p| p.virtuality == Some(AbstractT)));
    let header = FunctionHeaderT {
      id: env.id,
      attributes: self.typing_interner.alloc_slice_from_vec(vec![]),
      params: self.typing_interner.alloc_slice_from_vec(params2.to_vec()),
      return_type: return_reference_type2,
      maybe_origin_function_templata: origin_function.map(|_f| FunctionTemplataT {
        outer_env: env.parent_env,
        function_template_id: &env.template_id,
      }),
    };

    // Find self, but instead of calling it like a regular function call, call it like an interface.
    // We do this instead of grabbing the prototype out of the environment because we want to get its
    // instantiation bounds too (well, we want them to be added to the coutputs).
    // Per @DRSINI, this triggers overload resolution with 0 explicit template args and
    // placeholder-typed self arg. Defaults must not be in the initial rules or they'd
    // conflict with arg-inferred placeholders.
    let imprecise_name = get_imprecise_name(self.scout_arena, env.id.local_name)
      .expect("vassertSome: TemplatasStore.getImpreciseName env.id.localName");
    let param_types: Vec<KindT<'s, 't>> = params2.iter().map(|p| p.tyype).collect();
    let env_as_iindenizen = self.typing_interner.alloc(IInDenizenEnvironmentT::Function(env));
    let calling_env = *env_as_iindenizen;
    let explicit_template_arg_rules_s = &[];
    let positional_explicit_template_arg_runes_s = &[];
    let receiving_rune_to_explicit_template_arg_rune = &[];
    let context_region = RegionT::Default;
    let extra_envs_to_look_in = &[];
    let potential_banner = self.find_function(
      calling_env,
      coutputs,
      call_range,
      call_location,
      imprecise_name,
      explicit_template_arg_rules_s,
      positional_explicit_template_arg_runes_s,
      receiving_rune_to_explicit_template_arg_rune,
      context_region,
      &param_types,
      extra_envs_to_look_in,
      true,
      false,
    )?;
    // VCOORD: simplify
    let prototype = match (match potential_banner {
      Err(e) => Ok(Err(e)),
      Ok(potential_banner) => Ok(Ok(StampFunctionSuccess {
        prototype: potential_banner.prototype,
        inferences: IndexMap::default(),
      })),
    })? {
      Ok(stamp) => stamp.prototype,
      Err(fff) => {
        // Name the rejection kind per candidate rather than dumping the payload: an
        // InferFailure carries a whole solve tree, which buries the one fact that
        // distinguishes "no such override" from "the override exists and didn't solve".
        let reasons: Vec<String> = fff
          .rejected_callee_to_reason
          .iter()
          .map(|(_candidate, reason)| match reason {
            IFindFunctionFailureReason::WrongNumberOfArguments { supplied, expected } => {
              format!("WrongNumberOfArguments (supplied {}, expected {})", supplied, expected)
            }
            IFindFunctionFailureReason::WrongNumberOfTemplateArguments { supplied, expected } => {
              format!(
                "WrongNumberOfTemplateArguments (supplied {}, expected {})",
                supplied, expected
              )
            }
            IFindFunctionFailureReason::SpecificParamDoesntSend { index, .. } => {
              format!("SpecificParamDoesntSend (param {})", index)
            }
            IFindFunctionFailureReason::SpecificParamDoesntMatchExactly { index, .. } => {
              format!("SpecificParamDoesntMatchExactly (param {})", index)
            }
            IFindFunctionFailureReason::SpecificParamVirtualityDoesntMatch { index } => {
              format!("SpecificParamVirtualityDoesntMatch (param {})", index)
            }
            IFindFunctionFailureReason::Outscored => "Outscored".to_string(),
            IFindFunctionFailureReason::RuleTypeSolveFailure { .. } => {
              "RuleTypeSolveFailure".to_string()
            }
            IFindFunctionFailureReason::InferFailure { .. } => "InferFailure".to_string(),
            IFindFunctionFailureReason::FindFunctionResolveFailure { reason } => match reason {
              IResolvingError::ResolvingSolveFailedOrIncomplete(fs) => {
                format!("ResolveFailure (unsolved: {:?})", fs.unsolved_runes)
              }
              IResolvingError::ResolvingResolveConclusionError(_) => {
                "ResolveConclusionError".to_string()
              }
            },
            IFindFunctionFailureReason::CouldntEvaluateTemplateError { .. } => {
              "CouldntEvaluateTemplateError".to_string()
            }
          })
          .collect();
        // VCOORD: should we make this go through humanizing?
        panic!(
          "abstract body: no override found for {:?}, {} candidate(s) rejected: [{}]",
          fff.name,
          reasons.len(),
          reasons.join(", ")
        )
      }
    };

    let virtual_index =
      header.get_virtual_index().expect("vassertSome: header.getVirtualIndex") as i32;
    // This is a compiler-generated abstract-function body, so its nodes have no user source; the honest range is a synthesized internal one.
    let synth_range = RangeS::internal(self.scout_arena, -70100);
    let args: Vec<ExpressionTE<'s, 't>> = prototype
      .param_types()
      .iter()
      .enumerate()
      .map(|(index, param_type)| {
        ExpressionTE::ArgLookup(
          self.typing_interner.alloc(ArgLookupTE::new(synth_range, index as i32, *param_type)),
        )
      })
      .collect();
    let args_slice = self.typing_interner.alloc_slice_from_vec(args);
    let ifc = InterfaceFunctionCallTE::new(
      synth_range,
      self.typing_interner.alloc(prototype),
      virtual_index,
      prototype.return_type,
      args_slice,
    );
    let body = ExpressionTE::Block(self.typing_interner.alloc(BlockTE::new(
      synth_range,
      ExpressionTE::Return(self.typing_interner.alloc(ReturnTE::new(
        synth_range,
        ExpressionTE::InterfaceFunctionCall(self.typing_interner.alloc(ifc)),
      ))),
    )));

    Ok((header, body))
  }
}
