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
// mig: trait CitizenDefinitionT
pub trait CitizenDefinitionT {
}
/*
trait CitizenDefinitionT {
*/
// mig: fn template_name
fn template_name(&self) -> IdT;
/*
  def templateName: IdT[ICitizenTemplateNameT]
*/
// mig: fn generic_param_types
fn generic_param_types(&self) -> Vec<ITemplataType>;
/*
  def genericParamTypes: Vector[ITemplataType]
*/
// mig: fn instantiated_citizen
fn instantiated_citizen(&self) -> ICitizenTT;
/*
  def instantiatedCitizen: ICitizenTT
*/
// mig: fn default_region
fn default_region(&self) -> RegionT;
/*
  def defaultRegion: RegionT
}
*/
// mig: struct StructDefinitionT
pub struct StructDefinitionT {
    pub template_name: IdT,
    pub instantiated_citizen: StructTT,
    pub attributes: Vec<ICitizenAttributeT>,
    pub weakable: bool,
    pub mutability: ITemplataT,
    pub members: Vec<IStructMemberT>,
    pub is_closure: bool,
    pub instantiation_bound_params: InstantiationBoundArgumentsT,
}
// mig: impl StructDefinitionT
impl StructDefinitionT {}
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
// mig: fn default_region
fn default_region(&self) -> RegionT {
    panic!("Unimplemented: default_region");
}
/*
  def defaultRegion: RegionT = RegionT()
*/
// mig: fn generic_param_types
fn generic_param_types(&self) -> Vec<ITemplataType> {
    panic!("Unimplemented: generic_param_types");
}
/*
  override def genericParamTypes: Vector[ITemplataType] = {
    instantiatedCitizen.id.localName.templateArgs.map(_.tyype)
  }
*/
// mig: fn equals
fn equals(&self, obj: &Self) -> bool {
    panic!("Unimplemented: equals");
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
// mig: fn get_member_and_index
fn get_member_and_index(&self, needle_name: &IVarNameT) -> Option<(&NormalStructMemberT, usize)> {
    panic!("Unimplemented: get_member_and_index");
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
// mig: trait IStructMemberT
pub trait IStructMemberT {
}
/*
sealed trait IStructMemberT {
*/
// mig: fn name
fn name(&self) -> &IVarNameT;
/*
  def name: IVarNameT
}
*/
// mig: struct NormalStructMemberT
pub struct NormalStructMemberT {
    pub name: IVarNameT,
    pub variability: VariabilityT,
    pub tyype: IMemberTypeT,
}
// mig: impl NormalStructMemberT
impl NormalStructMemberT {
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
// mig: struct VariadicStructMemberT
pub struct VariadicStructMemberT {
    pub name: IVarNameT,
    pub tyype: PlaceholderTemplataT,
}
// mig: impl VariadicStructMemberT
impl VariadicStructMemberT {
}
/*
case class VariadicStructMemberT(
  name: IVarNameT,
  tyype: PlaceholderTemplataT[PackTemplataType]
) extends IStructMemberT {
  vpass()
}
*/
// mig: trait IMemberTypeT
pub trait IMemberTypeT {
}
/*
sealed trait IMemberTypeT  {
*/
// mig: fn reference
fn reference(&self) -> CoordT;
/*
  def reference: CoordT
*/
// mig: fn expect_reference_member
fn expect_reference_member(&self) -> ReferenceMemberTypeT {
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
// mig: fn expect_address_member
fn expect_address_member(&self) -> AddressMemberTypeT {
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
// mig: struct AddressMemberTypeT
pub struct AddressMemberTypeT {
    pub reference: CoordT,
}
// mig: impl AddressMemberTypeT
impl AddressMemberTypeT {
}
/*
case class AddressMemberTypeT(reference: CoordT) extends IMemberTypeT
*/
// mig: struct ReferenceMemberTypeT
pub struct ReferenceMemberTypeT {
    pub reference: CoordT,
}
// mig: impl ReferenceMemberTypeT
impl ReferenceMemberTypeT {
}
/*
case class ReferenceMemberTypeT(reference: CoordT) extends IMemberTypeT
*/
// mig: struct InterfaceDefinitionT
pub struct InterfaceDefinitionT {
    pub template_name: IdT,
    pub instantiated_interface: InterfaceTT,
    pub ref_: InterfaceTT,
    pub attributes: Vec<ICitizenAttributeT>,
    pub weakable: bool,
    pub mutability: ITemplataT,
    pub instantiation_bound_params: InstantiationBoundArgumentsT,
    pub internal_methods: Vec<(PrototypeT, usize)>,
}
// mig: impl InterfaceDefinitionT
impl InterfaceDefinitionT {}
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
// mig: fn default_region
fn default_region(&self) -> RegionT {
    panic!("Unimplemented: default_region");
}
/*
  def defaultRegion: RegionT = RegionT()
*/
// mig: fn generic_param_types
fn generic_param_types(&self) -> Vec<ITemplataType> {
    panic!("Unimplemented: generic_param_types");
}
/*
  override def genericParamTypes: Vector[ITemplataType] = {
    instantiatedCitizen.id.localName.templateArgs.map(_.tyype)
  }
*/
// mig: fn instantiated_citizen
fn instantiated_citizen(&self) -> ICitizenTT {
    panic!("Unimplemented: instantiated_citizen");
}
/*
  override def instantiatedCitizen: ICitizenTT = instantiatedInterface
*/
// mig: fn equals
fn equals(&self, obj: &Self) -> bool {
    panic!("Unimplemented: equals");
}
/*
  override def equals(obj: Any): Boolean = vcurious();
override def hashCode(): Int = vcurious()
//  override def getRef = ref
}
*/