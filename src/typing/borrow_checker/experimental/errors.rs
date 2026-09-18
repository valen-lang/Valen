//! Borrow-checker error construction and rendering, kept in `experimental/` so the old checker owns its
//! diagnostics. The canonical `borrow_error::humanize` is stubbed on purpose; `sorcerous` renders its
//! own, and the core error humanizer dispatches to whichever checker the feature selects.

use crate::typing::borrow_checker::borrow_error::BorrowErrorKind;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::names::names::IVarNameT;
use crate::utils::range::RangeS;

impl<'s, 'ctx, 't> Compiler<'s, 'ctx, 't> {
  /// Wrap a borrow-check error kind and its source range into a compile error. Replaces the old
  /// `BorrowErrorKind::at` method — the wrapping belongs at the call site, not on the error value.
  pub(crate) fn borrow_error(
    &self,
    kind: BorrowErrorKind<'s, 't>,
    range: RangeS<'s>,
  ) -> ICompileErrorT<'s, 't> {
    ICompileErrorT::BorrowCheckError {
      range: self.typing_interner.alloc_slice_from_vec(vec![range]),
      kind,
    }
  }
}

/// Render a borrow-check diagnostic to text. The core error humanizer dispatches here (via the
/// feature-gated re-export from `borrow_checker`) so the old checker keeps its own wording while
/// `sorcerous` renders its own. `range` (the error's source spans) isn't needed — the wording comes
/// from `kind` — but the humanizer passes it for a uniform signature across both checkers.
pub fn humanize_borrow_error<'s, 't>(
  _range: &&'t [RangeS<'s>],
  kind: &BorrowErrorKind<'s, 't>,
) -> String {
  match kind {
    BorrowErrorKind::AliasingIntoDisjointMutGroups { local, arg_a, arg_b, group_a, group_b } => {
      format!(
        "Arguments {} and {} both borrow into {}, but their parameters are in disjoint mutated \
         groups {} and {}, which the callee may treat as non-aliasing.",
        arg_a,
        arg_b,
        var_name(local),
        group_a.0,
        group_b.0,
      )
    }
    BorrowErrorKind::BorrowIntoMovedArgument { local, borrow_arg, move_arg } => {
      format!(
        "Argument {} borrows into {}, but argument {} moves it, so the borrow would dangle.",
        borrow_arg,
        var_name(local),
        move_arg,
      )
    }
    BorrowErrorKind::UseAfterChurn { local } => {
      format!(
        "{} references an array element, which a preceding churn of its group may have moved or \
         deleted, so it can't be used here.",
        var_name(local),
      )
    }
    BorrowErrorKind::UseAfterChurnTemporary => {
      "This reference into an array element is held while a sibling argument churns its group, \
       which may have moved or deleted the element, so it can't be passed here."
        .to_string()
    }
    BorrowErrorKind::GrouplessReturnBorrow => {
      "This function returns a borrow reference with no group. Annotate the group it points into, \
       like `&T in g`."
        .to_string()
    }
    BorrowErrorKind::UnderivableBorrowGroup => {
      "The group of this borrow reference can't be determined from the expression that produces it."
        .to_string()
    }
    BorrowErrorKind::UndeclaredChurn => {
      "this call churns a group reached through a parameter, but the enclosing function does not \
       declare a mut effect for it."
        .to_string()
    }
  }
}

fn var_name<'s, 't>(name: &IVarNameT<'s, 't>) -> &'s str {
  match name {
    IVarNameT::Member(code_var) => code_var.imprecise_name.name.0,
    IVarNameT::Local(code_var) => code_var.imprecise_name.name.0,
    _ => "a local",
  }
}
