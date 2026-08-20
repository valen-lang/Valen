//! Rung-2 use-after-churn: a flow-sensitive walk over a finished function body.
//!
//! A reference bound to a runtime-sized-array *element* points into a **child group**. A call that
//! churns the parent group may move or delete that element, so using such a reference afterward is
//! an error. A reference to the whole array or to an inline member is in the parent group and is
//! never invalidated. The walk threads two facts through the body in evaluation order: which locals
//! hold a child-group reference, and which of those a preceding churn may have invalidated.

use crate::typing::ast::ast::FunctionDefinitionT;
use crate::typing::ast::expressions::{ExpressionTE, FunctionCallTE};
use crate::typing::borrow_checker::borrow_error::BorrowErrorKind;
use crate::typing::borrow_checker::call_check::{is_mut_target, param_group_name};
use crate::typing::borrow_checker::place_path::{place_path, PlacePath};
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::env::function_environment_t::LocalVariable;
use crate::typing::names::names::{IdValT, IVarNameT};
use crate::typing::types::types::KindT;
use crate::utils::range::RangeS;
use std::collections::{HashMap, HashSet};
use std::mem::{replace, take};

/// Identity key for a local — its arena address, stable for the `'t` lifetime.
fn local_key(local: &LocalVariable) -> usize {
  local as *const LocalVariable as usize
}

struct Liveness<'s, 'ctx, 't, 'a> {
  compiler: &'a Compiler<'s, 'ctx, 't>,
  coutputs: &'a CompilerOutputs<'s, 't>,
  /// Where a violation is reported. Exact call/use ranges are deferred (source-ranges rule), so this
  /// is the enclosing function's range, matching the joint-argument check.
  range: RangeS<'s>,
  /// Locals bound to a child-group (array-element) reference, keyed by identity, with their place so
  /// the churned array's root can be matched.
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
  coutputs: &CompilerOutputs<'s, 't>,
  compiler: &Compiler<'s, 'ctx, 't>,
  range: RangeS<'s>,
) -> Result<(), ICompileErrorT<'s, 't>> {
  let mut liveness = Liveness {
    compiler,
    coutputs,
    range,
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

  /// Invalidate every live element reference rooted in an array the callee churns. A callee churns
  /// the array handed to each parameter whose group it declares `mut`; a reference into a *different*
  /// array, or into an array handed to a non-`mut` parameter, is untouched.
  fn apply_churn(&mut self, call: &'t FunctionCallTE<'s, 't>) {
    let churned_roots = self.churned_roots(call);
    if churned_roots.is_empty() {
      return;
    }
    let newly_invalid: Vec<usize> = self
      .element_refs
      .iter()
      .filter(|(_, path)| churned_roots.iter().any(|root| path.root == *root))
      .map(|(key, _)| *key)
      .collect();
    self.invalidated.extend(newly_invalid);
  }

  /// The root local of each array the callee churns (a `mut`-group parameter's argument).
  fn churned_roots(&self, call: &'t FunctionCallTE<'s, 't>) -> Vec<IVarNameT<'s, 't>> {
    let template_id_val =
      Compiler::get_function_template(self.compiler.typing_interner, call.callable.id);
    let template_id = self.compiler.typing_interner.intern_id(IdValT {
      package_coord: template_id_val.package_coord,
      init_steps: template_id_val.init_steps,
      local_name: template_id_val.local_name,
    });
    let callee = match self.coutputs.peek_postparsed_function(template_id) {
      Some(callee) => callee,
      None => return Vec::new(),
    };
    let mut roots = Vec::new();
    let arg_count = call.args.len().min(callee.params.len());
    for i in 0..arg_count {
      let Some(group) = param_group_name(&callee.params[i]) else {
        continue;
      };
      if !is_mut_target(callee.effects, group) {
        continue;
      }
      if let Some(path) = place_path(&call.args[i]) {
        roots.push(path.root);
      }
    }
    roots
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
