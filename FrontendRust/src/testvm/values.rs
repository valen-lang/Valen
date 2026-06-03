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
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
  pub referrers: HashMap<IObjectReferrerV<'v, 'h, 's>, i32>,
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
  pub fn increment_ref_count(&mut self, referrer: IObjectReferrerV<'v, 'h, 's>) {
    if matches!(self.kind, KindV::Void(_)) {
      return;
    }
    let referrers = &mut self.referrers;
    match referrer {
      IObjectReferrerV::RegisterToObjectReferrer(_) => {
        // We can have multiple of these, thats fine
      }
      _ => {
        if referrers.contains_key(&referrer) {
          panic!("nooo");
        }
      }
    }
    let current = *referrers.get(&referrer).unwrap_or(&0);
    referrers.insert(referrer, current + 1);
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
  pub fn decrement_ref_count(&mut self, referrer: IObjectReferrerV<'v, 'h, 's>) {
    if matches!(self.kind, KindV::Void(_)) {
      return;
    }
    let referrers = &mut self.referrers;
    if !referrers.contains_key(&referrer) {
      panic!("nooooo");
    }
    let new_count = *referrers.get(&referrer).unwrap() - 1;
    referrers.insert(referrer, new_count);
    if new_count == 0 {
      referrers.remove(&referrer);
      assert!(!referrers.contains_key(&referrer));
    }
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
    if matches!(self.kind, KindV::Void(_)) {
      // Void has no RC
      return;
    }
    let referrers: Vec<(&IObjectReferrerV<'v, 'h, 's>, &i32)> = match maybe_ownership_filter {
      None => self.referrers.iter().collect(),
      Some(ownership_filter) => self.referrers.iter().filter(|(key, _)| ownership_filter.contains(&key.ownership())).collect(),
    };
    let matching_referrers: Vec<i32> = referrers.iter().map(|(_, v)| **v).collect();
    if matching_referrers.len() as i32 != expected_num {
      panic!("Expected {} of {}but was {}:\n{:?}",
        expected_num,
        maybe_ownership_filter.map(|of| format!("{:?} ", of)).unwrap_or_default(),
        matching_referrers.len(),
        matching_referrers);
    }
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
  pub fn print_refs(&self, _vivem_dout: &mut crate::testvm::vivem::PrintStream) {
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
    if matches!(self.kind, KindV::Void(_)) {
      return 1;
    }
    let referrers = &self.referrers;
    let result = match maybe_ownership_filter {
      None => referrers.len() as i32,
      Some(ownership_filter) => referrers.keys().filter(|k| k.ownership() == ownership_filter).count() as i32,
    };
    result
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
#[derive(Copy, Clone)]
pub enum KindV<'v, 'h, 's> {
  Void(VoidV),
  Int(IntV<'v, 'h, 's>),
  Bool(BoolV<'v, 'h, 's>),
  Float(FloatV<'v, 'h, 's>),
  Str(StrV<'v, 'h, 's>),
  Opaque(OpaqueV<'v, 'h, 's>),
  StructInstance(&'v StructInstanceV<'v, 'h, 's>),
  ArrayInstance(&'v ArrayInstanceV<'v, 'h, 's>),
}
/*
sealed trait KindV {
*/
// mig: fn tyype
impl<'v, 'h, 's> KindV<'v, 'h, 's> where 's: 'h, 'h: 'v {
  pub fn tyype(&self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>) -> RRKindV<'v, 'h, 's> {
    match self {
      KindV::Void(v) => v.tyype(interner),
      KindV::Int(v) => v.tyype(interner),
      KindV::Bool(v) => v.tyype(interner),
      KindV::Float(v) => v.tyype(interner),
      KindV::Str(v) => v.tyype(interner),
      KindV::Opaque(v) => v.tyype(interner),
      KindV::StructInstance(v) => v.tyype(interner),
      KindV::ArrayInstance(v) => v.tyype(interner),
    }
  }
}
/*
  def tyype: RRKind
}
*/
// mig: enum PrimitiveKindV<'v, 'h, 's>
/// Temporary state
#[derive(Copy, Clone)]
pub enum PrimitiveKindV<'v, 'h, 's> {
  Void(VoidV),
  Int(IntV<'v, 'h, 's>),
  Bool(BoolV<'v, 'h, 's>),
  Float(FloatV<'v, 'h, 's>),
  Str(StrV<'v, 'h, 's>),
  Opaque(OpaqueV<'v, 'h, 's>),
}
/*
sealed trait PrimitiveKindV extends KindV
*/
// Scala uses subtype polymorphism (`PrimitiveKindV extends KindV`); per SSTREX
// the Rust port uses flat sister enums, so the subset→superset conversion is
// expressed as an explicit `From` impl.
impl<'v, 'h, 's> From<PrimitiveKindV<'v, 'h, 's>> for KindV<'v, 'h, 's> {
  fn from(p: PrimitiveKindV<'v, 'h, 's>) -> Self {
    match p {
      PrimitiveKindV::Void(v) => KindV::Void(v),
      PrimitiveKindV::Int(v) => KindV::Int(v),
      PrimitiveKindV::Bool(v) => KindV::Bool(v),
      PrimitiveKindV::Float(v) => KindV::Float(v),
      PrimitiveKindV::Str(v) => KindV::Str(v),
      PrimitiveKindV::Opaque(v) => KindV::Opaque(v),
    }
  }
}
/* Guardian: disable-all */
// mig: VoidV
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct VoidV;
/*
case object VoidV extends PrimitiveKindV {
*/
// mig: fn tyype
impl VoidV {
  pub fn tyype<'v, 'h, 's>(&self, _interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>) -> RRKindV<'v, 'h, 's> where 's: 'h, 'h: 'v, {
    RRKindV { hamut: KindHT::VoidHT(crate::final_ast::types::VoidHT), _phantom: PhantomData }
  }
}
/*
  override def tyype = RRKind(VoidHT())
}
*/
// mig: struct IntV
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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
  pub fn tyype(&self, _interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>) -> RRKindV<'v, 'h, 's> {
    RRKindV { hamut: KindHT::IntHT(crate::final_ast::types::IntHT { bits: self.bits }), _phantom: PhantomData }
  }
}
/*
  override def tyype = RRKind(IntHT(bits))
}
*/
// mig: struct BoolV
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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
  pub fn tyype(&self, _interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>) -> RRKindV<'v, 'h, 's> {
    RRKindV { hamut: KindHT::BoolHT(crate::final_ast::types::BoolHT), _phantom: PhantomData }
  }
}
/*
  override def tyype = RRKind(BoolHT())
}
*/
// mig: struct FloatV
/// Temporary state
#[derive(Copy, Clone, PartialEq, Debug)]
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
  pub fn tyype(&self, _interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>) -> RRKindV<'v, 'h, 's> {
    RRKindV { hamut: KindHT::FloatHT(crate::final_ast::types::FloatHT), _phantom: PhantomData }
  }
}
/*
  override def tyype = RRKind(FloatHT())
}
*/
// mig: struct StrV
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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
  pub fn tyype(&self, _interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>) -> RRKindV<'v, 'h, 's> {
    RRKindV { hamut: KindHT::StrHT(crate::final_ast::types::StrHT), _phantom: PhantomData }
  }
}
/*
  override def tyype = RRKind(StrHT())
}
*/
// mig: struct OpaqueV
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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
  pub fn tyype(&self, _interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>) -> RRKindV<'v, 'h, 's> {
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
  pub fn tyype(&self, interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>) -> RRKindV<'v, 'h, 's> {
    RRKindV { hamut: KindHT::StructHT(self.struct_h.get_ref(interner)), _phantom: PhantomData }
  }
}
/*
  override def tyype = RRKind(structH.getRef)
*/
// mig: fn get_reference_member
impl<'v, 'h, 's> StructInstanceV<'v, 'h, 's> {
  pub fn get_reference_member(&self, index: i32) -> ReferenceV<'v, 'h, 's> {
    let members = self.members.get().expect("StructInstance has no members");
    let (_tyype, r#ref) = (self.struct_h.members[index as usize].tyype, members[index as usize]);
    r#ref
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
  pub fn set_reference_member(&self, vivem_bump: &'v bumpalo::Bump, index: i32, reference: ReferenceV<'v, 'h, 's>) {
    let mut new_members: Vec<ReferenceV<'v, 'h, 's>> = self.members.get().expect("StructInstance has no members").to_vec();
    new_members[index as usize] = reference;
    self.members.set(Some(vivem_bump.alloc_slice_copy(&new_members)));
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
    self.members.set(None);
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
  pub fn tyype(&self, _interner: &crate::simplifying::hammer_interner::HammerInterner<'s, 'h>) -> RRKindV<'v, 'h, 's> {
    RRKindV { hamut: self.type_h.kind, _phantom: PhantomData }
  }
}
/*
  override def tyype = RRKind(typeH.kind)
*/
// mig: fn get_element
impl<'v, 'h, 's> ArrayInstanceV<'v, 'h, 's> {
  pub fn get_element(&self, index: i64) -> ReferenceV<'v, 'h, 's> {
    let elements = self.elements.get();
    if index < 0 || index as usize >= elements.len() {
      panic!("PanicException");
    }
    elements[index as usize]
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
    let elements = self.elements.get();
    assert!(!elements.is_empty());
    let r#ref = elements[elements.len() - 1];
    self.elements.set(&elements[0..elements.len() - 1]);
    r#ref
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
    self.elements.get().len() as i64
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
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
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
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
    AllocationIdV { tyype: RRKindV { hamut: self.actual_kind.hamut, _phantom: PhantomData }, num: self.num }
  }
}
// mig: fn actual_coord (val actualCoord — Scala synthesized field)
impl<'v, 'h, 's> ReferenceV<'v, 'h, 's> {
  pub fn actual_coord(&self) -> RRReferenceV<'v, 'h, 's> {
    RRReferenceV {
      hamut: crate::final_ast::types::CoordH {
        ownership: self.ownership,
        location: self.location,
        kind: self.actual_kind.hamut,
      },
      _phantom: PhantomData,
    }
  }
}
// mig: fn seen_as_coord (val seenAsCoord — Scala synthesized field)
impl<'v, 'h, 's> ReferenceV<'v, 'h, 's> {
  pub fn seen_as_coord(&self) -> RRReferenceV<'v, 'h, 's> {
    RRReferenceV {
      hamut: crate::final_ast::types::CoordH {
        ownership: self.ownership,
        location: self.location,
        kind: self.seen_as_kind.hamut,
      },
      _phantom: PhantomData,
    }
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
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub enum IObjectReferrerV<'v, 'h, 's> {
  VariableToObjectReferrer(VariableToObjectReferrerV<'v, 'h, 's>),
  MemberToObjectReferrer(MemberToObjectReferrerV<'v, 'h, 's>),
  ElementToObjectReferrer(ElementToObjectReferrerV<'v, 'h, 's>),
  RegisterToObjectReferrer(RegisterToObjectReferrerV<'v, 'h, 's>),
  RegisterHoldToObjectReferrer(RegisterHoldToObjectReferrerV<'v, 'h, 's>),
  ArgumentToObjectReferrer(ArgumentToObjectReferrerV<'v, 'h, 's>),
}
/*
sealed trait IObjectReferrer {
*/
// mig: fn ownership
impl<'v, 'h, 's> IObjectReferrerV<'v, 'h, 's> {
  pub fn ownership(&self) -> OwnershipH {
    match self {
      IObjectReferrerV::VariableToObjectReferrer(r) => r.ownership,
      IObjectReferrerV::MemberToObjectReferrer(r) => r.ownership,
      IObjectReferrerV::ElementToObjectReferrer(r) => r.ownership,
      IObjectReferrerV::RegisterToObjectReferrer(r) => r.ownership,
      IObjectReferrerV::RegisterHoldToObjectReferrer(r) => r.ownership,
      IObjectReferrerV::ArgumentToObjectReferrer(r) => r.ownership,
    }
  }
}
/*
  def ownership: OwnershipH
}
*/
// mig: struct VariableToObjectReferrerV
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct VariableAddressV<'v, 'h, 's>
where 's: 'h, 'h: 'v,
{
  pub call_id: CallIdV<'v, 'h, 's>,
  pub local: Local<'s, 'h>,
}
/*
case class VariableAddressV(callId: CallId, local: Local) {
*/
// mig: fn to_string (realized-by-impl Display)
// (Realized by `impl Display for VariableAddressV<'v, 'h, 's>` below.)
/*
  override def toString: String = "*v:" + callId + "#v" + local.id.number
*/
impl<'v, 'h, 's> std::fmt::Display for VariableAddressV<'v, 'h, 's> where 's: 'h, 'h: 'v {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "*v:{}#v{}", self.call_id, self.local.id.number)
  }
}
/* Guardian: disable-all */
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
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct MemberAddressV<'v, 'h, 's> {
  pub struct_id: AllocationIdV<'v, 'h, 's>,
  pub field_index: i32,
}
/*
case class MemberAddressV(structId: AllocationId, fieldIndex: Int) {
*/
// mig: fn to_string
impl<'v, 'h, 's> MemberAddressV<'v, 'h, 's> {
  pub fn to_string(&self) -> String {
    format!("*o:{}.{}", self.struct_id.num, self.field_index)
  }
}
/*
  override def toString: String = "*o:" + structId.num + "." + fieldIndex
}
*/
// mig: struct ElementAddressV<'v, 'h, 's>
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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
// Rust adaptation: Scala's `case class CallId` gets `toString` autoboxed into
// string-concat via `+`. The Rust port realizes that as `impl Display`, mirroring
// the Scala `toString` body line-for-line. The `to_string(&self) -> StrI<'s>`
// method above stays panic-stubbed until a caller needs the interned form.
impl<'v, 'h, 's> std::fmt::Display for CallIdV<'v, 'h, 's> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "ƒ{}/{}", self.call_depth, self.function.id.shortened_name.0)
  }
}
/* Guardian: disable-all */
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
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
#[derive(Clone)]
pub struct VariableV<'v, 'h, 's> {
  pub id: VariableAddressV<'v, 'h, 's>,
  pub reference: ReferenceV<'v, 'h, 's>,
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
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
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
  pub fn add_step(&self, bump: &'v bumpalo::Bump, i: i32) -> ExpressionIdV<'v, 'h, 's> {
    let old_len = self.path.len();
    let new_path: &'v mut [i32] = bump.alloc_slice_fill_with(old_len + 1, |idx| {
        if idx < old_len { self.path[idx] } else { i }
    });
    ExpressionIdV { call_id: self.call_id, path: new_path }
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
