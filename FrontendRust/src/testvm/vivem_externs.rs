use crate::testvm::values::ReferenceV;
use crate::testvm::heap::AdapterForExternsV;
use crate::final_ast::ast::PrototypeH;
use crate::final_ast::types::CoordH;
use crate::final_ast::types::KindHT;
use crate::final_ast::types::LocationH;
use crate::final_ast::types::OwnershipH;
use crate::testvm::values::BoolV;
use crate::testvm::values::FloatV;
use crate::testvm::values::IntV;
use crate::testvm::values::KindV;
use crate::testvm::values::StrV;
use crate::testvm::vivem::PanicExceptionV;
use crate::testvm::vivem::VmRuntimeErrorV;
use std::marker::PhantomData;

/*
package dev.vale.testvm

import dev.vale.finalast._
import dev.vale.{vassert, vfail}

import java.lang.ArithmeticException
import dev.vale.vfail

object VivemExterns {
*/
// mig: fn panic
pub fn panic<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 0);
    Err(VmRuntimeErrorV::PanicException(PanicExceptionV))
}
/*
  def panic(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 0)
    throw new PanicException()
  }
*/
// mig: fn add_float_float
pub fn add_float_float<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Float(FloatV { value: a_value, .. }), KindV::Float(FloatV { value: b_value, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Float(FloatV { value: a_value + b_value, _phantom: PhantomData }))
        }
        _ => panic!("add_float_float: non-FloatV args"),
    })
}
/*
  def addFloatFloat(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (FloatV(aValue), FloatV(bValue)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, FloatV(aValue + bValue))
      }
    }
  }
*/
// mig: fn multiply_float_float
pub fn multiply_float_float<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Float(FloatV { value: a_value, .. }), KindV::Float(FloatV { value: b_value, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Float(FloatV { value: a_value * b_value, _phantom: PhantomData }))
        }
        _ => panic!("multiply_float_float: non-FloatV args"),
    })
}
/*
  def multiplyFloatFloat(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (FloatV(aValue), FloatV(bValue)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, FloatV(aValue * bValue))
      }
    }
  }
*/
// mig: fn divide_float_float
pub fn divide_float_float<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Float(FloatV { value: a_value, .. }), KindV::Float(FloatV { value: b_value, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Float(FloatV { value: a_value / b_value, _phantom: PhantomData }))
        }
        _ => panic!("divide_float_float: non-FloatV args"),
    })
}
/*
  def divideFloatFloat(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (FloatV(aValue), FloatV(bValue)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, FloatV(aValue / bValue))
      }
    }
  }
*/
// mig: fn subtract_float_float
pub fn subtract_float_float<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Float(FloatV { value: a_value, .. }), KindV::Float(FloatV { value: b_value, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Float(FloatV { value: a_value - b_value, _phantom: PhantomData }))
        }
        _ => panic!("subtract_float_float: non-FloatV args"),
    })
}
/*
  def subtractFloatFloat(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (FloatV(aValue), FloatV(bValue)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, FloatV(aValue - bValue))
      }
    }
  }
*/
// mig: fn add_str_str
pub fn add_str_str<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 6);
    let a_str = match memory.dereference(args[0]) { KindV::Str(StrV { value, .. }) => value, _ => panic!("add_str_str: arg 0 not StrV") };
    let a_begin = match memory.dereference(args[1]) { KindV::Int(IntV { value, bits: 32, .. }) => value, _ => panic!("add_str_str: arg 1 not IntV(_, 32)") };
    let a_length = match memory.dereference(args[2]) { KindV::Int(IntV { value, bits: 32, .. }) => value, _ => panic!("add_str_str: arg 2 not IntV(_, 32)") };
    let b_str = match memory.dereference(args[3]) { KindV::Str(StrV { value, .. }) => value, _ => panic!("add_str_str: arg 3 not StrV") };
    let b_begin = match memory.dereference(args[4]) { KindV::Int(IntV { value, bits: 32, .. }) => value, _ => panic!("add_str_str: arg 4 not IntV(_, 32)") };
    let b_length = match memory.dereference(args[5]) { KindV::Int(IntV { value, bits: 32, .. }) => value, _ => panic!("add_str_str: arg 5 not IntV(_, 32)") };
    let a_slice = &a_str.0[a_begin as usize .. (a_begin as i32 + a_length as i32) as usize];
    let b_slice = &b_str.0[b_begin as usize .. (b_begin as i32 + b_length as i32) as usize];
    let concat = format!("{}{}", a_slice, b_slice);
    let interned = memory.scout_arena.intern_str(&concat);
    Ok(memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::YonderH, KindV::Str(StrV { value: interned, _phantom: PhantomData })))
}
/*
  def addStrStr(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 6)
    val StrV(aStr) = memory.dereference(args(0))
    val IntV(aBegin, 32) = memory.dereference(args(1))
    val IntV(aLength, 32) = memory.dereference(args(2))
    val StrV(bStr) = memory.dereference(args(3))
    val IntV(bBegin, 32) = memory.dereference(args(4))
    val IntV(bLength, 32) = memory.dereference(args(5))
    memory.addAllocationForReturn(MutableShareH, YonderH, StrV(aStr.substring(aBegin.toInt, aBegin.toInt + aLength.toInt) + bStr.substring(bBegin.toInt, bBegin.toInt + bLength.toInt)))
  }
*/
// mig: fn getch
pub fn getch<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert!(args.is_empty());
    let next = (memory.stdin)();
    let code = if next.0.is_empty() { 0i64 } else { next.0.chars().next().unwrap() as i64 };
    Ok(memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Int(IntV { value: code, bits: 32, _phantom: PhantomData })))
}
/*
  def getch(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.isEmpty)
    val next = memory.stdin()
    val code = if (next.isEmpty) { 0 } else { next.charAt(0).charValue().toInt }
    memory.addAllocationForReturn(MutableShareH, InlineH, IntV(code, 32))
  }
*/
// mig: fn less_than_float
pub fn less_than_float<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Float(FloatV { value: a_value, .. }), KindV::Float(FloatV { value: b_value, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Bool(BoolV { value: a_value < b_value, _phantom: PhantomData }))
        }
        _ => panic!("less_than_float: non-FloatV args"),
    })
}
/*
  def lessThanFloat(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (FloatV(aValue), FloatV(bValue)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(aValue < bValue))
      }
    }
  }
*/
// mig: fn greater_than_float
pub fn greater_than_float<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, { panic!("Unimplemented: greater_than_float"); }
/*
  def greaterThanFloat(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (FloatV(aValue), FloatV(bValue)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(aValue > bValue))
      }
    }
  }
*/
// mig: fn eq_float_float
pub fn eq_float_float<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Float(FloatV { value: a_value, .. }), KindV::Float(FloatV { value: b_value, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Bool(BoolV { value: a_value == b_value, _phantom: PhantomData }))
        }
        _ => panic!("eq_float_float: non-FloatV args"),
    })
}
/*
  def eqFloatFloat(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (FloatV(aValue), FloatV(bValue)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(aValue == bValue))
      }
    }
  }
*/
// mig: fn eq_str_str
pub fn eq_str_str<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 6);
    let left_str = match memory.dereference(args[0]) { KindV::Str(StrV { value, .. }) => value, _ => panic!("eq_str_str: arg 0 not StrV") };
    let left_str_start = match memory.dereference(args[1]) { KindV::Int(IntV { value, bits: 32, .. }) => value, _ => panic!("eq_str_str: arg 1 not IntV(_, 32)") };
    let left_str_len = match memory.dereference(args[2]) { KindV::Int(IntV { value, bits: 32, .. }) => value, _ => panic!("eq_str_str: arg 2 not IntV(_, 32)") };
    let right_str = match memory.dereference(args[3]) { KindV::Str(StrV { value, .. }) => value, _ => panic!("eq_str_str: arg 3 not StrV") };
    let right_str_start = match memory.dereference(args[4]) { KindV::Int(IntV { value, bits: 32, .. }) => value, _ => panic!("eq_str_str: arg 4 not IntV(_, 32)") };
    let right_str_len = match memory.dereference(args[5]) { KindV::Int(IntV { value, bits: 32, .. }) => value, _ => panic!("eq_str_str: arg 5 not IntV(_, 32)") };
    // BUG: Scala uses .slice(start, len) but Scala's slice takes (from, until), so the
    // "len" arg is being misinterpreted as an end index. Mirroring Scala parity-faithfully.
    let result_eq = &left_str.0[left_str_start as usize .. left_str_len as usize] == &right_str.0[right_str_start as usize .. right_str_len as usize];
    Ok(memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Bool(BoolV { value: result_eq, _phantom: PhantomData })))
}
/*
  def eqStrStr(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 6)
    val StrV(leftStr) = memory.dereference(args(0))
    val IntV(leftStrStart, 32) = memory.dereference(args(1))
    val IntV(leftStrLen, 32) = memory.dereference(args(2))
    val StrV(rightStr) = memory.dereference(args(3))
    val IntV(rightStrStart, 32) = memory.dereference(args(4))
    val IntV(rightStrLen, 32) = memory.dereference(args(5))
    val result = BoolV(leftStr.slice(leftStrStart.toInt, leftStrLen.toInt) == rightStr.slice(rightStrStart.toInt, rightStrLen.toInt))
    memory.addAllocationForReturn(MutableShareH, InlineH, result)
  }
*/
// mig: fn eq_bool_bool
pub fn eq_bool_bool<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Bool(BoolV { value: a_value, .. }), KindV::Bool(BoolV { value: b_value, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Bool(BoolV { value: a_value == b_value, _phantom: PhantomData }))
        }
        _ => panic!("eq_bool_bool: non-BoolV args"),
    })
}
/*
  def eqBoolBool(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (BoolV(aValue), BoolV(bValue)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(aValue == bValue))
      }
    }
  }
*/
// mig: fn and
pub fn and<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, { panic!("Unimplemented: and"); }
/*
  def and(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (BoolV(aValue), BoolV(bValue)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(aValue && bValue))
      }
    }
  }
*/
// mig: fn or
pub fn or<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, { panic!("Unimplemented: or"); }
/*
  def or(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (BoolV(aValue), BoolV(bValue)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(aValue || bValue))
      }
    }
  }
*/
// mig: fn not
pub fn not<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 1);
    let value = match memory.dereference(args[0]) {
        KindV::Bool(BoolV { value, .. }) => value,
        _ => panic!("not: non-BoolV arg"),
    };
    Ok(memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Bool(BoolV { value: !value, _phantom: PhantomData })))
}
/*
  def not(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val BoolV(value) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(!value))
  }
*/
// mig: fn sqrt
pub fn sqrt<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 1);
    let value = match memory.dereference(args[0]) {
        KindV::Float(FloatV { value, .. }) => value,
        _ => panic!("sqrt: non-FloatV arg"),
    };
    Ok(memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Float(FloatV { value: value.sqrt(), _phantom: PhantomData })))
}
/*
  def sqrt(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val FloatV(value) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, InlineH, FloatV(Math.sqrt(value).toFloat))
  }
*/
// mig: fn str_length
pub fn str_length<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 1);
    let value = match memory.dereference(args[0]) {
        KindV::Str(StrV { value, .. }) => value,
        _ => panic!("str_length: non-StrV arg"),
    };
    Ok(memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Int(IntV { value: value.0.len() as i64, bits: 32, _phantom: PhantomData })))
}
/*
  def strLength(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val StrV(value) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, InlineH, IntV(value.length, 32))
  }
*/
// mig: fn cast_float_str
pub fn cast_float_str<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 1);
    let value = match memory.dereference(args[0]) {
        KindV::Float(FloatV { value, .. }) => value,
        _ => panic!("cast_float_str: non-FloatV arg"),
    };
    let interned = memory.scout_arena.intern_str(&value.to_string());
    Ok(memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::YonderH, KindV::Str(StrV { value: interned, _phantom: PhantomData })))
}
/*
  def castFloatStr(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val FloatV(value) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, YonderH, StrV(value.toString))
  }
*/
// mig: fn negate_float
pub fn negate_float<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 1);
    let value = match memory.dereference(args[0]) {
        KindV::Float(FloatV { value, .. }) => value,
        _ => panic!("negate_float: non-FloatV arg"),
    };
    Ok(memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Float(FloatV { value: -value, _phantom: PhantomData })))
}
/*
  def negateFloat(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val FloatV(value) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, InlineH, FloatV(-value))
  }
*/
// mig: fn print
pub fn print<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 3);
    let a_str = match memory.dereference(args[0]) {
        KindV::Str(StrV { value, .. }) => value,
        _ => panic!("print: arg 0 not StrV"),
    };
    let a_begin = match memory.dereference(args[1]) {
        KindV::Int(IntV { value, bits: 32, .. }) => value,
        _ => panic!("print: arg 1 not IntV(_, 32)"),
    };
    let a_length = match memory.dereference(args[2]) {
        KindV::Int(IntV { value, bits: 32, .. }) => value,
        _ => panic!("print: arg 2 not IntV(_, 32)"),
    };
    let substring = &a_str.0[a_begin as usize .. (a_begin as i32 + a_length as i32) as usize];
    let substring_interned = memory.scout_arena.intern_str(substring);
    (memory.stdout)(substring_interned);
    Ok(memory.make_void())
}
/*
  def print(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 3)
    val StrV(aStr) = memory.dereference(args(0))
    val IntV(aBegin, 32) = memory.dereference(args(1))
    val IntV(aLength, 32) = memory.dereference(args(2))
    memory.stdout(aStr.substring(aBegin.toInt, aBegin.toInt + aLength.toInt))
    memory.makeVoid()
  }
*/
// mig: fn add_i32
pub fn add_i32<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Int(IntV { value: a_value, bits: 32, .. }), KindV::Int(IntV { value: b_value, bits: 32, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Int(IntV { value: (a_value as i32 + b_value as i32) as i64, bits: 32, _phantom: PhantomData }))
        }
        _ => panic!("add_i32: non-IntV(_, 32) args"),
    })
}
/*
  def addI32(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 32), IntV(bValue, 32)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, IntV(aValue.toInt + bValue.toInt, 32))
      }
    }
  }
*/
// mig: fn multiply_i32
pub fn multiply_i32<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Int(IntV { value: a_value, bits: 32, .. }), KindV::Int(IntV { value: b_value, bits: 32, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Int(IntV { value: (a_value as i32).wrapping_mul(b_value as i32) as i64, bits: 32, _phantom: PhantomData }))
        }
        _ => panic!("multiply_i32: non-IntV(_, 32) args"),
    })
}
/*
  def multiplyI32(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 32), IntV(bValue, 32)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, IntV(aValue.toInt * bValue.toInt, 32))
      }
    }
  }
*/
// mig: fn divide_i32
pub fn divide_i32<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Int(IntV { value: a_value, bits: 32, .. }), KindV::Int(IntV { value: b_value, bits: 32, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Int(IntV { value: (a_value as i32).wrapping_div(b_value as i32) as i64, bits: 32, _phantom: PhantomData }))
        }
        _ => panic!("divide_i32: non-IntV(_, 32) args"),
    })
}
/*
  def divideI32(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 32), IntV(bValue, 32)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, IntV(aValue.toInt / bValue.toInt, 32))
      }
    }
  }
*/
// mig: fn mod_i32
pub fn mod_i32<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Int(IntV { value: a_value, bits: 32, .. }), KindV::Int(IntV { value: b_value, bits: 32, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Int(IntV { value: ((a_value as i32) % (b_value as i32)) as i64, bits: 32, _phantom: PhantomData }))
        }
        _ => panic!("mod_i32: non-IntV(_, 32) args"),
    })
}
/*
  def modI32(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 32), IntV(bValue, 32)) => {
        try {
          memory.addAllocationForReturn(MutableShareH, InlineH, IntV(aValue.toInt % bValue.toInt, 32))
        } catch {
          case _ : ArithmeticException => vfail()
        }
      }
    }
  }
*/
// mig: fn subtract_i32
pub fn subtract_i32<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Int(IntV { value: a_value, bits: 32, .. }), KindV::Int(IntV { value: b_value, bits: 32, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Int(IntV { value: (a_value as i32).wrapping_sub(b_value as i32) as i64, bits: 32, _phantom: PhantomData }))
        }
        _ => panic!("subtract_i32: non-IntV(_, 32) args"),
    })
}
/*
  def subtractI32(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 32), IntV(bValue, 32)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, IntV(aValue.toInt - bValue.toInt, 32))
      }
    }
  }
*/
// mig: fn less_than_i32
pub fn less_than_i32<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Int(IntV { value: a_value, bits: 32, .. }), KindV::Int(IntV { value: b_value, bits: 32, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Bool(BoolV { value: a_value < b_value, _phantom: PhantomData }))
        }
        _ => panic!("less_than_i32: non-IntV(_, 32) args"),
    })
}
/*
  def lessThanI32(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 32), IntV(bValue, 32)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(aValue < bValue))
      }
    }
  }
*/
// mig: fn less_than_or_eq_i32
pub fn less_than_or_eq_i32<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Int(IntV { value: a_value, bits: 32, .. }), KindV::Int(IntV { value: b_value, bits: 32, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Bool(BoolV { value: a_value <= b_value, _phantom: PhantomData }))
        }
        _ => panic!("less_than_or_eq_i32: non-IntV(_, 32) args"),
    })
}
/*
  def lessThanOrEqI32(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 32), IntV(bValue, 32)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(aValue <= bValue))
      }
    }
  }
*/
// mig: fn greater_than_i32
pub fn greater_than_i32<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Int(IntV { value: a_value, bits: 32, .. }), KindV::Int(IntV { value: b_value, bits: 32, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Bool(BoolV { value: a_value > b_value, _phantom: PhantomData }))
        }
        _ => panic!("greater_than_i32: non-IntV(_, 32) args"),
    })
}
/*
  def greaterThanI32(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 32), IntV(bValue, 32)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(aValue > bValue))
      }
    }
  }
*/
// mig: fn greater_than_or_eq_i32
pub fn greater_than_or_eq_i32<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Int(IntV { value: a_value, bits: 32, .. }), KindV::Int(IntV { value: b_value, bits: 32, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Bool(BoolV { value: a_value >= b_value, _phantom: PhantomData }))
        }
        _ => panic!("greater_than_or_eq_i32: non-IntV(_, 32) args"),
    })
}
/*
  def greaterThanOrEqI32(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 32), IntV(bValue, 32)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(aValue >= bValue))
      }
    }
  }
*/
// mig: fn eq_i32
pub fn eq_i32<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Int(IntV { value: a_value, bits: 32, .. }), KindV::Int(IntV { value: b_value, bits: 32, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Bool(BoolV { value: a_value == b_value, _phantom: PhantomData }))
        }
        _ => panic!("eq_i32: non-IntV(_, 32) args"),
    })
}
/*
  def eqI32(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 32), IntV(bValue, 32)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(aValue == bValue))
      }
    }
  }
*/
// mig: fn cast_i32_str
pub fn cast_i32_str<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 1);
    let value = match memory.dereference(args[0]) {
        KindV::Int(IntV { value, bits: 32, .. }) => value,
        _ => panic!("cast_i32_str: non-IntV(_, 32) arg"),
    };
    let interned = memory.scout_arena.intern_str(&value.to_string());
    Ok(memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::YonderH, KindV::Str(StrV { value: interned, _phantom: PhantomData })))
}
/*
  def castI32Str(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val IntV(value, 32) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, YonderH, StrV(value.toString))
  }
*/
// mig: fn cast_float_i32
pub fn cast_float_i32<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 1);
    let value = match memory.dereference(args[0]) {
        KindV::Float(FloatV { value, .. }) => value,
        _ => panic!("cast_float_i32: non-FloatV arg"),
    };
    Ok(memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Int(IntV { value: value as i32 as i64, bits: 32, _phantom: PhantomData })))
}
/*
  def castFloatI32(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val FloatV(value) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, InlineH, IntV(value.toInt, 32))
  }
*/
// mig: fn cast_i32_float
pub fn cast_i32_float<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 1);
    let value = match memory.dereference(args[0]) {
        KindV::Int(IntV { value, bits: 32, .. }) => value,
        _ => panic!("cast_i32_float: non-IntV(_, 32) arg"),
    };
    Ok(memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Float(FloatV { value: value as f64, _phantom: PhantomData })))
}
/*
  def castI32Float(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val IntV(value, 32) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, InlineH, FloatV(value.toFloat))
  }
*/
// mig: fn add_i64
pub fn add_i64<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, { panic!("Unimplemented: add_i64"); }
/*
  def addI64(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 64), IntV(bValue, 64)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, IntV(aValue + bValue, 64))
      }
    }
  }
*/
// mig: fn multiply_i64
pub fn multiply_i64<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Int(IntV { value: a_value, bits: 64, .. }), KindV::Int(IntV { value: b_value, bits: 64, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Int(IntV { value: a_value.wrapping_mul(b_value), bits: 64, _phantom: PhantomData }))
        }
        _ => panic!("multiply_i64: non-IntV(_, 64) args"),
    })
}
/*
  def multiplyI64(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 64), IntV(bValue, 64)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, IntV(aValue * bValue, 64))
      }
    }
  }
*/
// mig: fn divide_i64
pub fn divide_i64<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Int(IntV { value: a_value, bits: 64, .. }), KindV::Int(IntV { value: b_value, bits: 64, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Int(IntV { value: a_value.wrapping_div(b_value), bits: 64, _phantom: PhantomData }))
        }
        _ => panic!("divide_i64: non-IntV(_, 64) args"),
    })
}
/*
  def divideI64(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 64), IntV(bValue, 64)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, IntV(aValue / bValue, 64))
      }
    }
  }
*/
// mig: fn truncate_i64_to_i32
pub fn truncate_i64_to_i32<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 1);
    let value = match memory.dereference(args[0]) {
        KindV::Int(IntV { value, bits: 64, .. }) => value,
        _ => panic!("truncate_i64_to_i32: non-IntV(_, 64) arg"),
    };
    let result = value & 0xFFFFFFFFi64;
    Ok(memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Int(IntV { value: result, bits: 32, _phantom: PhantomData })))
}
/*
  def truncateI64ToI32(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val IntV(value, 64) = memory.dereference(args(0))
    val result = value & 0xFFFFFFFFL
    memory.addAllocationForReturn(MutableShareH, InlineH, IntV(result, 32))
  }
*/
// mig: fn mod_i64
pub fn mod_i64<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, { panic!("Unimplemented: mod_i64"); }
/*
  def modI64(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 64), IntV(bValue, 64)) => {
        try {
          memory.addAllocationForReturn(MutableShareH, InlineH, IntV(aValue % bValue, 64))
        } catch {
          case _ : ArithmeticException => vfail()
        }
      }
    }
  }
*/
// mig: fn subtract_i64
pub fn subtract_i64<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 2);
    let a_kind = memory.dereference(args[0]);
    let b_kind = memory.dereference(args[1]);
    Ok(match (a_kind, b_kind) {
        (KindV::Int(IntV { value: a_value, bits: 64, .. }), KindV::Int(IntV { value: b_value, bits: 64, .. })) => {
            memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Int(IntV { value: a_value.wrapping_sub(b_value), bits: 64, _phantom: PhantomData }))
        }
        _ => panic!("subtract_i64: non-IntV(_, 64) args"),
    })
}
/*
  def subtractI64(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 64), IntV(bValue, 64)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, IntV(aValue - bValue, 64))
      }
    }
  }
*/
// mig: fn less_than_i64
pub fn less_than_i64<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, { panic!("Unimplemented: less_than_i64"); }
/*
  def lessThanI64(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 64), IntV(bValue, 64)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(aValue < bValue))
      }
    }
  }
*/
// mig: fn less_than_or_eq_i64
pub fn less_than_or_eq_i64<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, { panic!("Unimplemented: less_than_or_eq_i64"); }
/*
  def lessThanOrEqI64(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 64), IntV(bValue, 64)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(aValue <= bValue))
      }
    }
  }
*/
// mig: fn greater_than_i64
pub fn greater_than_i64<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, { panic!("Unimplemented: greater_than_i64"); }
/*
  def greaterThanI64(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 64), IntV(bValue, 64)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(aValue > bValue))
      }
    }
  }
*/
// mig: fn greater_than_or_eq_i64
pub fn greater_than_or_eq_i64<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, { panic!("Unimplemented: greater_than_or_eq_i64"); }
/*
  def greaterThanOrEqI64(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 64), IntV(bValue, 64)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(aValue >= bValue))
      }
    }
  }
*/
// mig: fn eq_i64
pub fn eq_i64<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, { panic!("Unimplemented: eq_i64"); }
/*
  def eqI64(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 2)
    val aKind = memory.dereference(args(0))
    val bKind = memory.dereference(args(1))
    (aKind, bKind) match {
      case (IntV(aValue, 64), IntV(bValue, 64)) => {
        memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(aValue == bValue))
      }
    }
  }
*/
// mig: fn cast_i64_str
pub fn cast_i64_str<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert_eq!(args.len(), 1);
    let value = match memory.dereference(args[0]) {
        KindV::Int(IntV { value, bits: 64, .. }) => value,
        _ => panic!("cast_i64_str: non-IntV(_, 64) arg"),
    };
    let interned = memory.scout_arena.intern_str(&value.to_string());
    Ok(memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::YonderH, KindV::Str(StrV { value: interned, _phantom: PhantomData })))
}
/*
  def castI64Str(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val IntV(value, 64) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, YonderH, StrV(value.toString))
  }
*/
// mig: fn cast_float_i64
pub fn cast_float_i64<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, { panic!("Unimplemented: cast_float_i64"); }
/*
  def castFloatI64(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val FloatV(value) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, InlineH, IntV(value.toInt, 64))
  }
*/
// mig: fn cast_i64_float
pub fn cast_i64_float<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, { panic!("Unimplemented: cast_i64_float"); }
/*
  def castI64Float(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val IntV(value, 64) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, InlineH, FloatV(value.toFloat))
  }
*/
// mig: fn new_vec
pub fn new_vec<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, prototype: &PrototypeH<'s, 'h>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert!(args.len() == 0);
    let opaque_coord = match prototype.return_type {
        CoordH { ownership: own, location: loc, kind: KindHT::OpaqueHT(s) } => CoordH { ownership: own, location: loc, kind: KindHT::OpaqueHT(s) },
        _ => panic!(),
    };
    Ok(memory.new_opaque(opaque_coord))
}
/*
  def newVec(memory: AdapterForExterns, prototype: PrototypeH, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 0)
    val opaqueCoord =
      prototype.returnType match {
        case CoordH(own, loc, s @ OpaqueHT(_, _, _)) => CoordH(own, loc, s)
      }
    memory.newOpaque(opaqueCoord)
  }
*/
// mig: fn new_vec_with_capacity
pub fn new_vec_with_capacity<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, prototype: &PrototypeH<'s, 'h>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert!(args.len() == 1);
    // This whole function only exists for testing purposes
    match memory.dereference(args[0]) {
        KindV::Int(IntV { value: 42, bits: 64, .. }) => {}
        _ => panic!(),
    }
    let opaque_coord = match prototype.return_type {
        CoordH { ownership: own, location: loc, kind: KindHT::OpaqueHT(s) } => CoordH { ownership: own, location: loc, kind: KindHT::OpaqueHT(s) },
        _ => panic!(),
    };
    Ok(memory.new_opaque(opaque_coord))
}
/*
  def newVecWithCapacity(memory: AdapterForExterns, prototype: PrototypeH, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    // This whole function only exists for testing purposes
    memory.dereference(args(0)) match {
      case IntV(42, 64) =>
    }

    val opaqueCoord =
      prototype.returnType match {
        case CoordH(own, loc, s@OpaqueHT(_, _, _)) => CoordH(own, loc, s)
      }
    memory.newOpaque(opaqueCoord)
  }
*/
// mig: fn vec_capacity
pub fn vec_capacity<'v, 'h, 's>(memory: &mut AdapterForExternsV<'_, 'v, 'h, 's>, _prototype: &PrototypeH<'s, 'h>, args: &'v [ReferenceV<'v, 'h, 's>]) -> Result<ReferenceV<'v, 'h, 's>, VmRuntimeErrorV<'s>> where 's: 'h, 'h: 'v, {
    assert!(args.len() == 1);
    match memory.dereference(args[0]) {
        KindV::Opaque(_) => {}
        _ => panic!(),
    }
    // This whole function just exists for testing, there are some tests that feed 42 in to newVecWithCapacity
    Ok(memory.add_allocation_for_return(OwnershipH::MutableShareH, LocationH::InlineH, KindV::Int(IntV { value: 42, bits: 64, _phantom: PhantomData })))
}
/*
  def vecCapacity(memory: AdapterForExterns, prototype: PrototypeH, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    memory.dereference(args(0)) match {
      case OpaqueV(_) =>
    }

    // This whole function just exists for testing, there are some tests that feed 42 in to newVecWithCapacity
    memory.addAllocationForReturn(MutableShareH, InlineH, IntV(42, 64))
  }
}

*/
