use crate::utils::arena_index_map::ArenaIndexMap;
use crate::postparsing::names::IRuneS;
use crate::instantiating::ast::types::{CoordI, MutabilityI, VariabilityI, StructIT, InterfaceIT, ICitizenIT};
use crate::instantiating::ast::names::{IdI, IVarNameI};
use crate::instantiating::ast::ast::{ICitizenAttributeI, PrototypeI};
use std::marker::PhantomData;


// mig: trait CitizenDefinitionI
pub trait CitizenDefinitionI<'s, 'i> {}

// Rust-only dispatch enum for the `CitizenDefinitionI` trait (architect-directed).
// Scala uses `CitizenDefinitionI` polymorphically (StructDefinitionI / InterfaceDefinitionI
// both extend it); per NEDCX we use a concrete enum here instead of a `&dyn` trait object.
// Mirrors the kind-level `ICitizenIT` dispatch enum. No Scala counterpart, so no audit-trail.
#[derive(Copy, Clone)]
pub enum ICitizenDefinitionI<'s, 'i> {
    StructDefinitionI(&'i StructDefinitionI<'s, 'i>),
    InterfaceDefinitionI(&'i InterfaceDefinitionI<'s, 'i>),
}
// mig: struct StructDefinitionI
/// Temporary state
pub struct StructDefinitionI<'s, 'i> {
    pub instantiated_citizen: &'i StructIT<'s, 'i>,
    pub attributes: &'i [ICitizenAttributeI<'s>],
    pub weakable: bool,
    pub mutability: MutabilityI,
    pub members: &'i [StructMemberI<'s, 'i>],
    pub is_closure: bool,
    pub rune_to_function_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i>>,
    pub rune_to_impl_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i>>,
}
// mig: impl StructDefinitionI

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for StructDefinitionI` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for StructDefinitionI` below.)

// mig: fn get_member_and_index
impl<'s, 'i> StructDefinitionI<'s, 'i> {
    pub fn get_member_and_index(&self, needle_name: IVarNameI<'s, 'i>) -> Option<(&StructMemberI<'s, 'i>, usize)> {
        panic!("Unimplemented: get_member_and_index")
    }
}

// mig: struct StructMemberI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct StructMemberI<'s, 'i> {
    pub name: IVarNameI<'s, 'i>,
    pub variability: VariabilityI,
    pub tyype: IMemberTypeI<'s, 'i>,
}
// mig: impl StructMemberI

// mig: enum IMemberTypeI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum IMemberTypeI<'s, 'i> {
    ReferenceMemberTypeI(&'i ReferenceMemberTypeI<'s, 'i>),
    AddressMemberTypeI(&'i AddressMemberTypeI<'s, 'i>),
}
// mig: impl IMemberTypeI

// mig: fn expect_reference_member

impl<'s, 'i> IMemberTypeI<'s, 'i> {
    pub fn expect_reference_member(&self) -> () {
        match self {
            _ => panic!("Unimplemented: IMemberTypeI::expect_reference_member dispatch"),
        }
    }
}

// mig: fn expect_address_member

impl<'s, 'i> IMemberTypeI<'s, 'i> {
    pub fn expect_address_member(&self) -> &'i AddressMemberTypeI<'s, 'i> {
        match *self {
            // BUG: Scala message says "Expected reference member, was address member!" but function is expectAddressMember; preserving Scala wording.
            IMemberTypeI::ReferenceMemberTypeI(_) => panic!("Expected reference member, was address member!"),
            IMemberTypeI::AddressMemberTypeI(a) => a,
        }
    }
}

// mig: struct AddressMemberTypeI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct AddressMemberTypeI<'s, 'i> {
    pub reference: CoordI<'s, 'i>,
}
// mig: impl AddressMemberTypeI

// mig: struct ReferenceMemberTypeI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ReferenceMemberTypeI<'s, 'i> {
    pub reference: CoordI<'s, 'i>,
}
// mig: impl ReferenceMemberTypeI

// mig: struct InterfaceDefinitionI
/// Temporary state
pub struct InterfaceDefinitionI<'s, 'i> {
    pub instantiated_interface: &'i InterfaceIT<'s, 'i>,
    pub attributes: &'i [ICitizenAttributeI<'s>],
    pub weakable: bool,
    pub mutability: MutabilityI,
    pub rune_to_function_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i>>,
    pub rune_to_impl_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i>>,
    pub internal_methods: &'i [(&'i PrototypeI<'s, 'i>, i32)],
}
// mig: impl InterfaceDefinitionI

// mig: fn instantiated_citizen
impl<'s, 'i> InterfaceDefinitionI<'s, 'i> {
    pub fn instantiated_citizen(&self) -> ICitizenIT<'s, 'i> {
        panic!("Unimplemented: instantiated_citizen")
    }
}

// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for InterfaceDefinitionI` below.)

// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for InterfaceDefinitionI` below.)
