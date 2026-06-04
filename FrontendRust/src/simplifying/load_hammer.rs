// From Frontend/SimplifyingPass/src/dev/vale/simplifying/LoadHammer.scala
//
// Per typing-pass `Compiler` precedent, `LoadHammer` is not a Rust struct.
// Methods become `impl Hammer { ... }` blocks colocated here.

use crate::final_ast::instructions::ExpressionH;
use crate::final_ast::types::CoordH;
use crate::instantiating::ast::ast::FunctionHeaderI;
use crate::instantiating::ast::expressions::{
    AddressMemberLookupIE, ExpressionIE, LocalLookupIE, ReferenceExpressionIE, SoftLoadIE,
};
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::ast::names::IVarNameI;
use crate::instantiating::ast::types::{cI, CoordI, OwnershipI, VariabilityI};
use crate::simplifying::hamuts::Hamuts;
use crate::simplifying::hammer::{Hammer, Locals};

/*
package dev.vale.simplifying

import dev.vale._
import dev.vale.finalast._
import dev.vale.finalast._
import dev.vale.instantiating.ast._
import dev.vale.postparsing.RegionTemplataType

class LoadHammer(
    keywords: Keywords,
    typeHammer: TypeHammer,
    nameHammer: NameHammer,
    structHammer: StructHammer,
    expressionHammer: ExpressionHammer) {
*/

// mig: fn translate_load
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_load(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        load2: &SoftLoadIE<'s, 'i, cI>,
    ) -> (ExpressionH<'s, 'h>, Vec<ExpressionIE<'s, 'i, cI>>)
    {
        use crate::instantiating::ast::expressions::{AddressExpressionIE, LocalLookupIE};
        use crate::instantiating::ast::ast::ILocalVariableI;
        let SoftLoadIE { expr: source_expr2, target_ownership, .. } = *load2;
        let (loaded_access_h, source_deferreds) = match source_expr2 {
            AddressExpressionIE::LocalLookup(LocalLookupIE { local_variable: ILocalVariableI::ReferenceLocalVariableI(r), .. }) => {
                let combined_target_ownership = target_ownership;
                self.translate_mundane_local_load(hinputs, hamuts, current_function_header, locals, &r.name, r.collapsed_coord, combined_target_ownership)
            }
            AddressExpressionIE::LocalLookup(LocalLookupIE { local_variable: ILocalVariableI::AddressibleLocalVariableI(a), .. }) => {
                let combined_target_ownership = target_ownership;
                self.translate_addressible_local_load(hinputs, hamuts, current_function_header, locals, &a.name, a.variability, a.collapsed_coord, combined_target_ownership)
            }
            AddressExpressionIE::ReferenceMemberLookup(rml) => {
                self.translate_mundane_member_load(hinputs, hamuts, current_function_header, locals, rml.struct_expr, &rml.member_name, target_ownership, rml.member_reference)
            }
            AddressExpressionIE::AddressMemberLookup(aml) => {
                self.translate_addressible_member_load(hinputs, hamuts, current_function_header, locals, aml.struct_expr, &aml.member_name, target_ownership, aml.member_reference)
            }
            AddressExpressionIE::RuntimeSizedArrayLookup(rsl) => {
                let combined_target_ownership = target_ownership;
                self.translate_mundane_runtime_sized_array_load(hinputs, hamuts, current_function_header, locals, rsl.array_expr, rsl.index_expr, combined_target_ownership, rsl.element_type)
            }
            AddressExpressionIE::StaticSizedArrayLookup(s) => {
                self.translate_mundane_static_sized_array_load(hinputs, hamuts, current_function_header, locals, s.array_expr, s.index_expr, target_ownership)
            }
        };
        (loaded_access_h, source_deferreds)
    }
}
/*
  def translateLoad(
      hinputs: HinputsI,
      hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
      locals: LocalsBox,
      load2: SoftLoadIE):
  (ExpressionH[KindHT], Vector[ExpressionI]) = {
    val SoftLoadIE(sourceExpr2, targetOwnership, _) = load2

    val (loadedAccessH, sourceDeferreds) =
      sourceExpr2 match {
        case LocalLookupIE(ReferenceLocalVariableI(varId, variability, reference), _) => {
          // DO NOI SUBMIT combine this with below
          val combinedTargetOwnership = targetOwnership
//            (targetOwnership, sourceRegion) match {
//              case (OwnI, _) => OwnI
//              case (BorrowI, RegionTemplata(true)) => MutableBorrowI
//              case (BorrowI, RegionTemplata(false)) => ImmutableBorrowI
//              case (ShareI, RegionTemplata(true)) => MutableShareI
//              case (ShareI, RegionTemplata(false)) => ImmutableShareI
//              case (WeakI, _) => vimpl()
//            }
          translateMundaneLocalLoad(hinputs, hamuts, currentFunctionHeader, locals, varId, reference, combinedTargetOwnership)
        }
        case LocalLookupIE(AddressibleLocalVariableI(varId, variability, localReference2), _) => {
          // DO NOI SUBMIT combine this with below
          val combinedTargetOwnership = vregionmut()
//            (targetOwnership, sourceRegion) match {
//              case (OwnI, _) => OwnI
//              case (BorrowI, RegionTemplata(true)) => MutableBorrowI
//              case (BorrowI, RegionTemplata(false)) => ImmutableBorrowI
//              case (ShareI, RegionTemplata(true)) => MutableShareI
//              case (ShareI, RegionTemplata(false)) => ImmutableShareI
//              case (WeakI, _) => vimpl()
//            }
          translateAddressibleLocalLoad(hinputs, hamuts, currentFunctionHeader, locals, varId, variability, localReference2, vregionmut(targetOwnership))
        }
        case ReferenceMemberLookupIE(_,structExpr2, memberName, memberType2, _) => {
//          val sourceRegion: ITemplata[RegionTemplataType] = vimpl()
          // DO NOI SUBMIT combine this with below
//          val combinedTargetOwnership =
//            (targetOwnership, sourceRegion) match {
//              case (OwnI, _) => OwnI
//              case (BorrowI, RegionTemplata(true)) => MutableBorrowI
//              case (BorrowI, RegionTemplata(false)) => ImmutableBorrowI
//              case (ShareI, RegionTemplata(true)) => MutableShareI
//              case (ShareI, RegionTemplata(false)) => ImmutableShareI
//              case (WeakI, _) => vimpl()
//            }
          translateMundaneMemberLoad(hinputs, hamuts, currentFunctionHeader, locals, structExpr2, memberType2, memberName, targetOwnership)
        }
        case AddressMemberLookupIE(structExpr2, memberName, memberType2, _) => {
//          val sourceRegion: ITemplataI[RegionTemplataType] = vimpl()
          // DO NOI SUBMIT combine this with below
          vregionmut()
//          val combinedTargetOwnership =
//            (targetOwnership, sourceRegion) match {
//              case (OwnI, _) => OwnI
//              case (BorrowI, RegionTemplata(true)) => MutableBorrowI
//              case (BorrowI, RegionTemplata(false)) => ImmutableBorrowI
//              case (ShareI, RegionTemplata(true)) => MutableShareI
//              case (ShareI, RegionTemplata(false)) => ImmutableShareI
//              case (WeakI, _) => vimpl()
//            }
          translateAddressibleMemberLoad(hinputs, hamuts, currentFunctionHeader, locals, structExpr2, memberName, memberType2, vregionmut(targetOwnership))
        }
        case RuntimeSizedArrayLookupIE(arrayExpr2, indexExpr2, _, _) => {
//          val sourceRegion: ITemplataI[RegionTemplataType] = vimpl()
          // DO NOI SUBMIT combine this with below
          val combinedTargetOwnership = targetOwnership
//            (targetOwnership, sourceRegion) match {
//              case (OwnI, _) => OwnI
//              case (BorrowI, RegionTemplata(true)) => MutableBorrowI
//              case (BorrowI, RegionTemplata(false)) => ImmutableBorrowI
//              case (ShareI, RegionTemplata(true)) => MutableShareI
//              case (ShareI, RegionTemplata(false)) => ImmutableShareI
//              case (WeakI, _) => vimpl()
//            }
          translateMundaneRuntimeSizedArrayLoad(hinputs, hamuts, currentFunctionHeader, locals, arrayExpr2, indexExpr2, combinedTargetOwnership)
        }
        case StaticSizedArrayLookupIE(_, arrayExpr2, indexExpr2, _, _) => {
//          val sourceRegion: ITemplataI[RegionTemplataType] = vimpl()
          // DO NOI SUBMIT combine this with below
//          val combinedTargetOwnership =
//            (targetOwnership, sourceRegion) match {
//              case (OwnI, _) => OwnI
//              case (BorrowI, RegionTemplata(true)) => MutableBorrowI
//              case (BorrowI, RegionTemplata(false)) => ImmutableBorrowI
//              case (ShareI, RegionTemplata(true)) => MutableShareI
//              case (ShareI, RegionTemplata(false)) => ImmutableShareI
//              case (WeakI, _) => vimpl()
//            }
          translateMundaneStaticSizedArrayLoad(
            hinputs, hamuts, currentFunctionHeader, locals, arrayExpr2, indexExpr2, targetOwnership)
        }
      }

    // Note how we return the deferreds upward, see Hammer doc for why.

    (loadedAccessH, sourceDeferreds)
  }
*/

// mig: fn translate_mundane_runtime_sized_array_load
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_mundane_runtime_sized_array_load(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        array_expr2: ReferenceExpressionIE<'s, 'i, cI>,
        index_expr2: ReferenceExpressionIE<'s, 'i, cI>,
        target_ownership_i: OwnershipI,
        result_type2: CoordI<'s, 'i, cI>,
    ) -> (ExpressionH<'s, 'h>, Vec<ExpressionIE<'s, 'i, cI>>)
    {
        let _ = result_type2;
        let target_ownership = crate::simplifying::conversions::evaluate_ownership(target_ownership_i);
        let (array_result_line, array_deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, locals, ExpressionIE::Reference(array_expr2));
        let array_access = array_result_line.expect_runtime_sized_array_access();
        let (index_expr_result_line, index_deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, locals, ExpressionIE::Reference(index_expr2));
        let index_access = index_expr_result_line.expect_int_access();
        assert!(target_ownership == crate::final_ast::types::OwnershipH::MutableBorrowH || target_ownership == crate::final_ast::types::OwnershipH::ImmutableBorrowH || target_ownership == crate::final_ast::types::OwnershipH::ImmutableShareH || target_ownership == crate::final_ast::types::OwnershipH::MutableShareH);
        let rsa = hamuts.get_runtime_sized_array(array_access.result_type().kind.expect_runtime_sized_array_ht());
        let expected_element_type = rsa.element_type;
        let location = match (target_ownership, expected_element_type.location) {
            (crate::final_ast::types::OwnershipH::ImmutableBorrowH, _) => crate::final_ast::types::LocationH::YonderH,
            (crate::final_ast::types::OwnershipH::MutableBorrowH, _) => crate::final_ast::types::LocationH::YonderH,
            (crate::final_ast::types::OwnershipH::OwnH, location) => location,
            (crate::final_ast::types::OwnershipH::MutableShareH, location) => location,
            (crate::final_ast::types::OwnershipH::ImmutableShareH, location) => location,
            _ => panic!("translate_mundane_runtime_sized_array_load: unexpected ownership"),
        };
        let result_type = crate::final_ast::types::CoordH { ownership: target_ownership, location, kind: expected_element_type.kind };
        let loaded_node_h = ExpressionH::RuntimeSizedArrayLoadH(self.interner.alloc(crate::final_ast::instructions::RuntimeSizedArrayLoadH {
            array_expression: array_access,
            index_expression: index_access,
            target_ownership,
            expected_element_type,
            result_type,
        }));
        let mut deferreds: Vec<ExpressionIE<'s, 'i, cI>> = array_deferreds;
        deferreds.extend(index_deferreds);
        (loaded_node_h, deferreds)
    }
}
/*
  private def translateMundaneRuntimeSizedArrayLoad(
      hinputs: HinputsI,
      hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
      locals: LocalsBox,
      arrayExpr2: ReferenceExpressionIE,
      indexExpr2: ReferenceExpressionIE,
      targetOwnershipI: OwnershipI,
  ): (ExpressionH[KindHT], Vector[ExpressionI]) = {
    val targetOwnership = Conversions.evaluateOwnership(targetOwnershipI)

    val (arrayResultLine, arrayDeferreds) =
      expressionHammer.translate(hinputs, hamuts, currentFunctionHeader, locals, arrayExpr2);
    val arrayAccess = arrayResultLine.expectRuntimeSizedArrayAccess()

    val (indexExprResultLine, indexDeferreds) =
      expressionHammer.translate(hinputs, hamuts, currentFunctionHeader, locals, indexExpr2);
    val indexAccess = indexExprResultLine.expectIntAccess()

    vassert(
      targetOwnership == MutableBorrowH ||
      targetOwnership == ImmutableBorrowH ||
        targetOwnership == ImmutableShareH ||
        targetOwnership == MutableShareH)

    val rsa = hamuts.getRuntimeSizedArray(arrayAccess.resultType.kind)
    val expectedElementType = rsa.elementType
    val resultType = {
      val location =
        (targetOwnership, expectedElementType.location) match {
          case (ImmutableBorrowH, _) => YonderH
          case (MutableBorrowH, _) => YonderH
          case (OwnH, location) => location
          case (MutableShareH, location) => location
          case (ImmutableShareH, location) => location
        }
      CoordH(targetOwnership, location, expectedElementType.kind)
    }

    // We're storing into a regular reference element of an array.
    val loadedNodeH =
        RuntimeSizedArrayLoadH(
          arrayAccess,
          indexAccess,
          targetOwnership,
          expectedElementType,
          resultType)

    (loadedNodeH, arrayDeferreds ++ indexDeferreds)
  }
*/

// mig: fn translate_mundane_static_sized_array_load
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_mundane_static_sized_array_load(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        array_expr2: ReferenceExpressionIE<'s, 'i, cI>,
        index_expr2: ReferenceExpressionIE<'s, 'i, cI>,
        target_ownership_i: OwnershipI,
    ) -> (ExpressionH<'s, 'h>, Vec<ExpressionIE<'s, 'i, cI>>)
    {
        let target_ownership = crate::simplifying::conversions::evaluate_ownership(target_ownership_i);
        let (array_result_line, array_deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, locals, ExpressionIE::Reference(array_expr2));
        let array_access = array_result_line.expect_static_sized_array_access();
        let (index_expr_result_line, index_deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, locals, ExpressionIE::Reference(index_expr2));
        let index_access = index_expr_result_line.expect_int_access();
        assert!(target_ownership == crate::final_ast::types::OwnershipH::MutableBorrowH || target_ownership == crate::final_ast::types::OwnershipH::ImmutableBorrowH || target_ownership == crate::final_ast::types::OwnershipH::MutableShareH || target_ownership == crate::final_ast::types::OwnershipH::ImmutableShareH);
        let ssa = hamuts.get_static_sized_array(array_access.result_type().kind.expect_static_sized_array_ht());
        let expected_element_type = ssa.element_type;
        let location = match (target_ownership, expected_element_type.location) {
            (crate::final_ast::types::OwnershipH::MutableBorrowH, _) | (crate::final_ast::types::OwnershipH::ImmutableBorrowH, _) => crate::final_ast::types::LocationH::YonderH,
            (crate::final_ast::types::OwnershipH::OwnH, location) => location,
            (crate::final_ast::types::OwnershipH::ImmutableShareH, location) | (crate::final_ast::types::OwnershipH::MutableShareH, location) => location,
            _ => panic!("translate_mundane_static_sized_array_load: unexpected ownership"),
        };
        let result_type = crate::final_ast::types::CoordH { ownership: target_ownership, location, kind: expected_element_type.kind };
        let loaded_node_h = ExpressionH::StaticSizedArrayLoadH(self.interner.alloc(crate::final_ast::instructions::StaticSizedArrayLoadH {
            array_expression: array_access,
            index_expression: index_access,
            target_ownership,
            expected_element_type,
            array_size: ssa.size,
            result_type,
        }));
        let mut combined_deferreds = array_deferreds;
        combined_deferreds.extend(index_deferreds);
        (loaded_node_h, combined_deferreds)
    }
}
/*
  private def translateMundaneStaticSizedArrayLoad(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    arrayExpr2: ReferenceExpressionIE,
    indexExpr2: ReferenceExpressionIE,
    targetOwnershipI: OwnershipI,
  ): (ExpressionH[KindHT], Vector[ExpressionI]) = {
    val targetOwnership = Conversions.evaluateOwnership(targetOwnershipI)

    val (arrayResultLine, arrayDeferreds) =
      expressionHammer.translate(hinputs, hamuts, currentFunctionHeader, locals, arrayExpr2);
    val arrayAccess = arrayResultLine.expectStaticSizedArrayAccess()

    val (indexExprResultLine, indexDeferreds) =
      expressionHammer.translate(hinputs, hamuts, currentFunctionHeader, locals, indexExpr2);
    val indexAccess = indexExprResultLine.expectIntAccess()

    vassert(
      targetOwnership == finalast.MutableBorrowH ||
        targetOwnership == finalast.ImmutableBorrowH ||
        targetOwnership == finalast.MutableShareH ||
        targetOwnership == finalast.ImmutableShareH)

    val ssa = hamuts.getStaticSizedArray(arrayAccess.resultType.kind)
    val expectedElementType = ssa.elementType
    val resultType = {
      val location =
        (targetOwnership, expectedElementType.location) match {
          case (MutableBorrowH | ImmutableBorrowH, _) => YonderH
          case (OwnH, location) => location
          case (ImmutableShareH | MutableShareH, location) => location
        }
      CoordH(targetOwnership, location, expectedElementType.kind)
    }

    // We're storing into a regular reference element of an array.
    val loadedNodeH =
        StaticSizedArrayLoadH(
          arrayAccess,
          indexAccess,
          targetOwnership,
          expectedElementType,
          ssa.size,
          resultType)

    (loadedNodeH, arrayDeferreds ++ indexDeferreds)
  }
*/

// mig: fn translate_addressible_member_load
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_addressible_member_load(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        struct_expr2: ReferenceExpressionIE<'s, 'i, cI>,
        member_name: &'i IVarNameI<'s, 'i, cI>,
        target_ownership_i: OwnershipI,
        expected_member_coord: CoordI<'s, 'i, cI>,
    ) -> (ExpressionH<'s, 'h>, Vec<ExpressionIE<'s, 'i, cI>>)
    {
        let (struct_result_line, struct_deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, locals, ExpressionIE::Reference(struct_expr2));
        let struct_it = match struct_expr2.result().kind {
            crate::instantiating::ast::types::KindIT::StructIT(sr) => sr,
            _ => panic!("translate_addressible_member_load: non-struct kind"),
        };
        let struct_def_i = self.lookup_struct(hinputs, hamuts, *struct_it);
        let member_index = struct_def_i.members.iter().position(|m| m.name == *member_name).expect("member not found") as i32;
        assert!(member_index >= 0);
        let member2 = &struct_def_i.members[member_index as usize];
        let variability = member2.variability;
        let boxed_type2 = member2.tyype.expect_address_member().reference;
        let boxed_type_h = self.translate_coord(hinputs, hamuts, boxed_type2);
        let box_struct_ref_h = self.make_box(hinputs, hamuts, variability, boxed_type2, boxed_type_h);
        let box_in_struct_coord = crate::final_ast::types::CoordH {
            ownership: crate::final_ast::types::OwnershipH::MutableBorrowH,
            location: crate::final_ast::types::LocationH::YonderH,
            kind: crate::final_ast::types::KindHT::StructHT(box_struct_ref_h),
        };
        let var_full_name_h = self.translate_full_name(hinputs, hamuts, &crate::instantiating::ast::names::add_step(&current_function_header.id, crate::instantiating::ast::names::INameI::from(*member_name)));
        let load_box_node = ExpressionH::MemberLoadH(self.interner.alloc(crate::final_ast::instructions::MemberLoadH {
            struct_expression: struct_result_line.expect_struct_access(),
            member_index,
            expected_member_type: box_in_struct_coord,
            result_type: box_in_struct_coord,
            member_name: var_full_name_h,
        }));
        let target_ownership = crate::simplifying::conversions::evaluate_ownership(target_ownership_i);
        let load_result_type = crate::final_ast::types::CoordH {
            ownership: target_ownership,
            location: boxed_type_h.location,
            kind: boxed_type_h.kind,
        };
        let loaded_node_h = ExpressionH::MemberLoadH(self.interner.alloc(crate::final_ast::instructions::MemberLoadH {
            struct_expression: load_box_node.expect_struct_access(),
            member_index: crate::simplifying::let_hammer::BOX_MEMBER_INDEX,
            expected_member_type: boxed_type_h,
            result_type: load_result_type,
            member_name: self.add_step(hamuts, box_struct_ref_h.id, self.keywords.box_member_name),
        }));
        (loaded_node_h, struct_deferreds)
    }
}
/*
  private def translateAddressibleMemberLoad(
      hinputs: HinputsI,
      hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
      locals: LocalsBox,
      structExpr2: ReferenceExpressionIE,
      memberName: IVarNameI[cI],
      expectedType2: CoordI[cI],
      targetOwnershipI: OwnershipI,
  ): (ExpressionH[KindHT], Vector[ExpressionI]) = {
    val (structResultLine, structDeferreds) =
      expressionHammer.translate(hinputs, hamuts, currentFunctionHeader, locals, structExpr2);

    val structIT =
      structExpr2.result.kind match {
        case sr @ StructIT(_) => sr
//        case TupleIT(_, sr) => sr
//        case PackIT(_, sr) => sr
      }
    val structDefI = structHammer.lookupStruct(hinputs, hamuts, structIT)
    val memberIndex = structDefI.members.indexWhere(_.name == memberName)
    vassert(memberIndex >= 0)
    val member2 =
      structDefI.members(memberIndex) match {
        case n @ StructMemberI(name, variability, tyype) => n
      }

    val variability = member2.variability

    val boxedType2 = member2.tyype.expectAddressMember().reference

    val (boxedTypeH) =
      typeHammer.translateCoord(hinputs, hamuts, boxedType2);

    val (boxStructRefH) =
      structHammer.makeBox(hinputs, hamuts, variability, boxedType2, boxedTypeH)

    val boxInStructCoord = CoordH(vregionmut(MutableBorrowH), YonderH, boxStructRefH)

    // We're storing into a struct's member that is a box. The stack is also
    // pointing at this box. First, get the box, then mutate what's inside.
    var varFullNameH =
    nameHammer.translateFullName(
      hinputs, hamuts, INameI.addStep(currentFunctionHeader.id, memberName))
    val loadBoxNode =
        MemberLoadH(
          structResultLine.expectStructAccess(),
          memberIndex,
          boxInStructCoord,
          boxInStructCoord,
          varFullNameH)

    val targetOwnership = Conversions.evaluateOwnership(targetOwnershipI)
    val loadResultType =
      CoordH(
        targetOwnership,
        boxedTypeH.location,
        boxedTypeH.kind)
    val loadedNodeH =
        MemberLoadH(
          loadBoxNode.expectStructAccess(),
          LetHammer.BOX_MEMBER_INDEX,
          boxedTypeH,
          loadResultType,
          nameHammer.addStep(hamuts, boxStructRefH.id, keywords.BOX_MEMBER_NAME.str))
    (loadedNodeH, structDeferreds)
  }
*/

// mig: fn translate_mundane_member_load
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_mundane_member_load(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        struct_expr2: ReferenceExpressionIE<'s, 'i, cI>,
        member_name: &'i IVarNameI<'s, 'i, cI>,
        target_ownership_i: OwnershipI,
        expected_member_coord: CoordI<'s, 'i, cI>,
    ) -> (ExpressionH<'s, 'h>, Vec<ExpressionIE<'s, 'i, cI>>)
    {
        let (struct_result_line, struct_deferreds) =
            self.translate_expression(hinputs, hamuts, current_function_header, locals, ExpressionIE::Reference(struct_expr2));
        let expected_member_type_h = self.translate_coord(hinputs, hamuts, expected_member_coord);
        let struct_it = match struct_expr2.result().kind {
            crate::instantiating::ast::types::KindIT::StructIT(sr) => sr,
            _ => panic!("translate_mundane_member_load: struct_expr2.result.kind not StructIT"),
        };
        let struct_def_i = self.lookup_struct(hinputs, hamuts, *struct_it);
        let member_index = struct_def_i.members.iter().position(|m| m.name == *member_name).expect("memberIndex >= 0") as i32;
        let target_ownership = crate::simplifying::conversions::evaluate_ownership(target_ownership_i);
        let load_result_type = crate::final_ast::types::CoordH { ownership: target_ownership, location: expected_member_type_h.location, kind: expected_member_type_h.kind };
        let loaded_node = ExpressionH::MemberLoadH(self.interner.alloc(crate::final_ast::instructions::MemberLoadH {
            struct_expression: struct_result_line.expect_struct_access(),
            member_index,
            expected_member_type: expected_member_type_h,
            result_type: load_result_type,
            member_name: self.translate_full_name(hinputs, hamuts, &crate::instantiating::ast::names::add_step(&current_function_header.id, (*member_name).into())),
        }));
        (loaded_node, struct_deferreds)
    }
}
/*
  private def translateMundaneMemberLoad(
      hinputs: HinputsI,
      hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
      locals: LocalsBox,
      structExpr2: ReferenceExpressionIE,
      expectedMemberCoord: CoordI[cI],
      memberName: IVarNameI[cI],
//      resultCoord: Coord,
      targetOwnershipI: OwnershipI,
  ): (ExpressionH[KindHT], Vector[ExpressionI]) = {
    val (structResultLine, structDeferreds) =
      expressionHammer.translate(hinputs, hamuts, currentFunctionHeader, locals, structExpr2);

    val (expectedMemberTypeH) =
      typeHammer.translateCoord(hinputs, hamuts, expectedMemberCoord);
//    val (resultTypeH) =
//      typeHammer.translateReference(hinputs, hamuts, resultCoord);

    val structIT =
      structExpr2.result.kind match {
        case sr @ StructIT(_) => sr
//        case TupleIT(_, sr) => sr
//        case PackIT(_, sr) => sr
      }
    val structDefI = structHammer.lookupStruct(hinputs, hamuts, structIT)
    val memberIndex = structDefI.members.indexWhere(_.name == memberName)
    vassert(memberIndex >= 0)

    val targetOwnership = Conversions.evaluateOwnership(targetOwnershipI)
    val loadResultType = CoordH(targetOwnership, expectedMemberTypeH.location, expectedMemberTypeH.kind)

    // We're loading from a regular reference member of a struct.
    val loadedNode =
        MemberLoadH(
          structResultLine.expectStructAccess(),
          memberIndex,
          expectedMemberTypeH,
          loadResultType,
          nameHammer.translateFullName(hinputs, hamuts, INameI.addStep(currentFunctionHeader.id, memberName)))
    (loadedNode, structDeferreds)
  }
*/

// mig: fn translate_addressible_local_load
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_addressible_local_load(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        var_id: &'i IVarNameI<'s, 'i, cI>,
        variability: VariabilityI,
        local_reference2: CoordI<'s, 'i, cI>,
        target_ownership_i: OwnershipI,
    ) -> (ExpressionH<'s, 'h>, Vec<ExpressionIE<'s, 'i, cI>>)
    {
        let local = locals.get_by_var_name(var_id).unwrap();
        assert!(!locals.unstackified_vars.contains(&local.id));
        let local_type_h = self.translate_coord(hinputs, hamuts, local_reference2);
        let box_struct_ref_h = self.make_box(hinputs, hamuts, variability, local_reference2, local_type_h);
        assert!(local.type_h.kind == crate::final_ast::types::KindHT::StructHT(box_struct_ref_h));
        let var_name_h = self.translate_full_name(hinputs, hamuts, &crate::instantiating::ast::names::add_step(&current_function_header.id, crate::instantiating::ast::names::INameI::from(*var_id)));
        let load_box_node = ExpressionH::LocalLoadH(self.interner.alloc(crate::final_ast::instructions::LocalLoadH {
            local,
            target_ownership: crate::final_ast::types::OwnershipH::MutableBorrowH,
            local_name: var_name_h,
        }));
        let target_ownership = crate::simplifying::conversions::evaluate_ownership(target_ownership_i);
        let load_result_type = crate::final_ast::types::CoordH {
            ownership: target_ownership,
            location: local_type_h.location,
            kind: local_type_h.kind,
        };
        let loaded_node = ExpressionH::MemberLoadH(self.interner.alloc(crate::final_ast::instructions::MemberLoadH {
            struct_expression: load_box_node.expect_struct_access(),
            member_index: crate::simplifying::let_hammer::BOX_MEMBER_INDEX,
            expected_member_type: local_type_h,
            result_type: load_result_type,
            member_name: self.add_step(hamuts, box_struct_ref_h.id, self.keywords.box_member_name),
        }));
        (loaded_node, Vec::new())
    }
}
/*
  def translateAddressibleLocalLoad(
      hinputs: HinputsI,
      hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
      locals: LocalsBox,
      varId: IVarNameI[cI],
      variability: VariabilityI,
      localReference2: CoordI[cI],
      targetOwnershipI: OwnershipI,
  ): (ExpressionH[KindHT], Vector[ExpressionI]) = {
    val local = locals.get(varId).get
    vassert(!locals.unstackifiedVars.contains(local.id))

    val (localTypeH) =
      typeHammer.translateCoord(hinputs, hamuts, localReference2);
    val (boxStructRefH) =
      structHammer.makeBox(hinputs, hamuts, variability, localReference2, localTypeH)
    vassert(local.typeH.kind == boxStructRefH)

    // This means we're trying to load from a local variable that holds a box.
    // We need to load the box, then mutate its contents.
    val varNameH =
    nameHammer.translateFullName(
      hinputs, hamuts, INameI.addStep(currentFunctionHeader.id, varId))
    val loadBoxNode =
        LocalLoadH(
          local,
          vregionmut(MutableBorrowH),
          varNameH)

    val targetOwnership = Conversions.evaluateOwnership(targetOwnershipI)
    val loadResultType = CoordH(targetOwnership, localTypeH.location, localTypeH.kind)

    val loadedNode =
        MemberLoadH(
          loadBoxNode.expectStructAccess(),
          LetHammer.BOX_MEMBER_INDEX,
          localTypeH,
          loadResultType,
          nameHammer.addStep(hamuts, boxStructRefH.id, keywords.BOX_MEMBER_NAME.str))
    (loadedNode, Vector.empty)
  }
*/

// mig: fn translate_mundane_local_load
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_mundane_local_load(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        var_id: &'i IVarNameI<'s, 'i, cI>,
        local_type: CoordI<'s, 'i, cI>,
        target_ownership_i: OwnershipI,
    ) -> (ExpressionH<'s, 'h>, Vec<ExpressionIE<'s, 'i, cI>>)
    {
        let target_ownership = crate::simplifying::conversions::evaluate_ownership(target_ownership_i);
        let local = locals.get_by_var_name(var_id).expect("wot");
        assert!(!locals.unstackified_vars.contains(&local.id));
        let loaded_node = ExpressionH::LocalLoadH(self.interner.alloc(crate::final_ast::instructions::LocalLoadH {
            local,
            target_ownership,
            local_name: self.translate_full_name(hinputs, hamuts, &crate::instantiating::ast::names::add_step(&current_function_header.id, (*var_id).into())),
        }));
        (loaded_node, Vec::new())
    }
}
/*
  def translateMundaneLocalLoad(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    varId: IVarNameI[cI],
    localType: CoordI[cI],
    targetOwnershipI: OwnershipI,
  ): (ExpressionH[KindHT], Vector[ExpressionI]) = {
    val targetOwnership = Conversions.evaluateOwnership(targetOwnershipI)

    val local = locals.get(varId) match {
      case Some(x) => x
      case None => {
        vfail("wot")
      }
    }
    vassert(!locals.unstackifiedVars.contains(local.id))

//    val (expectedTypeH) =
//      typeHammer.translateCoord(hinputs, hamuts, expectedType2);
//    vassert(localType == local.typeH)

    val loadedNode =
        LocalLoadH(
          local,
          targetOwnership,
          nameHammer.translateFullName(
            hinputs, hamuts, INameI.addStep(currentFunctionHeader.id, varId)))
    (loadedNode, Vector.empty)
  }
*/

// mig: fn translate_local_address
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_local_address(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        lookup2: &LocalLookupIE<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        let local_var = lookup2.local_variable;
        let local = locals.get_by_var_name(&local_var.name()).unwrap();
        assert!(!locals.unstackified_vars.contains(&local.id));
        let _box_struct_ref_h = self.make_box(hinputs, hamuts, local_var.variability(), local_var.collapsed_coord(), local.type_h);
        let var_id_full = crate::instantiating::ast::names::add_step(&current_function_header.id, crate::instantiating::ast::names::INameI::from(local_var.name()));
        ExpressionH::LocalLoadH(self.interner.alloc(crate::final_ast::instructions::LocalLoadH {
            local,
            target_ownership: crate::final_ast::types::OwnershipH::MutableBorrowH,
            local_name: self.translate_full_name(hinputs, hamuts, &var_id_full),
        }))
    }
}
/*
  def translateLocalAddress(
      hinputs: HinputsI,
      hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
      locals: LocalsBox,
      lookup2: LocalLookupIE):
  (ExpressionH[KindHT]) = {
    val LocalLookupIE(localVar, _) = lookup2;
    vregionmut()

    val local = locals.get(localVar.name).get
    vassert(!locals.unstackifiedVars.contains(local.id))
    val (boxStructRefH) =
      structHammer.makeBox(hinputs, hamuts, localVar.variability, localVar.collapsedCoord, local.typeH)

    // This means we're trying to load from a local variable that holds a box.
    // We need to load the box, then mutate its contents.
    val loadBoxNode =
      LocalLoadH(
        local,
        vregionmut(MutableBorrowH),
        nameHammer.translateFullName(hinputs, hamuts, INameI.addStep(currentFunctionHeader.id, localVar.name)))
    loadBoxNode
  }
*/

// mig: fn translate_member_address
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_member_address(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        lookup2: &AddressMemberLookupIE<'s, 'i, cI>,
    ) -> (ExpressionH<'s, 'h>, Vec<ExpressionIE<'s, 'i, cI>>)
    {
        let struct_expr2 = lookup2.struct_expr;
        let member_name = lookup2.member_name;
        let (struct_result_line, struct_deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, locals, ExpressionIE::Reference(struct_expr2));
        let struct_it = match struct_expr2.result().kind {
            crate::instantiating::ast::types::KindIT::StructIT(sr) => sr,
            _ => panic!("translate_member_address: non-struct kind"),
        };
        let struct_def_i = self.lookup_struct(hinputs, hamuts, *struct_it);
        let member_index = struct_def_i.members.iter().position(|m| m.name == member_name).expect("member not found") as i32;
        assert!(member_index >= 0);
        let member2 = &struct_def_i.members[member_index as usize];
        let variability = member2.variability;
        assert!(variability == crate::instantiating::ast::types::VariabilityI::Varying, "Expected varying for member {:?}", member_name);
        let boxed_type2 = member2.tyype.expect_address_member().reference;
        let boxed_type_h = self.translate_coord(hinputs, hamuts, boxed_type2);
        let box_struct_ref_h = self.make_box(hinputs, hamuts, variability, boxed_type2, boxed_type_h);
        let expected_struct_box_member_type = crate::final_ast::types::CoordH {
            ownership: crate::final_ast::types::OwnershipH::MutableBorrowH,
            location: crate::final_ast::types::LocationH::YonderH,
            kind: crate::final_ast::types::KindHT::StructHT(box_struct_ref_h),
        };
        let load_result_type = crate::final_ast::types::CoordH {
            ownership: crate::final_ast::types::OwnershipH::MutableBorrowH,
            location: crate::final_ast::types::LocationH::YonderH,
            kind: crate::final_ast::types::KindHT::StructHT(box_struct_ref_h),
        };
        let load_box_node = ExpressionH::MemberLoadH(self.interner.alloc(crate::final_ast::instructions::MemberLoadH {
            struct_expression: struct_result_line.expect_struct_access(),
            member_index,
            expected_member_type: expected_struct_box_member_type,
            result_type: load_result_type,
            member_name: self.translate_full_name(hinputs, hamuts, &crate::instantiating::ast::names::add_step(&current_function_header.id, crate::instantiating::ast::names::INameI::from(member_name))),
        }));
        (load_box_node, struct_deferreds)
    }
}
/*
  // In this, we're basically taking an addressible lookup, in other words,
  // a reference to a box.
  def translateMemberAddress(
      hinputs: HinputsI,
      hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
      locals: LocalsBox,
      lookup2: AddressMemberLookupIE):
  (ExpressionH[KindHT], Vector[ExpressionI]) = {
    val AddressMemberLookupIE(structExpr2, memberName, resultType2, _) = lookup2;

    val (structResultLine, structDeferreds) =
      expressionHammer.translate(hinputs, hamuts, currentFunctionHeader, locals, structExpr2);

    val structIT =
      structExpr2.result.kind match {
        case sr @ StructIT(_) => sr
//        case TupleIT(_, sr) => sr
//        case PackIT(_, sr) => sr
      }
    val structDefI = structHammer.lookupStruct(hinputs, hamuts, structIT)
    val memberIndex = structDefI.members.indexWhere(_.name == memberName)
    vassert(memberIndex >= 0)
    val member2 =
      structDefI.members(memberIndex) match {
        case n @ StructMemberI(name, variability, tyype) => n
      }

    val variability = member2.variability
    vassert(variability == VaryingI, "Expected varying for member " + memberName) // curious

    val boxedType2 = member2.tyype.expectAddressMember().reference

    val (boxedTypeH) =
      typeHammer.translateCoord(hinputs, hamuts, boxedType2);

    val (boxStructRefH) =
      structHammer.makeBox(hinputs, hamuts, variability, boxedType2, boxedTypeH)

    // We expect a borrow because structs never own boxes, they only borrow them
    val expectedStructBoxMemberType = CoordH(vregionmut(MutableBorrowH), YonderH, boxStructRefH)

    val loadResultType =
      CoordH(
        // Boxes are either owned or borrowed. We only own boxes from locals,
        // and we're loading from a struct here, so we're getting a borrow to the
        // box from the struct.
        vregionmut(MutableBorrowH),
        YonderH,
        boxStructRefH)

    // We're storing into a struct's member that is a box. The stack is also
    // pointing at this box. First, get the box, then mutate what's inside.
    val loadBoxNode =
      MemberLoadH(
        structResultLine.expectStructAccess(),
        memberIndex,
        expectedStructBoxMemberType,
        loadResultType,
        nameHammer.translateFullName(hinputs, hamuts, INameI.addStep(currentFunctionHeader.id, memberName)))

    (loadBoxNode, structDeferreds)
  }
*/

// mig: fn get_borrowed_location (free helper — no &self needed)
pub fn get_borrowed_location<'s, 'h>(member_type: CoordH<'s, 'h>) -> crate::final_ast::types::LocationH
where 's: 'h,
{
    panic!("Unimplemented: get_borrowed_location");
}
/*
  def getBorrowedLocation(memberType: CoordH[KindHT]) = {
    (memberType.ownership, memberType.location) match {
      case (OwnH, _) => YonderH
      case (ImmutableBorrowH | MutableBorrowH, _) => YonderH
      case (MutableShareH | ImmutableShareH, location) => location
    }
  }
}
*/
