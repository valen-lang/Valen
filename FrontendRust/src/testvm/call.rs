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
  pub args: HashMap<i32, Option<ReferenceV<'v, 'h, 's>>>,
  pub locals: HashMap<VariableAddressV<'v, 'h, 's>, VariableV<'v, 'h, 's>>,
}
/*
class Call(callId: CallId, in_args: Vector[ReferenceV]) {
  private val args = mutable.HashMap[Int, Option[ReferenceV]]() ++ in_args.indices.zip(in_args.map(arg => Some(arg))).toMap

  private val locals = mutable.HashMap[VariableAddressV, VariableV]()

*/
// mig: fn add_local
impl<'v, 'h, 's> CallV<'v, 'h, 's> {
  pub fn add_local(&mut self, var_addr: VariableAddressV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>, tyype: CoordH<'s, 'h>) {
    assert_eq!(var_addr.call_id, self.call_id);
    let locals = &mut self.locals;
    assert!(!locals.contains_key(&var_addr));
    assert!(!locals.iter().any(|(addr, _)| addr.local.id.number == var_addr.local.id.number));
    locals.insert(var_addr, crate::testvm::values::VariableV {
      id: var_addr,
      reference: std::cell::Cell::new(reference),
      expected_type: tyype,
    });
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
  pub fn remove_local(&mut self, var_addr: VariableAddressV<'v, 'h, 's>) {
    assert_eq!(var_addr.call_id, self.call_id);
    let locals = &mut self.locals;
    assert!(locals.contains_key(&var_addr));
    locals.remove(&var_addr);
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
    let locals = &self.locals;
    let result = locals.get(&addr).expect("get_local: not found").clone();
    result
  }
}
/*
  def getLocal(addr: VariableAddressV) = {
    vassertSome(locals.get(addr))
  }

*/
// mig: fn mutate_local
impl<'v, 'h, 's> CallV<'v, 'h, 's> {
  pub fn mutate_local(&mut self, var_addr: VariableAddressV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>) {
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
  pub fn take_argument(&mut self, index: i32) -> ReferenceV<'v, 'h, 's> {
    assert!((index as usize) < self.args.len());
    match self.args.get(&index).copied() {
      Some(Some(r#ref)) => {
        self.args.insert(index, None);
        r#ref
      }
      Some(None) => panic!("Already took from argument {}", index),
      None => panic!("take_argument: missing argument key {} (assert should have caught this)", index),
    }
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
  pub fn prepare_to_die(&mut self) {
    let locals = &self.locals;
    assert!(locals.is_empty());
    let args = &self.args;
    let undead_args: Vec<_> = args.iter().filter_map(|(i, v)| v.map(|val| (*i, val))).collect();
    if !undead_args.is_empty() {
        panic!("Undead arguments:\n{:?}", undead_args);
    }
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
