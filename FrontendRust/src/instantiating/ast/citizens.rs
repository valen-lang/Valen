use crate::utils::arena_index_map::ArenaIndexMap;
use crate::postparsing::names::IRuneS;
use crate::instantiating::ast::types::{CoordI, MutabilityI, VariabilityI, StructIT, InterfaceIT, ICitizenIT};
use crate::instantiating::ast::names::{IdI, IVarNameI};
use crate::instantiating::ast::ast::{ICitizenAttributeI, PrototypeI};
use std::marker::PhantomData;


pub trait CitizenDefinitionI<'s, 'i> {}

#[derive(Copy, Clone)]
pub enum ICitizenDefinitionI<'s, 'i> {
    StructDefinitionI(&'i StructDefinitionI<'s, 'i>),
    InterfaceDefinitionI(&'i InterfaceDefinitionI<'s, 'i>),
}
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



impl<'s, 'i> StructDefinitionI<'s, 'i> {
    pub fn get_member_and_index(&self, needle_name: IVarNameI<'s, 'i>) -> Option<(&StructMemberI<'s, 'i>, usize)> {
        panic!("Unimplemented: get_member_and_index")
    }
}

/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct StructMemberI<'s, 'i> {
    pub name: IVarNameI<'s, 'i>,
    pub variability: VariabilityI,
    pub tyype: IMemberTypeI<'s, 'i>,
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum IMemberTypeI<'s, 'i> {
    ReferenceMemberTypeI(&'i ReferenceMemberTypeI<'s, 'i>),
    AddressMemberTypeI(&'i AddressMemberTypeI<'s, 'i>),
}


impl<'s, 'i> IMemberTypeI<'s, 'i> {
    pub fn expect_reference_member(&self) -> () {
        match self {
            _ => panic!("Unimplemented: IMemberTypeI::expect_reference_member dispatch"),
        }
    }
}


impl<'s, 'i> IMemberTypeI<'s, 'i> {
    pub fn expect_address_member(&self) -> &'i AddressMemberTypeI<'s, 'i> {
        match *self {
            IMemberTypeI::ReferenceMemberTypeI(_) => panic!("Expected reference member, was address member!"),
            IMemberTypeI::AddressMemberTypeI(a) => a,
        }
    }
}

/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct AddressMemberTypeI<'s, 'i> {
    pub reference: CoordI<'s, 'i>,
}

/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ReferenceMemberTypeI<'s, 'i> {
    pub reference: CoordI<'s, 'i>,
}

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

impl<'s, 'i> InterfaceDefinitionI<'s, 'i> {
    pub fn instantiated_citizen(&self) -> ICitizenIT<'s, 'i> {
        panic!("Unimplemented: instantiated_citizen")
    }
}


