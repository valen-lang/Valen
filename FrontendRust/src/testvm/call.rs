use std::cell::Cell;
use std::collections::HashMap;
use std::marker::PhantomData;
use crate::final_ast::types::{KindHT, CoordH};
use crate::testvm::values::{
    AllocationIdV, CallIdV, ExpressionIdV, IObjectReferrerV,
    ReferenceV, RegisterV, VariableAddressV, VariableV,
};

/*
package dev.vale.testvm

import dev.vale.finalast.{KindHT, CoordH}
import dev.vale.{vassert, vassertSome, vfail}
import dev.vale.finalast.{KindHT, CoordH}
import dev.vale.vfail

import scala.collection.mutable

*/
// mig: struct CallV<'v, 'h, 's>
/// Temporary state
pub struct CallV<'v, 'h, 's> {
  pub call_id: CallIdV<'v, 'h, 's>,
  pub in_args: &'v [ReferenceV<'v, 'h, 's>],
  pub args: Cell<HashMap<i32, Option<ReferenceV<'v, 'h, 's>>>>,
  pub locals: Cell<HashMap<VariableAddressV<'v, 'h, 's>, VariableV<'v, 'h, 's>>>,
}
/*
class Call(callId: CallId, in_args: Vector[ReferenceV]) {
  private val args = mutable.HashMap[Int, Option[ReferenceV]]() ++ in_args.indices.zip(in_args.map(arg => Some(arg))).toMap

  private val locals = mutable.HashMap[VariableAddressV, VariableV]()

*/
// mig: fn add_local
impl<'v, 'h, 's> CallV<'v, 'h, 's> {
  pub fn add_local(&self, var_addr: VariableAddressV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>, tyype: CoordH<'s, 'h>) {
    panic!("Unimplemented: add_local");
  }
}
/*
  def addLocal(varAddr: VariableAddressV, reference: ReferenceV, tyype: CoordH[KindHT]): Unit = {
    vassert(varAddr.callId == callId)
    vassert(!locals.contains(varAddr))
    vassert(!locals.exists(_._1.local.id.number == varAddr.local.id.number))
    locals.put(varAddr, VariableV(varAddr, reference, tyype))
  }

*/
// mig: fn remove_local
impl<'v, 'h, 's> CallV<'v, 'h, 's> {
  pub fn remove_local(&self, var_addr: VariableAddressV<'v, 'h, 's>) {
    panic!("Unimplemented: remove_local");
  }
}
/*
  def removeLocal(varAddr: VariableAddressV): Unit = {
    vassert(varAddr.callId == callId)
    vassert(locals.contains(varAddr))
    locals.remove(varAddr)
  }

*/
// mig: fn get_local
impl<'v, 'h, 's> CallV<'v, 'h, 's> {
  pub fn get_local(&self, addr: VariableAddressV<'v, 'h, 's>) -> VariableV<'v, 'h, 's> {
    panic!("Unimplemented: get_local");
  }
}
/*
  def getLocal(addr: VariableAddressV) = {
    vassertSome(locals.get(addr))
  }

*/
// mig: fn mutate_local
impl<'v, 'h, 's> CallV<'v, 'h, 's> {
  pub fn mutate_local(&self, var_addr: VariableAddressV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>) {
    panic!("Unimplemented: mutate_local");
  }
}
/*
  def mutateLocal(varAddr: VariableAddressV, reference: ReferenceV, expectedType: CoordH[KindHT]): Unit = {
    locals(varAddr).reference = reference
  }

*/
// mig: fn take_argument
impl<'v, 'h, 's> CallV<'v, 'h, 's> {
  pub fn take_argument(&self, index: i32) -> ReferenceV<'v, 'h, 's> {
    panic!("Unimplemented: take_argument");
  }
}
/*
  def takeArgument(index: Int): ReferenceV = {
    vassert(index < args.size)
    args(index) match {
      case Some(ref) => {
        args.put(index, None)
        ref
      }
      case None => {
        vfail("Already took from argument " + index)
      }
    }
  }

*/
// mig: fn prepare_to_die
impl<'v, 'h, 's> CallV<'v, 'h, 's> {
  pub fn prepare_to_die(&self) {
    panic!("Unimplemented: prepare_to_die");
  }
}
/*
  def prepareToDie() = {
    vassert(locals.isEmpty)

//    // Make sure all locals were unletted
//    locals.foreach({ case (varAddr, variable) =>
//      // We trip this when we don't Unstackify something so its still alive on
//      // the stack.
//      vassert(variable.reference == None)
//      locals.remove(varAddr)
//    })
//    while (localAddrStack.nonEmpty) {
//      localAddrStack.pop()
//    }
//
//    vassert(localAddrStack.size == locals.size)

    val undeadArgs =
      args.collect({
        case (index, Some(value)) => (index, value)
      })
    if (undeadArgs.nonEmpty) {
      vfail("Undead arguments:\n" + undeadArgs.mkString("\n"))
    }

//    val undeadRegisters =
//      registersById.collect({
//        case (registerId, Some(register)) => (registerId, register)
//      })
//    if (undeadRegisters.nonEmpty) {
//      vfail("Undead registers:\n" + undeadRegisters.mkString("\n"))
//    }
  }
}

*/
