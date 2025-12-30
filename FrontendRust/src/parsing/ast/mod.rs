// AST module - re-exports all parsing AST types
// Mirrors Frontend/ParsingPass/src/dev/vale/parsing/ast/

pub mod ast;
pub mod expressions;
pub mod pattern;
pub mod rules;
pub mod templex;

// Re-export everything from ast.rs
pub use ast::{
    UnitP,
    NameP,
    FileP,
    IDenizenP,
    ImplP,
    ExportAsP,
    ImportP,
    IAttributeP,
    IMacroInclusionP,
    IRuneAttributeP,
    StructP,
    StructMembersP,
    IStructContent,
    InterfaceP,
    FunctionP,
    FunctionHeaderP,
    FunctionReturnP,
    GenericParameterP,
    GenericParameterTypeP,
    GenericParametersP,
    TemplateRulesP,
    ParamsP,
    MutabilityP,
    VariabilityP,
    OwnershipP,
    LoadAsP,
    LocationP,
};

// Re-export everything from expressions.rs
pub use expressions::{
    IExpressionPE,
    VoidPE,
    PackPE,
    SubExpressionPE,
    AndPE,
    OrPE,
    IfPE,
    WhilePE,
    EachPE,
    RangePE,
    DestructPE,
    UnletPE,
    MutatePE,
    ReturnPE,
    BreakPE,
    LetPE,
    TuplePE,
    IArraySizeP,
    ConstructArrayPE,
    ConstantIntPE,
    ConstantBoolPE,
    ConstantStrPE,
    ConstantFloatPE,
    StrInterpolatePE,
    DotPE,
    IndexPE,
    FunctionCallPE,
    BraceCallPE,
    NotPE,
    AugmentPE,
    TransmigratePE,
    BinaryCallPE,
    MethodCallPE,
    IImpreciseNameP,
    LookupPE,
    TemplateArgsP,
    MagicParamLookupPE,
    LambdaPE,
    BlockPE,
    ConsecutorPE,
    ShortcallPE,
};

// Re-export everything from pattern.rs
pub use pattern::{
    AbstractP,
    ParameterP,
    DestinationLocalP,
    PatternPP,
    DestructureP,
    INameDeclarationP,
};

// Re-export everything from rules.rs
pub use rules::{
    IRulexPR,
    ITypePR,
};

// Re-export everything from templex.rs
pub use templex::{
    ITemplexPT,
    RegionRunePT,
    OwnershipPT,
    PackPT,
};

