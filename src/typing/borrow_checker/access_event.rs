use crate::typing::ast::ast::LocT;
use crate::typing::borrow_checker::ast_g::GroupStep;
use crate::typing::names::names::IVarNameT;

pub enum AccessEventG<'s, 't> {
  Read {
    base_ref: IVarNameT<'s, 't>,
    group: Vec<GroupStep<'s, 't>>,
    loct: LocT<'t>,
  },
  Store {
    base_ref: IVarNameT<'s, 't>,
    group: Vec<GroupStep<'s, 't>>,
    loct: LocT<'t>,
  },
  Call {
    touched: Vec<Vec<GroupStep<'s, 't>>>,
    loct: LocT<'t>,
  },
  Marker { value: i32 }
}
