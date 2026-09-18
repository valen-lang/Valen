use crate::postparsing::names::IRuneS;
use crate::StrI;
use crate::typing::names::names::IVarNameT;

// An expression for expressing the group(s) a function might mutate or a ref might point at (as opposed to GroupStep which is a specific mutation to a specific group).
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum GroupExprG<'s, 't, 'g> {
  Rune(&'s IRuneS<'s>), // a group param, e.g. <g'>, resolved to its id
  ParamAnonymousGroup(&'t IVarNameT<'s, 't>), // A param's group if it doesn't come from a rune or another param. The StrI is the parameter's name
  Local(&'t IVarNameT<'s, 't>), // A local's implicitly declared group.
  Member { base: &'g GroupExprG<'s, 't, 'g>, member_name: StrI<'s> }, // `x.items`
  ChildElements { base: &'g GroupExprG<'s, 't, 'g> }, // the `[]` part of `x.items[]` if items is a Box/Vec/RSA
  InlineElements { base: &'g GroupExprG<'s, 't, 'g> }, // the `[]` part of `x.items[]` if items is a SSA.
  Variant { base: &'g GroupExprG<'s, 't, 'g>, variant_name: StrI<'s> }, // an enum's variant, the `WarpEngine` part of `my_ship.engine_enum.WarpEngine`
  Union { members: &'g [&'g GroupExprG<'s, 't, 'g>] }, // This ref points at multiple groups, or this function mutates multiple groups
  Ellipsis { base: &'g GroupExprG<'s, 't, 'g> }, // the `...` part of `x...`
}
