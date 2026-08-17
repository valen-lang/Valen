#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RegionTemplataType {}

/// The type of a group generic param's value (`ITemplataT::Group`). The param stays uniform with
/// type/int params; its value is the ceremonial constant `Group(Default)` and never enters a `KindT`.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct GroupTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ImplTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct KindTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct IntegerTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct BooleanTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct StringTemplataType {}

// VCOORD: Internal-only marker for runes whose conclusion is a concrete
// function prototype (result / prototype_rune of ResolveSR / CallSiteFuncSR /
// DefinitionFuncSR). The surface `T Prot` type is retired; this marker is not
// exposed there. Kept parallel to PackTemplataType — no surface counterpart,
// but the rune-type solver needs the marker for surviving bound-machinery rules.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PrototypeTemplataType {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct PackTemplataType<'s> {
  pub element_type: &'s ITemplataType<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct TemplateTemplataType<'s> {
  pub param_types: &'s [ITemplataType<'s>],
  pub return_type: &'s ITemplataType<'s>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum ITemplataType<'s> {
  RegionTemplataType(RegionTemplataType),
  GroupTemplataType(GroupTemplataType),
  ImplTemplataType(ImplTemplataType),
  KindTemplataType(KindTemplataType),
  FunctionTemplataType(FunctionTemplataType),
  IntegerTemplataType(IntegerTemplataType),
  BooleanTemplataType(BooleanTemplataType),
  StringTemplataType(StringTemplataType),
  PrototypeTemplataType(PrototypeTemplataType),
  PackTemplataType(PackTemplataType<'s>),
  TemplateTemplataType(TemplateTemplataType<'s>),
}
