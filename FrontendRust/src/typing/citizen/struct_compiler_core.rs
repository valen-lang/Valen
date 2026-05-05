use crate::higher_typing::ast::{FunctionA, InterfaceA, StructA};
use crate::postparsing::ast::{ICitizenAttributeS, IStructMemberS, LocationInDenizen};
use crate::postparsing::names::IFunctionDeclarationNameS;
use crate::typing::ast::ast::ICitizenAttributeT;
use crate::typing::ast::citizens::{IStructMemberT, InterfaceDefinitionT, NormalStructMemberT, StructDefinitionT};
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::env::environment::{CitizenEnvironmentT, IInDenizenEnvironmentT};
use crate::typing::env::function_environment_t::NodeEnvironmentT;
use crate::typing::templata::templata::FunctionTemplataT;
use crate::typing::hinputs_t::InstantiationBoundArgumentsT;
use crate::typing::types::types::{MutabilityT, StructTT};
use crate::utils::range::RangeS;

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
import dev.vale.typing.ast.{ICitizenAttributeT, SealedT}
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
        outer_env: &'t IInDenizenEnvironmentT<'s, 't>,
        struct_runes_env: &'t CitizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        struct_a: &'s StructA<'s>,
    ) {
        panic!("Unimplemented: Slab 15 — body migration");
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
        panic!("Unimplemented: Slab 15 — body migration");
    }
/*
  def translateCitizenAttributes(attrs: Vector[ICitizenAttributeS]): Vector[ICitizenAttributeT] = {
    attrs.map({
      case SealedS => SealedT
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
        outer_env: &'t IInDenizenEnvironmentT<'s, 't>,
        interface_runes_env: &'t CitizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        parent_ranges: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        interface_a: &'s InterfaceA<'s>,
    ) -> &'t InterfaceDefinitionT<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
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
        env: &'t IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        members: &[IStructMemberS<'s>],
    ) -> Vec<IStructMemberT<'s, 't>> {
        panic!("Unimplemented: Slab 15 — body migration");
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
        env: &'t IInDenizenEnvironmentT<'s, 't>,
        coutputs: &mut CompilerOutputs<'s, 't>,
        member: IStructMemberS<'s>,
    ) -> IStructMemberT<'s, 't> {
        panic!("Unimplemented: Slab 15 — body migration");
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
    ) -> (StructTT<'s, 't>, MutabilityT, FunctionTemplataT<'s, 't>) {
        use crate::typing::names::names::*;
        use crate::typing::templata::templata::*;
        use crate::typing::types::types::*;

        let is_mutable = members.iter().any(|_m| {
            panic!("implement: is_mutable check in make_closure_understruct_core")
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

        use crate::postparsing::names::{INameValS, IFunctionDeclarationNameValS, FunctionNameS, INameS, IFunctionDeclarationNameS};
        use crate::typing::env::i_env_entry::IEnvEntryT;
        use crate::typing::env::environment::{TemplatasStoreBuilder, IEnvironmentT};

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
            outer_env: self.typing_interner.alloc(IEnvironmentT::Citizen(struct_inner_env)),
            function: function_a,
        };

        coutputs.declare_type(understruct_templated_id);
        coutputs.declare_type_outer_env(understruct_templated_id,
            self.typing_interner.alloc(IInDenizenEnvironmentT::Citizen(struct_outer_env)));
        coutputs.declare_type_inner_env(understruct_templated_id,
            self.typing_interner.alloc(IInDenizenEnvironmentT::Citizen(struct_inner_env)));
        coutputs.declare_type_mutability(understruct_templated_id, ITemplataT::Mutability(MutabilityTemplataT { mutability }));

        let closure_struct_definition = StructDefinitionT {
            template_name: *understruct_templated_id,
            instantiated_citizen: *understruct_struct_tt,
            attributes: self.typing_interner.alloc_slice_from_vec(vec![]),
            weakable: false,
            mutability: ITemplataT::Mutability(MutabilityTemplataT { mutability }),
            members: self.typing_interner.alloc_slice_from_vec(members.iter().map(|_m| {
                panic!("implement: convert NormalStructMemberT ref to owned IStructMemberT")
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
        use std::collections::HashSet;
        use crate::typing::env::environment::ILookupContext;
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
            coutputs, parent_ranges, call_location, drop_function_templata);

        (closured_vars_struct_ref, mutability, function_templata)
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