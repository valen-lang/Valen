use crate::interner::StrI;
use crate::keywords::Keywords;
use crate::parsing::ast::ast::IMacroInclusionP;
use crate::postparsing::ast::IFunctionAttributeS;
use crate::postparsing::ast::{FunctionS, ImplS, InterfaceS, ProgramS, StructS};
use crate::postparsing::ast::{ICitizenAttributeS, LocationInDenizen, MacroCallS};
use crate::postparsing::itemplatatype::ITemplataType;
use crate::postparsing::names::CodeNameS;
use crate::postparsing::names::IFunctionDeclarationNameS;
use crate::postparsing::names::IImpreciseNameValS;
use crate::postparsing::names::IStructDeclarationNameS;
use crate::postparsing::names::{IImpreciseNameS, IRuneS};
use crate::postparsing::rules::rules::IRulexSR;
use crate::scout_arena::ScoutArena;
use crate::typing::ast::ast::ICitizenAttributeT;
use crate::typing::ast::ast::LocationInFunctionEnvironmentT;
use crate::typing::ast::ast::ParameterT;
use crate::typing::ast::ast::PrototypeValT;
use crate::typing::ast::ast::{FunctionHeaderT, InterfaceEdgeBlueprintT, KindExportT, PrototypeT};
use crate::typing::ast::citizens::StructMemberT;
use crate::typing::ast::expressions::{ConsecutorTE, ExpressionTE, VoidLiteralTE};
use crate::typing::citizen::struct_compiler::IResolveOutcome;
use crate::typing::citizen::struct_compiler::UncheckedDefiningConclusions;
use crate::typing::compilation::TypingPassOptions;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::{CompilerOutputs, DeferredActionT};
use crate::typing::env::environment::ExportEnvironmentT;
use crate::typing::env::environment::ExternEnvironmentT;
use crate::typing::env::environment::{
  get_imprecise_name, make_top_level_environment, GlobalEnvironmentT, IEnvironmentT,
  IInDenizenEnvironmentT, PackageEnvironmentT, TemplatasStoreBuilder, TemplatasStoreT,
};
use crate::typing::env::function_environment_t::FunctionEnvironmentT;
use crate::typing::env::i_env_entry::{
  FunctionEnvEntry, IEnvEntryT, ImplEnvEntry, InterfaceEnvEntry, StructEnvEntry,
};
use crate::typing::function::function_compiler::IResolveFunctionResult;
use crate::typing::function::function_compiler::StampFunctionSuccess;
use crate::typing::hinputs_t::HinputsT;
use crate::typing::hinputs_t::InstantiationBoundArgumentsT;
use crate::typing::infer_compiler::InferEnv;
use crate::typing::macros::macros::{
  FunctionBodyMacro, GeneratedAhtDenizen, OnInterfaceDefinedMacro, OnStructDefinedMacro,
};
use crate::typing::names::names::CitizenTemplateNameT;
use crate::typing::names::names::ExportNameT;
use crate::typing::names::names::ExportTemplateNameT;
use crate::typing::names::names::ExternFunctionNameValT;
use crate::typing::names::names::ExternNameT;
use crate::typing::names::names::ExternTemplateNameT;
use crate::typing::names::names::FunctionBoundNameValT;
use crate::typing::names::names::FunctionBoundTemplateNameT;
use crate::typing::names::names::{
  IFunctionTemplateNameT, IImplTemplateNameT, IInstantiationNameT, IInterfaceTemplateNameT, INameT,
  IStructTemplateNameT, ITemplateNameT, IdT, IdValT, PackageTopLevelNameT, PrimitiveNameT,
};
use crate::typing::names::names::{PredictedFunctionNameValT, PredictedFunctionTemplateNameT};
use crate::typing::oracles::Oracles;
use crate::typing::overload_resolver::FindFunctionFailure;
#[cfg(feature = "rust_interop")]
use crate::typing::rust_interop::{
  create_postparsed_function, declare_rust_import, is_rust_backed, RustImportSeed,
};
use crate::typing::templata::templata::ImplDefinitionTemplataT;
use crate::typing::templata::templata::{
  FunctionTemplataT, ITemplataT, InterfaceDefinitionTemplataT, KindTemplataT, PlaceholderTemplataT,
  PrototypeTemplataT, RuntimeSizedArrayTemplateTemplataT, StaticSizedArrayTemplateTemplataT,
  StructDefinitionTemplataT,
};
use crate::typing::templata_compiler::IBoundArgumentsSource;
use crate::typing::types::types::ICitizenTT;
use crate::typing::types::types::ISubKindTT;
use crate::typing::types::types::RegionT;
use crate::typing::types::types::RuntimeSizedArrayTT;
use crate::typing::types::types::StaticSizedArrayTT;
use crate::typing::types::types::StructTT;
use crate::typing::types::types::{
  BoolT, BorrowRefT, FloatT, IntT, KindT, NeverT, OwnRefT, ShareRefT, StrT, USizeT, VoidT, WeakRefT,
};
use crate::typing::typing_interner::TypingInterner;
use crate::utils::code_hierarchy::{FileCoordinateMap, PackageCoordinate, PackageCoordinateMap};
use crate::utils::fx::HashMap;
use crate::utils::fx::HashSet;
use crate::utils::fx::IndexMap;
use crate::utils::range::RangeS;
use std::iter::empty;
use std::iter::once;
use std::marker::PhantomData;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IFunctionGenerator {
  StructConstructor,
  StructDrop,
  InterfaceDrop,
  RsaDropInto,
  RsaLen,
  RsaCapacity,
  RsaNew,
  RsaPop,
  RsaPush,
  SsaDropInto,
  SsaLen,
  LockWeak,
  SameInstance,
  AsSubtype,
  AbstractBody,
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn print(&self, x: ()) {
    panic!("Unimplemented: Slab 15");
    // println("###: " + x)
  }
}
pub struct Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub scout_arena: &'ctx ScoutArena<'s>,
  pub typing_interner: &'ctx TypingInterner<'s, 't>,
  pub keywords: &'ctx Keywords<'s>,
  pub opts: &'ctx TypingPassOptions,
  // The Rust-interop oracle: a borrowed query service, alongside the other borrowed
  // services above. Present only in the rustc-linked binary. It answers questions;
  // it never accumulates anything, so it lives here and not on CompilerOutputs
  // (which exists to be drained into HinputsT).
  pub oracles: Oracles<'ctx, 's, 't>,
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't>
where
  's: 't,
{
  pub fn new(
    scout_arena: &'ctx ScoutArena<'s>,
    typing_interner: &'ctx TypingInterner<'s, 't>,
    keywords: &'ctx Keywords<'s>,
    opts: &'ctx TypingPassOptions,
    oracles: Oracles<'ctx, 's, 't>,
  ) -> Self {
    Compiler { scout_arena, typing_interner, keywords, opts, oracles }
  }

  pub fn get_placeholders_in_id(&self, accum: &mut Vec<IdT<'s, 't>>, id: IdT<'s, 't>) {
    match id.local_name {
      INameT::KindPlaceholder(_) => accum.push(id),
      INameT::KindPlaceholderTemplate(_) => accum.push(id),
      _ => {}
    }
  }

  pub fn get_placeholders_in_templata(
    &self,
    accum: &mut Vec<IdT<'s, 't>>,
    templata: ITemplataT<'s, 't>,
  ) {
    match templata {
      ITemplataT::Kind(KindTemplataT { kind }) => self.get_placeholders_in_kind(accum, *kind),
      ITemplataT::Placeholder(PlaceholderTemplataT { id, .. }) => accum.push(*id),
      ITemplataT::Integer(_) => {}
      ITemplataT::Boolean(_) => {}
      ITemplataT::String(_) => {}
      ITemplataT::RuntimeSizedArrayTemplate(_) => {}
      ITemplataT::StaticSizedArrayTemplate(_) => {}
      ITemplataT::InterfaceDefinition(_) => {}
      ITemplataT::StructDefinition(_) => {}
      ITemplataT::ImplDefinition(_) => {}
      ITemplataT::CoordList(_) => {
        panic!("implement: get_placeholders_in_templata CoordList");
        // coords.foreach(c => getPlaceholdersInKind(accum, c.kind))
      }
      ITemplataT::Prototype(_) => {
        panic!("implement: get_placeholders_in_templata Prototype");
        // getPlaceholdersInId(accum, prototype.id)
        // prototype.paramTypes.foreach(c => getPlaceholdersInKind(accum, c.kind))
        // getPlaceholdersInKind(accum, prototype.returnType.kind)
      }
      ITemplataT::Isa(_) => {
        panic!("implement: get_placeholders_in_templata Isa");
        // getPlaceholdersInKind(accum, subKind)
        // getPlaceholdersInKind(accum, superKind)
      }
      ITemplataT::Group(_) => {}
      _ => {
        panic!("implement: get_placeholders_in_templata other");
      }
    }
  }

  pub fn get_placeholders_in_kind(&self, accum: &mut Vec<IdT<'s, 't>>, kind: KindT<'s, 't>) {
    match kind {
      KindT::Int(_) => {}
      KindT::Bool(_) => {}
      KindT::Float(_) => {}
      KindT::USize(_) => {}
      KindT::Void(_) => {}
      KindT::Never(_) => {}
      KindT::Str(_) => {}
      KindT::RuntimeSizedArray(rsa) => {
        self.get_placeholders_in_kind(accum, rsa.element_type());
      }
      KindT::StaticSizedArray(ssa) => {
        self.get_placeholders_in_templata(accum, ssa.size());
        self.get_placeholders_in_kind(accum, ssa.element_type());
      }
      KindT::Struct(s) => {
        let inst_name = IInstantiationNameT::try_from(s.id.local_name)
          .expect("StructTT id local_name must be an IInstantiationNameT");
        for arg in inst_name.template_args() {
          self.get_placeholders_in_templata(accum, *arg);
        }
      }
      KindT::Interface(i) => {
        let inst_name = IInstantiationNameT::try_from(i.id.local_name)
          .expect("InterfaceTT id local_name must be an IInstantiationNameT");
        for arg in inst_name.template_args() {
          self.get_placeholders_in_templata(accum, *arg);
        }
      }
      KindT::KindPlaceholder(p) => accum.push(p.id),
      KindT::OverloadSet(_) => {}
      KindT::BorrowRef(BorrowRefT { inner }) => {
        self.get_placeholders_in_kind(accum, *inner);
      }
      KindT::OwnRef(OwnRefT { inner }) => {
        self.get_placeholders_in_kind(accum, *inner);
      }
      KindT::ShareRef(ShareRefT { inner }) => {
        self.get_placeholders_in_kind(accum, *inner);
      }
      KindT::WeakRef(WeakRefT { inner }) => {
        self.get_placeholders_in_kind(accum, *inner);
      }
    }
  }

  pub fn sanity_check_conclusion(
    &self,
    envs: &InferEnv<'s, 't>,
    _state: &mut CompilerOutputs<'s, 't>,
    _rune: IRuneS<'s>,
    templata: ITemplataT<'s, 't>,
  ) {
    let mut accum: Vec<IdT<'s, 't>> = Vec::new();
    self.get_placeholders_in_templata(&mut accum, templata);

    if !accum.is_empty() {
      let root_denizen_env = envs.original_calling_env.root_compiling_denizen_env();
      let root_id = root_denizen_env.id();
      let original_calling_env_template_name: IdT<'s, 't> =
        match ITemplateNameT::try_from(root_id.local_name) {
          Ok(_x) => root_id,
          Err(_) => match IInstantiationNameT::try_from(root_id.local_name) {
            Ok(x) => *self.typing_interner.intern_id(IdValT {
              package_coord: root_id.package_coord,
              init_steps: root_id.init_steps,
              local_name: INameT::from(x.template()),
            }),
            Err(_) => panic!(
              "sanityCheckConclusion: unexpected root id local_name: {:?}",
              root_id.local_name
            ),
          },
        };
      let template_steps = original_calling_env_template_name.steps();
      for placeholder_name in &accum {
        let placeholder_steps = placeholder_name.steps();
        assert!(
          placeholder_steps.starts_with(&template_steps),
          "Placeholder {:?} steps don't start with template steps",
          placeholder_name
        );
      }
    }
  }

  // VCOORD: doublecheck before re-enabling. Dead today (its only caller is commented out in
  // compiler_solver.rs). The BorrowRef/OwnRef/ShareRef/WeakRef arms unimplemented!()-panic on
  // any ref-wrapped kind; peel references first.
  pub fn is_descendant_kind(
    &self,
    _envs: &InferEnv<'s, 't>,
    _coutputs: &mut CompilerOutputs<'s, 't>,
    kind: KindT<'s, 't>,
  ) -> bool {
    match kind {
      KindT::KindPlaceholder(kp) => self.is_descendant(
        _coutputs,
        _envs.parent_ranges,
        _envs.call_location,
        _envs.original_calling_env,
        ISubKindTT::KindPlaceholder(kp),
      ),
      KindT::RuntimeSizedArray(_) => false,
      KindT::OverloadSet(_) => false,
      KindT::Never(_) => true,
      KindT::StaticSizedArray(_) => false,
      KindT::Struct(s) => self.is_descendant(
        _coutputs,
        _envs.parent_ranges,
        _envs.call_location,
        _envs.original_calling_env,
        ISubKindTT::Struct(s),
      ),
      KindT::Interface(i) => self.is_descendant(
        _coutputs,
        _envs.parent_ranges,
        _envs.call_location,
        _envs.original_calling_env,
        ISubKindTT::Interface(i),
      ),
      KindT::Int(_)
      | KindT::Bool(_)
      | KindT::Float(_)
      | KindT::USize(_)
      | KindT::Str(_)
      | KindT::Void(_) => false,
      KindT::BorrowRef(_) => unimplemented!(),
      KindT::OwnRef(_) => unimplemented!(),
      KindT::ShareRef(_) => unimplemented!(),
      KindT::WeakRef(_) => unimplemented!(),
    }
  }

  // VCOORD: doublecheck before re-enabling. Dead today (its only caller is commented out in
  // compiler_solver.rs). A BorrowRef(Interface) or KindPlaceholder falls to _ => false and
  // silently drops an upcast candidate; peel references and handle placeholders first.
  pub fn is_ancestor_kind(
    &self,
    _envs: &InferEnv<'s, 't>,
    _coutputs: &mut CompilerOutputs<'s, 't>,
    kind: KindT<'s, 't>,
  ) -> bool {
    match kind {
      KindT::Interface(_) => true,
      _ => false,
    }
  }

  pub fn lookup_templata_imprecise(
    &self,
    envs: InferEnv<'s, 't>,
    state: &mut CompilerOutputs<'s, 't>,
    range: &[RangeS<'s>],
    name: IImpreciseNameS<'s>,
  ) -> Option<ITemplataT<'s, 't>> {
    self.lookup_templata_by_rune(envs.self_env, state, range, name)
  }

  pub fn predict_static_sized_array_kind(
    &self,
    _envs: InferEnv<'s, 't>,
    _state: &mut CompilerOutputs<'s, 't>,
    size: ITemplataT<'s, 't>,
    element: KindT<'s, 't>,
    region: RegionT,
  ) -> StaticSizedArrayTT<'s, 't> {
    self.resolve_static_sized_array(size, element, region)
  }

  pub fn predict_runtime_sized_array_kind(
    &self,
    _envs: InferEnv<'s, 't>,
    _state: &mut CompilerOutputs<'s, 't>,
    element: KindT<'s, 't>,
    region: RegionT,
  ) -> RuntimeSizedArrayTT<'s, 't> {
    self.resolve_runtime_sized_array(element, region)
  }

  pub fn kind_is_from_template(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    actual_citizen_ref: KindT<'s, 't>,
    expected_citizen_templata: ITemplataT<'s, 't>,
  ) -> bool {
    match actual_citizen_ref {
      KindT::RuntimeSizedArray(_) => {
        matches!(expected_citizen_templata, ITemplataT::RuntimeSizedArrayTemplate(_))
      }
      KindT::StaticSizedArray(_) => {
        matches!(expected_citizen_templata, ITemplataT::StaticSizedArrayTemplate(_))
      }
      other => match ICitizenTT::try_from(other) {
        Ok(s) => self.citizen_is_from_template(coutputs, s, expected_citizen_templata),
        Err(_) => false,
      },
    }
  }

  pub fn get_ancestors(
    &self,
    envs: InferEnv<'s, 't>,
    coutputs: &mut CompilerOutputs<'s, 't>,
    descendant: KindT<'s, 't>,
    include_self: bool,
  ) -> HashSet<KindT<'s, 't>> {
    let mut result: HashSet<KindT<'s, 't>> = HashSet::default();
    if include_self {
      result.insert(descendant);
    }
    match ISubKindTT::try_from(descendant) {
      Ok(s) => {
        for parent in self.get_parents(
          coutputs,
          envs.parent_ranges,
          envs.call_location,
          envs.original_calling_env,
          s,
        ) {
          result.insert(KindT::from(parent));
        }
      }
      Err(_) => {}
    }
    result
  }

  pub fn struct_is_closure(
    &self,
    _state: &mut CompilerOutputs<'s, 't>,
    _struct_tt: StructTT<'s, 't>,
  ) -> bool {
    panic!("Unimplemented: struct_is_closure");
    // val structDef = state.lookupStruct(structTT.id)
    // structDef.isClosure
  }

  pub fn predict_function(
    &self,
    envs: InferEnv<'s, 't>,
    _state: &mut CompilerOutputs<'s, 't>,
    _function_range: RangeS<'s>,
    name: StrI<'s>,
    param_coords: &'t [KindT<'s, 't>],
    return_coord: KindT<'s, 't>,
  ) -> PrototypeTemplataT<'s, 't> {
    let tmpl = self
      .typing_interner
      .intern_predicted_function_template_name(PredictedFunctionTemplateNameT { human_name: name });
    let pred_name =
      self.typing_interner.intern_predicted_function_name(PredictedFunctionNameValT {
        template: tmpl,
        template_args: &[],
        parameters: param_coords,
      });
    let id = envs
      .original_calling_env
      .denizen_id()
      .add_step(self.typing_interner, INameT::PredictedFunction(pred_name));
    let prototype = self.typing_interner.intern_prototype(PrototypeValT {
      id: IdValT {
        package_coord: id.package_coord,
        init_steps: id.init_steps,
        local_name: id.local_name,
      },
      return_type: return_coord,
    });
    PrototypeTemplataT { prototype }
  }

  pub fn assemble_prototype(
    &self,
    envs: InferEnv<'s, 't>,
    state: &mut CompilerOutputs<'s, 't>,
    _range: RangeS<'s>,
    name: StrI<'s>,
    coords: &'t [KindT<'s, 't>],
    return_type: KindT<'s, 't>,
  ) -> &'t PrototypeT<'s, 't> {
    let tmpl = self
      .typing_interner
      .intern_function_bound_template_name(FunctionBoundTemplateNameT { human_name: name });
    let bound_name = self.typing_interner.intern_function_bound_name(FunctionBoundNameValT {
      template: tmpl,
      template_args: &[],
      parameters: coords,
    });
    let id = envs
      .original_calling_env
      .denizen_id()
      .add_step(self.typing_interner, INameT::FunctionBound(bound_name));
    let result = self.typing_interner.intern_prototype(PrototypeValT {
      id: IdValT {
        package_coord: id.package_coord,
        init_steps: id.init_steps,
        local_name: id.local_name,
      },
      return_type,
    });
    // This is a function bound, and there's no such thing as a function bound with function bounds.
    let empty_bounds = self.typing_interner.alloc(InstantiationBoundArgumentsT {
      rune_to_bound_prototype: self.typing_interner.alloc_index_map_from_iter(empty()),
      rune_to_citizen_rune_to_reachable_prototype: self
        .typing_interner
        .alloc_index_map_from_iter(empty()),
      rune_to_bound_impl: self.typing_interner.alloc_index_map_from_iter(empty()),
    });
    state.add_instantiation_bounds(
      self.opts.global_options.sanity_check,
      self.typing_interner,
      envs.original_calling_env.denizen_template_id(),
      result.id,
      empty_bounds,
    );
    result
  }

  pub fn evaluate_generic_function_from_non_call_for_header(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    parent_ranges: &[RangeS<'s>],
    call_location: LocationInDenizen<'s>,
    function_templata: FunctionTemplataT<'s, 't>,
  ) -> Result<&'t FunctionHeaderT<'s, 't>, ICompileErrorT<'s, 't>> {
    self.evaluate_generic_function_from_non_call(
      coutputs,
      parent_ranges,
      call_location,
      function_templata,
    )
  }

  pub fn scout_expected_function_for_prototype(
    &self,
    _env: IInDenizenEnvironmentT<'s, 't>,
    _coutputs: &mut CompilerOutputs<'s, 't>,
    _call_range: &[RangeS<'s>],
    _call_location: LocationInDenizen<'s>,
    _function_name: IImpreciseNameS<'s>,
    _explicit_template_arg_rules_s: &[IRulexSR<'s>],
    _explicit_template_arg_runes_s: &[IRuneS<'s>],
    _context_region: RegionT,
    _args: &[KindT<'s, 't>],
    _extra_envs_to_look_in: &[IInDenizenEnvironmentT<'s, 't>],
    _exact: bool,
  ) -> StampFunctionSuccess<'s, 't> {
    panic!("Unimplemented: scout_expected_function_for_prototype");
    // overloadResolver.findFunction(
    //   env, coutputs, callRange, callLocation, functionName,
    //   explicitTemplateArgRulesS, positionalExplicitTemplateArgRunesS,
    //   receivingRuneToExplicitTemplateArgRune, contextRegion,
    //   args, extraEnvsToLookIn, exact) match {
    //   case Err(e) => throw CompileErrorExceptionT(CouldntFindFunctionToCallT(callRange, e))
    //   case Ok(x) => x
    // }
  }

  pub fn generate_function(
    &self,
    _generator: IFunctionGenerator,
    _full_env: &'t FunctionEnvironmentT<'s, 't>,
    _coutputs: &mut CompilerOutputs<'s, 't>,
    _life: LocationInFunctionEnvironmentT<'t>,
    _call_range: &[RangeS<'s>],
    _origin_function: Option<&'s FunctionS<'s>>,
    _param_coords: &[ParameterT<'s, 't>],
    _maybe_ret_coord: Option<KindT<'s, 't>>,
  ) -> &'t FunctionHeaderT<'s, 't> {
    panic!("Unimplemented: generate_function");
    // generator.generate(
    //   functionCompilerCore, structCompiler, destructorCompiler, arrayCompiler,
    //   fullEnv, coutputs, life, callRange, originFunction, paramCoords, maybeRetCoord)
  }

  // Total accessor for a function's postparsed AST, keyed by its template id. Callers cannot observe
  // whether it was already built: this always returns, or vfails on a compiler bug. A Rust function
  // builds on its first lookup here (the #[cfg] create hook is wired in the lazy step); a Vale
  // function is always already seeded at index time. This is the one seam through which every
  // function read passes, so no read site ever touches the sealed table directly. See the sealing
  // VCOORD in compiler_outputs.rs. Struct/interface/impl are always eager, so their reads stay on the
  // total coutputs.get_postparsed_* accessors, which likewise never reveal existence.
  pub(in crate::typing) fn get_or_create_postparsed_function(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
    template_id: &'t IdT<'s, 't>,
  ) -> &'s FunctionS<'s> {
    if let Some(f) = coutputs.peek_postparsed_function(template_id) {
      return f;
    }
    #[cfg(feature = "rust_interop")]
    {
      if let Some(f) = create_postparsed_function(self, coutputs, template_id) {
        return f;
      }
    }
    panic!("vfail: no postparsed function for {:?}", template_id);
  }

  pub fn evaluate<'p>(
    &self,
    _code_map: &FileCoordinateMap<'p, String>,
    file_to_program_s: &FileCoordinateMap<'s, ProgramS<'s>>,
  ) -> Result<HinputsT<'s, 't>, ICompileErrorT<'s, 't>> {
    let name_to_struct_defined_macro: HashMap<StrI<'s>, OnStructDefinedMacro> = {
      let mut m = HashMap::default();
      m.insert(self.keywords.derive_struct_constructor, OnStructDefinedMacro::StructConstructor);
      m.insert(self.keywords.derive_struct_drop, OnStructDefinedMacro::StructDrop);
      m
    };
    let name_to_interface_defined_macro: HashMap<StrI<'s>, OnInterfaceDefinedMacro> = {
      let mut m = HashMap::default();
      m.insert(self.keywords.derive_interface_drop, OnInterfaceDefinedMacro::InterfaceDrop);
      m.insert(
        self.keywords.derive_anonymous_substruct,
        OnInterfaceDefinedMacro::AnonymousInterface,
      );
      m
    };

    let mut template_id_to_postparsed_function: IndexMap<&'t IdT<'s, 't>, &'s FunctionS<'s>> =
      IndexMap::default();
    let mut template_id_to_postparsed_struct: IndexMap<&'t IdT<'s, 't>, &'s StructS<'s>> =
      IndexMap::default();
    let mut template_id_to_postparsed_interface: IndexMap<&'t IdT<'s, 't>, &'s InterfaceS<'s>> =
      IndexMap::default();
    let mut template_id_to_postparsed_impl: IndexMap<&'t IdT<'s, 't>, &'s ImplS<'s>> =
      IndexMap::default();
    let mut id_and_env_entry: Vec<(&'t IdT<'s, 't>, IEnvEntryT<'s, 't>)> = Vec::new();
    // A package's denizens can be spread across several files; each is registered under its
    // own file's package coord, so the same package is simply visited once per file.
    for (file_coord, program_a) in &file_to_program_s.file_coord_to_contents {
      let coord = file_coord.package_coord;
      let pkg_top_level_name =
        self.typing_interner.intern_package_top_level_name(PackageTopLevelNameT {});
      let pkg_top_level = INameT::PackageTopLevel(pkg_top_level_name);
      for struct_a in program_a.structs.iter() {
        let struct_template_name = self.translate_struct_name(struct_a.name);
        let struct_name_local: INameT<'s, 't> = match struct_template_name {
          IStructTemplateNameT::StructTemplate(r) => INameT::StructTemplate(r),
          IStructTemplateNameT::AnonymousSubstructTemplate(r) => {
            INameT::AnonymousSubstructTemplate(r)
          }
          IStructTemplateNameT::LambdaCitizenTemplate(_) => {
            panic!("Unimplemented: LambdaCitizenTemplate in struct translation")
          }
        };
        let package_name = self.typing_interner.intern_id(IdValT {
          package_coord: coord,
          init_steps: &[],
          local_name: pkg_top_level,
        });
        let struct_name_t = package_name.add_step(self.typing_interner, struct_name_local);
        template_id_to_postparsed_struct.insert(struct_name_t, struct_a);
        id_and_env_entry.push((
          struct_name_t,
          IEnvEntryT::Struct(StructEnvEntry { template_id: struct_name_t, tyype: struct_a.tyype }),
        ));
        for internal_method in struct_a.internal_methods.iter() {
          let (_, method_template_id) =
            self.internal_method_template_id(struct_name_t, internal_method);
          template_id_to_postparsed_function.insert(method_template_id, internal_method);
        }
        for aht_denizen in
          self.preprocess_struct(&name_to_struct_defined_macro, *struct_name_t, struct_a)
        {
          id_and_env_entry.push((aht_denizen.template_id(), aht_denizen.env_entry()));
          match aht_denizen {
            GeneratedAhtDenizen::Function(id, f) => {
              template_id_to_postparsed_function.insert(id, f);
            }
            GeneratedAhtDenizen::Struct(id, s) => {
              template_id_to_postparsed_struct.insert(id, s);
            }
            GeneratedAhtDenizen::Impl(id, i) => {
              template_id_to_postparsed_impl.insert(id, i);
            }
          }
        }
      }
      for interface_a in program_a.interfaces.iter() {
        let interface_template_name = self.translate_interface_name(*interface_a.name);
        let interface_name_local: INameT<'s, 't> = match interface_template_name {
          IInterfaceTemplateNameT::InterfaceTemplate(r) => INameT::InterfaceTemplate(r),
        };
        let package_name = self.typing_interner.intern_id(IdValT {
          package_coord: coord,
          init_steps: &[],
          local_name: pkg_top_level,
        });
        let interface_name_t = package_name.add_step(self.typing_interner, interface_name_local);
        template_id_to_postparsed_interface.insert(interface_name_t, interface_a);
        id_and_env_entry.push((
          interface_name_t,
          IEnvEntryT::Interface(InterfaceEnvEntry {
            template_id: interface_name_t,
            tyype: interface_a.tyype,
          }),
        ));
        for internal_method in interface_a.internal_methods.iter() {
          let (_, method_template_id) =
            self.internal_method_template_id(interface_name_t, internal_method);
          template_id_to_postparsed_function.insert(method_template_id, internal_method);
        }
        for aht_denizen in self.preprocess_interface(
          &name_to_interface_defined_macro,
          *interface_name_t,
          interface_a,
        ) {
          id_and_env_entry.push((aht_denizen.template_id(), aht_denizen.env_entry()));
          match aht_denizen {
            GeneratedAhtDenizen::Function(id, f) => {
              template_id_to_postparsed_function.insert(id, f);
            }
            GeneratedAhtDenizen::Struct(id, s) => {
              template_id_to_postparsed_struct.insert(id, s);
            }
            GeneratedAhtDenizen::Impl(id, i) => {
              template_id_to_postparsed_impl.insert(id, i);
            }
          }
        }
      }
      for impl_a in program_a.impls.iter() {
        let impl_template_name = self.translate_impl_name(
          impl_a.name,
          impl_a.sub_citizen_imprecise_name,
          impl_a.super_interface_imprecise_name,
        );
        let impl_name_local: INameT<'s, 't> = match impl_template_name {
          IImplTemplateNameT::ImplTemplate(r) => INameT::ImplTemplate(r),
          IImplTemplateNameT::ImplBoundTemplate(_) => {
            panic!("Unimplemented: ImplBoundTemplate in impl translation")
          }
          IImplTemplateNameT::AnonymousSubstructImplTemplate(_) => {
            panic!("Unimplemented: AnonymousSubstructImplTemplate in impl translation")
          }
        };
        let package_name = self.typing_interner.intern_id(IdValT {
          package_coord: coord,
          init_steps: &[],
          local_name: pkg_top_level,
        });
        let impl_name_t = package_name.add_step(self.typing_interner, impl_name_local);
        template_id_to_postparsed_impl.insert(impl_name_t, impl_a);
        id_and_env_entry
          .push((impl_name_t, IEnvEntryT::Impl(ImplEnvEntry { template_id: impl_name_t })));
      }
      for function_a in program_a.implemented_functions.iter() {
        let function_template_name = self.translate_generic_function_name(function_a.name);
        let function_name_local: INameT<'s, 't> = match function_template_name {
          IFunctionTemplateNameT::FunctionTemplate(r) => INameT::FunctionTemplate(r),
          IFunctionTemplateNameT::ForwarderFunctionTemplate(r) => {
            INameT::ForwarderFunctionTemplate(r)
          }
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
          IFunctionTemplateNameT::PredictedFunctionTemplate(r) => {
            INameT::PredictedFunctionTemplate(r)
          }
        };
        let package_name = self.typing_interner.intern_id(IdValT {
          package_coord: coord,
          init_steps: &[],
          local_name: pkg_top_level,
        });
        let function_name_t = package_name.add_step(self.typing_interner, function_name_local);
        template_id_to_postparsed_function.insert(function_name_t, function_a);
        id_and_env_entry.push((
          function_name_t,
          IEnvEntryT::Function(FunctionEnvEntry { template_id: function_name_t }),
        ));
      }
    }

    let pkg_top_level_for_group = INameT::PackageTopLevel(
      self.typing_interner.intern_package_top_level_name(PackageTopLevelNameT {}),
    );
    // Per @IIIOZ: IndexMap so iteration at line ~1350 (into global_env.name_to_top_level_environment)
    // preserves id_and_env_entry source order — otherwise the package env's `global_namespaces`
    // slice ends up in random per-process HashMap order, and lookups that walk it nondeterministically
    // pick a different "drop" overload per run.
    let mut namespace_name_to_entries: IndexMap<
      &'t IdT<'s, 't>,
      Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)>,
    > = IndexMap::default();
    for (name, env_entry) in &id_and_env_entry {
      let package_id = self.typing_interner.intern_id(IdValT {
        package_coord: name.package_coord,
        init_steps: name.init_steps,
        local_name: pkg_top_level_for_group,
      });
      namespace_name_to_entries
        .entry(package_id)
        .or_insert_with(Vec::new)
        .push((name.local_name, *env_entry));
    }
    let mut namespace_name_to_templatas_vec: Vec<(&'t IdT<'s, 't>, &'t TemplatasStoreT<'s, 't>)> =
      Vec::new();
    for (package_id, entries) in namespace_name_to_entries {
      let mut builder = TemplatasStoreBuilder::new(package_id);
      builder.add_entries(self.scout_arena, entries);
      namespace_name_to_templatas_vec.push((package_id, builder.build_in(self.typing_interner)));
    }

    let builtin_coord: &'s PackageCoordinate<'s> =
      self.scout_arena.intern_package_coordinate(self.keywords.empty_string, &[]);
    let builtin_id = self.typing_interner.intern_id(IdValT {
      package_coord: builtin_coord,
      init_steps: &[],
      local_name: INameT::PackageTopLevel(
        self.typing_interner.intern_package_top_level_name(PackageTopLevelNameT {}),
      ),
    });
    let mut builtins_builder = TemplatasStoreBuilder::new(builtin_id);
    let primitives: &[(StrI<'s>, KindT<'s, 't>)] = &[
      (self.keywords.int, KindT::Int(IntT::I32)),
      (self.keywords.i64, KindT::Int(IntT::I64)),
      (self.keywords.bool, KindT::Bool(BoolT)),
      (self.keywords.float, KindT::Float(FloatT)),
      (self.keywords.usize, KindT::USize(USizeT)),
      (self.keywords.__never, KindT::Never(NeverT { from_break: false })),
      (self.keywords.str, KindT::Str(StrT)),
      (self.keywords.void, KindT::Void(VoidT)),
    ];
    for (human_name, kind) in primitives {
      let prim = INameT::Primitive(
        self.typing_interner.intern_primitive_name(PrimitiveNameT { human_name: *human_name }),
      );
      let kind_t = ITemplataT::Kind(self.typing_interner.alloc(KindTemplataT { kind: *kind }));
      builtins_builder.name_to_entry.push((prim, IEnvEntryT::Templata(kind_t)));
      if let Some(imprecise) = get_imprecise_name(self.scout_arena, prim) {
        builtins_builder
          .imprecise_to_entries
          .entry(imprecise)
          .or_insert_with(Vec::new)
          .push(IEnvEntryT::Templata(kind_t));
      }
    }
    {
      let prim = INameT::Primitive(
        self
          .typing_interner
          .intern_primitive_name(PrimitiveNameT { human_name: self.keywords.array }),
      );
      let entry = IEnvEntryT::Templata(ITemplataT::RuntimeSizedArrayTemplate(
        RuntimeSizedArrayTemplateTemplataT {},
      ));
      builtins_builder.name_to_entry.push((prim, entry));
      if let Some(imprecise) = get_imprecise_name(self.scout_arena, prim) {
        builtins_builder.imprecise_to_entries.entry(imprecise).or_insert_with(Vec::new).push(entry);
      }
    }
    {
      let prim = INameT::Primitive(
        self
          .typing_interner
          .intern_primitive_name(PrimitiveNameT { human_name: self.keywords.static_array }),
      );
      let entry = IEnvEntryT::Templata(ITemplataT::StaticSizedArrayTemplate(
        StaticSizedArrayTemplateTemplataT {},
      ));
      builtins_builder.name_to_entry.push((prim, entry));
      if let Some(imprecise) = get_imprecise_name(self.scout_arena, prim) {
        builtins_builder.imprecise_to_entries.entry(imprecise).or_insert_with(Vec::new).push(entry);
      }
    }
    let builtins = builtins_builder.build_in(self.typing_interner);

    // Handle `import rust.whatever` imports.
    #[cfg(feature = "rust_interop")]
    {
      if let Some((id, _)) =
        namespace_name_to_templatas_vec.iter().find(|(id, _)| is_rust_backed(id))
      {
        panic!("Overlap, a Vale package claimed the reserved `rust` module: {id:?}");
      }
      // Loop over all `import rust.whatever` imports, and turn each into an env entry (and eagerly postparse it
      // if it's a type).
      let mut per_crate: IndexMap<
        &'s PackageCoordinate<'s>,
        Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)>,
      > = IndexMap::default();
      for program in file_to_program_s.file_coord_to_contents.values() {
        for import in program.imports {
          if import.module_name != self.keywords.rust {
            continue;
          }
          // Oracle must be present if the user is doing any `import rust` things.
          let oracle = self
            .oracles
            .rust
            // VCOORD: make this into an error?
            .expect("an `import rust.…` statement, but no Rust oracle was provided");
          let name = match oracle.resolve_import(import) {
            Some(name) => name,
            None => {
              let mut path =
                import.package_names.iter().map(|s| format!("{}.", s.0)).collect::<String>();
              path.push_str(import.importee_name.0);
              return Err(ICompileErrorT::UnresolvableRustImport {
                range: self.typing_interner.alloc_slice_from_vec(vec![import.range]),
                path,
              });
            }
          };
          let (local_name, entry, seed) = declare_rust_import(self, name);
          match seed {
            Some(RustImportSeed::Struct(id, s)) => {
              template_id_to_postparsed_struct.insert(id, s);
            }
            Some(RustImportSeed::Interface(id, i)) => {
              template_id_to_postparsed_interface.insert(id, i);
            }
            None => {}
          }
          let coord =
            self.scout_arena.intern_package_coordinate(name.module_name, name.package_names);
          per_crate.entry(coord).or_default().push((local_name, entry));
        }
      }
      for (coord, entries) in per_crate {
        let package_id = self.typing_interner.intern_id(IdValT {
          package_coord: coord,
          init_steps: &[],
          local_name: INameT::PackageTopLevel(
            self.typing_interner.intern_package_top_level_name(PackageTopLevelNameT {}),
          ),
        });
        let mut store = TemplatasStoreBuilder::new(package_id);
        store.add_entries(self.scout_arena, entries);
        namespace_name_to_templatas_vec.push((package_id, store.build_in(self.typing_interner)));
      }
    }

    let name_to_top_level_environment =
      self.typing_interner.alloc_slice_from_vec(namespace_name_to_templatas_vec);

    let mut name_to_function_body_macro =
      self.typing_interner.alloc_index_map::<StrI<'s>, FunctionBodyMacro>();
    name_to_function_body_macro
      .insert(self.keywords.abstract_body, FunctionBodyMacro::AbstractBody);
    name_to_function_body_macro
      .insert(self.keywords.struct_constructor_generator, FunctionBodyMacro::StructConstructor);
    name_to_function_body_macro.insert(self.keywords.drop_generator, FunctionBodyMacro::StructDrop);
    name_to_function_body_macro
      .insert(self.keywords.vale_runtime_sized_array_len, FunctionBodyMacro::RsaLen);
    name_to_function_body_macro
      .insert(self.keywords.vale_runtime_sized_array_new, FunctionBodyMacro::RsaNew);
    name_to_function_body_macro
      .insert(self.keywords.vale_runtime_sized_array_push, FunctionBodyMacro::RsaPush);
    name_to_function_body_macro
      .insert(self.keywords.vale_runtime_sized_array_pop, FunctionBodyMacro::RsaPop);
    name_to_function_body_macro
      .insert(self.keywords.vale_runtime_sized_array_capacity, FunctionBodyMacro::RsaCapacity);
    name_to_function_body_macro
      .insert(self.keywords.vale_static_sized_array_len, FunctionBodyMacro::SsaLen);
    name_to_function_body_macro
      .insert(self.keywords.vale_runtime_sized_array_drop_into, FunctionBodyMacro::RsaDropInto);
    name_to_function_body_macro
      .insert(self.keywords.vale_static_sized_array_drop_into, FunctionBodyMacro::SsaDropInto);
    name_to_function_body_macro.insert(self.keywords.vale_lock_weak, FunctionBodyMacro::LockWeak);
    name_to_function_body_macro
      .insert(self.keywords.vale_same_instance, FunctionBodyMacro::SameInstance);
    name_to_function_body_macro.insert(self.keywords.vale_as_subtype, FunctionBodyMacro::AsSubtype);

    let global_env: &'t GlobalEnvironmentT<'s, 't> =
      self.typing_interner.alloc(GlobalEnvironmentT {
        name_to_top_level_environment,
        name_to_function_body_macro,
        builtins,
      });

    let mut coutputs = CompilerOutputs::new(
      template_id_to_postparsed_function,
      template_id_to_postparsed_struct,
      template_id_to_postparsed_interface,
      template_id_to_postparsed_impl,
    );

    self.compile_static_sized_array(global_env, &mut coutputs);
    self.compile_runtime_sized_array(global_env, &mut coutputs);

    // Indexing phase
    for (package_id, templatas) in global_env.name_to_top_level_environment {
      let env = make_top_level_environment(global_env, **package_id, self.typing_interner);
      let env_ref: IEnvironmentT<'s, 't> = IEnvironmentT::Package(env);
      for (_name, entry) in templatas.name_to_entry.iter() {
        match entry {
          IEnvEntryT::Struct(StructEnvEntry { template_id: id, tyype }) => {
            let templata = StructDefinitionTemplataT {
              declaring_env: env_ref,
              struct_template_id: id,
              tyype: *tyype,
            };
            self.precompile_struct(&mut coutputs, templata);
          }
          IEnvEntryT::Interface(InterfaceEnvEntry { template_id: id, tyype }) => {
            let templata = InterfaceDefinitionTemplataT {
              declaring_env: env_ref,
              interface_template_id: id,
              tyype: *tyype,
            };
            self.precompile_interface(&mut coutputs, templata);
          }
          _ => {}
        }
      }
    }

    // Compiling phase
    let mut unchecked_defining_conclusionses: Vec<UncheckedDefiningConclusions<'s, 't>> =
      Vec::new();
    for (package_id, templatas) in global_env.name_to_top_level_environment {
      let env = make_top_level_environment(global_env, **package_id, self.typing_interner);
      let env_ref: IEnvironmentT<'s, 't> = IEnvironmentT::Package(env);
      // This makes it so anything starting with an underscore is compiled in the order
      // of their names.
      // AFTERM: is there a better solution here? should we always order things?
      let mut orderable_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> = Vec::new();
      let mut unordered_entries: Vec<(INameT<'s, 't>, IEnvEntryT<'s, 't>)> = Vec::new();
      for (name, entry) in templatas.name_to_entry.iter() {
        match name {
          INameT::StructTemplate(s) if s.human_name.0.starts_with("_") => {
            orderable_entries.push((*name, *entry))
          }
          INameT::InterfaceTemplate(i) if i.human_namee.0.starts_with("_") => {
            orderable_entries.push((*name, *entry))
          }
          _ => unordered_entries.push((*name, *entry)),
        }
      }
      // orderedEntries = orderableEntries.sortBy(_._1.humanName.str)
      orderable_entries.sort_by(|(a, _), (b, _)| {
        CitizenTemplateNameT::try_from(*a)
          .unwrap()
          .human_name()
          .0
          .cmp(CitizenTemplateNameT::try_from(*b).unwrap().human_name().0)
      });
      let all_entries = orderable_entries.into_iter().chain(unordered_entries.into_iter());
      for (_name, entry) in all_entries {
        match entry {
          IEnvEntryT::Struct(StructEnvEntry { template_id: id, tyype }) => {
            let templata =
              StructDefinitionTemplataT { declaring_env: env_ref, struct_template_id: id, tyype };
            let unchecked_conclusions =
              self.compile_struct(&mut coutputs, &[], LocationInDenizen { path: &[] }, templata)?;
            let struct_a = coutputs.get_postparsed_struct(id);
            let maybe_export = struct_a.attributes.iter().find_map(|a| match a {
              ICitizenAttributeS::Export(e) => Some(e),
              _ => None,
            });
            match maybe_export {
              None => {}
              Some(export_s) => {
                let template_name =
                  self.typing_interner.intern_export_template_name(ExportTemplateNameT {
                    code_loc: struct_a.range.begin,
                  });
                let template_id_steps: Vec<INameT<'s, 't>> = vec![];
                let template_id = *self.typing_interner.intern_id(IdValT {
                  package_coord: package_id.package_coord,
                  init_steps: &template_id_steps,
                  local_name: INameT::ExportTemplate(template_name),
                });
                let template_id_ref = self.typing_interner.alloc(template_id);
                let export_outer_templatas =
                  TemplatasStoreBuilder::new(template_id_ref).build_in(self.typing_interner);
                let _export_outer_env = self.typing_interner.alloc(ExportEnvironmentT {
                  global_env,
                  parent_env: env,
                  template_id,
                  id: template_id,
                  templatas: export_outer_templatas,
                });
                let placeholdered_export_name =
                  self.typing_interner.intern_export_name(ExportNameT {
                    template: template_name,
                  });
                let placeholdered_export_id_steps: Vec<INameT<'s, 't>> = vec![];
                let placeholdered_export_id = *self.typing_interner.intern_id(IdValT {
                  package_coord: package_id.package_coord,
                  init_steps: &placeholdered_export_id_steps,
                  local_name: INameT::Export(placeholdered_export_name),
                });
                let placeholdered_export_id_ref =
                  self.typing_interner.alloc(placeholdered_export_id);
                let export_templatas = TemplatasStoreBuilder::new(placeholdered_export_id_ref)
                  .build_in(self.typing_interner);
                let export_env = self.typing_interner.alloc(ExportEnvironmentT {
                  global_env,
                  parent_env: env,
                  template_id,
                  id: placeholdered_export_id,
                  templatas: export_templatas,
                });
                let export_env_as_iindenizen = IInDenizenEnvironmentT::Export(export_env);
                let export_call_range = self.typing_interner.alloc_slice_copy(&[struct_a.range]);
                let export_placeholdered_struct = match self.resolve_struct(
                  &mut coutputs,
                  export_env_as_iindenizen,
                  export_call_range,
                  LocationInDenizen { path: &[] },
                  templata,
                  &[],
                ) {
                  IResolveOutcome::ResolveSuccess(s) => self.typing_interner.alloc(s.kind),
                  IResolveOutcome::ResolveFailure(_f) => {
                    panic!("vwat: resolve struct failed for export")
                  }
                };
                let export_name = match struct_a.name {
                  IStructDeclarationNameS::TopLevelStructDeclarationName(n) => n.name,
                  IStructDeclarationNameS::AnonymousSubstructTemplateName(_) => {
                    panic!("vwat: anonymous substruct in export handler")
                  }
                };
                coutputs.add_kind_export(
                  struct_a.range,
                  KindT::Struct(export_placeholdered_struct),
                  placeholdered_export_id,
                  export_name,
                  self.typing_interner,
                );
              }
            }
            unchecked_defining_conclusionses.push(unchecked_conclusions);
          }
          IEnvEntryT::Interface(InterfaceEnvEntry { template_id: id, tyype }) => {
            let templata = InterfaceDefinitionTemplataT {
              declaring_env: env_ref,
              interface_template_id: id,
              tyype,
            };
            let unchecked_conclusions = self.compile_interface(
              &mut coutputs,
              &[],
              LocationInDenizen { path: &[] },
              templata,
            )?;
            let interface_a = coutputs.get_postparsed_interface(id);
            let maybe_export = interface_a.attributes.iter().find_map(|a| match a {
              ICitizenAttributeS::Export(e) => Some(e),
              _ => None,
            });
            match maybe_export {
              None => {}
              Some(_export_s) => {
                let template_name =
                  self.typing_interner.intern_export_template_name(ExportTemplateNameT {
                    code_loc: interface_a.range.begin,
                  });
                let template_id_steps: Vec<INameT<'s, 't>> = vec![];
                let template_id = *self.typing_interner.intern_id(IdValT {
                  package_coord: package_id.package_coord,
                  init_steps: &template_id_steps,
                  local_name: INameT::ExportTemplate(template_name),
                });
                let template_id_ref = self.typing_interner.alloc(template_id);
                let export_outer_templatas =
                  TemplatasStoreBuilder::new(template_id_ref).build_in(self.typing_interner);
                let _export_outer_env = self.typing_interner.alloc(ExportEnvironmentT {
                  global_env,
                  parent_env: env,
                  template_id,
                  id: template_id,
                  templatas: export_outer_templatas,
                });
                let placeholdered_export_name =
                  self.typing_interner.intern_export_name(ExportNameT {
                    template: template_name,
                  });
                let placeholdered_export_id_steps: Vec<INameT<'s, 't>> = vec![];
                let placeholdered_export_id = *self.typing_interner.intern_id(IdValT {
                  package_coord: package_id.package_coord,
                  init_steps: &placeholdered_export_id_steps,
                  local_name: INameT::Export(placeholdered_export_name),
                });
                let placeholdered_export_id_ref =
                  self.typing_interner.alloc(placeholdered_export_id);
                let export_templatas = TemplatasStoreBuilder::new(placeholdered_export_id_ref)
                  .build_in(self.typing_interner);
                let export_env = self.typing_interner.alloc(ExportEnvironmentT {
                  global_env,
                  parent_env: env,
                  template_id,
                  id: placeholdered_export_id,
                  templatas: export_templatas,
                });
                let export_env_as_iindenizen = IInDenizenEnvironmentT::Export(export_env);
                let export_call_range = self.typing_interner.alloc_slice_copy(&[interface_a.range]);
                let export_placeholdered_kind = match self.resolve_interface(
                  &mut coutputs,
                  export_env_as_iindenizen,
                  export_call_range,
                  LocationInDenizen { path: &[] },
                  templata,
                  &[],
                ) {
                  IResolveOutcome::ResolveSuccess(s) => self.typing_interner.alloc(s.kind),
                  IResolveOutcome::ResolveFailure(_f) => {
                    panic!("vwat: resolve interface failed for export")
                  }
                };
                let export_name = interface_a.name.name;
                coutputs.add_kind_export(
                  interface_a.range,
                  KindT::Interface(export_placeholdered_kind),
                  placeholdered_export_id,
                  export_name,
                  self.typing_interner,
                );
              }
            }
            unchecked_defining_conclusionses.push(unchecked_conclusions);
          }
          _ => {}
        }
      }
    }

    // Struct/interface resolution phase
    for unchecked in unchecked_defining_conclusionses.into_iter() {
      let _instantiation_bound_args_unused = match self.check_defining_conclusions_and_resolve(
        unchecked.envs,
        &mut coutputs,
        &unchecked.ranges,
        unchecked.call_location,
        &unchecked.definition_rules,
        &[],
        &unchecked.conclusions,
      ) {
        Err(_f) => {
          panic!("implement: check_defining_conclusions_and_resolve error in resolution phase");
          // throw CompileErrorExceptionT(TypingPassDefiningError(ranges, DefiningResolveConclusionError(f)))
        }
        Ok(c) => c,
      };
    }

    // Impl compile phase
    for (package_id, templatas) in global_env.name_to_top_level_environment {
      let package_env = make_top_level_environment(global_env, **package_id, self.typing_interner);
      let package_env_t: IEnvironmentT<'s, 't> = IEnvironmentT::Package(package_env);
      for (_name, entry) in templatas.name_to_entry.iter() {
        match entry {
          IEnvEntryT::Impl(ImplEnvEntry { template_id: id, .. }) => {
            let impl_templata = self
              .typing_interner
              .alloc(ImplDefinitionTemplataT { env: package_env_t, impl_template_id: id });
            self.compile_impl(&mut coutputs, LocationInDenizen { path: &[] }, *impl_templata)?;
          }
          _ => {}
        }
      }
    }

    // Function compile phase
    for (package_id, templatas) in global_env.name_to_top_level_environment {
      if !package_id.init_steps.is_empty() {
        continue;
      }
      // Skip the whole `rust` package, because we lazily postparse rust methods
      // when their postparseds are requested.
      // VRI: consider some other thing to loop over?
      // VRI: consider making vale lazy in some way too.
      #[cfg(feature = "rust_interop")]
      {
        if is_rust_backed(package_id) {
          continue;
        }
      }
      let global_namespaces: Vec<&TemplatasStoreT<'s, 't>> =
        global_env.name_to_top_level_environment.iter().map(|(_, ts)| *ts).collect();
      let global_namespaces = self.typing_interner.alloc_slice_from_vec(global_namespaces);
      let package_env = self.typing_interner.alloc(PackageEnvironmentT {
        global_env,
        id: **package_id,
        global_namespaces,
      });
      let package_env_t: IEnvironmentT<'s, 't> = IEnvironmentT::Package(package_env);
      for (_name, entry) in templatas.name_to_entry.iter() {
        match entry {
          IEnvEntryT::Function(FunctionEnvEntry { template_id: id }) => {
            let templata = FunctionTemplataT { outer_env: package_env_t, function_template_id: id };
            let _header = self.evaluate_generic_function_from_non_call(
              &mut coutputs,
              &[],
              LocationInDenizen { path: &[] },
              templata,
            )?;
            let function_a = self.get_or_create_postparsed_function(&mut coutputs, id);
            let maybe_export = function_a.attributes.iter().find_map(|a| match a {
              IFunctionAttributeS::Export(e) => Some(e),
              _ => None,
            });
            match maybe_export {
              None => {}
              Some(_export_s) => {
                let template_name =
                  self.typing_interner.intern_export_template_name(ExportTemplateNameT {
                    code_loc: function_a.range.begin,
                  });
                let template_id_steps: Vec<INameT<'s, 't>> = vec![];
                let template_id = *self.typing_interner.intern_id(IdValT {
                  package_coord: package_id.package_coord,
                  init_steps: &template_id_steps,
                  local_name: INameT::ExportTemplate(template_name),
                });
                let template_id_ref = self.typing_interner.alloc(template_id);
                let export_outer_templatas =
                  TemplatasStoreBuilder::new(template_id_ref).build_in(self.typing_interner);
                let _export_outer_env = self.typing_interner.alloc(ExportEnvironmentT {
                  global_env,
                  parent_env: package_env,
                  template_id,
                  id: template_id,
                  templatas: export_outer_templatas,
                });
                let region_placeholder = RegionT::Default;
                let placeholdered_export_name =
                  self.typing_interner.intern_export_name(ExportNameT {
                    template: template_name,
                  });
                let placeholdered_export_id_steps: Vec<INameT<'s, 't>> = vec![];
                let placeholdered_export_id = *self.typing_interner.intern_id(IdValT {
                  package_coord: package_id.package_coord,
                  init_steps: &placeholdered_export_id_steps,
                  local_name: INameT::Export(placeholdered_export_name),
                });
                let placeholdered_export_id_ref =
                  self.typing_interner.alloc(placeholdered_export_id);
                let export_templatas = TemplatasStoreBuilder::new(placeholdered_export_id_ref)
                  .build_in(self.typing_interner);
                let export_env = self.typing_interner.alloc(ExportEnvironmentT {
                  global_env,
                  parent_env: package_env,
                  template_id,
                  id: placeholdered_export_id,
                  templatas: export_templatas,
                });
                let export_env_as_iindenizen = IInDenizenEnvironmentT::Export(export_env);
                let call_ranges = self.typing_interner.alloc_slice_copy(&[function_a.range]);
                let export_placeholdered_prototype = match self
                  .evaluate_generic_light_function_from_call_for_prototype(
                    &mut coutputs,
                    call_ranges,
                    LocationInDenizen { path: &[] },
                    export_env_as_iindenizen,
                    templata,
                    &[],
                    region_placeholder,
                    &[],
                    &[],
                  )? {
                  IResolveFunctionResult::ResolveFunctionSuccess(success) => {
                    success.prototype.prototype
                  }
                  IResolveFunctionResult::ResolveFunctionFailure(failure) => {
                    return Err(ICompileErrorT::TypingPassResolvingError {
                      range: self.typing_interner.alloc_slice_copy(&[function_a.range]),
                      inner: failure.reason,
                    });
                  }
                };
                let export_name = match function_a.name {
                  IFunctionDeclarationNameS::FunctionName(fn_name_s) => fn_name_s.name,
                  other => panic!("vwat: {:?}", other),
                };
                coutputs.add_function_export(
                  function_a.range,
                  export_placeholdered_prototype,
                  placeholdered_export_id,
                  export_name,
                  self.typing_interner,
                );
              }
            }
          }
          _ => {}
        }
      }
    }

    // Export compile phase
    for (file_coord, program_a) in &file_to_program_s.file_coord_to_contents {
      let coord = file_coord.package_coord;
      for export in program_a.exports.iter() {
        let package_top_level_name =
          self.typing_interner.intern_package_top_level_name(PackageTopLevelNameT {});
        let package_id_steps: Vec<INameT<'s, 't>> = vec![];
        let package_id = *self.typing_interner.intern_id(IdValT {
          package_coord: coord,
          init_steps: &package_id_steps,
          local_name: INameT::PackageTopLevel(package_top_level_name),
        });
        let package_env = make_top_level_environment(global_env, package_id, self.typing_interner);

        let type_rune_t = export.rune.clone();

        let template_name = self
          .typing_interner
          .intern_export_template_name(ExportTemplateNameT { code_loc: export.range.begin });
        let template_id_steps: Vec<INameT<'s, 't>> = vec![];
        let template_id = *self.typing_interner.intern_id(IdValT {
          package_coord: coord,
          init_steps: &template_id_steps,
          local_name: INameT::ExportTemplate(template_name),
        });
        let template_id_ref = self.typing_interner.alloc(template_id);
        let export_outer_templatas =
          TemplatasStoreBuilder::new(template_id_ref).build_in(self.typing_interner);
        let _export_outer_env = self.typing_interner.alloc(ExportEnvironmentT {
          global_env,
          parent_env: package_env,
          template_id,
          id: template_id,
          templatas: export_outer_templatas,
        });

        let region_placeholder = RegionT::Default;

        let placeholdered_export_name = self
          .typing_interner
          .intern_export_name(ExportNameT { template: template_name });
        let placeholdered_export_id_steps: Vec<INameT<'s, 't>> = vec![];
        let placeholdered_export_id = *self.typing_interner.intern_id(IdValT {
          package_coord: coord,
          init_steps: &placeholdered_export_id_steps,
          local_name: INameT::Export(placeholdered_export_name),
        });
        let placeholdered_export_id_ref = self.typing_interner.alloc(placeholdered_export_id);
        let export_templatas =
          TemplatasStoreBuilder::new(placeholdered_export_id_ref).build_in(self.typing_interner);
        let export_env = self.typing_interner.alloc(ExportEnvironmentT {
          global_env,
          parent_env: package_env,
          template_id,
          id: placeholdered_export_id,
          templatas: export_templatas,
        });
        let export_env_as_iindenizen = IInDenizenEnvironmentT::Export(export_env);
        let export_env_as_ienv = IEnvironmentT::Export(export_env);

        let rune_to_type: IndexMap<IRuneS<'s>, ITemplataType<'s>> = self.derive_rune_to_type(
          &coutputs,
          export_env_as_iindenizen,
          vec![export.range],
          &[],
          export.rules,
          IndexMap::default(),
        );

        let parent_ranges_t: &'t [RangeS<'s>] =
          self.typing_interner.alloc_slice_copy(&[export.range]);

        let complete_define_solve = match self.solve_for_defining(
          InferEnv {
            original_calling_env: export_env_as_iindenizen,
            parent_ranges: parent_ranges_t,
            call_location: LocationInDenizen { path: &[] },
            self_env: export_env_as_ienv,
            context_region: region_placeholder,
          },
          &mut coutputs,
          export.rules,
          &[], // An export has no where-clause, so no bounds to conjure.
          &rune_to_type,
          parent_ranges_t,
          LocationInDenizen { path: &[] },
          &[],
          &[],
        ) {
          Err(_f) => {
            panic!("implement: TypingPassDefiningError from export solve_for_defining");
            // throw CompileErrorExceptionT(TypingPassDefiningError(ranges, f))
          }
          Ok(c) => c,
        };

        match complete_define_solve.conclusions.get(&type_rune_t.rune) {
          Some(ITemplataT::Kind(kt)) => {
            coutputs.add_kind_export(
              export.range,
              kt.kind,
              placeholdered_export_id,
              export.exported_name,
              self.typing_interner,
            );
          }
          Some(_) => panic!("vimpl"),
          None => panic!("vfail"),
        }
      }
    }

    // val (interfaceEdgeBlueprints, interfaceToSubCitizenToEdge) =
    //   Profiler.frame(() => { edgeCompiler.compileITables(coutputs) })
    let (interface_edge_blueprints, interface_to_sub_citizen_to_edge) =
      self.compile_i_tables(&mut coutputs)?;

    // Deferred function compilation loop
    // while (coutputs.peekNextDeferredFunctionBodyCompile().nonEmpty || coutputs.peekNextDeferredFunctionCompile().nonEmpty)
    while coutputs.peek_next_deferred_function_body_compile().is_some()
      || coutputs.peek_next_deferred_function_compile().is_some()
    {
      // while (coutputs.peekNextDeferredFunctionCompile().nonEmpty)
      while coutputs.peek_next_deferred_function_compile().is_some() {
        // val nextDeferredEvaluatingFunction = coutputs.peekNextDeferredFunctionCompile().get
        let next_deferred = coutputs.peek_next_deferred_function_compile().unwrap();
        match next_deferred {
          DeferredActionT::EvaluateFunction { name, calling_env, origin, template_args: _ } => {
            let name_val = *name;
            let calling_env = *calling_env;
            let _origin: &'s FunctionS<'s> = origin;

            let outer_env: IEnvironmentT<'s, 't> = IEnvironmentT::from(calling_env);
            let templata = FunctionTemplataT { outer_env, function_template_id: name };
            self.evaluate_generic_function_from_non_call_for_header(
              &mut coutputs,
              &[],
              LocationInDenizen { path: &[] },
              templata,
            )?;

            // coutputs.markDeferredFunctionCompiled(nextDeferredEvaluatingFunction.name)
            coutputs.mark_deferred_function_compiled(name_val);
          }
          _ => panic!("vcurious: unexpected deferred action variant in function-compile loop"),
        }
      }
      // if (coutputs.peekNextDeferredFunctionBodyCompile().nonEmpty)
      if coutputs.peek_next_deferred_function_body_compile().is_some() {
        let next_deferred = coutputs.peek_next_deferred_function_body_compile().unwrap();
        match next_deferred {
          DeferredActionT::EvaluateFunctionBody {
            prototype,
            full_env_snapshot,
            call_range,
            call_location,
            life,
            attributes_t,
            params_t,
            is_destructor,
            maybe_explicit_return_coord,
            instantiation_bound_params,
          } => {
            let prototype = *prototype;
            let full_env_snapshot = *full_env_snapshot;
            let call_range = *call_range;
            let call_location = *call_location;
            let life = *life;
            let attributes_t = *attributes_t;
            let params_t = *params_t;
            let is_destructor = *is_destructor;
            let maybe_explicit_return_coord = *maybe_explicit_return_coord;
            let instantiation_bound_params = *instantiation_bound_params;

            // (nextDeferredEvaluatingFunctionBody.call)(coutputs)
            self.finish_function_maybe_deferred(
              &mut coutputs,
              full_env_snapshot,
              call_range,
              call_location,
              life,
              attributes_t,
              params_t,
              is_destructor,
              maybe_explicit_return_coord,
              instantiation_bound_params,
            )?;

            // coutputs.markDeferredFunctionBodyCompiled(nextDeferredEvaluatingFunctionBody.prototypeT)
            coutputs.mark_deferred_function_body_compiled(prototype);
          }
          _ => panic!("implement: unexpected deferred action type"),
        }
      }
    }

    // ensureDeepExports(coutputs)
    self.ensure_deep_exports(&mut coutputs)?;

    // val (reachableInterfaces, reachableStructs, reachableFunctions) =
    //   (coutputs.getAllInterfaces(), coutputs.getAllStructs(), coutputs.getAllFunctions())
    let reachable_interfaces = coutputs.get_all_interfaces();
    let reachable_structs = coutputs.get_all_structs();
    let reachable_functions = coutputs.get_all_functions();

    // interfaceEdgeBlueprints.groupBy(_.interface).mapValues(vassertOne(_))
    let mut interface_to_edge_blueprints: HashMap<
      IdT<'s, 't>,
      &'t InterfaceEdgeBlueprintT<'s, 't>,
    > = HashMap::default();
    for blueprint in interface_edge_blueprints.iter() {
      let prev = interface_to_edge_blueprints.insert(blueprint.interface, blueprint);
      assert!(prev.is_none(), "vassertOne: multiple blueprints for same interface");
    }

    // coutputs.getInstantiationNameToFunctionBoundToRune()
    let raw_instantiation_bounds = coutputs.get_instantiation_name_to_function_bound_to_rune();
    let mut instantiation_name_to_instantiation_bounds: HashMap<
      IdT<'s, 't>,
      &'t InstantiationBoundArgumentsT<'s, 't>,
    > = HashMap::default();
    for (id, bounds) in raw_instantiation_bounds.iter() {
      instantiation_name_to_instantiation_bounds.insert(*id, *bounds);
    }

    let hinputs = HinputsT {
      interfaces: reachable_interfaces,
      structs: reachable_structs,
      functions: reachable_functions.clone(),
      interface_to_edge_blueprints,
      interface_to_sub_citizen_to_edge,
      instantiation_name_to_instantiation_bounds,
      kind_exports: coutputs.get_kind_exports(),
      function_exports: coutputs.get_function_exports(),
      kind_externs: coutputs.get_kind_externs(),
      function_externs: coutputs.get_function_externs(),
      sub_citizen_to_interface_to_edge: HashMap::default(),
    };

    // vassert(reachableFunctions.toVector.map(_.header.id).distinct.size == reachableFunctions.toVector.map(_.header.id).size)
    {
      let ids: Vec<_> = reachable_functions.iter().map(|f| f.header.id).collect();
      let distinct: HashSet<_> = ids.iter().collect();
      assert!(ids.len() == distinct.len());
    }

    Ok(hinputs)
  }

  pub fn preprocess_struct(
    &self,
    name_to_struct_defined_macro: &HashMap<StrI<'s>, OnStructDefinedMacro>,
    struct_name_t: IdT<'s, 't>,
    struct_a: &'s StructS<'s>,
  ) -> Vec<GeneratedAhtDenizen<'s, 't>> {
    let macro1 = self.scout_arena.alloc(MacroCallS {
      range: struct_a.range,
      include: IMacroInclusionP::CallMacro,
      macro_name: self.keywords.derive_struct_constructor,
    }) as &'s MacroCallS<'s>;
    let macro2 = self.scout_arena.alloc(MacroCallS {
      range: struct_a.range,
      include: IMacroInclusionP::CallMacro,
      macro_name: self.keywords.derive_struct_drop,
    }) as &'s MacroCallS<'s>;
    let default_called_macros = [macro1, macro2];
    let attr_refs: Vec<&'s ICitizenAttributeS<'s>> = struct_a.attributes.iter().collect();
    let macros_to_call = self.determine_macros_to_call(
      name_to_struct_defined_macro,
      &default_called_macros[..],
      &[struct_a.range],
      &attr_refs,
    );
    let mut generated_aht_denizens = Vec::new();
    for macro_ in macros_to_call {
      generated_aht_denizens.extend(macro_.get_struct_sibling_entries(
        self,
        struct_name_t,
        struct_a,
      ));
    }
    generated_aht_denizens
  }

  pub fn preprocess_interface(
    &self,
    name_to_interface_defined_macro: &HashMap<StrI<'s>, OnInterfaceDefinedMacro>,
    interface_name_t: IdT<'s, 't>,
    interface_a: &'s InterfaceS<'s>,
  ) -> Vec<GeneratedAhtDenizen<'s, 't>> {
    let macro1 = self.scout_arena.alloc(MacroCallS {
      range: interface_a.range,
      include: IMacroInclusionP::CallMacro,
      macro_name: self.keywords.derive_interface_drop,
    }) as &'s MacroCallS<'s>;
    let macro2 = self.scout_arena.alloc(MacroCallS {
      range: interface_a.range,
      include: IMacroInclusionP::CallMacro,
      macro_name: self.keywords.derive_anonymous_substruct,
    }) as &'s MacroCallS<'s>;
    let default_called_macros = [macro1, macro2];
    let attr_refs: Vec<&'s ICitizenAttributeS<'s>> = interface_a.attributes.iter().collect();
    let macros_to_call = self.determine_macros_to_call(
      name_to_interface_defined_macro,
      &default_called_macros[..],
      &[interface_a.range],
      &attr_refs,
    );
    let mut generated_aht_denizens = Vec::new();
    for macro_ in macros_to_call {
      generated_aht_denizens.extend(macro_.get_interface_sibling_entries(
        self,
        interface_name_t,
        interface_a,
      ));
    }
    generated_aht_denizens
  }

  pub fn determine_macros_to_call<T: Clone>(
    &self,
    name_to_macro: &HashMap<StrI<'s>, T>,
    default_called_macros: &[&'s MacroCallS<'s>],
    parent_ranges: &[RangeS<'s>],
    attributes: &[&'s ICitizenAttributeS<'s>],
  ) -> Vec<T> {
    let macros_to_call: Vec<&'s MacroCallS<'s>> =
      attributes.iter().fold(default_called_macros.to_vec(), |macros_to_call, attr| match attr {
        ICitizenAttributeS::MacroCall(mc) if mc.include == IMacroInclusionP::CallMacro => {
          if macros_to_call.iter().any(|m| m.macro_name == mc.macro_name) {
            panic!("Calling macro twice: {:?}", mc.macro_name);
          }
          let mut result = macros_to_call;
          result.push(mc);
          result
        }
        ICitizenAttributeS::MacroCall(mc) if mc.include == IMacroInclusionP::DontCallMacro => {
          macros_to_call.into_iter().filter(|m| m.macro_name != mc.macro_name).collect()
        }
        _ => macros_to_call,
      });
    macros_to_call
      .into_iter()
      .map(|macro_call| match name_to_macro.get(&macro_call.macro_name) {
        None => panic!("Macro not found: {:?}", macro_call.macro_name),
        Some(m) => m.clone(),
      })
      .collect()
  }

  pub fn ensure_deep_exports(
    &self,
    coutputs: &mut CompilerOutputs<'s, 't>,
  ) -> Result<(), ICompileErrorT<'s, 't>> {
    // val packageToKindToExport =
    //   coutputs.getKindExports
    //     .map(kindExport => (kindExport.id.packageCoord, kindExport.tyype, kindExport))
    //     .groupBy(_._1)
    //     .mapValues(
    //       _.map(x => (x._2, x._3))
    //         .groupBy(_._1)
    //         .mapValues({
    //           case Vector() => vwat()
    //           case Vector(only) => only
    //           case multiple => throw CompileErrorExceptionT(TypeExportedMultipleTimes(...))
    //         }))
    let kind_export_triples: Vec<(
      &'s PackageCoordinate<'s>,
      KindT<'s, 't>,
      &'t KindExportT<'s, 't>,
    )> =
      coutputs.get_kind_exports().iter().map(|ke| (ke.id.package_coord, ke.tyype, *ke)).collect();
    // Per @IIIOZ: IndexMap so iteration at the package/kind loops below is deterministic.
    // Upstream kind_export_triples is from coutputs.get_kind_exports() (Vec, deterministic).
    let mut grouped_by_package: IndexMap<
      &'s PackageCoordinate<'s>,
      Vec<(KindT<'s, 't>, &'t KindExportT<'s, 't>)>,
    > = IndexMap::default();
    for (pc, k, ke) in kind_export_triples.into_iter() {
      grouped_by_package.entry(pc).or_insert_with(Vec::new).push((k, ke));
    }
    let package_to_kind_to_export: IndexMap<
      &'s PackageCoordinate<'s>,
      IndexMap<KindT<'s, 't>, &'t KindExportT<'s, 't>>,
    > = {
      let mut result: IndexMap<
        &'s PackageCoordinate<'s>,
        IndexMap<KindT<'s, 't>, &'t KindExportT<'s, 't>>,
      > = IndexMap::default();
      for (pc, kind_pairs) in grouped_by_package.into_iter() {
        let mut grouped_by_kind: IndexMap<KindT<'s, 't>, Vec<&'t KindExportT<'s, 't>>> =
          IndexMap::default();
        for (k, ke) in kind_pairs.into_iter() {
          grouped_by_kind.entry(k).or_insert_with(Vec::new).push(ke);
        }
        let mut inner: IndexMap<KindT<'s, 't>, &'t KindExportT<'s, 't>> = IndexMap::default();
        for (k, exports) in grouped_by_kind.into_iter() {
          let only = match exports.as_slice() {
            [] => panic!("vwat"),
            [only] => *only,
            _ => {
              let exports_copies: Vec<KindExportT<'s, 't>> = exports
                .iter()
                .map(|ke| KindExportT {
                  range: ke.range,
                  tyype: ke.tyype,
                  id: ke.id,
                  exported_name: ke.exported_name,
                })
                .collect();
              let exports_slice = self.typing_interner.alloc_slice_from_vec(exports_copies);
              let range_slice = self.typing_interner.alloc_slice_copy(&[exports[0].range]);
              return Err(ICompileErrorT::TypeExportedMultipleTimes {
                range: range_slice,
                paackage: *exports[0].id.package_coord,
                exports: exports_slice,
              });
            }
          };
          inner.insert(k, only);
        }
        result.insert(pc, inner);
      }
      result
    };

    // coutputs.getFunctionExports.foreach(funcExport => {
    //   val exportedKindToExport = packageToKindToExport.getOrElse(funcExport.exportId.packageCoord, Map())
    //   (Vector(funcExport.prototype.returnType) ++ funcExport.prototype.paramTypes)
    //     .foreach(paramType => {
    //       if (!Compiler.isPrimitive(paramType.kind) && !exportedKindToExport.contains(paramType.kind)) {
    //         throw CompileErrorExceptionT(ExportedFunctionDependedOnNonExportedKind(...))
    //       }
    //     })
    // })
    let empty_kind_map: IndexMap<KindT<'s, 't>, &'t KindExportT<'s, 't>> = IndexMap::default();
    for func_export in coutputs.get_function_exports().iter() {
      let exported_kind_to_export = package_to_kind_to_export
        .get(func_export.export_id.package_coord)
        .unwrap_or(&empty_kind_map);
      let all_types: Vec<KindT<'s, 't>> = once(func_export.prototype.return_type)
        .chain(func_export.prototype.param_types().iter().copied())
        .collect();
      for param_type in all_types {
        if !self.is_primitive(param_type) && !exported_kind_to_export.contains_key(&param_type) {
          let range_t = self.typing_interner.alloc_slice_copy(&[func_export.range]);
          let signature_t = self.typing_interner.alloc(func_export.prototype.to_signature());
          return Err(ICompileErrorT::ExportedFunctionDependedOnNonExportedKind {
            range: range_t,
            paackage: *func_export.export_id.package_coord,
            signature: signature_t,
            non_exported_kind: param_type,
          });
        }
      }
    }

    for function_extern in coutputs.get_function_externs().iter() {
      let exported_kind_to_export = package_to_kind_to_export
        .get(function_extern.extern_placeholdered_id.package_coord)
        .unwrap_or(&empty_kind_map);
      let all_types: Vec<KindT<'s, 't>> = once(function_extern.prototype.return_type)
        .chain(function_extern.prototype.param_types().iter().copied())
        .collect();
      for param_type in all_types {
        if !self.is_primitive(param_type) && !exported_kind_to_export.contains_key(&param_type) {
          // Method-own and container-inherited template params surface here as
          // placeholders at definition time (e.g. `extern func bar<C>(c C)` inside
          // `extern struct Foo<A>` has C and A as KindPlaceholderTs in the wrapper
          // prototype). Placeholders are substitution slots, not concrete types; the
          // actual concrete kind for each monomorphization is what matters for ABI,
          // and gets checked at instantiation.
          let kind_is_fine_in_extern_func = match param_type {
            KindT::Struct(s) => coutputs
              .lookup_struct(s.id, self)
              .attributes
              .iter()
              .any(|a| matches!(a, ICitizenAttributeT::Extern(_))),
            KindT::KindPlaceholder(_) => true,
            _ => false,
          };
          if !kind_is_fine_in_extern_func {
            let range_t = self.typing_interner.alloc_slice_copy(&[function_extern.range]);
            let signature_t = self.typing_interner.alloc(function_extern.prototype.to_signature());
            return Err(ICompileErrorT::ExternFunctionDependedOnNonExportedKind {
              range: range_t,
              paackage: *function_extern.extern_placeholdered_id.package_coord,
              signature: signature_t,
              non_exported_kind: param_type,
            });
          }
        }
      }
    }

    // packageToKindToExport.foreach((packageCoord, exportedKindToExport) =>
    //   exportedKindToExport.foreach((exportedKind, (kind, export)) =>
    //     exportedKind match { case StructTT(_) => ...; case contentsStaticSizedArrayTT(...) => ...; ... }))
    for (package_coord, exported_kind_to_export) in package_to_kind_to_export.iter() {
      for (exported_kind, export) in exported_kind_to_export.iter() {
        match exported_kind {
          KindT::Struct(sr) => {
            let struct_def = coutputs.lookup_struct(sr.id, self);
            let substituter = self.get_placeholder_substituter(
              self.opts.global_options.sanity_check,
              struct_def.template_name,
              sr.id,
              IBoundArgumentsSource::InheritBoundsFromTypeItself,
            );
            for member in struct_def.members.iter() {
              let member_coord = substituter.substitute_for_kind(coutputs, member.tyype);
              let member_kind = member_coord;
              if !self.is_primitive(member_kind)
                && !exported_kind_to_export.contains_key(&member_kind)
              {
                let range_t = self.typing_interner.alloc_slice_copy(&[export.range]);
                return Err(ICompileErrorT::ExportedKindDependedOnNonExportedKind {
                  range: range_t,
                  paackage: **package_coord,
                  exported_kind: *exported_kind,
                  non_exported_kind: member_kind,
                });
              }
            }
          }
          KindT::StaticSizedArray(as_tt) => {
            let element_kind = as_tt.element_type();
            if !self.is_primitive(element_kind)
              && !exported_kind_to_export.contains_key(&element_kind)
            {
              let range_t = self.typing_interner.alloc_slice_copy(&[export.range]);
              return Err(ICompileErrorT::ExportedKindDependedOnNonExportedKind {
                range: range_t,
                paackage: **package_coord,
                exported_kind: *exported_kind,
                non_exported_kind: element_kind,
              });
            }
          }
          KindT::RuntimeSizedArray(rsa) => {
            let element_kind = match rsa.name.local_name {
              INameT::RuntimeSizedArray(rsan) => rsan.arr.element_type,
              _ => panic!("vwat"),
            };
            if !self.is_primitive(element_kind)
              && !exported_kind_to_export.contains_key(&element_kind)
            {
              let range_t = self.typing_interner.alloc_slice_copy(&[export.range]);
              return Err(ICompileErrorT::ExportedKindDependedOnNonExportedKind {
                range: range_t,
                paackage: **package_coord,
                exported_kind: *exported_kind,
                non_exported_kind: element_kind,
              });
            }
          }
          // VCOORD: an exported interface's dependencies are checked nowhere. Structs and
          // both array kinds walk their contents above; this arm walks nothing, so an
          // interface whose method signatures mention a non-exported kind exports clean.
          // Whether that is right turns on what an exported interface *is*: upstream ruled
          // that design-2's class-tier `interface` gets no Rust projection at all and
          // crosses as an opaque handle, which would need no check — but our interfaces are
          // ~86.5% struct-tier, and the struct tier does project. Decide the tier question
          // before filling this in.
          KindT::Interface(_) => {}
          KindT::KindPlaceholder(_)
          | KindT::OverloadSet(_)
          | KindT::Void(_)
          | KindT::Int(_)
          | KindT::Bool(_)
          | KindT::Str(_)
          | KindT::Float(_)
          | KindT::USize(_)
          | KindT::Never(_) => {
            panic!("vwat: unexpected kind in exportedKindToExport");
          }
          KindT::BorrowRef(_) => unimplemented!(),
          KindT::OwnRef(_) => unimplemented!(),
          KindT::ShareRef(_) => unimplemented!(),
          KindT::WeakRef(_) => unimplemented!(),
        }
      }
    }
    Ok(())
  }

  pub fn is_root_function(&self, function_a: &'s FunctionS<'s>) -> bool {
    panic!("Unimplemented: Slab 15");
    // functionA.name match {
    //   case FunctionNameS(StrI("main"), _) => return true
    //   case _ =>
    // }
    // functionA.attributes.exists({
    //   case ExportS(_) => true
    //   case ExternS(_) => true
    //   case _ => false
    // })
  }

  pub fn is_root_struct(&self, struct_a: &'s StructS<'s>) -> bool {
    panic!("Unimplemented: Slab 15");
    // structA.attributes.exists({ case ExportS(_) => true case _ => false })
  }

  pub fn is_root_interface(&self, interface_a: &'s InterfaceS<'s>) -> bool {
    panic!("Unimplemented: Slab 15");
    // interfaceA.attributes.exists({ case ExportS(_) => true case _ => false })
  }

  pub fn consecutive(&self, exprs: &[ExpressionTE<'s, 't>]) -> ExpressionTE<'s, 't> {
    match exprs {
      [] => panic!("Shouldn't have zero-element consecutors!"),
      [only] => *only,
      _ => {
        let flattened: Vec<ExpressionTE<'s, 't>> = exprs
          .iter()
          .flat_map(|e| match e {
            ExpressionTE::Consecutor(c) => c.exprs.to_vec(),
            other => vec![*other],
          })
          .collect();

        let without_init_voids: Vec<ExpressionTE<'s, 't>> = {
          let (init, last) = flattened.split_at(flattened.len() - 1);
          let mut filtered: Vec<ExpressionTE<'s, 't>> =
            init.iter().filter(|e| !matches!(e, ExpressionTE::VoidLiteral(_))).copied().collect();
          filtered.push(last[0]);
          filtered
        };

        match without_init_voids.as_slice() {
          [] => panic!("Shouldn't have zero-element consecutors!"),
          [only] => *only,
          _ => {
            let exprs_slice = self.typing_interner.alloc_slice_copy(&without_init_voids);
            ExpressionTE::Consecutor(self.typing_interner.alloc(ConsecutorTE::new(exprs_slice)))
          }
        }
      }
    }
  }

  pub fn is_primitive(&self, kind: KindT<'s, 't>) -> bool {
    match kind {
      KindT::Void(_)
      | KindT::Int(_)
      | KindT::Bool(_)
      | KindT::Str(_)
      | KindT::Never(_)
      | KindT::Float(_)
      | KindT::USize(_) => true,
      KindT::KindPlaceholder(_) => false,
      KindT::Struct(_) => false,
      KindT::Interface(_) => false,
      KindT::StaticSizedArray(_) => false,
      KindT::RuntimeSizedArray(_) => false,
      KindT::OverloadSet(_) => false,
      // VCOORD: settle on a definition for primitive. err on the side of *not* this, probably...
      KindT::BorrowRef(_) | KindT::OwnRef(_) | KindT::ShareRef(_) | KindT::WeakRef(_) => true,
    }
  }

  pub fn get_mutabilities(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    concrete_values2: &[KindT<'s, 't>],
  ) -> Vec<ITemplataT<'s, 't>> {
    panic!("Unimplemented: Slab 15");
    // concreteValues2.map(concreteValue2 => getMutability(coutputs, concreteValue2))
  }
}
