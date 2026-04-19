use crate::postparsing::names::IImpreciseNameS;
use crate::typing::names::names::*;
use crate::typing::env::environment::*;

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
// merged into OwnershipT above
/*
case object ShareT extends OwnershipT {
  override def toString: String = "share"
}
*/
// merged into OwnershipT above
/*
case object OwnT extends OwnershipT {
  override def toString: String = "own"
}
*/
// merged into OwnershipT above
/*
case object BorrowT extends OwnershipT {
  override def toString: String = "borrow"
}
*/
// merged into OwnershipT above
/*
case object WeakT extends OwnershipT {
  override def toString: String = "weak"
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MutabilityT {
    Mutable,
    Immutable,
}
/*
sealed trait MutabilityT  {
}
*/
// merged into MutabilityT above
/*
case object MutableT extends MutabilityT {
  override def toString: String = "mut"
}
*/
// merged into MutabilityT above
/*
case object ImmutableT extends MutabilityT {
  override def toString: String = "imm"
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum VariabilityT {
    Final,
    Varying,
}
/*
sealed trait VariabilityT  {
}
*/
// merged into VariabilityT above
/*
case object FinalT extends VariabilityT {
  override def toString: String = "final"
}
*/
// merged into VariabilityT above
/*
case object VaryingT extends VariabilityT {
  override def toString: String = "vary"
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum LocationT {
    Inline,
    Yonder,
}
/*
sealed trait LocationT  {
}
*/
// merged into LocationT above
/*
case object InlineT extends LocationT {
  override def toString: String = "inl"
}
*/
// merged into LocationT above
/*
case object YonderT extends LocationT {
  override def toString: String = "heap"
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RegionT;
/*
case class RegionT()
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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
// KindT is inline-owned (not arena-interned). Concrete non-primitive payloads
// (StructTT, InterfaceTT, etc.) are arena-interned and held as &'t refs here.
// Primitives (NeverT, VoidT, IntT, BoolT, StrT, FloatT) inline by value —
// they're small enough to skip arena indirection.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum KindT<'s, 't> {
  Never(NeverT),
  Void(VoidT),
  Int(IntT),
  Bool(BoolT),
  Str(StrT),
  Float(FloatT),
  Struct(&'t StructTT<'s, 't>),
  Interface(&'t InterfaceTT<'s, 't>),
  StaticSizedArray(&'t StaticSizedArrayTT<'s, 't>),
  RuntimeSizedArray(&'t RuntimeSizedArrayTT<'s, 't>),
  KindPlaceholder(&'t KindPlaceholderT<'s, 't>),
  OverloadSet(&'t OverloadSetT<'s, 't>),
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct VoidT;
/*
// Mostly for interoperability with extern functions
case class VoidT() extends KindT {
  override def isPrimitive: Boolean = true
}
*/
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IntT {
  pub bits: i32,
}
/*
case class IntT(bits: Int) extends KindT {
  override def isPrimitive: Boolean = true
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BoolT;
/*
case class BoolT() extends KindT {
  override def isPrimitive: Boolean = true

}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StrT;
/*
case class StrT() extends KindT {
  override def isPrimitive: Boolean = false

}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FloatT;
/*
case class FloatT() extends KindT {
  override def isPrimitive: Boolean = true
}
*/
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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
// Inline-owned wrapper enum; concrete payloads are arena-interned &'t refs.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ISubKindTT<'s, 't> {
  Struct(&'t StructTT<'s, 't>),
  Interface(&'t InterfaceTT<'s, 't>),
  StaticSizedArray(&'t StaticSizedArrayTT<'s, 't>),
  RuntimeSizedArray(&'t RuntimeSizedArrayTT<'s, 't>),
  KindPlaceholder(&'t KindPlaceholderT<'s, 't>),
}
/*
// Structs, interfaces, and placeholders
sealed trait ISubKindTT extends KindT {
  def id: IdT[ISubKindNameT]
}
*/
// Inline-owned wrapper enum; concrete payloads are arena-interned &'t refs.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ISuperKindTT<'s, 't> {
  Interface(&'t InterfaceTT<'s, 't>),
  KindPlaceholder(&'t KindPlaceholderT<'s, 't>),
}
/*
// Interfaces and placeholders
sealed trait ISuperKindTT extends KindT {
  def id: IdT[ISuperKindNameT]
}
*/
// Inline-owned wrapper enum; concrete payloads are arena-interned &'t refs.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ICitizenTT<'s, 't> {
  Struct(&'t StructTT<'s, 't>),
  Interface(&'t InterfaceTT<'s, 't>),
}
/*
sealed trait ICitizenTT extends ISubKindTT with IInterning {
  def id: IdT[ICitizenNameT]
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OverloadSetT<'s, 't> {
  pub env: &'t IInDenizenEnvironmentT<'s, 't>,
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
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
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

// -- Simple / shallow concretes (reuse struct itself as Val) ------------------
// The 6 concrete Kind payloads above (StructTT, InterfaceTT, StaticSizedArrayTT,
// RuntimeSizedArrayTT, KindPlaceholderT, OverloadSetT) are arena-interned but
// have no `&'t [...]` slice fields — each holds either a canonical IdT<'s, 't>
// (canonicalized by IdValT before this Val is constructed) or scout-lifetime
// refs only (OverloadSetT). So their permanent struct doubles as the lookup
// Val. No separate `*ValT` type is defined for any of them.
//
// The wrapper enums KindT / ICitizenTT / ISubKindTT / ISuperKindTT are
// inline-owned 16-byte Copy values (never arena-allocated), so they don't
// need Val companions either. Casts between them are `match`-and-rewrap via
// the From/TryFrom bridges below.

// -- Union enums for the Kind-payload interning family ----------------------
// Per handoff-slab-4.md Gotcha 2: typing interner uses one HashMap per
// sealed-trait family with a tagged-union Val key. These mirror scout's
// INameValS/INameS pattern.
//
// All 6 variants are "simple" (struct is its own Val, no 'tmp lifetime).

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum InternedKindPayloadValT<'s, 't>
where 's: 't,
{
  StructTT(StructTT<'s, 't>),
  InterfaceTT(InterfaceTT<'s, 't>),
  StaticSizedArrayTT(StaticSizedArrayTT<'s, 't>),
  RuntimeSizedArrayTT(RuntimeSizedArrayTT<'s, 't>),
  KindPlaceholder(KindPlaceholderT<'s, 't>),
  OverloadSet(OverloadSetT<'s, 't>),
}

#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum InternedKindPayloadT<'s, 't>
where 's: 't,
{
  StructTT(&'t StructTT<'s, 't>),
  InterfaceTT(&'t InterfaceTT<'s, 't>),
  StaticSizedArrayTT(&'t StaticSizedArrayTT<'s, 't>),
  RuntimeSizedArrayTT(&'t RuntimeSizedArrayTT<'s, 't>),
  KindPlaceholder(&'t KindPlaceholderT<'s, 't>),
  OverloadSet(&'t OverloadSetT<'s, 't>),
}

// -- From bridges: concrete payload → each wrapper enum it belongs to --------

impl<'s, 't> From<&'t StructTT<'s, 't>> for ICitizenTT<'s, 't> {
  fn from(x: &'t StructTT<'s, 't>) -> Self { ICitizenTT::Struct(x) }
}
impl<'s, 't> From<&'t StructTT<'s, 't>> for ISubKindTT<'s, 't> {
  fn from(x: &'t StructTT<'s, 't>) -> Self { ISubKindTT::Struct(x) }
}
impl<'s, 't> From<&'t StructTT<'s, 't>> for KindT<'s, 't> {
  fn from(x: &'t StructTT<'s, 't>) -> Self { KindT::Struct(x) }
}

impl<'s, 't> From<&'t InterfaceTT<'s, 't>> for ICitizenTT<'s, 't> {
  fn from(x: &'t InterfaceTT<'s, 't>) -> Self { ICitizenTT::Interface(x) }
}
impl<'s, 't> From<&'t InterfaceTT<'s, 't>> for ISubKindTT<'s, 't> {
  fn from(x: &'t InterfaceTT<'s, 't>) -> Self { ISubKindTT::Interface(x) }
}
impl<'s, 't> From<&'t InterfaceTT<'s, 't>> for ISuperKindTT<'s, 't> {
  fn from(x: &'t InterfaceTT<'s, 't>) -> Self { ISuperKindTT::Interface(x) }
}
impl<'s, 't> From<&'t InterfaceTT<'s, 't>> for KindT<'s, 't> {
  fn from(x: &'t InterfaceTT<'s, 't>) -> Self { KindT::Interface(x) }
}

impl<'s, 't> From<&'t StaticSizedArrayTT<'s, 't>> for ISubKindTT<'s, 't> {
  fn from(x: &'t StaticSizedArrayTT<'s, 't>) -> Self { ISubKindTT::StaticSizedArray(x) }
}
impl<'s, 't> From<&'t StaticSizedArrayTT<'s, 't>> for KindT<'s, 't> {
  fn from(x: &'t StaticSizedArrayTT<'s, 't>) -> Self { KindT::StaticSizedArray(x) }
}

impl<'s, 't> From<&'t RuntimeSizedArrayTT<'s, 't>> for ISubKindTT<'s, 't> {
  fn from(x: &'t RuntimeSizedArrayTT<'s, 't>) -> Self { ISubKindTT::RuntimeSizedArray(x) }
}
impl<'s, 't> From<&'t RuntimeSizedArrayTT<'s, 't>> for KindT<'s, 't> {
  fn from(x: &'t RuntimeSizedArrayTT<'s, 't>) -> Self { KindT::RuntimeSizedArray(x) }
}

impl<'s, 't> From<&'t KindPlaceholderT<'s, 't>> for ISubKindTT<'s, 't> {
  fn from(x: &'t KindPlaceholderT<'s, 't>) -> Self { ISubKindTT::KindPlaceholder(x) }
}
impl<'s, 't> From<&'t KindPlaceholderT<'s, 't>> for ISuperKindTT<'s, 't> {
  fn from(x: &'t KindPlaceholderT<'s, 't>) -> Self { ISuperKindTT::KindPlaceholder(x) }
}
impl<'s, 't> From<&'t KindPlaceholderT<'s, 't>> for KindT<'s, 't> {
  fn from(x: &'t KindPlaceholderT<'s, 't>) -> Self { KindT::KindPlaceholder(x) }
}

impl<'s, 't> From<&'t OverloadSetT<'s, 't>> for KindT<'s, 't> {
  fn from(x: &'t OverloadSetT<'s, 't>) -> Self { KindT::OverloadSet(x) }
}

// -- From bridges: narrow sub-enum → wider sub-enum / KindT ------------------

impl<'s, 't> From<ICitizenTT<'s, 't>> for ISubKindTT<'s, 't> {
  fn from(c: ICitizenTT<'s, 't>) -> Self {
    match c {
      ICitizenTT::Struct(x) => ISubKindTT::Struct(x),
      ICitizenTT::Interface(x) => ISubKindTT::Interface(x),
    }
  }
}
impl<'s, 't> From<ICitizenTT<'s, 't>> for KindT<'s, 't> {
  fn from(c: ICitizenTT<'s, 't>) -> Self {
    match c {
      ICitizenTT::Struct(x) => KindT::Struct(x),
      ICitizenTT::Interface(x) => KindT::Interface(x),
    }
  }
}
impl<'s, 't> From<ISubKindTT<'s, 't>> for KindT<'s, 't> {
  fn from(s: ISubKindTT<'s, 't>) -> Self {
    match s {
      ISubKindTT::Struct(x) => KindT::Struct(x),
      ISubKindTT::Interface(x) => KindT::Interface(x),
      ISubKindTT::StaticSizedArray(x) => KindT::StaticSizedArray(x),
      ISubKindTT::RuntimeSizedArray(x) => KindT::RuntimeSizedArray(x),
      ISubKindTT::KindPlaceholder(x) => KindT::KindPlaceholder(x),
    }
  }
}
impl<'s, 't> From<ISuperKindTT<'s, 't>> for KindT<'s, 't> {
  fn from(s: ISuperKindTT<'s, 't>) -> Self {
    match s {
      ISuperKindTT::Interface(x) => KindT::Interface(x),
      ISuperKindTT::KindPlaceholder(x) => KindT::KindPlaceholder(x),
    }
  }
}

// -- TryFrom bridges: wider → narrower ---------------------------------------

impl<'s, 't> TryFrom<KindT<'s, 't>> for ICitizenTT<'s, 't> {
  type Error = ();
  fn try_from(k: KindT<'s, 't>) -> Result<Self, ()> {
    match k {
      KindT::Struct(x) => Ok(ICitizenTT::Struct(x)),
      KindT::Interface(x) => Ok(ICitizenTT::Interface(x)),
      _ => Err(()),
    }
  }
}
impl<'s, 't> TryFrom<KindT<'s, 't>> for ISubKindTT<'s, 't> {
  type Error = ();
  fn try_from(k: KindT<'s, 't>) -> Result<Self, ()> {
    match k {
      KindT::Struct(x) => Ok(ISubKindTT::Struct(x)),
      KindT::Interface(x) => Ok(ISubKindTT::Interface(x)),
      KindT::StaticSizedArray(x) => Ok(ISubKindTT::StaticSizedArray(x)),
      KindT::RuntimeSizedArray(x) => Ok(ISubKindTT::RuntimeSizedArray(x)),
      KindT::KindPlaceholder(x) => Ok(ISubKindTT::KindPlaceholder(x)),
      _ => Err(()),
    }
  }
}
impl<'s, 't> TryFrom<KindT<'s, 't>> for ISuperKindTT<'s, 't> {
  type Error = ();
  fn try_from(k: KindT<'s, 't>) -> Result<Self, ()> {
    match k {
      KindT::Interface(x) => Ok(ISuperKindTT::Interface(x)),
      KindT::KindPlaceholder(x) => Ok(ISuperKindTT::KindPlaceholder(x)),
      _ => Err(()),
    }
  }
}
impl<'s, 't> TryFrom<ISubKindTT<'s, 't>> for ICitizenTT<'s, 't> {
  type Error = ();
  fn try_from(s: ISubKindTT<'s, 't>) -> Result<Self, ()> {
    match s {
      ISubKindTT::Struct(x) => Ok(ICitizenTT::Struct(x)),
      ISubKindTT::Interface(x) => Ok(ICitizenTT::Interface(x)),
      _ => Err(()),
    }
  }
}
