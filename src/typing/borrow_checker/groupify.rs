//! Phase 1: `groupify_function` walks the typed body and produces the grouped `IExpressionGE`.
//!
//! For every reference-typed local it records the group its referent lives in; at every call it
//! records the groups the call churns and its joint-argument facts; each loop node carries its body's
//! aggregated churns. Parameter groups come from `make_kind_g`, which reads a group at every depth of
//! the written type — so a nested `&Opt<&Thing in g> in d` is read once, here.

use crate::interner::StrI;
use crate::postparsing::ast::{FunctionS, ParameterS};
use crate::postparsing::names::{IRuneS, IVarDeclarationNameS};
use crate::postparsing::rules::types::{EffectS, GroupS, ITypeST, RegionS};
use crate::typing::ast::ast::FunctionDefinitionT;
use crate::typing::ast::expressions::{ExpressionTE, FunctionCallTE};
use crate::typing::borrow_checker::borrow_types::{
  group_expr_from_group_s, subst_group_expr, BorrowRefGT, GroupExprG, KindGT,
};
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::borrow_checker::grouped_ast::{
  split_unions, IExpressionGE, MutEffectPath,
};
use crate::typing::compiler::Compiler;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::names::names::{IVarNameT, IdT, IdValT, INameT};
use crate::typing::types::types::{KindT, VoidT};
use crate::utils::fx::IndexMap;
use crate::utils::range::RangeS;
use bumpalo::Bump;

/// Phase-1 state: the reference-typed locals seen so far and where each points, plus the function
/// being grouped (for parameter types).
struct GCtx<'s, 't> {
  /// Each in-scope local's full grouped type (its `let`'s bound value, with groups at every depth). A
  /// `LocalLookup` borrows it, a `Deref`/`Unlet` reads through it, so nested groups flow from here.
  locals: Vec<(IVarNameT<'s, 't>, KindGT<'s, 't>)>,
  function_s: &'s FunctionS<'s>,
  function_t: &'t FunctionDefinitionT<'s, 't>,
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
  /// Build the grouped body for one function, or an error if a borrow's group is underivable.
  pub fn groupify_function<'g>(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    function_s: &'s FunctionS<'s>,
    function_t: &'t FunctionDefinitionT<'s, 't>,
    arena: &'g Bump,
  ) -> Result<IExpressionGE<'s, 't, 'g>, ICompileErrorT<'s, 't>> {
    let mut ctx = GCtx { locals: vec![], function_s, function_t };
    let body = self.groupify(coutputs, &function_t.body, &mut ctx, arena);
    Ok(body)
  }

  /// Rebuild one typed expression as its grouped mirror, allocating children in `arena`. The result
  /// `KindGT` and the checker payloads (bind, call churns/uses/joint, loop churns, branch divergence)
  /// are filled in; everything else mirrors `ExpressionTE` structurally.
  fn groupify<'g>(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    expr: &ExpressionTE<'s, 't>,
    ctx: &mut GCtx<'s, 't>,
    arena: &'g Bump,
  ) -> IExpressionGE<'s, 't, 'g> {
    // Post-order: build the grouped children first, then compose this node's result `KindGT` from
    // them (and from tracked locals / parameters / the callee signature for the leaves).
    let node = match expr {
      ExpressionTE::LetAndLend(e) => {
        let child = arena.alloc(self.groupify(coutputs, &e.expr, ctx, arena));
        // `&{ let tmp = e; tmp }`: a fresh temporary local bound to `e`, then lent. Track the temp so a
        // later use resolves, and produce a borrow of it grouped `Local(tmp)`.
        ctx.locals.push((e.variable.name, child.result().clone()));
        let result = ref_kind_g(GroupExprG::Local(e.variable.name), child.result().clone());
        IExpressionGE::LetAndLend { result, expr: child }
      }
      ExpressionTE::LockWeak(e) => {
        let inner_expr = arena.alloc(self.groupify(coutputs, &e.inner_expr, ctx, arena));
        // A weak lock yields `Opt<&T>`; the nested borrow's group is a deferred (weak) feature, so the
        // groupless structure trips the `has_empty_group` panic in `node_result`.
        let result = self.make_kind_g_groupless(expr.result());
        IExpressionGE::LockWeak { result, inner_expr }
      }
      ExpressionTE::BorrowToWeak(e) => {
        let inner_expr = arena.alloc(self.groupify(coutputs, &e.inner_expr, ctx, arena));
        let result = self.make_kind_g_groupless(expr.result());
        IExpressionGE::BorrowToWeak { result, inner_expr }
      }
      ExpressionTE::LetNormal(l) => {
        let child = arena.alloc(self.groupify(coutputs, &l.expr, ctx, arena));
        // Track every local by its full grouped type (the bound value's), so a later lookup / move /
        // deref reads its groups at every depth. `bind` still carries the outer group for phase 2's
        // registration (only borrow bindings register a live reference).
        ctx.locals.push((l.variable.name, child.result().clone()));
        let bind = match child.result() {
          KindGT::BorrowRef(b) => Some((l.variable.name, b.group.clone())),
          _ => None,
        };
        IExpressionGE::LetNormal { result: void_kind_g(), expr: child, bind }
      }
      ExpressionTE::Unlet(u) => {
        // `^x` moves the local out — its result is the local's grouped type.
        IExpressionGE::Unlet { result: self.local_type(ctx, u.variable.name, u.variable.tyype) }
      }
      ExpressionTE::Discard(e) => {
        IExpressionGE::Discard { result: void_kind_g(), expr: arena.alloc(self.groupify(coutputs, &e.expr, ctx, arena)) }
      }
      ExpressionTE::If(if_te) => {
        let condition = arena.alloc(self.groupify(coutputs, &if_te.condition, ctx, arena));
        let then_call = arena.alloc(self.groupify(coutputs, &if_te.then_call, ctx, arena));
        let else_call = arena.alloc(self.groupify(coutputs, &if_te.else_call, ctx, arena));
        let then_diverges = matches!(if_te.then_call.result(), KindT::Never(_));
        let else_diverges = matches!(if_te.else_call.result(), KindT::Never(_));
        // Result is the non-diverging branch's grouped result.
        let result = if then_diverges { else_call.result().clone() } else { then_call.result().clone() };
        IExpressionGE::If { result, condition, then_call, else_call, then_diverges, else_diverges }
      }
      ExpressionTE::While(w) => {
        let body = arena.alloc(self.groupify(coutputs, &w.block.inner, ctx, arena));
        // The loop's churns are the closure of every call's churns inside its (already-grouped) body,
        // including nested loops — so phase 2 needs no fixpoint.
        let mut mut_effects = vec![];
        collect_subtree_churns(body, &mut mut_effects);
        IExpressionGE::While { result: self.make_kind_g_groupless(expr.result()), body, mut_effects }
      }
      ExpressionTE::Mutate(m) => IExpressionGE::Mutate {
        result: void_kind_g(),
        destination_expr: arena.alloc(self.groupify(coutputs, &m.destination_expr, ctx, arena)),
        source_expr: arena.alloc(self.groupify(coutputs, &m.source_expr, ctx, arena)),
      },
      ExpressionTE::Restackify(e) => IExpressionGE::Restackify {
        result: void_kind_g(),
        source_expr: arena.alloc(self.groupify(coutputs, &e.source_expr, ctx, arena)),
      },
      ExpressionTE::Return(r) => IExpressionGE::Return {
        result: self.make_kind_g_groupless(expr.result()),
        source_expr: arena.alloc(self.groupify(coutputs, &r.source_expr, ctx, arena)),
      },
      ExpressionTE::Break(_) => IExpressionGE::Break { result: self.make_kind_g_groupless(expr.result()) },
      ExpressionTE::Block(b) => {
        let inner = arena.alloc(self.groupify(coutputs, &b.inner, ctx, arena));
        let result = inner.result().clone();
        IExpressionGE::Block { result, inner }
      }
      ExpressionTE::Consecutor(c) => {
        let exprs = arena.alloc_slice_fill_iter(c.exprs.iter().map(|e| self.groupify(coutputs, e, ctx, arena)));
        let result = exprs.last().expect("consecutor with no expressions").result().clone();
        IExpressionGE::Consecutor { result, exprs }
      }
      ExpressionTE::StaticArrayFromValues(e) => IExpressionGE::StaticArrayFromValues {
        result: self.make_kind_g_groupless(expr.result()),
        elements: arena
          .alloc_slice_fill_iter(e.elements.iter().map(|x| self.groupify(coutputs, x, ctx, arena))),
      },
      ExpressionTE::ArraySize(e) => IExpressionGE::ArraySize {
        result: self.make_kind_g_groupless(expr.result()),
        array: arena.alloc(self.groupify(coutputs, &e.array, ctx, arena)),
      },
      ExpressionTE::IsSameInstance(e) => IExpressionGE::IsSameInstance {
        result: self.make_kind_g_groupless(expr.result()),
        left: arena.alloc(self.groupify(coutputs, &e.left, ctx, arena)),
        right: arena.alloc(self.groupify(coutputs, &e.right, ctx, arena)),
      },
      ExpressionTE::AsSubtype(e) => {
        let source_expr = arena.alloc(self.groupify(coutputs, &e.source_expr, ctx, arena));
        let result = self.cast_result(expr.result(), source_expr.result());
        IExpressionGE::AsSubtype { result, source_expr }
      }
      ExpressionTE::VoidLiteral(_) => IExpressionGE::VoidLiteral { result: void_kind_g() },
      ExpressionTE::ConstantInt(_) => IExpressionGE::ConstantInt { result: self.make_kind_g_groupless(expr.result()) },
      ExpressionTE::ConstantBool(_) => IExpressionGE::ConstantBool { result: self.make_kind_g_groupless(expr.result()) },
      ExpressionTE::ConstantStr(_) => IExpressionGE::ConstantStr { result: self.make_kind_g_groupless(expr.result()) },
      ExpressionTE::ConstantFloat(_) => IExpressionGE::ConstantFloat { result: self.make_kind_g_groupless(expr.result()) },
      ExpressionTE::ArgLookup(a) => {
        IExpressionGE::ArgLookup { result: self.arg_type(a.param_index as usize, ctx) }
      }
      ExpressionTE::ArrayLength(e) => IExpressionGE::ArrayLength {
        result: self.make_kind_g_groupless(expr.result()),
        array_expr: arena.alloc(self.groupify(coutputs, &e.array_expr, ctx, arena)),
      },
      ExpressionTE::InterfaceFunctionCall(e) => IExpressionGE::InterfaceFunctionCall {
        result: self.make_kind_g_groupless(expr.result()),
        args: arena.alloc_slice_fill_iter(e.args.iter().map(|a| self.groupify(coutputs, a, ctx, arena))),
      },
      ExpressionTE::ExternFunctionCall(e) => IExpressionGE::ExternFunctionCall {
        result: self.make_kind_g_groupless(expr.result()),
        args: arena.alloc_slice_fill_iter(e.args.iter().map(|a| self.groupify(coutputs, a, ctx, arena))),
      },
      ExpressionTE::FunctionCall(call) => {
        let args = arena
          .alloc_slice_fill_iter(call.args.iter().map(|a| self.groupify(coutputs, a, ctx, arena)));
        let (result, mut_effects) = match self.resolve_callee(coutputs, call) {
          Some(callee) => {
            // Bind each callee group rune to the caller-side group of the argument passed for it, read
            // off that argument's already-grouped result. Both the return group and the churn effects
            // cross into the caller frame through this substitution.
            let subst = arg_rune_subst(callee, args);
            (self.call_result_kind(call, callee, &subst), self.call_mut_effects(call, callee, &subst))
          }
          // No callee signature (e.g. a lambda call): groupless return — a borrow there is underivable
          // and trips the totality panic; a non-borrow return is fine.
          None => (self.make_kind_g_groupless(call.callable.return_type), vec![]),
        };
        IExpressionGE::FunctionCall { result, args, mut_effects, call }
      }
      ExpressionTE::Reinterpret(e) => {
        let child = arena.alloc(self.groupify(coutputs, &e.expr, ctx, arena));
        let result = self.cast_result(expr.result(), child.result());
        IExpressionGE::Reinterpret { result, expr: child }
      }
      ExpressionTE::Construct(e) => IExpressionGE::Construct {
        result: self.make_kind_g_groupless(expr.result()),
        args: arena.alloc_slice_fill_iter(e.args.iter().map(|a| self.groupify(coutputs, a, ctx, arena))),
      },
      ExpressionTE::NewRuntimeSizedArray(e) => IExpressionGE::NewRuntimeSizedArray {
        result: self.make_kind_g_groupless(expr.result()),
        capacity_expr: arena.alloc(self.groupify(coutputs, &e.capacity_expr, ctx, arena)),
      },
      ExpressionTE::StaticArrayFromCallable(e) => IExpressionGE::StaticArrayFromCallable {
        result: self.make_kind_g_groupless(expr.result()),
        generator: arena.alloc(self.groupify(coutputs, &e.generator, ctx, arena)),
      },
      ExpressionTE::DestroyStaticSizedArrayIntoFunction(e) => {
        IExpressionGE::DestroyStaticSizedArrayIntoFunction {
          result: void_kind_g(),
          array_expr: arena.alloc(self.groupify(coutputs, &e.array_expr, ctx, arena)),
          consumer: arena.alloc(self.groupify(coutputs, &e.consumer, ctx, arena)),
        }
      }
      ExpressionTE::DestroyStaticSizedArrayIntoLocals(e) => {
        IExpressionGE::DestroyStaticSizedArrayIntoLocals {
          result: void_kind_g(),
          expr: arena.alloc(self.groupify(coutputs, &e.expr, ctx, arena)),
        }
      }
      ExpressionTE::DestroyRuntimeSizedArray(e) => IExpressionGE::DestroyRuntimeSizedArray {
        result: void_kind_g(),
        array_expr: arena.alloc(self.groupify(coutputs, &e.array_expr, ctx, arena)),
      },
      ExpressionTE::RuntimeSizedArrayCapacity(e) => IExpressionGE::RuntimeSizedArrayCapacity {
        result: self.make_kind_g_groupless(expr.result()),
        array_expr: arena.alloc(self.groupify(coutputs, &e.array_expr, ctx, arena)),
      },
      ExpressionTE::PushRuntimeSizedArray(e) => IExpressionGE::PushRuntimeSizedArray {
        result: void_kind_g(),
        array_expr: arena.alloc(self.groupify(coutputs, &e.array_expr, ctx, arena)),
        new_element_expr: arena.alloc(self.groupify(coutputs, &e.new_element_expr, ctx, arena)),
      },
      ExpressionTE::PopRuntimeSizedArray(e) => IExpressionGE::PopRuntimeSizedArray {
        result: self.make_kind_g_groupless(expr.result()),
        array_expr: arena.alloc(self.groupify(coutputs, &e.array_expr, ctx, arena)),
      },
      ExpressionTE::InterfaceToInterfaceUpcast(e) => {
        let inner_expr = arena.alloc(self.groupify(coutputs, &e.inner_expr, ctx, arena));
        let result = self.cast_result(expr.result(), inner_expr.result());
        IExpressionGE::InterfaceToInterfaceUpcast { result, inner_expr }
      }
      ExpressionTE::Upcast(e) => {
        let inner_expr = arena.alloc(self.groupify(coutputs, &e.inner_expr, ctx, arena));
        let result = self.cast_result(expr.result(), inner_expr.result());
        IExpressionGE::Upcast { result, inner_expr }
      }
      ExpressionTE::Destroy(e) => {
        IExpressionGE::Destroy { result: void_kind_g(), expr: arena.alloc(self.groupify(coutputs, &e.expr, ctx, arena)) }
      }
      ExpressionTE::CopyPrim(e) => {
        IExpressionGE::CopyPrim { result: self.make_kind_g_groupless(expr.result()), inner: arena.alloc(self.groupify(coutputs, &e.inner, ctx, arena)) }
      }
      ExpressionTE::LocalLookup(l) => {
        // `x` yields a borrow of the local's storage, pointing at group `Local(x)`; its referent is the
        // local's grouped type, so nested groups flow through.
        let inner = self.local_type(ctx, l.local_variable.name, l.local_variable.tyype);
        IExpressionGE::LocalLookup { result: ref_kind_g(GroupExprG::Local(l.local_variable.name), inner) }
      }
      ExpressionTE::StaticSizedArrayLookup(e) => {
        let array_expr = arena.alloc(self.groupify(coutputs, &e.array_expr, ctx, arena));
        let index_expr = arena.alloc(self.groupify(coutputs, &e.index_expr, ctx, arena));
        let result = self.element_result(expr.result(), array_expr.result());
        IExpressionGE::StaticSizedArrayLookup { result, array_expr, index_expr }
      }
      ExpressionTE::RuntimeSizedArrayLookup(e) => {
        let array_expr = arena.alloc(self.groupify(coutputs, &e.array_expr, ctx, arena));
        let index_expr = arena.alloc(self.groupify(coutputs, &e.index_expr, ctx, arena));
        let result = self.element_result(expr.result(), array_expr.result());
        IExpressionGE::RuntimeSizedArrayLookup { result, array_expr, index_expr }
      }
      ExpressionTE::MemberLookup(e) => {
        let struct_expr = arena.alloc(self.groupify(coutputs, &e.struct_expr, ctx, arena));
        let result = self.member_result(expr.result(), struct_expr.result(), &e.member_name);
        IExpressionGE::MemberLookup { result, struct_expr }
      }
      ExpressionTE::Deref(d) => {
        let inner = arena.alloc(self.groupify(coutputs, &d.inner, ctx, arena));
        // Dereferencing peels one borrow: the result is the referent of the inner grouped borrow, with
        // its nested groups intact.
        let result = deref_kind_g(inner.result());
        IExpressionGE::Deref { result, inner }
      }
    };
    node
  }

  /// A local's grouped type. A `let`-bound local (the only kind whose referent groups matter — a
  /// reference binding) is tracked with groups at every depth. A local we tracked nothing for is a
  /// non-reference (or a compiler temporary), whose grouped type is exactly its plain typed type — it
  /// has no reference groups to record. If such an untracked local is nonetheless a reference, its
  /// groupless referent surfaces as an `Empty` and trips the totality panic (a real derivation gap).
  fn local_type(&self, ctx: &GCtx<'s, 't>, name: IVarNameT<'s, 't>, typed: KindT<'s, 't>) -> KindGT<'s, 't> {
    match ctx.locals.iter().find(|(n, _)| *n == name) {
      Some((_, k)) => k.clone(),
      None => self.make_kind_g_groupless(typed),
    }
  }

  /// A parameter's full grouped type, read from its written type at every depth (`make_kind_g`).
  fn arg_type(&self, i: usize, ctx: &GCtx<'s, 't>) -> KindGT<'s, 't> {
    let ps = ctx.function_s.params.get(i).expect("arg index out of range");
    let pt = ctx.function_t.header.params.get(i).expect("arg index out of range");
    self.make_kind_g(pt.tyype, &ps.tyype, Some(pt.name))
  }

  /// A call's grouped result: the callee's declared return type with its groups crossed into the caller
  /// frame via `subst` (each callee group rune → the caller group its argument was bound to). A callee
  /// with no return signature (e.g. a lambda's inferred return) yields the groupless structure — a
  /// borrow there is underivable and trips the totality panic; a non-borrow return is fine.
  fn call_result_kind(
    &self,
    call: &FunctionCallTE<'s, 't>,
    callee: &'s FunctionS<'s>,
    subst: &IndexMap<IRuneS<'s>, GroupExprG<'s, 't>>,
  ) -> KindGT<'s, 't> {
    match callee.maybe_return_type.as_ref() {
      Some(return_st) => {
        let return_kind_g = self.make_kind_g(call.callable.return_type, return_st, None);
        self.substitute_groups(&return_kind_g, subst)
      }
      None => self.make_kind_g_groupless(call.callable.return_type),
    }
  }

  /// A cast (upcast / reinterpret / as-subtype) keeps the operand's outer group and re-expresses the
  /// referent's structure. A nested borrow in that structure has no derivable group and trips the panic.
  fn cast_result(&self, cast_kind: KindT<'s, 't>, operand: &KindGT<'s, 't>) -> KindGT<'s, 't> {
    match (cast_kind, operand) {
      (KindT::BorrowRef(b), KindGT::BorrowRef(ob)) => {
        ref_kind_g(ob.group.clone(), self.make_kind_g_groupless(b.inner))
      }
      (other, _) => self.make_kind_g_groupless(other),
    }
  }

  /// An array-element access yields a borrow into the array's `Elements` child group.
  fn element_result(&self, access_kind: KindT<'s, 't>, array: &KindGT<'s, 't>) -> KindGT<'s, 't> {
    match (access_kind, array) {
      (KindT::BorrowRef(b), KindGT::BorrowRef(ab)) => ref_kind_g(
        GroupExprG::Elements { base: Box::new(ab.group.clone()) },
        self.make_kind_g_groupless(b.inner),
      ),
      (other, _) => self.make_kind_g_groupless(other),
    }
  }

  /// A member access yields a borrow into the struct's `Member` child group. A member that is itself a
  /// borrow (a closure capture, or a borrow-reference field) has no derivable group and trips the panic.
  fn member_result(
    &self,
    access_kind: KindT<'s, 't>,
    struct_val: &KindGT<'s, 't>,
    member_name_t: &IVarNameT<'s, 't>,
  ) -> KindGT<'s, 't> {
    match (access_kind, struct_val) {
      (KindT::BorrowRef(b), KindGT::BorrowRef(sb)) => {
        let member_name = match member_name_t {
          IVarNameT::Member(cv) => cv.imprecise_name.name,
          IVarNameT::Local(cv) => cv.imprecise_name.name,
          _ => panic!("vfail: member lookup with a non-member name"),
        };
        ref_kind_g(
          GroupExprG::Member { base: Box::new(sb.group.clone()), member_name },
          self.make_kind_g_groupless(b.inner),
        )
      }
      (other, _) => self.make_kind_g_groupless(other),
    }
  }

  /// The caller-side groups a call churns, from the callee's declared effects and parameter groups.
  fn call_mut_effects(
    &self,
    call: &FunctionCallTE<'s, 't>,
    callee: &'s FunctionS<'s>,
    subst: &IndexMap<IRuneS<'s>, GroupExprG<'s, 't>>,
  ) -> Vec<MutEffectPath<'s, 't>> {
    let mut paths = vec![];
    for effect in callee.effects {
      if let EffectS::Mut(gs) = effect {
        let caller = subst_group_expr(&group_expr_from_group_s(gs), subst);
        for steps in split_unions(&caller) {
          paths.push(MutEffectPath { effecting_node_loc: call.loct, steps });
        }
      }
    }
    paths
  }

  /// Resolve a call's callee to its scout `FunctionS` via the template id.
  // VLOOOOK: Option return — needs VOPT approval or removal
  pub(crate) fn resolve_callee(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    call: &FunctionCallTE<'s, 't>,
  ) -> Option<&'s FunctionS<'s>> {
    let inst_id = call.callable.id;
    let template_local = match inst_id.local_name {
      INameT::Function(fnt) => INameT::FunctionTemplate(fnt.template),
      _ => return None,
    };
    let template_id: &'t IdT<'s, 't> = self.typing_interner.intern_id(IdValT {
      package_coord: inst_id.package_coord,
      init_steps: inst_id.init_steps,
      local_name: template_local,
    });
    coutputs.peek_postparsed_function(template_id)
  }
}

/// Every churn inside a grouped subtree, for a loop's aggregated `mut_effects`: each call's
/// `mut_effects`, recursing through all children including nested loops.
fn collect_subtree_churns<'s, 't, 'g>(
  node: &IExpressionGE<'s, 't, 'g>,
  out: &mut Vec<MutEffectPath<'s, 't>>,
) {
  if let IExpressionGE::FunctionCall { mut_effects, .. } = node {
    out.extend(mut_effects.iter().cloned());
  }
  for child in node.children() {
    collect_subtree_churns(child, out);
  }
}

/// A borrow reference `KindGT` from a group and its referent type.
fn ref_kind_g<'s, 't>(group: GroupExprG<'s, 't>, inner: KindGT<'s, 't>) -> KindGT<'s, 't> {
  KindGT::BorrowRef(BorrowRefGT { group, inner: Box::new(inner) })
}

/// The `void` result `KindGT`, for statement-like nodes (a `let`, a mutate, a discard, a drop).
fn void_kind_g<'s, 't>() -> KindGT<'s, 't> {
  KindGT::Void(VoidT)
}

/// Peel one borrow: a `Deref`'s result is its operand's referent, carrying that referent's own groups.
fn deref_kind_g<'s, 't>(operand: &KindGT<'s, 't>) -> KindGT<'s, 't> {
  match operand {
    KindGT::BorrowRef(b) => (*b.inner).clone(),
    other => panic!("vfail: deref of a non-borrow: {:?}", other),
  }
}

/// The innermost local a place expression is rooted in.
// VLOOOOK: Option return — needs VOPT approval or removal
pub(crate) fn place_root_local<'s, 't>(expr: &ExpressionTE<'s, 't>) -> Option<IVarNameT<'s, 't>> {
  match expr {
    ExpressionTE::LocalLookup(l) => Some(l.local_variable.name),
    ExpressionTE::RuntimeSizedArrayLookup(a) => place_root_local(&a.array_expr),
    ExpressionTE::StaticSizedArrayLookup(a) => place_root_local(&a.array_expr),
    ExpressionTE::MemberLookup(m) => place_root_local(&m.struct_expr),
    ExpressionTE::Deref(d) => place_root_local(&d.inner),
    _ => None,
  }
}

/// The local an argument moves (`^local` lowers to an `Unlet`), if any.
// VLOOOOK: Option return — needs VOPT approval or removal
pub(crate) fn moved_local<'s, 't>(expr: &ExpressionTE<'s, 't>) -> Option<IVarNameT<'s, 't>> {
  match expr {
    ExpressionTE::Unlet(u) => Some(u.variable.name),
    _ => None,
  }
}

/// The source range to point a held-register diagnostic at: the argument's own range. A call points
/// at its call range (its result is the held reference); a place expression at its range.
// VLOOOOK: Option return — needs VOPT approval or removal
pub(crate) fn held_range<'s, 't>(arg: &ExpressionTE<'s, 't>) -> Option<RangeS<'s>> {
  match arg {
    ExpressionTE::FunctionCall(c) => c.range.first().copied(),
    _ => expr_range(arg),
  }
}

/// The source range of a place expression, for a diagnostic at the use site.
// VLOOOOK: Option return — needs VOPT approval or removal
pub(crate) fn expr_range<'s, 't>(expr: &ExpressionTE<'s, 't>) -> Option<RangeS<'s>> {
  match expr {
    ExpressionTE::LocalLookup(l) => Some(l.range),
    ExpressionTE::RuntimeSizedArrayLookup(a) => Some(a.range),
    ExpressionTE::StaticSizedArrayLookup(a) => Some(a.range),
    ExpressionTE::MemberLookup(m) => Some(m.range),
    ExpressionTE::Deref(d) => Some(d.range),
    _ => None,
  }
}

/// The group rune a borrow parameter declares (`&T in g`), if any.
// VLOOOOK: Option return — needs VOPT approval or removal
pub(crate) fn param_group_rune<'s>(param: &ParameterS<'s>) -> Option<IRuneS<'s>> {
  if let ITypeST::BorrowRef(st) = param.tyype {
    if let RegionS::Group(GroupS::Rune(ru)) = st.region {
      return Some(ru.rune);
    }
  }
  None
}

/// The root rune of an effect's group.
// VLOOOOK: Option return — needs VOPT approval or removal
pub(crate) fn effect_root_rune<'s>(gs: &GroupS<'s>) -> Option<IRuneS<'s>> {
  match gs {
    GroupS::Rune(ru) => Some(ru.rune),
    GroupS::Member { base, .. } => effect_root_rune(base),
    GroupS::Elements { base } => effect_root_rune(base),
    GroupS::Ellipsis { base } => effect_root_rune(base),
    _ => None,
  }
}

/// The human name of a group rune (only code runes have one).
// VLOOOOK: Option return — needs VOPT approval or removal
pub(crate) fn rune_name<'s>(rune: IRuneS<'s>) -> Option<StrI<'s>> {
  match rune {
    IRuneS::CodeRune(cn) => Some(cn.name),
    _ => None,
  }
}

/// The callee-rune → caller-group substitution for a call: each parameter that declares a borrow
/// group (`&T in g`) maps its rune to the caller-side group of the argument bound to it — read off the
/// argument's already-grouped result (an argument that isn't a borrow contributes nothing).
fn arg_rune_subst<'s, 't, 'g>(
  callee: &'s FunctionS<'s>,
  grouped_args: &[IExpressionGE<'s, 't, 'g>],
) -> IndexMap<IRuneS<'s>, GroupExprG<'s, 't>> {
  let mut subst = IndexMap::default();
  for (i, param) in callee.params.iter().enumerate() {
    if let ITypeST::BorrowRef(st) = param.tyype {
      if let RegionS::Group(GroupS::Rune(ru)) = st.region {
        if let Some(arg) = grouped_args.get(i) {
          if let KindGT::BorrowRef(b) = arg.result() {
            subst.insert(ru.rune, b.group.clone());
          }
        }
      }
    }
  }
  subst
}
