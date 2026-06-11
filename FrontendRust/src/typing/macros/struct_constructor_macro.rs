use crate::interner::StrI;
use crate::utils::range::RangeS;
use crate::postparsing::ast::LocationInDenizen;
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::compiler_outputs::*;
use crate::typing::compiler::Compiler;
use crate::typing::templata::templata::*;
use crate::typing::templata_compiler::IBoundArgumentsSource;
use crate::typing::ast::citizens::{IStructMemberT, IMemberTypeT};
use crate::higher_typing::ast::*;
use crate::postparsing::names::{IRuneValS, ReturnRuneS, StructNameRuneS, ImplicitCoercionKindRuneValS, ICitizenDeclarationNameS, IVarNameS, IFunctionDeclarationNameValS, INameValS, IStructDeclarationNameS, ConstructorNameS};
use crate::postparsing::rules::rules::{LookupSR, CallSR, CoerceToCoordSR, IRulexSR, RuneUsage};
use crate::postparsing::patterns::patterns::{CaptureS, AtomSP};
use crate::postparsing::ast::{ParameterS, IBodyS, GeneratedBodyS, IStructMemberS};
use crate::postparsing::itemplatatype::{ITemplataType, CoordTemplataType, KindTemplataType, TemplateTemplataType, FunctionTemplataType};
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::typing::names::names::IdValT;
use std::collections::HashMap;
use crate::higher_typing::ast::FunctionA;
use crate::postparsing::names::IFunctionDeclarationNameS;

/*
package dev.vale.typing.macros

import dev.vale.highertyping.{FunctionA, StructA}
import dev.vale.postparsing.patterns.{AtomSP, CaptureS}
import dev.vale.postparsing.rules._
import dev.vale.postparsing._
import dev.vale.typing.{ArrayCompiler, CompileErrorExceptionT, CompilerOutputs, CouldntFindFunctionToCallT, InheritBoundsFromTypeItself, OverloadResolver, TemplataCompiler, TypingPassOptions, UseBoundsFromContainer, ast}
import dev.vale.typing.ast.{ArgLookupTE, BlockTE, ConstructTE, FunctionDefinitionT, FunctionHeaderT, LocationInFunctionEnvironmentT, ParameterT, ReturnTE}
import dev.vale.typing.citizen.StructCompiler
import dev.vale.typing.env.{FunctionEnvEntry, FunctionEnvironmentT}
import dev.vale.typing.names.{CitizenNameT, CitizenTemplateNameT, FunctionNameT, ICitizenNameT, ICitizenTemplateNameT, IFunctionNameT, IFunctionTemplateNameT, INameT, ITemplateNameT, IdT, NameTranslator, KindPlaceholderNameT}
import dev.vale.{Err, Interner, Keywords, Ok, PackageCoordinate, Profiler, RangeS, StrI, vassert, vassertSome, vcurious, vimpl}
import dev.vale.typing.types._
import dev.vale.highertyping.FunctionA
import dev.vale.postparsing.ConstructorNameS
import dev.vale.postparsing.patterns.AtomSP
import dev.vale.typing.OverloadResolver.FindFunctionFailure
import dev.vale.typing.ast._
import dev.vale.typing.env.PackageEnvironmentT
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing.function.{DestructorCompiler, FunctionCompilerCore}
import dev.vale.typing.infer.CouldntFindFunction
import dev.vale.typing.templata.ITemplataT.expectMutability
import dev.vale.typing.templata._
import dev.vale.typing.types.InterfaceTT

import scala.collection.mutable

*/
// (Scala `class StructConstructorMacro(opts, interner, keywords, nameTranslator,
//  destructorCompiler)` absorbed onto `Compiler`; the method bodies live at
//  `Compiler::get_struct_sibling_entries_struct_constructor` and
//  `Compiler::generate_function_body_struct_constructor` below.)
/*
class StructConstructorMacro(
  opts: TypingPassOptions,
  interner: Interner,
  keywords: Keywords,
  nameTranslator: NameTranslator,
  destructorCompiler: DestructorCompiler,
) extends IOnStructDefinedMacro with IFunctionBodyMacro {

  val generatorId: StrI = keywords.structConstructorGenerator

  val macroName: StrI = keywords.DeriveStructConstructor

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_struct_sibling_entries_struct_constructor(
        &self,
        struct_name: IdT<'s, 't>,
        struct_a: &'s StructA<'s>,
    ) -> Vec<(IdT<'s, 't>, IEnvEntryT<'s, 't>)> {

        if struct_a.members.iter().any(|m| matches!(m, IStructMemberS::VariadicStructMember(_))) {
            // Dont generate constructors for variadic structs, not supported yet.
            // Only one we have right now is tuple, which has its own special syntax for constructing.
            return vec![];
        }
        let mut rune_to_type: HashMap<_, _> = HashMap::new();
        let mut rules: Vec<IRulexSR<'s>> = Vec::new();

        // We dont need these, they really just contain bounds and stuff, which we'd inherit from our parameters anyway.
        // However, if we leave it out, then this (from an IRAGP test):
        //   struct Bork<T, Y> where T = Y { t T; y Y; }
        // thing's constructor would be:
        //   func Bork<T, Y>(t T, y Y) Bork<T, Y> { ... }
        // and it fails to resolve that return type there because it doesn't meet the struct's conditions, because it didn't
        // repeat the rules from the struct's header, specifically the T = Y rule.
        // So, we just include all the rules from the constructor's header.
        // If we ever need to drop that functionality (the T = Y nonsense) then we can probably take out the inheriting of
        // the header rules.
        for (k, v) in struct_a.header_rune_to_type.iter() { rune_to_type.insert(*k, *v); }
        for r in struct_a.header_rules.iter() { rules.push(*r); }

        // We include these because they become our parameters. If a struct contains a Opt<^MyNode<T>> we want those two
        // CallSRs in our function rules too.
        for (k, v) in struct_a.members_rune_to_type.iter() { rune_to_type.insert(*k, *v); }
        for r in struct_a.member_rules.iter() { rules.push(*r); }

        let struct_name_range = struct_a.name.range();
        let ret_rune_s = self.scout_arena.intern_rune(IRuneValS::ReturnRune(ReturnRuneS {}));
        let ret_rune = RuneUsage { range: struct_name_range, rune: ret_rune_s };
        rune_to_type.insert(ret_rune.rune, ITemplataType::CoordTemplataType(CoordTemplataType {}));

        let struct_name_as_citizen: ICitizenDeclarationNameS<'s> = struct_a.name.into();
        let struct_generic_rune_s = self.scout_arena.intern_rune(IRuneValS::StructNameRune(StructNameRuneS { struct_name: struct_name_as_citizen }));
        let struct_generic_rune = RuneUsage { range: struct_name_range, rune: struct_generic_rune_s };
        rune_to_type.insert(struct_generic_rune.rune, ITemplataType::TemplateTemplataType(struct_a.tyype));

        let struct_imprecise_name = struct_a.name.get_imprecise_name(self.scout_arena);
        rules.push(IRulexSR::Lookup(LookupSR {
            range: struct_name_range,
            rune: struct_generic_rune,
            name: struct_imprecise_name,
        }));

        let struct_kind_rune_s = self.scout_arena.intern_rune(IRuneValS::ImplicitCoercionKindRune(ImplicitCoercionKindRuneValS {
            range: struct_name_range,
            original_coord_rune: struct_generic_rune_s,
        }));
        let struct_kind_rune = RuneUsage { range: struct_name_range, rune: struct_kind_rune_s };
        rune_to_type.insert(struct_kind_rune.rune, ITemplataType::KindTemplataType(KindTemplataType {}));
        let generic_param_runes: Vec<_> = struct_a.generic_parameters.iter().map(|p| p.rune).collect();
        let generic_param_runes_slice = self.scout_arena.alloc_slice_copy(&generic_param_runes);
        rules.push(IRulexSR::Call(CallSR {
            range: struct_name_range,
            result_rune: struct_kind_rune,
            template_rune: struct_generic_rune,
            args: generic_param_runes_slice,
        }));

        rules.push(IRulexSR::CoerceToCoord(CoerceToCoordSR {
            range: struct_name_range,
            coord_rune: ret_rune,
            kind_rune: struct_kind_rune,
        }));

        let params: Vec<ParameterS<'s>> = struct_a.members.iter().flat_map(|m| {
            match m {
                IStructMemberS::NormalStructMember(member) => {
                    let capture = CaptureS { name: IVarNameS::CodeVarName(member.name), mutate: false };
                    vec![ParameterS::new(member.range, None, false, AtomSP {
                        range: member.range,
                        name: Some(capture),
                        coord_rune: Some(member.type_rune),
                        destructure: None,
                    })]
                }
                IStructMemberS::VariadicStructMember(_) => vec![],
            }
        }).collect();
        for param in &params {
            if let Some(coord_rune) = param.pattern.coord_rune {
                rune_to_type.insert(coord_rune.rune, ITemplataType::CoordTemplataType(CoordTemplataType {}));
            }
        }

        let mut rune_to_type_map = self.scout_arena.alloc_index_map();
        for (k, v) in rune_to_type { rune_to_type_map.insert(k, v); }
        let params_slice = self.scout_arena.alloc_slice_from_vec(params);
        let rules_slice = self.scout_arena.alloc_slice_copy(&rules);
        let function_a = self.scout_arena.alloc(FunctionA::new(
            struct_a.range,
            IFunctionDeclarationNameS::ConstructorName(
                &*self.scout_arena.alloc(ConstructorNameS { tlcd: struct_name_as_citizen })
            ),
            &[],
            TemplateTemplataType { param_types: struct_a.tyype.param_types, return_type: self.scout_arena.alloc(ITemplataType::FunctionTemplataType(FunctionTemplataType {})) },
            struct_a.generic_parameters,
            rune_to_type_map,
            params_slice,
            Some(ret_rune),
            rules_slice,
            IBodyS::GeneratedBody(GeneratedBodyS { generator_id: self.keywords.struct_constructor_generator }),
        ));
        let function_name_s = self.scout_arena.intern_name(INameValS::FunctionDeclaration(IFunctionDeclarationNameValS::ConstructorName(
            ConstructorNameS { tlcd: struct_name_as_citizen }
        )));
        let translated_local_name = self.translate_name_step(function_name_s);
        let result_id = *self.typing_interner.intern_id(IdValT {
            package_coord: struct_name.package_coord,
            init_steps: struct_name.init_steps,
            local_name: translated_local_name,
        });
        vec![(result_id, IEnvEntryT::Function(function_a))]
    }
/*
  override def getStructSiblingEntries(structName: IdT[INameT], structA: StructA):
  Vector[(IdT[INameT], FunctionEnvEntry)] = {
    if (structA.members.collect({ case VariadicStructMemberS(_, _, _) => }).nonEmpty) {
      // Dont generate constructors for variadic structs, not supported yet.
      // Only one we have right now is tuple, which has its own special syntax for constructing.
      return Vector()
    }
    val runeToType = mutable.HashMap[IRuneS, ITemplataType]()
    val rules = mutable.ArrayBuffer[IRulexSR]()

    // We dont need these, they really just contain bounds and stuff, which we'd inherit from our parameters anyway.
    // However, if we leave it out, then this (from an IRAGP test):
    //   struct Bork<T, Y> where T = Y { t T; y Y; }
    // thing's constructor would be:
    //   func Bork<T, Y>(t T, y Y) Bork<T, Y> { ... }
    // and it fails to resolve that return type there because it doesn't meet the struct's conditions, because it didn't
    // repeat the rules from the struct's header, specifically the T = Y rule.
    // So, we just include all the rules from the constructor's header.
    // If we ever need to drop that functionality (the T = Y nonsense) then we can probably take out the inheriting of
    // the header rules.
    runeToType ++= structA.headerRuneToType
    rules ++= structA.headerRules

    // We include these because they become our parameters. If a struct contains a Opt<^MyNode<T>> we want those two
    // CallSRs in our function rules too.
    runeToType ++= structA.membersRuneToType
    rules ++= structA.memberRules


    val retRune = RuneUsage(structA.name.range, ReturnRuneS())
    runeToType += (retRune.rune -> CoordTemplataType())
    val structNameRange = structA.name.range
    val structGenericRune = StructNameRuneS(structA.name)
    runeToType += (structGenericRune -> structA.tyype)
    rules += LookupSR(structNameRange, RuneUsage(structNameRange, structGenericRune), structA.name.getImpreciseName(interner))

    val structKindRune = RuneUsage(structNameRange, ImplicitCoercionKindRuneS(structNameRange, structGenericRune))
    runeToType += (structKindRune.rune -> KindTemplataType())
    rules += CallSR(structNameRange, structKindRune, RuneUsage(structNameRange, structGenericRune), structA.genericParameters.map(_.rune).toVector)

    rules += CoerceToCoordSR(structNameRange, retRune, structKindRune)

    val params =
      structA.members.zipWithIndex.flatMap({
        case (NormalStructMemberS(range, name, variability, typeRune), index) => {
          val capture = CaptureS(interner.intern(CodeVarNameS(name)), false)
          Vector(ParameterS(range, None, false, AtomSP(range, Some(capture), Some(typeRune), None)))
        }
        case (VariadicStructMemberS(range, variability, typeRune), index) => {
          Vector()
        }
      })
    runeToType ++= params.flatMap(_.pattern.coordRune.map(_.rune)).map(_ -> CoordTemplataType())

    val functionA =
      FunctionA(
        structA.range,
        interner.intern(ConstructorNameS(structA.name)),
        Vector(),
        TemplateTemplataType(structA.tyype.paramTypes, FunctionTemplataType()),
        structA.genericParameters,
        runeToType.toMap,
        params,
        Some(retRune),
        rules.toVector,
        GeneratedBodyS(generatorId))

    Vector(
      structName.copy(localName = nameTranslator.translateNameStep(functionA.name)) ->
        FunctionEnvEntry(functionA))
  }


*/
    pub fn generate_function_body_struct_constructor(
        &self,
        coutputs: &mut CompilerOutputs<'s, 't>,
        env: &'t FunctionEnvironmentT<'s, 't>,
        generator_id: StrI<'s>,
        life: LocationInFunctionEnvironmentT<'t>,
        call_range: &[RangeS<'s>],
        call_location: LocationInDenizen<'s>,
        origin_function: Option<&FunctionA<'s>>,
        param_coords: &[ParameterT<'s, 't>],
        maybe_ret_coord: Option<CoordT<'s, 't>>,
    ) -> (FunctionHeaderT<'s, 't>, ReferenceExpressionTE<'s, 't>) {
        let ret_coord = maybe_ret_coord.expect("vassertSome: maybeRetCoord");
        let struct_tt = match ret_coord.kind {
            KindT::Struct(s) => s,
            _ => panic!("Expected struct kind in generate_function_body_struct_constructor"),
        };
        let definition = coutputs.lookup_struct(struct_tt.id, self);
        let instantiation_bound_params = definition.instantiation_bound_params;
        let instantiation_bounds = coutputs.get_instantiation_bounds(self.typing_interner, struct_tt.id).expect("vassertSome: getInstantiationBounds");
        let bound_arguments_source = IBoundArgumentsSource::UseBoundsFromContainer {
            instantiation_bound_params,
            instantiation_bound_arguments: instantiation_bounds,
        };
        let members: Vec<(IVarNameT<'s, 't>, CoordT<'s, 't>)> = {
            let placeholder_substituter = self.get_placeholder_substituter(
                false, // sanity_check
                env.template_id,
                struct_tt.id,
                bound_arguments_source,
            );
            definition.members.iter().map(|member| {
                match member {
                    IStructMemberT::Normal(n) => {
                        match &n.tyype {
                            IMemberTypeT::Reference(r) => {
                                (n.name, placeholder_substituter.substitute_for_coord(coutputs, r.reference))
                            }
                            IMemberTypeT::Address(_) => panic!("vcurious: AddressMemberTypeT in generate_function_body_struct_constructor"),
                        }
                    }
                    IStructMemberT::Variadic(_) => panic!("vimpl: VariadicStructMemberT in generate_function_body_struct_constructor"),
                }
            }).collect()
        };

        let constructor_id = env.id;
        assert!(
            constructor_id.local_name.parameters().len() == members.len(),
            "vassert: constructorId.localName.parameters.size == members.size"
        );

        let constructor_params: Vec<ParameterT<'s, 't>> = members.iter().map(|(name, coord)| {
            ParameterT { name: *name, virtuality: None, pre_checked: false, tyype: *coord }
        }).collect();

        let bound_arguments_source2 = IBoundArgumentsSource::UseBoundsFromContainer {
            instantiation_bound_params,
            instantiation_bound_arguments: instantiation_bounds,
        };
        let mutability = self.struct_compiler_get_mutability(
            false, // sanity_check
            coutputs,
            env.template_id,
            RegionT { region: IRegionT::Default },
            *struct_tt,
            bound_arguments_source2,
        );
        let constructor_return_ownership = match mutability {
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Mutable }) => OwnershipT::Own,
            ITemplataT::Mutability(MutabilityTemplataT { mutability: MutabilityT::Immutable }) => OwnershipT::Share,
            ITemplataT::Placeholder(p) if matches!(p.tyype, ITemplataType::MutabilityTemplataType(_)) => OwnershipT::Own,
            _ => panic!("Unexpected mutability type in generate_function_body_struct_constructor"),
        };
        let constructor_return_type = CoordT { ownership: constructor_return_ownership, region: RegionT { region: IRegionT::Default }, kind: KindT::Struct(struct_tt) };

        let constructor_params_slice = self.typing_interner.alloc_slice_from_vec(constructor_params);
        let header = FunctionHeaderT {
            id: constructor_id,
            attributes: self.typing_interner.alloc_slice_from_vec(vec![]),
            params: constructor_params_slice,
            return_type: constructor_return_type,
            maybe_origin_function_templata: Some(env.templata()),
        };

        let args: Vec<ExpressionTE<'s, 't>> = constructor_params_slice.iter().enumerate().map(|(index, p)| {
            ExpressionTE::Reference(ReferenceExpressionTE::ArgLookup(self.typing_interner.alloc(ArgLookupTE { param_index: index as i32, coord: p.tyype })))
        }).collect();
        let args_slice = self.typing_interner.alloc_slice_from_vec(args);
        let struct_tt_ref = self.typing_interner.alloc(struct_tt);
        let construct_expr = ReferenceExpressionTE::Construct(self.typing_interner.alloc(ConstructTE {
            struct_tt: struct_tt_ref,
            result_reference: constructor_return_type,
            args: args_slice,
        }));
        let return_expr = ReferenceExpressionTE::Return(self.typing_interner.alloc(ReturnTE { source_expr: construct_expr }));
        let body = ReferenceExpressionTE::Block(self.typing_interner.alloc(BlockTE { inner: return_expr }));
        (header, body)
    }
/*
  override def generateFunctionBody(
    env: FunctionEnvironmentT,
    coutputs: CompilerOutputs,
    generatorId: StrI,
    life: LocationInFunctionEnvironmentT,
    callRange: List[RangeS],
      callLocation: LocationInDenizen,
    originFunction: Option[FunctionA],
    paramCoords: Vector[ParameterT],
    maybeRetCoord: Option[CoordT]):
  (FunctionHeaderT, ReferenceExpressionTE) = {
    val Some(CoordT(_, _, structTT @ StructTT(_))) = maybeRetCoord
    val definition = coutputs.lookupStruct(structTT.id)
    val placeholderSubstituter =
      TemplataCompiler.getPlaceholderSubstituter(
        opts.globalOptions.sanityCheck,
        interner,
        keywords,
        env.denizenTemplateId,
        structTT.id,
        // We only know about this struct from the return type, we don't get to inherit any of its
        // bounds or guarantees from. Satisfy them from our environment instead.
        UseBoundsFromContainer(
          definition.instantiationBoundParams,
          vassertSome(coutputs.getInstantiationBounds(structTT.id))))
    val members =
      definition.members.map({
        case NormalStructMemberT(name, _, ReferenceMemberTypeT(tyype)) => {
          (name, placeholderSubstituter.substituteForCoord(coutputs, tyype))
        }
        case NormalStructMemberT(name, variability, AddressMemberTypeT(tyype)) => vcurious()
        case VariadicStructMemberT(name, tyype) => vimpl()
      })

    val constructorId = env.id
    vassert(constructorId.localName.parameters.size == members.size)
    val constructorParams =
      members.map({ case (name, coord) => ParameterT(name, None, false, coord) })
    val mutability =
      StructCompiler.getMutability(
        opts.globalOptions.sanityCheck,
        interner, keywords, coutputs, env.denizenTemplateId, RegionT(DefaultRegionT), structTT,
        // Not entirely sure if this is right, but it's consistent with using it for the return kind
        // and its the more conservative option so we'll go with it for now.
        UseBoundsFromContainer(
          definition.instantiationBoundParams,
          vassertSome(coutputs.getInstantiationBounds(structTT.id))))
    val constructorReturnOwnership =
      mutability match {
        case MutabilityTemplataT(MutableT) => OwnT
        case MutabilityTemplataT(ImmutableT) => ShareT
        case PlaceholderTemplataT(idT, MutabilityTemplataType()) => OwnT
      }
    val constructorReturnType = CoordT(constructorReturnOwnership, RegionT(DefaultRegionT), structTT)

    // not virtual because how could a constructor be virtual
    val header =
      ast.FunctionHeaderT(
        constructorId,
        Vector.empty,
        constructorParams,
        constructorReturnType,
        Some(env.templata))

    val body =
      BlockTE(
        ReturnTE(
          ConstructTE(
            structTT,
            constructorReturnType,
            constructorParams.zipWithIndex.map({ case (p, index) => ArgLookupTE(index, p.tyype) }))))
    (header, body)
  }
}
*/
}
