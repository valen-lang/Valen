use crate::parsing::ast::IMacroInclusionP;
use crate::postparsing::ast::MacroCallS;
use crate::postparsing::ast::{ExternS, ICitizenAttributeS, IStructMemberS, LocationInDenizen};
use crate::postparsing::ast::{FunctionS, IBodyS, InterfaceS, StructS};
use crate::postparsing::names::FunctionNameS;
use crate::postparsing::names::IFunctionDeclarationNameS;
use crate::postparsing::names::INameS;
use crate::postparsing::names::INameValS;
use crate::postparsing::names::IStructDeclarationNameS;
use crate::postparsing::names::RuneNameS;
use crate::postparsing::names::{CodeNameS, CodeNameValS, IImpreciseNameS, IImpreciseNameValS, RuneNameValS};
use crate::typing::ast::ast::PrototypeT;
use crate::typing::ast::ast::{ExternT, ICitizenAttributeT, LocT};
use crate::typing::ast::citizens::{InterfaceDefinitionT, StructDefinitionT, StructMemberT};
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::compiler_outputs::DeferredActionT;
use crate::typing::env::environment::{CitizenEnvironmentT, GlobalEnvironmentT, IInDenizenEnvironmentT};
use crate::typing::env::environment::{IEnvironmentT, ILookupContext, TemplatasStoreBuilder};
use crate::typing::env::function_environment_t::NodeEnvironmentT;
use crate::typing::env::i_env_entry::{FunctionEnvEntry, IEnvEntryT};
use crate::typing::hinputs_t::make;
use crate::typing::hinputs_t::InstantiationBoundArgumentsT;
use crate::typing::names::names::IInterfaceTemplateNameT;
use crate::typing::names::names::*;
use crate::typing::names::names::{MemberNameT, IVarNameT};
use crate::typing::names::names::{IInstantiationNameT, INameT, IStructTemplateNameT, IdValT};
#[cfg(feature = "rust_interop")]
use crate::typing::rust_interop::is_rust_backed;
use crate::typing::templata::templata::FunctionTemplataT;
use crate::typing::templata::templata::ITemplataT;
use crate::typing::templata::templata::*;
use crate::typing::templata_compiler::translate_sharedness;
use crate::typing::types::types::InterfaceTTValT;
use crate::typing::types::types::StructTTValT;
use crate::typing::types::types::*;
use crate::typing::types::types::{SharednessT, StructTT};
use crate::utils::fx::HashSet;
use crate::utils::range::RangeS;
use std::iter::once;
use std::marker::PhantomData;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn compile_struct_core(
    &self,
    outer_env: IInDenizenEnvironmentT<'s, 't>,
    struct_runes_env: &'t CitizenEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    parent_ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    struct_a: &'s StructS<'s>,
  ) -> Result<(), ICompileErrorT<'s, 't>> {
    let template_args =
      IInstantiationNameT::try_from(struct_runes_env.id.local_name).unwrap().template_args();
    let template_id_t = struct_runes_env.template_id;
    let template_name_t = IStructTemplateNameT::try_from(template_id_t.local_name).unwrap();
    let placeholdered_name_t =
      template_name_t.make_struct_name(self.typing_interner, template_args);
    let template_id_steps = template_id_t.init_steps.to_vec();
    let placeholdered_id_t = *self.typing_interner.intern_id(IdValT {
      package_coord: template_id_t.package_coord,
      init_steps: &template_id_steps,
      local_name: placeholdered_name_t,
    });

    // Usually when we make a StructTT we put the instantiation bounds into the coutputs,
    // but this isn't really an instantiation, so we don't here.
    let placeholdered_struct_tt =
      *self.typing_interner.intern_struct_tt(StructTTValT { id: placeholdered_id_t });

    let attributes_without_export_or_macros: Vec<ICitizenAttributeS<'s>> = struct_a
      .attributes
      .iter()
      .filter(|attr| match attr {
        ICitizenAttributeS::Export(_) => false,
        ICitizenAttributeS::MacroCall(_) => false,
        _ => true,
      })
      .copied()
      .collect();

    let sharedness: SharednessT = translate_sharedness(struct_a.sharedness);

    let is_extern =
      struct_a.attributes.iter().any(|attr| matches!(attr, ICitizenAttributeS::Extern(_)));
    if is_extern && sharedness == SharednessT::Shared {
      // VCOORD: error message here instead of a panic
      panic!(
                "extern struct {:?} is declared `share`; post-cut design forbids share-flavored extern structs (they must be Own+Inline). Remove the `share` keyword.",
                struct_a.name,
            );
    }

    let default_called_macros: Vec<MacroCallS<'s>> = vec![MacroCallS {
      range: struct_a.range,
      include: IMacroInclusionP::CallMacro,
      macro_name: self.keywords.derive_struct_drop,
    }];
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

    let inner_templatas =
      TemplatasStoreBuilder::new(self.typing_interner.alloc(placeholdered_id_t))
        .build_in(self.typing_interner);
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

    for (name, entry) in outer_env.templatas().name_to_entry.iter() {
      match entry {
        IEnvEntryT::Function(FunctionEnvEntry { template_id: id }) => {
          // Lazily compile rust structs' methods.
          // VRI: Soon, we should lazily compile vale's internal methods too,
          // getting rid of this whole loop.
          #[cfg(feature = "rust_interop")]
          {
            if is_rust_backed(id) {
              continue;
            }
          }
          let deferred_name = outer_env.id().add_step(self.typing_interner, *name);
          coutputs.defer_evaluating_function(
            DeferredActionT::EvaluateFunction { function_id: deferred_name });
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
      sharedness,
      members: members_slice,
      instantiation_bound_params,
    });

    coutputs.add_struct(struct_def_t);
    Ok(())
  }

  pub fn translate_citizen_attributes(
    &self,
    attrs: &[ICitizenAttributeS<'s>],
  ) -> Vec<ICitizenAttributeT<'s>> {
    attrs
      .iter()
      .map(|attr| match attr {
        ICitizenAttributeS::Sealed(_) => ICitizenAttributeT::Sealed,
        ICitizenAttributeS::Extern(ExternS { package_coord: p }) => {
          ICitizenAttributeT::Extern(ExternT { package_coord: **p })
        }
        ICitizenAttributeS::MacroCall(_) => panic!("vwat: MacroCallS should have been processed"),
        x => panic!("vimpl: {:?}", x),
      })
      .collect()
  }

  pub fn compile_interface_core(
    &self,
    global_env: &'t GlobalEnvironmentT<'s, 't>,
    outer_env: IInDenizenEnvironmentT<'s, 't>,
    interface_runes_env: &'t CitizenEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    parent_ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    interface_a: &'s InterfaceS<'s>,
  ) -> Result<&'t InterfaceDefinitionT<'s, 't>, ICompileErrorT<'s, 't>> {
    let template_args =
      IInstantiationNameT::try_from(interface_runes_env.id.local_name).unwrap().template_args();
    let template_id_t = interface_runes_env.template_id;
    let template_name_t = IInterfaceTemplateNameT::try_from(template_id_t.local_name).unwrap();
    let placeholdered_name_t =
      template_name_t.make_interface_name(self.typing_interner, template_args);
    let template_id_steps = template_id_t.init_steps.to_vec();
    let placeholdered_id_t = *self.typing_interner.intern_id(IdValT {
      package_coord: template_id_t.package_coord,
      init_steps: &template_id_steps,
      local_name: placeholdered_name_t,
    });

    // Usually when we make an InterfaceTT we put the instantiation bounds into the coutputs,
    // but this isn't really an instantiation, so we don't here.
    let placeholdered_interface_tt =
      *self.typing_interner.intern_interface_tt(InterfaceTTValT { id: placeholdered_id_t });

    let attributes_without_export_or_macros: Vec<ICitizenAttributeS<'s>> = interface_a
      .attributes
      .iter()
      .filter(|attr| match attr {
        ICitizenAttributeS::Export(_) => false,
        ICitizenAttributeS::MacroCall(_) => false,
        _ => true,
      })
      .copied()
      .collect();
    let _maybe_export =
      interface_a.attributes.iter().find(|attr| matches!(attr, ICitizenAttributeS::Export(_)));

    let sharedness: SharednessT = translate_sharedness(interface_a.sharedness);

    let mut internal_methods: Vec<(PrototypeT<'s, 't>, usize)> = Vec::new();
    for (_name, entry) in outer_env.templatas().name_to_entry.iter() {
      if let IEnvEntryT::Function(FunctionEnvEntry { template_id: id }) = entry {
        // Lazily compile a rust enum's methods — they are inherent, not virtual interface methods, so
        // they must not enter the vtable, and force-compiling one here would reference this interface
        // before it's registered. The same skip the struct-compile loop uses.
        // A rust *trait*'s abstract methods are also is_rust_backed, but they ARE virtual interface
        // methods — the interface's contract that override resolution checks against — so they must be
        // compiled into the vtable eagerly like a native interface's. They are distinguishable: a
        // trait's abstract method is eagerly registered with an `AbstractBody`, while an enum's
        // inherent method is lazy (absent from the postparsed cache here) and never abstract.
        // VRI: Soon, we should lazily compile vale's internal methods too,
        // getting rid of this whole loop.
        // VRI: this is basically saying, dont skip interface methods for rust traits, because
        // interface methods must be listed in internal methods, so that ... look_for_override can
        // see them.
        #[cfg(feature = "rust_interop")]
        {
          let is_abstract_interface_method = coutputs
            .peek_postparsed_function(id)
            .map_or(false, |f| matches!(f.body, IBodyS::AbstractBody(_)));
          if is_rust_backed(id) && !is_abstract_interface_method {
            continue;
          }
        }
        let outer_env_ienv = IEnvironmentT::from(outer_env);
        let header = self.evaluate_generic_function_from_non_call_for_header(
          coutputs,
          global_env,
          parent_ranges,
          call_location,
          id,
        )?;
        let virtual_index = header
          .get_virtual_index()
          .expect("vwat: interface internal method must have a virtual index");
        internal_methods.push((header.to_prototype(), virtual_index));
      }
    }

    let rune_to_function_bound =
      self.assemble_rune_to_function_bound(interface_runes_env.templatas);
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
      sharedness,
      instantiation_bound_params,
      internal_methods: internal_methods_slice,
    });

    coutputs.add_interface(interface_def_t);

    Ok(interface_def_t)
  }

  pub fn make_struct_members(
    &self,
    env: IInDenizenEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    members: &[IStructMemberS<'s>],
  ) -> Vec<StructMemberT<'s, 't>> {
    members.iter().map(|m| self.make_struct_member(env, coutputs, *m)).collect()
  }

  pub fn make_struct_member(
    &self,
    env: IInDenizenEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    member: IStructMemberS<'s>,
  ) -> StructMemberT<'s, 't> {
    // Look up the type for this member's rune
    let tyype = env
      .lookup_nearest_with_imprecise_name(
        self.scout_arena.intern_imprecise_name(IImpreciseNameValS::RuneName(RuneNameValS {
          rune: (*member.type_rune()).rune,
        })),
        once(ILookupContext::TemplataLookupContext).collect(),
        self.typing_interner,
      )
      .expect("make_struct_member: type not found")
      .expect_kind();

    match member {
      IStructMemberS::NormalStructMember(n) => {
        StructMemberT {
          name: self.typing_interner.intern_member_name(MemberNameT {
            imprecise_name: self.scout_arena.intern_code_name(n.name),
            loct: LocT::from_lid(self.typing_interner, n.lid),
          }),
          tyype,
        }
      }
      IStructMemberS::VariadicStructMember(_) => {
        panic!("Unimplemented: make_struct_member VariadicStructMemberS");
        // vimpl()
      }
    }
  }

  pub fn make_closure_understruct_core(
    &self,
    containing_function_env: &'t NodeEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    global_env: &'t GlobalEnvironmentT<'s, 't>,
    parent_ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    name: IFunctionDeclarationNameS<'s>,
    function_a: &'s FunctionS<'s>,
    members: &[&'t StructMemberT<'s, 't>],
  ) -> Result<(StructTT<'s, 't>, SharednessT, FunctionTemplataT<'s, 't>), ICompileErrorT<'s, 't>>
  {
    // VCOORD: make a life builder for stuff like this, this is fragile.
    let closure_life =
        LocT::from_lid(self.typing_interner, call_location)
        .add(self.typing_interner, 0);

    // VCOORD:
    // In the distant future, we'll want to opt into shared closures with a simpler syntax.
    let sharedness = SharednessT::Single;

    let understruct_template_name_t =
      self.typing_interner.intern_lambda_citizen_template_name(LambdaCitizenTemplateNameT {
        code_location: self.translate_code_location(function_a.range.begin),
      });
    let understruct_templated_id = containing_function_env
      .id()
      .add_step(self.typing_interner, INameT::LambdaCitizenTemplate(understruct_template_name_t));

    let understruct_instantiated_name_t =
      IStructTemplateNameT::LambdaCitizenTemplate(understruct_template_name_t)
        .make_struct_name(self.typing_interner, &[]);
    let understruct_instantiated_id =
      containing_function_env.id().add_step(self.typing_interner, understruct_instantiated_name_t);

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
      }),
    );
    let understruct_struct_tt =
      self.typing_interner.intern_struct_tt(StructTTValT { id: *understruct_instantiated_id });

    let drop_func_name_t = INameT::FunctionTemplate(
      self.typing_interner.intern_function_template_name(FunctionTemplateNameT {
        human_name: self.keywords.drop,
        code_location: function_a.range.begin,
      }),
    );

    // We declare the function into the environment that we use to compile the
    // struct, so that those who use the struct can reach into its environment
    // and see the function and use it.
    // See CSFMSEO and SAFHE.
    let call_func_name_t = INameT::FunctionTemplate(
      self.typing_interner.intern_function_template_name(FunctionTemplateNameT {
        human_name: self.keywords.underscores_call,
        code_location: function_a.range.begin,
      }),
    );

    let drop_function_decl_name_s = self.scout_arena.alloc_function_declaration_name(
      IFunctionDeclarationNameS::FunctionName(FunctionNameS {
        imprecise_name: self.scout_arena.intern_code_name(self.keywords.drop),
        code_location: function_a.range.begin,
        lid: LocationInDenizen { path: &[] },
      }),
    );
    let call_func_template_id =
      understruct_templated_id.add_step(self.typing_interner, call_func_name_t);
    coutputs.register_postparsed_function(call_func_template_id, function_a);

    let drop_function_a =
      self.make_implicit_drop_function_struct_drop(*drop_function_decl_name_s, function_a.range);
    let drop_function_a_ref = self.scout_arena.alloc(drop_function_a);
    let drop_func_template_id =
      understruct_templated_id.add_step(self.typing_interner, drop_func_name_t);
    coutputs.register_postparsed_function(drop_func_template_id, drop_function_a_ref);

    let mut outer_store = TemplatasStoreBuilder::new(understruct_templated_id);
    outer_store.add_entries(
      self.scout_arena,
      vec![
        (
          call_func_name_t,
          IEnvEntryT::Function(FunctionEnvEntry { template_id: call_func_template_id }),
        ),
        (
          drop_func_name_t,
          IEnvEntryT::Function(FunctionEnvEntry { template_id: drop_func_template_id }),
        ),
        (
          understruct_instantiated_name_t,
          IEnvEntryT::Templata(ITemplataT::Kind(
            self
              .typing_interner
              .alloc(KindTemplataT { kind: KindT::Struct(understruct_struct_tt) }),
          )),
        ),
        (
          INameT::Self_(self.typing_interner.intern_self_name(SelfNameT {
            loct: closure_life.add(self.typing_interner, 0),
          })),
          IEnvEntryT::Templata(ITemplataT::Kind(
            self
              .typing_interner
              .alloc(KindTemplataT { kind: KindT::Struct(understruct_struct_tt) }),
          )),
        ),
      ],
    );
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
      function_template_id: call_func_template_id,
    };

    coutputs.declare_type(understruct_templated_id);
    coutputs.declare_type_outer_env(
      understruct_templated_id,
      IInDenizenEnvironmentT::Citizen(struct_outer_env),
    );
    coutputs.declare_type_inner_env(
      understruct_templated_id,
      IInDenizenEnvironmentT::Citizen(struct_inner_env),
    );

    let closure_struct_definition = StructDefinitionT {
      template_name: *understruct_templated_id,
      instantiated_citizen: *understruct_struct_tt,
      attributes: self.typing_interner.alloc_slice_from_vec(vec![]),
      sharedness,
      members: self.typing_interner.alloc_slice_from_vec(
        members
          .iter()
          .map(|m| {
            let tyype = m.tyype;
            StructMemberT { name: m.name, tyype }
          })
          .collect::<Vec<_>>(),
      ),
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
        [ILookupContext::ExpressionLookupContext].into_iter().collect(),
        self.typing_interner,
      ) {
        Some(ITemplataT::Function(ft)) => *ft,
        _ => panic!("Couldn't find closure drop function we just added!"),
      }
    };
    self.evaluate_generic_function_from_non_call(
      coutputs,
      global_env,
      parent_ranges,
      call_location,
      drop_function_templata.function_template_id,
    )?;

    Ok((closured_vars_struct_ref, sharedness, function_templata))
  }
}
