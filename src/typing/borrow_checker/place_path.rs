use crate::typing::ast::expressions::ExpressionTE;
use crate::typing::names::names::IVarNameT;

/// One step of a place path below its root local.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Segment<'s, 't> {
  /// A named struct member, e.g. `.flagship`. Stays in the parent group.
  Member(IVarNameT<'s, 't>),
  /// An element of a runtime-sized array, e.g. `arr[i]`. The index is not tracked (it is a runtime
  /// value), so all elements of one array share one segment. An element is in a **child group**: the
  /// container can move or delete it independently, so a churn of the parent invalidates it.
  Element,
}

/// The place an argument expression refers to: a root local plus the member steps taken from it.
/// Two paths alias when they refer to overlapping storage.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PlacePath<'s, 't> {
  pub root: IVarNameT<'s, 't>,
  pub segments: Vec<Segment<'s, 't>>,
}

impl<'s, 't> PlacePath<'s, 't> {
  /// Two place paths alias when they refer to overlapping storage: the same root, and one segment
  /// list is a prefix of the other. Equal lists are the same place; a proper prefix is one place
  /// nested in the other; a first differing segment means disjoint siblings (the
  /// sibling-disjointness lemma), which do not alias.
  pub fn aliases(&self, other: &PlacePath<'s, 't>) -> bool {
    if self.root != other.root {
      return false;
    }
    let common = self.segments.len().min(other.segments.len());
    self.segments[..common] == other.segments[..common]
  }

  /// Whether this path names a place in a **child group** — i.e. it steps through an array element.
  /// A churn of the parent group invalidates references into child groups (an element can be moved
  /// or deleted); a parent-group reference (a root or member-only path) survives.
  pub fn is_child_group(&self) -> bool {
    self.segments.iter().any(|segment| matches!(segment, Segment::Element))
  }
}

/// The local an argument moves out, if it is a move (an `UnletTE`, produced by `^local`). A non-move
/// argument returns `None`.
pub fn moved_root<'s, 't>(expr: &ExpressionTE<'s, 't>) -> Option<IVarNameT<'s, 't>> {
  match *expr {
    ExpressionTE::Unlet(unlet) => Some(unlet.variable.name),
    _ => None,
  }
}

/// Extract the place an argument refers to, if it is a place expression: a local, a member of one
/// (transparently through a deref). A non-place argument returns `None`.
pub fn place_path<'s, 't>(expr: &ExpressionTE<'s, 't>) -> Option<PlacePath<'s, 't>> {
  match *expr {
    ExpressionTE::LocalLookup(local_lookup) => {
      Some(PlacePath { root: local_lookup.local_variable.name, segments: Vec::new() })
    }
    ExpressionTE::Deref(deref) => place_path(&deref.inner),
    ExpressionTE::MemberLookup(member_lookup) => {
      let mut path = place_path(&member_lookup.struct_expr)?;
      path.segments.push(Segment::Member(member_lookup.member_name));
      Some(path)
    }
    ExpressionTE::RuntimeSizedArrayLookup(array_lookup) => {
      let mut path = place_path(&array_lookup.array_expr)?;
      path.segments.push(Segment::Element);
      Some(path)
    }
    ExpressionTE::StaticSizedArrayLookup(array_lookup) => {
      let mut path = place_path(&array_lookup.array_expr)?;
      path.segments.push(Segment::Element);
      Some(path)
    }
    _ => None,
  }
}
