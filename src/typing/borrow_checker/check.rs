//! The borrow checker's entry point, per `docs/architecture/borrowing-design.md`.
//!
//! `check_function` runs the two phases: `groupify_function` builds the grouped body (each reference
//! binding carries its group, each call its churns and joint-argument facts), then `check_usages`
//! walks it once and rejects a use of a reference a churn spoiled. It stays pure — all inputs
//! immutable, the only output an error.

use bumpalo::Bump;

use crate::postparsing::ast::FunctionS;
use crate::postparsing::rules::types::{ITypeST, RegionS};
use crate::typing::ast::ast::FunctionDefinitionT;
use crate::typing::borrow_checker::borrow_error::BorrowErrorKind;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::compiler_outputs::CompilerOutputs;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
  /// Borrow-check one finished function body. `check_arena` (the `'g` arena) holds the grouped AST
  /// that phase 1 builds and phase 2 walks.
  pub fn check_function<'g>(
    &self,
    coutputs: &CompilerOutputs<'s, 't>,
    function_s: &'s FunctionS<'s>,
    function_t: &'t FunctionDefinitionT<'s, 't>,
    check_arena: &'g Bump,
  ) -> Result<(), ICompileErrorT<'s, 't>> {
    self.check_return_group(function_s)?;
    let body_g = self.groupify_function(coutputs, function_s, function_t, check_arena)?;
    self.check_usages(coutputs, &body_g)
  }

  /// A returned reference must declare the group it points into (signature-only derivation): reject a
  /// return type that is a borrow with no group.
  fn check_return_group(
    &self,
    function_s: &'s FunctionS<'s>,
  ) -> Result<(), ICompileErrorT<'s, 't>> {
    if let Some(ITypeST::BorrowRef(st)) = &function_s.maybe_return_type {
      if matches!(st.region, RegionS::Unspecified) {
        return Err(BorrowErrorKind::GrouplessReturnBorrow.at(self, st.range));
      }
    }
    Ok(())
  }
}
