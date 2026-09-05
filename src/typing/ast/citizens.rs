use crate::postparsing::itemplatatype::ITemplataType;
use crate::postparsing::names::IImpreciseNameS;
use crate::scout_arena::ScoutArena;
use crate::typing::ast::ast::*;
use crate::typing::hinputs_t::*;
use crate::typing::names::names::*;
use crate::typing::templata::templata::*;
use crate::typing::types::types::*;

/// Value-type (see @TFITCX)
pub enum CitizenDefinitionT<'s, 't> {
  Struct(&'t StructDefinitionT<'s, 't>),
  Interface(&'t InterfaceDefinitionT<'s, 't>),
}

impl<'s, 't> CitizenDefinitionT<'s, 't>
where
  's: 't,
{
  pub fn template_name(&self) -> IdT<'s, 't> {
    match self {
      CitizenDefinitionT::Struct(s) => {
        panic!("Unimplemented: template_name Struct");
        // s.templateName
      }
      CitizenDefinitionT::Interface(i) => {
        panic!("Unimplemented: template_name Interface");
        // i.templateName
      }
    }
  }

  pub fn generic_param_types(&self, scout_arena: &ScoutArena<'s>) -> Vec<ITemplataType<'s>> {
    match self {
      CitizenDefinitionT::Struct(s) => s.generic_param_types(scout_arena),
      CitizenDefinitionT::Interface(i) => i.generic_param_types(scout_arena),
    }
  }

  pub fn instantiated_citizen(&self) -> ICitizenTT<'s, 't> {
    match self {
      CitizenDefinitionT::Struct(s) => ICitizenTT::Struct(&s.instantiated_citizen),
      CitizenDefinitionT::Interface(i) => ICitizenTT::Interface(&i.instantiated_interface),
    }
  }

  pub fn default_region(&self) -> RegionT {
    match self {
      CitizenDefinitionT::Struct(s) => {
        panic!("Unimplemented: default_region Struct");
        // s.defaultRegion
      }
      CitizenDefinitionT::Interface(i) => {
        panic!("Unimplemented: default_region Interface");
        // i.defaultRegion
      }
    }
  }
}
/// Arena-allocated (see @TFITCX)
pub struct StructDefinitionT<'s, 't> {
  pub template_name: IdT<'s, 't>,
  pub instantiated_citizen: StructTT<'s, 't>,
  pub attributes: &'t [ICitizenAttributeT<'s>],
  pub sharedness: SharednessT,
  pub members: &'t [StructMemberT<'s, 't>],
  pub instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
}

impl<'s, 't> StructDefinitionT<'s, 't> {
  fn default_region(&self) -> RegionT {
    panic!("Unimplemented: default_region");
    // RegionT(DefaultRegionT)
  }

  fn generic_param_types(&self, scout_arena: &ScoutArena<'s>) -> Vec<ITemplataType<'s>> {
    IStructNameT::try_from(self.instantiated_citizen.id.local_name)
      .unwrap()
      .template_args()
      .iter()
      .map(|t| t.tyype(scout_arena))
      .collect()
  }

  pub fn get_member_and_index(
    &self,
    needle: IImpreciseNameS<'s>,
  ) -> Option<(&StructMemberT<'s, 't>, usize)> {
    for (index, member) in self.members.iter().enumerate() {
      if IImpreciseNameS::CodeName(member.name.imprecise_name) == needle {
        return Some((member, index));
      }
    }
    None
  }
}

/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructMemberT<'s, 't> {
  pub name: &'t MemberNameT<'s, 't>,
  pub tyype: KindT<'s, 't>,
}

/// Arena-allocated (see @TFITCX)
pub struct InterfaceDefinitionT<'s, 't> {
  pub template_name: IdT<'s, 't>,
  pub instantiated_interface: InterfaceTT<'s, 't>,
  pub ref_: InterfaceTT<'s, 't>,
  pub attributes: &'t [ICitizenAttributeT<'s>],
  pub sharedness: SharednessT,
  pub instantiation_bound_params: &'t InstantiationBoundArgumentsT<'s, 't>,
  pub internal_methods: &'t [(PrototypeT<'s, 't>, usize)],
}

impl<'s, 't> InterfaceDefinitionT<'s, 't> {
  fn default_region(&self) -> RegionT {
    panic!("Unimplemented: default_region");
    // RegionT(DefaultRegionT)
  }

  fn generic_param_types(&self, scout_arena: &ScoutArena<'s>) -> Vec<ITemplataType<'s>> {
    IInterfaceNameT::try_from(self.instantiated_interface.id.local_name)
        .unwrap()
        .template_args()
        .iter()
        .map(|t| t.tyype(scout_arena))
        .collect()
  }

  fn instantiated_citizen(&self) -> ICitizenTT<'s, 't> {
    panic!("Unimplemented: instantiated_citizen");
    // instantiatedInterface
  }
}
