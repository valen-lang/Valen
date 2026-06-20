// VISTODO: rename Hinputs everywhere
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::postparsing::names::IRuneS;
use crate::instantiating::ast::types::StructIT;
use crate::instantiating::ast::names::IdI;
use crate::instantiating::ast::names::{IStructTemplateNameI, IInterfaceTemplateNameI, IImplTemplateNameI, IFunctionTemplateNameI};
use crate::instantiating::ast::ast::{
    EdgeI, FunctionDefinitionI, FunctionExportI, FunctionExternI, InterfaceEdgeBlueprintI,
    KindExportI, KindExternI, PrototypeI,
};
use crate::instantiating::ast::citizens::{ICitizenDefinitionI, InterfaceDefinitionI, StructDefinitionI};
use crate::instantiating::ast::names::INameI;
use crate::instantiating::ast::names::StructNameI;


/// Temporary state (see @TFITCX)
pub struct InstantiationBoundArgumentsI<'s, 'i> where 's: 'i {
    pub rune_to_function_bound_arg: ArenaIndexMap<'i, IRuneS<'s>, &'i PrototypeI<'s, 'i>>,
    pub caller_rune_to_callee_rune_to_reachable_func:
        ArenaIndexMap<'i, IRuneS<'s>, ArenaIndexMap<'i, IRuneS<'s>, &'i PrototypeI<'s, 'i>>>,
    pub rune_to_impl_bound_arg: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i>>,
}


/// Temporary state (see @TFITCX) — top-level container for instantiated output.
pub struct HinputsI<'s, 'i> where 's: 'i {
    pub interfaces: &'i [InterfaceDefinitionI<'s, 'i>],
    pub structs: &'i [&'i StructDefinitionI<'s, 'i>],
    pub functions: &'i [&'i FunctionDefinitionI<'s, 'i>],
    pub interface_to_edge_blueprints:
        ArenaIndexMap<'i, IdI<'s, 'i>, InterfaceEdgeBlueprintI<'s, 'i>>,
    pub interface_to_sub_citizen_to_edge:
        ArenaIndexMap<'i, IdI<'s, 'i>, ArenaIndexMap<'i, IdI<'s, 'i>, EdgeI<'s, 'i>>>,
    pub kind_exports: &'i [KindExportI<'s, 'i>],
    pub function_exports: &'i [FunctionExportI<'s, 'i>],
    pub kind_externs: ArenaIndexMap<'i, &'i StructIT<'s, 'i>, KindExternI<'s, 'i>>,
    pub function_externs: &'i [FunctionExternI<'s, 'i>],
}

impl<'s, 'i> HinputsI<'s, 'i> where 's: 'i {
    pub fn to_string(&self) -> String {
        panic!("Unimplemented: to_string")
        // "HinputsI#()"
    }
    pub fn lookup_function_by_str(&self, human_name: &str) -> &'i FunctionDefinitionI<'s, 'i> {
        let matches: Vec<&&'i FunctionDefinitionI<'s, 'i>> = self.functions.iter().filter(|f| {
            match f.header.id.local_name {
                INameI::FunctionNameIX(n) => n.template.human_name.0 == human_name,
                _ => false,
            }
        }).collect();
        if matches.is_empty() {
            panic!("Function \"{}\" not found!", human_name);
        } else if matches.len() > 1 {
            panic!("Multiple found!");
        }
        matches[0]
    }



    pub fn lookup_struct(
        &self,
        _struct_id: &IdI<'s, 'i>,
    ) -> &'i StructDefinitionI<'s, 'i> {
        self.structs.iter().find(|s| s.instantiated_citizen.id == *_struct_id).copied().expect("lookup_struct: not found")
    }

    pub fn lookup_interface(
        &self,
        _interface_id: &IdI<'s, 'i>,
    ) -> &'i InterfaceDefinitionI<'s, 'i> {
        self.interfaces.iter().find(|i| i.instantiated_interface.id == *_interface_id).expect("lookup_interface: not found")
    }

    pub fn lookup_citizen(
        &self,
        _citizen_id: &IdI<'s, 'i>,
    ) -> ICitizenDefinitionI<'s, 'i> {
        panic!("Unimplemented: lookup_citizen")
        // vassertOne(structs.find(_.instantiatedCitizen.id == citizenId) ++ interfaces.find(_.instantiatedCitizen.id == citizenId))
    }

    pub fn lookup_struct_by_template(
        &self,
        _struct_template_name: &IStructTemplateNameI<'s, 'i>,
    ) -> &'i StructDefinitionI<'s, 'i> {
        panic!("Unimplemented: lookup_struct_by_template")
        // vassertSome(structs.find(_.instantiatedCitizen.id.localName.template == structTemplateName))
    }

    pub fn lookup_interface_by_template(
        &self,
        _interface_template_name: &IInterfaceTemplateNameI<'s, 'i>,
    ) -> &'i InterfaceDefinitionI<'s, 'i> {
        panic!("Unimplemented: lookup_interface_by_template")
        // vassertSome(interfaces.find(_.instantiatedCitizen.id.localName.template == interfaceTemplateName))
    }

    pub fn lookup_impl_by_template(
        &self,
        _impl_template_name: &IImplTemplateNameI<'s, 'i>,
    ) -> &'i EdgeI<'s, 'i> {
        panic!("Unimplemented: lookup_impl_by_template")
        // vassertSome(interfaceToSubCitizenToEdge.flatMap(_._2.values).find(_.edgeId.localName.template == implTemplateName))
    }

    pub fn lookup_edge(
        &self,
        _impl_id: &IdI<'s, 'i>,
    ) -> &'i EdgeI<'s, 'i> {
        panic!("Unimplemented: lookup_edge")
        // vassertOne(interfaceToSubCitizenToEdge.flatMap(_._2.values).find(_.edgeId == implId))
    }

    pub fn lookup_function_by_template(
        &self,
        _func_template_name: &IFunctionTemplateNameI<'s, 'i>,
    ) -> Option<&'i FunctionDefinitionI<'s, 'i>> {
        panic!("Unimplemented: lookup_function_by_template")
        // functions.find(_.header.id.localName.template == funcTemplateName).headOption
    }

    pub fn lookup_function(
        &self,
        _human_name: &str,
    ) -> &'i FunctionDefinitionI<'s, 'i> {
        panic!("Unimplemented: lookup_function")
        // val matches = functions.filter(f => f.header.id.localName match {
        //   case FunctionNameIX(n, _, _) if n.humanName.str == humanName => true
        //   case _ => false
        // })
        // if (matches.size == 0) vfail("Function \"" + humanName + "\" not found!")
        // else if (matches.size > 1) vfail("Multiple found!")
        // matches.head
    }

    pub fn lookup_struct_by_name(
        &self,
        human_name: &str,
    ) -> &'i StructDefinitionI<'s, 'i> {
        let matches: Vec<&&'i StructDefinitionI<'s, 'i>> = self.structs.iter().filter(|s| {
            match s.instantiated_citizen.id.local_name {
                INameI::StructName(StructNameI { template: IStructTemplateNameI::StructTemplate(t), .. }) if t.human_name.0 == human_name => true,
                _ => false,
            }
        }).collect();
        if matches.is_empty() {
            panic!("Struct \"{}\" not found!", human_name);
        } else if matches.len() > 1 {
            panic!("Multiple found!");
        }
        matches[0]
    }

    pub fn lookup_impl(
        &self,
        _sub_citizen_it: &IdI<'s, 'i>,
        _interface_it: &IdI<'s, 'i>,
    ) -> &'i EdgeI<'s, 'i> {
        panic!("Unimplemented: lookup_impl")
        // vassertSome(vassertSome(interfaceToSubCitizenToEdge.get(interfaceIT)).get(subCitizenIT))
    }

    pub fn lookup_interface_by_name(
        &self,
        _human_name: &str,
    ) -> &'i InterfaceDefinitionI<'s, 'i> {
        panic!("Unimplemented: lookup_interface_by_name")
        // val matches = interfaces.filter(s => s.instantiatedCitizen.id.localName match {
        //   case InterfaceNameI(InterfaceTemplateNameI(n), _) if n.str == humanName => true
        //   case _ => false
        // })
        // if (matches.size == 0) vfail("Interface \"" + humanName + "\" not found!")
        // else if (matches.size > 1) vfail("Multiple found!")
        // matches.head
    }

    pub fn lookup_user_function(
        &self,
        _human_name: &str,
    ) -> &'i FunctionDefinitionI<'s, 'i> {
        panic!("Unimplemented: lookup_user_function")
        // val matches = functions.filter(f => simpleNameI.unapply(f.header.id).contains(humanName))
        //                          .filter(_.header.isUserFunction)
        // if (matches.size == 0) vfail("Not found!")
        // else if (matches.size > 1) vfail("Multiple found!")
        // matches.head
    }

    pub fn get_all_non_extern_functions(
        &self,
    ) -> Vec<&'i FunctionDefinitionI<'s, 'i>> {
        panic!("Unimplemented: get_all_non_extern_functions")
        // functions.filter(!_.header.isExtern)
    }

    pub fn get_all_user_functions(
        &self,
    ) -> Vec<&'i FunctionDefinitionI<'s, 'i>> {
        panic!("Unimplemented: get_all_user_functions")
        // functions.filter(_.header.isUserFunction)
    }
}
