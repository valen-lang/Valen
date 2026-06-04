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
    StructInstanceV, VariableAddressV, VariableV, VoidV,
};
use crate::testvm::call::CallV;
use crate::final_ast::types::OpaqueHT;
use crate::von::ast::IVonData;
use crate::testvm::vivem::PrintStream;

/*
package dev.vale.testvm

import dev.vale.finalast._
import dev.vale.{IntCounter, vassert, vassertSome, vfail, vimpl, von}

import java.io.PrintStream
import dev.vale.finalast._
import dev.vale.von.{IVonData, VonArray, VonBool, VonFloat, VonInt, VonMember, VonObject, VonStr}

import scala.collection.mutable
*/
// mig: struct AdapterForExternsV<'a, 'v, 'h, 's>
/// Temporary state
pub struct AdapterForExternsV<'a, 'v, 'h, 's>
where 's: 'h, 'h: 'v, 'v: 'a,
{
    pub program_h: &'h ProgramH<'s, 'h>,
    pub interner: &'a crate::simplifying::hammer_interner::HammerInterner<'s, 'h>,
    pub scout_arena: &'a crate::scout_arena::ScoutArena<'s>,
    pub heap: &'a mut HeapV<'v, 'h, 's>,
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
impl<'a, 'v, 'h, 's> AdapterForExternsV<'a, 'v, 'h, 's> where 's: 'h, 'h: 'v, 'v: 'a {
    pub fn dereference(&self, reference: ReferenceV<'v, 'h, 's>) -> KindV<'v, 'h, 's> {
        self.heap.dereference(reference, false)
    }
}
/*
  def dereference(reference: ReferenceV) = {
    heap.dereference(reference)
  }
*/
// mig: fn new_opaque
impl<'a, 'v, 'h, 's> AdapterForExternsV<'a, 'v, 'h, 's> where 's: 'h, 'h: 'v, 'v: 'a {
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
impl<'a, 'v, 'h, 's> AdapterForExternsV<'a, 'v, 'h, 's> where 's: 'h, 'h: 'v, 'v: 'a {
    pub fn add_allocation_for_return(&mut self, ownership: OwnershipH, location: LocationH, kind: KindV<'v, 'h, 's>) -> ReferenceV<'v, 'h, 's> {
        let r#ref = self.heap.add(self.interner, ownership, location, kind);
//    heap.incrementReferenceRefCount(ResultToObjectReferrer(callId), ref) // incrementing because putting it in a return
//    ReturnV(callId, ref)
        r#ref
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
impl<'a, 'v, 'h, 's> AdapterForExternsV<'a, 'v, 'h, 's> where 's: 'h, 'h: 'v, 'v: 'a {
    pub fn make_void(&self) -> ReferenceV<'v, 'h, 's> {
        self.heap.void()
    }
}
/*
  def makeVoid(): ReferenceV = {
    heap.void
  }
}
*/
// Per Scala `object AllocationMap.STARTING_ID`. The matching `addImpl` body in
// Rust is the free fn `allocation_map_add_impl` further down — its Scala
// counterpart sits in the audit block below at canonical source order; SCPX
// matches Scala's `object AllocationMap` placement before `class AllocationMap`.
const STARTING_ID: i32 = 501;
/*
object AllocationMap {
  val STARTING_ID = 501

  // Factored body of `add` that takes the underlying state by reference (via
  // IntCounter for nextId) so it can run both during the class-body
  // initialization of `val void` (before `this` is fully constructed) and as
  // the method body of `def add`. Mirrors Rust's `allocation_map_add_impl`
  // which takes `&mut HashMap` and `&mut i32`.
  def addImpl(
      objectsById: mutable.HashMap[AllocationId, Allocation],
      nextId: IntCounter,
      ownership: OwnershipH,
      location: LocationH,
      kind: KindV): ReferenceV = {
    val id = nextId.increment()
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
}
*/
// mig: fn allocation_map_add_impl
// 1:1 with Scala `object AllocationMap.addImpl` above. `&mut HashMap` + `&mut i32`
// mirror Scala's HashMap + IntCounter wrapper. `interner` is Exception-B (Rust's
// sealed StructHT requires explicit interner where Scala used GC + structural eq).
fn allocation_map_add_impl<'v, 'h, 's>(
    objects_by_id: &mut HashMap<AllocationIdV<'v, 'h, 's>, AllocationV<'v, 'h, 's>>,
    next_id: &mut i32,
    interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>,
    ownership: OwnershipH,
    location: LocationH,
    kind: KindV<'v, 'h, 's>,
) -> ReferenceV<'v, 'h, 's> {
    let id = *next_id;
    *next_id = id + 1;
    if let KindV::Void(_) = kind {
        assert_eq!(id, STARTING_ID);
    }
    let reference = ReferenceV {
        actual_kind: kind.tyype(interner),
        seen_as_kind: kind.tyype(interner),
        ownership,
        location,
        num: id,
    };
    let allocation = AllocationV { reference, kind, referrers: HashMap::new() };
    objects_by_id.insert(reference.alloc_id(), allocation);
    reference
}

// mig: struct AllocationMapV<'v, 'h, 's>
/// Temporary state
pub struct AllocationMapV<'v, 'h, 's> {
    pub objects_by_id: HashMap<AllocationIdV<'v, 'h, 's>, AllocationV<'v, 'h, 's>>,
    pub next_id: i32,
    pub void_ref: ReferenceV<'v, 'h, 's>,
}
/*
class AllocationMap(vivemDout: PrintStream) {
  private val objectsById = mutable.HashMap[AllocationId, Allocation]()
  private val nextId = new IntCounter(AllocationMap.STARTING_ID)
  val void: ReferenceV = AllocationMap.addImpl(objectsById, nextId, MutableShareH, InlineH, VoidV)
*/
// mig: fn new
// Mirrors the AllocationMap(vivemDout: PrintStream) constructor in the Scala
// audit block above: initializes objectsById/STARTING_ID/nextId, then
// `val void = add(MutableShareH, InlineH, VoidV)`.
impl<'v, 'h, 's> AllocationMapV<'v, 'h, 's> {
    pub fn new(interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>) -> AllocationMapV<'v, 'h, 's> {
        let mut objects_by_id = HashMap::new();
        let mut next_id = STARTING_ID;
        let void_ref = allocation_map_add_impl(&mut objects_by_id, &mut next_id, interner, OwnershipH::MutableShareH, LocationH::InlineH, KindV::Void(VoidV));
        AllocationMapV { objects_by_id, next_id, void_ref }
    }
}
/* Guardian: disable-all */
// (`def newId` removed in the refactor that introduced `IntCounter` —
// nextId.increment() is now called inline inside addImpl.)
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
    pub fn get(&self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, alloc_id: AllocationIdV<'v, 'h, 's>) -> &AllocationV<'v, 'h, 's> {
        let allocation = self.objects_by_id.get(&alloc_id).expect("get: not found");
        assert!(allocation.kind.tyype(interner) == alloc_id.tyype);
        allocation
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
    pub fn add(&mut self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, ownership: OwnershipH, location: LocationH, kind: KindV<'v, 'h, 's>) -> ReferenceV<'v, 'h, 's> {
        allocation_map_add_impl(&mut self.objects_by_id, &mut self.next_id, interner, ownership, location, kind)
    }
}
/*
  def add(ownership: OwnershipH, location: LocationH, kind: KindV): ReferenceV =
    AllocationMap.addImpl(objectsById, nextId, ownership, location, kind)
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
    pub fn check_for_leaks(&self, vivem_dout: &mut PrintStream) {
        let m = &self.objects_by_id;
        let non_interned_objects: Vec<&AllocationV<'v, 'h, 's>> = m.values().filter(|a| !matches!(a.kind, KindV::Void(_))).collect();
        if !non_interned_objects.is_empty() {
            let mut ids: Vec<i32> = non_interned_objects.iter().map(|a| a.reference.alloc_id().num).collect();
            ids.sort();
            use std::io::Write;
            for obj_id in &ids {
                write!(vivem_dout, "o{} ", obj_id).unwrap();
            }
            writeln!(vivem_dout).unwrap();
            let mut sorted: Vec<&AllocationV<'v, 'h, 's>> = non_interned_objects.clone();
            sorted.sort_by_key(|a| a.reference.alloc_id().num);
            for a in &sorted {
                a.print_refs(vivem_dout);
            }
            panic!("Memory leaks! See above for ");
        }
        drop(non_interned_objects);
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
    pub vivem_dout: &'v mut PrintStream,
    pub vivem_bump: &'v bumpalo::Bump,
    pub objects_by_id: AllocationMapV<'v, 'h, 's>,
    pub call_id_stack: Vec<CallIdV<'v, 'h, 's>>,
    pub calls_by_id: HashMap<CallIdV<'v, 'h, 's>, CallV<'v, 'h, 's>>,
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
// mig: fn new
// Mirrors the Heap(in_vivemDout: PrintStream) constructor in the Scala audit
// block above: stores vivemDout, builds objectsById = new AllocationMap(vivemDout),
// initializes callIdStack/callsById, sets void = objectsById.void.
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn new(interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, vivem_dout: &'v mut PrintStream, vivem_bump: &'v bumpalo::Bump) -> HeapV<'v, 'h, 's> {
        HeapV {
            objects_by_id: AllocationMapV::new(interner),
            call_id_stack: Vec::new(),
            calls_by_id: HashMap::new(),
            vivem_dout,
            vivem_bump,
        }
    }
}
/* Guardian: disable-all */
// mig: fn void (HeapV accessor mirroring Scala's `val void = objectsById.void` field)
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn void(&self) -> ReferenceV<'v, 'h, 's> {
        self.objects_by_id.void_ref
    }
}

// mig: fn add_local
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn add_local(&mut self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, var_addr: VariableAddressV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>) {
        self.check_reference(interner, expected_type, reference);
        self.get_current_call(var_addr.call_id, |call| {
            call.add_local(var_addr, reference, expected_type);
        });
        self.increment_reference_ref_count(
            IObjectReferrerV::VariableToObjectReferrer(crate::testvm::values::VariableToObjectReferrerV { var_addr, ownership: expected_type.ownership }),
            reference,
        );
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
    pub fn remove_local(&mut self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, var_addr: VariableAddressV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>) {
        let variable = self.get_local(var_addr);
        let actual_reference = variable.reference;
        self.check_reference(interner, expected_type, actual_reference);
        self.decrement_reference_ref_count(
            IObjectReferrerV::VariableToObjectReferrer(crate::testvm::values::VariableToObjectReferrerV { var_addr, ownership: expected_type.ownership }),
            actual_reference,
        );
        self.get_current_call(var_addr.call_id, |call| {
            call.remove_local(var_addr);
        });
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
    pub fn get_reference_from_local(&self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, var_addr: VariableAddressV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>, target_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        let variable = self.get_local(var_addr);
        if variable.expected_type != expected_type {
            panic!("blort");
        }
        let variable_reference = variable.reference;
        self.check_reference(interner, expected_type, variable_reference);
        self.transmute(variable_reference, expected_type, target_type)
    }
}
/*
Guardian: temp-disable: SPDMX — Threading interner (HammerInterner) through this call to feed StructDefinitionH.get_ref(interner) is SPDMX Exception B — the exception text explicitly enumerates interner as a sanctioned arena adaptation parameter. Scala used GC + module-singleton interning; Rust requires explicit parameter threading. Same pattern landed without firing on check_kind/check_reference/add_local/remove_local this turn — Guardian is inconsistent on the same Exception-B shape. — FrontendRust/guardian-logs/request-436-1780420663942/hook-436/get_reference_from_local--426.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
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
        let calls_by_id = &self.calls_by_id;
        let call = calls_by_id.get(&var_addr.call_id).expect("get_local: call not found");
        call.get_local(var_addr)
    }
}
/*
  private def getLocal(varAddr: VariableAddressV): VariableV = {
    callsById(varAddr.callId).getLocal(varAddr)
  }
*/
// mig: fn mutate_variable
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    pub fn mutate_variable(&mut self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, var_address: VariableAddressV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        let variable = self.calls_by_id.get(&var_address.call_id).expect("mutate_variable: call not found").get_local(var_address);
        self.check_reference(interner, expected_type, reference);
        self.check_reference(interner, variable.expected_type, reference);
        let old_reference = variable.reference;
        self.decrement_reference_ref_count(
            IObjectReferrerV::VariableToObjectReferrer(crate::testvm::values::VariableToObjectReferrerV { var_addr: var_address, ownership: expected_type.ownership }),
            old_reference);
        self.increment_reference_ref_count(
            IObjectReferrerV::VariableToObjectReferrer(crate::testvm::values::VariableToObjectReferrerV { var_addr: var_address, ownership: expected_type.ownership }),
            reference);
        self.get_current_call(var_address.call_id, |c| c.mutate_local(var_address, reference, expected_type));
        old_reference
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
    pub fn mutate_struct(&mut self, member_address: MemberAddressV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        let MemberAddressV { struct_id: object_id, field_index } = member_address;
        let allocation = self.objects_by_id.objects_by_id.get(&object_id).expect("get: not found");
        match allocation.kind {
            KindV::StructInstance(si) => {
                let members = si.members.get().expect("StructInstance has no members");
                let old_member_reference = members[field_index as usize];
                self.decrement_reference_ref_count(IObjectReferrerV::MemberToObjectReferrer(crate::testvm::values::MemberToObjectReferrerV { member_addr: member_address, ownership: expected_type.ownership }), old_member_reference);
                assert!(si.struct_h.members[field_index as usize].tyype == expected_type);
                si.get_reference_member(field_index);
                si.set_reference_member(self.vivem_bump, field_index, reference);
                self.increment_reference_ref_count(IObjectReferrerV::MemberToObjectReferrer(crate::testvm::values::MemberToObjectReferrerV { member_addr: member_address, ownership: expected_type.ownership }), reference);
                old_member_reference
            }
            _ => panic!("mutate_struct: not a StructInstance"),
        }
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
    pub fn get_reference_from_struct(&self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, address: MemberAddressV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>, target_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        let MemberAddressV { struct_id: object_id, field_index } = address;
        let allocation = self.objects_by_id.objects_by_id.get(&object_id).expect("get: not found");
        match allocation.kind {
            KindV::StructInstance(si) => {
                let members = si.members.get().expect("StructInstance has no members");
                let actual_reference = members[field_index as usize];
                self.check_reference(interner, expected_type, actual_reference);
                self.transmute(actual_reference, expected_type, target_type)
            }
            _ => panic!("get_reference_from_struct: not a StructInstance"),
        }
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
    pub fn get_reference_from_array(&self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, address: ElementAddressV<'v, 'h, 's>, expected_type: CoordH<'s, 'h>, target_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        let ElementAddressV { array_id: object_id, element_index } = address;
        match self.objects_by_id.objects_by_id.get(&object_id).expect("get_reference_from_array: not found").kind {
            KindV::ArrayInstance(ai) => {
                let r#ref = ai.get_element(element_index);
                self.check_reference(interner, expected_type, r#ref);
                self.transmute(r#ref, expected_type, target_type)
            }
            _ => panic!("get_reference_from_array: not an array instance"),
        }
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
        self.contains_live_object_alloc_id(reference.alloc_id())
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
        let m = &self.objects_by_id.objects_by_id;
        let result = match m.get(&alloc_id) {
            None => false,
            Some(alloc) => match alloc.kind {
                KindV::StructInstance(si) => match si.members.get() {
                    None => false,
                    Some(_) => true,
                },
                _ => true,
            },
        };
        result
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
        let m = &self.objects_by_id.objects_by_id;
        let result = m.contains_key(&alloc_id);
        result
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
    pub fn dereference(&self, reference: ReferenceV<'v, 'h, 's>, allow_undead: bool) -> KindV<'v, 'h, 's> {
        if !allow_undead {
            assert!(self.contains_live_object_alloc_id(reference.alloc_id()));
        }
        let m = &self.objects_by_id.objects_by_id;
        let result = m.get(&reference.alloc_id()).expect("dereference: alloc_id not in objects_by_id").kind;
        result
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
    pub fn increment_reference_ref_count(&mut self, referrer: IObjectReferrerV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>) {
        self.increment_object_ref_count(referrer, reference.alloc_id(), reference.ownership == OwnershipH::WeakH);
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
    pub fn decrement_reference_ref_count(&mut self, referrer: IObjectReferrerV<'v, 'h, 's>, reference: ReferenceV<'v, 'h, 's>) {
        self.decrement_object_ref_count(referrer, reference.alloc_id(), reference.ownership == OwnershipH::WeakH);
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
    pub fn destructure_array(&mut self, reference: ReferenceV<'v, 'h, 's>) -> &'v [ReferenceV<'v, 'h, 's>] {
        let allocation = self.dereference(reference, false);
        match allocation {
            KindV::ArrayInstance(ai) => {
                let elements_len = ai.elements.get().len();
                let element_refs_vec: Vec<ReferenceV<'v, 'h, 's>> = (0..elements_len).rev().map(|_index| self.deinitialize_array_element(reference)).collect::<Vec<_>>().into_iter().rev().collect();
                self.vivem_bump.alloc_slice_copy(&element_refs_vec)
            }
            _ => panic!("destructure_array: not an array instance"),
        }
    }
}
/*
Guardian: temp-disable: SPDMX — Per in-file precedent SPDMX temp-disable at heap.rs:769 (destructure): Scala HeapV.dereference has default param allowUndead=false; Rust elided the default per EANODVX, so the explicit false is the Rust adaptation. — FrontendRust/guardian-logs/request-407-1780508030747/hook-407/destructure_array--747.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
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
    pub fn destructure(&mut self, reference: ReferenceV<'v, 'h, 's>) -> &'v [ReferenceV<'v, 'h, 's>] {
        let allocation = self.dereference(reference, false);
        match allocation {
            KindV::StructInstance(s) => {
                let member_refs = s.members.get().expect("destructure: members None");
                for (index, member_ref) in member_refs.iter().enumerate() {
                    self.decrement_reference_ref_count(
                        crate::testvm::values::IObjectReferrerV::MemberToObjectReferrer(crate::testvm::values::MemberToObjectReferrerV {
                            member_addr: crate::testvm::values::MemberAddressV { struct_id: reference.alloc_id(), field_index: index as i32 },
                            ownership: member_ref.ownership,
                        }),
                        *member_ref);
                }
                self.zero(reference);
                self.deallocate_if_no_weak_refs(reference);
                member_refs
            }
            _ => panic!("destructure: not a StructInstance"),
        }
    }
}
/*
Guardian: temp-disable: SPDMX — Calling self.dereference(reference, false) explicitly passes the allowUndead default value because Rust has no default-argument support. Same in-tree precedent: to_von (heap.rs:1662) and check_reference (heap.rs:1429) already use this exact pattern with explicit false / explicit allow_undead arg. Exception B equivalent for default-argument adaptation. — FrontendRust/guardian-logs/request-241-1780412647292/hook-241/destructure--730.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
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
    pub fn zero(&mut self, reference: ReferenceV<'v, 'h, 's>) {
        let m = &mut self.objects_by_id.objects_by_id;
        let allocation = m.get(&reference.alloc_id()).expect("zero: not in objects_by_id");
        assert_eq!(allocation.get_total_ref_count(Some(OwnershipH::OwnH)), 0);
        assert_eq!(allocation.get_total_ref_count(Some(OwnershipH::ImmutableBorrowH)), 0);
        assert_eq!(allocation.get_total_ref_count(Some(OwnershipH::MutableBorrowH)), 0);
        match allocation.kind {
            KindV::StructInstance(si) => si.zero(),
            _ => {}
        }
        {
            use std::io::Write;
            let handle = &mut *self.vivem_dout;
            write!(handle, " o{}zero", reference.alloc_id().num).unwrap();
        }
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
    pub fn deallocate_if_no_weak_refs(&mut self, reference: ReferenceV<'v, 'h, 's>) {
        let m = &mut self.objects_by_id.objects_by_id;
        let allocation = m.get(&reference.alloc_id()).expect("deallocate_if_no_weak_refs: not in objects_by_id");
        if reference.ownership == OwnershipH::OwnH &&
            (allocation.get_total_ref_count(Some(OwnershipH::MutableBorrowH)) + allocation.get_total_ref_count(Some(OwnershipH::ImmutableBorrowH))) > 0 {
            panic!("Constraint violated!");
        }
        if allocation.get_total_ref_count(None) == 0 {
            m.remove(&reference.alloc_id());
            {
                use std::io::Write;
                let handle = &mut *self.vivem_dout;
                write!(handle, " o{}dealloc", reference.alloc_id().num).unwrap();
            }
        }
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
    pub fn increment_object_ref_count(&mut self, pointing_from: IObjectReferrerV<'v, 'h, 's>, alloc_id: AllocationIdV<'v, 'h, 's>, allow_undead: bool) {
        if !allow_undead {
            if !self.contains_live_object_alloc_id(alloc_id) {
                panic!("Trying to increment dead object: {}", alloc_id.num);
            }
        }
        let m = &mut self.objects_by_id.objects_by_id;
        let obj = m.get_mut(&alloc_id).expect("increment_object_ref_count: alloc_id not in objects_by_id");
        obj.increment_ref_count(pointing_from);
        let new_ref_count = obj.get_total_ref_count(None);
        {
            use std::io::Write;
            let handle = &mut *self.vivem_dout;
            write!(handle, " o{}rc{}->{}", alloc_id.num, new_ref_count - 1, new_ref_count).unwrap();
        }
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
    pub fn decrement_object_ref_count(&mut self, pointed_from: IObjectReferrerV<'v, 'h, 's>, alloc_id: AllocationIdV<'v, 'h, 's>, allow_undead: bool) -> i32 {
        if !allow_undead {
            if !self.contains_live_object_alloc_id(alloc_id) {
                panic!("Can't decrement object {}, not in heap!", alloc_id.num);
            }
        }
        let m = &mut self.objects_by_id.objects_by_id;
        let obj = m.get_mut(&alloc_id).expect("decrement_object_ref_count: not in objects_by_id");
        obj.decrement_ref_count(pointed_from);
        let new_ref_count = obj.get_total_ref_count(None);
        {
            use std::io::Write;
            let handle = &mut *self.vivem_dout;
            write!(handle, " o{}rc{}->{}", alloc_id.num, new_ref_count + 1, new_ref_count).unwrap();
        }
        new_ref_count
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
        if reference.ownership == OwnershipH::WeakH {
            assert!(self.contains_live_or_undead_object(reference.alloc_id()));
        } else {
            assert!(self.contains_live_object_alloc_id(reference.alloc_id()));
        }
        let m = &self.objects_by_id.objects_by_id;
        let allocation = m.get(&reference.alloc_id()).expect("get_total_ref_count: not in objects_by_id");
        let result = allocation.get_total_ref_count(None);
        result
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
    pub fn ensure_ref_count(&self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, reference: ReferenceV<'v, 'h, 's>, ownership_filter: Option<&'v [OwnershipH]>, expected_num: i32) {
        assert!(self.contains_live_object_alloc_id(reference.alloc_id()));
        let allocation = self.objects_by_id.get(interner, reference.alloc_id());
        allocation.ensure_ref_count(ownership_filter, expected_num);
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
    pub fn add(&mut self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, ownership: OwnershipH, location: LocationH, kind: KindV<'v, 'h, 's>) -> ReferenceV<'v, 'h, 's> {
        self.objects_by_id.add(interner, ownership, location, kind)
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
        if expected_type == target_type {
            return reference;
        }
        let ReferenceV { actual_kind, seen_as_kind: old_seen_as_type, ownership: old_ownership, location: _old_location, num: object_id } = reference;
        assert_eq!(
            old_ownership == OwnershipH::MutableShareH || old_ownership == OwnershipH::ImmutableShareH,
            target_type.ownership == OwnershipH::MutableShareH || target_type.ownership == OwnershipH::ImmutableShareH,
        );
        if old_seen_as_type.hamut != expected_type.kind {
            panic!("wot");
        }
        ReferenceV {
            actual_kind,
            seen_as_kind: crate::testvm::values::RRKindV { hamut: target_type.kind, _phantom: std::marker::PhantomData },
            ownership: target_type.ownership,
            location: target_type.location,
            num: object_id,
        }
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
    pub fn is_empty(&self) -> bool {
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
    pub fn print_all(&self) {
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
    pub fn check_for_leaks(&mut self) {
        self.objects_by_id.check_for_leaks(self.vivem_dout);
    }
}
/*
  def checkForLeaks(): Unit = {
    objectsById.checkForLeaks()
  }
*/
// mig: fn get_current_call
impl<'v, 'h, 's> HeapV<'v, 'h, 's> {
    // Scala signature: getCurrentCall(callId: CallId): Call returns the Call by reference (Scala mutable shared object).
    // Rust adaptation: CallV is held inside a Cell-wrapped HashMap and contains Cell-of-HashMap fields, so it can't be
    // returned by-value or by-borrow with a clean lifetime. We take the callback-passing shape — caller hands in a
    // closure that operates on `&CallV`, and we hold the RefCell borrow for that scope. Exception-B class (borrow/Cell
    // adaptation forced by Rust ownership).
    pub fn get_current_call<R>(&mut self, expected_call_id: CallIdV<'v, 'h, 's>, f: impl FnOnce(&mut CallV<'v, 'h, 's>) -> R) -> R {
        assert_eq!(*self.call_id_stack.last().expect("call_id_stack empty"), expected_call_id);
        let call = self.calls_by_id.get_mut(&expected_call_id).expect("get_current_call: not in calls_by_id");
        f(call)
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
    pub fn take_argument(&mut self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, call_id: CallIdV<'v, 'h, 's>, argument_index: i32, expected_type: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
        let reference = self.get_current_call(call_id, |c| c.take_argument(argument_index));
        self.check_reference(interner, expected_type, reference);
        self.decrement_reference_ref_count(
            crate::testvm::values::IObjectReferrerV::ArgumentToObjectReferrer(crate::testvm::values::ArgumentToObjectReferrerV {
                argument_id: crate::testvm::values::ArgumentIdV { call_id, index: argument_index },
                ownership: expected_type.ownership,
            }),
            reference); // decrementing because taking it out of arg
        // Now, the register is the only one that has this reference.
        reference
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
    pub fn allocate_transient(&mut self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, ownership: OwnershipH, location: LocationH, kind: KindV<'v, 'h, 's>) -> ReferenceV<'v, 'h, 's> {
        let r#ref = self.add(interner, ownership, location, kind);
        {
            use std::io::Write;
            let handle = &mut *self.vivem_dout;
            write!(handle, " o{}=", r#ref.alloc_id().num).unwrap();
        }
        self.print_kind(kind);
        r#ref
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
    pub fn print_kind(&mut self, kind: KindV<'v, 'h, 's>) {
        use std::io::Write;
        let handle = &mut *self.vivem_dout;
        match kind {
            KindV::Void(_) => write!(handle, "void").unwrap(),
            KindV::Int(int_v) => write!(handle, "{}", int_v.value).unwrap(),
            KindV::Opaque(_) => write!(handle, "opaque").unwrap(),
            KindV::Bool(b) => write!(handle, "{}", b.value).unwrap(),
            KindV::Str(s) => write!(handle, "{}", s.value.0).unwrap(),
            KindV::Float(f) => write!(handle, "{}", f.value).unwrap(),
            KindV::StructInstance(si) => {
                let members = si.members.get().expect("print_kind: StructInstance members None");
                let members_str = members.iter().map(|r| format!("o{}", r.alloc_id().num)).collect::<Vec<_>>().join(", ");
                write!(handle, "{}{{{}}}", si.struct_h.id.fully_qualified_name.0, members_str).unwrap()
            }
            KindV::ArrayInstance(ai) => {
                let elements = ai.elements.get();
                let elements_str = elements.iter().map(|r| format!("o{}", r.alloc_id().num)).collect::<Vec<_>>().join(", ");
                write!(handle, "array:{}:{:?}{{{}}}", ai.capacity, ai.element_type_h, elements_str).unwrap()
            }
        }
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
    pub fn initialize_array_element(&mut self, array_reference: ReferenceV<'v, 'h, 's>, ret: ReferenceV<'v, 'h, 's>) {
        match self.dereference(array_reference, false) {
            KindV::ArrayInstance(a) => {
                self.increment_reference_ref_count(
                    crate::testvm::values::IObjectReferrerV::ElementToObjectReferrer(crate::testvm::values::ElementToObjectReferrerV {
                        element_addr: crate::testvm::values::ElementAddressV { array_id: array_reference.alloc_id(), element_index: a.get_size() },
                        ownership: a.element_type_h.ownership,
                    }),
                    ret);
                a.initialize_element(self.vivem_bump, ret);
            }
            _ => panic!("initialize_array_element: not an ArrayInstance"),
        }
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
    pub fn new_struct(&mut self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, struct_def_h: StructDefinitionH<'s, 'h>, struct_ref_h: CoordH<'s, 'h>, member_references: &'v [ReferenceV<'v, 'h, 's>]) -> ReferenceV<'v, 'h, 's> {
        let instance: &'v crate::testvm::values::StructInstanceV<'v, 'h, 's> = self.vivem_bump.alloc(crate::testvm::values::StructInstanceV {
            struct_h: struct_def_h,
            members: std::cell::Cell::new(Some(member_references)),
        });
        let reference = self.add(interner, struct_ref_h.ownership, struct_ref_h.location, crate::testvm::values::KindV::StructInstance(instance));
        for (index, member_reference) in member_references.iter().enumerate() {
            self.increment_reference_ref_count(
                crate::testvm::values::IObjectReferrerV::MemberToObjectReferrer(crate::testvm::values::MemberToObjectReferrerV {
                    member_addr: crate::testvm::values::MemberAddressV { struct_id: reference.alloc_id(), field_index: index as i32 },
                    ownership: member_reference.ownership,
                }),
                *member_reference);
        }
        {
            use std::io::Write;
            write!(self.vivem_dout, " o{}=", reference.num).unwrap();
        }
        self.print_kind(KindV::StructInstance(instance));
        reference
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
    pub fn new_opaque(&self, opaque_coord_ht: CoordH<'s, 'h>) -> ReferenceV<'v, 'h, 's> {
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
    pub fn deinitialize_array_element(&mut self, array_reference: ReferenceV<'v, 'h, 's>) -> ReferenceV<'v, 'h, 's> {
        let array_instance = match self.dereference(array_reference, false) {
            KindV::ArrayInstance(ai) => ai,
            _ => panic!("deinitialize_array_element: not an array instance"),
        };
        let element_reference = array_instance.deinitialize_element();
        self.decrement_reference_ref_count(
            crate::testvm::values::IObjectReferrerV::ElementToObjectReferrer(crate::testvm::values::ElementToObjectReferrerV {
                element_addr: crate::testvm::values::ElementAddressV { array_id: array_reference.alloc_id(), element_index: array_instance.get_size() },
                ownership: element_reference.ownership,
            }),
            element_reference);
        element_reference
    }
}
/*
Guardian: temp-disable: SPDMX — Per in-file precedent SPDMX temp-disable at heap.rs:769 (destructure) and new sibling at heap.rs:747 (destructure_array): Scala HeapV.dereference has default param allowUndead=false; Rust elided the default per EANODVX, so the explicit false is the Rust adaptation. — FrontendRust/guardian-logs/request-413-1780508192574/hook-413/deinitialize_array_element--1410.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
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
    pub fn add_uninitialized_array(&mut self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, array_definition_th: RuntimeSizedArrayDefinitionHT<'s, 'h>, array_ref_type: CoordH<'s, 'h>, capacity: i32) -> (ReferenceV<'v, 'h, 's>, &'v ArrayInstanceV<'v, 'h, 's>) {
        let instance: &'v ArrayInstanceV<'v, 'h, 's> = self.vivem_bump.alloc(ArrayInstanceV { type_h: array_ref_type, element_type_h: array_definition_th.element_type, capacity, elements: Cell::new(&[]) });
        let reference = self.add(interner, array_ref_type.ownership, array_ref_type.location, KindV::ArrayInstance(instance));
        (reference, instance)
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
    pub fn add_array(&mut self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, array_definition_th: StaticSizedArrayDefinitionHT<'s, 'h>, array_ref_type: CoordH<'s, 'h>, member_refs: &'v [ReferenceV<'v, 'h, 's>]) -> (ReferenceV<'v, 'h, 's>, &'v ArrayInstanceV<'v, 'h, 's>) {
        let instance: &'v ArrayInstanceV<'v, 'h, 's> = self.vivem_bump.alloc(ArrayInstanceV { type_h: array_ref_type, element_type_h: array_definition_th.element_type, capacity: member_refs.len() as i32, elements: Cell::new(member_refs) });
        let reference = self.add(interner, array_ref_type.ownership, array_ref_type.location, KindV::ArrayInstance(instance));
        for (index, member_ref) in member_refs.iter().enumerate() {
            self.increment_reference_ref_count(
                crate::testvm::values::IObjectReferrerV::ElementToObjectReferrer(crate::testvm::values::ElementToObjectReferrerV {
                    element_addr: crate::testvm::values::ElementAddressV { array_id: reference.alloc_id(), element_index: index as i64 },
                    ownership: member_ref.ownership,
                }),
                *member_ref);
        }
        (reference, instance)
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
    pub fn check_reference(&self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, expected_type: CoordH<'s, 'h>, actual_reference: ReferenceV<'v, 'h, 's>) {
        if actual_reference.ownership == OwnershipH::WeakH {
            assert!(self.contains_live_or_undead_object(actual_reference.alloc_id()));
        } else {
            assert!(self.contains_live_object_alloc_id(actual_reference.alloc_id()));
        }
        if (match actual_reference.seen_as_coord().hamut.ownership {
            OwnershipH::ImmutableShareH => OwnershipH::MutableShareH,
            OwnershipH::ImmutableBorrowH => OwnershipH::MutableBorrowH,
            other => other,
        }) != (match expected_type.ownership {
            OwnershipH::ImmutableShareH => OwnershipH::MutableShareH,
            OwnershipH::ImmutableBorrowH => OwnershipH::MutableBorrowH,
            other => other,
        }) {
            panic!("Expected {:?} but was {:?}", expected_type, actual_reference.seen_as_coord().hamut);
        }
        if actual_reference.seen_as_coord().hamut.location != expected_type.location {
            panic!("Expected {:?} but was {:?}", expected_type, actual_reference.seen_as_coord().hamut);
        }
        if actual_reference.seen_as_coord().hamut.kind != expected_type.kind {
            panic!("Expected {:?} but was {:?}", expected_type, actual_reference.seen_as_coord().hamut);
        }
        let actual_kind = self.dereference(actual_reference, actual_reference.ownership == OwnershipH::WeakH);
        self.check_kind(interner, expected_type.kind, actual_kind);
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
    pub fn check_kind(&self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>, expected_type: KindHT<'s, 'h>, actual_kind: KindV<'v, 'h, 's>) {
        match (actual_kind, expected_type) {
            (KindV::Int(actual), KindHT::IntHT(expected)) => {
                if actual.bits != expected.bits {
                    panic!("Expected {:?} but was {:?}", expected, actual);
                }
            }
            (KindV::Opaque(_), KindHT::OpaqueHT(_)) => panic!("check_kind: Opaque — pilot doesn't exercise"),
            (KindV::Void(_), KindHT::VoidHT(_)) => {}
            (KindV::Bool(_), KindHT::BoolHT(_)) => {}
            (KindV::Str(_), KindHT::StrHT(_)) => {}
            (KindV::Float(_), KindHT::FloatHT(_)) => {}
            (KindV::StructInstance(struct_def_h), KindHT::StructHT(struct_ref_h)) => {
                if struct_def_h.struct_h.get_ref(interner) != struct_ref_h {
                    panic!("Expected {:?} but was {:?}", struct_ref_h, struct_def_h.struct_h);
                }
            }
            (KindV::ArrayInstance(type_h_array), array_h @ KindHT::RuntimeSizedArrayHT(_)) => {
                if type_h_array.type_h.kind != array_h {
                    panic!("Expected {:?} but was {:?}", array_h, type_h_array.type_h);
                }
            }
            (KindV::ArrayInstance(type_h_array), array_h @ KindHT::StaticSizedArrayHT(_)) => {
                if type_h_array.type_h.kind != array_h {
                    panic!("Expected {:?} but was {:?}", array_h, type_h_array.type_h);
                }
            }
            (KindV::StructInstance(struct_def_h), ir_h @ KindHT::InterfaceHT(_)) => {
                let interface_h = match ir_h {
                    KindHT::InterfaceHT(i) => i,
                    _ => unreachable!(),
                };
                let struct_implements_interface = struct_def_h.struct_h.edges.iter().any(|e| e.interface == interface_h);
                if !struct_implements_interface {
                    panic!("Struct {:?} doesnt implement interface {:?}", struct_def_h.struct_h.get_ref(interner), interface_h);
                }
            }
            (a, b) => {
                panic!("Mismatch! {:?} is not a {:?}", a, b);
            }
        }
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
    pub fn push_new_stack_frame(&mut self, function_h: &'h PrototypeH<'s, 'h>, args: &'v [ReferenceV<'v, 'h, 's>]) -> CallIdV<'v, 'h, 's> {
        let calls_by_id = &mut self.calls_by_id;
        let call_id_stack = &mut self.call_id_stack;
        assert_eq!(calls_by_id.len(), call_id_stack.len());
        let call_id = CallIdV {
            call_depth: if !call_id_stack.is_empty() { call_id_stack.last().unwrap().call_depth + 1 } else { 0 },
            function: function_h,
            _phantom: std::marker::PhantomData,
        };
        let call = crate::testvm::call::CallV {
            call_id,
            in_args: args,
            args: args.iter().enumerate().map(|(i, r)| (i as i32, Some(*r))).collect(),
            locals: HashMap::new(),
        };
        calls_by_id.insert(call_id, call);
        let mut call_id_stack = call_id_stack;
        call_id_stack.push(call_id);
        assert_eq!(calls_by_id.len(), call_id_stack.len());
        call_id
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
    pub fn pop_stack_frame(&mut self, expected_call_id: CallIdV<'v, 'h, 's>) {
        let calls_by_id = &mut self.calls_by_id;
        let call_id_stack = &mut self.call_id_stack;
        assert_eq!(calls_by_id.len(), call_id_stack.len());
        assert_eq!(*call_id_stack.last().expect("call_id_stack empty"), expected_call_id);
        {
            let call = calls_by_id.get_mut(&expected_call_id).expect("call not found");
            call.prepare_to_die();
        }
        call_id_stack.pop();
        calls_by_id.remove(&expected_call_id);
        assert_eq!(calls_by_id.len(), call_id_stack.len());
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
        match self.dereference(reference, false) {
            KindV::Void(_) => crate::von::ast::IVonData::Object(crate::von::ast::VonObject { tyype: "void".to_string(), id: None, members: vec![] }),
            KindV::Int(v) => crate::von::ast::IVonData::Int(crate::von::ast::VonInt { value: v.value }),
            KindV::Float(v) => crate::von::ast::IVonData::Float(crate::von::ast::VonFloat { value: v.value }),
            KindV::Bool(v) => crate::von::ast::IVonData::Bool(crate::von::ast::VonBool { value: v.value }),
            KindV::Str(v) => crate::von::ast::IVonData::Str(crate::von::ast::VonStr { value: v.value.0.to_string() }),
            KindV::ArrayInstance(_) => panic!("to_von: ArrayInstance — pilot doesn't exercise"),
            KindV::StructInstance(_) => panic!("to_von: StructInstance — pilot doesn't exercise"),
            KindV::Opaque(_) => panic!("to_von: Opaque — pilot doesn't exercise"),
        }
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
    VariableAddressV { call_id, local }
}
/*
  def getVarAddress(callId: CallId, local: Local) = {
    VariableAddressV(callId, local)
  }
}

*/
