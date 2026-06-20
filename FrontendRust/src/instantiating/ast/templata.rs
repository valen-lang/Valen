use crate::instantiating::ast::types::{CoordI, KindIT, OwnershipI, MutabilityI, VariabilityI, LocationI};
use crate::instantiating::ast::ast::{FunctionHeaderI, PrototypeI};
use crate::instantiating::ast::names::{IdI, IImplNameI};
use crate::interner::StrI;
use crate::utils::range::RangeS;
use crate::postparsing::itemplatatype::TemplateTemplataType;
use crate::typing::types::types::{KindT, RegionT};
use std::marker::PhantomData;


pub fn expect_coord<'s, 'i>(templata: ITemplataI<'s, 'i>) -> ITemplataI<'s, 'i> {
    panic!("Unimplemented: expect_coord");
    // templata match { case t @ CoordTemplataI(_, _) => t; case other => vfail(other) }
}

pub fn expect_coord_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> CoordTemplataI<'s, 'i> {
    match templata {
        ITemplataI::Coord(t) => t,
        _ => panic!("expect_coord_templata: not a CoordTemplataI"),
    }
}

pub fn expect_integer_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> IntegerTemplataI {
    match templata {
        ITemplataI::Integer(t) => t,
        _ => panic!("vfail"),
    }
}

pub fn expect_mutability_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> MutabilityTemplataI {
    match templata {
        ITemplataI::Mutability(m) => m,
        _ => panic!("expect_mutability_templata: not a MutabilityTemplataI"),
    }
}

pub fn expect_variability_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> VariabilityTemplataI {
    match templata {
        ITemplataI::Variability(t) => t,
        _ => panic!("expect_variability_templata: not a VariabilityTemplataI"),
    }
}

pub fn expect_kind<'s, 'i>(templata: ITemplataI<'s, 'i>) -> ITemplataI<'s, 'i> {
    panic!("Unimplemented: expect_kind");
    // templata match { case t @ KindTemplataI(_) => t; case _ => vfail() }
}

pub fn expect_kind_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> KindTemplataI<'s, 'i> {
    panic!("Unimplemented: expect_kind_templata");
    // templata match { case t @ KindTemplataI(_) => t; case _ => vfail() }
}

pub fn expect_region_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> RegionT {
    panic!("Unimplemented: expect_region_templata");
    // templata match { case t @ RegionTemplataI(_) => t; case _ => vfail() }
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ITemplataI<'s, 'i> {
  Coord(CoordTemplataI<'s, 'i>),
  Kind(KindTemplataI<'s, 'i>),
  RuntimeSizedArrayTemplate(RuntimeSizedArrayTemplateTemplataI),
  StaticSizedArrayTemplate(StaticSizedArrayTemplateTemplataI),
  Function(FunctionTemplataI<'s, 'i>),
  StructDefinition(StructDefinitionTemplataI<'s, 'i>),
  InterfaceDefinition(InterfaceDefinitionTemplataI<'s, 'i>),
  ImplDefinition(ImplDefinitionTemplataI<'s, 'i>),
  Ownership(OwnershipTemplataI),
  Variability(VariabilityTemplataI),
  Mutability(MutabilityTemplataI),
  Location(LocationTemplataI),
  Boolean(BooleanTemplataI),
  Integer(IntegerTemplataI),
  String(StringTemplataI<'s>),
  Prototype(PrototypeTemplataI<'s, 'i>),
  Isa(IsaTemplataI<'s, 'i>),
  CoordList(CoordListTemplataI<'s, 'i>),
  Region(RegionT),
  ExternFunction(ExternFunctionTemplataI<'s, 'i>),
}

impl<'s, 'i> ITemplataI<'s, 'i> {
  pub fn expect_coord_templata(&self) -> CoordTemplataI<'s, 'i> {
    panic!("Unimplemented: expect_coord_templata");
    // this match { case c@CoordTemplataI(_, _) => c; case other => vwat(other) }
  }

  pub fn expect_region_templata(&self) -> RegionT {
    panic!("Unimplemented: expect_region_templata");
    // this match { case c@RegionTemplataI(_) => c; case other => vwat(other) }
  }
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct CoordTemplataI<'s, 'i> {
  pub region: RegionT,
  pub coord: CoordI<'s, 'i>,
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct KindTemplataI<'s, 'i> {
  pub kind: KindIT<'s, 'i>,
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct RuntimeSizedArrayTemplateTemplataI {
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct StaticSizedArrayTemplateTemplataI {
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct FunctionTemplataI<'s, 'i> {
  pub env_id: IdI<'s, 'i>,
}

impl<'s, 'i> FunctionTemplataI<'s, 'i> {
  pub fn get_template_name(&self) -> IdI<'s, 'i> {
    panic!("Unimplemented: get_template_name");
    // vimpl()
  }
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct StructDefinitionTemplataI<'s, 'i> {
  pub env_id: IdI<'s, 'i>,
  pub tyype: TemplateTemplataType<'s>,
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum CitizenDefinitionTemplataI<'s, 'i> {
  Struct(StructDefinitionTemplataI<'s, 'i>),
  Interface(InterfaceDefinitionTemplataI<'s, 'i>),
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct InterfaceDefinitionTemplataI<'s, 'i> {
  pub env_id: IdI<'s, 'i>,
  pub tyype: TemplateTemplataType<'s>,
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ImplDefinitionTemplataI<'s, 'i> {
  pub env_id: IdI<'s, 'i>,
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct OwnershipTemplataI {
  pub ownership: OwnershipI,
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct VariabilityTemplataI {
  pub variability: VariabilityI,
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct MutabilityTemplataI {
  pub mutability: MutabilityI,
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct LocationTemplataI {
  pub location: LocationI,
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct BooleanTemplataI {
  pub value: bool,
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct IntegerTemplataI {
  pub value: i64,
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct StringTemplataI<'s> {
  pub value: StrI<'s>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrototypeTemplataI<'s, 'i> {
  pub declaration_range: RangeS<'s>,
  pub prototype: &'i PrototypeI<'s, 'i>,
}

#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IsaTemplataI<'s, 'i> {
  pub declaration_range: RangeS<'s>,
  pub impl_name: IdI<'s, 'i>,
  pub sub_kind: KindIT<'s, 'i>,
  pub super_kind: KindIT<'s, 'i>,
}

/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct CoordListTemplataI<'s, 'i> {
  pub coords: &'i[CoordI<'s, 'i>],
}


#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExternFunctionTemplataI<'s, 'i> {
  pub header: &'i FunctionHeaderI<'s, 'i>,
}

