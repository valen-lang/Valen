use std::cell::Cell;
use std::collections::HashMap;
use std::marker::PhantomData;
use crate::interner::StrI;
use crate::final_ast::types::{KindHT, CoordH, LocationH, OwnershipH, StructHT, StaticSizedArrayHT, RuntimeSizedArrayHT, StaticSizedArrayDefinitionHT, RuntimeSizedArrayDefinitionHT};
use crate::final_ast::ast::{ProgramH, PrototypeH, StructDefinitionH};
use crate::final_ast::instructions::Local;
use crate::testvm::values::{
    AllocationIdV, AllocationV, ArrayInstanceV, CallIdV, ElementAddressV, ExpressionIdV,
    IObjectReferrerV, KindV, MemberAddressV, PrimitiveKindV, ReferenceRegisterV, ReferenceV, RegisterV,
    StructInstanceV, VariableAddressV, VariableV,
};
use crate::testvm::call::CallV;
use crate::final_ast::types::OpaqueHT;
use crate::von::ast::IVonData;

/*
package dev.vale.testvm

import dev.vale.finalast._
import dev.vale.{vassert, vassertSome, vfail, vimpl, von}

import java.io.PrintStream
import dev.vale.finalast._
import dev.vale.von.{IVonData, VonArray, VonBool, VonFloat, VonInt, VonMember, VonObject, VonStr}

import scala.collection.mutable
*/
// mig: struct AdapterForExternsV<'v, 'h, 's>
/// Temporary state
pub struct AdapterForExternsV<'v, 'h, 's> {
    pub program_h: &'h ProgramH<'s, 'h>,
    pub heap: HeapV<'v, 'h, 's>,
    pub call_id: CallIdV<'v, 'h, 's>,
    pub stdin: &'v dyn Fn() -> StrI<'s>,
    pub stdout: &'v dyn Fn(StrI<'s>),
}
/*
class AdapterForExterns(
    val programH: ProgramH,
    private val heap: Heap,
    callId: CallId,
    val stdin: (() => String),
    val stdout: (String => Unit)
) {
*/
// mig: fn dereference
impl<'v, 'h, 's> AdapterForExternsV<'v, 'h, 's> {
    pub fn dereference(&self, reference: ReferenceV<'v, 'h, 's>) -> KindV<'v, 'h, 's> {
        panic!("Unimplemented: dereference");
    }
}
/*
  def dereference(reference: ReferenceV) = {
    heap.dereference(reference)
  }
*/
// mig: fn new_opaque
impl<'v, 'h, 's> AdapterForExternsV<'v, 'h, 's> {
    pub fn new_opaque(&self, opaque_ht: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: new_opaque");
    }
}
/*
  def newOpaque(opaqueHT: CoordH[OpaqueHT]):
  ReferenceV = {
    heap.newOpaque(opaqueHT)
  }
*/
// mig: fn add_allocation_for_return
impl<'v, 'h, 's> AdapterForExternsV<'v, 'h, 's> {
    pub fn add_allocation_for_return(&self, ownership: OwnershipH, location: LocationH, kind: KindV<'v, 'h, 's>) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: add_allocation_for_return");
    }
}
/*
  def addAllocationForReturn(ownership: OwnershipH, location: LocationH, kind: KindV): ReferenceV = {
    val ref = heap.add(ownership, location, kind)
//    heap.incrementReferenceRefCount(ResultToObjectReferrer(callId), ref) // incrementing because putting it in a return
//    ReturnV(callId, ref)
    ref
  }
*/
// mig: fn make_void
impl<'v, 'h, 's> AdapterForExternsV<'v, 'h, 's> {
    pub fn make_void(&self) -> ReferenceV {
        panic!("Unimplemented: make_void");
    }
}
/*
  def makeVoid(): ReferenceV = {
    heap.void
  }
}
*/
// mig: struct AllocationMapV<'v, 'h, 's>
/// Temporary state
pub struct AllocationMapV<'v, 'h, 's> {
    pub objects_by_id: Cell<HashMap<AllocationIdV<'v, 'h, 's>, AllocationV<'v, 'h, 's>>>,
    pub next_id: Cell<i32>,
    pub void_ref: ReferenceV<'v, 'h, 's>,
}
/*
class AllocationMap(vivemDout: PrintStream) {
  private val objectsById = mutable.HashMap[AllocationId, Allocation]()
  private val STARTING_ID = 501
  private var nextId = STARTING_ID;
  val void: ReferenceV = add(MutableShareH, InlineH, VoidV)
*/
// mig: fn new_id
impl<'v, 'h, 's> AllocationMapV<'v, 'h, 's> {
    pub fn new_id(&self) -> AllocationIdV<'v, 'h, 's> {
        panic!("Unimplemented: new_id");
    }
}
/*
  private def newId() = {
    val id = nextId;
    nextId = nextId + 1
    id
  }
*/
// mig: fn is_empty
impl<'v, 'h, 's> AllocationMapV<'v, 'h, 's> {
    pub fn is_empty(&self) -> bool {
        panic!("Unimplemented: is_empty");
    }
}
/*
  def isEmpty: Boolean = {
    objectsById.isEmpty
  }
*/
// mig: fn size
impl<'v, 'h, 's> AllocationMapV<'v, 'h, 's> {
    pub fn size(&self) -> usize {
        panic!("Unimplemented: size");
    }
}
/*
  def size = {
    objectsById.size
  }
*/
// mig: fn get
impl<'v, 'h, 's> AllocationMapV<'v, 'h, 's> {
    pub fn get(&self, alloc_id: AllocationIdV<'v, 'h, 's>) -> AllocationV<'v, 'h, 's> {
        panic!("Unimplemented: get");
    }
}
/*
  def get(allocId: AllocationId) = {
    val allocation = vassertSome(objectsById.get(allocId))
    vassert(allocation.kind.tyype == allocId.tyype)
    allocation
  }
*/
// mig: fn remove
impl<'v, 'h, 's> AllocationMapV<'v, 'h, 's> {
    pub fn remove(&self, alloc_id: AllocationIdV<'v, 'h, 's>) {
        panic!("Unimplemented: remove");
    }
}
/*
  def remove(allocId: AllocationId): Unit = {
    vassert(contains(allocId))
    vassert(objectsById(allocId).kind != VoidV)
    objectsById.remove(allocId)
  }
*/
// mig: fn contains
impl<'v, 'h, 's> AllocationMapV<'v, 'h, 's> {
    pub fn contains(&self, alloc_id: AllocationIdV<'v, 'h, 's>) -> bool {
        panic!("Unimplemented: contains");
    }
}
/*
  def contains(allocId: AllocationId): Boolean = {
    objectsById.get(allocId) match {
      case None => false
      case Some(allocation) => {
        vassert(allocation.kind.tyype.hamut == allocId.tyype.hamut)
        true
      }
    }
  }
*/
// mig: fn add
impl<'v, 'h, 's> AllocationMapV<'v, 'h, 's> {
    pub fn add(&self, ownership: OwnershipH, location: LocationH, kind: KindV<'v, 'h, 's>) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: add");
    }
}
/*
  def add(ownership: OwnershipH, location: LocationH, kind: KindV) = {
    val id = newId()
    if (kind == VoidV) {
      // Make sure it only happens once
      vassert(id == STARTING_ID)
    }
    val reference =
      ReferenceV(
        // These two are the same because when we allocate something,
        // we see it for what it truly is.
        //                                          ~ Wisdom ~
        actualKind = kind.tyype,
        seenAsKind = kind.tyype,
        ownership,
        location,
        id)
    val allocation = new Allocation(reference, kind)
    objectsById.put(reference.allocId, allocation)
    reference
  }
*/
// mig: fn print_all
impl<'v, 'h, 's> AllocationMapV<'v, 'h, 's> {
    pub fn print_all(&self) {
        panic!("Unimplemented: print_all");
    }
}
/*
  def printAll(): Unit = {
    objectsById.foreach({
      case (id, allocation) => vivemDout.println(id + " (" + allocation.getTotalRefCount(None) + " refs) = " + allocation.kind)
    })
  }
*/
// mig: fn check_for_leaks
impl<'v, 'h, 's> AllocationMapV<'v, 'h, 's> {
    pub fn check_for_leaks(&self) {
        panic!("Unimplemented: check_for_leaks");
    }
}
/*
  def checkForLeaks(): Unit = {
    val nonInternedObjects = objectsById.values.filter(_.kind != VoidV)
    if (nonInternedObjects.nonEmpty) {
      nonInternedObjects
        .map(_.reference.allocId.num)
        .toVector
        .sorted
        .foreach(objId => print("o" + objId + " "))
      println()
      nonInternedObjects.toVector.sortWith(_.reference.allocId.num < _.reference.allocId.num).foreach(_.printRefs())
      vfail("Memory leaks! See above for ")
    }
  }
}
*/
// mig: struct HeapV<'v, 'h, 's>
/// Temporary state
pub struct HeapV<'v, 'h, 's> {
    pub objects_by_id: AllocationMapV<'v, 'h, 's>,
    pub call_id_stack: Cell<Vec<CallIdV<'v, 'h, 's>>>,
    pub calls_by_id: Cell<HashMap<CallIdV<'v, 'h, 's>, CallV<'v, 'h, 's>>>,
}
/*
// Just keeps track of all active objects
class Heap(in_vivemDout: PrintStream) {
  val vivemDout = in_vivemDout

  // private
  val objectsById = new AllocationMap(vivemDout)

  private val callIdStack = mutable.Stack[CallId]();
  private val callsById = mutable.HashMap[CallId, Call]()

  val void = objectsById.void
*/
// mig: fn add_local
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn add_local(&self, var_addr: VariableAddressV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>) {
        panic!("Unimplemented: add_local");
    }
}
/*
  def addLocal(varAddr: VariableAddressV, reference: ReferenceV, expectedType: CoordH[KindHT]) = {
    val call = getCurrentCall(varAddr.callId)
    checkReference(expectedType, reference)
    call.addLocal(varAddr, reference, expectedType)
    incrementReferenceRefCount(VariableToObjectReferrer(varAddr, expectedType.ownership), reference)
  }
*/
// mig: fn get_reference
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn get_reference(&self, var_addr: VariableAddressV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: get_reference");
    }
}
/*
  def getReference(varAddr: VariableAddressV, expectedType: CoordH[KindHT]) = {
    callsById(varAddr.callId).getLocal(varAddr).reference
  }
*/
// mig: fn remove_local
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn remove_local(&self, var_addr: VariableAddressV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>) {
        panic!("Unimplemented: remove_local");
    }
}
/*
  def removeLocal(varAddr: VariableAddressV, expectedType: CoordH[KindHT]) = {
    val call = getCurrentCall(varAddr.callId)
    val variable = getLocal(varAddr)
    val actualReference = variable.reference
    checkReference(expectedType, actualReference)
    decrementReferenceRefCount(VariableToObjectReferrer(varAddr, expectedType.ownership), actualReference)
    call.removeLocal(varAddr)
  }
*/
// mig: fn get_reference_from_local
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn get_reference_from_local(&self, var_addr: VariableAddressV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>, target_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: get_reference_from_local");
    }
}
/*
  def getReferenceFromLocal(
      varAddr: VariableAddressV,
      expectedType: CoordH[KindHT],
      targetType: CoordH[KindHT]
  ): ReferenceV = {
    val variable = getLocal(varAddr)
    if (variable.expectedType != expectedType) {
      vfail("blort")
    }
    checkReference(expectedType, variable.reference)
    transmute(variable.reference, expectedType, targetType)
  }
*/
// mig: fn get_local
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn get_local(&self, var_addr: VariableAddressV<'v, 'h, 's>) -> VariableV<'v, 'h, 's> {
        panic!("Unimplemented: get_local");
    }
}
/*
  private def getLocal(varAddr: VariableAddressV): VariableV = {
    callsById(varAddr.callId).getLocal(varAddr)
  }
*/
// mig: fn mutate_variable
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn mutate_variable(&self, var_address: VariableAddressV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: mutate_variable");
    }
}
/*
  def mutateVariable(varAddress: VariableAddressV, reference: ReferenceV, expectedType: CoordH[KindHT]): ReferenceV = {
    val variable = callsById(varAddress.callId).getLocal(varAddress)
    checkReference(expectedType, reference)
    checkReference(variable.expectedType, reference)
    val oldReference = variable.reference
    decrementReferenceRefCount(VariableToObjectReferrer(varAddress, expectedType.ownership), oldReference)

    incrementReferenceRefCount(VariableToObjectReferrer(varAddress, expectedType.ownership), reference)
    callsById(varAddress.callId).mutateLocal(varAddress, reference, expectedType)
    oldReference
  }
*/
// mig: fn mutate_array
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn mutate_array(&self, element_address: ElementAddressV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: mutate_array");
    }
}
/*
  def mutateArray(elementAddress: ElementAddressV, reference: ReferenceV, expectedType: CoordH[KindHT]): ReferenceV = {
    val ElementAddressV(arrayRef, elementIndex) = elementAddress
    objectsById.get(arrayRef).kind match {
      case ai @ ArrayInstanceV(_, _, _, _) => {
        val oldReference = ai.getElement(elementIndex)
        decrementReferenceRefCount(ElementToObjectReferrer(elementAddress, expectedType.ownership), oldReference)

        ai.setElement(elementIndex, reference)
        incrementReferenceRefCount(ElementToObjectReferrer(elementAddress, expectedType.ownership), reference)
        oldReference
      }
    }
  }
*/
// mig: fn mutate_struct
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn mutate_struct(&self, member_address: MemberAddressV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: mutate_struct");
    }
}
/*
  def mutateStruct(memberAddress: MemberAddressV, reference: ReferenceV, expectedType: CoordH[KindHT]):
  ReferenceV = {
    val MemberAddressV(objectId, fieldIndex) = memberAddress
    objectsById.get(objectId).kind match {
      case si @ StructInstanceV(structDefH, Some(members)) => {
        val oldMemberReference = members(fieldIndex)
        decrementReferenceRefCount(MemberToObjectReferrer(memberAddress, expectedType.ownership), oldMemberReference)
//        maybeDeallocate(actualReference)
        vassert(structDefH.members(fieldIndex).tyype == expectedType)
        // We only do this to check that it's non-empty. curiosity assert, do we gotta do somethin special if somethin was moved out
        si.getReferenceMember(fieldIndex)
        si.setReferenceMember(fieldIndex, reference)
        incrementReferenceRefCount(MemberToObjectReferrer(memberAddress, expectedType.ownership), reference)
        oldMemberReference
      }
    }
  }
*/
// mig: fn get_reference_from_struct
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn get_reference_from_struct(&self, address: MemberAddressV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>, target_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: get_reference_from_struct");
    }
}
/*
  def getReferenceFromStruct(
    address: MemberAddressV,
    expectedType: CoordH[KindHT],
    targetType: CoordH[KindHT]
  ): ReferenceV = {
    val MemberAddressV(objectId, fieldIndex) = address
    objectsById.get(objectId).kind match {
      case StructInstanceV(_, Some(members)) => {
        val actualReference = members(fieldIndex)
        checkReference(expectedType, actualReference)
        transmute(actualReference, expectedType, targetType)
      }
    }
  }
*/
// mig: fn get_reference_from_array
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn get_reference_from_array(&self, address: ElementAddressV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>, target_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: get_reference_from_array");
    }
}
/*
  def getReferenceFromArray(
    address: ElementAddressV,
    expectedType: CoordH[KindHT],
    targetType: CoordH[KindHT]
  ): ReferenceV = {
    val ElementAddressV(objectId, elementIndex) = address
    objectsById.get(objectId).kind match {
      case ai @ ArrayInstanceV(_, _, _, _) => {
        val ref = ai.getElement(elementIndex)
        checkReference(expectedType, ref)
        transmute(ref, expectedType, targetType)
      }
    }
  }
*/
// mig: fn contains_live_object
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn contains_live_object(&self, reference: ReferenceV<'v, 'h, 's>) -> bool {
        panic!("Unimplemented: contains_live_object");
    }
}
/*
  def containsLiveObject(reference: ReferenceV): Boolean = {
    containsLiveObject(reference.allocId)
  }
*/
// mig: fn contains_live_object
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn contains_live_object_alloc_id(&self, alloc_id: AllocationIdV<'v, 'h, 's>) -> bool {
        panic!("Unimplemented: contains_live_object");
    }
}
/*
  private def containsLiveObject(allocId: AllocationId): Boolean = {
    if (!objectsById.contains(allocId))
      return false
    val alloc = objectsById.get(allocId)
    alloc.kind match {
      case StructInstanceV(_, None) => false
      case StructInstanceV(_, Some(_)) => true
      case _ => true
    }
  }
*/
// mig: fn contains_live_or_undead_object
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn contains_live_or_undead_object(&self, alloc_id: AllocationIdV<'v, 'h, 's>) -> bool {
        panic!("Unimplemented: contains_live_or_undead_object");
    }
}
/*
  // Undead meaning it's been zero'd out, but its still allocated because a weak
  // is pointing at it.
  private def containsLiveOrUndeadObject(allocId: AllocationId): Boolean = {
    objectsById.contains(allocId)
  }
*/
// mig: fn dereference
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn dereference_heap(&self, reference: ReferenceV, allow_undead: bool) -> KindV {
        panic!("Unimplemented: dereference");
    }
}
/*
  // Undead meaning it's been zero'd out, but its still allocated because a weak
  // is pointing at it.
  def dereference(reference: ReferenceV, allowUndead: Boolean = false): KindV = {
    if (!allowUndead) {
      vassert(containsLiveObject(reference.allocId))
    }
    objectsById.get(reference.allocId).kind
  }
*/
// mig: fn is_same_instance
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn is_same_instance(&self, call_id: CallIdV, left: ReferenceV, right: ReferenceV) -> ReferenceV {
        panic!("Unimplemented: is_same_instance");
    }
}
/*
  def isSameInstance(callId: CallId, left: ReferenceV, right: ReferenceV): ReferenceV = {
    val ref = allocateTransient(MutableShareH, InlineH, BoolV(left.allocId == right.allocId))
    incrementReferenceRefCount(RegisterToObjectReferrer(callId, MutableShareH), ref)
    ref
  }
*/
// mig: fn increment_reference_hold_count
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn increment_reference_hold_count(&self, expression_id: ExpressionIdV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>) {
        panic!("Unimplemented: increment_reference_hold_count");
    }
}
/*
  def incrementReferenceHoldCount(expressionId: ExpressionId, reference: ReferenceV) = {
    incrementObjectRefCount(RegisterHoldToObjectReferrer(expressionId, reference.ownership), reference.allocId)
  }
*/
// mig: fn decrement_reference_hold_count
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn decrement_reference_hold_count(&self, expression_id: ExpressionIdV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>) {
        panic!("Unimplemented: decrement_reference_hold_count");
    }
}
/*
  def decrementReferenceHoldCount(expressionId: ExpressionId, reference: ReferenceV) = {
    decrementObjectRefCount(RegisterHoldToObjectReferrer(expressionId, reference.ownership), reference.allocId)
  }
*/
// mig: fn increment_reference_ref_count
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn increment_reference_ref_count(&self, referrer: IObjectReferrerV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>) {
        panic!("Unimplemented: increment_reference_ref_count");
    }
}
/*
  // rename to incrementObjectRefCount
  def incrementReferenceRefCount(referrer: IObjectReferrer, reference: ReferenceV) = {
    incrementObjectRefCount(referrer, reference.allocId, reference.ownership == WeakH)
  }
*/
// mig: fn decrement_reference_ref_count
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn decrement_reference_ref_count(&self, referrer: IObjectReferrerV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>) {
        panic!("Unimplemented: decrement_reference_ref_count");
    }
}
/*
  // rename to decrementObjectRefCount
  def decrementReferenceRefCount(referrer: IObjectReferrer, reference: ReferenceV) = {
    decrementObjectRefCount(referrer, reference.allocId, reference.ownership == WeakH)
  }
*/
// mig: fn destructure_array
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn destructure_array(&self, reference: ReferenceV<'v, 'h, 's>) -> &'v [ReferenceV<'v, 'h, 's>] {
        panic!("Unimplemented: destructure_array");
    }
}
/*
  def destructureArray(reference: ReferenceV): Vector[ReferenceV] = {
    val allocation = dereference(reference)
    allocation match {
      case ArrayInstanceV(_, _, _, elements) => {
        val elementRefs =
          elements.indices.toVector
            .reverse
            .map(index => deinitializeArrayElement(reference))
            .reverse
        elementRefs
      }
    }
  }
*/
// mig: fn destructure
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn destructure(&self, reference: ReferenceV<'v, 'h, 's>) -> &'v [ReferenceV<'v, 'h, 's>] {
        panic!("Unimplemented: destructure");
    }
}
/*
  def destructure(reference: ReferenceV): Vector[ReferenceV] = {
    val allocation = dereference(reference)
    allocation match {
      case StructInstanceV(structDefH, Some(memberRefs)) => {
        memberRefs.zipWithIndex.foreach({ case (memberRef, index) =>
          decrementReferenceRefCount(
            MemberToObjectReferrer(
              MemberAddressV(reference.allocId, index), memberRef.ownership),
            memberRef)
        })

        zero(reference)
        deallocateIfNoWeakRefs(reference)
        memberRefs
      }
    }
  }
*/
// mig: fn zero
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn zero(&self, reference: ReferenceV<'v, 'h, 's>) {
        panic!("Unimplemented: zero");
    }
}
/*
  // This is *conceptually* destroying the object, this is when the owning
  // reference disappears. If there are no weak refs, we also deallocate.
  // Otherwise, we deallocate when the last weak ref disappears.
  def zero(reference: ReferenceV) = {
    val allocation = objectsById.get(reference.allocId)
    vassert(allocation.getTotalRefCount(Some(OwnH)) == 0)
    vassert(allocation.getTotalRefCount(Some(ImmutableBorrowH)) == 0)
    vassert(allocation.getTotalRefCount(Some(MutableBorrowH)) == 0)
    allocation.kind match {
      case si @ StructInstanceV(_, _) => si.zero()
      case _ =>
    }
    vivemDout.print(" o" + reference.allocId.num + "zero")
  }
*/
// mig: fn deallocate_if_no_weak_refs
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn deallocate_if_no_weak_refs(&self, reference: ReferenceV<'v, 'h, 's>) {
        panic!("Unimplemented: deallocate_if_no_weak_refs");
    }
}
/*
  // This happens when the last owning and weak references disappear.
  def deallocateIfNoWeakRefs(reference: ReferenceV) = {
    val allocation = objectsById.get(reference.allocId)
    if (reference.actualCoord.hamut.ownership == OwnH &&
      (allocation.getTotalRefCount(Some(MutableBorrowH)) + allocation.getTotalRefCount(Some(ImmutableBorrowH))) > 0) {
      throw ConstraintViolatedException("Constraint violated!")
    }
    if (allocation.getTotalRefCount(None) == 0) {
      objectsById.remove(reference.allocId)
      vivemDout.print(" o" + reference.allocId.num + "dealloc")
    }
  }
*/
// mig: fn increment_object_ref_count
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn increment_object_ref_count(&self, pointing_from: IObjectReferrerV<'v, 'h, 's>, alloc_id: AllocationIdV<'v, 'h, 's>, allow_undead: bool) {
        panic!("Unimplemented: increment_object_ref_count");
    }
}
/*
  private def incrementObjectRefCount(
      pointingFrom: IObjectReferrer,
      allocId: AllocationId,
      allowUndead: Boolean = false): Unit = {
    if (!allowUndead) {
      if (!containsLiveObject(allocId)) {
        vfail("Trying to increment dead object: " + allocId)
      }
    }
    val obj = objectsById.get(allocId)
    obj.incrementRefCount(pointingFrom)
    val newRefCount = obj.getTotalRefCount(None)
    vivemDout.print(" o" + allocId.num + "rc" + (newRefCount - 1) + "->" + newRefCount)
  }
*/
// mig: fn decrement_object_ref_count
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn decrement_object_ref_count(&self, pointed_from: IObjectReferrerV<'v, 'h, 's>, alloc_id: AllocationIdV<'v, 'h, 's>, allow_undead: bool) -> i32 {
        panic!("Unimplemented: decrement_object_ref_count");
    }
}
/*
  private def decrementObjectRefCount(
      pointedFrom: IObjectReferrer,
      allocId: AllocationId,
      allowUndead: Boolean = false):
  Int = {
    if (!allowUndead) {
      if (!containsLiveObject(allocId)) {
        vfail("Can't decrement object " + allocId + ", not in heap!")
      }
    }
    val obj = objectsById.get(allocId)
    obj.decrementRefCount(pointedFrom)
    val newRefCount = obj.getTotalRefCount(None)
    vivemDout.print(" o" + allocId.num + "rc" + (newRefCount + 1) + "->" + newRefCount)
//    if (newRefCount == 0) {
//      deallocate(objectId)
//    }
    newRefCount
  }
*/
// mig: fn get_ref_count
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn get_ref_count(&self, reference: ReferenceV<'v, 'h, 's>) -> i32 {
        panic!("Unimplemented: get_ref_count");
    }
}
/*
  def getRefCount(reference: ReferenceV): Int = {
    if (reference.actualKind.hamut == VoidHT()) {
      // Let's pretend Void permanently has 1 RC, so nobody tries to free it
      return 1
    }
    if (reference.ownership == WeakH) {
      vassert(containsLiveOrUndeadObject(reference.allocId))
    } else {
      vassert(containsLiveObject(reference.allocId))
    }
    val allocation = objectsById.get(reference.allocId)
    allocation.getRefCount()
  }
*/
// mig: fn get_total_ref_count
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn get_total_ref_count(&self, reference: ReferenceV<'v, 'h, 's>) -> i32 {
        panic!("Unimplemented: get_total_ref_count");
    }
}
/*
  def getTotalRefCount(reference: ReferenceV): Int = {
    if (reference.ownership == WeakH) {
      vassert(containsLiveOrUndeadObject(reference.allocId))
    } else {
      vassert(containsLiveObject(reference.allocId))
    }
    val allocation = objectsById.get(reference.allocId)
    allocation.getTotalRefCount(None)
  }
*/
// mig: fn ensure_ref_count
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn ensure_ref_count(&self, reference: ReferenceV<'v, 'h, 's>, ownership_filter: Option<&'v [OwnershipH]>, expected_num: i32) {
        panic!("Unimplemented: ensure_ref_count");
    }
}
/*
  def ensureRefCount(reference: ReferenceV, ownershipFilter: Option[Set[OwnershipH]], expectedNum: Int) = {
    vassert(containsLiveObject(reference.allocId))
    val allocation = objectsById.get(reference.allocId)
    allocation.ensureRefCount(ownershipFilter, expectedNum)
  }
*/
// mig: fn add
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn add_heap(&self, ownership: OwnershipH, location: LocationH, kind: KindV<'v, 'h, 's>) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: add");
    }
}
/*
  def add(ownership: OwnershipH, location: LocationH, kind: KindV): ReferenceV = {
    objectsById.add(ownership, location, kind)
  }
*/
// mig: fn transmute
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn transmute(&self, reference: ReferenceV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>, target_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: transmute");
    }
}
/*
  def transmute(
    reference: ReferenceV,
    expectedType: CoordH[KindHT],
    targetType: CoordH[KindHT]):
  ReferenceV = {
    if (expectedType == targetType) {
      return reference
    }

    val ReferenceV(actualKind, oldSeenAsType, oldOwnership, oldLocation, objectId) = reference
    vassert(
      (oldOwnership == MutableShareH || oldOwnership == ImmutableShareH) ==
        (targetType.ownership == MutableShareH || targetType.ownership == ImmutableShareH))
    if (oldSeenAsType.hamut != expectedType.kind) {
      // not sure if the above .actualType is right

      vfail("wot")
    }

    ReferenceV(
      actualKind,
      RRKind(targetType.kind),
      targetType.ownership,
      targetType.location,
      objectId)
  }
*/
// mig: fn cast
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn cast(&self, call_id: CallIdV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>, target_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: cast");
    }
}
/*
  def cast(
    callId: CallId,
    reference: ReferenceV,
    expectedType: CoordH[KindHT],
    targetType: CoordH[KindHT]):
  ReferenceV = {
    if (expectedType == targetType) {
      return reference
    }

    val ReferenceV(actualKind, oldSeenAsType, oldOwnership, oldLocation, objectId) = reference
    vassert((oldOwnership == MutableShareH) == (targetType.ownership == MutableShareH))
    if (oldSeenAsType.hamut != expectedType.kind) {
      // not sure if the above .actualType is right

      vfail("wot")
    }

    incrementReferenceRefCount(RegisterToObjectReferrer(callId, targetType.ownership), reference)
    decrementReferenceRefCount(RegisterToObjectReferrer(callId, oldOwnership), reference)

    ReferenceV(
      actualKind,
      RRKind(targetType.kind),
      targetType.ownership,
      targetType.location,
      objectId)
  }
*/
// mig: fn is_empty
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn is_empty_heap(&self) -> bool {
        panic!("Unimplemented: is_empty");
    }
}
/*
  def isEmpty: Boolean = {
    objectsById.isEmpty
  }
*/
// mig: fn print_all
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn print_all_heap(&self) {
        panic!("Unimplemented: print_all");
    }
}
/*
  def printAll() = {
    objectsById.printAll()
  }
*/
// mig: fn count_unreachable_allocations
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn count_unreachable_allocations(&self, roots: &'v [ReferenceV<'v, 'h, 's>]) -> usize {
        panic!("Unimplemented: count_unreachable_allocations");
    }
}
/*
  def countUnreachableAllocations(roots: Vector[ReferenceV]) = {
    val numReachables = findReachableAllocations(roots).size
    vassert(numReachables <= objectsById.size)
    objectsById.size - numReachables
  }
*/
// mig: fn find_reachable_allocations
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn find_reachable_allocations(&self, input_reachables: &'v [ReferenceV<'v, 'h, 's>]) -> HashMap<ReferenceV<'v, 'h, 's>, AllocationV<'v, 'h, 's>> {
        panic!("Unimplemented: find_reachable_allocations");
    }
}
/*
  def findReachableAllocations(
      inputReachables: Vector[ReferenceV]): Map[ReferenceV, Allocation] = {
    val destinationMap = mutable.Map[ReferenceV, Allocation]()
    inputReachables.foreach(inputReachable => {
      innerFindReachableAllocations(destinationMap, inputReachable)
    })
    innerFindReachableAllocations(destinationMap, void)
    destinationMap.toMap
  }
*/
// mig: fn inner_find_reachable_allocations
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn inner_find_reachable_allocations(&self, destination_map: &mut HashMap<ReferenceV<'v, 'h, 's>, AllocationV<'v, 'h, 's>>, input_reachable: ReferenceV<'v, 'h, 's>) {
        panic!("Unimplemented: inner_find_reachable_allocations");
    }
}
/*
  private def innerFindReachableAllocations(
      destinationMap: mutable.Map[ReferenceV, Allocation],
      inputReachable: ReferenceV): Unit = {
    // Doublecheck that all the inputReachables are actually in this ..
    vassert(containsLiveObject(inputReachable.allocId))
    vassert(objectsById.get(inputReachable.allocId).kind.tyype.hamut == inputReachable.actualKind.hamut)

    val allocation = objectsById.get(inputReachable.allocId)
    if (destinationMap.contains(inputReachable)) {
      return
    }

    destinationMap.put(inputReachable, allocation)
    allocation.kind match {
      case IntV(_, _) =>
      case VoidV =>
      case BoolV(_) =>
      case FloatV(_) =>
      case StructInstanceV(structDefH, Some(members)) => {
        members.zip(structDefH.members).foreach({
          case (reference, StructMemberH(_, _, referenceH)) => {
            innerFindReachableAllocations(destinationMap, reference)
          }
        })
      }
    }
  }
*/
// mig: fn check_for_leaks
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn check_for_leaks_heap(&self) {
        panic!("Unimplemented: check_for_leaks");
    }
}
/*
  def checkForLeaks(): Unit = {
    objectsById.checkForLeaks()
  }
*/
// mig: fn get_current_call
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn get_current_call(&self, expected_call_id: CallIdV<'v, 'h, 's>) -> CallV<'v, 'h, 's> {
        panic!("Unimplemented: get_current_call");
    }
}
/*
  def getCurrentCall(expectedCallId: CallId) = {
    vassert(callIdStack.top == expectedCallId)
    callsById(expectedCallId)
  }
*/
// mig: fn take_argument
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn take_argument(&self, call_id: CallIdV<'v, 'h, 's>, argument_index: i32, expected_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: take_argument");
    }
}
/*
  def takeArgument(callId: CallId, argumentIndex: Int, expectedType: CoordH[KindHT]) = {
    val reference = getCurrentCall(callId).takeArgument(argumentIndex)
    checkReference(expectedType, reference)
    decrementReferenceRefCount(
      ArgumentToObjectReferrer(ArgumentId(callId, argumentIndex), expectedType.ownership),
      reference) // decrementing because taking it out of arg
    // Now, the register is the only one that has this reference.
    reference
  }
*/
// mig: fn allocate_transient
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn allocate_transient(&self, ownership: OwnershipH, location: LocationH, kind: KindV) -> ReferenceV {
        panic!("Unimplemented: allocate_transient");
    }
}
/*
  // For example, for the integer we pass into the array generator
  def allocateTransient(ownership: OwnershipH, location: LocationH, kind: KindV) = {
    val ref = add(ownership, location, kind)
    vivemDout.print(" o" + ref.allocId.num + "=")
    printKind(kind)
    ref
  }
*/
// mig: fn print_kind
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn print_kind(&self, kind: KindV<'v, 'h, 's>) {
        panic!("Unimplemented: print_kind");
    }
}
/*
  def printKind(kind: KindV) = {
    kind match {
      case VoidV => vivemDout.print("void")
      case IntV(value, _) => vivemDout.print(value)
      case OpaqueV(_) => vivemDout.print("opaque")
      case BoolV(value) => vivemDout.print(value)
      case StrV(value) => vivemDout.print(value)
      case FloatV(value) => vivemDout.print(value)
      case StructInstanceV(structH, Some(members)) => {
        vivemDout.print(structH.id + "{" + members.map("o" + _.allocId.num).mkString(", ") + "}")
      }
      case ArrayInstanceV(typeH, memberTypeH, capacity, elements) => vivemDout.print("array:" + capacity + ":" + memberTypeH + "{" + elements.map("o" + _.allocId.num).mkString(", ") + "}")
    }
  }
*/
// mig: fn initialize_array_element
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn initialize_array_element(&self, array_reference: ReferenceV<'v, 'h, 's>, ret: ReferenceV<'v, 'h, 's>) {
        panic!("Unimplemented: initialize_array_element");
    }
}
/*
  def initializeArrayElement(
      arrayReference: ReferenceV,
      ret: ReferenceV) = {
    dereference(arrayReference) match {
      case a @ ArrayInstanceV(_, _, _, _) => {
        incrementReferenceRefCount(
          ElementToObjectReferrer(
            ElementAddressV(arrayReference.allocId, a.getSize()), a.elementTypeH.ownership),
          ret)
        a.initializeElement(ret)
      }
    }
  }
*/
// mig: fn new_struct
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn new_struct(&self, struct_def_h: StructDefinitionH<'s, 'h>, struct_ref_h: CoordH<'s, 'h>, member_references: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: new_struct");
    }
}
/*
  def newStruct(
      structDefH: StructDefinitionH,
      structRefH: CoordH[StructHT],
      memberReferences: Vector[ReferenceV]):
  ReferenceV = {
    val instance = StructInstanceV(structDefH, Some(memberReferences.toVector))
    val reference = add(structRefH.ownership, structRefH.location, instance)

    memberReferences.zipWithIndex.foreach({ case (memberReference, index) =>
      incrementReferenceRefCount(
        MemberToObjectReferrer(MemberAddressV(reference.allocId, index), memberReference.ownership),
        memberReference)
    })

    vivemDout.print(" o" + reference.num + "=")
    printKind(instance)
    reference
  }
*/
// mig: fn new_opaque
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn new_opaque_heap(&self, opaque_coord_ht: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: new_opaque");
    }
}
/*
  def newOpaque(opaqueCoordHT: CoordH[OpaqueHT]): ReferenceV = {
    val instance = OpaqueV(opaqueCoordHT.kind)
    val reference = add(opaqueCoordHT.ownership, opaqueCoordHT.location, instance)

    vivemDout.print(" o" + reference.num + "=")
    printKind(instance)
    reference
  }
*/
// mig: fn deinitialize_array_element
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn deinitialize_array_element(&self, array_reference: ReferenceV<'v, 'h, 's>) -> ReferenceV<'v, 'h, 's> {
        panic!("Unimplemented: deinitialize_array_element");
    }
}
/*
  def deinitializeArrayElement(arrayReference: ReferenceV) = {
    val arrayInstance @ ArrayInstanceV(_, _, _, _) = dereference(arrayReference)
    val elementReference = arrayInstance.deinitializeElement()
    decrementReferenceRefCount(
      ElementToObjectReferrer(ElementAddressV(arrayReference.allocId, arrayInstance.getSize()), elementReference.ownership),
      elementReference)
    elementReference
  }

//  def initializeArrayElementFromRegister(
//      arrayReference: ReferenceV,
//      elementReference: ReferenceV) = {
//    val arrayInstance @ ArrayInstanceV(_, _, _, _) = dereference(arrayReference)
//    val index = arrayInstance.getSize()
//    incrementReferenceRefCount(
//      ElementToObjectReferrer(ElementAddressV(arrayReference.allocId, index), elementReference.ownership),
//      elementReference)
//    arrayInstance.initializeElement(elementReference)
//  }
*/
// mig: fn add_uninitialized_array
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn add_uninitialized_array(&self, array_definition_th: RuntimeSizedArrayDefinitionHT, array_ref_type: CoordH<'s, 'h>, capacity: i32) -> (ReferenceV<'v, 'h, 's>, ArrayInstanceV<'v, 'h, 's>) {
        panic!("Unimplemented: add_uninitialized_array");
    }
}
/*
  def addUninitializedArray(
      arrayDefinitionTH: RuntimeSizedArrayDefinitionHT,
      arrayRefType: CoordH[RuntimeSizedArrayHT],
      capacity: Int):
  (ReferenceV, ArrayInstanceV) = {
    val instance = ArrayInstanceV(arrayRefType, arrayDefinitionTH.elementType, capacity, Vector())
    val reference = add(arrayRefType.ownership, arrayRefType.location, instance)
    (reference, instance)
  }
*/
// mig: fn add_array
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn add_array(&self, array_definition_th: StaticSizedArrayDefinitionHT, array_ref_type: CoordH<'s, 'h>, member_refs: &'v [ReferenceV<'v, 'h, 's>]) -> (ReferenceV<'v, 'h, 's>, ArrayInstanceV<'v, 'h, 's>) {
        panic!("Unimplemented: add_array");
    }
}
/*
  def addArray(
    arrayDefinitionTH: StaticSizedArrayDefinitionHT,
    arrayRefType: CoordH[StaticSizedArrayHT],
    memberRefs: Vector[ReferenceV]):
  (ReferenceV, ArrayInstanceV) = {
    val instance = ArrayInstanceV(arrayRefType, arrayDefinitionTH.elementType, memberRefs.size, memberRefs.toVector)
    val reference = add(arrayRefType.ownership, arrayRefType.location, instance)
    memberRefs.zipWithIndex.foreach({ case (memberRef, index) =>
      incrementReferenceRefCount(
        ElementToObjectReferrer(ElementAddressV(reference.allocId, index), memberRef.ownership),
        memberRef)
    })
    (reference, instance)
  }
*/
// mig: fn check_reference
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn check_reference(&self, expected_type: CoordH<'s, 'h>, actual_reference: ReferenceV<'v, 'h, 's>) {
        panic!("Unimplemented: check_reference");
    }
}
/*

  def checkReference(expectedType: CoordH[KindHT], actualReference: ReferenceV): Unit = {
    if (actualReference.ownership == WeakH) {
      vassert(containsLiveOrUndeadObject(actualReference.allocId))
    } else {
      vassert(containsLiveObject(actualReference.allocId))
    }
    if ((actualReference.seenAsCoord.hamut.ownership match {
      case ImmutableShareH => MutableShareH
      case ImmutableBorrowH => MutableBorrowH
      case other => other
    }) != (expectedType.ownership  match {
      case ImmutableShareH => MutableShareH
      case ImmutableBorrowH => MutableBorrowH
      case other => other
    })) {
      vfail("Expected " + expectedType + " but was " + actualReference.seenAsCoord.hamut)
    }
    if (actualReference.seenAsCoord.hamut.location != expectedType.location) {
      vfail("Expected " + expectedType + " but was " + actualReference.seenAsCoord.hamut)
    }
    if (actualReference.seenAsCoord.hamut.kind != expectedType.kind) {
      vfail("Expected " + expectedType + " but was " + actualReference.seenAsCoord.hamut)
    }
    val actualKind = dereference(actualReference, actualReference.ownership == WeakH)
    checkKind(expectedType.kind, actualKind)
  }
*/
// mig: fn check_reference_register
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn check_reference_register(&self, tyype: CoordH<'s, 'h>, register: RegisterV<'v, 'h, 's>) -> ReferenceRegisterV<'v, 'h, 's> {
        panic!("Unimplemented: check_reference_register");
    }
}
/*

  def checkReferenceRegister(tyype: CoordH[KindHT], register: RegisterV): ReferenceRegisterV = {
    val reg = register.expectReferenceRegister()
    checkReference(tyype, reg.reference)
    reg
  }
*/
// mig: fn check_kind
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn check_kind(&self, expected_type: KindHT<'s, 'h>, actual_kind: KindV<'v, 'h, 's>) {
        panic!("Unimplemented: check_kind");
    }
}
/*
  def checkKind(expectedType: KindHT, actualKind: KindV): Unit = {
    (actualKind, expectedType) match {
      case (IntV(_, actualBits), IntHT(expectedBits)) => {
        if (actualBits != expectedBits) {
          vfail("Expected " + expectedType + " but was " + actualKind)
        }
      }
      case (OpaqueV(a), b @ OpaqueHT(_, _, _)) => {
        if (a != b) {
          vfail("Expected " + expectedType + " but was " + actualKind)
        }
      }
      case (VoidV, VoidHT()) =>
      case (BoolV(_), BoolHT()) =>
      case (StrV(_), StrHT()) =>
      case (FloatV(_), FloatHT()) =>
      case (StructInstanceV(structDefH, _), structRefH @ StructHT(_)) => {
        if (structDefH.getRef != structRefH) {
          vfail("Expected " + structRefH + " but was " + structDefH)
        }
      }
      case (ArrayInstanceV(typeH, actualElementTypeH, _, _), arrayH @ RuntimeSizedArrayHT(_)) => {
        if (typeH.kind != arrayH) {
          vfail("Expected " + arrayH + " but was " + typeH)
        }
      }
      case (ArrayInstanceV(typeH, actualElementTypeH, _, _), arrayH @ StaticSizedArrayHT(_)) => {
        if (typeH.kind != arrayH) {
          vfail("Expected " + arrayH + " but was " + typeH)
        }
      }
      case (StructInstanceV(structDefH, _), irH @ InterfaceHT(_)) => {
        val structImplementsInterface =
          structDefH.edges.exists(_.interface == irH)
        if (!structImplementsInterface) {
          vfail("Struct " + structDefH.getRef + " doesnt implement interface " + irH)
        }
      }
      case (a, b) => {
        vfail("Mismatch! " + a + " is not a " + b)
      }
    }
  }
*/
// mig: fn check_struct_id
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn check_struct_id(&self, expected_struct_type: StructHT<'s, 'h>, expected_struct_pointer_type: CoordH<'s, 'h>, register: RegisterV<'v, 'h, 's>) -> AllocationIdV<'v, 'h, 's> {
        panic!("Unimplemented: check_struct_id");
    }
}
/*
  def checkStructId(expectedStructType: StructHT, expectedStructPointerType: CoordH[KindHT], register: RegisterV): AllocationId = {
    val reference = checkReferenceRegister(expectedStructPointerType, register).reference
    dereference(reference) match {
      case siv @ StructInstanceV(structDefH, _) => {
        vassert(structDefH.getRef == expectedStructType)
      }
      case _ => vfail("Expected a struct but was " + register)
    }
    reference.allocId
  }
*/
// mig: fn check_struct_coord
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn check_struct_coord(&self, expected_struct_type: StructHT<'s, 'h>, expected_struct_pointer_type: CoordH<'s, 'h>, register: RegisterV<'v, 'h, 's>) -> StructInstanceV<'v, 'h, 's> {
        panic!("Unimplemented: check_struct_coord");
    }
}
/*
  def checkStructCoord(expectedStructType: StructHT, expectedStructPointerType: CoordH[KindHT], register: RegisterV): StructInstanceV = {
    val reference = checkReferenceRegister(expectedStructPointerType, register).reference
    dereference(reference) match {
      case siv @ StructInstanceV(structDefH, _) => {
        vassert(structDefH.getRef == expectedStructType)
        siv
      }
      case _ => vfail("Expected a struct but was " + register)
    }
  }
*/
// mig: fn check_struct_coord
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn check_struct_coord_ref(&self, expected_struct_type: StructHT<'s, 'h>, reference: ReferenceV<'v, 'h, 's>) -> StructInstanceV<'v, 'h, 's> {
        panic!("Unimplemented: check_struct_coord");
    }
}
/*
  def checkStructCoord(expectedStructType: StructHT, reference: ReferenceV): StructInstanceV = {
    dereference(reference) match {
      case siv @ StructInstanceV(structDefH, _) => {
        vassert(structDefH.getRef == expectedStructType)
        siv
      }
      case _ => vfail("Expected a struct but was " + reference)
    }
  }
*/
// mig: fn push_new_stack_frame
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn push_new_stack_frame(&self, function_h: PrototypeH<'s, 'h>, args: &'v [ReferenceV<'v, 'h, 's>]) -> CallIdV<'v, 'h, 's> {
        panic!("Unimplemented: push_new_stack_frame");
    }
}
/*
  def pushNewStackFrame(functionH: PrototypeH, args: Vector[ReferenceV]) = {
    vassert(callsById.size == callIdStack.size)
    val callId =
      CallId(
        if (callIdStack.nonEmpty) callIdStack.top.callDepth + 1 else 0,
        functionH)
    val call = new Call(callId, args)
    callsById.put(callId, call)
    callIdStack.push(callId)
    vassert(callsById.size == callIdStack.size)
    callId
  }
*/
// mig: fn pop_stack_frame
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn pop_stack_frame(&self, expected_call_id: CallIdV<'v, 'h, 's>) {
        panic!("Unimplemented: pop_stack_frame");
    }
}
/*
  def popStackFrame(expectedCallId: CallId): Unit = {
    vassert(callsById.size == callIdStack.size)
    vassert(callIdStack.top == expectedCallId)
    val call = callsById(expectedCallId)
    call.prepareToDie()
    callIdStack.pop()
    callsById.remove(expectedCallId)
    vassert(callsById.size == callIdStack.size)
  }
*/
// mig: fn to_von
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn to_von(&self, reference: ReferenceV<'v, 'h, 's>) -> IVonData {
        panic!("Unimplemented: to_von");
    }
}
/*
  def toVon(ref: ReferenceV): IVonData = {
    dereference(ref) match {
      case VoidV => VonObject("void", None, Vector())
      case IntV(value, bits) => VonInt(value)
      case FloatV(value) => VonFloat(value)
      case BoolV(value) => VonBool(value)
      case StrV(value) => VonStr(value)
      case ArrayInstanceV(_, _, _, elements) => {
        VonArray(None, elements.map(toVon))
      }
      case StructInstanceV(structH, Some(members)) => {
        vassert(members.size == structH.members.size)
        von.VonObject(
          structH.id.toString,
          None,
          structH.members.zip(members).zipWithIndex.map({ case ((memberH, memberV), index) =>
            VonMember(vimpl(memberH.name.toString), toVon(memberV))
          }).toVector)
      }
    }
  }
*/
// mig: fn get_var_address
pub fn get_var_address<'v, 'h, 's>(call_id: CallIdV<'v, 'h, 's>, local: Local<'s, 'h>) -> VariableAddressV<'v, 'h, 's> {
    panic!("Unimplemented: get_var_address");
}
/*
  def getVarAddress(callId: CallId, local: Local) = {
    VariableAddressV(callId, local)
  }
}

*/
