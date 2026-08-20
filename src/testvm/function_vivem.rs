use crate::interner::StrI;
use crate::instantiating::ast::hinputs::HinputsI;
use crate::instantiating::ast::ast::{FunctionDefinitionI, PrototypeI};
use crate::instantiating::ast::names::INameI;
use crate::instantiating::instantiating_interner::InstantiatingInterner;
use crate::testvm::values::{CallIdV, ReferenceV};
use crate::testvm::heap::{AdapterForExternsV, HeapV};
use crate::testvm::expression_vivem::NodeReturnV;
use crate::scout_arena::ScoutArena;
use crate::testvm::expression_vivem::INodeExecuteResultV;
use crate::testvm::expression_vivem::execute_node;
use crate::testvm::values::ArgumentIdV;
use crate::testvm::values::ArgumentToObjectReferrerV;
use crate::testvm::values::ExpressionIdV;
use crate::testvm::values::IObjectReferrerV;
use crate::testvm::vivem::VmRuntimeErrorV;
use crate::testvm::vivem_externs::add_float_float;
use crate::testvm::vivem_externs::add_i32;
use crate::testvm::vivem_externs::add_str_str;
use crate::testvm::vivem_externs::cast_float_i32;
use crate::testvm::vivem_externs::cast_float_str;
use crate::testvm::vivem_externs::cast_i32_float;
use crate::testvm::vivem_externs::cast_i32_str;
use crate::testvm::vivem_externs::cast_i64_str;
use crate::testvm::vivem_externs::divide_float_float;
use crate::testvm::vivem_externs::divide_i32;
use crate::testvm::vivem_externs::divide_i64;
use crate::testvm::vivem_externs::eq_bool_bool;
use crate::testvm::vivem_externs::eq_float_float;
use crate::testvm::vivem_externs::eq_i32;
use crate::testvm::vivem_externs::eq_str_str;
use crate::testvm::vivem_externs::getch;
use crate::testvm::vivem_externs::greater_than_i32;
use crate::testvm::vivem_externs::greater_than_or_eq_i32;
use crate::testvm::vivem_externs::less_than_float;
use crate::testvm::vivem_externs::less_than_i32;
use crate::testvm::vivem_externs::less_than_or_eq_i32;
use crate::testvm::vivem_externs::mod_i32;
use crate::testvm::vivem_externs::multiply_float_float;
use crate::testvm::vivem_externs::multiply_i32;
use crate::testvm::vivem_externs::multiply_i64;
use crate::testvm::vivem_externs::negate_float;
use crate::testvm::vivem_externs::new_vec;
use crate::testvm::vivem_externs::new_vec_with_capacity;
use crate::testvm::vivem_externs::not;
use crate::testvm::vivem_externs::panic;
use crate::testvm::vivem_externs::print;
use crate::testvm::vivem_externs::sqrt;
use crate::testvm::vivem_externs::str_length;
use crate::testvm::vivem_externs::subtract_float_float;
use crate::testvm::vivem_externs::subtract_i32;
use crate::testvm::vivem_externs::subtract_i64;
use crate::testvm::vivem_externs::truncate_i64_to_i32;
use crate::testvm::vivem_externs::vec_capacity;
use std::io::Write;


pub fn execute_function<'i, 's, 'v>(
    program_h: &'i HinputsI<'s, 'i>,
    interner: &InstantiatingInterner<'s, 'i>, scout_arena: &ScoutArena<'s>, stdin: &'v dyn Fn() -> StrI<'s>,
    stdout: &'v dyn Fn(StrI<'s>),
    heap: &mut HeapV<'v, 'i, 's>,
    args: &'v [ReferenceV<'v, 'i, 's>],
    function_h: &'i FunctionDefinitionI<'s, 'i>,
) -> Result<(CallIdV<'v, 'i, 's>, NodeReturnV<'v, 'i, 's>), VmRuntimeErrorV<'s>> {
    // The stack-frame call-id needs an `&'i PrototypeI`; the header computes one by value, so
    // arena-allocate it.
    let prototype: &'i PrototypeI<'s, 'i> = interner.bump().alloc(function_h.header.to_prototype());
    let call_id = heap.push_new_stack_frame(prototype, args);
    {
        let handle = &mut *heap.vivem_dout;
        let prefix = "  ".repeat(call_id.call_depth as usize);
        write!(handle, "{}Entering function {}", prefix, call_id).unwrap();
    }
    // Increment all the args to show that they have arguments referring to them.
    // These will be decremented at some point in the callee function.
    for arg_index in 0..args.len() {
        let arg_index_i32 = arg_index as i32;
        heap.increment_reference_ref_count(
            IObjectReferrerV::ArgumentToObjectReferrer(ArgumentToObjectReferrerV {
                argument_id: ArgumentIdV { call_id, index: arg_index_i32 },
            }),
            args[arg_index],
        );
    }
    {
        let handle = &mut *heap.vivem_dout;
        writeln!(handle).unwrap();
    }
    let root_expression_id = ExpressionIdV { call_id, path: &[] };
    let return_ref = match execute_node(program_h, interner, scout_arena, stdin, stdout, heap, root_expression_id, &function_h.body) {
        INodeExecuteResultV::Return(r) => NodeReturnV { return_ref: r.return_ref },
        INodeExecuteResultV::Break(_) => panic!("execute_function: NodeBreak vwat"),
        INodeExecuteResultV::Continue(c) => NodeReturnV { return_ref: c.result_ref },
        INodeExecuteResultV::Error(e) => return Err(e),
    };
    {
        let handle = &mut *heap.vivem_dout;
        writeln!(handle).unwrap();
        let prefix = "  ".repeat(call_id.call_depth as usize);
        write!(handle, "{}Returning", prefix).unwrap();
    }
    heap.pop_stack_frame(call_id);
    {
        let handle = &mut *heap.vivem_dout;
        writeln!(handle).unwrap();
    }
    Ok((call_id, return_ref))
}

pub fn get_extern_function<'i, 's, 'v>(
    _program_h: &HinputsI<'s, 'i>,
    ref_: &PrototypeI<'s, 'i>,
) -> Box<dyn for<'a> Fn(&mut AdapterForExternsV<'a, 'v, 'i, 's>, &'v [ReferenceV<'v, 'i, 's>]) -> Result<ReferenceV<'v, 'i, 's>, VmRuntimeErrorV<'s>> + 'i>
where 's: 'i, 'i: 'v,
{
    // Externs are ordinary functions carrying the `extern` attribute, so their prototype name is a
    // FunctionNameIX; the builtin dispatch key is its human name (e.g. "__vbi_addI32").
    let name = match ref_.id.local_name {
        INameI::FunctionNameIX(n) => n.template.human_name.0,
        // A builtin/extern function's prototype carries an ExternFunctionNameI, whose human_name is
        // the dispatch key (e.g. "__vbi_addI32").
        INameI::ExternFunction(n) => n.human_name.0,
        other => panic!("get_extern_function: unexpected prototype name variant {:?}", other),
    };
    match name {
        "__vbi_addI32" => Box::new(add_i32),
        "__vbi_addFloatFloat" => Box::new(add_float_float),
        "__vbi_panic" => Box::new(panic),
        "__vbi_multiplyI32" => Box::new(multiply_i32),
        "__vbi_subtractFloatFloat" => Box::new(subtract_float_float),
        "__vbi_divideI32" => Box::new(divide_i32),
        "__vbi_multiplyFloatFloat" => Box::new(multiply_float_float),
        "__vbi_divideFloatFloat" => Box::new(divide_float_float),
        "__vbi_subtractI32" => Box::new(subtract_i32),
        "__vbi_addStr" => Box::new(add_str_str),
        "__getch" => Box::new(getch),
        "__vbi_eqFloatFloat" => Box::new(eq_float_float),
        "sqrt" => Box::new(sqrt),
        "__vbi_lessThanI32" => Box::new(less_than_i32),
        "__vbi_lessThanFloat" => Box::new(less_than_float),
        "__vbi_greaterThanOrEqI32" => Box::new(greater_than_or_eq_i32),
        "__vbi_greaterThanI32" => Box::new(greater_than_i32),
        "__vbi_eqI32" => Box::new(eq_i32),
        "__vbi_eqBoolBool" => Box::new(eq_bool_bool),
        "__vbi_printstr" => Box::new(print),
        "__vbi_not" => Box::new(not),
        "__vbi_castI32Str" => Box::new(cast_i32_str),
        "__vbi_castI64Str" => Box::new(cast_i64_str),
        "castI32Float" => Box::new(cast_i32_float),
        "castFloatI32" => Box::new(cast_float_i32),
        "__vbi_lessThanOrEqI32" => Box::new(less_than_or_eq_i32),
        "__vbi_modI32" => Box::new(mod_i32),
        "__vbi_strLength" => Box::new(str_length),
        "__vbi_castFloatStr" => Box::new(cast_float_str),
        "__vbi_streq" => Box::new(eq_str_str),
        "__vbi_negateFloat" => Box::new(negate_float),
        "__vbi_multiplyI64" => Box::new(multiply_i64),
        "__vbi_divideI64" => Box::new(divide_i64),
        "__vbi_subtractI64" => Box::new(subtract_i64),
        "TruncateI64ToI32" => Box::new(truncate_i64_to_i32),
        "VecOuterNew<i32>" => { let proto = *ref_; Box::new(move |memory, args| new_vec(memory, &proto, args)) },
        "Vec.new<i32>" => { let proto = *ref_; Box::new(move |memory, args| new_vec(memory, &proto, args)) },
        "Vec.with_capacity<i32>" => { let proto = *ref_; Box::new(move |memory, args| new_vec_with_capacity(memory, &proto, args)) },
        "Vec.capacity<i32>" => { let proto = *ref_; Box::new(move |memory, args| vec_capacity(memory, &proto, args)) },
        other => panic!("get_extern_function: unimplemented extern {}", other),
    }
}
