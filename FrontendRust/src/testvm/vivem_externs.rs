use crate::testvm::values::ReferenceV;
use crate::testvm::heap::AdapterForExternsV;
use crate::final_ast::ast::PrototypeH;

/*
package dev.vale.testvm

import dev.vale.finalast._
import dev.vale.{vassert, vfail}

import java.lang.ArithmeticException
import dev.vale.vfail

object VivemExterns {
*/
// mig: fn panic
pub fn panic<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: panic"); }
/*
  def panic(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 0)
    throw new PanicException()
  }
*/
// mig: fn add_float_float
pub fn add_float_float<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: add_float_float"); }
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
pub fn multiply_float_float<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: multiply_float_float"); }
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
pub fn divide_float_float<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: divide_float_float"); }
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
pub fn subtract_float_float<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: subtract_float_float"); }
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
pub fn add_str_str<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: add_str_str"); }
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
pub fn getch<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: getch"); }
/*
  def getch(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.isEmpty)
    val next = memory.stdin()
    val code = if (next.isEmpty) { 0 } else { next.charAt(0).charValue().toInt }
    memory.addAllocationForReturn(MutableShareH, InlineH, IntV(code, 32))
  }
*/
// mig: fn less_than_float
pub fn less_than_float<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: less_than_float"); }
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
pub fn greater_than_float<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: greater_than_float"); }
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
pub fn eq_float_float<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: eq_float_float"); }
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
pub fn eq_str_str<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: eq_str_str"); }
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
pub fn eq_bool_bool<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: eq_bool_bool"); }
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
pub fn and<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: and"); }
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
pub fn or<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: or"); }
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
pub fn not<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: not"); }
/*
  def not(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val BoolV(value) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, InlineH, BoolV(!value))
  }
*/
// mig: fn sqrt
pub fn sqrt<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: sqrt"); }
/*
  def sqrt(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val FloatV(value) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, InlineH, FloatV(Math.sqrt(value).toFloat))
  }
*/
// mig: fn str_length
pub fn str_length<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: str_length"); }
/*
  def strLength(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val StrV(value) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, InlineH, IntV(value.length, 32))
  }
*/
// mig: fn cast_float_str
pub fn cast_float_str<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: cast_float_str"); }
/*
  def castFloatStr(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val FloatV(value) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, YonderH, StrV(value.toString))
  }
*/
// mig: fn negate_float
pub fn negate_float<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: negate_float"); }
/*
  def negateFloat(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val FloatV(value) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, InlineH, FloatV(-value))
  }
*/
// mig: fn print
pub fn print<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: print"); }
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
pub fn add_i32<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: add_i32"); }
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
pub fn multiply_i32<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: multiply_i32"); }
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
pub fn divide_i32<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: divide_i32"); }
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
pub fn mod_i32<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: mod_i32"); }
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
pub fn subtract_i32<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: subtract_i32"); }
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
pub fn less_than_i32<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: less_than_i32"); }
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
pub fn less_than_or_eq_i32<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: less_than_or_eq_i32"); }
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
pub fn greater_than_i32<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: greater_than_i32"); }
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
pub fn greater_than_or_eq_i32<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: greater_than_or_eq_i32"); }
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
pub fn eq_i32<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: eq_i32"); }
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
pub fn cast_i32_str<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: cast_i32_str"); }
/*
  def castI32Str(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val IntV(value, 32) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, YonderH, StrV(value.toString))
  }
*/
// mig: fn cast_float_i32
pub fn cast_float_i32<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: cast_float_i32"); }
/*
  def castFloatI32(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val FloatV(value) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, InlineH, IntV(value.toInt, 32))
  }
*/
// mig: fn cast_i32_float
pub fn cast_i32_float<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: cast_i32_float"); }
/*
  def castI32Float(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val IntV(value, 32) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, InlineH, FloatV(value.toFloat))
  }
*/
// mig: fn add_i64
pub fn add_i64<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: add_i64"); }
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
pub fn multiply_i64<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: multiply_i64"); }
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
pub fn divide_i64<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: divide_i64"); }
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
pub fn truncate_i64_to_i32<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: truncate_i64_to_i32"); }
/*
  def truncateI64ToI32(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val IntV(value, 64) = memory.dereference(args(0))
    val result = value & 0xFFFFFFFFL
    memory.addAllocationForReturn(MutableShareH, InlineH, IntV(result, 32))
  }
*/
// mig: fn mod_i64
pub fn mod_i64<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: mod_i64"); }
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
pub fn subtract_i64<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: subtract_i64"); }
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
pub fn less_than_i64<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: less_than_i64"); }
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
pub fn less_than_or_eq_i64<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: less_than_or_eq_i64"); }
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
pub fn greater_than_i64<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: greater_than_i64"); }
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
pub fn greater_than_or_eq_i64<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: greater_than_or_eq_i64"); }
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
pub fn eq_i64<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: eq_i64"); }
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
pub fn cast_i64_str<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: cast_i64_str"); }
/*
  def castI64Str(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val IntV(value, 64) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, YonderH, StrV(value.toString))
  }
*/
// mig: fn cast_float_i64
pub fn cast_float_i64<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: cast_float_i64"); }
/*
  def castFloatI64(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val FloatV(value) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, InlineH, IntV(value.toInt, 64))
  }
*/
// mig: fn cast_i64_float
pub fn cast_i64_float<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: cast_i64_float"); }
/*
  def castI64Float(memory: AdapterForExterns, args: Vector[ReferenceV]): ReferenceV = {
    vassert(args.size == 1)
    val IntV(value, 64) = memory.dereference(args(0))
    memory.addAllocationForReturn(MutableShareH, InlineH, FloatV(value.toFloat))
  }
*/
// mig: fn new_vec
pub fn new_vec<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, prototype: &PrototypeH<'s, 'h>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: new_vec"); }
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
pub fn new_vec_with_capacity<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, prototype: &PrototypeH<'s, 'h>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: new_vec_with_capacity"); }
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
pub fn vec_capacity<'v, 'h, 's>(memory: &AdapterForExternsV<'v, 'h, 's>, prototype: &PrototypeH<'s, 'h>, args: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> where 's: 'h, 'h: 'v, { panic!("Unimplemented: vec_capacity"); }
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
