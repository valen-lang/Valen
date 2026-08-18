use crate::postparsing::ast::LocationInDenizen;
use crate::postparsing::expressions::*;
use crate::postparsing::names::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::utils::fx::HashSet;
use crate::utils::range::RangeS;
use std::iter::once;

// deleted: delegate trait removed per god-struct refactor (Compiler now holds all methods directly)

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn evaluate_block(
    &self,
    parent_fate: &mut FunctionEnvironmentBuilder<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    life: LocationInFunctionEnvironmentT<'t>,
    parent_ranges: &'t [RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    region: RegionT,
    block_1: &'s BlockSE<'s>,
  ) -> (
    &'t BlockTE<'s, 't>,
    HashSet<IVarNameT<'s, 't>>,
    HashSet<IVarNameT<'s, 't>>,
    HashSet<KindT<'s, 't>>,
  ) {
    panic!("Unimplemented: Slab 15");
    // evaluateBlockStatements with child fate, BlockTE wrap, return effect sets via getEffectsSince
  }

  pub fn evaluate_block_statements_block(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    starting_nenv: &'t NodeEnvironmentT<'s, 't>,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    parent_ranges: &'t [RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    life: LocationInFunctionEnvironmentT<'t>,
    region: RegionT,
    block_se: &'s BlockSE<'s>,
  ) -> Result<(ExpressionTE<'s, 't>, HashSet<KindT<'s, 't>>), ICompileErrorT<'s, 't>> {
    let (undestructed_root_expression_with_pending_temps, returns_from_exprs, pending_drops_from_exprs) = self
      .evaluate_expression(
        coutputs,
        nenv,
        life.add(self.typing_interner, 0),
        parent_ranges,
        call_location,
        region,
        block_se.expr,
      )?;

    let drop_range = RangeS::new(block_se.range.end, block_se.range.end);
    let drop_ranges: Vec<RangeS<'s>> =
      once(drop_range).chain(parent_ranges.iter().copied()).collect();

    // Do any temporary locals' pending drops
    let undestructed_root_expression =
        self.drop_since(
          coutputs,
          nenv,
          &drop_ranges,
          call_location,
          life,
          region,
          undestructed_root_expression_with_pending_temps,
          pending_drops_from_exprs.take_vars()
        )?;

    // Drop any still live local variables
    let new_expr =
        self.drop_since(
          coutputs,
          nenv,
          &drop_ranges,
          call_location,
          life,
          region,
          undestructed_root_expression,
          nenv.snapshot(self.typing_interner).get_live_variables_introduced_since(starting_nenv)
        )?;

    Ok((new_expr, returns_from_exprs))
  }
}
