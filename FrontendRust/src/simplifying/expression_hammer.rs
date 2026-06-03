// From Frontend/SimplifyingPass/src/dev/vale/simplifying/ExpressionHammer.scala
//
// Per typing-pass `Compiler` precedent, `ExpressionHammer` is not a Rust struct.
// Methods become `impl Hammer { ... }` blocks colocated here.

use crate::final_ast::instructions::{ExpressionH, WhileH};
use crate::instantiating::ast::ast::{FunctionHeaderI, PrototypeI};
use crate::instantiating::ast::expressions::{
    DestroyImmRuntimeSizedArrayIE, DestroyStaticSizedArrayIntoFunctionIE, ExpressionIE, IfIE,
    NewImmRuntimeSizedArrayIE, NewMutRuntimeSizedArrayIE, ReferenceExpressionIE,
    StaticArrayFromCallableIE, WhileIE,
};
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::ast::types::{cI, CoordI};
use crate::simplifying::hamuts::Hamuts;
use crate::simplifying::hammer::{Hammer, Locals};

/*
package dev.vale.simplifying

import dev.vale.finalast._
import dev.vale._
import dev.vale.finalast._
import dev.vale.instantiating.ast._

import scala.collection.immutable.Map

class ExpressionHammer(
    keywords: Keywords,
    typeHammer: TypeHammer,
    nameHammer: NameHammer,
    structHammer: StructHammer,
    functionHammer: FunctionHammer) {
  val blockHammer = new BlockHammer(this, typeHammer)
  val loadHammer = new LoadHammer(keywords, typeHammer, nameHammer, structHammer, this)
  val letHammer = new LetHammer(typeHammer, nameHammer, structHammer, this, loadHammer)
  val mutateHammer = new MutateHammer(keywords, typeHammer, nameHammer, structHammer, this)
*/

// mig: fn translate_expression (Scala `ExpressionHammer.translate` — disambiguated
// from `Hammer.translate` per overload-suffix pattern, since both methods now
// live on the same `impl Hammer` per typing-pass collapse.)
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_expression(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        expr2: ExpressionIE<'s, 'i, cI>,
    ) -> (ExpressionH<'s, 'h>, Vec<ExpressionIE<'s, 'i, cI>>)
    {
        use crate::instantiating::ast::expressions::ReferenceExpressionIE as RE;
        match expr2 {
            ExpressionIE::Reference(r) => match r {
                RE::ConstantInt(c) => {
                    (ExpressionH::ConstantIntH(self.interner.alloc(crate::final_ast::instructions::ConstantIntH { value: c.value, bits: c.bits })), Vec::new())
                }
                RE::VoidLiteral(_) => {
                    let construct_h = ExpressionH::ConstantVoidH(self.interner.alloc(crate::final_ast::instructions::ConstantVoidH));
                    (construct_h, Vec::new())
                }
                RE::ConstantStr(c) => (ExpressionH::ConstantStrH(self.interner.alloc(crate::final_ast::instructions::ConstantStrH { value: self.interner.bump().alloc_str(c.value), _marker: std::marker::PhantomData })), Vec::new()),
                RE::ConstantFloat(c) => {
                    (ExpressionH::ConstantF64H(self.interner.alloc(crate::final_ast::instructions::ConstantF64H { value: c.value })), Vec::new())
                }
                RE::ConstantBool(c) => {
                    (ExpressionH::ConstantBoolH(self.interner.alloc(crate::final_ast::instructions::ConstantBoolH { value: c.value })), Vec::new())
                }
                RE::LetNormal(let2) => {
                    let let_h = self.translate_let(hinputs, hamuts, current_function_header, locals, let2);
                    (let_h, Vec::new())
                }
                RE::Restackify(let2) => panic!("translate_expression: Restackify branch"),
                RE::LetAndLend(let2) => {
                    let borrow_access = self.translate_let_and_point(hinputs, hamuts, current_function_header, locals, let2);
                    (borrow_access, Vec::new())
                }
                RE::Destroy(des2) => {
                    let destroy_h = self.translate_destroy(hinputs, hamuts, current_function_header, locals, des2);
                    // Compiler destructures put things in local variables (even though hammer itself
                    // uses registers internally to make that happen).
                    // Since all the members landed in locals, we still need something to ret, so we
                    // return a void.
                    (destroy_h, Vec::new())
                }
                RE::DestroyStaticSizedArrayIntoLocals(des2) => panic!("translate_expression: DestroyStaticSizedArrayIntoLocals branch"),
                RE::Unlet(unlet2) => {
                    let value_access = self.translate_unlet(hinputs, hamuts, current_function_header, locals, unlet2);
                    (value_access, Vec::new())
                }
                RE::Mutate(mutate2) => {
                    let access = self.translate_mutate(hinputs, hamuts, current_function_header, locals, mutate2);
                    (access, Vec::new())
                }
                RE::Mutabilify(b) => panic!("translate_expression: Mutabilify branch"),
                RE::Immutabilify(b) => panic!("translate_expression: Immutabilify branch"),
                RE::Block(b) => {
                    let block_h = self.translate_block(hinputs, hamuts, current_function_header, locals, b);
                    (ExpressionH::BlockH(block_h), Vec::new())
                }
                RE::FunctionCall(call2) => {
                    let args_ie: Vec<ExpressionIE<'s, 'i, cI>> = call2.args.iter().map(|a| ExpressionIE::Reference(*a)).collect();
                    let access = self.translate_function_pointer_call(hinputs, hamuts, current_function_header, locals, &call2.callable, &args_ie, call2.result);
                    (access, Vec::new())
                }
                RE::PreCheckBorrow(p) => panic!("translate_expression: PreCheckBorrow branch"),
                RE::InterfaceFunctionCall(ic) => panic!("translate_expression: InterfaceFunctionCall branch"),
                RE::Consecutor(c) => {
                    let exprs_ie = c.exprs;
                    let mut exprs_he: Vec<ExpressionH<'s, 'h>> = Vec::new();
                    for next_ie in exprs_ie.iter() {
                        let last_is_never = match exprs_he.last().map(|e| e.result_type().kind) {
                            Some(crate::final_ast::types::KindHT::NeverHT(_)) => true,
                            _ => false,
                        };
                        if last_is_never {
                            continue;
                        }
                        let (next_he, next_deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, locals, ExpressionIE::Reference(*next_ie));
                        let next_expr_with_deferreds_he = self.translate_deferreds(hinputs, hamuts, current_function_header, locals, next_he, next_deferreds);
                        exprs_he.push(next_expr_with_deferreds_he);
                    }
                    let last_is_never = match exprs_he.last().map(|e| e.result_type().kind) {
                        Some(crate::final_ast::types::KindHT::NeverHT(_)) => {
                            return (crate::simplifying::hammer::consecrash(self.interner, locals, &exprs_he), Vec::new());
                        }
                        _ => {}
                    };
                    let _ = last_is_never;
                    (crate::simplifying::hammer::consecutive(self.interner, &exprs_he), Vec::new())
                }
                RE::ArrayLength(a) => panic!("translate_expression: ArrayLength branch"),
                RE::RuntimeSizedArrayCapacity(a) => panic!("translate_expression: RuntimeSizedArrayCapacity branch"),
                RE::ArraySize(a) => panic!("translate_expression: ArraySize branch"),
                RE::LockWeak(a) => panic!("translate_expression: LockWeak branch"),
                RE::BorrowToWeak(a) => panic!("translate_expression: BorrowToWeak branch"),
                RE::Defer(d2) => {
                    let crate::instantiating::ast::expressions::DeferIE { inner_expr, deferred_expr, result: _ } = *d2;
                    let (inner_expr_he, inner_deferreds) =
                        self.translate_expression(hinputs, hamuts, current_function_header, locals, ExpressionIE::Reference(inner_expr));
                    let mut new_deferreds: Vec<ExpressionIE<'s, 'i, cI>> = vec![ExpressionIE::Reference(deferred_expr)];
                    new_deferreds.extend(inner_deferreds);
                    (inner_expr_he, new_deferreds)
                }
                RE::If(if2) => {
                    let maybe_access = self.translate_if(hinputs, hamuts, current_function_header, locals, if2);
                    (maybe_access, Vec::new())
                }
                RE::While(while2) => {
                    let while_h = self.translate_while(hinputs, hamuts, current_function_header, locals, while2);
                    (ExpressionH::WhileH(while_h), Vec::new())
                }
                RE::Return(return_ie) => {
                    let inner_expr = return_ie.source_expr;
                    let inner_result = ExpressionIE::Reference(inner_expr).result();
                    assert!(matches!(inner_result.kind, crate::instantiating::ast::types::KindIT::NeverIT(crate::instantiating::ast::types::NeverIT { from_break: false, .. })) || inner_result == current_function_header.return_type);
                    let (inner_expr_he, inner_deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, locals, ExpressionIE::Reference(inner_expr));
                    let inner_with_deferreds = self.translate_deferreds(hinputs, hamuts, current_function_header, locals, inner_expr_he, inner_deferreds);
                    match inner_with_deferreds.result_type().kind {
                        crate::final_ast::types::KindHT::NeverHT(_) => {
                            return (inner_with_deferreds, Vec::new());
                        }
                        _ => {}
                    }
                    assert!(ExpressionIE::Reference(inner_expr).result() == current_function_header.return_type);
                    (ExpressionH::ReturnH(self.interner.alloc(crate::final_ast::instructions::ReturnH { source_expression: inner_with_deferreds })), Vec::new())
                }
                RE::Break(_) => (ExpressionH::BreakH(self.interner.alloc(crate::final_ast::instructions::BreakH)), Vec::new()),
                RE::Discard(discard_ie) => {
                    let inner_expr = discard_ie.expr;
                    let (undiscarded_inner_expr_h, inner_deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, locals, ExpressionIE::Reference(inner_expr));
                    assert!(inner_deferreds.is_empty());
                    let inner_expr_h = ExpressionH::DiscardH(self.interner.alloc(crate::final_ast::instructions::DiscardH { source_expression: undiscarded_inner_expr_h }));
                    let inner_with_deferreds_expr_h = self.translate_deferreds(hinputs, hamuts, current_function_header, locals, inner_expr_h, inner_deferreds);
                    (inner_with_deferreds_expr_h, Vec::new())
                }
                RE::Tuple(t) => {
                    let crate::instantiating::ast::expressions::TupleIE { elements: exprs, result: result_type } = *t;
                    let exprs_ie: Vec<crate::instantiating::ast::expressions::ExpressionIE<'s, 'i, cI>> = exprs.iter().map(|e| crate::instantiating::ast::expressions::ExpressionIE::Reference(*e)).collect();
                    let (results_he, deferreds) = self.translate_expressions_until_never(hinputs, hamuts, current_function_header, locals, &exprs_ie);
                    match results_he.last().map(|e| e.result_type().kind) {
                        Some(crate::final_ast::types::KindHT::NeverHT(_)) => {
                            return (crate::simplifying::hammer::consecrash(self.interner, locals, &results_he), Vec::new());
                        }
                        _ => {}
                    }
                    let result_struct_i = match result_type.kind {
                        crate::instantiating::ast::types::KindIT::StructIT(s) => s,
                        _ => panic!("Tuple: result_type.kind not StructIT"),
                    };
                    assert!(result_struct_i.id.package_coord.module.0 != "rust");
                    let underlying_struct_ref_h = self.translate_struct_i(hinputs, hamuts, result_struct_i);
                    let result_reference = self.translate_coord(hinputs, hamuts, result_type);
                    assert!(result_reference.kind == crate::final_ast::types::KindHT::StructHT(underlying_struct_ref_h));
                    let struct_def_h = *hamuts.struct_t_to_struct_def_h().get(result_struct_i).expect("structDefH not in map");
                    assert!(results_he.len() == struct_def_h.members.len());
                    let target_member_names: Vec<&'h crate::final_ast::ast::IdH<'s, 'h>> = struct_def_h.members.iter().map(|m| m.name).collect();
                    let new_struct_node = ExpressionH::NewStructH(self.interner.alloc(crate::final_ast::instructions::NewStructH {
                        source_expressions: self.interner.bump().alloc_slice_copy(&results_he),
                        target_member_names: self.interner.bump().alloc_slice_copy(&target_member_names),
                        result_type: result_reference.expect_struct_coord(),
                    }));
                    let new_struct_and_deferreds_expr_h = self.translate_deferreds(hinputs, hamuts, current_function_header, locals, new_struct_node, deferreds);
                    (new_struct_and_deferreds_expr_h, Vec::new())
                }
                RE::StaticArrayFromValues(a) => {
                    let crate::instantiating::ast::expressions::StaticArrayFromValuesIE { elements: exprs, result_reference: array_reference_2, array_type: array_type_2 } = *a;
                    let exprs_ie: Vec<crate::instantiating::ast::expressions::ExpressionIE<'s, 'i, cI>> = exprs.iter().map(|e| crate::instantiating::ast::expressions::ExpressionIE::Reference(*e)).collect();
                    let (results_he, deferreds) = self.translate_expressions_until_never(hinputs, hamuts, current_function_header, locals, &exprs_ie);
                    match results_he.last().map(|e| e.result_type().kind) {
                        Some(crate::final_ast::types::KindHT::NeverHT(_)) => {
                            return (panic!("Unimplemented: StaticArrayFromValues Never branch (Hammer.consecrash)"), Vec::new());
                        }
                        _ => {}
                    }
                    let underlying_array_h = self.translate_static_sized_array(hinputs, hamuts, array_type_2);
                    let array_reference_h = self.translate_coord(hinputs, hamuts, array_reference_2);
                    assert!(array_reference_h.kind == crate::final_ast::types::KindHT::StaticSizedArrayHT(underlying_array_h));
                    let new_struct_node = ExpressionH::NewArrayFromValuesH(self.interner.alloc(crate::final_ast::instructions::NewArrayFromValuesH {
                        result_type: array_reference_h.expect_static_sized_array_coord(),
                        source_expressions: self.interner.alloc_slice_from_vec(results_he),
                    }));
                    let new_struct_and_deferreds_expr_h = self.translate_deferreds(hinputs, hamuts, current_function_header, locals, new_struct_node, deferreds);
                    (new_struct_and_deferreds_expr_h, Vec::new())
                }
                RE::IsSameInstance(a) => panic!("translate_expression: IsSameInstance branch"),
                RE::AsSubtype(a) => panic!("translate_expression: AsSubtype branch"),
                RE::ArgLookup(a) => {
                    let crate::instantiating::ast::expressions::ArgLookupIE { param_index, coord: type2 } = *a;
                    let type_h = self.translate_coord(hinputs, hamuts, type2);
                    assert!(current_function_header.param_types()[param_index as usize] == type2);
                    assert!(self.translate_coord(hinputs, hamuts, current_function_header.param_types()[param_index as usize]) == type_h);
                    let arg_node = ExpressionH::ArgumentH(self.interner.alloc(crate::final_ast::instructions::ArgumentH { result_type: type_h, argument_index: param_index }));
                    (arg_node, Vec::new())
                }
                RE::ExternFunctionCall(efc) => {
                    let access = self.translate_extern_function_call(hinputs, hamuts, current_function_header, locals, &efc.prototype2, efc.args);
                    (access, Vec::new())
                }
                RE::Reinterpret(a) => panic!("translate_expression: Reinterpret branch"),
                RE::Construct(a) => {
                    let (members_he, deferreds) = self.translate_expressions_until_never(hinputs, hamuts, current_function_header, locals, a.args);
                    // Don't evaluate anything that can't ever be run, see BRCOBS
                    match members_he.last().map(|e| e.result_type().kind) {
                        Some(crate::final_ast::types::KindHT::NeverHT(_)) => {
                            panic!("Unimplemented: Construct Never branch (Hammer.consecrash)");
                        }
                        _ => {}
                    }
                    let result_type_h = self.translate_coord(hinputs, hamuts, a.result);
                    let struct_def_h = *hamuts.struct_t_to_struct_def_h().get(&a.struct_tt).unwrap();
                    assert_eq!(members_he.len(), struct_def_h.members.len());
                    for (member_he, member_h) in members_he.iter().zip(struct_def_h.members.iter()) {
                        assert_eq!(member_he.result_type(), member_h.tyype);
                    }
                    let member_names: Vec<&'h crate::final_ast::ast::IdH<'s, 'h>> = struct_def_h.members.iter().map(|m| m.name).collect();
                    let new_struct_node = ExpressionH::NewStructH(self.interner.alloc(crate::final_ast::instructions::NewStructH {
                        source_expressions: self.interner.bump().alloc_slice_fill_iter(members_he.into_iter()),
                        target_member_names: self.interner.bump().alloc_slice_fill_iter(member_names.into_iter()),
                        result_type: { assert!(matches!(result_type_h.kind, crate::final_ast::types::KindHT::StructHT(_))); result_type_h },
                    }));
                    let new_struct_and_deferreds_expr_h = self.translate_deferreds(hinputs, hamuts, current_function_header, locals, new_struct_node, deferreds);
                    (new_struct_and_deferreds_expr_h, Vec::new())
                }
                RE::NewMutRuntimeSizedArray(a) => panic!("translate_expression: NewMutRuntimeSizedArray branch"),
                RE::StaticArrayFromCallable(a) => panic!("translate_expression: StaticArrayFromCallable branch"),
                RE::DestroyStaticSizedArrayIntoFunction(a) => panic!("translate_expression: DestroyStaticSizedArrayIntoFunction branch"),
                RE::DestroyMutRuntimeSizedArray(a) => panic!("translate_expression: DestroyMutRuntimeSizedArray branch"),
                RE::PushRuntimeSizedArray(a) => panic!("translate_expression: PushRuntimeSizedArray branch"),
                RE::PopRuntimeSizedArray(a) => panic!("translate_expression: PopRuntimeSizedArray branch"),
                RE::InterfaceToInterfaceUpcast(a) => panic!("translate_expression: InterfaceToInterfaceUpcast branch"),
                RE::Upcast(up) => {
                    let target_pointer_type2 = up.result;
                    let source_pointer_type2 = ExpressionIE::Reference(up.inner_expr).result();
                    let source_pointer_type_h = self.translate_coord(hinputs, hamuts, source_pointer_type2);
                    let target_pointer_type_h = self.translate_coord(hinputs, hamuts, target_pointer_type2);
                    let _source_struct_ref_h = match source_pointer_type_h.kind {
                        crate::final_ast::types::KindHT::StructHT(s) => s,
                        _ => panic!("Upcast: source not struct"),
                    };
                    let target_interface_ref_h = match target_pointer_type_h.kind {
                        crate::final_ast::types::KindHT::InterfaceHT(i) => i,
                        _ => panic!("Upcast: target not interface"),
                    };
                    let (inner_expr_he, inner_deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, locals, ExpressionIE::Reference(up.inner_expr));
                    let upcast_node = ExpressionH::StructToInterfaceUpcastH(self.interner.alloc(crate::final_ast::instructions::StructToInterfaceUpcastH {
                        source_expression: inner_expr_he.expect_struct_access(),
                        target_interface: target_interface_ref_h,
                    }));
                    (upcast_node, inner_deferreds)
                }
                RE::SoftLoad(load2) => {
                    let (loaded_access_h, deferreds) = self.translate_load(hinputs, hamuts, current_function_header, locals, load2);
                    (loaded_access_h, deferreds)
                }
                RE::DestroyImmRuntimeSizedArray(a) => panic!("translate_expression: DestroyImmRuntimeSizedArray branch"),
                RE::NewImmRuntimeSizedArray(a) => panic!("translate_expression: NewImmRuntimeSizedArray branch"),
            },
            ExpressionIE::Address(_) => panic!("translate_expression: Address branch"),
        }
    }
}
/*
  // stackHeight is the number of locals that have been declared in ancestor
  // blocks and previously in this block. It's used to figure out the index of
  // a newly declared local.
  // Returns:
  // - result register id
  // - deferred expressions, to move to after the enclosing call. head is put first after call.
  def translate(
      hinputs: HinputsI,
      hamuts: HamutsBox,
      currentFunctionHeader: FunctionHeaderI,
      locals: LocalsBox,
      expr2: ExpressionI
  ): (ExpressionH[KindHT], Vector[ExpressionI]) = {
    expr2 match {
      case ConstantIntIE(numTemplata, bits) => {
        val num = numTemplata
        (ConstantIntH(num, bits), Vector.empty)
      }
      case VoidLiteralIE() => {
        val constructH = ConstantVoidH()
        (constructH, Vector.empty)
      }
      case ConstantStrIE(value) => {
        (ConstantStrH(value), Vector.empty)
      }
      case ConstantFloatIE(value) => {
        (ConstantF64H(value), Vector.empty)
      }
      case ConstantBoolIE(value) => {
        (ConstantBoolH(value), Vector.empty)
      }
      case let2 @ LetNormalIE(_, _, _) => {
        val letH =
          letHammer.translateLet(hinputs, hamuts, currentFunctionHeader, locals, let2)
        (letH, Vector.empty)
      }
      case let2 @ RestackifyIE(_, _, _) => {
        val letH =
          letHammer.translateRestackify(hinputs, hamuts, currentFunctionHeader, locals, let2)
        (letH, Vector.empty)
      }
      case let2 @ LetAndLendIE(_, _, _, _) => {
        val borrowAccess =
          letHammer.translateLetAndPoint(hinputs, hamuts, currentFunctionHeader, locals, let2)
        (borrowAccess, Vector.empty)
      }
      case des2 @ DestroyIE(_, _, _) => {
        val destroyH =
            letHammer.translateDestroy(hinputs, hamuts, currentFunctionHeader, locals, des2)
        // Compiler destructures put things in local variables (even though hammer itself
        // uses registers internally to make that happen).
        // Since all the members landed in locals, we still need something to ret, so we
        // return a void.
        (destroyH, Vector.empty)
      }
      case des2 @ DestroyStaticSizedArrayIntoLocalsIE(_, _, _) => {
        val destructureH =
            letHammer.translateDestructureStaticSizedArray(hinputs, hamuts, currentFunctionHeader, locals, des2)
        // Compiler destructures put things in local variables (even though hammer itself
        // uses registers internally to make that happen).
        // Since all the members landed in locals, we still need something to ret, so we
        // return a void.
        (destructureH, Vector.empty)
      }
      case unlet2 @ UnletIE(_, _) => {
        val valueAccess =
          letHammer.translateUnlet(
            hinputs, hamuts, currentFunctionHeader, locals, unlet2)
        (valueAccess, Vector.empty)
      }
      case mutate2 @ MutateIE(_, _, _) => {
        val newEmptyPackStructNodeHE =
          mutateHammer.translateMutate(hinputs, hamuts, currentFunctionHeader, locals, mutate2)
        (newEmptyPackStructNodeHE, Vector.empty)
      }
      case b @ MutabilifyIE(_, _) => {
        val pureH =
          blockHammer.translateMutabilify(hinputs, hamuts, currentFunctionHeader, locals, b)
        (pureH, Vector.empty)
      }
      case b@ImmutabilifyIE(_, _) => {
        val pureH =
          blockHammer.translateImmutabilify(hinputs, hamuts, currentFunctionHeader, locals, b)
        (pureH, Vector.empty)
      }
      case b @ BlockIE(_, _) => {
        val blockH =
          blockHammer.translateBlock(hinputs, hamuts, currentFunctionHeader, locals, b)
        (blockH, Vector.empty)
      }
      case call2 @ FunctionCallIE(callableExpr, args, _) => {
        val access =
          translateFunctionPointerCall(
            hinputs, hamuts, currentFunctionHeader, locals, callableExpr, args, call2.result)
        (access, Vector.empty)
      }
      case PreCheckBorrowIE(inner) => {
        val (innerHE, deferreds) =
          translate(hinputs, hamuts, currentFunctionHeader, locals, inner)
        (PreCheckBorrowH(innerHE), deferreds)
      }
      case InterfaceFunctionCallIE(superFunctionPrototype, virtualParamIndex, argsExprs2, resultType2) => {
        val access =
          translateInterfaceFunctionCall(
            hinputs, hamuts, currentFunctionHeader, locals, superFunctionPrototype, virtualParamIndex, resultType2, argsExprs2)
        (access, Vector.empty)
      }

      case ConsecutorIE(exprsIE, _) => {
        // If there's an expression returning a Never, then remove all the expressions after that.
        // See BRCOBS.
        val exprsHE =
          exprsIE.foldLeft(Vector[ExpressionH[KindHT]]())({
            case (previousHE, nextIE) => {
              previousHE.lastOption.map(_.resultType.kind) match {
                case Some(NeverHT(_)) => previousHE
                case _ => {
                  val (nextHE, nextDeferreds) =
                    translate(hinputs, hamuts, currentFunctionHeader, locals, nextIE);
                  val nextExprWithDeferredsHE =
                    translateDeferreds(
                      hinputs, hamuts, currentFunctionHeader, locals, nextHE, nextDeferreds)
                  previousHE :+ nextExprWithDeferredsHE
                }
              }
            }
          })
        exprsHE.lastOption.map(_.resultType.kind) match {
          case Some(NeverHT(_)) => {
            return (Hammer.consecrash(locals, exprsHE), Vector.empty)
          }
          case _ =>
        }

        (Hammer.consecutive(exprsHE), Vector.empty)
      }

      case ArrayLengthIE(arrayExpr2) => {
        val (resultHE, deferreds) =
          translate(hinputs, hamuts, currentFunctionHeader, locals, arrayExpr2);

        val lengthResultNode = ArrayLengthH(resultHE);

        val arrayLengthAndDeferredsExprH =
          translateDeferreds(hinputs, hamuts, currentFunctionHeader, locals, lengthResultNode, deferreds)

        (arrayLengthAndDeferredsExprH, Vector.empty)
      }

      case RuntimeSizedArrayCapacityIE(arrayExpr2) => {
        val (resultHE, deferreds) =
          translate(hinputs, hamuts, currentFunctionHeader, locals, arrayExpr2);

        val lengthResultNode = ArrayCapacityH(resultHE);

        val arrayLengthAndDeferredsExprH =
          translateDeferreds(hinputs, hamuts, currentFunctionHeader, locals, lengthResultNode, deferreds)

        (arrayLengthAndDeferredsExprH, Vector.empty)
      }

      case LockWeakIE(innerExpr2, resultOptBorrowType2, someConstructor, noneConstructor, _, _, _) => {
        val (resultHE, deferreds) =
          translate(hinputs, hamuts, currentFunctionHeader, locals, innerExpr2);
        val (resultOptBorrowTypeH) =
          typeHammer.translateCoord(hinputs, hamuts, resultOptBorrowType2)

        val someConstructorH =
          functionHammer.translateFunctionRef(hinputs, hamuts, currentFunctionHeader, someConstructor);
        val noneConstructorH =
          functionHammer.translateFunctionRef(hinputs, hamuts, currentFunctionHeader, noneConstructor);

        val resultNode =
          LockWeakH(
            resultHE,
            resultOptBorrowTypeH.expectInterfaceCoord(),
            someConstructorH.prototype,
            noneConstructorH.prototype);
        (resultNode, deferreds)
      }

      case TupleIE(exprs, resultType) => {
        val (resultsHE, deferreds) =
          translateExpressionsUntilNever(
            hinputs, hamuts, currentFunctionHeader, locals, exprs);
        // Don't evaluate anything that can't ever be run, see BRCOBS
        resultsHE.lastOption.map(_.resultType.kind) match {
          case Some(NeverHT(_)) => {
            return (Hammer.consecrash(locals, resultsHE), Vector())
          }
          case _ =>
        }

        val resultStructI = resultType.kind match { case s @ StructIT(_) => s }
        assert(resultStructI.id.packageCoord.module.str != "rust") // DO NOT SUBMIT
        val (underlyingStructRefH) =
          structHammer.translateStructI(hinputs, hamuts, resultStructI);
        val (resultReference) =
          typeHammer.translateCoord(hinputs, hamuts, resultType)
        vassert(resultReference.kind == underlyingStructRefH)

        val structDefH = hamuts.structTToStructDefH(resultStructI)
        vassert(resultsHE.size == structDefH.members.size)
        val newStructNode =
          NewStructH(
            resultsHE,
            structDefH.members.map(_.name),
            resultReference.expectStructCoord())
        // Export locals from inside the pack

        val newStructAndDeferredsExprH =
            translateDeferreds(hinputs, hamuts, currentFunctionHeader, locals, newStructNode, deferreds)

        (newStructAndDeferredsExprH, Vector.empty)
      }

      case StaticArrayFromValuesIE(exprs, arrayReference2, arrayType2) => {
        val (resultsHE, deferreds) =
          translateExpressionsUntilNever(hinputs, hamuts, currentFunctionHeader, locals, exprs);
        // Don't evaluate anything that can't ever be run, see BRCOBS
        resultsHE.lastOption.map(_.resultType.kind) match {
          case Some(NeverHT(_)) => {
            return (Hammer.consecrash(locals, resultsHE), Vector())
          }
          case _ =>
        }
        val (underlyingArrayH) =
          typeHammer.translateStaticSizedArray(hinputs, hamuts, arrayType2);

        val (arrayReferenceH) =
          typeHammer.translateCoord(hinputs, hamuts, arrayReference2)
        vassert(arrayReferenceH.kind == underlyingArrayH)

        val newStructNode =
          NewArrayFromValuesH(
            arrayReferenceH.expectStaticSizedArrayCoord(),
            resultsHE)

        val newStructAndDeferredsExprH =
        translateDeferreds(hinputs, hamuts, currentFunctionHeader, locals, newStructNode, deferreds)

        (newStructAndDeferredsExprH, Vector.empty)
      }

      case ConstructIE(structIT, resultType2, memberExprs) => {
        val (membersHE, deferreds) =
          translateExpressionsUntilNever(hinputs, hamuts, currentFunctionHeader, locals, memberExprs);
        // Don't evaluate anything that can't ever be run, see BRCOBS
        membersHE.lastOption.map(_.resultType.kind) match {
          case Some(NeverHT(_)) => {
            return (Hammer.consecrash(locals, membersHE), Vector())
          }
          case _ =>
        }

        val (resultTypeH) =
          typeHammer.translateCoord(hinputs, hamuts, resultType2)


        val structDefH = hamuts.structTToStructDefH(structIT)
        vassert(membersHE.size == structDefH.members.size)
        membersHE.zip(structDefH.members).foreach({ case (memberHE, memberH ) =>
          vassert(memberHE.resultType == memberH.tyype)
        })
        val newStructNode =
          NewStructH(
            membersHE,
            structDefH.members.map(_.name),
            resultTypeH.expectStructCoord())

        val newStructAndDeferredsExprH =
            translateDeferreds(hinputs, hamuts, currentFunctionHeader, locals, newStructNode, deferreds)

        (newStructAndDeferredsExprH, Vector.empty)
      }

      case load2 @ SoftLoadIE(_, _, _) => {
        val (loadedAccessH, deferreds) =
          loadHammer.translateLoad(hinputs, hamuts, currentFunctionHeader, locals, load2)
        (loadedAccessH, deferreds)
      }

      case lookup2 @ LocalLookupIE(AddressibleLocalVariableI(_, _, _), _) => {
        vregionmut()
        val loadBoxAccess =
          loadHammer.translateLocalAddress(hinputs, hamuts, currentFunctionHeader, locals, lookup2)
        (loadBoxAccess, Vector.empty)
      }

      case lookup2 @ AddressMemberLookupIE(_, _, _, _) => {
        val (loadBoxAccess, deferreds) =
          loadHammer.translateMemberAddress(hinputs, hamuts, currentFunctionHeader, locals, lookup2)
        (loadBoxAccess, deferreds)
      }

      case if2 @ IfIE(_, _, _, _) => {
        val maybeAccess =
          translateIf(hinputs, hamuts, currentFunctionHeader, locals, if2)
        (maybeAccess, Vector.empty)
      }

      case prsaIE @ PushRuntimeSizedArrayIE(_, _) => {

        val PushRuntimeSizedArrayIE(arrayIE, newcomerIE) = prsaIE;

        val (arrayHE, arrayDeferreds) =
          translate(
            hinputs, hamuts, currentFunctionHeader, locals, arrayIE);
        val rsaHE = arrayHE.expectRuntimeSizedArrayAccess()
        val rsaDefH = hamuts.getRuntimeSizedArray(rsaHE.resultType.kind)

        val (newcomerHE, newcomerDeferreds) =
          translate(
            hinputs, hamuts, currentFunctionHeader, locals, newcomerIE);

        vassert(newcomerHE.resultType == rsaDefH.elementType)

        val constructArrayCallNode = PushRuntimeSizedArrayH(rsaHE, newcomerHE)

        val access =
          translateDeferreds(
            hinputs, hamuts, currentFunctionHeader, locals, constructArrayCallNode, arrayDeferreds ++ newcomerDeferreds)

        (access, Vector.empty)
      }

      case prsaIE @ PopRuntimeSizedArrayIE(_, _) => {
        val PopRuntimeSizedArrayIE(arrayIE, _) = prsaIE;

        val (arrayHE, arrayDeferreds) =
          translate(
            hinputs, hamuts, currentFunctionHeader, locals, arrayIE);
        val rsaHE = arrayHE.expectRuntimeSizedArrayAccess()
        val rsaDefH = hamuts.getRuntimeSizedArray(rsaHE.resultType.kind)

        val constructArrayCallNode = PopRuntimeSizedArrayH(rsaHE, rsaDefH.elementType)

        val access =
          translateDeferreds(
            hinputs, hamuts, currentFunctionHeader, locals, constructArrayCallNode, arrayDeferreds)

        (access, Vector.empty)
      }

      case nmrsaIE @ NewMutRuntimeSizedArrayIE(_, _, _) => {
        val access =
          translateNewMutRuntimeSizedArray(
            hinputs, hamuts, currentFunctionHeader, locals, nmrsaIE)
        (access, Vector.empty)
      }

      case nirsaIE @ NewImmRuntimeSizedArrayIE(_, _, _, _, _) => {
        val access =
          translateNewImmRuntimeSizedArray(
            hinputs, hamuts, currentFunctionHeader, locals, nirsaIE)
        (access, Vector.empty)
      }

      case ca2 @ StaticArrayFromCallableIE(_, _, _, _) => {
        val access =
          translateStaticArrayFromCallable(
            hinputs, hamuts, currentFunctionHeader, locals, ca2)
        (access, Vector.empty)
      }

      case ReinterpretIE(innerExpr, resultType2, _) => {
        // Check types; it's overkill because reinterprets are rather scary.
        val innerExprResultType2 = innerExpr.result
        val (innerExprResultTypeH) = typeHammer.translateCoord(hinputs, hamuts, innerExprResultType2);
        val (resultTypeH) = typeHammer.translateCoord(hinputs, hamuts, resultType2);
        innerExprResultTypeH.kind match {
          case NeverHT(_) =>
          case _ => {
            if (innerExprResultTypeH != resultTypeH) {
              vfail(innerExprResultTypeH + " doesnt match " + resultTypeH);
            }
          }
        }

        val (innerExprHE, deferreds) =
          translate(hinputs, hamuts, currentFunctionHeader, locals, innerExpr);

        // Not always the case:
        //   vcurious(innerExprResultTypeH.kind == NeverH() || resultTypeH.kind == NeverH())
        // for example when we're destructuring a TupleT2 or PackT2, we interpret from that
        // to its understruct.

        // These both trip:
        //   vcurious(innerExprResultTypeH == resultTypeH)
        //   vcurious(innerExprResultTypeH != resultTypeH)
        // Because sometimes theyre actually the same, because we might interpret a tuple to
        // its understruct, and sometimes theyre different, when we're making a Never into
        // an Int for example when one branch of an If panics or returns.

        innerExprResultTypeH.kind match {
          case NeverHT(_) => vfail()
          case _ =>
        }
        resultTypeH.kind match {
          case NeverHT(_) => vfail()
          case _ =>
        }
        (innerExprHE, deferreds)
      }

      case up @ InterfaceToInterfaceUpcastIE(innerExpr, targetInterfaceRef2, _) => {
        val targetPointerType2 = up.result;
        val sourcePointerType2 = innerExpr.result

        val (sourcePointerTypeH) =
          typeHammer.translateCoord(hinputs, hamuts, sourcePointerType2);
        val (targetPointerTypeH) =
          typeHammer.translateCoord(hinputs, hamuts, targetPointerType2);

        val sourceStructRefH = sourcePointerTypeH.kind.asInstanceOf[InterfaceHT]
        val targetInterfaceRefH = targetPointerTypeH.kind.asInstanceOf[InterfaceHT]

        val (innerExprHE, innerDeferreds) =
          translate(hinputs, hamuts, currentFunctionHeader, locals, innerExpr);
        // Upcasting an interface is technically a no-op with our language, but the sculptor
        // will still want to do it to do some checking in debug mode.
        val upcastNode =
            InterfaceToInterfaceUpcastH(
              innerExprHE.expectInterfaceAccess(),
              targetInterfaceRefH);
        (upcastNode, innerDeferreds)
      }

      case up @ UpcastIE(innerExpr, targetInterfaceRef2, _, _) => {
        val targetPointerType2 = up.result;
        val sourcePointerType2 = innerExpr.result

        val (sourcePointerTypeH) =
          typeHammer.translateCoord(hinputs, hamuts, sourcePointerType2);
        val (targetPointerTypeH) =
          typeHammer.translateCoord(hinputs, hamuts, targetPointerType2);

        val sourceStructRefH = sourcePointerTypeH.kind.asInstanceOf[StructHT]

        val targetInterfaceRefH = targetPointerTypeH.kind.asInstanceOf[InterfaceHT]

        val (innerExprHE, innerDeferreds) =
          translate(hinputs, hamuts, currentFunctionHeader, locals, innerExpr);
        // Upcasting an interface is technically a no-op with our language, but the sculptor
        // will still want to do it to do some checking in debug mode.
        val upcastNode =
            StructToInterfaceUpcastH(
              innerExprHE.expectStructAccess(),
              targetInterfaceRefH)
        (upcastNode, innerDeferreds)
      }

      case ExternFunctionCallIE(prototype2, argsExprs2, _) => {
        val access =
          translateExternFunctionCall(hinputs, hamuts, currentFunctionHeader, locals, prototype2, argsExprs2)
        (access, Vector.empty)
      }

      case while2 @ WhileIE(_, _) => {
        val whileH =
            translateWhile(hinputs, hamuts, currentFunctionHeader, locals, while2)
        (whileH, Vector.empty)
      }

      case DeferIE(innerExpr, deferredExpr, _) => {
        val (innerExprHE, innerDeferreds) =
          translate(hinputs, hamuts, currentFunctionHeader, locals, innerExpr);
        (innerExprHE, Vector(deferredExpr) ++ innerDeferreds)
      }

      case DiscardIE(innerExpr) => {
        val (undiscardedInnerExprH, innerDeferreds) =
          translate(hinputs, hamuts, currentFunctionHeader, locals, innerExpr);
        vassert(innerDeferreds.isEmpty) // BMHD, probably need to translate them here.
        val innerExprH = DiscardH(undiscardedInnerExprH)
        val innerWithDeferredsExprH =
          translateDeferreds(hinputs, hamuts, currentFunctionHeader, locals, innerExprH, innerDeferreds)
        (innerWithDeferredsExprH, Vector.empty)
      }
      case ReturnIE(innerExpr) => {
        vassert(
          innerExpr.result.kind == NeverIT[cI](false) ||
          innerExpr.result == currentFunctionHeader.returnType)

        val (innerExprHE, innerDeferreds) =
          translate(hinputs, hamuts, currentFunctionHeader, locals, innerExpr);

        // Return is a special case where we execute the *inner* expression (not the whole return expression) and
        // then the deferreds and then the return. See MEDBR.
        val innerWithDeferreds =
            translateDeferreds(hinputs, hamuts, currentFunctionHeader, locals, innerExprHE, innerDeferreds)

        // If we're returning a never, just strip this Return, because we'll
        // never get here.
        innerWithDeferreds.resultType.kind match {
          case NeverHT(_) => {
            return (innerWithDeferreds, Vector.empty)
          }
          case _ =>
        }

        vassert(innerExpr.result == currentFunctionHeader.returnType)
        (ReturnH(innerWithDeferreds), Vector.empty)
      }
      case ArgLookupIE(paramIndex, type2) => {
        val typeH = typeHammer.translateCoord(hinputs, hamuts, type2)
        vassert(currentFunctionHeader.paramTypes(paramIndex) == type2)
        vassert(typeHammer.translateCoord(hinputs, hamuts, currentFunctionHeader.paramTypes(paramIndex)) == typeH)
        val argNode = ArgumentH(typeH, paramIndex)
        (argNode, Vector.empty)
      }

      case das2 @ DestroyStaticSizedArrayIntoFunctionIE(_, _, _, _) => {
        val dasH =
            translateDestroyStaticSizedArray(
              hinputs, hamuts, currentFunctionHeader, locals, das2)
        (dasH, Vector.empty)
      }

      case das2 @ DestroyImmRuntimeSizedArrayIE(_, _, _, _) => {
        val drsaH =
            translateDestroyImmRuntimeSizedArray(
              hinputs, hamuts, currentFunctionHeader, locals, das2)
        (drsaH, Vector.empty)
      }

//      case UnreachableMootIE(innerExpr) => {
//        val (innerExprHE, innerDeferreds) =
//          translate(hinputs, hamuts, currentFunctionHeader, locals, innerExpr);
//        val innerWithDeferredsH =
//          translateDeferreds(hinputs, hamuts, currentFunctionHeader, locals, innerExprHE, innerDeferreds)
//
//        // Throw away the inner expression because we dont want it to be generated, because
//        // theyll never get run anyway.
//        // We only translated them above to mark unstackified things unstackified.
//
//        val void = ConstantVoidH()
//
//        (void, Vector.empty)
//      }

      case BorrowToWeakIE(innerExpr, _) => {
        val (innerExprHE, innerDeferreds) =
          translate(hinputs, hamuts, currentFunctionHeader, locals, innerExpr);
        (BorrowToWeakH(innerExprHE), innerDeferreds)
      }

      case IsSameInstanceIE(leftExprI, rightExprI) => {
        val (leftExprHE, leftDeferreds) =
          translate(hinputs, hamuts, currentFunctionHeader, locals, leftExprI);
        val (rightExprHE, rightDeferreds) =
          translate(hinputs, hamuts, currentFunctionHeader, locals, rightExprI);
        val resultHE = IsSameInstanceH(leftExprHE, rightExprHE)

        val expr = translateDeferreds(hinputs, hamuts, currentFunctionHeader, locals, resultHE, leftDeferreds ++ rightDeferreds)
        (expr, Vector.empty)
      }

      case AsSubtypeIE(leftExprI, targetSubtype, resultOptType, someConstructor, noneConstructor, _, _, _, _) => {
        val (resultHE, deferreds) =
          translate(hinputs, hamuts, currentFunctionHeader, locals, leftExprI);
        val (targetSubtypeH) =
          typeHammer.translateCoord(hinputs, hamuts, targetSubtype).kind
        val (resultOptTypeH) =
          typeHammer.translateCoord(hinputs, hamuts, resultOptType).expectInterfaceCoord()

        val someConstructorH =
          functionHammer.translateFunctionRef(hinputs, hamuts, currentFunctionHeader, someConstructor);
        val noneConstructorH =
          functionHammer.translateFunctionRef(hinputs, hamuts, currentFunctionHeader, noneConstructor);

        val resultNode =
          AsSubtypeH(
            resultHE,
            targetSubtypeH,
            resultOptTypeH,
            someConstructorH.prototype,
            noneConstructorH.prototype);
        (resultNode, deferreds)
      }

      case DestroyMutRuntimeSizedArrayIE(rsaIE) => {
        val (rsaHE, rsaDeferreds) =
          translate(hinputs, hamuts, currentFunctionHeader, locals, rsaIE);

        val destroyHE =
          DestroyMutRuntimeSizedArrayH(rsaHE.expectRuntimeSizedArrayAccess())

        val expr = translateDeferreds(hinputs, hamuts, currentFunctionHeader, locals, destroyHE, rsaDeferreds)
        (expr, Vector.empty)
      }

      case BreakIE() => {
        (BreakH(), Vector.empty)
      }

      case _ => {
        vfail("wat " + expr2)
      }
    }
  }
*/

// mig: fn translate_deferreds
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_deferreds(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        original_expr: ExpressionH<'s, 'h>,
        deferreds: Vec<ExpressionIE<'s, 'i, cI>>,
    ) -> ExpressionH<'s, 'h>
    {
        if deferreds.is_empty() {
            return original_expr;
        }
        let (deferred_exprs, deferred_deferreds) =
            self.translate_expressions_until_never(hinputs, hamuts, current_function_header, locals, &deferreds);
        match deferred_exprs.last().map(|e| e.result_type().kind) {
            Some(crate::final_ast::types::KindHT::NeverHT(_)) => {
                return crate::simplifying::hammer::consecrash(self.interner, locals, &deferred_exprs);
            }
            _ => {}
        }
        if deferred_exprs.iter().map(|e| e.result_type().kind).any(|k| !matches!(k, crate::final_ast::types::KindHT::VoidHT(_))) {
            panic!("translate_deferreds: vcurious — deferred had non-void result");
        }
        if locals.locals.len() != locals.locals.len() {
            // There shouldnt have been any locals introduced
            panic!("wat");
        }
        assert!(deferred_deferreds.is_empty());
        assert!(!deferred_exprs.is_empty());
        let new_exprs: Vec<ExpressionH<'s, 'h>> =
            if matches!(original_expr.result_type().kind, crate::final_ast::types::KindHT::VoidHT(_)) {
                let void = ExpressionH::ConstantVoidH(self.interner.alloc(crate::final_ast::instructions::ConstantVoidH));
                let mut v = vec![original_expr];
                v.extend(deferred_exprs.iter().copied());
                v.push(void);
                v
            } else {
                let temporary_result_local = locals.add_hammer_local(original_expr.result_type(), crate::final_ast::types::Variability::Final);
                let stackify = ExpressionH::StackifyH(self.interner.alloc(crate::final_ast::instructions::StackifyH { source_expr: original_expr, local: temporary_result_local, name: None }));
                let unstackify = ExpressionH::UnstackifyH(self.interner.alloc(crate::final_ast::instructions::UnstackifyH { local: temporary_result_local }));
                locals.mark_unstackified(temporary_result_local.id);
                let mut v = vec![stackify];
                v.extend(deferred_exprs.iter().copied());
                v.push(unstackify);
                v
            };
        let result = ExpressionH::ConsecutorH(self.interner.alloc(crate::final_ast::instructions::ConsecutorH {
            exprs: self.interner.bump().alloc_slice_copy(&new_exprs),
        }));
        assert!(original_expr.result_type() == result.result_type());
        result
    }
}
/*
  def translateDeferreds(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    originalExpr: ExpressionH[KindHT],
    deferreds: Vector[ExpressionI]):
  ExpressionH[KindHT] = {
    if (deferreds.isEmpty) {
      return originalExpr
    }

    val (deferredExprs, deferredDeferreds) =
      translateExpressionsUntilNever(
        hinputs, hamuts, currentFunctionHeader, locals, deferreds)
    // Don't evaluate anything that can't ever be run, see BRCOBS
    deferredExprs.lastOption.map(_.resultType.kind) match {
      case Some(NeverHT(_)) => {
        return Hammer.consecrash(locals, deferredExprs)
      }
      case _ =>
    }
    if (deferredExprs.map(_.resultType.kind).toSet != Set(VoidHT())) {
      // curiosity, why would a deferred ever have a result
      vcurious()
    }
    if (locals.locals.size != locals.locals.size) {
      // There shouldnt have been any locals introduced
      vfail("wat")
    }
    vassert(deferredDeferreds.isEmpty)
    // Don't need these, they should all be voids anyway

    vcurious(deferredExprs.nonEmpty)

    val newExprs =
      if (originalExpr.resultType.kind == VoidHT()) {
        val void = ConstantVoidH()
        Vector(originalExpr) ++ (deferredExprs :+ void)
      } else {
        val temporaryResultLocal = locals.addHammerLocal(originalExpr.resultType, Final)
        val stackify = StackifyH(originalExpr, temporaryResultLocal, None)
        val unstackify = UnstackifyH(temporaryResultLocal)
        locals.markUnstackified(temporaryResultLocal.id)
        Vector(stackify) ++ deferredExprs ++ Vector(unstackify)
      }

    val result = ConsecutorH(newExprs)
    vassert(originalExpr.resultType == result.resultType)
    result
  }
*/

// mig: fn translate_expressions_until_never
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_expressions_until_never(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        exprs_ie: &[ExpressionIE<'s, 'i, cI>],
    ) -> (Vec<ExpressionH<'s, 'h>>, Vec<ExpressionIE<'s, 'i, cI>>)
    {
        let (exprs_he, deferreds) = exprs_ie.iter().fold((Vec::<ExpressionH<'s, 'h>>::new(), Vec::<ExpressionIE<'s, 'i, cI>>::new()), |(prev_exprs_he, prev_deferreds), next_ie| {
            let prev_is_never = match prev_exprs_he.last().map(|e| e.result_type().kind) {
                Some(crate::final_ast::types::KindHT::NeverHT(_)) => true,
                _ => false,
            };
            if prev_is_never {
                (prev_exprs_he, Vec::new())
            } else {
                let (next_he, next_deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, locals, *next_ie);
                let mut new_exprs = prev_exprs_he;
                new_exprs.push(next_he);
                let mut new_deferreds = prev_deferreds;
                new_deferreds.extend(next_deferreds);
                (new_exprs, new_deferreds)
            }
        });
        // We'll never get to the deferreds, so forget them.
        match exprs_he.last().map(|e| e.result_type().kind) {
            Some(crate::final_ast::types::KindHT::NeverHT(_)) => (exprs_he, Vec::new()),
            _ => (exprs_he, deferreds),
        }
    }
}
/*
  def translateExpressionsUntilNever(
    hinputs: HinputsI, hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    exprsIE: Vector[ExpressionI]):
  (Vector[ExpressionH[KindHT]], Vector[ExpressionI]) = {
    val (exprsHE, deferreds) =
      exprsIE.foldLeft((Vector[ExpressionH[KindHT]](), Vector[ExpressionI]()))({
        // If we previously saw a Never, stop there, don't proceed, don't even waste
        // time compiling the rest.
        case ((prevExprsHE, _), _)
            if (prevExprsHE.lastOption.map(_.resultType.kind) match {
              case Some(NeverHT(_)) => true case _ => false
            }) => {
          (prevExprsHE, Vector())
        }
        case ((prevExprsHE, prevDeferreds), nextIE) => {
          val (nextHE, nextDeferreds) =
            translate(hinputs, hamuts, currentFunctionHeader, locals, nextIE);
          (prevExprsHE :+ nextHE, prevDeferreds ++ nextDeferreds)
        }
      })

    // We'll never get to the deferreds, so forget them.
    exprsHE.lastOption.map(_.resultType.kind) match {
      case Some(NeverHT(_)) => (exprsHE, Vector())
      case _ => (exprsHE, deferreds)
    }
  }
*/

// mig: fn translate_expressions_and_deferreds
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_expressions_and_deferreds(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        exprs2: &[ExpressionIE<'s, 'i, cI>],
    ) -> ExpressionH<'s, 'h>
    {
        let exprs: Vec<ExpressionH<'s, 'h>> = exprs2.iter().map(|expr2| {
            let (first_he, first_deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, locals, *expr2);
            self.translate_deferreds(hinputs, hamuts, current_function_header, locals, first_he, first_deferreds)
        }).collect();
        crate::simplifying::hammer::consecutive(self.interner, &exprs)
    }
}
/*
  def translateExpressionsAndDeferreds(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    exprs2: Vector[ExpressionI]):
  ExpressionH[KindHT] = {
    val exprs =
      exprs2.map({ case expr2 =>
        val (firstHE, firstDeferreds) =
          translate(hinputs, hamuts, currentFunctionHeader, locals, expr2);
        val firstExprWithDeferredsH =
          translateDeferreds(hinputs, hamuts, currentFunctionHeader, locals, firstHE, firstDeferreds)
        firstExprWithDeferredsH
      })
    Hammer.consecutive(exprs)
  }
*/

// mig: fn translate_extern_function_call
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_extern_function_call(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        prototype2: &'i PrototypeI<'s, 'i, cI>,
        args_exprs2: &[ReferenceExpressionIE<'s, 'i, cI>],
    ) -> ExpressionH<'s, 'h>
    {
        let args_ie: Vec<ExpressionIE<'s, 'i, cI>> = args_exprs2.iter().map(|a| ExpressionIE::Reference(*a)).collect();
        let (args_he, args_deferreds) = self.translate_expressions_until_never(hinputs, hamuts, current_function_header, locals, &args_ie);
        // Don't evaluate anything that can't ever be run, see BRCOBS
        if !args_he.is_empty() && matches!(args_he.last().unwrap().result_type().kind, crate::final_ast::types::KindHT::NeverHT(crate::final_ast::types::NeverHT { from_break: true })) {
            return crate::simplifying::hammer::consecrash(self.interner, locals, &args_he);
        }
        let param_types = self.translate_coords(hinputs, hamuts, &prototype2.param_types());
        assert!(args_he.iter().map(|e| e.result_type()).collect::<Vec<_>>() == param_types);
        let function_ref_h = self.translate_function_ref(hinputs, hamuts, current_function_header, prototype2);
        let call_result_node = ExpressionH::ExternCallH(self.interner.alloc(crate::final_ast::instructions::ExternCallH { function: function_ref_h.prototype, args_expressions: self.interner.bump().alloc_slice_fill_iter(args_he.into_iter()) }));
        self.translate_deferreds(hinputs, hamuts, current_function_header, locals, call_result_node, args_deferreds)
    }
}
/*
  def translateExternFunctionCall(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    prototype2: PrototypeI[cI],
    argsExprs2: Vector[ReferenceExpressionIE]):
  (ExpressionH[KindHT]) = {
    val (argsHE, argsDeferreds) =
      translateExpressionsUntilNever(
        hinputs, hamuts, currentFunctionHeader, locals, argsExprs2);
    // Don't evaluate anything that can't ever be run, see BRCOBS
    if (argsHE.nonEmpty && argsHE.last.resultType.kind == NeverHT(true)) {
      return Hammer.consecrash(locals, argsHE)
    }

    // Doublecheck the types
    val (paramTypes) =
      typeHammer.translateCoords(hinputs, hamuts, prototype2.paramTypes);
    vassert(argsHE.map(_.resultType) == paramTypes)

    val (functionRefH) =
      functionHammer.translateFunctionRef(hinputs, hamuts, currentFunctionHeader, prototype2);

    val callResultNode = ExternCallH(functionRefH.prototype, argsHE)

    translateDeferreds(
      hinputs, hamuts, currentFunctionHeader, locals, callResultNode, argsDeferreds)
  }
*/

// mig: fn translate_function_pointer_call
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_function_pointer_call(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        function: &'i PrototypeI<'s, 'i, cI>,
        args: &[ExpressionIE<'s, 'i, cI>],
        result_type2: CoordI<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        let return_type2 = function.return_type;
        let param_types = function.param_types();
        let (args_he, args_deferreds) = self.translate_expressions_until_never(hinputs, hamuts, current_function_header, locals, args);
        // Don't evaluate anything that can't ever be run, see BRCOBS
        match args_he.last().map(|e| e.result_type().kind) {
            Some(crate::final_ast::types::KindHT::NeverHT(_)) => {
                return crate::simplifying::hammer::consecrash(self.interner, locals, &args_he);
            }
            _ => {}
        }
        let prototype_h = self.translate_prototype(hinputs, hamuts, function);
        let param_types_h = self.translate_coords(hinputs, hamuts, &param_types);
        assert!(args_he.iter().map(|e| e.result_type()).collect::<Vec<_>>() == param_types_h);
        let return_type_h = self.translate_coord(hinputs, hamuts, return_type2);
        let result_type_h = self.translate_coord(hinputs, hamuts, result_type2);
        assert!(return_type_h == result_type_h);
        let call_result_node = ExpressionH::CallH(self.interner.alloc(crate::final_ast::instructions::CallH { function: prototype_h, args_expressions: self.interner.bump().alloc_slice_fill_iter(args_he.into_iter()) }));
        self.translate_deferreds(hinputs, hamuts, current_function_header, locals, call_result_node, args_deferreds)
    }
}
/*
  def translateFunctionPointerCall(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    function: PrototypeI[cI],
    args: Vector[ExpressionI],
    resultType2: CoordI[cI]):
  ExpressionH[KindHT] = {
    val returnType2 = function.returnType
    val paramTypes = function.paramTypes
    val (argsHE, argsDeferreds) =
      translateExpressionsUntilNever(
        hinputs, hamuts, currentFunctionHeader, locals, args);
    // Don't evaluate anything that can't ever be run, see BRCOBS
    argsHE.lastOption.map(_.resultType.kind) match {
      case Some(NeverHT(_)) => {
        return Hammer.consecrash(locals, argsHE)
      }
      case _ =>
    }

    val prototypeH =
      typeHammer.translatePrototype(hinputs, hamuts, function)

    // Doublecheck the types
    val (paramTypesH) =
      typeHammer.translateCoords(hinputs, hamuts, paramTypes)
    vassert(argsHE.map(_.resultType) == paramTypesH)

    // Doublecheck return
    val (returnTypeH) = typeHammer.translateCoord(hinputs, hamuts, returnType2)
    val (resultTypeH) = typeHammer.translateCoord(hinputs, hamuts, resultType2);
    vassert(returnTypeH == resultTypeH)

    val callResultNode = CallH(prototypeH, argsHE)

    translateDeferreds(
      hinputs, hamuts, currentFunctionHeader, locals, callResultNode, argsDeferreds)
  }
*/

// mig: fn translate_new_mut_runtime_sized_array
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_new_mut_runtime_sized_array(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        construct_array2: &NewMutRuntimeSizedArrayIE<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        panic!("Unimplemented: translate_new_mut_runtime_sized_array");
    }
}
/*
  def translateNewMutRuntimeSizedArray(
    hinputs: HinputsI, hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    constructArray2: NewMutRuntimeSizedArrayIE):
  (ExpressionH[KindHT]) = {
    val NewMutRuntimeSizedArrayIE(arrayType2, capacityExpr2, _) = constructArray2;

    val (capacityRegisterId, capacityDeferreds) =
      translate(
        hinputs, hamuts, currentFunctionHeader, locals, capacityExpr2);

    val (arrayRefTypeH) =
      typeHammer.translateCoord(
        hinputs, hamuts, constructArray2.result)

    val (arrayTypeH) =
      typeHammer.translateRuntimeSizedArray(hinputs, hamuts, arrayType2)
    vassert(arrayRefTypeH.expectRuntimeSizedArrayCoord().kind == arrayTypeH)

    val elementType = hamuts.getRuntimeSizedArray(arrayTypeH).elementType

    val constructArrayCallNode =
      NewMutRuntimeSizedArrayH(
        capacityRegisterId.expectIntAccess(),
        elementType,
        arrayRefTypeH.expectRuntimeSizedArrayCoord())

    translateDeferreds(
      hinputs, hamuts, currentFunctionHeader, locals, constructArrayCallNode, capacityDeferreds)
  }
*/

// mig: fn translate_new_imm_runtime_sized_array
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_new_imm_runtime_sized_array(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        construct_array2: &NewImmRuntimeSizedArrayIE<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        panic!("Unimplemented: translate_new_imm_runtime_sized_array");
    }
}
/*
  def translateNewImmRuntimeSizedArray(
    hinputs: HinputsI, hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    constructArray2: NewImmRuntimeSizedArrayIE):
  (ExpressionH[KindHT]) = {
    val NewImmRuntimeSizedArrayIE(arrayType2, sizeExpr2, generatorExpr2, generatorMethod, _) = constructArray2;

    val (sizeRegisterId, sizeDeferreds) =
      translate(
        hinputs, hamuts, currentFunctionHeader, locals, sizeExpr2);

    val (generatorRegisterId, generatorDeferreds) =
      translate(
        hinputs, hamuts, currentFunctionHeader, locals, generatorExpr2);

    val (arrayRefTypeH) =
      typeHammer.translateCoord(
        hinputs, hamuts, constructArray2.result)

    val (arrayTypeH) =
      typeHammer.translateRuntimeSizedArray(hinputs, hamuts, arrayType2)
    vassert(arrayRefTypeH.expectRuntimeSizedArrayCoord().kind == arrayTypeH)

    val elementType = hamuts.getRuntimeSizedArray(arrayTypeH).elementType

    val generatorMethodH =
      typeHammer.translatePrototype(hinputs, hamuts, generatorMethod)

    val constructArrayCallNode =
      NewImmRuntimeSizedArrayH(
        sizeRegisterId.expectIntAccess(),
        generatorRegisterId,
        generatorMethodH,
        elementType,
        arrayRefTypeH.expectRuntimeSizedArrayCoord())

    translateDeferreds(
      hinputs, hamuts, currentFunctionHeader, locals, constructArrayCallNode, generatorDeferreds ++ sizeDeferreds)
  }
*/

// mig: fn translate_static_array_from_callable
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_static_array_from_callable(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        expr_ie: &StaticArrayFromCallableIE<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        panic!("Unimplemented: translate_static_array_from_callable");
    }
}
/*
  def translateStaticArrayFromCallable(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    exprIE: StaticArrayFromCallableIE):
  (ExpressionH[KindHT]) = {
    val StaticArrayFromCallableIE(arrayType2, generatorExpr2, generatorMethod, _) = exprIE;

    val (generatorRegisterId, generatorDeferreds) =
      translate(
        hinputs, hamuts, currentFunctionHeader, locals, generatorExpr2);

    val (arrayRefTypeH) =
      typeHammer.translateCoord(
        hinputs, hamuts, exprIE.result)

    val (arrayTypeH) =
      typeHammer.translateStaticSizedArray(hinputs, hamuts, arrayType2)
    vassert(arrayRefTypeH.expectStaticSizedArrayCoord().kind == arrayTypeH)

    val elementType = hamuts.getStaticSizedArray(arrayTypeH).elementType

    val generatorMethodH =
      typeHammer.translatePrototype(hinputs, hamuts, generatorMethod)

    val constructArrayCallNode =
      StaticArrayFromCallableH(
        generatorRegisterId,
        generatorMethodH,
        elementType,
        arrayRefTypeH.expectStaticSizedArrayCoord())

    translateDeferreds(
      hinputs, hamuts, currentFunctionHeader, locals, constructArrayCallNode, generatorDeferreds)
  }
*/

// mig: fn translate_destroy_static_sized_array
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_destroy_static_sized_array(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        das2: &DestroyStaticSizedArrayIntoFunctionIE<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        panic!("Unimplemented: translate_destroy_static_sized_array");
    }
}
/*
  def translateDestroyStaticSizedArray(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    das2: DestroyStaticSizedArrayIntoFunctionIE):
  ExpressionH[KindHT] = {
    val DestroyStaticSizedArrayIntoFunctionIE(arrayExpr2, staticSizedArrayType, consumerExpr2, consumerMethod2) = das2;

    val (arrayTypeH) =
      typeHammer.translateStaticSizedArray(hinputs, hamuts, staticSizedArrayType)
    val (arrayRefTypeH) =
      typeHammer.translateCoord(hinputs, hamuts, arrayExpr2.result)
    vassert(arrayRefTypeH.expectStaticSizedArrayCoord().kind == arrayTypeH)

    val (arrayExprResultHE, arrayExprDeferreds) =
      translate(
        hinputs, hamuts, currentFunctionHeader, locals, arrayExpr2);

    val (consumerCallableResultHE, consumerCallableDeferreds) =
      translate(
        hinputs, hamuts, currentFunctionHeader, locals, consumerExpr2);

    val staticSizedArrayDef = hamuts.getStaticSizedArray(arrayTypeH)

    val consumerMethod =
      typeHammer.translatePrototype(hinputs, hamuts, consumerMethod2)

    val destroyStaticSizedArrayCallNode =
      DestroyStaticSizedArrayIntoFunctionH(
        arrayExprResultHE.expectStaticSizedArrayAccess(),
        consumerCallableResultHE,
        consumerMethod,
        staticSizedArrayDef.elementType,
        staticSizedArrayDef.size)

    translateDeferreds(
      hinputs, hamuts, currentFunctionHeader, locals, destroyStaticSizedArrayCallNode, consumerCallableDeferreds ++ arrayExprDeferreds)
  }
*/

// mig: fn translate_destroy_imm_runtime_sized_array
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_destroy_imm_runtime_sized_array(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        das2: &DestroyImmRuntimeSizedArrayIE<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        panic!("Unimplemented: translate_destroy_imm_runtime_sized_array");
    }
}
/*
  def translateDestroyImmRuntimeSizedArray(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    das2: DestroyImmRuntimeSizedArrayIE):
  ExpressionH[KindHT] = {
    val DestroyImmRuntimeSizedArrayIE(arrayExpr2, runtimeSizedArrayType2, consumerExpr2, consumerMethod2) = das2;

    //    val RuntimeSizedArrayT2(RawArrayT2(memberType2, mutability)) = runtimeSizedArrayType2

    val (arrayTypeH) =
      typeHammer.translateRuntimeSizedArray(hinputs, hamuts, runtimeSizedArrayType2)
    val (arrayRefTypeH) =
      typeHammer.translateCoord(hinputs, hamuts, arrayExpr2.result)
    vassert(arrayRefTypeH.expectRuntimeSizedArrayCoord().kind == arrayTypeH)

    val (arrayExprResultHE, arrayExprDeferreds) =
      translate(
        hinputs, hamuts, currentFunctionHeader, locals, arrayExpr2);

    val (consumerCallableResultHE, consumerCallableDeferreds) =
      translate(
        hinputs, hamuts, currentFunctionHeader, locals, consumerExpr2);

    val consumerMethod =
      typeHammer.translatePrototype(hinputs, hamuts, consumerMethod2)

    val elementType =
      hamuts.getRuntimeSizedArray(
        arrayExprResultHE.expectRuntimeSizedArrayAccess().resultType.kind)
        .elementType

    val destroyStaticSizedArrayCallNode =
      DestroyImmRuntimeSizedArrayH(
        arrayExprResultHE.expectRuntimeSizedArrayAccess(),
        consumerCallableResultHE,
        consumerMethod,
        elementType)

    translateDeferreds(
      hinputs, hamuts, currentFunctionHeader, locals, destroyStaticSizedArrayCallNode, consumerCallableDeferreds ++ arrayExprDeferreds)
  }
*/

// mig: fn translate_if
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_if(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        parent_locals: &mut Locals<'s, 'i, 'h>,
        if2: &IfIE<'s, 'i, cI>,
    ) -> ExpressionH<'s, 'h>
    {
        let condition2 = if2.condition;
        let then_block2 = if2.then_call;
        let else_block2 = if2.else_call;
        let (condition_block_h, cond_deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, parent_locals, ExpressionIE::Reference(condition2));
        assert!(cond_deferreds.is_empty());
        assert_eq!(condition_block_h.result_type(), crate::final_ast::types::CoordH { ownership: crate::final_ast::types::OwnershipH::MutableShareH, location: crate::final_ast::types::LocationH::InlineH, kind: crate::final_ast::types::KindHT::BoolHT(crate::final_ast::types::BoolHT) });
        let mut then_locals = parent_locals.snapshot();
        let (then_block_h, then_deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, &mut then_locals, ExpressionIE::Reference(then_block2));
        assert!(then_deferreds.is_empty());
        let then_result_coord = then_block_h.result_type();
        parent_locals.set_next_local_id_number(then_locals.next_local_id_number);
        let mut else_locals = parent_locals.snapshot();
        let (else_block_h, else_deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, &mut else_locals, ExpressionIE::Reference(else_block2));
        assert!(else_deferreds.is_empty());
        let else_result_coord = else_block_h.result_type();
        parent_locals.set_next_local_id_number(else_locals.next_local_id_number);
        let common_supertype_h = self.translate_coord(hinputs, hamuts, if2.result);
        let if_call_node = ExpressionH::IfH(self.interner.alloc(crate::final_ast::instructions::IfH {
            condition_block: condition_block_h.expect_bool_access(),
            then_block: then_block_h,
            else_block: else_block_h,
            common_supertype: common_supertype_h,
        }));
        let then_continues = match then_result_coord.kind { crate::final_ast::types::KindHT::NeverHT(_) => false, _ => true };
        let else_continues = match else_result_coord.kind { crate::final_ast::types::KindHT::NeverHT(_) => false, _ => true };
        let unstackifies_of_parent_locals: std::collections::HashSet<crate::final_ast::instructions::VariableIdH<'s, 'h>> =
            if then_continues && else_continues {
                let parent_locals_after_then: std::collections::HashSet<_> = then_locals.locals.keys().copied().filter(|k| !then_locals.unstackified_vars.contains(k)).collect();
                let parent_locals_after_else: std::collections::HashSet<_> = else_locals.locals.keys().copied().filter(|k| !else_locals.unstackified_vars.contains(k)).collect();
                if parent_locals_after_then != parent_locals_after_else {
                    panic!("Internal error: Mismatch in if branches' parent-unstackifies");
                }
                then_locals.unstackified_vars.iter().copied().collect()
            } else if then_continues {
                then_locals.unstackified_vars.iter().copied().collect()
            } else if else_continues {
                else_locals.unstackified_vars.iter().copied().collect()
            } else {
                std::collections::HashSet::new()
            };
        let parent_locals_to_unstackify: Vec<_> = parent_locals.locals.keys().copied()
            .filter(|k| !parent_locals.unstackified_vars.contains(k))
            .filter(|k| unstackifies_of_parent_locals.contains(k))
            .collect();
        for var in parent_locals_to_unstackify {
            parent_locals.mark_unstackified(var);
        }
        if_call_node
    }
}
/*
  def translateIf(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    parentLocals: LocalsBox,
    if2: IfIE):
  ExpressionH[KindHT] = {
    val IfIE(condition2, thenBlock2, elseBlock2, _) = if2

    val (conditionBlockH, Vector()) =
      translate(hinputs, hamuts, currentFunctionHeader, parentLocals, condition2);
    vassert(conditionBlockH.resultType == CoordH(MutableShareH, InlineH, BoolHT()))

    val thenLocals = LocalsBox(parentLocals.snapshot)
    val (thenBlockH, Vector()) =
      translate(hinputs, hamuts, currentFunctionHeader, thenLocals, thenBlock2);
    val thenResultCoord = thenBlockH.resultType
    parentLocals.setNextLocalIdNumber(thenLocals.nextLocalIdNumber)

    val elseLocals = LocalsBox(parentLocals.snapshot)
    val (elseBlockH, Vector()) =
      translate(hinputs, hamuts, currentFunctionHeader, elseLocals, elseBlock2);
    val elseResultCoord = elseBlockH.resultType
    parentLocals.setNextLocalIdNumber(elseLocals.nextLocalIdNumber)

    val commonSupertypeH =
      typeHammer.translateCoord(hinputs, hamuts, if2.result)

    val ifCallNode = IfH(conditionBlockH.expectBoolAccess(), thenBlockH, elseBlockH, commonSupertypeH)


    val thenContinues = thenResultCoord.kind match { case NeverHT(_) => false case _ => true }
    val elseContinues = elseResultCoord.kind match { case NeverHT(_) => false case _ => true }

    val unstackifiesOfParentLocals =
      if (thenContinues && elseContinues) { // Both continue
        val parentLocalsAfterThen = thenLocals.locals.keySet -- thenLocals.unstackifiedVars
        val parentLocalsAfterElse = elseLocals.locals.keySet -- elseLocals.unstackifiedVars
        // The same outside-if variables should still exist no matter which branch we went down.
        if (parentLocalsAfterThen != parentLocalsAfterElse) {
          vfail("Internal error:\nIn function " + currentFunctionHeader + "\nMismatch in if branches' parent-unstackifies:\nThen branch: " + parentLocalsAfterThen + "\nElse branch: " + parentLocalsAfterElse)
        }
        // Since theyre the same, just arbitrarily use the then.
        thenLocals.unstackifiedVars
      } else if (thenContinues) {
        // Then continues, else does not
        // Throw away any information from the else. But do consider those from the then.
        thenLocals.unstackifiedVars
      } else if (elseContinues) {
        // Else continues, then does not
        elseLocals.unstackifiedVars
      } else {
        // Neither continues, so neither unstackifies things.
        // It also kind of doesnt matter, no code after this will run.
        Set[VariableIdH]()
      }

    val parentLocalsToUnstackify =
    // All the parent locals...
      parentLocals.locals.keySet
        // ...minus the ones that were unstackified before...
        .diff(parentLocals.unstackifiedVars)
        // ...which were unstackified by the branch.
        .intersect(unstackifiesOfParentLocals)
    parentLocalsToUnstackify.foreach(parentLocals.markUnstackified)

    ifCallNode
  }
*/

// mig: fn translate_while
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_while(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        while2: &'i WhileIE<'s, 'i, cI>,
    ) -> &'h WhileH<'s, 'h>
    {
        let body_expr2 = &while2.block;
        let (expr_without_deferreds, deferreds) = self.translate_expression(hinputs, hamuts, current_function_header, locals, ExpressionIE::Reference(ReferenceExpressionIE::Block(body_expr2)));
        let expr = self.translate_deferreds(hinputs, hamuts, current_function_header, locals, expr_without_deferreds, deferreds);
        let while_call_node = self.interner.alloc(crate::final_ast::instructions::WhileH { body_block: expr });
        while_call_node
    }
}
/*
  def translateWhile(
    hinputs: HinputsI, hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    while2: WhileIE):
  WhileH = {

    val WhileIE(bodyExpr2, _) = while2

    val (exprWithoutDeferreds, deferreds) =
      translate(hinputs, hamuts, currentFunctionHeader, locals, bodyExpr2);
    val expr =
      translateDeferreds(hinputs, hamuts, currentFunctionHeader, locals, exprWithoutDeferreds, deferreds)

    val whileCallNode = WhileH(expr)
    whileCallNode
  }
*/

// mig: fn translate_interface_function_call
impl<'s, 'i, 'h, 'ctx> Hammer<'s, 'i, 'h, 'ctx>
where 's: 'h, 's: 'i, 'i: 'h,
{
    pub fn translate_interface_function_call(
        &self,
        hinputs: &HinputsI<'s, 'i>,
        hamuts: &mut Hamuts<'s, 'i, 'h>,
        current_function_header: &FunctionHeaderI<'s, 'i>,
        locals: &mut Locals<'s, 'i, 'h>,
        super_function_prototype: &'i PrototypeI<'s, 'i, cI>,
        virtual_param_index: i32,
        result_type2: CoordI<'s, 'i, cI>,
        args_exprs2: &[ExpressionIE<'s, 'i, cI>],
    ) -> ExpressionH<'s, 'h>
    {
        panic!("Unimplemented: translate_interface_function_call");
    }
}
/*
  def translateInterfaceFunctionCall(
    hinputs: HinputsI,
    hamuts: HamutsBox,
    currentFunctionHeader: FunctionHeaderI,
    locals: LocalsBox,
    superFunctionPrototype: PrototypeI[cI],
    virtualParamIndex: Int,
    resultType2: CoordI[cI],
    argsExprs2: Vector[ExpressionI]):
  ExpressionH[KindHT] = {
    val (argsHE, argsDeferreds) =
      translateExpressionsUntilNever(
        hinputs, hamuts, currentFunctionHeader, locals, argsExprs2);
    // Don't evaluate anything that can't ever be run, see BRCOBS
    if (argsHE.nonEmpty && argsHE.last.resultType.kind == NeverHT(false)) {
      return Hammer.consecrash(locals, argsHE)
    }

//    val virtualParamIndex = superFunctionHeader.getVirtualIndex.get
    val CoordI(_, interfaceIT @ InterfaceIT(_)) =
      superFunctionPrototype.paramTypes(virtualParamIndex)
    val (interfaceRefH) =
      structHammer.translateInterface(hinputs, hamuts, interfaceIT)
    val edge = hinputs.interfaceToEdgeBlueprints(interfaceIT.id)
    vassert(edge.interface == interfaceIT.id)
    val indexInEdge = edge.superFamilyRootHeaders.indexWhere(x => superFunctionPrototype.toSignature == x._1.toSignature)
    vassert(indexInEdge >= 0)

    val (prototypeH) = typeHammer.translatePrototype(hinputs, hamuts, superFunctionPrototype)

    val callNode =
      InterfaceCallH(
        argsHE,
        virtualParamIndex,
        interfaceRefH,
        indexInEdge,
        prototypeH)

    translateDeferreds(
      hinputs, hamuts, currentFunctionHeader, locals, callNode, argsDeferreds)
  }
}
*/
