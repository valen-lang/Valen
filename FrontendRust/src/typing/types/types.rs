/*
package dev.vale.typing.types

import dev.vale._
import dev.vale.postparsing.IImpreciseNameS
import dev.vale.typing.ast.{AbstractT, FunctionHeaderT, ICitizenAttributeT}
import dev.vale.typing.env.IInDenizenEnvironmentT
import dev.vale.typing.names._
import dev.vale.highertyping._
import dev.vale.postparsing._
import dev.vale.typing._
import dev.vale.typing.ast._
import dev.vale.typing.templata._
import dev.vale.typing.types._

import scala.collection.immutable.List
*/
use crate::postparsing::names::IImpreciseNameS;
use crate::typing::names::names::*;
use crate::typing::env::environment::*;

// mig: enum OwnershipT
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum OwnershipT {
    Share,
    Own,
    Borrow,
    Weak,
}
/*
sealed trait OwnershipT  {
}
*/
// mig: struct ShareT
// merged into OwnershipT above
/*
case object ShareT extends OwnershipT {
  override def toString: String = "share"
}
*/
// mig: struct OwnT
// merged into OwnershipT above
/*
case object OwnT extends OwnershipT {
  override def toString: String = "own"
}
*/
// mig: struct BorrowT
// merged into OwnershipT above
/*
case object BorrowT extends OwnershipT {
  override def toString: String = "borrow"
}
*/
// mig: struct WeakT
// merged into OwnershipT above
/*
case object WeakT extends OwnershipT {
  override def toString: String = "weak"
}
*/
// mig: enum MutabilityT
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MutabilityT {
    Mutable,
    Immutable,
}
/*
sealed trait MutabilityT  {
}
*/
// mig: struct MutableT
// merged into MutabilityT above
/*
case object MutableT extends MutabilityT {
  override def toString: String = "mut"
}
*/
// mig: struct ImmutableT
// merged into MutabilityT above
/*
case object ImmutableT extends MutabilityT {
  override def toString: String = "imm"
}
*/
// mig: enum VariabilityT
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum VariabilityT {
    Final,
    Varying,
}
/*
sealed trait VariabilityT  {
}
*/
// mig: struct FinalT
// merged into VariabilityT above
/*
case object FinalT extends VariabilityT {
  override def toString: String = "final"
}
*/
// mig: struct VaryingT
// merged into VariabilityT above
/*
case object VaryingT extends VariabilityT {
  override def toString: String = "vary"
}
*/
// mig: enum LocationT
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum LocationT {
    Inline,
    Yonder,
}
/*
sealed trait LocationT  {
}
*/
// mig: struct InlineT
// merged into LocationT above
/*
case object InlineT extends LocationT {
  override def toString: String = "inl"
}
*/
// mig: struct YonderT
// merged into LocationT above
/*
case object YonderT extends LocationT {
  override def toString: String = "heap"
}
*/
// mig: struct RegionT
#[derive(Copy, Clone)]
pub struct RegionT;
/*
case class RegionT()
*/
// mig: struct CoordT
#[derive(Copy, Clone)]
pub struct CoordT<'s, 't> {
  pub ownership: OwnershipT,
  pub region: RegionT,
  pub kind: KindT<'s, 't>,
}
/*
case class CoordT(
  ownership: OwnershipT,
  // TODO(regions): Replace with an actual region.
  // Usually these will just be placeholders, but one day we might want to say e.g. host'
  region: RegionT,
  kind: KindT)  {

  vpass()

  kind match {
    case IntT(_) | BoolT() | StrT() | FloatT() | VoidT() | NeverT(_) => {
      vassert(ownership == ShareT)
    }
    case _ =>
  }
  if (ownership == OwnT) {
    // See CSHROOR for why we don't assert this.
    // vassert(permission == Readwrite)
  }
}
*/
// mig: enum KindT
// TODO: non-primitive variants (StructTT, InterfaceTT, StaticSizedArrayTT,
//   RuntimeSizedArrayTT, KindPlaceholderT, OverloadSet) are deferred to Slab 3;
//   _Phantom stays until then to anchor the 's/'t lifetime params.
#[derive(Copy, Clone)]
pub enum KindT<'s, 't> {
  Never(NeverT),
  Void(VoidT),
  Int(IntT),
  Bool(BoolT),
  Str(StrT),
  Float(FloatT),
  _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
/*
sealed trait KindT {
  // Note, we don't have a mutability: Mutability in here because this Kind
  // should be enough to uniquely identify a type, and no more.
  // We can always get the mutability for a struct from the coutputs.

  def expectCitizen(): ICitizenTT = {
    this match {
      case c : ICitizenTT => c
      case _ => vfail()
    }
  }

  def expectInterface(): InterfaceTT = {
    this match {
      case c @ InterfaceTT(_) => c
      case _ => vfail()
    }
  }

  def expectStruct(): StructTT = {
    this match {
      case c @ StructTT(_) => c
      case _ => vfail()
    }
  }

  def isPrimitive: Boolean
}
*/
// mig: struct NeverT
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct NeverT {
  pub from_break: bool,
}
/*
// like Scala's Nothing. No instance of this can ever happen.
case class NeverT(
  // True if this Never came from a break.
  // While will have to know about this; if it's a Never from a ret, it should
  // propagate it, but if its body is a break never, the while produces a void.
  // See BRCOBS.
  fromBreak: Boolean
) extends KindT {
  override def isPrimitive: Boolean = true
}
*/
// mig: struct VoidT
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct VoidT;
/*
// Mostly for interoperability with extern functions
case class VoidT() extends KindT {
  override def isPrimitive: Boolean = true
}
*/
// mig: impl IntT
impl IntT {
    pub const I32: IntT = IntT { bits: 32 };
    pub const I64: IntT = IntT { bits: 64 };
}
/*
object IntT {
  val i32: IntT = IntT(32)
  val i64: IntT = IntT(64)
}
*/
// mig: struct IntT
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IntT {
  pub bits: i32,
}
/*
case class IntT(bits: Int) extends KindT {
  override def isPrimitive: Boolean = true
}
*/
// mig: struct BoolT
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BoolT;
/*
case class BoolT() extends KindT {
  override def isPrimitive: Boolean = true

}
*/
// mig: struct StrT
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StrT;
/*
case class StrT() extends KindT {
  override def isPrimitive: Boolean = false

}
*/
// mig: struct FloatT
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FloatT;
/*
case class FloatT() extends KindT {
  override def isPrimitive: Boolean = true
}
*/
// mig: fn unapply
fn unapply_contents_static_sized_array_tt() {
  panic!("Unimplemented: unapply_contents_static_sized_array_tt");
}
/*
object contentsStaticSizedArrayTT {
  def unapply(ssa: StaticSizedArrayTT):
  Option[(ITemplataT[IntegerTemplataType], ITemplataT[MutabilityTemplataType], ITemplataT[VariabilityTemplataType], CoordT, RegionT)] = {
    val IdT(_, _, StaticSizedArrayNameT(_, size, variability, RawArrayNameT(mutability, coord, selfRegion))) = ssa.name
    Some((size, mutability, variability, coord, selfRegion))
  }
}
*/
// mig: struct StaticSizedArrayTT
pub struct StaticSizedArrayTT<'s, 't> {
  pub name: IdT<'s, 't>,
}
/*
case class StaticSizedArrayTT(
  name: IdT[StaticSizedArrayNameT]
) extends KindT with IInterning {
  vassert(name.initSteps.isEmpty)
  override def isPrimitive: Boolean = false
  def mutability: ITemplataT[MutabilityTemplataType] = name.localName.arr.mutability
  def elementType = name.localName.arr.elementType
  def size = name.localName.size
  def variability = name.localName.variability
}
*/
// mig: fn unapply
fn unapply_contents_runtime_sized_array_tt() {
  panic!("Unimplemented: unapply_contents_runtime_sized_array_tt");
}
/*
object contentsRuntimeSizedArrayTT {
  def unapply(rsa: RuntimeSizedArrayTT):
  Option[(ITemplataT[MutabilityTemplataType], CoordT, RegionT)] = {
    val IdT(_, _, RuntimeSizedArrayNameT(_, RawArrayNameT(mutability, coord, selfRegion))) = rsa.name
    Some((mutability, coord, selfRegion))
  }
}
*/
// mig: struct RuntimeSizedArrayTT
pub struct RuntimeSizedArrayTT<'s, 't> {
  pub name: IdT<'s, 't>,
}
/*
case class RuntimeSizedArrayTT(
  name: IdT[RuntimeSizedArrayNameT]
) extends KindT with IInterning {
  override def isPrimitive: Boolean = false
  def mutability = name.localName.arr.mutability
  def elementType = name.localName.arr.elementType
}
*/
// mig: fn unapply
fn unapply_i_citizen_tt() {
  panic!("Unimplemented: unapply_i_citizen_tt");
}
/*
object ICitizenTT {
  def unapply(self: ICitizenTT): Option[IdT[ICitizenNameT]] = {
    Some(self.id)
  }
}
*/
// mig: enum ISubKindTT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum ISubKindTT<'s, 't> {
  _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
/*
// Structs, interfaces, and placeholders
sealed trait ISubKindTT extends KindT {
  def id: IdT[ISubKindNameT]
}
*/
// mig: enum ISuperKindTT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum ISuperKindTT<'s, 't> {
  _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
/*
// Interfaces and placeholders
sealed trait ISuperKindTT extends KindT {
  def id: IdT[ISuperKindNameT]
}
*/
// mig: enum ICitizenTT
// TODO: placeholder PhantomData — replace with real fields during body migration
pub enum ICitizenTT<'s, 't> {
  _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
/*
sealed trait ICitizenTT extends ISubKindTT with IInterning {
  def id: IdT[ICitizenNameT]
}
*/
// mig: struct StructTT
pub struct StructTT<'s, 't> {
  pub id: IdT<'s, 't>,
}
/*
// These should only be made by StructCompiler, which puts the definition and bounds into coutputs at the same time
case class StructTT(id: IdT[IStructNameT]) extends ICitizenTT {
  override def isPrimitive: Boolean = false
  (id.initSteps.lastOption, id.localName) match {
    case (Some(StructTemplateNameT(_)), StructNameT(_, _)) => vfail()
    case _ =>
  }
}
*/
// mig: struct InterfaceTT
pub struct InterfaceTT<'s, 't> {
  pub id: IdT<'s, 't>,
}
/*
case class InterfaceTT(id: IdT[IInterfaceNameT]) extends ICitizenTT with ISuperKindTT {
  override def isPrimitive: Boolean = false
  (id.initSteps.lastOption, id.localName) match {
    case (Some(InterfaceTemplateNameT(_)), InterfaceNameT(_, _)) => vfail()
    case _ =>
  }
}
*/
// mig: struct OverloadSetT
pub struct OverloadSetT<'s, 't> {
  pub env: &'s IInDenizenEnvironmentT<'s, 't>,
  pub name: &'s IImpreciseNameS<'s>,
}
/*
// Represents a bunch of functions that have the same name.
// See ROS.
// Lowers to an empty struct.
case class OverloadSetT(
  env: IInDenizenEnvironmentT,
  // The name to look for in the environment.
  name: IImpreciseNameS
) extends KindT with IInterning {
  override def isPrimitive: Boolean = true
  vpass()

}
*/
// mig: struct KindPlaceholderT
pub struct KindPlaceholderT<'s, 't> {
  pub id: IdT<'s, 't>,
}
/*
// At some point it'd be nice to make Coord.kind into a templata so we can directly have a
// placeholder templata instead of needing this special kind.
case class KindPlaceholderT(id: IdT[KindPlaceholderNameT]) extends ISubKindTT with ISuperKindTT {
  override def isPrimitive: Boolean = false
}
*/
