use crate::postparsing::ast::{FunctionS, ImplS, InterfaceS, StructS};
use crate::typing::templata::templata::ITemplataT;
use std::hash::Hash;
use std::hash::Hasher;
use std::mem::discriminant;
use std::ptr::eq;



/// Polyvalue (see @TFITCX) — derive Eq/Hash; never hand-roll `ptr::eq` on the outer `&self` (see @PVECFPZ).
#[derive(Copy, Clone, Debug)]
pub enum IEnvEntryT<'s, 't>
where 's: 't,
{
  Function(&'s FunctionS<'s>),
  Struct(&'s StructS<'s>),
  Interface(&'s InterfaceS<'s>),
  Impl(&'s ImplS<'s>),
  Templata(ITemplataT<'s, 't>),
}


// FunctionS/StructS/InterfaceS/ImplS are arena-allocated (ATDCX) and don't
// derive PartialEq/Eq/Hash. Compare/hash those variants by pointer identity;
// ITemplataT is itself Eq+Hash.
impl<'s, 't> PartialEq for IEnvEntryT<'s, 't>
where 's: 't,
{
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (IEnvEntryT::Function(a), IEnvEntryT::Function(b)) => eq(*a, *b),
      (IEnvEntryT::Struct(a), IEnvEntryT::Struct(b)) => eq(*a, *b),
      (IEnvEntryT::Interface(a), IEnvEntryT::Interface(b)) => eq(*a, *b),
      (IEnvEntryT::Impl(a), IEnvEntryT::Impl(b)) => eq(*a, *b),
      (IEnvEntryT::Templata(a), IEnvEntryT::Templata(b)) => a == b,
      _ => false,
    }
  }
  
}

impl<'s, 't> Eq for IEnvEntryT<'s, 't> where 's: 't {}
impl<'s, 't> Hash for IEnvEntryT<'s, 't>
where 's: 't,
{
  fn hash<H: Hasher>(&self, state: &mut H) {
    discriminant(self).hash(state);
    match self {
      IEnvEntryT::Function(a) => (*a as *const FunctionS<'s>).hash(state),
      IEnvEntryT::Struct(a) => (*a as *const StructS<'s>).hash(state),
      IEnvEntryT::Interface(a) => (*a as *const InterfaceS<'s>).hash(state),
      IEnvEntryT::Impl(a) => (*a as *const ImplS<'s>).hash(state),
      IEnvEntryT::Templata(t) => t.hash(state),
    }
  }
  
}





