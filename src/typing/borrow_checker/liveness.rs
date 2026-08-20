//! Rung-2 use-after-churn: a flow-sensitive walk over a finished function body.
//!
//! A reference bound to a runtime-sized-array *element* points into a **child group**. A call that
//! churns the parent group may move or delete that element, so using such a reference afterward is
//! an error. A reference to the whole array or to an inline member is in the parent group and is
//! never invalidated. The walk threads two facts through the body in evaluation order: which locals
//! hold a child-group reference, and which of those a preceding churn may have invalidated.

use crate::interner::StrI;
use crate::postparsing::ast::FunctionS;
use crate::postparsing::names::IVarDeclarationNameS;
use crate::typing::ast::ast::FunctionDefinitionT;
use crate::typing::ast::expressions::{ExpressionTE, FunctionCallTE};
use crate::typing::borrow_checker::borrow_error::BorrowErrorKind;
use crate::typing::borrow_checker::call_check::{is_mut_target, param_group_name, resolve_callee};
use crate::typing::borrow_checker::place_path::{place_path, PlacePath};
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::env::function_environment_t::LocalVariable;
use crate::typing::names::names::IVarNameT;
use crate::typing::types::types::KindT;
use crate::utils::range::RangeS;
use std::collections::{HashMap, HashSet};
use std::mem::{replace, take};

/// Identity key for a local — its arena address, stable for the `'t` lifetime.
fn local_key(local: &LocalVariable) -> usize {
  local as *const LocalVariable as usize
}

/// The caller-side group a root local belongs to, for matching a churn against a live borrow.
///
/// Two references are in the same group — so a churn of one invalidates child-group references of
/// the other (common-group aliasing) — exactly when their roots resolve to the same `RootGroup`.
/// A root that is a caller parameter declared `in r` resolves to that group rune (`ParamRune`), so
/// two sibling parameters sharing one rune alias; anything else (a fresh local, or a parameter with
/// no declared group) is its own group (`Local`), disjoint by default. Compared by name/identity —
/// no `GroupB` is minted.
#[derive(Clone, Copy, PartialEq, Eq)]
enum RootGroup<'s, 't> {
  ParamRune(StrI<'s>),
  Local(IVarNameT<'s, 't>),
}

struct Liveness<'s, 'ctx, 't, 'a> {
  compiler: &'a Compiler<'s, 'ctx, 't>,
  coutputs: &'a CompilerOutputs<'s, 't>,
  /// Where a violation is reported. Exact call/use ranges are deferred (source-ranges rule), so this
  /// is the enclosing function's range, matching the joint-argument check.
  range: RangeS<'s>,
  /// Caller-parameter code-name → its declared group-rune name, for resolving a root local to its
  /// `RootGroup`. Built once from the enclosing function's parameters.
  param_groups: HashMap<StrI<'s>, StrI<'s>>,
  /// Locals bound to a child-group (array-element) reference, keyed by identity, with their place so
  /// the churned array's group can be matched.
  element_refs: HashMap<usize, PlacePath<'s, 't>>,
  /// Element refs a preceding churn may have invalidated.
  invalidated: HashSet<usize>,
  /// Whether a use of an invalidated reference is reported. Cleared while iterating a loop to a
  /// fixpoint, so the silent passes accumulate invalidations without reporting a use twice.
  reporting: bool,
}

/// Walk `function`'s finished body and reject a use of a child-group reference after a churn.
pub fn check_use_after_churn<'s, 'ctx, 't>(
  function: &'t FunctionDefinitionT<'s, 't>,
  function_s: &'s FunctionS<'s>,
  coutputs: &CompilerOutputs<'s, 't>,
  compiler: &Compiler<'s, 'ctx, 't>,
  range: RangeS<'s>,
) -> Result<(), ICompileErrorT<'s, 't>> {
  let mut param_groups = HashMap::new();
  for param in function_s.params {
    if let (IVarDeclarationNameS::CodeVarName(param_name), Some(group)) =
      (&param.name, param_group_name(param))
    {
      param_groups.insert(param_name.name, group);
    }
  }
  let mut liveness = Liveness {
    compiler,
    coutputs,
    range,
    param_groups,
    element_refs: HashMap::new(),
    invalidated: HashSet::new(),
    reporting: true,
  };
  liveness.walk(&function.body)
}

impl<'s, 'ctx, 't, 'a> Liveness<'s, 'ctx, 't, 'a> {
  fn walk(&mut self, expr: &'t ExpressionTE<'s, 't>) -> Result<(), ICompileErrorT<'s, 't>> {
    match *expr {
      // A binding whose initializer names a child-group place records a new element reference. Walk
      // the initializer first (a use nested in it is checked against the current state), then record.
      ExpressionTE::LetNormal(let_normal) => {
        self.walk(&let_normal.expr)?;
        self.record_if_element_ref(let_normal.variable, &let_normal.expr);
        Ok(())
      }
      ExpressionTE::LetAndLend(let_and_lend) => {
        self.walk(&let_and_lend.expr)?;
        self.record_if_element_ref(let_and_lend.variable, &let_and_lend.expr);
        Ok(())
      }
      // A call: evaluate its arguments (a use of an already-invalidated ref there is the violation),
      // then churn. A churn may move or delete every child-group element, so every live element ref
      // is invalidated afterward.
      ExpressionTE::FunctionCall(call) => {
        for arg in call.args {
          self.walk(arg)?;
        }
        self.apply_churn(call);
        Ok(())
      }
      // A mention of a local: if it is a child-group reference a churn has invalidated, reject it.
      ExpressionTE::LocalLookup(local_lookup) => {
        let key = local_key(local_lookup.local_variable);
        if self.reporting && self.element_refs.contains_key(&key) && self.invalidated.contains(&key) {
          return Err(
            BorrowErrorKind::UseAfterChurn { local: local_lookup.local_variable.name }
              .at(self.compiler, self.range),
          );
        }
        Ok(())
      }
      // Structural nodes: recurse into children in evaluation order.
      ExpressionTE::Consecutor(consecutor) => {
        for inner in consecutor.exprs {
          self.walk(inner)?;
        }
        Ok(())
      }
      ExpressionTE::Block(block) => self.walk(&block.inner),
      ExpressionTE::Discard(discard) => self.walk(&discard.expr),
      ExpressionTE::Return(ret) => self.walk(&ret.source_expr),
      // Invalidation is a may-fact: each arm is walked from the pre-`if` state, and the sets union
      // at the join. An arm that diverges (returns/breaks) never reaches the code after the `if`, so
      // it contributes nothing.
      ExpressionTE::If(if_expr) => {
        self.walk(&if_expr.condition)?;
        let pre_if = self.invalidated.clone();
        self.walk(&if_expr.then_call)?;
        let then_invalidated = replace(&mut self.invalidated, pre_if.clone());
        self.walk(&if_expr.else_call)?;
        let else_invalidated = take(&mut self.invalidated);
        let mut joined = pre_if;
        if !matches!(if_expr.then_call.result(), KindT::Never(_)) {
          joined.extend(then_invalidated);
        }
        if !matches!(if_expr.else_call.result(), KindT::Never(_)) {
          joined.extend(else_invalidated);
        }
        self.invalidated = joined;
        Ok(())
      }
      // A loop's back-edge carries a churn from one iteration into the next, so a reference used at
      // the top of the body may already be invalidated. Iterate the body to a fixpoint: walk it
      // silently, union its end state back into the head, and repeat until the head stops growing
      // (monotone and non-cascading, so it settles in a couple of passes). Then walk once more from
      // that head, reporting. A reference created fresh inside the body starts each iteration
      // uninvalidated, so it stays live; a reference created before the loop does not.
      ExpressionTE::While(while_expr) => {
        let body = &while_expr.block.inner;
        let incoming = self.invalidated.clone();
        let outer_reporting = self.reporting;
        self.reporting = false;
        let mut head = incoming.clone();
        loop {
          self.invalidated = head.clone();
          self.walk(body)?;
          let body_end = take(&mut self.invalidated);
          let next_head: HashSet<usize> = incoming.union(&body_end).copied().collect();
          if next_head == head {
            break;
          }
          head = next_head;
        }
        self.reporting = outer_reporting;
        self.invalidated = head.clone();
        self.walk(body)?;
        // After the loop, a reference the body churns is possibly-invalidated (the loop may have run).
        self.invalidated = head;
        Ok(())
      }
      ExpressionTE::Mutate(mutate) => {
        self.walk(&mutate.destination_expr)?;
        self.walk(&mutate.source_expr)
      }
      ExpressionTE::Deref(deref) => self.walk(&deref.inner),
      ExpressionTE::MemberLookup(member_lookup) => self.walk(&member_lookup.struct_expr),
      ExpressionTE::RuntimeSizedArrayLookup(array_lookup) => {
        self.walk(&array_lookup.array_expr)?;
        self.walk(&array_lookup.index_expr)
      }
      // Everything else is treated as a leaf for now. Nodes here that carry child expressions are a
      // known coverage gap, extended with a red test when a reachable use can nest there.
      _ => Ok(()),
    }
  }

  /// Invalidate every live element reference whose array is in a group the callee churns. A callee
  /// churns the group handed to each parameter whose group it declares `mut`; a reference into an
  /// array in a *different* group, or into an array handed to a non-`mut` parameter, is untouched.
  /// Matching is by **group**, not by root local: a churn through one parameter invalidates element
  /// references rooted in a *sibling* parameter that shares the same group (common-group aliasing),
  /// not only references rooted in the churned argument itself.
  fn apply_churn(&mut self, call: &'t FunctionCallTE<'s, 't>) {
    let churned = self.churned_groups(call);
    if churned.is_empty() {
      return;
    }
    let newly_invalid: Vec<usize> = self
      .element_refs
      .iter()
      .filter(|(_, path)| churned.iter().any(|group| self.root_group(path.root) == *group))
      .map(|(key, _)| *key)
      .collect();
    self.invalidated.extend(newly_invalid);
  }

  /// The caller-side group a root local belongs to (see `RootGroup`).
  fn root_group(&self, root: IVarNameT<'s, 't>) -> RootGroup<'s, 't> {
    if let IVarNameT::Local(local) = root {
      if let Some(group) = self.param_groups.get(&local.name) {
        return RootGroup::ParamRune(*group);
      }
    }
    RootGroup::Local(root)
  }

  /// The caller-side group of each array the callee churns (a `mut`-group parameter's argument).
  fn churned_groups(&self, call: &'t FunctionCallTE<'s, 't>) -> Vec<RootGroup<'s, 't>> {
    let callee = match resolve_callee(call, self.coutputs, self.compiler) {
      Some(callee) => callee,
      None => return Vec::new(),
    };
    let mut groups = Vec::new();
    let arg_count = call.args.len().min(callee.params.len());
    for i in 0..arg_count {
      let Some(group) = param_group_name(&callee.params[i]) else {
        continue;
      };
      if !is_mut_target(callee.effects, group) {
        continue;
      }
      if let Some(path) = place_path(&call.args[i]) {
        groups.push(self.root_group(path.root));
      }
    }
    groups
  }

  /// Record `variable` as a child-group reference if `init` names an array element.
  fn record_if_element_ref(
    &mut self,
    variable: &'t LocalVariable<'s, 't>,
    init: &'t ExpressionTE<'s, 't>,
  ) {
    if let Some(path) = place_path(init) {
      if path.is_child_group() {
        let key = local_key(variable);
        // A fresh binding is a new borrow — not invalidated, even if a churn invalidated a prior
        // binding of the same local (e.g. a reference re-taken each loop iteration).
        self.invalidated.remove(&key);
        self.element_refs.insert(key, path);
      }
    }
  }
}
