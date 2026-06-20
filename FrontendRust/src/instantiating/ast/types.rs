use std::marker::PhantomData;

use crate::instantiating::ast::names::{IdI, IInterfaceNameI, IStructNameI, RuntimeSizedArrayNameI, StaticSizedArrayNameI};
use crate::instantiating::instantiating_interner::MustIntern;
use crate::instantiating::ast::names::INameI;
use crate::instantiating::ast::templata::CoordTemplataI;


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


/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum MutabilityI {
  Mutable,
  Immutable,
}


/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum VariabilityI {
  Final,
  Varying,
}


/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum LocationI {
  Inline,
  Yonder,
}


impl<'s, 'i> CoordI<'s, 'i> where 's: 'i {
  pub fn void() -> CoordI<'s, 'i> {
    panic!("Unimplemented: void");
    // CoordI[R](MutableShareI, VoidIT())
  }
}


/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CoordI<'s, 'i> where 's: 'i {
  pub ownership: OwnershipI,
  pub kind: KindIT<'s, 'i>,
}

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

impl<'s, 'i> KindIT<'s, 'i> where 's: 'i {
  pub fn is_primitive(&self) -> bool {
    panic!("Unimplemented: is_primitive");
    // abstract method (each KindIT case overrides; primitives true, citizens/arrays false)
  }

  pub fn expect_citizen(&self) -> ICitizenIT<'s, 'i> {
    panic!("Unimplemented: expect_citizen");
    // this match { case c : ICitizenIT[R] => c; case _ => vfail() }
  }

  pub fn expect_interface(&self) -> &'i InterfaceIT<'s, 'i> {
    match self {
      KindIT::InterfaceIT(c) => c,
      _ => panic!("expect_interface: not an interface"),
    }
  }

  pub fn expect_struct(&self) -> &'i StructIT<'s, 'i> {
    panic!("Unimplemented: expect_struct");
    // this match { case c @ StructIT(_) => c; case _ => vfail() }
  }
}

/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct NeverIT {
  pub from_break: bool,
}

/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct VoidIT {
}

/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IntIT {
  pub bits: i32,
}

/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BoolIT {
}

/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StrIT {
}

/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FloatIT {
}



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



/// Polyvalue (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ISubKindIT<'s, 'i> where 's: 'i {
  StructIT(&'i StructIT<'s, 'i>),
  InterfaceIT(&'i InterfaceIT<'s, 'i>),
}

/// Polyvalue (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ICitizenIT<'s, 'i> where 's: 'i {
  StructIT(&'i StructIT<'s, 'i>),
  InterfaceIT(&'i InterfaceIT<'s, 'i>),
}
impl<'s, 'i> ICitizenIT<'s, 'i> where 's: 'i {
    pub fn id(&self) -> IdI<'s, 'i> {
        match self {
            ICitizenIT::StructIT(s) => s.id,
            ICitizenIT::InterfaceIT(i) => i.id,
        }
    }
}

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
