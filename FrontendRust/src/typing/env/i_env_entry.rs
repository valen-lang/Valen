use crate::typing::names::names::IdT;
use crate::typing::templata::templata::ITemplataT;
use crate::postparsing::itemplatatype::TemplateTemplataType;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionEnvEntry<'s, 't> where 's: 't {
  pub template_id: &'t IdT<'s, 't>,
  // We can add tyype here if convenient
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StructEnvEntry<'s, 't> where 's: 't {
  pub template_id: &'t IdT<'s, 't>,
  pub tyype: TemplateTemplataType<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InterfaceEnvEntry<'s, 't> where 's: 't {
  pub template_id: &'t IdT<'s, 't>,
  pub tyype: TemplateTemplataType<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplEnvEntry<'s, 't> where 's: 't {
  pub template_id: &'t IdT<'s, 't>,
}


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum IEnvEntryT<'s, 't>
where 's: 't,
{
  Function(FunctionEnvEntry<'s, 't>),
  Struct(StructEnvEntry<'s, 't>),
  Interface(InterfaceEnvEntry<'s, 't>),
  Impl(ImplEnvEntry<'s, 't>),
  Templata(ITemplataT<'s, 't>),
}
