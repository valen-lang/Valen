
use crate::instantiating::ast::names::*;
use crate::instantiating::ast::types::*;
use crate::instantiating::ast::ast::*;
use crate::instantiating::ast::citizens::*;
use crate::instantiating::ast::templata::*;
use crate::instantiating::ast::hinputs::*;
use crate::instantiating::ast::expressions::*;
use crate::instantiating::instantiating_interner::InstantiatingInterner;
use crate::typing::typing_interner::TypingInterner;
use crate::instantiating::collector;
use crate::instantiating::collector::NodeRefI;
use crate::typing::names::names::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::types::types::*;
use crate::typing::hinputs_t::*;
use crate::typing::compiler::Compiler;
use crate::utils::vassert::vassert_one;
use crate::postparsing::names::{IImpreciseNameS, IRuneS};
use crate::postparsing::post_parser_error_humanizer::humanize_imprecise_name;
use crate::scout_arena::ScoutArena;
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::keywords::Keywords;
use crate::compile_options::GlobalOptions;
use crate::typing::templata::templata::ITemplataT;
use crate::typing::templata_compiler::peel_all_references;
use crate::typing::ast::expressions::ExpressionTE;
use crate::typing::env::function_environment_t::LocalVariable;
use crate::utils::fx::IndexMap;
use crate::instantiating::ast::ast::ExternI;
use crate::instantiating::ast::ast::ICitizenAttributeI;
use crate::instantiating::ast::ast::KindExternI;
use crate::instantiating::ast::ast::LocationInFunctionEnvironmentI;
use crate::instantiating::ast::ast::LocalVariableI;
use crate::instantiating::ast::citizens::InterfaceDefinitionI;
use crate::instantiating::ast::citizens::StructDefinitionI;
use crate::instantiating::ast::citizens::StructMemberI;
use crate::instantiating::ast::expressions::DerefIE;
use crate::instantiating::ast::expressions::ArgLookupIE;
use crate::instantiating::ast::expressions::ArrayLengthIE;
use crate::instantiating::ast::expressions::AsSubtypeIE;
use crate::instantiating::ast::expressions::ConstructIE;
use crate::instantiating::ast::expressions::DestroyIE;
use crate::instantiating::ast::expressions::DestroyRuntimeSizedArrayIE;
use crate::instantiating::ast::expressions::DestroyStaticSizedArrayIntoFunctionIE;
use crate::instantiating::ast::expressions::DestroyStaticSizedArrayIntoLocalsIE;
use crate::instantiating::ast::expressions::ExpressionIE;
use crate::instantiating::ast::expressions::ExternFunctionCallIE;
use crate::instantiating::ast::expressions::FunctionCallIE;
use crate::instantiating::ast::expressions::InterfaceFunctionCallIE;
use crate::instantiating::ast::expressions::IsSameInstanceIE;
use crate::instantiating::ast::expressions::LocalLookupIE;
use crate::instantiating::ast::expressions::MutateIE;
use crate::instantiating::ast::expressions::NewRuntimeSizedArrayIE;
use crate::instantiating::ast::expressions::PopRuntimeSizedArrayIE;
use crate::instantiating::ast::expressions::PushRuntimeSizedArrayIE;
use crate::instantiating::ast::expressions::MemberLookupIE;
use crate::instantiating::ast::expressions::RestackifyIE;
use crate::instantiating::ast::expressions::RuntimeSizedArrayCapacityIE;
use crate::instantiating::ast::expressions::RuntimeSizedArrayLookupIE;
use crate::instantiating::ast::expressions::StaticArrayFromCallableIE;
use crate::instantiating::ast::expressions::StaticArrayFromValuesIE;
use crate::instantiating::ast::expressions::StaticSizedArrayLookupIE;
use crate::instantiating::ast::expressions::UpcastIE;
use crate::instantiating::ast::names::AnonymousSubstructConstructorNameI;
use crate::instantiating::ast::names::AnonymousSubstructConstructorTemplateNameI;
use crate::instantiating::ast::names::AnonymousSubstructImplNameI;
use crate::instantiating::ast::names::AnonymousSubstructImplTemplateNameI;
use crate::instantiating::ast::names::AnonymousSubstructNameI;
use crate::instantiating::ast::names::AnonymousSubstructTemplateNameI;
use crate::instantiating::ast::names::ClosureParamNameI;
use crate::instantiating::ast::names::ConstructingMemberNameI;
use crate::instantiating::ast::names::ForwarderFunctionNameI;
use crate::instantiating::ast::names::ForwarderFunctionTemplateNameI;
use crate::instantiating::ast::names::ICitizenTemplateNameI;
use crate::instantiating::ast::names::IImplNameI;
use crate::instantiating::ast::names::IImplTemplateNameI;
use crate::instantiating::ast::names::IInterfaceTemplateNameI;
use crate::instantiating::ast::names::ImplNameI;
use crate::instantiating::ast::names::ImplTemplateNameI;
use crate::instantiating::ast::names::InterfaceNameI;
use crate::instantiating::ast::names::InterfaceTemplateNameI;
use crate::instantiating::ast::names::IterableNameI;
use crate::instantiating::ast::names::IterationOptionNameI;
use crate::instantiating::ast::names::IteratorNameI;
use crate::instantiating::ast::names::LambdaCitizenNameI;
use crate::instantiating::ast::names::LambdaCitizenTemplateNameI;
use crate::instantiating::ast::names::MagicParamNameI;
use crate::instantiating::ast::names::RawArrayNameI;
use crate::instantiating::ast::names::RuntimeSizedArrayNameI;
use crate::instantiating::ast::names::RuntimeSizedArrayTemplateNameI;
use crate::instantiating::ast::names::SelfNameI;
use crate::instantiating::ast::names::StaticSizedArrayNameI;
use crate::instantiating::ast::names::StaticSizedArrayTemplateNameI;
use crate::instantiating::ast::names::StructNameI;
use crate::instantiating::ast::names::StructTemplateNameI;
use crate::instantiating::ast::names::TypingPassBlockResultVarNameI;
use crate::instantiating::ast::names::TypingPassTemporaryVarNameI;
use crate::instantiating::ast::templata::expect_integer_templata;
use crate::instantiating::ast::types::BoolIT;
use crate::instantiating::ast::types::IntIT;
use crate::instantiating::ast::types::KindIT;
use crate::instantiating::ast::types::StrIT;
use crate::instantiating::ast::types::BorrowRefIT;
use crate::instantiating::ast::types::OwnRefIT;
use crate::instantiating::ast::types::ShareRefIT;
use crate::instantiating::ast::types::WeakRefIT;
use crate::instantiating::ast::types::USizeIT;
use crate::instantiating::ast::types::VoidIT;
use crate::typing::ast::ast::ICitizenAttributeT;
use crate::typing::ast::ast::LocationInFunctionEnvironmentT;
use crate::typing::ast::expressions::ArgLookupTE;
use crate::typing::ast::expressions::ArrayLengthTE;
use crate::typing::ast::expressions::AsSubtypeTE;
use crate::typing::ast::expressions::BorrowToWeakTE;
use crate::typing::ast::expressions::ConstructTE;
use crate::typing::ast::expressions::DestroyRuntimeSizedArrayTE;
use crate::typing::ast::expressions::DestroyStaticSizedArrayIntoFunctionTE;
use crate::typing::ast::expressions::DestroyStaticSizedArrayIntoLocalsTE;
use crate::typing::ast::expressions::DestroyTE;
use crate::typing::ast::expressions::ExternFunctionCallTE;
use crate::typing::ast::expressions::FunctionCallTE;
use crate::typing::ast::expressions::InterfaceFunctionCallTE;
use crate::typing::ast::expressions::IsSameInstanceTE;
use crate::typing::ast::expressions::LetAndLendTE;
use crate::typing::ast::expressions::LocalLookupTE;
use crate::typing::ast::expressions::LockWeakTE;
use crate::typing::ast::expressions::MutateTE;
use crate::typing::ast::expressions::NewRuntimeSizedArrayTE;
use crate::typing::ast::expressions::PopRuntimeSizedArrayTE;
use crate::typing::ast::expressions::PushRuntimeSizedArrayTE;
use crate::typing::ast::expressions::MemberLookupTE;
use crate::typing::ast::expressions::RuntimeSizedArrayCapacityTE;
use crate::typing::ast::expressions::RuntimeSizedArrayLookupTE;
use crate::typing::ast::expressions::DerefTE;
use crate::typing::ast::expressions::StaticArrayFromCallableTE;
use crate::typing::ast::expressions::StaticArrayFromValuesTE;
use crate::typing::ast::expressions::StaticSizedArrayLookupTE;
use crate::typing::ast::expressions::UpcastTE;
use crate::typing::names::names::AnonymousSubstructConstructorNameT;
use crate::typing::names::names::AnonymousSubstructConstructorTemplateNameT;
use crate::typing::names::names::AnonymousSubstructImplNameT;
use crate::typing::names::names::AnonymousSubstructImplTemplateNameT;
use crate::typing::names::names::AnonymousSubstructTemplateNameT;
use crate::typing::names::names::ClosureParamNameT;
use crate::typing::names::names::ExternFunctionNameT;
use crate::typing::names::names::ForwarderFunctionNameT;
use crate::typing::names::names::ForwarderFunctionTemplateNameT;
use crate::typing::names::names::FunctionBoundNameT;
use crate::typing::names::names::FunctionBoundTemplateNameT;
use crate::typing::names::names::ICitizenTemplateNameT;
use crate::typing::names::names::IInstantiationNameT;
use crate::typing::names::names::IInterfaceNameT;
use crate::typing::names::names::INameT;
use crate::typing::names::names::ImplNameT;
use crate::typing::names::names::ImplTemplateNameT;
use crate::typing::names::names::InterfaceNameT;
use crate::typing::names::names::InterfaceTemplateNameT;
use crate::typing::names::names::IterableNameT;
use crate::typing::names::names::IterationOptionNameT;
use crate::typing::names::names::IteratorNameT;
use crate::typing::names::names::MagicParamNameT;
use crate::typing::names::names::RawArrayNameT;
use crate::typing::names::names::RuntimeSizedArrayNameT;
use crate::typing::names::names::StaticSizedArrayNameT;
use crate::typing::names::names::StructTemplateNameT;
use crate::typing::names::names::TypingPassBlockResultVarNameT;
use crate::typing::names::names::TypingPassTemporaryVarNameT;
use crate::typing::templata::templata::PlaceholderTemplataT;
use crate::typing::types::types::RegionT;
use crate::utils::utils::union_maps_expect_no_conflict;
use crate::utils::fx::HashMap;
use crate::utils::fx::HashSet;
use std::marker::PhantomData;
use std::mem::discriminant;
use std::mem::transmute;
use crate::instantiating::ast::types::SharednessI;
use crate::instantiating::instantiated_humanizer::humanize_name;
use crate::typing::types::types::KindT;
use crate::typing::types::types::SharednessT;

/// Temporary state
#[derive(Clone, PartialEq, Eq)]
pub struct DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i> where 's: 't, 's: 'i {
    pub func_id_to_bound_arg_prototype: IndexMap<IdT<'s, 't>, &'i PrototypeI<'s, 'i>>,
    pub bound_param_impl_id_to_bound_arg_impl_id: IndexMap<IdT<'s, 't>, IdI<'s, 'i>>,
}



impl<'s, 't, 'i> DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i> where 's: 't, 's: 'i {
    pub fn plus(&self, that: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>) -> DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i> {
        DenizenBoundToDenizenCallerBoundArgI {
            func_id_to_bound_arg_prototype: union_maps_expect_no_conflict(&self.func_id_to_bound_arg_prototype, &that.func_id_to_bound_arg_prototype, |x, y| x == y),
            bound_param_impl_id_to_bound_arg_impl_id: union_maps_expect_no_conflict(&self.bound_param_impl_id_to_bound_arg_impl_id, &that.bound_param_impl_id_to_bound_arg_impl_id, |x, y| x == y),
        }
    }
}


/// Temporary state
pub struct InstantiatedOutputsI<'s, 't, 'i> where 's: 't, 's: 'i {
    pub functions: IndexMap<IdI<'s, 'i>, &'i FunctionDefinitionI<'s, 'i>>,
    pub structs: IndexMap<IdI<'s, 'i>, &'i StructDefinitionI<'s, 'i>>,
    pub static_sized_arrays: IndexMap<IdI<'s, 'i>, &'i StaticSizedArrayIT<'s, 'i>>,
    pub runtime_sized_arrays: IndexMap<IdI<'s, 'i>, &'i RuntimeSizedArrayIT<'s, 'i>>,
    pub interfaces_without_methods: IndexMap<IdI<'s, 'i>, &'i InterfaceDefinitionI<'s, 'i>>,
    pub struct_to_sharedness: IndexMap<IdI<'s, 'i>, SharednessI>,
    pub struct_to_bounds: IndexMap<IdI<'s, 'i>, DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>,
    pub interface_to_sharedness: IndexMap<IdI<'s, 'i>, SharednessI>,
    pub interface_to_bounds: IndexMap<IdI<'s, 'i>, DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>,
    pub impl_to_sharedness: IndexMap<IdI<'s, 'i>, SharednessI>,
    pub impl_to_bounds: IndexMap<IdI<'s, 'i>, DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>,
    pub interface_to_impls: IndexMap<IdI<'s, 'i>, Vec<(IdT<'s, 't>, IdI<'s, 'i>)>>,
    // Inner value is (virtual_param_index, index_in_edge). index_in_edge is the method's vtable
    // slot = its position in typing's InterfaceEdgeBlueprintT.super_family_root_headers. After the
    // worklist drains, each inner map is sorted by index_in_edge so the blueprint/internal_methods/
    // edge all emit in typing's order (matching the slot stamped on each InterfaceFunctionCallIE).
    pub interface_to_abstract_func_to_virtual_index: IndexMap<IdI<'s, 'i>, IndexMap<PrototypeI<'s, 'i>, (usize, i32)>>,
    pub impls: IndexMap<IdI<'s, 'i>, (ICitizenIT<'s, 'i>, IdI<'s, 'i>, DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, InstantiationBoundArgumentsI<'s, 'i>)>,
    pub abstract_func_to_bounds: IndexMap<IdI<'s, 'i>, (DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, &'i InstantiationBoundArgumentsI<'s, 'i>)>,
    pub interface_to_impl_to_abstract_prototype_to_override: IndexMap<IdI<'s, 'i>, IndexMap<IdI<'s, 'i>, IndexMap<PrototypeI<'s, 'i>, PrototypeI<'s, 'i>>>>,
    pub new_impls: Vec<(IdT<'s, 't>, IdI<'s, 'i>, InstantiationBoundArgumentsI<'s, 'i>)>,
    pub new_abstract_funcs: Vec<(PrototypeT<'s, 't>, PrototypeI<'s, 'i>, usize, IdI<'s, 'i>, InstantiationBoundArgumentsI<'s, 'i>)>,
    pub new_functions: Vec<(PrototypeT<'s, 't>, PrototypeI<'s, 'i>, InstantiationBoundArgumentsI<'s, 'i>, Option<DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>)>,
    pub kind_externs: Vec<KindExternI<'s, 'i>>,
    pub function_externs: Vec<FunctionExternI<'s, 'i>>,
}



impl<'s, 't, 'i> InstantiatedOutputsI<'s, 't, 'i> where 's: 't, 's: 'i {
  pub fn new() -> Self {
    InstantiatedOutputsI {
      functions: IndexMap::default(),
      structs: IndexMap::default(),
      static_sized_arrays: IndexMap::default(),
      runtime_sized_arrays: IndexMap::default(),
      interfaces_without_methods: IndexMap::default(),
      struct_to_sharedness: IndexMap::default(),
      struct_to_bounds: IndexMap::default(),
      interface_to_sharedness: IndexMap::default(),
      interface_to_bounds: IndexMap::default(),
      impl_to_sharedness: IndexMap::default(),
      impl_to_bounds: IndexMap::default(),
      interface_to_impls: IndexMap::default(),
      interface_to_abstract_func_to_virtual_index: IndexMap::default(),
      impls: IndexMap::default(),
      abstract_func_to_bounds: IndexMap::default(),
      interface_to_impl_to_abstract_prototype_to_override: IndexMap::default(),
      new_impls: Vec::new(),
      new_abstract_funcs: Vec::new(),
      new_functions: Vec::new(),
      kind_externs: Vec::new(),
      function_externs: Vec::new(),
    }
  }

    pub fn add_method_to_v_table(&mut self, impl_id: IdI<'s, 'i>, super_interface_id: IdI<'s, 'i>, abstract_func_prototype: PrototypeI<'s, 'i>, override_: PrototypeI<'s, 'i>) {
        let map = self.interface_to_impl_to_abstract_prototype_to_override
            .entry(super_interface_id).or_insert_with(IndexMap::default)
            .entry(impl_id).or_insert_with(IndexMap::default);
        assert!(!map.contains_key(&abstract_func_prototype));
        map.insert(abstract_func_prototype, override_);
    }
}


pub fn translate<'s, 'ctx, 't, 'i>(opts: &'ctx GlobalOptions, interner: &'ctx InstantiatingInterner<'s, 'i>, typing_interner: &'ctx TypingInterner<'s, 't>, scout_arena: &'ctx ScoutArena<'s>, keywords: &'ctx Keywords<'s>, hinputs: &'ctx HinputsT<'s, 't>) -> HinputsI<'s, 'i>
where 's: 't, 's: 'i {
    let mut monouts = InstantiatedOutputsI::new();
    let instantiator = InstantiatorI { opts, interner, typing_interner, scout_arena, keywords, hinputs };
    instantiator.translate_method(&mut monouts)
}


/// Temporary state
pub struct InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub opts: &'ctx GlobalOptions,
    pub interner: &'ctx InstantiatingInterner<'s, 'i>,
    pub typing_interner: &'ctx TypingInterner<'s, 't>,
    pub scout_arena: &'ctx ScoutArena<'s>,
    pub keywords: &'ctx Keywords<'s>,
    pub hinputs: &'ctx HinputsT<'s, 't>,
}



impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_method(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>) -> HinputsI<'s, 'i> {
        let HinputsT {
            interfaces: _interfaces_t,
            structs: _structs_t,
            functions: _functions_t,
            interface_to_edge_blueprints: _interface_to_edge_blueprints_t,
            interface_to_sub_citizen_to_edge: _interface_to_sub_citizen_to_edge_t,
            instantiation_name_to_instantiation_bounds: _instantiation_name_to_function_bound_to_rune_t,
            kind_exports: kind_exports_t,
            function_exports: function_exports_t,
            kind_externs: _kind_externs_t,
            function_externs: function_externs_t,
            sub_citizen_to_interface_to_edge: _,
        } = self.hinputs;

        let kind_exports: Vec<KindExportI<'s, 'i>> =
            kind_exports_t.iter().map(|&kind_export_t| {
                let KindExportT { range, tyype, id: export_placeholdered_id_t, exported_name } = kind_export_t;
                let export_id = self.translate_id(
                    export_placeholdered_id_t,
                    |export_name_t: &INameT<'s, 't>| -> INameI<'s, 'i> {
                        match export_name_t {
                            INameT::Export(ExportNameT { template: ExportTemplateNameT { code_loc, .. }, .. }) => {
                                INameI::Export(self.interner.alloc(ExportNameI {
                                    template: ExportTemplateNameI { code_loc: *code_loc },
                                }))
                            }
                            _ => {
                                panic!("Unimplemented: translate_method kind_exports translateId closure");
                                // case other => vimpl(other)
                            }
                        }
                    });
                let substitutions = self.assemble_placeholder_map(export_placeholdered_id_t, &export_id);
                let denizen_bound_to_denizen_caller_supplied_thing = DenizenBoundToDenizenCallerBoundArgI {
                    func_id_to_bound_arg_prototype: IndexMap::default(),
                    bound_param_impl_id_to_bound_arg_impl_id: IndexMap::default(),
                };
                let kind_it = self.translate_kind(
                    monouts,
                    export_placeholdered_id_t,
                    &denizen_bound_to_denizen_caller_supplied_thing,
                    &substitutions,
                    &RegionT::Default,
                    &tyype);
                KindExportI {
                    range: *range,
                    tyype: kind_it,
                    id: export_id,
                    exported_name: *exported_name,
                }
            }).collect();

        let function_exports: Vec<FunctionExportI<'s, 'i>> =
            function_exports_t.iter().map(|&function_export_t| {
                let FunctionExportT { range, prototype: prototype_t, export_id: export_placeholdered_id_t, exported_name } = function_export_t;
                let perspective_region_t = RegionT::Default;
                let export_id = self.translate_id(
                    export_placeholdered_id_t,
                    |export_name_t: &INameT<'s, 't>| -> INameI<'s, 'i> {
                        match export_name_t {
                            INameT::Export(ExportNameT { template: ExportTemplateNameT { code_loc, .. }, .. }) => {
                                INameI::Export(self.interner.alloc(ExportNameI {
                                    template: ExportTemplateNameI { code_loc: *code_loc },
                                }))
                            }
                            _ => {
                                panic!("Unimplemented: translate_method function_exports translateId closure");
                                // case other => vimpl(other)
                            }
                        }
                    });
                let substitutions = self.assemble_placeholder_map(export_placeholdered_id_t, &export_id);

                let denizen_bound_to_denizen_caller_supplied_thing = DenizenBoundToDenizenCallerBoundArgI {
                    func_id_to_bound_arg_prototype: IndexMap::default(),
                    bound_param_impl_id_to_bound_arg_impl_id: IndexMap::default(),
                };
                let prototype =
                    self.translate_prototype(
                        monouts,
                        export_placeholdered_id_t,
                        &denizen_bound_to_denizen_caller_supplied_thing,
                        &substitutions,
                        &perspective_region_t,
                        &prototype_t);

                FunctionExportI {
                    range: *range,
                    prototype: self.interner.alloc(PrototypeI { id: prototype.id, return_type: prototype.return_type }),
                    export_id,
                    exported_name: *exported_name,
                }
            }).collect();

        let non_generic_func_externs: Vec<FunctionExternI<'s, 'i>> =
            function_externs_t.iter().flat_map(|&function_extern_t| -> Option<FunctionExternI<'s, 'i>> {
                let FunctionExternT { range: _range, extern_placeholdered_id: extern_placeholdered_id_t, prototype: prototype_t, extern_name: _externed_name, generic_parameter_inheritance: maybe_inheritance } = function_extern_t;
                let is_generic = !IInstantiationNameT::try_from(prototype_t.id.local_name).unwrap().template_args().is_empty();
                if is_generic {
                    // We don't handle generic externs yet, that comes later when we see what instantiations are actually needed.
                    // We handle those like we handle normal non-extern generic functions.
                    None
                } else {
                    let perspective_region_t = RegionT::Default;

                    let extern_id = self.translate_id(
                        extern_placeholdered_id_t,
                        |extern_name_t: &INameT<'s, 't>| -> INameI<'s, 'i> {
                            match extern_name_t {
                                INameT::Extern(ExternNameT { template: ExternTemplateNameT { code_loc, .. }, .. }) => {
                                    INameI::Extern(self.interner.alloc(ExternNameI {
                                        template: ExternTemplateNameI { code_loc: *code_loc },
                                    }))
                                }
                                _ => {
                                    panic!("Unimplemented: translate_method function_externs translateId closure");
                                    // case other => vimpl(other)
                                }
                            }
                        });
                    let substitutions = self.assemble_placeholder_map(extern_placeholdered_id_t, &extern_id);

                    let denizen_bound_to_denizen_caller_supplied_thing = DenizenBoundToDenizenCallerBoundArgI {
                        func_id_to_bound_arg_prototype: IndexMap::default(),
                        bound_param_impl_id_to_bound_arg_impl_id: IndexMap::default(),
                    };
                    let prototype =
                        self.translate_prototype(
                            monouts,
                            extern_placeholdered_id_t,
                            &denizen_bound_to_denizen_caller_supplied_thing,
                            &substitutions,
                            &perspective_region_t,
                            &prototype_t);

                    Some(FunctionExternI {
                        prototype: self.interner.alloc(PrototypeI { id: prototype.id, return_type: prototype.return_type }),
                        num_inherited_generic_parameters: maybe_inheritance.as_ref().map(|i| i.num_inherited_generic_parameters).unwrap_or(0),
                    })
                }
            }).collect();

        while {
            // We make structs and interfaces eagerly as we come across them
            // if (monouts.newStructs.nonEmpty) {
            //   val newStructName = monouts.newStructs.dequeue()
            //   DenizentranslateStructDefinition(opts, interner, keywords, hinputs, monouts, newStructName)
            //   true
            // } else if (monouts.newInterfaces.nonEmpty) {
            //   val (newInterfaceName, calleeRuneToSuppliedPrototype) = monouts.newInterfaces.dequeue()
            //   DenizentranslateInterfaceDefinition(
            //     opts, interner, keywords, hinputs, monouts, newInterfaceName, calleeRuneToSuppliedPrototype)
            //   true
            // } else
            if !monouts.new_functions.is_empty() {
                let (new_func_id_t, new_func_id, instantiation_bound_args, maybe_denizen_bound_to_denizen_caller_supplied_thing) =
                    monouts.new_functions.remove(0);
                self.translate_function_callsite(
                    monouts, &new_func_id_t, &new_func_id, &instantiation_bound_args,
                    maybe_denizen_bound_to_denizen_caller_supplied_thing.as_ref());
                true
            } else if !monouts.new_impls.is_empty() {
                let (impl_id_t, impl_id, instantiation_bounds_for_unsubstituted_impl) = monouts.new_impls.remove(0);
                self.translate_impl_callsite(monouts, &impl_id_t, &impl_id, instantiation_bounds_for_unsubstituted_impl);
                true
            } else if !monouts.new_abstract_funcs.is_empty() {
                let (abstract_func_t, abstract_func, virtual_index, interface_id, instantiation_bound_args) = monouts.new_abstract_funcs.remove(0);
                self.translate_abstract_func(monouts, &interface_id, &abstract_func_t, &abstract_func, virtual_index, instantiation_bound_args);
                true
            } else {
                false
            }
        } {}

        // Reorder each interface's methods into typing's blueprint slot order, so the blueprint,
        // internal_methods and edges below all emit in the order the call sites' index_in_edge uses.
        for (_interface, abstract_funcs) in monouts.interface_to_abstract_func_to_virtual_index.iter_mut() {
            abstract_funcs.sort_by(|_k1, v1, _k2, v2| v1.1.cmp(&v2.1));
        }

        let interface_edge_blueprints =
            ArenaIndexMap::from_iter_in(
                monouts.interface_to_abstract_func_to_virtual_index.iter().map(|(interface, abstract_func_prototypes)| -> (IdI<'s, 'i>, InterfaceEdgeBlueprintI<'s, 'i>) {
                    let mut entries: Vec<(&'i PrototypeI<'s, 'i>, i32)> = Vec::new();
                    for (proto, (idx, _index_in_edge)) in abstract_func_prototypes.iter() {
                        entries.push((self.interner.alloc(*proto), *idx as i32));
                    }
                    (*interface, InterfaceEdgeBlueprintI { interface: *interface, super_family_root_headers: self.interner.bump().alloc_slice_fill_iter(entries.into_iter()) })
                }),
                self.interner.bump());

        let interfaces: Vec<InterfaceDefinitionI<'s, 'i>> =
            monouts.interfaces_without_methods.values().map(|interface| {
                let InterfaceDefinitionI { instantiated_interface: ref_, attributes, weakable, sharedness: mutability, .. } = **interface;
                let map = monouts.interface_to_abstract_func_to_virtual_index.get(&ref_.id).expect("vassertSome: interface_to_abstract_func_to_virtual_index");
                let mut methods_entries: Vec<(&'i PrototypeI<'s, 'i>, i32)> = Vec::new();
                for (proto, (idx, _index_in_edge)) in map.iter() {
                    methods_entries.push((self.interner.alloc(*proto), *idx as i32));
                }
                InterfaceDefinitionI {
                    instantiated_interface: ref_,
                    attributes,
                    weakable,
                  sharedness: mutability,
                    rune_to_function_bound: ArenaIndexMap::new_in(self.interner.bump()),
                    rune_to_impl_bound: ArenaIndexMap::new_in(self.interner.bump()),
                    internal_methods: self.interner.bump().alloc_slice_fill_iter(methods_entries.into_iter()),
                                    }
            }).collect();

        let interface_to_sub_citizen_to_edge =
            ArenaIndexMap::from_iter_in(
                monouts.interface_to_impls.iter().map(|(interface, impls)| -> (IdI<'s, 'i>, ArenaIndexMap<'i, IdI<'s, 'i>, EdgeI<'s, 'i>>) {
                    let inner_iter = impls.iter().map(|(_impl_id_t, impl_id_i)| -> (IdI<'s, 'i>, EdgeI<'s, 'i>) {
                        let (sub_citizen, parent_interface, _, _) = monouts.impls.get(impl_id_i).expect("vassertSome: monouts.impls");
                        assert!(parent_interface == interface);
                        let abstract_func_to_virtual_index = monouts.interface_to_abstract_func_to_virtual_index.get(interface).expect("vassertSome: interface_to_abstract_func_to_virtual_index");
                        let abstract_func_prototype_to_override_prototype = abstract_func_to_virtual_index.iter().map(|(abstract_func_prototype, (virtual_index, _index_in_edge))| -> (IdI<'s, 'i>, &'i PrototypeI<'s, 'i>) {
                            let override_prototype = monouts.interface_to_impl_to_abstract_prototype_to_override
                                .get(interface).expect("vassertSome interface_to_impl_to_abstract_prototype_to_override (interface)")
                                .get(impl_id_i).expect("vassertSome interface_to_impl_to_abstract_prototype_to_override (impl)")
                                .get(abstract_func_prototype).expect("vassertSome interface_to_impl_to_abstract_prototype_to_override (abstract_func_prototype)");
                            assert!(IFunctionNameI::try_from(abstract_func_prototype.id.local_name).unwrap().parameters()[*virtual_index] !=
                                IFunctionNameI::try_from(override_prototype.id.local_name).unwrap().parameters()[*virtual_index]);
                            (abstract_func_prototype.id, self.interner.alloc(*override_prototype))
                        });
                        let edge = EdgeI {
                            edge_id: *impl_id_i,
                            sub_citizen: *sub_citizen,
                            super_interface: *interface,
                            rune_to_func_bound: ArenaIndexMap::new_in(self.interner.bump()),
                            rune_to_impl_bound: ArenaIndexMap::new_in(self.interner.bump()),
                            abstract_func_to_override_func: ArenaIndexMap::from_iter_in(abstract_func_prototype_to_override_prototype, self.interner.bump()),
                        };
                        (sub_citizen.id(), edge)
                    });
                    (*interface, ArenaIndexMap::from_iter_in(inner_iter, self.interner.bump()))
                }),
                self.interner.bump());

        let result_hinputs =
            HinputsI {
                interfaces: self.interner.alloc_slice_from_vec(interfaces),
                structs: self.interner.alloc_slice_from_vec(monouts.structs.values().copied().collect()),
                static_sized_arrays: self.interner.alloc_slice_from_vec(monouts.static_sized_arrays.values().copied().collect()),
                runtime_sized_arrays: self.interner.alloc_slice_from_vec(monouts.runtime_sized_arrays.values().copied().collect()),
                functions: self.interner.alloc_slice_from_vec(monouts.functions.values().copied().collect()),
                interface_to_edge_blueprints: interface_edge_blueprints,
                interface_to_sub_citizen_to_edge,
                kind_exports: self.interner.alloc_slice_from_vec(kind_exports),
                function_exports: self.interner.alloc_slice_from_vec(function_exports),
                kind_externs: ArenaIndexMap::from_iter_in(
                    monouts.kind_externs.iter().map(|x| -> (&'i StructIT<'s, 'i>, KindExternI<'s, 'i>) {
                        (x.r#struct, *x)
                    }),
                    self.interner.bump()),
                function_externs: self.interner.alloc_slice_from_vec(
                    non_generic_func_externs.into_iter().chain(monouts.function_externs.iter().copied()).collect()),
            };
        result_hinputs
    }

    pub fn translate_id(
        &self,
        id_t: &IdT<'s, 't>,
        func: impl Fn(&INameT<'s, 't>) -> INameI<'s, 'i>,
    ) -> IdI<'s, 'i> {
        let init_steps_i = id_t.init_steps.iter().map(Self::translate_name).collect::<Vec<_>>();
        IdI {
            package_coord: id_t.package_coord,
            init_steps: self.interner.alloc_slice_from_vec(init_steps_i),
            local_name: func(&id_t.local_name),
        }
    }


    pub fn translate_export_name(_denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, _export_name_t: &ExportNameT<'s, 't>) -> ExportNameI<'s> {
        panic!("Unimplemented: translate_export_name");
        // val ExportNameT(ExportTemplateNameT(codeLoc), _) = exportNameT
        // ExportNameI(
        //   ExportTemplateNameI(codeLoc),
        //   RegionTemplataI(0))
    }


    pub fn translate_export_template_name(_export_template_name_t: &ExportTemplateNameT<'s>) -> ExportTemplateNameI<'s> {
        panic!("Unimplemented: translate_export_template_name");
        // val ExportTemplateNameT(codeLoc) = exportTemplateNameT
        // ExportTemplateNameI(codeLoc)
    }


    pub fn translate_name(_t: &INameT<'s, 't>) -> INameI<'s, 'i> {
        panic!("Unimplemented: translate_name");
        // vimpl()
    }


    pub fn translate_interface_callsite(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _interface_id_t: &IdT<'s, 't>, _interface_id: &IdI<'s, 'i>, _instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) {
        let interface_def_t = self.find_interface(_interface_id_t);
        let denizen_bound_to_denizen_caller_supplied_thing = Self::assemble_instantiation_bound_param_to_arg(&interface_def_t.instantiation_bound_params, _instantiation_bound_args);
        if let Some(x) = _monouts.interface_to_bounds.get(_interface_id) {
            assert!(*x == denizen_bound_to_denizen_caller_supplied_thing, "vcurious: interface_to_bounds mismatch");
        }
        _monouts.interface_to_bounds.insert(*_interface_id, denizen_bound_to_denizen_caller_supplied_thing.clone());
        let substitutions = self.assemble_placeholder_map(&interface_def_t.instantiated_interface.id, _interface_id);
        self.translate_interface_definition(_monouts, _interface_id_t, &denizen_bound_to_denizen_caller_supplied_thing, &substitutions, _interface_id, interface_def_t);
    }


    pub fn assemble_instantiation_bound_param_to_arg(instantiation_bound_params: &InstantiationBoundArgumentsT<'s, 't>, instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) -> DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i> {
        assert!(instantiation_bound_args.rune_to_function_bound_arg.len() == instantiation_bound_params.rune_to_bound_prototype.len());
        assert!(
            instantiation_bound_args.caller_rune_to_callee_rune_to_reachable_func.iter().filter(|(_, v)| !v.is_empty()).count() ==
                instantiation_bound_params.rune_to_citizen_rune_to_reachable_prototype.iter().filter(|(_, v)| !v.citizen_rune_to_reachable_prototype.is_empty()).count());
        assert!(instantiation_bound_args.rune_to_impl_bound_arg.len() == instantiation_bound_params.rune_to_bound_impl.len());
        DenizenBoundToDenizenCallerBoundArgI {
            func_id_to_bound_arg_prototype:
                instantiation_bound_args.rune_to_function_bound_arg.iter().map(|(callee_rune, supplied_function_i)| -> (IdT<'s, 't>, &'i PrototypeI<'s, 'i>) {
                    (instantiation_bound_params.rune_to_bound_prototype.get(callee_rune).expect("vassertSome: rune_to_bound_prototype").id, *supplied_function_i)
                }).chain(
                    instantiation_bound_args.caller_rune_to_callee_rune_to_reachable_func.iter().flat_map(|(caller_rune, callee_rune_to_reachable_func)| -> Vec<(IdT<'s, 't>, &'i PrototypeI<'s, 'i>)> {
                        if !callee_rune_to_reachable_func.is_empty() {
                            let m = instantiation_bound_params.rune_to_citizen_rune_to_reachable_prototype.get(caller_rune).expect("vassertSome: rune_to_citizen_rune_to_reachable_prototype");
                            assert!(m.citizen_rune_to_reachable_prototype.len() == callee_rune_to_reachable_func.len());
                            callee_rune_to_reachable_func.iter().map(|(callee_rune, reachable_func_i)| {
                                let reachable_func_t = m.citizen_rune_to_reachable_prototype.get(callee_rune).expect("vassertSome: citizen_rune_to_reachable_prototype");
                                (reachable_func_t.id, *reachable_func_i)
                            }).collect()
                        } else {
                            Vec::new()
                        }
                    })
                ).collect(),
            bound_param_impl_id_to_bound_arg_impl_id:
                instantiation_bound_args.rune_to_impl_bound_arg.iter().map(|(callee_rune, supplied_impl_t)| -> (IdT<'s, 't>, IdI<'s, 'i>) {
                    (*instantiation_bound_params.rune_to_bound_impl.get(callee_rune).expect("vassertSome: rune_to_bound_impl"), *supplied_impl_t)
                }).collect(),
        }
    }


    pub fn assemble_callee_denizen_function_bounds(_callee_rune_to_receiver_bound_t: &IndexMap<IRuneS<'s>, IdT<'s, 't>>, _callee_rune_to_supplied_prototype: &IndexMap<IRuneS<'s>, PrototypeI<'s, 'i>>) -> IndexMap<IdT<'s, 't>, PrototypeI<'s, 'i>> {
        panic!("Unimplemented: assemble_callee_denizen_function_bounds");
        // calleeRuneToSuppliedPrototype.map({ case (calleeRune, suppliedFunctionT) =>
        //   vassertSome(calleeRuneToReceiverBoundT.get(calleeRune)) -> suppliedFunctionT
        // })
    }


    pub fn assemble_callee_denizen_impl_bounds(_callee_rune_to_receiver_bound_t: &IndexMap<IRuneS<'s>, IdT<'s, 't>>, _callee_rune_to_supplied_impl: &IndexMap<IRuneS<'s>, IdI<'s, 'i>>) -> IndexMap<IdT<'s, 't>, IdI<'s, 'i>> {
        panic!("Unimplemented: assemble_callee_denizen_impl_bounds");
        // calleeRuneToSuppliedImpl.map({ case (calleeRune, suppliedFunctionT) =>
        //   vassertSome(calleeRuneToReceiverBoundT.get(calleeRune)) -> suppliedFunctionT
        // })
    }


    pub fn translate_struct_callsite(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _struct_id_t: &IdT<'s, 't>, _struct_id: &IdI<'s, 'i>, _instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) {
        let struct_def_t = self.find_struct(_struct_id_t);
        let denizen_bound_to_denizen_caller_supplied_thing =
            Self::assemble_instantiation_bound_param_to_arg(&struct_def_t.instantiation_bound_params, _instantiation_bound_args);
        match _monouts.struct_to_bounds.get(_struct_id) {
            Some(_x) => {
                return;
            }
            None => {}
        }
        _monouts.struct_to_bounds.insert(*_struct_id, denizen_bound_to_denizen_caller_supplied_thing.clone());
        let substitutions = self.assemble_placeholder_map(&struct_def_t.instantiated_citizen.id, _struct_id);
        self.translate_struct_definition(_monouts, _struct_id_t, &denizen_bound_to_denizen_caller_supplied_thing, &substitutions, _struct_id_t, _struct_id, struct_def_t);
    }


    pub fn find_struct(&self, _struct_id: &IdT<'s, 't>) -> &'t StructDefinitionT<'s, 't> {
        let target = Compiler::get_super_template(self.typing_interner, *_struct_id);
        let matches: Vec<_> = self.hinputs.structs.iter().filter(|s| Compiler::get_super_template(self.typing_interner, s.instantiated_citizen.id) == target).collect();
        assert_eq!(matches.len(), 1);
        matches[0]
    }


    pub fn find_interface(&self, _interface_id: &IdT<'s, 't>) -> &'t InterfaceDefinitionT<'s, 't> {
        let target = Compiler::get_super_template(self.typing_interner, *_interface_id);
        let matches: Vec<_> = self.hinputs.interfaces.iter().filter(|i| Compiler::get_super_template(self.typing_interner, i.instantiated_interface.id) == target).collect();
        assert_eq!(matches.len(), 1);
        matches[0]
    }


    pub fn find_impl(&self, _impl_id: &IdT<'s, 't>) -> &'t EdgeT<'s, 't> {
        panic!("Unimplemented: find_impl");
        // vassertOne(
        //   hinputs.interfaceToSubCitizenToEdge.values.flatMap(subCitizenToEdge => {
        //     subCitizenToEdge.values.filter(edge => {
        //       TemplataCompiler.getSuperTemplate(edge.edgeId) ==
        //           TemplataCompiler.getSuperTemplate(implId)
        //     })
        //   }))
    }


    pub fn translate_override(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, impl_id_t: &IdT<'s, 't>, _impl_id: &IdI<'s, 'i>, abstract_func_prototype_t: &PrototypeT<'s, 't>, _abstract_func_prototype: &PrototypeI<'s, 'i>, _abstract_func_instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) {
        let impl_template_id = Compiler::get_impl_template(self.typing_interner, *impl_id_t);
        let edge_t = vassert_one(
            self.hinputs.interface_to_sub_citizen_to_edge.values()
                .flat_map(|sub_to_edge| sub_to_edge.values().copied())
                .filter(|edge| Compiler::get_impl_template(self.typing_interner, edge.edge_id) == impl_template_id));
        let _edge_id = edge_t.edge_id;
        let _edge_sub_citizen = edge_t.sub_citizen;
        let _edge_super_interface = edge_t.super_interface;
        let edge_abstract_func_to_override_func = &edge_t.abstract_func_to_override_func;
        let abstract_func_template_name = Compiler::get_function_template(self.typing_interner, abstract_func_prototype_t.id);
        let abstract_func_placeholdered_name_t = self.hinputs.functions.iter().copied()
            .find(|func| Compiler::get_function_template(self.typing_interner, func.header.id) == abstract_func_template_name)
            .expect("vassertSome abstractFuncPlaceholderedNameT")
            .header.id;
        let override_t = *edge_abstract_func_to_override_func.get(&abstract_func_placeholdered_name_t).expect("vassertSome OverrideT");
        let dispatcher_id_t = override_t.dispatcher_call_id;
        let _impl_placeholder_to_dispatcher_placeholder = override_t.impl_placeholder_to_dispatcher_placeholder;
        let _impl_placeholder_to_case_placeholder = override_t.impl_placeholder_to_case_placeholder;
        let _dispatcher_and_case_placeholdered_impl_reachable_prototypes = &override_t.dispatcher_and_case_placeholdered_impl_reachable_prototypes;
        let _dispatcher_case_id_t = override_t.case_id;
        let _override_prototype_t = override_t.override_prototype;
        let _dispatcher_instantiation_bound_params = override_t.dispatcher_instantiation_bound_params;
        let _dispatcher_template_id = Compiler::get_template(self.typing_interner, dispatcher_id_t);
        let dispatcher_template_args = IInstantiationNameT::try_from(dispatcher_id_t.local_name).unwrap().template_args();
        let dispatcher_placeholder_id_to_supplied_templata: Vec<(IdT<'s, 't>, ITemplataI<'s, 'i>)> =
            dispatcher_template_args.iter().map(|dispatcher_placeholder_templata| {
                let dispatcher_placeholder_id = Compiler::get_placeholder_templata_id(*dispatcher_placeholder_templata);
                let impl_placeholder = _impl_placeholder_to_dispatcher_placeholder.iter().find(|(_, v)| v == dispatcher_placeholder_templata).expect("vassertSome implPlaceholderToDispatcherPlaceholder").0;
                let index = match impl_placeholder.local_name {
                    INameT::KindPlaceholder(kp) => kp.template.index,
                    INameT::NonKindNonRegionPlaceholder(nk) => nk.index,
                    _ => panic!("vwat translate_override dispatcher placeholder index"),
                };
                let impl_id_c_local: IImplNameI<'s, 'i> = _impl_id.local_name.try_into().unwrap();
                let templata: ITemplataI<'s, 'i> = impl_id_c_local.template_args()[index as usize];
                (dispatcher_placeholder_id, templata)
            }).collect();
        let case_local_name = match _dispatcher_case_id_t.local_name {
            INameT::OverrideDispatcherCase(n) => n,
            _ => panic!("translate_override: dispatcher_case_id_t.local_name not OverrideDispatcherCase"),
        };
        let dispatcher_case_placeholder_id_to_supplied_templata: Vec<(IdT<'s, 't>, ITemplataI<'s, 'i>)> =
            case_local_name.independent_impl_template_args.iter().enumerate().map(|(_enum_index, case_placeholder_templata)| {
                let case_placeholder_id = Compiler::get_placeholder_templata_id(*case_placeholder_templata);
                let impl_placeholder = _impl_placeholder_to_case_placeholder.iter().find(|(_, v)| v == case_placeholder_templata).expect("vassertSome implPlaceholderToCasePlaceholder").0;
                let index = match impl_placeholder.local_name {
                    INameT::KindPlaceholder(kp) => kp.template.index,
                    _ => panic!("vwat translate_override case placeholder index"),
                };
                let impl_id_c_local: IImplNameI<'s, 'i> = _impl_id.local_name.try_into().unwrap();
                let templata: ITemplataI<'s, 'i> = impl_id_c_local.template_args()[index as usize];
                (case_placeholder_id, templata)
            }).collect();
        let dispatcher_placeholder_id_to_supplied_templata_map: HashMap<IdT<'s, 't>, ITemplataI<'s, 'i>> =
            dispatcher_placeholder_id_to_supplied_templata.iter().copied().collect();
        let dispatcher_case_placeholder_id_to_supplied_templata_map: HashMap<IdT<'s, 't>, ITemplataI<'s, 'i>> =
            dispatcher_case_placeholder_id_to_supplied_templata.iter().copied().collect();
        assert!(dispatcher_placeholder_id_to_supplied_templata_map.len() + dispatcher_case_placeholder_id_to_supplied_templata_map.len() ==
            dispatcher_placeholder_id_to_supplied_templata_map.iter().chain(dispatcher_case_placeholder_id_to_supplied_templata_map.iter()).map(|(k, _)| *k).collect::<HashSet<_>>().len());
        let mut _case_substitutions: HashMap<IdT<'s, 't>, ITemplataI<'s, 'i>> = dispatcher_placeholder_id_to_supplied_templata_map.clone();
        _case_substitutions.extend(dispatcher_case_placeholder_id_to_supplied_templata_map.iter().map(|(k, v)| (*k, *v)));

        let impl_rune_to_impl_instantiation_bound_args = &_monouts.impls.get(_impl_id).expect("vassertSome monouts.impls").3;
        let _bound_param_prototype_t_to_bound_arg_prototype_i_from_impl: HashMap<IdT<'s, 't>, &'i PrototypeI<'s, 'i>> =
            _dispatcher_and_case_placeholdered_impl_reachable_prototypes.iter().flat_map(|(rune_in_impl, citizen_rune_to_bound)| {
                citizen_rune_to_bound.iter().map(move |(rune_in_citizen, prototype_t)| {
                    let INameT::FunctionBound(_fbn) = prototype_t.id.local_name else {
                        panic!("translate_override: prototype_t.id.local_name not FunctionBound");
                    };
                    let prototype_i = *impl_rune_to_impl_instantiation_bound_args.caller_rune_to_callee_rune_to_reachable_func
                        .get(rune_in_impl).expect("vassertSome rune_in_impl")
                        .get(rune_in_citizen).expect("vassertSome rune_in_citizen");
                    (prototype_t.id, prototype_i)
                })
            }).collect();
        let dispatcher_instantiation_bound_params_to_args = Self::assemble_instantiation_bound_param_to_arg(_dispatcher_instantiation_bound_params, _abstract_func_instantiation_bound_args);

        let mut bound_param_func_id_to_bound_arg_index_map: IndexMap<IdT<'s, 't>, &'i PrototypeI<'s, 'i>> = IndexMap::default();
        for (k, v) in _bound_param_prototype_t_to_bound_arg_prototype_i_from_impl.iter() {
            bound_param_func_id_to_bound_arg_index_map.insert(*k, *v);
        }
        let extra_bounds = DenizenBoundToDenizenCallerBoundArgI {
            func_id_to_bound_arg_prototype: bound_param_func_id_to_bound_arg_index_map,
            bound_param_impl_id_to_bound_arg_impl_id: IndexMap::default(),
        };
        let case_instantiation_bound_params_to_args = dispatcher_instantiation_bound_params_to_args.plus(&extra_bounds);

        let case_substitutions_idx: IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>> = _case_substitutions.iter().map(|(k, v)| (*k, *v)).collect();
        let override_prototype =
            self.translate_prototype(_monouts, &_dispatcher_case_id_t, &case_instantiation_bound_params_to_args, &case_substitutions_idx, &RegionT::Default, &_override_prototype_t);

        let super_interface_id = _monouts.impls.get(_impl_id).expect("vassertSome monouts.impls").1;
        _monouts.add_method_to_v_table(*_impl_id, super_interface_id, *_abstract_func_prototype, override_prototype);
    }


    pub fn translate_impl_callsite(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _impl_id_t: &IdT<'s, 't>, impl_id: &IdI<'s, 'i>, _instantiation_bounds_for_unsubstituted_impl: InstantiationBoundArgumentsI<'s, 'i>) {
        let impl_template_id = Compiler::get_impl_template(self.typing_interner, *_impl_id_t);
        let impl_definition = vassert_one(self.hinputs.interface_to_sub_citizen_to_edge.iter().flat_map(|(_, m)| m.values()).filter(|edge| {
            Compiler::get_impl_template(self.typing_interner, edge.edge_id) == impl_template_id
        }));

        let denizen_bound_to_denizen_caller_supplied_thing = Self::assemble_instantiation_bound_param_to_arg(&impl_definition.instantiation_bound_params, &_instantiation_bounds_for_unsubstituted_impl);
        let substitutions = self.assemble_placeholder_map(&impl_definition.edge_id, impl_id);
        self.translate_impl_definition(_monouts, _impl_id_t, _instantiation_bounds_for_unsubstituted_impl, &denizen_bound_to_denizen_caller_supplied_thing, &substitutions, _impl_id_t, impl_id, impl_definition);
    }


    pub fn translate_function_callsite(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, desired_prototype_t: &PrototypeT<'s, 't>, desired_prototype: &PrototypeI<'s, 'i>, _supplied_bound_args: &InstantiationBoundArgumentsI<'s, 'i>, _maybe_denizen_bound_to_denizen_caller_supplied_thing: Option<&DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>) -> &'i FunctionDefinitionI<'s, 'i> {
        let desired_func_super_template_name = Compiler::get_super_template(self.typing_interner, desired_prototype_t.id);
        let func_t =
            vassert_one(self.hinputs.functions.iter().filter(|func_t| {
                Compiler::get_super_template(self.typing_interner, func_t.header.id) == desired_func_super_template_name
            }));

        let denizen_bound_to_denizen_caller_supplied_thing =
            match _maybe_denizen_bound_to_denizen_caller_supplied_thing {
                Some(x) => x.clone(),
                None => Self::assemble_instantiation_bound_param_to_arg(&func_t.instantiation_bound_params, _supplied_bound_args),
            };
        let _args_m: Vec<_> = IFunctionNameI::try_from(desired_prototype.id.local_name).unwrap().parameters().iter().map(|c| *c).collect();
        let _params_t: Vec<_> = func_t.header.params.iter().map(|p| p.tyype).collect();

        let substitutions =
            self.assemble_placeholder_map(&func_t.header.id, &desired_prototype.id);

        let monomorphized_func_t =
            self.translate_function_definition(
                monouts, &desired_prototype_t.id, &denizen_bound_to_denizen_caller_supplied_thing, &substitutions, &desired_prototype, func_t);

        assert!(desired_prototype.return_type == monomorphized_func_t.header.return_type);

        monomorphized_func_t
    }


    pub fn translate_abstract_func(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, interface_id: &IdI<'s, 'i>, desired_abstract_prototype_t: &PrototypeT<'s, 't>, desired_abstract_prototype: &PrototypeI<'s, 'i>, virtual_index: usize, supplied_bound_args: InstantiationBoundArgumentsI<'s, 'i>) {
        let desired_abstract_prototype = *desired_abstract_prototype;

        let desired_super_template_id = Compiler::get_super_template(self.typing_interner, desired_abstract_prototype_t.id);
        let func_t = vassert_one(self.hinputs.functions.iter().copied().filter(|f| {
            Compiler::get_super_template(self.typing_interner, f.header.id) == desired_super_template_id
        }));

        let denizen_bound_to_denizen_caller_supplied_thing =
            Self::assemble_instantiation_bound_param_to_arg(&func_t.instantiation_bound_params, &supplied_bound_args);

        let _args_m: Vec<KindIT<'s, 'i>> = IFunctionNameI::try_from(desired_abstract_prototype.id.local_name).unwrap().parameters().iter().map(|c| *c).collect();
        let _params_t: Vec<KindT<'s, 't>> = func_t.header.params.iter().map(|p| p.tyype).collect();

        assert!(!monouts.abstract_func_to_bounds.contains_key(&desired_abstract_prototype.id));
        let supplied_bound_args_ref: &'i InstantiationBoundArgumentsI<'s, 'i> = self.interner.bump().alloc(supplied_bound_args);
        monouts.abstract_func_to_bounds.insert(desired_abstract_prototype.id, (denizen_bound_to_denizen_caller_supplied_thing, supplied_bound_args_ref));

        // The vtable slot is this method's position in typing's interface blueprint (typing owns
        // the order); stored so the map can be sorted by it, matching each call's index_in_edge.
        let typed_interface_id =
            match peel_all_references(desired_abstract_prototype_t.param_types()[virtual_index]) {
                KindT::Interface(ir) => ir.id,
                other => panic!("abstract func virtual param is not an interface: {:?}", other),
            };
        let index_in_edge =
            self.hinputs.interface_to_edge_blueprints.get(&typed_interface_id)
                .expect("vassertSome: interface_to_edge_blueprints for abstract func")
                .super_family_root_headers.iter()
                .position(|(header_proto, _)| header_proto.id == desired_abstract_prototype_t.id)
                .expect("vassertSome: abstract func not in interface blueprint") as i32;
        let abstract_funcs = monouts.interface_to_abstract_func_to_virtual_index.get_mut(interface_id).expect("vassertSome interface_to_abstract_func_to_virtual_index");
        assert!(!abstract_funcs.contains_key(&desired_abstract_prototype));
        abstract_funcs.insert(desired_abstract_prototype, (virtual_index, index_in_edge));

        let impls = monouts.interface_to_impls.get(interface_id).expect("vassertSome interface_to_impls").clone();
        for (impl_t, impl_) in impls.iter() {
            self.translate_override(monouts, impl_t, impl_, desired_abstract_prototype_t, &desired_abstract_prototype, supplied_bound_args_ref);
        }
    }


    pub fn assemble_placeholder_map(&self, id_t: &IdT<'s, 't>, id: &IdI<'s, 'i>) -> IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>> {
        let mut result: IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>> = match id_t.init_non_package_id(self.typing_interner) {
            None => IndexMap::default(),
            Some(init_non_package_id_t) => {
                self.assemble_placeholder_map(&init_non_package_id_t, &id.init_non_package_id().unwrap())
            }
        };
        match IInstantiationNameT::try_from(id_t.local_name) {
            Ok(_local_name_t) => {
                let instantiation_id = match IInstantiationNameI::try_from(id.local_name) {
                    Ok(_) => id,
                    Err(_) => panic!("vwat"), // e.g. idT is an instantiation like Vec<int> and idS is a template Vec
                };
                let inner = self.assemble_placeholder_map_inner(id_t, instantiation_id);
                result.extend(inner);
            }
            Err(_) => {}
        }
        result
    }


    pub fn assemble_placeholder_map_inner(&self, id_t: &IdT<'s, 't>, id: &IdI<'s, 'i>) -> IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>> {
        IInstantiationNameT::try_from(id_t.local_name).unwrap().template_args()
            .iter()
            .zip(IInstantiationNameI::try_from(id.local_name).unwrap().template_args(self.interner).iter())
            .flat_map(|(template_arg_t, template_arg_i)| -> Vec<(IdT<'s, 't>, ITemplataI<'s, 'i>)> {
                match (template_arg_t, template_arg_i) {
                    (ITemplataT::Kind(kt), kind_templata_i) => {
                        match kt.kind {
                            KindT::KindPlaceholder(kp) => vec![(kp.id, *kind_templata_i)],
                            _ => panic!("assemble_placeholder_map_inner: KindTemplataT non-placeholder arm"),
                        }
                    }
                    (ITemplataT::Placeholder(pt), templata_i) => vec![(pt.id, *templata_i)],
                    _ => panic!("assemble_placeholder_map_inner: unimplemented arm"),
                }
            })
            .collect()
    }


    pub fn translate_struct_member(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, perspective_region_t: &RegionT, member: &StructMemberT<'s, 't>) -> (KindIT<'s, 'i>, StructMemberI<'s, 'i>) {
        let StructMemberT { name, tyype } = member;
        let kind = self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, tyype);
        let name = self.translate_var_name(name);
        (kind, StructMemberI { name, tyype: kind })
    }

    pub fn translate_mutability(_m: &SharednessT) -> SharednessI {
        match _m {
            SharednessT::Single => SharednessI::Single,
            SharednessT::Shared => SharednessI::Shared,
        }
    }


    pub fn translate_prototype(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, perspective_region_t: &RegionT, desired_prototype_t: &PrototypeT<'s, 't>) -> PrototypeI<'s, 'i> {
        let PrototypeT { id: desired_prototype_id_unsubstituted, return_type: desired_prototype_return_type_unsubstituted } = desired_prototype_t;

        let rune_to_bound_args_for_call =
            self.translate_bound_args_for_callee(
                monouts,
                denizen_name,
                denizen_bound_to_denizen_caller_supplied_thing,
                substitutions,
                perspective_region_t,
                self.hinputs.get_instantiation_bound_args(desired_prototype_t.id));

        let return_it =
            self.translate_kind(
                monouts,
                denizen_name,
                denizen_bound_to_denizen_caller_supplied_thing,
                substitutions,
                perspective_region_t,
                desired_prototype_return_type_unsubstituted);

        let desired_prototype =
            PrototypeI {
                id: self.translate_function_id(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, desired_prototype_id_unsubstituted),
                return_type: return_it,
            };

        match desired_prototype_t.id {
            IdT { local_name: INameT::FunctionBound(_), .. } => {
                let func_bound_name = desired_prototype_t.id;
                let prototype = *denizen_bound_to_denizen_caller_supplied_thing.func_id_to_bound_arg_prototype.get(&func_bound_name).expect("vassertSome: func_id_to_bound_arg_prototype");
                *prototype
            }
            IdT { local_name: INameT::ExternFunction(_), .. } => {


                desired_prototype
            }
            IdT { local_name: last, .. } => {
                match last {
                    INameT::LambdaCallFunction(_) => {
                        // Lambdas Can Call Sibling Lambdas (LCCSL)
                        // If we want to call a lambda, there are three possibilities I've seen:
                        // - We're in the root denizen and we want to call our own lambda.
                        // - We're in a lambda and we want to call an even deeper lambda.
                        // - (This is the weird one) we want to call a *sibling* lambda.
                        // In all cases, make sure the denizen roots of everyone agree.
                        let denizen_root_super_template = Compiler::get_root_super_template(self.typing_interner, *denizen_name);
                        let desired_prototype_root_super_template = Compiler::get_root_super_template(self.typing_interner, desired_prototype_t.id);
                        assert!(denizen_root_super_template == desired_prototype_root_super_template);
                    }
                    _ => {}
                }

                // If we're instantiating something whose name starts with our name, then we're instantiating our lambda.
                let maybe_denizen_bound_to_denizen_caller_supplied_thing =
                    if Compiler::get_super_template(self.typing_interner, desired_prototype_t.id).steps()
                        .starts_with(&Compiler::get_super_template(self.typing_interner, *denizen_name).steps()) {
                        // We need to supply our bounds to our lambdas, see LCCPGB and LCNBAFA.
                        Some(denizen_bound_to_denizen_caller_supplied_thing.clone())
                    } else {
                        if self.opts.sanity_check {
                            let desired_func_super_template_name = Compiler::get_super_template(self.typing_interner, desired_prototype_t.id);
                            let func_t =
                                vassert_one(self.hinputs.functions.iter().filter(|func_t| {
                                    Compiler::get_super_template(self.typing_interner, func_t.header.id) == desired_func_super_template_name
                                }));
                            assert!(rune_to_bound_args_for_call.rune_to_function_bound_arg.len() == func_t.instantiation_bound_params.rune_to_bound_prototype.len());
                            assert!(
                                rune_to_bound_args_for_call.caller_rune_to_callee_rune_to_reachable_func.iter().filter(|(_, v)| !v.is_empty()).count() ==
                                    func_t.instantiation_bound_params.rune_to_citizen_rune_to_reachable_prototype.iter().filter(|(_, v)| !v.citizen_rune_to_reachable_prototype.is_empty()).count());
                            assert!(rune_to_bound_args_for_call.rune_to_impl_bound_arg.len() == func_t.instantiation_bound_params.rune_to_bound_impl.len());
                        }
                        None
                    };
                monouts.new_functions.push((
                    *desired_prototype_t,
                    desired_prototype,
                    rune_to_bound_args_for_call,
                    maybe_denizen_bound_to_denizen_caller_supplied_thing,
                ));
                desired_prototype
            }
        }
    }


    pub fn translate_bound_args_for_callee(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, instantiation_bound_args_for_call_unsubstituted: &InstantiationBoundArgumentsT<'s, 't>) -> InstantiationBoundArgumentsI<'s, 'i> {
        let rune_to_supplied_bound_prototype_for_call_unsubstituted =
            &instantiation_bound_args_for_call_unsubstituted.rune_to_bound_prototype;
        // For any that are placeholders themselves, let's translate those into actual prototypes.
        let rune_to_supplied_prototype_for_call: ArenaIndexMap<'i, IRuneS<'s>, &'i PrototypeI<'s, 'i>> =
            ArenaIndexMap::from_iter_in(
                rune_to_supplied_bound_prototype_for_call_unsubstituted.iter().map(|(rune, supplied_prototype_unsubstituted)| {
                    let prototype: &'i PrototypeI<'s, 'i> = match supplied_prototype_unsubstituted.id {
                        IdT { local_name: INameT::FunctionBound(_), .. } => {
                            let func_bound_name = supplied_prototype_unsubstituted.id;
                            *_denizen_bound_to_denizen_caller_supplied_thing.func_id_to_bound_arg_prototype.get(&func_bound_name).expect("vassertSome: func_id_to_bound_arg_prototype")
                        }
                        _ => {
                            let prototype =
                                self.translate_prototype(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, supplied_prototype_unsubstituted);
                            self.interner.alloc(prototype)
                        }
                    };
                    (*rune, prototype)
                }),
                self.interner.bump());
        // And now we have a map from the callee's rune to the *instantiated* callee's prototypes.

        let caller_rune_to_callee_rune_to_supplied_reachable_prototype_for_call_unsubstituted =
            &instantiation_bound_args_for_call_unsubstituted.rune_to_citizen_rune_to_reachable_prototype;
        // For any that are placeholders themselves, let's translate those into actual prototypes.
        let rune_to_supplied_reachable_prototype_for_call: ArenaIndexMap<'i, IRuneS<'s>, ArenaIndexMap<'i, IRuneS<'s>, &'i PrototypeI<'s, 'i>>> =
            ArenaIndexMap::from_iter_in(
                caller_rune_to_callee_rune_to_supplied_reachable_prototype_for_call_unsubstituted.iter().map(|(caller_rune, callee_rune_to_supplied_reachable_prototype_for_call_unsubstituted)| {
                    let inner: ArenaIndexMap<'i, IRuneS<'s>, &'i PrototypeI<'s, 'i>> =
                        ArenaIndexMap::from_iter_in(
                            callee_rune_to_supplied_reachable_prototype_for_call_unsubstituted.citizen_rune_to_reachable_prototype.iter().map(|(callee_rune, supplied_reachable_prototype_for_call_unsubstituted)| {
                                let prototype_i: &'i PrototypeI<'s, 'i> = match supplied_reachable_prototype_for_call_unsubstituted.id {
                                    IdT { local_name: INameT::FunctionBound(_), .. } => {
                                        let func_bound_name = supplied_reachable_prototype_for_call_unsubstituted.id;
                                        *_denizen_bound_to_denizen_caller_supplied_thing.func_id_to_bound_arg_prototype.get(&func_bound_name).expect("vassertSome: func_id_to_bound_arg_prototype")
                                    }
                                    _ => {
                                        let prototype =
                                            self.translate_prototype(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, supplied_reachable_prototype_for_call_unsubstituted);
                                        self.interner.alloc(prototype)
                                    }
                                };
                                (*callee_rune, prototype_i)
                            }),
                            self.interner.bump());
                    (*caller_rune, inner)
                }),
                self.interner.bump());
        // And now we have a map from the callee's rune to the *instantiated* callee's prototypes.

        let rune_to_supplied_impl_for_call_unsubstituted =
            &instantiation_bound_args_for_call_unsubstituted.rune_to_bound_impl;
        // For any that are placeholders themselves, let's translate those into actual prototypes.
        let rune_to_supplied_impl_for_call: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i>> =
            ArenaIndexMap::from_iter_in(
                rune_to_supplied_impl_for_call_unsubstituted.iter().map(|(rune, supplied_impl_unsubstituted)| {
                    let impl_id = match supplied_impl_unsubstituted.local_name {
                        INameT::ImplBound(_) => {
                            *_denizen_bound_to_denizen_caller_supplied_thing.bound_param_impl_id_to_bound_arg_impl_id.get(supplied_impl_unsubstituted).expect("vassertSome bound_param_impl_id_to_bound_arg_impl_id")
                        }
                        _ => {
                            self.translate_impl_id(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, supplied_impl_unsubstituted)
                        }
                    };
                    (*rune, impl_id)
                }),
                self.interner.bump());
        // And now we have a map from the callee's rune to the *instantiated* callee's impls.

        InstantiationBoundArgumentsI {
            rune_to_function_bound_arg: rune_to_supplied_prototype_for_call,
            caller_rune_to_callee_rune_to_reachable_func: rune_to_supplied_reachable_prototype_for_call,
            rune_to_impl_bound_arg: rune_to_supplied_impl_for_call,
        }
    }


    pub fn translate_struct_definition(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _new_id_t: &IdT<'s, 't>, _new_id: &IdI<'s, 'i>, _struct_def_t: &StructDefinitionT<'s, 't>) {
        let StructDefinitionT { template_name: _, instantiated_citizen: _, attributes, weakable, sharedness, members, is_closure, instantiation_bound_params: _ } = _struct_def_t;
        let perspective_region_t = RegionT::Default;
        let sharedness_i = Self::translate_mutability(sharedness);
        if _monouts.struct_to_sharedness.contains_key(_new_id) {
            return;
        }
        _monouts.struct_to_sharedness.insert(*_new_id, sharedness_i);
        let attributes_i: Vec<ICitizenAttributeI<'s>> = attributes.iter().map(|a| Self::translate_citizen_attribute(a)).collect();
        let members_i: Vec<StructMemberI<'s, 'i>> = members.iter().map(|m| {
            let (_, sm) = self.translate_struct_member(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, &perspective_region_t, m);
            sm
        }).collect();
        let result = StructDefinitionI {
            instantiated_citizen: self.interner.alloc(StructIT { id: *_new_id }),
            attributes: self.interner.bump().alloc_slice_fill_iter(attributes_i.into_iter()),
            weakable: *weakable,
            sharedness: sharedness_i,
            members: self.interner.bump().alloc_slice_fill_iter(members_i.into_iter()),
            is_closure: *is_closure,
            rune_to_function_bound: ArenaIndexMap::new_in(self.interner.bump()),
            rune_to_impl_bound: ArenaIndexMap::new_in(self.interner.bump()),
        };
        assert_eq!(result.instantiated_citizen.id, *_new_id);
        let result_ref: &'i StructDefinitionI<'s, 'i> = self.interner.alloc(result);
        _monouts.structs.insert(result_ref.instantiated_citizen.id, result_ref);
        if result_ref.attributes.iter().any(|a| matches!(a, ICitizenAttributeI::ExternI(_))) {
            _monouts.kind_externs.push(KindExternI { r#struct: result_ref.instantiated_citizen });
        }
    }


    pub fn translate_interface_definition(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _new_id: &IdI<'s, 'i>, _interface_def_t: &InterfaceDefinitionT<'s, 't>) {
        if _monouts.interface_to_sharedness.contains_key(_new_id) {
            return;
        }
        let InterfaceDefinitionT { template_name: _, instantiated_interface: _, ref_: _, attributes, weakable, sharedness, instantiation_bound_params: _, internal_methods: _ } = _interface_def_t;
        assert!(!_monouts.interface_to_impl_to_abstract_prototype_to_override.contains_key(_new_id));
        _monouts.interface_to_impl_to_abstract_prototype_to_override.insert(*_new_id, IndexMap::default());
        assert!(!_monouts.interface_to_abstract_func_to_virtual_index.contains_key(_new_id));
        _monouts.interface_to_abstract_func_to_virtual_index.insert(*_new_id, IndexMap::default());
        assert!(!_monouts.interface_to_impls.contains_key(_new_id));
        _monouts.interface_to_impls.insert(*_new_id, Vec::new());
        let sharedness_i = Self::translate_mutability(sharedness);
        assert!(!_monouts.interface_to_sharedness.contains_key(_new_id));
        _monouts.interface_to_sharedness.insert(*_new_id, sharedness_i);
        let new_interface_it = self.interner.alloc(InterfaceIT { id: *_new_id });
        let attributes_i: Vec<ICitizenAttributeI<'s>> = attributes.iter().map(|a| Self::translate_citizen_attribute(a)).collect();
        let result = InterfaceDefinitionI {
            instantiated_interface: new_interface_it,
            attributes: self.interner.bump().alloc_slice_fill_iter(attributes_i.into_iter()),
            weakable: *weakable,
            sharedness: sharedness_i,
            rune_to_function_bound: ArenaIndexMap::new_in(self.interner.bump()),
            rune_to_impl_bound: ArenaIndexMap::new_in(self.interner.bump()),
            internal_methods: &[],
                    };
        let result_ref: &'i InterfaceDefinitionI<'s, 'i> = self.interner.alloc(result);
        _monouts.interfaces_without_methods.insert(result_ref.instantiated_interface.id, result_ref);
        assert_eq!(result_ref.instantiated_interface.id, *_new_id);
    }


    pub fn translate_function_header(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, perspective_region_t: &RegionT, header_t: &FunctionHeaderT<'s, 't>) -> FunctionHeaderI<'s, 'i> {
        let new_id =
            self.translate_function_id(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &header_t.id);

        let return_it =
            self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &header_t.return_type);
        let return_ic = return_it;

        let result =
            FunctionHeaderI {
                id: new_id,
                attributes: self.interner.alloc_slice_from_vec(header_t.attributes.iter().map(|a| Self::translate_function_attribute(a)).collect()),
                params: self.interner.alloc_slice_from_vec(header_t.params.iter().map(|p| self.translate_parameter(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, p)).collect()),
                return_type: return_ic,
            };

        result
    }


    pub fn translate_function_attribute(x: &IFunctionAttributeT<'s>) -> IFunctionAttributeI<'s> {
        match x {
            IFunctionAttributeT::UserFunction => IFunctionAttributeI::UserFunctionI,
            IFunctionAttributeT::Pure => IFunctionAttributeI::PureI,
            IFunctionAttributeT::Extern(e) => IFunctionAttributeI::ExternI(ExternI { package_coord: e.package_coord }),
            _ => {
                panic!("Unimplemented: translate_function_attribute other");
                // case other => vimpl(other)
            }
        }
    }


    pub fn translate_citizen_attribute(x: &ICitizenAttributeT<'s>) -> ICitizenAttributeI<'s> {
        match x {
            ICitizenAttributeT::Sealed => ICitizenAttributeI::SealedI,
            ICitizenAttributeT::Extern(extern_t) => ICitizenAttributeI::ExternI(ExternI { package_coord: extern_t.package_coord }),
        }
    }


    pub fn translate_function_definition(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, desired_prototype: &PrototypeI<'s, 'i>, function_t: &FunctionDefinitionT<'s, 't>) -> &'i FunctionDefinitionI<'s, 'i> {

        let perspective_region_t = RegionT::Default;
          // functionT.header.id.localName.templateArgs.last match {
          //   case PlaceholderTemplataT(IdT(packageCoord, initSteps, r @ RegionPlaceholderNameT(_, _, _, _)), RegionTemplataType()) => {
          //     IdT(packageCoord, initSteps, r)
          //   }
          //   case _ => vwat()
          // }

        let function_id =
            self.translate_function_id(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, &perspective_region_t, &function_t.header.id);

        match monouts.functions.get(&function_id) {
            Some(func) => return *func,
            None => {}
        }

        let new_header = self.translate_function_header(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, &perspective_region_t, function_t.header);

        if new_header.to_prototype() != *desired_prototype {
            self.translate_function_header(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, &perspective_region_t, function_t.header);
            panic!("vfail");
        }

        let (_body_it, body_ce) =
            self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, &perspective_region_t, &function_t.body);

        let result: &'i FunctionDefinitionI<'s, 'i> =
            self.interner.alloc(FunctionDefinitionI {
                header: new_header,
                rune_to_func_bound: ArenaIndexMap::new_in(self.interner.bump()),
                rune_to_impl_bound: ArenaIndexMap::new_in(self.interner.bump()),
                body: body_ce,
            });

        monouts.functions.insert(result.header.id, result);
        result
    }


    pub fn translate_local_variable(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, perspective_region_t: &RegionT, variable: &LocalVariable<'s, 't>) -> (KindIT<'s, 'i>, &'i LocalVariableI<'s, 'i>) {
        let LocalVariable { name: id, tyype } = variable;
        let kind = self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, tyype);
        let var_name = self.translate_var_name(id);
        let local = self.interner.alloc(LocalVariableI { name: var_name, tyype: kind });
        (kind, local)
    }


    pub fn translate_addr_expr(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, perspective_region_t: &RegionT, expr: &ExpressionTE<'s, 't>) -> (KindIT<'s, 'i>, ExpressionIE<'s, 'i>) {
        // A lookup yields a borrow of its target's storage.
        let result_kind = self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &expr.result());
        let result_borrow = match result_kind {
            KindIT::BorrowRefIT(r) => r,
            _ => panic!("translate_addr_expr: lookup result is not a borrow"),
        };
        let result_ce = match expr {
            ExpressionTE::LocalLookup(ll) => {
                let LocalLookupTE { range, local_variable, .. } = **ll;
                let (_local_it, local_variable_i) = self.translate_local_variable(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, local_variable);
                ExpressionIE::LocalLookup(self.interner.bump().alloc(LocalLookupIE {
                    range,
                    local_variable: local_variable_i,
                    result: result_borrow,
                }))
            }
            ExpressionTE::MemberLookup(rml) => {
                let MemberLookupTE { range, struct_expr: struct_expr_t, member_name: member_name_t, .. } = **rml;
                let (struct_it, struct_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &struct_expr_t);
                let struct_borrow = match struct_it {
                    KindIT::BorrowRefIT(borrow) => borrow,
                    other => panic!("MemberLookup struct_expr must produce a borrow, got {:?}", other),
                };
                let member_name = self.translate_var_name(&member_name_t);
                // Resolve the member's index by name from the struct definition (typing owns member
                // order, which instantiation preserves) so downstream codegen never re-derives it.
                let struct_id_t = match peel_all_references(struct_expr_t.result()) {
                    KindT::Struct(s) => s.id,
                    other => panic!("MemberLookup struct_expr type must be a struct, got {:?}", other),
                };
                let member_index =
                    self.find_struct(&struct_id_t).members.iter()
                        .position(|m| m.name == member_name_t)
                        .expect("MemberLookup: member name not found in struct") as i32;
                // The member's (instantiated) type is the storage type the result borrow wraps.
                let member_type = result_borrow.inner;
                ExpressionIE::MemberLookup(self.interner.bump().alloc(MemberLookupIE {
                    range,
                    struct_expr: struct_ce,
                    struct_type: struct_borrow,
                    member_index,
                    member_name,
                    member_type,
                    result: result_borrow,
                }))
            }
            ExpressionTE::StaticSizedArrayLookup(s) => {
                let StaticSizedArrayLookupTE { range, array_expr: array_expr_t, index_expr: index_expr_t, .. } = **s;
                let (array_it, array_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &array_expr_t);
                let (index_it, index_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &index_expr_t);
                let array_borrow = match array_it {
                    KindIT::BorrowRefIT(borrow) => borrow,
                    other => panic!("StaticSizedArrayLookup array_expr must produce a borrow, got {:?}", other),
                };
                ExpressionIE::StaticSizedArrayLookup(self.interner.alloc(StaticSizedArrayLookupIE {
                    range,
                    array_expr: array_ce,
                    array_type: array_borrow,
                    index_expr: index_ce,
                    index_type: index_it,
                    result: result_borrow,
                }))
            }
            ExpressionTE::RuntimeSizedArrayLookup(rslt) => {
                let RuntimeSizedArrayLookupTE { range, array_expr, index_expr, .. } = **rslt;
                let (array_it, array_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &array_expr);
                let (index_it, index_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &index_expr);
                let array_borrow = match array_it {
                    KindIT::BorrowRefIT(borrow) => borrow,
                    other => panic!("RuntimeSizedArrayLookup array_expr must produce a borrow, got {:?}", other),
                };
                ExpressionIE::RuntimeSizedArrayLookup(self.interner.alloc(RuntimeSizedArrayLookupIE {
                    range,
                    array_expr: array_ce,
                    array_type: array_borrow,
                    index_expr: index_ce,
                    index_type: index_it,
                    result: result_borrow,
                }))
            }
            _ => panic!("translate_addr_expr: not an address (lookup) expression"),
        };
        (result_kind, result_ce)
    }


    pub fn translate_expr(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, perspective_region_t: &RegionT, expr: &ExpressionTE<'s, 't>) -> (KindIT<'s, 'i>, ExpressionIE<'s, 'i>) {
        self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, expr)
    }


    pub fn translate_ref_expr(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, perspective_region_t: &RegionT, expr: &ExpressionTE<'s, 't>) -> (KindIT<'s, 'i>, ExpressionIE<'s, 'i>) {
        let _denizen_template_name = Compiler::get_template(self.typing_interner, *denizen_name);
        // The result of any expression is just its onion result kind, monomorphized. This replaces
        // the old ownership-composition: ownership is now which wrap surrounds the kind.
        let result_it = self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &expr.result());
        let result_ce = match expr {
            ExpressionTE::LetAndLend(lal) => {
                let LetAndLendTE { variable, expr: source_expr_t, .. } = **lal;
                let (_source_it, source_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &source_expr_t);
                let (_local_it, local_i) =
                    self.translate_local_variable(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &variable);
                ExpressionIE::LetAndLend(self.interner.bump().alloc(LetAndLendIE {
                    variable: local_i,
                    expr: source_ce,
                    result: result_it,
                }))
            }
            ExpressionTE::LockWeak(lw) => {
                let LockWeakTE { inner_expr, some_constructor, none_constructor, some_impl_name, none_impl_name, .. } = **lw;
                let (inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &inner_expr);
                let some_proto =
                    self.translate_prototype(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, some_constructor);
                let none_proto =
                    self.translate_prototype(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, none_constructor);
                let some_impl_id = self.translate_impl_id(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &some_impl_name);
                let none_impl_id = self.translate_impl_id(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &none_impl_name);
                ExpressionIE::LockWeak(self.interner.bump().alloc(LockWeakIE {
                    inner_expr: inner_ce,
                    source_type: inner_it,
                    some_constructor: some_proto,
                    none_constructor: none_proto,
                    some_impl_name: some_impl_id,
                    none_impl_name: none_impl_id,
                    result: result_it,
                }))
            }
            ExpressionTE::BorrowToWeak(b) => {
                let BorrowToWeakTE { inner_expr, .. } = **b;
                let (inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &inner_expr);
                ExpressionIE::BorrowToWeak(self.interner.bump().alloc(BorrowToWeakIE { inner_expr: inner_ce, source_type: inner_it, result: result_it }))
            }
            ExpressionTE::LetNormal(l) => {
                let (_inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &l.expr);
                let (_local_it, local_i) =
                    self.translate_local_variable(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &l.variable);
                ExpressionIE::LetNormal(self.interner.alloc(LetNormalIE {
                    variable: local_i,
                    expr: inner_ce,
                    result: result_it,
                }))
            }
            ExpressionTE::Unlet(u) => {
                let (_local_it, local_i) =
                    self.translate_local_variable(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &u.variable);
                ExpressionIE::Unlet(self.interner.alloc(UnletIE {
                    variable: local_i,
                    result: result_it,
                }))
            }
            ExpressionTE::Discard(d) => {
                let (inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &d.expr);
                ExpressionIE::Discard(self.interner.alloc(DiscardIE { expr: inner_ce, source_type: inner_it }))
            }
            ExpressionTE::If(if_te) => {
                let (_condition_it, condition_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &if_te.condition);
                let (then_it, then_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &if_te.then_call);
                let (else_it, else_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &if_te.else_call);
                ExpressionIE::If(self.interner.alloc(IfIE {
                    condition: condition_ce,
                    then_call: then_ce,
                    else_call: else_ce,
                    then_result_type: then_it,
                    else_result_type: else_it,
                    result: result_it,
                }))
            }
            ExpressionTE::While(w) => {
                let (inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &w.block.inner);
                ExpressionIE::While(self.interner.alloc(WhileIE {
                    block: BlockIE { inner: inner_ce, inner_type: inner_it, result: inner_it },
                    result: result_it,
                }))
            }
            ExpressionTE::Mutate(m) => {
                let MutateTE { destination_expr: destination_tt, source_expr, .. } = **m;
                let (destination_it, destination_ce) = self.translate_addr_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &destination_tt);
                let (source_it, source_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &source_expr);
                let destination_borrow = match destination_it {
                    KindIT::BorrowRefIT(borrow) => borrow,
                    other => panic!("Mutate destination_expr must produce a borrow, got {:?}", other),
                };
                ExpressionIE::Mutate(self.interner.bump().alloc(MutateIE {
                    destination_expr: destination_ce,
                    destination_type: destination_borrow,
                    source_expr: source_ce,
                    source_type: source_it,
                    result: result_it,
                }))
            }
            ExpressionTE::Restackify(r) => {
                let (_inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &r.source_expr);
                let (_local_it, local_i) =
                    self.translate_local_variable(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &r.variable);
                ExpressionIE::Restackify(self.interner.alloc(RestackifyIE {
                    variable: local_i,
                    source_expr: inner_ce,
                    result: result_it,
                }))
            }
            ExpressionTE::Return(r) => {
                let (inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &r.source_expr);
                ExpressionIE::Return(self.interner.alloc(ReturnIE {
                    source_expr: inner_ce,
                    source_type: inner_it,
                }))
            }
            ExpressionTE::Break(_) => {
                ExpressionIE::Break(self.interner.alloc(BreakIE))
            }
            ExpressionTE::Block(b) => {
                let (inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &b.inner);
                ExpressionIE::Block(self.interner.alloc(BlockIE {
                    inner: inner_ce,
                    inner_type: inner_it,
                    result: result_it,
                }))
            }
            ExpressionTE::Consecutor(c) => {
                let inners_ce: Vec<_> =
                    c.exprs.iter().map(|inner_te| {
                        self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, inner_te).1
                    }).collect();
                ExpressionIE::Consecutor(self.interner.alloc(ConsecutorIE {
                    exprs: self.interner.alloc_slice_from_vec(inners_ce),
                    result: result_it,
                }))
            }
            ExpressionTE::StaticArrayFromValues(s) => {
                let StaticArrayFromValuesTE { elements, array_type, .. } = **s;
                let elements_ce: Vec<_> = elements.iter().map(|element_te| {
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, element_te).1
                }).collect();
                let ssa_tt = self.translate_static_sized_array(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &array_type);
                ExpressionIE::StaticArrayFromValues(self.interner.alloc(StaticArrayFromValuesIE {
                    elements: self.interner.alloc_slice_from_vec(elements_ce),
                    result: result_it,
                    array_type: self.interner.alloc(ssa_tt),
                }))
            }
            ExpressionTE::ArraySize(_) => {
                panic!("Unimplemented: translate_ref_expr ArraySize");
            }
            ExpressionTE::IsSameInstance(isi) => {
                let IsSameInstanceTE { left, right, .. } = **isi;
                let (left_it, left_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &left);
                let (right_it, right_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &right);
                ExpressionIE::IsSameInstance(self.interner.alloc(IsSameInstanceIE {
                    left: left_ce,
                    left_type: left_it,
                    right: right_ce,
                    right_type: right_it,
                }))
            }
            ExpressionTE::AsSubtype(asx) => {
                let AsSubtypeTE { source_expr, target_type: target_subtype, ok_constructor, err_constructor, impl_name: impl_id_t, ok_impl_name: ok_result_impl_id_t, err_impl_name: err_result_impl_id_t, .. } = **asx;
                let (source_it, source_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &source_expr);
                let target_coord = self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &target_subtype);
                let ok = self.translate_prototype(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, ok_constructor);
                let err = self.translate_prototype(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, err_constructor);
                let impl_id = self.translate_impl_id(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &impl_id_t);
                let ok_impl_id = self.translate_impl_id(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &ok_result_impl_id_t);
                let err_impl_id = self.translate_impl_id(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &err_result_impl_id_t);
                ExpressionIE::AsSubtype(self.interner.bump().alloc(AsSubtypeIE {
                    source_expr: source_ce,
                    source_type: source_it,
                    target_type: target_coord,
                    ok_constructor: self.interner.bump().alloc(ok),
                    err_constructor: self.interner.bump().alloc(err),
                    impl_name: impl_id,
                    ok_impl_name: ok_impl_id,
                    err_impl_name: err_impl_id,
                    result: result_it,
                }))
            }
            ExpressionTE::VoidLiteral(_) => {
                ExpressionIE::VoidLiteral(self.interner.alloc(VoidLiteralIE))
            }
            ExpressionTE::ConstantInt(c) => {
                ExpressionIE::ConstantInt(self.interner.alloc(ConstantIntIE {
                    value: expect_integer_templata(self.translate_templata(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &c.value)).value,
                    bits: c.bits,
                }))
            }
            ExpressionTE::ConstantBool(c) => {
                ExpressionIE::ConstantBool(self.interner.alloc(ConstantBoolIE { value: c.value }))
            }
            ExpressionTE::ConstantStr(c) => {
                ExpressionIE::ConstantStr(self.interner.alloc(ConstantStrIE { _marker: PhantomData, value: c.value.0, result: result_it }))
            }
            ExpressionTE::ConstantFloat(c) => {
                ExpressionIE::ConstantFloat(self.interner.alloc(ConstantFloatIE { value: c.value }))
            }
            ExpressionTE::ArgLookup(al) => {
                let ArgLookupTE { param_index, .. } = **al;
                ExpressionIE::ArgLookup(self.interner.alloc(ArgLookupIE { param_index, tyype: result_it }))
            }
            ExpressionTE::ArrayLength(al) => {
                let ArrayLengthTE { array_expr, .. } = **al;
                let (array_it, array_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &array_expr);
                let array_borrow = match array_it {
                    KindIT::BorrowRefIT(borrow) => borrow,
                    other => panic!("ArrayLength array_expr must produce a borrow, got {:?}", other),
                };
                ExpressionIE::ArrayLength(self.interner.alloc(ArrayLengthIE {
                    array_expr: array_ce,
                    array_type: array_borrow,
                }))
            }
            ExpressionTE::InterfaceFunctionCall(ifc) => {
                let InterfaceFunctionCallTE { super_function_prototype: super_function_prototype_t, virtual_param_index, args, .. } = **ifc;
                let super_function_prototype =
                    self.translate_prototype(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, super_function_prototype_t);
                let args_ce: Vec<ExpressionIE<'s, 'i>> = args.iter().map(|arg| {
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, arg).1
                }).collect();
                // Typing owns the vtable order: the slot is this method's position in its
                // interface's blueprint (InterfaceEdgeBlueprintT.super_family_root_headers).
                let typed_interface_id =
                    match peel_all_references(super_function_prototype_t.param_types()[virtual_param_index as usize]) {
                        KindT::Interface(ir) => ir.id,
                        other => panic!("InterfaceFunctionCall virtual param is not an interface: {:?}", other),
                    };
                let blueprint =
                    self.hinputs.interface_to_edge_blueprints.get(&typed_interface_id)
                        .expect("vassertSome: interface_to_edge_blueprints for InterfaceFunctionCall");
                let index_in_edge =
                    blueprint.super_family_root_headers.iter()
                        .position(|(header_proto, _)| header_proto.id == super_function_prototype_t.id)
                        .expect("vassertSome: super_function_prototype not in interface blueprint") as i32;
                let result_ce = ExpressionIE::InterfaceFunctionCall(self.interner.bump().alloc(InterfaceFunctionCallIE {
                    super_function_prototype: self.interner.bump().alloc(super_function_prototype),
                    virtual_param_index,
                    index_in_edge,
                    args: self.interner.alloc_slice_from_vec(args_ce),
                    result: result_it,
                }));
                let interface_id = super_function_prototype.param_types()[virtual_param_index as usize].expect_interface().id;
                let instantiation_bound_args = self.translate_bound_args_for_callee(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, self.hinputs.get_instantiation_bound_args(super_function_prototype_t.id));
                monouts.new_abstract_funcs.push((*super_function_prototype_t, super_function_prototype, virtual_param_index as usize, interface_id, instantiation_bound_args));
                result_ce
            }
            ExpressionTE::ExternFunctionCall(efc) => {
                let ExternFunctionCallTE { prototype2, args, .. } = **efc;
                let prototype = self.translate_prototype(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, prototype2);
                let args_ce: Vec<ExpressionIE<'s, 'i>> = args.iter().map(|arg_te| self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, arg_te).1).collect();
                let result_ce = ExpressionIE::ExternFunctionCall(self.interner.bump().alloc(ExternFunctionCallIE { prototype2: prototype, args: self.interner.bump().alloc_slice_fill_iter(args_ce.into_iter()), result: result_it }));
                match prototype2.id.local_name {
                    INameT::ExternFunction(ExternFunctionNameT { human_name, template_args, .. }) if !template_args.is_empty() => {
                        let num_inherited = self.hinputs.function_externs.iter().find(|fe| {
                            fe.prototype.id.package_coord == prototype2.id.package_coord
                                && fe.prototype.id.init_steps == prototype2.id.init_steps
                                && match fe.prototype.id.local_name {
                                    INameT::ExternFunction(ExternFunctionNameT { human_name: hn, .. }) => hn == human_name,
                                    _ => false,
                                }
                        })
                        .and_then(|fe| fe.generic_parameter_inheritance.as_ref().map(|i| i.num_inherited_generic_parameters))
                        .unwrap_or(0);
                        monouts.function_externs.push(FunctionExternI {
                            prototype: self.interner.alloc(PrototypeI { id: prototype.id, return_type: prototype.return_type }),
                            num_inherited_generic_parameters: num_inherited,
                        });
                    }
                    _ => {}
                }
                result_ce
            }
            ExpressionTE::FunctionCall(fc) => {
                let FunctionCallTE { callable: prototype_t, args, .. } = fc;
                let inners_ce: Vec<ExpressionIE<'s, 'i>> = args.iter().map(|arg_te| {
                    let (_arg_it, arg_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, arg_te);
                    arg_ce
                }).collect();
                let prototype = self.translate_prototype(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, prototype_t);
                ExpressionIE::FunctionCall(self.interner.alloc(FunctionCallIE {
                    callable: prototype,
                    args: self.interner.bump().alloc_slice_fill_iter(inners_ce.into_iter()),
                    result: result_it,
                }))
            }
            ExpressionTE::Reinterpret(r) => {
                // A Reinterpret is a type-identity node from typing (e.g. `@x` viewed as `&x`)
                // that only exists to bridge kinds pre-monomorphization. Once substitution is
                // done its source and result kinds coincide, so assert that and emit the inner
                // expression directly. Reinterpret never reaches the I-IR or the backend.
                // VCOORD: need arcana
                let (inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &r.expr);
                assert_eq!(inner_it, result_it, "Reinterpret source kind != result kind after substitution");
                inner_ce
            }
            ExpressionTE::CopyPrim(cp) => {
                let (inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &cp.inner);
                ExpressionIE::CopyPrim(self.interner.alloc(CopyPrimIE {
                    inner: inner_ce,
                    source_type: inner_it,
                    result: result_it,
                }))
            }
            ExpressionTE::Construct(c) => {
                let ConstructTE { struct_tt, args, .. } = **c;
                let args_ce: Vec<ExpressionIE<'s, 'i>> = args.iter().map(|arg_te| {
                    self.translate_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, arg_te).1
                }).collect();
                let bound_args = self.translate_bound_args_for_callee(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &self.hinputs.get_instantiation_bound_args(struct_tt.id));
                let struct_it = self.translate_struct(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, struct_tt, &bound_args);
                ExpressionIE::Construct(self.interner.bump().alloc(ConstructIE {
                    struct_tt: struct_it,
                    result: result_it,
                    args: self.interner.bump().alloc_slice_fill_iter(args_ce.into_iter()),
                }))
            }
            ExpressionTE::NewRuntimeSizedArray(nmrsa) => {
                let NewRuntimeSizedArrayTE { array_type: array_tt, region: _, capacity_expr, .. } = **nmrsa;
                let array_it = self.translate_runtime_sized_array(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, array_tt);
                let (_capacity_it, capacity_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &capacity_expr);
                ExpressionIE::NewRuntimeSizedArray(self.interner.alloc(NewRuntimeSizedArrayIE {
                    array_type: array_it,
                    capacity_expr: capacity_ce,
                    result: result_it,
                }))
            }
            ExpressionTE::StaticArrayFromCallable(s) => {
                let StaticArrayFromCallableTE { array_type, region: _, generator, generator_method, .. } = **s;
                let ssa_it = self.translate_static_sized_array(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, array_type);
                let (_generator_it, generator_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &generator);
                let generator_prototype = self.translate_prototype(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, generator_method);
                ExpressionIE::StaticArrayFromCallable(self.interner.alloc(StaticArrayFromCallableIE {
                    array_type: ssa_it,
                    generator: generator_ce,
                    generator_method: generator_prototype,
                    result: result_it,
                }))
            }
            ExpressionTE::DestroyStaticSizedArrayIntoFunction(d) => {
                let DestroyStaticSizedArrayIntoFunctionTE { array_expr: array_expr_t, array_type: array_type_t, consumer: consumer_t, consumer_method: consumer_method_t, .. } = **d;
                let (_array_it, array_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &array_expr_t);
                let ssa_it = self.translate_static_sized_array(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, array_type_t);
                let (_consumer_it, consumer_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &consumer_t);
                let consumer_prototype =
                    self.translate_prototype(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, consumer_method_t);
                ExpressionIE::DestroyStaticSizedArrayIntoFunction(self.interner.alloc(DestroyStaticSizedArrayIntoFunctionIE {
                    array_expr: array_ce,
                    array_type: ssa_it,
                    consumer: consumer_ce,
                    consumer_method: consumer_prototype,
                }))
            }
            ExpressionTE::DestroyStaticSizedArrayIntoLocals(d) => {
                let DestroyStaticSizedArrayIntoLocalsTE { expr: expr_t, static_sized_array: _, destination_reference_variables, .. } = **d;
                let (source_it, source_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &expr_t);
                let (ssa_it, size) = match source_it {
                    KindIT::StaticSizedArrayIT(s) => {
                        match s.name.local_name {
                            INameI::StaticSizedArray(n) => (*s, n.size),
                            _ => panic!("DestroyStaticSizedArrayIntoLocals: local_name not StaticSizedArrayNameI"),
                        }
                    }
                    _ => panic!("DestroyStaticSizedArrayIntoLocals: source_it not StaticSizedArrayIT"),
                };
                assert!(size == destination_reference_variables.len() as i64);
                let dest_vars_vec: Vec<&'i LocalVariableI<'s, 'i>> = destination_reference_variables.iter().map(|dest_ref_var_t| {
                    self.translate_local_variable(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, dest_ref_var_t).1
                }).collect();
                ExpressionIE::DestroyStaticSizedArrayIntoLocals(self.interner.alloc(DestroyStaticSizedArrayIntoLocalsIE {
                    expr: source_ce,
                    static_sized_array: ssa_it,
                    destination_reference_variables: self.interner.alloc_slice_from_vec(dest_vars_vec),
                }))
            }
            ExpressionTE::DestroyRuntimeSizedArray(d) => {
                let DestroyRuntimeSizedArrayTE { array_expr, .. } = **d;
                let (array_it, array_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &array_expr);
                ExpressionIE::DestroyRuntimeSizedArray(self.interner.alloc(DestroyRuntimeSizedArrayIE {
                    array_expr: array_ce,
                    array_type: array_it,
                }))
            }
            ExpressionTE::RuntimeSizedArrayCapacity(r) => {
                let RuntimeSizedArrayCapacityTE { array_expr, .. } = **r;
                let (array_it, array_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &array_expr);
                let array_borrow = match array_it {
                    KindIT::BorrowRefIT(borrow) => borrow,
                    other => panic!("RuntimeSizedArrayCapacity array_expr must produce a borrow, got {:?}", other),
                };
                ExpressionIE::RuntimeSizedArrayCapacity(self.interner.alloc(RuntimeSizedArrayCapacityIE {
                    array_expr: array_ce,
                    array_type: array_borrow,
                }))
            }
            ExpressionTE::PushRuntimeSizedArray(prsa) => {
                let PushRuntimeSizedArrayTE { array_expr, new_element_expr, .. } = **prsa;
                let (array_it, array_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &array_expr);
                let (element_it, element_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &new_element_expr);
                let array_borrow = match array_it {
                    KindIT::BorrowRefIT(borrow) => borrow,
                    other => panic!("PushRuntimeSizedArray array_expr must produce a borrow, got {:?}", other),
                };
                ExpressionIE::PushRuntimeSizedArray(self.interner.alloc(PushRuntimeSizedArrayIE {
                    array_expr: array_ce,
                    array_type: array_borrow,
                    new_element_expr: element_ce,
                    element_type: element_it,
                }))
            }
            ExpressionTE::PopRuntimeSizedArray(p) => {
                let PopRuntimeSizedArrayTE { array_expr, .. } = **p;
                let (array_it, array_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &array_expr);
                let array_borrow = match array_it {
                    KindIT::BorrowRefIT(borrow) => borrow,
                    other => panic!("PopRuntimeSizedArray array_expr must produce a borrow, got {:?}", other),
                };
                ExpressionIE::PopRuntimeSizedArray(self.interner.alloc(PopRuntimeSizedArrayIE {
                    array_expr: array_ce,
                    array_type: array_borrow,
                    result: result_it,
                }))
            }
            ExpressionTE::InterfaceToInterfaceUpcast(_) => {
                panic!("Unimplemented: translate_ref_expr InterfaceToInterfaceUpcast");
            }
            ExpressionTE::Upcast(u) => {
                let UpcastTE { inner_expr: inner_expr_unsubstituted, target_super_kind, impl_name: untranslated_impl_id, .. } = *u;
                let impl_id = self.translate_impl_id(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &untranslated_impl_id);
                let (inner_it, inner_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &inner_expr_unsubstituted);
                let super_kind = self.translate_super_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &target_super_kind);
                ExpressionIE::Upcast(self.interner.bump().alloc(UpcastIE {
                    inner_expr: inner_ce,
                    source_type: inner_it,
                    target_interface: super_kind,
                    impl_name: impl_id,
                    result: result_it,
                }))
            }
            ExpressionTE::Destroy(d) => {
                let DestroyTE { expr: expr_t, struct_tt, destination_reference_variables, .. } = **d;
                let (_source_it, source_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &expr_t);
                let bound_args = self.translate_bound_args_for_callee(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &self.hinputs.get_instantiation_bound_args(struct_tt.id));
                let struct_id = self.translate_struct_id(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &struct_tt.id, &bound_args);
                let dest_ref_vars: Vec<&'i LocalVariableI<'s, 'i>> =
                    destination_reference_variables.iter().map(|dest_ref_var_t| {
                        self.translate_local_variable(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, dest_ref_var_t).1
                    }).collect();
                ExpressionIE::Destroy(self.interner.bump().alloc(DestroyIE {
                    expr: source_ce,
                    struct_tt: StructIT { id: struct_id },
                    destination_reference_variables: self.interner.bump().alloc_slice_copy(&dest_ref_vars),
                }))
            }
            ExpressionTE::Deref(d) => {
                let (inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &d.inner);
                ExpressionIE::Deref(self.interner.alloc(DerefIE {
                    range: d.range,
                    inner: inner_ce,
                    source_type: inner_it,
                    result: result_it,
                }))
            }
            ExpressionTE::LocalLookup(_)
            | ExpressionTE::StaticSizedArrayLookup(_)
            | ExpressionTE::RuntimeSizedArrayLookup(_)
            | ExpressionTE::MemberLookup(_) => {
                return self.translate_addr_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, expr);
            }
        };
        (result_it, result_ce)
    }


    pub fn maybe_immutabilify(_inner_ie: &ExpressionIE<'s, 'i>) -> ExpressionIE<'s, 'i> {
        panic!("Unimplemented: maybe_immutabilify");
        // innerIE.result.kind match {
        //   case x if x.isPrimitive => return innerIE
        //   case _ =>
        // }
        // innerIE match {
        //   case SoftLoadIE(expr, MutableBorrowI, result) => return SoftLoadIE(expr, ImmutableBorrowI, result.copy(ownership = ImmutableBorrowI))
        //   case SoftLoadIE(expr, MutableShareI, result) => return SoftLoadIE(expr, ImmutableShareI, result.copy(ownership = ImmutableShareI))
        //   case _ =>
        // }
        // innerIE.result.ownership match {
        //   case OwnI => innerIE
        //   case ImmutableBorrowI | ImmutableShareI => innerIE
        //   case MutableBorrowI => ImmutabilifyIE(innerIE, innerIE.result.copy(ownership = ImmutableBorrowI))
        //   case MutableShareI => ImmutabilifyIE(innerIE, innerIE.result.copy(ownership = ImmutableShareI))
        // }
    }


    pub fn run_in_new_pure_region<T>(_denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _denizen_template_name: &IdT<'s, 't>, _new_default_region_t: &ITemplataT<'s, 't>, _run: impl Fn(&IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, &RegionT) -> T) -> T {
        panic!("Unimplemented: run_in_new_pure_region");
        // val newDefaultRegionNameT = RegionT(DefaultRegionT)
        // val newPerspectiveRegionT = newDefaultRegionNameT
        // val newDefaultRegion = RegionT(DefaultRegionT)
        // run(substitutions, newPerspectiveRegionT)
    }


    pub fn translate_function_id(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, perspective_region_t: &RegionT, full_name_t: &IdT<'s, 't>) -> IdI<'s, 'i> {
        let IdT { package_coord: module, init_steps: steps, local_name: last, .. } = *full_name_t;
        let full_name =
            IdI {
                package_coord: module,
                init_steps: self.interner.alloc_slice_from_vec(
                    steps.iter().map(|step| self.translate_name_substituting(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, step)).collect::<Vec<_>>()),
                local_name: INameI::from(self.translate_function_name(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &IFunctionNameT::try_from(last).unwrap())),
            };
        full_name
    }


    pub fn translate_struct_id(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, _struct_id_t: &IdT<'s, 't>, _instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) -> IdI<'s, 'i> {
        let IdT { package_coord: module, init_steps: steps, local_name: last_t, .. } = _struct_id_t;
        let last_t_struct: IStructNameT<'s, 't> = (*last_t).try_into().unwrap();
        let translated_steps: Vec<INameI<'s, 'i>> = steps.iter().map(|n| self.translate_name_substituting(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, n)).collect();
        let struct_name_si = self.translate_struct_name(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &last_t_struct);
        let full_name = IdI {
            package_coord: module,
            init_steps: self.interner.bump().alloc_slice_fill_iter(translated_steps.into_iter()),
            local_name: struct_name_si.into(),
        };
        self.translate_struct_callsite(_monouts, _struct_id_t, &full_name, _instantiation_bound_args);
        full_name
    }


    pub fn translate_interface_id(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, _interface_id_t: &IdT<'s, 't>, _instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) -> IdI<'s, 'i> {
        let IdT { package_coord: module, init_steps: steps, local_name: last_t, .. } = _interface_id_t;
        let last_t_interface = match last_t {
            INameT::Interface(i) => IInterfaceNameT::Interface(*i),
            _ => panic!("translate_interface_id: local_name not Interface"),
        };
        let translated_steps: Vec<INameI<'s, 'i>> = steps.iter().map(|_n| panic!("translate_interface_id: non-empty init_steps not yet ported")).collect();
        let interface_name_si = self.translate_interface_name(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &last_t_interface);
        let full_name = IdI {
            package_coord: module,
            init_steps: self.interner.bump().alloc_slice_fill_iter(translated_steps.into_iter()),
            local_name: interface_name_si.into(),
        };
        self.translate_interface_callsite(_monouts, _interface_id_t, &full_name, _instantiation_bound_args);
        full_name
    }


    pub fn translate_impl_id(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, _impl_id_t: &IdT<'s, 't>) -> IdI<'s, 'i> {
        let IdT { package_coord: module, init_steps: steps, local_name: last_t, .. } = *_impl_id_t;
        match last_t {
            INameT::ImplBound(_) => {
                *_denizen_bound_to_denizen_caller_supplied_thing.bound_param_impl_id_to_bound_arg_impl_id.get(_impl_id_t).expect("translate_impl_id: missing impl bound")
            }
            _ => {
                let translated_steps: Vec<INameI<'s, 'i>> = steps.iter().map(|s| Self::translate_name(s)).collect();
                let impl_name_i = self.translate_impl_name(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &IImplNameT::try_from(last_t).expect("translate_impl_id: non-impl name"));
                let impl_id = IdI { package_coord: module, init_steps: self.interner.bump().alloc_slice_fill_iter(translated_steps.into_iter()), local_name: INameI::from(impl_name_i) };
                let bound_args_for_call_unsubstituted = self.hinputs.get_instantiation_bound_args(*_impl_id_t);
                let rune_to_bound_args_for_new_impl = self.translate_bound_args_for_callee(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &bound_args_for_call_unsubstituted);
                _monouts.new_impls.push((*_impl_id_t, impl_id, rune_to_bound_args_for_new_impl));
                impl_id
            }
        }
    }


    pub fn translate_citizen_name(&self, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, _t: &ICitizenNameT<'s, 't>) -> ICitizenNameI<'s, 'i> {
        panic!("Unimplemented: translate_citizen_name");
        // t match {
        //   case s : IStructNameT => translateStructName(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, s)
        //   case i : IInterfaceNameT => translateInterfaceName(denizenName, denizenBoundToDenizenCallerSuppliedThing,substitutions, perspectiveRegionT, i)
        // }
    }


    pub fn translate_id_from_substitutions(_substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, _id: &IdT<'s, 't>) -> IdI<'s, 'i> {
        panic!("Unimplemented: translate_id_from_substitutions");
        // id match {
        //   case other => vimpl(other)
        // }
    }


    pub fn translate_citizen_id(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, _citizen_id_t: &IdT<'s, 't>, _instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) -> IdI<'s, 'i> {
        panic!("Unimplemented: translate_citizen_id");
        // id match {
        //   case IdT(module, steps, last : IStructNameT) => translateStructId(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, IdT(module, steps, last), instantiationBoundArgs)
        //   case IdT(module, steps, last : IInterfaceNameT) => translateInterfaceId(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, IdT(module, steps, last), instantiationBoundArgs)
        //   case other => vimpl(other)
        // }
    }



    pub fn translate_citizen(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, _citizen: &ICitizenTT<'s, 't>, _instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) -> ICitizenIT<'s, 'i> {
        match _citizen {
            ICitizenTT::Struct(s) => {
                let s_i = self.translate_struct(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, s, _instantiation_bound_args);
                ICitizenIT::StructIT(self.interner.alloc(StructIT { id: s_i.id }))
            }
            ICitizenTT::Interface(i) => {
                let i_i = self.translate_interface(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, i, _instantiation_bound_args);
                ICitizenIT::InterfaceIT(self.interner.alloc(InterfaceIT { id: i_i.id }))
            }
        }
    }


    pub fn translate_struct(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, _struct: &StructTT<'s, 't>, _instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) -> StructIT<'s, 'i> {
        let StructTT { id: full_name, .. } = _struct;
        let translated_id = self.translate_struct_id(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, full_name, _instantiation_bound_args);
        let desired_struct = StructIT { id: translated_id };
        desired_struct
    }


    pub fn translate_interface(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, _interface: &InterfaceTT<'s, 't>, _instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) -> InterfaceIT<'s, 'i> {
        let InterfaceTT { id: full_name, .. } = _interface;
        let translated_id = self.translate_interface_id(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, full_name, _instantiation_bound_args);
        InterfaceIT { id: translated_id }
    }


    pub fn translate_super_kind(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, _kind: &ISuperKindTT<'s, 't>) -> InterfaceIT<'s, 'i> {
        match _kind {
            ISuperKindTT::Interface(i) => {
                let bound_args = self.translate_bound_args_for_callee(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &self.hinputs.get_instantiation_bound_args(i.id));
                self.translate_interface(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, i, &bound_args)
            }
            ISuperKindTT::KindPlaceholder(_) => panic!("Unimplemented: translate_super_kind KindPlaceholder"),
        }
    }


    pub fn translate_placeholder(&self, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _t: &KindPlaceholderT<'s, 't>) -> KindIT<'s, 'i> {
        panic!("Unimplemented: translate_placeholder");
        // val newSubstitutingTemplata = vassertSome(substitutions.get(t.id))
        // ITemplataI.expectKindTemplata(newSubstitutingTemplata).kind
    }


    pub fn translate_static_sized_array(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, perspective_region_t: &RegionT, ssa_tt: &StaticSizedArrayTT<'s, 't>) -> StaticSizedArrayIT<'s, 'i> {
        let StaticSizedArrayTT { name: id_t, .. } = ssa_tt;
        let IdT { package_coord, init_steps, local_name, .. } = *id_t;
        let ssa_name_t = match local_name {
            INameT::StaticSizedArray(n) => *n,
            _ => panic!("translate_static_sized_array: local_name not StaticSizedArrayNameT"),
        };
        let StaticSizedArrayNameT { template: _, size: size_t, arr } = ssa_name_t;
        let RawArrayNameT { element_type: element_type_t, self_region: _ } = *arr;
        let new_perspective_region_t = RegionT::Default;
        let _ssa_region = RegionT::Default;
        let int_templata = expect_integer_templata(self.translate_templata(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, &new_perspective_region_t, &size_t)).value;
        let element_type = self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, &new_perspective_region_t, &element_type_t);
        let translated_init_steps: Vec<INameI<'s, 'i>> = init_steps.iter().map(|n| Self::translate_name(n)).collect();
        let local_name_i = INameI::StaticSizedArray(self.interner.alloc(StaticSizedArrayNameI {
            template: StaticSizedArrayTemplateNameI,
            size: int_templata,
            arr: RawArrayNameI {
                element_type,
                self_region: RegionT::Default,
            },
        }));
        let id_i = IdI {
            package_coord,
            init_steps: self.interner.alloc_slice_from_vec(translated_init_steps),
            local_name: local_name_i,
        };
        let ssa_it = StaticSizedArrayIT { name: id_i };
        // Collect the distinct array kind so the backend can declare its region.
        monouts.static_sized_arrays.entry(id_i).or_insert_with(|| self.interner.alloc(ssa_it));
        ssa_it
    }


    pub fn translate_runtime_sized_array(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, perspective_region_t: &RegionT, rsa_tt: &RuntimeSizedArrayTT<'s, 't>) -> RuntimeSizedArrayIT<'s, 'i> {
        let RuntimeSizedArrayTT { name: id_t, .. } = rsa_tt;
        let IdT { package_coord, init_steps, local_name, .. } = *id_t;
        let rsa_name_t = match local_name {
            INameT::RuntimeSizedArray(n) => *n,
            _ => panic!("translate_runtime_sized_array: local_name not RuntimeSizedArrayNameT"),
        };
        let RuntimeSizedArrayNameT { template: _, arr } = rsa_name_t;
        let RawArrayNameT { element_type: element_type_t, self_region: _ } = *arr;
        let new_perspective_region_t = RegionT::Default;
        let _rsa_region = RegionT::Default;
        let element_type = self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, &new_perspective_region_t, &element_type_t);
        let translated_init_steps: Vec<INameI<'s, 'i>> = init_steps.iter().map(|n| Self::translate_name(n)).collect();
        let local_name_i = INameI::RuntimeSizedArray(self.interner.alloc(RuntimeSizedArrayNameI {
            template: RuntimeSizedArrayTemplateNameI,
            arr: RawArrayNameI {
                element_type,
                self_region: RegionT::Default,
            },
        }));
        let id_i = IdI {
            package_coord,
            init_steps: self.interner.alloc_slice_from_vec(translated_init_steps),
            local_name: local_name_i,
        };
        let rsa_it = RuntimeSizedArrayIT { name: id_i };
        // Collect the distinct array kind so the backend can declare its region.
        monouts.runtime_sized_arrays.entry(id_i).or_insert_with(|| self.interner.alloc(rsa_it));
        rsa_it
    }


    pub fn translate_kind(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, perspective_region_t: &RegionT, kind_t: &KindT<'s, 't>) -> KindIT<'s, 'i> {
        match kind_t {
            KindT::Int(int_t) => KindIT::IntIT(IntIT { bits: int_t.bits }),
            KindT::Bool(_) => KindIT::BoolIT(BoolIT {  }),
            KindT::Float(_) => KindIT::FloatIT(FloatIT {  }),
            KindT::Void(_) => KindIT::VoidIT(VoidIT {  }),
            KindT::Str(_) => KindIT::StrIT(StrIT {  }),
            KindT::USize(_) => KindIT::USizeIT(USizeIT {  }),
            KindT::Never(never_t) => KindIT::NeverIT(NeverIT { from_break: never_t.from_break }),
            KindT::KindPlaceholder(p) => {
                let sub = substitutions.get(&p.id).expect("translate_kind: missing placeholder substitution");
                match sub {
                    ITemplataI::Kind(k) => k.kind,
                    _ => panic!("translate_kind: placeholder substitution was not a Kind"),
                }
            }
            KindT::Struct(s) => {
                let bound_args = self.translate_bound_args_for_callee(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &self.hinputs.get_instantiation_bound_args(s.id));
                let struct_it = self.translate_struct(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, s, &bound_args);
                KindIT::StructIT(self.interner.alloc(struct_it))
            }
            KindT::Interface(s) => {
                let bound_args = self.translate_bound_args_for_callee(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &self.hinputs.get_instantiation_bound_args(s.id));
                let interface_it = self.translate_interface(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, s, &bound_args);
                KindIT::InterfaceIT(self.interner.alloc(interface_it))
            }
            KindT::StaticSizedArray(a) => KindIT::StaticSizedArrayIT(self.interner.alloc(self.translate_static_sized_array(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, a))),
            KindT::RuntimeSizedArray(a) => KindIT::RuntimeSizedArrayIT(self.interner.alloc(self.translate_runtime_sized_array(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, a))),
            // Onion wrap layers: recurse into the inner kind and re-wrap (mirrors typing's
            // replace_value_type_in_ref). Ownership is which wrap surrounds the base kind.
            KindT::BorrowRef(r) => {
                let inner = self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &r.inner);
                KindIT::BorrowRefIT(self.interner.alloc(BorrowRefIT { inner }))
            }
            KindT::OwnRef(r) => {
                let inner = self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &r.inner);
                KindIT::OwnRefIT(self.interner.alloc(OwnRefIT { inner }))
            }
            KindT::ShareRef(r) => {
                let inner = self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &r.inner);
                KindIT::ShareRefIT(self.interner.alloc(ShareRefIT { inner }))
            }
            KindT::WeakRef(r) => {
                let inner = self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &r.inner);
                KindIT::WeakRefIT(self.interner.alloc(WeakRefIT { inner }))
            }
            KindT::OverloadSet(_) => panic!("translate_kind: OverloadSet is not instantiable"),
        }
    }


    pub fn translate_parameter(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, perspective_region_t: &RegionT, param_t: &ParameterT<'s, 't>) -> ParameterI<'s, 'i> {
        let ParameterT { name, virtuality, tyype, .. } = param_t;
        let type_it =
            self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, tyype);
        let name = self.translate_var_name(name);
        ParameterI {
            name,
            virtuality: virtuality.map(|v| match v { AbstractT => AbstractI }),
            tyype: type_it,
        }
    }


    pub fn translate_templata(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, templata_t: &ITemplataT<'s, 't>) -> ITemplataI<'s, 'i> {
        let result = match templata_t {
            ITemplataT::Placeholder(p) => {
                let PlaceholderTemplataT { id: n, tyype: _ } = **p;
                *_substitutions.get(&n).expect("translate_templata Placeholder: substitution missing")
            }
            ITemplataT::Integer(value) => ITemplataI::Integer(IntegerTemplataI { value: *value }),
            ITemplataT::Boolean(_) => {
                panic!("Unimplemented: translate_templata Boolean");
                // case BooleanTemplataT(value) => BooleanTemplataI(value)
            }
            ITemplataT::String(_) => {
                panic!("Unimplemented: translate_templata String");
                // case StringTemplataT(value) => StringTemplataI(value)
            }
            ITemplataT::Kind(c) => ITemplataI::Kind(KindTemplataI { kind: self.translate_kind(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &c.kind) }),
            _ => {
                panic!("Unimplemented: translate_templata other");
                // case other => vimpl(other)
            }
        };
        result
    }


    pub fn translate_var_name(&self, name: &IVarNameT<'s, 't>) -> IVarNameI<'s, 'i> {
        match name {
            IVarNameT::TypingPassFunctionResultVar(_) => IVarNameI::TypingPassFunctionResultVar(self.interner.alloc(TypingPassFunctionResultVarNameI)),
            IVarNameT::Member(x) => IVarNameI::Member(self.interner.alloc(MemberNameI {
                name: self.scout_arena.intern_str(&humanize_imprecise_name(IImpreciseNameS::CodeName(x.imprecise_name))),
            })),
            IVarNameT::Local(x) => IVarNameI::Local(self.interner.alloc(LocalNameI {
                name: self.scout_arena.intern_str(&humanize_imprecise_name(IImpreciseNameS::CodeName(x.imprecise_name))),
                life: LocationInFunctionEnvironmentI { path: self.interner.alloc_slice_from_vec(x.life.path.to_vec()) },
            })),
            IVarNameT::ClosureParam(ClosureParamNameT { life: LocationInFunctionEnvironmentT { path, .. }, .. }) => IVarNameI::ClosureParam(self.interner.alloc(ClosureParamNameI { life: LocationInFunctionEnvironmentI { path: self.interner.alloc_slice_from_vec(path.to_vec()) } })),
            IVarNameT::TypingPassBlockResultVar(TypingPassBlockResultVarNameT { life: LocationInFunctionEnvironmentT { path, .. } }) => {
                IVarNameI::TypingPassBlockResultVar(self.interner.alloc(TypingPassBlockResultVarNameI {
                                        life: LocationInFunctionEnvironmentI { path: self.interner.alloc_slice_from_vec(path.to_vec()) },
                }))
            }
            IVarNameT::TypingPassTemporaryVar(TypingPassTemporaryVarNameT { life: LocationInFunctionEnvironmentT { path, .. } }) => {
                IVarNameI::TypingPassTemporaryVar(self.interner.alloc(TypingPassTemporaryVarNameI {
                                        life: LocationInFunctionEnvironmentI { path: self.interner.alloc_slice_from_vec(path.to_vec()) },
                }))
            }
            IVarNameT::ConstructingMember(x) => IVarNameI::ConstructingMember(self.interner.alloc(ConstructingMemberNameI {
                name: self.scout_arena.intern_str(&humanize_imprecise_name(IImpreciseNameS::ConstructingMemberImpreciseName(x.imprecise_name))),
            })),
            IVarNameT::Iterable(IterableNameT { life: LocationInFunctionEnvironmentT { path, .. } }) => IVarNameI::Iterable(self.interner.alloc(IterableNameI { life: LocationInFunctionEnvironmentI { path: self.interner.alloc_slice_from_vec(path.to_vec()) } })),
            IVarNameT::Iterator(IteratorNameT { life: LocationInFunctionEnvironmentT { path, .. } }) => IVarNameI::Iterator(self.interner.alloc(IteratorNameI { life: LocationInFunctionEnvironmentI { path: self.interner.alloc_slice_from_vec(path.to_vec()) } })),
            IVarNameT::IterationOption(IterationOptionNameT { life: LocationInFunctionEnvironmentT { path, .. } }) => IVarNameI::IterationOption(self.interner.alloc(IterationOptionNameI { life: LocationInFunctionEnvironmentI { path: self.interner.alloc_slice_from_vec(path.to_vec()) } })),
            IVarNameT::MagicParam(MagicParamNameT { life: LocationInFunctionEnvironmentT { path, .. } }) => IVarNameI::MagicParam(self.interner.alloc(MagicParamNameI { life: LocationInFunctionEnvironmentI { path: self.interner.alloc_slice_from_vec(path.to_vec()) } })),
            IVarNameT::Self_(_) => IVarNameI::Self_(self.interner.alloc(SelfNameI)),
        }
    }


    pub fn translate_function_template_name(&self, _func_template_name_t: &IFunctionTemplateNameT<'s, 't>) -> IFunctionTemplateNameI<'s, 'i> {
        match _func_template_name_t {
            IFunctionTemplateNameT::FunctionTemplate(ftn) => {
                let FunctionTemplateNameT { human_name, code_location: code_loc, .. } = **ftn;
                IFunctionTemplateNameI::FunctionTemplate(self.interner.alloc(FunctionTemplateNameI { human_name, code_location: code_loc }))
            }
            #[allow(unreachable_patterns)]
            other => panic!("translate_function_template_name: unimplemented variant {:?}", discriminant(other)),
        }
    }


    pub fn translate_function_name(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, perspective_region_t: &RegionT, name: &IFunctionNameT<'s, 't>) -> IFunctionNameI<'s, 'i> {
        match *name {
            IFunctionNameT::Function(function_name_t) => {
                let FunctionNameT { template: function_template_name_t, template_args, parameters: params, .. } = *function_name_t;
                let FunctionTemplateNameT { human_name, code_location: code_loc, .. } = *function_template_name_t;
                IFunctionNameI::Function(
                    self.interner.alloc(FunctionNameIX {
                        template: FunctionTemplateNameI { human_name, code_location: code_loc },
                        template_args: self.interner.alloc_slice_from_vec(
                            template_args.iter().map(|template_arg| self.translate_templata(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, template_arg)).collect::<Vec<_>>()),
                        parameters: self.interner.alloc_slice_from_vec(
                            params.iter().map(|param| self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, param)).collect::<Vec<_>>()),
                    }))
            }
            IFunctionNameT::ForwarderFunction(n) => {
                let ForwarderFunctionNameT { template, inner } = *n;
                let ForwarderFunctionTemplateNameT { inner: inner_template, index } = *template;
                IFunctionNameI::ForwarderFunction(
                    self.interner.alloc(ForwarderFunctionNameI {
                        template: *self.interner.alloc(ForwarderFunctionTemplateNameI {
                            inner: self.translate_function_template_name(&inner_template),
                            index,
                        }),
                        inner: self.translate_function_name(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &inner),
                    }))
            }
            IFunctionNameT::ExternFunction(n) => {
                let ExternFunctionNameT { human_name, template_args, parameters, .. } = *n;
                IFunctionNameI::ExternFunction(
                    self.interner.alloc(ExternFunctionNameI {
                        human_name,
                        template_args: self.interner.alloc_slice_from_vec(template_args.iter().map(|template_arg| self.translate_templata(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, template_arg)).collect::<Vec<_>>()),
                        parameters: self.interner.alloc_slice_from_vec(parameters.iter().map(|param| self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, param)).collect::<Vec<_>>()),
                    }))
            }
            IFunctionNameT::FunctionBound(fbn) => {
                let FunctionBoundNameT { template, template_args, parameters: params, .. } = *fbn;
                let FunctionBoundTemplateNameT { human_name, .. } = *template;
                IFunctionNameI::FunctionBound(
                    self.interner.alloc(FunctionBoundNameI {
                        template: FunctionBoundTemplateNameI { human_name },
                        template_args: self.interner.alloc_slice_from_vec(
                            template_args.iter().map(|template_arg| self.translate_templata(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, template_arg)).collect::<Vec<_>>()),
                        parameters: self.interner.alloc_slice_from_vec(
                            params.iter().map(|param| self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, param)).collect::<Vec<_>>()),
                    }))
            }
            IFunctionNameT::AnonymousSubstructConstructor(n) => {
                let AnonymousSubstructConstructorNameT { template, template_args, parameters: params, .. } = *n;
                let inner_template_name_i = match self.translate_name_substituting(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &INameT::AnonymousSubstructConstructorTemplate(template)) {
                    INameI::AnonymousSubstructConstructorTemplate(x) => *x,
                    _ => panic!("translate_function_name AnonymousSubstructConstructor: expected AnonymousSubstructConstructorTemplate"),
                };
                IFunctionNameI::AnonymousSubstructConstructor(
                    self.interner.alloc(AnonymousSubstructConstructorNameI {
                        template: inner_template_name_i,
                        template_args: self.interner.alloc_slice_from_vec(template_args.iter().map(|t| self.translate_templata(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, t)).collect::<Vec<_>>()),
                        parameters: self.interner.alloc_slice_from_vec(params.iter().map(|p| self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, p)).collect::<Vec<_>>()),
                    }))
            }
            IFunctionNameT::LambdaCallFunction(n) => {
                let LambdaCallFunctionNameT { template: LambdaCallFunctionTemplateNameT { code_location, param_types: param_types_for_generic, .. }, template_args, parameters: param_types, .. } = *n;
                IFunctionNameI::LambdaCallFunction(
                    self.interner.alloc(LambdaCallFunctionNameI {
                        template: *self.interner.alloc(LambdaCallFunctionTemplateNameI {
                                                        code_location: *code_location,
                            param_types: self.interner.alloc_slice_from_vec(param_types_for_generic.iter().map(|p| self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, p)).collect::<Vec<_>>()),
                        }),
                        template_args: self.interner.alloc_slice_from_vec(template_args.iter().map(|t| self.translate_templata(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, t)).collect::<Vec<_>>()),
                        parameters: self.interner.alloc_slice_from_vec(param_types.iter().map(|p| self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, p)).collect::<Vec<_>>()),
                    }))
            }
            IFunctionNameT::OverrideDispatcher(_) => {
                panic!("Unimplemented: translate_function_name OverrideDispatcher");
                // OverrideDispatcherNameI(
                //   OverrideDispatcherTemplateNameI(
                //     translateId[IImplTemplateNameT, IImplTemplateNameI[sI]](implTemplateId, translateImplTemplateName)),
                //   templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
                //   paramTypes.map(translateCoord(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)))
            }
            _other => {
                panic!("Unimplemented: translate_function_name other");
                // case other => vimpl(other)
            }
        }
    }


    pub fn translate_impl_name(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, _name: &IImplNameT<'s, 't>) -> IImplNameI<'s, 'i> {
        match _name {
            IImplNameT::Impl(n) => {
                let ImplNameT { template: ImplTemplateNameT { code_location, .. }, template_args, sub_citizen, .. } = **n;
                let template_args_i: Vec<ITemplataI<'s, 'i>> = template_args.iter().map(|t| self.translate_templata(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, t)).collect();
                let sub_citizen_id = sub_citizen.id();
                let bound_args_for_callee = self.translate_bound_args_for_callee(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &self.hinputs.get_instantiation_bound_args(sub_citizen_id));
                let sub_citizen_i = self.translate_citizen(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &sub_citizen, &bound_args_for_callee);
                IImplNameI::Impl(self.interner.alloc(ImplNameI {
                    template: IImplTemplateNameI::ImplTemplate(self.interner.alloc(ImplTemplateNameI { code_location: *code_location })),
                    template_args: self.interner.bump().alloc_slice_fill_iter(template_args_i.into_iter()),
                    sub_citizen: sub_citizen_i,
                }))
            }
            IImplNameT::ImplBound(_) => {
                panic!("Unimplemented: translate_impl_name ImplBound");
                // ImplBoundNameI(
                //   ImplBoundTemplateNameI(codeLocationS),
                //   templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)))
            }
            IImplNameT::AnonymousSubstructImpl(n) => {
                let AnonymousSubstructImplNameT { template, template_args, sub_citizen, .. } = **n;
                let AnonymousSubstructImplTemplateNameT { interface, .. } = *template;
                let template_args_i: Vec<ITemplataI<'s, 'i>> = template_args.iter().map(|t| self.translate_templata(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, t)).collect();
                let sub_citizen_id = sub_citizen.id();
                let bound_args_for_callee = self.translate_bound_args_for_callee(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &self.hinputs.get_instantiation_bound_args(sub_citizen_id));
                let sub_citizen_i = self.translate_citizen(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &sub_citizen, &bound_args_for_callee);
                IImplNameI::AnonymousSubstructImpl(self.interner.alloc(AnonymousSubstructImplNameI {
                    template: AnonymousSubstructImplTemplateNameI {
                        interface: self.translate_interface_template_name(&interface),
                    },
                    template_args: self.interner.bump().alloc_slice_fill_iter(template_args_i.into_iter()),
                    sub_citizen: sub_citizen_i,
                }))
            }
        }
    }


    pub fn translate_impl_template_name(_name: &IImplTemplateNameT<'s, 't>) -> IImplTemplateNameI<'s, 'i> {
        panic!("Unimplemented: translate_impl_template_name");
        // name match {
        //   case ImplTemplateNameT(codeLocationS) => ImplTemplateNameI(codeLocationS)
        //   case ImplBoundTemplateNameT(codeLocationS) => ImplBoundTemplateNameI(codeLocationS)
        //   case AnonymousSubstructImplTemplateNameT(interface) => {
        //     AnonymousSubstructImplTemplateNameI(
        //       translateInterfaceTemplateName(interface))
        //   }
        // }
    }


    pub fn translate_struct_name(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, _name: &IStructNameT<'s, 't>) -> IStructNameI<'s, 'i> {
        let new_perspective_region_t = RegionT::Default;
        match _name {
            IStructNameT::Struct(StructNameT { template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name, .. }), template_args, .. }) => {
                let template_args_si: Vec<ITemplataI<'s, 'i>> = template_args.iter().map(|t| self.translate_templata(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, &new_perspective_region_t, t)).collect();
                IStructNameI::Struct(self.interner.alloc(StructNameI {
                    template: IStructTemplateNameI::StructTemplate(self.interner.alloc(StructTemplateNameI { human_name: *human_name })),
                    template_args: self.interner.bump().alloc_slice_fill_iter(template_args_si.into_iter()),
                }))
            }
            IStructNameT::AnonymousSubstruct(AnonymousSubstructNameT { template, template_args, .. }) => {
                let AnonymousSubstructTemplateNameT { interface, .. } = **template;
                let template_args_si: Vec<ITemplataI<'s, 'i>> = template_args.iter().map(|t| self.translate_templata(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, &new_perspective_region_t, t)).collect();
                IStructNameI::AnonymousSubstruct(self.interner.alloc(AnonymousSubstructNameI {
                    template: *self.interner.alloc(AnonymousSubstructTemplateNameI {
                        interface: self.translate_interface_template_name(&interface),
                    }),
                    template_args: self.interner.bump().alloc_slice_fill_iter(template_args_si.into_iter()),
                }))
            }
            IStructNameT::LambdaCitizen(LambdaCitizenNameT { template: LambdaCitizenTemplateNameT { code_location, .. } }) => {
                IStructNameI::LambdaCitizen(self.interner.alloc(LambdaCitizenNameI {
                    template: *self.interner.alloc(LambdaCitizenTemplateNameI { code_location: *code_location }),
                }))
            }
            other => panic!("translate_struct_name: unimplemented variant {:?}", discriminant(other)),
        }
    }


    pub fn translate_interface_name(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, _name: &IInterfaceNameT<'s, 't>) -> IInterfaceNameI<'s, 'i> {
        match _name {
            IInterfaceNameT::Interface(InterfaceNameT { template: InterfaceTemplateNameT { human_namee: human_name, .. }, template_args, .. }) => {
                let template_args_si: Vec<ITemplataI<'s, 'i>> = template_args.iter().map(|t| self.translate_templata(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, t)).collect();
                IInterfaceNameI::Interface(self.interner.alloc(InterfaceNameI {
                    template: IInterfaceTemplateNameI::InterfaceTemplate(self.interner.alloc(InterfaceTemplateNameI { human_namee: *human_name })),
                    template_args: self.interner.bump().alloc_slice_fill_iter(template_args_si.into_iter()),
                }))
            }
            #[allow(unreachable_patterns)] // catch-all; unreachable until more IInterfaceNameT variants exist
            other => panic!("translate_interface_name: unimplemented variant {:?}", discriminant(other)),
        }
    }


    pub fn translate_interface_template_name(&self, _name: &IInterfaceTemplateNameT<'s, 't>) -> IInterfaceTemplateNameI<'s, 'i> {
        match _name {
            IInterfaceTemplateNameT::InterfaceTemplate(InterfaceTemplateNameT { human_namee, .. }) => {
                IInterfaceTemplateNameI::InterfaceTemplate(self.interner.alloc(InterfaceTemplateNameI { human_namee: *human_namee }))
            }
            #[allow(unreachable_patterns)]
            other => panic!("translate_interface_template_name: unimplemented variant {:?}", discriminant(other)),
        }
    }

    pub fn translate_name_substituting(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _perspective_region_t: &RegionT, name: &INameT<'s, 't>) -> INameI<'s, 'i> {
        match *name {
            n if IVarNameT::try_from(n).is_ok() => panic!("Unimplemented: translate_name_substituting IVarNameT"),
            INameT::KindPlaceholderTemplate(_) => panic!("translate_name_substituting: KindPlaceholderTemplate vwat"),
            INameT::KindPlaceholder(_) => panic!("translate_name_substituting: KindPlaceholder vwat"),
            INameT::Struct(_) => panic!("Unimplemented: translate_name_substituting Struct"),
            INameT::ForwarderFunctionTemplate(fftn) => {
                let ForwarderFunctionTemplateNameT { inner, index } = *fftn;
                INameI::ForwarderFunctionTemplate(self.interner.alloc(ForwarderFunctionTemplateNameI {
                    inner: self.translate_function_template_name(&inner),
                    index,
                }))
            }
            INameT::AnonymousSubstructConstructorTemplate(astn) => {
                let AnonymousSubstructConstructorTemplateNameT { substruct, .. } = *astn;
                let substruct_as_name: INameT<'s, 't> = match substruct {
                    ICitizenTemplateNameT::StaticSizedArrayTemplate(x) => x.into(),
                    ICitizenTemplateNameT::RuntimeSizedArrayTemplate(x) => x.into(),
                    ICitizenTemplateNameT::LambdaCitizenTemplate(x) => x.into(),
                    ICitizenTemplateNameT::StructTemplate(x) => x.into(),
                    ICitizenTemplateNameT::InterfaceTemplate(x) => x.into(),
                    ICitizenTemplateNameT::AnonymousSubstructTemplate(x) => x.into(),
                };
                let translated = self.translate_name_substituting(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &substruct_as_name);
                let citizen_template_name_i: ICitizenTemplateNameI<'s, 'i> = ICitizenTemplateNameI::try_from(translated).unwrap();
                INameI::AnonymousSubstructConstructorTemplate(self.interner.alloc(AnonymousSubstructConstructorTemplateNameI { substruct: citizen_template_name_i }))
            }
            INameT::FunctionTemplate(ftn) => {
                let FunctionTemplateNameT { human_name, code_location: code_loc, .. } = *ftn;
                INameI::FunctionTemplate(self.interner.alloc(FunctionTemplateNameI { human_name, code_location: code_loc }))
            }
            INameT::StructTemplate(stn) => {
                let StructTemplateNameT { human_name, .. } = *stn;
                INameI::StructTemplate(self.interner.alloc(StructTemplateNameI { human_name }))
            }
            INameT::LambdaCitizenTemplate(LambdaCitizenTemplateNameT { code_location, .. }) => {
                INameI::LambdaCitizenTemplate(self.interner.alloc(LambdaCitizenTemplateNameI { code_location: *code_location }))
            }
            INameT::AnonymousSubstructTemplate(astn) => {
                let AnonymousSubstructTemplateNameT { interface, .. } = *astn;
                INameI::AnonymousSubstructTemplate(self.interner.alloc(AnonymousSubstructTemplateNameI {
                    interface: self.translate_interface_template_name(&interface),
                }))
            }
            INameT::LambdaCitizen(_) => panic!("Unimplemented: translate_name_substituting LambdaCitizen"),
            INameT::InterfaceTemplate(itn) => {
                let InterfaceTemplateNameT { human_namee, .. } = *itn;
                INameI::InterfaceTemplate(self.interner.alloc(InterfaceTemplateNameI { human_namee }))
            }
            INameT::Function(_) | INameT::ForwarderFunction(_) | INameT::ExternFunction(_) | INameT::FunctionBound(_) | INameT::LambdaCallFunction(_) | INameT::AnonymousSubstructConstructor(_) | INameT::PredictedFunction(_) => {
                let f: IFunctionNameT<'s, 't> = (*name).try_into().unwrap();
                INameI::from(self.translate_function_name(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &f))
            }
            _ => panic!("Unimplemented: translate_name_substituting other"),
        }
    }


    pub fn translate_impl_definition(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _instantiation_bounds_for_unsubstituted_impl: InstantiationBoundArgumentsI<'s, 'i>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i>>, _impl_id_t: &IdT<'s, 't>, _impl_id: &IdI<'s, 'i>, _impl_definition: &EdgeT<'s, 't>) {
        let perspective_region_t = RegionT::Default;
        let sub_citizen_bound_args = self.translate_bound_args_for_callee(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, &perspective_region_t, &self.hinputs.get_instantiation_bound_args(_impl_definition.sub_citizen.id()));
        let sub_citizen = self.translate_citizen(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, &perspective_region_t, &_impl_definition.sub_citizen, &sub_citizen_bound_args);
        let super_interface_bound_args = self.translate_bound_args_for_callee(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, &perspective_region_t, &self.hinputs.get_instantiation_bound_args(_impl_definition.super_interface));
        let super_interface = self.translate_interface_id(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, &perspective_region_t, &_impl_definition.super_interface, &super_interface_bound_args);

        let mutability = *_monouts.interface_to_sharedness.get(&super_interface).expect("translate_impl_definition: superInterfaceC mutability missing");
        if _monouts.impl_to_sharedness.contains_key(_impl_id) {
            return;
        }
        _monouts.impl_to_sharedness.insert(*_impl_id, mutability);

        // We assemble the EdgeI at the very end of the instantiating stage.

        _monouts.impls.insert(*_impl_id, (sub_citizen, super_interface, _denizen_bound_to_denizen_caller_supplied_thing.clone(), _instantiation_bounds_for_unsubstituted_impl));

        _monouts.interface_to_impl_to_abstract_prototype_to_override.get_mut(&super_interface).expect("vassertSome: interface_to_impl_to_abstract_prototype_to_override")
            .insert(*_impl_id, IndexMap::default());
        _monouts.interface_to_impls.get_mut(&super_interface).expect("vassertSome: interface_to_impls").push((*_impl_id_t, *_impl_id));
    }
}
