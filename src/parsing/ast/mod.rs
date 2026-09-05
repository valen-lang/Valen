// AST module - re-exports all parsing AST types
// Mirrors Frontend/ParsingPass/src/dev/vale/parsing/ast/

pub mod ast;
pub mod expressions;
pub mod pattern;
pub mod rules;
pub mod templex;

// Re-export everything from ast.rs
pub use ast::{
  AbstractAttributeP, BuiltinAttributeP, ExportAsP, ExportAttributeP, ExternAttributeP, FileP,
  FunctionHeaderP, FunctionP, FunctionReturnP, GenericParameterP, GenericParameterTypeP,
  GenericParametersP, IAttributeP, IDenizenP, IMacroInclusionP, IRuneAttributeP, IStructContent,
  ImplP, ImportP, InterfaceP, LoadAsP, MacroCallP, NameP, NormalStructMemberP, ParamsP,
  SealedAttributeP, SharednessP, StructMembersP, StructP, TemplateRulesP, UnitP,
  VariadicStructMemberP,
};

// Re-export everything from expressions.rs
pub use expressions::{
  AndPE, BinaryCallPE, BlockPE, BorrowPE, BraceCallPE, BreakPE, ConsecutorPE, ConstantBoolPE,
  ConstantFloatPE, ConstantIntPE, ConstantStrPE, ConstructArrayPE, DestructPE, DotPE, EachPE,
  FunctionCallPE, IArraySizeP, IExpressionPE, IImpreciseNameP, IfPE, IndexPE, LambdaPE, LetPE,
  LookupPE, MagicParamLookupPE, MethodCallPE, MovePE, MutatePE, NotPE, OrPE, PackPE, RangePE,
  ReturnPE, ShortcallPE, StaticSizedArraySizeP, StrInterpolatePE, SubExpressionPE, TemplateArgsP,
  TransmigratePE, TuplePE, UnletPE, VoidPE, WeakPE, WhilePE,
};

// Re-export everything from pattern.rs
pub use pattern::{
  AbstractP, DestinationLocalP, DestructureP, INameDeclarationP, ParameterP, PatternPP,
};

// Re-export everything from rules.rs
pub use rules::{BuiltinCallPR, DotPR, EqualsPR, IRulexPR, ITypePR, OrPR, PackPR, TypedPR};

// Re-export everything from templex.rs
pub use templex::{
  AnonymousRunePT, BoolPT, BorrowRefPT, CallPT, EffectP, FuncPT, FunctionPT, GroupP, ITemplexPT,
  IntPT, NameOrRunePT, OwnRefPT, PackPT, RegionP, RegionRunePT, RuntimeSizedArrayPT, StringPT,
  TuplePT, TypedRunePT, WeakRefPT,
};
