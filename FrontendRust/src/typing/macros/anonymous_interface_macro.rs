use crate::higher_typing::ast::*;
use crate::postparsing::ast::NormalStructMemberS;
use crate::postparsing::names::{AnonymousSubstructTemplateNameS, IRuneS};
use crate::postparsing::rules::rules::{IRulexSR, RuneUsage};
use crate::typing::names::names::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::compiler::Compiler;

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
        use crate::postparsing::ast::{ICitizenAttributeS, SealedS, NormalStructMemberS};
        use crate::postparsing::names::{
            IRuneValS, AnonymousSubstructTemplateNameS, AnonymousSubstructImplDeclarationNameS,
            AnonymousSubstructTemplateRuneS, AnonymousSubstructKindRuneS,
            AnonymousSubstructParentInterfaceTemplateRuneS, AnonymousSubstructParentInterfaceKindRuneS,
            IImplDeclarationNameS,
        };
        use crate::postparsing::rules::rules::{LookupSR, CallSR, IRulexSR, RuneUsage};
        use crate::postparsing::itemplatatype::{ITemplataType, KindTemplataType, TemplateTemplataType};
        use crate::typing::names::names::IdValT;
        use crate::higher_typing::ast::ImplA;
        use crate::utils::arena_index_map::ArenaIndexMap;

        if interface_a.attributes.iter().any(|a| matches!(a, ICitizenAttributeS::Sealed(_))) {
            return vec![];
        }

        let member_runes: Vec<RuneUsage<'s>> =
            interface_a.internal_methods.iter().enumerate().map(|(_index, _method)| {
                panic!("implement: member_runes for non-zero-method interface")
            }).collect();
        let members: Vec<NormalStructMemberS<'s>> =
            interface_a.internal_methods.iter().zip(member_runes.iter()).enumerate().map(|(_index, (_method, _rune))| {
                panic!("implement: members for non-zero-method interface")
            }).collect();

        let struct_name_s = AnonymousSubstructTemplateNameS { interface_name: *interface_a.name };
        let struct_name_s_ref = self.scout_arena.alloc(struct_name_s);
        let struct_local_name = self.translate_name_step(crate::postparsing::names::INameS::AnonymousSubstructTemplateName(struct_name_s_ref));
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
            interface_a.internal_methods.iter().zip(member_runes.iter()).enumerate().map(|(_method_index, (_method, _rune))| {
                panic!("implement: forwarder_methods for non-zero-method interface")
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
        panic!("Unimplemented: Slab 15 — body migration");
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
        panic!("Unimplemented: Slab 15 — body migration");
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
        use crate::postparsing::names::{IRuneValS, AnonymousSubstructVoidKindRuneS, AnonymousSubstructVoidCoordRuneS, CodeNameS, IImpreciseNameValS};
        use crate::postparsing::itemplatatype::{ITemplataType, KindTemplataType, CoordTemplataType};
        use crate::postparsing::rules::rules::{IRulexSR, LookupSR, CoerceToCoordSR};
        use crate::utils::range::RangeS;

        let range = |n: i32| RangeS::internal(self.scout_arena, n);
        let use_rune = |n: i32, rune: crate::postparsing::names::IRuneS<'s>| RuneUsage { range: range(n), rune };

        let mut rules_builder: Vec<IRulexSR<'s>> = Vec::new();
        let mut rune_to_type: Vec<(crate::postparsing::names::IRuneS<'s>, ITemplataType<'s>)> = Vec::new();

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

        let mut struct_generic_params: Vec<&'s crate::postparsing::ast::GenericParameterS<'s>> = Vec::new();
        for gp in interface_a.generic_parameters.iter() {
            struct_generic_params.push(*gp);
        }
        for _mr in member_runes.iter() {
            panic!("implement: member generic params for non-zero-method interface");
        }

        for ((_internal_method, _member_rune), _method_index) in
            interface_a.internal_methods.iter().zip(member_runes.iter()).zip(0i32..) {
            panic!("implement: method loop body in make_struct_anonymous_interface");
        }

        let member_coord_types: Vec<ITemplataType<'s>> = member_runes.iter()
            .map(|_mr| ITemplataType::CoordTemplataType(CoordTemplataType {}))
            .collect();
        let mut param_types: Vec<ITemplataType<'s>> = interface_a.tyype.param_types.to_vec();
        param_types.extend(member_coord_types);
        let param_types_slice = self.scout_arena.alloc_slice_from_vec(param_types);
        let kind_type = self.scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {}));
        let tyype = crate::postparsing::itemplatatype::TemplateTemplataType {
            param_types: param_types_slice,
            return_type: kind_type,
        };

        let header_rune_to_type = self.scout_arena.alloc_index_map_from_iter(rune_to_type);
        let header_rules_slice = self.scout_arena.alloc_slice_from_vec(rules_builder);
        let members_rune_to_type = self.scout_arena.alloc_index_map::<crate::postparsing::names::IRuneS<'s>, ITemplataType<'s>>();
        let member_rules_slice: &'s [IRulexSR<'s>] = self.scout_arena.alloc_slice_from_vec(vec![]);
        let generic_params_slice = self.scout_arena.alloc_slice_from_vec(struct_generic_params);
        let attributes_slice: &'s [crate::postparsing::ast::ICitizenAttributeS<'s>] = self.scout_arena.alloc_slice_from_vec(vec![]);
        let members_slice: &'s [crate::postparsing::ast::IStructMemberS<'s>] = self.scout_arena.alloc_slice_from_vec(
            members.iter().map(|m| crate::postparsing::ast::IStructMemberS::NormalStructMember(*m)).collect::<Vec<_>>());

        let struct_a = StructA::new(
            interface_a.range,
            crate::postparsing::names::IStructDeclarationNameS::AnonymousSubstructTemplateName(
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
      members)
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
        panic!("Unimplemented: Slab 15 — body migration");
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
