use crate::utils::arena_index_map::ArenaIndexMap;
use crate::postparsing::names::IRuneS;
use crate::instantiating::ast::types::{KindIT, SharednessI, StructIT, InterfaceIT, ICitizenIT};
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
    pub sharedness: SharednessI,
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


/// A struct member is name-keyed and carries an onion kind directly — no reference/address
/// member split (addressibility is retired). Mirrors typing's named + KindT members.
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct StructMemberI<'s, 'i> {
    pub name: IVarNameI<'s, 'i>,
    pub tyype: KindIT<'s, 'i>,
}



/// Temporary state
pub struct InterfaceDefinitionI<'s, 'i> {
    pub instantiated_interface: &'i InterfaceIT<'s, 'i>,
    pub attributes: &'i [ICitizenAttributeI<'s>],
    pub weakable: bool,
    pub sharedness: SharednessI,
    pub rune_to_function_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i>>,
    pub rune_to_impl_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i>>,
    pub internal_methods: &'i [(&'i PrototypeI<'s, 'i>, i32)],
}



impl<'s, 'i> InterfaceDefinitionI<'s, 'i> {
    pub fn instantiated_citizen(&self) -> ICitizenIT<'s, 'i> {
        panic!("Unimplemented: instantiated_citizen")
    }
}
