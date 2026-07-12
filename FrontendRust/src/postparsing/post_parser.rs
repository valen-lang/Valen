// Coordinates the Scout (post-parsing) pass

// AFTERM: rename Denizen to Definition, and maybe Citizen to TypeDefinition
// AFTERM: rename ScoutCompilation to PostParserCompilation

use crate::compile_options::GlobalOptions;
use crate::scout_arena::ScoutArena;
use crate::keywords::Keywords;
use crate::lexing::ast::RangeL;
use crate::lexing::errors::FailedParse;
use crate::parsing::ast::{
  FileP, GenericParameterP, IAttributeP, IDenizenP, IMacroInclusionP, IStructContent, ITemplexPT,
  SharednessP, StructP,
};
use crate::parsing::ast::rules::get_ordered_rune_declarations_from_rulexes_with_duplicates;
use crate::parsing::parser::ParserCompilation;
use crate::postparsing::ast::{
  ExportAsS, GenericParameterDefaultS, GenericParameterS,
  IBodyS, ICitizenAttributeS,
  IGenericParameterTypeS, KindGenericParameterTypeS, ImportS, ImplS, InterfaceS, IStructMemberS,
  LocationInDenizenBuilder, MacroCallS, NormalStructMemberS, OtherGenericParameterTypeS,
  ProgramS, RegionGenericParameterTypeS, StructS, VariadicStructMemberS,
};
use crate::postparsing::expressions::{ConsecutorSE, IExpressionSE};
use crate::postparsing::function_scout::IFunctionParent;
use crate::postparsing::itemplatatype::{
  ITemplataType, KindTemplataType, PackTemplataType,
  RegionTemplataType, TemplateTemplataType,
};
use crate::postparsing::names::{
  CodeNameS, CodeRuneS, ExportAsNameS, IFunctionDeclarationNameS, IImplDeclarationNameS, IImpreciseNameS,
  IImpreciseNameValS, INameS, INameValS, DenizenDefaultRegionRuneS, IRuneS, IRuneValS,
  IStructDeclarationNameS, IVarNameS, ImplDeclarationNameS, TopLevelInterfaceDeclarationNameS,
  TopLevelStructDeclarationNameS,
};
use crate::postparsing::rules::rule_scout::{translate_rulexes, translate_type};
use crate::postparsing::rules::templex_scout::{translate_templex, add_literal_rule};
use crate::postparsing::rules::rules::{
  EqualsSR, IRulexSR, RuneUsage,
};
use crate::postparsing::variable_uses::{VariableDeclarations, VariableUses};
use crate::utils::code_hierarchy::FileCoordinateMap;
use crate::code_source::CodeSource;
use crate::utils::code_hierarchy::{FileCoordinate, PackageCoordinate};
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::utils::range::{CodeLocationS, RangeS};
use crate::utils::fx::HashMap;
use crate::utils::fx::IndexSet;
use crate::parsing::ast::IImpreciseNameP;
use crate::postparsing::names::{IterableNameS, IteratorNameS, IterationOptionNameS};
use crate::parse_arena::ParseArena;
use crate::postparsing::ast::FunctionS;
use crate::parsing::ast::ImplP;
use crate::parsing::ast::IRulexPR;
use crate::parsing::ast::ExportAsP;
use crate::parsing::ast::ImportP;
use crate::postparsing::rules::rules::ILiteralSL;
use crate::postparsing::ast::ExportS;
use crate::postparsing::ast::SealedS;
use crate::parsing::ast::InterfaceP;
use crate::parsing::ast::FunctionP;
use crate::postparsing::ast::ExternS;
use crate::postparsing::function_scout::ParentCitizen;
use crate::postparsing::rules::templex_scout::translate_maybe_type_into_rune;
use crate::utils::fx::HashSet;
#[derive(Debug, PartialEq)]
pub struct CompileErrorExceptionS<'s> {
  pub err: ICompileErrorS<'s>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ICompileErrorS<'s> {
  CouldntFindVarToMutateS(CouldntFindVarToMutateS<'s>),
  CouldntFindRuneS(CouldntFindRuneS<'s>),
  StatementAfterReturnS(StatementAfterReturnS<'s>),
  VariableNameAlreadyExists(VariableNameAlreadyExists<'s>),
  InterfaceMethodNeedsSelf(InterfaceMethodNeedsSelf<'s>),
  VirtualAndAbstractGoTogether(VirtualAndAbstractGoTogether<'s>),
  InitializingRuntimeSizedArrayRequiresSizeAndCallable(
    InitializingRuntimeSizedArrayRequiresSizeAndCallable<'s>,
  ),
  InitializingStaticSizedArrayRequiresSizeAndCallable(
    InitializingStaticSizedArrayRequiresSizeAndCallable<'s>,
  ),
  ExternHasBodyS(ExternHasBodyS<'s>),
  RangedInternalErrorS(RangedInternalErrorS<'s>),
  CantOwnershipInterfaceInImpl(CantOwnershipInterfaceInImpl<'s>),
  CantOwnershipStructInImpl(CantOwnershipStructInImpl<'s>),
  /// A parameter uses destructure syntax but the function has no body block to
  /// prepend the desugared LetSE into (extern / abstract / generated bodies).
  ParamDestructureRequiresBody { range: RangeS<'s> },
}


impl ICompileErrorS<'_> {
  pub fn range(&self) -> &RangeS<'_> {
    match self {
      ICompileErrorS::CouldntFindVarToMutateS(x) => &x.range,
      ICompileErrorS::CouldntFindRuneS(x) => &x.range,
      ICompileErrorS::StatementAfterReturnS(x) => &x.range,
      ICompileErrorS::VariableNameAlreadyExists(x) => &x.range,
      ICompileErrorS::InterfaceMethodNeedsSelf(x) => &x.range,
      ICompileErrorS::VirtualAndAbstractGoTogether(x) => &x.range,
      ICompileErrorS::InitializingRuntimeSizedArrayRequiresSizeAndCallable(x) => &x.range,
      ICompileErrorS::InitializingStaticSizedArrayRequiresSizeAndCallable(x) => &x.range,
      ICompileErrorS::ExternHasBodyS(x) => &x.range,
      ICompileErrorS::RangedInternalErrorS(x) => &x.range,
      ICompileErrorS::CantOwnershipInterfaceInImpl(x) => &x.range,
      ICompileErrorS::CantOwnershipStructInImpl(x) => &x.range,
      ICompileErrorS::ParamDestructureRequiresBody { range } => range,
    }
  }

}


#[derive(Clone, Debug, PartialEq)]
pub struct CouldntFindVarToMutateS<'s> {
  pub range: RangeS<'s>,
  pub name: String,
}


#[derive(Clone, Debug, PartialEq)]
pub struct CouldntFindRuneS<'s> {
  pub range: RangeS<'s>,
  pub name: String,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StatementAfterReturnS<'s> {
  pub range: RangeS<'s>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExternHasBodyS<'s> {
  pub range: RangeS<'s>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializingRuntimeSizedArrayRequiresSizeAndCallable<'s> {
  pub range: RangeS<'s>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializingStaticSizedArrayRequiresSizeAndCallable<'s> {
  pub range: RangeS<'s>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CantOwnershipInterfaceInImpl<'s> {
  pub range: RangeS<'s>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CantOwnershipStructInImpl<'s> {
  pub range: RangeS<'s>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VariableNameAlreadyExists<'s> {
  pub range: RangeS<'s>,
  pub name: IVarNameS<'s>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InterfaceMethodNeedsSelf<'s> {
  pub range: RangeS<'s>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VirtualAndAbstractGoTogether<'s> {
  pub range: RangeS<'s>,
}


#[derive(Clone, Debug, PartialEq)]
pub struct RangedInternalErrorS<'s> {
  pub range: RangeS<'s>,
  pub message: String,
}

#[derive(Clone, Debug, PartialEq)]
// SPORK
pub enum IEnvironmentS<'s> {
  Environment(EnvironmentS<'s>),
  FunctionEnvironment(FunctionEnvironmentS<'s>),
}

impl<'s> IEnvironmentS<'s> {
  pub fn file(&self) -> &'s FileCoordinate<'s> {
    match self {
      IEnvironmentS::Environment(environment) => environment.file,
      IEnvironmentS::FunctionEnvironment(function_environment) => function_environment.file,
    }
  }
  

  pub fn all_declared_runes(&self) -> IndexSet<IRuneS<'s>> {
    match self {
      IEnvironmentS::Environment(environment) => environment.all_declared_runes(),
      IEnvironmentS::FunctionEnvironment(function_environment) => function_environment.all_declared_runes(),
    }
  }
  
  pub fn local_declared_runes(&self) -> IndexSet<IRuneS<'s>> {
    match self {
      IEnvironmentS::Environment(environment) => environment.local_declared_runes(),
      IEnvironmentS::FunctionEnvironment(function_environment) => {
        function_environment.local_declared_runes()
      }
    }
  }
  

}


#[derive(Clone, Debug, PartialEq)]
pub struct EnvironmentS<'s> {
  pub file: &'s FileCoordinate<'s>,
  pub parent_env: Option<Box<EnvironmentS<'s>>>,
  pub name: INameS<'s>,
  pub user_declared_runes: IndexSet<IRuneS<'s>>,
}

impl<'s> EnvironmentS<'s> {


  pub fn local_declared_runes(&self) -> IndexSet<IRuneS<'s>> {
    self.user_declared_runes.clone()
  }
  

  pub fn all_declared_runes(&self) -> IndexSet<IRuneS<'s>> {
    let mut runes = self.user_declared_runes.clone();
    if let Some(parent_env) = &self.parent_env {
      runes.extend(parent_env.all_declared_runes());
    }
    runes
  }
  
}


#[derive(Clone, Debug, PartialEq)]
pub struct FunctionEnvironmentS<'s> {
  pub file: &'s FileCoordinate<'s>,
  pub name: IFunctionDeclarationNameS<'s>,
  pub parent_env: Option<Box<IEnvironmentS<'s>>>,
  pub declared_runes: IndexSet<IRuneS<'s>>,
  pub num_explicit_params: i32,
  pub is_interface_internal_method: bool,
}


impl<'s> FunctionEnvironmentS<'s> {
  pub fn local_declared_runes(&self) -> IndexSet<IRuneS<'s>> {
    self.declared_runes.clone()
  }

  pub fn all_declared_runes(&self) -> IndexSet<IRuneS<'s>> {
    let mut runes = self.declared_runes.clone();
    if let Some(parent_env) = &self.parent_env {
      runes.extend(parent_env.all_declared_runes());
    }
    runes
  }

  pub fn child(&self) -> FunctionEnvironmentS<'s> {
    FunctionEnvironmentS::<'s> {
      file: self.file,
      name: self.name.clone(),
      parent_env: Some(Box::new(IEnvironmentS::FunctionEnvironment(self.clone()))),
      declared_runes: IndexSet::default(),
      num_explicit_params: self.num_explicit_params,
      is_interface_internal_method: false,
    }
  }


}


#[derive(Clone, Debug, PartialEq)]
pub struct StackFrame<'s> {
  pub file: &'s FileCoordinate<'s>,
  pub name: IFunctionDeclarationNameS<'s>,
  pub parent_env: FunctionEnvironmentS<'s>,
  pub maybe_parent: Option<Box<StackFrame<'s>>>,
  pub context_region: IRuneS<'s>,
  pub pure_height: i32,
  pub locals: VariableDeclarations<'s>,
}

impl<'s> StackFrame<'s> {

// MIGALLOW: ++ -> plus
pub fn plus(&self, new_vars: &VariableDeclarations<'s>) -> StackFrame<'s> {
  StackFrame::<'s> {
    file: self.file,
    name: self.name.clone(),
    parent_env: self.parent_env.clone(),
    maybe_parent: self.maybe_parent.clone(),
    context_region: self.context_region.clone(),
    pure_height: self.pure_height,
    locals: self.locals.plus_plus(new_vars),
  }
}


pub fn all_declarations(&self) -> VariableDeclarations<'_> {
  match &self.maybe_parent {
    Some(parent) => self.locals.plus_plus(&parent.all_declarations()),
    None => self.locals.plus_plus(&PostParser::<'s, '_, '_>::no_declarations()),
  }
}

pub fn find_variable(&self, name: &IImpreciseNameS<'s>) -> Option<IVarNameS<'s>> {
  match self.locals.find(name) {
    Some(full_name_s) => Some(full_name_s),
    None => match &self.maybe_parent {
      None => None,
      Some(parent) => parent.find_variable(name),
    },
  }
}

}


// MIGALLOW: noVariableUses -> no_variable_uses
// MIGALLOW: noDeclarations -> no_declarations
impl<'s, 'p, 'ctx> PostParser<'s, 'p, 'ctx>
{
  pub fn no_variable_uses() -> VariableUses<'s> {
    VariableUses::<'s>::empty()
  }
  
  // AFTERM: consider moving no_declarations out of PostParser
  pub fn no_declarations() -> VariableDeclarations<'s> {
    VariableDeclarations { vars: Vec::new() }
  }
  

  pub fn eval_range(file: &'s FileCoordinate<'s>, range: RangeL) -> RangeS<'s> {
    RangeS::new(
      Self::eval_pos(file, range.begin()),
      Self::eval_pos(file, range.end()),
    )
  }
  
  pub fn eval_pos(file: &'s FileCoordinate<'s>, pos: i32) -> CodeLocationS<'s> {
    CodeLocationS {
      file,
      offset: pos,
    }
  }
  
}

pub(crate) fn translate_imprecise_name<'s, 'p>(
  scout_arena: &ScoutArena<'s>,
  file: &'s FileCoordinate<'s>,
  name: &IImpreciseNameP<'p>,
) -> IImpreciseNameS<'s> {
  match name {
    // Re-intern string from 'p into 's
    IImpreciseNameP::LookupName(n) => scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS { name: scout_arena.intern_str(n.str().as_str()) })),
    IImpreciseNameP::IterableName(range) => scout_arena.intern_imprecise_name(IImpreciseNameValS::IterableName(IterableNameS {
      range: PostParser::eval_range(file, *range),
    })),
    IImpreciseNameP::IteratorName(range) => scout_arena.intern_imprecise_name(IImpreciseNameValS::IteratorName(IteratorNameS {
      range: PostParser::eval_range(file, *range),
    })),
    IImpreciseNameP::IterationOptionName(range) => scout_arena.intern_imprecise_name(IImpreciseNameValS::IterationOptionName(IterationOptionNameS {
      range: PostParser::eval_range(file, *range),
    })),
  }
}

fn determine_denizen_type<'s>(
  _template_result_type: ITemplataType<'s>,
  _identifying_runes_s: &[IRuneS<'s>],
  _rune_a_to_type: &HashMap<IRuneS<'s>, ITemplataType<'s>>,
) -> Result<ITemplataType<'s>, IRuneS<'s>> {
  panic!("Unimplemented determine_denizen_type");
}

fn get_human_name<'s, 'p>(
  _scout_arena: &ScoutArena<'s>,
  _templex: &ITemplexPT<'p>,
) -> IImpreciseNameS<'s> {
  panic!("Unimplemented get_human_name");
}

impl<'s, 'p, 'ctx> PostParser<'s, 'p, 'ctx>
{
  pub fn consecutive(&self, exprs: Vec<&'s IExpressionSE<'s>>) -> &'s IExpressionSE<'s> {
    assert!(!exprs.is_empty(), "POSTPARSER_CONSECUTIVE_EMPTY");
    if exprs.len() == 1 {
      return exprs.into_iter().next().unwrap();
    }
    let mut flattened = Vec::new();
    for expr in exprs {
      match expr {
        IExpressionSE::Consecutor(consecutor) => flattened.extend(consecutor.exprs.iter().copied()),
        other => flattened.push(other),
      }
    }
    let slice = self.scout_arena.alloc_slice_from_vec(flattened);
    &*self.scout_arena.alloc(IExpressionSE::Consecutor(ConsecutorSE { exprs: slice }))
  }

pub(crate) fn scout_generic_parameter(
  &self,
  env: IEnvironmentS<'s>,
  lidb: &mut LocationInDenizenBuilder,
  rune_to_explicit_type: &mut Vec<(IRuneS<'s>, ITemplataType<'s>)>,
  rule_builder: &mut Vec<IRulexSR<'s>>,
  // This might seem a bit weird, because the region rune usually comes last and is usually
  // mentioned at the end of the header too. But indeed we need it for knowing the region to use
  // for generic params' default values.
  context_region: IRuneS<'s>,
  generic_param_p: &GenericParameterP<'p>,
  param_rune_s: RuneUsage<'s>,
  // Returns a possible implicit region generic param (see MNRFGC), and the translated original
  // generic param.
) -> GenericParameterS<'s> {
  let file = env.file();
  let generic_param_range_s = PostParser::eval_range(file, generic_param_p.range);
  let rune_s = param_rune_s;

  let type_s = match &generic_param_p.maybe_type {
    None => ITemplataType::KindTemplataType(KindTemplataType {}),
    Some(type_p) => translate_type(self.scout_arena, type_p.tyype),
  };
  rune_to_explicit_type.push((rune_s.rune.clone(), type_s.clone()));

  assert!(
    generic_param_p.coord_region.is_none(),
    "POSTPARSER_SCOUT_GENERIC_PARAMETER_COORD_REGION_NOT_YET_IMPLEMENTED"
  );

  let generic_param_type_s = match type_s {
    ITemplataType::KindTemplataType(_) => {
      if !generic_param_p.attributes.is_empty() {
        panic!("POSTPARSER_SCOUT_GENERIC_PARAMETER_BAD_KIND_RUNE_ATTRIBUTE");
      }
      IGenericParameterTypeS::KindGenericParameterType(KindGenericParameterTypeS {})
    }
    ITemplataType::RegionTemplataType(_) => {
      if !generic_param_p.attributes.is_empty() {
        panic!("POSTPARSER_SCOUT_GENERIC_PARAMETER_BAD_REGION_RUNE_ATTRIBUTE");
      }
      IGenericParameterTypeS::RegionGenericParameterType(
        RegionGenericParameterTypeS {},
      )
    }
    _ => {
      if !generic_param_p.attributes.is_empty() {
        panic!("POSTPARSER_SCOUT_GENERIC_PARAMETER_BAD_OTHER_RUNE_ATTRIBUTE");
      }
      IGenericParameterTypeS::OtherGenericParameterType(
        OtherGenericParameterTypeS::new(type_s),
      )
    }
  };
  let default_s = generic_param_p.maybe_default.map(|default_pt| {
    let mut uncategorized_rules = Vec::new();
    let result_rune = translate_templex(
      self.scout_arena, self.keywords, env, lidb, &mut uncategorized_rules, context_region, &default_pt,
    );
    uncategorized_rules.push(IRulexSR::Equals(EqualsSR {
      range: generic_param_range_s,
      left: rune_s.clone(),
      right: result_rune.clone(),
    }));

    let mut rules_to_leave_in_default_argument = Vec::new();
    for r in uncategorized_rules {
      match r {
        IRulexSR::KindList(_) => rule_builder.push(r), // Hoist it up into regular rules
        IRulexSR::Literal(_) => rules_to_leave_in_default_argument.push(&*self.scout_arena.alloc(r)),
        IRulexSR::Lookup(_) => rules_to_leave_in_default_argument.push(&*self.scout_arena.alloc(r)),
        IRulexSR::Resolve(_) => rules_to_leave_in_default_argument.push(&*self.scout_arena.alloc(r)),
        // Per @DRSINI, this EqualsSR aliases the param rune to the default's resultRune.
        // We KEEP it in the default's rules (rather than hoisting) so the default is fully
        // self-contained — it travels intact when GenericParameterS is inherited (e.g. by
        // struct internal methods). At default-fire time, the typing pass registers the
        // default-only runes via solverState.registerRunes(default.runeToType.keys).
        IRulexSR::Equals(_) => rules_to_leave_in_default_argument.push(&*self.scout_arena.alloc(r)),
        IRulexSR::CallSiteFunc(_) => rule_builder.push(r), // Hoist it up into regular rules
        IRulexSR::DefinitionFunc(_) => rule_builder.push(r), // Hoist it up into regular rules
        other => panic!("vwat: {:?}", other),
      }
    }

    let default_rune_to_type = self.scout_arena.alloc_slice_from_vec(
      vec![(result_rune.rune.clone(), generic_param_type_s.tyype())]);
    GenericParameterDefaultS {
      result_rune: result_rune.rune,
      rules: self.scout_arena.alloc_slice_from_vec(rules_to_leave_in_default_argument),
      rune_to_type: default_rune_to_type,
    }
  });

  return GenericParameterS {
      range: generic_param_range_s,
      rune: rune_s,
      tyype: generic_param_type_s,
      default: default_s,
    };
  }

}

pub struct PostParser<'s, 'p, 'ctx> {
  pub global_options: GlobalOptions,
  pub scout_arena: &'ctx ScoutArena<'s>,
  pub keywords: &'ctx Keywords<'s>,
  pub keywords_p: &'ctx Keywords<'p>, // Per @PPSPASTNZ, synthetic parser nodes need 'p-interned keyword strings
  pub parse_arena: &'ctx ParseArena<'p>, // Per @PPSPASTNZ, for allocating synthetic parser AST nodes
}


impl<'s, 'p, 'ctx> PostParser<'s, 'p, 'ctx>
{
  // MIGALLOW: new -> new
  pub fn new(
    global_options: GlobalOptions,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    keywords_p: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
  ) -> Self {
    Self {
      global_options,
      scout_arena,
      keywords,
      keywords_p,
      parse_arena,
    }
  }

  pub fn scout_program(
    &self,
    file_coordinate: &'s FileCoordinate<'s>,
    parsed: &FileP<'p>,
  ) -> Result<ProgramS<'s>, ICompileErrorS<'s>>
  {
    let mut structs: Vec<&'s StructS<'s>> = Vec::new();
    for denizen in parsed.denizens {
      if let IDenizenP::TopLevelStruct(struct_p) = denizen {
        structs.push(&*self.scout_arena.alloc(self.scout_struct(file_coordinate, struct_p)?));
      }
    }

    let mut interfaces: Vec<&'s InterfaceS<'s>> = Vec::new();
    for denizen in parsed.denizens {
      if let IDenizenP::TopLevelInterface(interface_p) = denizen {
        interfaces.push(&*self.scout_arena.alloc(self.scout_interface(file_coordinate, interface_p)?));
      }
    }

    let mut impls: Vec<&'s ImplS<'s>> = Vec::new();
    for denizen in parsed.denizens {
      if let IDenizenP::TopLevelImpl(impl_p) = denizen {
        impls.push(&*self.scout_arena.alloc(self.scout_impl(file_coordinate, impl_p)?));
      }
    }

    let mut implemented_functions: Vec<&'s FunctionS<'s>> = Vec::new();
    for denizen in parsed.denizens {
      if let IDenizenP::TopLevelFunction(function_p) = denizen {
        let (function_s, function_uses) =
          self.scout_function(
            file_coordinate,
            function_p,
            IFunctionParent::FunctionNoParent,
          )?;
        assert!(function_uses.uses.is_empty());
        if let IBodyS::CodeBody(code_body_s) = &function_s.body {
          assert!(
            code_body_s.body.closured_names.is_empty(),
            "POSTPARSER_SCOUT_PROGRAM_TOP_LEVEL_FUNCTION_USES_PARENT_VARS"
          );
        }
        implemented_functions.push(function_s);
      }
    }

    let mut exports: Vec<&'s ExportAsS<'s>> = Vec::new();
    for denizen in parsed.denizens {
      if let IDenizenP::TopLevelExportAs(export_as_p) = denizen {
        exports.push(&*self.scout_arena.alloc(self.scout_export_as(file_coordinate, export_as_p)));
      }
    }

    let mut imports: Vec<&'s ImportS<'s>> = Vec::new();
    for denizen in parsed.denizens {
      if let IDenizenP::TopLevelImport(import_p) = denizen {
        imports.push(&*self.scout_arena.alloc(self.scout_import(file_coordinate, import_p)));
      }
    }

    Ok(ProgramS {
      structs: self.scout_arena.alloc_slice_from_vec(structs),
      interfaces: self.scout_arena.alloc_slice_from_vec(interfaces),
      impls: self.scout_arena.alloc_slice_from_vec(impls),
      implemented_functions: self.scout_arena.alloc_slice_from_vec(implemented_functions),
      exports: self.scout_arena.alloc_slice_from_vec(exports),
      imports: self.scout_arena.alloc_slice_from_vec(imports),
    })
  }

fn scout_impl(
  &self,
  file: &'s FileCoordinate<'s>,
  impl0: &ImplP<'p>,
) -> Result<ImplS<'s>, ICompileErrorS<'s>> {
  let range_s = PostParser::eval_range(file, impl0.range);

  let template_rules_p: &[IRulexPR<'p>] = impl0
    .template_rules
    .as_ref()
    .map(|template_rules_p| template_rules_p.rules)
    .unwrap_or(&[]);

  let code_location = range_s.begin.clone();
  let impl_name = ImplDeclarationNameS { code_location };

  let user_specified_identifying_runes = impl0
    .generic_params
    .as_ref()
    .map(|generic_parameters_p| {
      generic_parameters_p
        .params
        .iter()
        .map(|generic_parameter_p| RuneUsage {
          range: PostParser::eval_range(file, generic_parameter_p.name.range()),
          rune: self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
            name: self.scout_arena.intern_str(generic_parameter_p.name.str().as_str()),
          })),
        })
        .collect::<Vec<_>>()
    })
    .unwrap_or_default();

  // Mirrors:
  // RulePUtils.getOrderedRuneDeclarationsFromRulexesWithDuplicates(templateRulesP)
  let runes_from_rules =
    get_ordered_rune_declarations_from_rulexes_with_duplicates(
      template_rules_p,
    )
    .into_iter()
    .map(|name_p| RuneUsage {
      range: PostParser::eval_range(file, name_p.range()),
      rune: self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS { name: self.scout_arena.intern_str(name_p.str().as_str()) })),
    })
    .collect::<Vec<_>>();

  let mut user_declared_runes = user_specified_identifying_runes.clone();
  user_declared_runes.extend(runes_from_rules);

  let user_declared_runes_set = user_declared_runes
    .iter()
    .map(|rune_usage| rune_usage.rune.clone())
    .collect::<Vec<_>>();

  let impl_env = IEnvironmentS::Environment(EnvironmentS {
    file,
    parent_env: None,
    name: self.scout_arena.intern_name(INameValS::ImplDeclaration(impl_name.clone())),
    user_declared_runes: user_declared_runes
      .iter()
      .map(|rune_usage| rune_usage.rune.clone())
      .collect(),
  });

  let mut lidb = LocationInDenizenBuilder::new(Vec::new());
  let mut rule_builder = Vec::<IRulexSR<'s>>::new();
  let mut rune_to_explicit_type = Vec::<(IRuneS<'s>, ITemplataType<'s>)>::new();

  let default_region_rune_range_s = RangeS::new(
    range_s.end.clone(),
    range_s.end.clone(),
  );
  let default_region_rune_s = self.scout_arena.intern_rune(IRuneValS::DenizenDefaultRegionRune(
    DenizenDefaultRegionRuneS {
      denizen_name: self.scout_arena.intern_name(INameValS::ImplDeclaration(impl_name.clone())),
    },
  ));
  let maybe_region_generic_param = Some(GenericParameterS {
    range: default_region_rune_range_s.clone(),
    rune: RuneUsage {
      range: default_region_rune_range_s.clone(),
      rune: default_region_rune_s.clone(),
    },
    tyype: IGenericParameterTypeS::RegionGenericParameterType(
      RegionGenericParameterTypeS {},
    ),
    default: None,
  });

  let generic_parameters_p = impl0
    .generic_params
    .as_ref()
    .map(|generic_parameters_p| generic_parameters_p.params)
    .unwrap_or(&[]);

  // We'll add the implicit runes to the end, see IRRAE.
  let user_specified_generic_parameters_s: Vec<&'s GenericParameterS<'s>> = generic_parameters_p
    .iter()
    .zip(user_specified_identifying_runes.iter())
    .map(|(g, r)| {
      let mut child_lidb = lidb.child();
      &*self.scout_arena.alloc(self.scout_generic_parameter(
        impl_env.clone(),
        &mut child_lidb,
        &mut rune_to_explicit_type,
        &mut rule_builder,
        default_region_rune_s.clone(),
        g,
        r.clone(),
      ))
    })
    .collect::<Vec<_>>();
  let _user_specified_runes_implicit_region_runes_s =
    maybe_region_generic_param.as_ref().map(|_x| Vec::<GenericParameterS<'s>>::new());

  {
    let mut child_lidb = lidb.child();
    translate_rulexes(self.scout_arena,
      self.keywords,
      impl_env.clone(),
      &mut child_lidb,
      &mut rule_builder,
      &mut rune_to_explicit_type,
      default_region_rune_s.clone(),
      template_rules_p,
    );
  }

  let struct_ = match &impl0.struct_ {
    None => {
      return Err(ICompileErrorS::RangedInternalErrorS(RangedInternalErrorS {
        range: range_s.clone(),
        message: "Impl needs struct!".to_string(),
      }));
    }
    Some(x) => x,
  };

  let struct_rune = {
    let mut child_lidb = lidb.child();
    translate_maybe_type_into_rune(
      self.scout_arena,
        self.keywords,
      impl_env.clone(),
      &mut child_lidb,
      range_s.clone(),
      &mut rule_builder,
      default_region_rune_s.clone(),
      Some(struct_),
    )
  };

  let interface_rune = {
    let mut child_lidb = lidb.child();
    translate_maybe_type_into_rune(
      self.scout_arena,
        self.keywords,
      impl_env.clone(),
      &mut child_lidb,
      range_s.clone(),
      &mut rule_builder,
      default_region_rune_s.clone(),
      Some(&impl0.interface),
    )
  };

  let sub_citizen_imprecise_name = match struct_ {
    ITemplexPT::Call(call)
      if matches!(call.template, ITemplexPT::NameOrRune(_))
        && match call.template {
          ITemplexPT::NameOrRune(name)
            if !user_declared_runes_set.contains(&self.scout_arena.intern_rune(IRuneValS::CodeRune(
              CodeRuneS {
                name: self.scout_arena.intern_str(name.name.str().as_str()),
              },
            ))) =>
          {
            true
          }
          _ => false,
        } =>
    {
      let ITemplexPT::NameOrRune(name) = call.template else {
        panic!("POSTPARSER_SCOUT_IMPL_IMPOSSIBLE_CALL_TEMPLATE_SHAPE");
      };
      self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
        name: self.scout_arena.intern_str(name.name.str().as_str()),
      }))
    }
    ITemplexPT::NameOrRune(name)
      if !user_declared_runes_set.contains(&self.scout_arena.intern_rune(IRuneValS::CodeRune(
        CodeRuneS {
          name: self.scout_arena.intern_str(name.name.str().as_str()),
        },
      ))) =>
    {
      self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
        name: self.scout_arena.intern_str(name.name.str().as_str()),
      }))
    }
    _ => {
      return Err(ICompileErrorS::RangedInternalErrorS(RangedInternalErrorS {
        range: PostParser::eval_range(file, struct_.range()),
        message: "Can't determine name of struct!".to_string(),
      }));
    }
  };

  let super_interface_imprecise_name = match &impl0.interface {
    ITemplexPT::Call(call)
      if matches!(call.template, ITemplexPT::NameOrRune(_))
        && match call.template {
          ITemplexPT::NameOrRune(name)
            if !user_declared_runes_set.contains(&self.scout_arena.intern_rune(IRuneValS::CodeRune(
              CodeRuneS {
                name: self.scout_arena.intern_str(name.name.str().as_str()),
              },
            ))) =>
          {
            true
          }
          _ => false,
        } =>
    {
      let ITemplexPT::NameOrRune(name) = call.template else {
        panic!("POSTPARSER_SCOUT_IMPL_IMPOSSIBLE_CALL_TEMPLATE_SHAPE");
      };
      self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
        name: self.scout_arena.intern_str(name.name.str().as_str()),
      }))
    }
    ITemplexPT::NameOrRune(name)
      if !user_declared_runes_set.contains(&self.scout_arena.intern_rune(IRuneValS::CodeRune(
        CodeRuneS {
          name: self.scout_arena.intern_str(name.name.str().as_str()),
        },
      ))) =>
    {
      self.scout_arena.intern_imprecise_name(IImpreciseNameValS::CodeName(CodeNameS {
        name: self.scout_arena.intern_str(name.name.str().as_str()),
      }))
    }
    _ => {
      return Err(ICompileErrorS::RangedInternalErrorS(RangedInternalErrorS {
        range: PostParser::eval_range(file, struct_.range()),
        message: "Can't determine name of struct!".to_string(),
      }));
    }
  };

  let generic_parameters_s = user_specified_generic_parameters_s;
  // ++ userSpecifiedRunesImplicitRegionRunesS
  let _maybe_region_generic_param = maybe_region_generic_param;

  let param_types_vec: Vec<ITemplataType<'s>> = generic_parameters_s
      .iter()
      .map(|generic_parameter_s| generic_parameter_s.tyype.tyype())
      .collect();
  let tyype = ITemplataType::TemplateTemplataType(TemplateTemplataType {
    param_types: self.scout_arena.alloc_slice_copy(&param_types_vec),
    return_type: self.scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {})),
  });

  Ok(ImplS::new(
    range_s,
    IImplDeclarationNameS::ImplDeclarationName(impl_name),
    self.scout_arena.alloc_slice_from_vec(generic_parameters_s),
    self.scout_arena.alloc_slice_from_vec(rule_builder),
    tyype,
    struct_rune,
    sub_citizen_imprecise_name,
    interface_rune,
    super_interface_imprecise_name,
  ))
}

fn scout_export_as(
  &self,
  file: &'s FileCoordinate<'s>,
  export_as_p: &ExportAsP<'p>,
) -> ExportAsS<'s> {
  let range_s = Self::eval_range(file, export_as_p.range);
  let pos = range_s.begin.clone();
  let export_name = self.scout_arena.intern_name(INameValS::ExportAsName(ExportAsNameS { code_location: pos }));
  let export_env = IEnvironmentS::Environment(EnvironmentS {
    file,
    parent_env: None,
    name: export_name,
    user_declared_runes: Default::default(),
  });
  let mut lidb = LocationInDenizenBuilder::new(Vec::new());
  let mut rule_builder = Vec::<IRulexSR<'s>>::new();
  let mut rune_to_explicit_type = Vec::<(IRuneS<'s>, ITemplataType<'s>)>::new();
  let region_range = RangeS::new(range_s.end.clone(), range_s.end.clone());
  let default_region_rune_s = self.scout_arena.intern_rune(IRuneValS::DenizenDefaultRegionRune(
    DenizenDefaultRegionRuneS { denizen_name: export_name },
  ));
  rune_to_explicit_type.push((default_region_rune_s.clone(), ITemplataType::RegionTemplataType(RegionTemplataType {})));
  let _region_generic_param = GenericParameterS {
    range: region_range.clone(),
    rune: RuneUsage { range: region_range, rune: default_region_rune_s.clone() },
    tyype: IGenericParameterTypeS::RegionGenericParameterType(RegionGenericParameterTypeS {}),
    default: None,
  };
  let rune_s = translate_templex(
    self.scout_arena,
    self.keywords,
    export_env,
    &mut lidb,
    &mut rule_builder,
    default_region_rune_s,
    &export_as_p.struct_,
  );
  ExportAsS {
    range: range_s,
    rules: self.scout_arena.alloc_slice_from_vec(rule_builder),
    export_name: ExportAsNameS { code_location: pos },
    rune: rune_s,
    exported_name: self.scout_arena.intern_str(export_as_p.exported_name.str().as_str()),
  }
}

fn scout_import(
  &self,
  file: &'s FileCoordinate<'s>,
  import_p: &ImportP<'p>,
) -> ImportS<'s> {
  let _pos = PostParser::eval_pos(file, import_p.range.begin());

  ImportS {
    range: PostParser::eval_range(file, import_p.range),
    module_name: self.scout_arena.intern_str(import_p.module_name.str().as_str()),
    package_names: self.scout_arena.alloc_slice_from_vec(
      import_p.package_steps.iter().map(|n| self.scout_arena.intern_str(n.str().as_str())).collect(),
    ),
    importee_name: self.scout_arena.intern_str(import_p.importee_name.str().as_str()),
  }
}

  fn scout_struct(
    &self,
    file: &'s FileCoordinate<'s>,
    head: &StructP<'p>,
  ) -> Result<StructS<'s>, ICompileErrorS<'s>> {
    let struct_range_s = Self::eval_range(file, head.range);
    let struct_name = self.scout_arena.intern_struct_declaration_name(TopLevelStructDeclarationNameS {
      name: self.scout_arena.intern_str(head.name.str().as_str()),
      range: Self::eval_range(file, head.name.range()),
    });
    let body_range_s = Self::eval_range(file, head.body_range);
    let mut lidb = LocationInDenizenBuilder::new(Vec::new());

    let generic_parameters_p: &[GenericParameterP<'p>] = head
      .identifying_runes
      .as_ref()
      .map(|x| x.params as &[GenericParameterP<'p>])
      .unwrap_or(&[]);
    let user_specified_identifying_runes = generic_parameters_p
      .iter()
      .map(|generic_parameter| RuneUsage {
        range: Self::eval_range(file, generic_parameter.name.range()),
        rune: self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
          name: self.scout_arena.intern_str(generic_parameter.name.str().as_str()),
        })),
      })
      .collect::<Vec<_>>();
    let template_rules_p: &[IRulexPR<'p>] = head
      .template_rules
      .as_ref()
      .map(|x| x.rules as &[IRulexPR<'p>])
      .unwrap_or(&[]);
    let runes_from_rules =
      get_ordered_rune_declarations_from_rulexes_with_duplicates(&template_rules_p)
      .iter()
      .map(|name_p| RuneUsage {
        range: Self::eval_range(file, name_p.range()),
        rune: self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
          name: self.scout_arena.intern_str(name_p.str().as_str()),
        })),
      })
      .collect::<Vec<_>>();
    let user_declared_runes = user_specified_identifying_runes
      .iter()
      .chain(runes_from_rules.iter())
      .cloned()
      .collect::<Vec<_>>();
    let struct_env = IEnvironmentS::Environment(EnvironmentS {
      file,
      parent_env: None,
      name: self.scout_arena.intern_name(INameValS::TopLevelStructDeclaration(struct_name.clone())),
      user_declared_runes: user_declared_runes
        .iter()
        .map(|x| x.rune.clone())
        .collect(),
    });

    let mut header_rule_builder = Vec::<IRulexSR<'s>>::new();
    let mut header_rune_to_explicit_type = Vec::<(IRuneS<'s>, ITemplataType<'s>)>::new();

    let (_default_region_rune_range_s, default_region_rune_s, _maybe_region_generic_param) =
      match &head.maybe_default_region_rune {
        None => {
          let region_range = RangeS::new(
            body_range_s.begin.clone(),
            body_range_s.begin.clone(),
          );
          let rune = self
            .scout_arena
            .intern_rune(IRuneValS::DenizenDefaultRegionRune(
              DenizenDefaultRegionRuneS {
              denizen_name: self.scout_arena.intern_name(INameValS::TopLevelStructDeclaration(
                struct_name.clone(),
              )),
            }));
          // Put back in when we have regions
          // header_rune_to_explicit_type.push((rune.clone(), ITemplataType::RegionTemplataType(RegionTemplataType {})));
          let implicit_region_generic_param = GenericParameterS {
            range: region_range.clone(),
            rune: RuneUsage {
              range: region_range.clone(),
              rune: rune.clone(),
            },
            tyype: IGenericParameterTypeS::RegionGenericParameterType(
              RegionGenericParameterTypeS {},
            ),
            default: None,
          };
          (region_range, rune, Some(implicit_region_generic_param))
        }
        Some(region_rune_p) => {
          let region_range_s = Self::eval_range(file, region_rune_p.range);
          let region_name = match &region_rune_p.name {
            None => panic!("impl isolates"),
            Some(name) => name.str(),
          };
          let rune = self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
            name: self.scout_arena.intern_str(region_name.as_str()),
          }));
          if !struct_env.all_declared_runes().contains(&rune) {
            return Err(ICompileErrorS::CouldntFindRuneS(CouldntFindRuneS {
              range: region_range_s.clone(),
              name: region_name.as_str().to_string(),
            }));
          }
          (region_range_s, rune, None)
        }
      };

    let struct_user_specified_generic_parameters_s: Vec<&'s GenericParameterS<'s>> = generic_parameters_p
      .iter()
      .zip(user_specified_identifying_runes.iter())
      .map(|(g, r)| {
        &*self.scout_arena.alloc(self.scout_generic_parameter(
          struct_env.clone(),
          &mut lidb.child(),
          &mut header_rune_to_explicit_type,
          &mut header_rule_builder,
          default_region_rune_s.clone(),
          g,
          r.clone(),
        ))
      })
      .collect::<Vec<_>>();
    // Put back in when we have regions
    // let generic_parameters_s = struct_user_specified_generic_parameters_s ++ maybe_region_generic_param ++ user_specified_runes_implicit_region_runes_s;
    let generic_parameters_s = struct_user_specified_generic_parameters_s;

    translate_rulexes(self.scout_arena,
      self.keywords,
      struct_env.clone(),
      &mut lidb.child(),
      &mut header_rule_builder,
      &mut header_rune_to_explicit_type,
      default_region_rune_s.clone(),
      &template_rules_p,
    );

    let mut member_rule_builder = Vec::<IRulexSR<'s>>::new();
    let mut members_rune_to_explicit_type = self.scout_arena.alloc_index_map::<IRuneS, ITemplataType<'s>>();

    let mut internal_methods_p = Vec::<&'p FunctionP<'p>>::new();
    let members_s = head
      .members
      .contents
      .iter()
      .flat_map(|member| match member {
        IStructContent::NormalStructMember(member) => {
          let member_rune = translate_templex(
            self.scout_arena,
            self.keywords,
            struct_env.clone(),
            &mut lidb.child(),
            &mut member_rule_builder,
            default_region_rune_s.clone(),
            &member.tyype,
          );
          members_rune_to_explicit_type.insert(
            member_rune.rune.clone(),
            ITemplataType::KindTemplataType(KindTemplataType {}),
          );
          vec![IStructMemberS::NormalStructMember(NormalStructMemberS {
            range: Self::eval_range(file, member.range),
            name: self.scout_arena.intern_str(member.name.str().as_str()),
            type_rune: member_rune,
          })]
        }
        IStructContent::VariadicStructMember(member) => {
          let member_rune = translate_templex(
            self.scout_arena,
            self.keywords,
            struct_env.clone(),
            &mut lidb.child(),
            &mut member_rule_builder,
            default_region_rune_s.clone(),
            &member.tyype,
          );
          members_rune_to_explicit_type.insert(
            member_rune.rune.clone(),
            ITemplataType::PackTemplataType(PackTemplataType {
              element_type: &*self.scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {})),
            }),
          );
          vec![IStructMemberS::VariadicStructMember(VariadicStructMemberS {
            range: Self::eval_range(file, member.range),
            type_rune: member_rune,
          })]
        }
        IStructContent::StructMethod(func_p) => {
          internal_methods_p.push(func_p);
          vec![]
        }
      })
      .collect::<Vec<_>>();

    let header_rules_s = header_rule_builder;
    let member_rules_s = member_rule_builder;
    let all_rules_s = header_rules_s
      .iter()
      .chain(member_rules_s.iter())
      .cloned()
      .collect::<Vec<_>>();
    let mut all_rune_to_explicit_type = header_rune_to_explicit_type.clone();
    all_rune_to_explicit_type.extend(
      members_rune_to_explicit_type
        .iter()
        .map(|(rune, tyype)| (rune.clone(), tyype.clone())),
    );

    let param_types_vec: Vec<ITemplataType<'s>> = generic_parameters_s
        .iter()
        .map(|x| x.tyype.tyype())
        .collect();
    let tyype = TemplateTemplataType {
      param_types: self.scout_arena.alloc_slice_copy(&param_types_vec),
      return_type: self.scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {})),
    };
    let weakable = head
      .attributes
      .iter()
      .any(|attr| matches!(attr, IAttributeP::WeakableAttribute(_)));
    let attrs_without_linear_s = Self::translate_citizen_attributes(
      self.scout_arena,
      file,
      self.scout_arena.intern_name(INameValS::TopLevelStructDeclaration(struct_name.clone())),
      &head
        .attributes
        .iter()
        .filter(|attr| {
          !matches!(
            attr,
            IAttributeP::WeakableAttribute(_)
          )
        })
        .cloned()
        .collect::<Vec<_>>(),
    );
    let attrs_s = attrs_without_linear_s;

    let generic_parameters_s_arena: &'s [&'s GenericParameterS<'s>] = self.scout_arena.alloc_slice_from_vec(generic_parameters_s.clone());
    let internal_methods_s_vec: Vec<&'s FunctionS<'s>> = internal_methods_p.iter().map(|method| -> Result<&'s FunctionS<'s>, ICompileErrorS<'s>> {
      self.scout_interface_member(
        ParentCitizen {
          citizen_is_interface: false,
          citizen_env: struct_env.clone(),
          citizen_generic_params: generic_parameters_s_arena,
          citizen_rules: all_rules_s.clone(),
          citizen_rune_to_explicit_type: all_rune_to_explicit_type.iter().map(|(r, t)| (r.clone(), t.clone())).collect(),
        },
        method,
      )
    }).collect::<Result<Vec<_>, _>>()?;

    Ok(StructS::new(
      struct_range_s,
      IStructDeclarationNameS::TopLevelStructDeclarationName(*struct_name),
      self.scout_arena.alloc_slice_from_vec(attrs_s),
      weakable,
      self.scout_arena.alloc_slice_from_vec(generic_parameters_s),
      head.sharedness,
      tyype,
      self.scout_arena.alloc_slice_from_vec(header_rules_s),
      self.scout_arena.alloc_slice_from_vec(member_rules_s),
      self.scout_arena.alloc_slice_from_vec(members_s),
      self.scout_arena.alloc_slice_from_vec(internal_methods_s_vec),
    ))
  }

fn translate_citizen_attributes(
  interner: &ScoutArena<'s>,
  file: &'s FileCoordinate<'s>,
  _denizen_name: INameS<'s>,
  attrs_p: &[IAttributeP<'p>],
) -> Vec<ICitizenAttributeS<'s>> {
  attrs_p
    .iter()
    .map(|attr_p| match attr_p {
      IAttributeP::ExportAttribute(_) => {
        ICitizenAttributeS::Export(ExportS {
          package_coordinate: file.package_coord,
        })
      }
      IAttributeP::ExternAttribute(_) => {
        ICitizenAttributeS::Extern(ExternS {
          package_coord: file.package_coord,
        })
      }
      IAttributeP::SealedAttribute(_) => {
        ICitizenAttributeS::Sealed(SealedS)
      }
      IAttributeP::MacroCall(macro_call_p) => {
        ICitizenAttributeS::MacroCall(MacroCallS {
          range: PostParser::eval_range(file, macro_call_p.range),
          include: macro_call_p.inclusion,
          macro_name: interner.intern_str(macro_call_p.name.str().as_str()),
        })
      }
      _ => panic!("POSTPARSER_TRANSLATE_CITIZEN_ATTRIBUTES_NOT_YET_IMPLEMENTED"),
    })
    .collect()
}

  fn scout_interface(
    &self,
    file: &'s FileCoordinate<'s>,
    interface: &InterfaceP<'p>,
  ) -> Result<InterfaceS<'s>, ICompileErrorS<'s>> {
    let interface_range_s = Self::eval_range(file, interface.range);
    let _body_range_s = Self::eval_range(file, interface.body_range);
    let interface_name = self.scout_arena.intern_interface_declaration_name(TopLevelInterfaceDeclarationNameS {
      name: self.scout_arena.intern_str(interface.name.str().as_str()),
      range: Self::eval_range(file, interface.name.range()),
    });
    let rules_p: &[IRulexPR<'p>] = interface
      .template_rules
      .as_ref()
      .map(|x| x.rules as &[IRulexPR<'p>])
      .unwrap_or(&[]);

    let mut lidb = LocationInDenizenBuilder::new(Vec::new());
    assert!(
      interface.maybe_default_region_rune.is_none(),
      "POSTPARSER_SCOUT_INTERFACE_DEFAULT_REGION_RUNE_NOT_YET_IMPLEMENTED"
    );

    let weakable = interface
      .attributes
      .iter()
      .any(|attr| matches!(attr, IAttributeP::WeakableAttribute(_)));
    let attrs_without_linear_s = Self::translate_citizen_attributes(
      self.scout_arena,
      file,
      self.scout_arena.intern_name(INameValS::TopLevelInterfaceDeclaration(interface_name.clone())),
      &interface
        .attributes
        .iter()
        .filter(|attr| {
          !matches!(
            attr,
            IAttributeP::WeakableAttribute(_)
          )
        })
        .cloned()
        .collect::<Vec<_>>(),
    );
    let attributes = attrs_without_linear_s;

    let generic_parameters_p: &[GenericParameterP<'p>] = interface
      .maybe_identifying_runes
      .as_ref()
      .map(|x| x.params as &[GenericParameterP<'p>])
      .unwrap_or(&[]);

    let user_specified_identifying_runes = generic_parameters_p
      .iter()
      .map(|generic_parameter| RuneUsage {
        range: Self::eval_range(file, generic_parameter.name.range()),
        rune: self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
          name: self.scout_arena.intern_str(generic_parameter.name.str().as_str()),
        })),
      })
      .collect::<Vec<_>>();

    let runes_from_rules =
      get_ordered_rune_declarations_from_rulexes_with_duplicates(&rules_p)
      .iter()
      .map(|name_p| RuneUsage {
        range: Self::eval_range(file, name_p.range()),
        rune: self.scout_arena.intern_rune(IRuneValS::CodeRune(CodeRuneS {
          name: self.scout_arena.intern_str(name_p.str().as_str()),
        })),
      })
      .collect::<Vec<_>>();
    let user_declared_runes: IndexSet<IRuneS<'s>> = user_specified_identifying_runes
      .iter()
      .chain(runes_from_rules.iter())
      .map(|x| x.rune.clone())
      .collect();
    let interface_env = IEnvironmentS::Environment(EnvironmentS {
      file,
      parent_env: None,
      name: self.scout_arena.intern_name(INameValS::TopLevelInterfaceDeclaration(interface_name.clone())),
      user_declared_runes,
    });

    let mut rule_builder = Vec::<IRulexSR<'s>>::new();
    // This is an array instead of a map so we can detect conflicts afterward
    let mut rune_to_explicit_type = Vec::<(IRuneS<'s>, ITemplataType<'s>)>::new();

    // Put this back in when we have regions
    let default_region_rune_s = self.scout_arena.intern_rune(IRuneValS::DenizenDefaultRegionRune(
      DenizenDefaultRegionRuneS {
        denizen_name: self.scout_arena.intern_name(INameValS::TopLevelInterfaceDeclaration(
          interface_name.clone(),
        )),
      },
    ));

    let generic_parameters_s: Vec<&'s GenericParameterS<'s>> = generic_parameters_p
      .iter()
      .zip(user_specified_identifying_runes.iter())
      .map(|(g, r)| {
        &*self.scout_arena.alloc(self.scout_generic_parameter(
          interface_env.clone(),
          &mut lidb.child(),
          &mut rune_to_explicit_type,
          &mut rule_builder,
          default_region_rune_s.clone(),
          g,
          r.clone(),
        ))
      })
      .collect::<Vec<_>>();

    translate_rulexes(self.scout_arena,
      self.keywords,
      interface_env.clone(),
      &mut lidb.child(),
      &mut rule_builder,
      &mut rune_to_explicit_type,
      default_region_rune_s.clone(),
      &rules_p,
    );

    let rules_s = rule_builder;

    let generic_parameters_s: &'s [&'s GenericParameterS<'s>] = self.scout_arena.alloc_slice_from_vec(generic_parameters_s);

    let param_types_vec: Vec<ITemplataType<'s>> = generic_parameters_s.iter().map(|x| x.tyype.tyype()).collect();
    let tyype = TemplateTemplataType {
      param_types: self.scout_arena.alloc_slice_copy(&param_types_vec),
      return_type: self.scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {})),
    };

    let mut internal_methods = Vec::new();
    for member in interface.members {
      internal_methods.push(self.scout_interface_member(
        ParentCitizen {
          citizen_is_interface: true,
          citizen_env: interface_env.clone(),
          citizen_generic_params: generic_parameters_s,
          citizen_rules: rules_s.clone(),
          citizen_rune_to_explicit_type: rune_to_explicit_type.iter().cloned().collect(),
        },
        member,
      )?);
    }

    Ok(InterfaceS::new(
      interface_range_s,
      interface_name,
      self.scout_arena.alloc_slice_from_vec(attributes),
      weakable,
      generic_parameters_s,
      interface.sharedness,
      tyype,
      self.scout_arena.alloc_slice_from_vec(rules_s),
      self.scout_arena.alloc_slice_from_vec(internal_methods),
    ))
  }

}


pub use scout_compilation::ScoutCompilation;
// Wrapped in a private submodule for _sealed to work.
mod scout_compilation {
  use super::*;

  pub struct ScoutCompilation<'s, 'ctx, 'p> {
    global_options: GlobalOptions,
    scout_arena: &'ctx ScoutArena<'s>,
    keywords: &'ctx Keywords<'s>,
    keywords_p: &'ctx Keywords<'p>,
    parse_arena: &'ctx ParseArena<'p>,
    parser_compilation: ParserCompilation<'p, 'ctx>,
    scoutput_cache: Option<FileCoordinateMap<'s, ProgramS<'s>>>,
  }

  impl<'s, 'ctx, 'p> ScoutCompilation<'s, 'ctx, 'p>
  {
    pub fn new(
      scout_arena: &'ctx ScoutArena<'s>,
      keywords: &'ctx Keywords<'s>,
      parser_keywords: &'ctx Keywords<'p>,
      parse_arena: &'ctx ParseArena<'p>,
      packages_to_build: Vec<&'p PackageCoordinate<'p>>,
      code_source: &'ctx CodeSource<'p>,
      global_options: GlobalOptions,
    ) -> Self {
      let parser_compilation = ParserCompilation::new(
        global_options.clone(),
        parse_arena,
        parser_keywords,
        packages_to_build,
        code_source,
      );

      ScoutCompilation {
        global_options,
        scout_arena,
        keywords,
        keywords_p: parser_keywords,
        parse_arena,
        parser_compilation,
        scoutput_cache: None,
      }
    }


    pub fn get_code_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
      self.parser_compilation.get_code_map()
    }


    pub fn get_parseds(&mut self) -> Result<FileCoordinateMap<'p, (FileP<'p>, Vec<RangeL>)>, FailedParse<'p>> {
      self.parser_compilation.get_parseds()
    }


    pub fn get_vpst_map(&mut self) -> Result<FileCoordinateMap<'p, String>, FailedParse<'p>> {
      self.parser_compilation.get_vpst_map()
    }


    pub fn get_scoutput(&mut self) -> Result<&FileCoordinateMap<'s, ProgramS<'s>>, ICompileErrorS<'s>> {
      if self.scoutput_cache.is_some() {
        return Ok(self.scoutput_cache.as_ref().unwrap());
      }

      let parseds = self.parser_compilation.expect_parseds();
      let post_parser = PostParser::new(
        self.global_options.clone(),
        self.scout_arena,
        self.keywords,
        self.keywords_p,
        self.parse_arena,
      );
      let mut scoutput: FileCoordinateMap<'s, ProgramS<'s>> = FileCoordinateMap::new();
      for (file_coordinate_p, (file_p, _comments_and_ranges)) in &parseds.file_coord_to_contents {
        let package_coord_s: &'s PackageCoordinate<'s> = self.scout_arena.intern_package_coordinate(
          self.scout_arena.intern_str(file_coordinate_p.package_coord.module.as_str()),
          &file_coordinate_p.package_coord.packages.iter()
            .map(|s| self.scout_arena.intern_str(s.as_str()))
            .collect::<Vec<_>>(),
        );
        let file_coordinate_s: &'s FileCoordinate<'s> = self.scout_arena.intern_file_coordinate(
          package_coord_s,
          file_coordinate_p.filepath.as_str(),
        );
        let program_s = post_parser.scout_program(file_coordinate_s, file_p)?;
        scoutput.put(file_coordinate_s, program_s);
      }
      // Re-intern package_coord_to_file_coords from 'p to 's
      for (pkg_p, files_p) in &parseds.package_coord_to_file_coords {
        let pkg_s = self.scout_arena.intern_package_coordinate(
          self.scout_arena.intern_str(pkg_p.module.as_str()),
          &pkg_p.packages.iter().map(|s| self.scout_arena.intern_str(s.as_str())).collect::<Vec<_>>(),
        );
        let files_s: Vec<&'s FileCoordinate<'s>> = files_p.iter().map(|fc| {
          self.scout_arena.intern_file_coordinate(pkg_s, fc.filepath.as_str())
        }).collect();
        scoutput.package_coord_to_file_coords.insert(pkg_s, files_s);
      }
      self.scoutput_cache = Some(scoutput);
      Ok(self.scoutput_cache.as_ref().unwrap())
    }


    pub fn expect_scoutput(&mut self) -> &FileCoordinateMap<'s, ProgramS<'s>> {
      match self.get_scoutput() {
        Ok(x) => x,
        Err(e) => {
          panic!("ScoutCompilation.expect_scoutput failed: {:?}", e)
        }
      }
    }
  }
}

