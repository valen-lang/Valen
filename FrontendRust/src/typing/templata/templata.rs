/*
package dev.vale.typing.templata

import dev.vale.highertyping.{FunctionA, ImplA, InterfaceA, StructA}
import dev.vale.postparsing._
import dev.vale.typing.ast.{FunctionHeaderT, PrototypeT}
import dev.vale.typing.env.IInDenizenEnvironmentT
import dev.vale.typing.names._
import dev.vale.typing.types._
import dev.vale.{RangeS, StrI, vassert, vfail, vimpl, vpass, vwat}
import dev.vale.highertyping._
import dev.vale.typing.ast._
import dev.vale.typing.env._
import dev.vale.typing.types._

import scala.collection.immutable.List


object ITemplataT {
*/
use crate::interner::StrI;
use crate::higher_typing::ast::*;
use crate::typing::ast::ast::{FunctionHeaderT, PrototypeT};
use crate::typing::env::environment::*;
use crate::typing::names::names::IdT;
use crate::typing::types::types::*;
use crate::utils::range::RangeS;

fn expect_mutability<'s, 't>(templata: ITemplataT<'s, 't>) -> ITemplataT<'s, 't> {
  panic!("Unimplemented: expect_mutability");
}
/*
  def expectMutability(templata: ITemplataT[ITemplataType]): ITemplataT[MutabilityTemplataType] = {
    templata match {
      case t @ MutabilityTemplataT(_) => t
      case PlaceholderTemplataT(idT, MutabilityTemplataType()) => PlaceholderTemplataT(idT, MutabilityTemplataType())
      case _ => vfail()
    }
  }

*/
fn expect_variability<'s, 't>(templata: ITemplataT<'s, 't>) -> ITemplataT<'s, 't> {
  panic!("Unimplemented: expect_variability");
}
/*
  def expectVariability(templata: ITemplataT[ITemplataType]): ITemplataT[VariabilityTemplataType] = {
    templata match {
      case t @ VariabilityTemplataT(_) => t
      case PlaceholderTemplataT(idT, VariabilityTemplataType()) => PlaceholderTemplataT(idT, VariabilityTemplataType())
      case _ => vfail()
    }
  }

*/
fn expect_integer<'s, 't>(templata: ITemplataT<'s, 't>) -> ITemplataT<'s, 't> {
  panic!("Unimplemented: expect_integer");
}
/*
  def expectInteger(templata: ITemplataT[ITemplataType]): ITemplataT[IntegerTemplataType] = {
    templata match {
      case t @ IntegerTemplataT(_) => t
      case PlaceholderTemplataT(idT, IntegerTemplataType()) => PlaceholderTemplataT(idT, IntegerTemplataType())
      case other => vfail(other)
    }
  }

*/
fn expect_coord<'s, 't>(templata: ITemplataT<'s, 't>) -> ITemplataT<'s, 't> {
  panic!("Unimplemented: expect_coord");
}
/*
  def expectCoord(templata: ITemplataT[ITemplataType]): ITemplataT[CoordTemplataType] = {
    templata match {
      case t @ CoordTemplataT(_) => t
      case PlaceholderTemplataT(idT, CoordTemplataType()) => PlaceholderTemplataT(idT, CoordTemplataType())
      case other => vfail(other)
    }
  }

*/
fn expect_coord_templata<'s, 't>(templata: ITemplataT<'s, 't>) -> CoordTemplataT<'s, 't> {
  panic!("Unimplemented: expect_coord_templata");
}
/*
  def expectCoordTemplata(templata: ITemplataT[ITemplataType]): CoordTemplataT = {
    templata match {
      case t @ CoordTemplataT(_) => t
      case other => vfail(other)
    }
  }

*/
fn expect_prototype_templata<'s, 't>(templata: ITemplataT<'s, 't>) -> PrototypeTemplataT<'s, 't> {
  panic!("Unimplemented: expect_prototype_templata");
}
/*
  def expectPrototypeTemplata(templata: ITemplataT[ITemplataType]): PrototypeTemplataT[IFunctionNameT] = {
    templata match {
      case t@PrototypeTemplataT(_) => t
      case other => vfail(other)
    }
  }

*/
fn expect_integer_templata<'s, 't>(templata: ITemplataT<'s, 't>) -> IntegerTemplataT {
  panic!("Unimplemented: expect_integer_templata");
}
/*
  def expectIntegerTemplata(templata: ITemplataT[ITemplataType]): IntegerTemplataT = {
    templata match {
      case t @ IntegerTemplataT(_) => t
      case _ => vfail()
    }
  }

*/
fn expect_mutability_templata<'s, 't>(templata: ITemplataT<'s, 't>) -> MutabilityTemplataT {
  panic!("Unimplemented: expect_mutability_templata");
}
/*
  def expectMutabilityTemplata(templata: ITemplataT[ITemplataType]): MutabilityTemplataT = {
    templata match {
      case t @ MutabilityTemplataT(_) => t
      case _ => vfail()
    }
  }

*/
fn expect_variability_templata<'s, 't>(templata: ITemplataT<'s, 't>) -> ITemplataT<'s, 't> {
  panic!("Unimplemented: expect_variability_templata");
}
/*
  def expectVariabilityTemplata(templata: ITemplataT[ITemplataType]): ITemplataT[VariabilityTemplataType] = {
    templata match {
      case t @ VariabilityTemplataT(_) => t
      case _ => vfail()
    }
  }

*/
fn expect_kind<'s, 't>(templata: ITemplataT<'s, 't>) -> ITemplataT<'s, 't> {
  panic!("Unimplemented: expect_kind");
}
/*
  def expectKind(templata: ITemplataT[ITemplataType]): ITemplataT[KindTemplataType] = {
    templata match {
      case t @ KindTemplataT(_) => t
      case PlaceholderTemplataT(idT, KindTemplataType()) => PlaceholderTemplataT(idT, KindTemplataType())
      case _ => vfail()
    }
  }

*/
fn expect_kind_templata<'s, 't>(templata: ITemplataT<'s, 't>) -> KindTemplataT<'s, 't> {
  panic!("Unimplemented: expect_kind_templata");
}
/*
  def expectKindTemplata(templata: ITemplataT[ITemplataType]): KindTemplataT = {
    templata match {
      case t @ KindTemplataT(_) => t
      case _ => vfail()
    }
  }
}
*/
// Inline-owned wrapper enum per §6.6. Scala's `ITemplataT[+T <: ITemplataType]`
// outer phantom parameter is erased in Rust. Interned payloads are held as
// &'t refs; Copy-value variants are held inline.
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum ITemplataT<'s, 't> {
  Coord(&'t CoordTemplataT<'s, 't>),
  Kind(&'t KindTemplataT<'s, 't>),
  Placeholder(&'t PlaceholderTemplataT<'s, 't>),
  Mutability(MutabilityTemplataT),
  Variability(VariabilityTemplataT),
  Ownership(OwnershipTemplataT),
  Integer(i64),
  Boolean(bool),
  String(StrI<'s>),
  Prototype(&'t PrototypeTemplataT<'s, 't>),
  Isa(&'t IsaTemplataT<'s, 't>),
  CoordList(&'t CoordListTemplataT<'s, 't>),
  RuntimeSizedArrayTemplate(RuntimeSizedArrayTemplateTemplataT<'s, 't>),
  StaticSizedArrayTemplate(StaticSizedArrayTemplateTemplataT<'s, 't>),
  Function(&'t FunctionTemplataT<'s, 't>),
  StructDefinition(&'t StructDefinitionTemplataT<'s, 't>),
  InterfaceDefinition(&'t InterfaceDefinitionTemplataT<'s, 't>),
  ImplDefinition(&'t ImplDefinitionTemplataT<'s, 't>),
  ExternFunction(&'t ExternFunctionTemplataT<'s, 't>),
}
/*
sealed trait ITemplataT[+T <: ITemplataType]  {
  def tyype: T
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CoordTemplataT<'s, 't> {
  pub coord: CoordT<'s, 't>,
}
impl<'s, 't> CoordTemplataT<'s, 't> {}
/*
case class CoordTemplataT(coord: CoordT) extends ITemplataT[CoordTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: CoordTemplataType = CoordTemplataType()

  vpass()
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PlaceholderTemplataT<'s, 't> {
  pub id: IdT<'s, 't>,
  pub tyype: ITemplataT<'s, 't>,
}
impl<'s, 't> PlaceholderTemplataT<'s, 't> {}
/*
case class PlaceholderTemplataT[+T <: ITemplataType](
  idT: IdT[IPlaceholderNameT],
  tyype: T
) extends ITemplataT[T] {
  tyype match {
    case CoordTemplataType() => vwat()
    case KindTemplataType() => vwat()
    case _ =>
  }
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct KindTemplataT<'s, 't> {
  pub kind: KindT<'s, 't>,
}
impl<'s, 't> KindTemplataT<'s, 't> {}
/*
case class KindTemplataT(kind: KindT) extends ITemplataT[KindTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: KindTemplataType = KindTemplataType()
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayTemplateTemplataT<'s, 't> {
  pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
impl<'s, 't> RuntimeSizedArrayTemplateTemplataT<'s, 't> {}
/*
case class RuntimeSizedArrayTemplateTemplataT() extends ITemplataT[TemplateTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: TemplateTemplataType = TemplateTemplataType(Vector(MutabilityTemplataType(), CoordTemplataType()), KindTemplataType())
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayTemplateTemplataT<'s, 't> {
  pub _phantom: std::marker::PhantomData<(&'s (), &'t ())>,
}
impl<'s, 't> StaticSizedArrayTemplateTemplataT<'s, 't> {}
/*
case class StaticSizedArrayTemplateTemplataT() extends ITemplataT[TemplateTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: TemplateTemplataType = TemplateTemplataType(Vector(IntegerTemplataType(), MutabilityTemplataType(), VariabilityTemplataType(), CoordTemplataType()), KindTemplataType())
}



*/
#[derive(Copy, Clone, Debug)]
pub struct FunctionTemplataT<'s, 't> {
  pub outer_env: &'s IEnvironmentT<'s, 't>,
  pub function: &'s FunctionA<'s>,
}
impl<'s, 't> FunctionTemplataT<'s, 't> {}
impl<'s, 't> PartialEq for FunctionTemplataT<'s, 't> {
  fn eq(&self, other: &Self) -> bool {
    std::ptr::eq(self.outer_env, other.outer_env) && std::ptr::eq(self.function, other.function)
  }
}
impl<'s, 't> Eq for FunctionTemplataT<'s, 't> {}
impl<'s, 't> std::hash::Hash for FunctionTemplataT<'s, 't> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    std::ptr::hash(self.outer_env, state);
    std::ptr::hash(self.function, state);
  }
}
/*
case class FunctionTemplataT(
  // The environment this function was declared in.
  // Has the name of the surrounding environment, does NOT include function's name.
  // We need this because, for example, lambdas need to find their underlying struct
  // somewhere.
  // See TMRE for more on these environments.
  outerEnv: IEnvironmentT,

  // This is the env entry that the function came from originally. It has all the parent
  // structs and interfaces. See NTKPRR for more.
  function: FunctionA
) extends ITemplataT[TemplateTemplataType] {
  vassert(outerEnv.id.packageCoord == function.name.packageCoordinate)

  val hash = runtime.ScalaRunTime._hashCode(this)

  override def hashCode(): Int = hash;

  override def equals(obj: Any): Boolean = {
    obj match {
      case FunctionTemplataT(thatEnv, thatFunction) => {
        function.range == thatFunction.range &&
          function.name == thatFunction.name
      }
      case _ => false
    }
  }

  override def tyype: TemplateTemplataType = vfail()

  vpass()

  // Make sure we didn't accidentally code something to include the function's name as
  // the last step.
  // This assertion is helpful now, but will false-positive trip when someone
  // tries to make an interface with the same name as its containing. At that point,
  // feel free to remove this assertion.
  (outerEnv.id.localName, function.name) match {
    case (FunctionNameT(envFunctionName, _, _), FunctionNameS(sourceName, _)) => vassert(envFunctionName != sourceName)
    case _ =>
  }



  def getTemplateName(): IdT[INameT] = {
    vimpl()
//    outerEnv.fullName.addStep(nameTranslator.translateFunctionNameToTemplateName(function.name))
  }

  def debugString: String = outerEnv.id + ":" + function.name
}

*/
#[derive(Copy, Clone, Debug)]
pub struct StructDefinitionTemplataT<'s, 't> {
  pub declaring_env: &'s IEnvironmentT<'s, 't>,
  pub origin_struct: &'s StructA<'s>,
}
impl<'s, 't> StructDefinitionTemplataT<'s, 't> {}
impl<'s, 't> PartialEq for StructDefinitionTemplataT<'s, 't> {
  fn eq(&self, other: &Self) -> bool {
    std::ptr::eq(self.declaring_env, other.declaring_env) && std::ptr::eq(self.origin_struct, other.origin_struct)
  }
}
impl<'s, 't> Eq for StructDefinitionTemplataT<'s, 't> {}
impl<'s, 't> std::hash::Hash for StructDefinitionTemplataT<'s, 't> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    std::ptr::hash(self.declaring_env, state);
    std::ptr::hash(self.origin_struct, state);
  }
}
/*
case class StructDefinitionTemplataT(
  // The paackage this struct was declared in.
  // has the name of the surrounding environment, does NOT include struct's name.
  // See TMRE for more on these environments.
  declaringEnv: IEnvironmentT,

  // This is the env entry that the struct came from originally. It has all the parent
  // structs and interfaces. See NTKPRR for more.
  originStruct: StructA,
) extends CitizenDefinitionTemplataT {
  override def originCitizen: CitizenA = originStruct

  vassert(declaringEnv.id.packageCoord == originStruct.name.range.file.packageCoordinate)

  val hash = runtime.ScalaRunTime._hashCode(this)

  override def hashCode(): Int = hash;
  override def tyype: TemplateTemplataType = {
    // Note that this might disagree with originStruct.tyype, which might not be a TemplateTemplataType().
    // In Compiler, StructTemplatas are templates, even if they have zero arguments.
    val allRuneToType = originStruct.headerRuneToType ++ originStruct.membersRuneToType
    TemplateTemplataType(
      originStruct.genericParameters
        .map(_.rune.rune)
        .map(allRuneToType),
      KindTemplataType())
  }

  // Make sure we didn't accidentally code something to include the structs's name as
  // the last step.
  // This assertion is helpful now, but will false-positive trip when someone
  // tries to make an interface with the same name as its containing. At that point,
  // feel free to remove this assertion.
  (declaringEnv.id.localName, originStruct.name) match {
    case (CitizenNameT(envFunctionName, _), TopLevelCitizenDeclarationNameS(sourceName, _)) => vassert(envFunctionName != sourceName)
    case _ =>
  }

  def debugString: String = declaringEnv.id + ":" + originStruct.name
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IContainer<'s, 't> {
  Interface(ContainerInterface<'s>),
  Struct(ContainerStruct<'s>),
  Function(ContainerFunction<'s>),
  Impl(ContainerImpl<'s>),
  _Phantom(std::marker::PhantomData<&'t ()>),
}
/*
sealed trait IContainer
*/
#[derive(Copy, Clone, Debug)]
pub struct ContainerInterface<'s> {
  pub interface: &'s InterfaceA<'s>,
}
impl<'s> ContainerInterface<'s> {}
impl<'s> PartialEq for ContainerInterface<'s> {
  fn eq(&self, other: &Self) -> bool { std::ptr::eq(self.interface, other.interface) }
}
impl<'s> Eq for ContainerInterface<'s> {}
impl<'s> std::hash::Hash for ContainerInterface<'s> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) { std::ptr::hash(self.interface, state); }
}
/*
case class ContainerInterface(interface: InterfaceA) extends IContainer {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/
#[derive(Copy, Clone, Debug)]
pub struct ContainerStruct<'s> {
  pub struct_: &'s StructA<'s>,
}
impl<'s> ContainerStruct<'s> {}
impl<'s> PartialEq for ContainerStruct<'s> {
  fn eq(&self, other: &Self) -> bool { std::ptr::eq(self.struct_, other.struct_) }
}
impl<'s> Eq for ContainerStruct<'s> {}
impl<'s> std::hash::Hash for ContainerStruct<'s> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) { std::ptr::hash(self.struct_, state); }
}
/*
case class ContainerStruct(struct: StructA) extends IContainer {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/
#[derive(Copy, Clone, Debug)]
pub struct ContainerFunction<'s> {
  pub function: &'s FunctionA<'s>,
}
impl<'s> ContainerFunction<'s> {}
impl<'s> PartialEq for ContainerFunction<'s> {
  fn eq(&self, other: &Self) -> bool { std::ptr::eq(self.function, other.function) }
}
impl<'s> Eq for ContainerFunction<'s> {}
impl<'s> std::hash::Hash for ContainerFunction<'s> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) { std::ptr::hash(self.function, state); }
}
/*
case class ContainerFunction(function: FunctionA) extends IContainer {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/
#[derive(Copy, Clone, Debug)]
pub struct ContainerImpl<'s> {
  pub impl_: &'s ImplA<'s>,
}
impl<'s> ContainerImpl<'s> {}
impl<'s> PartialEq for ContainerImpl<'s> {
  fn eq(&self, other: &Self) -> bool { std::ptr::eq(self.impl_, other.impl_) }
}
impl<'s> Eq for ContainerImpl<'s> {}
impl<'s> std::hash::Hash for ContainerImpl<'s> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) { std::ptr::hash(self.impl_, state); }
}
/*
case class ContainerImpl(impl: ImplA) extends IContainer {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CitizenDefinitionTemplataT<'s, 't> {
  Struct(&'t StructDefinitionTemplataT<'s, 't>),
  Interface(&'t InterfaceDefinitionTemplataT<'s, 't>),
}
impl<'s, 't> CitizenDefinitionTemplataT<'s, 't> {}
/*
sealed trait CitizenDefinitionTemplataT extends ITemplataT[TemplateTemplataType] {
  def declaringEnv: IEnvironmentT
  def originCitizen: CitizenA
}
object CitizenDefinitionTemplataT {
*/
fn unapply<'s, 't>(c: CitizenDefinitionTemplataT<'s, 't>) -> Option<(IEnvironmentT<'s, 't>, &'s dyn CitizenA<'s>)> {
  panic!("Unimplemented: unapply");
}
/*
  def unapply(c: CitizenDefinitionTemplataT): Option[(IEnvironmentT, CitizenA)] = {
    c match {
      case StructDefinitionTemplataT(env, origin) => Some((env, origin))
      case InterfaceDefinitionTemplataT(env, origin) => Some((env, origin))
    }
  }
}

*/
#[derive(Copy, Clone, Debug)]
pub struct InterfaceDefinitionTemplataT<'s, 't> {
  pub declaring_env: &'s IEnvironmentT<'s, 't>,
  pub origin_interface: &'s InterfaceA<'s>,
}
impl<'s, 't> InterfaceDefinitionTemplataT<'s, 't> {}
impl<'s, 't> PartialEq for InterfaceDefinitionTemplataT<'s, 't> {
  fn eq(&self, other: &Self) -> bool {
    std::ptr::eq(self.declaring_env, other.declaring_env) && std::ptr::eq(self.origin_interface, other.origin_interface)
  }
}
impl<'s, 't> Eq for InterfaceDefinitionTemplataT<'s, 't> {}
impl<'s, 't> std::hash::Hash for InterfaceDefinitionTemplataT<'s, 't> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    std::ptr::hash(self.declaring_env, state);
    std::ptr::hash(self.origin_interface, state);
  }
}
/*
case class InterfaceDefinitionTemplataT(
  // The paackage this interface was declared in.
  // Has the name of the surrounding environment, does NOT include interface's name.
  // See TMRE for more on these environments.
  declaringEnv: IEnvironmentT,

  // This is the env entry that the interface came from originally. It has all the parent
  // structs and interfaces. See NTKPRR for more.
  originInterface: InterfaceA
) extends CitizenDefinitionTemplataT {
  override def originCitizen: CitizenA = originInterface

  vassert(declaringEnv.id.packageCoord == originInterface.name.range.file.packageCoordinate)

  vpass()
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: TemplateTemplataType = {
    // Note that this might disagree with originStruct.tyype, which might not be a TemplateTemplataType().
    // In Compiler, StructTemplatas are templates, even if they have zero arguments.
    TemplateTemplataType(
      originInterface.genericParameters.map(_.rune.rune).map(originInterface.runeToType),
      KindTemplataType())
  }

  // Make sure we didn't accidentally code something to include the interface's name as
  // the last step.
  // This assertion is helpful now, but will false-positive trip when someone
  // tries to make an interface with the same name as its containing. At that point,
  // feel free to remove this assertion.
  (declaringEnv.id.localName, originInterface.name) match {
    case (CitizenNameT(envFunctionName, _), TopLevelCitizenDeclarationNameS(sourceName, _)) => vassert(envFunctionName != sourceName)
    case _ =>
  }



  def getTemplateName(): INameT = {
    InterfaceTemplateNameT(originInterface.name.name)//, nameTranslator.translateCodeLocation(originInterface.name.range.begin))
  }

  def debugString: String = declaringEnv.id + ":" + originInterface.name
}

*/
#[derive(Copy, Clone, Debug)]
pub struct ImplDefinitionTemplataT<'s, 't> {
  pub env: &'s IEnvironmentT<'s, 't>,
  pub impl_: &'s ImplA<'s>,
}
impl<'s, 't> ImplDefinitionTemplataT<'s, 't> {}
impl<'s, 't> PartialEq for ImplDefinitionTemplataT<'s, 't> {
  fn eq(&self, other: &Self) -> bool {
    std::ptr::eq(self.env, other.env) && std::ptr::eq(self.impl_, other.impl_)
  }
}
impl<'s, 't> Eq for ImplDefinitionTemplataT<'s, 't> {}
impl<'s, 't> std::hash::Hash for ImplDefinitionTemplataT<'s, 't> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
    std::ptr::hash(self.env, state);
    std::ptr::hash(self.impl_, state);
  }
}
/*
case class ImplDefinitionTemplataT(
  // The paackage this interface was declared in.
  // See TMRE for more on these environments.
  env: IEnvironmentT,
//
//  // The containers are the structs/interfaces/impls/functions that this thing is inside.
//  // E.g. if LinkedList has a Node substruct, then the Node's templata will have one
//  // container, the LinkedList.
//  // See NTKPRR for why we have these parents.
//  containers: Vector[IContainer],

  // This is the impl that the interface came from originally. It has all the parent
  // structs and interfaces. See NTKPRR for more.
  impl: ImplA
) extends ITemplataT[ImplTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: ImplTemplataType = ImplTemplataType()
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OwnershipTemplataT {
    pub ownership: OwnershipT,
}
impl OwnershipTemplataT {}
/*
case class OwnershipTemplataT(ownership: OwnershipT) extends ITemplataT[OwnershipTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: OwnershipTemplataType = OwnershipTemplataType()
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct VariabilityTemplataT {
    pub variability: VariabilityT,
}
impl VariabilityTemplataT {}
/*
case class VariabilityTemplataT(variability: VariabilityT) extends ITemplataT[VariabilityTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: VariabilityTemplataType = VariabilityTemplataType()
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MutabilityTemplataT {
    pub mutability: MutabilityT,
}
impl MutabilityTemplataT {}
/*
case class MutabilityTemplataT(mutability: MutabilityT) extends ITemplataT[MutabilityTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: MutabilityTemplataType = MutabilityTemplataType()
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LocationTemplataT {
    pub location: LocationT,
}
impl LocationTemplataT {}
/*
case class LocationTemplataT(location: LocationT) extends ITemplataT[LocationTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: LocationTemplataType = LocationTemplataType()
}

*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BooleanTemplataT {
    pub value: bool,
}
impl BooleanTemplataT {}
/*
case class BooleanTemplataT(value: Boolean) extends ITemplataT[BooleanTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: BooleanTemplataType = BooleanTemplataType()
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IntegerTemplataT {
    pub value: i64,
}
impl IntegerTemplataT {}
/*
case class IntegerTemplataT(value: Long) extends ITemplataT[IntegerTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: IntegerTemplataType = IntegerTemplataType()
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StringTemplataT<'s> {
    pub value: StrI<'s>,
}
impl<'s> StringTemplataT<'s> {}
/*
case class StringTemplataT(value: String) extends ITemplataT[StringTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: StringTemplataType = StringTemplataType()
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrototypeTemplataT<'s, 't> {
  pub prototype: &'t PrototypeT<'s, 't>,
}
impl<'s, 't> PrototypeTemplataT<'s, 't> {}
/*
case class PrototypeTemplataT[+T <: IFunctionNameT](
    // Removed this because we want to merge different bound functions from different places, see MFBFDP.
    //   declarationRange: RangeS,
    prototype: PrototypeT[T]
) extends ITemplataT[PrototypeTemplataType] {
  vpass()
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: PrototypeTemplataType = PrototypeTemplataType()
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IsaTemplataT<'s, 't> {
  pub declaration_range: RangeS<'s>,
  pub impl_name: IdT<'s, 't>,
  pub sub_kind: KindT<'s, 't>,
  pub super_kind: KindT<'s, 't>,
}
impl<'s, 't> IsaTemplataT<'s, 't> {}
/*
case class IsaTemplataT(declarationRange: RangeS, implName: IdT[IImplNameT], subKind: KindT, superKind: KindT) extends ITemplataT[ImplTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: ImplTemplataType = ImplTemplataType()
}
*/
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CoordListTemplataT<'s, 't> {
  pub coords: &'t [CoordT<'s, 't>],
}
impl<'s, 't> CoordListTemplataT<'s, 't> {}

// Transient Val for interning: holds a stack-borrowed slice (&'tmp) instead of
// the canonical &'t slice. Per @DSAUIMZ / IDEPFL, this lets callers construct a
// lookup key without arena-allocating the coords Vec on a HashMap hit.
#[derive(Copy, Clone, Debug)]
pub struct CoordListTemplataValT<'s, 't, 'tmp>
where 's: 't, 't: 'tmp,
{
  pub coords: &'tmp [CoordT<'s, 't>],
}
/*
case class CoordListTemplataT(coords: Vector[CoordT]) extends ITemplataT[PackTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: PackTemplataType = PackTemplataType(CoordTemplataType())
  vpass()

}

// ExternFunction/ImplTemplata are here because for example when we create an anonymous interface
// substruct, we want to add its forwarding functions and its impl to the environment, but it's
// very difficult to add the ImplA and FunctionA for those. So, we allow having coutputs like
// these directly in the environment.
// These should probably be renamed from Extern to something else... they could be supplied
// by plugins, but theyre also used internally.

*/
#[derive(Copy, Clone)]
pub struct ExternFunctionTemplataT<'s, 't> {
  pub header: &'t FunctionHeaderT<'s, 't>,
}
impl<'s, 't> ExternFunctionTemplataT<'s, 't> {}
impl<'s, 't> PartialEq for ExternFunctionTemplataT<'s, 't> {
  fn eq(&self, other: &Self) -> bool { std::ptr::eq(self.header, other.header) }
}
impl<'s, 't> Eq for ExternFunctionTemplataT<'s, 't> {}
impl<'s, 't> std::hash::Hash for ExternFunctionTemplataT<'s, 't> {
  fn hash<H: std::hash::Hasher>(&self, state: &mut H) { std::ptr::hash(self.header, state); }
}
// FunctionHeaderT doesn't derive Debug yet; treat the header as an opaque ptr.
impl<'s, 't> std::fmt::Debug for ExternFunctionTemplataT<'s, 't> {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("ExternFunctionTemplataT")
      .field("header", &(self.header as *const _))
      .finish()
  }
}
/*
case class ExternFunctionTemplataT(header: FunctionHeaderT) extends ITemplataT[ITemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: ITemplataType = vfail()
}
*/
