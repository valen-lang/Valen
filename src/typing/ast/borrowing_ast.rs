//! Owned, backend-facing types for the borrow checker's block-scoped restrict analysis: a restrict
//! region and the lifetime-free group and location keys it carries. They are deliberately owned (no
//! arena lifetime) so `FunctionAliasingInfoT` can hold them past the check arena and hand them to the
//! backend unchanged. See `docs/architecture/borrowing-design.md`.

/// A block-scoped restrict region: a span of a function where one reference is the sole live reference
/// into `scope`'s group. The backend emits `!alias.scope {scope}` + `!noalias {disjoint}` on the
/// `accesses` (loads/stores) inside the span, and `!noalias {scope}` on the `calls` (which are proven
/// not to touch the group). `markers` are bare-integer landmarks placed in the span so a test can pin
/// the region by value. Everything is owned, so `FunctionAliasingInfoT` carries no arena lifetime.
#[derive(Clone, Debug)]
pub struct RestrictRegionT {
  pub scope: GroupIdT,
  pub disjoint: Vec<GroupIdT>,
  pub accesses: Vec<LocKey>,
  pub calls: Vec<LocKey>,
  pub markers: Vec<i32>,
}

impl RestrictRegionT {
  /// The scope group's source-level name, e.g. `g`.
  pub fn scope_group_name(&self) -> String {
    self.scope.name()
  }

  /// The source-level names of the groups this region's accesses are disjoint from.
  pub fn disjoint_group_names(&self) -> Vec<String> {
    self.disjoint.iter().map(|g| g.name()).collect()
  }
}

/// An owned, lifetime-free mirror of a group path. It is the backend's alias-scope identity (compared
/// by equality to give each distinct group one MDNode) and renders to a source-level name for
/// diagnostics and tests. Leaves are owned strings rather than interned handles, so it holds no arena
/// lifetime.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct GroupIdT {
  pub steps: Vec<GroupIdStepT>,
}

/// One step of an owned group path (the lifetime-free mirror of `GroupStep`).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum GroupIdStepT {
  /// A named group rune, e.g. `g` in `func f<g'>(a &Ship in g)`.
  Rune(String),
  /// A borrow parameter's anonymous group, named by the parameter.
  ParamAnonymousGroup(String),
  /// A local reference's group, named by the local.
  Local(String),
  /// A member step into a group, e.g. `.tiles`.
  Member(String),
  /// The elements-group step, e.g. `[]`.
  Elements,
}

impl GroupIdT {
  /// The group's source-level name, e.g. `g` or `g.tiles[]`.
  pub fn name(&self) -> String {
    let mut s = String::new();
    for step in &self.steps {
      match step {
        GroupIdStepT::Rune(n) | GroupIdStepT::ParamAnonymousGroup(n) | GroupIdStepT::Local(n) => {
          s.push_str(n)
        }
        GroupIdStepT::Member(n) => {
          s.push('.');
          s.push_str(n)
        }
        GroupIdStepT::Elements => s.push_str("[]"),
      }
    }
    s
  }
}

/// An owned copy of a `LocT` path: a lifetime-free key naming one access site inside a restrict region,
/// matched against the instantiated `LocI` in codegen.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct LocKey {
  pub path: Vec<i32>,
}
