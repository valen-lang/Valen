use std::marker::PhantomData;

use crate::instantiating::ast::names::{IdI, IInterfaceNameI, IStructNameI, RuntimeSizedArrayNameI, StaticSizedArrayNameI};
use crate::instantiating::instantiating_interner::MustIntern;
use crate::instantiating::ast::names::INameI;
use crate::instantiating::ast::templata::CoordTemplataI;


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

// mig: case object ImmutableShareI

// mig: case object MutableShareI

// mig: case object OwnI

// mig: case object WeakI

// mig: case object ImmutableBorrowI

// mig: case object MutableBorrowI

// mig: enum MutabilityI
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MutabilityI {
  Mutable,
  Immutable,
}
// mig: impl MutabilityI

// mig: case object MutableI

// mig: case object ImmutableI

// mig: enum VariabilityI
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum VariabilityI {
  Final,
  Varying,
}
// mig: impl VariabilityI

// mig: case object FinalI

// mig: case object VaryingI

// mig: enum LocationI
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum LocationI {
  Inline,
  Yonder,
}
// mig: impl LocationI

// mig: case object InlineI

// mig: case object YonderI


// mig: fn void
impl<'s, 'i> CoordI<'s, 'i> where 's: 'i {
  pub fn void() -> CoordI<'s, 'i> {
    panic!("Unimplemented: void");
    // CoordI[R](MutableShareI, VoidIT())
  }
}


// mig: struct CoordI
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CoordI<'s, 'i> where 's: 'i {
  pub ownership: OwnershipI,
  pub kind: KindIT<'s, 'i>,
}
// mig: impl CoordI

// mig: enum KindIT
// Per Scala parity: primitives (NeverIT, VoidIT, IntIT, BoolIT, StrIT, FloatIT)
// are TFITCX Value-type and held inline; the 4 compound payloads
// (StaticSizedArrayIT, RuntimeSizedArrayIT, StructIT, InterfaceIT) are arena-
// interned and held by `&'i` ref (see @WVSBIZ).
/// Polyvalue (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum KindIT<'s, 'i> where 's: 'i {
  NeverIT(NeverIT),
  VoidIT(VoidIT),
  IntIT(IntIT),
  BoolIT(BoolIT),
  StrIT(StrIT),
  FloatIT(FloatIT),
  StaticSizedArrayIT(&'i StaticSizedArrayIT<'s, 'i>),
  RuntimeSizedArrayIT(&'i RuntimeSizedArrayIT<'s, 'i>),
  StructIT(&'i StructIT<'s, 'i>),
  InterfaceIT(&'i InterfaceIT<'s, 'i>),
}
// mig: impl KindIT

// mig: fn is_primitive
impl<'s, 'i> KindIT<'s, 'i> where 's: 'i {
  pub fn is_primitive(&self) -> bool {
    panic!("Unimplemented: is_primitive");
    // abstract method (each KindIT case overrides; primitives true, citizens/arrays false)
  }

// mig: fn expect_citizen
  pub fn expect_citizen(&self) -> ICitizenIT<'s, 'i> {
    panic!("Unimplemented: expect_citizen");
    // this match { case c : ICitizenIT[R] => c; case _ => vfail() }
  }

// mig: fn expect_interface
  pub fn expect_interface(&self) -> &'i InterfaceIT<'s, 'i> {
    match self {
      KindIT::InterfaceIT(c) => c,
      _ => panic!("expect_interface: not an interface"),
    }
  }

// mig: fn expect_struct
  pub fn expect_struct(&self) -> &'i StructIT<'s, 'i> {
    panic!("Unimplemented: expect_struct");
    // this match { case c @ StructIT(_) => c; case _ => vfail() }
  }
}

// mig: struct NeverIT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct NeverIT {
  pub from_break: bool,
}
// mig: impl NeverIT

// mig: struct VoidIT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct VoidIT {
}
// mig: impl VoidIT

// mig: struct IntIT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IntIT {
  pub bits: i32,
}
// mig: impl IntIT

// mig: struct BoolIT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BoolIT {
}
// mig: impl BoolIT

// mig: struct StrIT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StrIT {
}
// mig: impl StrIT

// mig: struct FloatIT
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FloatIT {
}
// mig: impl FloatIT

// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<StaticSizedArrayIT> for ...` or inline match.)


// mig: struct StaticSizedArrayIT
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayIT<'s, 'i> where 's: 'i {
  pub name: IdI<'s, 'i>,
  pub _must_intern: MustIntern,
}

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayITValI<'s, 'i> where 's: 'i {
  pub name: IdI<'s, 'i>,
}
// mig: impl StaticSizedArrayIT
impl<'s, 'i> StaticSizedArrayIT<'s, 'i> where 's: 'i {
  pub fn mutability(self) -> MutabilityI {
    match self.name.local_name {
      INameI::StaticSizedArray(n) => n.arr.mutability,
      _ => panic!("StaticSizedArrayIT::mutability: name.local_name is not StaticSizedArrayNameI"),
    }
  }
  pub fn element_type(self) -> CoordTemplataI<'s, 'i> {
    match self.name.local_name {
      INameI::StaticSizedArray(n) => n.arr.element_type,
      _ => panic!("StaticSizedArrayIT::element_type: name.local_name is not StaticSizedArrayNameI"),
    }
  }
  pub fn size(self) -> i64 {
    match self.name.local_name {
      INameI::StaticSizedArray(n) => n.size,
      _ => panic!("StaticSizedArrayIT::size: name.local_name is not StaticSizedArrayNameI"),
    }
  }
  pub fn variability(self) -> VariabilityI {
    match self.name.local_name {
      INameI::StaticSizedArray(n) => n.variability,
      _ => panic!("StaticSizedArrayIT::variability: name.local_name is not StaticSizedArrayNameI"),
    }
  }
}

// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<RuntimeSizedArrayIT> for ...` or inline match.)


// mig: struct RuntimeSizedArrayIT
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayIT<'s, 'i> where 's: 'i {
  pub name: IdI<'s, 'i>,
  pub _must_intern: MustIntern,
}

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayITValI<'s, 'i> where 's: 'i {
  pub name: IdI<'s, 'i>,
}
// mig: impl RuntimeSizedArrayIT
impl<'s, 'i> RuntimeSizedArrayIT<'s, 'i> where 's: 'i {
  pub fn mutability(self) -> MutabilityI {
    match self.name.local_name {
      INameI::RuntimeSizedArray(n) => n.arr.mutability,
      _ => panic!("RuntimeSizedArrayIT::mutability: name.local_name is not RuntimeSizedArrayNameI"),
    }
  }
  pub fn element_type(self) -> CoordTemplataI<'s, 'i> {
    match self.name.local_name {
      INameI::RuntimeSizedArray(n) => n.arr.element_type,
      _ => panic!("RuntimeSizedArrayIT::element_type: name.local_name is not RuntimeSizedArrayNameI"),
    }
  }
}

// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom<ICitizenIT> for ...` or inline match.)


// mig: enum ISubKindIT
/// Polyvalue (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ISubKindIT<'s, 'i> where 's: 'i {
  StructIT(&'i StructIT<'s, 'i>),
  InterfaceIT(&'i InterfaceIT<'s, 'i>),
}
// mig: impl ISubKindIT

// mig: enum ICitizenIT
/// Polyvalue (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ICitizenIT<'s, 'i> where 's: 'i {
  StructIT(&'i StructIT<'s, 'i>),
  InterfaceIT(&'i InterfaceIT<'s, 'i>),
}
// mig: impl ICitizenIT
// mig: fn id (Scala `def id: IdI[R, ICitizenNameI[R]]` on trait)
impl<'s, 'i> ICitizenIT<'s, 'i> where 's: 'i {
    pub fn id(&self) -> IdI<'s, 'i> {
        match self {
            ICitizenIT::StructIT(s) => s.id,
            ICitizenIT::InterfaceIT(i) => i.id,
        }
    }
}

// mig: struct StructIT
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructIT<'s, 'i> where 's: 'i {
  pub id: IdI<'s, 'i>,
  pub _must_intern: MustIntern,
}

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructITValI<'s, 'i> where 's: 'i {
  pub id: IdI<'s, 'i>,
}
// mig: impl StructIT

// mig: struct InterfaceIT
/// Interned (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceIT<'s, 'i> where 's: 'i {
  pub id: IdI<'s, 'i>,
  pub _must_intern: MustIntern,
}

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceITValI<'s, 'i> where 's: 'i {
  pub id: IdI<'s, 'i>,
}
// mig: impl InterfaceIT

// -- Union enums for the Kind-payload interning family ---------------------
// Per architect Slab 16b: kind payloads dispatch through a tagged-union pair.
// R stays phantom — the actual per-(type×region-mode) family separation lives
// in the interner (3 HashMaps per logical type, one per region mode).

/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum InternedKindPayloadValI<'s, 'i> where 's: 'i {
  StructIT(StructITValI<'s, 'i>),
  InterfaceIT(InterfaceITValI<'s, 'i>),
  StaticSizedArrayIT(StaticSizedArrayITValI<'s, 'i>),
  RuntimeSizedArrayIT(RuntimeSizedArrayITValI<'s, 'i>),
}

/// Polyvalue (see @TFITCX)
#[derive(Copy, Clone, Hash, PartialEq, Eq, Debug)]
pub enum InternedKindPayloadI<'s, 'i> where 's: 'i {
  StructIT(&'i StructIT<'s, 'i>),
  InterfaceIT(&'i InterfaceIT<'s, 'i>),
  StaticSizedArrayIT(&'i StaticSizedArrayIT<'s, 'i>),
  RuntimeSizedArrayIT(&'i RuntimeSizedArrayIT<'s, 'i>),
}
