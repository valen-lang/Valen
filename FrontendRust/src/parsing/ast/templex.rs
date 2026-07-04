use super::ast::{SharednessP, NameP};
use super::rules::ITypePR;
use crate::interner::StrI;
use crate::lexing::RangeL;


#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ITemplexPT<'p> {
  AnonymousRune(AnonymousRunePT),
  Bool(BoolPT),
  Point(PointPT<'p>),
  Call(CallPT<'p>),
  Function(FunctionPT<'p>),
  Inline(InlinePT<'p>),
  Int(IntPT),
  RegionRune(RegionRunePT<'p>),
  Tuple(TuplePT<'p>),
  NameOrRune(NameOrRunePT<'p>),
  BorrowRef(BorrowRefPT<'p>),
  WeakRef(WeakRefPT<'p>),
  ShareRef(ShareRefPT<'p>),
  HeapOwnRef(HeapOwnRefPT<'p>),
  Pack(PackPT<'p>),
  Func(FuncPT<'p>),
  StaticSizedArray(StaticSizedArrayPT<'p>),
  RuntimeSizedArray(RuntimeSizedArrayPT<'p>),
  String(StringPT<'p>),
  TypedRune(TypedRunePT<'p>),
}
impl ITemplexPT<'_> {
  pub fn range(&self) -> RangeL {
    match self {
      ITemplexPT::AnonymousRune(r) => r.range,
      ITemplexPT::Bool(r) => r.range,
      ITemplexPT::Point(r) => r.range,
      ITemplexPT::Call(r) => r.range,
      ITemplexPT::Function(r) => r.range,
      ITemplexPT::Inline(r) => r.range,
      ITemplexPT::Int(r) => r.range,
      ITemplexPT::RegionRune(r) => r.range,
      ITemplexPT::Tuple(r) => r.range,
      ITemplexPT::NameOrRune(n) => n.name.0,
      ITemplexPT::BorrowRef(r) => r.range,
      ITemplexPT::WeakRef(r) => r.range,
      ITemplexPT::ShareRef(r) => r.range,
      ITemplexPT::HeapOwnRef(r) => r.range,
      ITemplexPT::Pack(p) => p.range,
      ITemplexPT::Func(r) => r.range,
      ITemplexPT::StaticSizedArray(r) => r.range,
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
pub struct PointPT<'p> {
  pub range: RangeL,
  pub inner: &'p ITemplexPT<'p>,
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
pub struct InlinePT<'p> {
  pub range: RangeL,
  pub inner: &'p ITemplexPT<'p>,
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
pub struct SharednessPT(pub RangeL, pub SharednessP);


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


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct BorrowRefPT<'p> {
  pub range: RangeL,
  pub inner: &'p ITemplexPT<'p>,
  pub region: Option<&'p RegionRunePT<'p>>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WeakRefPT<'p> {
  pub range: RangeL,
  pub inner: &'p ITemplexPT<'p>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ShareRefPT<'p> {
  pub range: RangeL,
  pub inner: &'p ITemplexPT<'p>,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct HeapOwnRefPT<'p> {
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
pub struct StaticSizedArrayPT<'p> {
  pub range: RangeL,
  pub size: &'p ITemplexPT<'p>,
  pub element: &'p ITemplexPT<'p>,
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


