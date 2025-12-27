use crate::StrI;

/// Position range in source code
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RangeL {
    pub begin: i32,
    pub end: i32,
}

impl RangeL {
    pub fn new(begin: i32, end: i32) -> Self {
        assert!(begin == end || begin <= end);
        RangeL { begin, end }
    }

    pub fn zero() -> Self {
        RangeL { begin: 0, end: 0 }
    }
}

/// A file with top-level denizens
#[derive(Clone, Debug, PartialEq)]
pub struct FileL {
    pub denizens: Vec<IDenizenL>,
    pub comment_ranges: Vec<RangeL>,
}

/// Top-level items in a file
#[derive(Clone, Debug, PartialEq)]
pub enum IDenizenL {
    TopLevelFunction(FunctionL),
    TopLevelStruct(StructL),
    TopLevelInterface(InterfaceL),
    TopLevelImpl(ImplL),
    TopLevelExportAs(ExportAsL),
    TopLevelImport(ImportL),
}

/// Impl block
#[derive(Clone, Debug, PartialEq)]
pub struct ImplL {
    pub range: RangeL,
    pub identifying_runes: Option<AngledLE>,
    pub template_rules: Option<ScrambleLE>,
    pub struct_: Option<ScrambleLE>,  // Option because we can say `impl MyInterface;` inside a struct
    pub interface: ScrambleLE,
    pub attributes: Vec<IAttributeL>,
}

/// Export as declaration
#[derive(Clone, Debug, PartialEq)]
pub struct ExportAsL {
    pub range: RangeL,
    pub contents: ScrambleLE,
}

/// Import declaration
#[derive(Clone, Debug, PartialEq)]
pub struct ImportL {
    pub range: RangeL,
    pub module_name: WordLE,
    pub package_steps: Vec<WordLE>,
    pub importee_name: WordLE,
}

/// Struct definition
#[derive(Clone, Debug, PartialEq)]
pub struct StructL {
    pub range: RangeL,
    pub name: WordLE,
    pub attributes: Vec<IAttributeL>,
    pub mutability: Option<ScrambleLE>,
    pub identifying_runes: Option<AngledLE>,
    pub template_rules: Option<ScrambleLE>,
    pub members: ScrambleLE,
}

/// Interface definition
#[derive(Clone, Debug, PartialEq)]
pub struct InterfaceL {
    pub range: RangeL,
    pub name: WordLE,
    pub attributes: Vec<IAttributeL>,
    pub mutability: Option<ScrambleLE>,
    pub maybe_identifying_runes: Option<AngledLE>,
    pub template_rules: Option<ScrambleLE>,
    pub body_range: RangeL,
    pub members: Vec<FunctionL>,
}

/// Attributes on declarations
#[derive(Clone, Debug, PartialEq)]
pub enum IAttributeL {
    AbstractAttribute(RangeL),
    ExportAttribute(RangeL),
    PureAttribute(RangeL),
    AdditiveAttribute(RangeL),
    ExternAttribute { range: RangeL, maybe_custom_name: Option<ParendLE> },
    LinearAttribute(RangeL),
    WeakableAttribute(RangeL),
    SealedAttribute(RangeL),
    MacroCall { range: RangeL, inclusion: IMacroInclusionL, name: WordLE },
}

/// Macro inclusion type
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum IMacroInclusionL {
    CallMacro,
    DontCallMacro,
}

/// Function definition
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionL {
    pub range: RangeL,
    pub header: FunctionHeaderL,
    pub body: Option<FunctionBodyL>,
}

/// Function body
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionBodyL {
    pub body: CurliedLE,
}

/// Function header
#[derive(Clone, Debug, PartialEq)]
pub struct FunctionHeaderL {
    pub range: RangeL,
    pub name: WordLE,
    pub attributes: Vec<IAttributeL>,
    pub maybe_user_specified_identifying_runes: Option<AngledLE>,
    pub params: ParendLE,
    /// Includes: where clause, return type, default region for the body
    /// Basically, everything up until the body's { or a ;
    pub trailing_details: ScrambleLE,
}

/// Node in the lexer tree
pub trait INodeLE {
    fn range(&self) -> RangeL;
}

/// A scramble of lexer nodes (no structure yet)
#[derive(Clone, Debug, PartialEq)]
pub struct ScrambleLE {
    pub range: RangeL,
    pub elements: Vec<Box<INodeLEEnum>>,
}

impl INodeLE for ScrambleLE {
    fn range(&self) -> RangeL {
        self.range
    }
}

/// Enum wrapper for INodeLE to allow storing in vectors
#[derive(Clone, Debug, PartialEq)]
pub enum INodeLEEnum {
    Parend(ParendLE),
    Curlied(CurliedLE),
    Squared(SquaredLE),
    Angled(AngledLE),
    Word(WordLE),
    Symbol(SymbolLE),
    String(StringLE),
    ParsedInteger(ParsedIntegerLE),
    ParsedDouble(ParsedDoubleLE),
    Scramble(ScrambleLE),  // For recursive cases
}

impl INodeLE for INodeLEEnum {
    fn range(&self) -> RangeL {
        match self {
            INodeLEEnum::Parend(x) => x.range,
            INodeLEEnum::Curlied(x) => x.range,
            INodeLEEnum::Squared(x) => x.range,
            INodeLEEnum::Angled(x) => x.range,
            INodeLEEnum::Word(x) => x.range,
            INodeLEEnum::Symbol(x) => x.range,
            INodeLEEnum::String(x) => x.range,
            INodeLEEnum::ParsedInteger(x) => x.range,
            INodeLEEnum::ParsedDouble(x) => x.range,
            INodeLEEnum::Scramble(x) => x.range,
        }
    }
}

/// Parenthesized expression
#[derive(Clone, Debug, PartialEq)]
pub struct ParendLE {
    pub range: RangeL,
    pub contents: ScrambleLE,
}

impl INodeLE for ParendLE {
    fn range(&self) -> RangeL {
        self.range
    }
}

/// Angled brackets (generics)
#[derive(Clone, Debug, PartialEq)]
pub struct AngledLE {
    pub range: RangeL,
    pub contents: ScrambleLE,
}

impl INodeLE for AngledLE {
    fn range(&self) -> RangeL {
        self.range
    }
}

/// Squared brackets (arrays)
#[derive(Clone, Debug, PartialEq)]
pub struct SquaredLE {
    pub range: RangeL,
    pub contents: ScrambleLE,
}

impl INodeLE for SquaredLE {
    fn range(&self) -> RangeL {
        self.range
    }
}

/// Curly braces (blocks)
#[derive(Clone, Debug, PartialEq)]
pub struct CurliedLE {
    pub range: RangeL,
    pub contents: ScrambleLE,
}

impl INodeLE for CurliedLE {
    fn range(&self) -> RangeL {
        self.range
    }
}

/// Word/identifier
#[derive(Clone, Debug, PartialEq)]
pub struct WordLE {
    pub range: RangeL,
    pub str: StrI,
}

impl INodeLE for WordLE {
    fn range(&self) -> RangeL {
        self.range
    }
}

/// Single character symbol
#[derive(Clone, Debug, PartialEq)]
pub struct SymbolLE {
    pub range: RangeL,
    pub c: char,
}

impl INodeLE for SymbolLE {
    fn range(&self) -> RangeL {
        self.range
    }
}

/// String literal
#[derive(Clone, Debug, PartialEq)]
pub struct StringLE {
    pub range: RangeL,
    pub parts: Vec<StringPart>,
}

impl INodeLE for StringLE {
    fn range(&self) -> RangeL {
        self.range
    }
}

/// Part of a string (literal or interpolated expression)
#[derive(Clone, Debug, PartialEq)]
pub enum StringPart {
    Literal { range: RangeL, s: String },
    Expr(ScrambleLE),
}

/// Parsed integer literal
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedIntegerLE {
    pub range: RangeL,
    pub value: i64,
    pub bits: Option<i64>,
}

impl INodeLE for ParsedIntegerLE {
    fn range(&self) -> RangeL {
        self.range
    }
}

/// Parsed floating-point literal
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedDoubleLE {
    pub range: RangeL,
    pub value: f64,
    pub bits: Option<i64>,
}

impl INodeLE for ParsedDoubleLE {
    fn range(&self) -> RangeL {
        self.range
    }
}

