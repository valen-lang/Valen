use super::ast::NameP;
use super::rules::ITypePR;
use crate::interner::StrI;
use crate::lexing::RangeL;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ITemplexPT<'p> {
  AnonymousRune(AnonymousRunePT),
  Bool(BoolPT),
  Call(CallPT<'p>),
  Function(FunctionPT<'p>),
  Int(IntPT),
  RegionRune(RegionRunePT<'p>),
  Tuple(TuplePT<'p>),
  NameOrRune(NameOrRunePT<'p>),
  BorrowRef(BorrowRefPT<'p>),
  WeakRef(WeakRefPT<'p>),
  OwnRef(OwnRefPT<'p>),
  Pack(PackPT<'p>),
  Func(FuncPT<'p>),
  RuntimeSizedArray(RuntimeSizedArrayPT<'p>),
  String(StringPT<'p>),
  TypedRune(TypedRunePT<'p>),
}
impl ITemplexPT<'_> {
  pub fn range(&self) -> RangeL {
    match self {
      ITemplexPT::AnonymousRune(r) => r.range,
      ITemplexPT::Bool(r) => r.range,
      ITemplexPT::Call(r) => r.range,
      ITemplexPT::Function(r) => r.range,
      ITemplexPT::Int(r) => r.range,
      ITemplexPT::RegionRune(r) => r.range,
      ITemplexPT::Tuple(r) => r.range,
      ITemplexPT::NameOrRune(n) => n.name.0,
      ITemplexPT::BorrowRef(r) => r.range,
      ITemplexPT::WeakRef(r) => r.range,
      ITemplexPT::OwnRef(r) => r.range,
      ITemplexPT::Pack(p) => p.range,
      ITemplexPT::Func(r) => r.range,
      ITemplexPT::RuntimeSizedArray(r) => r.range,
      ITemplexPT::String(r) => r.range,
      ITemplexPT::TypedRune(r) => r.range,
    }
  }
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AnonymousRunePT {
  pub range: RangeL,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BoolPT {
  pub range: RangeL,
  pub value: bool,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CallPT<'p> {
  pub range: RangeL,
  pub template: &'p ITemplexPT<'p>,
  pub args: &'p [&'p ITemplexPT<'p>],
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FunctionPT<'p> {
  pub range: RangeL,
  pub mutability: Option<&'p ITemplexPT<'p>>,
  pub parameters: &'p PackPT<'p>,
  pub return_type: &'p ITemplexPT<'p>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IntPT {
  pub range: RangeL,
  pub value: i64,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RegionRunePT<'p> {
  pub range: RangeL,
  pub name: Option<NameP<'p>>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TuplePT<'p> {
  pub range: RangeL,
  pub elements: &'p [&'p ITemplexPT<'p>],
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct NameOrRunePT<'p> {
  pub name: NameP<'p>,
  _sealed: (),
}
impl<'p> NameOrRunePT<'p> {
  pub fn new(name: NameP<'p>) -> Self {
    assert!(name.as_str() != "_", "vassert: NameOrRunePT name must not be \"_\"");
    Self { name, _sealed: () }
  }
}


/// The region of a borrow reference. `held` and an explicit region annotation are sibling values
/// here alongside "no annotation", so a borrow's region lives in one slot.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum RegionP<'p> {
  /// No region written: `&Ship`.
  Unspecified,
  /// A held reference: `held Ship`. A borrow into an anonymous region the callee treats as
  /// undestroyable, proven at the call site by the caller.
  Held,
  /// An explicit region annotation: `&'Ship` (anonymous rune) or `&i'Ship` (named).
  Rune(&'p RegionRunePT<'p>),
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BorrowRefPT<'p> {
  pub range: RangeL,
  pub inner: &'p ITemplexPT<'p>,
  pub region: RegionP<'p>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WeakRefPT<'p> {
  pub range: RangeL,
  pub inner: &'p ITemplexPT<'p>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct OwnRefPT<'p> {
  pub range: RangeL,
  pub inner: &'p ITemplexPT<'p>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PackPT<'p> {
  pub range: RangeL,
  pub members: &'p [&'p ITemplexPT<'p>],
}



#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FuncPT<'p> {
  pub range: RangeL,
  pub name: NameP<'p>,
  pub params_range: RangeL,
  pub parameters: &'p [&'p ITemplexPT<'p>],
  pub return_type: &'p ITemplexPT<'p>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RuntimeSizedArrayPT<'p> {
  pub range: RangeL,
  pub element: &'p ITemplexPT<'p>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StringPT<'p> {
  pub range: RangeL,
  pub str: StrI<'p>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct TypedRunePT<'p> {
  pub range: RangeL,
  pub rune: NameP<'p>,
  pub tyype: ITypePR,
}


