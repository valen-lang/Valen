use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::postparsing::ast::LocationInDenizen;
use crate::utils::range::RangeS;
use crate::postparsing::names::*;
use crate::postparsing::expressions::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::compiler_outputs::*;
use std::collections::HashSet;
use std::iter::once;

/*
package dev.vale.typing.expression

//import dev.vale.astronomer.{BlockSE, IExpressionSE}
import dev.vale.RangeS
import dev.vale.postparsing._
import dev.vale.typing._
import dev.vale.typing.ast.{BlockTE, LocationInFunctionEnvironmentT, ReferenceExpressionTE}
import dev.vale.typing.env._
import dev.vale.typing.function.DestructorCompiler
import dev.vale.typing.names._
import dev.vale.typing.types.CoordT
import dev.vale.postparsing.ExpressionScout
import dev.vale.typing._
import dev.vale.typing.ast._
import dev.vale.typing.env._
import dev.vale.typing.types._
import dev.vale.RangeS
import dev.vale.typing.templata._

import scala.collection.immutable.{List, Set}

*/
// deleted: delegate trait removed per god-struct refactor (Compiler now holds all methods directly)
/*
trait IBlockCompilerDelegate {
  def evaluateAndCoerceToReferenceExpression(
    coutputs: CompilerOutputs,
    nenv: NodeEnvironmentBox,
    life: LocationInFunctionEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    region: RegionT,
    expr1: IExpressionSE):
  (ReferenceExpressionTE, Set[CoordT])

  def dropSince(
    coutputs: CompilerOutputs,
    startingNenv: NodeEnvironmentT,
    nenv: NodeEnvironmentBox,
    range: List[RangeS],
    callLocation: LocationInDenizen,
    life: LocationInFunctionEnvironmentT,
    region: RegionT,
    unresultifiedUndestructedExpressions: ReferenceExpressionTE):
  ReferenceExpressionTE
}
*/
/*
class BlockCompiler(
    opts: TypingPassOptions,

    destructorCompiler: DestructorCompiler,
    localHelper: LocalHelper,
    delegate: IBlockCompilerDelegate) {

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn evaluate_block(
        &self,
        parent_fate: &mut FunctionEnvironmentBuilder<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        life: LocationInFunctionEnvironmentT<'s, 't>,
        parent_ranges: &'t [RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        region: RegionT,
        block_1: &'s BlockSE<'s>,
    ) -> (&'t BlockTE<'s, 't>, HashSet<IVarNameT<'s, 't>>, HashSet<IVarNameT<'s, 't>>, HashSet<CoordT<'s, 't>>) {
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  // This is NOT USED FOR EVERY BLOCK!
  // This is just for the simplest kind of block.
  // This can serve as an example for how we can use together all the tools provided by BlockCompiler.
  // Returns:
  // - The resulting block expression
  // - All locals from outside the block that we unstackified inside the block
  // - All locals from outside the block that we restackified inside the block
  // - Num anonymous variables that were made
  // - Types of all returns from inside the block
  def evaluateBlock(
    parentFate: FunctionEnvironmentBoxT,
    coutputs: CompilerOutputs,
    life: LocationInFunctionEnvironmentT,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    region: RegionT,
    block1: BlockSE):
  (BlockTE, Set[IVarNameT], Set[IVarNameT], Set[CoordT]) = {
    val nenv = NodeEnvironmentBox(parentFate.makeChildNodeEnvironment(block1, life))
    val startingNenv = nenv.snapshot

    val (expressionsWithResult, returnsFromExprs) =
      evaluateBlockStatements(
        coutputs, startingNenv, nenv, parentRanges, callLocation, life, region, block1)

    val block2 = BlockTE(expressionsWithResult)

    val (unstackifiedAncestorLocals, restackifiedAncestorLocals) = nenv.snapshot.getEffectsSince(startingNenv)
    (block2, unstackifiedAncestorLocals, restackifiedAncestorLocals, returnsFromExprs)
  }
*/
    pub fn evaluate_block_statements_block(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        starting_nenv: &'t NodeEnvironmentT<'s, 't>,
        nenv: &mut NodeEnvironmentBox<'s, 't>,
        parent_ranges: &'t [RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        life: LocationInFunctionEnvironmentT<'s, 't>,
        region: RegionT,
        block_se: &'s BlockSE<'s>,
    ) -> Result<(ReferenceExpressionTE<'s, 't>, HashSet<CoordT<'s, 't>>), ICompileErrorT<'s, 't>> {
        let (unnevered_unresultified_undestructed_root_expression, returns_from_exprs) =
            self.evaluate_and_coerce_to_reference_expression(
                coutputs, nenv, life.add(self.typing_interner, 0), parent_ranges,
                call_location, region, block_se.expr)?;

        let unresultified_undestructed_expressions =
            unnevered_unresultified_undestructed_root_expression;

        let drop_range = RangeS { begin: block_se.range.end, end: block_se.range.end };
        let drop_ranges: Vec<RangeS<'s>> =
            once(drop_range).chain(parent_ranges.iter().copied()).collect();
        let new_expr =
            self.drop_since(
                coutputs, starting_nenv, nenv,
                &drop_ranges, call_location, life, region,
                unresultified_undestructed_expressions)?;

        Ok((new_expr, returns_from_exprs))
    }
/*
  def evaluateBlockStatements(
    coutputs: CompilerOutputs,
    startingNenv: NodeEnvironmentT,
    nenv: NodeEnvironmentBox,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    life: LocationInFunctionEnvironmentT,
    region: RegionT,
    blockSE: BlockSE):
  (ReferenceExpressionTE, Set[CoordT]) = {
    val (unneveredUnresultifiedUndestructedRootExpression, returnsFromExprs) =
      delegate.evaluateAndCoerceToReferenceExpression(
        coutputs, nenv, life + 0, parentRanges,
        callLocation, region, blockSE.expr);

    val unneveredUnresultifiedUndestructedExpressions =
      unneveredUnresultifiedUndestructedRootExpression

//    val unneveredUnresultifiedUndestructedExpressions =
//      unneveredUnresultifiedUndestructedRootExpression match {
//        case ConsecutorTE(exprs) => exprs
//        case other => Vector(other)
//      }

    val unresultifiedUndestructedExpressions =
      unneveredUnresultifiedUndestructedExpressions
//    val unresultifiedUndestructedExpressions =
//      unneveredUnresultifiedUndestructedExpressions.indexWhere(_.kind == NeverT()) match {
//        case -1 => unneveredUnresultifiedUndestructedExpressions
//        case indexOfFirstNever => {
//          unneveredUnresultifiedUndestructedExpressions.zipWithIndex.map({ case (e, i) =>
//            if (i <= indexOfFirstNever) {
//              e
//            } else {
//              UnreachableMootTE(e)
//            }
//          })
//        }
//      }

    val newExpr =
      delegate.dropSince(
        coutputs,
        startingNenv,
        nenv,
        RangeS(blockSE.range.end, blockSE.range.end) :: parentRanges,
        callLocation,
        life,
        region,
        unresultifiedUndestructedExpressions)

    (newExpr, returnsFromExprs)
  }

//  private def evaluateBlockStatementsInner(
//    coutputs: CompilerOutputs,
//    nenv: FunctionEnvironmentBox,
//    life: LocationInFunctionEnvironment,
//    expr1: List[IExpressionSE]):
//  (List[ReferenceExpressionTE], Set[CoordT]) = {
//    expr1 match {
//      case Nil => (Nil, Set())
//      case first1 :: rest1 => {
//        val (perhapsUndestructedFirstExpr2, returnsFromFirst) =
//          delegate.evaluateAndCoerceToReferenceExpression(
//            coutputs, nenv, life + 0, first1);
//
//        val destructedFirstExpr2 =
//          if (rest1.isEmpty) {
//            // This is the last expression, which means it's getting returned,
//            // so don't destruct it.
//            (perhapsUndestructedFirstExpr2) // Do nothing
//          } else {
//            // This isn't the last expression
//            perhapsUndestructedFirstExpr2.result.kind match {
//              case VoidT() => perhapsUndestructedFirstExpr2
//              case _ => destructorCompiler.drop(nenv, coutputs, perhapsUndestructedFirstExpr2)
//            }
//          }
//
//        val (restExprs2, returnsFromRest) =
//          evaluateBlockStatementsInner(coutputs, nenv, life + 1, rest1)
//
//        (destructedFirstExpr2 +: restExprs2, returnsFromFirst ++ returnsFromRest)
//      }
//    }
//  }

}
*/
}
