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
use crate::higher_typing::ast::*;
use crate::typing::env::environment::*;

// mig: fn expect_mutability
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
// mig: fn expect_variability
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
// mig: fn expect_integer
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
// mig: fn expect_coord
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
// mig: fn expect_coord_templata
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
// mig: fn expect_prototype_templata
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
// mig: fn expect_integer_templata
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
// mig: fn expect_mutability_templata
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
// mig: fn expect_variability_templata
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
// mig: fn expect_kind
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
// mig: fn expect_kind_templata
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
// mig: enum ITemplataT
pub enum ITemplataT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait ITemplataT[+T <: ITemplataType]  {
  def tyype: T
}

*/
// mig: struct CoordTemplataT
pub struct CoordTemplataT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl CoordTemplataT
impl<'s, 't> CoordTemplataT<'s, 't> {}
/*
case class CoordTemplataT(coord: CoordT) extends ITemplataT[CoordTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: CoordTemplataType = CoordTemplataType()

  vpass()
}
*/
// mig: struct PlaceholderTemplataT
pub struct PlaceholderTemplataT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl PlaceholderTemplataT
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
// mig: struct KindTemplataT
pub struct KindTemplataT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl KindTemplataT
impl<'s, 't> KindTemplataT<'s, 't> {}
/*
case class KindTemplataT(kind: KindT) extends ITemplataT[KindTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: KindTemplataType = KindTemplataType()
}
*/
// mig: struct RuntimeSizedArrayTemplateTemplataT
pub struct RuntimeSizedArrayTemplateTemplataT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl RuntimeSizedArrayTemplateTemplataT
impl<'s, 't> RuntimeSizedArrayTemplateTemplataT<'s, 't> {}
/*
case class RuntimeSizedArrayTemplateTemplataT() extends ITemplataT[TemplateTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: TemplateTemplataType = TemplateTemplataType(Vector(MutabilityTemplataType(), CoordTemplataType()), KindTemplataType())
}
*/
// mig: struct StaticSizedArrayTemplateTemplataT
pub struct StaticSizedArrayTemplateTemplataT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl StaticSizedArrayTemplateTemplataT
impl<'s, 't> StaticSizedArrayTemplateTemplataT<'s, 't> {}
/*
case class StaticSizedArrayTemplateTemplataT() extends ITemplataT[TemplateTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: TemplateTemplataType = TemplateTemplataType(Vector(IntegerTemplataType(), MutabilityTemplataType(), VariabilityTemplataType(), CoordTemplataType()), KindTemplataType())
}



*/
// mig: struct FunctionTemplataT
pub struct FunctionTemplataT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl FunctionTemplataT
impl<'s, 't> FunctionTemplataT<'s, 't> {}
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
// mig: struct StructDefinitionTemplataT
pub struct StructDefinitionTemplataT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl StructDefinitionTemplataT
impl<'s, 't> StructDefinitionTemplataT<'s, 't> {}
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
// mig: enum IContainer
pub enum IContainer<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
/*
sealed trait IContainer
*/
// mig: struct ContainerInterface
pub struct ContainerInterface<'s>(pub std::marker::PhantomData<&'s ()>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl ContainerInterface
impl<'s> ContainerInterface<'s> {}
/*
case class ContainerInterface(interface: InterfaceA) extends IContainer {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/
// mig: struct ContainerStruct
pub struct ContainerStruct<'s>(pub std::marker::PhantomData<&'s ()>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl ContainerStruct
impl<'s> ContainerStruct<'s> {}
/*
case class ContainerStruct(struct: StructA) extends IContainer {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/
// mig: struct ContainerFunction
pub struct ContainerFunction<'s>(pub std::marker::PhantomData<&'s ()>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl ContainerFunction
impl<'s> ContainerFunction<'s> {}
/*
case class ContainerFunction(function: FunctionA) extends IContainer {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/
// mig: struct ContainerImpl
pub struct ContainerImpl<'s>(pub std::marker::PhantomData<&'s ()>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl ContainerImpl
impl<'s> ContainerImpl<'s> {}
/*
case class ContainerImpl(impl: ImplA) extends IContainer {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }

*/
// mig: enum CitizenDefinitionTemplataT
pub enum CitizenDefinitionTemplataT<'s, 't> { _Phantom(std::marker::PhantomData<(&'s (), &'t ())>) }
// mig: impl CitizenDefinitionTemplataT
impl<'s, 't> CitizenDefinitionTemplataT<'s, 't> {}
/*
sealed trait CitizenDefinitionTemplataT extends ITemplataT[TemplateTemplataType] {
  def declaringEnv: IEnvironmentT
  def originCitizen: CitizenA
}
object CitizenDefinitionTemplataT {
*/
// mig: fn unapply
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
// mig: struct InterfaceDefinitionTemplataT
pub struct InterfaceDefinitionTemplataT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl InterfaceDefinitionTemplataT
impl<'s, 't> InterfaceDefinitionTemplataT<'s, 't> {}
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
// mig: struct ImplDefinitionTemplataT
pub struct ImplDefinitionTemplataT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl ImplDefinitionTemplataT
impl<'s, 't> ImplDefinitionTemplataT<'s, 't> {}
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
// mig: struct OwnershipTemplataT
pub struct OwnershipTemplataT;
// mig: impl OwnershipTemplataT
impl OwnershipTemplataT {}
/*
case class OwnershipTemplataT(ownership: OwnershipT) extends ITemplataT[OwnershipTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: OwnershipTemplataType = OwnershipTemplataType()
}
*/
// mig: struct VariabilityTemplataT
pub struct VariabilityTemplataT;
// mig: impl VariabilityTemplataT
impl VariabilityTemplataT {}
/*
case class VariabilityTemplataT(variability: VariabilityT) extends ITemplataT[VariabilityTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: VariabilityTemplataType = VariabilityTemplataType()
}
*/
// mig: struct MutabilityTemplataT
pub struct MutabilityTemplataT;
// mig: impl MutabilityTemplataT
impl MutabilityTemplataT {}
/*
case class MutabilityTemplataT(mutability: MutabilityT) extends ITemplataT[MutabilityTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: MutabilityTemplataType = MutabilityTemplataType()
}
*/
// mig: struct LocationTemplataT
pub struct LocationTemplataT;
// mig: impl LocationTemplataT
impl LocationTemplataT {}
/*
case class LocationTemplataT(location: LocationT) extends ITemplataT[LocationTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: LocationTemplataType = LocationTemplataType()
}

*/
// mig: struct BooleanTemplataT
pub struct BooleanTemplataT;
// mig: impl BooleanTemplataT
impl BooleanTemplataT {}
/*
case class BooleanTemplataT(value: Boolean) extends ITemplataT[BooleanTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: BooleanTemplataType = BooleanTemplataType()
}
*/
// mig: struct IntegerTemplataT
pub struct IntegerTemplataT;
// mig: impl IntegerTemplataT
impl IntegerTemplataT {}
/*
case class IntegerTemplataT(value: Long) extends ITemplataT[IntegerTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: IntegerTemplataType = IntegerTemplataType()
}
*/
// mig: struct StringTemplataT
pub struct StringTemplataT;
// mig: impl StringTemplataT
impl StringTemplataT {}
/*
case class StringTemplataT(value: String) extends ITemplataT[StringTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: StringTemplataType = StringTemplataType()
}
*/
// mig: struct PrototypeTemplataT
pub struct PrototypeTemplataT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl PrototypeTemplataT
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
// mig: struct IsaTemplataT
pub struct IsaTemplataT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl IsaTemplataT
impl<'s, 't> IsaTemplataT<'s, 't> {}
/*
case class IsaTemplataT(declarationRange: RangeS, implName: IdT[IImplNameT], subKind: KindT, superKind: KindT) extends ITemplataT[ImplTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: ImplTemplataType = ImplTemplataType()
}
*/
// mig: struct CoordListTemplataT
pub struct CoordListTemplataT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl CoordListTemplataT
impl<'s, 't> CoordListTemplataT<'s, 't> {}
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
// mig: struct ExternFunctionTemplataT
pub struct ExternFunctionTemplataT<'s, 't>(pub std::marker::PhantomData<(&'s (), &'t ())>);
// TODO: placeholder PhantomData — replace with real fields during body migration
// mig: impl ExternFunctionTemplataT
impl<'s, 't> ExternFunctionTemplataT<'s, 't> {}
/*
case class ExternFunctionTemplataT(header: FunctionHeaderT) extends ITemplataT[ITemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: ITemplataType = vfail()
}
*/
