// From Frontend/FinalAST/src/dev/vale/finalast/types.scala
//
// H-side output types for the simplifying pass. Mirrors src/instantiating/ast/types.rs
// pattern: Polyvalue + Interned compounds + Val pairs + dispatch enum. Lifetimes
// are `<'s, 'h>` with `where 's: 'h` (one region mode; H-side is post-collapse).

use std::marker::PhantomData;

use crate::final_ast::ast::{IdH, PrototypeH};
use crate::interner::StrI;
use crate::simplifying::hammer_interner::MustIntern;
use crate::simplifying::hammer_interner::HammerInterner;
use crate::utils::code_hierarchy::FileCoordinate;
use crate::utils::code_hierarchy::PackageCoordinate;

/*
package dev.vale.finalast

import dev.vale.{FileCoordinate, Interner, Keywords, PackageCoordinate, vassert, vfail, vimpl}
*/

// mig: case class CoordH[+T <: KindHT]
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CoordH<'s, 'h> where 's: 'h {
    pub ownership: OwnershipH,
    pub location: LocationH,
    pub kind: KindHT<'s, 'h>,
}
/*
// Represents a reference type.
// A reference contains these things:
// - The kind; the thing that this reference points at.
// - The ownership; this reference's relationship to the kind. This can be:
//   - Share, which means the references all share ownership of the kind. This
//     means that the kind will only be deallocated once all references to it are
//     gone. Share references can only point at immutable kinds, and immutable
//     kinds can *only* be pointed at by share references.
//   - Owning, which means this reference owns the object, and when this reference
//     disappears (without being moved), the object should disappear (this is taken
//     care of by the typing stage). Owning refs can only point at mutable kinds.
//   - Constraint, which means this reference doesn't own the kind. The kind
//     is guaranteed not to die while this constraint ref is active (indeed if it did
//     the program would panic). Constraint refs can only point at mutable kinds.
//   - Raw, which is a weird ownership and should go away. We point at Void with this.
//     TODO: Get rid of raw.
//   - (in the future) Weak, which is a reference that will null itself out when the
//     kind is destroyed. Weak refs can only point at mutable kinds.
// - Permission, how one can modify the object through this reference.
//   - Readonly, we cannot modify the object through this reference.
//   - Readwrite, we can.
// - (in the future) Location, either inline or yonder. Inline means that this reference
//   isn't actually a pointer, it's just the value itself, like C's Car vs Car*.
// In previous stages, this is referred to as a "coord", because these four things can be
// thought of as dimensions of a coordinate.
case class CoordH[+T <: KindHT](
    ownership: OwnershipH,
    location: LocationH,
    kind: T) {
  val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash;

  (ownership, location, kind) match {
    case (OwnH, YonderH, _) =>
    case (OwnH, InlineH, OpaqueHT(_, _, _)) =>
    case (ImmutableShareH | MutableShareH, _, _) =>
    case (MutableBorrowH | ImmutableBorrowH, YonderH, _) =>
    case (WeakH, YonderH, _) =>
    case _ => vfail()
  }

  kind match {
    case IntHT(_) | BoolHT() | FloatHT() | NeverHT(_) => {
      // Make sure that if we're pointing at a primitives, it's via a Share reference.
      // We don't want any ImmutableShareH, it's better to only ever have one ownership for
      // primitives.
      vassert(ownership == MutableShareH)
      vassert(location == InlineH)
    }
    case StrHT() => {
      // Strings need to be yonder because Midas needs to do refcounting for them.
      vassert(ownership == ImmutableShareH || ownership == MutableShareH)
      vassert(location == YonderH)
    }
    case s @ OpaqueHT(_, _, _) => {
      vassert(ownership == ImmutableShareH || ownership == MutableShareH)
      vassert(location == InlineH)
    }
    case s @ StructHT(name) => {
      val isBox = name.fullyQualifiedName.startsWith("__Box")

      if (isBox) {
        vassert(ownership == OwnH || ownership == ImmutableBorrowH || ownership == MutableBorrowH)
      }
    }
    case _ =>
  }

  // Convenience function for casting this to a Reference which the compiler knows
  // points at a static sized array.
  def expectStaticSizedArrayCoord() = {
    kind match {
      case atH @ StaticSizedArrayHT(_) => CoordH[StaticSizedArrayHT](ownership, location, atH)
    }
  }
  // Convenience function for casting this to a Reference which the compiler knows
  // points at an unstatic sized array.
  def expectRuntimeSizedArrayCoord() = {
    kind match {
      case atH @ RuntimeSizedArrayHT(_) => CoordH[RuntimeSizedArrayHT](ownership, location, atH)
    }
  }
  // Convenience function for casting this to a Reference which the compiler knows
  // points at struct.
  def expectStructCoord() = {
    kind match {
      case atH @ StructHT(_) => CoordH[StructHT](ownership, location, atH)
    }
  }
  // Convenience function for casting this to a Reference which the compiler knows
  // points at interface.
  def expectInterfaceCoord() = {
    kind match {
      case atH @ InterfaceHT(_) => CoordH[InterfaceHT](ownership, location, atH)
    }
  }
}
*/

// mig: fn expect_static_sized_array_coord
impl<'s, 'h> CoordH<'s, 'h> where 's: 'h {
    pub fn expect_static_sized_array_coord(&self) -> Self {
        match self.kind {
            KindHT::StaticSizedArrayHT(_) => *self,
            _ => panic!("expect_static_sized_array_coord: not a static sized array"),
        }
    }
// mig: fn expect_runtime_sized_array_coord
    pub fn expect_runtime_sized_array_coord(&self) -> Self {
        match self.kind {
            KindHT::RuntimeSizedArrayHT(_) => *self,
            _ => panic!("expect_runtime_sized_array_coord: not a runtime sized array"),
        }
    }
// mig: fn expect_struct_coord
    pub fn expect_struct_coord(&self) -> Self {
        match self.kind {
            KindHT::StructHT(_) => *self,
            _ => panic!("expect_struct_coord: not a struct"),
        }
    }
// mig: fn expect_interface_coord
    pub fn expect_interface_coord(&self) -> Self {
        match self.kind {
            KindHT::InterfaceHT(_) => *self,
            _ => panic!("expect_interface_coord: not an interface"),
        }
    }
}

impl<'s, 'h> KindHT<'s, 'h> where 's: 'h {
    pub fn expect_struct_h(&self) -> &'h StructHT<'s, 'h> {
        match *self {
            KindHT::StructHT(s) => s,
            _ => panic!("expect_struct_h: not a struct"),
        }
    }
    pub fn expect_static_sized_array_ht(&self) -> &'h StaticSizedArrayHT<'s, 'h> {
        match *self {
            KindHT::StaticSizedArrayHT(s) => s,
            _ => panic!("expect_static_sized_array_ht: not a static sized array"),
        }
    }
    pub fn expect_runtime_sized_array_ht(&self) -> &'h RuntimeSizedArrayHT<'s, 'h> {
        match *self {
            KindHT::RuntimeSizedArrayHT(s) => s,
            _ => panic!("expect_runtime_sized_array_ht: not a runtime sized array"),
        }
    }
}
// mig: sealed trait KindHT
/// Polyvalue
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum KindHT<'s, 'h> where 's: 'h {
    IntHT(IntHT),
    VoidHT(VoidHT),
    OpaqueHT(&'h OpaqueHT<'s, 'h>),
    BoolHT(BoolHT),
    StrHT(StrHT),
    FloatHT(FloatHT),
    NeverHT(NeverHT),
    InterfaceHT(&'h InterfaceHT<'s, 'h>),
    StructHT(&'h StructHT<'s, 'h>),
    StaticSizedArrayHT(&'h StaticSizedArrayHT<'s, 'h>),
    RuntimeSizedArrayHT(&'h RuntimeSizedArrayHT<'s, 'h>),
}
/*
// A value, a thing that can be pointed at. See ReferenceH for more information.
sealed trait KindHT {
  def packageCoord(interner: Interner, keywords: Keywords): PackageCoordinate
//  def isRustOpaqueType(): Boolean
}
*/

// mig: object IntHT
/*
object IntHT {
  val i32 = IntHT(32)
}
*/

// mig: case class IntHT
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IntHT {
    pub bits: i32,
}
/*
case class IntHT(bits: Int) extends KindHT {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def packageCoord(interner: Interner, keywords: Keywords): PackageCoordinate = PackageCoordinate.BUILTIN(interner, keywords)
//  override def isRustOpaqueType(): Boolean = false
}
*/

// mig: case class VoidHT
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct VoidHT;
/*
case class VoidHT() extends KindHT {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def packageCoord(interner: Interner, keywords: Keywords): PackageCoordinate = PackageCoordinate.BUILTIN(interner, keywords)
//  override def isRustOpaqueType(): Boolean = false
}
*/

// mig: case class OpaqueHT
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OpaqueHT<'s, 'h> where 's: 'h {
    pub package_coord: PackageCoordinate<'s>,
    pub struct_id: &'h IdH<'s, 'h>,
    pub simple_id: SimpleId<'s, 'h>,
}
/*
case class OpaqueHT(
  packageCoord: PackageCoordinate,
  structId: IdH,
  simpleId: SimpleId
) extends KindHT {
  val hash = runtime.ScalaRunTime._hashCode(this); override def hashCode(): Int = hash;
  override def packageCoord(interner: Interner, keywords: Keywords): PackageCoordinate = packageCoord
//    override def isRustOpaqueType(): Boolean = false
}
*/

// mig: case class BoolHT
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BoolHT;
/*
case class BoolHT() extends KindHT {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def packageCoord(interner: Interner, keywords: Keywords): PackageCoordinate = PackageCoordinate.BUILTIN(interner, keywords)
//  override def isRustOpaqueType(): Boolean = false
}
*/

// mig: case class StrHT
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StrHT;
/*
case class StrHT() extends KindHT {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def packageCoord(interner: Interner, keywords: Keywords): PackageCoordinate = PackageCoordinate.BUILTIN(interner, keywords)
//  override def isRustOpaqueType(): Boolean = false
}
*/

// mig: case class FloatHT
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FloatHT;
/*
case class FloatHT() extends KindHT {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def packageCoord(interner: Interner, keywords: Keywords): PackageCoordinate = PackageCoordinate.BUILTIN(interner, keywords)
//  override def isRustOpaqueType(): Boolean = false
}
*/

// mig: case class NeverHT
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct NeverHT {
    pub from_break: bool,
}
/*
// A primitive which can never be instantiated. If something returns this, it
// means that it will never actually return. For example, the return type of
// __panic() is a NeverH.
// TODO: This feels weird being a kind in metal. Figure out a way to not
// have this? Perhaps replace all kinds with Optional[Optional[KindH]],
// where None is never, Some(None) is Void, and Some(Some(_)) is a normal thing.
case class NeverHT(fromBreak: Boolean) extends KindHT {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def packageCoord(interner: Interner, keywords: Keywords): PackageCoordinate = PackageCoordinate.BUILTIN(interner, keywords)
//  override def isRustOpaqueType(): Boolean = false
}
*/

// mig: case class InterfaceHT
/// Interning permanent (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceHT<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s, 'h>,
    pub _must_intern: MustIntern,
}

// mig: case class InterfaceHT (transient companion)
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceHTValH<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s, 'h>,
}
/*
case class InterfaceHT(
  // Unique identifier for the interface.
  id: IdH
) extends KindHT {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def packageCoord(interner: Interner, keywords: Keywords): PackageCoordinate = id.packageCoordinate
//  override def isRustOpaqueType(): Boolean = false
}
*/

// mig: case class StructHT
/// Interning permanent (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructHT<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s, 'h>,
    pub _must_intern: MustIntern,
}

// mig: case class StructHT (transient companion)
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructHTValH<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s, 'h>,
}
/*
case class StructHT(
  // Unique identifier for the interface.
  id: IdH
) extends KindHT {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def packageCoord(interner: Interner, keywords: Keywords): PackageCoordinate = id.packageCoordinate
//  override def isRustOpaqueType(): Boolean = id.packageCoordinate.module.str == "rust"
}
*/

// mig: case class StaticSizedArrayHT
/// Interning permanent (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayHT<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s, 'h>,
    pub _must_intern: MustIntern,
}

// mig: case class StaticSizedArrayHT (transient companion)
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayHTValH<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s, 'h>,
}
/*
// An array whose size is known at compile time, and therefore doesn't need to
// carry around its size at runtime.
case class StaticSizedArrayHT(
  // This is useful for naming the Midas struct that wraps this array and its ref count.
  id: IdH,
) extends KindHT {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def packageCoord(interner: Interner, keywords: Keywords): PackageCoordinate = id.packageCoordinate
//  override def isRustOpaqueType(): Boolean = false
}
*/

// mig: case class StaticSizedArrayDefinitionHT
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayDefinitionHT<'s, 'h> where 's: 'h {
    pub name: &'h IdH<'s, 'h>,
    pub size: i64,
    pub mutability: Mutability,
    pub variability: Variability,
    pub element_type: CoordH<'s, 'h>,
}
// mig: fn kind (on StaticSizedArrayDefinitionHT — see Scala `def kind = StaticSizedArrayHT(name)` in audit block below)
impl<'s, 'h> StaticSizedArrayDefinitionHT<'s, 'h> where 's: 'h {
    pub fn kind(&self, interner: &HammerInterner<'s, 'h>) -> &'h StaticSizedArrayHT<'s, 'h> {
        interner.intern_static_sized_array_ht(StaticSizedArrayHTValH { id: self.name })
    }
}
/*
// An array whose size is known at compile time, and therefore doesn't need to
// carry around its size at runtime.
case class StaticSizedArrayDefinitionHT(
  // This is useful for naming the Midas struct that wraps this array and its ref count.
  name: IdH,
  // The size of the array.
  size: Long,
  mutability: Mutability,
  variability: Variability,
  elementType: CoordH[KindHT]
) {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  def kind = StaticSizedArrayHT(name)
}
*/

// mig: case class RuntimeSizedArrayHT
/// Interning permanent (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayHT<'s, 'h> where 's: 'h {
    pub name: &'h IdH<'s, 'h>,
    pub _must_intern: MustIntern,
}

// mig: case class RuntimeSizedArrayHT (transient companion)
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayHTValH<'s, 'h> where 's: 'h {
    pub name: &'h IdH<'s, 'h>,
}
/*
case class RuntimeSizedArrayHT(
  // This is useful for naming the Midas struct that wraps this array and its ref count.
  name: IdH,
) extends KindHT {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def packageCoord(interner: Interner, keywords: Keywords): PackageCoordinate = name.packageCoordinate
//  override def isRustOpaqueType(): Boolean = false
}
*/

// mig: case class RuntimeSizedArrayDefinitionHT
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayDefinitionHT<'s, 'h> where 's: 'h {
    pub name: &'h IdH<'s, 'h>,
    pub mutability: Mutability,
    pub element_type: CoordH<'s, 'h>,
}
// mig: fn kind (on RuntimeSizedArrayDefinitionHT — see Scala `def kind = RuntimeSizedArrayHT(name)` in audit block below)
impl<'s, 'h> RuntimeSizedArrayDefinitionHT<'s, 'h> where 's: 'h {
    pub fn kind(&self, interner: &HammerInterner<'s, 'h>) -> &'h RuntimeSizedArrayHT<'s, 'h> {
        interner.intern_runtime_sized_array_ht(RuntimeSizedArrayHTValH { name: self.name })
    }
}
/*
case class RuntimeSizedArrayDefinitionHT(
  // This is useful for naming the Midas struct that wraps this array and its ref count.
  name: IdH,
  mutability: Mutability,
  elementType: CoordH[KindHT]
) {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  def kind = RuntimeSizedArrayHT(name)
}
*/

// mig: case class CodeLocation
/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct CodeLocation<'s> {
    pub file: FileCoordinate<'s>,
    pub offset: i32,
}
/*
// Place in the original source code that something came from. Useful for uniquely
// identifying templates.
case class CodeLocation(
  file: FileCoordinate,
  offset: Int) {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/

// mig: sealed trait OwnershipH
// mig: case objects OwnH, MutableBorrowH, ImmutableBorrowH, MutableShareH, ImmutableShareH, WeakH
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum OwnershipH {
    OwnH,
    MutableBorrowH,
    ImmutableBorrowH,
    MutableShareH,
    ImmutableShareH,
    WeakH,
}
/*
// Ownership is the way a reference relates to the kind's lifetime, see
// ReferenceH for explanation.
sealed trait OwnershipH
case object OwnH extends OwnershipH
case object MutableBorrowH extends OwnershipH
case object ImmutableBorrowH extends OwnershipH
case object MutableShareH extends OwnershipH
case object ImmutableShareH extends OwnershipH
case object WeakH extends OwnershipH
*/

// mig: sealed trait LocationH
// mig: case objects InlineH, YonderH
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum LocationH {
    InlineH,
    YonderH,
}
/*
// Location says whether a reference contains the kind's location (yonder) or
// contains the kind itself (inline).
// Yes, it's weird to consider a reference containing a kind, but it makes a
// lot of things simpler for the language.
// Examples (with C++ translations):
//   This will create a variable `car` that lives on the stack ("inline"):
//     Vale: car = inl Car(4, "Honda Civic");
//     C++:  Car car(4, "Honda Civic");
//   This will create a variable `car` that lives on the heap ("yonder"):
//     Vale: car = Car(4, "Honda Civic");
//     C++:  Car* car = new Car(4, "Honda Civic");
//   This will create a struct Spaceship whose engine and reactor are allocated
//   separately somewhere else on the heap (yonder):
//     Vale: struct Car { engine Engine; reactor Reactor; }
//     C++:  class Car { Engine* engine; Reactor* reactor; }
//   This will create a struct Spaceship whose engine and reactor are embedded
//   into its own memory (inline):
//     Vale: struct Car { engine inl Engine; reactor inl Reactor; }
//     C++:  class Car { Engine engine; Reactor reactor; }
// Note that the compiler will often automatically add an `inl` onto whatever
// local variables it can, to speed up the program.
sealed trait LocationH
// Means that the kind will be in the containing stack frame or struct.
case object InlineH extends LocationH
// Means that the kind will be allocated separately, in the heap.
case object YonderH extends LocationH
*/

// mig: sealed trait Mutability
// mig: case objects Immutable, Mutable
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Mutability {
    Immutable,
    Mutable,
}
/*
// Used to say whether an object can be modified or not.
// Structs and interfaces specify whether theyre immutable or mutable, but all
// primitives are immutable (after all, you can't change 4 itself to be another
// number).
sealed trait Mutability
// Immutable structs can only contain or point at other immutable structs, in
// other words, something immutable is *deeply* immutable.
// Immutable things can only be referred to with Share references.
case object Immutable extends Mutability
// Mutable objects have a lifetime.
case object Mutable extends Mutability
*/

// mig: sealed trait Variability
// mig: case objects Final, Varying
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Variability {
    Final,
    Varying,
}
/*
// Used to say whether a variable (or member) reference can be changed to point
// at something else.
// Examples (with C++ translations):
//   This will create a varying local, which can be changed to point elsewhere:
//     Vale:
//       x = Car(4, "Honda Civic");
//       set x = someOtherCar;
//       set x = Car(4, "Toyota Camry");
//     C++:
//       Car* x = new Car(4, "Honda Civic");
//       x = someOtherCar;
//       x = new Car(4, "Toyota Camry");
//   This will create a final local, which can't be changed to point elsewhere:
//     Vale: x = Car(4, "Honda Civic");
//     C++:  Car* const x = new Car(4, "Honda Civic");
//   Note the position of the const, which says that the pointer cannot change,
//   but we can still change the members of the Car, which is also true in Vale:
//     Vale:
//       x = Car(4, "Honda Civic");
//       mut x.numWheels = 6;
//     C++:
//       Car* const x = new Car(4, "Honda Civic");
//       x->numWheels = 6;
// In other words, variability affects whether the variable (or member) can be
// changed to point at something different, but it doesn't affect whether we can
// change anything inside the kind (this reference's permission and the
// kind struct's member's variability affect that).
sealed trait Variability
case object Final extends Variability
case object Varying extends Variability
*/

// mig: case class SimpleId
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SimpleId<'s, 'h> where 's: 'h {
    pub steps: &'h [SimpleIdStep<'s, 'h>],
}
/*
case class SimpleId(steps: Vector[SimpleIdStep]) {
  vassert(steps.length > 0)
}
*/

// mig: case class SimpleIdStep
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SimpleIdStep<'s, 'h> where 's: 'h {
    pub name: StrI<'s>,
    pub template_args: &'h [SimpleId<'s, 'h>],
}
/*
case class SimpleIdStep(
    name: String, // Could also be & or &mut
    templateArgs: Vector[SimpleId])
*/

// mig: case class HamutsFunctionExtern
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct HamutsFunctionExtern<'s, 'h> where 's: 'h {
    pub maybe_extern_name: StrI<'s>,
    pub prototype: &'h PrototypeH<'s, 'h>,
    pub simple_id: SimpleId<'s, 'h>,
}
/*
case class HamutsFunctionExtern(
    maybeExternName: String,
    prototype: PrototypeH,
    simpleId: SimpleId)
*/

// mig: case class HamutsKindExtern
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct HamutsKindExtern<'s, 'h> where 's: 'h {
    pub maybe_extern_name: StrI<'s>,
    pub kind: KindHT<'s, 'h>,
    pub simple_id: SimpleId<'s, 'h>,
}
/*
case class HamutsKindExtern(
    maybeExternName: String,
    kind: KindHT,
    simpleId: SimpleId)
*/

// --- Dispatch enums for the kind-payload interner family ---

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum InternedKindPayloadValH<'s, 'h> where 's: 'h {
    StructHT(StructHTValH<'s, 'h>),
    InterfaceHT(InterfaceHTValH<'s, 'h>),
    StaticSizedArrayHT(StaticSizedArrayHTValH<'s, 'h>),
    RuntimeSizedArrayHT(RuntimeSizedArrayHTValH<'s, 'h>),
}

/// Interning permanent (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum InternedKindPayloadH<'s, 'h> where 's: 'h {
    StructHT(&'h StructHT<'s, 'h>),
    InterfaceHT(&'h InterfaceHT<'s, 'h>),
    StaticSizedArrayHT(&'h StaticSizedArrayHT<'s, 'h>),
    RuntimeSizedArrayHT(&'h RuntimeSizedArrayHT<'s, 'h>),
}

// Suppress unused-PhantomData warning if any helper needs it.
#[allow(dead_code)]
type _PhantomS<'s, 'h> = PhantomData<(&'s (), &'h ())>;
