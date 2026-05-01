use crate::higher_typing::ast::FunctionA;
use crate::postparsing::ast::{IBodyS, LocationInDenizen, ParameterS};
use crate::postparsing::expressions::{BodySE, IExpressionSE};
use crate::postparsing::patterns::patterns::AtomSP;
use crate::typing::ast::ast::{LocationInFunctionEnvironmentT, ParameterT};
use crate::typing::ast::expressions::{ArgLookupTE, BlockTE, ReferenceExpressionTE};
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::env::function_environment_t::{FunctionEnvironmentT, NodeEnvironmentBuilder};
use crate::typing::types::types::{CoordT, RegionT};
use crate::utils::range::RangeS;
use std::collections::HashSet;

/*
package dev.vale.typing.function

//import dev.vale.astronomer.{AtomSP, FunctionA, BodySE, ExportA, IExpressionSE, IFunctionAttributeA, LocalA, ParameterS, PureA, UserFunctionA}
import dev.vale.{Err, Ok, RangeS, Result, vassert, vcurious, vpass, vwat}
import dev.vale.highertyping.FunctionA
import dev.vale.parsing.ast.INameDeclarationP
import dev.vale.postparsing.patterns.{AtomSP, CaptureS}
import dev.vale.postparsing._
import dev.vale.typing._
import dev.vale.typing.ast.{ArgLookupTE, BlockTE, LocationInFunctionEnvironmentT, ParameterT, ReferenceExpressionTE, ReturnTE}
import dev.vale.typing.env.{NodeEnvironmentT, NodeEnvironmentBox}
import dev.vale.typing.names._
import dev.vale.typing.types._
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale._
import dev.vale.postparsing._
import dev.vale.postparsing.patterns.AtomSP
import dev.vale.typing._
import dev.vale.typing.ast._
import dev.vale.typing.env._

import scala.collection.immutable.{List, Set}
*/
// deleted: delegate trait removed per god-struct refactor (Compiler now holds all methods directly)
/*
trait IBodyCompilerDelegate {
  def evaluateBlockStatements(
    coutputs: CompilerOutputs,
    startingNenv: NodeEnvironmentT,
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    parentRanges: List[RangeS],
      callLocation: LocationInDenizen,
    region: RegionT,
    exprs: BlockSE):
  (ReferenceExpressionTE, Set[CoordT])

  def translatePatternList(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    region: RegionT,
    patterns1: Vector[AtomSP],
    patternInputExprs2: Vector[ReferenceExpressionTE]):
  ReferenceExpressionTE
}
*/
/*
class BodyCompiler(
  opts: TypingPassOptions,

  nameTranslator: NameTranslator,

    templataCompiler: TemplataCompiler,
    convertHelper: ConvertHelper,
    delegate: IBodyCompilerDelegate) {

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn declare_and_evaluate_function_body(
        &self,
        func_outer_env: &'t FunctionEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        life: LocationInFunctionEnvironmentT<'s>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        function_1: &'s FunctionA<'s>,
        maybe_explicit_return_coord: Option<CoordT<'s, 't>>,
        params_2: &'t [ParameterT<'s, 't>],
        is_destructor: bool,
    ) -> (Option<CoordT<'s, 't>>, &'t BlockTE<'s, 't>) {
        // val bodyS = function1.body match { case CodeBodyS(b) => b; case _ => vwat() }
        let body_s = match &function_1.body {
            IBodyS::CodeBody(b) => b,
            _ => panic!("Expected CodeBodyS"),
        };

        // maybeExplicitReturnCoord match { ... }
        match maybe_explicit_return_coord {
            None => {
                panic!("implement: declareAndEvaluateFunctionBody — inferred return type");
            }
            Some(explicit_ret_coord) => {
                // val (body2, returns) = evaluateFunctionBody(...)
                let (body2, returns) =
                    self.evaluate_function_body(
                        func_outer_env, coutputs, life, parent_ranges,
                        func_outer_env.default_region, call_location,
                        &function_1.params.iter().collect::<Vec<_>>(), params_2, body_s.body,
                        is_destructor, Some(explicit_ret_coord))
                    .unwrap_or_else(|_| panic!("implement: BodyResultDoesntMatch error handling"));

                // vcurious(returns.size <= 1)
                assert!(returns.len() <= 1);
                // (returns.headOption, body2.result.kind) match { ... }
                match returns.iter().next() {
                    Some(x) if *x == explicit_ret_coord => {
                        // Let it through, it returns the expected type.
                    }
                    _ => {
                        panic!("implement: return type checking branches");
                    }
                }

                (None, body2)
            }
        }
    }
/*
  // Returns:
  // - IF we had to infer it, the return type.
  // - The body.
  def declareAndEvaluateFunctionBody(
    funcOuterEnv: FunctionEnvironmentT,
    coutputs: CompilerOutputs,
    life: LocationInFunctionEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    function1: FunctionA,
    maybeExplicitReturnCoord: Option[CoordT],
    params2: Vector[ParameterT],
    isDestructor: Boolean):
  (Option[CoordT], BlockTE) = {
    val bodyS =
      function1.body match {
        case CodeBodyS(b) => b
        case _ => vwat()
      }

      maybeExplicitReturnCoord match {
        case None => {
          val (body2, returns) =
            evaluateFunctionBody(
                funcOuterEnv,
              coutputs,
              life,
              parentRanges,
              funcOuterEnv.defaultRegion,
              callLocation,
              function1.params,
              params2,
              bodyS,
              isDestructor,
              None) match {
              case Err(ResultTypeMismatchError(expectedType, actualType)) => {
                throw CompileErrorExceptionT(BodyResultDoesntMatch(function1.range :: parentRanges, function1.name, expectedType, actualType))

              }
              case Ok((body, returns)) => (body, returns)
            }

          vassert(body2.result.kind != NeverT(true))
          val returnType2 =
            if (returns.isEmpty && body2.result.kind == NeverT(false)) {
              // No returns yet the body results in a Never. This can happen if we call panic from inside.
              body2.result.coord
            } else {
              vassert(returns.nonEmpty)
              if (returns.size > 1) {
                throw CompileErrorExceptionT(RangedInternalErrorT(bodyS.range :: parentRanges, "Can't infer return type because " + returns.size + " types are returned:" + returns.map("\n" + _)))
              }
              returns.head
            }

          (Some(returnType2), body2)
        }
        case Some(explicitRetCoord) => {
          val (body2, returns) =
            evaluateFunctionBody(
                funcOuterEnv,
                coutputs,
                life,
                parentRanges,
              funcOuterEnv.defaultRegion,
              callLocation,
                function1.params,
                params2,
                bodyS,
                isDestructor,
                Some(explicitRetCoord)) match {
              case Err(ResultTypeMismatchError(expectedType, actualType)) => {
                throw CompileErrorExceptionT(BodyResultDoesntMatch(function1.range :: parentRanges, function1.name, expectedType, actualType))
              }
              case Ok((body, returns)) => (body, returns)
            }


          vcurious(returns.size <= 1)
          (returns.headOption, body2.result.kind) match {
            case (Some(x), _) if x == explicitRetCoord => {
              // Let it through, it returns the expected type.
            }
            case (Some(CoordT(ShareT, _, NeverT(false))), _) => {
              // Let it through, it returns a never but we expect something else, that's fine
            }
            case (None, NeverT(false)) => {
              // Let it through, it doesn't return anything yet it results in a never, which means
              // we called panic or something from inside.
            }
            case _ => {
              throw CompileErrorExceptionT(
                CouldntConvertForReturnT(
                  bodyS.range :: parentRanges, explicitRetCoord, returns.head))
            }
          }

          (None, body2)
        }
      }
  }

*/
}

pub struct ResultTypeMismatchError;
/*
  case class ResultTypeMismatchError(expectedType: CoordT, actualType: CoordT) {
    val hash = runtime.ScalaRunTime._hashCode(this)
    override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious();
    vpass()
  }

*/

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_function_body(
        &self,
        func_outer_env: &'t FunctionEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        life: LocationInFunctionEnvironmentT<'s>,
        parent_ranges: &[RangeS<'s>],
        region: RegionT,
        call_location: LocationInDenizen<'s>,
        params_1: &[&'s ParameterS<'s>],
        params_2: &'t [ParameterT<'s, 't>],
        body_1: &'s BodySE<'s>,
        is_destructor: bool,
        maybe_expected_result_type: Option<CoordT<'s, 't>>,
    ) -> Result<(&'t BlockTE<'s, 't>, HashSet<CoordT<'s, 't>>), ResultTypeMismatchError> {
        // val env = NodeEnvironmentBox(funcOuterEnv.makeChildNodeEnvironment(body1.block, life))
        let block_as_expr: &'s IExpressionSE<'s> =
            self.scout_arena.alloc(IExpressionSE::Block(body_1.block));
        let mut env = func_outer_env.make_child_node_environment(block_as_expr, life.clone());

        let starting_env = env.snapshot(self.typing_interner);

        // val patternsTE = evaluateLets(env, coutputs, life + 0, body1.range :: parentRanges, callLocation, region, params1, params2)
        let range_list: Vec<RangeS<'s>> =
            std::iter::once(body_1.range).chain(parent_ranges.iter().copied()).collect();
        let params_2_refs: Vec<&'t ParameterT<'s, 't>> = params_2.iter().collect();
        let patterns_te = self.evaluate_lets(
            &mut env, coutputs, life.add(0),
            &range_list, call_location, region, params_1, &params_2_refs);

        let (statements_from_block, returns_from_inside_maybe_with_never) =
            self.evaluate_block_statements(
                coutputs, starting_env, &mut env, life.add(1),
                parent_ranges, call_location, starting_env.default_region, body_1.block);

        let unconverted_body_without_return =
            self.consecutive(&[patterns_te, statements_from_block]);

        let converted_body_without_return = match maybe_expected_result_type {
            None => {
                panic!("implement: evaluateFunctionBody — None expectedResultType");
            }
            Some(expected_result_type) => {
                panic!("implement: evaluateFunctionBody — type conversion");
            }
        };

        panic!("implement: evaluateFunctionBody — return wrapping");
    }
/*
  private def evaluateFunctionBody(
    funcOuterEnv: FunctionEnvironmentT,
    coutputs: CompilerOutputs,
    life: LocationInFunctionEnvironmentT,
    parentRanges: List[RangeS],
    region: RegionT,
      callLocation: LocationInDenizen,
    params1: Vector[ParameterS],
    params2: Vector[ParameterT],
    body1: BodySE,
    isDestructor: Boolean,
    maybeExpectedResultType: Option[CoordT]):
  Result[(BlockTE, Set[CoordT]), ResultTypeMismatchError] = {
    val env = NodeEnvironmentBox(funcOuterEnv.makeChildNodeEnvironment(body1.block, life))
    val startingEnv = env.snapshot

    val patternsTE =
      evaluateLets(env, coutputs, life + 0, body1.range :: parentRanges, callLocation, region, params1, params2);

    val (statementsFromBlock, returnsFromInsideMaybeWithNever) =
      delegate.evaluateBlockStatements(
        coutputs, startingEnv, env, life + 1, parentRanges, callLocation, startingEnv.defaultRegion, body1.block);

    val unconvertedBodyWithoutReturn = Compiler.consecutive(Vector(patternsTE, statementsFromBlock))


    val convertedBodyWithoutReturn =
      maybeExpectedResultType match {
        case None => unconvertedBodyWithoutReturn
        case Some(expectedResultType) => {
          if (templataCompiler.isTypeConvertible(coutputs, startingEnv, parentRanges, callLocation, unconvertedBodyWithoutReturn.result.coord, expectedResultType)) {
            if (unconvertedBodyWithoutReturn.kind == NeverT(false)) {
              unconvertedBodyWithoutReturn
            } else {
              convertHelper.convert(funcOuterEnv, coutputs, body1.range :: parentRanges, callLocation, unconvertedBodyWithoutReturn, expectedResultType);
            }
          } else {
            return Err(ResultTypeMismatchError(expectedResultType, unconvertedBodyWithoutReturn.result.coord))
          }
        }
      }


    // If the function doesn't end in a ret, then add one for it.
    val (convertedBodyWithReturn, returnsMaybeWithNever) =
      if (convertedBodyWithoutReturn.kind == NeverT(false)) {
        (convertedBodyWithoutReturn, returnsFromInsideMaybeWithNever)
      } else {
        (ReturnTE(convertedBodyWithoutReturn), returnsFromInsideMaybeWithNever + convertedBodyWithoutReturn.result.coord)
      }
    // If we already had a ret, then the above will add a Never to the returns, but that's fine, it will be filtered
    // out below.


    // val returns =
    //   if (returnsMaybeWithNever.size > 1 && returnsMaybeWithNever.contains(CoordT(ShareT, NeverT(false)))) {
    //     returnsMaybeWithNever - CoordT(ShareT, NeverT(false))
    //   } else {
    //     returnsMaybeWithNever
    //   }

    val returns =
      if (returnsMaybeWithNever.size > 1) {
        returnsMaybeWithNever.filter({
          case CoordT(ShareT, _, NeverT(false)) => false
          case _ => true
        })
      } else {
        returnsMaybeWithNever
      }

    if (isDestructor) {
      // If it's a destructor, make sure that we've actually destroyed/moved/unlet'd
      // the parameter. For now, we'll just check if it's been moved away, but soon
      // we'll want fate to track whether it's been destroyed, and do that check instead.
      // We don't want the user to accidentally just move it somewhere, they need to
      // promise it gets destroyed.
      val destructeeName = params2.head.name
      if (!env.unstackifieds.contains(destructeeName)) {
        throw CompileErrorExceptionT(RangedInternalErrorT(body1.range :: parentRanges, "Destructee wasn't moved/destroyed!"))
      }
    }

    Ok((ast.BlockTE(convertedBodyWithReturn), returns))
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_lets(
        &self,
        nenv: &mut NodeEnvironmentBuilder<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        life: LocationInFunctionEnvironmentT<'s>,
        range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        region: RegionT,
        params_1: &[&'s ParameterS<'s>],
        params_2: &[&'t ParameterT<'s, 't>],
    ) -> &'t ReferenceExpressionTE<'s, 't> {
        // val paramLookups2 = params2.zipWithIndex.map({ case (p, index) => ArgLookupTE(index, p.tyype) })
        let param_lookups_2: Vec<ReferenceExpressionTE<'s, 't>> =
            params_2.iter().enumerate().map(|(index, p)| {
                ReferenceExpressionTE::ArgLookup(ArgLookupTE {
                    param_index: index as i32,
                    coord: p.tyype,
                })
            }).collect();

        let param_lookups_2_refs: Vec<&'t ReferenceExpressionTE<'s, 't>> =
            param_lookups_2.into_iter().map(|e| &*self.typing_interner.alloc(e)).collect();
        let patterns: Vec<&'s AtomSP<'s>> = params_1.iter().map(|p| &p.pattern).collect();
        let let_exprs_2 = self.translate_pattern_list(
            coutputs, nenv, life, range, call_location,
            &patterns, &param_lookups_2_refs, region);

        // todo: at this point, to allow for recursive calls, add a callable type to the environment
        // for everything inside the body to use

        if !params_1.is_empty() {
            panic!("implement: evaluateLets — params1.foreach check");
        }

        let_exprs_2
    }
/*
  // Produce the lets at the start of a function.
  private def evaluateLets(
      nenv: NodeEnvironmentBox,
      coutputs: CompilerOutputs,
      life: LocationInFunctionEnvironmentT,
      range: List[RangeS],
      callLocation: LocationInDenizen,
      region: RegionT,
      params1: Vector[ParameterS],
      params2: Vector[ParameterT]):
  ReferenceExpressionTE = {
    val paramLookups2 =
      params2.zipWithIndex.map({ case (p, index) => ArgLookupTE(index, p.tyype) })
    val letExprs2 =
      delegate.translatePatternList(
        coutputs, nenv, life, range, callLocation, region, params1.map(_.pattern), paramLookups2);

    // todo: at this point, to allow for recursive calls, add a callable type to the environment
    // for everything inside the body to use

    params1.foreach({
      case ParameterS(_, _, _, AtomSP(_, Some(CaptureS(name, false)), _, _)) => {
        if (!nenv.declaredLocals.exists(_.name == nameTranslator.translateVarNameStep(name))) {
          throw CompileErrorExceptionT(RangedInternalErrorT(range, "wot couldnt find " + name))
        }
      }
      case _ =>
    });

    (letExprs2)
  }

}
*/
}