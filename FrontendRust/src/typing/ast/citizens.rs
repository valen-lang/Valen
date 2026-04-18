/*
package dev.vale.typing.ast

import dev.vale.postparsing._
import dev.vale.typing.{InstantiationBoundArgumentsT, TemplataCompiler}
import dev.vale.typing.names._
import dev.vale.typing.templata._
import dev.vale.typing.types._
import dev.vale.{StrI, vcurious, vfail, vpass}

import scala.collection.immutable.Map

// A "citizen" is a struct or an interface.
*/
use crate::typing::names::names::*;
use crate::typing::types::types::*;
use crate::typing::templata::templata::*;
use crate::typing::hinputs_t::*;
use crate::typing::ast::ast::*;
use crate::postparsing::itemplatatype::ITemplataType;

pub enum CitizenDefinitionT<'s, 't> {
    _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
/*
trait CitizenDefinitionT {
*/
fn citizen_definition_template_name<'s, 't>() -> IdT<'s, 't> {
    panic!("Unimplemented: template_name");
}
/*
  def templateName: IdT[ICitizenTemplateNameT]
*/
fn citizen_definition_generic_param_types<'s>() -> Vec<ITemplataType<'s>> {
    panic!("Unimplemented: generic_param_types");
}
/*
  def genericParamTypes: Vector[ITemplataType]
*/
fn citizen_definition_instantiated_citizen<'s, 't>() -> ICitizenTT<'s, 't> {
    panic!("Unimplemented: instantiated_citizen");
}
/*
  def instantiatedCitizen: ICitizenTT
*/
fn citizen_definition_default_region() -> RegionT {
    panic!("Unimplemented: default_region");
}
/*
  def defaultRegion: RegionT
}
*/
pub struct StructDefinitionT<'s, 't> {
    pub template_name: IdT<'s, 't>,
    pub instantiated_citizen: StructTT<'s, 't>,
    pub attributes: Vec<ICitizenAttributeT<'s, 't>>,
    pub weakable: bool,
    pub mutability: ITemplataT<'s, 't>,
    pub members: Vec<IStructMemberT<'s, 't>>,
    pub is_closure: bool,
    pub instantiation_bound_params: InstantiationBoundArgumentsT<'s, 't>,
}
impl<'s, 't> StructDefinitionT<'s, 't> {}
/*
case class StructDefinitionT(
  templateName: IdT[IStructTemplateNameT],
  // In typing pass, this will have placeholders. Monomorphizing will give it a real name.
  instantiatedCitizen: StructTT,
  attributes: Vector[ICitizenAttributeT],
  weakable: Boolean,
  mutability: ITemplataT[MutabilityTemplataType],
  members: Vector[IStructMemberT],
  isClosure: Boolean,
  instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT]
) extends CitizenDefinitionT {
*/
impl<'s, 't> StructDefinitionT<'s, 't> {
    fn default_region(&self) -> RegionT {
        panic!("Unimplemented: default_region");
    }
}
/*
  def defaultRegion: RegionT = RegionT()
*/
impl<'s, 't> StructDefinitionT<'s, 't> {
    fn generic_param_types(&self) -> Vec<ITemplataType<'s>> {
        panic!("Unimplemented: generic_param_types");
    }
}
/*
  override def genericParamTypes: Vector[ITemplataType] = {
    instantiatedCitizen.id.localName.templateArgs.map(_.tyype)
  }
*/
impl<'s, 't> StructDefinitionT<'s, 't> {
    fn equals(&self, obj: &Self) -> bool {
        panic!("Unimplemented: equals");
    }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()

//  override def getRef: StructTT = ref
//
//  def getMember(memberName: StrI): NormalStructMemberT = {
//    members.find(p => p.name.equals(CodeVarNameT(memberName))) match {
//      case None => vfail("Couldn't find member " + memberName)
//      case Some(member) => member
//    }
//  }
//
//  private def getIndex(memberName: IVarNameT): Int = {
//    members.zipWithIndex.find(p => p._1.name.equals(memberName)) match {
//      case None => vfail("wat")
//      case Some((member, index)) => index
//    }
//  }
*/
impl<'s, 't> StructDefinitionT<'s, 't> {
    fn get_member_and_index(&self, needle_name: &IVarNameT<'s, 't>) -> Option<(&NormalStructMemberT<'s, 't>, usize)> {
        panic!("Unimplemented: get_member_and_index");
    }
}
/*
  def getMemberAndIndex(needleName: IVarNameT): Option[(NormalStructMemberT, Int)] = {
    members.zipWithIndex
      .foreach({
        case (m @ NormalStructMemberT(hayName, _, _), index) if hayName == needleName => {
          return Some((m, index))
        }
        case _ =>
      })
    None
  }
}
*/
pub enum IStructMemberT<'s, 't> {
    _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
/*
sealed trait IStructMemberT {
*/
fn struct_member_name<'s, 't>() -> IVarNameT<'s, 't> {
    panic!("Unimplemented: struct_member_name");
}
/*
  def name: IVarNameT
}
*/
pub struct NormalStructMemberT<'s, 't> {
    pub name: IVarNameT<'s, 't>,
    pub variability: VariabilityT,
    pub tyype: IMemberTypeT<'s, 't>,
}
impl<'s, 't> NormalStructMemberT<'s, 't> {
}
/*
case class NormalStructMemberT(
  name: IVarNameT,
  // In the case of address members, this refers to the variability of the pointee variable.
  variability: VariabilityT,
  tyype: IMemberTypeT
) extends IStructMemberT {
  vpass()
}
*/
pub struct VariadicStructMemberT<'s, 't> {
    pub name: IVarNameT<'s, 't>,
    pub tyype: PlaceholderTemplataT<'s, 't>,
}
impl<'s, 't> VariadicStructMemberT<'s, 't> {
}
/*
case class VariadicStructMemberT(
  name: IVarNameT,
  tyype: PlaceholderTemplataT[PackTemplataType]
) extends IStructMemberT {
  vpass()
}
*/
pub enum IMemberTypeT<'s, 't> {
    _Phantom(std::marker::PhantomData<(&'s (), &'t ())>),
}
/*
sealed trait IMemberTypeT  {
*/
fn member_type_reference<'s, 't>() -> CoordT<'s, 't> {
    panic!("Unimplemented: member_type_reference");
}
/*
  def reference: CoordT
*/
fn member_type_expect_reference_member<'s, 't>() -> ReferenceMemberTypeT<'s, 't> {
    panic!("Unimplemented: expect_reference_member");
}
/*
  def expectReferenceMember(): ReferenceMemberTypeT = {
    this match {
      case r @ ReferenceMemberTypeT(_) => r
      case a @ AddressMemberTypeT(_) => vfail("Expected reference member, was address member!")
    }
  }
*/
fn member_type_expect_address_member<'s, 't>() -> AddressMemberTypeT<'s, 't> {
    panic!("Unimplemented: expect_address_member");
}
/*
  def expectAddressMember(): AddressMemberTypeT = {
    this match {
      case r @ ReferenceMemberTypeT(_) => vfail("Expected reference member, was address member!")
      case a @ AddressMemberTypeT(_) => a
    }
  }
}
*/
pub struct AddressMemberTypeT<'s, 't> {
    pub reference: CoordT<'s, 't>,
}
impl<'s, 't> AddressMemberTypeT<'s, 't> {
}
/*
case class AddressMemberTypeT(reference: CoordT) extends IMemberTypeT
*/
pub struct ReferenceMemberTypeT<'s, 't> {
    pub reference: CoordT<'s, 't>,
}
impl<'s, 't> ReferenceMemberTypeT<'s, 't> {
}
/*
case class ReferenceMemberTypeT(reference: CoordT) extends IMemberTypeT
*/
pub struct InterfaceDefinitionT<'s, 't> {
    pub template_name: IdT<'s, 't>,
    pub instantiated_interface: InterfaceTT<'s, 't>,
    pub ref_: InterfaceTT<'s, 't>,
    pub attributes: Vec<ICitizenAttributeT<'s, 't>>,
    pub weakable: bool,
    pub mutability: ITemplataT<'s, 't>,
    pub instantiation_bound_params: InstantiationBoundArgumentsT<'s, 't>,
    pub internal_methods: Vec<(PrototypeT<'s, 't>, usize)>,
}
impl<'s, 't> InterfaceDefinitionT<'s, 't> {}
/*
case class InterfaceDefinitionT(
  templateName: IdT[IInterfaceTemplateNameT],
  instantiatedInterface: InterfaceTT,
  ref: InterfaceTT,
  attributes: Vector[ICitizenAttributeT],
  weakable: Boolean,
  mutability: ITemplataT[MutabilityTemplataType],
  instantiationBoundParams: InstantiationBoundArgumentsT[FunctionBoundNameT, ImplBoundNameT],
  // This does not include abstract functions declared outside the interface.
  // Note from later: Though, sometimes macros add functions into the inside.
  // See IMRFDI for why we need to remember only the internal methods here.
  internalMethods: Vector[(PrototypeT[IFunctionNameT], Int)]
) extends CitizenDefinitionT {
*/
impl<'s, 't> InterfaceDefinitionT<'s, 't> {
    fn default_region(&self) -> RegionT {
        panic!("Unimplemented: default_region");
    }
}
/*
  def defaultRegion: RegionT = RegionT()
*/
impl<'s, 't> InterfaceDefinitionT<'s, 't> {
    fn generic_param_types(&self) -> Vec<ITemplataType<'s>> {
        panic!("Unimplemented: generic_param_types");
    }
}
/*
  override def genericParamTypes: Vector[ITemplataType] = {
    instantiatedCitizen.id.localName.templateArgs.map(_.tyype)
  }
*/
impl<'s, 't> InterfaceDefinitionT<'s, 't> {
    fn instantiated_citizen(&self) -> ICitizenTT<'s, 't> {
        panic!("Unimplemented: instantiated_citizen");
    }
}
/*
  override def instantiatedCitizen: ICitizenTT = instantiatedInterface
*/
impl<'s, 't> InterfaceDefinitionT<'s, 't> {
    fn equals(&self, obj: &Self) -> bool {
        panic!("Unimplemented: equals");
    }
}
/*
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
//  override def getRef = ref
}
*/