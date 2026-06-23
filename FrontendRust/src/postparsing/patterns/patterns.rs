use crate::postparsing::names::IVarNameS;
use crate::postparsing::rules::RuneUsage;
use crate::utils::range::RangeS;


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CaptureS<'s> {
  pub name: IVarNameS<'s>,
  pub mutate: bool,
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct AtomSP<'s> {
  pub range: RangeS<'s>,
  pub name: Option<CaptureS<'s>>,
  pub coord_rune: Option<RuneUsage<'s>>,
  pub destructure: Option<&'s [AtomSP<'s>]>,
}
