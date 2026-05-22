/*
package dev.vale.instantiating.ast

import dev.vale.typing.ast.{FunctionHeaderT, FunctionDefinitionT, PrototypeT}
import dev.vale.typing.names._
import dev.vale.typing.ast._
import dev.vale.typing.names._

object simpleNameI {
*/
// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom for IdI` below.)
/*
  def unapply[R <: IRegionsModeI](id: IdI[R, INameI[R]]): Option[String] = {
    id.localName match {
//      case ImplDeclareNameI(_) => None
      case LambdaCallFunctionNameI(_, _, _) => Some("__call")
      case LetNameI(_) => None
      case UnnamedLocalNameI(_) => None
      case FunctionBoundNameI(FunctionBoundTemplateNameI(humanName), _, _) => Some(humanName.str)
      case ClosureParamNameI(_) => None
      case MagicParamNameI(_) => None
      case CodeVarNameI(name) => Some(name.str)
      case FunctionNameIX(FunctionTemplateNameI(humanName, _), _, _) => Some(humanName.str)
      case LambdaCitizenNameI(_) => None
      case StructNameI(StructTemplateNameI(humanName), _) => Some(humanName.str)
      case StructTemplateNameI(humanName) => Some(humanName.str)
      case InterfaceNameI(InterfaceTemplateNameI(humanName), _) => Some(humanName.str)
      case InterfaceTemplateNameI(humanName) => Some(humanName.str)
      case AnonymousSubstructTemplateNameI(InterfaceTemplateNameI(humanNamee)) => Some(humanNamee.str)
    }
  }
}

object functionNameI {
*/
// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom for FunctionDefinitionI` below.)
/*
  def unapply(function2: FunctionDefinitionI): Option[String] = {
    unapply(function2.header)
  }
*/
// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom for FunctionHeaderI` below.)
/*
  def unapply(header: FunctionHeaderI): Option[String] = {
    simpleNameI.unapply(header.id)
  }
*/
// mig: fn unapply (realized-by-TryFrom)
// (Realized via `impl TryFrom for PrototypeI` below.)
/*
  def unapply[R <: IRegionsModeI](prototype: PrototypeI[R]): Option[String] = {
    simpleNameI.unapply(prototype.id)
  }
}
*/