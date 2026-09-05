use crate::postparsing::ast::FunctionS;
use crate::postparsing::ast::{IBodyS, LocationInDenizen, ParameterS};
use crate::postparsing::expressions::{BodySE, IExpressionSE};
use crate::postparsing::patterns::patterns::AtomSP;
use crate::typing::ast::ast::{LocT, ParameterT};
use crate::typing::ast::expressions::{
  ArgLookupTE, BlockTE, ExpressionTE, LetNormalTE, ReturnTE, VoidLiteralTE,
};
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::env::environment::IInDenizenEnvironmentT;
use crate::typing::env::function_environment_t::{FunctionEnvironmentT, NodeEnvironmentBox};
use crate::typing::types::types::{KindT, NeverT, RegionT};
use crate::utils::fx::HashSet;
use crate::utils::range::RangeS;
use std::iter::once;

// deleted: delegate trait removed per god-struct refactor (Compiler now holds all methods directly)

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn declare_and_evaluate_function_body(
    &self,
    func_outer_env: &'t FunctionEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    loct: LocT<'t>,
    parent_ranges: &'t [RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    function_1: &'s FunctionS<'s>,
    maybe_explicit_return_coord: Option<KindT<'s, 't>>,
    params_2: &'t [ParameterT<'s, 't>],
    is_destructor: bool,
  ) -> Result<(Option<KindT<'s, 't>>, &'t BlockTE<'s, 't>), ICompileErrorT<'s, 't>> {
    // val bodyS = function1.body match { case CodeBodyS(b) => b; case _ => vwat() }
    let body_s = match &function_1.body {
      IBodyS::CodeBody(b) => b,
      _ => panic!("Expected CodeBodyS"),
    };

    // maybeExplicitReturnCoord match { ... }
    match maybe_explicit_return_coord {
      None => {
        let (body2, returns) = match self.evaluate_function_body(
          func_outer_env,
          coutputs,
          loct,
          parent_ranges,
          func_outer_env.default_region,
          call_location,
          &function_1.params.iter().collect::<Vec<_>>(),
          params_2,
          body_s.body,
          is_destructor,
          None,
        )? {
          Err(ResultTypeMismatchError { expected_type, actual_type }) => {
            let range_list: &'t [RangeS<'s>] = self.typing_interner.alloc_slice_copy(
              &once(function_1.range).chain(parent_ranges.iter().copied()).collect::<Vec<_>>(),
            );
            return Err(ICompileErrorT::BodyResultDoesntMatch {
              range: range_list,
              function_name: function_1.name,
              expected_return_type: expected_type,
              result_type: actual_type,
            });
          }
          Ok((body, returns)) => (body, returns),
        };

        assert!(body2.result != KindT::Never(NeverT { from_break: true }));
        let return_type2 =
          if returns.is_empty() && body2.result == KindT::Never(NeverT { from_break: false }) {
            // No returns yet the body results in a Never. This can happen if we call panic from inside.
            body2.result
          } else {
            assert!(!returns.is_empty());
            if returns.len() > 1 {
              panic!("Can't infer return type because {} types are returned", returns.len());
            }
            *returns.iter().next().unwrap()
          };

        Ok((Some(return_type2), body2))
      }
      Some(explicit_ret_coord) => {
        // val (body2, returns) = evaluateFunctionBody(...)
        let (body2, returns) = match self.evaluate_function_body(
          func_outer_env,
          coutputs,
          loct,
          parent_ranges,
          func_outer_env.default_region,
          call_location,
          &function_1.params.iter().collect::<Vec<_>>(),
          params_2,
          body_s.body,
          is_destructor,
          Some(explicit_ret_coord),
        )? {
          Err(ResultTypeMismatchError { expected_type, actual_type }) => {
            let range_list: &'t [RangeS<'s>] = self.typing_interner.alloc_slice_copy(
              &once(function_1.range).chain(parent_ranges.iter().copied()).collect::<Vec<_>>(),
            );
            return Err(ICompileErrorT::BodyResultDoesntMatch {
              range: range_list,
              function_name: function_1.name,
              expected_return_type: expected_type,
              result_type: actual_type,
            });
          }
          Ok((body, returns)) => (body, returns),
        };

        // vcurious(returns.size <= 1)
        assert!(returns.len() <= 1);
        // (returns.headOption, body2.result.kind) match { ... }
        match (returns.iter().next(), body2.result) {
          (Some(x), _) if *x == explicit_ret_coord => {
            // Let it through, it returns the expected type.
          }
          // VCOORD: doublecheck this: post-cut Share+Never is rejected by CoordT::new, so this guard should be unreachable.
          (Some(KindT::Never(NeverT { from_break: false })), _) => {
            // Let it through, it returns a never but we expect something else, that's fine
          }
          (None, KindT::Never(NeverT { from_break: false })) => {
            // Let it through, it doesn't return anything yet it results in a never, which means
            // we called panic or something from inside.
          }
          _ => {
            panic!("implement: CouldntConvertForReturnT error");
            // throw CompileErrorExceptionT(CouldntConvertForReturnT(range :: parentRanges, returnType, actualReturnType))
          }
        }

        Ok((None, body2))
      }
    }
  }
}

pub struct ResultTypeMismatchError<'s, 't> {
  pub expected_type: KindT<'s, 't>,
  pub actual_type: KindT<'s, 't>,
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn evaluate_function_body(
    &self,
    func_outer_env: &'t FunctionEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    loct: LocT<'t>,
    parent_ranges: &'t [RangeS<'s>],
    region: RegionT,
    call_location: LocationInDenizen<'s>,
    params_1: &[&'s ParameterS<'s>],
    params_2: &'t [ParameterT<'s, 't>],
    body_1: &'s BodySE<'s>,
    is_destructor: bool,
    maybe_expected_result_type: Option<KindT<'s, 't>>,
  ) -> Result<
    Result<(&'t BlockTE<'s, 't>, HashSet<KindT<'s, 't>>), ResultTypeMismatchError<'s, 't>>,
    ICompileErrorT<'s, 't>,
  > {
    // val env = NodeEnvironmentBox(funcOuterEnv.makeChildNodeEnvironment(body1.block, life))
    let block_as_expr: &'s IExpressionSE<'s> =
      self.scout_arena.alloc(IExpressionSE::Block(body_1.block));
    let mut env = func_outer_env.make_child_node_environment(block_as_expr, loct.clone());

    let starting_env = env.snapshot(self.typing_interner);

    // val patternsTE = evaluateLets(env, coutputs, life + 0, body1.range :: parentRanges, callLocation, region, params1, params2)
    let range_list: &'t [RangeS<'s>] = self.typing_interner.alloc_slice_copy(
      &once(body_1.range).chain(parent_ranges.iter().copied()).collect::<Vec<_>>(),
    );
    let params_2_refs: Vec<&'t ParameterT<'s, 't>> = params_2.iter().collect();
    let patterns_te = self.evaluate_lets(
      &mut env,
      coutputs,
      loct.add(self.typing_interner, 0),
      range_list,
      call_location,
      region,
      params_1,
      &params_2_refs,
    );

    let (statements_from_block, returns_from_inside_maybe_with_never) = self
      .evaluate_block_statements(
        coutputs,
        starting_env,
        &mut env,
        loct.add(self.typing_interner, 1),
        parent_ranges,
        call_location,
        starting_env.default_region,
        body_1.block,
      )?;

    let unconverted_body_without_return = self.consecutive(parent_ranges[0], &[patterns_te, statements_from_block]);

    let starting_env_ref = IInDenizenEnvironmentT::Node(starting_env);
    let converted_body_without_return = match maybe_expected_result_type {
      None => unconverted_body_without_return,
      Some(expected_result_type) => {
        if self.is_type_convertible(
          coutputs,
          starting_env_ref,
          parent_ranges,
          call_location,
          unconverted_body_without_return.result(),
          expected_result_type,
        ) {
          if unconverted_body_without_return.result() == KindT::Never(NeverT { from_break: false })
          {
            unconverted_body_without_return
          } else {
            self.convert(
              &mut env,
              loct,
              coutputs,
              &range_list,
              call_location,
              region,
              unconverted_body_without_return,
              expected_result_type,
            )?
          }
        } else {
          return Ok(Err(ResultTypeMismatchError {
            expected_type: expected_result_type,
            actual_type: unconverted_body_without_return.result(),
          }));
        }
      }
    };

    let (converted_body_with_return, returns_maybe_with_never) =
      if converted_body_without_return.result() == KindT::Never(NeverT { from_break: false }) {
        (converted_body_without_return, returns_from_inside_maybe_with_never)
      } else {
        let mut returns = returns_from_inside_maybe_with_never;
        returns.insert(converted_body_without_return.result());
        let return_te = ExpressionTE::Return(
          self.typing_interner.alloc(ReturnTE::new(parent_ranges[0], converted_body_without_return)),
        );
        (return_te, returns)
      };

    let returns = if returns_maybe_with_never.len() > 1 {
      returns_maybe_with_never
        .into_iter()
        .filter(|c| !matches!(c, KindT::Never(NeverT { from_break: false })))
        .collect()
    } else {
      returns_maybe_with_never
    };

    if is_destructor {
      // If it's a destructor, make sure that we've actually destroyed/moved/unlet'd
      // the parameter, because otherwise we'll get infinite recursion like in this function:
      //     func drop(self Ship) {
      //       // implicitly calls drop(self) which is... this function. infinite recursion.
      //     }
      // For now, we'll just check if it's been moved away, but soon
      // we'll want fate to track whether it's been destroyed, and do that check instead.
      // We don't want the user to accidentally just move it somewhere, they need to
      // promise it gets destroyed.
      // The parameter's `ParameterT.name` isn't the same value as the local it was bound into (the
      // binding carries the unique `Local` name, the parameter its source name), so resolve the
      // parameter to its actual local and check that local's name — which is what unstackify records.
      let param_imprecise = params_2[0]
        .name
        .imprecise_name()
        .expect("destructee param has no imprecise name");
      let destructee_name = env
        .get_variable(param_imprecise, self.typing_interner)
        .expect("destructee param not bound as a local")
        .name();
      if !env.unstackified_locals.contains(&destructee_name) {
        panic!("Destructee wasn't moved/destroyed!");
      }
    }

    Ok(Ok((&*self.typing_interner.alloc(BlockTE::new(parent_ranges[0], converted_body_with_return)), returns)))
  }

  pub fn evaluate_lets(
    &self,
    nenv: &mut NodeEnvironmentBox<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    loct: LocT<'t>,
    range: &'t [RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    region: RegionT,
    params_1: &[&'s ParameterS<'s>],
    params_2: &[&'t ParameterT<'s, 't>],
  ) -> ExpressionTE<'s, 't> {
    // val paramLookups2 = params2.zipWithIndex.map({ case (p, index) => ArgLookupTE(index, p.tyype) })
    let param_lookups_2: Vec<ExpressionTE<'s, 't>> = params_2
      .iter()
      .enumerate()
      .map(|(index, p)| {
        ExpressionTE::ArgLookup(self.typing_interner.alloc(ArgLookupTE::new(range[0], index as i32, p.tyype)))
      })
      .collect();

    // A param's name is its binding: bind each one to its argument. A destructuring param
    // additionally gets a `<destructure> = <name>;` let at the body head, synthesized during
    // postparse, so no pattern is translated here. Synthetic DesugaredParamNames are bound too,
    // since that body-head let loads them by name (see @PFVSZ).
    let mut let_exprs: Vec<ExpressionTE<'s, 't>> = Vec::new();
    for (param_1, param_lookup_2) in params_1.iter().zip(param_lookups_2.into_iter()) {
      let local = self.make_user_local_variable(
        coutputs,
        nenv,
        range,
        param_1.name,
        param_lookup_2.result(),
      );
      let_exprs.push(ExpressionTE::LetNormal(
        self.typing_interner.alloc(LetNormalTE::new(range[0], local, param_lookup_2)),
      ));
    }

    // todo: at this point, to allow for recursive calls, add a callable type to the environment
    // for everything inside the body to use

    let_exprs.push(ExpressionTE::VoidLiteral(
      self.typing_interner.alloc(VoidLiteralTE::new(range[0], nenv.default_region())),
    ));
    self.consecutive(range[0], &let_exprs)
  }
}
