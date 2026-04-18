/*
package dev.vale.typing.macros

import dev.vale.{RangeS, StrI}
import dev.vale.typing.CompilerOutputs
import dev.vale.typing.ast.{FunctionHeaderT, LocationInFunctionEnvironmentT, ParameterT}
import dev.vale.typing.env.{FunctionEnvironmentT, IEnvEntry}
import dev.vale.typing.names.{INameT, IdT}
import dev.vale.typing.types._
import dev.vale.RangeS
import dev.vale.highertyping.{FunctionA, ImplA, InterfaceA, StructA}
import dev.vale.postparsing.{LocationInDenizen, MutabilityTemplataType}
import dev.vale.typing.ast._
import dev.vale.typing.env.IEnvEntry
import dev.vale.typing.names.CitizenTemplateNameT
import dev.vale.typing.templata.ITemplataT
import dev.vale.typing.types.InterfaceTT
*/

// mig: trait IFunctionBodyMacro
// Dispatch-tag enum replacing Scala's IFunctionBodyMacro trait; bodies live as
// Compiler::generate_function_body_<suffix> methods.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum FunctionBodyMacro {
    LockWeak,
    AsSubtype,
    StructDrop,
    StructConstructor,
    AbstractBody,
    SameInstance,
    RsaLen,
    RsaMutableNew,
    RsaImmutableNew,
    RsaDropInto,
    RsaMutableCapacity,
    RsaMutablePop,
    RsaMutablePush,
    SsaLen,
    SsaDropInto,
}
/*
trait IFunctionBodyMacro {
//  def generatorId: String

  def generateFunctionBody(
    env: FunctionEnvironmentT,
    coutputs: CompilerOutputs,
    generatorId: StrI,
    life: LocationInFunctionEnvironmentT,
    callRange: List[RangeS],
    callLocation: LocationInDenizen,
    originFunction: Option[FunctionA],
    paramCoords: Vector[ParameterT],
    maybeRetCoord: Option[CoordT]):
  (FunctionHeaderT, ReferenceExpressionTE)
}
*/
// mig: trait IOnStructDefinedMacro
// Dispatch-tag enum replacing Scala's IOnStructDefinedMacro trait; bodies live on impl Compiler.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OnStructDefinedMacro {
    StructConstructor,
    StructDrop,
}
/*
trait IOnStructDefinedMacro {
  def getStructSiblingEntries(
    structName: IdT[INameT], structA: StructA):
  Vector[(IdT[INameT], IEnvEntry)]
}
*/
// mig: trait IOnInterfaceDefinedMacro
// Dispatch-tag enum replacing Scala's IOnInterfaceDefinedMacro trait; bodies live on impl Compiler.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OnInterfaceDefinedMacro {
    AnonymousInterface,
    InterfaceDrop,
}
/*
trait IOnInterfaceDefinedMacro {
  def getInterfaceSiblingEntries(
    interfaceName: IdT[INameT], interfaceA: InterfaceA):
  Vector[(IdT[INameT], IEnvEntry)]
}
*/
// mig: trait IOnImplDefinedMacro
// Dispatch-tag enum replacing Scala's IOnImplDefinedMacro trait; bodies live on impl Compiler.
// (No concrete implementors in the current codebase — Scala initializes this map empty.)
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum OnImplDefinedMacro {}
/*
trait IOnImplDefinedMacro {
  def getImplSiblingEntries(implName: IdT[INameT], implA: ImplA):
  Vector[(IdT[INameT], IEnvEntry)]
}
*/
