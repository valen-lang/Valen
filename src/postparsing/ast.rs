use crate::interner::StrI;
use crate::parsing::ast::{IMacroInclusionP, SharednessP};
use crate::postparsing::expressions::BodySE;
use crate::postparsing::itemplatatype::{
  ITemplataType, KindTemplataType, RegionTemplataType, TemplateTemplataType,
};
use crate::postparsing::names::{
  ExportAsNameS, IFunctionDeclarationNameS, IImplDeclarationNameS, IImpreciseNameS, IRuneS,
  IStructDeclarationNameS, IVarNameS, ImplDeclarationNameS, TopLevelCitizenDeclarationNameS,
  TopLevelInterfaceDeclarationNameS, TopLevelStructDeclarationNameS,
};
use crate::postparsing::patterns::AtomSP;
use crate::postparsing::rules::types::ITypeST;
use crate::postparsing::rules::{IRulexSR, ImplBoundS, RuneUsage};
use crate::scout_arena::ScoutArena;
use crate::utils::arena_index_map::ArenaIndexMap;
use crate::utils::code_hierarchy::PackageCoordinate;
use crate::utils::range::RangeS;

pub trait IExpressionSE<'s> {
  fn range(&self) -> RangeS<'s>;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ProgramS<'s> {
  pub structs: &'s [&'s StructS<'s>],
  pub interfaces: &'s [&'s InterfaceS<'s>],
  pub impls: &'s [&'s ImplS<'s>],
  // VCOORD: why is this called implemented_functions?
  pub implemented_functions: &'s [&'s FunctionS<'s>],
  pub exports: &'s [&'s ExportAsS<'s>],
  pub imports: &'s [&'s ImportS<'s>],
}

impl<'s> ProgramS<'s> {
  pub fn lookup_function(&'s self, name: &str) -> &'s FunctionS<'s> {
    let matches: Vec<&'s FunctionS<'s>> = self
      .implemented_functions
      .iter()
      .filter(|f| match &f.name {
        IFunctionDeclarationNameS::FunctionName(n) => n.name.as_str() == name,
        _ => false,
      })
      .map(|f| *f)
      .collect::<Vec<&'s FunctionS<'s>>>();
    assert_eq!(matches.len(), 1);
    matches[0]
  }

  pub fn lookup_interface(&self, name: &str) -> &'s InterfaceS<'s> {
    let matches = self.interfaces.iter().copied().find(|i| i.name.name.as_str() == name);
    assert_eq!(matches.is_some(), true);
    matches.unwrap()
  }

  pub fn lookup_struct(&self, name: &str) -> &'s StructS<'s> {
    let matches: Vec<&'s StructS<'s>> = self
      .structs
      .iter()
      .copied()
      .filter(|s| s.name.expect_top_level().name.as_str() == name)
      .collect();
    assert_eq!(matches.len(), 1);
    matches[0]
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ICitizenAttributeS<'s> {
  Extern(ExternS<'s>),
  Sealed(SealedS),
  Builtin(BuiltinS<'s>),
  MacroCall(MacroCallS<'s>),
  Export(ExportS<'s>),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IFunctionAttributeS<'s> {
  Extern(ExternS<'s>),
  Builtin(BuiltinS<'s>),
  Export(ExportS<'s>),
  UserFunction(UserFunctionS),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExternS<'s> {
  pub package_coord: &'s PackageCoordinate<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct SealedS;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BuiltinS<'s> {
  // AFTERM: can we give everything a lifetime into an arena so we can
  // all have references instead of using Arc everywhere?
  pub generator_name: StrI<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct MacroCallS<'s> {
  pub range: RangeS<'s>,
  pub include: IMacroInclusionP,
  pub macro_name: StrI<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExportS<'s> {
  pub package_coordinate: &'s PackageCoordinate<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UserFunctionS;

#[derive(Debug, PartialEq)]
pub enum ICitizenS<'s> {
  Struct(StructS<'s>),
  Interface(InterfaceS<'s>),
}

impl<'s> ICitizenS<'s> {
  pub fn name(&self) -> TopLevelCitizenDeclarationNameS<'_> {
    match self {
      ICitizenS::Struct(s) => TopLevelCitizenDeclarationNameS::from(s.name.expect_top_level()),
      ICitizenS::Interface(i) => TopLevelCitizenDeclarationNameS::from(i.name),
    }
  }

  pub fn tyype(&self) -> &TemplateTemplataType<'s> {
    match self {
      ICitizenS::Struct(s) => &s.tyype,
      ICitizenS::Interface(i) => &i.tyype,
    }
  }

  pub fn generic_params(&self) -> &'s [&'s GenericParameterS<'s>] {
    match self {
      ICitizenS::Struct(s) => s.generic_params,
      ICitizenS::Interface(i) => i.generic_params,
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct StructS<'s> {
  pub range: RangeS<'s>,
  pub name: IStructDeclarationNameS<'s>,
  pub attributes: &'s [ICitizenAttributeS<'s>],
  pub weakable: bool,
  pub generic_params: &'s [&'s GenericParameterS<'s>],
  pub sharedness: SharednessP,
  pub tyype: TemplateTemplataType<'s>,
  pub header_rules: &'s [IRulexSR<'s>],
  pub member_rules: &'s [IRulexSR<'s>],
  pub members: &'s [IStructMemberS<'s>],
  pub internal_methods: &'s [&'s FunctionS<'s>],
  /// `where implements(Sub, Super)` clauses; see ImplBoundS.
  pub impl_bounds: &'s [ImplBoundS<'s>],
  _sealed: (),
}

impl<'s> StructS<'s> {
  pub fn new(
    range: RangeS<'s>,
    name: IStructDeclarationNameS<'s>,
    attributes: &'s [ICitizenAttributeS<'s>],
    weakable: bool,
    generic_params: &'s [&'s GenericParameterS<'s>],
    sharedness: SharednessP,
    tyype: TemplateTemplataType<'s>,
    header_rules: &'s [IRulexSR<'s>],
    member_rules: &'s [IRulexSR<'s>],
    members: &'s [IStructMemberS<'s>],
    internal_methods: &'s [&'s FunctionS<'s>],
    impl_bounds: &'s [ImplBoundS<'s>],
  ) -> Self {
    assert!(
      !generic_params.iter().any(|x| matches!(x.rune.rune, IRuneS::DenizenDefaultRegionRune(_))),
      "vassert: generic_params should not contain DenizenDefaultRegionRuneS"
    );
    Self {
      range,
      name,
      attributes,
      weakable,
      generic_params,
      sharedness,
      tyype,
      header_rules,
      member_rules,
      members,
      internal_methods,
      impl_bounds,
      _sealed: (),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IStructMemberS<'s> {
  NormalStructMember(NormalStructMemberS<'s>),
  VariadicStructMember(VariadicStructMemberS<'s>),
}

impl<'s> IStructMemberS<'s> {
  pub fn range(&self) -> RangeS<'_> {
    match self {
      IStructMemberS::NormalStructMember(m) => m.range.clone(),
      IStructMemberS::VariadicStructMember(m) => m.range.clone(),
    }
  }

  pub fn type_rune(&self) -> &RuneUsage<'s> {
    match self {
      IStructMemberS::NormalStructMember(m) => &m.type_rune,
      IStructMemberS::VariadicStructMember(m) => &m.type_rune,
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NormalStructMemberS<'s> {
  pub range: RangeS<'s>,
  pub name: StrI<'s>,
  pub type_rune: RuneUsage<'s>, // VCOORD: remove this in favor of the ITypeST
  pub tyype: ITypeST<'s>,
  // Per @PFVSZ, the member's type split into its outer ref wraps and the value they enclose, so a
  // constructor param built from this member is byte-identical to a user-written param.
  pub value_type_rune: RuneUsage<'s>,
  pub type_outer_ref_rules: &'s [IRulexSR<'s>],
  pub value_type_rules: &'s [IRulexSR<'s>],
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct VariadicStructMemberS<'s> {
  pub range: RangeS<'s>,
  pub type_rune: RuneUsage<'s>, // VCOORD: remove this in favor of the ITypeST
  pub tyype: ITypeST<'s>,
}

#[derive(Debug, PartialEq)]
pub struct InterfaceS<'s> {
  pub range: RangeS<'s>,
  pub name: &'s TopLevelInterfaceDeclarationNameS<'s>,
  pub attributes: &'s [ICitizenAttributeS<'s>],
  pub weakable: bool,
  pub generic_params: &'s [&'s GenericParameterS<'s>],
  pub sharedness: SharednessP,
  pub tyype: TemplateTemplataType<'s>,
  pub rules: &'s [IRulexSR<'s>],
  pub internal_methods: &'s [&'s FunctionS<'s>],
  /// `where implements(Sub, Super)` clauses; see ImplBoundS.
  pub impl_bounds: &'s [ImplBoundS<'s>],
  _sealed: (),
}
impl<'s> InterfaceS<'s> {
  pub fn new(
    range: RangeS<'s>,
    name: &'s TopLevelInterfaceDeclarationNameS<'s>,
    attributes: &'s [ICitizenAttributeS<'s>],
    weakable: bool,
    generic_params: &'s [&'s GenericParameterS<'s>],
    sharedness: SharednessP,
    tyype: TemplateTemplataType<'s>,
    rules: &'s [IRulexSR<'s>],
    internal_methods: &'s [&'s FunctionS<'s>],
    impl_bounds: &'s [ImplBoundS<'s>],
  ) -> Self {
    assert!(
      !generic_params.iter().any(|x| matches!(x.rune.rune, IRuneS::DenizenDefaultRegionRune(_))),
      "vassert: generic_params should not contain DenizenDefaultRegionRuneS"
    );
    for internal_method in internal_methods {
      assert!(
        generic_params == internal_method.generic_params,
        "vassert: genericParams == internalMethod.genericParams"
      );
    }
    Self {
      range,
      name,
      attributes,
      weakable,
      generic_params,
      sharedness,
      tyype,
      rules,
      internal_methods,
      impl_bounds,
      _sealed: (),
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct ImplS<'s> {
  pub range: RangeS<'s>,
  pub name: IImplDeclarationNameS<'s>,
  pub user_specified_identifying_runes: &'s [&'s GenericParameterS<'s>],
  pub rules: &'s [IRulexSR<'s>],
  pub tyype: ITemplataType<'s>,
  pub struct_kind_rune: RuneUsage<'s>, // VCOORD: remove this in favor of the ITypeST
  pub sub_citizen_imprecise_name: IImpreciseNameS<'s>,
  pub sub_citizen_type: ITypeST<'s>,
  pub interface_kind_rune: RuneUsage<'s>, // VCOORD: remove this in favor of the ITypeST
  pub super_interface_imprecise_name: IImpreciseNameS<'s>,
  pub super_interface_type: ITypeST<'s>,
  /// `where implements(Sub, Super)` clauses; see ImplBoundS.
  pub impl_bounds: &'s [ImplBoundS<'s>],
  _sealed: (),
}

impl<'s> ImplS<'s> {
  pub fn new(
    range: RangeS<'s>,
    name: IImplDeclarationNameS<'s>,
    user_specified_identifying_runes: &'s [&'s GenericParameterS<'s>],
    rules: &'s [IRulexSR<'s>],
    tyype: ITemplataType<'s>,
    struct_kind_rune: RuneUsage<'s>,
    sub_citizen_imprecise_name: IImpreciseNameS<'s>,
    sub_citizen_type: ITypeST<'s>,
    interface_kind_rune: RuneUsage<'s>,
    super_interface_imprecise_name: IImpreciseNameS<'s>,
    super_interface_type: ITypeST<'s>,
    impl_bounds: &'s [ImplBoundS<'s>],
  ) -> Self {
    Self {
      range,
      name,
      user_specified_identifying_runes,
      rules,
      tyype,
      struct_kind_rune,
      sub_citizen_imprecise_name,
      sub_citizen_type,
      interface_kind_rune,
      super_interface_imprecise_name,
      super_interface_type,
      impl_bounds,
      _sealed: (),
    }
  }
}

#[derive(Debug, PartialEq)]
pub struct ExportAsS<'s> {
  pub range: RangeS<'s>,
  pub rules: &'s [IRulexSR<'s>],
  pub export_name: ExportAsNameS<'s>,
  pub rune: RuneUsage<'s>, // VCOORD: remove this in favor of the ITypeST
  pub exported_name: StrI<'s>,
  pub tyype: ITypeST<'s>,
}

#[derive(Debug, PartialEq)]
pub struct ImportS<'s> {
  pub range: RangeS<'s>,
  pub module_name: StrI<'s>,
  pub package_names: &'s [StrI<'s>],
  pub importee_name: StrI<'s>,
}

pub fn interface_s_name<'s>(interface_s: &InterfaceS<'s>) -> TopLevelCitizenDeclarationNameS<'s> {
  TopLevelCitizenDeclarationNameS::from(interface_s.name)
}

pub fn struct_s_name<'s>(struct_s: &StructS<'s>) -> TopLevelCitizenDeclarationNameS<'s> {
  TopLevelCitizenDeclarationNameS::from(struct_s.name.expect_top_level())
}

#[derive(Debug, PartialEq)]
pub struct ParameterS<'s> {
  pub range: RangeS<'s>,
  pub virtuality: Option<AbstractSP<'s>>,
  pub pre_checked: bool,
  /// The parameter's name:
  /// - a user-written param keeps its real name (`p` -> CodeVarName, `&self` -> CodeVarName(self)).
  /// - an anonymous or ignored param (`Pair[a, b]`, `_ Pair`) gets a synthetic DesugaredParamName.
  /// A destructure's inner names live on a body-head LetSE that loads this name, synthesized only
  /// when the param actually destructures.
  pub name: IVarNameS<'s>,
  // Per @PFVSZ, the parameter's type is split into its outer ref wraps and the value they enclose.
  /// Rune for the full type: the outer wraps plus the value type they enclose. Equal to
  /// value_type_rune when type_outer_ref_rules is empty (the param has no outer wraps).
  pub full_type_rune: RuneUsage<'s>,
  /// Rune for the value type: the named-type root, past the outer wraps.
  pub value_type_rune: RuneUsage<'s>,
  /// The outer &/weak wraps that build the full type. Only BorrowRefSR /
  /// WeakRefSR variants live here; they chain from
  /// full_type_rune down to value_type_rune.
  pub type_outer_ref_rules: &'s [IRulexSR<'s>],
  /// Rules that build the value type (Lookup/Call/etc., possibly with nested BorrowRefs
  /// inside template args).
  pub value_type_rules: &'s [IRulexSR<'s>],
  _sealed: (),
}
impl<'s> ParameterS<'s> {
  pub fn new(
    range: RangeS<'s>,
    virtuality: Option<AbstractSP<'s>>,
    pre_checked: bool,
    name: IVarNameS<'s>,
    full_type_rune: RuneUsage<'s>,
    value_type_rune: RuneUsage<'s>,
    type_outer_ref_rules: &'s [IRulexSR<'s>],
    value_type_rules: &'s [IRulexSR<'s>],
  ) -> Self {
    debug_assert!(
      matches!(
        name,
        IVarNameS::CodeVarName(_)
          | IVarNameS::ConstructingMemberName(_)
          | IVarNameS::ClosureParamName(_)
          | IVarNameS::MagicParamName(_)
          | IVarNameS::DesugaredParamName(_)
      ),
      "ParameterS.name must be a param name (real, or synthetic DesugaredParamName)"
    );
    debug_assert!(
      type_outer_ref_rules
        .iter()
        .all(|r| matches!(r, IRulexSR::BorrowRef(_) | IRulexSR::WeakRef(_) | IRulexSR::OwnRef(_))),
      "type_outer_ref_rules may only contain onion ref wraps"
    );
    debug_assert!(
      !type_outer_ref_rules.is_empty() || full_type_rune.rune == value_type_rune.rune,
      "full_type_rune must equal value_type_rune when type_outer_ref_rules is empty"
    );
    Self {
      range,
      virtuality,
      pre_checked,
      name,
      full_type_rune,
      value_type_rune,
      type_outer_ref_rules,
      value_type_rules,
      _sealed: (),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AbstractSP<'s> {
  pub range: RangeS<'s>,
  pub is_internal_method: bool,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IBodyS<'s> {
  ExternBody(ExternBodyS),
  AbstractBody(AbstractBodyS),
  GeneratedBody(GeneratedBodyS<'s>),
  CodeBody(CodeBodyS<'s>),
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ExternBodyS {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AbstractBodyS {}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GeneratedBodyS<'s> {
  pub generator_id: StrI<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CodeBodyS<'s> {
  pub body: &'s BodySE<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IGenericParameterTypeS<'s> {
  RegionGenericParameterType(RegionGenericParameterTypeS),
  KindGenericParameterType(KindGenericParameterTypeS),
  OtherGenericParameterType(OtherGenericParameterTypeS<'s>),
}

impl<'s> IGenericParameterTypeS<'s> {
  pub fn expect_region(&self) -> &RegionGenericParameterTypeS {
    match self {
      IGenericParameterTypeS::RegionGenericParameterType(x) => x,
      _ => panic!("Expected region generic parameter type"),
    }
  }

  pub fn tyype(&self) -> ITemplataType<'s> {
    match self {
      IGenericParameterTypeS::RegionGenericParameterType(x) => x.tyype(),
      IGenericParameterTypeS::KindGenericParameterType(x) => x.tyype(),
      IGenericParameterTypeS::OtherGenericParameterType(x) => x.tyype.clone(),
    }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RegionGenericParameterTypeS {}

impl RegionGenericParameterTypeS {
  pub fn tyype<'a>(&self) -> ITemplataType<'a> {
    ITemplataType::RegionTemplataType(RegionTemplataType {})
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct KindGenericParameterTypeS {}

impl KindGenericParameterTypeS {
  pub fn tyype<'a>(&self) -> ITemplataType<'a> {
    ITemplataType::KindTemplataType(KindTemplataType {})
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OtherGenericParameterTypeS<'s> {
  pub tyype: ITemplataType<'s>,
  _sealed: (),
}
impl<'s> OtherGenericParameterTypeS<'s> {
  pub fn new(tyype: ITemplataType<'s>) -> Self {
    assert!(
      !matches!(tyype, ITemplataType::RegionTemplataType(_) | ITemplataType::KindTemplataType(_)),
      "vwat: Use RegionGenericParameterTypeS or KindGenericParameterTypeS for these types"
    );
    Self { tyype, _sealed: () }
  }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GenericParameterS<'s> {
  pub range: RangeS<'s>,
  pub rune: RuneUsage<'s>,
  pub tyype: IGenericParameterTypeS<'s>,
  pub default: Option<GenericParameterDefaultS<'s>>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GenericParameterDefaultS<'s> {
  pub result_rune: IRuneS<'s>,
  pub rules: &'s [&'s IRulexSR<'s>], // VCOORD: remove this in favor of the ITypeST
  pub tyype: ITypeST<'s>,
}

#[derive(Debug, PartialEq)]
pub struct FunctionS<'s> {
  pub range: RangeS<'s>,
  pub name: IFunctionDeclarationNameS<'s>,
  pub attributes: &'s [IFunctionAttributeS<'s>],
  pub generic_params: &'s [&'s GenericParameterS<'s>],
  pub tyype: TemplateTemplataType<'s>,
  pub params: &'s [ParameterS<'s>],
  pub maybe_ret_kind_rune: Option<RuneUsage<'s>>,
  // Called header rules because it doesn't include any of the rules from the parameters, those are
  // in ParameterS.
  pub header_rules: &'s [IRulexSR<'s>],
  /// `where implements(Sub, Super)` clauses. Kept out of `rules` because they are checked after
  /// the solve rather than solved; see ImplBoundS.
  pub impl_bounds: &'s [ImplBoundS<'s>],
  pub body: &'s IBodyS<'s>,
  _sealed: (),
}
impl<'s> FunctionS<'s> {
  pub fn new(
    range: RangeS<'s>,
    name: IFunctionDeclarationNameS<'s>,
    attributes: &'s [IFunctionAttributeS<'s>],
    generic_params: &'s [&'s GenericParameterS<'s>],
    tyype: TemplateTemplataType<'s>,
    params: &'s [ParameterS<'s>],
    maybe_ret_kind_rune: Option<RuneUsage<'s>>,
    rules: &'s [IRulexSR<'s>],
    impl_bounds: &'s [ImplBoundS<'s>],
    body: &'s IBodyS<'s>,
  ) -> Self {
    assert!(
      !generic_params.iter().any(|x| matches!(x.rune.rune, IRuneS::DenizenDefaultRegionRune(_))),
      "vassert: generic_params should not contain DenizenDefaultRegionRuneS"
    );
    match body {
      IBodyS::ExternBody(_) | IBodyS::AbstractBody(_) | IBodyS::GeneratedBody(_) => {
        assert!(
          !matches!(name, IFunctionDeclarationNameS::LambdaDeclarationName(_)),
          "vwat: extern/abstract/generated body must not be lambda"
        );
      }
      IBodyS::CodeBody(code_body) => {
        if !code_body.body.closured_names.is_empty() {
          assert!(
            matches!(name, IFunctionDeclarationNameS::LambdaDeclarationName(_)),
            "vwat: closured code body must be lambda"
          );
        }
      }
    }
    Self {
      range,
      name,
      attributes,
      generic_params,
      tyype,
      params,
      maybe_ret_kind_rune,
      header_rules: rules,
      impl_bounds,
      body,
      _sealed: (),
    }
  }

  pub fn is_light(&self) -> bool {
    match &self.body {
      IBodyS::ExternBody(_) | IBodyS::AbstractBody(_) | IBodyS::GeneratedBody(_) => true,
      IBodyS::CodeBody(body) => body.body.closured_names.is_empty(),
    }
  }

  pub fn is_lambda(&self) -> bool {
    matches!(self.name, IFunctionDeclarationNameS::LambdaDeclarationName(_))
  }
}

#[derive(Debug, PartialEq)]
pub struct LocationInDenizenBuilder {
  path: Vec<i32>,
  consumed: bool,
  next_child: i32,
}

impl LocationInDenizenBuilder {
  // MIGALLOW: new -> new
  pub fn new(path: Vec<i32>) -> Self {
    Self { path, consumed: false, next_child: 1 }
  }

  pub fn child(&mut self) -> LocationInDenizenBuilder {
    let child = self.next_child;
    self.next_child += 1;
    let mut child_path = self.path.clone();
    child_path.push(child);
    LocationInDenizenBuilder::new(child_path)
  }

  // Per @DSAUIMZ, this is for NON-interned uses only (expression AST nodes).
  pub fn consume_in<'x>(&mut self, arena: &'x bumpalo::Bump) -> LocationInDenizen<'x> {
    assert!(
      !self.consumed,
      "Location in denizen was already used for something, add a .child() somewhere."
    );
    self.consumed = true;
    LocationInDenizen { path: arena.alloc_slice_copy(&self.path) }
  }

  // Per @DSAUIMZ, this is for NON-interned uses only (expression AST nodes).
  // Takes a ScoutArena instead of raw Bump to avoid exposing the allocator.
  // V: this feels weird. theres nothing guaranteeing that this LocationInDenizen will actually land anywhere,
  // in which case we're just leaking those allocations. i think we need a LocationInDenizenVal.
  // maybe LocationInDenizenVal can even be a stack-based linked list.
  pub fn consume_in_arena<'x>(&mut self, arena: &ScoutArena<'x>) -> LocationInDenizen<'x> {
    assert!(
      !self.consumed,
      "Location in denizen was already used for something, add a .child() somewhere."
    );
    self.consumed = true;
    LocationInDenizen { path: arena.alloc_slice_copy(&self.path) }
  }

  // Per @DSAUIMZ, this is the only way to construct a LocationInDenizenVal.
  // Borrows from the builder's Vec, so 'tmp is a stack lifetime, not 's.
  pub fn borrow_val(&mut self) -> LocationInDenizenVal<'_> {
    assert!(
      !self.consumed,
      "Location in denizen was already used for something, add a .child() somewhere."
    );
    self.consumed = true;
    LocationInDenizenVal { path: &self.path }
  }
}

/// A path identifying a specific location within a denizen (function, struct, etc.).
/// Each element in the path is a child index, forming a tree address.
///
/// Parameterized on lifetime `'x` because LocationInDenizen lives in different
/// arenas depending on its owner:
/// - When inside rune structs (e.g. ImplicitRuneS), it's interned into the
///   `'s` interner arena, so `'x = 's`.
/// - When inside expression structs (e.g. FunctionSE), it's allocated
///   in the `'s` scout arena, so `'x = 's`.
///
/// The path is an arena-allocated slice rather than a Vec so that the entire
/// struct can live in an arena without heap pointers.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocationInDenizen<'x> {
  pub path: &'x [i32],
}

/// Borrowed view of a LocationInDenizen path, for use as an intern lookup key.
/// Per @DSAUIMZ, fields are private to prevent pre-allocation.
/// Only constructible via LocationInDenizenBuilder::borrow_val().
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct LocationInDenizenVal<'tmp> {
  path: &'tmp [i32],
}

impl<'tmp> LocationInDenizenVal<'tmp> {
  /// Read access to path contents (for Hash/Eq/Debug implementations).
  pub fn path(&self) -> &[i32] {
    self.path
  }

  /// Per @DSAUIMZ, only called inside intern methods on a miss.
  pub(crate) fn promote_in<'s>(&self, arena: &'s bumpalo::Bump) -> LocationInDenizen<'s> {
    LocationInDenizen { path: arena.alloc_slice_copy(self.path) }
  }

  /// Per @DSAUIMZ, only used inside intern methods to construct stored HashMap keys
  /// from a just-promoted LocationInDenizen.
  pub(crate) fn from_canonical<'s>(lid: &LocationInDenizen<'s>) -> LocationInDenizenVal<'s> {
    LocationInDenizenVal { path: lid.path }
  }
}

impl<'x> LocationInDenizen<'x> {
  pub fn before(&self, that: &LocationInDenizen) -> bool {
    for (this_step, that_step) in self.path.iter().zip(that.path.iter()) {
      if this_step < that_step {
        return true;
      }
      if this_step > that_step {
        return false;
      }
    }
    if self.path.len() < that.path.len() {
      return true;
    }
    if self.path.len() > that.path.len() {
      return false;
    }
    false
  }
}

#[derive(Debug, PartialEq)]
pub enum IDenizenS<'s> {
  TopLevelFunction(TopLevelFunctionS<'s>),
  TopLevelImpl(TopLevelImplS<'s>),
  TopLevelExportAs(TopLevelExportAsS<'s>),
  TopLevelImport(TopLevelImportS<'s>),
  TopLevelStruct(TopLevelStructS<'s>),
  TopLevelInterface(TopLevelInterfaceS<'s>),
}

#[derive(Debug, PartialEq)]
pub struct TopLevelFunctionS<'s> {
  pub function: FunctionS<'s>,
}

#[derive(Debug, PartialEq)]
pub struct TopLevelImplS<'s> {
  pub impl_: ImplS<'s>,
}

#[derive(Debug, PartialEq)]
pub struct TopLevelExportAsS<'s> {
  pub export: ExportAsS<'s>,
}

#[derive(Debug, PartialEq)]
pub struct TopLevelImportS<'s> {
  pub imporrt: ImportS<'s>,
}

#[derive(Debug, PartialEq)]
pub enum ICitizenDenizenS<'s> {
  TopLevelStruct(TopLevelStructS<'s>),
  TopLevelInterface(TopLevelInterfaceS<'s>),
}

impl<'s> ICitizenDenizenS<'s> {
  pub fn citizen(&self) -> ! {
    panic!("ICitizenDenizenS::citizen is dead code")
  }
}

// MIGALLOW: unapply -> as_citizen_denizen
pub fn as_citizen_denizen<'s>(_x: &IDenizenS<'s>) -> Option<ICitizenDenizenS<'s>> {
  panic!("as_citizen_denizen is dead code")
}

#[derive(Debug, PartialEq)]
pub struct TopLevelStructS<'s> {
  pub strukt: StructS<'s>,
}

#[derive(Debug, PartialEq)]
pub struct TopLevelInterfaceS<'s> {
  pub interface: InterfaceS<'s>,
}

#[derive(Debug, PartialEq)]
pub struct FileS<'s> {
  pub denizens: Vec<IDenizenS<'s>>,
}
