use std::marker::PhantomData;

use crate::instantiating::ast::names::{IdI, IInterfaceNameI, IStructNameI, RuntimeSizedArrayNameI, StaticSizedArrayNameI};
use crate::instantiating::instantiating_interner::MustIntern;

/*
package dev.vale.instantiating.ast

import dev.vale._
import dev.vale.postparsing.IImpreciseNameS
import dev.vale.typing.ast._
import dev.vale.typing.env._
import dev.vale.typing.names._
import dev.vale.highertyping._
import dev.vale.postparsing._
import dev.vale.typing._
import dev.vale.typing.ast._
import dev.vale.typing.templata._
import dev.vale.typing.types._

import scala.collection.immutable.List
*/
// mig: enum OwnershipI
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum OwnershipI {
  ImmutableShare,
  MutableShare,
  Own,
  Weak,
  ImmutableBorrow,
  MutableBorrow,
}
// mig: impl OwnershipI
/*
sealed trait OwnershipI {
}
*/
// mig: case object ImmutableShareI
/*
// Instantiator turns BorrowI into MutableBorrowI and ImmutableBorrowI, see HRALII
case object ImmutableShareI extends OwnershipI {
  override def toString: String = "immshare"
}
*/
// mig: case object MutableShareI
/*
// Instantiator turns ShareI into MutableShareI and ImmutableShareI, see HRALII
// Ironic because shared things are immutable, this is rather referring to the refcount.
case object MutableShareI extends OwnershipI {
  override def toString: String = "mutshare"
}
*/
// mig: case object OwnI
/*
case object OwnI extends OwnershipI {
  override def toString: String = "own"
}
*/
// mig: case object WeakI
/*
case object WeakI extends OwnershipI {
  override def toString: String = "weak"
}
*/
// mig: case object ImmutableBorrowI
/*
// Instantiator turns BorrowI into MutableBorrowI and ImmutableBorrowI, see HRALII
case object ImmutableBorrowI extends OwnershipI {
  override def toString: String = "immborrow"
}
*/
// mig: case object MutableBorrowI
/*
// Instantiator turns BorrowI into MutableBorrowI and ImmutableBorrowI, see HRALII
case object MutableBorrowI extends OwnershipI {
  override def toString: String = "mutborrow"
}
*/
// mig: enum MutabilityI
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MutabilityI {
  Mutable,
  Immutable,
}
// mig: impl MutabilityI
/*
sealed trait MutabilityI {
}
*/
// mig: case object MutableI
/*
case object MutableI extends MutabilityI {
  override def toString: String = "mut"
}
*/
// mig: case object ImmutableI
/*
case object ImmutableI extends MutabilityI {
  override def toString: String = "imm"
}
*/
// mig: enum VariabilityI
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum VariabilityI {
  Final,
  Varying,
}
// mig: impl VariabilityI
/*
sealed trait VariabilityI {
}
*/
// mig: case object FinalI
/*
case object FinalI extends VariabilityI {
  override def toString: String = "final"
}
*/
// mig: case object VaryingI
/*
case object VaryingI extends VariabilityI {
  override def toString: String = "vary"
}
*/
// mig: enum LocationI
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum LocationI {
  Inline,
  Yonder,
}
// mig: impl LocationI
/*
sealed trait LocationI {
}
*/
// mig: case object InlineI
/*
case object InlineI extends LocationI {
  override def toString: String = "inl"
}
*/
// mig: case object YonderI
/*
case object YonderI extends LocationI {
  override def toString: String = "heap"
}
*/
// mig: enum IRegionsModeI
// Per architect Slab 16a: kept as pure compile-time phantom marker. The three
// ZST variants below (sI/nI/cI) implement a private `Sealed` trait so only this
// module can introduce new region modes — same closed-set guarantee as Scala's
// `sealed trait IRegionsModeI`.
mod region_mode_sealed {
    pub trait Sealed {}
}
pub trait IRegionsModeIT: region_mode_sealed::Sealed {}

#[allow(non_camel_case_types)]
pub enum IRegionsModeI {
  sI(sI),
  nI(nI),
  cI(cI),
}
// mig: impl IRegionsModeI
/*
sealed trait IRegionsModeI
*/
// mig: struct sI
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
#[allow(non_camel_case_types)]
pub struct sI;
impl region_mode_sealed::Sealed for sI {}
impl IRegionsModeIT for sI {}
// mig: impl sI
/*
// See CCFCTS, these need to have zero members. If we need to have members, we'll need to stop
// casting from collapsed to subjective ASTs.
class sI() extends IRegionsModeI
*/
// Region modes are covariant in Scala (`nI <: sI`); Rust erases that covariance (same rationale as
// +T-erasure). Per CCFCTS the modes are inert zero-member tags, so `nI`/`cI` alias `sI` to make ASTs
// across modes interchangeable (e.g. passing `PrototypeI<nI>` where `PrototypeI<sI>` is expected).
// mig: struct nI
#[allow(non_camel_case_types)]
pub type nI = sI;
// mig: impl nI
/*
class nI() extends sI // Stands for new. Serves as a starting point for a new instantiation.
*/
// mig: struct cI
#[allow(non_camel_case_types)]
pub type cI = sI;
// mig: impl cI
/*
class cI() extends IRegionsModeI

object CoordI {
*/
// mig: fn void
impl<'s, 'i, R> CoordI<'s, 'i, R> where 's: 'i {
  pub fn void() -> CoordI<'s, 'i, R> { panic!("Unimplemented: void"); }
}
/*
  def void[R <: IRegionsModeI]: CoordI[R] = CoordI[R](MutableShareI, VoidIT())
*/
/*
}
*/
// mig: struct CoordI
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CoordI<'s, 'i, R> where 's: 'i {
  pub ownership: OwnershipI,
  pub kind: KindIT<'s, 'i, R>,
}
// mig: impl CoordI
/*
case class CoordI[+R <: IRegionsModeI](
  ownership: OwnershipI,
  kind: KindIT[R])  {

  vpass()

  kind match {
    case IntIT(_) | BoolIT() | StrIT() | FloatIT() | VoidIT() | NeverIT(_) => {
//      // We don't want any ImmutableShareH, it's better to only ever have one ownership for
//      // primitives.
//      vassert(ownership == MutableShareI)
    }
    case RuntimeSizedArrayIT(IdI(_, _, RuntimeSizedArrayNameI(_, RawArrayNameI(_, _, arrRegion)))) =>
    case StaticSizedArrayIT(IdI(_, _, StaticSizedArrayNameI(_, _, _, RawArrayNameI(_, _, arrRegion)))) =>
    case StructIT(IdI(_, _, localName)) =>
    case InterfaceIT(IdI(_, _, localName)) =>
    case _ =>
  }
  if (ownership == OwnI) {
    // See CSHROOR for why we don't assert this.
    // vassert(permission == Readwrite)
  }
  (ownership, kind) match {
    case (MutableBorrowI, StrIT()) => vwat()
    case (MutableBorrowI, IntIT(_)) => vwat()
    case _ =>
  }
}
*/
// mig: enum KindIT
// Per Scala parity: primitives (NeverIT, VoidIT, IntIT, BoolIT, StrIT, FloatIT)
// are TFITCX Value-type and held inline; the 4 compound payloads
// (StaticSizedArrayIT, RuntimeSizedArrayIT, StructIT, InterfaceIT) are arena-
// interned and held by `&'i` ref (see @WVSBIZ).
/// Polyvalue (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum KindIT<'s, 'i, R> where 's: 'i {
  NeverIT(NeverIT<R>),
  VoidIT(VoidIT<R>),
  IntIT(IntIT<R>),
  BoolIT(BoolIT<R>),
  StrIT(StrIT<R>),
  FloatIT(FloatIT<R>),
  StaticSizedArrayIT(&'i StaticSizedArrayIT<'s, 'i, R>),
  RuntimeSizedArrayIT(&'i RuntimeSizedArrayIT<'s, 'i, R>),
  StructIT(&'i StructIT<'s, 'i, R>),
  InterfaceIT(&'i InterfaceIT<'s, 'i, R>),
}
// mig: impl KindIT
/*
sealed trait KindIT[+R <: IRegionsModeI] {
  // Note, we don't have a mutability: Mutability in here because this Kind
  // should be enough to uniquely identify a type, and no more.
  // We can always get the mutability for a struct from the coutputs.
*/
// mig: fn is_primitive
impl<'s, 'i, R> KindIT<'s, 'i, R> where 's: 'i {
  pub fn is_primitive(&self) -> bool { panic!("Unimplemented: is_primitive"); }
}
/*
  def isPrimitive: Boolean
*/
// mig: fn expect_citizen
impl<'s, 'i, R> KindIT<'s, 'i, R> where 's: 'i {
  pub fn expect_citizen(&self) -> ICitizenIT<'s, 'i, R> { panic!("Unimplemented: expect_citizen"); }
}
/*
  def expectCitizen(): ICitizenIT[R] = {
    this match {
      case c : ICitizenIT[R] => c
      case _ => vfail()
    }
  }
*/
// mig: fn expect_interface
impl<'s, 'i, R> KindIT<'s, 'i, R> where 's: 'i {
  pub fn expect_interface(&self) -> &'i InterfaceIT<'s, 'i, R> {
    match self {
      KindIT::InterfaceIT(c) => c,
      _ => panic!("expect_interface: not an interface"),
    }
  }
}
/*
  def expectInterface(): InterfaceIT[R] = {
    this match {
      case c @ InterfaceIT(_) => c
      case _ => vfail()
    }
  }
*/
// mig: fn expect_struct
impl<'s, 'i, R> KindIT<'s, 'i, R> where 's: 'i {
  pub fn expect_struct(&self) -> &'i StructIT<'s, 'i, R> { panic!("Unimplemented: expect_struct"); }
}
/*
  def expectStruct(): StructIT[R] = {
    this match {
      case c @ StructIT(_) => c
      case _ => vfail()
    }
  }
}
*/
// mig: struct NeverIT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct NeverIT<R> {
  pub from_break: bool,
  pub _marker: PhantomData<R>,
}
// mig: impl NeverIT
/*
// like Scala's Nothing. No instance of this can ever happen.
case class NeverIT[+R <: IRegionsModeI](
  // True if this Never came from a break.
  // While will have to know about this; if IT's a Never from a ret, IT should
  // propagate IT, but if its body is a break never, the while produces a void.
  // See BRCOBS.
  fromBreak: Boolean
) extends KindIT[R] {
  override def isPrimitive: Boolean = true
}
*/
// mig: struct VoidIT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct VoidIT<R> {
  pub _marker: PhantomData<R>,
}
// mig: impl VoidIT
/*
// Mostly for interoperability with extern functions
case class VoidIT[+R <: IRegionsModeI]() extends KindIT[R] {
  override def isPrimitive: Boolean = true
}
*/
// mig: struct IntIT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IntIT<R> {
  pub bits: i32,
  pub _marker: PhantomData<R>,
}
// mig: impl IntIT
/*
case class IntIT[+R <: IRegionsModeI](bits: Int) extends KindIT[R] {
  override def isPrimitive: Boolean = true
}
*/
// mig: struct BoolIT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BoolIT<R> {
  pub _marker: PhantomData<R>,
}
// mig: impl BoolIT
/*
case class BoolIT[+R <: IRegionsModeI]() extends KindIT[R] {
  override def isPrimitive: Boolean = true
}
*/
// mig: struct StrIT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StrIT<R> {
  pub _marker: PhantomData<R>,
}
// mig: impl StrIT
/*
case class StrIT[+R <: IRegionsModeI]() extends KindIT[R] {
  override def isPrimitive: Boolean = false
}
*/
// mig: struct FloatIT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FloatIT<R> {
  pub _marker: PhantomData<R>,
}
// mig: impl FloatIT
/*
case class FloatIT[+R <: IRegionsModeI]() extends KindIT[R] {
  override def isPrimitive: Boolean = true
}

object contentsStaticSizedArrayIT {
*/
// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<StaticSizedArrayIT<R>> for ...` or inline match.)
/*
  def unapply[R <: IRegionsModeI](ssa: StaticSizedArrayIT[R]):
  Option[(Long, MutabilityI, VariabilityI, CoordTemplataI[R], RegionTemplataI[R])] = {
    val IdI(_, _, StaticSizedArrayNameI(_, size, variability, RawArrayNameI(mutability, coord, selfRegion))) = ssa.name
    Some((size, mutability, variability, coord, selfRegion))
  }
*/
/*
}
*/
// mig: struct StaticSizedArrayIT
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayIT<'s, 'i, R> where 's: 'i {
  pub name: IdI<'s, 'i, R>,
  pub _must_intern: MustIntern,
}

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayITValI<'s, 'i, R> where 's: 'i {
  pub name: IdI<'s, 'i, R>,
}
// mig: impl StaticSizedArrayIT
impl<'s, 'i, R: Copy> StaticSizedArrayIT<'s, 'i, R> where 's: 'i {
  pub fn mutability(self) -> MutabilityI {
    match self.name.local_name {
      crate::instantiating::ast::names::INameI::StaticSizedArray(n) => n.arr.mutability,
      _ => panic!("StaticSizedArrayIT::mutability: name.local_name is not StaticSizedArrayNameI"),
    }
  }
  pub fn element_type(self) -> crate::instantiating::ast::templata::CoordTemplataI<'s, 'i, R> {
    match self.name.local_name {
      crate::instantiating::ast::names::INameI::StaticSizedArray(n) => n.arr.element_type,
      _ => panic!("StaticSizedArrayIT::element_type: name.local_name is not StaticSizedArrayNameI"),
    }
  }
  pub fn size(self) -> i64 {
    match self.name.local_name {
      crate::instantiating::ast::names::INameI::StaticSizedArray(n) => n.size,
      _ => panic!("StaticSizedArrayIT::size: name.local_name is not StaticSizedArrayNameI"),
    }
  }
  pub fn variability(self) -> VariabilityI {
    match self.name.local_name {
      crate::instantiating::ast::names::INameI::StaticSizedArray(n) => n.variability,
      _ => panic!("StaticSizedArrayIT::variability: name.local_name is not StaticSizedArrayNameI"),
    }
  }
}
/*
case class StaticSizedArrayIT[+R <: IRegionsModeI](
  name: IdI[R, StaticSizedArrayNameI[R]]
) extends KindIT[R] {
  vassert(name.initSteps.isEmpty)
  override def isPrimitive: Boolean = false
  def mutability: MutabilityI = name.localName.arr.mutability
  def elementType = name.localName.arr.elementType
  def size = name.localName.size
  def variability = name.localName.variability
}

object contentsRuntimeSizedArrayIT {
*/
// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<RuntimeSizedArrayIT<R>> for ...` or inline match.)
/*
  def unapply[R <: IRegionsModeI](rsa: RuntimeSizedArrayIT[R]):
  Option[(MutabilityI, CoordTemplataI[R], RegionTemplataI[R])] = {
    val IdI(_, _, RuntimeSizedArrayNameI(_, RawArrayNameI(mutability, coord, selfRegion))) = rsa.name
    Some((mutability, coord, selfRegion))
  }
*/
/*
}
*/
// mig: struct RuntimeSizedArrayIT
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayIT<'s, 'i, R> where 's: 'i {
  pub name: IdI<'s, 'i, R>,
  pub _must_intern: MustIntern,
}

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayITValI<'s, 'i, R> where 's: 'i {
  pub name: IdI<'s, 'i, R>,
}
// mig: impl RuntimeSizedArrayIT
impl<'s, 'i, R: Copy> RuntimeSizedArrayIT<'s, 'i, R> where 's: 'i {
  pub fn mutability(self) -> MutabilityI {
    match self.name.local_name {
      crate::instantiating::ast::names::INameI::RuntimeSizedArray(n) => n.arr.mutability,
      _ => panic!("RuntimeSizedArrayIT::mutability: name.local_name is not RuntimeSizedArrayNameI"),
    }
  }
  pub fn element_type(self) -> crate::instantiating::ast::templata::CoordTemplataI<'s, 'i, R> {
    match self.name.local_name {
      crate::instantiating::ast::names::INameI::RuntimeSizedArray(n) => n.arr.element_type,
      _ => panic!("RuntimeSizedArrayIT::element_type: name.local_name is not RuntimeSizedArrayNameI"),
    }
  }
}
/*
case class RuntimeSizedArrayIT[+R <: IRegionsModeI](
  name: IdI[R, RuntimeSizedArrayNameI[R]]
) extends KindIT[R] {
  override def isPrimitive: Boolean = false
  def mutability = name.localName.arr.mutability
  def elementType = name.localName.arr.elementType

//  name.localName.arr.selfRegion match {
//    case RegionTemplata(false) => vwat()
//    case _ =>
//  }
}

object ICitizenIT {
*/
// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<ICitizenIT<R>> for ...` or inline match.)
/*
  def unapply[R <: IRegionsModeI](self: ICitizenIT[R]): Option[IdI[R, ICitizenNameI[R]]] = {
    Some(self.id)
  }
*/
/*
}
*/
// mig: enum ISubKindIT
/// Polyvalue (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ISubKindIT<'s, 'i, R> where 's: 'i {
  StructIT(&'i StructIT<'s, 'i, R>),
  InterfaceIT(&'i InterfaceIT<'s, 'i, R>),
}
// mig: impl ISubKindIT
/*
// Structs, interfaces, and placeholders
sealed trait ISubKindIT[+R <: IRegionsModeI] extends KindIT[R] {
  def id: IdI[R, ISubKindNameI[R]]
}
*/
// mig: enum ICitizenIT
/// Polyvalue (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ICitizenIT<'s, 'i, R> where 's: 'i {
  StructIT(&'i StructIT<'s, 'i, R>),
  InterfaceIT(&'i InterfaceIT<'s, 'i, R>),
}
// mig: impl ICitizenIT
// mig: fn id (Scala `def id: IdI[R, ICitizenNameI[R]]` on trait)
impl<'s, 'i, R: Copy> ICitizenIT<'s, 'i, R> where 's: 'i {
    pub fn id(&self) -> IdI<'s, 'i, R> {
        match self {
            ICitizenIT::StructIT(s) => s.id,
            ICitizenIT::InterfaceIT(i) => i.id,
        }
    }
}
/*
sealed trait ICitizenIT[+R <: IRegionsModeI] extends ISubKindIT[R] {
  def id: IdI[R, ICitizenNameI[R]]
}
*/
// mig: struct StructIT
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructIT<'s, 'i, R> where 's: 'i {
  pub id: IdI<'s, 'i, R>,
  pub _must_intern: MustIntern,
}

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructITValI<'s, 'i, R> where 's: 'i {
  pub id: IdI<'s, 'i, R>,
}
// mig: impl StructIT
/*
// These should only be made by StructCompiler, which puts the definition and bounds into coutputs at the same time
case class StructIT[+R <: IRegionsModeI](id: IdI[R, IStructNameI[R]]) extends ICitizenIT[R] {
  override def isPrimitive: Boolean = false
  (id.initSteps.lastOption, id.localName) match {
    case (Some(StructTemplateNameI(_)), StructNameI(_, _)) => vfail()
    case _ =>
  }
}
*/
// mig: struct InterfaceIT
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceIT<'s, 'i, R> where 's: 'i {
  pub id: IdI<'s, 'i, R>,
  pub _must_intern: MustIntern,
}

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceITValI<'s, 'i, R> where 's: 'i {
  pub id: IdI<'s, 'i, R>,
}
// mig: impl InterfaceIT
/*
case class InterfaceIT[+R <: IRegionsModeI](id: IdI[R, IInterfaceNameI[R]]) extends ICitizenIT[R] {
  override def isPrimitive: Boolean = false
  (id.initSteps.lastOption, id.localName) match {
    case (Some(InterfaceTemplateNameI(_)), InterfaceNameI(_, _)) => vfail()
    case _ =>
  }
}
*/
// -- Union enums for the Kind-payload interning family ---------------------
// Per architect Slab 16b: kind payloads dispatch through a tagged-union pair.
// R stays phantom — the actual per-(type×region-mode) family separation lives
// in the interner (3 HashMaps per logical type, one per region mode).

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum InternedKindPayloadValI<'s, 'i, R> where 's: 'i {
  StructIT(StructITValI<'s, 'i, R>),
  InterfaceIT(InterfaceITValI<'s, 'i, R>),
  StaticSizedArrayIT(StaticSizedArrayITValI<'s, 'i, R>),
  RuntimeSizedArrayIT(RuntimeSizedArrayITValI<'s, 'i, R>),
}

/// Polyvalue (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum InternedKindPayloadI<'s, 'i, R> where 's: 'i {
  StructIT(&'i StructIT<'s, 'i, R>),
  InterfaceIT(&'i InterfaceIT<'s, 'i, R>),
  StaticSizedArrayIT(&'i StaticSizedArrayIT<'s, 'i, R>),
  RuntimeSizedArrayIT(&'i RuntimeSizedArrayIT<'s, 'i, R>),
}
