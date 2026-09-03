//! Phase 2: `check_usages` walks the grouped body and rejects a use of a reference a churn spoiled.
//!
//! It threads a `GroupSubtree` — a tree of groups as the function knows them — registering each live
//! reference under the group(s) it points at, and stamping `invalidated_by` on the references a churn
//! spoils. It walks the mirror in evaluation order: most nodes just recurse into their children; the
//! four that carry checker payloads are handled directly. At a call it walks the arguments (registering
//! each held-register temporary as it is produced), checks the argument reference-uses and the held
//! registers, reports any joint-argument violation, then applies the call's churns. `if` runs each
//! branch from the pre-branch state and unions the non-diverging invalidations; `while` pre-applies its
//! body's churns, so there is no fixpoint. See `docs/architecture/borrowing-design.md`.

use crate::postparsing::names::IRuneS;
use crate::postparsing::rules::types::EffectS;
use crate::typing::ast::ast::LocT;
use crate::typing::ast::expressions::{ExpressionTE, FunctionCallTE};
use crate::typing::borrow_checker::borrow_error::BorrowErrorKind;
use crate::typing::borrow_checker::borrow_types::{GroupExprG, KindGT};
use crate::typing::borrow_checker::grouped_ast::{
  flatten, paths_alias, GroupStep, IExpressionGE, JointFact,
};
use crate::typing::borrow_checker::groupify::{
  effect_root_rune, expr_range, held_range, moved_local, param_group_rune, place_root_local,
  rune_name,
};
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::names::names::IVarNameT;
use crate::utils::fx;
use crate::utils::range::RangeS;

/// A live reference's key in the tree: a named local, or an unnamed held register (a mid-expression
/// temporary), distinguished by a per-function counter so distinct temporaries never collide.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum RefKey<'s, 't> {
  Named(IVarNameT<'s, 't>),
  Held(u32),
}

/// One live reference's state: the churn that invalidated it, if any.
#[derive(Clone, Copy, Default)]
struct LocalEntry<'t> {
  invalidated_by: Option<LocT<'t>>,
}

/// A subtree for a group as the containing function knows it; it grows as the function learns of new
/// groups. The doc's `IndexSet<LocalEntry>` is realized as an `IndexMap` keyed by the entry's ref key
/// with the `LocalEntry` as the value — the same information, kept deterministic, but able to hold the
/// mutable `invalidated_by` an `IndexSet` element cannot.
#[derive(Clone, Default)]
struct GroupSubtree<'s, 't> {
  /// References whose group ends exactly at this node.
  locals: fx::IndexMap<RefKey<'s, 't>, LocalEntry<'t>>,
  /// References pointing at an ellipsis (`g...`) somewhere inside this node's group.
  locals_in_ellipsis: fx::IndexMap<RefKey<'s, 't>, LocalEntry<'t>>,
  name_to_child: fx::IndexMap<GroupStep<'s, 't>, GroupSubtree<'s, 't>>,
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
  /// Check the grouped body, returning the first violation.
  pub fn check_usages<'g>(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    body: &IExpressionGE<'s, 't, 'g>,
  ) -> Result<(), ICompileErrorT<'s, 't>> {
    let mut tree = GroupSubtree::default();
    let mut next_held = 0;
    self.check_ge(coutputs, body, &mut tree, &mut next_held)
  }

  fn check_ge<'g>(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    node: &IExpressionGE<'s, 't, 'g>,
    tree: &mut GroupSubtree<'s, 't>,
    next_held: &mut u32,
  ) -> Result<(), ICompileErrorT<'s, 't>> {
    match node {
      IExpressionGE::LetNormal { expr, bind, .. } => {
        self.check_ge(coutputs, expr, tree, next_held)?;
        if let Some((local, group)) = bind {
          register(tree, RefKey::Named(*local), group);
        }
        Ok(())
      }
      IExpressionGE::FunctionCall { args, mut_effects, call, .. } => {
        // Walk each grouped argument, registering its held register (if any) as soon as it is
        // produced, so a later sibling argument's churn can invalidate it.
        let mut held_keys = vec![];
        for (i, arg) in args.iter().enumerate() {
          self.check_ge(coutputs, arg, tree, next_held)?;
          if let Some((group, range)) = held_register_of(&call.args[i], arg) {
            let key = RefKey::Held(*next_held);
            *next_held += 1;
            register(tree, key, group);
            held_keys.push((key, range));
          }
        }
        // A named-local reference used as an argument.
        for typed_arg in call.args {
          if let Some((local, range)) = arg_ref_use(typed_arg) {
            if is_use_after_churn(tree, RefKey::Named(local)) {
              return Err(BorrowErrorKind::UseAfterChurn { local }.at(self, range));
            }
          }
        }
        for (key, range) in &held_keys {
          if is_use_after_churn(tree, *key) {
            return Err(BorrowErrorKind::UseAfterChurnTemporary.at(self, *range));
          }
        }
        if let Some(fact) = self.joint_facts(coutputs, call, args).first() {
          return Err(self.joint_error(fact));
        }
        for path in mut_effects {
          churn(tree, &path.steps, path.effecting_node_loc);
        }
        Ok(())
      }
      IExpressionGE::If { condition, then_call, else_call, then_diverges, else_diverges, .. } => {
        self.check_ge(coutputs, condition, tree, next_held)?;
        let mut then_tree = tree.clone();
        self.check_ge(coutputs, then_call, &mut then_tree, next_held)?;
        let mut else_tree = tree.clone();
        self.check_ge(coutputs, else_call, &mut else_tree, next_held)?;
        merge(tree, &then_tree, &else_tree, *then_diverges, *else_diverges);
        Ok(())
      }
      IExpressionGE::While { body, mut_effects, .. } => {
        for path in mut_effects {
          churn(tree, &path.steps, path.effecting_node_loc);
        }
        self.check_ge(coutputs, body, tree, next_held)
      }
      // Every other node has nothing for the checker itself; walk its children in evaluation order.
      other => {
        for child in other.children() {
          self.check_ge(coutputs, child, tree, next_held)?;
        }
        Ok(())
      }
    }
  }

  /// The joint-argument facts at a call: a borrow into a moved argument, and aliasing borrows into
  /// distinct mutated groups. Argument identities and ranges come from the typed `call`; each
  /// argument's group path comes from its grouped result. Empty when the callee cannot be resolved.
  fn joint_facts<'g>(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    call: &FunctionCallTE<'s, 't>,
    grouped_args: &'g [IExpressionGE<'s, 't, 'g>],
  ) -> Vec<JointFact<'s, 't>> {
    let Some(callee) = self.resolve_callee(coutputs, call) else {
      return vec![];
    };
    let places: Vec<Option<(IVarNameT<'s, 't>, Vec<GroupStep<'s, 't>>, RangeS<'s>)>> = call
      .args
      .iter()
      .enumerate()
      .map(|(i, arg)| {
        let root = place_root_local(arg)?;
        let group = result_borrow_group(grouped_args.get(i)?.result())?;
        let range = expr_range(arg)?;
        Some((root, flatten(group), range))
      })
      .collect();
    let moves: Vec<Option<IVarNameT<'s, 't>>> = call.args.iter().map(moved_local).collect();

    let mut facts = vec![];
    for (i, place) in places.iter().enumerate() {
      if let Some((root_i, _, range_i)) = place {
        for (j, moved) in moves.iter().enumerate() {
          if i != j {
            if let Some(moved) = moved {
              if root_i == moved {
                facts.push(JointFact::BorrowIntoMoved {
                  local: *moved,
                  borrow_arg: i,
                  move_arg: j,
                  range: *range_i,
                });
              }
            }
          }
        }
      }
    }

    let param_runes: Vec<Option<IRuneS<'s>>> = callee.params.iter().map(param_group_rune).collect();
    let mutated: Vec<IRuneS<'s>> = callee
      .effects
      .iter()
      .filter_map(|e| match e {
        EffectS::Mut(gs) => effect_root_rune(gs),
        _ => None,
      })
      .collect();
    for i in 0..call.args.len() {
      for j in (i + 1)..call.args.len() {
        if let (Some((root_i, path_i, range_i)), Some((_, path_j, _))) = (&places[i], &places[j]) {
          if let (Some(ri), Some(rj)) =
            (param_runes.get(i).copied().flatten(), param_runes.get(j).copied().flatten())
          {
            if ri != rj && (mutated.contains(&ri) || mutated.contains(&rj)) && paths_alias(path_i, path_j) {
              if let (Some(ga), Some(gb)) = (rune_name(ri), rune_name(rj)) {
                facts.push(JointFact::AliasingDisjointMut {
                  local: *root_i,
                  arg_a: i,
                  arg_b: j,
                  group_a: ga,
                  group_b: gb,
                  range: *range_i,
                });
              }
            }
          }
        }
      }
    }
    facts
  }

  /// Build the compile error for a joint-argument fact.
  fn joint_error(&self, fact: &JointFact<'s, 't>) -> ICompileErrorT<'s, 't> {
    match fact {
      JointFact::BorrowIntoMoved { local, borrow_arg, move_arg, range } => {
        BorrowErrorKind::BorrowIntoMovedArgument {
          local: *local,
          borrow_arg: *borrow_arg,
          move_arg: *move_arg,
        }
        .at(self, *range)
      }
      JointFact::AliasingDisjointMut { local, arg_a, arg_b, group_a, group_b, range } => {
        BorrowErrorKind::AliasingIntoDisjointMutGroups {
          local: *local,
          arg_a: *arg_a,
          arg_b: *arg_b,
          group_a: *group_a,
          group_b: *group_b,
        }
        .at(self, *range)
      }
    }
  }
}

/// A named-local reference used as a value argument (`local` or `*local`): the local and its range.
// VLOOOOK: Option return — needs VOPT approval or removal
fn arg_ref_use<'s, 't>(arg: &ExpressionTE<'s, 't>) -> Option<(IVarNameT<'s, 't>, RangeS<'s>)> {
  let inner = match arg {
    ExpressionTE::Deref(d) => &d.inner,
    other => other,
  };
  if let ExpressionTE::LocalLookup(l) = inner {
    return Some((l.local_variable.name, l.range));
  }
  None
}

/// If an argument is an unnamed held reference — a temporary whose grouped result is a borrow, not a
/// named-local use already covered by `arg_ref_use` — its group and the range to diagnose at.
// VLOOOOK: Option return — needs VOPT approval or removal
fn held_register_of<'a, 's, 't, 'g>(
  typed_arg: &ExpressionTE<'s, 't>,
  arg_g: &'a IExpressionGE<'s, 't, 'g>,
) -> Option<(&'a GroupExprG<'s, 't>, RangeS<'s>)> {
  if arg_ref_use(typed_arg).is_some() {
    return None;
  }
  let group = result_borrow_group(arg_g.result())?;
  let range = held_range(typed_arg)?;
  Some((group, range))
}

/// The group a result type borrows into, if it is a borrow reference.
// VLOOOOK: Option return — needs VOPT approval or removal
fn result_borrow_group<'a, 's, 't>(kind: &'a KindGT<'s, 't>) -> Option<&'a GroupExprG<'s, 't>> {
  match kind {
    KindGT::BorrowRef(b) => Some(&b.group),
    _ => None,
  }
}

/// Register a live reference under each group its `GroupExprG` names: a union under each member, an
/// ellipsis in the target node's `locals_in_ellipsis`, anything else in the target node's `locals`.
fn register<'s, 't>(root: &mut GroupSubtree<'s, 't>, key: RefKey<'s, 't>, group: &GroupExprG<'s, 't>) {
  match group {
    GroupExprG::Union { members } => {
      for m in members {
        register(root, key, m);
      }
    }
    GroupExprG::Ellipsis { base } => {
      let node = navigate(root, &flatten(base));
      node.locals_in_ellipsis.entry(key).or_default();
    }
    other => {
      let node = navigate(root, &flatten(other));
      node.locals.entry(key).or_default();
    }
  }
}

/// Walk (creating as needed) to the subtree at `steps`.
fn navigate<'a, 's, 't>(
  node: &'a mut GroupSubtree<'s, 't>,
  steps: &[GroupStep<'s, 't>],
) -> &'a mut GroupSubtree<'s, 't> {
  let mut cur = node;
  for step in steps {
    cur = cur.name_to_child.entry(step.clone()).or_default();
  }
  cur
}

/// Apply a `mut(g)` churn: `steps` is `g`'s path (an ellipsis in the effect is already collapsed, as
/// `mut(g...) ≡ mut(g)`). Invalidate every reference into `g`'s `Elements` descendants, and every
/// ellipsis reference at, below, or above `g`. A reference to `g` itself, or to an inline member,
/// survives.
fn churn<'s, 't>(root: &mut GroupSubtree<'s, 't>, steps: &[GroupStep<'s, 't>], loc: LocT<'t>) {
  match steps.split_first() {
    // A proper ancestor of the churned group: its ellipsis references overlap `g`, so they die.
    Some((first, rest)) => {
      stamp(&mut root.locals_in_ellipsis, loc);
      if let Some(child) = root.name_to_child.get_mut(first) {
        churn(child, rest, loc);
      }
    }
    // The churned group itself.
    None => invalidate_from_churned(root, loc, false),
  }
}

/// Invalidate everything at and below the churned group: its own ellipsis references and every
/// descendant's, plus the `locals` of any descendant reached by crossing an `Elements` edge (the only
/// independently-destructible child group). The churned group's own `locals` survive.
fn invalidate_from_churned<'s, 't>(
  node: &mut GroupSubtree<'s, 't>,
  loc: LocT<'t>,
  crossed_elements: bool,
) {
  stamp(&mut node.locals_in_ellipsis, loc);
  if crossed_elements {
    stamp(&mut node.locals, loc);
  }
  for (step, child) in node.name_to_child.iter_mut() {
    let crossed = crossed_elements || matches!(step, GroupStep::Elements);
    invalidate_from_churned(child, loc, crossed);
  }
}

/// Stamp `invalidated_by` on every not-yet-invalidated entry.
fn stamp<'s, 't>(entries: &mut fx::IndexMap<RefKey<'s, 't>, LocalEntry<'t>>, loc: LocT<'t>) {
  for entry in entries.values_mut() {
    if entry.invalidated_by.is_none() {
      entry.invalidated_by = Some(loc);
    }
  }
}

/// Whether any registered entry for `key` (at any group it points into) has been invalidated.
fn is_use_after_churn<'s, 't>(node: &GroupSubtree<'s, 't>, key: RefKey<'s, 't>) -> bool {
  node.locals.get(&key).is_some_and(|e| e.invalidated_by.is_some())
    || node.locals_in_ellipsis.get(&key).is_some_and(|e| e.invalidated_by.is_some())
    || node.name_to_child.values().any(|c| is_use_after_churn(c, key))
}

/// Merge two branch states back into the pre-branch tree: a reference is invalidated after the `if`
/// iff a non-diverging branch invalidated it. `orig` still holds the pre-branch invalidation, and the
/// branch trees share its structure (they only appended), so the merge walks them in lockstep;
/// branch-local registrations fall away.
fn merge<'s, 't>(
  orig: &mut GroupSubtree<'s, 't>,
  then_tree: &GroupSubtree<'s, 't>,
  else_tree: &GroupSubtree<'s, 't>,
  then_diverges: bool,
  else_diverges: bool,
) {
  merge_entries(&mut orig.locals, &then_tree.locals, &else_tree.locals, then_diverges, else_diverges);
  merge_entries(
    &mut orig.locals_in_ellipsis,
    &then_tree.locals_in_ellipsis,
    &else_tree.locals_in_ellipsis,
    then_diverges,
    else_diverges,
  );
  for (step, child) in orig.name_to_child.iter_mut() {
    let then_child = then_tree.name_to_child.get(step).expect("branch dropped a group edge");
    let else_child = else_tree.name_to_child.get(step).expect("branch dropped a group edge");
    merge(child, then_child, else_child, then_diverges, else_diverges);
  }
}

/// Merge one node's entries: an entry stays invalidated iff a non-diverging branch invalidated it; if
/// both branches diverge, the pre-branch value stands.
fn merge_entries<'s, 't>(
  orig: &mut fx::IndexMap<RefKey<'s, 't>, LocalEntry<'t>>,
  then_m: &fx::IndexMap<RefKey<'s, 't>, LocalEntry<'t>>,
  else_m: &fx::IndexMap<RefKey<'s, 't>, LocalEntry<'t>>,
  then_diverges: bool,
  else_diverges: bool,
) {
  for (key, base) in orig.iter_mut() {
    let then_inv =
      if then_diverges { None } else { then_m.get(key).expect("branch dropped an entry").invalidated_by };
    let else_inv =
      if else_diverges { None } else { else_m.get(key).expect("branch dropped an entry").invalidated_by };
    base.invalidated_by =
      if then_diverges && else_diverges { base.invalidated_by } else { then_inv.or(else_inv) };
  }
}
