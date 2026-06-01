use std::cell::Cell;
use std::collections::HashMap;
use std::marker::PhantomData;
use crate::interner::StrI;
use crate::final_ast::types::{KindHT, CoordH, LocationH, OwnershipH, OpaqueHT};
use crate::final_ast::ast::{PrototypeH, StructDefinitionH};
use crate::final_ast::instructions::Local;

/*
package dev.vale.testvm

import dev.vale.finalast.{BoolHT, FloatHT, IntHT, KindHT, Local, LocationH, OwnershipH, PrototypeH, CoordH, StrHT, StructDefinitionH, VoidHT}
import dev.vale.{vassert, vcheck, vfail}
import dev.vale.finalast._
import dev.vale.vimpl

// RR = Runtime Result. Don't use these to determine behavior, just use
// these to check that things are as we expect.
*/
// mig: struct RRReferenceV
/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct RRReferenceV<'v, 'h, 's>
where 's: 'h, 'h: 'v,
{
  pub hamut: CoordH<'s, 'h>,
  pub _phantom: std::marker::PhantomData<&'v ()>,
}
/*
case class RRReference(hamut: CoordH[KindHT]) {
  val hash = runtime.ScalaRunTime._hashCode(this);
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for RRReferenceV<'v, 'h, 's>` below.)
/*
override def hashCode(): Int = hash;  }
*/
// mig: struct RRKindV<'v, 'h, 's>
/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct RRKindV<'v, 'h, 's>
where 's: 'h, 'h: 'v,
{
  pub hamut: KindHT<'s, 'h>,
  pub _phantom: std::marker::PhantomData<&'v ()>,
}
/*
case class RRKind(hamut: KindHT) {
  val hash = runtime.ScalaRunTime._hashCode(this);
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for RRKindV<'v, 'h, 's>` below.)
/*
override def hashCode(): Int = hash; }
*/
// mig: struct AllocationV<'v, 'h, 's>
/// Temporary state
pub struct AllocationV<'v, 'h, 's> {
  pub reference: ReferenceV<'v, 'h, 's>,
  pub kind: KindV<'v, 'h, 's>,
  pub referrers: Cell<HashMap<IObjectReferrerV<'v, 'h, 's>, i32>>,
}
/*
class Allocation(
    val reference: ReferenceV, // note that this cannot change
    val kind: KindV // note that this cannot change
) {
  private var referrers = Map[IObjectReferrer, Int]()
*/
// mig: fn id
impl<'v, 'h, 's> AllocationV<'v, 'h, 's> {
  pub fn id(&self) -> AllocationIdV<'v, 'h, 's> {
    panic!("Unimplemented: id");
  }
}
/*
  def id = reference.allocId
*/
// mig: fn increment_ref_count
impl<'v, 'h, 's> AllocationV<'v, 'h, 's> {
  pub fn increment_ref_count(&self, referrer: IObjectReferrerV<'v, 'h, 's>) {
    panic!("Unimplemented: increment_ref_count");
  }
}
/*
  def incrementRefCount(referrer: IObjectReferrer): Unit = {
    if (kind == VoidV) {
      // Void has no RC
      return
    }
    referrer match {
      case RegisterToObjectReferrer(_, _) => {
        // We can have multiple of these, thats fine
      }
      case _ => {
        if (referrers.contains(referrer)) {
          vfail("nooo")
        }
      }
    }
    referrers = referrers + (referrer -> (referrers.getOrElse(referrer, 0) + 1))
  }
*/
// mig: fn decrement_ref_count
impl<'v, 'h, 's> AllocationV<'v, 'h, 's> {
  pub fn decrement_ref_count(&self, referrer: IObjectReferrerV<'v, 'h, 's>) {
    panic!("Unimplemented: decrement_ref_count");
  }
}
/*
  def decrementRefCount(referrer: IObjectReferrer): Unit = {
    if (kind == VoidV) {
      // Void has no RC
      return
    }
    if (!referrers.contains(referrer)) {
      vfail("nooooo\n" + referrer + "\nnot in:\n" + referrers)
    }
    referrers = referrers + (referrer -> (referrers(referrer) - 1))
    if (referrers(referrer) == 0) {
      referrers = referrers - referrer
      vassert(!referrers.contains(referrer))
    }
  }
*/
// mig: fn get_ref_count
impl<'v, 'h, 's> AllocationV<'v, 'h, 's> {
  pub fn get_ref_count(&self) -> i32 {
    panic!("Unimplemented: get_ref_count");
  }
}
/*
  def getRefCount(): Int = {
    if (kind == VoidV) {
      // Pretend void has 1 RC so nobody ever deallocates it
      return 1
    }
    referrers
      .toVector
      .map(_._2)
      .sum
  }
*/
// mig: fn ensure_ref_count
impl<'v, 'h, 's> AllocationV<'v, 'h, 's> {
  pub fn ensure_ref_count(&self, maybe_ownership_filter: Option<&'v [OwnershipH]>, expected_num: i32) {
    panic!("Unimplemented: ensure_ref_count");
  }
}
/*
  def ensureRefCount(maybeOwnershipFilter: Option[Set[OwnershipH]], expectedNum: Int): Unit = {
    if (kind == VoidV) {
      // Void has no RC
      return
    }
    var referrers = this.referrers
    referrers =
      maybeOwnershipFilter match {
        case None => referrers
        case Some(ownershipFilter) => referrers.filter({ case (key, _) => ownershipFilter.contains(key.ownership)})
      }
    val matchingReferrers = referrers.toVector.map(_._2)
    vcheck(
      matchingReferrers.size == expectedNum,
      "Expected " +
        expectedNum + " of " +
        maybeOwnershipFilter.map(_.toString + " ").getOrElse("") +
        "but was " + matchingReferrers.size + ":\n" +
        matchingReferrers.mkString("\n"),
      ConstraintViolatedException)
  }
*/
// mig: fn print_refs
impl<'v, 'h, 's> AllocationV<'v, 'h, 's> {
  pub fn print_refs(&self) {
    panic!("Unimplemented: print_refs");
  }
}
/*
  def printRefs() = {
    if (getTotalRefCount(None) > 0) {
      println("o" + reference.allocId.num + ": " + referrers.mkString(" "))
    }
  }
*/
// mig: fn get_total_ref_count
impl<'v, 'h, 's> AllocationV<'v, 'h, 's> {
  pub fn get_total_ref_count(&self, maybe_ownership_filter: Option<OwnershipH>) -> i32 {
    panic!("Unimplemented: get_total_ref_count");
  }
}
/*
  def getTotalRefCount(maybeOwnershipFilter: Option[OwnershipH]): Int = {
    if (kind == VoidV) {
      // Pretend void has 1 RC so nobody ever deallocates it
      return 1
    }
    maybeOwnershipFilter match {
      case None => referrers.size
      case Some(ownershipFilter) => referrers.keys.count(_.ownership == ownershipFilter)
    }
  }
*/
// mig: fn finalize
impl<'v, 'h, 's> AllocationV<'v, 'h, 's> {
  pub fn finalize(&self) {
    panic!("Unimplemented: finalize");
  }
}
/*
  override def finalize(): Unit = {
//    vassert(referrers.isEmpty)
  }
*/
// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<Wide> for Narrow` or inline match.)
/*
  def unapply(arg: Allocation): Option[KindV] = Some(kind)
}

object Allocation {
*/
// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<Wide> for Narrow` or inline match.)
/*
  def unapply(arg: Allocation): Option[KindV] = {
    Some(arg.kind)
  }
}
*/
// mig: enum KindV<'v, 'h, 's>
/// Temporary state
pub enum KindV<'v, 'h, 's> {
  Void(&'v VoidV),
  Int(&'v IntV<'v, 'h, 's>),
  Bool(&'v BoolV<'v, 'h, 's>),
  Float(&'v FloatV<'v, 'h, 's>),
  Str(&'v StrV<'v, 'h, 's>),
  Opaque(&'v OpaqueV<'v, 'h, 's>),
  StructInstance(&'v StructInstanceV<'v, 'h, 's>),
  ArrayInstance(&'v ArrayInstanceV<'v, 'h, 's>),
}
/*
sealed trait KindV {
*/
// mig: fn tyype
impl<'v, 'h, 's> KindV<'v, 'h, 's> {
  pub fn tyype(&self) -> RRKindV<'v, 'h, 's> {
    panic!("Unimplemented: tyype");
  }
}
/*
  def tyype: RRKind
}
*/
// mig: enum PrimitiveKindV<'v, 'h, 's>
/// Temporary state
pub enum PrimitiveKindV<'v, 'h, 's> {
  Void(&'v VoidV),
  Int(&'v IntV<'v, 'h, 's>),
  Bool(&'v BoolV<'v, 'h, 's>),
  Float(&'v FloatV<'v, 'h, 's>),
  Str(&'v StrV<'v, 'h, 's>),
  Opaque(&'v OpaqueV<'v, 'h, 's>),
}
/*
sealed trait PrimitiveKindV extends KindV
*/
// mig: VoidV
pub struct VoidV;
/*
case object VoidV extends PrimitiveKindV {
*/
// mig: fn tyype
impl VoidV {
  pub fn tyype<'v, 'h, 's>(&self) -> RRKindV<'v, 'h, 's> where 's: 'h, 'h: 'v, {
    panic!("Unimplemented: tyype_void");
  }
}
/*
  override def tyype = RRKind(VoidHT())
}
*/
// mig: struct IntV
/// Temporary state
pub struct IntV<'v, 'h, 's>
where 's: 'h, 'h: 'v,
{
  pub value: i64,
  pub bits: i32,
  pub _phantom: PhantomData<(&'v (), &'h (), &'s ())>,
}
/*
case class IntV(value: Long, bits: Int) extends PrimitiveKindV {
*/
// mig: fn tyype
impl<'v, 'h, 's> IntV<'v, 'h, 's> {
  pub fn tyype(&self) -> RRKindV<'v, 'h, 's> {
    panic!("Unimplemented: tyype_int");
  }
}
/*
  override def tyype = RRKind(IntHT(bits))
}
*/
// mig: struct BoolV
/// Temporary state
pub struct BoolV<'v, 'h, 's>
where 's: 'h, 'h: 'v,
{
  pub value: bool,
  pub _phantom: PhantomData<(&'v (), &'h (), &'s ())>,
}
/*
case class BoolV(value: Boolean) extends PrimitiveKindV {
*/
// mig: fn tyype
impl<'v, 'h, 's> BoolV<'v, 'h, 's> {
  pub fn tyype(&self) -> RRKindV<'v, 'h, 's> {
    panic!("Unimplemented: tyype_bool");
  }
}
/*
  override def tyype = RRKind(BoolHT())
}
*/
// mig: struct FloatV
/// Temporary state
pub struct FloatV<'v, 'h, 's>
where 's: 'h, 'h: 'v,
{
  pub value: f64,
  pub _phantom: PhantomData<(&'v (), &'h (), &'s ())>,
}
/*
case class FloatV(value: Double) extends PrimitiveKindV {
*/
// mig: fn tyype
impl<'v, 'h, 's> FloatV<'v, 'h, 's> {
  pub fn tyype(&self) -> RRKindV<'v, 'h, 's> {
    panic!("Unimplemented: tyype_float");
  }
}
/*
  override def tyype = RRKind(FloatHT())
}
*/
// mig: struct StrV
/// Temporary state
pub struct StrV<'v, 'h, 's>
where 's: 'h, 'h: 'v,
{
  pub value: StrI<'s>,
  pub _phantom: PhantomData<(&'v (), &'h ())>,
}
/*
case class StrV(value: String) extends PrimitiveKindV {
*/
// mig: fn tyype
impl<'v, 'h, 's> StrV<'v, 'h, 's> {
  pub fn tyype(&self) -> RRKindV<'v, 'h, 's> {
    panic!("Unimplemented: tyype_str");
  }
}
/*
  override def tyype = RRKind(StrHT())
}
*/
// mig: struct OpaqueV
/// Temporary state
pub struct OpaqueV<'v, 'h, 's>
where 's: 'h, 'h: 'v,
{
  pub opaque_ht: OpaqueHT<'s, 'h>,
  pub _phantom: PhantomData<&'v ()>,
}
/*
case class OpaqueV(opaqueHT: OpaqueHT) extends PrimitiveKindV {
*/
// mig: fn tyype
impl<'v, 'h, 's> OpaqueV<'v, 'h, 's> {
  pub fn tyype(&self) -> RRKindV<'v, 'h, 's> {
    panic!("Unimplemented: tyype_opaque");
  }
}
/*
  override def tyype = RRKind(opaqueHT)
}
*/
// mig: struct StructInstanceV<'v, 'h, 's>
/// Temporary state
pub struct StructInstanceV<'v, 'h, 's>
where 's: 'h, 'h: 'v,
{
  pub struct_h: StructDefinitionH<'s, 'h>,
  pub members: Cell<Option<&'v [ReferenceV<'v, 'h, 's>]>>,
}
/*
case class StructInstanceV(
    structH: StructDefinitionH,
    private var members: Option[Vector[ReferenceV]]
) extends KindV {
  vassert(members.get.size == structH.members.size)
*/
// mig: fn tyype
impl<'v, 'h, 's> StructInstanceV<'v, 'h, 's> {
  pub fn tyype(&self) -> RRKindV<'v, 'h, 's> {
    panic!("Unimplemented: tyype_struct_instance");
  }
}
/*
  override def tyype = RRKind(structH.getRef)
*/
// mig: fn get_reference_member
impl<'v, 'h, 's> StructInstanceV<'v, 'h, 's> {
  pub fn get_reference_member(&self, index: i32) -> ReferenceV<'v, 'h, 's> {
    panic!("Unimplemented: get_reference_member");
  }
}
/*
  def getReferenceMember(index: Int) = {
    (structH.members(index).tyype, members.get(index)) match {
      case (_, ref) => ref
    }
  }
*/
// mig: fn set_reference_member
impl<'v, 'h, 's> StructInstanceV<'v, 'h, 's> {
  pub fn set_reference_member(&self, index: i32, reference: ReferenceV<'v, 'h, 's>) {
    panic!("Unimplemented: set_reference_member");
  }
}
/*
  def setReferenceMember(index: Int, reference: ReferenceV) = {
    members = Some(members.get.updated(index, reference))
  }
*/
// mig: fn zero
impl<'v, 'h, 's> StructInstanceV<'v, 'h, 's> {
  pub fn zero(&self) {
    panic!("Unimplemented: zero");
  }
}
/*
  // Zeros out the memory, which happens when the owning ref goes away.
  // The allocation is still alive until we let go of the last weak ref
  // too.
  def zero(): Unit = {
    members = None
  }
}
*/
// mig: struct ArrayInstanceV<'v, 'h, 's>
/// Temporary state
pub struct ArrayInstanceV<'v, 'h, 's> {
  pub type_h: CoordH<'s, 'h>,
  pub element_type_h: CoordH<'s, 'h>,
  pub capacity: i32,
  pub elements: Cell<&'v [ReferenceV<'v, 'h, 's>]>,
}
/*
case class ArrayInstanceV(
    typeH: CoordH[KindHT],
    elementTypeH: CoordH[KindHT],
    capacity: Int,
    private var elements: Vector[ReferenceV]
) extends KindV {
*/
// mig: fn tyype
impl<'v, 'h, 's> ArrayInstanceV<'v, 'h, 's> {
  pub fn tyype(&self) -> RRKindV<'v, 'h, 's> {
    panic!("Unimplemented: tyype_array_instance");
  }
}
/*
  override def tyype = RRKind(typeH.kind)
*/
// mig: fn get_element
impl<'v, 'h, 's> ArrayInstanceV<'v, 'h, 's> {
  pub fn get_element(&self, index: i64) -> ReferenceV<'v, 'h, 's> {
    panic!("Unimplemented: get_element");
  }
}
/*
  def getElement(index: Long): ReferenceV = {
    if (index < 0 || index >= elements.size) {
      throw PanicException();
    }
    elements(index.toInt)
  }
*/
// mig: fn set_element
impl<'v, 'h, 's> ArrayInstanceV<'v, 'h, 's> {
  pub fn set_element(&self, index: i64, ref_: ReferenceV<'v, 'h, 's>) {
    panic!("Unimplemented: set_element");
  }
}
/*
  def setElement(index: Long, ref: ReferenceV) = {
    if (index < 0 || index >= elements.size) {
      throw PanicException();
    }
    elements = elements.updated(index.toInt, ref)
  }
*/
// mig: fn initialize_element
impl<'v, 'h, 's> ArrayInstanceV<'v, 'h, 's> {
  pub fn initialize_element(&self, ref_: ReferenceV<'v, 'h, 's>) {
    panic!("Unimplemented: initialize_element");
  }
}
/*
  def initializeElement(ref: ReferenceV) = {
    vassert(elements.size < capacity)
    elements = elements :+ ref
  }
*/
// mig: fn deinitialize_element
impl<'v, 'h, 's> ArrayInstanceV<'v, 'h, 's> {
  pub fn deinitialize_element(&self) -> ReferenceV<'v, 'h, 's> {
    panic!("Unimplemented: deinitialize_element");
  }
}
/*
  def deinitializeElement() = {
    vassert(elements.nonEmpty)
    val ref = elements.last
    elements = elements.slice(0, elements.size - 1)
    ref
  }
*/
// mig: fn get_size
impl<'v, 'h, 's> ArrayInstanceV<'v, 'h, 's> {
  pub fn get_size(&self) -> i64 {
    panic!("Unimplemented: get_size");
  }
}
/*
  def getSize() = {
    elements.size
  }
}
*/
// mig: struct AllocationIdV<'v, 'h, 's>
/// Temporary state
pub struct AllocationIdV<'v, 'h, 's> {
  pub tyype: RRKindV<'v, 'h, 's>,
  pub num: i32,
}
/*
case class AllocationId(tyype: RRKind, num: Int) {
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for AllocationIdV<'v, 'h, 's>` below.)
/*
  override def hashCode(): Int = num
}
*/
// mig: struct ReferenceV<'v, 'h, 's>
/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ReferenceV<'v, 'h, 's> {
  pub actual_kind: RRKindV<'v, 'h, 's>,
  pub seen_as_kind: RRKindV<'v, 'h, 's>,
  pub ownership: OwnershipH,
  pub location: LocationH,
  pub num: i32,
}
/*
case class ReferenceV(
  // actualType and seenAsType will be different in the case of interface reference.
  // Otherwise they'll be the same.

  // What is the actual type of what we're pointing to (as opposed to an interface).
  // If we have a Car reference to a Civic, then this will be Civic.
  actualKind: RRKind,
  // What do we see the type as. If we have a Car reference to a Civic, then this will be Car.
  seenAsKind: RRKind,

  ownership: OwnershipH,

  location: LocationH,

  // Negative number means it's an empty struct (like void).
  num: Int
) {
*/
// mig: fn alloc_id
impl<'v, 'h, 's> ReferenceV<'v, 'h, 's> {
  pub fn alloc_id(&self) -> AllocationIdV<'v, 'h, 's> {
    panic!("Unimplemented: alloc_id");
  }
}
/*
  def allocId = AllocationId(RRKind(actualKind.hamut), num)
  val actualCoord: RRReference = RRReference(CoordH(ownership, location, actualKind.hamut))
  val seenAsCoord: RRReference = RRReference(CoordH(ownership, location, seenAsKind.hamut))
}
*/
// mig: enum IObjectReferrerV<'v, 'h, 's>
/// Temporary state
pub enum IObjectReferrerV<'v, 'h, 's> {
  VariableToObjectReferrer(&'v VariableToObjectReferrerV<'v, 'h, 's>),
  MemberToObjectReferrer(&'v MemberToObjectReferrerV<'v, 'h, 's>),
  ElementToObjectReferrer(&'v ElementToObjectReferrerV<'v, 'h, 's>),
  RegisterToObjectReferrer(&'v RegisterToObjectReferrerV<'v, 'h, 's>),
  RegisterHoldToObjectReferrer(&'v RegisterHoldToObjectReferrerV<'v, 'h, 's>),
  ArgumentToObjectReferrer(&'v ArgumentToObjectReferrerV<'v, 'h, 's>),
}
/*
sealed trait IObjectReferrer {
*/
// mig: fn ownership
impl<'v, 'h, 's> IObjectReferrerV<'v, 'h, 's> {
  pub fn ownership(&self) -> OwnershipH {
    panic!("Unimplemented: ownership");
  }
}
/*
  def ownership: OwnershipH
}
*/
// mig: struct VariableToObjectReferrerV
/// Temporary state
pub struct VariableToObjectReferrerV<'v, 'h, 's> {
  pub var_addr: VariableAddressV<'v, 'h, 's>,
  pub ownership: OwnershipH,
}
/*
case class VariableToObjectReferrer(varAddr: VariableAddressV, ownership: OwnershipH) extends IObjectReferrer {
  val hash = runtime.ScalaRunTime._hashCode(this);
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for VariableToObjectReferrerV` below.)
/*
override def hashCode(): Int = hash;  }
*/
// mig: struct MemberToObjectReferrerV
/// Temporary state
pub struct MemberToObjectReferrerV<'v, 'h, 's> {
  pub member_addr: MemberAddressV<'v, 'h, 's>,
  pub ownership: OwnershipH,
}
/*
case class MemberToObjectReferrer(memberAddr: MemberAddressV, ownership: OwnershipH) extends IObjectReferrer {
  val hash = runtime.ScalaRunTime._hashCode(this);
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for MemberToObjectReferrerV` below.)
/*
override def hashCode(): Int = hash;  }
*/
// mig: struct ElementToObjectReferrerV
/// Temporary state
pub struct ElementToObjectReferrerV<'v, 'h, 's> {
  pub element_addr: ElementAddressV<'v, 'h, 's>,
  pub ownership: OwnershipH,
}
/*
case class ElementToObjectReferrer(elementAddr: ElementAddressV, ownership: OwnershipH) extends IObjectReferrer {
  val hash = runtime.ScalaRunTime._hashCode(this);
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ElementToObjectReferrerV` below.)
/*
override def hashCode(): Int = hash;  }
*/
// mig: struct RegisterToObjectReferrerV
/// Temporary state
pub struct RegisterToObjectReferrerV<'v, 'h, 's> {
  pub call_id: CallIdV<'v, 'h, 's>,
  pub ownership: OwnershipH,
}
/*
case class RegisterToObjectReferrer(callId: CallId, ownership: OwnershipH) extends IObjectReferrer {
  val hash = runtime.ScalaRunTime._hashCode(this);
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for RegisterToObjectReferrerV` below.)
/*
override def hashCode(): Int = hash;  }
*/
// mig: struct RegisterHoldToObjectReferrerV
/// Temporary state
pub struct RegisterHoldToObjectReferrerV<'v, 'h, 's> {
  pub expression_id: ExpressionIdV<'v, 'h, 's>,
  pub ownership: OwnershipH,
}
/*
// This is us holding onto something during a while loop or array generator call, so the called functions dont eat them and deallocate them
case class RegisterHoldToObjectReferrer(expressionId: ExpressionId, ownership: OwnershipH) extends IObjectReferrer {
  val hash = runtime.ScalaRunTime._hashCode(this);
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for RegisterHoldToObjectReferrerV` below.)
/*
override def hashCode(): Int = hash;  }
//case class ResultToObjectReferrer(callId: CallId) extends IObjectReferrer {
  //val hash = runtime.ScalaRunTime._hashCode(this);
//override def hashCode(): Int = hash;  }
*/
// mig: struct ArgumentToObjectReferrerV
/// Temporary state
pub struct ArgumentToObjectReferrerV<'v, 'h, 's> {
  pub argument_id: ArgumentIdV<'v, 'h, 's>,
  pub ownership: OwnershipH,
}
/*
case class ArgumentToObjectReferrer(argumentId: ArgumentId, ownership: OwnershipH) extends IObjectReferrer {
  val hash = runtime.ScalaRunTime._hashCode(this);
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ArgumentToObjectReferrerV` below.)
/*
override def hashCode(): Int = hash;  }
*/
// mig: struct VariableAddressV<'v, 'h, 's>
/// Temporary state
pub struct VariableAddressV<'v, 'h, 's>
where 's: 'h, 'h: 'v,
{
  pub call_id: CallIdV<'v, 'h, 's>,
  pub local: Local<'s, 'h>,
}
/*
case class VariableAddressV(callId: CallId, local: Local) {
*/
// mig: fn to_string
impl<'v, 'h, 's> VariableAddressV<'v, 'h, 's> {
  pub fn to_string(&self) -> StrI<'s> {
    panic!("Unimplemented: to_string_variable_address");
  }
}
/*
  override def toString: String = "*v:" + callId + "#v" + local.id.number
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for VariableAddressV<'v, 'h, 's>` below.)
/*
  override def equals(obj: Any): Boolean = {
    obj match {
      case VariableAddressV(thatCallId, thatLocal) => {
        callId == thatCallId && local.id == thatLocal.id
      }
      case _ => false
    }
  }
}
*/
// mig: struct MemberAddressV<'v, 'h, 's>
/// Temporary state
pub struct MemberAddressV<'v, 'h, 's> {
  pub struct_id: AllocationIdV<'v, 'h, 's>,
  pub field_index: i32,
}
/*
case class MemberAddressV(structId: AllocationId, fieldIndex: Int) {
*/
// mig: fn to_string
impl<'v, 'h, 's> MemberAddressV<'v, 'h, 's> {
  pub fn to_string(&self) -> StrI<'s> {
    panic!("Unimplemented: to_string_member_address");
  }
}
/*
  override def toString: String = "*o:" + structId.num + "." + fieldIndex
}
*/
// mig: struct ElementAddressV<'v, 'h, 's>
/// Temporary state
pub struct ElementAddressV<'v, 'h, 's> {
  pub array_id: AllocationIdV<'v, 'h, 's>,
  pub element_index: i64,
}
/*
case class ElementAddressV(arrayId: AllocationId, elementIndex: Long) {
*/
// mig: fn to_string
impl<'v, 'h, 's> ElementAddressV<'v, 'h, 's> {
  pub fn to_string(&self) -> StrI<'s> {
    panic!("Unimplemented: to_string_element_address");
  }
}
/*
  override def toString: String = "*o:" + arrayId.num + "." + elementIndex
}

// Used in tracking reference counts/maps.
*/
// mig: struct CallIdV<'v, 'h, 's>
/// Temporary state
pub struct CallIdV<'v, 'h, 's>
where 's: 'h, 'h: 'v,
{
  pub call_depth: i32,
  pub function: &'h PrototypeH<'s, 'h>,
  pub _phantom: PhantomData<&'v ()>,
}
/*
case class CallId(callDepth: Int, function: PrototypeH) {
*/
// mig: fn to_string
impl<'v, 'h, 's> CallIdV<'v, 'h, 's> {
  pub fn to_string(&self) -> StrI<'s> {
    panic!("Unimplemented: to_string_call_id");
  }
}
/*
  override def toString: String = "ƒ" + callDepth + "/" + (function.id.shortenedName)
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for CallIdV<'v, 'h, 's>` below.)
/*
  override def hashCode(): Int = callDepth + function.id.shortenedName.hashCode
}
//case class RegisterId(blockId: BlockId, lineInBlock: Int) {
  //val hash = runtime.ScalaRunTime._hashCode(this);
//override def hashCode(): Int = hash;  }
*/
// mig: struct ArgumentIdV<'v, 'h, 's>
/// Temporary state
pub struct ArgumentIdV<'v, 'h, 's> {
  pub call_id: CallIdV<'v, 'h, 's>,
  pub index: i32,
}
/*
case class ArgumentId(callId: CallId, index: Int) {
  val hash = runtime.ScalaRunTime._hashCode(this);
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ArgumentIdV<'v, 'h, 's>` below.)
/*
override def hashCode(): Int = hash;  }
*/
// mig: struct VariableV<'v, 'h, 's>
/// Temporary state
pub struct VariableV<'v, 'h, 's> {
  pub id: VariableAddressV<'v, 'h, 's>,
  pub reference: Cell<ReferenceV<'v, 'h, 's>>,
  pub expected_type: CoordH<'s, 'h>,
}
/*
case class VariableV(
    id: VariableAddressV,
    var reference: ReferenceV,
    expectedType: CoordH[KindHT]) {
}
*/
// mig: struct ExpressionIdV<'v, 'h, 's>
/// Temporary state
pub struct ExpressionIdV<'v, 'h, 's> {
  pub call_id: CallIdV<'v, 'h, 's>,
  pub path: &'v [i32],
}
/*
case class ExpressionId(
  callId: CallId,
  path: Vector[Int]
) {
*/
// mig: fn add_step
impl<'v, 'h, 's> ExpressionIdV<'v, 'h, 's> {
  pub fn add_step(&self, i: i32) -> ExpressionIdV<'v, 'h, 's> {
    panic!("Unimplemented: add_step");
  }
}
/*
  def addStep(i: Int): ExpressionId = ExpressionId(callId, path :+ i)
}
*/
// mig: enum RegisterV<'v, 'h, 's>
/// Temporary state
pub enum RegisterV<'v, 'h, 's> {
  ReferenceRegister(&'v ReferenceRegisterV<'v, 'h, 's>),
}
/*
sealed trait RegisterV {
*/
// mig: fn expect_reference_register
impl<'v, 'h, 's> RegisterV<'v, 'h, 's> {
  pub fn expect_reference_register(&self) -> ReferenceRegisterV<'v, 'h, 's> {
    panic!("Unimplemented: expect_reference_register");
  }
}
/*
  def expectReferenceRegister() = {
    this match {
      case rr @ ReferenceRegisterV(reference) => {
        rr
      }
    }
  }
}
*/
// mig: struct ReferenceRegisterV
/// Temporary state
pub struct ReferenceRegisterV<'v, 'h, 's> {
  pub reference: ReferenceV<'v, 'h, 's>,
}
/*
case class ReferenceRegisterV(reference: ReferenceV) extends RegisterV {
  val hash = runtime.ScalaRunTime._hashCode(this);
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for ReferenceRegisterV` below.)
/*
override def hashCode(): Int = hash;  }

*/
// mig: struct VivemPanicV
/// Temporary state
pub struct VivemPanicV<'v, 'h, 's>
where 's: 'h, 'h: 'v,
{
  pub message: StrI<'s>,
  pub _phantom: PhantomData<(&'v (), &'h ())>,
}
/*
case class VivemPanic(message: String) extends Exception
*/
