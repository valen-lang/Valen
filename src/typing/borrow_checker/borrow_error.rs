use crate::interner::StrI;
use crate::typing::compiler::Compiler;
use crate::typing::compiler_error_reporter::ICompileErrorT;
use crate::typing::names::names::IVarNameT;
use crate::utils::range::RangeS;

#[derive(Debug)]
pub enum BorrowErrorKind<'s, 't> {
  AliasingIntoDisjointMutGroups {
    local: IVarNameT<'s, 't>,
    arg_a: usize,
    arg_b: usize,
    group_a: StrI<'s>,
    group_b: StrI<'s>,
  },
  BorrowIntoMovedArgument {
    local: IVarNameT<'s, 't>,
    borrow_arg: usize,
    move_arg: usize,
  },
  UseAfterChurn {
    local: IVarNameT<'s, 't>,
  },
  UseAfterChurnTemporary,
  GrouplessReturnBorrow,
  UnderivableBorrowGroup,
  UndeclaredChurn,
}

impl<'s, 't> BorrowErrorKind<'s, 't> {
  pub fn humanize(&self) -> String {
    match self {
      BorrowErrorKind::AliasingIntoDisjointMutGroups { local, arg_a, arg_b, group_a, group_b } => {
        unimplemented!()
        // format!(
        //   "Arguments {} and {} both borrow into {}, but their parameters are in disjoint mutated \
        //    groups {} and {}, which the callee may treat as non-aliasing.",
        //   arg_a,
        //   arg_b,
        //   var_name(local),
        //   group_a.0,
        //   group_b.0,
        // )
      }
      BorrowErrorKind::BorrowIntoMovedArgument { local, borrow_arg, move_arg } => {
        unimplemented!()
        // format!(
        //   "Argument {} borrows into {}, but argument {} moves it, so the borrow would dangle.",
        //   borrow_arg,
        //   var_name(local),
        //   move_arg,
        // )
      }
      BorrowErrorKind::UseAfterChurn { local } => {
        unimplemented!()
        // format!(
        //   "{} references an array element, which a preceding churn of its group may have moved or \
        //    deleted, so it can't be used here.",
        //   var_name(local),
        // )
      }
      BorrowErrorKind::UseAfterChurnTemporary => {
        unimplemented!()
        // "This reference into an array element is held while a sibling argument churns its group, \
        //  which may have moved or deleted the element, so it can't be passed here."
        //   .to_string()
      }
      BorrowErrorKind::GrouplessReturnBorrow => {
        unimplemented!()
        // "This function returns a borrow reference with no group. Annotate the group it points into, \
        //  like `&T in g`."
        //   .to_string()
      }
      BorrowErrorKind::UnderivableBorrowGroup => {
        unimplemented!()
        // "The group of this borrow reference can't be determined from the expression that produces it."
        //   .to_string()
      }
      BorrowErrorKind::UndeclaredChurn => {
        unimplemented!()
        // "this call churns a group reached through a parameter, but the enclosing function does not \
        //  declare a mut effect for it."
        //   .to_string()
      }
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
