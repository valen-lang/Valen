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
pub fn expect_coord<'s, 't>(templata: ITemplataI<'s, 't>) -> ITemplataI<'s, 't> { panic!("Unimplemented: expect_coord"); }
/*
  def expectCoord[R <: IRegionsModeI](templata: ITemplataI[R]): ITemplataI[R] = {
    templata match {
      case t @ CoordTemplataI(_, _) => t
      case other => vfail(other)
    }
  }
*/
// mig: fn expect_coord_templata
pub fn expect_coord_templata<'s, 't>(templata: ITemplataI<'s, 't>) -> CoordTemplataI<'s, 't> { panic!("Unimplemented: expect_coord_templata"); }
/*
  def expectCoordTemplata[R <: IRegionsModeI](templata: ITemplataI[R]): CoordTemplataI[R] = {
    templata match {
      case t @ CoordTemplataI(_, _) => t
      case other => vfail(other)
    }
  }
*/
// mig: fn expect_integer_templata
pub fn expect_integer_templata<'s, 't>(templata: ITemplataI<'s, 't>) -> IntegerTemplataI<'s, 't> { panic!("Unimplemented: expect_integer_templata"); }
/*
  def expectIntegerTemplata[R <: IRegionsModeI](templata: ITemplataI[R]): IntegerTemplataI[R] = {
    templata match {
      case t @ IntegerTemplataI(_) => t
      case _ => vfail()
    }
  }
*/
// mig: fn expect_mutability_templata
pub fn expect_mutability_templata<'s, 't>(templata: ITemplataI<'s, 't>) -> MutabilityTemplataI<'s, 't> { panic!("Unimplemented: expect_mutability_templata"); }
/*
  def expectMutabilityTemplata[R <: IRegionsModeI](templata: ITemplataI[R]): MutabilityTemplataI[R] = {
    templata match {
      case t @ MutabilityTemplataI(_) => t
      case _ => vfail()
    }
  }
*/
// mig: fn expect_variability_templata
pub fn expect_variability_templata<'s, 't>(templata: ITemplataI<'s, 't>) -> VariabilityTemplataI<'s, 't> { panic!("Unimplemented: expect_variability_templata"); }
/*
  def expectVariabilityTemplata[R <: IRegionsModeI](templata: ITemplataI[R]): VariabilityTemplataI[R] = {
    templata match {
      case t @ VariabilityTemplataI(_) => t
      case _ => vfail()
    }
  }
*/
// mig: fn expect_kind
pub fn expect_kind<'s, 't>(templata: ITemplataI<'s, 't>) -> ITemplataI<'s, 't> { panic!("Unimplemented: expect_kind"); }
/*
  def expectKind[R <: IRegionsModeI](templata: ITemplataI[R]): ITemplataI[R] = {
    templata match {
      case t @ KindTemplataI(_) => t
      case _ => vfail()
    }
  }
*/
// mig: fn expect_kind_templata
pub fn expect_kind_templata<'s, 't>(templata: ITemplataI<'s, 't>) -> KindTemplataI<'s, 't> { panic!("Unimplemented: expect_kind_templata"); }
/*
  def expectKindTemplata[R <: IRegionsModeI](templata: ITemplataI[R]): KindTemplataI[R] = {
    templata match {
      case t @ KindTemplataI(_) => t
      case _ => vfail()
    }
  }
*/
// mig: fn expect_region_templata
pub fn expect_region_templata<'s, 't>(templata: ITemplataI<'s, 't>) -> RegionTemplataI<'s, 't> { panic!("Unimplemented: expect_region_templata"); }
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum ITemplataI<'s, 't> {
  // Placeholder variant; will be filled in during reconciliation
  _Placeholder(()),
}
// mig: impl ITemplataI
/*
sealed trait ITemplataI[+R <: IRegionsModeI] {
*/
// mig: fn expect_coord_templata
impl<'s, 't> ITemplataI<'s, 't> {
  pub fn expect_coord_templata(&self) -> CoordTemplataI<'s, 't> { panic!("Unimplemented: expect_coord_templata"); }
}
/*
  def expectCoordTemplata(): CoordTemplataI[R] = {
    this match {
      case c@CoordTemplataI(_, _) => c
      case other => vwat(other)
    }
  }
*/
// mig: fn expect_region_templata
impl<'s, 't> ITemplataI<'s, 't> {
  pub fn expect_region_templata(&self) -> RegionTemplataI<'s, 't> { panic!("Unimplemented: expect_region_templata"); }
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct CoordTemplataI<'s, 't> {
  pub region: RegionTemplataI<'s, 't>,
  pub coord: CoordI<'s, 't>,
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct KindTemplataI<'s, 't> {
  pub kind: KindIT<'s, 't>,
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct RuntimeSizedArrayTemplateTemplataI<'s, 't> {
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct StaticSizedArrayTemplateTemplataI<'s, 't> {
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct FunctionTemplataI<'s, 't> {
  pub env_id: IdI<'s, 't>,
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
impl<'s, 't> FunctionTemplataI<'s, 't> {
  pub fn get_template_name(&self) -> IdI<'s, 't> { panic!("Unimplemented: get_template_name"); }
}
/*
  def getTemplateName(): IdI[R, INameI[R]] = vimpl()
}

*/
// mig: struct StructDefinitionTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct StructDefinitionTemplataI<'s, 't> {
  pub env_id: IdI<'s, 't>,
  pub tyype: TemplateTemplataType,
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum CitizenDefinitionTemplataI<'s, 't> {
  // Placeholder variant; will be filled in during reconciliation
  _Placeholder(()),
}
// mig: impl CitizenDefinitionTemplataI
/*
sealed trait CitizenDefinitionTemplataI[+R <: IRegionsModeI] extends ITemplataI[R]

*/
// mig: struct InterfaceDefinitionTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct InterfaceDefinitionTemplataI<'s, 't> {
  pub env_id: IdI<'s, 't>,
  pub tyype: TemplateTemplataType,
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ImplDefinitionTemplataI<'s, 't> {
  pub env_id: IdI<'s, 't>,
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct OwnershipTemplataI<'s, 't> {
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct VariabilityTemplataI<'s, 't> {
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct MutabilityTemplataI<'s, 't> {
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct LocationTemplataI<'s, 't> {
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct BooleanTemplataI<'s, 't> {
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct IntegerTemplataI<'s, 't> {
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
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct StringTemplataI<'s, 't> {
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
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct PrototypeTemplataI<'s, 't> {
  pub declaration_range: RangeS,
  pub prototype: PrototypeI<'s, 't>,
}
// mig: impl PrototypeTemplataI
/*
case class PrototypeTemplataI[+R <: IRegionsModeI](declarationRange: RangeS, prototype: PrototypeI[R]) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}
*/
// mig: struct IsaTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct IsaTemplataI<'s, 't> {
  pub declaration_range: RangeS,
  pub impl_name: IdI<'s, 't>,
  pub sub_kind: KindT,
  pub super_kind: KindIT<'s, 't>,
}
// mig: impl IsaTemplataI
/*
case class IsaTemplataI[+R <: IRegionsModeI](declarationRange: RangeS, implName: IdI[R, IImplNameI[R]], subKind: KindT, superKind: KindIT[R]) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}
*/
// mig: struct CoordListTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct CoordListTemplataI<'s, 't> {
  pub coords: &'t [CoordI<'s, 't>],
}
// mig: impl CoordListTemplataI
/*
case class CoordListTemplataI[+R <: IRegionsModeI](coords: Vector[CoordI[R]]) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

  vpass()
}
*/
// mig: struct RegionTemplataI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct RegionTemplataI<'s, 't> {
  pub pure_height: i32,
}
// mig: impl RegionTemplataI
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
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub struct ExternFunctionTemplataI<'s, 't> {
  pub header: FunctionHeaderI<'s, 't>,
}
// mig: impl ExternFunctionTemplataI
/*
case class ExternFunctionTemplataI[+R <: IRegionsModeI](header: FunctionHeaderI) extends ITemplataI[R] {
  val hash = runtime.ScalaRunTime._hashCode(this)
  override def hashCode(): Int = hash;

}
*/
