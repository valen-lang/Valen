use crate::interner::StrI;
use crate::lexing::RangeL;
use std::sync::Arc;

// ============================================================================
// Core types
// ============================================================================

/// Unit marker (something that exists in source)
#[derive(Clone, Debug, PartialEq)]
pub struct UnitP {
    pub range: RangeL,
}

/// Name in source code
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct NameP {
    pub range: RangeL,
    pub str: Arc<StrI>,
}

/// File coordinate (package + file path)
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FileCoordinate {
    pub package_coord: PackageCoordinate,
    pub filepath: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PackageCoordinate {
    pub module: Arc<StrI>,
    pub packages: Vec<Arc<StrI>>,
}

/// Parsed file
#[derive(Clone, Debug, PartialEq)]
pub struct FileP {
    pub file_coord: FileCoordinate,
    pub comments_ranges: Vec<RangeL>,
    pub denizens: Vec<IDenizenP>,
}

// ============================================================================
// Top-level items
// ============================================================================

#[derive(Clone, Debug, PartialEq)]
pub enum IDenizenP {
    TopLevelFunction(FunctionP),
    TopLevelStruct(StructP),
    TopLevelInterface(InterfaceP),
    TopLevelImpl(ImplP),
    TopLevelExportAs(ExportAsP),
    TopLevelImport(ImportP),
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImplP {
    pub range: RangeL,
    pub generic_params: Option<GenericParametersP>,
    pub template_rules: Option<TemplateRulesP>,
    pub struct_: Option<ITemplexPT>,
    pub interface: ITemplexPT,
    pub attributes: Vec<IAttributeP>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExportAsP {
    pub range: RangeL,
    pub struct_: ITemplexPT,
    pub exported_name: NameP,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImportP {
    pub range: RangeL,
    pub module_name: NameP,
    pub package_steps: Vec<NameP>,
    pub importee_name: NameP,
}

// ============================================================================
// Attributes
// ============================================================================

#[derive(Clone, Debug, PartialEq)]
pub enum IAttributeP {
    WeakableAttribute(RangeL),
    SealedAttribute(RangeL),
    MacroCall { range: RangeL, inclusion: IMacroInclusionP, name: NameP },
    AbstractAttribute(RangeL),
    ExternAttribute(RangeL),
    BuiltinAttribute { range: RangeL, generator_name: NameP },
    ExportAttribute(RangeL),
    PureAttribute(RangeL),
    AdditiveAttribute(RangeL),
    LinearAttribute(RangeL),
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IMacroInclusionP {
    CallMacro,
    DontCallMacro,
}

// ============================================================================
// Rune attributes
// ============================================================================

#[derive(Clone, Debug, PartialEq)]
pub enum IRuneAttributeP {
    ImmutableRuneAttribute(RangeL),
    MutableRuneAttribute(RangeL),
    ReadOnlyRegionRuneAttribute(RangeL),
    ReadWriteRegionRuneAttribute(RangeL),
    ImmutableRegionRuneAttribute(RangeL),
    AdditiveRegionRuneAttribute(RangeL),
    PoolRuneAttribute(RangeL),
    ArenaRuneAttribute(RangeL),
    BumpRuneAttribute(RangeL),
}

impl IRuneAttributeP {
    pub fn range(&self) -> RangeL {
        match self {
            IRuneAttributeP::ImmutableRuneAttribute(r) => *r,
            IRuneAttributeP::MutableRuneAttribute(r) => *r,
            IRuneAttributeP::ReadOnlyRegionRuneAttribute(r) => *r,
            IRuneAttributeP::ReadWriteRegionRuneAttribute(r) => *r,
            IRuneAttributeP::ImmutableRegionRuneAttribute(r) => *r,
            IRuneAttributeP::AdditiveRegionRuneAttribute(r) => *r,
            IRuneAttributeP::PoolRuneAttribute(r) => *r,
            IRuneAttributeP::ArenaRuneAttribute(r) => *r,
            IRuneAttributeP::BumpRuneAttribute(r) => *r,
        }
    }
}

// ============================================================================
// Struct
// ============================================================================

#[derive(Clone, Debug, PartialEq)]
pub struct StructP {
    pub range: RangeL,
    pub name: NameP,
    pub attributes: Vec<IAttributeP>,
    pub mutability: Option<ITemplexPT>,
    pub identifying_runes: Option<GenericParametersP>,
    pub template_rules: Option<TemplateRulesP>,
    pub maybe_default_region_rune: Option<RegionRunePT>,
    pub body_range: RangeL,
    pub members: StructMembersP,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StructMembersP {
    pub range: RangeL,
    pub contents: Vec<IStructContent>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum IStructContent {
    StructMethod(FunctionP),
    NormalStructMember {
        range: RangeL,
        name: NameP,
        variability: VariabilityP,
        tyype: ITemplexPT,
    },
    VariadicStructMember {
        range: RangeL,
        variability: VariabilityP,
        tyype: ITemplexPT,
    },
}

// ============================================================================
// Interface
// ============================================================================

#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceP {
    pub range: RangeL,
    pub name: NameP,
    pub attributes: Vec<IAttributeP>,
    pub mutability: Option<ITemplexPT>,
    pub maybe_identifying_runes: Option<GenericParametersP>,
    pub template_rules: Option<TemplateRulesP>,
    pub maybe_default_region_rune: Option<RegionRunePT>,
    pub body_range: RangeL,
    pub members: Vec<FunctionP>,
}

// ============================================================================
// Function
// ============================================================================

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionP {
    pub range: RangeL,
    pub header: FunctionHeaderP,
    pub body: Option<Box<BlockPE>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionHeaderP {
    pub range: RangeL,
    pub name: Option<NameP>,
    pub attributes: Vec<IAttributeP>,
    pub generic_parameters: Option<GenericParametersP>,
    pub template_rules: Option<TemplateRulesP>,
    pub params: Option<ParamsP>,
    pub ret: FunctionReturnP,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionReturnP {
    pub range: RangeL,
    pub ret_type: Option<ITemplexPT>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenericParameterP {
    pub range: RangeL,
    pub name: NameP,
    pub maybe_type: Option<GenericParameterTypeP>,
    pub coord_region: Option<RegionRunePT>,
    pub attributes: Vec<IRuneAttributeP>,
    pub maybe_default: Option<ITemplexPT>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenericParameterTypeP {
    pub range: RangeL,
    pub tyype: ITypePR,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenericParametersP {
    pub range: RangeL,
    pub params: Vec<GenericParameterP>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TemplateRulesP {
    pub range: RangeL,
    pub rules: Vec<IRulexPR>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParamsP {
    pub range: RangeL,
    pub params: Vec<ParameterP>,
}

// ============================================================================
// Primitives
// ============================================================================

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum MutabilityP {
    Mutable,
    Immutable,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum VariabilityP {
    Final,
    Varying,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OwnershipP {
    Own,
    Borrow,
    Live,
    Weak,
    Share,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LoadAsP {
    Move,
    LoadAsBorrow,
    LoadAsWeak,
    Use,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LocationP {
    Inline,
    Yonder,
}

// ============================================================================
// Patterns
// ============================================================================

#[derive(Clone, Debug, PartialEq)]
pub struct AbstractP {
    pub range: RangeL,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParameterP {
    pub range: RangeL,
    pub virtuality: Option<AbstractP>,
    pub maybe_pre_checked: Option<RangeL>,
    pub self_borrow: Option<RangeL>,
    pub pattern: Option<PatternPP>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DestinationLocalP {
    pub decl: INameDeclarationP,
    pub mutate: Option<RangeL>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PatternPP {
    pub range: RangeL,
    pub destination: Option<DestinationLocalP>,
    pub templex: Option<ITemplexPT>,
    pub destructure: Option<DestructureP>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DestructureP {
    pub range: RangeL,
    pub patterns: Vec<PatternPP>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum INameDeclarationP {
    LocalNameDeclaration(NameP),
    IgnoredLocalNameDeclaration(RangeL),
    IterableNameDeclaration(RangeL),
    IteratorNameDeclaration(RangeL),
    IterationOptionNameDeclaration(RangeL),
    ConstructingMemberNameDeclaration(NameP),
}

impl INameDeclarationP {
    pub fn range(&self) -> RangeL {
        match self {
            INameDeclarationP::LocalNameDeclaration(n) => n.range,
            INameDeclarationP::IgnoredLocalNameDeclaration(r) => *r,
            INameDeclarationP::IterableNameDeclaration(r) => *r,
            INameDeclarationP::IteratorNameDeclaration(r) => *r,
            INameDeclarationP::IterationOptionNameDeclaration(r) => *r,
            INameDeclarationP::ConstructingMemberNameDeclaration(n) => n.range,
        }
    }
}

// ============================================================================
// Templex (type expressions)
// ============================================================================

#[derive(Clone, Debug, PartialEq)]
pub enum ITemplexPT {
    AnonymousRune(RangeL),
    Bool { range: RangeL, value: bool },
    Point { range: RangeL, inner: Box<ITemplexPT> },
    Call { range: RangeL, template: Box<ITemplexPT>, args: Vec<ITemplexPT> },
    Function { range: RangeL, mutability: Option<Box<ITemplexPT>>, parameters: Box<PackPT>, return_type: Box<ITemplexPT> },
    Inline { range: RangeL, inner: Box<ITemplexPT> },
    Int { range: RangeL, value: i64 },
    RegionRune(RegionRunePT),
    Location { range: RangeL, location: LocationP },
    Tuple { range: RangeL, elements: Vec<ITemplexPT> },
    Mutability { range: RangeL, mutability: MutabilityP },
    NameOrRune(NameP),
    Interpreted { range: RangeL, maybe_ownership: Option<Box<OwnershipPT>>, maybe_region: Option<Box<RegionRunePT>>, inner: Box<ITemplexPT> },
    Ownership { range: RangeL, ownership: OwnershipP },
    Pack(PackPT),
    Func { range: RangeL, name: NameP, params_range: RangeL, parameters: Vec<ITemplexPT>, return_type: Box<ITemplexPT> },
    StaticSizedArray { range: RangeL, mutability: Box<ITemplexPT>, variability: Box<ITemplexPT>, size: Box<ITemplexPT>, element: Box<ITemplexPT> },
    RuntimeSizedArray { range: RangeL, mutability: Box<ITemplexPT>, element: Box<ITemplexPT> },
    Share { range: RangeL, inner: Box<ITemplexPT> },
    String { range: RangeL, str: String },
    TypedRune { range: RangeL, rune: NameP, tyype: ITypePR },
    Variability { range: RangeL, variability: VariabilityP },
}

impl ITemplexPT {
    pub fn range(&self) -> RangeL {
        match self {
            ITemplexPT::AnonymousRune(r) => *r,
            ITemplexPT::Bool { range, .. } => *range,
            ITemplexPT::Point { range, .. } => *range,
            ITemplexPT::Call { range, .. } => *range,
            ITemplexPT::Function { range, .. } => *range,
            ITemplexPT::Inline { range, .. } => *range,
            ITemplexPT::Int { range, .. } => *range,
            ITemplexPT::RegionRune(r) => r.range,
            ITemplexPT::Location { range, .. } => *range,
            ITemplexPT::Tuple { range, .. } => *range,
            ITemplexPT::Mutability { range, .. } => *range,
            ITemplexPT::NameOrRune(n) => n.range,
            ITemplexPT::Interpreted { range, .. } => *range,
            ITemplexPT::Ownership { range, .. } => *range,
            ITemplexPT::Pack(p) => p.range,
            ITemplexPT::Func { range, .. } => *range,
            ITemplexPT::StaticSizedArray { range, .. } => *range,
            ITemplexPT::RuntimeSizedArray { range, .. } => *range,
            ITemplexPT::Share { range, .. } => *range,
            ITemplexPT::String { range, .. } => *range,
            ITemplexPT::TypedRune { range, .. } => *range,
            ITemplexPT::Variability { range, .. } => *range,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct RegionRunePT {
    pub range: RangeL,
    pub name: Option<NameP>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OwnershipPT {
    pub range: RangeL,
    pub ownership: OwnershipP,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PackPT {
    pub range: RangeL,
    pub members: Vec<ITemplexPT>,
}

// ============================================================================
// Rules
// ============================================================================

#[derive(Clone, Debug, PartialEq)]
pub enum IRulexPR {
    Equals { range: RangeL, left: Box<IRulexPR>, right: Box<IRulexPR> },
    Or { range: RangeL, possibilities: Vec<IRulexPR> },
    Dot { range: RangeL, container: Box<IRulexPR>, member_name: NameP },
    Components { range: RangeL, container: ITypePR, components: Vec<IRulexPR> },
    Typed { range: RangeL, rune: Option<NameP>, tyype: ITypePR },
    Templex(ITemplexPT),
    BuiltinCall { range: RangeL, name: NameP, args: Vec<IRulexPR> },
    Pack { range: RangeL, elements: Vec<IRulexPR> },
}

impl IRulexPR {
    pub fn range(&self) -> RangeL {
        match self {
            IRulexPR::Equals { range, .. } => *range,
            IRulexPR::Or { range, .. } => *range,
            IRulexPR::Dot { range, .. } => *range,
            IRulexPR::Components { range, .. } => *range,
            IRulexPR::Typed { range, .. } => *range,
            IRulexPR::Templex(t) => t.range(),
            IRulexPR::BuiltinCall { range, .. } => *range,
            IRulexPR::Pack { range, .. } => *range,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ITypePR {
    IntType,
    BoolType,
    OwnershipType,
    MutabilityType,
    VariabilityType,
    LocationType,
    CoordType,
    CoordListType,
    PrototypeType,
    KindType,
    RegionType,
    CitizenTemplateType,
}

// ============================================================================
// Expressions
// ============================================================================

/// Expression enum - idiomatic Rust replacement for Scala's trait hierarchy
#[derive(Clone, Debug, PartialEq)]
pub enum IExpressionPE {
    Void(VoidPE),
    Pack(PackPE),
    SubExpression(SubExpressionPE),
    And(AndPE),
    Or(OrPE),
    If(IfPE),
    While(WhilePE),
    Each(EachPE),
    Range(RangePE),
    Destruct(DestructPE),
    Unlet(UnletPE),
    Mutate(MutatePE),
    Return(ReturnPE),
    Break(BreakPE),
    Let(LetPE),
    Tuple(TuplePE),
    ConstructArray(ConstructArrayPE),
    ConstantInt(ConstantIntPE),
    ConstantBool(ConstantBoolPE),
    ConstantStr(ConstantStrPE),
    ConstantFloat(ConstantFloatPE),
    StrInterpolate(StrInterpolatePE),
    Dot(DotPE),
    Index(IndexPE),
    FunctionCall(FunctionCallPE),
    BraceCall(BraceCallPE),
    Not(NotPE),
    Augment(AugmentPE),
    Transmigrate(TransmigratePE),
    BinaryCall(BinaryCallPE),
    MethodCall(MethodCallPE),
    Lookup(LookupPE),
    MagicParamLookup(MagicParamLookupPE),
    Lambda(LambdaPE),
    Block(BlockPE),
    Consecutor(ConsecutorPE),
    Shortcall(ShortcallPE),
}

impl IExpressionPE {
    pub fn range(&self) -> RangeL {
        match self {
            IExpressionPE::Void(x) => x.range,
            IExpressionPE::Pack(x) => x.range,
            IExpressionPE::SubExpression(x) => x.range,
            IExpressionPE::And(x) => x.range,
            IExpressionPE::Or(x) => x.range,
            IExpressionPE::If(x) => x.range,
            IExpressionPE::While(x) => x.range,
            IExpressionPE::Each(x) => x.range,
            IExpressionPE::Range(x) => x.range,
            IExpressionPE::Destruct(x) => x.range,
            IExpressionPE::Unlet(x) => x.range,
            IExpressionPE::Mutate(x) => x.range,
            IExpressionPE::Return(x) => x.range,
            IExpressionPE::Break(x) => x.range,
            IExpressionPE::Let(x) => x.range,
            IExpressionPE::Tuple(x) => x.range,
            IExpressionPE::ConstructArray(x) => x.range,
            IExpressionPE::ConstantInt(x) => x.range,
            IExpressionPE::ConstantBool(x) => x.range,
            IExpressionPE::ConstantStr(x) => x.range,
            IExpressionPE::ConstantFloat(x) => x.range,
            IExpressionPE::StrInterpolate(x) => x.range,
            IExpressionPE::Dot(x) => x.range,
            IExpressionPE::Index(x) => x.range,
            IExpressionPE::FunctionCall(x) => x.range,
            IExpressionPE::BraceCall(x) => x.range,
            IExpressionPE::Not(x) => x.range,
            IExpressionPE::Augment(x) => x.range,
            IExpressionPE::Transmigrate(x) => x.range,
            IExpressionPE::BinaryCall(x) => x.range,
            IExpressionPE::MethodCall(x) => x.range,
            IExpressionPE::Lookup(x) => x.name.range(),
            IExpressionPE::MagicParamLookup(x) => x.range,
            IExpressionPE::Lambda(x) => x.function.range,
            IExpressionPE::Block(x) => x.range,
            IExpressionPE::Consecutor(x) => {
                assert!(!x.inners.is_empty());
                let begin = x.inners.first().unwrap().range().begin;
                let end = x.inners.last().unwrap().range().end;
                RangeL { begin, end }
            }
            IExpressionPE::Shortcall(x) => x.range,
        }
    }

    pub fn needs_semicolon_before_next_statement(&self) -> bool {
        match self {
            IExpressionPE::Void(_) => false,
            IExpressionPE::Pack(_) => true,
            IExpressionPE::SubExpression(_) => true,
            IExpressionPE::And(_) => true,
            IExpressionPE::Or(_) => true,
            IExpressionPE::If(_) => false,
            IExpressionPE::While(_) => false,
            IExpressionPE::Each(_) => false,
            IExpressionPE::Range(_) => true,
            IExpressionPE::Destruct(_) => true,
            IExpressionPE::Unlet(_) => true,
            IExpressionPE::Mutate(_) => true,
            IExpressionPE::Return(_) => true,
            IExpressionPE::Break(_) => true,
            IExpressionPE::Let(_) => true,
            IExpressionPE::Tuple(_) => true,
            IExpressionPE::ConstructArray(_) => true,
            IExpressionPE::ConstantInt(_) => true,
            IExpressionPE::ConstantBool(_) => true,
            IExpressionPE::ConstantStr(_) => true,
            IExpressionPE::ConstantFloat(_) => true,
            IExpressionPE::StrInterpolate(_) => true,
            IExpressionPE::Dot(_) => true,
            IExpressionPE::Index(_) => true,
            IExpressionPE::FunctionCall(_) => true,
            IExpressionPE::BraceCall(_) => true,
            IExpressionPE::Not(_) => true,
            IExpressionPE::Augment(_) => true,
            IExpressionPE::Transmigrate(_) => true,
            IExpressionPE::BinaryCall(_) => true,
            IExpressionPE::MethodCall(_) => true,
            IExpressionPE::Lookup(_) => true,
            IExpressionPE::MagicParamLookup(_) => true,
            IExpressionPE::Lambda(_) => true,
            IExpressionPE::Block(_) => false,
            IExpressionPE::Consecutor(x) => x.inners.last().unwrap().needs_semicolon_before_next_statement(),
            IExpressionPE::Shortcall(_) => true,
        }
    }

    pub fn produces_result(&self) -> bool {
        match self {
            IExpressionPE::Void(_) => false,
            IExpressionPE::Pack(_) => true,
            IExpressionPE::SubExpression(_) => true,
            IExpressionPE::And(_) => true,
            IExpressionPE::Or(_) => true,
            IExpressionPE::If(x) => x.then_body.inner.produces_result(),
            IExpressionPE::While(_) => false,
            IExpressionPE::Each(x) => x.body.inner.produces_result(),
            IExpressionPE::Range(_) => true,
            IExpressionPE::Destruct(_) => false,
            IExpressionPE::Unlet(_) => false,
            IExpressionPE::Mutate(_) => true,
            IExpressionPE::Return(_) => false,
            IExpressionPE::Break(_) => false,
            IExpressionPE::Let(_) => false,
            IExpressionPE::Tuple(_) => true,
            IExpressionPE::ConstructArray(_) => true,
            IExpressionPE::ConstantInt(_) => true,
            IExpressionPE::ConstantBool(_) => true,
            IExpressionPE::ConstantStr(_) => true,
            IExpressionPE::ConstantFloat(_) => true,
            IExpressionPE::StrInterpolate(_) => true,
            IExpressionPE::Dot(_) => true,
            IExpressionPE::Index(_) => true,
            IExpressionPE::FunctionCall(_) => true,
            IExpressionPE::BraceCall(_) => true,
            IExpressionPE::Not(_) => true,
            IExpressionPE::Augment(_) => true,
            IExpressionPE::Transmigrate(_) => true,
            IExpressionPE::BinaryCall(_) => true,
            IExpressionPE::MethodCall(_) => true,
            IExpressionPE::Lookup(_) => true,
            IExpressionPE::MagicParamLookup(_) => true,
            IExpressionPE::Lambda(_) => true,
            IExpressionPE::Block(x) => x.inner.produces_result(),
            IExpressionPE::Consecutor(x) => x.inners.last().unwrap().produces_result(),
            IExpressionPE::Shortcall(_) => true,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct VoidPE {
    pub range: RangeL,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PackPE {
    pub range: RangeL,
    pub inners: Vec<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct SubExpressionPE {
    pub range: RangeL,
    pub inner: Box<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AndPE {
    pub range: RangeL,
    pub left: Box<IExpressionPE>,
    pub right: Box<BlockPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct OrPE {
    pub range: RangeL,
    pub left: Box<IExpressionPE>,
    pub right: Box<BlockPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfPE {
    pub range: RangeL,
    pub condition: Box<IExpressionPE>,
    pub then_body: Box<BlockPE>,
    pub else_body: Box<BlockPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WhilePE {
    pub range: RangeL,
    pub condition: Box<IExpressionPE>,
    pub body: Box<BlockPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct EachPE {
    pub range: RangeL,
    pub maybe_pure: Option<RangeL>,
    pub entry_pattern: PatternPP,
    pub in_keyword_range: RangeL,
    pub iterable_expr: Box<IExpressionPE>,
    pub body: Box<BlockPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct RangePE {
    pub range: RangeL,
    pub from_expr: Box<IExpressionPE>,
    pub to_expr: Box<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DestructPE {
    pub range: RangeL,
    pub inner: Box<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UnletPE {
    pub range: RangeL,
    pub name: IImpreciseNameP,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MutatePE {
    pub range: RangeL,
    pub mutatee: Box<IExpressionPE>,
    pub source: Box<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ReturnPE {
    pub range: RangeL,
    pub expr: Box<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BreakPE {
    pub range: RangeL,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LetPE {
    pub range: RangeL,
    pub pattern: PatternPP,
    pub source: Box<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TuplePE {
    pub range: RangeL,
    pub elements: Vec<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum IArraySizeP {
    RuntimeSized,
    StaticSized { size_pt: Option<ITemplexPT> },
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstructArrayPE {
    pub range: RangeL,
    pub type_pt: Option<ITemplexPT>,
    pub mutability_pt: Option<ITemplexPT>,
    pub variability_pt: Option<ITemplexPT>,
    pub size: IArraySizeP,
    pub initializing_individual_elements: bool,
    pub args: Vec<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantIntPE {
    pub range: RangeL,
    pub value: i64,
    pub bits: Option<i64>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantBoolPE {
    pub range: RangeL,
    pub value: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantStrPE {
    pub range: RangeL,
    pub value: String,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConstantFloatPE {
    pub range: RangeL,
    pub value: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct StrInterpolatePE {
    pub range: RangeL,
    pub parts: Vec<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct DotPE {
    pub range: RangeL,
    pub left: Box<IExpressionPE>,
    pub operator_range: RangeL,
    pub member: NameP,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IndexPE {
    pub range: RangeL,
    pub left: Box<IExpressionPE>,
    pub args: Vec<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCallPE {
    pub range: RangeL,
    pub operator_range: RangeL,
    pub callable_expr: Box<IExpressionPE>,
    pub arg_exprs: Vec<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BraceCallPE {
    pub range: RangeL,
    pub operator_range: RangeL,
    pub subject_expr: Box<IExpressionPE>,
    pub arg_exprs: Vec<IExpressionPE>,
    pub callable_readwrite: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct NotPE {
    pub range: RangeL,
    pub inner: Box<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct AugmentPE {
    pub range: RangeL,
    pub target_ownership: OwnershipP,
    pub inner: Box<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TransmigratePE {
    pub range: RangeL,
    pub target_region: NameP,
    pub inner: Box<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BinaryCallPE {
    pub range: RangeL,
    pub function_name: NameP,
    pub left_expr: Box<IExpressionPE>,
    pub right_expr: Box<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MethodCallPE {
    pub range: RangeL,
    pub subject_expr: Box<IExpressionPE>,
    pub operator_range: RangeL,
    pub method_lookup: Box<LookupPE>,
    pub arg_exprs: Vec<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum IImpreciseNameP {
    LookupName(NameP),
    IterableName(RangeL),
    IteratorName(RangeL),
    IterationOptionName(RangeL),
}

impl IImpreciseNameP {
    pub fn range(&self) -> RangeL {
        match self {
            IImpreciseNameP::LookupName(n) => n.range,
            IImpreciseNameP::IterableName(r) => *r,
            IImpreciseNameP::IteratorName(r) => *r,
            IImpreciseNameP::IterationOptionName(r) => *r,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct LookupPE {
    pub name: IImpreciseNameP,
    pub template_args: Option<TemplateArgsP>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TemplateArgsP {
    pub range: RangeL,
    pub args: Vec<ITemplexPT>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct MagicParamLookupPE {
    pub range: RangeL,
}

#[derive(Clone, Debug, PartialEq)]
pub struct LambdaPE {
    pub captures: Option<UnitP>,
    pub function: FunctionP,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BlockPE {
    pub range: RangeL,
    pub maybe_pure: Option<RangeL>,
    pub maybe_default_region: Option<RegionRunePT>,
    pub inner: Box<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ConsecutorPE {
    pub inners: Vec<IExpressionPE>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ShortcallPE {
    pub range: RangeL,
    pub arg_exprs: Vec<IExpressionPE>,
}

