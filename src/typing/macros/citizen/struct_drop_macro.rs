use crate::interner::StrI;
use crate::postparsing::ast::FunctionS;
use crate::postparsing::ast::LocationInDenizen;
use crate::postparsing::ast::*;
use crate::postparsing::ast::{GeneratedBodyS, IBodyS, ParameterS};
use crate::postparsing::itemplatatype::*;
use crate::postparsing::itemplatatype::{
  FunctionTemplataType, ITemplataType, KindTemplataType, TemplateTemplataType,
};
use crate::postparsing::names::CodeNameS;
use crate::postparsing::names::IImpreciseNameValS;
use crate::postparsing::names::*;
use crate::postparsing::names::{
  CodeVarNameS, FunctionNameS, IFunctionDeclarationNameS, IFunctionDeclarationNameValS, INameValS,
  IRuneValS, IVarNameS, MacroVoidKindRuneS, SelfKindRuneS, SelfKindTemplateRuneS,
};
use crate::postparsing::patterns::patterns::{AtomSP, CaptureS};
use crate::postparsing::rules::rules::{CallSR, IRulexSR, LookupSR, RuneUsage};
use crate::typing::ast::ast::*;
use crate::typing::ast::expressions::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::macros::macros::GeneratedAhtDenizen;
use crate::typing::names::names::*;
use crate::typing::names::names::{IFunctionTemplateNameT, INameT};
use crate::typing::templata::templata::*;
use crate::typing::templata_compiler::IBoundArgumentsSource;
use crate::typing::types::types::*;
use crate::utils::range::CodeLocationS;
use crate::utils::range::RangeS;
use std::marker::PhantomData;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn get_struct_sibling_entries_struct_drop(
    &self,
    struct_name: IdT<'s, 't>,
    struct_a: &'s StructS<'s>,
  ) -> Vec<GeneratedAhtDenizen<'s, 't>> {
    let range = |n: i32| -> RangeS<'s> {
      let loc = CodeLocationS::internal(self.scout_arena, n);
      RangeS::new(loc, loc)
    };
    let use_ = |n: i32, rune| RuneUsage { range: range(n), rune };

    let mut rules: Vec<IRulexSR<'s>> = Vec::new();
    // Use the same rules as the original struct, see MDSFONARFO.
    for r in struct_a.header_rules.iter() {
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
    let self_kind_template_rune_s =
      self.scout_arena.intern_rune(IRuneValS::SelfKindTemplateRune(SelfKindTemplateRuneS {
        loc: struct_a.range.begin,
      }));
    rules.push(IRulexSR::Lookup(LookupSR {
      range: struct_a.name.range(),
      rune: RuneUsage { range: struct_a.name.range(), rune: self_kind_template_rune_s },
      parts: self
        .scout_arena
        .alloc_slice_copy(&[struct_a.name.get_imprecise_name(self.scout_arena)]),
    }));

    let self_kind_rune_s = self.scout_arena.intern_rune(IRuneValS::SelfKindRune(SelfKindRuneS {}));
    let generic_param_runes: Vec<_> = struct_a.generic_params.iter().map(|p| p.rune).collect();
    let generic_param_runes_slice = self.scout_arena.alloc_slice_copy(&generic_param_runes);
    rules.push(IRulexSR::Call(CallSR {
      range: struct_a.name.range(),
      result_rune: use_(-64002, self_kind_rune_s),
      template_rune: RuneUsage { range: struct_a.name.range(), rune: self_kind_template_rune_s },
      args: generic_param_runes_slice,
    }));

    // Use the same generic parameters as the struct
    let function_generic_parameters = struct_a.generic_params;

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
      code_location: struct_a.range.begin,
    });
    let rules_slice = self.scout_arena.alloc_slice_copy(&rules);
    let drop_function_a = self.scout_arena.alloc(FunctionS::new(
      struct_a.range,
      name_s,
      &[],
      function_generic_parameters,
      function_templata_type,
      self.scout_arena.alloc_slice_from_vec(vec![ParameterS::new(
        range(-1340),
        None,
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
      self.scout_arena.alloc(IBodyS::GeneratedBody(GeneratedBodyS {
        generator_id: self.keywords.drop_generator,
      })),
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
    let drop_name_t = struct_name.add_step(self.typing_interner, drop_name_local);
    vec![GeneratedAhtDenizen::Function(drop_name_t, drop_function_a)]
  }

  pub fn make_implicit_drop_function_struct_drop(
    &self,
    drop_or_free_function_name_s: IFunctionDeclarationNameS<'s>,
    struct_range: RangeS<'s>,
  ) -> FunctionS<'s> {
    let internal_range = |n: i32| {
      let loc = CodeLocationS::internal(self.scout_arena, n);
      RangeS::new(loc, loc)
    };

    let drop_p1k_rune =
      self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: self.keywords.drop_p1k }));
    let drop_vk_rune =
      self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: self.keywords.drop_vk }));

    let params = self.scout_arena.alloc_slice_from_vec(vec![ParameterS::new(
      internal_range(-1342),
      None,
      false,
      IVarNameS::CodeVarName(self.keywords.x),
      RuneUsage { range: internal_range(-64002), rune: drop_p1k_rune },
      RuneUsage { range: internal_range(-64002), rune: drop_p1k_rune },
      self.scout_arena.alloc_slice_from_vec::<IRulexSR<'s>>(Vec::new()),
      self.scout_arena.alloc_slice_from_vec::<IRulexSR<'s>>(Vec::new()),
    )]);

    let maybe_ret_coord_rune =
      Some(RuneUsage { range: internal_range(-64002), rune: drop_vk_rune });

    let self_name_s =
      self.scout_arena.intern_imprecise_name(IImpreciseNameValS::SelfName(SelfNameS {}));
    let void_name_s = self
      .scout_arena
      .intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: self.keywords.void }));

    let rules = self.scout_arena.alloc_slice_from_vec(vec![
      IRulexSR::Lookup(LookupSR {
        range: internal_range(-1672161),
        rune: RuneUsage { range: internal_range(-64002), rune: drop_p1k_rune },
        parts: self.scout_arena.alloc_slice_copy(&[self_name_s]),
      }),
      IRulexSR::Lookup(LookupSR {
        range: internal_range(-1672162),
        rune: RuneUsage { range: internal_range(-64002), rune: drop_vk_rune },
        parts: self.scout_arena.alloc_slice_copy(&[void_name_s]),
      }),
    ]);

    FunctionS::new(
      struct_range,
      drop_or_free_function_name_s,
      self.scout_arena.alloc_slice_from_vec(vec![]),
      self.scout_arena.alloc_slice_from_vec(vec![]),
      TemplateTemplataType {
        param_types: self.scout_arena.alloc_slice_from_vec(vec![]),
        return_type: self
          .scout_arena
          .alloc(ITemplataType::FunctionTemplataType(FunctionTemplataType {})),
      },
      params,
      maybe_ret_coord_rune,
      rules,
      &[],
      self.scout_arena.alloc(IBodyS::GeneratedBody(GeneratedBodyS {
        generator_id: self.keywords.drop_generator,
      })),
    )
  }

  pub fn generate_function_body_struct_drop(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    env: &'t FunctionEnvironmentT<'s, 't>,
    generator_id: StrI<'s>,
    life: LocationInFunctionEnvironmentT<'t>,
    call_range: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    origin_function1: Option<&'s FunctionS<'s>>,
    params2: &[ParameterT<'s, 't>],
    maybe_ret_coord: Option<KindT<'s, 't>>,
  ) -> Result<(FunctionHeaderT<'s, 't>, ExpressionTE<'s, 't>), ICompileErrorT<'s, 't>> {
    let body_env = IInDenizenEnvironmentT::Function(env);

    let struct_tt = match params2[0].tyype {
      KindT::Struct(s) => s,
      _ => panic!("struct drop: first param is not a struct"),
    };
    let struct_def = coutputs.lookup_struct(struct_tt.id, self);
    // A share citizen is only ever held ShareRef-wrapped; a single one is held bare.
    let struct_type = match struct_def.sharedness {
      SharednessT::Single => KindT::Struct(struct_tt),
      SharednessT::Shared => {
        KindT::ShareRef(self.typing_interner.alloc(ShareRefT { inner: KindT::Struct(struct_tt) }))
      }
    };

    let ret = KindT::Void(VoidT {});
    let params_arena: &'t [ParameterT<'s, 't>] =
      self.typing_interner.alloc_slice_from_vec(params2.to_vec());
    let header = FunctionHeaderT {
      id: env.id,
      attributes: &[],
      params: params_arena,
      return_type: ret,
      maybe_origin_function_templata: Some(env.templata()),
    };

    coutputs.declare_function_return_type(
      self.typing_interner.alloc(header.to_signature()),
      header.return_type,
    );

    let is_extern =
      struct_def.attributes.iter().any(|a| matches!(a, ICitizenAttributeT::Extern(_)));
    let body_expr: ExpressionTE<'s, 't> = match struct_def.sharedness {
      SharednessT::Shared => ExpressionTE::Discard(self.typing_interner.alloc(DiscardTE::new(
        ExpressionTE::ArgLookup(self.typing_interner.alloc(ArgLookupTE::new(0, struct_type))),
      ))),
      SharednessT::Single if is_extern => {
        // VCOORD: implement this per todo/opaque-extern-drop.md
        panic!("auto-generated drop for extern struct is unsupported; supply an explicit `extern func drop(...)` for {:?}", struct_def.instantiated_citizen);
      }
      SharednessT::Single => {
        let member_local_variables: Vec<&'t LocalVariable<'s, 't>> = struct_def
          .members
          .iter()
          .map(|member| {
            let substituter = self.get_placeholder_substituter(
              self.opts.global_options.sanity_check,
              env.template_id,
              struct_tt.id,
              IBoundArgumentsSource::InheritBoundsFromTypeItself,
            );
            let reference = substituter.substitute_for_kind(coutputs, member.tyype);
            let member_local: &'t LocalVariable<'s, 't> =
              self.typing_interner.alloc(LocalVariable { name: member.name, tyype: reference });
            member_local
          })
          .collect();
        let member_local_variables_slice =
          self.typing_interner.alloc_slice_from_vec(member_local_variables.clone());
        let arg_lookup =
          ExpressionTE::ArgLookup(self.typing_interner.alloc(ArgLookupTE::new(0, struct_type)));
        let destroy = ExpressionTE::Destroy(self.typing_interner.alloc(DestroyTE::new(
          arg_lookup,
          struct_tt,
          member_local_variables_slice,
        )));
        let origin_range: Vec<RangeS<'s>> = origin_function1.map(|f| f.range).into_iter().collect();
        let drop_call_range: Vec<RangeS<'s>> =
          origin_range.into_iter().chain(call_range.iter().copied()).collect();
        let drop_call_range_slice = self.typing_interner.alloc_slice_from_vec(drop_call_range);
        let drop_exprs: Vec<ExpressionTE<'s, 't>> = member_local_variables
          .iter()
          .map(|v| {
            let unlet = ExpressionTE::Unlet(self.typing_interner.alloc(UnletTE::new(*v)));
            self.drop(
              body_env,
              coutputs,
              drop_call_range_slice,
              call_location,
              RegionT::Default,
              unlet,
            )
          })
          .collect::<Result<Vec<_>, _>>()?;
        let mut all_exprs: Vec<ExpressionTE<'s, 't>> = vec![destroy];
        all_exprs.extend(drop_exprs.into_iter());
        self.consecutive(&all_exprs)
      }
    };

    let return_expr = ExpressionTE::Return(self.typing_interner.alloc(ReturnTE::new(
      ExpressionTE::VoidLiteral(self.typing_interner.alloc(VoidLiteralTE::new(RegionT::Default))),
    )));
    let body = ExpressionTE::Block(
      self.typing_interner.alloc(BlockTE::new(self.consecutive(&[body_expr, return_expr]))),
    );

    Ok((header, body))
  }
}
