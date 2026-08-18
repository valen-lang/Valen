use std::marker::PhantomData;

use crate::instantiating::ast::names::{IdI, IInterfaceNameI, IStructNameI, RuntimeSizedArrayNameI, StaticSizedArrayNameI};
use crate::instantiating::ast::names::INameI;



/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum SharednessI {
  Single,
  Shared,
}


// The onion "wrap" layers. Ownership is which wrap surrounds the base kind — or none: an owned
// value is a bare kind with zero wraps (an owned Ship is KindIT::StructIT(..) directly). Mirrors
// typing's BorrowRefT/OwnRefT/ShareRefT/WeakRefT. Per BCHATZ there is no region/group here.

/// Polyvalue (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BorrowRefIT<'s, 'i> where 's: 'i {
  pub inner: KindIT<'s, 'i>,
}

/// Polyvalue (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OwnRefIT<'s, 'i> where 's: 'i {
  pub inner: KindIT<'s, 'i>,
}

/// Polyvalue (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ShareRefIT<'s, 'i> where 's: 'i {
  pub inner: KindIT<'s, 'i>,
}

/// Polyvalue (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct WeakRefIT<'s, 'i> where 's: 'i {
  pub inner: KindIT<'s, 'i>,
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
  USizeIT(USizeIT),
  StaticSizedArrayIT(&'i StaticSizedArrayIT<'s, 'i>),
  RuntimeSizedArrayIT(&'i RuntimeSizedArrayIT<'s, 'i>),
  StructIT(&'i StructIT<'s, 'i>),
  InterfaceIT(&'i InterfaceIT<'s, 'i>),
  BorrowRefIT(&'i BorrowRefIT<'s, 'i>),
  OwnRefIT(&'i OwnRefIT<'s, 'i>),
  ShareRefIT(&'i ShareRefIT<'s, 'i>),
  WeakRefIT(&'i WeakRefIT<'s, 'i>),
}



impl<'s, 'i> KindIT<'s, 'i> where 's: 'i {
  pub fn is_primitive(&self) -> bool {
    matches!(
      self,
      KindIT::NeverIT(_) | KindIT::VoidIT(_) | KindIT::IntIT(_) | KindIT::BoolIT(_)
        | KindIT::StrIT(_) | KindIT::FloatIT(_) | KindIT::USizeIT(_),
    )
  }


  pub fn expect_citizen(&self) -> ICitizenIT<'s, 'i> {
    match self {
      KindIT::StructIT(s) => ICitizenIT::StructIT(s),
      KindIT::InterfaceIT(i) => ICitizenIT::InterfaceIT(i),
      _ => panic!("expect_citizen: not a citizen"),
    }
  }


  pub fn expect_interface(&self) -> &'i InterfaceIT<'s, 'i> {
    match self {
      KindIT::InterfaceIT(c) => c,
      _ => panic!("expect_interface: not an interface"),
    }
  }


  pub fn expect_struct(&self) -> &'i StructIT<'s, 'i> {
    match self {
      KindIT::StructIT(s) => s,
      _ => panic!("expect_struct: not a struct"),
    }
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



/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct USizeIT {
}



/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayIT<'s, 'i> where 's: 'i {
  pub name: IdI<'s, 'i>,
}

impl<'s, 'i> StaticSizedArrayIT<'s, 'i> where 's: 'i {
  pub fn element_type(self) -> KindIT<'s, 'i> {
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
}



/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayIT<'s, 'i> where 's: 'i {
  pub name: IdI<'s, 'i>,
}

impl<'s, 'i> RuntimeSizedArrayIT<'s, 'i> where 's: 'i {
  pub fn element_type(self) -> KindIT<'s, 'i> {
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


/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructIT<'s, 'i> where 's: 'i {
  pub id: IdI<'s, 'i>,
}



/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceIT<'s, 'i> where 's: 'i {
  pub id: IdI<'s, 'i>,
}
