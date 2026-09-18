//! `calculate_aliasing_info` — the borrow checker's third phase, on the canonical value types.
//!
//! It reports which parameters may be treated as `noalias` (a signature-only per-parameter verdict) and
//! which spans are restrict regions (where one reference is the sole live reference into its group).
//! Regions come from the access log `groupify_function` recorded.

use bumpalo::Bump;

use crate::postparsing::ast::FunctionS;
use crate::postparsing::rules::types::{ITypeST, RegionS};
use crate::typing::ast::ast::{FunctionAliasingInfoT, FunctionDefinitionT};
use crate::typing::ast::borrowing_ast::{GroupIdStepT, GroupIdT, LocKey, RestrictRegionT};
use crate::typing::borrow_checker::access_event::AccessEventG;
use crate::typing::borrow_checker::ast_g::GroupStep;
use crate::typing::borrow_checker::experimental::grouped_ast::{flatten, paths_alias};
use crate::typing::borrow_checker::experimental::groupify::rune_name;
use crate::typing::borrow_checker::kind_g::KindGT;
use crate::typing::compiler::Compiler;
use crate::typing::names::names::IVarNameT;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
  /// Report which parameters are `noalias` and which spans are restrict regions. The per-parameter
  /// verdict is signature-only: a borrow parameter qualifies when its group path is not aliased by any
  /// other parameter's. The restrict regions come from the `access_log`.
  pub(crate) fn calculate_aliasing_info<'g>(
    &self,
    function_s: &'s FunctionS<'s>,
    function_t: &'t FunctionDefinitionT<'s, 't>,
    access_log: &[AccessEventG<'s, 't>],
    arena: &'g Bump,
  ) -> FunctionAliasingInfoT {
    let params_t = &function_t.header.params;
    let paths: Vec<Option<Vec<GroupStep<'s, 't>>>> = (0..params_t.len())
      .map(|i| {
        let ps = function_s.params.get(i)?;
        match &ps.tyype {
          ITypeST::BorrowRef(st) if !matches!(st.region, RegionS::Held) => {
            match self.make_kind_g(params_t[i].tyype, &ps.tyype, Some(&params_t[i].name), arena) {
              KindGT::BorrowRef(b) => Some(flatten(b.group)),
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

/// Derive the restrict regions from the access log. For each group root that appears as an access
/// target, find maximal spans where every access goes through one reference and no call touches it.
fn compute_restrict_regions<'s, 't>(log: &[AccessEventG<'s, 't>]) -> Vec<RestrictRegionT> {
  let mut roots: Vec<GroupStep<'s, 't>> = vec![];
  for ev in log {
    if let AccessEventG::Read { group, .. } | AccessEventG::Store { group, .. } = ev {
      if let Some(root) = group.first() {
        if !roots.contains(root) {
          roots.push(*root);
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
  let mut current: Option<(usize, IVarNameT<'s, 't>)> = None;
  for (k, ev) in log.iter().enumerate() {
    match ev {
      AccessEventG::Read { base_ref, group, .. } | AccessEventG::Store { base_ref, group, .. }
        if group.first() == Some(root) =>
      {
        match current {
          None => current = Some((k, *base_ref)),
          Some((start, sole_ref)) if sole_ref != *base_ref => {
            emit_region(log, root, start, k, sole_ref, out);
            current = Some((k, *base_ref));
          }
          _ => {}
        }
      }
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

/// Collect a span's accesses (through `sole_ref` into `root`), calls and disjoint groups, and push a
/// region if it has at least one access and one call.
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
      AccessEventG::Read { base_ref, group, loct } | AccessEventG::Store { base_ref, group, loct } => {
        if group.first() == Some(root) && *base_ref == sole_ref {
          accesses.push(LocKey { path: loct.path.to_vec() });
        } else if let Some(other) = group.first() {
          if other != root && !disjoint_roots.contains(other) {
            disjoint_roots.push(*other);
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
      GroupIdStepT::Rune(rune_name(**r).map(|n| n.0.to_string()).unwrap_or_else(|| "?".to_string()))
    }
    GroupStep::ParamAnonymousGroup(v) => GroupIdStepT::ParamAnonymousGroup(var_name_string(v)),
    GroupStep::Local(v) => GroupIdStepT::Local(var_name_string(v)),
    GroupStep::Member { member_name } => GroupIdStepT::Member(member_name.0.to_string()),
    // Both `[]` element groups render as the elements step; the destructibility distinction matters to
    // the checker, not to the backend's region id.
    GroupStep::ChildElements | GroupStep::InlineElements => GroupIdStepT::Elements,
    GroupStep::Variant { .. } => {
      panic!("vfail: variant group step in a restrict region is not yet supported")
    }
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
