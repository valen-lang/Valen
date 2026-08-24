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
    // VCOORD: rename to declare_type_sharedness
    coutputs.declare_type_sharedness(struct_template_id, translate_sharedness(struct_a.sharedness));
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
    coutputs
      .declare_type_sharedness(interface_template_id, translate_sharedness(interface_a.sharedness));
    // We do this here because we might compile a virtual function somewhere before we compile
    // the interface. The virtual function will need to know if the type is sealed to know
    // whether it's allowed to be virtual on this interface.
    coutputs.declare_type_sealed(
      *interface_template_id,
      interface_a.attributes.iter().any(|a| matches!(a, ICitizenAttributeS::Sealed(_))),
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
    parent_ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    interface_templata: InterfaceDefinitionTemplataT<'s, 't>,
  ) -> Result<UncheckedDefiningConclusions<'s, 't>, ICompileErrorT<'s, 't>> {
    self.compile_interface_layer(coutputs, parent_ranges, call_location, interface_templata)
  }

  pub fn make_closure_understruct(
    &self,
    containing_function_env: &'t NodeEnvironmentT<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
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
      parent_ranges,
      call_location,
      name,
      function_s,
      members,
    )
  }

  pub fn get_compound_type_mutability(&self, member_types: &[KindT<'s, 't>]) -> SharednessT {
    panic!("Unimplemented: Slab 15");
    // val membersOwnerships = memberTypes2.map(_.ownership)
    // val allMembersImmutable = membersOwnerships.isEmpty || membersOwnerships.toSet == Set(ShareT)
    // if (allMembersImmutable) ImmutableT else MutableT
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
    coutputs.lookup_struct(struct_tt.id, self).sharedness
  }
}
