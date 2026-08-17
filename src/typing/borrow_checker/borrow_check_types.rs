// Borrow-checker data types. `B` suffix = borrow-checker-only (even though borrow checking runs
// inside the typing pass). These are minted by the checker from the scout-side `GroupS`/`EffectS`
// plus the typed conclusions; none of them ever lives on a `KindT` or on the durable `FunctionHeaderT`.

use crate::interner::StrI;
use crate::typing::names::names::IdT;

/// The borrow checker's typed group algebra, minted from the declaration-side `GroupS` plus the
/// typed conclusions. Never in a `KindT`, never on the durable header — the checker reconstructs it
/// per value by tracing a place to its anchor and reading the anchor's `GroupS` off the side table.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum GroupB<'s, 't> {
  /// `mut(())` — the empty group; the identity of union.
  Empty,
  /// A group param, resolved to its id.
  Rune(IdT<'s, 't>),
  /// A local, resolved to its id.
  Local(IdT<'s, 't>),
  /// `x.items` — the named member.
  Member { base: Box<GroupB<'s, 't>>, member_name: StrI<'s> },
  /// `x.items[]` — an element of the member.
  Elements { base: Box<GroupB<'s, 't>> },
  /// `(a | b)` — a union. Canonical on construction (flatten nested unions, drop `Empty` members,
  /// dedup, sort by a stable key, collapse a singleton to its member) so permission-map keys behave.
  Union { members: Vec<GroupB<'s, 't>> },
}

/// A borrow-checker effect over a `GroupB`.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum EffectB<'s, 't> {
  Mut(GroupB<'s, 't>),
  NotMut(GroupB<'s, 't>),
}
