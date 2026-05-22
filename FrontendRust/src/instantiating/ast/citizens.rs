/*
package dev.vale.instantiating.ast

import dev.vale.postparsing._
import dev.vale._

import scala.collection.immutable.Map

// A "citizen" is a struct or an interface.
*/
// mig: trait CitizenDefinitionI
pub trait CitizenDefinitionI<'s, 't> {}
/*
trait CitizenDefinitionI {
//  def genericParamTypes: Vector[ITemplataType]
  def instantiatedCitizen: ICitizenIT[cI]
}
*/
// mig: struct StructDefinitionI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct StructDefinitionI<'s, 't> {
    pub instantiated_citizen: (),
    pub attributes: (),
    pub weakable: (),
    pub mutability: (),
    pub members: (),
    pub is_closure: (),
    pub rune_to_function_bound: (),
    pub rune_to_impl_bound: (),
}
// mig: impl StructDefinitionI
/*
case class StructDefinitionI(
//  templateName: IdI[cI, IStructTemplateNameI],
  // In typing pass, this will have placeholders. Monomorphizing will give it a real name.
  instantiatedCitizen: StructIT[cI],
  attributes: Vector[ICitizenAttributeI],
  weakable: Boolean,
  mutability: MutabilityI,
  members: Vector[StructMemberI],
  isClosure: Boolean,
  runeToFunctionBound: Map[IRuneS, IdI[cI, FunctionBoundNameI[cI]]],
  runeToImplBound: Map[IRuneS, IdI[cI, ImplBoundNameI[cI]]],
) extends CitizenDefinitionI {
//  override def genericParamTypes: Vector[ITemplataType] = {
//    instantiatedCitizen.id.localName.templateArgs.map(_.tyype)
//  }
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for StructDefinitionI` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for StructDefinitionI` below.)
/*
override def hashCode(): Int = vcurious()

//  override def getRef: StructIT = ref
//
//  def getMember(memberName: StrI): StructMemberI = {
//    members.find(p => p.name.equals(CodeVarNameI(memberName))) match {
//      case None => vfail("Couldn't find member " + memberName)
//      case Some(member) => member
//    }
//  }
//
//  private def getIndex(memberName: IVarNameI): Int = {
//    members.zipWithIndex.find(p => p._1.name.equals(memberName)) match {
//      case None => vfail("wat")
//      case Some((member, index)) => index
//    }
//  }
*/
// mig: fn get_member_and_index
impl<'s, 't> StructDefinitionI<'s, 't> {
    pub fn get_member_and_index(&self, needle_name: ()) -> Option<()> {
        panic!("Unimplemented: get_member_and_index")
    }
}
/*
  def getMemberAndIndex(needleName: IVarNameI[cI]): Option[(StructMemberI, Int)] = {
    members.zipWithIndex
      .foreach({
        case (m @ StructMemberI(hayName, _, _), index) if hayName == needleName => {
          return Some((m, index))
        }
        case _ =>
      })
    None
  }
}
*/
// mig: struct StructMemberI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct StructMemberI<'s, 't> {
    pub name: (),
    pub variability: (),
    pub tyype: (),
}
// mig: impl StructMemberI
/*
case class StructMemberI(
  name: IVarNameI[cI],
  // In the case of address members, this refers to the variability of the pointee variable.
  variability: VariabilityI,
  tyype: IMemberTypeI
) {
  vpass()
}
*/
// mig: enum IMemberTypeI
/// Polyvalue
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
pub enum IMemberTypeI<'s, 't> {
    ReferenceMemberTypeI(&'t ReferenceMemberTypeI<'s, 't>),
    AddressMemberTypeI(&'t AddressMemberTypeI<'s, 't>),
}
// mig: impl IMemberTypeI
/*
sealed trait IMemberTypeI  {
*/
// mig: fn reference
/* Guardian: disable-all */
impl<'s, 't> IMemberTypeI<'s, 't> {
    pub fn reference(&self) -> () {
        match self {
            _ => panic!("Unimplemented: IMemberTypeI::reference dispatch"),
        }
    }
}
/*
  def reference: CoordI[cI]
*/
// mig: fn expect_reference_member
/* Guardian: disable-all */
impl<'s, 't> IMemberTypeI<'s, 't> {
    pub fn expect_reference_member(&self) -> () {
        match self {
            _ => panic!("Unimplemented: IMemberTypeI::expect_reference_member dispatch"),
        }
    }
}
/*
  def expectReferenceMember(): ReferenceMemberTypeI = {
    this match {
      case r @ ReferenceMemberTypeI(_) => r
      case a @ AddressMemberTypeI(_) => vfail("Expected reference member, was address member!")
    }
  }
*/
// mig: fn expect_address_member
/* Guardian: disable-all */
impl<'s, 't> IMemberTypeI<'s, 't> {
    pub fn expect_address_member(&self) -> () {
        match self {
            _ => panic!("Unimplemented: IMemberTypeI::expect_address_member dispatch"),
        }
    }
}
/*
  def expectAddressMember(): AddressMemberTypeI = {
    this match {
      case r @ ReferenceMemberTypeI(_) => vfail("Expected reference member, was address member!")
      case a @ AddressMemberTypeI(_) => a
    }
  }
}
*/
// mig: struct AddressMemberTypeI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct AddressMemberTypeI<'s, 't> {
    pub reference: (),
}
// mig: impl AddressMemberTypeI
/*
case class AddressMemberTypeI(reference: CoordI[cI]) extends IMemberTypeI
*/
// mig: struct ReferenceMemberTypeI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ReferenceMemberTypeI<'s, 't> {
    pub reference: (),
}
// mig: impl ReferenceMemberTypeI
/*
case class ReferenceMemberTypeI(reference: CoordI[cI]) extends IMemberTypeI
*/
// mig: struct InterfaceDefinitionI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct InterfaceDefinitionI<'s, 't> {
    pub instantiated_interface: (),
    pub attributes: (),
    pub weakable: (),
    pub mutability: (),
    pub rune_to_function_bound: (),
    pub rune_to_impl_bound: (),
    pub internal_methods: (),
}
// mig: impl InterfaceDefinitionI
/*
case class InterfaceDefinitionI(
//  templateName: IdI[cI, IInterfaceTemplateNameI],
  instantiatedInterface: InterfaceIT[cI],
//  ref: InterfaceIT,
  attributes: Vector[ICitizenAttributeI],
  weakable: Boolean,
  mutability: MutabilityI,
  runeToFunctionBound: Map[IRuneS, IdI[cI, FunctionBoundNameI[cI]]],
  runeToImplBound: Map[IRuneS, IdI[cI, ImplBoundNameI[cI]]],
  // This does not include abstract functions declared outside the interface.
  // Note from later: Though, sometimes macros add functions into the inside.
  // See IMRFDI for why we need to remember only the internal methods here.
  internalMethods: Vector[(PrototypeI[cI], Int)]
) extends CitizenDefinitionI {
//  override def genericParamTypes: Vector[ITemplataType] = {
//    instantiatedCitizen.id.localName.templateArgs.map(_.tyype)
//  }
*/
// mig: fn instantiated_citizen
impl<'s, 't> InterfaceDefinitionI<'s, 't> {
    pub fn instantiated_citizen(&self) -> () {
        panic!("Unimplemented: instantiated_citizen")
    }
}
/*
  override def instantiatedCitizen: ICitizenIT[cI] = instantiatedInterface
*/
// mig: fn eq (realized-by-impl PartialEq)
// (Realized by `impl PartialEq for InterfaceDefinitionI` below.)
/*
  override def equals(obj: Any): Boolean = vcurious();
*/
// mig: fn hash_code (realized-by-impl Hash)
// (Realized by `impl Hash for InterfaceDefinitionI` below.)
/*
override def hashCode(): Int = vcurious()
//  override def getRef = ref
}
*/