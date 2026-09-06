//! `calculate_aliasing_info` — the borrow checker's third phase, per
//! `docs/architecture/borrowing-design.md`'s `calculate_aliasing_info` section.
//!
//! It reads the finished function and its grouped body and reports which parameters (and, later,
//! locals) may be treated as `noalias`, for the backend to feed to LLVM as `restrict`-equivalent
//! aliasing information.

use crate::postparsing::ast::FunctionS;
use crate::postparsing::rules::types::{ITypeST, RegionS};
use crate::typing::ast::ast::{FunctionAliasingInfoT, FunctionDefinitionT};
use crate::typing::borrow_checker::borrow_types::KindGT;
use crate::typing::borrow_checker::grouped_ast::{flatten, paths_alias, GroupStep, IExpressionGE};
use crate::typing::compiler::Compiler;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
  /// Track which parameters (and, later, locals) are `noalias` and where, to feed the backend for
  /// optimization. Currently a signature-only per-parameter verdict: a borrow parameter qualifies when
  /// its group path is not aliased by any other parameter's, exactly as `groupify` derives it. The
  /// grouped `body` is threaded in for the forthcoming block-scoped restrict regions.
  pub(crate) fn calculate_aliasing_info<'g>(
    &self,
    function_s: &'s FunctionS<'s>,
    function_t: &'t FunctionDefinitionT<'s, 't>,
    _body: &'g IExpressionGE<'s, 't, 'g>,
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
    FunctionAliasingInfoT { param_noalias }
  }
}
