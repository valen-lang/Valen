use crate::instantiating::ast::types::{CoordI, KindIT, OwnershipI, MutabilityI, VariabilityI, LocationI};
use crate::instantiating::ast::ast::{FunctionHeaderI, PrototypeI};
use crate::instantiating::ast::names::{IdI, IImplNameI};
use crate::interner::StrI;
use crate::utils::range::RangeS;
use crate::postparsing::itemplatatype::TemplateTemplataType;
use crate::typing::types::types::{KindT, RegionT};
use std::marker::PhantomData;

/*
package dev.vale.instantiating.ast

import dev.vale.postparsing._
import dev.vale.typing.env._
import dev.vale.typing.names._
import dev.vale.typing.types._
import dev.vale.{RangeS, StrI, vassert, vfail, vimpl, vpass, vwat}
import dev.vale.highertyping._
import dev.vale.typing.ast._
import dev.vale.typing.env._
import dev.vale.typing.types._

import scala.collection.immutable.List


object ITemplataI {
*/
// mig: fn expect_coord
pub fn expect_coord<'s, 'i>(templata: ITemplataI<'s, 'i>) -> ITemplataI<'s, 'i> { panic!("Unimplemented: expect_coord"); }
/*
  def expectCoord[R <: IRegionsModeI](templata: ITemplataI[R]): ITemplataI[R] = {
    templata match {
      case t @ CoordTemplataI(_, _) => t
      case other => vfail(other)
    }
  }
*/
// mig: fn expect_coord_templata
pub fn expect_coord_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> CoordTemplataI<'s, 'i> {
    match templata {
        ITemplataI::Coord(t) => t,
        _ => panic!("expect_coord_templata: not a CoordTemplataI"),
    }
}
/*
  def expectCoordTemplata[R <: IRegionsModeI](templata: ITemplataI[R]): CoordTemplataI[R] = {
    templata match {
      case t @ CoordTemplataI(_, _) => t
      case other => vfail(other)
    }
  }
*/
// mig: fn expect_integer_templata
pub fn expect_integer_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> IntegerTemplataI {
    match templata {
        ITemplataI::Integer(t) => t,
        _ => panic!("vfail"),
    }
}
/*
  def expectIntegerTemplata[R <: IRegionsModeI](templata: ITemplataI[R]): IntegerTemplataI[R] = {
    templata match {
      case t @ IntegerTemplataI(_) => t
      case _ => vfail()
    }
  }
*/
// mig: fn expect_mutability_templata
pub fn expect_mutability_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> MutabilityTemplataI {
    match templata {
        ITemplataI::Mutability(m) => m,
        _ => panic!("expect_mutability_templata: not a MutabilityTemplataI"),
    }
}
/*
  def expectMutabilityTemplata[R <: IRegionsModeI](templata: ITemplataI[R]): MutabilityTemplataI[R] = {
    templata match {
      case t @ MutabilityTemplataI(_) => t
      case _ => vfail()
    }
  }
*/
// mig: fn expect_variability_templata
pub fn expect_variability_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> VariabilityTemplataI {
    match templata {
        ITemplataI::Variability(t) => t,
        _ => panic!("expect_variability_templata: not a VariabilityTemplataI"),
    }
}
/*
  def expectVariabilityTemplata[R <: IRegionsModeI](templata: ITemplataI[R]): VariabilityTemplataI[R] = {
    templata match {
      case t @ VariabilityTemplataI(_) => t
      case _ => vfail()
    }
  }
*/
// mig: fn expect_kind
pub fn expect_kind<'s, 'i>(templata: ITemplataI<'s, 'i>) -> ITemplataI<'s, 'i> { panic!("Unimplemented: expect_kind"); }
/*
  def expectKind[R <: IRegionsModeI](templata: ITemplataI[R]): ITemplataI[R] = {
    templata match {
      case t @ KindTemplataI(_) => t
      case _ => vfail()
    }
  }
*/
// mig: fn expect_kind_templata
pub fn expect_kind_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> KindTemplataI<'s, 'i> { panic!("Unimplemented: expect_kind_templata"); }
/*
  def expectKindTemplata[R <: IRegionsModeI](templata: ITemplataI[R]): KindTemplataI[R] = {
    templata match {
      case t @ KindTemplataI(_) => t
      case _ => vfail()
    }
  }
*/
// mig: fn expect_region_templata
pub fn expect_region_templata<'s, 'i>(templata: ITemplataI<'s, 'i>) -> RegionT { panic!("Unimplemented: expect_region_templata"); }
/*
  def expectRegionTemplata[R <: IRegionsModeI](templata: ITemplataI[R]): RegionTemplataI[R] = {
    templata match {
      case t @ RegionTemplataI(_) => t
      case _ => vfail()
    }
  }

}

*/
// mig: enum ITemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum ITemplataI<'s, 'i> {
  Coord(CoordTemplataI<'s, 'i>),
  Kind(KindTemplataI<'s, 'i>),
  RuntimeSizedArrayTemplate(RuntimeSizedArrayTemplateTemplataI),
  StaticSizedArrayTemplate(StaticSizedArrayTemplateTemplataI),
  Function(FunctionTemplataI<'s, 'i>),
  StructDefinition(StructDefinitionTemplataI<'s, 'i>),
  InterfaceDefinition(InterfaceDefinitionTemplataI<'s, 'i>),
  ImplDefinition(ImplDefinitionTemplataI<'s, 'i>),
  Ownership(OwnershipTemplataI),
  Variability(VariabilityTemplataI),
  Mutability(MutabilityTemplataI),
  Location(LocationTemplataI),
  Boolean(BooleanTemplataI),
  Integer(IntegerTemplataI),
  String(StringTemplataI<'s>),
  Prototype(PrototypeTemplataI<'s, 'i>),
  Isa(IsaTemplataI<'s, 'i>),
  CoordList(CoordListTemplataI<'s, 'i>),
  Region(RegionT),
  ExternFunction(ExternFunctionTemplataI<'s, 'i>),
}
// mig: impl ITemplataI
/*
sealed trait ITemplataI[+R <: IRegionsModeI] {
*/
// mig: fn expect_coord_templata
impl<'s, 'i> ITemplataI<'s, 'i> {
  pub fn expect_coord_templata(&self) -> CoordTemplataI<'s, 'i> { panic!("Unimplemented: expect_coord_templata"); }
/*
  def expectCoordTemplata(): CoordTemplataI[R] = {
    this match {
      case c@CoordTemplataI(_, _) => c
      case other => vwat(other)
    }
  }
*/
// mig: fn expect_region_templata
  pub fn expect_region_templata(&self) -> RegionT { panic!("Unimplemented: expect_region_templata"); }
}
/*
  def expectRegionTemplata(): RegionTemplataI[R] = {
    this match {
      case c@RegionTemplataI(_) => c
      case other => vwat(other)
    }
  }
}

//// The typing phase never makes one of these, they're purely abstract and conceptual in the
//// typing phase. The monomorphizer is the one that actually makes these templatas.
//case class RegionTemplataI[+R <: IRegionsModeI](pureHeight: Int) extends ITemplataI[R] {
//  vpass()
//  val hash = runtime.ScalaRunTime._hashCode(this);
//override def hashCode(): Int = hash;
//
//}

*/
// mig: struct CoordTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct CoordTemplataI<'s, 'i> {
  pub region: RegionT,
  pub coord: CoordI<'s, 'i>,
}
// mig: impl CoordTemplataI
/*
case class CoordTemplataI[+R <: IRegionsModeI](
    region: RegionTemplataI[R],
    coord: CoordI[R]
) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

  this match {
    case CoordTemplataI(RegionTemplataI(-1), CoordI(ImmutableShareI, StrIT())) => {
      vpass()
    }
    case _ =>
  }

  vpass()
}
//case class PlaceholderTemplataI[+T <: ITemplataType](
//  fullNameT: IdI[R, IPlaceholderNameI],
//  tyype: T
//) extends ITemplataI[R] {
//  tyype match {
//    case CoordTemplataType() => vwat()
//    case KindTemplataType() => vwat()
//    case _ =>
//  }
//  val hash = runtime.ScalaRunTime._hashCode(this);
//override def hashCode(): Int = hash;
//}
*/
// mig: struct KindTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct KindTemplataI<'s, 'i> {
  pub kind: KindIT<'s, 'i>,
}
// mig: impl KindTemplataI
/*
case class KindTemplataI[+R <: IRegionsModeI](kind: KindIT[R]) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}
*/
// mig: struct RuntimeSizedArrayTemplateTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct RuntimeSizedArrayTemplateTemplataI {
}
// mig: impl RuntimeSizedArrayTemplateTemplataI
/*
case class RuntimeSizedArrayTemplateTemplataI[+R <: IRegionsModeI]() extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}
*/
// mig: struct StaticSizedArrayTemplateTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct StaticSizedArrayTemplateTemplataI {
}
// mig: impl StaticSizedArrayTemplateTemplataI
/*
case class StaticSizedArrayTemplateTemplataI[+R <: IRegionsModeI]() extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}



*/
// mig: struct FunctionTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct FunctionTemplataI<'s, 'i> {
  pub env_id: IdI<'s, 'i>,
}
// mig: impl FunctionTemplataI
/*
case class FunctionTemplataI[+R <: IRegionsModeI](
  envId: IdI[R, FunctionTemplateNameI[R]]
) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;


*/
// mig: fn get_template_name
impl<'s, 'i> FunctionTemplataI<'s, 'i> {
  pub fn get_template_name(&self) -> IdI<'s, 'i> { panic!("Unimplemented: get_template_name"); }
}
/*
  def getTemplateName(): IdI[R, INameI[R]] = vimpl()
}

*/
// mig: struct StructDefinitionTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct StructDefinitionTemplataI<'s, 'i> {
  pub env_id: IdI<'s, 'i>,
  pub tyype: TemplateTemplataType<'s>,
}
// mig: impl StructDefinitionTemplataI
/*
case class StructDefinitionTemplataI[+R <: IRegionsModeI](
  envId: IdI[R, StructTemplateNameI[R]],
  tyype: TemplateTemplataType
) extends CitizenDefinitionTemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}

*/
// mig: enum CitizenDefinitionTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum CitizenDefinitionTemplataI<'s, 'i> {
  Struct(StructDefinitionTemplataI<'s, 'i>),
  Interface(InterfaceDefinitionTemplataI<'s, 'i>),
}
// mig: impl CitizenDefinitionTemplataI
/*
sealed trait CitizenDefinitionTemplataI[+R <: IRegionsModeI] extends ITemplataI[R]

*/
// mig: struct InterfaceDefinitionTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct InterfaceDefinitionTemplataI<'s, 'i> {
  pub env_id: IdI<'s, 'i>,
  pub tyype: TemplateTemplataType<'s>,
}
// mig: impl InterfaceDefinitionTemplataI
/*
case class InterfaceDefinitionTemplataI[+R <: IRegionsModeI](
  envId: IdI[R, InterfaceTemplateNameI[R]],
  tyype: TemplateTemplataType
) extends CitizenDefinitionTemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;
}

*/
// mig: struct ImplDefinitionTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct ImplDefinitionTemplataI<'s, 'i> {
  pub env_id: IdI<'s, 'i>,
}
// mig: impl ImplDefinitionTemplataI
/*
case class ImplDefinitionTemplataI[+R <: IRegionsModeI](
  envId: IdI[R, INameI[R]]
//  // The paackage this interface was declared in.
//  // See TMRE for more on these environments.
//  env: IEnvironment,
////
////  // The containers are the structs/interfaces/impls/functions that this thing is inside.
////  // E.g. if LinkedList has a Node substruct, then the Node's templata will have one
////  // container, the LinkedList.
////  // See NTKPRR for why we have these parents.
////  containers: Vector[IContainer],
//
//  // This is the impl that the interface came from originally. It has all the parent
//  // structs and interfaces. See NTKPRR for more.
//  impl: ImplA
) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}

*/
// mig: struct OwnershipTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct OwnershipTemplataI {
  pub ownership: OwnershipI,
}
// mig: impl OwnershipTemplataI
/*
case class OwnershipTemplataI[+R <: IRegionsModeI](ownership: OwnershipI) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}
*/
// mig: struct VariabilityTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct VariabilityTemplataI {
  pub variability: VariabilityI,
}
// mig: impl VariabilityTemplataI
/*
case class VariabilityTemplataI[+R <: IRegionsModeI](variability: VariabilityI) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}
*/
// mig: struct MutabilityTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct MutabilityTemplataI {
  pub mutability: MutabilityI,
}
// mig: impl MutabilityTemplataI
/*
case class MutabilityTemplataI[+R <: IRegionsModeI](mutability: MutabilityI) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}
*/
// mig: struct LocationTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct LocationTemplataI {
  pub location: LocationI,
}
// mig: impl LocationTemplataI
/*
case class LocationTemplataI[+R <: IRegionsModeI](location: LocationI) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}

*/
// mig: struct BooleanTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct BooleanTemplataI {
  pub value: bool,
}
// mig: impl BooleanTemplataI
/*
case class BooleanTemplataI[+R <: IRegionsModeI](value: Boolean) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}
*/
// mig: struct IntegerTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct IntegerTemplataI {
  pub value: i64,
}
// mig: impl IntegerTemplataI
/*
case class IntegerTemplataI[+R <: IRegionsModeI](value: Long) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}
*/
// mig: struct StringTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct StringTemplataI<'s> {
  pub value: StrI<'s>,
}
// mig: impl StringTemplataI
/*
case class StringTemplataI[+R <: IRegionsModeI](value: String) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}
*/
// mig: struct PrototypeTemplataI
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct PrototypeTemplataI<'s, 'i> {
  pub declaration_range: RangeS<'s>,
  pub prototype: &'i PrototypeI<'s, 'i>,
}
// mig: impl PrototypeTemplataI
/*
case class PrototypeTemplataI[+R <: IRegionsModeI](declarationRange: RangeS, prototype: PrototypeI[R]) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}
*/
// mig: struct IsaTemplataI
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct IsaTemplataI<'s, 'i> {
  pub declaration_range: RangeS<'s>,
  pub impl_name: IdI<'s, 'i>,
  pub sub_kind: KindIT<'s, 'i>,
  pub super_kind: KindIT<'s, 'i>,
}
// mig: impl IsaTemplataI
/*
case class IsaTemplataI[+R <: IRegionsModeI](declarationRange: RangeS, implName: IdI[R, IImplNameI[R]], subKind: KindIT[R], superKind: KindIT[R]) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}
*/
// mig: struct CoordListTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct CoordListTemplataI<'s, 'i> {
  pub coords: &'i[CoordI<'s, 'i>],
}
// mig: impl CoordListTemplataI
/*
case class CoordListTemplataI[+R <: IRegionsModeI](coords: Vector[CoordI[R]]) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

  vpass()
}
*/
/*
case class RegionTemplataI[+R <: IRegionsModeI](pureHeight: Int) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}

// ExternFunction/ImplTemplata are here because for example when we create an anonymous interface
// substruct, we want to add its forwarding functions and its impl to the environment, but it's
// very difficult to add the ImplA and FunctionA for those. So, we allow having coutputs like
// these directly in the environment.
// These should probably be renamed from Extern to something else... they could be supplied
// by plugins, but theyre also used internally.

*/
// mig: struct ExternFunctionTemplataI
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub struct ExternFunctionTemplataI<'s, 'i> {
  pub header: &'i FunctionHeaderI<'s, 'i>,
}
// mig: impl ExternFunctionTemplataI
/*
case class ExternFunctionTemplataI[+R <: IRegionsModeI](header: FunctionHeaderI) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}
*/
