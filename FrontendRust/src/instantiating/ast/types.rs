use std::marker::PhantomData;

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
pub enum OwnershipI {
  ImmutableShareI(()),
  MutableShareI(()),
  OwnI(()),
  WeakI(()),
  ImmutableBorrowI(()),
  MutableBorrowI(()),
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
pub enum MutabilityI {
  MutableI(()),
  ImmutableI(()),
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
pub enum VariabilityI {
  FinalI(()),
  VaryingI(()),
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
pub enum LocationI {
  InlineI(()),
  YonderI(()),
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
#[allow(non_camel_case_types)]
pub struct sI;
// mig: impl sI
/*
// See CCFCTS, these need to have zero members. If we need to have members, we'll need to stop
// casting from collapsed to subjective ASTs.
class sI() extends IRegionsModeI
*/
// mig: struct nI
#[allow(non_camel_case_types)]
pub struct nI;
// mig: impl nI
/*
class nI() extends sI // Stands for new. Serves as a starting point for a new instantiation.
*/
// mig: struct cI
#[allow(non_camel_case_types)]
pub struct cI;
// mig: impl cI
/*
class cI() extends IRegionsModeI

object CoordI {
*/
// mig: fn void
impl<'t, R> CoordI<'t, R> {
  pub fn void<R>() -> CoordI<'t, R> { panic!("Unimplemented: void"); }
}
/*
  def void[R <: IRegionsModeI]: CoordI[R] = CoordI[R](MutableShareI, VoidIT())
*/
/*
}
*/
// mig: struct CoordI
pub struct CoordI<'t, R> {
  pub ownership: OwnershipI,
  pub kind: KindIT<'t, R>,
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
pub enum KindIT<'t, R> {
  NeverIT(&'t NeverIT<R>),
  VoidIT(&'t VoidIT<R>),
  IntIT(&'t IntIT<R>),
  BoolIT(&'t BoolIT<R>),
  StrIT(&'t StrIT<R>),
  FloatIT(&'t FloatIT<R>),
  StaticSizedArrayIT(&'t StaticSizedArrayIT<R>),
  RuntimeSizedArrayIT(&'t RuntimeSizedArrayIT<R>),
  StructIT(&'t StructIT<R>),
  InterfaceIT(&'t InterfaceIT<R>),
}
// mig: impl KindIT
/*
sealed trait KindIT[+R <: IRegionsModeI] {
  // Note, we don't have a mutability: Mutability in here because this Kind
  // should be enough to uniquely identify a type, and no more.
  // We can always get the mutability for a struct from the coutputs.
*/
// mig: fn is_primitive
impl<'t, R> KindIT<'t, R> {
  pub fn is_primitive(&self) -> bool { panic!("Unimplemented: is_primitive"); }
}
/*
  def isPrimitive: Boolean
*/
// mig: fn expect_citizen
impl<'t, R> KindIT<'t, R> {
  pub fn expect_citizen(&self) -> ICitizenIT<'t, R> { panic!("Unimplemented: expect_citizen"); }
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
impl<'t, R> KindIT<'t, R> {
  pub fn expect_interface(&self) -> InterfaceIT<R> { panic!("Unimplemented: expect_interface"); }
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
impl<'t, R> KindIT<'t, R> {
  pub fn expect_struct(&self) -> StructIT<R> { panic!("Unimplemented: expect_struct"); }
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
pub struct NeverIT<R> {
  pub from_break: bool,
  _phantom: PhantomData<R>,
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
pub struct VoidIT<R> {
  _phantom: PhantomData<R>,
}
// mig: impl VoidIT
/*
// Mostly for interoperability with extern functions
case class VoidIT[+R <: IRegionsModeI]() extends KindIT[R] {
  override def isPrimitive: Boolean = true
}
*/
// mig: struct IntIT
pub struct IntIT<R> {
  pub bits: i32,
  _phantom: PhantomData<R>,
}
// mig: impl IntIT
/*
case class IntIT[+R <: IRegionsModeI](bits: Int) extends KindIT[R] {
  override def isPrimitive: Boolean = true
}
*/
// mig: struct BoolIT
pub struct BoolIT<R> {
  _phantom: PhantomData<R>,
}
// mig: impl BoolIT
/*
case class BoolIT[+R <: IRegionsModeI]() extends KindIT[R] {
  override def isPrimitive: Boolean = true
}
*/
// mig: struct StrIT
pub struct StrIT<R> {
  _phantom: PhantomData<R>,
}
// mig: impl StrIT
/*
case class StrIT[+R <: IRegionsModeI]() extends KindIT[R] {
  override def isPrimitive: Boolean = false
}
*/
// mig: struct FloatIT
pub struct FloatIT<R> {
  _phantom: PhantomData<R>,
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
pub struct StaticSizedArrayIT<R>(std::marker::PhantomData<R>);
// TODO: populate fields when src/instantiating/ast/names.rs is fully migrated.
// mig: impl StaticSizedArrayIT
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
pub struct RuntimeSizedArrayIT<R>(std::marker::PhantomData<R>);
// TODO: populate fields when src/instantiating/ast/names.rs is fully migrated.
// mig: impl RuntimeSizedArrayIT
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
pub enum ISubKindIT<'t, R> {
  StructIT(&'t StructIT<R>),
  InterfaceIT(&'t InterfaceIT<R>),
}
// mig: impl ISubKindIT
/*
// Structs, interfaces, and placeholders
sealed trait ISubKindIT[+R <: IRegionsModeI] extends KindIT[R] {
  def id: IdI[R, ISubKindNameI[R]]
}
*/
// mig: enum ICitizenIT
pub enum ICitizenIT<'t, R> {
  StructIT(&'t StructIT<R>),
  InterfaceIT(&'t InterfaceIT<R>),
}
// mig: impl ICitizenIT
/*
sealed trait ICitizenIT[+R <: IRegionsModeI] extends ISubKindIT[R] {
  def id: IdI[R, ICitizenNameI[R]]
}
*/
// mig: struct StructIT
pub struct StructIT<R>(std::marker::PhantomData<R>);
// TODO: populate fields when src/instantiating/ast/names.rs is fully migrated.
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
pub struct InterfaceIT<R>(std::marker::PhantomData<R>);
// TODO: populate fields when src/instantiating/ast/names.rs is fully migrated.
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