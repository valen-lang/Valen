//! `calculate_aliasing_info` — the borrow checker's third phase, per
//! `docs/architecture/borrowing-design.md`'s `calculate_aliasing_info` section.
//!
//! It reports which parameters may be treated as `noalias` (a signature-only per-parameter verdict) and
//! which spans are restrict regions (where one reference is the sole live reference into its group),
//! for the backend to feed to LLVM as `restrict`-equivalent aliasing information. Regions come from the
//! access log `groupify_function` recorded, not a second tree walk.

use crate::postparsing::ast::FunctionS;
use crate::postparsing::rules::types::{ITypeST, RegionS};
use crate::typing::ast::ast::{FunctionAliasingInfoT, FunctionDefinitionT};
use crate::typing::ast::borrowing_ast::{GroupIdStepT, GroupIdT, LocKey, RestrictRegionT};
use crate::typing::borrow_checker::borrow_types::KindGT;
use crate::typing::borrow_checker::grouped_ast::{flatten, paths_alias, AccessEventG, GroupStep};
use crate::typing::borrow_checker::groupify::rune_name;
use crate::typing::compiler::Compiler;
use crate::typing::names::names::IVarNameT;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
  /// Report which parameters are `noalias` and which spans are restrict regions, to feed the backend for
  /// optimization. The per-parameter verdict is signature-only: a borrow parameter qualifies when its
  /// group path is not aliased by any other parameter's, exactly as `groupify` derives it. The restrict
  /// regions come from the `access_log` walked out of `groupify_function`.
  pub(crate) fn calculate_aliasing_info(
    &self,
    function_s: &'s FunctionS<'s>,
    function_t: &'t FunctionDefinitionT<'s, 't>,
    access_log: &[AccessEventG<'s, 't>],
  ) -> FunctionAliasingInfoT {
    let params_t = &function_t.header.params;
    // Each parameter's group path, or None when the parameter is not a `noalias`-eligible borrow.
    let paths: Vec<Option<Vec<GroupStep<'s, 't>>>> = (0..params_t.len())
      .map(|i| {
        let ps = function_s.params.get(i)?;
        // A `held` borrow's group representation is deferred, so make no promise about it.
        match &ps.tyype {
          ITypeST::BorrowRef(st) if !matches!(st.region, RegionS::Held) => {
            match self.make_kind_g(params_t[i].tyype, &ps.tyype, Some(params_t[i].name)) {
              KindGT::BorrowRef(b) => Some(flatten(&b.group)),
              _ => None,
            }
          }
          _ => None,
        }
      })
      .collect();
    let param_noalias = (0..paths.len())
      .map(|i| match &paths[i] {
        None => false,
        Some(path_i) => !paths.iter().enumerate().any(|(j, other)| {
          j != i && other.as_ref().map_or(false, |path_j| paths_alias(path_i, path_j))
        }),
      })
      .collect();
    FunctionAliasingInfoT { param_noalias, restrict_regions: compute_restrict_regions(access_log) }
  }
}

/// Derive the restrict regions from the access log. For each group (identified by its root step), find
/// maximal spans where every access into that group goes through one reference and no call in the span
/// touches the group; a span with at least one access and one call is a region.
fn compute_restrict_regions<'s, 't>(log: &[AccessEventG<'s, 't>]) -> Vec<RestrictRegionT> {
  // The distinct group roots that appear as an access target.
  let mut roots: Vec<GroupStep<'s, 't>> = vec![];
  for ev in log {
    if let AccessEventG::Access { group, .. } = ev {
      if let Some(root) = group.first() {
        if !roots.contains(root) {
          roots.push(root.clone());
        }
      }
    }
  }
  let mut regions = vec![];
  for root in &roots {
    regions_for_root(log, root, &mut regions);
  }
  regions
}

/// Scan the log for maximal spans exclusive to one reference into `root`, emitting a region per span.
fn regions_for_root<'s, 't>(
  log: &[AccessEventG<'s, 't>],
  root: &GroupStep<'s, 't>,
  out: &mut Vec<RestrictRegionT>,
) {
  // The current span: where it started, and the sole reference into `root` seen in it.
  let mut current: Option<(usize, IVarNameT<'s, 't>)> = None;
  for (k, ev) in log.iter().enumerate() {
    match ev {
      // An access into `root` through a different reference ends the span and starts a new one.
      AccessEventG::Access { base_ref, group, .. } if group.first() == Some(root) => match current {
        None => current = Some((k, *base_ref)),
        Some((start, sole_ref)) if sole_ref != *base_ref => {
          emit_region(log, root, start, k, sole_ref, out);
          current = Some((k, *base_ref));
        }
        _ => {}
      },
      // A call that touches `root` breaks exclusivity, ending the span.
      AccessEventG::Call { touched, .. } if touched.iter().any(|t| t.first() == Some(root)) => {
        if let Some((start, sole_ref)) = current.take() {
          emit_region(log, root, start, k, sole_ref, out);
        }
      }
      _ => {}
    }
  }
  if let Some((start, sole_ref)) = current.take() {
    emit_region(log, root, start, log.len(), sole_ref, out);
  }
}

/// Collect a span's accesses (through `sole_ref` into `root`), calls, markers and disjoint groups, and
/// push a region if it has at least one access and one call.
fn emit_region<'s, 't>(
  log: &[AccessEventG<'s, 't>],
  root: &GroupStep<'s, 't>,
  start: usize,
  end: usize,
  sole_ref: IVarNameT<'s, 't>,
  out: &mut Vec<RestrictRegionT>,
) {
  let mut accesses = vec![];
  let mut calls = vec![];
  let mut markers = vec![];
  let mut disjoint_roots: Vec<GroupStep<'s, 't>> = vec![];
  for ev in &log[start..end] {
    match ev {
      AccessEventG::Access { base_ref, group, loct, .. }
        if group.first() == Some(root) && *base_ref == sole_ref =>
      {
        accesses.push(LocKey { path: loct.path.to_vec() });
      }
      AccessEventG::Access { group, .. } => {
        if let Some(other) = group.first() {
          if other != root && !disjoint_roots.contains(other) {
            disjoint_roots.push(other.clone());
          }
        }
      }
      AccessEventG::Call { loct, .. } => calls.push(LocKey { path: loct.path.to_vec() }),
      AccessEventG::Marker { value } => markers.push(*value),
    }
  }
  if !accesses.is_empty() && !calls.is_empty() {
    out.push(RestrictRegionT {
      scope: group_id_from_steps(std::slice::from_ref(root)),
      disjoint: disjoint_roots.iter().map(|s| group_id_from_steps(std::slice::from_ref(s))).collect(),
      accesses,
      calls,
      markers,
    });
  }
}

/// The owned, lifetime-free mirror of a flattened group path.
fn group_id_from_steps<'s, 't>(steps: &[GroupStep<'s, 't>]) -> GroupIdT {
  GroupIdT { steps: steps.iter().map(group_id_step).collect() }
}

fn group_id_step<'s, 't>(step: &GroupStep<'s, 't>) -> GroupIdStepT {
  match step {
    GroupStep::Rune(r) => {
      GroupIdStepT::Rune(rune_name(*r).map(|n| n.0.to_string()).unwrap_or_else(|| "?".to_string()))
    }
    GroupStep::ParamAnonymousGroup(v) => GroupIdStepT::ParamAnonymousGroup(var_name_string(v)),
    GroupStep::Local(v) => GroupIdStepT::Local(var_name_string(v)),
    GroupStep::Member { member_name } => GroupIdStepT::Member(member_name.0.to_string()),
    GroupStep::Elements => GroupIdStepT::Elements,
  }
}

/// A variable's source-level name, for rendering a group that names a parameter or local.
fn var_name_string<'s, 't>(v: &IVarNameT<'s, 't>) -> String {
  match v {
    IVarNameT::Local(ln) => ln.imprecise_name.name.0.to_string(),
    IVarNameT::Member(mn) => mn.imprecise_name.name.0.to_string(),
    _ => "?".to_string(),
  }
}
