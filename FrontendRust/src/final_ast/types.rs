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



// mig: case class CoordH[+T <: KindHT]
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CoordH<'s, 'h> where 's: 'h {
    pub ownership: OwnershipH,
    pub location: LocationH,
    pub kind: KindHT<'s, 'h>,
}


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


// mig: object IntHT


// mig: case class IntHT
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IntHT {
    pub bits: i32,
}


// mig: case class VoidHT
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct VoidHT;


// mig: case class OpaqueHT
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OpaqueHT<'s, 'h> where 's: 'h {
    pub package_coord: PackageCoordinate<'s>,
    pub struct_id: &'h IdH<'s>,
    pub simple_id: SimpleId<'s, 'h>,
}


// mig: case class BoolHT
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BoolHT;


// mig: case class StrHT
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StrHT;


// mig: case class FloatHT
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct FloatHT;


// mig: case class NeverHT
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct NeverHT {
    pub from_break: bool,
}


// mig: case class InterfaceHT
/// Interning permanent (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceHT<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s>,
    pub _must_intern: MustIntern,
}

// mig: case class InterfaceHT (transient companion)
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceHTValH<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s>,
}


// mig: case class StructHT
/// Interning permanent (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructHT<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s>,
    pub _must_intern: MustIntern,
}

// mig: case class StructHT (transient companion)
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructHTValH<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s>,
}


// mig: case class StaticSizedArrayHT
/// Interning permanent (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayHT<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s>,
    pub _must_intern: MustIntern,
}

// mig: case class StaticSizedArrayHT (transient companion)
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayHTValH<'s, 'h> where 's: 'h {
    pub id: &'h IdH<'s>,
}


// mig: case class StaticSizedArrayDefinitionHT
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayDefinitionHT<'s, 'h> where 's: 'h {
    pub name: &'h IdH<'s>,
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


// mig: case class RuntimeSizedArrayHT
/// Interning permanent (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayHT<'s, 'h> where 's: 'h {
    pub name: &'h IdH<'s>,
    pub _must_intern: MustIntern,
}

// mig: case class RuntimeSizedArrayHT (transient companion)
/// Interning transient (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayHTValH<'s, 'h> where 's: 'h {
    pub name: &'h IdH<'s>,
}


// mig: case class RuntimeSizedArrayDefinitionHT
/// Temporary state
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayDefinitionHT<'s, 'h> where 's: 'h {
    pub name: &'h IdH<'s>,
    pub mutability: Mutability,
    pub element_type: CoordH<'s, 'h>,
}
// mig: fn kind (on RuntimeSizedArrayDefinitionHT — see Scala `def kind = RuntimeSizedArrayHT(name)` in audit block below)
impl<'s, 'h> RuntimeSizedArrayDefinitionHT<'s, 'h> where 's: 'h {
    pub fn kind(&self, interner: &HammerInterner<'s, 'h>) -> &'h RuntimeSizedArrayHT<'s, 'h> {
        interner.intern_runtime_sized_array_ht(RuntimeSizedArrayHTValH { name: self.name })
    }
}


// mig: case class CodeLocation
/// Temporary state
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct CodeLocation<'s> {
    pub file: FileCoordinate<'s>,
    pub offset: i32,
}


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


// mig: sealed trait LocationH
// mig: case objects InlineH, YonderH
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum LocationH {
    InlineH,
    YonderH,
}


// mig: sealed trait Mutability
// mig: case objects Immutable, Mutable
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Mutability {
    Immutable,
    Mutable,
}


// mig: sealed trait Variability
// mig: case objects Final, Varying
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Variability {
    Final,
    Varying,
}


// mig: case class SimpleId
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SimpleId<'s, 'h> where 's: 'h {
    pub steps: &'h [SimpleIdStep<'s, 'h>],
}


// mig: case class SimpleIdStep
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct SimpleIdStep<'s, 'h> where 's: 'h {
    pub name: StrI<'s>,
    pub template_args: &'h [SimpleId<'s, 'h>],
}


// mig: case class HamutsFunctionExtern
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct HamutsFunctionExtern<'s, 'h> where 's: 'h {
    pub maybe_extern_name: StrI<'s>,
    pub prototype: &'h PrototypeH<'s, 'h>,
    pub simple_id: SimpleId<'s, 'h>,
}


// mig: case class HamutsKindExtern
/// Value-type
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct HamutsKindExtern<'s, 'h> where 's: 'h {
    pub maybe_extern_name: StrI<'s>,
    pub kind: KindHT<'s, 'h>,
    pub simple_id: SimpleId<'s, 'h>,
}


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
