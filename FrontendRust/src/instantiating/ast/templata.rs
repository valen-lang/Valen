use crate::instantiating::ast::types::{CoordI, KindIT, OwnershipI, MutabilityI, VariabilityI, LocationI};
use crate::instantiating::ast::ast::{FunctionHeaderI, PrototypeI};
use crate::instantiating::ast::names::{IdI, IImplNameI};
use crate::interner::StrI;
use crate::utils::range::RangeS;
use crate::postparsing::itemplatatype::TemplateTemplataType;
use crate::typing::types::types::{KindT, RegionT};
use std::marker::PhantomData;


// mig: fn expect_coord
pub fn expect_coord<'s, 'i>(templata: ITemplataI<'s, 'i>) -> ITemplataI<'s, 'i> {
    panic!("Unimplemented: expect_coord");
    // templata match { case t @ CoordTemplataI(_, _) => t; case other => vfail(other) }
}

// mig: fn expect_coord_templata
pub fn expect_coord_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> CoordTemplataI<'s, 'i> {
    match templata {
        ITemplataI::Coord(t) => t,
        _ => panic!("expect_coord_templata: not a CoordTemplataI"),
    }
}

// mig: fn expect_integer_templata
pub fn expect_integer_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> IntegerTemplataI {
    match templata {
        ITemplataI::Integer(t) => t,
        _ => panic!("vfail"),
    }
}

// mig: fn expect_mutability_templata
pub fn expect_mutability_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> MutabilityTemplataI {
    match templata {
        ITemplataI::Mutability(m) => m,
        _ => panic!("expect_mutability_templata: not a MutabilityTemplataI"),
    }
}

// mig: fn expect_variability_templata
pub fn expect_variability_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> VariabilityTemplataI {
    match templata {
        ITemplataI::Variability(t) => t,
        _ => panic!("expect_variability_templata: not a VariabilityTemplataI"),
    }
}

// mig: fn expect_kind
pub fn expect_kind<'s, 'i>(templata: ITemplataI<'s, 'i>) -> ITemplataI<'s, 'i> {
    panic!("Unimplemented: expect_kind");
    // templata match { case t @ KindTemplataI(_) => t; case _ => vfail() }
}

// mig: fn expect_kind_templata
pub fn expect_kind_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> KindTemplataI<'s, 'i> {
    panic!("Unimplemented: expect_kind_templata");
    // templata match { case t @ KindTemplataI(_) => t; case _ => vfail() }
}

// mig: fn expect_region_templata
pub fn expect_region_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> RegionT {
    panic!("Unimplemented: expect_region_templata");
    // templata match { case t @ RegionTemplataI(_) => t; case _ => vfail() }
}

// mig: enum ITemplataI
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
// mig: impl ITemplataI

// mig: fn expect_coord_templata
impl<'s, 'i> ITemplataI<'s, 'i> {
  pub fn expect_coord_templata(&self) -> CoordTemplataI<'s, 'i> {
    panic!("Unimplemented: expect_coord_templata");
    // this match { case c@CoordTemplataI(_, _) => c; case other => vwat(other) }
  }

// mig: fn expect_region_templata
  pub fn expect_region_templata(&self) -> RegionT {
    panic!("Unimplemented: expect_region_templata");
    // this match { case c@RegionTemplataI(_) => c; case other => vwat(other) }
  }
}

// mig: struct CoordTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct CoordTemplataI<'s, 'i> {
  pub region: RegionT,
  pub coord: CoordI<'s, 'i>,
}
// mig: impl CoordTemplataI

// mig: struct KindTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct KindTemplataI<'s, 'i> {
  pub kind: KindIT<'s, 'i>,
}
// mig: impl KindTemplataI

// mig: struct RuntimeSizedArrayTemplateTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct RuntimeSizedArrayTemplateTemplataI {
}
// mig: impl RuntimeSizedArrayTemplateTemplataI

// mig: struct StaticSizedArrayTemplateTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct StaticSizedArrayTemplateTemplataI {
}
// mig: impl StaticSizedArrayTemplateTemplataI

// mig: struct FunctionTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct FunctionTemplataI<'s, 'i> {
  pub env_id: IdI<'s, 'i>,
}
// mig: impl FunctionTemplataI

// mig: fn get_template_name
impl<'s, 'i> FunctionTemplataI<'s, 'i> {
  pub fn get_template_name(&self) -> IdI<'s, 'i> {
    panic!("Unimplemented: get_template_name");
    // vimpl()
  }
}

// mig: struct StructDefinitionTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct StructDefinitionTemplataI<'s, 'i> {
  pub env_id: IdI<'s, 'i>,
  pub tyype: TemplateTemplataType<'s>,
}
// mig: impl StructDefinitionTemplataI

// mig: enum CitizenDefinitionTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum CitizenDefinitionTemplataI<'s, 'i> {
  Struct(StructDefinitionTemplataI<'s, 'i>),
  Interface(InterfaceDefinitionTemplataI<'s, 'i>),
}
// mig: impl CitizenDefinitionTemplataI

// mig: struct InterfaceDefinitionTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct InterfaceDefinitionTemplataI<'s, 'i> {
  pub env_id: IdI<'s, 'i>,
  pub tyype: TemplateTemplataType<'s>,
}
// mig: impl InterfaceDefinitionTemplataI

// mig: struct ImplDefinitionTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ImplDefinitionTemplataI<'s, 'i> {
  pub env_id: IdI<'s, 'i>,
}
// mig: impl ImplDefinitionTemplataI

// mig: struct OwnershipTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct OwnershipTemplataI {
  pub ownership: OwnershipI,
}
// mig: impl OwnershipTemplataI

// mig: struct VariabilityTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct VariabilityTemplataI {
  pub variability: VariabilityI,
}
// mig: impl VariabilityTemplataI

// mig: struct MutabilityTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct MutabilityTemplataI {
  pub mutability: MutabilityI,
}
// mig: impl MutabilityTemplataI

// mig: struct LocationTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct LocationTemplataI {
  pub location: LocationI,
}
// mig: impl LocationTemplataI

// mig: struct BooleanTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct BooleanTemplataI {
  pub value: bool,
}
// mig: impl BooleanTemplataI

// mig: struct IntegerTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct IntegerTemplataI {
  pub value: i64,
}
// mig: impl IntegerTemplataI

// mig: struct StringTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct StringTemplataI<'s> {
  pub value: StrI<'s>,
}
// mig: impl StringTemplataI

// mig: struct PrototypeTemplataI
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrototypeTemplataI<'s, 'i> {
  pub declaration_range: RangeS<'s>,
  pub prototype: &'i PrototypeI<'s, 'i>,
}
// mig: impl PrototypeTemplataI

// mig: struct IsaTemplataI
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IsaTemplataI<'s, 'i> {
  pub declaration_range: RangeS<'s>,
  pub impl_name: IdI<'s, 'i>,
  pub sub_kind: KindIT<'s, 'i>,
  pub super_kind: KindIT<'s, 'i>,
}
// mig: impl IsaTemplataI

// mig: struct CoordListTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct CoordListTemplataI<'s, 'i> {
  pub coords: &'i[CoordI<'s, 'i>],
}
// mig: impl CoordListTemplataI


// mig: struct ExternFunctionTemplataI
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExternFunctionTemplataI<'s, 'i> {
  pub header: &'i FunctionHeaderI<'s, 'i>,
}
// mig: impl ExternFunctionTemplataI

