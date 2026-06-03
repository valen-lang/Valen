// From Frontend/SimplifyingPass/src/dev/vale/simplifying/LetHammer.scala
//
// Per typing-pass `Compiler` precedent, `LetHammer` is not a Rust struct.
// Methods become `impl Hammer { ... }` blocks colocated here.
// `LetHammer.BOX_MEMBER_INDEX` (Scala `object LetHammer`) becomes a module
// constant.

use crate::final_ast::instructions::{ExpressionH, RestackifyH, StackifyH};
use crate::final_ast::types::CoordH;
use crate::instantiating::ast::ast::FunctionHeaderI;
use crate::instantiating::ast::expressions::{
    DestroyIE, DestroyStaticSizedArrayIntoLocalsIE, LetAndLendIE, LetNormalIE, ReferenceExpressionIE,
    RestackifyIE, UnletIE,
};
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::ast::names::IVarNameI;
use crate::instantiating::ast::types::{cI, CoordI, VariabilityI};
use crate::simplifying::hamuts::Hamuts;
use crate::simplifying::hammer::{Hammer, Locals};

/*
package dev.vale.simplifying

import dev.vale.finalast._
import dev.vale._
import dev.vale.finalast._
import dev.vale.instantiating.ast._
*/

// mig: const BOX_MEMBER_INDEX
pub const BOX_MEMBER_INDEX: i32 = 0;
/*
object LetHammer {
  val BOX_MEMBER_INDEX: Int = 0
}

class LetHammer(
    typeHammer: TypeHammer,
    nameHammer: NameHammer,
    structHammer: StructHammer,
    expressionHammer: ExpressionHammer,
    loadHammer: LoadHammer) {
*/

// mig: fn translate_let
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_let(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        let2: &'i LetNormalIE<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        let local_variable = let2.variable;
        let source_expr2 = let2.expr;
        let (source_expr_he, deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, locals, crate::instantiating::ast::expressions::ExpressionIE::Reference(source_expr2));
        let source_result_pointer_type_h = self.translate_coord(hinputs, hamuts, source_expr2.result());
        match source_expr_he.result_type().kind {
            crate::final_ast::types::KindHT::NeverHT(_) => return source_expr_he,
            _ => {}
        }
        let stackify_node = match local_variable {
            crate::instantiating::ast::ast::ILocalVariableI::ReferenceLocalVariableI(rlv) => {
                ExpressionH::StackifyH(self.translate_mundane_let(hinputs, hamuts, current_function_header, locals, source_expr_he, source_result_pointer_type_h, &rlv.name, rlv.variability))
            }
            crate::instantiating::ast::ast::ILocalVariableI::AddressibleLocalVariableI(alv) => {
                self.translate_addressible_let(hinputs, hamuts, current_function_header, locals, source_expr_he, source_result_pointer_type_h, &alv.name, alv.variability, alv.collapsed_coord)
            }
        };
        self.translate_deferreds(hinputs, hamuts, current_function_header, locals, stackify_node, deferreds)
    }
}
/*
Guardian: temp-disable: SPDMX — Rust-Scala adaptation: Scala uses subtyping (`StackifyH extends ExpressionH`) so `translateMundaneLet`'s `StackifyH` return is auto-upcast to `ExpressionH` at the assignment site. Rust requires explicit enum-variant wrapping (`ExpressionH::StackifyH(...)`); the wrapper preserves Scala semantics 1:1. translate_mundane_let keeps its `&'h StackifyH` return per the sibling Guardian verdict that rejected widening its signature. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-1731-1780113318898/hook-1731/translate_let--49.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def translateLet(
      hinputs: HinputsI,
      hamuts: HamutsBox,
      currentFunctionHeader: FunctionHeaderI,
      locals: LocalsBox,
      let2: LetNormalIE):
  ExpressionH[KindHT] = {
    val LetNormalIE(localVariable, sourceExpr2, _) = let2

    val (sourceExprHE, deferreds) =
      expressionHammer.translate(hinputs, hamuts, currentFunctionHeader, locals, sourceExpr2);
    val (sourceResultPointerTypeH) =
      typeHammer.translateCoord(hinputs, hamuts, sourceExpr2.result)

    sourceExprHE.resultType.kind match {
      // We'll never get to this let, so strip it out. See BRCOBS.
      case NeverHT(_) => return sourceExprHE
      case _ =>
    }

    val stackifyNode =
      localVariable match {
        case ReferenceLocalVariableI(varId, variability, type2) => {
          translateMundaneLet(
            hinputs, hamuts, currentFunctionHeader, locals, sourceExprHE, sourceResultPointerTypeH, varId, variability)
        }
        case AddressibleLocalVariableI(varId, variability, reference) => {
          translateAddressibleLet(
            hinputs, hamuts, currentFunctionHeader, locals, sourceExprHE, sourceResultPointerTypeH, varId, variability, reference)
        }
      }

    expressionHammer.translateDeferreds(
      hinputs, hamuts, currentFunctionHeader, locals, stackifyNode, deferreds)
  }
*/

// mig: fn translate_restackify
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_restackify(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        let2: &RestackifyIE<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        panic!("Unimplemented: translate_restackify");
    }
}
/*
  def translateRestackify(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    let2: RestackifyIE):
  ExpressionH[KindHT] = {
    val RestackifyIE(localVariable, sourceExpr2, _) = let2

    val (sourceExprHE, deferreds) =
      expressionHammer.translate(hinputs, hamuts, currentFunctionHeader, locals, sourceExpr2);
    val (sourceResultPointerTypeH) =
      typeHammer.translateCoord(hinputs, hamuts, sourceExpr2.result)

    sourceExprHE.resultType.kind match {
      // We'll never get to this let, so strip it out. See BRCOBS.
      case NeverHT(_) => return sourceExprHE
      case _ =>
    }

    val stackifyNode =
      localVariable match {
        case ReferenceLocalVariableI(varId, variability, type2) => {
          translateMundaneRestackify(
            hinputs, hamuts, currentFunctionHeader, locals, sourceExprHE, varId)
        }
        case AddressibleLocalVariableI(varId, variability, reference) => {
          translateAddressibleRestackify(
            hinputs, hamuts, currentFunctionHeader, locals, sourceExprHE, sourceResultPointerTypeH, varId, variability, reference)
        }
      }

    expressionHammer.translateDeferreds(
      hinputs, hamuts, currentFunctionHeader, locals, stackifyNode, deferreds)
  }
*/

// mig: fn translate_let_and_point
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_let_and_point(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        let_ie: &LetAndLendIE<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        let LetAndLendIE { variable: local_variable, expr: source_expr2, target_ownership: _target_ownership, result: _ } = *let_ie;
        let (source_expr_he, deferreds) =
            self.translate_expression(hinputs, hamuts, current_function_header, locals, crate::instantiating::ast::expressions::ExpressionIE::Reference(source_expr2));
        let source_result_pointer_type_h = self.translate_coord(hinputs, hamuts, source_expr2.result());
        let borrow_access = match local_variable {
            crate::instantiating::ast::ast::ILocalVariableI::ReferenceLocalVariableI(r) => {
                self.translate_mundane_let_and_point(hinputs, hamuts, current_function_header, locals, source_expr2, source_expr_he, source_result_pointer_type_h, let_ie, &r.name, r.variability)
            }
            crate::instantiating::ast::ast::ILocalVariableI::AddressibleLocalVariableI(_) => panic!("translate_let_and_point: AddressibleLocalVariableI arm"),
        };
        self.translate_deferreds(hinputs, hamuts, current_function_header, locals, borrow_access, deferreds)
    }
}
/*
  def translateLetAndPoint(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    letIE: LetAndLendIE):
  (ExpressionH[KindHT]) = {
    val LetAndLendIE(localVariable, sourceExpr2, targetOwnership, _) = letIE

    val (sourceExprHE, deferreds) =
      expressionHammer.translate(hinputs, hamuts, currentFunctionHeader, locals, sourceExpr2);
    val (sourceResultPointerTypeH) =
      typeHammer.translateCoord(hinputs, hamuts, sourceExpr2.result)

    val borrowAccess =
      localVariable match {
        case ReferenceLocalVariableI(varId, variability, type2) => {
          translateMundaneLetAndPoint(
            hinputs, hamuts, currentFunctionHeader, locals, sourceExpr2, sourceExprHE, sourceResultPointerTypeH, letIE, varId, variability)
        }
        case AddressibleLocalVariableI(varId, variability, reference) => {
          translateAddressibleLetAndPoint(
            hinputs, hamuts, currentFunctionHeader, locals, sourceExpr2, sourceExprHE, sourceResultPointerTypeH, letIE, varId, variability, reference)
        }
      }

    expressionHammer.translateDeferreds(
      hinputs, hamuts, currentFunctionHeader, locals, borrowAccess, deferreds)
  }
*/

// mig: fn translate_addressible_let
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub(crate) fn translate_addressible_let(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        source_expr_he: ExpressionH<'s, 'h>,
        source_result_pointer_type_h: CoordH<'s, 'h>,
        var_id: &'i IVarNameI<'s, 'i, cI>,
        variability: VariabilityI,
        reference: CoordI<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        panic!("Unimplemented: translate_addressible_let");
    }
}
/*
  private def translateAddressibleLet(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    sourceExprHE: ExpressionH[KindHT],
    sourceResultPointerTypeH: CoordH[KindHT],
    varId: IVarNameI[cI],
    variability: VariabilityI,
    reference: CoordI[cI]):
  ExpressionH[KindHT] = {
    val (boxStructRefH) =
      structHammer.makeBox(hinputs, hamuts, variability, reference, sourceResultPointerTypeH)
    val expectedLocalBoxType = CoordH(OwnH, YonderH, boxStructRefH)

    val varIdNameH = nameHammer.translateFullName(hinputs, hamuts, INameI.addStep(currentFunctionHeader.id, varId))
    val local =
      locals.addTypingPassLocal(
        varId, varIdNameH, Conversions.evaluateVariability(variability), expectedLocalBoxType)

    StackifyH(
      NewStructH(
        Vector(sourceExprHE),
        hamuts.structDefs.find(_.getRef == boxStructRefH).get.members.map(_.name),
        expectedLocalBoxType),
      local,
      Some(nameHammer.translateFullName(hinputs, hamuts, INameI.addStep(currentFunctionHeader.id, varId))))
  }
*/

// mig: fn translate_addressible_restackify
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub(crate) fn translate_addressible_restackify(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        source_expr_he: ExpressionH<'s, 'h>,
        source_result_pointer_type_h: CoordH<'s, 'h>,
        var_id: &'i IVarNameI<'s, 'i, cI>,
        variability: VariabilityI,
        reference: CoordI<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        panic!("Unimplemented: translate_addressible_restackify");
    }
}
/*
  private def translateAddressibleRestackify(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    sourceExprHE: ExpressionH[KindHT],
    sourceResultPointerTypeH: CoordH[KindHT],
    varId: IVarNameI[cI],
    variability: VariabilityI,
    reference: CoordI[cI]):
  ExpressionH[KindHT] = {
    val (boxStructRefH) =
      structHammer.makeBox(hinputs, hamuts, variability, reference, sourceResultPointerTypeH)
    val expectedLocalBoxType = CoordH(OwnH, YonderH, boxStructRefH)

    locals.markRestackified(varId)

    val local = vassertSome(locals.get(varId))

    RestackifyH(
      NewStructH(
        Vector(sourceExprHE),
        hamuts.structDefs.find(_.getRef == boxStructRefH).get.members.map(_.name),
        expectedLocalBoxType),
      local,
      Some(nameHammer.translateFullName(hinputs, hamuts, INameI.addStep(currentFunctionHeader.id, varId))))
  }
*/

// mig: fn translate_addressible_let_and_point
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub(crate) fn translate_addressible_let_and_point(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        source_expr2: ReferenceExpressionIE<'s, 'i, cI>,
        source_expr_he: ExpressionH<'s, 'h>,
        source_result_pointer_type_h: CoordH<'s, 'h>,
        let_ie: &LetAndLendIE<'s, 'i, cI>,
        var_id: &'i IVarNameI<'s, 'i, cI>,
        variability: VariabilityI,
        reference: CoordI<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        panic!("Unimplemented: translate_addressible_let_and_point");
    }
}
/*
  private def translateAddressibleLetAndPoint(
    hinputs: HinputsI,
    hamuts: HamutsBox,
      currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    sourceExpr2: ReferenceExpressionIE,
    sourceExprHE: ExpressionH[KindHT],
    sourceResultPointerTypeH: CoordH[KindHT],
    letIE: LetAndLendIE,
    varId: IVarNameI[cI],
    variability: VariabilityI,
    reference: CoordI[cI]):
  (ExpressionH[KindHT]) = {
    val stackifyH =
      translateAddressibleLet(
        hinputs, hamuts, currentFunctionHeader, locals, sourceExprHE, sourceResultPointerTypeH, varId, variability, reference)
    val (borrowAccess, Vector()) =
      loadHammer.translateAddressibleLocalLoad(
        hinputs,
        hamuts,
        currentFunctionHeader,
        locals,
        varId,
        variability,
        sourceExpr2.result,
        letIE.result.ownership)
    ConsecutorH(Vector(stackifyH, borrowAccess))
  }
*/

// mig: fn translate_mundane_let
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub(crate) fn translate_mundane_let(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        source_expr_he: ExpressionH<'s, 'h>,
        source_result_pointer_type_h: CoordH<'s, 'h>,
        var_id: &'i IVarNameI<'s, 'i, cI>,
        variability: VariabilityI,
    ) -> &'h StackifyH<'s, 'h>
    {
        match source_expr_he.result_type().kind {
            crate::final_ast::types::KindHT::NeverHT(_) => panic!("translate_mundane_let: source NeverHT (vwat)"),
            _ => {}
        }
        let var_id_full = crate::instantiating::ast::names::add_step(&current_function_header.id, crate::instantiating::ast::names::INameI::from(*var_id));
        let var_id_name_h = self.translate_full_name(hinputs, hamuts, &var_id_full);
        let local_index = locals.add_typing_pass_local(var_id, var_id_name_h, crate::simplifying::conversions::evaluate_variability(variability), source_result_pointer_type_h);
        let stack_node = self.interner.alloc(crate::final_ast::instructions::StackifyH {
            source_expr: source_expr_he,
            local: local_index,
            name: Some(self.translate_full_name(hinputs, hamuts, &var_id_full)),
        });
        stack_node
    }
}
/*
  private def translateMundaneLet(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    sourceExprHE: ExpressionH[KindHT],
    sourceResultPointerTypeH: CoordH[KindHT],
    varId: IVarNameI[cI],
    variability: VariabilityI):
  StackifyH = {
    sourceExprHE.resultType.kind match {
      case NeverHT(_) => vwat()
      case _ =>
    }
    val varIdNameH = nameHammer.translateFullName(hinputs, hamuts, INameI.addStep(currentFunctionHeader.id, varId))
    val localIndex =
      locals.addTypingPassLocal(varId, varIdNameH, Conversions.evaluateVariability(variability), sourceResultPointerTypeH)
    val stackNode =
      StackifyH(
        sourceExprHE,
        localIndex,
        Some(nameHammer.translateFullName(hinputs, hamuts, INameI.addStep(currentFunctionHeader.id, varId))))
    stackNode
  }
*/

// mig: fn translate_mundane_restackify
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub(crate) fn translate_mundane_restackify(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        source_expr_he: ExpressionH<'s, 'h>,
        var_id: &'i IVarNameI<'s, 'i, cI>,
    ) -> &'h RestackifyH<'s, 'h>
    {
        panic!("Unimplemented: translate_mundane_restackify");
    }
}
/*
  private def translateMundaneRestackify(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    sourceExprHE: ExpressionH[KindHT],
    varId: IVarNameI[cI]):
  RestackifyH = {
    locals.markRestackified(varId)

    sourceExprHE.resultType.kind match {
      case NeverHT(_) => vwat()
      case _ =>
    }
    val local = vassertSome(locals.get(varId))
    val stackNode =
      RestackifyH(
        sourceExprHE,
        local,
        Some(nameHammer.translateFullName(hinputs, hamuts, INameI.addStep(currentFunctionHeader.id, varId))))
    stackNode
  }
*/

// mig: fn translate_mundane_let_and_point
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub(crate) fn translate_mundane_let_and_point(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        source_expr2: ReferenceExpressionIE<'s, 'i, cI>,
        source_expr_he: ExpressionH<'s, 'h>,
        source_result_pointer_type_h: CoordH<'s, 'h>,
        let_ie: &LetAndLendIE<'s, 'i, cI>,
        var_id: &'i IVarNameI<'s, 'i, cI>,
        variability: VariabilityI,
    ) -> ExpressionH<'s, 'h>
    {
        let stackify_h = self.translate_mundane_let(hinputs, hamuts, current_function_header, locals, source_expr_he, source_result_pointer_type_h, var_id, variability);
        let (borrow_access, borrow_deferreds) =
            self.translate_mundane_local_load(hinputs, hamuts, current_function_header, locals, var_id, source_expr2.result(), let_ie.result.ownership);
        assert!(borrow_deferreds.is_empty());
        let _ = borrow_deferreds;
        ExpressionH::ConsecutorH(self.interner.alloc(crate::final_ast::instructions::ConsecutorH {
            exprs: self.interner.bump().alloc_slice_copy(&[ExpressionH::StackifyH(stackify_h), borrow_access]),
        }))
    }
}
/*
    private def translateMundaneLetAndPoint(
      hinputs: HinputsI,
      hamuts: HamutsBox,
      currentFunctionHeader: FunctionHeaderI,
      locals: LocalsBox,
      sourceExpr2: ReferenceExpressionIE,
      sourceExprHE: ExpressionH[KindHT],
      sourceResultPointerTypeH: CoordH[KindHT],
      letIE: LetAndLendIE,
      varId: IVarNameI[cI],
      variability: VariabilityI):
    ExpressionH[KindHT] = {
    val stackifyH =
      translateMundaneLet(
        hinputs,
        hamuts,
        currentFunctionHeader,
        locals,
        sourceExprHE,
        sourceResultPointerTypeH,
        varId,
        variability)

    val (borrowAccess, Vector()) =
      loadHammer.translateMundaneLocalLoad(
        hinputs,
        hamuts,
        currentFunctionHeader,
        locals,
        varId,
        sourceExpr2.result,
        letIE.result.ownership)

      ConsecutorH(Vector(stackifyH, borrowAccess))
  }
*/

// mig: fn translate_unlet
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_unlet(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        unlet2: &UnletIE<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        let var_name = unlet2.variable.name();
        let local = match locals.get_by_var_name(&var_name) {
            None => panic!("Unletting an unknown variable: {:?}", var_name),
            Some(local) => local,
        };
        match unlet2.variable {
            crate::instantiating::ast::ast::ILocalVariableI::ReferenceLocalVariableI(rlv) => {
                let local_type2 = rlv.collapsed_coord;
                let _local_type_h = self.translate_coord(hinputs, hamuts, local_type2);
                let unstackify_node = ExpressionH::UnstackifyH(self.interner.alloc(crate::final_ast::instructions::UnstackifyH { local }));
                locals.mark_unstackified_by_var_name(&rlv.name);
                unstackify_node
            }
            crate::instantiating::ast::ast::ILocalVariableI::AddressibleLocalVariableI(_) => {
                panic!("translate_unlet: AddressibleLocalVariableI branch")
            }
        }
    }
}
/*
  def translateUnlet(
      hinputs: HinputsI,
      hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
      locals: LocalsBox,
      unlet2: UnletIE):
  (ExpressionH[KindHT]) = {
    val local =
      locals.get(unlet2.variable.name) match {
        case None => {
          vfail("Unletting an unknown variable: " + unlet2.variable.name)
        }
        case Some(local) => local
      }

    unlet2.variable match {
      case ReferenceLocalVariableI(varId, _, localType2) => {
        val localTypeH = typeHammer.translateCoord(hinputs, hamuts, localType2)
        val unstackifyNode = UnstackifyH(local)
        locals.markUnstackified(varId)
        unstackifyNode
      }
      case AddressibleLocalVariableI(varId, variability, innerType2) => {
        val innerTypeH = typeHammer.translateCoord(hinputs, hamuts, innerType2)
        val structRefH =
          structHammer.makeBox(hinputs, hamuts, variability, innerType2, innerTypeH)

        val unstackifyBoxNode = UnstackifyH(local)
        locals.markUnstackified(varId)

        val innerLocal = locals.addHammerLocal(innerTypeH, Conversions.evaluateVariability(variability))

        val desH =
          DestroyH(
            unstackifyBoxNode.expectStructAccess(),
            Vector(innerTypeH),
            Vector(innerLocal))
        locals.markUnstackified(innerLocal.id)

        val unstackifyContentsNode = UnstackifyH(innerLocal)

        ConsecutorH(Vector(desH, unstackifyContentsNode))
      }
    }
  }
*/

// mig: fn translate_destructure_static_sized_array
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_destructure_static_sized_array(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        des2: &DestroyStaticSizedArrayIntoLocalsIE<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        panic!("Unimplemented: translate_destructure_static_sized_array");
    }
}
/*
  def translateDestructureStaticSizedArray(
    hinputs: HinputsI,
    hamuts: HamutsBox,
      currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    des2: DestroyStaticSizedArrayIntoLocalsIE
  ): ExpressionH[KindHT] = {
    val DestroyStaticSizedArrayIntoLocalsIE(sourceExpr2, arrSeqI, destinationReferenceLocalVariables) = des2

    val (sourceExprHE, sourceExprDeferreds) =
      expressionHammer.translate(hinputs, hamuts, currentFunctionHeader, locals, sourceExpr2);

    vassert(destinationReferenceLocalVariables.size == arrSeqI.size)

    // Destructure2 will immediately destroy any addressible references inside it
    // (see Destructure2 comments).
    // In the post-addressible world with all our boxes and stuff, an addressible
    // reference member is actually a borrow reference to a box.
    // Destructure2's destroying of addressible references translates to hammer
    // unborrowing the references to boxes.
    // However, the typingpass only supplied variables for the reference members,
    // so we need to introduce our own local variables here.

    val (localTypes, localIndices) =
      destinationReferenceLocalVariables
        .map(destinationReferenceLocalVariable => {
          val (memberRefTypeH) =
            typeHammer.translateCoord(hinputs, hamuts, arrSeqI.elementType.coord)
          val varIdNameH =
            nameHammer.translateFullName(
              hinputs, hamuts, INameI.addStep(currentFunctionHeader.id, destinationReferenceLocalVariable.name))
          val localIndex =
            locals.addTypingPassLocal(
              destinationReferenceLocalVariable.name,
              varIdNameH,
              Conversions.evaluateVariability(destinationReferenceLocalVariable.variability),
              memberRefTypeH)
          (memberRefTypeH, localIndex)
        })
        .unzip

    val stackNode =
        DestroyStaticSizedArrayIntoLocalsH(
          sourceExprHE.expectStaticSizedArrayAccess(),
          localTypes,
          localIndices.toVector)

    expressionHammer.translateDeferreds(
      hinputs, hamuts, currentFunctionHeader, locals, stackNode, sourceExprDeferreds)
  }
*/

// mig: fn translate_destroy
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_destroy(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        des2: &DestroyIE<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        let crate::instantiating::ast::expressions::DestroyIE { expr: source_expr2, struct_tt: struct_ti, destination_reference_variables: destination_reference_local_variables } = *des2;
        let (source_expr_he, source_expr_deferreds) =
            self.translate_expression(hinputs, hamuts, current_function_header, locals, crate::instantiating::ast::expressions::ExpressionIE::Reference(source_expr2));
        let struct_def_t = self.lookup_struct(hinputs, hamuts, struct_ti);
        // We put Vector.empty here to make sure that we've consumed all the destination
        // reference local variables.
        let mut remaining_destination_reference_local_variables: Vec<&crate::instantiating::ast::ast::ReferenceLocalVariableI<'s, 'i>> = destination_reference_local_variables.iter().collect();
        let mut local_types: Vec<crate::final_ast::types::CoordH<'s, 'h>> = Vec::new();
        let mut local_indices: Vec<crate::final_ast::instructions::Local<'s, 'h>> = Vec::new();
        for member2 in struct_def_t.members.iter() {
            match member2.tyype {
                crate::instantiating::ast::citizens::IMemberTypeI::ReferenceMemberTypeI(member_ref_type2) => {
                    let destination_reference_local_variable = remaining_destination_reference_local_variables.remove(0);
                    let member_ref_type_h = self.translate_coord(hinputs, hamuts, member_ref_type2.reference);
                    let var_id_full = crate::instantiating::ast::names::add_step(&current_function_header.id, crate::instantiating::ast::names::INameI::from(destination_reference_local_variable.name));
                    let var_id_name_h = self.translate_full_name(hinputs, hamuts, &var_id_full);
                    let local_index = locals.add_typing_pass_local(
                        &destination_reference_local_variable.name,
                        var_id_name_h,
                        crate::simplifying::conversions::evaluate_variability(destination_reference_local_variable.variability),
                        member_ref_type_h);
                    local_types.push(member_ref_type_h);
                    local_indices.push(local_index);
                }
                crate::instantiating::ast::citizens::IMemberTypeI::AddressMemberTypeI(_) => {
                    panic!("translate_destroy: AddressMemberTypeI arm not yet migrated");
                }
            }
        }
        assert!(remaining_destination_reference_local_variables.is_empty());
        let _ = &mut remaining_destination_reference_local_variables;
        let destructure_h =
            crate::final_ast::instructions::ExpressionH::DestroyH(self.interner.alloc(crate::final_ast::instructions::DestroyH {
                struct_expression: source_expr_he.expect_struct_access(),
                local_types: self.interner.bump().alloc_slice_copy(&local_types),
                local_indices: self.interner.bump().alloc_slice_copy(&local_indices),
            }));
        let unboxings_h: Vec<crate::final_ast::instructions::ExpressionH<'s, 'h>> =
            struct_def_t.members.iter().zip(local_types.iter().zip(local_indices.iter())).flat_map(|(member, (_local_type, _local))| {
                match member.tyype {
                    crate::instantiating::ast::citizens::IMemberTypeI::ReferenceMemberTypeI(_) => Vec::<crate::final_ast::instructions::ExpressionH<'s, 'h>>::new(),
                    crate::instantiating::ast::citizens::IMemberTypeI::AddressMemberTypeI(_) => panic!("translate_destroy: AddressMemberTypeI unboxing arm not yet migrated"),
                }
            }).collect();
        let mut destructure_and_unboxings: Vec<crate::final_ast::instructions::ExpressionH<'s, 'h>> = vec![destructure_h];
        destructure_and_unboxings.extend(unboxings_h);
        let destructure_and_unboxings_h = crate::final_ast::instructions::ExpressionH::ConsecutorH(self.interner.alloc(crate::final_ast::instructions::ConsecutorH {
            exprs: self.interner.bump().alloc_slice_copy(&destructure_and_unboxings),
        }));
        self.translate_deferreds(hinputs, hamuts, current_function_header, locals, destructure_and_unboxings_h, source_expr_deferreds)
    }
}
/*
Guardian: temp-disable: SPDMX — intern_struct_it_ci(StructITValI { id }) is the canonical interning idiom for going from StructIT-by-value to &StructIT — StructITValI is a parity-neutral single-field wrapper for the intern operation. Same in-tree precedent: instantiator.rs:2448, 3337, 3402; region_collapser_individual.rs:361, 736. Exception B-class (interning + arena ref). — FrontendRust/guardian-logs/request-388-1780419320750/hook-388/translate_destroy--684.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def translateDestroy(
      hinputs: HinputsI,
      hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
      locals: LocalsBox,
      des2: DestroyIE):
  ExpressionH[KindHT] = {
    val DestroyIE(sourceExpr2, structTI, destinationReferenceLocalVariables) = des2

    val (sourceExprHE, sourceExprDeferreds) =
      expressionHammer.translate(hinputs, hamuts, currentFunctionHeader, locals, sourceExpr2);

//    val structDefT = hinputs.lookupStruct(TemplataCompiler.getStructTemplate(structTT.fullName))
    val structDefT = structHammer.lookupStruct(hinputs, hamuts, structTI)

    // Destructure2 will immediately destroy any addressible references inside it
    // (see Destructure2 comments).
    // In the post-addressible world with all our boxes and stuff, an addressible
    // reference member is actually a borrow reference to a box.
    // Destructure2's destroying of addressible references translates to hammer
    // unborrowing the references to boxes.
    // However, the typingpass only supplied variables for the reference members,
    // so we need to introduce our own local variables here.

    // We put Vector.empty here to make sure that we've consumed all the destination
    // reference local variables.
    val (Vector(), localTypes, localIndices) =
      structDefT.members.foldLeft((destinationReferenceLocalVariables, Vector[CoordH[KindHT]](), Vector[Local]()))({
        case ((remainingDestinationReferenceLocalVariables, previousLocalTypes, previousLocalIndices), member2) => {
          member2 match {
            case StructMemberI(name, variability, ReferenceMemberTypeI(memberRefType2)) => {
              val destinationReferenceLocalVariable = remainingDestinationReferenceLocalVariables.head

              val (memberRefTypeH) =
                typeHammer.translateCoord(hinputs, hamuts, memberRefType2)
              val varIdNameH = nameHammer.translateFullName(hinputs, hamuts, INameI.addStep(currentFunctionHeader.id, destinationReferenceLocalVariable.name))
              val localIndex =
                locals.addTypingPassLocal(
                  destinationReferenceLocalVariable.name,
                  varIdNameH,
                  Conversions.evaluateVariability(destinationReferenceLocalVariable.variability),
                  memberRefTypeH)
              (remainingDestinationReferenceLocalVariables.tail, previousLocalTypes :+ memberRefTypeH, previousLocalIndices :+ localIndex)
            }
            // The struct might have addressibles in them, which translate to
            // borrow refs of boxes which contain things. We're moving that borrow
            // ref into a local variable. We'll then unlet the local variable, and
            // unborrow it.
            case StructMemberI(name, variability, AddressMemberTypeI(memberRefType2)) => {
              val (memberRefTypeH) =
                typeHammer.translateCoord(hinputs, hamuts, memberRefType2);
              // In the case of an addressible struct member, its variability refers to the
              // variability of the pointee variable, see structMember2
              val (boxStructRefH) =
                structHammer.makeBox(hinputs, hamuts, variability, memberRefType2, memberRefTypeH)
              // Structs only ever borrow boxes, boxes are only ever owned by the stack.
              val localBoxType = CoordH(vregionmut(MutableBorrowH), YonderH, boxStructRefH)
              val localIndex = locals.addHammerLocal(localBoxType, Final)

              (remainingDestinationReferenceLocalVariables, previousLocalTypes :+ localBoxType, previousLocalIndices :+ localIndex)
            }
          }
        }
      })

    val destructureH =
        DestroyH(
          sourceExprHE.expectStructAccess(),
          localTypes,
          localIndices.toVector)

    val unboxingsH =
      structDefT.members.zip(localTypes.zip(localIndices)).flatMap({
        case (StructMemberI(_, _, ReferenceMemberTypeI(_)), (localType, local)) => Vector.empty
        case (StructMemberI(_, _, AddressMemberTypeI(_)), (localType, local)) => {
          // localType is the box type.
          // First, unlet it, then discard the contents.
          // We discard instead of putting it into a local because address members
          // can't own, they only refer to a box owned elsewhere.
          val unstackifyNode = UnstackifyH(local)
          locals.markUnstackified(local.id)

          val discardNode = DiscardH(unstackifyNode)
          Vector(discardNode)
        }
      })

    val destructureAndUnboxings = ConsecutorH(Vector(destructureH) ++ unboxingsH)

    expressionHammer.translateDeferreds(
      hinputs, hamuts, currentFunctionHeader, locals, destructureAndUnboxings, sourceExprDeferreds)
  }
}
*/
