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

/// Collect every `FunctionCallTE` reachable in a finished body, descending the structural nodes an
/// ordinary body is built from. Leaves without child expressions are skipped; control-flow nodes are
/// added as later slices exercise nesting.
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
    _ => {}
  }
}
