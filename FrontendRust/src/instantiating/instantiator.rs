/*
package dev.vale.instantiating

import dev.vale.instantiating.ast._
import dev.vale.options.GlobalOptions
import dev.vale._
import dev.vale.instantiating.ast.ITemplataI.expectRegionTemplata
import dev.vale.postparsing._
import dev.vale.typing.TemplataCompiler._
import dev.vale.typing._
import dev.vale.typing.ast._
import dev.vale.typing.env._
import dev.vale.typing.names._
import dev.vale.typing.templata.ITemplataT._
import dev.vale.typing.templata._
import dev.vale.typing.types._

import scala.collection.immutable.Map
import scala.collection.mutable
*/
use crate::instantiating::ast::names::*;
use crate::instantiating::ast::types::*;
use crate::instantiating::ast::ast::*;
use crate::instantiating::ast::citizens::*;
use crate::instantiating::ast::templata::*;
use crate::instantiating::ast::hinputs::*;
use crate::instantiating::ast::expressions::*;
use crate::instantiating::instantiating_interner::InstantiatingInterner;
use crate::typing::typing_interner::TypingInterner;
use crate::instantiating::region_collapser_individual;
use crate::instantiating::region_collapser_consistent;
use crate::instantiating::region_counter;
use crate::instantiating::collector;
use crate::instantiating::collector::NodeRefI;
use crate::typing::names::names::*;
use crate::typing::ast::ast::*;
use crate::typing::ast::citizens::*;
use crate::typing::types::types::*;
use crate::typing::hinputs_t::*;
use crate::typing::compiler::Compiler;
use crate::utils::vassert::vassert_one;
use crate::postparsing::names::IRuneS;
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::keywords::Keywords;
use crate::compile_options::GlobalOptions;
use crate::typing::templata::templata::ITemplataT;
use crate::typing::ast::expressions::{ReferenceExpressionTE, ExpressionTE, AddressExpressionTE};
use crate::typing::env::function_environment_t::{ILocalVariableT, ReferenceLocalVariableT, AddressibleLocalVariableT};
use indexmap::IndexMap;
// mig: struct DenizenBoundToDenizenCallerBoundArgI
/// Temporary state
#[derive(Clone, PartialEq, Eq)]
pub struct DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i> where 's: 't, 's: 'i {
    pub func_id_to_bound_arg_prototype: IndexMap<IdT<'s, 't>, &'i PrototypeI<'s, 'i, sI>>,
    pub bound_param_impl_id_to_bound_arg_impl_id: IndexMap<IdT<'s, 't>, IdI<'s, 'i, sI>>,
}
// mig: impl DenizenBoundToDenizenCallerBoundArgI
/*
case class DenizenBoundToDenizenCallerBoundArgS(
  funcIdToBoundArgPrototype: Map[IdT[FunctionBoundNameT], PrototypeI[sI]],
  boundParamImplIdToBoundArgImplId: Map[IdT[ImplBoundNameT], IdI[sI, IImplNameI[sI]]]) {
*/
// mig: fn plus
impl<'s, 't, 'i> DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i> where 's: 't, 's: 'i {
    pub fn plus(&self, that: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>) -> DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i> {
        panic!("Unimplemented: plus");
    }
}
/*
  def plus(that: DenizenBoundToDenizenCallerBoundArgS): DenizenBoundToDenizenCallerBoundArgS = {
    val DenizenBoundToDenizenCallerBoundArgS(
      thatFuncIdToBoundArgPrototype,
      thatBoundParamImplIdToBoundArgImplId
    ) = that

    DenizenBoundToDenizenCallerBoundArgS(
      U.unionMapsExpectNoConflict[IdT[FunctionBoundNameT], PrototypeI[sI]](funcIdToBoundArgPrototype, thatFuncIdToBoundArgPrototype, _==_),
      U.unionMapsExpectNoConflict[IdT[ImplBoundNameT], IdI[sI, IImplNameI
          [sI]]](boundParamImplIdToBoundArgImplId, thatBoundParamImplIdToBoundArgImplId, _==_))
  }
}
*/
// mig: struct InstantiatedOutputsI
/// Temporary state
pub struct InstantiatedOutputsI<'s, 't, 'i> where 's: 't, 's: 'i {
    pub functions: IndexMap<IdI<'s, 'i, cI>, &'i FunctionDefinitionI<'s, 'i>>,
    pub structs: IndexMap<IdI<'s, 'i, cI>, &'i StructDefinitionI<'s, 'i, cI>>,
    pub interfaces_without_methods: IndexMap<IdI<'s, 'i, cI>, &'i InterfaceDefinitionI<'s, 'i, cI>>,
    pub struct_to_mutability: IndexMap<IdI<'s, 'i, cI>, MutabilityI>,
    pub struct_to_bounds: IndexMap<IdI<'s, 'i, sI>, DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>,
    pub interface_to_mutability: IndexMap<IdI<'s, 'i, cI>, MutabilityI>,
    pub interface_to_bounds: IndexMap<IdI<'s, 'i, sI>, DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>,
    pub impl_to_mutability: IndexMap<IdI<'s, 'i, cI>, MutabilityI>,
    pub impl_to_bounds: IndexMap<IdI<'s, 'i, sI>, DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>,
    pub interface_to_impls: IndexMap<IdI<'s, 'i, cI>, Vec<(IdT<'s, 't>, IdI<'s, 'i, cI>)>>,
    pub interface_to_abstract_func_to_virtual_index: IndexMap<IdI<'s, 'i, cI>, IndexMap<PrototypeI<'s, 'i, cI>, usize>>,
    pub impls: IndexMap<IdI<'s, 'i, cI>, (ICitizenIT<'s, 'i, cI>, IdI<'s, 'i, cI>, DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, InstantiationBoundArgumentsI<'s, 'i>)>,
    pub abstract_func_to_bounds: IndexMap<IdI<'s, 'i, cI>, (DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, InstantiationBoundArgumentsI<'s, 'i>)>,
    pub interface_to_impl_to_abstract_prototype_to_override: IndexMap<IdI<'s, 'i, cI>, IndexMap<IdI<'s, 'i, cI>, IndexMap<PrototypeI<'s, 'i, cI>, PrototypeI<'s, 'i, cI>>>>,
    pub new_impls: Vec<(IdT<'s, 't>, IdI<'s, 'i, nI>, InstantiationBoundArgumentsI<'s, 'i>)>,
    pub new_abstract_funcs: Vec<(PrototypeT<'s, 't>, PrototypeI<'s, 'i, nI>, usize, IdI<'s, 'i, cI>, InstantiationBoundArgumentsI<'s, 'i>)>,
    pub new_functions: Vec<(PrototypeT<'s, 't>, PrototypeI<'s, 'i, nI>, InstantiationBoundArgumentsI<'s, 'i>, Option<DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>)>,
    pub kind_externs: Vec<KindExternI<'s, 'i>>,
    pub function_externs: Vec<FunctionExternI<'s, 'i>>,
}
/*
class InstantiatedOutputs() {
  val functions: mutable.HashMap[IdI[cI, IFunctionNameI[cI]], FunctionDefinitionI] =
    mutable.HashMap()
  val structs: mutable.HashMap[IdI[cI, IStructNameI[cI]], StructDefinitionI] = mutable.HashMap()
  val interfacesWithoutMethods: mutable.HashMap[IdI[cI, IInterfaceNameI[cI]], InterfaceDefinitionI] = mutable.HashMap()

  // We can get some recursion if we have a self-referential struct like:
  //   struct Node<T> { value T; next Opt<Node<T>>; }
  // So we need these to short-circuit that nonsense.
  val structToMutability: mutable.HashMap[IdI[cI, IStructNameI[cI]], MutabilityI] = mutable.HashMap()
  val structToBounds: mutable.HashMap[IdI[sI, IStructNameI[sI]], DenizenBoundToDenizenCallerBoundArgS] = mutable.HashMap()
  val interfaceToMutability: mutable.HashMap[IdI[cI, IInterfaceNameI[cI]], MutabilityI] = mutable.HashMap()
  val interfaceToBounds: mutable.HashMap[IdI[sI, IInterfaceNameI[sI]], DenizenBoundToDenizenCallerBoundArgS] = mutable.HashMap()
  val implToMutability: mutable.HashMap[IdI[cI, IImplNameI[cI]], MutabilityI] = mutable.HashMap()
  val implToBounds: mutable.HashMap[IdI[sI, IImplNameI[sI]], DenizenBoundToDenizenCallerBoundArgS] = mutable.HashMap()

  //  val immKindToDestructor: mutable.HashMap[KindT, PrototypeT] =
  //    mutable.HashMap[KindT, PrototypeT]()

  // We already know from the hinputs that Some<T> implements Opt<T>.
  // In this map, we'll know that Some<int> implements Opt<int>, Some<bool> implements Opt<bool>, etc.
  val interfaceToImpls: mutable.HashMap[IdI[cI, IInterfaceNameI[cI]], mutable.HashSet[(IdT[IImplNameT], IdI[cI, IImplNameI[cI]])]] =
  mutable.HashMap()
  val interfaceToAbstractFuncToVirtualIndex: mutable.HashMap[IdI[cI, IInterfaceNameI[cI]], mutable.HashMap[PrototypeI[cI], Int]] =
    mutable.HashMap()
  val impls:
    mutable.HashMap[
      IdI[cI, IImplNameI[cI]],
      (ICitizenIT[cI], IdI[cI, IInterfaceNameI[cI]], DenizenBoundToDenizenCallerBoundArgS, InstantiationBoundArgumentsI)] =
    mutable.HashMap()
  // We already know from the hinputs that Opt<T has drop> has func drop(T).
  // In this map, we'll know that Opt<int> has func drop(int).
  val abstractFuncToBounds: mutable.HashMap[IdI[cI, IFunctionNameI[cI]], (DenizenBoundToDenizenCallerBoundArgS, InstantiationBoundArgumentsI)] =
    mutable.HashMap()
  // This map collects all overrides for every impl. We'll use it to assemble vtables soon.
  val interfaceToImplToAbstractPrototypeToOverride:
    mutable.HashMap[IdI[cI, IInterfaceNameI[cI]], mutable.HashMap[IdI[cI, IImplNameI[cI]], mutable.HashMap[PrototypeI[cI], PrototypeI[cI]]]] =
    mutable.HashMap()

  // These are new impls and abstract funcs we discover for interfaces.
  // As we discover a new impl or a new abstract func, we'll later need to stamp a lot more overrides either way.
  val newImpls: mutable.Queue[(IdT[IImplNameT], IdI[nI, IImplNameI[nI]], InstantiationBoundArgumentsI)] = mutable.Queue()
  // The int is a virtual index
  val newAbstractFuncs: mutable.Queue[(PrototypeT[IFunctionNameT], PrototypeI[nI], Int, IdI[cI, IInterfaceNameI[cI]], InstantiationBoundArgumentsI)] = mutable.Queue()
  val newFunctions: mutable.Queue[(PrototypeT[IFunctionNameT], PrototypeI[nI], InstantiationBoundArgumentsI, Option[DenizenBoundToDenizenCallerBoundArgS])] = mutable.Queue()
  val kindExterns = new mutable.ArrayBuffer[KindExternI]()
  val functionExterns = new mutable.ArrayBuffer[FunctionExternI]()
*/
// mig: impl InstantiatedOutputsI
// mig: fn new
impl<'s, 't, 'i> InstantiatedOutputsI<'s, 't, 'i> where 's: 't, 's: 'i {
  pub fn new() -> Self {
    InstantiatedOutputsI {
      functions: IndexMap::new(),
      structs: IndexMap::new(),
      interfaces_without_methods: IndexMap::new(),
      struct_to_mutability: IndexMap::new(),
      struct_to_bounds: IndexMap::new(),
      interface_to_mutability: IndexMap::new(),
      interface_to_bounds: IndexMap::new(),
      impl_to_mutability: IndexMap::new(),
      impl_to_bounds: IndexMap::new(),
      interface_to_impls: IndexMap::new(),
      interface_to_abstract_func_to_virtual_index: IndexMap::new(),
      impls: IndexMap::new(),
      abstract_func_to_bounds: IndexMap::new(),
      interface_to_impl_to_abstract_prototype_to_override: IndexMap::new(),
      new_impls: Vec::new(),
      new_abstract_funcs: Vec::new(),
      new_functions: Vec::new(),
      kind_externs: Vec::new(),
      function_externs: Vec::new(),
    }
  }
}
// mig: fn add_method_to_v_table
impl<'s, 't, 'i> InstantiatedOutputsI<'s, 't, 'i> where 's: 't, 's: 'i {
    pub fn add_method_to_v_table(&mut self, impl_id: IdI<'s, 'i, cI>, super_interface_id: IdI<'s, 'i, cI>, abstract_func_prototype: PrototypeI<'s, 'i, cI>, override_: PrototypeI<'s, 'i, cI>) {
        panic!("Unimplemented: add_method_to_v_table");
    }
}
/*
  def addMethodToVTable(
    implId: IdI[cI, IImplNameI[cI]],
    superInterfaceId: IdI[cI, IInterfaceNameI[cI]],
    abstractFuncPrototype: PrototypeI[cI],
    overrride: PrototypeI[cI]
  ) = {
    val map =
      interfaceToImplToAbstractPrototypeToOverride
        .getOrElseUpdate(superInterfaceId, mutable.HashMap())
        .getOrElseUpdate(implId, mutable.HashMap())
    vassert(!map.contains(abstractFuncPrototype))
    map.put(abstractFuncPrototype, overrride)
  }
}

object Instantiator {
*/
// mig: fn translate
pub fn translate<'s, 'ctx, 't, 'i>(opts: &'ctx GlobalOptions, interner: &'ctx InstantiatingInterner<'s, 'i>, typing_interner: &'ctx TypingInterner<'s, 't>, keywords: &'ctx Keywords<'s>, hinputs: &'ctx HinputsT<'s, 't>) -> HinputsI<'s, 'i>
where 's: 't, 's: 'i {
    let mut monouts = InstantiatedOutputsI::new();
    let instantiator = InstantiatorI { opts, interner, typing_interner, keywords, hinputs };
    instantiator.translate_method(&mut monouts)
}
/*
  def translate(
    opts: GlobalOptions,
    interner: Interner,
    keywords: Keywords,
    hinputs: HinputsT):
  HinputsI = {
    val monouts = new InstantiatedOutputs()
    val instantiator = new Instantiator(opts, interner, keywords, hinputs, monouts)
    instantiator.translate()
  }
}
*/
// mig: struct InstantiatorI
/// Temporary state
pub struct InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub opts: &'ctx GlobalOptions,
    pub interner: &'ctx InstantiatingInterner<'s, 'i>,
    // Scala used one Interner; Rust split it into typing + instantiating, so the instantiator holds
    // the typing half too, for T-side helpers like TemplataCompiler::get_super_template.
    pub typing_interner: &'ctx TypingInterner<'s, 't>,
    pub keywords: &'ctx Keywords<'s>,
    pub hinputs: &'ctx HinputsT<'s, 't>,
}
// mig: impl InstantiatorI
/*
class Instantiator(
  opts: GlobalOptions,
  interner: Interner,
  keywords: Keywords,
  hinputs: HinputsT,
  monouts: InstantiatedOutputs) {
*/
// mig: fn translate
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

        let kind_exports_c: Vec<KindExportI<'s, 'i>> =
            kind_exports_t.iter().map(|&kind_export_t| {
                let KindExportT { range, tyype, id: export_placeholdered_id_t, exported_name } = kind_export_t;
                let export_id_s = self.translate_id(
                    export_placeholdered_id_t,
                    |export_name_t: &INameT<'s, 't>| -> INameValI<'s, 'i, sI> {
                        match export_name_t {
                            INameT::Export(ExportNameT { template: ExportTemplateNameT { code_loc, .. }, .. }) => {
                                INameValI::Export(ExportNameI {
                                    template: ExportTemplateNameI { _marker: std::marker::PhantomData, code_loc: *code_loc },
                                    region: RegionTemplataI { pure_height: 0, _marker: std::marker::PhantomData },
                                })
                            }
                            _ => panic!("Unimplemented: translate_method kind_exports translateId closure"),
                        }
                    });
                let export_id_c =
                    region_collapser_individual::collapse_export_id(self.interner, region_counter::count_export_id(&export_id_s), &export_id_s);
                let substitutions = self.assemble_placeholder_map(export_placeholdered_id_t, &export_id_s);
                let denizen_bound_to_denizen_caller_supplied_thing = DenizenBoundToDenizenCallerBoundArgI {
                    func_id_to_bound_arg_prototype: IndexMap::new(),
                    bound_param_impl_id_to_bound_arg_impl_id: IndexMap::new(),
                };
                let kind_it = self.translate_kind(
                    monouts,
                    export_placeholdered_id_t,
                    &denizen_bound_to_denizen_caller_supplied_thing,
                    &substitutions,
                    &RegionT { region: IRegionT::Default },
                    &tyype);
                let kind_ct = region_collapser_individual::collapse_kind(self.interner, &kind_it);
                KindExportI {
                    range: *range,
                    tyype: kind_ct,
                    id: export_id_c,
                    exported_name: *exported_name,
                }
            }).collect();

        let function_exports_c: Vec<FunctionExportI<'s, 'i>> =
            function_exports_t.iter().map(|&function_export_t| {
                let FunctionExportT { range, prototype: prototype_t, export_id: export_placeholdered_id_t, exported_name } = function_export_t;
                let perspective_region_t = RegionT { region: IRegionT::Default };
                let export_id_s = self.translate_id(
                    export_placeholdered_id_t,
                    |export_name_t: &INameT<'s, 't>| -> INameValI<'s, 'i, sI> {
                        match export_name_t {
                            INameT::Export(ExportNameT { template: ExportTemplateNameT { code_loc, .. }, .. }) => {
                                INameValI::Export(ExportNameI {
                                    template: ExportTemplateNameI { _marker: std::marker::PhantomData, code_loc: *code_loc },
                                    region: RegionTemplataI { pure_height: 0, _marker: std::marker::PhantomData },
                                })
                            }
                            _ => panic!("Unimplemented: translate_method function_exports translateId closure"),
                        }
                    });
                let export_id_c =
                    region_collapser_individual::collapse_export_id(self.interner, region_counter::count_export_id(&export_id_s), &export_id_s);
                let substitutions = self.assemble_placeholder_map(export_placeholdered_id_t, &export_id_s);

                let denizen_bound_to_denizen_caller_supplied_thing = DenizenBoundToDenizenCallerBoundArgI {
                    func_id_to_bound_arg_prototype: IndexMap::new(),
                    bound_param_impl_id_to_bound_arg_impl_id: IndexMap::new(),
                };
                let (_, prototype_c) =
                    self.translate_prototype(
                        monouts,
                        export_placeholdered_id_t,
                        &denizen_bound_to_denizen_caller_supplied_thing,
                        &substitutions,
                        &perspective_region_t,
                        &prototype_t);

                // Scala's `Collector.all(prototypeC, {case PlaceholderTemplataT => vwat()})` sanity check is omitted:
                // Rust's I-side AST is statically typed (PrototypeI holds only ITemplataI, which has no placeholder
                // variant), so a leftover typing-pass placeholder can't reach a typed PrototypeI. (Architect-approved parity gap.)
                FunctionExportI {
                    range: *range,
                    prototype: self.interner.intern_prototype_ci(PrototypeIValI { id: prototype_c.id, return_type: prototype_c.return_type }),
                    export_id: export_id_c,
                    exported_name: *exported_name,
                }
            }).collect();

        let non_generic_func_externs_c: Vec<FunctionExternI<'s, 'i>> =
            function_externs_t.iter().flat_map(|&function_extern_t| -> Option<FunctionExternI<'s, 'i>> {
                let FunctionExternT { range: _range, extern_placeholdered_id: extern_placeholdered_id_t, prototype: prototype_t, extern_name: _externed_name, generic_parameter_inheritance: maybe_inheritance } = function_extern_t;
                let is_generic = !IInstantiationNameT::try_from(prototype_t.id.local_name).unwrap().template_args().is_empty();
                if is_generic {
                    // We don't handle generic externs yet, that comes later when we see what instantiations are actually needed.
                    // We handle those like we handle normal non-extern generic functions.
                    None
                } else {
                    let perspective_region_t = RegionT { region: IRegionT::Default };

                    let extern_id_s = self.translate_id(
                        extern_placeholdered_id_t,
                        |extern_name_t: &INameT<'s, 't>| -> INameValI<'s, 'i, sI> {
                            match extern_name_t {
                                INameT::Extern(ExternNameT { template: ExternTemplateNameT { code_loc, .. }, .. }) => {
                                    INameValI::Extern(ExternNameI {
                                        template: ExternTemplateNameI { _marker: std::marker::PhantomData, code_loc: *code_loc },
                                        region: RegionTemplataI { pure_height: 0, _marker: std::marker::PhantomData },
                                    })
                                }
                                _ => panic!("Unimplemented: translate_method function_externs translateId closure"),
                            }
                        });
                    let _extern_id_c =
                        region_collapser_individual::collapse_extern_id(self.interner, region_counter::count_extern_id(&extern_id_s), &extern_id_s);

                    let substitutions = self.assemble_placeholder_map(extern_placeholdered_id_t, &extern_id_s);

                    let denizen_bound_to_denizen_caller_supplied_thing = DenizenBoundToDenizenCallerBoundArgI {
                        func_id_to_bound_arg_prototype: IndexMap::new(),
                        bound_param_impl_id_to_bound_arg_impl_id: IndexMap::new(),
                    };
                    let (_, prototype_c) =
                        self.translate_prototype(
                            monouts,
                            extern_placeholdered_id_t,
                            &denizen_bound_to_denizen_caller_supplied_thing,
                            &substitutions,
                            &perspective_region_t,
                            &prototype_t);

                    // Scala's `Collector.all(prototypeC, {case PlaceholderTemplataT => vwat()})` sanity check is omitted:
                    // Rust's I-side AST is statically typed (PrototypeI holds only ITemplataI, no placeholder variant),
                    // so a leftover typing-pass placeholder can't reach a typed PrototypeI. (Architect-approved parity gap.)
                    Some(FunctionExternI {
                        prototype: self.interner.intern_prototype_ci(PrototypeIValI { id: prototype_c.id, return_type: prototype_c.return_type }),
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
                let (new_func_id_t, new_func_id_n, instantiation_bound_args, maybe_denizen_bound_to_denizen_caller_supplied_thing) =
                    monouts.new_functions.remove(0);
                self.translate_function(
                    monouts, &new_func_id_t, &new_func_id_n, &instantiation_bound_args,
                    maybe_denizen_bound_to_denizen_caller_supplied_thing.as_ref());
                true
            } else if !monouts.new_impls.is_empty() {
                let (impl_id_t, impl_id_n, instantiation_bounds_for_unsubstituted_impl) = monouts.new_impls.remove(0);
                self.translate_impl(monouts, &impl_id_t, &impl_id_n, instantiation_bounds_for_unsubstituted_impl);
                true
            } else if !monouts.new_abstract_funcs.is_empty() {
                panic!("Unimplemented: translate_method while newAbstractFuncs")
            } else {
                false
            }
        } {}

        let interface_edge_blueprints =
            ArenaIndexMap::from_iter_in(
                monouts.interface_to_abstract_func_to_virtual_index.iter().map(|(interface, abstract_func_prototypes)| -> (IdI<'s, 'i, cI>, InterfaceEdgeBlueprintI<'s, 'i>) {
                    let mut entries: Vec<(&'i PrototypeI<'s, 'i, cI>, i32)> = Vec::new();
                    for (proto, idx) in abstract_func_prototypes.iter() {
                        entries.push((self.interner.alloc(*proto), *idx as i32));
                    }
                    (*interface, InterfaceEdgeBlueprintI { interface: *interface, super_family_root_headers: self.interner.bump().alloc_slice_fill_iter(entries.into_iter()) })
                }),
                self.interner.bump());

        let interfaces: Vec<InterfaceDefinitionI<'s, 'i, cI>> =
            monouts.interfaces_without_methods.values().map(|interface| {
                let crate::instantiating::ast::citizens::InterfaceDefinitionI { instantiated_interface: ref_, attributes, weakable, mutability, .. } = **interface;
                let map = monouts.interface_to_abstract_func_to_virtual_index.get(&ref_.id).expect("vassertSome: interface_to_abstract_func_to_virtual_index");
                let mut methods_entries: Vec<(&'i PrototypeI<'s, 'i, cI>, i32)> = Vec::new();
                for (proto, idx) in map.iter() {
                    methods_entries.push((self.interner.alloc(*proto), *idx as i32));
                }
                crate::instantiating::ast::citizens::InterfaceDefinitionI {
                    instantiated_interface: ref_,
                    attributes,
                    weakable,
                    mutability,
                    rune_to_function_bound: crate::utils::arena_index_map::ArenaIndexMap::new_in(self.interner.bump()),
                    rune_to_impl_bound: crate::utils::arena_index_map::ArenaIndexMap::new_in(self.interner.bump()),
                    internal_methods: self.interner.bump().alloc_slice_fill_iter(methods_entries.into_iter()),
                    _marker: std::marker::PhantomData,
                }
            }).collect();

        let interface_to_sub_citizen_to_edge =
            ArenaIndexMap::from_iter_in(
                monouts.interface_to_impls.iter().map(|(interface, impls)| -> (IdI<'s, 'i, cI>, ArenaIndexMap<'i, IdI<'s, 'i, cI>, EdgeI<'s, 'i>>) {
                    let inner_iter = impls.iter().map(|(_impl_id_t, impl_id_i)| -> (IdI<'s, 'i, cI>, EdgeI<'s, 'i>) {
                        let (sub_citizen, parent_interface, _, _) = monouts.impls.get(impl_id_i).expect("vassertSome: monouts.impls");
                        assert!(parent_interface == interface);
                        let abstract_func_to_virtual_index = monouts.interface_to_abstract_func_to_virtual_index.get(interface).expect("vassertSome: interface_to_abstract_func_to_virtual_index");
                        let abstract_func_prototype_to_override_prototype = abstract_func_to_virtual_index.iter().map(|(_abstract_func_prototype, _virtual_index)| -> (IdI<'s, 'i, cI>, &'i PrototypeI<'s, 'i, cI>) {
                            panic!("translate_method interfaceToSubCitizenToEdge: abstractFuncPrototypeToOverridePrototype branch")
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
                functions: self.interner.alloc_slice_from_vec(monouts.functions.values().copied().collect()),
                interface_to_edge_blueprints: interface_edge_blueprints,
                interface_to_sub_citizen_to_edge,
                kind_exports: self.interner.alloc_slice_from_vec(kind_exports_c),
                function_exports: self.interner.alloc_slice_from_vec(function_exports_c),
                kind_externs: ArenaIndexMap::from_iter_in(
                    monouts.kind_externs.iter().map(|x| -> (&'i StructIT<'s, 'i, cI>, KindExternI<'s, 'i>) {
                        (x.r#struct, *x)
                    }),
                    self.interner.bump()),
                function_externs: self.interner.alloc_slice_from_vec(
                    non_generic_func_externs_c.into_iter().chain(monouts.function_externs.iter().copied()).collect()),
            };
        result_hinputs
    }
}
/*
  def translate():
  HinputsI = {

    val HinputsT(
    interfacesT,
    structsT,
    functionsT,
    //      oldImmKindToDestructorT,
    interfaceToEdgeBlueprintsT,
    interfaceToSubCitizenToEdgeT,
    instantiationNameToFunctionBoundToRuneT,
    kindExportsT,
    functionExportsT,
    kindExternsT,
    functionExternsT) = hinputs

    val kindExportsC =
      kindExportsT.map({ case KindExportT(range, tyype, exportPlaceholderedIdT, exportedName) =>

        val exportIdS =
          translateId[ExportNameT, ExportNameI[sI]](
            exportPlaceholderedIdT,
            { case ExportNameT(ExportTemplateNameT(codeLoc), _) =>
              ExportNameI(ExportTemplateNameI(codeLoc), RegionTemplataI(0))
            })
        val exportIdC =
          RegionCollapserIndividual.collapseExportId(RegionCounter.countExportId(exportIdS), exportIdS)

        val substitutions = assemblePlaceholderMap(exportPlaceholderedIdT, exportIdS)

        val denizenBoundToDenizenCallerSuppliedThing = DenizenBoundToDenizenCallerBoundArgS(Map(),  Map())
        val kindIT =
          translateKind(
            exportPlaceholderedIdT, denizenBoundToDenizenCallerSuppliedThing, substitutions, RegionT(DefaultRegionT), tyype)
        val kindCT = RegionCollapserIndividual.collapseKind(kindIT)

        KindExportI(range, kindCT, exportIdC, exportedName)
      })

    val functionExportsC =
      functionExportsT.map({ case FunctionExportT(range, prototypeT, exportPlaceholderedIdT, exportedName) =>
        val perspectiveRegionT = RegionT(DefaultRegionT)

        val exportIdS =
          translateId[ExportNameT, ExportNameI[sI]](
            exportPlaceholderedIdT,
            { case ExportNameT(ExportTemplateNameT(codeLoc), _) =>
              ExportNameI(ExportTemplateNameI(codeLoc), RegionTemplataI(0))
            })
        val exportIdC =
          RegionCollapserIndividual.collapseExportId(RegionCounter.countExportId(exportIdS), exportIdS)

        val substitutions = assemblePlaceholderMap(exportPlaceholderedIdT, exportIdS)

        val (_, prototypeC) =
          translatePrototype(
            exportPlaceholderedIdT,
            DenizenBoundToDenizenCallerBoundArgS(Map(), Map()),
            substitutions,
            perspectiveRegionT,
            prototypeT)
        Collector.all(prototypeC, { case PlaceholderTemplataT(_, _) => vwat() })
        FunctionExportI(range, prototypeC, exportIdC, exportedName)
      })

    val nonGenericFuncExternsC =
      functionExternsT.flatMap({ case FunctionExternT(range, externPlaceholderedIdT, prototypeT, externedName, maybeInheritance) =>
        val isGeneric = prototypeT.id.localName.templateArgs.nonEmpty
        if (isGeneric) {
          // We don't handle generic externs yet, that comes later when we see what instantiations are actually needed.
          // We handle those like we handle normal non-extern generic functions.
          None
        } else {
          val perspectiveRegionT = RegionT(DefaultRegionT)

          val externIdS =
            translateId[ExternNameT, ExternNameI[sI]](
              externPlaceholderedIdT,
              { case ExternNameT(ExternTemplateNameT(codeLoc), _) =>
                ExternNameI(ExternTemplateNameI(codeLoc), RegionTemplataI(0))
              })
          val externIdC =
            RegionCollapserIndividual.collapseExternId(RegionCounter.countExternId(externIdS), externIdS)

          val substitutions = assemblePlaceholderMap(externPlaceholderedIdT, externIdS)

          val (_, prototypeC) =
            translatePrototype(
              externPlaceholderedIdT,
              DenizenBoundToDenizenCallerBoundArgS(Map(), Map()),
              substitutions,
              perspectiveRegionT,
              prototypeT)
          Collector.all(prototypeC, { case PlaceholderTemplataT(_, _) => vwat() })
          Some(FunctionExternI(prototypeC, maybeInheritance.map(_.numInheritedGenericParameters).getOrElse(0)))
        }
      })

    while ({
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
      if (monouts.newFunctions.nonEmpty) {
        val (newFuncIdT, newFuncIdN, instantiationBoundArgs, maybeDenizenBoundToDenizenCallerSuppliedThing) =
          monouts.newFunctions.dequeue()
        translateFunction(
          opts, interner, keywords, hinputs, monouts, newFuncIdT, newFuncIdN, instantiationBoundArgs,
          maybeDenizenBoundToDenizenCallerSuppliedThing)
        true
      } else if (monouts.newImpls.nonEmpty) {
        val (implIdT, implIdN, instantiationBoundsForUnsubstitutedImpl) =
          monouts.newImpls.dequeue()
        translateImpl(
          opts, interner, keywords, hinputs, monouts, implIdT, implIdN, instantiationBoundsForUnsubstitutedImpl)
        true
      } else if (monouts.newAbstractFuncs.nonEmpty) {
        val (abstractFuncT, abstractFunc, virtualIndex, interfaceId, instantiationBoundArgs) =
          monouts.newAbstractFuncs.dequeue()
        translateAbstractFunc(
          opts, interner, keywords, hinputs, monouts, interfaceId, abstractFuncT, abstractFunc, virtualIndex, instantiationBoundArgs)
        true
      } else {
        false
      }
    }) {}

    //    interfaceToEdgeBlueprints.foreach({ case (interfacePlaceholderedId, edge) =>
    //      val instantiator = new DenizenInstantiator(interner, monouts, interfacePlaceholderedId)
    //
    //    })

    val interfaceEdgeBlueprints =
      monouts.interfaceToAbstractFuncToVirtualIndex.map({ case (interface, abstractFuncPrototypes) =>
        interface -> InterfaceEdgeBlueprintI(interface, abstractFuncPrototypes.toVector)
      }).toMap

    val interfaces =
      monouts.interfacesWithoutMethods.values.map(interface => {
        val InterfaceDefinitionI(ref, attributes, weakable, mutability, _, _, _) = interface
        InterfaceDefinitionI(
          ref, attributes, weakable, mutability, Map(), Map(),
          vassertSome(
            monouts.interfaceToAbstractFuncToVirtualIndex.get(ref.id)).toVector)
      })

    val interfaceToSubCitizenToEdge =
      monouts.interfaceToImpls.map({ case (interface, impls) =>
        interface ->
          impls.map({ case (implIdT, implIdI) =>
            val (subCitizen, parentInterface, _, _) = vassertSome(monouts.impls.get(implIdI))
            vassert(parentInterface == interface)
            val abstractFuncToVirtualIndex =
              vassertSome(monouts.interfaceToAbstractFuncToVirtualIndex.get(interface))
            val abstractFuncPrototypeToOverridePrototype =
              abstractFuncToVirtualIndex.map({ case (abstractFuncPrototype, virtualIndex) =>
                val overrride =
                  vassertSome(
                    vassertSome(
                      vassertSome(monouts.interfaceToImplToAbstractPrototypeToOverride.get(interface))
                        .get(implIdI))
                      .get(abstractFuncPrototype))

                vassert(
                  abstractFuncPrototype.id.localName.parameters(virtualIndex).kind !=
                    overrride.id.localName.parameters(virtualIndex).kind)

                abstractFuncPrototype.id -> overrride
              })

            val edge =
              EdgeI(
                implIdI,
                subCitizen,
                interface,
                Map(),
                Map(),
                abstractFuncPrototypeToOverridePrototype.toMap)
            subCitizen.id -> edge
          }).toMap
      }).toMap

    val resultHinputs =
      HinputsI(
        interfaces.toVector,
        monouts.structs.values.toVector,
        monouts.functions.values.toVector,
        //      monouts.immKindToDestructor.toMap,
        interfaceEdgeBlueprints,
        interfaceToSubCitizenToEdge,
//        Map(),
        kindExportsC,
        functionExportsC,
        monouts.kindExterns.map(x => x.struct -> x).toMap,
        // Non-generic extern functions are translated up-front, before the main instantiating loop.
        // Generic extern functions are translated at their callsites, so we only translate the actually needed
        // instantiations, similar to how normal generic functions work.
        (nonGenericFuncExternsC ++ monouts.functionExterns).toVector)

    resultHinputs
  }
*/
// mig: fn translate_id
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    // Rust adaptation (SPDMX): Scala's translateId[T <: INameT, Y <: INameI[sI]] is generic over the
    // narrow name type, but Rust collapsed IdT/IdI's name param into the wide INameT/INameI enums (see
    // IdI, names.rs:24-28). translate_id mirrors that collapse: `func` takes the wide &INameT and returns
    // the transient INameValI<sI>, which we intern to the permanent INameI. Takes &self for the interner.
    pub fn translate_id(
        &self,
        id_t: &IdT<'s, 't>,
        func: impl Fn(&INameT<'s, 't>) -> INameValI<'s, 'i, sI>,
    ) -> IdI<'s, 'i, sI> {
        let init_steps_i = id_t.init_steps.iter().map(Self::translate_name).collect::<Vec<_>>();
        IdI {
            package_coord: id_t.package_coord,
            init_steps: self.interner.alloc_slice_from_vec(init_steps_i),
            local_name: self.interner.intern_name_si(func(&id_t.local_name)),
        }
    }
}
/*
  def translateId[T <: INameT, Y <: INameI[sI]](idT: IdT[T], func: T => Y): IdI[sI, Y] = {
    val IdT(packageCoord, initStepsT, localNameT) = idT
    IdI[sI, Y](packageCoord, initStepsT.map(translateName(_)), func(localNameT))
  }
*/
// mig: fn translate_export_name
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_export_name(_denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _export_name_t: &ExportNameT<'s, 't>) -> ExportNameI<'s, 'i, sI> {
        panic!("Unimplemented: translate_export_name");
    }
}
/*
  def translateExportName(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    exportNameT: ExportNameT):
  ExportNameI[sI] = {
    val ExportNameT(ExportTemplateNameT(codeLoc), _) = exportNameT
    ExportNameI(
      ExportTemplateNameI(codeLoc),
      RegionTemplataI(0))
  }
*/
// mig: fn translate_export_template_name
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_export_template_name(_export_template_name_t: &ExportTemplateNameT<'s, 't>) -> ExportTemplateNameI<'s, 'i, sI> {
        panic!("Unimplemented: translate_export_template_name");
    }
}
/*
  def translateExportTemplateName(exportTemplateNameT: ExportTemplateNameT): ExportTemplateNameI[sI] = {
    val ExportTemplateNameT(codeLoc) = exportTemplateNameT
    ExportTemplateNameI(codeLoc)
  }
*/
// mig: fn translate_name
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_name(_t: &INameT<'s, 't>) -> INameI<'s, 'i, sI> {
        panic!("Unimplemented: translate_name");
    }
}
/*
  def translateName(t: INameT): INameI[sI] = {
    vimpl()
  }
*/
// mig: fn collapse_and_translate_interface_definition
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn collapse_and_translate_interface_definition(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _interface_id_t: &IdT<'s, 't>, _interface_id_s: &IdI<'s, 'i, sI>, _instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) {
        // Scala's `if (opts.sanityCheck) { vassert(Collector.all(..., {case KindPlaceholderNameT(_) => }).isEmpty) }`
        // sanity check is omitted: same architect-approved parity gap as collapse_and_translate_struct_definition.
        let interface_def_t = self.find_interface(_interface_id_t);
        let denizen_bound_to_denizen_caller_supplied_thing = Self::assemble_instantiation_bound_param_to_arg(&interface_def_t.instantiation_bound_params, _instantiation_bound_args);
        if let Some(x) = _monouts.interface_to_bounds.get(_interface_id_s) {
            assert!(*x == denizen_bound_to_denizen_caller_supplied_thing, "vcurious: interface_to_bounds mismatch");
        }
        _monouts.interface_to_bounds.insert(*_interface_id_s, denizen_bound_to_denizen_caller_supplied_thing.clone());
        let substitutions = self.assemble_placeholder_map(&interface_def_t.instantiated_interface.id, _interface_id_s);
        let interface_id_c = crate::instantiating::region_collapser_individual::collapse_interface_id(self.interner, _interface_id_s);
        self.translate_collapsed_interface_definition(_monouts, _interface_id_t, &denizen_bound_to_denizen_caller_supplied_thing, &substitutions, &interface_id_c, interface_def_t);
    }
}
/*
Guardian: temp-disable: SPDMX — Architect-approved parity gap (used twice in this file already, e.g. in collapse_and_translate_struct_definition and translate_method's function_exports_c/function_externs_c closures): Scala's KindPlaceholderNameT collector check is vacuous on the I-side because INameI has no KindPlaceholder variant. Same precedent. The Some(_x) vcurious is also a debug-only sanity check (matches Scala vcurious semantics). — /Volumes/V/Vale/FrontendRust/guardian-logs/request-2433-1780135108211/hook-2433/collapse_and_translate_interface_definition--746.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def collapseAndTranslateInterfaceDefinition(
    opts: GlobalOptions,
    interner: Interner,
    keywords: Keywords,
    hinputs: HinputsT,
    monouts: InstantiatedOutputs,
    interfaceIdT: IdT[IInterfaceNameT],
    interfaceIdS: IdI[sI, IInterfaceNameI[sI]],
    instantiationBoundArgs: InstantiationBoundArgumentsI):
  Unit = {

    if (opts.sanityCheck) {
      vassert(Collector.all(interfaceIdS, { case KindPlaceholderNameT(_) => }).isEmpty)
    }

    val interfaceTemplateIdT = TemplataCompiler.getInterfaceTemplate(interfaceIdT)

    val interfaceDefT = findInterface(hinputs, interfaceIdT)

    val denizenBoundToDenizenCallerSuppliedThing =
      assembleInstantiationBoundParamToArg(
        interfaceDefT.instantiationBoundParams,
        instantiationBoundArgs)
    monouts.interfaceToBounds.get(interfaceIdS) match {
      case Some(x) => {
        vcurious(x == denizenBoundToDenizenCallerSuppliedThing)
      }
      case None =>
    }
    monouts.interfaceToBounds.put(interfaceIdS, denizenBoundToDenizenCallerSuppliedThing)

    val substitutions =
      assemblePlaceholderMap(
        // One would imagine we'd get interfaceId.last.templateArgs here, because that's the interface
        // we're about to monomorphize. However, only the top level denizen has placeholders, see LHPCTLD.
        // This interface might not be the top level denizen, such as if it's a lambda.
        // TODO(regions): might be obsolete?
        interfaceDefT.instantiatedCitizen.id,
        interfaceIdS)
    //    val instantiator =
    //      new Instantiator(
    //        opts,
    //        interner,
    //        keywords,
    //        hinputs,
    //        monouts,
    //        interfaceTemplate,
    //        interfaceIdT,
    //        denizenBoundToDenizenCallerSuppliedThing)
    val interfaceIdC =
      RegionCollapserIndividual.collapseInterfaceId(interfaceIdS)

    translateCollapsedInterfaceDefinition(
      interfaceIdT, denizenBoundToDenizenCallerSuppliedThing, substitutions, interfaceIdC, interfaceDefT)
  }
*/
// mig: fn assemble_instantiation_bound_param_to_arg
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn assemble_instantiation_bound_param_to_arg(instantiation_bound_params: &InstantiationBoundArgumentsT<'s, 't>, instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) -> DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i> {
        assert!(instantiation_bound_args.rune_to_function_bound_arg.len() == instantiation_bound_params.rune_to_bound_prototype.len());
        assert!(
            instantiation_bound_args.caller_rune_to_callee_rune_to_reachable_func.iter().filter(|(_, v)| !v.is_empty()).count() ==
                instantiation_bound_params.rune_to_citizen_rune_to_reachable_prototype.iter().filter(|(_, v)| !v.citizen_rune_to_reachable_prototype.is_empty()).count());
        assert!(instantiation_bound_args.rune_to_impl_bound_arg.len() == instantiation_bound_params.rune_to_bound_impl.len());
        DenizenBoundToDenizenCallerBoundArgI {
            func_id_to_bound_arg_prototype:
                instantiation_bound_args.rune_to_function_bound_arg.iter().map(|(callee_rune, supplied_function_i)| -> (IdT<'s, 't>, &'i PrototypeI<'s, 'i, sI>) {
                    (instantiation_bound_params.rune_to_bound_prototype.get(callee_rune).expect("vassertSome: rune_to_bound_prototype").id, *supplied_function_i)
                }).chain(
                    instantiation_bound_args.caller_rune_to_callee_rune_to_reachable_func.iter().flat_map(|(_caller_rune, _callee_rune_to_reachable_func)| -> Vec<(IdT<'s, 't>, &'i PrototypeI<'s, 'i, sI>)> {
                        panic!("Unimplemented: assemble_instantiation_bound_param_to_arg callerRuneToCalleeRuneToReachableFunc")
                    })
                ).collect(),
            bound_param_impl_id_to_bound_arg_impl_id:
                instantiation_bound_args.rune_to_impl_bound_arg.iter().map(|(_callee_rune, _supplied_impl_t)| -> (IdT<'s, 't>, IdI<'s, 'i, sI>) {
                    panic!("Unimplemented: assemble_instantiation_bound_param_to_arg runeToImplBoundArg")
                }).collect(),
        }
    }
}
/*
  def assembleInstantiationBoundParamToArg(
      instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT],
      instantiationBoundArgs: InstantiationBoundArgumentsI): DenizenBoundToDenizenCallerBoundArgS = {
    vassert(instantiationBoundArgs.runeToFunctionBoundArg.size == instantiationBoundParams.runeToBoundPrototype.size)
    vassert(
      instantiationBoundArgs.callerRuneToCalleeRuneToReachableFunc.count(_._2.nonEmpty) ==
          instantiationBoundParams.runeToCitizenRuneToReachablePrototype.count(_._2.citizenRuneToReachablePrototype.nonEmpty))
    vassert(instantiationBoundArgs.runeToImplBoundArg.size == instantiationBoundParams.runeToBoundImpl.size)
    DenizenBoundToDenizenCallerBoundArgS(
      instantiationBoundArgs.runeToFunctionBoundArg.map({ case (calleeRune, suppliedFunctionI) =>
        vassertSome(instantiationBoundParams.runeToBoundPrototype.get(calleeRune)).id -> suppliedFunctionI
      }) ++
      instantiationBoundArgs.callerRuneToCalleeRuneToReachableFunc.flatMap({ case (callerRune, calleeRuneToReachableFunc) =>
        if (calleeRuneToReachableFunc.nonEmpty) {
          val m = vassertSome(instantiationBoundParams.runeToCitizenRuneToReachablePrototype.get(callerRune))
          vassert(m.citizenRuneToReachablePrototype.size == calleeRuneToReachableFunc.size)
          calleeRuneToReachableFunc.map({ case (calleeRune, reachableFuncI) =>
            val reachableFuncT = vassertSome(m.citizenRuneToReachablePrototype.get(calleeRune))
            reachableFuncT.id -> reachableFuncI
          })
        } else {
          List()
        }
      }),
      instantiationBoundArgs.runeToImplBoundArg.map({ case (calleeRune, suppliedImplT) =>
        vassertSome(instantiationBoundParams.runeToBoundImpl.get(calleeRune)) -> suppliedImplT
      }))
  }
*/
// mig: fn assemble_callee_denizen_function_bounds
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn assemble_callee_denizen_function_bounds(_callee_rune_to_receiver_bound_t: &IndexMap<IRuneS<'s>, IdT<'s, 't>>, _callee_rune_to_supplied_prototype: &IndexMap<IRuneS<'s>, PrototypeI<'s, 'i, sI>>) -> IndexMap<IdT<'s, 't>, PrototypeI<'s, 'i, sI>> {
        panic!("Unimplemented: assemble_callee_denizen_function_bounds");
    }
}
/*
  def assembleCalleeDenizenFunctionBounds(
    // This is from the receiver's perspective, they have some runes for their required functions.
    calleeRuneToReceiverBoundT: Map[IRuneS, IdT[FunctionBoundNameT]],
    // This is a map from the receiver's rune to the bound that the caller is supplying.
    calleeRuneToSuppliedPrototype: Map[IRuneS, PrototypeI[sI]]
  ): Map[IdT[FunctionBoundNameT], PrototypeI[sI]] = {
    calleeRuneToSuppliedPrototype.map({ case (calleeRune, suppliedFunctionT) =>
      vassertSome(calleeRuneToReceiverBoundT.get(calleeRune)) -> suppliedFunctionT
    })
  }
*/
// mig: fn assemble_callee_denizen_impl_bounds
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn assemble_callee_denizen_impl_bounds(_callee_rune_to_receiver_bound_t: &IndexMap<IRuneS<'s>, IdT<'s, 't>>, _callee_rune_to_supplied_impl: &IndexMap<IRuneS<'s>, IdI<'s, 'i, sI>>) -> IndexMap<IdT<'s, 't>, IdI<'s, 'i, sI>> {
        panic!("Unimplemented: assemble_callee_denizen_impl_bounds");
    }
}
/*
  def assembleCalleeDenizenImplBounds(
    // This is from the receiver's perspective, they have some runes for their required functions.
    calleeRuneToReceiverBoundT: Map[IRuneS, IdT[ImplBoundNameT]],
    // This is a map from the receiver's rune to the bound that the caller is supplying.
    calleeRuneToSuppliedImpl: Map[IRuneS, IdI[sI, IImplNameI[sI]]]
  ): Map[IdT[ImplBoundNameT], IdI[sI, IImplNameI[sI]]] = {
    calleeRuneToSuppliedImpl.map({ case (calleeRune, suppliedFunctionT) =>
      vassertSome(calleeRuneToReceiverBoundT.get(calleeRune)) -> suppliedFunctionT
    })
  }
*/
// mig: fn collapse_and_translate_struct_definition
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn collapse_and_translate_struct_definition(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _struct_id_t: &IdT<'s, 't>, _struct_id_s: &IdI<'s, 'i, sI>, _instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) {
        // Scala's `if (opts.sanityCheck) { vassert(Collector.all(structIdS, {case KindPlaceholderNameT(_) => }).isEmpty) }`
        // sanity check is omitted: the I-side AST is statically typed and can't structurally hold a
        // typing-pass KindPlaceholder name, so it's vacuously empty. (Architect-approved parity gap.)
        let struct_def_t = self.find_struct(_struct_id_t);
        let denizen_bound_to_denizen_caller_supplied_thing =
            Self::assemble_instantiation_bound_param_to_arg(&struct_def_t.instantiation_bound_params, _instantiation_bound_args);
        match _monouts.struct_to_bounds.get(_struct_id_s) {
            Some(_x) => {
                return;
            }
            None => {}
        }
        _monouts.struct_to_bounds.insert(*_struct_id_s, denizen_bound_to_denizen_caller_supplied_thing.clone());
        let substitutions = self.assemble_placeholder_map(&struct_def_t.instantiated_citizen.id, _struct_id_s);
        let struct_id_c = crate::instantiating::region_collapser_individual::collapse_struct_id(self.interner, _struct_id_s);
        self.translate_collapsed_struct_definition(_monouts, _struct_id_t, &denizen_bound_to_denizen_caller_supplied_thing, &substitutions, _struct_id_t, &struct_id_c, struct_def_t);
    }
}
/*
  def collapseAndTranslateStructDefinition(
    opts: GlobalOptions,
    interner: Interner,
    keywords: Keywords,
    hinputs: HinputsT,
    monouts: InstantiatedOutputs,
    structIdT: IdT[IStructNameT],
    structIdS: IdI[sI, IStructNameI[sI]],
    instantiationBoundArgs: InstantiationBoundArgumentsI):
  Unit = {
    if (opts.sanityCheck) {
      vassert(Collector.all(structIdS, { case KindPlaceholderNameT(_) => }).isEmpty)
    }

    val structTemplate = TemplataCompiler.getStructTemplate(structIdT)

    val structDefT = findStruct(hinputs, structIdT)

    val denizenBoundToDenizenCallerSuppliedThing =
      assembleInstantiationBoundParamToArg(structDefT.instantiationBoundParams, instantiationBoundArgs)
      // DenizenBoundToDenizenCallerBoundArgS(
      //   assembleCalleeDenizenFunctionBounds(
      //     .runeToFunctionBoundArg.mapValues(_.id), instantiationBoundArgs.runeToFunctionBoundArg),
      //   assembleCalleeDenizenImplBounds(
      //     structDefT.instantiationBoundParams.runeToImplBoundArg, instantiationBoundArgs.runeToImplBoundArg))
    monouts.structToBounds.get(structIdS) match {
      case Some(x) => {
        vcurious(x == denizenBoundToDenizenCallerSuppliedThing)
        return
      }
      case None =>
    }
    monouts.structToBounds.put(structIdS, denizenBoundToDenizenCallerSuppliedThing)

    val substitutions =
      assemblePlaceholderMap(
        // One would imagine we'd get structId.last.templateArgs here, because that's the struct
        // we're about to monomorphize. However, only the top level denizen has placeholders, see LHPCTLD.
        // This struct might not be the top level denizen, such as if it's a lambda.
        // TODO(regions): might be obsolete?
        structDefT.instantiatedCitizen.id,
        structIdS)
//    val instantiator =
//      new Instantiator(
//        opts,
//        interner,
//        keywords,
//        hinputs,
//        monouts,
//        structTemplate,
//        structIdT,
//        denizenBoundToDenizenCallerSuppliedThing)
    val structIdC =
      RegionCollapserIndividual.collapseStructId(structIdS)
    translateCollapsedStructDefinition(
      structIdT, denizenBoundToDenizenCallerSuppliedThing, substitutions, structIdT, structIdC, structDefT)
  }
*/
// mig: fn find_struct
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn find_struct(&self, _struct_id: &IdT<'s, 't>) -> &'t StructDefinitionT<'s, 't> {
        let target = crate::typing::compiler::Compiler::get_super_template(self.typing_interner, *_struct_id);
        let matches: Vec<_> = self.hinputs.structs.iter().filter(|s| crate::typing::compiler::Compiler::get_super_template(self.typing_interner, s.instantiated_citizen.id) == target).collect();
        assert_eq!(matches.len(), 1);
        matches[0]
    }
}
/*
  private def findStruct(hinputs: HinputsT, structId: IdT[IStructNameT]) = {
    vassertOne(
      hinputs.structs
        .filter(structT => {
          TemplataCompiler.getSuperTemplate(structT.instantiatedCitizen.id) ==
            TemplataCompiler.getSuperTemplate(structId)
        }))
  }
*/
// mig: fn find_interface
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn find_interface(&self, _interface_id: &IdT<'s, 't>) -> &'t InterfaceDefinitionT<'s, 't> {
        let target = crate::typing::compiler::Compiler::get_super_template(self.typing_interner, *_interface_id);
        let matches: Vec<_> = self.hinputs.interfaces.iter().filter(|i| crate::typing::compiler::Compiler::get_super_template(self.typing_interner, i.instantiated_interface.id) == target).collect();
        assert_eq!(matches.len(), 1);
        matches[0]
    }
}
/*
Guardian: temp-disable: SPDMX — Matches the find_struct precedent (already landed in this file) — Scala vassertOne ports as assert_eq!(len, 1) + indexing. Both find_struct and find_interface are the same shape; if SPDMX is satisfied for find_struct it should be for find_interface too. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-2457-1780135852835/hook-2457/find_interface--1009.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  private def findInterface(hinputs: HinputsT, interfaceId: IdT[IInterfaceNameT]) = {
    vassertOne(
      hinputs.interfaces
        .filter(interfaceT => {
          TemplataCompiler.getSuperTemplate(interfaceT.instantiatedCitizen.id) ==
            TemplataCompiler.getSuperTemplate(interfaceId)
        }))
  }
*/
// mig: fn find_impl
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn find_impl(&self, _impl_id: &IdT<'s, 't>) -> &'t EdgeT<'s, 't> {
        panic!("Unimplemented: find_impl");
    }
}
/*
  private def findImpl(hinputs: HinputsT, implId: IdT[IImplNameT]): EdgeT = {
    vassertOne(
      hinputs.interfaceToSubCitizenToEdge.values.flatMap(subCitizenToEdge => {
        subCitizenToEdge.values.filter(edge => {
          TemplataCompiler.getSuperTemplate(edge.edgeId) ==
              TemplataCompiler.getSuperTemplate(implId)
        })
      }))
  }
*/
// mig: fn translate_override
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_override(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _impl_id_t: &IdT<'s, 't>, _impl_id_c: &IdI<'s, 'i, cI>, _abstract_func_prototype_t: &PrototypeT<'s, 't>, _abstract_func_prototype_c: &PrototypeI<'s, 'i, cI>, _abstract_func_instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) {
        panic!("Unimplemented: translate_override");
    }
}
/*
  def translateOverride(
    opts: GlobalOptions,
    interner: Interner,
    keywords: Keywords,
    hinputs: HinputsT,
    monouts: InstantiatedOutputs,
    implIdT: IdT[IImplNameT],
    implIdC: IdI[cI, IImplNameI[cI]],
    abstractFuncPrototypeT: PrototypeT[IFunctionNameT],
    abstractFuncPrototypeC: PrototypeI[cI],
    abstractFuncInstantiationBoundArgs: InstantiationBoundArgumentsI):
  Unit = {
    // Our ultimate goal in here is to make a PrototypeI[cI] for the override.
    // To do that, we're going to compile a dispatcher function given an impl, see CDFGI.
    //
    // For example:
    //
    //   abstract func launch<X, Y, Z>(self &ISpaceship<X, Y, Z>, bork X) where exists drop(Y)void; // Known as "abst"
    //   struct Raza<A, B, C> { ... }
    //   impl<I, J> ISpaceship<int, I, J> for Raza<I, J>; // This impl known as "ri"
    //   func launch<Y, Z>(self &Raza<Y, Z>, bork int) where exists drop(Y)void { ... } // Known as "over"
    //
    // we're going to pretend that there's instead a "dispatcher" function that's just forwarding calls based on the
    // type of self:
    //
    //   func dispatcher<int, str, bool>(self &ISpaceship<int, str, bool>, bork int) where exists drop(str)void {
    //     match self {
    //       raza &Raza<Y, Z> => launch(raza, bork)
    //       ...
    //     }
    //   }
    //
    // this dispatcher function is also known as "dis". Note how we know the concrete types right now (int, str, bool),
    // that's because we're in the instantiator and we know those.
    // Our ultimate goal is to find (and instantiate) the prototype for that launch(raza, bork) in there.

    // First step: gather a bunch of details about the given impl, super interface (ISpaceship), sub citizen (Raza)
    // and the abstract function (virtual func launch).
    val implTemplateId = TemplataCompiler.getImplTemplate(implIdT)
    val edgeT =
      vassertOne(
        hinputs.interfaceToSubCitizenToEdge
          .flatMap(_._2.values)
          .filter(edge => TemplataCompiler.getImplTemplate(edge.edgeId) == implTemplateId))
    val EdgeT(
      edgeId,
      edgeSubCitizen,
      edgeSuperInterface,
      _,
      edgeAbstractFuncToOverrideFunc
    ) = edgeT

    val abstractFuncTemplateName = TemplataCompiler.getFunctionTemplate(abstractFuncPrototypeT.id)
    val abstractFuncPlaceholderedNameT =
      vassertSome(
        hinputs.functions
          .find(func => TemplataCompiler.getFunctionTemplate(func.header.id) == abstractFuncTemplateName))
        .header.id

    // Luckily, the typing phase knows what the override is.
    // In this example, it's func launch<Y, Z>(self &Raza<Y, Z>, bork int)
    // We just have to instantiate it, given that someone called the abstract function with certain known types.
    // If they called launch(&ISpaceship<int, str, bool>, int) then we know:
    // - abst$A = int
    // - abst$B = str
    // - abst$C = bool
    // But we need to know over$Y and over$Z.

    val OverrideT(
        dispatcherIdT,
        implPlaceholderToDispatcherPlaceholder,
        implPlaceholderToCasePlaceholder,
        dispatcherAndCasePlaceholderedImplReachablePrototypes,
        dispatcherCaseIdT,
        overridePrototypeT,
        dispatcherInstantiationBoundParams) =
      vassertSome(edgeAbstractFuncToOverrideFunc.get(abstractFuncPlaceholderedNameT))
    val dispatcherTemplateId = TemplataCompiler.getTemplate(dispatcherIdT)

    // We currently know the abstract function's caller's runes and how they map to the instantiated values,
    // - abst$A = int
    // - abst$B = str
    // - abst$C = bool
    // ...but this dispatcher function is different than the abstract function. The dispatcher function has its own
    // runes, here:
    // - dis$I
    // - dis$J
    // So we'll map the abstract function's caller's runes to the dispatcher's runes.
    // TODO: Feels like these can be simplified somehow...
    // Per @PASDZ, the dispatcher's placeholders are named under its own template denizen,
    // distinct from the impl's denizen below.
    val dispatcherPlaceholderIdToSuppliedTemplata =
      dispatcherIdT.localName.templateArgs
        .map(dispatcherPlaceholderTemplata => {
          val dispatcherPlaceholderId =
            TemplataCompiler.getPlaceholderTemplataId(dispatcherPlaceholderTemplata)
          val implPlaceholder =
            vassertSome(
              // This implPlaceholderToDispatcherPlaceholder has a map of the impl runes to the dispatcher runes, like:
              // - ri$I -> dis$I
              // - ri$J -> dis$J
              implPlaceholderToDispatcherPlaceholder
                  .find(_._2 == dispatcherPlaceholderTemplata))._1
          val index =
            implPlaceholder match {
              case IdT(_, _, KindPlaceholderNameT(KindPlaceholderTemplateNameT(index, rune))) => index
              case IdT(_, _, NonKindNonRegionPlaceholderNameT(index, rune)) => index
              case other => vwat(other)
            }
          // Here we're grabbing it from the instantiated impl that we're overriding, here ri<bool, str>.
          val templataC = implIdC.localName.templateArgs(index)
          // This is a collapsed, but it needs to be subjective from this dispatcher's perspective.

          // TODO(regions): Figure out how to turn this into an sI.
          dispatcherPlaceholderId -> vregionmut(templataC.asInstanceOf[ITemplataI[sI]])
        })
    // In this case we'll end up with:
    //   dis/dis$I -> bool
    //   dis/dis$J -> str
    // However, such as in the Milano case, there might be some independent runes that we need to
    // figure out how to supply. We'll conceptually grab these from the receiving struct.
    // The impl knows the receiving struct.
    // Per @PASDZ, these case-specific runes are named under the impl's denizen — a different
    // path prefix than the dispatcher's, so the case map and the dispatcher map are disjoint
    // and can be ++-merged below.
    val dispatcherCasePlaceholderIdToSuppliedTemplata =
      dispatcherCaseIdT.localName.independentImplTemplateArgs.zipWithIndex.map({
        case (casePlaceholderTemplata, index) => {
          val casePlaceholderId =
            TemplataCompiler.getPlaceholderTemplataId(casePlaceholderTemplata)
          val implPlaceholder =
            vassertSome(
              implPlaceholderToCasePlaceholder.find(_._2 == casePlaceholderTemplata))._1
          val IdT(_, _, KindPlaceholderNameT(KindPlaceholderTemplateNameT(index, rune))) = implPlaceholder
          val templata = implIdC.localName.templateArgs(index)
          // TODO(regions): Figure out how to turn this into an sI.
          casePlaceholderId -> vregionmut(templata.asInstanceOf[ITemplataI[sI]])
        }
      })
    val dispatcherPlaceholderIdToSuppliedTemplataMap = dispatcherPlaceholderIdToSuppliedTemplata.toMap
    val dispatcherCasePlaceholderIdToSuppliedTemplataMap = dispatcherCasePlaceholderIdToSuppliedTemplata.toMap
    // Per @PASDZ, the dispatcher's placeholders and the case-impl's independent placeholders
    // belong to different denizens, so their full path-encoded ids are disjoint by construction
    // and we can ++-merge them into one flat substitutions map. This assertion is the runtime
    // check that the disjointness actually holds.
    vassert(
      (dispatcherPlaceholderIdToSuppliedTemplataMap ++ dispatcherCasePlaceholderIdToSuppliedTemplataMap).size ==
        dispatcherPlaceholderIdToSuppliedTemplataMap.size + dispatcherCasePlaceholderIdToSuppliedTemplataMap.size)

    val caseSubstitutions =
      dispatcherPlaceholderIdToSuppliedTemplataMap ++
        dispatcherCasePlaceholderIdToSuppliedTemplataMap

    // Now that we have the values for the dispatcher placeholders, let's get the values for the function/impl bounds.

    // The instantiator's next step for an override is to bring in some bound functions from the impl that already
    // exists.
    //
    // Let's say we had this:
    //   interface IObserver<W> { }
    //   abstract func handleLaunch<X>(self virtual &IObserver<LaunchEvent<X>>);
    //   struct Firefly<Y> where exists drop(T)void { }
    //   impl<Z> IObserver<LaunchEvent<Z>> for Firefly<Engine<Z>>; // inherits drop(Engine<Z>)
    //   func handleLaunch<T>(self &Ship<LaunchEvent<T>>) { ... }
    // What we need is the sub citizen for this interface, in terms of the abstract function.
    //
    // To do that, the first step was to pretend we're compiling the abstract function, like so:
    //   func handleLaunch<launch$X>(self &IObserver<LaunchEvent<launch$X>>) { ... }
    // and have it feed the interface self param into the impl to make it resolve the struct, like so:
    //   impl<Z> (IObserver<LaunchEvent<launch$X>> = IObserver<LaunchEvent<Z>>) for Firefly<Engine<Z>>;
    // which solved for Firefly<Engine<launch$X>> and predicts that some bounds should exist:
    // - ZD = func impl.predicted:drop(Firefly<Engine<$launchX>>);
    //
    // At this point, we need to conjure some bounds from that, knowing that the instantiator can fill the actual ones
    // from the impl.
    // We *could* use the PredictedFunctionNameT that come out of the solver, buuut let's not. Let's turn it from:
    // - ZD = func impl                   .predicted:drop(Firefly<Engine<$launchX>>);
    // into:
    // - ZD = func dispatcher:handleLaunch.bound:drop    (Firefly<Engine<$launchX>>);
    // remembering the impl rune it came from.

    // Grab the actual instantiated bounds that were used to make the impl.
    val implRuneToImplInstantiationBoundArgs = vassertSome(monouts.impls.get(implIdC))._4
    val boundParamPrototypeTToBoundArgPrototypeIFromImpl =
      // This is how the typing phase referred to the impl's bound prototypes.
      // We're making a map from those names to the actual prototypes the impl was instantiated with.
      dispatcherAndCasePlaceholderedImplReachablePrototypes.toVector.flatMap({
        case (runeInImpl, citizenRuneToBound) => {
          citizenRuneToBound.toVector.map({
            case (runeInCitizen, prototypeT@PrototypeT(IdT(_, _, FunctionBoundNameT(FunctionBoundTemplateNameT(_), _, _)), _)) => {
              val prototypeI =
                vassertSome(
                  vassertSome(
                    implRuneToImplInstantiationBoundArgs.callerRuneToCalleeRuneToReachableFunc
                        .get(runeInImpl))
                      .get(runeInCitizen))
              prototypeT.id -> prototypeI
            }
          })
        }
      })
          .toMap
    // TODO: Catch impls up

    val dispatcherInstantiationBoundParamsToArgs =
    // Here we're matching up the runes of the callsite's instantiation bound args with the
    // runes of the abstract function definition's instantiation bound params.
      assembleInstantiationBoundParamToArg(
        dispatcherInstantiationBoundParams,
        abstractFuncInstantiationBoundArgs)

    // Here we're adding in any bounds that the struct/impl know about that the call site might not know about.
    val caseInstantiationBoundParamsToArgs =
      dispatcherInstantiationBoundParamsToArgs
          .plus(
            DenizenBoundToDenizenCallerBoundArgS(
              boundParamPrototypeTToBoundArgPrototypeIFromImpl,
              Map())) // TODO: Catch impls up

    val (overridePrototypeS, overridePrototypeC) =
      translatePrototype(
        dispatcherCaseIdT,
        caseInstantiationBoundParamsToArgs,
        caseSubstitutions,
        RegionT(DefaultRegionT),
        overridePrototypeT)

    val superInterfaceId = vassertSome(monouts.impls.get(implIdC))._2

    monouts.addMethodToVTable(implIdC, superInterfaceId, abstractFuncPrototypeC, overridePrototypeC)
  }
*/
// mig: fn translate_impl
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_impl(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _impl_id_t: &IdT<'s, 't>, _impl_id_n: &IdI<'s, 'i, nI>, _instantiation_bounds_for_unsubstituted_impl: InstantiationBoundArgumentsI<'s, 'i>) {
        // This works because the sI/cI are never actually used in these instances, they are just a
        // compile-time type-system bit of tracking, see CCFCTS.
        let impl_id_s: &IdI<'s, 'i, sI> = unsafe { &*(_impl_id_n as *const IdI<'s, 'i, nI> as *const IdI<'s, 'i, sI>) };
        let impl_id_c = crate::instantiating::region_collapser_individual::collapse_impl_id(self.interner, impl_id_s);

        let impl_template_id = Compiler::get_impl_template(self.typing_interner, *_impl_id_t);
        let impl_definition = vassert_one(self.hinputs.interface_to_sub_citizen_to_edge.iter().flat_map(|(_, m)| m.values()).filter(|edge| {
            Compiler::get_impl_template(self.typing_interner, edge.edge_id) == impl_template_id
        }));

        let denizen_bound_to_denizen_caller_supplied_thing = Self::assemble_instantiation_bound_param_to_arg(&impl_definition.instantiation_bound_params, &_instantiation_bounds_for_unsubstituted_impl);
        let substitutions = self.assemble_placeholder_map(&impl_definition.edge_id, impl_id_s);
        self.translate_collapsed_impl_definition(_monouts, _impl_id_t, _instantiation_bounds_for_unsubstituted_impl, &denizen_bound_to_denizen_caller_supplied_thing, &substitutions, _impl_id_t, impl_id_s, &impl_id_c, impl_definition);
    }
}
/*
  def translateImpl(
    opts: GlobalOptions,
    interner: Interner,
    keywords: Keywords,
    hinputs: HinputsT,
    monouts: InstantiatedOutputs,
    implIdT: IdT[IImplNameT],
    implIdN: IdI[nI, IImplNameI[nI]],
    instantiationBoundsForUnsubstitutedImpl: InstantiationBoundArgumentsI):
  Unit = {
    // This works because the sI/cI are never actually used in these instances, they are just a
    // compile-time type-system bit of tracking, see CCFCTS.
    val implIdS: IdI[sI, IImplNameI[sI]] = implIdN
    val implIdC = RegionCollapserIndividual.collapseImplId(implIdS)

    val implTemplateId = TemplataCompiler.getImplTemplate(implIdT)
    val implDefinition =
      vassertOne(
        hinputs.interfaceToSubCitizenToEdge
          .flatMap(_._2.values)
          .filter(edge => {
            //TemplataCompiler.getSuperTemplate(edge.edgeId) == TemplataCompiler.getSuperTemplate(implTemplateId), doesnt fix it
            TemplataCompiler.getImplTemplate(edge.edgeId) == implTemplateId
          }))

    val denizenBoundToDenizenCallerSuppliedThing =
      assembleInstantiationBoundParamToArg(
        implDefinition.instantiationBoundParams,
        instantiationBoundsForUnsubstitutedImpl)

    val substitutions = assemblePlaceholderMap(implDefinition.edgeId, implIdS)
    translateCollapsedImplDefinition(
      implIdT,
      instantiationBoundsForUnsubstitutedImpl,
      denizenBoundToDenizenCallerSuppliedThing,
      substitutions,
      implIdT,
      implIdS,
      implIdC,
      implDefinition)
  }
*/
// mig: fn translate_function
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_function(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, desired_prototype_t: &PrototypeT<'s, 't>, desired_prototype_n: &PrototypeI<'s, 'i, nI>, _supplied_bound_args: &InstantiationBoundArgumentsI<'s, 'i>, _maybe_denizen_bound_to_denizen_caller_supplied_thing: Option<&DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>>) -> &'i FunctionDefinitionI<'s, 'i> {
        // This works because the sI/cI are never actually used in these instances, they are just a
        // compile-time type-system bit of tracking, see CCFCTS.
        let desired_prototype_s: &PrototypeI<'s, 'i, sI> = desired_prototype_n;
        let desired_prototype_c =
            region_collapser_individual::collapse_prototype(self.interner, desired_prototype_s);

        let desired_func_super_template_name = Compiler::get_super_template(self.typing_interner, desired_prototype_t.id);
        let func_t =
            vassert_one(self.hinputs.functions.iter().filter(|func_t| {
                Compiler::get_super_template(self.typing_interner, func_t.header.id) == desired_func_super_template_name
            }));

        let denizen_bound_to_denizen_caller_supplied_thing_from_denizen_itself =
            match _maybe_denizen_bound_to_denizen_caller_supplied_thing {
                Some(x) => x.clone(),
                None => Self::assemble_instantiation_bound_param_to_arg(&func_t.instantiation_bound_params, _supplied_bound_args),
            };
        let _args_m: Vec<_> = IFunctionNameI::try_from(desired_prototype_s.id.local_name).unwrap().parameters().iter().map(|c| c.kind).collect();
        let _params_t: Vec<_> = func_t.header.params.iter().map(|p| p.tyype.kind).collect();

        let denizen_bound_to_denizen_caller_supplied_thing_from_denizen_itself_and_params =
            denizen_bound_to_denizen_caller_supplied_thing_from_denizen_itself;

        let denizen_bound_to_denizen_caller_supplied_thing =
            denizen_bound_to_denizen_caller_supplied_thing_from_denizen_itself_and_params;

        let substitutions =
            self.assemble_placeholder_map(&func_t.header.id, &desired_prototype_s.id);

        let monomorphized_func_t =
            self.translate_collapsed_function(
                monouts, &desired_prototype_t.id, &denizen_bound_to_denizen_caller_supplied_thing, &substitutions, &desired_prototype_c, func_t);

        assert!(desired_prototype_c.return_type == monomorphized_func_t.header.return_type);

        monomorphized_func_t
    }
}
/*
  def translateFunction(
    opts: GlobalOptions,
    interner: Interner,
    keywords: Keywords,
    hinputs: HinputsT,
    monouts: InstantiatedOutputs,
    desiredPrototypeT: PrototypeT[IFunctionNameT],
    desiredPrototypeN: PrototypeI[nI],
    suppliedBoundArgs: InstantiationBoundArgumentsI,
    // This is only Some if this is a lambda. This will contain the prototypes supplied to the top
    // level denizen by its own caller, see LCNBAFA.
    maybeDenizenBoundToDenizenCallerSuppliedThing: Option[DenizenBoundToDenizenCallerBoundArgS]):
  FunctionDefinitionI = {
    // This works because the sI/cI are never actually used in these instances, they are just a
    // compile-time type-system bit of tracking, see CCFCTS.
    val desiredPrototypeS: PrototypeI[sI] = desiredPrototypeN
    val desiredPrototypeC =
      RegionCollapserIndividual.collapsePrototype(desiredPrototypeS)

    val desiredFuncSuperTemplateName = TemplataCompiler.getSuperTemplate(desiredPrototypeT.id)
    val funcT =
      vassertOne(
        hinputs.functions
          .filter(funcT => TemplataCompiler.getSuperTemplate(funcT.header.id) == desiredFuncSuperTemplateName))


    val denizenBoundToDenizenCallerSuppliedThingFromDenizenItself =
      maybeDenizenBoundToDenizenCallerSuppliedThing.getOrElse({
        assembleInstantiationBoundParamToArg(funcT.instantiationBoundParams, suppliedBoundArgs)
        // DenizenBoundToDenizenCallerBoundArgS(
        //   // This is a top level denizen, and someone's calling it. Assemble the bounds!
        //   assembleCalleeDenizenFunctionBounds(funcT.instantiationBoundParams.runeToFunctionBoundArg.mapValues(_.id), suppliedBoundArgs.runeToFunctionBoundArg),
        //   // This is a top level denizen, and someone's calling it. Assemble the bounds!
        //   assembleCalleeDenizenImplBounds(funcT.instantiationBoundParams.runeToImplBoundArg, suppliedBoundArgs.runeToImplBoundArg))
      })
    val argsM = desiredPrototypeS.id.localName.parameters.map(_.kind)
    val paramsT = funcT.header.params.map(_.tyype.kind)
    // val denizenBoundToDenizenCallerSuppliedThingFromParams =
    //   paramsT.zip(argsM).flatMap({ case (a, x) =>
    //     hoistBoundsFromParameter(hinputs, monouts, a, x)
    //   })

    val denizenBoundToDenizenCallerSuppliedThingFromDenizenItselfAndParams =
      denizenBoundToDenizenCallerSuppliedThingFromDenizenItself
      // Vector(denizenBoundToDenizenCallerSuppliedThingFromDenizenItself) ++
      //   denizenBoundToDenizenCallerSuppliedThingFromParams

    val denizenBoundToDenizenCallerSuppliedThing =
      denizenBoundToDenizenCallerSuppliedThingFromDenizenItselfAndParams
      // DenizenBoundToDenizenCallerBoundArgS(
      //   denizenBoundToDenizenCallerSuppliedThingFromDenizenItselfAndParams
      //     .map(_.funcBoundToCallerSuppliedBoundArgFunc)
      //     .reduceOption(_ ++ _).getOrElse(Map()),
      //   denizenBoundToDenizenCallerSuppliedThingFromDenizenItselfAndParams
      //       .map(_.funcReachableToCallerSuppliedReachableArgFunc)
      //       .reduceOption(_ ++ _).getOrElse(Map()),
      //   denizenBoundToDenizenCallerSuppliedThingFromDenizenItselfAndParams
      //     .map(_.implBoundToCallerSuppliedBoundArgImpl)
      //     .reduceOption(_ ++ _).getOrElse(Map()))


    val substitutions = assemblePlaceholderMap(funcT.header.id, desiredPrototypeS.id)
//    val instantiator =
//      new Instantiator(
//        opts,
//        interner,
//        keywords,
//        hinputs,
//        monouts,
//        funcTemplateNameT,
//        desiredPrototypeT.id,
//        denizenBoundToDenizenCallerSuppliedThing)

    val monomorphizedFuncT =
      translateCollapsedFunction(
        desiredPrototypeT.id, denizenBoundToDenizenCallerSuppliedThing, substitutions, desiredPrototypeC, funcT)

    vassert(desiredPrototypeC.returnType == monomorphizedFuncT.header.returnType)

    monomorphizedFuncT
  }
*/
// mig: fn translate_abstract_func
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_abstract_func(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _interface_id_c: &IdI<'s, 'i, cI>, _desired_abstract_prototype_t: &PrototypeT<'s, 't>, _desired_abstract_prototype_n: &PrototypeI<'s, 'i, nI>, _virtual_index: usize, _supplied_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) {
        panic!("Unimplemented: translate_abstract_func");
    }
}
/*
  def translateAbstractFunc(
      opts: GlobalOptions,
      interner: Interner,
      keywords: Keywords,
      hinputs: HinputsT,
      monouts: InstantiatedOutputs,
      interfaceIdC: IdI[cI, IInterfaceNameI[cI]],
      desiredAbstractPrototypeT: PrototypeT[IFunctionNameT],
      desiredAbstractPrototypeN: PrototypeI[nI],
      virtualIndex: Int,
      suppliedBoundArgs: InstantiationBoundArgumentsI):
  Unit = {
    // This works because the sI/cI are never actually used in these instances, they are just a
    // compile-time type-system bit of tracking, see CCFCTS.
    val desiredAbstractPrototypeS: PrototypeI[sI] = desiredAbstractPrototypeN
    val desiredAbstractPrototypeC =
      RegionCollapserIndividual.collapsePrototype(desiredAbstractPrototypeS)

    val desiredSuperTemplateId = TemplataCompiler.getSuperTemplate(desiredAbstractPrototypeT.id)
    val funcT =
      vassertOne(
        hinputs.functions
            .filter(funcT => TemplataCompiler.getSuperTemplate(funcT.header.id) == desiredSuperTemplateId))


    val denizenBoundToDenizenCallerSuppliedThingFromDenizenItself =
      assembleInstantiationBoundParamToArg(funcT.instantiationBoundParams, suppliedBoundArgs)
        // DenizenBoundToDenizenCallerBoundArgS(
        //   // This is a top level denizen, and someone's calling it. Assemble the bounds!
        //   assembleCalleeDenizenFunctionBounds(funcT.instantiationBoundParams.runeToFunctionBoundArg.mapValues(_.id), suppliedBoundArgs.runeToFunctionBoundArg),
        //   // This is a top level denizen, and someone's calling it. Assemble the bounds!
        //   assembleCalleeDenizenImplBounds(funcT.instantiationBoundParams.runeToImplBoundArg, suppliedBoundArgs.runeToImplBoundArg))
    val argsM = desiredAbstractPrototypeS.id.localName.parameters.map(_.kind)
    val paramsT = funcT.header.params.map(_.tyype.kind)
    // val denizenBoundToDenizenCallerSuppliedThingFromParams =
    //   paramsT.zip(argsM).flatMap({ case (a, x) =>
    //     hoistBoundsFromParameter(hinputs, monouts, a, x)
    //   })

    // val denizenBoundToDenizenCallerSuppliedThingFromDenizenItselfAndParams =
    //   Vector(denizenBoundToDenizenCallerSuppliedThingFromDenizenItself) ++
    //       denizenBoundToDenizenCallerSuppliedThingFromParams

    val denizenBoundToDenizenCallerSuppliedThing = denizenBoundToDenizenCallerSuppliedThingFromDenizenItself
      // DenizenBoundToDenizenCallerBoundArgS(
      //   denizenBoundToDenizenCallerSuppliedThingFromDenizenItselfAndParams
      //       .map(_.funcBoundToCallerSuppliedBoundArgFunc)
      //       .reduceOption(_ ++ _).getOrElse(Map()),
      //   denizenBoundToDenizenCallerSuppliedThingFromDenizenItselfAndParams
      //       .map(_.funcReachableToCallerSuppliedReachableArgFunc)
      //       .reduceOption(_ ++ _).getOrElse(Map()),
      //   denizenBoundToDenizenCallerSuppliedThingFromDenizenItselfAndParams
      //       .map(_.implBoundToCallerSuppliedBoundArgImpl)
      //       .reduceOption(_ ++ _).getOrElse(Map()))


    vassert(!monouts.abstractFuncToBounds.contains(desiredAbstractPrototypeC.id))
    monouts.abstractFuncToBounds.put(desiredAbstractPrototypeC.id, (denizenBoundToDenizenCallerSuppliedThing, suppliedBoundArgs))

    val abstractFuncs = vassertSome(monouts.interfaceToAbstractFuncToVirtualIndex.get(interfaceIdC))
    vassert(!abstractFuncs.contains(desiredAbstractPrototypeC))
    abstractFuncs.put(desiredAbstractPrototypeC, virtualIndex)

    vassertSome(monouts.interfaceToImpls.get(interfaceIdC)).foreach({ case (implT, impl) =>
      translateOverride(opts, interner, keywords, hinputs, monouts, implT, impl, desiredAbstractPrototypeT, desiredAbstractPrototypeC, suppliedBoundArgs)
    })
  }
*/
// mig: fn assemble_placeholder_map
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn assemble_placeholder_map(&self, id_t: &IdT<'s, 't>, id_s: &IdI<'s, 'i, sI>) -> IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>> {
        let mut result: IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>> = match id_t.init_non_package_id(self.typing_interner) {
            None => IndexMap::new(),
            Some(init_non_package_id_t) => {
                self.assemble_placeholder_map(&init_non_package_id_t, &id_s.init_non_package_id().unwrap())
            }
        };
        match IInstantiationNameT::try_from(id_t.local_name) {
            Ok(_local_name_t) => {
                let instantiation_id_t = id_t;
                let instantiation_id_s =
                    match IInstantiationNameI::try_from(id_s.local_name) {
                        Ok(_) => id_s,
                        Err(_) => {
                            // We could get here if, for example, idT is an instantiation like Vec<int> and idS is a template Vec.
                            panic!("vwat")
                        }
                    };
                let inner = self.assemble_placeholder_map_inner(instantiation_id_t, instantiation_id_s);
                result.extend(inner);
            }
            Err(_) => {}
        }
        result
    }
}
/*
  // Per @PASDZ, the result is a flat map keyed by the full path-encoded placeholder id;
  // multiple denizens' placeholders can coexist (lambdas, override dispatchers) without colliding.
  def assemblePlaceholderMap(
      idT: IdT[INameT],
      idS: IdI[sI, INameI[sI]]):
  Map[IdT[IPlaceholderNameT], ITemplataI[sI]] = {
    (idT.initNonPackageId() match {
      case None => Map()
      case Some(initNonPackageIdT) => {
        assemblePlaceholderMap(initNonPackageIdT, vassertSome(idS.initNonPackageId()))
      }
    }) ++
    (idT match {
      case IdT(packageCoordT, initStepsT, localNameT: IInstantiationNameT) => {
        val instantiationIdT = IdT(packageCoordT, initStepsT, localNameT)
        val instantiationIdS =
          idS match {
            case IdI(packageCoordI, initStepsI, localNameUncastedI) => {
              if (localNameUncastedI.isInstanceOf[IInstantiationNameI[sI]]) {
                IdI(packageCoordI, initStepsI, localNameUncastedI.asInstanceOf[IInstantiationNameI[sI]])
              } else {
                // We could get here if, for example, idT is an instantiation like Vec<int> and idS is a template Vec.
                vwat()
              }
            }
            case _ => vwat()
          }
        assemblePlaceholderMapInner(instantiationIdT, instantiationIdS)
      }
      case _ => Map()
    })
  }
*/
// mig: fn assemble_placeholder_map_inner
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn assemble_placeholder_map_inner(&self, id_t: &IdT<'s, 't>, id_s: &IdI<'s, 'i, sI>) -> IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>> {
        let placeholdered_name = id_t;
        IInstantiationNameT::try_from(placeholdered_name.local_name).unwrap().template_args()
            .iter()
            .zip(IInstantiationNameI::try_from(id_s.local_name).unwrap().template_args(self.interner).iter())
            .flat_map(|(template_arg_t, template_arg_i)| -> Vec<(IdT<'s, 't>, ITemplataI<'s, 'i, sI>)> {
                use crate::typing::templata::templata::ITemplataT;
                use crate::typing::types::types::{KindT, MutabilityT};
                use crate::instantiating::ast::types::MutabilityI;
                match (template_arg_t, template_arg_i) {
                    (ITemplataT::Coord(ct), c @ ITemplataI::Coord(_)) => {
                        match ct.coord.kind {
                            KindT::KindPlaceholder(kp) => vec![(kp.id, *c)],
                            _ => vec![],
                        }
                    }
                    (ITemplataT::Kind(kt), kind_templata_i) => {
                        match kt.kind {
                            KindT::KindPlaceholder(kp) => vec![(kp.id, *kind_templata_i)],
                            _ => panic!("assemble_placeholder_map_inner: KindTemplataT non-placeholder arm"),
                        }
                    }
                    (ITemplataT::Placeholder(pt), templata_i) => vec![(pt.id, *templata_i)],
                    (ITemplataT::Mutability(mt), ITemplataI::Mutability(mi)) if matches!(mt.mutability, MutabilityT::Mutable) && matches!(mi.mutability, MutabilityI::Mutable) => vec![],
                    (ITemplataT::Mutability(mt), ITemplataI::Mutability(mi)) if matches!(mt.mutability, MutabilityT::Immutable) && matches!(mi.mutability, MutabilityI::Immutable) => vec![],
                    _ => panic!("assemble_placeholder_map_inner: unimplemented arm"),
                }
            })
            .collect()
    }
}
/*
  def assemblePlaceholderMapInner(
    idT: IdT[IInstantiationNameT],
    idS: IdI[sI, IInstantiationNameI[sI]]):
  Map[IdT[IPlaceholderNameT], ITemplataI[sI]] = {
    val placeholderedName = idT
//    val placeholderedName =
//      idT match {
//        case IdT(_, _, localName : IStructNameT) => {
//          hinputs.lookupStructByTemplate(localName.template).instantiatedCitizen.id
//        }
//        case IdT(_, _, localName : IInterfaceNameT) => {
//          hinputs.lookupInterfaceByTemplate(localName.template).instantiatedInterface.id
//        }
//        case IdT(_, _, localName : IFunctionNameT) => {
//          vassertSome(hinputs.lookupFunction(localName.template)).header.id
//        }
//        case IdT(_, _, localName : IImplNameT) => {
//          hinputs.lookupImplByTemplate(localName.template).edgeId
//        }
//        case IdT(_, _, localName : ExportNameT) => {
//          vassertOne(
//            hinputs.kindExports.filter(_.id.localName.template == localName.template).map(_.id) ++
//              hinputs.functionExports.filter(_.exportId.localName.template == localName.template).map(_.exportId))
//        }
//      }

      placeholderedName.localName.templateArgs
        .zip(idS.localName.templateArgs)
        .flatMap({
          case (
              CoordTemplataT(CoordT(placeholderOwnership, RegionT(DefaultRegionT), kindT)),
              c @ CoordTemplataI(regionI, _)) => {
            kindT match {
              case KindPlaceholderT(kindPlaceholderId) => {
                vregionmut()
                // vassert(placeholderOwnership == OwnT || placeholderOwnership == ShareT)
                // In "Array has" test, we actually have a placeholder thats a borrow.

                // // We might need to do something with placeholderRegion here, but I think we can just
                // // assume it correctly matches up with the coord's region. The typing phase should have
                // // made sure it matches up nicely.
                // // If we hit this vimpl, then we might need to find some way to hand in the region,
                // // even though we lost that in the translation to IdI which has no regions. We might be
                // // able to scavenge it from the name, though it might be tricky to get the region of
                // // region-less primitives. Perhaps we can assume theyre the same region as their
                // // parent template?
                // val regionTemplata =
                //   maybeRegionHeight.map(x => RegionTemplataI[sI](x)).getOrElse(vimpl())
                vcurious(regionI.pureHeight <= 0) // These are subjective, but they should be negative
                List(
                  (kindPlaceholderId -> c))
              }
              // This could be e.g. *i32 and *!i32, in other words the template arg is already populated. This can
              // happen if we're processing a lambda's name.
              // placeholderedName *doesn't* contain a placeholder like one might normally expect:
              //   test/main.lam:0:34.__call{lam:0:34, *i32}<__call$0>    (doesn't have this)
              // Instead placeholderedName might be:
              //   test/main.lam:0:34.__call{lam:0:34, *i32}<*i32>
              // ...because the typing phase already filled it in.
              // Theoretically the typing phase could have stripped that out before now, maybe. Don't know.
              // Either way, it is there.
              // Just ignore it, we don't need a mapping for it.
              case _ => {
                List()
              }
            }
          }
          case (KindTemplataT(KindPlaceholderT(placeholderId)), kindTemplataI) => {
            List((placeholderId -> kindTemplataI))
          }
          case (PlaceholderTemplataT(placeholderId, tyype), templataI) => {
            List((placeholderId -> templataI))
          }
          case (MutabilityTemplataT(MutableT),MutabilityTemplataI(MutableI)) |
               (MutabilityTemplataT(ImmutableT),MutabilityTemplataI(ImmutableI)) => {
            // We once got a `mut` for the placeholdered name's templata.
            // That's because we do some specialization for arrays still.
            // They don't come with a placeholder, so ignore them.
            List()
          }
          case other => vimpl(other)
        })
        .toMap
  }

  // // This isn't just for parameters, it's for impl subcitizens, and someday for cases too.
  // // See NBIFP
  // private def hoistBoundsFromParameter(
  //   hinputs: HinputsT,
  //   monouts: InstantiatedOutputs,
  //   paramT: KindT,
  //   paramS: KindIT[sI]):
  // Option[DenizenBoundToDenizenCallerBoundArgS] = {
  //   (paramT, paramS) match {
  //     case (StructTT(structIdT), StructIT(structIdI)) => {
  //       val calleeRuneToBoundArgT = hinputs.getInstantiationBoundArgs(structIdT)
  //       val structDenizenBoundToDenizenCallerSuppliedThing =
  //         vassertSome(monouts.structToBounds.get(structIdI))
  //       val structT = findStruct(hinputs, structIdT)
  //       val denizenBoundToDenizenCallerSuppliedThing =
  //         hoistBoundsFromParameterInner(
  //           structDenizenBoundToDenizenCallerSuppliedThing,
  //           calleeRuneToBoundArgT,
  //           structT.instantiationBoundParams.runeToFunctionBoundArg.mapValues(_.id),
  //           structT.instantiationBoundParams.callerKindRuneToReachableBoundArguments.mapValues(_.citizenRuneToReachablePrototype.mapValues(_.prototype.id)),
  //           structT.instantiationBoundParams.runeToImplBoundArg)
  //       Some(denizenBoundToDenizenCallerSuppliedThing)
  //     }
  //     case (InterfaceTT(interfaceIdT), InterfaceIT(interfaceIdM)) => {
  //       val calleeRuneToBoundArgT = hinputs.getInstantiationBoundArgs(interfaceIdT)
  //       val interfaceDenizenBoundToDenizenCallerSuppliedThing = vassertSome(monouts.interfaceToBounds.get(interfaceIdM))
  //       val interfaceT = findInterface(hinputs, interfaceIdT)
  //       val denizenBoundToDenizenCallerSuppliedThing =
  //         hoistBoundsFromParameterInner(
  //           interfaceDenizenBoundToDenizenCallerSuppliedThing,
  //           calleeRuneToBoundArgT,
  //           interfaceT.instantiationBoundParams.runeToFunctionBoundArg.mapValues(_.id),
  //           interfaceT.instantiationBoundParams.callerKindRuneToReachableBoundArguments.mapValues(_.citizenRuneToReachablePrototype.mapValues(_.prototype.id)),
  //           interfaceT.instantiationBoundParams.runeToImplBoundArg)
  //       Some(denizenBoundToDenizenCallerSuppliedThing)
  //     }
  //     case _ => None
  //   }
  // }

  // // See NBIFP
  // private def hoistBoundsFromParameterInner(
  //   parameterDenizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
  //   calleeRuneToBoundArgT: InstantiationBoundArgumentsT[IFunctionNameT, IFunctionNameT, IImplNameT],
  //   calleeRuneToCalleeFunctionBoundT: Map[IRuneS, IdT[FunctionBoundNameT]],
  //   callerRuneToCalleeRuneToCalleeFunctionReachableT: Map[IRuneS, Map[IRuneS, IdT[ReachableFunctionNameT]]],
  //   calleeRuneToCalleeImplBoundT: Map[IRuneS, IdT[ImplBoundNameT]]):
  // DenizenBoundToDenizenCallerBoundArgS = {
  //   val calleeFunctionBoundTToBoundArgM = parameterDenizenBoundToDenizenCallerSuppliedThing.funcBoundToCallerSuppliedBoundArgFunc
  //   val calleeFunctionReachableTToReachableArgM = parameterDenizenBoundToDenizenCallerSuppliedThing.funcReachableToCallerSuppliedReachableArgFunc
  //   val implBoundTToBoundArgM = parameterDenizenBoundToDenizenCallerSuppliedThing.implBoundToCallerSuppliedBoundArgImpl
  //
  //   val callerSuppliedBoundToInstantiatedFunction =
  //     calleeRuneToCalleeFunctionBoundT.map({ case (calleeRune, calleeBoundT) =>
  //       // We don't care about the callee bound, we only care about what we're sending in to it.
  //       val (_) = calleeBoundT
  //
  //       // This is the prototype the caller is sending in to the callee to satisfy its bounds.
  //       val boundArgT = vassertSome(calleeRuneToBoundArgT.runeToFunctionBoundArg.get(calleeRune))
  //       boundArgT.id match {
  //         case IdT(packageCoord, initSteps, last@FunctionBoundNameT(_, _, _)) => {
  //           // The bound arg is also the same thing as the caller bound.
  //           val callerBoundT = IdT(packageCoord, initSteps, last)
  //
  //           // The bound arg we're sending in is actually one of our (the caller) own bounds.
  //           //
  //           // "But wait, we didn't specify any bounds."
  //           // This is actually a bound that was implicitly added from NBIFP.
  //           //
  //           // We're going to pull this in as our own bound.
  //           val instantiatedPrototype = vassertSome(calleeFunctionBoundTToBoundArgM.get(calleeBoundT))
  //           Some(callerBoundT -> instantiatedPrototype)
  //         }
  //         case _ => vcurious(); None
  //       }
  //     }).flatten.toMap
  //
  //   val callerSuppliedReachableToInstantiatedFunction =
  //     callerRuneToCalleeRuneToCalleeFunctionReachableT.flatMap({ case (callerRune, calleeRuneToCalleeFunctionReachableT) =>
  //       calleeRuneToCalleeFunctionReachableT.flatMap({ case (calleeRune, calleeReachableT) =>
  //         // We don't care about the callee bound, we only care about what we're sending in to it.
  //         val (_) = calleeReachableT
  //
  //         // This is the prototype the caller is sending in to the callee to satisfy its bounds.
  //         val boundArgT =
  //           vassertSome(
  //             vassertSome(calleeRuneToBoundArgT.callerKindRuneToReachableBoundArguments.get(callerRune))
  //                 .citizenRuneToReachablePrototype.get(calleeRune))
  //         boundArgT.prototype.id match {
  //           case IdT(packageCoord, initSteps, last@ReachableFunctionNameT(_, _, _)) => {
  //             // The bound arg is also the same thing as the caller bound.
  //             val callerReachableT = IdT(packageCoord, initSteps, last)
  //
  //             // The bound arg we're sending in is actually one of our (the caller) own bounds.
  //             //
  //             // "But wait, we didn't specify any bounds."
  //             // This is actually a bound that was implicitly added from NBIFP.
  //             //
  //             // We're going to pull this in as our own bound.
  //             val instantiatedPrototype = vassertSome(calleeFunctionReachableTToReachableArgM.get(calleeReachableT))
  //             Some(callerReachableT -> instantiatedPrototype)
  //           }
  //           case _ => vcurious(); None
  //         }
  //       })
  //     })
  //
  //   val callerSuppliedBoundToInstantiatedImpl =
  //     calleeRuneToCalleeImplBoundT.map({
  //       case (calleeRune, calleeBoundT) =>
  //         // We don't care about the callee bound, we only care about what we're sending in to it.
  //         val (_) = calleeBoundT
  //
  //         // This is the prototype the caller is sending in to the callee to satisfy its bounds.
  //         val boundArgT = vassertSome(calleeRuneToBoundArgT.runeToImplBoundArg.get(calleeRune))
  //         boundArgT match {
  //           case IdT(packageCoord, initSteps, last@ImplBoundNameT(_, _)) => {
  //             val boundT = IdT(packageCoord, initSteps, last)
  //             // The bound arg we're sending in is actually one of our (the caller) own bounds.
  //             //
  //             // "But wait, we didn't specify any bounds."
  //             // This is actually a bound that was implicitly added from NBIFP.
  //             //
  //             // We're going to pull this in as our own bound.
  //             val instantiatedPrototype = vassertSome(implBoundTToBoundArgM.get(boundT))
  //             Some(boundT -> instantiatedPrototype)
  //           }
  //           case _ => vcurious(); None
  //         }
  //     }).flatten.toMap
  //
  //   val denizenBoundToDenizenCallerSuppliedThing =
  //     DenizenBoundToDenizenCallerBoundArgS(
  //       callerSuppliedBoundToInstantiatedFunction,
  //       callerSuppliedReachableToInstantiatedFunction,
  //       callerSuppliedBoundToInstantiatedImpl)
  //   denizenBoundToDenizenCallerSuppliedThing
  // }

//  def translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, templata: ITemplataT[ITemplataType]): ITemplataI = {
//    vimpl()
//  }

  //  selfFunctionBoundToRuneUnsubstituted: Map[PrototypeT, IRuneS],
  //  denizenRuneToDenizenCallerPrototype: Map[IRuneS, PrototypeT]) {

  //  if (opts.sanityCheck) {
  //    denizenFunctionBoundToDenizenCallerSuppliedPrototype.foreach({
  //      case (denizenFunctionBound, denizenCallerSuppliedPrototype) => {
  //        vassert(Collector.all(denizenCallerSuppliedPrototype, { case PlaceholderTemplateNameT(_) => }).isEmpty)
  //      }
  //    })
  //  }
*/
// mig: fn translate_struct_member
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_struct_member(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _member: &IStructMemberT<'s, 't>) -> (CoordI<'s, 'i, sI>, StructMemberI<'s, 'i, cI>) {
        match _member {
            IStructMemberT::Normal(n) => {
                let crate::typing::ast::citizens::NormalStructMemberT { name, variability, tyype } = n;
                let (member_subjective_it, member_type_i) = match tyype {
                    crate::typing::ast::citizens::IMemberTypeT::Reference(r) => {
                        let type_s = self.translate_coord(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &r.reference);
                        let result_ref = crate::instantiating::ast::citizens::ReferenceMemberTypeI {
                            reference: crate::instantiating::region_collapser_individual::collapse_coord(self.interner, &type_s.coord),
                            _marker: std::marker::PhantomData,
                        };
                        let result_ref: &'i crate::instantiating::ast::citizens::ReferenceMemberTypeI<'s, 'i, cI> = self.interner.bump().alloc(result_ref);
                        (type_s.coord, crate::instantiating::ast::citizens::IMemberTypeI::ReferenceMemberTypeI(result_ref))
                    }
                    crate::typing::ast::citizens::IMemberTypeT::Address(a) => {
                        let type_s = self.translate_coord(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &a.reference);
                        let result = crate::instantiating::ast::citizens::AddressMemberTypeI {
                            reference: crate::instantiating::region_collapser_individual::collapse_coord(self.interner, &type_s.coord),
                            _marker: std::marker::PhantomData,
                        };
                        let result: &'i crate::instantiating::ast::citizens::AddressMemberTypeI<'s, 'i, cI> = self.interner.bump().alloc(result);
                        (type_s.coord, crate::instantiating::ast::citizens::IMemberTypeI::AddressMemberTypeI(result))
                    }
                };
                let name_s = Self::translate_var_name(self.interner, name);
                let member_c = StructMemberI {
                    name: crate::instantiating::region_collapser_individual::collapse_var_name(self.interner, &name_s),
                    variability: Self::translate_variability(variability),
                    tyype: member_type_i,
                };
                (member_subjective_it, member_c)
            }
            IStructMemberT::Variadic(_) => panic!("Unimplemented: translate_struct_member Variadic"),
        }
    }
}
/*
  def translateStructMember(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    member: IStructMemberT):
  StructMemberI = {
    member match {
      case NormalStructMemberT(name, variability, tyype) => {
        val (memberSubjectiveIT, memberTypeI) =
          tyype match {
            case ReferenceMemberTypeT(unsubstitutedCoord) => {
              val typeS =
                translateCoord(
                  denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, unsubstitutedCoord)
              val result =
                ReferenceMemberTypeI(
                  RegionCollapserIndividual.collapseCoord(typeS.coord))
              (typeS, result)
            }
            case AddressMemberTypeT(unsubstitutedCoord) => {
              val typeS =
                translateCoord(
                  denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, unsubstitutedCoord)
              val result = AddressMemberTypeI(RegionCollapserIndividual.collapseCoord(typeS.coord))
              (typeS, result)
            }
          }
        val nameS = translateVarName(name)
        val memberC =
          StructMemberI(
            RegionCollapserIndividual.collapseVarName(nameS),
            translateVariability(variability),
            memberTypeI)
        memberC
      }
      case VariadicStructMemberT(name, tyype) => {
        vimpl()
      }
    }
  }
*/
// mig: fn translate_variability
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_variability(x: &VariabilityT) -> VariabilityI {
        match x {
            VariabilityT::Varying => VariabilityI::Varying,
            VariabilityT::Final => VariabilityI::Final,
        }
    }
}
/*
  def translateVariability(x: VariabilityT): VariabilityI = {
    x match {
      case VaryingT => VaryingI
      case FinalT => FinalI
    }
  }
*/
// mig: fn translate_mutability
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_mutability(_m: &MutabilityT) -> MutabilityI {
        match _m {
            MutabilityT::Mutable => MutabilityI::Mutable,
            MutabilityT::Immutable => MutabilityI::Immutable,
        }
    }
}
/*
  def translateMutability(m: MutabilityT): MutabilityI = {
    m match {
      case MutableT => MutableI
      case ImmutableT => ImmutableI
    }
  }
*/
// mig: fn translate_prototype
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_prototype(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, perspective_region_t: &RegionT, desired_prototype_t: &PrototypeT<'s, 't>) -> (PrototypeI<'s, 'i, sI>, PrototypeI<'s, 'i, cI>) {
        let PrototypeT { id: desired_prototype_id_unsubstituted, return_type: desired_prototype_return_type_unsubstituted } = desired_prototype_t;

        let rune_to_bound_args_for_call =
            self.translate_bound_args_for_callee(
                monouts,
                denizen_name,
                denizen_bound_to_denizen_caller_supplied_thing,
                substitutions,
                perspective_region_t,
                self.hinputs.get_instantiation_bound_args(desired_prototype_t.id));

        let return_subjective_it =
            self.translate_coord(
                monouts,
                denizen_name,
                denizen_bound_to_denizen_caller_supplied_thing,
                substitutions,
                perspective_region_t,
                desired_prototype_return_type_unsubstituted);

        let desired_prototype_s =
            self.interner.intern_prototype_si(PrototypeIValI {
                id: self.translate_function_id(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, desired_prototype_id_unsubstituted),
                return_type: return_subjective_it.coord,
            });

        match desired_prototype_t.id {
            IdT { local_name: INameT::FunctionBound(_), .. } => {
                let func_bound_name = desired_prototype_t.id;
                let prototype_s = *denizen_bound_to_denizen_caller_supplied_thing.func_id_to_bound_arg_prototype.get(&func_bound_name).expect("vassertSome: func_id_to_bound_arg_prototype");
                let prototype_c = region_collapser_individual::collapse_prototype(self.interner, prototype_s);
                (*prototype_s, prototype_c)
            }
            IdT { local_name: INameT::ExternFunction(_), .. } => {
                if self.opts.sanity_check {
                    // Scala's `vassert(Collector.all(desiredPrototypeS, {case KindPlaceholderTemplateNameT => }).isEmpty)`
                    // sanity check is omitted: the I-side AST is statically typed and can't structurally hold a
                    // typing-pass placeholder template name, so it's vacuously empty. (Architect-approved parity gap.)
                }
                let desired_prototype_c =
                    region_collapser_individual::collapse_prototype(self.interner, desired_prototype_s);
                (*desired_prototype_s, desired_prototype_c)
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
                        let denizen_root_super_template = crate::typing::compiler::Compiler::get_root_super_template(self.typing_interner, *denizen_name);
                        let desired_prototype_root_super_template = crate::typing::compiler::Compiler::get_root_super_template(self.typing_interner, desired_prototype_t.id);
                        assert!(denizen_root_super_template == desired_prototype_root_super_template);
                    }
                    _ => {}
                }

                let desired_prototype_c =
                    region_collapser_individual::collapse_prototype(self.interner, desired_prototype_s);
                let desired_prototype_n =
                    region_collapser_consistent::collapse_prototype(
                        self.interner,
                        region_counter::count_prototype_map(desired_prototype_s),
                        desired_prototype_s);

                assert!(region_collapser_individual::collapse_prototype(self.interner, &desired_prototype_n) == desired_prototype_c);

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
                    desired_prototype_n,
                    rune_to_bound_args_for_call,
                    maybe_denizen_bound_to_denizen_caller_supplied_thing,
                ));
                (*desired_prototype_s, desired_prototype_c)
            }
        }
    }
}
/*
  // This is run at the call site, from the caller's perspective
  def translatePrototype(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    desiredPrototypeT: PrototypeT[IFunctionNameT]):
  (PrototypeI[sI], PrototypeI[cI]) = {
    val PrototypeT(desiredPrototypeIdUnsubstituted, desiredPrototypeReturnTypeUnsubstituted) = desiredPrototypeT

    val runeToBoundArgsForCall =
      translateBoundArgsForCallee(
        denizenName,
        denizenBoundToDenizenCallerSuppliedThing,
        substitutions,
        perspectiveRegionT,
        hinputs.getInstantiationBoundArgs(desiredPrototypeT.id))

    val returnSubjectiveIT =
      translateCoord(
        denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, desiredPrototypeReturnTypeUnsubstituted)

    val desiredPrototypeS =
      PrototypeI[sI](
        translateFunctionId(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, desiredPrototypeIdUnsubstituted),
        returnSubjectiveIT.coord)

    desiredPrototypeT.id match {
      case IdT(packageCoord, initSteps, name @ FunctionBoundNameT(_, _, _)) => {
        val funcBoundName = IdT(packageCoord, initSteps, name)
        val prototypeS = vassertSome(denizenBoundToDenizenCallerSuppliedThing.funcIdToBoundArgPrototype.get(funcBoundName))
        //        if (opts.sanityCheck) {
        //          vassert(Collector.all(result, { case PlaceholderTemplateNameT(_) => }).isEmpty)
        //        }

        val prototypeC =
          RegionCollapserIndividual.collapsePrototype(prototypeS)

        (prototypeS, prototypeC)
      }

      case IdT(packageCoord, initSteps, name@FunctionBoundNameT(_, _, _)) => {
        val actualPrototypeS =
          vassertSome(denizenBoundToDenizenCallerSuppliedThing.funcIdToBoundArgPrototype.get(IdT(packageCoord, initSteps, name)))

        val actualDesiredPrototypeC =
          RegionCollapserIndividual.collapsePrototype(actualPrototypeS)
        (actualPrototypeS, actualDesiredPrototypeC)
      }
      case IdT(_, _, ExternFunctionNameT(_, _, _)) => {
        if (opts.sanityCheck) {
          vassert(Collector.all(desiredPrototypeS, { case KindPlaceholderTemplateNameT(_, _) => }).isEmpty)
        }
        val desiredPrototypeC =
          RegionCollapserIndividual.collapsePrototype(desiredPrototypeS)
        (desiredPrototypeS, desiredPrototypeC)
      }
      case IdT(_, _, last) => {
        last match {
          case LambdaCallFunctionNameT(_, _, _) => {
            // Lambdas Can Call Sibling Lambdas (LCCSL)
            // If we want to call a lambda, there are three possibilities I've seen:
            // - We're in the root denizen and we want to call our own lambda.
            // - We're in a lambda and we want to call an even deeper lambda.
            // - (This is the weird one) we want to call a *sibling* lambda.
            // In all cases, make sure the denizen roots of everyone agree.
            val denizenRootSuperTemplate = TemplataCompiler.getRootSuperTemplate(interner, denizenName)
            val desiredPrototypeRootSuperTemplate = TemplataCompiler.getRootSuperTemplate(interner, desiredPrototypeT.id)
            vassert(denizenRootSuperTemplate == desiredPrototypeRootSuperTemplate)

//            (denizenName.steps.last, desiredPrototypeS.id.steps.init.init.last) match {
//              case (
//                  FunctionNameT(FunctionTemplateNameT(nameA,codeLocA),templateArgsA,parametersA),
//                  FunctionNameIX(FunctionTemplateNameI(nameB,codeLocB),templateArgsB,parametersB)) => {
//                // Make sure we're talking about roughly the same function
//                vassert(nameA == nameB)
//                vassert(codeLocA == codeLocB)
//                vassert(templateArgsA.length == templateArgsB.length)
//                vassert(parametersA.length == parametersB.length)
//                // Could we have a false positive here if we're doing things on different templates?
//                // I don't think so.
//              }
//              case (
//                  LambdaCallFunctionNameT(LambdaCallFunctionTemplateNameT(codeLocA,paramsTTA),templateArgsA,parametersA),
//                  LambdaCallFunctionNameI(LambdaCallFunctionTemplateNameI(codeLocB,paramsTTB),templateArgsB,parametersB)) => {
//                // Make sure we're talking about roughly the same function
//                vassert(codeLocA == codeLocB)
//                vassert(paramsTTA == paramsTTB)
//                vassert(templateArgsA.length == templateArgsB.length)
//                vassert(parametersA.length == parametersB.length)
//              }
//              case other => vwat(other)
//            }
          }
          case _ =>
        }

//        // Let's say we want to call 1'myPureDisplay(0'board).
//        // We want that to become 0'myPureDisplay(-1'board).
//        // The default region we send should always be zero, and all incoming imms should be negative.
//        // TODO(regions): centralize docs
//        // TODO use an array instead of a map here
//        val oldRegionPureHeights =
//          Collector.all(uncollapsedDesiredPrototypeI, {
//            case RegionTemplataI(pureHeight) => pureHeight
//          }).toVector.distinct.sorted
//        val oldToNewRegionPureHeight =
//          oldRegionPureHeights.zipWithIndex.map({ case (oldRegionPureHeight, index) =>
//            (oldRegionPureHeight, index - (oldRegionPureHeights.length - 1))
//          }).toMap

        val desiredPrototypeC =
          RegionCollapserIndividual.collapsePrototype(desiredPrototypeS)

        val desiredPrototypeN =
          RegionCollapserConsistent.collapsePrototype(
            RegionCounter.countPrototype(desiredPrototypeS),
            desiredPrototypeS)

        vassert(RegionCollapserIndividual.collapsePrototype(desiredPrototypeN) == desiredPrototypeC)

        monouts.newFunctions.enqueue(
          (
            desiredPrototypeT,
            desiredPrototypeN,
            runeToBoundArgsForCall,
            // If we're instantiating something whose name starts with our name, then we're instantiating our lambda.
            if (TemplataCompiler.getSuperTemplate(desiredPrototypeT.id).steps.startsWith(TemplataCompiler.getSuperTemplate(denizenName).steps)) {
              // We need to supply our bounds to our lambdas, see LCCPGB and LCNBAFA.
              Some(denizenBoundToDenizenCallerSuppliedThing)
            } else {
              if (opts.sanityCheck) {
                val desiredFuncSuperTemplateName = TemplataCompiler.getSuperTemplate(desiredPrototypeT.id)
                val funcT =
                  vassertOne(
                    hinputs.functions
                        .filter(funcT => {
                          TemplataCompiler.getSuperTemplate(funcT.header.id) == desiredFuncSuperTemplateName
                        }))
                vassert(runeToBoundArgsForCall.runeToFunctionBoundArg.size == funcT.instantiationBoundParams.runeToBoundPrototype.size)
                vassert(
                  runeToBoundArgsForCall.callerRuneToCalleeRuneToReachableFunc.count(_._2.nonEmpty) ==
                      funcT.instantiationBoundParams.runeToCitizenRuneToReachablePrototype.count(_._2.citizenRuneToReachablePrototype.nonEmpty))
                vassert(runeToBoundArgsForCall.runeToImplBoundArg.size == funcT.instantiationBoundParams.runeToBoundImpl.size)
              }
              None
            }))
        (desiredPrototypeS, desiredPrototypeC)
      }
    }
  }
*/
// mig: fn translate_bound_args_for_callee
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_bound_args_for_callee(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, instantiation_bound_args_for_call_unsubstituted: &InstantiationBoundArgumentsT<'s, 't>) -> InstantiationBoundArgumentsI<'s, 'i> {
        let rune_to_supplied_bound_prototype_for_call_unsubstituted =
            &instantiation_bound_args_for_call_unsubstituted.rune_to_bound_prototype;
        // For any that are placeholders themselves, let's translate those into actual prototypes.
        let rune_to_supplied_prototype_for_call: ArenaIndexMap<'i, IRuneS<'s>, &'i PrototypeI<'s, 'i, sI>> =
            ArenaIndexMap::from_iter_in(
                rune_to_supplied_bound_prototype_for_call_unsubstituted.iter().map(|(rune, supplied_prototype_unsubstituted)| {
                    let prototype_s: &'i PrototypeI<'s, 'i, sI> = match supplied_prototype_unsubstituted.id {
                        IdT { local_name: INameT::FunctionBound(_), .. } => {
                            let func_bound_name = supplied_prototype_unsubstituted.id;
                            *_denizen_bound_to_denizen_caller_supplied_thing.func_id_to_bound_arg_prototype.get(&func_bound_name).expect("vassertSome: func_id_to_bound_arg_prototype")
                        }
                        _ => {
                            let (prototype_i, _prototype_c) =
                                self.translate_prototype(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, supplied_prototype_unsubstituted);
                            self.interner.alloc(prototype_i)
                        }
                    };
                    (*rune, prototype_s)
                }),
                self.interner.bump());
        // And now we have a map from the callee's rune to the *instantiated* callee's prototypes.

        let caller_rune_to_callee_rune_to_supplied_reachable_prototype_for_call_unsubstituted =
            &instantiation_bound_args_for_call_unsubstituted.rune_to_citizen_rune_to_reachable_prototype;
        // For any that are placeholders themselves, let's translate those into actual prototypes.
        let rune_to_supplied_reachable_prototype_for_call: ArenaIndexMap<'i, IRuneS<'s>, ArenaIndexMap<'i, IRuneS<'s>, &'i PrototypeI<'s, 'i, sI>>> =
            ArenaIndexMap::from_iter_in(
                caller_rune_to_callee_rune_to_supplied_reachable_prototype_for_call_unsubstituted.iter().map(|(_caller_rune, _callee_rune_to_supplied_reachable_prototype_for_call_unsubstituted)| {
                    panic!("Unimplemented: translate_bound_args_for_callee rune_to_supplied_reachable_prototype_for_call")
                }),
                self.interner.bump());
        // And now we have a map from the callee's rune to the *instantiated* callee's prototypes.

        let rune_to_supplied_impl_for_call_unsubstituted =
            &instantiation_bound_args_for_call_unsubstituted.rune_to_bound_impl;
        // For any that are placeholders themselves, let's translate those into actual prototypes.
        let rune_to_supplied_impl_for_call: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i, sI>> =
            ArenaIndexMap::from_iter_in(
                rune_to_supplied_impl_for_call_unsubstituted.iter().map(|(_rune, _supplied_impl_unsubstituted)| {
                    panic!("Unimplemented: translate_bound_args_for_callee rune_to_supplied_impl_for_call")
                }),
                self.interner.bump());
        // And now we have a map from the callee's rune to the *instantiated* callee's impls.

        InstantiationBoundArgumentsI {
            rune_to_function_bound_arg: rune_to_supplied_prototype_for_call,
            caller_rune_to_callee_rune_to_reachable_func: rune_to_supplied_reachable_prototype_for_call,
            rune_to_impl_bound_arg: rune_to_supplied_impl_for_call,
        }
    }
}
/*
  private def translateBoundArgsForCallee(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    // This contains a map from rune to a prototype, specifically the prototype that we
    // (the *template* caller) is supplying to the *template* callee. This prototype might
    // be a placeholder, phrased in terms of our (the *template* caller's) placeholders
    instantiationBoundArgsForCallUnsubstituted: InstantiationBoundArgumentsT[IFunctionNameT, IImplNameT]):
  InstantiationBoundArgumentsI = {
    val runeToSuppliedBoundPrototypeForCallUnsubstituted =
      instantiationBoundArgsForCallUnsubstituted.runeToBoundPrototype
    val runeToSuppliedPrototypeForCall =
    // For any that are placeholders themselves, let's translate those into actual prototypes.
      runeToSuppliedBoundPrototypeForCallUnsubstituted.map({ case (rune, suppliedPrototypeUnsubstituted) =>
        rune ->
          (suppliedPrototypeUnsubstituted.id match {
            case IdT(packageCoord, initSteps, name @ FunctionBoundNameT(_, _, _)) => {
              vassertSome(
                denizenBoundToDenizenCallerSuppliedThing.funcIdToBoundArgPrototype.get(
                  IdT(packageCoord, initSteps, name)))
            }
            case IdT(packageCoord, initSteps, name@FunctionBoundNameT(_, _, _)) => {
              vassertSome(
                denizenBoundToDenizenCallerSuppliedThing.funcIdToBoundArgPrototype.get(
                  IdT(packageCoord, initSteps, name)))
            }
            case _ => {
              val (prototypeI, prototypeC) =
                translatePrototype(
                  denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, suppliedPrototypeUnsubstituted)
              prototypeI
            }
          })
      })
    // And now we have a map from the callee's rune to the *instantiated* callee's prototypes.

    val callerRuneToCalleeRuneToSuppliedReachablePrototypeForCallUnsubstituted =
      instantiationBoundArgsForCallUnsubstituted.runeToCitizenRuneToReachablePrototype
    val runeToSuppliedReachablePrototypeForCall =
    // For any that are placeholders themselves, let's translate those into actual prototypes.
      callerRuneToCalleeRuneToSuppliedReachablePrototypeForCallUnsubstituted.map({ case (callerRune, calleeRuneToSuppliedReachablePrototypeForCallUnsubstituted) =>
        callerRune ->
        calleeRuneToSuppliedReachablePrototypeForCallUnsubstituted.citizenRuneToReachablePrototype
            .map({ case (calleeRune, suppliedReachablePrototypeForCallUnsubstituted) =>
              calleeRune ->
                  (suppliedReachablePrototypeForCallUnsubstituted.id match {
                    case IdT(packageCoord, initSteps, name@FunctionBoundNameT(_, _, _)) => {
                      vassertSome(
                        denizenBoundToDenizenCallerSuppliedThing.funcIdToBoundArgPrototype.get(
                          IdT(packageCoord, initSteps, name)))
                    }
//                    case IdT(packageCoord, initSteps, name@FunctionBoundNameT(_, _, _)) => {
//                      vassertSome(
//                        denizenBoundToDenizenCallerSuppliedThing.funcIdToBoundArgPrototype.get(
//                          IdT(packageCoord, initSteps, name)))
//                    }
                    case _ => {
                      val (prototypeI, prototypeC) =
                        translatePrototype(
                          denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, suppliedReachablePrototypeForCallUnsubstituted)
                      prototypeI
                    }
                  })
            })
      })
    // And now we have a map from the callee's rune to the *instantiated* callee's prototypes.

    val runeToSuppliedImplForCallUnsubstituted =
      instantiationBoundArgsForCallUnsubstituted.runeToBoundImpl
    val runeToSuppliedImplForCall =
    // For any that are placeholders themselves, let's translate those into actual prototypes.
      runeToSuppliedImplForCallUnsubstituted.map({ case (rune, suppliedImplUnsubstituted) =>
        rune ->
          (suppliedImplUnsubstituted match {
            case IdT(packageCoord, initSteps, name @ ImplBoundNameT(_, _)) => {
              vassertSome(
                denizenBoundToDenizenCallerSuppliedThing.boundParamImplIdToBoundArgImplId.get(
                  IdT(packageCoord, initSteps, name)))
            }
            case _ => {
              val implNameS =
                translateImplId(
                  denizenName, denizenBoundToDenizenCallerSuppliedThing,substitutions, perspectiveRegionT, suppliedImplUnsubstituted)
              implNameS
            }
          })
      })
    // And now we have a map from the callee's rune to the *instantiated* callee's impls.

    InstantiationBoundArgumentsI(
      runeToSuppliedPrototypeForCall,
      runeToSuppliedReachablePrototypeForCall,
      runeToSuppliedImplForCall)
  }
*/
// mig: fn translate_collapsed_struct_definition
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_collapsed_struct_definition(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _new_id_t: &IdT<'s, 't>, _new_id: &IdI<'s, 'i, cI>, _struct_def_t: &StructDefinitionT<'s, 't>) {
        let StructDefinitionT { template_name: _, instantiated_citizen: _, attributes, weakable, mutability: mutability_t, members, is_closure, instantiation_bound_params: _ } = _struct_def_t;
        // Scala's `if (opts.sanityCheck) { vassert(Collector.all(newId, {case KindPlaceholderNameT(_) => }).isEmpty) }`
        // sanity check is omitted: the I-side AST is statically typed and can't structurally hold a typing-pass
        // KindPlaceholder name, so it's vacuously empty. (Architect-approved parity gap.)
        let perspective_region_t = RegionT { region: IRegionT::Default };
        let mutability = crate::instantiating::ast::templata::expect_mutability_templata(self.translate_templata(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, &perspective_region_t, mutability_t)).mutability;
        if _monouts.struct_to_mutability.contains_key(_new_id) {
            return;
        }
        _monouts.struct_to_mutability.insert(*_new_id, mutability);
        let attributes_i: Vec<crate::instantiating::ast::ast::ICitizenAttributeI<'s>> = attributes.iter().map(|a| Self::translate_citizen_attribute(a)).collect();
        let members_i: Vec<crate::instantiating::ast::citizens::StructMemberI<'s, 'i, cI>> = members.iter().map(|m| {
            let (_, sm) = self.translate_struct_member(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, &perspective_region_t, m);
            sm
        }).collect();
        let result = crate::instantiating::ast::citizens::StructDefinitionI {
            instantiated_citizen: self.interner.intern_struct_it_ci(crate::instantiating::ast::types::StructITValI { id: *_new_id }),
            attributes: self.interner.bump().alloc_slice_fill_iter(attributes_i.into_iter()),
            weakable: *weakable,
            mutability,
            members: self.interner.bump().alloc_slice_fill_iter(members_i.into_iter()),
            is_closure: *is_closure,
            rune_to_function_bound: crate::utils::arena_index_map::ArenaIndexMap::new_in(self.interner.bump()),
            rune_to_impl_bound: crate::utils::arena_index_map::ArenaIndexMap::new_in(self.interner.bump()),
        };
        assert_eq!(result.instantiated_citizen.id, *_new_id);
        let attrs_for_extern_check = self.interner.bump().alloc_slice_fill_iter(result.attributes.iter().copied()) as &[_];
        let result_ref: &'i crate::instantiating::ast::citizens::StructDefinitionI<'s, 'i, cI> = self.interner.alloc(result);
        _monouts.structs.insert(result_ref.instantiated_citizen.id, result_ref);
        if attrs_for_extern_check.iter().any(|a| matches!(a, crate::instantiating::ast::ast::ICitizenAttributeI::ExternI(_))) {
            _monouts.kind_externs.push(crate::instantiating::ast::ast::KindExternI { r#struct: result_ref.instantiated_citizen });
        }
        // Scala's `if (opts.sanityCheck) { vassert ... }` sanity checks on `result.instantiated_citizen` and
        // `result.members` for `KindPlaceholderNameT` are omitted: same parity gap as the earlier sanity-check block.
    }
}
/*
Guardian: temp-disable: SPDMX — Architect-approved parity gap (already used twice in this file, e.g. in `translate_method`'s `function_exports_c` and `function_externs_c` closures): Scala's `if (opts.sanityCheck) { vassert(Collector.all(..., {case KindPlaceholderNameT(_) => }).isEmpty) }` sanity check is omitted because the I-side AST is statically typed and can't structurally hold a typing-pass KindPlaceholder name. Same precedent applies for both sanity-check blocks here. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-2254-1780130350734/hook-2254/translate_collapsed_struct_definition--2303.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def translateCollapsedStructDefinition(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    newIdT: IdT[IStructNameT],
    newId: IdI[cI, IStructNameI[cI]],
    structDefT: StructDefinitionT):
  Unit = {
    val StructDefinitionT(templateName, instantiatedCitizen, attributes, weakable, mutabilityT, members, isClosure, _) = structDefT

    if (opts.sanityCheck) {
      vassert(Collector.all(newId, { case KindPlaceholderNameT(_) => }).isEmpty)
    }

    val perspectiveRegionT = RegionT(DefaultRegionT)
      // structDefT.instantiatedCitizen.id.localName.templateArgs.last match {
      //   case PlaceholderTemplataT(IdT(packageCoord, initSteps, r @ RegionPlaceholderNameT(_, _, _, _)), RegionTemplataType()) => {
      //     IdT(packageCoord, initSteps, r)
      //   }
      //   case _ => vwat()
      // }

    val mutability = ITemplataI.expectMutabilityTemplata(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, mutabilityT)).mutability

    if (monouts.structToMutability.contains(newId)) {
      return
    }
    monouts.structToMutability.put(newId, mutability)

//    val currentPureHeight = vimpl()

    val result =
      StructDefinitionI(
//        templateName,
        StructIT(newId),
        attributes.map(translateCitizenAttribute),
        weakable,
        mutability,
        members.map(memberT => {
          translateStructMember(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, memberT)
        }),
        isClosure,
        Map(),
        Map())

    vassert(result.instantiatedCitizen.id == newId)

    monouts.structs.put(result.instantiatedCitizen.id, result)

    if (result.attributes.exists({ case ExternI(_) => true case _ => false})) {
      monouts.kindExterns += KindExternI(StructIT(newId))
    }

    if (opts.sanityCheck) {
      vassert(Collector.all(result.instantiatedCitizen, { case KindPlaceholderNameT(_) => }).isEmpty)
      vassert(Collector.all(result.members, { case KindPlaceholderNameT(_) => }).isEmpty)
    }
    result
  }
*/
// mig: fn translate_collapsed_interface_definition
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_collapsed_interface_definition(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _new_id_c: &IdI<'s, 'i, cI>, _interface_def_t: &InterfaceDefinitionT<'s, 't>) {
        if _monouts.interface_to_mutability.contains_key(_new_id_c) {
            return;
        }
        let InterfaceDefinitionT { template_name: _, instantiated_interface: _, ref_: _, attributes, weakable, mutability: mutability_t, instantiation_bound_params: _, internal_methods: _ } = _interface_def_t;
        assert!(!_monouts.interface_to_impl_to_abstract_prototype_to_override.contains_key(_new_id_c));
        _monouts.interface_to_impl_to_abstract_prototype_to_override.insert(*_new_id_c, IndexMap::new());
        assert!(!_monouts.interface_to_abstract_func_to_virtual_index.contains_key(_new_id_c));
        _monouts.interface_to_abstract_func_to_virtual_index.insert(*_new_id_c, IndexMap::new());
        assert!(!_monouts.interface_to_impls.contains_key(_new_id_c));
        _monouts.interface_to_impls.insert(*_new_id_c, Vec::new());
        // Scala `if (opts.sanityCheck) { ... }` collector check omitted (architect-approved parity gap).
        let perspective_region_t = RegionT { region: IRegionT::Default };
        let mutability = crate::instantiating::ast::templata::expect_mutability_templata(self.translate_templata(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, &perspective_region_t, mutability_t)).mutability;
        assert!(!_monouts.interface_to_mutability.contains_key(_new_id_c));
        _monouts.interface_to_mutability.insert(*_new_id_c, mutability);
        let new_interface_it = self.interner.intern_interface_it_ci(crate::instantiating::ast::types::InterfaceITValI { id: *_new_id_c });
        let attributes_i: Vec<crate::instantiating::ast::ast::ICitizenAttributeI<'s>> = attributes.iter().map(|a| Self::translate_citizen_attribute(a)).collect();
        let result = crate::instantiating::ast::citizens::InterfaceDefinitionI {
            instantiated_interface: new_interface_it,
            attributes: self.interner.bump().alloc_slice_fill_iter(attributes_i.into_iter()),
            weakable: *weakable,
            mutability,
            rune_to_function_bound: crate::utils::arena_index_map::ArenaIndexMap::new_in(self.interner.bump()),
            rune_to_impl_bound: crate::utils::arena_index_map::ArenaIndexMap::new_in(self.interner.bump()),
            internal_methods: &[],
            _marker: std::marker::PhantomData,
        };
        let result_ref: &'i crate::instantiating::ast::citizens::InterfaceDefinitionI<'s, 'i, cI> = self.interner.alloc(result);
        _monouts.interfaces_without_methods.insert(result_ref.instantiated_interface.id, result_ref);
        assert_eq!(result_ref.instantiated_interface.id, *_new_id_c);
        // Scala second `if (opts.sanityCheck) { ... }` collector check omitted (same parity gap).
    }
}
/*
Guardian: temp-disable: SPDMX — Architect-approved parity gap (already used in collapse_and_translate_struct_definition, collapse_and_translate_interface_definition, translate_method's exports/externs closures, translate_collapsed_struct_definition): Scala's `if (opts.sanityCheck) { vassert(Collector.all(..., {case KindPlaceholderNameT(_) => }).isEmpty) }` sanity checks for typing-pass KindPlaceholderNameT are vacuous on the I-side AST (statically typed, can't structurally hold a placeholder). Both sanity-check blocks here are the same pattern. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-2467-1780136111536/hook-2467/translate_collapsed_interface_definition--2422.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  // This inner function is conceptually from the interface's own perspective. That's why it's
  // taking in a collapsed id.
  def translateCollapsedInterfaceDefinition(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    newIdC: IdI[cI, IInterfaceNameI[cI]],
    interfaceDefT: InterfaceDefinitionT):
  Unit = {
    if (monouts.interfaceToMutability.contains(newIdC)) {
      return
    }

    val InterfaceDefinitionT(templateName, instantiatedCitizen, ref, attributes, weakable, mutabilityT, _, internalMethods) = interfaceDefT

    vassert(!monouts.interfaceToImplToAbstractPrototypeToOverride.contains(newIdC))
    monouts.interfaceToImplToAbstractPrototypeToOverride.put(newIdC, mutable.HashMap())

    vassert(!monouts.interfaceToAbstractFuncToVirtualIndex.contains(newIdC))
    monouts.interfaceToAbstractFuncToVirtualIndex.put(newIdC, mutable.HashMap())

    vassert(!monouts.interfaceToImpls.contains(newIdC))
    monouts.interfaceToImpls.put(newIdC, mutable.HashSet())

    if (opts.sanityCheck) {
      vassert(Collector.all(newIdC, { case KindPlaceholderNameT(_) => }).isEmpty)
    }

    val perspectiveRegionT = RegionT(DefaultRegionT)
    // interfaceDefT.instantiatedCitizen.id.localName.templateArgs.last match {
    //   case PlaceholderTemplataT(IdT(packageCoord, initSteps, r @ RegionPlaceholderNameT(_, _, _, _)), RegionTemplataType()) => {
    //     IdT(packageCoord, initSteps, r)
    //   }
    //   case _ => vwat()
    // }

    val mutability = ITemplataI.expectMutabilityTemplata(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, mutabilityT)).mutability

    vassert(!monouts.interfaceToMutability.contains(newIdC))
    monouts.interfaceToMutability.put(newIdC, mutability)

    //    val currentPureHeight = vimpl()

    val newInterfaceIT = InterfaceIT(newIdC)

    val result =
      InterfaceDefinitionI(
        newInterfaceIT,
        attributes.map(translateCitizenAttribute),
        weakable,
        mutability,
        Map(),
        Map(),
        Vector())

    monouts.interfacesWithoutMethods.put(newIdC, result)

    vassert(result.instantiatedCitizen.id == newIdC)

    if (opts.sanityCheck) {
      vassert(Collector.all(result.instantiatedInterface, { case KindPlaceholderNameT(_) => }).isEmpty)
    }

    result
  }
*/
// mig: fn translate_function_header
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_function_header(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, perspective_region_t: &RegionT, header_t: &FunctionHeaderT<'s, 't>) -> FunctionHeaderI<'s, 'i> {
        let new_id_s =
            self.translate_function_id(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &header_t.id);
        let new_id_c =
            region_collapser_individual::collapse_id(self.interner, &new_id_s, |x| INameI::from(region_collapser_individual::collapse_function_name(self.interner, &IFunctionNameI::try_from(*x).unwrap())));

        let return_it =
            self.translate_coord(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &header_t.return_type);
        let return_ic = region_collapser_individual::collapse_coord(self.interner, &return_it.coord);

        let result =
            FunctionHeaderI {
                id: new_id_c,
                attributes: self.interner.alloc_slice_from_vec(header_t.attributes.iter().map(|a| Self::translate_function_attribute(a)).collect()),
                params: self.interner.alloc_slice_from_vec(header_t.params.iter().map(|p| self.translate_parameter(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, p)).collect()),
                return_type: return_ic,
            };

        result
    }
}
/*
  def translateFunctionHeader(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    header: FunctionHeaderT):
  FunctionHeaderI = {
    val FunctionHeaderT(fullName, attributes, params, returnType, maybeOriginFunctionTemplata) = header

    val newIdS =
      translateFunctionId(
        denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, fullName)
    val newIdC =
      RegionCollapserIndividual.collapseId[IFunctionNameI[sI], IFunctionNameI[cI]](
        newIdS,
        x => RegionCollapserIndividual.collapseFunctionName( x))

    val returnIT =
      translateCoord(
        denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, returnType)
    val returnIC = RegionCollapserIndividual.collapseCoord(returnIT.coord)

    val result =
      FunctionHeaderI(
        newIdC,
        attributes.map(translateFunctionAttribute),
        params.map(translateParameter(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
        returnIC)

    result
  }
*/
// mig: fn translate_function_attribute
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_function_attribute(x: &IFunctionAttributeT<'s>) -> IFunctionAttributeI<'s> {
        match x {
            IFunctionAttributeT::UserFunction => IFunctionAttributeI::UserFunctionI,
            IFunctionAttributeT::Pure => panic!("Unimplemented: translate_function_attribute Pure"),
            IFunctionAttributeT::Extern(e) => IFunctionAttributeI::ExternI(crate::instantiating::ast::ast::ExternI { package_coord: e.package_coord }),
            _ => panic!("Unimplemented: translate_function_attribute other"),
        }
    }
}
/*
  def translateFunctionAttribute(x: IFunctionAttributeT): IFunctionAttributeI = {
    x match {
      case UserFunctionT => UserFunctionI
      case PureT => PureI
      case ExternT(packageCoord) => ExternI(packageCoord)
      case other => vimpl(other)
    }
  }
*/
// mig: fn translate_citizen_attribute
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_citizen_attribute(x: &crate::typing::ast::ast::ICitizenAttributeT<'s>) -> crate::instantiating::ast::ast::ICitizenAttributeI<'s> {
        match x {
            crate::typing::ast::ast::ICitizenAttributeT::Sealed => crate::instantiating::ast::ast::ICitizenAttributeI::SealedI,
            crate::typing::ast::ast::ICitizenAttributeT::Extern(extern_t) => crate::instantiating::ast::ast::ICitizenAttributeI::ExternI(crate::instantiating::ast::ast::ExternI { package_coord: extern_t.package_coord }),
        }
    }
}
/*
  def translateCitizenAttribute(x: ICitizenAttributeT): ICitizenAttributeI = {
    x match {
      case SealedT => SealedI
      case ExternT(packageCoord) => ExternI(packageCoord)
      case other => vimpl(other)
    }
  }
*/
// mig: fn translate_collapsed_function
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_collapsed_function(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, desired_prototype_c: &PrototypeI<'s, 'i, cI>, function_t: &FunctionDefinitionT<'s, 't>) -> &'i FunctionDefinitionI<'s, 'i> {
        if self.opts.sanity_check {
            collector::all_in_substitutions(substitutions, &|node| -> Option<()> {
                if let NodeRefI::Templata(ITemplataI::Region(r)) = node {
                    if r.pure_height > 0 { panic!("vwat: substitutions contains RegionTemplataI(pure_height > 0)") }
                }
                None
            });
        }

        let perspective_region_t = RegionT { region: IRegionT::Default };
          // functionT.header.id.localName.templateArgs.last match {
          //   case PlaceholderTemplataT(IdT(packageCoord, initSteps, r @ RegionPlaceholderNameT(_, _, _, _)), RegionTemplataType()) => {
          //     IdT(packageCoord, initSteps, r)
          //   }
          //   case _ => vwat()
          // }

        let function_id_s =
            self.translate_function_id(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, &perspective_region_t, &function_t.header.id);
        let function_id_c =
            region_collapser_individual::collapse_function_id(self.interner, &function_id_s);

        match monouts.functions.get(&function_id_c) {
            Some(func) => return *func,
            None => {}
        }

        let new_header = self.translate_function_header(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, &perspective_region_t, function_t.header);

        if new_header.to_prototype(self.interner) != *desired_prototype_c {
            self.translate_function_header(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, &perspective_region_t, function_t.header);
            panic!("vfail");
        }

        let (_body_subjective_it, body_ce) =
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
}
/*
  def translateCollapsedFunction(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    // For doublechecking we're getting the actual function we requested
    desiredPrototypeC: PrototypeI[cI],
    functionT: FunctionDefinitionT):
  FunctionDefinitionI = {
    val FunctionDefinitionT(headerT, _, bodyT) = functionT

    val FunctionHeaderT(fullName, attributes, params, returnType, maybeOriginFunctionTemplata) = headerT

    if (opts.sanityCheck) {
      Collector.all(substitutions.toVector, {
        case RegionTemplataI(x) if x > 0 => vwat()
      })
    }

    val perspectiveRegionT = RegionT(DefaultRegionT)
      // functionT.header.id.localName.templateArgs.last match {
      //   case PlaceholderTemplataT(IdT(packageCoord, initSteps, r @ RegionPlaceholderNameT(_, _, _, _)), RegionTemplataType()) => {
      //     IdT(packageCoord, initSteps, r)
      //   }
      //   case _ => vwat()
      // }

    val functionIdS =
      translateFunctionId(
        denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, fullName)
    val functionIdC =
      RegionCollapserIndividual.collapseFunctionId(functionIdS)

    monouts.functions.get(functionIdC) match {
      case Some(func) => return func
      case None =>
    }

    val newHeader = translateFunctionHeader(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, headerT)

    if (newHeader.toPrototype != desiredPrototypeC) {
      translateFunctionHeader(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, headerT)
      vfail()
    }

    val (bodySubjectiveIT, bodyCE) =
      translateRefExpr(
        denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, bodyT)

    val result = FunctionDefinitionI(newHeader, Map(), Map(), bodyCE)

    monouts.functions.put(result.header.id, result)
    result
  }
*/
// mig: fn translate_local_variable
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_local_variable(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, perspective_region_t: &RegionT, variable: &ILocalVariableT<'s, 't>) -> (CoordI<'s, 'i, sI>, ILocalVariableI<'s, 'i>) {
        match variable {
            ILocalVariableT::Reference(r) => {
                let (coord, local) =
                    self.translate_reference_local_variable(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, r);
                (coord, ILocalVariableI::ReferenceLocalVariableI(self.interner.alloc(local)))
            }
            ILocalVariableT::Addressible(a) => {
                let (coord, local) =
                    self.translate_addressible_local_variable(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, a);
                (coord, ILocalVariableI::AddressibleLocalVariableI(self.interner.alloc(local)))
            }
        }
    }
}
/*
  def translateLocalVariable(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    variable: ILocalVariableT):
  // Returns subjective coord and the local var
  (CoordI[sI], ILocalVariableI) = {
    variable match {
      case r @ ReferenceLocalVariableT(_, _, _) => {
        translateReferenceLocalVariable(
          denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, r)
      }
      case a @ AddressibleLocalVariableT(_, _, _) => {
        translateAddressibleLocalVariable(
          denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, a)
      }
    }
  }
*/
// mig: fn translate_reference_local_variable
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_reference_local_variable(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, perspective_region_t: &RegionT, variable: &ReferenceLocalVariableT<'s, 't>) -> (CoordI<'s, 'i, sI>, ReferenceLocalVariableI<'s, 'i>) {
        let ReferenceLocalVariableT { name: id, variability, coord } = variable;
        let coord_s =
            self.translate_coord(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, coord);
        let var_name_s = Self::translate_var_name(self.interner, id);
        let local_c =
            ReferenceLocalVariableI {
                name: region_collapser_individual::collapse_var_name(self.interner, &var_name_s),
                variability: Self::translate_variability(variability),
                collapsed_coord: region_collapser_individual::collapse_coord(self.interner, &coord_s.coord),
            };
        (coord_s.coord, local_c)
    }
}
/*
  def translateReferenceLocalVariable(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    variable: ReferenceLocalVariableT):
  // Returns subjective coord and the local var
  (CoordI[sI], ReferenceLocalVariableI) = {
    val ReferenceLocalVariableT(id, variability, coord) = variable
    val coordS =
      translateCoord(
        denizenName,
        denizenBoundToDenizenCallerSuppliedThing,
        substitutions,
        perspectiveRegionT,
        coord)
    val varNameS = translateVarName(id)
    val localC =
      ReferenceLocalVariableI(
        RegionCollapserIndividual.collapseVarName(varNameS),
        translateVariability(variability),
        RegionCollapserIndividual.collapseCoord(coordS.coord))
    (coordS.coord, localC)
  }
*/
// mig: fn translate_addressible_local_variable
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_addressible_local_variable(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _variable: &AddressibleLocalVariableT<'s, 't>) -> (CoordI<'s, 'i, sI>, AddressibleLocalVariableI<'s, 'i>) {
        let AddressibleLocalVariableT { name: id, variability, coord } = _variable;
        let coord_s = self.translate_coord(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, &RegionT { region: IRegionT::Default }, coord);
        let var_s = Self::translate_var_name(self.interner, id);
        let local_c = AddressibleLocalVariableI {
            name: crate::instantiating::region_collapser_individual::collapse_var_name(self.interner, &var_s),
            variability: Self::translate_variability(variability),
            collapsed_coord: crate::instantiating::region_collapser_individual::collapse_coord(self.interner, &coord_s.coord),
        };
        (coord_s.coord, local_c)
    }
}
/*
  def translateAddressibleLocalVariable(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    variable: AddressibleLocalVariableT):
  // Returns subjective coord and the local var
  (CoordI[sI], AddressibleLocalVariableI) = {
    val AddressibleLocalVariableT(id, variability, coord) = variable
    val coordS =
      translateCoord(
        denizenName,
        denizenBoundToDenizenCallerSuppliedThing,
        substitutions,
        RegionT(DefaultRegionT),
        coord)
    val varS = translateVarName(id)
    val localC =
      AddressibleLocalVariableI(
        RegionCollapserIndividual.collapseVarName(varS),
        translateVariability(variability),
        RegionCollapserIndividual.collapseCoord(coordS.coord))
    (coordS.coord, localC)
  }
*/
// mig: fn translate_addr_expr
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_addr_expr(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _expr: &AddressExpressionTE<'s, 't>) -> (CoordI<'s, 'i, sI>, AddressExpressionIE<'s, 'i, cI>) {
        match _expr {
            AddressExpressionTE::LocalLookup(ll) => {
                let crate::typing::ast::expressions::LocalLookupTE { range: _range, local_variable: local_variable_t } = **ll;
                let (local_subjective_it, local_variable_i) = self.translate_local_variable(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &local_variable_t);
                let result_subjective_it = local_subjective_it;
                let result_ce = AddressExpressionIE::LocalLookup(self.interner.bump().alloc(crate::instantiating::ast::expressions::LocalLookupIE {
                    local_variable: local_variable_i,
                    result: crate::instantiating::region_collapser_individual::collapse_coord(self.interner, &result_subjective_it),
                }));
                (result_subjective_it, result_ce)
            }
            AddressExpressionTE::ReferenceMemberLookup(rml) => {
                let crate::typing::ast::expressions::ReferenceMemberLookupTE { range, struct_expr: struct_expr_t, member_name: member_name_t, member_reference: member_coord_t, variability } = **rml;
                let (_struct_subjective_it, struct_ce) =
                    self.translate_ref_expr(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &struct_expr_t);
                let member_name = Self::translate_var_name(self.interner, &member_name_t);
                let member_coord_s = self.translate_coord(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &member_coord_t);
                let result_subjective_it = member_coord_s;
                let result_ce = AddressExpressionIE::ReferenceMemberLookup(self.interner.bump().alloc(crate::instantiating::ast::expressions::ReferenceMemberLookupIE {
                    range,
                    struct_expr: struct_ce,
                    member_name: crate::instantiating::region_collapser_individual::collapse_var_name(self.interner, &member_name),
                    member_reference: crate::instantiating::region_collapser_individual::collapse_coord(self.interner, &result_subjective_it.coord),
                    variability: Self::translate_variability(&variability),
                }));
                (result_subjective_it.coord, result_ce)
            }
            AddressExpressionTE::StaticSizedArrayLookup(s) => {
                let crate::typing::ast::expressions::StaticSizedArrayLookupTE { range, array_expr: array_expr_t, array_type: _, index_expr: index_expr_t, element_type: element_type_t, variability } = **s;
                let (_array_subjective_it, array_ce) =
                    self.translate_ref_expr(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &array_expr_t);
                let element_type_s = self.translate_coord(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &element_type_t).coord;
                let (_index_it, index_ce) =
                    self.translate_ref_expr(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &index_expr_t);
                let result_coord = CoordI { ownership: element_type_s.ownership, kind: element_type_s.kind };
                let result_ce = AddressExpressionIE::StaticSizedArrayLookup(self.interner.alloc(crate::instantiating::ast::expressions::StaticSizedArrayLookupIE {
                    range,
                    array_expr: array_ce,
                    index_expr: index_ce,
                    element_type: region_collapser_individual::collapse_coord(self.interner, &result_coord),
                    variability: Self::translate_variability(&variability),
                }));
                (result_coord, result_ce)
            }
            AddressExpressionTE::AddressMemberLookup(a) => {
                let crate::typing::ast::expressions::AddressMemberLookupTE { range: _range, struct_expr, member_name, result_type2, variability } = **a;
                let (_struct_it, struct_ce) = self.translate_ref_expr(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &struct_expr);
                let var_name_s = Self::translate_var_name(self.interner, &member_name);
                let result_it = self.translate_coord(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &result_type2);
                let variability_c = Self::translate_variability(&variability);
                let result_ce = AddressExpressionIE::AddressMemberLookup(self.interner.alloc(crate::instantiating::ast::expressions::AddressMemberLookupIE {
                    struct_expr: struct_ce,
                    member_name: crate::instantiating::region_collapser_individual::collapse_var_name(self.interner, &var_name_s),
                    member_reference: crate::instantiating::region_collapser_individual::collapse_coord(self.interner, &result_it.coord),
                    variability: variability_c,
                }));
                (result_it.coord, result_ce)
            }
            AddressExpressionTE::RuntimeSizedArrayLookup(_) => panic!("Unimplemented: translate_addr_expr RuntimeSizedArrayLookup"),
        }
    }
}
/*
  def translateAddrExpr(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    expr: AddressExpressionTE):
  // Returns the subjective coord (see HCCSCS) and the expression.
  (CoordI[sI], AddressExpressionIE) = {
    expr match {
      case LocalLookupTE(range, localVariableT) => {
//        // We specifically don't *translate* LocalLookupTE.localVariable because we can't translate
//        // it properly from here with our current understandings of the regions' mutabilities, we
//        // need its original type. See CTOTFIPB.
//        val localVariable = env.lookupOriginalTranslatedVariable(localVariableT.name)
        val (localSubjectiveIT, localVariableI) =
          translateLocalVariable(
            denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, localVariableT)
//
//        val sourceRegion =
//          ITemplataI.expectRegionTemplata(
//            translateTemplata(
//              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, vimpl()))

//        val subjectiveResultIT =
//          CoordI(
//            (localVariableI.coord.ownership, coordRegionIsMutable(substitutions, perspectiveRegionT, localVariableT.coord)) match {
//              case (OwnT, _) => OwnI
//              case other => vimpl(other)
//            },
//            localVariableI.coord.kind)

        val resultSubjectiveIT = localSubjectiveIT
        val resultCE =
          LocalLookupIE(
            localVariableI,
            RegionCollapserIndividual.collapseCoord(resultSubjectiveIT))
        (resultSubjectiveIT, resultCE)
      }
      case ReferenceMemberLookupTE(range, structExprT, memberNameT, memberCoordT, variability) => {
        val (structSubjectiveIT, structCE) =
          translateRefExpr(
            denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, structExprT)
        val memberName = translateVarName(memberNameT)

        val memberCoordS =
          translateCoord(
            denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, memberCoordT)

        val resultSubjectiveIT = memberCoordS
        val resultCE =
          ReferenceMemberLookupIE(
            range,
            structCE,
            RegionCollapserIndividual.collapseVarName(memberName),
            RegionCollapserIndividual.collapseCoord(resultSubjectiveIT.coord),
            translateVariability(variability))
        (resultSubjectiveIT.coord, resultCE)
      }
      case StaticSizedArrayLookupTE(range, arrayExprT, arrayType, indexExprT, elementTypeT, variability) => {
        val (arraySubjectiveIT, arrayCE) =
          translateRefExpr(
            denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, arrayExprT)

        val elementTypeS =
          translateCoord(
            denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, elementTypeT).coord

        val (indexIT, indexCE) =
          translateRefExpr(
            denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, indexExprT)

        val resultCoord = CoordI(elementTypeS.ownership, elementTypeS.kind)

        val resultCE =
          StaticSizedArrayLookupIE(
            range,
            arrayCE,
            indexCE,
            RegionCollapserIndividual.collapseCoord(resultCoord),
            translateVariability(variability))
        (resultCoord, resultCE)
      }
      case AddressMemberLookupTE(range, structExpr, memberName, resultType2, variability) => {
        val (structIT, structCE) =
          translateRefExpr(
            denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, structExpr)
        val varNameS = translateVarName(memberName)
        val resultIT =
          translateCoord(
            denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, resultType2)
        val variabilityC = translateVariability(variability)

        val resultCE =
          AddressMemberLookupIE(
            structCE,
            RegionCollapserIndividual.collapseVarName(varNameS),
            RegionCollapserIndividual.collapseCoord(resultIT.coord),
            variabilityC)
        (resultIT.coord, resultCE)
      }
      case RuntimeSizedArrayLookupTE(range, arrayExpr, rsaTT, indexExpr, variability) => {
        val (arrayIT, arrayCE) =
          translateRefExpr(
            denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, arrayExpr)
        val rsaIT =
          translateRuntimeSizedArray(
            denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, rsaTT)
        val (indexIT, indexCE) =
          translateRefExpr(
            denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, indexExpr)
        val variabilityC = translateVariability(variability)

        // We can't just say rsaIT.elementType here because that's the element from the array's own
        // perspective.
        val elementIT =
          translateCoord(
            denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, rsaTT.elementType)

        val resultIT = elementIT
        val resultCE =
          RuntimeSizedArrayLookupIE(
            arrayCE, indexCE, RegionCollapserIndividual.collapseCoord(elementIT.coord), variabilityC)
        (resultIT.coord, resultCE)
      }
      case other => vimpl(other)
    }
  }
*/
// mig: fn translate_expr
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_expr(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, perspective_region_t: &RegionT, expr: &ExpressionTE<'s, 't>) -> (CoordI<'s, 'i, sI>, ExpressionIE<'s, 'i, cI>) {
        match expr {
            ExpressionTE::Reference(r) => {
                let (it, ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, r);
                (it, ExpressionIE::Reference(ce))
            }
            ExpressionTE::Address(a) => {
                let (it, ce) = self.translate_addr_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, a);
                (it, ExpressionIE::Address(ce))
            }
        }
    }
}
/*
  def translateExpr(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,

    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    expr: ExpressionT):
  // Returns the subjective coord (see HCCSCS) and the expression.
  (CoordI[sI], ExpressionI) = {
    expr match {
      case r : ReferenceExpressionTE => {
        translateRefExpr(
          denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, r)
      }
      case a : AddressExpressionTE => {
        translateAddrExpr(
          denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, a)
      }
    }
  }
*/
// mig: fn translate_ref_expr
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_ref_expr(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, perspective_region_t: &RegionT, expr: &ReferenceExpressionTE<'s, 't>) -> (CoordI<'s, 'i, sI>, ReferenceExpressionIE<'s, 'i, cI>) {
        let _denizen_template_name = Compiler::get_template(self.typing_interner, *denizen_name);
        match expr {
            ReferenceExpressionTE::LetAndLend(lal) => {
                let crate::typing::ast::expressions::LetAndLendTE { variable, expr: source_expr_t, target_ownership: outer_ownership_t } = **lal;
                let (source_subjective_it, source_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &source_expr_t);
                let result_ownership_c =
                    Self::translate_ownership(
                        substitutions,
                        perspective_region_t,
                        &Self::compose_ownerships_second(&outer_ownership_t, &source_subjective_it.ownership),
                        &source_expr_t.result().coord.region);
                let result_it = CoordI { ownership: result_ownership_c, kind: source_subjective_it.kind };
                let (_local_it, local_i) =
                    self.translate_local_variable(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &variable);
                let result_ce = ReferenceExpressionIE::LetAndLend(self.interner.bump().alloc(LetAndLendIE {
                    variable: local_i,
                    expr: source_ce,
                    target_ownership: result_ownership_c,
                    result: region_collapser_individual::collapse_coord(self.interner, &result_it),
                }));
                (result_it, result_ce)
            }
            ReferenceExpressionTE::LockWeak(_) => panic!("Unimplemented: translate_ref_expr LockWeak"),
            ReferenceExpressionTE::BorrowToWeak(_) => panic!("Unimplemented: translate_ref_expr BorrowToWeak"),
            ReferenceExpressionTE::LetNormal(l) => {
                let (_inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &l.expr);
                let (_local_it, local_i) =
                    self.translate_local_variable(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &l.variable);
                // env.addTranslatedVariable(variableT.name, vimpl(translatedVariable))
                let subjective_result_it = CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::VoidIT(VoidIT { _marker: std::marker::PhantomData }) };
                let expr_ce = ReferenceExpressionIE::LetNormal(self.interner.alloc(LetNormalIE {
                    variable: local_i,
                    expr: inner_ce,
                    result: region_collapser_individual::collapse_coord(self.interner, &subjective_result_it),
                }));
                (subjective_result_it, expr_ce)
            }
            ReferenceExpressionTE::Unlet(u) => {
                let (local_it, local_ce) =
                    self.translate_local_variable(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &u.variable);
                let result_it = local_it;
                // val local = env.lookupOriginalTranslatedVariable(variable.name)
                let result_ce = ReferenceExpressionIE::Unlet(self.interner.alloc(UnletIE {
                    variable: local_ce,
                    result: region_collapser_individual::collapse_coord(self.interner, &result_it),
                }));
                (result_it, result_ce)
            }
            ReferenceExpressionTE::Discard(d) => {
                let (_inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &d.expr);
                let result_ce = ReferenceExpressionIE::Discard(self.interner.alloc(DiscardIE { expr: inner_ce }));
                (CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::VoidIT(VoidIT { _marker: std::marker::PhantomData }) }, result_ce)
            }
            ReferenceExpressionTE::Defer(d) => {
                let crate::typing::ast::expressions::DeferTE { inner_expr, deferred_expr } = **d;
                let (inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &inner_expr);
                let (_deferred_it, deferred_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &deferred_expr);
                let result_it = inner_it;
                let result_ce = ReferenceExpressionIE::Defer(self.interner.bump().alloc(DeferIE {
                    inner_expr: inner_ce,
                    deferred_expr: deferred_ce,
                    result: region_collapser_individual::collapse_coord(self.interner, &result_it),
                }));
                (result_it, result_ce)
            }
            ReferenceExpressionTE::If(if_te) => {
                let (_condition_it, condition_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &if_te.condition);
                let (then_it, then_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &if_te.then_call);
                let (else_it, else_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &if_te.else_call);
                let result_it =
                    match (then_it, else_it) {
                        (a, b) if a == b => a,
                        (a, CoordI { kind: KindIT::NeverIT(_), .. }) => a,
                        (CoordI { kind: KindIT::NeverIT(_), .. }, b) => b,
                        _ => panic!("vwat"),
                    };
                let result_ce = ReferenceExpressionIE::If(self.interner.alloc(IfIE {
                    condition: condition_ce,
                    then_call: then_ce,
                    else_call: else_ce,
                    result: region_collapser_individual::collapse_coord(self.interner, &result_it),
                }));
                (result_it, result_ce)
            }
            ReferenceExpressionTE::While(w) => {
                let (inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &w.block.inner);

                // While loops must always produce void.
                // If we want a foreach/map/whatever construct, the loop should instead
                // add things to a list inside; WhileIE shouldnt do it for it.
                let result_it =
                    match inner_it {
                        CoordI { kind: KindIT::VoidIT(_), .. } => inner_it,
                        CoordI { kind: KindIT::NeverIT(NeverIT { from_break: true, .. }), .. } => CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::VoidIT(VoidIT { _marker: std::marker::PhantomData }) },
                        CoordI { kind: KindIT::NeverIT(NeverIT { from_break: false, .. }), .. } => inner_it,
                        _ => panic!("vwat"),
                    };
                let result_ce =
                    ReferenceExpressionIE::While(self.interner.alloc(WhileIE {
                        block: BlockIE { inner: inner_ce, result: region_collapser_individual::collapse_coord(self.interner, &inner_it) },
                        result: region_collapser_individual::collapse_coord(self.interner, &result_it),
                    }));
                (result_it, result_ce)
            }
            ReferenceExpressionTE::Mutate(m) => {
                let crate::typing::ast::expressions::MutateTE { destination_expr: destination_tt, source_expr } = **m;
                let (destination_it, destination_ce) = self.translate_addr_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &destination_tt);
                let (_source_it, source_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &source_expr);
                let result_it = destination_it;
                let result_ce = ReferenceExpressionIE::Mutate(self.interner.bump().alloc(crate::instantiating::ast::expressions::MutateIE {
                    destination_expr: destination_ce,
                    source_expr: source_ce,
                    result: region_collapser_individual::collapse_coord(self.interner, &result_it),
                }));
                (result_it, result_ce)
            }
            ReferenceExpressionTE::Restackify(_) => panic!("Unimplemented: translate_ref_expr Restackify"),
            ReferenceExpressionTE::Transmigrate(_) => panic!("Unimplemented: translate_ref_expr Transmigrate"),
            ReferenceExpressionTE::Return(r) => {
                let (_inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &r.source_expr);
                let result_ce = ReferenceExpressionIE::Return(self.interner.alloc(ReturnIE {
                    source_expr: inner_ce,
                }));
                (CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::NeverIT(NeverIT { from_break: false, _marker: std::marker::PhantomData }) }, result_ce)
            }
            ReferenceExpressionTE::Break(_) => {
                let result_ce = ReferenceExpressionIE::Break(self.interner.alloc(BreakIE(std::marker::PhantomData)));
                (CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::NeverIT(NeverIT { from_break: true, _marker: std::marker::PhantomData }) }, result_ce)
            }
            ReferenceExpressionTE::Block(b) => {
                let (inner_it, inner_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &b.inner);
                let result_it = inner_it;
                let result_ce = ReferenceExpressionIE::Block(self.interner.alloc(BlockIE {
                    inner: inner_ce,
                    result: region_collapser_individual::collapse_coord(self.interner, &result_it),
                }));
                (result_it, result_ce)
            }
            ReferenceExpressionTE::Pure(_) => panic!("Unimplemented: translate_ref_expr Pure"),
            ReferenceExpressionTE::Consecutor(c) => {
                let result_tt = c.result().coord;
                let result_it =
                    self.translate_coord(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &result_tt)
                        .coord;
                let inners_ce: Vec<_> =
                    c.exprs.iter().map(|inner_te| {
                        self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, inner_te).1
                    }).collect();
                let result_ce = ReferenceExpressionIE::Consecutor(self.interner.alloc(ConsecutorIE {
                    exprs: self.interner.alloc_slice_from_vec(inners_ce),
                    result: region_collapser_individual::collapse_coord(self.interner, &result_it),
                }));
                (result_it, result_ce)
            }
            ReferenceExpressionTE::Tuple(t) => {
                let crate::typing::ast::expressions::TupleTE { elements, result_reference } = **t;
                let elements_ce: Vec<_> = elements.iter().map(|element_te| {
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, element_te).1
                }).collect();
                let result_it =
                    self.translate_coord(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &result_reference)
                        .coord;
                (result_it, ReferenceExpressionIE::Tuple(self.interner.bump().alloc(TupleIE {
                    elements: self.interner.alloc_slice_from_vec(elements_ce),
                    result: region_collapser_individual::collapse_coord(self.interner, &result_it),
                })))
            }
            ReferenceExpressionTE::StaticArrayFromValues(s) => {
                let crate::typing::ast::expressions::StaticArrayFromValuesTE { elements, result_reference, array_type } = **s;
                let result_it =
                    self.translate_coord(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &result_reference)
                        .coord;
                let elements_ce: Vec<_> = elements.iter().map(|element_te| {
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, element_te).1
                }).collect();
                let ssa_tt = self.translate_static_sized_array(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &array_type);
                let result_ce = ReferenceExpressionIE::StaticArrayFromValues(self.interner.alloc(crate::instantiating::ast::expressions::StaticArrayFromValuesIE {
                    elements: self.interner.alloc_slice_from_vec(elements_ce),
                    result_reference: region_collapser_individual::collapse_coord(self.interner, &result_it),
                    array_type: region_collapser_individual::collapse_static_sized_array(self.interner, &ssa_tt),
                }));
                (result_it, result_ce)
            }
            ReferenceExpressionTE::ArraySize(_) => panic!("Unimplemented: translate_ref_expr ArraySize"),
            ReferenceExpressionTE::IsSameInstance(_) => panic!("Unimplemented: translate_ref_expr IsSameInstance"),
            ReferenceExpressionTE::AsSubtype(_) => panic!("Unimplemented: translate_ref_expr AsSubtype"),
            ReferenceExpressionTE::VoidLiteral(_) => {
                (CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::VoidIT(VoidIT { _marker: std::marker::PhantomData }) },
                 ReferenceExpressionIE::VoidLiteral(self.interner.alloc(VoidLiteralIE(std::marker::PhantomData))))
            }
            ReferenceExpressionTE::ConstantInt(c) => {
                let result_ce = ReferenceExpressionIE::ConstantInt(self.interner.alloc(ConstantIntIE {
                    value: expect_integer_templata(self.translate_templata(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &c.value)).value,
                    bits: c.bits,
                    _marker: std::marker::PhantomData,
                }));
                (CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::IntIT(IntIT { bits: c.bits, _marker: std::marker::PhantomData }) }, result_ce)
            }
            ReferenceExpressionTE::ConstantBool(c) => {
                let result_ce = ReferenceExpressionIE::ConstantBool(self.interner.alloc(ConstantBoolIE { _marker: std::marker::PhantomData, value: c.value }));
                (CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::BoolIT(BoolIT { _marker: std::marker::PhantomData }) }, result_ce)
            }
            ReferenceExpressionTE::ConstantStr(c) => {
                let result_ce = ReferenceExpressionIE::ConstantStr(self.interner.alloc(ConstantStrIE { _marker: std::marker::PhantomData, value: c.value.0 }));
                (CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::StrIT(crate::instantiating::ast::types::StrIT { _marker: std::marker::PhantomData }) }, result_ce)
            }
            ReferenceExpressionTE::ConstantFloat(c) => {
                let result_ce = ReferenceExpressionIE::ConstantFloat(self.interner.alloc(ConstantFloatIE { _marker: std::marker::PhantomData, value: c.value }));
                (CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::FloatIT(FloatIT { _marker: std::marker::PhantomData }) }, result_ce)
            }
            ReferenceExpressionTE::ArgLookup(al) => {
                let crate::typing::ast::expressions::ArgLookupTE { param_index, coord: reference } = **al;
                let type_s =
                    self.translate_coord(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &reference)
                        .coord;
                let result_ce = ReferenceExpressionIE::ArgLookup(self.interner.alloc(crate::instantiating::ast::expressions::ArgLookupIE { param_index, coord: region_collapser_individual::collapse_coord(self.interner, &type_s) }));
                (type_s, result_ce)
            }
            ReferenceExpressionTE::ArrayLength(_) => panic!("Unimplemented: translate_ref_expr ArrayLength"),
            ReferenceExpressionTE::InterfaceFunctionCall(_) => panic!("Unimplemented: translate_ref_expr InterfaceFunctionCall"),
            ReferenceExpressionTE::ExternFunctionCall(efc) => {
                let crate::typing::ast::expressions::ExternFunctionCallTE { prototype2, args } = **efc;
                let (prototype_i, prototype_c) = self.translate_prototype(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, prototype2);
                let args_ce: Vec<ReferenceExpressionIE<'s, 'i, cI>> = args.iter().map(|arg_te| self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, arg_te).1).collect();
                let result_it = prototype_i.return_type;
                let result_ce = ReferenceExpressionIE::ExternFunctionCall(self.interner.bump().alloc(crate::instantiating::ast::expressions::ExternFunctionCallIE { prototype2: prototype_c, args: self.interner.bump().alloc_slice_fill_iter(args_ce.into_iter()), result: prototype_c.return_type }));
                match prototype2.id.local_name {
                    INameT::ExternFunction(crate::typing::names::names::ExternFunctionNameT { human_name, template_args, .. }) if !template_args.is_empty() => {
                        let num_inherited = self.hinputs.function_externs.iter().find(|fe| {
                            fe.prototype.id.package_coord == prototype2.id.package_coord
                                && fe.prototype.id.init_steps == prototype2.id.init_steps
                                && match fe.prototype.id.local_name {
                                    INameT::ExternFunction(crate::typing::names::names::ExternFunctionNameT { human_name: hn, .. }) => hn == human_name,
                                    _ => false,
                                }
                        })
                        .and_then(|fe| fe.generic_parameter_inheritance.as_ref().map(|i| i.num_inherited_generic_parameters))
                        .unwrap_or(0);
                        monouts.function_externs.push(FunctionExternI {
                            prototype: self.interner.intern_prototype_ci(PrototypeIValI { id: prototype_c.id, return_type: prototype_c.return_type }),
                            num_inherited_generic_parameters: num_inherited,
                        });
                    }
                    _ => {}
                }
                (result_it, result_ce)
            }
            ReferenceExpressionTE::FunctionCall(fc) => {
                let crate::typing::ast::expressions::FunctionCallTE { callable: prototype_t, args, return_type: _return_type } = fc;
                let inners_ce: Vec<crate::instantiating::ast::expressions::ReferenceExpressionIE<'s, 'i, cI>> = args.iter().map(|arg_te| {
                    let (_arg_it, arg_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, arg_te);
                    arg_ce
                }).collect();
                let (prototype_i, prototype_c) = self.translate_prototype(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, prototype_t);
                let return_coord_it = prototype_i.return_type;
                let return_coord_ct = crate::instantiating::region_collapser_individual::collapse_coord(self.interner, &return_coord_it);
                let result_ce = crate::instantiating::ast::expressions::ReferenceExpressionIE::FunctionCall(self.interner.alloc(crate::instantiating::ast::expressions::FunctionCallIE {
                    callable: prototype_c,
                    args: self.interner.bump().alloc_slice_fill_iter(inners_ce.into_iter()),
                    result: crate::instantiating::region_collapser_individual::collapse_coord(self.interner, &return_coord_it),
                }));
                let _ = return_coord_ct;
                (return_coord_it, result_ce)
            }
            ReferenceExpressionTE::Reinterpret(_) => panic!("Unimplemented: translate_ref_expr Reinterpret"),
            ReferenceExpressionTE::Construct(c) => {
                let crate::typing::ast::expressions::ConstructTE { struct_tt, result_reference, args } = **c;
                let result_it = self.translate_coord(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &result_reference).coord;
                let args_ce: Vec<crate::instantiating::ast::expressions::ExpressionIE<'s, 'i, cI>> = args.iter().map(|arg_te| {
                    self.translate_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, arg_te).1
                }).collect();
                let bound_args = self.translate_bound_args_for_callee(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &self.hinputs.get_instantiation_bound_args(struct_tt.id));
                let struct_it = self.translate_struct(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, struct_tt, &bound_args);
                let result_ce = ReferenceExpressionIE::Construct(self.interner.bump().alloc(crate::instantiating::ast::expressions::ConstructIE {
                    struct_tt: *self.interner.intern_struct_it_ci(crate::instantiating::ast::types::StructITValI { id: crate::instantiating::region_collapser_individual::collapse_struct_id(self.interner, &struct_it.id) }),
                    result: crate::instantiating::region_collapser_individual::collapse_coord(self.interner, &result_it),
                    args: self.interner.bump().alloc_slice_fill_iter(args_ce.into_iter()),
                }));
                (result_it, result_ce)
            }
            ReferenceExpressionTE::NewMutRuntimeSizedArray(_) => panic!("Unimplemented: translate_ref_expr NewMutRuntimeSizedArray"),
            ReferenceExpressionTE::StaticArrayFromCallable(_) => panic!("Unimplemented: translate_ref_expr StaticArrayFromCallable"),
            ReferenceExpressionTE::DestroyStaticSizedArrayIntoFunction(_) => panic!("Unimplemented: translate_ref_expr DestroyStaticSizedArrayIntoFunction"),
            ReferenceExpressionTE::DestroyStaticSizedArrayIntoLocals(_) => panic!("Unimplemented: translate_ref_expr DestroyStaticSizedArrayIntoLocals"),
            ReferenceExpressionTE::DestroyMutRuntimeSizedArray(_) => panic!("Unimplemented: translate_ref_expr DestroyMutRuntimeSizedArray"),
            ReferenceExpressionTE::RuntimeSizedArrayCapacity(_) => panic!("Unimplemented: translate_ref_expr RuntimeSizedArrayCapacity"),
            ReferenceExpressionTE::PushRuntimeSizedArray(_) => panic!("Unimplemented: translate_ref_expr PushRuntimeSizedArray"),
            ReferenceExpressionTE::PopRuntimeSizedArray(_) => panic!("Unimplemented: translate_ref_expr PopRuntimeSizedArray"),
            ReferenceExpressionTE::InterfaceToInterfaceUpcast(_) => panic!("Unimplemented: translate_ref_expr InterfaceToInterfaceUpcast"),
            ReferenceExpressionTE::Upcast(u) => {
                let crate::typing::ast::expressions::UpcastTE { inner_expr: inner_expr_unsubstituted, target_super_kind, impl_name: untranslated_impl_id } = *u;
                let impl_id = self.translate_impl_id(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &untranslated_impl_id);
                let result_it = self.translate_coord(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &u.result().coord);
                let (_inner_it, inner_ce) = self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &inner_expr_unsubstituted);
                let super_kind_s = self.translate_super_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &target_super_kind);
                let result_ce = ReferenceExpressionIE::Upcast(self.interner.bump().alloc(crate::instantiating::ast::expressions::UpcastIE {
                    inner_expr: inner_ce,
                    target_interface: *self.interner.intern_interface_it_ci(crate::instantiating::ast::types::InterfaceITValI { id: crate::instantiating::region_collapser_individual::collapse_interface_id(self.interner, &super_kind_s.id) }),
                    impl_name: crate::instantiating::region_collapser_individual::collapse_impl_id(self.interner, &impl_id),
                    result: crate::instantiating::region_collapser_individual::collapse_coord(self.interner, &result_it.coord),
                }));
                (result_it.coord, result_ce)
            }
            ReferenceExpressionTE::SoftLoad(sl) => {
                let crate::typing::ast::expressions::SoftLoadTE { expr: original_inner, target_ownership: original_target_ownership } = **sl;
                let (inner_it, inner_ce) = self.translate_addr_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &original_inner);
                let target_ownership = match (original_target_ownership, inner_it.ownership) {
                    (OwnershipT::Share, OwnershipI::ImmutableShare) => OwnershipI::ImmutableShare,
                    (OwnershipT::Share, OwnershipI::MutableShare) => OwnershipI::MutableShare,
                    (OwnershipT::Borrow, OwnershipI::ImmutableShare) => OwnershipI::ImmutableShare,
                    (OwnershipT::Borrow, OwnershipI::MutableShare) => OwnershipI::MutableShare,
                    (OwnershipT::Borrow, OwnershipI::ImmutableBorrow) => OwnershipI::ImmutableBorrow,
                    (OwnershipT::Borrow, OwnershipI::MutableBorrow) | (OwnershipT::Borrow, OwnershipI::Own) => {
                        // if (coordRegionIsMutable(substitutions, perspectiveRegionT, originalInner.result.coord)) {
                        OwnershipI::MutableBorrow
                        // } else { ImmutableBorrowI }
                    }
                    (OwnershipT::Weak, OwnershipI::ImmutableShare) => OwnershipI::ImmutableShare,
                    (OwnershipT::Weak, OwnershipI::MutableShare) => OwnershipI::MutableShare,
                    (OwnershipT::Weak, OwnershipI::Own) => OwnershipI::Weak,
                    (OwnershipT::Weak, OwnershipI::ImmutableBorrow) => OwnershipI::Weak,
                    (OwnershipT::Weak, OwnershipI::MutableBorrow) => OwnershipI::Weak,
                    (OwnershipT::Weak, OwnershipI::Weak) => OwnershipI::Weak,
                    other => panic!("SoftLoad: vwat {:?}", other),
                };
                let result_it = CoordI { ownership: target_ownership, kind: inner_it.kind };
                let result_ce = ReferenceExpressionIE::SoftLoad(self.interner.bump().alloc(crate::instantiating::ast::expressions::SoftLoadIE { expr: inner_ce, target_ownership, result: crate::instantiating::region_collapser_individual::collapse_coord(self.interner, &result_it) }));
                (result_it, result_ce)
            }
            ReferenceExpressionTE::Destroy(d) => {
                let crate::typing::ast::expressions::DestroyTE { expr: expr_t, struct_tt, destination_reference_variables } = **d;
                let (_source_it, source_ce) =
                    self.translate_ref_expr(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &expr_t);
                let bound_args = self.translate_bound_args_for_callee(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &self.hinputs.get_instantiation_bound_args(struct_tt.id));
                let struct_it = self.translate_struct_id(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, &struct_tt.id, &bound_args);
                let dest_ref_vars: Vec<crate::instantiating::ast::ast::ReferenceLocalVariableI<'s, 'i>> =
                    destination_reference_variables.iter().map(|dest_ref_var_t| {
                        self.translate_reference_local_variable(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, dest_ref_var_t).1
                    }).collect();
                let result_ce = ReferenceExpressionIE::Destroy(self.interner.bump().alloc(crate::instantiating::ast::expressions::DestroyIE {
                    expr: source_ce,
                    struct_tt: *self.interner.intern_struct_it_ci(crate::instantiating::ast::types::StructITValI { id: crate::instantiating::region_collapser_individual::collapse_struct_id(self.interner, &struct_it) }),
                    destination_reference_variables: self.interner.bump().alloc_slice_copy(&dest_ref_vars),
                }));
                (CoordI { ownership: OwnershipI::MutableShare, kind: KindIT::VoidIT(VoidIT { _marker: std::marker::PhantomData }) }, result_ce)
            }
            ReferenceExpressionTE::DestroyImmRuntimeSizedArray(_) => panic!("Unimplemented: translate_ref_expr DestroyImmRuntimeSizedArray"),
            ReferenceExpressionTE::NewImmRuntimeSizedArray(_) => panic!("Unimplemented: translate_ref_expr NewImmRuntimeSizedArray"),
        }
    }
}
/*
Guardian: temp-disable: TUCMPX — The Scala source itself returns MutableBorrowI unconditionally in this arm — the if/regionIsMutable conditional is commented out in the Scala source (lines 3471-3475 of the audit-trail). This is not unimplemented code; it is faithful 1:1 transcription of the Scala body. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-2707-1780143376785/hook-2707/translate_ref_expr--3065.0.TodosAndUnimplementedCodeMustPanic-TUCMPX.TodosAndUnimplementedCodeMustPanic-TUCMPX.verdict.md
Guardian: temp-disable: SPDMX — vregionmut is a documented passthrough/breakpoint-marker per migration-policy (migrate `vregionmut(x)` → `x`), same precedent as the existing temp-disable for `compose_ownerships` and `translate_coord` in this file. The five Weak arms preserve the Scala-returned values verbatim. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-2707-1780143376785/hook-2707/translate_ref_expr--3065.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def translateRefExpr(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,

    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    expr: ReferenceExpressionTE):
  // Returns the subjective coord (see HCCSCS) and the expression.
  (CoordI[sI], ReferenceExpressionIE) = {
    val denizenTemplateName = TemplataCompiler.getTemplate(denizenName)
    val (resultIT, resultCE) =
      expr match {
        case RestackifyTE(variable, inner) => {
          val (innerIT, innerCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, inner)
          val (localIT, localI) =
            translateLocalVariable(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, variable)
          //          env.addTranslatedVariable(variableT.name, vimpl(translatedVariable))
          val subjectiveResultIT = CoordI[sI](MutableShareI, VoidIT())
          val exprCE =
            RestackifyIE(
              localI, innerCE, RegionCollapserIndividual.collapseCoord(subjectiveResultIT))
          (subjectiveResultIT, exprCE)
        }

        case LetNormalTE(variableT, innerTE) => {
          val (innerIT, innerCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, innerTE)
          val (localIT, localI) =
            translateLocalVariable(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, variableT)
//          env.addTranslatedVariable(variableT.name, vimpl(translatedVariable))
          val subjectiveResultIT = CoordI[sI](MutableShareI, VoidIT())
          val exprCE =
            LetNormalIE(
              localI, innerCE, RegionCollapserIndividual.collapseCoord(subjectiveResultIT))
          (subjectiveResultIT, exprCE)
        }
        case BlockTE(inner) => {
          val (innerIT, innerCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, inner)
          val resultIT = innerIT
          val resultCE = BlockIE(innerCE, RegionCollapserIndividual.collapseCoord(resultIT))
          (resultIT, resultCE)
        }
        case ReturnTE(inner) => {
          val (innerIT, innerCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, inner)
          val resultCE = ReturnIE(innerCE)
          (CoordI[sI](MutableShareI, NeverIT(false)), resultCE)
        }
        case c @ ConsecutorTE(inners) => {
          val resultTT = c.result.coord
          val resultIT =
            translateCoord(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, resultTT)
                .coord
          val innersCE =
            inners.map(innerTE => {
              translateRefExpr(
                denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, innerTE)._2
            })
          val resultCE = ConsecutorIE(innersCE, RegionCollapserIndividual.collapseCoord(resultIT))
          (resultIT, resultCE)
        }
        case ConstantIntTE(value, bits, _) => {
          val resultCE =
            ConstantIntIE(
              ITemplataI.expectIntegerTemplata(
                translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, value)).value,
              bits)
          (CoordI[sI](MutableShareI, IntIT(bits)), resultCE)
        }
        case ConstantStrTE(value, _) => {
          val resultCE = ConstantStrIE(value)
          (CoordI[sI](MutableShareI, StrIT()), resultCE)
        }
        case ConstantBoolTE(value, _) => {
          val resultCE = ConstantBoolIE(value)
          (CoordI[sI](MutableShareI, BoolIT()), resultCE)
        }
        case ConstantFloatTE(value, _) => {
          val resultCE = ConstantFloatIE(value)
          (CoordI[sI](MutableShareI, BoolIT()), resultCE)
        }
        case UnletTE(variable) => {
          val (localIT, localCE) =
            translateLocalVariable(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, variable)
          val resultIT = localIT
//          val local = env.lookupOriginalTranslatedVariable(variable.name)
          val resultCE = UnletIE(localCE, RegionCollapserIndividual.collapseCoord(resultIT))
          (resultIT, resultCE)
        }
        case DiscardTE(innerTE) => {
          val (innerIT, innerCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, innerTE)
          val resultCE = DiscardIE(innerCE)
          (CoordI[sI](MutableShareI, VoidIT()), resultCE)
        }
        case VoidLiteralTE(_) => {
          (CoordI[sI](MutableShareI, VoidIT()), VoidLiteralIE())
        }
        case FunctionCallTE(prototypeT, args, returnType) => {
          val innersCE =
            args.map(argTE => {
              val (argIT, argCE) =
                translateRefExpr(
                  denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, argTE)
              // if (pure && argIT.ownership == MutableBorrowI) {
              //   PreCheckBorrowIE(argCE)
              // } else {
                argCE
              // }
            })

          val (prototypeI, prototypeC) =
            translatePrototype(
              denizenName,
              denizenBoundToDenizenCallerSuppliedThing,
              substitutions,
              perspectiveRegionT,
              prototypeT)
          val returnCoordIT = prototypeI.returnType
            // translateCoord(
            //   denizenName,
            //   denizenBoundToDenizenCallerSuppliedThing,
            //   substitutions,
            //   perspectiveRegionT,
            //   returnCoordT)
            //     .coord
          val returnCoordCT =
            RegionCollapserIndividual.collapseCoord(returnCoordIT)
          val resultCE = FunctionCallIE(prototypeC, innersCE, returnCoordCT)
          (returnCoordIT, resultCE)
        }
        case InterfaceFunctionCallTE(superFunctionPrototypeT, virtualParamIndex, resultReference, args) => {
          val (superFunctionPrototypeI, superFunctionPrototypeC) =
            translatePrototype(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, superFunctionPrototypeT)
          val resultIT = superFunctionPrototypeI.returnType
          val resultCE =
            InterfaceFunctionCallIE(
              superFunctionPrototypeC,
              virtualParamIndex,
              args.map(arg => {
                translateRefExpr(
                  denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, arg)._2
              }),
              RegionCollapserIndividual.collapseCoord(resultIT))
          val interfaceIdC =
            superFunctionPrototypeC.paramTypes(virtualParamIndex).kind.expectInterface().id
          //        val interfaceId =
          //          translateInterfaceId(
          //            interfaceIdT,
          //            translateBoundArgsForCallee(denizenName, denizenBoundToDenizenCallerSuppliedThing,
          //              hinputs.getInstantiationBounds(callee.toPrototype.fullName)))

          val instantiationBoundArgs =
            translateBoundArgsForCallee(denizenName, denizenBoundToDenizenCallerSuppliedThing,
              substitutions,
              perspectiveRegionT,
              // but this is literally calling itself from where its defined
              // perhaps we want the thing that originally called
              hinputs.getInstantiationBoundArgs(superFunctionPrototypeT.id))

          val superFunctionPrototypeN =
            RegionCollapserConsistent.collapsePrototype(
              RegionCounter.countPrototype(superFunctionPrototypeI),
              superFunctionPrototypeI)

          vassert(RegionCollapserIndividual.collapsePrototype(superFunctionPrototypeN) == superFunctionPrototypeC)

          monouts.newAbstractFuncs.enqueue(
            (superFunctionPrototypeT, superFunctionPrototypeN, virtualParamIndex, interfaceIdC, instantiationBoundArgs))

          (resultIT, resultCE)
        }
        case ArgLookupTE(paramIndex, reference) => {
          val typeS =
            translateCoord(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, reference)
                .coord
          val resultCE = ArgLookupIE(paramIndex, RegionCollapserIndividual.collapseCoord(typeS))
          (typeS, resultCE)
        }
        case SoftLoadTE(originalInner, originalTargetOwnership) => {
          val (innerIT, innerCE) =
            translateAddrExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, originalInner)
          val targetOwnership =
            // First, figure out what ownership it is after substitution.
            // if we have an owned T but T is a &Ship, then own + borrow = borrow
            (originalTargetOwnership, innerIT.ownership) match {
//              case (a, b) if a == b => a
              case (ShareT, ImmutableShareI) => ImmutableShareI
              case (ShareT, MutableShareI) => MutableShareI
              case (BorrowT, ImmutableShareI) => ImmutableShareI
              case (BorrowT, MutableShareI) => MutableShareI
              case (BorrowT, ImmutableBorrowI) => ImmutableBorrowI
              case (BorrowT, MutableBorrowI | OwnI) => {
                // if (coordRegionIsMutable(substitutions, perspectiveRegionT, originalInner.result.coord)) {
                MutableBorrowI
                // } else {
                //   ImmutableBorrowI
                // }
              }
              case (WeakT, ImmutableShareI) => vregionmut(ImmutableShareI)
              case (WeakT, MutableShareI) => vregionmut(MutableShareI)
             case (WeakT, OwnI) => vregionmut(WeakI)
              case (WeakT, ImmutableBorrowI) => vregionmut(WeakI)
              case (WeakT, MutableBorrowI) => vregionmut(WeakI)
              case (WeakT, WeakI) => vregionmut(WeakI)
              case other => vwat(other)
            }
          val resultIT = CoordI[sI](targetOwnership, innerIT.kind)
          val resultCE =
            SoftLoadIE(innerCE, targetOwnership, RegionCollapserIndividual.collapseCoord(resultIT))
          (resultIT, resultCE)
        }
        case ExternFunctionCallTE(prototype2, args) => {
          val (prototypeI, prototypeC) =
            translatePrototype(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, prototype2)
          val argsCE =
            args.map(argTE => {
              translateRefExpr(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, argTE)._2
            })
          val resultIT = prototypeI.returnType
          val resultCE = ExternFunctionCallIE(prototypeC, argsCE, prototypeC.returnType)
          // Only generic externs need to be collected here. Non-generic externs are translated
          // up-front before the main instantiating loop.
          prototype2.id.localName match {
            case dev.vale.typing.names.ExternFunctionNameT(humanName, templateArgs, _) if templateArgs.nonEmpty =>
              // Linear-scan functionExternsT for the matching definition record to recover
              // numInheritedGenericParameters. Key on (packageCoord, initSteps, humanName),
              // which is stable across definition (placeholder templateArgs) and callsite
              // (concrete templateArgs).
              val numInherited =
                hinputs.functionExterns.find(fe =>
                  fe.prototype.id.packageCoord == prototype2.id.packageCoord &&
                  fe.prototype.id.initSteps == prototype2.id.initSteps &&
                  (fe.prototype.id.localName match {
                    case dev.vale.typing.names.ExternFunctionNameT(hn, _, _) => hn == humanName
                    case _ => false
                  }))
                  .flatMap(_.genericParameterInheritance.map(_.numInheritedGenericParameters))
                  .getOrElse(0)
              monouts.functionExterns += FunctionExternI(prototypeC, numInherited)
            case _ =>
          }

          (resultIT, resultCE)
        }
        case ConstructTE(structTT, resultReference, args) => {
          val resultIT =
            translateCoord(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, resultReference)
                .coord

          //          val freePrototype = translatePrototype(freePrototypeT)
          //          // They might disagree on the ownership, and thats fine.
          //          // That free prototype is only going to take an owning or a share reference, and we'll only
          //          // use it if we have a shared reference so it's all good.
          //          vassert(coord.kind == vassertSome(freePrototype.fullName.last.parameters.headOption).kind)
          //          if (coord.ownership == ShareT) {
          //            monouts.immKindToDestructor.put(coord.kind, freePrototype)
          //          }

          val argsCE =
            args.map(argTE => {
              translateExpr(
                denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, argTE)._2
            })

          val structIT =
            translateStruct(
              denizenName,
              denizenBoundToDenizenCallerSuppliedThing,
              substitutions,
              perspectiveRegionT,
              structTT,
              translateBoundArgsForCallee(
                denizenName,
                denizenBoundToDenizenCallerSuppliedThing,
                substitutions,
                perspectiveRegionT,
                hinputs.getInstantiationBoundArgs(structTT.id)))

          val resultCE =
            ConstructIE(
              StructIT(RegionCollapserIndividual.collapseStructId(structIT.id)),
              RegionCollapserIndividual.collapseCoord(resultIT),
              argsCE)
          (resultIT, resultCE)
        }
        case DestroyTE(exprT, structTT, destinationReferenceVariables) => {
          val (sourceIT, sourceCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, exprT)

          val structIT =
            translateStructId(
              denizenName,
              denizenBoundToDenizenCallerSuppliedThing,
              substitutions,
              perspectiveRegionT,
              structTT.id,
              translateBoundArgsForCallee(
                denizenName,
                denizenBoundToDenizenCallerSuppliedThing,
                substitutions,
                perspectiveRegionT,
                hinputs.getInstantiationBoundArgs(structTT.id)))

//          val resultT =
//            expr.result.coord.kind match {
//              case s @ StructIT(_) => s
//              case other => vwat(other)
//            }

//          val structDef = vassertSome(monouts.structs.get(resultT.id))
//          vassert(structDef.members.size == destinationReferenceVariables.size)

          val resultCE =
            DestroyIE(
              sourceCE,
              StructIT(RegionCollapserIndividual.collapseStructId(structIT)),
              destinationReferenceVariables.map(destRefVarT => {
                translateReferenceLocalVariable(
                  denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions,
                  perspectiveRegionT, destRefVarT)._2
              }))
          (CoordI[sI](MutableShareI, VoidIT()), resultCE)
        }
        case DestroyStaticSizedArrayIntoLocalsTE(exprT, ssaTT, destinationReferenceVariables) => {
          val (sourceIT, sourceCE) = translateRefExpr(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, exprT)
          val (ssaIT, size) =
            sourceIT.kind match {
              case s @ StaticSizedArrayIT(IdI(_, _, StaticSizedArrayNameI(_, size, _, _))) => (s, size)
              case other => vwat(other)
            }

          vassert(size == destinationReferenceVariables.size)
          val resultCE =
            DestroyStaticSizedArrayIntoLocalsIE(
              sourceCE,
              RegionCollapserIndividual.collapseStaticSizedArray(ssaIT),
            destinationReferenceVariables.map(destRefVarT => {
              translateReferenceLocalVariable(
                denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, destRefVarT)._2
            }))
          (CoordI[sI](MutableShareI, VoidIT()), resultCE)
        }
        case MutateTE(destinationTT, sourceExpr) => {
          val (destinationIT, destinationCE) =
            translateAddrExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, destinationTT)
          val (sourceIT, sourceCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, sourceExpr)
          val resultIT = destinationIT
          val resultCE = MutateIE(destinationCE, sourceCE, RegionCollapserIndividual.collapseCoord(resultIT))
          (resultIT, resultCE)
        }
        case u @ UpcastTE(innerExprUnsubstituted, targetSuperKind, untranslatedImplId) => {
          val implId =
            translateImplId(
              denizenName,
              denizenBoundToDenizenCallerSuppliedThing,
              substitutions,
              perspectiveRegionT,
              untranslatedImplId)
          //          val freePrototype = translatePrototype(freePrototypeT)
          val resultIT =
            translateCoord(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, u.result.coord)
          // They might disagree on the ownership, and thats fine.
          // That free prototype is only going to take an owning or a share reference, and we'll only
          // use it if we have a shared reference so it's all good.
          //          vassert(coord.kind == vassertSome(freePrototype.fullName.last.parameters.headOption).kind)
          //          if (coord.ownership == ShareT) {
          //            monouts.immKindToDestructor.put(coord.kind, freePrototypeT)
          //          }

          val (innerIT, innerCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, innerExprUnsubstituted)

          val superKindS =
            translateSuperKind(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, targetSuperKind)

          val resultCE =
            UpcastIE(
              innerCE,
              InterfaceIT(RegionCollapserIndividual.collapseInterfaceId(superKindS.id)),
              RegionCollapserIndividual.collapseImplId(implId),
              RegionCollapserIndividual.collapseCoord(resultIT.coord))
          (resultIT.coord, resultCE)
        }
        case IfTE(condition, thenCall, elseCall) => {
          val (conditionIT, conditionCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, condition)
          val (thenIT, thenCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, thenCall)
          val (elseIT, elseCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, elseCall)
          val resultIT =
            (thenIT, elseIT) match {
              case (a, b) if a == b => a
              case (a, CoordI(_, NeverIT(_))) => a
              case (CoordI(_, NeverIT(_)), b) => b
              case other => vwat(other)
            }

          val resultCE = IfIE(conditionCE, thenCE, elseCE, RegionCollapserIndividual.collapseCoord(resultIT))
          (resultIT, resultCE)
        }
        case IsSameInstanceTE(left, right) => {
          val (leftIT, leftCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, left)
          val (rightIT, rightCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, right)
          val resultCE = IsSameInstanceIE(leftCE, rightCE)
          (CoordI[sI](MutableShareI, BoolIT()), resultCE)
        }
        case StaticArrayFromValuesTE(elements, resultReference, arrayType) => {

          //          val freePrototype = translatePrototype(freePrototypeT)
          val resultIT =
            translateCoord(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, resultReference)
                .coord
          // They might disagree on the ownership, and thats fine.
          // That free prototype is only going to take an owning or a share reference, and we'll only
          // use it if we have a shared reference so it's all good.
          //          vassert(coord.kind == vassertSome(freePrototype.fullName.last.parameters.headOption).kind)
          //          if (coord.ownership == ShareT) {
          //            monouts.immKindToDestructor.put(coord.kind, freePrototypeT)
          //          }

          val elementsCE =
            elements.map(elementTE => {
              translateRefExpr(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, elementTE)._2
            })

          val ssaTT =
            translateStaticSizedArray(
              denizenName,
              denizenBoundToDenizenCallerSuppliedThing,
              substitutions,
              perspectiveRegionT,
              arrayType)

          val resultCE =
            StaticArrayFromValuesIE(
              elementsCE,
              RegionCollapserIndividual.collapseCoord(resultIT),
              RegionCollapserIndividual.collapseStaticSizedArray(ssaTT))
          (resultIT, resultCE)
        }
        case DeferTE(innerExpr, deferredExpr) => {
          val (innerIT, innerCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, innerExpr)
          val (deferredIT, deferredCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, deferredExpr)
          val resultIT = innerIT
          val resultCE =
            DeferIE(innerCE, deferredCE, RegionCollapserIndividual.collapseCoord(resultIT))
          (resultIT, resultCE)
        }
        case LetAndLendTE(variable, sourceExprT, outerOwnershipT) => {
          val (sourceSubjectiveIT, sourceCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, sourceExprT)

          val resultOwnershipC =
            translateOwnership(
              substitutions,
              perspectiveRegionT,
              // TODO: see if we can combine this with the other composeOwnerships function.
              composeOwnerships(outerOwnershipT, sourceSubjectiveIT.ownership),
              sourceExprT.result.coord.region)

          val resultIT = CoordI(resultOwnershipC, sourceSubjectiveIT.kind)

          val (localIT, localI) =
            translateLocalVariable(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, variable)

          val resultCE =
            LetAndLendIE(
              localI,
              sourceCE,
              resultOwnershipC,
              RegionCollapserIndividual.collapseCoord(resultIT))
          (resultIT, resultCE)
        }
        case BorrowToWeakTE(innerExpr) => {
          val (innerIT, innerCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, innerExpr)


          val resultIT = innerIT.copy(ownership = WeakI)
          val resultCT = RegionCollapserIndividual.collapseCoord(resultIT)

          (resultIT, BorrowToWeakIE(innerCE, resultCT))
        }
        case WhileTE(BlockTE(inner)) => {
          val (innerIT, innerCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, inner)

          // While loops must always produce void.
          // If we want a foreach/map/whatever construct, the loop should instead
          // add things to a list inside; WhileIE shouldnt do it for it.
          val resultIT =
            innerIT match {
              case CoordI(_, VoidIT()) => innerIT
              case CoordI(_, NeverIT(true)) => CoordI[sI](MutableShareI, VoidIT())
              case CoordI(_, NeverIT(false)) => innerIT
              case _ => vwat()
            }

          val resultCE =
            WhileIE(
              BlockIE(innerCE, RegionCollapserIndividual.collapseCoord(innerIT)),
              RegionCollapserIndividual.collapseCoord(resultIT))
          (resultIT, resultCE)
        }
        case BreakTE(_) => {
          val resultCE = BreakIE()
          (CoordI[sI](MutableShareI, NeverIT(true)), resultCE)
        }
        case LockWeakTE(innerExpr, resultOptBorrowType, someConstructor, noneConstructor, someImplUntranslatedId, noneImplUntranslatedId) => {
          val resultIT =
            translateCoord(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, resultOptBorrowType).coord
          val resultCT = RegionCollapserIndividual.collapseCoord(resultIT)
          val resultCE =
            LockWeakIE(
              translateRefExpr(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, innerExpr)._2,
              resultCT,
              translatePrototype(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, someConstructor)._2,
              translatePrototype(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, noneConstructor)._2,
              RegionCollapserIndividual.collapseImplId(
                translateImplId(
                  denizenName,
                  denizenBoundToDenizenCallerSuppliedThing,
                  substitutions,
                  perspectiveRegionT,
                  someImplUntranslatedId)),
              RegionCollapserIndividual.collapseImplId(
                translateImplId(
                  denizenName,
                  denizenBoundToDenizenCallerSuppliedThing,
                  substitutions,
                  perspectiveRegionT,
                  noneImplUntranslatedId)),
              resultCT)
          (resultIT, resultCE)
        }
        case DestroyStaticSizedArrayIntoFunctionTE(arrayExpr, arrayType, consumer, consumerMethod) => {
          val (arrayIT, arrayCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, arrayExpr)
          val ssaIT =
            translateStaticSizedArray(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, arrayType)
          val (consumerIT, consumerCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, consumer)
          val (consumerPrototypeI, consumerPrototypeC) =
            translatePrototype(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, consumerMethod)
          val resultCE =
            DestroyStaticSizedArrayIntoFunctionIE(
              arrayCE, RegionCollapserIndividual.collapseStaticSizedArray(ssaIT), consumerCE, consumerPrototypeC)
          (CoordI[sI](MutableShareI, VoidIT()), resultCE)
        }
        case NewImmRuntimeSizedArrayTE(arrayType, _, sizeExpr, generator, generatorMethod) => {
          //          val freePrototype = translatePrototype(freePrototypeT)

          val rsaIT =
            translateRuntimeSizedArray(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, arrayType)
          val (sizeIT, sizeCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, sizeExpr)
          val (generatorIT, generatorCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, generator)
          val (generatorPrototypeI, generatorPrototypeC) =
            translatePrototype(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, generatorMethod)

          val resultIT =
            CoordI[sI](
              rsaIT.mutability match {
                case MutableI => OwnI
                case ImmutableI => MutableShareI
              },
              rsaIT)

          val resultCE =
            NewImmRuntimeSizedArrayIE(
              RegionCollapserIndividual.collapseRuntimeSizedArray(rsaIT),
              sizeCE,
              generatorCE,
              generatorPrototypeC,
              RegionCollapserIndividual.collapseCoord(resultIT))
          (resultIT, resultCE)

          // They might disagree on the ownership, and thats fine.
          // That free prototype is only going to take an owning or a share reference, and we'll only
          // use it if we have a shared reference so it's all good.
          //          vassert(coord.kind == vassertSome(freePrototype.fullName.last.parameters.headOption).kind)
          //          if (coord.ownership == ShareT) {
          //            monouts.immKindToDestructor.put(coord.kind, freePrototype)
          //          }

        }
        case StaticArrayFromCallableTE(arrayType, _, generator, generatorMethod) => {
          //          val freePrototype = translatePrototype(freePrototypeT)

          val ssaIT =
            translateStaticSizedArray(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, arrayType)
          val (generatorIT, generatorCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, generator)
          val (generatorPrototypeI, generatorPrototypeC) =
            translatePrototype(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, generatorMethod)

          val resultIT =
            CoordI[sI](
              ssaIT.mutability match {
                case MutableI => OwnI
                case ImmutableI => MutableShareI
              },
              ssaIT)

          val resultCE =
            StaticArrayFromCallableIE(
              RegionCollapserIndividual.collapseStaticSizedArray(ssaIT),
              generatorCE,
              generatorPrototypeC,
              RegionCollapserIndividual.collapseCoord(resultIT))

          // They might disagree on the ownership, and thats fine.
          // That free prototype is only going to take an owning or a share reference, and we'll only
          // use it if we have a shared reference so it's all good.
          //          vassert(coord.kind == vassertSome(freePrototype.fullName.last.parameters.headOption).kind)
          //          if (coord.ownership == ShareT) {
          //            monouts.immKindToDestructor.put(coord.kind, freePrototype)
          //          }

          (resultIT, resultCE)
        }
        case RuntimeSizedArrayCapacityTE(arrayExpr) => {
          val (arrayIT, arrayCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, arrayExpr)
          val resultCE = RuntimeSizedArrayCapacityIE(arrayCE)
          (CoordI[sI](MutableShareI, IntIT(32)), resultCE)
        }
        case PushRuntimeSizedArrayTE(arrayExpr, newElementExpr) => {
          val (arrayIT, arrayCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, arrayExpr)
          val (elementIT, elementCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, newElementExpr)
          val resultCE = PushRuntimeSizedArrayIE(arrayCE, elementCE)
          (CoordI[sI](MutableShareI, VoidIT()), resultCE)
        }
        case PopRuntimeSizedArrayTE(arrayExpr) => {
          val (arrayIT, arrayCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, arrayExpr)
          val elementIT =
            arrayIT.kind match {
              case RuntimeSizedArrayIT(IdI(_, _, RuntimeSizedArrayNameI(_, RawArrayNameI(_, elementType, _)))) => {
                elementType.coord
              }
              case other => vwat(other)
            }
          val resultCE = PopRuntimeSizedArrayIE(arrayCE, RegionCollapserIndividual.collapseCoord(elementIT))
          (elementIT, resultCE)
        }
        case ArrayLengthTE(arrayExpr) => {
          val (arrayIT, arrayCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, arrayExpr)
          val resultIT = CoordI[sI](MutableShareI, IntIT(32))
          val resultCE = ArrayLengthIE(arrayCE)
          (resultIT, resultCE)
        }
        case DestroyImmRuntimeSizedArrayTE(arrayExpr, arrayType, consumer, consumerMethod) => {
          val (arrayIT, arrayCE) = translateRefExpr(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, arrayExpr)
          val rsaIT = translateRuntimeSizedArray(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, arrayType)
          val (consumerIT, consumerCE) = translateRefExpr(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, consumer)
          val (prototypeI, prototypeC) =
            translatePrototype(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, consumerMethod)

          val resultCE =
            DestroyImmRuntimeSizedArrayIE(
              arrayCE,
              RegionCollapserIndividual.collapseRuntimeSizedArray(rsaIT),
              consumerCE,
              prototypeC)
          (CoordI[sI](MutableShareI, VoidIT()), resultCE)
        }
        case DestroyMutRuntimeSizedArrayTE(arrayExpr) => {
          val (arrayIT, arrayCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, arrayExpr)
          val resultCE = DestroyMutRuntimeSizedArrayIE(arrayCE)
          (CoordI.void[sI], resultCE)
        }
        case NewMutRuntimeSizedArrayTE(arrayTT, _, capacityExpr) => {
          val arrayIT =
            translateRuntimeSizedArray(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, arrayTT)
          val resultIT =
            CoordI[sI](
              arrayIT.mutability match {
                case MutableI => OwnI
                case ImmutableI => MutableShareI
              },
              arrayIT)

          val (capacityIT, capacityCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, capacityExpr)

          val resultCE =
            NewMutRuntimeSizedArrayIE(
              RegionCollapserIndividual.collapseRuntimeSizedArray(arrayIT),
              capacityCE,
              RegionCollapserIndividual.collapseCoord(resultIT))
          (resultIT, resultCE)
        }
        case TupleTE(elements, resultReference) => {
          val elementsCE =
            elements.map(elementTE => {
              translateRefExpr(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, elementTE)._2
            })

          val resultIT =
            translateCoord(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, resultReference)
                .coord

          (resultIT, TupleIE(elementsCE, RegionCollapserIndividual.collapseCoord(resultIT)))
        }
        case AsSubtypeTE(sourceExpr, targetSubtype, resultResultType, okConstructor, errConstructor, implIdT, okResultImplIdT, errResultImplIdT) => {
          val (sourceIT, sourceCE) =
            translateRefExpr(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, sourceExpr)
          val resultIT =
            translateCoord(
              denizenName, denizenBoundToDenizenCallerSuppliedThing,
              substitutions,
              perspectiveRegionT,
              resultResultType).coord
          val resultCE =
            AsSubtypeIE(
              sourceCE,
              RegionCollapserIndividual.collapseCoord(translateCoord(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, targetSubtype).coord),
              RegionCollapserIndividual.collapseCoord(translateCoord(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, resultResultType).coord),
              translatePrototype(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, okConstructor)._2,
              translatePrototype(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, errConstructor)._2,
              RegionCollapserIndividual.collapseImplId(
                translateImplId(
                  denizenName,
                  denizenBoundToDenizenCallerSuppliedThing,
                  substitutions,
                  perspectiveRegionT,
                  implIdT)),
              RegionCollapserIndividual.collapseImplId(
                translateImplId(
                  denizenName,
                  denizenBoundToDenizenCallerSuppliedThing,
                  substitutions,
                  perspectiveRegionT,
                  okResultImplIdT)),
              RegionCollapserIndividual.collapseImplId(
                translateImplId(
                  denizenName,
                  denizenBoundToDenizenCallerSuppliedThing,
                 substitutions,
                  perspectiveRegionT,
                  errResultImplIdT)),
              RegionCollapserIndividual.collapseCoord(resultIT))
          (resultIT, resultCE)
        }
        case other => vimpl(other)
      }
    //    if (opts.sanityCheck) {
    //      vassert(Collector.all(resultRefExpr, { case PlaceholderNameT(_) => }).isEmpty)
    //    }
    (resultIT, resultCE)
  }
*/
// mig: fn maybe_immutabilify
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn maybe_immutabilify(_inner_ie: &ReferenceExpressionIE<'s, 'i, cI>) -> ReferenceExpressionIE<'s, 'i, cI> {
        panic!("Unimplemented: maybe_immutabilify");
    }
}
/*
  private def maybeImmutabilify(innerIE: ReferenceExpressionIE): ReferenceExpressionIE = {
    innerIE.result.kind match {
      case x if x.isPrimitive => {
        return innerIE // These are conceptually moved into the receiver's region
      }
      case _ => // continue
    }
    innerIE match {
      case SoftLoadIE(expr, MutableBorrowI, result) => {
        return SoftLoadIE(expr, ImmutableBorrowI, result.copy(ownership = ImmutableBorrowI))
      }
      case SoftLoadIE(expr, MutableShareI, result) => {
        return SoftLoadIE(expr, ImmutableShareI, result.copy(ownership = ImmutableShareI))
      }
      case _ => //continue
    }
    innerIE.result.ownership match {
      case OwnI => innerIE // These are being moved into the receiver's region
      case ImmutableBorrowI | ImmutableShareI => innerIE
      case MutableBorrowI => {
        ImmutabilifyIE(innerIE, innerIE.result.copy(ownership = ImmutableBorrowI))
      }
      case MutableShareI => {
        ImmutabilifyIE(innerIE, innerIE.result.copy(ownership = ImmutableShareI))
      }
    }
  }
*/
// mig: fn run_in_new_pure_region
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn run_in_new_pure_region<T>(_denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _denizen_template_name: &IdT<'s, 't>, _new_default_region_t: &ITemplataT<'s, 't>, _run: impl Fn(&IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, &RegionT) -> T) -> T {
        panic!("Unimplemented: run_in_new_pure_region");
    }
}
/*
  private def runInNewPureRegion[T](
      denizenName: IdT[IInstantiationNameT],
      denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,

      substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
      denizenTemplateName: IdT[ITemplateNameT],
      newDefaultRegionT: ITemplataT[RegionTemplataType],
      run: (Map[IdT[IPlaceholderNameT], ITemplataI[sI]], RegionT) => T):
  T = {
    val newDefaultRegionNameT = RegionT(DefaultRegionT)
    val newPerspectiveRegionT = newDefaultRegionNameT

    val newDefaultRegion = RegionT(DefaultRegionT)
    run(substitutions, newPerspectiveRegionT)
  }
*/
// mig: fn translate_ownership
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_ownership(_substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, ownership_t: &OwnershipT, _region_t: &RegionT) -> OwnershipI {
        match ownership_t {
            OwnershipT::Own => OwnershipI::Own,
            OwnershipT::Borrow => OwnershipI::MutableBorrow,
            OwnershipT::Share => OwnershipI::MutableShare,
            OwnershipT::Weak => panic!("translate_ownership: WeakT vimpl"),
        }
    }
}
/*
  def translateOwnership(
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    ownershipT: OwnershipT,
    regionT: RegionT):
  OwnershipI = {
    ownershipT match { // Now  if it's a borrow, figure out whether it's mutable or immutable
      case OwnT => OwnI
      case BorrowT => {
        // if (regionIsMutable(substitutions, perspectiveRegionT, expectRegionPlaceholder(regionT))) {
        MutableBorrowI
        // } else {
        //   ImmutableBorrowI
        // }
      }
      case ShareT => {
        // if (regionIsMutable(substitutions, perspectiveRegionT, expectRegionPlaceholder(regionT))) {
        MutableShareI
        // } else {
        //   ImmutableShareI
        // }
      }
      case WeakT => vimpl()
    }
  }
*/
// mig: fn compose_ownerships
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn compose_ownerships(outer_ownership: &OwnershipT, inner_ownership: &OwnershipI, kind: &KindIT<'s, 'i, sI>) -> OwnershipI {
        match kind {
            KindIT::IntIT(_) | KindIT::BoolIT(_) | KindIT::VoidIT(_) => OwnershipI::MutableShare,
            _ => {
                match (outer_ownership, inner_ownership) {
                    (OwnershipT::Own, OwnershipI::Own) => OwnershipI::Own,
                    (OwnershipT::Own, OwnershipI::MutableShare) | (OwnershipT::Own, OwnershipI::ImmutableShare)
                    | (OwnershipT::Borrow, OwnershipI::MutableShare) | (OwnershipT::Borrow, OwnershipI::ImmutableShare) => {
                        OwnershipI::MutableShare
                    }
                    (OwnershipT::Own, OwnershipI::MutableBorrow) => {
                        // vregionmut() // here too maybe?
                        OwnershipI::MutableBorrow
                    }
                    (OwnershipT::Borrow, OwnershipI::Own) => {
                        // vregionmut() // we'll probably want a regionIsMutable call like above
                        OwnershipI::MutableBorrow
                    }
                    (OwnershipT::Borrow, OwnershipI::MutableBorrow) => {
                        // vregionmut() // we'll probably want a regionIsMutable call like above
                        OwnershipI::MutableBorrow
                    }
                    (OwnershipT::Weak, OwnershipI::Own) => {
                        // vregionmut() // here too maybe?
                        OwnershipI::Weak
                    }
                    (OwnershipT::Share, OwnershipI::MutableShare) => {
                        // vregionmut() // here too maybe?
                        OwnershipI::MutableShare
                    }
                    other => panic!("compose_ownerships: vwat {:?}", other),
                }
            }
        }
    }
}
/*
Guardian: temp-disable: SPDMX — vregionmut is a documented passthrough/breakpoint-marker per migration-policy (migrate `vregionmut(x)` → `x`, no-arg `vregionmut()` is a TODO/regions-marker no-op), same precedent as the existing temp-disable on translate_coord in this file at line 4581. The five arms preserve the Scala return values verbatim and the `// vregionmut()` comments preserve the audit trail. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-2590-1780139831308/hook-2590/compose_ownerships--4166.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  private def composeOwnerships(outerOwnership: OwnershipT, innerOwnership: OwnershipI, kind: KindIT[sI]) = {
    // TODO: see if we can combine this with the other composeOwnerships function.
    kind match {
      case IntIT(_) | BoolIT() | VoidIT() => {
        // We don't want any ImmutableShareH for primitives, it's better to only ever have one
        // ownership for primitives.
        MutableShareI
      }
      case _ => {
        ((outerOwnership, innerOwnership) match {
          case (OwnT, OwnI) => OwnI
          // case (OwnT, ImmutableShareI) => ImmutableShareI
          case (OwnT | BorrowT, MutableShareI | ImmutableShareI) => {
            // We disregard whether it's a MutableShareI or ImmutableShareI because
            // that was likely calculated under different circumstances from a
            // different perspective region.
            // We'll recalculate it now with out own perspective region.
            // See IPOMFIC.
            //if (regionIsMutable(substitutions, perspectiveRegionT, expectRegionPlaceholder(outerRegion))) {
            MutableShareI
            // } else {
            //   ImmutableShareI
            // }
          }
          //                case (OwnT, BorrowT) => BorrowT
          case (OwnT, MutableBorrowI) => {
            vregionmut() // here too maybe?
            MutableBorrowI
          }
          //                case (BorrowT, OwnT) => BorrowT
          case (BorrowT, OwnI) => {
            vregionmut() // we'll probably want a regionIsMutable call like above
            MutableBorrowI
          }
          //                case (BorrowT, BorrowT) => BorrowT
          case (BorrowT, MutableBorrowI) => {
            vregionmut() // we'll probably want a regionIsMutable call like above
            MutableBorrowI
          }
          //                case (BorrowT, WeakT) => WeakT
          //                case (BorrowT, ShareT) => ShareT
          //                case (WeakT, OwnT) => WeakT
          case (WeakT, OwnI) => {
            vregionmut() // here too maybe?
            WeakI
          }
          //                case (WeakT, BorrowT) => WeakT
          //                case (WeakT, WeakT) => WeakT
          //                case (WeakT, ShareT) => ShareT
          //                case (ShareT, ShareT) => ShareT
          case (ShareT, MutableShareI) => {
            vregionmut() // here too maybe?
            MutableShareI
          }
          //                case (OwnT, ShareT) => ShareT
          case other => vwat(other)
        })
      }
    }
  }
*/
// mig: fn compose_ownerships
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn compose_ownerships_second(outer_ownership: &OwnershipT, inner_ownership: &OwnershipI) -> OwnershipT {
        match (outer_ownership, inner_ownership) {
            (OwnershipT::Own, OwnershipI::Own) => OwnershipT::Own,
            (OwnershipT::Own, OwnershipI::MutableBorrow) => OwnershipT::Borrow,
            (OwnershipT::Borrow, OwnershipI::Own) => OwnershipT::Borrow,
            (OwnershipT::Borrow, OwnershipI::MutableBorrow) => OwnershipT::Borrow,
            (OwnershipT::Borrow, OwnershipI::Weak) => OwnershipT::Weak,
            (OwnershipT::Borrow, OwnershipI::MutableShare) => OwnershipT::Share,
            (OwnershipT::Weak, OwnershipI::Own) => OwnershipT::Weak,
            (OwnershipT::Weak, OwnershipI::MutableBorrow) => OwnershipT::Weak,
            (OwnershipT::Weak, OwnershipI::Weak) => OwnershipT::Weak,
            (OwnershipT::Weak, OwnershipI::MutableShare) => OwnershipT::Share,
            (OwnershipT::Share, OwnershipI::MutableShare) => OwnershipT::Share,
            (OwnershipT::Own, OwnershipI::MutableShare) => OwnershipT::Share,
            other => panic!("compose_ownerships_second: vwat {:?}", other),
        }
    }
}
/*
  // TODO: see if we can combine this with the other composeOwnerships function.
  def composeOwnerships(
    outerOwnership: OwnershipT,
    innerOwnership: OwnershipI):
  OwnershipT = {
    (outerOwnership, innerOwnership) match {
      case (OwnT, OwnI) => OwnT
      case (OwnT, MutableBorrowI) => BorrowT
      case (BorrowT, OwnI) => BorrowT
      case (BorrowT, MutableBorrowI) => BorrowT
      case (BorrowT, WeakI) => WeakT
      case (BorrowT, MutableShareI) => ShareT
      case (WeakT, OwnI) => WeakT
      case (WeakT, MutableBorrowI) => WeakT
      case (WeakT, WeakI) => WeakT
      case (WeakT, MutableShareI) => ShareT
      case (ShareT, MutableShareI) => ShareT
      case (OwnT, MutableShareI) => ShareT
      case other => vwat(other)
    }
  }
*/
// mig: fn translate_function_id
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_function_id(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, perspective_region_t: &RegionT, full_name_t: &IdT<'s, 't>) -> IdI<'s, 'i, sI> {
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
}
/*
  def translateFunctionId(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    fullNameT: IdT[IFunctionNameT]):
  IdI[sI, IFunctionNameI[sI]] = {
    val IdT(module, steps, last) = fullNameT
    val fullName =
      IdI(
        module,
        steps.map(translateName(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
        translateFunctionName(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, last))
    //    if (opts.sanityCheck) {
    //      vassert(Collector.all(fullName, { case PlaceholderNameT(_) => }).isEmpty)
    //    }
    fullName
  }

//  def translateRegionId(
//    substitutions: Map[IdT[INameT], Map[IdT[IPlaceholderNameT], ITemplata[ITemplataType]]],
//    perspectiveRegionT: GlobalRegionT,
//    fullNameT: IdT[IRegionNameT]//,
//    //instantiationBoundArgs: InstantiationBoundArguments
//  ):
//  IdT[IRegionNameT] = {
//    fullNameT match {
//      case IdT(packageCoord, initSteps, r @ RegionPlaceholderNameT(_, _, _, _)) => {
//          IdT(
//            packageCoord,
//            initSteps.map(translateName(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
//            translateRegionName(substitutions, perspectiveRegionT, r))
//      }
//    }
//  }
*/
// mig: fn translate_struct_id
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_struct_id(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _struct_id_t: &IdT<'s, 't>, _instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) -> IdI<'s, 'i, sI> {
        let IdT { package_coord: module, init_steps: steps, local_name: last_t, .. } = _struct_id_t;
        let last_t_struct: IStructNameT<'s, 't> = (*last_t).try_into().unwrap();
        let translated_steps: Vec<INameI<'s, 'i, sI>> = steps.iter().map(|n| self.translate_name_substituting(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, n)).collect();
        let struct_name_si = self.translate_struct_name(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &last_t_struct);
        let full_name_s = IdI {
            package_coord: module,
            init_steps: self.interner.bump().alloc_slice_fill_iter(translated_steps.into_iter()),
            local_name: struct_name_si.into(),
        };
        self.collapse_and_translate_struct_definition(_monouts, _struct_id_t, &full_name_s, _instantiation_bound_args);
        full_name_s
    }
}
/*
  def translateStructId(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    fullNameT: IdT[IStructNameT],
    instantiationBoundArgs: InstantiationBoundArgumentsI):
  IdI[sI, IStructNameI[sI]] = {
    val IdT(module, steps, lastT) = fullNameT

    val fullNameS =
      IdI(
        module,
        steps.map(translateName(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
        translateStructName(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, lastT))


    collapseAndTranslateStructDefinition(
      opts, interner, keywords, hinputs, monouts, fullNameT, fullNameS, instantiationBoundArgs)

    fullNameS
  }
*/
// mig: fn translate_interface_id
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_interface_id(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _interface_id_t: &IdT<'s, 't>, _instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) -> IdI<'s, 'i, sI> {
        let IdT { package_coord: module, init_steps: steps, local_name: last_t, .. } = _interface_id_t;
        let last_t_interface = match last_t {
            INameT::Interface(i) => crate::typing::names::names::IInterfaceNameT::Interface(*i),
            _ => panic!("translate_interface_id: local_name not Interface"),
        };
        let translated_steps: Vec<INameI<'s, 'i, sI>> = steps.iter().map(|_n| panic!("translate_interface_id: non-empty init_steps not yet ported")).collect();
        let interface_name_si = self.translate_interface_name(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &last_t_interface);
        let full_name_s = IdI {
            package_coord: module,
            init_steps: self.interner.bump().alloc_slice_fill_iter(translated_steps.into_iter()),
            local_name: interface_name_si.into(),
        };
        self.collapse_and_translate_interface_definition(_monouts, _interface_id_t, &full_name_s, _instantiation_bound_args);
        full_name_s
    }
}
/*
  def translateInterfaceId(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    fullNameT: IdT[IInterfaceNameT],
    instantiationBoundArgs: InstantiationBoundArgumentsI):
  IdI[sI, IInterfaceNameI[sI]] = {
    val IdT(module, steps, last) = fullNameT
    val newIdS =
      IdI(
        module,
        steps.map(translateName(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
        translateInterfaceName(denizenName, denizenBoundToDenizenCallerSuppliedThing,substitutions, perspectiveRegionT, last))

    collapseAndTranslateInterfaceDefinition(
      opts, interner, keywords, hinputs, monouts, fullNameT, newIdS, instantiationBoundArgs)

    newIdS
  }
*/
// mig: fn translate_impl_id
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_impl_id(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _impl_id_t: &IdT<'s, 't>) -> IdI<'s, 'i, sI> {
        let IdT { package_coord: module, init_steps: steps, local_name: last_t, .. } = *_impl_id_t;
        match last_t {
            INameT::ImplBound(_) => {
                let impl_bound_name = *_impl_id_t;
                let impl_id_s = *_denizen_bound_to_denizen_caller_supplied_thing.bound_param_impl_id_to_bound_arg_impl_id.get(&impl_bound_name).expect("translate_impl_id: missing impl bound");
                impl_id_s
            }
            _ => {
                let translated_steps: Vec<INameI<'s, 'i, sI>> = steps.iter().map(|s| Self::translate_name(s)).collect();
                let impl_name_i = self.translate_impl_name(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &IImplNameT::try_from(last_t).expect("translate_impl_id: non-impl name"));
                let impl_id_s = IdI { package_coord: module, init_steps: self.interner.bump().alloc_slice_fill_iter(translated_steps.into_iter()), local_name: INameI::from(impl_name_i) };
                let impl_id_n = crate::instantiating::region_collapser_consistent::collapse_impl_id(self.interner, &crate::instantiating::region_counter::count_impl_id_map(&impl_id_s), &impl_id_s);
                let bound_args_for_call_unsubstituted = self.hinputs.get_instantiation_bound_args(*_impl_id_t);
                let rune_to_bound_args_for_new_impl = self.translate_bound_args_for_callee(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &bound_args_for_call_unsubstituted);
                _monouts.new_impls.push((*_impl_id_t, impl_id_n, rune_to_bound_args_for_new_impl));
                impl_id_s
            }
        }
    }
}
/*
  def translateImplId(
      denizenName: IdT[IInstantiationNameT],
      denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
      substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
      perspectiveRegionT: RegionT,
      implIdT: IdT[IImplNameT]):
  IdI[sI, IImplNameI[sI]] = {
    val IdT(module, steps, lastT) = implIdT

    // collapseAndTranslateImplDefinition(
    //   opts, interner, keywords, hinputs, monouts, implIdT, implIdT, instantiationBoundArgs)

    implIdT match {
      case IdT(packageCoord, initSteps, name @ ImplBoundNameT(_, _)) => {
        val implBoundName = IdT(packageCoord, initSteps, name)
        val implIdS = vassertSome(denizenBoundToDenizenCallerSuppliedThing.boundParamImplIdToBoundArgImplId.get(implBoundName))

        // val implIdC =
        //   RegionCollapserIndividual.collapseImplId(implIdS)
        implIdS
      }
      case _ => {
        val implIdS =
          IdI(
            module,
            steps.map(translateName(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
            translateImplName(
              denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, lastT))

        val implIdN =
          RegionCollapserConsistent.collapseImplId(
            RegionCounter.countImplId(implIdS),
            implIdS)

        val runeToBoundArgsForNewImpl =
          translateBoundArgsForCallee(
            denizenName,
            denizenBoundToDenizenCallerSuppliedThing,
            substitutions,
            perspectiveRegionT,
            hinputs.getInstantiationBoundArgs(implIdT))

        monouts.newImpls.enqueue((implIdT, implIdN, runeToBoundArgsForNewImpl))

        implIdS
      }
    }
  }
*/
// mig: fn translate_citizen_name
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_citizen_name(&self, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _t: &ICitizenNameT<'s, 't>) -> ICitizenNameI<'s, 'i, sI> {
        panic!("Unimplemented: translate_citizen_name");
    }
}
/*
  def translateCitizenName(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    t: ICitizenNameT):
  ICitizenNameI[sI] = {
    t match {
      case s : IStructNameT => translateStructName(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, s)
      case i : IInterfaceNameT => translateInterfaceName(denizenName, denizenBoundToDenizenCallerSuppliedThing,substitutions, perspectiveRegionT, i)
    }
  }
*/
// mig: fn translate_id
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_id_from_substitutions(_substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _id: &IdT<'s, 't>) -> IdI<'s, 'i, sI> {
        panic!("Unimplemented: translate_id_from_substitutions");
    }
}
/*
  def translateId(
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    id: IdT[INameT]):
  IdI[sI, INameI[sI]] = {
    id match {
      case other => vimpl(other)
    }
  }
*/
// mig: fn translate_citizen_id
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_citizen_id(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _citizen_id_t: &IdT<'s, 't>, _instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) -> IdI<'s, 'i, sI> {
        panic!("Unimplemented: translate_citizen_id");
    }
}
/*
  def translateCitizenId(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    id: IdT[ICitizenNameT],
    instantiationBoundArgs: InstantiationBoundArgumentsI):
  IdI[sI, ICitizenNameI[sI]] = {
    id match {
      case IdT(module, steps, last : IStructNameT) => {
        translateStructId(
          denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, IdT(module, steps, last), instantiationBoundArgs)
      }
      case IdT(module, steps, last : IInterfaceNameT) => {
        translateInterfaceId(
          denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, IdT(module, steps, last), instantiationBoundArgs)
      }
      case other => vimpl(other)
    }
  }
*/
// mig: fn translate_coord
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_coord(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, perspective_region_t: &RegionT, coord_t: &CoordT<'s, 't>) -> CoordTemplataI<'s, 'i, sI> {
        let CoordT { ownership: outer_ownership, region: _outer_region, kind } = coord_t;
        let _outer_region_i = RegionT { region: IRegionT::Default };
          // translateTemplata(
          //   denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, outerRegion)
          //     .expectRegionTemplata()

        match kind {
            KindT::KindPlaceholder(placeholder_id) => {
                let sub = substitutions.get(&placeholder_id.id).expect("translate_coord: missing placeholder substitution");
                match sub {
                    ITemplataI::Coord(c) => {
                        let CoordTemplataI { region: _region, coord: CoordI { ownership: inner_ownership, kind: inner_kind } } = *c;
                        let combined_ownership = Self::compose_ownerships(outer_ownership, &inner_ownership, &inner_kind);
                        CoordTemplataI { region: RegionTemplataI { pure_height: 0, _marker: std::marker::PhantomData }, coord: CoordI { ownership: combined_ownership, kind: inner_kind } }
                    }
                    ITemplataI::Kind(_) => panic!("Unimplemented: translate_coord KindPlaceholder->Kind"),
                    _ => panic!("Unimplemented: translate_coord KindPlaceholder other"),
                }
            }
            other => {
                // We could, for example, be translating an Vector<myFunc$0, T> (which is temporarily regarded mutable)
                // to an Vector<imm, int> (which is immutable).
                // So, we have to check for that here and possibly make the ownership share.
                let kind = self.translate_kind(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, other);
                let new_ownership =
                    match kind {
                        KindIT::IntIT(_) | KindIT::BoolIT(_) | KindIT::VoidIT(_) => {
                            // We don't want any ImmutableShareH for primitives, it's better to only ever have one
                            // ownership for primitives.
                            OwnershipI::MutableShare
                        }
                        _ => {
                            let mutability = Self::get_mutability(monouts, &region_collapser_individual::collapse_kind(self.interner, &kind));
                            match (match (*outer_ownership, mutability) {
                                (_, MutabilityI::Immutable) => OwnershipT::Share,
                                (other, MutabilityI::Mutable) => other,
                            }) { // Now  if it's a borrow, figure out whether it's mutable or immutable
                                OwnershipT::Borrow => {
                                    // if (regionIsMutable(substitutions, perspectiveRegionT, expectRegionPlaceholder(outerRegion))) {
                                    OwnershipI::MutableBorrow
                                    // } else {
                                    //   ImmutableBorrowI
                                    // }
                                }
                                OwnershipT::Share => {
                                    // if (regionIsMutable(substitutions, perspectiveRegionT, expectRegionPlaceholder(outerRegion))) {
                                    OwnershipI::MutableShare
                                    // } else {
                                    //   ImmutableShareI
                                    // }
                                }
                                OwnershipT::Own => {
                                    // We don't have this assert because we sometimes can see owning references even
                                    // though we dont hold them, see RMLRMO.
                                    // vassert(regionIsMutable(substitutions, perspectiveRegionT, expectRegionPlaceholder(outerRegion)))
                                    OwnershipI::Own
                                }
                                OwnershipT::Weak => {
                                    OwnershipI::Weak
                                }
                            }
                        }
                    };
//        val newRegion = expectRegionTemplata(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, outerRegion))
                CoordTemplataI { region: RegionTemplataI { pure_height: 0, _marker: std::marker::PhantomData }, coord: CoordI { ownership: new_ownership, kind } }
            }
        }
    }
}
/*
Guardian: temp-disable: SPDMX — vregionmut is a documented passthrough — migration-policy maps `vregionmut(x)` → `x` (TL-confirmed round 25). Scala `case WeakT => vregionmut(WeakI)` faithfully ports to `OwnershipI::Weak`; not a silent omission, not a TODO (a panic would be wrong since vregionmut just returns its arg). — /Volumes/V/Vale/FrontendRust/guardian-logs/request-706-1780023651663/hook-706/translate_coord--3881.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def translateCoord(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    coord: CoordT):
  CoordTemplataI[sI] = {
    val CoordT(outerOwnership, outerRegion, kind) = coord
    val outerRegionI = RegionT(DefaultRegionT)
      // translateTemplata(
      //   denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, outerRegion)
      //     .expectRegionTemplata()

    kind match {
      case KindPlaceholderT(placeholderId) => {
        // Let's get the index'th placeholder from the top level denizen.
        // If we're compiling a function or a struct, it might actually be a lambda function or lambda struct.
        // In these cases, the topLevelDenizenPlaceholderIndexToTemplata actually came from the containing function,
        // see LHPCTLD.

        // Per @PASDZ, placeholderId is path-encoded (it includes its owning denizen), so the flat
        // single-level map suffices — no per-denizen outer key.
        vassertSome(substitutions.get(placeholderId)) match {
          case CoordTemplataI(region, CoordI(innerOwnership, kind)) => {
            // TODO: see if we can combine this with the other composeOwnerships function.
            val combinedOwnership =
              composeOwnerships(outerOwnership, innerOwnership, kind)
//            vassert(innerRegion == translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, outerRegion))
            CoordTemplataI(RegionTemplataI(0), CoordI(combinedOwnership, kind))
          }
          case KindTemplataI(kind) => {
//            val newOwnership =
//              getMutability(kind) match {
//                case ImmutableT => ShareT
//                case MutableT => outerOwnership
//              }
            // CoordTemplataI(RegionTemplataI(0), CoordI(newOwnership), vimpl(kind))
            CoordTemplataI(RegionTemplataI(0), CoordI(vimpl(), vimpl(kind)))
          }
        }
      }
      case other => {
        // We could, for example, be translating an Vector<myFunc$0, T> (which is temporarily regarded mutable)
        // to an Vector<imm, int> (which is immutable).
        // So, we have to check for that here and possibly make the ownership share.
        val kind = translateKind(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, other)
        val newOwnership =
          kind match {
            case IntIT(_) | BoolIT() | VoidIT() => {
              // We don't want any ImmutableShareH for primitives, it's better to only ever have one
              // ownership for primitives.
              MutableShareI
            }
            case _ => {
              val mutability = getMutability(RegionCollapserIndividual.collapseKind(kind))
              ((outerOwnership, mutability) match {
                case (_, ImmutableI) => ShareT
                case (other, MutableI) => other
              }) match { // Now  if it's a borrow, figure out whether it's mutable or immutable
                case BorrowT => {
                  // if (regionIsMutable(substitutions, perspectiveRegionT, expectRegionPlaceholder(outerRegion))) {
                  MutableBorrowI
                  // } else {
                  //   ImmutableBorrowI
                  // }
                }
                case ShareT => {
                  // if (regionIsMutable(substitutions, perspectiveRegionT, expectRegionPlaceholder(outerRegion))) {
                  MutableShareI
                  // } else {
                  //   ImmutableShareI
                  // }
                }
                case OwnT => {
                  // We don't have this assert because we sometimes can see owning references even
                  // though we dont hold them, see RMLRMO.
                  // vassert(regionIsMutable(substitutions, perspectiveRegionT, expectRegionPlaceholder(outerRegion)))
                  OwnI
                }
                case WeakT => {
                  vregionmut(WeakI)
                }
              }
            }
          }
//        val newRegion = expectRegionTemplata(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, outerRegion))
        CoordTemplataI(RegionTemplataI(0), CoordI(newOwnership, kind))
      }
    }
  }
*/
// mig: fn get_mutability
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn get_mutability(_monouts: &InstantiatedOutputsI<'s, 't, 'i>, kind_it: &KindIT<'s, 'i, cI>) -> MutabilityI {
        match kind_it {
            KindIT::IntIT(_) | KindIT::BoolIT(_) | KindIT::StrIT(_) | KindIT::NeverIT(_) | KindIT::FloatIT(_) | KindIT::VoidIT(_) => MutabilityI::Immutable,
            KindIT::StructIT(s) => *_monouts.struct_to_mutability.get(&s.id).expect("get_mutability: struct not found"),
            KindIT::InterfaceIT(i) => *_monouts.interface_to_mutability.get(&i.id).expect("get_mutability: interface not found"),
            KindIT::RuntimeSizedArrayIT(_) => panic!("Unimplemented: get_mutability RuntimeSizedArray"),
            KindIT::StaticSizedArrayIT(ssa) => {
                match ssa.name.local_name {
                    INameI::StaticSizedArray(n) => n.arr.mutability,
                    _ => panic!("get_mutability StaticSizedArray: local_name not StaticSizedArrayNameI"),
                }
            }
        }
    }
}
/*
  def getMutability(t: KindIT[cI]): MutabilityI = {
    t match {
      case IntIT(_) | BoolIT() | StrIT() | NeverIT(_) | FloatIT() | VoidIT() => ImmutableI
      case StructIT(name) => {
        vassertSome(monouts.structToMutability.get(name))
      }
      case InterfaceIT(name) => {
        vassertSome(monouts.interfaceToMutability.get(name))
      }
      case RuntimeSizedArrayIT(IdI(_, _, RuntimeSizedArrayNameI(_, RawArrayNameI(mutability, _, region)))) => {
        mutability
      }
      case StaticSizedArrayIT(IdI(_, _, StaticSizedArrayNameI(_, _, _, RawArrayNameI(mutability, _, region)))) => {
        mutability
      }
      case other => vimpl(other)
    }
  }
*/
// mig: fn translate_citizen
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_citizen(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _citizen: &ICitizenTT<'s, 't>, _instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) -> ICitizenIT<'s, 'i, sI> {
        match _citizen {
            ICitizenTT::Struct(s) => {
                let s_i = self.translate_struct(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, s, _instantiation_bound_args);
                ICitizenIT::StructIT(self.interner.intern_struct_it_si(crate::instantiating::ast::types::StructITValI { id: s_i.id }))
            }
            ICitizenTT::Interface(i) => {
                let i_i = self.translate_interface(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, i, _instantiation_bound_args);
                ICitizenIT::InterfaceIT(self.interner.intern_interface_it_si(crate::instantiating::ast::types::InterfaceITValI { id: i_i.id }))
            }
        }
    }
}
/*
  def translateCitizen(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    citizen: ICitizenTT,
    instantiationBoundArgs: InstantiationBoundArgumentsI):
  ICitizenIT[sI] = {
    citizen match {
      case s @ StructTT(_) => translateStruct(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, s, instantiationBoundArgs)
      case s @ InterfaceTT(_) => translateInterface(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, s, instantiationBoundArgs)
    }
  }
*/
// mig: fn translate_struct
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_struct(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _struct: &StructTT<'s, 't>, _instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) -> StructIT<'s, 'i, sI> {
        let StructTT { id: full_name, .. } = _struct;
        let translated_id = self.translate_struct_id(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, full_name, _instantiation_bound_args);
        let desired_struct = *self.interner.intern_struct_it_si(crate::instantiating::ast::types::StructITValI { id: translated_id });
        desired_struct
    }
}
/*
  def translateStruct(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    struct: StructTT,
    instantiationBoundArgs: InstantiationBoundArgumentsI):
  StructIT[sI] = {
    val StructTT(fullName) = struct

    val desiredStruct =
      StructIT(
        translateStructId(
          denizenName, denizenBoundToDenizenCallerSuppliedThing,
          substitutions, perspectiveRegionT, fullName, instantiationBoundArgs))

    desiredStruct
  }
*/
// mig: fn translate_interface
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_interface(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _interface: &InterfaceTT<'s, 't>, _instantiation_bound_args: &InstantiationBoundArgumentsI<'s, 'i>) -> InterfaceIT<'s, 'i, sI> {
        let InterfaceTT { id: full_name, .. } = _interface;
        let translated_id = self.translate_interface_id(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, full_name, _instantiation_bound_args);
        *self.interner.intern_interface_it_si(crate::instantiating::ast::types::InterfaceITValI { id: translated_id })
    }
}
/*
  def translateInterface(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    interface: InterfaceTT,
    instantiationBoundArgs: InstantiationBoundArgumentsI):
  InterfaceIT[sI] = {
    val InterfaceTT(fullName) = interface

    val desiredInterface =
      InterfaceIT(
        translateInterfaceId(
          denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, fullName, instantiationBoundArgs))

    desiredInterface
  }
*/
// mig: fn translate_super_kind
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_super_kind(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _kind: &ISuperKindTT<'s, 't>) -> InterfaceIT<'s, 'i, sI> {
        match _kind {
            ISuperKindTT::Interface(i) => {
                let bound_args = self.translate_bound_args_for_callee(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &self.hinputs.get_instantiation_bound_args(i.id));
                self.translate_interface(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, i, &bound_args)
            }
            ISuperKindTT::KindPlaceholder(_) => panic!("Unimplemented: translate_super_kind KindPlaceholder"),
        }
    }
}
/*
  def translateSuperKind(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    kind: ISuperKindTT):
  InterfaceIT[sI] = {
    kind match {
      case i @ InterfaceTT(_) => {
        translateInterface(
          denizenName,
          denizenBoundToDenizenCallerSuppliedThing,
          substitutions,
          perspectiveRegionT,
          i,
          translateBoundArgsForCallee(
            denizenName,
            denizenBoundToDenizenCallerSuppliedThing,
            substitutions,
            perspectiveRegionT,
            hinputs.getInstantiationBoundArgs(i.id)))
      }
      case p @ KindPlaceholderT(_) => {
        translatePlaceholder(substitutions, p) match {
          case s : InterfaceIT[_] => {
            vassert(s.isInstanceOf[InterfaceIT[sI]])
            s.asInstanceOf[InterfaceIT[sI]]
          }
          case other => vwat(other)
        }
      }
    }
  }
*/
// mig: fn translate_placeholder
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_placeholder(&self, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _t: &KindPlaceholderT<'s, 't>) -> KindIT<'s, 'i, sI> {
        panic!("Unimplemented: translate_placeholder");
    }
}
/*
  def translatePlaceholder(
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    t: KindPlaceholderT):
  KindIT[sI] = {
    // Per @PASDZ, t.id is path-encoded so it identifies the placeholder uniquely across any
    // denizens whose placeholders happen to coexist in this substitutions map.
    val newSubstitutingTemplata = vassertSome(substitutions.get(t.id))
    ITemplataI.expectKindTemplata(newSubstitutingTemplata).kind
  }
*/
// mig: fn translate_static_sized_array
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_static_sized_array(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, perspective_region_t: &RegionT, ssa_tt: &StaticSizedArrayTT<'s, 't>) -> StaticSizedArrayIT<'s, 'i, sI> {
        let StaticSizedArrayTT { name: id_t, .. } = ssa_tt;
        let IdT { package_coord, init_steps, local_name, .. } = *id_t;
        let ssa_name_t = match local_name {
            INameT::StaticSizedArray(n) => *n,
            _ => panic!("translate_static_sized_array: local_name not StaticSizedArrayNameT"),
        };
        let crate::typing::names::names::StaticSizedArrayNameT { template: _, size: size_t, variability: variability_t, arr } = ssa_name_t;
        let crate::typing::names::names::RawArrayNameT { mutability: mutability_t, element_type: element_type_t, self_region: _ } = *arr;
        let new_perspective_region_t = RegionT { region: crate::typing::types::types::IRegionT::Default };
        let _ssa_region = RegionT { region: crate::typing::types::types::IRegionT::Default };
        let int_templata = crate::instantiating::ast::templata::expect_integer_templata(self.translate_templata(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, &new_perspective_region_t, &size_t)).value;
        let variability_templata = crate::instantiating::ast::templata::expect_variability_templata(self.translate_templata(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, &new_perspective_region_t, &variability_t)).variability;
        let mutability_templata = crate::instantiating::ast::templata::expect_mutability_templata(self.translate_templata(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, &new_perspective_region_t, &mutability_t)).mutability;
        let element_type = self.translate_coord(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, &new_perspective_region_t, &element_type_t);
        let translated_init_steps: Vec<INameI<'s, 'i, sI>> = init_steps.iter().map(|n| Self::translate_name(n)).collect();
        let local_name_i = INameI::StaticSizedArray(self.interner.alloc(crate::instantiating::ast::names::StaticSizedArrayNameI {
            template: crate::instantiating::ast::names::StaticSizedArrayTemplateNameI(std::marker::PhantomData),
            size: int_templata,
            variability: variability_templata,
            arr: crate::instantiating::ast::names::RawArrayNameI {
                mutability: mutability_templata,
                element_type,
                self_region: crate::instantiating::ast::templata::RegionTemplataI { pure_height: 0, _marker: std::marker::PhantomData },
            },
        }));
        let id_i = IdI {
            package_coord,
            init_steps: self.interner.alloc_slice_from_vec(translated_init_steps),
            local_name: local_name_i,
        };
        *self.interner.intern_static_sized_array_it_si(crate::instantiating::ast::types::StaticSizedArrayITValI { name: id_i })
    }
}
/*
  def translateStaticSizedArray(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    ssaTT: StaticSizedArrayTT):
  StaticSizedArrayIT[sI] = {
    val StaticSizedArrayTT(
    IdT(
    packageCoord,
    initSteps,
    StaticSizedArrayNameT(StaticSizedArrayTemplateNameT(), sizeT, variabilityT, RawArrayNameT(mutabilityT, elementTypeT, _)))) = ssaTT

    val newPerspectiveRegionT = RegionT(DefaultRegionT)
      // ssaRegionT match {
      //   case PlaceholderTemplataT(IdT(packageCoord, initSteps, r @ RegionPlaceholderNameT(_, _, _, _)), RegionTemplataType()) => {
      //     IdT(packageCoord, initSteps, r)
      //   }
      //   case _ => vwat()
      // }

    // We use newPerspectiveRegionT for these because of TTTDRM.
    val ssaRegion = RegionT(DefaultRegionT)
    // We dont have this assert because this might be a templata deep in a struct or function's
    // name, so the heights might actually be negative.
    // vassert(Some(ssaRegion.pureHeight) == newPerspectiveRegionT.localName.pureHeight)
    val intTemplata = ITemplataI.expectIntegerTemplata(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, newPerspectiveRegionT, sizeT)).value
    val variabilityTemplata = ITemplataI.expectVariabilityTemplata(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, newPerspectiveRegionT, variabilityT)).variability
    val mutabilityTemplata =
      ITemplataI.expectMutabilityTemplata(
        translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, newPerspectiveRegionT, mutabilityT)).mutability
    val elementType =
      translateCoord(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, newPerspectiveRegionT, elementTypeT)

    StaticSizedArrayIT(
      IdI(
        packageCoord,
        initSteps.map(translateName(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
        StaticSizedArrayNameI(
          StaticSizedArrayTemplateNameI(),
          intTemplata,
          variabilityTemplata,
          RawArrayNameI(
            mutabilityTemplata,
            elementType,
            RegionTemplataI(0)))))
  }
*/
// mig: fn translate_runtime_sized_array
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_runtime_sized_array(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _rsa_tt: &RuntimeSizedArrayTT<'s, 't>) -> RuntimeSizedArrayIT<'s, 'i, sI> {
        panic!("Unimplemented: translate_runtime_sized_array");
    }
}
/*
  def translateRuntimeSizedArray(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
      rsaTT: RuntimeSizedArrayTT):
  RuntimeSizedArrayIT[sI] = {
    val RuntimeSizedArrayTT(
      IdT(
      packageCoord,
      initSteps,
      RuntimeSizedArrayNameT(RuntimeSizedArrayTemplateNameT(), RawArrayNameT(mutabilityT, elementTypeT, _)))) = rsaTT

    val newPerspectiveRegionT = RegionT(DefaultRegionT)
      // rsaRegionT match {
      //   case PlaceholderTemplataT(IdT(packageCoord, initSteps, r @ RegionPlaceholderNameT(_, _, _, _)), RegionTemplataType()) => {
      //     IdT(packageCoord, initSteps, r)
      //   }
      //   case _ => vwat()
      // }

    // We use newPerspectiveRegionT for these because of TTTDRM.
    val rsaRegion = RegionT(DefaultRegionT)
    // We dont have this assert because this might be a templata deep in a struct or function's
    // name, so the heights might actually be negative.
    // vassert(Some(ssaRegion.pureHeight) == newPerspectiveRegionT.localName.pureHeight)
    val mutabilityTemplata =
      ITemplataI.expectMutabilityTemplata(
        translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, newPerspectiveRegionT, mutabilityT)).mutability
    val elementType = translateCoord(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, newPerspectiveRegionT, elementTypeT)

    RuntimeSizedArrayIT(
      IdI(
        packageCoord,
        initSteps.map(translateName(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
        RuntimeSizedArrayNameI(
          RuntimeSizedArrayTemplateNameI(),
          RawArrayNameI(
            mutabilityTemplata,
            elementType,
            RegionTemplataI(0)))))
  }
*/
// mig: fn translate_kind
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_kind(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, kind_t: &KindT<'s, 't>) -> KindIT<'s, 'i, sI> {
        match kind_t {
            KindT::Int(int_t) => KindIT::IntIT(IntIT { bits: int_t.bits, _marker: std::marker::PhantomData }),
            KindT::Bool(_) => KindIT::BoolIT(BoolIT { _marker: std::marker::PhantomData }),
            KindT::Float(_) => KindIT::FloatIT(FloatIT { _marker: std::marker::PhantomData }),
            KindT::Void(_) => KindIT::VoidIT(VoidIT { _marker: std::marker::PhantomData }),
            KindT::Str(_) => KindIT::StrIT(StrIT { _marker: std::marker::PhantomData }),
            KindT::Never(never_t) => KindIT::NeverIT(NeverIT { from_break: never_t.from_break, _marker: std::marker::PhantomData }),
            KindT::KindPlaceholder(_p) => panic!("Unimplemented: translate_kind KindPlaceholder"),
            KindT::Struct(s) => {
                let bound_args = self.translate_bound_args_for_callee(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &self.hinputs.get_instantiation_bound_args(s.id));
                let struct_it = self.translate_struct(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, s, &bound_args);
                KindIT::StructIT(self.interner.alloc(struct_it))
            }
            KindT::Interface(s) => {
                let bound_args = self.translate_bound_args_for_callee(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &self.hinputs.get_instantiation_bound_args(s.id));
                let interface_it = self.translate_interface(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, s, &bound_args);
                KindIT::InterfaceIT(self.interner.alloc(interface_it))
            }
            KindT::StaticSizedArray(a) => KindIT::StaticSizedArrayIT(self.interner.alloc(self.translate_static_sized_array(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, a))),
            KindT::RuntimeSizedArray(_a) => panic!("Unimplemented: translate_kind RuntimeSizedArray"),
            _other => panic!("Unimplemented: translate_kind other"),
        }
    }
}
/*
  def translateKind(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    kind: KindT):
  KindIT[sI] = {
    kind match {
      case IntT(bits) => IntIT(bits)
      case BoolT() => BoolIT()
      case FloatT() => FloatIT()
      case VoidT() => VoidIT()
      case StrT() => StrIT()
      case NeverT(fromBreak) => NeverIT(fromBreak)
      case p @ KindPlaceholderT(_) => translatePlaceholder(substitutions, p)
      case s @ StructTT(_) => {
        translateStruct(
          denizenName,
          denizenBoundToDenizenCallerSuppliedThing,
          substitutions,
          perspectiveRegionT,
          s,
          translateBoundArgsForCallee(denizenName, denizenBoundToDenizenCallerSuppliedThing,
            substitutions, perspectiveRegionT, hinputs.getInstantiationBoundArgs(s.id)))
      }
      case s @ InterfaceTT(_) => {
        translateInterface(
          denizenName,
          denizenBoundToDenizenCallerSuppliedThing,
          substitutions,
          perspectiveRegionT,
          s,
          translateBoundArgsForCallee(
            denizenName, denizenBoundToDenizenCallerSuppliedThing,
            substitutions, perspectiveRegionT, hinputs.getInstantiationBoundArgs(s.id)))
      }
      case a @ contentsStaticSizedArrayTT(_, _, _, _, _) => translateStaticSizedArray(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, a)
      case a @ contentsRuntimeSizedArrayTT(_, _, _) => translateRuntimeSizedArray(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, a)
      case other => vimpl(other)
    }
  }
*/
// mig: fn translate_parameter
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_parameter(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, perspective_region_t: &RegionT, param_t: &ParameterT<'s, 't>) -> ParameterI<'s, 'i> {
        let ParameterT { name, virtuality, pre_checked, tyype } = param_t;
        let type_it =
            self.translate_coord(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, tyype)
                .coord;
        let name_s = Self::translate_var_name(self.interner, name);
        ParameterI {
            name: region_collapser_individual::collapse_var_name(self.interner, &name_s),
            virtuality: virtuality.map(|v| match v { AbstractT => AbstractI }),
            pre_checked: *pre_checked,
            tyype: region_collapser_individual::collapse_coord(self.interner, &type_it),
        }
    }
}
/*
  def translateParameter(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    param: ParameterT):
  ParameterI = {
    val ParameterT(name, virtuality, preChecked, tyype) = param
    val typeIT =
      translateCoord(
        denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, tyype)
          .coord
    val nameS = translateVarName(name)
    ParameterI(
      RegionCollapserIndividual.collapseVarName(nameS),
      virtuality.map({ case AbstractT() => AbstractI() }),
      preChecked,
      RegionCollapserIndividual.collapseCoord(typeIT))
  }
*/
// mig: fn translate_templata
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_templata(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, templata_t: &ITemplataT<'s, 't>) -> ITemplataI<'s, 'i, sI> {
        let result = match templata_t {
            ITemplataT::Placeholder(p) => {
                let crate::typing::templata::templata::PlaceholderTemplataT { id: n, tyype: _ } = **p;
                *_substitutions.get(&n).expect("translate_templata Placeholder: substitution missing")
            }
            ITemplataT::Integer(value) => ITemplataI::Integer(IntegerTemplataI { value: *value, _marker: std::marker::PhantomData }),
            ITemplataT::Boolean(_) => panic!("Unimplemented: translate_templata Boolean"),
            ITemplataT::String(_) => panic!("Unimplemented: translate_templata String"),
            ITemplataT::Coord(c) => ITemplataI::Coord(self.translate_coord(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &c.coord)),
            ITemplataT::Mutability(m) => ITemplataI::Mutability(crate::instantiating::ast::templata::MutabilityTemplataI { mutability: Self::translate_mutability(&m.mutability), _marker: std::marker::PhantomData }),
            ITemplataT::Variability(v) => ITemplataI::Variability(crate::instantiating::ast::templata::VariabilityTemplataI { variability: Self::translate_variability(&v.variability), _marker: std::marker::PhantomData }),
            ITemplataT::Kind(_) => panic!("Unimplemented: translate_templata Kind"),
            _ => panic!("Unimplemented: translate_templata other"),
        };
        // Scala `if (opts.sanityCheck) { vassert(Collector.all(result, { case KindPlaceholderNameT(_) => }).isEmpty) }`
        // is omitted per SPDMX Exception Y: KindPlaceholderNameT has no I-side counterpart (INameI has no
        // KindPlaceholder variant), so the collector's partial function can never match — the check is vacuous.
        result
    }
}
/*
  def translateTemplata(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    templata: ITemplataT[ITemplataType]):
  ITemplataI[sI] = {
    val result =
      templata match {
        case PlaceholderTemplataT(n, tyype) => {
          // Per @PASDZ, n is path-encoded — sufficient on its own as a substitutions lookup key.
          val substitution =
            vassertSome(substitutions.get(n))
          substitution
        }
        case IntegerTemplataT(value) => IntegerTemplataI[sI](value)
        case BooleanTemplataT(value) => BooleanTemplataI[sI](value)
        case StringTemplataT(value) => StringTemplataI[sI](value)
        case CoordTemplataT(coord) => {
          translateCoord(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, coord)
        }
        case MutabilityTemplataT(mutability) => MutabilityTemplataI[sI](translateMutability(mutability))
        case VariabilityTemplataT(variability) => VariabilityTemplataI[sI](translateVariability(variability))
        case KindTemplataT(kind) => KindTemplataI[sI](translateKind(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, kind))
        case other => vimpl(other)
      }
    if (opts.sanityCheck) {
      vassert(Collector.all(result, { case KindPlaceholderNameT(_) => }).isEmpty)
    }
    result
  }
*/
// mig: fn translate_var_name
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_var_name(interner: &InstantiatingInterner<'s, 'i>, name: &IVarNameT<'s, 't>) -> IVarNameI<'s, 'i, sI> {
        match name {
            IVarNameT::TypingPassFunctionResultVar(_) => IVarNameI::TypingPassFunctionResultVar(interner.intern_typing_pass_function_result_var_name_si(TypingPassFunctionResultVarNameI(std::marker::PhantomData))),
            IVarNameT::CodeVar(x) => IVarNameI::CodeVar(interner.intern_code_var_name_si(CodeVarNameI { _marker: std::marker::PhantomData, name: x.name })),
            IVarNameT::ClosureParam(crate::typing::names::names::ClosureParamNameT { code_location, .. }) => IVarNameI::ClosureParam(interner.intern_closure_param_name_si(crate::instantiating::ast::names::ClosureParamNameI { _marker: std::marker::PhantomData, code_location: *code_location })),
            IVarNameT::TypingPassBlockResultVar(crate::typing::names::names::TypingPassBlockResultVarNameT { life: crate::typing::ast::ast::LocationInFunctionEnvironmentT { path, .. } }) => {
                IVarNameI::TypingPassBlockResultVar(interner.intern_typing_pass_block_result_var_name_si(crate::instantiating::ast::names::TypingPassBlockResultVarNameI {
                    _marker: std::marker::PhantomData,
                    life: crate::instantiating::ast::ast::LocationInFunctionEnvironmentI { path: interner.alloc_slice_from_vec(path.to_vec()) },
                }))
            }
            IVarNameT::TypingPassTemporaryVar(crate::typing::names::names::TypingPassTemporaryVarNameT { life: crate::typing::ast::ast::LocationInFunctionEnvironmentT { path, .. } }) => {
                IVarNameI::TypingPassTemporaryVar(interner.intern_typing_pass_temporary_var_name_si(crate::instantiating::ast::names::TypingPassTemporaryVarNameI {
                    _marker: std::marker::PhantomData,
                    life: crate::instantiating::ast::ast::LocationInFunctionEnvironmentI { path: interner.alloc_slice_from_vec(path.to_vec()) },
                }))
            }
            IVarNameT::ConstructingMember(x) => IVarNameI::ConstructingMember(interner.intern_constructing_member_name_si(crate::instantiating::ast::names::ConstructingMemberNameI { _marker: std::marker::PhantomData, name: x.name })),
            IVarNameT::Iterable(_) => panic!("Unimplemented: translate_var_name Iterable"),
            IVarNameT::Iterator(_) => panic!("Unimplemented: translate_var_name Iterator"),
            IVarNameT::IterationOption(_) => panic!("Unimplemented: translate_var_name IterationOption"),
            IVarNameT::MagicParam(crate::typing::names::names::MagicParamNameT { code_location2, .. }) => IVarNameI::MagicParam(interner.intern_magic_param_name_si(crate::instantiating::ast::names::MagicParamNameI { _marker: std::marker::PhantomData, code_location_2: *code_location2 })),
            IVarNameT::Self_(_) => panic!("Unimplemented: translate_var_name SelfName"),
            _ => panic!("Unimplemented: translate_var_name other"),
        }
    }
}
/*
Guardian: temp-disable: SPDMX — Vacuous Collector.all: Scala `Collector.all(result, { case KindPlaceholderNameT(_) => }).isEmpty` collects typing-pass KindPlaceholderNameT, but the I-side INameI has no KindPlaceholder variant, so the predicate can never be written (variant doesn't exist) nor match — the check is vacuous. Cannot be panic-stubbed because opts.sanity_check is live for the driving test; omitted with a marker comment (Exception Y was removed; architect authorized temp-disable for these). — /Volumes/V/Vale/FrontendRust/guardian-logs/request-1004-1780094668814/hook-1004/translate_templata--4673.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def translateVarName(
    name: IVarNameT):
  IVarNameI[sI] = {
    name match {
      case TypingPassFunctionResultVarNameT() => TypingPassFunctionResultVarNameI()
      case CodeVarNameT(x) => CodeVarNameI(x)
      case ClosureParamNameT(x) => ClosureParamNameI(x)
      case TypingPassBlockResultVarNameT(LocationInFunctionEnvironmentT(path)) => TypingPassBlockResultVarNameI(LocationInFunctionEnvironmentI(path))
      case TypingPassTemporaryVarNameT(LocationInFunctionEnvironmentT(path)) => TypingPassTemporaryVarNameI(LocationInFunctionEnvironmentI(path))
      case ConstructingMemberNameT(x) => ConstructingMemberNameI(x)
      case IterableNameT(range) => IterableNameI(range)
      case IteratorNameT(range) => IteratorNameI(range)
      case IterationOptionNameT(range) => IterationOptionNameI(range)
      case MagicParamNameT(codeLocation2) => MagicParamNameI(codeLocation2)
      case SelfNameT() => SelfNameI()
      case other => vimpl(other)
    }
  }
*/
// mig: fn translate_function_template_name
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_function_template_name(_func_template_name_t: &IFunctionTemplateNameT<'s, 't>) -> IFunctionTemplateNameI<'s, 'i, sI> {
        panic!("Unimplemented: translate_function_template_name");
    }
}
/*
  def translateFunctionTemplateName(name: IFunctionTemplateNameT): IFunctionTemplateNameI[sI] = {
    name match {
      case FunctionTemplateNameT(humanName, codeLocation) => FunctionTemplateNameI(humanName, codeLocation)
      case other => vimpl(other)
    }
  }
*/
// mig: fn translate_function_name
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_function_name(&self, monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, denizen_name: &IdT<'s, 't>, denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, perspective_region_t: &RegionT, name: &IFunctionNameT<'s, 't>) -> IFunctionNameI<'s, 'i, sI> {
        match *name {
            IFunctionNameT::Function(function_name_t) => {
                let FunctionNameT { template: function_template_name_t, template_args, parameters: params, .. } = *function_name_t;
                let FunctionTemplateNameT { human_name, code_location: code_loc, .. } = *function_template_name_t;
                IFunctionNameI::Function(
                    self.interner.intern_function_name_x_si(FunctionNameIX {
                        template: FunctionTemplateNameI { _marker: std::marker::PhantomData, human_name, code_location: code_loc },
                        template_args: self.interner.alloc_slice_from_vec(
                            template_args.iter().map(|template_arg| self.translate_templata(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, template_arg)).collect::<Vec<_>>()),
                        parameters: self.interner.alloc_slice_from_vec(
                            params.iter().map(|param| self.translate_coord(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, param).coord).collect::<Vec<_>>()),
                    }))
            }
            IFunctionNameT::ForwarderFunction(_) => panic!("Unimplemented: translate_function_name ForwarderFunction"),
            IFunctionNameT::ExternFunction(n) => {
                let ExternFunctionNameT { human_name, template_args, parameters, .. } = *n;
                IFunctionNameI::ExternFunction(
                    self.interner.intern_extern_function_name_si(ExternFunctionNameI {
                        human_name,
                        template_args: self.interner.alloc_slice_from_vec(template_args.iter().map(|template_arg| self.translate_templata(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, template_arg)).collect::<Vec<_>>()),
                        parameters: self.interner.alloc_slice_from_vec(parameters.iter().map(|param| self.translate_coord(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, param).coord).collect::<Vec<_>>()),
                    }))
            }
            IFunctionNameT::FunctionBound(fbn) => {
                let crate::typing::names::names::FunctionBoundNameT { template, template_args, parameters: params, .. } = *fbn;
                let crate::typing::names::names::FunctionBoundTemplateNameT { human_name, .. } = *template;
                IFunctionNameI::FunctionBound(
                    self.interner.intern_function_bound_name_si(FunctionBoundNameI {
                        template: FunctionBoundTemplateNameI { _marker: std::marker::PhantomData, human_name },
                        template_args: self.interner.alloc_slice_from_vec(
                            template_args.iter().map(|template_arg| self.translate_templata(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, template_arg)).collect::<Vec<_>>()),
                        parameters: self.interner.alloc_slice_from_vec(
                            params.iter().map(|param| self.translate_coord(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, param).coord).collect::<Vec<_>>()),
                    }))
            }
            IFunctionNameT::AnonymousSubstructConstructor(_) => panic!("Unimplemented: translate_function_name AnonymousSubstructConstructor"),
            IFunctionNameT::LambdaCallFunction(n) => {
                let LambdaCallFunctionNameT { template: LambdaCallFunctionTemplateNameT { code_location, param_types: param_types_for_generic, .. }, template_args, parameters: param_types, .. } = *n;
                IFunctionNameI::LambdaCallFunction(
                    self.interner.intern_lambda_call_function_name_si(LambdaCallFunctionNameI {
                        template: *self.interner.intern_lambda_call_function_template_name_si(LambdaCallFunctionTemplateNameI {
                            _marker: std::marker::PhantomData,
                            code_location: *code_location,
                            param_types: self.interner.alloc_slice_from_vec(param_types_for_generic.iter().map(|p| self.translate_coord(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, p).coord).collect::<Vec<_>>()),
                        }),
                        template_args: self.interner.alloc_slice_from_vec(template_args.iter().map(|t| self.translate_templata(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, t)).collect::<Vec<_>>()),
                        parameters: self.interner.alloc_slice_from_vec(param_types.iter().map(|p| self.translate_coord(monouts, denizen_name, denizen_bound_to_denizen_caller_supplied_thing, substitutions, perspective_region_t, p).coord).collect::<Vec<_>>()),
                    }))
            }
            IFunctionNameT::OverrideDispatcher(_) => panic!("Unimplemented: translate_function_name OverrideDispatcher"),
            _other => panic!("Unimplemented: translate_function_name other"),
        }
    }
}
/*
  def translateFunctionName(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    name: IFunctionNameT):
  IFunctionNameI[sI] = {
    name match {
      case FunctionNameT(FunctionTemplateNameT(humanName, codeLoc), templateArgs, params) => {
        FunctionNameIX(
          FunctionTemplateNameI(humanName, codeLoc),
          templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
          params.map(translateCoord(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _).coord))
      }
      case ForwarderFunctionNameT(ForwarderFunctionTemplateNameT(innerTemplate, index), inner) => {
        ForwarderFunctionNameI(
          ForwarderFunctionTemplateNameI(
            translateFunctionTemplateName(innerTemplate),
            index),
          translateFunctionName(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, inner))
      }
      case ExternFunctionNameT(humanName, templateArgs, parameters) => {
        ExternFunctionNameI(
          humanName,
          templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
          parameters.map(translateCoord(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _).coord))
      }
      case FunctionBoundNameT(FunctionBoundTemplateNameT(humanName), templateArgs, params) => {
        FunctionBoundNameI(
          FunctionBoundTemplateNameI(humanName),
          templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
          params.map(translateCoord(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _).coord))
      }
      case FunctionBoundNameT(FunctionBoundTemplateNameT(humanName), templateArgs, params) => {
        ReachableFunctionNameI(
          ReachableFunctionTemplateNameI(humanName),
          templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
          params.map(translateCoord(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _).coord))
      }
      case AnonymousSubstructConstructorNameT(template, templateArgs, params) => {
        AnonymousSubstructConstructorNameI(
          translateName(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, template) match {
            case x @ AnonymousSubstructConstructorTemplateNameI(_) => x
            case other => vwat(other)
          },
          templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
          params.map(translateCoord(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _).coord))
      }
      // Per @LAGTNGZ, paramTypesForGeneric carries the argTypes that distinguish one lambda's __call from another.
      case LambdaCallFunctionNameT(LambdaCallFunctionTemplateNameT(codeLocation, paramTypesForGeneric), templateArgs, paramTypes) => {
        LambdaCallFunctionNameI(
          LambdaCallFunctionTemplateNameI(
            codeLocation,
            paramTypesForGeneric.map(translateCoord(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _).coord)),
          templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
          paramTypes.map(translateCoord(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _).coord))
      }
      case OverrideDispatcherNameT(OverrideDispatcherTemplateNameT(implTemplateId), templateArgs, paramTypes) => {
        OverrideDispatcherNameI(
          OverrideDispatcherTemplateNameI(
            translateId[IImplTemplateNameT, IImplTemplateNameI[sI]](implTemplateId, translateImplTemplateName)),
          templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
          paramTypes.map(translateCoord(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _).coord))
      }
//      case FunctionBoundNameT(FunctionBoundTemplateNameT(humanName, runeInImpl, runeInCitizen), templateArgs, paramTypes) => {
//        CaseFunctionFromImplNameI(
//          CaseFunctionFromImplTemplateNameI(humanName, runeInImpl, runeInCitizen),
//          templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
//          paramTypes.map(translateCoord(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _).coord))
//      }
      case other => vimpl(other)
    }
  }
*/
// mig: fn translate_impl_name
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_impl_name(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _name: &IImplNameT<'s, 't>) -> IImplNameI<'s, 'i, sI> {
        match _name {
            IImplNameT::Impl(n) => {
                let crate::typing::names::names::ImplNameT { template: crate::typing::names::names::ImplTemplateNameT { code_location_s, .. }, template_args, sub_citizen, .. } = **n;
                let template_args_i: Vec<ITemplataI<'s, 'i, sI>> = template_args.iter().map(|t| self.translate_templata(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, t)).collect();
                let sub_citizen_id = sub_citizen.id();
                let bound_args_for_callee = self.translate_bound_args_for_callee(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &self.hinputs.get_instantiation_bound_args(sub_citizen_id));
                let sub_citizen_i = self.translate_citizen(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &sub_citizen, &bound_args_for_callee);
                IImplNameI::Impl(self.interner.intern_impl_name_si(crate::instantiating::ast::names::ImplNameI {
                    template: crate::instantiating::ast::names::IImplTemplateNameI::ImplTemplate(self.interner.intern_impl_template_name_si(crate::instantiating::ast::names::ImplTemplateNameI { _marker: std::marker::PhantomData, code_location_s: *code_location_s })),
                    template_args: self.interner.bump().alloc_slice_fill_iter(template_args_i.into_iter()),
                    sub_citizen: sub_citizen_i,
                }))
            }
            IImplNameT::ImplBound(_) => panic!("Unimplemented: translate_impl_name ImplBound"),
            IImplNameT::AnonymousSubstructImpl(_) => panic!("Unimplemented: translate_impl_name AnonymousSubstructImpl"),
        }
    }
}
/*
  def translateImplName(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    name: IImplNameT):
  IImplNameI[sI] = {
    name match {
      case ImplNameT(ImplTemplateNameT(codeLocationS), templateArgs, subCitizen) => {
        ImplNameI(
          ImplTemplateNameI(codeLocationS),
          templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
          translateCitizen(denizenName, denizenBoundToDenizenCallerSuppliedThing,
            substitutions,
            perspectiveRegionT,
            subCitizen,
            translateBoundArgsForCallee(
              denizenName,
              denizenBoundToDenizenCallerSuppliedThing,
              substitutions,
              perspectiveRegionT,
              hinputs.getInstantiationBoundArgs(subCitizen.id))))
      }
      case ImplBoundNameT(ImplBoundTemplateNameT(codeLocationS), templateArgs) => {
        ImplBoundNameI(
          ImplBoundTemplateNameI(codeLocationS),
          templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)))
      }
      case AnonymousSubstructImplNameT(AnonymousSubstructImplTemplateNameT(interface), templateArgs, subCitizen) => {
        AnonymousSubstructImplNameI(
          AnonymousSubstructImplTemplateNameI(
            translateInterfaceTemplateName(interface)),
          templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)),
          translateCitizen(
            denizenName,
            denizenBoundToDenizenCallerSuppliedThing,
            substitutions,
            perspectiveRegionT,
            subCitizen,
            translateBoundArgsForCallee(
              denizenName,
              denizenBoundToDenizenCallerSuppliedThing,
              substitutions,
              perspectiveRegionT,
              hinputs.getInstantiationBoundArgs(subCitizen.id))))
      }
    }
  }
*/
// mig: fn translate_impl_template_name
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_impl_template_name(_name: &IImplTemplateNameT<'s, 't>) -> IImplTemplateNameI<'s, 'i, sI> {
        panic!("Unimplemented: translate_impl_template_name");
    }
}
/*
  def translateImplTemplateName(
      name: IImplTemplateNameT):
  IImplTemplateNameI[sI] = {
    name match {
      case ImplTemplateNameT(codeLocationS) => ImplTemplateNameI(codeLocationS)
      case ImplBoundTemplateNameT(codeLocationS) => ImplBoundTemplateNameI(codeLocationS)
      case AnonymousSubstructImplTemplateNameT(interface) => {
        AnonymousSubstructImplTemplateNameI(
          translateInterfaceTemplateName(interface))
      }
    }
  }

//  def translateRegionName(
//    substitutions: Map[IdT[INameT], Map[IdT[IPlaceholderNameT], ITemplata[ITemplataType]]],
//    perspectiveRegionT: GlobalRegionT,
//    name: IRegionNameT):
//  IRegionNameT = {
//    name match {
//      case RegionPlaceholderNameT(index, rune, originallyIntroducedLocation, originallyMutable) => {
//
//      }
//    }
//  }
*/
// mig: fn translate_struct_name
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_struct_name(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _name: &IStructNameT<'s, 't>) -> IStructNameI<'s, 'i, sI> {
        let new_perspective_region_t = RegionT { region: IRegionT::Default };
        match _name {
            IStructNameT::Struct(StructNameT { template: IStructTemplateNameT::StructTemplate(StructTemplateNameT { human_name, .. }), template_args, .. }) => {
                let template_args_si: Vec<ITemplataI<'s, 'i, sI>> = template_args.iter().map(|t| self.translate_templata(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, &new_perspective_region_t, t)).collect();
                IStructNameI::Struct(self.interner.intern_struct_name_si(crate::instantiating::ast::names::StructNameI {
                    template: IStructTemplateNameI::StructTemplate(self.interner.intern_struct_template_name_si(crate::instantiating::ast::names::StructTemplateNameI { _marker: std::marker::PhantomData, human_name: *human_name })),
                    template_args: self.interner.bump().alloc_slice_fill_iter(template_args_si.into_iter()),
                }))
            }
            IStructNameT::AnonymousSubstruct(_) => panic!("translate_struct_name: AnonymousSubstruct branch"),
            IStructNameT::LambdaCitizen(LambdaCitizenNameT { template: LambdaCitizenTemplateNameT { code_location, .. } }) => {
                IStructNameI::LambdaCitizen(self.interner.intern_lambda_citizen_name_si(crate::instantiating::ast::names::LambdaCitizenNameI {
                    template: *self.interner.intern_lambda_citizen_template_name_si(crate::instantiating::ast::names::LambdaCitizenTemplateNameI { _marker: std::marker::PhantomData, code_location: *code_location }),
                }))
            }
            other => panic!("translate_struct_name: unimplemented variant {:?}", std::mem::discriminant(other)),
        }
    }
}
/*
  def translateStructName(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    // See TTTDRM, this is the region from which we're determining other regions' mutabilities.
    perspectiveRegionT: RegionT,
    name: IStructNameT):
  IStructNameI[sI] = {
    val newPerspectiveRegionT = RegionT(DefaultRegionT)
      // vassertSome(name.templateArgs.lastOption) match {
      //   case PlaceholderTemplataT(IdT(packageCoord, initSteps, r @ RegionPlaceholderNameT(_, _, _, _)), RegionTemplataType()) => {
      //     IdT(packageCoord, initSteps, r)
      //   }
      //   case _ => vwat()
      // }
    name match {
      case StructNameT(StructTemplateNameT(humanName), templateArgs) => {
        StructNameI(
          StructTemplateNameI(humanName),
          // We use newPerspectiveRegionT here because of TTTDRM.
          templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, newPerspectiveRegionT, _)))
      }
      case AnonymousSubstructNameT(AnonymousSubstructTemplateNameT(interface), templateArgs) => {
        AnonymousSubstructNameI(
          AnonymousSubstructTemplateNameI(
            translateInterfaceTemplateName(interface)),
          // We use newPerspectiveRegionT here because of TTTDRM.
          templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, newPerspectiveRegionT, _)))
      }
      case LambdaCitizenNameT(LambdaCitizenTemplateNameT(codeLocation)) => {
        LambdaCitizenNameI(LambdaCitizenTemplateNameI(codeLocation))
      }
      case other => vimpl(other)
    }
  }
*/
// mig: fn translate_interface_name
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_interface_name(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, _name: &IInterfaceNameT<'s, 't>) -> IInterfaceNameI<'s, 'i, sI> {
        match _name {
            IInterfaceNameT::Interface(crate::typing::names::names::InterfaceNameT { template: crate::typing::names::names::InterfaceTemplateNameT { human_namee: human_name, .. }, template_args, .. }) => {
                let template_args_si: Vec<ITemplataI<'s, 'i, sI>> = template_args.iter().map(|t| self.translate_templata(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, t)).collect();
                IInterfaceNameI::Interface(self.interner.intern_interface_name_si(crate::instantiating::ast::names::InterfaceNameI {
                    template: crate::instantiating::ast::names::IInterfaceTemplateNameI::InterfaceTemplate(self.interner.intern_interface_template_name_si(crate::instantiating::ast::names::InterfaceTemplateNameI { _marker: std::marker::PhantomData, human_namee: *human_name })),
                    template_args: self.interner.bump().alloc_slice_fill_iter(template_args_si.into_iter()),
                }))
            }
            other => panic!("translate_interface_name: unimplemented variant {:?}", std::mem::discriminant(other)),
        }
    }
}
/*
  def translateInterfaceName(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    name: IInterfaceNameT):
  IInterfaceNameI[sI] = {
    name match {
      case InterfaceNameT(InterfaceTemplateNameT(humanName), templateArgs) => {
        InterfaceNameI(
          InterfaceTemplateNameI(humanName),
          templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)))
      }
      case other => vimpl(other)
    }
  }
*/
// mig: fn translate_interface_template_name
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_interface_template_name(_name: &IInterfaceTemplateNameT<'s, 't>) -> IInterfaceTemplateNameI<'s, 'i, sI> {
        panic!("Unimplemented: translate_interface_template_name");
    }
}
/*
  def translateInterfaceTemplateName(
    name: IInterfaceTemplateNameT):
  IInterfaceTemplateNameI[sI] = {
    name match {
      case InterfaceTemplateNameT(humanName) => InterfaceTemplateNameI(humanName)
      case other => vimpl(other)
    }
  }
*/
// mig: fn translate_name
// Rust adaptation (SPDMX-S): Scala overloads `translateName`. The 1-arg
// `translateName(t: INameT)` is `translate_name` above; this 5-arg version is
// suffixed `_substituting` to disambiguate (Rust lacks overloading). Slice pipeline
// emitted no stub for this overload (TL-added per NNDX escalation).
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_name_substituting(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _perspective_region_t: &RegionT, name: &INameT<'s, 't>) -> INameI<'s, 'i, sI> {
        match *name {
            n if IVarNameT::try_from(n).is_ok() => panic!("Unimplemented: translate_name_substituting IVarNameT"),
            INameT::KindPlaceholderTemplate(_) => panic!("translate_name_substituting: KindPlaceholderTemplate vwat"),
            INameT::KindPlaceholder(_) => panic!("translate_name_substituting: KindPlaceholder vwat"),
            INameT::Struct(_) => panic!("Unimplemented: translate_name_substituting Struct"),
            INameT::ForwarderFunctionTemplate(_) => panic!("Unimplemented: translate_name_substituting ForwarderFunctionTemplate"),
            INameT::AnonymousSubstructConstructorTemplate(_) => panic!("Unimplemented: translate_name_substituting AnonymousSubstructConstructorTemplate"),
            INameT::FunctionTemplate(ftn) => {
                let FunctionTemplateNameT { human_name, code_location: code_loc, .. } = *ftn;
                INameI::FunctionTemplate(self.interner.intern_function_template_name_si(FunctionTemplateNameI { _marker: std::marker::PhantomData, human_name, code_location: code_loc }))
            }
            INameT::StructTemplate(stn) => {
                let crate::typing::names::names::StructTemplateNameT { human_name, .. } = *stn;
                INameI::StructTemplate(self.interner.intern_struct_template_name_si(crate::instantiating::ast::names::StructTemplateNameI { _marker: std::marker::PhantomData, human_name }))
            }
            INameT::LambdaCitizenTemplate(LambdaCitizenTemplateNameT { code_location, .. }) => {
                INameI::LambdaCitizenTemplate(self.interner.intern_lambda_citizen_template_name_si(crate::instantiating::ast::names::LambdaCitizenTemplateNameI { _marker: std::marker::PhantomData, code_location: *code_location }))
            }
            INameT::AnonymousSubstructTemplate(_) => panic!("Unimplemented: translate_name_substituting AnonymousSubstructTemplate"),
            INameT::LambdaCitizen(_) => panic!("Unimplemented: translate_name_substituting LambdaCitizen"),
            INameT::InterfaceTemplate(_) => panic!("Unimplemented: translate_name_substituting InterfaceTemplate"),
            INameT::Function(_) | INameT::ForwarderFunction(_) | INameT::ExternFunction(_) | INameT::FunctionBound(_) | INameT::LambdaCallFunction(_) | INameT::AnonymousSubstructConstructor(_) | INameT::PredictedFunction(_) => {
                let f: IFunctionNameT<'s, 't> = (*name).try_into().unwrap();
                INameI::from(self.translate_function_name(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, _perspective_region_t, &f))
            }
            _ => panic!("Unimplemented: translate_name_substituting other"),
        }
    }
}
/*
  def translateName(
    denizenName: IdT[IInstantiationNameT],
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    perspectiveRegionT: RegionT,
    name: INameT):
  INameI[sI] = {
    name match {
      case v : IVarNameT => translateVarName(v)
      case KindPlaceholderTemplateNameT(index, _) => vwat()
      case KindPlaceholderNameT(inner) => vwat()
      case StructNameT(StructTemplateNameT(humanName), templateArgs) => {
        StructNameI(
          StructTemplateNameI(humanName),
          templateArgs.map(translateTemplata(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, _)))
      }
      case ForwarderFunctionTemplateNameT(inner, index) => {
        ForwarderFunctionTemplateNameI(
          translateFunctionTemplateName(inner),
          index)
      }
      case AnonymousSubstructConstructorTemplateNameT(substructTemplateName) => {
        AnonymousSubstructConstructorTemplateNameI(
          translateName(denizenName, denizenBoundToDenizenCallerSuppliedThing, substitutions, perspectiveRegionT, substructTemplateName) match {
            case x : ICitizenTemplateNameI[sI] => x
            case other => vwat(other)
          })
      }
      case FunctionTemplateNameT(humanName, codeLoc) => FunctionTemplateNameI(humanName, codeLoc)
      case StructTemplateNameT(humanName) => StructTemplateNameI(humanName)
      case LambdaCitizenTemplateNameT(codeLoc) => LambdaCitizenTemplateNameI(codeLoc)
      case AnonymousSubstructTemplateNameT(interface) => {
        AnonymousSubstructTemplateNameI(
          translateInterfaceTemplateName(interface))
      }
      case LambdaCitizenNameT(LambdaCitizenTemplateNameT(codeLocation)) => {
        LambdaCitizenNameI(LambdaCitizenTemplateNameI(codeLocation))
      }
      case InterfaceTemplateNameT(humanNamee) => InterfaceTemplateNameI(humanNamee)
      //      case FreeTemplateNameT(codeLoc) => name
      case f : IFunctionNameT => translateFunctionName(denizenName, denizenBoundToDenizenCallerSuppliedThing,substitutions, perspectiveRegionT, f)
      case other => vimpl(other)
    }
  }
*/
// mig: fn translate_collapsed_impl_definition
impl<'s, 'ctx, 't, 'i> InstantiatorI<'s, 'ctx, 't, 'i> where 's: 't, 's: 'i {
    pub fn translate_collapsed_impl_definition(&self, _monouts: &mut InstantiatedOutputsI<'s, 't, 'i>, _denizen_name: &IdT<'s, 't>, _instantiation_bounds_for_unsubstituted_impl: InstantiationBoundArgumentsI<'s, 'i>, _denizen_bound_to_denizen_caller_supplied_thing: &DenizenBoundToDenizenCallerBoundArgI<'s, 't, 'i>, _substitutions: &IndexMap<IdT<'s, 't>, ITemplataI<'s, 'i, sI>>, _impl_id_t: &IdT<'s, 't>, _impl_id_s: &IdI<'s, 'i, sI>, _impl_id_c: &IdI<'s, 'i, cI>, _impl_definition: &EdgeT<'s, 't>) {
        // Scala's `if (opts.sanityCheck) { vassert(Collector.all(implIdS, { case KindPlaceholderNameT(_) => }).isEmpty) }`
        // sanity check is omitted: architect-approved parity gap, vacuous on sI-typed I-side AST.
        let perspective_region_t = RegionT { region: IRegionT::Default };
        let sub_citizen_bound_args = self.translate_bound_args_for_callee(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, &perspective_region_t, &self.hinputs.get_instantiation_bound_args(_impl_definition.sub_citizen.id()));
        let sub_citizen_s = self.translate_citizen(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, &perspective_region_t, &_impl_definition.sub_citizen, &sub_citizen_bound_args);
        let sub_citizen_c = crate::instantiating::region_collapser_individual::collapse_citizen(self.interner, &sub_citizen_s);
        let super_interface_bound_args = self.translate_bound_args_for_callee(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, &perspective_region_t, &self.hinputs.get_instantiation_bound_args(_impl_definition.super_interface));
        let super_interface_s = self.translate_interface_id(_monouts, _denizen_name, _denizen_bound_to_denizen_caller_supplied_thing, _substitutions, &perspective_region_t, &_impl_definition.super_interface, &super_interface_bound_args);
        let super_interface_c = crate::instantiating::region_collapser_individual::collapse_interface_id(self.interner, &super_interface_s);

        let mutability = *_monouts.interface_to_mutability.get(&super_interface_c).expect("translate_collapsed_impl_definition: superInterfaceC mutability missing");
        if _monouts.impl_to_mutability.contains_key(_impl_id_c) {
            return;
        }
        _monouts.impl_to_mutability.insert(*_impl_id_c, mutability);

        // We assemble the EdgeI at the very end of the instantiating stage.

        _monouts.impls.insert(*_impl_id_c, (sub_citizen_c, super_interface_c, _denizen_bound_to_denizen_caller_supplied_thing.clone(), _instantiation_bounds_for_unsubstituted_impl));

        _monouts.interface_to_impl_to_abstract_prototype_to_override.get_mut(&super_interface_c).expect("vassertSome: interface_to_impl_to_abstract_prototype_to_override")
            .insert(*_impl_id_c, IndexMap::new());
        _monouts.interface_to_impls.get_mut(&super_interface_c).expect("vassertSome: interface_to_impls").push((*_impl_id_t, *_impl_id_c));
    }
}
/*
Guardian: temp-disable: SPDMX — Architect-approved parity gap (same precedent as translate_collapsed_struct_definition in this file): Scala's `if (opts.sanityCheck) { vassert(Collector.all(implIdS, { case KindPlaceholderNameT(_) => }).isEmpty) }` is vacuously empty on the I-side AST because the value is statically typed sI and can't structurally hold a typing-pass KindPlaceholder name. — /Volumes/V/Vale/FrontendRust/guardian-logs/request-2786-1780145591183/hook-2786/translate_collapsed_impl_definition--5682.0.ScalaParityDuringMigration-SPDMX.ScalaParityDuringMigration-SPDMX.verdict.md
  def translateCollapsedImplDefinition(
    denizenName: IdT[IInstantiationNameT],
    implInstantiationBoundArgs: InstantiationBoundArgumentsI,
    denizenBoundToDenizenCallerSuppliedThing: DenizenBoundToDenizenCallerBoundArgS,
    substitutions: Map[IdT[IPlaceholderNameT], ITemplataI[sI]],
    implIdT: IdT[IImplNameT],
    implIdS: IdI[sI, IImplNameI[sI]],
    implIdC: IdI[cI, IImplNameI[cI]],
    implDefinition: EdgeT):
  Unit = {
    val EdgeT(_, subCitizen, superInterface, instantiationBoundParams, abstractFuncToOverrideFunc) = implDefinition

    if (opts.sanityCheck) {
      vassert(Collector.all(implIdS, { case KindPlaceholderNameT(_) => }).isEmpty)
    }

    val perspectiveRegionT = RegionT(DefaultRegionT)
    // structDefT.instantiatedCitizen.id.localName.templateArgs.last match {
    //   case PlaceholderTemplataT(IdT(packageCoord, initSteps, r @ RegionPlaceholderNameT(_, _, _, _)), RegionTemplataType()) => {
    //     IdT(packageCoord, initSteps, r)
    //   }
    //   case _ => vwat()
    // }

    val subCitizenS =
      translateCitizen(
        denizenName, denizenBoundToDenizenCallerSuppliedThing,
        substitutions,
        RegionT(DefaultRegionT),
        implDefinition.subCitizen,
        translateBoundArgsForCallee(denizenName, denizenBoundToDenizenCallerSuppliedThing,
          substitutions,
          RegionT(DefaultRegionT),
          hinputs.getInstantiationBoundArgs(implDefinition.subCitizen.id)))
    val subCitizenC =
      RegionCollapserIndividual.collapseCitizen(subCitizenS)
    val superInterfaceS =
      translateInterfaceId(
        denizenName, denizenBoundToDenizenCallerSuppliedThing,
        substitutions,
        RegionT(DefaultRegionT),
        implDefinition.superInterface,
        translateBoundArgsForCallee(denizenName, denizenBoundToDenizenCallerSuppliedThing,
          substitutions,
          RegionT(DefaultRegionT),
          hinputs.getInstantiationBoundArgs(implDefinition.superInterface)))
    val superInterfaceC =
      RegionCollapserIndividual.collapseInterfaceId(superInterfaceS)

    val mutability = vassertSome(monouts.interfaceToMutability.get(superInterfaceC))
    if (monouts.implToMutability.contains(implIdC)) {
      return
    }
    monouts.implToMutability.put(implIdC, mutability)


    // We assemble the EdgeI at the very end of the instantiating stage.

    monouts.impls.put(implIdC, (subCitizenC, superInterfaceC, denizenBoundToDenizenCallerSuppliedThing, implInstantiationBoundArgs))

    vassertSome(monouts.interfaceToImplToAbstractPrototypeToOverride.get(superInterfaceC))
      .put(implIdC, mutable.HashMap())
    vassertSome(monouts.interfaceToImpls.get(superInterfaceC)).add((implIdT, implIdC))
  }
}
*/