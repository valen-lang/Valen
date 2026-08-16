use crate::postparsing::ast::AbstractSP;
use crate::postparsing::ast::FunctionS;
use crate::postparsing::ast::*;
use crate::postparsing::ast::{AbstractBodyS, IBodyS, ParameterS};
use crate::postparsing::itemplatatype::{
  FunctionTemplataType, ITemplataType, KindTemplataType, TemplateTemplataType,
};
use crate::postparsing::names::CodeNameS;
use crate::postparsing::names::IImpreciseNameValS;
use crate::postparsing::names::TopLevelCitizenDeclarationNameS;
use crate::postparsing::names::{
  FunctionNameS, IFunctionDeclarationNameS, IFunctionDeclarationNameValS, INameValS, IRuneValS,
  IVarNameS, MacroSelfKindRuneS, MacroSelfKindTemplateRuneS, MacroVoidKindRuneS,
};
use crate::postparsing::patterns::patterns::{AtomSP, CaptureS};
use crate::postparsing::rules::rules::{CallSR, IRulexSR, LookupSR, RuneUsage};
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::env::environment::*;
use crate::typing::macros::macros::GeneratedAhtDenizen;
use crate::typing::names::names::*;
use crate::typing::names::names::{IFunctionTemplateNameT, INameT};
use crate::utils::range::{CodeLocationS, RangeS};

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn get_interface_sibling_entries_interface_drop(
    &self,
    interface_name: IdT<'s, 't>,
    interface_a: &'s InterfaceS<'s>,
  ) -> Vec<GeneratedAhtDenizen<'s, 't>> {
    let range = |n: i32| -> RangeS<'s> {
      let loc = CodeLocationS::internal(self.scout_arena, n);
      RangeS::new(loc, loc)
    };
    let use_ = |n: i32, rune| RuneUsage { range: range(n), rune };

    let mut rules: Vec<IRulexSR<'s>> = Vec::new();
    // Use the same rules as the original interface, see MDSFONARFO.
    for r in interface_a.rules.iter() {
      rules.push(*r);
    }

    let void_kind_rune_s =
      self.scout_arena.intern_rune(IRuneValS::MacroVoidKindRune(MacroVoidKindRuneS {}));
    rules.push(IRulexSR::Lookup(LookupSR {
      range: range(-1672147),
      rune: use_(-64002, void_kind_rune_s),
      parts: self.scout_arena.alloc_slice_copy(&[self.scout_arena.intern_imprecise_name(
        IImpreciseNameValS::CodeName(CodeNameS { name: self.keywords.void }),
      )]),
    }));
    let interface_name_range = interface_a.name.range;
    let interface_citizen_name = TopLevelCitizenDeclarationNameS::from(interface_a.name);
    let interface_imprecise_name = interface_citizen_name.get_imprecise_name(self.scout_arena);

    let self_kind_template_rune_s = self
      .scout_arena
      .intern_rune(IRuneValS::MacroSelfKindTemplateRune(MacroSelfKindTemplateRuneS {}));
    rules.push(IRulexSR::Lookup(LookupSR {
      range: interface_name_range,
      rune: RuneUsage { range: interface_name_range, rune: self_kind_template_rune_s },
      parts: self.scout_arena.alloc_slice_copy(&[interface_imprecise_name]),
    }));

    let self_kind_rune_s =
      self.scout_arena.intern_rune(IRuneValS::MacroSelfKindRune(MacroSelfKindRuneS {}));
    let generic_param_runes: Vec<_> = interface_a.generic_params.iter().map(|p| p.rune).collect();
    let generic_param_runes_slice = self.scout_arena.alloc_slice_copy(&generic_param_runes);
    rules.push(IRulexSR::Call(CallSR {
      range: interface_name_range,
      result_rune: use_(-64002, self_kind_rune_s),
      template_rune: RuneUsage { range: interface_name_range, rune: self_kind_template_rune_s },
      args: generic_param_runes_slice,
    }));

    // Use the same generic parameters as the interface, see MDSFONARFO.
    let function_generic_parameters = interface_a.generic_params;

    let function_templata_type = TemplateTemplataType {
      param_types: self.scout_arena.alloc_slice_from_vec(
        function_generic_parameters.iter().map(|p| p.tyype.tyype()).collect(),
      ),
      return_type: self
        .scout_arena
        .alloc(ITemplataType::FunctionTemplataType(FunctionTemplataType {})),
    };

    let name_s = IFunctionDeclarationNameS::FunctionName(FunctionNameS {
      name: self.keywords.drop,
      code_location: interface_a.name.range.begin,
    });
    let rules_slice = self.scout_arena.alloc_slice_copy(&rules);
    let drop_function_a = self.scout_arena.alloc(FunctionS::new(
      interface_a.range,
      name_s,
      &[],
      function_generic_parameters,
      function_templata_type,
      self.scout_arena.alloc_slice_from_vec(vec![ParameterS::new(
        range(-1340),
        Some(AbstractSP { range: range(-64002), is_internal_method: true }),
        false,
        IVarNameS::CodeVarName(self.keywords.thiss),
        use_(-64002, self_kind_rune_s),
        use_(-64002, self_kind_rune_s),
        self.scout_arena.alloc_slice_from_vec::<IRulexSR<'s>>(Vec::new()),
        self.scout_arena.alloc_slice_from_vec::<IRulexSR<'s>>(Vec::new()),
      )]),
      Some(use_(-64002, void_kind_rune_s)),
      rules_slice,
      &[],
      self.scout_arena.alloc(IBodyS::AbstractBody(AbstractBodyS {})),
    ));
    let drop_name_local = match self.translate_generic_function_name(drop_function_a.name) {
      IFunctionTemplateNameT::FunctionTemplate(r) => INameT::FunctionTemplate(r),
      IFunctionTemplateNameT::ForwarderFunctionTemplate(r) => INameT::ForwarderFunctionTemplate(r),
      IFunctionTemplateNameT::ConstructorTemplate(r) => INameT::ConstructorTemplate(r),
      IFunctionTemplateNameT::AnonymousSubstructConstructorTemplate(r) => {
        INameT::AnonymousSubstructConstructorTemplate(r)
      }
      IFunctionTemplateNameT::LambdaCallFunctionTemplate(r) => {
        INameT::LambdaCallFunctionTemplate(r)
      }
      IFunctionTemplateNameT::OverrideDispatcherTemplate(r) => {
        INameT::OverrideDispatcherTemplate(r)
      }
      IFunctionTemplateNameT::ExternFunction(r) => INameT::ExternFunction(r),
      IFunctionTemplateNameT::FunctionBoundTemplate(r) => INameT::FunctionBoundTemplate(r),
      IFunctionTemplateNameT::PredictedFunctionTemplate(r) => INameT::PredictedFunctionTemplate(r),
    };
    let drop_name_t_ref = self.typing_interner.intern_id(IdValT {
      package_coord: interface_name.package_coord,
      init_steps: interface_name.init_steps,
      local_name: drop_name_local,
    });
    vec![GeneratedAhtDenizen::Function(drop_name_t_ref, drop_function_a)]
  }
}
