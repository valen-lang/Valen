use crate::postparsing::names::IImpreciseNameS;
use crate::typing::names::names::*;
use crate::typing::env::environment::*;
use crate::typing::templata::templata::ITemplataT;
use crate::typing::typing_interner::MustIntern;

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
/// Value-type (see @TFITCX)
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
/// Value-type (see @TFITCX)
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
/// Value-type (see @TFITCX)
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
/// Value-type (see @TFITCX)
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
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IRegionT {
  Iso,
  // TODO: Get rid of this when we have an actual default region
  Default,
}

/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RegionT {
  pub region: IRegionT,
}
/*
sealed trait IRegionT
case object IsoRegionT extends IRegionT
// TODO: Get rid of this when we have an actual default region
case object DefaultRegionT extends IRegionT
case class RegionT(region: IRegionT)
*/
/// Value-type (see @TFITCX)
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
// Primitives inline by value; compound types use &'t to keep the enum small (see @WVSBIZ).
/// Polyvalue (see @TFITCX) — derive Eq/Hash; never hand-roll `ptr::eq` on the outer `&self` (see @PVECFPZ).
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
*/
impl<'s, 't> KindT<'s, 't> {
  pub fn expect_citizen(&self) -> ICitizenTT<'s, 't> {
    match self {
      KindT::Struct(c) => ICitizenTT::Struct(c),
      KindT::Interface(c) => ICitizenTT::Interface(c),
      _ => panic!("vfail"),
    }
  }
  /*
  def expectCitizen(): ICitizenTT = {
    this match {
      case c : ICitizenTT => c
      case _ => vfail()
    }
  }
  */
  /* Guardian: disable-all */
}
impl<'s, 't> KindT<'s, 't> {
  pub fn expect_interface(&self) -> &'t InterfaceTT<'s, 't> {
    match self {
      KindT::Interface(c) => c,
      _ => panic!("vfail"),
    }
  }
  /*
  def expectInterface(): InterfaceTT = {
    this match {
      case c @ InterfaceTT(_) => c
      case _ => vfail()
    }
  }
  */
  /* Guardian: disable-all */
}
impl<'s, 't> KindT<'s, 't> {
  pub fn expect_struct(&self) -> &'t StructTT<'s, 't> {
    match self {
      KindT::Struct(c) => c,
      _ => panic!("vfail"),
    }
  }
  /*
  def expectStruct(): StructTT = {
    this match {
      case c @ StructTT(_) => c
      case _ => vfail()
    }
  }
  */
  /* Guardian: disable-all */
}
impl<'s, 't> KindT<'s, 't> {
  pub fn is_primitive(&self) -> bool {
    match self {
      KindT::Never(_) => true,
      KindT::Void(_) => true,
      KindT::Int(_) => true,
      KindT::Bool(_) => true,
      KindT::Str(_) => false,
      KindT::Float(_) => true,
      KindT::Struct(_) => false,
      KindT::Interface(_) => false,
      KindT::StaticSizedArray(_) => false,
      KindT::RuntimeSizedArray(_) => false,
      KindT::KindPlaceholder(_) => false,
      KindT::OverloadSet(_) => true,
    }
  }
  /*
  def isPrimitive: Boolean
  */
  /* Guardian: disable-all */
}
/*
}
*/
/// Value-type (see @TFITCX)
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
/// Value-type (see @TFITCX)
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
/*
object IntT {
  val i32: IntT = IntT(32)
  val i64: IntT = IntT(64)
}
*/
}
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IntT {
  pub bits: i32,
}
/*
case class IntT(bits: Int) extends KindT {
  override def isPrimitive: Boolean = true
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BoolT;
/*
case class BoolT() extends KindT {
  override def isPrimitive: Boolean = true

}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StrT;
/*
case class StrT() extends KindT {
  override def isPrimitive: Boolean = false

}
*/
/// Value-type (see @TFITCX)
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
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayTT<'s, 't> {
  pub name: IdT<'s, 't>,
  pub _must_intern: MustIntern,
}
/*
case class StaticSizedArrayTT(
  name: IdT[StaticSizedArrayNameT]
) extends KindT with IInterning {
*/
/*
  vassert(name.initSteps.isEmpty)
*/
/*
  override def isPrimitive: Boolean = false
*/
impl<'s, 't> StaticSizedArrayTT<'s, 't> where 's: 't {
  pub fn mutability(&self) -> ITemplataT<'s, 't> {
    match self.name.local_name {
      INameT::StaticSizedArray(ssa_name) => ssa_name.arr.mutability,
      _ => panic!("vwat"),
    }
  }
  /*
  def mutability: ITemplataT[MutabilityTemplataType] = name.localName.arr.mutability
  */
}
impl<'s, 't> StaticSizedArrayTT<'s, 't> where 's: 't {
  pub fn element_type(&self) -> CoordT<'s, 't> {
    match self.name.local_name {
      INameT::StaticSizedArray(ssa_name) => ssa_name.arr.element_type,
      _ => panic!("vwat"),
    }
  }
  /*
  def elementType = name.localName.arr.elementType
  */
}
impl<'s, 't> StaticSizedArrayTT<'s, 't> where 's: 't {
  pub fn size(&self) -> ITemplataT<'s, 't> {
    match self.name.local_name {
      INameT::StaticSizedArray(ssa_name) => ssa_name.size,
      _ => panic!("vwat"),
    }
  }
  /*
  def size = name.localName.size
  */
}
impl<'s, 't> StaticSizedArrayTT<'s, 't> where 's: 't {
  pub fn variability(&self) -> ITemplataT<'s, 't> {
    match self.name.local_name {
      INameT::StaticSizedArray(ssa_name) => ssa_name.variability,
      _ => panic!("vwat"),
    }
  }
  /*
  def variability = name.localName.variability
  */
}
/*
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
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayTTValT<'s, 't> {
  pub name: IdT<'s, 't>,
}

/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayTT<'s, 't> {
  pub name: IdT<'s, 't>,
  pub _must_intern: MustIntern,
}
/*
case class RuntimeSizedArrayTT(
  name: IdT[RuntimeSizedArrayNameT]
) extends KindT with IInterning {
  override def isPrimitive: Boolean = false
*/
impl<'s, 't> RuntimeSizedArrayTT<'s, 't> where 's: 't {
  pub fn mutability(&self) -> ITemplataT<'s, 't> {
    match self.name.local_name {
      INameT::RuntimeSizedArray(rsa_name) => rsa_name.arr.mutability,
      _ => panic!("vwat"),
    }
  }
  /*
  def mutability = name.localName.arr.mutability
  */
}
impl<'s, 't> RuntimeSizedArrayTT<'s, 't> where 's: 't {
  pub fn element_type(&self) -> CoordT<'s, 't> {
    match self.name.local_name {
      INameT::RuntimeSizedArray(rsa_name) => rsa_name.arr.element_type,
      _ => panic!("vwat"),
    }
  }
  /*
  def elementType = name.localName.arr.elementType
  */
}
/*
}
*/
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayTTValT<'s, 't> {
  pub name: IdT<'s, 't>,
}

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
/// Polyvalue (see @TFITCX) — derive Eq/Hash; never hand-roll `ptr::eq` on the outer `&self` (see @PVECFPZ).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ISubKindTT<'s, 't> {
  Struct(&'t StructTT<'s, 't>),
  Interface(&'t InterfaceTT<'s, 't>),
  KindPlaceholder(&'t KindPlaceholderT<'s, 't>),
}
/*
// Structs, interfaces, and placeholders
sealed trait ISubKindTT extends KindT {
*/
impl<'s, 't> ISubKindTT<'s, 't> where 's: 't {
    pub fn id(&self) -> IdT<'s, 't> {
        match self {
            ISubKindTT::Struct(s) => s.id,
            ISubKindTT::Interface(i) => i.id,
            ISubKindTT::KindPlaceholder(kp) => kp.id,
        }
    }
    /*
  def id: IdT[ISubKindNameT]
*/
    pub fn expect_citizen(&self) -> ICitizenTT<'s, 't> {
        match self {
            ISubKindTT::Struct(s) => ICitizenTT::Struct(s),
            ISubKindTT::Interface(i) => ICitizenTT::Interface(i),
            ISubKindTT::KindPlaceholder(_) => panic!("vfail"),
        }
    }
    /* Guardian: disable-all */
}
impl<'s, 't> ISubKindTT<'s, 't> where 's: 't {
    pub fn expect_interface(&self) -> &'t InterfaceTT<'s, 't> {
        KindT::from(*self).expect_interface()
    }
    /* Guardian: disable-all */
}
impl<'s, 't> ISubKindTT<'s, 't> where 's: 't {
    pub fn expect_struct(&self) -> &'t StructTT<'s, 't> {
        KindT::from(*self).expect_struct()
    }
    /* Guardian: disable-all */
}
impl<'s, 't> ISubKindTT<'s, 't> where 's: 't {
    pub fn is_primitive(&self) -> bool {
        KindT::from(*self).is_primitive()
    }
    /* Guardian: disable-all */
}
/*
}
*/
// Inline-owned wrapper enum; concrete payloads are arena-interned &'t refs.
/// Polyvalue (see @TFITCX) — derive Eq/Hash; never hand-roll `ptr::eq` on the outer `&self` (see @PVECFPZ).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ISuperKindTT<'s, 't> {
  Interface(&'t InterfaceTT<'s, 't>),
  KindPlaceholder(&'t KindPlaceholderT<'s, 't>),
}
/*
// Interfaces and placeholders
sealed trait ISuperKindTT extends KindT {
*/
impl<'s, 't> ISuperKindTT<'s, 't> where 's: 't {
    pub fn id(&self) -> IdT<'s, 't> {
        match self {
            ISuperKindTT::Interface(i) => i.id,
            ISuperKindTT::KindPlaceholder(kp) => kp.id,
        }
    }
    /*
  def id: IdT[ISuperKindNameT]
*/
}
impl<'s, 't> ISuperKindTT<'s, 't> where 's: 't {
    pub fn expect_citizen(&self) -> ICitizenTT<'s, 't> {
        KindT::from(*self).expect_citizen()
    }
    /* Guardian: disable-all */
}
impl<'s, 't> ISuperKindTT<'s, 't> where 's: 't {
    pub fn expect_interface(&self) -> &'t InterfaceTT<'s, 't> {
        KindT::from(*self).expect_interface()
    }
    /* Guardian: disable-all */
}
impl<'s, 't> ISuperKindTT<'s, 't> where 's: 't {
    pub fn expect_struct(&self) -> &'t StructTT<'s, 't> {
        KindT::from(*self).expect_struct()
    }
    /* Guardian: disable-all */
}
impl<'s, 't> ISuperKindTT<'s, 't> where 's: 't {
    pub fn is_primitive(&self) -> bool {
        KindT::from(*self).is_primitive()
    }
    /* Guardian: disable-all */
}
/*
}
*/
// Inline-owned wrapper enum; concrete payloads are arena-interned &'t refs.
/// Polyvalue (see @TFITCX) — derive Eq/Hash; never hand-roll `ptr::eq` on the outer `&self` (see @PVECFPZ).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ICitizenTT<'s, 't> {
  Struct(&'t StructTT<'s, 't>),
  Interface(&'t InterfaceTT<'s, 't>),
}
/*
sealed trait ICitizenTT extends ISubKindTT with IInterning {
*/
impl<'s, 't> ICitizenTT<'s, 't> where 's: 't {
    pub fn id(&self) -> IdT<'s, 't> {
        match self {
            ICitizenTT::Struct(s) => s.id,
            ICitizenTT::Interface(i) => i.id,
        }
    }
    /*
  def id: IdT[ICitizenNameT]
*/
}
impl<'s, 't> ICitizenTT<'s, 't> where 's: 't {
    pub fn expect_citizen(&self) -> ICitizenTT<'s, 't> {
        *self
    }
    /* Guardian: disable-all */
}
impl<'s, 't> ICitizenTT<'s, 't> where 's: 't {
    pub fn expect_interface(&self) -> &'t InterfaceTT<'s, 't> {
        KindT::from(*self).expect_interface()
    }
    /* Guardian: disable-all */
}
impl<'s, 't> ICitizenTT<'s, 't> where 's: 't {
    pub fn expect_struct(&self) -> &'t StructTT<'s, 't> {
        KindT::from(*self).expect_struct()
    }
    /* Guardian: disable-all */
}
impl<'s, 't> ICitizenTT<'s, 't> where 's: 't {
    pub fn is_primitive(&self) -> bool {
        KindT::from(*self).is_primitive()
    }
    /* Guardian: disable-all */
}
/*
}
*/
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructTT<'s, 't> {
  pub id: IdT<'s, 't>,
  pub _must_intern: MustIntern,
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
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructTTValT<'s, 't> {
  pub id: IdT<'s, 't>,
}

/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceTT<'s, 't> {
  pub id: IdT<'s, 't>,
  pub _must_intern: MustIntern,
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
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceTTValT<'s, 't> {
  pub id: IdT<'s, 't>,
}

/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OverloadSetT<'s, 't> {
  pub env: IInDenizenEnvironmentT<'s, 't>,
  pub name: &'s IImpreciseNameS<'s>,
  pub _must_intern: MustIntern,
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
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OverloadSetTValT<'s, 't> {
  pub env: IInDenizenEnvironmentT<'s, 't>,
  pub name: &'s IImpreciseNameS<'s>,
}

/// Value-type (see @TFITCX)
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
// Dispatch enums for kind interning — these types are interned per @WVSBIZ
// (enum budget: KindT stores them behind &'t to stay small and Copy).
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum InternedKindPayloadValT<'s, 't>
where 's: 't,
{
  StructTT(StructTTValT<'s, 't>),
  InterfaceTT(InterfaceTTValT<'s, 't>),
  StaticSizedArrayTT(StaticSizedArrayTTValT<'s, 't>),
  RuntimeSizedArrayTT(RuntimeSizedArrayTTValT<'s, 't>),
  KindPlaceholder(KindPlaceholderT<'s, 't>),
  OverloadSet(OverloadSetTValT<'s, 't>),
}

/// Polyvalue (see @TFITCX) — derive Eq/Hash; never hand-roll `ptr::eq` on the outer `&self` (see @PVECFPZ).
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
  /* Guardian: disable-all */
}
impl<'s, 't> From<&'t StructTT<'s, 't>> for ISubKindTT<'s, 't> {
  fn from(x: &'t StructTT<'s, 't>) -> Self { ISubKindTT::Struct(x) }
  /* Guardian: disable-all */
}
impl<'s, 't> From<&'t StructTT<'s, 't>> for KindT<'s, 't> {
  fn from(x: &'t StructTT<'s, 't>) -> Self { KindT::Struct(x) }
  /* Guardian: disable-all */
}

impl<'s, 't> From<&'t InterfaceTT<'s, 't>> for ICitizenTT<'s, 't> {
  fn from(x: &'t InterfaceTT<'s, 't>) -> Self { ICitizenTT::Interface(x) }
  /* Guardian: disable-all */
}
impl<'s, 't> From<&'t InterfaceTT<'s, 't>> for ISubKindTT<'s, 't> {
  fn from(x: &'t InterfaceTT<'s, 't>) -> Self { ISubKindTT::Interface(x) }
  /* Guardian: disable-all */
}
impl<'s, 't> From<&'t InterfaceTT<'s, 't>> for ISuperKindTT<'s, 't> {
  fn from(x: &'t InterfaceTT<'s, 't>) -> Self { ISuperKindTT::Interface(x) }
  /* Guardian: disable-all */
}
impl<'s, 't> From<&'t InterfaceTT<'s, 't>> for KindT<'s, 't> {
  fn from(x: &'t InterfaceTT<'s, 't>) -> Self { KindT::Interface(x) }
  /* Guardian: disable-all */
}

impl<'s, 't> From<&'t StaticSizedArrayTT<'s, 't>> for KindT<'s, 't> {
  fn from(x: &'t StaticSizedArrayTT<'s, 't>) -> Self { KindT::StaticSizedArray(x) }
  /* Guardian: disable-all */
}

impl<'s, 't> From<&'t RuntimeSizedArrayTT<'s, 't>> for KindT<'s, 't> {
  fn from(x: &'t RuntimeSizedArrayTT<'s, 't>) -> Self { KindT::RuntimeSizedArray(x) }
  /* Guardian: disable-all */
}

impl<'s, 't> From<&'t KindPlaceholderT<'s, 't>> for ISubKindTT<'s, 't> {
  fn from(x: &'t KindPlaceholderT<'s, 't>) -> Self { ISubKindTT::KindPlaceholder(x) }
  /* Guardian: disable-all */
}
impl<'s, 't> From<&'t KindPlaceholderT<'s, 't>> for ISuperKindTT<'s, 't> {
  fn from(x: &'t KindPlaceholderT<'s, 't>) -> Self { ISuperKindTT::KindPlaceholder(x) }
  /* Guardian: disable-all */
}
impl<'s, 't> From<&'t KindPlaceholderT<'s, 't>> for KindT<'s, 't> {
  fn from(x: &'t KindPlaceholderT<'s, 't>) -> Self { KindT::KindPlaceholder(x) }
  /* Guardian: disable-all */
}

impl<'s, 't> From<&'t OverloadSetT<'s, 't>> for KindT<'s, 't> {
  fn from(x: &'t OverloadSetT<'s, 't>) -> Self { KindT::OverloadSet(x) }
  /* Guardian: disable-all */
}

// -- From bridges: narrow sub-enum → wider sub-enum / KindT ------------------

impl<'s, 't> From<ICitizenTT<'s, 't>> for ISubKindTT<'s, 't> {
  fn from(c: ICitizenTT<'s, 't>) -> Self {
    match c {
      ICitizenTT::Struct(x) => ISubKindTT::Struct(x),
      ICitizenTT::Interface(x) => ISubKindTT::Interface(x),
    }
  }
  /* Guardian: disable-all */
}
impl<'s, 't> From<ICitizenTT<'s, 't>> for KindT<'s, 't> {
  fn from(c: ICitizenTT<'s, 't>) -> Self {
    match c {
      ICitizenTT::Struct(x) => KindT::Struct(x),
      ICitizenTT::Interface(x) => KindT::Interface(x),
    }
  }
  /* Guardian: disable-all */
}
impl<'s, 't> From<ISubKindTT<'s, 't>> for KindT<'s, 't> {
  fn from(s: ISubKindTT<'s, 't>) -> Self {
    match s {
      ISubKindTT::Struct(x) => KindT::Struct(x),
      ISubKindTT::Interface(x) => KindT::Interface(x),
      ISubKindTT::KindPlaceholder(x) => KindT::KindPlaceholder(x),
    }
  }
  /* Guardian: disable-all */
}
impl<'s, 't> From<ISuperKindTT<'s, 't>> for KindT<'s, 't> {
  fn from(s: ISuperKindTT<'s, 't>) -> Self {
    match s {
      ISuperKindTT::Interface(x) => KindT::Interface(x),
      ISuperKindTT::KindPlaceholder(x) => KindT::KindPlaceholder(x),
    }
  }
  /* Guardian: disable-all */
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
  /* Guardian: disable-all */
}
impl<'s, 't> TryFrom<KindT<'s, 't>> for ISubKindTT<'s, 't> {
  type Error = ();
  fn try_from(k: KindT<'s, 't>) -> Result<Self, ()> {
    match k {
      KindT::Struct(x) => Ok(ISubKindTT::Struct(x)),
      KindT::Interface(x) => Ok(ISubKindTT::Interface(x)),
      KindT::KindPlaceholder(x) => Ok(ISubKindTT::KindPlaceholder(x)),
      _ => Err(()),
    }
  }
  /* Guardian: disable-all */
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
  /* Guardian: disable-all */
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
  /* Guardian: disable-all */
}
