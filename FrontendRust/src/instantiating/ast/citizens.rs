use crate::utils::arena_index_map::ArenaIndexMap;
use crate::postparsing::names::IRuneS;
use crate::instantiating::ast::types::{cI, CoordI, MutabilityI, VariabilityI, StructIT, InterfaceIT, ICitizenIT};
use crate::instantiating::ast::names::{IdI, IVarNameI};
use crate::instantiating::ast::ast::{ICitizenAttributeI, PrototypeI};
use std::marker::PhantomData;

/*
package dev.vale.instantiating.ast

import dev.vale.postparsing._
import dev.vale._

import scala.collection.immutable.Map

// A "citizen" is a struct or an interface.
*/
// mig: trait CitizenDefinitionI
pub trait CitizenDefinitionI<'s, 'i, R> {}
/*
trait CitizenDefinitionI {
//  def genericParamTypes: Vector[ITemplataType]
  def instantiatedCitizen: ICitizenIT[cI]
}
*/
// Rust-only dispatch enum for the `CitizenDefinitionI` trait (architect-directed).
// Scala uses `CitizenDefinitionI` polymorphically (StructDefinitionI / InterfaceDefinitionI
// both extend it); per NEDCX we use a concrete enum here instead of a `&dyn` trait object.
// Mirrors the kind-level `ICitizenIT` dispatch enum. No Scala counterpart, so no audit-trail.
#[derive(Copy, Clone)]
pub enum ICitizenDefinitionI<'s, 'i, R> {
    StructDefinitionI(&'i StructDefinitionI<'s, 'i, R>),
    InterfaceDefinitionI(&'i InterfaceDefinitionI<'s, 'i, R>),
}
// mig: struct StructDefinitionI
/// Temporary state
pub struct StructDefinitionI<'s, 'i, R> {
    pub instantiated_citizen: &'i StructIT<'s, 'i, cI>,
    pub attributes: &'i [ICitizenAttributeI<'s>],
    pub weakable: bool,
    pub mutability: MutabilityI,
    pub members: &'i [StructMemberI<'s, 'i, R>],
    pub is_closure: bool,
    pub rune_to_function_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i, cI>>,
    pub rune_to_impl_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i, cI>>,
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
impl<'s, 'i, R> StructDefinitionI<'s, 'i, R> {
    pub fn get_member_and_index(&self, needle_name: IVarNameI<'s, 'i, cI>) -> Option<(&StructMemberI<'s, 'i, R>, usize)> {
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
pub struct StructMemberI<'s, 'i, R> {
    pub name: IVarNameI<'s, 'i, cI>,
    pub variability: VariabilityI,
    pub tyype: IMemberTypeI<'s, 'i, R>,
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
pub enum IMemberTypeI<'s, 'i, R> {
    ReferenceMemberTypeI(&'i ReferenceMemberTypeI<'s, 'i, R>),
    AddressMemberTypeI(&'i AddressMemberTypeI<'s, 'i, R>),
}
// mig: impl IMemberTypeI
/*
sealed trait IMemberTypeI  {
*/
// mig: fn expect_reference_member
/* Guardian: disable-all */
impl<'s, 'i, R> IMemberTypeI<'s, 'i, R> {
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
impl<'s, 'i, R> IMemberTypeI<'s, 'i, R> {
    pub fn expect_address_member(&self) -> &'i AddressMemberTypeI<'s, 'i, R> {
        match *self {
            // BUG: Scala message says "Expected reference member, was address member!" but function is expectAddressMember; preserving Scala wording.
            IMemberTypeI::ReferenceMemberTypeI(_) => panic!("Expected reference member, was address member!"),
            IMemberTypeI::AddressMemberTypeI(a) => a,
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
pub struct AddressMemberTypeI<'s, 'i, R> {
    pub reference: CoordI<'s, 'i, cI>,
    pub _marker: PhantomData<R>,
}
// mig: impl AddressMemberTypeI
/*
case class AddressMemberTypeI(reference: CoordI[cI]) extends IMemberTypeI
*/
// mig: struct ReferenceMemberTypeI
/// Temporary state
#[derive(PartialEq, Eq, Hash)]
pub struct ReferenceMemberTypeI<'s, 'i, R> {
    pub reference: CoordI<'s, 'i, cI>,
    pub _marker: PhantomData<R>,
}
// mig: impl ReferenceMemberTypeI
/*
case class ReferenceMemberTypeI(reference: CoordI[cI]) extends IMemberTypeI
*/
// mig: struct InterfaceDefinitionI
/// Temporary state
pub struct InterfaceDefinitionI<'s, 'i, R> {
    pub instantiated_interface: &'i InterfaceIT<'s, 'i, cI>,
    pub attributes: &'i [ICitizenAttributeI<'s>],
    pub weakable: bool,
    pub mutability: MutabilityI,
    pub rune_to_function_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i, cI>>,
    pub rune_to_impl_bound: ArenaIndexMap<'i, IRuneS<'s>, IdI<'s, 'i, cI>>,
    pub internal_methods: &'i [(&'i PrototypeI<'s, 'i, cI>, i32)],
    pub _marker: PhantomData<R>,
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
impl<'s, 'i, R> InterfaceDefinitionI<'s, 'i, R> {
    pub fn instantiated_citizen(&self) -> ICitizenIT<'s, 'i, cI> {
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