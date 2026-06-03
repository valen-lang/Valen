use std::cell::Cell;
use std::collections::HashMap;
use std::io::Write;
use std::marker::PhantomData;
use crate::interner::StrI;
use crate::final_ast::types::{KindHT, CoordH, LocationH, OwnershipH, InterfaceHT};
use crate::final_ast::ast::{ProgramH, PrototypeH, FunctionH};
use crate::final_ast::instructions::ExpressionH;
use crate::testvm::values::{
    AllocationIdV, AllocationV, CallIdV, ExpressionIdV, IObjectReferrerV,
    KindV, PrimitiveKindV, ReferenceV, RegisterV, VariableAddressV, VariableV,
};
use crate::testvm::heap::HeapV;

/*
package dev.vale.testvm

import dev.vale.finalast._
import dev.vale.{vassert, vassertOne, vassertSome, vcurious, vfail, vimpl, vregionmut, vwat, finalast => m}
import dev.vale.finalast._

import scala.collection.mutable

object ExpressionVivem {
*/
// mig: enum INodeExecuteResultV
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum INodeExecuteResultV<'v, 'h, 's> {
  Continue(NodeContinueV<'v, 'h, 's>),
  Return(NodeReturnV<'v, 'h, 's>),
  Break(NodeBreakV<'v, 'h, 's>),
}
/*
  // The contained reference has a ResultToObjectReferrer pointing at it.
  // This is so if we do something like [4, 5].0, and that 4 is being
  // returned to the parent node, it's not deallocated from its ref count
  // going to 0.
  sealed trait INodeExecuteResult
*/
// mig: struct NodeContinueV<'v, 'h, 's>
/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct NodeContinueV<'v, 'h, 's> {
  pub result_ref: ReferenceV<'v, 'h, 's>,
}
/*
  case class NodeContinue(resultRef: ReferenceV) extends INodeExecuteResult {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious(); }
*/
// mig: struct NodeReturnV<'v, 'h, 's>
/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct NodeReturnV<'v, 'h, 's> {
  pub return_ref: ReferenceV<'v, 'h, 's>,
}
/*
  case class NodeReturn(returnRef: ReferenceV) extends INodeExecuteResult {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious(); }
*/
// mig: struct NodeBreakV<'v, 'h, 's>
/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct NodeBreakV<'v, 'h, 's>
where 's: 'h, 'h: 'v,
{
  pub _phantom: std::marker::PhantomData<(&'v (), &'h (), &'s ())>,
}
/*
  case class NodeBreak() extends INodeExecuteResult {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash;
override def equals(obj: Any): Boolean = vcurious(); }
*/
// mig: fn make_primitive
pub fn make_primitive<'v, 'h, 's>(heap: &mut HeapV<'v, 'h, 's>, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, call_id: CallIdV<'v, 'h, 's>, location: LocationH, kind: KindV<'v, 'h, 's>) -> ReferenceV<'v, 'h, 's> {
    assert!(!matches!(kind, KindV::Void(_)));
    let r#ref = heap.allocate_transient(interner, OwnershipH::MutableShareH, location, kind);
    heap.increment_reference_ref_count(
        crate::testvm::values::IObjectReferrerV::RegisterToObjectReferrer(
            crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: OwnershipH::MutableShareH }
        ),
        r#ref,
    );
    r#ref
}
/*
  def makePrimitive(heap: Heap, callId: CallId, location: LocationH, kind: KindV) = {
    vassert(kind != VoidV)
    val ref = heap.allocateTransient(MutableShareH, location, kind)
    heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, MutableShareH), ref)
    ref
  }
*/
// mig: fn take_argument
pub fn take_argument<'v, 'h, 's>(heap: &mut HeapV<'v, 'h, 's>, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, call_id: CallIdV<'v, 'h, 's>, argument_index: i32, result_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
    let r#ref = heap.take_argument(interner, call_id, argument_index, result_type);
    heap.increment_reference_ref_count(
        IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: result_type.ownership }),
        r#ref);
    r#ref
}
/*
Guardian: temp-disable: SPDMX — Threading interner (HammerInterner) through this call to feed StructDefinitionH.get_ref(interner) is SPDMX Exception B — the exception text explicitly enumerates interner as a sanctioned arena adaptation parameter. Scala used GC + module-singleton interning; Rust requires explicit parameter threading. Same pattern landed without firing on check_kind/check_reference/add_local/remove_local this turn — Guardian is inconsistent on the same Exception-B shape. — FrontendRust/guardian-logs/request-454-1780421180348/hook-454/take_argument--100.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def takeArgument(heap: Heap, callId: CallId, argumentIndex: Int, resultType: CoordH[KindHT]) = {
    val ref = heap.takeArgument(callId, argumentIndex, resultType)
    heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, resultType.ownership), ref)
    ref
  }
*/
// mig: fn possess_callee_return
pub fn possess_callee_return<'v, 'h, 's>(heap: &mut HeapV<'v, 'h, 's>, call_id: CallIdV<'v, 'h, 's>, callee_call_id: CallIdV<'v, 'h, 's>, result: &NodeReturnV<'v, 'h, 's>) -> ReferenceV<'v, 'h, 's> {
    heap.decrement_reference_ref_count(
        IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id: callee_call_id, ownership: result.return_ref.ownership }),
        result.return_ref,
    );
    heap.increment_reference_ref_count(
        IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: result.return_ref.ownership }),
        result.return_ref,
    );
    result.return_ref
}
/*
  def possessCalleeReturn(heap: Heap, callId: CallId, calleeCallId: CallId, result: NodeReturn) = {
    heap.decrementReferenceRefCount(RegisterToObjectReferrer(calleeCallId, result.returnRef.ownership), result.returnRef)
    heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, result.returnRef.ownership), result.returnRef)
    result.returnRef
  }
*/
// mig: fn upcast
pub fn upcast<'v, 'h, 's>(source_reference: ReferenceV<'v, 'h, 's>, target_interface_ref: InterfaceHT<'s, 'h>) -> ReferenceV<'v, 'h, 's> { panic!("Unimplemented: upcast"); }
/*
  def upcast(sourceReference: ReferenceV, targetInterfaceRef: InterfaceHT): ReferenceV = {
    ReferenceV(
      sourceReference.actualKind,
      RRKind(targetInterfaceRef),
      sourceReference.ownership,
      sourceReference.location,
      sourceReference.num)
  }
*/
// mig: fn execute_node
pub fn execute_node<'v, 'h, 's>(program_h: &'h ProgramH<'s, 'h>, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, scout_arena: &crate::scout_arena::ScoutArena<'s>, stdin: &'v dyn Fn() -> StrI<'s>, stdout: &'v dyn Fn(StrI<'s>), heap: &mut HeapV<'v, 'h, 's>, expression_id: ExpressionIdV<'v, 'h, 's>, node: &ExpressionH<'s, 'h>) -> INodeExecuteResultV<'v, 'h, 's> {
    let node_name = match node {
        ExpressionH::ConstantVoidH(_) => "ConstantVoidH",
        ExpressionH::ConstantIntH(_) => "ConstantIntH",
        ExpressionH::ConstantBoolH(_) => "ConstantBoolH",
        ExpressionH::ConstantStrH(_) => "ConstantStrH",
        ExpressionH::ConstantF64H(_) => "ConstantF64H",
        ExpressionH::ArgumentH(_) => "ArgumentH",
        ExpressionH::StackifyH(_) => "StackifyH",
        ExpressionH::RestackifyH(_) => "RestackifyH",
        ExpressionH::UnstackifyH(_) => "UnstackifyH",
        ExpressionH::DestroyH(_) => "DestroyH",
        ExpressionH::DestroyStaticSizedArrayIntoLocalsH(_) => "DestroyStaticSizedArrayIntoLocalsH",
        ExpressionH::StructToInterfaceUpcastH(_) => "StructToInterfaceUpcastH",
        ExpressionH::InterfaceToInterfaceUpcastH(_) => "InterfaceToInterfaceUpcastH",
        ExpressionH::LocalStoreH(_) => "LocalStoreH",
        ExpressionH::LocalLoadH(_) => "LocalLoadH",
        ExpressionH::MemberStoreH(_) => "MemberStoreH",
        ExpressionH::MemberLoadH(_) => "MemberLoadH",
        ExpressionH::NewArrayFromValuesH(_) => "NewArrayFromValuesH",
        ExpressionH::StaticSizedArrayStoreH(_) => "StaticSizedArrayStoreH",
        ExpressionH::RuntimeSizedArrayStoreH(_) => "RuntimeSizedArrayStoreH",
        ExpressionH::RuntimeSizedArrayLoadH(_) => "RuntimeSizedArrayLoadH",
        ExpressionH::StaticSizedArrayLoadH(_) => "StaticSizedArrayLoadH",
        ExpressionH::CallH(_) => "CallH",
        ExpressionH::ExternCallH(_) => "ExternCallH",
        ExpressionH::InterfaceCallH(_) => "InterfaceCallH",
        ExpressionH::IfH(_) => "IfH",
        ExpressionH::WhileH(_) => "WhileH",
        ExpressionH::ConsecutorH(_) => "ConsecutorH",
        ExpressionH::BlockH(_) => "BlockH",
        ExpressionH::MutabilifyH(_) => "MutabilifyH",
        ExpressionH::ImmutabilifyH(_) => "ImmutabilifyH",
        ExpressionH::ReturnH(_) => "ReturnH",
        ExpressionH::NewImmRuntimeSizedArrayH(_) => "NewImmRuntimeSizedArrayH",
        ExpressionH::NewMutRuntimeSizedArrayH(_) => "NewMutRuntimeSizedArrayH",
        ExpressionH::PushRuntimeSizedArrayH(_) => "PushRuntimeSizedArrayH",
        ExpressionH::PopRuntimeSizedArrayH(_) => "PopRuntimeSizedArrayH",
        ExpressionH::StaticArrayFromCallableH(_) => "StaticArrayFromCallableH",
        ExpressionH::DestroyStaticSizedArrayIntoFunctionH(_) => "DestroyStaticSizedArrayIntoFunctionH",
        ExpressionH::DestroyImmRuntimeSizedArrayH(_) => "DestroyImmRuntimeSizedArrayH",
        ExpressionH::DestroyMutRuntimeSizedArrayH(_) => "DestroyMutRuntimeSizedArrayH",
        ExpressionH::BreakH(_) => "BreakH",
        ExpressionH::NewStructH(_) => "NewStructH",
        ExpressionH::ArrayLengthH(_) => "ArrayLengthH",
        ExpressionH::ArrayCapacityH(_) => "ArrayCapacityH",
        ExpressionH::BorrowToWeakH(_) => "BorrowToWeakH",
        ExpressionH::IsSameInstanceH(_) => "IsSameInstanceH",
        ExpressionH::AsSubtypeH(_) => "AsSubtypeH",
        ExpressionH::LockWeakH(_) => "LockWeakH",
        ExpressionH::DiscardH(_) => "DiscardH",
        ExpressionH::PreCheckBorrowH(_) => "PreCheckBorrowH",
    };
    {
        let handle = &mut *heap.vivem_dout;
        write!(handle, "<{}> ", node_name).unwrap();
    }
    let result = execute_node_inner(program_h, interner, scout_arena, stdin, stdout, heap, expression_id, node);
    {
        let handle = &mut *heap.vivem_dout;
        writeln!(handle, "</{}>", node_name).unwrap();
    }
    result
}
/*
  def executeNode(
    programH: ProgramH,
    stdin: (() => String),
    stdout: (String => Unit),
    heap: Heap,
    expressionId: ExpressionId,
    node: ExpressionH[KindHT] // rename to expression
  ): INodeExecuteResult = {
    heap.vivemDout.print("<" + node.getClass.getSimpleName + "> ")
    val result = executeNodeInner(programH, stdin, stdout, heap, expressionId, node)
    heap.vivemDout.println("</" + node.getClass.getSimpleName + ">")
    result
  }
*/
// mig: fn execute_node_inner
pub fn execute_node_inner<'v, 'h, 's>(program_h: &'h ProgramH<'s, 'h>, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, scout_arena: &crate::scout_arena::ScoutArena<'s>, stdin: &'v dyn Fn() -> StrI<'s>, stdout: &'v dyn Fn(StrI<'s>), heap: &mut HeapV<'v, 'h, 's>, expression_id: ExpressionIdV<'v, 'h, 's>, node: &ExpressionH<'s, 'h>) -> INodeExecuteResultV<'v, 'h, 's> {
    let call_id = expression_id.call_id;
    match node {
        ExpressionH::NewStructH(n) => {
            let crate::final_ast::instructions::NewStructH { source_expressions: args_exprs, target_member_names: _, result_type: struct_ref_h } = **n;
            let struct_kind_h = match struct_ref_h.kind {
                crate::final_ast::types::KindHT::StructHT(s) => s,
                _ => panic!("NewStructH: result_type not StructHT"),
            };
            let struct_def_h = program_h.lookup_struct(interner, struct_kind_h);
            let member_references_vec: Vec<crate::testvm::values::ReferenceV<'v, 'h, 's>> = args_exprs.iter().enumerate().map(|(i, arg_expr)| {
                match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, i as i32), arg_expr) {
                    INodeExecuteResultV::Return(_) => {
                        // do we have to, like, discard the previously made arguments?
                        // what happens with those?
                        panic!("NewStructH arg produced Return — vimpl; return r");
                    }
                    INodeExecuteResultV::Break(_) => panic!("NewStructH arg produced Break — vwat"),
                    INodeExecuteResultV::Continue(c) => c.result_ref,
                }
            }).collect();
            for r in &member_references_vec {
                heap.decrement_reference_ref_count(
                    IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: r.ownership }),
                    *r,
                );
            }
            assert_eq!(member_references_vec.len(), struct_def_h.members.len());
            let member_references: &'v [crate::testvm::values::ReferenceV<'v, 'h, 's>] = heap.vivem_bump.alloc_slice_copy(&member_references_vec);
            let reference = heap.new_struct(interner, *struct_def_h, struct_ref_h, member_references);
            heap.increment_reference_ref_count(
                IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: reference.ownership }),
                reference,
            );
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: reference })
        }
        ExpressionH::ConstantIntH(c) => {
            let crate::final_ast::instructions::ConstantIntH { value, bits } = **c;
            let r#ref = make_primitive(heap, interner, call_id, LocationH::InlineH, KindV::Int(crate::testvm::values::IntV { value, bits, _phantom: std::marker::PhantomData }));
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: r#ref })
        }
        ExpressionH::ReturnH(r) => {
            let source_expr = r.source_expression;
            let source_ref = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &source_expr) {
                ret @ INodeExecuteResultV::Return(_) => return ret,
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Break(_) => panic!("execute_node_inner: ReturnH source produced Break — vwat"),
            };
            INodeExecuteResultV::Return(NodeReturnV { return_ref: source_ref })
        }
        ExpressionH::UnstackifyH(u) => {
            let crate::final_ast::instructions::UnstackifyH { local } = **u;
            let var_address = crate::testvm::heap::get_var_address(expression_id.call_id, local);
            let reference = heap.get_reference_from_local(interner, var_address, local.type_h, local.type_h);
            heap.increment_reference_ref_count(
                IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: reference.ownership }),
                reference,
            );
            {
                let handle = &mut *heap.vivem_dout;
                write!(handle, " ^{}", var_address).unwrap();
            }
            heap.remove_local(interner, var_address, local.type_h);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: reference })
        }
        ExpressionH::StackifyH(s) => {
            let crate::final_ast::instructions::StackifyH { source_expr, local, name: _ } = **s;
            let reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &source_expr) {
                ret @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_)) => return ret,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            let var_addr = crate::testvm::heap::get_var_address(expression_id.call_id, local);
            heap.add_local(interner, var_addr, reference, source_expr.result_type());
            {
                let handle = &mut *heap.vivem_dout;
                write!(handle, " v{}/{}<-o{}", var_addr.call_id.call_depth, var_addr.local.id.number, reference.num).unwrap();
            }
            discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, source_expr.result_type(), reference);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: heap.void() })
        }
        ExpressionH::BlockH(b) => {
            let source_expr = b.inner;
            execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &source_expr)
        }
        ExpressionH::ConsecutorH(c) => {
            let crate::final_ast::instructions::ConsecutorH { exprs: inner_exprs } = **c;
            let mut last_inner_expr_result_ref: Option<crate::testvm::values::ReferenceV<'v, 'h, 's>> = None;
            for (i, inner_expr) in inner_exprs.iter().enumerate() {
                match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, i as i32), inner_expr) {
                    ret @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_)) => return ret,
                    INodeExecuteResultV::Continue(cc) => {
                        if i == inner_exprs.len() - 1 {
                            last_inner_expr_result_ref = Some(cc.result_ref);
                        }
                    }
                }
                {
                    let handle = &mut *heap.vivem_dout;
                    writeln!(handle).unwrap();
                }
            }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: last_inner_expr_result_ref.expect("ConsecutorH: empty innerExprs") })
        }
        ExpressionH::CallH(c) => {
            let crate::final_ast::instructions::CallH { function: prototype_h, args_expressions: args_exprs } = **c;
            let mut arg_refs: Vec<crate::testvm::values::ReferenceV<'v, 'h, 's>> = Vec::new();
            for (i, arg_expr) in args_exprs.iter().enumerate() {
                let r = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, i as i32), arg_expr) {
                    ret @ (INodeExecuteResultV::Break(_) | INodeExecuteResultV::Return(_)) => return ret,
                    INodeExecuteResultV::Continue(c) => c.result_ref,
                };
                arg_refs.push(r);
            }
            let function_h = program_h.lookup_function(prototype_h);
            {
                use std::io::Write;
                let handle = &mut *heap.vivem_dout;
                writeln!(handle).unwrap();
                writeln!(handle, "{}Making new stack frame (call)", "  ".repeat(expression_id.call_id.call_depth as usize)).unwrap();
            }
            for r in arg_refs.iter() {
                heap.decrement_reference_ref_count(
                    crate::testvm::values::IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: r.ownership }),
                    *r);
            }
            let arg_refs_slice: &'v [crate::testvm::values::ReferenceV<'v, 'h, 's>] = heap.vivem_bump.alloc_slice_copy(&arg_refs);
            let (callee_call_id, retuurn) = crate::testvm::function_vivem::execute_function(program_h, interner, scout_arena, stdin, stdout, heap, arg_refs_slice, function_h);
            {
                use std::io::Write;
                let handle = &mut *heap.vivem_dout;
                write!(handle, "{}Getting return reference", "  ".repeat(expression_id.call_id.call_depth as usize)).unwrap();
            }
            let return_ref = possess_callee_return(heap, call_id, callee_call_id, &retuurn);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: return_ref })
        }
        ExpressionH::NewStructH(n) => {
            let crate::final_ast::instructions::NewStructH { source_expressions: args_exprs, target_member_names: _, result_type: struct_ref_h } = **n;
            let struct_def_h = program_h.lookup_struct(interner, match struct_ref_h.kind { crate::final_ast::types::KindHT::StructHT(s) => s, _ => panic!("NewStructH: result_type.kind not StructHT") });
            let mut member_references: Vec<crate::testvm::values::ReferenceV<'v, 'h, 's>> = Vec::new();
            for (i, arg_expr) in args_exprs.iter().enumerate() {
                let r = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, i as i32), arg_expr) {
                    ret @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_)) => return ret,
                    INodeExecuteResultV::Continue(c) => c.result_ref,
                };
                member_references.push(r);
            }
            for r in member_references.iter() {
                heap.decrement_reference_ref_count(
                    crate::testvm::values::IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: r.ownership }),
                    *r);
            }
            assert!(member_references.len() == struct_def_h.members.len());
            let member_references_slice: &'v [crate::testvm::values::ReferenceV<'v, 'h, 's>] = heap.vivem_bump.alloc_slice_copy(&member_references);
            let reference = heap.new_struct(interner, *struct_def_h, struct_ref_h, member_references_slice);
            heap.increment_reference_ref_count(
                crate::testvm::values::IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: reference.ownership }),
                reference);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: reference })
        }
        ExpressionH::ArgumentH(a) => {
            let crate::final_ast::instructions::ArgumentH { result_type, argument_index } = **a;
            let r#ref = take_argument(heap, interner, call_id, argument_index, result_type);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: r#ref })
        }
        ExpressionH::ConstantVoidH(_) => {
            let r#ref = heap.void();
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: r#ref })
        }
        ExpressionH::ConstantBoolH(c) => {
            let crate::final_ast::instructions::ConstantBoolH { value } = **c;
            let r#ref = make_primitive(heap, interner, call_id, LocationH::InlineH, KindV::Bool(crate::testvm::values::BoolV { value, _phantom: std::marker::PhantomData }));
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: r#ref })
        }
        ExpressionH::ConstantF64H(c) => {
            let crate::final_ast::instructions::ConstantF64H { value } = **c;
            let r#ref = make_primitive(heap, interner, call_id, LocationH::InlineH, KindV::Float(crate::testvm::values::FloatV { value, _phantom: std::marker::PhantomData }));
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: r#ref })
        }
        ExpressionH::ConstantStrH(c) => {
            let crate::final_ast::instructions::ConstantStrH { value, _marker: _ } = **c;
            let interned = scout_arena.intern_str(value);
            let r#ref = make_primitive(heap, interner, call_id, LocationH::YonderH, KindV::Str(crate::testvm::values::StrV { value: interned, _phantom: std::marker::PhantomData }));
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: r#ref })
        }
        ExpressionH::DiscardH(d) => {
            let source_expr = d.source_expression;
            match source_expr.result_type().ownership {
                OwnershipH::MutableShareH => {}
                OwnershipH::ImmutableShareH => {}
                OwnershipH::ImmutableBorrowH => {}
                OwnershipH::MutableBorrowH => {}
                OwnershipH::WeakH => {}
                OwnershipH::OwnH => panic!("MatchError: OwnH"),
            }
            let source_ref = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &source_expr) {
                r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_)) => return r,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            // Lots of instructions do this, not just Discard, see DINSIE.
            discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, source_expr.result_type(), source_ref);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: heap.void() })
        }
        ExpressionH::LocalLoadH(ll_ref) => {
            let ll = **ll_ref;
            let crate::final_ast::instructions::LocalLoadH { local, target_ownership, local_name: _name } = ll;
            assert!(target_ownership != OwnershipH::OwnH); // should have been Unstackified instead
            let var_address = crate::testvm::heap::get_var_address(expression_id.call_id, local);
            let reference = heap.get_reference_from_local(interner, var_address, local.type_h, ExpressionH::LocalLoadH(ll_ref).result_type());
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: reference.ownership }), reference);
            {
                let handle = &mut *heap.vivem_dout;
                write!(handle, " *{}", var_address).unwrap();
            }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: reference })
        }
        ExpressionH::CallH(c) => {
            let crate::final_ast::instructions::CallH { function: prototype_h, args_expressions: args_exprs } = **c;
            let arg_refs: Vec<crate::testvm::values::ReferenceV<'v, 'h, 's>> =
                args_exprs.iter().enumerate().map(|(i, arg_expr)| {
                    match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, i as i32), arg_expr) {
                        INodeExecuteResultV::Break(_) | INodeExecuteResultV::Return(_) => panic!("execute_node_inner: CallH arg produced Break/Return — vwat (BRCOBS)"),
                        INodeExecuteResultV::Continue(c) => c.result_ref,
                    }
                }).collect();

            let function_h = program_h.lookup_function(prototype_h);
            {
                let handle = &mut *heap.vivem_dout;
                writeln!(handle).unwrap();
                let prefix = "  ".repeat(expression_id.call_id.call_depth as usize);
                writeln!(handle, "{}Making new stack frame (call)", prefix).unwrap();
            }

            for r in arg_refs.iter() {
                heap.decrement_reference_ref_count(
                    IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: r.ownership }),
                    *r,
                );
            }

            let arg_refs_slice: &'v [crate::testvm::values::ReferenceV<'v, 'h, 's>] = heap.vivem_bump.alloc_slice_copy(&arg_refs);
            let (callee_call_id, retuurn) =
                crate::testvm::function_vivem::execute_function(program_h, interner, scout_arena, stdin, stdout, heap, arg_refs_slice, function_h);
            {
                let handle = &mut *heap.vivem_dout;
                let prefix = "  ".repeat(expression_id.call_id.call_depth as usize);
                write!(handle, "{}Getting return reference", prefix).unwrap();
            }
            let return_ref = possess_callee_return(heap, call_id, callee_call_id, &retuurn);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: return_ref })
        }
        ExpressionH::LocalStoreH(s) => {
            let crate::final_ast::instructions::LocalStoreH { local: local_index, source_expression: source_expr, local_name: name } = **s;
            let reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &source_expr) {
                r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_)) => return r,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            let var_address = crate::testvm::heap::get_var_address(expression_id.call_id, local_index);
            {
                use std::io::Write;
                let handle = &mut *heap.vivem_dout;
                write!(handle, " {}(\"{}\")", var_address, name).unwrap();
                write!(handle, "<-{}", reference.num).unwrap();
            }
            let old_ref = heap.mutate_variable(interner, var_address, reference, source_expr.result_type());
            discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, source_expr.result_type(), reference);
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: old_ref.ownership }), old_ref);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: old_ref })
        }
        ExpressionH::DestroyH(d) => {
            let crate::final_ast::instructions::DestroyH { struct_expression: struct_expr, local_types, local_indices: locals } = **d;
            let struct_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &struct_expr) {
                r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_)) => return r,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            heap.decrement_reference_ref_count(
                IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: struct_expr.result_type().ownership }),
                struct_reference);
            // DDSOT
            heap.ensure_ref_count(interner, struct_reference, Some(&[OwnershipH::OwnH, OwnershipH::MutableBorrowH, OwnershipH::ImmutableBorrowH]), 0);
            let old_member_references = heap.destructure(struct_reference);
            assert!(old_member_references.len() == locals.len());
            for ((member_ref, local_type), local_index) in old_member_references.iter().zip(local_types.iter()).zip(locals.iter()) {
                let var_addr = crate::testvm::heap::get_var_address(expression_id.call_id, *local_index);
                heap.add_local(interner, var_addr, *member_ref, *local_type);
                {
                    use std::io::Write;
                    let handle = &mut *heap.vivem_dout;
                    write!(handle, " v{}<-o{}", var_addr, member_ref.num).unwrap();
                }
            }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: heap.void() })
        }
        ExpressionH::ExternCallH(e) => {
            let crate::final_ast::instructions::ExternCallH { function: prototype_h, args_expressions: args_exprs } = **e;
            let extern_function = crate::testvm::function_vivem::get_extern_function(program_h, prototype_h);
            let arg_refs: Vec<crate::testvm::values::ReferenceV<'v, 'h, 's>> =
                args_exprs.iter().enumerate().map(|(i, arg_expr)| {
                    match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, i as i32), arg_expr) {
                        INodeExecuteResultV::Break(_) | INodeExecuteResultV::Return(_) => panic!("execute_node_inner: ExternCallH arg produced Break/Return — vwat (BRCOBS)"),
                        INodeExecuteResultV::Continue(c) => c.result_ref,
                    }
                }).collect();
            let arg_refs_slice: &'v [crate::testvm::values::ReferenceV<'v, 'h, 's>] = heap.vivem_bump.alloc_slice_copy(&arg_refs);
            let result_ref = {
                let mut adapter = crate::testvm::heap::AdapterForExternsV {
                    program_h,
                    interner,
                    scout_arena,
                    heap: &mut *heap,
                    call_id: crate::testvm::values::CallIdV { call_depth: expression_id.call_id.call_depth + 1, function: prototype_h, _phantom: std::marker::PhantomData },
                    stdin,
                    stdout,
                };
                extern_function(&mut adapter, arg_refs_slice)
            };
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: result_ref.ownership }), result_ref);
            // Special case for externs; externs arent allowed to change ref counts at all.
            // So, we just drop these normally.
            for (r, arg_expr) in arg_refs.iter().zip(args_exprs.iter()) {
                let expected_type = arg_expr.result_type();
                discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, expected_type, *r);
            }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref })
        }
        ExpressionH::IfH(i) => {
            let crate::final_ast::instructions::IfH { condition_block, then_block, else_block, common_supertype: _ } = **i;
            let condition_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &condition_block) {
                ret @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_)) => return ret,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            let condition_kind = heap.dereference(condition_reference, false);
            let condition_value = match condition_kind {
                crate::testvm::values::KindV::Bool(crate::testvm::values::BoolV { value, .. }) => value,
                _ => panic!("execute_node_inner: IfH condition not BoolV"),
            };
            discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, condition_block.result_type(), condition_reference);
            let block_result = if condition_value == true {
                match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 1), &then_block) {
                    ret @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_)) => return ret,
                    INodeExecuteResultV::Continue(c) => c.result_ref,
                }
            } else {
                match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 2), &else_block) {
                    ret @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_)) => return ret,
                    INodeExecuteResultV::Continue(c) => c.result_ref,
                }
            };
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: block_result })
        }
        ExpressionH::MemberStoreH(ms) => {
            let crate::final_ast::instructions::MemberStoreH { result_type: _result_type, struct_expression: struct_expr, member_index, source_expression: source_expr, member_name } = **ms;
            let struct_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &struct_expr) {
                r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_)) => return r,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            let source_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 1), &source_expr) {
                r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_)) => return r,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            let address = crate::testvm::values::MemberAddressV { struct_id: struct_reference.alloc_id(), field_index: member_index };
            {
                let handle = &mut *heap.vivem_dout;
                write!(handle, " {}(\"{}\")", address.to_string(), member_name).unwrap();
                write!(handle, "<-{}", source_reference.num).unwrap();
            }
            let old_member_reference = heap.mutate_struct(address, source_reference, source_expr.result_type());
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: old_member_reference.ownership }), old_member_reference);
            discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, struct_expr.result_type(), struct_reference);
            discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, source_expr.result_type(), source_reference);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: old_member_reference })
        }
        ExpressionH::MemberLoadH(ml) => {
            let crate::final_ast::instructions::MemberLoadH { struct_expression: struct_expr, member_index, expected_member_type, result_type, member_name: _ } = **ml;
            let struct_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &struct_expr) {
                r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_)) => return r,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            let address = crate::testvm::values::MemberAddressV { struct_id: struct_reference.alloc_id(), field_index: member_index };
            {
                let handle = &mut *heap.vivem_dout;
                write!(handle, " *{}", address.to_string()).unwrap();
            }
            let member_reference = heap.get_reference_from_struct(interner, address, expected_member_type, result_type);
            assert!(result_type.ownership != crate::final_ast::types::OwnershipH::OwnH);
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: member_reference.ownership }), member_reference);
            discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, struct_expr.result_type(), struct_reference);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: member_reference })
        }
        ExpressionH::NewArrayFromValuesH(n) => {
            let crate::final_ast::instructions::NewArrayFromValuesH { result_type: array_ref_type, source_expressions: element_exprs } = **n;
            let element_refs: Vec<crate::testvm::values::ReferenceV<'v, 'h, 's>> =
                element_exprs.iter().enumerate().map(|(i, arg_expr)| {
                    match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, i as i32), arg_expr) {
                        INodeExecuteResultV::Return(_) => panic!("execute_node_inner: NewArrayFromValuesH element produced Return — vimpl"),
                        INodeExecuteResultV::Break(_) => panic!("execute_node_inner: NewArrayFromValuesH element produced Break — vimpl"),
                        INodeExecuteResultV::Continue(c) => c.result_ref,
                    }
                }).collect();
            for r in element_refs.iter() {
                heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: r.ownership }), *r);
            }
            let ssa_def = program_h.lookup_static_sized_array(array_ref_type.kind.expect_static_sized_array_ht());
            let element_refs_slice: &'v [crate::testvm::values::ReferenceV<'v, 'h, 's>] = heap.vivem_bump.alloc_slice_copy(&element_refs);
            let (array_reference, array_instance) = heap.add_array(interner, *ssa_def, array_ref_type, element_refs_slice);
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: array_reference.ownership }), array_reference);
            write!(heap.vivem_dout, " o{}=", array_reference.num).unwrap();
            heap.print_kind(crate::testvm::values::KindV::ArrayInstance(array_instance));
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: array_reference })
        }
        ExpressionH::StaticSizedArrayLoadH(ssal) => {
            let crate::final_ast::instructions::StaticSizedArrayLoadH { array_expression: array_expr, index_expression: index_expr, target_ownership, expected_element_type, array_size: _, result_type } = **ssal;
            let array_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &array_expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            let index_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 1), &index_expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            let index = match heap.dereference(index_reference, false) {
                crate::testvm::values::KindV::Int(int_v) if int_v.bits == 32 => int_v.value as i32,
                _ => panic!("execute_node_inner: StaticSizedArrayLoadH index not IntV(_, 32)"),
            };
            let address = crate::testvm::values::ElementAddressV { array_id: array_reference.alloc_id(), element_index: index as i64 };
            write!(heap.vivem_dout, " **o:{}.{}", address.array_id.num, address.element_index).unwrap();
            let source = heap.get_reference_from_array(interner, address, expected_element_type, result_type);
            if target_ownership == crate::final_ast::types::OwnershipH::OwnH {
                panic!("impl me?");
            } else {
            }
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: source.ownership }), source);
            discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, index_expr.result_type(), index_reference);
            discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, array_expr.result_type(), array_reference);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: source })
        }
        ExpressionH::DestroyStaticSizedArrayIntoLocalsH(d) => {
            let crate::final_ast::instructions::DestroyStaticSizedArrayIntoLocalsH { struct_expression: arr_expr, local_types, local_indices: locals } = **d;
            let arr_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &arr_expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: arr_reference.ownership }), arr_reference);
            if arr_expr.result_type().ownership == crate::final_ast::types::OwnershipH::OwnH {
                heap.ensure_ref_count(interner, arr_reference, None, 0);
            } else {
                // Not doing
                //   heap.ensureTotalRefCount(arrReference, 0)
                // for share because we might be taking in a shared reference and not be destroying it.
            }
            let old_member_references = heap.destructure_array(arr_reference);
            if arr_reference.ownership == crate::final_ast::types::OwnershipH::OwnH {
                heap.zero(arr_reference);
                heap.deallocate_if_no_weak_refs(arr_reference);
            }
            assert!(old_member_references.len() == locals.len());
            for ((member_ref, local_type), local_index) in old_member_references.iter().zip(local_types.iter()).zip(locals.iter()) {
                let var_addr = crate::testvm::heap::get_var_address(expression_id.call_id, *local_index);
                heap.add_local(interner, var_addr, *member_ref, *local_type);
                write!(heap.vivem_dout, " v{}<-o{}", var_addr, member_ref.num).unwrap();
            }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: heap.void() })
        }
        ExpressionH::DestroyStaticSizedArrayIntoFunctionH(d) => {
            let crate::final_ast::instructions::DestroyStaticSizedArrayIntoFunctionH { array_expression: array_expr, consumer_expression: consumer_me, consumer_method, array_element_type: _, array_size: _ } = **d;
            let array_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &array_expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            let consumer_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 1), &consumer_me) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            heap.check_reference(interner, consumer_me.result_type(), consumer_reference);
            heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: array_reference.ownership }), array_reference);
            heap.ensure_ref_count(interner, array_reference, None, 0);
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: array_reference.ownership }), array_reference);
            let ssa_def_m = program_h.lookup_static_sized_array(array_expr.result_type().kind.expect_static_sized_array_ht());
            consume_elements(program_h, interner, scout_arena, stdin, stdout, heap, expression_id, call_id, array_reference, consumer_reference, *consumer_method, ssa_def_m.size, &mut |_, _| {});
            heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: array_reference.ownership }), array_reference);
            heap.zero(array_reference);
            heap.deallocate_if_no_weak_refs(array_reference);
            discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, consumer_me.result_type(), consumer_reference);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: heap.void() })
        }
        ExpressionH::BreakH(_) => {
            return INodeExecuteResultV::Break(NodeBreakV { _phantom: std::marker::PhantomData });
        }
        ExpressionH::WhileH(w) => {
            let crate::final_ast::instructions::WhileH { body_block } = **w;
            let mut continuing = true;
            while continuing {
                match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &body_block) {
                    ret @ INodeExecuteResultV::Return(_) => return ret,
                    INodeExecuteResultV::Break(_) => continuing = false,
                    INodeExecuteResultV::Continue(c) => {
                        discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, body_block.result_type(), c.result_ref);
                    }
                }
            }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: heap.void() })
        }
        other => panic!("execute_node_inner: unimplemented arm {:?}", std::mem::discriminant(other)),
    }
}
/*
Guardian: temp-disable: SPDMX — Scala HeapV.dereference has default param allowUndead=false; Rust elided the default per EANODVX, so the explicit false is the Rust adaptation. In-file precedent: heap.rs:54 (AdapterForExternsV::dereference) calls self.heap.dereference(reference, false) with exactly this shape. — FrontendRust/guardian-logs/request-630-1780499508349/hook-630/execute_node_inner--227.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def executeNodeInner(
                   programH: ProgramH,
                   stdin: (() => String),
                   stdout: (String => Unit),
                   heap: Heap,
                   expressionId: ExpressionId,
                   node: ExpressionH[KindHT] // rename to expression
  ): INodeExecuteResult = {
    val callId = expressionId.callId

    node match {
      case PreCheckBorrowH(innerExpr) => {
        val innerRef =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), innerExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        return NodeContinue(innerRef)
      }
      case DiscardH(sourceExpr) => {
        sourceExpr.resultType.ownership match {
          case MutableShareH =>
          case ImmutableShareH =>
          case ImmutableBorrowH =>
          case MutableBorrowH =>
          case WeakH =>
        }
        val sourceRef =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), sourceExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        // Lots of instructions do this, not just Discard, see DINSIE.
        discard(programH, heap, stdout, stdin, callId, sourceExpr.resultType, sourceRef)
        NodeContinue(heap.void)
      }
      case ConstantVoidH() => {
        val ref = heap.void
        NodeContinue(ref)
      }
      case ExternCallH(prototypeH, argsExprs) => {
        val externFunction = FunctionVivem.getExternFunction(programH, prototypeH)

        val argRefs =
          argsExprs.zipWithIndex.map({ case (argExpr, i) =>
            executeNode(programH, stdin, stdout, heap, expressionId.addStep(i), argExpr) match {
              case NodeBreak() | NodeReturn(_) => {
                // This shouldnt be possible because break and return can only
                // be statements, not expressions, see BRCOBS.
                vwat()
              }
              case NodeContinue(r) => r
            }
          })

        val resultRef =
          externFunction(
            new AdapterForExterns(
              programH,
              heap,
              CallId(expressionId.callId.callDepth + 1, prototypeH),
              stdin,
              stdout),
            argRefs.toVector)
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, resultRef.ownership), resultRef)

        // Special case for externs; externs arent allowed to change ref counts at all.
        // So, we just drop these normally.
        argRefs.zip(argsExprs.map(_.resultType))
          .foreach({ case (r, expectedType) => discard(programH, heap, stdout, stdin, callId, expectedType, r) })

        NodeContinue(resultRef)
      }
      case ConstantIntH(value, bits) => {
        val ref = makePrimitive(heap, callId, InlineH, IntV(value, bits))
        NodeContinue(ref)
      }
      case ConstantF64H(value) => {
        val ref = makePrimitive(heap, callId, InlineH, FloatV(value))
        NodeContinue(ref)
      }
      case ConstantStrH(value) => {
        val ref = makePrimitive(heap, callId, YonderH, StrV(value))
        NodeContinue(ref)
      }
      case ConstantBoolH(value) => {
        val ref = makePrimitive(heap, callId, InlineH, BoolV(value))
        NodeContinue(ref)
      }
      case ArgumentH(resultType, argumentIndex) => {
        val ref = takeArgument(heap, callId, argumentIndex, resultType)
        NodeContinue(ref)
      }
      case BreakH() => {
        return NodeBreak()
      }
      case ReturnH(sourceExpr) => {
        val sourceRef =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), sourceExpr) match {
            case r @ NodeReturn(_) => {
              // This can happen if we do for example:
              //   return if (true) {
              //         return 7;
              //       } else {
              //         8
              //       };
              return r
            }
            case NodeContinue(r) => r
          }
        return NodeReturn(sourceRef)
      }
      case IsSameInstanceH(leftExpr, rightExpr) => {
        val leftRef =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), leftExpr) match {
            case r @ NodeReturn(_) => {
              vcurious()
              return r
            }
            case NodeContinue(r) => r
          }
        val rightRef =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(1), rightExpr) match {
            case r @ NodeReturn(_) => {
              vcurious()
              return r
            }
            case NodeContinue(r) => r
          }
        discard(programH, heap, stdout, stdin, callId, leftExpr.resultType, leftRef)
        discard(programH, heap, stdout, stdin, callId, rightExpr.resultType, rightRef)

        val ref = heap.isSameInstance(callId, leftRef, rightRef)

        NodeContinue(ref)
      }
      case MutabilifyH(inner) => {
        val sourceRef =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), inner) match {
            case r@(NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        val resultRef =
          sourceRef.copy(
            ownership =
            sourceRef.ownership match {
              case ImmutableShareH => MutableShareH
              case ImmutableBorrowH => MutableBorrowH
              case other => vwat()
            })
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, resultRef.ownership), resultRef)
        heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, sourceRef.ownership), sourceRef)
        NodeContinue(resultRef)
      }
      case ImmutabilifyH(inner) => {
        val sourceRef =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), inner) match {
            case r@(NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        val resultRef =
          sourceRef.copy(
            ownership =
              sourceRef.ownership match {
                case MutableShareH => ImmutableShareH
                case MutableBorrowH => ImmutableBorrowH
                case other => vwat()
              })
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, resultRef.ownership), resultRef)
        heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, sourceRef.ownership), sourceRef)

        NodeContinue(resultRef)
      }
      case BlockH(sourceExpr) => {
        executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), sourceExpr)
      }
      case ConsecutorH(innerExprs) => {
        var lastInnerExprResultRef: Option[ReferenceV] = None

        for (i <- innerExprs.indices) {
          val innerExpr = innerExprs(i)

          executeNode(programH, stdin, stdout, heap, expressionId.addStep(i), innerExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(innerExprResultRef) => {
              if (i == innerExprs.size - 1) {
                lastInnerExprResultRef = Some(innerExprResultRef)
              }
            }
          }

          heap.vivemDout.println()
        }

        NodeContinue(vassertSome(lastInnerExprResultRef))
      }
      case DestroyH(structExpr, localTypes, locals) => {
        val structReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), structExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }

        heap.decrementReferenceRefCount(
          RegisterToObjectReferrer(callId, structExpr.resultType.ownership),
          structReference)

        // DDSOT
        heap.ensureRefCount(structReference, Some(Set(OwnH, MutableBorrowH, ImmutableBorrowH)), 0)

        val oldMemberReferences = heap.destructure(structReference)

        vassert(oldMemberReferences.size == locals.size)
        oldMemberReferences.zip(localTypes).zip(locals).foreach({ case ((memberRef, localType), localIndex) =>
          val varAddr = heap.getVarAddress(expressionId.callId, localIndex)
          heap.addLocal(varAddr, memberRef, localType)
          heap.vivemDout.print(" v" + varAddr + "<-o" + memberRef.num)
        })
        NodeContinue(heap.void)
      }
      case DestroyStaticSizedArrayIntoLocalsH(arrExpr, localTypes, locals) => {
        val arrReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), arrExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }

        heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, arrReference.ownership), arrReference)

        if (arrExpr.resultType.ownership == OwnH) {
          heap.ensureRefCount(arrReference, None, 0)
        } else {
          // Not doing
          //   heap.ensureTotalRefCount(arrReference, 0)
          // for share because we might be taking in a shared reference and not be destroying it.
        }

        val oldMemberReferences = heap.destructureArray(arrReference)

        if (arrReference.ownership == OwnH) {
          heap.zero(arrReference)
          heap.deallocateIfNoWeakRefs(arrReference)
        }

        vassert(oldMemberReferences.size == locals.size)
        oldMemberReferences.zip(localTypes).zip(locals).foreach({ case ((memberRef, localType), localIndex) =>
          val varAddr = heap.getVarAddress(expressionId.callId, localIndex)
          heap.addLocal(varAddr, memberRef, localType)
          heap.vivemDout.print(" v" + varAddr + "<-o" + memberRef.num)
        })
        NodeContinue(heap.void)
      }
      case ArrayLengthH(arrExpr) => {
        val arrayReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), arrExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }

        val arr @ ArrayInstanceV(_, _, _, _) = heap.dereference(arrayReference)

        discard(programH, heap, stdout, stdin, callId, arrExpr.resultType, arrayReference)

        val lenRef = makePrimitive(heap, callId, InlineH, IntV(arr.getSize(), 32))
        NodeContinue(lenRef)
      }
      case ArrayCapacityH(arrExpr) => {
        val arrayReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), arrExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }

        val arr @ ArrayInstanceV(_, _, _, _) = heap.dereference(arrayReference)

        discard(programH, heap, stdout, stdin, callId, arrExpr.resultType, arrayReference)

        val lenRef = makePrimitive(heap, callId, InlineH, IntV(arr.capacity, 32))
        NodeContinue(lenRef)
      }
      case waH @ BorrowToWeakH(sourceExpr) => {
        val constraintRef =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), sourceExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        vassert(constraintRef.ownership == MutableBorrowH || constraintRef.ownership == ImmutableBorrowH)

        val weakRef = heap.transmute(constraintRef, sourceExpr.resultType, waH.resultType)
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, weakRef.ownership), weakRef)
        discard(programH, heap, stdout, stdin, callId, sourceExpr.resultType, constraintRef)

        NodeContinue(weakRef)
      }
      case AsSubtypeH(sourceExpr, targetKind, resultType, okConstructor, errConstructor) => {
        val sourceRef =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), sourceExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }

        if (sourceRef.actualKind.hamut == targetKind) {
//          val newRef = ReferenceH(BorrowH, YonderH, sourceExpr.resultType.permission, sourceExpr.resultType.kind)
          val refAliasedAsSubtype = heap.transmute(sourceRef, sourceExpr.resultType, okConstructor.params.head)

          heap.vivemDout.println()
          heap.vivemDout.println("  " * expressionId.callId.callDepth + "Making new stack frame (lock call)")

          val function = programH.lookupFunction(okConstructor)
          // The receiver should increment with their own arg referrers.
          heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, sourceRef.ownership), sourceRef)

          val (calleeCallId, retuurn) =
            FunctionVivem.executeFunction(
              programH, stdin, stdout, heap, Vector(refAliasedAsSubtype), function)
          heap.vivemDout.print("  " * expressionId.callId.callDepth + "Getting return reference")

          val returnRef = possessCalleeReturn(heap, callId, calleeCallId, retuurn)

          NodeContinue(upcast(returnRef, resultType.kind))
        } else {
          heap.vivemDout.println()
          heap.vivemDout.println("  " * expressionId.callId.callDepth + "Making new stack frame (lock call)")

          val function = programH.lookupFunction(errConstructor)
          // The receiver should increment with their own arg referrers.
          heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, sourceRef.ownership), sourceRef)

          val (calleeCallId, retuurn) =
            FunctionVivem.executeFunction(
              programH, stdin, stdout, heap, Vector(sourceRef), function)
          heap.vivemDout.print("  " * expressionId.callId.callDepth + "Getting return reference")

          val returnRef = possessCalleeReturn(heap, callId, calleeCallId, retuurn)

          NodeContinue(upcast(returnRef, resultType.kind))
        }
      }
      case LockWeakH(sourceExpr, resultType, someConstructor, noneConstructor) => {
        val weakRef =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), sourceExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        vassert(weakRef.ownership == WeakH)

        if (heap.containsLiveObject(weakRef)) {
          val expectedRef = CoordH(vregionmut(MutableBorrowH), YonderH, sourceExpr.resultType.kind)
          val constraintRef = heap.transmute(weakRef, sourceExpr.resultType, expectedRef)

          heap.vivemDout.println()
          heap.vivemDout.println("  " * expressionId.callId.callDepth + "Making new stack frame (lock call)")

          val function = programH.lookupFunction(someConstructor)
          // The receiver should increment with their own arg referrers.
          heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, weakRef.ownership), weakRef)

          val (calleeCallId, retuurn) =
            FunctionVivem.executeFunction(
              programH, stdin, stdout, heap, Vector(constraintRef), function)
          heap.vivemDout.print("  " * expressionId.callId.callDepth + "Getting return reference")

          val returnRef = possessCalleeReturn(heap, callId, calleeCallId, retuurn)

          NodeContinue(upcast(returnRef, resultType.kind))
        } else {
          discard(programH, heap, stdout, stdin, callId, sourceExpr.resultType, weakRef)

          heap.vivemDout.println()
          heap.vivemDout.println("  " * expressionId.callId.callDepth + "Making new stack frame (lock call)")

          val function = programH.lookupFunction(noneConstructor)

          val (calleeCallId, retuurn) =
            FunctionVivem.executeFunction(
              programH, stdin, stdout, heap, Vector(), function)
          heap.vivemDout.print("  " * expressionId.callId.callDepth + "Getting return reference")

          val returnRef = possessCalleeReturn(heap, callId, calleeCallId, retuurn)
          NodeContinue(upcast(returnRef, resultType.kind))
        }
      }
      case StackifyH(sourceExpr, localIndex, name) => {
        val reference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), sourceExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }

        val varAddr = heap.getVarAddress(expressionId.callId, localIndex)
        heap.addLocal(varAddr, reference, sourceExpr.resultType)
        heap.vivemDout.print(" v" + varAddr + "<-o" + reference.num)

        discard(programH, heap, stdout, stdin, callId, sourceExpr.resultType, reference)

        NodeContinue(heap.void)
      }
      case RestackifyH(sourceExpr, localIndex, name) => {
        val reference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), sourceExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }

        val varAddr = heap.getVarAddress(expressionId.callId, localIndex)
        heap.addLocal(varAddr, reference, sourceExpr.resultType)
        heap.vivemDout.print(" v" + varAddr + "<-o" + reference.num)

        discard(programH, heap, stdout, stdin, callId, sourceExpr.resultType, reference)

        NodeContinue(heap.void)
      }
      case LocalStoreH(localIndex, sourceExpr, name) => {
        val reference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), sourceExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }

        val varAddress = heap.getVarAddress(expressionId.callId, localIndex)
        heap.vivemDout.print(" " + varAddress + "(\"" + name + "\")")
        heap.vivemDout.print("<-" + reference.num)
        val oldRef = heap.mutateVariable(varAddress, reference, sourceExpr.resultType)

        discard(programH, heap, stdout, stdin, callId, sourceExpr.resultType, reference)

        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, oldRef.ownership), oldRef)
        NodeContinue(oldRef)
      }

      case MemberStoreH(resultType, structExpr, memberIndex, sourceExpr, memberName) => {
        val structReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), structExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        val sourceReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(1), sourceExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }

        val address = MemberAddressV(structReference.allocId, memberIndex)
        heap.vivemDout.print(" " + address + "(\"" + memberName + "\")")
        heap.vivemDout.print("<-" + sourceReference.num)
        val oldMemberReference = heap.mutateStruct(address, sourceReference, sourceExpr.resultType)
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, oldMemberReference.ownership), oldMemberReference)

        discard(programH, heap, stdout, stdin, callId, structExpr.resultType, structReference)
        discard(programH, heap, stdout, stdin, callId, sourceExpr.resultType, sourceReference)

        NodeContinue(oldMemberReference)
      }

      case RuntimeSizedArrayStoreH(arrayExpr, indexExpr, sourceExpr, resultType) => {
        val arrayReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), arrayExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        val indexReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(1), indexExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        val sourceReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(2), sourceExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        val IntV(elementIndex, 32) = heap.dereference(indexReference)

        val address = ElementAddressV(arrayReference.allocId, elementIndex.toInt)
        heap.vivemDout.print(" " + address)
        heap.vivemDout.print("<-" + sourceReference.num)
        val oldMemberReference = heap.mutateArray(address, sourceReference, sourceExpr.resultType)
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, oldMemberReference.ownership), oldMemberReference)

        discard(programH, heap, stdout, stdin, callId, sourceExpr.resultType, sourceReference)
        discard(programH, heap, stdout, stdin, callId, indexExpr.resultType, indexReference)
        discard(programH, heap, stdout, stdin, callId, arrayExpr.resultType, arrayReference)
        NodeContinue(oldMemberReference)
      }

      case StaticSizedArrayStoreH(structExpr, indexExpr, sourceExpr, resultType) => {
        vimpl()
//        val indexReference = heap.takeReferenceFromExpr(ExprId(blockId, indexExpr.exprId), indexExpr.resultType)
//        val arrayReference = heap.takeReferenceFromExpr(ExprId(blockId, structExpr.exprId), structExpr.resultType)
//        val IntV(elementIndex) = heap.dereference(indexReference)
//
//        val address = ElementAddressV(arrayReference.allocId, elementIndex)
//        val reference = heap.takeReferenceFromExpr(ExprId(blockId, sourceExpr.exprId), sourceExpr.resultType)
//        heap.vivemDout.print(" " + address)
//        heap.vivemDout.print("<-" + reference.num)
//        val oldMemberReference = heap.mutateArray(address, reference, sourceExpr.resultType)
//        heap.setReferenceExpr(exprId, oldMemberReference)
//        NodeContinue(exprId))
      }

      case ll @ LocalLoadH(local, targetOwnership, name) => {
        vassert(targetOwnership != OwnH) // should have been Unstackified instead
        val varAddress = heap.getVarAddress(expressionId.callId, local)
        val reference = heap.getReferenceFromLocal(varAddress, local.typeH, ll.resultType)
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, reference.ownership), reference)
        heap.vivemDout.print(" *" + varAddress)
        NodeContinue(reference)
      }

      case UnstackifyH(local) => {
        val varAddress = heap.getVarAddress(expressionId.callId, local)
        val reference = heap.getReferenceFromLocal(varAddress, local.typeH, local.typeH)
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, reference.ownership), reference)
        heap.vivemDout.print(" ^" + varAddress)
        heap.removeLocal(varAddress, local.typeH)
        NodeContinue(reference)
      }
      case CallH(prototypeH, argsExprs) => {
        val argRefs =
          argsExprs.zipWithIndex.map({ case (argExpr, i) =>
            executeNode(programH, stdin, stdout, heap, expressionId.addStep(i), argExpr) match {
              case NodeBreak() | NodeReturn(_) => {
                // This shouldnt be possible because break and return can only
                // be statements, not expressions, see BRCOBS.
                vwat()
              }
              case NodeContinue(r) => r
            }
          })

        val functionH = programH.lookupFunction(prototypeH)
//        if (functionH.isExtern) {
//          val externFunction = FunctionVivem.getExternFunction(programH, prototypeH)
//
//          val resultRef =
//            externFunction(
//              new AdapterForExterns(
//                programH,
//                heap,
//                CallId(expressionId.callId.callDepth + 1, prototypeH),
//                stdin,
//                stdout),
//              argRefs.toVector)
//          heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, resultRef.ownership), resultRef)
//
//          // Special case for externs; externs arent allowed to change ref counts at all.
//          // So, we just drop these normally.
//          argRefs.zip(argsExprs.map(_.resultType))
//            .foreach({ case (r, expectedType) => discard(programH, heap, stdout, stdin, callId, expectedType, r) })
//
//          NodeContinue(resultRef)
//        } else {
          heap.vivemDout.println()
          heap.vivemDout.println("  " * expressionId.callId.callDepth + "Making new stack frame (call)")

          // The receiver should increment with their own arg referrers.
          argRefs.foreach(r => heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, r.ownership), r))

          val (calleeCallId, retuurn) =
            FunctionVivem.executeFunction(
              programH, stdin, stdout, heap, argRefs.toVector, functionH)
          heap.vivemDout.print("  " * expressionId.callId.callDepth + "Getting return reference")

          val returnRef = possessCalleeReturn(heap, callId, calleeCallId, retuurn)
          NodeContinue(returnRef)
//        }
      }
      case InterfaceCallH(argsExprs, virtualParamIndex, interfaceRefH, indexInEdge, functionType) => {
        // undeviewed = not deviewed = the virtual param is still a view and we want it to
        // be a struct.
        val undeviewedArgReferences =
          argsExprs.zipWithIndex.map({ case (argExpr, i) =>
            executeNode(programH, stdin, stdout, heap, expressionId.addStep(i), argExpr) match {
              case r @ NodeReturn(_) => {
                vimpl() // do we have to, like, discard the previously made arguments?
                // what happens with those?
                return r
              }
              case NodeContinue(r) => r
            }
          })

        heap.vivemDout.println()
        heap.vivemDout.println("  " * callId.callDepth + "Making new stack frame (icall)")

        // The receiver should increment with their own arg referrers.
        undeviewedArgReferences.foreach(r => heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, r.ownership), r))

        val (functionH, (calleeCallId, retuurn)) =
          executeInterfaceFunction(programH, stdin, stdout, heap, undeviewedArgReferences, virtualParamIndex, interfaceRefH, indexInEdge, functionType)

        val returnRef = possessCalleeReturn(heap, callId, calleeCallId, retuurn)
        NodeContinue(returnRef)
      }
      case NewStructH(argsExprs, targetMemberNames, structRefH) => {
        val structDefH = programH.lookupStruct(structRefH.kind)

        val memberReferences =
          argsExprs.zipWithIndex.map({ case (argExpr, i) =>
            executeNode(programH, stdin, stdout, heap, expressionId.addStep(i), argExpr) match {
              case r @ NodeReturn(_) => {
                vimpl() // do we have to, like, discard the previously made arguments?
                // what happens with those?
                return r
              }
              case NodeContinue(r) => r
            }
          })

        memberReferences.foreach(r => heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, r.ownership), r))

        vassert(memberReferences.size == structDefH.members.size)
        val reference = heap.newStruct(structDefH, structRefH, memberReferences)
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, reference.ownership), reference)

        NodeContinue(reference)
      }
      case NewArrayFromValuesH(arrayRefType, elementExprs) => {
        val elementRefs =
          elementExprs.zipWithIndex.map({ case (argExpr, i) =>
            executeNode(programH, stdin, stdout, heap, expressionId.addStep(i), argExpr) match {
              case r @ NodeReturn(_) => {
                vimpl() // do we have to, like, discard the previously made arguments?
                // what happens with those?
                return r
              }
              case NodeContinue(r) => r
            }
          })

        elementRefs.foreach(r => heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, r.ownership), r))

        val ssaDef = programH.lookupStaticSizedArray(arrayRefType.kind)
        val (arrayReference, arrayInstance) =
          heap.addArray(ssaDef, arrayRefType, elementRefs)
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, arrayReference.ownership), arrayReference)

        heap.vivemDout.print(" o" + arrayReference.num + "=")
        heap.printKind(arrayInstance)
        NodeContinue(arrayReference)
      }

      case ml @ MemberLoadH(structExpr, memberIndex, expectedMemberType, resultType, memberName) => {
        val structReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), structExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }

        val address = MemberAddressV(structReference.allocId, memberIndex)

        heap.vivemDout.print(" *" + address)
        val memberReference = heap.getReferenceFromStruct(address, expectedMemberType, ml.resultType)
        vassert(resultType.ownership != OwnH)
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, memberReference.ownership), memberReference)

        discard(programH, heap, stdout, stdin, callId, structExpr.resultType, structReference)
        NodeContinue(memberReference)
      }

      case rsal @ RuntimeSizedArrayLoadH(arrayExpr, indexExpr, targetOwnership, expectedElementType, resultType) => {
        val arrayReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), arrayExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        val indexIntReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(1), indexExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }

        val index =
          heap.dereference(indexIntReference) match {
            case IntV(value, 32) => value.toInt
          }

        val address = ElementAddressV(arrayReference.allocId, index)

        heap.vivemDout.print(" *" + address)
        val source = heap.getReferenceFromArray(address, expectedElementType, resultType)
        if (targetOwnership == OwnH) {
          vfail("impl me?")
        } else {
        }
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, source.ownership), source)

        discard(programH, heap, stdout, stdin, callId, indexExpr.resultType, indexIntReference)
        discard(programH, heap, stdout, stdin, callId, arrayExpr.resultType, arrayReference)
        NodeContinue(source)
      }

      case StaticSizedArrayLoadH(arrayExpr, indexExpr, targetOwnership, expectedElementType, arraySize, resultType) => {
        val arrayReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), arrayExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        val indexReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(1), indexExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        val index =
          heap.dereference(indexReference) match {
            case IntV(value, 32) => value.toInt
          }

        val address = ElementAddressV(arrayReference.allocId, index)

        heap.vivemDout.print(" *" + address)
        val source = heap.getReferenceFromArray(address, expectedElementType, resultType)
        if (targetOwnership == OwnH) {
          vfail("impl me?")
        } else {
        }
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, source.ownership), source)

        discard(programH, heap, stdout, stdin, callId, indexExpr.resultType, indexReference)
        discard(programH, heap, stdout, stdin, callId, arrayExpr.resultType, arrayReference)
        NodeContinue(source)
      }
      case siu @ StructToInterfaceUpcastH(sourceExpr, targetInterfaceRef) => {
        val sourceReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), sourceExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }

        val targetReference = upcast(sourceReference, targetInterfaceRef)
        NodeContinue(targetReference)
      }
      case IfH(conditionBlock, thenBlock, elseBlock, commonSupertype) => {
        val conditionReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), conditionBlock) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        val conditionKind = heap.dereference(conditionReference)
        val BoolV(conditionValue) = conditionKind;

        discard(programH, heap, stdout, stdin, callId, conditionBlock.resultType, conditionReference)

        val blockResult =
          if (conditionValue == true) {
            executeNode(programH, stdin, stdout, heap, expressionId.addStep(1), thenBlock) match {
              case r @ (NodeReturn(_) | NodeBreak()) => return r
              case NodeContinue(r) => r
            }
          } else {
            executeNode(programH, stdin, stdout, heap, expressionId.addStep(2), elseBlock) match {
              case r @ (NodeReturn(_) | NodeBreak()) => return r
              case NodeContinue(r) => r
            }
          }
        NodeContinue(blockResult)
      }
      case WhileH(bodyBlock) => {
        var continue = true
        while (continue) {
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), bodyBlock) match {
            case r @ NodeReturn(_) => return r
            case NodeBreak() => continue = false
            case NodeContinue(r) => {
              discard(programH, heap, stdout, stdin, callId, bodyBlock.resultType, r)
            }
          }
        }
        NodeContinue(heap.void)
      }
      case cac @ NewImmRuntimeSizedArrayH(sizeExpr, generatorExpr, generatorPrototype, _, arrayRefType) => {
        val sizeReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), sizeExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        val sizeKind = heap.dereference(sizeReference)
        val IntV(size, 32) = sizeKind;
        val rsaDef = programH.lookupRuntimeSizedArray(arrayRefType.kind)
        val (arrayReference, arrayInstance) =
          heap.addUninitializedArray(rsaDef, arrayRefType, size.toInt)
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, arrayReference.ownership), arrayReference)

        val generatorReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), generatorExpr) match {
            case nr @ NodeReturn(_) => return nr
            case NodeContinue(v) => v
          }

        generateElements(
          programH, stdin, stdout, heap, expressionId, callId, generatorReference, generatorPrototype, size.toInt,
          (i, elementRef) => {
            // No need to increment or decrement, we're conceptually moving the return value
            // from the return slot to the array slot
            heap.initializeArrayElement(arrayReference, elementRef)
          })

        discard(programH, heap, stdout, stdin, callId, generatorExpr.resultType, generatorReference)
        discard(programH, heap, stdout, stdin, callId, sizeExpr.resultType, sizeReference)

        heap.vivemDout.print(" o" + arrayReference.num + "=")
        heap.printKind(arrayInstance)

        NodeContinue(arrayReference)
      }

      case NewMutRuntimeSizedArrayH(capacityHE, elementHT, arrayRefType) => {
        val capacityReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), capacityHE) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        val capacityValue = heap.dereference(capacityReference)
        val IntV(capacity, 32) = capacityValue;

        val rsaDef = programH.lookupRuntimeSizedArray(arrayRefType.kind)
        val (arrayReference, arrayInstance) =
          heap.addUninitializedArray(rsaDef, arrayRefType, capacity.toInt)
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, arrayReference.ownership), arrayReference)

        discard(programH, heap, stdout, stdin, callId, capacityHE.resultType, capacityReference)

        heap.vivemDout.print(" o" + arrayReference.num + "=")
        heap.printKind(arrayInstance)
        NodeContinue(arrayReference)
      }

      case PushRuntimeSizedArrayH(arrayHE, newcomerHE) => {
        val arrayReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), arrayHE) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        val rsaDef = programH.lookupRuntimeSizedArray(arrayHE.resultType.kind)
        vassert(rsaDef.elementType == newcomerHE.resultType)
        val rsa @ ArrayInstanceV(_, _, _, _) = heap.dereference(arrayReference)

        val newcomerReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), newcomerHE) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        val newcomerVE = heap.dereference(newcomerReference)

        heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, newcomerReference.ownership), newcomerReference)
        heap.initializeArrayElement(arrayReference, newcomerReference)

        discard(programH, heap, stdout, stdin, callId, arrayHE.resultType, arrayReference)

        heap.vivemDout.print(" o" + arrayReference.num + "+=")
        heap.printKind(newcomerVE)
        NodeContinue(heap.void)
      }

      case PopRuntimeSizedArrayH(arrayHE, elementType) => {
        val arrayReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), arrayHE) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
//        val rsaDef = programH.lookupRuntimeSizedArray(arrayHE.resultType.kind)
//        val rsa @ ArrayInstanceV(_, _, _, _) = heap.dereference(arrayReference)

        val resultReference = heap.deinitializeArrayElement(arrayReference)
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, resultReference.ownership), resultReference)
        val resultValue = heap.dereference(resultReference)

        discard(programH, heap, stdout, stdin, callId, arrayHE.resultType, arrayReference)

        heap.vivemDout.print(" o" + arrayReference.num + "-=")
        heap.printKind(resultValue)
        NodeContinue(resultReference)
      }

      case cac @ StaticArrayFromCallableH(generatorExpr, generatorPrototype, _, arrayRefType) => {
        val ssaDef = programH.lookupStaticSizedArray(arrayRefType.kind)

        val generatorReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), generatorExpr) match {
            case nr @ NodeReturn(_) => return nr
            case NodeContinue(v) => v
          }

        val elementRefs = mutable.MutableList[ReferenceV]()

        generateElements(
          programH, stdin, stdout, heap, expressionId, callId, generatorReference, generatorPrototype, ssaDef.size,
          (i, elementRef) => {
            // No need to increment or decrement, we're conceptually moving the return value
            // from the return slot to the array slot
            elementRefs += elementRef
          })

        discard(programH, heap, stdout, stdin, callId, generatorExpr.resultType, generatorReference)

        val (arrayReference, arrayInstance) =
          heap.addArray(ssaDef, arrayRefType, elementRefs.toVector)
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, arrayReference.ownership), arrayReference)

        heap.vivemDout.print(" o" + arrayReference.num + "=")
        heap.printKind(arrayInstance)
        NodeContinue(arrayReference)
      }

      case DestroyStaticSizedArrayIntoFunctionH(arrayExpr, consumerME, consumerMethod, arrayElementType, arraySize) => {
        val arrayReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), arrayExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }

        val consumerReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(1), consumerME) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }
        heap.checkReference(consumerME.resultType, consumerReference)

        // Temporarily reduce to 0. We do this instead of ensure(1) to better detect a bug
        // where there might be one different kind of referrer.
        heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, arrayReference.ownership), arrayReference)
        heap.ensureRefCount(arrayReference, None, 0)
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, arrayReference.ownership), arrayReference)

        val ssaDefM = programH.lookupStaticSizedArray(arrayExpr.resultType.kind)

        consumeElements(
          programH, stdin, stdout, heap, expressionId, callId, arrayReference, consumerReference, consumerMethod, ssaDefM.size, (_, _) => {})

        heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, arrayReference.ownership), arrayReference)
        heap.zero(arrayReference)
        heap.deallocateIfNoWeakRefs(arrayReference)

        discard(programH, heap, stdout, stdin, callId, consumerME.resultType, consumerReference)

        NodeContinue(heap.void)
      }

      case cac @ DestroyImmRuntimeSizedArrayH(arrayExpr, consumerInterfaceExpr, consumerMethod, arrayElementType) => {
        vcurious()
      }

      case DestroyMutRuntimeSizedArrayH(arrayExpr) => {
        val arrayReference =
          executeNode(programH, stdin, stdout, heap, expressionId.addStep(0), arrayExpr) match {
            case r @ (NodeReturn(_) | NodeBreak()) => return r
            case NodeContinue(r) => r
          }

        // Temporarily reduce to 0. We do this instead of ensure(1) to better detect a bug
        // where there might be one different kind of referrer.
        heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, arrayReference.ownership), arrayReference)
        heap.ensureRefCount(arrayReference, None, 0)
        heap.incrementReferenceRefCount(RegisterToObjectReferrer(callId, arrayReference.ownership), arrayReference)

        val elements =
          heap.dereference(arrayReference) match {
            case ArrayInstanceV(_, _, _, elements) => elements
          }
        vassert(elements.size == 0)

        heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, arrayReference.ownership), arrayReference)
        heap.zero(arrayReference)
        heap.deallocateIfNoWeakRefs(arrayReference)

        NodeContinue(heap.void)
      }
    }
  }
*/
// mig: fn consume_elements
pub fn consume_elements<'v, 'h, 's>(program_h: &'h ProgramH<'s, 'h>, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, scout_arena: &crate::scout_arena::ScoutArena<'s>, stdin: &'v dyn Fn() -> StrI<'s>, stdout: &'v dyn Fn(StrI<'s>), heap: &mut HeapV<'v, 'h, 's>, _expression_id: ExpressionIdV<'v, 'h, 's>, call_id: CallIdV<'v, 'h, 's>, array_reference: ReferenceV<'v, 'h, 's>, consumer_reference: ReferenceV<'v, 'h, 's>, consumer_prototype: PrototypeH<'s, 'h>, size: i64, receiver: &mut dyn FnMut(i64, ReferenceV<'v, 'h, 's>)) {
    let consumer_function = program_h.lookup_function(&consumer_prototype);
    for i in (0..size).rev() {
        writeln!(heap.vivem_dout).unwrap();
        let prefix = "  ".repeat(call_id.call_depth as usize);
        writeln!(heap.vivem_dout, "{}Making new stack frame (consumer)", prefix).unwrap();
        writeln!(heap.vivem_dout).unwrap();
        let element_addr = crate::testvm::values::ElementAddressV { array_id: array_reference.alloc_id(), element_index: i };
        write!(heap.vivem_dout, " *{}", element_addr.to_string()).unwrap();
        let element_reference = heap.deinitialize_array_element(array_reference);
        writeln!(heap.vivem_dout).unwrap();
        writeln!(heap.vivem_dout, "{}Making new stack frame (icall)", prefix).unwrap();
        let args_vec: Vec<ReferenceV<'v, 'h, 's>> = vec![consumer_reference, element_reference];
        let args_slice: &'v [ReferenceV<'v, 'h, 's>] = heap.vivem_bump.alloc_slice_copy(&args_vec);
        let (callee_call_id, retuurn) = crate::testvm::function_vivem::execute_function(program_h, interner, scout_arena, stdin, stdout, heap, args_slice, consumer_function);
        write!(heap.vivem_dout, "{}Getting return reference", prefix).unwrap();
        let return_ref = possess_callee_return(heap, call_id, callee_call_id, &retuurn);
        heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: return_ref.ownership }), return_ref);
        receiver(i, return_ref);
    }
}
/*
Guardian: temp-disable: SPDMX — Per SPDMX Exception B / architect 2026-06-03 ruling, threading scout_arena/interner through vivem entry chain is required because execute_function takes them. Same shape as execute_node/execute_node_inner/inner_execute precedent. — FrontendRust/guardian-logs/request-127-1780519533282/hook-127/consume_elements--1664.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  private def consumeElements(
    programH: ProgramH,
    stdin: () => String,
    stdout: String => Unit,
    heap: Heap,
    expressionId: ExpressionId,
    callId: CallId,
    arrayReference: ReferenceV,
    consumerReference: ReferenceV,
    consumerPrototype: PrototypeH,
    size: Long,
    receiver: (Long, ReferenceV) => Unit):
  Unit = {
    val consumerFunction = programH.lookupFunction(consumerPrototype)

    (0L until size).reverse.foreach(i => {
      heap.vivemDout.println()
      heap.vivemDout.println("  " * callId.callDepth + "Making new stack frame (consumer)")

//      val indexReference = heap.allocateTransient(ShareH, InlineH, IntV(i, 32))

      heap.vivemDout.println()

      heap.vivemDout.print(" *" + ElementAddressV(arrayReference.allocId, i))
      val elementReference = heap.deinitializeArrayElement(arrayReference)
      // Not incrementing ref count here because we're moving it from the array

      heap.vivemDout.println()
      heap.vivemDout.println("  " * callId.callDepth + "Making new stack frame (icall)")

      val (calleeCallId, retuurn) =
        FunctionVivem.executeFunction(
          programH,
          stdin,
          stdout,
          heap,
          Vector(consumerReference, elementReference),
          consumerFunction)

      heap.vivemDout.print("  " * callId.callDepth + "Getting return reference")

      val returnRef = possessCalleeReturn(heap, callId, calleeCallId, retuurn)

      // This decrements it, but does not discard it.
      heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, returnRef.ownership), returnRef)

      receiver(i, returnRef)
    });
  }
*/
// mig: fn generate_elements
pub fn generate_elements<'v, 'h, 's>(program_h: &ProgramH<'s, 'h>, stdin: &dyn Fn() -> String, stdout: &dyn Fn(String), heap: &mut HeapV<'v, 'h, 's>, expression_id: ExpressionIdV<'v, 'h, 's>, call_id: CallIdV<'v, 'h, 's>, generator_reference: ReferenceV<'v, 'h, 's>, generator_prototype: PrototypeH<'s, 'h>, size: i64, receiver: &mut dyn FnMut(i64, ReferenceV<'v, 'h, 's>)) { panic!("Unimplemented: generate_elements"); }
/*
  private def generateElements(
    programH: ProgramH,
    stdin: () => String,
    stdout: String => Unit,
    heap: Heap,
    expressionId: ExpressionId,
    callId: CallId,
    generatorReference: ReferenceV,
    generatorPrototype: PrototypeH,
    size: Long,
    receiver: (Long, ReferenceV) => Unit):
  Unit = {
    val generatorFunction = programH.lookupFunction(generatorPrototype)

    (0L until size).foreach(i => {
      heap.vivemDout.println()
      heap.vivemDout.println("  " * callId.callDepth + "Making new stack frame (generator)")

      val indexReference = heap.allocateTransient(MutableShareH, InlineH, IntV(i, 32))

      heap.vivemDout.println()

      heap.vivemDout.println()
      heap.vivemDout.println("  " * callId.callDepth + "Making new stack frame (icall)")

      val (calleeCallId, retuurn) =
        FunctionVivem.executeFunction(
          programH,
          stdin,
          stdout,
          heap,
          Vector(generatorReference, indexReference),
          generatorFunction)

      heap.vivemDout.print("  " * callId.callDepth + "Getting return reference")

      val returnRef = possessCalleeReturn(heap, callId, calleeCallId, retuurn)

      // This decrements it, but does not discard it.
      heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, returnRef.ownership), returnRef)

      receiver(i, returnRef)
    });
  }
*/
// mig: fn execute_interface_function
pub fn execute_interface_function<'v, 'h, 's>(program_h: &ProgramH<'s, 'h>, stdin: &dyn Fn() -> String, stdout: &dyn Fn(String), heap: &mut HeapV<'v, 'h, 's>, undeviewed_arg_references: &[ReferenceV<'v, 'h, 's>], virtual_param_index: i32, interface_ref_h: InterfaceHT<'s, 'h>, index_in_edge: i32, function_type: PrototypeH<'s, 'h>) -> (FunctionH<'s, 'h>, (CallIdV<'v, 'h, 's>, INodeExecuteResultV<'v, 'h, 's>)) { panic!("Unimplemented: execute_interface_function"); }
/*
  private def executeInterfaceFunction(
      programH: ProgramH,
      stdin: () => String,
      stdout: String => Unit,
      heap: Heap,
      undeviewedArgReferences: Vector[ReferenceV],
      virtualParamIndex: Int,
      interfaceRefH: InterfaceHT,
      indexInEdge: Int,
      functionType: PrototypeH) = {

    val interfaceReference = undeviewedArgReferences(virtualParamIndex)

    // Vivem wants to know the type of an undead object so it can call a weak-self
    // method after it's been dropped. Midas can do this (it relies on it for resilient
    // mode) though some other platforms probably won't be able to.
    val edge =
      heap.dereference(interfaceReference, allowUndead = true) match {
        case StructInstanceV(structH, _) => structH.edges.find(_.interface == interfaceRefH).get
        case other => vwat(other.toString)
      }

    val ReferenceV(actualStruct, actualInterfaceKind, actualOwnership, actualLocation, allocNum) = interfaceReference
    vassert(actualInterfaceKind.hamut == interfaceRefH)
    val structReference = ReferenceV(actualStruct, actualStruct, actualOwnership, actualLocation, allocNum)

    val prototypeH = edge.structPrototypesByInterfaceMethod.values.toVector(indexInEdge)
    val functionH = programH.lookupFunction(prototypeH)

    val actualPrototype = functionH.prototype
    val expectedPrototype = functionType
    // We would compare functionH.type to functionType directly, but
    // functionH.type expects a struct and prototypeH expects an interface.

    // First, check that all the other params are correct.
    undeviewedArgReferences.zipWithIndex.zip(actualPrototype.params).zip(expectedPrototype.params).foreach({
      case (((argReference, index), actualFunctionParamType), expectedFunctionParamType) => {
        // Skip the interface line for now, we check it below
        if (index != virtualParamIndex) {
          heap.checkReference(actualFunctionParamType, argReference)
          heap.checkReference(expectedFunctionParamType, argReference)
          vassert(actualFunctionParamType == expectedFunctionParamType)
        }
      }
    })

    val deviewedArgReferences = undeviewedArgReferences.updated(virtualParamIndex, structReference)

    val maybeReturnReference =
      FunctionVivem.executeFunction(
        programH,
        stdin,
        stdout,
        heap,
        deviewedArgReferences.toVector,
        functionH)
    (functionH, maybeReturnReference)
  }
*/
// mig: fn discard
pub fn discard<'v, 'h, 's>(program_h: &'h ProgramH<'s, 'h>, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, scout_arena: &crate::scout_arena::ScoutArena<'s>, heap: &mut HeapV<'v, 'h, 's>, stdout: &'v dyn Fn(StrI<'s>), stdin: &'v dyn Fn() -> StrI<'s>, call_id: CallIdV<'v, 'h, 's>, expected_reference: CoordH<'s, 'h>, actual_reference: ReferenceV<'v, 'h, 's>) {
    heap.decrement_reference_ref_count(
        crate::testvm::values::IObjectReferrerV::RegisterToObjectReferrer(
            crate::testvm::values::RegisterToObjectReferrerV { call_id, ownership: actual_reference.ownership }
        ),
        actual_reference,
    );
    cleanup(program_h, interner, heap, stdout, stdin, call_id, expected_reference, actual_reference);
}
/*
  def discard(
    programH: ProgramH,
    heap: Heap,
    stdout: String => Unit,
    stdin: () => String,
    callId: CallId,
    expectedReference: CoordH[KindHT],
    actualReference: ReferenceV
  ): Unit = {
    heap.decrementReferenceRefCount(RegisterToObjectReferrer(callId, actualReference.ownership), actualReference)
    cleanup(programH, heap, stdout, stdin, callId, expectedReference, actualReference)
  }
*/
// mig: fn cleanup
pub fn cleanup<'v, 'h, 's>(program_h: &ProgramH<'s, 'h>, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, heap: &mut HeapV<'v, 'h, 's>, stdout: &dyn Fn(StrI<'s>), stdin: &dyn Fn() -> StrI<'s>, call_id: CallIdV<'v, 'h, 's>, expected_reference: CoordH<'s, 'h>, actual_reference: ReferenceV<'v, 'h, 's>) {
    if heap.get_total_ref_count(actual_reference) == 0 {
        match expected_reference.ownership {
            OwnershipH::OwnH => {}
            OwnershipH::WeakH => panic!("cleanup: Weak — pilot doesn't exercise"),
            OwnershipH::MutableBorrowH | OwnershipH::ImmutableBorrowH => {}
            OwnershipH::MutableShareH | OwnershipH::ImmutableShareH => {
                match expected_reference.kind {
                    KindHT::VoidHT(_) | KindHT::IntHT(_) | KindHT::BoolHT(_) | KindHT::StrHT(_) | KindHT::FloatHT(_) | KindHT::OpaqueHT(_) => {
                        heap.zero(actual_reference);
                        heap.deallocate_if_no_weak_refs(actual_reference);
                    }
                    KindHT::StructHT(sr) => {
                        let struct_def = program_h.lookup_struct(interner, sr);
                        let member_expected_types: Vec<CoordH<'s, 'h>> = struct_def.members.iter().map(|m| m.tyype).collect();
                        let member_refs = heap.destructure(actual_reference);
                        assert_eq!(member_expected_types.len(), member_refs.len());
                        for (member_ref, member_expected_type) in member_refs.iter().zip(member_expected_types.iter()) {
                            cleanup(program_h, interner, heap, stdout, stdin, call_id, *member_expected_type, *member_ref);
                        }
                    }
                    KindHT::InterfaceHT(_) => panic!("cleanup: InterfaceHT — pilot doesn't exercise"),
                    KindHT::RuntimeSizedArrayHT(_) => panic!("cleanup: RuntimeSizedArrayHT — pilot doesn't exercise"),
                    KindHT::StaticSizedArrayHT(ssa_ht) => {
                        let element_refs = heap.destructure_array(actual_reference);
                        let element_type = program_h.lookup_static_sized_array(ssa_ht).element_type;
                        for element_ref in element_refs.iter() {
                            cleanup(program_h, interner, heap, stdout, stdin, call_id, element_type, *element_ref);
                        }
                        heap.zero(actual_reference);
                        heap.deallocate_if_no_weak_refs(actual_reference);
                    }
                    KindHT::NeverHT(_) => panic!("cleanup: NeverHT — pilot doesn't exercise"),
                }
            }
        }
    }
}
/*
  def cleanup(
    programH: ProgramH,
    heap: Heap,
    stdout: String => Unit,
    stdin: () => String,
    callId: CallId,
    expectedReference: CoordH[KindHT],
    actualReference: ReferenceV
  ): Unit = {

    if (heap.getTotalRefCount(actualReference) == 0) {
      expectedReference.ownership match {
        case OwnH => // Do nothing, Vivem often discards owning things, if we're making a new owning reference to it.
        case WeakH => {
          heap.deallocateIfNoWeakRefs(actualReference)
        }
        case MutableBorrowH | ImmutableBorrowH => // Do nothing.
        case MutableShareH | ImmutableShareH => {
          expectedReference.kind match {
            case VoidHT() | IntHT(_) | BoolHT() | StrHT() | FloatHT() | OpaqueHT(_, _, _) => {
              heap.zero(actualReference)
              heap.deallocateIfNoWeakRefs(actualReference)
            }
            //            case ir @ InterfaceRefH(_) => {
            //              heap.vivemDout.println()
            //              heap.vivemDout.println("  " * callId.callDepth + "Making new stack frame (discard icall)")
            //              val prototypeH = programH.lookupPackage(expectedReference.kind.packageCoord).immDestructorsByKind(expectedReference.kind)
            //              val indexInEdge = programH.lookupInterface(ir).methods.indexWhere(_.prototypeH == prototypeH)
            //              vassert(indexInEdge >= 0)
            //              val (functionH, (calleeCallId, retuurn)) =
            //                executeInterfaceFunction(programH, stdin, stdout, heap, Vector(actualReference), 0, ir, indexInEdge, prototypeH)
            //              heap.vivemDout.print("  " * callId.callDepth + "Getting return reference")
            //              val returnRef = possessCalleeReturn(heap, callId, calleeCallId, retuurn)
            //              vassert(returnRef.actualKind.hamut == ProgramH.emptyTupleStructRef)
            //              discard(programH, heap, stdout, stdin, callId, prototypeH.returnType, returnRef)
            //            }
            case sr @ StructHT(_) => {
              val structDef = programH.lookupStruct(sr)
              val memberExpectedTypes = structDef.members.map(_.tyype)
              val memberRefs = heap.destructure(actualReference)
              vassert(memberExpectedTypes.size == memberRefs.size)
              memberRefs.zip(memberExpectedTypes).foreach({ case (memberRef, memberExpectedType) =>
                cleanup(programH, heap, stdout, stdin, callId, memberExpectedType, memberRef)
              })
            }
            case ir @ InterfaceHT(_) => {
              val actualConcreteType =
                actualReference.actualKind.hamut match { case sr @ StructHT(_) => sr case other => vwat(other) }
              val structDef = programH.lookupStruct(actualConcreteType)
              val memberExpectedTypes = structDef.members.map(_.tyype)
              val memberRefs = heap.destructure(actualReference)
              vassert(memberExpectedTypes.size == memberRefs.size)
              memberRefs.zip(memberExpectedTypes).foreach({ case (memberRef, memberExpectedType) =>
                cleanup(programH, heap, stdout, stdin, callId, memberExpectedType, memberRef)
              })
            }
            case rsaHT @ RuntimeSizedArrayHT(_) => {
              val elementRefs = heap.destructureArray(actualReference)
              val elementType = programH.lookupRuntimeSizedArray(rsaHT).elementType
              elementRefs.foreach(elementRef => {
                cleanup(programH, heap, stdout, stdin, callId, elementType, elementRef)
              })
              heap.zero(actualReference)
              heap.deallocateIfNoWeakRefs(actualReference)
            }
            case ssaHT @ StaticSizedArrayHT(_) => {
              val elementRefs = heap.destructureArray(actualReference)
              val elementType = programH.lookupStaticSizedArray(ssaHT).elementType
              elementRefs.foreach(elementRef => {
                cleanup(programH, heap, stdout, stdin, callId, elementType, elementRef)
              })
              heap.zero(actualReference)
              heap.deallocateIfNoWeakRefs(actualReference)
            }
//            case ... => {
//              heap.vivemDout.println()
//              heap.vivemDout.println("  " * callId.callDepth + "Making new stack frame (discard call)")
//              val prototypeH = vimpl()
//              //                vassertOne(
//              //                  programH.packages.packageCoordToContents.values
//              //                    .flatMap(_.immDestructorsByKind.get(expectedReference.kind)))
//              val functionH = programH.lookupFunction(prototypeH)
//              val (calleeCallId, retuurn) =
//                FunctionVivem.executeFunction(
//                  programH, stdin, stdout, heap, Vector(actualReference), functionH)
//              heap.vivemDout.print("  " * callId.callDepth + "Getting return reference")
//              val returnRef = possessCalleeReturn(heap, callId, calleeCallId, retuurn)
//              vassert(returnRef.actualKind.hamut == VoidH())
//              cleanup(programH, heap, stdout, stdin, callId, vimpl(), returnRef)
//            }
          }
        }
      }
    }
  }
}

*/
