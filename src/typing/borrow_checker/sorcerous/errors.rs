use crate::typing::borrow_checker::borrow_error::BorrowErrorKind;
use crate::utils::range::RangeS;

pub fn humanize_borrow_error<'s, 't>(
  _range: &&'t [RangeS<'s>],
  kind: &BorrowErrorKind<'s, 't>,
) -> String {
  unimplemented!()
}