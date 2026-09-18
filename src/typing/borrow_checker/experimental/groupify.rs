//! Phase 1: `groupify_function` walks the typed body and produces the grouped `IExpressionGE`, on the
//! canonical value types. For every reference-typed local it records the group its referent lives in; at
//! every call it records the groups it churns (`mut_effects`) and its joint-argument facts; each loop
//! node carries its body's aggregated churns. Everything is allocated in the `'g` check arena.

use crate::postparsing::ast::{FunctionS, ParameterS};
use crate::postparsing::names::{IRuneS, IVarDeclarationNameS};
use crate::postparsing::rules::types::{EffectS, GroupS, ITypeST, RegionS};
use crate::typing::ast::ast::FunctionDefinitionT;
use crate::typing::ast::expressions::{ExpressionTE, FunctionCallTE};
use crate::typing::borrow_checker::ast_g::{GroupStep, MutEffectPath};
use crate::typing::borrow_checker::access_event::AccessEventG;
use crate::typing::borrow_checker::experimental::borrow_types::{group_expr_from_group_s, subst_group_expr};
use crate::typing::borrow_checker::experimental::grouped_ast::{flatten, split_unions, IExpressionGE};
use crate::typing::borrow_checker::group_expr::GroupExprG;
use crate::typing::borrow_checker::kind_g::{BorrowRefGT, KindGT, VoidGT};
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::CompilerOutputs;
use crate::typing::names::names::{IdT, IdValT, INameT, IVarNameT};
use crate::typing::templata::templata::ITemplataT;
use crate::typing::types::types::KindT;
use crate::utils::fx::IndexMap;
use crate::utils::range::RangeS;
use bumpalo::Bump;
use crate::interner::StrI;

/// Phase-1 state: the reference-typed locals seen so far and where each points, plus the function being
/// grouped, and the access log for `calculate_aliasing_info`.
struct GCtx<'s, 't, 'g> {
  locals: Vec<(IVarNameT<'s, 't>, KindGT<'s, 't, 'g>)>,
  function_s: &'s FunctionS<'s>,
  function_t: &'t FunctionDefinitionT<'s, 't>,
  access_log: Vec<AccessEventG<'s, 't>>,
}

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
  /// Build the grouped body for one function, or an error if a borrow's group is underivable.
  pub fn groupify_function<'g>(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    function_s: &'s FunctionS<'s>,
    function_t: &'t FunctionDefinitionT<'s, 't>,
    arena: &'g Bump,
  ) -> Result<(IExpressionGE<'s, 't, 'g>, Vec<AccessEventG<'s, 't>>), ICompileErrorT<'s, 't>> {
    let mut ctx = GCtx { locals: vec![], function_s, function_t, access_log: vec![] };
    let body = self.groupify(coutputs, &function_t.body, &mut ctx, arena);
    Ok((body, ctx.access_log))
  }

  /// Rebuild one typed expression as its grouped mirror, allocating children in `arena`.
  fn groupify<'g>(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    expr: &ExpressionTE<'s, 't>,
    ctx: &mut GCtx<'s, 't, 'g>,
    arena: &'g Bump,
  ) -> IExpressionGE<'s, 't, 'g> {
    match expr {
      ExpressionTE::LetAndLend(e) => {
        let child = arena.alloc(self.groupify(coutputs, &e.expr, ctx, arena));
        ctx.locals.push((e.variable.name, child.result()));
        let result = ref_kind_g(GroupExprG::Local(&e.variable.name), child.result(), arena);
        IExpressionGE::LetAndLend { result, expr: child }
      }
      ExpressionTE::LockWeak(e) => {
        let inner_expr = arena.alloc(self.groupify(coutputs, &e.inner_expr, ctx, arena));
        let result = self.make_kind_g_groupless(expr.result(), arena);
        IExpressionGE::LockWeak { result, inner_expr }
      }
      ExpressionTE::BorrowToWeak(e) => {
        let inner_expr = arena.alloc(self.groupify(coutputs, &e.inner_expr, ctx, arena));
        let result = self.make_kind_g_groupless(expr.result(), arena);
        IExpressionGE::BorrowToWeak { result, inner_expr }
      }
      ExpressionTE::LetNormal(l) => {
        let child = arena.alloc(self.groupify(coutputs, &l.expr, ctx, arena));
        ctx.locals.push((l.variable.name, child.result()));
        let bind = match child.result() {
          KindGT::BorrowRef(b) => Some((l.variable.name, b.group)),
          _ => None,
        };
        IExpressionGE::LetNormal { result: void_kind_g(), expr: child, bind }
      }
      ExpressionTE::Unlet(u) => {
        IExpressionGE::Unlet { result: self.local_type(ctx, u.variable.name, u.variable.tyype, arena) }
      }
      ExpressionTE::Discard(e) => IExpressionGE::Discard {
        result: void_kind_g(),
        expr: arena.alloc(self.groupify(coutputs, &e.expr, ctx, arena)),
      },
      ExpressionTE::If(if_te) => {
        let condition = arena.alloc(self.groupify(coutputs, &if_te.condition, ctx, arena));
        let then_call = arena.alloc(self.groupify(coutputs, &if_te.then_call, ctx, arena));
        let else_call = arena.alloc(self.groupify(coutputs, &if_te.else_call, ctx, arena));
        let then_diverges = matches!(if_te.then_call.result(), KindT::Never(_));
        let else_diverges = matches!(if_te.else_call.result(), KindT::Never(_));
        let result = if then_diverges { else_call.result() } else { then_call.result() };
        IExpressionGE::If { result, condition, then_call, else_call, then_diverges, else_diverges }
      }
      ExpressionTE::While(w) => {
        let body = arena.alloc(self.groupify(coutputs, &w.block.inner, ctx, arena));
        let mut mut_effects = vec![];
        collect_subtree_churns(body, &mut mut_effects);
        IExpressionGE::While { result: self.make_kind_g_groupless(expr.result(), arena), body, mut_effects }
      }
      ExpressionTE::Mutate(m) => {
        let destination_expr = arena.alloc(self.groupify(coutputs, &m.destination_expr, ctx, arena));
        let source_expr = arena.alloc(self.groupify(coutputs, &m.source_expr, ctx, arena));
        if let Some((base_ref, group)) = self.base_ref_and_group(ctx, &m.destination_expr, arena) {
          ctx.access_log.push(AccessEventG::Store { base_ref, group, loct: m.loct });
        }
        IExpressionGE::Mutate { result: void_kind_g(), destination_expr, source_expr }
      }
      ExpressionTE::Restackify(e) => IExpressionGE::Restackify {
        result: void_kind_g(),
        source_expr: arena.alloc(self.groupify(coutputs, &e.source_expr, ctx, arena)),
      },
      ExpressionTE::Return(r) => IExpressionGE::Return {
        result: self.make_kind_g_groupless(expr.result(), arena),
        source_expr: arena.alloc(self.groupify(coutputs, &r.source_expr, ctx, arena)),
      },
      ExpressionTE::Break(_) => {
        IExpressionGE::Break { result: self.make_kind_g_groupless(expr.result(), arena) }
      }
      ExpressionTE::Block(b) => {
        let inner = arena.alloc(self.groupify(coutputs, &b.inner, ctx, arena));
        let result = inner.result();
        IExpressionGE::Block { result, inner }
      }
      ExpressionTE::Consecutor(c) => {
        // Group each statement, recording a bare-integer statement as a marker between its neighbors'
        // accesses so a restrict region can be pinned by value.
        let mut grouped = Vec::with_capacity(c.exprs.len());
        for e in c.exprs.iter() {
          let g = self.groupify(coutputs, e, ctx, arena);
          if let Some(value) = statement_marker(e) {
            ctx.access_log.push(AccessEventG::Marker { value });
          }
          grouped.push(g);
        }
        let exprs = arena.alloc_slice_fill_iter(grouped.into_iter());
        let result = exprs.last().expect("consecutor with no expressions").result();
        IExpressionGE::Consecutor { result, exprs }
      }
      ExpressionTE::StaticArrayFromValues(e) => IExpressionGE::StaticArrayFromValues {
        result: self.make_kind_g_groupless(expr.result(), arena),
        elements: arena
          .alloc_slice_fill_iter(e.elements.iter().map(|x| self.groupify(coutputs, x, ctx, arena))),
      },
      ExpressionTE::ArraySize(e) => IExpressionGE::ArraySize {
        result: self.make_kind_g_groupless(expr.result(), arena),
        array: arena.alloc(self.groupify(coutputs, &e.array, ctx, arena)),
      },
      ExpressionTE::IsSameInstance(e) => IExpressionGE::IsSameInstance {
        result: self.make_kind_g_groupless(expr.result(), arena),
        left: arena.alloc(self.groupify(coutputs, &e.left, ctx, arena)),
        right: arena.alloc(self.groupify(coutputs, &e.right, ctx, arena)),
      },
      ExpressionTE::AsSubtype(e) => {
        let source_expr = arena.alloc(self.groupify(coutputs, &e.source_expr, ctx, arena));
        let result = self.cast_result(expr.result(), source_expr.result(), arena);
        IExpressionGE::AsSubtype { result, source_expr }
      }
      ExpressionTE::VoidLiteral(_) => IExpressionGE::VoidLiteral { result: void_kind_g() },
      ExpressionTE::ConstantInt(_) => {
        IExpressionGE::ConstantInt { result: self.make_kind_g_groupless(expr.result(), arena) }
      }
      ExpressionTE::ConstantBool(_) => {
        IExpressionGE::ConstantBool { result: self.make_kind_g_groupless(expr.result(), arena) }
      }
      ExpressionTE::ConstantStr(_) => {
        IExpressionGE::ConstantStr { result: self.make_kind_g_groupless(expr.result(), arena) }
      }
      ExpressionTE::ConstantFloat(_) => {
        IExpressionGE::ConstantFloat { result: self.make_kind_g_groupless(expr.result(), arena) }
      }
      ExpressionTE::ArgLookup(a) => {
        IExpressionGE::ArgLookup { result: self.arg_type(a.param_index as usize, ctx, arena) }
      }
      ExpressionTE::ArrayLength(e) => IExpressionGE::ArrayLength {
        result: self.make_kind_g_groupless(expr.result(), arena),
        array_expr: arena.alloc(self.groupify(coutputs, &e.array_expr, ctx, arena)),
      },
      ExpressionTE::InterfaceFunctionCall(e) => IExpressionGE::InterfaceFunctionCall {
        result: self.make_kind_g_groupless(expr.result(), arena),
        args: arena.alloc_slice_fill_iter(e.args.iter().map(|a| self.groupify(coutputs, a, ctx, arena))),
      },
      ExpressionTE::BoundFunctionCall(e) => IExpressionGE::InterfaceFunctionCall {
        result: self.make_kind_g_groupless(expr.result(), arena),
        args: arena.alloc_slice_fill_iter(e.args.iter().map(|a| self.groupify(coutputs, a, ctx, arena))),
      },
      ExpressionTE::ExternFunctionCall(e) => IExpressionGE::ExternFunctionCall {
        result: self.make_kind_g_groupless(expr.result(), arena),
        args: arena.alloc_slice_fill_iter(e.args.iter().map(|a| self.groupify(coutputs, a, ctx, arena))),
      },
      ExpressionTE::FunctionCall(call) => {
        let args = arena
          .alloc_slice_fill_iter(call.args.iter().map(|a| self.groupify(coutputs, a, ctx, arena)));
        let (result, mut_effects) = match self.resolve_callee(coutputs, call) {
          Some(callee) => {
            let subst = arg_rune_subst(callee, args);
            (
              self.call_result_kind(call, callee, &subst, arena),
              self.call_mut_effects(call, callee, &subst, arena),
            )
          }
          None => (self.make_kind_g_groupless(call.callable.return_type, arena), vec![]),
        };
        let touched: Vec<Vec<GroupStep<'s, 't>>> =
          mut_effects.iter().map(|m| m.steps.iter().map(|s| **s).collect()).collect();
        ctx.access_log.push(AccessEventG::Call { touched, loct: call.loct });
        IExpressionGE::FunctionCall { result, args, mut_effects, call }
      }
      ExpressionTE::Reinterpret(e) => {
        let child = arena.alloc(self.groupify(coutputs, &e.expr, ctx, arena));
        let result = self.cast_result(expr.result(), child.result(), arena);
        IExpressionGE::Reinterpret { result, expr: child }
      }
      ExpressionTE::Construct(e) => IExpressionGE::Construct {
        result: self.make_kind_g_groupless(expr.result(), arena),
        args: arena.alloc_slice_fill_iter(e.args.iter().map(|a| self.groupify(coutputs, a, ctx, arena))),
      },
      ExpressionTE::NewRuntimeSizedArray(e) => IExpressionGE::NewRuntimeSizedArray {
        result: self.make_kind_g_groupless(expr.result(), arena),
        capacity_expr: arena.alloc(self.groupify(coutputs, &e.capacity_expr, ctx, arena)),
      },
      ExpressionTE::StaticArrayFromCallable(e) => IExpressionGE::StaticArrayFromCallable {
        result: self.make_kind_g_groupless(expr.result(), arena),
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
        result: self.make_kind_g_groupless(expr.result(), arena),
        array_expr: arena.alloc(self.groupify(coutputs, &e.array_expr, ctx, arena)),
      },
      ExpressionTE::PushRuntimeSizedArray(e) => IExpressionGE::PushRuntimeSizedArray {
        result: void_kind_g(),
        array_expr: arena.alloc(self.groupify(coutputs, &e.array_expr, ctx, arena)),
        new_element_expr: arena.alloc(self.groupify(coutputs, &e.new_element_expr, ctx, arena)),
      },
      ExpressionTE::PopRuntimeSizedArray(e) => IExpressionGE::PopRuntimeSizedArray {
        result: self.make_kind_g_groupless(expr.result(), arena),
        array_expr: arena.alloc(self.groupify(coutputs, &e.array_expr, ctx, arena)),
      },
      ExpressionTE::InterfaceToInterfaceUpcast(e) => {
        let inner_expr = arena.alloc(self.groupify(coutputs, &e.inner_expr, ctx, arena));
        let result = self.cast_result(expr.result(), inner_expr.result(), arena);
        IExpressionGE::InterfaceToInterfaceUpcast { result, inner_expr }
      }
      ExpressionTE::UpcastInterface(e) => {
        let inner_expr = arena.alloc(self.groupify(coutputs, &e.inner_expr, ctx, arena));
        let result = self.cast_result(expr.result(), inner_expr.result(), arena);
        IExpressionGE::Upcast { result, inner_expr }
      }
      ExpressionTE::UpcastGeneric(e) => {
        let inner_expr = arena.alloc(self.groupify(coutputs, &e.inner_expr, ctx, arena));
        let result = self.cast_result(expr.result(), inner_expr.result(), arena);
        IExpressionGE::Upcast { result, inner_expr }
      }
      ExpressionTE::Destroy(e) => IExpressionGE::Destroy {
        result: void_kind_g(),
        expr: arena.alloc(self.groupify(coutputs, &e.expr, ctx, arena)),
      },
      ExpressionTE::CopyPrim(e) => {
        let inner = arena.alloc(self.groupify(coutputs, &e.inner, ctx, arena));
        if let Some((base_ref, group)) = self.base_ref_and_group(ctx, &e.inner, arena) {
          ctx.access_log.push(AccessEventG::Read { base_ref, group, loct: e.loct });
        }
        IExpressionGE::CopyPrim { result: self.make_kind_g_groupless(expr.result(), arena), inner }
      }
      ExpressionTE::LocalLookup(l) => {
        let inner = self.local_type(ctx, l.local_variable.name, l.local_variable.tyype, arena);
        IExpressionGE::LocalLookup {
          result: ref_kind_g(GroupExprG::Local(&l.local_variable.name), inner, arena),
        }
      }
      ExpressionTE::StaticSizedArrayLookup(e) => {
        let array_expr = arena.alloc(self.groupify(coutputs, &e.array_expr, ctx, arena));
        let index_expr = arena.alloc(self.groupify(coutputs, &e.index_expr, ctx, arena));
        let result = self.element_result(expr.result(), array_expr.result(), arena);
        IExpressionGE::StaticSizedArrayLookup { result, array_expr, index_expr }
      }
      ExpressionTE::RuntimeSizedArrayLookup(e) => {
        let array_expr = arena.alloc(self.groupify(coutputs, &e.array_expr, ctx, arena));
        let index_expr = arena.alloc(self.groupify(coutputs, &e.index_expr, ctx, arena));
        let result = self.element_result(expr.result(), array_expr.result(), arena);
        IExpressionGE::RuntimeSizedArrayLookup { result, array_expr, index_expr }
      }
      ExpressionTE::MemberLookup(e) => {
        let struct_expr = arena.alloc(self.groupify(coutputs, &e.struct_expr, ctx, arena));
        let result = self.member_result(expr.result(), struct_expr.result(), &e.member_name, arena);
        IExpressionGE::MemberLookup { result, struct_expr }
      }
      ExpressionTE::Deref(d) => {
        let inner = arena.alloc(self.groupify(coutputs, &d.inner, ctx, arena));
        let result = deref_kind_g(inner.result());
        if !matches!(result, KindGT::BorrowRef(_)) {
          if let Some((base_ref, group)) = self.base_ref_and_group(ctx, &d.inner, arena) {
            ctx.access_log.push(AccessEventG::Read { base_ref, group, loct: d.loct });
          }
        }
        IExpressionGE::Deref { result, inner }
      }
    }
  }

  /// A local's grouped type: a tracked reference binding carries groups at every depth; anything else is
  /// its plain typed type, groupless.
  fn local_type<'g>(
    &self,
    ctx: &GCtx<'s, 't, 'g>,
    name: IVarNameT<'s, 't>,
    typed: KindT<'s, 't>,
    arena: &'g Bump,
  ) -> KindGT<'s, 't, 'g> {
    match ctx.locals.iter().find(|(n, _)| *n == name) {
      Some((_, k)) => *k,
      None => self.make_kind_g_groupless(typed, arena),
    }
  }

  /// A parameter's full grouped type, read from its written type at every depth.
  fn arg_type<'g>(&self, i: usize, ctx: &GCtx<'s, 't, 'g>, arena: &'g Bump) -> KindGT<'s, 't, 'g> {
    let ps = ctx.function_s.params.get(i).expect("arg index out of range");
    let pt = ctx.function_t.header.params.get(i).expect("arg index out of range");
    self.make_kind_g(pt.tyype, &ps.tyype, Some(&pt.name), arena)
  }

  /// A call's grouped result: the callee's declared return type, groups crossed into the caller frame.
  fn call_result_kind<'g>(
    &self,
    call: &FunctionCallTE<'s, 't>,
    callee: &'s FunctionS<'s>,
    subst: &IndexMap<IRuneS<'s>, GroupExprG<'s, 't, 'g>>,
    arena: &'g Bump,
  ) -> KindGT<'s, 't, 'g> {
    match callee.maybe_return_type.as_ref() {
      Some(return_st) => {
        let return_kind_g = self.make_kind_g(call.callable.return_type, return_st, None, arena);
        self.substitute_groups(return_kind_g, subst, arena)
      }
      None => self.make_kind_g_groupless(call.callable.return_type, arena),
    }
  }

  /// A cast keeps the operand's outer group and re-expresses the referent's structure.
  fn cast_result<'g>(
    &self,
    cast_kind: KindT<'s, 't>,
    operand: KindGT<'s, 't, 'g>,
    arena: &'g Bump,
  ) -> KindGT<'s, 't, 'g> {
    match (cast_kind, operand) {
      (KindT::BorrowRef(b), KindGT::BorrowRef(ob)) => {
        ref_kind_g(ob.group, self.make_kind_g_groupless(b.inner, arena), arena)
      }
      (other, _) => self.make_kind_g_groupless(other, arena),
    }
  }

  /// An array-element access yields a borrow into the array's child-elements group.
  fn element_result<'g>(
    &self,
    access_kind: KindT<'s, 't>,
    array: KindGT<'s, 't, 'g>,
    arena: &'g Bump,
  ) -> KindGT<'s, 't, 'g> {
    match (access_kind, array) {
      (KindT::BorrowRef(b), KindGT::BorrowRef(ab)) => ref_kind_g(
        GroupExprG::ChildElements { base: arena.alloc(ab.group) },
        self.make_kind_g_groupless(b.inner, arena),
        arena,
      ),
      (other, _) => self.make_kind_g_groupless(other, arena),
    }
  }

  /// A member access yields a borrow into the struct's member child group.
  fn member_result<'g>(
    &self,
    access_kind: KindT<'s, 't>,
    struct_val: KindGT<'s, 't, 'g>,
    member_name_t: &IVarNameT<'s, 't>,
    arena: &'g Bump,
  ) -> KindGT<'s, 't, 'g> {
    match (access_kind, struct_val) {
      (KindT::BorrowRef(b), KindGT::BorrowRef(sb)) => {
        let member_name = match member_name_t {
          IVarNameT::Member(cv) => cv.imprecise_name.name,
          IVarNameT::Local(cv) => cv.imprecise_name.name,
          _ => panic!("vfail: member lookup with a non-member name"),
        };
        ref_kind_g(
          GroupExprG::Member { base: arena.alloc(sb.group), member_name },
          self.make_kind_g_groupless(b.inner, arena),
          arena,
        )
      }
      (other, _) => self.make_kind_g_groupless(other, arena),
    }
  }

  /// The caller-side groups a call churns, from the callee's declared effects and parameter groups.
  fn call_mut_effects<'g>(
    &self,
    call: &FunctionCallTE<'s, 't>,
    callee: &'s FunctionS<'s>,
    subst: &IndexMap<IRuneS<'s>, GroupExprG<'s, 't, 'g>>,
    arena: &'g Bump,
  ) -> Vec<MutEffectPath<'s, 't, 'g>> {
    let mut paths = vec![];
    for effect in callee.effects {
      if let EffectS::Mut(gs) = effect {
        let caller = subst_group_expr(group_expr_from_group_s(gs, arena), subst, arena);
        for steps in split_unions(caller) {
          let step_refs: Vec<&'g GroupStep<'s, 't>> =
            steps.iter().map(|s| &*arena.alloc(*s)).collect();
          paths.push(MutEffectPath {
            effecting_node_loc: call.loct,
            steps: arena.alloc_slice_fill_iter(step_refs),
          });
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

  /// The root reference an access chain goes through, and the flat group that reference points into.
  fn base_ref_and_group<'g>(
    &self,
    ctx: &GCtx<'s, 't, 'g>,
    expr: &ExpressionTE<'s, 't>,
    arena: &'g Bump,
  ) -> Option<(IVarNameT<'s, 't>, Vec<GroupStep<'s, 't>>)> {
    match expr {
      ExpressionTE::LocalLookup(l) => {
        let ty = self.local_type(ctx, l.local_variable.name, l.local_variable.tyype, arena);
        Some((l.local_variable.name, borrowref_group(ty)?))
      }
      ExpressionTE::ArgLookup(a) => {
        let name = ctx.function_t.header.params.get(a.param_index as usize)?.name;
        Some((name, borrowref_group(self.arg_type(a.param_index as usize, ctx, arena))?))
      }
      ExpressionTE::Deref(d) => self.base_ref_and_group(ctx, &d.inner, arena),
      ExpressionTE::CopyPrim(e) => self.base_ref_and_group(ctx, &e.inner, arena),
      ExpressionTE::MemberLookup(e) => self.base_ref_and_group(ctx, &e.struct_expr, arena),
      ExpressionTE::StaticSizedArrayLookup(e) => self.base_ref_and_group(ctx, &e.array_expr, arena),
      ExpressionTE::RuntimeSizedArrayLookup(e) => self.base_ref_and_group(ctx, &e.array_expr, arena),
      _ => None,
    }
  }
}

/// The flattened group a borrow result points into, or `None` for a non-borrow.
fn borrowref_group<'s, 't, 'g>(k: KindGT<'s, 't, 'g>) -> Option<Vec<GroupStep<'s, 't>>> {
  match k {
    KindGT::BorrowRef(b) => Some(flatten(b.group)),
    _ => None,
  }
}

/// Every churn inside a grouped subtree, for a loop's aggregated `mut_effects`.
fn collect_subtree_churns<'s, 't, 'g>(
  node: &IExpressionGE<'s, 't, 'g>,
  out: &mut Vec<MutEffectPath<'s, 't, 'g>>,
) {
  if let IExpressionGE::FunctionCall { mut_effects, .. } = node {
    // Canonical `MutEffectPath` isn't `Copy`; its fields are, so mirror it by hand.
    out.extend(
      mut_effects
        .iter()
        .map(|m| MutEffectPath { effecting_node_loc: m.effecting_node_loc, steps: m.steps }),
    );
  }
  for child in node.children() {
    collect_subtree_churns(child, out);
  }
}

/// A borrow reference `KindGT` from a group and its referent type.
fn ref_kind_g<'s, 't, 'g>(
  group: GroupExprG<'s, 't, 'g>,
  inner: KindGT<'s, 't, 'g>,
  arena: &'g Bump,
) -> KindGT<'s, 't, 'g> {
  KindGT::BorrowRef(arena.alloc(BorrowRefGT { inner, group }))
}

/// The `void` result `KindGT`, for statement-like nodes.
fn void_kind_g<'s, 't, 'g>() -> KindGT<'s, 't, 'g> {
  KindGT::Void(VoidGT)
}

/// Peel one borrow: a `Deref`'s result is its operand's referent.
fn deref_kind_g<'s, 't, 'g>(operand: KindGT<'s, 't, 'g>) -> KindGT<'s, 't, 'g> {
  match operand {
    KindGT::BorrowRef(b) => b.inner,
    other => panic!("vfail: deref of a non-borrow: {:?}", other),
  }
}

/// A statement-position bare integer landmark (`103;`), for tests to pin a restrict region by value.
/// Matches a `ConstantInt` standing on its own — with or without the `Discard` the typing pass wraps a
/// dropped value in.
fn statement_marker<'s, 't>(expr: &ExpressionTE<'s, 't>) -> Option<i32> {
  let inner = match expr {
    ExpressionTE::Discard(d) => &d.expr,
    other => other,
  };
  match inner {
    ExpressionTE::ConstantInt(c) => match &c.value {
      ITemplataT::Integer(n) => Some(*n as i32),
      _ => None,
    },
    _ => None,
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

/// The source range to point a held-register diagnostic at: the argument's own range.
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

/// The callee-rune → caller-group substitution for a call.
fn arg_rune_subst<'s, 't, 'g>(
  callee: &'s FunctionS<'s>,
  grouped_args: &[IExpressionGE<'s, 't, 'g>],
) -> IndexMap<IRuneS<'s>, GroupExprG<'s, 't, 'g>> {
  let mut subst = IndexMap::default();
  for (i, param) in callee.params.iter().enumerate() {
    if let ITypeST::BorrowRef(st) = param.tyype {
      if let RegionS::Group(GroupS::Rune(ru)) = st.region {
        if let Some(arg) = grouped_args.get(i) {
          if let KindGT::BorrowRef(b) = arg.result() {
            subst.insert(ru.rune, b.group);
          }
        }
      }
    }
  }
  subst
}
