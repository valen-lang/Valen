use crate::interner::StrI;
use crate::final_ast::ast::{FunctionH, ProgramH, PrototypeH};
use crate::final_ast::types::KindHT;
use crate::testvm::values::{CallIdV, ReferenceV};
use crate::testvm::heap::{AdapterForExternsV, HeapV};
use crate::testvm::expression_vivem::NodeReturnV;
use crate::scout_arena::ScoutArena;
use crate::simplifying::hammer_interner::HammerInterner;
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

/*
package dev.vale.testvm

import dev.vale.finalast.{FunctionH, ProgramH, PrototypeH}
import dev.vale.testvm.ExpressionVivem.NodeReturn
import dev.vale.finalast._
import ExpressionVivem.{NodeBreak, NodeContinue, NodeReturn}
import dev.vale.{vimpl, vwat}
import dev.vale.{finalast => m}

object FunctionVivem {
*/
// mig: fn execute_function
pub fn execute_function<'h, 's, 'v>(
    program_h: &'h ProgramH<'s, 'h>,
    interner: &HammerInterner<'s, 'h>, scout_arena: &ScoutArena<'s>, stdin: &'v dyn Fn() -> StrI<'s>,
    stdout: &'v dyn Fn(StrI<'s>),
    heap: &mut HeapV<'v, 'h, 's>,
    args: &'v [ReferenceV<'v, 'h, 's>],
    function_h: &'h FunctionH<'s, 'h>,
) -> Result<(CallIdV<'v, 'h, 's>, NodeReturnV<'v, 'h, 's>), VmRuntimeErrorV<'s>> {
    let call_id = heap.push_new_stack_frame(function_h.prototype, args);
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
                ownership: args[arg_index].ownership,
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
/*
  def executeFunction(
      programH: ProgramH,
      stdin: (() => String),
      stdout: (String => Unit),
      heap: Heap,
      args: Vector[ReferenceV],
      functionH: FunctionH
  ): (CallId, NodeReturn) = {
    val callId = heap.pushNewStackFrame(functionH.prototype, args)

    heap.vivemDout.print("  " * callId.callDepth + "Entering function " + callId)

    // Increment all the args to show that they have arguments referring to them.
    // These will be decremented at some point in the callee function.
    args.indices.foreach(argIndex => {
      heap.incrementReferenceRefCount(
        ArgumentToObjectReferrer(ArgumentId(callId, argIndex), args(argIndex).ownership),
        args(argIndex))
    })

    heap.vivemDout.println()

    val rootExpressionId = ExpressionId(callId, Vector.empty)
    val returnRef =
      ExpressionVivem.executeNode(programH, stdin, stdout, heap, rootExpressionId, functionH.body) match {
        case NodeReturn(r) => NodeReturn(r)
        case NodeBreak() => vwat()
        case NodeContinue(r) => NodeReturn(r)
      }

    heap.vivemDout.println()
    heap.vivemDout.print("  " * callId.callDepth + "Returning")

    heap.popStackFrame(callId)

    heap.vivemDout.println()

    (callId, returnRef)
  }
*/
// mig: fn get_extern_function
pub fn get_extern_function<'h, 's, 'v>(
    _program_h: &ProgramH<'s, 'h>,
    ref_: &PrototypeH<'s, 'h>,
) -> Box<dyn for<'a> Fn(&mut AdapterForExternsV<'a, 'v, 'h, 's>, &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> + 'h>
where 's: 'h, 'h: 'v,
{
    let name = ref_.id.fully_qualified_name.0.replace("v::builtins::arith", "");
    match name.as_str() {
        "__vbi_addI32" => Box::new(add_i32),
        "__vbi_addFloatFloat" => Box::new(add_float_float),
        "__vbi_panic" => Box::new(panic),
        "__vbi_multiplyI32" => Box::new(multiply_i32),
        "__vbi_subtractFloatFloat" => Box::new(subtract_float_float),
        "__vbi_divideI32" => Box::new(divide_i32),
        "__vbi_multiplyFloatFloat" => Box::new(multiply_float_float),
        "__vbi_divideFloatFloat" => Box::new(divide_float_float),
        "__vbi_subtractI32" => Box::new(subtract_i32),
        "addStr" => Box::new(add_str_str),
        "__getch" => Box::new(getch),
        "__vbi_eqFloatFloat" => Box::new(eq_float_float),
        "sqrt" => Box::new(sqrt),
        "__vbi_lessThanI32" => Box::new(less_than_i32),
        "__vbi_lessThanFloat" => Box::new(less_than_float),
        "__vbi_greaterThanOrEqI32" => Box::new(greater_than_or_eq_i32),
        "__vbi_greaterThanI32" => Box::new(greater_than_i32),
        "__vbi_eqI32" => Box::new(eq_i32),
        "__vbi_eqBoolBool" => Box::new(eq_bool_bool),
        "printstr" => Box::new(print),
        "__vbi_not" => Box::new(not),
        "castI32Str" => Box::new(cast_i32_str),
        "castI64Str" => Box::new(cast_i64_str),
        "castI32Float" => Box::new(cast_i32_float),
        "castFloatI32" => Box::new(cast_float_i32),
        "__vbi_lessThanOrEqI32" => Box::new(less_than_or_eq_i32),
        "__vbi_modI32" => Box::new(mod_i32),
        "__vbi_strLength" => Box::new(str_length),
        "castFloatStr" => Box::new(cast_float_str),
        "streq" => Box::new(eq_str_str),
        "__vbi_negateFloat" => Box::new(negate_float),
        "__vbi_multiplyI64" => Box::new(multiply_i64),
        "__vbi_divideI64" => Box::new(divide_i64),
        "__vbi_subtractI64" => Box::new(subtract_i64),
        "TruncateI64ToI32" => Box::new(truncate_i64_to_i32),
        "VecOuterNew<i32>" => { let ref_ = ref_.clone(); Box::new(move |memory, args| new_vec(memory, &ref_, args)) },
        "Vec.new<i32>" => { let ref_ = ref_.clone(); Box::new(move |memory, args| new_vec(memory, &ref_, args)) },
        "Vec.with_capacity<i32>" => { let ref_ = ref_.clone(); Box::new(move |memory, args| new_vec_with_capacity(memory, &ref_, args)) },
        "Vec.capacity<i32>" => { let ref_ = ref_.clone(); Box::new(move |memory, args| vec_capacity(memory, &ref_, args)) },
        other => panic!("get_extern_function: unimplemented extern {}", other),
    }
}
/*
  def getExternFunction(programH: ProgramH, ref: PrototypeH): (AdapterForExterns, Vector[ReferenceV]) => ReferenceV = {
    ref.id.fullyQualifiedName
      // The tests have a mode where they can interpret the builtins as separate packages, instead
      // of pulling it all in as one giant namespace. In that case, it prefixes things such as
      // v::builtins::arith. We can add other prefixes here too as needed.
      .replaceAllLiterally("v::builtins::arith", "") match {
      case """__vbi_addI32""" => VivemExterns.addI32
      case """__vbi_addFloatFloat""" => VivemExterns.addFloatFloat
      case """__vbi_panic""" => VivemExterns.panic
      case """__vbi_multiplyI32""" => VivemExterns.multiplyI32
      case """__vbi_subtractFloatFloat""" => VivemExterns.subtractFloatFloat
      case """__vbi_divideI32""" => VivemExterns.divideI32
      case """__vbi_multiplyFloatFloat""" => VivemExterns.multiplyFloatFloat
      case """__vbi_divideFloatFloat""" => VivemExterns.divideFloatFloat
      case """__vbi_subtractI32""" => VivemExterns.subtractI32
      case """addStr""" => VivemExterns.addStrStr
      case """__getch""" => VivemExterns.getch
      case """__vbi_eqFloatFloat""" => VivemExterns.eqFloatFloat
      case """sqrt""" => VivemExterns.sqrt
      case """__vbi_lessThanI32""" => VivemExterns.lessThanI32
      case """__vbi_lessThanFloat""" => VivemExterns.lessThanFloat
      case """__vbi_greaterThanOrEqI32""" => VivemExterns.greaterThanOrEqI32
      case """__vbi_greaterThanI32""" => VivemExterns.greaterThanI32
      case """__vbi_eqI32""" => VivemExterns.eqI32
      case """__vbi_eqBoolBool""" => VivemExterns.eqBoolBool
      case """printstr""" => VivemExterns.print
      case """__vbi_not""" => VivemExterns.not
      case """castI32Str""" => VivemExterns.castI32Str
      case """castI64Str""" => VivemExterns.castI64Str
      case """castI32Float""" => VivemExterns.castI32Float
      case """castFloatI32""" => VivemExterns.castFloatI32
      case """__vbi_lessThanOrEqI32""" => VivemExterns.lessThanOrEqI32
      case """__vbi_and""" => VivemExterns.and
      case """__vbi_or""" => VivemExterns.or
      case """__vbi_modI32""" => VivemExterns.modI32
      case """__vbi_strLength""" => VivemExterns.strLength
      case """castFloatStr""" => VivemExterns.castFloatStr
      case """streq""" => VivemExterns.eqStrStr
      case """__vbi_negateFloat""" => VivemExterns.negateFloat
      case """__vbi_addI64""" => VivemExterns.addI64
      case """__vbi_multiplyI64""" => VivemExterns.multiplyI64
      case """__vbi_divideI64""" => VivemExterns.divideI64
      case """__vbi_subtractI64""" => VivemExterns.subtractI64
      case """__vbi_lessThanI64""" => VivemExterns.lessThanI64
      case """__vbi_greaterThanOrEqI64""" => VivemExterns.greaterThanOrEqI64
      case """__vbi_eqI64""" => VivemExterns.eqI64
      case """__vbi_castI64Str""" => VivemExterns.castI64Str
      case """__vbi_castI64Float""" => VivemExterns.castI64Float
      case """__vbi_castFloatI64""" => VivemExterns.castFloatI64
      case """__vbi_lessThanOrEqI64""" => VivemExterns.lessThanOrEqI64
      case """__vbi_modI64""" => VivemExterns.modI64
      case """TruncateI64ToI32""" => VivemExterns.truncateI64ToI32
      case """VecOuterNew<i32>""" => VivemExterns.newVec(_, ref, _)
      case """Vec.new<i32>""" => VivemExterns.newVec(_, ref, _)
      case """Vec.with_capacity<i32>""" => VivemExterns.newVecWithCapacity(_, ref, _)
      case """Vec.capacity<i32>""" => VivemExterns.vecCapacity(_, ref, _)
      case _ => vimpl(ref.id.fullyQualifiedName)
    }
  }
*/
/*
}
*/
