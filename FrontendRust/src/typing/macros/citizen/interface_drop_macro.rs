use crate::higher_typing::ast::*;
use crate::typing::names::names::*;
use crate::typing::env::environment::*;
use crate::typing::env::i_env_entry::*;
use crate::typing::compiler::Compiler;
use crate::postparsing::names::{IRuneValS, MacroVoidKindRuneS, MacroVoidCoordRuneS, MacroSelfKindTemplateRuneS, MacroSelfKindRuneS, MacroSelfCoordRuneS, IVarNameS, IFunctionDeclarationNameValS, INameValS, FunctionNameS, IFunctionDeclarationNameS};
use crate::postparsing::rules::rules::{LookupSR, CallSR, CoerceToCoordSR, IRulexSR, RuneUsage};
use crate::postparsing::patterns::patterns::{CaptureS, AtomSP};
use crate::postparsing::ast::{ParameterS, IBodyS, AbstractBodyS};
use crate::postparsing::itemplatatype::{ITemplataType, CoordTemplataType, KindTemplataType, TemplateTemplataType, FunctionTemplataType};
use crate::typing::names::names::{IFunctionTemplateNameT, INameT};
use crate::utils::range::{RangeS, CodeLocationS};
use std::collections::HashMap;
use crate::postparsing::names::IImpreciseNameValS;
use crate::postparsing::names::CodeNameS;
use crate::postparsing::names::TopLevelCitizenDeclarationNameS;
use crate::higher_typing::ast::FunctionA;
use crate::postparsing::ast::AbstractSP;

/*
package dev.vale.typing.macros.citizen

import dev.vale.highertyping.{FunctionA, InterfaceA}
import dev.vale.postparsing.patterns._
import dev.vale.postparsing.rules._
import dev.vale._
import dev.vale.postparsing._
import dev.vale.typing.ast.PrototypeT
import dev.vale.typing.env.{FunctionEnvEntry, IEnvEntry}
import dev.vale.typing.expression.CallCompiler
import dev.vale.typing.macros.IOnInterfaceDefinedMacro
import dev.vale.typing.names.{INameT, IdT, NameTranslator}
import dev.vale.typing.types.MutabilityT
import dev.vale.highertyping.FunctionA
import dev.vale.parsing.ast.MoveP
import dev.vale.postparsing._
import dev.vale.typing.env.IInDenizenEnvironmentT
import dev.vale.typing.names.FunctionTemplateNameT
import dev.vale.typing.types._
import dev.vale.typing.OverloadResolver

import scala.collection.mutable
*/
// (Scala `class InterfaceDropMacro(interner, keywords, nameTranslator)` absorbed onto
//  `Compiler`; the method body lives at
//  `Compiler::get_interface_sibling_entries_interface_drop` below.)
/*
class InterfaceDropMacro(
  interner: Interner,
  keywords: Keywords,
  nameTranslator: NameTranslator
) extends IOnInterfaceDefinedMacro {

  val macroName: StrI = keywords.DeriveInterfaceDrop
*/
impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where 's: 't,
{
    pub fn get_interface_sibling_entries_interface_drop(
        &self,
        interface_name: IdT<'s, 't>,
        interface_a: &'s InterfaceA<'s>,
    ) -> Vec<(IdT<'s, 't>, IEnvEntryT<'s, 't>)> {

        let range = |n: i32| -> RangeS<'s> {
            let loc = CodeLocationS::internal(self.scout_arena, n);
            RangeS { begin: loc, end: loc }
        };
        let use_ = |n: i32, rune| RuneUsage { range: range(n), rune };

        let mut rules: Vec<IRulexSR<'s>> = Vec::new();
        // Use the same rules as the original interface, see MDSFONARFO.
        for r in interface_a.rules.iter() { rules.push(*r); }
        let mut rune_to_type: HashMap<_, _> = HashMap::new();
        // Use the same runes as the original interface, see MDSFONARFO.
        for (k, v) in interface_a.rune_to_type.iter() { rune_to_type.insert(*k, *v); }

        let void_kind_rune_s = self.scout_arena.intern_rune(IRuneValS::MacroVoidKindRune(MacroVoidKindRuneS {}));
        rune_to_type.insert(void_kind_rune_s, ITemplataType::KindTemplataType(KindTemplataType {}));
        rules.push(IRulexSR::Lookup(LookupSR {
            range: range(-1672147),
            rune: use_(-64002, void_kind_rune_s),
            name: self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: self.keywords.void })),
        }));
        let void_coord_rune_s = self.scout_arena.intern_rune(IRuneValS::MacroVoidCoordRune(MacroVoidCoordRuneS {}));
        rune_to_type.insert(void_coord_rune_s, ITemplataType::CoordTemplataType(CoordTemplataType {}));
        rules.push(IRulexSR::CoerceToCoord(CoerceToCoordSR {
            range: range(-1672147),
            coord_rune: use_(-64002, void_coord_rune_s),
            kind_rune: use_(-64002, void_kind_rune_s),
        }));

        let interface_name_range = interface_a.name.range;
        let interface_citizen_name = TopLevelCitizenDeclarationNameS::from(interface_a.name);
        let interface_imprecise_name = interface_citizen_name.get_imprecise_name(self.scout_arena);

        let self_kind_template_rune_s = self.scout_arena.intern_rune(IRuneValS::MacroSelfKindTemplateRune(MacroSelfKindTemplateRuneS {}));
        rune_to_type.insert(self_kind_template_rune_s, ITemplataType::TemplateTemplataType(interface_a.tyype));
        rules.push(IRulexSR::Lookup(LookupSR {
            range: interface_name_range,
            rune: RuneUsage { range: interface_name_range, rune: self_kind_template_rune_s },
            name: interface_imprecise_name,
        }));

        let self_kind_rune_s = self.scout_arena.intern_rune(IRuneValS::MacroSelfKindRune(MacroSelfKindRuneS {}));
        rune_to_type.insert(self_kind_rune_s, ITemplataType::KindTemplataType(KindTemplataType {}));
        let generic_param_runes: Vec<_> = interface_a.generic_parameters.iter().map(|p| p.rune).collect();
        let generic_param_runes_slice = self.scout_arena.alloc_slice_copy(&generic_param_runes);
        rules.push(IRulexSR::Call(CallSR {
            range: interface_name_range,
            result_rune: use_(-64002, self_kind_rune_s),
            template_rune: RuneUsage { range: interface_name_range, rune: self_kind_template_rune_s },
            args: generic_param_runes_slice,
        }));

        let self_coord_rune_s = self.scout_arena.intern_rune(IRuneValS::MacroSelfCoordRune(MacroSelfCoordRuneS {}));
        rune_to_type.insert(self_coord_rune_s, ITemplataType::CoordTemplataType(CoordTemplataType {}));
        rules.push(IRulexSR::CoerceToCoord(CoerceToCoordSR {
            range: interface_name_range,
            coord_rune: RuneUsage { range: interface_name_range, rune: self_coord_rune_s },
            kind_rune: RuneUsage { range: interface_name_range, rune: self_kind_rune_s },
        }));

        // Use the same generic parameters as the interface, see MDSFONARFO.
        let function_generic_parameters = interface_a.generic_parameters;

        let function_templata_type = TemplateTemplataType {
            param_types: self.scout_arena.alloc_slice_from_vec(
                function_generic_parameters.iter().map(|p| *rune_to_type.get(&p.rune.rune).unwrap()).collect()
            ),
            return_type: self.scout_arena.alloc(ITemplataType::FunctionTemplataType(FunctionTemplataType {})),
        };

        let name_s = IFunctionDeclarationNameS::FunctionName(FunctionNameS {
            name: self.keywords.drop,
            code_location: interface_a.name.range.begin,
        });
        let mut rune_to_type_map = self.scout_arena.alloc_index_map();
        for (k, v) in rune_to_type { rune_to_type_map.insert(k, v); }
        let rules_slice = self.scout_arena.alloc_slice_copy(&rules);
        let drop_function_a = self.scout_arena.alloc(FunctionA::new(
            interface_a.range,
            name_s,
            &[],
            function_templata_type,
            function_generic_parameters,
            rune_to_type_map,
            self.scout_arena.alloc_slice_from_vec(vec![ParameterS::new(
                range(-1340),
                Some(AbstractSP { range: range(-64002), is_internal_method: true }),
                false,
                AtomSP {
                    range: range(-1340),
                    name: Some(CaptureS { name: IVarNameS::CodeVarName(self.keywords.thiss), mutate: false }),
                    coord_rune: Some(use_(-64002, self_coord_rune_s)),
                    destructure: None,
                },
            )]),
            Some(use_(-64002, void_coord_rune_s)),
            rules_slice,
            IBodyS::AbstractBody(AbstractBodyS {}),
        ));
        let drop_name_local = match self.translate_generic_function_name(drop_function_a.name) {
            IFunctionTemplateNameT::FunctionTemplate(r) => INameT::FunctionTemplate(r),
            IFunctionTemplateNameT::ForwarderFunctionTemplate(r) => INameT::ForwarderFunctionTemplate(r),
            IFunctionTemplateNameT::ConstructorTemplate(r) => INameT::ConstructorTemplate(r),
            IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(r) => INameT::AnonymousSubstructConstructorTemplate(r),
            IFunctionTemplateNameT::LambdaCallFunctionTemplate(r) => INameT::LambdaCallFunctionTemplate(r),
            IFunctionTemplateNameT::OverrideDispatcherTemplate(r) => INameT::OverrideDispatcherTemplate(r),
            IFunctionTemplateNameT::ExternFunction(r) => INameT::ExternFunction(r),
            IFunctionTemplateNameT::FunctionBoundTemplate(r) => INameT::FunctionBoundTemplate(r),
            IFunctionTemplateNameT::PredictedFunctionTemplate(r) => INameT::PredictedFunctionTemplate(r),
        };
        let drop_name_t = *self.typing_interner.intern_id(IdValT {
            package_coord: interface_name.package_coord,
            init_steps: interface_name.init_steps,
            local_name: drop_name_local,
        });
        vec![(drop_name_t, IEnvEntryT::Function(drop_function_a))]
    }
/*
  override def getInterfaceSiblingEntries(interfaceName: IdT[INameT], interfaceA: InterfaceA): Vector[(IdT[INameT], FunctionEnvEntry)] = {
    def range(n: Int) = RangeS.internal(interner, n)
    def use(n: Int, rune: IRuneS) = RuneUsage(range(n), rune)


    val rules = new Accumulator[IRulexSR]()
    // Use the same rules as the original interface, see MDSFONARFO.
    // Per @DRSINI, this copies the hoisted EqualsSR from default generic params.
    interfaceA.rules.foreach(r => rules.add(r))
    val runeToType = mutable.HashMap[IRuneS, ITemplataType]()
    // Use the same runes as the original interface, see MDSFONARFO.
    interfaceA.runeToType.foreach(runeToType += _)

    val voidKindRune = MacroVoidKindRuneS()
    runeToType.put(voidKindRune, KindTemplataType())
    rules.add(LookupSR(range(-1672147),use(-64002, voidKindRune),interner.intern(CodeNameS(keywords.void))))
    val voidCoordRune = MacroVoidCoordRuneS()
    runeToType.put(voidCoordRune, CoordTemplataType())
    rules.add(CoerceToCoordSR(range(-1672147),use(-64002, voidCoordRune),use(-64002, voidKindRune)))

    val selfTemplateRune = MacroSelfKindTemplateRuneS()
    runeToType += (selfTemplateRune -> interfaceA.tyype)
    rules.add(
      LookupSR(
        interfaceA.name.range,
        RuneUsage(interfaceA.name.range, selfTemplateRune),
        interfaceA.name.getImpreciseName(interner)))

    val selfKindRune = MacroSelfKindRuneS()
    runeToType += (selfKindRune -> KindTemplataType())
    rules.add(
      CallSR(
        interfaceA.name.range,
        use(-64002, selfKindRune),
        RuneUsage(interfaceA.name.range, selfTemplateRune),
        interfaceA.genericParameters.map(_.rune).toVector))

    val selfCoordRune = MacroSelfCoordRuneS()
    runeToType += (selfCoordRune -> CoordTemplataType())
    rules.add(
      CoerceToCoordSR(
        interfaceA.name.range,
        RuneUsage(interfaceA.name.range, selfCoordRune),
        RuneUsage(interfaceA.name.range, selfKindRune)))

    // Use the same generic parameters as the interface, see MDSFONARFO.
    // Per @DRSINI, defaults are harmless here because they're only added incrementally
    // for unsolved runes during solveForResolving, not eagerly.
    val functionGenericParameters = interfaceA.genericParameters

    val functionTemplataType =
      TemplateTemplataType(
        functionGenericParameters.map(_.rune.rune).map(runeToType),
        FunctionTemplataType())

    val dropFunctionA =
      FunctionA(
        interfaceA.name.range,
        interner.intern(FunctionNameS(keywords.drop, interfaceA.name.range.begin)),
        Vector(),
        functionTemplataType,
        functionGenericParameters,
        runeToType.toMap,
        Vector(
          ParameterS(
            range(-1340),
            Some(AbstractSP(range(-64002), true)),
            false,
            AtomSP(
              range(-1340),
              Some(CaptureS(interner.intern(CodeVarNameS(keywords.thiss)), false)),
              Some(use(-64002, selfCoordRune)), None))),
        Some(use(-64002, voidCoordRune)),
        rules.buildArray().toVector,
        AbstractBodyS)

    Vector(
      interfaceName.copy(localName = nameTranslator.translateGenericFunctionName(dropFunctionA.name)) ->
        FunctionEnvEntry(dropFunctionA))
  }
}
*/
}
