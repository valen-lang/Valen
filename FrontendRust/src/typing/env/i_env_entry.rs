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
pub enum IEnvEntry {}
/*
sealed trait IEnvEntry
*/
pub struct FunctionEnvEntry<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
/*
// We dont have the unevaluatedContainers in here because see TMRE
case class FunctionEnvEntry(function: FunctionA) extends IEnvEntry {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
*/
pub struct ImplEnvEntry;
/*
case class ImplEnvEntry(impl: ImplA) extends IEnvEntry {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/
pub struct StructEnvEntry;
/*
case class StructEnvEntry(struct: StructA) extends IEnvEntry {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/
pub struct InterfaceEnvEntry;
/*
case class InterfaceEnvEntry(interface: InterfaceA) extends IEnvEntry {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/
pub struct TemplataEnvEntry;
/*
case class TemplataEnvEntry(templata: ITemplataT[ITemplataType]) extends IEnvEntry {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  vpass()
}
*/