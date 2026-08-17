use crate::interner::StrI;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::names::names::IVarNameT;
use crate::utils::range::RangeS;

/// A borrow-check violation. Each kind carries the facts a diagnostic needs; `humanize` renders it.
/// New kinds are added here (a variant plus a `humanize` branch) with no further core edit — the
/// `ICompileErrorT::BorrowCheckError` humanizer arm delegates to `humanize`.
#[derive(Debug)]
pub enum BorrowErrorKind<'s, 't> {
  /// Two arguments bound to parameters in distinct mutated groups alias the same place; the callee
  /// is entitled to treat those groups as disjoint.
  AliasingIntoDisjointMutGroups {
    local: IVarNameT<'s, 't>,
    arg_a: usize,
    arg_b: usize,
    group_a: StrI<'s>,
    group_b: StrI<'s>,
  },
  /// A borrow argument is rooted in a local that another argument moves, so the borrow would dangle.
  BorrowIntoMovedArgument {
    local: IVarNameT<'s, 't>,
    borrow_arg: usize,
    move_arg: usize,
  },
}

impl<'s, 't> BorrowErrorKind<'s, 't> {
  /// Wrap this violation into a compile error located at `range`.
  pub fn at<'ctx>(
    self,
    compiler: &Compiler<'s, 'ctx, 't>,
    range: RangeS<'s>,
  ) -> ICompileErrorT<'s, 't> {
    ICompileErrorT::BorrowCheckError {
      range: compiler.typing_interner.alloc_slice_copy(&[range]),
      kind: self,
    }
  }

  /// Render this violation for a human.
  pub fn humanize(&self) -> String {
    match self {
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
    }
  }
}

/// The source name of a local, when it has one, for diagnostics.
fn var_name<'s, 't>(name: &IVarNameT<'s, 't>) -> &'s str {
  match name {
    IVarNameT::CodeVar(code_var) => code_var.name.0,
    _ => "a local",
  }
}
