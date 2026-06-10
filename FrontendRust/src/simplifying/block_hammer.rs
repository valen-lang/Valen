// From Frontend/SimplifyingPass/src/dev/vale/simplifying/BlockHammer.scala
//
// Scala's `BlockHammer` is collapsed: per typing-pass `Compiler` precedent
// (no sub-`StructCompiler`/`ExpressionCompiler`/etc.), all sub-hammer methods
// live as `impl Hammer { ... }` blocks colocated in per-area files.
// `BlockHammer` itself is NOT a Rust struct.

use crate::final_ast::instructions::{BlockH, ImmutabilifyH, MutabilifyH};
use crate::instantiating::ast::ast::FunctionHeaderI;
use crate::instantiating::ast::expressions::{BlockIE, ImmutabilifyIE, MutabilifyIE};
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::ast::types::cI;
use crate::simplifying::hamuts::Hamuts;
use crate::simplifying::hammer::{Hammer, Locals};
use crate::final_ast::types::KindHT;
use crate::instantiating::ast::expressions::ExpressionIE;
use std::collections::HashSet;

/*
package dev.vale.simplifying

import dev.vale.finalast.{BlockH, ImmutabilifyH, ImmutableBorrowH, ImmutableShareH, MutabilifyH, MutableBorrowH, MutableShareH, NeverHT}
import dev.vale.instantiating.ast._
import dev.vale.{vassert, vcurious, vfail, vimpl, vwat, finalast => m}

class BlockHammer(expressionHammer: ExpressionHammer, typeHammer: TypeHammer) {
*/

// mig: fn translate_block
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_block(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        parent_locals: &mut Locals<'s, 'i, 'h>,
        block2: &BlockIE<'s, 'i, cI>,
    ) -> &'h BlockH<'s, 'h>
    {
        let mut block_locals = parent_locals.snapshot();
        let expr_h = self.translate_expressions_and_deferreds(
            hinputs, hamuts, current_function_header, &mut block_locals,
            &[ExpressionIE::Reference(block2.inner)]);
        let parent_local_ids: HashSet<_> = parent_locals.locals.keys().copied().collect();
        let local_ids_in_this_block: HashSet<_> = block_locals.locals.keys().copied().filter(|k| !parent_local_ids.contains(k)).collect();
        let unstackified_local_ids_in_this_block: HashSet<_> = block_locals.unstackified_vars.iter().copied().filter(|k| local_ids_in_this_block.contains(k)).collect();
        if local_ids_in_this_block != unstackified_local_ids_in_this_block {
            match expr_h.result_type().kind {
                KindHT::NeverHT(_) => {}
                _ => panic!("Ununstackified local: {:?}", local_ids_in_this_block.difference(&unstackified_local_ids_in_this_block).collect::<Vec<_>>()),
            }
        }
        let parent_unstackified: HashSet<_> = parent_locals.unstackified_vars.iter().copied().collect();
        let unstackified_locals_from_parent: Vec<_> = parent_locals.locals.keys().copied()
            .filter(|k| !parent_unstackified.contains(k))
            .filter(|k| block_locals.unstackified_vars.contains(k))
            .collect();
        for var in unstackified_locals_from_parent {
            parent_locals.mark_unstackified(var);
        }
        parent_locals.set_next_local_id_number(block_locals.next_local_id_number);
        self.interner.alloc(BlockH { inner: expr_h })
    }
}
/*
  def translateBlock(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    parentLocals: LocalsBox,
    block2: BlockIE):
  (BlockH) = {
    val blockLocals = LocalsBox(parentLocals.snapshot)

    val exprH =
      expressionHammer.translateExpressionsAndDeferreds(
        hinputs, hamuts, currentFunctionHeader, blockLocals, Vector(block2.inner));

    // We dont vassert(deferreds.isEmpty) here, see BMHD for why.

    val localIdsInThisBlock = blockLocals.locals.keys.toSet.diff(parentLocals.locals.keys.toSet)
//    val localsInThisBlock = localIdsInThisBlock.map(blockLocals.locals)
    val unstackifiedLocalIdsInThisBlock = blockLocals.unstackifiedVars.intersect(localIdsInThisBlock)

    if (localIdsInThisBlock != unstackifiedLocalIdsInThisBlock) {
      exprH.resultType.kind match {
        case NeverHT(_) => {
          // Then it's forgivable, because the block never reaches its end.
        }
        case _ => {
          // This probably means that there was no UnletH or DestructureH for that variable.
          vfail("Ununstackified local: " + (localIdsInThisBlock -- unstackifiedLocalIdsInThisBlock))
        }
      }
    }

    // All the parent locals...
    val unstackifiedLocalsFromParent =
      parentLocals.locals.keySet
        // ...minus the ones that were unstackified before...
        .diff(parentLocals.unstackifiedVars)
        // ...which were unstackified by the block.
        .intersect(blockLocals.unstackifiedVars)
    unstackifiedLocalsFromParent.foreach(parentLocals.markUnstackified)
    parentLocals.setNextLocalIdNumber(blockLocals.nextLocalIdNumber)

//    start here, we're returning locals and thats not optimal DO NOT SUBMIT?
    BlockH(exprH)
  }
*/

// mig: fn translate_mutabilify
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_mutabilify(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        node: &MutabilifyIE<'s, 'i, cI>,
    ) -> &'h MutabilifyH<'s, 'h>
    {
        panic!("Unimplemented: translate_mutabilify");
    }
}
/*
  def translateMutabilify(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    node: MutabilifyIE):
  MutabilifyH = {
    val MutabilifyIE(innerIE, resultCoordI) = node

    val innerHE =
      expressionHammer.translateExpressionsAndDeferreds(
        hinputs, hamuts, currentFunctionHeader, locals, Vector(innerIE))

    val resultCoordH =
      typeHammer.translateCoord(hinputs, hamuts, resultCoordI)

    vassert(
      resultCoordH.ownership ==
        (innerHE.resultType.ownership match {
          case ImmutableShareH => MutableShareH
          case ImmutableBorrowH => MutableBorrowH
          case other => other
        }))

    MutabilifyH(innerHE)
  }
*/

// mig: fn translate_immutabilify
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_immutabilify(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        node: &ImmutabilifyIE<'s, 'i, cI>,
    ) -> &'h ImmutabilifyH<'s, 'h>
    {
        panic!("Unimplemented: translate_immutabilify");
    }
}
/*
  def translateImmutabilify(
      hinputs: HinputsI,
      hamuts: HamutsBox,
      currentFunctionHeader: FunctionHeaderI,
      locals: LocalsBox,
      node: ImmutabilifyIE):
  ImmutabilifyH = {
    val ImmutabilifyIE(innerIE, resultCoordI) = node

    val innerHE =
      expressionHammer.translateExpressionsAndDeferreds(
        hinputs, hamuts, currentFunctionHeader, locals, Vector(innerIE))

    val resultCoordH =
      typeHammer.translateCoord(hinputs, hamuts, resultCoordI)

    vassert(
      resultCoordH.ownership ==
          (innerHE.resultType.ownership match {
            case MutableShareH => ImmutableShareH
            case MutableBorrowH => ImmutableBorrowH
            case other => other
          }))

    ImmutabilifyH(innerHE)
  }
}
*/
