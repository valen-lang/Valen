use crate::keywords::Keywords;
use crate::utils::range::RangeS;

use crate::postparsing::ast::*;
use crate::postparsing::names::*;

use crate::interner::Interner;
use crate::postparsing::ast::ICitizenAttributeS;
use crate::postparsing::ast::LocationInDenizen;
use crate::postparsing::rules::rules::*;
use crate::typing::ast::citizens::*;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::*;
use crate::typing::env::environment::*;
use crate::typing::env::function_environment_t::*;
use crate::typing::env::i_env_entry::{FunctionEnvEntry, IEnvEntryT};
use crate::typing::infer_compiler::*;
use crate::typing::names::names::*;
#[cfg(feature = "rust_interop")]
use crate::typing::rust_interop::rust_method_entries;
use crate::typing::templata::templata::*;
use crate::typing::templata_compiler::*;
use crate::typing::types::types::*;
use crate::utils::fx::HashMap;
use crate::utils::fx::IndexMap;
use std::marker::PhantomData;
pub struct UncheckedDefiningConclusions<'s, 't> {
  pub envs: InferEnv<'s, 't>,
  pub ranges: Vec<RangeS<'s>>,
  pub call_location: LocationInDenizen<'s>,
  pub definition_rules: Vec<IRulexSR<'s>>,
  pub conclusions: IndexMap<IRuneS<'s>, ITemplataT<'s, 't>>,
}

// deleted: delegate trait removed per god-struct refactor (Compiler now holds all methods directly)

pub enum IResolveOutcome<'s, 't, T> {
  ResolveSuccess(ResolveSuccess<'s, 't, T>),
  ResolveFailure(ResolveFailure<'s, 't, T>),
}

fn resolve_outcome_expect<'s, 't, T>(
  this: IResolveOutcome<'s, 't, T>,
) -> ResolveSuccess<'s, 't, T> {
  panic!("Unimplemented: expect");
  // abstract method — see ResolveSuccess.expect / ResolveFailure.expect
}

pub struct ResolveSuccess<'s, 't, T> {
  pub kind: T,
  pub _phantom: PhantomData<(&'s (), &'t ())>,
}
impl<'s, 't, T> ResolveSuccess<'s, 't, T> {
  fn expect(self) -> ResolveSuccess<'s, 't, T> {
    panic!("Unimplemented: expect");
    // this
  }
}

#[derive(Debug)]
pub struct ResolveFailure<'s, 't, T> {
  pub range: Vec<RangeS<'s>>,
  pub x: IResolvingError<'s, 't>,
  pub _phantom: PhantomData<T>,
}
impl<'s, 't, T> ResolveFailure<'s, 't, T> {
  fn expect(self) -> ResolveSuccess<'s, 't, T> {
    panic!("Unimplemented: expect");
    // throw CompileErrorExceptionT(TypingPassResolvingError(range, x))
  }
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn resolve_struct(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    calling_env: IInDenizenEnvironmentT<'s, 't>,
    call_range: &'t [RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    struct_templata: StructDefinitionTemplataT<'s, 't>,
    uncoerced_template_args: &[ITemplataT<'s, 't>],
  ) -> IResolveOutcome<'s, 't, StructTT<'s, 't>> {
    self.resolve_struct_layer(
      coutputs,
      calling_env,
      call_range,
      call_location,
      struct_templata,
      uncoerced_template_args,
    )
  }

  /// The (local name, template id) for one of a citizen's internal methods, derived from the
  /// citizen's own template id.
  pub fn internal_method_template_id(
    &self,
    parent_template_id: &'t IdT<'s, 't>,
    internal_method: &'s FunctionS<'s>,
  ) -> (INameT<'s, 't>, &'t IdT<'s, 't>) {
    let local_name = INameT::from(self.translate_generic_function_name(internal_method.name));
    (local_name, parent_template_id.add_step(self.typing_interner, local_name))
  }

  pub fn precompile_struct(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    struct_templata: StructDefinitionTemplataT<'s, 't>,
  ) -> () {
    let declaring_env = struct_templata.declaring_env;
    let struct_a = coutputs.get_postparsed_struct(struct_templata.struct_template_id);
    let struct_template_id =
      self.resolve_struct_template(coutputs, self.typing_interner.alloc(struct_templata));
    coutputs.declare_type(struct_template_id);
    // Build internal method entries for the outer env
    let internal_method_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> = struct_a
      .internal_methods
      .iter()
      .map(|internal_method| {
        let (local_name, func_template_id) =
          self.internal_method_template_id(struct_template_id, internal_method);
        (local_name, IEnvEntryT::Function(FunctionEnvEntry { template_id: func_template_id }))
      })
      .collect();
    let sibling_key = struct_template_id.add_step(
      self.typing_interner,
      INameT::PackageTopLevel(
        self.typing_interner.intern_package_top_level_name(PackageTopLevelNameT {}),
      ),
    );
    let sibling_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> = declaring_env
      .global_env()
      .name_to_top_level_environment
      .iter()
      .filter(|(id, _)| **id == *sibling_key)
      .flat_map(|(_, ts)| ts.name_to_entry.iter().map(|(n, e)| (*n, *e)))
      .collect();
    // A Rust-backed type's methods and associated functions live in THIS outer env, added as id-only
    // entries that synthesize lazily on first call (the citizen-compile loop skips them, so they are
    // not force-compiled here). Under the feature only; a Vale struct adds nothing here. Costs one
    // oracle.methods() query (no fn_sig) per imported type. (A Rust type's drop stays a top-level
    // eager entry: it needs no fn_sig, and its receiver sig is manufactured at import.)
    #[cfg_attr(not(feature = "rust_interop"), allow(unused_mut))]
    let mut all_outer_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> =
      internal_method_entries.into_iter().chain(sibling_entries.into_iter()).collect();
    #[cfg(feature = "rust_interop")]
    all_outer_entries.extend(rust_method_entries(self, struct_template_id));
    let mut outer_store = TemplatasStoreBuilder::new(struct_template_id);
    outer_store.add_entries(self.scout_arena, all_outer_entries);
    let outer_templatas = outer_store.build_in(self.typing_interner);
    let outer_env = self.typing_interner.alloc(CitizenEnvironmentT {
      global_env: declaring_env.global_env(),
      parent_env: declaring_env,
      template_id: *struct_template_id,
      id: *struct_template_id,
      templatas: outer_templatas,
    });
    let outer_env_ref = IInDenizenEnvironmentT::Citizen(outer_env);
    coutputs.declare_type_outer_env(struct_template_id, outer_env_ref);
  }

  pub fn precompile_interface(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    interface_templata: InterfaceDefinitionTemplataT<'s, 't>,
  ) -> () {
    let declaring_env = interface_templata.declaring_env;
    let interface_a = coutputs.get_postparsed_interface(interface_templata.interface_template_id);
    let interface_template_id =
      self.resolve_interface_template(coutputs, self.typing_interner.alloc(interface_templata));
    coutputs.declare_type(interface_template_id);
    // We do this here because we might compile a virtual function somewhere before we compile
    // the interface. The virtual function will need to know if the type is sealed to know
    // whether it's allowed to be virtual on this interface.
    coutputs.declare_type_sealed(
      *interface_template_id,
      !interface_a.attributes.iter().any(|a| matches!(a, ICitizenAttributeS::Open(_))),
    );
    // Build internal method entries for the outer env
    let internal_method_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> = interface_a
      .internal_methods
      .iter()
      .map(|internal_method| {
        let (local_name, func_template_id) =
          self.internal_method_template_id(interface_template_id, internal_method);
        (local_name, IEnvEntryT::Function(FunctionEnvEntry { template_id: func_template_id }))
      })
      .collect();
    // Merge in sibling entries from the global environment
    let sibling_key = interface_template_id.add_step(
      self.typing_interner,
      INameT::PackageTopLevel(
        self.typing_interner.intern_package_top_level_name(PackageTopLevelNameT {}),
      ),
    );
    let sibling_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> = declaring_env
      .global_env()
      .name_to_top_level_environment
      .iter()
      .filter(|(id, _)| **id == *sibling_key)
      .flat_map(|(_, ts)| ts.name_to_entry.iter().map(|(n, e)| (*n, *e)))
      .collect();
    #[cfg_attr(not(feature = "rust_interop"), allow(unused_mut))]
    let mut all_outer_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> =
      internal_method_entries.into_iter().chain(sibling_entries.into_iter()).collect();
    #[cfg(feature = "rust_interop")]
    all_outer_entries.extend(rust_method_entries(self, interface_template_id));
    let mut outer_store = TemplatasStoreBuilder::new(interface_template_id);
    outer_store.add_entries(self.scout_arena, all_outer_entries);
    let outer_templatas = outer_store.build_in(self.typing_interner);
    let outer_env = self.typing_interner.alloc(CitizenEnvironmentT {
      global_env: declaring_env.global_env(),
      parent_env: declaring_env,
      template_id: *interface_template_id,
      id: *interface_template_id,
      templatas: outer_templatas,
    });
    let outer_env_ref = IInDenizenEnvironmentT::Citizen(outer_env);
    coutputs.declare_type_outer_env(interface_template_id, outer_env_ref);
  }

  pub fn compile_struct(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    parent_ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    struct_templata: StructDefinitionTemplataT<'s, 't>,
  ) -> Result<UncheckedDefiningConclusions<'s, 't>, ICompileErrorT<'s, 't>> {
    self.compile_struct_layer(coutputs, parent_ranges, call_location, struct_templata)
  }

  pub fn predict_interface(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    calling_env: IInDenizenEnvironmentT<'s, 't>,
    call_range: &'t [RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    interface_templata: InterfaceDefinitionTemplataT<'s, 't>,
    uncoerced_template_args: &[ITemplataT<'s, 't>],
  ) -> InterfaceTT<'s, 't> {
    self.predict_interface_layer(
      coutputs,
      calling_env,
      call_range,
      call_location,
      interface_templata,
      uncoerced_template_args,
    )
  }

  pub fn predict_struct(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    calling_env: IInDenizenEnvironmentT<'s, 't>,
    call_range: &'t [RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    struct_templata: StructDefinitionTemplataT<'s, 't>,
    uncoerced_template_args: &[ITemplataT<'s, 't>],
  ) -> StructTT<'s, 't> {
    self.predict_struct_layer(
      coutputs,
      calling_env,
      call_range,
      call_location,
      struct_templata,
      uncoerced_template_args,
    )
  }

  pub fn resolve_interface(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    calling_env: IInDenizenEnvironmentT<'s, 't>,
    call_range: &'t [RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    interface_templata: InterfaceDefinitionTemplataT<'s, 't>,
    uncoerced_template_args: &[ITemplataT<'s, 't>],
  ) -> IResolveOutcome<'s, 't, InterfaceTT<'s, 't>> {
    self.resolve_interface_layer(
      coutputs,
      calling_env,
      call_range,
      call_location,
      interface_templata,
      uncoerced_template_args,
    )
  }

  pub fn compile_interface(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    global_env: &'t GlobalEnvironmentT<'s, 't>,
    parent_ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    interface_templata: InterfaceDefinitionTemplataT<'s, 't>,
  ) -> Result<UncheckedDefiningConclusions<'s, 't>, ICompileErrorT<'s, 't>> {
    self.compile_interface_layer(coutputs, global_env, parent_ranges, call_location, interface_templata)
  }

  pub fn make_closure_understruct(
    &self,
    containing_function_env: &'t NodeEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    global_env: &'t GlobalEnvironmentT<'s, 't>,
    parent_ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    name: IFunctionDeclarationNameS<'s>,
    function_s: &'s FunctionS<'s>,
    members: &[&'t StructMemberT<'s, 't>],
  ) -> Result<(StructTT<'s, 't>, SharednessT, FunctionTemplataT<'s, 't>), ICompileErrorT<'s, 't>>
  {
    self.make_closure_understruct_layer(
      containing_function_env,
      coutputs,
      global_env,
      parent_ranges,
      call_location,
      name,
      function_s,
      members,
    )
  }

  // VCOORD: see if we can get rid of this function and just inline it
  pub fn struct_compiler_get_sharedness(
    &self,
    _sanity_check: bool,
    coutputs: &mut CompilerOutputs<'s, 't>,
    _original_calling_denizen_id: IdT<'s, 't>,
    _region: RegionT,
    struct_tt: StructTT<'s, 't>,
    _bound_arguments_source: IBoundArgumentsSource<'s, 't>,
  ) -> SharednessT {
    // Sharedness is parse-time-known and not template-parametric, so no substitution needed.
    coutputs.lookup_struct(*struct_tt.id, self).sharedness
  }

  /// Each of the citizen's own `where func` bounds, evaluated with its generic runes bound DIRECTLY
  /// to `substituting_args` (the citizen instance's template args, already in the caller's terms), so
  /// the produced name + return are already in the caller's terms — no citizen placeholders are ever
  /// conjured. Anchor-free: it hands back each bound's identity name and return type, keyed by the
  /// bound's result rune; `import_function_bound` anchors and registers it into a denizen. Derived
  /// purely from the postparsed `func_bounds` (no compiled inner env); recomputed per call.
  pub fn resolve_citizen_bounds(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    citizen_template_id: &'t IdT<'s, 't>,
    substituting_args: &[ITemplataT<'s, 't>],
  ) -> IndexMap<IRuneS<'s>, (&'s FunctionS<'s>, &'t FunctionBoundNameT<'s, 't>, KindT<'s, 't>)> {
    let mut out: IndexMap<
      IRuneS<'s>,
      (&'s FunctionS<'s>, &'t FunctionBoundNameT<'s, 't>, KindT<'s, 't>),
    > = IndexMap::default();
    // A citizen with no postparsed AHT reachable here — a lambda/anonymous closure struct, or an
    // un-illuminated Rust citizen — declares no `where func` bounds, so it contributes none.
    let (generic_params, func_bounds): (
      &'s [&'s GenericParameterS<'s>],
      &'s [(RuneUsage<'s>, FunctionS<'s>)],
    ) = match coutputs.peek_postparsed_type(citizen_template_id) {
      Some(ICitizenDenizenS::TopLevelStruct(s)) => (s.generic_params, s.func_bounds),
      Some(ICitizenDenizenS::TopLevelInterface(i)) => (i.generic_params, i.func_bounds),
      None => return out,
    };
    // The citizen's generic runes bound directly to the use-site args (same positional order the
    // resolve/instantiation machinery pairs them by). Its outer env is only for evaluate_templex's
    // primitive Name lookups, never for a citizen placeholder.
    let env = IEnvironmentT::from(coutputs.get_outer_env_for_type(*citizen_template_id));
    let rune_to_substitution_templata: IndexMap<IRuneS<'s>, ITemplataT<'s, 't>> = generic_params
        .iter()
        .zip(substituting_args.iter())
        .map(|(gp, arg)| (gp.rune.rune, *arg))
        .collect();
    for (result_rune, func_bound) in func_bounds {
      let human_name = match func_bound.name {
        IFunctionDeclarationNameS::FunctionName(fns) => fns.imprecise_name.name,
        other => panic!("substitute_citizen_own_bounds: unexpected bound name {:?}", other),
      };
      let param_coords: Vec<KindT<'s, 't>> = func_bound
          .params
          .iter()
          .map(|p| self.evaluate_templex(env, &rune_to_substitution_templata, &p.tyype).expect_kind())
          .collect();
      let return_type = self
          .evaluate_templex(
            env,
            &rune_to_substitution_templata,
            &func_bound.maybe_return_type.expect("function bound has no return type"),
          )
          .expect_kind();
      let template_name = self
          .typing_interner
          .intern_function_bound_template_name(FunctionBoundTemplateNameT { human_name });
      let bound_name = self.typing_interner.intern_function_bound_name(FunctionBoundNameValT {
        template: template_name,
        template_args: &[],
        parameters: self.typing_interner.alloc_slice_from_vec(param_coords),
      });
      out.insert(result_rune.rune, (func_bound, bound_name, return_type));
    }
    out
  }

  pub fn instantiation_template_args(
    &self,
    citizen_instance_id: IdT<'s, 't>,
  ) -> &'t [ITemplataT<'s, 't>] {
    let local_name: IInstantiationNameT<'s, 't> =
      citizen_instance_id.local_name.try_into().unwrap_or_else(|_| {
        panic!(
          "citizen_instance_template_args: localName must be IInstantiationNameT, got {:?}",
          citizen_instance_id.local_name
        )
      });
    local_name.template_args()
  }
}
