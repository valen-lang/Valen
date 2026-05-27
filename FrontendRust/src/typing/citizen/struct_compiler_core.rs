use crate::higher_typing::ast::{FunctionA, InterfaceA, StructA};
use crate::postparsing::ast::{ExternS, ICitizenAttributeS, IStructMemberS, LocationInDenizen};
use crate::postparsing::names::IFunctionDeclarationNameS;
use crate::typing::ast::ast::{ExternT, ICitizenAttributeT};
use crate::typing::ast::citizens::{IStructMemberT, IMemberTypeT, InterfaceDefinitionT, NormalStructMemberT, StructDefinitionT};
use crate::typing::names::names::{CodeVarNameT, IVarNameT};
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::env::environment::{CitizenEnvironmentT, IInDenizenEnvironmentT};
use crate::typing::env::function_environment_t::NodeEnvironmentT;
use crate::typing::templata::templata::FunctionTemplataT;
use crate::typing::hinputs_t::InstantiationBoundArgumentsT;
use crate::typing::types::types::{MutabilityT, OwnershipT, StructTT, VariabilityT};
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::utils::range::RangeS;
use crate::typing::names::names::{IInstantiationNameT, IStructTemplateNameT, IdValT, INameT};
use crate::typing::env::environment::{TemplatasStoreBuilder, IEnvironmentT, ILookupContext};
use crate::typing::types::types::StructTTValT;
use crate::typing::compiler_outputs::DeferredActionT;
use crate::typing::env::i_env_entry::IEnvEntryT;
use crate::typing::templata::templata::{ITemplataT, expect_mutability};
use crate::typing::hinputs_t::make;
use crate::postparsing::names::{IImpreciseNameValS, RuneNameValS};
use crate::parsing::ast::IMacroInclusionP;
use std::collections::HashSet;
use crate::typing::names::names::IInterfaceTemplateNameT;
use crate::typing::types::types::InterfaceTTValT;
use std::marker::PhantomData;
use crate::postparsing::names::RuneNameS;
use crate::typing::templata::conversions::evaluate_variability;
use crate::typing::names::names::*;
use crate::typing::templata::templata::*;
use crate::typing::types::types::*;
use crate::typing::ast::citizens::ReferenceMemberTypeT;
use crate::postparsing::names::INameValS;
use crate::postparsing::names::IFunctionDeclarationNameValS;
use crate::postparsing::names::FunctionNameS;
use crate::postparsing::names::INameS;
use crate::typing::ast::citizens::AddressMemberTypeT;
use crate::postparsing::ast::MacroCallS;
use crate::typing::templata::templata::MutabilityTemplataT;
use crate::postparsing::names::IStructDeclarationNameS;
use crate::typing::ast::ast::PrototypeT;

/*
package dev.vale.typing.citizen

import dev.vale.highertyping.{FunctionA, InterfaceA, StructA}
import dev.vale._
import dev.vale.parsing.ast.{CallMacroP, DontCallMacroP}
import dev.vale.postparsing.rules.RuneUsage
import dev.vale.postparsing._
import dev.vale.typing.expression.CallCompiler
import dev.vale.highertyping._
import dev.vale.typing.types._
import dev.vale.typing.templata._
import dev.vale.postparsing._
import dev.vale.typing.OverloadResolver.FindFunctionFailure
import dev.vale.typing.ast.{ExternT, ICitizenAttributeT, SealedT}
import dev.vale.typing.{CompileErrorExceptionT, CompilerOutputs, ImmStructCantHaveVaryingMember, RangedInternalErrorT, TypingPassOptions, env}
import dev.vale.typing.{ast, _}
import dev.vale.typing.env._
import dev.vale.typing.function.FunctionCompiler
import dev.vale.parsing.ast.DontCallMacroP
import dev.vale.typing.env.{CitizenEnvironmentT, FunctionEnvEntry, IInDenizenEnvironmentT, TemplataEnvEntry, TemplataLookupContext, TemplatasStore}
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.typing.ast._

import scala.collection.immutable.List

class StructCompilerCore(
  opts: TypingPassOptions,
  interner: Interner,
  keywords: Keywords,
  nameTranslator: NameTranslator,
  delegate: IStructCompilerDelegate) {
*/
/*
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn compile_struct_core(
        &self,
        outer_env: IInDenizenEnvironmentT<'s, 't>,
        struct_runes_env: &'t CitizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        struct_a: &'s StructA<'s>,
    ) -> Result<(), ICompileErrorT<'s, 't>> {

        let template_args = IInstantiationNameT::try_from(struct_runes_env.id.local_name)
            .unwrap()
            .template_args();
        let template_id_t = struct_runes_env.template_id;
        let template_name_t = IStructTemplateNameT::try_from(template_id_t.local_name).unwrap();
        let placeholdered_name_t = template_name_t.make_struct_name(self.typing_interner, template_args);
        // Rust adaptation (SPDMX-B): Scala uses .copy(localName=...) — build new IdT via intern_id.
        let template_id_steps = template_id_t.init_steps.to_vec();
        let placeholdered_id_t = *self.typing_interner.intern_id(IdValT {
            package_coord: template_id_t.package_coord,
            init_steps: &template_id_steps,
            local_name: placeholdered_name_t,
        });

        // Usually when we make a StructTT we put the instantiation bounds into the coutputs,
        // but this isn't really an instantiation, so we don't here.
        let placeholdered_struct_tt = *self.typing_interner.intern_struct_tt(StructTTValT { id: placeholdered_id_t });

        let attributes_without_export_or_macros: Vec<ICitizenAttributeS<'s>> =
            struct_a.attributes.iter().filter(|attr| {
                match attr {
                    ICitizenAttributeS::Export(_) => false,
                    ICitizenAttributeS::MacroCall(_) => false,
                    _ => true,
                }
            }).copied().collect();

        let rune_name_s = self.scout_arena.intern_imprecise_name(
            IImpreciseNameValS::RuneName(RuneNameValS { rune: struct_a.mutability_rune.rune }));
        let struct_runes_env_as_iindenizen = IInDenizenEnvironmentT::Citizen(struct_runes_env);
        let mutability_results = struct_runes_env_as_iindenizen
            .lookup_nearest_with_imprecise_name(rune_name_s, {
                let mut s = HashSet::new();
                s.insert(ILookupContext::TemplataLookupContext);
                s
            }, self.typing_interner);
        let mutability = match mutability_results {
            Some(m) => expect_mutability(m),
            None => panic!("vwat: no mutability rune found"),
        };

        let default_called_macros: Vec<MacroCallS<'s>> = vec![
            MacroCallS {
                range: struct_a.range,
                include: IMacroInclusionP::CallMacro,
                macro_name: self.keywords.derive_struct_drop,
            },
        ];
        let mut macros_to_call = default_called_macros;
        for attr in struct_a.attributes.iter() {
            match attr {
                ICitizenAttributeS::MacroCall(mc) if mc.include == IMacroInclusionP::CallMacro => {
                    if macros_to_call.iter().any(|m| m.macro_name == mc.macro_name) {
                        panic!("Calling macro twice: {:?}", mc.macro_name);
                    }
                    macros_to_call.push(*mc);
                }
                ICitizenAttributeS::MacroCall(mc) if mc.include == IMacroInclusionP::DontCallMacro => {
                    macros_to_call.retain(|m| m.macro_name != mc.macro_name);
                }
                _ => {}
            }
        }

        let inner_templatas = TemplatasStoreBuilder::new(
            self.typing_interner.alloc(placeholdered_id_t)
        ).build_in(self.typing_interner);
        let struct_inner_env = self.typing_interner.alloc(CitizenEnvironmentT {
            global_env: struct_runes_env.global_env,
            parent_env: IEnvironmentT::Citizen(struct_runes_env),
            template_id: template_id_t,
            id: placeholdered_id_t,
            templatas: inner_templatas,
        });
        let struct_inner_env_ref: IInDenizenEnvironmentT<'s, 't> =
            IInDenizenEnvironmentT::Citizen(struct_inner_env);

        let members_vec = self.make_struct_members(struct_inner_env_ref, coutputs, struct_a.members);

        if mutability == ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }) {
            for (index, member) in members_vec.iter().enumerate() {
                let member_s = &struct_a.members[index];
                let member_range = member_s.range();
                let member_name = match member_s {
                    IStructMemberS::NormalStructMember(m) => m.name.0,
                    IStructMemberS::VariadicStructMember(_) => "(unnamed)",
                };
                let member_range_with_parent: Vec<RangeS<'s>> =
                    std::iter::once(member_range).chain(parent_ranges.iter().copied()).collect();
                let member_range_t = self.typing_interner.alloc_slice_copy(&member_range_with_parent);
                let struct_name_s = match &struct_a.name {
                    IStructDeclarationNameS::TopLevelStructDeclarationName(n) =>
                        INameS::TopLevelStructDeclaration(n),
                    other => panic!("implement: struct_name_s for non-TopLevelStructDeclarationName: {:?}", other),
                };
                match member {
                    IStructMemberT::Variadic(_) => {
                        panic!("implement: immutable variadic struct member check");
                    }
                    IStructMemberT::Normal(NormalStructMemberT { variability, tyype, .. }) => {
                        if *variability == VariabilityT::Varying {
                            return Err(ICompileErrorT::ImmStructCantHaveVaryingMember {
                                range: member_range_t,
                                struct_name: struct_name_s,
                                member_name,
                            });
                        }
                        if tyype.reference().ownership != OwnershipT::Share {
                            return Err(ICompileErrorT::ImmStructCantHaveMutableMember {
                                range: member_range_t,
                                struct_name: struct_name_s,
                                member_name,
                            });
                        }
                    }
                }
            }
        }

        for (name, entry) in outer_env.templatas().name_to_entry.iter() {
            match entry {
                IEnvEntryT::Function(function_a) => {
                    let deferred_name = outer_env.id().add_step(self.typing_interner, *name);
                    coutputs.defer_evaluating_function(DeferredActionT::EvaluateFunction {
                        name: deferred_name,
                        calling_env: outer_env,
                        origin: function_a,
                        template_args: &[],
                    });
                }
                _ => panic!("vcurious: unexpected entry in outer_env.templatas"),
            }
        }

        let rune_to_function_bound = self.assemble_rune_to_function_bound(struct_runes_env.templatas);
        let rune_to_impl_bound = self.assemble_rune_to_impl_bound(struct_runes_env.templatas);

        let attributes_t = self.translate_citizen_attributes(&attributes_without_export_or_macros);
        let members_slice = self.typing_interner.alloc_slice_from_vec(members_vec);
        let attributes_slice = self.typing_interner.alloc_slice_from_vec(attributes_t);
        let instantiation_bound_params = make(
            self.typing_interner,
            rune_to_function_bound.into_iter().map(|(k, v)| (k, *v)).collect(),
            vec![],
            rune_to_impl_bound.into_iter().collect(),
        );

        let struct_def_t = self.typing_interner.alloc(StructDefinitionT {
            template_name: template_id_t,
            instantiated_citizen: placeholdered_struct_tt,
            attributes: attributes_slice,
            weakable: struct_a.weakable,
            mutability,
            members: members_slice,
            is_closure: false,
            instantiation_bound_params,
        });

        coutputs.add_struct(struct_def_t);
        Ok(())
    }
/*
  def compileStruct(
    outerEnv: IInDenizenEnvironmentT,
    structRunesEnv: CitizenEnvironmentT[IStructNameT, IStructTemplateNameT],
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    structA: StructA):
  Unit = {
    val templateArgs = structRunesEnv.id.localName.templateArgs
    val templateIdT = structRunesEnv.templateId
    val templateNameT = templateIdT.localName
    val placeholderedNameT = templateNameT.makeStructName(interner, templateArgs)
    val placeholderedIdT = templateIdT.copy(localName = placeholderedNameT)

    // Usually when we make a StructTT we put the instantiation bounds into the coutputs,
    // but this isn't really an instantiation, so we don't here.
    val placeholderedStructTT = interner.intern(StructTT(placeholderedIdT))

    val attributesWithoutExportOrMacros =
      structA.attributes.filter({
        case ExportS(_) => false
        case MacroCallS(range, dontCall, macroName) => false
        case _ => true
      })

    val mutability =
      structRunesEnv.lookupNearestWithImpreciseName(
        interner.intern(RuneNameS(structA.mutabilityRune.rune)),
        Set(TemplataLookupContext)).toList match {
        case List(m) => ITemplataT.expectMutability(m)
        case _ => vwat()
      }

    val defaultCalledMacros =
      Vector(
        MacroCallS(structA.range, CallMacroP, keywords.DeriveStructDrop))//,
//        MacroCallS(structA.range, CallMacroP, keywords.DeriveStructFree),
//        MacroCallS(structA.range, CallMacroP, keywords.DeriveImplFree))
    val macrosToCall =
      structA.attributes.foldLeft(defaultCalledMacros)({
        case (macrosToCall, mc @ MacroCallS(range, CallMacroP, macroName)) => {
          if (macrosToCall.exists(_.macroName == macroName)) {
            throw CompileErrorExceptionT(RangedInternalErrorT(range :: parentRanges, "Calling macro twice: " + macroName))
          }
          macrosToCall :+ mc
        }
        case (macrosToCall, MacroCallS(_, DontCallMacroP, macroName)) => macrosToCall.filter(_.macroName != macroName)
        case (macrosToCall, _) => macrosToCall
      })

    val structInnerEnv =
      CitizenEnvironmentT(
        structRunesEnv.globalEnv,
        structRunesEnv,
        templateIdT,
        placeholderedIdT,
        TemplatasStore(placeholderedIdT, Map(), Map()))

    val members = makeStructMembers(structInnerEnv, coutputs, structA.members)

    if (mutability == MutabilityTemplataT(ImmutableT)) {
      members.zipWithIndex.foreach({
        case (VariadicStructMemberT(name, tyype), index) => {
          vimpl() // Dont have imm variadics yet
        }
        case (NormalStructMemberT(name, variability, tyype), index) => {
          if (variability == VaryingT) {
            throw CompileErrorExceptionT(
              ImmStructCantHaveVaryingMember(
                structA.members(index).range :: parentRanges,
                structA.name,
                structA.members(index) match {
                  case NormalStructMemberS(range, name, variability, typeRune) => name.str
                  case VariadicStructMemberS(range, variability, typeRune) => "(unnamed)"
                }))
          }

          if (tyype.reference.ownership != ShareT) {
            throw CompileErrorExceptionT(
              ImmStructCantHaveMutableMember(
                structA.members(index).range :: parentRanges,
                structA.name,
                structA.members(index) match {
                  case NormalStructMemberS(range, name, variability, typeRune) => name.str
                  case VariadicStructMemberS(range, variability, typeRune) => "(unnamed)"
                }))
          }
        }
      })
    }

    outerEnv.templatas.entriesByNameT.foreach({
      case (name, FunctionEnvEntry(functionA)) => {
        // These have to be delegated, otherwise some compiling functions won't have what we expect.
        // For example, MyShip.drop will expect to see the members of MyEngine, but we haven't compiled
        // MyEngine yet.
        // We need to defer all these functions until after the structs and interfaces are done.
        coutputs.deferEvaluatingFunction(
          DeferredEvaluatingFunction(
            outerEnv.id.addStep(name),
            (coutputs) => {
              delegate.evaluateGenericFunctionFromNonCallForHeader(
                coutputs, parentRanges, callLocation, FunctionTemplataT(outerEnv, functionA))
            }))
      }
      case _ => vcurious()
    })

    val runeToFunctionBound = TemplataCompiler.assembleRuneToFunctionBound(structRunesEnv.templatas)
    val runeToImplBound = TemplataCompiler.assembleRuneToImplBound(structRunesEnv.templatas)

    val structDefT =
      StructDefinitionT(
        templateIdT,
        placeholderedStructTT,
        translateCitizenAttributes(attributesWithoutExportOrMacros),
        structA.weakable,
        mutability,
        members,
        false,
        InstantiationBoundArgumentsT.make[FunctionBoundNameT, ImplBoundNameT](
          runeToFunctionBound,
          Map(), // Structs don't have reachable bounds
          runeToImplBound))

    coutputs.addStruct(structDefT);
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn translate_citizen_attributes(
        &self,
        attrs: &[ICitizenAttributeS<'s>],
    ) -> Vec<ICitizenAttributeT<'s>> {
        attrs.iter().map(|attr| {
            match attr {
                ICitizenAttributeS::Sealed(_) => ICitizenAttributeT::Sealed,
                ICitizenAttributeS::Extern(ExternS { package_coord: p }) => ICitizenAttributeT::Extern(ExternT { package_coord: **p }),
                ICitizenAttributeS::MacroCall(_) => panic!("vwat: MacroCallS should have been processed"),
                x => panic!("vimpl: {:?}", x),
            }
        }).collect()
    }
/*
  def translateCitizenAttributes(attrs: Vector[ICitizenAttributeS]): Vector[ICitizenAttributeT] = {
    attrs.map({
      case SealedS => SealedT
      case ExternS(p) => ExternT(p)
      case MacroCallS(_, _, _) => vwat() // Should have been processed
      case x => vimpl(x.toString)
    })
  }


*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn compile_interface_core(
        &self,
        outer_env: IInDenizenEnvironmentT<'s, 't>,
        interface_runes_env: &'t CitizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        interface_a: &'s InterfaceA<'s>,
    ) -> Result<&'t InterfaceDefinitionT<'s, 't>, ICompileErrorT<'s, 't>> {

        let template_args = IInstantiationNameT::try_from(interface_runes_env.id.local_name)
            .unwrap()
            .template_args();
        let template_id_t = interface_runes_env.template_id;
        let template_name_t = IInterfaceTemplateNameT::try_from(template_id_t.local_name).unwrap();
        let placeholdered_name_t = template_name_t.make_interface_name(self.typing_interner, template_args);
        // Rust adaptation (SPDMX-B): Scala uses .copy(localName=...) — build new IdT via intern_id.
        let template_id_steps = template_id_t.init_steps.to_vec();
        let placeholdered_id_t = *self.typing_interner.intern_id(IdValT {
            package_coord: template_id_t.package_coord,
            init_steps: &template_id_steps,
            local_name: placeholdered_name_t,
        });

        // Usually when we make an InterfaceTT we put the instantiation bounds into the coutputs,
        // but this isn't really an instantiation, so we don't here.
        let placeholdered_interface_tt = *self.typing_interner.intern_interface_tt(InterfaceTTValT { id: placeholdered_id_t });

        let attributes_without_export_or_macros: Vec<ICitizenAttributeS<'s>> =
            interface_a.attributes.iter().filter(|attr| {
                match attr {
                    ICitizenAttributeS::Export(_) => false,
                    ICitizenAttributeS::MacroCall(_) => false,
                    _ => true,
                }
            }).copied().collect();
        let _maybe_export = interface_a.attributes.iter().find(|attr| matches!(attr, ICitizenAttributeS::Export(_)));

        let rune_name_s = self.scout_arena.intern_imprecise_name(
            IImpreciseNameValS::RuneName(RuneNameValS { rune: interface_a.mutability_rune.rune }));
        let interface_runes_env_as_iindenizen = IInDenizenEnvironmentT::Citizen(interface_runes_env);
        let mutability_results = interface_runes_env_as_iindenizen
            .lookup_nearest_with_imprecise_name(rune_name_s, {
                let mut s = HashSet::new();
                s.insert(ILookupContext::TemplataLookupContext);
                s
            }, self.typing_interner);
        let mutability = match mutability_results {
            Some(m) => expect_mutability(m),
            None => panic!("vwat: no mutability rune found for interface"),
        };

        let mut internal_methods: Vec<(PrototypeT<'s, 't>, usize)> = Vec::new();
        for (_name, entry) in outer_env.templatas().name_to_entry.iter() {
            if let IEnvEntryT::Function(function_a) = entry {
                let outer_env_ienv = IEnvironmentT::from(outer_env);
                let header = self.evaluate_generic_function_from_non_call_for_header(
                    coutputs, parent_ranges, call_location,
                    FunctionTemplataT { outer_env: outer_env_ienv, function: function_a })?;
                let virtual_index = header.get_virtual_index()
                    .expect("vwat: interface internal method must have a virtual index");
                internal_methods.push((header.to_prototype(), virtual_index));
            }
        }

        let rune_to_function_bound = self.assemble_rune_to_function_bound(interface_runes_env.templatas);
        let rune_to_impl_bound = self.assemble_rune_to_impl_bound(interface_runes_env.templatas);

        let attributes_t = self.translate_citizen_attributes(&attributes_without_export_or_macros);
        let attributes_slice = self.typing_interner.alloc_slice_from_vec(attributes_t);
        let internal_methods_slice = self.typing_interner.alloc_slice_from_vec(internal_methods);
        let instantiation_bound_params = make(
            self.typing_interner,
            rune_to_function_bound.into_iter().map(|(k, v)| (k, *v)).collect(),
            vec![],
            rune_to_impl_bound.into_iter().collect(),
        );

        let interface_def_t = self.typing_interner.alloc(InterfaceDefinitionT {
            template_name: template_id_t,
            instantiated_interface: placeholdered_interface_tt,
            ref_: placeholdered_interface_tt,
            attributes: attributes_slice,
            weakable: interface_a.weakable,
            mutability,
            instantiation_bound_params,
            internal_methods: internal_methods_slice,
        });

        coutputs.add_interface(interface_def_t);

        Ok(interface_def_t)
    }
/*
  // Takes a IEnvironment because we might be inside a:
  // struct<T> Thing<T> {
  //   t: T;
  // }
  // which means we need some way to know what T is.
  def compileInterface(
    outerEnv: IInDenizenEnvironmentT,
    interfaceRunesEnv: CitizenEnvironmentT[IInterfaceNameT, IInterfaceTemplateNameT],
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    interfaceA: InterfaceA):
  (InterfaceDefinitionT) = {
    val templateArgs = interfaceRunesEnv.id.localName.templateArgs
    val templateIdT = interfaceRunesEnv.templateId
    val templateNameT = templateIdT.localName
    val placeholderedNameT = templateNameT.makeInterfaceName(interner, templateArgs)
    val placeholderedIdT = templateIdT.copy(localName = placeholderedNameT)

    // Usually when we make a StructTT we put the instantiation bounds into the coutputs,
    // but this isn't really an instantiation, so we don't here.
    val placeholderedInterfaceTT = interner.intern(InterfaceTT(placeholderedIdT))

    val attributesWithoutExportOrMacros =
      interfaceA.attributes.filter({
        case ExportS(_) => false
        case MacroCallS(range, dontCall, macroName) => false
        case _ => true
      })
    val maybeExport =
      interfaceA.attributes.collectFirst { case e@ExportS(_) => e }


    val mutability =
      ITemplataT.expectMutability(
        vassertSome(
          interfaceRunesEnv.lookupNearestWithImpreciseName(
            interner.intern(RuneNameS(interfaceA.mutabilityRune.rune)),
            Set(TemplataLookupContext))))

    val internalMethods =
      outerEnv.templatas.entriesByNameT.collect({
        case (name, FunctionEnvEntry(functionA)) => {
          val header =
            delegate.evaluateGenericFunctionFromNonCallForHeader(
              coutputs, parentRanges, callLocation, FunctionTemplataT(outerEnv, functionA))
          header.toPrototype -> vassertSome(header.getVirtualIndex)
        }
      }).toVector

    val runeToFunctionBound = TemplataCompiler.assembleRuneToFunctionBound(interfaceRunesEnv.templatas)
    val runeToImplBound = TemplataCompiler.assembleRuneToImplBound(interfaceRunesEnv.templatas)

    val interfaceDef2 =
      InterfaceDefinitionT(
        templateIdT,
        placeholderedInterfaceTT,
        interner.intern(placeholderedInterfaceTT),
        translateCitizenAttributes(attributesWithoutExportOrMacros),
        interfaceA.weakable,
        mutability,
        InstantiationBoundArgumentsT.make[FunctionBoundNameT, ImplBoundNameT](
          runeToFunctionBound,
          Map(), // Interfaces don't have reachable bounds
          runeToImplBound),
        internalMethods)
    coutputs.addInterface(interfaceDef2)

    (interfaceDef2)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn make_struct_members(
        &self,
        env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        members: &[IStructMemberS<'s>],
    ) -> Vec<IStructMemberT<'s, 't>> {
        members.iter().map(|m| self.make_struct_member(env, coutputs, *m)).collect()
    }
/*
  private def makeStructMembers(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    members: Vector[IStructMemberS]):
  Vector[IStructMemberT] = {
    members.map(makeStructMember(env, coutputs, _))
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn make_struct_member(
        &self,
        env: IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        member: IStructMemberS<'s>,
    ) -> IStructMemberT<'s, 't> {
        let type_rune_s = (*member.type_rune()).rune;
        let type_templata = match env.lookup_nearest_with_imprecise_name(
            self.scout_arena.intern_imprecise_name(
                IImpreciseNameValS::RuneName(RuneNameValS { rune: type_rune_s })
            ),
            {
                let mut s = HashSet::new();
                s.insert(ILookupContext::TemplataLookupContext);
                s
            },
            self.typing_interner,
        ) {
            Some(t) => t,
            None => panic!("Unimplemented: make_struct_member type not found"),
        };
        let variability_t = evaluate_variability(member.variability());
        match member {
            IStructMemberS::NormalStructMember(n) => {
                let coord = match type_templata {
                    ITemplataT::Coord(c) => c.coord,
                    _ => panic!("Unimplemented: make_struct_member non-coord type for NormalStructMemberS"),
                };
                IStructMemberT::Normal(NormalStructMemberT {
                    name: IVarNameT::CodeVar(self.typing_interner.intern_code_var_name(CodeVarNameT { name: n.name, _phantom: PhantomData })),
                    variability: variability_t,
                    tyype: IMemberTypeT::Reference(ReferenceMemberTypeT { reference: coord }),
                })
            }
            IStructMemberS::VariadicStructMember(_) => panic!("Unimplemented: make_struct_member VariadicStructMemberS"),
        }
    }
/*
  private def makeStructMember(
    env: IInDenizenEnvironmentT,
    coutputs: CompilerOutputs,
    member: IStructMemberS):
  IStructMemberT = {
    val typeTemplata =
      vassertOne(
        env.lookupNearestWithImpreciseName(
          interner.intern(RuneNameS(member.typeRune.rune)), Set(TemplataLookupContext)))
    val variabilityT = Conversions.evaluateVariability(member.variability)
    member match {
      case NormalStructMemberS(_, name, _, _) => {
        val CoordTemplataT(coord) = typeTemplata
        NormalStructMemberT(
          interner.intern(CodeVarNameT(name)),
          variabilityT,
          ReferenceMemberTypeT(coord))
      }
      case VariadicStructMemberS(_, variability, coordListRune) => {
        val placeholderTemplata =
          env.lookupNearestWithName(interner.intern(RuneNameT(coordListRune.rune)), Set(TemplataLookupContext)) match {
            case Some(PlaceholderTemplataT(idT, PackTemplataType(CoordTemplataType()))) => {
              PlaceholderTemplataT(idT, PackTemplataType(CoordTemplataType()))
            }
            case _ => vwat()
          }
        VariadicStructMemberT(
          interner.intern(CodeVarNameT(keywords.emptyString)),
          placeholderTemplata)
      }
    }
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn make_closure_understruct_core(
        &self,
        containing_function_env: &'t NodeEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        name: IFunctionDeclarationNameS<'s>,
        function_a: &'s FunctionA<'s>,
        members: &[&'t NormalStructMemberT<'s, 't>],
    ) -> Result<(StructTT<'s, 't>, MutabilityT, FunctionTemplataT<'s, 't>), ICompileErrorT<'s, 't>> {

        let is_mutable = members.iter().any(|m| {
            if m.variability == VariabilityT::Varying {
                true
            } else {
                match &m.tyype {
                    IMemberTypeT::Address(_) => true,
                    IMemberTypeT::Reference(ReferenceMemberTypeT { reference }) => {
                        match reference.ownership {
                            OwnershipT::Own | OwnershipT::Borrow | OwnershipT::Weak => true,
                            OwnershipT::Share => false,
                        }
                    }
                }
            }
        });
        let mutability = if is_mutable { MutabilityT::Mutable } else { MutabilityT::Immutable };

        let understruct_template_name_t =
            self.typing_interner.intern_lambda_citizen_template_name(LambdaCitizenTemplateNameT {
                code_location: self.translate_code_location(function_a.range.begin),
                _phantom: std::marker::PhantomData,
            });
        let understruct_templated_id =
            containing_function_env.id().add_step(
                self.typing_interner,
                INameT::LambdaCitizenTemplate(understruct_template_name_t));

        let understruct_instantiated_name_t =
            IStructTemplateNameT::LambdaCitizenTemplate(understruct_template_name_t)
                .make_struct_name(self.typing_interner, &[]);
        let understruct_instantiated_id =
            containing_function_env.id().add_step(
                self.typing_interner,
                understruct_instantiated_name_t);

        // Lambdas have no bounds, so we just supply empty maps
        coutputs.add_instantiation_bounds(
            self.opts.global_options.sanity_check,
            self.typing_interner,
            *understruct_templated_id,
            *understruct_instantiated_id,
            self.typing_interner.alloc(InstantiationBoundArgumentsT {
                rune_to_bound_prototype: self.typing_interner.alloc_index_map(),
                rune_to_citizen_rune_to_reachable_prototype: self.typing_interner.alloc_index_map(),
                rune_to_bound_impl: self.typing_interner.alloc_index_map(),
            }));
        let understruct_struct_tt = self.typing_interner.intern_struct_tt(StructTTValT {
            id: *understruct_instantiated_id,
        });

        let drop_func_name_t = INameT::FunctionTemplate(
            self.typing_interner.intern_function_template_name(FunctionTemplateNameT {
                human_name: self.keywords.drop,
                code_location: function_a.range.begin,
                _phantom: std::marker::PhantomData,
            }));

        // We declare the function into the environment that we use to compile the
        // struct, so that those who use the struct can reach into its environment
        // and see the function and use it.
        // See CSFMSEO and SAFHE.
        let call_func_name_t = INameT::FunctionTemplate(
            self.typing_interner.intern_function_template_name(FunctionTemplateNameT {
                human_name: self.keywords.underscores_call,
                code_location: function_a.range.begin,
                _phantom: std::marker::PhantomData,
            }));


        let drop_name_s = self.scout_arena.intern_name(
            INameValS::FunctionDeclaration(
                IFunctionDeclarationNameValS::FunctionName(FunctionNameS {
                    name: self.keywords.drop,
                    code_location: function_a.range.begin,
                })));
        let drop_function_decl_name_s = match drop_name_s {
            INameS::FunctionDeclaration(f) => f,
            _ => panic!("unexpected"),
        };

        let drop_function_a =
            self.make_implicit_drop_function_struct_drop(*drop_function_decl_name_s, function_a.range);
        let drop_function_a_ref = self.scout_arena.alloc(drop_function_a);

        let mut outer_store = TemplatasStoreBuilder::new(understruct_templated_id);
        outer_store.add_entries(
            self.scout_arena,
            vec![
                (call_func_name_t, IEnvEntryT::Function(function_a)),
                (drop_func_name_t, IEnvEntryT::Function(drop_function_a_ref)),
                (understruct_instantiated_name_t, IEnvEntryT::Templata(
                    ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: KindT::Struct(understruct_struct_tt) })))),
                (INameT::Self_(self.typing_interner.intern_self_name(SelfNameT { _phantom: std::marker::PhantomData })),
                 IEnvEntryT::Templata(
                    ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: KindT::Struct(understruct_struct_tt) })))),
            ]);
        let outer_templatas = outer_store.build_in(self.typing_interner);

        let struct_outer_env = self.typing_interner.alloc(CitizenEnvironmentT {
            global_env: containing_function_env.global_env(),
            parent_env: containing_function_env.into(),
            template_id: *understruct_templated_id,
            id: *understruct_templated_id,
            templatas: outer_templatas,
        });

        let mut inner_store = TemplatasStoreBuilder::new(understruct_instantiated_id);
        // There are no inferences we'd need to add, because it's a lambda and they don't have
        // any rules or anything.
        inner_store.add_entries(self.scout_arena, vec![]);
        let inner_templatas = inner_store.build_in(self.typing_interner);

        let struct_inner_env = self.typing_interner.alloc(CitizenEnvironmentT {
            global_env: struct_outer_env.global_env,
            parent_env: IEnvironmentT::Citizen(struct_outer_env),
            template_id: *understruct_templated_id,
            id: *understruct_instantiated_id,
            templatas: inner_templatas,
        });

        // We return this from the function in case we want to eagerly compile it (which we do
        // if it's not a template).
        let function_templata = FunctionTemplataT {
            outer_env: IEnvironmentT::Citizen(struct_inner_env),
            function: function_a,
        };

        coutputs.declare_type(understruct_templated_id);
        coutputs.declare_type_outer_env(understruct_templated_id,
            IInDenizenEnvironmentT::Citizen(struct_outer_env));
        coutputs.declare_type_inner_env(understruct_templated_id,
            IInDenizenEnvironmentT::Citizen(struct_inner_env));
        coutputs.declare_type_mutability(understruct_templated_id, ITemplataT::Mutability(MutabilityTemplataT { mutability }));

        let closure_struct_definition = StructDefinitionT {
            template_name: *understruct_templated_id,
            instantiated_citizen: *understruct_struct_tt,
            attributes: self.typing_interner.alloc_slice_from_vec(vec![]),
            weakable: false,
            mutability: ITemplataT::Mutability(MutabilityTemplataT { mutability }),
            members: self.typing_interner.alloc_slice_from_vec(members.iter().map(|m| {
                let tyype = match &m.tyype {
                    IMemberTypeT::Address(a) => IMemberTypeT::Address(AddressMemberTypeT { reference: a.reference }),
                    IMemberTypeT::Reference(r) => IMemberTypeT::Reference(ReferenceMemberTypeT { reference: r.reference }),
                };
                IStructMemberT::Normal(NormalStructMemberT { name: m.name, variability: m.variability, tyype })
            }).collect::<Vec<_>>()),
            is_closure: true,
            instantiation_bound_params: self.typing_interner.alloc(InstantiationBoundArgumentsT {
                rune_to_bound_prototype: self.typing_interner.alloc_index_map(),
                rune_to_citizen_rune_to_reachable_prototype: self.typing_interner.alloc_index_map(),
                rune_to_bound_impl: self.typing_interner.alloc_index_map(),
            }),
        };
        coutputs.add_struct(self.typing_interner.alloc(closure_struct_definition));

        let closured_vars_struct_ref = *understruct_struct_tt;

        // Always evaluate a drop, drops only capture borrows so there should always be a drop defined
        // on all members.
        let drop_function_templata = {
            let inner_env: IEnvironmentT = IEnvironmentT::Citizen(struct_inner_env);
            match inner_env.lookup_nearest_with_name(
                drop_func_name_t,
                HashSet::from([ILookupContext::ExpressionLookupContext]),
                self.typing_interner,
            ) {
                Some(ITemplataT::Function(ft)) => *ft,
                _ => panic!("Couldn't find closure drop function we just added!"),
            }
        };
        self.evaluate_generic_function_from_non_call(
            coutputs, parent_ranges, call_location, drop_function_templata)?;

        Ok((closured_vars_struct_ref, mutability, function_templata))
    }
/*
  // Makes a struct to back a closure
  def makeClosureUnderstruct(
    containingFunctionEnv: NodeEnvironmentT,
    coutputs: CompilerOutputs,
    parentRanges: List[RangeS],
    callLocation: LocationInDenizen,
    name: IFunctionDeclarationNameS,
    functionA: FunctionA,
    members: Vector[NormalStructMemberT]):
  (StructTT, MutabilityT, FunctionTemplataT) = {
    val isMutable =
      members.exists({ case NormalStructMemberT(name, variability, tyype) =>
        if (variability == VaryingT) {
          true
        } else {
          tyype match {
            case AddressMemberTypeT(reference) => true
            case ReferenceMemberTypeT(reference) => {
              reference.ownership match {
                case OwnT | BorrowT | WeakT => true
                case ShareT => false
              }
            }
          }
        }
      })
    val mutability = if (isMutable) MutableT else ImmutableT

    val understructTemplateNameT =
      interner.intern(LambdaCitizenTemplateNameT(nameTranslator.translateCodeLocation(functionA.range.begin)))
    val understructTemplatedId =
      containingFunctionEnv.id
        .addStep(understructTemplateNameT)

    val understructInstantiatedNameT =
      understructTemplateNameT.makeStructName(interner, Vector())
    val understructInstantiatedId =
      containingFunctionEnv.id.addStep(understructInstantiatedNameT)

    // Lambdas have no bounds, so we just supply Map()
    coutputs.addInstantiationBounds(
      opts.globalOptions.sanityCheck,
      interner,
      understructTemplatedId,
      understructInstantiatedId,
      InstantiationBoundArgumentsT.make(
        Map(),
        Map(), // Structs don't have reachable bounds
        Map()))
    val understructStructTT = interner.intern(StructTT(understructInstantiatedId))

    val dropFuncNameT =
      interner.intern(FunctionTemplateNameT(keywords.drop, functionA.range.begin))

    // We declare the function into the environment that we use to compile the
    // struct, so that those who use the struct can reach into its environment
    // and see the function and use it.
    // See CSFMSEO and SAFHE.
    val structOuterEnv =
      CitizenEnvironmentT(
        containingFunctionEnv.globalEnv,
        containingFunctionEnv,
        understructTemplatedId,
        understructTemplatedId,
        TemplatasStore(understructTemplatedId, Map(), Map())
          .addEntries(
            interner,
            Vector(
              interner.intern(FunctionTemplateNameT(keywords.underscoresCall, functionA.range.begin)) ->
                env.FunctionEnvEntry(functionA),
              dropFuncNameT ->
                FunctionEnvEntry(
                  containingFunctionEnv.globalEnv.structDropMacro.makeImplicitDropFunction(
                    interner.intern(FunctionNameS(keywords.drop, functionA.range.begin)), functionA.range)),
              understructInstantiatedNameT -> TemplataEnvEntry(KindTemplataT(understructStructTT)),
              interner.intern(SelfNameT()) -> TemplataEnvEntry(KindTemplataT(understructStructTT)))))

    val structInnerEnv =
      CitizenEnvironmentT(
        structOuterEnv.globalEnv,
        structOuterEnv,
        understructTemplatedId,
        understructInstantiatedId,
        TemplatasStore(understructInstantiatedId, Map(), Map())
          // There are no inferences we'd need to add, because it's a lambda and they don't have
          // any rules or anything.
          .addEntries(interner, Vector()))

    // We return this from the function in case we want to eagerly compile it (which we do
    // if it's not a template).
    val functionTemplata = FunctionTemplataT(structInnerEnv, functionA)

    coutputs.declareType(understructTemplatedId)
    coutputs.declareTypeOuterEnv(understructTemplatedId, structOuterEnv)
    coutputs.declareTypeInnerEnv(understructTemplatedId, structInnerEnv)
    coutputs.declareTypeMutability(understructTemplatedId, MutabilityTemplataT(mutability))

    val closureStructDefinition =
      StructDefinitionT(
        understructTemplatedId,
        understructStructTT,
        Vector.empty,
        false,
        MutabilityTemplataT(mutability),
        members,
        true,
        // Closures have no function bounds or impl bounds
        InstantiationBoundArgumentsT.make[FunctionBoundNameT, ImplBoundNameT](
          Map(),
          Map(), // Structs don't have reachable bounds
          Map()));
    coutputs.addStruct(closureStructDefinition)

    val closuredVarsStructRef = understructStructTT;

//    if (mutability == ImmutableT) {
      // Adds the free function to the coutputs
      // Free is indeed ordinary because it just takes in the lambda struct. The lambda struct
      // isn't templated. The lambda call function might be, but the struct isnt.

    // Always evaluate a drop, drops only capture borrows so there should always be a drop defined
    // on all members.
      delegate.evaluateGenericFunctionFromNonCallForHeader(
        coutputs,
        parentRanges,
        callLocation,
        structInnerEnv.lookupNearestWithName(dropFuncNameT, Set(ExpressionLookupContext)) match {
          case Some(ft@FunctionTemplataT(_, _)) => ft
          case _ => throw CompileErrorExceptionT(RangedInternalErrorT(functionA.range :: parentRanges, "Couldn't find closure drop function we just added!"))
        })
//    }

    (closuredVarsStructRef, mutability, functionTemplata)
  }
}
*/
}