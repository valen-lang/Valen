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
  /// A reference into a child group (a runtime-sized array element) is used after a call churned the
  /// parent group, which may have moved or deleted the element.
  UseAfterChurn {
    local: IVarNameT<'s, 't>,
  },
  /// Like `UseAfterChurn`, but the churned reference is an unnamed held register (a temporary waiting
  /// in a register to be passed to a call), so there is no local name to point at.
  UseAfterChurnTemporary,
  /// A function's return type is a borrow with no group (`&T` rather than `&T in g`). Signature-only:
  /// callers derive the returned reference's group from this annotation, so it must be present.
  GrouplessReturnBorrow,
  /// An expression produces a borrow whose group could not be derived (looking only at signatures) —
  /// it came out empty. Every borrow must carry a real group.
  UnderivableBorrowGroup,
  /// A call churns a group reached through one of the enclosing function's parameters, but that
  /// function's signature declares no `mut(...)` effect covering it — so callers cannot see the churn.
  /// Every parameter-group churn must be declared.
  UndeclaredChurn,
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
}

/// The source name of a local, when it has one, for diagnostics.
fn var_name<'s, 't>(name: &IVarNameT<'s, 't>) -> &'s str {
  match name {
    IVarNameT::Member(code_var) => code_var.imprecise_name.name.0,
    IVarNameT::Local(code_var) => code_var.imprecise_name.name.0,
    _ => "a local",
  }
}
