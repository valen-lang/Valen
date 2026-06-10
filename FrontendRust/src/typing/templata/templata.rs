use crate::interner::StrI;
use crate::higher_typing::ast::*;
use crate::postparsing::itemplatatype::{
  BooleanTemplataType, CoordTemplataType, ITemplataType, ImplTemplataType,
  IntegerTemplataType, KindTemplataType, LocationTemplataType,
  MutabilityTemplataType, OwnershipTemplataType, PrototypeTemplataType,
  StringTemplataType, TemplateTemplataType, VariabilityTemplataType,
};
use crate::typing::ast::ast::{FunctionHeaderT, PrototypeT};
use crate::typing::env::environment::*;
use crate::typing::names::names::IdT;
use crate::typing::types::types::*;
use crate::utils::range::RangeS;
use crate::scout_arena::ScoutArena;
use crate::higher_typing::ast::CitizenA;
use std::fmt::Debug;
use std::fmt::Formatter;
use std::fmt::Result;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;

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

pub fn expect_mutability<'s, 't>(templata: ITemplataT<'s, 't>) -> ITemplataT<'s, 't> {
  match templata {
    // case t @ MutabilityTemplataT(_) => t
    t @ ITemplataT::Mutability(_) => t,
    // case PlaceholderTemplataT(idT, MutabilityTemplataType()) => PlaceholderTemplataT(idT, MutabilityTemplataType())
    ITemplataT::Placeholder(p) if matches!(p.tyype, ITemplataType::MutabilityTemplataType(_)) => templata,
    // case _ => vfail()
    _ => panic!("expect_mutability: not a mutability"),
  }
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
pub fn expect_variability<'s, 't>(templata: ITemplataT<'s, 't>) -> ITemplataT<'s, 't> {
  match templata {
    // case t @ VariabilityTemplataT(_) => t
    t @ ITemplataT::Variability(_) => t,
    // case PlaceholderTemplataT(idT, VariabilityTemplataType()) => PlaceholderTemplataT(idT, VariabilityTemplataType())
    ITemplataT::Placeholder(p) if matches!(p.tyype, ITemplataType::VariabilityTemplataType(_)) => templata,
    // case _ => vfail()
    _ => panic!("expect_variability: not a variability"),
  }
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
pub fn expect_integer<'s, 't>(templata: ITemplataT<'s, 't>) -> ITemplataT<'s, 't> {
  match templata {
    // case t @ IntegerTemplataT(_) => t
    t @ ITemplataT::Integer(_) => t,
    // case PlaceholderTemplataT(idT, IntegerTemplataType()) => PlaceholderTemplataT(idT, IntegerTemplataType())
    ITemplataT::Placeholder(p) if matches!(p.tyype, ITemplataType::IntegerTemplataType(_)) => templata,
    // case other => vfail(other)
    _ => panic!("expect_integer: not an integer"),
  }
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
pub fn expect_coord_templata<'s, 't>(templata: ITemplataT<'s, 't>) -> CoordTemplataT<'s, 't> {
  match templata {
    // case t @ CoordTemplataT(_) => t
    ITemplataT::Coord(t) => *t,
    // case other => vfail(other)
    _ => panic!("expect_coord_templata: not a coord"),
  }
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
// Interned payloads behind &'t; scalar variants inline. See @WVSBIZ for why.
/// Polyvalue (see @TFITCX) — derive Eq/Hash; never hand-roll `ptr::eq` on the outer `&self` (see @PVECFPZ).
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
  Location(LocationTemplataT),
}
impl<'s, 't> ITemplataT<'s, 't> where 's: 't {
  pub fn tyype(&self, scout_arena: &ScoutArena<'s>) -> ITemplataType<'s> {
    match self {
      ITemplataT::Coord(_) => ITemplataType::CoordTemplataType(CoordTemplataType {}),
      ITemplataT::Kind(_) => ITemplataType::KindTemplataType(KindTemplataType {}),
      ITemplataT::Placeholder(p) => p.tyype,
      ITemplataT::Mutability(_) => ITemplataType::MutabilityTemplataType(MutabilityTemplataType {}),
      ITemplataT::Variability(_) => ITemplataType::VariabilityTemplataType(VariabilityTemplataType {}),
      ITemplataT::Ownership(_) => ITemplataType::OwnershipTemplataType(OwnershipTemplataType {}),
      ITemplataT::Integer(_) => ITemplataType::IntegerTemplataType(IntegerTemplataType {}),
      ITemplataT::Boolean(_) => ITemplataType::BooleanTemplataType(BooleanTemplataType {}),
      ITemplataT::String(_) => ITemplataType::StringTemplataType(StringTemplataType {}),
      ITemplataT::Prototype(_) => ITemplataType::PrototypeTemplataType(PrototypeTemplataType {}),
      ITemplataT::Isa(_) => ITemplataType::ImplTemplataType(ImplTemplataType {}),
      ITemplataT::ImplDefinition(_) => ITemplataType::ImplTemplataType(ImplTemplataType {}),
      ITemplataT::Location(_) => ITemplataType::LocationTemplataType(LocationTemplataType {}),
      ITemplataT::CoordList(_) => panic!("Unimplemented: tyype on CoordList"),
      ITemplataT::RuntimeSizedArrayTemplate(_) => ITemplataType::TemplateTemplataType(TemplateTemplataType {
        param_types: scout_arena.alloc_slice_copy(&[
          ITemplataType::MutabilityTemplataType(MutabilityTemplataType {}),
          ITemplataType::CoordTemplataType(CoordTemplataType {}),
        ]),
        return_type: scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {})),
      }),
      ITemplataT::StaticSizedArrayTemplate(_) => ITemplataType::TemplateTemplataType(TemplateTemplataType {
        param_types: scout_arena.alloc_slice_copy(&[
          ITemplataType::IntegerTemplataType(IntegerTemplataType {}),
          ITemplataType::MutabilityTemplataType(MutabilityTemplataType {}),
          ITemplataType::VariabilityTemplataType(VariabilityTemplataType {}),
          ITemplataType::CoordTemplataType(CoordTemplataType {}),
        ]),
        return_type: scout_arena.alloc(ITemplataType::KindTemplataType(KindTemplataType {})),
      }),
      ITemplataT::Function(_) => panic!("Unimplemented: tyype on Function"),
      // Note that this might disagree with originStruct.tyype, which might not be a TemplateTemplataType().
      // In Compiler, StructTemplatas are templates, even if they have zero arguments.
      ITemplataT::StructDefinition(s) => ITemplataType::TemplateTemplataType(s.origin_struct.tyype),
      // Note that this might disagree with originStruct.tyype, which might not be a TemplateTemplataType().
      // In Compiler, InterfaceTemplatas are templates, even if they have zero arguments.
      ITemplataT::InterfaceDefinition(i) => ITemplataType::TemplateTemplataType(i.origin_interface.tyype),
      ITemplataT::ExternFunction(_) => panic!("Unimplemented: tyype on ExternFunction"),
    }
  }
}
/*
sealed trait ITemplataT[+T <: ITemplataType]  {
  def tyype: T
}

*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CoordTemplataT<'s, 't> {
  pub coord: CoordT<'s, 't>,
}
/*
case class CoordTemplataT(coord: CoordT) extends ITemplataT[CoordTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: CoordTemplataType = CoordTemplataType()

  vpass()
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PlaceholderTemplataT<'s, 't> {
  pub id: IdT<'s, 't>,
  pub tyype: ITemplataType<'s>,
}
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
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct KindTemplataT<'s, 't> {
  pub kind: KindT<'s, 't>,
}
/*
case class KindTemplataT(kind: KindT) extends ITemplataT[KindTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: KindTemplataType = KindTemplataType()
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct RuntimeSizedArrayTemplateTemplataT<'s, 't> {
  pub _phantom: PhantomData<(&'s (), &'t ())>,
}
/*
case class RuntimeSizedArrayTemplateTemplataT() extends ITemplataT[TemplateTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: TemplateTemplataType = TemplateTemplataType(Vector(MutabilityTemplataType(), CoordTemplataType()), KindTemplataType())
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StaticSizedArrayTemplateTemplataT<'s, 't> {
  pub _phantom: PhantomData<(&'s (), &'t ())>,
}
/*
case class StaticSizedArrayTemplateTemplataT() extends ITemplataT[TemplateTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: TemplateTemplataType = TemplateTemplataType(Vector(IntegerTemplataType(), MutabilityTemplataType(), VariabilityTemplataType(), CoordTemplataType()), KindTemplataType())
}



*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, Debug)]
pub struct FunctionTemplataT<'s, 't> {
  pub outer_env: IEnvironmentT<'s, 't>,
  pub function: &'s FunctionA<'s>,
}
impl<'s, 't> PartialEq for FunctionTemplataT<'s, 't> {
  fn eq(&self, other: &Self) -> bool {
    self.function.range == other.function.range
      && self.function.name == other.function.name
  }
  /* Guardian: disable-all */
}
impl<'s, 't> Eq for FunctionTemplataT<'s, 't> {}
impl<'s, 't> Hash for FunctionTemplataT<'s, 't> {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.function.range.hash(state);
    self.function.name.hash(state);
  }
  /* Guardian: disable-all */
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
*/
/*
  vassert(outerEnv.id.packageCoord == function.name.packageCoordinate)

*/
/*
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

*/
/*
  // Make sure we didn't accidentally code something to include the function's name as
  // the last step.
  // This assertion is helpful now, but will false-positive trip when someone
  // tries to make an interface with the same name as its containing. At that point,
  // feel free to remove this assertion.
  (outerEnv.id.localName, function.name) match {
    case (FunctionNameT(envFunctionName, _, _), FunctionNameS(sourceName, _)) => vassert(envFunctionName != sourceName)
    case _ =>
  }

*/
impl<'s, 't> FunctionTemplataT<'s, 't> where 's: 't {
  pub fn get_template_name(&self) -> IdT<'s, 't> {
    panic!("Unimplemented: get_template_name");
  }
  /*
  def getTemplateName(): IdT[INameT] = {
    vimpl()
//    outerEnv.fullName.addStep(nameTranslator.translateFunctionNameToTemplateName(function.name))
  }
  */
  pub fn debug_string(&self) -> String {
    panic!("Unimplemented: debug_string");
  }
  /*
  def debugString: String = outerEnv.id + ":" + function.name
  */
}
/*
}

*/
// AFTERM: figure out why some templatas compare environment and some don't —
// `FunctionTemplataT.equals` ignores `outerEnv` (Scala templata.scala:161-169)
// but this type's derived equality includes `declaring_env`.
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StructDefinitionTemplataT<'s, 't> {
  pub declaring_env: IEnvironmentT<'s, 't>,
  pub origin_struct: &'s StructA<'s>,
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
*/
/*
  override def originCitizen: CitizenA = originStruct

*/
/*
  vassert(declaringEnv.id.packageCoord == originStruct.name.range.file.packageCoordinate)

*/
/*
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

*/
/*
  // Make sure we didn't accidentally code something to include the structs's name as
  // the last step.
  // This assertion is helpful now, but will false-positive trip when someone
  // tries to make an interface with the same name as its containing. At that point,
  // feel free to remove this assertion.
  (declaringEnv.id.localName, originStruct.name) match {
    case (CitizenNameT(envFunctionName, _), TopLevelCitizenDeclarationNameS(sourceName, _)) => vassert(envFunctionName != sourceName)
    case _ =>
  }

*/
/*
  def debugString: String = declaringEnv.id + ":" + originStruct.name
}

*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum IContainer<'s> {
  Interface(ContainerInterface<'s>),
  Struct(ContainerStruct<'s>),
  Function(ContainerFunction<'s>),
  Impl(ContainerImpl<'s>),
}
/*
sealed trait IContainer
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ContainerInterface<'s> {
  pub interface: &'s InterfaceA<'s>,
}
/*
case class ContainerInterface(interface: InterfaceA) extends IContainer {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ContainerStruct<'s> {
  pub struct_: &'s StructA<'s>,
}
/*
case class ContainerStruct(struct: StructA) extends IContainer {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ContainerFunction<'s> {
  pub function: &'s FunctionA<'s>,
}
/*
case class ContainerFunction(function: FunctionA) extends IContainer {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ContainerImpl<'s> {
  pub impl_: &'s ImplA<'s>,
}
/*
case class ContainerImpl(impl: ImplA) extends IContainer {
  val hash = runtime.ScalaRunTime._hashCode(this);
override def hashCode(): Int = hash; }

*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum CitizenDefinitionTemplataT<'s, 't> {
  Struct(&'t StructDefinitionTemplataT<'s, 't>),
  Interface(&'t InterfaceDefinitionTemplataT<'s, 't>),
}
/*
sealed trait CitizenDefinitionTemplataT extends ITemplataT[TemplateTemplataType] {
*/
impl<'s, 't> CitizenDefinitionTemplataT<'s, 't> where 's: 't {
  pub fn declaring_env(&self) -> IEnvironmentT<'s, 't> {
    panic!("Unimplemented: declaring_env");
  }
  /*
  def declaringEnv: IEnvironmentT
  */
  pub fn origin_citizen(&self) -> &'s dyn CitizenA<'s> {
    panic!("Unimplemented: origin_citizen");
  }
  /*
  def originCitizen: CitizenA
  */
}
/*
}
*/
/*
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
// AFTERM: figure out why some templatas compare environment and some don't —
// `FunctionTemplataT.equals` ignores `outerEnv` (Scala templata.scala:161-169)
// but this type's derived equality includes `declaring_env`.
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct InterfaceDefinitionTemplataT<'s, 't> {
  pub declaring_env: IEnvironmentT<'s, 't>,
  pub origin_interface: &'s InterfaceA<'s>,
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
*/
/*
  override def originCitizen: CitizenA = originInterface

*/
/*
  vassert(declaringEnv.id.packageCoord == originInterface.name.range.file.packageCoordinate)

  vpass()
*/
/*
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: TemplateTemplataType = {
    // Note that this might disagree with originStruct.tyype, which might not be a TemplateTemplataType().
    // In Compiler, StructTemplatas are templates, even if they have zero arguments.
    TemplateTemplataType(
      originInterface.genericParameters.map(_.rune.rune).map(originInterface.runeToType),
      KindTemplataType())
  }

*/
/*
  // Make sure we didn't accidentally code something to include the interface's name as
  // the last step.
  // This assertion is helpful now, but will false-positive trip when someone
  // tries to make an interface with the same name as its containing. At that point,
  // feel free to remove this assertion.
  (declaringEnv.id.localName, originInterface.name) match {
    case (CitizenNameT(envFunctionName, _), TopLevelCitizenDeclarationNameS(sourceName, _)) => vassert(envFunctionName != sourceName)
    case _ =>
  }

*/
/*


  def getTemplateName(): INameT = {
    InterfaceTemplateNameT(originInterface.name.name)//, nameTranslator.translateCodeLocation(originInterface.name.range.begin))
  }

  def debugString: String = declaringEnv.id + ":" + originInterface.name
}

*/
// AFTERM: figure out why some templatas compare environment and some don't —
// `FunctionTemplataT.equals` ignores `outerEnv` (Scala templata.scala:161-169)
// but this type's derived equality includes `env`.
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ImplDefinitionTemplataT<'s, 't> {
  pub env: IEnvironmentT<'s, 't>,
  pub impl_: &'s ImplA<'s>,
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
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct OwnershipTemplataT {
    pub ownership: OwnershipT,
}
/*
case class OwnershipTemplataT(ownership: OwnershipT) extends ITemplataT[OwnershipTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: OwnershipTemplataType = OwnershipTemplataType()
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct VariabilityTemplataT {
    pub variability: VariabilityT,
}
/*
case class VariabilityTemplataT(variability: VariabilityT) extends ITemplataT[VariabilityTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: VariabilityTemplataType = VariabilityTemplataType()
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct MutabilityTemplataT {
    pub mutability: MutabilityT,
}
/*
case class MutabilityTemplataT(mutability: MutabilityT) extends ITemplataT[MutabilityTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: MutabilityTemplataType = MutabilityTemplataType()
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct LocationTemplataT {
    pub location: LocationT,
}
/*
case class LocationTemplataT(location: LocationT) extends ITemplataT[LocationTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: LocationTemplataType = LocationTemplataType()
}

*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct BooleanTemplataT {
    pub value: bool,
}
/*
case class BooleanTemplataT(value: Boolean) extends ITemplataT[BooleanTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: BooleanTemplataType = BooleanTemplataType()
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IntegerTemplataT {
    pub value: i64,
}
/*
case class IntegerTemplataT(value: Long) extends ITemplataT[IntegerTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: IntegerTemplataType = IntegerTemplataType()
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct StringTemplataT<'s> {
    pub value: StrI<'s>,
}
/*
case class StringTemplataT(value: String) extends ITemplataT[StringTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: StringTemplataType = StringTemplataType()
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrototypeTemplataT<'s, 't> {
  pub prototype: &'t PrototypeT<'s, 't>,
}
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
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IsaTemplataT<'s, 't> {
  pub declaration_range: RangeS<'s>,
  pub impl_name: IdT<'s, 't>,
  pub sub_kind: KindT<'s, 't>,
  pub super_kind: KindT<'s, 't>,
}
/*
case class IsaTemplataT(declarationRange: RangeS, implName: IdT[IImplNameT], subKind: KindT, superKind: KindT) extends ITemplataT[ImplTemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: ImplTemplataType = ImplTemplataType()
}
*/
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct CoordListTemplataT<'s, 't> {
  pub coords: &'t [CoordT<'s, 't>],
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
/// Value-type (see @TFITCX)
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct ExternFunctionTemplataT<'s, 't> {
  pub header: &'t FunctionHeaderT<'s, 't>,
}
/*
case class ExternFunctionTemplataT(header: FunctionHeaderT) extends ITemplataT[ITemplataType] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
  override def tyype: ITemplataType = vfail()
}
*/
// FunctionHeaderT doesn't derive Debug yet; render by content (id) for @IIIOZ
// cross-run determinism — pointer addresses vary across runs due to ASLR.
impl<'s, 't> Debug for ExternFunctionTemplataT<'s, 't> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    f.debug_struct("ExternFunctionTemplataT")
      .field("header_id", &self.header.id)
      .finish()
  }
  /* Guardian: disable-all */
}

// (Templata payload interning family removed — types are TFITCX Value-type per
// Scala parity. Construction goes via `bump.alloc(FooTemplataT { ... })`.)
