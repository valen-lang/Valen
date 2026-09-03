//! The grouped AST — phase 1's output, consumed by phase 2 (`check_usages`).
//!
//! `IExpressionGE` mirrors `ExpressionTE` variant-for-variant (`src/typing/ast/expressions.rs`), so
//! `groupify_function` walks the typed body and rebuilds it with group information filled in:
//!
//!  * Every node carries its result `KindGT` (a borrow's `BorrowRefGT` names the group it points at).
//!  * Every `FunctionCall` carries the groups it churns (`mut_effects`), its argument reference-uses,
//!    and its joint-argument facts.
//!  * Every `While` carries its body's aggregated `mut_effects`, so phase 2 needs no loop fixpoint.
//!
//! The tree is allocated in the `'g` arena — children are `&'g` node refs and `&'g [..]` slices. See
//! `docs/architecture/borrowing-design.md`.

use crate::interner::StrI;
use crate::postparsing::names::IRuneS;
use crate::typing::ast::ast::LocT;
use crate::typing::ast::expressions::FunctionCallTE;
use crate::typing::borrow_checker::borrow_types::{GroupExprG, KindGT};
use crate::typing::names::names::IVarNameT;
use crate::utils::range::RangeS;

/// One step of a group path — the flattened form of a `GroupExprG`, so a churned group can be
/// compared against where a reference points.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum GroupStep<'s, 't> {
  Rune(IRuneS<'s>),
  ParamAnonymousGroup(IVarNameT<'s, 't>),
  Local(IVarNameT<'s, 't>),
  Member { member_name: StrI<'s> },
  Elements,
}

/// A specific mutation to a specific group: the churning call's location plus the caller-side group
/// path it mutates.
#[derive(Clone)]
pub struct MutEffectPath<'s, 't> {
  pub effecting_node_loc: LocT<'t>,
  pub steps: Vec<GroupStep<'s, 't>>,
}

/// One joint-argument violation candidate at a call, in the two shapes the checker reports.
#[derive(Clone)]
pub enum JointFact<'s, 't> {
  /// A borrow argument rooted in a local that a sibling argument moves.
  BorrowIntoMoved { local: IVarNameT<'s, 't>, borrow_arg: usize, move_arg: usize, range: RangeS<'s> },
  /// Two aliasing borrow arguments bound to parameters in distinct mutated groups.
  AliasingDisjointMut {
    local: IVarNameT<'s, 't>,
    arg_a: usize,
    arg_b: usize,
    group_a: StrI<'s>,
    group_b: StrI<'s>,
    range: RangeS<'s>,
  },
}

/// The group-annotated mirror of `ExpressionTE`: one variant per `ExpressionTE` variant, each
/// carrying its result `KindGT`. Children are `'g`-arena refs. Reference bindings (`LetNormal.bind`)
/// carry where they point; calls carry their churns/uses/joint-facts; `while` carries its aggregated
/// churns; `if` carries branch divergence.
pub enum IExpressionGE<'s, 't, 'g> {
  LetAndLend { result: KindGT<'s, 't>, expr: &'g IExpressionGE<'s, 't, 'g> },
  LockWeak { result: KindGT<'s, 't>, inner_expr: &'g IExpressionGE<'s, 't, 'g> },
  BorrowToWeak { result: KindGT<'s, 't>, inner_expr: &'g IExpressionGE<'s, 't, 'g> },
  /// A `let`. `bind` is set when the bound value is a tracked reference: the local and its group.
  LetNormal {
    result: KindGT<'s, 't>,
    expr: &'g IExpressionGE<'s, 't, 'g>,
    // VLOOOOK: Option field — needs VOPT approval or removal
    bind: Option<(IVarNameT<'s, 't>, GroupExprG<'s, 't>)>,
  },
  Unlet { result: KindGT<'s, 't> },
  Discard { result: KindGT<'s, 't>, expr: &'g IExpressionGE<'s, 't, 'g> },
  If {
    result: KindGT<'s, 't>,
    condition: &'g IExpressionGE<'s, 't, 'g>,
    then_call: &'g IExpressionGE<'s, 't, 'g>,
    else_call: &'g IExpressionGE<'s, 't, 'g>,
    then_diverges: bool,
    else_diverges: bool,
  },
  While {
    result: KindGT<'s, 't>,
    body: &'g IExpressionGE<'s, 't, 'g>,
    mut_effects: Vec<MutEffectPath<'s, 't>>,
  },
  Mutate {
    result: KindGT<'s, 't>,
    destination_expr: &'g IExpressionGE<'s, 't, 'g>,
    source_expr: &'g IExpressionGE<'s, 't, 'g>,
  },
  Restackify { result: KindGT<'s, 't>, source_expr: &'g IExpressionGE<'s, 't, 'g> },
  Return { result: KindGT<'s, 't>, source_expr: &'g IExpressionGE<'s, 't, 'g> },
  Break { result: KindGT<'s, 't> },
  Block { result: KindGT<'s, 't>, inner: &'g IExpressionGE<'s, 't, 'g> },
  Consecutor { result: KindGT<'s, 't>, exprs: &'g [IExpressionGE<'s, 't, 'g>] },
  StaticArrayFromValues { result: KindGT<'s, 't>, elements: &'g [IExpressionGE<'s, 't, 'g>] },
  ArraySize { result: KindGT<'s, 't>, array: &'g IExpressionGE<'s, 't, 'g> },
  IsSameInstance {
    result: KindGT<'s, 't>,
    left: &'g IExpressionGE<'s, 't, 'g>,
    right: &'g IExpressionGE<'s, 't, 'g>,
  },
  AsSubtype { result: KindGT<'s, 't>, source_expr: &'g IExpressionGE<'s, 't, 'g> },
  VoidLiteral { result: KindGT<'s, 't> },
  ConstantInt { result: KindGT<'s, 't> },
  ConstantBool { result: KindGT<'s, 't> },
  ConstantStr { result: KindGT<'s, 't> },
  ConstantFloat { result: KindGT<'s, 't> },
  ArgLookup { result: KindGT<'s, 't> },
  ArrayLength { result: KindGT<'s, 't>, array_expr: &'g IExpressionGE<'s, 't, 'g> },
  InterfaceFunctionCall { result: KindGT<'s, 't>, args: &'g [IExpressionGE<'s, 't, 'g>] },
  ExternFunctionCall { result: KindGT<'s, 't>, args: &'g [IExpressionGE<'s, 't, 'g>] },
  /// A call. Phase 2 walks its grouped arguments first (nested churns/uses/held registers), then runs
  /// the argument checks — reference-uses, held-register uses, and joint-argument overlaps/aliasing —
  /// then applies `mut_effects`. The argument checks need the typed argument identities and ranges, so
  /// the node keeps the originating `call`; the group paths come from the grouped `args`' results.
  FunctionCall {
    result: KindGT<'s, 't>,
    args: &'g [IExpressionGE<'s, 't, 'g>],
    mut_effects: Vec<MutEffectPath<'s, 't>>,
    call: &'t FunctionCallTE<'s, 't>,
  },
  Reinterpret { result: KindGT<'s, 't>, expr: &'g IExpressionGE<'s, 't, 'g> },
  Construct { result: KindGT<'s, 't>, args: &'g [IExpressionGE<'s, 't, 'g>] },
  NewRuntimeSizedArray { result: KindGT<'s, 't>, capacity_expr: &'g IExpressionGE<'s, 't, 'g> },
  StaticArrayFromCallable { result: KindGT<'s, 't>, generator: &'g IExpressionGE<'s, 't, 'g> },
  DestroyStaticSizedArrayIntoFunction {
    result: KindGT<'s, 't>,
    array_expr: &'g IExpressionGE<'s, 't, 'g>,
    consumer: &'g IExpressionGE<'s, 't, 'g>,
  },
  DestroyStaticSizedArrayIntoLocals { result: KindGT<'s, 't>, expr: &'g IExpressionGE<'s, 't, 'g> },
  DestroyRuntimeSizedArray { result: KindGT<'s, 't>, array_expr: &'g IExpressionGE<'s, 't, 'g> },
  RuntimeSizedArrayCapacity { result: KindGT<'s, 't>, array_expr: &'g IExpressionGE<'s, 't, 'g> },
  PushRuntimeSizedArray {
    result: KindGT<'s, 't>,
    array_expr: &'g IExpressionGE<'s, 't, 'g>,
    new_element_expr: &'g IExpressionGE<'s, 't, 'g>,
  },
  PopRuntimeSizedArray { result: KindGT<'s, 't>, array_expr: &'g IExpressionGE<'s, 't, 'g> },
  InterfaceToInterfaceUpcast { result: KindGT<'s, 't>, inner_expr: &'g IExpressionGE<'s, 't, 'g> },
  Upcast { result: KindGT<'s, 't>, inner_expr: &'g IExpressionGE<'s, 't, 'g> },
  Destroy { result: KindGT<'s, 't>, expr: &'g IExpressionGE<'s, 't, 'g> },
  CopyPrim { result: KindGT<'s, 't>, inner: &'g IExpressionGE<'s, 't, 'g> },
  LocalLookup { result: KindGT<'s, 't> },
  StaticSizedArrayLookup {
    result: KindGT<'s, 't>,
    array_expr: &'g IExpressionGE<'s, 't, 'g>,
    index_expr: &'g IExpressionGE<'s, 't, 'g>,
  },
  RuntimeSizedArrayLookup {
    result: KindGT<'s, 't>,
    array_expr: &'g IExpressionGE<'s, 't, 'g>,
    index_expr: &'g IExpressionGE<'s, 't, 'g>,
  },
  MemberLookup { result: KindGT<'s, 't>, struct_expr: &'g IExpressionGE<'s, 't, 'g> },
  Deref { result: KindGT<'s, 't>, inner: &'g IExpressionGE<'s, 't, 'g> },
}

impl<'s, 't, 'g> IExpressionGE<'s, 't, 'g> {
  /// This node's group-annotated result type.
  pub fn result(&self) -> &KindGT<'s, 't> {
    match self {
      IExpressionGE::LetAndLend { result, .. }
      | IExpressionGE::LockWeak { result, .. }
      | IExpressionGE::BorrowToWeak { result, .. }
      | IExpressionGE::LetNormal { result, .. }
      | IExpressionGE::Unlet { result }
      | IExpressionGE::Discard { result, .. }
      | IExpressionGE::If { result, .. }
      | IExpressionGE::While { result, .. }
      | IExpressionGE::Mutate { result, .. }
      | IExpressionGE::Restackify { result, .. }
      | IExpressionGE::Return { result, .. }
      | IExpressionGE::Break { result }
      | IExpressionGE::Block { result, .. }
      | IExpressionGE::Consecutor { result, .. }
      | IExpressionGE::StaticArrayFromValues { result, .. }
      | IExpressionGE::ArraySize { result, .. }
      | IExpressionGE::IsSameInstance { result, .. }
      | IExpressionGE::AsSubtype { result, .. }
      | IExpressionGE::VoidLiteral { result }
      | IExpressionGE::ConstantInt { result }
      | IExpressionGE::ConstantBool { result }
      | IExpressionGE::ConstantStr { result }
      | IExpressionGE::ConstantFloat { result }
      | IExpressionGE::ArgLookup { result }
      | IExpressionGE::ArrayLength { result, .. }
      | IExpressionGE::InterfaceFunctionCall { result, .. }
      | IExpressionGE::ExternFunctionCall { result, .. }
      | IExpressionGE::FunctionCall { result, .. }
      | IExpressionGE::Reinterpret { result, .. }
      | IExpressionGE::Construct { result, .. }
      | IExpressionGE::NewRuntimeSizedArray { result, .. }
      | IExpressionGE::StaticArrayFromCallable { result, .. }
      | IExpressionGE::DestroyStaticSizedArrayIntoFunction { result, .. }
      | IExpressionGE::DestroyStaticSizedArrayIntoLocals { result, .. }
      | IExpressionGE::DestroyRuntimeSizedArray { result, .. }
      | IExpressionGE::RuntimeSizedArrayCapacity { result, .. }
      | IExpressionGE::PushRuntimeSizedArray { result, .. }
      | IExpressionGE::PopRuntimeSizedArray { result, .. }
      | IExpressionGE::InterfaceToInterfaceUpcast { result, .. }
      | IExpressionGE::Upcast { result, .. }
      | IExpressionGE::Destroy { result, .. }
      | IExpressionGE::CopyPrim { result, .. }
      | IExpressionGE::LocalLookup { result }
      | IExpressionGE::StaticSizedArrayLookup { result, .. }
      | IExpressionGE::RuntimeSizedArrayLookup { result, .. }
      | IExpressionGE::MemberLookup { result, .. }
      | IExpressionGE::Deref { result, .. } => result,
    }
  }

  /// This node's child sub-expressions, in evaluation order. Phase 2's generic walk visits these;
  /// the nodes that carry checker payloads (`LetNormal`, `FunctionCall`, `While`, `If`) are handled
  /// directly and do not rely on this.
  pub fn children(&self) -> Vec<&'g IExpressionGE<'s, 't, 'g>> {
    match self {
      IExpressionGE::LetAndLend { expr, .. }
      | IExpressionGE::LetNormal { expr, .. }
      | IExpressionGE::Discard { expr, .. }
      | IExpressionGE::DestroyStaticSizedArrayIntoLocals { expr, .. }
      | IExpressionGE::Destroy { expr, .. }
      | IExpressionGE::Reinterpret { expr, .. } => vec![expr],
      IExpressionGE::LockWeak { inner_expr, .. }
      | IExpressionGE::BorrowToWeak { inner_expr, .. }
      | IExpressionGE::InterfaceToInterfaceUpcast { inner_expr, .. }
      | IExpressionGE::Upcast { inner_expr, .. } => vec![inner_expr],
      IExpressionGE::Block { inner, .. } | IExpressionGE::CopyPrim { inner, .. } => vec![inner],
      IExpressionGE::Deref { inner, .. } => vec![inner],
      IExpressionGE::Restackify { source_expr, .. }
      | IExpressionGE::Return { source_expr, .. }
      | IExpressionGE::AsSubtype { source_expr, .. } => vec![source_expr],
      IExpressionGE::ArraySize { array, .. } => vec![array],
      IExpressionGE::ArrayLength { array_expr, .. }
      | IExpressionGE::DestroyRuntimeSizedArray { array_expr, .. }
      | IExpressionGE::RuntimeSizedArrayCapacity { array_expr, .. }
      | IExpressionGE::PopRuntimeSizedArray { array_expr, .. } => vec![array_expr],
      IExpressionGE::NewRuntimeSizedArray { capacity_expr, .. } => vec![capacity_expr],
      IExpressionGE::StaticArrayFromCallable { generator, .. } => vec![generator],
      IExpressionGE::MemberLookup { struct_expr, .. } => vec![struct_expr],
      IExpressionGE::Mutate { destination_expr, source_expr, .. } => vec![destination_expr, source_expr],
      IExpressionGE::IsSameInstance { left, right, .. } => vec![left, right],
      IExpressionGE::DestroyStaticSizedArrayIntoFunction { array_expr, consumer, .. } => {
        vec![array_expr, consumer]
      }
      IExpressionGE::PushRuntimeSizedArray { array_expr, new_element_expr, .. } => {
        vec![array_expr, new_element_expr]
      }
      IExpressionGE::StaticSizedArrayLookup { array_expr, index_expr, .. }
      | IExpressionGE::RuntimeSizedArrayLookup { array_expr, index_expr, .. } => {
        vec![array_expr, index_expr]
      }
      IExpressionGE::Consecutor { exprs, .. }
      | IExpressionGE::StaticArrayFromValues { elements: exprs, .. }
      | IExpressionGE::InterfaceFunctionCall { args: exprs, .. }
      | IExpressionGE::ExternFunctionCall { args: exprs, .. }
      | IExpressionGE::FunctionCall { args: exprs, .. }
      | IExpressionGE::Construct { args: exprs, .. } => exprs.iter().collect(),
      IExpressionGE::If { condition, then_call, else_call, .. } => {
        vec![condition, then_call, else_call]
      }
      IExpressionGE::While { body, .. } => vec![body],
      IExpressionGE::Unlet { .. }
      | IExpressionGE::Break { .. }
      | IExpressionGE::VoidLiteral { .. }
      | IExpressionGE::ConstantInt { .. }
      | IExpressionGE::ConstantBool { .. }
      | IExpressionGE::ConstantStr { .. }
      | IExpressionGE::ConstantFloat { .. }
      | IExpressionGE::ArgLookup { .. }
      | IExpressionGE::LocalLookup { .. } => vec![],
    }
  }
}

/// Flatten a non-union `GroupExprG` to a root-to-leaf step path.
pub fn flatten<'s, 't>(g: &GroupExprG<'s, 't>) -> Vec<GroupStep<'s, 't>> {
  match g {
    GroupExprG::Rune(r) => vec![GroupStep::Rune(*r)],
    GroupExprG::ParamAnonymousGroup(s) => vec![GroupStep::ParamAnonymousGroup(*s)],
    GroupExprG::Local(s) => vec![GroupStep::Local(*s)],
    GroupExprG::Member { base, member_name } => {
      let mut v = flatten(base);
      v.push(GroupStep::Member { member_name: *member_name });
      v
    }
    GroupExprG::Elements { base } => {
      let mut v = flatten(base);
      v.push(GroupStep::Elements);
      v
    }
    // A `...` step collapses to its base: `mut(g...)` churns exactly `mut(g)`, and an ellipsis
    // reference's own invalidation is handled directly in `is_invalidated`, not via flattening.
    GroupExprG::Ellipsis { base } => flatten(base),
    GroupExprG::Union { .. } => vec![],
  }
}

/// Split a (possibly union) group into one flat path per non-union member.
pub fn split_unions<'s, 't>(g: &GroupExprG<'s, 't>) -> Vec<Vec<GroupStep<'s, 't>>> {
  match g {
    GroupExprG::Union { members } => members.iter().flat_map(split_unions).collect(),
    other => vec![flatten(other)],
  }
}

/// Whether two flattened group paths overlap: one is a prefix of the other (nested), including equal.
pub(crate) fn paths_alias<'s, 't>(a: &[GroupStep<'s, 't>], b: &[GroupStep<'s, 't>]) -> bool {
  let n = a.len().min(b.len());
  a[..n] == b[..n]
}
