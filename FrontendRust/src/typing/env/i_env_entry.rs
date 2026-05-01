use crate::higher_typing::ast::{FunctionA, ImplA, InterfaceA, StructA};
use crate::typing::templata::templata::ITemplataT;

/*
package dev.vale.typing.env

import dev.vale.highertyping.{FunctionA, ImplA, InterfaceA, StructA}
import dev.vale.typing.templata.ITemplataT
import dev.vale.vpass
import dev.vale.highertyping.FunctionA
import dev.vale.postparsing.ITemplataType
import dev.vale.typing.templata.IContainer
import dev.vale.typing.types.InterfaceTT
import dev.vale.vpass
*/

/// Value-type (see @TFITCX)
#[derive(Copy, Clone, Debug)]
pub enum IEnvEntryT<'s, 't>
where 's: 't,
{
  Function(&'s FunctionA<'s>),
  Struct(&'s StructA<'s>),
  Interface(&'s InterfaceA<'s>),
  Impl(&'s ImplA<'s>),
  Templata(ITemplataT<'s, 't>),
}
/*
sealed trait IEnvEntry
*/

// FunctionA/StructA/InterfaceA/ImplA are arena-allocated (ATDCX) and don't
// derive PartialEq/Eq/Hash. Compare/hash those variants by pointer identity;
// ITemplataT is itself Eq+Hash (Slab 3).
impl<'s, 't> PartialEq for IEnvEntryT<'s, 't>
where 's: 't,
{
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (IEnvEntryT::Function(a), IEnvEntryT::Function(b)) => std::ptr::eq(*a, *b),
      (IEnvEntryT::Struct(a), IEnvEntryT::Struct(b)) => std::ptr::eq(*a, *b),
      (IEnvEntryT::Interface(a), IEnvEntryT::Interface(b)) => std::ptr::eq(*a, *b),
      (IEnvEntryT::Impl(a), IEnvEntryT::Impl(b)) => std::ptr::eq(*a, *b),
      (IEnvEntryT::Templata(a), IEnvEntryT::Templata(b)) => a == b,
      _ => false,
    }
  }
  /* Guardian: disable-all */
}

impl<'s, 't> Eq for IEnvEntryT<'s, 't> where 's: 't {}
impl<'s, 't> std::hash::Hash for IEnvEntryT<'s, 't>
where 's: 't,
{
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    std::mem::discriminant(self).hash(state);
    match self {
      IEnvEntryT::Function(a) => (*a as *const FunctionA<'s>).hash(state),
      IEnvEntryT::Struct(a) => (*a as *const StructA<'s>).hash(state),
      IEnvEntryT::Interface(a) => (*a as *const InterfaceA<'s>).hash(state),
      IEnvEntryT::Impl(a) => (*a as *const ImplA<'s>).hash(state),
      IEnvEntryT::Templata(t) => t.hash(state),
    }
  }
  /* Guardian: disable-all */
}
/*
// We dont have the unevaluatedContainers in here because see TMRE
case class FunctionEnvEntry(function: FunctionA) extends IEnvEntry {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
*/
/*
case class ImplEnvEntry(impl: ImplA) extends IEnvEntry {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/
/*
case class StructEnvEntry(struct: StructA) extends IEnvEntry {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/
/*
case class InterfaceEnvEntry(interface: InterfaceA) extends IEnvEntry {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/
/*
case class TemplataEnvEntry(templata: ITemplataT[ITemplataType]) extends IEnvEntry {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  vpass()
}
*/
