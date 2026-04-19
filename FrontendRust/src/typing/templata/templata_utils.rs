/*
package dev.vale.typing.templata

import dev.vale.typing.ast.{FunctionHeaderT, FunctionDefinitionT, PrototypeT}
import dev.vale.typing.names._
import dev.vale.typing.ast._
import dev.vale.typing.names._

object simpleNameT {
*/
use crate::typing::ast::ast::*;
use crate::typing::names::names::*;

pub fn unapply_simple_name<'s, 't>(id: &IdT<'s, 't>) -> Option<String>
where 's: 't,
{
  panic!("Unimplemented: unapply_simple_name");
}
/*
  def unapply(id: IdT[INameT]): Option[String] = {
    id.localName match {
//      case ImplDeclareNameT(_) => None
      case LambdaCallFunctionNameT(_, _, _) => Some("__call")
      case LetNameT(_) => None
      case UnnamedLocalNameT(_) => None
      case FunctionBoundNameT(FunctionBoundTemplateNameT(humanName), _, _) => Some(humanName.str)
      case ClosureParamNameT(_) => None
      case MagicParamNameT(_) => None
      case CodeVarNameT(name) => Some(name.str)
      case FunctionNameT(FunctionTemplateNameT(humanName, _), _, _) => Some(humanName.str)
      case LambdaCitizenNameT(_) => None
      case StructNameT(StructTemplateNameT(humanName), _) => Some(humanName.str)
      case StructTemplateNameT(humanName) => Some(humanName.str)
      case InterfaceNameT(InterfaceTemplateNameT(humanName), _) => Some(humanName.str)
      case InterfaceTemplateNameT(humanName) => Some(humanName.str)
      case AnonymousSubstructTemplateNameT(InterfaceTemplateNameT(humanNamee)) => Some(humanNamee.str)
    }
  }
}

object functionNameT {
*/
pub fn unapply_function_name_def(function2: &FunctionDefinitionT) -> Option<String> {
  panic!("Unimplemented: unapply_function_name_def");
}
/*
  def unapply(function2: FunctionDefinitionT): Option[String] = {
    unapply(function2.header)
  }
*/
pub fn unapply_function_name_header(header: &FunctionHeaderT) -> Option<String> {
  panic!("Unimplemented: unapply_function_name_header");
}
/*
  def unapply(header: FunctionHeaderT): Option[String] = {
    simpleNameT.unapply(header.id)
  }
*/
pub fn unapply_function_name_prototype(prototype: &PrototypeT) -> Option<String> {
  panic!("Unimplemented: unapply_function_name_prototype");
}
/*
  def unapply(prototype: PrototypeT[IFunctionNameT]): Option[String] = {
    simpleNameT.unapply(prototype.id)
  }
}

*/
