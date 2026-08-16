use crate::postparsing::ast::GenericParameterS;
use crate::postparsing::itemplatatype::ITemplataType;
use crate::postparsing::names::IRuneS;
use crate::postparsing::rules::rules::IRulexSR;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::env::environment::IInDenizenEnvironmentT;
use crate::typing::rune_typing::rune_type_solver::RuneTypeSolver;
use crate::utils::fx::IndexMap;
use crate::utils::range::RangeS;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  // Derive a denizen's rune->type map, the type of every rune it mentions.
  // `extra_runes_and_types` carries the few runes that live outside the generic-parameter
  // list, like an impl's own kind runes or a lambda's param kind runes.
  pub fn derive_rune_to_type(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    parent_env: IInDenizenEnvironmentT<'s, 't>,
    ranges: Vec<RangeS<'s>>,
    generic_params: &[&GenericParameterS<'s>],
    rules: &[IRulexSR<'s>],
    extra_runes_and_types: IndexMap<IRuneS<'s>, ITemplataType<'s>>,
  ) -> IndexMap<IRuneS<'s>, ITemplataType<'s>> {
    let mut initial_rune_to_type = extra_runes_and_types;
    for generic_param in generic_params {
      initial_rune_to_type.insert(generic_param.rune.rune, generic_param.tyype.tyype());
    }
    let generic_param_runes: Vec<IRuneS<'s>> =
      generic_params.iter().map(|gp| gp.rune.rune).collect();
    let env = self.create_rune_type_solver_env(parent_env);
    let solver = RuneTypeSolver { scout_arena: self.scout_arena };
    match solver.solve_rune_types(
      coutputs,
      self.opts.global_options.sanity_check,
      &env,
      ranges,
      rules,
      &generic_param_runes,
      true,
      initial_rune_to_type,
    ) {
      Ok(map) => map,
      Err(e) => panic!("CouldntSolveRuneTypesT in derive_rune_to_type: {:?}", e),
    }
  }
}
