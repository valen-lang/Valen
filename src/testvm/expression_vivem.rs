use std::cell::Cell;
use crate::utils::fx::HashMap;
use std::io::Write;
use std::marker::PhantomData;
use crate::interner::StrI;
use crate::instantiating::ast::types::{KindIT, InterfaceIT, BorrowRefIT};
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::ast::ast::PrototypeI;
use crate::instantiating::ast::expressions::ExpressionIE;
use crate::instantiating::instantiating_interner::InstantiatingInterner;
use crate::testvm::values::OwnershipV;
use crate::testvm::values::outer_ownership;
use crate::testvm::values::strip_outer_references;
use crate::testvm::values::{
    AllocationIdV, AllocationV, CallIdV, ExpressionIdV, IObjectReferrerV,
    KindV, PrimitiveKindV, ReferenceV, RegisterV, VariableAddressV, VariableV,
};
use crate::testvm::heap::HeapV;
use crate::scout_arena::ScoutArena;
use crate::testvm::function_vivem::execute_function;
use crate::testvm::function_vivem::get_extern_function;
use crate::testvm::heap::AdapterForExternsV;
use crate::testvm::heap::get_var_address;
use crate::testvm::values::BoolV;
use crate::testvm::values::ElementAddressV;
use crate::testvm::values::FloatV;
use crate::testvm::values::IntV;
use crate::testvm::values::MemberAddressV;
use crate::testvm::values::RRKindV;
use crate::testvm::values::RegisterToObjectReferrerV;
use crate::testvm::values::StrV;
use crate::testvm::vivem::VmRuntimeErrorV;
use std::mem::discriminant;



/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum INodeExecuteResultV<'v, 'h, 's> {
  Continue(NodeContinueV<'v, 'h, 's>),
  Return(NodeReturnV<'v, 'h, 's>),
  Break(NodeBreakV<'v, 'h, 's>),
  Error(VmRuntimeErrorV<'s>),
}


/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct NodeContinueV<'v, 'h, 's> {
  pub result_ref: ReferenceV<'v, 'h, 's>,
}


/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct NodeReturnV<'v, 'h, 's> {
  pub return_ref: ReferenceV<'v, 'h, 's>,
}


/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct NodeBreakV<'v, 'h, 's>
where 's: 'h, 'h: 'v,
{
  pub _phantom: PhantomData<(&'v (), &'h (), &'s ())>,
}

pub fn make_primitive<'v, 'i, 's>(heap: &mut HeapV<'v, 'i, 's>, interner: &InstantiatingInterner<'s, 'i>, call_id: CallIdV<'v, 'i, 's>, ownership: OwnershipV, kind: KindV<'v, 'i, 's>) -> ReferenceV<'v, 'i, 's> {
    assert!(!matches!(kind, KindV::Void(_)));
    let r#ref = heap.allocate_transient(interner, ownership, kind);
    heap.increment_reference_ref_count(
        IObjectReferrerV::RegisterToObjectReferrer(
            RegisterToObjectReferrerV { call_id }
        ),
        r#ref,
    );
    r#ref
}


pub fn take_argument<'v, 'i, 's>(heap: &mut HeapV<'v, 'i, 's>, interner: &InstantiatingInterner<'s, 'i>, call_id: CallIdV<'v, 'i, 's>, argument_index: i32, result_type: KindIT<'s, 'i>) -> ReferenceV<'v, 'i, 's> {
    let r#ref = heap.take_argument(interner, call_id, argument_index, result_type);
    heap.increment_reference_ref_count(
        IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }),
        r#ref);
    r#ref
}


pub fn possess_callee_return<'v, 'h, 's>(heap: &mut HeapV<'v, 'h, 's>, call_id: CallIdV<'v, 'h, 's>, callee_call_id: CallIdV<'v, 'h, 's>, result: &NodeReturnV<'v, 'h, 's>) -> ReferenceV<'v, 'h, 's> {
    heap.decrement_reference_ref_count(
        IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id: callee_call_id }),
        result.return_ref,
    );
    heap.increment_reference_ref_count(
        IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }),
        result.return_ref,
    );
    result.return_ref
}


pub fn upcast<'v, 'i, 's>(source_reference: ReferenceV<'v, 'i, 's>, target_interface_ref: &'i InterfaceIT<'s, 'i>) -> ReferenceV<'v, 'i, 's> {
    // An interface view is a bare kind (`KindIT::InterfaceIT`), so `seen_as_kind` stays wrap-free;
    // ownership carries over unchanged, and identity stays the concrete `actual_kind`.
    ReferenceV::new(
        source_reference.actual_kind,
        RRKindV { hamut: KindIT::InterfaceIT(target_interface_ref), _phantom: PhantomData },
        source_reference.ownership,
        source_reference.num,
    )
}


pub fn execute_node<'v, 'i, 's>(program_h: &'i HinputsI<'s, 'i>, interner: &InstantiatingInterner<'s, 'i>, scout_arena: &ScoutArena<'s>, stdin: &'v dyn Fn() -> StrI<'s>, stdout: &'v dyn Fn(StrI<'s>), heap: &mut HeapV<'v, 'i, 's>, expression_id: ExpressionIdV<'v, 'i, 's>, node: &ExpressionIE<'s, 'i>) -> INodeExecuteResultV<'v, 'i, 's> {
    let node_name = match node {
        ExpressionIE::LetAndLend(_) => "LetAndLend",
        ExpressionIE::LockWeak(_) => "LockWeak",
        ExpressionIE::BorrowToWeak(_) => "BorrowToWeak",
        ExpressionIE::LetNormal(_) => "LetNormal",
        ExpressionIE::Restackify(_) => "Restackify",
        ExpressionIE::Unlet(_) => "Unlet",
        ExpressionIE::Discard(_) => "Discard",
        ExpressionIE::If(_) => "If",
        ExpressionIE::While(_) => "While",
        ExpressionIE::Mutate(_) => "Mutate",
        ExpressionIE::Return(_) => "Return",
        ExpressionIE::Break(_) => "Break",
        ExpressionIE::Block(_) => "Block",
        ExpressionIE::Consecutor(_) => "Consecutor",
        ExpressionIE::StaticArrayFromValues(_) => "StaticArrayFromValues",
        ExpressionIE::ArraySize(_) => "ArraySize",
        ExpressionIE::IsSameInstance(_) => "IsSameInstance",
        ExpressionIE::AsSubtype(_) => "AsSubtype",
        ExpressionIE::VoidLiteral(_) => "VoidLiteral",
        ExpressionIE::ConstantInt(_) => "ConstantInt",
        ExpressionIE::ConstantBool(_) => "ConstantBool",
        ExpressionIE::ConstantStr(_) => "ConstantStr",
        ExpressionIE::ConstantFloat(_) => "ConstantFloat",
        ExpressionIE::ArgLookup(_) => "ArgLookup",
        ExpressionIE::ArrayLength(_) => "ArrayLength",
        ExpressionIE::InterfaceFunctionCall(_) => "InterfaceFunctionCall",
        ExpressionIE::ExternFunctionCall(_) => "ExternFunctionCall",
        ExpressionIE::FunctionCall(_) => "FunctionCall",
        ExpressionIE::Construct(_) => "Construct",
        ExpressionIE::NewRuntimeSizedArray(_) => "NewRuntimeSizedArray",
        ExpressionIE::StaticArrayFromCallable(_) => "StaticArrayFromCallable",
        ExpressionIE::DestroyStaticSizedArrayIntoFunction(_) => "DestroyStaticSizedArrayIntoFunction",
        ExpressionIE::DestroyStaticSizedArrayIntoLocals(_) => "DestroyStaticSizedArrayIntoLocals",
        ExpressionIE::DestroyRuntimeSizedArray(_) => "DestroyRuntimeSizedArray",
        ExpressionIE::RuntimeSizedArrayCapacity(_) => "RuntimeSizedArrayCapacity",
        ExpressionIE::PushRuntimeSizedArray(_) => "PushRuntimeSizedArray",
        ExpressionIE::PopRuntimeSizedArray(_) => "PopRuntimeSizedArray",
        ExpressionIE::InterfaceToInterfaceUpcast(_) => "InterfaceToInterfaceUpcast",
        ExpressionIE::Upcast(_) => "Upcast",
        ExpressionIE::Destroy(_) => "Destroy",
        ExpressionIE::CopyPrim(_) => "CopyPrim",
        ExpressionIE::LocalLookup(_) => "LocalLookup",
        ExpressionIE::StaticSizedArrayLookup(_) => "StaticSizedArrayLookup",
        ExpressionIE::RuntimeSizedArrayLookup(_) => "RuntimeSizedArrayLookup",
        ExpressionIE::MemberLookup(_) => "MemberLookup",
        ExpressionIE::Deref(_) => "Deref",
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


pub fn execute_node_inner<'v, 'i, 's>(program_h: &'i HinputsI<'s, 'i>, interner: &InstantiatingInterner<'s, 'i>, scout_arena: &ScoutArena<'s>, stdin: &'v dyn Fn() -> StrI<'s>, stdout: &'v dyn Fn(StrI<'s>), heap: &mut HeapV<'v, 'i, 's>, expression_id: ExpressionIdV<'v, 'i, 's>, node: &ExpressionIE<'s, 'i>) -> INodeExecuteResultV<'v, 'i, 's> {
    let call_id = expression_id.call_id;
    match node {
        ExpressionIE::Construct(cons) => {
            let struct_def = program_h.lookup_struct(&cons.struct_tt.id);
            let member_references_vec: Vec<ReferenceV<'v, 'i, 's>> = cons.args.iter().enumerate().map(|(i, arg_expr)| {
                match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, i as i32), arg_expr) {
                    INodeExecuteResultV::Return(_) => {
                        // do we have to, like, discard the previously made arguments?
                        // what happens with those?
                        panic!("Construct arg produced Return — vimpl; return r");
                    }
                    INodeExecuteResultV::Break(_) => panic!("Construct arg produced Break — vwat"),
                    INodeExecuteResultV::Continue(c) => c.result_ref,
                    INodeExecuteResultV::Error(_) => panic!("Construct arg produced Error — vimpl (closure can't propagate)"),
                }
            }).collect();
            for r in &member_references_vec {
                heap.decrement_reference_ref_count(
                    IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }),
                    *r,
                );
            }
            assert_eq!(member_references_vec.len(), struct_def.members.len());
            let member_references: &'v [ReferenceV<'v, 'i, 's>] = heap.vivem_bump.alloc_slice_copy(&member_references_vec);
            let reference = heap.new_struct(interner, struct_def, cons.result, member_references);
            heap.increment_reference_ref_count(
                IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }),
                reference,
            );
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: reference })
        }
        ExpressionIE::ConstantInt(c) => {
            let r#ref = make_primitive(heap, interner, call_id, OwnershipV::Own, KindV::Int(IntV { value: c.value, bits: c.bits, _phantom: PhantomData }));
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: r#ref })
        }
        ExpressionIE::Return(r) => {
            let source_ref = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &r.source_expr) {
                ret @ INodeExecuteResultV::Return(_) => return ret,
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Break(_) => panic!("execute_node_inner: Return source produced Break — vwat"),
                ret @ INodeExecuteResultV::Error(_) => return ret,
            };
            INodeExecuteResultV::Return(NodeReturnV { return_ref: source_ref })
        }
        ExpressionIE::Unlet(u) => {
            let var_address = get_var_address(expression_id.call_id, u.variable);
            // expected == target, so `transmute` returns the stored reference as-is (moved out owned).
            let reference = heap.get_reference_from_local(interner, var_address, u.variable.tyype, u.variable.tyype);
            heap.increment_reference_ref_count(
                IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }),
                reference,
            );
            {
                let handle = &mut *heap.vivem_dout;
                write!(handle, " ^{}", var_address).unwrap();
            }
            heap.remove_local(interner, var_address, u.variable.tyype);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: reference })
        }
        ExpressionIE::LetNormal(l) => {
            let reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &l.expr) {
                ret @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return ret,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            let var_addr = get_var_address(expression_id.call_id, l.variable);
            heap.add_local(interner, var_addr, reference, l.variable.tyype);
            {
                let handle = &mut *heap.vivem_dout;
                write!(handle, " v{}/{:?}<-o{}", var_addr.call_id.call_depth, var_addr.name, reference.num).unwrap();
            }
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, l.expr.result(), reference) { return INodeExecuteResultV::Error(e); }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: heap.void() })
        }
        ExpressionIE::LetAndLend(l) => {
            let reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &l.expr) {
                ret @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return ret,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            let var_addr = get_var_address(expression_id.call_id, l.variable);
            heap.add_local(interner, var_addr, reference, l.variable.tyype);
            {
                let handle = &mut *heap.vivem_dout;
                write!(handle, " v{}/{:?}<-o{}", var_addr.call_id.call_depth, var_addr.name, reference.num).unwrap();
            }
            // The value is now owned by the local; drop the evaluation register-referrer, then lend a
            // borrow of the freshly-created local as the result (@Double-References).
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, l.expr.result(), reference) { return INodeExecuteResultV::Error(e); }
            let borrow = heap.get_reference_from_local(interner, var_addr, l.variable.tyype, l.result);
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), borrow);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: borrow })
        }
        ExpressionIE::Block(b) => {
            execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &b.inner)
        }
        ExpressionIE::Consecutor(c) => {
            let mut last_inner_expr_result_ref: Option<ReferenceV<'v, 'i, 's>> = None;
            for (i, inner_expr) in c.exprs.iter().enumerate() {
                match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, i as i32), inner_expr) {
                    ret @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return ret,
                    INodeExecuteResultV::Continue(cc) => {
                        if i == c.exprs.len() - 1 {
                            last_inner_expr_result_ref = Some(cc.result_ref);
                        }
                    }
                }
                {
                    let handle = &mut *heap.vivem_dout;
                    writeln!(handle).unwrap();
                }
            }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: last_inner_expr_result_ref.expect("Consecutor: empty exprs") })
        }
        ExpressionIE::FunctionCall(fc) => {
            let mut arg_refs: Vec<ReferenceV<'v, 'i, 's>> = Vec::new();
            for (i, arg_expr) in fc.args.iter().enumerate() {
                let r = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, i as i32), arg_expr) {
                    ret @ (INodeExecuteResultV::Break(_) | INodeExecuteResultV::Return(_) | INodeExecuteResultV::Error(_)) => return ret,
                    INodeExecuteResultV::Continue(c) => c.result_ref,
                };
                arg_refs.push(r);
            }
            // `lookup_function` (by prototype) is a stub; resolve the callee by matching its id.
            let function_h = *program_h.functions.iter().find(|f| f.header.id == fc.callable.id).expect("FunctionCall: callee not found");
            {
                let handle = &mut *heap.vivem_dout;
                writeln!(handle).unwrap();
                writeln!(handle, "{}Making new stack frame (call)", "  ".repeat(expression_id.call_id.call_depth as usize)).unwrap();
            }
            for r in arg_refs.iter() {
                heap.decrement_reference_ref_count(
                    IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }),
                    *r);
            }
            let arg_refs_slice: &'v [ReferenceV<'v, 'i, 's>] = heap.vivem_bump.alloc_slice_copy(&arg_refs);
            let (callee_call_id, retuurn) = match execute_function(program_h, interner, scout_arena, stdin, stdout, heap, arg_refs_slice, function_h) {
                Ok(t) => t,
                Err(e) => return INodeExecuteResultV::Error(e),
            };
            {
                let handle = &mut *heap.vivem_dout;
                write!(handle, "{}Getting return reference", "  ".repeat(expression_id.call_id.call_depth as usize)).unwrap();
            }
            let return_ref = possess_callee_return(heap, call_id, callee_call_id, &retuurn);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: return_ref })
        }
        ExpressionIE::ArgLookup(a) => {
            let r#ref = take_argument(heap, interner, call_id, a.param_index, a.tyype);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: r#ref })
        }
        ExpressionIE::VoidLiteral(_) => {
            let r#ref = heap.void();
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: r#ref })
        }
        ExpressionIE::ConstantBool(c) => {
            let r#ref = make_primitive(heap, interner, call_id, OwnershipV::Own, KindV::Bool(BoolV { value: c.value, _phantom: PhantomData }));
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: r#ref })
        }
        ExpressionIE::ConstantFloat(c) => {
            let r#ref = make_primitive(heap, interner, call_id, OwnershipV::Own, KindV::Float(FloatV { value: c.value, _phantom: PhantomData }));
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: r#ref })
        }
        ExpressionIE::ConstantStr(c) => {
            let interned = scout_arena.intern_str(c.value);
            // Str stays Share — RC'd/shared.
            let r#ref = make_primitive(heap, interner, call_id, OwnershipV::Share, KindV::Str(StrV { value: interned, _phantom: PhantomData }));
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: r#ref })
        }
        ExpressionIE::Discard(d) => {
            let source_ref = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &d.expr) {
                r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            // Lots of instructions do this, not just Discard, see DINSIE.
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, d.expr.result(), source_ref) { return INodeExecuteResultV::Error(e); }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: heap.void() })
        }
        ExpressionIE::LocalLookup(ll) => {
            // VCOORD: simplify this, LocalLoad should only return references
            // Onion LocalLookup always yields a `BorrowRef` of the local's storage, so the old
            // "primitives-only-as-Own" carve-out is gone — the result is always a reference.
            // /VCOORD
            let var_address = get_var_address(expression_id.call_id, ll.local_variable);
            // Look up the local's stored reference and re-view it as this lookup's `&(local kind)`
            // result (points at the same object; @Double-References).
            let target_type = KindIT::BorrowRefIT(ll.result);
            let reference = heap.get_reference_from_local(interner, var_address, ll.local_variable.tyype, target_type);
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), reference);
            {
                let handle = &mut *heap.vivem_dout;
                write!(handle, " *{}", var_address).unwrap();
            }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: reference })
        }
        ExpressionIE::Deref(d) => {
            let inner_ref = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &d.inner) {
                r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            // Peel one outer wrap (@Double-References / S8): keep the same pointee, re-derive the
            // ownership tag + seen_as from the peeled type (so `&weak Ship` → `weak Ship`). Never a
            // copy (that is CopyPrim), allocates nothing.
            let peeled = RRKindV { hamut: d.result, _phantom: PhantomData };
            let result_ref = ReferenceV::new(inner_ref.actual_kind, peeled.strip_outer_references(), peeled.outer_ownership(), inner_ref.num);
            // Re-key the register-referrer from the inner flavor to the peeled flavor (strong↔weak can flip).
            heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), inner_ref);
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), result_ref);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref })
        }
        ExpressionIE::Mutate(m) => {
            // Onion unifies local/member/element stores into one Mutate whose `destination_expr` is a
            // lookup. Dispatch on it to find the storage, mutate, and return the displaced old value.
            let old_reference = match m.destination_expr {
                ExpressionIE::LocalLookup(ll) => {
                    let source_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &m.source_expr) {
                        r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                        INodeExecuteResultV::Continue(c) => c.result_ref,
                    };
                    let var_address = get_var_address(expression_id.call_id, ll.local_variable);
                    let old_ref = heap.mutate_variable(interner, var_address, source_reference, m.source_expr.result());
                    heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), old_ref);
                    if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, m.source_expr.result(), source_reference) { return INodeExecuteResultV::Error(e); }
                    old_ref
                }
                ExpressionIE::MemberLookup(ml) => {
                    let struct_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &ml.struct_expr) {
                        r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                        INodeExecuteResultV::Continue(c) => c.result_ref,
                    };
                    let source_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 1), &m.source_expr) {
                        r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                        INodeExecuteResultV::Continue(c) => c.result_ref,
                    };
                    let field_index = match heap.dereference(struct_reference, false) {
                        KindV::StructInstance(si) => si.struct_h.members.iter().position(|mem| mem.name == ml.member_name).expect("Mutate: member not found") as i32,
                        _ => panic!("Mutate: MemberLookup destination not a StructInstance"),
                    };
                    let address = MemberAddressV { struct_id: struct_reference.alloc_id(), field_index };
                    let old_member = heap.mutate_struct(address, source_reference, m.source_expr.result());
                    heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), old_member);
                    if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, ml.struct_expr.result(), struct_reference) { return INodeExecuteResultV::Error(e); }
                    if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, m.source_expr.result(), source_reference) { return INodeExecuteResultV::Error(e); }
                    old_member
                }
                ExpressionIE::StaticSizedArrayLookup(sal) => {
                    let array_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &sal.array_expr) {
                        r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                        INodeExecuteResultV::Continue(c) => c.result_ref,
                    };
                    let index_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 1), &sal.index_expr) {
                        r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                        INodeExecuteResultV::Continue(c) => c.result_ref,
                    };
                    let source_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 2), &m.source_expr) {
                        r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                        INodeExecuteResultV::Continue(c) => c.result_ref,
                    };
                    let element_index = match heap.dereference(index_reference, false) {
                        KindV::Int(int_v) if int_v.bits == 32 => int_v.value,
                        _ => panic!("Mutate: StaticSizedArrayLookup index not IntV(_, 32)"),
                    };
                    let address = ElementAddressV { array_id: array_reference.alloc_id(), element_index };
                    let old = heap.mutate_array(address, source_reference, m.source_expr.result());
                    heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), old);
                    if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, m.source_expr.result(), source_reference) { return INodeExecuteResultV::Error(e); }
                    if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, sal.index_expr.result(), index_reference) { return INodeExecuteResultV::Error(e); }
                    if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, sal.array_expr.result(), array_reference) { return INodeExecuteResultV::Error(e); }
                    old
                }
                ExpressionIE::RuntimeSizedArrayLookup(ral) => {
                    let array_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &ral.array_expr) {
                        r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                        INodeExecuteResultV::Continue(c) => c.result_ref,
                    };
                    let index_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 1), &ral.index_expr) {
                        r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                        INodeExecuteResultV::Continue(c) => c.result_ref,
                    };
                    let source_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 2), &m.source_expr) {
                        r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                        INodeExecuteResultV::Continue(c) => c.result_ref,
                    };
                    let element_index = match heap.dereference(index_reference, false) {
                        KindV::Int(int_v) if int_v.bits == 32 => int_v.value,
                        _ => panic!("Mutate: RuntimeSizedArrayLookup index not IntV(_, 32)"),
                    };
                    let address = ElementAddressV { array_id: array_reference.alloc_id(), element_index };
                    let old = heap.mutate_array(address, source_reference, m.source_expr.result());
                    heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), old);
                    if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, m.source_expr.result(), source_reference) { return INodeExecuteResultV::Error(e); }
                    if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, ral.index_expr.result(), index_reference) { return INodeExecuteResultV::Error(e); }
                    if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, ral.array_expr.result(), array_reference) { return INodeExecuteResultV::Error(e); }
                    old
                }
                _ => panic!("Mutate: unexpected destination_expr {:?}", m.destination_expr),
            };
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: old_reference })
        }
        ExpressionIE::Destroy(d) => {
            let struct_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &d.expr) {
                r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            heap.decrement_reference_ref_count(
                IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }),
                struct_reference);
            // DDSOT
            if let Err(e) = heap.ensure_ref_count(interner, scout_arena, struct_reference, Some(false), 0) { return INodeExecuteResultV::Error(e); }
            let old_member_references = match heap.destructure(struct_reference) {
                Ok(r) => r,
                Err(e) => return INodeExecuteResultV::Error(e),
            };
            assert!(old_member_references.len() == d.destination_reference_variables.len());
            // Destructured members bind to the destination locals; each local's declared type is its tyype.
            for (member_ref, local_var) in old_member_references.iter().zip(d.destination_reference_variables.iter()) {
                let var_addr = get_var_address(expression_id.call_id, *local_var);
                heap.add_local(interner, var_addr, *member_ref, local_var.tyype);
                {
                    let handle = &mut *heap.vivem_dout;
                    write!(handle, " v{}<-o{}", var_addr, member_ref.num).unwrap();
                }
            }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: heap.void() })
        }
        ExpressionIE::ExternFunctionCall(e) => {
            // `prototype2` is an inline `Copy` value-type; borrow it straight from the `&'i` node to
            // get the `&'i PrototypeI` the stack-frame call-id needs (no copy).
            let prototype: &'i PrototypeI<'s, 'i> = &e.prototype2;
            let extern_function = get_extern_function(program_h, prototype);
            let arg_refs: Vec<ReferenceV<'v, 'i, 's>> =
                e.args.iter().enumerate().map(|(i, arg_expr)| {
                    match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, i as i32), arg_expr) {
                        INodeExecuteResultV::Break(_) | INodeExecuteResultV::Return(_) => panic!("execute_node_inner: ExternFunctionCall arg produced Break/Return — vwat (BRCOBS)"),
                        INodeExecuteResultV::Error(_) => panic!("execute_node_inner: ExternFunctionCall arg produced Error — vimpl (closure can't propagate)"),
                        INodeExecuteResultV::Continue(c) => c.result_ref,
                    }
                }).collect();
            let arg_refs_slice: &'v [ReferenceV<'v, 'i, 's>] = heap.vivem_bump.alloc_slice_copy(&arg_refs);
            let result_ref = {
                let mut adapter = AdapterForExternsV {
                    program_h,
                    interner,
                    scout_arena,
                    heap: &mut *heap,
                    call_id: CallIdV { call_depth: expression_id.call_id.call_depth + 1, function: prototype, _phantom: PhantomData },
                    stdin,
                    stdout,
                };
                match extern_function(&mut adapter, arg_refs_slice) {
                    Ok(r) => r,
                    Err(e) => return INodeExecuteResultV::Error(e),
                }
            };
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), result_ref);
            // Special case for externs; externs arent allowed to change ref counts at all.
            // So, we just drop these normally.
            for (r, arg_expr) in arg_refs.iter().zip(e.args.iter()) {
                if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, arg_expr.result(), *r) { return INodeExecuteResultV::Error(e); }
            }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref })
        }
        ExpressionIE::If(i) => {
            let condition_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &i.condition) {
                ret @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return ret,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            let condition_value = match heap.dereference(condition_reference, false) {
                KindV::Bool(BoolV { value, .. }) => value,
                _ => panic!("execute_node_inner: If condition not BoolV"),
            };
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, i.condition.result(), condition_reference) { return INodeExecuteResultV::Error(e); }
            let block_result = if condition_value {
                match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 1), &i.then_call) {
                    ret @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return ret,
                    INodeExecuteResultV::Continue(c) => c.result_ref,
                }
            } else {
                match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 2), &i.else_call) {
                    ret @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return ret,
                    INodeExecuteResultV::Continue(c) => c.result_ref,
                }
            };
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: block_result })
        }
        ExpressionIE::MemberLookup(ml) => {
            let struct_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &ml.struct_expr) {
                r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            // Members are name-keyed onion-side; resolve the index (and declared type) off the instance.
            let (field_index, expected_member_type) = match heap.dereference(struct_reference, false) {
                KindV::StructInstance(si) => {
                    let idx = si.struct_h.members.iter().position(|mem| mem.name == ml.member_name).expect("MemberLookup: member not found");
                    (idx as i32, si.struct_h.members[idx].tyype)
                }
                _ => panic!("MemberLookup: struct not a StructInstance"),
            };
            let address = MemberAddressV { struct_id: struct_reference.alloc_id(), field_index };
            {
                let handle = &mut *heap.vivem_dout;
                write!(handle, " *{}", address.to_string()).unwrap();
            }
            // VCOORD: revisit
            // Onion MemberLookup always yields a BorrowRef of the member's storage — no OwnH carve-out.
            // /VCOORD
            let member_reference = heap.get_reference_from_struct(interner, address, expected_member_type, KindIT::BorrowRefIT(ml.result));
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), member_reference);
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, ml.struct_expr.result(), struct_reference) { return INodeExecuteResultV::Error(e); }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: member_reference })
        }
        ExpressionIE::StaticArrayFromValues(saf) => {
            let element_refs: Vec<ReferenceV<'v, 'i, 's>> =
                saf.elements.iter().enumerate().map(|(i, arg_expr)| {
                    match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, i as i32), arg_expr) {
                        INodeExecuteResultV::Return(_) => panic!("execute_node_inner: StaticArrayFromValues element produced Return — vimpl"),
                        INodeExecuteResultV::Break(_) => panic!("execute_node_inner: StaticArrayFromValues element produced Break — vimpl"),
                        INodeExecuteResultV::Continue(c) => c.result_ref,
                        INodeExecuteResultV::Error(_) => panic!("execute_node_inner: StaticArrayFromValues element produced Error — vimpl (closure can't propagate)"),
                    }
                }).collect();
            for r in element_refs.iter() {
                heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), *r);
            }
            let element_refs_slice: &'v [ReferenceV<'v, 'i, 's>] = heap.vivem_bump.alloc_slice_copy(&element_refs);
            let (array_reference, array_instance) = heap.add_array(interner, saf.result, element_refs_slice);
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), array_reference);
            write!(heap.vivem_dout, " o{}=", array_reference.num).unwrap();
            heap.print_kind(KindV::ArrayInstance(array_instance));
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: array_reference })
        }
        ExpressionIE::StaticSizedArrayLookup(ssal) => {
            let array_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &ssal.array_expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Error(e) => return INodeExecuteResultV::Error(e),
            };
            let index_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 1), &ssal.index_expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Error(e) => return INodeExecuteResultV::Error(e),
            };
            let index = match heap.dereference(index_reference, false) {
                KindV::Int(int_v) if int_v.bits == 32 => int_v.value as i32,
                _ => panic!("execute_node_inner: StaticSizedArrayLookup index not IntV(_, 32)"),
            };
            let address = ElementAddressV { array_id: array_reference.alloc_id(), element_index: index as i64 };
            write!(heap.vivem_dout, " **o:{}.{}", address.array_id.num, address.element_index).unwrap();
            // VCOORD: get rid of this, SSALoadH should only return a reference
            // Onion lookup always yields a BorrowRef of the element's storage — no OwnH carve-out.
            // /VCOORD
            let source = heap.get_reference_from_array(interner, address, ssal.array_type.element_type(), KindIT::BorrowRefIT(ssal.result));
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), source);
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, ssal.index_expr.result(), index_reference) { return INodeExecuteResultV::Error(e); }
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, ssal.array_expr.result(), array_reference) { return INodeExecuteResultV::Error(e); }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: source })
        }
        ExpressionIE::DestroyStaticSizedArrayIntoLocals(d) => {
            let arr_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &d.expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Error(e) => return INodeExecuteResultV::Error(e),
            };
            heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), arr_reference);
            if outer_ownership(d.expr.result()) == OwnershipV::Own {
                if let Err(e) = heap.ensure_ref_count(interner, scout_arena, arr_reference, None, 0) { return INodeExecuteResultV::Error(e); }
            } else {
                // Not doing
                //   heap.ensureTotalRefCount(arrReference, 0)
                // for share because we might be taking in a shared reference and not be destroying it.
            }
            let old_member_references = heap.destructure_array(arr_reference);
            if arr_reference.ownership == OwnershipV::Own {
                heap.zero(arr_reference);
                if let Err(e) = heap.deallocate_if_no_weak_refs(arr_reference) {
                    return INodeExecuteResultV::Error(e);
                }
            }
            assert!(old_member_references.len() == d.destination_reference_variables.len());
            for (member_ref, local_var) in old_member_references.iter().zip(d.destination_reference_variables.iter()) {
                let var_addr = get_var_address(expression_id.call_id, *local_var);
                heap.add_local(interner, var_addr, *member_ref, local_var.tyype);
                write!(heap.vivem_dout, " v{}<-o{}", var_addr, member_ref.num).unwrap();
            }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: heap.void() })
        }
        ExpressionIE::DestroyStaticSizedArrayIntoFunction(d) => {
            let array_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &d.array_expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Error(e) => return INodeExecuteResultV::Error(e),
            };
            let consumer_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 1), &d.consumer) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Error(e) => return INodeExecuteResultV::Error(e),
            };
            heap.check_reference(interner, d.consumer.result(), consumer_reference);
            heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), array_reference);
            if let Err(e) = heap.ensure_ref_count(interner, scout_arena, array_reference, None, 0) { return INodeExecuteResultV::Error(e); }
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), array_reference);
            if let Err(e) = consume_elements(program_h, interner, scout_arena, stdin, stdout, heap, expression_id, call_id, array_reference, consumer_reference, d.consumer_method, d.array_type.size(), &mut |_, _| {}) {
                return INodeExecuteResultV::Error(e);
            }
            heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), array_reference);
            heap.zero(array_reference);
            if let Err(e) = heap.deallocate_if_no_weak_refs(array_reference) {
                return INodeExecuteResultV::Error(e);
            }
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, d.consumer.result(), consumer_reference) { return INodeExecuteResultV::Error(e); }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: heap.void() })
        }
        ExpressionIE::Break(_) => {
            return INodeExecuteResultV::Break(NodeBreakV { _phantom: PhantomData });
        }
        ExpressionIE::DestroyRuntimeSizedArray(d) => {
            let array_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &d.array_expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Error(e) => return INodeExecuteResultV::Error(e),
            };
            heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), array_reference);
            if let Err(e) = heap.ensure_ref_count(interner, scout_arena, array_reference, None, 0) { return INodeExecuteResultV::Error(e); }
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), array_reference);
            let elements = match heap.dereference(array_reference, false) {
                KindV::ArrayInstance(a) => a.elements.get(),
                _ => panic!("execute_node_inner: DestroyRuntimeSizedArray array deref not ArrayInstance"),
            };
            assert!(elements.is_empty());
            heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), array_reference);
            heap.zero(array_reference);
            if let Err(e) = heap.deallocate_if_no_weak_refs(array_reference) {
                return INodeExecuteResultV::Error(e);
            }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: heap.void() })
        }
        ExpressionIE::ArrayLength(al) => {
            let array_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &al.array_expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Error(e) => return INodeExecuteResultV::Error(e),
            };
            let arr = match heap.dereference(array_reference, false) {
                KindV::ArrayInstance(a) => a,
                _ => panic!("execute_node_inner: ArrayLength array deref not ArrayInstance"),
            };
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, al.array_expr.result(), array_reference) { return INodeExecuteResultV::Error(e); }
            let len_ref = make_primitive(heap, interner, call_id, OwnershipV::Own, KindV::Int(IntV { value: arr.get_size(), bits: 32, _phantom: PhantomData }));
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: len_ref })
        }
        ExpressionIE::RuntimeSizedArrayCapacity(rsc) => {
            let array_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &rsc.array_expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Error(e) => return INodeExecuteResultV::Error(e),
            };
            let arr = match heap.dereference(array_reference, false) {
                KindV::ArrayInstance(a) => a,
                _ => panic!("execute_node_inner: RuntimeSizedArrayCapacity array deref not ArrayInstance"),
            };
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, rsc.array_expr.result(), array_reference) { return INodeExecuteResultV::Error(e); }
            let cap_ref = make_primitive(heap, interner, call_id, OwnershipV::Own, KindV::Int(IntV { value: arr.capacity as i64, bits: 32, _phantom: PhantomData }));
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: cap_ref })
        }
        ExpressionIE::While(w) => {
            let mut r#continue = true;
            while r#continue {
                match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &w.block.inner) {
                    INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                    INodeExecuteResultV::Break(_) => r#continue = false,
                    INodeExecuteResultV::Continue(c) => {
                        if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, w.block.inner.result(), c.result_ref) { return INodeExecuteResultV::Error(e); }
                    }
                    INodeExecuteResultV::Error(e) => return INodeExecuteResultV::Error(e),
                }
            }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: heap.void() })
        }
        ExpressionIE::InterfaceFunctionCall(ifc) => {
            let undeviewed_arg_references: Vec<ReferenceV<'v, 'i, 's>> = ifc.args.iter().enumerate().map(|(i, arg_expr)| {
                match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, i as i32), arg_expr) {
                    INodeExecuteResultV::Return(_) => panic!("InterfaceFunctionCall arg produced Return — vimpl"),
                    INodeExecuteResultV::Break(_) => panic!("InterfaceFunctionCall arg produced Break — vwat"),
                    INodeExecuteResultV::Continue(c) => c.result_ref,
                    INodeExecuteResultV::Error(_) => panic!("InterfaceFunctionCall arg produced Error — vimpl (closure can't propagate)"),
                }
            }).collect();
            {
                let handle = &mut *heap.vivem_dout;
                writeln!(handle).unwrap();
                writeln!(handle, "{}Making new stack frame (icall)", "  ".repeat(call_id.call_depth as usize)).unwrap();
            }
            for r in &undeviewed_arg_references {
                heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), *r);
            }
            let (callee_call_id, retuurn) = match execute_interface_function(program_h, interner, scout_arena, stdin, stdout, heap, heap.vivem_bump.alloc_slice_copy(&undeviewed_arg_references), ifc.virtual_param_index, ifc.super_function_prototype) {
                Ok(t) => t,
                Err(e) => return INodeExecuteResultV::Error(e),
            };
            let return_ref = match retuurn {
                INodeExecuteResultV::Return(ref r) => possess_callee_return(heap, call_id, callee_call_id, r),
                _ => panic!("InterfaceFunctionCall: callee did not return"),
            };
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: return_ref })
        }
        ExpressionIE::AsSubtype(a) => {
            let source_ref = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &a.source_expr) {
                r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            let (constructor, deviewed_args): (&'i PrototypeI<'s, 'i>, Vec<ReferenceV<'v, 'i, 's>>) =
                if source_ref.actual_kind.hamut == a.target_type {
                    let ref_aliased_as_subtype = heap.transmute(source_ref, a.source_expr.result(), a.ok_constructor.param_types()[0]);
                    (a.ok_constructor, vec![ref_aliased_as_subtype])
                } else {
                    (a.err_constructor, vec![source_ref])
                };
            {
                let handle = &mut *heap.vivem_dout;
                writeln!(handle).unwrap();
                writeln!(handle, "{}Making new stack frame (lock call)", "  ".repeat(expression_id.call_id.call_depth as usize)).unwrap();
            }
            let function = *program_h.functions.iter().find(|f| f.header.id == constructor.id).expect("AsSubtype: constructor not found");
            heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), source_ref);
            let args_slice: &'v [ReferenceV<'v, 'i, 's>] = heap.vivem_bump.alloc_slice_copy(&deviewed_args);
            let (callee_call_id, retuurn) = match execute_function(program_h, interner, scout_arena, stdin, stdout, heap, args_slice, function) {
                Ok(t) => t,
                Err(e) => return INodeExecuteResultV::Error(e),
            };
            {
                let handle = &mut *heap.vivem_dout;
                write!(handle, "{}Getting return reference", "  ".repeat(expression_id.call_id.call_depth as usize)).unwrap();
            }
            let return_ref = possess_callee_return(heap, call_id, callee_call_id, &retuurn);
            let target_interface_ref = match a.result {
                KindIT::InterfaceIT(i) => i,
                _ => panic!("AsSubtype: result not InterfaceIT"),
            };
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: upcast(return_ref, target_interface_ref) })
        }
        ExpressionIE::Upcast(u) => {
            let source_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &u.inner_expr) {
                r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            let target_reference = upcast(source_reference, &u.target_interface);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: target_reference })
        }
        ExpressionIE::InterfaceToInterfaceUpcast(i2i) => {
            let source_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &i2i.inner_expr) {
                r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            let target_reference = upcast(source_reference, &i2i.target_interface);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: target_reference })
        }
        ExpressionIE::RuntimeSizedArrayLookup(rsal) => {
            let array_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &rsal.array_expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Error(e) => return INodeExecuteResultV::Error(e),
            };
            let index_int_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 1), &rsal.index_expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Error(e) => return INodeExecuteResultV::Error(e),
            };
            let index = match heap.dereference(index_int_reference, false) {
                KindV::Int(int_v) if int_v.bits == 32 => int_v.value as i32,
                _ => panic!("execute_node_inner: RuntimeSizedArrayLookup index not IntV(_, 32)"),
            };
            let address = ElementAddressV { array_id: array_reference.alloc_id(), element_index: index as i64 };
            write!(heap.vivem_dout, " **o:{}.{}", address.array_id.num, address.element_index).unwrap();
            let source = heap.get_reference_from_array(interner, address, rsal.array_type.element_type(), KindIT::BorrowRefIT(rsal.result));
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), source);
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, rsal.index_expr.result(), index_int_reference) { return INodeExecuteResultV::Error(e); }
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, rsal.array_expr.result(), array_reference) { return INodeExecuteResultV::Error(e); }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: source })
        }
        ExpressionIE::PopRuntimeSizedArray(pop) => {
            let array_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &pop.array_expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Error(e) => return INodeExecuteResultV::Error(e),
            };
            let result_reference = heap.deinitialize_array_element(array_reference);
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), result_reference);
            let result_value = heap.dereference(result_reference, false);
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, pop.array_expr.result(), array_reference) { return INodeExecuteResultV::Error(e); }
            write!(heap.vivem_dout, " o{}-=", array_reference.num).unwrap();
            heap.print_kind(result_value);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: result_reference })
        }
        ExpressionIE::PushRuntimeSizedArray(prsa) => {
            let array_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &prsa.array_expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Error(e) => return INodeExecuteResultV::Error(e),
            };
            match heap.dereference(array_reference, false) {
                KindV::ArrayInstance(_) => {}
                _ => panic!("execute_node_inner: PushRuntimeSizedArray array deref not ArrayInstance"),
            };
            let newcomer_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &prsa.new_element_expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Error(e) => return INodeExecuteResultV::Error(e),
            };
            let newcomer_ve = heap.dereference(newcomer_reference, false);
            heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), newcomer_reference);
            heap.initialize_array_element(array_reference, newcomer_reference);
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, prsa.array_expr.result(), array_reference) { return INodeExecuteResultV::Error(e); }
            write!(heap.vivem_dout, " o{}+=", array_reference.num).unwrap();
            heap.print_kind(newcomer_ve);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: heap.void() })
        }
        ExpressionIE::NewRuntimeSizedArray(nrsa) => {
            let capacity_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &nrsa.capacity_expr) {
                INodeExecuteResultV::Return(r) => return INodeExecuteResultV::Return(r),
                INodeExecuteResultV::Break(b) => return INodeExecuteResultV::Break(b),
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Error(e) => return INodeExecuteResultV::Error(e),
            };
            let capacity_value = heap.dereference(capacity_reference, false);
            let capacity = match capacity_value {
                KindV::Int(int_v) if int_v.bits == 32 => int_v.value as i32,
                _ => panic!("execute_node_inner: NewRuntimeSizedArray capacity not IntV(_, 32)"),
            };
            let (array_reference, array_instance) = heap.add_uninitialized_array(interner, nrsa.result, capacity);
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), array_reference);
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, nrsa.capacity_expr.result(), capacity_reference) { return INodeExecuteResultV::Error(e); }
            write!(heap.vivem_dout, " o{}=", array_reference.num).unwrap();
            heap.print_kind(KindV::ArrayInstance(&array_instance));
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: array_reference })
        }
        ExpressionIE::IsSameInstance(isi) => {
            let left_ref = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &isi.left) {
                ret @ INodeExecuteResultV::Return(_) => return ret,
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Break(_) => panic!("execute_node_inner: IsSameInstance left produced Break — vwat"),
                ret @ INodeExecuteResultV::Error(_) => return ret,
            };
            let right_ref = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 1), &isi.right) {
                ret @ INodeExecuteResultV::Return(_) => return ret,
                INodeExecuteResultV::Continue(c) => c.result_ref,
                INodeExecuteResultV::Break(_) => panic!("execute_node_inner: IsSameInstance right produced Break — vwat"),
                ret @ INodeExecuteResultV::Error(_) => return ret,
            };
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, isi.left.result(), left_ref) { return INodeExecuteResultV::Error(e); }
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, isi.right.result(), right_ref) { return INodeExecuteResultV::Error(e); }
            let r#ref = heap.is_same_instance(interner, call_id, left_ref, right_ref);
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: r#ref })
        }
        ExpressionIE::StaticArrayFromCallable(cac) => {
            let generator_reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &cac.generator) {
                nr @ INodeExecuteResultV::Return(_) => return nr,
                INodeExecuteResultV::Break(_) => panic!("execute_node_inner: StaticArrayFromCallable generator produced Break — vwat"),
                INodeExecuteResultV::Continue(v) => v.result_ref,
                nr @ INodeExecuteResultV::Error(_) => return nr,
            };
            let mut element_refs: Vec<ReferenceV<'v, 'i, 's>> = Vec::new();
            if let Err(e) = generate_elements(program_h, interner, scout_arena, stdin, stdout, heap, expression_id, call_id, generator_reference, cac.generator_method, cac.array_type.size(), &mut |_i, element_ref, _heap| {
                element_refs.push(element_ref);
            }) { return INodeExecuteResultV::Error(e); }
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, cac.generator.result(), generator_reference) { return INodeExecuteResultV::Error(e); }
            let element_refs_slice: &'v [ReferenceV<'v, 'i, 's>] = heap.vivem_bump.alloc_slice_copy(&element_refs);
            let (array_reference, array_instance) = heap.add_array(interner, cac.result, element_refs_slice);
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), array_reference);
            write!(heap.vivem_dout, " o{}=", array_reference.num).unwrap();
            heap.print_kind(KindV::ArrayInstance(array_instance));
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: array_reference })
        }
        ExpressionIE::Restackify(rs) => {
            let reference = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &rs.source_expr) {
                ret @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return ret,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            let var_addr = get_var_address(expression_id.call_id, rs.variable);
            heap.add_local(interner, var_addr, reference, rs.variable.tyype);
            {
                let handle = &mut *heap.vivem_dout;
                write!(handle, " v{}/{:?}<-o{}", var_addr.call_id.call_depth, var_addr.name, reference.num).unwrap();
            }
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, rs.source_expr.result(), reference) { return INodeExecuteResultV::Error(e); }
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: heap.void() })
        }
        ExpressionIE::BorrowToWeak(btw) => {
            let constraint_ref = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &btw.inner_expr) {
                r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            assert!(constraint_ref.ownership == OwnershipV::Borrow);

            let weak_ref = heap.transmute(constraint_ref, btw.inner_expr.result(), btw.result);
            heap.increment_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), weak_ref);
            if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, btw.inner_expr.result(), constraint_ref) { return INodeExecuteResultV::Error(e); }

            INodeExecuteResultV::Continue(NodeContinueV { result_ref: weak_ref })
        }
        ExpressionIE::LockWeak(lw) => {
            let weak_ref = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &lw.inner_expr) {
                r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            assert!(weak_ref.ownership == OwnershipV::Weak);

            if heap.contains_live_object(weak_ref) {
                // Live: transmute the weak into the borrow (constraint) type the Some-constructor wants.
                let constraint_type = lw.some_constructor.param_types()[0];
                let constraint_ref = heap.transmute(weak_ref, lw.inner_expr.result(), constraint_type);
                {
                    let handle = &mut *heap.vivem_dout;
                    writeln!(handle).unwrap();
                    writeln!(handle, "{}Making new stack frame (lock call)", "  ".repeat(expression_id.call_id.call_depth as usize)).unwrap();
                }
                let function = *program_h.functions.iter().find(|f| f.header.id == lw.some_constructor.id).expect("LockWeak: some_constructor not found");
                heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), weak_ref);
                let args_slice: &'v [ReferenceV<'v, 'i, 's>] = heap.vivem_bump.alloc_slice_copy(&[constraint_ref]);
                let (callee_call_id, retuurn) = match execute_function(program_h, interner, scout_arena, stdin, stdout, heap, args_slice, function) { Ok(t) => t, Err(e) => return INodeExecuteResultV::Error(e), };
                {
                    let handle = &mut *heap.vivem_dout;
                    write!(handle, "{}Getting return reference", "  ".repeat(expression_id.call_id.call_depth as usize)).unwrap();
                }
                let return_ref = possess_callee_return(heap, call_id, callee_call_id, &retuurn);
                let target_interface_ref = match lw.result {
                    KindIT::InterfaceIT(i) => i,
                    _ => panic!("LockWeak: result not InterfaceIT"),
                };
                INodeExecuteResultV::Continue(NodeContinueV { result_ref: upcast(return_ref, target_interface_ref) })
            } else {
                if let Err(e) = discard(program_h, interner, scout_arena, heap, stdout, stdin, call_id, lw.inner_expr.result(), weak_ref) { return INodeExecuteResultV::Error(e); }
                {
                    let handle = &mut *heap.vivem_dout;
                    writeln!(handle).unwrap();
                    writeln!(handle, "{}Making new stack frame (lock call)", "  ".repeat(expression_id.call_id.call_depth as usize)).unwrap();
                }
                let function = *program_h.functions.iter().find(|f| f.header.id == lw.none_constructor.id).expect("LockWeak: none_constructor not found");
                let args_slice: &'v [ReferenceV<'v, 'i, 's>] = heap.vivem_bump.alloc_slice_copy(&[]);
                let (callee_call_id, retuurn) = match execute_function(program_h, interner, scout_arena, stdin, stdout, heap, args_slice, function) { Ok(t) => t, Err(e) => return INodeExecuteResultV::Error(e), };
                {
                    let handle = &mut *heap.vivem_dout;
                    write!(handle, "{}Getting return reference", "  ".repeat(expression_id.call_id.call_depth as usize)).unwrap();
                }
                let return_ref = possess_callee_return(heap, call_id, callee_call_id, &retuurn);
                let target_interface_ref = match lw.result {
                    KindIT::InterfaceIT(i) => i,
                    _ => panic!("LockWeak: result not InterfaceIT"),
                };
                INodeExecuteResultV::Continue(NodeContinueV { result_ref: upcast(return_ref, target_interface_ref) })
            }
        }
        ExpressionIE::CopyPrim(cp) => {
            let inner_ref = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, expression_id.add_step(heap.vivem_bump, 0), &cp.inner) {
                r @ (INodeExecuteResultV::Return(_) | INodeExecuteResultV::Break(_) | INodeExecuteResultV::Error(_)) => return r,
                INodeExecuteResultV::Continue(c) => c.result_ref,
            };
            INodeExecuteResultV::Continue(NodeContinueV { result_ref: inner_ref })
        }
        // VCOORD: revisit
        // Source and target coord are the same MutableShare-flavored ref today (Borrow-of-share collapses at
        // instantiator), so executing inner and passing through is correct. Once Backend supports
        // Borrow-of-share, this arm gains RC-bump semantics for the reflavor.
        // (Reinterpret/Alias arm removed — the onion IR no longer has a Reinterpret variant.)
        other => panic!("execute_node_inner: unimplemented arm {:?}", discriminant(other)),
    }
}


pub fn consume_elements<'v, 'i, 's>(program_h: &'i HinputsI<'s, 'i>, interner: &InstantiatingInterner<'s, 'i>, scout_arena: &ScoutArena<'s>, stdin: &'v dyn Fn() -> StrI<'s>, stdout: &'v dyn Fn(StrI<'s>), heap: &mut HeapV<'v, 'i, 's>, _expression_id: ExpressionIdV<'v, 'i, 's>, call_id: CallIdV<'v, 'i, 's>, array_reference: ReferenceV<'v, 'i, 's>, consumer_reference: ReferenceV<'v, 'i, 's>, consumer_prototype: PrototypeI<'s, 'i>, size: i64, receiver: &mut dyn FnMut(i64, ReferenceV<'v, 'i, 's>)) -> Result<(), VmRuntimeErrorV<'s>> {
    let consumer_function = *program_h.functions.iter().find(|f| f.header.id == consumer_prototype.id).expect("consume_elements: consumer not found");
    for i in (0..size).rev() {
        writeln!(heap.vivem_dout).unwrap();
        let prefix = "  ".repeat(call_id.call_depth as usize);
        writeln!(heap.vivem_dout, "{}Making new stack frame (consumer)", prefix).unwrap();
        writeln!(heap.vivem_dout).unwrap();
        let element_addr = ElementAddressV { array_id: array_reference.alloc_id(), element_index: i };
        write!(heap.vivem_dout, " *{}", element_addr.to_string()).unwrap();
        let element_reference = heap.deinitialize_array_element(array_reference);
        writeln!(heap.vivem_dout).unwrap();
        writeln!(heap.vivem_dout, "{}Making new stack frame (icall)", prefix).unwrap();
        let args_vec: Vec<ReferenceV<'v, 'i, 's>> = vec![consumer_reference, element_reference];
        let args_slice: &'v [ReferenceV<'v, 'i, 's>] = heap.vivem_bump.alloc_slice_copy(&args_vec);
        let (callee_call_id, retuurn) = execute_function(program_h, interner, scout_arena, stdin, stdout, heap, args_slice, consumer_function)?;
        write!(heap.vivem_dout, "{}Getting return reference", prefix).unwrap();
        let return_ref = possess_callee_return(heap, call_id, callee_call_id, &retuurn);
        heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), return_ref);
        receiver(i, return_ref);
    }
    Ok(())
}


pub fn generate_elements<'v, 'i, 's>(program_h: &'i HinputsI<'s, 'i>, interner: &InstantiatingInterner<'s, 'i>, scout_arena: &ScoutArena<'s>, stdin: &'v dyn Fn() -> StrI<'s>, stdout: &'v dyn Fn(StrI<'s>), heap: &mut HeapV<'v, 'i, 's>, _expression_id: ExpressionIdV<'v, 'i, 's>, call_id: CallIdV<'v, 'i, 's>, generator_reference: ReferenceV<'v, 'i, 's>, generator_prototype: PrototypeI<'s, 'i>, size: i64, receiver: &mut dyn FnMut(i64, ReferenceV<'v, 'i, 's>, &mut HeapV<'v, 'i, 's>)) -> Result<(), VmRuntimeErrorV<'s>> {
    let generator_function = *program_h.functions.iter().find(|f| f.header.id == generator_prototype.id).expect("generate_elements: generator not found");
    for i in 0..size {
        {
            let handle = &mut *heap.vivem_dout;
            writeln!(handle).unwrap();
            let prefix = "  ".repeat(call_id.call_depth as usize);
            writeln!(handle, "{}Making new stack frame (generator)", prefix).unwrap();
        }
        let index_reference = heap.allocate_transient(interner, OwnershipV::Own, KindV::Int(IntV { value: i, bits: 32, _phantom: PhantomData }));
        {
            let handle = &mut *heap.vivem_dout;
            writeln!(handle).unwrap();
            writeln!(handle).unwrap();
            let prefix = "  ".repeat(call_id.call_depth as usize);
            writeln!(handle, "{}Making new stack frame (icall)", prefix).unwrap();
        }
        let args = heap.vivem_bump.alloc_slice_copy(&[generator_reference, index_reference]);
        let (callee_call_id, retuurn) = execute_function(program_h, interner, scout_arena, stdin, stdout, heap, args, generator_function)?;
        {
            let handle = &mut *heap.vivem_dout;
            let prefix = "  ".repeat(call_id.call_depth as usize);
            write!(handle, "{}Getting return reference", prefix).unwrap();
        }
        let return_ref = possess_callee_return(heap, call_id, callee_call_id, &retuurn);
        heap.decrement_reference_ref_count(IObjectReferrerV::RegisterToObjectReferrer(RegisterToObjectReferrerV { call_id }), return_ref);
        receiver(i, return_ref, heap);
    }
    Ok(())
}


pub fn execute_interface_function<'v, 'i, 's>(_program_h: &'i HinputsI<'s, 'i>, _interner: &InstantiatingInterner<'s, 'i>, _scout_arena: &ScoutArena<'s>, _stdin: &'v dyn Fn() -> StrI<'s>, _stdout: &'v dyn Fn(StrI<'s>), _heap: &mut HeapV<'v, 'i, 's>, _undeviewed_arg_references: &'v [ReferenceV<'v, 'i, 's>], _virtual_param_index: i32, _super_function_prototype: &'i PrototypeI<'s, 'i>) -> Result<(CallIdV<'v, 'i, 's>, INodeExecuteResultV<'v, 'i, 's>), VmRuntimeErrorV<'s>> {
    // Onion interface dispatch is a genuine sub-port, not yet done: edges moved off the struct def
    // into `HinputsI.interface_to_sub_citizen_to_edge` with a changed `EdgeI` shape, and the
    // deviewing (interface ref -> concrete struct ref) plus the method-index into the edge need the
    // onion edge model worked out. Unexercised by the pilot.
    panic!("Unimplemented: execute_interface_function (onion interface dispatch)");
}


pub fn discard<'v, 'i, 's>(program_h: &'i HinputsI<'s, 'i>, interner: &InstantiatingInterner<'s, 'i>, scout_arena: &ScoutArena<'s>, heap: &mut HeapV<'v, 'i, 's>, stdout: &'v dyn Fn(StrI<'s>), stdin: &'v dyn Fn() -> StrI<'s>, call_id: CallIdV<'v, 'i, 's>, expected_reference: KindIT<'s, 'i>, actual_reference: ReferenceV<'v, 'i, 's>) -> Result<(), VmRuntimeErrorV<'s>> {
    heap.decrement_reference_ref_count(
        IObjectReferrerV::RegisterToObjectReferrer(
            RegisterToObjectReferrerV { call_id }
        ),
        actual_reference,
    );
    cleanup(program_h, interner, heap, stdout, stdin, call_id, expected_reference, actual_reference)
}


pub fn cleanup<'v, 'i, 's>(program_h: &'i HinputsI<'s, 'i>, interner: &InstantiatingInterner<'s, 'i>, heap: &mut HeapV<'v, 'i, 's>, stdout: &dyn Fn(StrI<'s>), stdin: &dyn Fn() -> StrI<'s>, call_id: CallIdV<'v, 'i, 's>, expected_reference: KindIT<'s, 'i>, actual_reference: ReferenceV<'v, 'i, 's>) -> Result<(), VmRuntimeErrorV<'s>> {
    if heap.get_total_ref_count(actual_reference) != 0 {
        return Ok(());
    }
    let expected = RRKindV { hamut: expected_reference, _phantom: PhantomData };
    match expected.outer_ownership() {
        OwnershipV::Weak => {
            heap.deallocate_if_no_weak_refs(actual_reference)?;
            Ok(())
        }
        OwnershipV::Borrow => Ok(()),
        OwnershipV::Own | OwnershipV::Share => {
            // Strong reference at refcount-zero: dispatch on the bare kind. Own and Share share
            // the same cleanup shape — primitives deallocate inline; aggregates destructure and
            // recurse on members/elements.
            match expected.strip_outer_references().hamut {
                KindIT::VoidIT(_) | KindIT::IntIT(_) | KindIT::BoolIT(_) | KindIT::FloatIT(_) | KindIT::StrIT(_) | KindIT::NeverIT(_) | KindIT::USizeIT(_) => {
                    heap.zero(actual_reference);
                    heap.deallocate_if_no_weak_refs(actual_reference)?;
                }
                KindIT::StructIT(sr) => {
                    let struct_def = program_h.lookup_struct(&sr.id);
                    let member_expected_types: Vec<KindIT<'s, 'i>> = struct_def.members.iter().map(|m| m.tyype).collect();
                    let member_refs = heap.destructure(actual_reference)?;
                    assert_eq!(member_expected_types.len(), member_refs.len());
                    for (member_ref, member_expected_type) in member_refs.iter().zip(member_expected_types.iter()) {
                        cleanup(program_h, interner, heap, stdout, stdin, call_id, *member_expected_type, *member_ref)?;
                    }
                }
                KindIT::InterfaceIT(_ir) => {
                    let actual_concrete_type = match actual_reference.actual_kind.hamut {
                        KindIT::StructIT(sr) => sr,
                        _ => panic!("cleanup: InterfaceIT actual_kind not StructIT"),
                    };
                    let struct_def = program_h.lookup_struct(&actual_concrete_type.id);
                    let member_expected_types: Vec<KindIT<'s, 'i>> = struct_def.members.iter().map(|m| m.tyype).collect();
                    let member_refs = heap.destructure(actual_reference)?;
                    assert_eq!(member_expected_types.len(), member_refs.len());
                    for (member_ref, member_expected_type) in member_refs.iter().zip(member_expected_types.iter()) {
                        cleanup(program_h, interner, heap, stdout, stdin, call_id, *member_expected_type, *member_ref)?;
                    }
                }
                KindIT::RuntimeSizedArrayIT(rsa) => {
                    let element_refs = heap.destructure_array(actual_reference);
                    let element_type = rsa.element_type();
                    for element_ref in element_refs.iter() {
                        cleanup(program_h, interner, heap, stdout, stdin, call_id, element_type, *element_ref)?;
                    }
                    heap.zero(actual_reference);
                    heap.deallocate_if_no_weak_refs(actual_reference)?;
                }
                KindIT::StaticSizedArrayIT(ssa) => {
                    let element_refs = heap.destructure_array(actual_reference);
                    let element_type = ssa.element_type();
                    for element_ref in element_refs.iter() {
                        cleanup(program_h, interner, heap, stdout, stdin, call_id, element_type, *element_ref)?;
                    }
                    heap.zero(actual_reference);
                    heap.deallocate_if_no_weak_refs(actual_reference)?;
                }
                KindIT::BorrowRefIT(_) | KindIT::OwnRefIT(_) | KindIT::ShareRefIT(_) | KindIT::WeakRefIT(_) => {
                    unreachable!("cleanup: kind still wrapped after strip_outer_references");
                }
            }
            Ok(())
        }
    }
}

