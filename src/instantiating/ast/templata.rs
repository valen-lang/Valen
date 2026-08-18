use crate::instantiating::ast::types::KindIT;
use crate::instantiating::ast::ast::{FunctionHeaderI, PrototypeI};
use crate::instantiating::ast::names::IdI;
use crate::interner::StrI;
use crate::utils::range::RangeS;
use crate::postparsing::itemplatatype::TemplateTemplataType;
use std::marker::PhantomData;



pub fn expect_kind<'s, 'i>(templata: ITemplataI<'s, 'i>) -> ITemplataI<'s, 'i> {
    panic!("Unimplemented: expect_kind");
    // templata match { case t @ KindTemplataI(_) => t; case _ => vfail() }
}


pub fn expect_kind_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> KindTemplataI<'s, 'i> {
    match templata {
        ITemplataI::Kind(t) => t,
        _ => panic!("expect_kind_templata: not a KindTemplataI"),
    }
}


pub fn expect_integer_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> IntegerTemplataI {
    match templata {
        ITemplataI::Integer(t) => t,
        _ => panic!("vfail"),
    }
}


/// A coord is now just an onion kind, so there is no Coord templata — Kind carries it.
/// Ownership/Location/Region templatas are gone: ownership is a wrap on the kind, and
/// regions/groups are declaration-side (BCHATZ). Mirrors typing's ITemplataT.
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ITemplataI<'s, 'i> {
  Kind(KindTemplataI<'s, 'i>),
  RuntimeSizedArrayTemplate(RuntimeSizedArrayTemplateTemplataI),
  StaticSizedArrayTemplate(StaticSizedArrayTemplateTemplataI),
  Function(FunctionTemplataI<'s, 'i>),
  StructDefinition(StructDefinitionTemplataI<'s, 'i>),
  InterfaceDefinition(InterfaceDefinitionTemplataI<'s, 'i>),
  ImplDefinition(ImplDefinitionTemplataI<'s, 'i>),
  Boolean(BooleanTemplataI),
  Integer(IntegerTemplataI),
  String(StringTemplataI<'s>),
  Prototype(PrototypeTemplataI<'s, 'i>),
  Isa(IsaTemplataI<'s, 'i>),
  KindList(KindListTemplataI<'s, 'i>),
  ExternFunction(ExternFunctionTemplataI<'s, 'i>),
}



impl<'s, 'i> ITemplataI<'s, 'i> {
  pub fn expect_kind_templata(&self) -> KindTemplataI<'s, 'i> {
    match self {
      ITemplataI::Kind(k) => *k,
      _ => panic!("expect_kind_templata: not a KindTemplataI"),
    }
  }
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
pub struct KindListTemplataI<'s, 'i> {
  pub kinds: &'i [KindIT<'s, 'i>],
}




#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExternFunctionTemplataI<'s, 'i> {
  pub header: &'i FunctionHeaderI<'s, 'i>,
}
