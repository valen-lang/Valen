use crate::higher_typing::ast::*;
use crate::postparsing::ast::NormalStructMemberS;
use crate::postparsing::names::{AnonymousSubstructTemplateNameS, IRuneS};
use crate::postparsing::rules::rules::{IRulexSR, RuneUsage};
use crate::typing::names::names::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::compiler::Compiler;
use crate::postparsing::ast::ICitizenAttributeS;
use crate::postparsing::ast::SealedS;
use crate::postparsing::rules::rules::LookupSR;
use crate::postparsing::rules::rules::CallSR;
use crate::postparsing::itemplatatype::{ITemplataType, KindTemplataType, TemplateTemplataType};
use crate::typing::names::names::IdValT;
use crate::higher_typing::ast::ImplA;
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::postparsing::rules::rules::CoerceToCoordSR;
use crate::postparsing::rules::rules::AugmentSR;
use crate::postparsing::names::{IRuneValS, AnonymousSubstructMethodInheritedRuneValS};
use crate::postparsing::names::AnonymousSubstructVoidKindRuneS;
use crate::postparsing::names::AnonymousSubstructVoidCoordRuneS;
use crate::postparsing::names::CodeNameS;
use crate::postparsing::names::IImpreciseNameValS;
use crate::postparsing::itemplatatype::CoordTemplataType;
use crate::utils::range::RangeS;
use crate::postparsing::rules::rules::PackSR;
use crate::postparsing::rules::rules::DefinitionFuncSR;
use crate::postparsing::rules::rules::CallSiteFuncSR;
use crate::postparsing::rules::rules::ResolveSR;
use crate::postparsing::itemplatatype::{PackTemplataType, PrototypeTemplataType};
use crate::parsing::ast::ast::OwnershipP;
use crate::postparsing::ast::ParameterS;
use crate::postparsing::itemplatatype::OwnershipTemplataType;
use crate::postparsing::itemplatatype::FunctionTemplataType;
use crate::postparsing::rules::rules::CoordComponentsSR;
use crate::postparsing::ast::{GenericParameterS, IBodyS, CodeBodyS, LocationInDenizen, AbstractSP};
use crate::postparsing::expressions::{BodySE, BlockSE, IExpressionSE, FunctionCallSE, DotSE, LocalLoadSE, LocalS, IVariableUseCertainty};
use crate::postparsing::patterns::patterns::{AtomSP, CaptureS};
use crate::parsing::ast::ast::LoadAsP;
use crate::postparsing::names::AnonymousSubstructMemberRuneS;
use crate::parsing::ast::VariabilityP;
use crate::postparsing::names::INameS;
use crate::postparsing::ast::IGenericParameterTypeS;
use crate::postparsing::ast::CoordGenericParameterTypeS;
use crate::postparsing::ast::IStructMemberS;
use crate::postparsing::names::IStructDeclarationNameS;

/*
package dev.vale.typing.macros

import dev.vale.highertyping.{FunctionA, ImplA, InterfaceA, StructA}
import dev.vale._
import dev.vale.parsing.ast.{BorrowP, FinalP, OwnP, UseP}
import dev.vale.postparsing.patterns._
import dev.vale.postparsing._
import dev.vale.postparsing.rules._
import dev.vale.typing.{OverloadResolver, TypingPassOptions}
import dev.vale.typing.citizen.StructCompiler
import dev.vale.typing.env.{FunctionEnvEntry, IEnvEntry, ImplEnvEntry, StructEnvEntry}
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing.macros.citizen._
import dev.vale.typing.names._
import dev.vale.typing.types.MutabilityT
import dev.vale.highertyping.FunctionA
import dev.vale.postparsing.patterns._
import dev.vale.typing.ast._
import dev.vale.typing.env.PackageEnvironmentT
import dev.vale.typing.function.FunctionCompilerCore
import dev.vale.typing.macros.citizen.StructDropMacro
import dev.vale.typing.names.AnonymousSubstructImplNameT
import dev.vale.typing.templata.ExternFunctionTemplataT
import dev.vale.typing.ast
import dev.vale.typing.types.CoordT

import scala.collection.immutable.List
import scala.collection.mutable

*/
// (Scala `class AnonymousInterfaceMacro(opts, interner, keywords, nameTranslator,
//  overloadCompiler, structCompiler, structConstructorMacro, structDropMacro)` absorbed
//  onto `Compiler`; the method bodies live at
//  `Compiler::{get_interface_sibling_entries_anonymous_interface, map_runes_anonymous_interface,
//  inherited_method_rune_anonymous_interface, make_struct_anonymous_interface,
//  make_forwarder_function_anonymous_interface}` below.)
/*
class AnonymousInterfaceMacro(
    opts: TypingPassOptions,
    interner: Interner,
    keywords: Keywords,
    nameTranslator: NameTranslator,
    overloadCompiler: OverloadResolver,
    structCompiler: StructCompiler,
    structConstructorMacro: StructConstructorMacro,
    structDropMacro: StructDropMacro
) extends IOnInterfaceDefinedMacro {

  val macroName: StrI = keywords.DeriveAnonymousSubstruct

//  val generatorId: String = "interfaceConstructorGenerator"

//  override def getInterfaceChildEntries(interfaceName: FullNameT[INameT], interfaceA: InterfaceA, mutability: MutabilityT): Vector[(FullNameT[INameT], IEnvEntry)] = {
//    vimpl()
//  }

*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_interface_sibling_entries_anonymous_interface(
        &self,
        interface_name: IdT<'s, 't>,
        interface_a: &'s InterfaceA<'s>,
    ) -> Vec<(IdT<'s, 't>, IEnvEntryT<'s, 't>)> {
        use crate::postparsing::names::{
            IRuneValS, AnonymousSubstructTemplateNameS, AnonymousSubstructImplDeclarationNameS,
            AnonymousSubstructTemplateRuneS, AnonymousSubstructKindRuneS,
            AnonymousSubstructParentInterfaceTemplateRuneS, AnonymousSubstructParentInterfaceKindRuneS,
            IImplDeclarationNameS,
        };

        if interface_a.attributes.iter().any(|a| matches!(a, ICitizenAttributeS::Sealed(_))) {
            return vec![];
        }

        let member_runes: Vec<RuneUsage<'s>> =
            interface_a.internal_methods.iter().enumerate().map(|(_index, method)| {
                let rune = self.scout_arena.intern_rune(
                    IRuneValS::AnonymousSubstructMemberRune(AnonymousSubstructMemberRuneS {
                        interface: *interface_a.name,
                        method: method.name,
                    }));
                RuneUsage { range: RangeS::new(method.range.begin, method.range.begin), rune }
            }).collect();
        let members: Vec<NormalStructMemberS<'s>> =
            interface_a.internal_methods.iter().zip(member_runes.iter()).enumerate().map(|(index, (method, rune))| {
                NormalStructMemberS {
                    range: method.range,
                    name: self.scout_arena.intern_str(&index.to_string()),
                    variability: VariabilityP::Final,
                    type_rune: *rune,
                }
            }).collect();

        let struct_name_s = AnonymousSubstructTemplateNameS { interface_name: *interface_a.name };
        let struct_name_s_ref = self.scout_arena.alloc(struct_name_s);
        let struct_local_name = self.translate_name_step(INameS::AnonymousSubstructTemplateName(struct_name_s_ref));
        let struct_name_t_steps = interface_name.init_steps.to_vec();
        let struct_name_t = *self.typing_interner.intern_id(IdValT {
            package_coord: interface_name.package_coord,
            init_steps: &struct_name_t_steps,
            local_name: struct_local_name,
        });

        let struct_a = self.make_struct_anonymous_interface(
            interface_a,
            &member_runes,
            &members,
            struct_name_s,
        );

        let more_entries = self.get_struct_sibling_entries_struct_constructor(struct_name_t, struct_a);
        let mut more_entries2 = self.get_struct_sibling_entries_struct_drop(struct_name_t, struct_a);
        let mut more_entries_combined = more_entries;
        more_entries_combined.append(&mut more_entries2);

        let forwarder_methods: Vec<(IdT<'s, 't>, IEnvEntryT<'s, 't>)> =
            interface_a.internal_methods.iter().zip(member_runes.iter()).enumerate().map(|(method_index, (method, _rune))| {
                let local_name: INameT<'s, 't> = self.translate_generic_function_name(method.name).into();
                let name = *self.typing_interner.intern_id(IdValT {
                    package_coord: struct_name_t.package_coord,
                    init_steps: struct_name_t.init_steps,
                    local_name,
                });
                let forwarder = self.make_forwarder_function_anonymous_interface(
                    struct_name_s, interface_a, struct_a, *method, method_index as i32);
                (name, IEnvEntryT::Function(forwarder))
            }).collect();

        let anon_template_rune = self.scout_arena.intern_rune(
            IRuneValS::AnonymousSubstructTemplateRune(AnonymousSubstructTemplateRuneS {})
        );
        let anon_kind_rune = self.scout_arena.intern_rune(
            IRuneValS::AnonymousSubstructKindRune(AnonymousSubstructKindRuneS {})
        );
        let parent_interface_template_rune = self.scout_arena.intern_rune(
            IRuneValS::AnonymousSubstructParentInterfaceTemplateRune(AnonymousSubstructParentInterfaceTemplateRuneS {})
        );
        let parent_interface_kind_rune = self.scout_arena.intern_rune(
            IRuneValS::AnonymousSubstructParentInterfaceKindRune(AnonymousSubstructParentInterfaceKindRuneS {})
        );

        let struct_imprecise_name = struct_a.name.get_imprecise_name(self.scout_arena);
        let interface_imprecise_name = interface_a.name.get_imprecise_name(self.scout_arena);

        let rules: Vec<IRulexSR<'s>> = vec![
            IRulexSR::Lookup(LookupSR {
                range: struct_a.range,
                rune: RuneUsage { range: struct_a.range, rune: anon_template_rune },
                name: struct_imprecise_name,
            }),
            IRulexSR::Call(CallSR {
                range: struct_a.range,
                result_rune: RuneUsage { range: struct_a.range, rune: anon_kind_rune },
                template_rune: RuneUsage { range: struct_a.range, rune: anon_template_rune },
                args: self.scout_arena.alloc_slice_from_vec(
                    struct_a.generic_parameters.iter().map(|gp| gp.rune).collect()
                ),
            }),
            IRulexSR::Lookup(LookupSR {
                range: interface_a.range,
                rune: RuneUsage { range: interface_a.range, rune: parent_interface_template_rune },
                name: interface_imprecise_name,
            }),
            IRulexSR::Call(CallSR {
                range: interface_a.range,
                result_rune: RuneUsage { range: interface_a.range, rune: parent_interface_kind_rune },
                template_rune: RuneUsage { range: interface_a.range, rune: parent_interface_template_rune },
                args: self.scout_arena.alloc_slice_from_vec(
                    interface_a.generic_parameters.iter().map(|gp| gp.rune).collect()
                ),
            }),
        ];

        let mut rune_to_type: ArenaIndexMap<'s, IRuneS<'s>, ITemplataType<'s>> = self.scout_arena.alloc_index_map();
        for gp in struct_a.generic_parameters.iter() {
            let tyype = *struct_a.header_rune_to_type.get(&gp.rune.rune).unwrap();
            rune_to_type.insert(gp.rune.rune, tyype);
        }
        rune_to_type.insert(anon_kind_rune, ITemplataType::KindTemplataType(KindTemplataType {}));
        rune_to_type.insert(anon_template_rune, ITemplataType::TemplateTemplataType(struct_a.tyype));
        rune_to_type.insert(parent_interface_kind_rune, ITemplataType::KindTemplataType(KindTemplataType {}));
        rune_to_type.insert(parent_interface_template_rune, ITemplataType::TemplateTemplataType(interface_a.tyype));

        let struct_kind_rune_s = RuneUsage { range: interface_a.range, rune: anon_kind_rune };
        let interface_kind_rune_s = RuneUsage { range: interface_a.range, rune: parent_interface_kind_rune };

        let impl_name_s = IImplDeclarationNameS::AnonymousSubstructImplDeclarationName(
            AnonymousSubstructImplDeclarationNameS { interface: *interface_a.name }
        );

        let rules_slice = self.scout_arena.alloc_slice_from_vec(rules);
        let impl_a = self.scout_arena.alloc(ImplA {
            range: interface_a.range,
            name: impl_name_s,
            generic_params: struct_a.generic_parameters,
            rules: rules_slice,
            rune_to_type,
            sub_citizen_rune: struct_kind_rune_s,
            sub_citizen_imprecise_name: struct_imprecise_name,
            interface_kind_rune: interface_kind_rune_s,
            super_interface_imprecise_name: interface_imprecise_name,
        });

        let impl_local_name = self.translate_name_step(impl_a.name.to_i_name_s(self.scout_arena));
        let impl_name_t_steps = struct_name_t.init_steps.to_vec();
        let impl_name_t = *self.typing_interner.intern_id(IdValT {
            package_coord: struct_name_t.package_coord,
            init_steps: &impl_name_t_steps,
            local_name: impl_local_name,
        });

        let mut result = vec![
            (struct_name_t, IEnvEntryT::Struct(struct_a)),
            (impl_name_t, IEnvEntryT::Impl(impl_a)),
        ];
        result.extend(more_entries_combined);
        result.extend(forwarder_methods);
        result
    }
/*
  override def getInterfaceSiblingEntries(interfaceName: IdT[INameT], interfaceA: InterfaceA): Vector[(IdT[INameT], IEnvEntry)] = {
    if (interfaceA.attributes.contains(SealedS)) {
      return Vector()
    }

    val memberRunes =
      interfaceA.internalMethods.zipWithIndex.map({ case (method, index) =>
        RuneUsage(RangeS(method.range.begin, method.range.begin), AnonymousSubstructMemberRuneS(interfaceA.name, method.name))
      })
    val members =
      interfaceA.internalMethods.zip(memberRunes).zipWithIndex.map({ case ((method, rune), index) =>
        NormalStructMemberS(method.range, interner.intern(StrI(index.toString)), FinalP, rune)
      })

    val structNameS = interner.intern(AnonymousSubstructTemplateNameS(interfaceA.name))
    val structNameT = interfaceName.copy(localName = nameTranslator.translateNameStep(structNameS))
    val structA = makeStruct(interfaceA, memberRunes, members, structNameS)

    val moreEntries =
//        interfaceFreeMacro.getInterfaceSiblingEntries(interfaceName, interfaceA) ++
        structConstructorMacro.getStructSiblingEntries(structNameT, structA) ++
        structDropMacro.getStructSiblingEntries(structNameT, structA)// ++
        //structFreeMacro.getStructSiblingEntries(structNameT, structA)

    val forwarderMethods =
      interfaceA.internalMethods.zip(memberRunes).zipWithIndex.map({ case ((method, rune), methodIndex) =>
        val name = structNameT.copy(localName = nameTranslator.translateGenericFunctionName(method.name))
        (name, FunctionEnvEntry(makeForwarderFunction(structNameS, interfaceA, structA, method, methodIndex)))
      })

    val rules =
      //structA.headerRules ++
      // structA.memberRules ++
      Vector(
        LookupSR(
          structA.range,
          RuneUsage(structA.range, AnonymousSubstructTemplateRuneS()),
          structA.name.getImpreciseName(interner)),
        CallSR(
          structA.range,
          RuneUsage(structA.range, AnonymousSubstructKindRuneS()),
          RuneUsage(structA.range, AnonymousSubstructTemplateRuneS()),
          structA.genericParameters.map(_.rune).toVector),
        LookupSR(
          interfaceA.range,
          RuneUsage(interfaceA.range, AnonymousSubstructParentInterfaceTemplateRuneS()),
          interfaceA.name.getImpreciseName(interner)),
        CallSR(
          interfaceA.range,
          RuneUsage(interfaceA.range, AnonymousSubstructParentInterfaceKindRuneS()),
          RuneUsage(interfaceA.range, AnonymousSubstructParentInterfaceTemplateRuneS()),
          interfaceA.genericParameters.map(_.rune).toVector))
    val runeToType =
      structA.genericParameters.map(_.rune.rune)
        .map(rune => rune -> vassertSome(structA.headerRuneToType.get(rune)))
        .toMap ++
//      structA.headerRuneToType ++
//       structA.membersRuneToType ++
      Vector(
        (AnonymousSubstructKindRuneS() -> KindTemplataType()),
        (AnonymousSubstructTemplateRuneS() -> structA.tyype),
        (AnonymousSubstructParentInterfaceKindRuneS() -> KindTemplataType()),
        (AnonymousSubstructParentInterfaceTemplateRuneS() -> interfaceA.tyype))
    val structKindRuneS = RuneUsage(interfaceA.range, AnonymousSubstructKindRuneS())
    val interfaceKindRuneS = RuneUsage(interfaceA.range, AnonymousSubstructParentInterfaceKindRuneS())

    val implNameS = interner.intern(AnonymousSubstructImplDeclarationNameS(interfaceA.name))
//    val implImpreciseNameS = interner.intern(ImplImpreciseNameS(RuleScout.getRuneKindTemplate(rules, structKindRuneS.rune)))

    val implA =
      ImplA(
        interfaceA.range,
        implNameS,
//        // Just getting the template name (or the kind name if not template), see INSHN.
//        implImpreciseNameS,
        structA.genericParameters,
        rules.toVector,
        runeToType,
        structKindRuneS,
        structA.name.getImpreciseName(interner),
        interfaceKindRuneS,
        interfaceA.name.getImpreciseName(interner))
    val implNameT = structNameT.copy(localName = nameTranslator.translateNameStep(implA.name))
//    val implSiblingEntries =
//      implDropMacro.getImplSiblingEntries(implNameT, implA)

    Vector[(IdT[INameT], IEnvEntry)](
      (structNameT, StructEnvEntry(structA)),
      (implNameT, ImplEnvEntry(implA))) ++
      moreEntries ++
      forwarderMethods
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn map_runes_anonymous_interface(
        &self,
        rule: IRulexSR<'s>,
        func: impl Fn(IRuneS<'s>) -> IRuneS<'s>,
    ) -> IRulexSR<'s> {
        match rule {
            IRulexSR::Lookup(x) => IRulexSR::Lookup(LookupSR {
                range: x.range,
                rune: RuneUsage { range: x.rune.range, rune: func(x.rune.rune) },
                name: x.name,
            }),
            IRulexSR::MaybeCoercingLookup(_) => panic!("implement: map_runes_anonymous_interface MaybeCoercingLookup"),
            IRulexSR::RuneParentEnvLookup(_) => panic!("implement: map_runes_anonymous_interface RuneParentEnvLookup"),
            IRulexSR::Equals(_) => panic!("implement: map_runes_anonymous_interface Equals"),
            IRulexSR::DefinitionCoordIsa(_) => panic!("implement: map_runes_anonymous_interface DefinitionCoordIsa"),
            IRulexSR::CallSiteCoordIsa(_) => panic!("implement: map_runes_anonymous_interface CallSiteCoordIsa"),
            IRulexSR::KindComponents(_) => panic!("implement: map_runes_anonymous_interface KindComponents"),
            IRulexSR::CoordComponents(_) => panic!("implement: map_runes_anonymous_interface CoordComponents"),
            IRulexSR::PrototypeComponents(_) => panic!("implement: map_runes_anonymous_interface PrototypeComponents"),
            IRulexSR::Resolve(_) => panic!("implement: map_runes_anonymous_interface Resolve"),
            IRulexSR::CallSiteFunc(_) => panic!("implement: map_runes_anonymous_interface CallSiteFunc"),
            IRulexSR::DefinitionFunc(_) => panic!("implement: map_runes_anonymous_interface DefinitionFunc"),
            IRulexSR::OneOf(_) => panic!("implement: map_runes_anonymous_interface OneOf"),
            IRulexSR::IsConcrete(_) => panic!("implement: map_runes_anonymous_interface IsConcrete"),
            IRulexSR::IsInterface(_) => panic!("implement: map_runes_anonymous_interface IsInterface"),
            IRulexSR::IsStruct(_) => panic!("implement: map_runes_anonymous_interface IsStruct"),
            IRulexSR::CoerceToCoord(x) => IRulexSR::CoerceToCoord(CoerceToCoordSR {
                range: x.range,
                coord_rune: RuneUsage { range: x.coord_rune.range, rune: func(x.coord_rune.rune) },
                kind_rune: RuneUsage { range: x.kind_rune.range, rune: func(x.kind_rune.rune) },
            }),
            IRulexSR::Literal(_) => panic!("implement: map_runes_anonymous_interface Literal"),
            IRulexSR::Augment(x) => {
                IRulexSR::Augment(AugmentSR {
                    range: x.range,
                    result_rune: RuneUsage { range: x.result_rune.range, rune: func(x.result_rune.rune) },
                    ownership: x.ownership,
                    inner_rune: RuneUsage { range: x.inner_rune.range, rune: func(x.inner_rune.rune) },
                })
            }
            IRulexSR::MaybeCoercingCall(_) => panic!("implement: map_runes_anonymous_interface MaybeCoercingCall"),
            IRulexSR::Call(x) => {
                let new_args: Vec<RuneUsage<'s>> = x.args.iter()
                    .map(|ru| RuneUsage { range: ru.range, rune: func(ru.rune) })
                    .collect();
                IRulexSR::Call(CallSR {
                    range: x.range,
                    result_rune: RuneUsage { range: x.result_rune.range, rune: func(x.result_rune.rune) },
                    template_rune: RuneUsage { range: x.template_rune.range, rune: func(x.template_rune.rune) },
                    args: self.scout_arena.alloc_slice_from_vec(new_args),
                })
            }
            IRulexSR::Pack(_) => panic!("implement: map_runes_anonymous_interface Pack"),
            IRulexSR::RefListCompoundMutability(_) => panic!("implement: map_runes_anonymous_interface RefListCompoundMutability"),
            other => panic!("vimpl: map_runes_anonymous_interface {:?}", other),
        }
    }
/*
  private def mapRunes(rule: IRulexSR, func: IRuneS => IRuneS): IRulexSR = {
    rule match {
      case LookupSR(range, RuneUsage(a, rune), name) => LookupSR(range, RuneUsage(a, func(rune)), name)
      case MaybeCoercingLookupSR(range, RuneUsage(a, rune), name) => LookupSR(range, RuneUsage(a, func(rune)), name)
      case RuneParentEnvLookupSR(range, RuneUsage(a, rune)) => RuneParentEnvLookupSR(range, RuneUsage(a, func(rune)))
      case EqualsSR(range, RuneUsage(a, left), RuneUsage(b, right)) => EqualsSR(range, RuneUsage(a, func(left)), RuneUsage(b, func(right)))
      case DefinitionCoordIsaSR(range, RuneUsage(z, result), RuneUsage(a, sub), RuneUsage(b, suuper)) => DefinitionCoordIsaSR(range, RuneUsage(z, func(result)), RuneUsage(a, func(sub)), RuneUsage(b, func(suuper)))
      case CallSiteCoordIsaSR(range, maybeResult, RuneUsage(a, sub), RuneUsage(b, suuper)) => {
        CallSiteCoordIsaSR(
          range,
          maybeResult.map({ case RuneUsage(z, result) => RuneUsage(z, func(result)) }),
          RuneUsage(a, func(sub)),
          RuneUsage(b, func(suuper)))
      }
      case KindComponentsSR(range, RuneUsage(a, resultRune), RuneUsage(b, mutabilityRune)) => KindComponentsSR(range, RuneUsage(a, func(resultRune)), RuneUsage(b, func(mutabilityRune)))
      case CoordComponentsSR(range, RuneUsage(a, resultRune), RuneUsage(b, ownershipRune), RuneUsage(c, kindRune)) => CoordComponentsSR(range, RuneUsage(a, func(resultRune)), RuneUsage(b, func(ownershipRune)), RuneUsage(c, func(kindRune)))
      case PrototypeComponentsSR(range, RuneUsage(a, resultRune), RuneUsage(b, paramsRune), RuneUsage(c, returnRune)) => PrototypeComponentsSR(range, RuneUsage(a, func(resultRune)), RuneUsage(b, func(paramsRune)), RuneUsage(c, func(returnRune)))
      case ResolveSR(range, RuneUsage(a, resultRune), name, RuneUsage(b, paramsListRune), RuneUsage(c, returnRune)) => ResolveSR(range, RuneUsage(a, func(resultRune)), name, RuneUsage(b, func(paramsListRune)), RuneUsage(c, func(returnRune)))
      case CallSiteFuncSR(range, RuneUsage(a, resultRune), name, RuneUsage(b, paramsListRune), RuneUsage(c, returnRune)) => CallSiteFuncSR(range, RuneUsage(a, func(resultRune)), name, RuneUsage(b, func(paramsListRune)), RuneUsage(c, func(returnRune)))
      case DefinitionFuncSR(range, RuneUsage(a, resultRune), name, RuneUsage(b, paramsListRune), RuneUsage(c, returnRune)) => DefinitionFuncSR(range, RuneUsage(a, func(resultRune)), name, RuneUsage(b, func(paramsListRune)), RuneUsage(c, func(returnRune)))
      case OneOfSR(range, RuneUsage(a, rune), literals) => OneOfSR(range, RuneUsage(a, func(rune)), literals)
      case IsConcreteSR(range, RuneUsage(a, rune)) => IsConcreteSR(range, RuneUsage(a, func(rune)))
      case IsInterfaceSR(range, RuneUsage(a, rune)) => IsInterfaceSR(range, RuneUsage(a, func(rune)))
      case IsStructSR(range, RuneUsage(a, rune)) => IsStructSR(range, RuneUsage(a, func(rune)))
      case CoerceToCoordSR(range, RuneUsage(a, coordRune), RuneUsage(b, kindRune)) => CoerceToCoordSR(range, RuneUsage(a, func(coordRune)), RuneUsage(b, func(kindRune)))
      case LiteralSR(range, RuneUsage(a, rune), literal) => LiteralSR(range, RuneUsage(a, func(rune)), literal)
      case AugmentSR(range, RuneUsage(a, resultRune), ownership, RuneUsage(b, innerRune)) => AugmentSR(range, RuneUsage(a, func(resultRune)), ownership, RuneUsage(b, func(innerRune)))
      case MaybeCoercingCallSR(range, RuneUsage(a, resultRune), RuneUsage(b, templateRune), args) => MaybeCoercingCallSR(range, RuneUsage(a, func(resultRune)), RuneUsage(b, func(templateRune)), args.map({ case RuneUsage(c, rune) => RuneUsage(c, func(rune)) }))
      case CallSR(range, RuneUsage(a, resultRune), RuneUsage(b, templateRune), args) => CallSR(range, RuneUsage(a, func(resultRune)), RuneUsage(b, func(templateRune)), args.map({ case RuneUsage(c, rune) => RuneUsage(c, func(rune)) }))
      case PackSR(range, RuneUsage(a, resultRune), members) => PackSR(range, RuneUsage(a, resultRune), members.map({ case RuneUsage(c, rune) => RuneUsage(c, func(rune)) }))
//      case StaticSizedArraySR(range, RuneUsage(a, resultRune), RuneUsage(b, mutabilityRune), RuneUsage(c, variabilityRune), RuneUsage(d, sizeRune), RuneUsage(e, elementRune)) => StaticSizedArraySR(range, RuneUsage(a, func(resultRune)), RuneUsage(b, func(mutabilityRune)), RuneUsage(c, func(variabilityRune)), RuneUsage(d, func(sizeRune)), RuneUsage(e, func(elementRune)))
//      case RuntimeSizedArraySR(range, RuneUsage(a, resultRune), RuneUsage(b, mutabilityRune), RuneUsage(c, elementRune)) => RuntimeSizedArraySR(range, RuneUsage(a, func(resultRune)), RuneUsage(b, func(mutabilityRune)), RuneUsage(c, func(elementRune)))
      case RefListCompoundMutabilitySR(range, RuneUsage(a, resultRune), RuneUsage(b, coordListRune)) => RefListCompoundMutabilitySR(range, RuneUsage(a, func(resultRune)), RuneUsage(b, func(coordListRune)))
      case other => vimpl(other)
    }
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn inherited_method_rune_anonymous_interface(
        &self,
        interface_a: &'s InterfaceA<'s>,
        method: &'s FunctionA<'s>,
        rune: IRuneS<'s>,
    ) -> IRuneS<'s> {
        self.scout_arena.intern_rune(IRuneValS::AnonymousSubstructMethodInheritedRune(
            AnonymousSubstructMethodInheritedRuneValS {
                interface: *interface_a.name,
                method: method.name,
                inner: rune,
            }))
    }
/*
  // These are how the forwarder function refers to runes from the abstract function it's overriding. After all, the
  // forwarder function copies all the runes and rules from the abstract function, so we rename them here to avoid any
  // weird collisions.
  private def inheritedMethodRune(interfaceA: InterfaceA, method: FunctionA, rune: IRuneS): IRuneS = {
    AnonymousSubstructMethodInheritedRuneS(interfaceA.name, method.name, rune)
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn make_struct_anonymous_interface(
        &self,
        interface_a: &'s InterfaceA<'s>,
        member_runes: &[RuneUsage<'s>],
        members: &[NormalStructMemberS<'s>],
        struct_template_name_s: AnonymousSubstructTemplateNameS<'s>,
    ) -> &'s StructA<'s> {

        let range = |n: i32| RangeS::internal(self.scout_arena, n);
        let use_rune = |n: i32, rune: IRuneS<'s>| RuneUsage { range: range(n), rune };

        let mut rules_builder: Vec<IRulexSR<'s>> = Vec::new();
        let mut rune_to_type: Vec<(IRuneS<'s>, ITemplataType<'s>)> = Vec::new();

        for rule in interface_a.rules.iter() {
            rules_builder.push(*rule);
        }

        for (rune, tyype) in interface_a.rune_to_type.iter() {
            rune_to_type.push((*rune, *tyype));
        }
        for mr in member_runes.iter() {
            rune_to_type.push((mr.rune, ITemplataType::CoordTemplataType(CoordTemplataType {})));
        }

        let void_kind_rune = self.scout_arena.intern_rune(IRuneValS::AnonymousSubstructVoidKindRune(AnonymousSubstructVoidKindRuneS {}));
        rune_to_type.push((void_kind_rune, ITemplataType::KindTemplataType(KindTemplataType {})));
        let void_imprecise_name = self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: self.keywords.void }));
        rules_builder.push(IRulexSR::Lookup(LookupSR {
            range: range(-1672147),
            rune: use_rune(-64002, void_kind_rune),
            name: void_imprecise_name,
        }));

        let void_coord_rune = self.scout_arena.intern_rune(IRuneValS::AnonymousSubstructVoidCoordRune(AnonymousSubstructVoidCoordRuneS {}));
        rune_to_type.push((void_coord_rune, ITemplataType::CoordTemplataType(CoordTemplataType {})));
        rules_builder.push(IRulexSR::CoerceToCoord(CoerceToCoordSR {
            range: range(-1672147),
            coord_rune: use_rune(-64002, void_coord_rune),
            kind_rune: use_rune(-64002, void_kind_rune),
        }));

        let mut struct_generic_params: Vec<&'s GenericParameterS<'s>> = Vec::new();
        for gp in interface_a.generic_parameters.iter() {
            struct_generic_params.push(*gp);
        }
        for mr in member_runes.iter() {
            let gp = self.scout_arena.alloc(GenericParameterS {
                range: mr.range,
                rune: *mr,
                tyype: IGenericParameterTypeS::CoordGenericParameterType(
                    CoordGenericParameterTypeS {
                        coord_region: None,
                        kind_mutable: true,
                        region_mutable: false,
                    }),
                default: None,
            });
            struct_generic_params.push(gp);
        }

        use crate::postparsing::names::{
            AnonymousSubstructMethodSelfBorrowCoordRuneS,
            AnonymousSubstructMethodSelfOwnCoordRuneS,
            AnonymousSubstructFunctionBoundParamsListRuneS,
            AnonymousSubstructFunctionInterfaceTemplateRuneS,
            AnonymousSubstructFunctionInterfaceKindRuneS,
            AnonymousSubstructFunctionBoundPrototypeRuneS,
            AnonymousSubstructDropBoundParamsListRuneS,
            AnonymousSubstructDropBoundPrototypeRuneS,
        };
        for ((internal_method, member_rune), _method_index) in
            interface_a.internal_methods.iter().zip(member_runes.iter()).zip(0i32..) {
            let internal_method = *internal_method;
            for (method_rune, tyype) in internal_method.rune_to_type.iter() {
                let inherited = self.inherited_method_rune_anonymous_interface(interface_a, internal_method, *method_rune);
                rune_to_type.push((inherited, *tyype));
            }
            for rule in internal_method.rules.iter() {
                let mapped = self.map_runes_anonymous_interface(*rule, |method_rune| {
                    self.inherited_method_rune_anonymous_interface(interface_a, internal_method, method_rune)
                });
                rules_builder.push(mapped);
            }

            let original_ret_rune = internal_method.maybe_ret_coord_rune.unwrap();
            let return_rune = RuneUsage {
                range: original_ret_rune.range,
                rune: self.inherited_method_rune_anonymous_interface(interface_a, internal_method, original_ret_rune.rune),
            };

            // __call bound block
            {
                let self_borrow_coord_rune_s = self.scout_arena.intern_rune(IRuneValS::AnonymousSubstructMethodSelfBorrowCoordRune(
                    AnonymousSubstructMethodSelfBorrowCoordRuneS {
                        interface: *interface_a.name,
                        method: internal_method.name,
                    }));
                rune_to_type.push((self_borrow_coord_rune_s, ITemplataType::CoordTemplataType(CoordTemplataType {})));
                rules_builder.push(IRulexSR::Augment(AugmentSR {
                    range: internal_method.range,
                    result_rune: RuneUsage { range: internal_method.range, rune: self_borrow_coord_rune_s },
                    ownership: Some(OwnershipP::Borrow),
                    inner_rune: *member_rune,
                }));

                let mut param_runes: Vec<RuneUsage<'s>> = Vec::new();
                for param in internal_method.params.iter() {
                    match param.virtuality {
                        None => {
                            param_runes.push(RuneUsage {
                                range: param.pattern.range,
                                rune: self.inherited_method_rune_anonymous_interface(
                                    interface_a, internal_method, param.pattern.coord_rune.unwrap().rune),
                            });
                        }
                        Some(_) => {
                            param_runes.push(RuneUsage {
                                range: param.pattern.range,
                                rune: self_borrow_coord_rune_s,
                            });
                        }
                    }
                }
                let method_params_list_rune = RuneUsage {
                    range: internal_method.range,
                    rune: self.scout_arena.intern_rune(IRuneValS::AnonymousSubstructFunctionBoundParamsListRune(
                        AnonymousSubstructFunctionBoundParamsListRuneS {
                            interface: *interface_a.name,
                            method: internal_method.name,
                        })),
                };
                let param_runes_slice = self.scout_arena.alloc_slice_from_vec(param_runes);
                rules_builder.push(IRulexSR::Pack(PackSR {
                    range: internal_method.range,
                    result_rune: method_params_list_rune,
                    members: param_runes_slice,
                }));
                let coord_type_ref = self.scout_arena.alloc(ITemplataType::CoordTemplataType(CoordTemplataType {}));
                rune_to_type.push((method_params_list_rune.rune, ITemplataType::PackTemplataType(PackTemplataType { element_type: coord_type_ref })));

                let interface_params: Vec<&'s ParameterS<'s>> = internal_method.params.iter()
                    .filter(|p| p.virtuality.is_some())
                    .collect();
                assert_eq!(interface_params.len(), 1, "vassertOne");
                let interface_param = interface_params[0];
                let original_interface_coord_rune = interface_param.pattern.coord_rune.unwrap().rune;
                let interface_coord_rune = RuneUsage {
                    range: interface_param.range,
                    rune: self.inherited_method_rune_anonymous_interface(
                        interface_a, internal_method, interface_param.pattern.coord_rune.unwrap().rune),
                };
                rune_to_type.push((interface_coord_rune.rune, ITemplataType::CoordTemplataType(CoordTemplataType {})));

                let mut collected: Vec<IRuneS<'s>> = Vec::new();
                for rule in internal_method.rules.iter() {
                    match rule {
                        IRulexSR::Augment(a) if a.result_rune.rune.ptr_eq(&original_interface_coord_rune) => {
                            collected.push(a.inner_rune.rune);
                        }
                        _ => {}
                    }
                }
                assert_eq!(collected.len(), 1, "vassertOne");
                let method_interface_coord_rune = RuneUsage {
                    range: interface_param.range,
                    rune: self.inherited_method_rune_anonymous_interface(interface_a, internal_method, collected[0]),
                };

                let method_interface_template_rune = RuneUsage {
                    range: interface_param.range,
                    rune: self.scout_arena.intern_rune(IRuneValS::AnonymousSubstructFunctionInterfaceTemplateRune(
                        AnonymousSubstructFunctionInterfaceTemplateRuneS {
                            interface: *interface_a.name,
                            method: internal_method.name,
                        })),
                };
                rune_to_type.push((method_interface_template_rune.rune, ITemplataType::TemplateTemplataType(interface_a.tyype)));

                let method_interface_kind_rune = RuneUsage {
                    range: interface_param.range,
                    rune: self.scout_arena.intern_rune(IRuneValS::AnonymousSubstructFunctionInterfaceKindRune(
                        AnonymousSubstructFunctionInterfaceKindRuneS {
                            interface: *interface_a.name,
                            method: internal_method.name,
                        })),
                };
                rune_to_type.push((method_interface_kind_rune.rune, ITemplataType::KindTemplataType(KindTemplataType {})));

                rules_builder.push(IRulexSR::Lookup(LookupSR {
                    range: interface_param.range,
                    rune: method_interface_template_rune,
                    name: interface_a.name.get_imprecise_name(self.scout_arena),
                }));
                let generic_param_runes: Vec<RuneUsage<'s>> = interface_a.generic_parameters.iter().map(|gp| gp.rune).collect();
                let generic_param_runes_slice = self.scout_arena.alloc_slice_from_vec(generic_param_runes);
                rules_builder.push(IRulexSR::Call(CallSR {
                    range: interface_param.range,
                    result_rune: method_interface_kind_rune,
                    template_rune: method_interface_template_rune,
                    args: generic_param_runes_slice,
                }));
                rules_builder.push(IRulexSR::CoerceToCoord(CoerceToCoordSR {
                    range: interface_param.range,
                    coord_rune: method_interface_coord_rune,
                    kind_rune: method_interface_kind_rune,
                }));

                let method_prototype_rune = RuneUsage {
                    range: internal_method.range,
                    rune: self.scout_arena.intern_rune(IRuneValS::AnonymousSubstructFunctionBoundPrototypeRune(
                        AnonymousSubstructFunctionBoundPrototypeRuneS {
                            interface: *interface_a.name,
                            method: internal_method.name,
                        })),
                };
                rules_builder.push(IRulexSR::DefinitionFunc(DefinitionFuncSR {
                    range: internal_method.range,
                    result_rune: method_prototype_rune,
                    name: self.keywords.underscores_call,
                    params_list_rune: method_params_list_rune,
                    return_rune,
                }));
                rules_builder.push(IRulexSR::CallSiteFunc(CallSiteFuncSR {
                    range: internal_method.range,
                    prototype_rune: method_prototype_rune,
                    name: self.keywords.underscores_call,
                    params_list_rune: method_params_list_rune,
                    return_rune,
                }));
                rules_builder.push(IRulexSR::Resolve(ResolveSR {
                    range: internal_method.range,
                    result_rune: method_prototype_rune,
                    name: self.keywords.underscores_call,
                    params_list_rune: method_params_list_rune,
                    return_rune,
                }));
                rune_to_type.push((method_prototype_rune.rune, ITemplataType::PrototypeTemplataType(PrototypeTemplataType {})));
            }

            // drop bound block
            {
                let self_own_coord_rune_s = self.scout_arena.intern_rune(IRuneValS::AnonymousSubstructMethodSelfOwnCoordRune(
                    AnonymousSubstructMethodSelfOwnCoordRuneS {
                        interface: *interface_a.name,
                        method: internal_method.name,
                    }));
                rune_to_type.push((self_own_coord_rune_s, ITemplataType::CoordTemplataType(CoordTemplataType {})));
                rules_builder.push(IRulexSR::Augment(AugmentSR {
                    range: internal_method.range,
                    result_rune: RuneUsage { range: internal_method.range, rune: self_own_coord_rune_s },
                    ownership: Some(OwnershipP::Own),
                    inner_rune: *member_rune,
                }));

                let drop_params_list_rune = RuneUsage {
                    range: internal_method.range,
                    rune: self.scout_arena.intern_rune(IRuneValS::AnonymousSubstructDropBoundParamsListRune(
                        AnonymousSubstructDropBoundParamsListRuneS {
                            interface: *interface_a.name,
                            method: internal_method.name,
                        })),
                };
                let drop_params_slice = self.scout_arena.alloc_slice_from_vec(vec![RuneUsage {
                    range: internal_method.range,
                    rune: self_own_coord_rune_s,
                }]);
                rules_builder.push(IRulexSR::Pack(PackSR {
                    range: internal_method.range,
                    result_rune: drop_params_list_rune,
                    members: drop_params_slice,
                }));
                let coord_type_ref2 = self.scout_arena.alloc(ITemplataType::CoordTemplataType(CoordTemplataType {}));
                rune_to_type.push((drop_params_list_rune.rune, ITemplataType::PackTemplataType(PackTemplataType { element_type: coord_type_ref2 })));

                let drop_prototype_rune = RuneUsage {
                    range: internal_method.range,
                    rune: self.scout_arena.intern_rune(IRuneValS::AnonymousSubstructDropBoundPrototypeRune(
                        AnonymousSubstructDropBoundPrototypeRuneS {
                            interface: *interface_a.name,
                            method: internal_method.name,
                        })),
                };
                let void_coord_ru = RuneUsage { range: internal_method.range, rune: void_coord_rune };
                rules_builder.push(IRulexSR::DefinitionFunc(DefinitionFuncSR {
                    range: internal_method.range,
                    result_rune: drop_prototype_rune,
                    name: self.keywords.drop,
                    params_list_rune: drop_params_list_rune,
                    return_rune: void_coord_ru,
                }));
                rules_builder.push(IRulexSR::CallSiteFunc(CallSiteFuncSR {
                    range: internal_method.range,
                    prototype_rune: drop_prototype_rune,
                    name: self.keywords.drop,
                    params_list_rune: drop_params_list_rune,
                    return_rune: void_coord_ru,
                }));
                rules_builder.push(IRulexSR::Resolve(ResolveSR {
                    range: internal_method.range,
                    result_rune: drop_prototype_rune,
                    name: self.keywords.drop,
                    params_list_rune: drop_params_list_rune,
                    return_rune: void_coord_ru,
                }));
                rune_to_type.push((drop_prototype_rune.rune, ITemplataType::PrototypeTemplataType(PrototypeTemplataType {})));
            }
        }

        let member_coord_types: Vec<ITemplataType<'s>> = member_runes.iter()
            .map(|_mr| ITemplataType::CoordTemplataType(CoordTemplataType {}))
            .collect();
        let mut param_types: Vec<ITemplataType<'s>> = interface_a.tyype.param_types.to_vec();
        param_types.extend(member_coord_types);
        let param_types_slice = self.scout_arena.alloc_slice_from_vec(param_types);
        let kind_type = self.scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {}));
        let tyype = TemplateTemplataType {
            param_types: param_types_slice,
            return_type: kind_type,
        };

        let header_rune_to_type = self.scout_arena.alloc_index_map_from_iter(rune_to_type);
        let header_rules_slice = self.scout_arena.alloc_slice_from_vec(rules_builder);
        let members_rune_to_type = self.scout_arena.alloc_index_map::<IRuneS<'s>, ITemplataType<'s>>();
        let member_rules_slice: &'s [IRulexSR<'s>] = self.scout_arena.alloc_slice_from_vec(vec![]);
        let generic_params_slice = self.scout_arena.alloc_slice_from_vec(struct_generic_params);
        let attributes_slice: &'s [ICitizenAttributeS<'s>] = self.scout_arena.alloc_slice_from_vec(vec![]);
        let members_slice: &'s [IStructMemberS<'s>] = self.scout_arena.alloc_slice_from_vec(
            members.iter().map(|m| IStructMemberS::NormalStructMember(*m)).collect::<Vec<_>>());

        let struct_a = StructA::new(
            interface_a.range,
            IStructDeclarationNameS::AnonymousSubstructTemplateName(
                *self.scout_arena.alloc(struct_template_name_s)),
            attributes_slice,
            false,
            interface_a.mutability_rune,
            interface_a.maybe_predicted_mutability,
            tyype,
            generic_params_slice,
            header_rune_to_type,
            header_rules_slice,
            members_rune_to_type,
            member_rules_slice,
            members_slice,
            &[],
        );
        self.scout_arena.alloc(struct_a)
    }
/*
  private def makeStruct(interfaceA: InterfaceA, memberRunes: Vector[RuneUsage], members: Vector[NormalStructMemberS], structTemplateNameS: AnonymousSubstructTemplateNameS) = {
    def range(n: Int) = RangeS.internal(interner, n)
    def use(n: Int, rune: IRuneS) = RuneUsage(range(n), rune)

    // For this interface:
    //
    //   #!DeriveInterfaceDrop
    //   sealed interface Bork<A Ref, B Ref> {
    //     func bork(virtual self &Bork<A Ref, B Ref>, a Opt<A>) B;
    //   }
    //
    // We're trying to make a struct with a bunch of callables:
    //
    //   #!DeriveStructDrop
    //   struct IBorkForwarder<A Ref, B Ref, Lam>
    //       where func drop(Lam)void, func __call(&Lam, Opt<A>)B {
    //     lam Lam;
    //   }

    val rulesBuilder = new Accumulator[IRulexSR]()
    val runeToType = mutable.HashMap[IRuneS, ITemplataType]()

    interfaceA.rules.foreach(x => rulesBuilder.add(x))

    runeToType ++= interfaceA.runeToType
    runeToType ++= memberRunes.map(_.rune -> CoordTemplataType())

    val voidKindRune = AnonymousSubstructVoidKindRuneS()
    runeToType.put(voidKindRune, KindTemplataType())
    rulesBuilder.add(LookupSR(range(-1672147),use(-64002, voidKindRune),interner.intern(CodeNameS(keywords.void))))
    val voidCoordRune = AnonymousSubstructVoidCoordRuneS()
    runeToType.put(voidCoordRune, CoordTemplataType())
    rulesBuilder.add(CoerceToCoordSR(range(-1672147),use(-64002, voidCoordRune),use(-64002, voidKindRune)))

    val structGenericParams =
      interfaceA.genericParameters ++
        memberRunes.map(mr => GenericParameterS(mr.range, mr, CoordGenericParameterTypeS(vregionmut(None), true, false), None))

    interfaceA.internalMethods.zip(memberRunes).zipWithIndex.foreach({ case ((internalMethod, memberRune), methodIndex) =>
      val methodRuneToType =
        internalMethod.runeToType.map({ case (methodRune, tyype) =>
          inheritedMethodRune(interfaceA, internalMethod, methodRune) -> tyype
        })
      runeToType ++= methodRuneToType
      val methodRules =
        internalMethod.rules.map(rule => mapRunes(rule, methodRune => {
          inheritedMethodRune(interfaceA, internalMethod, methodRune)
        }))
      rulesBuilder.addAll(methodRules)

      val returnRune = {
        val originalRetRune = vassertSome(internalMethod.maybeRetCoordRune)
        RuneUsage(
          originalRetRune.range,
          inheritedMethodRune(interfaceA, internalMethod, originalRetRune.rune))
      }

      // Now we make the __call bound, which involves figuring out the params and return runes and
      // assembling a call rule for it.
      {
        val selfBorrowCoordRuneS =
          AnonymousSubstructMethodSelfBorrowCoordRuneS(interfaceA.name, internalMethod.name)
        runeToType += selfBorrowCoordRuneS -> CoordTemplataType()
        rulesBuilder.add(
          AugmentSR(internalMethod.range, RuneUsage(internalMethod.range, selfBorrowCoordRuneS), Some(BorrowP), memberRune))

        val paramRunes =
          internalMethod.params.map({
            case ParameterS(_, None, _, AtomSP(range, name, coordRune, destructure)) => {
              RuneUsage(range, inheritedMethodRune(interfaceA, internalMethod, vassertSome(coordRune).rune))
            }
            case ParameterS(_, Some(_), _, AtomSP(range, name, coordRune, destructure)) => {
              RuneUsage(range, selfBorrowCoordRuneS)
            }
          })
        val methodParamsListRune =
          RuneUsage(internalMethod.range, AnonymousSubstructFunctionBoundParamsListRuneS(interfaceA.name, internalMethod.name))
        rulesBuilder.add(PackSR(internalMethod.range, methodParamsListRune, paramRunes.toVector))
        runeToType.put(methodParamsListRune.rune, PackTemplataType(CoordTemplataType()))

        // the struct runes are guaranteed to line up with the interface runes...
        // but not necessarily this function's runes.
        // we need to grab the owner

        // Let's say we had a:
        //
        //   func bork<X Ref, Y Ref>(virtual self &IBork<X, Y>, Opt<X>) Y;
        //
        // our bound will probably look like:
        //
        //   func __call(&Lam, Opt<A>)B
        //
        // we need to make a IBork<B, A> = IBork<X, Y> to connect those two worlds of runes.
        val interfaceParam =
          vassertOne(internalMethod.params.filter(_.virtuality.nonEmpty))
        val originalInterfaceCoordRune = vassertSome(interfaceParam.pattern.coordRune).rune
        val interfaceCoordRune =
          RuneUsage(interfaceParam.range, inheritedMethodRune(interfaceA, internalMethod, vassertSome(interfaceParam.pattern.coordRune).rune))
        runeToType.put(interfaceCoordRune.rune, CoordTemplataType())

        val methodInterfaceCoordRune =
          RuneUsage(
            interfaceParam.range,
            inheritedMethodRune(interfaceA, internalMethod,
              vassertOne(
                internalMethod.rules.collect({
                  case AugmentSR(_, resultRune, ownership, innerRune) if resultRune.rune == originalInterfaceCoordRune => {
                    innerRune.rune
                  }
                }))))

//        val methodInterfaceKindRune =
//          RuneUsage(interfaceParam.range, AnonymousSubstructFunctionInterfaceKindRune(interfaceA.name, internalMethod.name))
//        runeToType.put(methodInterfaceKindRune.rune, KindTemplataType())

//        val methodInterfaceOwnershipRune =
//          RuneUsage(interfaceParam.range, AnonymousSubstructFunctionInterfaceOwnershipRune(interfaceA.name, internalMethod.name))
//        runeToType.put(methodInterfaceOwnershipRune.rune, OwnershipTemplataType())

        val methodInterfaceTemplateRune =
          RuneUsage(interfaceParam.range, AnonymousSubstructFunctionInterfaceTemplateRune(interfaceA.name, internalMethod.name))
        runeToType.put(methodInterfaceTemplateRune.rune, interfaceA.tyype)

        val methodInterfaceKindRune =
          RuneUsage(interfaceParam.range, AnonymousSubstructFunctionInterfaceKindRune(interfaceA.name, internalMethod.name))
        runeToType.put(methodInterfaceKindRune.rune, KindTemplataType())

        rulesBuilder.add(
          LookupSR(interfaceParam.range, methodInterfaceTemplateRune, interfaceA.name.getImpreciseName(interner)))
//        rulesBuilder.add(
//          CoordComponentsSR(interfaceParam.range, interfaceCoordRune, methodInterfaceOwnershipRune, methodInterfaceKindRune))
        rulesBuilder.add(
          CallSR(interfaceParam.range, methodInterfaceKindRune, methodInterfaceTemplateRune, interfaceA.genericParameters.map(_.rune).toVector))
        rulesBuilder.add(
          CoerceToCoordSR(interfaceParam.range, methodInterfaceCoordRune, methodInterfaceKindRune))

        val methodPrototypeRune =
          RuneUsage(
            internalMethod.range,
            AnonymousSubstructFunctionBoundPrototypeRuneS(interfaceA.name, internalMethod.name))
        rulesBuilder.add(
          DefinitionFuncSR(
            internalMethod.range, methodPrototypeRune, keywords.underscoresCall, methodParamsListRune, returnRune))
        rulesBuilder.add(
          CallSiteFuncSR(
            internalMethod.range, methodPrototypeRune, keywords.underscoresCall, methodParamsListRune, returnRune))
        rulesBuilder.add(
          ResolveSR(
            internalMethod.range, methodPrototypeRune, keywords.underscoresCall, methodParamsListRune, returnRune))
        runeToType.put(methodPrototypeRune.rune, PrototypeTemplataType())
      }

      // Now we make the drop bound, which involves figuring out the params and return runes and
      // assembling a call rule for it.
      {
        val selfOwnCoordRuneS =
          AnonymousSubstructMethodSelfOwnCoordRuneS(interfaceA.name, internalMethod.name)
        runeToType += selfOwnCoordRuneS -> CoordTemplataType()
        rulesBuilder.add(
          AugmentSR(internalMethod.range, RuneUsage(internalMethod.range, selfOwnCoordRuneS), Some(OwnP), memberRune))

        val dropParamsListRune =
          RuneUsage(internalMethod.range, AnonymousSubstructDropBoundParamsListRuneS(interfaceA.name, internalMethod.name))
        rulesBuilder.add(
          PackSR(
            internalMethod.range,
            dropParamsListRune,
            Vector(RuneUsage(internalMethod.range, selfOwnCoordRuneS))))
        runeToType.put(dropParamsListRune.rune, PackTemplataType(CoordTemplataType()))

        val dropPrototypeRune =
          RuneUsage(
            internalMethod.range,
            AnonymousSubstructDropBoundPrototypeRuneS(interfaceA.name, internalMethod.name))
        rulesBuilder.add(
          DefinitionFuncSR(
            internalMethod.range, dropPrototypeRune, keywords.drop, dropParamsListRune, RuneUsage(internalMethod.range, voidCoordRune)))
        rulesBuilder.add(
          CallSiteFuncSR(
            internalMethod.range, dropPrototypeRune, keywords.drop, dropParamsListRune, RuneUsage(internalMethod.range, voidCoordRune)))
        rulesBuilder.add(
          ResolveSR(
            internalMethod.range, dropPrototypeRune, keywords.drop, dropParamsListRune, RuneUsage(internalMethod.range, voidCoordRune)))
        runeToType.put(dropPrototypeRune.rune, PrototypeTemplataType())
      }
    })

    StructA(
      interfaceA.range,
      structTemplateNameS,
      Vector(),
      false,
      interfaceA.mutabilityRune,
      interfaceA.maybePredictedMutability,
      TemplateTemplataType(
        interfaceA.tyype.paramTypes ++ memberRunes.map(_ => CoordTemplataType()),
        KindTemplataType()),
      structGenericParams,
      runeToType.toMap,
      rulesBuilder.buildArray(),
      Map(),
      Vector(),
      members,
      Vector())
  }

*/
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn make_forwarder_function_anonymous_interface(
        &self,
        struct_name_s: AnonymousSubstructTemplateNameS<'s>,
        interface: &'s InterfaceA<'s>,
        struct_: &'s StructA<'s>,
        method: &'s FunctionA<'s>,
        method_index: i32,
    ) -> &'s FunctionA<'s> {
        use crate::postparsing::names::{
            IRuneValS, INameValS, IVarNameS, SelfOwnershipRuneS, SelfKindRuneS, SelfCoordRuneS,
            SelfKindTemplateRuneS, AnonymousSubstructParentInterfaceTemplateRuneS,
            AnonymousSubstructTemplateImpreciseNameValS, IImpreciseNameValS,
            IFunctionDeclarationNameValS, ForwarderFunctionDeclarationNameValS,
            INameS, IRuneS,
        };

        let struct_type = struct_.tyype;
        let method_range = method.range;
        let attributes = method.attributes;
        let method_original_type = method.tyype;
        let method_original_identifying_runes: &'s [&'s GenericParameterS<'s>] = method.generic_parameters;
        let original_params = method.params;
        let method_original_rules = method.rules;

        // vassert(struct.genericParameters.map(_.rune).startsWith(methodOriginalIdentifyingRunes.map(_.rune)))
        let starts_with = struct_.generic_parameters.len() >= method_original_identifying_runes.len()
            && struct_.generic_parameters.iter().zip(method_original_identifying_runes.iter())
                .all(|(a, b)| a.rune.rune.ptr_eq(&b.rune.rune));
        assert!(starts_with, "vassert: struct.genericParameters.startsWith(methodOriginalIdentifyingRunes)");

        let mut generic_params_vec: Vec<&'s GenericParameterS<'s>> = Vec::new();
        for gp in struct_.generic_parameters.iter() {
            let new_rune = self.inherited_method_rune_anonymous_interface(interface, method, gp.rune.rune);
            generic_params_vec.push(self.scout_arena.alloc(GenericParameterS {
                range: gp.range,
                rune: RuneUsage { range: gp.rune.range, rune: new_rune },
                tyype: gp.tyype,
                default: gp.default,
            }));
        }

        let mut rune_to_type: Vec<(IRuneS<'s>, ITemplataType<'s>)> = Vec::new();
        let mut rules: Vec<IRulexSR<'s>> = Vec::new();

        for (method_rune, tyype) in method.rune_to_type.iter() {
            let inherited = self.inherited_method_rune_anonymous_interface(interface, method, *method_rune);
            rune_to_type.push((inherited, *tyype));
        }
        for rule in method_original_rules.iter() {
            let mapped = self.map_runes_anonymous_interface(*rule, |method_rune| {
                self.inherited_method_rune_anonymous_interface(interface, method, method_rune)
            });
            rules.push(mapped);
        }
        let original_ret_rune = method.maybe_ret_coord_rune.unwrap();
        let inherited_return_rune = RuneUsage {
            range: original_ret_rune.range,
            rune: self.inherited_method_rune_anonymous_interface(interface, method, original_ret_rune.rune),
        };

        for param in struct_.generic_parameters.iter() {
            let inh = self.inherited_method_rune_anonymous_interface(interface, method, param.rune.rune);
            rune_to_type.push((inh, param.tyype.tyype()));
        }

        let self_ownership_rune = self.scout_arena.intern_rune(IRuneValS::SelfOwnershipRune(SelfOwnershipRuneS {}));
        rune_to_type.push((self_ownership_rune, ITemplataType::OwnershipTemplataType(OwnershipTemplataType {})));
        let interface_kind_rune = self.scout_arena.intern_rune(IRuneValS::AnonymousSubstructParentInterfaceTemplateRune(AnonymousSubstructParentInterfaceTemplateRuneS {}));
        rune_to_type.push((interface_kind_rune, ITemplataType::KindTemplataType(KindTemplataType {})));
        let self_kind_rune = self.scout_arena.intern_rune(IRuneValS::SelfKindRune(SelfKindRuneS {}));
        rune_to_type.push((self_kind_rune, ITemplataType::KindTemplataType(KindTemplataType {})));
        let self_coord_rune = self.scout_arena.intern_rune(IRuneValS::SelfCoordRune(SelfCoordRuneS {}));
        rune_to_type.push((self_coord_rune, ITemplataType::CoordTemplataType(CoordTemplataType {})));
        let self_kind_template_rune = self.scout_arena.intern_rune(IRuneValS::SelfKindTemplateRune(SelfKindTemplateRuneS { loc: struct_.range.begin }));
        rune_to_type.push((self_kind_template_rune, ITemplataType::TemplateTemplataType(struct_type)));

        let mut abstract_param_index: i32 = -1;
        for (i, param) in original_params.iter().enumerate() {
            let is_abstract = match param.virtuality {
                Some(AbstractSP { .. }) => true,
                None => false,
            };
            if is_abstract {
                abstract_param_index = i as i32;
                break;
            }
        }
        assert!(abstract_param_index >= 0, "vassert: abstractParamIndex >= 0");
        let abstract_param = &original_params[abstract_param_index as usize];
        let abstract_param_range = abstract_param.pattern.range;
        let abstract_param_coord_rune = RuneUsage {
            range: abstract_param_range,
            rune: self.inherited_method_rune_anonymous_interface(
                interface, method, abstract_param.pattern.coord_rune.unwrap().rune),
        };
        rune_to_type.push((abstract_param_coord_rune.rune, ITemplataType::CoordTemplataType(CoordTemplataType {})));

        let destructuring_interface_rule = IRulexSR::CoordComponents(CoordComponentsSR {
            range: abstract_param_range,
            result_rune: abstract_param_coord_rune,
            ownership_rune: RuneUsage { range: abstract_param_range, rune: self_ownership_rune },
            kind_rune: RuneUsage { range: abstract_param_range, rune: interface_kind_rune },
        });
        rules.push(destructuring_interface_rule);

        let struct_interface_imprecise = struct_name_s.interface_name.get_imprecise_name(self.scout_arena);
        let lookup_struct_template_rule = IRulexSR::Lookup(LookupSR {
            range: abstract_param_range,
            rune: RuneUsage { range: abstract_param_range, rune: self_kind_template_rune },
            name: self.scout_arena.intern_imprecise_name(IImpreciseNameValS::AnonymousSubstructTemplateImpreciseName(
                AnonymousSubstructTemplateImpreciseNameValS { interface_imprecise_name: struct_interface_imprecise })),
        });
        rules.push(lookup_struct_template_rule);

        let gp_runes_vec: Vec<RuneUsage<'s>> = generic_params_vec.iter().map(|gp| gp.rune).collect();
        let gp_runes_slice = self.scout_arena.alloc_slice_from_vec(gp_runes_vec);
        let lookup_struct_rule = IRulexSR::Call(CallSR {
            range: abstract_param_range,
            result_rune: RuneUsage { range: abstract_param_range, rune: self_kind_rune },
            template_rune: RuneUsage { range: abstract_param_range, rune: self_kind_template_rune },
            args: gp_runes_slice,
        });
        rules.push(lookup_struct_rule);

        let assembling_struct_rule = IRulexSR::CoordComponents(CoordComponentsSR {
            range: abstract_param_range,
            result_rune: RuneUsage { range: abstract_param_range, rune: self_coord_rune },
            ownership_rune: RuneUsage { range: abstract_param_range, rune: self_ownership_rune },
            kind_rune: RuneUsage { range: abstract_param_range, rune: self_kind_rune },
        });
        rules.push(assembling_struct_rule);

        let mut new_params_vec: Vec<ParameterS<'s>> = Vec::new();
        for param in original_params.iter() {
            match param.virtuality {
                Some(_) => {
                    new_params_vec.push(ParameterS {
                        range: abstract_param_range,
                        virtuality: None,
                        pre_checked: false,
                        pattern: AtomSP {
                            range: abstract_param_range,
                            name: Some(CaptureS { name: IVarNameS::SelfName, mutate: false }),
                            coord_rune: Some(RuneUsage { range: abstract_param_coord_rune.range, rune: self_coord_rune }),
                            destructure: None,
                        },
                    });
                }
                None => {
                    let old_rune_usage = param.pattern.coord_rune.unwrap();
                    let new_rune = RuneUsage {
                        range: old_rune_usage.range,
                        rune: self.inherited_method_rune_anonymous_interface(interface, method, old_rune_usage.rune),
                    };
                    new_params_vec.push(ParameterS {
                        range: param.range,
                        virtuality: param.virtuality,
                        pre_checked: param.pre_checked,
                        pattern: AtomSP {
                            range: param.pattern.range,
                            name: param.pattern.name,
                            coord_rune: Some(new_rune),
                            destructure: param.pattern.destructure,
                        },
                    });
                }
            }
        }

        // Body: FunctionCallSE(DotSE(LocalLoad(self), index, false), args)
        let self_local_load = self.scout_arena.alloc(IExpressionSE::LocalLoad(LocalLoadSE {
            range: method_range,
            name: IVarNameS::SelfName,
            target_ownership: LoadAsP::Use,
        }));
        let dot_member = self.scout_arena.intern_str(&method_index.to_string());
        let callable_expr = self.scout_arena.alloc(IExpressionSE::Dot(DotSE {
            range: method_range,
            left: self_local_load,
            member: dot_member,
            borrow_container: false,
        }));

        let mut call_args: Vec<&'s IExpressionSE<'s>> = Vec::new();
        for (i, param) in new_params_vec.iter().enumerate() {
            if (i as i32) == abstract_param_index { continue; }
            let nm = param.pattern.name.unwrap().name;
            call_args.push(self.scout_arena.alloc(IExpressionSE::LocalLoad(LocalLoadSE {
                range: method_range,
                name: nm,
                target_ownership: LoadAsP::Use,
            })));
        }
        let call_args_slice = self.scout_arena.alloc_slice_from_vec(call_args);

        let new_body_expr = self.scout_arena.alloc(IExpressionSE::FunctionCall(FunctionCallSE {
            range: method_range,
            location: LocationInDenizen { path: &[] },
            callable_expr,
            arg_exprs: call_args_slice,
        }));

        let locals_vec: Vec<LocalS<'s>> = new_params_vec.iter().map(|p| {
            let nm = p.pattern.name.unwrap().name;
            LocalS {
                var_name: nm,
                self_borrowed: IVariableUseCertainty::NotUsed,
                self_moved: IVariableUseCertainty::Used,
                self_mutated: IVariableUseCertainty::NotUsed,
                child_borrowed: IVariableUseCertainty::NotUsed,
                child_moved: IVariableUseCertainty::NotUsed,
                child_mutated: IVariableUseCertainty::NotUsed,
            }
        }).collect();
        let locals_slice = self.scout_arena.alloc_slice_from_vec(locals_vec);
        let block_se = self.scout_arena.alloc(BlockSE {
            range: method_range,
            locals: locals_slice,
            expr: new_body_expr,
        });
        let body_se = self.scout_arena.alloc(BodySE {
            range: method_range,
            closured_names: self.scout_arena.alloc_slice_from_vec::<IVarNameS<'s>>(vec![]),
            block: block_se,
        });
        let body = IBodyS::CodeBody(CodeBodyS { body: body_se });

        // Forwarder name
        let forwarder_name = match self.scout_arena.intern_name(INameValS::FunctionDeclaration(
            IFunctionDeclarationNameValS::ForwarderFunctionDeclarationName(ForwarderFunctionDeclarationNameValS {
                inner: method.name,
                index: method_index,
            }))) {
            INameS::FunctionDeclaration(r) => *r,
            _ => panic!("vwat: intern_name returned non-FunctionDeclaration"),
        };

        // Tyype: param_types ++ struct.genericParameters.map(_ => CoordTemplataType()), return FunctionTemplataType
        let mut new_param_types: Vec<ITemplataType<'s>> = method_original_type.param_types.to_vec();
        for _ in struct_.generic_parameters.iter() {
            new_param_types.push(ITemplataType::CoordTemplataType(CoordTemplataType {}));
        }
        let new_param_types_slice = self.scout_arena.alloc_slice_from_vec(new_param_types);
        let return_type_ref = self.scout_arena.alloc(ITemplataType::FunctionTemplataType(FunctionTemplataType {}));
        let new_tyype = TemplateTemplataType { param_types: new_param_types_slice, return_type: return_type_ref };

        let new_params_slice = self.scout_arena.alloc_slice_from_vec(new_params_vec);
        let rules_slice = self.scout_arena.alloc_slice_from_vec(rules);
        let generic_params_slice = self.scout_arena.alloc_slice_from_vec(generic_params_vec);
        let rune_to_type_map = self.scout_arena.alloc_index_map_from_iter(rune_to_type);

        self.scout_arena.alloc(FunctionA::new(
            method_range,
            forwarder_name,
            attributes,
            new_tyype,
            generic_params_slice,
            rune_to_type_map,
            new_params_slice,
            Some(inherited_return_rune),
            rules_slice,
            body,
        ))
    }
/*
  private def makeForwarderFunction(
    structNameS: AnonymousSubstructTemplateNameS,
    interface: InterfaceA,
    struct: StructA,
    method: FunctionA,
    methodIndex: Int):
  FunctionA = {
    val structType = struct.tyype
    val FunctionA(methodRange, name, attributes, methodOriginalType, methodOriginalIdentifyingRunes, methodOriginalRuneToType, originalParams, maybeRetCoordRune, methodOriginalRules, body) = method

    vassert(struct.genericParameters.map(_.rune).startsWith(methodOriginalIdentifyingRunes.map(_.rune)))
    val genericParams =
      struct.genericParameters
          .map({ case GenericParameterS(range, RuneUsage(runeRange, rune), tyype, default) =>
            GenericParameterS(range, RuneUsage(runeRange, inheritedMethodRune(interface, method, rune)), tyype, default)
          })

    val runeToType = mutable.HashMap[IRuneS, ITemplataType]()
    val rules = new Accumulator[IRulexSR]()

    // First we're going to pull in all the rules and runes from the parent function. This *would* make our function
    // identical to the parent function, *if* our parameters used the same coord runes. We won't, we'll do something
    // different for the overriding param. Still, we'll need all these rules and calculations from our parent.
    val inheritedRuneToType =
      methodOriginalRuneToType.map({ case (methodRune, tyype) =>
        inheritedMethodRune(interface, method, methodRune) -> tyype
      })
    runeToType ++= inheritedRuneToType
    val inheritedMethodRules =
      methodOriginalRules.map(rule => mapRunes(rule, methodRune => {
        inheritedMethodRune(interface, method, methodRune)
      }))
    rules.addAll(inheritedMethodRules)
    val inheritedReturnRune = {
      val originalRetRune = vassertSome(method.maybeRetCoordRune)
      RuneUsage(
        originalRetRune.range,
        inheritedMethodRune(interface, method, originalRetRune.rune))
    }

    // Now we're going to pull in the struct, which we'll use instead of the interface for the overriding param coord
    // rune.
    runeToType ++= struct.genericParameters.map(param => inheritedMethodRune(interface, method, param.rune.rune) -> param.tyype.tyype)
    // We don't want to pull in all of their rules, we can already reach their bounds, see NBIFP.
    //   runeToType ++= struct.headerRuneToType
    //   runeToType ++= struct.membersRuneToType

    // Now let's destructure the interface coord, and make a new coord with the struct to use as the overriding param
    // coord rune instead.

    val selfOwnershipRune = SelfOwnershipRuneS()
    runeToType.put(selfOwnershipRune, OwnershipTemplataType())
    val interfaceKindRune = AnonymousSubstructParentInterfaceTemplateRuneS()
    runeToType.put(interfaceKindRune, KindTemplataType())
    val selfKindRune = SelfKindRuneS()
    runeToType.put(selfKindRune, KindTemplataType())
    val selfCoordRune = SelfCoordRuneS()
    runeToType.put(selfCoordRune, CoordTemplataType())
    val selfKindTemplateRune = SelfKindTemplateRuneS(struct.range.begin)
    runeToType.put(selfKindTemplateRune, structType)

    val abstractParamIndex =
      originalParams.indexWhere(param => {
        param.virtuality match {
          case Some(AbstractSP(_, _)) => true
          case _ => false
        }
      })
    vassert(abstractParamIndex >= 0)
    val abstractParam = originalParams(abstractParamIndex)
    val abstractParamRange = abstractParam.pattern.range
    val abstractParamCoordRune =
      RuneUsage(
        abstractParamRange,
        // This should be the same as one of the runes inherited above.
        inheritedMethodRune(interface, method, vassertSome(abstractParam.pattern.coordRune).rune)) // https://github.com/ValeLang/Vale/issues/370
    runeToType.put(abstractParamCoordRune.rune, CoordTemplataType())

    val destructuringInterfaceRule =
      CoordComponentsSR(
        abstractParamRange,
        abstractParamCoordRune,
        RuneUsage(abstractParamRange, selfOwnershipRune),
        RuneUsage(abstractParamRange, interfaceKindRune))

    rules.add(destructuringInterfaceRule)
    val lookupStructTemplateRule =
      LookupSR(
        abstractParamRange,
        RuneUsage(abstractParamRange, selfKindTemplateRune),
        interner.intern(AnonymousSubstructTemplateImpreciseNameS(structNameS.interfaceName.getImpreciseName(interner))))
    rules.add(lookupStructTemplateRule)
    val lookupStructRule =
      CallSR(
        abstractParamRange,
        RuneUsage(abstractParamRange, selfKindRune),
        RuneUsage(abstractParamRange, selfKindTemplateRune),
        genericParams.map(_.rune).toVector)
    rules.add(lookupStructRule)

    val assemblingStructRule =
      CoordComponentsSR(
        abstractParamRange,
        RuneUsage(abstractParamRange, selfCoordRune),
        RuneUsage(abstractParamRange, selfOwnershipRune),
        RuneUsage(abstractParamRange, selfKindRune))
    rules.add(assemblingStructRule)

    val newParams =
      originalParams.map({
        case ParameterS(_, Some(_), _, AtomSP(_, _, Some(_), _)) => {
          ParameterS(
            abstractParamRange,
            None, //Some(OverrideSP(abstractParamRange, RuneUsage(abstractParamCoordRune.range, AnonymousSubstructParentInterfaceTemplateRuneS()))),
            false,
            AtomSP(
              abstractParamRange,
              Some(CaptureS(interner.intern(SelfNameS()), false)),
              Some(RuneUsage(abstractParamCoordRune.range, selfCoordRune)),
              None))
        }
        case p @ ParameterS(_, None, _, a @ AtomSP(_, _, Some(RuneUsage(runeRange, oldRune)), _)) => {
          val rune = RuneUsage(runeRange, inheritedMethodRune(interface, method, oldRune))
          p.copy(pattern = a.copy(coordRune = Some(rune)))
        }
      })

    val newBody =
      FunctionCallSE(
        methodRange,
        vregionmut(LocationInDenizen(Vector())),
        DotSE(
          methodRange,
          LocalLoadSE(methodRange, interner.intern(SelfNameS()), UseP),
          interner.intern(StrI(methodIndex.toString)),
          false),
        // Params minus the abstract param
        (newParams.slice(0, abstractParamIndex) ++ newParams.slice(abstractParamIndex + 1, newParams.length))
          .map(param => vassertSome(param.pattern.name).name)
          .map(name => LocalLoadSE(methodRange, name, UseP)))

    FunctionA(
      methodRange,
      interner.intern(ForwarderFunctionDeclarationNameS(name, methodIndex)),
      attributes,
      TemplateTemplataType(
        methodOriginalType.paramTypes ++ struct.genericParameters.map(_ => CoordTemplataType()),
        FunctionTemplataType()),
      genericParams,
      runeToType.toMap,
      newParams,
      Some(inheritedReturnRune),
      rules.buildArray().toVector,
      CodeBodyS(
        BodySE(
          methodRange,
          Vector(),
          BlockSE(
            methodRange,
            newParams.map(param => vassertSome(param.pattern.name).name).map(LocalS(_, NotUsed, Used, NotUsed, NotUsed, NotUsed, NotUsed)),
            newBody))))
  }
}
*/
}
