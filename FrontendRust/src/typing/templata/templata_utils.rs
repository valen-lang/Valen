use crate::typing::ast::ast::*;
use crate::typing::names::names::*;

/*
package dev.vale.typing.templata

import dev.vale.typing.ast.{FunctionHeaderT, FunctionDefinitionT, PrototypeT}
import dev.vale.typing.names._
import dev.vale.typing.ast._
import dev.vale.typing.names._

object simpleNameT {
*/
pub fn unapply_simple_name<'s, 't>(id: &IdT<'s, 't>) -> Option<String>
where 's: 't,
{
  match id.local_name {
    INameT::LambdaCallFunction(_) => Some("__call".to_string()),
    INameT::Let(_) => None,
    INameT::UnnamedLocal(_) => None,
    INameT::FunctionBound(n) => Some(n.template.human_name.as_str().to_string()),
    INameT::ClosureParam(_) => None,
    INameT::MagicParam(_) => None,
    INameT::CodeVar(n) => Some(n.name.as_str().to_string()),
    INameT::Function(n) => Some(n.template.human_name.as_str().to_string()),
    INameT::LambdaCitizen(_) => None,
    INameT::Struct(n) => match n.template {
      IStructTemplateNameT::StructTemplate(st) => Some(st.human_name.as_str().to_string()),
      _ => unreachable!(),
    },
    INameT::StructTemplate(st) => Some(st.human_name.as_str().to_string()),
    INameT::Interface(n) => Some(n.template.human_namee.as_str().to_string()),
    INameT::InterfaceTemplate(it) => Some(it.human_namee.as_str().to_string()),
    INameT::AnonymousSubstructTemplate(n) => match n.interface {
      IInterfaceTemplateNameT::InterfaceTemplate(it) => Some(it.human_namee.as_str().to_string()),
    },
    _ => panic!("Unimplemented: unapply_simple_name for {:?}", id.local_name),
  }
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
pub fn unapply_function_name_def<'s, 't>(function2: &FunctionDefinitionT<'s, 't>) -> Option<String> {
  unapply_function_name_header(&function2.header)
}
/*
  def unapply(function2: FunctionDefinitionT): Option[String] = {
    unapply(function2.header)
  }
*/
pub fn unapply_function_name_header<'s, 't>(header: &FunctionHeaderT<'s, 't>) -> Option<String> {
  unapply_simple_name(&header.id)
}
/*
  def unapply(header: FunctionHeaderT): Option[String] = {
    simpleNameT.unapply(header.id)
  }
*/
pub fn unapply_function_name_prototype<'s, 't>(prototype: &PrototypeT<'s, 't>) -> Option<String> {
  unapply_simple_name(&prototype.id)
}
/*
  def unapply(prototype: PrototypeT[IFunctionNameT]): Option[String] = {
    simpleNameT.unapply(prototype.id)
  }
}

*/
