use crate::postparsing::ast::FunctionS;
use crate::typing::ast::ast::FunctionDefinitionT;
use crate::typing::ast::expressions::{ExpressionTE, FunctionCallTE};
use crate::typing::borrow_checker::call_check::check_call;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::CompilerOutputs;

/// Post-hoc, read-only borrow check of one finished function body. Pure read of `function` +
/// `function_s`; mutates nothing, triggers no resolution. Runs at the tail of the function's
/// typecheck, so it never demotes a candidate. Surfaces a violation as `Err`; multi-error
/// accumulation is a future refactor.
pub fn check_function<'s, 'ctx, 't>(
  function: &'t FunctionDefinitionT<'s, 't>,
  function_s: &'s FunctionS<'s>,
  coutputs: &CompilerOutputs<'s, 't>,
  compiler: &Compiler<'s, 'ctx, 't>,
) -> Result<(), ICompileErrorT<'s, 't>> {
  let mut calls = Vec::new();
  collect_calls(&function.body, &mut calls);
  for call in calls {
    check_call(call, function_s.range, coutputs, compiler)?;
  }
  Ok(())
}

/// Collect every `FunctionCallTE` reachable in a finished body, descending into each node's child
/// expressions. The match is **exhaustive on purpose**: a new `ExpressionTE` variant fails to compile
/// here until someone decides whether the walk descends it, so the walk cannot silently regain a gap.
fn collect_calls<'s, 't>(expr: &ExpressionTE<'s, 't>, out: &mut Vec<&'t FunctionCallTE<'s, 't>>) {
  match *expr {
    ExpressionTE::FunctionCall(call) => {
      out.push(call);
      for arg in call.args {
        collect_calls(arg, out);
      }
    }
    ExpressionTE::Consecutor(consecutor) => {
      for inner in consecutor.exprs {
        collect_calls(inner, out);
      }
    }
    ExpressionTE::Block(block) => collect_calls(&block.inner, out),
    ExpressionTE::Discard(discard) => collect_calls(&discard.expr, out),
    ExpressionTE::If(if_expr) => {
      collect_calls(&if_expr.condition, out);
      collect_calls(&if_expr.then_call, out);
      collect_calls(&if_expr.else_call, out);
    }
    ExpressionTE::While(while_expr) => collect_calls(&while_expr.block.inner, out),
    ExpressionTE::LetNormal(let_normal) => collect_calls(&let_normal.expr, out),
    ExpressionTE::Return(ret) => collect_calls(&ret.source_expr, out),
    ExpressionTE::Mutate(mutate) => {
      collect_calls(&mutate.destination_expr, out);
      collect_calls(&mutate.source_expr, out);
    }
    // Not descended today. Several of these hold child expressions (`ExternFunctionCall`,
    // `InterfaceFunctionCall`, the member/array lookups, `Tuple`, `Construct`, `LetAndLend`, the
    // array ops) and are known gaps: add a recursing arm with a red test when a reachable violating
    // call can nest there. The rest are leaves. Kept explicit rather than a wildcard so a new variant
    // forces this decision.
    ExpressionTE::AddressMemberLookup(_)
    | ExpressionTE::ArgLookup(_)
    | ExpressionTE::ArrayLength(_)
    | ExpressionTE::ArraySize(_)
    | ExpressionTE::AsSubtype(_)
    | ExpressionTE::BorrowToWeak(_)
    | ExpressionTE::Break(_)
    | ExpressionTE::ConstantBool(_)
    | ExpressionTE::ConstantFloat(_)
    | ExpressionTE::ConstantInt(_)
    | ExpressionTE::ConstantStr(_)
    | ExpressionTE::Construct(_)
    | ExpressionTE::CopyPrim(_)
    | ExpressionTE::Defer(_)
    | ExpressionTE::Deref(_)
    | ExpressionTE::Destroy(_)
    | ExpressionTE::DestroyRuntimeSizedArray(_)
    | ExpressionTE::DestroyStaticSizedArrayIntoFunction(_)
    | ExpressionTE::DestroyStaticSizedArrayIntoLocals(_)
    | ExpressionTE::ExternFunctionCall(_)
    | ExpressionTE::InterfaceFunctionCall(_)
    | ExpressionTE::InterfaceToInterfaceUpcast(_)
    | ExpressionTE::IsSameInstance(_)
    | ExpressionTE::LetAndLend(_)
    | ExpressionTE::LocalLookup(_)
    | ExpressionTE::LockWeak(_)
    | ExpressionTE::NewRuntimeSizedArray(_)
    | ExpressionTE::PopRuntimeSizedArray(_)
    | ExpressionTE::PushRuntimeSizedArray(_)
    | ExpressionTE::ReferenceMemberLookup(_)
    | ExpressionTE::Reinterpret(_)
    | ExpressionTE::Restackify(_)
    | ExpressionTE::RuntimeSizedArrayCapacity(_)
    | ExpressionTE::RuntimeSizedArrayLookup(_)
    | ExpressionTE::StaticArrayFromCallable(_)
    | ExpressionTE::StaticArrayFromValues(_)
    | ExpressionTE::StaticSizedArrayLookup(_)
    | ExpressionTE::Tuple(_)
    | ExpressionTE::Unlet(_)
    | ExpressionTE::Upcast(_)
    | ExpressionTE::VoidLiteral(_) => {}
  }
}
